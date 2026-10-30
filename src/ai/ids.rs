
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — AI-Domain Identifiers
//!
//! Path: `src/ai/ids.rs`
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines strongly typed identifiers for entities owned by the Zamani
//! artificial intelligence computational domain.
//!
//! This module reuses the canonical ZUIR identifier infrastructure.
//! It does not introduce a competing universal identifier system.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - AI model identities.
//! - AI parameter identities.
//! - AI dataset identities.
//! - AI training-run identities.
//! - AI agent identities.
//! - AI pipeline identities.
//! - AI artifact identities.
//! - AI knowledge identities.
//! - AI evidence identities.
//! - AI decision identities.
//!
//! It does not own:
//!
//! - Universal IR identities.
//! - Quantum identifiers.
//! - Source-file identities.
//! - Physical hardware identifiers.
//! - Distributed participant identities.
//! - Persistent content fingerprints.
//! - Identity storage or authorization.
//!
//! ## Canonical dependencies
//!
//! `crate::zuir::ids` owns:
//!
//! - TypedId.
//! - IdAllocator.
//! - InvalidIdValue.
//! - IdSpaceExhausted.
//! - ProgramId.
//! - ModuleId.
//! - FunctionId.
//! - RegionId.
//! - BlockId.
//! - OperationId.
//! - ValueId.
//! - TypeId.
//! - DialectId.
//! - SymbolId.
//! - ExtensionId.
//! - SourceId.
//!
//! AI re-exports these canonical types where required.
//!
//! ## Identity semantics
//!
//! Each AI identity category is a distinct Rust type.
//!
//! Numeric equality across different categories does not make their
//! identities interchangeable.
//!
//! Zero is invalid.
//!
//! Raw construction validates representation but does not establish
//! uniqueness.
//!
//! Allocator uniqueness is guaranteed only within one allocator instance.
//! Independent allocator instances have independent identity scopes.
//!
//! ## Scalability
//!
//! Identifiers use nonzero u128 values.
//!
//! No AI-specific maximum is imposed on models, datasets, agents,
//! pipelines, parameters, training runs or other AI entities.
//!
//! The representation is finite. Exhaustion is explicit and must never
//! result in silent wrapping or identifier reuse.
//!
//! ## POCO-REAF
//!
//! These identifiers are runtime identity tokens, not globally stable
//! identities.
//!
//! Persistent identity, canonical content fingerprints and artifact
//! provenance must be handled by their respective subsystems.
//!
//! ## Integration
//!
//! model/definition.rs       -> AiModelId
//! model/parameters.rs       -> AiParameterId
//! data/datasets.rs          -> AiDatasetId
//! learning/training.rs      -> AiTrainingRunId
//! agents/definition.rs      -> AiAgentId
//! pipeline/definition.rs    -> AiPipelineId
//! deployment/artifacts.rs   -> AiArtifactId
//! reasoning/knowledge.rs    -> AiKnowledgeId
//! reasoning/evidence.rs     -> AiEvidenceId
//! reasoning/decisions.rs    -> AiDecisionId
//!
//! The existing frontend semantic AiId remains a separate semantic
//! boundary identity. It must not be silently replaced by these types.
//!
//! ## Freeze criteria
//!
//! - Canonical ZUIR allocator reused.
//! - No duplicate universal identifiers.
//! - All AI categories strongly typed.
//! - Zero rejected.
//! - Serde preserves the nonzero invariant.
//! - Allocation exhaustion explicit.
//! - Independent allocation scopes documented.
//! - Concurrent allocation tested.
//! - No unsafe code.
//! - No hardware-specific assumptions.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::NonZeroU128;

// ============================================================================
// CANONICAL ZUIR IDENTIFIERS
// ============================================================================
//
// Re-export existing identifiers instead of creating duplicates.
//
// ZUIR remains the authoritative owner of universal computational identity.

pub use crate::zuir::ids::{
    BlockId,
    DialectId,
    ExtensionId,
    FunctionId,
    IdAllocator,
    IdSpaceExhausted,
    InvalidIdValue,
    ModuleId,
    OperationId,
    ProgramId,
    RegionId,
    SourceId,
    SymbolId,
    TypeId,
    TypedId,
    ValueId,
};

// ============================================================================
// AI IDENTIFIER DEFINITION
// ============================================================================
//
// Each AI entity receives a distinct type.
//
// All types implement the canonical ZUIR TypedId contract and can therefore
// use the existing ZUIR IdAllocator.
//
// NonZeroU128 ensures that zero cannot represent a valid identity.

