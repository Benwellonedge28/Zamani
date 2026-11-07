#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Requirement Analysis
//!
//! Path: `src/compiler/analysis/requirements.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Provide the compiler-facing boundary for consuming, preserving, organizing,
//! and validating requirements established by canonical semantic analysis.
//!
//! The compiler must know what a program requires before it can evaluate
//! possible realizations. It must not redefine the meaning of those
//! requirements.
//!
//! This module therefore distinguishes:
//!
//! - canonical semantic resource requirements;
//! - canonical semantic capability requirements;
//! - compiler-facing requirement metadata retained for compatibility;
//! - requirement extraction failures;
//! - incomplete requirement information.
//!
//! ## AUTHORITY
//!
//! Language syntax:
//! - `grammar/Zamani.g4`
//! - `grammar/core/requirements.g4`
//! - `grammar/resources/requirements.g4`
//! - `grammar/core/names.g4`
//! - `grammar/core/capabilities.g4`
//!
//! Normative semantic contracts:
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! Canonical semantic implementations:
//! - `crate::frontend::semantic::resources`
//! - `crate::frontend::semantic::capabilities`
//!
//! Downstream compiler consumers:
//! - `crate::compiler::analysis::capabilities`
//! - `crate::compiler::analysis::feasibility`
//! - `crate::compiler::planning`
//! - `crate::compiler::backends`
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - compiler-facing aggregation of canonical requirement information;
//! - preservation of canonical resource requirements;
//! - preservation of canonical capability requirements;
//! - validation of compiler-side requirement collections;
//! - requirement collection compatibility types;
//! - deterministic presentation of compiler-owned requirement records;
//! - explicit reporting of incomplete resource extraction.
//!
//! ## DOES NOT OWN
//!
//! This module does not own:
//!
//! - source-language syntax;
//! - AST declarations or source-node identity;
//! - semantic name resolution;
//! - semantic type checking;
//! - semantic resource quantities or units;
//! - semantic capability identities;
//! - capability registries or evidence;
//! - constraint interpretation or solving;
//! - preference evaluation;
//! - hint interpretation;
//! - target discovery;
//! - hardware inventories;
//! - target compatibility;
//! - target feasibility;
//! - physical resource allocation;
//! - scheduling, routing, placement, or execution;
//! - domain-specific IR;
//! - backend code generation.
//!
//! ## REQUIRED CONCEPTUAL DISTINCTIONS
//!
//! Requirement:
//!     What the program needs.
//!
//! Capability:
//!     What an implementation or execution environment can provide.
//!
//! Constraint:
//!     What an admissible realization must obey.
//!
//! Preference:
//!     What is desirable but not mandatory.
//!
//! Hint:
//!     Optional implementation guidance.
//!
//! Feasibility:
//!     Whether a proposed realization satisfies applicable requirements,
//!     capabilities, constraints, policies, and resource conditions.
//!
//! These concepts must never be collapsed into one decision or one
//! interchangeable collection.
//!
//! In particular:
//!
//! - A preference is not a mandatory requirement.
//! - A hint is not a correctness condition.
//! - A capability declaration is not evidence that a target supports it.
//! - A requirement is not evidence that a target can satisfy it.
//! - An unresolved condition is not a satisfied condition.
//! - An incomplete collection is not a complete analysis.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! The intended objective is:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! This module imposes no language-level maximum on:
//!
//! - resource requirement counts;
//! - capability requirement counts;
//! - resource quantities;
//! - namespaces;
//! - computational domains;
//! - hardware populations;
//! - distributed participants;
//! - memory capacities;
//! - quantum resources;
//! - tensor dimensions;
//! - future computing resources.
//!
//! It does not convert arbitrary semantic resource quantities into
//! `usize`, `u64`, `u128`, or floating-point values.
//!
//! Semantic quantities remain owned by the canonical semantic model.
//!
//! Collection sizes are bounded by available memory and any explicitly
//! configured operational compilation budget. Those operational limits
//! must not become language-level hardware ceilings.
//!
//! No finite implementation can process literally infinite input.
//!
//! ## DETERMINISM
//!
//! Compiler-owned keyed collections use ordered maps.
//!
//! Canonical semantic resource requirements retain their supplied order.
//! Independent source requirements must not be merged merely because they
//! have equal logical values.
//!
//! Deterministic ordering must not depend on hash-map iteration, hardware
//! discovery order, thread completion order, or timestamps.
//!
//! ## ERROR HANDLING
//!
//! Invalid compiler-owned identities and conflicting source-node reuse
//! produce explicit errors.
//!
//! Canonical semantic extraction failures remain visible in the returned
//! analysis object.
//!
//! A partial extraction must never be reported as complete.
//!
//! ## CONCURRENCY
//!
//! The types in this module contain owned data and do not use global mutable
//! state. Independent compilation sessions may construct independent
//! requirement collections.
//!
//! Synchronization and shared mutable compilation state belong to the
//! compilation context and session.
//!
//! ## SECURITY
//!
//! This module performs no I/O, network access, process execution, hardware
//! discovery, or source-code execution.
//!
//! It does not trust a requirement as evidence of target capability.
//!
//! No unsafe Rust is permitted.
//!
//! ## INTEGRATION
//!
//! Canonical resource requirements:
//! `frontend::semantic::resources::model::ResourceRequirement`
//!
//! Canonical resource extraction:
//! `frontend::semantic::resources::requirements::RequirementExtractionReport`
//!
//! Canonical capability requirements:
//! `frontend::semantic::capabilities::requirements::CapabilityRequirements`
//!
//! Compiler capability analysis:
//! `compiler::analysis::capabilities`
//!
//! Feasibility analysis:
//! `compiler::analysis::feasibility`
//!
//! Target capability matching:
//! `compiler::planning::capability_matching`
//!
//! Backend compatibility:
//! `backend::compatibility`
//!
//! This module does not invoke those downstream systems. It provides the
//! validated requirement information they consume.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;

