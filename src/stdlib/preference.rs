
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Preference Contracts
//!
//! Path: `src/stdlib/preference.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides target-independent, extensible preference identities,
//! preference declarations, provenance, and ordered preference collections.
//!
//! A preference describes a desirable property among otherwise valid
//! realizations. It is non-mandatory.
//!
//! This module deliberately does not define a second expression language.
//! The preference payload is generic so that the canonical frontend semantic
//! model, resource model, or another domain authority can supply its own
//! typed representation.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Open-world preference identities.
//! - Preference declarations.
//! - Optional explanatory rationale.
//! - Opaque preference provenance.
//! - Ordered preference collections.
//! - Duplicate declaration reporting.
//! - Deterministic presentation.
//!
//! This module does not own:
//!
//! - Source grammar or parsing.
//! - AST nodes or source spans.
//! - Semantic name resolution.
//! - Resource quantities, units, or expressions.
//! - Capability definitions or discovery.
//! - Mandatory requirements.
//! - Mandatory constraints.
//! - Non-binding optimization hints.
//! - Policy authorization.
//! - Preference conflict resolution.
//! - Preference scoring or ranking.
//! - Target selection.
//! - Hardware discovery or allocation.
//! - Provider selection.
//! - Scheduling, routing, or execution.
//! - Compiler or IR representations.
//!
//! ## Architectural authorities
//!
//! The following repository subsystems remain authoritative:
//!
//! - `grammar/resources/preferences.g4`:
//!   resource-preference syntax.
//! - `grammar/policies/preferences.g4`:
//!   policy-preference syntax.
//! - `src/frontend/semantic/`:
//!   source-level semantic interpretation.
//! - `src/frontend/semantic/resources/`:
//!   canonical resource semantics.
//! - `src/stdlib/capability.rs`:
//!   capability descriptions and availability.
//! - `src/stdlib/requirement.rs`:
//!   mandatory requirement identities.
//! - `src/zuir/ir/resources.rs`:
//!   resource-intent representation.
//! - `src/compiler/planning/`:
//!   planning and realization decisions.
//! - `src/runtime/` and HAL/providers:
//!   actual execution and physical resource interaction.
//!
//! ## Fundamental distinctions
//!
//! Requirement:
//!   A mandatory condition that a valid realization must satisfy.
//!
//! Capability:
//!   An ability an environment or implementation can provide.
//!
//! Preference:
//!   A desirable property among otherwise valid realizations.
//!
//! Constraint:
//!   A mandatory restriction on an otherwise valid realization.
//!
//! Hint:
//!   Non-binding implementation or optimization guidance.
//!
//! Feasibility:
//!   An evidence-based assessment of whether requirements and constraints
//!   can be satisfied.
//!
//! These concepts must never be silently converted into one another.
//!
//! In particular:
//!
//! - A preference must never become a mandatory requirement.
//! - A preference must never become a mandatory constraint.
//! - A preference must never imply capability availability.
//! - A preference must never authorize an operation.
//! - A preference must never override a mandatory requirement.
//! - A preference must never be treated as proof of feasibility.
//!
//! ## POCO-REAF
//!
//! Preference identities are open-world.
//!
//! New computational domains, preference properties, extensions, and future
//! execution technologies can introduce new names without modifying this
//! module.
//!
//! A preference can describe desirable characteristics for:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware realization.
//! - Hybrid computation.
//! - Distributed computation.
//! - AI and tensor computation.
//! - Networking.
//! - Scientific computation.
//! - Embedded systems.
//! - Future computational domains.
//!
//! This module does not select any particular machine, device, provider,
//! architecture, accelerator, node, or execution location.
//!
//! ## Scalability
//!
//! No arbitrary language-level maximum is imposed on:
//!
//! - Preference count.
//! - Namespace depth.
//! - Preference-name length.
//! - Collection size.
//! - Number of domains.
//! - Number of preference properties.
//!
//! Actual memory and processing requirements remain finite and depend on
//! available resources and explicit operational budgets.
//!
//! This module does not promise literal infinite storage or computation.
//!
//! ## Determinism
//!
//! Preference identity is determined by its namespace and name.
//!
//! Preference collections preserve insertion order and duplicate declarations.
//!
//! Canonicalized views use deterministic key ordering without changing the
//! original collection.
//!
//! ## Payload contract
//!
//! `Preference<T>` stores a caller-supplied typed payload.
//!
//! The payload may be a canonical semantic expression, resource-preference
//! model, domain-specific value, or another explicitly defined representation.
//!
//! This module does not parse, interpret, evaluate, score, normalize, or
//! compare payloads.
//!
//! ## Integration contract
//!
//! `src/stdlib/mod.rs` must expose this module using:
//!
//!     pub mod preference;
//!
//! Frontend and semantic integrations must preserve the canonical semantic
//! representation in the generic payload.
//!
//! Compiler and ZUIR integrations must use explicit adapters.
//!
//! No semantic, compiler, IR, runtime, or hardware subsystem should depend
//! on this module merely to obtain its own canonical preference semantics.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

