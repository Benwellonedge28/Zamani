//! Capability kind for the Zamani source AST.
//!
//! # Responsibility
//!
//! This module classifies the *role* of a source-level capability without
//! defining the concrete capability itself.
//!
//! A capability has two conceptually separate parts:
//!
//! 1. `CapabilityKind`
//!    - describes what role the capability plays;
//!    - remains domain-neutral;
//!    - provides stable structural classification;
//!    - must not contain backend/vendor/hardware identities.
//!
//! 2. Capability identity
//!    - is owned by `capability.rs`;
//!    - identifies a capability through its namespace/name;
//!    - may represent present or future computational technologies.
//!
//! This separation is essential for POCO-REAF:
//!
//!     Program Once
//!     Compile Once
//!     Run Everywhere
//!     Anywhere
//!     Forever
//!
//! The AST describes programmer intent. Capability resolution, resource
//! discovery, backend selection, mapping, routing, scheduling, calibration,
//! error correction, and execution happen downstream.
//!
//! # Architectural boundary
//!
//! This module MUST NOT depend on:
//!
//! - quantum hardware;
//! - QPU vendors;
//! - backend APIs;
//! - physical qubit identifiers;
//! - topology;
//! - gate sets;
//! - calibration;
//! - routing;
//! - scheduling;
//! - QEC implementations;
//! - noise models;
//! - execution jobs;
//! - credentials;
//! - runtime state;
//! - ZUIR implementation details;
//! - QIR/LLVM/MLIR types.
//!
//! # Scalability
//!
//! There are no resource-count limits here. A capability kind contains no
//! machine-sized collection and no fixed resource cardinality.
//!
//! The enum contains only the small set of *language-level classification
//! categories*. Concrete capabilities remain open-ended through the
//! namespaced capability identity owned by `capability.rs`.
//!
//! Rust: 1.97 / 1.97.1
//! `unsafe`: forbidden.

use core::fmt;
use core::str::FromStr;

/// Structural classification of a source-level capability.
///
/// `CapabilityKind` deliberately describes a capability's role rather than
/// enumerating individual technologies.
///
/// For example, this type may classify a capability as representing a
/// requirement, provision, constraint, permission, effect, or extension
/// mechanism. It must never become an enumeration of vendors or hardware.
///
/// Concrete identities such as a future quantum, classical, photonic,
/// neuromorphic, distributed, accelerator, or other capability remain
/// namespaced data handled by the capability identity layer.
///
/// # Extensibility
///
/// `Extension` is intentionally present as an escape hatch for capability
/// categories introduced by language extensions. It prevents the native AST
/// from requiring a new core enum variant merely because a future extension
/// introduces a new classification.
///
/// The extension's stable identity is carried separately by the capability
/// representation/metadata rather than encoded into this enum.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum CapabilityKind {
    /// The program requires a capability from the execution environment.
    ///
    /// This is the normal form for source-level requirements such as:
    ///
    /// `requires capability("some.namespace", "some-capability")`
    ///
    /// The actual capability identity is not stored here.
    Requirement,

    /// The program or compilation unit provides a capability.
    ///
    /// This describes an advertised source-level capability. It does not
    /// imply that the capability is physically realizable on a particular
    /// machine.
    Provision,

    /// The program places a constraint on an otherwise selectable
    /// capability.
    ///
    /// The constraint itself belongs to the capability/constraint model;
    /// this kind only identifies its structural role.
    Constraint,

    /// The capability grants permission to perform a source-level action.
    ///
    /// Authorization and security policy are resolved outside the AST.
    Permission,

    /// The capability describes an effect or execution property required by
    /// the source program.
    ///
    /// This does not replace the AST effect system. It allows a capability
    /// declaration to be classified without coupling it to a particular
    /// effect implementation.
    Effect,

    /// A capability introduced by an extension namespace.
    ///
    /// Extension-specific meaning must remain outside this core enum.
    Extension,
}

impl CapabilityKind {
    /// Returns the stable source/schema name of this kind.
    ///
    /// These names are intended for diagnostics, deterministic textual
    /// serialization, schema interchange, and tooling.
    ///
    /// The returned strings are part of the AST schema contract and should
    /// therefore not be casually renamed.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requirement => "requirement",
            Self::Provision => "provision",
            Self::Constraint => "constraint",
            Self::Permission => "permission",
            Self::Effect => "effect",
            Self::Extension => "extension",
        }
    }

    /// Returns `true` when this kind represents a capability requirement.
    #[must_use]
    pub const fn is_requirement(self) -> bool {
        matches!(self, Self::Requirement)
    }

    /// Returns `true` when this kind represents a capability provision.
    #[must_use]
    pub const fn is_provision(self) -> bool {
        matches!(self, Self::Provision)
    }

    /// Returns `true` when this kind represents a capability constraint.
    #[must_use]
    pub const fn is_constraint(self) -> bool {
        matches!(self, Self::Constraint)
    }

    /// Returns `true` when this kind represents a permission.
    #[must_use]
    pub const fn is_permission(self) -> bool {
        matches!(self, Self::Permission)
    }

    /// Returns `true` when this kind represents an effect classification.
    #[must_use]
    pub const fn is_effect(self) -> bool {
        matches!(self, Self::Effect)
    }

    /// Returns `true` when this kind is extension-defined.
    #[must_use]
    pub const fn is_extension(self) -> bool {
        matches!(self, Self::Extension)
    }

    /// Returns whether the kind represents a declarative capability
    /// relationship.
    ///
    /// This is deliberately about source structure, not backend semantics.
    #[must_use]
    pub const fn is_declarative(self) -> bool {
        matches!(
            self,
            Self::Requirement | Self::Provision | Self::Constraint
        )
    }

    /// Returns whether the kind can affect authorization policy.
    ///
    /// This does not perform authorization.
    #[must_use]
    pub const fn participates_in_permission_model(self) -> bool {
        matches!(self, Self::Permission)
    }

    /// Parses a stable kind name.
    ///
    /// Parsing is intentionally strict for the core schema. Unknown names
    /// must not silently become a known core kind.
    ///
    /// Extension-specific names should be handled by the extension registry
    /// rather than by adding arbitrary strings to this parser.
    pub fn parse(value: &str) -> Result<Self, CapabilityKindParseError> {
        match value {
            "requirement" => Ok(Self::Requirement),
            "provision" => Ok(Self::Provision),
            "constraint" => Ok(Self::Constraint),
            "permission" => Ok(Self::Permission),
            "effect" => Ok(Self::Effect),
            "extension" => Ok(Self::Extension),
            _ => Err(CapabilityKindParseError::UnknownKind {
                value: value.to_owned(),
            }),
        }
    }

    /// Returns every core capability kind.
    ///
    /// The returned collection is deliberately tiny and fixed because it
    /// describes the language's structural categories, not computational
    /// resources.
    ///
    /// Concrete capability identities remain unbounded.
    #[must_use]
    pub const fn core_kinds() -> &'static [Self] {
        &[
            Self::Requirement,
            Self::Provision,
            Self::Constraint,
            Self::Permission,
            Self::Effect,
            Self::Extension,
        ]
    }
}

