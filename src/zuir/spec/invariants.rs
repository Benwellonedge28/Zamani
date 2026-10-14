#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR — Canonical Invariant Catalogue
//!
//! Path: `src/zuir/spec/invariants.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the stable, machine-readable catalogue of invariants governing
//! Zamani's Universal Intermediate Representation (ZUIR).
//!
//! This module specifies:
//! - stable identifiers for universal invariants;
//! - the scope and applicability of each invariant;
//! - the authoritative verification owner for each invariant;
//! - the conditions under which a check is mandatory;
//! - extension-defined invariant identifier validation;
//! - the relationship between invariant definitions, verification,
//!   conformance testing, and semantic preservation.
//!
//! This is a contract catalogue, not an IR verifier. It does not inspect
//! concrete IR nodes, traverse programs, execute operations, or claim that
//! an invariant has been verified.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/config.rs`
//! - `src/zuir/diagnostics.rs`
//!
//! The normative specifications define meaning. This file identifies
//! the invariants and assigns their verification ownership.
//! Concrete verification algorithms belong to the verifier subsystem.
//!
//! ## OWNS
//!
//! - Core invariant identifiers and descriptors.
//! - Invariant classification and applicability.
//! - Verification ownership declarations.
//! - Extension invariant namespace validation.
//! - The invariant catalogue's consistency tests.
//!
//! ## DOES_NOT_OWN
//!
//! - IR nodes, types, operations, blocks, regions, or control-flow graphs.
//! - Identifier allocation or global identity uniqueness.
//! - AST structure or source-language semantic analysis.
//! - Concrete verification algorithms.
//! - Dialect registration or executable extension hooks.
//! - Target capabilities, physical resource allocation, or scheduling.
//! - Diagnostics rendering, serialization, or error propagation.
//! - A claim that every listed invariant is already implemented.
//!
//! ## INTEGRATION CONTRACT
//!
//! `spec/mod.rs` should expose this module:
//!
//!     pub mod invariants;
//!
//! `verify/structural.rs` owns structural and reference-integrity checks.
//! `verify/types.rs` owns type correctness.
//! `verify/control_flow.rs` owns control-flow correctness.
//! `verify/dominance.rs` owns SSA and dominance checks, if applicable.
//! `verify/effects.rs` owns effect consistency.
//! `verify/ownership.rs` owns ownership and linearity checks.
//! `verify/resources.rs` owns symbolic resource consistency.
//! `verify/dialects.rs` owns registered dialect and operation checks.
//! `verify/full.rs` orchestrates mandatory verification stages.
//!
//! These paths are ownership contracts for the intended architecture.
//! The existence and signatures of those modules must be verified before
//! wiring imports or claiming end-to-end integration.
//!
//! Builders may reject invalid local construction early but must not
//! become a competing authority for whole-program verification.
//!
//! ## SCALABILITY
//!
//! The core catalogue contains semantic rules, not hardware capacities.
//! It defines no universal maximum for program size, operation count,
//! qubits, processors, devices, tensor rank, register width, or nodes.
//!
//! Actual verification consumes finite resources. Operational budgets
//! belong to invocation configuration and must produce explicit outcomes
//! when exhausted. Resource exhaustion is not proof of semantic invalidity.
//!
//! ## SSA POLICY
//!
//! SSA-specific invariants are conditional. The normative ZUIR specification
//! must settle the canonical value model before those checks are enabled.
//! Specialized domains may use a different value model if their semantics
//! and verification rules are explicitly declared.
//!
//! ## EXTENSIBILITY
//!
//! The core catalogue is intentionally finite and stable. New domain
//! operations and domain-specific invariants must not require adding every
//! operation to this file. Extensions identify their invariants using
//! validated, namespaced identifiers and register their own verification
//! contracts through the dialect/extension subsystem.
//!
//! An unknown invariant identifier must never be interpreted as verified.
//!
//! ## FREEZE CRITERIA
//!
//! - Each core invariant has a unique stable identifier.
//! - Every invariant has an explicit authoritative owner.
//! - Conditional checks declare their applicability.
//! - Extension identifiers cannot collide with core identifiers.
//! - Resource exhaustion cannot be confused with successful verification.
//! - No target-specific or hardware-capacity assumptions are introduced.
//! - Tests validate catalogue consistency and namespace rules.

