
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Public API
//!
//! Path: `src/hybrid/api.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide a small, stable public entry point for configuring and validating
//! the domain-neutral contracts used by Zamani hybrid computation.
//!
//! The API supports the POCO-REAF architecture:
//!
//! `Program Once -> Compile Once -> Run Everywhere -> Anywhere -> Forever`
//!
//! subject to semantic preservation, compatible domain implementations,
//! required capabilities, available resources, explicit policies, and target
//! feasibility.
//!
//! This module is a facade over canonical hybrid types. It does not create
//! another AST, semantic model, universal IR, quantum IR, compiler pipeline,
//! scheduler, or runtime.
//!
//! ## Ownership
//!
//! This module owns:
//! - The public entry point for hybrid contract validation.
//! - Explicit configuration of API operations.
//! - Validation reports for a collection of domain contracts.
//! - Aggregate operational-budget checks for contract collections.
//!
//! This module does not own:
//! - Source parsing, AST construction, name resolution, or type checking.
//! - Domain-specific operation semantics.
//! - Domain registration or implementation-conformance evidence.
//! - Hybrid graph construction or graph verification.
//! - Physical allocation, scheduling, routing, QEC, or code generation.
//! - Runtime execution, device discovery, or vendor integration.
//! - Serialization or schema migration.
//!
//! ## Integration contract
//!
//! - `super::config::HybridConfig` owns operational and validation policies.
//! - `super::config::BudgetKind` identifies operational budgets.
//! - `super::contract::DomainContract` owns individual contract structure.
//! - `super::contract::DomainInterfaceRef` identifies a domain interface.
//! - `super::errors::HybridError` is the canonical failure type.
//! - `super::errors::HybridErrorCode` supplies stable machine-readable codes.
//! - `super::version::DomainContractVersion` identifies contract versions.
//!
//! Domain registry resolution, graph-level validation, semantic type
//! compatibility, and target feasibility must be implemented by their
//! respective owners. A successful result here does not claim that a domain
//! exists, that an implementation fulfils its guarantees, or that a target
//! can execute the computation.
//!
//! ## Scalability
//!
//! This API defines no universal maximum number of contracts, domains,
//! inputs, outputs, or dependencies. Collection sizes are constrained by
//! available resources and by optional operational policies supplied through
//! `HybridConfig`.
//!
//! Aggregate accounting uses checked arithmetic. Arithmetic overflow returns
//! an explicit error instead of wrapping.
//!
//! An unspecified budget does not mean that memory or computation is
//! physically infinite.
//!
//! ## Determinism
//!
//! Validation does not change declaration order or mutate contract meaning.
//! Reports contain counts and byte estimates, not hash-map iteration order.
//! The same contracts and configuration produce the same validation result.
//!
//! ## Completion boundary
//!
//! This file is complete when its public types and functions are stable,
//! covered by tests, and integrated through `src/hybrid/mod.rs`.
//! It must not be expanded into a catch-all implementation module.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::config::{BudgetKind, HybridConfig};
use super::contract::{DomainContract, DomainInterfaceRef};
use super::errors::{HybridError, HybridErrorCode};

/// Public entry point for hybrid contract validation.
///
/// `HybridApi` owns a configuration value for the operations invoked through
/// this facade. It does not own a global registry or mutable process-wide
/// state.
///
/// Clone the API when an independent caller needs the same explicit policy.
/// The configuration is immutable through this API after construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridApi {
    config: HybridConfig,
}

impl Default for HybridApi {
    /// Creates an API using the documented default configuration.
    ///
    /// Default budgets are unspecified unless `HybridConfig` says otherwise.
    fn default() -> Self {
        Self::new(HybridConfig::default())
    }
}

impl HybridApi {
    /// Creates the public API with an explicit configuration.
    ///
    /// The configuration is retained as supplied. Operations enforce the
    /// budgets relevant to the work they perform.
    #[must_use]
    pub fn new(config: HybridConfig) -> Self {
        Self { config }
    }

    /// Returns the configuration used by this API.
    #[must_use]
    pub const fn config(&self) -> &HybridConfig {
        &self.config
    }

    /// Validates one domain contract using this API's configuration.
    ///
    /// This verifies the contract's structural invariants and configured
    /// metadata budget. It does not resolve dependencies against a registry,
    /// prove that guarantees are implemented, or establish target feasibility.
    pub fn validate_contract(
        &self,
        contract: &DomainContract,
    ) -> Result<ContractValidationReport, HybridError> {
        contract
            .validate_with_config(&self.config)
            .map_err(|error| error.in_component("hybrid public API"))?;

        Ok(ContractValidationReport {
            contract_count: 1,
            input_count: contract.inputs().len(),
            output_count: contract.outputs().len(),
            dependency_count: contract.dependencies().len(),
            guarantee_count: contract.guarantees().len()
                + contract
                    .outputs()
                    .iter()
                    .map(|output| output.guarantees().len())
                    .sum::<usize>(),
            estimated_metadata_bytes: contract.estimated_metadata_bytes()?,
        })
    }

