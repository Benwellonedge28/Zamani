
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — External Tool Requirements
//!
//! Path: `src/toolchain/external_tools/requirements.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the provider-neutral declarative requirements for external tools
//! used during Zamani project validation, compilation, testing, packaging,
//! verification, deployment, and other registered operations.
//!
//! ## Ownership
//!
//! This module owns:
//! - Stable external-tool requirement identifiers.
//! - Open computational-domain identifiers.
//! - Open capability identifiers.
//! - Required and optional requirement policies.
//! - Numeric tool-version constraints.
//! - Requirement metadata and rationale.
//! - Deterministic collections of tool requirements.
//! - Requirement-set validation and duplicate detection.
//!
//! This module does NOT own:
//! - Executable discovery or PATH lookup.
//! - Executable authorization or trust verification.
//! - Process creation or execution.
//! - Version probing.
//! - Process output capture or cancellation.
//! - Backend registration.
//! - Target selection or physical resource allocation.
//! - Compiler or runtime implementation.
//!
//! ## Integration ownership
//!
//! `backend::toolchain` owns:
//! - `ToolchainSpec`;
//! - `ToolVersion`;
//! - `VersionRequirement`;
//! - `ToolchainManager`;
//! - `ToolchainDescriptor`.
//!
//! `backend::process` owns process execution.
//!
//! The future `external_tools::resolution` module translates these
//! declarative requirements into backend tool specifications.
//!
//! `external_tools::validation` checks discovered tool descriptors against
//! the original requirements.
//!
//! `external_tools::reporting` reports resolution and validation outcomes.
//!
//! ## Portability
//!
//! A requirement describes a tool needed to realize an operation. It does
//! not prescribe a particular vendor, executable location, operating system,
//! processor, accelerator, or hardware implementation.
//!
//! ## Scalability
//!
//! No arbitrary maximum number of requirements, domains, capabilities,
//! projects, targets, or computational resources is imposed.
//!
//! Concrete memory, storage, processing, and execution budgets remain
//! responsibilities of the appropriate operational policies.
//!
//! ## Version semantics
//!
//! Numeric versions use major.minor.patch ordering. This is intentionally
//! compatible with the existing backend `ToolVersion` contract.
//! Vendor-specific version schemes must be interpreted by an appropriate
//! adapter rather than guessed by this module.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

/// A validated identifier used by an external-tool requirement.
///
/// Identifiers are open-ended rather than selected from a closed enum.
///
/// Canonical syntax:
///
/// - The first character must be a lowercase ASCII letter.
/// - Remaining characters may be lowercase ASCII letters, digits,
///   `.`, `_`, or `-`.
/// - Namespace segments separated by `.` must not be empty.
///
/// Examples:
///
/// - `llvm.clang`
/// - `llvm.linker`
/// - `hdl.synthesis`
/// - `quantum.simulator`
/// - `ai.model_compiler`
/// - `distributed.communication`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequirementId(String);

impl RequirementId {
    /// Construct an identifier after validating its canonical syntax.
    pub fn new(value: impl Into<String>) -> Result<Self, RequirementError> {
        let value = value.into();
        validate_identifier("requirement", &value)?;
        Ok(Self(value))
    }

    /// Return the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its canonical string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for RequirementId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An open-ended computational-domain identifier.
///
/// Domain identifiers describe the context in which a tool is needed.
/// They do not select a backend or establish hardware availability.
///
/// Examples include `classical`, `quantum`, `hdl`, `ai`, `hybrid`,
/// `distributed`, and future registered domains.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainId(String);

impl DomainId {
    /// Construct a validated domain identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, RequirementError> {
        let value = value.into();
        validate_identifier("domain", &value)?;
        Ok(Self(value))
    }

    /// Return the canonical domain identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DomainId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An open-ended computational capability identifier.
///
/// Capabilities describe what an external tool is expected to provide.
/// They are metadata for downstream capability validation, not proof that
/// a discovered executable actually implements the capability.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Construct a validated capability identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, RequirementError> {
        let value = value.into();
        validate_identifier("capability", &value)?;
        Ok(Self(value))
    }

    /// Return the canonical capability identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Numeric version used by declarative tool-version constraints.
