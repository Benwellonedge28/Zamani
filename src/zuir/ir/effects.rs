#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Semantic Effects
//!
//! Canonical path: `src/zuir/ir/effects.rs`
//! Rust edition: 2021
//! Minimum intended Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Define the target-independent representation of operation effects,
//! effect sets, effect summaries, and policies for checking declared effects.
//!
//! Effects describe semantic behavior that matters to program correctness,
//! transformation legality, execution ordering, interoperability, and
//! lowering. Examples include memory access, I/O, allocation, synchronization,
//! communication, nondeterminism, measurement, and extension-defined behavior.
//!
//! The core effect vocabulary is open-ended. This module does not enumerate
//! every possible effect and does not execute effectful operations.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/function.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//!
//! ## OWNS
//!
//! - Effect-name sets and their local invariants.
//! - Effect classifications and declarative effect properties.
//! - Effect summaries and their completeness status.
//! - Policies for checking allowed and forbidden effects.
//! - Adapters between operation effect declarations and effect sets.
//! - Structured effect-specific errors.
//!
//! ## DOES_NOT_OWN
//!
//! - Source-language effect syntax or parsing.
//! - Source name resolution, type inference, or semantic analysis.
//! - The authoritative operation representation.
//! - Dialect registration or executable extension code.
//! - Whole-program effect inference or interprocedural analysis.
//! - Control-flow, dominance, ownership, or alias analysis.
//! - Target capabilities, physical resource allocation, or scheduling.
//! - Runtime enforcement of effects.
//! - Diagnostic rendering or source-map storage.
//!
//! ## INTEGRATION
//!
//! `ZuirOperation` currently stores effects as `BTreeSet<QualifiedName>`.
//! This module deliberately uses that same representation at the boundary;
//! adopting this module does not require changing `operation.rs`.
//!
//! A dialect definition should supply the authoritative properties and
//! interpretation of each effect name. The whole-program verifier and effect
//! analysis should consume these definitions and operation declarations.
//!
//! `ZuirFunction` may declare function-level effects. Interprocedural analysis
//! must compare those declarations with the effects of reachable operations
//! and callees. That analysis is not implemented here.
//!
//! ## SEMANTIC RULES
//!
//! - An effect declaration is not proof that the declaration is complete.
//! - An absent effect is not automatically proof of purity.
//! - Unknown effect names must not be treated as harmless.
//! - An effect name is not a target capability or resource requirement.
//! - Effect sets are unordered semantic collections; their iteration order
//!   is deterministic because they use `BTreeSet`.
//! - Effect ordering and dependencies must be represented by control flow,
//!   memory/effect dependencies, or dialect semantics—not by set iteration.
//!
//! ## SCALABILITY
//!
//! No universal maximum effect count, operation count, device count, qubit
//! count, tensor rank, or hardware capacity is imposed here.
//!
//! Collections grow dynamically. Concrete execution remains bounded by
//! available memory, address space, and implementation resources. This module
//! does not claim that allocation can never fail.
//!
//! ## FREEZE CRITERIA
//!
//! - Effect names reuse `QualifiedName`.
//! - Existing operation effect declarations remain compatible.
//! - Unknown effects are handled by explicit policy.
//! - Incomplete effect knowledge cannot silently become a purity claim.
//! - Validation is deterministic and does not execute extensions.
//! - Tests cover invalid names, duplicates, policy failures, and summaries.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

use crate::zuir::ir::operation::ZuirOperation;
use crate::zuir::schema::types::QualifiedName;

/// Result type for this module.
pub type EffectResult<T> = Result<T, EffectError>;

// =============================================================================
// Effect identity and classification
// =============================================================================

/// Broad classification for an effect definition.
///
/// This classification is descriptive. It is not a closed registry of all
/// effect names. Extensions can define additional names and use
/// `EffectClass::Extension` when no built-in category adequately describes
/// their semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EffectClass {
    /// Reads or observes memory or state.
    Read,

    /// Writes or mutates memory or state.
    Write,

    /// Performs external input or output.
    Io,

    /// Allocates or releases resources.
    Allocation,

    /// Coordinates concurrent or parallel execution.
    Synchronization,

    /// Communicates with another process, actor, device, or endpoint.
    Communication,

    /// Introduces nondeterministic behavior.
    Nondeterminism,

    /// Observes a quantum state or measurement result.
    Measurement,

    /// Performs a state transition whose behavior is externally observable.
    StateTransition,

    /// An effect defined by a dialect or extension.
    Extension,
}

