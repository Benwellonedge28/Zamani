
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Standard Library — HDL Domain
//!
//! Stable standard-library facade for hardware-description language
//! functionality.
//!
//! Path: `src/stdlib/domains/hdl.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Expose the existing Zamani HDL subsystem through the standard-library
//! namespace without implementing a competing HDL language, parser,
//! semantic model, intermediate representation, synthesis system,
//! simulator, target registry, or hardware execution engine.
//!
//! This module is an API composition boundary. The actual implementations
//! remain in their canonical repository locations.
//!
//! ## Architectural authorities
//!
//! Source syntax:
//!     grammar/hdl/
//!
//! HDL grammar composition:
//!     grammar/hdl/hdl.g4
//!
//! HDL grammar architecture:
//!     grammar/hdl/README.md
//!
//! Normative HDL semantics:
//!     grammar/spec/hdl.md
//!
//! Canonical frontend:
//!     src/frontend/
//!
//! HDL parser:
//!     src/frontend/parser/domains/hdl.rs
//!
//! HDL semantic validation:
//!     src/frontend/semantic/domains/hdl.rs
//!
//! Canonical HDL implementation:
//!     src/hdl/
//!
//! Hardware-domain intermediate representation:
//!     src/hdl/hdlir/
//!
//! Universal intermediate representation:
//!     src/zuir/
//!
//! Compiler:
//!     src/compiler/
//!
//! Runtime:
//!     src/runtime/
//!
//! Hardware abstraction and providers:
//!     Their respective canonical repository subsystems.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The standard-library HDL namespace.
//! - Stable access to canonical HDL APIs.
//! - Explicit public re-exports.
//! - Standard-library facade identity.
//! - Facade-level compatibility tests.
//!
//! This module does not own:
//!
//! - HDL grammar or lexical rules.
//! - Parser implementation or parser state.
//! - AST nodes or source locations.
//! - Semantic validation algorithms.
//! - HDL-specific semantic types.
//! - Shared capability or resource models.
//! - HDLIR structures, builders, passes, or serialization.
//! - ZUIR structures or conversion algorithms.
//! - Synthesis, placement, routing, or scheduling.
//! - Target discovery or physical resource allocation.
//! - Simulation kernels.
//! - Hardware drivers or vendor-specific interfaces.
//! - Runtime execution.
//!
//! Each of these responsibilities remains with its canonical owner.
//!
//! ## POCO-REAF
//!
//! Zamani's portability objective is:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! HDL programs express hardware structure, behavior, and implementation
//! intent independently of unnecessary physical-target assumptions.
//!
//! The standard-library facade introduces no universal maximum for:
//!
//! - HDL modules;
//! - interfaces;
//! - ports;
//! - signals;
//! - nets;
//! - registers;
//! - memories;
//! - clocks;
//! - processes;
//! - state machines;
//! - pipeline stages;
//! - instances;
//! - hardware widths;
//! - dimensions;
//! - devices;
//! - processors;
//! - accelerators;
//! - execution nodes;
//! - resource quantities.
//!
//! Symbolic quantities and target-independent requirements must remain
//! available through the canonical semantic and HDL interfaces.
//!
//! Actual compilation and execution remain finite and are constrained by
//! available resources, implementation capabilities, and explicitly
//! configured operational budgets.
//!
//! No finite implementation can guarantee literal infinite computation.
//! The objective is to avoid arbitrary language-level ceilings.
//!
//! ## Requirement and realization separation
//!
//! The following concepts must remain distinct:
//!
//! - Requirement: what a design needs.
//! - Capability: what a possible target provides.
//! - Preference: what is desirable.
//! - Constraint: what an implementation must respect.
//! - Hint: non-binding optimization information.
//! - Feasibility: whether a realization can satisfy the design.
//!
//! This facade does not redefine these concepts. It exposes the existing
//! HDL and canonical semantic interfaces that implement them.
//!
//! ## Integration pipeline
//!
//! ```text
//! Zamani HDL source
//!        |
//!        v
//! Canonical grammar and lexer
//!        |
//!        v
//! Frontend parser and AST
//!        |
//!        v
//! Canonical semantic validation
//!        |
//!        v
//! Shared requirements, effects, capabilities and resources
//!        |
//!        v
//! ZUIR and canonical domain representations
//!        |
//!        v
//! HDLIR
//!        |
//!        v
//! Analysis / verification / optimization
//!        |
//!        v
//! Target feasibility and realization planning
//!        |
//!        v
//! Compiler / lowering / synthesis
//!        |
//!        v
//! Runtime / HAL / providers
//!        |
//!        v
//! Physical realization
//! ```
//!
//! This is an architectural integration contract. Exposing an API does
//! not imply that every downstream compilation or hardware stage is
//! already implemented or production-ready.
//!
//! ## Error behavior
//!
//! This facade must preserve canonical errors and diagnostics.
//!
//! It must not:
//!
//! - discard parser errors;
//! - convert incomplete semantic validation into success;
//! - treat unknown target capability evidence as support;
//! - treat infeasibility as successful realization;
//! - conceal unsupported format conversions;
//! - report simulated or conceptual hardware operations as completed.
//!
//! ## Initialization
//!
//! This module is intentionally stateless.
//!
//! It does not initialize the entire standard library, initialize
//! hardware, discover devices, or start runtime services.
//!
//! Accessing this module must not trigger global initialization.
//!
//! ## Compatibility
//!
//! Public types are re-exports of canonical HDL implementations.
//! No replacement HDL types are defined here.
//!
//! Changes to canonical HDL APIs must be reflected through deliberate
//! compatibility decisions rather than duplicated implementations.
//!
//! ## Production freeze criteria
//!
//! - The canonical frontend is exposed from the crate root.
//! - The canonical HDL subsystem is exposed from the crate root.
//! - Every re-export resolves to an existing canonical implementation.
//! - The standard-library domain module is registered.
//! - No competing HDL semantic or IR authority is introduced.
//! - Parser and semantic errors remain distinguishable.
//! - Target infeasibility remains distinguishable from source invalidity.
//! - No arbitrary hardware-capacity ceiling is introduced.
//! - Safe Rust is maintained.
//! - Facade tests pass.
//! - Repository formatting, compilation, testing, and Clippy checks pass.

