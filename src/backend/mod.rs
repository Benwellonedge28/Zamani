
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Backend
//!
//! Canonical backend module root.
//!
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! This module is the stable integration boundary between the compiler's
//! verified intermediate representation and target-specific artifact
//! generation.
//!
//! It preserves the existing backend API during migration. It does not
//! define a second source-language semantic model, universal IR, quantum
//! IR, HDL IR, linker, or runtime.
//!
//! ## Architectural authority
//!
//! The backend must respect the following repository contracts:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/lexer/tokens.g4`
//! - `src/compiler/mod.rs`
//! - `src/compiler_types.rs`
//! - `src/ir_gen.rs`
//! - `src/ir_verify.rs`
//! - `src/compiler/llvm_backend.rs`
//! - `src/compiler/wasm_backend.rs`
//! - `src/quantum/ir/`
//! - `src/hdl/`
//!
//! The grammar defines source syntax. Semantic analysis establishes source
//! meaning. The applicable IR defines the representation being compiled.
//! The backend validates target feasibility and generates target-specific
//! artifacts without redefining source semantics.
//!
//! ## POCO-REAF
//!
//! Zamani targets:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! Portability means preserving program meaning and supporting explicit
//! target realization. It does not mean that every target supports every
//! operation or can execute every program.
//!
//! Unsupported capabilities, unavailable toolchains, incompatible ABIs,
//! infeasible resource requirements, and invalid input must be reported
//! explicitly. A backend must not silently substitute different semantics.
//!
//! No universal hardware-capacity limits are defined here. Concrete target
//! constraints and configurable operational budgets belong to their
//! appropriate contracts.
//!
//! ## Ownership boundaries
//!
//! This module owns:
//!
//! - the public backend module boundary;
//! - compatibility exports during migration;
//! - the API version identifying this boundary.
//!
//! It does not own:
//!
//! - source parsing or semantic analysis;
//! - the canonical IR implementation;
//! - physical-device allocation or quantum routing;
//! - HDL synthesis and hardware realization;
//! - distributed scheduling;
//! - external toolchain implementation;
//! - runtime execution or deployment policy.
//!
//! ## Migration contract
//!
//! Before this module replaces the existing `src/backend.rs`, that file
//! must be moved to `src/backend/legacy.rs` without changing its contents.
//! The legacy implementation remains an adapter until each target has been
//! audited and migrated. It must not become a permanent second backend
//! registry or the source of competing target contracts.
//!
//! The compatibility exports below intentionally preserve the API currently
//! consumed by `src/compiler/mod.rs` and other repository callers.
//!
//! IMPORTANT: these exports do not certify the legacy target implementations
//! as production-ready. The architecture misdispatch and misleading LLVM
//! artifact contract identified during the repository audit still require
//! correction in the owning implementation before release.

/// Version of the public backend module boundary.
///
/// This is not the Zamani language version, IR format version, target
/// architecture version, or external toolchain version.
pub const BACKEND_API_VERSION: u16 = 1;

/// Existing backend implementation retained during the staged migration.
///
/// This module is transitional. Target implementations must be migrated
/// individually after their contracts and tests have been established.
#[path = "legacy.rs"]
mod legacy;

/// Existing backend trait.
///
/// Retained for compatibility with existing target implementations.
pub use legacy::Backend;

/// Existing code-generation error type.
///
/// New implementations should evolve toward structured error codes and
/// preserve causal information without requiring callers to parse messages.
pub use legacy::CodeGenError;

/// Existing code-generation output type.
///
/// The current string-based output contract is retained for compatibility.
/// It must not be used to imply that an artifact has been validated or that
/// arbitrary output text is executable target code.
pub use legacy::CodeGenOutput;

/// Existing code-generation dispatcher.
///
/// This is retained temporarily to avoid breaking current compiler callers.
/// It is not certified for release until target dispatch, artifact validity,
/// IR verification, and side-effect behavior have been corrected.
pub use legacy::CodeGenerator;

/// Existing LLVM IR adapter.
pub use legacy::LlvmIrBackend;

/// Existing x86-64 adapter.
pub use legacy::X86Backend;

/// Existing WebAssembly adapter.
pub use legacy::WasmBackend;

/// Existing QASM adapter.
pub use legacy::QasmBackend;

/// Existing NanoControl adapter.
pub use legacy::NanoBackend;

/// Existing RISC-V adapter.
pub use legacy::RiscVBackend;

/// Existing MTS bytecode adapter.
pub use legacy::MtsBackend;

/// Existing WebAssembly compatibility adapter.
pub use legacy::NewWasmBackendAdapter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_api_version_is_nonzero() {
        assert_ne!(BACKEND_API_VERSION, 0);
    }

    #[test]
    fn existing_backend_contract_remains_available() {
        fn accepts_backend<T: Backend>(_: &T) {}

        accepts_backend(&X86Backend);
        accepts_backend(&WasmBackend);
        accepts_backend(&QasmBackend);
        accepts_backend(&NanoBackend);
        accepts_backend(&RiscVBackend);
        accepts_backend(&MtsBackend);
        accepts_backend(&LlvmIrBackend);
        accepts_backend(&NewWasmBackendAdapter);
    }

    #[test]
    fn existing_error_contract_remains_available() {
        let error = CodeGenError::new("test failure", "test-target");

        assert_eq!(error.message, "test failure");
        assert_eq!(error.target, "test-target");
    }
}
