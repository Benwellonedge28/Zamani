
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Outlining
//!
//! Path: `src/compiler/lowering/outlining.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide a compiler-facing orchestration contract for semantics-preserving
//! outlining of computational entities.
//!
//! Outlining extracts a computational entity or region into one or more
//! separately representable entities while preserving the observable
//! semantics required by the source program.
//!
//! This module coordinates domain-owned outlining implementations. It does
//! not implement another IR, AST, semantic model, optimizer, verifier,
//! decomposition algorithm, partitioner, backend, or runtime.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Compiler-facing outlining provider contracts.
//! - Strategy selection and validation for one explicit request.
//! - Admission based on canonical legality evidence.
//! - Outlining result validation.
//! - Semantic-preservation verification admission.
//! - Deterministic outlined-entity normalization.
//! - Structured outlining errors and outcomes.
//!
//! This module does not own:
//!
//! - Source-language syntax or AST construction.
//! - Type inference, ownership analysis, or semantic authority.
//! - Canonical ZUIR or domain IR definitions.
//! - IR verification algorithms.
//! - Domain-specific outlining algorithms.
//! - Classical, quantum, HDL, AI, or tensor IR builders.
//! - Hybrid semantic composition.
//! - Distributed partitioning or placement.
//! - Physical resource allocation.
//! - Hardware discovery, routing, or scheduling.
//! - Backend code generation.
//! - Runtime execution.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Legality:
//!     crate::zuir::lowering::legality
//!
//! Lowering context:
//!     crate::zuir::lowering::context
//!
//! Verified compiler IR lowering:
//!     crate::compiler::ir::lowering
//!
//! Universal IR:
//!     crate::zuir
//!
//! Classical IR:
//!     crate::classical
//!
//! Quantum IR:
//!     crate::quantum::ir
//!
//! Hardware IR:
//!     crate::hdl::hdlir
//!
//! AI:
//!     crate::ai
//!
//! Hybrid composition:
//!     crate::hybrid
//!
//! ## OUTLINING SEMANTICS
//!
//! Outlining must preserve all applicable observable behavior, including:
//!
//! - Input and output values.
//! - Return behavior.
//! - Control-flow dependencies.
//! - Memory and aliasing behavior.
//! - Ownership and lifetime obligations.
//! - Side effects and effect ordering.
//! - Exception and failure behavior.
//! - Synchronization and concurrency semantics.
//! - Resource and capability requirements.
//! - Numerical and approximation guarantees.
//! - Quantum measurement and state semantics.
//! - HDL timing and signal semantics.
//! - Provenance and source correspondence.
//!
//! The applicable contract depends on the source representation and the
//! domain-owned outlining implementation.
//!
//! A provider's preservation-contract identifier is not proof of correctness.
//! The independent verifier must validate the generated artifact.
//!
//! Approximate transformations must not be accepted as exact outlining.
//! They require a separate explicit approximation contract and policy.
//!
//! ## LEGALITY
//!
//! This API accepts only a canonical `LegalityStatus::Legal` decision.
//!
//! Other decisions are not authorization to outline:
//!
//! - Conditional requires its conditions to be resolved.
//! - ExpansionRequired belongs to decomposition.
//! - Unsupported must remain unsupported.
//! - Deferred must remain unresolved.
//!
//! Outlining does not turn an unresolved legality decision into a legal one.
//!
//! ## ENTITY MAPPING
//!
//! Outlining may change entity categories. For example, a source region may
//! become a destination function.
//!
//! `LoweringContext::record_mapping()` deliberately rejects mappings between
//! different `EntityKind` values. This module does not weaken that invariant.
//!
//! Instead, the verified outcome explicitly retains the source entity and
//! its outlined destination entities. A future provenance adapter may
//! translate these relationships into a suitable cross-kind provenance
//! representation.
//!
//! ## SCALABILITY
//!
//! This module introduces no universal maximum for:
//!
//! - Functions or regions.
//! - Operations or instructions.
//! - Generated entities.
//! - Captured values.
//! - Modules or compilation units.
//! - Quantum operations or logical qubits.
//! - HDL signals or registers.
//! - Tensor dimensions or rank.
//! - AI graph nodes.
//! - Distributed participants.
//! - Computing domains.
//!
//! Collections grow dynamically.
//!
//! The provider owns its algorithmic complexity and detailed operational
//! accounting. The shared lowering context supplies optional work-budget
//! accounting.
//!
//! No finite implementation can promise literal infinity. Address-space,
//! allocator, integer-representation, algorithmic, and physical resource
//! constraints remain real and must be reported.
//!
//! ## DETERMINISM
//!
//! The orchestration layer:
//!
//! - Does not use random identifiers.
//! - Does not use global mutable state.
//! - Does not perform implicit hardware discovery.
//! - Sorts outlined entity identities deterministically.
//! - Rejects duplicate outlined entity identities.
//! - Rejects a mismatched strategy.
//! - Does not silently choose among competing providers.
//!
//! Provider determinism is part of the provider contract.
//!
//! ## TRANSACTIONAL BEHAVIOR
//!
//! The provider must return a complete artifact or an error.
//!
//! The compiler validates the provider output and runs the independent
//! verifier before constructing a successful outcome.
//!
//! This module does not publish the artifact through external mutable state.
//! Providers and verifiers must not expose partially constructed output
//! through external side effects.
//!
//! ## GRAMMAR BOUNDARY
//!
//! This module consumes established semantic and IR contracts.
//!
//! It does not parse grammar productions, introduce source keywords, or
//! determine the meaning of source-language constructs.
//!
//! Relevant specifications include:
//!
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/compilation-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//!
//! ## INTEGRATION
//!
//! Parent:
//!     src/compiler/lowering/mod.rs
//!
//! Compiler root:
//!     src/compiler/mod.rs
//!
//! Legality:
//!     src/zuir/lowering/legality.rs
//!
//! Context:
//!     src/zuir/lowering/context.rs
//!
//! Verified IR lowering:
//!     src/compiler/ir/lowering.rs
//!
//! Domain providers:
//!     src/zuir
//!     src/classical
//!     src/quantum/ir
//!     src/hdl/hdlir
//!     src/ai
//!     src/hybrid
//!     src/distributed
//!
//! ## FREEZE CRITERIA
//!
//! - Only explicitly legal candidates enter this API.
//! - Strategy identifiers match exactly.
//! - Domain algorithms remain domain-owned.
//! - The preservation verifier succeeds before an outcome is returned.
//! - Outlined entity identities are deterministic and unique.
//! - Cross-kind relationships do not misuse lowering mappings.
//! - Provider and verifier errors remain inspectable.
//! - No arbitrary hardware ceilings or unsafe code are introduced.
//! - Parent module declarations and repository integration tests pass.

