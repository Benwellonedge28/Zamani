
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Region Integration
//!
//! Canonical path: `src/classical/zcir/ir/region.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Expose ZUIR's canonical region representation through the ZCIR namespace.
//! A region groups ordered block references and declares region ownership,
//! structural interpretation, capture policy, captures, and attributes.
//!
//! This is an adapter, not an independent region implementation.
//!
//! ## Authority
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/ir/use_def.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! ## Ownership
//!
//! ZUIR remains the sole owner of region IDs, region data, region-local
//! validation, capture declarations, attributes, and region errors.
//! Block contents remain owned by `ir/block.rs` and operations by
//! `ir/operation.rs`. Function and operation modules declare region
//! ownership through canonical `RegionOwner` values.
//!
//! This file owns only the ZCIR-facing names and compatibility boundary.
//!
//! ## Integration contract
//!
//! - Function-owned regions use the canonical `RegionOwner::Function`.
//! - Operation-owned regions use the canonical `RegionOwner::Operation`.
//! - Region block lists contain canonical `BlockId` references.
//! - Captures refer to canonical `ValueId` identities.
//! - Attributes use canonical schema types.
//! - The full-IR verifier resolves references and checks reciprocal
//!   ownership, visibility, type correctness, control flow, effects,
//!   dominance, and dialect legality.
//! - Builders and transformations must not create competing region IDs or
//!   independent copies of region state.
//! - Transformations that change block membership must also repair affected
//!   control-flow edges and invalidate or rebuild derived analyses.
//!
//! ## Semantic boundary
//!
//! Grammar and frontend code define and parse source syntax. Semantic
//! analysis resolves names, types, effects, capabilities, and requirements.
//! This module receives canonical IR identities and does not parse source
//! text or infer the meaning of a region.
//!
//! A region's structural kind does not itself implement scheduling,
//! execution, hardware placement, or domain-specific operation semantics.
//!
//! ## Scalability and POCO-REAF
//!
//! This facade introduces no universal maximum number of regions, blocks,
//! captures, attributes, operations, processors, devices, or nodes.
//! Storage and validation use the canonical ZUIR implementation.
//!
//! Actual execution is limited by available memory, time, identifier space,
//! target capabilities, and other real constraints. Literal infinity is
//! not physically achievable. Resource exhaustion or target infeasibility
//! must be reported rather than silently changing program meaning.
//!
//! Operational budgets belong to the relevant compiler, verifier, decoder,
//! or runtime configuration. They must not become universal semantic
//! ceilings.
//!
//! ## Validation and trust
//!
//! Region-local validation does not prove that referenced blocks or captured
//! values exist, nor does it establish ownership reciprocity or whole-program
//! correctness. Imported or deserialized IR must pass the full verifier
//! before trusted optimization, lowering, or execution.
//!
//! ## Compatibility and freeze criteria
//!
//! These aliases preserve canonical type identity; they do not promise that
//! all future ZUIR API changes are source-compatible. Persisted-format
//! compatibility follows the canonical ZUIR versioning contract.
//!
//! Freeze this adapter only when its parent module registers it, the
//! canonical APIs compile, focused tests pass, and the repository's
//! integration gates pass.

/// Canonical region representation.
pub use crate::zuir::ir::region::ZuirRegion;

/// Canonical region owner declaration.
pub use crate::zuir::ir::region::RegionOwner;

/// Canonical region structural interpretation.
pub use crate::zuir::ir::region::RegionKind;

/// Canonical region capture-isolation policy.
pub use crate::zuir::ir::region::RegionIsolation;

/// Canonical capture access mode.
pub use crate::zuir::ir::region::CaptureMode;

/// Canonical region-local error type.
pub use crate::zuir::ir::region::RegionError;

/// Canonical region construction and validation result type.
pub use crate::zuir::ir::region::RegionResult;

/// ZCIR-facing alias for the canonical ZUIR region.
pub type ZcirRegion = ZuirRegion;

/// ZCIR-facing alias for the canonical region owner.
pub type ZcirRegionOwner = RegionOwner;

/// ZCIR-facing alias for the canonical region kind.
pub type ZcirRegionKind = RegionKind;

/// ZCIR-facing alias for the canonical region isolation policy.
pub type ZcirRegionIsolation = RegionIsolation;

/// ZCIR-facing alias for the canonical capture mode.
pub type ZcirCaptureMode = CaptureMode;

/// ZCIR-facing alias for the canonical region error.
pub type ZcirRegionError = RegionError;

