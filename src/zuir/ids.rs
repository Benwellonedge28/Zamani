
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR — Canonical Typed Identifiers
//!
//! Path: `src/zuir/ids.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! This module defines strongly typed identifiers for entities in the Zamani
//! Universal Intermediate Representation (ZUIR).
//!
//! Identifiers are intentionally independent of concrete IR nodes, builders,
//! dialect registries, verification, serialization formats, and backends.
//! Higher-level modules may depend on this module; this module must not
//! depend on those higher-level modules.
//!
//! ## OWNS
//!
//! - The canonical ZUIR identifier types.
//! - Nonzero identifier representation and validation.
//! - Identifier allocation and exhaustion behavior.
//! - Type-safe identifier equality, ordering, and hashing.
//! - The distinction between runtime identity and stable content identity.
//!
//! ## DOES NOT OWN
//!
//! - AST node identity or source-span representation.
//! - Quantum logical or physical qubit identifiers.
//! - Target-specific device, register, memory, or hardware identifiers.
//! - Symbol resolution, type definitions, or IR node storage.
//! - Persistent content fingerprints or cryptographic hashes.
//! - Serialization formats for complete ZUIR programs.
//!
//! ## IDENTITY CONTRACT
//!
//! Each entity category has a distinct Rust type. A `BlockId` cannot be
//! passed where an `OperationId` is required without an explicit conversion
//! that the caller must deliberately implement.
//!
//! The numeric value is an opaque identity token, not an array index,
//! hardware address, source location, execution order, or content hash.
//!
//! Identifiers produced by one allocator are unique within that allocator's
//! sequence. Separate allocators are independent and may issue equal numeric
//! values. A complete IR owner must therefore establish the allocation scope
//! it requires and prevent collisions when combining independently allocated
//! entities.
//!
//! ## STABILITY CONTRACT
//!
//! Allocated IDs are process-local identity tokens unless an enclosing
//! representation explicitly persists and restores them. Numeric IDs are not
//! automatically globally unique, deterministic across compilations, or
//! stable across serialization round trips.
//!
//! For stable cross-build identity, use a separately specified canonical
//! content fingerprint or an explicitly persisted identity. Do not treat an
//! allocation counter as a reproducible content hash.
//!
//! ## SCALABILITY CONTRACT
//!
//! No hardware capacity, operation count, qubit count, tensor rank, node
//! count, register width, or device count is imposed by this module.
//!
//! The representation uses a nonzero `u128`, giving a large finite identity
//! space. No finite integer representation can provide literal infinity.
//! Exhaustion is detected and returned as an error; identifiers never wrap
//! around and silently reuse an identity.
//!
//! The allocator uses dynamically synchronized state and requires no unsafe
//! code. Its lock makes concurrent calls through the same allocator safe.
//!
//! ## INTEGRATION CONTRACT
//!
//! Future modules should import only the identifier types they require:
//!
//! - `ir/*`: program, module, function, region, block, operation and value IDs.
//! - `types/*`: type IDs.
//! - `dialect/*`: dialect and extension IDs.
//! - `values/symbols.rs`: symbol IDs.
//! - `source_map.rs`: source IDs when stable source identity is needed.
//!
//! Quantum-domain identities remain owned by `quantum::ir`; this module
//! must not introduce a second `QubitId` or `PhysicalQubitId`.
//!
//! Errors are defined locally so this foundational file does not depend on
//! a future `errors.rs`. A later error facade may re-export these errors,
//! but must not define competing equivalents.
//!
//! ## FREEZE CRITERIA
//!
//! - All identifier categories have distinct types.
//! - Zero cannot be constructed through the public raw-value constructor.
//! - Allocation exhaustion is explicit and never wraps.
//! - Allocator state is safe for concurrent access.
//! - Equality, ordering and hashing operate on the identifier value.
//! - No hardware-specific semantics or dependencies are introduced.
//! - Unit tests cover type distinction, allocation, exhaustion and concurrency.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;
use std::num::NonZeroU128;
use std::sync::Mutex;

/// Error returned when a caller attempts to construct an invalid identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvalidIdValue {
    /// Zero is reserved as an invalid/uninitialized identifier.
    Zero,
}

impl fmt::Display for InvalidIdValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => f.write_str("identifier value must be nonzero"),
        }
    }
}

impl std::error::Error for InvalidIdValue {}

/// Error returned when an identifier allocator has exhausted its value space.
///
/// Exhaustion is permanent for that allocator. The allocator never wraps,
/// resets automatically, or issues a previously issued identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdSpaceExhausted;

impl fmt::Display for IdSpaceExhausted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("identifier allocator exhausted its nonzero u128 value space")
    }
}