use crate::frontend::semantic::resources::model::{
    ResourceIntent,
    ResourceRequirement,
};

use crate::frontend::semantic::resources::requirements::{
    RequirementExtractionError,
    RequirementExtractionReport,
};

use crate::frontend::semantic::capabilities::requirements::{
    CapabilityRequirements,
};

/// Schema version of the compiler-owned requirement compatibility model.
///
/// This is not the Zamani language version, semantic-model version,
/// AST version, IR version, or artifact version.
pub const REQUIREMENT_MODEL_SCHEMA_VERSION: u16 = 2;

/// Stable architectural identity for this module.
pub const REQUIREMENTS_MODULE_NAME: &str =
    "zamani.compiler.analysis.requirements";

/// A stable, open, namespaced compiler requirement identifier.
///
/// Identifiers are case-sensitive and are not restricted to a built-in
/// collection of computational domains.
///
/// Examples:
///
/// - `classical.integer_arithmetic`
/// - `quantum.measurement`
/// - `quantum.mid_circuit_measurement`
/// - `ai.tensor_operation`
/// - `hdl.synthesis`
/// - `distributed.execution`
/// - `networking.communication`
/// - `future.photonic.coherent_operation`
///
/// These examples are not an exhaustive registry.
///
/// New domains and requirement identities must not require modification
/// of this type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequirementId(String);