// ============================================================================
// PREFERENCE IDENTITY
// ============================================================================

/// A stable, open-world identity for a preference.
///
/// A preference identity consists of an optional namespace and a name.
///
/// Namespaces distinguish preference domains and extensions. Names identify
/// individual preference properties within those namespaces.
///
/// Examples:
///
/// - `latency`
/// - `throughput`
/// - `energy`
/// - `classical::locality`
/// - `quantum::fidelity`
/// - `hdl::timing_margin`
/// - `hybrid::communication_cost`
/// - `distributed::data_locality`
/// - `ai::inference_latency`
/// - `future::architecture::execution_characteristic`
///
/// Identities are preserved exactly. This type does not perform case folding,
/// alias resolution, semantic name resolution, or grammar parsing.
///
/// The type does not impose a fixed list of preference names or domains.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PreferenceKey {
    namespace: Option<String>,
    name: String,
}

impl PreferenceKey {
    /// Constructs a namespaced preference identity.
    ///
    /// The namespace may contain multiple `::`-separated components.
    ///
    /// The name must be a single non-empty component.
    ///
    /// No arbitrary maximum length or namespace depth is imposed.
    ///
    /// # Errors
    ///
    /// Returns [`PreferenceError`] if either component is malformed.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, PreferenceError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_namespace(&namespace)?;
        validate_component(
            &name,
            PreferenceComponent::Name,
        )?;

        if name.contains("::") {
            return Err(PreferenceError::InvalidName {
                name,
                reason: "preference name must be one namespace component".to_owned(),
            });
        }

        Ok(Self {
            namespace: Some(namespace),
            name,
        })
    }

    /// Constructs an unqualified preference identity.
    ///
    /// This supports standardized or contextually scoped preference names
    /// such as `latency`, `energy`, or `throughput`.
    ///
    /// Whether an unqualified name is valid in a particular source scope is
    /// determined by semantic analysis, not by this constructor.
    ///
    /// # Errors
    ///
    /// Returns [`PreferenceError`] if the name is malformed.
    pub fn unqualified(
        name: impl Into<String>,
    ) -> Result<Self, PreferenceError> {
        let name = name.into();

        validate_component(
            &name,
            PreferenceComponent::Name,
        )?;

        if name.contains("::") {
            return Err(PreferenceError::InvalidName {
                name,
                reason: "unqualified preference name must not contain a namespace separator".to_owned(),
            });
        }

        Ok(Self {
            namespace: None,
            name,
        })
    }

    /// Parses a qualified preference identity.
    ///
    /// The final `::` separates the namespace from the name.
    ///
    /// Examples:
    ///
    /// - `latency` becomes an unqualified identity.
    /// - `quantum::fidelity` becomes namespace `quantum`, name `fidelity`.
    /// - `future::architecture::metric` becomes namespace
    ///   `future::architecture`, name `metric`.
    ///
    /// This parses only the identity boundary. It does not implement the
    /// Zamani grammar or perform semantic name resolution.
    ///
    /// # Errors
    ///
    /// Returns [`PreferenceError`] if the supplied identity is malformed.
    pub fn from_qualified_name(
        value: impl AsRef<str>,
    ) -> Result<Self, PreferenceError> {
        let value = value.as_ref();

        match value.rsplit_once("::") {
            Some((namespace, name)) => {
                Self::new(namespace, name)
            }
            None => Self::unqualified(value),
        }
    }

    /// Returns the optional namespace exactly as stored.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Returns the preference name exactly as stored.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether this identity is namespaced.
    #[must_use]
    pub fn is_namespaced(&self) -> bool {
        self.namespace.is_some()
    }

    /// Returns the qualified textual identity.
    ///
    /// The returned string is intended for diagnostics, display, and
    /// interchange. Identity comparisons should use [`PreferenceKey`].
    #[must_use]
    pub fn qualified_name(&self) -> String {
        match &self.namespace {
            Some(namespace) => {
                format!("{namespace}::{}", self.name)
            }
            None => self.name.clone(),
        }
    }

    /// Checks an identity without allocating a temporary key.
    #[must_use]
    pub fn matches(
        &self,
        namespace: Option<&str>,
        name: &str,
    ) -> bool {
        self.namespace() == namespace && self.name == name
    }
}

impl fmt::Display for PreferenceKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match &self.namespace {
            Some(namespace) => {
                write!(formatter, "{namespace}::{}", self.name)
            }
            None => formatter.write_str(&self.name),
        }
    }
}

impl TryFrom<&str> for PreferenceKey {
    type Error = PreferenceError;

    fn try_from(
        value: &str,
    ) -> Result<Self, Self::Error> {
        Self::from_qualified_name(value)
    }
}

