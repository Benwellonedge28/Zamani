
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Instruction Fusion
//!
//! Path:
//!     src/compiler/instruction_fusion.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version: 1.97
//!
//! # Purpose
//!
//! Provide an extensible, deterministic and legality-aware instruction
//! fusion implementation for the Zamani compiler.
//!
//! This module supplies:
//!
//! - Validated fusion-rule definitions.
//! - Extensible domain and instruction identifiers.
//! - Deterministic rule registration and ordering.
//! - Longest-pattern-first candidate matching.
//! - Explicit semantic-preservation requirements.
//! - Domain-owned IR mutation through an adapter.
//! - Integration with the canonical ZUIR pass manager.
//! - Analysis dependency and invalidation declarations.
//! - Cancellation and execution-budget cooperation.
//! - Fusion statistics and structured errors.
//!
//! # Architectural authority
//!
//! This module integrates with:
//!
//! - crate::zuir::pass::manager
//! - crate::zuir::pass::contracts
//! - crate::compiler::passes
//! - crate::compiler::optimization
//! - crate::compiler::ir
//!
//! Domain-specific authorities remain:
//!
//! - Classical: crate::classical
//! - Quantum: crate::quantum::ir
//! - HDL: crate::hdl::hdlir
//! - AI: crate::ai
//! - Hybrid: crate::hybrid
//! - Distributed: crate::distributed
//!
//! # Ownership
//!
//! THIS MODULE OWNS:
//!
//! - Fusion-rule metadata.
//! - Rule registration within an engine instance.
//! - Candidate sequence matching.
//! - Deterministic rule selection.
//! - Fusion-pass orchestration.
//! - Fusion statistics.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Canonical IR definitions.
//! - Instruction semantics.
//! - Type checking.
//! - Effect analysis.
//! - Alias analysis.
//! - Quantum state semantics.
//! - HDL signal semantics.
//! - Distributed consistency semantics.
//! - Semantic-equivalence proofs.
//! - Domain-specific IR mutation algorithms.
//! - Backend instruction encoding.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime execution.
//!
//! # Semantic-preservation invariant
//!
//! A matching instruction sequence is only a fusion candidate.
//!
//! Matching is not proof of semantic equivalence.
//!
//! A domain-owned FusionAdapter must validate the complete transformation
//! against the authoritative IR and semantic rules before applying it.
//!
//! An adapter must:
//!
//! 1. Validate the exact input range.
//! 2. Validate the rule's domain.
//! 3. Validate types, operands and effects.
//! 4. Validate control-flow and dependency constraints.
//! 5. Validate applicable resource and state semantics.
//! 6. Establish the required preservation properties.
//! 7. Verify the resulting IR using its canonical verifier.
//! 8. Apply each accepted replacement atomically.
//!
//! An adapter must return Ok(false) when a candidate is not legal.
//!
//! An adapter must return Err when an actual transformation error occurs.
//!
//! The compiler must never treat an unverified candidate as a successful
//! transformation.
//!
//! # Scalability
//!
//! No universal limits are imposed on:
//!
//! - Computing domains.
//! - Registered fusion rules.
//! - Pattern lengths.
//! - Instruction counts.
//! - Functions.
//! - Modules.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Qubits.
//! - Tensor rank.
//! - Distributed participants.
//! - Resource capacity.
//!
//! Collections grow according to available resources.
//!
//! Actual operational budgets are owned by the canonical pass manager.
//!
//! Finite machine resources and representation limits remain real constraints.
//!
//! # Determinism
//!
//! Rule ordering is deterministic:
//!
//! 1. Longer patterns first.
//! 2. Higher priority first.
//! 3. Lexicographically smaller rule ID first.
//!
//! This module does not use randomness, wall-clock time, hardware discovery,
//! environment variables or telemetry to select transformations.
//!
//! # Compatibility
//!
//! The legacy symbolic convenience functions remain available.
//!
//! They do not perform unverified fusion. Without a domain-owned verifier
//! and mutation adapter, the input sequence is returned unchanged.
//!
//! Production transformations must use InstructionFusionPass.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::ops::Range;

