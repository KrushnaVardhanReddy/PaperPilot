# Watch Folder Daemon Report

## Implementation Details
- **Command**: `paperpilot watch <DIR>`
- **Debouncing**: Handled via standard channel timeouts and `Instant::now()` checks against a configurable `settle_delay_ms` threshold.
- **Event Engine**: `notify` v6.1 (pure Rust, cross-platform `RecommendedWatcher`).
- **In-process execution**: Re-uses `paperpilot-cli` core routing logic without spawning subprocesses.

## Test Verification
- Simulated file drop triggers `Create`/`Modify` events.
- Debouncer correctly waited the specified delay before executing.
- Processed output was generated in the correct output directory.

## Performance Metrics (Estimated)
- **Event Latency**: <1ms overhead on OS events via `notify`.
- **Memory Overhead**: Minimal (storing only active file paths in a `HashMap`).
- **Thread Profile**: 1 Main monitoring loop, 1 Tokio runtime for async tasks (shutdown/webhooks).

## CLI Example Tested
```bash
paperpilot watch /tmp/watch_in \
  --output /tmp/watch_out \
  --operation compress \
  --settle-delay-ms 100
```
