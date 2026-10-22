
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Domain Interface Contracts
//!
//! Path: `src/hybrid/domain/interface.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Define the domain-participation interface used by Zamani's hybrid
//! composition subsystem.
//!
//! An interface describes how an independently defined computational domain
//! participates in a hybrid computation. It combines the canonical
//! `DomainContract` with additional interface-level declarations for:
//!
//! - required capabilities;
//! - semantic effects;
//! - completion behavior;
//! - failure behavior;
//! - version compatibility expectations;
//! - portability claims.
//!
//! The interface is descriptive metadata. It does not execute a computation,
//! establish that an implementation is trustworthy, or prove that a target
//! can realize the computation.
//!
//! ## Canonical ownership
//!
//! Reuse the existing owners:
//!
//! - `contract.rs`: DomainContract, DomainInterfaceRef, inputs, outputs,
//!   ownership expectations, dependencies and semantic guarantees.
//! - `domain/identity.rs`: DomainId.
//! - `version.rs`: DomainContractVersion.
//! - `errors.rs`: HybridError and HybridErrorCode.
//! - `config.rs`: HybridConfig and operational budgets.
//!
//! This module does not create another IR, AST, type system, effect checker,
//! capability registry, compiler, scheduler, or runtime.
//!
//! ## Portability
//!
//! Domain identifiers, capability identifiers and effect identifiers are
//! extensible. There is no fixed inventory of domains or capabilities and
//! no hardware-size ceiling.
//!
//! Operational budgets are explicit policy. The absence of a configured
//! budget is not a claim of infinite physical memory or execution time.
//!
//! A portability declaration is metadata, not proof of universal execution.
//! A target that cannot satisfy required semantics must be rejected or
//! reported as infeasible by the appropriate downstream subsystem.
//!
//! ## Integration
//!
//! - `domain/identity.rs` owns validated domain identity.
//! - `contract.rs` owns the underlying interface and port contract.
//! - `domain/registry.rs` owns registration and exact-version lookup.
//! - `domain/composition.rs` owns participation in a composition.
//! - `graph/verify.rs` owns graph-level dependency validation.
//! - `resources/` owns capability satisfaction and feasibility analysis.
//! - `integration/semantic.rs` maps canonical semantic facts into interfaces.
//! - `integration/quantum.rs` references `quantum::ir`; it does not redefine
//!   quantum operation semantics.
//! - `integration/classical.rs` reuses the canonical classical facilities.
//! - Compiler, backend and runtime layers own realization and execution.
//! - `grammar/spec/hybrid.md` and `grammar/hybrid/` remain authoritative for
//!   source-language and hybrid-composition semantics.
//!
//! This module intentionally does not depend on the parser, AST, quantum IR,
//! compiler, runtime, hardware discovery, or vendor SDKs.

use serde::{Deserialize, Serialize};

use super::identity::{DomainId, DomainIdError};
use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::contract::{
    DomainContract,
    DomainInterfaceRef,
};
use crate::hybrid::errors::{
    HybridError,
    HybridErrorCode,
};
use crate::hybrid::version::DomainContractVersion;

/// The strength of a capability requirement.
///
/// Required capabilities are necessary for a conforming realization.
/// Advisory capabilities may guide implementation choices but cannot
/// silently change the meaning of the program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStrength {
    /// The implementation must establish that this capability is satisfied.
    Required,

    /// The capability is a preference, not a semantic prerequisite.
    Advisory,
}

/// A namespaced capability requirement.
///
/// Capability names are open identifiers, not a closed enumeration.
/// Examples include `quantum.measurement`, `tensor.compute`, and
/// `accelerator.parallel_execution`.
///
/// This declaration does not assert that a capability is available.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    capability: String,
    strength: CapabilityStrength,
    rationale: Option<String>,
}

impl CapabilityRequirement {
    /// Construct a capability requirement.
    ///
    /// The capability identifier uses the same extensible identifier
    /// grammar as DomainId. No fixed identifier-length ceiling is imposed.
    pub fn new(
        capability: impl Into<String>,
        strength: CapabilityStrength,
        rationale: Option<String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            capability: capability.into(),
            strength,
            rationale,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the canonical capability identifier.
    #[must_use]
    pub fn capability(&self) -> &str {
        &self.capability
    }

    /// Return the requirement strength.
    #[must_use]
    pub const fn strength(&self) -> CapabilityStrength {
        self.strength
    }

    /// Return the optional explanation.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }

    /// Whether the capability is a mandatory requirement.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        matches!(self.strength, CapabilityStrength::Required)
    }

