
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani LLVM Backend Module
//!
//! Path: `src/backend/llvm/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the public composition boundary for the LLVM textual-IR
//! emitter and LLVM IR verifier.
//!
//! It provides a single entry point for:
//!
//! 1. Verifying the repository's existing Zamani IR.
//! 2. Emitting LLVM textual IR for an explicitly selected target.
//! 3. Optionally verifying the generated LLVM IR with `llvm-as`.
//! 4. Returning structured artifacts and errors without console output.
//!
//! ## Ownership boundaries
//!
//! This module does not:
//!
//! - define another Zamani IR;
//! - parse or reinterpret Zamani source semantics;
//! - select a target from the host machine;
//! - allocate physical hardware resources;
//! - implement quantum routing, scheduling, or error correction;
//! - implement HDL synthesis;
//! - generate native object files itself;
//! - link or deploy final executables;
//! - write final build artifacts to an implicit filesystem location.
//!
//! The existing `crate::ir_gen::IrModule` remains the input representation
//! for the current emitter. Its long-term relationship to the repository's
//! newer canonical IR architecture must be resolved through an explicit
//! migration, not by silently introducing a second IR here.
//!
//! ## Pipeline
//!
//! ```text
//! Existing Zamani IR
//!        |
//!        v
//! crate::ir_verify::verify_module
//!        |
//!        v
//! LlvmEmitter
//!        |
//!        v
//! LLVM textual IR
//!        |
//!        +-----------------------+
//!        |                       |
//!        v                       v
//! return generated IR     LlvmIrVerifier
//!                                |
//!                                v
//!                         verified LLVM IR
//!                                |
//!                                v
//!                compiler LLVM optimization / object generation
//! ```
//!
//! ## Target and scalability contract
//!
//! The target triple and optional data layout must come from the caller's
//! resolved target configuration. This module never substitutes the build
//! host's architecture for the requested target.
//!
//! There are no arbitrary universal limits on functions, instructions,
//! types, devices, CPUs, qubits, tensor dimensions, or hardware capacity
//! encoded by this module. Actual compilation is necessarily finite and
//! remains subject to available memory, address space, execution time,
//! toolchain support, and explicit operational policies.
//!
//! Unsupported operations must be rejected by the emitter or a preceding
//! domain-specific lowering stage. They must not silently become comments,
//! no-ops, or behavior with different semantics.
//!
//! ## Verification guarantees
//!
//! `emit` establishes that the existing Zamani IR verifier accepts the
//! input and that the LLVM emitter accepts the module's supported subset.
//!
//! `emit_and_verify` additionally requires the configured `llvm-as` process
//! to succeed and produce non-empty bitcode.
//!
//! Neither operation proves semantic equivalence between Zamani and LLVM,
//! target-machine availability, runtime correctness, ABI compatibility with
//! arbitrary external libraries, or successful physical execution.
//! Those properties require their respective validation and test stages.

pub mod emitter;
pub mod verification;

pub use emitter::{
    LlvmEmitter,
    LlvmEmitterConfig,
    LlvmEmitterError,
    LlvmEmitterResult,
    LlvmTarget,
};

pub use verification::{
    LlvmIrVerifier,
    LlvmVerificationConfig,
    LlvmVerificationError,
    LlvmVerificationReport,
};

use crate::ir_gen::IrModule;
use crate::ir_verify::{self, IrVerificationError};

use std::error::Error;
use std::fmt;
use std::path::PathBuf;

/// Result type for the integrated LLVM module API.
pub type LlvmModuleResult<T> = Result<T, LlvmModuleError>;

/// Errors returned by the integrated LLVM module API.
///
/// The underlying errors are retained instead of being reduced to
/// human-readable strings. This allows callers to make reliable decisions
/// based on the failure category.
#[derive(Debug)]
pub enum LlvmModuleError {
    /// The existing Zamani IR verifier rejected the input module.
    InvalidZamaniIr(Vec<IrVerificationError>),

    /// The LLVM text emitter rejected the module or requested target.
    Emission(LlvmEmitterError),

