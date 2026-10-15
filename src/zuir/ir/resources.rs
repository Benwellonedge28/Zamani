
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Resource Intent
//!
//! Path: `src/zuir/ir/resources.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: safe Rust only.
//!
//! PURPOSE
//!
//! Represent target-independent resource requirements, constraints, budgets,
//! preferences, hints, capability requirements, and topology intent.
//!
//! Resource intent describes what a valid realization must satisfy. It does
//! not allocate, reserve, discover, route, schedule, or identify physical
//! resources.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `grammar/DESIGN.md`
//! - `grammar/spec/resources.md`
//! - `grammar/resources/README.md`
//! - `grammar/resources/constraints.g4`
//! - `grammar/resources/budgets.g4`
//! - `grammar/resources/capabilities.g4`
//! - `src/zuir/errors.rs`
//! - `src/zuir/config.rs`
//!
//! OWNS
//!
//! - Resource key and exact non-negative decimal quantity representations.
//! - Resource assertions and their intent categories.
//! - Capability requirements.
//! - Portable topology relationships.
//! - Resource intent aggregation and local validation.
//!
//! DOES NOT OWN
//!
//! - Source parsing or AST representation.
//! - Semantic name resolution or expression evaluation.
//! - Physical resource discovery, allocation, reservation, or release.
//! - Target capability inventories or feasibility decisions.
//! - Scheduling, placement, routing, QEC, ZQN, or HAL.
//! - Whole-program verification or resource cost analysis.
//!
//! SCALABILITY
//!
//! Resource names and collections are open-ended. Quantities are represented
//! as canonical decimal strings instead of host-sized integers or floats.
//! This avoids fixed-width truncation and floating-point comparison errors.
//!
//! No artificial universal maximum is imposed on qubits, memory, processors,
//! nodes, devices, tensor dimensions, resource kinds, or resource assertions.
//! Actual work is necessarily bounded by available memory, time, and other
//! process resources; this module does not promise literal infinity.
//!
//! INTEGRATION
//!
//! - `ir/operation.rs`, `ir/function.rs`, and `ir/module.rs` may reference or
//!   aggregate this model through their own established contracts.
//! - `verify/resources.rs` should check cross-entity validity, contradictory
//!   mandatory assertions, registered units, capabilities, and target rules.
//! - `analysis/resources.rs` should derive resource estimates and symbolic
//!   bounds without mutating source intent.
//! - `lowering/` and target adapters should resolve intent against actual
//!   capabilities and capacity.
//! - Grammar/AST/semantic adapters should translate source declarations into
//!   these types without parsing source syntax in this module.
//!
//! This module deliberately does not import planned siblings.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::errors::{ZuirError, ZuirErrorKind, ZuirResult};

// =============================================================================
// Resource keys
// =============================================================================

/// A validated, open-world resource, property, unit, capability, or topology
/// name.
///
/// Names are semantic identifiers, not physical addresses or device IDs.
/// Namespaces are separated by dots. The vocabulary is intentionally open:
/// extensions can introduce new names without modifying this module.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct ResourceKey(String);

impl ResourceKey {
    /// Create a validated semantic key.
    ///
    /// Accepted segments are non-empty and contain ASCII letters, digits,
    /// underscores, or hyphens. A segment may not begin with a hyphen.
    /// Dots separate namespace segments.
    ///
    /// This deliberately validates the representation only. Registry
    /// membership and authorization are separate concerns.
    pub fn new(value: impl Into<String>) -> ZuirResult<Self> {
        let value = value.into();

        let valid = !value.is_empty()
            && value.split('.').all(|segment| {
                !segment.is_empty()
                    && !segment.starts_with('-')
                    && segment.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric()
                            || byte == b'_'
                            || byte == b'-'
                    })
            });

        if !valid {
            return Err(resource_error(
                ZuirErrorKind::InvalidInput,
                format!("invalid resource key `{value}`"),
            ));
        }

        Ok(Self(value))
    }

    /// Borrow the canonical key text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Revalidate data received through deserialization or interchange.
    pub fn validate(&self) -> ZuirResult<()> {
        Self::new(self.0.clone()).map(|_| ())
    }
}

impl fmt::Display for ResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Exact decimal magnitudes
// =============================================================================