    /// Validate this declaration, including after deserialization.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_namespaced_identifier(
            "capability",
            &self.capability,
        )?;

        if let Some(rationale) = &self.rationale {
            validate_nonempty_text("capability rationale", rationale)?;
        }

        Ok(())
    }
}

/// A reference to an effect defined by the canonical semantic model.
///
/// The identifier names an effect; it does not implement the effect or
/// replace semantic effect checking.
///
/// Examples of possible effect namespaces include `classical.io`,
/// `quantum.measurement`, and `distributed.communication`. These examples
/// are identifiers, not a closed list of effects.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EffectRef {
    identifier: String,
}

impl EffectRef {
    /// Construct an effect reference.
    pub fn new(
        identifier: impl Into<String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            identifier: identifier.into(),
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the canonical effect identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Validate this reference, including after deserialization.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_namespaced_identifier("effect", &self.identifier)
    }
}

/// Declared completion behavior for an interface invocation.
///
/// These values describe a contract. They are not runtime states and do not
/// replace the synchronization or completion model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionBehavior {
    /// The invocation's result is available when the invocation returns.
    CompleteOnReturn,

    /// The invocation may return before its computation completes.
    MayReturnPending,

    /// Completion follows an explicit dependency or completion contract.
    ExplicitCompletionContract,

    /// Completion semantics are defined by an external canonical contract.
    ExternalContract,
}

/// Declared failure behavior for an interface invocation.
///
/// This declaration informs downstream error and recovery analysis.
/// It does not guarantee that an implementation will behave correctly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureBehavior {
    /// Failure is propagated according to the canonical error contract.
    Propagate,

    /// The contract permits a failure to be handled by a declared recovery
    /// mechanism.
    Recoverable,

    /// Failure may leave a partial result whose validity is separately
    /// described by the canonical result contract.
    PartialResultPossible,

    /// Failure semantics are defined by another canonical contract.
    ExternalContract,
}

/// The portability claim made by an interface declaration.
///
/// These are descriptive claims. They do not establish that a particular
/// target supports the interface or that an implementation is conforming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortabilityLevel {
    /// The interface contract expresses no inherent target-specific
    /// requirement. Required capabilities and resources still apply.
    TargetIndependent,

    /// Realization depends on required capabilities, resources or supported
    /// implementation mappings.
    CapabilityConditional,

    /// Realization depends on an explicitly documented implementation-specific
    /// contract. The source program's semantics must still be preserved.
    ImplementationSpecific,
}

/// Portability metadata associated with an interface.
///
/// `rationale` explains the declaration. It is not a substitute for
/// capability checks, compatibility validation or implementation evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortabilityDeclaration {
    level: PortabilityLevel,
    rationale: Option<String>,
}

impl PortabilityDeclaration {
    /// Construct portability metadata.
    pub fn new(
        level: PortabilityLevel,
        rationale: Option<String>,
    ) -> Result<Self, HybridError> {
        let value = Self { level, rationale };
        value.validate()?;
        Ok(value)
    }

    /// Return the declared portability level.
    #[must_use]
    pub const fn level(&self) -> PortabilityLevel {
        self.level
    }

    /// Return the optional explanation.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }

    /// Validate this declaration, including after deserialization.
    pub fn validate(&self) -> Result<(), HybridError> {
        if let Some(rationale) = &self.rationale {
            validate_nonempty_text("portability rationale", rationale)?;
        }

        Ok(())
    }
}

/// Version matching expectation declared by a consumer of an interface.
///
/// A version policy describes which references may be considered for further
/// compatibility analysis. It does not prove semantic compatibility.
///
/// In particular, equal major versions alone are not proof that two domain
/// contracts have equivalent meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionCompatibilityPolicy {
    /// Only the exact requested version may be selected.
    Exact,

    /// A matching major version is a candidate for further compatibility
    /// analysis. It must not be accepted without that analysis.
    SameMajorRequiresValidation,

    /// A different version requires an explicit, validated adapter or
    /// migration contract.
    ExplicitAdapterRequired,
}

