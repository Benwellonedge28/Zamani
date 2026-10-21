
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Quantum-target compatibility for the Zamani backend.
//!
//! Path: `src/backend/quantum/compatibility.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Evaluate whether a target can satisfy the quantum feature and resource
//! requirements supplied by the compiler for a quantum compilation request.
//!
//! This module is an adapter over the existing, canonical backend
//! compatibility evaluator. It deliberately does not implement a second
//! capability registry, resource model, or compatibility policy.
//!
//! # Ownership
//!
//! This module owns:
//! - Quantum-specific validation of requirement namespaces.
//! - A convenient quantum request builder.
//! - A quantum target-profile facade.
//! - Quantum-specific evaluation and reporting helpers.
//!
//! The parent backend compatibility module owns:
//! - CapabilityId and ResourceId.
//! - CapabilitySupport and SupportLevel.
//! - ResourceAvailability and DecimalInteger.
//! - CompatibilityRequest and CompatibilityPolicy.
//! - CompatibilityEvaluator and CompatibilityReport.
//! - The canonical compatibility decision rules.
//!
//! `crate::quantum::ir` remains the canonical quantum IR.
//!
//! # Non-responsibilities
//!
//! This module does not:
//! - Inspect source syntax or define language semantics.
//! - Invent quantum requirements by guessing from target names.
//! - Define a closed list of quantum gates or operation dialects.
//! - Perform lowering, decomposition, routing, scheduling, or placement.
//! - Allocate physical qubits or contact a quantum device.
//! - Implement simulation, calibration, QEC, or noise modelling.
//! - Claim that compilation succeeded merely because a target profile exists.
//!
//! The caller must derive requirements from verified semantic facts or the
//! canonical quantum IR and supply evidence from the actual target adapter.
//!
//! # Compatibility invariant
//!
//! A target is accepted only when the shared evaluator determines that all
//! hard requirements are satisfied under the supplied evidence and policy.
//! Unknown evidence is not success. Partial support is not full support.
//! Experimental support is not accepted for hard requirements by default.
//!
//! # Scalability
//!
//! There is no fixed maximum number of features, resource dimensions,
//! logical qubits, physical qubits, operations, dialects, or target types.
//! Resource quantities use the shared compatibility model's decimal-string
//! representation. Concrete resource limits belong to explicit policy and
//! target evidence, not to this module.
//!
//! # Integration order
//!
//! 1. `src/backend/compatibility.rs` remains the compatibility authority.
//! 2. The quantum semantic/IR analysis produces a request.
//! 3. The target adapter supplies evidence in a QuantumTargetProfile.
//! 4. This module invokes the shared evaluator.
//! 5. Selection/validation decides whether the compilation pipeline proceeds.
//! 6. Quantum lowering and hardware realization remain downstream.

use std::error::Error;
use std::fmt;

use super::super::compatibility::{
    CapabilityId,
    CapabilityRequirement,
    CapabilitySupport,
    CompatibilityError,
    CompatibilityEvaluator,
    CompatibilityPolicy,
    CompatibilityReport,
    CompatibilityRequest,
    DecimalInteger,
    InventoryCompleteness,
    RequirementStrength,
    ResourceAvailability,
    ResourceId,
    ResourceRequirement,
    SupportLevel,
    TargetCapabilityProfile,
};

/// Errors specific to building a quantum compatibility request or profile.
///
/// Errors originating in the shared compatibility model are preserved as
/// `Shared`; quantum namespace errors are reported as `InvalidQuantumId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantumCompatibilityError {
    /// A feature or resource identifier does not belong to the quantum
    /// namespace required by this API.
    InvalidQuantumId {
        kind: &'static str,
        value: String,
    },

    /// The shared compatibility model rejected an identifier, quantity,
    /// duplicate declaration, or target profile.
    Shared(CompatibilityError),
}

impl fmt::Display for QuantumCompatibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuantumId { kind, value } => write!(
                formatter,
                "invalid quantum {kind} identifier `{value}`: \
                 identifiers must use the `quantum.` namespace"
            ),
            Self::Shared(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for QuantumCompatibilityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidQuantumId { .. } => None,
            Self::Shared(error) => Some(error),
        }
    }
}

impl From<CompatibilityError> for QuantumCompatibilityError {
    fn from(error: CompatibilityError) -> Self {
        Self::Shared(error)
    }
}