/// A canonical, finite, non-negative base-10 decimal magnitude.
///
/// Examples: `0`, `12`, `0.25`, `100000000000000000000000000000000000`.
///
/// The value is not tied to a particular unit. Units belong to
/// `ResourceQuantity`.
///
/// This representation supports exact ordering without `f32`, `f64`, or
/// conversion to a fixed-width integer. Arithmetic and unit conversion are
/// intentionally delegated to a dedicated evaluator/analysis subsystem.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct DecimalMagnitude(String);

impl DecimalMagnitude {
    /// Parse and canonicalize a non-negative decimal magnitude.
    ///
    /// Scientific notation, signs, separators, NaN, and infinity are rejected.
    /// Leading integer zeroes and trailing fractional zeroes are normalized.
    pub fn parse(input: &str) -> ZuirResult<Self> {
        if input.is_empty() {
            return Err(invalid_magnitude(input));
        }

        let mut parts = input.split('.');
        let integer = parts.next().unwrap_or_default();
        let fraction = parts.next();

        if parts.next().is_some()
            || integer.is_empty()
            || !integer.bytes().all(|b| b.is_ascii_digit())
            || fraction.is_some_and(|f| {
                f.is_empty() || !f.bytes().all(|b| b.is_ascii_digit())
            })
        {
            return Err(invalid_magnitude(input));
        }

        let integer = integer.trim_start_matches('0');
        let integer = if integer.is_empty() { "0" } else { integer };

        let fraction = fraction.unwrap_or_default().trim_end_matches('0');

        let canonical = if fraction.is_empty() {
            integer.to_owned()
        } else {
            format!("{integer}.{fraction}")
        };

        Ok(Self(canonical))
    }

    /// Construct exact zero.
    #[must_use]
    pub fn zero() -> Self {
        Self("0".to_owned())
    }

    /// Borrow the canonical decimal text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether this value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0 == "0"
    }

    /// Compare exact non-negative decimal values without floating point.
    #[must_use]
    pub fn compare(&self, other: &Self) -> Ordering {
        let (left_integer, left_fraction) = split_decimal(&self.0);
        let (right_integer, right_fraction) = split_decimal(&other.0);

        left_integer
            .len()
            .cmp(&right_integer.len())
            .then_with(|| left_integer.cmp(right_integer))
            .then_with(|| {
                let width = left_fraction.len().max(right_fraction.len());
                (0..width)
                    .map(|index| {
                        left_fraction.as_bytes().get(index).copied().unwrap_or(b'0')
                    })
                    .cmp((0..width).map(|index| {
                        right_fraction.as_bytes().get(index).copied().unwrap_or(b'0')
                    }))
            })
    }

    /// Validate canonical form after deserialization.
    pub fn validate(&self) -> ZuirResult<()> {
        let parsed = Self::parse(&self.0)?;

        if parsed.0 != self.0 {
            return Err(resource_error(
                ZuirErrorKind::InvalidInput,
                format!("non-canonical decimal magnitude `{}`", self.0),
            ));
        }

        Ok(())
    }
}

impl fmt::Display for DecimalMagnitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialOrd for DecimalMagnitude {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DecimalMagnitude {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

fn split_decimal(value: &str) -> (&str, &str) {
    value.split_once('.').unwrap_or((value, ""))
}

fn invalid_magnitude(value: &str) -> ZuirError {
    resource_error(
        ZuirErrorKind::InvalidInput,
        format!("invalid non-negative decimal magnitude `{value}`"),
    )
}

// =============================================================================
// Quantities
// =============================================================================

/// A quantity paired with an explicitly named unit.
///
/// The unit is semantic metadata. This type does not assume that values
/// expressed in different units are directly comparable. Unit equivalence
/// and conversion must be established by the unit registry or evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceQuantity {
    magnitude: DecimalMagnitude,
    unit: ResourceKey,
}

impl ResourceQuantity {
    /// Construct a quantity from a decimal magnitude and semantic unit name.
    pub fn new(
        magnitude: DecimalMagnitude,
        unit: ResourceKey,
    ) -> ZuirResult<Self> {
        magnitude.validate()?;
        unit.validate()?;

        Ok(Self { magnitude, unit })
    }

    /// Parse a quantity without converting it to machine-sized numeric types.
    pub fn parse(
        magnitude: &str,
        unit: impl Into<String>,
    ) -> ZuirResult<Self> {
        Self::new(
            DecimalMagnitude::parse(magnitude)?,
            ResourceKey::new(unit)?,
        )
    }

