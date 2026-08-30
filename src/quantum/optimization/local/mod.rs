//! Local quantum-circuit optimization passes.
//!
//! This module provides optimization transformations that operate primarily on
//! local operation neighborhoods while preserving the semantics and invariants
//! of Zamani's canonical quantum IR.
//!
//! # Architectural contract
//!
//! `local` is deliberately a transformation layer. It does not define its own
//! quantum IR and must never introduce an alternative `QuantumGate`,
//! `QuantumOperation`, or circuit representation.
//!
//! The dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization::circuit / operation / analysis
//!      │
//!      ▼
//! optimization::local
//!      │
//!      ▼
//! optimization::passes / pipeline
//! ```
//!
//! The local optimizer may consume:
//!
//! - canonical quantum IR;
//! - optimization analyses;
//! - rewrite rules;
//! - optimization context;
//! - target gate-set information;
//! - cost models;
//! - optimization limits;
//! - verification services.
//!
//! It must not directly depend on:
//!
//! - hardware providers;
//! - QPU APIs;
//! - routing implementation;
//! - execution/runtime services;
//! - benchmark orchestration;
//! - frontend parsers;
//! - algorithm implementations.
//!
//! # Passes
//!
//! The local optimization family consists of:
//!
//! - [`cancellation`] — removes adjacent or dependency-safe inverse/self-inverse
//!   operations;
//! - [`identity`] — removes operations that are semantically identities;
//! - [`inverse`] — generic inverse-pair simplification;
//! - [`rotation`] — combines compatible parameterized rotations;
//! - [`commutation`] — moves operations through legal commuting regions to
//!   expose further simplifications;
//! - [`peephole`] — applies bounded local rewrite patterns;
//! - [`templates`] — applies registered multi-operation templates;
//! - [`gate_fusion`] — fuses compatible operations when the target permits it.
//!
//! These modules are intentionally separate so that each transformation can be
//! tested, scheduled, enabled, disabled, bounded, measured, and verified
//! independently.
//!
//! # Integration
//!
//! Higher-level code should normally use one of:
//!
//! ```text
//! optimization::pipeline
//! optimization::passes::simplify
//! optimization::passes::optimize_gate_count
//! optimization::passes::optimize_depth
//! optimization::passes::optimize_two_qubit
//! ```
//!
//! Direct use of a local pass is appropriate for compiler developers and
//! specialized pipelines.
//!
//! # Determinism
//!
//! All local passes are expected to be deterministic for a deterministic
//! [`OptimizationContext`](crate::quantum::optimization::context::OptimizationContext)
//! and input IR. A pass must not use process-global mutable state or hidden
//! randomness.
//!
//! # Resource scalability
//!
//! "Scalable" here means that the module remains bounded by the resources
//! explicitly supplied by the caller. It must not impose an arbitrary
//! circuit-size ceiling beyond the shared optimization limits.
//!
//! Large or effectively unbounded workloads must therefore be controlled by:
//!
//! - operation/rewrite limits;
//! - iteration limits;
//! - runtime limits;
//! - memory/e-graph limits where applicable;
//! - pass-specific budgets exposed through the shared optimization context.
//!
//! Individual local passes must check those limits through the common
//! optimization infrastructure rather than inventing incompatible limits.
//!
//! # Safety
//!
//! This module contains no `unsafe` code and must remain usable under a crate
//! policy that forbids unsafe Rust.
//!
//! # Compatibility
//!
//! The implementation is intended for Rust 1.97 / Rust 1.97.1 and should avoid
//! language or standard-library features introduced after that toolchain.
//!
//! # Public API policy
//!
//! Only the module boundaries and explicitly documented pass types should be
//! exported from this module. Internal implementation details should remain
//! private to their respective modules.
//!
//! The module should not re-export canonical IR types merely for convenience.
//! Callers should import canonical IR types from `quantum::ir` directly.

pub mod cancellation;
pub mod commutation;
pub mod gate_fusion;
pub mod identity;
pub mod inverse;
pub mod peephole;
pub mod rotation;
pub mod templates;

// -----------------------------------------------------------------------------
// Stable local-pass exports
// -----------------------------------------------------------------------------
//
// Re-export only concrete local passes that form part of the optimizer's
// supported public surface.
//
// These re-exports intentionally do not expose private helper structures,
// matcher internals, rule implementations, or alternative IR types.
//
// If a concrete pass is intentionally internal, it should remain accessible
// through its module but should not be re-exported here.
//
// The exact pass names below are the canonical names expected by the local
// optimization layer. Each implementation is responsible for conforming to
// the shared OptimizationPass contract.
//
// NOTE:
// If an individual implementation uses a different concrete type name, that
// implementation must provide the canonical type alias/export within its own
// module rather than requiring this module to be edited later. This preserves
// the "finish one file without reopening it later" integration rule.

