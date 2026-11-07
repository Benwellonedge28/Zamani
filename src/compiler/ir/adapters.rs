
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Canonical IR Adapters
//!
//! Path: `src/compiler/ir/adapters.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the compiler-facing boundary for invoking representation
//! adapters owned by the canonical Zamani IR subsystems.
//!
//! This module coordinates existing adapters. It does not define
//! another intermediate representation, semantic model, type system,
//! identity system, verifier, optimizer, lowering engine, or backend.
//!
//! ## Canonical ownership
//!
//! The following modules remain authoritative:
//!
//! - `crate::zuir`: universal intermediate representation.
//! - `crate::classical::zcir`: classical IR integration.
//! - `crate::quantum::ir`: quantum computational representation.
//! - `crate::hdl::hdlir`: hardware-description representation.
//! - `crate::ai`: AI computational domain.
//! - `crate::hybrid`: hybrid computation integration.
//! - `crate::distributed`: distributed computation integration.
//!
//! The frontend and semantic model remain authoritative for source
//! syntax, names, types, effects, requirements, capabilities,
//! constraints, preferences, and semantic provenance.
//!
//! ## Currently implemented conversion
//!
//! The current ZCIR representation exposes canonical ZUIR entities.
//! Therefore, the supported ZCIR/ZUIR conversion is an identity
//! conversion.
//!
//! The authoritative implementation remains:
//!
//! - `crate::classical::zcir::lowering::to_zuir`.
//! - `crate::classical::zcir::lowering::from_zuir`.
//!
//! This module delegates to those implementations.
//!
//! ## Unsupported conversions
//!
//! This module does not currently implement:
//!
//! - Quantum IR to ZUIR.
//! - HDLIR to ZUIR.
//! - AI IR to ZUIR.
//! - Tensor IR to ZUIR.
//! - Hybrid IR to ZUIR.
//! - Distributed IR to ZUIR.
//! - Legacy IR to ZUIR.
//!
//! The existence of a representation does not prove that a conversion
//! exists. A future conversion must be implemented by its authoritative
//! domain subsystem and exposed here only after its contract is defined.
//!
//! ## Conversion and verification
//!
//! Conversion success is not verification success.
//!
//! Identity conversion preserves the supplied entity. It does not prove
//! that the entity or its containing program is structurally valid,
//! semantically valid, target-compatible, portable, or executable.
//!
//! Verification is delegated to the canonical verifier for the source
//! and destination representation.
//!
//! A mutation after verification invalidates any assumption that the
//! earlier verification report describes the modified snapshot.
//!
//! ## Conversion guarantees
//!
//! A descriptor records the guarantee declared by an adapter.
//!
//! A descriptor is metadata, not cryptographic evidence or an
//! independently verified proof of semantic preservation.
//!
//! Only the adapter implementation and its owning domain contract
//! can establish whether a conversion actually preserves semantics.
//!
//! Potentially lossy conversions must not be used as implicit
//! portable translations.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary universal ceiling on:
//!
//! - Programs, modules, functions, regions, or blocks.
//! - Operations, values, types, or effects.
//! - Qubits, signals, registers, or tensor dimensions.
//! - Tensor rank.
//! - Devices, processors, or accelerators.
//! - Distributed participants.
//! - Computing domains.
//!
//! The currently implemented identity adapters do not traverse,
//! reconstruct, or duplicate the represented IR.
//!
//! Actual representation and conversion work remains subject to
//! available memory, address space, identifier capacity, implementation
//! support, target capabilities, and explicit operational budgets.
//!
//! Literal infinite physical resources are not promised.
//!
//! ## Non-responsibilities
//!
//! This module does not:
//!
//! - Parse Zamani source or grammar.
//! - Construct AST nodes.
//! - Perform semantic analysis or type inference.
//! - Allocate canonical IR identities.
//! - Reimplement domain conversion algorithms.
//! - Verify IR.
//! - Select hardware or backend providers.
//! - Allocate physical resources.
//! - Schedule, route, or place computation.
//! - Generate backend code.
//! - Execute programs.
//! - Serialize or deserialize IR.
//!
//! ## Integration
//!
//! Parent:
//!     `crate::compiler::ir`
//!
//! Canonical ZCIR adapters:
//!     `crate::classical::zcir::lowering::to_zuir`
//!     `crate::classical::zcir::lowering::from_zuir`
//!
//! Verification:
//!     `crate::compiler::ir::input`
//!     `crate::compiler::ir::verification`
//!
//! Translation:
//!     `crate::compiler::ir::translation`
//!
//! Lowering:
//!     `crate::compiler::ir::lowering`
//!
//! Downstream consumers:
//!     compiler analysis, passes, optimization, planning,
//!     lowering, and backend integration.
//!
//! ## Freeze criteria
//!
//! - All implemented conversions delegate to their canonical owner.
//! - No competing representation or identity system is introduced.
//! - Unsupported conversions are never reported as available.
//! - Conversion guarantees are not confused with verification evidence.
//! - No arbitrary universal resource ceiling is introduced.
//! - No unsafe Rust is introduced.
//! - All public APIs and module declarations resolve.
//! - Repository formatting, compilation, tests, and Clippy pass.

