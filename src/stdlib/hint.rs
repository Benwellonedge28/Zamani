
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Hint Infrastructure
//!
//! Path: `src/stdlib/hint.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines portable, extensible contracts for non-binding implementation
//! and optimization guidance.
//!
//! A hint communicates information that a downstream implementation MAY
//! consider when deciding how to realize an otherwise valid computation.
//!
//! A hint is advisory. It MUST NOT independently change program semantics,
//! establish capability availability, authorize an operation, impose a
//! mandatory condition, or establish execution feasibility.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Open-world hint identities.
//! - Generic, typed hint payloads.
//! - Optional explanatory rationale.
//! - Opaque hint provenance.
//! - Optional descriptive metadata.
//! - Ordered hint collections.
//! - Duplicate declaration preservation.
//! - Deterministic collection inspection.
//! - Local declaration validation.
//!
//! This module does NOT own:
//!
//! - Source grammar or parsing.
//! - AST nodes or source spans.
//! - Semantic name resolution.
//! - Semantic interpretation of hint payloads.
//! - Resource quantities, units, or expressions.
//! - Requirement definitions or evaluation.
//! - Capability definitions or discovery.
//! - Preference scoring or ranking.
//! - Constraint enforcement.
//! - Feasibility analysis.
//! - Provider discovery or selection.
//! - Hardware discovery or allocation.
//! - Target selection.
//! - Optimization algorithms.
//! - Scheduling or routing.
//! - Runtime execution.
//! - IR definitions or transformations.
//!
//! Those responsibilities remain with their canonical repository
//! authorities.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Source grammar:
//!     grammar/Zamani.g4
//!
//! Resource-hint grammar:
//!     grammar/resources/hints.g4
//!
//! Resource grammar composition:
//!     grammar/resources/resources.g4
//!
//! Semantic model:
//!     grammar/specification/semantic-model.md
//!
//! Scalability model:
//!     grammar/specification/scalability-model.md
//!
//! POCO-REAF:
//!     grammar/specification/poco-reaf.md
//!
//! Canonical semantic authority:
//!     src/frontend/semantic/
//!
//! Canonical resource authority:
//!     src/frontend/semantic/resources/
//!
//! Compiler analysis and planning:
//!     src/compiler/analysis/
//!     src/compiler/planning/
//!
//! Universal intermediate representation:
//!     src/zuir/
//!
//! Runtime:
//!     src/runtime/
//!
//! Hardware abstraction:
//!     Existing HAL and provider implementations.
//!
//! ## FUNDAMENTAL DISTINCTIONS
//!
//! Requirement:
//!     A mandatory condition that must be satisfied.
//!
//! Capability:
//!     An ability an implementation or environment can provide.
//!
//! Preference:
//!     A desirable, non-mandatory property.
//!
//! Constraint:
//!     A mandatory restriction on acceptable outcomes.
//!
//! Hint:
//!     Non-binding implementation or optimization guidance.
//!
//! Feasibility:
//!     An evidence-based assessment of whether applicable conditions
//!     can be satisfied.
//!
//! These concepts MUST NOT be silently converted into one another.
//!
//! ## POCO-REAF
//!
//! Hint identities are open-world.
//!
//! New computational domains, technologies, optimization strategies,
//! execution characteristics, and future extensions can introduce
//! new hint identities without modifying this module.
//!
//! The generic payload allows the canonical frontend semantic model,
//! resource model, compiler, or another domain authority to provide
//! its own typed representation.
//!
//! This module does not impose arbitrary universal maximums on:
//!
//! - Hint declarations.
//! - Namespace depth.
//! - Identifier length.
//! - Metadata entries.
//! - Hint collection size.
//! - Computing domains.
//! - CPUs, GPUs, FPGAs, ASICs, or QPUs.
//! - Logical or physical qubits.
//! - Memory or storage.
//! - Register width.
//! - Tensor rank.
//! - Distributed participants.
//!
//! Actual memory and processing requirements depend on available
//! resources and explicitly configured operational budgets.
//!
//! This does not promise literally infinite computation or storage.
//!
//! ## DETERMINISM
//!
//! Hint identity is determined by namespace and name.
//!
//! Hint collections preserve insertion order and duplicate declarations.
//!
//! Metadata uses deterministic key ordering.
//!
//! Canonicalized collection views use deterministic identity ordering.
//!
//! No timestamp, random identifier, address, hardware state, or global
//! mutable registry participates in hint identity.
//!
//! ## PAYLOAD CONTRACT
//!
//! Hint<T> stores a caller-supplied typed payload.
//!
//! The payload may represent:
//!
//! - Resource optimization guidance.
//! - Compiler optimization guidance.
//! - Execution strategy suggestions.
//! - Placement suggestions.
//! - Scheduling suggestions.
//! - Quantum realization guidance.
//! - HDL synthesis guidance.
//! - Distributed execution guidance.
//! - AI and tensor execution guidance.
//! - Future computational-domain guidance.
//!
//! This module does not parse, evaluate, normalize, compare, or execute T.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No global mutable state.
//! No implicit logging.
//! No printing.
//! No I/O.
//! No hardware inspection.
//! No runtime initialization.
//! No hidden evaluation.
//! No panic-based input validation.
//!
//! ## INTEGRATION
//!
//! `src/stdlib/mod.rs` must expose this module:
//!
//!     pub mod hint;
//!
//! Frontend and semantic integrations must preserve canonical typed
//! payloads through explicit adapters.
//!
//! Compiler and IR integrations must preserve the advisory nature
//! of hints.
//!
//! Runtime and provider integrations MAY consume applicable hints,
//! but MUST NOT interpret them as mandatory conditions.
//!
//! No semantic, IR, compiler, or runtime subsystem should depend on
//! this module merely to obtain its own canonical semantic hint model.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