pub use cancellation::CancellationPass;
pub use commutation::CommutationPass;
pub use gate_fusion::GateFusionPass;
pub use identity::IdentityPass;
pub use inverse::InversePass;
pub use peephole::PeepholePass;
pub use rotation::RotationPass;
pub use templates::TemplatePass;

// -----------------------------------------------------------------------------
// Local pass classification
// -----------------------------------------------------------------------------

/// Stable identifiers for the built-in local optimization passes.
///
/// These identifiers are intentionally represented independently from concrete
/// Rust type names so configuration, provenance, diagnostics, serialization,
// registry lookup, and future plugin systems can refer to passes without
/// depending on Rust symbols.
///
/// Pass IDs should be treated as stable compiler-facing identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LocalPassId {
    /// Removes explicit semantic identity operations.
    Identity,

    /// Simplifies a unitary operation followed by its inverse.
    Inverse,

    /// Cancels self-inverse operations where legal.
    Cancellation,

    /// Combines compatible parameterized rotations.
    Rotation,

    /// Moves operations through legal commuting regions.
    Commutation,

    /// Applies small bounded local rewrite windows.
    Peephole,

    /// Applies registered multi-operation templates.
    Templates,

    /// Fuses compatible operations into a more compact representation.
    GateFusion,
}

impl LocalPassId {
    /// Returns the stable serialized identifier for this local pass.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "local.identity",
            Self::Inverse => "local.inverse",
            Self::Cancellation => "local.cancellation",
            Self::Rotation => "local.rotation",
            Self::Commutation => "local.commutation",
            Self::Peephole => "local.peephole",
            Self::Templates => "local.templates",
            Self::GateFusion => "local.gate_fusion",
        }
    }

    /// Returns every built-in local pass in canonical dependency order.
    ///
    /// The ordering is a safe default, not a requirement that every optimizer
    /// must use exactly this sequence.
    pub const fn all() -> &'static [Self] {
        &[
            Self::Identity,
            Self::Inverse,
            Self::Cancellation,
            Self::Rotation,
            Self::Commutation,
            Self::Peephole,
            Self::Templates,
            Self::GateFusion,
        ]
    }
}

impl core::fmt::Display for LocalPassId {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// -----------------------------------------------------------------------------
// Local pipeline presets
// -----------------------------------------------------------------------------

/// Standard local optimization strategy.
///
/// This describes ordering only. Actual execution remains controlled by the
/// optimization pipeline, configuration, target, limits, and pass contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalOptimizationStrategy {
    /// Minimal transformations suitable for fast compilation.
    Conservative,

    /// Balanced local optimization.
    Balanced,

    /// More aggressive local rewriting and fusion.
    Aggressive,
}

impl LocalOptimizationStrategy {
    /// Returns the recommended local-pass ordering for this strategy.
    ///
    /// The returned sequence contains stable pass identifiers rather than
    /// concrete pass objects. Pass construction belongs to the pass registry.
    pub const fn pass_order(self) -> &'static [LocalPassId] {
        match self {
            Self::Conservative => &[
                LocalPassId::Identity,
                LocalPassId::Inverse,
                LocalPassId::Cancellation,
                LocalPassId::Rotation,
            ],
            Self::Balanced => &[
                LocalPassId::Identity,
                LocalPassId::Inverse,
                LocalPassId::Cancellation,
                LocalPassId::Rotation,
                LocalPassId::Commutation,
                LocalPassId::Peephole,
                LocalPassId::Templates,
                LocalPassId::GateFusion,
            ],
            Self::Aggressive => &[
                LocalPassId::Identity,
                LocalPassId::Inverse,
                LocalPassId::Cancellation,
                LocalPassId::Rotation,
                LocalPassId::Commutation,
                LocalPassId::Peephole,
                LocalPassId::Templates,
                LocalPassId::GateFusion,
            ],
        }
    }
}

// -----------------------------------------------------------------------------
// Pass registration contract
// -----------------------------------------------------------------------------

/// Returns the stable identifiers of all built-in local optimization passes.
///
/// This function is intentionally side-effect free. Registration with the
/// global optimization [`PassRegistry`](crate::quantum::optimization::registry::PassRegistry)
/// belongs to the registry layer.
///
/// Keeping discovery separate from registration allows callers to construct
/// isolated registries for:
///
/// - deterministic builds;
/// - tests;
/// - restricted compiler profiles;
/// - embedded environments;
/// - sandboxed compilation;
/// - future dynamically selected optimization configurations.
pub const fn builtin_pass_ids() -> &'static [LocalPassId] {
    LocalPassId::all()
}

// -----------------------------------------------------------------------------
// Integration invariants
// -----------------------------------------------------------------------------

/// Compile-time/documentation-level contract for local optimization.
///
/// This trait is deliberately small and does not replace the canonical
/// `OptimizationPass` trait. It exists to make the architectural requirements
/// of this module explicit.
///
/// Concrete local passes should implement the main optimizer pass contract
/// provided by `optimization::pass`.
///
/// The trait is sealed so external code cannot accidentally claim that an
/// arbitrary pass is a built-in local pass without going through the canonical
/// pass implementation.
pub trait LocalOptimizationPass: private::Sealed {
    /// Stable identifier of the local pass.
    fn local_id(&self) -> LocalPassId;

