# GitHub Workflow Monitor - Architecture & Plugin Guide

## Running Locally



## 1. High-Level Architecture Overview

**pmon** is a lightweight desktop application designed to monitor CI/CD workflows. It follows a modular architecture where the core system provides the UI and state management, while specific CI providers (like GitHub) are implemented as compile-time plugins.

### Core Technology Stack
- **Backend**: Rust (Tauri Core) - Handles system integration, API orchestration, and state persistence.
- **Frontend**: Vue.js 3 + TypeScript - Provides the reactive UI dashboard.
- **Communication**: Tauri Events & Commands - Bridges the Rust backend and WebView frontend.

### Architecture Components

1.  **Core System (Rust)**
    - Core business logic and state management.
    - Provides ports for different plugins:
      - `DataProvider` - Trait to data to plugins.
      - `DashboardProvider` - Trait to 
    - State store for repositories and workflow runs.
2.  **Plugin Layer**
    -   Implements the `DataProvider` trait.
    -   Runs independently (usually in background tasks).
    -   Normalizes provider-specific data into generic `Repository` and `WorkflowStream` models.
    -   Pushes updates to the Core via the `CoreApi`.

3.  **Frontend (Vue.js)**
    -   **Dashboard**: Displays real-time status grids.
    -   **Store (Pinia)**: Syncs with the backend state.
    -   **Config UI**: Visual editor for `config.yaml`.

### Data Flow
1.  **Config**: User defines repositories in `config.yaml`.
2.  **Init**: Core initializes enabled plugins (e.g., GitHub) based on config.
3.  **Fetch**: Plugin polls external API (e.g., GitHub REST API) or listens for webhooks.
4.  **Normalize**: Plugin converts external data to `WorkflowStream` / `Run` structs.
5.  **Push**: Plugin calls `CoreApi::update_workflow_stream()`.
6.  **Broadcast**: Core emits event to Frontend.
7.  **Render**: Frontend updates the dashboard.

---

## 2. Plugin API Definition

The plugin system is designed around two main traits: `DataProvider` (implemented by the plugin) and `CoreApi` (provided to the plugin).

### Data Models

Plugins must normalize data into these shared structures:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub url: String,
    pub targets: Vec<Target>, // e.g., Branch("main"), PullRequest(...)
    pub workflow_streams: Vec<WorkflowStream>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStream {
    pub id: String,         // Unique ID (e.g., filename or workflow ID)
    pub name: String,       // Display name
    pub runs: Vec<Run>,     // History of runs
    pub current_run: Option<Run>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: String,
    pub name: String,
    pub status: RunStatus, // Success, Failed, Running, etc.
    pub url: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
```

### The `DataProvider` Trait

Every plugin must implement this trait to be registered with the core.

```rust
#[async_trait]
pub trait DataProvider: Send + Sync {
    /// Unique identifier for the provider (e.g., "github")
    fn name(&self) -> &'static str;
    
    /// Validate and authenticate using the provider-specific config section
    async fn authenticate(&mut self, config: &serde_json::Value) -> Result<(), ProviderError>;
    
    /// Start the monitoring loop.
    /// `core_api` is used to push updates back to the main application.
    async fn start_monitoring(&mut self, core_api: Box<dyn CoreApi>) -> Result<(), ProviderError>;
    
    /// Stop all background tasks.
    async fn stop_monitoring(&mut self) -> Result<(), ProviderError>;
}
```

### The `CoreApi` Trait

Passed to plugins during `start_monitoring`.

```rust
#[async_trait]
pub trait CoreApi: Send + Sync {
    /// Update the state of a repository (targets, metadata)
    async fn update_repository(&self, repository: Repository) -> Result<(), CoreError>;
    
    /// Push a new or updated workflow stream state
    async fn update_workflow_stream(&self, repo_id: &str, stream: WorkflowStream) -> Result<(), CoreError>;
    
    /// Send a system notification (e.g., "Build Failed")
    async fn notify(&self, notification: Notification) -> Result<(), CoreError>;
}
```

---

## 3. GitHub Plugin Definition

The GitHub Provider is the reference implementation of a `DataProvider`.

### Configuration
Uses the `github` key in `config.yaml`.

```yaml
github:
  token_env: "GITHUB_TOKEN" # Env var containing the PAT
  api_base_url: "https://api.github.com"
  poll_interval: 30         # Seconds between checks
```

### Implementation Details

1.  **Authentication**:
    -   Reads `token_env` from environment variables.
    -   Initializes an `octocrab` client instance.

2.  **Monitoring Strategy (Polling)**:
    -   Spawns a Tokio background task.
    -   Iterates through all configured repositories.
    -   **Endpoints Used**:
        -   `GET /repos/{owner}/{repo}/actions/workflows`: Discover workflows.
        -   `GET /repos/{owner}/{repo}/actions/runs`: Fetch latest statuses.
    -   **Rate Limiting**: Checks `x-ratelimit-remaining` headers and adjusts polling if necessary.

3.  **Data Mapping**:
    -   **Repository**: Maps `owner/repo` string.
    -   **WorkflowStream**: Maps to a `.yml` workflow file.
    -   **Run**: Maps individual workflow execution.
    -   **Target**: Filters runs by `head_branch` to match configured branches (e.g., "main").
