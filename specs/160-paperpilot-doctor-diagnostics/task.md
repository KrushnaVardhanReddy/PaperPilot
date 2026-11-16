# Task 160A: Implement `paperpilot doctor` Diagnostics Engine

## Checklist
- [ ] Create `crates/paperpilot-core/src/diagnostics.rs` containing diagnostic probe structs (`ProbeResult`, `DoctorReport`, `ProbeStatus`)
- [ ] Implement individual diagnostic probes:
  - [ ] Port 7823 bind check
  - [ ] Temp directory write/delete permissions
  - [ ] Local model directory check (`~/.cache/paperpilot/models`)
  - [ ] Disk space check on scratch volume
  - [ ] Zero-trace cleanup check
- [ ] Implement CLI subcommand `paperpilot doctor` in `crates/paperpilot-cli/src/main.rs` with colored ANSI terminal report
- [ ] Add `GET /health` and `GET /api/v1/doctor` to `crates/paperpilot-gateway`
- [ ] Add Tauri command `run_system_doctor` in `apps/desktop/src-tauri/src/lib.rs`
- [ ] Add "Run System Diagnostics" button & modal in `apps/desktop/src/lib/components/layout/SettingsPanel.svelte`
- [ ] Add unit test suite `test_doctor_probes.rs`
- [ ] Validate compilation and test pass with `cargo check -j 4`