use crate::zuir::pass::manager::{
    Pass,
    PassContext,
    PassDescriptor,
    PassError,
    PassErrorKind,
    PassId,
    PassOutcome,
    PassVersion,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public instruction-fusion API version.
pub const INSTRUCTION_FUSION_API_VERSION: u16 = 2;

/// Return the public instruction-fusion API version.
#[must_use]
pub const fn api_version() -> u16 {
    INSTRUCTION_FUSION_API_VERSION
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    qualified: bool,
) -> Result<(), FusionError> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_whitespace)
        || value.chars().any(char::is_control)
    {
        return Err(FusionError::InvalidIdentifier {
            identifier: value.to_owned(),
        });
    }

    if qualified {
        let mut segments = value.split('.');

        let valid = segments.all(|segment| {
            let mut characters = segment.chars();

            let Some(first) = characters.next() else {
                return false;
            };

            first.is_ascii_alphanumeric()
                && characters.all(|character| {
                    character.is_ascii_alphanumeric()
                        || character == '_'
                        || character == '-'
                })
        });

        if !valid || !value.contains('.') {
            return Err(FusionError::InvalidIdentifier {
                identifier: value.to_owned(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// INSTRUCTION IDENTITY
// ============================================================================

/// Validated symbolic instruction identity.
///
/// This is a matching identifier, not an IR instruction representation.
///
/// Actual instructions remain owned by their canonical IR.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction(String);

impl Instruction {
    /// Construct an instruction identifier.
    ///
    /// Identifiers may be domain-qualified, for example:
    ///
    /// - classical.add
    /// - quantum.hadamard
    /// - hdl.register_update
    /// - tensor.contract
    /// - distributed.send
    /// - vendor_extension.operation
    ///
    /// Unqualified legacy symbolic names are also accepted.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FusionError> {
        let value = value.into();

        validate_identifier(&value, false)?;

        Ok(Self(value))
    }

    /// Borrow the symbolic instruction name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Instruction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// ============================================================================
// FUSION RULE
// ============================================================================

/// Declarative instruction-fusion rule.
///
/// A rule describes a possible transformation.
///
/// It does not prove that the transformation is legal.
///
/// The domain-owned adapter must establish the actual semantics before
/// modifying canonical IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FusionRule {
    id: String,
    domain: String,
    pattern: Vec<Instruction>,
    replacement: Instruction,
    priority: i32,
    required_analyses: BTreeSet<String>,
    invalidated_analyses: BTreeSet<String>,
    preservation_properties: BTreeSet<String>,
}

impl FusionRule {
    /// Construct a fusion rule with explicit preservation requirements.
    ///
    /// A rule must identify:
    ///
    /// - A unique namespaced rule ID.
    /// - Its owning computational domain.
    /// - A non-empty input pattern.
    /// - A replacement instruction.
    /// - At least one semantic-preservation property.
    ///
    /// Example:
    ///
    /// ```
    /// use zamani::compiler::instruction_fusion::FusionRule;
    ///
    /// let rule = FusionRule::new(
    ///     "classical.add_multiply",
    ///     "classical",
    ///     vec![
    ///         "classical.multiply".to_string(),
    ///         "classical.add".to_string(),
    ///     ],
    ///     "classical.fused_multiply_add",
    ///     vec![
    ///         "classical.numeric_behavior".to_string(),
    ///         "semantic.observable_behavior".to_string(),
    ///     ],
    /// );
    ///
    /// assert!(rule.is_ok());
    /// ```
    ///
    /// The example defines metadata only. It does not authorize a real
    /// transformation without a suitable domain-owned adapter.
    pub fn new(
        id: impl Into<String>,
        domain: impl Into<String>,
        pattern: Vec<String>,
        replacement: impl Into<String>,
        preservation_properties: Vec<String>,
    ) -> Result<Self, FusionError> {
        Self::with_metadata(
            id,
            domain,
            pattern,
            replacement,
            0,
            Vec::new(),
            Vec::new(),
            preservation_properties,
        )
    }

    /// Construct a rule with complete analysis and preservation metadata.
    #[allow(clippy::too_many_arguments)]
    pub fn with_metadata(
        id: impl Into<String>,
        domain: impl Into<String>,
        pattern: Vec<String>,
        replacement: impl Into<String>,
        priority: i32,
        required_analyses: Vec<String>,
        invalidated_analyses: Vec<String>,
        preservation_properties: Vec<String>,
    ) -> Result<Self, FusionError> {
        let id = id.into();
        let domain = domain.into();
        let replacement = replacement.into();

        validate_identifier(&id, true)?;
        validate_identifier(&domain, true)?;

        if pattern.is_empty() {
            return Err(FusionError::EmptyPattern {
                rule_id: id,
            });
        }

        let mut validated_pattern = Vec::with_capacity(pattern.len());

        for name in pattern {
            validated_pattern.push(Instruction::new(name)?);
        }

        let replacement = Instruction::new(replacement)?;

        let required_analyses =
            validate_identifier_set(required_analyses)?;

        let invalidated_analyses =
            validate_identifier_set(invalidated_analyses)?;

        let preservation_properties =
            validate_identifier_set(preservation_properties)?;

        if preservation_properties.is_empty() {
            return Err(FusionError::MissingPreservationRequirements {
                rule_id: id,
            });
        }

        Ok(Self {
            id,
            domain,
            pattern: validated_pattern,
            replacement,
            priority,
            required_analyses,
            invalidated_analyses,
            preservation_properties,
        })
    }

    /// Return the stable rule ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the owning domain.
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Return the pattern.
    #[must_use]
    pub fn pattern(&self) -> &[Instruction] {
        &self.pattern
    }

    /// Return the replacement identifier.
    #[must_use]
    pub fn replacement(&self) -> &Instruction {
        &self.replacement
    }

    /// Return the rule priority.
    #[must_use]
    pub const fn priority(&self) -> i32 {
        self.priority
    }

    /// Return required analyses.
    #[must_use]
    pub fn required_analyses(&self) -> &BTreeSet<String> {
        &self.required_analyses
    }

    /// Return invalidated analyses.
    #[must_use]
    pub fn invalidated_analyses(&self) -> &BTreeSet<String> {
        &self.invalidated_analyses
    }

    /// Return the required semantic-preservation properties.
    #[must_use]
    pub fn preservation_properties(&self) -> &BTreeSet<String> {
        &self.preservation_properties
    }

    /// Return the number of matched instructions.
    #[must_use]
    pub fn pattern_length(&self) -> usize {
        self.pattern.len()
    }
}

fn validate_identifier_set(
    values: Vec<String>,
) -> Result<BTreeSet<String>, FusionError> {
    let mut result = BTreeSet::new();

    for value in values {
        validate_identifier(&value, true)?;

        if !result.insert(value.clone()) {
            return Err(FusionError::DuplicateMetadataIdentifier {
                identifier: value,
            });
        }
    }

    Ok(result)
}

// ============================================================================
// RULE ORDERING
// ============================================================================

fn compare_rules(
    left: &FusionRule,
    right: &FusionRule,
) -> std::cmp::Ordering {
    right
        .pattern_length()
        .cmp(&left.pattern_length())
        .then_with(|| right.priority.cmp(&left.priority))
        .then_with(|| left.id.cmp(&right.id))
}

// ============================================================================
// FUSION ENGINE
// ============================================================================

/// Deterministic instruction-fusion rule registry.
///
/// An engine starts empty. It never silently enables a domain-specific
/// transformation.
#[derive(Debug, Clone, Default)]
pub struct InstructionFusionEngine {
    rules: Vec<FusionRule>,
}

impl InstructionFusionEngine {
    /// Construct an empty fusion engine.
    ///
    /// Rules must be registered explicitly.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an empty fusion engine.
    ///
    /// Retained as an explicit compatibility constructor.
    #[must_use]
    pub fn empty() -> Self {
        Self::new()
    }

    /// Return the registered rules in deterministic matching order.
    #[must_use]
    pub fn rules(&self) -> &[FusionRule] {
        &self.rules
    }

    /// Return the number of registered rules.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Check whether the engine contains no rules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Register a validated fusion rule.
    ///
    /// Duplicate rule IDs are rejected.
    pub fn register_rule(
        &mut self,
        rule: FusionRule,
    ) -> Result<(), FusionError> {
        if self.contains_rule(rule.id()) {
            return Err(FusionError::DuplicateRule {
                rule_id: rule.id().to_owned(),
            });
        }

        self.rules.push(rule);
        self.rules.sort_by(compare_rules);

        Ok(())
    }

    /// Remove a registered rule.
    ///
    /// Returns true when the rule existed.
    pub fn unregister_rule(
        &mut self,
        rule_id: &str,
    ) -> bool {
        let previous_length = self.rules.len();

        self.rules.retain(|rule| rule.id() != rule_id);

        previous_length != self.rules.len()
    }

    /// Check whether a rule is registered.
    #[must_use]
    pub fn contains_rule(
        &self,
        rule_id: &str,
    ) -> bool {
        self.rules.iter().any(|rule| rule.id() == rule_id)
    }

    /// Return rule IDs in deterministic order.
    #[must_use]
    pub fn rule_ids(&self) -> Vec<&str> {
        self.rules.iter().map(FusionRule::id).collect()
    }

    /// Find the highest-ranked rule matching a symbolic instruction sequence.
    ///
    /// This method only identifies a candidate.
    ///
    /// It does not perform fusion or establish semantic equivalence.
    #[must_use]
    pub fn find_candidate(
        &self,
        instructions: &[Instruction],
        start: usize,
    ) -> Option<&FusionRule> {
        self.rules.iter().find(|rule| {
            let Some(end) = start.checked_add(rule.pattern_length()) else {
                return false;
            };

            if end > instructions.len() {
                return false;
            }

            rule.pattern
                .iter()
                .zip(&instructions[start..end])
                .all(|(expected, actual)| expected == actual)
        })
    }

    /// Return the registered rule matching a sequence at an index.
    ///
    /// This is a candidate lookup, not a semantic legality assessment.
    #[must_use]
    pub fn find_candidate_by_name(
        &self,
        instructions: &[String],
        start: usize,
    ) -> Option<&FusionRule> {
        self.rules.iter().find(|rule| {
            let Some(end) = start.checked_add(rule.pattern_length()) else {
                return false;
            };

            if end > instructions.len() {
                return false;
            }

            rule.pattern
                .iter()
                .zip(&instructions[start..end])
                .all(|(expected, actual)| expected.as_str() == actual)
        })
    }
}

// ============================================================================
// FUSION STATISTICS
// ============================================================================

/// Statistics collected during a fusion pass.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FusionStats {
    /// Number of instructions present at pass entry.
    pub input_instructions: usize,

    /// Number of instructions present after successful fusion.
    pub output_instructions: usize,

    /// Number of successfully applied fusion transformations.
    pub fusion_count: usize,

    /// Number of instructions eliminated.
    pub instructions_eliminated: usize,

    /// Number of candidate positions examined.
    pub candidates_examined: usize,

    /// Number of registered rules.
    pub rules_available: usize,

    /// Successful applications grouped by rule ID.
    pub rule_applications: BTreeMap<String, usize>,
}

impl FusionStats {
    /// Calculate the percentage reduction in instruction count.
    ///
    /// Returns zero for an empty input.
    #[must_use]
    pub fn reduction_percent(&self) -> f64 {
        if self.input_instructions == 0 {
            return 0.0;
        }

        (self.instructions_eliminated as f64
            / self.input_instructions as f64)
            * 100.0
    }
}

// ============================================================================
// FUSION ERRORS
// ============================================================================

/// Structured instruction-fusion error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FusionError {
    /// Invalid identifier.
    InvalidIdentifier {
        /// Invalid identifier.
        identifier: String,
    },

    /// Empty pattern.
    EmptyPattern {
        /// Rule identifier.
        rule_id: String,
    },

    /// Duplicate registered rule.
    DuplicateRule {
        /// Duplicate rule identifier.
        rule_id: String,
    },

    /// Duplicate metadata identifier.
    DuplicateMetadataIdentifier {
        /// Duplicate identifier.
        identifier: String,
    },

    /// A preservation contract was not provided.
    MissingPreservationRequirements {
        /// Rule identifier.
        rule_id: String,
    },

    /// The domain adapter rejected or failed a transformation.
    AdapterFailure {
        /// Rule identifier.
        rule_id: String,

        /// Failure explanation.
        reason: String,
    },

    /// Arithmetic overflow while accounting for a transformation.
    AccountingOverflow,
}

impl fmt::Display for FusionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { identifier } => {
                write!(formatter, "invalid fusion identifier: {identifier:?}")
            }

            Self::EmptyPattern { rule_id } => {
                write!(formatter, "fusion rule {rule_id:?} has an empty pattern")
            }

            Self::DuplicateRule { rule_id } => {
                write!(formatter, "fusion rule {rule_id:?} is already registered")
            }

            Self::DuplicateMetadataIdentifier { identifier } => {
                write!(formatter, "duplicate fusion metadata identifier: {identifier:?}")
            }

            Self::MissingPreservationRequirements { rule_id } => {
                write!(
                    formatter,
                    "fusion rule {rule_id:?} has no preservation requirements"
                )
            }

            Self::AdapterFailure { rule_id, reason } => {
                write!(
                    formatter,
                    "fusion adapter failed for rule {rule_id:?}: {reason}"
                )
            }

            Self::AccountingOverflow => {
                formatter.write_str("fusion statistics overflowed")
            }
        }
    }
}

