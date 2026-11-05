
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Conversion
//!
//! Path: `src/compiler/lowering/conversion.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide a stable compiler-facing conversion request and invocation
//! boundary for converting verified canonical IR through existing,
//! authoritative domain-owned conversion implementations.
//!
//! This module is an orchestration adapter. It does not implement a
//! second conversion engine.
//!
//! ## Canonical authorities
//!
//! - `crate::compiler::ir::input`
//! - `crate::compiler::ir::adapters`
//! - `crate::compiler::ir::translation`
//! - `crate::compiler::ir::lowering`
//! - `crate::zuir::lowering::api`
//! - `crate::zuir::lowering::context`
//! - `crate::zuir::lowering::legality`
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The compiler/lowering conversion request container.
//! - The compiler/lowering conversion invocation entry point.
//! - The public integration boundary for verified conversion.
//!
//! This module does not own:
//!
//! - Canonical IR entities or identifiers.
//! - Source-language semantics.
//! - Type inference or ownership analysis.
//! - Representation adapter algorithms.
//! - Conversion legality algorithms.
//! - Lowering context state or entity mappings.
//! - Domain-specific conversion implementations.
//! - IR verification algorithms.
//! - Optimization passes.
//! - Target discovery or physical resource allocation.
//! - Scheduling, routing, or placement.
//! - Backend code generation.
//! - Runtime execution.
//!
//! These responsibilities remain with their existing authorities.
//!
//! ## Conversion contract
//!
//! A conversion request must contain:
//!
//! - A `VerifiedIrInput` produced by the canonical compiler IR
//!   verification gate.
//! - An existing `AdapterDescriptor` describing the representation
//!   direction and conversion guarantee.
//! - A canonical `LoweringContext`.
//! - A `LegalityCandidate` containing the applicable semantic and
//!   capability evidence.
//! - A domain-owned implementation of `LoweringBackend`.
//!
//! The request does not establish that its candidate is legal.
//! The canonical legality subsystem makes that determination.
//!
//! The request does not implement a conversion merely because a
//! source and destination representation have been named.
//!
//! The domain backend must implement the actual conversion.
//!
//! ## Semantic preservation
//!
//! The existing translation boundary rejects conversions declared
//! potentially lossy.
//!
//! A conversion must not silently discard:
//!
//! - Types or values.
//! - Control-flow relationships.
//! - Effects.
//! - Ownership or borrowing constraints.
//! - Memory semantics.
//! - Numerical guarantees.
//! - Synchronization requirements.
//! - Resource requirements.
//! - Provenance.
//! - Domain-specific computational meaning.
//!
//! Unsupported or unresolved conversions must remain distinguishable
//! from completed conversions.
//!
//! ## Verification
//!
//! Source verification is enforced through `VerifiedIrInput`.
//!
//! The canonical lowering boundary also checks the candidate's
//! explicit `source_verified` evidence.
//!
//! This module does not implement another verifier.
//!
//! A successful conversion does not automatically prove that the
//! destination representation is valid. The destination must pass
//! its own authoritative verifier before downstream consumers trust it.
//!
//! ## Scalability and POCO-REAF
//!
//! This module introduces no fixed limits on:
//!
//! - Programs or modules.
//! - Functions, regions, or blocks.
//! - Operations or values.
//! - Types or effects.
//! - Qubits or signals.
//! - Registers or tensor dimensions.
//! - Tensor rank.
//! - Devices or processors.
//! - Accelerators or distributed participants.
//! - Computing domains.
//!
//! The module does not duplicate the input IR graph.
//!
//! Collection management, conversion work, and resource consumption
//! remain the responsibility of the canonical lowering context and
//! domain-owned implementation.
//!
//! No finite software implementation can promise literal infinity.
//! Resource exhaustion must be reported rather than interpreted as
//! successful conversion.
//!
//! ## Determinism
//!
//! This module introduces:
//!
//! - No random identifiers.
//! - No process-global mutable state.
//! - No implicit hardware discovery.
//! - No implicit target selection.
//! - No filesystem or network operations.
//! - No console output.
//!
//! Determinism of the actual conversion is the responsibility of the
//! domain-owned backend and its declared conversion contract.
//!
//! ## Grammar boundary
//!
//! Source syntax belongs to the grammar and frontend.
//! Source meaning belongs to semantic analysis.
//! Canonical computational representations belong to their IR owners.
//!
//! `grammar/compile/lowering.g4` describes source-level lowering intent.
//! It does not implement this module's conversion algorithms.
//!
//! This module does not parse grammar productions or introduce
//! source-language keywords.
//!
//! ## Integration
//!
//! Parent:
//!   `src/compiler/lowering/mod.rs`
//!
//! Compiler root:
//!   `src/compiler/mod.rs`
//!
//! Canonical compiler IR:
//!   `src/compiler/ir/`
//!
//! Canonical lowering:
//!   `src/zuir/lowering/`
//!
//! Domain implementations:
//!   `src/classical/zcir/lowering/`
//!   `src/quantum/ir/`
//!   `src/hdl/hdlir/`
//!   `src/ai/`
//!   `src/hybrid/`
//!
//! ## Freeze criteria
//!
//! - The request uses canonical verification and lowering types.
//! - Conversion delegates to the existing verified lowering entry point.
//! - Lossy conversions remain rejected by the existing translation API.
//! - Backend and legality errors retain their original types.
//! - Non-legal outcomes cannot be mistaken for completed conversions.
//! - No competing conversion or IR implementation is introduced.
//! - The module is registered by its parent.
//! - Repository formatting, compilation, tests, and Clippy pass.

use crate::compiler::ir::adapters::AdapterDescriptor;

