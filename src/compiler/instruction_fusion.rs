//! Zamani Compiler — Instruction Fusion Engine
//!
//! Production-grade deterministic instruction-fusion pass.
//!
//! The fusion engine transforms recognized instruction sequences into
//! semantically equivalent macro-instructions. It deliberately operates on
//! symbolic instruction names rather than target-specific machine encodings.
//!
//! Design goals:
//! - deterministic output
//! - explicit fusion rules
//! - longest-match-first selection
//! - no mutation of caller-owned input
//! - overlap-safe matching
//! - invalid/empty rules are rejected
//! - useful optimization statistics
//! - backwards-compatible convenience API
//! - no dependency on telemetry or runtime state
//!
//! SRO telemetry may be used by a higher-level optimization planner to choose
//! which rules to enable, but this module itself remains deterministic.
//!
//! # Safety
//!
//! Fusion is only legal when a rule has been explicitly registered. The engine
//! never guesses that two arbitrary instructions are equivalent to a macro.
//!
//! A backend consuming the resulting macro-instructions is responsible for
//! defining their exact lowering/semantics.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Canonical instruction representation used by the fusion engine.
///
/// A `String` remains the public compatibility representation, while this
/// wrapper provides validation and a stable semantic type for new callers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instruction(String);

impl Instruction {
    /// Creates a validated instruction.
    pub fn new(value: impl Into<String>) -> Result<Self, FusionError> {
        let value = value.into();

        validate_instruction_name(&value)?;

        Ok(Self(value))
    }

    /// Returns the instruction name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the instruction and returns its owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Instruction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// A single instruction-fusion rule.
///
/// Example:
///
/// `SPIKE_EMIT + MEMBRANE_INTEGRATE -> MACRO_SPIKE_INTEGRATE`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FusionRule {
    /// Unique rule identifier.
    pub id: String,

    /// Input instruction sequence.
    pub pattern: Vec<String>,

    /// Replacement macro-instruction.
    pub replacement: String,

    /// Optional priority used when multiple rules match at the same position.
    ///
    /// Higher priority wins.
    pub priority: i32,
}

impl FusionRule {
    /// Creates and validates a fusion rule.
    pub fn new(
        id: impl Into<String>,
        pattern: Vec<String>,
        replacement: impl Into<String>,
    ) -> Result<Self, FusionError> {
        Self::with_priority(id, pattern, replacement, 0)
    }

    /// Creates a rule with an explicit priority.
    pub fn with_priority(
        id: impl Into<String>,
        pattern: Vec<String>,
        replacement: impl Into<String>,
        priority: i32,
    ) -> Result<Self, FusionError> {
        let id = id.into();
        let replacement = replacement.into();

        if id.trim().is_empty() {
            return Err(FusionError::InvalidRule {
                rule_id: id,
                reason: "rule identifier cannot be empty".to_string(),
            });
        }

        if pattern.is_empty() {
            return Err(FusionError::InvalidRule {
                rule_id: id,
                reason: "rule pattern cannot be empty".to_string(),
            });
        }

        for instruction in &pattern {
            validate_instruction_name(instruction).map_err(|_| {
                FusionError::InvalidRule {
                    rule_id: id.clone(),
                    reason: format!(
                        "invalid instruction in pattern: '{}'",
                        instruction
                    ),
                }
            })?;
        }

        validate_instruction_name(&replacement).map_err(|_| {
            FusionError::InvalidRule {
                rule_id: id.clone(),
                reason: format!(
                    "invalid replacement instruction: '{}'",
                    replacement
                ),
            }
        })?;

        Ok(Self {
            id,
            pattern,
            replacement,
            priority,
        })
    }

    /// Number of instructions consumed by this rule.
    pub fn pattern_length(&self) -> usize {
        self.pattern.len()
    }
}

/// Statistics produced by a fusion pass.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FusionStats {
    /// Number of instructions supplied to the pass.
    pub input_instructions: usize,

    /// Number of instructions produced by the pass.
    pub output_instructions: usize,

    /// Number of successful fusions.
    pub fusion_count: usize,

    /// Number of instructions removed through fusion.
    pub instructions_eliminated: usize,

    /// Number of candidate positions examined.
    pub candidates_examined: usize,

    /// Number of rules registered when the pass started.
    pub rules_available: usize,
}

impl FusionStats {
    /// Returns the reduction percentage.
    pub fn reduction_percent(&self) -> f64 {
        if self.input_instructions == 0 {
            return 0.0;
        }

        self.instructions_eliminated as f64
            / self.input_instructions as f64
            * 100.0
    }
}