    #[must_use]
    pub fn magnitude(&self) -> &DecimalMagnitude {
        &self.magnitude
    }

    #[must_use]
    pub fn unit(&self) -> &ResourceKey {
        &self.unit
    }

    pub fn validate(&self) -> ZuirResult<()> {
        self.magnitude.validate()?;
        self.unit.validate()
    }
}

// =============================================================================
// Intent categories and comparisons
// =============================================================================

/// Semantic force of a resource assertion.
///
/// Requirements and hard constraints must be satisfied for a valid
/// realization. Budgets express declared limits or allowances. Preferences
/// and hints must never silently become mandatory constraints.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ResourceIntentKind {
    /// A mandatory resource condition.
    Requirement,

    /// A mandatory condition on a valid realization.
    Constraint,

    /// A declared allowance or consumption envelope.
    Budget,

    /// A non-mandatory optimization objective.
    Preference,

    /// Advisory information that cannot change semantics.
    Hint,
}

/// Comparison between a resource quantity and a declared bound.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ResourceRelation {
    /// Actual value must be greater than or equal to the declared value.
    AtLeast,

    /// Actual value must be less than or equal to the declared value.
    AtMost,

    /// Actual value must equal the declared value.
    Exactly,

    /// Actual value must be strictly greater than the declared value.
    GreaterThan,

    /// Actual value must be strictly less than the declared value.
    LessThan,
}

/// One assertion about a named resource dimension.
///
/// Examples of dimension keys include `quantum.logical_qubits`,
/// `memory.capacity`, `processing.units`, `network.bandwidth`,
/// `execution.latency`, `energy.consumption`, and extension-defined names.
///
/// The module does not hard-code this list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceAssertion {
    dimension: ResourceKey,
    relation: ResourceRelation,
    quantity: ResourceQuantity,
    kind: ResourceIntentKind,
}

impl ResourceAssertion {
    pub fn new(
        dimension: ResourceKey,
        relation: ResourceRelation,
        quantity: ResourceQuantity,
        kind: ResourceIntentKind,
    ) -> ZuirResult<Self> {
        dimension.validate()?;
        quantity.validate()?;

        Ok(Self {
            dimension,
            relation,
            quantity,
            kind,
        })
    }

    #[must_use]
    pub fn dimension(&self) -> &ResourceKey {
        &self.dimension
    }

    #[must_use]
    pub const fn relation(&self) -> ResourceRelation {
        self.relation
    }

    #[must_use]
    pub fn quantity(&self) -> &ResourceQuantity {
        &self.quantity
    }

    #[must_use]
    pub const fn kind(&self) -> ResourceIntentKind {
        self.kind
    }

    pub fn validate(&self) -> ZuirResult<()> {
        self.dimension.validate()?;
        self.quantity.validate()
    }

    /// Evaluate a known actual quantity against this assertion.
    ///
    /// This method compares values only when the units have identical names.
    /// It does not perform implicit unit conversion or target feasibility
    /// analysis. `Ok(false)` means the known value violates the relation.
    pub fn evaluate_same_unit(
        &self,
        actual: &ResourceQuantity,
    ) -> ZuirResult<bool> {
        actual.validate()?;

        if actual.unit != self.quantity.unit {
            return Err(resource_error(
                ZuirErrorKind::InvalidInput,
                format!(
                    "cannot compare units `{}` and `{}` without a unit-conversion contract",
                    actual.unit, self.quantity.unit
                ),
            ));
        }

        let ordering = actual.magnitude.cmp(&self.quantity.magnitude);

        Ok(match self.relation {
            ResourceRelation::AtLeast => ordering != Ordering::Less,
            ResourceRelation::AtMost => ordering != Ordering::Greater,
            ResourceRelation::Exactly => ordering == Ordering::Equal,
            ResourceRelation::GreaterThan => ordering == Ordering::Greater,
            ResourceRelation::LessThan => ordering == Ordering::Less,
        })
    }
}

// =============================================================================
// Capability intent
// =============================================================================

/// A required or preferred capability.
///
/// Capability names are open-world semantic names, not vendor identifiers.
/// Whether a target provides a capability is determined by a downstream
/// capability registry or target adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityIntent {
    name: ResourceKey,
    kind: ResourceIntentKind,
}

