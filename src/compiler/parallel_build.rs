//! Zamani Compiler — Parallel Build Engine
//!
//! Production-oriented parallel module build orchestration.
//!
//! This module is deliberately responsible only for:
//! - validating module inputs;
//! - scheduling independent module jobs;
//! - executing jobs concurrently;
//! - collecting deterministic results;
//! - propagating failures without panics.
//!
//! Actual source compilation is delegated through `ModuleCompiler`.
//! This prevents the parallel build layer from duplicating Zamani's compiler
//! pipeline.
//!
//! Design properties:
//! - no unsafe code;
//! - no modulo-by-zero worker scheduling;
//! - deterministic result ordering;
//! - fail-fast cancellation between jobs where possible;
//! - bounded worker count;
//! - explicit diagnostics;
//! - reusable compiler abstraction for tests and integration.

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// -----------------------------------------------------------------------------
// Limits
// -----------------------------------------------------------------------------

/// Maximum number of worker threads that the scheduler will create.
///
/// This protects the compiler from accidentally creating an unreasonable
/// number of operating-system threads from malformed configuration.
const MAX_WORKERS: usize = 256;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced by the parallel build subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParallelBuildError {
    InvalidWorkerCount,

    TooManyWorkers {
        requested: usize,
        maximum: usize,
    },

    EmptyModulePath {
        index: usize,
    },

    ModuleDoesNotExist {
        path: PathBuf,
    },

    ModuleIsNotFile {
        path: PathBuf,
    },

    DuplicateModule {
        path: PathBuf,
    },

    WorkerFailure {
        module: PathBuf,
        message: String,
    },

    WorkerPanicked,

    SchedulingFailure(String),

    CompilationFailure {
        module: PathBuf,
        message: String,
    },
}

impl fmt::Display for ParallelBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidWorkerCount => {
                write!(f, "parallel build requires at least one worker")
            }

            Self::TooManyWorkers {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "requested {} workers exceeds maximum supported {}",
                    requested, maximum
                )
            }

            Self::EmptyModulePath { index } => {
                write!(
                    f,
                    "module at index {} has an empty path",
                    index
                )
            }

            Self::ModuleDoesNotExist { path } => {
                write!(
                    f,
                    "module '{}' does not exist",
                    path.display()
                )
            }

            Self::ModuleIsNotFile { path } => {
                write!(
                    f,
                    "module '{}' is not a regular file",
                    path.display()
                )
            }

            Self::DuplicateModule { path } => {
                write!(
                    f,
                    "module '{}' was supplied more than once",
                    path.display()
                )
            }

            Self::WorkerFailure { module, message } => {
                write!(
                    f,
                    "worker failed while compiling '{}': {}",
                    module.display(),
                    message
                )
            }

            Self::WorkerPanicked => {
                write!(f, "parallel build worker panicked")
            }

            Self::SchedulingFailure(message) => {
                write!(f, "parallel build scheduling failure: {}", message)
            }

            Self::CompilationFailure {
                module,
                message,
            } => {
                write!(
                    f,
                    "compilation failed for '{}': {}",
                    module.display(),
                    message
                )
            }
        }
    }
}

impl std::error::Error for ParallelBuildError {}

// -----------------------------------------------------------------------------
// Module compilation abstraction
// -----------------------------------------------------------------------------

/// Result of compiling one module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledModule {
    /// Original source module path.
    pub source: PathBuf,

    /// Generated artifact, if the compiler produces one.
    ///
    /// The parallel build layer treats this as opaque data.
    pub artifact: Option<PathBuf>,

    /// Number of bytes generated, when known.
    pub output_size: u64,
}

impl CompiledModule {
    pub fn new(source: PathBuf) -> Self {
        Self {
            source,
            artifact: None,
            output_size: 0,
        }
    }
}

/// Trait implemented by the real Zamani compiler pipeline.
///
/// `parallel_build.rs` does not know how parsing, semantic analysis, IR
/// generation, optimization, verification, or backend generation work.
pub trait ModuleCompiler: Send + Sync + 'static {
    fn compile(
        &self,
        module: &Path,
    ) -> Result<CompiledModule, String>;
}