impl Default for CapabilityKind {
    /// The default is a capability requirement because a capability appearing
    /// in source most commonly expresses an environmental requirement.
    ///
    /// Callers should prefer explicit construction when the source grammar
    /// provides an explicit kind.
    fn default() -> Self {
        Self::Requirement
    }
}

impl fmt::Display for CapabilityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CapabilityKind {
    type Err = CapabilityKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

/// Error returned when parsing a capability kind fails.
///
/// The error preserves the original value so diagnostics can report exactly
/// what appeared in the source or serialized representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityKindParseError {
    /// The supplied kind name is not a core capability kind.
    UnknownKind {
        /// The unrecognized value.
        value: String,
    },
}

impl fmt::Display for CapabilityKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownKind { value } => {
                write!(formatter, "unknown capability kind `{value}`")
            }
        }
    }
}

impl std::error::Error for CapabilityKindParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_names_are_round_trip_parseable() {
        for kind in CapabilityKind::core_kinds() {
            let encoded = kind.as_str();
            let decoded = CapabilityKind::parse(encoded)
                .expect("every core capability kind must parse");

            assert_eq!(*kind, decoded);
        }
    }

    #[test]
    fn display_uses_stable_schema_name() {
        assert_eq!(
            CapabilityKind::Requirement.to_string(),
            "requirement"
        );
        assert_eq!(
            CapabilityKind::Provision.to_string(),
            "provision"
        );
        assert_eq!(
            CapabilityKind::Constraint.to_string(),
            "constraint"
        );
        assert_eq!(
            CapabilityKind::Permission.to_string(),
            "permission"
        );
        assert_eq!(
            CapabilityKind::Effect.to_string(),
            "effect"
        );
        assert_eq!(
            CapabilityKind::Extension.to_string(),
            "extension"
        );
    }

    #[test]
    fn from_str_matches_parse() {
        assert_eq!(
            "requirement"
                .parse::<CapabilityKind>()
                .expect("valid capability kind"),
            CapabilityKind::Requirement
        );

        assert_eq!(
            "extension"
                .parse::<CapabilityKind>()
                .expect("valid capability kind"),
            CapabilityKind::Extension
        );
    }

    #[test]
    fn unknown_kind_is_rejected() {
        let error = CapabilityKind::parse("future-vendor-specific-kind")
            .expect_err("unknown kinds must not become core kinds");

        assert_eq!(
            error,
            CapabilityKindParseError::UnknownKind {
                value: "future-vendor-specific-kind".to_owned(),
            }
        );
    }

    #[test]
    fn classification_helpers_are_consistent() {
        assert!(CapabilityKind::Requirement.is_requirement());
        assert!(CapabilityKind::Provision.is_provision());
        assert!(CapabilityKind::Constraint.is_constraint());
        assert!(CapabilityKind::Permission.is_permission());
        assert!(CapabilityKind::Effect.is_effect());
        assert!(CapabilityKind::Extension.is_extension());

        assert!(CapabilityKind::Requirement.is_declarative());
        assert!(CapabilityKind::Provision.is_declarative());
        assert!(CapabilityKind::Constraint.is_declarative());

        assert!(!CapabilityKind::Permission.is_declarative());
        assert!(!CapabilityKind::Effect.is_declarative());
        assert!(!CapabilityKind::Extension.is_declarative());
    }

    #[test]
    fn kind_is_copyable_and_orderable() {
        let mut kinds = CapabilityKind::core_kinds().to_vec();
        kinds.sort();

        assert_eq!(kinds.len(), CapabilityKind::core_kinds().len());
    }

    #[test]
    fn default_is_requirement() {
        assert_eq!(
            CapabilityKind::default(),
            CapabilityKind::Requirement
        );
    }

    #[test]
    fn no_vendor_or_hardware_identity_is_encoded() {
        // This test documents an architectural invariant:
        // capability kinds classify relationships, not technologies.
        //
        // A future capability such as:
        //
        //   quantum.superconducting.some-provider
        //
        // must be represented by the capability identity, not by adding a
        // CapabilityKind variant.
        assert_eq!(
            CapabilityKind::Requirement.as_str(),
            "requirement"
        );
    }
}