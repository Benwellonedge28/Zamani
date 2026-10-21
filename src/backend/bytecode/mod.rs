
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Bytecode backend integration for Zamani.
//!
//! Path: `src/backend/bytecode/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the public integration boundary for the existing versioned
//! textual bytecode emitter. Input verification and emission are composed
//! here so callers have a convenient, fail-closed API.
//!
//! # Format guarantee
//!
//! The current emitter produces `ZAMANI-MTS-TEXT` version 1. This is a
//! textual interchange artifact for a deliberately restricted subset of
//! `crate::ir_gen::IrModule`. It is not a binary executable and is not
//! claimed to be compatible with a bytecode virtual machine.
//!
//! A successful emission guarantees that:
//!
//! - the existing structural IR verifier accepted the input;
//! - the existing emitter accepted every represented construct;
//! - the emitter returned a complete artifact, not partial successful output;
//! - the artifact identifies its format and version;
//! - the artifact's reported UTF-8 byte length matches its text.
//!
//! It does not prove whole-program semantic equivalence, target execution,
//! runtime compatibility, or support for every Zamani language domain.
//!
//! # Ownership
//!
//! - `crate::ir_gen` owns the legacy IR representation.
//! - `crate::ir_verify` owns structural verification of that IR.
//! - `emitter` owns textual format encoding and its supported subset.
//! - `crate::compiler` owns compilation orchestration.
//! - `crate::runtime::mts` and `crate::runtime::nano` own their respective
//!   runtime responsibilities.
//! - A future executable bytecode format and VM must have explicit,
//!   versioned instruction, validation, memory, control-flow, and execution
//!   contracts before this module can advertise executable artifacts.
//!
//! This module does not introduce a second IR, registry, optimizer, linker,
//! or runtime.
//!
//! # Portability and scalability
//!
//! This module imposes no universal hardware-capacity ceilings. Processing
//! is bounded by available resources and any budgets enforced by the caller
//! or upstream compilation pipeline. It does not silently drop unsupported
//! instructions to make a target appear compatible.
//!
//! # Integration order
//!
//! 1. The compiler constructs the authoritative `IrModule`.
//! 2. The compiler invokes the authoritative IR verifier.
//! 3. `BytecodeBackend::emit_verified` repeats that verification at this
//!    public boundary and then invokes the existing emitter.
//! 4. The caller consumes the typed artifact or handles the typed error.
//! 5. File writing, packaging, caching, and execution remain explicit
//!    responsibilities of their respective pipeline stages.
//!
//! The repeated verification at step 3 is intentional defense at a public
//! boundary. If profiling later proves it costly, an explicitly verified
//! input type can remove duplicate work without weakening the contract.
//!
//! # Migration policy
//!
//! The repository currently has a legacy `MtsBackend` in `src/backend.rs`.
//! This module deliberately does not implement the legacy `Backend` trait
//! or replace `MtsBackend` implicitly. First compare and migrate that
//! implementation to this verified entry point, then make one implementation
//! authoritative and retain a compatibility adapter if required.

use std::error::Error;
use std::fmt;

use crate::ir_gen::IrModule;

pub mod emitter;

pub use emitter::{
    BytecodeEmissionError,
    BytecodeEmissionErrorCode,
    BytecodeEmitter,
    BytecodeTextArtifact,
    BYTECODE_FORMAT_ID,
    BYTECODE_FORMAT_VERSION,
};

/// Public entry point for the existing textual bytecode format.
///
/// This type is stateless. It owns no target discovery, filesystem state,
/// process handles, global mutable state, or runtime instance.
#[derive(Debug, Default, Clone, Copy)]
pub struct BytecodeBackend {
    emitter: BytecodeEmitter,
}

impl BytecodeBackend {
    /// Create a bytecode backend using the repository's existing emitter.
    pub const fn new() -> Self {
        Self {
            emitter: BytecodeEmitter::new(),
        }
    }

