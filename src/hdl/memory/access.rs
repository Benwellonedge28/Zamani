
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Memory Access Analysis
//!
//! Path: `src/hdl/memory/access.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Initial target-independent implementation. Freeze after integration with
//! the memory module, AST adapter, semantic diagnostics, and workspace tests.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/memories.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/memory/model.rs`
//! - `src/zuir/types/type_def.rs`
//!
//! The grammar owns memory-access syntax. The canonical AST owns expressions
//! and source identities. The shared semantic system owns name resolution,
//! expression evaluation, and type checking. `model.rs` owns the logical
//! memory declaration. This file owns access-request validation and static
//! address/collision assessment.
//!
//! ## Owns
//!
//! - Target-independent logical read, write, and read/write requests.
//! - Preservation of canonical source-expression references.
//! - Memory-port permission checks.
//! - Address-rank and statically decidable bounds checks.
//! - Explicit unresolved results for symbolic or dynamic addresses.
//! - Conservative analysis of potential simultaneous-access collisions.
//!
//! ## Does not own
//!
//! - Parsing or constructing a replacement AST.
//! - Symbol resolution or general expression evaluation.
//! - A replacement type or width system.
//! - Memory contents, storage allocation, or execution.
//! - Arbitration, scheduling, or simulation event ordering.
//! - Read-during-write or write-during-write observable behavior.
//! - Physical memory ports, banks, addresses, controllers, or caches.
//! - Synthesis, placement, routing, or target resource allocation.
//! - HDLIR, its builders, passes, verifiers, serialization, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration contract
//!
//! The AST/semantic adapter constructs a `MemoryAccessRequest` using:
//!
//! - the canonical `MemoryId`;
//! - a declared `MemoryPortId`;
//! - the source operation;
//! - one address term per logical memory dimension;
//! - the canonical AST node for the access, when available.
//!
//! The adapter must evaluate expressions only through the canonical
//! expression/constant-evaluation facilities. It must not guess unresolved
//! values or convert negative, oversized, or symbolic expressions by
//! truncation.
//!
//! `validate_access` checks the request against `HdlMemory`. A successful
//! result means only that local permissions and statically decidable address
//! checks passed. A result requiring evaluation is NOT a proof of validity.
//!
//! `analyze_collision` reports whether two requests may overlap. The future
//! `semantics.rs` must decide their observable behavior according to the
//! language specification; this file never invents memory timing semantics.
//!
//! ## Scalability
//!
//! No universal maximum is imposed on address width, memory dimensions, or
//! access count. Addresses use `u128` only for already-concrete non-negative
//! integer literals represented by this API; symbolic values are preserved
//! using the canonical `SymbolicExpression` type. The API does not claim that
//! every source integer fits in `u128`: an AST adapter must retain larger
//! integer expressions symbolically or report evaluation/conversion failure.
//!
//! Collections grow according to available resources. Resource exhaustion
//! remains an operational failure, not a language-level memory limit.
//!
//! ## Determinism
//!
//! Validation follows source dimension order and returns stable typed results.
//! Collision analysis is independent of physical port numbering and target
//! discovery. Unknown overlap is never silently treated as disjoint.
//!
//! ## Freeze criteria
//!
//! - Access requests reuse the canonical logical memory and port identities.
//! - Symbolic addresses and extents remain symbolic.
//! - Permission violations and static out-of-bounds addresses are explicit.
//! - Incomplete analysis is never reported as proof of validity.
//! - Collision analysis does not define execution order or memory behavior.
//! - No physical allocation or HDLIR dependency is introduced.
//! - Unit tests and full workspace checks pass.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::hdl::memory::model::{
    HdlMemory, MemoryId, MemoryModelError, MemoryPortAccess, MemoryPortId,
};
use crate::zuir::types::Dimension;
use crate::zuir::types::SymbolicExpression;