// =============================================================================
// IMPORTS
// =============================================================================

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

use crate::zuir::lowering::context::{
    EntityKey,
    LoweringContext,
    LoweringContextError,
};

use crate::zuir::lowering::legality::{
    LegalityReport,
    LegalityStatus,
};

// =============================================================================
// API VERSION
// =============================================================================

/// Public API version for compiler outlining orchestration.
///
/// This version is independent of the Zamani language version, grammar
/// version, semantic-model version, ZUIR version, domain IR versions,
/// and backend ABI versions.
pub const OUTLINING_API_VERSION: u16 = 1;

// =============================================================================
// RESULT TYPES
// =============================================================================

/// Result alias for compiler outlining orchestration.
pub type OutliningResult<T, E, V> =
    Result<OutliningOutcome<T>, OutliningError<E, V>>;

/// Semantic-preservation contract declared by an outlining provider.
///
/// The identifier must refer to a stable, qualified domain contract.
/// It is not, by itself, proof that the outlining transformation is correct.
///
/// The independent verification callback must validate the generated
/// artifact against this contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreservationContract {
    /// Stable, qualified identifier of the preservation contract.
    pub identifier: String,
}

impl PreservationContract {
    /// Construct a preservation contract identifier.
    ///
    /// The identifier is validated when the provider output is accepted.
    #[must_use]
    pub fn new(identifier: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

/// Output returned by a domain-owned outlining provider.
///
/// The provider owns the generated artifact and identifies every outlined
/// entity associated with the source entity.
///
/// The compiler coordinator validates and normalizes these identities.
#[derive(Debug)]
pub struct OutliningOutput<T> {
    /// Artifact produced by the domain-owned outlining implementation.
    pub artifact: T,

    /// Entities created or exposed by the outlining transformation.
    ///
    /// Entity kinds may differ from the source entity kind. For example,
    /// a region may be outlined into a function.
    pub outlined_entities: Vec<EntityKey>,

    /// Contract governing semantic preservation.
    pub preservation: PreservationContract,
}

impl<T> OutliningOutput<T> {
    /// Construct an outlining output.
    #[must_use]
    pub fn new(
        artifact: T,
        outlined_entities: Vec<EntityKey>,
        preservation: PreservationContract,
    ) -> Self {
        Self {
            artifact,
            outlined_entities,
            preservation,
        }
    }
}

// =============================================================================
// REQUEST
// =============================================================================

/// Explicit request to execute one outlining strategy.
///
/// The legality report must have been produced by the canonical ZUIR
/// lowering-legality analysis using authoritative semantic and IR evidence.
///
/// The strategy identifier must identify the exact provider implementation
/// selected by the caller or an upstream registry.
#[derive(Debug, Clone, Copy)]
pub struct OutliningRequest<'a> {
    legality: &'a LegalityReport,
    strategy: &'a str,
}

impl<'a> OutliningRequest<'a> {
    /// Construct an outlining request.
    ///
    /// Strategy validation occurs when the request is executed.
    #[must_use]
    pub fn new(
        legality: &'a LegalityReport,
        strategy: &'a str,
    ) -> Self {
        Self {
            legality,
            strategy,
        }
    }

