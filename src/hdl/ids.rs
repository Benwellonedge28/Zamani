
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDL identity infrastructure
//!
//! Path: `src/hdl/ids.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Foundation contract; freeze after integration tests pass.
//!
//! ## Purpose
//!
//! Provide explicit, strongly typed identities for HDL-owned entities while
//! reusing canonical AST and semantic identities wherever they already exist.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/frontend/semantic/model/ids.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! The grammar and specifications define meaning and ownership. The AST owns
//! syntax-node identity. The semantic model owns semantic identity types and
//! checked identity allocation. This module adds only HDL-specific identity
//! types that have no equivalent in those canonical layers.
//!
//! ## Canonical identity reuse
//!
//! - `HdlAstNodeId` aliases the canonical AST `NodeId`.
//! - `HdlDeclarationId` aliases the semantic `SymbolId`.
//! - `HdlModuleId` aliases the semantic `ModuleId`.
//! - `HdlDesignId` aliases the generic semantic `SemanticId`.
//! - HDL instances, clock domains, and libraries have distinct HDL-owned ID
//!   types because they do not have corresponding dedicated types in the
//!   canonical semantic identity catalogue inspected for this implementation.
//!
//! These aliases are intentional: they do not create new identity systems.
//! They also do not make an AST node, declaration, module, or design
//! interchangeable with one another when their canonical types differ.
//!
//! ## Owns
//!
//! - Public HDL aliases for canonical identity types.
//! - Distinct numeric IDs for HDL-owned instances, clock domains, and libraries.
//! - Type-compatible use of the canonical semantic ID allocator.
//! - Tests for identity separation, deterministic allocation, and exhaustion.
//!
//! ## Does not own
//!
//! - Source-level identifier spelling or name resolution.
//! - AST `NodeId` allocation or semantic `SymbolId`/`ModuleId` allocation.
//! - A replacement semantic ID allocator.
//! - Source spans, source maps, diagnostics, or provenance records.
//! - Physical pins, wires, registers, cells, devices, runtime handles, or
//!   vendor-specific resource identities.
//! - HDL intermediate representation implementation. All HDLIR implementation
//!   remains under `src/hdl/hdlir/`.
//!
//! ## Identity scope and stability
//!
//! An ID identifies an entity only within the namespace and lifetime
//! established by the owning design, registry, or compilation context.
//! Numeric equality between IDs allocated by separate allocator instances
//! does not establish that the entities are the same.
//!
//! IDs are deterministic for the same allocator starting point and allocation
//! order. They are not promised to remain stable across unrelated builds,
//! changed traversal order, or changed source programs. Persistent identities
//! require a separately specified, versioned persistence policy.
//!
//! ## Scalability
//!
//! This module imposes no language-level limits on the number of modules,
//! declarations, instances, clock domains, libraries, ports, signals, or
//! physical resources a design may describe. The canonical semantic allocator
//! uses checked `u64` identity values and reports exhaustion without wrapping
//! or reusing an issued ID. This representational boundary is not a hardware
//! capacity limit.
//!
//! Allocation is explicit and caller-owned. There is no global allocator,
//! pointer-derived identity, address-based identity, or hidden shared state.
//! The owning compiler remains responsible for operational budgets and for
//! reporting incomplete work when resources are exhausted.
//!
//! ## Integration
//!
//! The HDL composition root may expose this module after the canonical
//! `frontend` and semantic model modules are reachable from the crate root.
//! Design declarations should use the canonical aliases for AST nodes,
//! declarations, modules, and generic semantic entities. HDL instance,
//! clock-domain, and library owners may use the dedicated types and instantiate
//! `SemanticIdAllocator` for each relevant namespace.
//!
//! This module intentionally does not depend on the HDL validator's textual
//! `HdlId`: a source-level name is not a unique internal entity identity.
//!
//! ## Freeze criteria
//!
//! - Canonical identities are reused rather than redeclared.
//! - Every new identity type has one clear HDL-owned meaning.
//! - Allocation is explicit, deterministic, checked, and non-wrapping.
//! - No ID is treated as a persistent cross-build identifier without a
//!   separately specified persistence contract.
//! - The crate module tree compiles and the focused tests pass.

