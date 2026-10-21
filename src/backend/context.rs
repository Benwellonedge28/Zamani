
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Immutable compilation context for Zamani backend operations.
//!
//! Path: src/backend/context.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Responsibility
//!
//! Owns a validated, immutable snapshot of:
//! - the existing compiler configuration plus backend policies;
//! - one explicitly selected target descriptor;
//! - structured diagnostics already associated with this context.
//!
//! The context is shared read-only between validation, lowering, emitters,
//! and artifact verification. Each compilation must construct its own context.
//!
//! # Ownership boundaries
//!
//! This module does not:
//! - parse source or define language semantics;
//! - verify or represent an IR;
//! - select a target from a candidate set;
//! - register or instantiate backend implementations;
//! - allocate physical resources or establish resource feasibility;
//! - perform target-specific lowering or execute external tools;
//! - write files, print messages, or inspect the machine;
//! - maintain a global diagnostic collector or global mutable state.
//!
//! The input/verified-IR contract belongs in the backend input boundary.
//! That module does not yet exist in the inspected repository, so this
//! context deliberately does not invent an IR type or claim that an input
//! program has been verified.
//!
//! # Immutability
//!
//! The public API exposes shared references, not mutable references to the
//! configuration, target, or diagnostic collection. Cloning a context shares
//! its immutable configuration and target through Arc.
//!
//! Adding diagnostics returns a new context snapshot. It does not mutate
//! snapshots already held by another compilation stage.
//!
//! # Scalability
//!
//! This module imposes no universal limits on hardware capacity, program
//! size, target count, qubits, CPUs, devices, nodes, memory, or tensor rank.
//! Operational budgets are carried by BackendConfig and enforced by the
//! appropriate consumers; this module does not pretend to enforce them.
//!
//! # Integration
//!
//! - config.rs owns BackendConfig and configuration validation.
//! - target.rs owns TargetDescriptor and target validation.
//! - diagnostics.rs owns BackendDiagnostic.
//! - errors.rs owns BackendError and BackendResult.
//! - selection.rs chooses a target before this context is constructed.
//! - validation.rs and pipeline.rs should consume this context.
//! - emitters must not use this context as proof of target feasibility.
//!
//! The canonical module root must declare `pub mod context;`.

use std::sync::Arc;

use crate::compiler_types::CompilerConfig;

use super::config::BackendConfig;
use super::diagnostics::{BackendDiagnostic, BackendDiagnosticLevel};
use super::errors::{BackendError, BackendResult};
use super::target::TargetDescriptor;

/// Immutable backend context for one compilation request.
///
/// Construction validates both the backend configuration and target
/// descriptor. A context therefore cannot be created through the public API
/// with a configuration or descriptor that fails those validation methods.
///
/// A valid context does not prove that the target supports a program, that a
/// toolchain is installed, that physical resources are available, or that an
/// artifact will be generated successfully. Those are later-stage contracts.
#[derive(Debug, Clone)]
pub struct BackendContext {
    config: Arc<BackendConfig>,
    target: Arc<TargetDescriptor>,
    diagnostics: Arc<[BackendDiagnostic]>,
}

impl BackendContext {
    /// Construct a context from an explicit backend configuration and target.
    ///
    /// No target is discovered or inferred from the host machine.
    pub fn new(
        config: BackendConfig,
        target: TargetDescriptor,
    ) -> BackendResult<Self> {
        BackendContextBuilder::new(config, target).build()
    }

    /// Construct a context using the repository's existing compiler settings.
    ///
    /// Backend-specific policies receive their documented defaults through
    /// `BackendConfig::from_compiler_config`.
    pub fn from_compiler_config(
        config: &CompilerConfig,
        target: TargetDescriptor,
    ) -> BackendResult<Self> {
        Self::new(BackendConfig::from_compiler_config(config), target)
    }

    /// Return the validated backend configuration.
    pub fn config(&self) -> &BackendConfig {
        self.config.as_ref()
    }

    /// Return the explicitly selected and validated target descriptor.
    pub fn target(&self) -> &TargetDescriptor {
        self.target.as_ref()
    }

    /// Return the immutable diagnostic snapshot associated with this context.
    pub fn diagnostics(&self) -> &[BackendDiagnostic] {
        self.diagnostics.as_ref()
    }