// ============================================================================
// HINT IDENTITY
// ============================================================================

/// A stable, open-world identity for an advisory hint.
///
/// An identity consists of an optional namespace and a name.
///
/// The namespace distinguishes domains and extensions.
///
/// The name identifies an individual hint within its namespace.
///
/// Examples:
///
/// - `execution::locality`
/// - `execution::parallelism`
/// - `resource::memory_locality`
/// - `classical::vectorization`
/// - `quantum::error_mitigation`
/// - `hdl::synthesis_strategy`
/// - `distributed::data_locality`
/// - `ai::tensor_layout`
/// - `future::execution_strategy`
///
/// These are illustrative identifiers, not a closed list.
///
/// Namespaces and names are open-world.
///
/// Identity strings are preserved exactly.
///
/// This type does not perform semantic name resolution, case folding,
/// alias expansion, grammar parsing, or provider lookup.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HintKey {
    namespace: Option<String>,
    name: String,
}

impl HintKey {
    /// Constructs a namespaced hint identity.
    ///
    /// Namespace components may be separated by `::`.
    ///
    /// Empty components, leading or trailing whitespace, and control
    /// characters are rejected.
    ///
    /// No arbitrary maximum namespace depth or identifier length
    /// is imposed.
    ///
    /// # Errors
    ///
    /// Returns [`HintError`] when a component is malformed.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, HintError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_namespace(&namespace)?;
        validate_name(&name)?;

        Ok(Self {
            namespace: Some(namespace),
            name,
        })
    }

    /// Constructs an unqualified hint identity.
    ///
    /// # Errors
    ///
    /// Returns [`HintError`] when the name is malformed.
    pub fn unqualified(
        name: impl Into<String>,
    ) -> Result<Self, HintError> {
        let name = name.into();

        validate_name(&name)?;

        Ok(Self {
            namespace: None,
            name,
        })
    }

    /// Returns the optional namespace exactly as stored.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Returns the hint name exactly as stored.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the fully qualified textual identity.
    ///
    /// Unqualified names are returned without a namespace separator.
    ///
    /// Structured comparisons should use [`HintKey`] itself.
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
        self.namespace.as_deref() == namespace
            && self.name == name
    }
}

impl fmt::Display for HintKey {
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

impl TryFrom<(&str, &str)> for HintKey {
    type Error = HintError;

    fn try_from(
        (namespace, name): (&str, &str),
    ) -> Result<Self, Self::Error> {
        Self::new(namespace, name)
    }
}

// ============================================================================
// HINT ORIGIN
// ============================================================================

/// Describes the origin of a hint.
///
/// Origin is opaque provenance metadata.
///
/// It does not establish that a hint is trustworthy, authorized,
/// semantically valid, or applicable.
///
/// Trust and provenance verification remain the responsibility
/// of the canonical security and semantic authorities.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HintOrigin {
    system: String,
    identifier: Option<String>,
}