/// Result of a fusion pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FusionResult {
    /// Optimized instruction sequence.
    pub instructions: Vec<String>,

    /// Pass statistics.
    pub stats: FusionStats,

    /// Number of times each rule was applied.
    pub rule_applications: HashMap<String, usize>,
}

/// Errors produced by the fusion engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FusionError {
    InvalidInstruction(String),
    InvalidRule {
        rule_id: String,
        reason: String,
    },
    DuplicateRule(String),
}

impl fmt::Display for FusionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInstruction(instruction) => {
                write!(
                    formatter,
                    "invalid instruction '{}'",
                    instruction
                )
            }

            Self::InvalidRule { rule_id, reason } => {
                write!(
                    formatter,
                    "invalid fusion rule '{}': {}",
                    rule_id, reason
                )
            }

            Self::DuplicateRule(rule_id) => {
                write!(
                    formatter,
                    "fusion rule '{}' is already registered",
                    rule_id
                )
            }
        }
    }
}

impl std::error::Error for FusionError {}

/// Production-grade instruction fusion engine.
#[derive(Debug, Clone)]
pub struct InstructionFusionEngine {
    rules: Vec<FusionRule>,
}

impl Default for InstructionFusionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl InstructionFusionEngine {
    /// Creates an engine with Zamani's standard fusion rules.
    pub fn new() -> Self {
        let mut engine = Self { rules: Vec::new() };

        // This rule preserves compatibility with the original implementation.
        //
        // Construction is guaranteed to succeed because these are compile-time
        // constants under our control.
        engine
            .register_rule(
                FusionRule::with_priority(
                    "spike_emit_membrane_integrate",
                    vec![
                        "SPIKE_EMIT".to_string(),
                        "MEMBRANE_INTEGRATE".to_string(),
                    ],
                    "MACRO_SPIKE_INTEGRATE",
                    100,
                )
                .expect("built-in fusion rule must be valid"),
            )
            .expect("built-in fusion rule must be unique");

        engine
    }

    /// Creates an engine without built-in rules.
    ///
    /// Useful for a backend or test that wants complete control over the
    /// enabled fusion set.
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    /// Returns all registered rules.
    pub fn rules(&self) -> &[FusionRule] {
        &self.rules
    }

    /// Registers a fusion rule.
    pub fn register_rule(&mut self, rule: FusionRule) -> Result<(), FusionError> {
        if self.rules.iter().any(|existing| existing.id == rule.id) {
            return Err(FusionError::DuplicateRule(rule.id));
        }

        self.rules.push(rule);

        // Keep matching deterministic.
        //
        // 1. Longer patterns first.
        // 2. Higher priority first.
        // 3. Rule ID lexicographically as final tie-breaker.
        self.rules.sort_by(compare_rules);

        Ok(())
    }

    /// Removes a fusion rule by identifier.
    ///
    /// Returns `true` when a rule was removed.
    pub fn unregister_rule(&mut self, rule_id: &str) -> bool {
        let original_len = self.rules.len();

        self.rules.retain(|rule| rule.id != rule_id);

        original_len != self.rules.len()
    }

    /// Returns whether a rule is registered.
    pub fn contains_rule(&self, rule_id: &str) -> bool {
        self.rules.iter().any(|rule| rule.id == rule_id)
    }

    /// Performs instruction fusion and returns complete statistics.
    pub fn fuse_with_stats(
        &self,
        raw_instructions: &[String],
    ) -> FusionResult {
        let mut optimized = Vec::with_capacity(raw_instructions.len());
        let mut rule_applications = HashMap::new();

        let mut stats = FusionStats {
            input_instructions: raw_instructions.len(),
            rules_available: self.rules.len(),
            ..FusionStats::default()
        };

        let mut index = 0;

        while index < raw_instructions.len() {
            stats.candidates_examined += 1;

            if let Some(rule) =
                self.find_matching_rule(raw_instructions, index)
            {
                optimized.push(rule.replacement.clone());

                *rule_applications
                    .entry(rule.id.clone())
                    .or_insert(0) += 1;

                stats.fusion_count += 1;

                stats.instructions_eliminated +=
                    rule.pattern_length().saturating_sub(1);

                index += rule.pattern_length();
            } else {
                optimized.push(raw_instructions[index].clone());
                index += 1;
            }
        }

        stats.output_instructions = optimized.len();

        FusionResult {
            instructions: optimized,
            stats,
            rule_applications,
        }
    }

