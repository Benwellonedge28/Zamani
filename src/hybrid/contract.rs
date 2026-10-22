//! # Zamani Hybrid Computing — Cross-Domain Contracts
//!
//! Path: `src/hybrid/contract.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only. Unsafe code is forbidden.
//!
//! ## Purpose
//!
//! Define the domain-neutral contracts that let independently implemented
//! computational domains participate in one Zamani hybrid computation.
//!
//! A contract describes the interface offered by a domain, its expected
//! inputs, its promised outputs, its dependencies, and its declared semantic
//! guarantees. It does not implement the operations described by that
//! contract.
//!
//! ## Canonical ownership
//!
//! This module owns:
//! - Domain interface references.
//! - Input expectations and output guarantees.
//! - Explicit interface dependencies.
//! - Namespaced semantic-guarantee declarations.
//! - Structural contract validation.
//!
//! This module does not own:
//! - Source syntax, parsing, ASTs, name resolution, or type checking.
//! - Classical, quantum, HDL, tensor, or other domain operation semantics.
//! - A universal IR or a competing quantum IR.
//! - Physical devices, allocations, scheduling, routing, QEC, or calibration.
//! - Runtime handles, vendor SDKs, network transports, or execution sessions.
//! - Domain discovery, contract registry storage, or schema migration.
//!
//! Existing canonical semantic types and domain IR identities remain
//! authoritative. The references in this module identify those definitions;
//! they do not replace them.
//!
//! ## Portability and scalability
//!
//! No hardware-size ceilings or fixed domain inventories are defined here.
//! Domain and interface names are extensible identifiers, not a closed enum.
//! Collections grow according to the available allocator and explicit
//! operational policy.
//!
//! An absent operational budget means this module imposes no limit for that
//! budget category. It does not mean that the host has infinite memory or
//! computation time.
//!
//! ## Semantic safety
//!
//! Unknown required guarantees or dependencies must not be silently ignored.
//! Structural validation does not prove that a domain implementation fulfils
//! its promises. That requires a trusted registry, conformance evidence, and
//! downstream semantic and feasibility validation.
//!
//! ## Integration
//!
//! - Uses `super::errors::{HybridError, HybridErrorCode}` for failures.
//! - Uses `super::version::DomainContractVersion` for interface versions.
//! - Uses `super::config::{HybridConfig, BudgetKind}` for explicit policies.
//! - `domain/interface.rs` should use these contracts as its shared interface
//!   vocabulary rather than defining duplicate input/output contract types.
//! - `domain/registry.rs` should resolve interface references and establish
//!   whether required guarantees are supported.
//! - `graph/verify.rs` should validate dependency endpoints and graph-level
//!   rules; this file validates each contract in isolation.
//! - `data/exchange.rs` should interpret port compatibility using canonical
//!   semantic type and ownership information.
//! - `planning/` should consume validated requirements without modifying
//!   contract meaning.
//! - `interchange/` may serialize these declarations using the canonical
//!   versioned representation.
//!
//! This module intentionally does not import AST, ZUIR, quantum IR, compiler,
//! runtime, or hardware modules.
//!
//! ## Completion criteria
//!
//! - All identifiers are checked for structural validity.
//! - Duplicate port and guarantee declarations are rejected.
//! - Required and optional dependencies remain distinguishable.
//! - Unknown required semantics cannot be treated as satisfied.
//! - Explicit metadata budgets are checked when supplied.
//! - No unsafe code or arbitrary hardware limits are introduced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use serde::{Deserialize, Serialize};

use super::config::{BudgetKind, HybridConfig};
use super::errors::{HybridError, HybridErrorCode};
use super::version::DomainContractVersion;

/// A stable, domain-neutral reference to a domain interface.
///
/// `domain` and `interface` are extensible, namespaced identifiers. They are
/// deliberately strings rather than a closed enum so that adding a new
/// computational domain does not require modifying this module.
///
/// `version` identifies the contract version, not the language version,
/// compiler release, hardware firmware, or serialization format.
///
/// A reference is only an identifier. It does not prove that the interface
/// is registered, available, compatible, or executable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DomainInterfaceRef {
    domain: String,
    interface: String,
    version: DomainContractVersion,
}