    /// Return the canonical legality report.
    #[must_use]
    pub fn legality(&self) -> &'a LegalityReport {
        self.legality
    }

    /// Return the requested outlining strategy.
    #[must_use]
    pub fn strategy(&self) -> &'a str {
        self.strategy
    }
}

// =============================================================================
// DOMAIN PROVIDER
// =============================================================================

/// Domain-owned outlining provider contract.
///
/// Each implementation owns its actual outlining algorithm and destination
/// IR construction.
///
/// Examples include:
//!
//! - Classical function or region outlining.
//! - Quantum computation-region outlining.
//! - HDL module or process outlining.
//! - AI graph or model-component outlining.
//! - Tensor computation outlining.
//! - Hybrid computation-boundary outlining.
//! - Distributed computation outlining.
//!
//! A provider must preserve all applicable source semantics and must return
//! a complete artifact or an error.
//!
//! The provider must not silently change program meaning.
//!
//! It must not perform physical resource allocation, hardware routing,
//! scheduling, or runtime execution through this interface.
pub trait OutliningProvider {
    /// Artifact type produced by this provider.
    type Artifact;

    /// Error type returned by the domain implementation.
    type Error: Error + 'static;

    /// Return the stable, qualified strategy identifier.
    fn strategy_id(&self) -> &str;

    /// Outline the source entity.
    ///
    /// This method is called only after:
    ///
    /// - The canonical legality status has been checked.
    /// - The requested strategy has been validated.
    /// - The requested strategy matches this provider.
    /// - The lowering context accepts the orchestration work charge.
    ///
    /// The provider receives the original source identity and immutable
    /// lowering context.
    ///
    /// It must not publish partially constructed output through external
    /// mutable state.
    fn outline(
        &mut self,
        source: &EntityKey,
        context: &LoweringContext,
    ) -> Result<OutliningOutput<Self::Artifact>, Self::Error>;
}

