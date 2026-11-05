
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — IR Adapters
//!
//! Path: `src/compiler/ir/adapters.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the compiler-facing integration boundary for adapting
//! representations that are already owned by canonical Zamani IR
//! subsystems.
//!
//! This module coordinates existing adapters. It does not create
//! another IR, identity system, type system, semantic model,
//! verifier, optimizer, or serialization format.
//!
//! ## Currently supported conversion
//!
//! The existing ZCIR interface exposes canonical ZUIR entities.
//! Consequently, the ZCIR <-> ZUIR conversion is an identity
//! conversion:
//!
//!     ZCIR entity <-> canonical ZUIR entity
//!
//! The existing implementation in:
//!
//!     crate::classical::zcir::lowering::to_zuir
//!     crate::classical::zcir::lowering::from_zuir
//!
//! remains authoritative.
//!
//! This module exposes those operations through the compiler namespace.
//!
//! ## Deliberately unsupported conversions
//!
//! This module does not claim to implement:
//!
//!     Quantum IR -> ZUIR
//!     HDLIR       -> ZUIR
//!     AI IR       -> ZUIR
//!     Tensor IR   -> ZUIR
//!     Legacy IR   -> ZUIR
//!
//! Such conversions require explicit, domain-owned semantic
//! conversion contracts and implementations.
//!
//! A representation is not convertible merely because its fields
//! resemble fields in another representation.
//!
//! Unsupported conversions must be implemented by their canonical
//! domain owners and registered here only after their contracts,
//! verification, and semantic-preservation behavior are established.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Compiler-facing adapter entry points.
//! - Conversion direction metadata.
//! - Explicit identity-conversion guarantees.
//! - Borrowed and mutable access through existing canonical adapters.
//! - Compiler-facing adapter tests.
//!
//! This module does not own:
//!
//! - ZUIR entities or identifiers.
//! - ZCIR entities or identifiers.
//! - Quantum IR entities or identifiers.
//! - HDLIR entities or identifiers.
//! - AI or tensor representations.
//! - Semantic analysis or type inference.
//! - Domain-specific conversion algorithms.
//! - Verification algorithms.
//! - Optimization or transformation passes.
//! - Hardware discovery or resource allocation.
//! - Scheduling, routing, or physical placement.
//! - Backend code generation or runtime execution.
//!
//! ## Trust and verification
//!
//! Conversion success does not imply verification success.
//!
//! Identity conversion preserves the supplied entity, but does not
//! establish that its containing IR graph is valid.
//!
//! Compiler consumers must use the canonical verification boundary
//! in `crate::compiler::ir::input` and the appropriate domain
//! verification APIs.
//!
//! Any mutation after verification invalidates the assumption that
//! the previous verification report describes the modified snapshot.
//!
//! ## Semantic-preservation contract
//!
//! The currently supported ZCIR/ZUIR conversion is lossless because
//! the two interfaces use the same canonical Rust representation.
//!
//! No reconstruction, normalization, identifier remapping,
//! operation dropping, type substitution, effect removal,
//! control-flow rewriting, or metadata rewriting occurs.
//!
//! Future non-identity adapters must explicitly document:
//!
//! - Source and destination representation versions.
//! - Supported source constructs.
//! - Destination constructs.
//! - Type and value mapping.
//! - Identity preservation or remapping.
//! - Control-flow preservation.
//! - Effect preservation.
//! - Ownership preservation.
//! - Resource-requirement preservation.
//! - Provenance and source-map preservation.
//! - Unsupported constructs.
//! - Lossy-conversion behavior.
//! - Verification requirements.
//! - Determinism guarantees.
//!
//! A conversion that cannot preserve required semantics must fail.
//!
//! ## Scalability and POCO-REAF
//!
//! This module imposes no fixed maximum on:
//!
//! - Modules.
//! - Functions.
//! - Regions.
//! - Blocks.
//! - Operations.
//! - Values.
//! - Types.
//! - Qubits.
//! - Signals.
//! - Tensor dimensions.
//! - Devices.
//! - Processing units.
//! - Distributed participants.
//! - Computing domains.
//!
//! The currently supported identity adapters perform constant
//! adapter work and do not traverse or duplicate the represented IR.
//!
//! The actual representation and downstream processing remain
//! subject to available memory, address space, identifier capacity,
//! implementation support, and explicit operational budgets.
//!
//! No finite implementation can promise literal infinite resources.
//!
//! ## Determinism
//!
//! The adapters:
//!
//! - Introduce no random identifiers.
//! - Use no process-global mutable state.
//! - Perform no implicit hardware discovery.
//! - Perform no filesystem or network operations.
//! - Produce no console output.
//! - Do not select a target.
//! - Do not mutate the input.
//!
//! ## Integration
//!
//! Parent:
//!     src/compiler/ir/mod.rs
//!
//! Compiler root:
//!     src/compiler/mod.rs
//!
//! Existing authoritative adapters:
//!     src/classical/zcir/lowering/to_zuir.rs
//!     src/classical/zcir/lowering/from_zuir.rs
//!
//! Verification boundary:
//!     src/compiler/ir/input.rs
//!     src/compiler/ir/verification.rs
//!
//! Downstream:
//!     compiler analysis
//!     compiler passes
//!     compiler optimization
//!     compiler planning
//!     compiler lowering
//!     compiler backend integration
//!
//! The compiler must not register a domain conversion here until
//! the domain's actual conversion API exists and is verified.
//!
//! ## Freeze criteria
//!
//! - All adapter calls delegate to the canonical owning subsystem.
//! - No duplicate representation or identity type is introduced.
//! - Unsupported conversions are not reported as successful.
//! - Conversion success is not confused with verification.
//! - No unsafe code or arbitrary hardware ceiling is introduced.
//! - Compiler module declarations resolve.
//! - Repository formatting, compilation, tests, and Clippy pass.