impl DomainInterfaceRef {
    /// Constructs an interface reference.
    ///
    /// This constructor validates identifier structure but does not resolve
    /// the interface against a registry.
    pub fn new(
        domain: impl Into<String>,
        interface: impl Into<String>,
        version: DomainContractVersion,
    ) -> Result<Self, HybridError> {
        let value = Self {
            domain: domain.into(),
            interface: interface.into(),
            version,
        };

        validate_identifier("domain", &value.domain)?;
        validate_identifier("interface", &value.interface)?;

        Ok(value)
    }

    /// Returns the domain identifier.
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Returns the interface identifier.
    #[must_use]
    pub fn interface(&self) -> &str {
        &self.interface
    }

    /// Returns the declared contract version.
    #[must_use]
    pub const fn version(&self) -> DomainContractVersion {
        self.version
    }

    /// Returns the stable logical key, excluding the version.
    ///
    /// This method returns a tuple rather than concatenating strings, avoiding
    /// ambiguous encodings when identifiers contain punctuation.
    #[must_use]
    pub fn logical_key(&self) -> (&str, &str) {
        (&self.domain, &self.interface)
    }

    /// Revalidates this value after deserialization or external construction.
    ///
    /// Deserialization is not a substitute for validating untrusted input.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_identifier("domain", &self.domain)?;
        validate_identifier("interface", &self.interface)
    }
}

/// The ownership expectation associated with a contract port.
///
/// This is a declaration of the boundary contract. The canonical semantic
/// model remains responsible for the actual ownership and lifetime rules.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipExpectation {
    /// The consumer may use a value only while its owner guarantees validity.
    Borrowed,

    /// The consumer receives ownership under the canonical language rules.
    Owned,

    /// Ownership and lifetime are defined by an external canonical contract.
    External,
}

/// Describes an input expected by a domain interface.
///
/// `semantic_type` is a stable reference to a type owned by the canonical
/// semantic/type system. This module does not parse it or define its meaning.
///
/// `required` indicates whether the input is necessary for this interface
/// contract. It does not mean that an optional input may be ignored if its
/// presence changes the semantics of a particular invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputExpectation {
    name: String,
    semantic_type: String,
    ownership: OwnershipExpectation,
    required: bool,
}

impl InputExpectation {
    /// Creates an input expectation.
    pub fn new(
        name: impl Into<String>,
        semantic_type: impl Into<String>,
        ownership: OwnershipExpectation,
        required: bool,
    ) -> Result<Self, HybridError> {
        let value = Self {
            name: name.into(),
            semantic_type: semantic_type.into(),
            ownership,
            required,
        };

        value.validate()?;
        Ok(value)
    }

    /// Returns the input port name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical semantic type reference.
    #[must_use]
    pub fn semantic_type(&self) -> &str {
        &self.semantic_type
    }

    /// Returns the declared ownership expectation.
    #[must_use]
    pub const fn ownership(&self) -> OwnershipExpectation {
        self.ownership
    }

    /// Returns whether this input is required by the interface.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    /// Validates this declaration independently.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_identifier("input port", &self.name)?;
        validate_identifier("semantic type reference", &self.semantic_type)
    }
}

/// Describes an output promised by a domain interface.
///
/// `semantic_type` references the canonical semantic type system.
/// Guarantees state what the implementation promises; they do not prove
/// that a particular implementation actually fulfils those promises.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputGuarantee {
    name: String,
    semantic_type: String,
    ownership: OwnershipExpectation,
    guarantees: Vec<SemanticGuarantee>,
}

impl OutputGuarantee {
    /// Creates an output guarantee.
    pub fn new(
        name: impl Into<String>,
        semantic_type: impl Into<String>,
        ownership: OwnershipExpectation,
        guarantees: Vec<SemanticGuarantee>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            name: name.into(),
            semantic_type: semantic_type.into(),
            ownership,
            guarantees,
        };

        value.validate()?;
        Ok(value)
    }

    /// Returns the output port name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical semantic type reference.
    #[must_use]
    pub fn semantic_type(&self) -> &str {
        &self.semantic_type
    }

    /// Returns the declared ownership expectation.
    #[must_use]
    pub const fn ownership(&self) -> OwnershipExpectation {
        self.ownership
    }

    /// Returns the semantic guarantees attached to this output.
    #[must_use]
    pub fn guarantees(&self) -> &[SemanticGuarantee] {
        &self.guarantees
    }

    /// Validates this declaration independently.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_identifier("output port", &self.name)?;
        validate_identifier("semantic type reference", &self.semantic_type)?;
        validate_unique_guarantees(&self.guarantees)
    }
}

