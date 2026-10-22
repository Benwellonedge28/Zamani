//! # Zamani Hybrid Computing — Domain Contract Registry
//!
//! Path: `src/hybrid/domain/registry.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only. `unsafe` is forbidden.
//!
//! ## Purpose
//!
//! Provide an explicit, extensible, deterministic registry of validated
//! domain interface contracts for hybrid computation.
//!
//! The registry answers:
//! - Is this exact domain interface contract registered?
//! - Which contract versions have been registered for this interface?
//! - What validated contract describes the requested interface?
//! - Can a new contract be registered without replacing an existing one?
//!
//! The registry does not establish that a domain implementation is installed,
//! available, trustworthy, executable, or capable of fulfilling its promises.
//! Registration records a declared contract; it is not proof of conformance.
//!
//! ## Canonical ownership
//!
//! Reuses:
//! - `domain::identity::DomainId` for validated domain identifiers.
//! - `hybrid::contract::DomainContract` for interface contracts.
//! - `hybrid::contract::DomainInterfaceRef` for exact interface references.
//! - `hybrid::version::DomainContractVersion` for contract versions.
//! - `hybrid::errors::HybridError` for structured failures.
//! - `hybrid::config::HybridConfig` for explicit metadata budgets.
//!
//! This module does not define a second domain identity, interface contract,
//! semantic model, universal IR, quantum IR, compiler pipeline, or runtime.
//!
//! ## Versioning policy
//!
//! Each registration is keyed by the complete interface reference:
//!
//!     (domain identifier, interface identifier, contract version)
//!
//! Multiple versions of the same logical interface may coexist.
//! Registering the same exact reference twice is an error, even when the
//! supplied contract values happen to be identical.
//!
//! Lookup is exact. This module never silently substitutes a different
//! version, selects a "latest" version, or assumes that two versions are
//! semantically compatible. Compatibility decisions belong to the relevant
//! contract/specification policy and integration layer.
//!
//! ## Determinism
//!
//! Registrations are stored in a `BTreeMap` keyed by canonical interface
//! references. Iteration and version enumeration therefore use stable
//! key ordering and never depend on registration order.
//!
//! ## Scalability
//!
//! There is no built-in maximum domain count, interface count, version count,
//! identifier length, or registration count. Storage grows according to
//! available resources and the configured operational policy.
//!
//! `HybridConfig` may impose an explicit metadata budget. Such a budget is an
//! operational policy, not a language-semantic or hardware-capacity limit.
//! No budget is silently invented when the caller does not provide one.
//!
//! ## Concurrency and lifecycle
//!
//! The registry is an ordinary owned value. It has no global mutable
//! singleton, implicit initialization, background task, or hidden lock.
//! Callers that need shared access may use an appropriate synchronization
//! primitive at their own integration boundary.
//!
//! Registration and removal require mutable access. A successful lookup
//! returns an `Arc<DomainContract>`, allowing the caller to retain an
//! immutable contract snapshot independently of subsequent registry changes.
//!
//! Removing a registration does not invalidate an `Arc` already returned
//! to a caller. Such a value remains a snapshot of the registered contract;
//! callers must not interpret it as proof that the registration still exists.
//!
//! ## Integration
//!
//! - `domain/identity.rs` remains the canonical owner of `DomainId`.
//! - `contract.rs` remains the canonical owner of interface contracts.
//! - `version.rs` remains the canonical owner of contract versions.
//! - `config.rs` supplies optional operational budgets.
//! - `domain/interface.rs` should adapt registered contracts to domain
//!   participation interfaces without duplicating their contract metadata.
//! - `domain/composition.rs` and `graph/verify.rs` may resolve exact
//!   references through this registry.
//! - `integration/compiler.rs` may consume registry lookups when validating
//!   a hybrid compilation request.
//! - Runtime and backend integrations must perform their own availability,
//!   capability, feasibility, and implementation-conformance checks.
//! - `grammar/spec/hybrid.md` and `grammar/hybrid/` remain authoritative for
//!   hybrid-language semantics.
//!
//! This module does not import the parser, AST, quantum IR, ZUIR, compiler,
//! runtime, hardware discovery, or vendor-specific SDKs.
//!
//! ## Completion criteria
//!
//! - Exact duplicate registrations are rejected.
//! - Different versions can coexist.
//! - Unknown references are reported explicitly.
//! - All returned contracts are structurally validated.
//! - Registry ordering is deterministic.
//! - Optional budgets are checked when supplied.
//! - Removing a registration cannot invalidate an existing contract snapshot.
//! - No fixed domain or hardware limits and no unsafe Rust are introduced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::sync::Arc;

