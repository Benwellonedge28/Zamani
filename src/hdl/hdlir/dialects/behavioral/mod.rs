#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Behavioral Dialect
//!
//! Path: `src/hdl/hdlir/dialects/behavioral/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Initial behavioral-dialect contract; requires integration tests.
//!
//! ## Purpose
//!
//! Define the HDLIR behavioral-dialect verification boundary for logical
//! hardware processes. This module validates process-local structural
//! contracts and delegates extension-defined semantics and attribute
//! interpretation to an explicit verifier supplied by the caller.
//!
//! The existing `ir::processes` module remains the canonical owner of process
//! records, process categories, event metadata, and local process invariants.
//! This dialect does not create a competing behavioral IR.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/processes.g4`.
//! - `grammar/hdl/combinational.g4`.
//! - `grammar/hdl/sequential.g4`.
//! - `grammar/hdl/clocking.g4` and `grammar/hdl/clocks.g4`.
//! - `src/hdl/api.rs`.
//! - `src/hdl/ids.rs`.
//! - `src/hdl/hdlir/errors.rs`.
//! - `src/hdl/hdlir/ir/processes.rs`.
//! - `src/hdl/hdlir/ir/operations.rs`.
//! - `src/hdl/hdlir/ir/regions.rs`.
//! - `src/hdl/hdlir/ir/clocks.rs`.
//! - `src/hdl/hdlir/schema/registry.rs`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! Grammar owns syntax. The canonical frontend owns source-level meaning and
//! validation. ZUIR owns universal computational meaning. HDLIR owns the
//! hardware-domain representation and its dialect-specific contracts.
//!
//! ## Owns
//!
//! - The behavioral dialect's local verification entry point.
//! - The contract for verifying extension-defined process categories.
//! - The contract for validating process attributes against their schemas.
//! - Explicit failures when required extension semantics cannot be verified.
//!
//! ## Does not own
//!
//! - Parsing, AST construction, source spans, name resolution, or frontend
//!   semantic analysis.
//! - Process record definitions, event representations, or process storage.
//! - Operation execution, simulation, synthesis, scheduling, or lowering.
//! - The schema registry or a second operation/type registry.
//! - Whole-graph reference resolution, type checking, dataflow analysis,
//!   assignment legality, or sequential state-transition verification.
//! - Clock-tree construction, clock-domain crossing implementation,
//!   physical registers, placement, routing, or vendor-specific realization.
//!
//! ## Behavioral semantics
//!
//! `HdlirProcess::validate_structure` is the canonical local structural
//! validator. This dialect invokes it rather than duplicating its rules.
//!
//! A locally valid process is not necessarily a fully valid behavioral
//! process. The enclosing graph verifier must still resolve the body region,
//! event sources, clock-domain references, values, operations, and types.
//! It must also establish assignment legality, combinational completeness,
//! state-transition semantics, and any required clock/reset constraints.
//!
//! A combinational process must not introduce persistent state implicitly.
//! A sequential process describes state-transition intent, not a particular
//! physical register implementation. An event-driven process preserves its
//! declared activation conditions. These properties must be established from
//! the process body and registered operation semantics; process metadata
//! alone cannot prove them.
//!
//! Protocol and custom process categories require an extension verifier.
//! Their names or enum variants alone do not establish their semantics.
//!
//! ## Extension integration
//!
//! The extension verifier is supplied by the integration layer. An adapter
//! may consult the canonical HDLIR schema registry, registered operation
//! contracts, and dialect-specific verifiers. This module intentionally does
//! not assume unverified registry lookup or callback APIs.
//!
//! If an extension verifier is absent, an extension-defined process category
//! or non-empty attribute set cannot be fully verified and is rejected as
//! unsupported. It is never treated as successfully verified by default.
//!
//! ## Scalability and determinism
//!
//! This module defines no universal maximum for processes, events, attributes,
//! statements, operations, signals, widths, or hardware resources.
//!
//! It introduces no fixed hardware-capacity constants. Collection sizes and
//! processing effort remain subject to available resources and any explicit
//! operational budgets enforced by the caller. This module does not claim to
//! process literally infinite input.
//!
//! It does not silently truncate process metadata, discard events, ignore
//! attributes, or downgrade an incomplete extension check to success.
//! Deterministic behavior is inherited from the existing process model and
//! required of extension verifiers for equivalent inputs.
//!
//! ## Integration
//!
//! - `dialects/mod.rs` should declare this module only after it exists.
//! - `ir/processes.rs` remains the canonical process-data owner.
//! - `ir/operations.rs` remains the canonical owner of attribute values.
//! - `schema/registry.rs` remains the canonical schema registry.
//! - `verify/processes.rs` should call this module for local behavioral
//!   contracts as part of whole-graph process verification.
//! - `verify/structure.rs` must independently resolve body-region references.
//! - `verify/clocks.rs` must independently validate clock-domain and event
//!   references and clocking constraints.
//! - `verify/types.rs` and `verify/connectivity.rs` must validate body
//!   operation types, signal relationships, and connectivity.
//! - Semantic lowering must preserve process kind, ordered events,
//!   clock-domain association, attributes, body region, and provenance.
//!
//! This module must not be declared until `src/hdl/hdlir/mod.rs` and its
//! required child modules have been made valid Rust module declarations.
//!
//! ## Completion criteria
//!
//! - The file compiles with the actual repository module tree.
//! - Process-local structural validation is delegated to the canonical model.
//! - Extension categories and attributes fail explicitly without a verifier.
//! - The whole-graph verifier remains responsible for cross-reference checks.
//! - Tests cover accepted and rejected process categories and extension cases.
//! - Formatting, all-target compilation, tests, and Clippy pass in the actual
//!   repository checkout.

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ir::processes::{HdlirProcess, HdlirProcessKind};

