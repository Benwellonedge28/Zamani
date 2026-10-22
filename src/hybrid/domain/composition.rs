
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Domain Composition
//!
//! Path: `src/hybrid/domain/composition.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Represent the structural composition of participating computational
//! domain instances in one hybrid computation.
//!
//! A composition identifies domain-interface invocations and their optional
//! structural containment relationships. It does not define the operations
//! performed by those interfaces and does not replace the canonical AST,
//! semantic model, classical IR, quantum IR, or future ZUIR representation.
//!
//! ## Ownership
//!
//! This module owns:
//! - `DomainComposition`, the validated collection of domain instances;
//! - `DomainInstance`, a graph-scoped reference to a domain interface;
//! - structural containment between domain instances;
//! - composition-level structural validation;
//! - composition metadata accounting for explicit operational budgets.
//!
//! This module does not own:
//! - source parsing, AST nodes, name resolution, or type checking;
//! - domain operation semantics or interface contracts themselves;
//! - execution dependencies, control-flow edges, or synchronization;
//! - hardware discovery, device allocation, placement, or scheduling;
//! - compilation, lowering, routing, QEC, or runtime execution;
//! - registry lifecycle or proof of implementation conformance.
//!
//! ## Architectural boundary
//!
//! `parent` means structural containment only. It does not imply execution
//! order, data dependency, completion dependency, synchronization, or
//! causality. Those relationships belong to the canonical graph, control,
//! and synchronization layers.
//!
//! `interface` is a reference to a contract owned by `hybrid::contract`.
//! A reference does not prove that the interface is registered, supported,
//! feasible, or executable.
//!
//! ## Scalability
//!
//! This module defines no fixed domain-count, nesting-depth, graph-size, or
//! hardware-capacity ceiling. Collections grow according to available
//! resources. Optional operational limits are enforced only when a caller
//! supplies `HybridConfig`.
//!
//! Parent-cycle validation is iterative rather than recursively traversing
//! nested composition, avoiding call-stack growth proportional to nesting.
//! Allocation can still fail because real machines have finite resources.
//!
//! ## Serialization and trust
//!
//! Deserialization validates each domain instance and the complete
//! composition. The representation is still not proof that referenced
//! interfaces exist or that an implementation fulfils its contract.
//! Call `validate_with_config` when explicit operational budgets must apply.
//!
//! ## Integration
//!
//! - `domain/identity.rs` owns extensible semantic domain identifiers.
//! - `hybrid/ids.rs` owns graph-scoped domain-instance identities.
//! - `hybrid/contract.rs` owns `DomainInterfaceRef` and contract semantics.
//! - `hybrid/config.rs` owns explicit operational budgets.
//! - `graph/` will own dependency edges and whole-computation graph analysis.
//! - `domain/registry.rs` will resolve interface references.
//! - `planning/` and downstream compiler layers will assess realization.
//! - `integration/` will adapt canonical frontend and domain representations.
//!
//! These dependencies are intentional. This file must not depend on runtime,
//! backend, physical-device, or vendor-specific implementations.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::contract::DomainInterfaceRef;
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::ids::{DomainInstanceId, HybridGraphId};

use super::identity::DomainId;

/// One participating domain interface within a hybrid composition.
///
/// The identity is scoped to a hybrid graph. The interface reference
/// identifies the contract to be resolved by a registry or integration
/// adapter; it is not an embedded copy of that contract.
///
/// `parent` expresses structural containment only. It is deliberately
/// separate from dataflow and execution dependencies.
///
/// The optional label is descriptive metadata and never determines semantic
/// identity or execution behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "DomainInstanceWire")]
pub struct DomainInstance {
    id: DomainInstanceId,
    interface: DomainInterfaceRef,
    parent: Option<DomainInstanceId>,
    label: Option<String>,
}

/// Deserialization representation for a domain instance.
///
/// Construction through `TryFrom` ensures that deserialization does not
/// bypass the instance's structural invariants.
#[derive(Debug, Deserialize)]
struct DomainInstanceWire {
    id: DomainInstanceId,
    interface: DomainInterfaceRef,
    parent: Option<DomainInstanceId>,
    label: Option<String>,
}

impl TryFrom<DomainInstanceWire> for DomainInstance {
    type Error = HybridError;

    fn try_from(value: DomainInstanceWire) -> Result<Self, Self::Error> {
        Self::new(value.id, value.interface, value.parent, value.label)
    }
}