use crate::compiler::ir::input::VerifiedIrInput;

use crate::compiler::ir::lowering::{
    lower_verified_with,
    IrLoweringError,
    IrLoweringOutcome,
    IrLoweringResult,
};

use crate::zuir::lowering::api::LoweringBackend;

use crate::zuir::lowering::context::LoweringContext;

use crate::zuir::lowering::legality::LegalityCandidate;

// =============================================================================
// PUBLIC COMPATIBILITY EXPORTS
// =============================================================================

/// The canonical compiler IR lowering error.
///
/// This is re-exported rather than redefined. Callers therefore receive
/// exactly the same error type as the existing compiler IR lowering API.
pub use crate::compiler::ir::lowering::IrLoweringError as ConversionError;

/// The canonical compiler IR lowering outcome.
///
/// This is re-exported rather than redefined. The original legality
/// report, translation descriptor, artifact, and destination mappings
/// remain available through the authoritative outcome API.
pub use crate::compiler::ir::lowering::IrLoweringOutcome as ConversionOutcome;

/// The canonical compiler IR lowering result.
///
/// No second result contract is introduced.
pub type ConversionResult<T, E> = IrLoweringResult<T, E>;

// =============================================================================
// CONVERSION REQUEST
// =============================================================================

/// One verified compiler conversion request.
///
/// This type groups the inputs required by the existing verified
/// conversion and lowering infrastructure.
///
/// It does not independently validate the supplied legality evidence.
/// The canonical lowering implementation remains responsible for
/// checking the verification marker and conversion legality.
///
/// The request borrows the verified source, lowering context, and
/// backend. It owns the candidate because the canonical lowering API
/// consumes that candidate.
///
/// # Lifetimes
///
/// `'request` is the lifetime of the borrowed verification input,
/// context, and backend.
///
/// `'ir` is the lifetime of the underlying verified IR snapshot.
pub struct ConversionRequest<'request, 'ir, B> {
    verified_input: &'request VerifiedIrInput<'ir>,
    descriptor: AdapterDescriptor,
    context: &'request mut LoweringContext,
    candidate: LegalityCandidate,
    backend: &'request mut B,
}

impl<'request, 'ir, B> ConversionRequest<'request, 'ir, B> {
    /// Construct a conversion request.
    ///
    /// The constructor performs no conversion and does not claim that
    /// the supplied candidate is legal.
    ///
    /// The canonical conversion entry point performs the actual
    /// verification-marker checks and delegates legality analysis.
    #[must_use]
    pub fn new(
        verified_input: &'request VerifiedIrInput<'ir>,
        descriptor: AdapterDescriptor,
        context: &'request mut LoweringContext,
        candidate: LegalityCandidate,
        backend: &'request mut B,
    ) -> Self {
        Self {
            verified_input,
            descriptor,
            context,
            candidate,
            backend,
        }
    }

    /// Return the requested representation conversion descriptor.
    ///
    /// The descriptor is metadata supplied by the authoritative
    /// representation adapter. It does not prove that a conversion
    /// implementation exists or that conversion is legal.
    #[must_use]
    pub const fn descriptor(&self) -> AdapterDescriptor {
        self.descriptor
    }
}

// =============================================================================
// VERIFIED CONVERSION ENTRY POINT
// =============================================================================

/// Convert one verified source entity through an authoritative
/// domain-owned lowering backend.
///
/// This is the compiler/lowering conversion entry point.
///
/// The implementation delegates to
/// `crate::compiler::ir::lowering::lower_verified_with`.
///
/// It does not implement another conversion algorithm.
///
/// # Behavior
///
/// 1. Accept the verification-gated source input.
/// 2. Pass the adapter descriptor to the canonical translation boundary.
/// 3. Pass the lowering context and legality candidate to canonical
///    lowering.
/// 4. Delegate conversion-guarantee validation to the existing
///    translation implementation.
/// 5. Delegate legality analysis to the canonical ZUIR lowering layer.
/// 6. Invoke the domain backend only when direct lowering is authorized.
/// 7. Preserve the canonical lowering outcome and its error information.
///
/// # Errors
///
/// Returns the original `IrLoweringError` type.
///
/// Errors retain their original categories, including invalid verified
/// input, missing source-verification evidence, lossy conversion
/// rejection, legality failure, backend failure, and mapping failure.
///
/// # Non-legal outcomes
///
/// A successful function return does not necessarily mean that an
/// artifact was produced.
///
/// The canonical legality subsystem can return an outcome that does
/// not authorize direct lowering.
///
/// Callers must inspect `is_lowered()` and `legality()` on the returned
/// `ConversionOutcome`.
///
/// # Destination verification
///
/// A produced artifact must still pass its authoritative destination
/// verifier. Conversion success is not destination verification.
///
/// # Portability
///
/// This function does not select a physical target, allocate devices,
/// schedule operations, or prove that a particular machine can execute
/// the resulting artifact.
pub fn convert_verified_with<'request, 'ir, B>(
    request: ConversionRequest<'request, 'ir, B>,
) -> ConversionResult<B::Artifact, B::Error>
where
    B: LoweringBackend,
{
    lower_verified_with(
        request.verified_input,
        request.descriptor,
        request.context,
        request.candidate,
        request.backend,
    )
}

// =============================================================================
// API INFORMATION
// =============================================================================

/// Return the compiler-facing conversion API version.
///
/// This version identifies the request and invocation contract in this
/// module. It is independent of the Zamani language version, grammar
/// version, ZUIR version, domain IR versions, and backend ABI versions.
#[must_use]
pub const fn api_version() -> u16 {
    1
}