use core::fmt;

/// Stable identifier for a universal ZUIR invariant.
///
/// The string value is part of the verification and conformance contract.
/// Existing identifiers must not be renamed without an explicit
/// compatibility and migration decision.
pub mod id {
    /// Every referenced entity must exist in the applicable identity scope.
    pub const REFERENCE_INTEGRITY: &str = "zuir.reference_integrity";

    /// Entity identities must be unique in their declared scope.
    pub const IDENTITY_SCOPE: &str = "zuir.identity_scope";

    /// Structural containment must be valid and unambiguous.
    pub const STRUCTURAL_CONTAINMENT: &str = "zuir.structural_containment";

    /// Definitions and declarations must satisfy their uniqueness rules.
    pub const DEFINITION_UNIQUENESS: &str = "zuir.definition_uniqueness";

    /// Required program and module relationships must be consistent.
    pub const PROGRAM_MODULE_CONSISTENCY: &str = "zuir.program_module_consistency";

    /// Function signatures must agree with their declared interfaces.
    pub const FUNCTION_SIGNATURE: &str = "zuir.function_signature";

    /// Operand and result references must agree with operation schemas.
    pub const OPERATION_SCHEMA: &str = "zuir.operation_schema";

    /// Every value reference must have a valid definition or permitted origin.
    pub const VALUE_ORIGIN: &str = "zuir.value_origin";

    /// Type references must resolve and type rules must be respected.
    pub const TYPE_CORRECTNESS: &str = "zuir.type_correctness";

    /// Control-flow edges and terminators must be structurally consistent.
    pub const CONTROL_FLOW: &str = "zuir.control_flow";

    /// SSA definitions must be unique when the selected value model requires it.
    pub const SSA_SINGLE_DEFINITION: &str = "zuir.ssa_single_definition";

    /// SSA uses must satisfy dominance and use-legality rules when applicable.
    pub const SSA_USE_LEGALITY: &str = "zuir.ssa_use_legality";

    /// Def-use information must agree with canonical definitions and uses.
    pub const DEF_USE_CONSISTENCY: &str = "zuir.def_use_consistency";

    /// Operation effects must agree with their declared semantic contracts.
    pub const EFFECT_CONSISTENCY: &str = "zuir.effect_consistency";

    /// Ownership, borrowing, aliasing, and lifetime rules must be respected.
    pub const OWNERSHIP_CONSISTENCY: &str = "zuir.ownership_consistency";

    /// Linear and affine values must obey their applicable use rules.
    pub const LINEARITY: &str = "zuir.linearity";

    /// Resource requirements must be internally consistent and well-formed.
    pub const RESOURCE_CONSISTENCY: &str = "zuir.resource_consistency";

    /// Capability requirements must be explicit and correctly scoped.
    pub const CAPABILITY_CONSISTENCY: &str = "zuir.capability_consistency";

    /// Required dialects and operation definitions must resolve.
    pub const DIALECT_RESOLUTION: &str = "zuir.dialect_resolution";

    /// Extension schemas and versions must satisfy their compatibility rules.
    pub const EXTENSION_COMPATIBILITY: &str = "zuir.extension_compatibility";

    /// Unknown extensions must not silently acquire executable semantics.
    pub const UNKNOWN_EXTENSION_SAFETY: &str = "zuir.unknown_extension_safety";

    /// Required attributes and metadata must satisfy their schemas.
    pub const ATTRIBUTE_CONSISTENCY: &str = "zuir.attribute_consistency";

    /// Persisted or imported representations must satisfy format contracts.
    pub const REPRESENTATION_VERSION: &str = "zuir.representation_version";

    /// Canonical encoding must obey its documented determinism contract.
    pub const CANONICAL_ENCODING: &str = "zuir.canonical_encoding";

    /// Source and transformation provenance must remain well-formed when present.
    pub const PROVENANCE_CONSISTENCY: &str = "zuir.provenance_consistency";

