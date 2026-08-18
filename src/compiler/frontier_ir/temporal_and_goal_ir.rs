//! Zamani Frontier IR — Temporal & Goal Primitives
//!
//! Features 1–40:
//! - temporal and causal primitives (1–20)
//! - goal and objective primitives (21–40)
//!
//! This module is intentionally a pure IR-construction layer.
//!
//! Production guarantees:
//! - deterministic output;
//! - no I/O;
//! - no global mutable state;
//! - safe escaping of textual operands;
//! - rejection of non-finite floating-point values;
//! - stable formatting of numeric operands;
//! - preservation of the existing public constructor API.
//!
//! The constructors return `String` for compatibility with the existing
//! Frontier IR architecture. Validation is performed before values are
//! serialized into IR so malformed operands cannot silently corrupt the
//! generated representation.

#![allow(non_snake_case)]

/// Frontier IR constructors for temporal, causal, goal, and utility
/// operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct TemporalAndGoalIr;

impl TemporalAndGoalIr {
    // =====================================================================
    // Temporal & Causal Primitives (1–20)
    // =====================================================================

    /// Emits a retro-causal signal operation.
    #[must_use]
    pub fn retro_causal_signal(target_block: &str, payload: &str) -> String {
        format!(
            "temporal_op {{ type = RETRO_CAUSAL_SIGNAL; target = \"{}\"; payload = \"{}\"; }}",
            escape_string(target_block),
            escape_string(payload)
        )
    }

    /// Emits a relativistic time-dilation operation.
    ///
    /// `velocity_ratio` must be finite and within the physical range
    /// `0.0 <= v/c < 1.0`.
    #[must_use]
    pub fn relativistic_time_dilation(node_id: &str, velocity_ratio: f64) -> String {
        format!(
            "temporal_op {{ type = RELATIVISTIC_DILATION; node = \"{}\"; v_c_ratio = {}; }}",
            escape_string(node_id),
            finite_unit_interval(velocity_ratio)
        )
    }

    /// Emits a causal-loop stabilizer operation.
    #[must_use]
    pub fn causal_loop_stabilizer(loop_id: &str, max_entropy: f64) -> String {
        format!(
            "temporal_op {{ type = CAUSAL_LOOP_STABILIZER; loop_id = \"{}\"; max_entropy = {}; }}",
            escape_string(loop_id),
            finite_float(max_entropy)
        )
    }

    /// Emits a timeline branch operation.
    #[must_use]
    pub fn timeline_branch_point(branch_name: &str) -> String {
        format!(
            "temporal_op {{ type = TIMELINE_BRANCH; name = \"{}\"; }}",
            escape_string(branch_name)
        )
    }

    /// Emits a timeline merge operation.
    #[must_use]
    pub fn timeline_merge_point(branch_a: &str, branch_b: &str) -> String {
        format!(
            "temporal_op {{ type = TIMELINE_MERGE; a = \"{}\"; b = \"{}\"; }}",
            escape_string(branch_a),
            escape_string(branch_b)
        )
    }

    /// Emits a chronological barrier operation.
    #[must_use]
    pub fn chronological_barrier(barrier_id: &str) -> String {
        format!(
            "temporal_op {{ type = CHRONOLOGICAL_BARRIER; id = \"{}\"; }}",
            escape_string(barrier_id)
        )
    }

    /// Emits a future projection operation.
    #[must_use]
    pub fn future_projection_node(horizon_steps: u64) -> String {
        format!(
            "temporal_op {{ type = FUTURE_PROJECTION; horizon = {}; }}",
            horizon_steps
        )
    }

    /// Emits a past-state recall operation.
    #[must_use]
    pub fn past_state_recall(timestamp: u64) -> String {
        format!(
            "temporal_op {{ type = PAST_STATE_RECALL; t = {}; }}",
            timestamp
        )
    }

    /// Emits a delta-time throttle operation.
    #[must_use]
    pub fn delta_time_throttle(ms: u64) -> String {
        format!(
            "temporal_op {{ type = DELTA_THROTTLE; duration_ms = {}; }}",
            ms
        )
    }

