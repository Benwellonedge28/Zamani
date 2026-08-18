//! Zamani Frontier IR — Cognitive, AGI & ASI Primitives
//!
//! Features 41–60:
//! - neural and cognitive primitives;
//! - memory and representation primitives;
//! - learning and reasoning primitives;
//! - AGI/ASI monitoring and safety anchors.
//!
//! This module is a pure Frontier-IR construction layer.
//!
//! Production guarantees:
//! - deterministic output;
//! - no I/O or global mutable state;
//! - safe escaping of textual operands;
//! - rejection of non-finite floating-point values;
//! - preservation of the existing public constructor API;
//! - unit coverage for all public constructors.

#![allow(non_snake_case)]

/// Frontier IR constructors for cognitive, AGI and ASI-related operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct CognitiveAndAsiIr;

impl CognitiveAndAsiIr {
    // =====================================================================
    // Cognitive / AGI / ASI Primitives (41–60)
    // =====================================================================

    /// Emits a neural-plasticity operation.
    #[must_use]
    pub fn neural_plasticity_node(layer: &str, rate: f64) -> String {
        format!(
            "cognitive_op {{ type = NEURAL_PLASTICITY; layer = \"{}\"; rate = {}; }}",
            escape_string(layer),
            finite_float(rate)
        )
    }

    /// Emits an attention-focus operation.
    #[must_use]
    pub fn dynamic_attention_focus(head_id: usize, weight: f64) -> String {
        format!(
            "cognitive_op {{ type = ATTENTION_FOCUS; head = {}; weight = {}; }}",
            head_id,
            finite_float(weight)
        )
    }

    /// Emits a knowledge-distillation operation.
    #[must_use]
    pub fn knowledge_distillation_pipe(teacher: &str, student: &str) -> String {
        format!(
            "cognitive_op {{ type = KNOWLEDGE_DISTILLATION; teacher = \"{}\"; student = \"{}\"; }}",
            escape_string(teacher),
            escape_string(student)
        )
    }

    /// Emits a recursive self-improvement operation.
    #[must_use]
    pub fn recursive_self_improvement_loop(generation: u64) -> String {
        format!(
            "cognitive_op {{ type = RECURSIVE_SELF_IMPROVEMENT; gen = {}; }}",
            generation
        )
    }

    /// Emits an ASI-emergence monitoring operation.
    #[must_use]
    pub fn asi_emergence_monitor(threshold: f64) -> String {
        format!(
            "cognitive_op {{ type = ASI_EMERGENCE_MONITOR; threshold = {}; }}",
            finite_float(threshold)
        )
    }

    /// Emits a neuro-symbolic bridge operation.
    #[must_use]
    pub fn neuro_symbolic_bridge(symbolic_rule: &str, neural_net: &str) -> String {
        format!(
            "cognitive_op {{ type = NEURO_SYMBOLIC; rule = \"{}\"; net = \"{}\"; }}",
            escape_string(symbolic_rule),
            escape_string(neural_net)
        )
    }

    /// Emits a working-memory capacity declaration.
    #[must_use]
    pub fn working_memory_cache(capacity_tokens: usize) -> String {
        format!(
            "cognitive_op {{ type = WORKING_MEMORY; tokens = {}; }}",
            capacity_tokens
        )
    }

    /// Emits an episodic-memory storage operation.
    #[must_use]
    pub fn episodic_memory_store(event_tag: &str) -> String {
        format!(
            "cognitive_op {{ type = EPISODIC_STORE; tag = \"{}\"; }}",
            escape_string(event_tag)
        )
    }

    /// Emits a semantic-network embedding operation.
    #[must_use]
    pub fn semantic_network_embedding(dimension: usize) -> String {
        format!(
            "cognitive_op {{ type = SEMANTIC_EMBEDDING; dim = {}; }}",
            dimension
        )
    }

    /// Emits a meta-learning optimizer operation.
    #[must_use]
    pub fn metalearning_meta_optimizer(algorithm: &str) -> String {
        format!(
            "cognitive_op {{ type = METALEARNING; algo = \"{}\"; }}",
            escape_string(algorithm)
        )
    }

    /// Emits a synthetic-intuition heuristic operation.
    #[must_use]
    pub fn synthetic_intuition_heuristic(bias: f64) -> String {
        format!(
            "cognitive_op {{ type = SYNTHETIC_INTUITION; bias = {}; }}",
            finite_float(bias)
        )
    }