    /// Transformations must preserve required program semantics.
    pub const TRANSFORMATION_PRESERVATION: &str =
        "zuir.transformation_preservation";

    /// Lowering must preserve meaning or explicitly reject unsupported mappings.
    pub const LOWERING_PRESERVATION: &str = "zuir.lowering_preservation";

    /// Arithmetic used in validation must not silently overflow.
    pub const CHECKED_ARITHMETIC: &str = "zuir.checked_arithmetic";

    /// Verification completion and failure states must be reported honestly.
    pub const VERIFICATION_COMPLETENESS: &str = "zuir.verification_completeness";

    /// Operational resource exhaustion must be distinguishable from invalid IR.
    pub const RESOURCE_EXHAUSTION: &str = "zuir.resource_exhaustion";

    /// Semantic meaning must not depend accidentally on target identity.
    pub const TARGET_INDEPENDENCE: &str = "zuir.target_independence";

    /// Ordering constraints and observable effects must be preserved.
    pub const OBSERVABLE_ORDERING: &str = "zuir.observable_ordering";

    /// Numeric transformations must respect the declared numerical contract.
    pub const NUMERICAL_SEMANTICS: &str = "zuir.numerical_semantics";

    /// Concurrent and distributed behavior must respect declared contracts.
    pub const CONCURRENCY_CONSISTENCY: &str = "zuir.concurrency_consistency";

    /// Domain-specific semantic requirements must be checked by their owner.
    pub const DOMAIN_CONFORMANCE: &str = "zuir.domain_conformance";
}

/// Classification of an invariant's normative scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvariantClass {
    /// Applies to every accepted ZUIR representation.
    Universal,

    /// Applies only when a declared representation model or feature requires it.
    Conditional,

    /// Applies to a registered domain or extension.
    DomainSpecific,
}

/// Declares when an invariant must be enforced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Applicability {
    /// Mandatory for every representation accepted as valid.
    Always,

    /// Mandatory only when the corresponding representation model is enabled.
    WhenSsaIsEnabled,

    /// Mandatory when the representation contains cached def-use information.
    WhenDefUseIndexesAreStored,

    /// Mandatory when provenance information is present.
    WhenProvenanceIsPresent,

    /// Mandatory when importing or interpreting persisted representations.
    WhenImportingOrDecoding,

    /// Mandatory when canonical encoding is requested.
    WhenCanonicalEncodingIsRequested,

    /// Mandatory when a transformation is performed.
    WhenTransforming,

    /// Mandatory when lowering to a downstream representation.
    WhenLowering,

    /// Mandatory for operations and semantics defined by a registered domain.
    ForRegisteredDomain,
}

/// Authoritative verification owner.
///
/// Values are module paths, not imports. This allows the catalogue to be
/// introduced before the concrete verifier modules are integrated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationOwner {
    Structural,
    Types,
    ControlFlow,
    Dominance,
    DefUse,
    Effects,
    Ownership,
    Resources,
    Dialects,
    Serialization,
    Provenance,
    Transformations,
    Lowering,
    NumericSemantics,
    Concurrency,
    FullVerification,
    ExtensionVerifier,
}

impl VerificationOwner {
    /// Return the canonical intended module path for this owner.
    #[must_use]
    pub const fn module_path(self) -> &'static str {
        match self {
            Self::Structural => "crate::zuir::verify::structural",
            Self::Types => "crate::zuir::verify::types",
            Self::ControlFlow => "crate::zuir::verify::control_flow",
            Self::Dominance => "crate::zuir::verify::dominance",
            Self::DefUse => "crate::zuir::verify::def_use",
            Self::Effects => "crate::zuir::verify::effects",
            Self::Ownership => "crate::zuir::verify::ownership",
            Self::Resources => "crate::zuir::verify::resources",
            Self::Dialects => "crate::zuir::verify::dialects",
            Self::Serialization => "crate::zuir::serialization::validation",
            Self::Provenance => "crate::zuir::verify::provenance",
            Self::Transformations => "crate::zuir::pass::contracts",
            Self::Lowering => "crate::zuir::lowering::legality",
            Self::NumericSemantics => "crate::zuir::verify::numeric",
            Self::Concurrency => "crate::zuir::verify::concurrency",
            Self::FullVerification => "crate::zuir::verify::full",
            Self::ExtensionVerifier => "registered extension verifier",
        }
    }
}