impl CapabilityIntent {
    pub fn new(
        name: ResourceKey,
        kind: ResourceIntentKind,
    ) -> ZuirResult<Self> {
        if !matches!(
            kind,
            ResourceIntentKind::Requirement
                | ResourceIntentKind::Constraint
                | ResourceIntentKind::Preference
                | ResourceIntentKind::Hint
        ) {
            return Err(resource_error(
                ZuirErrorKind::InvalidInput,
                "capability intent cannot use the resource-budget category",
            ));
        }

        name.validate()?;
        Ok(Self { name, kind })
    }

    #[must_use]
    pub fn name(&self) -> &ResourceKey {
        &self.name
    }

    #[must_use]
    pub const fn kind(&self) -> ResourceIntentKind {
        self.kind
    }

    pub fn validate(&self) -> ZuirResult<()> {
        self.name.validate()
    }
}

// =============================================================================
// Portable topology intent
// =============================================================================

/// Relationship between logical resource groups.
///
/// `left` and `right` are logical names, not physical device identifiers.
/// Physical placement and topology discovery are downstream responsibilities.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum TopologyRelation {
    /// Logical resources should be close under the selected topology metric.
    Near,

    /// Logical resources must be connected by a valid communication path.
    Connected,

    /// Logical resources should share a realization scope.
    CoLocate,

    /// Logical resources must not share a realization scope.
    Separate,

    /// Logical resources should have low communication cost.
    LowCommunicationCost,

    /// An extension-defined topology relation.
    Extension,
}

/// A portable relationship constraint over logical resource groups.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyIntent {
    relation: TopologyRelation,
    left: ResourceKey,
    right: ResourceKey,
    kind: ResourceIntentKind,
    extension_relation: Option<ResourceKey>,
}

impl TopologyIntent {
    pub fn new(
        relation: TopologyRelation,
        left: ResourceKey,
        right: ResourceKey,
        kind: ResourceIntentKind,
    ) -> ZuirResult<Self> {
        Self::with_extension(relation, left, right, kind, None)
    }

    /// Construct an extension-defined relation.
    ///
    /// An extension relation is only descriptive until its semantics are
    /// registered and validated by the responsible dialect or topology
    /// subsystem.
    pub fn extension(
        relation_name: ResourceKey,
        left: ResourceKey,
        right: ResourceKey,
        kind: ResourceIntentKind,
    ) -> ZuirResult<Self> {
        Self::with_extension(
            TopologyRelation::Extension,
            left,
            right,
            kind,
            Some(relation_name),
        )
    }

    fn with_extension(
        relation: TopologyRelation,
        left: ResourceKey,
        right: ResourceKey,
        kind: ResourceIntentKind,
        extension_relation: Option<ResourceKey>,
    ) -> ZuirResult<Self> {
        left.validate()?;
        right.validate()?;

        if left == right
            && matches!(
                relation,
                TopologyRelation::Separate
            )
        {
            return Err(resource_error(
                ZuirErrorKind::ResourceConstraintViolation,
                "a logical resource group cannot be required to be separate from itself",
            ));
        }

        if relation == TopologyRelation::Extension
            && extension_relation.is_none()
        {
            return Err(resource_error(
                ZuirErrorKind::InvalidInput,
                "extension topology relation requires a qualified relation name",
            ));
        }

        if let Some(name) = &extension_relation {
            name.validate()?;
        }

        Ok(Self {
            relation,
            left,
            right,
            kind,
            extension_relation,
        })
    }

    #[must_use]
    pub const fn relation(&self) -> TopologyRelation {
        self.relation
    }

    #[must_use]
    pub fn left(&self) -> &ResourceKey {
        &self.left
    }

    #[must_use]
    pub fn right(&self) -> &ResourceKey {
        &self.right
    }

    #[must_use]
    pub const fn kind(&self) -> ResourceIntentKind {
        self.kind
    }

    #[must_use]
    pub fn extension_relation(&self) -> Option<&ResourceKey> {
        self.extension_relation.as_ref()
    }

    pub fn validate(&self) -> ZuirResult<()> {
        self.left.validate()?;
        self.right.validate()?;

        match (self.relation, self.extension_relation.as_ref()) {
            (TopologyRelation::Extension, Some(name)) => name.validate(),
            (TopologyRelation::Extension, None) => Err(resource_error(
                ZuirErrorKind::InvalidInput,
                "extension topology relation has no registered name",
            )),
            (_, Some(_)) => Err(resource_error(
                ZuirErrorKind::InvalidInput,
                "built-in topology relation must not carry an extension name",
            )),
            (_, None) => Ok(()),
        }
    }
}