use super::identity::DomainId;
use crate::hybrid::config::HybridConfig;
use crate::hybrid::contract::{DomainContract, DomainInterfaceRef};
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::version::DomainContractVersion;

/// The canonical registry of domain interface contracts.
///
/// The complete interface reference is the key. In particular, contract
/// versions are part of the key, so two versions of one logical interface
/// can be registered without overwriting one another.
///
/// `DomainContract` remains the authoritative description of the interface.
/// The registry stores validated contracts; it does not implement them.
#[derive(Debug, Clone, Default)]
pub struct DomainRegistry {
    contracts: BTreeMap<DomainInterfaceRef, Arc<DomainContract>>,
}

impl DomainRegistry {
    /// Creates an empty registry.
    ///
    /// No built-in domain is automatically registered. A domain identifier
    /// does not imply that an implementation or interface exists.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of registered interface versions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.contracts.len()
    }

    /// Returns whether the registry contains no registrations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty()
    }

    /// Registers a structurally valid domain interface contract.
    ///
    /// Registration is atomic with respect to the registry's logical state:
    /// validation and duplicate checks happen before insertion.
    ///
    /// A duplicate exact reference is rejected. A different version of the
    /// same logical interface may be registered independently.
    ///
    /// This method performs structural validation but does not apply an
    /// application-specific metadata budget. Use
    /// [`Self::register_with_config`] when policy-controlled validation is
    /// required.
    pub fn register(
        &mut self,
        contract: DomainContract,
    ) -> Result<Arc<DomainContract>, HybridError> {
        self.register_inner(contract, None)
    }

    /// Registers a contract after applying the supplied operational policy.
    ///
    /// The contract is validated using `HybridConfig`, including any explicit
    /// metadata-byte budget. No implicit budget is added by this registry.
    pub fn register_with_config(
        &mut self,
        contract: DomainContract,
        config: &HybridConfig,
    ) -> Result<Arc<DomainContract>, HybridError> {
        self.register_inner(contract, Some(config))
    }

    fn register_inner(
        &mut self,
        contract: DomainContract,
        config: Option<&HybridConfig>,
    ) -> Result<Arc<DomainContract>, HybridError> {
        if let Some(config) = config {
            contract
                .validate_with_config(config)
                .map_err(|error| error.in_component("hybrid domain registry"))?;
        } else {
            contract
                .validate()
                .map_err(|error| error.in_component("hybrid domain registry"))?;
        }

        let interface = contract.interface().clone();

        // DomainInterfaceRef validates its string identifiers. DomainId is
        // also checked here so registry identity follows the canonical domain
        // identifier rules rather than maintaining a second identifier rule.
        DomainId::new(interface.domain().to_owned()).map_err(|error| {
            HybridError::new(
                HybridErrorCode::InvalidContract,
                format!(
                    "contract domain identifier {:?} is invalid: {error}",
                    interface.domain()
                ),
            )
            .at_entity(
                "hybrid domain registry",
                interface.domain(),
                "domain identifier validation failed",
            )
        })?;

        if self.contracts.contains_key(&interface) {
            return Err(HybridError::new(
                HybridErrorCode::DuplicateIdentity,
                format!(
                    "interface contract {}::{} version {} is already registered",
                    interface.domain(),
                    interface.interface(),
                    interface.version(),
                ),
            )
            .at_entity(
                "hybrid domain registry",
                format!(
                    "{}::{}@{}",
                    interface.domain(),
                    interface.interface(),
                    interface.version()
                ),
                "duplicate exact interface reference",
            ));
        }

        let contract = Arc::new(contract);
        self.contracts
            .insert(interface, Arc::clone(&contract));

        Ok(contract)
    }

    /// Looks up the exact interface contract reference.
    ///
    /// No version fallback or compatibility inference is performed.
    #[must_use]
    pub fn get(
        &self,
        interface: &DomainInterfaceRef,
    ) -> Option<Arc<DomainContract>> {
        self.contracts.get(interface).cloned()
    }

    /// Looks up an exact interface reference or returns a structured error.
    ///
    /// A missing reference is an availability/registration failure, not proof
    /// that the domain itself is invalid or impossible to implement.
    pub fn require(
        &self,
        interface: &DomainInterfaceRef,
    ) -> Result<Arc<DomainContract>, HybridError> {
        self.get(interface).ok_or_else(|| {
            HybridError::unknown_domain(format!(
                "no registered contract for interface {}::{} version {}",
                interface.domain(),
                interface.interface(),
                interface.version(),
            ))
            .at_entity(
                "hybrid domain registry",
                format!(
                    "{}::{}@{}",
                    interface.domain(),
                    interface.interface(),
                    interface.version()
                ),
                "exact interface reference was not found",
            )
        })
    }

    /// Returns whether the exact interface reference is registered.
    #[must_use]
    pub fn contains(&self, interface: &DomainInterfaceRef) -> bool {
        self.contracts.contains_key(interface)
    }

    /// Returns registered versions of one logical domain interface.
    ///
    /// Results are ordered by `DomainContractVersion`, independently of the
    /// order in which registrations were inserted.
    ///
    /// The returned versions are descriptive only. This method does not
    /// designate a preferred version or assert compatibility between them.
    #[must_use]
    pub fn versions(
        &self,
        domain: &DomainId,
        interface_name: &str,
    ) -> Vec<DomainContractVersion> {
        self.contracts
            .keys()
            .filter(|reference| {
                reference.domain() == domain.as_str()
                    && reference.interface() == interface_name
            })
            .map(DomainInterfaceRef::version)
            .collect()
    }

    /// Returns all registered contracts for a domain in deterministic order.
    ///
    /// Ordering is by the complete `DomainInterfaceRef` key, not insertion
    /// order. The returned values are immutable snapshots.
    #[must_use]
    pub fn for_domain(
        &self,
        domain: &DomainId,
    ) -> Vec<Arc<DomainContract>> {
        self.contracts
            .iter()
            .filter(|(reference, _)| reference.domain() == domain.as_str())
            .map(|(_, contract)| Arc::clone(contract))
            .collect()
    }

    /// Iterates over all registrations in deterministic key order.
    ///
    /// The iterator exposes immutable references to registry-owned keys and
    /// `Arc` values. It does not allow callers to mutate registry contents.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (&DomainInterfaceRef, &Arc<DomainContract>)>
    {
        self.contracts.iter()
    }

    /// Removes one exact interface-version registration.
    ///
    /// Previously returned `Arc<DomainContract>` values remain valid
    /// immutable snapshots. Removal does not cancel running work, revoke
    /// external capabilities, unload an implementation, or affect runtime
    /// sessions.
    pub fn unregister(
        &mut self,
        interface: &DomainInterfaceRef,
    ) -> Result<Arc<DomainContract>, HybridError> {
        self.contracts.remove(interface).ok_or_else(|| {
            HybridError::new(
                HybridErrorCode::UnknownDomain,
                format!(
                    "cannot unregister missing interface {}::{} version {}",
                    interface.domain(),
                    interface.interface(),
                    interface.version(),
                ),
            )
            .at_entity(
                "hybrid domain registry",
                format!(
                    "{}::{}@{}",
                    interface.domain(),
                    interface.interface(),
                    interface.version()
                ),
                "no matching registration exists",
            )
        })
    }

    /// Removes all versions of one logical interface.
    ///
    /// Returns the removed registrations in deterministic key order.
    /// An empty result means no matching registrations existed.
    ///
    /// This operation affects registry membership only. It does not unload
    /// implementations or invalidate contract snapshots already returned.
    pub fn unregister_all_versions(
        &mut self,
        domain: &DomainId,
        interface_name: &str,
    ) -> Vec<Arc<DomainContract>> {
        let keys: Vec<DomainInterfaceRef> = self
            .contracts
            .keys()
            .filter(|reference| {
                reference.domain() == domain.as_str()
                    && reference.interface() == interface_name
            })
            .cloned()
            .collect();

        keys.into_iter()
            .filter_map(|key| self.contracts.remove(&key))
            .collect()
    }

    /// Clears all registrations.
    ///
    /// This only changes registry membership. It does not unload code,
    /// revoke capabilities, cancel work, or invalidate retained snapshots.
    pub fn clear(&mut self) {
        self.contracts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::contract::DomainContract;
    use crate::hybrid::version::DomainContractVersion;

    fn contract(
        domain: &str,
        name: &str,
        version: DomainContractVersion,
    ) -> DomainContract {
        let interface =
            DomainInterfaceRef::new(domain, name, version).expect("valid interface reference");

        DomainContract::new(
            interface,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect("valid empty interface contract")
    }

    fn version(major: u32, minor: u32, patch: u32) -> DomainContractVersion {
        DomainContractVersion::new(major, minor, patch)
    }

    fn reference(
        domain: &str,
        name: &str,
        version: DomainContractVersion,
    ) -> DomainInterfaceRef {
        DomainInterfaceRef::new(domain, name, version).expect("valid interface reference")
    }

    #[test]
    fn new_registry_is_empty() {
        let registry = DomainRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registers_and_resolves_exact_reference() {
        let mut registry = DomainRegistry::new();
        let expected = reference("quantum", "measurement", version(1, 0, 0));

        registry
            .register(contract("quantum", "measurement", version(1, 0, 0)))
            .expect("registration succeeds");

        let actual = registry.get(&expected).expect("contract is registered");

        assert_eq!(actual.interface(), &expected);
        assert!(registry.contains(&expected));
    }

    #[test]
    fn duplicate_exact_reference_is_rejected() {
        let mut registry = DomainRegistry::new();

        registry
            .register(contract("quantum", "measurement", version(1, 0, 0)))
            .expect("first registration succeeds");

        let error = registry
            .register(contract("quantum", "measurement", version(1, 0, 0)))
            .expect_err("duplicate registration must fail");

        assert_eq!(error.code(), HybridErrorCode::DuplicateIdentity);
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn multiple_versions_can_coexist() {
        let mut registry = DomainRegistry::new();

        registry
            .register(contract("quantum", "measurement", version(2, 0, 0)))
            .expect("newer version registers");

        registry
            .register(contract("quantum", "measurement", version(1, 0, 0)))
            .expect("older version registers independently");

        let versions = registry.versions(
            &DomainId::new("quantum").expect("valid domain"),
            "measurement",
        );

        assert_eq!(versions, vec![version(1, 0, 0), version(2, 0, 0)]);
    }

    #[test]
    fn lookup_does_not_fall_back_to_another_version() {
        let mut registry = DomainRegistry::new();

        registry
            .register(contract("quantum", "measurement", version(1, 0, 0)))
            .expect("registration succeeds");

        let missing = reference("quantum", "measurement", version(2, 0, 0));

        assert!(registry.get(&missing).is_none());
        assert!(registry.require(&missing).is_err());
    }

    #[test]
    fn registrations_iterate_in_deterministic_key_order() {
        let mut registry = DomainRegistry::new();

        registry
            .register(contract("quantum", "measure", version(1, 0, 0)))
            .expect("registration succeeds");

        registry
            .register(contract("classical", "evaluate", version(1, 0, 0)))
            .expect("registration succeeds");

        let domains: Vec<&str> = registry
            .iter()
            .map(|(reference, _)| reference.domain())
            .collect();

        assert_eq!(domains, vec!["classical", "quantum"]);
    }

    #[test]
    fn unregister_preserves_previously_returned_snapshot() {
        let mut registry = DomainRegistry::new();
        let reference = reference("classical", "evaluate", version(1, 0, 0));

        let snapshot = registry
            .register(contract("classical", "evaluate", version(1, 0, 0)))
            .expect("registration succeeds");

        let removed = registry
            .unregister(&reference)
            .expect("registration exists");

        assert!(registry.is_empty());
        assert_eq!(removed.interface(), &reference);
        assert_eq!(snapshot.interface(), &reference);
    }

    #[test]
    fn unregister_all_versions_removes_only_matching_interface() {
        let mut registry = DomainRegistry::new();

        registry
            .register(contract("quantum", "measure", version(1, 0, 0)))
            .expect("registration succeeds");

        registry
            .register(contract("quantum", "measure", version(2, 0, 0)))
            .expect("registration succeeds");

        registry
            .register(contract("quantum", "prepare", version(1, 0, 0)))
            .expect("registration succeeds");

        let removed = registry.unregister_all_versions(
            &DomainId::new("quantum").expect("valid domain"),
            "measure",
        );

        assert_eq!(removed.len(), 2);
        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&reference(
            "quantum",
            "prepare",
            version(1, 0, 0)
        )));
    }

    #[test]
    fn configured_registration_accepts_default_policy() {
        let mut registry = DomainRegistry::new();

        registry
            .register_with_config(
                contract("classical", "evaluate", version(1, 0, 0)),
                &HybridConfig::default(),
            )
            .expect("default policy has no implicit metadata ceiling");

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn unknown_domain_does_not_require_a_global_registry() {
        let mut first = DomainRegistry::new();
        let second = DomainRegistry::new();

        first
            .register(contract(
                "org.example.experimental",
                "compute",
                version(1, 0, 0),
            ))
            .expect("extension domain registers");

        assert_eq!(first.len(), 1);
        assert!(second.is_empty());
    }
}