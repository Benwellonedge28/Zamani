
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Language Programming Interface (LPI).
//!
//! Path: `src/classical/interoperability/lpi.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide a stable, host-facing interface to Zamani's existing classical
//! interoperability contracts. The LPI allows Rust callers and future host
//! language bindings to use a consistent entry point for ABI validation,
//! foreign-function declaration lookup, data-exchange validation, and
//! compatibility assessment.
//!
//! # Ownership
//!
//! This module owns:
//! - The LPI protocol version.
//! - The public LPI façade and its delegation contract.
//! - The stable grouping of existing interoperability operations.
//!
//! This module does not own:
//! - Zamani source syntax, parsing, ASTs, or semantic analysis.
//! - ABI contracts, function signatures, or calling conventions.
//! - Foreign-function declaration storage or external-call IR.
//! - Data-exchange contracts or compatibility algorithms.
//! - Compiler sessions, language-server state, or compilation pipelines.
//! - Library loading, symbol resolution, authorization, or foreign execution.
//! - Hardware discovery, resource allocation, scheduling, or target lowering.
//!
//! # Canonical authorities
//!
//! - `abi.rs` owns ABI contract definitions and validation.
//! - `foreign_calls.rs` owns foreign-call declaration and IR adaptation.
//! - `data_exchange.rs` owns data-exchange contracts.
//! - `compatibility.rs` owns exchange compatibility assessment.
//! - `api.rs` owns the existing interoperability façade.
//! - `crate::classical::error` owns shared classical errors.
//! - `crate::quantum::ir::classical::extern_call` owns canonical external-call IR.
//! - `grammar/interoperability/abi.g4` owns ABI source syntax.
//! - `grammar/Zamani.g4` and `grammar/antlr/ZamaniParser.g4` own grammar
//!   composition; neither is replaced or extended by this Rust API.
//!
//! # POCO-REAF and scalability
//!
//! The LPI introduces no semantic ceilings for functions, arguments, results,
//! endpoints, devices, workers, collections, or program size.
//!
//! Actual feasibility depends on the supplied contract, target capabilities,
//! explicit operational policies, and available resources. An unavailable
//! capability or insufficient resource must be reported by its owning
//! subsystem rather than concealed by this façade.
//!
//! # Failure behavior
//!
//! Operations return the existing structured `ClassicalResult<T>` where
//! applicable. Compatibility assessment preserves the canonical report.
//! Unknown compatibility evidence is not silently accepted by the strict
//! compatibility operation.
//!
//! The LPI does not execute migrations, load libraries, resolve native
//! symbols, or execute foreign functions.
//!
//! # Integration
//!
//! The parent `interoperability/mod.rs` must declare `pub mod lpi;` and
//! `pub mod api;` alongside the existing contract modules.
//!
//! The parent module must not copy or redefine their public types.
//! The classical module tree must then be registered from `classical/mod.rs`
//! and exposed through `lib.rs` only after its actual module graph is valid.
//!
//! # Compatibility
//!
//! The LPI protocol version describes this host-facing interface, not the
//! Zamani language version, grammar version, ABI version, or IR version.
//! Changes to public signatures or behavior require compatibility review.

use crate::classical::error::ClassicalResult;
use crate::classical::interoperability::abi::AbiContract;
use crate::classical::interoperability::api::InteroperabilityApi;
use crate::classical::interoperability::compatibility::{
    CompatibilityPolicy,
    CompatibilityReport,
};
use crate::classical::interoperability::data_exchange::{
    DataExchangeContract,
    DataExchangeEndpoint,
};
use crate::classical::interoperability::foreign_calls::ForeignFunctionDeclaration;

/// Current version of the LPI protocol.
///
/// This version is independent of the language, grammar, ABI, and IR versions.
/// Increment the major version for incompatible LPI contract changes.
/// Minor and patch changes must follow the repository's compatibility policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LpiProtocolVersion {
    /// Incompatible protocol revision.
    pub major: u32,
    /// Backward-compatible protocol additions.
    pub minor: u32,
    /// Backward-compatible corrections.
    pub patch: u32,
}

impl LpiProtocolVersion {
    /// Constructs a protocol version.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// Protocol version implemented by this LPI source.
pub const LPI_PROTOCOL_VERSION: LpiProtocolVersion =
    LpiProtocolVersion::new(1, 0, 0);

/// Stable, stateless entry point to Zamani interoperability services.
///
/// This façade intentionally owns no session, registry, credential, provider,
/// target, runtime handle, or mutable global state. Such state must be owned
/// by the relevant compiler, security, provider, or runtime subsystem.
///
/// The type is cheap to construct and may be shared by callers. Sharing this
/// value does not make external operations thread-safe; the implementation
/// performing any eventual operation must establish its own concurrency and
/// authorization guarantees.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LanguageProgrammingInterface {
    interoperability: InteroperabilityApi,
}