/// Machine-readable description of a core invariant.
///
/// Descriptions explain the contract; they are not a substitute for
/// normative specification text or executable verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvariantDescriptor {
    /// Stable, namespaced identifier.
    pub id: &'static str,

    /// Normative classification.
    pub class: InvariantClass,

    /// Condition under which this invariant must be enforced.
    pub applicability: Applicability,

    /// Authoritative verifier or verification subsystem.
    pub owner: VerificationOwner,

    /// Concise statement of the invariant.
    pub statement: &'static str,
}

/// Canonical catalogue of core ZUIR invariants.
///
/// This catalogue describes required verification coverage. It does not
/// imply that the listed verification algorithms already exist.
///
/// Domain-specific invariants are registered by extensions and are not
/// enumerated exhaustively here.
pub const CORE_INVARIANTS: &[InvariantDescriptor] = &[
    InvariantDescriptor {
        id: id::REFERENCE_INTEGRITY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Structural,
        statement: "Every reference resolves in its declared identity scope.",
    },
    InvariantDescriptor {
        id: id::IDENTITY_SCOPE,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Structural,
        statement: "Entity identities are unique within their declared scope.",
    },
    InvariantDescriptor {
        id: id::STRUCTURAL_CONTAINMENT,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Structural,
        statement: "Containment relationships are valid and unambiguous.",
    },
    InvariantDescriptor {
        id: id::DEFINITION_UNIQUENESS,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Structural,
        statement: "Definitions obey the uniqueness rules of their namespace.",
    },
    InvariantDescriptor {
        id: id::PROGRAM_MODULE_CONSISTENCY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Structural,
        statement: "Program and module membership and declarations are consistent.",
    },
    InvariantDescriptor {
        id: id::FUNCTION_SIGNATURE,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Types,
        statement: "Function inputs, outputs, and declared callable contracts agree.",
    },
    InvariantDescriptor {
        id: id::OPERATION_SCHEMA,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Dialects,
        statement: "Operations satisfy their registered schemas and constraints.",
    },
    InvariantDescriptor {
        id: id::VALUE_ORIGIN,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Structural,
        statement: "Every value has a permitted definition or explicit origin.",
    },
    InvariantDescriptor {
        id: id::TYPE_CORRECTNESS,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Types,
        statement: "Types resolve and all applicable type rules are satisfied.",
    },
    InvariantDescriptor {
        id: id::CONTROL_FLOW,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::ControlFlow,
        statement: "Blocks, edges, terminators, and successor arguments are consistent.",
    },
    InvariantDescriptor {
        id: id::SSA_SINGLE_DEFINITION,
        class: InvariantClass::Conditional,
        applicability: Applicability::WhenSsaIsEnabled,
        owner: VerificationOwner::Dominance,
        statement: "Each SSA value has exactly one permitted definition.",
    },
    InvariantDescriptor {
        id: id::SSA_USE_LEGALITY,
        class: InvariantClass::Conditional,
        applicability: Applicability::WhenSsaIsEnabled,
        owner: VerificationOwner::Dominance,
        statement: "SSA uses satisfy dominance and value-model-specific legality.",
    },
    InvariantDescriptor {
        id: id::DEF_USE_CONSISTENCY,
        class: InvariantClass::Conditional,
        applicability: Applicability::WhenDefUseIndexesAreStored,
        owner: VerificationOwner::DefUse,
        statement: "Stored def-use indexes agree with canonical definitions and uses.",
    },
    InvariantDescriptor {
        id: id::EFFECT_CONSISTENCY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Effects,
        statement: "Declared effects and effect constraints are internally consistent.",
    },
    InvariantDescriptor {
        id: id::OWNERSHIP_CONSISTENCY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Ownership,
        statement: "Applicable ownership, borrowing, aliasing, and lifetime rules hold.",
    },
    InvariantDescriptor {
        id: id::LINEARITY,
        class: InvariantClass::Conditional,
        applicability: Applicability::Always,
        owner: VerificationOwner::Ownership,
        statement: "Linear and affine resources obey their declared usage rules.",
    },
    InvariantDescriptor {
        id: id::RESOURCE_CONSISTENCY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Resources,
        statement: "Resource requirements and constraints are well-formed and consistent.",
    },
    InvariantDescriptor {
        id: id::CAPABILITY_CONSISTENCY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Resources,
        statement: "Capability requirements are explicit and correctly scoped.",
    },
    InvariantDescriptor {
        id: id::DIALECT_RESOLUTION,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Dialects,
        statement: "Required dialects and operation definitions resolve unambiguously.",
    },
    InvariantDescriptor {
        id: id::EXTENSION_COMPATIBILITY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Dialects,
        statement: "Extension and dialect versions satisfy declared compatibility rules.",
    },
    InvariantDescriptor {
        id: id::UNKNOWN_EXTENSION_SAFETY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Dialects,
        statement: "Unknown extensions never acquire implicit executable semantics.",
    },
    InvariantDescriptor {
        id: id::ATTRIBUTE_CONSISTENCY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Dialects,
        statement: "Reserved and schema-defined attributes satisfy their constraints.",
    },
    InvariantDescriptor {
        id: id::REPRESENTATION_VERSION,
        class: InvariantClass::Universal,
        applicability: Applicability::WhenImportingOrDecoding,
        owner: VerificationOwner::Serialization,
        statement: "Imported representations obey supported schema and version contracts.",
    },
    InvariantDescriptor {
        id: id::CANONICAL_ENCODING,
        class: InvariantClass::Conditional,
        applicability: Applicability::WhenCanonicalEncodingIsRequested,
        owner: VerificationOwner::Serialization,
        statement: "Canonical encoding follows the documented ordering and encoding rules.",
    },
    InvariantDescriptor {
        id: id::PROVENANCE_CONSISTENCY,
        class: InvariantClass::Conditional,
        applicability: Applicability::WhenProvenanceIsPresent,
        owner: VerificationOwner::Provenance,
        statement: "Present source and transformation provenance is well-formed.",
    },
    InvariantDescriptor {
        id: id::TRANSFORMATION_PRESERVATION,
        class: InvariantClass::Universal,
        applicability: Applicability::WhenTransforming,
        owner: VerificationOwner::Transformations,
        statement: "A transformation preserves its declared semantic contract.",
    },
    InvariantDescriptor {
        id: id::LOWERING_PRESERVATION,
        class: InvariantClass::Universal,
        applicability: Applicability::WhenLowering,
        owner: VerificationOwner::Lowering,
        statement: "Lowering preserves required meaning or explicitly rejects unsupported mappings.",
    },
    InvariantDescriptor {
        id: id::CHECKED_ARITHMETIC,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Structural,
        statement: "Validation arithmetic cannot silently overflow or wrap.",
    },
    InvariantDescriptor {
        id: id::VERIFICATION_COMPLETENESS,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::FullVerification,
        statement: "Verification never reports success when mandatory checks failed or were incomplete.",
    },
    InvariantDescriptor {
        id: id::RESOURCE_EXHAUSTION,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::FullVerification,
        statement: "Operational budget exhaustion is reported distinctly from invalid semantics.",
    },
    InvariantDescriptor {
        id: id::TARGET_INDEPENDENCE,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::FullVerification,
        statement: "Core semantics do not depend on accidental properties of a target.",
    },
    InvariantDescriptor {
        id: id::OBSERVABLE_ORDERING,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Effects,
        statement: "Observable effects and ordering constraints are preserved.",
    },
    InvariantDescriptor {
        id: id::NUMERICAL_SEMANTICS,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::NumericSemantics,
        statement: "Numerical operations respect declared precision, overflow, and rounding semantics.",
    },
    InvariantDescriptor {
        id: id::CONCURRENCY_CONSISTENCY,
        class: InvariantClass::Universal,
        applicability: Applicability::Always,
        owner: VerificationOwner::Concurrency,
        statement: "Concurrency and communication respect their declared synchronization and ordering contracts.",
    },
    InvariantDescriptor {
        id: id::DOMAIN_CONFORMANCE,
        class: InvariantClass::DomainSpecific,
        applicability: Applicability::ForRegisteredDomain,
        owner: VerificationOwner::ExtensionVerifier,
        statement: "Registered domain-specific invariants are checked by their declared verifier.",
    },
];