impl RequirementId {
    /// Creates a validated requirement identifier.
    ///
    /// Empty or whitespace-only identifiers and identifiers containing
    /// control characters are rejected.
    ///
    /// Names are otherwise preserved exactly. This constructor does not
    /// perform semantic name resolution, case folding, alias expansion,
    /// or domain registration.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(RequirementError::EmptyIdentifier);
        }

        if value.chars().any(char::is_control) {
            return Err(RequirementError::ControlCharacter {
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier exactly as stored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for RequirementId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Compatibility classification for compiler-owned requirement records.
///
/// This type is retained for callers of the original compiler requirement
/// API. It is not a replacement for canonical semantic resource intent,
/// capability requirements, constraints, preferences, or hints.
///
/// New semantic analysis must retain the canonical semantic types.
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
pub enum RequirementStrength {
    /// The property is mandatory under the compiler-owned record contract.
    Hard,

    /// The property is desirable but is not a semantic correctness condition.
    Preferred,

    /// The property is advisory implementation guidance.
    Advisory,
}

/// A canonical exact, non-negative decimal quantity for compatibility
/// records.
///
/// This compatibility type is not the canonical semantic resource quantity.
/// New semantic resource requirements must use
/// `frontend::semantic::resources::model::ExactQuantity`.
///
/// The representation is decimal text to avoid introducing a machine-sized
/// integer ceiling.
///
/// Accepted syntax:
///
/// - `DIGITS`
/// - `DIGITS "." DIGITS`
///
/// Leading integer zeroes and trailing fractional zeroes are removed.
/// Zero is represented as `"0"`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DecimalQuantity(String);

impl DecimalQuantity {
    /// Creates and canonicalizes an exact decimal quantity.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RequirementError::InvalidDecimal {
                value,
            });
        }

        let mut parts = value.split('.');

        let integer = parts.next().unwrap_or_default();
        let fraction = parts.next();

        if parts.next().is_some()
            || integer.is_empty()
            || !integer.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(RequirementError::InvalidDecimal {
                value,
            });
        }

        if let Some(fraction) = fraction {
            if fraction.is_empty()
                || !fraction.bytes().all(|byte| byte.is_ascii_digit())
            {
                return Err(RequirementError::InvalidDecimal {
                    value,
                });
            }
        }

        let integer = integer.trim_start_matches('0');

        let integer = if integer.is_empty() {
            "0"
        } else {
            integer
        };

        let fraction = fraction
            .unwrap_or_default()
            .trim_end_matches('0');

        let canonical = if fraction.is_empty() {
            integer.to_owned()
        } else {
            format!("{integer}.{fraction}")
        };

        Ok(Self(canonical))
    }

    /// Returns the canonical decimal representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether the quantity is exactly zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0 == "0"
    }

    /// Compares two decimal quantities numerically without converting them
    /// into a bounded integer or floating-point representation.
    #[must_use]
    pub fn numeric_cmp(
        &self,
        other: &Self,
    ) -> core::cmp::Ordering {
        let (left_integer, left_fraction) =
            split_decimal(&self.0);

        let (right_integer, right_fraction) =
            split_decimal(&other.0);

        let integer_order = left_integer
            .len()
            .cmp(&right_integer.len())
            .then_with(|| left_integer.cmp(right_integer));

        if integer_order != core::cmp::Ordering::Equal {
            return integer_order;
        }

        let width = left_fraction
            .len()
            .max(right_fraction.len());

        for index in 0..width {
            let left = left_fraction
                .as_bytes()
                .get(index)
                .copied()
                .unwrap_or(b'0');

            let right = right_fraction
                .as_bytes()
                .get(index)
                .copied()
                .unwrap_or(b'0');

            let ordering = left.cmp(&right);

            if ordering != core::cmp::Ordering::Equal {
                return ordering;
            }
        }

        core::cmp::Ordering::Equal
    }
}

fn split_decimal(
    value: &str,
) -> (&str, &str) {
    match value.split_once('.') {
        Some((integer, fraction)) => (integer, fraction),
        None => (value, ""),
    }
}

impl fmt::Display for DecimalQuantity {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Compatibility value for compiler-owned requirement records.
///
/// This is intentionally not a replacement for the canonical semantic
/// resource quantity or symbolic expression representation.
///
/// New semantic integrations must retain their original canonical values
/// instead of converting them into this compatibility enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RequirementValue {
    /// A boolean property.
    Boolean(bool),

    /// An exact decimal quantity.
    Quantity(DecimalQuantity),

    /// An opaque textual identity.
    Text(String),

    /// A symbolic expression reference owned by semantic analysis.
    Symbolic {
        /// Canonical semantic expression identifier.
        expression_id: String,
    },
}

impl RequirementValue {
    /// Creates a textual requirement value.
    #[must_use]
    pub fn text(
        value: impl Into<String>,
    ) -> Self {
        Self::Text(value.into())
    }