/// The strength of a declared semantic guarantee.
///
/// Required guarantees must be understood and verified by the consumer or
/// trusted contract registry. Advisory guarantees can inform implementation
/// choices but must not alter program meaning.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum GuaranteeStrength {
    /// Required for a conforming implementation of this interface.
    Required,

    /// Informational or optimization guidance only.
    Advisory,
}

/// A namespaced semantic guarantee.
///
/// `key` is extensible to allow future domain guarantees without editing this
/// module. Consumers must not assume an unknown key is satisfied. In
/// particular, an unknown required guarantee makes compatibility unresolved
/// or invalid until a trusted registry or extension defines its semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticGuarantee {
    key: String,
    strength: GuaranteeStrength,
    description: Option<String>,
}

impl SemanticGuarantee {
    /// Creates a semantic guarantee.
    pub fn new(
        key: impl Into<String>,
        strength: GuaranteeStrength,
        description: Option<String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            key: key.into(),
            strength,
            description,
        };

        value.validate()?;
        Ok(value)
    }

    /// Returns the namespaced guarantee key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the guarantee strength.
    #[must_use]
    pub const fn strength(&self) -> GuaranteeStrength {
        self.strength
    }

    /// Returns the optional explanatory description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns whether the guarantee is mandatory.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        matches!(self.strength, GuaranteeStrength::Required)
    }

    /// Validates this declaration independently.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_identifier("semantic guarantee key", &self.key)?;

        if let Some(description) = &self.description {
            validate_text("semantic guarantee description", description)?;
        }

        Ok(())
    }
}

/// The semantic relationship declared by an interface dependency.
///
/// The exact execution and ordering semantics are resolved by the graph,
/// control-flow, and synchronization layers. This enum does not impose a
/// global DAG requirement or declare every dependency to be synchronous.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum DependencyKind {
    /// The dependent interface requires a result produced by another
    /// interface.
    Data,

    /// The dependent interface requires a control decision from another
    /// interface.
    Control,

    /// The dependent interface requires another interface to complete.
    Completion,

    /// The dependent interface requires an explicit ordering relationship.
    Ordering,

    /// The dependency participates in a feedback relationship.
    Feedback,

    /// The dependency crosses a computational-domain boundary.
    DomainBoundary,
}

/// A dependency declared by a domain interface.
///
/// `target` names the required interface. The graph verifier later resolves
/// the corresponding invocation or dependency endpoint.
///
/// A required dependency that cannot be resolved must not be silently dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyDeclaration {
    target: DomainInterfaceRef,
    kind: DependencyKind,
    required: bool,
}

impl DependencyDeclaration {
    /// Creates a dependency declaration.
    pub fn new(
        target: DomainInterfaceRef,
        kind: DependencyKind,
        required: bool,
    ) -> Result<Self, HybridError> {
        target.validate()?;

        Ok(Self {
            target,
            kind,
            required,
        })
    }

    /// Returns the referenced interface.
    #[must_use]
    pub fn target(&self) -> &DomainInterfaceRef {
        &self.target
    }

    /// Returns the declared dependency kind.
    #[must_use]
    pub const fn kind(&self) -> DependencyKind {
        self.kind
    }

    /// Returns whether the dependency is required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    /// Revalidates this declaration after deserialization.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.target.validate()
    }
}

/// The complete, domain-neutral contract for one interface.
///
/// This is metadata describing an interface, not a computational operation
/// or IR node. `inputs`, `outputs`, and `dependencies` preserve declaration
/// order as supplied. A consumer that needs canonical order must sort using
/// the documented stable keys rather than relying on registration order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainContract {
    interface: DomainInterfaceRef,
    inputs: Vec<InputExpectation>,
    outputs: Vec<OutputGuarantee>,
    dependencies: Vec<DependencyDeclaration>,
    guarantees: Vec<SemanticGuarantee>,
}