// =============================================================================
// OUTCOME
// =============================================================================

/// Successful result of one verified outlining operation.
///
/// This type is constructed only after:
///
/// 1. The legality decision explicitly authorizes the transformation.
/// 2. The strategy identifier is valid.
/// 3. The strategy matches the provider.
/// 4. The provider returns a complete artifact.
/// 5. The preservation contract identifier is valid.
/// 6. Outlined entity identities are valid and unique.
/// 7. The independent verification callback succeeds.
///
/// The outcome records the source-to-outlined-entity relationship without
/// incorrectly forcing cross-kind relationships into `LoweringContext`'s
/// same-kind mapping API.
///
/// A successful outcome does not prove physical target feasibility,
/// successful linking, or successful execution.
#[derive(Debug)]
pub struct OutliningOutcome<T> {
    source: EntityKey,
    artifact: T,
    outlined_entities: Vec<EntityKey>,
    preservation: PreservationContract,
    strategy: String,
    diagnostic_code: &'static str,
}

impl<T> OutliningOutcome<T> {
    /// Return the source entity that was outlined.
    #[must_use]
    pub fn source(&self) -> &EntityKey {
        &self.source
    }

    /// Borrow the generated artifact.
    #[must_use]
    pub fn artifact(&self) -> &T {
        &self.artifact
    }

    /// Consume the outcome and return the generated artifact.
    #[must_use]
    pub fn into_artifact(self) -> T {
        self.artifact
    }

    /// Return the deterministic outlined-entity identities.
    #[must_use]
    pub fn outlined_entities(&self) -> &[EntityKey] {
        &self.outlined_entities
    }

    /// Return the semantic-preservation contract.
    #[must_use]
    pub fn preservation(&self) -> &PreservationContract {
        &self.preservation
    }

    /// Return the selected outlining strategy.
    #[must_use]
    pub fn strategy(&self) -> &str {
        &self.strategy
    }

    /// Return the legality diagnostic code.
    #[must_use]
    pub fn diagnostic_code(&self) -> &'static str {
        self.diagnostic_code
    }

    /// Return whether the transformation produced outlined entities.
    ///
    /// An empty set is permitted only if the independent verifier accepts
    /// the artifact and its preservation contract. This allows a provider
    /// to represent a verified no-op where the selected strategy defines
    /// such behavior.
    #[must_use]
    pub fn has_outlined_entities(&self) -> bool {
        !self.outlined_entities.is_empty()
    }
}

// =============================================================================
// ERRORS
// =============================================================================

/// Errors produced by compiler outlining orchestration.
#[derive(Debug)]
pub enum OutliningError<E, V> {
    /// The legality report did not authorize outlining.
    LegalityNotEstablished {
        /// Stable legality diagnostic code.
        diagnostic_code: &'static str,
    },

    /// The requested strategy identifier is empty or malformed.
    InvalidStrategyIdentifier,

    /// The provider's strategy identifier is empty or malformed.
    InvalidProviderStrategyIdentifier,

    /// The provider does not implement the requested strategy.
    StrategyMismatch {
        /// Strategy requested by the caller.
        requested: String,

        /// Strategy supplied by the provider.
        provided: String,
    },

    /// The provider returned an invalid preservation-contract identifier.
    InvalidPreservationContract,

    /// The provider returned a duplicate outlined entity identity.
    DuplicateOutlinedEntity {
        /// Duplicate entity identity.
        entity: EntityKey,
    },

    /// The domain-owned outlining implementation failed.
    Provider(E),

    /// The independent artifact and semantic-preservation verifier failed.
    Verification(V),

    /// The canonical lowering context rejected work accounting.
    Context(LoweringContextError),
}

