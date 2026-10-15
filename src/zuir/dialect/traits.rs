#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Dialect Traits
//!
//! Path: `src/zuir/dialect/traits.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97 or later.
//! Safety: `unsafe` code is forbidden.
//!
//! ## PURPOSE
//!
//! Define reusable contracts for validating, identifying, inspecting, and
//! consuming ZUIR dialect declarations without duplicating the concrete
//! declaration types owned by `dialect/definition.rs`.
//!
//! These traits establish stable interfaces for the dialect registry,
//! compatibility checks, diagnostics, tooling, and future operation-schema
//! integrations.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/dialect/definition.rs`
//! - `src/zuir/dialect/registry.rs`
//! - `src/zuir/dialect/versioning.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compatibility.md`
//!
//! ## OWNS
//!
//! - Reusable validation and canonical-identity contracts.
//! - Read-only interfaces over dialect declarations.
//! - Interfaces for inspecting exported declarations, dependencies,
//!   implementation requirements, and compatibility declarations.
//!
//! ## DOES_NOT_OWN
//!
//! - Concrete dialect manifest storage.
//! - Global registration, dependency-graph resolution, or registry policy.
//! - Version comparison or migration execution.
//! - Operation-schema definitions or operation execution.
//! - Source syntax, lexical tokens, parser rules, or AST construction.
//! - Semantic analysis, whole-program IR verification, or lowering.
//! - Authorization, sandboxing, executable extension loading, or trust proof.
//! - Hardware discovery, physical allocation, scheduling, or runtime behavior.
//!
//! ## DEPENDENCY CONTRACT
//!
//! This module depends only on canonical dialect declarations and schema-level
//! names. It must not depend on the registry, builders, verifier, runtime,
//! backend, or source grammar implementation.
//!
//! `definition.rs` remains the authoritative owner of concrete manifest
//! fields and local validation rules.
//!
//! `registry.rs` remains the authoritative owner of registration state,
//! namespace conflicts, dependency resolution, and registry policy.
//!
//! `versioning.rs` remains the authoritative owner of compatibility
//! decisions and migration requirements.
//!
//! ## SCALABILITY
//!
//! These traits impose no universal maximum on dialects, exports, interfaces,
//! operations, types, qubits, devices, nodes, tensor dimensions, or hardware
//! resources. Actual collections and computational work remain bounded by
//! representable values, implementation capabilities, and available resources.
//!
//! ## SECURITY
//!
//! Implementing a trait does not establish trust or authorization.
//! A declarative manifest must never be treated as executable code.
//! Any executable extension requires the separate authorization, verification,
//! and isolation controls defined by the relevant subsystem.
//!
//! ## FREEZE CRITERIA
//!
//! Freeze only after:
//! - The dialect module publicly declares `traits`.
//! - All referenced concrete types exist with the APIs used below.
//! - Unit and integration tests pass against the repository's actual module
//!   graph and supported compiler toolchain.
//! - The public API and compatibility implications are reviewed.
//! - Registry and versioning consumers use these contracts where appropriate.

use crate::zuir::dialect::definition::{
    DialectCompatibility,
    DialectDefinition,
    DialectDefinitionError,
    DialectDependency,
    DialectExport,
    DialectExportName,
    DialectImplementation,
    DialectName,
    DialectTrust,
    ExportKind,
};
use crate::zuir::schema::types::{
    QualifiedName,
    SchemaTypeError,
};
use crate::zuir::version::DialectVersion;

// =============================================================================
// Validation contract
// =============================================================================

/// A value that can validate its own local invariants.
///
/// This trait deliberately does not imply that a value is registered,
/// authorized, executable, compatible with the current ZUIR schema, or
/// suitable for a particular target.
///
/// Implementations should perform deterministic, side-effect-free validation.
///
/// The associated error type allows each canonical owner to retain its
/// structured error representation without this module introducing a second
/// global error hierarchy.
pub trait ValidateContract {
    /// The structured error produced by local validation.
    type Error;

    /// Validate local invariants.
    ///
    /// This method must not mutate the value or perform external I/O.
    fn validate_contract(&self) -> Result<(), Self::Error>;
}