/// Result type for quantum compatibility construction operations.
pub type QuantumCompatibilityResult<T> =
    Result<T, QuantumCompatibilityError>;

/// Validate the namespace used by this quantum-specific API.
///
/// The namespace is a convention and ownership boundary, not a closed list
/// of known operations. Examples such as `quantum.measurement` and
/// `quantum.dynamic_control` are illustrative, not exhaustive.
fn validate_quantum_identifier(
    kind: &'static str,
    value: &str,
) -> QuantumCompatibilityResult<()> {
    if !value.starts_with("quantum.")
        || value.len() == "quantum.".len()
    {
        return Err(QuantumCompatibilityError::InvalidQuantumId {
            kind,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// A target-independent quantum feature requirement.
///
/// Feature identifiers are extensible strings in the `quantum.` namespace.
/// They describe a semantic capability required by the program, not a
/// vendor gate name or a physical instruction.
///
/// Use narrow feature IDs when distinctions affect program semantics.
/// For example, a program requiring mid-circuit measurement must not be
/// represented as requiring only generic measurement support.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumFeatureRequirement {
    capability: CapabilityId,
    strength: RequirementStrength,
    rationale: Option<String>,
}

impl QuantumFeatureRequirement {
    /// Construct a quantum feature requirement.
    pub fn new(
        feature: impl Into<String>,
        strength: RequirementStrength,
    ) -> QuantumCompatibilityResult<Self> {
        let feature = feature.into();
        validate_quantum_identifier("feature", &feature)?;

        let capability = CapabilityId::new(feature)?;

        Ok(Self {
            capability,
            strength,
            rationale: None,
        })
    }

    /// Construct a hard requirement that the target must satisfy.
    pub fn required(
        feature: impl Into<String>,
    ) -> QuantumCompatibilityResult<Self> {
        Self::new(feature, RequirementStrength::Required)
    }

    /// Construct a mandatory implementation constraint.
    pub fn constraint(
        feature: impl Into<String>,
    ) -> QuantumCompatibilityResult<Self> {
        Self::new(feature, RequirementStrength::Constraint)
    }

    /// Construct a non-mandatory target preference.
    pub fn preference(
        feature: impl Into<String>,
    ) -> QuantumCompatibilityResult<Self> {
        Self::new(feature, RequirementStrength::Preference)
    }

    /// Construct a non-binding implementation hint.
    pub fn hint(
        feature: impl Into<String>,
    ) -> QuantumCompatibilityResult<Self> {
        Self::new(feature, RequirementStrength::Hint)
    }

    /// Attach human-readable provenance for the requirement.
    ///
    /// Rationale is descriptive; compatibility decisions use the feature
    /// identifier and requirement strength, not this text.
    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Self {
        self.rationale = Some(rationale.into());
        self
    }

    /// Return the canonical feature identifier.
    pub fn feature(&self) -> &str {
        self.capability.as_str()
    }

    /// Return the requirement strength.
    pub fn strength(&self) -> RequirementStrength {
        self.strength
    }

    fn into_shared(self) -> CapabilityRequirement {
        let mut requirement =
            CapabilityRequirement::new(self.capability, self.strength);

        if let Some(rationale) = self.rationale {
            requirement = requirement.with_rationale(rationale);
        }

        requirement
    }
}

/// A minimum requirement for a quantum resource dimension.
///
/// Resource names are extensible. Examples include
/// `quantum.logical_qubits` and `quantum.physical_qubits`; these are
/// identifiers, not universal capacity assumptions.
///
/// Quantities are non-negative decimal integers as defined by the shared
/// compatibility contract, avoiding a fixed machine-integer width in this
/// API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumResourceRequirement {
    resource: ResourceId,
    minimum: DecimalInteger,
    strength: RequirementStrength,
    rationale: Option<String>,
}

impl QuantumResourceRequirement {
    /// Construct a quantum resource requirement.
    pub fn new(
        resource: impl Into<String>,
        minimum: impl Into<String>,
        strength: RequirementStrength,
    ) -> QuantumCompatibilityResult<Self> {
        let resource = resource.into();
        validate_quantum_identifier("resource", &resource)?;

        Ok(Self {
            resource: ResourceId::new(resource)?,
            minimum: DecimalInteger::new(minimum)?,
            strength,
            rationale: None,
        })
    }

    /// Construct a hard minimum resource requirement.
    pub fn required(
        resource: impl Into<String>,
        minimum: impl Into<String>,
    ) -> QuantumCompatibilityResult<Self> {
        Self::new(resource, minimum, RequirementStrength::Required)
    }

    /// Attach human-readable provenance for the requirement.
    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Self {
        self.rationale = Some(rationale.into());
        self
    }

    /// Return the resource identifier.
    pub fn resource(&self) -> &str {
        self.resource.as_str()
    }

    /// Return the required minimum as a decimal string.
    pub fn minimum(&self) -> &str {
        self.minimum.as_str()
    }

    fn into_shared(self) -> ResourceRequirement {
        let mut requirement = ResourceRequirement::new(
            self.resource,
            self.minimum,
            self.strength,
        );

        if let Some(rationale) = self.rationale {
            requirement = requirement.with_rationale(rationale);
        }

        requirement
    }
}

/// Requirements collected from the verified quantum program.
///
/// This is a request, not an IR. It must be derived from canonical semantic
/// facts, IR analysis, or explicit user constraints. It must not be inferred
/// from a source filename, target name, or a fixed gate catalogue.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QuantumCompatibilityRequest {
    features: Vec<QuantumFeatureRequirement>,
    resources: Vec<QuantumResourceRequirement>,
    policy: CompatibilityPolicy,
}

impl QuantumCompatibilityRequest {
    /// Create an empty request with the shared evaluator's safe default
    /// policy. Experimental support does not satisfy hard requirements by
    /// default.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace the compatibility policy explicitly.
    ///
    /// Enabling experimental support is a caller decision and must remain
    /// visible to the user and in build provenance.
    pub fn with_policy(mut self, policy: CompatibilityPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Add a feature requirement.
    pub fn require_feature(
        &mut self,
        requirement: QuantumFeatureRequirement,
    ) {
        self.features.push(requirement);
    }

    /// Add a resource requirement.
    pub fn require_resource(
        &mut self,
        requirement: QuantumResourceRequirement,
    ) {
        self.resources.push(requirement);
    }

    /// Builder-style feature requirement addition.
    pub fn with_feature(
        mut self,
        requirement: QuantumFeatureRequirement,
    ) -> Self {
        self.require_feature(requirement);
        self
    }

    /// Builder-style resource requirement addition.
    pub fn with_resource(
        mut self,
        requirement: QuantumResourceRequirement,
    ) -> Self {
        self.require_resource(requirement);
        self
    }

    /// Number of declared feature requirements.
    pub fn feature_count(&self) -> usize {
        self.features.len()
    }

    /// Number of declared resource requirements.
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Evaluate the request against an evidence-bearing target profile.
    ///
    /// The shared evaluator owns all compatibility decision rules and
    /// deterministic diagnostic ordering.
    pub fn evaluate(
        &self,
        target: &QuantumTargetProfile,
    ) -> QuantumCompatibilityReport {
        let mut request = CompatibilityRequest::new()
            .with_policy(self.policy);

        for feature in &self.features {
            request.require_capability(
                CapabilityRequirement::new(
                    feature.capability.clone(),
                    feature.strength,
                )
                .with_optional_rationale(feature.rationale.as_deref()),
            );
        }

        for resource in &self.resources {
            request.require_resource(
                ResourceRequirement::new(
                    resource.resource.clone(),
                    resource.minimum.clone(),
                    resource.strength,
                )
                .with_optional_rationale(resource.rationale.as_deref()),
            );
        }

        let report = CompatibilityEvaluator.evaluate(
            &request,
            target.shared_profile(),
        );

        QuantumCompatibilityReport { report }
    }

    /// Convert this request to the shared backend representation.
    ///
    /// This is useful when the generic backend selection or validation
    /// pipeline must evaluate quantum requirements alongside other domains.
    pub fn into_shared(self) -> CompatibilityRequest {
        let mut request =
            CompatibilityRequest::new().with_policy(self.policy);

        for feature in self.features {
            request.require_capability(feature.into_shared());
        }

        for resource in self.resources {
            request.require_resource(resource.into_shared());
        }

        request
    }
}

/// A quantum target's declared capability and resource evidence.
///
/// This facade wraps the existing generic target profile. It does not infer
/// capabilities from a target's name, vendor, or architecture.
///
/// Target adapters must declare only what their implementation and toolchain
/// can substantiate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumTargetProfile {
    profile: TargetCapabilityProfile,
}

impl QuantumTargetProfile {
    /// Construct a target profile with an incomplete capability inventory.
    ///
    /// Unknown features remain unknown until evidence is supplied. This is
    /// safer than assuming an unlisted feature is unsupported.
    pub fn new(
        target_id: impl Into<String>,
    ) -> QuantumCompatibilityResult<Self> {
        Ok(Self {
            profile: TargetCapabilityProfile::new(target_id)?,
        })
    }

    /// Declare support evidence for a quantum feature.
    ///
    /// Duplicate capability declarations are rejected by the shared profile.
    pub fn declare_feature(
        &mut self,
        feature: impl Into<String>,
        level: SupportLevel,
        evidence: Option<String>,
        limitations: Vec<String>,
    ) -> QuantumCompatibilityResult<()> {
        let feature = feature.into();
        validate_quantum_identifier("feature", &feature)?;

        let capability = CapabilityId::new(feature)?;
        let mut support = CapabilitySupport::new(capability, level);

        if let Some(evidence) = evidence {
            support = support.with_evidence(evidence);
        }

        for limitation in limitations {
            support = support.with_limitation(limitation);
        }

        self.profile.declare_capability(support)?;
        Ok(())
    }

    /// Declare known resource availability.
    pub fn declare_resource(
        &mut self,
        resource: impl Into<String>,
        availability: ResourceAvailability,
    ) -> QuantumCompatibilityResult<()> {
        let resource = resource.into();
        validate_quantum_identifier("resource", &resource)?;

        self.profile.declare_resource(
            ResourceId::new(resource)?,
            availability,
        )?;

        Ok(())
    }

    /// Mark the target's capability inventory exhaustive.
    ///
    /// Only call this when the target adapter can substantiate completeness.
    /// Once complete, an undeclared feature is considered unsupported by the
    /// shared evaluator.
    pub fn mark_inventory_complete(&mut self) {
        self.profile.mark_capability_inventory_complete();
    }

    /// Return the target identifier.
    pub fn target_id(&self) -> &str {
        &self.profile.target_id
    }

    /// Return whether the profile claims an exhaustive capability inventory.
    pub fn inventory_completeness(&self) -> InventoryCompleteness {
        self.profile.inventory_completeness
    }

    /// Access the shared compatibility profile without duplicating it.
    pub fn shared_profile(&self) -> &TargetCapabilityProfile {
        &self.profile
    }

    /// Consume the facade and return the canonical shared profile.
    pub fn into_shared(self) -> TargetCapabilityProfile {
        self.profile
    }
}

/// Quantum-facing view of the canonical compatibility report.
///
/// The underlying report is preserved so generic selection and validation
/// can use the same status, issue codes, severities, and evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumCompatibilityReport {
    report: CompatibilityReport,
}