use crate::classical::zcir::lowering::from_zuir::{
    as_zcir,
    as_zcir_mut,
    from_zuir,
};

use crate::classical::zcir::lowering::to_zuir::{
    as_zuir,
    as_zuir_mut,
    to_zuir,
    ZuirCanonical,
};

// =============================================================================
// ADAPTER CONTRACT VERSION
// =============================================================================

/// Public API version of the compiler IR adapter contract.
///
/// This version is independent of the Zamani language version,
/// grammar version, semantic-model version, ZUIR version, and
/// backend ABI version.
pub const IR_ADAPTERS_API_VERSION: u16 = 1;

// =============================================================================
// REPRESENTATION IDENTIFIERS
// =============================================================================

/// Identifies a representation participating in a compiler adapter.
///
/// These identifiers describe representation ownership. They do
/// not create or own the corresponding representation.
///
/// The enum is non-exhaustive so new representation families can
/// be introduced without requiring callers to assume that the
/// current list is complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Representation {
    /// Zamani Universal Intermediate Representation.
    Zuir,

    /// Zamani Classical Intermediate Representation integration.
    ///
    /// The current ZCIR interface uses canonical ZUIR entities.
    Zcir,

    /// Canonical Zamani quantum intermediate representation.
    Quantum,

    /// Canonical Zamani hardware-description intermediate representation.
    Hdl,

    /// AI computational representation.
    Ai,

    /// Canonical tensor computational representation.
    Tensor,

    /// Existing legacy IR generated by `crate::ir_gen`.
    Legacy,
}

impl Representation {
    /// Returns the stable, human-readable representation identifier.
    ///
    /// These identifiers are diagnostic labels. They are not
    /// serialization schemas or persistent artifact identifiers.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Zuir => "zuir",
            Self::Zcir => "zcir",
            Self::Quantum => "quantum-ir",
            Self::Hdl => "hdlir",
            Self::Ai => "ai-ir",
            Self::Tensor => "tensor-ir",
            Self::Legacy => "legacy-ir",
        }
    }
}

// =============================================================================
// CONVERSION GUARANTEES
// =============================================================================