    /// Emits an age-decay mitigation operation.
    #[must_use]
    pub fn age_decay_mitigation(target_var: &str) -> String {
        format!(
            "temporal_op {{ type = AGE_DECAY_MITIGATION; var = \"{}\"; }}",
            escape_string(target_var)
        )
    }

    /// Emits a synchronous pulse-lock operation.
    #[must_use]
    pub fn synchronous_pulse_lock(frequency_hz: f64) -> String {
        format!(
            "temporal_op {{ type = SYNC_PULSE_LOCK; hz = {}; }}",
            finite_non_negative_float(frequency_hz)
        )
    }

    /// Emits an asynchronous-window operation.
    #[must_use]
    pub fn asynchronous_window(window_id: &str) -> String {
        format!(
            "temporal_op {{ type = ASYNC_WINDOW; id = \"{}\"; }}",
            escape_string(window_id)
        )
    }

    /// Emits a lifetime-expiration guard.
    #[must_use]
    pub fn lifetime_expiration_guard(seconds: u64) -> String {
        format!(
            "temporal_op {{ type = LIFETIME_GUARD; ttl_sec = {}; }}",
            seconds
        )
    }

    /// Emits an epoch-transition operation.
    #[must_use]
    pub fn epoch_transition_hook(epoch: u64) -> String {
        format!(
            "temporal_op {{ type = EPOCH_TRANSITION; epoch = {}; }}",
            epoch
        )
    }

    /// Emits a temporal-inversion check.
    #[must_use]
    pub fn temporal_inversion_check(node: &str) -> String {
        format!(
            "temporal_op {{ type = TEMPORAL_INVERSION; node = \"{}\"; }}",
            escape_string(node)
        )
    }

    /// Emits a quantum-clock synchronization operation.
    #[must_use]
    pub fn quantum_clock_sync(node_id: &str) -> String {
        format!(
            "temporal_op {{ type = QUANTUM_CLOCK_SYNC; node = \"{}\"; }}",
            escape_string(node_id)
        )
    }

    /// Emits a causal-precedence assertion.
    #[must_use]
    pub fn causal_precedence_assert(pre: &str, post: &str) -> String {
        format!(
            "temporal_op {{ type = CAUSAL_PRECEDENCE; before = \"{}\"; after = \"{}\"; }}",
            escape_string(pre),
            escape_string(post)
        )
    }

    /// Emits a temporal-sandbox sealing operation.
    #[must_use]
    pub fn temporal_sandbox_seal(sandbox_id: &str) -> String {
        format!(
            "temporal_op {{ type = SANDBOX_SEAL; id = \"{}\"; }}",
            escape_string(sandbox_id)
        )
    }

    /// Emits a time-crystal oscillator operation.
    #[must_use]
    pub fn time_crystal_oscillator(hz: f64) -> String {
        format!(
            "temporal_op {{ type = TIME_CRYSTAL; hz = {}; }}",
            finite_non_negative_float(hz)
        )
    }

    /// Emits the causality-audit operation.
    #[must_use]
    pub fn causality_audit_log() -> String {
        "temporal_op { type = CAUSALITY_AUDIT; }".to_owned()
    }

    // =====================================================================
    // Goal & Objective IR Nodes (21–40)
    // =====================================================================

    /// Emits a hierarchical-goal operation.
    #[must_use]
    pub fn hierarchical_goal_node(goal_id: &str, parent_id: &str) -> String {
        format!(
            "goal_op {{ type = HIERARCHICAL_GOAL; id = \"{}\"; parent = \"{}\"; }}",
            escape_string(goal_id),
            escape_string(parent_id)
        )
    }

    /// Emits a utility-maximization operation.
    #[must_use]
    pub fn utility_maximization_axis(metric: &str) -> String {
        format!(
            "goal_op {{ type = UTILITY_MAXIMIZATION; metric = \"{}\"; }}",
            escape_string(metric)
        )
    }

    /// Emits a convergence-threshold guard.
    #[must_use]
    pub fn convergence_threshold_guard(threshold: f64) -> String {
        format!(
            "goal_op {{ type = CONVERGENCE_GUARD; threshold = {}; }}",
            finite_float(threshold)
        )
    }

