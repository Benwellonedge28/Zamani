
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical concurrency: portable atomic-operation contracts.
//!
//! File: `src/classical/concurrency/atomics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: repository-declared Rust 1.97
//!
//! # Purpose
//!
//! This module describes and validates atomic operations independently of
//! their physical realization. It provides:
//!
//! - a canonical operation vocabulary for atomic access;
//! - validation of memory-order constraints;
//! - explicit compare-exchange success/failure ordering;
//! - a backend-facing operation specification;
//! - access to the existing local host atomic wrappers.
//!
//! # Ownership
//!
//! `synchronization.rs` remains the owner of `MemoryOrder`,
//! `SynchronizationError`, `SyncAtomicBool`, and `SyncAtomicUsize`.
//! This module does not redefine them.
//!
//! The frontend AST owns parsed syntax. Semantic analysis resolves atomic
//! targets, validates mutability and types, determines effects and capabilities,
//! and normalizes source-level memory-order intent. The existing canonical IR
//! remains the representation used by compiler transformations.
//!
//! This module is not an AST, IR, scheduler, executor, distributed protocol,
//! hardware abstraction layer, or lock-free guarantee.
//!
//! # POCO-REAF and resources
//!
//! Atomic operations express observable semantics, not a particular CPU,
//! register width, cache protocol, device, or lock-free implementation.
//! Backend capability checks determine whether a target can realize an
//! operation directly or needs a permitted alternative.
//!
//! This module imposes no maximum number of atomic objects, tasks, threads,
//! workers, nodes, or devices. Host representations and actual resource
//! availability remain finite.
//!
//! # Memory-order boundary
//!
//! `MemoryOrder` is defined by `synchronization.rs`. The grammar additionally
//! accepts `consume`; semantic analysis must normalize that source-level order
//! to a supported semantic order before constructing this specification.
//! This module does not silently reinterpret source syntax.
//!
//! An omitted source-level order remains `None` here. The language's semantic
//! specification or a verified lowering policy must determine its meaning.
//!
//! # Integration
//!
//! - `grammar/concurrency/synchronization.g4`: owns atomic source syntax.
//! - `src/frontend/ast/`: owns the parsed representation.
//! - semantic validation: resolves targets and supplies normalized orders.
//! - `synchronization.rs`: owns memory-order types and local host primitives.
//! - canonical IR: preserves synchronization semantics through lowering.
//! - execution planning and target backends: check capabilities and realize
//!   the operation.
//!
//! No runtime execution occurs in this module.

use std::error::Error;
use std::fmt;

pub use super::synchronization::{
    MemoryOrder, SyncAtomicBool, SyncAtomicUsize, SynchronizationError,
};

/// The operation requested by an atomic source construct.
///
/// This describes operation semantics; it does not prescribe a target
/// instruction, register width, lock-free implementation, or backend.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AtomicOperationKind {
    /// Read the current value atomically.
    Load,
    /// Replace the current value atomically.
    Store,
    /// Replace the current value and return its previous value.
    Exchange,
    /// Replace the value only when it equals the expected value.
    CompareExchange,
    /// Atomically add to the current value.
    FetchAdd,
    /// Atomically subtract from the current value.
    FetchSubtract,
    /// Atomically apply bitwise AND.
    FetchAnd,
    /// Atomically apply bitwise OR.
    FetchOr,
    /// Atomically apply bitwise XOR.
    FetchXor,
    /// Atomically select the lesser value.
    FetchMin,
    /// Atomically select the greater value.
    FetchMax,
}

impl AtomicOperationKind {
    /// Whether this operation is a read-only atomic access.
    pub const fn is_load(self) -> bool {
        matches!(self, Self::Load)
    }

    /// Whether this operation is a write-only atomic access.
    pub const fn is_store(self) -> bool {
        matches!(self, Self::Store)
    }

    /// Whether this operation uses compare-exchange success/failure orders.
    pub const fn is_compare_exchange(self) -> bool {
        matches!(self, Self::CompareExchange)
    }

