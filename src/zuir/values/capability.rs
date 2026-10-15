
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Capability References and Requirements
//!
//! Canonical path: `src/zuir/values/capability.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//!
//! ## PURPOSE
//!
//! Represent target-independent capability references and explicit
//! capability requirements in the ZUIR value layer.
//!
//! A capability names an ability or facility. A requirement states what
//! a program or IR entity needs. Neither proves that a particular target
//! provides the capability, has sufficient resources, or authorizes its use.
//!
//! ## STATUS
//!
//! Proposed implementation. Freeze only after module integration,
//! compilation, and the required tests succeed.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/config.rs`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - `grammar/core/capabilities.g4`
//! - `grammar/declarations/resources.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## OWNS
//!
//! - Capability references at the ZUIR value boundary.
//! - Portable capability-version and version-constraint representations.
//! - Typed, extensible capability property values.
//! - Capability requirement records.
//! - Local structural validation and validation budgets.
//! - Stable errors for invalid local representations.
//!
//! ## DOES_NOT_OWN
//!
//! - Source syntax, parsing, or AST nodes.
//! - Name resolution or semantic capability registration.
//! - The frontend's canonical semantic capability model.
//! - Target capability inventories or hardware discovery.
//! - Authorization, trust, security policy, or runtime permissions.
//! - Physical resource allocation, feasibility, placement, or scheduling.
//! - Operation semantics, type compatibility, or IR verification.
//! - Capability execution or backend dispatch.
//!
//! ## INTEGRATION
//!
//! - `schema/types.rs` owns `QualifiedName`.
//! - Frontend semantic analysis converts resolved capability requirements
//!   into these ZUIR records through a dedicated adapter.
//! - IR entities or resource contracts own the association between a
//!   requirement and its program, module, function, or operation.
//! - Target feasibility analysis compares requirements against target
//!   evidence and available resources.
//! - Serialization validates decoded records before accepting them.
//! - `quantum::ir`, classical IR, HDL, and other domain adapters retain
//!   ownership of their domain-specific representation and conversion.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! There is no closed catalogue of capability names and no universal limit
//! on capability count, namespace depth, property count, or hardware size.
//! Names use the existing qualified-name contract. Collections are dynamic.
//! Optional budgets constrain validation work, not program meaning.
//!
//! All concrete implementations remain bounded by available resources and
//! finite representation limits. Exhaustion must be reported, never silently
//! reinterpreted as a missing capability or a semantic incompatibility.
//!
//! ## SEMANTIC PRESERVATION
//!
//! Unknown capability names may be represented and transported. They must
//! not be treated as provided or executable without a registered definition
//! and appropriate target evidence.
//!
//! This module validates local structure only. It does not claim that a
//! requirement is satisfiable, authorized, or supported by any target.

use crate::zuir::schema::types::{QualifiedName, SchemaTypeError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// =============================================================================
// Capability reference
// =============================================================================

/// A qualified, target-independent capability identity.
///
/// The qualified name is open-world. New capability names do not require
/// adding variants to this type or modifying the universal IR.
///
/// Construction validates the qualified name, but does not resolve it
/// against a capability registry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityRef {
    name: QualifiedName,
}

impl CapabilityRef {
    /// Construct a capability reference from a qualified name.
    pub fn new(name: QualifiedName) -> Result<Self, CapabilityValidationError> {
        name.validate()
            .map_err(CapabilityValidationError::InvalidName)?;

        Ok(Self { name })
    }

    /// Parse a canonical `::`-separated capability name.
    pub fn parse(name: &str) -> Result<Self, CapabilityValidationError> {
        let name = QualifiedName::parse(name)
            .map_err(CapabilityValidationError::InvalidName)?;

        Self::new(name)
    }

    /// Borrow the canonical qualified name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the canonical display form.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        self.name.canonical_string()
    }

    /// Revalidate the representation, including after deserialization.
    pub fn validate(&self) -> Result<(), CapabilityValidationError> {
        self.name
            .validate()
            .map_err(CapabilityValidationError::InvalidName)
    }

    /// Consume the reference and return its qualified name.
    #[must_use]
    pub fn into_name(self) -> QualifiedName {
        self.name
    }
}

