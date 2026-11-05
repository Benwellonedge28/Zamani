
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Decomposition Orchestration
//!
//! Path: `src/compiler/lowering/decomposition.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the compiler-facing orchestration boundary for semantic-preserving
//! decomposition of computational entities.
//!
//! Decomposition expands an entity into a composition of other entities when
//! direct lowering is not justified or supported, but an explicitly
//! registered expansion strategy is available.
//!
//! This module coordinates domain-owned decomposition implementations.
//! It does not implement another IR, semantic model, optimizer, verifier,
//! backend, target registry, or domain-specific decomposition algorithm.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Compiler-facing decomposition provider contracts.
//! - Strategy matching against canonical legality decisions.
//! - Decomposition result validation orchestration.
//! - Semantic-preservation verification admission.
//! - Deterministic destination normalization.
//! - Source-to-destination mapping registration.
//! - Structured decomposition errors and outcomes.
//!
//! This module does not own:
//!
//! - Source-language syntax or AST construction.
//! - Semantic analysis or type inference.
//! - Canonical ZUIR or domain IR definitions.
//! - IR verification algorithms.
//! - Domain-specific decomposition algorithms.
//! - Quantum gate synthesis or decomposition.
//! - Classical instruction expansion.
//! - HDL synthesis or technology mapping.
//! - AI or tensor graph decomposition.
//! - Distributed partitioning.
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
//! Lowering context and mappings:
//!     crate::zuir::lowering::context
//!
//! Direct lowering:
//!     crate::zuir::lowering::api
//!
//! Compiler representation integration:
//!     crate::compiler::ir
//!
//! Quantum decomposition:
//!     crate::quantum::optimization::synthesis::decomposition
//!
//! Classical IR:
//!     crate::classical::zcir
//!
//! Hardware description:
//!     crate::hdl::hdlir
//!
//! AI:
//!     crate::ai
//!
//! ## SEMANTIC SAFETY
//!
//! Decomposition is not automatically semantics-preserving merely because
//! the output contains valid IR entities.
//!
//! A provider must identify the preservation contract governing its
//! transformation. An independent verifier must validate the generated
//! artifact against the original legality report and that contract.
//!
//! The compiler accepts only artifacts that pass this verification callback.
//!
//! This module does not claim to mathematically prove arbitrary semantic
//! equivalence. The supplied verifier must be authoritative for the
//! representation and transformation being checked.
//!
//! Approximate transformations must not be represented as exact
//! decomposition. They require a separate explicit approximation contract
//! and policy-controlled compiler path.
//!
//! ## LEGALITY
//!
//! Decomposition is invoked only when the canonical legality report contains:
//!
//!     LegalityStatus::ExpansionRequired { strategy }
//!
//! Directly legal operations must use the normal lowering API.
//! Conditional, deferred, unsupported, or otherwise unresolved operations
//! must not be treated as decomposition requests.
//!
//! ## SCALABILITY
//!
//! This module introduces no universal maximum for:
//!
//! - Operations or instructions.
//! - Quantum gates or logical qubits.
//! - Signals, registers, or hardware structures.
//! - Tensor dimensions or tensor rank.
//! - AI graph nodes.
//! - Distributed participants.
//! - Generated entities.
//! - Computing domains.
//! - Decomposition strategies.
//!
//! Collections grow dynamically.
//!
//! The domain provider owns its algorithmic complexity and operational
//! resource accounting. The lowering context supplies the shared work budget.
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
//! - Sorts and deduplicates destination identities.
//! - Rejects a mismatched strategy instead of selecting one implicitly.
//! - Does not silently choose among competing providers.
//!
//! Provider determinism is part of the provider's own contract.
//!
//! ## TRANSACTIONAL MAPPING
//!
//! The complete destination mapping set is normalized and category-checked
//! before any mapping is recorded.
//!
//! The artifact verifier runs before mapping mutation.
//!
//! The canonical context currently exposes individual mapping insertion.
//! This module prevalidates all category constraints before insertion,
//! preventing category-related partial updates.
//!
//! Providers and verifiers must not publish partially constructed artifacts
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
//! - grammar/spec/quantum.md
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
//! Direct lowering:
//!     src/zuir/lowering/api.rs
//!
//! Domain providers:
//!     src/quantum/optimization/synthesis/decomposition.rs
//!     src/classical/zcir/lowering/
//!     src/hdl/hdlir/
//!     src/ai/
//!     src/hybrid/
//!
//! ## FREEZE CRITERIA
//!
//! - Only ExpansionRequired decisions enter this API.
//! - Strategy identifiers must match exactly.
//! - Domain algorithms remain domain-owned.
//! - The preservation verifier must succeed before mappings are recorded.
//! - Destination mappings are deterministic.
//! - Invalid entity-category mappings are rejected.
//! - Backend and verification errors remain inspectable.
//! - No arbitrary hardware ceilings or unsafe code are introduced.
//! - Parent module declarations and repository integration tests pass.