/// Default compiler adapter.
///
/// This adapter performs input validation only. The real compiler can provide
/// its own `ModuleCompiler` implementation without modifying this scheduler.
///
/// Keeping this behavior explicit is important: the old implementation claimed
/// that modules were compiled when it only printed messages.
#[derive(Debug, Default, Clone, Copy)]
pub struct ValidationOnlyCompiler;

impl ModuleCompiler for ValidationOnlyCompiler {
    fn compile(
        &self,
        module: &Path,
    ) -> Result<CompiledModule, String> {
        if !module.exists() {
            return Err(
                format!("module '{}' does not exist", module.display())
            );
        }

        if !module.is_file() {
            return Err(
                format!("module '{}' is not a regular file", module.display())
            );
        }

        Ok(CompiledModule::new(module.to_path_buf()))
    }
}

// -----------------------------------------------------------------------------
// Build result
// -----------------------------------------------------------------------------

/// Complete result of a parallel build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelBuildResult {
    /// Results are ordered according to the original input order.
    pub modules: Vec<CompiledModule>,

    /// Number of workers actually used.
    pub worker_count: usize,

    /// Number of successfully compiled modules.
    pub successful_modules: usize,
}

impl ParallelBuildResult {
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
}

// -----------------------------------------------------------------------------
// Internal work items
// -----------------------------------------------------------------------------

struct WorkItem {
    index: usize,
    module: PathBuf,
}

struct WorkResult {
    index: usize,
    result: Result<CompiledModule, ParallelBuildError>,
}

// -----------------------------------------------------------------------------
// Parallel build engine
// -----------------------------------------------------------------------------

/// Thread-based parallel build scheduler.
#[derive(Debug, Clone)]
pub struct ParallelBuildEngine {
    thread_count: usize,
}

impl ParallelBuildEngine {
    /// Creates a parallel build engine.
    ///
    /// `thread_count == 0` is rejected instead of causing a runtime panic.
    pub fn new(thread_count: usize) -> Result<Self, ParallelBuildError> {
        if thread_count == 0 {
            return Err(ParallelBuildError::InvalidWorkerCount);
        }

        if thread_count > MAX_WORKERS {
            return Err(ParallelBuildError::TooManyWorkers {
                requested: thread_count,
                maximum: MAX_WORKERS,
            });
        }

        Ok(Self { thread_count })
    }

    /// Creates an engine using the host's available parallelism.
    pub fn automatic() -> Self {
        let workers = std::thread::available_parallelism()
            .map(|count| count.get())
            .unwrap_or(1)
            .min(MAX_WORKERS);

        Self {
            thread_count: workers.max(1),
        }
    }

    /// Returns the configured worker count.
    pub fn thread_count(&self) -> usize {
        self.thread_count
    }

