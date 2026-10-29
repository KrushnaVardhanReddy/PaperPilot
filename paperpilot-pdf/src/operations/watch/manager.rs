use crate::operations::output_naming::OutputNameTemplate;
use crate::operations::workflow::{run_workflow_bytes_detailed, validate_workflow_definition};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::workflow::{SavedWorkflow, get_preset_by_id, parse_workflow};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, mpsc};
static RUN_ID: AtomicU64 = AtomicU64::new(1);
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchConfig {
    pub directory: PathBuf,
    /// Preset ID, recipe path, or serialized workflow. Never included in events.
    pub recipe: String,
    pub output_dir: Option<PathBuf>,
    #[serde(default = "default_settle")]
    pub settle_delay_ms: u64,
    pub move_original: Option<PathBuf>,
    pub error_dir: Option<PathBuf>,
    #[serde(default)]
    pub once: bool,
}
fn default_settle() -> u64 {
    500
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WatchStatus {
    Watching,
    Stopping,
    Stopped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchSnapshot {
    pub run_id: u64,
    pub id: String,
    pub directory: PathBuf,
    pub output_dir: PathBuf,
    pub recipe_name: String,
    pub status: WatchStatus,
    pub succeeded: u64,
    pub failed: u64,
    pub last_processed_file: Option<PathBuf>,
    pub last_processed_at: Option<String>,
    pub last_error: Option<String>,
    pub last_output: Option<PathBuf>,
    pub last_receipt: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEvent {
    pub kind: String,
    pub watcher: WatchSnapshot,
    pub file: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub receipt: Option<PathBuf>,
    pub error: Option<String>,
}
pub type EventSink = Arc<dyn Fn(WatchEvent) + Send + Sync>;
struct ManagedWatch {
    finished: Arc<AtomicBool>,
    snapshot: Arc<Mutex<WatchSnapshot>>,
    cancel: CancellationToken,
    thread: Arc<Mutex<Option<JoinHandle<()>>>>,
}

#[derive(Default)]
pub struct WatcherManager {
    entries: Mutex<HashMap<String, ManagedWatch>>,
    shutdown_lock: Mutex<()>,
    shutting_down: AtomicBool,
}
fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}
fn invalid(message: impl Into<String>) -> PdfError {
    PdfError::InvalidInput(message.into())
}

impl WatcherManager {
    /// Returns only after notify has registered the real OS watcher. No runtime
    /// thread is blocked by the polling loop; it owns a dedicated worker thread.
    pub fn start(
        &self,
        id: String,
        config: WatchConfig,
        sink: EventSink,
    ) -> OperationResult<WatchSnapshot> {
        self.start_with_naming(id, config, None, sink)
    }
    pub fn start_with_naming(
        &self,
        id: String,
        config: WatchConfig,
        name_template: Option<String>,
        sink: EventSink,
    ) -> OperationResult<WatchSnapshot> {
        let naming = name_template
            .as_deref()
            .map(OutputNameTemplate::parse)
            .transpose()?;
        if id.is_empty()
            || id.len() > 80
            || !id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(invalid("Invalid watcher ID"));
        }
        let prepared = PreparedWatch::new(config, naming)?;
        let mut entries = lock(&self.entries);
        if self.shutting_down.load(Ordering::SeqCst) {
            return Err(invalid("Watchers are shutting down"));
        }
        if let Some(existing) = entries.get(&id) {
            if !existing.finished.load(Ordering::SeqCst)
                || matches!(
                    lock(&existing.snapshot).status,
                    WatchStatus::Watching | WatchStatus::Stopping
                )
            {
                return Err(invalid("This watcher is already running"));
            }
        }
        for (other_id, entry) in entries.iter() {
            let s = lock(&entry.snapshot);
            if other_id != &id
                && matches!(s.status, WatchStatus::Watching | WatchStatus::Stopping)
                && (s.directory == prepared.directory
                    || s.output_dir == prepared.directory
                    || s.directory == prepared.output)
            {
                return Err(invalid(
                    "This folder is already watched or is another active watcher's output",
                ));
            }
        }
        let snapshot = Arc::new(Mutex::new(WatchSnapshot {
            run_id: RUN_ID.fetch_add(1, Ordering::Relaxed),
            id: id.clone(),
            directory: prepared.directory.clone(),
            output_dir: prepared.output.clone(),
            recipe_name: prepared.workflow.name.clone(),
            status: WatchStatus::Watching,
            succeeded: 0,
            failed: 0,
            last_processed_file: None,
            last_processed_at: None,
            last_error: None,
            last_output: None,
            last_receipt: None,
        }));
        let cancel = CancellationToken::new();
        let worker_cancel = cancel.clone();
        let worker_snapshot = snapshot.clone();
        let finished = Arc::new(AtomicBool::new(false));
        let worker_finished = finished.clone();
        let (ready_tx, ready_rx) = mpsc::sync_channel(1);
        let thread = std::thread::Builder::new()
            .name(format!("paperpilot-watch-{id}"))
            .spawn(move || {
                let result = prepared.run(&worker_cancel, &worker_snapshot, &sink, ready_tx);
                let mut s = lock(&worker_snapshot);
                s.status = if result.is_ok() {
                    WatchStatus::Stopped
                } else {
                    WatchStatus::Error
                };
                if let Err(error) = result {
                    s.last_error = Some(error.to_string());
                }
                let event = WatchEvent {
                    kind: "stopped".into(),
                    watcher: s.clone(),
                    file: None,
                    output: None,
                    receipt: None,
                    error: s.last_error.clone(),
                };
                drop(s);
                sink(event);
                worker_finished.store(true, Ordering::SeqCst);
            })?;
        match ready_rx.recv_timeout(Duration::from_secs(10)) {
            Ok(Ok(())) => {}
            Ok(Err(message)) => {
                cancel.cancel();
                let _ = thread.join();
                return Err(invalid(message));
            }
            Err(_) => {
                cancel.cancel();
                return Err(invalid("Watcher startup did not complete"));
            }
        }
        entries.insert(
            id,
            ManagedWatch {
                finished,
                snapshot: snapshot.clone(),
                cancel,
                thread: Arc::new(Mutex::new(Some(thread))),
            },
        );
        Ok(lock(&snapshot).clone())
    }

    pub fn list(&self) -> Vec<WatchSnapshot> {
        let mut snapshots: Vec<_> = lock(&self.entries)
            .values()
            .map(|entry| lock(&entry.snapshot).clone())
            .collect();
        snapshots.sort_by(|a, b| a.id.cmp(&b.id));
        snapshots
    }
    /// Waits for an in-flight PDF operation and joins the worker. After return no
    /// further files are processed and no late success events can be emitted.
    pub fn stop(&self, id: &str) -> OperationResult<WatchSnapshot> {
        let (thread, snapshot) = {
            let mut entries = lock(&self.entries);
            let entry = entries
                .get_mut(id)
                .ok_or_else(|| invalid("Watcher not found"))?;
            entry.cancel.cancel();
            {
                let mut s = lock(&entry.snapshot);
                if s.status == WatchStatus::Watching {
                    s.status = WatchStatus::Stopping;
                }
            }
            (entry.thread.clone(), entry.snapshot.clone())
        };
        let mut join = lock(&thread);
        if let Some(thread) = join.take() {
            thread
                .join()
                .map_err(|_| PdfError::Other("Watcher thread failed".into()))?;
        }
        Ok(lock(&snapshot).clone())
    }
    pub fn remove(&self, id: &str) -> OperationResult<()> {
        let stopped = self.stop(id)?;
        let mut entries = lock(&self.entries);
        if entries
            .get(id)
            .is_some_and(|entry| lock(&entry.snapshot).run_id != stopped.run_id)
        {
            return Err(invalid("Watcher restarted before removal; stop it again"));
        }
        entries.remove(id);
        Ok(())
    }
    pub fn shutdown(&self) {
        let _shutdown = lock(&self.shutdown_lock);
        self.shutting_down.store(true, Ordering::SeqCst);
        let entries = std::mem::take(&mut *lock(&self.entries));
        for entry in entries.values() {
            entry.cancel.cancel();
        }
        for (_, entry) in entries {
            if let Some(thread) = lock(&entry.thread).take() {
                let _ = thread.join();
            }
        }
        self.shutting_down.store(false, Ordering::SeqCst);
    }
}
impl Drop for WatcherManager {
    fn drop(&mut self) {
        self.shutdown();
    }
}

pub fn resolve_watch_recipe(source: &str) -> OperationResult<SavedWorkflow> {
    let workflow = if source.trim_start().starts_with('{') || source.contains('\n') {
        parse_workflow(source)?
    } else if let Some(preset) = get_preset_by_id(source) {
        preset
    } else {
        parse_workflow(&std::fs::read_to_string(source)?)?
    };
    validate_workflow_definition(&workflow)?;
    Ok(workflow)
}

struct PreparedWatch {
    naming: Option<OutputNameTemplate>,
    config: WatchConfig,
    directory: PathBuf,
    output: PathBuf,
    errors: PathBuf,
    move_to: Option<PathBuf>,
    workflow: SavedWorkflow,
}
impl PreparedWatch {
    fn new(config: WatchConfig, naming: Option<OutputNameTemplate>) -> OperationResult<Self> {
        if !(50..=60000).contains(&config.settle_delay_ms) {
            return Err(invalid(
                "Settle delay must be between 50 and 60000 milliseconds",
            ));
        }
        let workflow = resolve_watch_recipe(&config.recipe)?;
        let directory = config.directory.canonicalize()?;
        if !directory.is_dir() {
            return Err(invalid("Watch path must be a directory"));
        }
        fn ensure(path: PathBuf) -> OperationResult<PathBuf> {
            std::fs::create_dir_all(&path)?;
            Ok(path.canonicalize()?)
        }
        let output = ensure(
            config
                .output_dir
                .clone()
                .unwrap_or_else(|| directory.join("processed")),
        )?;
        let errors = ensure(
            config
                .error_dir
                .clone()
                .unwrap_or_else(|| directory.join(".error")),
        )?;
        let move_to = config.move_original.clone().map(ensure).transpose()?;
        if output == directory
            || errors == directory
            || output == errors
            || move_to
                .as_ref()
                .is_some_and(|p| p == &directory || p == &output || p == &errors)
        {
            return Err(invalid(
                "Input, output, error and archive folders must be different",
            ));
        }
        Ok(Self {
            naming,
            config,
            directory,
            output,
            errors,
            move_to,
            workflow,
        })
    }

    fn run(
        self,
        cancel: &CancellationToken,
        snapshot: &Arc<Mutex<WatchSnapshot>>,
        sink: &EventSink,
        ready: mpsc::SyncSender<Result<(), String>>,
    ) -> OperationResult<()> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
            Ok(watcher) => watcher,
            Err(e) => {
                let _ = ready.send(Err(e.to_string()));
                return Err(invalid(e.to_string()));
            }
        };
        if !self.config.once {
            if let Err(e) = watcher.watch(&self.directory, RecursiveMode::NonRecursive) {
                let _ = ready.send(Err(e.to_string()));
                return Err(invalid(e.to_string()));
            }
        }
        if ready.send(Ok(())).is_err() || cancel.is_cancelled() {
            return Ok(());
        }
        sink(WatchEvent {
            kind: "started".into(),
            watcher: lock(snapshot).clone(),
            file: None,
            output: None,
            receipt: None,
            error: None,
        });
        let settle = Duration::from_millis(self.config.settle_delay_ms);
        let mut pending = HashMap::<PathBuf, (Instant, u64)>::new();
        let mut attempted = HashMap::<PathBuf, String>::new();
        let mut read_retries = HashMap::<PathBuf, u8>::new();
        for file in std::fs::read_dir(&self.directory)? {
            let path = file?.path();
            if eligible(&path, &self.directory) {
                pending.insert(path, (Instant::now(), 0));
            }
        }
        loop {
            if cancel.is_cancelled() {
                break;
            }
            if self.config.once && pending.is_empty() {
                break;
            }
            match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(Ok(event))
                    if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) =>
                {
                    for path in event.paths {
                        if eligible(&path, &self.directory) {
                            pending.insert(path, (Instant::now(), 0));
                        }
                    }
                }
                Ok(Err(e)) => {
                    return Err(PdfError::Other(format!("Filesystem watcher failed: {e}")));
                }
                _ => {}
            }
            let mut files: Vec<_> = pending
                .iter()
                .filter(|(_, (seen, _))| seen.elapsed() >= settle)
                .map(|(p, _)| p.clone())
                .collect();
            files.sort();
            for path in files {
                if cancel.is_cancelled() {
                    break;
                }
                let prior_size = pending.get(&path).map(|(_, size)| *size).unwrap_or(0);
                let size = match std::fs::metadata(&path) {
                    Ok(m) => m.len(),
                    Err(_) => {
                        pending.remove(&path);
                        continue;
                    }
                };
                if prior_size != size {
                    pending.insert(path, (Instant::now(), size));
                    continue;
                }
                let bytes = match std::fs::read(&path) {
                    Ok(b) => b,
                    Err(error) => {
                        let retry = read_retries.entry(path.clone()).or_default();
                        *retry += 1;
                        if *retry >= 20 {
                            return Err(PdfError::IoError(error));
                        }
                        pending.insert(path, (Instant::now(), size));
                        continue;
                    }
                };
                read_retries.remove(&path);
                pending.remove(&path);
                let hash = sha256(&bytes);
                if attempted.get(&path) == Some(&hash) {
                    continue;
                }
                attempted.insert(path.clone(), hash.clone());
                self.process(&path, &bytes, &hash, snapshot, sink);
            }
        }
        Ok(())
    }

    fn process(
        &self,
        path: &Path,
        bytes: &[u8],
        hash: &str,
        snapshot: &Arc<Mutex<WatchSnapshot>>,
        sink: &EventSink,
    ) {
        let mut output = None;
        let mut receipt = None;
        let result = (|| -> OperationResult<()> {
            let original_name = path
                .file_name()
                .ok_or_else(|| invalid("Missing file name"))?
                .to_string_lossy();
            let index = {
                let s = lock(snapshot);
                s.succeeded + s.failed + 1
            };
            let name = match &self.naming {
                Some(template) => template.render_pdf(path, bytes, chrono::Utc::now(), index)?,
                None => original_name.to_string(),
            };
            let (result, report) =
                run_workflow_bytes_detailed(bytes, &self.workflow).map_err(|f| f.cause)?;
            let out = publish(&self.output, &name, &result)?;
            output = Some(out.clone());
            let audit = serde_json::json!({ "timestamp": chrono::Utc::now().to_rfc3339(), "file": path, "sha256": hash, "output": out, "output_sha256": sha256(&result), "success":true, "report": report });
            if let Some(archive) = &self.move_to {
                // Copy without replacement before removing the original; a failed
                // archive never destroys the source or overwrites an earlier file.
                publish(archive, &original_name, bytes)?;
                std::fs::remove_file(path)?;
            }
            receipt = Some(publish(
                &self.output,
                &format!(
                    "{}.receipt.json",
                    out.file_name().unwrap().to_string_lossy()
                ),
                &serde_json::to_vec_pretty(&audit).map_err(|e| PdfError::Other(e.to_string()))?,
            )?);
            Ok(())
        })();
        let error = result.as_ref().err().map(ToString::to_string);
        if let Some(message) = &error {
            let audit = serde_json::json!({"timestamp":chrono::Utc::now().to_rfc3339(),"file":path,"sha256":hash,"success":false,"error":message,"output":output});
            match publish(
                &self.errors,
                &format!(
                    "{}.receipt.json",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ),
                &serde_json::to_vec_pretty(&audit).unwrap_or_default(),
            ) {
                Ok(p) => receipt = Some(p),
                Err(e) => log::error!("Could not write failure receipt: {e}"),
            }
        }
        let mut s = lock(snapshot);
        if result.is_ok() {
            s.succeeded += 1;
        } else {
            s.failed += 1;
        }
        s.last_processed_file = Some(path.to_path_buf());
        s.last_processed_at = Some(chrono::Utc::now().to_rfc3339());
        s.last_error = error.clone();
        if output.is_some() {
            s.last_output = output.clone();
        }
        if receipt.is_some() {
            s.last_receipt = receipt.clone();
        }
        let event = WatchEvent {
            kind: if result.is_ok() {
                "file_processed"
            } else {
                "error"
            }
            .into(),
            watcher: s.clone(),
            file: Some(path.to_path_buf()),
            output,
            receipt,
            error,
        };
        drop(s);
        sink(event);
    }
}