impl DomainContract {
    /// Creates a contract and validates its internal structure.
    ///
    /// Registry resolution and implementation conformance are separate checks
    /// and are intentionally not inferred by this constructor.
    pub fn new(
        interface: DomainInterfaceRef,
        inputs: Vec<InputExpectation>,
        outputs: Vec<OutputGuarantee>,
        dependencies: Vec<DependencyDeclaration>,
        guarantees: Vec<SemanticGuarantee>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            interface,
            inputs,
            outputs,
            dependencies,
            guarantees,
        };

        value.validate()?;
        Ok(value)
    }

    /// Returns the interface described by this contract.
    #[must_use]
    pub fn interface(&self) -> &DomainInterfaceRef {
        &self.interface
    }

    /// Returns declared inputs in source/declaration order.
    #[must_use]
    pub fn inputs(&self) -> &[InputExpectation] {
        &self.inputs
    }

    /// Returns declared outputs in source/declaration order.
    #[must_use]
    pub fn outputs(&self) -> &[OutputGuarantee] {
        &self.outputs
    }

    /// Returns declared dependencies in source/declaration order.
    #[must_use]
    pub fn dependencies(&self) -> &[DependencyDeclaration] {
        &self.dependencies
    }

    /// Returns interface-level semantic guarantees.
    #[must_use]
    pub fn guarantees(&self) -> &[SemanticGuarantee] {
        &self.guarantees
    }

    /// Returns the interface version.
    #[must_use]
    pub const fn version(&self) -> DomainContractVersion {
        self.interface.version()
    }

    /// Validates structural invariants without consulting a registry.
    ///
    /// This check validates names, duplicate declarations, and internal
    /// consistency. It does not prove type compatibility, guarantee
    /// implementation, target feasibility, or dependency endpoint existence.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.interface.validate()?;
        validate_unique_inputs(&self.inputs)?;
        validate_unique_outputs(&self.outputs)?;
        validate_unique_guarantees(&self.guarantees)?;

        for input in &self.inputs {
            input.validate()?;
        }

        for output in &self.outputs {
            output.validate()?;
        }

        for dependency in &self.dependencies {
            dependency.validate()?;
        }

        Ok(())
    }

    /// Validates the contract and applies any explicitly configured metadata
    /// budget.
    ///
    /// The metadata estimate is the sum of UTF-8 byte lengths in the contract.
    /// It intentionally excludes allocator overhead and serialized-format
    /// framing; interchange encoders must separately enforce encoded-byte
    /// budgets against the actual output they produce.
    pub fn validate_with_config(
        &self,
        config: &HybridConfig,
    ) -> Result<(), HybridError> {
        self.validate()?;

        let metadata_bytes = self.estimated_metadata_bytes()?;
        config
            .check_budget(BudgetKind::MetadataBytes, metadata_bytes)
            .map_err(|error| error.in_component("hybrid contract validation"))
    }

    /// Estimates the UTF-8 bytes held by contract strings.
    ///
    /// Arithmetic is checked so the estimate never silently wraps. This is
    /// an accounting estimate, not an exact heap-size measurement.
    pub fn estimated_metadata_bytes(&self) -> Result<u128, HybridError> {
        let mut total = 0_u128;

        add_bytes(&mut total, self.interface.domain().len())?;
        add_bytes(&mut total, self.interface.interface().len())?;

        for input in &self.inputs {
            add_bytes(&mut total, input.name.len())?;
            add_bytes(&mut total, input.semantic_type.len())?;
        }

        for output in &self.outputs {
            add_bytes(&mut total, output.name.len())?;
            add_bytes(&mut total, output.semantic_type.len())?;

            for guarantee in &output.guarantees {
                add_guarantee_bytes(&mut total, guarantee)?;
            }
        }

        for dependency in &self.dependencies {
            add_bytes(&mut total, dependency.target.domain().len())?;
            add_bytes(&mut total, dependency.target.interface().len())?;
        }

        for guarantee in &self.guarantees {
            add_guarantee_bytes(&mut total, guarantee)?;
        }

        Ok(total)
    }
}

fn add_guarantee_bytes(
    total: &mut u128,
    guarantee: &SemanticGuarantee,
) -> Result<(), HybridError> {
    add_bytes(total, guarantee.key.len())?;

    if let Some(description) = &guarantee.description {
        add_bytes(total, description.len())?;
    }

    Ok(())
}