impl HintOrigin {
    /// Creates a hint origin.
    ///
    /// `system` identifies the originating subsystem or authority.
    ///
    /// `identifier` optionally identifies a particular declaration,
    /// artifact, analysis result, or originating record.
    ///
    /// # Errors
    ///
    /// Returns [`HintError`] if either supplied component is malformed.
    pub fn new(
        system: impl Into<String>,
        identifier: Option<String>,
    ) -> Result<Self, HintError> {
        let system = system.into();

        validate_text(
            &system,
            HintComponent::OriginSystem,
        )?;

        if let Some(value) = &identifier {
            validate_text(
                value,
                HintComponent::OriginIdentifier,
            )?;
        }

        Ok(Self {
            system,
            identifier,
        })
    }

    /// Returns the originating system.
    #[must_use]
    pub fn system(&self) -> &str {
        &self.system
    }

    /// Returns the optional originating identifier.
    #[must_use]
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }
}

// ============================================================================
// HINT DECLARATION
// ============================================================================

/// A typed, non-binding implementation hint.
///
/// `T` is supplied by the canonical semantic subsystem or an explicit
/// integration adapter.
///
/// The payload is intentionally generic.
///
/// This prevents the standard library from introducing a competing
/// resource-expression language or domain-specific semantic model.
///
/// A hint MAY be ignored by a valid implementation.
///
/// A hint MUST NOT override a mandatory requirement or constraint.
///
/// A hint MUST NOT establish capability availability or feasibility.
///
/// The payload is never evaluated by this module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hint<T> {
    key: HintKey,
    payload: T,
    rationale: Option<String>,
    origin: Option<HintOrigin>,
    metadata: BTreeMap<String, String>,
}

impl<T> Hint<T> {
    /// Constructs a hint with a typed payload.
    ///
    /// No semantic interpretation of the payload is performed.
    #[must_use]
    pub fn new(
        key: HintKey,
        payload: T,
    ) -> Self {
        Self {
            key,
            payload,
            rationale: None,
            origin: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Constructs a hint from namespace, name, and payload.
    ///
    /// # Errors
    ///
    /// Returns [`HintError`] if the identity is malformed.
    pub fn from_parts(
        namespace: impl Into<String>,
        name: impl Into<String>,
        payload: T,
    ) -> Result<Self, HintError> {
        Ok(Self::new(
            HintKey::new(namespace, name)?,
            payload,
        ))
    }

    /// Returns the hint identity.
    #[must_use]
    pub fn key(&self) -> &HintKey {
        &self.key
    }

    /// Returns the typed hint payload.
    #[must_use]
    pub fn payload(&self) -> &T {
        &self.payload
    }

    /// Returns mutable access to the payload.
    ///
    /// Mutating a payload does not change the hint identity.
    ///
    /// The caller remains responsible for preserving the payload's
    /// own canonical semantic contract.
    pub fn payload_mut(&mut self) -> &mut T {
        &mut self.payload
    }

    /// Consumes the hint and returns its payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }

    /// Returns the optional explanatory rationale.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }

    /// Attaches explanatory rationale.
    ///
    /// Rationale is descriptive metadata.
    ///
    /// It cannot change a hint into a requirement, preference,
    /// constraint, capability, or feasibility result.
    ///
    /// # Errors
    ///
    /// Returns [`HintError`] if the rationale is empty,
    /// contains only whitespace, or contains control characters.
    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Result<Self, HintError> {
        let rationale = rationale.into();

        validate_text(
            &rationale,
            HintComponent::Rationale,
        )?;

        self.rationale = Some(rationale);

        Ok(self)
    }

    /// Returns the optional origin.
    #[must_use]
    pub fn origin(&self) -> Option<&HintOrigin> {
        self.origin.as_ref()
    }

    /// Attaches opaque provenance.
    ///
    /// Provenance does not independently establish trust,
    /// authorization, or correctness.
    #[must_use]
    pub fn with_origin(
        mut self,
        origin: HintOrigin,
    ) -> Self {
        self.origin = Some(origin);
        self
    }

    /// Returns the immutable metadata map.
    ///
    /// Metadata keys are presented in deterministic lexical order.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Retrieves one metadata value without allocating.
    #[must_use]
    pub fn metadata_value(
        &self,
        key: &str,
    ) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Adds or replaces descriptive metadata.
    ///
    /// Metadata does not participate in hint identity.
    ///
    /// Replacing an existing metadata key is explicit and deterministic.
    ///
    /// # Errors
    ///
    /// Returns [`HintError`] if the metadata key or value is malformed.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, HintError> {
        self.insert_metadata(key, value)?;
        Ok(self)
    }

    /// Inserts or replaces descriptive metadata.
    ///
    /// Returns the previous value when the key already existed.
    ///
    /// # Errors
    ///
    /// Returns [`HintError`] if the metadata key or value is malformed.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, HintError> {
        let key = key.into();
        let value = value.into();

        validate_text(
            &key,
            HintComponent::MetadataKey,
        )?;

        validate_text(
            &value,
            HintComponent::MetadataValue,
        )?;

        Ok(self.metadata.insert(key, value))
    }