    /// Whether this operation reads and may update the atomic value.
    pub const fn is_read_modify_write(self) -> bool {
        matches!(
            self,
            Self::Exchange
                | Self::CompareExchange
                | Self::FetchAdd
                | Self::FetchSubtract
                | Self::FetchAnd
                | Self::FetchOr
                | Self::FetchXor
                | Self::FetchMin
                | Self::FetchMax
        )
    }
}

/// A validated, backend-neutral atomic-operation specification.
///
/// `success_order` describes the requested ordering for the operation.
/// `failure_order` applies only to compare-exchange and describes the ordering
/// when the expected value does not match.
///
/// This type does not contain a target address, AST node, IR node, device
/// handle, or physical resource identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AtomicOperationSpec {
    kind: AtomicOperationKind,
    success_order: Option<MemoryOrder>,
    failure_order: Option<MemoryOrder>,
}

impl AtomicOperationSpec {
    /// Construct a specification and validate its ordering constraints.
    ///
    /// For non-compare-exchange operations, `failure_order` must be `None`.
    ///
    /// For compare-exchange, an explicit failure order is checked against the
    /// success order. If the success order is omitted but a failure order is
    /// supplied, construction fails because the pair cannot be validated.
    ///
    /// If both orders are omitted, their meaning remains the responsibility of
    /// semantic analysis or the language's specified default-order policy.
    pub fn new(
        kind: AtomicOperationKind,
        success_order: Option<MemoryOrder>,
        failure_order: Option<MemoryOrder>,
    ) -> Result<Self, AtomicOperationSpecError> {
        if kind.is_load() {
            if let Some(order) = success_order {
                if !is_valid_load_order(order) {
                    return Err(AtomicOperationSpecError::InvalidOrder {
                        kind,
                        order,
                    });
                }
            }
        }

        if kind.is_store() {
            if let Some(order) = success_order {
                if !is_valid_store_order(order) {
                    return Err(AtomicOperationSpecError::InvalidOrder {
                        kind,
                        order,
                    });
                }
            }
        }

        if kind.is_compare_exchange() {
            match (success_order, failure_order) {
                (Some(success), Some(failure)) => {
                    if !is_valid_compare_exchange_failure_order(success, failure) {
                        return Err(
                            AtomicOperationSpecError::InvalidCompareExchangeOrders {
                                success,
                                failure,
                            },
                        );
                    }
                }
                (None, Some(_)) => {
                    return Err(
                        AtomicOperationSpecError::FailureOrderWithoutSuccessOrder,
                    );
                }
                _ => {}
            }
        } else if failure_order.is_some() {
            return Err(
                AtomicOperationSpecError::FailureOrderNotApplicable { kind },
            );
        }

        Ok(Self {
            kind,
            success_order,
            failure_order,
        })
    }

    /// Construct an operation with one source/semantic order.
    ///
    /// This is appropriate for load, store, exchange, and fetch operations.
    /// Compare-exchange should use `new` when the success and failure orders
    /// are known separately.
    pub fn with_order(
        kind: AtomicOperationKind,
        order: Option<MemoryOrder>,
    ) -> Result<Self, AtomicOperationSpecError> {
        if kind.is_compare_exchange() {
            return Err(
                AtomicOperationSpecError::CompareExchangeRequiresOrderPair,
            );
        }

        Self::new(kind, order, None)
    }

    /// Return the operation kind.
    pub const fn kind(self) -> AtomicOperationKind {
        self.kind
    }

    /// Return the requested success/operation order, if specified.
    pub const fn success_order(self) -> Option<MemoryOrder> {
        self.success_order
    }

    /// Return the compare-exchange failure order, if specified.
    pub const fn failure_order(self) -> Option<MemoryOrder> {
        self.failure_order
    }

    /// Whether the operation has an explicit ordering contract.
    ///
    /// For compare-exchange, this is true when the success order is present;
    /// a missing failure order may still need to be derived by a documented
    /// semantic policy before lowering.
    pub const fn has_explicit_order(self) -> bool {
        self.success_order.is_some()
    }

    /// Whether compare-exchange still needs its failure order resolved.
    pub const fn needs_failure_order_resolution(self) -> bool {
        self.kind.is_compare_exchange()
            && self.success_order.is_some()
            && self.failure_order.is_none()
    }
}