impl VersionCompatibilityPolicy {
    /// Determine whether two versions are candidates under this policy.
    ///
    /// `true` means only that the version-selection rule permits further
    /// analysis. It is not proof that the versions are semantically
    /// compatible or that the interface implementation is available.
    #[must_use]
    pub const fn permits_compatibility_analysis(
        self,
        requested: DomainContractVersion,
        offered: DomainContractVersion,
    ) -> bool {
        match self {
            Self::Exact => {
                requested.major() == offered.major()
                    && requested.minor() == offered.minor()
                    && requested.patch() == offered.patch()
            }
            Self::SameMajorRequiresValidation => {
                requested.major() == offered.major()
            }
            Self::ExplicitAdapterRequired => {
                requested.major() == offered.major()
                    && requested.minor() == offered.minor()
                    && requested.patch() == offered.patch()
            }
        }
    }
}

/// The canonical interface descriptor for a participating domain.
///
/// This descriptor wraps the existing DomainContract rather than duplicating
/// its input/output/dependency/guarantee declarations.
///
/// Capability requirements, effects, completion, failure, version policy,
/// and portability are additional participation metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainInterface {
    contract: DomainContract,
    domain: DomainId,
    capabilities: Vec<CapabilityRequirement>,
    effects: Vec<EffectRef>,
    completion: CompletionBehavior,
    failure: FailureBehavior,
    version_policy: VersionCompatibilityPolicy,
    portability: PortabilityDeclaration,
}

impl DomainInterface {
    /// Construct a domain interface and validate its internal structure.
    ///
    /// The supplied DomainId must agree with the domain identifier already
    /// declared in the canonical DomainContract.
    ///
    /// This constructor does not register the interface, resolve dependencies,
    /// check target capabilities, or prove implementation conformance.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contract: DomainContract,
        capabilities: Vec<CapabilityRequirement>,
        effects: Vec<EffectRef>,
        completion: CompletionBehavior,
        failure: FailureBehavior,
        version_policy: VersionCompatibilityPolicy,
        portability: PortabilityDeclaration,
    ) -> Result<Self, HybridError> {
        let domain = DomainId::new(contract.interface().domain())
            .map_err(domain_id_error)?;

        let value = Self {
            contract,
            domain,
            capabilities,
            effects,
            completion,
            failure,
            version_policy,
            portability,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the canonical domain identity.
    #[must_use]
    pub fn domain(&self) -> &DomainId {
        &self.domain
    }

    /// Return the canonical underlying interface contract.
    #[must_use]
    pub fn contract(&self) -> &DomainContract {
        &self.contract
    }

    /// Return the canonical interface reference.
    #[must_use]
    pub fn reference(&self) -> &DomainInterfaceRef {
        self.contract.interface()
    }

    /// Return the interface's declared contract version.
    #[must_use]
    pub const fn version(&self) -> DomainContractVersion {
        self.contract.version()
    }

    /// Return capability requirements in declaration order.
    #[must_use]
    pub fn capabilities(&self) -> &[CapabilityRequirement] {
        &self.capabilities
    }

    /// Return declared effects in declaration order.
    #[must_use]
    pub fn effects(&self) -> &[EffectRef] {
        &self.effects
    }

    /// Return declared completion behavior.
    #[must_use]
    pub const fn completion(&self) -> CompletionBehavior {
        self.completion
    }

    /// Return declared failure behavior.
    #[must_use]
    pub const fn failure(&self) -> FailureBehavior {
        self.failure
    }

    /// Return the version compatibility policy.
    #[must_use]
    pub const fn version_policy(&self) -> VersionCompatibilityPolicy {
        self.version_policy
    }

    /// Return portability metadata.
    #[must_use]
    pub const fn portability(&self) -> &PortabilityDeclaration {
        &self.portability
    }

    /// Revalidate this interface after deserialization or external input.
    ///
    /// Deserialization alone must never be treated as validation.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.contract.validate()?;
        self.portability.validate()?;

        let canonical_domain = DomainId::new(
            self.contract.interface().domain(),
        )
        .map_err(domain_id_error)?;

        if canonical_domain != self.domain {
            return Err(invalid_contract(
                "interface domain identity does not match its contract",
            ));
        }

        validate_unique_capabilities(&self.capabilities)?;
        validate_unique_effects(&self.effects)?;

        for capability in &self.capabilities {
            capability.validate()?;
        }

        for effect in &self.effects {
            effect.validate()?;
        }

        Ok(())
    }

    /// Validate the interface under explicit operational policies.
    ///
    /// The metadata budget accounts for the underlying contract and the
    /// additional interface metadata. It is an accounting estimate, not an
    /// exact heap-size or serialized-size measurement.
    pub fn validate_with_config(
        &self,
        config: &HybridConfig,
    ) -> Result<(), HybridError> {
        self.validate()?;

        let mut bytes = self.contract.estimated_metadata_bytes()?;

        for capability in &self.capabilities {
            add_metadata_bytes(&mut bytes, capability.capability.len())?;

            if let Some(rationale) = &capability.rationale {
                add_metadata_bytes(&mut bytes, rationale.len())?;
            }
        }

        for effect in &self.effects {
            add_metadata_bytes(&mut bytes, effect.identifier.len())?;
        }

        if let Some(rationale) = &self.portability.rationale {
            add_metadata_bytes(&mut bytes, rationale.len())?;
        }

        config
            .check_budget(BudgetKind::MetadataBytes, bytes)
            .map_err(|error| error.in_component("hybrid domain interface"))
    }

    /// Check whether an offered version is eligible for further compatibility
    /// analysis under this interface's declared policy.
    ///
    /// A successful result does not prove that the interface is compatible.
    /// Registry, adapter and semantic validation remain necessary.
    #[must_use]
    pub const fn permits_version_analysis(
        &self,
        requested: DomainContractVersion,
        offered: DomainContractVersion,
    ) -> bool {
        self.version_policy
            .permits_compatibility_analysis(requested, offered)
    }

    /// Return the required capability identifiers.
    ///
    /// The returned iterator preserves declaration order. It does not
    /// resolve capability availability.
    pub fn required_capabilities(
        &self,
    ) -> impl Iterator<Item = &str> {
        self.capabilities
            .iter()
            .filter(|item| item.is_required())
            .map(|item| item.capability())
    }
}

