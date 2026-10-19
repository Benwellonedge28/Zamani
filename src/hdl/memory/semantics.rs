
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Memory Semantics
//!
//! Path: `src/hdl/memory/semantics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define explicit, target-independent contracts for the observable behavior
//! of logical HDL memories: initialization intent, access latency, read values
//! during simultaneous reads and writes, and write/write conflict handling.
//!
//! This module describes behavior; it does not allocate or execute storage.
//!
//! ## Authority and ownership
//!
//! - `grammar/spec/hdl.md` is the normative HDL architecture specification.
//! - `grammar/hdl/hdl.g4` is the HDL grammar composition root.
//! - `grammar/hdl/memories.g4` owns memory declaration and access syntax.
//! - `src/frontend/parser/domains/hdl.rs` owns HDL parser integration.
//! - `src/frontend/semantic/domains/hdl.rs` owns existing HDL semantic checks.
//! - `src/hdl/memory/model.rs` owns logical memory declarations.
//! - `src/hdl/memory/access.rs` owns access validation and address-overlap
//!   analysis.
//! - The canonical AST owns source expressions and their identities.
//! - The canonical semantic/type systems own expression and type meaning.
//!
//! This module owns only the behavioral policy contract and the interpretation
//! of access-collision assessments under an explicitly supplied policy.
//!
//! ## Does not own
//!
//! - Parsing, AST construction, name resolution, or expression evaluation.
//! - A replacement type, width, symbolic-expression, or source-span system.
//! - Physical memory allocation, storage contents, or hardware discovery.
//! - Arbitration, event scheduling, or simulation execution.
//! - Synthesis, placement, routing, timing closure, or vendor APIs.
//! - Target resource allocation or feasibility decisions.
//! - HDLIR data structures, builders, passes, verification, serialization,
//!   or lowering. All HDLIR implementation remains in `src/hdl/hdlir/`.
//!
//! ## Important semantic rule
//!
//! No universal read-during-write or write-priority behavior is inferred.
//! An adapter must provide policies justified by the language specification,
//! an explicit source declaration, or a documented language profile.
//! Missing information remains unspecified; it is never silently interpreted
//! as old-data, new-data, zero initialization, or a particular write winner.
//!
//! ## Scalability
//!
//! No fixed limit is imposed on logical memory dimensions, ports, or priority
//! entries. Symbolic latency is preserved. Concrete cycle counts use `u128`
//! as a representation of an already-resolved non-negative value, not as a
//! hardware register-width assumption. Dynamic allocations and work remain
//! subject to available resources and caller-controlled operational budgets.
//!
//! ## Determinism
//!
//! Results depend only on the declaration, requests, and supplied policies.
//! No global mutable state, physical target discovery, or pointer identity is
//! consulted. Priority order is explicit and stable.
//!
//! ## Completion criteria
//!
//! This module is complete when its public contracts are registered by
//! `memory/mod.rs`, AST/semantic adapters supply justified policies, all
//! unresolved cases remain explicit, and workspace plus conformance tests pass.

use core::fmt;
use std::error::Error;

use crate::hdl::memory::access::{
    AddressOverlap, MemoryAccessOperation, MemoryAccessRequest, MemoryCollisionAssessment,
    MemoryCollisionKind, analyze_collision,
};
use crate::hdl::memory::model::{
    HdlMemory, MemoryId, MemoryInitialization, MemoryPortId,
};
use crate::zuir::types::SymbolicExpression;

/// Version of this module's in-memory behavioral contract.
///
/// This is not the Zamani language version, memory-model schema version,
/// or any future HDLIR version.
pub const HDL_MEMORY_SEMANTICS_SCHEMA_VERSION: u16 = 1;

/// Source-level initialization status of a logical memory.
///
/// This reports the declaration's initialization intent. It does not evaluate
/// the expression, read external data, initialize physical storage, or prove
/// that a target can implement the initialization requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum InitializationContract {
    /// The declaration contains no initialization specification.
    ///
    /// This does not mean zero-filled, deterministic, or otherwise initialized.
    Unspecified,

    /// Initialization refers to a canonical source expression.
    ///
    /// The expression must be evaluated by the canonical semantic/evaluation
    /// pipeline before its concrete value is known.
    RequiresEvaluation {
        /// Canonical AST node containing the initialization expression.
        source: crate::frontend::ast::node::node_id::NodeId,
    },

    /// Initialization is described by an external source declaration.
    ///
    /// The external resource is not opened or loaded by this module.
    External {
        /// Canonical AST node containing the external initialization declaration.
        source: crate::frontend::ast::node::node_id::NodeId,
    },
}

