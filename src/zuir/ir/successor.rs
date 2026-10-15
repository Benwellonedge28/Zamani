
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Control-Flow Successor
//!
//! Canonical path: `src/zuir/ir/successor.rs`
//! Rust edition: 2021
//! Minimum intended Rust version: 1.97
//! Safety: `unsafe` Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Represent one explicit control-flow edge from a terminator to a target
//! basic block, together with the ordered SSA values passed to that block.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/block.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! ## OWNS
//!
//! - The successor edge's target `BlockId`.
//! - The ordered list of `ValueId` arguments carried by the edge.
//! - Accessors and fallible construction from an iterator.
//!
//! ## DOES_NOT_OWN
//!
//! - Block, region, or operation storage.
//! - Control-flow terminator semantics.
//! - Global ID allocation or reference resolution.
//! - Type checking, SSA dominance, or whole-program verification.
//! - Source syntax, parsing, or AST construction.
//! - Hardware routing, quantum topology, or physical placement.
//!
//! ## INTEGRATION
//!
//! `block.rs` uses `Successor` inside `BlockTerminator`.
//! The full verifier resolves the target block, validates argument count
//! against target block arguments, checks value definitions and types, and
//! verifies dominance and region ownership.
//!
//! This module uses the canonical typed IDs from `ids.rs`; it does not
//! introduce raw integer IDs or a second identity system.
//!
//! ## SCALABILITY
//!
//! The argument list is dynamically sized. No fixed successor-argument,
//! block-count, operation-count, qubit-count, register-width, tensor-rank,
//! or hardware-capacity ceiling is imposed here.
//!
//! Fallible iterator construction reports allocation reservation failures.
//! Ordinary Rust collection allocation can still fail according to platform
//! allocator behavior. This type does not promise literal infinity.
//!
//! ## SEMANTIC CONTRACT
//!
//! Argument order is significant and must match the target block's declared
//! argument order. Repeated `ValueId` arguments are permitted: rejecting
//! repeated values here would incorrectly rule out valid control-flow edges.
//!
//! The existence and legality of referenced IDs are checked by the owning
//! program verifier, not inferred from this edge alone.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::zuir::ids::{BlockId, ValueId};

/// Result type for fallible successor construction.
pub type SuccessorResult<T> = Result<T, SuccessorError>;

/// Errors that can occur while constructing a successor edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuccessorError {
    /// Reserving storage for successor arguments failed.
    ///
    /// This indicates that the requested collection could not reserve
    /// additional capacity. It does not imply that the program is invalid.
    AllocationFailure,
}

impl fmt::Display for SuccessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllocationFailure => {
                f.write_str("unable to reserve memory for successor arguments")
            }
        }
    }
}

impl std::error::Error for SuccessorError {}

/// One explicit control-flow edge.
///
/// `target` identifies the destination block. `arguments` contains the
/// values passed to that block, in the order expected by its block arguments.
///
/// This type records an edge; it does not prove that the destination exists,
/// that the arguments have compatible types, or that the edge is legal in
/// the enclosing control-flow graph.
///
/// The fields are private so callers cannot accidentally replace the
/// collection with an unvalidated representation. Serde deserialization
/// must still be followed by whole-program verification before trusted use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Successor {
    target: BlockId,
    arguments: Vec<ValueId>,
}

impl Successor {
    /// Construct an edge with no block arguments.
    ///
    /// This preserves the existing `block.rs` constructor API.
    #[must_use]
    pub fn new(target: BlockId) -> Self {
        Self {
            target,
            arguments: Vec::new(),
        }
    }

    /// Construct an edge with explicitly ordered block arguments.
    ///
    /// The supplied vector is moved into the successor without cloning.
    /// Repeated values are allowed.
    #[must_use]
    pub fn with_arguments(target: BlockId, arguments: Vec<ValueId>) -> Self {
        Self { target, arguments }
    }

    /// Construct an edge from an iterator while reserving storage fallibly.
    ///
    /// This is useful when arguments are produced dynamically and callers
    /// want allocation reservation failures represented as a `Result`.
    ///
    /// The iterator's size hint is only a hint. The implementation also
    /// reserves when additional items exceed the initial capacity.
    ///
    /// This method cannot bound the time or memory consumed by an arbitrary
    /// iterator; callers handling untrusted or potentially unbounded input
    /// must enforce their own explicit resource budget.
    pub fn try_from_iter<I>(
        target: BlockId,
        arguments: I,
    ) -> SuccessorResult<Self>
    where
        I: IntoIterator<Item = ValueId>,
    {
        let iterator = arguments.into_iter();
        let (_, upper_bound) = iterator.size_hint();

        let mut collected = Vec::new();

        if let Some(capacity) = upper_bound {
            collected
                .try_reserve(capacity)
                .map_err(|_| SuccessorError::AllocationFailure)?;
        }

        for argument in iterator {
            if collected.len() == collected.capacity() {
                collected
                    .try_reserve(1)
                    .map_err(|_| SuccessorError::AllocationFailure)?;
            }

            collected.push(argument);
        }

        Ok(Self {
            target,
            arguments: collected,
        })
    }