    /// Returns the number of metadata entries.
    #[must_use]
    pub fn metadata_len(&self) -> usize {
        self.metadata.len()
    }

    /// Returns whether descriptive metadata is empty.
    #[must_use]
    pub fn metadata_is_empty(&self) -> bool {
        self.metadata.is_empty()
    }

    /// Replaces the payload while preserving declaration metadata.
    #[must_use]
    pub fn map_payload<U>(
        self,
        transform: impl FnOnce(T) -> U,
    ) -> Hint<U> {
        Hint {
            key: self.key,
            payload: transform(self.payload),
            rationale: self.rationale,
            origin: self.origin,
            metadata: self.metadata,
        }
    }

    /// Converts a borrowed hint into a hint containing a transformed
    /// payload while preserving declaration metadata.
    #[must_use]
    pub fn map_payload_ref<U>(
        &self,
        transform: impl FnOnce(&T) -> U,
    ) -> Hint<U> {
        Hint {
            key: self.key.clone(),
            payload: transform(&self.payload),
            rationale: self.rationale.clone(),
            origin: self.origin.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

// ============================================================================
// HINT COLLECTION
// ============================================================================

/// An ordered collection of advisory hints.
///
/// Insertion order is preserved.
///
/// Duplicate identities are deliberately preserved.
///
/// Different declarations with the same identity may have different
/// payloads, provenance, or explanatory metadata.
///
/// This collection does not resolve conflicts, rank hints, enforce hints,
/// or decide whether a hint applies to a particular target.
///
/// No arbitrary collection-size ceiling is imposed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HintSet<T> {
    hints: Vec<Hint<T>>,
}

impl<T> HintSet<T> {
    /// Creates an empty hint collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hints: Vec::new(),
        }
    }

