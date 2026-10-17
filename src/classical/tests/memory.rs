
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Contract tests for Zamani classical memory services.
//!
//! Path: src/classical/tests/memory.rs
//! Baseline: Rust 1.97+, Rust 2021 edition.
//!
//! # File contract
//!
//! Status: Proposed test implementation; not yet compiled in a local checkout.
//!
//! Authority:
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/memory/memory.g4
//! - grammar/memory/allocation.g4
//! - grammar/memory/deallocation.g4
//! - grammar/memory/borrowing.g4
//! - grammar/memory/lifetimes.g4
//! - src/zuir/ids.rs
//! - src/zuir/ir/ownership.rs
//!
//! This file tests the public contracts of the existing classical memory
//! modules. It does not define a second ownership model, borrow checker,
//! lifetime engine, allocator, physical address representation, AST, or IR.
//!
//! # Integration
//!
//! Register this file from src/classical/memory/mod.rs using:
//!
//! #[cfg(test)]
//! #[path = "../tests/memory.rs"]
//! mod contract_tests;
//!
//! The tests deliberately import each authoritative child module by its
//! qualified path. They do not rely on wildcard re-exports from memory/mod.rs.
//!
//! # Scalability contract
//!
//! No test assumes infinite physical memory. Tests distinguish:
//! - valid semantic intent;
//! - explicit operational budgets;
//! - representational overflow;
//! - target capability availability;
//! - provider allocation success.
//!
//! No universal hardware or data-size ceiling is encoded here.

use std::collections::BTreeSet;

use super::{address_space, allocation, borrowing, lifetime, ownership, regions};

use crate::zuir::ids::{BlockId, OperationId, RegionId, ValueId};

// ============================================================================
// Canonical identity helpers
// ============================================================================

fn value(raw: u128) -> ValueId {
    ValueId::new(raw).expect("test value ID must be nonzero")
}

fn region(raw: u128) -> RegionId {
    RegionId::new(raw).expect("test region ID must be nonzero")
}

fn operation(raw: u128) -> OperationId {
    OperationId::new(raw).expect("test operation ID must be nonzero")
}

fn block(raw: u128) -> BlockId {
    BlockId::new(raw).expect("test block ID must be nonzero")
}

// ============================================================================
// Ownership adapter contracts
// ============================================================================

#[test]
fn ownership_adapter_uses_canonical_contract_types() {
    let id = value(1);
    let contract = ownership::copyable(id, None).expect("valid contract");

    assert_eq!(contract.value, id);
    assert_eq!(contract.kind, ownership::OwnershipKind::Copy);
    assert!(contract.capabilities.may_copy);
    assert!(!contract.capabilities.must_consume);
    contract.validate().expect("contract validates");
}

#[test]
fn ownership_kinds_preserve_distinct_capability_policies() {
    let copy = ownership::copyable(value(1), None).expect("copyable");
    let move_only = ownership::move_only(value(2), None).expect("move-only");
    let affine = ownership::affine(value(3), None).expect("affine");
    let linear = ownership::linear(value(4), None).expect("linear");

    assert!(copy.capabilities.may_copy);

    assert!(!move_only.capabilities.may_copy);
    assert!(move_only.capabilities.may_move);

    assert!(!affine.capabilities.may_copy);
    assert!(!affine.capabilities.must_consume);
    assert!(affine.capabilities.may_drop);

    assert!(!linear.capabilities.may_copy);
    assert!(linear.capabilities.must_consume);
    assert!(!linear.capabilities.may_drop);
}

#[test]
fn ownership_contracts_can_reference_canonical_regions() {
    let id = value(10);
    let scope = region(20);

    let contract =
        ownership::move_only(id, Some(scope)).expect("valid contract");

    assert_eq!(contract.value, id);
    assert_eq!(contract.declared_in, Some(scope));
    contract.validate().expect("contract validates");
}