// =============================================================================
// CANONICAL ADAPTER IMPORTS
// =============================================================================

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
// API VERSION
// =============================================================================

/// Version of the compiler-facing IR adapter contract.
///
/// This version is independent of the language, grammar, semantic
/// model, ZUIR schema, domain IR, and backend ABI versions.
pub const IR_ADAPTERS_API_VERSION: u16 = 2;

/// Return the compiler IR adapter API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_ADAPTERS_API_VERSION
}

// =============================================================================
// REPRESENTATION IDENTIFIERS
// =============================================================================

/// Identifies a representation participating in an adapter contract.
///
/// These identifiers describe representation ownership. They do not
/// define or own the corresponding representation.
///
/// Known representations are named explicitly for compatibility
/// and convenient compiler integration.
///
/// `Extension` provides an identifier for additional representation
/// families without requiring this module to enumerate every future
/// computing domain.
///
/// Extension identifiers are static diagnostic/API identifiers.
/// They are not serialization schemas or globally unique provider IDs.
///
/// This type does not establish that an adapter exists for a pair
/// of representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Representation {
    /// Zamani Universal Intermediate Representation.
    Zuir,

    /// Zamani Classical Intermediate Representation integration.
    ///
    /// The current ZCIR API exposes canonical ZUIR entities.
    Zcir,

    /// Canonical quantum intermediate representation.
    Quantum,

    /// Hardware-description intermediate representation.
    Hdl,

    /// AI computational representation.
    Ai,

    /// Canonical tensor computational representation.
    Tensor,

    /// Existing legacy IR representation.
    Legacy,

    /// Additional representation identified by an extension-owned name.
    ///
    /// The identifier should be stable, non-empty, and namespaced by
    /// the subsystem that owns the representation.
    Extension(&'static str),
}

impl Representation {
    /// Return the human-readable representation identifier.
    ///
    /// The returned string is intended for diagnostics and API
    /// descriptions. It is not a persistent serialization identity.
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
            Self::Extension(identifier) => identifier,
        }
    }

    /// Return whether the representation identifier is non-empty.
    ///
    /// Built-in identifiers are always valid. Extension identifiers
    /// must contain at least one character.
    #[must_use]
    pub const fn is_well_formed(self) -> bool {
        !self.as_str().is_empty()
    }

    /// Return whether this is an extension-owned representation.
    #[must_use]
    pub const fn is_extension(self) -> bool {
        matches!(self, Self::Extension(_))
    }
}

// =============================================================================
// CONVERSION GUARANTEES
// =============================================================================

/// Describes the conversion guarantee declared by an adapter.
///
/// This describes the adapter contract. It does not independently
/// prove that the adapter implementation satisfies the declaration.
///
/// Verification of the source and destination remains a separate
/// responsibility of their canonical verification authorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConversionGuarantee {
    /// Source and destination use the same canonical representation.
    ///
    /// No reconstruction or semantic transformation occurs.
    Identity,

    /// The adapter declares that its conversion preserves the
    /// documented source semantics.
    ///
    /// The owning domain must provide the actual conversion contract,
    /// implementation, and tests that establish this guarantee.
    VerifiedSemanticPreservation,

    /// The conversion may lose, approximate, or omit information.
    ///
    /// This guarantee must not be accepted as an implicit portable
    /// translation.
    PotentiallyLossy,
}

impl ConversionGuarantee {
    /// Return whether this guarantee declares semantic preservation.
    ///
    /// This checks the declaration only. It does not prove the
    /// implementation's correctness.
    #[must_use]
    pub const fn declares_semantic_preservation(self) -> bool {
        matches!(
            self,
            Self::Identity | Self::VerifiedSemanticPreservation
        )
    }

    /// Return whether this guarantee describes a potentially
    /// information-losing conversion.
    #[must_use]
    pub const fn is_potentially_lossy(self) -> bool {
        matches!(self, Self::PotentiallyLossy)
    }
}

// =============================================================================
// ADAPTER DESCRIPTOR
// =============================================================================

