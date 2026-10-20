
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Sequential Dialect
//!
//! Path: `src/hdl/hdlir/dialects/sequential/mod.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Initial implementation; requires integration and conformance tests.
//!
//! ## Purpose
//!
//! Represent the sequential-specific contract between an existing HDLIR
//! process and the state updates described by its body. This dialect preserves
//! logical state-transition intent without choosing physical registers,
//! clock resources, implementation technology, or target-specific primitives.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/sequential.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/clocks.g4`
//! - `grammar/hdl/reset.g4`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ir/processes.rs`
//! - `src/hdl/hdlir/ir/clocks.rs`
//! - `src/hdl/hdlir/ir/values.rs`
//! - `src/hdl/hdlir/verify/` (whole-graph verification, when registered)
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The frontend owns source-level semantic truth.
//! ZUIR owns universal computational meaning. HDLIR owns this hardware-domain
//! representation and its local sequential contracts.
//!
//! ## Owns
//!
//! - The meaning-preserving classification of sequential assignment intent.
//! - Ordered references from a sequential process to state-update destinations
//!   and next-value expressions represented by existing HDLIR IDs.
//! - A deterministic registry of sequential-process contracts.
//! - Local checks and cross-reference checks for these records.
//!
//! ## Does not own
//!
//! - Parsing, AST construction, source-level name resolution, or type checking.
//! - A second process, clock-domain, reset, signal, or value identity system.
//! - The operation graph, region graph, expression evaluator, or control-flow
//!   graph.
//! - The semantics of conditional branches, priority, or multiple assignments
//!   on different paths; those require analysis of the owning process body.
//! - Physical flip-flops, registers, clock trees, reset networks, placement,
//!   routing, synthesis, timing closure, or vendor-specific APIs.
//! - ZUIR, target discovery, resource allocation, or hardware execution.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on processes or state updates. Collection size
//! is limited by available resources and the implementation's representable
//! types. `Vec` and `BTreeMap` grow as needed; no hardware-capacity constants
//! or host-word-size assumptions are used for hardware semantics.
//!
//! Repeated writes to the same destination are allowed in the ordered update
//! list because they may belong to different branches or ordered statements.
//! This file does not infer whether such writes conflict. That requires
//! control-flow-aware verification.
//!
//! ## Integration contract
//!
//! `HdlirSequentialProcessContract` refers to the canonical `HdlirProcessId`.
//! `HdlirSequentialProcessTable::validate_references` resolves it through the
//! existing `HdlirProcessTable`, checks that the referenced process is
//! sequential, and delegates signal/value/clock existence to callbacks backed
//! by the containing design graph.
//!
//! The enclosing graph verifier remains responsible for type compatibility,
//! module ownership, reset legality, clock-domain consistency, control-flow
//! semantics, and all checks not provable from these local records.

use std::collections::BTreeMap;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirClockDomainId, HdlirProcessId, HdlirSignalId, HdlirValueId,
};
use crate::hdl::hdlir::ir::processes::{
    HdlirProcess, HdlirProcessKind, HdlirProcessTable,
};

/// The source-level assignment behavior preserved by a sequential update.
///
/// These variants preserve a semantic distinction; they do not select a
/// physical implementation. The owning language specification and semantic
/// frontend determine the exact behavior of each source construct.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirSequentialAssignmentKind {
    /// An assignment with blocking statement-order semantics.
    ///
    /// The value observed by later statements is governed by the process
    /// semantics. This marker alone does not implement that evaluation.
    Blocking,

    /// An assignment with deferred/nonblocking update semantics.
    ///
    /// The update is applied according to the enclosing sequential process
    /// semantics, rather than being silently rewritten as a blocking write.
    NonBlocking,
}

/// One logical state update in a sequential process.
///
/// `destination` identifies an existing logical HDLIR signal. `next_value`
/// identifies an existing HDLIR value whose interpretation is governed by
/// the process body and canonical type system.
///
/// This record deliberately does not store a physical register ID or a
/// target-specific storage primitive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirStateUpdate {
    destination: HdlirSignalId,
    next_value: HdlirValueId,
    assignment_kind: HdlirSequentialAssignmentKind,
}