impl TryFrom<String> for PreferenceKey {
    type Error = PreferenceError;

    fn try_from(
        value: String,
    ) -> Result<Self, Self::Error> {
        Self::from_qualified_name(value)
    }
}

// ============================================================================
// PREFERENCE PROVENANCE
// ============================================================================

/// Provenance associated with a preference declaration.
///
/// Provenance fields are deliberately opaque. The frontend can preserve its
/// actual source-file identifiers, AST node identities, source spans, or
/// semantic producer information without forcing this foundational module
/// to import frontend-specific types.
///
/// Provenance does not change preference identity or payload semantics.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PreferenceOrigin {
    source_id: Option<String>,
    node_id: Option<String>,
    start_byte: Option<u64>,
    end_byte: Option<u64>,
    producer: Option<String>,
}

impl PreferenceOrigin {
    /// Creates an empty provenance record.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_id: None,
            node_id: None,
            start_byte: None,
            end_byte: None,
            producer: None,
        }
    }

    /// Sets an opaque source identifier.
    #[must_use]
    pub fn with_source_id(
        mut self,
        source_id: impl Into<String>,
    ) -> Self {
        self.source_id = Some(source_id.into());
        self
    }

    /// Sets an opaque AST or semantic node identifier.
    #[must_use]
    pub fn with_node_id(
        mut self,
        node_id: impl Into<String>,
    ) -> Self {
        self.node_id = Some(node_id.into());
        self
    }

    /// Sets the producer of this preference declaration.
    ///
    /// Examples include `frontend.semantic`, `stdlib`, or a domain adapter.
    #[must_use]
    pub fn with_producer(
        mut self,
        producer: impl Into<String>,
    ) -> Self {
        self.producer = Some(producer.into());
        self
    }

    /// Sets an inclusive start and exclusive end byte range.
    ///
    /// # Errors
    ///
    /// Returns [`PreferenceError::InvalidSourceRange`] if the end precedes
    /// the start.
    pub fn with_byte_range(
        mut self,
        start: u64,
        end: u64,
    ) -> Result<Self, PreferenceError> {
        if end < start {
            return Err(PreferenceError::InvalidSourceRange {
                start,
                end,
            });
        }

        self.start_byte = Some(start);
        self.end_byte = Some(end);

        Ok(self)
    }

    /// Returns the source identifier, if present.
    #[must_use]
    pub fn source_id(&self) -> Option<&str> {
        self.source_id.as_deref()
    }

    /// Returns the AST or semantic node identifier, if present.
    #[must_use]
    pub fn node_id(&self) -> Option<&str> {
        self.node_id.as_deref()
    }

    /// Returns the inclusive start byte offset, if present.
    #[must_use]
    pub const fn start_byte(&self) -> Option<u64> {
        self.start_byte
    }

    /// Returns the exclusive end byte offset, if present.
    #[must_use]
    pub const fn end_byte(&self) -> Option<u64> {
        self.end_byte
    }

    /// Returns the producer identifier, if present.
    #[must_use]
    pub fn producer(&self) -> Option<&str> {
        self.producer.as_deref()
    }

    /// Returns the byte range, if both endpoints are present.
    #[must_use]
    pub const fn byte_range(&self) -> Option<(u64, u64)> {
        match (self.start_byte, self.end_byte) {
            (Some(start), Some(end)) => Some((start, end)),
            _ => None,
        }
    }
}

// ============================================================================
// PREFERENCE DECLARATION
// ============================================================================

/// A non-mandatory preference declaration with a caller-defined payload.
///
/// `T` is deliberately generic.
///
/// The canonical frontend or semantic resource model supplies the payload.
/// This module does not introduce a competing resource-expression type,
/// numeric representation, AST, policy expression, or domain-specific value.
///
/// A preference is advisory. It does not become mandatory merely because it
/// is stored in this type.
///
/// # Examples
///
/// A simple application-defined payload:
///
/// ```
/// use zamani_compiler::stdlib::preference::{
///     Preference,
///     PreferenceKey,
/// };
///
/// let preference = Preference::new(
///     PreferenceKey::unqualified("latency").unwrap(),
///     "minimize".to_owned(),
/// );
///
/// assert_eq!(preference.key().name(), "latency");
/// assert_eq!(preference.value(), "minimize");
/// ```
///
/// A frontend adapter can instead use its canonical semantic expression
/// type as `T`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Preference<T> {
    key: PreferenceKey,
    value: T,
    rationale: Option<String>,
    origin: PreferenceOrigin,
}

impl<T> Preference<T> {
    /// Creates a preference declaration.
    ///
    /// The payload is retained without interpretation or modification.
    #[must_use]
    pub fn new(
        key: PreferenceKey,
        value: T,
    ) -> Self {
        Self {
            key,
            value,
            rationale: None,
            origin: PreferenceOrigin::new(),
        }
    }

