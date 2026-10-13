//! Zamani semantic capability model.
//!
//! File: src/frontend/semantic/capabilities/model.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Define the canonical, target-independent semantic representation of
//! capability identities, versions, version constraints, capability
//! properties, and capability requirements.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend AST capability
//!      |
//!      v
//! semantic name resolution
//!      |
//!      v
//! THIS MODULE
//!      |
//!      +--> capability requirements
//!      +--> semantic facts
//!      +--> domain validation
//!      +--> capability compatibility
//!      |
//!      v
//! canonical semantic program
//!      |
//!      v
//! canonical IR adapters
//!      |
//!      v
//! target capability discovery and feasibility
//!      |
//!      v
//! lowering / routing / scheduling / execution
//! ```
//!
//! # Ownership
//!
//! This module owns:
//!
//! - resolved, namespace-qualified semantic capability identities;
//! - semantic capability versions;
//! - version constraints and their matching rules;
//! - typed, deterministic capability properties;
//! - source-level semantic capability requirements;
//! - local validation errors and invariants.
//!
//! This module does NOT own:
//!
//! - source syntax or AST node definitions;
//! - name resolution or namespace imports;
//! - a registry of all known capabilities;
//! - target capability inventories or evidence;
//! - authentication, authorization, or security tokens;
//! - resource allocation or target selection;
//! - hardware discovery, topology, routing, or scheduling;
//! - quantum operations or a replacement for `quantum::ir`;
//! - a closed list of domains, devices, operations, or capabilities.
//!
//! # Authority and integration
//!
//! Syntax is governed by the canonical Zamani grammar and AST.
//! Semantic meaning is governed by `grammar/specification/semantic-model.md`
//! and the applicable normative specifications.
//!
//! The native AST capability currently lives under:
//!
//! `crate::frontend::ast::node::capabilities::capability`
//!
//! Its `Capability`, `CapabilityVersion`, and
//! `CapabilityVersionConstraint` types remain owned by the AST module.
//! The semantic analyzer converts validated AST values into this module's
//! semantic values after resolving names and applying the language rules.
//!
//! This module intentionally does not import the AST. That keeps the semantic
//! data model independently testable and avoids a reverse dependency from the
//! model into syntax infrastructure.
//!
//! The quantum IR retains its own canonical IR capability types. Conversion
//! into those types belongs in a downstream adapter, not here.
//!
//! # POCO-REAF and scalability
//!
//! A capability identity describes a semantic ability, not a particular
//! machine, vendor, device, physical qubit, register, or execution location.
//!
//! Identifiers are open-world: new namespaces and capability names do not
//! require changing a Rust enum or editing this file.
//!
//! There is no language-level maximum capability count, namespace depth,
//! property count, or hardware capacity in this module. Collections grow
//! according to available memory and any explicit, external compiler policy.
//! Rust strings and collections remain finite, as do actual compiler resources.
//!
//! Operational limits must be enforced by the analysis session or configured
//! compiler policy. They must not be disguised as semantic validity rules.
//!
//! # Determinism
//!
//! Semantic identity depends only on namespace and name.
//! Properties are stored in `BTreeMap` order. Requirement collections can use
//! `BTreeSet` or `BTreeMap` to obtain deterministic ordering.
//! No timestamp, address, random identifier, device state, or global mutable
//! registry participates in equality or hashing.
//!
//! # Requirements versus target evidence
//!
//! A `CapabilityRequirement` describes what must be satisfied.
//! It does not claim that any target provides the capability.
//! Target evidence, trust, authorization, compatibility decisions, and
//! feasibility belong to their own semantic services or downstream systems.
//!
//! # File contract
//!
//! Inputs: validated textual names, semantic versions, and typed properties.
//! Outputs: validated, deterministic semantic capability values.
//! Errors: explicit local validation failures; no panics for ordinary invalid
//! input.
//! Consumers: `capabilities/requirements.rs`, `capabilities/evidence.rs`,
//! `capabilities/compatibility.rs`, semantic facts, domain validators, and
//! downstream IR adapters.
//!
//! Freeze criteria:
//! - unit tests pass;
//! - AST conversion tests verify the actual AST contract;
//! - compatibility and determinism tests pass;
//! - module exports agree with `capabilities/mod.rs`;
//! - the complete workspace builds on the supported Rust toolchain.
//!
//! This module contains no unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// =============================================================================
// Capability identity
// =============================================================================