impl fmt::Display for CapabilityRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

// =============================================================================
// Capability versions
// =============================================================================

/// A numeric capability version.
///
/// These components describe version compatibility, not hardware capacity.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
    Serialize, Deserialize,
)]
pub struct CapabilityVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl CapabilityVersion {
    /// Construct a version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
    }

    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for CapabilityVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A constraint on the version of a required capability.
///
/// A matching version is not proof that an implementation exists or that
/// the implementation is compatible with a particular target.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
    Serialize, Deserialize,
)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum CapabilityVersionConstraint {
    /// No version restriction.
    #[default]
    Any,

    /// Require exactly this version.
    Exact(CapabilityVersion),

    /// Require this version or a newer version.
    AtLeast(CapabilityVersion),

    /// Require this version or an older version.
    AtMost(CapabilityVersion),

    /// Require a version inside this inclusive interval.
    Between {
        minimum: CapabilityVersion,
        maximum: CapabilityVersion,
    },
}

impl CapabilityVersionConstraint {
    /// Construct a validated inclusive interval.
    pub fn between(
        minimum: CapabilityVersion,
        maximum: CapabilityVersion,
    ) -> Result<Self, CapabilityValidationError> {
        if minimum > maximum {
            return Err(CapabilityValidationError::InvalidVersionRange {
                minimum,
                maximum,
            });
        }

        Ok(Self::Between { minimum, maximum })
    }

    /// Determine whether a version satisfies this constraint.
    #[must_use]
    pub const fn matches(self, version: CapabilityVersion) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(required) => {
                version.major == required.major
                    && version.minor == required.minor
                    && version.patch == required.patch
            }
            Self::AtLeast(required) => version_at_least(version, required),
            Self::AtMost(required) => version_at_least(required, version),
            Self::Between { minimum, maximum } => {
                version_at_least(version, minimum)
                    && version_at_least(maximum, version)
            }
        }
    }

    /// Validate the constraint, including values obtained by deserialization.
    pub fn validate(self) -> Result<(), CapabilityValidationError> {
        if let Self::Between { minimum, maximum } = self {
            if minimum > maximum {
                return Err(CapabilityValidationError::InvalidVersionRange {
                    minimum,
                    maximum,
                });
            }
        }

        Ok(())
    }
}

const fn version_at_least(
    version: CapabilityVersion,
    required: CapabilityVersion,
) -> bool {
    version.major > required.major
        || (version.major == required.major
            && (version.minor > required.minor
                || (version.minor == required.minor
                    && version.patch >= required.patch)))
}

// =============================================================================
// Capability property values
// =============================================================================

/// A portable, typed capability-property value.
///
/// Domain-specific property meaning and property-schema validation belong
/// to the registered capability definition, not this generic representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum CapabilityValue {
    Boolean(bool),
    SignedInteger(i128),
    UnsignedInteger(u128),

    /// Canonical numeric text interpreted by the owning property schema.
    Number(String),

    /// Arbitrary text, without implicit numeric or Boolean conversion.
    Text(String),

    /// Ordered property values.
    Sequence(Vec<CapabilityValue>),

    /// Deterministically ordered object properties.
    Object(BTreeMap<String, CapabilityValue>),

    /// Unresolved expression or symbolic property value.
    Symbolic(String),
}

// =============================================================================
// Requirements
// =============================================================================

/// An explicit requirement for a named capability.
///
/// The requirement does not state that a target provides the capability.
/// Its attachment to a program, function, or operation is owned by that
/// enclosing IR entity or resource contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    /// Open-world capability identity.
    pub capability: CapabilityRef,

    /// Required capability version or version range.
    #[serde(default)]
    pub version: CapabilityVersionConstraint,

    /// Additional typed constraints defined by the capability's schema.
    #[serde(default)]
    pub properties: BTreeMap<String, CapabilityValue>,
}