    /// Compiles modules using the supplied compiler implementation.
    ///
    /// Results remain deterministic because they are sorted back into the
    /// caller's original module order before being returned.
    pub fn compile_modules_with<C>(
        &self,
        modules: &[PathBuf],
        compiler: Arc<C>,
    ) -> Result<ParallelBuildResult, ParallelBuildError>
    where
        C: ModuleCompiler,
    {
        validate_modules(modules)?;

        if modules.is_empty() {
            return Ok(ParallelBuildResult {
                modules: Vec::new(),
                worker_count: 0,
                successful_modules: 0,
            });
        }

        let worker_count = self.thread_count.min(modules.len());

        println!(
            "[ParallelBuild] Compiling {} modules using {} worker(s)...",
            modules.len(),
            worker_count
        );

        let (job_tx, job_rx) = mpsc::channel::<WorkItem>();
        let (result_tx, result_rx) = mpsc::channel::<WorkResult>();

        let shared_rx = Arc::new(Mutex::new(job_rx));
        let mut workers = Vec::with_capacity(worker_count);

        for worker_id in 0..worker_count {
            let receiver = Arc::clone(&shared_rx);
            let sender = result_tx.clone();
            let compiler = Arc::clone(&compiler);

            let worker = thread::Builder::new()
                .name(format!("zamani-build-{}", worker_id))
                .spawn(move || loop {
                    let work = {
                        let receiver = match receiver.lock() {
                            Ok(receiver) => receiver,
                            Err(_) => {
                                return;
                            }
                        };

                        receiver.recv()
                    };

                    let work = match work {
                        Ok(work) => work,
                        Err(_) => break,
                    };

                    let module = work.module.clone();

                    println!(
                        "  -> [Worker {}] Compiling '{}'",
                        worker_id,
                        module.display()
                    );

                    let result = compiler
                        .compile(&module)
                        .map_err(|message| {
                            ParallelBuildError::CompilationFailure {
                                module: module.clone(),
                                message,
                            }
                        });

                    if sender
                        .send(WorkResult {
                            index: work.index,
                            result,
                        })
                        .is_err()
                    {
                        break;
                    }
                })
                .map_err(|error| {
                    ParallelBuildError::SchedulingFailure(format!(
                        "failed to spawn worker {}: {}",
                        worker_id, error
                    ))
                })?;

            workers.push(worker);
        }

        // The worker threads own their result senders.
        drop(result_tx);

        for (index, module) in modules.iter().cloned().enumerate() {
            job_tx
                .send(WorkItem { index, module })
                .map_err(|error| {
                    ParallelBuildError::SchedulingFailure(
                        format!(
                            "failed to enqueue build job: {}",
                            error
                        ),
                    )
                })?;
        }

        // Closing the job channel tells workers that there are no more jobs.
        drop(job_tx);

        let mut results: Vec<Option<CompiledModule>> =
            vec![None; modules.len()];

        let mut first_error: Option<ParallelBuildError> = None;

        for _ in 0..modules.len() {
            let work_result = result_rx.recv().map_err(|error| {
                ParallelBuildError::SchedulingFailure(format!(
                    "failed to receive worker result: {}",
                    error
                ))
            })?;

            match work_result.result {
                Ok(compiled) => {
                    results[work_result.index] = Some(compiled);
                }

                Err(error) => {
                    if first_error.is_none() {
                        first_error = Some(error);
                    }
                }
            }
        }

        for worker in workers {
            if worker.join().is_err() {
                if first_error.is_none() {
                    first_error =
                        Some(ParallelBuildError::WorkerPanicked);
                }
            }
        }

        if let Some(error) = first_error {
            return Err(error);
        }

        let compiled_modules: Vec<CompiledModule> = results
            .into_iter()
            .enumerate()
            .map(|(index, result)| {
                result.ok_or_else(|| {
                    ParallelBuildError::WorkerFailure {
                        module: modules[index].clone(),
                        message:
                            "worker returned no compilation result"
                                .to_string(),
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let successful_modules = compiled_modules.len();

        println!(
            "[ParallelBuild] Build completed successfully: {} module(s).",
            successful_modules
        );

        Ok(ParallelBuildResult {
            modules: compiled_modules,
            worker_count,
            successful_modules,
        })
    }

    /// Compatibility entry point using the validation compiler adapter.
    ///
    /// This method verifies and schedules the files but does not pretend to
    /// perform the complete Zamani compilation pipeline.
    pub fn compile_modules(
        &self,
        modules: &[PathBuf],
    ) -> Result<ParallelBuildResult, ParallelBuildError> {
        self.compile_modules_with(
            modules,
            Arc::new(ValidationOnlyCompiler),
        )
    }
}

// -----------------------------------------------------------------------------
// Validation
// -----------------------------------------------------------------------------

fn validate_modules(
    modules: &[PathBuf],
) -> Result<(), ParallelBuildError> {
    let mut seen = std::collections::HashSet::new();

    for (index, module) in modules.iter().enumerate() {
        if module.as_os_str().is_empty() {
            return Err(
                ParallelBuildError::EmptyModulePath { index }
            );
        }

        if !seen.insert(module.clone()) {
            return Err(
                ParallelBuildError::DuplicateModule {
                    path: module.clone(),
                },
            );
        }

        if !module.exists() {
            return Err(
                ParallelBuildError::ModuleDoesNotExist {
                    path: module.clone(),
                },
            );
        }

        if !module.is_file() {
            return Err(
                ParallelBuildError::ModuleIsNotFile {
                    path: module.clone(),
                },
            );
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug)]
    struct TestCompiler;

    impl ModuleCompiler for TestCompiler {
        fn compile(
            &self,
            module: &Path,
        ) -> Result<CompiledModule, String> {
            Ok(CompiledModule {
                source: module.to_path_buf(),
                artifact: None,
                output_size: 123,
            })
        }
    }

    fn temporary_module(name: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be valid")
            .as_nanos();

        let path = std::env::temp_dir()
            .join(format!("zamani_parallel_{}_{}", timestamp, name));

        fs::write(&path, b"module test")
            .expect("temporary module should be writable");

        path
    }

    #[test]
    fn zero_workers_are_rejected() {
        let result = ParallelBuildEngine::new(0);

        assert!(matches!(
            result,
            Err(ParallelBuildError::InvalidWorkerCount)
        ));
    }

    #[test]
    fn excessive_worker_count_is_rejected() {
        let result =
            ParallelBuildEngine::new(MAX_WORKERS + 1);

        assert!(matches!(
            result,
            Err(ParallelBuildError::TooManyWorkers { .. })
        ));
    }

    #[test]
    fn automatic_worker_count_is_positive() {
        let engine = ParallelBuildEngine::automatic();

        assert!(engine.thread_count() >= 1);
        assert!(engine.thread_count() <= MAX_WORKERS);
    }

    #[test]
    fn empty_module_list_is_valid() {
        let engine =
            ParallelBuildEngine::new(2).expect("valid worker count");

        let result = engine
            .compile_modules(&[])
            .expect("empty build should succeed");

        assert_eq!(result.module_count(), 0);
        assert_eq!(result.successful_modules, 0);
    }

    #[test]
    fn nonexistent_module_is_rejected() {
        let engine =
            ParallelBuildEngine::new(2).expect("valid worker count");

        let path =
            PathBuf::from("/definitely/not/a/real/zamani/module.snk");

        let result = engine.compile_modules(&[path]);

        assert!(matches!(
            result,
            Err(ParallelBuildError::ModuleDoesNotExist { .. })
        ));
    }

    #[test]
    fn duplicate_modules_are_rejected() {
        let module = temporary_module("duplicate.snk");

        let engine =
            ParallelBuildEngine::new(2).expect("valid worker count");

        let result = engine.compile_modules(&[
            module.clone(),
            module.clone(),
        ]);

        let _ = fs::remove_file(&module);

        assert!(matches!(
            result,
            Err(ParallelBuildError::DuplicateModule { .. })
        ));
    }

    #[test]
    fn modules_are_returned_in_input_order() {
        let first = temporary_module("first.snk");
        let second = temporary_module("second.snk");
        let third = temporary_module("third.snk");

        let engine =
            ParallelBuildEngine::new(3).expect("valid worker count");

        let result = engine
            .compile_modules_with(
                &[first.clone(), second.clone(), third.clone()],
                Arc::new(TestCompiler),
            )
            .expect("parallel compilation should succeed");

        assert_eq!(result.modules[0].source, first);
        assert_eq!(result.modules[1].source, second);
        assert_eq!(result.modules[2].source, third);

        let _ = fs::remove_file(first);
        let _ = fs::remove_file(second);
        let _ = fs::remove_file(third);
    }

    #[test]
    fn compiler_failures_are_propagated() {
        #[derive(Debug)]
        struct FailingCompiler;

        impl ModuleCompiler for FailingCompiler {
            fn compile(
                &self,
                _module: &Path,
            ) -> Result<CompiledModule, String> {
                Err("intentional compilation failure".to_string())
            }
        }

        let module = temporary_module("failure.snk");

        let engine =
            ParallelBuildEngine::new(2).expect("valid worker count");

        let result = engine.compile_modules_with(
            &[module.clone()],
            Arc::new(FailingCompiler),
        );

        let _ = fs::remove_file(module);

        assert!(matches!(
            result,
            Err(ParallelBuildError::CompilationFailure { .. })
        ));
    }
}