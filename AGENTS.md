# GitHub Workflow Monitor - Project Plan

## Overview

A lightweight desktop application built with Tauri + Rust to monitor GitHub Actions workflows across multiple
repositories, providing real-time status updates and desktop notifications for failed workflows.

## Requirements

- **Scope**: Monitor 1-5 GitHub projects
- **Platform**: Runs locally on user machine
- **Features needed**:
    - Overview dashboard of main and PR workflows
    - Quick access to failed workflow details
    - Desktop notifications on workflow failures
    - Real-time monitoring (seconds-level polling)

## Technology Stack

### Core: Tauri + Rust

- **Why chosen**: Lightweight (~10MB), secure, performant, cross-platform
- **Frontend**: Vue.js with TypeScript for UI
- **Backend**: Rust for API calls and system integration
- **Notifications**: Native OS notifications via Tauri APIs

### Plugin Architecture: Cargo Features

- **Why chosen**: Type-safe, zero-overhead, compile-time guarantees
- **Core plugins**: GitHub provider compiled via Cargo features
- **Future extensibility**: Easy to add new data providers as features
- **No dynamic loading**: Simpler deployment, better performance

### Alternative Platforms Considered

| Platform            | Pros                            | Cons                 | Decision               |
|---------------------|---------------------------------|----------------------|------------------------|
| **Tauri + Rust**    | Lightweight, secure, performant | Smaller ecosystem    | ✅ Selected             |
| Python + Tkinter/Qt | Rich ecosystem                  | Heavier, less modern | Good alternative       |
| Electron + Node.js  | Web tech familiar               | Heavy resource usage | Overkill               |
| Go + Fyne/Wails     | Simple deployment               | Smaller UI ecosystem | Viable but less mature |

### Frontend Frameworks Considered

| Framework         | Pros                            | Cons                      | Decision               |
|-------------------|---------------------------------|---------------------------|------------------------|
| **Vue.js**        | Simple learning curve, reactive   | Smaller ecosystem than React | ✅ Selected             |
| React             | Large ecosystem, popular         | More complex               | Good alternative       |
| Svelte            | Performant, less boilerplate     | Smaller ecosystem          | Viable but less mature |
| Angular           | Enterprise features, opinionated   | Complex, heavy             | Overkill               |

## Architecture

### Backend (Rust)

- **Plugin System**: Data provider trait with compile-time feature selection
- **GitHub Provider**: `octocrab` crate for REST API integration
- **Real-time Polling**: Tokio async tasks (30-60 second intervals)
- **Configuration**: YAML config with JSON schema validation
- **State Management**: In-memory with disk persistence
- **Security**: Token encryption, system keychain storage

### Frontend (Vue.js + TypeScript)

- **Dashboard View**: Grid showing workflow statuses across projects
- **Real-time Updates**: WebSocket connection from backend
- **Quick Actions**: "Open in Browser" buttons for failed workflows
- **Component-based Architecture**: Vue 3 Composition API with TypeScript
- **Reactive State**: Pinia for state management

### Key Features

1. **Workflow Status Grid**: Color-coded overview dashboard
2. **Failed Workflow Details**: Expandable rows with error logs
3. **Quick Open**: Direct links to workflow URLs in browser
4. **Native Notifications**: OS-level alerts for failures
5. **Configuration Management**: UI for managing monitored repos

## Plugin Architecture

### Data Provider Trait

```rust
#[async_trait]
pub trait DataProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    
    async fn authenticate(&mut self, config: &ProviderConfig) -> Result<(), ProviderError>;
    async fn list_workflows(&self, repo: &RepositoryConfig) -> Result<Vec<Workflow>, ProviderError>;
    async fn get_workflow_runs(&self, repo: &RepositoryConfig) -> Result<Vec<WorkflowRun>, ProviderError>;
    async fn get_run_details(&self, run_id: &str) -> Result<WorkflowRunDetails, ProviderError>;
    
    fn config_schema(&self) -> JsonSchema;
}
```

### Cargo Features Structure

```toml
[features]
default = ["github-provider"]
github-provider = ["octocrab"]
gitlab-provider = []  # Future
azure-devops-provider = []  # Future
```

### Provider Configuration System

```yaml
# yaml-language-server: $schema=./schema.json
version: "1.0"
app:
  name: "GitHub Workflow Monitor"
  polling_interval_seconds: 30
  notifications_enabled: true

# Provider-specific configuration
providers:
  github:
    token_env: "GITHUB_TOKEN"
    api_base_url: "https://api.github.com"
    timeout_seconds: 30

repositories:
  - name: "my-project"
    owner: "my-username"
    provider: "github"  # Reference to provider
    enabled: true
    workflows:
      - name: "CI"
        patterns: [ "ci.yml", "continuous-integration.yml" ]
        monitor_branches: [ "main", "master" ]
        monitor_prs: true
        notify_on_failure: true
        notify_on_success: false

notifications:
  desktop:
    enabled: true
    timeout_seconds: 10
    sound_enabled: true
```

## Configuration System

### File Structure

```
config.yaml          # Main configuration file
schema.json           # JSON schema for validation
```

### YAML Configuration Design