fn validate_unique_capabilities(
    capabilities: &[CapabilityRequirement],
) -> Result<(), HybridError> {
    for (index, capability) in capabilities.iter().enumerate() {
        if capabilities[..index]
            .iter()
            .any(|previous| previous.capability == capability.capability)
        {
            return Err(invalid_contract(format!(
                "duplicate capability requirement: {}",
                capability.capability
            )));
        }
    }

    Ok(())
}

fn validate_unique_effects(
    effects: &[EffectRef],
) -> Result<(), HybridError> {
    for (index, effect) in effects.iter().enumerate() {
        if effects[..index]
            .iter()
            .any(|previous| previous.identifier == effect.identifier)
        {
            return Err(invalid_contract(format!(
                "duplicate effect reference: {}",
                effect.identifier
            )));
        }
    }

    Ok(())
}

fn validate_namespaced_identifier(
    kind: &str,
    value: &str,
) -> Result<(), HybridError> {
    DomainId::new(value).map(|_| ()).map_err(|error| {
        invalid_contract(format!(
            "invalid {kind} identifier: {error}"
        ))
    })
}

fn validate_nonempty_text(
    kind: &str,
    value: &str,
) -> Result<(), HybridError> {
    if value.trim().is_empty() {
        return Err(invalid_contract(format!(
            "{kind} must not be empty or whitespace-only"
        )));
    }

    Ok(())
}

fn domain_id_error(error: DomainIdError) -> HybridError {
    invalid_contract(format!("invalid interface domain identifier: {error}"))
}

fn invalid_contract(message: impl Into<String>) -> HybridError {
    HybridError::new(HybridErrorCode::InvalidContract, message)
        .in_component("hybrid domain interface")
}