impl std::error::Error for IdSpaceExhausted {}

/// Common behavior implemented by all canonical ZUIR ID types.
///
/// Implementations must preserve the nonzero invariant and must not assign
/// meaning to the numeric value beyond identity within the documented scope.
pub trait TypedId:
    Copy + Eq + Ord + std::hash::Hash + fmt::Debug + Send + Sync + 'static
{
    /// Construct an ID from a validated nonzero value.
    ///
    /// This method is public because generic allocators need it. Callers
    /// should normally use the concrete ID type's `new` or `from_raw` API.
    #[doc(hidden)]
    fn from_nonzero(value: NonZeroU128) -> Self;

    /// Return the opaque numeric identity value.
    fn get(self) -> u128;
}

/// Define a distinct, serializable, nonzero ID type.
///
/// `NonZeroU128` also ensures that deserialization through Serde rejects zero
/// instead of silently constructing an invalid identifier.
macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(NonZeroU128);

        impl $name {
            /// Construct an ID from a nonzero numeric value.
            ///
            /// Raw values must be unique in the relevant identity scope.
            /// This function does not establish global uniqueness.
            pub const fn new(value: u128) -> Result<Self, InvalidIdValue> {
                match NonZeroU128::new(value) {
                    Some(value) => Ok(Self(value)),
                    None => Err(InvalidIdValue::Zero),
                }
            }

            /// Return the underlying opaque numeric value.
            pub const fn get(self) -> u128 {
                self.0.get()
            }

            /// Return the numeric value as a nonzero integer.
            pub const fn get_nonzero(self) -> NonZeroU128 {
                self.0
            }
        }

        impl TypedId for $name {
            fn from_nonzero(value: NonZeroU128) -> Self {
                Self(value)
            }

            fn get(self) -> u128 {
                self.0.get()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($name), "({})"), self.get())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.get().fmt(f)
            }
        }

        impl TryFrom<u128> for $name {
            type Error = InvalidIdValue;

            fn try_from(value: u128) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for u128 {
            fn from(value: $name) -> Self {
                value.get()
            }
        }
    };
}

define_id!(
    /// Identity of a ZUIR program.
    ProgramId
);

define_id!(
    /// Identity of a module within a ZUIR program or module namespace.
    ModuleId
);

define_id!(
    /// Identity of a function or other callable declaration.
    FunctionId
);

define_id!(
    /// Identity of a nested computation region.
    RegionId
);

define_id!(
    /// Identity of a basic block or equivalent control-flow unit.
    BlockId
);

define_id!(
    /// Identity of an operation in the IR.
    OperationId
);

define_id!(
    /// Identity of an IR value, including block arguments or operation results.
    ValueId
);

define_id!(
    /// Identity of a type definition in the ZUIR type namespace.
    TypeId
);

define_id!(
    /// Identity of a registered dialect.
    DialectId
);

define_id!(
    /// Identity of a resolved symbol.
    SymbolId
);

define_id!(
    /// Identity of a registered extension or extension manifest.
    ExtensionId
);

define_id!(
    /// Identity of a source artifact tracked by an enclosing source map.
    ///
    /// This identifies the source artifact, not an AST node or byte offset.
    SourceId
);

/// A thread-safe, monotonic allocator for one concrete ID category.
///
/// # Uniqueness scope
///
/// IDs allocated through a single allocator are unique for that allocator's
/// lifetime. Different allocators do not coordinate with one another.
///
/// The owner of a program, module, or registry is responsible for selecting
/// the correct allocator scope and for avoiding collisions when importing or
/// merging entities from independently allocated scopes.
///
/// # Concurrency
///
/// Concurrent calls to [`IdAllocator::try_next`] are serialized by a mutex.
/// The allocator is safe to share between threads when wrapped in an
/// `Arc`, subject to ordinary ownership rules.
///
/// # Exhaustion
///
/// The allocator issues values starting at one and advances monotonically.
/// Once the maximum nonzero `u128` value has been issued, subsequent calls
/// return [`IdSpaceExhausted`]. It never wraps to zero or reuses an ID.
pub struct IdAllocator<T: TypedId> {
    next: Mutex<Option<u128>>,
    marker: PhantomData<fn() -> T>,
}

impl<T: TypedId> IdAllocator<T> {
    /// Create an allocator whose first issued ID is one.
    pub const fn new() -> Self {
        Self {
            next: Mutex::new(Some(1)),
            marker: PhantomData,
        }
    }