/// Errors raised when constructing an atomic-operation specification.
///
/// These are contract-validation errors, not runtime hardware errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AtomicOperationSpecError {
    /// The requested order is invalid for the operation.
    InvalidOrder {
        kind: AtomicOperationKind,
        order: MemoryOrder,
    },
    /// The compare-exchange failure order is not permitted with its success
    /// order.
    InvalidCompareExchangeOrders {
        success: MemoryOrder,
        failure: MemoryOrder,
    },
    /// A failure order was provided for an operation other than
    /// compare-exchange.
    FailureOrderNotApplicable {
        kind: AtomicOperationKind,
    },
    /// A failure order cannot be validated without a success order.
    FailureOrderWithoutSuccessOrder,
    /// Compare-exchange requires the two-order constructor.
    CompareExchangeRequiresOrderPair,
}

impl fmt::Display for AtomicOperationSpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOrder { kind, order } => {
                write!(f, "memory order {order:?} is invalid for atomic {kind:?}")
            }
            Self::InvalidCompareExchangeOrders { success, failure } => {
                write!(
                    f,
                    "compare-exchange failure order {failure:?} is invalid \
                     with success order {success:?}"
                )
            }
            Self::FailureOrderNotApplicable { kind } => {
                write!(
                    f,
                    "failure memory order is not applicable to atomic {kind:?}"
                )
            }
            Self::FailureOrderWithoutSuccessOrder => {
                f.write_str(
                    "compare-exchange failure order requires a success order",
                )
            }
            Self::CompareExchangeRequiresOrderPair => {
                f.write_str(
                    "compare-exchange requires the constructor with success \
                     and failure orders",
                )
            }
        }
    }
}

impl Error for AtomicOperationSpecError {}

/// Validate the legal orderings for an atomic load.
///
/// Acquire and sequentially consistent loads are allowed; release semantics
/// cannot be expressed by a load.
const fn is_valid_load_order(order: MemoryOrder) -> bool {
    matches!(
        order,
        MemoryOrder::Relaxed
            | MemoryOrder::Acquire
            | MemoryOrder::SequentiallyConsistent
    )
}

/// Validate the legal orderings for an atomic store.
///
/// Release and sequentially consistent stores are allowed; acquire semantics
/// cannot be expressed by a store.
const fn is_valid_store_order(order: MemoryOrder) -> bool {
    matches!(
        order,
        MemoryOrder::Relaxed
            | MemoryOrder::Release
            | MemoryOrder::SequentiallyConsistent
    )
}