    /// Performs deterministic instruction fusion.
    ///
    /// This is the primary production API.
    pub fn fuse(
        &self,
        raw_instructions: &[String],
    ) -> Vec<String> {
        self.fuse_with_stats(raw_instructions).instructions
    }

    /// Backwards-compatible API retained from the original implementation.
    pub fn fuse_instructions(
        raw_instructions: &[String],
    ) -> Vec<String> {
        Self::new().fuse(raw_instructions)
    }

    /// Finds the highest-ranked matching rule at a specific instruction
    /// position.
    fn find_matching_rule(
        &self,
        instructions: &[String],
        start: usize,
    ) -> Option<&FusionRule> {
        self.rules.iter().find(|rule| {
            let end = start.saturating_add(rule.pattern_length());

            if end > instructions.len() {
                return false;
            }

            rule.pattern
                .iter()
                .enumerate()
                .all(|(offset, expected)| {
                    instructions[start + offset] == *expected
                })
        })
    }

    /// Returns all registered rule IDs in deterministic order.
    pub fn rule_ids(&self) -> Vec<&str> {
        self.rules.iter().map(|rule| rule.id.as_str()).collect()
    }
}

/// Validates a symbolic instruction name.
///
/// Instruction names intentionally use a conservative format:
/// ASCII alphanumeric characters, `_`, `.`, `-`, and `:`.
fn validate_instruction_name(
    instruction: &str,
) -> Result<(), FusionError> {
    if instruction.trim().is_empty() {
        return Err(FusionError::InvalidInstruction(
            instruction.to_string(),
        ));
    }

    if instruction != instruction.trim() {
        return Err(FusionError::InvalidInstruction(
            instruction.to_string(),
        ));
    }

    if !instruction
        .bytes()
        .all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'_' | b'.' | b'-' | b':')
        })
    {
        return Err(FusionError::InvalidInstruction(
            instruction.to_string(),
        ));
    }

    Ok(())
}

/// Deterministic rule ordering.
///
/// Longer patterns must be checked first so that a two- or three-instruction
/// fusion wins over a shorter rule sharing the same prefix.
fn compare_rules(left: &FusionRule, right: &FusionRule) -> Ordering {
    right
        .pattern_length()
        .cmp(&left.pattern_length())
        .then_with(|| right.priority.cmp(&left.priority))
        .then_with(|| left.id.cmp(&right.id))
}