impl QuantumCompatibilityReport {
    /// Return the canonical generic compatibility report.
    pub fn as_shared(&self) -> &CompatibilityReport {
        &self.report
    }

    /// Consume this wrapper and return the shared report.
    pub fn into_shared(self) -> CompatibilityReport {
        self.report
    }

    /// Return the evaluated target identifier.
    pub fn target_id(&self) -> &str {
        &self.report.target_id
    }

    /// Return the canonical compatibility status.
    pub fn status(
        &self,
    ) -> super::super::compatibility::CompatibilityStatus {
        self.report.status
    }

    /// Return true only when the shared evaluator accepts the target.
    ///
    /// Indeterminate evidence is not considered compatible.
    pub fn is_compatible(&self) -> bool {
        self.report.is_compatible()
    }

    /// Return true only for a fully compatible report without warnings.
    pub fn is_fully_compatible(&self) -> bool {
        self.report.is_fully_compatible()
    }

    /// Return true if the report contains errors.
    pub fn has_errors(&self) -> bool {
        self.report.has_errors()
    }

    /// Return true if the report contains warnings.
    pub fn has_warnings(&self) -> bool {
        self.report.has_warnings()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::compatibility::{
        CompatibilityStatus,
        DecimalInteger,
    };

    fn feature(name: &str) -> QuantumFeatureRequirement {
        QuantumFeatureRequirement::required(name)
            .expect("test feature identifier should be valid")
    }

    fn target_with_feature(
        target_id: &str,
        name: &str,
        level: SupportLevel,
    ) -> QuantumTargetProfile {
        let mut target = QuantumTargetProfile::new(target_id)
            .expect("test target identifier should be valid");

        target
            .declare_feature(name, level, Some("test evidence".into()), vec![])
            .expect("test feature should be declared");

        target
    }

    #[test]
    fn accepts_a_declared_supported_feature() {
        let request = QuantumCompatibilityRequest::new()
            .with_feature(feature("quantum.measurement"));

        let target = target_with_feature(
            "test.quantum",
            "quantum.measurement",
            SupportLevel::Supported,
        );

        let report = request.evaluate(&target);

        assert!(report.is_compatible());
        assert_eq!(report.target_id(), "test.quantum");
    }

    #[test]
    fn rejects_an_explicitly_unsupported_hard_requirement() {
        let request = QuantumCompatibilityRequest::new()
            .with_feature(feature("quantum.measurement"));

        let target = target_with_feature(
            "test.quantum",
            "quantum.measurement",
            SupportLevel::Unsupported,
        );

        let report = request.evaluate(&target);

        assert!(!report.is_compatible());
        assert!(report.has_errors());
        assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    }

    #[test]
    fn unknown_feature_is_not_assumed_to_be_supported() {
        let request = QuantumCompatibilityRequest::new()
            .with_feature(feature("quantum.future_operation"));

        let target = QuantumTargetProfile::new("incomplete.target")
            .expect("test target identifier should be valid");

        let report = request.evaluate(&target);

        assert_eq!(report.status(), CompatibilityStatus::Indeterminate);
        assert!(!report.is_compatible());
    }

    #[test]
    fn complete_inventory_makes_an_unlisted_feature_unsupported() {
        let request = QuantumCompatibilityRequest::new()
            .with_feature(feature("quantum.future_operation"));

        let mut target = QuantumTargetProfile::new("complete.target")
            .expect("test target identifier should be valid");
        target.mark_inventory_complete();

        let report = request.evaluate(&target);

        assert_eq!(report.status(), CompatibilityStatus::Incompatible);
        assert!(!report.is_compatible());
    }

    #[test]
    fn partial_support_does_not_satisfy_a_hard_requirement() {
        let request = QuantumCompatibilityRequest::new()
            .with_feature(feature("quantum.dynamic_control"));

        let target = target_with_feature(
            "partial.target",
            "quantum.dynamic_control",
            SupportLevel::Partial,
        );

        let report = request.evaluate(&target);

        assert!(!report.is_compatible());
        assert!(report.has_errors());
    }

    #[test]
    fn experimental_support_is_not_accepted_by_default() {
        let request = QuantumCompatibilityRequest::new()
            .with_feature(feature("quantum.mid_circuit_measurement"));

        let target = target_with_feature(
            "experimental.target",
            "quantum.mid_circuit_measurement",
            SupportLevel::Experimental,
        );

        let report = request.evaluate(&target);

        assert!(!report.is_compatible());
    }

    #[test]
    fn feature_identifiers_are_open_world_but_namespaced() {
        assert!(
            QuantumFeatureRequirement::required(
                "quantum.vendor_extension.new_operation"
            )
            .is_ok()
        );

        assert!(
            QuantumFeatureRequirement::required("vendor.new_operation")
                .is_err()
        );
    }

    #[test]
    fn quantum_resource_quantities_use_the_shared_decimal_contract() {
        let requirement = QuantumResourceRequirement::required(
            "quantum.logical_qubits",
            "100000000000000000000000000000000000000000000000000000000000",
        )
        .expect("valid decimal quantity should be accepted");

        assert_eq!(requirement.minimum(), "100000000000000000000000000000000000000000000000000000000000");
    }

    #[test]
    fn duplicate_feature_declarations_are_rejected() {
        let mut target = QuantumTargetProfile::new("duplicate.target")
            .expect("test target identifier should be valid");

        target
            .declare_feature(
                "quantum.measurement",
                SupportLevel::Supported,
                None,
                vec![],
            )
            .expect("first declaration should succeed");

        assert!(
            target
                .declare_feature(
                    "quantum.measurement",
                    SupportLevel::Supported,
                    None,
                    vec![],
                )
                .is_err()
        );
    }

    #[test]
    fn resource_identifier_validation_does_not_require_a_fixed_capacity() {
        let resource = ResourceId::new("quantum.logical_qubits")
            .expect("resource ID should be valid");
        let quantity = DecimalInteger::new("999999999999999999999999999999")
            .expect("decimal resource value should be valid");

        assert_eq!(resource.as_str(), "quantum.logical_qubits");
        assert_eq!(quantity.as_str(), "999999999999999999999999999999");
    }
}