    /// The configured LLVM verification tool rejected the generated text,
    /// could not be executed, or failed to produce valid bitcode.
    Verification(LlvmVerificationError),
}

impl fmt::Display for LlvmModuleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidZamaniIr(errors) => {
                write!(
                    formatter,
                    "Zamani IR verification failed with {} error(s)",
                    errors.len()
                )?;

                for error in errors {
                    write!(formatter, "\n  - {error}")?;
                }

                Ok(())
            }
            Self::Emission(error) => {
                write!(formatter, "LLVM IR emission failed: {error}")
            }
            Self::Verification(error) => {
                write!(formatter, "LLVM IR verification failed: {error}")
            }
        }
    }
}

impl Error for LlvmModuleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // The existing verifier returns a collection of diagnostics.
            // The collection itself is not a standard-library Error.
            Self::InvalidZamaniIr(_) => None,
            Self::Emission(error) => Some(error),
            Self::Verification(error) => Some(error),
        }
    }
}

impl From<LlvmEmitterError> for LlvmModuleError {
    fn from(error: LlvmEmitterError) -> Self {
        Self::Emission(error)
    }
}

impl From<LlvmVerificationError> for LlvmModuleError {
    fn from(error: LlvmVerificationError) -> Self {
        Self::Verification(error)
    }
}

/// An LLVM artifact that has passed both Zamani IR validation and LLVM
/// textual-IR verification.
///
/// The report records what the external verifier actually established.
/// It does not claim that the program has been executed or that its behavior
/// is equivalent to the source program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedLlvmModule {
    /// Generated LLVM textual IR.
    pub llvm_ir: String,

    /// Evidence returned by the configured LLVM verifier.
    pub verification: LlvmVerificationReport,

    /// Target configuration used during emission.
    pub target: LlvmTarget,
}

/// Emit LLVM textual IR after verifying the input Zamani IR.
///
/// This is the recommended pure-emission entry point when an orchestration
/// layer is responsible for deciding when and how LLVM tools are run.
///
/// No external process is started, no output file is created, and no
/// progress messages are printed.
///
/// # Errors
///
/// Returns `InvalidZamaniIr` when the repository's existing IR verifier
/// rejects the module, or `Emission` when the LLVM emitter rejects the
/// target or an unsupported or invalid instruction.
pub fn emit(
    module: &IrModule,
    target: LlvmTarget,
) -> LlvmModuleResult<String> {
    ir_verify::verify_module(module)
        .map_err(LlvmModuleError::InvalidZamaniIr)?;

    LlvmEmitter::new(target)
        .emit(module)
        .map_err(LlvmModuleError::Emission)
}