/// A logical memory access operation.
///
/// Built-in operations have explicit permission and collision rules.
/// Extension operations remain open-world: their semantics must be supplied
/// by an appropriate registered semantic provider before they can be
/// considered fully validated.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MemoryAccessOperation {
    /// Read one element without modifying memory contents.
    Read,

    /// Write one element.
    Write,

    /// Read and write according to the owning language's defined semantics.
    ///
    /// This variant does not define whether the read observes the old value,
    /// the new value, or an unspecified value when both actions address the
    /// same element in one logical event.
    ReadWrite,

    /// An extension-defined operation, identified by a contextual name.
    ///
    /// The grammar intentionally allows access declarations to remain
    /// extensible. This variant preserves an operation that is not one of the
    /// built-in operations above; it does not assert that the operation is
    /// supported or valid for every port.
    Extension(String),
}

impl MemoryAccessOperation {
    /// Return whether the built-in operation reads memory.
    ///
    /// `None` means that the extension operation's behavior is unknown here.
    #[must_use]
    pub const fn reads(&self) -> Option<bool> {
        match self {
            Self::Read | Self::ReadWrite => Some(true),
            Self::Write => Some(false),
            Self::Extension(_) => None,
        }
    }

    /// Return whether the built-in operation writes memory.
    ///
    /// `None` means that the extension operation's behavior is unknown here.
    #[must_use]
    pub const fn writes(&self) -> Option<bool> {
        match self {
            Self::Write | Self::ReadWrite => Some(true),
            Self::Read => Some(false),
            Self::Extension(_) => None,
        }
    }
}

/// One coordinate of a logical memory address.
///
/// A multidimensional memory is addressed with one coordinate per declared
/// dimension. Flattening, strides, physical address generation, and layout
/// are intentionally outside this module.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MemoryAddressTerm {
    /// A concrete non-negative integer coordinate.
    ///
    /// This is a mathematical address value represented within `u128`, not a
    /// promise that the target uses a `u128` address or register.
    Constant(u128),

    /// A canonical symbolic expression whose value is not yet known.
    Symbolic(SymbolicExpression),

    /// A dynamic expression or address whose value is not available to this
    /// static analysis.
    ///
    /// The optional node identifies the source expression in the canonical
    /// AST. It is not an expression tree owned by this module.
    Dynamic {
        /// Canonical source expression, if available.
        source: Option<NodeId>,
    },
}

impl MemoryAddressTerm {
    /// Construct a dynamic address with optional canonical source provenance.
    #[must_use]
    pub const fn dynamic(source: Option<NodeId>) -> Self {
        Self::Dynamic { source }
    }

    /// Return the concrete coordinate if this term is a constant.
    #[must_use]
    pub const fn as_constant(&self) -> Option<u128> {
        match self {
            Self::Constant(value) => Some(*value),
            Self::Symbolic(_) | Self::Dynamic { .. } => None,
        }
    }
}

/// A source-level logical memory access request.
///
/// `address` must contain one term for each declared memory dimension.
/// Requests preserve source provenance but do not execute or allocate memory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryAccessRequest {
    memory: MemoryId,
    port: MemoryPortId,
    operation: MemoryAccessOperation,
    address: Vec<MemoryAddressTerm>,
    source: Option<NodeId>,
}

impl MemoryAccessRequest {
    /// Construct a logical memory access request.
    ///
    /// This constructor preserves the supplied values. Port compatibility,
    /// address rank, and bounds are checked by [`validate_access`].
    #[must_use]
    pub fn new(
        memory: MemoryId,
        port: MemoryPortId,
        operation: MemoryAccessOperation,
        address: Vec<MemoryAddressTerm>,
    ) -> Self {
        Self {
            memory,
            port,
            operation,
            address,
            source: None,
        }
    }

    /// Attach the canonical AST node representing this access.
    #[must_use]
    pub fn with_source(mut self, source: NodeId) -> Self {
        self.source = Some(source);
        self
    }

    /// Return the logical memory identity.
    #[must_use]
    pub const fn memory(&self) -> &MemoryId {
        &self.memory
    }

    /// Return the logical port identity.
    #[must_use]
    pub const fn port(&self) -> &MemoryPortId {
        &self.port
    }