// =============================================================================
// Aggregate resource intent
// =============================================================================

/// All resource intent associated with a program, function, region, operation,
/// or other IR entity.
///
/// Scope and ownership of this value are established by the containing IR
/// entity. This type intentionally does not store physical allocations or
/// runtime observations.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ResourceIntent {
    assertions: Vec<ResourceAssertion>,
    capabilities: Vec<CapabilityIntent>,
    topology: Vec<TopologyIntent>,
    metadata: BTreeMap<ResourceKey, String>,
}

impl ResourceIntent {
    /// Create an empty intent.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an assertion without collapsing duplicates or changing intent.
    ///
    /// Conflicting mandatory assertions are retained so the verifier can
    /// produce a useful diagnostic with the full source context.
    pub fn add_assertion(
        &mut self,
        assertion: ResourceAssertion,
    ) -> ZuirResult<()> {
        assertion.validate()?;
        self.assertions.push(assertion);
        Ok(())
    }

    pub fn add_capability(
        &mut self,
        capability: CapabilityIntent,
    ) -> ZuirResult<()> {
        capability.validate()?;
        self.capabilities.push(capability);
        Ok(())
    }

    pub fn add_topology(
        &mut self,
        topology: TopologyIntent,
    ) -> ZuirResult<()> {
        topology.validate()?;
        self.topology.push(topology);
        Ok(())
    }

    /// Add namespaced, descriptive metadata.
    ///
    /// Metadata is not executable semantics unless an owning specification
    /// explicitly assigns it meaning. Replacing an existing key is explicit
    /// through this method's documented last-write behavior.
    pub fn insert_metadata(
        &mut self,
        key: ResourceKey,
        value: impl Into<String>,
    ) -> ZuirResult<Option<String>> {
        key.validate()?;
        Ok(self.metadata.insert(key, value.into()))
    }

    #[must_use]
    pub fn assertions(&self) -> &[ResourceAssertion] {
        &self.assertions
    }

    #[must_use]
    pub fn capabilities(&self) -> &[CapabilityIntent] {
        &self.capabilities
    }

    #[must_use]
    pub fn topology(&self) -> &[TopologyIntent] {
        &self.topology
    }

    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<ResourceKey, String> {
        &self.metadata
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.assertions.is_empty()
            && self.capabilities.is_empty()
            && self.topology.is_empty()
            && self.metadata.is_empty()
    }

    /// Validate all local invariants after deserialization or interchange.
    ///
    /// This does not prove that mandatory assertions are mutually consistent,
    /// that extension names are registered, or that a target can satisfy them.
    pub fn validate(&self) -> ZuirResult<()> {
        for assertion in &self.assertions {
            assertion.validate()?;
        }

        for capability in &self.capabilities {
            capability.validate()?;
        }

        for topology in &self.topology {
            topology.validate()?;
        }

        for key in self.metadata.keys() {
            key.validate()?;
        }

        Ok(())
    }

    /// Return assertions for one dimension in original declaration order.
    pub fn assertions_for(
        &self,
        dimension: &ResourceKey,
    ) -> impl Iterator<Item = &ResourceAssertion> {
        self.assertions
            .iter()
            .filter(move |assertion| assertion.dimension() == dimension)
    }

    /// Return only mandatory assertions.
    pub fn mandatory_assertions(
        &self,
    ) -> impl Iterator<Item = &ResourceAssertion> {
        self.assertions.iter().filter(|assertion| {
            matches!(
                assertion.kind(),
                ResourceIntentKind::Requirement
                    | ResourceIntentKind::Constraint
            )
        })
    }

    /// Return all capability names required by this intent.
    ///
    /// This is a view of declarations, not evidence that a target provides
    /// those capabilities.
    pub fn required_capabilities(
        &self,
    ) -> impl Iterator<Item = &ResourceKey> {
        self.capabilities.iter().filter_map(|capability| {
            matches!(
                capability.kind(),
                ResourceIntentKind::Requirement
                    | ResourceIntentKind::Constraint
            )
            .then_some(capability.name())
        })
    }