/// Declarative properties attached to a registered effect definition.
///
/// These properties are metadata used by verifiers and transformations.
/// They do not independently prove that an operation's declaration is
/// complete or that its implementation obeys the declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectProperties {
    /// Whether the effect can change externally observable behavior.
    pub observable: bool,

    /// Whether the effect may fail or produce an exceptional outcome.
    pub may_fail: bool,

    /// Whether the effect can introduce nondeterministic behavior.
    pub nondeterministic: bool,

    /// Whether operations carrying the effect may require ordering against
    /// other operations carrying the same effect.
    ///
    /// This is a conservative declaration, not an ordering graph. A
    /// dialect or analysis must determine the actual dependencies.
    pub may_require_ordering: bool,

    /// Whether this effect can interact with externally managed state.
    pub external_state: bool,
}

impl EffectProperties {
    /// Properties suitable for a registered effect that has no observable
    /// behavior and does not interact with external state.
    ///
    /// Use only when the effect definition explicitly guarantees these
    /// properties. This constructor does not imply that an operation with no
    /// effects is pure.
    #[must_use]
    pub const fn non_observable() -> Self {
        Self {
            observable: false,
            may_fail: false,
            nondeterministic: false,
            may_require_ordering: false,
            external_state: false,
        }
    }

    /// Conservative properties for an effect whose behavior is observable
    /// and may interact with external state.
    #[must_use]
    pub const fn externally_observable() -> Self {
        Self {
            observable: true,
            may_fail: true,
            nondeterministic: true,
            may_require_ordering: true,
            external_state: true,
        }
    }
}

/// A validated, registered description of an effect.
///
/// The effect name is its canonical identity. `class` and `properties` are
/// descriptive semantic metadata supplied by the authoritative dialect or
/// effect registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectDefinition {
    name: QualifiedName,
    class: EffectClass,
    properties: EffectProperties,
}

impl EffectDefinition {
    /// Construct an effect definition.
    pub fn new(
        name: QualifiedName,
        class: EffectClass,
        properties: EffectProperties,
    ) -> EffectResult<Self> {
        name.validate()
            .map_err(|error| EffectError::InvalidEffectName {
                name: name.to_string(),
                reason: error.to_string(),
            })?;

        Ok(Self {
            name,
            class,
            properties,
        })
    }

    /// Return the canonical effect name.
    #[must_use]
    pub const fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the broad descriptive classification.
    #[must_use]
    pub const fn class(&self) -> EffectClass {
        self.class
    }

    /// Return the declared semantic properties.
    #[must_use]
    pub const fn properties(&self) -> EffectProperties {
        self.properties
    }

    /// Revalidate the definition after deserialization or interchange.
    pub fn validate(&self) -> EffectResult<()> {
        self.name
            .validate()
            .map_err(|error| EffectError::InvalidEffectName {
                name: self.name.to_string(),
                reason: error.to_string(),
            })
    }
}

// =============================================================================
// Effect errors
// =============================================================================

/// Errors produced by effect construction, validation, and policy checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectError {
    /// An effect name is malformed.
    InvalidEffectName {
        name: String,
        reason: String,
    },

    /// A deserialized effect set contains an invalid name.
    InvalidEffectSet {
        name: String,
        reason: String,
    },

    /// A policy explicitly forbids a declared effect.
    ForbiddenEffect {
        effect: String,
    },

    /// The policy requires an effect to be present.
    MissingRequiredEffect {
        effect: String,
    },

    /// The policy rejects effect names not present in its allow-list.
    EffectNotAllowed {
        effect: String,
    },

    /// An effect was required to be registered, but no definition was
    /// available to the caller's registry or policy.
    UnknownEffect {
        effect: String,
    },

    /// The effect summary is not sufficiently complete for the requested
    /// operation, such as proving that a computation is pure.
    IncompleteEffectInformation {
        context: String,
    },
}