```yaml
# yaml-language-server: $schema=./schema.json
version: "1.0"
app:
  name: "GitHub Workflow Monitor"
  polling_interval_seconds: 30
  notifications_enabled: true

github:
  token_env: "GITHUB_TOKEN"
  api_base_url: "https://api.github.com"
  timeout_seconds: 30

repositories:
  - name: "my-project"
    owner: "my-username"
    enabled: true
    workflows:
      - name: "CI"
        patterns: [ "ci.yml", "continuous-integration.yml" ]
        monitor_branches: [ "main", "master" ]
        monitor_prs: true
        notify_on_failure: true
        notify_on_success: false

notifications:
  desktop:
    enabled: true
    timeout_seconds: 10
    sound_enabled: true
```

### Configuration Features

- **Hierarchical Structure**: Global, GitHub, repository, and workflow settings
- **Pattern Matching**: Monitor multiple workflows with regex patterns
- **Branch Filtering**: Control which branches to monitor
- **PR Monitoring**: Separate control for PR vs main branch workflows
- **Selective Notifications**: Configure failure/success notifications per workflow
- **IDE Support**: Auto-completion and validation via JSON schema
- **Security**: Environment variable tokens, never stored in config

### Rust Implementation

- **Data Structures**: Serde-derived structs for type safety
- **Schema Validation**: `schemars` for JSON schema generation
- **Runtime Validation**: `serde_json` for config validation
- **Error Handling**: Clear validation error messages

## API Integration

### GitHub API Endpoints

- `GET /repos/{owner}/{repo}/actions/workflows` - List workflows
- `GET /repos/{owner}/{repo}/actions/runs` - Get recent runs
- `GET /repos/{owner}/{repo}/actions/runs/{run_id}/jobs` - Get job details

### Polling Strategy

- **Background Tasks**: Async Tokio tasks
- **State Tracking**: Store last known state to detect changes
- **Rate Limiting**: Respect GitHub API limits
- **Prioritization**: Main branch and recent PR workflows

## Development Plan

### Phase 1: Core Infrastructure

1. **Setup Tauri project** with Vue.js + TypeScript frontend
2. **Implement plugin architecture** with data provider trait
3. **Create GitHub provider** as first plugin implementation
4. **Basic dashboard UI** showing repository list
5. **Configuration system** with YAML schema validation

### Phase 2: Monitoring Features

1. **Implement real-time polling** using plugin trait methods
2. **Workflow status grid** with color-coded indicators
3. **Failed workflow details** with expandable information
4. **Quick open functionality** to browser
5. **Provider abstraction** in UI (hide GitHub-specific details)

### Phase 3: Notifications & Polish

1. **Native notification system** using Tauri APIs
2. **Error handling** and user feedback
3. **Plugin validation** and configuration errors
4. **Testing** and performance optimization

## Security Considerations

### Token Management

- **Environment Variables**: `GITHUB_TOKEN` for authentication
- **System Keychain**: Store tokens securely using OS keychain
- **No Token Logging**: Prevent accidental token exposure
- **Encryption**: Encrypt sensitive data at rest

### API Security

- **Rate Limiting**: Respect GitHub API limits
- **Timeout Configuration**: Prevent hanging requests
- **Input Validation**: Sanitize URLs and patterns
- **HTTPS Only**: Enforce secure API communication

## Developer Experience

### IDE Integration

- **VSCode**: Automatic schema detection via `yaml-language-server`
- **Vim/Neovim**: Schema validation with LSP
- **IntelliJ**: Native YAML schema support

### CLI Tools

- **Configuration Validation**: `pmon config validate --file config.yaml`
- **Template Generation**: `pmon config init --template basic`
- **Schema Display**: `pmon config schema`

### Future Data Providers
- **GitLab CI/CD**: Monitor GitLab pipelines
- **Azure DevOps**: Monitor Azure Pipelines  
- **Jenkins**: Monitor Jenkins jobs
- **Custom Providers**: User-defined monitoring sources

### Future Enhancements

- **Configuration Hot-Reloading**: File watching with `notify` crate
- **Advanced Filtering**: More granular workflow filtering
- **Multi-Tenant Support**: Different GitHub accounts
- **Export/Import**: Configuration backup and sharing
- **Dynamic Plugin Loading**: Runtime plugin discovery and loading

## Performance & Resource Usage

### Optimization Strategies

- **Efficient Polling**: Only check changed repositories
- **State Caching**: Cache workflow runs to reduce API calls
- **Background Processing**: Non-blocking UI updates
- **Resource Limits**: Configurable concurrency limits

### Monitoring Metrics

- **API Usage**: Track GitHub API rate limits
- **Memory Usage**: Monitor application memory consumption
- **Response Times**: Track API response performance
- **Error Rates**: Monitor failed requests and notifications

## Testing Strategy

### Unit Tests

- **Configuration parsing** and validation
- **GitHub API client** error handling
- **State management** and data transformations

### Integration Tests

- **End-to-end workflows** with mock GitHub API
- **Notification system** testing
- **Configuration hot-reloading** (future)

### Manual Testing

- **Multi-platform compatibility** (Windows/macOS/Linux)
- **Real-world scenarios** with actual repositories
- **User experience** validation

## Deployment & Distribution

### Build Process

- **Cross-compilation** for all platforms
- **Code signing** for security
- **Notarization** (macOS)
- **Installer creation** (NSIS, MSI, DMG)

### Distribution Channels

- **GitHub Releases** for direct downloads
- **Package managers** (Homebrew, Chocolatey, apt)
- **Auto-updater** for seamless updates