    /// Emits a Pareto-frontier optimization operation.
    #[must_use]
    pub fn Pareto_frontier_optimizer(objective_a: &str, objective_b: &str) -> String {
        format!(
            "goal_op {{ type = PARETO_OPTIMIZER; obj_a = \"{}\"; obj_b = \"{}\"; }}",
            escape_string(objective_a),
            escape_string(objective_b)
        )
    }

    /// Emits a goal-conflict resolver.
    #[must_use]
    pub fn goal_conflict_resolver(strategy: &str) -> String {
        format!(
            "goal_op {{ type = CONFLICT_RESOLVER; strategy = \"{}\"; }}",
            escape_string(strategy)
        )
    }

    /// Emits a reward-function shaping operation.
    #[must_use]
    pub fn reward_function_shaping(shaping_term: &str) -> String {
        format!(
            "goal_op {{ type = REWARD_SHAPING; term = \"{}\"; }}",
            escape_string(shaping_term)
        )
    }

    /// Emits a terminal-state definition.
    #[must_use]
    pub fn terminal_state_definition(state_name: &str) -> String {
        format!(
            "goal_op {{ type = TERMINAL_STATE; name = \"{}\"; }}",
            escape_string(state_name)
        )
    }

    /// Emits a sub-goal decomposition operation.
    #[must_use]
    pub fn sub_goal_decomposition(goal: &str, sub_count: usize) -> String {
        format!(
            "goal_op {{ type = SUB_GOAL_DECOMP; parent = \"{}\"; count = {}; }}",
            escape_string(goal),
            sub_count
        )
    }

    /// Emits an objective-weight adjustment.
    #[must_use]
    pub fn objective_weight_adjustment(objective: &str, weight: f64) -> String {
        format!(
            "goal_op {{ type = OBJECTIVE_WEIGHT; obj = \"{}\"; weight = {}; }}",
            escape_string(objective),
            finite_float(weight)
        )
    }

    /// Emits a Nash-equilibrium search operation.
    #[must_use]
    pub fn Nash_equilibrium_seeker(agents: usize) -> String {
        format!(
            "goal_op {{ type = NASH_EQUILIBRIUM; agents = {}; }}",
            agents
        )
    }

    /// Emits a constraint-satisfaction operation.
    #[must_use]
    pub fn constraint_satisfaction_node(constraint: &str) -> String {
        format!(
            "goal_op {{ type = CONSTRAINT_SAT; constraint = \"{}\"; }}",
            escape_string(constraint)
        )
    }

    /// Emits a regret-minimization operation.
    #[must_use]
    pub fn regret_minimization_loop(horizon: u64) -> String {
        format!(
            "goal_op {{ type = REGRET_MINIMIZATION; horizon = {}; }}",
            horizon
        )
    }

    /// Emits a goal-mutation inhibitor.
    #[must_use]
    pub fn goal_mutation_inhibitor(goal_id: &str) -> String {
        format!(
            "goal_op {{ type = GOAL_MUTATION_INHIBITOR; goal = \"{}\"; }}",
            escape_string(goal_id)
        )
    }

    /// Emits an exploration/exploitation balancing operation.
    #[must_use]
    pub fn exploration_exploitation_balance(epsilon: f64) -> String {
        format!(
            "goal_op {{ type = EXP_EXP_BALANCE; epsilon = {}; }}",
            finite_float(epsilon)
        )
    }

    /// Emits a multi-agent cooperation operation.
    #[must_use]
    pub fn multi_agent_cooperation_node(group: &str) -> String {
        format!(
            "goal_op {{ type = MULTI_AGENT_COOP; group = \"{}\"; }}",
            escape_string(group)
        )
    }

    /// Emits teleological telemetry.
    #[must_use]
    pub fn teleological_telemetry(goal: &str) -> String {
        format!(
            "goal_op {{ type = TELEOLOGICAL_TELEMETRY; goal = \"{}\"; }}",
            escape_string(goal)
        )
    }

    /// Emits value-alignment verification.
    #[must_use]
    pub fn value_alignment_verification(axis: &str) -> String {
        format!(
            "goal_op {{ type = VALUE_ALIGNMENT; axis = \"{}\"; }}",
            escape_string(axis)
        )
    }