    /// Creates a symbolic requirement value.
    ///
    /// The expression identifier must be supplied by the canonical semantic
    /// authority. This constructor does not parse or evaluate expressions.
    pub fn symbolic(
        expression_id: impl Into<String>,
    ) -> Result<Self, RequirementError> {
        let expression_id = expression_id.into();

        if expression_id.trim().is_empty() {
            return Err(RequirementError::EmptyExpressionId);
        }

        Ok(Self::Symbolic {
            expression_id,
        })
    }
}

/// Provenance metadata for a compiler-owned compatibility requirement.
///
/// Canonical semantic resource and capability requirements retain their
/// original AST identities and provenance in their owning semantic types.
/// This structure is only for compatibility records that do not already
/// have a canonical semantic provenance representation.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct RequirementProvenance {
    /// Optional originating semantic node identifier.
    pub semantic_node_id: Option<String>,

    /// Optional originating module identity.
    pub module_id: Option<String>,

    /// Optional explanation for the requirement.
    pub explanation: Option<String>,
}

impl RequirementProvenance {
    /// Creates empty provenance.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }
}

/// A compiler-owned compatibility requirement record.
///
/// This type preserves the original public construction API. It must not be
/// used to replace canonical semantic resource or capability requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Requirement {
    /// Stable requirement identity.
    pub id: RequirementId,

    /// Compatibility classification.
    pub strength: RequirementStrength,

    /// Compatibility value.
    pub value: RequirementValue,

    /// Compatibility provenance.
    pub provenance: RequirementProvenance,
}

impl Requirement {
    /// Creates a compatibility requirement record.
    #[must_use]
    pub fn new(
        id: RequirementId,
        strength: RequirementStrength,
        value: RequirementValue,
    ) -> Self {
        Self {
            id,
            strength,
            value,
            provenance: RequirementProvenance::empty(),
        }
    }

    /// Attaches compatibility provenance.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: RequirementProvenance,
    ) -> Self {
        self.provenance = provenance;
        self
    }

    /// Returns whether the compatibility record is mandatory.
    #[must_use]
    pub fn is_hard(&self) -> bool {
        self.strength == RequirementStrength::Hard
    }
}

/// A deterministic collection of compiler-owned compatibility records.
///
/// Duplicate identifiers are rejected in this legacy compatibility
/// collection. This does not mean that independent semantic requirements
/// sharing a logical identity may be discarded.
///
/// Canonical semantic requirements are stored separately in
/// [`SemanticRequirements`] and retain their source-node identity.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RequirementSet {
    requirements: BTreeMap<RequirementId, Requirement>,
}

impl RequirementSet {
    /// Creates an empty compatibility requirement set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a compatibility requirement.
    ///
    /// Returns an error when its identifier already exists in this
    /// compatibility collection.
    ///
    /// Independent canonical semantic requirements must instead be retained
    /// through [`SemanticRequirements`].
    pub fn insert(
        &mut self,
        requirement: Requirement,
    ) -> Result<(), RequirementError> {
        let id = requirement.id.clone();

        if self.requirements.contains_key(&id) {
            return Err(RequirementError::DuplicateRequirement {
                id,
            });
        }

        self.requirements.insert(id, requirement);

        Ok(())
    }

    /// Returns a compatibility requirement by identifier.
    #[must_use]
    pub fn get(
        &self,
        id: &RequirementId,
    ) -> Option<&Requirement> {
        self.requirements.get(id)
    }

    /// Returns the number of compatibility records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Iterates in deterministic identifier order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &Requirement> {
        self.requirements.values()
    }

    /// Iterates over mandatory compatibility records.
    pub fn hard_requirements(
        &self,
    ) -> impl Iterator<Item = &Requirement> {
        self.requirements
            .values()
            .filter(|requirement| requirement.is_hard())
    }

    /// Iterates over preferred compatibility records.
    pub fn preferred_requirements(
        &self,
    ) -> impl Iterator<Item = &Requirement> {
        self.requirements
            .values()
            .filter(|requirement| {
                requirement.strength == RequirementStrength::Preferred
            })
    }

    /// Iterates over advisory compatibility records.
    pub fn advisory_requirements(
        &self,
    ) -> impl Iterator<Item = &Requirement> {
        self.requirements
            .values()
            .filter(|requirement| {
                requirement.strength == RequirementStrength::Advisory
            })
    }
}