    /// Emits a theory-of-mind model operation.
    #[must_use]
    pub fn theory_of_mind_model(agent_id: &str) -> String {
        format!(
            "cognitive_op {{ type = THEORY_OF_MIND; agent = \"{}\"; }}",
            escape_string(agent_id)
        )
    }

    /// Emits a cognitive-load balancing operation.
    #[must_use]
    pub fn cognitive_load_balancer(max_flops: f64) -> String {
        format!(
            "cognitive_op {{ type = COGNITIVE_LOAD_BALANCER; max_flops = {}; }}",
            finite_non_negative_float(max_flops)
        )
    }

    /// Emits a hyper-dimensional binding operation.
    #[must_use]
    pub fn hyper_dimensional_binding(vector_a: &str, vector_b: &str) -> String {
        format!(
            "cognitive_op {{ type = HD_BINDING; a = \"{}\"; b = \"{}\"; }}",
            escape_string(vector_a),
            escape_string(vector_b)
        )
    }

    /// Emits a zero-shot generalization guard.
    #[must_use]
    pub fn zero_shot_generalization_guard() -> String {
        "cognitive_op { type = ZERO_SHOT_GUARD; }".to_owned()
    }

    /// Emits an autonomous hypothesis-generator hook.
    #[must_use]
    pub fn autonomous_hypothesis_generator() -> String {
        "cognitive_op { type = HYPOTHESIS_GENERATOR; }".to_owned()
    }

    /// Emits an automated theorem-prover hook.
    #[must_use]
    pub fn automated_theorem_prover_hook(theory: &str) -> String {
        format!(
            "cognitive_op {{ type = THEOREM_PROVER; theory = \"{}\"; }}",
            escape_string(theory)
        )
    }

    /// Emits a concept-drift detection operation.
    #[must_use]
    pub fn concept_drift_detector(metric: &str) -> String {
        format!(
            "cognitive_op {{ type = CONCEPT_DRIFT; metric = \"{}\"; }}",
            escape_string(metric)
        )
    }

    /// Emits a collective swarm-intelligence operation.
    #[must_use]
    pub fn collective_swarm_intelligence(node_count: usize) -> String {
        format!(
            "cognitive_op {{ type = SWARM_INTELLIGENCE; nodes = {}; }}",
            node_count
        )
    }

    /// Emits the locked ASI core anchor.
    #[must_use]
    pub fn super_intelligence_core_anchor() -> String {
        "cognitive_op { type = ASI_CORE_ANCHOR; locked = true; }".to_owned()
    }
}

/// Escapes a textual Frontier IR operand.
///
/// Frontier IR uses double-quoted strings, so textual values must not be
/// inserted into the representation verbatim.
fn escape_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\0' => escaped.push_str("\\0"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0C}' => escaped.push_str("\\f"),
            character if character.is_control() => {
                use std::fmt::Write;

                write!(
                    &mut escaped,
                    "\\u{{{:04X}}}",
                    character as u32
                )
                .expect("writing to String cannot fail");
            }
            character => escaped.push(character),
        }
    }

    escaped
}

/// Serializes a finite floating-point value.
///
/// The existing API returns `String`, not `Result<String, _>`, so invalid
/// values are rejected with a deterministic panic instead of being emitted
/// as invalid IR.
fn finite_float(value: f64) -> String {
    assert!(
        value.is_finite(),
        "Frontier IR requires finite floating-point values"
    );

    format!("{value}")
}

