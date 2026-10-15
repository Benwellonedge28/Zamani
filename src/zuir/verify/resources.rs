
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Resource Verification
//!
//! Canonical path: `src/zuir/verify/resources.rs`
//! Edition: Rust 2021.
//! Minimum intended Rust version: 1.97.
//! Safety: safe Rust only.
//!
//! PURPOSE
//!
//! Verify resource-intent declarations using the canonical resource model
//! in `crate::zuir::ir::resources`.
//!
//! OWNERSHIP
//!
//! This module owns cross-intent consistency diagnostics and verification
//! reports. It does not own resource declarations, target inventories,
//! resource allocation, unit conversion, or resource estimation.
//!
//! AUTHORITY
//!
//! - `src/zuir/ir/resources.rs`
//! - `src/zuir/errors.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/spec/resources.md`
//! - `grammar/resources/constraints.g4`
//! - `grammar/resources/budgets.g4`
//! - `grammar/resources/capabilities.g4`
//! - `grammar/specification/scalability-model.md`
//!
//! SCALABILITY
//!
//! This module imposes no fixed resource, assertion, capability, topology,
//! or hardware-population limit. It uses dynamically sized collections and
//! ordered indexes. Work is bounded by available execution resources.
//!
//! SEMANTIC BOUNDARY
//!
//! - Requirements and constraints are checked as mandatory assertions.
//! - Preferences and hints are not silently promoted to requirements.
//! - The existing ResourceIntent API determines which assertions are
//!   considered mandatory; budget semantics remain owned by that model.
//! - Different units are never compared without a conversion contract.
//! - Unknown extension semantics are not inferred from their names.
//! - Target feasibility and physical allocation belong to later stages.
//!
//! INTEGRATION
//!
//! The semantic-model-to-ZUIR adapter supplies resource intents and their
//! logical scopes. The full verifier invokes this module. Resource analysis
//! derives estimates; lowering and target adapters determine feasibility.
//!
//! This module does not require changes to the canonical resource model.

use std::collections::{BTreeMap, BTreeSet};

use crate::zuir::ir::resources::{
    CapabilityIntent,
    ResourceAssertion,
    ResourceIntent,
    ResourceIntentKind,
    ResourceKey,
    ResourceQuantity,
    ResourceRelation,
    TopologyIntent,
    TopologyRelation,
};

// =============================================================================
// Public diagnostics
// =============================================================================

/// Severity of a resource-verification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceIssueSeverity {
    /// A proven violation of a checked invariant.
    Error,

    /// A condition that deserves attention but is not itself a violation.
    Warning,

    /// Verification needs an external contract or additional information.
    Deferred,
}

/// One resource-verification finding.
///
/// `code` is the machine-readable diagnostic identifier. Consumers should
/// branch on it rather than parse the human-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceIssue {
    code: &'static str,
    severity: ResourceIssueSeverity,
    scope: String,
    message: String,
}

impl ResourceIssue {
    fn new(
        code: &'static str,
        severity: ResourceIssueSeverity,
        scope: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            scope: scope.into(),
            message: message.into(),
        }
    }

    /// Return the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    /// Return the finding severity.
    #[must_use]
    pub const fn severity(&self) -> ResourceIssueSeverity {
        self.severity
    }

    /// Return the logical scope supplied by the caller.
    #[must_use]
    pub fn scope(&self) -> &str {
        &self.scope
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// =============================================================================
// Input and report
// =============================================================================

/// A resource intent together with its caller-defined logical scope.
///
/// Intents with the same scope are checked together for contradictions.
/// Scope names are labels, not resolved IR identities. The caller must use
/// the same scope label only for declarations that constrain the same
/// logical resource environment.
///
/// For example, declarations inherited into one effective function scope
/// may be supplied under the same scope label. Unrelated functions should
/// not be grouped merely because they have the same source-level name.
#[derive(Debug, Clone, Copy)]
pub struct ResourceIntentEntry<'a> {
    /// Stable-for-this-verification logical scope label.
    pub scope: &'a str,

    /// Canonical resource intent to verify.
    pub intent: &'a ResourceIntent,
}

impl<'a> ResourceIntentEntry<'a> {
    /// Construct one scoped resource-intent entry.
    #[must_use]
    pub const fn new(scope: &'a str, intent: &'a ResourceIntent) -> Self {
        Self { scope, intent }
    }
}

/// Summary of a resource-verification run.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResourceVerificationReport {
    issues: Vec<ResourceIssue>,
    intents_checked: usize,
    assertions_checked: usize,
    capabilities_checked: usize,
    topology_relations_checked: usize,
}