impl fmt::Display for EffectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEffectName { name, reason } => {
                write!(f, "invalid effect name {name}: {reason}")
            }
            Self::InvalidEffectSet { name, reason } => {
                write!(f, "invalid effect set entry {name}: {reason}")
            }
            Self::ForbiddenEffect { effect } => {
                write!(f, "effect {effect} is forbidden by policy")
            }
            Self::MissingRequiredEffect { effect } => {
                write!(f, "required effect {effect} is not declared")
            }
            Self::EffectNotAllowed { effect } => {
                write!(f, "effect {effect} is not allowed by policy")
            }
            Self::UnknownEffect { effect } => {
                write!(f, "effect {effect} has no registered definition")
            }
            Self::IncompleteEffectInformation { context } => {
                write!(f, "effect information is incomplete: {context}")
            }
        }
    }
}

impl std::error::Error for EffectError {}

// =============================================================================
// Effect sets
// =============================================================================

/// A deterministic set of declared or analyzed effect names.
///
/// A set is a collection of names, not a claim that all effects are known.
/// Use `EffectSummary` when the completeness of the information matters.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EffectSet {
    names: BTreeSet<QualifiedName>,
}

impl EffectSet {
    /// Construct an empty effect set.
    ///
    /// An empty set means that no effects are recorded in this set. It does
    /// not, by itself, prove that the represented computation is pure.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an effect set from effect names.
    ///
    /// Duplicate names are naturally coalesced. Malformed names are rejected.
    pub fn from_names<I>(names: I) -> EffectResult<Self>
    where
        I: IntoIterator<Item = QualifiedName>,
    {
        let mut result = Self::new();

        for name in names {
            result.insert(name)?;
        }

        Ok(result)
    }

    /// Construct an effect set from an operation's current declarations.
    ///
    /// The operation remains the owner of its stored declaration. This method
    /// validates and copies those names into an analysis-friendly wrapper.
    pub fn from_operation(operation: &ZuirOperation) -> EffectResult<Self> {
        Self::from_names(operation.effects().iter().cloned())
    }

    /// Return the number of distinct effect names.
    #[must_use]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Return whether no effect names are recorded.
    ///
    /// This is a structural property only, not a purity proof.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Iterate through effect names in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &QualifiedName> {
        self.names.iter()
    }

    /// Check whether the set contains an effect name.
    #[must_use]
    pub fn contains(&self, name: &QualifiedName) -> bool {
        self.names.contains(name)
    }

    /// Insert a validated effect name.
    ///
    /// Returns `true` if the set changed.
    pub fn insert(&mut self, name: QualifiedName) -> EffectResult<bool> {
        name.validate()
            .map_err(|error| EffectError::InvalidEffectName {
                name: name.to_string(),
                reason: error.to_string(),
            })?;

        Ok(self.names.insert(name))
    }

    /// Remove an effect name.
    ///
    /// Returns `true` if the set changed.
    pub fn remove(&mut self, name: &QualifiedName) -> bool {
        self.names.remove(name)
    }

    /// Check whether every name in `other` is present in this set.
    #[must_use]
    pub fn contains_all(&self, other: &Self) -> bool {
        other.names.is_subset(&self.names)
    }

    /// Check whether this set is a subset of `other`.
    #[must_use]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.names.is_subset(&other.names)
    }

    /// Add all names from another effect set.
    ///
    /// This operation represents set union; it does not encode execution
    /// ordering or prove that either input set is complete.
    pub fn extend(&mut self, other: &Self) {
        self.names.extend(other.names.iter().cloned());
    }

    /// Return a new set containing the union of both inputs.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.extend(other);
        result
    }

    /// Revalidate all names, including after deserialization.
    pub fn validate(&self) -> EffectResult<()> {
        for name in &self.names {
            name.validate()
                .map_err(|error| EffectError::InvalidEffectSet {
                    name: name.to_string(),
                    reason: error.to_string(),
                })?;
        }

        Ok(())
    }

    /// Borrow the underlying deterministic collection.
    ///
    /// This accessor is provided for adapters that must interoperate with the
    /// existing `ZuirOperation::effects()` API.
    #[must_use]
    pub const fn as_names(&self) -> &BTreeSet<QualifiedName> {
        &self.names
    }

    /// Consume the set and return its underlying names.
    #[must_use]
    pub fn into_names(self) -> BTreeSet<QualifiedName> {
        self.names
    }
}

impl IntoIterator for EffectSet {
    type Item = QualifiedName;
    type IntoIter = std::collections::btree_set::IntoIter<QualifiedName>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

// =============================================================================
// Effect knowledge and summaries
// =============================================================================

/// Describes how complete an effect summary is.
///
/// Completeness is explicit so downstream optimizations cannot silently
/// interpret an incomplete summary as proof of purity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectCompleteness {
    /// Effects have not been analyzed or their completeness is unknown.
    Unknown,