    /// Creates a hint collection with an allocation capacity.
    ///
    /// The capacity is an allocation hint, not a collection-size limit.
    ///
    /// # Panics
    ///
    /// May panic if the requested allocation capacity cannot be reserved.
    ///
    /// Use [`HintSet::try_with_capacity`] when allocation failure must
    /// be reported rather than propagated as a panic.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            hints: Vec::with_capacity(capacity),
        }
    }

    /// Creates a hint collection using fallible allocation.
    ///
    /// # Errors
    ///
    /// Returns [`HintError::Allocation`] if the requested capacity
    /// cannot be reserved.
    pub fn try_with_capacity(
        capacity: usize,
    ) -> Result<Self, HintError> {
        let mut hints = Vec::new();

        hints
            .try_reserve_exact(capacity)
            .map_err(|error| {
                HintError::Allocation {
                    message: error.to_string(),
                }
            })?;

        Ok(Self { hints })
    }

    /// Appends a hint while preserving insertion order.
    ///
    /// Duplicate identities are not discarded.
    pub fn push(
        &mut self,
        hint: Hint<T>,
    ) {
        self.hints.push(hint);
    }

    /// Attempts to reserve additional collection capacity.
    ///
    /// # Errors
    ///
    /// Returns [`HintError::Allocation`] if the reservation fails.
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), HintError> {
        self.hints
            .try_reserve(additional)
            .map_err(|error| HintError::Allocation {
                message: error.to_string(),
            })
    }

    /// Attempts to reserve additional exact collection capacity.
    ///
    /// This is an allocation operation only.
    ///
    /// # Errors
    ///
    /// Returns [`HintError::Allocation`] if the reservation fails.
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), HintError> {
        self.hints
            .try_reserve_exact(additional)
            .map_err(|error| HintError::Allocation {
                message: error.to_string(),
            })
    }

    /// Appends all supplied hints in iteration order.
    pub fn extend<I>(
        &mut self,
        hints: I,
    )
    where
        I: IntoIterator<Item = Hint<T>>,
    {
        self.hints.extend(hints);
    }

    /// Returns the number of stored hints.
    #[must_use]
    pub fn len(&self) -> usize {
        self.hints.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.hints.is_empty()
    }

    /// Returns an iterator preserving insertion order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &Hint<T>> {
        self.hints.iter()
    }

    /// Returns a mutable iterator preserving insertion order.
    pub fn iter_mut(
        &mut self,
    ) -> impl ExactSizeIterator<Item = &mut Hint<T>> {
        self.hints.iter_mut()
    }

    /// Retrieves a hint by its identity and occurrence index.
    ///
    /// The occurrence index counts only matching identities.
    ///
    /// This allows duplicate declarations to remain independently
    /// accessible.
    #[must_use]
    pub fn get(
        &self,
        key: &HintKey,
        occurrence: usize,
    ) -> Option<&Hint<T>> {
        self.hints
            .iter()
            .filter(|hint| hint.key() == key)
            .nth(occurrence)
    }

    /// Returns all hints matching an identity.
    pub fn get_all<'a>(
        &'a self,
        key: &'a HintKey,
    ) -> impl Iterator<Item = &'a Hint<T>> + 'a {
        self.hints
            .iter()
            .filter(move |hint| hint.key() == key)
    }

    /// Checks whether an identity is present.
    #[must_use]
    pub fn contains_key(
        &self,
        key: &HintKey,
    ) -> bool {
        self.hints
            .iter()
            .any(|hint| hint.key() == key)
    }

    /// Removes all hints matching an identity.
    ///
    /// Returns the number of removed declarations.
    ///
    /// Removal preserves the relative order of remaining declarations.
    pub fn remove_all(
        &mut self,
        key: &HintKey,
    ) -> usize {
        let original_len = self.hints.len();

        self.hints
            .retain(|hint| hint.key() != key);

        original_len - self.hints.len()
    }

    /// Consumes the collection and returns its ordered declarations.
    #[must_use]
    pub fn into_vec(self) -> Vec<Hint<T>> {
        self.hints
    }

    /// Returns a deterministic, identity-sorted borrowed view.
    ///
    /// The original insertion order is not modified.
    ///
    /// Declarations with identical keys retain their relative
    /// insertion order.
    #[must_use]
    pub fn canonicalized(&self) -> Vec<&Hint<T>> {
        let mut ordered: Vec<&Hint<T>> =
            self.hints.iter().collect();

        ordered.sort_by(|left, right| {
            left.key().cmp(right.key())
        });

        ordered
    }

    /// Returns groups of duplicate declaration indices.
    ///
    /// Each index refers to the original insertion order.
    ///
    /// Groups are ordered by identity.
    ///
    /// Only identities with more than one declaration are returned.
    ///
    /// No duplicate declarations are removed.
    #[must_use]
    pub fn duplicate_groups(&self) -> Vec<Vec<usize>> {
        let mut indices: BTreeMap<&HintKey, Vec<usize>> =
            BTreeMap::new();

        for (index, hint) in self.hints.iter().enumerate() {
            indices
                .entry(hint.key())
                .or_default()
                .push(index);
        }

        indices
            .into_values()
            .filter(|group| group.len() > 1)
            .collect()
    }

    /// Returns the total number of distinct hint identities.
    #[must_use]
    pub fn distinct_key_count(&self) -> usize {
        self.hints
            .iter()
            .map(Hint::key)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    }

    /// Clears all declarations.
    ///
    /// The allocated capacity may be retained for reuse.
    pub fn clear(&mut self) {
        self.hints.clear();
    }
}