impl ResourceVerificationReport {
    /// Borrow findings in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[ResourceIssue] {
        &self.issues
    }

    /// Whether the checks performed found no errors.
    ///
    /// Deferred findings do not count as proven violations, so callers that
    /// require complete semantic verification must also check `is_complete`.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| issue.severity == ResourceIssueSeverity::Error)
    }

    /// Whether this run has neither errors nor deferred checks.
    ///
    /// Warnings may still be present.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        !self.issues.iter().any(|issue| {
            matches!(
                issue.severity,
                ResourceIssueSeverity::Error
                    | ResourceIssueSeverity::Deferred
            )
        })
    }

    /// Number of reported errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ResourceIssueSeverity::Error)
            .count()
    }

    /// Number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ResourceIssueSeverity::Warning)
            .count()
    }

    /// Number of checks deferred to another subsystem.
    #[must_use]
    pub fn deferred_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ResourceIssueSeverity::Deferred)
            .count()
    }

    /// Number of intents inspected.
    #[must_use]
    pub const fn intents_checked(&self) -> usize {
        self.intents_checked
    }

    /// Number of assertions inspected.
    #[must_use]
    pub const fn assertions_checked(&self) -> usize {
        self.assertions_checked
    }

    /// Number of capability declarations inspected.
    #[must_use]
    pub const fn capabilities_checked(&self) -> usize {
        self.capabilities_checked
    }

    /// Number of topology relations inspected.
    #[must_use]
    pub const fn topology_relations_checked(&self) -> usize {
        self.topology_relations_checked
    }

    fn push(
        &mut self,
        code: &'static str,
        severity: ResourceIssueSeverity,
        scope: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.issues.push(ResourceIssue::new(
            code, severity, scope, message,
        ));
    }
}

// =============================================================================
// Internal bound model
// =============================================================================

/// One inclusive or exclusive numeric bound.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Bound {
    value: crate::zuir::ir::resources::DecimalMagnitude,
    exclusive: bool,
}

/// Intersection of mandatory assertions for one dimension and unit.
///
/// Bounds are combined in O(n log n) total time for each verification run
/// when grouped with the ordered indexes below, rather than comparing every
/// assertion with every other assertion.
#[derive(Debug, Clone, Default)]
struct Bounds {
    lower: Option<Bound>,
    upper: Option<Bound>,
}

impl Bounds {
    fn add(&mut self, assertion: &ResourceAssertion) {
        let value = assertion.quantity().magnitude().clone();

        match assertion.relation() {
            ResourceRelation::AtLeast => {
                self.update_lower(value, false);
            }
            ResourceRelation::GreaterThan => {
                self.update_lower(value, true);
            }
            ResourceRelation::AtMost => {
                self.update_upper(value, false);
            }
            ResourceRelation::LessThan => {
                self.update_upper(value, true);
            }
            ResourceRelation::Exactly => {
                self.update_lower(value.clone(), false);
                self.update_upper(value, false);
            }
        }
    }

    fn update_lower(
        &mut self,
        value: crate::zuir::ir::resources::DecimalMagnitude,
        exclusive: bool,
    ) {
        match &mut self.lower {
            None => {
                self.lower = Some(Bound { value, exclusive });
            }
            Some(current) => match value.cmp(&current.value) {
                std::cmp::Ordering::Greater => {
                    *current = Bound { value, exclusive };
                }
                std::cmp::Ordering::Equal => {
                    current.exclusive |= exclusive;
                }
                std::cmp::Ordering::Less => {}
            },
        }
    }

    fn update_upper(
        &mut self,
        value: crate::zuir::ir::resources::DecimalMagnitude,
        exclusive: bool,
    ) {
        match &mut self.upper {
            None => {
                self.upper = Some(Bound { value, exclusive });
            }
            Some(current) => match value.cmp(&current.value) {
                std::cmp::Ordering::Less => {
                    *current = Bound { value, exclusive };
                }
                std::cmp::Ordering::Equal => {
                    current.exclusive |= exclusive;
                }
                std::cmp::Ordering::Greater => {}
            },
        }
    }

    fn is_contradictory(&self) -> bool {
        let (Some(lower), Some(upper)) = (&self.lower, &self.upper) else {
            return false;
        };

        match lower.value.cmp(&upper.value) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => lower.exclusive || upper.exclusive,
        }
    }
}

// =============================================================================
// Entry point
// =============================================================================

