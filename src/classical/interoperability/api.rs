
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Interoperability API
//!
//! Path: `src/classical/interoperability/api.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides a stable, target-independent façade over the existing
//! interoperability contracts:
//!
//! - ABI declaration validation;
//! - foreign-function declaration lookup and arity validation;
//! - data-exchange contract validation;
//! - explicit data-boundary compatibility assessment;
//! - strict compatibility gating when migration or uncertainty must block
//!   an exchange.
//!
//! # Ownership
//!
//! This module owns only the façade and its orchestration policy.
//!
//! It does not own or redefine:
//!
//! - ABI declarations, signatures, calling conventions, or ABI validation;
//! - foreign-function declaration storage or external-call IR;
//! - schema, format, encoding, or data-exchange contracts;
//! - compatibility evidence or compatibility algorithms;
//! - source syntax, ASTs, semantic name resolution, or diagnostics rendering;
//! - target discovery, capability resolution, physical layout, or placement;
//! - library loading, symbol resolution, authorization, or call execution;
//! - serialization, transport, or migration execution.
//!
//! # Canonical ownership
//!
//! - `interoperability/abi.rs` owns declarative ABI contracts.
//! - `interoperability/foreign_calls.rs` adapts ABI declarations to the
//!   canonical external-call IR.
//! - `interoperability/data_exchange.rs` owns exchange contracts.
//! - `interoperability/compatibility.rs` owns compatibility evidence and
//!   evaluation.
//! - `crate::classical::error` owns the shared classical error vocabulary.
//! - `crate::quantum::ir::classical::extern_call` remains the canonical
//!   external-call IR.
//! - `grammar/interoperability/abi.g4` owns ABI source syntax.
//! - `grammar/data/serialization.g4` owns canonical serialization syntax.
//!
//! # POCO-REAF and scalability
//!
//! This façade adds no universal semantic ceilings. It does not allocate
//! payloads, discover hardware, execute foreign code, or infer compatibility
//! from identifier spelling.
//!
//! Unknown compatibility evidence is never silently accepted. An accepted
//! compatibility report is not proof that a codec, provider, target, or
//! runtime service is available; those checks belong to their respective
//! authorities.
//!
//! # Integration contract
//!
//! The parent `interoperability/mod.rs` must declare this module and the
//! existing sibling modules. The parent must not define duplicate contract
//! types. Callers may use this façade for orchestration or call the owning
//! modules directly when they need lower-level details.
//!
//! # Completion status
//!
//! This source is written against the APIs inspected in the repository.
//! It is not marked repository-verified until the module tree is registered
//! and the supported workspace build, test, formatting, and lint checks pass.

use crate::classical::error::{ClassicalError, ClassicalResult};
use crate::classical::interoperability::abi::AbiContract;
use crate::classical::interoperability::compatibility::{
    CompatibilityChecker,
    CompatibilityPolicy,
    CompatibilityReport,
};
use crate::classical::interoperability::data_exchange::{
    DataExchangeContract,
    DataExchangeEndpoint,
};
use crate::classical::interoperability::foreign_calls::{
    ForeignCallError,
    ForeignFunctionDeclaration,
};

/// Stateless entry point for classical interoperability operations.
///
/// The API retains no contracts, provider state, handles, credentials, or
/// runtime resources. Each operation validates the supplied contract at its
/// boundary and delegates to the subsystem that owns the relevant rules.
///
/// This type is intentionally zero-state. Future provider registries,
/// runtime sessions, or authorization contexts must be introduced through
/// their owning subsystems rather than hidden inside this façade.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InteroperabilityApi;

impl InteroperabilityApi {
    /// Creates the stateless interoperability façade.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates an ABI contract using the ABI subsystem's canonical rules.
    ///
    /// This validates the declaration only. It does not prove that a target
    /// supports the calling convention, that a library is installed, or that
    /// a symbol can be resolved.
    pub fn validate_abi_contract(
        &self,
        contract: &AbiContract,
    ) -> ClassicalResult<()> {
        contract.validate_classical()
    }

    /// Validates a data-exchange contract at an API trust boundary.
    ///
    /// Structural validation does not establish schema compatibility,
    /// payload validity, codec availability, transport authorization, or
    /// target feasibility.
    pub fn validate_data_exchange_contract(
        &self,
        contract: &DataExchangeContract,
    ) -> ClassicalResult<()> {
        contract
            .validate()
            .map_err(|error| error.into_classical_error())
    }