#[test]
fn ownership_index_rejects_duplicate_contracts_without_replacing_them() {
    let id = value(11);
    let first = ownership::copyable(id, None).expect("first contract");
    let second = ownership::move_only(id, None).expect("second contract");
    let mut index = ownership::OwnershipIndex::new();

    index.declare(first.clone()).expect("first declaration");

    assert!(index.declare(second).is_err());
    assert_eq!(index.contract(id), Some(&first));
    assert_eq!(index.contract_count(), 1);
    index.validate().expect("index remains valid");
}

#[test]
fn ownership_index_reports_use_of_undeclared_values() {
    let mut index = ownership::OwnershipIndex::new();
    let use_event = ownership::OwnershipUse::new(
        value(999),
        operation(1),
        block(1),
        ownership::OwnershipUseKind::Move,
    )
    .expect("valid use event");

    assert!(index.record_use(use_event).is_err());
    assert_eq!(index.contract_count(), 0);
    assert_eq!(index.use_count(), 0);
}

// ============================================================================
// Region scope contracts
// ============================================================================

#[test]
fn region_index_declares_roots_and_nested_scopes() {
    let root = region(1);
    let child = region(2);
    let grandchild = region(3);

    let mut index = regions::RegionScopeIndex::new();

    index.declare_root(root).expect("root");
    index.declare_child(child, root).expect("child");
    index.declare_child(grandchild, child).expect("grandchild");

    assert_eq!(index.len(), 3);
    assert_eq!(index.parent_of(root), Ok(None));
    assert_eq!(index.parent_of(child), Ok(Some(root)));
    assert_eq!(index.parent_of(grandchild), Ok(Some(child)));
    index.validate().expect("region index validates");
}

#[test]
fn enclosing_region_outlives_descendants() {
    let root = region(10);
    let child = region(11);
    let grandchild = region(12);

    let mut index = regions::RegionScopeIndex::new();
    index.declare_root(root).expect("root");
    index.declare_child(child, root).expect("child");
    index.declare_child(grandchild, child).expect("grandchild");

    assert_eq!(index.outlives(root, root), Ok(true));
    assert_eq!(index.outlives(root, grandchild), Ok(true));
    assert_eq!(index.outlives(child, grandchild), Ok(true));
    assert_eq!(index.outlives(grandchild, root), Ok(false));
}

#[test]
fn sibling_regions_do_not_outlive_one_another() {
    let root = region(20);
    let left = region(21);
    let right = region(22);

    let mut index = regions::RegionScopeIndex::new();
    index.declare_root(root).expect("root");
    index.declare_child(left, root).expect("left");
    index.declare_child(right, root).expect("right");

    assert_eq!(index.outlives(left, right), Ok(false));
    assert_eq!(index.outlives(right, left), Ok(false));
}

#[test]
fn independent_region_roots_do_not_gain_implicit_relationships() {
    let first = region(30);
    let second = region(31);

    let mut index = regions::RegionScopeIndex::new();
    index.declare_root(first).expect("first root");
    index.declare_root(second).expect("second root");

    assert_eq!(index.outlives(first, second), Ok(false));
    assert_eq!(index.outlives(second, first), Ok(false));
}

#[test]
fn region_index_rejects_missing_parents_and_duplicate_ids() {
    let parent = region(40);
    let child = region(41);
    let missing = region(42);

    let mut index = regions::RegionScopeIndex::new();
    index.declare_root(parent).expect("root");

    assert_eq!(
        index.declare_child(child, missing),
        Err(regions::RegionScopeError::MissingParent {
            child,
            parent: missing,
        })
    );

    index.declare_child(child, parent).expect("valid child");

    assert_eq!(
        index.declare_child(child, parent),
        Err(regions::RegionScopeError::DuplicateRegion(child))
    );

    assert_eq!(index.len(), 2);
    index.validate().expect("index remains valid");
}