use core::fmt;
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

/// Public API version for compiler decomposition orchestration.
///
/// This version is independent of the Zamani language version, grammar
/// version, semantic-model version, ZUIR version, domain IR versions,
//! and backend ABI versions.
pub const DECOMPOSITION_API_VERSION: u16 = 1;

// =============================================================================
// RESULT TYPES
// =============================================================================

/// Result alias for compiler decomposition orchestration.
pub type DecompositionResult<T, E, V> =
    Result<T, DecompositionError<E, V>>;

/// Describes the semantic-preservation contract claimed by a provider.
///
/// The identifier refers to a domain-owned specification or verification
/// contract. It is not, by itself, proof that the transformation is correct.
///
/// The independent verification callback must validate the generated
/// artifact under this contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreservationContract {
    /// Stable, qualified identifier of the preservation contract.
    pub identifier: String,
}

impl PreservationContract {
    /// Construct a preservation contract identifier.
    ///
    /// Empty identifiers and identifiers containing NUL characters are
    /// rejected by the decomposition coordinator.
    #[must_use]
    pub fn new(identifier: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

/// Output returned by a domain-owned decomposition provider.
///
/// The provider owns the actual generated artifact and identifies the
/// destination entities associated with the original source entity.
///
/// Destination entities are normalized and validated by the compiler
/// coordinator before their mappings are recorded.
#[derive(Debug)]
pub struct DecompositionOutput<T> {
    /// Artifact produced by the domain-owned decomposition algorithm.
    pub artifact: T,

    /// Destination entities produced or reused by the decomposition.
    pub destinations: Vec<EntityKey>,

    /// Contract governing the semantic preservation of the transformation.
    pub preservation: PreservationContract,
}

impl<T> DecompositionOutput<T> {
    /// Construct a decomposition output.
    #[must_use]
    pub fn new(
        artifact: T,
        destinations: Vec<EntityKey>,
        preservation: PreservationContract,
    ) -> Self {
        Self {
            artifact,
            destinations,
            preservation,
        }
    }
}

// =============================================================================
// DOMAIN PROVIDER
// =============================================================================

/// Domain-owned decomposition provider contract.
///
/// Each implementation owns its actual transformation algorithm.
///
/// Examples include:
///
/// - Quantum logical-operation decomposition.
/// - Classical instruction expansion.
/// - HDL structural expansion.
/// - AI graph decomposition.
/// - Tensor operation decomposition.
/// - Domain-specific hybrid expansion.
///
/// The provider must not silently change program meaning.
///
/// It must not perform physical resource allocation, hardware routing,
/// scheduling, or runtime execution through this interface.
///
/// A provider may retain references to its authoritative IR snapshot,
/// target contract, and domain-specific configuration.
///
/// The strategy identifier must be stable and qualified.
pub trait DecompositionProvider {
    /// Artifact type produced by this provider.
    type Artifact;

    /// Error type returned by the domain implementation.
    type Error: Error + 'static;

    /// Return the stable identifier of the supported strategy.
    fn strategy_id(&self) -> &str;

    /// Decompose one source entity.
    ///
    /// This method is called only after the compiler has confirmed that:
    ///
    /// - The legality status explicitly requires expansion.
    /// - The requested strategy matches this provider.
    /// - The lowering context has accepted the orchestration work charge.
    ///
    /// The provider receives the original source identity and immutable
    /// lowering context.
    ///
    /// It must return a complete artifact or an error. It must not expose
    /// partial output through external mutable state.
    fn decompose(
        &mut self,
        source: &EntityKey,
        context: &LoweringContext,
    ) -> Result<DecompositionOutput<Self::Artifact>, Self::Error>;
}

// =============================================================================
// OUTCOME
// =============================================================================

/// Successful result of one verified decomposition operation.
///
/// This type is constructed only after:
///
/// 1. The legality decision requires expansion.
/// 2. The strategy matches the provider.
/// 3. The provider returns a complete artifact.
/// 4. The preservation contract identifier is valid.
/// 5. All destination entity categories are valid.
/// 6. The independent verification callback succeeds.
/// 7. The source-to-destination mappings are recorded.
///
/// A successful outcome does not prove physical target feasibility,
/// successful linking, or successful execution.
#[derive(Debug)]
pub struct DecompositionOutcome<T> {
    source: EntityKey,
    artifact: T,
    destinations: Vec<EntityKey>,
    preservation: PreservationContract,
    strategy: String,
    diagnostic_code: &'static str,
}

impl<T> DecompositionOutcome<T> {
    /// Return the source entity that was decomposed.
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

    /// Return the normalized destination entity identities.
    #[must_use]
    pub fn destinations(&self) -> &[EntityKey] {
        &self.destinations
    }

    /// Return the preservation contract used for verification.
    #[must_use]
    pub fn preservation(&self) -> &PreservationContract {
        &self.preservation
    }

    /// Return the selected strategy identifier.
    #[must_use]
    pub fn strategy(&self) -> &str {
        &self.strategy
    }

    /// Return the original legality diagnostic code.
    #[must_use]
    pub fn diagnostic_code(&self) -> &'static str {
        self.diagnostic_code
    }

    /// Return whether the decomposition produced at least one destination.
    ///
    /// An empty destination set is permitted for transformations that
    /// legitimately eliminate an entity, provided the independent verifier
    /// confirms the semantic contract.
    #[must_use]
    pub fn has_destinations(&self) -> bool {
        !self.destinations.is_empty()
    }
}

// =============================================================================
// ERRORS
// =============================================================================

/// Errors produced by compiler decomposition orchestration.
#[derive(Debug)]
pub enum DecompositionError<E, V> {
    /// The legality report did not request expansion.
    NotExpansionRequired {
        /// Stable legality diagnostic code.
        diagnostic_code: &'static str,
    },