    /// Return the operation being requested.
    #[must_use]
    pub const fn operation(&self) -> &MemoryAccessOperation {
        &self.operation
    }

    /// Return the address coordinates in dimension order.
    #[must_use]
    pub fn address(&self) -> &[MemoryAddressTerm] {
        &self.address
    }

    /// Return the canonical source node, if available.
    #[must_use]
    pub const fn source(&self) -> Option<&NodeId> {
        self.source.as_ref()
    }
}

/// Why a static address check cannot establish a definite result.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum IndeterminateReason {
    /// The coordinate is symbolic and has not been evaluated.
    SymbolicAddress {
        /// Zero-based dimension index.
        dimension: usize,
    },

    /// The coordinate is dynamic or otherwise unavailable.
    DynamicAddress {
        /// Zero-based dimension index.
        dimension: usize,
    },

    /// The declared extent is symbolic.
    SymbolicExtent {
        /// Zero-based dimension index.
        dimension: usize,
    },

    /// The declared extent is dynamic.
    DynamicExtent {
        /// Zero-based dimension index.
        dimension: usize,
    },

    /// A registered semantic provider must interpret this operation.
    ExtensionOperation {
        /// The unmodified extension operation name.
        name: String,
    },
}

/// Static assessment of one address coordinate.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DimensionAddressCheck {
    /// The concrete coordinate is within the concrete extent.
    InBounds {
        /// Zero-based dimension index.
        dimension: usize,
        /// Concrete coordinate.
        index: u128,
        /// Concrete, exclusive upper bound.
        extent: u128,
    },

    /// Static analysis cannot decide whether the coordinate is in bounds.
    RequiresEvaluation {
        /// Zero-based dimension index.
        dimension: usize,
        /// Reason static analysis is incomplete.
        reason: IndeterminateReason,
    },
}

/// Overall certainty of the static address check.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccessCertainty {
    /// All dimensions were concretely checked and are in bounds.
    StaticallyInBounds,

    /// No known violation was found, but one or more checks remain unresolved.
    ///
    /// This result is deliberately not named `Valid`: a caller must resolve
    /// or explicitly propagate the uncertainty before claiming validity.
    RequiresEvaluation {
        /// Unresolved conditions, in deterministic dimension order.
        reasons: Vec<IndeterminateReason>,
    },

    /// The operation itself requires extension-defined semantic validation.
    ///
    /// Address checks may still have been performed where possible.
    RequiresExtensionSemantics {
        /// The unmodified extension operation name.
        name: String,
        /// Other unresolved address conditions, if any.
        address_reasons: Vec<IndeterminateReason>,
    },
}

/// Result of checking a request against a logical memory declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryAccessAssessment {
    /// Memory being accessed.
    pub memory: MemoryId,

    /// Declared logical port being used.
    pub port: MemoryPortId,

    /// Requested operation.
    pub operation: MemoryAccessOperation,

    /// Per-dimension address checks, in declaration order.
    pub dimensions: Vec<DimensionAddressCheck>,

    /// Overall static certainty.
    pub certainty: AccessCertainty,
}

/// Errors indicating that a memory access request violates a known invariant.
///
/// Indeterminate symbolic cases are not errors: they are represented in
/// [`MemoryAccessAssessment`] and must not be silently treated as success.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MemoryAccessError {
    /// The request refers to a different logical memory.
    MemoryIdentityMismatch {
        /// Identity of the declared memory.
        declared: MemoryId,
        /// Identity in the access request.
        requested: MemoryId,
    },

    /// The request names no port declared by this memory.
    UnknownPort {
        /// Requested logical port.
        port: MemoryPortId,
    },

    /// The port does not permit the requested built-in operation.
    IncompatiblePortAccess {
        /// Port used by the request.
        port: MemoryPortId,
        /// Requested operation.
        operation: MemoryAccessOperation,
        /// Access permissions declared for the port.
        declared_access: MemoryPortAccess,
    },

    /// The address contains a different number of coordinates than dimensions.
    AddressRankMismatch {
        /// Number of dimensions declared by the memory.
        expected: usize,
        /// Number of coordinates supplied by the request.
        actual: usize,
    },

    /// A dimension has a concrete zero extent.
    ///
    /// A zero extent cannot contain a valid coordinate. This is a declaration
    /// or semantic-model error, not a target-capacity limitation.
    ZeroExtent {
        /// Zero-based dimension index.
        dimension: usize,
    },

    /// A concrete coordinate is outside its declared concrete extent.
    OutOfBounds {
        /// Zero-based dimension index.
        dimension: usize,
        /// Concrete coordinate.
        index: u128,
        /// Concrete, exclusive upper bound.
        extent: u128,
    },

    /// The memory model failed its own local invariants.
    InvalidMemoryModel(MemoryModelError),
}