    /// Validates a collection of domain contracts as one API operation.
    ///
    /// The operation:
    ///
    /// 1. Validates each contract's structure.
    /// 2. Rejects duplicate interface identities at the same version.
    /// 3. Checks the configured domain-instance budget.
    /// 4. Checks the aggregate metadata-byte budget.
    ///
    /// Multiple versions of the same logical interface may coexist. They are
    /// distinct contract identities; determining whether a particular version
    /// satisfies a dependency belongs to the registry and compatibility layer.
    ///
    /// This method deliberately does not resolve dependencies. A declaration
    /// can reference an interface that is not present in this collection.
    /// Resolution is a separate graph/registry responsibility.
    pub fn validate_contracts(
        &self,
        contracts: &[DomainContract],
    ) -> Result<ContractValidationReport, HybridError> {
        self.config
            .check_budget(BudgetKind::DomainInstances, contracts.len() as u128)
            .map_err(|error| error.in_component("hybrid public API"))?;

        let mut identities = BTreeSet::new();
        let mut input_count = 0_usize;
        let mut output_count = 0_usize;
        let mut dependency_count = 0_usize;
        let mut guarantee_count = 0_usize;
        let mut estimated_metadata_bytes = 0_u128;

        for contract in contracts {
            contract
                .validate()
                .map_err(|error| error.in_component("hybrid public API"))?;

            let interface = contract.interface();
            let identity = (
                interface.domain().to_owned(),
                interface.interface().to_owned(),
                interface.version(),
            );

            if !identities.insert(identity) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "duplicate domain interface contract `{}` / `{}` at version {}",
                        interface.domain(),
                        interface.interface(),
                        interface.version()
                    ),
                )
                .in_component("hybrid public API"));
            }

            input_count = checked_add_count(
                input_count,
                contract.inputs().len(),
                "input",
            )?;

            output_count = checked_add_count(
                output_count,
                contract.outputs().len(),
                "output",
            )?;

            dependency_count = checked_add_count(
                dependency_count,
                contract.dependencies().len(),
                "dependency",
            )?;

            let output_guarantees = contract
                .outputs()
                .iter()
                .try_fold(0_usize, |total, output| {
                    total.checked_add(output.guarantees().len())
                })
                .ok_or_else(|| count_overflow("output guarantee"))?;

            guarantee_count = checked_add_count(
                guarantee_count,
                contract.guarantees().len(),
                "interface guarantee",
            )?;

            guarantee_count = checked_add_count(
                guarantee_count,
                output_guarantees,
                "output guarantee",
            )?;

            let contract_bytes = contract.estimated_metadata_bytes()?;

            estimated_metadata_bytes = estimated_metadata_bytes
                .checked_add(contract_bytes)
                .ok_or_else(|| {
                    HybridError::new(
                        HybridErrorCode::InvalidContract,
                        "aggregate contract metadata accounting overflowed",
                    )
                    .in_component("hybrid public API")
                })?;

            // Enforce the configured budget against the collection total,
            // not only against individual contracts.
            self.config
                .check_budget(
                    BudgetKind::MetadataBytes,
                    estimated_metadata_bytes,
                )
                .map_err(|error| error.in_component("hybrid public API"))?;
        }

        Ok(ContractValidationReport {
            contract_count: contracts.len(),
            input_count,
            output_count,
            dependency_count,
            guarantee_count,
            estimated_metadata_bytes,
        })
    }

    /// Validates an individual interface reference structurally.
    ///
    /// This is not registry lookup: a well-formed reference can still refer
    /// to an unavailable or incompatible interface.
    pub fn validate_interface_ref(
        &self,
        interface: &DomainInterfaceRef,
    ) -> Result<(), HybridError> {
        interface
            .validate()
            .map_err(|error| error.in_component("hybrid public API"))
    }
}

/// Summary of successful structural validation.
///
/// The byte count is an estimate of contract string metadata as defined by
/// `DomainContract::estimated_metadata_bytes`. It is not a heap-size estimate
/// and does not include serialized framing, allocator overhead, or unrelated
/// structures.
///
/// Counts use `usize` because they describe collections already represented
/// in the current process. They are not hardware capacities or language-level
/// limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractValidationReport {
    /// Number of validated interface contracts.
    pub contract_count: usize,

    /// Total number of declared input ports.
    pub input_count: usize,

    /// Total number of declared output ports.
    pub output_count: usize,

    /// Total number of declared interface dependencies.
    pub dependency_count: usize,

    /// Total number of interface-level and output-level guarantees.
    pub guarantee_count: usize,

    /// Estimated aggregate UTF-8 bytes held by contract strings.
    pub estimated_metadata_bytes: u128,
}

impl ContractValidationReport {
    /// Returns whether the validated collection is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.contract_count == 0
    }
}

fn checked_add_count(
    current: usize,
    additional: usize,
    kind: &str,
) -> Result<usize, HybridError> {
    current.checked_add(additional).ok_or_else(|| count_overflow(kind))
}

