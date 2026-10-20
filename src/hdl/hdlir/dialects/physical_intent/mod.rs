
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Physical Intent Dialect
//!
//! Path: `src/hdl/hdlir/dialects/physical_intent/mod.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the identity and policy boundary for explicitly target-aware
//! hardware implementation intent. This dialect distinguishes portable
//! semantic requirements from implementation preferences, hints, policies,
//! constraints, and deliberate target-specific commitments.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/constraints.g4` when present in the authoritative grammar
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/config.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! ## Ownership
//!
//! This module owns the dialect identity and the classification and
//! validation policy defined here. The shared schema registry owns
//! registration. The canonical HDLIR graph owns operation instances.
//! The frontend and semantic model own source meaning. Target discovery,
//! capability evidence, physical planning, and realization remain owned by
//! their respective repository subsystems.
//!
//! ## Non-goals
//!
//! This module does not own source parsing, a second semantic model, a
//! competing constraint-expression language, physical resource allocation,
//! pin assignment, placement, routing, timing closure, synthesis, or
//! vendor-specific runtime state.
//!
//! ## Scalability
//!
//! There are no fixed hardware counts, widths, device counts, or resource
//! ceilings in this module. It does not evaluate symbolic resource quantities
//! or convert them to host-sized integers. Actual evaluation belongs to the
//! responsible semantic or realization subsystem and must use checked
//! arithmetic and explicit operational budgets.
//!
//! ## Integration
//!
//! - `dialects/mod.rs` declares `pub mod physical_intent;`.
//! - The HDLIR composition root declares the implemented dialect tree.
//! - `schema/registry.rs` registers the versioned dialect identity.
//! - Generic operation definitions and attributes remain owned by the schema
//!   and IR subsystems.
//! - Whole-design verification must validate references, expression types,
//!   units, scope, conflicts, and applicable target-capability evidence.
//! - Lowering must preserve intent classifications and provenance.
//! - Realization must distinguish hard requirements from preferences and
//!   hints; it must never silently ignore a failed requirement.
//!
//! Registration does not imply that physical-intent operations have been
//! implemented, that a target exists, or that any constraint is satisfied.

use crate::hdl::hdlir::config::ExtensionPolicy;
use crate::hdl::hdlir::errors::HdlirResult;
use crate::hdl::hdlir::schema::registry::{
    DefinitionStability, DialectDefinition, QualifiedName, RegistryBatch,
    SchemaRegistry,
};
use crate::hdl::hdlir::version::HdlirVersion;

/// Canonical qualified name of the physical-intent dialect.
///
/// This is a schema namespace, not a physical target identifier.
pub const PHYSICAL_INTENT_DIALECT_NAME: &str = "zamani.physical_intent";

/// Version of the physical-intent dialect contract.
///
/// This is independent of the language version, crate version, and HDLIR
/// serialization format version.
pub const PHYSICAL_INTENT_DIALECT_VERSION: HdlirVersion =
    HdlirVersion::new(1, 0, 0);

/// Classification of a hardware intent declaration.
///
/// Classification controls how later verification and realization stages
/// must interpret a declaration. It does not itself store or evaluate the
/// declaration's expression.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhysicalIntentKind {
    /// A semantic condition that must hold for a valid implementation.
    ///
    /// Failure must be reported. A realization must not silently weaken
    /// the requirement.
    Requirement,

    /// A condition that a candidate realization must satisfy.
    ///
    /// Constraint satisfaction must be established by the owning verifier
    /// or realization stage, not inferred from successful IR construction.
    Constraint,

    /// A desirable property that may guide implementation choices.
    ///
    /// A preference may be unmet only when the governing policy allows it;
    /// unmet preferences must not be reinterpreted as satisfied requirements.
    Preference,

    /// Optional implementation guidance.
    ///
    /// A hint is not a correctness guarantee and must not change the
    /// program's defined behavior.
    Hint,

    /// A policy restricting permissible realization choices.
    ///
    /// The policy's authority, scope, and conflict-resolution behavior must
    /// be specified by the owning policy subsystem.
    Policy,

    /// An explicit commitment to target-specific implementation identity
    /// or behavior.
    ///
    /// Such a commitment may reduce portability. It must remain explicit
    /// and must not be silently generalized to unrelated targets.
    TargetCommitment,
}