impl HdlirStateUpdate {
    /// Construct a state update from existing HDLIR identities.
    ///
    /// The typed IDs guarantee non-zero identity values. Existence,
    /// declaration kind, type compatibility, and assignment legality must
    /// be checked against the containing graph.
    #[must_use]
    pub const fn new(
        destination: HdlirSignalId,
        next_value: HdlirValueId,
        assignment_kind: HdlirSequentialAssignmentKind,
    ) -> Self {
        Self {
            destination,
            next_value,
            assignment_kind,
        }
    }

    /// Return the logical state destination.
    #[must_use]
    pub const fn destination(&self) -> HdlirSignalId {
        self.destination
    }

    /// Return the HDLIR value supplying the update.
    #[must_use]
    pub const fn next_value(&self) -> HdlirValueId {
        self.next_value
    }

    /// Return the source-level assignment behavior.
    #[must_use]
    pub const fn assignment_kind(&self) -> HdlirSequentialAssignmentKind {
        self.assignment_kind
    }

    /// Validate invariants local to this record.
    ///
    /// Existence and type compatibility are graph-level responsibilities.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        // HDLIR ID types have private, non-zero representations. Keep this
        // method as the stable validation boundary for future local rules.
        if self.destination.get() == 0 {
            return Err(HdlirError::invalid_invariant(
                "sequential_update.destination_nonzero",
                "a state update must reference a non-zero signal identity",
            ));
        }

        if self.next_value.get() == 0 {
            return Err(HdlirError::invalid_invariant(
                "sequential_update.value_nonzero",
                "a state update must reference a non-zero value identity",
            ));
        }

        Ok(())
    }
}

/// Sequential-specific metadata associated with one existing HDLIR process.
///
/// The process body, events, and optional clock-domain association remain
/// owned by `HdlirProcess`. This record only attaches the ordered state-update
/// contract; it is not a second process declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirSequentialProcessContract {
    process_id: HdlirProcessId,
    state_updates: Vec<HdlirStateUpdate>,
}

impl HdlirSequentialProcessContract {
    /// Construct a sequential contract for an existing process identity.
    ///
    /// This constructor checks local records but does not resolve the process,
    /// destination signals, values, or clock domains. Call
    /// [`HdlirSequentialProcessContract::validate_references`] or validate
    /// the enclosing graph before treating the contract as verified.
    pub fn new(
        process_id: HdlirProcessId,
        state_updates: Vec<HdlirStateUpdate>,
    ) -> HdlirResult<Self> {
        let contract = Self {
            process_id,
            state_updates,
        };
        contract.validate_structure()?;
        Ok(contract)
    }

    /// Return the identity of the process this contract extends.
    #[must_use]
    pub const fn process_id(&self) -> HdlirProcessId {
        self.process_id
    }

    /// Return ordered state updates.
    ///
    /// The order is preserved. It must not be treated as proof that all
    /// updates execute on the same path or that repeated destinations are
    /// semantically conflicting.
    #[must_use]
    pub fn state_updates(&self) -> &[HdlirStateUpdate] {
        &self.state_updates
    }

    /// Return the number of state-update records.
    #[must_use]
    pub fn state_update_count(&self) -> usize {
        self.state_updates.len()
    }