/// Look up a core invariant by its stable identifier.
///
/// Returns `None` for unknown or extension-defined identifiers. Unknown
/// identifiers must not be treated as verified.
#[must_use]
pub fn find_core_invariant(identifier: &str) -> Option<&'static InvariantDescriptor> {
    CORE_INVARIANTS.iter().find(|entry| entry.id == identifier)
}

/// Return whether an identifier belongs to the core invariant catalogue.
#[must_use]
pub fn is_core_invariant(identifier: &str) -> bool {
    find_core_invariant(identifier).is_some()
}

/// Error returned when an extension invariant identifier is malformed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtensionInvariantIdError {
    Empty,
    InvalidCharacter,
    InvalidNamespace,
    CoreNamespaceReserved,
}

impl fmt::Display for ExtensionInvariantIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("invariant identifier must not be empty"),
            Self::InvalidCharacter => {
                f.write_str("invariant identifier contains an invalid character")
            }
            Self::InvalidNamespace => {
                f.write_str("extension invariant identifier must be namespaced")
            }
            Self::CoreNamespaceReserved => {
                f.write_str("the 'zuir' namespace is reserved for core invariants")
            }
        }
    }
}

impl std::error::Error for ExtensionInvariantIdError {}

/// Validated identifier for an extension-defined invariant.
///
/// Extension IDs are strings rather than variants in a universal enum,
/// allowing new domains to add invariants without modifying the core.
///
/// Accepted form:
///
/// `extension_namespace.invariant_name`
///
/// Each segment must begin with an ASCII lowercase letter. Subsequent
/// characters may be lowercase ASCII letters, digits, or underscores.
/// The first segment must not be `zuir`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtensionInvariantId(String);

