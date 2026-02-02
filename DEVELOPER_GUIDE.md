# Developer Guide - Nexus Dashboard

## Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Code Organization](#code-organization)
3. [Type Safety Strategy](#type-safety-strategy)
4. [Adding New Features](#adding-new-features)
5. [Performance Guidelines](#performance-guidelines)
6. [Debugging](#debugging)

## Architecture Overview

### The Three-Layer System

```
┌─────────────────────────────────────┐
│         React Frontend              │
│  (UI Components, Hooks, State)      │
└──────────────┬──────────────────────┘
               │ IPC via Tauri
┌──────────────▼──────────────────────┐
│      Tauri Commands Layer           │
│  (RPC Endpoints, Input Validation)  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│       Services Layer                │
│  (Business Logic, Data Access)      │
└─────────────────────────────────────┘
```

### Data Flow Example: System Monitoring

1. **Background Task** (`main.rs`):
   ```rust
   tokio::spawn(async move {
       loop {
           let stats = monitor.snapshot()?;
           app_handle.emit("system-stats", stats)?;
           tokio::time::sleep(Duration::from_secs(1)).await;
       }
   });
   ```

2. **React Hook** (`useSystemStats.ts`):
   ```typescript
   const [stats, setStats] = useState<SystemStats | null>(null);
   
   useEffect(() => {
       const unlisten = listen<SystemStats>("system-stats", (event) => {
           setStats(event.payload);
       });
       return () => { unlisten.then(f => f()); };
   }, []);
   ```

3. **Component** (`SystemSentinel/index.tsx`):
   ```tsx
   const { stats } = useSystemStats();
   return <div>{stats?.cpuUsage}%</div>;
   ```

## Code Organization

### Rust Module Structure

**Services** (`src-tauri/src/services/`)
- Pure business logic
- No Tauri dependencies
- Return `Result<T, anyhow::Error>`
- Testable in isolation

```rust
// ✅ Good
pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn snapshot(&mut self) -> Result<SystemStats> {
        // Implementation
    }
}

// ❌ Bad (Tauri-coupled)
#[tauri::command]
pub fn get_stats(app: AppHandle) -> SystemStats { }
```

**Commands** (`src-tauri/src/commands/`)
- Thin wrappers around services
- Handle Tauri state management
- Convert errors to strings for IPC

```rust
// ✅ Good
#[tauri::command]
#[specta::specta]
pub async fn list_notes() -> Result<Vec<NoteMetadata>, String> {
    let vault = ObsidianVault::new(VAULT_PATH).map_err(|e| e.to_string())?;
    vault.list_notes().map_err(|e| e.to_string())
}
```

**Models** (`src-tauri/src/models/`)
- Data transfer objects only
- Always derive `Serialize`, `Deserialize`, `Type`
- Use `#[serde(rename_all = "camelCase")]` for JS conventions

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub gpu_usage: Option<f32>,
}
```

### React Component Structure

**Module Pattern**:
```
modules/FeatureName/
├── index.tsx           # Main component (default export)
├── SubComponent.tsx    # Private sub-components
└── types.ts            # Local types (if needed)
```

**Component Guidelines**:
- One default export per module
- Use custom hooks for data fetching
- Keep components under 200 lines
- Extract complex logic to hooks

```tsx
// ✅ Good
export default function SystemSentinel() {
    const { stats, error, isLoading } = useSystemStats();
    
    if (error) return <ErrorCard error={error} />;
    if (isLoading) return <LoadingCard />;
    
    return <StatsDisplay stats={stats} />;
}
```

## Type Safety Strategy

### The Contract

**Rust defines the types, TypeScript imports them.**

1. Define model in Rust:
```rust
#[derive(Serialize, Type)]
pub struct Note {
    pub id: String,
    pub content: String,
}
```

2. Generate types:
```bash
cd src-tauri && cargo build
```

3. Use in TypeScript:
```typescript
import type { Note } from '@/lib/tauri';

function MyComponent() {
    const [note, setNote] = useState<Note | null>(null);
}
```

### Type Generation Checklist

Before committing:
- [ ] All new Rust commands have `#[specta::specta]`
- [ ] All new models derive `Type`
- [ ] `cargo build` runs without errors
- [ ] `src/lib/tauri.ts` contains new types

## Adding New Features

### Example: Adding a "Network Monitor" Module

**Step 1: Create Service**
```rust
// src-tauri/src/services/network_monitor.rs
pub struct NetworkMonitor {
    // Implementation
}

impl NetworkMonitor {
    pub fn get_stats(&self) -> Result<NetworkStats> {
        // Logic here
    }
}
```

**Step 2: Define Model**
```rust
// src-tauri/src/models/network.rs
#[derive(Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct NetworkStats {
    pub download_speed: u64, // bytes/sec
    pub upload_speed: u64,
}
```

**Step 3: Create Command**
```rust
// src-tauri/src/commands/network.rs
#[tauri::command]
#[specta::specta]
pub async fn get_network_stats() -> Result<NetworkStats, String> {
    let monitor = NetworkMonitor::new();
    monitor.get_stats().map_err(|e| e.to_string())
}
```

**Step 4: Register Command**
```rust
// src-tauri/src/main.rs
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    network::get_network_stats,
])
```

**Step 5: Create Hook**
```typescript
// src/hooks/useNetworkStats.ts
export function useNetworkStats() {
    const [stats, setStats] = useState<NetworkStats | null>(null);
    
    useEffect(() => {
        const interval = setInterval(async () => {
            const data = await invoke<NetworkStats>("get_network_stats");
            setStats(data);
        }, 1000);
        
        return () => clearInterval(interval);
    }, []);
    
    return { stats };
}
```

**Step 6: Create Component**
```tsx
// src/components/modules/NetworkMonitor/index.tsx
export default function NetworkMonitor() {
    const { stats } = useNetworkStats();
    
    return (
        <Card>
            <CardHeader>
                <CardTitle>Network Monitor</CardTitle>
            </CardHeader>
            <CardContent>
                <div>↓ {formatBytes(stats?.downloadSpeed || 0)}/s</div>
                <div>↑ {formatBytes(stats?.uploadSpeed || 0)}/s</div>
            </CardContent>
        </Card>
    );
}
```

**Step 7: Add to Grid**
```tsx
// src/App.tsx
<BentoGrid>
    {/* ... existing modules */}
    <NetworkMonitor />
</BentoGrid>
```

## Performance Guidelines

### Rust Side

**DO:**
- Use `tokio::spawn` for long-running tasks
- Cache expensive computations
- Use `Arc<Mutex<T>>` for shared state
- Prefer `&str` over `String` in function signatures

**DON'T:**
- Block the main thread
- Make synchronous filesystem calls in commands
- Create new NVML/sysinfo instances per call

### React Side

**DO:**
- Use `React.memo` for expensive components
- Debounce rapid state updates
- Virtualize long lists (if >100 items)
- Use `useCallback` for event handlers

**DON'T:**
- Fetch data in render
- Create inline object/array literals as deps
- Update state more than 60 times/second

### Example: Optimized Component

```tsx
const SystemSentinel = React.memo(() => {
    const { stats } = useSystemStats();
    
    // Only update chart data when stats change
    const chartData = useMemo(() => {
        return processChartData(stats);
    }, [stats]);
    
    // Memoize expensive render
    return (
        <Card>
            <ExpensiveChart data={chartData} />
        </Card>
    );
});
```

## Debugging

### Rust Backend

**Enable Logging:**
```rust
println!("Debug: {:?}", value);
eprintln!("Error: {}", error);
```

**Run Tests:**
```bash
cd src-tauri
cargo test -- --nocapture  # See println! output
```

**Check Type Generation:**
```bash
cargo build 2>&1 | grep specta
```

### React Frontend

**Tauri DevTools:**
```bash
npm run dev  # Opens with Chrome DevTools
```

**Check IPC Calls:**
```typescript
invoke("command_name", { arg: "value" })
    .then(result => console.log("Success:", result))
    .catch(err => console.error("Error:", err));
```

**Performance Profiling:**
```tsx
import { Profiler } from 'react';

<Profiler id="SystemSentinel" onRender={(id, phase, duration) => {
    console.log(`${id} (${phase}): ${duration}ms`);
}}>
    <SystemSentinel />
</Profiler>
```

### Common Issues

**Issue**: "Failed to load NVML"
**Solution**: Install `nvidia-driver` and `libnvidia-ml1`

**Issue**: TypeScript types not updating
**Solution**: `cd src-tauri && cargo clean && cargo build`

**Issue**: "Vault not found"
**Solution**: Check path in `commands/obsidian.rs` and ensure it exists

**Issue**: High CPU usage
**Solution**: Increase telemetry interval in `main.rs`

## Testing Checklist

Before submitting a PR:
- [ ] `cargo test` passes
- [ ] `npm run typecheck` passes
- [ ] `npm run lint` passes
- [ ] No console errors in dev mode
- [ ] Memory usage <150MB after 5 minutes
- [ ] UI responsive at 60fps
- [ ] All modules load without errors

## Best Practices Summary

1. **Type Safety First**: Always use `specta` for Rust↔TypeScript
2. **Separation of Concerns**: Keep services pure, commands thin
3. **Error Handling**: Always use `Result<T, E>` in Rust
4. **Performance**: Profile before optimizing
5. **Documentation**: Every public API gets `///` docs
6. **Testing**: Unit test business logic, integration test IPC

---

Happy coding! 🚀