fn count_overflow(kind: &str) -> HybridError {
    HybridError::new(
        HybridErrorCode::InvalidContract,
        format!("aggregate {kind} count overflowed"),
    )
    .in_component("hybrid public API")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::contract::{
        DependencyDeclaration, DependencyKind, DomainContract,
        DomainInterfaceRef, GuaranteeStrength, InputExpectation,
        OutputGuarantee, OwnershipExpectation, SemanticGuarantee,
    };
    use crate::hybrid::version::DomainContractVersion;

    fn interface(domain: &str, name: &str) -> DomainInterfaceRef {
        DomainInterfaceRef::new(
            domain,
            name,
            DomainContractVersion::new(1, 0, 0),
        )
        .expect("test interface should be structurally valid")
    }

    fn sample_contract(domain: &str, name: &str) -> DomainContract {
        let input = InputExpectation::new(
            "input",
            "core.integer",
            OwnershipExpectation::Borrowed,
            true,
        )
        .expect("test input should be valid");

        let output = OutputGuarantee::new(
            "output",
            "core.integer",
            OwnershipExpectation::Owned,
            vec![
                SemanticGuarantee::new(
                    "core.result.available",
                    GuaranteeStrength::Required,
                    None,
                )
                .expect("test guarantee should be valid"),
            ],
        )
        .expect("test output should be valid");

        DomainContract::new(
            interface(domain, name),
            vec![input],
            vec![output],
            Vec::new(),
            vec![
                SemanticGuarantee::new(
                    "core.deterministic",
                    GuaranteeStrength::Advisory,
                    None,
                )
                .expect("test guarantee should be valid"),
            ],
        )
        .expect("test contract should be valid")
    }

    #[test]
    fn validates_one_contract() {
        let api = HybridApi::default();
        let contract = sample_contract("classical", "compute");

        let report = api
            .validate_contract(&contract)
            .expect("contract should validate");

        assert_eq!(report.contract_count, 1);
        assert_eq!(report.input_count, 1);
        assert_eq!(report.output_count, 1);
        assert_eq!(report.dependency_count, 0);
        assert_eq!(report.guarantee_count, 2);
        assert!(report.estimated_metadata_bytes > 0);
    }

    #[test]
    fn validates_a_collection_of_distinct_contracts() {
        let api = HybridApi::default();
        let contracts = [
            sample_contract("classical", "compute"),
            sample_contract("quantum", "measure"),
        ];

        let report = api
            .validate_contracts(&contracts)
            .expect("distinct contracts should validate");

        assert_eq!(report.contract_count, 2);
        assert_eq!(report.input_count, 2);
        assert_eq!(report.output_count, 2);
        assert_eq!(report.guarantee_count, 4);
    }

    #[test]
    fn rejects_duplicate_interface_identity_at_same_version() {
        let api = HybridApi::default();
        let contracts = [
            sample_contract("classical", "compute"),
            sample_contract("classical", "compute"),
        ];

        let error = api
            .validate_contracts(&contracts)
            .expect_err("duplicate identities must fail");

        assert_eq!(error.code(), HybridErrorCode::DuplicateIdentity);
    }

    #[test]
    fn applies_the_aggregate_metadata_budget() {
        let mut config = HybridConfig::default();
        config.budgets.max_metadata_bytes = Some(1);

        let api = HybridApi::new(config);
        let contracts = [sample_contract("classical", "compute")];

        let error = api
            .validate_contracts(&contracts)
            .expect_err("metadata exceeding the configured budget must fail");

        assert_eq!(error.code(), HybridErrorCode::BudgetExhausted);
    }

    #[test]
    fn applies_the_domain_instance_budget() {
        let mut config = HybridConfig::default();
        config.budgets.max_domain_instances = Some(1);

        let api = HybridApi::new(config);
        let contracts = [
            sample_contract("classical", "compute"),
            sample_contract("quantum", "measure"),
        ];

        let error = api
            .validate_contracts(&contracts)
            .expect_err("too many contracts under policy must fail");

        assert_eq!(error.code(), HybridErrorCode::BudgetExhausted);
    }

    #[test]
    fn empty_contract_collection_is_valid_by_itself() {
        let api = HybridApi::default();

        let report = api
            .validate_contracts(&[])
            .expect("an empty collection has no structural violations");

        assert!(report.is_empty());
        assert_eq!(report.estimated_metadata_bytes, 0);
    }

    #[test]
    fn validates_interface_references_without_claiming_registry_resolution() {
        let api = HybridApi::default();

        api.validate_interface_ref(&interface("future.domain", "compute"))
            .expect("a well-formed extensible interface reference should pass");
    }

    #[test]
    fn contract_dependencies_remain_contract_owned() {
        let api = HybridApi::default();
        let dependency = DependencyDeclaration::new(
            interface("quantum", "measure"),
            DependencyKind::Data,
            true,
        )
        .expect("dependency declaration should be valid");

        let contract = DomainContract::new(
            interface("classical", "consume"),
            Vec::new(),
            Vec::new(),
            vec![dependency],
            Vec::new(),
        )
        .expect("contract should be valid");

        let report = api
            .validate_contract(&contract)
            .expect("contract structure should validate");

        assert_eq!(report.dependency_count, 1);
        // Registry resolution is deliberately outside this API's guarantee.
    }
}