/// A canonical semantic requirement collection for compiler consumption.
///
/// This structure deliberately retains the authoritative semantic types:
///
/// - `ResourceRequirement` from frontend semantic resources;
/// - `CapabilityRequirements` from frontend semantic capabilities.
///
/// It does not convert them into a competing compiler-owned semantic model.
///
/// Resource requirements preserve input order. Requirements originating
/// from different source nodes remain separate, even when their logical
/// contents are equal.
///
/// The canonical capability collection is retained intact. Its own
/// implementation owns source-node indexing, duplicate handling and
/// deterministic iteration.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SemanticRequirements {
    resource_requirements: Vec<ResourceRequirement>,
    capability_requirements: CapabilityRequirements,
    extraction_errors: Vec<RequirementExtractionError>,
    resource_extraction_complete: bool,
}

impl SemanticRequirements {
    /// Creates an empty semantic requirement collection.
    ///
    /// An empty collection alone does not prove that a program has no
    /// requirements. The caller must establish that the semantic analysis
    /// supplied a complete inventory.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs compiler requirements from canonical semantic outputs.
    ///
    /// Resource extraction errors are retained. A partial resource extraction
    /// is never promoted to complete.
    ///
    /// The capability collection must already have been produced by the
    /// canonical semantic capability analysis. This function does not
    /// reconstruct capability meaning or infer missing capabilities.
    pub fn from_semantic(
        resource_report: RequirementExtractionReport,
        capability_requirements: CapabilityRequirements,
    ) -> Result<Self, RequirementCollectionError> {
        let resource_extraction_complete =
            resource_report.is_complete();

        let extraction_errors =
            resource_report.errors().to_vec();

        let resource_requirements =
            resource_report.requirements().to_vec();

        let mut result = Self {
            resource_requirements: Vec::new(),
            capability_requirements,
            extraction_errors,
            resource_extraction_complete,
        };

        result.extend_resource_requirements(
            resource_requirements,
        )?;

        Ok(result)
    }

    /// Constructs compiler requirements from already validated canonical
    /// semantic resource requirements and capability requirements.
    ///
    /// This constructor is appropriate when the caller has already verified
    /// that resource extraction is complete.
    pub fn from_complete_semantic(
        resource_requirements: impl IntoIterator<Item = ResourceRequirement>,
        capability_requirements: CapabilityRequirements,
    ) -> Result<Self, RequirementCollectionError> {
        let mut result = Self {
            resource_requirements: Vec::new(),
            capability_requirements,
            extraction_errors: Vec::new(),
            resource_extraction_complete: true,
        };

        result.extend_resource_requirements(
            resource_requirements,
        )?;

        Ok(result)
    }

    /// Adds canonical semantic resource requirements transactionally.
    ///
    /// If the same source node is supplied more than once with conflicting
    /// requirement data, the operation fails and the original collection
    /// remains unchanged.
    ///
    /// Equal duplicate records for the same source node are treated as
    /// repeated delivery of the same semantic fact and are not inserted twice.
    ///
    /// Requirements without a source node are retained independently because
    /// their source identity cannot be established by this module.
    pub fn extend_resource_requirements(
        &mut self,
        requirements: impl IntoIterator<Item = ResourceRequirement>,
    ) -> Result<(), RequirementCollectionError> {
        let mut staged = self.resource_requirements.clone();

        let mut source_nodes: BTreeMap<NodeId, usize> =
            BTreeMap::new();

        for (index, existing) in staged.iter().enumerate() {
            if let Some(node) = existing.source_node() {
                source_nodes.entry(node).or_insert(index);
            }
        }

        for requirement in requirements {
            if let Some(node) = requirement.source_node() {
                if let Some(existing_index) = source_nodes.get(&node) {
                    let existing = staged
                        .get(*existing_index)
                        .ok_or(
                            RequirementCollectionError::InternalIndexInvariant,
                        )?;

                    if existing == &requirement {
                        continue;
                    }

                    return Err(
                        RequirementCollectionError::ConflictingSourceNode {
                            node,
                        },
                    );
                }

                let index = staged.len();

                staged.push(requirement);

                source_nodes.insert(node, index);
            } else {
                staged.push(requirement);
            }
        }

        self.resource_requirements = staged;

        Ok(())
    }

