#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Dialect Interfaces
//!
//! Canonical path: `src/zuir/dialect/interfaces.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: `unsafe` code is forbidden.
//!
//! ## PURPOSE
//!
//! Define versioned, extensible interface contracts shared by ZUIR dialects.
//! An interface describes a named contract that an operation, dialect, or
//! trusted compiler component may provide or require.
//!
//! This module separates:
//! - interface declarations and version requirements;
//! - trusted Rust implementations of interface contracts;
//! - dialect registration and dependency resolution;
//! - operation schemas and whole-program verification;
//! - target-specific lowering and physical feasibility.
//!
//! ## AUTHORITY
//!
//! This module must conform to:
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/lexer/tokens.g4`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/dialect/definition.rs`
//! - `src/zuir/dialect/registry.rs`
//! - `src/zuir/dialect/versioning.rs`
//!
//! ## OWNERSHIP
//!
//! OWNS:
//! - Interface declaration metadata.
//! - Version requirements for interfaces.
//! - Validation of interface declarations and requirements.
//! - The contract implemented by trusted Rust interface providers.
//!
//! DOES_NOT_OWN:
//! - Dialect manifests or registration state.
//! - Operation schemas or universal operation enums.
//! - The semantic meaning of individual operations.
//! - Whole-program verification.
//! - Executable extension loading, sandboxing, or authorization.
//! - Hardware discovery, physical allocation, or target lowering.
//! - Source syntax, keywords, parser rules, or AST construction.
//!
//! ## SCALABILITY
//!
//! Interface names and declarations are dynamically sized. There is no
//! architectural limit on the number of interfaces or computational domains.
//! Actual memory, execution time, and serialized-input limits remain subject
//! to explicit compiler and security budgets.
//!
//! ## EXTENSIBILITY
//!
//! Interface identity is a `QualifiedName`, not a closed Rust enum. New
//! domains and interfaces can be added without adding variants to this file.
//! Unknown interface names must remain inert until a consumer explicitly
//! resolves and validates them. A name alone does not grant executable
//! semantics.
//!
//! ## TRUST BOUNDARY
//!
//! `ImplementsInterface` is a Rust trait for trusted, statically linked
//! compiler components. It is not a plugin loader or sandbox boundary.
//! Implementations supplied by untrusted extensions must not be invoked
//! until a separate host policy has authorized and isolated them.
//!
//! ## INTEGRATION
//!
//! `definition.rs` owns dialect manifests and currently records interface
//! names as `QualifiedName` values. Its existing representation remains
//! authoritative for manifest compatibility.
//!
//! `registry.rs` owns registration, namespace uniqueness, and dependency
//! resolution.
//!
//! `versioning.rs` owns dialect-level compatibility decisions.
//!
//! `schema/types.rs` owns `QualifiedName`.
//!
//! `version.rs` owns `DialectVersion`.
//!
//! Operation schemas and verifiers may consume the contracts defined here.
//! They must not create duplicate interface identity or independently
//! reinterpret version requirements.
//!
//! ## FREEZE CRITERIA
//!
//! - Names and version requirements are validated.
//! - Deserialized declarations can be revalidated.
//! - Version checks are explicit and deterministic.
//! - No interface name implies executable behavior by itself.
//! - No fixed hardware or operation limits are introduced.
//! - Tests cover valid, invalid, and incompatible declarations.
//!
//! This module intentionally does not claim to implement interfaces for
//! quantum, classical, HDL, tensor, AI, or distributed operations. Those
//! implementations must be added at their authoritative owning modules.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::version::DialectVersion;

// =============================================================================
// Errors
// =============================================================================

/// Failures while declaring, validating, or checking an interface contract.
///
/// The error type is independent of diagnostics rendering, source parsing,
/// registry state, and target execution.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum InterfaceError {
    /// A qualified name or declaration contains invalid data.
    #[error("invalid interface declaration `{name}`: {reason}")]
    InvalidDeclaration {
        name: String,
        reason: String,
    },

    /// An interface cannot require itself.
    #[error("interface `{0}` cannot require itself")]
    SelfRequirement(String),

    /// An interface requirement was already declared.
    #[error("duplicate interface requirement `{0}`")]
    DuplicateRequirement(String),

    /// The provider implements a different interface from the required one.
    #[error(
        "interface mismatch: required `{required}`, provided `{provided}`"
    )]
    InterfaceMismatch {
        required: String,
        provided: String,
    },

    /// The provided interface version does not satisfy the requirement.
    #[error(
        "interface `{interface}` version {actual} does not satisfy {requirement}"
    )]
    VersionMismatch {
        interface: String,
        actual: DialectVersion,
        requirement: String,
    },

    /// The implementation failed its own contract validation.
    #[error("interface implementation `{implementation}` is invalid: {reason}")]
    InvalidImplementation {
        implementation: String,
        reason: String,
    },
}