/// Describes the guarantee made by a compiler adapter.
///
/// A guarantee describes the adapter's conversion behavior.
/// It does not assert that the input has passed verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConversionGuarantee {
    /// The source and destination use the same canonical Rust
    /// representation. No reconstruction or semantic transformation
    /// occurs.
    Identity,

    /// A separately implemented conversion establishes a documented
    /// semantics-preserving mapping.
    ///
    /// No adapter in this module currently claims this guarantee.
    VerifiedSemanticPreservation,

    /// The conversion may lose or approximate information.
    ///
    /// This guarantee is not acceptable for an implicit portable
    /// compiler conversion. It is provided as a descriptive contract
    /// value for explicit future APIs.
    PotentiallyLossy,
}

// =============================================================================
// ADAPTER DESCRIPTOR
// =============================================================================

/// Immutable description of a compiler-facing representation adapter.
///
/// Descriptors identify the source and destination representation
/// and state the conversion guarantee.
///
/// They do not prove that the input is valid, verified, portable,
/// target-compatible, or executable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdapterDescriptor {
    source: Representation,
    destination: Representation,
    guarantee: ConversionGuarantee,
}

impl AdapterDescriptor {
    /// Construct an adapter descriptor.
    ///
    /// This constructor is public so future domain-owned adapters
    /// can describe their contracts without depending on a compiler
    /// registry implementation.
    #[must_use]
    pub const fn new(
        source: Representation,
        destination: Representation,
        guarantee: ConversionGuarantee,
    ) -> Self {
        Self {
            source,
            destination,
            guarantee,
        }
    }

    /// Return the source representation.
    #[must_use]
    pub const fn source(self) -> Representation {
        self.source
    }

    /// Return the destination representation.
    #[must_use]
    pub const fn destination(self) -> Representation {
        self.destination
    }

    /// Return the declared conversion guarantee.
    #[must_use]
    pub const fn guarantee(self) -> ConversionGuarantee {
        self.guarantee
    }

    /// Return whether this adapter performs an identity conversion.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        matches!(self.guarantee, ConversionGuarantee::Identity)
    }
}

// =============================================================================
// CURRENTLY AVAILABLE ADAPTER DESCRIPTORS
// =============================================================================

/// Descriptor for the existing ZCIR-to-ZUIR identity adapter.
pub const ZCIR_TO_ZUIR: AdapterDescriptor = AdapterDescriptor::new(
    Representation::Zcir,
    Representation::Zuir,
    ConversionGuarantee::Identity,
);

/// Descriptor for the existing ZUIR-to-ZCIR identity adapter.
pub const ZUIR_TO_ZCIR: AdapterDescriptor = AdapterDescriptor::new(
    Representation::Zuir,
    Representation::Zcir,
    ConversionGuarantee::Identity,
);

// =============================================================================
// ZCIR -> ZUIR
// =============================================================================

/// Compiler-facing adapter from the ZCIR integration boundary
/// to canonical ZUIR.
///
/// The current ZCIR API exposes canonical ZUIR entities. Therefore,
/// the conversion delegates to the existing authoritative
/// `classical::zcir::lowering::to_zuir::to_zuir` function.
///
/// No entity is reconstructed, cloned, normalized, or remapped.
///
/// This function does not verify the containing IR graph.
///
/// # Type requirement
///
/// `T` must implement the canonical `ZuirCanonical` marker.
///
/// The marker is defined and implemented by the existing ZCIR
/// identity adapter. This module does not introduce another marker.
#[must_use = "the canonical ZUIR entity must be used"]
pub fn zcir_to_zuir<T: ZuirCanonical>(entity: T) -> T {
    to_zuir(entity)
}

/// Borrow a ZCIR-facing canonical entity as immutable ZUIR.
///
/// No cloning, allocation, or representation conversion occurs.
///
/// The returned reference has the lifetime of the supplied entity.
#[must_use]
pub fn zcir_as_zuir<T: ZuirCanonical>(entity: &T) -> &T {
    as_zuir(entity)
}

/// Borrow a ZCIR-facing canonical entity mutably through the ZUIR
/// identity boundary.
///
/// This function does not bypass the canonical entity's own
/// visibility, validation, or ownership rules.
///
/// Any mutation that may invalidate previous verification results
/// requires the affected IR to be verified again.
#[must_use]
pub fn zcir_as_zuir_mut<T: ZuirCanonical>(entity: &mut T) -> &mut T {
    as_zuir_mut(entity)
}