    /// Returns canonical semantic resource requirements in retained order.
    #[must_use]
    pub fn resource_requirements(
        &self,
    ) -> &[ResourceRequirement] {
        &self.resource_requirements
    }

    /// Returns the canonical semantic capability requirement collection.
    #[must_use]
    pub fn capability_requirements(
        &self,
    ) -> &CapabilityRequirements {
        &self.capability_requirements
    }

    /// Returns the retained resource extraction errors.
    #[must_use]
    pub fn extraction_errors(
        &self,
    ) -> &[RequirementExtractionError] {
        &self.extraction_errors
    }

    /// Returns whether resource extraction completed without errors.
    ///
    /// This does not establish target feasibility, capability compatibility,
    /// authorization, or successful compilation.
    #[must_use]
    pub const fn resource_extraction_is_complete(&self) -> bool {
        self.resource_extraction_complete
    }

    /// Returns whether the resource extraction report contained errors.
    #[must_use]
    pub fn has_resource_extraction_errors(&self) -> bool {
        !self.extraction_errors.is_empty()
    }

    /// Returns whether the resource collection is empty.
    ///
    /// This is not proof that the program has no resource requirements
    /// unless `resource_extraction_is_complete()` is also true.
    #[must_use]
    pub fn resource_requirements_are_empty(&self) -> bool {
        self.resource_requirements.is_empty()
    }

    /// Returns the number of retained resource requirements.
    #[must_use]
    pub fn resource_requirement_count(&self) -> usize {
        self.resource_requirements.len()
    }

    /// Returns the number of canonical capability requirements.
    #[must_use]
    pub fn capability_requirement_count(&self) -> usize {
        self.capability_requirements.len()
    }

    /// Returns whether resource extraction was complete and no resource
    /// extraction errors were retained.
    ///
    /// This is deliberately a narrow statement. It is not a general
    /// compilation-success or target-feasibility result.
    #[must_use]
    pub fn is_resource_analysis_complete(&self) -> bool {
        self.resource_extraction_complete
            && self.extraction_errors.is_empty()
    }

    /// Returns resource requirements with mandatory semantic intent.
    ///
    /// The canonical semantic model remains responsible for defining intent.
    pub fn required_resources(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.resource_requirements
            .iter()
            .filter(|requirement| {
                matches!(
                    requirement.intent(),
                    ResourceIntent::Required
                )
            })
    }

    /// Returns resource requirements with preferred semantic intent.
    ///
    /// A preference must not be promoted to a mandatory requirement.
    pub fn preferred_resources(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.resource_requirements
            .iter()
            .filter(|requirement| {
                matches!(
                    requirement.intent(),
                    ResourceIntent::Preferred
                )
            })
    }

    /// Returns resource requirements whose semantic intent is advisory.
    ///
    /// This includes semantic hints, not mandatory resource obligations.
    pub fn hinted_resources(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.resource_requirements
            .iter()
            .filter(|requirement| {
                matches!(
                    requirement.intent(),
                    ResourceIntent::Hint
                )
            })
    }

    /// Returns resource requirements that prohibit use of the described
    /// resource.
    pub fn forbidden_resources(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.resource_requirements
            .iter()
            .filter(|requirement| {
                matches!(
                    requirement.intent(),
                    ResourceIntent::Forbidden
                )
            })
    }

    /// Returns resource requirements describing allowed resource use.
    pub fn allowed_resources(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.resource_requirements
            .iter()
            .filter(|requirement| {
                matches!(
                    requirement.intent(),
                    ResourceIntent::Allowed
                )
            })
    }

    /// Returns resource requirements describing declared budgets.
    ///
    /// A semantic budget is not a physical allocation or a compiler
    /// operational budget. Its meaning remains governed by the semantic
    /// resource model and applicable language specifications.
    pub fn declared_resource_budgets(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.resource_requirements
            .iter()
            .filter(|requirement| {
                matches!(
                    requirement.intent(),
                    ResourceIntent::Budget
                )
            })
    }
}

