# Nexus Dashboard - System Architecture

```mermaid
graph TB
    subgraph "Frontend Layer - React 19"
        UI[React Components]
        Hooks[Custom Hooks]
        State[Zustand Store]
        
        UI --> Hooks
        Hooks --> State
    end
    
    subgraph "IPC Bridge - Tauri"
        Commands[Tauri Commands]
        Events[Event System]
        TypeGen[Specta Type Generator]
        
        Commands -.->|Auto-generates| TypeGen
        TypeGen -.->|TypeScript Types| Hooks
    end
    
    subgraph "Backend Layer - Rust"
        Services[Business Services]
        Models[Data Models]
        Utils[Utilities]
        
        Services --> Models
        Services --> Utils
    end
    
    subgraph "System Resources"
        FS[Filesystem]
        Hardware[Hardware APIs]
        Steam[Steam Directories]
        
        FS -->|Obsidian Vault| Services
        Hardware -->|NVML/sysinfo| Services
        Steam -->|Proton Paths| Services
    end
    
    UI -->|invoke commands| Commands
    Commands -->|call| Services
    Services -->|emit events| Events
    Events -->|listen| Hooks

    style UI fill:#06b6d4
    style Commands fill:#10b981
    style Services fill:#f59e0b
    style Hardware fill:#ef4444
```

## Data Flow: System Monitoring

```mermaid
sequenceDiagram
    participant BG as Background Task
    participant SM as SystemMonitor
    participant NVML as Nvidia NVML
    participant Tauri as Event Emitter
    participant Hook as useSystemStats
    participant UI as SystemSentinel

    loop Every 1 second
        BG->>SM: snapshot()
        SM->>NVML: get_utilization()
        NVML-->>SM: 75% GPU usage
        SM-->>BG: SystemStats
        BG->>Tauri: emit("system-stats", stats)
        Tauri->>Hook: Event payload
        Hook->>UI: setState(stats)
        UI->>UI: Re-render with new data
    end
```

## Module Architecture

```mermaid
graph LR
    subgraph "System Sentinel Module"
        SS_UI[UI Component]
        SS_Hook[useSystemStats]
        SS_Chart[Sparkline Chart]
        
        SS_UI --> SS_Hook
        SS_UI --> SS_Chart
    end
    
    subgraph "Obsidian Viewer Module"
        OV_UI[UI Component]
        OV_Hook[useObsidianNotes]
        OV_List[Note List]
        OV_View[Markdown Viewer]
        
        OV_UI --> OV_Hook
        OV_UI --> OV_List
        OV_UI --> OV_View
    end
    
    subgraph "Proton Reactor Module"
        PR_UI[UI Component]
        PR_Hook[useProtonVersions]
        PR_List[Version List]
        
        PR_UI --> PR_Hook
        PR_UI --> PR_List
    end
    
    SS_Hook -->|IPC| SystemMonitor[SystemMonitor Service]
    OV_Hook -->|IPC| Vault[ObsidianVault Service]
    PR_Hook -->|IPC| Scanner[ProtonScanner Service]
```

## Type Safety Flow

```mermaid
graph TD
    A[Define Rust Type] -->|#[derive Type]| B[Rust Struct]
    B --> C[Tauri Command]
    C -->|cargo build| D[Specta Generator]
    D -->|Generates| E[TypeScript Interface]
    E --> F[React Component]
    F -->|invoke| C
    C -->|Type-safe return| F
    
    style A fill:#f59e0b
    style E fill:#06b6d4
    style D fill:#10b981
```

## Deployment Architecture

```mermaid
graph TB
    Dev[Development Machine]
    
    Dev -->|npm run build| Build[Vite Build Process]
    Build --> ReactBundle[React Bundle]
    
    Dev -->|cargo build --release| RustBuild[Rust Compiler]
    RustBuild --> Binary[Native Binary]
    
    ReactBundle -->|Embedded in| Tauri[Tauri Bundler]
    Binary -->|Packaged with| Tauri
    
    Tauri --> Executable[nexus-dashboard]
    Executable -->|Runs on| Bazzite[Bazzite Linux]
    
    style Executable fill:#10b981
    style Bazzite fill:#06b6d4
```

## File System Access Patterns

```mermaid
graph LR
    App[Nexus Dashboard]
    
    App -->|Read| Vault[Obsidian Vault<br/>/var/home/.../fs0ciety]
    App -->|Scan| Steam1[~/.steam/<br/>compatibilitytools.d]
    App -->|Scan| Steam2[~/.local/share/Steam/<br/>compatibilitytools.d]
    App -->|Monitor| NVML[/dev/nvidia0<br/>via NVML]
    App -->|Read| Sysinfo[/proc/stat<br/>/sys/class/thermal]
    
    style Vault fill:#f59e0b
    style NVML fill:#ef4444
    style Sysinfo fill:#10b981
```

## Performance Optimization Strategy

```mermaid
graph TB
    subgraph "Rust Backend"
        Cache[Cached Computations]
        Async[Async Tasks]
        Arc[Arc<Mutex> State]
    end
    
    subgraph "React Frontend"
        Memo[React.memo]
        Callback[useCallback]
        Debounce[Debounced Updates]
    end
    
    subgraph "IPC Layer"
        Events[Event Batching]
        Types[Zero-Copy Serialization]
    end
    
    Cache --> Target1[<2s Cold Start]
    Async --> Target2[<100ms Latency]
    Memo --> Target3[60fps UI]
    Events --> Target2
    
    style Target1 fill:#10b981
    style Target2 fill:#10b981
    style Target3 fill:#10b981
```