/// A resolved, namespace-qualified semantic capability identity.
///
/// The identity is independent of implementation version, target, hardware,
/// source location, and evidence.
///
/// Examples:
///
/// - `zamani.quantum` + `dynamic_control`
/// - `zamani.accelerator` + `tensor_execution`
/// - `future.distributed` + `collective_execution`
///
/// The namespace is not required to be from a built-in list. Extensions can
/// introduce new namespaces without changing this type.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct CapabilityId {
    namespace: String,
    name: String,
}

impl CapabilityId {
    /// Construct a capability identity after validating its components.
    ///
    /// Names are preserved exactly; this constructor does not silently
    /// lowercase, normalize, alias, or resolve them.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CapabilityModelError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(
            &namespace,
            CapabilityComponent::Namespace,
        )?;
        validate_component(&name, CapabilityComponent::Name)?;

        Ok(Self { namespace, name })
    }

    /// Returns the namespace exactly as stored.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the unqualified capability name exactly as stored.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the fully qualified display form.
    ///
    /// The returned value is intended for display and diagnostics. It is not
    /// a substitute for structured identity in maps or equality checks.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }

    /// Returns true when the identity equals the supplied components.
    ///
    /// This avoids allocating a temporary identity for lookup.
    #[must_use]
    pub fn matches(&self, namespace: &str, name: &str) -> bool {
        self.namespace == namespace && self.name == name
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.namespace, self.name)
    }
}

// =============================================================================
// Capability versions
// =============================================================================

/// A semantic capability version.
///
/// Version components use `u64` to provide a stable, explicit numeric domain
/// independent of the host's pointer width. This is a representation choice,
/// not a hardware-capacity limit.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
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

/// A version constraint on a capability.
///
/// This type specifies a compatibility condition; it does not prove that a
/// compatible implementation exists or is available on a target.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub enum CapabilityVersionConstraint {
    /// Any version is acceptable.
    Any,

    /// Only this exact version is acceptable.
    Exact(CapabilityVersion),

    /// This version or a newer version is acceptable.
    AtLeast(CapabilityVersion),

    /// This version or an older version is acceptable.
    AtMost(CapabilityVersion),

    /// Versions within this inclusive interval are acceptable.
    Between {
        minimum: CapabilityVersion,
        maximum: CapabilityVersion,
    },
}

impl Default for CapabilityVersionConstraint {
    fn default() -> Self {
        Self::Any
    }
}

impl CapabilityVersionConstraint {
    /// Construct an inclusive version interval.
    pub fn between(
        minimum: CapabilityVersion,
        maximum: CapabilityVersion,
    ) -> Result<Self, CapabilityModelError> {
        if minimum > maximum {
            return Err(CapabilityModelError::InvalidVersionRange {
                minimum,
                maximum,
            });
        }

        Ok(Self::Between { minimum, maximum })
    }

    /// Check whether a concrete version satisfies this constraint.
    #[must_use]
    pub const fn matches(self, version: CapabilityVersion) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(required) => version.major == required.major
                && version.minor == required.minor
                && version.patch == required.patch,
            Self::AtLeast(required) => {
                version.major > required.major
                    || (version.major == required.major
                        && (version.minor > required.minor
                            || (version.minor == required.minor
                                && version.patch >= required.patch)))
            }
            Self::AtMost(required) => {
                version.major < required.major
                    || (version.major == required.major
                        && (version.minor < required.minor
                            || (version.minor == required.minor
                                && version.patch <= required.patch)))
            }
            Self::Between { minimum, maximum } => {
                let at_least_minimum =
                    version.major > minimum.major
                        || (version.major == minimum.major
                            && (version.minor > minimum.minor
                                || (version.minor == minimum.minor
                                    && version.patch >= minimum.patch)));

                let at_most_maximum =
                    version.major < maximum.major
                        || (version.major == maximum.major
                            && (version.minor < maximum.minor
                                || (version.minor == maximum.minor
                                    && version.patch <= maximum.patch)));

                at_least_minimum && at_most_maximum
            }
        }
    }
}

// =============================================================================
// Capability property values
// =============================================================================