/// Errors produced while validating compiler-owned requirement collections.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequirementCollectionError {
    /// The same canonical AST node was associated with conflicting
    /// semantic resource requirements.
    ConflictingSourceNode {
        /// Conflicting canonical AST node identity.
        node: NodeId,
    },

    /// An internal index invariant was violated.
    ///
    /// This indicates a compiler implementation defect, not an ordinary
    /// source-language error.
    InternalIndexInvariant,
}

impl fmt::Display for RequirementCollectionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ConflictingSourceNode { node } => {
                write!(
                    formatter,
                    "conflicting semantic resource requirements \
                     were supplied for AST node {node:?}"
                )
            }

            Self::InternalIndexInvariant => {
                formatter.write_str(
                    "requirement collection index invariant was violated",
                )
            }
        }
    }
}

impl Error for RequirementCollectionError {}

/// Errors produced by compatibility requirement construction and validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequirementError {
    /// An identifier is empty or whitespace-only.
    EmptyIdentifier,

    /// An identifier contains a control character.
    ControlCharacter {
        /// Invalid identifier.
        value: String,
    },

    /// A decimal quantity is malformed.
    InvalidDecimal {
        /// Invalid decimal representation.
        value: String,
    },

    /// A symbolic expression identifier is empty.
    EmptyExpressionId,

    /// A compatibility requirement identifier is duplicated.
    DuplicateRequirement {
        /// Duplicated identifier.
        id: RequirementId,
    },
}

impl fmt::Display for RequirementError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str(
                    "requirement identifier cannot be empty",
                )
            }

            Self::ControlCharacter { value } => {
                write!(
                    formatter,
                    "requirement identifier contains a control character: \
                     {value:?}"
                )
            }

            Self::InvalidDecimal { value } => {
                write!(
                    formatter,
                    "invalid non-negative decimal quantity: {value:?}"
                )
            }

            Self::EmptyExpressionId => {
                formatter.write_str(
                    "symbolic expression identifier cannot be empty",
                )
            }

            Self::DuplicateRequirement { id } => {
                write!(
                    formatter,
                    "compatibility requirement `{id}` is already present"
                )
            }
        }
    }
}

impl Error for RequirementError {}

/// Returns the stable architectural module name.
#[must_use]
pub const fn module_name() -> &'static str {
    REQUIREMENTS_MODULE_NAME
}