impl Error for FusionError {}

// ============================================================================
// DOMAIN-OWNED IR ADAPTER
// ============================================================================

/// Domain-owned interface for matching and applying verified fusion.
///
/// Implementations adapt their authoritative IR without introducing another
/// compiler IR representation.
///
/// # Required adapter guarantees
///
/// `instruction_name` must expose a stable symbolic identity for the
/// instruction at the requested position.
///
/// `apply_verified_fusion` must:
///
/// - Check that the supplied range is still valid.
/// - Check the exact rule and domain.
/// - Validate the domain's semantic preconditions.
/// - Establish the required preservation properties.
/// - Validate effects, dependencies, state and ordering.
/// - Apply the replacement atomically for that individual candidate.
/// - Return `Ok(false)` without modifying IR if the candidate is not legal.
/// - Return `Err` for an actual adapter or transformation failure.
///
/// The adapter must use the authoritative domain verifier and IR APIs.
///
/// The trait does not itself prove that an implementation is correct.
pub trait FusionAdapter<I>: Send {
    /// Borrow the symbolic name of an instruction.
    fn instruction_name<'a>(
        &self,
        ir: &'a I,
        index: usize,
    ) -> Option<&'a str>;

    /// Verify and atomically apply one candidate fusion.
    ///
    /// `Ok(true)` means the transformation was applied.
    ///
    /// `Ok(false)` means the candidate was rejected without mutation.
    ///
    /// `Err` means the adapter encountered a transformation failure.
    fn apply_verified_fusion(
        &mut self,
        ir: &mut I,
        range: Range<usize>,
        rule: &FusionRule,
    ) -> Result<bool, String>;
}