    /// Constructs a preference from a qualified identity and payload.
    ///
    /// # Errors
    ///
    /// Returns [`PreferenceError`] if the identity is malformed.
    pub fn from_parts(
        namespace: Option<impl Into<String>>,
        name: impl Into<String>,
        value: T,
    ) -> Result<Self, PreferenceError> {
        let key = match namespace {
            Some(namespace) => {
                PreferenceKey::new(namespace, name)?
            }
            None => {
                PreferenceKey::unqualified(name)?
            }
        };

        Ok(Self::new(key, value))
    }

    /// Returns the preference identity.
    #[must_use]
    pub fn key(&self) -> &PreferenceKey {
        &self.key
    }

    /// Returns the typed preference payload.
    ///
    /// The payload is not interpreted by this module.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns the optional explanatory rationale.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }

    /// Returns the provenance record.
    #[must_use]
    pub fn origin(&self) -> &PreferenceOrigin {
        &self.origin
    }

    /// Adds a non-empty human-readable rationale.
    ///
    /// Rationale is explanatory metadata. It must not change the meaning
    /// of the preference or turn it into a mandatory condition.
    ///
    /// # Errors
    ///
    /// Returns [`PreferenceError::EmptyRationale`] for empty or
    /// whitespace-only text.
    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Result<Self, PreferenceError> {
        let rationale = rationale.into();

        if rationale.trim().is_empty() {
            return Err(PreferenceError::EmptyRationale);
        }

        self.rationale = Some(rationale);

        Ok(self)
    }

    /// Attaches provenance to this preference.
    #[must_use]
    pub fn with_origin(
        mut self,
        origin: PreferenceOrigin,
    ) -> Self {
        self.origin = origin;
        self
    }

    /// Replaces the payload while preserving identity, rationale, and
    /// provenance.
    ///
    /// This is an explicit transformation. The caller is responsible for
    /// ensuring that the new payload represents the same preference
    /// according to the owning semantic model.
    #[must_use]
    pub fn map_value<U>(
        self,
        map: impl FnOnce(T) -> U,
    ) -> Preference<U> {
        Preference {
            key: self.key,
            value: map(self.value),
            rationale: self.rationale,
            origin: self.origin,
        }
    }

    /// Consumes the preference and returns its payload.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }

    /// Consumes the preference and returns its identity and payload.
    #[must_use]
    pub fn into_parts(self) -> (PreferenceKey, T) {
        (self.key, self.value)
    }
}

impl<T: fmt::Display> fmt::Display for Preference<T> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{}: {}", self.key, self.value)
    }
}

// ============================================================================
// PREFERENCE COLLECTION
// ============================================================================

/// An ordered collection of non-mandatory preference declarations.
///
/// The collection intentionally preserves:
///
/// - Insertion order.
/// - Duplicate preference identities.
/// - Different payloads for the same identity.
/// - Different provenance records.
/// - Potentially conflicting preferences.
///
/// This type does not:
///
/// - Deduplicate preferences.
/// - Resolve conflicting preferences.
/// - Assign weights or scores.
/// - Rank preferences.
/// - Evaluate preference expressions.
/// - Convert preferences into requirements or constraints.
/// - Select providers or hardware.
///
/// Duplicate and conflicting declarations remain available to semantic
/// analysis and downstream planning.
///
/// There is no fixed collection capacity. Growth depends on available
/// memory and any explicit operational budget outside this type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreferenceSet<T> {
    preferences: Vec<Preference<T>>,
}