impl ExtensionInvariantId {
    /// Validate and construct a namespaced extension invariant identifier.
    pub fn try_new(value: impl Into<String>) -> Result<Self, ExtensionInvariantIdError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ExtensionInvariantIdError::Empty);
        }

        let mut segments = value.split('.');
        let namespace = segments
            .next()
            .ok_or(ExtensionInvariantIdError::InvalidNamespace)?;

        let invariant_name = segments
            .next()
            .ok_or(ExtensionInvariantIdError::InvalidNamespace)?;

        // Exactly two segments are required by this initial public contract.
        if segments.next().is_some()
            || namespace.is_empty()
            || invariant_name.is_empty()
        {
            return Err(ExtensionInvariantIdError::InvalidNamespace);
        }

        if namespace == "zuir" {
            return Err(ExtensionInvariantIdError::CoreNamespaceReserved);
        }

        if !valid_segment(namespace) || !valid_segment(invariant_name) {
            return Err(ExtensionInvariantIdError::InvalidCharacter);
        }

        Ok(Self(value))
    }

    /// Return the validated identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return its identifier string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ExtensionInvariantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn valid_segment(segment: &str) -> bool {
    let mut chars = segment.chars();

    match chars.next() {
        Some(first) if first.is_ascii_lowercase() => {}
        _ => return false,
    }

    chars.all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || character == '_'
    })
}