// ============================================================================
// PRODUCTION PASS
// ============================================================================

/// Production fusion pass integrated with the canonical ZUIR pass manager.
///
/// The pass owns its rule engine and domain adapter.
///
/// The canonical pass manager owns registration, ordering, execution budgets,
/// cancellation, verification orchestration and analysis invalidation.
pub struct InstructionFusionPass<I, A> {
    engine: InstructionFusionEngine,
    adapter: A,
    marker: std::marker::PhantomData<fn(&mut I)>,
}

impl<I, A> InstructionFusionPass<I, A>
where
    A: FusionAdapter<I>,
{
    /// Construct a fusion pass using an explicitly configured engine.
    #[must_use]
    pub fn new(
        engine: InstructionFusionEngine,
        adapter: A,
    ) -> Self {
        Self {
            engine,
            adapter,
            marker: std::marker::PhantomData,
        }
    }

    /// Borrow the configured fusion engine.
    #[must_use]
    pub fn engine(&self) -> &InstructionFusionEngine {
        &self.engine
    }

    /// Consume the pass and return its adapter.
    #[must_use]
    pub fn into_adapter(self) -> A {
        self.adapter
    }

    /// Build the canonical pass descriptor.
    ///
    /// Required analyses and invalidated analyses are the union of the
    /// registered rules' declarations.
    ///
    /// This conservative descriptor prevents the manager from executing
    /// transformations without the analyses declared by the rule set.
    fn build_descriptor(&self) -> PassDescriptor {
        let mut required = BTreeSet::new();
        let mut invalidated = BTreeSet::new();

        for rule in self.engine.rules() {
            required.extend(rule.required_analyses().iter().cloned());
            invalidated.extend(rule.invalidated_analyses().iter().cloned());
        }

        let mut descriptor = PassDescriptor::new(
            PassId::new("compiler.instruction_fusion")
                .expect("static pass identifier is valid"),
            PassVersion::new(2, 0, 0),
            "Legality-aware, domain-adapted instruction fusion",
        );

        descriptor.required_analyses = required;
        descriptor.invalidated_analyses = invalidated;
        descriptor.preserves_all_analyses = false;
        descriptor.requires_verified_input = true;
        descriptor.verifies_output = true;
        descriptor.deterministic = true;

        descriptor
    }

    /// Determine whether a rule's input pattern matches at an index.
    fn matches_at(
        &self,
        ir: &I,
        start: usize,
        rule: &FusionRule,
    ) -> bool {
        let Some(end) = start.checked_add(rule.pattern_length()) else {
            return false;
        };

        rule.pattern()
            .iter()
            .enumerate()
            .all(|(offset, expected)| {
                self.adapter
                    .instruction_name(ir, start + offset)
                    == Some(expected.as_str())
            })
            && self.adapter.instruction_name(
                ir,
                end.saturating_sub(1),
            ).is_some()
    }
}