    /// Return the destination block identity.
    #[must_use]
    pub const fn target(&self) -> BlockId {
        self.target
    }

    /// Borrow the ordered successor arguments.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Return the number of arguments passed to the destination block.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Whether this edge passes no block arguments.
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Consume this edge and return its target and ordered arguments.
    #[must_use]
    pub fn into_parts(self) -> (BlockId, Vec<ValueId>) {
        (self.target, self.arguments)
    }

    /// Replace the argument list, returning the previous list.
    ///
    /// The caller is responsible for ensuring that the new arguments match
    /// the destination block. The full verifier remains authoritative.
    pub fn replace_arguments(
        &mut self,
        arguments: Vec<ValueId>,
    ) -> Vec<ValueId> {
        std::mem::replace(&mut self.arguments, arguments)
    }

    /// Append one argument while reporting reservation failure.
    ///
    /// This preserves the order in which arguments are appended.
    pub fn try_push_argument(
        &mut self,
        argument: ValueId,
    ) -> SuccessorResult<()> {
        if self.arguments.len() == self.arguments.capacity() {
            self.arguments
                .try_reserve(1)
                .map_err(|_| SuccessorError::AllocationFailure)?;
        }

        self.arguments.push(argument);
        Ok(())
    }

    /// Append several arguments using fallible reservation.
    ///
    /// On reservation failure, the successor's existing arguments remain
    /// unchanged. The incoming vector is consumed by this method.
    pub fn try_extend_arguments(
        &mut self,
        arguments: Vec<ValueId>,
    ) -> SuccessorResult<()> {
        self.arguments
            .try_reserve(arguments.len())
            .map_err(|_| SuccessorError::AllocationFailure)?;

        self.arguments.extend(arguments);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::{BlockId, ValueId};

    fn block_id(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block ID must be nonzero")
    }

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    #[test]
    fn new_creates_edge_without_arguments() {
        let target = block_id(1);
        let successor = Successor::new(target);

        assert_eq!(successor.target(), target);
        assert!(successor.arguments().is_empty());
        assert_eq!(successor.argument_count(), 0);
        assert!(successor.has_no_arguments());
    }

    #[test]
    fn arguments_preserve_their_order() {
        let target = block_id(2);
        let first = value_id(10);
        let second = value_id(20);
        let third = value_id(30);

        let successor =
            Successor::with_arguments(target, vec![first, second, third]);

        assert_eq!(successor.target(), target);
        assert_eq!(successor.arguments(), &[first, second, third]);
        assert_eq!(successor.argument_count(), 3);
    }

    #[test]
    fn repeated_arguments_are_permitted() {
        let value = value_id(7);
        let successor =
            Successor::with_arguments(block_id(3), vec![value, value]);

        assert_eq!(successor.arguments(), &[value, value]);
    }

    #[test]
    fn iterator_constructor_preserves_order() {
        let expected = vec![value_id(4), value_id(5), value_id(6)];

        let successor = Successor::try_from_iter(
            block_id(4),
            expected.iter().copied(),
        )
        .expect("small test collection should be constructible");

        assert_eq!(successor.arguments(), expected.as_slice());
    }

    #[test]
    fn push_and_extend_preserve_argument_order() {
        let first = value_id(1);
        let second = value_id(2);
        let third = value_id(3);

        let mut successor = Successor::new(block_id(5));

        successor
            .try_push_argument(first)
            .expect("small test collection should be constructible");

        successor
            .try_extend_arguments(vec![second, third])
            .expect("small test collection should be constructible");

        assert_eq!(successor.arguments(), &[first, second, third]);
    }

    #[test]
    fn replace_arguments_returns_the_previous_arguments() {
        let first = value_id(11);
        let second = value_id(12);
        let replacement = value_id(13);

        let mut successor =
            Successor::with_arguments(block_id(6), vec![first, second]);

        let old = successor.replace_arguments(vec![replacement]);

        assert_eq!(old, vec![first, second]);
        assert_eq!(successor.arguments(), &[replacement]);
    }

    #[test]
    fn into_parts_returns_target_and_arguments() {
        let target = block_id(7);
        let arguments = vec![value_id(21), value_id(22)];

        let successor =
            Successor::with_arguments(target, arguments.clone());

        let (actual_target, actual_arguments) = successor.into_parts();

        assert_eq!(actual_target, target);
        assert_eq!(actual_arguments, arguments);
    }
}