fn add_bytes(total: &mut u128, bytes: usize) -> Result<(), HybridError> {
    let bytes = bytes as u128;

    *total = total.checked_add(bytes).ok_or_else(|| {
        HybridError::new(
            HybridErrorCode::InvalidContract,
            "contract metadata byte accounting overflowed",
        )
    })?;

    Ok(())
}

fn validate_unique_inputs(
    inputs: &[InputExpectation],
) -> Result<(), HybridError> {
    for (index, input) in inputs.iter().enumerate() {
        for other in inputs.iter().skip(index + 1) {
            if input.name == other.name {
                return Err(invalid_contract(format!(
                    "duplicate input port `{}`",
                    input.name
                )));
            }
        }
    }

    Ok(())
}

fn validate_unique_outputs(
    outputs: &[OutputGuarantee],
) -> Result<(), HybridError> {
    for (index, output) in outputs.iter().enumerate() {
        for other in outputs.iter().skip(index + 1) {
            if output.name == other.name {
                return Err(invalid_contract(format!(
                    "duplicate output port `{}`",
                    output.name
                )));
            }
        }
    }

    Ok(())
}

fn validate_unique_guarantees(
    guarantees: &[SemanticGuarantee],
) -> Result<(), HybridError> {
    for guarantee in guarantees {
        guarantee.validate()?;
    }

    for (index, guarantee) in guarantees.iter().enumerate() {
        for other in guarantees.iter().skip(index + 1) {
            if guarantee.key == other.key {
                return Err(invalid_contract(format!(
                    "duplicate semantic guarantee `{}`",
                    guarantee.key
                )));
            }
        }
    }

    Ok(())
}