// =============================================================================
// Standard-library facade identity
// =============================================================================

/// Stable standard-library name for the HDL domain.
pub const HDL_STDLIB_DOMAIN_NAME: &str = "zamani.stdlib.hdl";

/// Version of this standard-library facade contract.
///
/// This version is independent of the Zamani language version, grammar
/// version, frontend semantic version, crate version, and HDLIR version.
pub const HDL_STDLIB_DOMAIN_API_VERSION: u16 = 1;

// =============================================================================
// Canonical HDL composition namespaces
// =============================================================================

/// Canonical HDL analysis facilities.
///
/// Implementations remain owned by `crate::hdl::analysis`.
pub use crate::hdl::analysis;

/// Canonical HDL public parser and semantic-validation facade.
pub use crate::hdl::api;

/// Canonical HDL capability integration.
///
/// Capability identity and evidence remain owned by the semantic subsystem.
pub use crate::hdl::capabilities;

/// Canonical HDL configuration and operational-budget contracts.
pub use crate::hdl::config;

/// Canonical HDL behavioral-contract facilities.
pub use crate::hdl::contract;

/// Canonical HDL diagnostic facilities.
pub use crate::hdl::diagnostics;

/// Canonical HDL structured errors.
pub use crate::hdl::error;

/// Canonical HDL expression integration facilities.
pub use crate::hdl::expressions;

/// Canonical HDL identity facilities.
pub use crate::hdl::ids;

/// Canonical HDL logical design and hierarchy facilities.
pub use crate::hdl::design;

/// Canonical HDL behavioral and logical operations.
pub use crate::hdl::logic;

/// Canonical HDL memory facilities.
pub use crate::hdl::memory;

/// Canonical HDL type facilities.
pub use crate::hdl::types;