// =============================================================================
// Version requirements
// =============================================================================

/// A version requirement for a named interface.
///
/// These rules govern declared interface versions only. They do not prove
/// that an implementation is available, trusted, or executable on a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "version", rename_all = "snake_case")]
pub enum InterfaceVersionRequirement {
    /// The provider must supply exactly the specified version.
    Exact(DialectVersion),

    /// The provider must have the same major version and a version greater
    /// than or equal to the specified version.
    SameMajorAtLeast(DialectVersion),

    /// The provider version is accepted by the caller's explicit policy.
    ///
    /// This variant does not mean that every version is automatically safe.
    AnyCompatible,
}

impl InterfaceVersionRequirement {
    /// Check whether a provided version satisfies this requirement.
    ///
    /// `AnyCompatible` deliberately returns `true` here because this method
    /// evaluates only the local requirement. Callers using that variant
    /// must additionally apply their explicit compatibility policy.
    #[must_use]
    pub fn matches_declared_version(self, actual: DialectVersion) -> bool {
        match self {
            Self::Exact(expected) => actual == expected,

            Self::SameMajorAtLeast(minimum) => {
                actual.major() == minimum.major() && actual >= minimum
            }

            Self::AnyCompatible => true,
        }
    }

    /// Whether a separate compatibility-policy decision is required.
    #[must_use]
    pub const fn requires_compatibility_policy(self) -> bool {
        matches!(self, Self::AnyCompatible)
    }

    /// Validate this requirement's representation.
    ///
    /// Dialect versions use numeric components, so all variants are
    /// structurally valid. Compatibility is evaluated when an actual
    /// provided version and policy are available.
    pub fn validate(self) -> Result<(), InterfaceError> {
        Ok(())
    }
}

impl fmt::Display for InterfaceVersionRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(version) => write!(f, "exactly {version}"),
            Self::SameMajorAtLeast(version) => {
                write!(f, "same major version as, and at least, {version}")
            }
            Self::AnyCompatible => f.write_str("any policy-approved compatible version"),
        }
    }
}

// =============================================================================
// Interface requirements
// =============================================================================

/// A named interface requirement.
///
/// The interface name identifies the contract. The version requirement
/// determines which declared interface versions can satisfy it.
///
/// This is a declarative value: it does not load or execute an implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterfaceRequirement {
    name: QualifiedName,
    version: InterfaceVersionRequirement,
}

impl InterfaceRequirement {
    /// Construct a validated interface requirement.
    pub fn new(
        name: QualifiedName,
        version: InterfaceVersionRequirement,
    ) -> Result<Self, InterfaceError> {
        name.validate().map_err(|error| {
            InterfaceError::InvalidDeclaration {
                name: name.canonical_string(),
                reason: error.to_string(),
            }
        })?;

        version.validate()?;

        Ok(Self { name, version })
    }

    /// Return the required interface's qualified name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the required interface version rule.
    #[must_use]
    pub const fn version_requirement(&self) -> InterfaceVersionRequirement {
        self.version
    }

    /// Validate the requirement, including after deserialization.
    pub fn validate(&self) -> Result<(), InterfaceError> {
        self.name.validate().map_err(|error| {
            InterfaceError::InvalidDeclaration {
                name: self.name.canonical_string(),
                reason: error.to_string(),
            }
        })?;

        self.version.validate()
    }