fn add_metadata_bytes(
    total: &mut u128,
    amount: usize,
) -> Result<(), HybridError> {
    *total = total
        .checked_add(amount as u128)
        .ok_or_else(|| {
            invalid_contract("interface metadata byte estimate overflowed")
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hybrid::contract::{
        DependencyDeclaration,
        DomainContract,
        DomainInterfaceRef,
        InputExpectation,
        OutputGuarantee,
        OwnershipExpectation,
        SemanticGuarantee,
        GuaranteeStrength,
    };
    use crate::hybrid::version::DomainContractVersion;

    fn sample_contract() -> DomainContract {
        let reference = DomainInterfaceRef::new(
            "quantum",
            "execute",
            DomainContractVersion::new(1, 0, 0),
        )
        .expect("valid interface reference");

        let input = InputExpectation::new(
            "input",
            "zamani.value",
            OwnershipExpectation::Borrowed,
            true,
        )
        .expect("valid input");

        let output = OutputGuarantee::new(
            "result",
            "zamani.result",
            OwnershipExpectation::Owned,
            vec![
                SemanticGuarantee::new(
                    "quantum.measurement.result",
                    GuaranteeStrength::Required,
                    None,
                )
                .expect("valid guarantee"),
            ],
        )
        .expect("valid output");

        DomainContract::new(
            reference,
            vec![input],
            vec![output],
            Vec::<DependencyDeclaration>::new(),
            Vec::<SemanticGuarantee>::new(),
        )
        .expect("valid contract")
    }

    fn sample_interface() -> DomainInterface {
        DomainInterface::new(
            sample_contract(),
            vec![
                CapabilityRequirement::new(
                    "quantum.measurement",
                    CapabilityStrength::Required,
                    Some("Measurement is part of this operation".to_owned()),
                )
                .expect("valid capability"),
            ],
            vec![
                EffectRef::new("quantum.measurement")
                    .expect("valid effect"),
            ],
            CompletionBehavior::ExplicitCompletionContract,
            FailureBehavior::Propagate,
            VersionCompatibilityPolicy::Exact,
            PortabilityDeclaration::new(
                PortabilityLevel::CapabilityConditional,
                Some("Requires a compatible quantum implementation".to_owned()),
            )
            .expect("valid portability"),
        )
        .expect("valid domain interface")
    }

    #[test]
    fn preserves_canonical_contract_identity() {
        let interface = sample_interface();

        assert_eq!(interface.domain().as_str(), "quantum");
        assert_eq!(interface.reference().interface(), "execute");
        assert_eq!(interface.version(), DomainContractVersion::new(1, 0, 0));
    }

    #[test]
    fn exposes_required_capabilities_without_claiming_availability() {
        let interface = sample_interface();

        let required: Vec<_> =
            interface.required_capabilities().collect();

        assert_eq!(required, vec!["quantum.measurement"]);
    }

    #[test]
    fn rejects_invalid_capability_identifiers() {
        assert!(
            CapabilityRequirement::new(
                "Quantum Measurement",
                CapabilityStrength::Required,
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_duplicate_capability_requirements() {
        let result = DomainInterface::new(
            sample_contract(),
            vec![
                CapabilityRequirement::new(
                    "quantum.measurement",
                    CapabilityStrength::Required,
                    None,
                )
                .expect("valid capability"),
                CapabilityRequirement::new(
                    "quantum.measurement",
                    CapabilityStrength::Advisory,
                    None,
                )
                .expect("valid capability"),
            ],
            Vec::new(),
            CompletionBehavior::CompleteOnReturn,
            FailureBehavior::Propagate,
            VersionCompatibilityPolicy::Exact,
            PortabilityDeclaration::new(
                PortabilityLevel::TargetIndependent,
                None,
            )
            .expect("valid portability"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn exact_version_policy_does_not_accept_a_different_version() {
        let interface = sample_interface();

        assert!(interface.permits_version_analysis(
            DomainContractVersion::new(1, 0, 0),
            DomainContractVersion::new(1, 0, 0),
        ));

        assert!(!interface.permits_version_analysis(
            DomainContractVersion::new(1, 0, 0),
            DomainContractVersion::new(1, 1, 0),
        ));
    }

    #[test]
    fn same_major_policy_only_permits_further_analysis() {
        let policy = VersionCompatibilityPolicy::SameMajorRequiresValidation;

        assert!(policy.permits_compatibility_analysis(
            DomainContractVersion::new(1, 0, 0),
            DomainContractVersion::new(1, 9, 0),
        ));

        assert!(!policy.permits_compatibility_analysis(
            DomainContractVersion::new(1, 0, 0),
            DomainContractVersion::new(2, 0, 0),
        ));
    }

    #[test]
    fn configuration_validation_uses_the_explicit_metadata_budget() {
        let interface = sample_interface();

        assert!(interface
            .validate_with_config(&HybridConfig::default())
            .is_ok());
    }
}