#[test]
fn region_iteration_is_deterministic_by_canonical_identity() {
    let ids = [region(103), region(101), region(102)];
    let mut index = regions::RegionScopeIndex::new();

    for id in ids {
        index.declare_root(id).expect("root declaration");
    }

    let ordered: Vec<_> = index.iter().map(|(id, _)| *id).collect();

    assert_eq!(ordered, vec![region(101), region(102), region(103)]);
}

// ============================================================================
// Lifetime environment contracts
// ============================================================================

#[test]
fn lifetime_names_accept_named_and_anonymous_forms() {
    let named = lifetime::LifetimeName::named("buffer")
        .expect("valid lifetime name");
    let anonymous = lifetime::LifetimeName::anonymous();

    assert_eq!(named.as_named(), Some("buffer"));
    assert!(!named.is_anonymous());

    assert_eq!(anonymous.as_named(), None);
    assert!(anonymous.is_anonymous());
}

#[test]
fn lifetime_source_spelling_is_normalized_without_the_apostrophe() {
    assert_eq!(
        lifetime::LifetimeName::from_source_spelling("'buffer")
            .expect("named lifetime")
            .as_named(),
        Some("buffer")
    );

    assert!(
        lifetime::LifetimeName::from_source_spelling("'_")
            .expect("anonymous lifetime")
            .is_anonymous()
    );
}

#[test]
fn lifetime_name_rejects_malformed_spelling() {
    assert!(lifetime::LifetimeName::named("").is_err());
    assert!(lifetime::LifetimeName::named(" buffer").is_err());
    assert!(lifetime::LifetimeName::named("buf'fer").is_err());
    assert!(lifetime::LifetimeName::from_source_spelling("buffer").is_err());
    assert!(lifetime::LifetimeName::from_source_spelling("'").is_err());
}

#[test]
fn declared_lifetimes_do_not_imply_outlives_relationships() {
    let longer = lifetime::LifetimeName::named("longer").expect("name");
    let shorter = lifetime::LifetimeName::named("shorter").expect("name");

    let mut environment = lifetime::LifetimeEnvironment::new();
    environment.declare(longer.clone()).expect("declare longer");
    environment.declare(shorter.clone()).expect("declare shorter");

    assert_eq!(
        environment.outlives(&longer, &shorter, &regions::RegionScopeIndex::new()),
        Ok(false)
    );
}

#[test]
fn explicit_lifetime_constraints_are_transitive() {
    let a = lifetime::LifetimeName::named("a").expect("name");
    let b = lifetime::LifetimeName::named("b").expect("name");
    let c = lifetime::LifetimeName::named("c").expect("name");

    let mut environment = lifetime::LifetimeEnvironment::new();
    environment.declare(a.clone()).expect("declare a");
    environment.declare(b.clone()).expect("declare b");
    environment.declare(c.clone()).expect("declare c");

    environment
        .add_outlives(a.clone(), b.clone())
        .expect("a outlives b");
    environment
        .add_outlives(b.clone(), c.clone())
        .expect("b outlives c");

    assert_eq!(
        environment.outlives(&a, &c, &regions::RegionScopeIndex::new()),
        Ok(true)
    );
    assert_eq!(
        environment.outlives(&c, &a, &regions::RegionScopeIndex::new()),
        Ok(false)
    );
}

#[test]
fn lifetime_environment_rejects_undeclared_constraint_endpoints() {
    let declared = lifetime::LifetimeName::named("declared").expect("name");
    let missing = lifetime::LifetimeName::named("missing").expect("name");

    let mut environment = lifetime::LifetimeEnvironment::new();
    environment
        .declare(declared.clone())
        .expect("declare lifetime");

    assert!(environment
        .add_outlives(declared, missing)
        .is_err());
}

#[test]
fn anonymous_lifetime_cannot_be_a_named_constraint_endpoint() {
    let anonymous = lifetime::LifetimeName::anonymous();
    let named = lifetime::LifetimeName::named("named").expect("name");

    assert!(lifetime::LifetimeConstraint::new(
        anonymous.clone(),
        named.clone(),
    )
    .is_err());

    assert!(lifetime::LifetimeConstraint::new(named, anonymous).is_err());
}