/// Scope of a physical-intent declaration.
///
/// This enum classifies scope; it does not replace references to actual
/// design, module, operation, or target objects.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PhysicalIntentScope {
    /// Applies to the design as a whole.
    Design,

    /// Applies to a logical module or module instance.
    Module,

    /// Applies to a logical operation or hardware construct.
    Operation,

    /// Applies to an explicitly identified implementation target.
    Target,
}

/// Policy for handling target-specific commitments during portable lowering.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum TargetBindingPolicy {
    /// Reject a target-specific commitment when no compatible target context
    /// has been established.
    #[default]
    RequireCompatibleTarget,

    /// Preserve target-specific commitments as unresolved intent.
    ///
    /// This is suitable for an intermediate stage that can carry such intent
    /// without claiming that the target has been selected or validated.
    PreserveUnresolved,

    /// Reject target-specific commitments at the current boundary.
    ///
    /// Use this policy when a consumer explicitly requires portable-only IR.
    RejectTargetSpecific,
}

/// Stateless registration and policy entry point for physical intent.
///
/// This type does not own declarations or the design graph.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct PhysicalIntentDialect;

impl PhysicalIntentDialect {
    /// Return the canonical qualified name of this dialect.
    ///
    /// # Errors
    ///
    /// Returns a structured error if the built-in schema name violates the
    /// shared qualified-name contract.
    pub fn name() -> HdlirResult<QualifiedName> {
        QualifiedName::parse(PHYSICAL_INTENT_DIALECT_NAME)
    }

    /// Construct the versioned dialect definition for registration.
    ///
    /// The definition establishes a namespace and its documented purpose.
    /// It does not assert that operations in this namespace are implemented.
    ///
    /// # Errors
    ///
    /// Returns a structured error if the name or definition is invalid.
    pub fn definition() -> HdlirResult<DialectDefinition> {
        DialectDefinition::new(
            Self::name()?,
            PHYSICAL_INTENT_DIALECT_VERSION,
            DefinitionStability::Core,
            "Explicit target-aware hardware implementation intent, \
             constraints, preferences, hints, policies, and commitments.",
        )
    }

    /// Register the dialect identity in the shared schema registry.
    ///
    /// Registration is atomic through `register_batch`. Existing definitions
    /// are not replaced. Operation definitions must be registered separately
    /// when their contracts and semantics have been implemented.
    ///
    /// # Errors
    ///
    /// Returns a structured error for duplicate registration, invalid schema
    /// definitions, policy violations, or registry-generation exhaustion.
    pub fn register(
        registry: &mut SchemaRegistry,
        extension_policy: ExtensionPolicy,
    ) -> HdlirResult<u64> {
        let batch = RegistryBatch::new().with_dialect(Self::definition()?);
        registry.register_batch(batch, extension_policy)
    }

    /// Check whether a classified intent is permitted at a boundary that
    /// requires portable-only declarations.
    ///
    /// A target commitment is not portable-only intent. Other classifications
    /// can still contain target-specific expressions, so callers must also
    /// verify the expression's references and target dependencies.
    #[must_use]
    pub const fn is_portable_classification(kind: PhysicalIntentKind) -> bool {
        !matches!(kind, PhysicalIntentKind::TargetCommitment)
    }

    /// Validate a target commitment against the selected target-binding
    /// policy and whether a compatible target context has been established.
    ///
    /// This checks policy only. It does not prove target capability,
    /// resource feasibility, timing closure, or physical realizability.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the policy rejects the commitment
    /// or requires a compatible target that has not been established.
    pub fn validate_target_commitment(
        policy: TargetBindingPolicy,
        compatible_target_established: bool,
    ) -> HdlirResult<()> {
        match policy {
            TargetBindingPolicy::RequireCompatibleTarget
                if !compatible_target_established =>
            {
                Err(crate::hdl::hdlir::errors::HdlirError::invalid_input(
                    "physical_intent.target",
                    "target-specific intent requires an established compatible target",
                ))
            }
            TargetBindingPolicy::RejectTargetSpecific => {
                Err(crate::hdl::hdlir::errors::HdlirError::invalid_input(
                    "physical_intent.target",
                    "target-specific intent is not permitted at this boundary",
                ))
            }
            TargetBindingPolicy::RequireCompatibleTarget
            | TargetBindingPolicy::PreserveUnresolved => Ok(()),
        }
    }