/// Immutable metadata describing one representation conversion.
///
/// A descriptor records the direction and declared guarantee of an
/// adapter. It does not implement the conversion and does not prove
/// that the conversion exists or is correct.
///
/// Actual conversion must be delegated to an implementation owned
/// by the authoritative source or destination domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdapterDescriptor {
    source: Representation,
    destination: Representation,
    guarantee: ConversionGuarantee,
}

impl AdapterDescriptor {
    /// Construct adapter metadata.
    ///
    /// This constructor does not register an adapter or implement
    /// a conversion. Callers must only declare guarantees that are
    /// established by the actual authoritative adapter implementation.
    ///
    /// In particular, declaring `VerifiedSemanticPreservation` is
    /// not a substitute for semantic-preservation tests or verification.
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

    /// Return whether this descriptor declares an identity conversion.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        matches!(self.guarantee, ConversionGuarantee::Identity)
    }

    /// Return whether this descriptor declares semantic preservation.
    ///
    /// This reports the declaration, not independent proof.
    #[must_use]
    pub const fn declares_semantic_preservation(self) -> bool {
        self.guarantee.declares_semantic_preservation()
    }

    /// Return whether both representation identifiers are well formed.
    #[must_use]
    pub const fn is_well_formed(self) -> bool {
        self.source.is_well_formed()
            && self.destination.is_well_formed()
    }
}

// =============================================================================
// IMPLEMENTED ADAPTER DESCRIPTORS
// =============================================================================

/// Descriptor for the implemented ZCIR-to-ZUIR identity adapter.
pub const ZCIR_TO_ZUIR: AdapterDescriptor = AdapterDescriptor::new(
    Representation::Zcir,
    Representation::Zuir,
    ConversionGuarantee::Identity,
);

/// Descriptor for the implemented ZUIR-to-ZCIR identity adapter.
pub const ZUIR_TO_ZCIR: AdapterDescriptor = AdapterDescriptor::new(
    Representation::Zuir,
    Representation::Zcir,
    ConversionGuarantee::Identity,
);

// =============================================================================
// ZCIR -> ZUIR
// =============================================================================

/// Adapt a canonical ZCIR-facing entity into the ZUIR-facing API.
///
/// The current ZCIR API exposes canonical ZUIR entities. Therefore,
/// this operation delegates to the authoritative identity adapter.
///
/// No entity is reconstructed, cloned, normalized, traversed, or
/// remapped.
///
/// This function does not verify the entity or its containing graph.
///
/// `T` must implement the canonical `ZuirCanonical` marker owned by
/// `crate::classical::zcir::lowering::to_zuir`.
///
/// # Complexity
///
/// O(1) adapter work, excluding normal Rust ownership transfer.
#[must_use = "the canonical ZUIR entity must be used"]
pub fn zcir_to_zuir<T: ZuirCanonical>(entity: T) -> T {
    to_zuir(entity)
}

/// Borrow a canonical ZCIR-facing entity through the ZUIR API.
///
/// No clone, allocation, graph traversal, or identity remapping occurs.
///
/// The returned reference has the same lifetime as the supplied entity.
#[must_use]
pub fn zcir_as_zuir<T: ZuirCanonical>(entity: &T) -> &T {
    as_zuir(entity)
}

/// Borrow a canonical ZCIR-facing entity mutably through the ZUIR API.
///
/// This delegates to the authoritative identity adapter and does not
/// bypass the canonical entity's visibility or invariants.
///
/// Any mutation that can invalidate an earlier verification result
/// requires the affected IR snapshot to be verified again.
#[must_use]
pub fn zcir_as_zuir_mut<T: ZuirCanonical>(entity: &mut T) -> &mut T {
    as_zuir_mut(entity)
}

// =============================================================================
// ZUIR -> ZCIR
// =============================================================================

/// Adapt a canonical ZUIR entity through the ZCIR-facing API.
///
/// The current ZCIR API exposes aliases and re-exports of canonical
/// ZUIR entities. This operation therefore delegates to the existing
/// authoritative identity adapter.
///
/// It does not construct a second classical IR graph or verify the
/// supplied entity.
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
/// The caller remains responsible for re-verification after a mutation
/// that could invalidate previously established verification evidence.
#[must_use]
pub fn zuir_as_zcir_mut<T: ZuirCanonical>(entity: &mut T) -> &mut T {
    as_zcir_mut(entity)
}

// =============================================================================
// ADAPTER AVAILABILITY
// =============================================================================