#[test]
fn lifetime_region_bindings_use_canonical_region_ids() {
    let name = lifetime::LifetimeName::named("buffer").expect("name");
    let scope = region(50);
    let mut scopes = regions::RegionScopeIndex::new();
    scopes.declare_root(scope).expect("scope");

    let mut environment = lifetime::LifetimeEnvironment::new();
    environment.declare(name.clone()).expect("declare lifetime");

    environment
        .bind_region(name.clone(), scope, &scopes)
        .expect("bind canonical region");

    assert_eq!(environment.region_of(&name), Some(scope));
    environment
        .validate(&scopes)
        .expect("lifetime environment validates");
}

#[test]
fn lifetime_region_binding_rejects_unknown_regions() {
    let name = lifetime::LifetimeName::named("buffer").expect("name");
    let missing = region(51);

    let mut environment = lifetime::LifetimeEnvironment::new();
    environment.declare(name.clone()).expect("declare lifetime");

    assert!(environment
        .bind_region(name.clone(), missing, &regions::RegionScopeIndex::new())
        .is_err());

    assert_eq!(environment.region_of(&name), None);
}

// ============================================================================
// Borrowing and ownership-index integration
// ============================================================================

#[test]
fn shared_borrow_is_recorded_in_both_canonical_indexes() {
    let source = value(60);
    let reference = value(61);
    let scope = region(62);
    let mut ownership_index = ownership::OwnershipIndex::new();

    ownership_index
        .declare(ownership::move_only(source, None).expect("source"))
        .expect("declare source");

    ownership_index
        .declare(ownership::shared_borrow(reference, Some(scope)).expect("ref"))
        .expect("declare reference");

    let record = borrowing::BorrowRecord::new(
        source,
        reference,
        operation(63),
        block(64),
        Some(scope),
        ownership::BorrowMutability::Shared,
    )
    .expect("borrow record");

    let mut borrow_index = borrowing::BorrowIndex::new();
    borrow_index
        .record_borrow(&mut ownership_index, record)
        .expect("record borrow");

    assert_eq!(borrow_index.len(), 1);
    assert!(borrow_index.get(reference).is_some());
    assert_eq!(ownership_index.use_count(), 1);

    borrow_index
        .validate_against(&ownership_index)
        .expect("borrow index validates");
    ownership_index.validate().expect("ownership index validates");
}

#[test]
fn exclusive_borrow_preserves_exclusive_access_mode() {
    let source = value(70);
    let reference = value(71);
    let scope = region(72);
    let mut ownership_index = ownership::OwnershipIndex::new();

    ownership_index
        .declare(ownership::move_only(source, None).expect("source"))
        .expect("declare source");

    ownership_index
        .declare(ownership::exclusive_borrow(reference, Some(scope)).expect("ref"))
        .expect("declare reference");

    let record = borrowing::BorrowRecord::new(
        source,
        reference,
        operation(73),
        block(74),
        Some(scope),
        ownership::BorrowMutability::Exclusive,
    )
    .expect("exclusive borrow");

    let mut borrow_index = borrowing::BorrowIndex::new();
    borrow_index
        .record_borrow(&mut ownership_index, record)
        .expect("record borrow");

    assert_eq!(
        borrow_index.get(reference).map(|entry| entry.mutability),
        Some(ownership::BorrowMutability::Exclusive)
    );
}

#[test]
fn self_borrow_is_rejected_before_index_mutation() {
    let id = value(80);

    assert_eq!(
        borrowing::BorrowRecord::new(
            id,
            id,
            operation(81),
            block(82),
            None,
            ownership::BorrowMutability::Shared,
        ),
        Err(borrowing::BorrowError::SelfBorrow)
    );
}