impl DomainInstance {
    /// Construct a domain instance and validate its local invariants.
    ///
    /// The referenced interface is not resolved here. Registry lookup and
    /// implementation availability are separate integration responsibilities.
    pub fn new(
        id: DomainInstanceId,
        interface: DomainInterfaceRef,
        parent: Option<DomainInstanceId>,
        label: Option<String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            id,
            interface,
            parent,
            label,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return this instance's graph-scoped identity.
    #[must_use]
    pub const fn id(&self) -> DomainInstanceId {
        self.id
    }

    /// Return the referenced interface contract identity.
    #[must_use]
    pub fn interface(&self) -> &DomainInterfaceRef {
        &self.interface
    }

    /// Return the optional structural parent.
    #[must_use]
    pub const fn parent(&self) -> Option<DomainInstanceId> {
        self.parent
    }

    /// Return the optional descriptive label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Return the validated semantic domain identifier.
    ///
    /// Domain IDs are extensible; this method does not require the domain to
    /// be one of Zamani's built-in domains.
    pub fn domain_id(&self) -> Result<DomainId, HybridError> {
        DomainId::new(self.interface.domain()).map_err(|error| {
            HybridError::new(
                HybridErrorCode::InvalidContract,
                format!(
                    "interface domain `{}` is not a valid canonical domain ID: {error}",
                    self.interface.domain()
                ),
            )
            .in_component("hybrid domain composition")
        })
    }

    /// Validate local invariants, including invariants that deserialization
    /// or a future internal constructor might otherwise bypass.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.interface
            .validate()
            .map_err(|error| error.in_component("hybrid domain instance"))?;

        // DomainInterfaceRef accepts general non-empty identifiers. The
        // semantic DomainId has a stricter canonical grammar, so validate
        // that boundary explicitly.
        self.domain_id()?;

        if self.id.local().get() == 0 {
            return Err(composition_error(
                HybridErrorCode::InvalidRepresentation,
                "domain-instance identity contains a zero local component",
            ));
        }

        if let Some(parent) = self.parent {
            if parent.graph() != self.id.graph() {
                return Err(composition_error(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "parent instance {parent} belongs to a different graph \
                         than child instance {}",
                        self.id
                    ),
                ));
            }

            if parent == self.id {
                return Err(composition_error(
                    HybridErrorCode::InvalidCycle,
                    format!("domain instance {} cannot contain itself", self.id),
                ));
            }
        }