impl fmt::Display for MemoryAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MemoryIdentityMismatch { declared, requested } => write!(
                f,
                "memory access identity `{requested}` does not match declared memory `{declared}`"
            ),
            Self::UnknownPort { port } => {
                write!(f, "memory access references undeclared port `{port}`")
            }
            Self::IncompatiblePortAccess {
                port,
                operation,
                declared_access,
            } => write!(
                f,
                "operation `{operation}` is incompatible with port `{port}` declared as {declared_access:?}"
            ),
            Self::AddressRankMismatch { expected, actual } => write!(
                f,
                "memory address has {actual} coordinates, but the declaration has {expected} dimensions"
            ),
            Self::ZeroExtent { dimension } => {
                write!(f, "memory dimension {dimension} has zero extent")
            }
            Self::OutOfBounds {
                dimension,
                index,
                extent,
            } => write!(
                f,
                "memory coordinate {index} is outside dimension {dimension} with extent {extent}"
            ),
            Self::InvalidMemoryModel(error) => {
                write!(f, "invalid logical memory model: {error}")
            }
        }
    }
}

impl Error for MemoryAccessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidMemoryModel(error) => Some(error),
            _ => None,
        }
    }
}

/// Validate one logical access against its declared memory.
///
/// The function validates local model invariants, memory identity, port
/// permissions, address rank, and every statically decidable bounds check.
///
/// A returned [`AccessCertainty::RequiresEvaluation`] or
/// [`AccessCertainty::RequiresExtensionSemantics`] is not proof of validity.
/// The canonical expression evaluator and any required extension provider
/// remain responsible for resolving those cases.
///
/// # Errors
///
/// Returns a typed error for a known violation, including a missing port,
/// incompatible built-in operation, rank mismatch, zero extent, or concrete
/// out-of-bounds coordinate.
pub fn validate_access(
    memory: &HdlMemory,
    request: &MemoryAccessRequest,
) -> Result<MemoryAccessAssessment, MemoryAccessError> {
    memory
        .validate()
        .map_err(MemoryAccessError::InvalidMemoryModel)?;

    if memory.id() != request.memory() {
        return Err(MemoryAccessError::MemoryIdentityMismatch {
            declared: memory.id().clone(),
            requested: request.memory().clone(),
        });
    }

    let port = memory
        .ports()
        .iter()
        .find(|candidate| candidate.id() == request.port())
        .ok_or_else(|| MemoryAccessError::UnknownPort {
            port: request.port().clone(),
        })?;

    validate_port_permission(port.access(), request)?;

    if request.address().len() != memory.dimensions().len() {
        return Err(MemoryAccessError::AddressRankMismatch {
            expected: memory.dimensions().len(),
            actual: request.address().len(),
        });
    }

    let mut checks = Vec::with_capacity(memory.dimensions().len());
    let mut unresolved = Vec::new();

    for (dimension, (extent, address)) in memory
        .dimensions()
        .iter()
        .zip(request.address().iter())
        .enumerate()
    {
        let check = check_dimension(dimension, extent, address)?;

        if let DimensionAddressCheck::RequiresEvaluation { reason, .. } = &check {
            unresolved.push(reason.clone());
        }

        checks.push(check);
    }

    let certainty = match &request.operation {
        MemoryAccessOperation::Extension(name) => {
            AccessCertainty::RequiresExtensionSemantics {
                name: name.clone(),
                address_reasons: unresolved,
            }
        }
        _ if unresolved.is_empty() => AccessCertainty::StaticallyInBounds,
        _ => AccessCertainty::RequiresEvaluation {
            reasons: unresolved,
        },
    };

    Ok(MemoryAccessAssessment {
        memory: memory.id().clone(),
        port: request.port().clone(),
        operation: request.operation().clone(),
        dimensions: checks,
        certainty,
    })
}