impl ValidateContract for QualifiedName {
    type Error = SchemaTypeError;

    fn validate_contract(&self) -> Result<(), Self::Error> {
        self.validate()
    }
}

impl ValidateContract for DialectName {
    type Error = DialectDefinitionError;

    fn validate_contract(&self) -> Result<(), Self::Error> {
        self.validate()
    }
}

impl ValidateContract for DialectExportName {
    type Error = DialectDefinitionError;

    fn validate_contract(&self) -> Result<(), Self::Error> {
        self.validate()
    }
}

impl ValidateContract for DialectExport {
    type Error = DialectDefinitionError;

    fn validate_contract(&self) -> Result<(), Self::Error> {
        self.validate()
    }
}

impl ValidateContract for DialectDependency {
    type Error = DialectDefinitionError;

    fn validate_contract(&self) -> Result<(), Self::Error> {
        self.validate()
    }
}

impl ValidateContract for DialectCompatibility {
    type Error = DialectDefinitionError;

    fn validate_contract(&self) -> Result<(), Self::Error> {
        self.validate()
    }
}

impl ValidateContract for DialectImplementation {
    type Error = DialectDefinitionError;

    fn validate_contract(&self) -> Result<(), Self::Error> {
        self.validate()
    }
}

impl ValidateContract for DialectDefinition {
    type Error = DialectDefinitionError;

    fn validate_contract(&self) -> Result<(), Self::Error> {
        self.validate()
    }
}

// =============================================================================
// Canonical identity contract
// =============================================================================

/// A value that can expose a canonical, human-readable identity.
///
/// Canonical identity is useful for diagnostics, logs, deterministic ordering,
/// and external references. It is not a cryptographic fingerprint.
///
/// The returned string is owned because the existing qualified-name APIs
/// provide canonical formatting as a `String`.
///
/// Consumers must not use display text alone as proof of registration,
/// authorization, or semantic equivalence.
pub trait CanonicalIdentity {
    /// Return the canonical textual identity.
    fn canonical_identity(&self) -> String;
}

impl CanonicalIdentity for QualifiedName {
    fn canonical_identity(&self) -> String {
        self.canonical_string()
    }
}

impl CanonicalIdentity for DialectName {
    fn canonical_identity(&self) -> String {
        self.canonical_string()
    }
}

impl CanonicalIdentity for DialectExportName {
    fn canonical_identity(&self) -> String {
        self.canonical_string()
    }
}

// =============================================================================
// Dialect version contract
// =============================================================================

/// Exposes the declared version of a dialect-bearing declaration.
///
/// This is the version declared by the manifest. It is not a compatibility
/// decision and must not be interpreted as proof that a consumer supports
/// the declaration.
pub trait HasDialectVersion {
    /// Return the declared dialect version.
    fn dialect_version(&self) -> DialectVersion;
}

impl HasDialectVersion for DialectDefinition {
    fn dialect_version(&self) -> DialectVersion {
        self.version()
    }
}

// =============================================================================
// Export declaration contract
// =============================================================================

/// Read-only access to a declared dialect export.
///
/// Export declarations identify the kind of declaration being exposed.
/// They do not contain or authorize arbitrary executable behavior.
pub trait ExportDeclaration {
    /// Return the export's canonical name.
    fn export_name(&self) -> &DialectExportName;

    /// Return the declaration category.
    fn export_kind(&self) -> ExportKind;

    /// Return the optional schema revision label.
    fn export_schema_revision(&self) -> Option<&str>;

    /// Return optional descriptive documentation.
    fn export_description(&self) -> Option<&str>;
}

impl ExportDeclaration for DialectExport {
    fn export_name(&self) -> &DialectExportName {
        self.name()
    }

    fn export_kind(&self) -> ExportKind {
        self.kind()
    }

    fn export_schema_revision(&self) -> Option<&str> {
        self.schema_revision()
    }

    fn export_description(&self) -> Option<&str> {
        self.description()
    }
}

// =============================================================================
// Dependency declaration contract
// =============================================================================