    /// Whether this context contains an error-severity diagnostic.
    ///
    /// This reports the state of this snapshot only. It does not replace the
    /// compiler-wide DiagnosticEngine or decide whether compilation may
    /// continue under every pipeline policy.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(BackendDiagnostic::is_error)
    }

    /// Return a new context snapshot with one additional diagnostic.
    ///
    /// The original context and its diagnostic snapshot remain unchanged.
    #[must_use]
    pub fn with_diagnostic(
        &self,
        diagnostic: BackendDiagnostic,
    ) -> Self {
        let mut diagnostics = Vec::with_capacity(self.diagnostics.len() + 1);
        diagnostics.extend(self.diagnostics.iter().cloned());
        diagnostics.push(diagnostic);

        Self {
            config: Arc::clone(&self.config),
            target: Arc::clone(&self.target),
            diagnostics: Arc::from(diagnostics),
        }
    }

    /// Return a new context snapshot with additional diagnostics.
    ///
    /// The iterator is consumed once. No diagnostic is printed, emitted to a
    /// global collector, or silently deduplicated by this method.
    #[must_use]
    pub fn with_diagnostics<I>(&self, additional: I) -> Self
    where
        I: IntoIterator<Item = BackendDiagnostic>,
    {
        let mut diagnostics = Vec::new();
        diagnostics.extend(self.diagnostics.iter().cloned());
        diagnostics.extend(additional);

        Self {
            config: Arc::clone(&self.config),
            target: Arc::clone(&self.target),
            diagnostics: Arc::from(diagnostics),
        }
    }

    /// Return a new context snapshot with its diagnostic list replaced.
    ///
    /// This is useful when a pipeline stage deliberately constructs a new
    /// stage-local diagnostic snapshot. Existing snapshots remain unchanged.
    #[must_use]
    pub fn with_replaced_diagnostics<I>(&self, diagnostics: I) -> Self
    where
        I: IntoIterator<Item = BackendDiagnostic>,
    {
        Self {
            config: Arc::clone(&self.config),
            target: Arc::clone(&self.target),
            diagnostics: Arc::from(
                diagnostics.into_iter().collect::<Vec<_>>(),
            ),
        }
    }

    /// Check the invariants that must remain true before a stage consumes
    /// this context.
    ///
    /// Construction already performs these checks. This method is provided
    /// for explicit pipeline-boundary validation and defense in depth.
    pub fn validate(&self) -> BackendResult<()> {
        self.config().validate().map_err(|error| {
            BackendError::invalid_configuration(error.to_string())
                .with_target(self.target().id().as_str())
                .with_context("validating backend compilation context")
        })?;

        self.target().validate().map_err(|error| {
            error
                .with_target(self.target().id().as_str())
                .with_context("validating backend compilation context")
        })?;

        Ok(())
    }
}

/// Builder for a validated immutable [`BackendContext`].
///
/// The builder is mutable only while assembling a context. After `build`,
/// downstream stages receive an immutable snapshot.
///
/// It intentionally requires the caller to supply the target selected by the
/// target-selection stage; it does not silently select a default target.
#[derive(Debug)]
pub struct BackendContextBuilder {
    config: BackendConfig,
    target: TargetDescriptor,
    diagnostics: Vec<BackendDiagnostic>,
}

impl BackendContextBuilder {
    /// Start constructing a context from explicit configuration and target.
    pub fn new(
        config: BackendConfig,
        target: TargetDescriptor,
    ) -> Self {
        Self {
            config,
            target,
            diagnostics: Vec::new(),
        }
    }

    /// Adapt the existing compiler configuration before adding context data.
    pub fn from_compiler_config(
        config: &CompilerConfig,
        target: TargetDescriptor,
    ) -> Self {
        Self::new(BackendConfig::from_compiler_config(config), target)
    }

    /// Add a diagnostic to the context being assembled.
    #[must_use]
    pub fn with_diagnostic(
        mut self,
        diagnostic: BackendDiagnostic,
    ) -> Self {
        self.diagnostics.push(diagnostic);
        self
    }

    /// Add multiple diagnostics to the context being assembled.
    #[must_use]
    pub fn with_diagnostics<I>(mut self, diagnostics: I) -> Self
    where
        I: IntoIterator<Item = BackendDiagnostic>,
    {
        self.diagnostics.extend(diagnostics);
        self
    }