///
/// This type deliberately does not interpret vendor-specific suffixes,
/// prerelease identifiers, or build metadata.
///
/// A provider with a different version scheme must supply an appropriate
/// adapter and must not silently map an unknown version to a numeric version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NumericVersion {
    /// Major version component.
    pub major: u64,

    /// Minor version component.
    pub minor: u64,

    /// Patch version component.
    pub patch: u64,
}

impl NumericVersion {
    /// Construct a numeric version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl fmt::Display for NumericVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

/// Inclusive version constraints for an external tool.
///
/// All configured constraints must hold.
///
/// An unconstrained requirement accepts any version that the downstream
/// provider can successfully identify.
///
/// This structure is independent of the backend's executable discovery
/// and version-probing implementation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VersionConstraint {
    minimum_inclusive: Option<NumericVersion>,
    maximum_inclusive: Option<NumericVersion>,
    exact: Option<NumericVersion>,
}

impl VersionConstraint {
    /// Accept any identifiable numeric version.
    #[must_use]
    pub const fn any() -> Self {
        Self {
            minimum_inclusive: None,
            maximum_inclusive: None,
            exact: None,
        }
    }

    /// Require a version greater than or equal to the supplied version.
    #[must_use]
    pub fn at_least(mut self, version: NumericVersion) -> Self {
        self.minimum_inclusive = Some(version);
        self
    }

    /// Require a version less than or equal to the supplied version.
    #[must_use]
    pub fn at_most(mut self, version: NumericVersion) -> Self {
        self.maximum_inclusive = Some(version);
        self
    }

    /// Require an exact numeric version.
    #[must_use]
    pub fn exactly(mut self, version: NumericVersion) -> Self {
        self.exact = Some(version);
        self
    }

    /// Return the minimum inclusive version, if configured.
    #[must_use]
    pub const fn minimum(&self) -> Option<NumericVersion> {
        self.minimum_inclusive
    }

    /// Return the maximum inclusive version, if configured.
    #[must_use]
    pub const fn maximum(&self) -> Option<NumericVersion> {
        self.maximum_inclusive
    }

    /// Return the exact version, if configured.
    #[must_use]
    pub const fn exact(&self) -> Option<NumericVersion> {
        self.exact
    }

    /// Validate that the configured constraints are mutually consistent.
    pub fn validate(&self) -> Result<(), RequirementError> {
        if let (Some(minimum), Some(maximum)) =
            (self.minimum_inclusive, self.maximum_inclusive)
        {
            if minimum > maximum {
                return Err(RequirementError::InvalidVersionConstraint {
                    reason: format!(
                        "minimum version {minimum} exceeds maximum version {maximum}"
                    ),
                });
            }
        }

        if let Some(exact) = self.exact {
            if self
                .minimum_inclusive
                .is_some_and(|minimum| exact < minimum)
            {
                return Err(RequirementError::InvalidVersionConstraint {
                    reason: format!(
                        "exact version {exact} is below the minimum version"
                    ),
                });
            }

            if self
                .maximum_inclusive
                .is_some_and(|maximum| exact > maximum)
            {
                return Err(RequirementError::InvalidVersionConstraint {
                    reason: format!(
                        "exact version {exact} exceeds the maximum version"
                    ),
                });
            }
        }

        Ok(())
    }

    /// Check whether an actual numeric version satisfies this constraint.
    pub fn matches(
        &self,
        actual: NumericVersion,
    ) -> Result<bool, RequirementError> {
        self.validate()?;

        if self.exact.is_some_and(|exact| actual != exact) {
            return Ok(false);
        }

        if self
            .minimum_inclusive
            .is_some_and(|minimum| actual < minimum)
        {
            return Ok(false);
        }

        if self
            .maximum_inclusive
            .is_some_and(|maximum| actual > maximum)
        {
            return Ok(false);
        }

        Ok(true)
    }
}

/// Whether a tool is mandatory or optional for an operation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum RequirementKind {
    /// The operation cannot proceed without a satisfying tool.
    #[default]
    Required,

    /// The tool is desirable or conditionally useful but is not mandatory.
    ///
    /// If an optional tool is discovered and selected, its configured
    /// version and capability constraints must still be satisfied.
    Optional,
}

/// Declarative requirement for one external tool.
///
/// This type does not identify an executable path. It identifies a logical
/// tool requirement that a registered provider must resolve.
///
/// `id` is a stable logical identity, not a command-line executable name.
///
/// One requirement can apply to multiple computational domains and can
/// require multiple capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalToolRequirement {
    id: RequirementId,
    kind: RequirementKind,
    version: VersionConstraint,
    domains: BTreeSet<DomainId>,
    capabilities: BTreeSet<CapabilityId>,
    purpose: Option<String>,
    origin: Option<String>,
}

