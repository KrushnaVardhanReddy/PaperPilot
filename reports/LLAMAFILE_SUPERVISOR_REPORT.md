# Llamafile Supervisor Report

## Overview
The `LlamafileSupervisor` module manages the lifecycle of the local standalone `.llamafile` execution within PaperPilot. It ensures smooth process spawning, initialization checking, port reuse, and robust termination.

## Features Verified

1. **Process Lifecycle & Guarantee of Cleanup:**
   - The supervisor safely spawns the binary and tracks the Child process ID (PID).
   - Upon being dropped (e.g., system termination or application exit), the `Drop` implementation safely calls `.stop()` which sends a `kill` signal (or `TerminateProcess` equivalent on Windows) to effectively ensure the process shuts down cleanly, preventing zombie processes. Tests successfully verified that dropping the supervisor handles stops the running binary.

2. **Initialization and Startup Polling:**
   - Uses exponential backoff to query `http://127.0.0.1:<port>/health` to ensure the process only resolves as ready when the model has fully loaded into memory.
   - Falls back to querying `http://127.0.0.1:<port>/v1/models` in case `/health` is unsupported by the underlying engine.

3. **Port Negotiation & Re-Use:**
   - Prevents port conflict errors. Before launching a new child process, the supervisor queries `http://127.0.0.1:<port>/v1/models`. If a service is already bound to that port and responding, the supervisor considers the resource as successfully launched and attaches to the existing session rather than throwing a duplicate binding error.

4. **Permissions Guard:**
   - Automatically injects executable permissions (`st_mode | 0o111`) into the target binary (on macOS/Linux systems) before starting the process via `std::os::unix::fs::PermissionsExt`.

## Testing Output
The suite `supervisor_tests.rs` validates all critical path scenarios successfully:
```
running 5 tests
test test_supervisor_port_conflict_reuse ... ok
test test_supervisor_start_and_stop ... ok
test test_supervisor_spawn_failed ... ok
test test_supervisor_drop_stops_process ... ok
test test_supervisor_timeout ... ok
test result: ok. 5 passed
```