macro_rules! define_ai_id {
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
            /// Construct an identifier from a nonzero numeric value.
            ///
            /// This validates the representation but does not establish
            /// uniqueness within an application or registry.
            pub const fn new(
                value: u128,
            ) -> Result<Self, InvalidIdValue> {
                match NonZeroU128::new(value) {
                    Some(value) => Ok(Self(value)),
                    None => Err(InvalidIdValue::Zero),
                }
            }

            /// Return the opaque numeric identity.
            pub const fn get(self) -> u128 {
                self.0.get()
            }

            /// Return the underlying nonzero numeric representation.
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
            fn fmt(
                &self,
                f: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                write!(
                    f,
                    concat!(stringify!($name), "({})"),
                    self.get()
                )
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                self.get().fmt(f)
            }
        }

        impl TryFrom<u128> for $name {
            type Error = InvalidIdValue;

            fn try_from(
                value: u128,
            ) -> Result<Self, Self::Error> {
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

// ============================================================================
// MODEL IDENTITIES
// ============================================================================

define_ai_id!(
    /// Identity of an AI model definition.
    ///
    /// Model architecture, parameters, state and execution are owned
    /// by their respective AI subsystems.
    AiModelId
);

define_ai_id!(
    /// Identity of an AI model parameter.
    ///
    /// Actual parameter values and tensor storage are owned by the
    /// appropriate tensor and model infrastructure.
    AiParameterId
);

// ============================================================================
// DATA IDENTITIES
// ============================================================================

define_ai_id!(
    /// Identity of a logical AI dataset.
    ///
    /// This does not identify its physical storage location.
    AiDatasetId
);

// ============================================================================
// LEARNING IDENTITIES
// ============================================================================

define_ai_id!(
    /// Identity of an AI training run.
    ///
    /// Training execution, checkpointing and lifecycle belong to
    /// the learning and runtime subsystems.
    AiTrainingRunId
);

// ============================================================================
// AGENT IDENTITIES
// ============================================================================

define_ai_id!(
    /// Identity of an AI agent definition.
    ///
    /// Agent sessions and distributed participant identities remain
    /// owned by their respective execution systems.
    AiAgentId
);

// ============================================================================
// PIPELINE IDENTITIES
// ============================================================================

define_ai_id!(
    /// Identity of an AI pipeline definition.
    AiPipelineId
);

// ============================================================================
// ARTIFACT IDENTITIES
// ============================================================================

define_ai_id!(
    /// Identity of an AI model or deployment artifact.
    ///
    /// Persistent fingerprints and artifact integrity are owned by
    /// the artifact and provenance infrastructure.
    AiArtifactId
);

// ============================================================================
// KNOWLEDGE AND REASONING IDENTITIES
// ============================================================================

define_ai_id!(
    /// Identity of an AI-domain knowledge entity.
    AiKnowledgeId
);

define_ai_id!(
    /// Identity of an evidence entity used by AI reasoning.
    AiEvidenceId
);

define_ai_id!(
    /// Identity of an AI-domain decision record.
    AiDecisionId
);

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn zero_is_rejected() {
        assert_eq!(
            AiModelId::new(0),
            Err(InvalidIdValue::Zero)
        );

        assert_eq!(
            AiDatasetId::new(0),
            Err(InvalidIdValue::Zero)
        );

        assert_eq!(
            AiAgentId::try_from(0),
            Err(InvalidIdValue::Zero)
        );
    }

    #[test]
    fn nonzero_values_are_preserved() {
        let id = AiModelId::new(42).expect("valid identifier");

        assert_eq!(id.get(), 42);
        assert_eq!(u128::from(id), 42);
        assert_eq!(id.to_string(), "42");
        assert_eq!(format!("{id:?}"), "AiModelId(42)");
    }

    #[test]
    fn maximum_nonzero_value_is_supported() {
        let id = AiModelId::new(u128::MAX)
            .expect("maximum u128 is nonzero");

        assert_eq!(id.get(), u128::MAX);
    }

    #[test]
    fn ai_identifier_categories_are_distinct_types() {
        assert_ne!(
            std::any::type_name::<AiModelId>(),
            std::any::type_name::<AiDatasetId>()
        );

        assert_ne!(
            std::any::type_name::<AiAgentId>(),
            std::any::type_name::<AiPipelineId>()
        );

        assert_ne!(
            std::any::type_name::<AiParameterId>(),
            std::any::type_name::<AiArtifactId>()
        );
    }

    #[test]
    fn independent_categories_can_have_equal_numeric_values() {
        let model = AiModelId::new(7).expect("valid model ID");
        let dataset = AiDatasetId::new(7).expect("valid dataset ID");

        assert_eq!(model.get(), dataset.get());

        // Their Rust types remain distinct.
        assert_ne!(
            std::any::type_name::<AiModelId>(),
            std::any::type_name::<AiDatasetId>()
        );
    }

    #[test]
    fn identifiers_support_ordering_and_hashing() {
        let first = AiModelId::new(1).expect("valid ID");
        let second = AiModelId::new(2).expect("valid ID");

        assert!(first < second);

        let identities: HashSet<_> = [first, second].into_iter().collect();

        assert_eq!(identities.len(), 2);
    }

    #[test]
    fn canonical_allocator_allocates_ai_identifiers() {
        let allocator = IdAllocator::<AiModelId>::new();

        let first = allocator.try_next().expect("first ID");
        let second = allocator.try_next().expect("second ID");

        assert_eq!(first.get(), 1);
        assert_eq!(second.get(), 2);
        assert!(first < second);
    }

    #[test]
    fn allocator_instances_have_independent_scopes() {
        let first_allocator = IdAllocator::<AiModelId>::new();
        let second_allocator = IdAllocator::<AiModelId>::new();

        let first = first_allocator.try_next().expect("first ID");
        let second = second_allocator.try_next().expect("second ID");

        // Independent allocators may issue equal numeric values.
        assert_eq!(first, second);
    }

    #[test]
    fn allocator_never_issues_zero() {
        let allocator = IdAllocator::<AiParameterId>::new();

        let id = allocator.try_next().expect("allocated ID");

        assert_ne!(id.get(), 0);
    }

    #[test]
    fn allocated_identifiers_are_unique_within_one_allocator() {
        let allocator = IdAllocator::<AiDatasetId>::new();

        let mut seen = HashSet::new();

        for _ in 0..10_000 {
            let id = allocator.try_next().expect("allocator has capacity");

            assert!(seen.insert(id));
        }
    }

    #[test]
    fn allocation_is_safe_for_concurrent_callers() {
        let allocator = Arc::new(IdAllocator::<AiModelId>::new());

        let workers = thread::available_parallelism()
            .map(|count| count.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for _ in 0..workers {
            let allocator = Arc::clone(&allocator);

            handles.push(thread::spawn(move || {
                let mut local = Vec::new();

                for _ in 0..1_000 {
                    local.push(
                        allocator.try_next()
                            .expect("allocator has capacity")
                    );
                }

                local
            }));
        }

        let mut collected = Vec::new();

        for handle in handles {
            collected.extend(
                handle.join().expect("worker must finish")
            );
        }

        let unique: HashSet<_> = collected.iter().copied().collect();

        assert_eq!(unique.len(), collected.len());
    }

    #[test]
    fn serde_round_trip_preserves_identifier() {
        let original = AiModelId::new(123)
            .expect("valid identifier");

        let encoded = serde_json::to_string(&original)
            .expect("serialization");

        let decoded: AiModelId = serde_json::from_str(&encoded)
            .expect("deserialization");

        assert_eq!(decoded, original);
        assert_eq!(decoded.get(), 123);
    }

    #[test]
    fn serde_rejects_zero() {
        let decoded = serde_json::from_str::<AiModelId>("0");

        assert!(decoded.is_err());
    }

    #[test]
    fn serde_preserves_ai_identifier_value() {
        let original = AiDatasetId::new(99)
            .expect("valid identifier");

        let encoded = serde_json::to_string(&original)
            .expect("serialization");

        let decoded: AiDatasetId = serde_json::from_str(&encoded)
            .expect("deserialization");

        assert_eq!(decoded, original);
    }

    #[test]
    fn universal_zuir_identifiers_remain_available() {
        let program = ProgramId::new(1)
            .expect("valid program ID");

        let operation = OperationId::new(1)
            .expect("valid operation ID");

        let value = ValueId::new(1)
            .expect("valid value ID");

        assert_eq!(program.get(), operation.get());
        assert_eq!(operation.get(), value.get());
    }

    #[test]
    fn ai_identifiers_use_canonical_allocator_contract() {
        let allocator = IdAllocator::<AiTrainingRunId>::new();

        let id = allocator.try_next()
            .expect("training run ID");

        assert_eq!(id.get(), 1);
        assert!(!allocator.is_exhausted());
    }
}
