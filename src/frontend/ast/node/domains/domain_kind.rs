//! # Zamani Native AST — Domain Kind
//!
//! Canonical domain-kind API for the native Zamani AST.
//!
//! ## Architectural role
//!
//! `DomainKind` identifies the computational-domain identity associated with a
//! source-level construct.
//!
//! The canonical representation is owned by [`super::domain::Domain`].
//! This module deliberately does **not** introduce a second representation.
//!
//! ```text
//!                    ┌──────────────────────────────┐
//!                    │ domain.rs                    │
//!                    │                              │
//!                    │ Canonical Domain identity    │
//!                    └──────────────┬───────────────┘
//!                                   │
//!                         canonical representation
//!                                   │
//!                    ┌──────────────▼───────────────┐
//!                    │ domain_kind.rs                │
//!                    │                              │
//!                    │ DomainKind = Domain          │
//!                    │ compatibility / semantic API │
//!                    └──────────────────────────────┘
//! ```
//!
//! ## Why this file is intentionally small
//!
//! A common architectural failure is to define:
//!
//! ```text
//! domain.rs       → Domain
//! domain_kind.rs  → DomainKind
//! domain_registry → another DomainId
//! semantic model  → another DomainIdentity
//! ```
//!
//! This creates multiple representations of the same concept, eventually
//! producing conversion code, inconsistent equality rules, incompatible
//! serialization, and difficult migration paths.
//!
//! Zamani instead has one authoritative source-level domain identity:
//!
//! ```text
//! frontend::ast::node::domains::Domain
//! ```
//!
//! `DomainKind` is therefore an API-level name for that same identity.
//!
//! ## POCO-REAF
//!
//! Domain kinds are intentionally:
//!
//! - namespace-qualified;
//! - extensible;
//! - source-level;
//! - hardware-independent;
//! - vendor-independent;
//! - backend-independent;
//! - machine-size-independent;
//! - qubit-count-independent;
//! - topology-independent;
//! - gate-set-independent;
//! - instruction-set-independent.
//!
//! A new computational domain does not require modifying this file.
//!
//! For example, all of the following can be represented by the canonical
//! [`Domain`] implementation:
//!
//! ```text
//! zamani:classical
//! zamani:quantum
//! zamani:hybrid
//! zamani:hdl
//! zamani:ai
//! zamani:neuromorphic
//! zamani:photonic
//! future.example:domain
//! ```
//!
//! The exact textual separator and validation rules remain the responsibility
//! of `domain.rs`. This file must not create a second set of rules.
//!
//! ## Critical architectural boundary
//!
//! `DomainKind` answers only:
//!
//! > "Which source-level computational domain identity is being referred to?"
//!
//! It does NOT answer:
//!
//! - which machine will execute it;
//! - which QPU will execute it;
//! - which CPU/GPU/FPGA/ASIC will execute it;
//! - which vendor provides the hardware;
//! - which topology is available;
//! - which qubits are physically assigned;
//! - how resources are allocated;
//! - how operations are routed;
//! - how operations are scheduled;
//! - how gates are decomposed;
//! - how pulses are generated;
//! - how calibration is performed;
//! - how QEC is implemented;
//! - how noise is mitigated;
//! - which backend is selected.
//!
//! Those concerns belong downstream.
//!
//! ```text
//! Source
//!   │
//!   ▼
//! Parser
//!   │
//!   ▼
//! Native AST
//!   │
//!   ├── Domain / DomainKind
//!   │
//!   ▼
//! Structural validation
//!   │
//!   ▼
//! Semantic analysis
//!   │
//!   ▼
//! Semantic Domain Identity
//!   │
//!   ▼
//! ZUIR
//!   │
//!   ├── Quantum IR
//!   ├── Classical IR
//!   ├── HDL IR
//!   └── future domain IR
//!   │
//!   ▼
//! Target lowering
//!   │
//!   ▼
//! Hardware / runtime
//! ```
//!
//! ## Domain-neutrality
//!
//! This file MUST NOT depend on:
//!
//! - `quantum::ir`;
//! - ZUIR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - OpenQASM;
//! - quantum hardware;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend APIs;
//! - runtime execution.
//!
//! ## No closed domain enumeration
//!
//! Do not replace this implementation with:
//!
//! ```text
//! enum DomainKind {
//!     Classical,
//!     Quantum,
//!     HDL,
//! }
//! ```
//!
//! That design would make every new computational domain require modification
//! of the native AST.
//!
//! The canonical [`Domain`] representation is instead namespaced and open to
//! extension.
//!
//! ## No duplicate identity
//!
//! This module deliberately does not define another struct such as:
//!
//! ```text
//! struct DomainKind {
//!     namespace: String,
//!     name: String,
//! }
//! ```
//!
//! because `Domain` already owns exactly that responsibility.
//!
//! There must be one authoritative representation.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! ```text
//! parser → DomainKind
//! ```
//!
//! The parser may construct a `DomainKind` through the public API inherited
//! from `Domain`.
//!
//! ### Structural validation
//!
//! `DomainKind` uses the same structural validation as `Domain`.
//!
//! No semantic registry lookup occurs here.
//!
//! ### Semantic analysis
//!
//! Semantic analysis resolves the source-level `DomainKind` against the domain
//! registry and produces the semantic domain identity.
//!
//! This file must not perform that lookup.
//!
//! ### Semantic model
//!
//! The semantic model may retain a resolved domain identity and associate it
//! with the originating AST `NodeId`.
//!
//! It must not require a second AST-level domain representation.
//!
//! ### ZUIR
//!
//! ZUIR consumes the resolved semantic domain information.
//!
//! `DomainKind` does not depend on ZUIR.
//!
//! ### Quantum
//!
//! `quantum` is merely one possible domain namespace.
//!
//! Quantum-specific implementation belongs downstream.
//!
//! ### Future domains
//!
//! A future domain should require no modification to this file.
//!
//! ## Scalability
//!
//! There is no domain-count limit encoded here.
//!
//! There is no machine-size limit encoded here.
//!
//! There is no qubit-count limit encoded here.
//!
//! There is no topology limit encoded here.
//!
//! There is no fixed number of domains encoded here.
//!
//! Any resource limits required for hostile-input protection belong to
//! configurable compiler/session policies, not this type.
//!
//! ## Determinism
//!
//! Because `DomainKind` is exactly the canonical `Domain` representation:
//!
//! - equality is deterministic;
//! - hashing is deterministic;
//! - ordering is deterministic;
//! - identity does not depend on pointers;
//! - identity does not depend on allocation addresses;
//! - identity does not depend on process state;
//! - identity does not depend on timestamps;
//! - identity does not depend on hardware;
//! - identity does not depend on backend state.
//!
//! ## Serialization
//!
//! Serialization is intentionally not implemented independently here.
//!
//! Any serialization implementation for `DomainKind` must use the canonical
//! `Domain` representation so that the AST cannot acquire two incompatible
//! serialization formats.
//!
//! ## Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ## Safety
//!
//! This file contains no `unsafe` code and performs no external operations.
//!
//! ```text
//! domain_kind.rs
//!     │
//!     └── domain.rs
//! ```
//!
//! The dependency is intentionally one-way.
//!
//! The canonical domain implementation does not depend on `domain_kind.rs`.
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - the `DomainKind` public API name;
//! - compatibility aliases for domain-related error/component types;
//! - documentation of domain-kind semantics;
//! - compile-time guarantees that `DomainKind` is the canonical `Domain`
//!   representation.
//!
//! This file does NOT own:
//!
//! - domain identity storage;
//! - domain validation implementation;
//! - domain registration;
//! - domain resolution;
//! - capability matching;
//! - resource matching;
//! - target selection;
//! - hardware selection.
//!
//! Those responsibilities remain in their appropriate layers.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Canonical source-level computational-domain identity.
///
/// `DomainKind` is deliberately an alias rather than a second data structure.
/// This guarantees that `Domain` and `DomainKind` cannot diverge in:
//!
//! - representation;
//! - equality;
//! - ordering;
//! - hashing;
//! - validation;
//! - textual identity;
//! - future extensions.
pub type DomainKind = super::domain::Domain;