fn validate_port_permission(
    declared_access: MemoryPortAccess,
    request: &MemoryAccessRequest,
) -> Result<(), MemoryAccessError> {
    let permitted = match &request.operation {
        MemoryAccessOperation::Read => declared_access.permits_read(),
        MemoryAccessOperation::Write => declared_access.permits_write(),
        MemoryAccessOperation::ReadWrite => {
            declared_access.permits_read() && declared_access.permits_write()
        }
        // The meaning of an extension operation is not known here. Its
        // provider must perform the operation-specific permission check.
        MemoryAccessOperation::Extension(_) => true,
    };

    if permitted {
        Ok(())
    } else {
        Err(MemoryAccessError::IncompatiblePortAccess {
            port: request.port().clone(),
            operation: request.operation().clone(),
            declared_access,
        })
    }
}

fn check_dimension(
    dimension: usize,
    extent: &Dimension,
    address: &MemoryAddressTerm,
) -> Result<DimensionAddressCheck, MemoryAccessError> {
    if matches!(extent, Dimension::Known(0)) {
        return Err(MemoryAccessError::ZeroExtent { dimension });
    }

    match (extent, address) {
        (Dimension::Known(extent), MemoryAddressTerm::Constant(index)) => {
            if index >= extent {
                Err(MemoryAccessError::OutOfBounds {
                    dimension,
                    index: *index,
                    extent: *extent,
                })
            } else {
                Ok(DimensionAddressCheck::InBounds {
                    dimension,
                    index: *index,
                    extent: *extent,
                })
            }
        }

        (_, MemoryAddressTerm::Symbolic(_)) => {
            let reason = IndeterminateReason::SymbolicAddress { dimension };
            Ok(DimensionAddressCheck::RequiresEvaluation { dimension, reason })
        }

        (_, MemoryAddressTerm::Dynamic { .. }) => {
            let reason = IndeterminateReason::DynamicAddress { dimension };
            Ok(DimensionAddressCheck::RequiresEvaluation { dimension, reason })
        }

        (Dimension::Symbolic(_), MemoryAddressTerm::Constant(_)) => {
            let reason = IndeterminateReason::SymbolicExtent { dimension };
            Ok(DimensionAddressCheck::RequiresEvaluation { dimension, reason })
        }

        (Dimension::Dynamic, MemoryAddressTerm::Constant(_)) => {
            let reason = IndeterminateReason::DynamicExtent { dimension };
            Ok(DimensionAddressCheck::RequiresEvaluation { dimension, reason })
        }
    }
}

/// Classification of the type of conflict between two accesses.
///
/// This is a classification of possible interference, not a statement about
/// which value a read observes or which write wins.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MemoryCollisionKind {
    /// Two writes may target the same element.
    WriteWrite,

    /// A read and a write may target the same element.
    ReadWrite,
}

/// Certainty that two requests refer to the same logical element.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AddressOverlap {
    /// At least one pair of concrete coordinates proves that the elements
    /// differ.
    Disjoint,

    /// Every coordinate is concrete and equal.
    Definite,

    /// No concrete coordinate proves disjointness, but equality is unresolved.
    Indeterminate,
}

