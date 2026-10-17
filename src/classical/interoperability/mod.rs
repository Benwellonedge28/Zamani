
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Interoperability Module
//!
//! Path: `src/classical/interoperability/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is prohibited.
//!
//! # Purpose
//!
//! Establish the canonical Rust module boundary for classical interoperability.
//! Register each implementation exactly once and expose its public interfaces
//! without duplicating contracts, algorithms, or intermediate representations.
//!
//! # Module ownership
//!
//! - `abi`: declarative ABI contracts, profiles, signatures, and validation.
//! - `foreign_calls`: foreign-function declaration handling and adaptation
//!   to the canonical external-call IR.
//! - `data_exchange`: data-exchange endpoint and transfer contracts.
//! - `compatibility`: data-boundary compatibility policies, evidence, and
//!   reports.
//! - `api`: stateless orchestration of the existing interoperability modules.
//! - `lpi`: versioned, host-facing Language Programming Interface.
//!
//! # Canonical architecture
//!
//! Source grammar
//!     -> frontend AST
//!     -> semantic validation
//!     -> ABI / data-exchange contracts
//!     -> canonical IR adaptation
//!     -> target lowering / runtime
//!
//! This module is a Rust module-registration boundary. It does not parse
//! source, perform semantic analysis, generate machine code, load libraries,
//! resolve native symbols, execute foreign calls, or allocate hardware.
//!
//! # Canonical external-call IR
//!
//! `crate::quantum::ir::classical::extern_call` remains the canonical
//! external-call IR. This module must not introduce a competing call IR,
//! ABI model, or runtime execution model.
//!
//! # Grammar integration
//!
//! `grammar/interoperability/abi.g4` owns ABI syntax.
//! `grammar/interoperability/ffi.g4` owns the source-level foreign-interface
//! boundary.
//! `grammar/interoperability/foreign-functions.g4` owns foreign-function
//! declaration syntax.
//! `grammar/data/serialization.g4` owns canonical serialization syntax.
//! `grammar/interoperability/serialization.g4` is an interoperability-facing
//! grammar boundary, not a second serialization language.
//! `grammar/antlr/ZamaniParser.g4` owns parser composition.
//!
//! The Rust module tree must not be used as a substitute for grammar,
//! frontend, or semantic integration.
//!
//! # Scalability and POCO-REAF
//!
//! This module imposes no universal limits on functions, arguments, results,
//! exchange endpoints, devices, workers, or program size. It owns no
//! allocation policy or physical-resource assumptions.
//!
//! Finite implementation resources, configured operational budgets, and
//! target-specific capabilities are handled by their respective owners.
//! Resource failure must not be misrepresented as a language-level semantic
//! ceiling.
//!
//! # Dependency direction
//!
//! The façade modules may depend on their owning contract modules.
//! Contract modules must not depend on the LPI or on this module's root-level
//! re-exports. The module graph must remain acyclic.
//!
//! # Integration requirements
//!
//! 1. Keep every declared file at the path shown below.
//! 2. Register this module from `src/classical/mod.rs` only after that module
//!    has a valid Rust implementation and its dependencies are registered.
//! 3. Expose `classical` from `src/lib.rs` only after the full module tree
//!    compiles.
//! 4. Preserve existing public module paths and validate downstream imports.
//! 5. Do not claim build verification until the workspace checks pass.
//!
//! # Compatibility policy
//!
//! Existing module paths remain public. The root-level re-exports below are
//! convenience entry points; the owning modules remain authoritative.
//! Breaking changes to public APIs or contract semantics require explicit
//! compatibility review.
//!
//! # Completion status
//!
//! Module registration and exports are specified here. Repository-wide
//! integration, compilation, tests, and lint verification remain separate
//! acceptance gates.

/// Declarative ABI contracts and validation.
pub mod abi;

/// Existing stateless interoperability façade.
pub mod api;

/// Data-boundary compatibility policies and reports.
pub mod compatibility;

/// Data-exchange endpoint and transfer contracts.
pub mod data_exchange;

/// Foreign-function declaration handling and canonical IR adaptation.
pub mod foreign_calls;

/// Host-facing Language Programming Interface.
pub mod lpi;

/// Convenient access to the existing interoperability façade.
///
/// The canonical definition remains in [`api`].
pub use api::InteroperabilityApi;

/// Convenient access to the host-facing language interface.
///
/// The canonical definitions remain in [`lpi`].
pub use lpi::{
    LanguageProgrammingInterface,
    LpiProtocolVersion,
    LPI_PROTOCOL_VERSION,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_public_interoperability_modules_are_registered() {
        // Compile-time references ensure the module tree exposes the expected
        // owning modules without introducing duplicate contract definitions.
        let _abi_module = core::any::type_name::<
            fn(&abi::AbiContract),
        >();

        let _api_module =
            core::any::type_name::<api::InteroperabilityApi>();

        let _compatibility_module =
            core::any::type_name::<compatibility::CompatibilityPolicy>();

        let _data_exchange_module =
            core::any::type_name::<data_exchange::DataExchangeContract>();

        let _foreign_calls_module =
            core::any::type_name::<
                foreign_calls::ForeignFunctionDeclaration<'static>,
            >();

        let _lpi_module =
            core::any::type_name::<lpi::LanguageProgrammingInterface>();
    }

    #[test]
    fn root_exports_match_their_canonical_module_definitions() {
        let api = InteroperabilityApi::new();
        let lpi = LanguageProgrammingInterface::new();

        assert_eq!(api, api::InteroperabilityApi::new());
        assert_eq!(
            lpi.protocol_version(),
            LPI_PROTOCOL_VERSION,
        );
        assert_eq!(
            lpi.protocol_version(),
            lpi::LPI_PROTOCOL_VERSION,
        );
    }

    #[test]
    fn lpi_protocol_version_is_nonzero_and_explicit() {
        assert!(LPI_PROTOCOL_VERSION.major > 0);
    }
}
