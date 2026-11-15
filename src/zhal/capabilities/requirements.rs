
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! File: `src/zhal/capabilities/requirements.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the hardware-facing representation of capability requirements
//! consumed by ZHAL's hardware capability and compatibility boundaries.
//!
//! This module connects target-independent semantic intent to physical
//! hardware capability observations without becoming another source-language
//! semantic model.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Hardware-facing capability requirement declarations.
//! - Requirement attribution and provenance references.
//! - Requirement rationale.
//! - Ordered collections of hardware capability requirements.
//! - Deterministic requirement views.
//! - Duplicate declaration reporting.
//! - Explicit conversion boundaries from canonical capability identities.
//!
//! This module does NOT own:
//!
//! - Source-language grammar or parsing.
//! - AST nodes or source locations.
//! - Semantic name resolution.
//! - Canonical semantic capability requirements.
//! - Capability definitions.
//! - Capability identity.
//! - Physical hardware capability evidence.
//! - Hardware discovery.
//! - Provider registration.
//! - Capability availability.
//! - Capability compatibility evaluation.
//! - Feasibility analysis.
//! - Resource quantities or resource expressions.
//! - Resource allocation.
//! - Device selection.
//! - Provider selection.
//! - Hardware reservation.
//! - Scheduling.
//! - Routing.
//! - Execution.
//! - Quantum operations.
//! - HDL semantics.
//! - Compiler IR.
//!
//! ## Canonical authorities
//!
//! Standard capability identity:
//!     crate::stdlib::capability::CapabilityId
//!
//! Canonical source-level capability requirements:
//!     crate::frontend::semantic::capabilities::model
//!
//! Standard requirement infrastructure:
//!     crate::stdlib::requirement
//!
//! Standard preferences:
//!     crate::stdlib::preference
//!
//! Standard constraints:
//!     crate::stdlib::constraint
//!
//! Standard hints:
//!     crate::stdlib::hint
//!
//! Standard feasibility:
//!     crate::stdlib::feasibility
//!
//! Physical hardware capability evidence:
//!     crate::zhal::capabilities::evidence
//!
//! Hardware availability:
//!     crate::zhal::capabilities::availability
//!
//! Hardware compatibility:
//!     crate::zhal::capabilities::compatibility
//!
//! Runtime resource allocation:
//!     crate::zur
//!
//! ## Fundamental distinctions
//!
//! Requirement:
//!     A mandatory condition that a valid hardware realization must satisfy.
//!
//! Capability:
//!     An ability an implementation or physical environment can provide.
//!
//! Preference:
//!     A desirable property that is not mandatory.
//!
//! Constraint:
//!     A mandatory restriction on an otherwise valid realization.
//!
//! Hint:
//!     Non-binding implementation guidance.
//!
//! Feasibility:
//!     An evidence-based assessment of whether mandatory conditions can
//!     currently be satisfied.
//!
//! These concepts must never be silently converted into one another.
//!
//! ## POCO-REAF
//!
//! Hardware capability identities remain open-world.
//!
//! This module does not enumerate:
//!
//! - CPU architectures.
//! - GPU architectures.
//! - Quantum processor types.
//! - FPGA families.
//! - ASIC implementations.
//! - Accelerators.
//! - Distributed execution technologies.
//! - Embedded platforms.
//! - Future computational technologies.
//!
//! There are no arbitrary maximums on:
//!
//! - Requirement count.
//! - Capability count.
//! - Namespace depth.
//! - Identifier length.
//! - Provenance reference length.
//! - Requirement collection size.
//! - Computational domains.
//! - Hardware categories.
//!
//! Actual memory and processing requirements remain finite and depend on
//! available resources and explicit operational budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Important integration rule
//!
//! A semantic capability requirement must be translated into this
//! hardware-facing representation by an explicit downstream adapter.
//!
//! The adapter must preserve capability identity and provenance.
//!
//! This module must never inspect source syntax or query hardware while
//! constructing a requirement.
//!
//! A requirement is not proof that a capability exists.
//!
//! A requirement is not proof that a capability is available.
//!
//! A requirement is not a resource reservation.
//!
//! A requirement is not a feasibility result.

use core::fmt;

use std::collections::BTreeMap;

use crate::stdlib::capability::CapabilityId;

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable identifier of the ZHAL hardware requirement component.
pub const COMPONENT: &str = "zhal.capabilities.requirements";

/// Version of the hardware requirement representation schema.
///
/// This is independent of the Zamani language version, Cargo package
/// version, ZHAL API version, provider API version and hardware firmware.
pub const REQUIREMENTS_SCHEMA_VERSION: &str = "1";

// ============================================================================
// REQUIREMENT VALIDATION ERRORS
// ============================================================================

/// Identifies the field that failed requirement validation.
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
#[non_exhaustive]
pub enum RequirementField {
    /// The descriptive requirement rationale.
    Rationale,

    /// The requirement origin.
    Origin,

    /// The optional external provenance reference.
    Reference,
}

impl fmt::Display for RequirementField {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Rationale => formatter.write_str("rationale"),
            Self::Origin => formatter.write_str("origin"),
            Self::Reference => formatter.write_str("reference"),
        }
    }
}

/// Errors encountered while constructing hardware requirements.
///
/// These errors concern malformed local data only.
///
/// They do not indicate unsupported capabilities, unavailable hardware,
/// incompatibility, infeasibility or authorization failure.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RequirementError {
    /// A required descriptive field was empty or whitespace-only.
    EmptyField {
        /// The field that failed validation.
        field: RequirementField,
    },

    /// A descriptive field contained leading or trailing whitespace.
    SurroundingWhitespace {
        /// The field that failed validation.
        field: RequirementField,
    },

    /// A descriptive field contained a control character.
    ControlCharacter {
        /// The field that failed validation.
        field: RequirementField,
    },
}

impl fmt::Display for RequirementError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "hardware requirement {field} must not be empty"
                )
            }

            Self::SurroundingWhitespace { field } => {
                write!(
                    formatter,
                    "hardware requirement {field} must not have \
                     leading or trailing whitespace"
                )
            }

            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "hardware requirement {field} must not contain \
                     control characters"
                )
            }
        }
    }
}

impl std::error::Error for RequirementError {}

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates an externally supplied descriptive reference.
///
/// References are open-world strings.
///
/// No arbitrary maximum length is imposed.
///
/// Leading/trailing whitespace and control characters are rejected
/// rather than silently normalized.
fn validate_text(
    value: &str,
    field: RequirementField,
) -> Result<(), RequirementError> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(RequirementError::EmptyField { field });
    }

    if value.trim() != value {
        return Err(
            RequirementError::SurroundingWhitespace { field },
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            RequirementError::ControlCharacter { field },
        );
    }

    Ok(())
}

// ============================================================================
// REQUIREMENT ORIGIN
// ============================================================================

/// Describes where a hardware capability requirement originated.
///
/// The origin is an open-world identifier.
///
/// Examples:
///
/// - `frontend.semantic`
/// - `classical.semantic`
/// - `quantum.semantic`
/// - `hdl.semantic`
/// - `ai.semantic`
/// - `distributed.semantic`
/// - `runtime.contract`
/// - `provider.contract`
/// - `external.integration`
/// - `future.domain`
///
/// These are examples, not a closed enumeration.
///
/// Origin attribution is descriptive metadata.
///
/// It does not authenticate the origin, resolve source names or establish
/// the authority of a requirement.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct RequirementOrigin {
    identifier: String,
    reference: Option<String>,
}

impl RequirementOrigin {
    /// Constructs a validated requirement origin.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier or optional reference is invalid.
    pub fn new(
        identifier: impl Into<String>,
        reference: Option<String>,
    ) -> Result<Self, RequirementError> {
        let identifier = identifier.into();

        validate_text(
            &identifier,
            RequirementField::Origin,
        )?;

        if let Some(value) = reference.as_deref() {
            validate_text(
                value,
                RequirementField::Reference,
            )?;
        }

        Ok(Self {
            identifier,
            reference,
        })
    }

    /// Returns the exact origin identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the optional external reference.
    #[must_use]
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }

    /// Revalidates this origin.
    ///
    /// This method is useful when an origin is reconstructed from an
    /// external representation.
    pub fn validate(&self) -> Result<(), RequirementError> {
        validate_text(
            &self.identifier,
            RequirementField::Origin,
        )?;

        if let Some(reference) = self.reference.as_deref() {
            validate_text(
                reference,
                RequirementField::Reference,
            )?;
        }

        Ok(())
    }
}

impl fmt::Display for RequirementOrigin {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.identifier)
    }
}

// ============================================================================
// HARDWARE CAPABILITY REQUIREMENT
// ============================================================================

/// A mandatory hardware-facing capability requirement.
///
/// This type represents the physical hardware-facing projection of a
/// canonical capability requirement.
///
/// It reuses the standard-library capability identity.
///
/// It does not redefine semantic capability identity, capability versions,
/// typed semantic properties, source-language requirements or resource
/// expressions.
///
/// A requirement may originate from classical, quantum, HDL, AI,
/// distributed, hybrid, embedded or future computational domains.
///
/// The identity remains independent of a particular physical device,
/// vendor, provider, backend, machine or execution location.
///
/// A requirement does not establish capability availability.
///
/// A requirement does not reserve a physical resource.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
)]
pub struct HardwareCapabilityRequirement {
    capability: CapabilityId,

    origin: Option<RequirementOrigin>,

    rationale: Option<String>,
}

impl HardwareCapabilityRequirement {
    /// Constructs a mandatory hardware capability requirement.
    ///
    /// The supplied capability identity must already be validated by
    /// the canonical standard-library capability constructor.
    #[must_use]
    pub fn new(
        capability: CapabilityId,
    ) -> Self {
        Self {
            capability,
            origin: None,
            rationale: None,
        }
    }

    /// Returns the canonical capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Returns the optional requirement origin.
    #[must_use]
    pub fn origin(&self) -> Option<&RequirementOrigin> {
        self.origin.as_ref()
    }

    /// Returns the optional human-readable rationale.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }

    /// Associates an explicitly identified requirement origin.
    ///
    /// Origin attribution does not authenticate the source.
    #[must_use]
    pub fn with_origin(
        mut self,
        origin: RequirementOrigin,
    ) -> Self {
        self.origin = Some(origin);
        self
    }

    /// Adds a validated explanatory rationale.
    ///
    /// Rationale is descriptive metadata.
    ///
    /// It cannot change a requirement into a preference, constraint,
    /// hint, capability declaration or feasibility result.
    ///
    /// # Errors
    ///
    /// Returns an error when the rationale is empty, has surrounding
    /// whitespace or contains control characters.
    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        let rationale = rationale.into();

        validate_text(
            &rationale,
            RequirementField::Rationale,
        )?;

        self.rationale = Some(rationale);

        Ok(self)
    }

    /// Revalidates all locally owned requirement fields.
    ///
    /// Capability identity validation is delegated to the canonical
    /// standard-library constructor and deserialization boundary.
    pub fn validate(&self) -> Result<(), RequirementError> {
        if let Some(origin) = self.origin.as_ref() {
            origin.validate()?;
        }

        if let Some(rationale) = self.rationale.as_deref() {
            validate_text(
                rationale,
                RequirementField::Rationale,
            )?;
        }

        Ok(())
    }

    /// Returns the canonical identity used for requirement grouping.
    ///
    /// Rationale and provenance do not change requirement identity.
    #[must_use]
    pub fn canonical_key(&self) -> &CapabilityId {
        &self.capability
    }
}

impl fmt::Display for HardwareCapabilityRequirement {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "requires capability {}",
            self.capability
        )
    }
}

// ============================================================================
// REQUIREMENT COLLECTION
// ============================================================================

/// An ordered collection of hardware capability requirements.
///
/// This collection preserves:
///
/// - Insertion order.
/// - Duplicate declarations.
/// - Independent requirement origins.
/// - Independent rationales.
/// - Potentially contradictory declarations.
///
/// Duplicates are not automatically removed because independent sources
/// may express the same capability requirement for different reasons.
///
/// This collection is not:
///
/// - A semantic analyzer.
/// - A capability registry.
/// - A capability discovery mechanism.
/// - A compatibility solver.
/// - A feasibility engine.
/// - A resource allocator.
/// - A hardware scheduler.
///
/// No fixed collection capacity is imposed.
///
/// Growth is determined by available memory and explicit operational
/// budgets outside this type.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
)]
pub struct HardwareCapabilityRequirementSet {
    requirements: Vec<HardwareCapabilityRequirement>,
}

impl HardwareCapabilityRequirementSet {
    /// Creates an empty requirement collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
        }
    }

    /// Constructs a collection from an iterator.
    ///
    /// Iterator order and duplicate declarations are preserved.
    #[must_use]
    pub fn from_requirements(
        requirements: impl IntoIterator<Item = HardwareCapabilityRequirement>,
    ) -> Self {
        Self {
            requirements: requirements.into_iter().collect(),
        }
    }

    /// Adds one hardware capability requirement.
    ///
    /// The requirement is appended without deduplication.
    pub fn push(
        &mut self,
        requirement: HardwareCapabilityRequirement,
    ) {
        self.requirements.push(requirement);
    }

    /// Appends a requirement and returns the updated collection.
    #[must_use]
    pub fn with(
        mut self,
        requirement: HardwareCapabilityRequirement,
    ) -> Self {
        self.push(requirement);
        self
    }

    /// Appends another requirement collection.
    ///
    /// Original order and duplicate declarations are preserved.
    pub fn extend(
        &mut self,
        other: Self,
    ) {
        self.requirements.extend(other.requirements);
    }

    /// Appends requirements from an iterator.
    ///
    /// Existing entries are not deduplicated or reordered.
    pub fn extend_from(
        &mut self,
        requirements: impl IntoIterator<Item = HardwareCapabilityRequirement>,
    ) {
        self.requirements.extend(requirements);
    }

    /// Returns the number of declarations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns requirements in insertion order.
    pub fn iter(
        &self,
    ) -> core::slice::Iter<'_, HardwareCapabilityRequirement> {
        self.requirements.iter()
    }

    /// Returns a requirement by insertion-order index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&HardwareCapabilityRequirement> {
        self.requirements.get(index)
    }

    /// Returns all requirements for a canonical capability identity.
    ///
    /// The iterator preserves the original insertion order.
    pub fn for_capability<'a>(
        &'a self,
        capability: &'a CapabilityId,
    ) -> impl Iterator<Item = &'a HardwareCapabilityRequirement> + 'a {
        self.requirements
            .iter()
            .filter(move |requirement| {
                requirement.capability() == capability
            })
    }

    /// Returns all requirements originating from a specified origin.
    ///
    /// The iterator preserves insertion order.
    pub fn for_origin<'a>(
        &'a self,
        origin: &'a RequirementOrigin,
    ) -> impl Iterator<Item = &'a HardwareCapabilityRequirement> + 'a {
        self.requirements
            .iter()
            .filter(move |requirement| {
                requirement.origin() == Some(origin)
            })
    }

    /// Checks whether a capability requirement is present.
    ///
    /// This establishes declaration presence only.
    ///
    /// It does not establish that a physical device provides the
    /// capability or that the requirement is feasible.
    #[must_use]
    pub fn contains(
        &self,
        capability: &CapabilityId,
    ) -> bool {
        self.requirements
            .iter()
            .any(|requirement| {
                requirement.capability() == capability
            })
    }

    /// Validates all stored requirement declarations.
    ///
    /// The first invalid declaration is returned.
    ///
    /// No mutation occurs.
    pub fn validate(&self) -> Result<(), RequirementError> {
        for requirement in &self.requirements {
            requirement.validate()?;
        }

        Ok(())
    }

    /// Returns a deterministic ordering of the requirements.
    ///
    /// The original collection is not modified.
    ///
    /// Duplicate declarations remain present.
    ///
    /// Ordering is determined by:
    ///
    /// 1. Capability identity.
    /// 2. Origin.
    /// 3. Rationale.
    #[must_use]
    pub fn canonicalized(
        &self,
    ) -> Vec<&HardwareCapabilityRequirement> {
        let mut requirements: Vec<
            &HardwareCapabilityRequirement
        > = self.requirements.iter().collect();

        requirements.sort_by(|left, right| {
            left.capability()
                .cmp(right.capability())
                .then_with(|| left.origin().cmp(&right.origin()))
                .then_with(|| left.rationale().cmp(&right.rationale()))
        });

        requirements
    }

    /// Reports groups of duplicate capability requirement identities.
    ///
    /// Returned indices refer to the original insertion order.
    ///
    /// Duplicate declarations are preserved.
    ///
    /// This method does not establish that duplicate requirements are
    /// redundant or contradictory.
    #[must_use]
    pub fn duplicate_groups(
        &self,
    ) -> Vec<(CapabilityId, Vec<usize>)> {
        let mut groups: BTreeMap<
            CapabilityId,
            Vec<usize>,
        > = BTreeMap::new();

        for (index, requirement) in
            self.requirements.iter().enumerate()
        {
            groups
                .entry(requirement.capability().clone())
                .or_default()
                .push(index);
        }

        groups
            .into_iter()
            .filter(|(_, indices)| indices.len() > 1)
            .collect()
    }

    /// Consumes the collection and returns its requirements in
    /// insertion order.
    #[must_use]
    pub fn into_vec(
        self,
    ) -> Vec<HardwareCapabilityRequirement> {
        self.requirements
    }

    /// Removes all requirements explicitly.
    ///
    /// This operation affects only this collection.
    pub fn clear(&mut self) {
        self.requirements.clear();
    }
}

impl IntoIterator for HardwareCapabilityRequirementSet {
    type Item = HardwareCapabilityRequirement;

    type IntoIter =
        std::vec::IntoIter<HardwareCapabilityRequirement>;

    fn into_iter(self) -> Self::IntoIter {
        self.requirements.into_iter()
    }
}

impl<'a> IntoIterator for &'a HardwareCapabilityRequirementSet {
    type Item = &'a HardwareCapabilityRequirement;

    type IntoIter =
        core::slice::Iter<'a, HardwareCapabilityRequirement>;

    fn into_iter(self) -> Self::IntoIter {
        self.requirements.iter()
    }
}

// ============================================================================
// EXPLICIT REQUIREMENT CONSTRUCTION
// ============================================================================

/// Constructs a hardware capability requirement from an existing
/// canonical standard-library capability identity.
///
/// This is intentionally a small adapter boundary.
///
/// Source-language name resolution and semantic capability conversion
/// must already have happened before this function is called.
#[must_use]
pub fn from_capability(
    capability: CapabilityId,
) -> HardwareCapabilityRequirement {
    HardwareCapabilityRequirement::new(capability)
}

/// Constructs a requirement collection from canonical capability
/// identities.
///
/// Input ordering and duplicates are preserved.
#[must_use]
pub fn from_capabilities(
    capabilities: impl IntoIterator<Item = CapabilityId>,
) -> HardwareCapabilityRequirementSet {
    HardwareCapabilityRequirementSet::from_requirements(
        capabilities
            .into_iter()
            .map(HardwareCapabilityRequirement::new),
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn capability(name: &str) -> CapabilityId {
        CapabilityId::new(name.to_owned())
            .expect("test capability identity must be valid")
    }

    fn origin(name: &str) -> RequirementOrigin {
        RequirementOrigin::new(
            name.to_owned(),
            None,
        )
        .expect("test origin must be valid")
    }

    #[test]
    fn creates_hardware_capability_requirement() {
        let identity = capability("quantum.dynamic_control");

        let requirement =
            HardwareCapabilityRequirement::new(
                identity.clone(),
            );

        assert_eq!(
            requirement.capability(),
            &identity,
        );

        assert!(requirement.origin().is_none());
        assert!(requirement.rationale().is_none());
    }

    #[test]
    fn preserves_open_world_capability_names() {
        let identity =
            capability("future.photonic.computation");

        let requirement =
            HardwareCapabilityRequirement::new(identity.clone());

        assert_eq!(
            requirement.capability(),
            &identity,
        );
    }

    #[test]
    fn preserves_requirement_origin() {
        let requirement =
            HardwareCapabilityRequirement::new(
                capability("classical.parallel_execution"),
            )
            .with_origin(origin("classical.semantic"));

        assert_eq!(
            requirement
                .origin()
                .map(RequirementOrigin::identifier),
            Some("classical.semantic"),
        );
    }

    #[test]
    fn preserves_requirement_rationale() {
        let requirement =
            HardwareCapabilityRequirement::new(
                capability("accelerator.tensor_execution"),
            )
            .with_rationale(
                "The workload requires tensor execution",
            )
            .expect("rationale must be valid");

        assert_eq!(
            requirement.rationale(),
            Some("The workload requires tensor execution"),
        );
    }

    #[test]
    fn rejects_empty_rationale() {
        let result =
            HardwareCapabilityRequirement::new(
                capability("classical.compute"),
            )
            .with_rationale("   ");

        assert!(matches!(
            result,
            Err(RequirementError::EmptyField {
                field: RequirementField::Rationale
            })
        ));
    }

    #[test]
    fn rejects_surrounding_whitespace() {
        let result = RequirementOrigin::new(
            " hardware.provider ",
            None,
        );

        assert!(matches!(
            result,
            Err(RequirementError::SurroundingWhitespace {
                field: RequirementField::Origin
            })
        ));
    }

    #[test]
    fn rejects_control_characters() {
        let result = RequirementOrigin::new(
            "hardware\nprovider",
            None,
        );

        assert!(matches!(
            result,
            Err(RequirementError::ControlCharacter {
                field: RequirementField::Origin
            })
        ));
    }

    #[test]
    fn requirement_collection_preserves_insertion_order() {
        let first = HardwareCapabilityRequirement::new(
            capability("quantum.compute"),
        );

        let second = HardwareCapabilityRequirement::new(
            capability("classical.compute"),
        );

        let set =
            HardwareCapabilityRequirementSet::new()
                .with(first)
                .with(second);

        assert_eq!(set.len(), 2);

        assert_eq!(
            set.get(0)
                .map(|value| value.capability().as_str()),
            Some("quantum.compute"),
        );

        assert_eq!(
            set.get(1)
                .map(|value| value.capability().as_str()),
            Some("classical.compute"),
        );
    }

    #[test]
    fn requirement_collection_preserves_duplicates() {
        let identity = capability("quantum.measurement");

        let set = from_capabilities([
            identity.clone(),
            identity.clone(),
        ]);

        assert_eq!(set.len(), 2);

        assert!(set.contains(&identity));

        assert_eq!(
            set.duplicate_groups().len(),
            1,
        );

        assert_eq!(
            set.duplicate_groups()[0].1,
            vec![0, 1],
        );
    }

    #[test]
    fn independent_origins_are_preserved() {
        let identity = capability("classical.compute");

        let first =
            HardwareCapabilityRequirement::new(identity.clone())
                .with_origin(origin("classical.semantic"));

        let second =
            HardwareCapabilityRequirement::new(identity)
                .with_origin(origin("distributed.semantic"));

        let set =
            HardwareCapabilityRequirementSet::from_requirements([
                first,
                second,
            ]);

        assert_eq!(set.len(), 2);

        assert_eq!(
            set.for_origin(&origin("classical.semantic"))
                .count(),
            1,
        );

        assert_eq!(
            set.for_origin(&origin("distributed.semantic"))
                .count(),
            1,
        );
    }

    #[test]
    fn capability_lookup_preserves_all_matching_declarations() {
        let identity = capability("hdl.synthesis");

        let set = from_capabilities([
            identity.clone(),
            capability("classical.compute"),
            identity.clone(),
        ]);

        assert_eq!(
            set.for_capability(&identity).count(),
            2,
        );
    }

    #[test]
    fn canonicalized_view_is_deterministic() {
        let set = from_capabilities([
            capability("quantum.compute"),
            capability("classical.compute"),
            capability("hdl.synthesis"),
        ]);

        let first = set.canonicalized();
        let second = set.canonicalized();

        assert_eq!(first, second);

        assert_eq!(
            first[0].capability().as_str(),
            "classical.compute",
        );

        assert_eq!(
            first[1].capability().as_str(),
            "hdl.synthesis",
        );

        assert_eq!(
            first[2].capability().as_str(),
            "quantum.compute",
        );
    }

    #[test]
    fn canonicalization_does_not_mutate_original_collection() {
        let set = from_capabilities([
            capability("quantum.compute"),
            capability("classical.compute"),
        ]);

        let original_first = set
            .get(0)
            .expect("first declaration must exist")
            .capability()
            .as_str()
            .to_owned();

        let _ = set.canonicalized();

        assert_eq!(
            set.get(0)
                .map(|value| value.capability().as_str()),
            Some(original_first.as_str()),
        );
    }

    #[test]
    fn validation_checks_all_requirement_metadata() {
        let requirement =
            HardwareCapabilityRequirement::new(
                capability("quantum.compute"),
            )
            .with_origin(origin("quantum.semantic"))
            .with_rationale("Quantum execution required")
            .expect("rationale must be valid");

        assert!(requirement.validate().is_ok());
    }

    #[test]
    fn collection_validation_succeeds_for_valid_requirements() {
        let set = from_capabilities([
            capability("classical.compute"),
            capability("quantum.compute"),
            capability("hdl.synthesis"),
            capability("future.photonic.compute"),
        ]);

        assert!(set.validate().is_ok());
    }

    #[test]
    fn empty_collection_is_valid() {
        let set = HardwareCapabilityRequirementSet::new();

        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
        assert!(set.validate().is_ok());
    }

    #[test]
    fn extend_preserves_order_and_duplicates() {
        let mut first = from_capabilities([
            capability("classical.compute"),
        ]);

        let second = from_capabilities([
            capability("quantum.compute"),
            capability("classical.compute"),
        ]);

        first.extend(second);

        assert_eq!(first.len(), 3);

        assert_eq!(
            first
                .iter()
                .map(|requirement| requirement.capability().as_str())
                .collect::<Vec<_>>(),
            vec![
                "classical.compute",
                "quantum.compute",
                "classical.compute",
            ],
        );
    }

    #[test]
    fn clear_only_changes_the_owned_collection() {
        let mut set = from_capabilities([
            capability("classical.compute"),
            capability("quantum.compute"),
        ]);

        set.clear();

        assert!(set.is_empty());
    }

    #[test]
    fn requirement_display_is_stable() {
        let requirement =
            HardwareCapabilityRequirement::new(
                capability("accelerator.tensor"),
            );

        assert_eq!(
            requirement.to_string(),
            "requires capability accelerator.tensor",
        );
    }
}