impl ExternalToolRequirement {
    /// Create a new required tool declaration.
    pub fn new(
        id: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        Ok(Self {
            id: RequirementId::new(id)?,
            kind: RequirementKind::Required,
            version: VersionConstraint::any(),
            domains: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            purpose: None,
            origin: None,
        })
    }

    /// Set whether the requirement is mandatory or optional.
    #[must_use]
    pub fn kind(mut self, kind: RequirementKind) -> Self {
        self.kind = kind;
        self
    }

    /// Set the version constraints.
    #[must_use]
    pub fn version(mut self, constraint: VersionConstraint) -> Self {
        self.version = constraint;
        self
    }

    /// Add a computational domain.
    ///
    /// Domains are open-ended. Adding a new domain does not require modifying
    /// this module.
    #[must_use]
    pub fn domain(mut self, domain: DomainId) -> Self {
        self.domains.insert(domain);
        self
    }

    /// Add multiple computational domains.
    #[must_use]
    pub fn domains<I>(mut self, domains: I) -> Self
    where
        I: IntoIterator<Item = DomainId>,
    {
        self.domains.extend(domains);
        self
    }

    /// Add a required capability.
    #[must_use]
    pub fn capability(mut self, capability: CapabilityId) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Add multiple required capabilities.
    #[must_use]
    pub fn capabilities<I>(mut self, capabilities: I) -> Self
    where
        I: IntoIterator<Item = CapabilityId>,
    {
        self.capabilities.extend(capabilities);
        self
    }

    /// Attach a human-readable purpose.
    ///
    /// Empty or whitespace-only purposes are rejected.
    pub fn purpose(
        mut self,
        purpose: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        self.purpose = Some(validate_nonempty_text(
            "purpose",
            purpose.into(),
        )?);
        Ok(self)
    }

    /// Attach an origin reference, such as a manifest declaration,
    /// build operation, or registered task.
    ///
    /// This is descriptive provenance. It does not authorize filesystem
    /// access or executable invocation.
    pub fn origin(
        mut self,
        origin: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        self.origin = Some(validate_nonempty_text(
            "origin",
            origin.into(),
        )?);
        Ok(self)
    }

    /// Validate the complete declaration.
    pub fn validate(&self) -> Result<(), RequirementError> {
        self.version.validate()?;

        if let Some(purpose) = &self.purpose {
            validate_nonempty_text("purpose", purpose.clone())?;
        }

        if let Some(origin) = &self.origin {
            validate_nonempty_text("origin", origin.clone())?;
        }

        Ok(())
    }

    /// Return the logical requirement identifier.
    #[must_use]
    pub fn id(&self) -> &RequirementId {
        &self.id
    }

    /// Return the requirement kind.
    #[must_use]
    pub const fn requirement_kind(&self) -> RequirementKind {
        self.kind
    }

    /// Return the version constraint.
    #[must_use]
    pub const fn version_constraint(&self) -> &VersionConstraint {
        &self.version
    }

    /// Return the applicable domains in deterministic order.
    #[must_use]
    pub fn domains(&self) -> &BTreeSet<DomainId> {
        &self.domains
    }

    /// Return the required capabilities in deterministic order.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<CapabilityId> {
        &self.capabilities
    }

    /// Return the purpose, if supplied.
    #[must_use]
    pub fn purpose_text(&self) -> Option<&str> {
        self.purpose.as_deref()
    }

    /// Return the origin reference, if supplied.
    #[must_use]
    pub fn origin_reference(&self) -> Option<&str> {
        self.origin.as_deref()
    }

    /// Return whether the tool is mandatory.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        matches!(self.kind, RequirementKind::Required)
    }

    /// Return whether the tool is optional.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        matches!(self.kind, RequirementKind::Optional)
    }
}

/// A deterministic collection of external-tool requirements.
///
/// Requirements are indexed by stable logical identifiers.
///
/// Duplicate identifiers are rejected. A caller must explicitly resolve
/// conflicting declarations rather than silently replacing an existing
/// requirement.
///
/// The collection has no arbitrary capacity limit. Its practical size is
/// bounded by available resources and the underlying allocator.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExternalToolRequirements {
    requirements: BTreeMap<RequirementId, ExternalToolRequirement>,
}

