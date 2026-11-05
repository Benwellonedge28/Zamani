
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compiler Service API.
//!
//! Path: src/compiler/api/compiler.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provides a stable, reusable compiler-service interface for applications
//! using the Zamani compiler.
//!
//! The service coordinates compilation through an explicitly supplied
//! compilation driver. It does not implement another compiler pipeline.
//!
//! # Architectural ownership
//!
//! Existing repository components retain ownership of:
//!
//! - source parsing;
//! - AST construction;
//! - semantic analysis;
//! - ownership and borrowing analysis;
//! - canonical intermediate representations;
//! - optimization;
//! - IR verification;
//! - hardware capability analysis;
//! - hybrid composition;
//! - backend code generation;
//! - linking;
//! - runtime execution.
//!
//! This module owns only:
//!
//! - compiler-service configuration association;
//! - compilation-driver invocation;
//! - source-file input handling;
//! - stable service-level integration;
//! - thread-safe sharing of immutable configuration;
//! - preservation of compilation results and errors.
//!
//! # POCO-REAF
//!
//! Zamani supports:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This API does not bind a source program to the host machine.
//!
//! Target selection is explicitly supplied through the existing
//! CompilerConfig contract.
//!
//! The service introduces no universal hardware limits.
//!
//! Practical compilation remains bounded only by:
//!
//! - available memory;
//! - available processing resources;
//! - operating-system constraints;
//! - explicitly configured operational budgets;
//! - capabilities of the selected compilation driver;
//! - capabilities of the requested target.
//!
//! # Integration
//!
//! Existing compiler API:
//!
//!     crate::compiler::compile_source
//!
//! Existing types:
//!
//!     crate::compiler::CompilerError
//!     crate::compiler::CompilationOutput
//!     crate::compiler_types::CompilerConfig
//!
//! The legacy adapter delegates to the existing compile_source function.
//!
//! Future pipeline implementations can implement CompilationDriver
//! without changing the CompilerService API.
//!
//! # Important
//!
//! A service cannot certify the correctness of an underlying driver.
//! Every driver must independently enforce semantic validation,
//! verification, security and target-realization contracts.

use std::path::Path;
use std::sync::Arc;

use crate::compiler_types::CompilerConfig;

use super::{CompilationOutput, CompilerError};

/// Compilation driver interface.
///
/// A driver connects the stable compiler service to an actual compilation
/// implementation.
///
/// Implementations must preserve the following invariants:
///
/// 1. Source meaning must not depend on the host machine.
/// 2. Explicit target requests must not be silently replaced.
/// 3. Unsupported operations must produce explicit errors.
/// 4. Semantic requirements must not be silently discarded.
/// 5. Compiler configuration must be respected.
/// 6. Invalid IR must not be treated as valid compilation input.
/// 7. Compilation errors must retain their original classification.
/// 8. No arbitrary hardware-capacity ceilings may be introduced.
///
/// Drivers must be thread-safe because the service may be shared between
/// concurrent callers.
///
/// A driver must not mutate shared configuration.
pub trait CompilationDriver: Send + Sync {
    /// Compile source text.
    ///
    /// `source_name` identifies the logical source file for diagnostics.
    ///
    /// It is not interpreted as a physical target or hardware identifier.
    ///
    /// The supplied configuration is authoritative for this compilation.
    ///
    /// # Errors
    ///
    /// Returns the existing structured CompilerError.
    fn compile_source(
        &self,
        source: &str,
        source_name: &str,
        config: &CompilerConfig,
    ) -> Result<CompilationOutput, CompilerError>;
}

/// Adapter for the repository's existing compilation pipeline.
///
/// This adapter deliberately does not reimplement parsing, semantic
/// analysis, IR generation, optimization or backend generation.
///
/// It delegates to:
///
///     crate::compiler::compile_source
///
/// This preserves compatibility while the existing pipeline is migrated
/// toward the complete universal compilation architecture.
///
/// IMPORTANT:
///
/// The legacy pipeline currently has outstanding production-readiness
/// issues, including incomplete ownership integration and target-specific
/// assumptions. Using this adapter does not certify those components.
#[derive(Debug, Default, Clone, Copy)]
pub struct LegacyPipelineDriver;

impl CompilationDriver for LegacyPipelineDriver {
    fn compile_source(
        &self,
        source: &str,
        source_name: &str,
        config: &CompilerConfig,
    ) -> Result<CompilationOutput, CompilerError> {
        super::compile_source(
            source,
            source_name,
            config.clone(),
        )
    }
}

