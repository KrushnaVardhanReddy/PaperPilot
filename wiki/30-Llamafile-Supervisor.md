# Llamafile Supervisor

The `LlamafileSupervisor` is a process manager dedicated to handling a standalone local `.llamafile` LLM. PaperPilot utilizes it to support 1-Click zero-install local inference out of the box, ensuring the end-user has a seamless experience operating offline models without complicated background dependency setups.

## Key Components

### `LlamafileSupervisor`
A structure encapsulating the configuration and the child process handle (via `std::process::Child`).

```rust
pub struct LlamafileSupervisor {
    model_path: PathBuf,
    port: u16,
    child: Arc<Mutex<Option<Child>>>,
    http_client: reqwest::Client,
}
```

### `SupervisorStatus`
A snapshot status payload useful for frontend display:
```rust
pub struct SupervisorStatus {
    pub is_running: bool,
    pub pid: Option<u32>,
    pub port: u16,
    pub model_path: PathBuf,
}
```

## Lifecycle Flow

1. **Initialization (`new`)**: Prepare the module with the `model_path` to the binary and the targeted `port`.
2. **Execution (`start`)**:
   - Checks if a server is already attached to the port. If yes, it shortcuts and assumes success.
   - Adjusts OS-level execute permissions for the binary on macOS/Linux.
   - Spawns the process using command line flags: `--server --jinja --port <port> --host 127.0.0.1 -ngl 999`.
   - Polls endpoints (`/health` and `/v1/models`) with exponential backoff until a 200 response ensures readiness.
3. **Observation (`status`)**: Used during runtime to assert health and active process IDs.
4. **Termination (`stop` / `Drop`)**: Kills and cleans up the child process aggressively. The `Drop` implementation ensures this happens synchronously if the main PaperPilot program crashes or exits suddenly.

## Best Practices
- **Never interact with the raw Child handle directly**: Use the async `.start()` and sync `.stop()` APIs provided.
- **Port Reuse**: Always assume that an already established local port mapping means a previous session is still alive or an external instance has been requested.