    /// Resolves a declared foreign function by its ABI-level function name.
    ///
    /// The returned value borrows the original ABI contract; this method
    /// creates no duplicate declaration registry. Contract and function
    /// validation are delegated to `ForeignFunctionDeclaration::resolve`.
    ///
    /// This operation does not resolve a native linker symbol or execute a
    /// foreign function.
    pub fn resolve_foreign_function<'a>(
        &self,
        contract: &'a AbiContract,
        function_name: &str,
    ) -> ClassicalResult<ForeignFunctionDeclaration<'a>> {
        ForeignFunctionDeclaration::resolve(contract, function_name)
            .map_err(ForeignCallError::into_classical_error)
    }

    /// Validates argument and result counts against a resolved declaration.
    ///
    /// This is an arity check only. Semantic analysis must still establish
    /// operand types, effects, ownership, lifetimes, capability requirements,
    /// and any target-specific ABI feasibility.
    pub fn validate_foreign_call_arity(
        &self,
        declaration: &ForeignFunctionDeclaration<'_>,
        argument_count: usize,
        result_count: usize,
    ) -> ClassicalResult<()> {
        declaration
            .validate_arity(argument_count, result_count)
            .map_err(ForeignCallError::into_classical_error)
    }

    /// Assesses compatibility between two exchange endpoints.
    ///
    /// The result is based on identity-only evidence from the existing
    /// compatibility subsystem. Equal identifiers establish identity for
    /// those dimensions, not codec availability or target feasibility.
    ///
    /// Callers with authoritative schema, format, encoding, or version
    /// resolution should use `CompatibilityChecker::assess` with explicit
    /// evidence rather than treating endpoint identity as a full proof.
    #[must_use]
    pub fn assess_endpoints(
        &self,
        source: &DataExchangeEndpoint,
        destination: &DataExchangeEndpoint,
        policy: CompatibilityPolicy,
    ) -> CompatibilityReport {
        CompatibilityChecker::assess_endpoints(source, destination, policy)
    }

    /// Validates an exchange contract and assesses its endpoint compatibility.
    ///
    /// The compatibility subsystem owns the evaluation policy and report.
    /// A report may be `Compatible`, `RequiresMigration`, `Incompatible`, or
    /// `Indeterminate`; this method does not execute migrations or exchanges.
    pub fn assess_data_exchange(
        &self,
        contract: &DataExchangeContract,
        policy: CompatibilityPolicy,
    ) -> ClassicalResult<CompatibilityReport> {
        CompatibilityChecker::assess_contract(contract, policy)
            .map_err(|error| error.into_classical_error())
    }

    /// Requires compatibility without migration or unresolved evidence.
    ///
    /// Returns the report only when its status is `Compatible`.
    ///
    /// The following outcomes return a structured classical validation error:
    ///
    /// - `RequiresMigration`: a separately resolved migration is necessary;
    /// - `Incompatible`: the exchange violates the selected policy;
    /// - `Indeterminate`: authoritative evidence is insufficient.
    ///
    /// This strict method never performs a migration, changes a data format,
    /// weakens a contract, or converts uncertainty into acceptance.
    pub fn require_compatible_without_migration(
        &self,
        contract: &DataExchangeContract,
        policy: CompatibilityPolicy,
    ) -> ClassicalResult<CompatibilityReport> {
        let report = self.assess_data_exchange(contract, policy)?;
        report.ensure_compatible()?;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classical::interoperability::compatibility::{
        CompatibilityFact,
        CompatibilityStatus,
    };
    use crate::classical::interoperability::data_exchange::DataExchangeEndpoint;

    fn endpoint(version: Option<&str>) -> DataExchangeEndpoint {
        DataExchangeEndpoint::new(
            "example.schema",
            "example.format",
            "example.encoding",
            version.map(str::to_owned),
        )
        .expect("test endpoint should be structurally valid")
    }

    #[test]
    fn api_is_stateless_and_constructible() {
        let first = InteroperabilityApi::new();
        let second = InteroperabilityApi;

        assert_eq!(first, second);
    }

    #[test]
    fn identical_endpoint_identities_are_compatible_under_exact_policy() {
        let api = InteroperabilityApi::new();
        let source = endpoint(Some("v1"));
        let destination = endpoint(Some("v1"));

        let report = api.assess_endpoints(
            &source,
            &destination,
            CompatibilityPolicy::Exact,
        );

        assert_eq!(report.status(), CompatibilityStatus::Compatible);
        assert!(report.is_exactly_equivalent());
    }

    #[test]
    fn missing_version_evidence_is_not_assumed_compatible() {
        let api = InteroperabilityApi::new();
        let source = endpoint(Some("v1"));
        let destination = endpoint(None);

        let report = api.assess_endpoints(
            &source,
            &destination,
            CompatibilityPolicy::Exact,
        );

        assert_eq!(report.status(), CompatibilityStatus::Indeterminate);
        assert_eq!(report.version().fact(), CompatibilityFact::Unknown);
    }

    #[test]
    fn strict_gate_rejects_indeterminate_compatibility() {
        let api = InteroperabilityApi::new();
        let source = endpoint(Some("v1"));
        let destination = endpoint(None);

        let contract = DataExchangeContract::new(
            "test.exchange",
            source,
            destination,
        )
        .expect("test exchange contract should be valid");

        let result = api.require_compatible_without_migration(
            &contract,
            CompatibilityPolicy::Exact,
        );

        assert!(result.is_err());
    }

    #[test]
    fn compatible_contract_passes_strict_gate() {
        let api = InteroperabilityApi::new();
        let source = endpoint(Some("v1"));
        let destination = endpoint(Some("v1"));

        let contract = DataExchangeContract::new(
            "test.exchange",
            source,
            destination,
        )
        .expect("test exchange contract should be valid");

        let report = api
            .require_compatible_without_migration(
                &contract,
                CompatibilityPolicy::Exact,
            )
            .expect("identical endpoint identities should pass");

        assert_eq!(report.status(), CompatibilityStatus::Compatible);
    }
}