    /// The provider's strategy identifier is empty or malformed.
    InvalidStrategyIdentifier,

    /// The provider does not implement the requested strategy.
    StrategyMismatch {
        /// Strategy required by the legality report.
        requested: String,

        /// Strategy exposed by the supplied provider.
        provided: String,
    },

    /// The provider returned an invalid preservation-contract identifier.
    InvalidPreservationContract,

    /// A destination entity has a different category from the source.
    DestinationKindMismatch {
        /// Source entity category.
        source: crate::zuir::lowering::context::EntityKind,

        /// Destination entity category.
        destination: crate::zuir::lowering::context::EntityKind,
    },

    /// The domain-owned decomposition algorithm failed.
    Provider(E),

    /// The independent artifact and semantic-preservation verifier failed.
    Verification(V),

    /// The canonical lowering context rejected work or mapping updates.
    Context(LoweringContextError),
}

impl<E: fmt::Display, V: fmt::Display> fmt::Display
    for DecompositionError<E, V>
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotExpansionRequired { diagnostic_code } => {
                write!(
                    formatter,
                    "decomposition requires an ExpansionRequired legality decision; received {diagnostic_code}"
                )
            }

            Self::InvalidStrategyIdentifier => {
                formatter.write_str(
                    "decomposition provider has an empty or malformed strategy identifier",
                )
            }

            Self::StrategyMismatch {
                requested,
                provided,
            } => {
                write!(
                    formatter,
                    "decomposition strategy mismatch: requested '{requested}', provider supplies '{provided}'"
                )
            }