#[test]
fn borrow_with_mismatched_region_is_rejected_atomically() {
    let source = value(90);
    let reference = value(91);
    let contract_region = region(92);
    let requested_region = region(93);
    let mut ownership_index = ownership::OwnershipIndex::new();

    ownership_index
        .declare(ownership::move_only(source, None).expect("source"))
        .expect("declare source");

    ownership_index
        .declare(
            ownership::shared_borrow(reference, Some(contract_region))
                .expect("reference"),
        )
        .expect("declare reference");

    let record = borrowing::BorrowRecord::new(
        source,
        reference,
        operation(94),
        block(95),
        Some(requested_region),
        ownership::BorrowMutability::Shared,
    )
    .expect("well-formed record");

    let mut borrow_index = borrowing::BorrowIndex::new();

    assert!(borrow_index
        .record_borrow(&mut ownership_index, record)
        .is_err());

    assert!(borrow_index.is_empty());
    assert_eq!(ownership_index.use_count(), 0);
}

#[test]
fn duplicate_borrow_record_is_rejected_without_replacing_the_first() {
    let source = value(100);
    let reference = value(101);
    let scope = region(102);
    let mut ownership_index = ownership::OwnershipIndex::new();

    ownership_index
        .declare(ownership::move_only(source, None).expect("source"))
        .expect("declare source");

    ownership_index
        .declare(ownership::shared_borrow(reference, Some(scope)).expect("ref"))
        .expect("declare reference");

    let first = borrowing::BorrowRecord::new(
        source,
        reference,
        operation(103),
        block(104),
        Some(scope),
        ownership::BorrowMutability::Shared,
    )
    .expect("first record");

    let second = borrowing::BorrowRecord::new(
        source,
        reference,
        operation(105),
        block(106),
        Some(scope),
        ownership::BorrowMutability::Shared,
    )
    .expect("second record");

    let mut borrow_index = borrowing::BorrowIndex::new();
    borrow_index
        .record_borrow(&mut ownership_index, first.clone())
        .expect("first borrow");

    assert!(matches!(
        borrow_index.record_borrow(&mut ownership_index, second),
        Err(borrowing::BorrowError::DuplicateBorrow(id)) if id == reference
    ));

    assert_eq!(borrow_index.get(reference), Some(&first));
    assert_eq!(borrow_index.len(), 1);
    assert_eq!(ownership_index.use_count(), 1);
}

// ============================================================================
// Logical address-space and capability contracts
// ============================================================================

#[test]
fn address_space_identifiers_are_open_world() {
    let logical = address_space::AddressSpaceId::new(
        "future.accelerator.shared-memory",
    )
    .expect("open-world identifier");

    let target =
        address_space::TargetId::new("future.target.family").expect("target");

    let capability =
        address_space::CapabilityId::new("memory.coherent-access")
            .expect("capability");

    assert_eq!(logical.as_str(), "future.accelerator.shared-memory");
    assert_eq!(target.as_str(), "future.target.family");
    assert_eq!(capability.as_str(), "memory.coherent-access");
}

#[test]
fn address_space_identifiers_reject_empty_and_controlled_names() {
    assert!(address_space::AddressSpaceId::new("").is_err());
    assert!(address_space::AddressSpaceId::new(" host").is_err());
    assert!(address_space::TargetId::new("target\ninjected").is_err());
    assert!(address_space::CapabilityId::new(" ").is_err());
}

#[test]
fn address_space_resolution_requires_an_explicit_binding() {
    let logical =
        address_space::AddressSpaceId::new("logical.buffer").expect("space");
    let target = address_space::TargetId::new("target.a").expect("target");
    let registry = address_space::AddressSpaceRegistry::new();

    registry
        .register_space(
            address_space::AddressSpaceDescriptor::new(
                logical.clone(),
                "memory",
            )
            .expect("descriptor"),
        )
        .expect("register descriptor");

    let available = BTreeSet::new();

    assert!(matches!(
        registry.resolve(&target, &logical, &available),
        Err(address_space::AddressSpaceError::UnboundAddressSpace { .. })
    ));
}