impl<T> Default for PreferenceSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PreferenceSet<T> {
    /// Creates an empty preference collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            preferences: Vec::new(),
        }
    }

    /// Creates a collection from an iterator.
    ///
    /// Iterator order and duplicate entries are preserved.
    #[must_use]
    pub fn from_preferences(
        preferences: impl IntoIterator<Item = Preference<T>>,
    ) -> Self {
        Self {
            preferences: preferences.into_iter().collect(),
        }
    }

    /// Adds a preference without deduplication or interpretation.
    pub fn push(
        &mut self,
        preference: Preference<T>,
    ) {
        self.preferences.push(preference);
    }

    /// Adds a preference and returns the updated collection.
    #[must_use]
    pub fn with(
        mut self,
        preference: Preference<T>,
    ) -> Self {
        self.push(preference);
        self
    }

    /// Appends another collection, preserving order and duplicates.
    pub fn extend(
        &mut self,
        other: Self,
    ) {
        self.preferences.extend(other.preferences);
    }

    /// Appends preferences from an iterator.
    ///
    /// Existing entries are not deduplicated or reordered.
    pub fn extend_from(
        &mut self,
        preferences: impl IntoIterator<Item = Preference<T>>,
    ) {
        self.preferences.extend(preferences);
    }

    /// Returns the number of stored preference declarations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.preferences.len()
    }

    /// Returns whether the collection contains no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.preferences.is_empty()
    }

    /// Returns preferences in insertion order.
    pub fn iter(
        &self,
    ) -> core::slice::Iter<'_, Preference<T>> {
        self.preferences.iter()
    }

    /// Returns a preference by insertion-order index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Preference<T>> {
        self.preferences.get(index)
    }

    /// Returns preferences belonging to a namespace.
    ///
    /// The returned iterator preserves insertion order.
    pub fn for_namespace<'a>(
        &'a self,
        namespace: &'a str,
    ) -> impl Iterator<Item = &'a Preference<T>> + 'a {
        self.preferences
            .iter()
            .filter(move |preference| {
                preference.key().namespace() == Some(namespace)
            })
    }

    /// Returns preferences with an identical identity.
    ///
    /// The returned iterator preserves insertion order.
    pub fn for_key<'a>(
        &'a self,
        key: &'a PreferenceKey,
    ) -> impl Iterator<Item = &'a Preference<T>> + 'a {
        self.preferences
            .iter()
            .filter(move |preference| preference.key() == key)
    }

    /// Returns whether a preference with the given identity is present.
    ///
    /// This checks declaration presence only. It does not establish that
    /// the preference is applicable, achievable, or satisfied.
    #[must_use]
    pub fn contains(
        &self,
        key: &PreferenceKey,
    ) -> bool {
        self.preferences
            .iter()
            .any(|preference| preference.key() == key)
    }

    /// Removes all declarations from this collection.
    ///
    /// This is an explicit local collection operation. It does not mutate
    /// source declarations or canonical semantic models.
    pub fn clear(&mut self) {
        self.preferences.clear();
    }

    /// Consumes the collection and returns its declarations in insertion
    /// order.
    #[must_use]
    pub fn into_vec(self) -> Vec<Preference<T>> {
        self.preferences
    }

    /// Returns a deterministically ordered view without mutating the
    /// original collection.
    ///
    /// Entries are ordered by identity. Entries with identical identities
    /// retain their relative insertion order.
    ///
    /// Duplicate declarations are preserved.
    #[must_use]
    pub fn canonicalized(&self) -> Vec<&Preference<T>> {
        let mut result: Vec<&Preference<T>> =
            self.preferences.iter().collect();

        result.sort_by(|left, right| {
            left.key().cmp(right.key())
        });

        result
    }

    /// Reports groups of duplicate preference identities.
    ///
    /// The returned indices refer to the original insertion order.
    ///
    /// Duplicate reporting does not imply that preferences are contradictory,
    /// redundant, or interchangeable. Those judgments belong to the
    /// appropriate semantic or planning subsystem.
    #[must_use]
    pub fn duplicate_groups(
        &self,
    ) -> Vec<(PreferenceKey, Vec<usize>)> {
        let mut groups: BTreeMap<
            PreferenceKey,
            Vec<usize>,
        > = BTreeMap::new();

        for (index, preference) in
            self.preferences.iter().enumerate()
        {
            groups
                .entry(preference.key().clone())
                .or_default()
                .push(index);
        }

        groups
            .into_iter()
            .filter(|(_, indices)| indices.len() > 1)
            .collect()
    }
}

impl<T> IntoIterator for PreferenceSet<T> {
    type Item = Preference<T>;
    type IntoIter = std::vec::IntoIter<Preference<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.preferences.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a PreferenceSet<T> {
    type Item = &'a Preference<T>;
    type IntoIter = core::slice::Iter<'a, Preference<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.preferences.iter()
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Identifies the preference identity component that failed validation.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PreferenceComponent {
    /// Namespace component.
    Namespace,

    /// Preference name.
    Name,
}

impl fmt::Display for PreferenceComponent {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Namespace => formatter.write_str("namespace"),
            Self::Name => formatter.write_str("name"),
        }
    }
}

/// Errors raised while constructing preference identities and declarations.
///
/// These errors concern malformed local data only.
///
/// They do not mean that:
///
/// - A preference is unsupported.
/// - A capability is unavailable.
/// - A requirement is unsatisfied.
/// - A constraint is violated.
/// - A workload is infeasible.
/// - A provider has rejected a preference.
///
/// Those outcomes belong to their respective authorities.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PreferenceError {
    /// A required identity component was empty or whitespace-only.
    EmptyComponent {
        component: PreferenceComponent,
    },

    /// An identity component contained leading or trailing whitespace.
    SurroundingWhitespace {
        component: PreferenceComponent,
    },

    /// An identity component contained a control character.
    ControlCharacter {
        component: PreferenceComponent,
    },

    /// A namespace contained an empty namespace segment.
    EmptyNamespaceSegment,

    /// A preference name contained an invalid namespace separator.
    InvalidName {
        name: String,
        reason: String,
    },

    /// A preference rationale was empty or whitespace-only.
    EmptyRationale,

    /// A source range had an end preceding its start.
    InvalidSourceRange {
        start: u64,
        end: u64,
    },
}