fn eligible(path: &Path, root: &Path) -> bool {
    path.parent() == Some(root)
        && path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("pdf"))
        && std::fs::symlink_metadata(path).is_ok_and(|m| m.is_file() && !m.file_type().is_symlink())
}

// The output and receipt are reserved atomically; pre-existing files are never
// replaced, including when two processes race for the same file name.
fn publish(directory: &Path, name: &str, bytes: &[u8]) -> OperationResult<PathBuf> {
    let name = Path::new(name);
    let stem = name.file_stem().unwrap_or_default().to_string_lossy();
    let ext = name
        .extension()
        .map(|v| format!(".{}", v.to_string_lossy()))
        .unwrap_or_default();
    for index in 0..10000 {
        let candidate = directory.join(if index == 0 {
            name.to_string_lossy().to_string()
        } else {
            format!("{stem}-{index}{ext}")
        });
        let mut temp = tempfile::NamedTempFile::new_in(directory)?;
        temp.write_all(bytes)?;
        match temp.persist_noclobber(&candidate) {
            Ok(_) => return Ok(candidate),
            Err(e) if e.error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(PdfError::IoError(e.error)),
        }
    }
    Err(invalid("Output name collision limit exceeded"))
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// One process-wide runtime lets desktop commands, local REST and MCP share real
/// status and cancellation rather than maintaining unrelated placeholder lists.
pub fn shared_watcher_manager() -> Arc<WatcherManager> {
    static MANAGER: std::sync::LazyLock<Arc<WatcherManager>> =
        std::sync::LazyLock::new(|| Arc::new(WatcherManager::default()));
    MANAGER.clone()
}