/// Reusable Zamani compiler service.
///
/// The service owns an immutable compiler configuration and a shared
/// compilation driver.
///
/// It does not own compiler-global mutable state.
///
/// The generic driver parameter allows different compilation pipeline
/// implementations to be integrated without changing the public service.
///
/// Sharing is safe when the supplied driver implements Send + Sync.
///
/// No fixed worker count or hardware inventory is imposed.
pub struct CompilerService<D: CompilationDriver> {
    config: Arc<CompilerConfig>,
    driver: Arc<D>,
}

impl<D: CompilationDriver> Clone for CompilerService<D> {
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            driver: Arc::clone(&self.driver),
        }
    }
}

impl<D: CompilationDriver> CompilerService<D> {
    /// Creates a compiler service using an explicitly supplied driver.
    ///
    /// No target is inferred from the host machine.
    ///
    /// Configuration validation and compilation-policy enforcement
    /// remain the responsibility of the selected driver and its
    /// authoritative compiler configuration implementation.
    pub fn new(
        config: CompilerConfig,
        driver: D,
    ) -> Self {
        Self {
            config: Arc::new(config),
            driver: Arc::new(driver),
        }
    }

    /// Creates a service using an already shared driver.
    ///
    /// This is useful when multiple compiler services share one
    /// immutable or internally synchronized driver.
    pub fn with_shared_driver(
        config: CompilerConfig,
        driver: Arc<D>,
    ) -> Self {
        Self {
            config: Arc::new(config),
            driver,
        }
    }

    /// Returns the immutable compiler configuration.
    #[must_use]
    pub fn config(&self) -> &CompilerConfig {
        self.config.as_ref()
    }

    /// Returns a reference to the configured compilation driver.
    #[must_use]
    pub fn driver(&self) -> &D {
        self.driver.as_ref()
    }

    /// Returns a shared reference to the driver.
    ///
    /// This permits compatible service instances to share the same driver.
    #[must_use]
    pub fn shared_driver(&self) -> Arc<D> {
        Arc::clone(&self.driver)
    }

    /// Compiles source text.
    ///
    /// This is the primary service operation.
    ///
    /// The caller supplies source text and a logical source name.
    ///
    /// No temporary source file is required.
    ///
    /// Suitable for:
    ///
    /// - command-line applications;
    /// - language servers;
    /// - IDE integrations;
    /// - REPL implementations;
    /// - embedded compiler consumers;
    /// - build systems;
    /// - testing infrastructure;
    /// - automated compilation services.
    ///
    /// # Errors
    ///
    /// Returns the original error from the selected compilation driver.
    pub fn compile_source(
        &self,
        source: &str,
        source_name: &str,
    ) -> Result<CompilationOutput, CompilerError> {
        self.driver.compile_source(
            source,
            source_name,
            self.config.as_ref(),
        )
    }