/// Dialect namespace used in behavioral verification diagnostics.
///
/// This is a stable logical namespace, not a physical target or vendor ID.
pub const DIALECT_NAME: &str = "hdlir.behavioral";

/// Contract required to verify extension-defined behavioral semantics.
///
/// Implementations belong to the schema/dialect integration layer, not to
/// this module. They should resolve the relevant registered contracts and
/// return an error if a contract is absent, incompatible, or violated.
///
/// Implementations must not claim successful verification when they have
/// skipped a required check or exhausted an operational budget.
///
/// This interface deliberately accepts the canonical process record. It
/// does not require a second behavioral-process representation.
pub trait BehavioralExtensionVerifier {
    /// Verify a protocol or custom process category.
    ///
    /// This method must check that the category is registered, version
    /// compatible, and semantically understood by the active implementation.
    /// It may additionally validate category-specific event and body
    /// requirements that can be checked from the process record.
    ///
    /// Whole-graph references and body-operation semantics remain the
    /// responsibility of the enclosing verifier.
    ///
    /// # Errors
    ///
    /// Returns a structured HDLIR error if the extension is unknown,
    /// incompatible, unsupported, invalid, or incompletely verified.
    fn verify_extension_process(&self, process: &HdlirProcess) -> HdlirResult<()>;

    /// Verify the schema-defined meaning of a process's attributes.
    ///
    /// An implementation must not infer semantics from an attribute name or
    /// opaque payload alone. It must use the registered compatible contract.
    ///
    /// # Errors
    ///
    /// Returns a structured HDLIR error if an attribute is unknown,
    /// incompatible, malformed, invalid for this process, or not verified.
    fn verify_process_attributes(&self, process: &HdlirProcess) -> HdlirResult<()>;
}