/// Serializes a finite non-negative floating-point value.
fn finite_non_negative_float(value: f64) -> String {
    assert!(
        value.is_finite(),
        "Frontier IR requires finite floating-point values"
    );

    assert!(
        value >= 0.0,
        "Frontier IR requires non-negative floating-point values"
    );

    format!("{value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neural_plasticity_has_expected_shape() {
        assert_eq!(
            CognitiveAndAsiIr::neural_plasticity_node("layer_1", 0.1),
            "cognitive_op { type = NEURAL_PLASTICITY; layer = \"layer_1\"; rate = 0.1; }"
        );
    }

    #[test]
    fn textual_operands_are_escaped() {
        let output = CognitiveAndAsiIr::knowledge_distillation_pipe(
            "teacher\"model",
            "student\\model\nv2",
        );

        assert_eq!(
            output,
            "cognitive_op { type = KNOWLEDGE_DISTILLATION; teacher = \"teacher\\\"model\"; student = \"student\\\\model\\nv2\"; }"
        );
    }

    #[test]
    fn unicode_is_preserved() {
        let output =
            CognitiveAndAsiIr::theory_of_mind_model("agent-世界");

        assert!(output.contains("agent-世界"));
    }

    #[test]
    fn integer_operands_are_serialized_deterministically() {
        assert_eq!(
            CognitiveAndAsiIr::dynamic_attention_focus(7, 0.75),
            "cognitive_op { type = ATTENTION_FOCUS; head = 7; weight = 0.75; }"
        );

        assert_eq!(
            CognitiveAndAsiIr::working_memory_cache(4096),
            "cognitive_op { type = WORKING_MEMORY; tokens = 4096; }"
        );

        assert_eq!(
            CognitiveAndAsiIr::recursive_self_improvement_loop(12),
            "cognitive_op { type = RECURSIVE_SELF_IMPROVEMENT; gen = 12; }"
        );
    }

    #[test]
    fn zero_argument_operations_are_stable() {
        assert_eq!(
            CognitiveAndAsiIr::zero_shot_generalization_guard(),
            "cognitive_op { type = ZERO_SHOT_GUARD; }"
        );

        assert_eq!(
            CognitiveAndAsiIr::autonomous_hypothesis_generator(),
            "cognitive_op { type = HYPOTHESIS_GENERATOR; }"
        );

        assert_eq!(
            CognitiveAndAsiIr::super_intelligence_core_anchor(),
            "cognitive_op { type = ASI_CORE_ANCHOR; locked = true; }"
        );
    }

    #[test]
    fn all_public_constructors_produce_non_empty_ir() {
        let outputs = [
            CognitiveAndAsiIr::neural_plasticity_node("layer", 0.1),
            CognitiveAndAsiIr::dynamic_attention_focus(1, 0.5),
            CognitiveAndAsiIr::knowledge_distillation_pipe("teacher", "student"),
            CognitiveAndAsiIr::recursive_self_improvement_loop(1),
            CognitiveAndAsiIr::asi_emergence_monitor(0.9),
            CognitiveAndAsiIr::neuro_symbolic_bridge("rule", "network"),
            CognitiveAndAsiIr::working_memory_cache(100),
            CognitiveAndAsiIr::episodic_memory_store("event"),
            CognitiveAndAsiIr::semantic_network_embedding(768),
            CognitiveAndAsiIr::metalearning_meta_optimizer("adam"),
            CognitiveAndAsiIr::synthetic_intuition_heuristic(0.2),
            CognitiveAndAsiIr::theory_of_mind_model("agent"),
            CognitiveAndAsiIr::cognitive_load_balancer(1_000_000.0),
            CognitiveAndAsiIr::hyper_dimensional_binding("a", "b"),
            CognitiveAndAsiIr::zero_shot_generalization_guard(),
            CognitiveAndAsiIr::autonomous_hypothesis_generator(),
            CognitiveAndAsiIr::automated_theorem_prover_hook("theory"),
            CognitiveAndAsiIr::concept_drift_detector("loss"),
            CognitiveAndAsiIr::collective_swarm_intelligence(8),
            CognitiveAndAsiIr::super_intelligence_core_anchor(),
        ];

        assert!(outputs.iter().all(|output| !output.is_empty()));
    }

    #[test]
    #[should_panic(expected = "finite floating-point")]
    fn nan_is_rejected() {
        CognitiveAndAsiIr::asi_emergence_monitor(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "finite floating-point")]
    fn positive_infinity_is_rejected() {
        CognitiveAndAsiIr::neural_plasticity_node("layer", f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "finite floating-point")]
    fn negative_infinity_is_rejected() {
        CognitiveAndAsiIr::synthetic_intuition_heuristic(f64::NEG_INFINITY);
    }

    #[test]
    #[should_panic(expected = "non-negative floating-point")]
    fn negative_flops_limit_is_rejected() {
        CognitiveAndAsiIr::cognitive_load_balancer(-1.0);
    }

    #[test]
    fn negative_regular_float_remains_supported() {
        let output =
            CognitiveAndAsiIr::synthetic_intuition_heuristic(-0.25);

        assert!(output.contains("bias = -0.25"));
    }

    #[test]
    fn special_characters_cannot_escape_the_string_operand() {
        let output = CognitiveAndAsiIr::episodic_memory_store(
            "tag\"; malicious = true; \"",
        );

        assert_eq!(
            output,
            "cognitive_op { type = EPISODIC_STORE; tag = \"tag\\\"; malicious = true; \\\"\"; }"
        );
    }
}