/// Verify a collection of scoped resource intents.
///
/// Checks:
///
/// - local validation of assertions, capabilities, topology, and metadata;
/// - contradictory mandatory bounds for the same scope, dimension, and unit;
/// - incompatible mandatory topology relations for the same scope and pair;
/// - declarations whose semantics require an external registry or evaluator.
///
/// This function does not query hardware, estimate actual consumption,
/// allocate resources, convert units, or establish target feasibility.
///
/// Results are deterministic for the same entries in the same order and
/// with the same canonical input data. Entries are not silently rewritten.
#[must_use]
pub fn verify_resources(
    entries: &[ResourceIntentEntry<'_>],
) -> ResourceVerificationReport {
    let mut report = ResourceVerificationReport::default();

    let mut bounds_by_scope:
        BTreeMap<(String, ResourceKey, ResourceKey), Bounds> =
        BTreeMap::new();

    let mut units_by_scope_dimension:
        BTreeMap<(String, ResourceKey), BTreeSet<ResourceKey>> =
        BTreeMap::new();

    let mut topology_by_scope:
        BTreeMap<(String, ResourceKey, ResourceKey), TopologyFlags> =
        BTreeMap::new();

    for entry in entries {
        report.intents_checked += 1;

        if let Err(error) = entry.intent.validate() {
            report.push(
                "ZUIR-RESOURCE-INVALID-INTENT",
                ResourceIssueSeverity::Error,
                entry.scope,
                format!("resource intent failed local validation: {error}"),
            );
        }

        verify_metadata(entry, &mut report);

        for assertion in entry.intent.assertions() {
            report.assertions_checked += 1;

            if let Err(error) = assertion.validate() {
                report.push(
                    "ZUIR-RESOURCE-INVALID-ASSERTION",
                    ResourceIssueSeverity::Error,
                    entry.scope,
                    format!("resource assertion is invalid: {error}"),
                );
                continue;
            }

            if !is_mandatory(assertion.kind()) {
                continue;
            }

            let dimension = assertion.dimension().clone();
            let unit = assertion.quantity().unit().clone();

            units_by_scope_dimension
                .entry((entry.scope.to_owned(), dimension.clone()))
                .or_default()
                .insert(unit.clone());

            bounds_by_scope
                .entry((entry.scope.to_owned(), dimension, unit))
                .or_default()
                .add(assertion);
        }

        for capability in entry.intent.capabilities() {
            report.capabilities_checked += 1;
            verify_capability(capability, entry.scope, &mut report);
        }

        for topology in entry.intent.topology() {
            report.topology_relations_checked += 1;
            verify_topology(topology, entry.scope, &mut report);

            if is_mandatory(topology.kind()) {
                let (left, right) = ordered_pair(
                    topology.left().clone(),
                    topology.right().clone(),
                );

                let flags = topology_by_scope
                    .entry((entry.scope.to_owned(), left, right))
                    .or_default();

                match topology.relation() {
                    TopologyRelation::CoLocate => flags.co_locate = true,
                    TopologyRelation::Separate => flags.separate = true,
                    _ => {}
                }
            }
        }
    }

    verify_bounds(&bounds_by_scope, &mut report);
    verify_mixed_units(&units_by_scope_dimension, &mut report);
    verify_topology_conflicts(&topology_by_scope, &mut report);

    report
}

// =============================================================================
// Local validation
// =============================================================================

fn verify_metadata(
    entry: &ResourceIntentEntry<'_>,
    report: &mut ResourceVerificationReport,
) {
    for key in entry.intent.metadata().keys() {
        if let Err(error) = key.validate() {
            report.push(
                "ZUIR-RESOURCE-INVALID-METADATA-KEY",
                ResourceIssueSeverity::Error,
                entry.scope,
                format!("resource metadata key is invalid: {error}"),
            );
        }
    }
}

fn verify_capability(
    capability: &CapabilityIntent,
    scope: &str,
    report: &mut ResourceVerificationReport,
) {
    if let Err(error) = capability.validate() {
        report.push(
            "ZUIR-RESOURCE-INVALID-CAPABILITY",
            ResourceIssueSeverity::Error,
            scope,
            format!("capability declaration is invalid: {error}"),
        );
    }

    // Capability registration and target availability are intentionally
    // not inferred from a well-formed name. The dialect/capability registry
    // and target feasibility analysis own those checks.
}

fn verify_topology(
    topology: &TopologyIntent,
    scope: &str,
    report: &mut ResourceVerificationReport,
) {
    if let Err(error) = topology.validate() {
        report.push(
            "ZUIR-RESOURCE-INVALID-TOPOLOGY",
            ResourceIssueSeverity::Error,
            scope,
            format!("topology declaration is invalid: {error}"),
        );
    }

    if topology.relation() == TopologyRelation::Extension {
        report.push(
            "ZUIR-RESOURCE-EXTENSION-TOPOLOGY-DEFERRED",
            ResourceIssueSeverity::Deferred,
            scope,
            format!(
                "extension topology relation `{}` requires its registered \
                 semantic contract to be checked",
                topology
                    .extension_relation()
                    .map(ResourceKey::as_str)
                    .unwrap_or("<missing>")
            ),
        );
    }
}

fn is_mandatory(kind: ResourceIntentKind) -> bool {
    matches!(
        kind,
        ResourceIntentKind::Requirement | ResourceIntentKind::Constraint
    )
}

// =============================================================================
// Mandatory assertion consistency
// =============================================================================

fn verify_bounds(
    bounds_by_scope: &BTreeMap<(String, ResourceKey, ResourceKey), Bounds>,
    report: &mut ResourceVerificationReport,
) {
    for ((scope, dimension, unit), bounds) in bounds_by_scope {
        if bounds.is_contradictory() {
            report.push(
                "ZUIR-RESOURCE-CONTRADICTORY-BOUNDS",
                ResourceIssueSeverity::Error,
                scope.clone(),
                format!(
                    "mandatory resource assertions for `{dimension}` with \
                     unit `{unit}` have an empty intersection"
                ),
            );
        }
    }
}

fn verify_mixed_units(
    units_by_scope_dimension:
        &BTreeMap<(String, ResourceKey), BTreeSet<ResourceKey>>,
    report: &mut ResourceVerificationReport,
) {
    for ((scope, dimension), units) in units_by_scope_dimension {
        if units.len() > 1 {
            report.push(
                "ZUIR-RESOURCE-UNIT-COMPARISON-DEFERRED",
                ResourceIssueSeverity::Deferred,
                scope.clone(),
                format!(
                    "mandatory assertions for `{dimension}` use multiple \
                     units ({}); no cross-unit comparison was attempted",
                    units
                        .iter()
                        .map(ResourceKey::as_str)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            );
        }
    }
}

// =============================================================================
// Topology consistency
// =============================================================================

#[derive(Debug, Clone, Copy, Default)]
struct TopologyFlags {
    co_locate: bool,
    separate: bool,
}

fn ordered_pair(
    left: ResourceKey,
    right: ResourceKey,
) -> (ResourceKey, ResourceKey) {
    if left <= right {
        (left, right)
    } else {
        (right, left)
    }
}

fn verify_topology_conflicts(
    topology_by_scope:
        &BTreeMap<(String, ResourceKey, ResourceKey), TopologyFlags>,
    report: &mut ResourceVerificationReport,
) {
    for ((scope, left, right), flags) in topology_by_scope {
        if flags.co_locate && flags.separate {
            report.push(
                "ZUIR-RESOURCE-CONTRADICTORY-TOPOLOGY",
                ResourceIssueSeverity::Error,
                scope.clone(),
                format!(
                    "logical resource groups `{left}` and `{right}` are \
                     required to be both co-located and separate"
                ),
            );
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ir::resources::{
        DecimalMagnitude,
        ResourceAssertion,
        ResourceIntent,
        ResourceIntentKind,
        ResourceKey,
        ResourceQuantity,
        ResourceRelation,
        TopologyIntent,
    };

    fn key(value: &str) -> ResourceKey {
        ResourceKey::new(value).expect("valid resource key")
    }

    fn quantity(value: &str, unit: &str) -> ResourceQuantity {
        ResourceQuantity::new(
            DecimalMagnitude::parse(value).expect("valid magnitude"),
            key(unit),
        )
        .expect("valid quantity")
    }

    fn assertion(
        dimension: &str,
        relation: ResourceRelation,
        value: &str,
        unit: &str,
    ) -> ResourceAssertion {
        ResourceAssertion::new(
            key(dimension),
            relation,
            quantity(value, unit),
            ResourceIntentKind::Requirement,
        )
        .expect("valid assertion")
    }

    fn run(intent: &ResourceIntent) -> ResourceVerificationReport {
        verify_resources(&[ResourceIntentEntry::new("test-scope", intent)])
    }

    #[test]
    fn accepts_empty_intent() {
        let intent = ResourceIntent::new();
        let report = run(&intent);

        assert!(report.is_valid(), "{:?}", report.issues());
        assert!(report.is_complete(), "{:?}", report.issues());
    }

    #[test]
    fn detects_conflicting_inclusive_bounds() {
        let mut intent = ResourceIntent::new();

        intent
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtLeast,
                "16",
                "GiB",
            ))
            .expect("add lower bound");

        intent
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtMost,
                "8",
                "GiB",
            ))
            .expect("add upper bound");

        let report = run(&intent);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.code() == "ZUIR-RESOURCE-CONTRADICTORY-BOUNDS"
        }));
    }

    #[test]
    fn detects_empty_interval_from_strict_bounds() {
        let mut intent = ResourceIntent::new();

        intent
            .add_assertion(assertion(
                "compute.parallelism",
                ResourceRelation::GreaterThan,
                "4",
                "units",
            ))
            .expect("add strict lower bound");

        intent
            .add_assertion(assertion(
                "compute.parallelism",
                ResourceRelation::AtMost,
                "4",
                "units",
            ))
            .expect("add upper bound");

        assert!(!run(&intent).is_valid());
    }

    #[test]
    fn accepts_compatible_bounds() {
        let mut intent = ResourceIntent::new();

        intent
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtLeast,
                "8",
                "GiB",
            ))
            .expect("add lower bound");

        intent
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtMost,
                "16",
                "GiB",
            ))
            .expect("add upper bound");

        let report = run(&intent);

        assert!(report.is_valid(), "{:?}", report.issues());
        assert!(report.is_complete(), "{:?}", report.issues());
    }

    #[test]
    fn does_not_compare_different_units_without_a_registry() {
        let mut intent = ResourceIntent::new();

        intent
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtLeast,
                "8",
                "GiB",
            ))
            .expect("add GiB assertion");

        intent
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtMost,
                "1",
                "TiB",
            ))
            .expect("add TiB assertion");

        let report = run(&intent);

        assert!(report.is_valid(), "{:?}", report.issues());
        assert!(!report.is_complete());
        assert!(report.issues().iter().any(|issue| {
            issue.code() == "ZUIR-RESOURCE-UNIT-COMPARISON-DEFERRED"
        }));
    }

    #[test]
    fn checks_only_assertions_that_the_model_marks_mandatory() {
        let mut intent = ResourceIntent::new();

        intent
            .add_assertion(
                ResourceAssertion::new(
                    key("memory.capacity"),
                    ResourceRelation::AtLeast,
                    quantity("16", "GiB"),
                    ResourceIntentKind::Preference,
                )
                .expect("valid preference"),
            )
            .expect("add preference");

        intent
            .add_assertion(
                ResourceAssertion::new(
                    key("memory.capacity"),
                    ResourceRelation::AtMost,
                    quantity("8", "GiB"),
                    ResourceIntentKind::Hint,
                )
                .expect("valid hint"),
            )
            .expect("add hint");

        let report = run(&intent);

        assert!(report.is_valid(), "{:?}", report.issues());
    }

    #[test]
    fn detects_contradictory_topology_intents() {
        let mut intent = ResourceIntent::new();

        intent
            .add_topology(
                TopologyIntent::new(
                    TopologyRelation::CoLocate,
                    key("group.alpha"),
                    key("group.beta"),
                    ResourceIntentKind::Constraint,
                )
                .expect("valid co-location"),
            )
            .expect("add co-location");

        intent
            .add_topology(
                TopologyIntent::new(
                    TopologyRelation::Separate,
                    key("group.beta"),
                    key("group.alpha"),
                    ResourceIntentKind::Requirement,
                )
                .expect("valid separation"),
            )
            .expect("add separation");

        let report = run(&intent);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.code() == "ZUIR-RESOURCE-CONTRADICTORY-TOPOLOGY"
        }));
    }

    #[test]
    fn unrelated_scopes_are_not_combined() {
        let mut lower = ResourceIntent::new();
        lower
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtLeast,
                "16",
                "GiB",
            ))
            .expect("add lower bound");

        let mut upper = ResourceIntent::new();
        upper
            .add_assertion(assertion(
                "memory.capacity",
                ResourceRelation::AtMost,
                "8",
                "GiB",
            ))
            .expect("add upper bound");

        let entries = [
            ResourceIntentEntry::new("function-a", &lower),
            ResourceIntentEntry::new("function-b", &upper),
        ];

        let report = verify_resources(&entries);

        assert!(report.is_valid(), "{:?}", report.issues());
    }
}