impl<E: fmt::Display, V: fmt::Display> fmt::Display
    for OutliningError<E, V>
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LegalityNotEstablished { diagnostic_code } => {
                write!(
                    formatter,
                    "outlining requires established legal semantics; received {diagnostic_code}"
                )
            }

            Self::InvalidStrategyIdentifier => {
                formatter.write_str(
                    "requested outlining strategy identifier is empty or malformed",
                )
            }

            Self::InvalidProviderStrategyIdentifier => {
                formatter.write_str(
                    "outlining provider strategy identifier is empty or malformed",
                )
            }

            Self::StrategyMismatch {
                requested,
                provided,
            } => {
                write!(
                    formatter,
                    "outlining strategy mismatch: requested '{requested}', provider supplies '{provided}'"
                )
            }

            Self::InvalidPreservationContract => {
                formatter.write_str(
                    "outlining provider returned an invalid semantic-preservation contract",
                )
            }

            Self::DuplicateOutlinedEntity { entity } => {
                write!(
                    formatter,
                    "outlining provider returned duplicate entity identity: {entity:?}"
                )
            }

            Self::Provider(error) => {
                write!(
                    formatter,
                    "domain outlining provider failed: {error}"
                )
            }

            Self::Verification(error) => {
                write!(
                    formatter,
                    "outlining artifact verification failed: {error}"
                )
            }

            Self::Context(error) => {
                write!(
                    formatter,
                    "outlining lowering context failed: {error}"
                )
            }
        }
    }
}

impl<E, V> Error for OutliningError<E, V>
where
    E: Error + 'static,
    V: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LegalityNotEstablished { .. } => None,
            Self::InvalidStrategyIdentifier => None,
            Self::InvalidProviderStrategyIdentifier => None,
            Self::StrategyMismatch { .. } => None,
            Self::InvalidPreservationContract => None,
            Self::DuplicateOutlinedEntity { .. } => None,
            Self::Provider(error) => Some(error),
            Self::Verification(error) => Some(error),
            Self::Context(error) => Some(error),
        }
    }
}

// =============================================================================
// VALIDATION
// =============================================================================

fn is_valid_identifier(value: &str) -> bool {
    !value.trim().is_empty() && !value.contains('\0')
}

fn normalize_entities(
    entities: Vec<EntityKey>,
) -> Result<Vec<EntityKey>, EntityKey> {
    let mut normalized = BTreeSet::new();

    for entity in entities {
        if !normalized.insert(entity.clone()) {
            return Err(entity);
        }
    }

    Ok(normalized.into_iter().collect())
}

// =============================================================================
// OUTLINING ENTRY POINT
// =============================================================================