impl LanguageProgrammingInterface {
    /// Constructs the LPI without probing hardware or allocating resources.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            interoperability: InteroperabilityApi::new(),
        }
    }

    /// Returns the protocol version implemented by this interface.
    #[must_use]
    pub const fn protocol_version(&self) -> LpiProtocolVersion {
        LPI_PROTOCOL_VERSION
    }

    /// Returns the existing interoperability façade.
    ///
    /// This provides access to lower-level operations without creating a
    /// competing implementation or duplicating their contract definitions.
    #[must_use]
    pub const fn interoperability_api(&self) -> &InteroperabilityApi {
        &self.interoperability
    }

    /// Validates an ABI contract using the canonical ABI subsystem.
    ///
    /// This validates the declaration. It does not establish that a target
    /// supports the ABI or that a native symbol can be loaded or resolved.
    pub fn validate_abi(
        &self,
        contract: &AbiContract,
    ) -> ClassicalResult<()> {
        self.interoperability.validate_abi_contract(contract)
    }

    /// Validates a data-exchange contract.
    ///
    /// Structural validity does not prove that a codec, schema resolver,
    /// transport, provider, or destination is available.
    pub fn validate_data_exchange(
        &self,
        contract: &DataExchangeContract,
    ) -> ClassicalResult<()> {
        self.interoperability
            .validate_data_exchange_contract(contract)
    }

    /// Resolves a declared foreign function from an ABI contract.
    ///
    /// The result borrows the supplied contract. This operation does not
    /// resolve a native linker symbol or execute the function.
    pub fn resolve_foreign_function<'a>(
        &self,
        contract: &'a AbiContract,
        function_name: &str,
    ) -> ClassicalResult<ForeignFunctionDeclaration<'a>> {
        self.interoperability
            .resolve_foreign_function(contract, function_name)
    }

    /// Checks the argument and result counts of a resolved declaration.
    ///
    /// This is an arity check, not a substitute for semantic type checking,
    /// effect checking, ownership analysis, or target ABI validation.
    pub fn validate_foreign_call_arity(
        &self,
        declaration: &ForeignFunctionDeclaration<'_>,
        argument_count: usize,
        result_count: usize,
    ) -> ClassicalResult<()> {
        self.interoperability.validate_foreign_call_arity(
            declaration,
            argument_count,
            result_count,
        )
    }

    /// Assesses compatibility between two exchange endpoints.
    ///
    /// The returned report preserves the canonical compatibility status and
    /// evidence. Endpoint identity alone must not be treated as proof of
    /// codec availability or full semantic compatibility.
    #[must_use]
    pub fn assess_endpoints(
        &self,
        source: &DataExchangeEndpoint,
        destination: &DataExchangeEndpoint,
        policy: CompatibilityPolicy,
    ) -> CompatibilityReport {
        self.interoperability
            .assess_endpoints(source, destination, policy)
    }

    /// Validates an exchange contract and assesses its compatibility.
    ///
    /// The report is returned without executing a migration or data transfer.
    pub fn assess_data_exchange(
        &self,
        contract: &DataExchangeContract,
        policy: CompatibilityPolicy,
    ) -> ClassicalResult<CompatibilityReport> {
        self.interoperability
            .assess_data_exchange(contract, policy)
    }

    /// Requires compatibility without migration or unresolved evidence.
    ///
    /// This delegates to the canonical strict compatibility gate. It never
    /// weakens the policy, changes the contract, or silently accepts
    /// indeterminate evidence.
    pub fn require_compatible_without_migration(
        &self,
        contract: &DataExchangeContract,
        policy: CompatibilityPolicy,
    ) -> ClassicalResult<CompatibilityReport> {
        self.interoperability
            .require_compatible_without_migration(contract, policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lpi_is_constructible_and_stateless() {
        let first = LanguageProgrammingInterface::new();
        let second = LanguageProgrammingInterface::default();

        assert_eq!(first, second);
    }

    #[test]
    fn lpi_reports_its_protocol_version() {
        let lpi = LanguageProgrammingInterface::new();

        assert_eq!(
            lpi.protocol_version(),
            LPI_PROTOCOL_VERSION,
        );
    }

    #[test]
    fn protocol_version_is_independent_of_language_version() {
        let version = LPI_PROTOCOL_VERSION;

        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
    }

    #[test]
    fn lpi_exposes_the_canonical_interoperability_api() {
        let lpi = LanguageProgrammingInterface::new();

        // Access through the LPI must return the façade it delegates to,
        // rather than construct a second API or duplicate its state.
        assert_eq!(
            lpi.interoperability_api(),
            &InteroperabilityApi::new(),
        );
    }
}