/// A typed, target-independent property value attached to a capability
/// requirement.
///
/// Values intentionally remain generic. Domain-specific meaning is supplied
/// by a validated capability definition or extension, not by this enum.
///
/// Floating-point values are represented by their source-independent textual
/// form rather than `f64`, avoiding NaN equality, non-total ordering, and
/// unstable map semantics. A domain-specific validator must parse and validate
/// the text according to the registered property's declared type.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub enum CapabilityPropertyValue {
    /// Boolean property.
    Boolean(bool),

    /// Signed integer property.
    Integer(i128),

    /// Unsigned integer property.
    Unsigned(u128),

    /// Exact decimal or domain-defined numeric spelling.
    ///
    /// The producer must use the canonical representation required by its
    /// property schema. This model does not assume a floating-point format.
    Number(String),

    /// Textual property.
    Text(String),

    /// A structured sequence of property values.
    Sequence(Vec<CapabilityPropertyValue>),

    /// A deterministic map of nested property values.
    Object(BTreeMap<String, CapabilityPropertyValue>),

    /// A symbolic expression whose meaning is resolved by another semantic
    /// service. It must not be treated as a concrete value without resolution.
    Symbolic(String),
}

impl CapabilityPropertyValue {
    /// Validate nested property keys and text payloads.
    ///
    /// This validates representation invariants only. It does not validate
    /// the value against a registered capability-property schema.
    pub fn validate(&self) -> Result<(), CapabilityModelError> {
        match self {
            Self::Boolean(_) | Self::Integer(_) | Self::Unsigned(_) => Ok(()),

            Self::Number(value) => {
                validate_non_empty_text(
                    value,
                    CapabilityComponent::PropertyValue,
                )
            }

            Self::Text(value) => {
                if value.chars().any(char::is_control) {
                    return Err(CapabilityModelError::ControlCharacter {
                        component: CapabilityComponent::PropertyValue,
                    });
                }
                Ok(())
            }

            Self::Sequence(values) => {
                for value in values {
                    value.validate()?;
                }
                Ok(())
            }

            Self::Object(values) => {
                for (key, value) in values {
                    validate_component(
                        key,
                        CapabilityComponent::PropertyName,
                    )?;
                    value.validate()?;
                }
                Ok(())
            }

            Self::Symbolic(expression) => validate_non_empty_text(
                expression,
                CapabilityComponent::PropertyValue,
            ),
        }
    }
}

// =============================================================================
// Capability requirements
// =============================================================================

/// A single semantic capability requirement.
///
/// This represents a required semantic ability and its optional version and
/// properties. It is not target evidence and cannot, by itself, establish
/// feasibility or grant authorization.
///
/// Property keys are stored in a `BTreeMap` for deterministic ordering.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct CapabilityRequirement {
    capability: CapabilityId,
    version: CapabilityVersionConstraint,
    properties: BTreeMap<String, CapabilityPropertyValue>,
}

impl CapabilityRequirement {
    /// Create a requirement for a capability without additional properties.
    #[must_use]
    pub fn new(capability: CapabilityId) -> Self {
        Self {
            capability,
            version: CapabilityVersionConstraint::Any,
            properties: BTreeMap::new(),
        }
    }

    /// Create a requirement with an explicit version constraint.
    ///
    /// The caller should obtain range constraints through
    /// `CapabilityVersionConstraint::between` so invalid intervals cannot be
    /// introduced accidentally.
    #[must_use]
    pub fn with_version(
        capability: CapabilityId,
        version: CapabilityVersionConstraint,
    ) -> Self {
        Self {
            capability,
            version,
            properties: BTreeMap::new(),
        }
    }

    /// Return the required capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Return the version constraint.
    #[must_use]
    pub const fn version_constraint(
        &self,
    ) -> CapabilityVersionConstraint {
        self.version
    }

    /// Return the requirement properties in deterministic key order.
    #[must_use]
    pub fn properties(
        &self,
    ) -> &BTreeMap<String, CapabilityPropertyValue> {
        &self.properties
    }

    /// Insert or replace a property.
    ///
    /// Replacement is explicit and atomic: invalid input leaves the existing
    /// map unchanged. The returned value is the previous property, if any.
    pub fn set_property(
        &mut self,
        name: impl Into<String>,
        value: CapabilityPropertyValue,
    ) -> Result<Option<CapabilityPropertyValue>, CapabilityModelError> {
        let name = name.into();

        validate_component(&name, CapabilityComponent::PropertyName)?;
        value.validate()?;

        Ok(self.properties.insert(name, value))
    }