    /// Some effects are known, but additional effects may exist.
    Partial,

    /// The producer claims the set is complete under a specified semantic
    /// contract. A verifier must still establish that the claim is valid.
    Complete,
}

/// An effect summary with an explicit completeness state.
///
/// `effects` is the known set. `completeness` explains what can be inferred
/// from that set. A `Complete` summary is a claim, not a cryptographic proof
/// or an automatic result of constructing this value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectSummary {
    effects: EffectSet,
    completeness: EffectCompleteness,
}

impl EffectSummary {
    /// Construct a summary from a set and completeness state.
    #[must_use]
    pub const fn new(
        effects: EffectSet,
        completeness: EffectCompleteness,
    ) -> Self {
        Self {
            effects,
            completeness,
        }
    }

    /// Construct a summary with unknown completeness.
    #[must_use]
    pub fn unknown() -> Self {
        Self::new(EffectSet::new(), EffectCompleteness::Unknown)
    }

    /// Construct a complete summary from a known set.
    ///
    /// Use only when the caller's analysis or trusted semantic contract
    /// justifies completeness.
    #[must_use]
    pub fn complete(effects: EffectSet) -> Self {
        Self::new(effects, EffectCompleteness::Complete)
    }

    /// Borrow the known effect set.
    #[must_use]
    pub const fn effects(&self) -> &EffectSet {
        &self.effects
    }

    /// Return the completeness state.
    #[must_use]
    pub const fn completeness(&self) -> EffectCompleteness {
        self.completeness
    }

    /// Return whether the summary is explicitly marked complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.completeness, EffectCompleteness::Complete)
    }

    /// Return whether the summary proves purity under the caller's trusted
    /// completeness contract.
    ///
    /// An empty but incomplete summary is not considered pure.
    pub fn is_proven_pure(&self) -> EffectResult<bool> {
        if !self.is_complete() {
            return Err(EffectError::IncompleteEffectInformation {
                context: "purity cannot be established from an incomplete effect summary"
                    .to_owned(),
            });
        }

        Ok(self.effects.is_empty())
    }

    /// Revalidate the summary's stored effect names.
    pub fn validate(&self) -> EffectResult<()> {
        self.effects.validate()
    }
}

// =============================================================================
// Effect policy
// =============================================================================

/// Policy for checking effect declarations.
///
/// Policy sets contain qualified effect names. An allow-list, when present,
/// is restrictive: any declared effect absent from that list is rejected.
/// A forbidden effect is rejected even if it also appears in the allow-list.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectPolicy {
    allowed: Option<EffectSet>,
    forbidden: EffectSet,
    required: EffectSet,
    require_registered_definitions: bool,
}

impl EffectPolicy {
    /// Construct a permissive policy.
    ///
    /// This policy does not reject unknown names merely because they are
    /// unknown. It still validates name structure and rejects explicitly
    /// forbidden effects. A production verifier may use a stricter policy.
    #[must_use]
    pub fn permissive() -> Self {
        Self::default()
    }

    /// Construct a policy that allows only the supplied effect names.
    #[must_use]
    pub fn allow_only(allowed: EffectSet) -> Self {
        Self {
            allowed: Some(allowed),
            ..Self::default()
        }
    }

    /// Borrow the optional allow-list.
    #[must_use]
    pub const fn allowed(&self) -> Option<&EffectSet> {
        self.allowed.as_ref()
    }

    /// Borrow the forbidden effect set.
    #[must_use]
    pub const fn forbidden(&self) -> &EffectSet {
        &self.forbidden
    }

    /// Borrow the required effect set.
    #[must_use]
    pub const fn required(&self) -> &EffectSet {
        &self.required
    }

    /// Set the allow-list.
    pub fn set_allowed(&mut self, allowed: Option<EffectSet>) {
        self.allowed = allowed;
    }

    /// Add an effect that must not be declared.
    pub fn forbid(&mut self, effect: QualifiedName) -> EffectResult<bool> {
        self.forbidden.insert(effect)
    }

    /// Add an effect that must be declared.
    pub fn require(&mut self, effect: QualifiedName) -> EffectResult<bool> {
        self.required.insert(effect)
    }