/// Result of conservative simultaneous-access collision analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MemoryCollisionAssessment {
    /// No conflicting writes are possible under the built-in operations.
    ///
    /// This includes read/read pairs. It does not mean that two requests
    /// cannot contend for a physical implementation resource.
    NoWriteConflict,

    /// A write conflict is possible or definite.
    ///
    /// The future memory semantics layer must decide whether the pair is
    /// legal and what observable behavior the language specifies.
    WriteConflict {
        /// Kind of possible conflict.
        kind: MemoryCollisionKind,
        /// Certainty of logical address overlap.
        overlap: AddressOverlap,
    },

    /// At least one operation is extension-defined, so the built-in
    /// conflict model cannot establish the result.
    RequiresExtensionSemantics {
        /// Names of extension-defined operations involved.
        operations: Vec<String>,
    },

    /// The requests refer to different logical memories and therefore cannot
    /// collide at the logical-element level.
    DifferentMemories,
}

/// Conservatively analyze whether two logical access requests may conflict.
///
/// A pair of concrete addresses is definitely disjoint if any coordinate
/// differs. Symbolic or dynamic addresses are never assumed disjoint merely
/// because their expressions differ textually.
///
/// This function does not prove that two accesses occur simultaneously.
/// The caller must establish event/concurrency relationships separately.
///
/// # Important
///
/// This function does not apply read-during-write policy, write priority,
/// atomicity, arbitration, or simulation ordering. Those belong to the
/// memory semantics and execution layers.
#[must_use]
pub fn analyze_collision(
    left: &MemoryAccessRequest,
    right: &MemoryAccessRequest,
) -> MemoryCollisionAssessment {
    if left.memory() != right.memory() {
        return MemoryCollisionAssessment::DifferentMemories;
    }

    let left_reads = left.operation().reads();
    let left_writes = left.operation().writes();
    let right_reads = right.operation().reads();
    let right_writes = right.operation().writes();

    if left_reads.is_none()
        || left_writes.is_none()
        || right_reads.is_none()
        || right_writes.is_none()
    {
        let mut operations = Vec::new();

        if let MemoryAccessOperation::Extension(name) = left.operation() {
            operations.push(name.clone());
        }

        if let MemoryAccessOperation::Extension(name) = right.operation() {
            if !operations.contains(name) {
                operations.push(name.clone());
            }
        }

        return MemoryCollisionAssessment::RequiresExtensionSemantics {
            operations,
        };
    }

    let has_write_conflict = left_writes == Some(true) || right_writes == Some(true);

    if !has_write_conflict {
        return MemoryCollisionAssessment::NoWriteConflict;
    }

    let kind = if left_reads == Some(true) || right_reads == Some(true) {
        MemoryCollisionKind::ReadWrite
    } else {
        MemoryCollisionKind::WriteWrite
    };

    let overlap = classify_address_overlap(left.address(), right.address());

    if overlap == AddressOverlap::Disjoint {
        MemoryCollisionAssessment::NoWriteConflict
    } else {
        MemoryCollisionAssessment::WriteConflict { kind, overlap }
    }
}