    /// Add a property only if the key is not already present.
    ///
    /// This is useful when combining independent requirement sources: duplicate
    /// keys are surfaced for the caller to resolve instead of being silently
    /// overwritten.
    pub fn add_property(
        &mut self,
        name: impl Into<String>,
        value: CapabilityPropertyValue,
    ) -> Result<(), CapabilityModelError> {
        let name = name.into();

        validate_component(&name, CapabilityComponent::PropertyName)?;
        value.validate()?;

        if self.properties.contains_key(&name) {
            return Err(CapabilityModelError::DuplicateProperty { name });
        }

        self.properties.insert(name, value);
        Ok(())
    }

    /// Check whether a concrete implementation version satisfies this
    /// requirement's version constraint.
    #[must_use]
    pub const fn accepts_version(&self, version: CapabilityVersion) -> bool {
        self.version.matches(version)
    }
}

// =============================================================================
// Structured errors
// =============================================================================

/// The component whose textual representation failed validation.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub enum CapabilityComponent {
    Namespace,
    Name,
    PropertyName,
    PropertyValue,
}

impl fmt::Display for CapabilityComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace => f.write_str("namespace"),
            Self::Name => f.write_str("name"),
            Self::PropertyName => f.write_str("property name"),
            Self::PropertyValue => f.write_str("property value"),
        }
    }
}

/// Local errors produced by the semantic capability model.
///
/// These are model-validation errors, not target-compatibility or
/// authorization failures. Those require separate diagnostics and services.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CapabilityModelError {
    EmptyComponent {
        component: CapabilityComponent,
    },

    ControlCharacter {
        component: CapabilityComponent,
    },

    InvalidVersionRange {
        minimum: CapabilityVersion,
        maximum: CapabilityVersion,
    },

    DuplicateProperty {
        name: String,
    },
}

impl fmt::Display for CapabilityModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyComponent { component } => {
                write!(f, "capability {component} must not be empty")
            }

            Self::ControlCharacter { component } => {
                write!(
                    f,
                    "capability {component} must not contain control characters"
                )
            }

            Self::InvalidVersionRange { minimum, maximum } => {
                write!(
                    f,
                    "invalid capability version interval: minimum {minimum} \
                     exceeds maximum {maximum}"
                )
            }

            Self::DuplicateProperty { name } => {
                write!(f, "capability property {name:?} is already defined")
            }
        }
    }
}

impl Error for CapabilityModelError {}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn validate_component(
    value: &str,
    component: CapabilityComponent,
) -> Result<(), CapabilityModelError> {
    if value.trim().is_empty() {
        return Err(CapabilityModelError::EmptyComponent { component });
    }

    if value.chars().any(char::is_control) {
        return Err(CapabilityModelError::ControlCharacter { component });
    }

    Ok(())
}