#[test]
fn address_space_resolution_requires_all_declared_capabilities() {
    let logical =
        address_space::AddressSpaceId::new("logical.coherent").expect("space");
    let target = address_space::TargetId::new("target.b").expect("target");
    let capability =
        address_space::CapabilityId::new("memory.coherent-access")
            .expect("capability");

    let descriptor = address_space::AddressSpaceDescriptor::new(
        logical.clone(),
        "memory",
    )
    .expect("descriptor")
    .requiring(capability.clone());

    let binding = address_space::TargetAddressSpaceBinding::new(
        target.clone(),
        logical.clone(),
        "backend.coherent",
    )
    .expect("binding");

    let registry = address_space::AddressSpaceRegistry::new();
    registry.register_space(descriptor).expect("register space");
    registry.bind_target(binding).expect("bind target");

    assert!(matches!(
        registry.resolve(&target, &logical, &BTreeSet::new()),
        Err(address_space::AddressSpaceError::MissingCapabilities { .. })
    ));

    let available = BTreeSet::from([capability]);
    let resolved = registry
        .resolve(&target, &logical, &available)
        .expect("capability supplied");

    assert_eq!(resolved.backend_space(), "backend.coherent");
}

#[test]
fn address_space_bindings_do_not_implicitly_apply_to_other_targets() {
    let logical =
        address_space::AddressSpaceId::new("logical.host").expect("space");
    let first = address_space::TargetId::new("target.first").expect("target");
    let second = address_space::TargetId::new("target.second").expect("target");

    let registry = address_space::AddressSpaceRegistry::new();

    registry
        .register_space(
            address_space::AddressSpaceDescriptor::new(
                logical.clone(),
                "memory",
            )
            .expect("descriptor"),
        )
        .expect("register space");

    registry
        .bind_target(
            address_space::TargetAddressSpaceBinding::new(
                first.clone(),
                logical.clone(),
                "backend.first",
            )
            .expect("binding"),
        )
        .expect("bind first target");

    assert!(registry
        .resolve(&first, &logical, &BTreeSet::new())
        .is_ok());

    assert!(matches!(
        registry.resolve(&second, &logical, &BTreeSet::new()),
        Err(address_space::AddressSpaceError::UnboundAddressSpace { .. })
    ));
}

// ============================================================================
// Allocation and explicit resource-budget contracts
// ============================================================================

#[test]
fn allocation_request_accepts_zero_bytes_and_open_world_space_names() {
    let request = allocation::AllocationRequest::new(
        0,
        "future.memory.space",
        "future.allocation.class",
    )
    .expect("zero-byte request is valid");

    assert_eq!(request.bytes(), 0);
    assert_eq!(request.space(), "future.memory.space");
    assert_eq!(request.class(), "future.allocation.class");
}

#[test]
fn allocation_request_rejects_invalid_space_and_class_names() {
    assert!(allocation::AllocationRequest::new(1, "", "ordinary").is_err());
    assert!(allocation::AllocationRequest::new(1, "host", " ").is_err());
}

#[test]
fn host_allocation_obeys_an_explicit_byte_budget() {
    let allocator = allocation::MemoryAllocator::host(
        allocation::AllocationLimits::unbounded().with_max_live_bytes(8),
    );

    let first = allocator
        .allocate(allocation::AllocationRequest::host(8))
        .expect("allocation within budget");

    assert!(first.is_live());
    assert_eq!(first.byte_len(), 8);

    let error = allocator
        .allocate(allocation::AllocationRequest::host(1))
        .expect_err("request exceeds the configured live-byte budget");

    assert!(matches!(
        error,
        allocation::AllocationError::BudgetExceeded {
            resource: "live bytes",
            ..
        }
    ));

    let accounting = allocator.accounting().expect("accounting snapshot");
    assert_eq!(accounting.live_bytes, 8);
    assert_eq!(accounting.live_allocations, 1);

    drop(first);

    let accounting = allocator.accounting().expect("released accounting");
    assert_eq!(accounting.live_bytes, 0);
    assert_eq!(accounting.live_allocations, 0);
    assert_eq!(accounting.peak_live_bytes, 8);
}