/// Canonical error type for construction and validation of a [`DomainKind`].
///
/// This is an alias to the canonical domain error rather than a duplicate
/// error hierarchy.
pub type DomainKindError = super::domain::DomainError;

/// Canonical component identifier used by domain validation.
///
/// This remains an alias so diagnostics and validation cannot acquire a second
/// incompatible component taxonomy.
pub type DomainKindComponent = super::domain::DomainComponent;

/// Schema version inherited from the canonical domain representation.
///
/// Do not create an independent version number here. Domain identity and
/// domain-kind identity are the same AST representation.
pub const DOMAIN_KIND_SCHEMA_VERSION: u16 =
    super::domain::DOMAIN_SCHEMA_VERSION;

/// Stable AST kind name inherited from the canonical domain implementation.
pub const DOMAIN_KIND_AST_NAME: &str =
    super::domain::DOMAIN_KIND_NAME;

/// Compile-time constructor convenience.
///
/// This function exists primarily to make the intended `DomainKind` API
/// explicit to callers and documentation tools.
///
/// It delegates completely to the canonical `Domain::new`.
#[inline]
pub fn new(
    namespace: impl Into<String>,
    name: impl Into<String>,
) -> Result<DomainKind, DomainKindError> {
    DomainKind::new(namespace, name)
}

/// Parses a canonical domain-kind identity.
///
/// The parsing rules are owned by [`super::domain::Domain`].
///
/// This prevents parser and semantic components from acquiring independent
/// textual identity rules.
#[inline]
pub fn parse(value: &str) -> Result<DomainKind, DomainKindError> {
    parse_domain_identity(value)
}

/// Validates an existing domain kind using the canonical domain validation
/// implementation.
///
/// This performs structural validation only.
#[inline]
pub fn validate(kind: &DomainKind) -> Result<(), DomainKindError> {
    kind.validate()
}

/// Returns whether two domain kinds identify exactly the same source-level
/// domain.
///
/// No semantic registry lookup occurs.
#[inline]
#[must_use]
pub fn same_domain(left: &DomainKind, right: &DomainKind) -> bool {
    left == right
}