fn classify_address_overlap(
    left: &[MemoryAddressTerm],
    right: &[MemoryAddressTerm],
) -> AddressOverlap {
    if left.len() != right.len() {
        // Malformed requests cannot establish disjointness. Validation should
        // reject their ranks before a caller relies on this analysis.
        return AddressOverlap::Indeterminate;
    }

    let mut all_equal_and_concrete = true;

    for (left_term, right_term) in left.iter().zip(right.iter()) {
        match (left_term.as_constant(), right_term.as_constant()) {
            (Some(left_value), Some(right_value)) if left_value != right_value => {
                return AddressOverlap::Disjoint;
            }
            (Some(_), Some(_)) => {}
            _ => all_equal_and_concrete = false,
        }
    }

    if all_equal_and_concrete {
        AddressOverlap::Definite
    } else {
        AddressOverlap::Indeterminate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concrete_address_terms_preserve_their_values() {
        assert_eq!(
            MemoryAddressTerm::Constant(u128::MAX).as_constant(),
            Some(u128::MAX)
        );
        assert_eq!(MemoryAddressTerm::Constant(0).as_constant(), Some(0));
    }

    #[test]
    fn dynamic_address_is_not_mistaken_for_a_constant() {
        assert_eq!(
            MemoryAddressTerm::dynamic(None).as_constant(),
            None
        );
    }

    #[test]
    fn builtin_operation_read_write_intent_is_explicit() {
        assert_eq!(MemoryAccessOperation::Read.reads(), Some(true));
        assert_eq!(MemoryAccessOperation::Read.writes(), Some(false));

        assert_eq!(MemoryAccessOperation::Write.reads(), Some(false));
        assert_eq!(MemoryAccessOperation::Write.writes(), Some(true));

        assert_eq!(MemoryAccessOperation::ReadWrite.reads(), Some(true));
        assert_eq!(MemoryAccessOperation::ReadWrite.writes(), Some(true));
    }

    #[test]
    fn extension_operation_is_not_assumed_to_be_read_or_write() {
        let operation = MemoryAccessOperation::Extension("atomic.compare_swap".into());

        assert_eq!(operation.reads(), None);
        assert_eq!(operation.writes(), None);
    }

    #[test]
    fn distinct_concrete_coordinates_prove_disjointness() {
        assert_eq!(
            classify_address_overlap(
                &[MemoryAddressTerm::Constant(4)],
                &[MemoryAddressTerm::Constant(5)],
            ),
            AddressOverlap::Disjoint
        );
    }

    #[test]
    fn equal_concrete_coordinates_prove_overlap() {
        assert_eq!(
            classify_address_overlap(
                &[
                    MemoryAddressTerm::Constant(4),
                    MemoryAddressTerm::Constant(7),
                ],
                &[
                    MemoryAddressTerm::Constant(4),
                    MemoryAddressTerm::Constant(7),
                ],
            ),
            AddressOverlap::Definite
        );
    }

    #[test]
    fn symbolic_coordinates_are_not_assumed_disjoint() {
        assert_eq!(
            classify_address_overlap(
                &[
                    MemoryAddressTerm::Constant(4),
                    MemoryAddressTerm::Dynamic { source: None },
                ],
                &[
                    MemoryAddressTerm::Constant(4),
                    MemoryAddressTerm::Constant(7),
                ],
            ),
            AddressOverlap::Indeterminate
        );
    }

    #[test]
    fn different_known_coordinate_proves_disjoint_even_if_other_is_symbolic() {
        assert_eq!(
            classify_address_overlap(
                &[
                    MemoryAddressTerm::Constant(4),
                    MemoryAddressTerm::Dynamic { source: None },
                ],
                &[
                    MemoryAddressTerm::Constant(5),
                    MemoryAddressTerm::Constant(7),
                ],
            ),
            AddressOverlap::Disjoint
        );
    }

    #[test]
    fn read_read_pair_has_no_logical_write_conflict() {
        let memory = MemoryId::new("memory").expect("valid memory identity");
        let read_a = MemoryAccessRequest::new(
            memory.clone(),
            MemoryPortId::new("read_a").expect("valid port"),
            MemoryAccessOperation::Read,
            vec![MemoryAddressTerm::Constant(0)],
        );
        let read_b = MemoryAccessRequest::new(
            memory,
            MemoryPortId::new("read_b").expect("valid port"),
            MemoryAccessOperation::Read,
            vec![MemoryAddressTerm::Constant(0)],
        );

        assert_eq!(
            analyze_collision(&read_a, &read_b),
            MemoryCollisionAssessment::NoWriteConflict
        );
    }

    #[test]
    fn write_requests_to_different_memories_cannot_collide_logically() {
        let left = MemoryAccessRequest::new(
            MemoryId::new("left").expect("valid memory"),
            MemoryPortId::new("write").expect("valid port"),
            MemoryAccessOperation::Write,
            vec![MemoryAddressTerm::Constant(0)],
        );
        let right = MemoryAccessRequest::new(
            MemoryId::new("right").expect("valid memory"),
            MemoryPortId::new("write").expect("valid port"),
            MemoryAccessOperation::Write,
            vec![MemoryAddressTerm::Constant(0)],
        );

        assert_eq!(
            analyze_collision(&left, &right),
            MemoryCollisionAssessment::DifferentMemories
        );
    }
}