            Self::InvalidPreservationContract => {
                formatter.write_str(
                    "decomposition provider returned an invalid semantic-preservation contract",
                )
            }

            Self::DestinationKindMismatch {
                source,
                destination,
            } => {
                write!(
                    formatter,
                    "decomposition destination category mismatch: source {source:?}, destination {destination:?}"
                )
            }

            Self::Provider(error) => {
                write!(
                    formatter,
                    "domain decomposition provider failed: {error}"
                )
            }

            Self::Verification(error) => {
                write!(
                    formatter,
                    "decomposition artifact verification failed: {error}"
                )
            }

            Self::Context(error) => {
                write!(
                    formatter,
                    "decomposition lowering context failed: {error}"
                )
            }
        }
    }
}

impl<E, V> Error for DecompositionError<E, V>
where
    E: Error + 'static,
    V: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotExpansionRequired { .. } => None,
            Self::InvalidStrategyIdentifier => None,
            Self::StrategyMismatch { .. } => None,
            Self::InvalidPreservationContract => None,
            Self::DestinationKindMismatch { .. } => None,
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

// =============================================================================
// DECOMPOSITION ENTRY POINT
// =============================================================================

/// Execute one explicitly authorized, domain-owned decomposition.
///
/// # Arguments
///
/// `legality`:
///
/// The canonical legality report produced by
/// `crate::zuir::lowering::legality::analyze_legality`.
///
/// `context`:
///
/// The shared lowering context. It owns mappings and operational work
/// accounting.
///
/// `provider`:
///
/// The domain-owned implementation of the requested expansion strategy.
///
/// `verify`:
///
/// An independent verifier for the generated artifact. It must validate
/// structural correctness and the semantic-preservation contract against
/// the original source and legality report.
///
/// # Processing
///
/// The coordinator:
///
/// 1. Requires `ExpansionRequired`.
/// 2. Extracts the required strategy identifier.
/// 3. Checks that the supplied provider implements exactly that strategy.
/// 4. Charges one unit of orchestration work.
/// 5. Invokes the domain-owned decomposition algorithm.
/// 6. Validates the preservation-contract identifier.
/// 7. Sorts and deduplicates destination identities.
/// 8. Checks all destination categories before mapping mutation.
/// 9. Invokes the independent artifact verifier.
/// 10. Records source-to-destination mappings.
/// 11. Returns the verified artifact and its mapping information.
///
/// # Verification callback
///
/// The callback receives:
///
/// - The original legality report.
/// - The source entity identity.
/// - The generated artifact.
/// - The preservation-contract identifier.
///
/// It must reject artifacts that do not preserve the required observable
/// behavior, types, effects, ownership, control dependencies, numerical
/// guarantees, resource requirements, or domain-specific semantics.
///
/// The callback must use the authoritative verifier for the artifact's
/// representation. It must not merely check that the artifact is non-empty.
///
/// # Errors
///
/// Returns an error if:
///
/// - The legality decision does not require expansion.
/// - The provider strategy does not match.
/// - The provider fails.
/// - The preservation contract is invalid.
/// - A destination category is inconsistent.
/// - Artifact verification fails.
/// - The lowering context rejects work or mappings.
///
/// # Important
///
/// This function does not automatically select among multiple providers.
/// Provider discovery and policy-based selection belong to a separate
/// compiler registry or planning subsystem.
///
/// It does not invoke direct lowering. Use the canonical lowering API for
/// a `Legal` decision.
///
/// It does not silently fall back to approximate decomposition.
pub fn decompose_expansion_with<P, Verify, VerifyError>(
    legality: &LegalityReport,
    context: &mut LoweringContext,
    provider: &mut P,
    verify: Verify,
) -> DecompositionResult<DecompositionOutcome<P::Artifact>, P::Error, VerifyError>
where
    P: DecompositionProvider,
    Verify: FnOnce(
        &LegalityReport,
        &EntityKey,
        &P::Artifact,
        &str,
    ) -> Result<(), VerifyError>,
    VerifyError: Error + 'static,
{
    // Expansion is a distinct legality decision. Directly legal,
    // conditional, deferred, and unsupported operations must not enter here.
    let requested_strategy = match &legality.status {
        LegalityStatus::ExpansionRequired { strategy } => strategy,
        _ => {
            return Err(
                DecompositionError::NotExpansionRequired {
                    diagnostic_code: legality.diagnostic_code,
                },
            );
        }
    };

    if !is_valid_identifier(requested_strategy) {
        return Err(DecompositionError::InvalidStrategyIdentifier);
    }

    let provided_strategy = provider.strategy_id();

    if !is_valid_identifier(provided_strategy) {
        return Err(DecompositionError::InvalidStrategyIdentifier);
    }

    if requested_strategy != provided_strategy {
        return Err(DecompositionError::StrategyMismatch {
            requested: requested_strategy.clone(),
            provided: provided_strategy.to_owned(),
        });
    }

    // Charge before invoking the provider. Failed work admission must not
    // execute the domain transformation.
    context
        .charge_work(1)
        .map_err(DecompositionError::Context)?;

    let source = legality.source.clone();

    // The provider owns the actual transformation.
    let mut output = provider
        .decompose(&source, context)
        .map_err(DecompositionError::Provider)?;

    if !is_valid_identifier(&output.preservation.identifier) {
        return Err(DecompositionError::InvalidPreservationContract);
    }

    // Deterministic normalization prevents registration order from depending
    // on provider-specific collection behavior.
    output.destinations.sort();
    output.destinations.dedup();

    // Validate the complete mapping set before changing the context.
    for destination in &output.destinations {
        if source.kind() != destination.kind() {
            return Err(DecompositionError::DestinationKindMismatch {
                source: source.kind().clone(),
                destination: destination.kind().clone(),
            });
        }
    }

    // The independent verifier must accept the artifact before any mappings
    // are recorded. A provider's preservation-contract declaration alone
    // is not proof of semantic equivalence.
    verify(
        legality,
        &source,
        &output.artifact,
        &output.preservation.identifier,
    )
    .map_err(DecompositionError::Verification)?;

    // All category constraints have already been checked. The canonical
    // context's mapping insertion is idempotent for duplicate associations.
    for destination in &output.destinations {
        context
            .record_mapping(source.clone(), destination.clone())
            .map_err(DecompositionError::Context)?;
    }

    Ok(DecompositionOutcome {
        source,
        artifact: output.artifact,
        destinations: output.destinations,
        preservation: output.preservation,
        strategy: requested_strategy.clone(),
        diagnostic_code: legality.diagnostic_code,
    })
}