    /// Return whether the contract contains no explicit state updates.
    ///
    /// A process without state updates is not necessarily invalid: its body
    /// may express other sequential behavior or require further analysis.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.state_updates.is_empty()
    }

    /// Validate the local representation without resolving graph references.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if self.process_id.get() == 0 {
            return Err(HdlirError::invalid_invariant(
                "sequential_contract.process_id_nonzero",
                "a sequential contract must reference a non-zero process identity",
            ));
        }

        for update in &self.state_updates {
            update.validate_structure()?;
        }

        Ok(())
    }

    /// Validate this contract against the existing process and graph indexes.
    ///
    /// The supplied callbacks must query the canonical containing graph.
    /// They are intentionally injected rather than replaced by private
    /// signal, value, or clock registries in this dialect.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the referenced process does not exist;
    /// - the referenced process is not classified as sequential;
    /// - an explicitly associated clock domain does not exist;
    /// - a destination signal or next-value reference cannot be resolved.
    ///
    /// This method does not prove type compatibility, reset legality,
    /// control-flow correctness, or semantic equivalence.
    pub fn validate_references<SignalExists, ValueExists, ClockDomainExists>(
        &self,
        processes: &HdlirProcessTable,
        mut signal_exists: SignalExists,
        mut value_exists: ValueExists,
        mut clock_domain_exists: ClockDomainExists,
    ) -> HdlirResult<()>
    where
        SignalExists: FnMut(HdlirSignalId) -> bool,
        ValueExists: FnMut(HdlirValueId) -> bool,
        ClockDomainExists: FnMut(HdlirClockDomainId) -> bool,
    {
        self.validate_structure()?;

        let process = processes.get(self.process_id).ok_or_else(|| {
            HdlirError::MissingEntity {
                entity_kind: "HDLIR process",
                identity: self.process_id.to_string(),
                scope: None,
            }
        })?;

        validate_sequential_process(process)?;

        if let Some(clock_domain) = process.clock_domain() {
            if !clock_domain_exists(clock_domain) {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "HDLIR clock domain",
                    identity: clock_domain.to_string(),
                    scope: Some(format!("process {}", self.process_id)),
                });
            }
        }

        for update in &self.state_updates {
            if !signal_exists(update.destination()) {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "HDLIR state destination signal",
                    identity: update.destination().to_string(),
                    scope: Some(format!("process {}", self.process_id)),
                });
            }

            if !value_exists(update.next_value()) {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "HDLIR next-state value",
                    identity: update.next_value().to_string(),
                    scope: Some(format!("process {}", self.process_id)),
                });
            }
        }

        Ok(())
    }
}

/// Check that a process is classified as sequential by the canonical process
/// representation. No parallel process classification is introduced here.
fn validate_sequential_process(process: &HdlirProcess) -> HdlirResult<()> {
    if !matches!(process.kind(), HdlirProcessKind::Sequential) {
        return Err(HdlirError::invalid_invariant(
            "sequential_contract.requires_sequential_process",
            format!(
                "process {} must have Sequential kind, found {:?}",
                process.id(),
                process.kind()
            ),
        ));
    }

    Ok(())
}

/// A deterministic collection of sequential-process contracts.
///
/// Keys are canonical process IDs, not vector indexes. The table does not
/// own process declarations, signal declarations, values, or clock domains;
/// those remain in their authoritative HDLIR graph tables.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlirSequentialProcessTable {
    contracts: BTreeMap<HdlirProcessId, HdlirSequentialProcessContract>,
}