/// Validate compare-exchange failure ordering against the success ordering.
///
/// Failure ordering cannot contain release semantics and cannot be stronger
/// than the success ordering. This follows the ordering constraints used by
/// Rust's standard atomic compare-exchange operations.
const fn is_valid_compare_exchange_failure_order(
    success: MemoryOrder,
    failure: MemoryOrder,
) -> bool {
    match success {
        MemoryOrder::Relaxed | MemoryOrder::Release => {
            matches!(failure, MemoryOrder::Relaxed)
        }
        MemoryOrder::Acquire | MemoryOrder::AcquireRelease => {
            matches!(failure, MemoryOrder::Relaxed | MemoryOrder::Acquire)
        }
        MemoryOrder::SequentiallyConsistent => matches!(
            failure,
            MemoryOrder::Relaxed
                | MemoryOrder::Acquire
                | MemoryOrder::SequentiallyConsistent
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_relaxed_load() {
        let spec = AtomicOperationSpec::with_order(
            AtomicOperationKind::Load,
            Some(MemoryOrder::Relaxed),
        )
        .expect("relaxed load is valid");

        assert_eq!(spec.kind(), AtomicOperationKind::Load);
        assert_eq!(spec.success_order(), Some(MemoryOrder::Relaxed));
        assert!(!spec.needs_failure_order_resolution());
    }

    #[test]
    fn rejects_release_load() {
        assert_eq!(
            AtomicOperationSpec::with_order(
                AtomicOperationKind::Load,
                Some(MemoryOrder::Release),
            ),
            Err(AtomicOperationSpecError::InvalidOrder {
                kind: AtomicOperationKind::Load,
                order: MemoryOrder::Release,
            })
        );
    }

    #[test]
    fn rejects_acquire_store() {
        assert_eq!(
            AtomicOperationSpec::with_order(
                AtomicOperationKind::Store,
                Some(MemoryOrder::Acquire),
            ),
            Err(AtomicOperationSpecError::InvalidOrder {
                kind: AtomicOperationKind::Store,
                order: MemoryOrder::Acquire,
            })
        );
    }

    #[test]
    fn accepts_release_store() {
        assert!(AtomicOperationSpec::with_order(
            AtomicOperationKind::Store,
            Some(MemoryOrder::Release),
        )
        .is_ok());
    }

    #[test]
    fn accepts_compare_exchange_acq_rel_acquire() {
        let spec = AtomicOperationSpec::new(
            AtomicOperationKind::CompareExchange,
            Some(MemoryOrder::AcquireRelease),
            Some(MemoryOrder::Acquire),
        )
        .expect("valid compare-exchange ordering");

        assert_eq!(
            spec.success_order(),
            Some(MemoryOrder::AcquireRelease)
        );
        assert_eq!(spec.failure_order(), Some(MemoryOrder::Acquire));
    }

    #[test]
    fn rejects_release_failure_order() {
        assert_eq!(
            AtomicOperationSpec::new(
                AtomicOperationKind::CompareExchange,
                Some(MemoryOrder::SequentiallyConsistent),
                Some(MemoryOrder::Release),
            ),
            Err(AtomicOperationSpecError::InvalidCompareExchangeOrders {
                success: MemoryOrder::SequentiallyConsistent,
                failure: MemoryOrder::Release,
            })
        );
    }

    #[test]
    fn rejects_failure_order_without_success_order() {
        assert_eq!(
            AtomicOperationSpec::new(
                AtomicOperationKind::CompareExchange,
                None,
                Some(MemoryOrder::Relaxed),
            ),
            Err(
                AtomicOperationSpecError::FailureOrderWithoutSuccessOrder
            )
        );
    }

    #[test]
    fn rejects_failure_order_for_fetch_operation() {
        assert_eq!(
            AtomicOperationSpec::new(
                AtomicOperationKind::FetchAdd,
                Some(MemoryOrder::Relaxed),
                Some(MemoryOrder::Relaxed),
            ),
            Err(AtomicOperationSpecError::FailureOrderNotApplicable {
                kind: AtomicOperationKind::FetchAdd,
            })
        );
    }

    #[test]
    fn omitted_order_remains_unspecified() {
        let spec = AtomicOperationSpec::with_order(
            AtomicOperationKind::Exchange,
            None,
        )
        .expect("an unspecified order is represented explicitly");

        assert_eq!(spec.success_order(), None);
        assert!(!spec.has_explicit_order());
    }

    #[test]
    fn compare_exchange_without_orders_needs_no_implicit_guess() {
        let spec = AtomicOperationSpec::new(
            AtomicOperationKind::CompareExchange,
            None,
            None,
        )
        .expect("unspecified orders are preserved");

        assert_eq!(spec.success_order(), None);
        assert_eq!(spec.failure_order(), None);
        assert!(!spec.needs_failure_order_resolution());
    }

    #[test]
    fn identifies_read_modify_write_operations() {
        assert!(AtomicOperationKind::Exchange.is_read_modify_write());
        assert!(AtomicOperationKind::FetchAdd.is_read_modify_write());
        assert!(AtomicOperationKind::CompareExchange.is_read_modify_write());
        assert!(!AtomicOperationKind::Load.is_read_modify_write());
        assert!(!AtomicOperationKind::Store.is_read_modify_write());
    }

    #[test]
    fn reuses_existing_local_atomic_primitives() {
        let flag = SyncAtomicBool::new(false);

        flag.store(true, MemoryOrder::Release)
            .expect("valid store");

        assert!(flag
            .load(MemoryOrder::Acquire)
            .expect("valid load"));

        let counter = SyncAtomicUsize::new(4);
        assert_eq!(
            counter.checked_fetch_add(3),
            Some(4),
            "checked_fetch_add returns the prior value when successful"
        );
        assert_eq!(
            counter.load(MemoryOrder::SequentiallyConsistent),
            Ok(7)
        );
    }
}
