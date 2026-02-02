# Contributing to Nexus Dashboard

First off, thank you for considering contributing to Nexus Dashboard! 🎉

## 🌟 Code of Conduct

This project follows a Code of Conduct. By participating, you are expected to uphold this code:
- Be respectful and inclusive
- Focus on what is best for the community
- Show empathy towards other community members

## 🚀 How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce**
- **Expected vs actual behavior**
- **System information** (OS, Rust version, Node version)
- **Screenshots** if applicable

**Bug Report Template:**
```markdown
## Description
[Clear description of the bug]

## To Reproduce
1. Go to '...'
2. Click on '...'
3. See error

## Expected Behavior
[What you expected to happen]

## System Info
- OS: Bazzite Linux / Fedora / etc.
- Rust: 1.75+
- Node: 20+
- GPU: Nvidia RTX 3080
```

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. Include:

- **Clear title and description**
- **Use case** - why is this enhancement needed?
- **Proposed solution**
- **Alternative solutions** you've considered

### Pull Requests

1. **Fork the repo** and create your branch from `main`
2. **Follow the code style** (Biome for TS, rustfmt for Rust)
3. **Write meaningful commit messages** (see Commit Convention below)
4. **Add tests** if applicable
5. **Update documentation** if needed
6. **Ensure all tests pass** (`cargo test && npm run typecheck`)

## 🏗️ Development Setup

### Prerequisites
```bash
# Rust
rustup update

# Node.js
node --version  # Should be 20+

# Nvidia drivers (for GPU monitoring)
nvidia-smi
```

### Setup
```bash
git clone https://github.com/fs0ciety7000/nexus.git
cd nexus
npm install
cd src-tauri && cargo build
cd .. && npm run dev
```

## 📝 Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, no logic change)
- `refactor`: Code refactor (no feature change)
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Maintenance, configuration

### Examples
```bash
feat(system): add disk usage monitoring
fix(obsidian): handle empty vault gracefully
docs: update installation instructions for Arch Linux
perf(telemetry): reduce polling interval overhead
```

## 🎨 Code Style

### Rust
- Run `cargo fmt` before committing
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- All public APIs must have `///` documentation
- Use `Result<T, E>` for fallible functions

```rust
/// Captures current hardware telemetry snapshot
///
/// # Errors
/// Returns error if NVML or sysinfo fails
pub fn snapshot(&mut self) -> Result<SystemStats> {
    // Implementation
}
```

### TypeScript
- Run `npm run format` before committing
- Use `npm run lint` to check
- Prefer functional components with hooks
- Always define prop types

```typescript
interface SystemSentinelProps {
  refreshInterval?: number;
}

export default function SystemSentinel({ 
  refreshInterval = 1000 
}: SystemSentinelProps) {
  // Implementation
}
```

### CSS/Tailwind
- Use Tailwind utility classes first
- Custom CSS only when necessary
- Follow BEM naming for custom classes
- Maintain dark theme compatibility

## 🧪 Testing

### Rust Tests
```bash
cd src-tauri
cargo test

# With output
cargo test -- --nocapture
```

### TypeScript Type Checking
```bash
npm run typecheck
```

### Manual Testing Checklist
- [ ] App starts without errors
- [ ] System monitoring updates in real-time
- [ ] Obsidian vault loads correctly
- [ ] Proton versions detected
- [ ] No memory leaks after 5 minutes
- [ ] UI remains responsive under load

## 📦 Adding New Modules

See `DEVELOPER_GUIDE.md` for detailed instructions. Quick overview:

1. **Create Rust service** in `src-tauri/src/services/`
2. **Define models** in `src-tauri/src/models/`
3. **Add commands** in `src-tauri/src/commands/`
4. **Register in main.rs**
5. **Create React component** in `src/components/modules/`
6. **Add to BentoGrid** in `src/App.tsx`

## 🔍 Code Review Process

1. **Self-review** your code before submitting
2. **Ensure CI passes** (tests, linting)
3. **Respond to feedback** constructively
4. **Request re-review** after changes

### Review Criteria
- Code follows style guidelines
- Changes are well-documented
- Tests pass
- No breaking changes (or properly documented)
- Performance impact considered

## 📚 Documentation

### When to Update Docs

Update documentation when:
- Adding new features
- Changing public APIs
- Fixing bugs that affect usage
- Adding configuration options

### Documentation Types

- **README.md**: User-facing, getting started
- **DEVELOPER_GUIDE.md**: Architecture, patterns
- **Inline comments**: Complex logic explanation
- **API docs**: All public Rust functions

## 🐛 Debugging Tips

### Rust Backend
```rust
// Add debug prints
println!("Debug: {:?}", value);
eprintln!("Error: {}", error);

// Run with backtrace
RUST_BACKTRACE=1 cargo run
```

### React Frontend
```typescript
// Use React DevTools
// Check console for errors
console.log("Component rendered:", props);
```

## 🎯 Priority Areas

We're especially interested in contributions for:

1. **New Modules**
   - Excel/CSV data visualization
   - Calendar integration
   - Game time tracking

2. **Performance**
   - Reduce memory footprint
   - Optimize rendering
   - Better caching strategies

3. **Platform Support**
   - Test on different Linux distros
   - AMD GPU support (ROCm)
   - Wayland compatibility

4. **Documentation**
   - Video tutorials
   - Architecture diagrams
   - Translation to other languages

## 📊 Performance Guidelines

All contributions should maintain:
- **Cold start**: <2s
- **Telemetry latency**: <100ms
- **Memory**: <150MB
- **UI frame rate**: 60fps

Profile before and after your changes!

## ❓ Questions?

- **General questions**: Open a Discussion
- **Bug reports**: Open an Issue
- **Feature requests**: Open an Issue with `enhancement` label
- **Security issues**: Email security@example.com (replace with actual)

## 🙏 Recognition

Contributors will be:
- Listed in README.md
- Mentioned in release notes
- Given credit in commit history

Thank you for contributing to Nexus Dashboard! 🚀