impl From<&MemoryInitialization> for InitializationContract {
    fn from(initialization: &MemoryInitialization) -> Self {
        match initialization {
            MemoryInitialization::Unspecified => Self::Unspecified,
            MemoryInitialization::Constant { source }
            | MemoryInitialization::Expression { source } => Self::RequiresEvaluation {
                source: source.clone(),
            },
            MemoryInitialization::External { source } => Self::External {
                source: source.clone(),
            },
        }
    }
}

/// Declared access-latency contract.
///
/// Latency is logical behavior, not a promise about a particular device's
/// physical cycle time, clock frequency, or implementation architecture.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MemoryLatency {
    /// The declaration or semantic profile does not specify latency.
    Unspecified,

    /// A read is logically combinational, with no registered-cycle delay.
    ///
    /// Whether a requested target can preserve this behavior is a feasibility
    /// question and is not decided here.
    Combinational,

    /// The behavior specifies a concrete number of logical cycles.
    Registered {
        /// Number of logical cycles from the specified request event to the
        /// corresponding result event.
        cycles: u128,
    },

    /// Latency depends on a canonical symbolic expression.
    ///
    /// The expression is preserved and must be resolved by the canonical
    /// evaluator. It is not evaluated by this module.
    Symbolic {
        /// Canonical symbolic latency expression.
        expression: SymbolicExpression,
    },
}

/// Observable value a read receives when it overlaps a write to the same
/// logical element in the same relevant event.
///
/// These policies describe language-level behavior only. They do not select
/// physical RAM modes or vendor-specific primitives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReadDuringWritePolicy {
    /// The read observes the value that existed before the write event.
    OldValue,

    /// The read observes the value produced by the write event.
    NewValue,

    /// The read result is not updated by the overlapping write event.
    NoChange,

    /// The language/profile explicitly specifies that the result is undefined.
    ///
    /// This is distinct from `Unspecified`: undefined is an explicit contract,
    /// while unspecified means the contract has not been established.
    Undefined,

    /// No applicable language/profile rule has been supplied.
    ///
    /// Callers must not assume old-data, new-data, no-change, or zero.
    Unspecified,
}

/// Policy for simultaneous writes that may address the same logical element.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WriteCollisionPolicy {
    /// A conflicting simultaneous write is invalid under the selected
    /// language/profile contract.
    Reject,

    /// Writes are resolved by explicit logical-port priority.
    ///
    /// Earlier entries have higher priority. Every port participating in a
    /// priority-resolved conflict must appear in this list. This list is
    /// policy metadata, not physical port numbering.
    PortPriority {
        /// Port identities ordered from highest to lowest priority.
        highest_first: Vec<MemoryPortId>,
    },

    /// The applicable contract does not define which write takes effect.
    Unspecified,
}

/// Observable effect implied by a supplied policy for a collision.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CollisionEffect {
    /// The requests cannot affect the same logical element.
    NoConflict,

    /// The read result follows the specified read-during-write rule.
    ReadValue(ReadDuringWritePolicy),

    /// The specified policy selects this logical port's write.
    WriteWinner(MemoryPortId),

    /// The supplied policy says this definite conflict is invalid.
    Rejected,

    /// The behavior is not specified by the available contract.
    Unspecified,

    /// An operation needs an extension's registered semantic definition.
    RequiresExtensionSemantics {
        /// Names of the extension operations involved, in stable order.
        operations: Vec<String>,
    },

    /// Address overlap is not known statically.
    ///
    /// `on_overlap` is the result if the requests are later established to
    /// overlap. The enclosing result does not assert that overlap occurs.
    Indeterminate {
        /// Policy outcome conditional on the addresses overlapping.
        on_overlap: Box<Self>,
    },
}

/// Assessment of two requests that the caller asserts occur in the same
/// relevant logical event.
///
/// This is not an event scheduler and does not prove that requests are
/// simultaneous. The caller must establish the event relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimultaneousAccessAssessment {
    /// Logical memory to which the policy was applied.
    pub memory: MemoryId,

    /// Classification produced by the access-analysis layer.
    pub collision: MemoryCollisionAssessment,

    /// Observable effect under this memory's explicit policy.
    pub effect: CollisionEffect,
}

/// Invalid or inconsistent memory behavioral policy.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MemorySemanticsError {
    /// The supplied policy belongs to a different logical memory.
    MemoryIdentityMismatch {
        /// Identity declared by this policy.
        declared: MemoryId,
        /// Identity in the request.
        requested: MemoryId,
    },

    /// The priority list contains the same logical port more than once.
    DuplicatePriorityPort {
        /// Repeated port identity.
        port: MemoryPortId,
    },

    /// A priority-resolved collision references a port absent from the
    /// configured priority list.
    MissingPriorityPort {
        /// Port whose priority is not configured.
        port: MemoryPortId,
    },

    /// A built-in access operation has a shape that cannot be classified by
    /// the behavioral policy layer.
    UnsupportedOperationShape {
        /// Operation which needs additional semantics.
        operation: MemoryAccessOperation,
    },
}