/// Verify the local behavioral contract of one canonical HDLIR process.
///
/// This entry point deliberately does not claim to verify the complete
/// process body or the enclosing design. It first delegates local invariants
/// to [`HdlirProcess::validate_structure`], then requires extension
/// verification where process-category or attribute semantics demand it.
///
/// # Arguments
///
/// * `process` — The canonical process record owned by `ir::processes`.
/// * `extensions` — An optional adapter backed by registered schema and
///   dialect contracts. It is required for protocol/custom process categories
///   and for any non-empty process attribute set.
///
/// # Errors
///
/// Returns an error when local process metadata is invalid, extension-defined
/// semantics cannot be verified, or attribute contracts cannot be checked.
/// Errors are propagated without converting them to success.
///
/// # Scope
///
/// Success means only that this function's local checks completed. It does
/// not prove that body-region references resolve, operations are type-correct,
/// assignments are legal, combinational outputs are complete, clock domains
/// are valid, or a physical target can realize the design.
pub fn verify_process_contract(
    process: &HdlirProcess,
    extensions: Option<&dyn BehavioralExtensionVerifier>,
) -> HdlirResult<()> {
    process.validate_structure()?;

    match process.kind() {
        HdlirProcessKind::Combinational | HdlirProcessKind::Sequential => {
            // Canonical process validation owns the local rules for these
            // built-in categories. Body and graph semantics are checked by
            // the enclosing HDLIR verifier.
        }
        HdlirProcessKind::EventDriven => {
            // The canonical process model checks that an event-driven process
            // declares at least one event. Event-source resolution is global.
        }
        HdlirProcessKind::Protocol | HdlirProcessKind::Custom(_) => {
            let verifier = extensions.ok_or_else(|| {
                HdlirError::UnsupportedFeature {
                    feature: DIALECT_NAME.to_owned(),
                    operation: format!("{:?}", process.kind()),
                    detail: Some(
                        "protocol and custom process semantics require a \
                         registered, version-compatible extension verifier"
                            .to_owned(),
                    ),
                }
            })?;

            verifier.verify_extension_process(process)?;
        }
    }

    if !process.attributes().is_empty() {
        let verifier = extensions.ok_or_else(|| HdlirError::UnsupportedFeature {
            feature: DIALECT_NAME.to_owned(),
            operation: "process attributes".to_owned(),
            detail: Some(
                "non-empty process attributes require schema-backed \
                 attribute verification"
                    .to_owned(),
            ),
        })?;

        verifier.verify_process_attributes(process)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::hdl::hdlir::ids::{HdlirProcessId, HdlirRegionId};

    fn process(kind: HdlirProcessKind) -> HdlirProcess {
        HdlirProcess::new(
            HdlirProcessId::new(1).expect("non-zero test process ID"),
            Some("test_process".to_owned()),
            kind,
            HdlirRegionId::new(1).expect("non-zero test region ID"),
            Vec::new(),
            None,
            BTreeMap::new(),
        )
        .expect("test process metadata should be structurally valid")
    }

    #[test]
    fn accepts_locally_valid_combinational_process() {
        let process = process(HdlirProcessKind::Combinational);
        assert!(verify_process_contract(&process, None).is_ok());
    }

    #[test]
    fn rejects_sequential_process_without_clock_or_edge_event() {
        let result = HdlirProcess::new(
            HdlirProcessId::new(2).expect("non-zero test process ID"),
            None,
            HdlirProcessKind::Sequential,
            HdlirRegionId::new(2).expect("non-zero test region ID"),
            Vec::new(),
            None,
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_event_driven_process_without_events() {
        let result = HdlirProcess::new(
            HdlirProcessId::new(3).expect("non-zero test process ID"),
            None,
            HdlirProcessKind::EventDriven,
            HdlirRegionId::new(3).expect("non-zero test region ID"),
            Vec::new(),
            None,
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_protocol_process_without_extension_verifier() {
        let process = process(HdlirProcessKind::Protocol);
        let result = verify_process_contract(&process, None);

        assert!(matches!(
            result,
            Err(HdlirError::UnsupportedFeature { .. })
        ));
    }

    #[test]
    fn does_not_claim_to_verify_graph_references() {
        // A valid non-zero region ID is only a reference. This local verifier
        // cannot establish that the enclosing graph actually contains it.
        let process = process(HdlirProcessKind::Combinational);

        assert!(verify_process_contract(&process, None).is_ok());
        // The graph-level verifier must still resolve process.body_region().
    }
}