impl ExternalToolRequirements {
    /// Construct an empty requirement collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a collection from declarations.
    ///
    /// Invalid declarations and duplicate identifiers cause an error.
    pub fn from_iter<I>(
        requirements: I,
    ) -> Result<Self, RequirementError>
    where
        I: IntoIterator<Item = ExternalToolRequirement>,
    {
        let mut collection = Self::new();

        for requirement in requirements {
            collection.insert(requirement)?;
        }

        Ok(collection)
    }

    /// Insert a requirement.
    ///
    /// Returns an error if the declaration is invalid or its identifier
    /// already exists.
    pub fn insert(
        &mut self,
        requirement: ExternalToolRequirement,
    ) -> Result<(), RequirementError> {
        requirement.validate()?;

        let id = requirement.id.clone();

        if self.requirements.contains_key(&id) {
            return Err(RequirementError::DuplicateRequirement {
                id: id.to_string(),
            });
        }

        self.requirements.insert(id, requirement);
        Ok(())
    }

    /// Merge another collection without replacing existing declarations.
    ///
    /// The operation is atomic with respect to the receiver: if any
    /// declaration conflicts or is invalid, the receiver remains unchanged.
    pub fn merge(
        &mut self,
        other: &ExternalToolRequirements,
    ) -> Result<(), RequirementError> {
        for requirement in other.iter() {
            requirement.validate()?;

            if self.requirements.contains_key(requirement.id()) {
                return Err(RequirementError::DuplicateRequirement {
                    id: requirement.id().to_string(),
                });
            }
        }

        self.requirements.extend(
            other
                .requirements
                .iter()
                .map(|(id, requirement)| (id.clone(), requirement.clone())),
        );

        Ok(())
    }

    /// Look up a declaration by its logical identifier.
    #[must_use]
    pub fn get(
        &self,
        id: &RequirementId,
    ) -> Option<&ExternalToolRequirement> {
        self.requirements.get(id)
    }

    /// Remove a declaration by its identifier.
    ///
    /// Returns the removed declaration when present.
    pub fn remove(
        &mut self,
        id: &RequirementId,
    ) -> Option<ExternalToolRequirement> {
        self.requirements.remove(id)
    }

    /// Iterate over declarations in deterministic identifier order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &ExternalToolRequirement> {
        self.requirements.values()
    }

    /// Iterate over mandatory declarations.
    pub fn required(
        &self,
    ) -> impl Iterator<Item = &ExternalToolRequirement> {
        self.iter().filter(|requirement| requirement.is_required())
    }

    /// Iterate over optional declarations.
    pub fn optional(
        &self,
    ) -> impl Iterator<Item = &ExternalToolRequirement> {
        self.iter().filter(|requirement| requirement.is_optional())
    }

    /// Return the number of declarations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Return whether the collection contains no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Remove all declarations.
    pub fn clear(&mut self) {
        self.requirements.clear();
    }

    /// Validate every declaration in the collection.
    pub fn validate(&self) -> Result<(), RequirementError> {
        for requirement in self.iter() {
            requirement.validate()?;
        }

        Ok(())
    }

    /// Return whether a requirement with the supplied identifier exists.
    #[must_use]
    pub fn contains(&self, id: &RequirementId) -> bool {
        self.requirements.contains_key(id)
    }

    /// Return the number of mandatory declarations.
    #[must_use]
    pub fn required_count(&self) -> usize {
        self.required().count()
    }

    /// Return the number of optional declarations.
    #[must_use]
    pub fn optional_count(&self) -> usize {
        self.optional().count()
    }
}

/// Errors encountered while constructing or validating requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequirementError {
    /// An identifier is empty or violates the canonical identifier grammar.
    InvalidIdentifier {
        /// Identifier category.
        category: &'static str,

        /// Supplied identifier.
        value: String,

        /// Reason for rejection.
        reason: &'static str,
    },

    /// A descriptive field is empty or contains only whitespace.
    EmptyText {
        /// Name of the descriptive field.
        field: &'static str,
    },

    /// Two declarations use the same logical requirement identifier.
    DuplicateRequirement {
        /// Conflicting identifier.
        id: String,
    },

    /// Version constraints contradict each other.
    InvalidVersionConstraint {
        /// Explanation of the contradiction.
        reason: String,
    },
}