/// Read-only access to a declared dialect dependency.
///
/// The registry remains responsible for resolving the dependency against
/// installed manifests, evaluating version requirements, and detecting
/// dependency cycles.
pub trait DependencyDeclaration {
    /// Return the dialect that is required.
    fn dependency_dialect(&self) -> &DialectName;

    /// Return the declared version requirement.
    fn dependency_version_requirement(
        &self,
    ) -> &crate::zuir::dialect::definition::VersionRequirement;

    /// Whether the dependency may be absent.
    fn dependency_is_optional(&self) -> bool;
}

impl DependencyDeclaration for DialectDependency {
    fn dependency_dialect(&self) -> &DialectName {
        self.dialect()
    }

    fn dependency_version_requirement(
        &self,
    ) -> &crate::zuir::dialect::definition::VersionRequirement {
        self.version_requirement()
    }

    fn dependency_is_optional(&self) -> bool {
        self.is_optional()
    }
}

// =============================================================================
// Dialect implementation requirements
// =============================================================================

/// Read-only access to the trust requirement declared by an implementation.
///
/// This describes the manifest's declared expectation. It does not indicate
/// that the host has approved, verified, sandboxed, or loaded the implementation.
pub trait HasTrustRequirement {
    /// Return the declared trust requirement.
    fn trust_requirement(&self) -> DialectTrust;
}

impl HasTrustRequirement for DialectImplementation {
    fn trust_requirement(&self) -> DialectTrust {
        self.trust()
    }
}

/// Query the declared lowering-interface requirements of an implementation.
///
/// An interface requirement identifies a contract that a compatible lowering
/// component must provide. It does not identify a specific device or vendor.
pub trait LoweringRequirements {
    /// Whether the implementation declares this required lowering interface.
    fn requires_lowering_interface(
        &self,
        interface: &QualifiedName,
    ) -> bool;

    /// Whether the implementation declares this required capability.
    fn requires_capability(
        &self,
        capability: &QualifiedName,
    ) -> bool;
}

impl LoweringRequirements for DialectImplementation {
    fn requires_lowering_interface(
        &self,
        interface: &QualifiedName,
    ) -> bool {
        self.lowering_interfaces()
            .any(|declared| declared == interface)
    }

    fn requires_capability(
        &self,
        capability: &QualifiedName,
    ) -> bool {
        self.required_capabilities()
            .any(|declared| declared == capability)
    }
}

// =============================================================================
// Dialect manifest contract
// =============================================================================

/// Common read-only interface to a validated dialect manifest.
///
/// This interface allows consumers to depend on a declared contract instead
/// of directly depending on the manifest's internal collection layout.
///
/// Implementations must preserve the invariants of their canonical declaration
/// owner. Callers must validate deserialized declarations before relying on
/// this interface.
///
/// This trait intentionally does not expose mutable manifest internals.
pub trait DialectManifest:
    ValidateContract<Error = DialectDefinitionError>
    + HasDialectVersion
{
    /// Return the canonical dialect name.
    fn dialect_name(&self) -> &DialectName;

    /// Look up an export by its qualified name.
    fn declared_export(
        &self,
        name: &DialectExportName,
    ) -> Option<&DialectExport>;

    /// Look up a dependency by dialect identity.
    fn declared_dependency(
        &self,
        name: &DialectName,
    ) -> Option<&DialectDependency>;

    /// Return the dialect's compatibility declaration.
    fn compatibility_declaration(&self) -> &DialectCompatibility;

    /// Return the declared implementation requirements.
    fn implementation_declaration(&self) -> &DialectImplementation;

    /// Whether an export with this name and kind is declared.
    fn declares_export(
        &self,
        name: &DialectExportName,
        kind: ExportKind,
    ) -> bool;

    /// Whether this dialect declares the given interface.
    fn declares_interface(&self, interface: &QualifiedName) -> bool;
}

impl DialectManifest for DialectDefinition {
    fn dialect_name(&self) -> &DialectName {
        self.name()
    }

    fn declared_export(
        &self,
        name: &DialectExportName,
    ) -> Option<&DialectExport> {
        self.export(name)
    }