impl HdlirSequentialProcessTable {
    /// Create an empty sequential-contract table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a contract, rejecting duplicate process registrations.
    ///
    /// The table remains unchanged when a duplicate is rejected.
    pub fn insert(
        &mut self,
        contract: HdlirSequentialProcessContract,
    ) -> HdlirResult<()> {
        contract.validate_structure()?;

        let id = contract.process_id();

        if self.contracts.contains_key(&id) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR sequential-process contract",
                identity: id.to_string(),
                scope: None,
            });
        }

        self.contracts.insert(id, contract);
        Ok(())
    }

    /// Return the contract for a process, if registered.
    #[must_use]
    pub fn get(
        &self,
        process_id: HdlirProcessId,
    ) -> Option<&HdlirSequentialProcessContract> {
        self.contracts.get(&process_id)
    }

    /// Remove and return a contract, if present.
    #[must_use]
    pub fn remove(
        &mut self,
        process_id: HdlirProcessId,
    ) -> Option<HdlirSequentialProcessContract> {
        self.contracts.remove(&process_id)
    }

    /// Iterate over contracts in deterministic process-ID order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &HdlirSequentialProcessContract> {
        self.contracts.values()
    }

    /// Return the number of registered contracts.
    #[must_use]
    pub fn len(&self) -> usize {
        self.contracts.len()
    }

    /// Return whether the table contains no contracts.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty()
    }

    /// Validate every registered contract against the canonical graph tables.
    ///
    /// Iteration is deterministic. The method stops at the first error; it
    /// never reports successful validation after a reference check fails.
    pub fn validate_references<SignalExists, ValueExists, ClockDomainExists>(
        &self,
        processes: &HdlirProcessTable,
        mut signal_exists: SignalExists,
        mut value_exists: ValueExists,
        mut clock_domain_exists: ClockDomainExists,
    ) -> HdlirResult<()>
    where
        SignalExists: FnMut(HdlirSignalId) -> bool,
        ValueExists: FnMut(HdlirValueId) -> bool,
        ClockDomainExists: FnMut(HdlirClockDomainId) -> bool,
    {
        for contract in self.contracts.values() {
            contract.validate_references(
                processes,
                &mut signal_exists,
                &mut value_exists,
                &mut clock_domain_exists,
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::hdl::hdlir::ids::{
        HdlirClockDomainId, HdlirProcessId, HdlirRegionId, HdlirSignalId,
        HdlirValueId,
    };
    use crate::hdl::hdlir::ir::processes::{
        HdlirProcess, HdlirProcessKind, HdlirProcessTable,
    };

    fn process_id(raw: u128) -> HdlirProcessId {
        HdlirProcessId::new(raw).expect("test ID must be non-zero")
    }

    fn signal_id(raw: u128) -> HdlirSignalId {
        HdlirSignalId::new(raw).expect("test ID must be non-zero")
    }

    fn value_id(raw: u128) -> HdlirValueId {
        HdlirValueId::new(raw).expect("test ID must be non-zero")
    }

    fn clock_domain_id(raw: u128) -> HdlirClockDomainId {
        HdlirClockDomainId::new(raw).expect("test ID must be non-zero")
    }

    fn sequential_process(id: HdlirProcessId) -> HdlirProcess {
        HdlirProcess::new(
            id,
            None,
            HdlirProcessKind::Sequential,
            HdlirRegionId::new(1).expect("test region ID must be non-zero"),
            Vec::new(),
            Some(clock_domain_id(1)),
            BTreeMap::new(),
        )
        .expect("sequential process metadata should be valid")
    }

    #[test]
    fn update_preserves_assignment_kind_and_references() {
        let update = HdlirStateUpdate::new(
            signal_id(7),
            value_id(11),
            HdlirSequentialAssignmentKind::NonBlocking,
        );

        assert_eq!(update.destination(), signal_id(7));
        assert_eq!(update.next_value(), value_id(11));
        assert_eq!(
            update.assignment_kind(),
            HdlirSequentialAssignmentKind::NonBlocking
        );
        assert!(update.validate_structure().is_ok());
    }

    #[test]
    fn table_rejects_duplicate_process_contracts() {
        let id = process_id(1);
        let mut table = HdlirSequentialProcessTable::new();

        let first =
            HdlirSequentialProcessContract::new(id, Vec::new()).unwrap();
        let second =
            HdlirSequentialProcessContract::new(id, Vec::new()).unwrap();

        assert!(table.insert(first).is_ok());
        assert!(table.insert(second).is_err());
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn references_validate_against_existing_process_and_graph() {
        let id = process_id(1);
        let process = sequential_process(id);
        let mut processes = HdlirProcessTable::new();
        processes.insert(process).unwrap();

        let contract = HdlirSequentialProcessContract::new(
            id,
            vec![HdlirStateUpdate::new(
                signal_id(2),
                value_id(3),
                HdlirSequentialAssignmentKind::NonBlocking,
            )],
        )
        .unwrap();

        let result = contract.validate_references(
            &processes,
            |signal| signal == signal_id(2),
            |value| value == value_id(3),
            |clock| clock == clock_domain_id(1),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn missing_state_destination_is_rejected() {
        let id = process_id(1);
        let mut processes = HdlirProcessTable::new();
        processes.insert(sequential_process(id)).unwrap();

        let contract = HdlirSequentialProcessContract::new(
            id,
            vec![HdlirStateUpdate::new(
                signal_id(2),
                value_id(3),
                HdlirSequentialAssignmentKind::NonBlocking,
            )],
        )
        .unwrap();

        let result = contract.validate_references(
            &processes,
            |_| false,
            |_| true,
            |_| true,
        );

        assert!(result.is_err());
    }

    #[test]
    fn non_sequential_process_cannot_use_sequential_contract() {
        let id = process_id(1);
        let process = HdlirProcess::new(
            id,
            None,
            HdlirProcessKind::Combinational,
            HdlirRegionId::new(1).unwrap(),
            Vec::new(),
            None,
            BTreeMap::new(),
        )
        .unwrap();

        let mut processes = HdlirProcessTable::new();
        processes.insert(process).unwrap();

        let contract =
            HdlirSequentialProcessContract::new(id, Vec::new()).unwrap();

        let result = contract.validate_references(
            &processes,
            |_| true,
            |_| true,
            |_| true,
        );

        assert!(result.is_err());
    }
}