/// Canonical HDL value facilities.
pub use crate::hdl::values;

/// Canonical HDL timing and clock-domain facilities.
pub use crate::hdl::timing;

/// Canonical HDL verification facilities.
pub use crate::hdl::verification;

/// Canonical HDL target descriptions, capability assessment,
/// constraints, and feasibility facilities.
///
/// This namespace does not itself discover or allocate hardware.
pub use crate::hdl::targets;

/// Canonical HDL realization plans and provider-facing interfaces.
///
/// This namespace does not independently implement physical execution.
pub use crate::hdl::realization;

/// Canonical HDL simulation request and execution interfaces.
///
/// The simulation implementation remains owned by `crate::hdl::simulation`.
pub use crate::hdl::simulation;

/// Canonical HDL library manifests and registry integration.
pub use crate::hdl::libraries;

/// Canonical HDL format import/export and interoperability facilities.
pub use crate::hdl::interoperability;

/// Canonical HDL testing and conformance facilities.
pub use crate::hdl::testing;

/// Canonical hardware-domain intermediate representation.
///
/// This is a namespace re-export, not a second standard-library IR.
pub use crate::hdl::hdlir;

// =============================================================================
// Canonical public HDL API
// =============================================================================

/// Canonical stateless HDL API facade.
pub use crate::hdl::api::HdlApi;

/// Canonical HDL API identity function.
pub use crate::hdl::api::api_name;

/// Canonical HDL API schema-version function.
pub use crate::hdl::api::api_schema_version;

/// Canonical HDL semantic-validation convenience function.
pub use crate::hdl::api::validate;

// =============================================================================
// Canonical parser contracts
// =============================================================================

/// Canonical loss-preserving HDL construct representation.
pub use crate::hdl::api::HdlConstruct;

/// Canonical HDL construct classification.
pub use crate::hdl::api::HdlConstructKind;

/// Canonical HDL domain parser.
pub use crate::hdl::api::HdlDomainParser;

/// Canonical HDL parsing error.
pub use crate::hdl::api::HdlParseError;

/// Canonical HDL parsing error classification.
pub use crate::hdl::api::HdlParseErrorKind;

/// Canonical HDL parser operational-budget configuration.
pub use crate::hdl::api::HdlParserLimits;

/// Canonical loss-preserving HDL token representation.
pub use crate::hdl::api::HdlSyntaxToken;

// =============================================================================
// Canonical semantic-validation contracts
// =============================================================================

/// Canonical HDL assignment summary.
pub use crate::hdl::api::HdlAssignment;

/// Canonical HDL clock declaration.
pub use crate::hdl::api::HdlClock;

/// Canonical HDL semantic diagnostic.
pub use crate::hdl::api::HdlDiagnostic;

/// Canonical HDL diagnostic code.
pub use crate::hdl::api::HdlDiagnosticCode;

/// Canonical HDL port or signal direction.
pub use crate::hdl::api::HdlDirection;

/// Canonical HDL semantic declaration identity.
pub use crate::hdl::api::HdlId;

/// Canonical HDL semantic-input construction error.
pub use crate::hdl::api::HdlInputError;

/// Canonical HDL process summary.
pub use crate::hdl::api::HdlProcess;

/// Canonical HDL process classification.
pub use crate::hdl::api::HdlProcessKind;

/// Canonical normalized HDL semantic-validation input.
pub use crate::hdl::api::HdlProgram;

/// Canonical HDL reset declaration.
pub use crate::hdl::api::HdlReset;

/// Canonical HDL reset behavior.
pub use crate::hdl::api::HdlResetBehavior;

/// Canonical HDL reset polarity.
pub use crate::hdl::api::HdlResetPolarity;

/// Canonical HDL diagnostic severity.
pub use crate::hdl::api::HdlSeverity;

/// Canonical HDL signal declaration.
pub use crate::hdl::api::HdlSignal;

/// Canonical HDL signedness.
pub use crate::hdl::api::HdlSignedness;