/// Returns the compiler-owned requirement model schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    REQUIREMENT_MODEL_SCHEMA_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirement_identifiers_are_open_and_namespaced() {
        let quantum = RequirementId::new(
            "quantum.mid_circuit_measurement",
        )
        .expect("valid quantum requirement");

        let future = RequirementId::new(
            "future.photonic.coherent_operation",
        )
        .expect("valid future requirement");

        assert_ne!(quantum, future);
    }

    #[test]
    fn empty_requirement_identifier_is_rejected() {
        assert_eq!(
            RequirementId::new(" "),
            Err(RequirementError::EmptyIdentifier),
        );
    }

    #[test]
    fn control_characters_are_rejected() {
        assert!(matches!(
            RequirementId::new("quantum.\nmeasurement"),
            Err(RequirementError::ControlCharacter { .. }),
        ));
    }

    #[test]
    fn decimal_quantity_is_canonicalized() {
        assert_eq!(
            DecimalQuantity::new("00012.3400")
                .expect("valid quantity")
                .as_str(),
            "12.34",
        );

        assert_eq!(
            DecimalQuantity::new("0000.000")
                .expect("valid zero")
                .as_str(),
            "0",
        );
    }

    #[test]
    fn decimal_quantity_supports_values_beyond_machine_integer_widths() {
        let value =
            "999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999";

        assert_eq!(
            DecimalQuantity::new(value)
                .expect("valid large quantity")
                .as_str(),
            value,
        );
    }

    #[test]
    fn decimal_quantity_comparison_is_numeric() {
        let smaller =
            DecimalQuantity::new("9.99").expect("valid quantity");

        let larger =
            DecimalQuantity::new("10.01").expect("valid quantity");

        assert_eq!(
            smaller.numeric_cmp(&larger),
            core::cmp::Ordering::Less,
        );
    }

    #[test]
    fn decimal_quantity_comparison_handles_fractional_widths() {
        let left =
            DecimalQuantity::new("1.2").expect("valid quantity");

        let right =
            DecimalQuantity::new("1.20").expect("valid quantity");

        assert_eq!(
            left.numeric_cmp(&right),
            core::cmp::Ordering::Equal,
        );
    }

    #[test]
    fn malformed_decimal_is_rejected() {
        assert!(
            DecimalQuantity::new("1.2.3").is_err()
        );

        assert!(
            DecimalQuantity::new("-1").is_err()
        );

        assert!(
            DecimalQuantity::new("1e10").is_err()
        );

        assert!(
            DecimalQuantity::new("").is_err()
        );
    }

    #[test]
    fn compatibility_requirement_set_rejects_duplicate_ids() {
        let id = RequirementId::new(
            "classical.integer_arithmetic",
        )
        .expect("valid ID");

        let mut set = RequirementSet::new();

        set.insert(Requirement::new(
            id.clone(),
            RequirementStrength::Hard,
            RequirementValue::Boolean(true),
        ))
        .expect("first insertion succeeds");

        assert!(matches!(
            set.insert(Requirement::new(
                id,
                RequirementStrength::Preferred,
                RequirementValue::Boolean(true),
            )),
            Err(RequirementError::DuplicateRequirement { .. }),
        ));
    }

    #[test]
    fn compatibility_requirements_are_deterministically_ordered() {
        let mut set = RequirementSet::new();

        for name in [
            "quantum.measurement",
            "classical.integer_arithmetic",
            "ai.tensor_operation",
        ] {
            set.insert(Requirement::new(
                RequirementId::new(name).expect("valid ID"),
                RequirementStrength::Hard,
                RequirementValue::Boolean(true),
            ))
            .expect("unique requirement");
        }

        let ids: Vec<&str> = set
            .iter()
            .map(|requirement| requirement.id.as_str())
            .collect();

        assert_eq!(
            ids,
            vec![
                "ai.tensor_operation",
                "classical.integer_arithmetic",
                "quantum.measurement",
            ],
        );
    }

    #[test]
    fn compatibility_strengths_remain_distinct() {
        let mut set = RequirementSet::new();

        for (name, strength) in [
            ("required", RequirementStrength::Hard),
            ("preferred", RequirementStrength::Preferred),
            ("advisory", RequirementStrength::Advisory),
        ] {
            set.insert(Requirement::new(
                RequirementId::new(name).expect("valid ID"),
                strength,
                RequirementValue::Boolean(true),
            ))
            .expect("unique requirement");
        }

        assert_eq!(set.hard_requirements().count(), 1);
        assert_eq!(set.preferred_requirements().count(), 1);
        assert_eq!(set.advisory_requirements().count(), 1);
    }

    #[test]
    fn symbolic_compatibility_values_preserve_semantic_references() {
        let value = RequirementValue::symbolic(
            "semantic.expression.123",
        )
        .expect("valid expression reference");

        assert_eq!(
            value,
            RequirementValue::Symbolic {
                expression_id: "semantic.expression.123".to_owned(),
            },
        );
    }

    #[test]
    fn empty_symbolic_reference_is_rejected() {
        assert_eq!(
            RequirementValue::symbolic(" "),
            Err(RequirementError::EmptyExpressionId),
        );
    }

    #[test]
    fn module_identity_and_schema_are_stable() {
        assert_eq!(
            module_name(),
            "zamani.compiler.analysis.requirements",
        );

        assert_eq!(
            schema_version(),
            REQUIREMENT_MODEL_SCHEMA_VERSION,
        );
    }

    #[test]
    fn semantic_requirements_start_with_incomplete_inventory() {
        let requirements = SemanticRequirements::new();

        assert!(
            !requirements.resource_extraction_is_complete(),
        );

        assert!(
            !requirements.is_resource_analysis_complete(),
        );

        assert!(
            requirements.resource_requirements_are_empty(),
        );
    }
}