use core::fmt;
use core::marker::PhantomData;
use core::num::NonZeroU64;
use core::str::FromStr;

pub use crate::frontend::ast::node::node_id::NodeId as HdlAstNodeId;
pub use crate::frontend::semantic::model::ids::{
    ModuleId as HdlModuleId,
    SemanticId as HdlDesignId,
    SemanticIdAllocationError as HdlIdAllocationError,
    SemanticIdAllocator,
    SemanticIdParseError as HdlIdParseError,
    SemanticIdValue,
    SymbolId as HdlDeclarationId,
};

/// The first valid numeric identity in the HDL-owned identity namespaces.
const FIRST_ID: u64 = 1;

/// Declare a distinct HDL-owned ID while preserving the canonical semantic
/// allocator contract.
macro_rules! define_hdl_id {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        ///
        /// This ID is an internal logical identity. It is not a source name,
        /// AST node ID, semantic declaration ID, physical address, vendor
        /// resource number, or runtime handle.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(NonZeroU64);

        impl $name {
            /// Construct an ID from a non-zero raw value.
            ///
            /// Returns `None` for zero, which is reserved as an invalid or
            /// unassigned value.
            #[must_use]
            pub const fn new(raw: u64) -> Option<Self> {
                match NonZeroU64::new(raw) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Return the numeric representation of this ID.
            #[must_use]
            pub const fn get(self) -> u64 {
                self.0.get()
            }

            /// Return whether this ID has the first valid numeric value.
            #[must_use]
            pub const fn is_first(self) -> bool {
                self.get() == FIRST_ID
            }

            /// Convert the ID to a host collection index when representable.
            ///
            /// An ID is not inherently a collection index. Callers must also
            /// validate the relevant collection's bounds and namespace.
            #[must_use]
            pub fn try_as_usize(self) -> Option<usize> {
                usize::try_from(self.get()).ok()
            }
        }

        impl SemanticIdValue for $name {
            #[doc(hidden)]
            fn from_nonzero(value: NonZeroU64) -> Self {
                Self(value)
            }

            fn get(self) -> u64 {
                self.get()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&self.get())
                    .finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}", self.get())
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.get()
            }
        }

        impl TryFrom<u64> for $name {
            type Error = HdlIdParseError;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                Self::from_raw(value)
            }
        }

        impl FromStr for $name {
            type Err = HdlIdParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let raw = value
                    .parse::<u64>()
                    .map_err(HdlIdParseError::InvalidInteger)?;
                Self::from_raw(raw)
            }
        }
    };
}

define_hdl_id!(
    HdlInstanceId,
    "Identity of one instance of an HDL module or component."
);

define_hdl_id!(
    HdlClockDomainId,
    "Identity of a logical HDL clock domain."
);

define_hdl_id!(
    HdlLibraryId,
    "Identity of a logical HDL library or library registry entry."
);

/// Identity namespace kinds for documentation, logging, and external adapters.
///
/// These labels describe identity categories; they are not ID values and must
/// not be used as a replacement for the distinct Rust ID types.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlIdentityKind {
    /// Canonical source AST node.
    AstNode,
    /// Semantic design entity using the generic semantic identity.
    Design,
    /// Semantic module.
    Module,
    /// Semantic declaration or symbol.
    Declaration,
    /// HDL module/component instance.
    Instance,
    /// HDL logical clock domain.
    ClockDomain,
    /// HDL library or library registry entry.
    Library,
}

impl HdlIdentityKind {
    /// Return the stable lowercase diagnostic/logging label for this category.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AstNode => "ast_node",
            Self::Design => "design",
            Self::Module => "module",
            Self::Declaration => "declaration",
            Self::Instance => "instance",
            Self::ClockDomain => "clock_domain",
            Self::Library => "library",
        }
    }
}