/// Resolve a stable core invariant ID to its verification owner.
#[must_use]
pub fn verification_owner(identifier: &str) -> Option<VerificationOwner> {
    find_core_invariant(identifier).map(|descriptor| descriptor.owner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn core_invariant_ids_are_unique_and_nonempty() {
        let mut seen = HashSet::new();

        for invariant in CORE_INVARIANTS {
            assert!(!invariant.id.is_empty());
            assert!(invariant.id.starts_with("zuir."));
            assert!(seen.insert(invariant.id));
        }
    }

    #[test]
    fn every_core_invariant_has_an_owner_and_statement() {
        for invariant in CORE_INVARIANTS {
            assert!(!invariant.owner.module_path().is_empty());
            assert!(!invariant.statement.is_empty());
        }
    }

    #[test]
    fn core_lookup_is_stable() {
        let invariant = find_core_invariant(id::REFERENCE_INTEGRITY)
            .expect("reference integrity is a core invariant");

        assert_eq!(invariant.id, id::REFERENCE_INTEGRITY);
        assert_eq!(invariant.owner, VerificationOwner::Structural);
        assert!(is_core_invariant(id::REFERENCE_INTEGRITY));
    }

    #[test]
    fn unknown_invariants_are_not_assumed_to_be_core_or_verified() {
        assert!(find_core_invariant("future_domain.new_rule").is_none());
        assert!(!is_core_invariant("future_domain.new_rule"));
        assert_eq!(verification_owner("future_domain.new_rule"), None);
    }

    #[test]
    fn ssa_invariants_are_conditional() {
        let definition = find_core_invariant(id::SSA_SINGLE_DEFINITION)
            .expect("SSA definition invariant exists");

        let legality = find_core_invariant(id::SSA_USE_LEGALITY)
            .expect("SSA use invariant exists");

        assert_eq!(definition.class, InvariantClass::Conditional);
        assert_eq!(
            definition.applicability,
            Applicability::WhenSsaIsEnabled
        );

        assert_eq!(legality.class, InvariantClass::Conditional);
        assert_eq!(
            legality.applicability,
            Applicability::WhenSsaIsEnabled
        );
    }

    #[test]
    fn extension_ids_accept_namespaced_identifiers() {
        let identifier = ExtensionInvariantId::try_new("quantum.measurement_valid")
            .expect("valid extension invariant ID");

        assert_eq!(identifier.as_str(), "quantum.measurement_valid");
        assert_eq!(
            identifier.to_string(),
            "quantum.measurement_valid"
        );
    }

    #[test]
    fn extension_ids_reject_empty_or_unqualified_names() {
        assert_eq!(
            ExtensionInvariantId::try_new(""),
            Err(ExtensionInvariantIdError::Empty)
        );

        assert_eq!(
            ExtensionInvariantId::try_new("measurement_valid"),
            Err(ExtensionInvariantIdError::InvalidNamespace)
        );
    }

    #[test]
    fn extension_ids_cannot_claim_the_core_namespace() {
        assert_eq!(
            ExtensionInvariantId::try_new("zuir.reference_integrity"),
            Err(ExtensionInvariantIdError::CoreNamespaceReserved)
        );
    }

    #[test]
    fn extension_ids_reject_invalid_segments() {
        for invalid in [
            "Quantum.measurement_valid",
            "quantum.Measurement",
            "quantum.measurement-valid",
            "quantum..measurement",
            "quantum.measurement.extra",
            "1quantum.measurement",
        ] {
            assert!(
                ExtensionInvariantId::try_new(invalid).is_err(),
                "expected rejection for {invalid}"
            );
        }
    }

    #[test]
    fn core_catalogue_does_not_define_hardware_capacity_ceilings() {
        for invariant in CORE_INVARIANTS {
            assert!(
                !invariant.id.contains("max_qubits"),
                "universal qubit ceiling must not be an invariant"
            );
            assert!(
                !invariant.id.contains("max_devices"),
                "universal device ceiling must not be an invariant"
            );
            assert!(
                !invariant.id.contains("max_nodes"),
                "universal node ceiling must not be an invariant"
            );
        }
    }

    #[test]
    fn operational_exhaustion_is_not_semantic_invalidity() {
        let invariant = find_core_invariant(id::RESOURCE_EXHAUSTION)
            .expect("resource exhaustion invariant exists");

        assert_eq!(invariant.class, InvariantClass::Universal);
        assert!(invariant.statement.contains("distinctly"));
    }

    #[test]
    fn extension_ids_round_trip_without_normalization() {
        let input = String::from("tensor.symbolic_shape_valid");
        let identifier = ExtensionInvariantId::try_new(input.clone())
            .expect("valid extension invariant ID");

        assert_eq!(identifier.as_str(), input);
        assert_eq!(identifier.into_string(), input);
    }
}