    /// Build the immutable context after validating configuration and target.
    pub fn build(self) -> BackendResult<BackendContext> {
        self.config.validate().map_err(|error| {
            BackendError::invalid_configuration(error.to_string())
                .with_target(self.target.id().as_str())
                .with_context("building backend compilation context")
        })?;

        self.target.validate().map_err(|error| {
            error
                .with_target(self.target.id().as_str())
                .with_context("building backend compilation context")
        })?;

        Ok(BackendContext {
            config: Arc::new(self.config),
            target: Arc::new(self.target),
            diagnostics: Arc::from(self.diagnostics),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::backend::config::{
        OutputDestination, OutputPolicy,
    };
    use crate::backend::diagnostics::{
        BackendDiagnostic, DiagnosticCode,
    };
    use crate::backend::target::{
        ExecutionModel, TargetDescriptor,
    };
    use std::path::PathBuf;

    fn target() -> TargetDescriptor {
        TargetDescriptor::from_name(
            "test.portable",
            "Portable test target",
            ExecutionModel::Other("test-model".to_owned()),
        )
        .expect("test target descriptor must be valid")
    }

    fn diagnostic(
        level: BackendDiagnosticLevel,
        code: &str,
        message: &str,
    ) -> BackendDiagnostic {
        BackendDiagnostic::new(
            level,
            DiagnosticCode::new(code)
                .expect("test diagnostic code must be valid"),
            message,
        )
    }

    #[test]
    fn construction_validates_and_preserves_explicit_target() {
        let target = target();
        let expected_id = target.id().as_str().to_owned();

        let context = BackendContext::new(
            BackendConfig::default(),
            target,
        )
        .expect("valid context should be constructed");

        assert_eq!(context.target().id().as_str(), expected_id);
        assert!(context.diagnostics().is_empty());
        assert!(!context.has_errors());
    }

    #[test]
    fn legacy_compiler_config_is_adapted_without_a_second_owner() {
        let compiler_config = CompilerConfig::default();
        let expected_target = compiler_config.target.clone();

        let context = BackendContext::from_compiler_config(
            &compiler_config,
            target(),
        )
        .expect("valid compiler configuration should be adapted");

        assert_eq!(context.config().compiler.target, expected_target);
    }

    #[test]
    fn invalid_backend_configuration_is_rejected() {
        let config = BackendConfig {
            output: OutputPolicy {
                destination: OutputDestination::Directory(PathBuf::new()),
                ..OutputPolicy::default()
            },
            ..BackendConfig::default()
        };

        let result = BackendContext::new(config, target());

        assert!(result.is_err());
    }

    #[test]
    fn diagnostic_updates_create_independent_snapshots() {
        let original = BackendContext::new(
            BackendConfig::default(),
            target(),
        )
        .expect("valid context");

        let updated = original.with_diagnostic(diagnostic(
            BackendDiagnosticLevel::Warning,
            "BCTX100",
            "optional optimization is unavailable",
        ));

        assert!(original.diagnostics().is_empty());
        assert_eq!(updated.diagnostics().len(), 1);
        assert!(!updated.has_errors());
    }

    #[test]
    fn error_diagnostics_are_visible_without_global_state() {
        let original = BackendContext::new(
            BackendConfig::default(),
            target(),
        )
        .expect("valid context");

        let updated = original.with_diagnostic(diagnostic(
            BackendDiagnosticLevel::Error,
            "BCTX200",
            "required target capability is unavailable",
        ));

        assert!(!original.has_errors());
        assert!(updated.has_errors());
        assert_eq!(updated.diagnostics().len(), 1);
    }

    #[test]
    fn diagnostic_batches_preserve_order_and_duplicates() {
        let first = diagnostic(
            BackendDiagnosticLevel::Warning,
            "BCTX300",
            "first warning",
        );
        let second = diagnostic(
            BackendDiagnosticLevel::Note,
            "BCTX301",
            "additional context",
        );

        let context = BackendContext::new(
            BackendConfig::default(),
            target(),
        )
        .expect("valid context")
        .with_diagnostics([first.clone(), second.clone(), first.clone()]);

        assert_eq!(context.diagnostics().len(), 3);
        assert_eq!(context.diagnostics()[0], first);
        assert_eq!(context.diagnostics()[1], second);
        assert_eq!(context.diagnostics()[2], first);
    }

    #[test]
    fn cloned_contexts_share_immutable_state_and_validate() {
        let context = BackendContext::new(
            BackendConfig::default(),
            target(),
        )
        .expect("valid context");

        let cloned = context.clone();

        assert_eq!(
            cloned.target().id().as_str(),
            context.target().id().as_str()
        );
        assert!(cloned.validate().is_ok());
    }

    #[test]
    fn context_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<BackendContext>();
    }

    #[test]
    fn construction_does_not_claim_target_feasibility() {
        let target = target();
        let context = BackendContext::new(
            BackendConfig::default(),
            target,
        )
        .expect("valid descriptive target context");

        // A descriptor can be valid without claiming an emitter exists.
        // The context must not silently upgrade its availability status.
        assert_eq!(
            context.target().availability(),
            crate::backend::target::ImplementationAvailability::DescriptorOnly
        );
    }
}