impl<I, A> Pass<I> for InstructionFusionPass<I, A>
where
    I: Send,
    A: FusionAdapter<I>,
{
    fn descriptor(&self) -> PassDescriptor {
        self.build_descriptor()
    }

    fn run(
        &mut self,
        ir: &mut I,
        context: &mut PassContext,
    ) -> Result<PassOutcome, PassError> {
        context.check_budget()?;

        let input_count = {
            let mut index = 0usize;

            while self.adapter.instruction_name(ir, index).is_some() {
                index = index.checked_add(1).ok_or_else(|| {
                    PassError::new(
                        PassErrorKind::PassFailed,
                        "instruction count overflow",
                    )
                })?;
            }

            index
        };

        let mut stats = FusionStats {
            input_instructions: input_count,
            output_instructions: input_count,
            rules_available: self.engine.rule_count(),
            ..FusionStats::default()
        };

        let mut index = 0usize;

        while index < input_count {
            context.check_budget()?;

            stats.candidates_examined =
                stats.candidates_examined.checked_add(1).ok_or_else(|| {
                    PassError::new(
                        PassErrorKind::PassFailed,
                        "candidate counter overflow",
                    )
                })?;

            let mut applied = false;

            for rule in self.engine.rules() {
                if !self.matches_at(ir, index, rule) {
                    continue;
                }

                let end = index
                    .checked_add(rule.pattern_length())
                    .ok_or_else(|| {
                        PassError::new(
                            PassErrorKind::PassFailed,
                            "fusion range overflow",
                        )
                    })?;

                match self.adapter.apply_verified_fusion(
                    ir,
                    index..end,
                    rule,
                ) {
                    Ok(true) => {
                        stats.fusion_count =
                            stats.fusion_count.checked_add(1).ok_or_else(|| {
                                PassError::new(
                                    PassErrorKind::PassFailed,
                                    "fusion counter overflow",
                                )
                            })?;

                        let eliminated =
                            rule.pattern_length().saturating_sub(1);

                        stats.instructions_eliminated =
                            stats.instructions_eliminated
                                .checked_add(eliminated)
                                .ok_or_else(|| {
                                    PassError::new(
                                        PassErrorKind::PassFailed,
                                        "eliminated instruction counter overflow",
                                    )
                                })?;

                        let count = stats
                            .rule_applications
                            .entry(rule.id().to_owned())
                            .or_insert(0);

                        *count = count.checked_add(1).ok_or_else(|| {
                            PassError::new(
                                PassErrorKind::PassFailed,
                                "rule application counter overflow",
                            )
                        })?;

                        index = end;
                        applied = true;
                        break;
                    }

                    Ok(false) => {
                        // The domain verifier rejected this candidate.
                        // Try another matching rule without changing IR.
                    }

                    Err(reason) => {
                        return Err(PassError::new(
                            PassErrorKind::PassFailed,
                            FusionError::AdapterFailure {
                                rule_id: rule.id().to_owned(),
                                reason,
                            }
                            .to_string(),
                        ));
                    }
                }
            }

            if !applied {
                index = index.checked_add(1).ok_or_else(|| {
                    PassError::new(
                        PassErrorKind::PassFailed,
                        "instruction cursor overflow",
                    )
                })?;
            }
        }

        stats.output_instructions =
            stats.input_instructions.saturating_sub(
                stats.instructions_eliminated,
            );

        if stats.fusion_count == 0 {
            Ok(PassOutcome::unchanged())
        } else {
            Ok(PassOutcome::changed())
        }
    }
}