/// Construct a checked allocator for HDL instance identities.
///
/// Each allocator must be owned by the context whose uniqueness boundary it
/// represents. Separate instance registries should not share or merge numeric
/// IDs without an explicit namespace/remapping policy.
#[must_use]
pub fn instance_id_allocator() -> SemanticIdAllocator<HdlInstanceId> {
    SemanticIdAllocator::new()
}

/// Construct a checked allocator for HDL clock-domain identities.
#[must_use]
pub fn clock_domain_id_allocator() -> SemanticIdAllocator<HdlClockDomainId> {
    SemanticIdAllocator::new()
}

/// Construct a checked allocator for HDL library identities.
#[must_use]
pub fn library_id_allocator() -> SemanticIdAllocator<HdlLibraryId> {
    SemanticIdAllocator::new()
}

/// Assert at compile time that an ID implements the semantic allocator's
/// required value contract.
fn assert_semantic_id_value<I: SemanticIdValue>() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_identity_aliases_are_available() {
        // These type annotations are intentional compile-time integration
        // checks. The aliases must remain the repository's canonical types.
        let _: Option<HdlAstNodeId> = HdlAstNodeId::new(1);
        let _: Option<HdlModuleId> = HdlModuleId::new(1);
        let _: Option<HdlDeclarationId> = HdlDeclarationId::new(1);
        let _: Option<HdlDesignId> = HdlDesignId::new(1);
    }

    #[test]
    fn hdl_owned_ids_are_distinct_types() {
        let instance = HdlInstanceId::new(1).expect("non-zero instance ID");
        let clock = HdlClockDomainId::new(1).expect("non-zero clock-domain ID");
        let library = HdlLibraryId::new(1).expect("non-zero library ID");

        assert_eq!(instance.get(), 1);
        assert_eq!(clock.get(), 1);
        assert_eq!(library.get(), 1);

        // Equal raw numbers do not make different identity categories equal.
        assert_eq!(instance.to_string(), clock.to_string());
        assert_eq!(instance.to_string(), library.to_string());
    }

    #[test]
    fn zero_is_rejected() {
        assert!(HdlInstanceId::new(0).is_none());
        assert!(HdlClockDomainId::new(0).is_none());
        assert!(HdlLibraryId::new(0).is_none());
    }

    #[test]
    fn ids_parse_and_display_without_truncation() {
        let raw = u64::MAX.to_string();
        let parsed: HdlInstanceId = raw.parse().expect("maximum u64 is representable");

        assert_eq!(parsed.get(), u64::MAX);
        assert_eq!(parsed.to_string(), raw);
        assert_eq!(parsed.try_as_usize(), usize::try_from(u64::MAX).ok());
    }

    #[test]
    fn invalid_id_text_is_reported() {
        assert!("not-an-id".parse::<HdlInstanceId>().is_err());
        assert!("0".parse::<HdlInstanceId>().is_err());
        assert!("-1".parse::<HdlInstanceId>().is_err());
    }

    #[test]
    fn allocator_is_explicit_and_deterministic() {
        let mut first = instance_id_allocator();
        let mut second = instance_id_allocator();

        assert_eq!(first.allocate().expect("first ID"), HdlInstanceId::new(1).unwrap());
        assert_eq!(first.allocate().expect("second ID"), HdlInstanceId::new(2).unwrap());

        assert_eq!(second.allocate().expect("same allocation order"), HdlInstanceId::new(1).unwrap());
        assert_eq!(first.allocated_count(), 2);
        assert_eq!(second.allocated_count(), 1);
    }

    #[test]
    fn identity_kinds_have_stable_labels() {
        assert_eq!(HdlIdentityKind::AstNode.as_str(), "ast_node");
        assert_eq!(HdlIdentityKind::ClockDomain.as_str(), "clock_domain");
        assert_eq!(HdlIdentityKind::Library.as_str(), "library");
    }

    #[test]
    fn all_hdl_owned_ids_satisfy_the_canonical_allocator_contract() {
        assert_semantic_id_value::<HdlInstanceId>();
        assert_semantic_id_value::<HdlClockDomainId>();
        assert_semantic_id_value::<HdlLibraryId>();
    }
}