/// Execute one explicitly requested, domain-owned outlining operation.
///
/// The supplied verification callback is mandatory. It must verify the
/// generated artifact against the original source entity, the complete
/// legality report, and the provider's preservation contract.
///
/// The verifier should validate, as applicable:
///
/// - Source and destination IR structural validity.
/// - Input and output signature compatibility.
/// - Captured-value completeness.
/// - Closure and environment correctness.
/// - Control-flow and return behavior.
/// - Effect preservation and effect ordering.
/// - Memory, ownership, aliasing, and lifetime obligations.
/// - Exception and failure behavior.
/// - Synchronization and concurrency semantics.
/// - Numerical and approximation guarantees.
/// - Domain-specific invariants.
/// - Provenance and source correspondence.
///
/// Merely checking that the output artifact is non-empty is insufficient.
///
/// # Errors
///
/// Returns an error if legality is not established, the strategy does not
/// match, the provider fails, its output is malformed, the verifier rejects
/// the output, or the lowering context cannot charge the operation.
pub fn outline_with<P, Verify, VerifyError>(
    request: OutliningRequest<'_>,
    context: &mut LoweringContext,
    provider: &mut P,
    verify: Verify,
) -> OutliningResult<P::Artifact, P::Error, VerifyError>
where
    P: OutliningProvider,
    Verify: FnOnce(
        &LegalityReport,
        &EntityKey,
        &P::Artifact,
        &[EntityKey],
        &PreservationContract,
    ) -> Result<(), VerifyError>,
    VerifyError: Error + 'static,
{
    let legality = request.legality;

    // Only an explicit canonical Legal decision authorizes outlining.
    if !matches!(&legality.status, LegalityStatus::Legal) {
        return Err(OutliningError::LegalityNotEstablished {
            diagnostic_code: legality.diagnostic_code,
        });
    }

    // Validate the requested strategy before invoking any provider.
    if !is_valid_identifier(request.strategy) {
        return Err(OutliningError::InvalidStrategyIdentifier);
    }

    // Validate the provider strategy independently.
    let provider_strategy = provider.strategy_id();

    if !is_valid_identifier(provider_strategy) {
        return Err(OutliningError::InvalidProviderStrategyIdentifier);
    }

    // A caller must not accidentally invoke a provider for another strategy.
    if request.strategy != provider_strategy {
        return Err(OutliningError::StrategyMismatch {
            requested: request.strategy.to_owned(),
            provided: provider_strategy.to_owned(),
        });
    }

    // Charge one orchestration work unit. The provider remains responsible
    // for charging its own detailed algorithmic work.
    context
        .charge_work(1)
        .map_err(OutliningError::Context)?;

    // The provider owns the actual transformation and destination IR.
    let output = provider
        .outline(&legality.source, context)
        .map_err(OutliningError::Provider)?;

    // Validate the provider's declared preservation contract.
    if !is_valid_identifier(&output.preservation.identifier) {
        return Err(OutliningError::InvalidPreservationContract);
    }

    // Normalize identities deterministically and reject duplicates.
    let outlined_entities =
        normalize_entities(output.outlined_entities).map_err(|entity| {
            OutliningError::DuplicateOutlinedEntity { entity }
        })?;

    // Independent verification is mandatory. A provider's declaration is
    // not proof that its transformation preserves source semantics.
    verify(
        legality,
        &legality.source,
        &output.artifact,
        &outlined_entities,
        &output.preservation,
    )
    .map_err(OutliningError::Verification)?;

    // Construct a successful outcome only after verification succeeds.
    Ok(OutliningOutcome {
        source: legality.source.clone(),
        artifact: output.artifact,
        outlined_entities,
        preservation: output.preservation,
        strategy: request.strategy.to_owned(),
        diagnostic_code: legality.diagnostic_code,
    })
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::lowering::context::EntityKind;

    #[test]
    fn strategy_identifier_rejects_empty_values() {
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("   "));
    }

    #[test]
    fn strategy_identifier_rejects_nul() {
        assert!(!is_valid_identifier("outline\0region"));
    }

    #[test]
    fn strategy_identifier_accepts_qualified_values() {
        assert!(is_valid_identifier("classical.region.v1"));
        assert!(is_valid_identifier("quantum.computation.extract"));
        assert!(is_valid_identifier("hdl.module.outline"));
        assert!(is_valid_identifier("extension.vendorless.strategy"));
    }

    #[test]
    fn entity_normalization_is_deterministic() {
        let first = EntityKey::new(
            EntityKind::Function,
            "test",
            "function.b",
        )
        .expect("valid entity");

        let second = EntityKey::new(
            EntityKind::Function,
            "test",
            "function.a",
        )
        .expect("valid entity");

        let normalized = normalize_entities(vec![
            first.clone(),
            second.clone(),
        ])
        .expect("unique entities");

        assert_eq!(normalized, vec![second, first]);
    }

    #[test]
    fn entity_normalization_rejects_duplicates() {
        let entity = EntityKey::new(
            EntityKind::Function,
            "test",
            "function.a",
        )
        .expect("valid entity");

        let result = normalize_entities(vec![
            entity.clone(),
            entity,
        ]);

        assert!(matches!(
            result,
            Err(duplicate)
                if duplicate.value() == "function.a"
        ));
    }

    #[test]
    fn outlining_preservation_contract_retains_identifier() {
        let contract =
            PreservationContract::new("classical.region.preservation.v1");

        assert_eq!(
            contract.identifier,
            "classical.region.preservation.v1"
        );
    }
}