#[test]
fn allocation_count_budget_is_separate_from_byte_budget() {
    let allocator = allocation::MemoryAllocator::host(
        allocation::AllocationLimits::unbounded()
            .with_max_live_allocations(1),
    );

    let first = allocator
        .allocate(allocation::AllocationRequest::host(0))
        .expect("first allocation");

    assert!(matches!(
        allocator.allocate(allocation::AllocationRequest::host(0)),
        Err(allocation::AllocationError::BudgetExceeded {
            resource: "live allocations",
            ..
        })
    ));

    drop(first);

    allocator
        .allocate(allocation::AllocationRequest::host(0))
        .expect("allocation succeeds after previous handle drops");
}

#[test]
fn allocator_clones_share_live_resource_accounting() {
    let allocator = allocation::MemoryAllocator::host(
        allocation::AllocationLimits::unbounded().with_max_live_bytes(16),
    );
    let clone = allocator.clone();

    let allocation = clone
        .allocate(allocation::AllocationRequest::host(12))
        .expect("allocation through clone");

    let accounting = allocator.accounting().expect("shared accounting");
    assert_eq!(accounting.live_bytes, 12);
    assert_eq!(accounting.live_allocations, 1);

    drop(allocation);

    let accounting = clone.accounting().expect("released accounting");
    assert_eq!(accounting.live_bytes, 0);
    assert_eq!(accounting.live_allocations, 0);
}

#[test]
fn allocation_failure_does_not_claim_live_storage() {
    let allocator = allocation::MemoryAllocator::host(
        allocation::AllocationLimits::unbounded().with_max_live_bytes(0),
    );

    assert!(matches!(
        allocator.allocate(allocation::AllocationRequest::host(1)),
        Err(allocation::AllocationError::BudgetExceeded { .. })
    ));

    let accounting = allocator.accounting().expect("accounting snapshot");
    assert_eq!(accounting.live_bytes, 0);
    assert_eq!(accounting.live_allocations, 0);
}

// ============================================================================
// Scale, determinism, and composition contracts
// ============================================================================

#[test]
fn region_and_lifetime_indexes_grow_with_declared_input() {
    let mut scopes = regions::RegionScopeIndex::new();
    let root = region(1_000_000);
    scopes.declare_root(root).expect("root");

    for raw in 1_000_001..1_000_257 {
        scopes
            .declare_child(region(raw), root)
            .expect("child declaration");
    }

    assert_eq!(scopes.len(), 257);
    scopes.validate().expect("all scopes valid");

    let mut environment = lifetime::LifetimeEnvironment::new();

    for index in 0..256 {
        environment
            .declare(
                lifetime::LifetimeName::named(format!("scope_{index}"))
                    .expect("valid generated test name"),
            )
            .expect("lifetime declaration");
    }

    assert_eq!(environment.len(), 256);
    environment
        .validate(&scopes)
        .expect("independent lifetime declarations are valid");
}

#[test]
fn memory_services_do_not_equate_valid_intent_with_available_resources() {
    let logical =
        address_space::AddressSpaceId::new("logical.large-buffer").expect("space");
    let target = address_space::TargetId::new("target.limited").expect("target");

    let registry = address_space::AddressSpaceRegistry::new();
    registry
        .register_space(
            address_space::AddressSpaceDescriptor::new(
                logical.clone(),
                "memory",
            )
            .expect("descriptor"),
        )
        .expect("register logical space");

    // The logical descriptor is valid, but without an explicit target binding
    // there is no basis for claiming that this target can realize it.
    assert_eq!(registry.space_count(), Ok(1));
    assert_eq!(registry.binding_count(), Ok(0));

    assert!(matches!(
        registry.resolve(&target, &logical, &BTreeSet::new()),
        Err(address_space::AddressSpaceError::UnboundAddressSpace { .. })
    ));
}