impl fmt::Display for PreferenceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyComponent { component } => {
                write!(
                    formatter,
                    "preference {component} must not be empty"
                )
            }

            Self::SurroundingWhitespace { component } => {
                write!(
                    formatter,
                    "preference {component} must not have leading or trailing whitespace"
                )
            }

            Self::ControlCharacter { component } => {
                write!(
                    formatter,
                    "preference {component} must not contain control characters"
                )
            }

            Self::EmptyNamespaceSegment => {
                formatter.write_str(
                    "preference namespace must not contain empty segments",
                )
            }

            Self::InvalidName { name, reason } => {
                write!(
                    formatter,
                    "invalid preference name `{name}`: {reason}"
                )
            }

            Self::EmptyRationale => {
                formatter.write_str(
                    "preference rationale must not be empty",
                )
            }

            Self::InvalidSourceRange { start, end } => {
                write!(
                    formatter,
                    "invalid preference source range: end offset {end} precedes start offset {start}"
                )
            }
        }
    }
}

impl Error for PreferenceError {}

// ============================================================================
// INTERNAL VALIDATION
// ============================================================================

fn validate_component(
    value: &str,
    component: PreferenceComponent,
) -> Result<(), PreferenceError> {
    if value.trim().is_empty() {
        return Err(PreferenceError::EmptyComponent {
            component,
        });
    }

    if value.trim() != value {
        return Err(PreferenceError::SurroundingWhitespace {
            component,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(PreferenceError::ControlCharacter {
            component,
        });
    }

    Ok(())
}

fn validate_namespace(
    namespace: &str,
) -> Result<(), PreferenceError> {
    validate_component(
        namespace,
        PreferenceComponent::Namespace,
    )?;

    for segment in namespace.split("::") {
        if segment.is_empty() {
            return Err(PreferenceError::EmptyNamespaceSegment);
        }

        validate_component(
            segment,
            PreferenceComponent::Namespace,
        )?;
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(value: &str) -> PreferenceKey {
        PreferenceKey::from_qualified_name(value)
            .expect("valid preference key")
    }

    #[test]
    fn accepts_unqualified_preference_names() {
        for name in [
            "latency",
            "throughput",
            "energy",
            "cost",
            "performance",
            "portability",
            "reliability",
        ] {
            let preference_key =
                PreferenceKey::unqualified(name)
                    .expect("valid unqualified name");

            assert_eq!(preference_key.name(), name);
            assert_eq!(preference_key.namespace(), None);
            assert!(!preference_key.is_namespaced());
        }
    }

    #[test]
    fn accepts_extensible_namespaced_preferences() {
        for value in [
            "classical::locality",
            "quantum::fidelity",
            "hdl::timing_margin",
            "hybrid::communication_cost",
            "distributed::data_locality",
            "ai::inference_latency",
            "future::architecture::execution_characteristic",
            "vendor_extension::special_metric",
        ] {
            let preference_key = key(value);

            assert_eq!(
                preference_key.qualified_name(),
                value
            );
        }
    }

    #[test]
    fn preserves_namespace_and_name() {
        let preference_key =
            PreferenceKey::new(
                "future::architecture",
                "execution_metric",
            )
            .expect("valid preference key");

        assert_eq!(
            preference_key.namespace(),
            Some("future::architecture")
        );

        assert_eq!(
            preference_key.name(),
            "execution_metric"
        );

        assert_eq!(
            preference_key.to_string(),
            "future::architecture::execution_metric"
        );
    }

    #[test]
    fn parses_unqualified_and_qualified_names() {
        let unqualified =
            PreferenceKey::from_qualified_name("latency")
                .unwrap();

        let qualified =
            PreferenceKey::from_qualified_name(
                "quantum::fidelity",
            )
            .unwrap();

        assert_eq!(unqualified.namespace(), None);
        assert_eq!(unqualified.name(), "latency");

        assert_eq!(
            qualified.namespace(),
            Some("quantum")
        );

        assert_eq!(qualified.name(), "fidelity");
    }

    #[test]
    fn rejects_empty_or_malformed_names() {
        for value in [
            "",
            " ",
            " latency",
            "latency ",
            "::latency",
            "quantum::",
            "quantum::::fidelity",
            "quantum::fidelity::",
        ] {
            assert!(
                PreferenceKey::from_qualified_name(value).is_err(),
                "unexpectedly accepted {value:?}"
            );
        }
    }

    #[test]
    fn rejects_control_characters() {
        assert!(
            PreferenceKey::unqualified("latency\n")
                .is_err()
        );

        assert!(
            PreferenceKey::new(
                "quantum",
                "fidelity\0",
            )
            .is_err()
        );
    }

    #[test]
    fn does_not_impose_an_arbitrary_name_length_limit() {
        let long_name = "a".repeat(4096);

        let preference_key =
            PreferenceKey::unqualified(long_name.clone())
                .expect("long preference name is valid");

        assert_eq!(
            preference_key.name(),
            long_name
        );
    }

    #[test]
    fn preference_retains_typed_payload() {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct SemanticPayload {
            expression_id: String,
            expression_schema: String,
        }

        let payload = SemanticPayload {
            expression_id: "node-17".to_owned(),
            expression_schema: "resource-expression-v1".to_owned(),
        };

        let preference = Preference::new(
            key("quantum::fidelity"),
            payload.clone(),
        );

        assert_eq!(preference.value(), &payload);
    }

    #[test]
    fn preference_does_not_require_payload_interpretation() {
        let preference = Preference::new(
            key("latency"),
            vec![1_u8, 2_u8, 3_u8],
        );

        assert_eq!(
            preference.value(),
            &[1_u8, 2_u8, 3_u8]
        );
    }

    #[test]
    fn rationale_is_optional_and_validated() {
        let preference = Preference::new(
            key("energy"),
            "minimize".to_owned(),
        )
        .with_rationale("Reduce energy consumption")
        .expect("valid rationale");

        assert_eq!(
            preference.rationale(),
            Some("Reduce energy consumption")
        );

        assert!(
            Preference::new(
                key("energy"),
                "minimize".to_owned(),
            )
            .with_rationale("   ")
            .is_err()
        );
    }

    #[test]
    fn provenance_is_preserved() {
        let origin = PreferenceOrigin::new()
            .with_source_id("module.zm")
            .with_node_id("node-17")
            .with_producer("frontend.semantic")
            .with_byte_range(10, 35)
            .expect("valid source range");

        let preference = Preference::new(
            key("classical::locality"),
            "prefer-local".to_owned(),
        )
        .with_origin(origin);

        assert_eq!(
            preference.origin().source_id(),
            Some("module.zm")
        );

        assert_eq!(
            preference.origin().node_id(),
            Some("node-17")
        );

        assert_eq!(
            preference.origin().producer(),
            Some("frontend.semantic")
        );

        assert_eq!(
            preference.origin().byte_range(),
            Some((10, 35))
        );
    }

    #[test]
    fn rejects_invalid_source_ranges() {
        assert_eq!(
            PreferenceOrigin::new()
                .with_byte_range(20, 10),
            Err(PreferenceError::InvalidSourceRange {
                start: 20,
                end: 10,
            })
        );
    }

    #[test]
    fn maps_payload_without_losing_metadata() {
        let preference = Preference::new(
            key("distributed::locality"),
            10_u64,
        )
        .with_rationale("Keep related data close")
        .unwrap()
        .with_origin(
            PreferenceOrigin::new()
                .with_producer("frontend.semantic"),
        );

        let mapped = preference.map_value(
            |value| value.to_string(),
        );

        assert_eq!(mapped.value(), "10");

        assert_eq!(
            mapped.rationale(),
            Some("Keep related data close")
        );

        assert_eq!(
            mapped.origin().producer(),
            Some("frontend.semantic")
        );

        assert_eq!(
            mapped.key().to_string(),
            "distributed::locality"
        );
    }

    #[test]
    fn preserves_duplicate_preferences() {
        let first = Preference::new(
            key("latency"),
            "minimize".to_owned(),
        );

        let second = Preference::new(
            key("latency"),
            "minimize".to_owned(),
        );

        let mut preferences = PreferenceSet::new();

        preferences.push(first);
        preferences.push(second);

        assert_eq!(preferences.len(), 2);
        assert_eq!(preferences.duplicate_groups().len(), 1);
    }

    #[test]
    fn preserves_different_values_for_the_same_identity() {
        let first = Preference::new(
            key("energy"),
            "minimize".to_owned(),
        );

        let second = Preference::new(
            key("energy"),
            "maximize-efficiency".to_owned(),
        );

        let preferences =
            PreferenceSet::from_preferences([
                first,
                second,
            ]);

        let values: Vec<&str> = preferences
            .for_key(&key("energy"))
            .map(|preference| preference.value().as_str())
            .collect();

        assert_eq!(
            values,
            vec![
                "minimize",
                "maximize-efficiency",
            ]
        );
    }

    #[test]
    fn preserves_insertion_order() {
        let preferences =
            PreferenceSet::from_preferences([
                Preference::new(
                    key("quantum::fidelity"),
                    "high".to_owned(),
                ),
                Preference::new(
                    key("classical::locality"),
                    "prefer-local".to_owned(),
                ),
                Preference::new(
                    key("distributed::locality"),
                    "prefer-nearby".to_owned(),
                ),
            ]);

        assert_eq!(
            preferences
                .get(0)
                .unwrap()
                .key()
                .to_string(),
            "quantum::fidelity"
        );

        assert_eq!(
            preferences
                .get(1)
                .unwrap()
                .key()
                .to_string(),
            "classical::locality"
        );
    }

    #[test]
    fn canonicalized_view_is_deterministic() {
        let preferences =
            PreferenceSet::from_preferences([
                Preference::new(
                    key("quantum::fidelity"),
                    "high".to_owned(),
                ),
                Preference::new(
                    key("classical::locality"),
                    "local".to_owned(),
                ),
                Preference::new(
                    key("energy"),
                    "minimize".to_owned(),
                ),
            ]);

        let original_first =
            preferences.get(0).unwrap().key().to_string();

        let canonicalized =
            preferences.canonicalized();

        assert_eq!(
            canonicalized[0].key().to_string(),
            "classical::locality"
        );

        assert_eq!(
            canonicalized[1].key().to_string(),
            "energy"
        );

        assert_eq!(
            canonicalized[2].key().to_string(),
            "quantum::fidelity"
        );

        assert_eq!(
            preferences.get(0).unwrap().key().to_string(),
            original_first
        );
    }

    #[test]
    fn canonicalization_preserves_relative_order_of_duplicates() {
        let preferences =
            PreferenceSet::from_preferences([
                Preference::new(
                    key("latency"),
                    "first".to_owned(),
                ),
                Preference::new(
                    key("energy"),
                    "second".to_owned(),
                ),
                Preference::new(
                    key("latency"),
                    "third".to_owned(),
                ),
            ]);

        let canonicalized =
            preferences.canonicalized();

        let latency_values: Vec<&str> = canonicalized
            .iter()
            .filter(|preference| preference.key().name() == "latency")
            .map(|preference| preference.value().as_str())
            .collect();

        assert_eq!(
            latency_values,
            vec!["first", "third"]
        );
    }

    #[test]
    fn duplicate_groups_report_original_indices() {
        let preferences =
            PreferenceSet::from_preferences([
                Preference::new(
                    key("latency"),
                    "minimize".to_owned(),
                ),
                Preference::new(
                    key("energy"),
                    "minimize".to_owned(),
                ),
                Preference::new(
                    key("latency"),
                    "prefer-low".to_owned(),
                ),
                Preference::new(
                    key("latency"),
                    "prefer-minimum".to_owned(),
                ),
            ]);

        let groups =
            preferences.duplicate_groups();

        assert_eq!(groups.len(), 1);

        assert_eq!(
            groups[0].0,
            key("latency")
        );

        assert_eq!(
            groups[0].1,
            vec![0, 2, 3]
        );
    }

    #[test]
    fn namespace_filter_preserves_order() {
        let preferences =
            PreferenceSet::from_preferences([
                Preference::new(
                    key("quantum::fidelity"),
                    "high".to_owned(),
                ),
                Preference::new(
                    key("classical::locality"),
                    "local".to_owned(),
                ),
                Preference::new(
                    key("quantum::latency"),
                    "low".to_owned(),
                ),
            ]);

        let quantum_names: Vec<&str> = preferences
            .for_namespace("quantum")
            .map(|preference| preference.key().name())
            .collect();

        assert_eq!(
            quantum_names,
            vec!["fidelity", "latency"]
        );
    }

    #[test]
    fn extend_preserves_entries_and_duplicates() {
        let mut first = PreferenceSet::new();

        first.push(Preference::new(
            key("latency"),
            "minimize".to_owned(),
        ));

        let second = PreferenceSet::from_preferences([
            Preference::new(
                key("energy"),
                "minimize".to_owned(),
            ),
            Preference::new(
                key("latency"),
                "prefer-low".to_owned(),
            ),
        ]);

        first.extend(second);

        assert_eq!(first.len(), 3);
        assert_eq!(first.duplicate_groups().len(), 1);
    }

    #[test]
    fn clear_removes_all_local_declarations() {
        let mut preferences =
            PreferenceSet::from_preferences([
                Preference::new(
                    key("latency"),
                    "minimize".to_owned(),
                ),
            ]);

        preferences.clear();

        assert!(preferences.is_empty());
    }

    #[test]
    fn owned_iteration_preserves_insertion_order() {
        let preferences =
            PreferenceSet::from_preferences([
                Preference::new(
                    key("latency"),
                    "first".to_owned(),
                ),
                Preference::new(
                    key("energy"),
                    "second".to_owned(),
                ),
            ]);

        let values: Vec<String> = preferences
            .into_iter()
            .map(|preference| preference.value().clone())
            .collect();

        assert_eq!(
            values,
            vec![
                "first".to_owned(),
                "second".to_owned(),
            ]
        );
    }

    #[test]
    fn preference_identity_is_hashable_and_orderable() {
        use std::collections::BTreeSet;
        use std::collections::HashSet;

        let mut ordered = BTreeSet::new();
        let mut hashed = HashSet::new();

        ordered.insert(key("latency"));
        ordered.insert(key("energy"));

        hashed.insert(key("latency"));
        hashed.insert(key("energy"));

        assert_eq!(ordered.len(), 2);
        assert_eq!(hashed.len(), 2);
    }
}