    /// Require the caller to check that each declared effect has a registered
    /// definition before accepting it.
    ///
    /// This module does not own the registry. Use `validate_with_registry`
    /// when a registry-backed definition lookup is available.
    pub fn require_registered_definitions(&mut self, required: bool) {
        self.require_registered_definitions = required;
    }

    /// Return whether registered definitions are required.
    #[must_use]
    pub const fn needs_registered_definitions(&self) -> bool {
        self.require_registered_definitions
    }

    /// Validate an effect set against the allow, deny, and require rules.
    ///
    /// This method can check only the names supplied to it. It cannot infer
    /// missing effects or verify that a declaration matches an operation's
    /// actual implementation.
    pub fn validate(&self, effects: &EffectSet) -> EffectResult<()> {
        effects.validate()?;
        self.allowed.as_ref().map(EffectSet::validate).transpose()?;
        self.forbidden.validate()?;
        self.required.validate()?;

        for effect in effects.iter() {
            if self.forbidden.contains(effect) {
                return Err(EffectError::ForbiddenEffect {
                    effect: effect.to_string(),
                });
            }

            if let Some(allowed) = &self.allowed {
                if !allowed.contains(effect) {
                    return Err(EffectError::EffectNotAllowed {
                        effect: effect.to_string(),
                    });
                }
            }
        }

        for required in self.required.iter() {
            if !effects.contains(required) {
                return Err(EffectError::MissingRequiredEffect {
                    effect: required.to_string(),
                });
            }
        }

        if self.require_registered_definitions {
            return Err(EffectError::IncompleteEffectInformation {
                context: "this policy requires registry-backed validation; \
                          call validate_with_registry with an authoritative \
                          definition lookup".to_owned(),
            });
        }

        Ok(())
    }