/// Emit LLVM textual IR and verify it using the configured `llvm-as` tool.
///
/// The executable may be a name resolved through the process environment
/// or an explicit executable path supplied by the caller. Hermetic builds
/// should use a toolchain-managed, pinned executable path.
///
/// The requested target triple is always checked against the emitted
/// module. If a data layout is provided by the target, it takes precedence.
/// Otherwise, a non-empty layout already present in the input module is
/// checked against the generated LLVM text.
///
/// This function verifies LLVM syntax and structural validity as accepted
/// by the configured LLVM assembler. It does not generate native object
/// code; use the existing `crate::compiler::llvm_backend::LlvmBackend` for
/// the repository's existing object-generation pipeline.
///
/// # Side effects
///
/// The underlying verifier creates an isolated temporary workspace and
/// invokes the configured LLVM tool. It does not write a final build
/// artifact to the project directory.
///
/// # Errors
///
/// Returns an error if Zamani IR verification, LLVM emission, metadata
/// validation, tool execution, or LLVM verification fails.
pub fn emit_and_verify(
    module: &IrModule,
    target: LlvmTarget,
    llvm_as: impl Into<PathBuf>,
) -> LlvmModuleResult<VerifiedLlvmModule> {
    // Validate the target-independent IR before target-specific lowering.
    // This prevents the LLVM emitter from becoming a substitute for the
    // repository's canonical input verifier.
    let llvm_ir = emit(module, target.clone())?;

    // Enforce the target selected by the caller. Never infer a target from
    // the host running the compiler.
    let mut verification_config =
        LlvmVerificationConfig::new(llvm_as)
            .with_target_triple(target.triple.clone());

    // The emitter uses the explicit target layout when supplied. When it
    // is absent, the current emitter preserves a non-empty module layout.
    let effective_layout = target.data_layout.clone().or_else(|| {
        if module.data_layout.trim().is_empty() {
            None
        } else {
            Some(module.data_layout.clone())
        }
    });

    if let Some(layout) = effective_layout {
        verification_config =
            verification_config.with_data_layout(layout);
    }

    let verification =
        LlvmIrVerifier::new(verification_config).verify(&llvm_ir)?;

    Ok(VerifiedLlvmModule {
        llvm_ir,
        verification,
        target,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir_gen::{IrFunction, IrInstruction, IrType, IrValue};

    fn test_target() -> LlvmTarget {
        LlvmTarget::new("x86_64-unknown-linux-gnu")
    }

    #[test]
    fn public_emission_api_emits_llvm_text_without_running_tools() {
        let mut module = IrModule::new("backend_llvm_mod_test");

        let mut function =
            IrFunction::new("main", Vec::new(), IrType::I32);

        function.push(IrInstruction::Ret(Some(
            IrValue::ConstInt(0, IrType::I32),
        )));

        module.functions.push(function);

        let llvm_ir = emit(&module, test_target())
            .expect("valid module should emit LLVM IR");

        assert!(
            llvm_ir.contains(
                "target triple = \"x86_64-unknown-linux-gnu\""
            )
        );
        assert!(llvm_ir.contains("define i32 @main()"));
        assert!(llvm_ir.contains("ret i32 0"));
    }

    #[test]
    fn rejects_invalid_zamani_ir_before_emission() {
        let mut module = IrModule::new("invalid_duplicate_functions");

        module.functions.push(IrFunction::new(
            "duplicate",
            Vec::new(),
            IrType::Void,
        ));
        module.functions.push(IrFunction::new(
            "duplicate",
            Vec::new(),
            IrType::Void,
        ));

        assert!(matches!(
            emit(&module, test_target()),
            Err(LlvmModuleError::InvalidZamaniIr(_))
        ));
    }

    #[test]
    fn rejects_an_empty_target_triple() {
        let module = IrModule::new("empty_target");

        assert!(matches!(
            emit(&module, LlvmTarget::new("  ")),
            Err(LlvmModuleError::Emission(
                LlvmEmitterError::InvalidTarget(_)
            ))
        ));
    }

    #[test]
    fn target_metadata_is_not_inferred_from_the_host() {
        let module = IrModule::new("explicit_target");

        let target = LlvmTarget::new("wasm32-unknown-unknown");

        let llvm_ir = emit(&module, target)
            .expect("an explicitly selected target should be preserved");

        assert!(llvm_ir.contains(
            "target triple = \"wasm32-unknown-unknown\""
        ));
        assert!(!llvm_ir.contains(
            "target triple = \"x86_64-unknown-linux-gnu\""
        ));
    }

    #[test]
    fn unsupported_quantum_instructions_are_not_silently_emitted() {
        use crate::ir_gen::IrRegister;

        let mut module = IrModule::new("unlowered_quantum");

        let mut function =
            IrFunction::new("main", Vec::new(), IrType::Void);

        function.push(IrInstruction::QuantumGate(
            IrRegister::new("q", IrType::Quantum),
            "H".to_owned(),
            Vec::new(),
        ));
        function.push(IrInstruction::Ret(None));

        assert!(matches!(
            emit(&module, test_target()),
            Err(LlvmModuleError::Emission(
                LlvmEmitterError::UnsupportedInstruction { .. }
            ))
                | Err(LlvmModuleError::Emission(
                    LlvmEmitterError::InvalidType(_)
                ))
                | Err(LlvmModuleError::InvalidZamaniIr(_))
        ));
    }
}