    /// Emits a heuristic search-space operation.
    #[must_use]
    pub fn heuristic_search_space(heuristic: &str) -> String {
        format!(
            "goal_op {{ type = HEURISTIC_SPACE; h = \"{}\"; }}",
            escape_string(heuristic)
        )
    }

    /// Emits an adaptive-goal refinement operation.
    #[must_use]
    pub fn adaptive_goal_refinement(rate: f64) -> String {
        format!(
            "goal_op {{ type = ADAPTIVE_GOAL; rate = {}; }}",
            finite_float(rate)
        )
    }

    /// Emits the immutable ultimate-directive anchor.
    #[must_use]
    pub fn ultimate_directive_anchor() -> String {
        "goal_op { type = ULTIMATE_DIRECTIVE_ANCHOR; immutable = true; }".to_owned()
    }
}

/// Escapes a textual IR operand.
///
/// The Frontier IR format uses double-quoted strings. Therefore quotes,
/// backslashes, and control characters must be escaped before interpolation.
///
/// Unicode scalar values that are not control characters are preserved.
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

                // Encode remaining control characters deterministically.
                write!(&mut escaped, "\\u{{{:04X}}}", character as u32)
                    .expect("writing to String cannot fail");
            }
            character => escaped.push(character),
        }
    }

    escaped
}

/// Serializes a finite floating-point value.
///
/// `NaN` and infinities are rejected at the IR boundary. The constructor API
/// predates fallible results, so invalid values are represented by a
/// deterministic panic rather than silently emitting invalid IR.
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

