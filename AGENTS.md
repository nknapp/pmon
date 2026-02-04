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

- **Core API**: Passive system receiving data from plugins
- **Plugin System**: Data providers manage their own update strategies
- **GitHub Provider**: `octocrab` crate with polling implementation
- **State Management**: In-memory with disk persistence
- **Event Bus**: Internal communication between core and frontend
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

### Generic Data Models

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub status: RunStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunStatus {
    Success,
    Failed,
    Running,
    Aborted,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStream {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub runs: Vec<Run>,
    pub current_run: Option<Run>,
    pub previous_run: Option<Run>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Target {
    Branch(String),
    PullRequest { number: u64, title: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub url: String,
    pub targets: Vec<Target>,
    pub workflow_streams: Vec<WorkflowStream>,
}
```

### Data Provider Trait

```rust
#[async_trait]
pub trait DataProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    
    async fn authenticate(&mut self, config: &ProviderConfig) -> Result<(), ProviderError>;
    async fn start_monitoring(&mut self, core_api: Box<dyn CoreApi>) -> Result<(), ProviderError>;
    async fn stop_monitoring(&mut self) -> Result<(), ProviderError>;
    
    fn config_schema(&self) -> JsonSchema;
}

/// Core API that plugins use to push data updates
#[async_trait]
pub trait CoreApi: Send + Sync {
    async fn update_repository(&self, repository: Repository) -> Result<(), CoreError>;
    async fn update_workflow_stream(&self, repo_id: &str, stream: WorkflowStream) -> Result<(), CoreError>;
    async fn update_run(&self, repo_id: &str, stream_id: &str, run: Run) -> Result<(), CoreError>;
    async fn notify(&self, notification: Notification) -> Result<(), CoreError>;
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

- **Hierarchical Structure**: Global, provider, repository, and workflow settings
- **Pattern Matching**: Monitor multiple workflows with regex patterns
- **Target Filtering**: Control which branches/PRs to monitor
- **Selective Notifications**: Configure failure/success notifications per workflow
- **Provider Abstraction**: Generic configuration that works for any provider
- **IDE Support**: Auto-completion and validation via JSON schema
- **Security**: Environment variable tokens, never stored in config

### Rust Implementation

- **Data Structures**: Serde-derived structs for type safety
- **Schema Validation**: `schemars` for JSON schema generation
- **Runtime Validation**: `serde_json` for config validation
- **Error Handling**: Clear validation error messages

## Data Model Hierarchy

```
Repository
├── Targets (Branches & Pull Requests)
│   ├── Branch "main"
│   ├── Branch "develop"
│   └── Pull Request #123
└── Workflow Streams
    ├── CI Stream
    │   ├── Current Run (Running)
    │   └── Previous Run (Success/Failed)
    ├── Deploy Stream
    │   └── Current Run (Success)
    └── Tests Stream
        └── Current Run (Failed)
```

## API Integration

### Provider-Specific Endpoints

#### GitHub API
- `GET /repos/{owner}/{repo}/actions/workflows` - List workflows
- `GET /repos/{owner}/{repo}/actions/runs` - Get recent runs
- `GET /repos/{owner}/{repo}/actions/runs/{run_id}/jobs` - Get job details

#### Future GitLab API
- `GET /projects/{id}/pipelines` - List pipelines
- `GET /projects/{id}/jobs` - Get job details

### Plugin Update Strategies

#### Polling-based (e.g., GitHub REST API)
- **Background Tasks**: Async Tokio tasks polling at intervals
- **State Tracking**: Store last known state to detect changes
- **Rate Limiting**: Respect provider-specific API limits
- **Prioritization**: Main branch and recent PR workflows

#### Real-time (e.g., GitHub Webhooks, GitLab Websockets)
- **Webhook Servers**: Receive immediate updates from providers
- **WebSocket Connections**: Persistent connections for live data
- **Event Processing**: Handle incoming workflow events immediately
- **Push-based Updates**: No polling overhead

#### Hybrid Approaches
- **Webhook + Polling**: Webhooks for immediate updates, polling for catch-up
- **Smart Polling**: Adaptive intervals based on activity
- **Batch Updates**: Efficient bulk updates during idle periods

## Development Plan

### Phase 1: Core Infrastructure

1. **Setup Tauri project** with Vue.js + TypeScript frontend
2. **Implement plugin architecture** with Core API and DataProvider trait
3. **Create GitHub provider** with polling implementation
4. **Implement Core API** for data reception and event broadcasting
5. **Basic dashboard UI** showing repository list
6. **Configuration system** with YAML schema validation

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