    /// Validate an effect set and require every effect to have a definition
    /// supplied by the caller.
    ///
    /// `lookup` should query the authoritative dialect/effect registry.
    /// The closure is intentionally passed in rather than storing executable
    /// callbacks in a serializable policy.
    pub fn validate_with_registry<F>(
        &self,
        effects: &EffectSet,
        mut lookup: F,
    ) -> EffectResult<()>
    where
        F: FnMut(&QualifiedName) -> Option<EffectDefinition>,
    {
        effects.validate()?;
        self.allowed.as_ref().map(EffectSet::validate).transpose()?;
        self.forbidden.validate()?;
        self.required.validate()?;

        for effect in effects.iter() {
            if self.forbidden.contains(effect) {
                return Err(EffectError::ForbiddenEffect {
                    effect: effect.to_string(),
                });
            }

            if let Some(allowed) = &self.allowed {
                if !allowed.contains(effect) {
                    return Err(EffectError::EffectNotAllowed {
                        effect: effect.to_string(),
                    });
                }
            }

            let definition =
                lookup(effect).ok_or_else(|| EffectError::UnknownEffect {
                    effect: effect.to_string(),
                })?;

            definition.validate()?;

            if definition.name() != effect {
                return Err(EffectError::UnknownEffect {
                    effect: effect.to_string(),
                });
            }
        }

        for required in self.required.iter() {
            if !effects.contains(required) {
                return Err(EffectError::MissingRequiredEffect {
                    effect: required.to_string(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Existing operation integration
// =============================================================================

/// Validate the structural correctness of the effect names declared by an
/// operation.
///
/// This does not verify the operation's actual semantics, dialect contract,
/// or whole-program effect completeness.
pub fn validate_operation_effects(
    operation: &ZuirOperation,
) -> EffectResult<()> {
    EffectSet::from_operation(operation).map(|_| ())
}

/// Copy an operation's declared effects into an explicitly incomplete summary.
///
/// A declaration is not a whole-program effect analysis. The returned summary
/// therefore does not claim completeness or prove purity.
pub fn declared_operation_effects(
    operation: &ZuirOperation,
) -> EffectResult<EffectSummary> {
    let effects = EffectSet::from_operation(operation)?;
    Ok(EffectSummary::new(effects, EffectCompleteness::Partial))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn name(parts: &[&str]) -> QualifiedName {
        QualifiedName::new(
            parts
                .iter()
                .map(|part| (*part).to_owned())
                .collect::<Vec<_>>(),
        )
        .expect("test qualified name should be valid")
    }

    fn effect(parts: &[&str]) -> QualifiedName {
        name(parts)
    }

    #[test]
    fn effect_set_deduplicates_names_and_is_deterministic() {
        let first = effect(&["core", "memory_read"]);
        let second = effect(&["core", "io"]);

        let set = EffectSet::from_names([
            first.clone(),
            second.clone(),
            first.clone(),
        ])
        .expect("valid names should be accepted");

        assert_eq!(set.len(), 2);
        assert!(set.contains(&first));
        assert!(set.contains(&second));

        let ordered: Vec<String> =
            set.iter().map(ToString::to_string).collect();

        let mut sorted = ordered.clone();
        sorted.sort();

        assert_eq!(ordered, sorted);
    }

    #[test]
    fn empty_effect_set_does_not_automatically_prove_purity() {
        let summary = EffectSummary::unknown();

        assert!(summary.is_proven_pure().is_err());
    }

    #[test]
    fn complete_empty_summary_can_prove_purity() {
        let summary = EffectSummary::complete(EffectSet::new());

        assert_eq!(summary.is_proven_pure(), Ok(true));
    }

    #[test]
    fn complete_nonempty_summary_is_not_pure() {
        let mut effects = EffectSet::new();
        effects
            .insert(effect(&["core", "io"]))
            .expect("valid effect name");

        let summary = EffectSummary::complete(effects);

        assert_eq!(summary.is_proven_pure(), Ok(false));
    }

    #[test]
    fn allow_list_rejects_unlisted_effects() {
        let allowed = EffectSet::from_names([
            effect(&["core", "memory_read"]),
        ])
        .expect("valid effect names");

        let policy = EffectPolicy::allow_only(allowed);

        let declared = EffectSet::from_names([
            effect(&["core", "io"]),
        ])
        .expect("valid effect names");

        assert!(matches!(
            policy.validate(&declared),
            Err(EffectError::EffectNotAllowed { .. })
        ));
    }

    #[test]
    fn forbidden_effect_is_rejected() {
        let mut policy = EffectPolicy::permissive();
        let forbidden = effect(&["core", "io"]);

        policy
            .forbid(forbidden.clone())
            .expect("valid effect name");

        let declared =
            EffectSet::from_names([forbidden]).expect("valid effect name");

        assert!(matches!(
            policy.validate(&declared),
            Err(EffectError::ForbiddenEffect { .. })
        ));
    }

    #[test]
    fn required_effect_must_be_present() {
        let mut policy = EffectPolicy::permissive();

        policy
            .require(effect(&["core", "measurement"]))
            .expect("valid effect name");

        assert!(matches!(
            policy.validate(&EffectSet::new()),
            Err(EffectError::MissingRequiredEffect { .. })
        ));
    }

    #[test]
    fn registry_validation_rejects_unknown_effects() {
        let declared = EffectSet::from_names([
            effect(&["example", "custom_effect"]),
        ])
        .expect("valid effect name");

        let policy = EffectPolicy::permissive();

        let result = policy.validate_with_registry(&declared, |_| None);

        assert!(matches!(
            result,
            Err(EffectError::UnknownEffect { .. })
        ));
    }

    #[test]
    fn registered_extension_effect_is_accepted() {
        let custom = effect(&["example", "custom_effect"]);

        let definition = EffectDefinition::new(
            custom.clone(),
            EffectClass::Extension,
            EffectProperties::externally_observable(),
        )
        .expect("valid definition");

        let declared =
            EffectSet::from_names([custom]).expect("valid effect name");

        let policy = EffectPolicy::permissive();

        assert!(policy
            .validate_with_registry(&declared, |name| {
                (name == definition.name()).then(|| definition.clone())
            })
            .is_ok());
    }

    #[test]
    fn union_combines_sets_without_duplicate_effects() {
        let left = EffectSet::from_names([
            effect(&["core", "memory_read"]),
        ])
        .expect("valid effect name");

        let right = EffectSet::from_names([
            effect(&["core", "memory_read"]),
            effect(&["core", "io"]),
        ])
        .expect("valid effect names");

        let combined = left.union(&right);

        assert_eq!(combined.len(), 2);
        assert!(combined.contains_all(&left));
        assert!(combined.contains_all(&right));
    }

    #[test]
    fn summary_validation_checks_stored_names() {
        let summary = EffectSummary::complete(EffectSet::new());

        assert!(summary.validate().is_ok());
    }
}