    fn declared_dependency(
        &self,
        name: &DialectName,
    ) -> Option<&DialectDependency> {
        self.dependency(name)
    }

    fn compatibility_declaration(&self) -> &DialectCompatibility {
        self.compatibility()
    }

    fn implementation_declaration(&self) -> &DialectImplementation {
        self.implementation()
    }

    fn declares_export(
        &self,
        name: &DialectExportName,
        kind: ExportKind,
    ) -> bool {
        self.has_export(name, kind)
    }

    fn declares_interface(&self, interface: &QualifiedName) -> bool {
        self.interfaces().any(|declared| declared == interface)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::dialect::definition::{
        DialectCompatibility,
        DialectImplementation,
        DialectTrust,
        VersionRequirement,
    };
    use crate::zuir::version::INITIAL_DIALECT_VERSION;

    fn test_definition() -> DialectDefinition {
        let name = DialectName::parse("test::portable").expect("valid dialect name");

        let compatibility = DialectCompatibility::new(
            VersionRequirement::registry_policy(),
            VersionRequirement::registry_policy(),
            false,
        )
        .expect("valid compatibility declaration");

        let implementation =
            DialectImplementation::new(DialectTrust::DeclarativeOnly);

        DialectDefinition::new(
            name,
            INITIAL_DIALECT_VERSION,
            compatibility,
            implementation,
        )
        .expect("valid dialect definition")
    }

    #[test]
    fn canonical_identity_is_stable_for_validated_names() {
        let name = DialectName::parse("quantum::operations")
            .expect("valid dialect name");

        assert_eq!(
            name.canonical_identity(),
            name.canonical_string()
        );
    }

    #[test]
    fn manifest_trait_exposes_canonical_declarations() {
        let definition = test_definition();

        assert_eq!(
            definition.dialect_name().canonical_identity(),
            "test::portable"
        );

        assert_eq!(
            definition.dialect_version(),
            INITIAL_DIALECT_VERSION
        );

        assert!(definition.validate_contract().is_ok());
    }

    #[test]
    fn export_trait_preserves_declaration_kind() {
        let export_name = DialectExportName::parse("compute::transform")
            .expect("valid export name");

        let export = DialectExport::new(export_name.clone(), ExportKind::Operation)
            .expect("valid export");

        assert_eq!(export.export_name(), &export_name);
        assert_eq!(export.export_kind(), ExportKind::Operation);
        assert_eq!(export.export_schema_revision(), None);
        assert_eq!(export.export_description(), None);
        assert!(export.validate_contract().is_ok());
    }

    #[test]
    fn implementation_trait_does_not_grant_trust() {
        let implementation =
            DialectImplementation::new(DialectTrust::DeclarativeOnly);

        assert_eq!(
            implementation.trust_requirement(),
            DialectTrust::DeclarativeOnly
        );
    }

    #[test]
    fn undeclared_capabilities_are_not_assumed() {
        let implementation =
            DialectImplementation::new(DialectTrust::DeclarativeOnly);

        let capability =
            QualifiedName::parse("quantum::measurement")
                .expect("valid capability name");

        assert!(!implementation.requires_capability(&capability));
    }

    #[test]
    fn undeclared_lowering_interfaces_are_not_assumed() {
        let implementation =
            DialectImplementation::new(DialectTrust::DeclarativeOnly);

        let interface =
            QualifiedName::parse("lowering::portable")
                .expect("valid interface name");

        assert!(!implementation.requires_lowering_interface(&interface));
    }

    #[test]
    fn manifest_does_not_claim_undeclared_exports() {
        let definition = test_definition();

        let export =
            DialectExportName::parse("future::operation")
                .expect("valid export name");

        assert!(definition.declared_export(&export).is_none());
        assert!(!definition.declares_export(&export, ExportKind::Operation));
    }

    #[test]
    fn manifest_does_not_claim_undeclared_interfaces() {
        let definition = test_definition();

        let interface =
            QualifiedName::parse("future::interface")
                .expect("valid interface name");

        assert!(!definition.declares_interface(&interface));
    }
}