impl CapabilityRequirement {
    /// Construct a requirement with no version restriction or properties.
    #[must_use]
    pub fn new(capability: CapabilityRef) -> Self {
        Self {
            capability,
            version: CapabilityVersionConstraint::Any,
            properties: BTreeMap::new(),
        }
    }

    /// Construct a requirement with a version constraint.
    pub fn with_version(
        capability: CapabilityRef,
        version: CapabilityVersionConstraint,
    ) -> Result<Self, CapabilityValidationError> {
        version.validate()?;

        Ok(Self {
            capability,
            version,
            properties: BTreeMap::new(),
        })
    }

    /// Insert a property, rejecting duplicate keys.
    ///
    /// The existing value is not overwritten if the key already exists.
    pub fn add_property(
        &mut self,
        name: impl Into<String>,
        value: CapabilityValue,
    ) -> Result<(), CapabilityValidationError> {
        let name = name.into();
        validate_property_name(&name)?;

        if self.properties.contains_key(&name) {
            return Err(CapabilityValidationError::DuplicateProperty { name });
        }

        self.properties.insert(name, value);
        Ok(())
    }

    /// Insert or replace a property explicitly.
    pub fn set_property(
        &mut self,
        name: impl Into<String>,
        value: CapabilityValue,
    ) -> Result<Option<CapabilityValue>, CapabilityValidationError> {
        let name = name.into();
        validate_property_name(&name)?;

        Ok(self.properties.insert(name, value))
    }

    /// Validate the requirement's local representation.
    pub fn validate(&self) -> Result<(), CapabilityValidationError> {
        self.validate_with(&CapabilityValidationBudget::unlimited())
    }