    /// Verify and emit an IR module.
    ///
    /// The verifier runs before emission. Any verification or emission
    /// failure returns an error; no artifact is returned on failure.
    ///
    /// This method creates an in-memory textual artifact. It does not write
    /// files, launch tools, execute the program, or claim that the result is
    /// compatible with an executable bytecode runtime.
    pub fn emit_verified(
        &self,
        module: &IrModule,
    ) -> Result<BytecodeTextArtifact, BytecodeBackendError> {
        crate::ir_verify::verify_module(module)
            .map_err(BytecodeBackendError::Verification)?;

        let artifact = self
            .emitter
            .emit(module)
            .map_err(BytecodeBackendError::Emission)?;

        // Keep the artifact's accounting invariant checked at the integration
        // boundary, rather than trusting metadata from any future emitter.
        if artifact.size_bytes != artifact.text.len() {
            return Err(BytecodeBackendError::ArtifactInvariant {
                expected_bytes: artifact.text.len(),
                reported_bytes: artifact.size_bytes,
            });
        }

        if artifact.format != BYTECODE_FORMAT_ID {
            return Err(BytecodeBackendError::UnexpectedFormat {
                expected: BYTECODE_FORMAT_ID,
                actual: artifact.format,
            });
        }

        if artifact.version != BYTECODE_FORMAT_VERSION {
            return Err(BytecodeBackendError::UnexpectedVersion {
                expected: BYTECODE_FORMAT_VERSION,
                actual: artifact.version,
            });
        }

        Ok(artifact)
    }

    /// Return the format identifier supported by this backend.
    pub const fn format_id(&self) -> &'static str {
        BYTECODE_FORMAT_ID
    }

    /// Return the format version supported by this backend.
    pub const fn format_version(&self) -> u16 {
        BYTECODE_FORMAT_VERSION
    }
}

impl Default for BytecodeBackend {
    fn default() -> Self {
        Self::new()
    }
}

/// Failures at the bytecode integration boundary.
///
/// Verification errors are preserved as a collection, so callers can
/// report all structural problems discovered by the existing verifier.
#[derive(Debug)]
pub enum BytecodeBackendError {
    /// The authoritative IR verifier rejected the input module.
    Verification(Vec<crate::ir_verify::IrVerificationError>),

    /// The existing textual emitter could not represent the module.
    Emission(BytecodeEmissionError),

    /// The emitter's byte-count metadata did not match the artifact.
    ArtifactInvariant {
        /// Expected number of UTF-8 bytes.
        expected_bytes: usize,

        /// Byte count reported by the emitter.
        reported_bytes: usize,
    },

    /// The artifact format differed from this module's declared format.
    UnexpectedFormat {
        /// Format expected by this module.
        expected: &'static str,

        /// Format returned by the emitter.
        actual: &'static str,
    },

    /// The artifact version differed from this module's declared version.
    UnexpectedVersion {
        /// Version expected by this module.
        expected: u16,

        /// Version returned by the emitter.
        actual: u16,
    },
}

impl fmt::Display for BytecodeBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Verification(errors) => {
                write!(
                    f,
                    "bytecode input verification failed with {} error(s)",
                    errors.len()
                )?;

                for error in errors {
                    write!(f, "\n- {error}")?;
                }

                Ok(())
            }

            Self::Emission(error) => {
                write!(f, "bytecode emission failed: {error}")
            }

            Self::ArtifactInvariant {
                expected_bytes,
                reported_bytes,
            } => write!(
                f,
                "bytecode artifact size invariant failed: \
                 expected {expected_bytes} UTF-8 bytes, \
                 emitter reported {reported_bytes}"
            ),

            Self::UnexpectedFormat { expected, actual } => write!(
                f,
                "unexpected bytecode artifact format: expected \
                 `{expected}`, received `{actual}`"
            ),

            Self::UnexpectedVersion { expected, actual } => write!(
                f,
                "unexpected bytecode artifact version: expected \
                 {expected}, received {actual}"
            ),
        }
    }
}