    /// Check the provided name and version against this requirement.
    pub fn check(
        &self,
        provided_name: &QualifiedName,
        provided_version: DialectVersion,
    ) -> Result<(), InterfaceError> {
        self.validate()?;
        provided_name.validate().map_err(|error| {
            InterfaceError::InvalidDeclaration {
                name: provided_name.canonical_string(),
                reason: error.to_string(),
            }
        })?;

        if &self.name != provided_name {
            return Err(InterfaceError::InterfaceMismatch {
                required: self.name.canonical_string(),
                provided: provided_name.canonical_string(),
            });
        }

        if !self.version.matches_declared_version(provided_version) {
            return Err(InterfaceError::VersionMismatch {
                interface: self.name.canonical_string(),
                actual: provided_version,
                requirement: self.version.to_string(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Interface definitions
// =============================================================================

/// Declarative definition of a named, versioned interface.
///
/// Definitions can describe relationships to other interfaces, but do not
/// contain executable callbacks. The `required_interfaces` map is ordered
/// to make iteration and diagnostic generation deterministic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterfaceDefinition {
    name: QualifiedName,
    version: DialectVersion,
    description: Option<String>,
    required_interfaces: BTreeMap<QualifiedName, InterfaceVersionRequirement>,
}

impl InterfaceDefinition {
    /// Construct an interface definition.
    pub fn new(
        name: QualifiedName,
        version: DialectVersion,
    ) -> Result<Self, InterfaceError> {
        name.validate().map_err(|error| {
            InterfaceError::InvalidDeclaration {
                name: name.canonical_string(),
                reason: error.to_string(),
            }
        })?;

        Ok(Self {
            name,
            version,
            description: None,
            required_interfaces: BTreeMap::new(),
        })
    }

    /// Attach optional human-readable documentation.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Return the interface's qualified name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the interface's declared version.
    #[must_use]
    pub const fn version(&self) -> DialectVersion {
        self.version
    }

    /// Return optional documentation.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Iterate over required interfaces in deterministic name order.
    pub fn required_interfaces(
        &self,
    ) -> impl Iterator<Item = (&QualifiedName, &InterfaceVersionRequirement)> {
        self.required_interfaces.iter()
    }

    /// Return the number of declared interface dependencies.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.required_interfaces.len()
    }

    /// Declare an interface required by this interface.
    ///
    /// Duplicate requirements are rejected rather than silently replaced.
    /// A dependency cycle spanning multiple definitions must be detected by
    /// the registry or a higher-level interface-graph validator.
    pub fn require_interface(
        &mut self,
        requirement: InterfaceRequirement,
    ) -> Result<(), InterfaceError> {
        requirement.validate()?;

        if requirement.name == self.name {
            return Err(InterfaceError::SelfRequirement(
                self.name.canonical_string(),
            ));
        }

        let name = requirement.name;
        if self.required_interfaces.contains_key(&name) {
            return Err(InterfaceError::DuplicateRequirement(
                name.canonical_string(),
            ));
        }

        self.required_interfaces.insert(name, requirement.version);
        Ok(())
    }

    /// Validate all local invariants, including after deserialization.
    ///
    /// This method does not resolve dependencies against a registry or prove
    /// that the required interfaces are installed.
    pub fn validate(&self) -> Result<(), InterfaceError> {
        self.name.validate().map_err(|error| {
            InterfaceError::InvalidDeclaration {
                name: self.name.canonical_string(),
                reason: error.to_string(),
            }
        })?;

        if let Some(description) = &self.description {
            if description.trim().is_empty() {
                return Err(InterfaceError::InvalidDeclaration {
                    name: self.name.canonical_string(),
                    reason: "description must be absent or non-empty".to_owned(),
                });
            }
        }

        for (name, version) in &self.required_interfaces {
            name.validate().map_err(|error| {
                InterfaceError::InvalidDeclaration {
                    name: name.canonical_string(),
                    reason: error.to_string(),
                }
            })?;

            version.validate()?;

            if name == &self.name {
                return Err(InterfaceError::SelfRequirement(
                    self.name.canonical_string(),
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Trusted implementation contract
// =============================================================================

/// Contract for a trusted Rust component that provides a named interface.
///
/// This trait is intended for statically linked compiler components, such as
/// implementations owned by standard dialects. It is object-safe so a future
/// trusted implementation registry can hold trait objects if needed.
///
/// Implementations must:
/// - return stable, validated names and versions;
/// - validate their own interface-specific invariants;
/// - avoid relying on target capabilities that have not been checked;
/// - preserve the interface's documented semantics.
///
/// This trait does not authorize loading third-party code.
pub trait ImplementsInterface: Send + Sync {
    /// Return the interface name implemented by this component.
    fn interface_name(&self) -> &QualifiedName;

    /// Return the interface version implemented by this component.
    fn interface_version(&self) -> DialectVersion;

    /// Return the stable qualified identity of this implementation.
    fn implementation_name(&self) -> &QualifiedName;

    /// Validate implementation-specific invariants.
    fn validate_interface_contract(&self) -> Result<(), InterfaceError>;
}

/// Validate a trusted implementation against a declared requirement.
///
/// This checks identity, version, and implementation-local invariants.
/// It does not establish trust for externally loaded code, prove that a
/// target supports the interface, or verify whole-program semantics.
pub fn validate_implementation<I: ImplementsInterface + ?Sized>(
    requirement: &InterfaceRequirement,
    implementation: &I,
) -> Result<(), InterfaceError> {
    requirement.check(
        implementation.interface_name(),
        implementation.interface_version(),
    )?;

    implementation
        .implementation_name()
        .validate()
        .map_err(|error| InterfaceError::InvalidImplementation {
            implementation: implementation.implementation_name().canonical_string(),
            reason: error.to_string(),
        })?;

    implementation
        .validate_interface_contract()
        .map_err(|error| InterfaceError::InvalidImplementation {
            implementation: implementation.implementation_name().canonical_string(),
            reason: error.to_string(),
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn qname(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test name must be valid")
    }

    fn version(major: u32, minor: u32, patch: u32) -> DialectVersion {
        DialectVersion::new(major, minor, patch)
    }

    #[test]
    fn constructs_and_validates_interface_definition() {
        let definition =
            InterfaceDefinition::new(qname("core::Pure"), version(1, 0, 0))
                .expect("definition should be valid")
                .with_description("Declares an operation with no observable effects.");

        assert_eq!(definition.name().canonical_string(), "core::Pure");
        assert_eq!(definition.version(), version(1, 0, 0));
        assert!(definition.validate().is_ok());
    }

    #[test]
    fn accepts_a_valid_interface_requirement() {
        let requirement = InterfaceRequirement::new(
            qname("core::Lowerable"),
            InterfaceVersionRequirement::SameMajorAtLeast(version(2, 1, 0)),
        )
        .expect("requirement should be valid");

        assert!(requirement
            .check(&qname("core::Lowerable"), version(2, 4, 0))
            .is_ok());
    }

    #[test]
    fn rejects_a_different_interface_name() {
        let requirement = InterfaceRequirement::new(
            qname("core::Pure"),
            InterfaceVersionRequirement::Exact(version(1, 0, 0)),
        )
        .expect("requirement should be valid");

        let result = requirement.check(&qname("quantum::Pure"), version(1, 0, 0));

        assert!(matches!(
            result,
            Err(InterfaceError::InterfaceMismatch { .. })
        ));
    }

    #[test]
    fn rejects_an_incompatible_version() {
        let requirement = InterfaceRequirement::new(
            qname("core::Lowerable"),
            InterfaceVersionRequirement::SameMajorAtLeast(version(2, 1, 0)),
        )
        .expect("requirement should be valid");

        let result = requirement.check(&qname("core::Lowerable"), version(3, 0, 0));

        assert!(matches!(
            result,
            Err(InterfaceError::VersionMismatch { .. })
        ));
    }

    #[test]
    fn allows_an_extensible_qualified_interface_name() {
        let definition = InterfaceDefinition::new(
            qname("vendor_example::tensor::Differentiable"),
            version(1, 0, 0),
        )
        .expect("extension-defined interface should be valid");

        assert!(definition.validate().is_ok());
    }

    #[test]
    fn rejects_self_requirements() {
        let mut definition =
            InterfaceDefinition::new(qname("core::Pure"), version(1, 0, 0))
                .expect("definition should be valid");

        let requirement = InterfaceRequirement::new(
            qname("core::Pure"),
            InterfaceVersionRequirement::Exact(version(1, 0, 0)),
        )
        .expect("requirement should be valid");

        assert!(matches!(
            definition.require_interface(requirement),
            Err(InterfaceError::SelfRequirement(_))
        ));
    }

    #[test]
    fn rejects_duplicate_requirements() {
        let mut definition =
            InterfaceDefinition::new(qname("quantum::Measurable"), version(1, 0, 0))
                .expect("definition should be valid");

        let first = InterfaceRequirement::new(
            qname("core::Lowerable"),
            InterfaceVersionRequirement::Exact(version(1, 0, 0)),
        )
        .expect("requirement should be valid");

        let second = InterfaceRequirement::new(
            qname("core::Lowerable"),
            InterfaceVersionRequirement::SameMajorAtLeast(version(1, 0, 0)),
        )
        .expect("requirement should be valid");

        assert!(definition.require_interface(first).is_ok());
        assert!(matches!(
            definition.require_interface(second),
            Err(InterfaceError::DuplicateRequirement(_))
        ));
    }

    #[test]
    fn any_compatible_requires_explicit_policy() {
        let requirement = InterfaceVersionRequirement::AnyCompatible;

        assert!(requirement.requires_compatibility_policy());
        assert!(requirement.matches_declared_version(version(9, 8, 7)));
    }
}