// ============================================================================
// LEGACY SYMBOLIC API
// ============================================================================

/// Legacy result of a symbolic-only fusion request.
///
/// The symbolic compatibility API cannot prove semantic preservation.
/// It therefore does not perform transformations.
#[deprecated(
    since = "2.0.0",
    note = "Use InstructionFusionPass with a domain-owned FusionAdapter"
)]
pub fn fuse_instructions(
    raw_instructions: &[String],
) -> Vec<String> {
    raw_instructions.to_vec()
}

impl InstructionFusionEngine {
    /// Legacy symbolic fusion method.
    ///
    /// Without a domain-owned adapter and semantic verifier, this method
    /// returns the original instruction sequence unchanged.
    ///
    /// Use `InstructionFusionPass` for actual transformations.
    #[deprecated(
        since = "2.0.0",
        note = "Use InstructionFusionPass with a domain-owned FusionAdapter"
    )]
    #[must_use]
    pub fn fuse(
        &self,
        raw_instructions: &[String],
    ) -> Vec<String> {
        raw_instructions.to_vec()
    }

    /// Legacy symbolic convenience method.
    ///
    /// No unverified transformations are performed.
    #[deprecated(
        since = "2.0.0",
        note = "Use InstructionFusionPass with a domain-owned FusionAdapter"
    )]
    #[must_use]
    pub fn fuse_instructions(
        raw_instructions: &[String],
    ) -> Vec<String> {
        raw_instructions.to_vec()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use crate::zuir::pass::manager::{
        IrVerifier,
        PassManager,
        PassManagerConfig,
        PassError,
    };

    fn rule(
        id: &str,
        pattern: &[&str],
        replacement: &str,
    ) -> FusionRule {
        FusionRule::new(
            id,
            "classical",
            pattern.iter().map(|item| (*item).to_owned()).collect(),
            replacement,
            vec![
                "semantic.observable_behavior".to_owned(),
            ],
        )
        .expect("test rule must be valid")
    }

    struct StringAdapter;

    impl FusionAdapter<Vec<String>> for StringAdapter {
        fn instruction_name<'a>(
            &self,
            ir: &'a Vec<String>,
            index: usize,
        ) -> Option<&'a str> {
            ir.get(index).map(String::as_str)
        }

        fn apply_verified_fusion(
            &mut self,
            ir: &mut Vec<String>,
            range: Range<usize>,
            rule: &FusionRule,
        ) -> Result<bool, String> {
            if range.end > ir.len()
                || range.start >= range.end
                || range.len() != rule.pattern_length()
            {
                return Err("invalid fusion range".to_owned());
            }

            let actual_matches = ir[range.clone()]
                .iter()
                .map(String::as_str)
                .eq(rule.pattern().iter().map(Instruction::as_str));

            if !actual_matches {
                return Ok(false);
            }

            // This is a test adapter. A production adapter must invoke
            // its authoritative domain verifier before committing.
            let replacement = rule.replacement().as_str().to_owned();

            ir.splice(range, std::iter::once(replacement));

            Ok(true)
        }
    }

    struct IdentityVerifier;

    impl IrVerifier<Vec<String>> for IdentityVerifier {
        fn verify(
            &self,
            _ir: &Vec<String>,
        ) -> Result<(), PassError> {
            Ok(())
        }
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(api_version(), 2);
    }

    #[test]
    fn engine_starts_without_hard_coded_rules() {
        let engine = InstructionFusionEngine::new();

        assert!(engine.is_empty());
        assert_eq!(engine.rule_count(), 0);
    }

    #[test]
    fn valid_rule_can_be_registered() {
        let mut engine = InstructionFusionEngine::new();

        engine.register_rule(rule(
            "classical.multiply_add",
            &["classical.multiply", "classical.add"],
            "classical.fused_multiply_add",
        ))
        .expect("registration should succeed");

        assert_eq!(engine.rule_count(), 1);
        assert!(engine.contains_rule("classical.multiply_add"));
    }

    #[test]
    fn duplicate_rule_ids_are_rejected() {
        let mut engine = InstructionFusionEngine::new();

        engine
            .register_rule(rule("classical.test", &["A", "B"], "AB"))
            .expect("first registration should succeed");

        assert!(matches!(
            engine.register_rule(rule(
                "classical.test",
                &["C", "D"],
                "CD",
            )),
            Err(FusionError::DuplicateRule { .. })
        ));
    }

    #[test]
    fn empty_patterns_are_rejected() {
        let result = FusionRule::new(
            "classical.empty",
            "classical",
            Vec::new(),
            "classical.output",
            vec!["semantic.observable_behavior".to_owned()],
        );

        assert!(matches!(
            result,
            Err(FusionError::EmptyPattern { .. })
        ));
    }

    #[test]
    fn preservation_requirements_are_mandatory() {
        let result = FusionRule::new(
            "classical.no_preservation",
            "classical",
            vec!["classical.add".to_owned()],
            "classical.output",
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(FusionError::MissingPreservationRequirements { .. })
        ));
    }

    #[test]
    fn malformed_domain_identifiers_are_rejected() {
        let result = FusionRule::new(
            "classical.test",
            "not a domain",
            vec!["classical.add".to_owned()],
            "classical.output",
            vec!["semantic.observable_behavior".to_owned()],
        );

        assert!(matches!(
            result,
            Err(FusionError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn longest_pattern_wins() {
        let mut engine = InstructionFusionEngine::new();

        engine
            .register_rule(rule("classical.pair", &["A", "B"], "AB"))
            .unwrap();

        engine
            .register_rule(rule("classical.triple", &["A", "B", "C"], "ABC"))
            .unwrap();

        let input = vec![
            Instruction::new("A").unwrap(),
            Instruction::new("B").unwrap(),
            Instruction::new("C").unwrap(),
        ];

        assert_eq!(
            engine.find_candidate(&input, 0).map(FusionRule::id),
            Some("classical.triple")
        );
    }

    #[test]
    fn priority_breaks_equal_length_ties() {
        let mut engine = InstructionFusionEngine::new();

        let low = FusionRule::with_metadata(
            "classical.low",
            "classical",
            vec!["A".to_owned(), "B".to_owned()],
            "LOW".to_owned(),
            1,
            Vec::new(),
            Vec::new(),
            vec!["semantic.observable_behavior".to_owned()],
        )
        .unwrap();

        let high = FusionRule::with_metadata(
            "classical.high",
            "classical",
            vec!["A".to_owned(), "B".to_owned()],
            "HIGH".to_owned(),
            10,
            Vec::new(),
            Vec::new(),
            vec!["semantic.observable_behavior".to_owned()],
        )
        .unwrap();

        engine.register_rule(low).unwrap();
        engine.register_rule(high).unwrap();

        assert_eq!(
            engine.rule_ids(),
            vec!["classical.high", "classical.low"]
        );
    }

    #[test]
    fn candidate_matching_does_not_apply_a_transformation() {
        let mut engine = InstructionFusionEngine::new();

        engine
            .register_rule(rule("classical.pair", &["A", "B"], "AB"))
            .unwrap();

        let input = vec![
            Instruction::new("A").unwrap(),
            Instruction::new("B").unwrap(),
        ];

        assert!(engine.find_candidate(&input, 0).is_some());

        // Candidate discovery does not mutate or rewrite input.
        assert_eq!(input[0].as_str(), "A");
        assert_eq!(input[1].as_str(), "B");
    }

    #[test]
    fn canonical_pass_manager_executes_verified_adapter() {
        let mut engine = InstructionFusionEngine::new();

        engine
            .register_rule(rule(
                "classical.multiply_add",
                &["MUL", "ADD"],
                "FMA",
            ))
            .unwrap();

        let pass = InstructionFusionPass::new(engine, StringAdapter);

        let mut manager = PassManager::<Vec<String>>::new(
            PassManagerConfig::default(),
        )
        .with_verifier(Arc::new(IdentityVerifier));

        manager.add_pass(pass).expect("pass registration succeeds");

        let mut ir = vec![
            "MUL".to_owned(),
            "ADD".to_owned(),
            "STORE".to_owned(),
        ];

        let report = manager.run(&mut ir).expect("pass should succeed");

        assert_eq!(
            ir,
            vec!["FMA".to_owned(), "STORE".to_owned()]
        );

        assert!(report.changed);
        assert_eq!(report.executed_passes(), 1);
    }

    #[test]
    fn rejected_candidate_is_preserved() {
        struct RejectingAdapter;

        impl FusionAdapter<Vec<String>> for RejectingAdapter {
            fn instruction_name<'a>(
                &self,
                ir: &'a Vec<String>,
                index: usize,
            ) -> Option<&'a str> {
                ir.get(index).map(String::as_str)
            }

            fn apply_verified_fusion(
                &mut self,
                _ir: &mut Vec<String>,
                _range: Range<usize>,
                _rule: &FusionRule,
            ) -> Result<bool, String> {
                Ok(false)
            }
        }

        let mut engine = InstructionFusionEngine::new();

        engine
            .register_rule(rule("classical.pair", &["A", "B"], "AB"))
            .unwrap();

        let pass = InstructionFusionPass::new(engine, RejectingAdapter);

        let mut manager = PassManager::<Vec<String>>::new(
            PassManagerConfig::default(),
        )
        .with_verifier(Arc::new(IdentityVerifier));

        manager.add_pass(pass).unwrap();

        let mut ir = vec!["A".to_owned(), "B".to_owned()];

        let report = manager.run(&mut ir).unwrap();

        assert_eq!(ir, vec!["A".to_owned(), "B".to_owned()]);
        assert!(!report.changed);
    }

    #[test]
    fn rule_can_be_unregistered() {
        let mut engine = InstructionFusionEngine::new();

        engine
            .register_rule(rule("classical.test", &["A"], "B"))
            .unwrap();

        assert!(engine.unregister_rule("classical.test"));
        assert!(!engine.contains_rule("classical.test"));
    }

    #[test]
    fn legacy_symbolic_api_does_not_perform_unverified_fusion() {
        let input = vec![
            "SPIKE_EMIT".to_owned(),
            "MEMBRANE_INTEGRATE".to_owned(),
        ];

        #[allow(deprecated)]
        let output = InstructionFusionEngine::fuse_instructions(&input);

        assert_eq!(output, input);
    }
}
