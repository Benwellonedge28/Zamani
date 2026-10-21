
```rust
//! Target-independent requirements for the Zamani backend.
//!
//! # Responsibility
//!
//! This module describes what a compilation workload requires or prefers.
//! It does not describe target capabilities, prove target feasibility, allocate
//! hardware resources, parse source code, or perform target selection.
//!
//! # Architectural boundaries
//!
//! - Frontend and semantic analysis own source-language meaning.
//! - This module owns the backend-facing representation of workload requirements.
//! - `capabilities.rs` owns target capability descriptions and evidence.
//! - `compatibility.rs` evaluates requirements against target capability evidence.
//! - `selection.rs` uses compatibility results when selecting a target.
//! - Domain subsystems retain ownership of physical resource allocation,
//!   quantum routing, HDL timing closure, distributed placement, and scheduling.
//!
//! # Scalability
//!
//! This module imposes no arbitrary universal maximum for resource quantities,
//! requirement counts, namespaces, or domain names. Resource quantities can be
//! represented as decimal strings when values exceed machine integer widths.
//! Operational limits, when needed, belong in explicit compiler configuration.
//!
//! # Determinism
//!
//! Requirements retain insertion order and source provenance. `canonical_key`
//! and `canonicalized` provide deterministic ordering for fingerprints and
//! comparison. Duplicate and contradictory requirements are preserved: they
//! must not be silently discarded before compatibility and feasibility analysis.
//!
//! # Integration policy
//!
//! This module deliberately does not assume constructors or field layouts from
//! `capabilities.rs`, `compiler_types.rs`, or frontend AST types. Add explicit
//! adapters at the integration boundary once their actual APIs are confirmed.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// Result type used by this module.
pub type RequirementResult<T> = Result<T, RequirementError>;

/// The category of a workload requirement.
///
/// A preference or hint must never be treated as a mandatory requirement.
/// A constraint describes an allowed condition and is not automatically
/// equivalent to a capability requirement.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequirementKind {
    /// The workload cannot preserve its required meaning without this property.
    RequiredCapability,

    /// Prefer this property when choosing among otherwise acceptable targets.
    Preference,

    /// A condition that a candidate target or deployment must satisfy.
    Constraint,

    /// An optimization suggestion; ignoring it must not change program meaning.
    Hint,

    /// A resource quantity or range needed by the workload.
    Resource,
}

/// A comparison operator for requirement values.
///
/// The interpretation of the compared property is domain-specific. For
/// example, comparing versions lexically is not valid; version requirements
/// should use a version-aware adapter instead.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequirementOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Exists,
    DoesNotExist,
    Contains,
    OneOf,
}

/// A validated, open-world identifier for a requirement property.
///
/// Examples:
///
/// - `quantum.measurement`
/// - `cpu.integer`
/// - `gpu.compute`
/// - `hdl.synthesis`
/// - `distributed.execution`
/// - `tensor.shape.rank`
///
/// Names are not restricted to a closed enumeration, so new domains can be
/// introduced without editing this module.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequirementKey(String);

impl RequirementKey {
    /// Creates a key after validating its namespace/name syntax.
    ///
    /// Segments are separated by dots. Each segment must begin with an ASCII
    /// letter or underscore; subsequent characters may also contain digits
    /// and hyphens. Keys are case-sensitive and are not silently normalized.
    pub fn new(value: impl Into<String>) -> RequirementResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(RequirementError::EmptyKey);
        }

        if value.trim() != value {
            return Err(RequirementError::InvalidKey {
                key: value,
                reason: "leading or trailing whitespace is not permitted".into(),
            });
        }

        for segment in value.split('.') {
            if segment.is_empty() {
                return Err(RequirementError::InvalidKey {
                    key: value,
                    reason: "empty namespace segments are not permitted".into(),
                });
            }

            let mut chars = segment.chars();
            let first = chars.next().ok_or(RequirementError::EmptyKey)?;

            if !(first.is_ascii_alphabetic() || first == '_')
                || !chars.all(|c| {
                    c.is_ascii_alphanumeric() || c == '_' || c == '-'
                })
            {
                return Err(RequirementError::InvalidKey {
                    key: value,
                    reason: format!("invalid namespace segment `{segment}`"),
                });
            }
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the top-level namespace segment.
    pub fn namespace(&self) -> &str {
        self.0.split('.').next().unwrap_or(&self.0)
    }
}

impl fmt::Display for RequirementKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A non-negative integer represented as canonical decimal digits.
///
/// Using decimal text avoids imposing `u64` or `u128` as a universal limit on
/// quantities such as byte counts, operation counts, logical resource counts,
/// tensor dimensions, or abstract capacities. This type does not itself imply
/// that a target can physically supply the represented quantity.
///
/// Leading zeros are removed except for the value zero itself.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DecimalQuantity(String);

impl DecimalQuantity {
    pub fn new(value: impl AsRef<str>) -> RequirementResult<Self> {
        let input = value.as_ref();

        if input.is_empty() || !input.bytes().all(|b| b.is_ascii_digit()) {
            return Err(RequirementError::InvalidQuantity(
                input.to_owned(),
            ));
        }

        let canonical = input.trim_start_matches('0');
        let canonical = if canonical.is_empty() {
            "0"
        } else {
            canonical
        };

        Ok(Self(canonical.to_owned()))
    }

    pub fn zero() -> Self {
        Self("0".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Compares non-negative decimal quantities without integer conversion.
    pub fn numeric_cmp(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(&other.0))
    }
}

impl fmt::Display for DecimalQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A requirement value.
///
/// Values are deliberately independent of target-specific capability models.
/// A downstream adapter can translate these values to the canonical types
/// defined by the frontend or capability subsystem.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequirementValue {
    Boolean(bool),
    Quantity(DecimalQuantity),
    Text(String),
    /// An unordered set is represented canonically using sorted unique values.
    TextSet(Vec<String>),
}

impl RequirementValue {
    pub fn quantity(value: impl AsRef<str>) -> RequirementResult<Self> {
        Ok(Self::Quantity(DecimalQuantity::new(value)?))
    }

    pub fn text_set(
        values: impl IntoIterator<Item = String>,
    ) -> Self {
        let mut values: Vec<String> = values.into_iter().collect();
        values.sort();
        values.dedup();
        Self::TextSet(values)
    }

    /// A stable, unambiguous textual representation for fingerprints.
    ///
    /// Length-prefixing prevents ambiguity when values contain punctuation.
    pub fn canonical_string(&self) -> String {
        match self {
            Self::Boolean(value) => format!("bool:{value}"),
            Self::Quantity(value) => format!("quantity:{}", value.as_str()),
            Self::Text(value) => format!("text:{}:{value}", value.len()),
            Self::TextSet(values) => {
                let body = values
                    .iter()
                    .map(|value| format!("{}:{value}", value.len()))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("text-set:{}:{body}", values.len())
            }
        }
    }
}

/// Provenance for a requirement originating in source, an AST, a semantic
/// model, a library, or a generated workload.
///
/// `source_id` is intentionally opaque here. An adapter can preserve the
/// repository's actual source-file ID, `NodeId`, and `Span` without requiring
/// this foundational module to guess their concrete types.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequirementOrigin {
    /// Stable source or model identifier, if known.
    pub source_id: Option<String>,

    /// Opaque node identifier, if the frontend provides one.
    pub node_id: Option<String>,

    /// Inclusive start byte offset, if known.
    pub start_byte: Option<u64>,

    /// Exclusive end byte offset, if known.
    pub end_byte: Option<u64>,

    /// Optional producer, such as `frontend.semantic` or `quantum.analysis`.
    pub producer: Option<String>,
}

impl RequirementOrigin {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a byte range after validating that it is well formed.
    pub fn with_byte_range(
        mut self,
        start: u64,
        end: u64,
    ) -> RequirementResult<Self> {
        if end < start {
            return Err(RequirementError::InvalidSourceRange { start, end });
        }

        self.start_byte = Some(start);
        self.end_byte = Some(end);
        Ok(self)
    }

    pub fn with_source_id(mut self, source_id: impl Into<String>) -> Self {
        self.source_id = Some(source_id.into());
        self
    }

    pub fn with_node_id(mut self, node_id: impl Into<String>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }

    pub fn with_producer(mut self, producer: impl Into<String>) -> Self {
        self.producer = Some(producer.into());
        self
    }
}

/// A single workload requirement.
///
/// `key`, `kind`, `operator`, and `value` are the logical requirement.
/// `origin` and `rationale` preserve explanation and provenance. Requirements
/// with identical logical content are not automatically merged because their
/// origins or roles may differ.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BackendRequirement {
    key: RequirementKey,
    kind: RequirementKind,
    operator: RequirementOperator,
    value: Option<RequirementValue>,
    unit: Option<String>,
    origin: RequirementOrigin,
    rationale: Option<String>,
}

impl BackendRequirement {
    pub fn new(
        key: RequirementKey,
        kind: RequirementKind,
        operator: RequirementOperator,
        value: Option<RequirementValue>,
    ) -> RequirementResult<Self> {
        Self::validate_value(operator, value.as_ref())?;

        Ok(Self {
            key,
            kind,
            operator,
            value,
            unit: None,
            origin: RequirementOrigin::default(),
            rationale: None,
        })
    }

    pub fn required_capability(
        key: RequirementKey,
    ) -> RequirementResult<Self> {
        Self::new(
            key,
            RequirementKind::RequiredCapability,
            RequirementOperator::Equal,
            Some(RequirementValue::Boolean(true)),
        )
    }

    pub fn preference(
        key: RequirementKey,
        value: RequirementValue,
    ) -> RequirementResult<Self> {
        Self::new(
            key,
            RequirementKind::Preference,
            RequirementOperator::Equal,
            Some(value),
        )
    }

    pub fn constraint(
        key: RequirementKey,
        operator: RequirementOperator,
        value: RequirementValue,
    ) -> RequirementResult<Self> {
        Self::new(
            key,
            RequirementKind::Constraint,
            operator,
            Some(value),
        )
    }

    pub fn resource(
        key: RequirementKey,
        operator: RequirementOperator,
        quantity: DecimalQuantity,
        unit: Option<String>,
    ) -> RequirementResult<Self> {
        let mut requirement = Self::new(
            key,
            RequirementKind::Resource,
            operator,
            Some(RequirementValue::Quantity(quantity)),
        )?;
        requirement.unit = unit;
        Ok(requirement)
    }

    fn validate_value(
        operator: RequirementOperator,
        value: Option<&RequirementValue>,
    ) -> RequirementResult<()> {
        match operator {
            RequirementOperator::Exists
            | RequirementOperator::DoesNotExist => {
                if value.is_some() {
                    return Err(RequirementError::UnexpectedValue {
                        operator,
                    });
                }
            }
            _ if value.is_none() => {
                return Err(RequirementError::MissingValue { operator });
            }
            _ => {}
        }

        if matches!(
            operator,
            RequirementOperator::LessThan
                | RequirementOperator::LessThanOrEqual
                | RequirementOperator::GreaterThan
                | RequirementOperator::GreaterThanOrEqual
        ) {
            match value {
                Some(RequirementValue::Quantity(_)) => {}
                Some(_) => {
                    return Err(RequirementError::InvalidOperatorValue {
                        operator,
                        reason:
                            "ordered comparisons require a numeric quantity"
                                .into(),
                    });
                }
                None => {}
            }
        }

        if operator == RequirementOperator::Contains
            && !matches!(value, Some(RequirementValue::Text(_)))
        {
            return Err(RequirementError::InvalidOperatorValue {
                operator,
                reason: "`contains` requires a text value".into(),
            });
        }

        if operator == RequirementOperator::OneOf
            && !matches!(value, Some(RequirementValue::TextSet(_)))
        {
            return Err(RequirementError::InvalidOperatorValue {
                operator,
                reason: "`one-of` requires a set of text values".into(),
            });
        }

        Ok(())
    }

    pub fn key(&self) -> &RequirementKey {
        &self.key
    }

    pub fn kind(&self) -> RequirementKind {
        self.kind
    }

    pub fn operator(&self) -> RequirementOperator {
        self.operator
    }

    pub fn value(&self) -> Option<&RequirementValue> {
        self.value.as_ref()
    }

    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }

    pub fn origin(&self) -> &RequirementOrigin {
        &self.origin
    }

    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn with_origin(mut self, origin: RequirementOrigin) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = Some(rationale.into());
        self
    }

    /// Returns a stable key for deterministic ordering and fingerprinting.
    ///
    /// Source provenance and rationale are excluded from the logical key;
    /// callers that need a provenance-sensitive fingerprint must include them
    /// separately.
    pub fn canonical_key(&self) -> String {
        let value = self
            .value
            .as_ref()
            .map(RequirementValue::canonical_string)
            .unwrap_or_else(|| "none".to_owned());
        let unit = self.unit.as_deref().unwrap_or("");

        format!(
            "{}|{:?}|{:?}|{}|{}:{}",
            self.key,
            self.kind,
            self.operator,
            value,
            unit.len(),
            unit
        )
    }
}

/// An ordered collection of workload requirements.
///
/// The collection intentionally preserves duplicates and contradictory
/// entries. A requirement set is not a solver and must not discard evidence
/// of a contradiction. `canonicalized` produces a separate sorted view for
/// stable fingerprints or reproducible reports.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RequirementSet {
    requirements: Vec<BackendRequirement>,
    metadata: BTreeMap<String, String>,
}

impl RequirementSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a requirement without deduplication.
    pub fn push(
        &mut self,
        requirement: BackendRequirement,
    ) {
        self.requirements.push(requirement);
    }

    /// Adds a requirement and returns the updated set.
    pub fn with(
        mut self,
        requirement: BackendRequirement,
    ) -> Self {
        self.push(requirement);
        self
    }

    /// Appends all requirements from another collection, preserving order and
    /// duplicates. Metadata keys from `other` overwrite matching keys.
    pub fn extend(&mut self, other: Self) {
        self.requirements.extend(other.requirements);
        self.metadata.extend(other.metadata);
    }

    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &BackendRequirement> {
        self.requirements.iter()
    }

    pub fn get(&self, index: usize) -> Option<&BackendRequirement> {
        self.requirements.get(index)
    }

    pub fn for_key<'a>(
        &'a self,
        key: &'a RequirementKey,
    ) -> impl Iterator<Item = &'a BackendRequirement> + 'a {
        self.requirements
            .iter()
            .filter(move |requirement| requirement.key() == key)
    }

    pub fn for_kind(
        &self,
        kind: RequirementKind,
    ) -> impl Iterator<Item = &BackendRequirement> {
        self.requirements
            .iter()
            .filter(move |requirement| requirement.kind() == kind)
    }

    /// Stores non-semantic collection metadata.
    ///
    /// Metadata must not be used to smuggle target capabilities or override
    /// the meaning of a requirement.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.metadata.insert(key.into(), value.into());
    }

    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns a sorted copy without changing the original insertion order.
    ///
    /// Equal logical requirements remain duplicated. Their origin and rationale
    /// are used as deterministic tie-breakers.
    pub fn canonicalized(&self) -> Vec<BackendRequirement> {
        let mut result = self.requirements.clone();
        result.sort_by(|left, right| {
            left.canonical_key()
                .cmp(&right.canonical_key())
                .then_with(|| left.origin().cmp(right.origin()))
                .then_with(|| left.rationale().cmp(&right.rationale()))
        });
        result
    }

    /// Returns requirements with an identical logical key.
    ///
    /// This reports groups; it does not decide whether two requirements
    /// conflict. Conflict evaluation belongs to compatibility/feasibility.
    pub fn duplicate_groups(
        &self,
    ) -> Vec<(String, Vec<usize>)> {
        let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();

        for (index, requirement) in self.requirements.iter().enumerate() {
            groups
                .entry(requirement.canonical_key())
                .or_default()
                .push(index);
        }

        groups
            .into_iter()
            .filter(|(_, indices)| indices.len() > 1)
            .collect()
    }
}

impl IntoIterator for RequirementSet {
    type Item = BackendRequirement;
    type IntoIter = std::vec::IntoIter<BackendRequirement>;

    fn into_iter(self) -> Self::IntoIter {
        self.requirements.into_iter()
    }
}

impl<'a> IntoIterator for &'a RequirementSet {
    type Item = &'a BackendRequirement;
    type IntoIter = std::slice::Iter<'a, BackendRequirement>;

    fn into_iter(self) -> Self::IntoIter {
        self.requirements.iter()
    }
}

/// Errors raised while constructing or validating requirement data.
///
/// These errors concern malformed requirement records only. They do not mean
/// that a target is unsupported or that a workload is infeasible.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequirementError {
    EmptyKey,
    InvalidKey {
        key: String,
        reason: String,
    },
    InvalidQuantity(String),
    InvalidSourceRange {
        start: u64,
        end: u64,
    },
    MissingValue {
        operator: RequirementOperator,
    },
    UnexpectedValue {
        operator: RequirementOperator,
    },
    InvalidOperatorValue {
        operator: RequirementOperator,
        reason: String,
    },
}

impl fmt::Display for RequirementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKey => f.write_str("requirement key must not be empty"),
            Self::InvalidKey { key, reason } => {
                write!(f, "invalid requirement key `{key}`: {reason}")
            }
            Self::InvalidQuantity(value) => {
                write!(
                    f,
                    "invalid non-negative decimal quantity `{value}`"
                )
            }
            Self::InvalidSourceRange { start, end } => {
                write!(
                    f,
                    "invalid source range: end offset {end} precedes start {start}"
                )
            }
            Self::MissingValue { operator } => {
                write!(f, "operator {operator:?} requires a value")
            }
            Self::UnexpectedValue { operator } => {
                write!(f, "operator {operator:?} does not accept a value")
            }
            Self::InvalidOperatorValue { operator, reason } => {
                write!(
                    f,
                    "invalid value for operator {operator:?}: {reason}"
                )
            }
        }
    }
}

impl Error for RequirementError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(value: &str) -> RequirementKey {
        RequirementKey::new(value).expect("valid test key")
    }

    #[test]
    fn accepts_extensible_domain_namespaces() {
        for value in [
            "cpu.integer",
            "quantum.measurement",
            "gpu.compute",
            "hdl.synthesis",
            "distributed.execution",
            "future_domain.new_capability",
        ] {
            assert_eq!(RequirementKey::new(value).unwrap().as_str(), value);
        }
    }

    #[test]
    fn rejects_malformed_requirement_keys() {
        for value in [
            "",
            ".quantum",
            "quantum.",
            "quantum..measurement",
            " quantum.measurement",
            "quantum.measurement ",
            "9cpu.integer",
        ] {
            assert!(RequirementKey::new(value).is_err(), "{value}");
        }
    }

    #[test]
    fn decimal_quantities_are_canonical_and_unbounded_by_machine_integers() {
        let a = DecimalQuantity::new("00042").unwrap();
        let b = DecimalQuantity::new("42").unwrap();
        let huge = DecimalQuantity::new(
            "999999999999999999999999999999999999999999999999999999999999",
        )
        .unwrap();

        assert_eq!(a, b);
        assert_eq!(a.as_str(), "42");
        assert!(huge.numeric_cmp(&a).is_gt());
        assert_eq!(DecimalQuantity::new("000").unwrap().as_str(), "0");
    }

    #[test]
    fn decimal_quantities_reject_signs_and_non_digits() {
        for value in ["-1", "+1", "1.5", "", "1e9", " 12"] {
            assert!(DecimalQuantity::new(value).is_err(), "{value}");
        }
    }

    #[test]
    fn preserves_required_capabilities_as_mandatory_requirements() {
        let requirement =
            BackendRequirement::required_capability(key("quantum.measurement"))
                .unwrap();

        assert_eq!(requirement.kind(), RequirementKind::RequiredCapability);
        assert_eq!(
            requirement.operator(),
            RequirementOperator::Equal
        );
        assert_eq!(
            requirement.value(),
            Some(&RequirementValue::Boolean(true))
        );
    }

    #[test]
    fn rejects_missing_and_unexpected_operator_values() {
        assert!(BackendRequirement::new(
            key("cpu.integer"),
            RequirementKind::Constraint,
            RequirementOperator::Equal,
            None,
        )
        .is_err());

        assert!(BackendRequirement::new(
            key("cpu.integer"),
            RequirementKind::Constraint,
            RequirementOperator::Exists,
            Some(RequirementValue::Boolean(true)),
        )
        .is_err());
    }

    #[test]
    fn preserves_duplicates_and_conflicts_for_downstream_analysis() {
        let first = BackendRequirement::constraint(
            key("memory.bytes"),
            RequirementOperator::GreaterThanOrEqual,
            RequirementValue::quantity("4096").unwrap(),
        )
        .unwrap();

        let second = BackendRequirement::constraint(
            key("memory.bytes"),
            RequirementOperator::LessThanOrEqual,
            RequirementValue::quantity("1024").unwrap(),
        )
        .unwrap();

        let mut requirements = RequirementSet::new();
        requirements.push(first.clone());
        requirements.push(first);
        requirements.push(second);

        assert_eq!(requirements.len(), 3);
        assert_eq!(requirements.duplicate_groups().len(), 1);
    }

    #[test]
    fn canonicalization_is_deterministic_without_mutating_input_order() {
        let a = BackendRequirement::preference(
            key("gpu.compute"),
            RequirementValue::Boolean(true),
        )
        .unwrap();

        let b = BackendRequirement::required_capability(key("cpu.integer"))
            .unwrap();

        let mut requirements = RequirementSet::new();
        requirements.push(a);
        requirements.push(b);

        let original_first = requirements.get(0).unwrap().key().as_str();
        let canonical_first = requirements.canonicalized()[0]
            .key()
            .as_str()
            .to_owned();

        assert_eq!(original_first, "gpu.compute");
        assert_eq!(canonical_first, "cpu.integer");
    }

    #[test]
    fn validates_source_ranges_and_preserves_provenance() {
        assert!(RequirementOrigin::new()
            .with_byte_range(9, 3)
            .is_err());

        let origin = RequirementOrigin::new()
            .with_source_id("module.zm")
            .with_node_id("node-17")
            .with_producer("frontend.semantic")
            .with_byte_range(3, 11)
            .unwrap();

        assert_eq!(origin.source_id.as_deref(), Some("module.zm"));
        assert_eq!(origin.node_id.as_deref(), Some("node-17"));
        assert_eq!(origin.start_byte, Some(3));
        assert_eq!(origin.end_byte, Some(11));
    }

    #[test]
    fn text_sets_have_canonical_order_and_no_duplicate_values() {
        let value = RequirementValue::text_set(vec![
            "wasm".to_owned(),
            "native".to_owned(),
            "wasm".to_owned(),
        ]);

        assert_eq!(
            value,
            RequirementValue::TextSet(vec![
                "native".to_owned(),
                "wasm".to_owned(),
            ])
        );
    }
}
```