impl fmt::Display for RequirementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier {
                category,
                value,
                reason,
            } => write!(
                formatter,
                "invalid {category} identifier `{value}`: {reason}"
            ),

            Self::EmptyText { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::DuplicateRequirement { id } => {
                write!(formatter, "duplicate external-tool requirement `{id}`")
            }

            Self::InvalidVersionConstraint { reason } => {
                write!(formatter, "invalid tool-version constraint: {reason}")
            }
        }
    }
}

impl Error for RequirementError {}

/// Validate the shared canonical identifier grammar.
fn validate_identifier(
    category: &'static str,
    value: &str,
) -> Result<(), RequirementError> {
    let invalid = |reason| RequirementError::InvalidIdentifier {
        category,
        value: value.to_owned(),
        reason,
    };

    if value.is_empty() {
        return Err(invalid("identifier must not be empty"));
    }

    if !value.is_ascii() {
        return Err(invalid("identifier must contain ASCII characters only"));
    }

    let mut previous_was_separator = true;

    for character in value.chars() {
        let is_letter = character.is_ascii_lowercase();
        let is_digit = character.is_ascii_digit();
        let is_separator = matches!(character, '.' | '_' | '-');

        if !(is_letter || is_digit || is_separator) {
            return Err(invalid(
                "only lowercase letters, digits, '.', '_' and '-' are allowed",
            ));
        }

        if is_separator && previous_was_separator {
            return Err(invalid(
                "identifiers must not start with or contain consecutive separators",
            ));
        }

        previous_was_separator = is_separator;
    }

    if previous_was_separator {
        return Err(invalid("identifier must not end with a separator"));
    }

    if !value
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_lowercase())
    {
        return Err(invalid(
            "identifier must start with a lowercase ASCII letter",
        ));
    }

    Ok(())
}