// =============================================================================
// ZUIR -> ZCIR
// =============================================================================

/// Compiler-facing adapter from canonical ZUIR to the ZCIR
/// integration boundary.
///
/// The current ZCIR API uses canonical ZUIR entities. The conversion
/// therefore delegates to the existing authoritative
/// `classical::zcir::lowering::from_zuir::from_zuir` function.
///
/// This is an identity conversion, not a conversion into a second
/// classical IR graph.
#[must_use = "the canonical ZCIR-facing entity must be used"]
pub fn zuir_to_zcir<T: ZuirCanonical>(entity: T) -> T {
    from_zuir(entity)
}

/// Borrow a canonical ZUIR entity through the ZCIR-facing API.
///
/// No clone, allocation, reconstruction, or identity remapping occurs.
#[must_use]
pub fn zuir_as_zcir<T: ZuirCanonical>(entity: &T) -> &T {
    as_zcir(entity)
}

/// Borrow a canonical ZUIR entity mutably through the ZCIR-facing API.
///
/// The caller remains responsible for re-verification after any
/// mutation that could invalidate an earlier verification result.
#[must_use]
pub fn zuir_as_zcir_mut<T: ZuirCanonical>(entity: &mut T) -> &mut T {
    as_zcir_mut(entity)
}

// =============================================================================
// ADAPTER AVAILABILITY
// =============================================================================

/// Describes whether a compiler-facing conversion is implemented.
///
/// This is an explicit availability query, not an adapter registry.
/// It prevents callers from interpreting a representation's existence
/// as proof that a conversion between representations exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdapterAvailability {
    /// The conversion is implemented and exposed by this module.
    Available(AdapterDescriptor),

    /// The representation exists, but this compiler adapter module
    /// does not currently implement the requested conversion.
    ///
    /// A domain-owned conversion may exist elsewhere; callers must
    /// consult that subsystem's documented API rather than assuming
    /// that this module implements it.
    NotProvided {
        /// Requested source representation.
        source: Representation,

        /// Requested destination representation.
        destination: Representation,
    },
}

/// Query the availability of a compiler-facing conversion.
///
/// Only the currently implemented ZCIR/ZUIR identity conversions
/// return `Available`.
///
/// In particular, this function does not claim that quantum,
/// HDL, AI, tensor, or legacy IR can be converted to ZUIR through
/// this module.
#[must_use]
pub const fn adapter_availability(
    source: Representation,
    destination: Representation,
) -> AdapterAvailability {
    match (source, destination) {
        (Representation::Zcir, Representation::Zuir) => {
            AdapterAvailability::Available(ZCIR_TO_ZUIR)
        }

        (Representation::Zuir, Representation::Zcir) => {
            AdapterAvailability::Available(ZUIR_TO_ZCIR)
        }

        _ => AdapterAvailability::NotProvided {
            source,
            destination,
        },
    }
}

// =============================================================================
// API VERSION
// =============================================================================