/// Canonical HDL source-location representation.
pub use crate::hdl::api::HdlSourceLocation;

/// Canonical HDL semantic-validation result.
pub use crate::hdl::api::HdlValidationResult;

/// Canonical HDL semantic-validation status.
pub use crate::hdl::api::HdlValidationStatus;

/// Canonical HDL value type.
pub use crate::hdl::api::HdlValueType;

/// Canonical HDL width representation.
pub use crate::hdl::api::HdlWidth;

/// Canonical HDL semantic validator.
///
/// This is the existing frontend semantic implementation, not a
/// standard-library validation algorithm.
pub use crate::frontend::semantic::domains::hdl::validate_hdl;

// =============================================================================
// Public convenience accessors
// =============================================================================

/// Return the stable standard-library HDL domain name.
#[must_use]
pub const fn domain_name() -> &'static str {
    HDL_STDLIB_DOMAIN_NAME
}

/// Return the version of this standard-library HDL facade contract.
#[must_use]
pub const fn domain_api_version() -> u16 {
    HDL_STDLIB_DOMAIN_API_VERSION
}

/// Construct the canonical stateless HDL API facade.
///
/// This function does not initialize hardware, discover targets,
/// or start runtime services.
#[must_use]
pub const fn api() -> HdlApi {
    HdlApi::new()
}

// =============================================================================
// Facade contract tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_library_domain_identity_is_stable() {
        assert_eq!(domain_name(), "zamani.stdlib.hdl");
        assert_eq!(domain_name(), HDL_STDLIB_DOMAIN_NAME);
    }

    #[test]
    fn standard_library_api_version_is_nonzero() {
        assert_ne!(domain_api_version(), 0);
        assert_eq!(
            domain_api_version(),
            HDL_STDLIB_DOMAIN_API_VERSION
        );
    }

    #[test]
    fn standard_library_api_is_the_canonical_hdl_api() {
        let standard_library_api = api();

        fn accepts_canonical_api(_: &crate::hdl::api::HdlApi) {}

        accepts_canonical_api(&standard_library_api);

        assert_eq!(
            standard_library_api.name(),
            crate::hdl::api::api_name()
        );

        assert_eq!(
            standard_library_api.schema_version(),
            crate::hdl::api::api_schema_version()
        );
    }

    #[test]
    fn standard_library_program_is_the_canonical_semantic_program() {
        fn accepts_canonical_program(
            _: &crate::frontend::semantic::domains::hdl::HdlProgram
        ) {}

        let program = HdlProgram::default();

        accepts_canonical_program(&program);
    }

    #[test]
    fn standard_library_validator_forwards_canonical_result() {
        let program = HdlProgram::default();

        let standard_library_result = validate(&program);
        let canonical_result =
            crate::frontend::semantic::domains::hdl::validate_hdl(&program);

        assert_eq!(
            standard_library_result,
            canonical_result
        );
    }

    #[test]
    fn standard_library_validation_does_not_claim_more_than_canonical_validation() {
        let program = HdlProgram::default();

        let result = api().validate(&program);

        let canonical =
            crate::frontend::semantic::domains::hdl::validate_hdl(&program);

        assert_eq!(result.status, canonical.status);
        assert_eq!(result.complete, canonical.complete);
        assert_eq!(result.diagnostics, canonical.diagnostics);
    }

    #[test]
    fn parser_limits_are_the_canonical_parser_limits() {
        let limits = HdlParserLimits::default();

        fn accepts_canonical_limits(
            _: &crate::frontend::parser::domains::hdl::HdlParserLimits
        ) {}

        accepts_canonical_limits(&limits);
    }

    #[test]
    fn canonical_hdlir_is_exposed_without_a_second_ir() {
        fn accepts_canonical_hdlir(
            _: &crate::hdl::hdlir::version::HdlIrVersion
        ) {
        }

        // The namespace itself is the canonical HDLIR module.
        let _canonical_module = crate::hdl::hdlir::version::HDLIR_VERSION;

        let _ = accepts_canonical_hdlir;
    }
}
