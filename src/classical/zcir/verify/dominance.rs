
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Dominance Analysis — Canonical ZUIR Adapter
//!
//! Path: `src/classical/zcir/verify/dominance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Expose canonical ZUIR dominance analysis through the ZCIR verification
//! namespace without creating a competing control-flow or dominance model.
//!
//! ZCIR is an integration boundary for classical computation. It must not
//! duplicate canonical IR identities, blocks, regions, control-flow edges,
//! reachability facts, or immediate-dominator algorithms.
//!
//! ## Architectural authority
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/verify/control_flow.rs`
//! - `src/zuir/verify/dominance.rs`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## Ownership
//!
//! This module owns only the ZCIR-facing import path for dominance analysis.
//!
//! The canonical ZUIR verifier owns:
//!
//! - Dominance input and result types.
//! - Region-local control-flow graph construction.
//! - Reachability analysis.
//! - Immediate-dominator computation.
//! - Dominance diagnostics and analysis counters.
//!
//! This module does not own or redefine:
//!
//! - BlockId, RegionId, or any other canonical identity.
//! - Basic blocks, regions, terminators, or successor edges.
//! - SSA definitions, uses, or block arguments.
//! - Structural, type, effect, ownership, or resource verification.
//! - Optimization, scheduling, physical allocation, or execution.
//!
//! ## Public contract
//!
//! Callers provide the canonical ZUIR regions and blocks. Results and
//! diagnostics are returned by the canonical ZUIR verifier.
//!
//! A successful dominance report establishes only the properties checked by
//! that verifier. It does not establish complete SSA, type, effect, ownership,
//! structural, or full-program correctness.
//!
//! ## Control-flow and SSA requirements
//!
//! Dominance is region-local. The canonical verifier uses explicit successors
//! represented by the canonical block terminator API. It must not infer
//! hidden control-flow edges from operation names or source syntax.
//!
//! SSA verification must additionally establish that each referenced value
//! has a valid definition, that ordinary uses are dominated by their
//! definitions, and that same-block definitions precede their uses.
//!
//! Block arguments and edge-carried values require edge-sensitive checks:
//! values supplied to a successor must be valid at the source terminator,
//! and their arity and types must match the destination block arguments.
//!
//! Unreachable blocks require the explicitly specified unreachable-code
//! policy. Dominance analysis alone must not silently certify invalid SSA.
//!
//! ## Scalability
//!
//! This adapter introduces no block-count, edge-count, processor-count,
//! register-width, memory-capacity, or other universal scalability ceiling.
//!
//! Its resource consumption is that of the canonical implementation and the
//! host's available memory, address space, and execution budget.
//!
//! No finite implementation can guarantee literal infinite computation.
//! Operational limits must be explicit and must not be misrepresented as
//! universal Zamani language restrictions.
//!
//! ## Integration
//!
//! 1. Register this file in `src/classical/zcir/verify/mod.rs` only after that
//!    module exists and its module contract is reviewed.
//! 2. Register the verification module in `src/classical/zcir/mod.rs`.
//! 3. Ensure the classical subsystem is registered by the crate root.
//! 4. Run structural and control-flow verification before relying on dominance
//!    results for transformations.
//! 5. Run the appropriate SSA, type, effect, and full-verification stages.
//! 6. Ensure IR mutations invalidate or recompute cached analysis results.
//!
//! Do not add a second dominator tree or a second block/region identity model.
//!
//! ## Freeze criteria
//!
//! - The exported API uses canonical ZUIR types.
//! - No dominance algorithm is duplicated here.
//! - No unsafe Rust is introduced.
//! - The parent modules register this file deliberately.
//! - Repository-wide formatting, compilation, lint, and test gates pass.
//!
//! This file is an adapter, not evidence that the entire ZCIR subsystem or
//! every downstream SSA-verification path is production-ready.

/// Canonical dominance-analysis input, using ZUIR regions and blocks.
pub use crate::zuir::verify::dominance::DominanceInput;

/// Canonical dominance-analysis diagnostic.
pub use crate::zuir::verify::dominance::DominanceIssue;

/// Canonical dominance-analysis result information.
pub use crate::zuir::verify::dominance::DominanceInfo;

/// Canonical dominance-analysis report.
pub use crate::zuir::verify::dominance::DominanceReport;

/// Run canonical ZUIR dominance analysis through the ZCIR namespace.
///
/// This is the same implementation as
/// `crate::zuir::verify::dominance::verify_dominance`.
/// It does not mutate the input IR.
///
/// # Example
///
/// ```ignore
/// use crate::classical::zcir::verify::dominance::{
///     verify_dominance, DominanceInput,
/// };
///
/// let input = DominanceInput {
///     regions: &regions,
///     blocks: &blocks,
/// };
///
/// let report = verify_dominance(&input);
///
/// if !report.is_valid() {
///     for issue in report.issues() {
///         eprintln!("{issue}");
///     }
/// }
/// ```
pub use crate::zuir::verify::dominance::verify_dominance;

/// Descriptive alias for callers that prefer an analysis-oriented name.
///
/// Both names invoke the exact same canonical ZUIR implementation.
pub use crate::zuir::verify::dominance::verify_dominance as analyze_dominance;