/// Describes whether this compiler adapter module implements a
/// requested representation conversion.
///
/// `Available` means that this module exposes a concrete adapter entry
/// point for the requested direction.
///
/// `NotProvided` means that this module does not expose that conversion.
/// It does not prove that no domain-owned adapter exists elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AdapterAvailability {
    /// This module exposes the requested adapter.
    Available(AdapterDescriptor),

    /// This module does not expose the requested adapter.
    NotProvided {
        /// Requested source representation.
        source: Representation,

        /// Requested destination representation.
        destination: Representation,
    },
}

impl AdapterAvailability {
    /// Return whether this module exposes an adapter for the request.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available(_))
    }

    /// Return the adapter descriptor when this module exposes one.
    #[must_use]
    pub const fn descriptor(self) -> Option<AdapterDescriptor> {
        match self {
            Self::Available(descriptor) => Some(descriptor),
            Self::NotProvided { .. } => None,
        }
    }
}

/// Query adapter availability within this compiler integration module.
///
/// Only the ZCIR/ZUIR identity conversions are currently available.
///
/// Quantum, HDL, AI, tensor, hybrid, distributed, and legacy
/// representations are not reported as convertible merely because
/// their owning subsystems exist.
///
/// This query is deliberately not a universal adapter registry.
/// Future domain adapters must be implemented by their owning
/// subsystems and explicitly integrated when their contracts exist.
#[must_use]
pub const fn adapter_availability(
    source: Representation,
    destination: Representation,
) -> AdapterAvailability {
    if !source.is_well_formed() || !destination.is_well_formed() {
        return AdapterAvailability::NotProvided {
            source,
            destination,
        };
    }

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
        assert_eq!(IR_ADAPTERS_API_VERSION, 2);
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
        assert!(ZCIR_TO_ZUIR.declares_semantic_preservation());
        assert!(ZCIR_TO_ZUIR.is_well_formed());
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
        assert!(ZUIR_TO_ZCIR.is_well_formed());
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
    fn borrowed_zcir_to_zuir_preserves_the_original_entity() {
        let id = block_id(3);
        let block = ZuirBlock::new(id);

        let converted = zcir_as_zuir(&block);

        assert_eq!(converted.id(), id);
        assert_eq!(block.id(), id);
    }

    #[test]
    fn borrowed_zuir_to_zcir_preserves_the_original_entity() {
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
    fn hybrid_and_distributed_conversions_are_not_assumed() {
        for source in [
            Representation::Extension("zamani.hybrid"),
            Representation::Extension("zamani.distributed"),
        ] {
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
    fn extension_representations_have_extensible_identifiers() {
        let representation = Representation::Extension("vendor.domain.ir");

        assert_eq!(representation.as_str(), "vendor.domain.ir");
        assert!(representation.is_extension());
        assert!(representation.is_well_formed());
    }

    #[test]
    fn empty_extension_identifiers_are_rejected_by_availability() {
        let source = Representation::Extension("");

        assert!(!source.is_well_formed());

        assert!(matches!(
            adapter_availability(source, Representation::Zuir),
            AdapterAvailability::NotProvided { .. }
        ));
    }

    #[test]
    fn conversion_guarantees_remain_distinct() {
        assert!(ConversionGuarantee::Identity
            .declares_semantic_preservation());

        assert!(ConversionGuarantee::VerifiedSemanticPreservation
            .declares_semantic_preservation());

        assert!(!ConversionGuarantee::PotentiallyLossy
            .declares_semantic_preservation());

        assert!(ConversionGuarantee::PotentiallyLossy
            .is_potentially_lossy());
    }

    #[test]
    fn descriptor_does_not_claim_a_conversion_exists() {
        let descriptor = AdapterDescriptor::new(
            Representation::Extension("future.domain"),
            Representation::Zuir,
            ConversionGuarantee::VerifiedSemanticPreservation,
        );

        assert!(descriptor.is_well_formed());

        // Descriptive metadata alone does not register an adapter.
        assert!(matches!(
            adapter_availability(
                descriptor.source(),
                descriptor.destination()
            ),
            AdapterAvailability::NotProvided { .. }
        ));
    }

    #[test]
    fn representation_identifiers_remain_stable() {
        assert_eq!(Representation::Zuir.as_str(), "zuir");
        assert_eq!(Representation::Zcir.as_str(), "zcir");
        assert_eq!(Representation::Quantum.as_str(), "quantum-ir");
        assert_eq!(Representation::Hdl.as_str(), "hdlir");
        assert_eq!(Representation::Ai.as_str(), "ai-ir");
        assert_eq!(Representation::Tensor.as_str(), "tensor-ir");
        assert_eq!(Representation::Legacy.as_str(), "legacy-ir");
    }
}