    /// Validate that a declaration's scope and classification are
    /// compatible with a portable-only boundary.
    ///
    /// This method validates the classification and target-binding policy
    /// only. The caller must still verify the declaration's actual scope
    /// reference, expression, units, and semantic conflicts.
    ///
    /// # Errors
    ///
    /// Returns a structured error when portable-only mode rejects a
    /// target-specific commitment.
    pub fn validate_portability(
        kind: PhysicalIntentKind,
        portable_only: bool,
    ) -> HdlirResult<()> {
        if portable_only && !Self::is_portable_classification(kind) {
            return Err(crate::hdl::hdlir::errors::HdlirError::invalid_input(
                "physical_intent.kind",
                "target commitment is not permitted in portable-only mode",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialect_identity_is_stable() {
        let name = PhysicalIntentDialect::name()
            .expect("built-in dialect name must be valid");

        assert_eq!(name.as_str(), PHYSICAL_INTENT_DIALECT_NAME);
        assert_eq!(PHYSICAL_INTENT_DIALECT_VERSION.major(), 1);
        assert_eq!(PHYSICAL_INTENT_DIALECT_VERSION.minor(), 0);
        assert_eq!(PHYSICAL_INTENT_DIALECT_VERSION.patch(), 0);
    }

    #[test]
    fn registration_uses_the_shared_registry() {
        let mut registry = SchemaRegistry::new();

        let generation = PhysicalIntentDialect::register(
            &mut registry,
            ExtensionPolicy::StableOnly,
        )
        .expect("physical-intent dialect registration should succeed");

        assert_eq!(generation, 1);

        let name = PhysicalIntentDialect::name()
            .expect("built-in dialect name must be valid");

        assert!(registry.dialect(&name).is_some());
    }

    #[test]
    fn duplicate_registration_is_rejected_atomically() {
        let mut registry = SchemaRegistry::new();

        PhysicalIntentDialect::register(
            &mut registry,
            ExtensionPolicy::StableOnly,
        )
        .expect("first registration should succeed");

        let generation_before = registry.generation();

        assert!(
            PhysicalIntentDialect::register(
                &mut registry,
                ExtensionPolicy::StableOnly,
            )
            .is_err()
        );

        assert_eq!(registry.generation(), generation_before);
    }

    #[test]
    fn portable_classifications_exclude_target_commitments() {
        assert!(PhysicalIntentDialect::is_portable_classification(
            PhysicalIntentKind::Requirement
        ));
        assert!(PhysicalIntentDialect::is_portable_classification(
            PhysicalIntentKind::Constraint
        ));
        assert!(PhysicalIntentDialect::is_portable_classification(
            PhysicalIntentKind::Preference
        ));
        assert!(PhysicalIntentDialect::is_portable_classification(
            PhysicalIntentKind::Hint
        ));
        assert!(PhysicalIntentDialect::is_portable_classification(
            PhysicalIntentKind::Policy
        ));
        assert!(!PhysicalIntentDialect::is_portable_classification(
            PhysicalIntentKind::TargetCommitment
        ));
    }

    #[test]
    fn portable_only_mode_rejects_target_commitments() {
        assert!(
            PhysicalIntentDialect::validate_portability(
                PhysicalIntentKind::Requirement,
                true,
            )
            .is_ok()
        );

        assert!(
            PhysicalIntentDialect::validate_portability(
                PhysicalIntentKind::TargetCommitment,
                true,
            )
            .is_err()
        );

        assert!(
            PhysicalIntentDialect::validate_portability(
                PhysicalIntentKind::TargetCommitment,
                false,
            )
            .is_ok()
        );
    }

    #[test]
    fn target_binding_policy_is_explicit() {
        assert!(
            PhysicalIntentDialect::validate_target_commitment(
                TargetBindingPolicy::RequireCompatibleTarget,
                false,
            )
            .is_err()
        );

        assert!(
            PhysicalIntentDialect::validate_target_commitment(
                TargetBindingPolicy::RequireCompatibleTarget,
                true,
            )
            .is_ok()
        );

        assert!(
            PhysicalIntentDialect::validate_target_commitment(
                TargetBindingPolicy::PreserveUnresolved,
                false,
            )
            .is_ok()
        );

        assert!(
            PhysicalIntentDialect::validate_target_commitment(
                TargetBindingPolicy::RejectTargetSpecific,
                true,
            )
            .is_err()
        );
    }
}