impl<T> Default for HintSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for HintSet<T> {
    type Item = Hint<T>;
    type IntoIter = std::vec::IntoIter<Hint<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.hints.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a HintSet<T> {
    type Item = &'a Hint<T>;
    type IntoIter = std::slice::Iter<'a, Hint<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.hints.iter()
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Identifies the component that failed local hint validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HintComponent {
    /// Namespace component.
    Namespace,

    /// Hint name.
    Name,

    /// Explanatory rationale.
    Rationale,

    /// Originating subsystem.
    OriginSystem,

    /// Origin identifier.
    OriginIdentifier,

    /// Metadata key.
    MetadataKey,

    /// Metadata value.
    MetadataValue,
}

impl fmt::Display for HintComponent {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Namespace => "namespace",
            Self::Name => "name",
            Self::Rationale => "rationale",
            Self::OriginSystem => "origin system",
            Self::OriginIdentifier => "origin identifier",
            Self::MetadataKey => "metadata key",
            Self::MetadataValue => "metadata value",
        };

        formatter.write_str(value)
    }
}

/// Errors produced by local hint declaration and collection operations.
///
/// These errors describe failures in this module's own contracts.
///
/// They do not represent semantic invalidity, unsupported hardware,
/// unavailable capabilities, or execution infeasibility.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HintError {
    /// A required textual component is empty.
    EmptyComponent {
        /// The component that was empty.
        component: HintComponent,
    },

    /// A component contains leading or trailing whitespace.
    SurroundingWhitespace {
        /// The malformed component.
        component: HintComponent,
    },

    /// A component contains a control character.
    ControlCharacter {
        /// The malformed component.
        component: HintComponent,
    },

    /// A namespace contains an empty segment.
    InvalidNamespace {
        /// The malformed namespace.
        namespace: String,
    },

    /// A hint name contains a namespace separator.
    QualifiedName {
        /// The malformed name.
        name: String,
    },

    /// A fallible collection allocation failed.
    Allocation {
        /// Description of the allocation failure.
        message: String,
    },
}

impl fmt::Display for HintError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyComponent { component } => {
                write!(
                    formatter,
                    "hint {component} cannot be empty"
                )
            }

            Self::SurroundingWhitespace { component } => {
                write!(
                    formatter,
                    "hint {component} cannot contain leading \
                     or trailing whitespace"
                )
            }

            Self::ControlCharacter { component } => {
                write!(
                    formatter,
                    "hint {component} cannot contain control characters"
                )
            }

            Self::InvalidNamespace { namespace } => {
                write!(
                    formatter,
                    "invalid hint namespace: {namespace:?}"
                )
            }

            Self::QualifiedName { name } => {
                write!(
                    formatter,
                    "hint name must not contain a namespace separator: \
                     {name:?}"
                )
            }

            Self::Allocation { message } => {
                write!(
                    formatter,
                    "hint collection allocation failed: {message}"
                )
            }
        }
    }
}

impl Error for HintError {}

// ============================================================================
// INTERNAL VALIDATION HELPERS
// ============================================================================