    /// Compiles a source file.
    ///
    /// Source reading belongs to the service's input boundary.
    /// Actual compilation remains delegated to CompilationDriver.
    ///
    /// File paths are not interpreted as target identifiers.
    ///
    /// The source is read as UTF-8.
    ///
    /// No arbitrary source-size ceiling is imposed by this API.
    ///
    /// Actual memory and filesystem constraints remain operational limits.
    ///
    /// # Errors
    ///
    /// Returns CompilerError::Io when reading fails.
    ///
    /// Returns the original compilation error when compilation fails.
    pub fn compile_path(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<CompilationOutput, CompilerError> {
        let path = path.as_ref();

        let source = std::fs::read_to_string(path).map_err(|error| {
            CompilerError::Io {
                path: path.display().to_string(),
                message: error.to_string(),
            }
        })?;

        let source_name = path.display().to_string();

        self.compile_source(
            &source,
            &source_name,
        )
    }

    /// Compatibility-friendly alias for source-file compilation.
    pub fn compile_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<CompilationOutput, CompilerError> {
        self.compile_path(path)
    }
}

impl CompilerService<LegacyPipelineDriver> {
    /// Explicitly creates a service using the existing compiler pipeline.
    ///
    /// This method is intentionally named to make the transitional
    /// implementation visible to callers.
    ///
    /// A future verified pipeline can replace this adapter without
    /// changing the CompilerService interface.
    pub fn with_legacy_pipeline(
        config: CompilerConfig,
    ) -> Self {
        Self::new(
            config,
            LegacyPipelineDriver,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler_types::{
        CompilationTarget,
        OptimizationLevel,
    };

    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    use std::sync::Arc;

    /// Test driver that returns the supplied source as output.
    ///
    /// This tests service behavior without pretending to implement
    /// a real target backend.
    #[derive(Debug, Default)]
    struct TestDriver {
        invocation_count: AtomicUsize,
    }

    impl CompilationDriver for TestDriver {
        fn compile_source(
            &self,
            source: &str,
            _source_name: &str,
            _config: &CompilerConfig,
        ) -> Result<CompilationOutput, CompilerError> {
            self.invocation_count.fetch_add(
                1,
                Ordering::Relaxed,
            );

            let bytes = source.as_bytes().to_vec();

            Ok(CompilationOutput {
                size_bytes: bytes.len(),
                bytes,
                target: "test-driver".to_string(),
            })
        }
    }

    fn test_config() -> CompilerConfig {
        CompilerConfig {
            target: CompilationTarget::UniversalIRExport(
                "zamani-portable-ir".to_string(),
            ),
            opt_level: OptimizationLevel::None,
            debug_info: false,
            verify: true,
            emit_ir: true,
            parallel: false,
        }
    }

    #[test]
    fn service_accepts_explicit_configuration() {
        let service = CompilerService::new(
            test_config(),
            TestDriver::default(),
        );

        assert!(service.config().verify);
        assert!(service.config().emit_ir);

        assert!(matches!(
            service.config().target,
            CompilationTarget::UniversalIRExport(_)
        ));
    }

    #[test]
    fn source_compilation_delegates_to_driver() {
        let driver = TestDriver::default();

        let service = CompilerService::new(
            test_config(),
            driver,
        );

        let output = service
            .compile_source(
                "let answer = 42;",
                "example.zm",
            )
            .expect("test driver should succeed");

        assert_eq!(
            output.bytes,
            b"let answer = 42;"
        );

        assert_eq!(
            output.target,
            "test-driver"
        );

        assert_eq!(
            output.size_bytes,
            "let answer = 42;".len()
        );
    }

    #[test]
    fn source_compilation_preserves_driver_errors() {
        struct FailingDriver;

        impl CompilationDriver for FailingDriver {
            fn compile_source(
                &self,
                _source: &str,
                _source_name: &str,
                _config: &CompilerConfig,
            ) -> Result<CompilationOutput, CompilerError> {
                Err(CompilerError::Parse {
                    message: "intentional test failure".to_string(),
                })
            }
        }

        let service = CompilerService::new(
            test_config(),
            FailingDriver,
        );

        let result = service.compile_source(
            "invalid source",
            "invalid.zm",
        );

        assert!(matches!(
            result,
            Err(CompilerError::Parse { .. })
        ));
    }

    #[test]
    fn cloned_services_share_the_same_driver() {
        let driver = Arc::new(TestDriver::default());

        let service = CompilerService::with_shared_driver(
            test_config(),
            Arc::clone(&driver),
        );

        let cloned = service.clone();

        service
            .compile_source("first", "first.zm")
            .expect("first compilation should succeed");

        cloned
            .compile_source("second", "second.zm")
            .expect("second compilation should succeed");

        assert_eq!(
            driver.invocation_count.load(Ordering::Relaxed),
            2
        );
    }

    #[test]
    fn legacy_driver_is_available_explicitly() {
        let driver = LegacyPipelineDriver;

        let config = test_config();

        assert!(config.verify);

        // The adapter implements the existing CompilationDriver
        // contract without introducing a second pipeline.
        let _: &dyn CompilationDriver = &driver;
    }

    #[test]
    fn service_configuration_is_not_implicitly_host_selected() {
        let config = test_config();

        assert!(matches!(
            config.target,
            CompilationTarget::UniversalIRExport(_)
        ));
    }

    #[test]
    fn missing_source_file_returns_io_error() {
        let service = CompilerService::new(
            test_config(),
            TestDriver::default(),
        );

        let path = std::env::temp_dir().join(format!(
            "zamani-nonexistent-compiler-source-{}.zm",
            std::process::id()
        ));

        let result = service.compile_path(path);

        assert!(matches!(
            result,
            Err(CompilerError::Io { .. })
        ));
    }
}