/// ZCIR-facing alias for the canonical region result.
pub type ZcirRegionResult<T> = RegionResult<T>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::{
        BlockId, FunctionId, OperationId, RegionId, ValueId,
    };

    fn region_id(raw: u128) -> RegionId {
        RegionId::new(raw).expect("test region ID must be nonzero")
    }

    fn function_id(raw: u128) -> FunctionId {
        FunctionId::new(raw).expect("test function ID must be nonzero")
    }

    fn operation_id(raw: u128) -> OperationId {
        OperationId::new(raw).expect("test operation ID must be nonzero")
    }

    fn block_id(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block ID must be nonzero")
    }

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    fn function_region() -> ZcirRegion {
        ZcirRegion::with_blocks(
            region_id(1),
            ZcirRegionOwner::Function(function_id(2)),
            ZcirRegionKind::Structured,
            ZcirRegionIsolation::CapturesAllowed,
            vec![block_id(3)],
        )
        .expect("test region should be valid")
    }

    #[test]
    fn zcir_region_is_the_canonical_zuir_region() {
        fn accepts_zcir(_: ZcirRegion) {}
        fn accepts_zuir(_: ZuirRegion) {}

        let region = function_region();

        accepts_zcir(region.clone());
        accepts_zuir(region.clone());

        assert_eq!(
            core::any::type_name::<ZcirRegion>(),
            core::any::type_name::<ZuirRegion>(),
        );
        assert_eq!(region.id(), region_id(1));
        assert_eq!(
            region.owner(),
            ZcirRegionOwner::Function(function_id(2)),
        );
    }

    #[test]
    fn region_preserves_ordered_block_references() {
        let mut region = function_region();

        region
            .add_block(block_id(4))
            .expect("new block reference should be accepted");

        assert_eq!(region.blocks(), &[block_id(3), block_id(4)]);
        assert_eq!(region.block_count(), 2);
        assert!(region.contains_block(block_id(3)));
        assert!(region.contains_block(block_id(4)));
        assert!(region.validate().is_ok());
    }

    #[test]
    fn duplicate_block_references_are_rejected() {
        let mut region = function_region();

        let error = region
            .add_block(block_id(3))
            .expect_err("duplicate block reference must be rejected");

        assert_eq!(
            error,
            ZcirRegionError::DuplicateBlock(block_id(3)),
        );
        assert_eq!(region.blocks(), &[block_id(3)]);
    }

    #[test]
    fn invalid_block_insertion_does_not_mutate_the_region() {
        let mut region = function_region();

        let error = region
            .insert_block(2, block_id(4))
            .expect_err("index beyond the end must be rejected");

        assert_eq!(
            error,
            ZcirRegionError::InvalidBlockIndex {
                index: 2,
                block_count: 1,
            },
        );
        assert_eq!(region.blocks(), &[block_id(3)]);
    }

    #[test]
    fn isolated_regions_reject_captures() {
        let mut region = function_region();

        region
            .set_isolation(ZcirRegionIsolation::Isolated)
            .expect("region without captures may become isolated");

        let error = region
            .add_capture(value_id(5), ZcirCaptureMode::Read)
            .expect_err("isolated regions cannot declare captures");

        assert_eq!(
            error,
            ZcirRegionError::CaptureInIsolatedRegion(value_id(5)),
        );
        assert_eq!(region.capture_count(), 0);
    }

    #[test]
    fn capture_declarations_preserve_their_modes() {
        let mut region = function_region();

        region
            .add_capture(value_id(6), ZcirCaptureMode::Read)
            .expect("capture should be accepted");

        assert_eq!(
            region.capture_mode(value_id(6)),
            Some(ZcirCaptureMode::Read),
        );

        assert!(region
            .set_capture_mode(value_id(6), ZcirCaptureMode::Write)
            .expect("existing capture mode can be changed"));

        assert_eq!(
            region.capture_mode(value_id(6)),
            Some(ZcirCaptureMode::Write),
        );

        assert!(region.remove_capture(value_id(6)));
        assert_eq!(region.capture_count(), 0);
    }

    #[test]
    fn operation_owned_regions_use_canonical_operation_identity() {
        let region = ZcirRegion::with_blocks(
            region_id(7),
            ZcirRegionOwner::Operation(operation_id(8)),
            ZcirRegionKind::ControlFlow,
            ZcirRegionIsolation::Isolated,
            vec![block_id(9)],
        )
        .expect("operation-owned region should be valid");

        assert_eq!(
            region.owner(),
            ZcirRegionOwner::Operation(operation_id(8)),
        );
        assert_eq!(region.kind(), &ZcirRegionKind::ControlFlow);
        assert_eq!(region.isolation(), ZcirRegionIsolation::Isolated);
        assert!(region.validate().is_ok());
    }

    #[test]
    fn empty_region_is_not_a_valid_complete_region() {
        let region = ZcirRegion::new(
            region_id(10),
            ZcirRegionOwner::Function(function_id(11)),
            ZcirRegionKind::Structured,
            ZcirRegionIsolation::CapturesAllowed,
        );

        assert!(!region.has_blocks());
        assert_eq!(
            region.validate(),
            Err(ZcirRegionError::EmptyRegion),
        );
    }

    #[test]
    fn replacing_blocks_rejects_duplicates_without_mutating_existing_state() {
        let mut region = function_region();

        let error = region
            .set_blocks(vec![block_id(12), block_id(12)])
            .expect_err("replacement with duplicate IDs must fail");

        assert_eq!(
            error,
            ZcirRegionError::DuplicateBlock(block_id(12)),
        );
        assert_eq!(region.blocks(), &[block_id(3)]);
    }

    #[test]
    fn region_error_and_result_are_canonical_aliases() {
        assert_eq!(
            core::any::type_name::<ZcirRegionError>(),
            core::any::type_name::<RegionError>(),
        );
    }
}
