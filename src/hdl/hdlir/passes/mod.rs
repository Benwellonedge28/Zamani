#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Transformation Passes
//!
//! Path: `src/hdl/hdlir/passes/mod.rs`
//! Rust edition: 2021.
//! Minimum Rust version: 1.97 or later.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for HDLIR transformation contracts,
//! pass execution, and preservation checking.
//!
//! It connects:
//!
//! - [`contracts`]: declarative pass identities, requirements, conditions,
//!   semantic guarantees, analysis dependencies, and preservation declarations;
//! - [`manager`]: checked orchestration of transformation applications;
//! - [`preservation`]: explicit evidence and acceptance checks for required
//!   semantic-preservation guarantees.
//!
//! ## Architectural authority
//!
//! This module follows the repository's normative HDL and compiler boundaries:
//!
//! - `grammar/DESIGN.md`;
//! - `grammar/spec/hdl.md`;
//! - `grammar/hdl/hdl.g4` and applicable HDL delegate grammars;
//! - `grammar/hdl/sequential.g4`;
//! - `grammar/hdl/processes.g4`;
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established as the normative HDLIR spec;
//! - `src/hdl/hdlir/config.rs`;
//! - `src/hdl/hdlir/errors.rs`;
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! Source syntax and source-level semantics remain owned by the canonical
//! grammar, frontend AST, and semantic model. ZUIR remains the universal
//! computational representation. HDLIR passes transform hardware-domain IR;
//! they do not own parsing, general name resolution, physical placement,
//! routing, synthesis, or timing closure.
//!
//! ## Ownership
//!
//! This module owns module registration and selected public re-exports only.
//! It does not implement transformations, verify equivalence, define a second
//! IR, or duplicate contracts declared in its child modules.
//!
//! ## Scalability and safety
//!
//! This composition root imposes no limits on the number of passes, operations,
//! signals, modules, clock domains, or hardware resources. Operational budgets
//! belong to the configuration and the relevant execution, analysis, and
//! verification implementations. Budget exhaustion or incomplete proof must
//! never be interpreted as successful preservation.
//!
//! No unsafe Rust is permitted.
//!
//! ## Integration
//!
//! `manager` uses contracts and preservation checking. Transformations should
//! declare their requirements through `contracts`, then be invoked through
//! the manager. The preservation provider must implement the semantics needed
//! to establish each requested guarantee; merely returning a report is not
//! proof of hardware equivalence.
//!
//! The public re-exports below are conveniences, not a replacement for the
//! owning modules. Types with potentially colliding names remain available
//! through their fully qualified module paths.
//!
//! ## Completion criteria
//!
//! This module is complete when:
//!
//! - all three child modules exist and compile;
//! - their cross-module API contracts agree;
//! - the HDLIR root declares this module;
//! - the former placeholder reference in `src/hdl/mod.rs` is removed or
//!   replaced with a meaningful module-registration test;
//! - repository-wide formatting, checking, tests, and Clippy pass.

/// Declarative pass identities, conditions, dependencies, and guarantees.
pub mod contracts;

/// Pass execution, condition evaluation, and candidate acceptance.
pub mod manager;

/// Preservation evidence and transformation-preservation checks.
pub mod preservation;

// -----------------------------------------------------------------------------
// Deliberate public re-exports
// -----------------------------------------------------------------------------

pub use contracts::{
    AnalysisFreshness,
    AnalysisRequirement,
    ConditionEvaluation,
    ConditionStrength,
    ContractCondition,
    ContractEvaluationReport,
    ContractKey,
    DeterminismGuarantee,
    PassContract,
    PassContractBuilder,
    PassIdentity,
    PassVersion,
    PreservationGuarantee,
    SemanticGuarantee,
};

pub use manager::{
    AnalysisCoordinator,
    ConditionDiagnostic,
    ConditionResult,
    ConditionStage,
    ContractEvaluator as PassConditionEvaluator,
    HdlirPass,
    PassExecutionReport,
    PassManager,
};

pub use preservation::{
    PreservationChecker,
    PreservationEvidence,
    PreservationEvidenceProvider,
    PreservationEvaluation,
    PreservationOutcome,
    PreservationReport,
    PreservationStatus,
};