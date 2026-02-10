# Contributing to PMON

Thank you for your interest in contributing to PMON! This document provides all the information you need to get started.

## Project Overview

PMON is a lightweight desktop application designed to monitor CI/CD workflows. It's built with:
- **Backend**: Rust (Tauri Core)
- **Frontend**: Vue.js 3 + TypeScript
- **Architecture**: Modular plugin system for different CI providers

## Prerequisites

### System Requirements

- **Operating System**: Linux (Ubuntu 24.04+ recommended), macOS, or Windows
- **Rust**: 1.70+ with `rustc`, `cargo`, and `rustup`
- **Node.js**: 18+ with npm
- **Git**: 2.30+

### Ubuntu/Debian Linux

#### Required system libraries

* See https://v2.tauri.app/start/prerequisites/ for system dependencies for your OS.
* We recommend using [mise-en-place](https://mise.jdx.dev/) to install basic build tools like rust and node.

## Development Setup

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/pmon.git
cd pmon
```

### 2. Enable mise

```bash
mise trust
```

### 2. Install Node Dependencies

```bash
npm install
```

### 3. Development Mode

Start the development server with hot reload:

```bash
# Using npm scripts
npm run tauri dev

# Or directly with Tauri CLI
npx tauri dev
```

This will:
- Start the Vite development server for the frontend
- Launch the Tauri development window
- Enable hot reload for both frontend and backend changes

### Docker Compose Helper (CI)

Use `./scripts/ci-compose` as a drop-in replacement for `docker compose` when running CI-style containers locally. It forwards all arguments while exporting `UID` and `GID` so containers can map file ownership correctly.

```bash
./scripts/run-local-ci up -d
./scripts/run-local-ci run --rm app npm test
./scripts/run-local-ci down
```

## Building

### Development Build

```bash
npm run tauri build -- --debug
```

### Production Build

```bash
npm run tauri build
```

The built artifacts will be available in:
- Linux: `src-tauri/target/release/bundle/deb/`
- macOS: `src-tauri/target/release/bundle/macos/`
- Windows: `src-tauri/target/release/bundle/msi/`

## Project Structure

```
pmon/
├── src/                     # Vue.js frontend source
│   ├── components/         # Vue components
│   ├── stores/             # Pinia state management
│   └── types/              # TypeScript definitions
├── src-tauri/              # Rust backend source
│   ├── src/                # Main application code
│   ├── plugins/            # CI provider plugins
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── package.json            # Node.js dependencies
└── README.md               # Project documentation
```

## Architecture Overview

### Core Components

1. **Tauri Core**: Handles system integration and UI
2. **Plugin System**: Extensible architecture for CI providers
3. **Data Models**: Shared structures for repositories and workflows
4. **Event System**: Real-time updates between backend and frontend

### Adding New CI Providers

1. Create a new plugin in `src-tauri/plugins/`
2. Implement the `DataProvider` trait
3. Add configuration to `tauri.conf.json`
4. Register the plugin in the main application

See `src-tauri/plugins/github/` for a reference implementation.

## Code Style

### Rust

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### TypeScript/Vue.js

- Use `npm run lint` for linting
- Use `npm run type-check` for type checking
- Follow [Vue 3 Style Guide](https://vuejs.org/style-guide/)

## Testing

### Frontend Tests

```bash
npm run test
```

### Backend Tests

```bash
cd src-tauri
cargo test
```

### End-to-End Tests

```bash
npm run tauri test
```

## Submitting Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed

### 3. Run Tests

```bash
# Frontend
npm run lint
npm run type-check
npm run test

# Backend
cd src-tauri
cargo fmt
cargo clippy
cargo test
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "feat: add your feature description"
```

Use [Conventional Commits](https://www.conventionalcommits.org/) format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `refactor:` for code refactoring
- `test:` for adding tests
- `chore:` for maintenance tasks

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Open a pull request on GitHub with:
- Clear description of changes
- Testing instructions
- Related issue numbers

## Common Issues

### Library Issues on Linux

If you encounter missing library errors:

**Using Nix** (recommended):
```bash
nix-shell  # This provides all required dependencies
```

**System Installation**:
```bash
sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0t64 libgdk-pixbuf-2.0-0
```

### Build Fails with Permission Errors

```bash
# Clean build cache
cd src-tauri
cargo clean
npm run tauri build
```

### Frontend Not Updating

```bash
# Clear Vite cache
rm -rf node_modules/.vite
npm run dev
```

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/your-username/pmon/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/pmon/discussions)
- **Documentation**: Check inline code comments and README.md

## License

By contributing to this project, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to PMON! 🚀