    /// Validate that mandatory assertions do not contain obvious contradictory
    /// bounds with identical dimension and unit names.
    ///
    /// This is a conservative local check. It does not evaluate symbolic
    /// expressions, convert units, infer target capacity, or replace the
    /// authoritative whole-program resource verifier.
    pub fn validate_obvious_contradictions(&self) -> ZuirResult<()> {
        let mandatory: Vec<&ResourceAssertion> =
            self.mandatory_assertions().collect();

        for (index, left) in mandatory.iter().enumerate() {
            for right in mandatory.iter().skip(index + 1) {
                if left.dimension != right.dimension
                    || left.quantity.unit != right.quantity.unit
                {
                    continue;
                }

                if assertions_are_obviously_incompatible(left, right) {
                    return Err(resource_error(
                        ZuirErrorKind::ResourceConstraintViolation,
                        format!(
                            "contradictory mandatory resource assertions for `{}`",
                            left.dimension
                        ),
                    ));
                }
            }
        }

        Ok(())
    }
}

fn assertions_are_obviously_incompatible(
    left: &ResourceAssertion,
    right: &ResourceAssertion,
) -> bool {
    let left_value = &left.quantity.magnitude;
    let right_value = &right.quantity.magnitude;

    match (left.relation, right.relation) {
        (ResourceRelation::Exactly, _) => {
            !relation_accepts(right.relation, left_value, right_value)
        }
        (_, ResourceRelation::Exactly) => {
            !relation_accepts(left.relation, right_value, left_value)
        }
        (ResourceRelation::AtLeast, ResourceRelation::AtMost) => {
            left_value > right_value
        }
        (ResourceRelation::AtMost, ResourceRelation::AtLeast) => {
            right_value > left_value
        }
        (ResourceRelation::GreaterThan, ResourceRelation::AtMost) => {
            left_value >= right_value
        }
        (ResourceRelation::AtMost, ResourceRelation::GreaterThan) => {
            right_value >= left_value
        }
        (ResourceRelation::GreaterThan, ResourceRelation::LessThan) => {
            left_value >= right_value
        }
        (ResourceRelation::LessThan, ResourceRelation::GreaterThan) => {
            right_value >= left_value
        }
        (ResourceRelation::GreaterThan, ResourceRelation::AtLeast) => {
            left_value < right_value
        }
        (ResourceRelation::AtLeast, ResourceRelation::GreaterThan) => {
            right_value < left_value
        }
        (ResourceRelation::LessThan, ResourceRelation::AtMost) => {
            left_value > right_value
        }
        (ResourceRelation::AtMost, ResourceRelation::LessThan) => {
            right_value > left_value
        }
        _ => false,
    }
}

fn relation_accepts(
    relation: ResourceRelation,
    actual: &DecimalMagnitude,
    bound: &DecimalMagnitude,
) -> bool {
    match relation {
        ResourceRelation::AtLeast => actual >= bound,
        ResourceRelation::AtMost => actual <= bound,
        ResourceRelation::Exactly => actual == bound,
        ResourceRelation::GreaterThan => actual > bound,
        ResourceRelation::LessThan => actual < bound,
    }
}