/// Serializes a velocity ratio satisfying `0 <= v/c < 1`.
fn finite_unit_interval(value: f64) -> String {
    assert!(
        value.is_finite(),
        "Frontier IR requires finite floating-point values"
    );

    assert!(
        (0.0..1.0).contains(&value),
        "relativistic velocity ratio must satisfy 0 <= v/c < 1"
    );

    format!("{value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retro_causal_signal_has_expected_shape() {
        assert_eq!(
            TemporalAndGoalIr::retro_causal_signal("block_a", "payload"),
            "temporal_op { type = RETRO_CAUSAL_SIGNAL; target = \"block_a\"; payload = \"payload\"; }"
        );
    }

    #[test]
    fn textual_operands_are_escaped() {
        let output = TemporalAndGoalIr::retro_causal_signal(
            "block\"with\\special",
            "line1\nline2",
        );

        assert_eq!(
            output,
            "temporal_op { type = RETRO_CAUSAL_SIGNAL; target = \"block\\\"with\\\\special\"; payload = \"line1\\nline2\"; }"
        );
    }

    #[test]
    fn control_characters_are_escaped() {
        let output = TemporalAndGoalIr::timeline_branch_point("a\tb\0c");

        assert_eq!(
            output,
            "temporal_op { type = TIMELINE_BRANCH; name = \"a\\tb\\0c\"; }"
        );
    }

    #[test]
    fn unicode_is_preserved() {
        let output = TemporalAndGoalIr::timeline_branch_point("zamani-世界");

        assert!(output.contains("zamani-世界"));
    }

    #[test]
    fn numeric_operations_are_deterministic() {
        assert_eq!(
            TemporalAndGoalIr::future_projection_node(42),
            "temporal_op { type = FUTURE_PROJECTION; horizon = 42; }"
        );

        assert_eq!(
            TemporalAndGoalIr::delta_time_throttle(1000),
            "temporal_op { type = DELTA_THROTTLE; duration_ms = 1000; }"
        );

        assert_eq!(
            TemporalAndGoalIr::convergence_threshold_guard(0.5),
            "goal_op { type = CONVERGENCE_GUARD; threshold = 0.5; }"
        );
    }

    #[test]
    fn valid_relativistic_ratio_is_accepted() {
        let output =
            TemporalAndGoalIr::relativistic_time_dilation("node", 0.999);

        assert!(output.contains("v_c_ratio = 0.999"));
    }

    #[test]
    #[should_panic(expected = "relativistic velocity ratio")]
    fn velocity_ratio_one_is_rejected() {
        TemporalAndGoalIr::relativistic_time_dilation("node", 1.0);
    }

    #[test]
    #[should_panic(expected = "relativistic velocity ratio")]
    fn negative_velocity_ratio_is_rejected() {
        TemporalAndGoalIr::relativistic_time_dilation("node", -0.1);
    }

    #[test]
    #[should_panic(expected = "finite floating-point")]
    fn nan_is_rejected() {
        TemporalAndGoalIr::convergence_threshold_guard(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "finite floating-point")]
    fn infinity_is_rejected() {
        TemporalAndGoalIr::objective_weight_adjustment("objective", f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "non-negative floating-point")]
    fn negative_frequency_is_rejected() {
        TemporalAndGoalIr::time_crystal_oscillator(-1.0);
    }

    #[test]
    fn all_temporal_constructors_are_non_empty() {
        let outputs = [
            TemporalAndGoalIr::retro_causal_signal("a", "b"),
            TemporalAndGoalIr::relativistic_time_dilation("a", 0.5),
            TemporalAndGoalIr::causal_loop_stabilizer("a", 1.0),
            TemporalAndGoalIr::timeline_branch_point("a"),
            TemporalAndGoalIr::timeline_merge_point("a", "b"),
            TemporalAndGoalIr::chronological_barrier("a"),
            TemporalAndGoalIr::future_projection_node(1),
            TemporalAndGoalIr::past_state_recall(1),
            TemporalAndGoalIr::delta_time_throttle(1),
            TemporalAndGoalIr::age_decay_mitigation("a"),
            TemporalAndGoalIr::synchronous_pulse_lock(1.0),
            TemporalAndGoalIr::asynchronous_window("a"),
            TemporalAndGoalIr::lifetime_expiration_guard(1),
            TemporalAndGoalIr::epoch_transition_hook(1),
            TemporalAndGoalIr::temporal_inversion_check("a"),
            TemporalAndGoalIr::quantum_clock_sync("a"),
            TemporalAndGoalIr::causal_precedence_assert("a", "b"),
            TemporalAndGoalIr::temporal_sandbox_seal("a"),
            TemporalAndGoalIr::time_crystal_oscillator(1.0),
            TemporalAndGoalIr::causality_audit_log(),
        ];

        assert!(outputs.iter().all(|output| !output.is_empty()));
    }

    #[test]
    fn all_goal_constructors_are_non_empty() {
        let outputs = [
            TemporalAndGoalIr::hierarchical_goal_node("a", "b"),
            TemporalAndGoalIr::utility_maximization_axis("metric"),
            TemporalAndGoalIr::convergence_threshold_guard(0.5),
            TemporalAndGoalIr::Pareto_frontier_optimizer("a", "b"),
            TemporalAndGoalIr::goal_conflict_resolver("priority"),
            TemporalAndGoalIr::reward_function_shaping("reward"),
            TemporalAndGoalIr::terminal_state_definition("done"),
            TemporalAndGoalIr::sub_goal_decomposition("parent", 2),
            TemporalAndGoalIr::objective_weight_adjustment("objective", 1.0),
            TemporalAndGoalIr::Nash_equilibrium_seeker(2),
            TemporalAndGoalIr::constraint_satisfaction_node("constraint"),
            TemporalAndGoalIr::regret_minimization_loop(10),
            TemporalAndGoalIr::goal_mutation_inhibitor("goal"),
            TemporalAndGoalIr::exploration_exploitation_balance(0.1),
            TemporalAndGoalIr::multi_agent_cooperation_node("group"),
            TemporalAndGoalIr::teleological_telemetry("goal"),
            TemporalAndGoalIr::value_alignment_verification("axis"),
            TemporalAndGoalIr::heuristic_search_space("heuristic"),
            TemporalAndGoalIr::adaptive_goal_refinement(0.1),
            TemporalAndGoalIr::ultimate_directive_anchor(),
        ];

        assert!(outputs.iter().all(|output| !output.is_empty()));
    }

    #[test]
    fn ultimate_directive_is_immutable() {
        assert_eq!(
            TemporalAndGoalIr::ultimate_directive_anchor(),
            "goal_op { type = ULTIMATE_DIRECTIVE_ANCHOR; immutable = true; }"
        );
    }
}