    /// Allocate the next identifier.
    ///
    /// Returns an error after the allocator exhausts the nonzero `u128`
    /// identity space. The numeric value is never silently reused.
    pub fn try_next(&self) -> Result<T, IdSpaceExhausted> {
        // Recovering a poisoned mutex is safe here: the only mutation is a
        // single Option<u128> update, and the invariant remains valid even
        // if a thread panicked while holding the lock.
        let mut guard = match self.next.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let current = guard.ok_or(IdSpaceExhausted)?;

        // Prepare the next state before returning the current value.
        // `None` is the permanent exhausted state.
        *guard = current.checked_add(1);

        let nonzero = NonZeroU128::new(current).ok_or(IdSpaceExhausted)?;
        Ok(T::from_nonzero(nonzero))
    }

    /// Return whether this allocator has exhausted its identity space.
    ///
    /// This is an observation only; a concurrent allocation may change the
    /// result immediately after this method returns.
    pub fn is_exhausted(&self) -> bool {
        let guard = match self.next.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.is_none()
    }
}

impl<T: TypedId> Default for IdAllocator<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: TypedId> fmt::Debug for IdAllocator<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdAllocator")
            .field("id_type", &std::any::type_name::<T>())
            .field("exhausted", &self.is_exhausted())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn raw_constructor_rejects_zero() {
        assert_eq!(ProgramId::new(0), Err(InvalidIdValue::Zero));
        assert_eq!(ModuleId::try_from(0), Err(InvalidIdValue::Zero));
    }

    #[test]
    fn raw_constructor_accepts_nonzero_values() {
        let id = ProgramId::new(42).expect("42 is nonzero");
        assert_eq!(id.get(), 42);
        assert_eq!(u128::from(id), 42);
        assert_eq!(id.to_string(), "42");
        assert_eq!(format!("{id:?}"), "ProgramId(42)");
    }

    #[test]
    fn different_id_categories_are_distinct_types() {
        let program = ProgramId::new(7).expect("nonzero");
        let module = ModuleId::new(7).expect("nonzero");

        // Equal numeric values do not make different ID categories equal.
        assert_eq!(program.get(), module.get());
        assert_ne!(format!("{program:?}"), format!("{module:?}"));
    }

    #[test]
    fn allocator_starts_at_one_and_increases() {
        let allocator = IdAllocator::<OperationId>::new();

        assert_eq!(allocator.try_next().expect("first ID").get(), 1);
        assert_eq!(allocator.try_next().expect("second ID").get(), 2);
        assert!(!allocator.is_exhausted());
    }

    #[test]
    fn allocator_instances_have_independent_scopes() {
        let first = IdAllocator::<ValueId>::new();
        let second = IdAllocator::<ValueId>::new();

        assert_eq!(first.try_next().expect("first").get(), 1);
        assert_eq!(second.try_next().expect("first").get(), 1);
    }

    #[test]
    fn allocated_ids_are_unique_within_one_allocator() {
        let allocator = IdAllocator::<BlockId>::new();
        let mut seen = HashSet::new();

        for _ in 0..10_000 {
            let id = allocator.try_next().expect("allocator has capacity");
            assert!(seen.insert(id));
        }
    }

    #[test]
    fn allocator_is_safe_for_concurrent_calls() {
        let allocator = Arc::new(IdAllocator::<ValueId>::new());
        let collected = Arc::new(Mutex::new(Vec::new()));
        let workers = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for _ in 0..workers {
            let allocator = Arc::clone(&allocator);
            let collected = Arc::clone(&collected);

            handles.push(thread::spawn(move || {
                let mut local = Vec::new();

                for _ in 0..1_000 {
                    local.push(allocator.try_next().expect("allocator has capacity"));
                }

                let mut shared = collected
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                shared.extend(local);
            }));
        }

        for handle in handles {
            handle.join().expect("worker must finish");
        }

        let values = collected
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let unique: HashSet<_> = values.iter().copied().collect();
        assert_eq!(unique.len(), values.len());
    }

    #[test]
    fn maximum_nonzero_value_is_a_valid_identity() {
        let id = ProgramId::new(u128::MAX).expect("maximum is nonzero");
        assert_eq!(id.get(), u128::MAX);
    }

    #[test]
    fn identifiers_are_orderable_and_hashable() {
        let lower = OperationId::new(1).expect("nonzero");
        let higher = OperationId::new(2).expect("nonzero");

        assert!(lower < higher);

        let set: HashSet<_> = [lower, higher].into_iter().collect();
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn serde_round_trip_preserves_id_value() {
        let original = FunctionId::new(123).expect("nonzero");
        let encoded = serde_json::to_string(&original).expect("serialize");
        let decoded: FunctionId =
            serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(decoded, original);
        assert_eq!(decoded.get(), 123);
    }

    #[test]
    fn serde_rejects_zero_identifier() {
        let decoded = serde_json::from_str::<ProgramId>("0");
        assert!(decoded.is_err());
    }
}