/// Convenience function for callers that only need the standard fusion pass.
pub fn fuse_instructions(
    raw_instructions: &[String],
) -> Vec<String> {
    InstructionFusionEngine::new().fuse(raw_instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn standard_spike_fusion_is_preserved() {
        let input = strings(&[
            "SPIKE_EMIT",
            "MEMBRANE_INTEGRATE",
        ]);

        let output = InstructionFusionEngine::fuse_instructions(&input);

        assert_eq!(
            output,
            strings(&["MACRO_SPIKE_INTEGRATE"])
        );
    }

    #[test]
    fn unrelated_instructions_are_preserved() {
        let input = strings(&[
            "LOAD",
            "ADD",
            "STORE",
        ]);

        let output = InstructionFusionEngine::fuse_instructions(&input);

        assert_eq!(output, input);
    }

    #[test]
    fn fusion_does_not_modify_input() {
        let input = strings(&[
            "SPIKE_EMIT",
            "MEMBRANE_INTEGRATE",
        ]);

        let original = input.clone();

        let _ = InstructionFusionEngine::fuse_instructions(&input);

        assert_eq!(input, original);
    }

    #[test]
    fn multiple_fusions_are_supported() {
        let input = strings(&[
            "SPIKE_EMIT",
            "MEMBRANE_INTEGRATE",
            "SPIKE_EMIT",
            "MEMBRANE_INTEGRATE",
        ]);

        let output = InstructionFusionEngine::fuse_instructions(&input);

        assert_eq!(
            output,
            strings(&[
                "MACRO_SPIKE_INTEGRATE",
                "MACRO_SPIKE_INTEGRATE",
            ])
        );
    }

    #[test]
    fn overlapping_matches_are_consumed_safely() {
        let mut engine = InstructionFusionEngine::empty();

        engine
            .register_rule(
                FusionRule::new(
                    "triple",
                    strings(&["A", "B", "C"]),
                    "ABC",
                )
                .unwrap(),
            )
            .unwrap();

        engine
            .register_rule(
                FusionRule::new(
                    "pair",
                    strings(&["A", "B"]),
                    "AB",
                )
                .unwrap(),
            )
            .unwrap();

        let result = engine.fuse_with_stats(&strings(&[
            "A", "B", "C",
        ]));

        // Longest match wins.
        assert_eq!(result.instructions, strings(&["ABC"]));
        assert_eq!(result.stats.fusion_count, 1);
    }

    #[test]
    fn higher_priority_wins_for_equal_length_rules() {
        let mut engine = InstructionFusionEngine::empty();

        engine
            .register_rule(
                FusionRule::with_priority(
                    "low",
                    strings(&["A", "B"]),
                    "LOW",
                    1,
                )
                .unwrap(),
            )
            .unwrap();

        engine
            .register_rule(
                FusionRule::with_priority(
                    "high",
                    strings(&["A", "B"]),
                    "HIGH",
                    100,
                )
                .unwrap(),
            )
            .unwrap();

        let output = engine.fuse(&strings(&["A", "B"]));

        assert_eq!(output, strings(&["HIGH"]));
    }

    #[test]
    fn duplicate_rule_ids_are_rejected() {
        let mut engine = InstructionFusionEngine::empty();

        let first = FusionRule::new(
            "duplicate",
            strings(&["A", "B"]),
            "AB",
        )
        .unwrap();

        let second = FusionRule::new(
            "duplicate",
            strings(&["C", "D"]),
            "CD",
        )
        .unwrap();

        assert!(engine.register_rule(first).is_ok());

        assert_eq!(
            engine.register_rule(second),
            Err(FusionError::DuplicateRule(
                "duplicate".to_string()
            ))
        );
    }

    #[test]
    fn empty_pattern_is_rejected() {
        let result = FusionRule::new(
            "invalid",
            Vec::new(),
            "RESULT",
        );

        assert!(matches!(
            result,
            Err(FusionError::InvalidRule { .. })
        ));
    }

    #[test]
    fn invalid_instruction_name_is_rejected() {
        let result = FusionRule::new(
            "invalid",
            strings(&["A B"]),
            "RESULT",
        );

        assert!(matches!(
            result,
            Err(FusionError::InvalidRule { .. })
        ));
    }

    #[test]
    fn empty_input_is_safe() {
        let engine = InstructionFusionEngine::new();

        let result = engine.fuse_with_stats(&[]);

        assert!(result.instructions.is_empty());
        assert_eq!(result.stats.input_instructions, 0);
        assert_eq!(result.stats.output_instructions, 0);
        assert_eq!(result.stats.fusion_count, 0);
    }

    #[test]
    fn statistics_are_correct() {
        let engine = InstructionFusionEngine::new();

        let result = engine.fuse_with_stats(&strings(&[
            "SPIKE_EMIT",
            "MEMBRANE_INTEGRATE",
            "LOAD",
        ]));

        assert_eq!(result.stats.input_instructions, 3);
        assert_eq!(result.stats.output_instructions, 2);
        assert_eq!(result.stats.fusion_count, 1);
        assert_eq!(result.stats.instructions_eliminated, 1);
        assert_eq!(result.stats.reduction_percent(), 100.0 / 3.0);
    }

    #[test]
    fn rule_can_be_removed() {
        let mut engine = InstructionFusionEngine::new();

        assert!(engine.contains_rule(
            "spike_emit_membrane_integrate"
        ));

        assert!(engine.unregister_rule(
            "spike_emit_membrane_integrate"
        ));

        assert!(!engine.contains_rule(
            "spike_emit_membrane_integrate"
        ));
    }

    #[test]
    fn rule_ids_are_deterministic() {
        let mut engine = InstructionFusionEngine::empty();

        engine
            .register_rule(
                FusionRule::new(
                    "z_rule",
                    strings(&["A"]),
                    "Z",
                )
                .unwrap(),
            )
            .unwrap();

        engine
            .register_rule(
                FusionRule::new(
                    "a_rule",
                    strings(&["B"]),
                    "A",
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            engine.rule_ids(),
            vec!["a_rule", "z_rule"]
        );
    }

    #[test]
    fn convenience_function_matches_engine_api() {
        let input = strings(&[
            "SPIKE_EMIT",
            "MEMBRANE_INTEGRATE",
        ]);

        let direct = fuse_instructions(&input);
        let engine = InstructionFusionEngine::new().fuse(&input);

        assert_eq!(direct, engine);
    }
}