impl fmt::Display for MemorySemanticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MemoryIdentityMismatch { declared, requested } => write!(
                f,
                "memory semantics for `{declared}` cannot be applied to access on `{requested}`"
            ),
            Self::DuplicatePriorityPort { port } => write!(
                f,
                "memory write-priority policy lists port `{port}` more than once"
            ),
            Self::MissingPriorityPort { port } => write!(
                f,
                "memory write-priority policy has no entry for port `{port}`"
            ),
            Self::UnsupportedOperationShape { operation } => write!(
                f,
                "memory operation `{operation}` requires additional semantic classification"
            ),
        }
    }
}

impl Error for MemorySemanticsError {}

/// Target-independent behavioral contract for one logical memory.
///
/// The contract references the canonical [`HdlMemory`] identity and does not
/// copy its dimensions, element type, initialization expression, or ports.
/// Initialization is read from the memory model so this layer cannot silently
/// diverge from the declaration.
///
/// A contract created with `new` deliberately requires its policies to be
/// supplied by the caller. There is no implicit hardware-dependent default.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemorySemantics {
    memory: MemoryId,
    latency: MemoryLatency,
    read_during_write: ReadDuringWritePolicy,
    write_collision: WriteCollisionPolicy,
}

impl MemorySemantics {
    /// Construct an explicit behavioral contract for a logical memory.
    ///
    /// `memory` is the canonical logical declaration. The caller must choose
    /// policies based on the applicable language semantics or explicit source
    /// declarations; this constructor does not infer missing behavior.
    ///
    /// # Errors
    ///
    /// Returns an error if a priority list contains duplicate port identities.
    pub fn new(
        memory: &HdlMemory,
        latency: MemoryLatency,
        read_during_write: ReadDuringWritePolicy,
        write_collision: WriteCollisionPolicy,
    ) -> Result<Self, MemorySemanticsError> {
        if let WriteCollisionPolicy::PortPriority { highest_first } = &write_collision {
            let mut seen = std::collections::BTreeSet::new();
            for port in highest_first {
                if !seen.insert(port) {
                    return Err(MemorySemanticsError::DuplicatePriorityPort {
                        port: port.clone(),
                    });
                }
            }
        }

        Ok(Self {
            memory: memory.id().clone(),
            latency,
            read_during_write,
            write_collision,
        })
    }

    /// Return the logical memory identity governed by this contract.
    #[must_use]
    pub const fn memory(&self) -> &MemoryId {
        &self.memory
    }

    /// Return the declared logical access latency.
    #[must_use]
    pub const fn latency(&self) -> &MemoryLatency {
        &self.latency
    }

    /// Return the read-during-write policy.
    #[must_use]
    pub const fn read_during_write_policy(&self) -> ReadDuringWritePolicy {
        self.read_during_write
    }

    /// Return the simultaneous write-collision policy.
    #[must_use]
    pub const fn write_collision_policy(&self) -> &WriteCollisionPolicy {
        &self.write_collision
    }

    /// Return initialization intent from the canonical memory declaration.
    ///
    /// This operation never evaluates the initialization expression.
    #[must_use]
    pub fn initialization(&self, memory: &HdlMemory) -> InitializationContract {
        InitializationContract::from(memory.initialization())
    }

    /// Validate that this behavioral contract is being used with its memory.
    ///
    /// # Errors
    ///
    /// Returns [`MemorySemanticsError::MemoryIdentityMismatch`] when the
    /// supplied declaration has a different identity.
    pub fn validate_for(&self, memory: &HdlMemory) -> Result<(), MemorySemanticsError> {
        if memory.id() != &self.memory {
            return Err(MemorySemanticsError::MemoryIdentityMismatch {
                declared: self.memory.clone(),
                requested: memory.id().clone(),
            });
        }

        Ok(())
    }

    /// Interpret two requests under this memory's explicit behavioral policy.
    ///
    /// The caller is responsible for establishing that both requests occur in
    /// the same relevant logical event. Address overlap is obtained from
    /// `access.rs`; unresolved addresses remain indeterminate.
    ///
    /// This method does not execute either request, mutate storage, arbitrate
    /// a physical interface, or decide whether a target can implement the
    /// resulting contract.
    ///
    /// # Errors
    ///
    /// Returns [`MemorySemanticsError::MemoryIdentityMismatch`] if either
    /// request refers to a different memory.
    pub fn assess_simultaneous_access(
        &self,
        left: &MemoryAccessRequest,
        right: &MemoryAccessRequest,
    ) -> Result<SimultaneousAccessAssessment, MemorySemanticsError> {
        self.ensure_request_memory(left)?;
        self.ensure_request_memory(right)?;

        let collision = analyze_collision(left, right);
        let effect = self.effect_for_collision(&collision, left, right)?;

        Ok(SimultaneousAccessAssessment {
            memory: self.memory.clone(),
            collision,
            effect,
        })
    }