// =============================================================================
// API INFORMATION
// =============================================================================

/// Return the compiler decomposition API version.
#[must_use]
pub const fn api_version() -> u16 {
    DECOMPOSITION_API_VERSION
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_version_is_stable() {
        assert_eq!(api_version(), DECOMPOSITION_API_VERSION);
        assert_eq!(DECOMPOSITION_API_VERSION, 1);
    }

    #[test]
    fn identifier_validation_rejects_empty_values() {
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("   "));
    }

    #[test]
    fn identifier_validation_rejects_nul() {
        assert!(!is_valid_identifier("domain.strategy\0invalid"));
    }

    #[test]
    fn identifier_validation_accepts_qualified_names() {
        assert!(is_valid_identifier("quantum.synthesis.exact"));
        assert!(is_valid_identifier("classical.lowering.expand"));
        assert!(is_valid_identifier("extension.vendor.operation"));
    }

    #[test]
    fn preservation_contract_keeps_its_identifier() {
        let contract =
            PreservationContract::new("quantum.exact-equivalence.v1");

        assert_eq!(
            contract.identifier,
            "quantum.exact-equivalence.v1"
        );
    }

    #[test]
    fn decomposition_output_preserves_provider_data() {
        let output = DecompositionOutput::new(
            vec!["operation.a", "operation.b"],
            Vec::new(),
            PreservationContract::new("domain.exact.v1"),
        );

        assert_eq!(
            output.artifact,
            vec!["operation.a", "operation.b"]
        );

        assert!(output.destinations.is_empty());

        assert_eq!(
            output.preservation.identifier,
            "domain.exact.v1"
        );
    }
}