fn validate_text(
    value: &str,
    component: HintComponent,
) -> Result<(), HintError> {
    if value.trim().is_empty() {
        return Err(HintError::EmptyComponent {
            component,
        });
    }

    if value.trim() != value {
        return Err(HintError::SurroundingWhitespace {
            component,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(HintError::ControlCharacter {
            component,
        });
    }

    Ok(())
}

fn validate_namespace(
    namespace: &str,
) -> Result<(), HintError> {
    validate_text(
        namespace,
        HintComponent::Namespace,
    )?;

    if namespace
        .split("::")
        .any(str::is_empty)
    {
        return Err(HintError::InvalidNamespace {
            namespace: namespace.to_owned(),
        });
    }

    Ok(())
}

fn validate_name(
    name: &str,
) -> Result<(), HintError> {
    validate_text(
        name,
        HintComponent::Name,
    )?;

    if name.contains("::") {
        return Err(HintError::QualifiedName {
            name: name.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(
        namespace: &str,
        name: &str,
    ) -> HintKey {
        HintKey::new(namespace, name)
            .expect("valid test identity")
    }

    #[test]
    fn constructs_namespaced_hint() {
        let identity = key(
            "execution",
            "parallelism",
        );

        assert_eq!(
            identity.namespace(),
            Some("execution")
        );

        assert_eq!(
            identity.name(),
            "parallelism"
        );

        assert_eq!(
            identity.qualified_name(),
            "execution::parallelism"
        );
    }

    #[test]
    fn constructs_unqualified_hint() {
        let identity = HintKey::unqualified("locality")
            .expect("valid test identity");

        assert_eq!(identity.namespace(), None);
        assert_eq!(identity.name(), "locality");
        assert_eq!(identity.qualified_name(), "locality");
    }

    #[test]
    fn supports_open_world_namespaces() {
        let identity = key(
            "future::photonic::computing",
            "execution_strategy",
        );

        assert_eq!(
            identity.namespace(),
            Some("future::photonic::computing")
        );
    }

    #[test]
    fn rejects_empty_namespace() {
        assert!(matches!(
            HintKey::new("", "locality"),
            Err(HintError::EmptyComponent {
                component: HintComponent::Namespace
            })
        ));
    }

    #[test]
    fn rejects_empty_name() {
        assert!(matches!(
            HintKey::new("execution", ""),
            Err(HintError::EmptyComponent {
                component: HintComponent::Name
            })
        ));
    }

    #[test]
    fn rejects_empty_namespace_segments() {
        assert!(matches!(
            HintKey::new("execution::::parallel", "locality"),
            Err(HintError::InvalidNamespace { .. })
        ));
    }

    #[test]
    fn rejects_qualified_name_component() {
        assert!(matches!(
            HintKey::new("execution", "parallel::locality"),
            Err(HintError::QualifiedName { .. })
        ));
    }

    #[test]
    fn rejects_control_characters() {
        assert!(matches!(
            HintKey::unqualified("local\nity"),
            Err(HintError::ControlCharacter {
                component: HintComponent::Name
            })
        ));
    }

    #[test]
    fn preserves_typed_payload() {
        let hint = Hint::new(
            key("execution", "parallelism"),
            128_u64,
        );

        assert_eq!(*hint.payload(), 128);
    }

    #[test]
    fn supports_generic_domain_payloads() {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct DomainGuidance {
            strategy: String,
            enabled: bool,
        }

        let payload = DomainGuidance {
            strategy: "adaptive".to_owned(),
            enabled: true,
        };

        let hint = Hint::new(
            key("future", "strategy"),
            payload.clone(),
        );

        assert_eq!(hint.payload(), &payload);
    }

    #[test]
    fn supports_rationale() {
        let hint = Hint::new(
            key("execution", "locality"),
            "prefer-local-data",
        )
        .with_rationale("Reduce communication overhead")
        .expect("valid rationale");

        assert_eq!(
            hint.rationale(),
            Some("Reduce communication overhead")
        );
    }

    #[test]
    fn rejects_empty_rationale() {
        let result = Hint::new(
            key("execution", "locality"),
            "local",
        )
        .with_rationale("   ");

        assert!(matches!(
            result,
            Err(HintError::EmptyComponent {
                component: HintComponent::Rationale
            })
        ));
    }

    #[test]
    fn supports_origin() {
        let origin = HintOrigin::new(
            "compiler.analysis",
            Some("analysis-42".to_owned()),
        )
        .expect("valid origin");

        let hint = Hint::new(
            key("execution", "locality"),
            "local",
        )
        .with_origin(origin);

        assert_eq!(
            hint.origin()
                .and_then(HintOrigin::identifier),
            Some("analysis-42")
        );
    }

    #[test]
    fn supports_deterministic_metadata() {
        let hint = Hint::new(
            key("execution", "locality"),
            "local",
        )
        .with_metadata("z", "last")
        .expect("valid metadata")
        .with_metadata("a", "first")
        .expect("valid metadata");

        let keys: Vec<&str> = hint
            .metadata()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(keys, vec!["a", "z"]);
    }

    #[test]
    fn metadata_replacement_is_explicit() {
        let mut hint = Hint::new(
            key("execution", "locality"),
            "local",
        );

        assert_eq!(
            hint.insert_metadata("mode", "initial")
                .expect("valid metadata"),
            None
        );

        assert_eq!(
            hint.insert_metadata("mode", "updated")
                .expect("valid metadata"),
            Some("initial".to_owned())
        );

        assert_eq!(
            hint.metadata_value("mode"),
            Some("updated")
        );
    }

    #[test]
    fn preserves_duplicate_declarations() {
        let identity = key(
            "execution",
            "parallelism",
        );

        let mut hints = HintSet::new();

        hints.push(Hint::new(
            identity.clone(),
            "adaptive",
        ));

        hints.push(Hint::new(
            identity,
            "local",
        ));

        assert_eq!(hints.len(), 2);
        assert_eq!(hints.duplicate_groups(), vec![vec![0, 1]]);
    }

    #[test]
    fn preserves_insertion_order() {
        let mut hints = HintSet::new();

        hints.push(Hint::new(
            key("execution", "locality"),
            "local",
        ));

        hints.push(Hint::new(
            key("execution", "parallelism"),
            "adaptive",
        ));

        let names: Vec<&str> = hints
            .iter()
            .map(|hint| hint.key().name())
            .collect();

        assert_eq!(
            names,
            vec!["locality", "parallelism"]
        );
    }

    #[test]
    fn canonicalized_view_is_deterministic() {
        let mut hints = HintSet::new();

        hints.push(Hint::new(
            key("execution", "z"),
            "last",
        ));

        hints.push(Hint::new(
            key("execution", "a"),
            "first",
        ));

        let ordered = hints.canonicalized();

        assert_eq!(
            ordered[0].key().name(),
            "a"
        );

        assert_eq!(
            ordered[1].key().name(),
            "z"
        );

        // Canonicalization must not mutate insertion order.
        assert_eq!(
            hints.iter().next().map(|hint| hint.key().name()),
            Some("z")
        );
    }

    #[test]
    fn retrieves_duplicate_occurrences() {
        let identity = key("execution", "locality");

        let mut hints = HintSet::new();

        hints.push(Hint::new(
            identity.clone(),
            "local",
        ));

        hints.push(Hint::new(
            identity.clone(),
            "regional",
        ));

        assert_eq!(
            hints.get(&identity, 0).map(Hint::payload),
            Some(&"local")
        );

        assert_eq!(
            hints.get(&identity, 1).map(Hint::payload),
            Some(&"regional")
        );

        assert!(hints.get(&identity, 2).is_none());
    }

    #[test]
    fn removes_all_matching_declarations() {
        let identity = key("execution", "locality");

        let mut hints = HintSet::new();

        hints.push(Hint::new(
            identity.clone(),
            "local",
        ));

        hints.push(Hint::new(
            key("execution", "parallelism"),
            "adaptive",
        ));

        hints.push(Hint::new(
            identity.clone(),
            "regional",
        ));

        assert_eq!(hints.remove_all(&identity), 2);
        assert_eq!(hints.len(), 1);
    }

    #[test]
    fn counts_distinct_identities() {
        let mut hints = HintSet::new();

        hints.push(Hint::new(
            key("execution", "locality"),
            "local",
        ));

        hints.push(Hint::new(
            key("execution", "locality"),
            "regional",
        ));

        hints.push(Hint::new(
            key("execution", "parallelism"),
            "adaptive",
        ));

        assert_eq!(hints.distinct_key_count(), 2);
    }

    #[test]
    fn maps_payload_without_losing_metadata() {
        let hint = Hint::new(
            key("execution", "locality"),
            10_u64,
        )
        .with_rationale("Advisory locality")
        .expect("valid rationale")
        .with_metadata("source", "analysis")
        .expect("valid metadata");

        let mapped = hint.map_payload(|value| value.to_string());

        assert_eq!(mapped.payload(), "10");

        assert_eq!(
            mapped.rationale(),
            Some("Advisory locality")
        );

        assert_eq!(
            mapped.metadata_value("source"),
            Some("analysis")
        );
    }

    #[test]
    fn fallible_collection_capacity_works() {
        let hints = HintSet::<String>::try_with_capacity(0)
            .expect("zero capacity is valid");

        assert!(hints.is_empty());
    }

    #[test]
    fn collection_default_is_empty() {
        let hints = HintSet::<String>::default();

        assert!(hints.is_empty());
    }

    #[test]
    fn into_iterator_preserves_order() {
        let mut hints = HintSet::new();

        hints.push(Hint::new(
            key("execution", "a"),
            1,
        ));

        hints.push(Hint::new(
            key("execution", "b"),
            2,
        ));

        let values: Vec<i32> = hints
            .into_iter()
            .map(|hint| *hint.payload())
            .collect();

        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn advisory_payload_does_not_create_semantic_authority() {
        // The payload is opaque to this module.
        // This test documents that construction does not evaluate
        // or enforce the supplied guidance.

        let hint = Hint::new(
            key("hardware", "accelerator"),
            "prefer-accelerator",
        );

        assert_eq!(
            hint.payload(),
            &"prefer-accelerator"
        );
    }
}