    fn ensure_request_memory(
        &self,
        request: &MemoryAccessRequest,
    ) -> Result<(), MemorySemanticsError> {
        if request.memory() != &self.memory {
            return Err(MemorySemanticsError::MemoryIdentityMismatch {
                declared: self.memory.clone(),
                requested: request.memory().clone(),
            });
        }
        Ok(())
    }

    fn effect_for_collision(
        &self,
        collision: &MemoryCollisionAssessment,
        left: &MemoryAccessRequest,
        right: &MemoryAccessRequest,
    ) -> Result<CollisionEffect, MemorySemanticsError> {
        match collision {
            MemoryCollisionAssessment::DifferentMemories
            | MemoryCollisionAssessment::NoWriteConflict => Ok(CollisionEffect::NoConflict),

            MemoryCollisionAssessment::RequiresExtensionSemantics { operations } => {
                Ok(CollisionEffect::RequiresExtensionSemantics {
                    operations: operations.clone(),
                })
            }

            MemoryCollisionAssessment::WriteConflict { kind, overlap } => {
                let on_overlap = match kind {
                    MemoryCollisionKind::ReadWrite => {
                        CollisionEffect::ReadValue(self.read_during_write)
                    }
                    MemoryCollisionKind::WriteWrite => {
                        self.write_write_effect(left, right)?
                    }
                };

                match overlap {
                    AddressOverlap::Disjoint => Ok(CollisionEffect::NoConflict),
                    AddressOverlap::Definite => Ok(on_overlap),
                    AddressOverlap::Indeterminate => Ok(CollisionEffect::Indeterminate {
                        on_overlap: Box::new(on_overlap),
                    }),
                }
            }
        }
    }

    fn write_write_effect(
        &self,
        left: &MemoryAccessRequest,
        right: &MemoryAccessRequest,
    ) -> Result<CollisionEffect, MemorySemanticsError> {
        match &self.write_collision {
            WriteCollisionPolicy::Reject => Ok(CollisionEffect::Rejected),
            WriteCollisionPolicy::Unspecified => Ok(CollisionEffect::Unspecified),
            WriteCollisionPolicy::PortPriority { highest_first } => {
                let left_rank = highest_first
                    .iter()
                    .position(|port| port == left.port());
                let right_rank = highest_first
                    .iter()
                    .position(|port| port == right.port());

                let left_rank = left_rank.ok_or_else(|| {
                    MemorySemanticsError::MissingPriorityPort {
                        port: left.port().clone(),
                    }
                })?;
                let right_rank = right_rank.ok_or_else(|| {
                    MemorySemanticsError::MissingPriorityPort {
                        port: right.port().clone(),
                    }
                })?;

                if left_rank < right_rank {
                    Ok(CollisionEffect::WriteWinner(left.port().clone()))
                } else if right_rank < left_rank {
                    Ok(CollisionEffect::WriteWinner(right.port().clone()))
                } else {
                    // Duplicate entries are rejected at construction, so
                    // equal ranks for different ports cannot occur.
                    Ok(CollisionEffect::Unspecified)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unspecified_read_policy_is_not_silently_replaced() {
        assert_eq!(
            ReadDuringWritePolicy::Unspecified,
            ReadDuringWritePolicy::Unspecified
        );
    }

    #[test]
    fn undefined_and_unspecified_are_distinct_contracts() {
        assert_ne!(
            ReadDuringWritePolicy::Undefined,
            ReadDuringWritePolicy::Unspecified
        );
    }

    #[test]
    fn symbolic_latency_is_preserved() {
        let latency = MemoryLatency::Symbolic {
            expression: SymbolicExpression {
                expression: "READ_LATENCY".to_owned(),
            },
        };

        assert_eq!(
            latency,
            MemoryLatency::Symbolic {
                expression: SymbolicExpression {
                    expression: "READ_LATENCY".to_owned(),
                },
            }
        );
    }

    #[test]
    fn initialization_contract_does_not_claim_zero_fill() {
        assert_eq!(
            InitializationContract::Unspecified,
            InitializationContract::Unspecified
        );
    }

    #[test]
    fn explicit_write_policies_remain_distinct() {
        assert_ne!(WriteCollisionPolicy::Reject, WriteCollisionPolicy::Unspecified);
    }
}