/// Returns the stable namespace of a domain kind.
#[inline]
#[must_use]
pub fn namespace(kind: &DomainKind) -> &str {
    kind.namespace()
}

/// Returns the unqualified name of a domain kind.
#[inline]
#[must_use]
pub fn name(kind: &DomainKind) -> &str {
    kind.name()
}

/// Returns the canonical qualified textual representation.
#[inline]
#[must_use]
pub fn qualified_name(kind: &DomainKind) -> String {
    kind.qualified_name()
}

/// Determines whether a domain belongs to a namespace.
///
/// This is a structural source-level query and must not be confused with
/// semantic domain registration.
#[inline]
#[must_use]
pub fn is_in_namespace(kind: &DomainKind, namespace: &str) -> bool {
    kind.namespace() == namespace
}

/// Determines whether a domain has the requested namespace and name.
#[inline]
#[must_use]
pub fn is(kind: &DomainKind, namespace: &str, name: &str) -> bool {
    kind.is(namespace, name)
}

/// Parses the canonical textual representation.
///
/// The canonical implementation currently exposes construction and qualified
/// representation directly. Keeping this helper localizes parsing compatibility
/// so consumers do not need to know the implementation details of `Domain`.
///
/// The canonical textual form produced by `Domain::qualified_name()` is:
///
/// ```text
/// namespace.name
/// ```
///
/// The split occurs at the first `.` so nested namespace components remain
/// representable.
///
/// Validation is ultimately delegated to `Domain::new`.
fn parse_domain_identity(value: &str) -> Result<DomainKind, DomainKindError> {
    let Some(separator) = value.find('.') else {
        return DomainKind::new(value, "");
    };

    let namespace = &value[..separator];
    let name = &value[separator + 1..];

    DomainKind::new(namespace, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_kind_is_exactly_domain() {
        let kind = new("zamani", "quantum").expect("valid domain");

        let domain: super::super::domain::Domain = kind.clone();

        assert_eq!(kind, domain);
    }

    #[test]
    fn domain_kind_uses_canonical_constructor() {
        let kind = new("zamani", "quantum").expect("valid domain");

        assert_eq!(kind.namespace(), "zamani");
        assert_eq!(kind.name(), "quantum");
        assert_eq!(qualified_name(&kind), "zamani.quantum");
    }

    #[test]
    fn domain_kind_validation_uses_domain_validation() {
        let kind = new("zamani", "quantum").expect("valid domain");

        assert_eq!(validate(&kind), Ok(()));
    }

    #[test]
    fn domain_kind_comparison_is_canonical() {
        let first = new("zamani", "quantum").expect("valid domain");
        let second = new("zamani", "quantum").expect("valid domain");

        assert!(same_domain(&first, &second));
        assert_eq!(first, second);
    }

    #[test]
    fn different_namespaces_are_distinct() {
        let first = new("zamani", "quantum").expect("valid domain");
        let second = new("future", "quantum").expect("valid domain");

        assert!(!same_domain(&first, &second));
    }

    #[test]
    fn arbitrary_future_domains_are_supported() {
        let kind = new(
            "future.example",
            "photonic-computing",
        )
        .expect("future domain must be representable");

        assert_eq!(namespace(&kind), "future.example");
        assert_eq!(name(&kind), "photonic-computing");
        assert_eq!(
            qualified_name(&kind),
            "future.example.photonic-computing"
        );
    }

    #[test]
    fn quantum_is_not_a_closed_enum_variant() {
        let quantum = new("zamani", "quantum").expect("valid domain");
        let future = new("future", "quantum").expect("valid domain");

        assert_ne!(quantum, future);
        assert_eq!(name(&quantum), "quantum");
        assert_eq!(name(&future), "quantum");
    }

    #[test]
    fn namespace_query_is_pure() {
        let kind = new("zamani", "quantum").expect("valid domain");

        assert!(is_in_namespace(&kind, "zamani"));
        assert!(!is_in_namespace(&kind, "hardware"));
    }

    #[test]
    fn identity_query_is_pure() {
        let kind = new("zamani", "quantum").expect("valid domain");

        assert!(is(&kind, "zamani", "quantum"));
        assert!(!is(&kind, "zamani", "classical"));
    }

    #[test]
    fn schema_identity_is_inherited() {
        assert_eq!(
            DOMAIN_KIND_SCHEMA_VERSION,
            super::super::domain::DOMAIN_SCHEMA_VERSION
        );

        assert_eq!(
            DOMAIN_KIND_AST_NAME,
            super::super::domain::DOMAIN_KIND_NAME
        );
    }

    #[test]
    fn error_type_is_canonical_domain_error() {
        let result = new("", "quantum");

        assert_eq!(
            result,
            Err(super::super::domain::DomainError::EmptyNamespace)
        );
    }

    #[test]
    fn no_second_identity_representation_exists() {
        fn accepts_domain(value: &super::super::domain::Domain) {
            assert_eq!(value.namespace(), "zamani");
        }

        let kind = new("zamani", "quantum").expect("valid domain");

        accepts_domain(&kind);
    }
}