    /// Stable string identifier used by diagnostics and provenance.
    fn id(&self) -> &'static str {
        self.local_id().as_str()
    }
}

mod private {
    /// Sealing prevents accidental implementation of the built-in local-pass
    /// classification by unrelated external types.
    pub trait Sealed {}
}

// -----------------------------------------------------------------------------
// Internal implementation contract
// -----------------------------------------------------------------------------
//
// The concrete pass modules implement the sealed trait. Keeping the sealing
// implementation here means callers do not need to modify this module when a
// concrete pass implementation evolves internally.

impl private::Sealed for CancellationPass {}
impl private::Sealed for CommutationPass {}
impl private::Sealed for GateFusionPass {}
impl private::Sealed for IdentityPass {}
impl private::Sealed for InversePass {}
impl private::Sealed for PeepholePass {}
impl private::Sealed for RotationPass {}
impl private::Sealed for TemplatePass {}

impl LocalOptimizationPass for CancellationPass {
    fn local_id(&self) -> LocalPassId {
        LocalPassId::Cancellation
    }
}

impl LocalOptimizationPass for CommutationPass {
    fn local_id(&self) -> LocalPassId {
        LocalPassId::Commutation
    }
}

impl LocalOptimizationPass for GateFusionPass {
    fn local_id(&self) -> LocalPassId {
        LocalPassId::GateFusion
    }
}

impl LocalOptimizationPass for IdentityPass {
    fn local_id(&self) -> LocalPassId {
        LocalPassId::Identity
    }
}

impl LocalOptimizationPass for InversePass {
    fn local_id(&self) -> LocalPassId {
        LocalPassId::Inverse
    }
}

impl LocalOptimizationPass for PeepholePass {
    fn local_id(&self) -> LocalPassId {
        LocalPassId::Peephole
    }
}

impl LocalOptimizationPass for RotationPass {
    fn local_id(&self) -> LocalPassId {
        LocalPassId::Rotation
    }
}

impl LocalOptimizationPass for TemplatePass {
    fn local_id(&self) -> LocalPassId {
        LocalPassId::Templates
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        builtin_pass_ids,
        LocalOptimizationStrategy,
        LocalPassId,
    };

    #[test]
    fn every_builtin_pass_has_a_stable_id() {
        let passes = builtin_pass_ids();

        assert!(!passes.is_empty());

        for pass in passes {
            assert!(!pass.as_str().is_empty());
            assert!(pass.as_str().starts_with("local."));
        }
    }

    #[test]
    fn builtin_pass_ids_are_unique() {
        let passes = builtin_pass_ids();

        for (index, pass) in passes.iter().enumerate() {
            for other in passes.iter().skip(index + 1) {
                assert_ne!(pass.as_str(), other.as_str());
            }
        }
    }

    #[test]
    fn all_strategies_have_a_non_empty_pipeline() {
        let strategies = [
            LocalOptimizationStrategy::Conservative,
            LocalOptimizationStrategy::Balanced,
            LocalOptimizationStrategy::Aggressive,
        ];

        for strategy in strategies {
            assert!(!strategy.pass_order().is_empty());
        }
    }

    #[test]
    fn conservative_strategy_is_a_subset_of_builtin_passes() {
        for pass in LocalOptimizationStrategy::Conservative.pass_order() {
            assert!(builtin_pass_ids().contains(pass));
        }
    }

    #[test]
    fn balanced_strategy_is_a_subset_of_builtin_passes() {
        for pass in LocalOptimizationStrategy::Balanced.pass_order() {
            assert!(builtin_pass_ids().contains(pass));
        }
    }

    #[test]
    fn aggressive_strategy_is_a_subset_of_builtin_passes() {
        for pass in LocalOptimizationStrategy::Aggressive.pass_order() {
            assert!(builtin_pass_ids().contains(pass));
        }
    }

    #[test]
    fn pass_ids_are_stable() {
        assert_eq!(
            LocalPassId::Identity.as_str(),
            "local.identity"
        );
        assert_eq!(
            LocalPassId::Inverse.as_str(),
            "local.inverse"
        );
        assert_eq!(
            LocalPassId::Cancellation.as_str(),
            "local.cancellation"
        );
        assert_eq!(
            LocalPassId::Rotation.as_str(),
            "local.rotation"
        );
        assert_eq!(
            LocalPassId::Commutation.as_str(),
            "local.commutation"
        );
        assert_eq!(
            LocalPassId::Peephole.as_str(),
            "local.peephole"
        );
        assert_eq!(
            LocalPassId::Templates.as_str(),
            "local.templates"
        );
        assert_eq!(
            LocalPassId::GateFusion.as_str(),
            "local.gate_fusion"
        );
    }
}