    /// Validate using an explicit operational budget.
    pub fn validate_with(
        &self,
        budget: &CapabilityValidationBudget,
    ) -> Result<(), CapabilityValidationError> {
        self.capability.validate()?;
        self.version.validate()?;

        let mut work = ValidationWork::new(budget);

        work.visit_node()?;
        work.visit_name(&self.capability.name)?;

        for (name, value) in &self.properties {
            validate_property_name(name)?;
            work.visit_text(name)?;

            // Iterative traversal avoids consuming the call stack for
            // deeply nested property values.
            let mut pending: Vec<&CapabilityValue> = vec![value];

            while let Some(current) = pending.pop() {
                work.visit_node()?;

                match current {
                    CapabilityValue::Boolean(_)
                    | CapabilityValue::SignedInteger(_)
                    | CapabilityValue::UnsignedInteger(_) => {}

                    CapabilityValue::Number(text)
                    | CapabilityValue::Symbolic(text) => {
                        if text.trim().is_empty() {
                            return Err(
                                CapabilityValidationError::EmptyPropertyValue,
                            );
                        }
                        work.visit_text(text)?;
                    }

                    CapabilityValue::Text(text) => {
                        work.visit_text(text)?;
                    }

                    CapabilityValue::Sequence(values) => {
                        for child in values.iter().rev() {
                            pending.push(child);
                        }
                    }

                    CapabilityValue::Object(fields) => {
                        // Reverse ordered iteration so traversal follows
                        // ascending key order when popped from the stack.
                        for (key, child) in fields.iter().rev() {
                            validate_property_name(key)?;
                            work.visit_text(key)?;
                            pending.push(child);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check whether a concrete version satisfies this requirement.
    #[must_use]
    pub const fn accepts_version(&self, version: CapabilityVersion) -> bool {
        self.version.matches(version)
    }
}

// =============================================================================
// Validation budgets
// =============================================================================

/// Optional limits on local capability-representation validation.
///
/// These limits are operational safeguards, not universal language limits.
/// `None` imposes no explicit budget for that dimension.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CapabilityValidationBudget {
    /// Maximum number of visited representation nodes.
    pub max_nodes: Option<usize>,

    /// Maximum cumulative number of visited UTF-8 text bytes.
    pub max_text_bytes: Option<usize>,
}

impl CapabilityValidationBudget {
    /// A budget with no explicit limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: None,
            max_text_bytes: None,
        }
    }

    /// A budget with explicit node and text-byte limits.
    #[must_use]
    pub const fn bounded(
        max_nodes: usize,
        max_text_bytes: usize,
    ) -> Self {
        Self {
            max_nodes: Some(max_nodes),
            max_text_bytes: Some(max_text_bytes),
        }
    }
}

struct ValidationWork<'a> {
    budget: &'a CapabilityValidationBudget,
    nodes: usize,
    text_bytes: usize,
}

impl<'a> ValidationWork<'a> {
    fn new(budget: &'a CapabilityValidationBudget) -> Self {
        Self {
            budget,
            nodes: 0,
            text_bytes: 0,
        }
    }

    fn visit_node(&mut self) -> Result<(), CapabilityValidationError> {
        self.nodes = self
            .nodes
            .checked_add(1)
            .ok_or(CapabilityValidationError::CounterOverflow)?;

        check_budget(
            self.nodes,
            self.budget.max_nodes,
            CapabilityBudgetKind::Nodes,
        )
    }

    fn visit_text(
        &mut self,
        text: &str,
    ) -> Result<(), CapabilityValidationError> {
        self.text_bytes = self
            .text_bytes
            .checked_add(text.len())
            .ok_or(CapabilityValidationError::CounterOverflow)?;

        check_budget(
            self.text_bytes,
            self.budget.max_text_bytes,
            CapabilityBudgetKind::TextBytes,
        )
    }

    fn visit_name(
        &mut self,
        name: &QualifiedName,
    ) -> Result<(), CapabilityValidationError> {
        for segment in name.segments() {
            self.visit_text(segment)?;
        }

        Ok(())
    }
}

fn check_budget(
    actual: usize,
    limit: Option<usize>,
    kind: CapabilityBudgetKind,
) -> Result<(), CapabilityValidationError> {
    if let Some(limit) = limit {
        if actual > limit {
            return Err(CapabilityValidationError::BudgetExceeded {
                kind,
                limit,
                actual,
            });
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityBudgetKind {
    Nodes,
    TextBytes,
}

// =============================================================================
// Local validation
// =============================================================================

fn validate_property_name(name: &str) -> Result<(), CapabilityValidationError> {
    if name.trim().is_empty() {
        return Err(CapabilityValidationError::EmptyPropertyName);
    }

    if name.chars().any(char::is_control) {
        return Err(CapabilityValidationError::ControlCharacter {
            component: "property name",
        });
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors for locally malformed capability representations.
///
/// These errors do not report target incompatibility, missing hardware,
/// authorization failure, or semantic property-schema violations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CapabilityValidationError {
    InvalidName(SchemaTypeError),

    InvalidVersionRange {
        minimum: CapabilityVersion,
        maximum: CapabilityVersion,
    },

    EmptyPropertyName,

    EmptyPropertyValue,

    ControlCharacter {
        component: &'static str,
    },

    DuplicateProperty {
        name: String,
    },

    BudgetExceeded {
        kind: CapabilityBudgetKind,
        limit: usize,
        actual: usize,
    },

    CounterOverflow,
}

impl fmt::Display for CapabilityValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(error) => {
                write!(f, "invalid capability name: {error}")
            }

            Self::InvalidVersionRange { minimum, maximum } => {
                write!(
                    f,
                    "invalid capability version interval: minimum {minimum} \
                     exceeds maximum {maximum}"
                )
            }

            Self::EmptyPropertyName => {
                f.write_str("capability property name must not be empty")
            }

            Self::EmptyPropertyValue => {
                f.write_str("capability numeric or symbolic value must not be empty")
            }

            Self::ControlCharacter { component } => {
                write!(
                    f,
                    "capability {component} must not contain control characters"
                )
            }

            Self::DuplicateProperty { name } => {
                write!(f, "duplicate capability property `{name}`")
            }

            Self::BudgetExceeded { kind, limit, actual } => {
                write!(
                    f,
                    "capability validation budget for {kind:?} exceeded: \
                     limit {limit}, actual {actual}"
                )
            }

            Self::CounterOverflow => {
                f.write_str("capability validation counter overflow")
            }
        }
    }
}

impl Error for CapabilityValidationError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn capability(name: &str) -> CapabilityRef {
        CapabilityRef::parse(name).expect("valid test capability")
    }

    #[test]
    fn accepts_open_world_qualified_names() {
        let reference = capability("future::distributed::collectives");

        assert_eq!(
            reference.canonical_name(),
            "future::distributed::collectives"
        );
    }

    #[test]
    fn rejects_empty_and_malformed_names() {
        assert!(CapabilityRef::parse("").is_err());
        assert!(CapabilityRef::parse("quantum::::measure").is_err());
    }

    #[test]
    fn validates_version_intervals() {
        let minimum = CapabilityVersion::new(1, 2, 0);
        let maximum = CapabilityVersion::new(2, 0, 0);

        let constraint =
            CapabilityVersionConstraint::between(minimum, maximum)
                .expect("valid interval");

        assert!(constraint.matches(CapabilityVersion::new(1, 5, 0)));
        assert!(constraint.matches(CapabilityVersion::new(2, 0, 0)));
        assert!(!constraint.matches(CapabilityVersion::new(2, 0, 1)));

        assert!(
            CapabilityVersionConstraint::between(maximum, minimum).is_err()
        );
    }

    #[test]
    fn rejects_duplicate_properties_without_overwriting() {
        let mut requirement =
            CapabilityRequirement::new(capability("compute::parallel"));

        requirement
            .add_property("mode", CapabilityValue::Text("safe".into()))
            .expect("first property");

        assert!(
            requirement
                .add_property("mode", CapabilityValue::Text("fast".into()))
                .is_err()
        );

        assert_eq!(
            requirement.properties.get("mode"),
            Some(&CapabilityValue::Text("safe".into()))
        );
    }

    #[test]
    fn property_maps_have_deterministic_key_order() {
        let mut requirement =
            CapabilityRequirement::new(capability("tensor::compute"));

        requirement
            .add_property("zeta", CapabilityValue::Boolean(true))
            .expect("property");

        requirement
            .add_property("alpha", CapabilityValue::Boolean(false))
            .expect("property");

        let keys: Vec<_> = requirement.properties.keys().map(String::as_str).collect();

        assert_eq!(keys, vec!["alpha", "zeta"]);
    }

    #[test]
    fn enforces_explicit_validation_budgets() {
        let requirement =
            CapabilityRequirement::new(capability("quantum::measurement"));

        let budget = CapabilityValidationBudget::bounded(0, usize::MAX);

        assert!(matches!(
            requirement.validate_with(&budget),
            Err(CapabilityValidationError::BudgetExceeded {
                kind: CapabilityBudgetKind::Nodes,
                ..
            })
        ));
    }

    #[test]
    fn validates_nested_properties_iteratively() {
        let mut value = CapabilityValue::Boolean(true);

        for _ in 0..128 {
            value = CapabilityValue::Sequence(vec![value]);
        }

        let mut requirement =
            CapabilityRequirement::new(capability("future::nested"));

        requirement
            .add_property("payload", value)
            .expect("property");

        requirement.validate().expect("valid nested property");
    }

    #[test]
    fn rejects_empty_symbolic_values() {
        let mut requirement =
            CapabilityRequirement::new(capability("compute::symbolic"));

        requirement
            .add_property("shape", CapabilityValue::Symbolic("   ".into()))
            .expect("property insertion checks the name, not expression semantics");

        assert!(matches!(
            requirement.validate(),
            Err(CapabilityValidationError::EmptyPropertyValue)
        ));
    }

    #[test]
    fn requirements_do_not_claim_target_support() {
        let requirement =
            CapabilityRequirement::new(capability("future::unimplemented"));

        // Structural validity is independent of whether a target provides it.
        assert!(requirement.validate().is_ok());
        assert!(requirement.accepts_version(CapabilityVersion::new(1, 0, 0)));
    }
}
