# PMON - GitHub Workflow Monitor

A lightweight desktop application designed to monitor CI/CD workflows with a modular plugin architecture for different CI providers.

## Overview

PMON provides real-time monitoring of your CI/CD pipelines through a clean, responsive desktop interface. Built with Rust (Tauri) and Vue.js for performance and cross-platform compatibility.

## Features

- 🔄 **Real-time Monitoring**: Live status updates of CI/CD workflows
- 🔌 **Plugin Architecture**: Extensible system for multiple CI providers
- 🖥️ **Native Desktop**: Fast, lightweight application using system WebView
- 📊 **Dashboard View**: Clear visualization of repository status
- 🔔 **Desktop Notifications**: Instant alerts for build failures and successes

## Quick Start

### Prerequisites

- **Linux**: Ubuntu 24.04+, or other modern distributions
- **macOS**: 10.15+ (Catalina)
- **Windows**: Windows 10/11
- **Rust**: 1.70+
- **Node.js**: 18+

### Installation

#### Option 1: Using Nix (Recommended)

```bash
git clone https://github.com/your-username/pmon.git
cd pmon
nix-shell  # Provides all required dependencies
npm install
npm run tauri dev
```

#### Option 2: System Dependencies

**Ubuntu/Debian:**
```bash
# Install dependencies
sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0t64 libgdk-pixbuf-2.0-0

# Clone and run
git clone https://github.com/your-username/pmon.git
cd pmon
npm install
npm run tauri dev
```

**macOS:**
```bash
brew install rust node npm
git clone https://github.com/your-username/pmon.git
cd pmon
npm install
npm run tauri dev
```

### Configuration

Create a `config.yaml` file in the project root:

```yaml
github:
  token_env: "GITHUB_TOKEN"  # Environment variable with your GitHub PAT
  api_base_url: "https://api.github.com"
  poll_interval: 30          # Seconds between checks

repositories:
  - owner: "your-username"
    name: "your-repo"
    targets:
      - branch: "main"
      - branch: "develop"
```

## Development

### Building

```bash
# Development build
npm run tauri build -- --debug

# Production build
npm run tauri build
```

### Testing

```bash
# Frontend tests
npm run test

# Backend tests
cd src-tauri && cargo test

# Linting
npm run lint
cd src-tauri && cargo clippy
```

## Architecture

PMON follows a modular architecture:

- **Core System**: Rust backend managing plugins and state
- **Plugin Layer**: Extensible CI providers (currently GitHub)
- **Frontend**: Vue.js 3 dashboard with real-time updates
- **Event System**: Efficient communication between components

### Adding CI Providers

1. Implement the `DataProvider` trait in `src-tauri/plugins/`
2. Add configuration to `tauri.conf.json`
3. Update the config schema

See the GitHub plugin in `src-tauri/plugins/github/` for a reference implementation.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-lang.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

- 📖 [Documentation](CONTRIBUTING.md)
- 🐛 [Report Issues](https://github.com/your-username/pmon/issues)
- 💬 [Discussions](https://github.com/your-username/pmon/discussions)