fn validate_non_empty_text(
    value: &str,
    component: CapabilityComponent,
) -> Result<(), CapabilityModelError> {
    if value.trim().is_empty() {
        return Err(CapabilityModelError::EmptyComponent { component });
    }

    if value.chars().any(char::is_control) {
        return Err(CapabilityModelError::ControlCharacter { component });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn capability() -> CapabilityId {
        CapabilityId::new(
            "zamani.quantum",
            "dynamic_control",
        )
        .expect("valid capability identity")
    }

    #[test]
    fn identity_is_namespace_qualified_and_stable() {
        let left = capability();
        let right = capability();

        assert_eq!(left, right);
        assert_eq!(left.qualified_name(), "zamani.quantum.dynamic_control");
        assert!(left.matches("zamani.quantum", "dynamic_control"));
        assert!(!left.matches("zamani.classical", "dynamic_control"));
    }

    #[test]
    fn identity_rejects_empty_components() {
        assert!(matches!(
            CapabilityId::new("", "operation"),
            Err(CapabilityModelError::EmptyComponent {
                component: CapabilityComponent::Namespace
            })
        ));

        assert!(matches!(
            CapabilityId::new("zamani.quantum", " "),
            Err(CapabilityModelError::EmptyComponent {
                component: CapabilityComponent::Name
            })
        ));
    }

    #[test]
    fn identity_rejects_control_characters() {
        assert!(matches!(
            CapabilityId::new("zamani.\nquantum", "measurement"),
            Err(CapabilityModelError::ControlCharacter {
                component: CapabilityComponent::Namespace
            })
        ));
    }

    #[test]
    fn arbitrary_namespaces_are_supported() {
        let id = CapabilityId::new(
            "future.photonic.computation",
            "coherent_transform",
        )
        .expect("open-world identity");

        assert_eq!(
            id.qualified_name(),
            "future.photonic.computation.coherent_transform"
        );
    }

    #[test]
    fn unicode_names_are_preserved() {
        let id = CapabilityId::new("future.example", "能力")
            .expect("Unicode is valid text");

        assert_eq!(id.name(), "能力");
    }

    #[test]
    fn version_constraints_match_correctly() {
        let one = CapabilityVersion::new(1, 0, 0);
        let two = CapabilityVersion::new(2, 0, 0);
        let three = CapabilityVersion::new(3, 0, 0);

        assert!(CapabilityVersionConstraint::Any.matches(two));
        assert!(CapabilityVersionConstraint::Exact(two).matches(two));
        assert!(!CapabilityVersionConstraint::Exact(two).matches(one));
        assert!(CapabilityVersionConstraint::AtLeast(two).matches(three));
        assert!(!CapabilityVersionConstraint::AtLeast(two).matches(one));
        assert!(CapabilityVersionConstraint::AtMost(two).matches(one));
        assert!(!CapabilityVersionConstraint::AtMost(two).matches(three));

        let interval = CapabilityVersionConstraint::between(one, three)
            .expect("valid interval");

        assert!(interval.matches(one));
        assert!(interval.matches(two));
        assert!(interval.matches(three));
        assert!(!interval.matches(CapabilityVersion::new(3, 0, 1)));
    }

    #[test]
    fn reversed_version_interval_is_rejected() {
        let minimum = CapabilityVersion::new(3, 0, 0);
        let maximum = CapabilityVersion::new(2, 0, 0);

        assert!(matches!(
            CapabilityVersionConstraint::between(minimum, maximum),
            Err(CapabilityModelError::InvalidVersionRange { .. })
        ));
    }

    #[test]
    fn requirement_properties_are_deterministic() {
        let mut requirement = CapabilityRequirement::new(capability());

        requirement
            .add_property(
                "mode",
                CapabilityPropertyValue::Text("adaptive".to_owned()),
            )
            .expect("valid property");

        requirement
            .add_property(
                "enabled",
                CapabilityPropertyValue::Boolean(true),
            )
            .expect("valid property");

        let keys: Vec<&str> = requirement
            .properties()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(keys, vec!["enabled", "mode"]);
    }

    #[test]
    fn duplicate_property_does_not_overwrite_existing_value() {
        let mut requirement = CapabilityRequirement::new(capability());

        requirement
            .add_property(
                "mode",
                CapabilityPropertyValue::Text("strict".to_owned()),
            )
            .expect("first property");

        assert!(matches!(
            requirement.add_property(
                "mode",
                CapabilityPropertyValue::Text("relaxed".to_owned()),
            ),
            Err(CapabilityModelError::DuplicateProperty { .. })
        ));

        assert_eq!(
            requirement.properties().get("mode"),
            Some(&CapabilityPropertyValue::Text("strict".to_owned()))
        );
    }

    #[test]
    fn invalid_property_replacement_is_atomic() {
        let mut requirement = CapabilityRequirement::new(capability());

        requirement
            .set_property(
                "mode",
                CapabilityPropertyValue::Text("strict".to_owned()),
            )
            .expect("valid property");

        assert!(requirement
            .set_property(
                "",
                CapabilityPropertyValue::Boolean(false),
            )
            .is_err());

        assert_eq!(
            requirement.properties().get("mode"),
            Some(&CapabilityPropertyValue::Text("strict".to_owned()))
        );
    }

    #[test]
    fn nested_property_values_are_validated() {
        let invalid = CapabilityPropertyValue::Object(BTreeMap::from([(
            String::new(),
            CapabilityPropertyValue::Boolean(true),
        )]));

        assert!(invalid.validate().is_err());
    }

    #[test]
    fn symbolic_values_are_preserved_without_evaluation() {
        let value = CapabilityPropertyValue::Symbolic(
            "required_parallelism".to_owned(),
        );

        assert!(value.validate().is_ok());
        assert_eq!(
            value,
            CapabilityPropertyValue::Symbolic(
                "required_parallelism".to_owned()
            )
        );
    }

    #[test]
    fn no_hardware_identity_is_needed() {
        let requirement = CapabilityRequirement::new(capability());

        assert_eq!(
            requirement.capability().qualified_name(),
            "zamani.quantum.dynamic_control"
        );

        // Requirements intentionally contain no target, device, vendor,
        // physical qubit, topology, scheduler, or runtime handle.
    }
}