        if let Some(label) = &self.label {
            if label.chars().any(char::is_control) {
                return Err(composition_error(
                    HybridErrorCode::InvalidRepresentation,
                    format!(
                        "domain-instance label for {} contains control characters",
                        self.id
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Estimate UTF-8 bytes held by this instance's string metadata.
    ///
    /// This is an accounting estimate, not an exact heap-size measurement.
    fn estimated_metadata_bytes(&self) -> Result<u128, HybridError> {
        let mut total = 0_u128;

        add_bytes(&mut total, self.interface.domain().len())?;
        add_bytes(&mut total, self.interface.interface().len())?;

        if let Some(label) = &self.label {
            add_bytes(&mut total, label.len())?;
        }

        Ok(total)
    }
}

/// A structural composition of domain instances within one hybrid graph.
///
/// This is a composition model, not an execution graph. Execution
/// dependencies, dataflow, ordering, and synchronization must be represented
/// by their owning subsystems.
///
/// Instances retain insertion order for callers that use declaration order.
/// Use `instances_canonical_order` when a stable identity-sorted view is
/// required for deterministic processing.
///
/// The composition must be validated before being accepted by a compiler,
/// planner, or interchange boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "DomainCompositionWire")]
pub struct DomainComposition {
    graph_id: HybridGraphId,
    instances: Vec<DomainInstance>,
}

/// Deserialization representation for a complete domain composition.
///
/// The conversion validates the entire structure, including duplicate
/// identities, parent references, graph ownership, and containment cycles.
#[derive(Debug, Deserialize)]
struct DomainCompositionWire {
    graph_id: HybridGraphId,
    instances: Vec<DomainInstance>,
}

impl TryFrom<DomainCompositionWire> for DomainComposition {
    type Error = HybridError;

    fn try_from(value: DomainCompositionWire) -> Result<Self, Self::Error> {
        Self::new(value.graph_id, value.instances)
    }
}

impl DomainComposition {
    /// Construct a composition and validate its structural invariants.
    ///
    /// An empty composition is allowed as an intermediate construction state.
    /// A consumer requiring an executable or fully populated computation
    /// must enforce that requirement at its own validation boundary.
    ///
    /// This constructor does not resolve interface references against a
    /// registry and does not claim target feasibility.
    pub fn new(
        graph_id: HybridGraphId,
        instances: Vec<DomainInstance>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            graph_id,
            instances,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the graph identity that scopes this composition.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Return the instances in insertion/declaration order.
    #[must_use]
    pub fn instances(&self) -> &[DomainInstance] {
        &self.instances
    }

    /// Return the number of participating domain instances.
    #[must_use]
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    /// Return whether the composition currently has no instances.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// Look up an instance by its graph-scoped identity.
    #[must_use]
    pub fn get(&self, id: DomainInstanceId) -> Option<&DomainInstance> {
        self.instances.iter().find(|instance| instance.id == id)
    }

    /// Return the number of distinct semantic domains participating.
    ///
    /// This count is derived from extensible domain identifiers. It does not
    /// imply that each domain is registered, supported, or executable.
    pub fn distinct_domain_count(&self) -> Result<usize, HybridError> {
        let mut domains = BTreeSet::new();

        for instance in &self.instances {
            domains.insert(instance.domain_id()?);
        }

        Ok(domains.len())
    }

    /// Return instances sorted by their graph-scoped identity.
    ///
    /// This method does not mutate the stored declaration order. It provides
    /// a deterministic view for equivalent identity sets, assuming the IDs
    /// themselves are stable for the artifact being processed.
    #[must_use]
    pub fn instances_canonical_order(&self) -> Vec<&DomainInstance> {
        let mut ordered: Vec<&DomainInstance> = self.instances.iter().collect();
        ordered.sort_by_key(|instance| instance.id);
        ordered
    }

    /// Add an instance while preserving composition validity.
    ///
    /// Parent references may point to instances added later; full validation
    /// is performed after all instances have been supplied. Duplicate IDs
    /// and graph mismatches are rejected before mutating the composition.
    pub fn add_instance(
        &mut self,
        instance: DomainInstance,
    ) -> Result<(), HybridError> {
        instance.validate()?;

        if instance.id.graph() != self.graph_id {
            return Err(composition_error(
                HybridErrorCode::InvalidReference,
                format!(
                    "domain instance {} belongs to graph {}, not composition graph {}",
                    instance.id,
                    instance.id.graph(),
                    self.graph_id
                ),
            ));
        }

        if self.instances.iter().any(|existing| existing.id == instance.id) {
            return Err(composition_error(
                HybridErrorCode::DuplicateIdentity,
                format!("duplicate domain-instance identity {}", instance.id),
            ));
        }

        self.instances.push(instance);
        Ok(())
    }

    /// Remove an instance if no other instance structurally contains it.
    ///
    /// This operation is atomic: on failure, the composition remains
    /// unchanged. Execution dependencies are not inspected here because
    /// they belong to the graph/dependency subsystem.
    pub fn remove_instance(
        &mut self,
        id: DomainInstanceId,
    ) -> Result<DomainInstance, HybridError> {
        let Some(index) = self.instances.iter().position(|item| item.id == id)
        else {
            return Err(composition_error(
                HybridErrorCode::InvalidReference,
                format!("cannot remove unknown domain instance {id}"),
            ));
        };

        if let Some(child) = self
            .instances
            .iter()
            .find(|item| item.parent == Some(id))
        {
            return Err(composition_error(
                HybridErrorCode::InvalidReference,
                format!(
                    "cannot remove domain instance {id}: it contains child instance {}",
                    child.id
                ),
            ));
        }

        Ok(self.instances.remove(index))
    }

    /// Validate structural invariants for this composition.
    ///
    /// This method is required after any operation that can alter the
    /// composition and at trust boundaries. It checks:
    ///
    /// - every instance belongs to this graph;
    /// - all instance IDs are unique;
    /// - every parent reference resolves within the same composition;
    /// - no instance contains itself;
    /// - the containment relation has no cycles;
    /// - every interface's domain ID is valid.
    ///
    /// It does not validate execution dependencies, registry resolution,
    /// interface implementation conformance, or target feasibility.
    pub fn validate(&self) -> Result<(), HybridError> {
        let mut by_id: BTreeMap<DomainInstanceId, &DomainInstance> =
            BTreeMap::new();

        for instance in &self.instances {
            instance.validate()?;

            if instance.id.graph() != self.graph_id {
                return Err(composition_error(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "domain instance {} belongs to graph {}, not composition graph {}",
                        instance.id,
                        instance.id.graph(),
                        self.graph_id
                    ),
                ));
            }

            if by_id.insert(instance.id, instance).is_some() {
                return Err(composition_error(
                    HybridErrorCode::DuplicateIdentity,
                    format!("duplicate domain-instance identity {}", instance.id),
                ));
            }
        }

        for instance in &self.instances {
            if let Some(parent_id) = instance.parent {
                if !by_id.contains_key(&parent_id) {
                    return Err(composition_error(
                        HybridErrorCode::InvalidReference,
                        format!(
                            "parent instance {parent_id} referenced by {} does not exist in this composition",
                            instance.id
                        ),
                    ));
                }
            }
        }

        self.validate_containment_acyclic(&by_id)
    }

    /// Validate structural invariants and enforce explicitly configured
    /// operational budgets.
    ///
    /// `DomainInstances` measures the number of declared instances.
    /// `MetadataBytes` measures UTF-8 bytes in interface identifiers and
    /// labels. Allocator overhead and serialized framing are excluded.
    pub fn validate_with_config(
        &self,
        config: &HybridConfig,
    ) -> Result<(), HybridError> {
        self.validate()?;

        config
            .check_budget(BudgetKind::DomainInstances, self.instances.len() as u128)
            .map_err(|error| error.in_component("hybrid domain composition"))?;

        let mut metadata_bytes = 0_u128;

        for instance in &self.instances {
            let instance_bytes = instance.estimated_metadata_bytes()?;

            metadata_bytes = metadata_bytes.checked_add(instance_bytes).ok_or_else(
                || {
                    composition_error(
                        HybridErrorCode::InvalidRepresentation,
                        "composition metadata byte accounting overflowed",
                    )
                },
            )?;
        }

        config
            .check_budget(BudgetKind::MetadataBytes, metadata_bytes)
            .map_err(|error| error.in_component("hybrid domain composition"))
    }

    /// Return an estimated UTF-8 byte count for composition string metadata.
    ///
    /// This estimate intentionally excludes allocator overhead and the
    /// serialized representation's framing bytes.
    pub fn estimated_metadata_bytes(&self) -> Result<u128, HybridError> {
        let mut total = 0_u128;

        for instance in &self.instances {
            let instance_bytes = instance.estimated_metadata_bytes()?;

            total = total.checked_add(instance_bytes).ok_or_else(|| {
                composition_error(
                    HybridErrorCode::InvalidRepresentation,
                    "composition metadata byte accounting overflowed",
                )
            })?;
        }

        Ok(total)
    }

    /// Validate the structural containment relation without recursive calls.
    ///
    /// The relation is a forest: each instance has zero or one parent.
    /// Multiple roots and arbitrarily deep valid containment are permitted,
    /// subject only to available resources and any explicit caller budget.
    fn validate_containment_acyclic(
        &self,
        by_id: &BTreeMap<DomainInstanceId, &DomainInstance>,
    ) -> Result<(), HybridError> {
        // 0 = unseen, 1 = on the current walk, 2 = fully checked.
        let mut state: BTreeMap<DomainInstanceId, u8> = BTreeMap::new();

        for instance in &self.instances {
            if state.get(&instance.id) == Some(&2) {
                continue;
            }

            let mut path: Vec<DomainInstanceId> = Vec::new();
            let mut current = Some(instance.id);

            while let Some(id) = current {
                match state.get(&id).copied().unwrap_or(0) {
                    2 => break,
                    1 => {
                        return Err(composition_error(
                            HybridErrorCode::InvalidCycle,
                            format!(
                                "structural containment cycle detected at domain instance {id}"
                            ),
                        ));
                    }
                    _ => {}
                }

                state.insert(id, 1);
                path.push(id);

                current = by_id
                    .get(&id)
                    .and_then(|node| node.parent);
            }

            for visited in path {
                state.insert(visited, 2);
            }
        }

        Ok(())
    }
}

fn add_bytes(total: &mut u128, bytes: usize) -> Result<(), HybridError> {
    let bytes = bytes as u128;

    *total = total.checked_add(bytes).ok_or_else(|| {
        composition_error(
            HybridErrorCode::InvalidRepresentation,
            "composition metadata byte accounting overflowed",
        )
    })?;

    Ok(())
}

fn composition_error(
    code: HybridErrorCode,
    message: impl Into<String>,
) -> HybridError {
    HybridError::new(code, message).in_component("hybrid domain composition")
}

impl fmt::Display for DomainInstance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} -> {}::{}@{}",
            self.id,
            self.interface.domain(),
            self.interface.interface(),
            self.interface.version()
        )?;

        if let Some(parent) = self.parent {
            write!(formatter, " (contained by {parent})")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::ids::{IdAllocator, HybridGraphId};
    use crate::hybrid::version::DomainContractVersion;

    fn fixture() -> (HybridGraphId, IdAllocator) {
        (HybridGraphId::new(1).unwrap(), IdAllocator::new())
    }

    fn instance(
        graph_id: HybridGraphId,
        allocator: &IdAllocator,
        domain: &str,
        interface_name: &str,
        parent: Option<DomainInstanceId>,
    ) -> DomainInstance {
        let id = allocator
            .allocate_domain_instance_id(graph_id)
            .expect("domain-instance allocation should succeed");

        let interface = DomainInterfaceRef::new(
            domain,
            interface_name,
            DomainContractVersion::new(1, 0, 0),
        )
        .expect("interface reference should be valid");

        DomainInstance::new(id, interface, parent, None)
            .expect("domain instance should be valid")
    }

    #[test]
    fn constructs_a_composition_across_different_domains() {
        let (graph_id, allocator) = fixture();

        let classical = instance(graph_id, &allocator, "classical", "evaluate", None);
        let quantum = instance(graph_id, &allocator, "quantum", "measure", None);

        let composition =
            DomainComposition::new(graph_id, vec![classical, quantum]).unwrap();

        assert_eq!(composition.len(), 2);
        assert_eq!(composition.distinct_domain_count().unwrap(), 2);
        assert_eq!(composition.graph_id(), graph_id);
    }

    #[test]
    fn supports_extensible_namespaced_domains() {
        let (graph_id, allocator) = fixture();

        let custom = instance(
            graph_id,
            &allocator,
            "org.example.tensor",
            "execute",
            None,
        );

        let composition = DomainComposition::new(graph_id, vec![custom]).unwrap();

        assert_eq!(composition.distinct_domain_count().unwrap(), 1);
    }

    #[test]
    fn allows_forward_parent_references_when_the_final_structure_is_valid() {
        let (graph_id, allocator) = fixture();

        let parent_id = allocator
            .allocate_domain_instance_id(graph_id)
            .unwrap();
        let child_id = allocator
            .allocate_domain_instance_id(graph_id)
            .unwrap();

        let child = DomainInstance::new(
            child_id,
            DomainInterfaceRef::new(
                "quantum",
                "execute",
                DomainContractVersion::new(1, 0, 0),
            )
            .unwrap(),
            Some(parent_id),
            None,
        )
        .unwrap();

        let parent = DomainInstance::new(
            parent_id,
            DomainInterfaceRef::new(
                "classical",
                "orchestrate",
                DomainContractVersion::new(1, 0, 0),
            )
            .unwrap(),
            None,
            None,
        )
        .unwrap();

        let composition =
            DomainComposition::new(graph_id, vec![child, parent]).unwrap();

        assert_eq!(composition.len(), 2);
    }

    #[test]
    fn rejects_missing_parent_references() {
        let (graph_id, allocator) = fixture();

        let parent_id = allocator
            .allocate_domain_instance_id(graph_id)
            .unwrap();

        let child = instance(
            graph_id,
            &allocator,
            "quantum",
            "execute",
            Some(parent_id),
        );

        let error = DomainComposition::new(graph_id, vec![child]).unwrap_err();

        assert_eq!(error.code(), HybridErrorCode::InvalidReference);
    }

    #[test]
    fn rejects_instances_from_another_graph() {
        let (graph_id, allocator) = fixture();
        let other_graph = HybridGraphId::new(2).unwrap();

        let foreign = instance(other_graph, &allocator, "quantum", "execute", None);

        let error =
            DomainComposition::new(graph_id, vec![foreign]).unwrap_err();

        assert_eq!(error.code(), HybridErrorCode::InvalidReference);
    }

    #[test]
    fn rejects_duplicate_instance_identities() {
        let (graph_id, allocator) = fixture();
        let original = instance(graph_id, &allocator, "quantum", "execute", None);

        let duplicate = DomainInstance::new(
            original.id(),
            original.interface().clone(),
            None,
            Some("duplicate".to_owned()),
        )
        .unwrap();

        let error =
            DomainComposition::new(graph_id, vec![original, duplicate]).unwrap_err();

        assert_eq!(error.code(), HybridErrorCode::DuplicateIdentity);
    }

    #[test]
    fn rejects_structural_containment_cycles() {
        let (graph_id, allocator) = fixture();

        let first_id = allocator
            .allocate_domain_instance_id(graph_id)
            .unwrap();
        let second_id = allocator
            .allocate_domain_instance_id(graph_id)
            .unwrap();

        let first = DomainInstance::new(
            first_id,
            DomainInterfaceRef::new(
                "classical",
                "outer",
                DomainContractVersion::new(1, 0, 0),
            )
            .unwrap(),
            Some(second_id),
            None,
        )
        .unwrap();

        let second = DomainInstance::new(
            second_id,
            DomainInterfaceRef::new(
                "quantum",
                "inner",
                DomainContractVersion::new(1, 0, 0),
            )
            .unwrap(),
            Some(first_id),
            None,
        )
        .unwrap();

        let error =
            DomainComposition::new(graph_id, vec![first, second]).unwrap_err();

        assert_eq!(error.code(), HybridErrorCode::InvalidCycle);
    }

    #[test]
    fn removes_leaf_instances_but_protects_containment_integrity() {
        let (graph_id, allocator) = fixture();

        let parent = instance(graph_id, &allocator, "classical", "outer", None);
        let parent_id = parent.id();
        let child = instance(
            graph_id,
            &allocator,
            "quantum",
            "inner",
            Some(parent_id),
        );
        let child_id = child.id();

        let mut composition =
            DomainComposition::new(graph_id, vec![parent, child]).unwrap();

        assert_eq!(
            composition.remove_instance(parent_id).unwrap_err().code(),
            HybridErrorCode::InvalidReference
        );

        assert_eq!(composition.remove_instance(child_id).unwrap().id(), child_id);
        assert_eq!(composition.remove_instance(parent_id).unwrap().id(), parent_id);
        assert!(composition.is_empty());
    }

    #[test]
    fn enforces_only_explicit_domain_instance_budgets() {
        let (graph_id, allocator) = fixture();

        let composition = DomainComposition::new(
            graph_id,
            vec![instance(graph_id, &allocator, "quantum", "execute", None)],
        )
        .unwrap();

        let mut config = HybridConfig::default();
        config.budgets.max_domain_instances = Some(0);

        assert_eq!(
            composition
                .validate_with_config(&config)
                .unwrap_err()
                .code(),
            HybridErrorCode::BudgetExhausted
        );

        let mut unlimited = HybridConfig::default();
        unlimited.budgets.max_domain_instances = None;

        assert!(composition.validate_with_config(&unlimited).is_ok());
    }

    #[test]
    fn serialization_round_trip_preserves_a_valid_composition() {
        let (graph_id, allocator) = fixture();

        let composition = DomainComposition::new(
            graph_id,
            vec![
                instance(graph_id, &allocator, "classical", "evaluate", None),
                instance(graph_id, &allocator, "quantum", "measure", None),
            ],
        )
        .unwrap();

        let encoded = serde_json::to_string(&composition).unwrap();
        let decoded: DomainComposition = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, composition);
        decoded.validate().unwrap();
    }

    #[test]
    fn canonical_view_is_sorted_without_mutating_declaration_order() {
        let (graph_id, allocator) = fixture();

        let first = instance(graph_id, &allocator, "quantum", "execute", None);
        let second = instance(graph_id, &allocator, "classical", "evaluate", None);

        let composition =
            DomainComposition::new(graph_id, vec![second.clone(), first.clone()])
                .unwrap();

        let ordered = composition.instances_canonical_order();

        assert!(ordered[0].id() < ordered[1].id());
        assert_eq!(composition.instances()[0].id(), second.id());
    }
}