fn resource_error(
    kind: ZuirErrorKind,
    message: impl Into<String>,
) -> ZuirError {
    ZuirError::new(kind, message.into())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn key(value: &str) -> ResourceKey {
        ResourceKey::new(value).expect("valid resource key")
    }

    fn quantity(value: &str, unit: &str) -> ResourceQuantity {
        ResourceQuantity::parse(value, unit).expect("valid quantity")
    }

    fn assertion(
        dimension: &str,
        relation: ResourceRelation,
        value: &str,
        unit: &str,
        kind: ResourceIntentKind,
    ) -> ResourceAssertion {
        ResourceAssertion::new(
            key(dimension),
            relation,
            quantity(value, unit),
            kind,
        )
        .expect("valid assertion")
    }

    #[test]
    fn resource_keys_are_open_ended_but_validated() {
        assert!(ResourceKey::new("quantum.logical_qubits").is_ok());
        assert!(ResourceKey::new("future.compute.resource").is_ok());
        assert!(ResourceKey::new("memory..capacity").is_err());
        assert!(ResourceKey::new("").is_err());
    }

    #[test]
    fn decimal_magnitudes_are_canonical() {
        assert_eq!(
            DecimalMagnitude::parse("00012.3400")
                .expect("parse")
                .as_str(),
            "12.34"
        );
        assert_eq!(
            DecimalMagnitude::parse("000.000")
                .expect("parse")
                .as_str(),
            "0"
        );
        assert!(DecimalMagnitude::parse("-1").is_err());
        assert!(DecimalMagnitude::parse("1e9").is_err());
        assert!(DecimalMagnitude::parse("NaN").is_err());
    }

    #[test]
    fn decimal_comparison_is_exact_beyond_machine_width() {
        let large = DecimalMagnitude::parse(
            "99999999999999999999999999999999999999999999999999",
        )
        .expect("large value");

        let larger = DecimalMagnitude::parse(
            "100000000000000000000000000000000000000000000000000",
        )
        .expect("larger value");

        assert!(large < larger);
        assert!(
            DecimalMagnitude::parse("0.1000000000000000000001")
                .expect("decimal")
                > DecimalMagnitude::parse("0.1").expect("decimal")
        );
    }

    #[test]
    fn quantities_never_implicitly_convert_units() {
        let item = assertion(
            "memory.capacity",
            ResourceRelation::AtLeast,
            "1024",
            "byte",
            ResourceIntentKind::Requirement,
        );

        assert!(item.evaluate_same_unit(&quantity("2048", "byte")).unwrap());
        assert!(
            item.evaluate_same_unit(&quantity("2", "kibibyte"))
                .is_err()
        );
    }

    #[test]
    fn mandatory_assertions_are_distinct_from_preferences() {
        let mut intent = ResourceIntent::new();

        intent
            .add_assertion(assertion(
                "quantum.logical_qubits",
                ResourceRelation::AtLeast,
                "128",
                "qubit",
                ResourceIntentKind::Requirement,
            ))
            .expect("add requirement");

        intent
            .add_assertion(assertion(
                "processing.units",
                ResourceRelation::AtLeast,
                "4",
                "unit",
                ResourceIntentKind::Preference,
            ))
            .expect("add preference");

        assert_eq!(intent.mandatory_assertions().count(), 1);
        assert_eq!(intent.assertions().len(), 2);
    }

    #[test]
    fn obvious_mandatory_contradictions_are_detected() {
        let mut intent = ResourceIntent::new();

        intent
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtLeast,
                "100",
                "byte",
                ResourceIntentKind::Requirement,
            ))
            .unwrap();

        intent
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtMost,
                "50",
                "byte",
                ResourceIntentKind::Constraint,
            ))
            .unwrap();

        assert!(intent.validate_obvious_contradictions().is_err());
    }

    #[test]
    fn equal_lower_and_upper_bounds_are_not_contradictory() {
        let mut intent = ResourceIntent::new();

        intent
            .add_assertion(assertion(
                "processing.units",
                ResourceRelation::AtLeast,
                "8",
                "unit",
                ResourceIntentKind::Requirement,
            ))
            .unwrap();

        intent
            .add_assertion(assertion(
                "processing.units",
                ResourceRelation::AtMost,
                "8",
                "unit",
                ResourceIntentKind::Constraint,
            ))
            .unwrap();

        assert!(intent.validate_obvious_contradictions().is_ok());
    }

    #[test]
    fn topology_intent_uses_logical_names_not_device_ids() {
        let topology = TopologyIntent::new(
            TopologyRelation::Connected,
            key("pipeline.stage_a"),
            key("pipeline.stage_b"),
            ResourceIntentKind::Constraint,
        )
        .expect("valid topology");

        assert_eq!(topology.left().as_str(), "pipeline.stage_a");
        assert_eq!(topology.right().as_str(), "pipeline.stage_b");
    }

    #[test]
    fn invalid_self_separation_is_rejected() {
        assert!(
            TopologyIntent::new(
                TopologyRelation::Separate,
                key("worker"),
                key("worker"),
                ResourceIntentKind::Constraint,
            )
            .is_err()
        );
    }

    #[test]
    fn intent_validates_after_deserialization_boundaries() {
        let mut intent = ResourceIntent::new();

        intent
            .add_capability(
                CapabilityIntent::new(
                    key("quantum.measurement"),
                    ResourceIntentKind::Requirement,
                )
                .expect("capability"),
            )
            .unwrap();

        intent
            .add_topology(
                TopologyIntent::extension(
                    key("vendor_extension.custom_relation"),
                    key("logical.group_a"),
                    key("logical.group_b"),
                    ResourceIntentKind::Hint,
                )
                .expect("extension relation"),
            )
            .unwrap();

        assert!(intent.validate().is_ok());
        assert_eq!(intent.required_capabilities().count(), 1);
    }
}