/// Return the compiler IR adapter API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_ADAPTERS_API_VERSION
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::BlockId;
    use crate::zuir::ir::ZuirBlock;

    fn block_id(value: u128) -> BlockId {
        BlockId::new(value).expect("test block ID must be nonzero")
    }

    #[test]
    fn adapter_api_version_is_available() {
        assert_eq!(api_version(), IR_ADAPTERS_API_VERSION);
        assert_eq!(IR_ADAPTERS_API_VERSION, 1);
    }

    #[test]
    fn zcir_to_zuir_descriptor_is_identity() {
        assert_eq!(ZCIR_TO_ZUIR.source(), Representation::Zcir);
        assert_eq!(ZCIR_TO_ZUIR.destination(), Representation::Zuir);
        assert_eq!(
            ZCIR_TO_ZUIR.guarantee(),
            ConversionGuarantee::Identity
        );
        assert!(ZCIR_TO_ZUIR.is_identity());
    }

    #[test]
    fn zuir_to_zcir_descriptor_is_identity() {
        assert_eq!(ZUIR_TO_ZCIR.source(), Representation::Zuir);
        assert_eq!(ZUIR_TO_ZCIR.destination(), Representation::Zcir);
        assert_eq!(
            ZUIR_TO_ZCIR.guarantee(),
            ConversionGuarantee::Identity
        );
        assert!(ZUIR_TO_ZCIR.is_identity());
    }

    #[test]
    fn zcir_to_zuir_preserves_canonical_block_identity() {
        let id = block_id(1);
        let block = ZuirBlock::new(id);

        let converted = zcir_to_zuir(block);

        assert_eq!(converted.id(), id);
    }

    #[test]
    fn zuir_to_zcir_preserves_canonical_block_identity() {
        let id = block_id(2);
        let block = ZuirBlock::new(id);

        let converted = zuir_to_zcir(block);

        assert_eq!(converted.id(), id);
    }

    #[test]
    fn borrowed_zcir_to_zuir_does_not_replace_entity() {
        let id = block_id(3);
        let block = ZuirBlock::new(id);

        let converted = zcir_as_zuir(&block);

        assert_eq!(converted.id(), id);
        assert_eq!(block.id(), id);
    }

    #[test]
    fn borrowed_zuir_to_zcir_does_not_replace_entity() {
        let id = block_id(4);
        let block = ZuirBlock::new(id);

        let converted = zuir_as_zcir(&block);

        assert_eq!(converted.id(), id);
        assert_eq!(block.id(), id);
    }

    #[test]
    fn mutable_identity_adapters_preserve_entity_identity() {
        let id = block_id(5);
        let mut block = ZuirBlock::new(id);

        {
            let converted = zcir_as_zuir_mut(&mut block);
            assert_eq!(converted.id(), id);
        }

        {
            let converted = zuir_as_zcir_mut(&mut block);
            assert_eq!(converted.id(), id);
        }

        assert_eq!(block.id(), id);
    }

    #[test]
    fn zcir_to_zuir_is_reported_as_available() {
        assert_eq!(
            adapter_availability(
                Representation::Zcir,
                Representation::Zuir
            ),
            AdapterAvailability::Available(ZCIR_TO_ZUIR)
        );
    }

    #[test]
    fn zuir_to_zcir_is_reported_as_available() {
        assert_eq!(
            adapter_availability(
                Representation::Zuir,
                Representation::Zcir
            ),
            AdapterAvailability::Available(ZUIR_TO_ZCIR)
        );
    }

    #[test]
    fn quantum_conversion_is_not_falsely_reported_as_available() {
        assert_eq!(
            adapter_availability(
                Representation::Quantum,
                Representation::Zuir
            ),
            AdapterAvailability::NotProvided {
                source: Representation::Quantum,
                destination: Representation::Zuir,
            }
        );
    }

    #[test]
    fn hdl_conversion_is_not_falsely_reported_as_available() {
        assert_eq!(
            adapter_availability(
                Representation::Hdl,
                Representation::Zuir
            ),
            AdapterAvailability::NotProvided {
                source: Representation::Hdl,
                destination: Representation::Zuir,
            }
        );
    }

    #[test]
    fn ai_and_tensor_conversions_are_not_assumed() {
        for source in [Representation::Ai, Representation::Tensor] {
            assert!(matches!(
                adapter_availability(source, Representation::Zuir),
                AdapterAvailability::NotProvided { .. }
            ));
        }
    }

    #[test]
    fn legacy_conversion_is_not_assumed() {
        assert!(matches!(
            adapter_availability(
                Representation::Legacy,
                Representation::Zuir
            ),
            AdapterAvailability::NotProvided { .. }
        ));
    }

    #[test]
    fn representation_identifiers_are_stable() {
        assert_eq!(Representation::Zuir.as_str(), "zuir");
        assert_eq!(Representation::Zcir.as_str(), "zcir");
        assert_eq!(Representation::Quantum.as_str(), "quantum-ir");
        assert_eq!(Representation::Hdl.as_str(), "hdlir");
        assert_eq!(Representation::Ai.as_str(), "ai-ir");
        assert_eq!(Representation::Tensor.as_str(), "tensor-ir");
        assert_eq!(Representation::Legacy.as_str(), "legacy-ir");
    }
}