/// Validate descriptive text without silently normalizing user input.
fn validate_nonempty_text(
    field: &'static str,
    value: String,
) -> Result<String, RequirementError> {
    if value.trim().is_empty() {
        return Err(RequirementError::EmptyText { field });
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn requirement(id: &str) -> ExternalToolRequirement {
        ExternalToolRequirement::new(id)
            .expect("valid test requirement identifier")
    }

    fn domain(id: &str) -> DomainId {
        DomainId::new(id).expect("valid test domain")
    }

    fn capability(id: &str) -> CapabilityId {
        CapabilityId::new(id).expect("valid test capability")
    }

    #[test]
    fn accepts_extensible_requirement_identifiers() {
        assert_eq!(
            RequirementId::new("llvm.clang")
                .expect("valid identifier")
                .as_str(),
            "llvm.clang"
        );

        assert!(RequirementId::new("future_domain.compiler").is_ok());
        assert!(RequirementId::new("vendor_specific.tool-v2").is_ok());
    }

    #[test]
    fn rejects_invalid_identifiers() {
        assert!(RequirementId::new("").is_err());
        assert!(RequirementId::new("LLVM.clang").is_err());
        assert!(RequirementId::new("1llvm.clang").is_err());
        assert!(RequirementId::new("llvm..clang").is_err());
        assert!(RequirementId::new("llvm.clang.").is_err());
        assert!(RequirementId::new("llvm clang").is_err());
        assert!(RequirementId::new("llvm/clang").is_err());
    }

    #[test]
    fn validates_numeric_version_ranges() {
        let constraint = VersionConstraint::any()
            .at_least(NumericVersion::new(1, 2, 0))
            .at_most(NumericVersion::new(2, 0, 0));

        assert!(constraint.validate().is_ok());

        assert!(
            constraint
                .matches(NumericVersion::new(1, 5, 0))
                .expect("valid constraint")
        );

        assert!(
            !constraint
                .matches(NumericVersion::new(2, 1, 0))
                .expect("valid constraint")
        );
    }

    #[test]
    fn rejects_contradictory_version_ranges() {
        let constraint = VersionConstraint::any()
            .at_least(NumericVersion::new(3, 0, 0))
            .at_most(NumericVersion::new(2, 0, 0));

        assert!(matches!(
            constraint.validate(),
            Err(RequirementError::InvalidVersionConstraint { .. })
        ));
    }

    #[test]
    fn rejects_exact_version_outside_range() {
        let constraint = VersionConstraint::any()
            .at_least(NumericVersion::new(2, 0, 0))
            .exactly(NumericVersion::new(1, 0, 0));

        assert!(constraint.validate().is_err());
    }

    #[test]
    fn supports_multiple_domains_and_capabilities() {
        let requirement = requirement("portable.compiler")
            .domain(domain("classical"))
            .domain(domain("quantum"))
            .domain(domain("hdl"))
            .capability(capability("compilation"))
            .capability(capability("cross_target_lowering"));

        assert_eq!(requirement.domains().len(), 3);
        assert_eq!(requirement.capabilities().len(), 2);
    }

    #[test]
    fn required_is_the_default_policy() {
        let requirement = requirement("classical.compiler");

        assert!(requirement.is_required());
        assert!(!requirement.is_optional());
    }

    #[test]
    fn optional_requirement_is_explicit() {
        let requirement = requirement("optional.optimizer")
            .kind(RequirementKind::Optional);

        assert!(requirement.is_optional());
        assert!(!requirement.is_required());
    }

    #[test]
    fn rejects_empty_purpose_and_origin() {
        assert!(
            requirement("compiler")
                .purpose("   ")
                .is_err()
        );

        assert!(
            requirement("compiler")
                .origin("")
                .is_err()
        );
    }

    #[test]
    fn requirement_collection_rejects_duplicates() {
        let mut requirements = ExternalToolRequirements::new();

        requirements
            .insert(requirement("llvm.clang"))
            .expect("first insertion");

        let result = requirements.insert(requirement("llvm.clang"));

        assert!(matches!(
            result,
            Err(RequirementError::DuplicateRequirement { .. })
        ));

        assert_eq!(requirements.len(), 1);
    }

    #[test]
    fn collection_iteration_is_deterministic() {
        let requirements = ExternalToolRequirements::from_iter([
            requirement("zeta.compiler"),
            requirement("alpha.compiler"),
            requirement("middle.compiler"),
        ])
        .expect("unique requirements");

        let ids: Vec<&str> = requirements
            .iter()
            .map(|item| item.id().as_str())
            .collect();

        assert_eq!(
            ids,
            vec![
                "alpha.compiler",
                "middle.compiler",
                "zeta.compiler"
            ]
        );
    }

    #[test]
    fn merge_is_atomic_on_duplicate() {
        let mut first = ExternalToolRequirements::new();
        first
            .insert(requirement("llvm.clang"))
            .expect("first insertion");

        let second = ExternalToolRequirements::from_iter([
            requirement("llvm.clang"),
            requirement("hdl.synthesis"),
        ])
        .expect("second collection");

        assert!(first.merge(&second).is_err());

        assert_eq!(first.len(), 1);
        assert!(first.contains(
            &RequirementId::new("llvm.clang").expect("valid identifier")
        ));
        assert!(!first.contains(
            &RequirementId::new("hdl.synthesis").expect("valid identifier")
        ));
    }

    #[test]
    fn required_and_optional_iteration_is_correct() {
        let requirements = ExternalToolRequirements::from_iter([
            requirement("required.compiler"),
            requirement("optional.optimizer")
                .kind(RequirementKind::Optional),
        ])
        .expect("valid requirements");

        assert_eq!(requirements.required_count(), 1);
        assert_eq!(requirements.optional_count(), 1);
        assert_eq!(requirements.required().count(), 1);
        assert_eq!(requirements.optional().count(), 1);
    }

    #[test]
    fn removal_returns_the_original_requirement() {
        let mut requirements = ExternalToolRequirements::new();

        requirements
            .insert(requirement("llvm.clang"))
            .expect("insertion");

        let id = RequirementId::new("llvm.clang").expect("valid identifier");

        let removed = requirements.remove(&id);

        assert!(removed.is_some());
        assert!(requirements.is_empty());
    }

    #[test]
    fn supports_future_computational_domains() {
        let requirement = requirement("future.tool")
            .domain(domain("neuromorphic"))
            .domain(domain("photonic"))
            .domain(domain("molecular"))
            .capability(capability("future_computation"));

        assert_eq!(requirement.domains().len(), 3);
        assert_eq!(requirement.capabilities().len(), 1);
    }

    #[test]
    fn empty_collection_is_valid() {
        let requirements = ExternalToolRequirements::new();

        assert!(requirements.is_empty());
        assert!(requirements.validate().is_ok());
    }
}