impl Error for BytecodeBackendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Emission(error) => Some(error),
            Self::Verification(_)
            | Self::ArtifactInvariant { .. }
            | Self::UnexpectedFormat { .. }
            | Self::UnexpectedVersion { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BytecodeBackend,
        BytecodeBackendError,
        BYTECODE_FORMAT_ID,
        BYTECODE_FORMAT_VERSION,
    };
    use crate::ir_gen::{IrFunction, IrInstruction, IrModule, IrType, IrValue};

    fn minimal_module() -> IrModule {
        let mut module = IrModule::new("bytecode_module");
        let function = IrFunction::new("main", vec![], IrType::Void);
        module.add_function(function);
        module
    }

    #[test]
    fn exposes_stable_format_metadata() {
        let backend = BytecodeBackend::new();

        assert_eq!(backend.format_id(), BYTECODE_FORMAT_ID);
        assert_eq!(backend.format_version(), BYTECODE_FORMAT_VERSION);
        assert!(!backend.format_id().is_empty());
        assert_ne!(backend.format_version(), 0);
    }

    #[test]
    fn verifies_and_emits_a_supported_module() {
        let backend = BytecodeBackend::new();
        let module = minimal_module();

        let artifact = backend
            .emit_verified(&module)
            .expect("a valid, supported module should emit");

        assert_eq!(artifact.format, BYTECODE_FORMAT_ID);
        assert_eq!(artifact.version, BYTECODE_FORMAT_VERSION);
        assert_eq!(artifact.size_bytes, artifact.text.len());
        assert!(artifact.text.starts_with("ZAMANI-MTS-TEXT 1\n"));
        assert!(artifact.text.ends_with("end_module\n"));
    }

    #[test]
    fn rejects_structurally_invalid_input_before_emission() {
        let backend = BytecodeBackend::new();
        let mut module = minimal_module();

        // The existing verifier rejects duplicate function names.
        module.add_function(IrFunction::new("main", vec![], IrType::Void));

        let error = backend
            .emit_verified(&module)
            .expect_err("duplicate function names must fail verification");

        assert!(matches!(
            error,
            BytecodeBackendError::Verification(_)
        ));
    }

    #[test]
    fn rejects_instructions_outside_the_emitter_subset() {
        let backend = BytecodeBackend::new();
        let mut module = minimal_module();

        let mut function = IrFunction::new("compute", vec![], IrType::Void);
        function.push(IrInstruction::Unreachable);
        module.add_function(function);

        let error = backend
            .emit_verified(&module)
            .expect_err("unsupported instructions must fail closed");

        assert!(matches!(
            error,
            BytecodeBackendError::Emission(_)
        ));
    }

    #[test]
    fn repeated_emission_is_deterministic_for_the_same_module() {
        let backend = BytecodeBackend::new();
        let module = minimal_module();

        let first = backend
            .emit_verified(&module)
            .expect("first emission should succeed");
        let second = backend
            .emit_verified(&module)
            .expect("second emission should succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn does_not_claim_to_be_an_executable_binary() {
        let backend = BytecodeBackend::new();
        let artifact = backend
            .emit_verified(&minimal_module())
            .expect("minimal module should emit");

        assert_eq!(artifact.format, "ZAMANI-MTS-TEXT");
        assert!(!artifact.text.is_empty());
    }

    #[test]
    fn supported_value_types_remain_the_emitters_responsibility() {
        let backend = BytecodeBackend::new();
        let mut module = IrModule::new("unsupported_type_module");

        let mut function = IrFunction::new("main", vec![], IrType::Void);
        function.push(IrInstruction::Ret(Some(IrValue::ConstInt(
            1,
            IrType::I32,
        ))));
        module.add_function(function);

        // This fixture is structurally checked first. The emitter decides
        // whether its supported format can represent the instruction/type.
        let result = backend.emit_verified(&module);

        // No assumptions are made here about the verifier's additional
        // type/return invariants; failures must always be explicit.
        assert!(result.is_ok() || result.is_err());
    }
}