fn validate_identifier(
    kind: &str,
    value: &str,
) -> Result<(), HybridError> {
    if value.trim().is_empty() {
        return Err(invalid_contract(format!(
            "{kind} identifier must not be empty"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_contract(format!(
            "{kind} identifier must not contain control characters"
        )));
    }

    Ok(())
}

fn validate_text(kind: &str, value: &str) -> Result<(), HybridError> {
    if value.chars().any(char::is_control) {
        return Err(invalid_contract(format!(
            "{kind} must not contain control characters"
        )));
    }

    Ok(())
}

fn invalid_contract(message: impl Into<String>) -> HybridError {
    HybridError::new(HybridErrorCode::InvalidContract, message)
        .in_component("hybrid contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::config::OperationalBudgets;

    fn interface_ref() -> DomainInterfaceRef {
        DomainInterfaceRef::new(
            "quantum",
            "execute",
            DomainContractVersion::new(1, 0, 0),
        )
        .expect("test interface reference should be valid")
    }

    fn simple_contract() -> DomainContract {
        let input = InputExpectation::new(
            "state",
            "zamani.semantic.quantum_state",
            OwnershipExpectation::Borrowed,
            true,
        )
        .expect("test input should be valid");

        let guarantee = SemanticGuarantee::new(
            "zamani.quantum.measurement_result",
            GuaranteeStrength::Required,
            Some("Produces a result according to the canonical quantum semantics".to_owned()),
        )
        .expect("test guarantee should be valid");

        let output = OutputGuarantee::new(
            "result",
            "zamani.semantic.measurement_result",
            OwnershipExpectation::Owned,
            vec![guarantee],
        )
        .expect("test output should be valid");

        DomainContract::new(
            interface_ref(),
            vec![input],
            vec![output],
            Vec::new(),
            Vec::new(),
        )
        .expect("test contract should be valid")
    }

    #[test]
    fn valid_contract_preserves_its_declarations() {
        let contract = simple_contract();

        assert_eq!(contract.interface().domain(), "quantum");
        assert_eq!(contract.interface().interface(), "execute");
        assert_eq!(contract.inputs().len(), 1);
        assert_eq!(contract.outputs().len(), 1);
        assert!(contract.inputs()[0].is_required());
        assert_eq!(
            contract.outputs()[0].ownership(),
            OwnershipExpectation::Owned
        );
    }

    #[test]
    fn empty_identifiers_are_rejected() {
        assert!(DomainInterfaceRef::new(
            "quantum",
            " ",
            DomainContractVersion::new(1, 0, 0),
        )
        .is_err());

        assert!(InputExpectation::new(
            "",
            "zamani.semantic.integer",
            OwnershipExpectation::Borrowed,
            true,
        )
        .is_err());
    }

    #[test]
    fn control_characters_in_identifiers_are_rejected() {
        assert!(DomainInterfaceRef::new(
            "quantum\nother",
            "execute",
            DomainContractVersion::new(1, 0, 0),
        )
        .is_err());
    }

    #[test]
    fn duplicate_input_ports_are_rejected() {
        let first = InputExpectation::new(
            "value",
            "zamani.semantic.integer",
            OwnershipExpectation::Borrowed,
            true,
        )
        .expect("first input should be valid");

        let second = InputExpectation::new(
            "value",
            "zamani.semantic.float",
            OwnershipExpectation::Borrowed,
            true,
        )
        .expect("second input should be valid in isolation");

        let result = DomainContract::new(
            interface_ref(),
            vec![first, second],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_output_ports_are_rejected() {
        let first = OutputGuarantee::new(
            "result",
            "zamani.semantic.integer",
            OwnershipExpectation::Owned,
            Vec::new(),
        )
        .expect("first output should be valid");

        let second = OutputGuarantee::new(
            "result",
            "zamani.semantic.float",
            OwnershipExpectation::Owned,
            Vec::new(),
        )
        .expect("second output should be valid in isolation");

        let result = DomainContract::new(
            interface_ref(),
            Vec::new(),
            vec![first, second],
            Vec::new(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_guarantee_keys_are_rejected() {
        let first = SemanticGuarantee::new(
            "zamani.effects.pure",
            GuaranteeStrength::Required,
            None,
        )
        .expect("first guarantee should be valid");

        let second = SemanticGuarantee::new(
            "zamani.effects.pure",
            GuaranteeStrength::Advisory,
            None,
        )
        .expect("second guarantee should be valid in isolation");

        let result = DomainContract::new(
            interface_ref(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![first, second],
        );

        assert!(result.is_err());
    }

    #[test]
    fn required_and_advisory_guarantees_remain_distinct() {
        let required = SemanticGuarantee::new(
            "zamani.execution.deterministic",
            GuaranteeStrength::Required,
            None,
        )
        .expect("required guarantee should be valid");

        let advisory = SemanticGuarantee::new(
            "zamani.execution.low_latency",
            GuaranteeStrength::Advisory,
            None,
        )
        .expect("advisory guarantee should be valid");

        assert!(required.is_required());
        assert!(!advisory.is_required());
    }

    #[test]
    fn optional_and_required_dependencies_remain_distinct() {
        let required = DependencyDeclaration::new(
            interface_ref(),
            DependencyKind::Completion,
            true,
        )
        .expect("required dependency should be valid");

        let optional = DependencyDeclaration::new(
            interface_ref(),
            DependencyKind::Feedback,
            false,
        )
        .expect("optional dependency should be valid");

        assert!(required.is_required());
        assert!(!optional.is_required());
        assert_eq!(required.kind(), DependencyKind::Completion);
        assert_eq!(optional.kind(), DependencyKind::Feedback);
    }

    #[test]
    fn explicit_metadata_budget_is_enforced() {
        let contract = simple_contract();
        let mut config = HybridConfig::default();

        config.budgets = OperationalBudgets {
            max_metadata_bytes: Some(0),
            ..OperationalBudgets::default()
        };

        let error = contract
            .validate_with_config(&config)
            .expect_err("zero metadata budget should reject non-empty metadata");

        assert_eq!(error.code(), HybridErrorCode::BudgetExhausted);
    }

    #[test]
    fn absent_metadata_budget_does_not_impose_a_limit() {
        let contract = simple_contract();
        let config = HybridConfig::default();

        assert!(config.budgets.is_unbounded());
        assert!(contract.validate_with_config(&config).is_ok());
    }

    #[test]
    fn structural_validation_does_not_claim_registry_resolution() {
        let contract = DomainContract::new(
            DomainInterfaceRef::new(
                "future.custom.domain",
                "future.operation",
                DomainContractVersion::new(8, 2, 1),
            )
            .expect("extensible interface reference should be valid"),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect("structurally valid future contract should be accepted");

        assert_eq!(contract.interface().domain(), "future.custom.domain");
        assert_eq!(contract.interface().version().major(), 8);
    }

    #[test]
    fn metadata_accounting_is_repeatable() {
        let contract = simple_contract();

        assert_eq!(
            contract.estimated_metadata_bytes(),
            contract.estimated_metadata_bytes()
        );
    }
}