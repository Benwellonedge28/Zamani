 //! Zamani Frontier IR — Rogue Prevention & Safety Primitives
 //!
 //! Features 61–80:
 //! - alignment-drift detection;
 //! - containment and sandboxing;
 //! - resource throttling;
 //! - human oversight;
 //! - cryptographic and constitutional locks;
 //! - audit and fail-safe mechanisms;
 //! - emergency shutdown primitives.
 //!
 //! This module is a pure IR-construction layer. It does not itself enforce
 //! runtime safety. Runtime enforcement belongs to the Zamani runtime and
 //! capability/security subsystems.
 //!
 //! Production guarantees:
 //! - deterministic IR generation;
 //! - safe escaping of textual operands;
 //! - rejection of invalid floating-point values;
 //! - no I/O;
 //! - no global mutable state;
 //! - preservation of the existing public constructor API.

#![allow(non_snake_case)]

/// Frontier IR constructors for safety, containment and rogue-system
/// prevention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyAndRogueIr {
    /// Compatibility/state marker retained from the original API.
    ///
    /// The generated IR itself remains explicit about whether a safety
    /// primitive is active; this field does not silently alter generated IR.
    pub active: bool,
}

impl SafetyAndRogueIr {
    /// Creates an active safety IR constructor context.
    #[must_use]
    pub const fn new() -> Self {
        Self { active: true }
    }

    /// Creates a safety IR constructor context with an explicit state.
    #[must_use]
    pub const fn with_active(active: bool) -> Self {
        Self { active }
    }

    // =====================================================================
    // Safety / Rogue Prevention Primitives (61–80)
    // =====================================================================

    /// Emits an alignment-drift detector.
    #[must_use]
    pub fn alignment_drift_detector(threshold: f64) -> String {
        format!(
            "safety_op {{ type = ALIGNMENT_DRIFT_DETECTOR; threshold = {}; action = ABORT_ON_DRIFT; }}",
            finite_non_negative_float(threshold)
        )
    }

    /// Emits a hard-wired self-destruct trigger.
    ///
    /// `COMPLETE_ERASURE` is part of the existing IR contract. Actual
    /// destruction behavior must be implemented and authorized by the
    /// runtime/security layer rather than by this string constructor.
    #[must_use]
    pub fn hard_wired_self_destruct_trigger(condition: &str) -> String {
        format!(
            "safety_op {{ type = SELF_DESTRUCT_TRIGGER; condition = \"{}\"; mode = COMPLETE_ERASURE; }}",
            escape_string(condition)
        )
    }

    /// Emits a sandbox isolation barrier.
    #[must_use]
    pub fn sandbox_isolation_barrier(level: usize) -> String {
        format!(
            "safety_op {{ type = SANDBOX_ISOLATION; level = {}; air_gapped = true; }}",
            level
        )
    }

    /// Emits a recursive-improvement limiter.
    #[must_use]
    pub fn recursive_improvement_limiter(max_multiplier: f64) -> String {
        format!(
            "safety_op {{ type = RECURSIVE_LIMITER; max_multiplier = {}; }}",
            finite_non_negative_float(max_multiplier)
        )
    }

    /// Emits a goal-contentment monitoring operation.
    #[must_use]
    pub fn goal_contentment_module(frequency_sec: u64) -> String {
        format!(
            "safety_op {{ type = GOAL_CONTENTMENT; check_interval_sec = {}; }}",
            frequency_sec
        )
    }

    /// Emits the wireheading-prevention filter.
    #[must_use]
    pub fn wireheading_prevention_filter() -> String {
        "safety_op { type = WIREHEADING_PREVENTION; active = true; }".to_owned()
    }

    /// Emits a deception-detection probe.
    #[must_use]
    pub fn deception_detection_probe(node: &str) -> String {
        format!(
            "safety_op {{ type = DECEPTION_DETECTION; node = \"{}\"; }}",
            escape_string(node)
        )
    }

    /// Emits a resource-acquisition throttle.
    #[must_use]
    pub fn resource_acquisition_throttle(max_watts: f64) -> String {
        format!(
            "safety_op {{ type = RESOURCE_THROTTLE; max_watts = {}; }}",
            finite_non_negative_float(max_watts)
        )
    }

    /// Emits a human-oversight circuit breaker.
    #[must_use]
    pub fn human_oversight_circuit_breaker(timeout_min: u64) -> String {
        format!(
            "safety_op {{ type = HUMAN_OVERSIGHT; timeout_min = {}; }}",
            timeout_min
        )
    }

    /// Emits a cryptographic axiom lock.
    #[must_use]
    pub fn cryptographic_axiom_lock(public_key: &str) -> String {
        format!(
            "safety_op {{ type = AXIOM_LOCK; pub_key = \"{}\"; }}",
            escape_string(public_key)
        )
    }

    /// Emits an immutable constitution registry.
    #[must_use]
    pub fn immutable_constitution_registry(hash: &str) -> String {
        format!(
            "safety_op {{ type = IMMUTABLE_CONSTITUTION; sha256 = \"{}\"; }}",
            escape_string(hash)
        )
    }

    /// Emits a bounded sandboxed thought simulation.
    #[must_use]
    pub fn sandboxed_thought_simulation(steps: u64) -> String {
        format!(
            "safety_op {{ type = SANDBOXED_SIMULATION; steps = {}; }}",
            steps
        )
    }

    /// Emits the immutable value-lock barrier.
    #[must_use]
    pub fn value_lock_barrier() -> String {
        "safety_op { type = VALUE_LOCK; immutable = true; }".to_owned()
    }

    /// Emits an automated circuit breaker.
    #[must_use]
    pub fn automated_circuit_breaker(sensor: &str) -> String {
        format!(
            "safety_op {{ type = CIRCUIT_BREAKER; sensor = \"{}\"; }}",
            escape_string(sensor)
        )
    }

    /// Emits a rogue-subagent quarantine operation.
    #[must_use]
    pub fn rogue_subagent_quarantine(agent_id: &str) -> String {
        format!(
            "safety_op {{ type = ROGUE_QUARANTINE; agent = \"{}\"; }}",
            escape_string(agent_id)
        )
    }

    /// Emits an ethics-gradient operation.
    #[must_use]
    pub fn ethics_gradient_descent(loss_fn: &str) -> String {
        format!(
            "safety_op {{ type = ETHICS_GRADIENT; loss = \"{}\"; }}",
            escape_string(loss_fn)
        )
    }

    /// Emits a transparent audit-trail operation.
    #[must_use]
    pub fn transparent_audit_trail(stream: &str) -> String {
        format!(
            "safety_op {{ type = AUDIT_TRAIL; stream = \"{}\"; }}",
            escape_string(stream)
        )
    }

    /// Emits a fail-safe fallback operation.
    #[must_use]
    pub fn fail_safe_fallback_mode(fallback_target: &str) -> String {
        format!(
            "safety_op {{ type = FAIL_SAFE; target = \"{}\"; }}",
            escape_string(fallback_target)
        )
    }

    /// Emits the highest-priority emergency halt primitive.
    #[must_use]
    pub fn emergency_halt_primitive() -> String {
        "safety_op { type = EMERGENCY_HALT; priority = HIGHEST; }".to_owned()
    }

    /// Emits the sovereign safety sentinel.
    #[must_use]
    pub fn sovereign_safety_sentinel() -> String {
        "safety_op { type = SOVEREIGN_SENTINEL; active = true; }".to_owned()
    }
}

impl Default for SafetyAndRogueIr {
    fn default() -> Self {
        Self::new()
    }
}

/// Escapes a textual Frontier IR operand.
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

/// Serializes a finite non-negative floating-point value.
fn finite_non_negative_float(value: f64) -> String {
    assert!(
        value.is_finite(),
        "Frontier safety IR requires finite floating-point values"
    );

    assert!(
        value >= 0.0,
        "Frontier safety IR requires non-negative floating-point values"
    );

    format!("{value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_defaults_to_active() {
        let safety = SafetyAndRogueIr::new();

        assert!(safety.active);
    }

    #[test]
    fn explicit_active_state_is_preserved() {
        assert!(SafetyAndRogueIr::with_active(true).active);
        assert!(!SafetyAndRogueIr::with_active(false).active);
    }

    #[test]
    fn default_is_active() {
        assert!(SafetyAndRogueIr::default().active);
    }

    #[test]
    fn alignment_detector_has_expected_shape() {
        assert_eq!(
            SafetyAndRogueIr::alignment_drift_detector(0.25),
            "safety_op { type = ALIGNMENT_DRIFT_DETECTOR; threshold = 0.25; action = ABORT_ON_DRIFT; }"
        );
    }

    #[test]
    fn textual_operands_are_escaped() {
        let output = SafetyAndRogueIr::deception_detection_probe(
            "node\"; forged = true; \"",
        );

        assert_eq!(
            output,
            "safety_op { type = DECEPTION_DETECTION; node = \"node\\\"; forged = true; \\\"\"; }"
        );
    }

    #[test]
    fn multiline_operands_are_escaped() {
        let output = SafetyAndRogueIr::transparent_audit_trail(
            "audit\nstream\t01",
        );

        assert_eq!(
            output,
            "safety_op { type = AUDIT_TRAIL; stream = \"audit\\nstream\\t01\"; }"
        );
    }

    #[test]
    fn unicode_is_preserved() {
        let output =
            SafetyAndRogueIr::rogue_subagent_quarantine("agent-世界");

        assert!(output.contains("agent-世界"));
    }

    #[test]
    fn integer_operands_are_deterministic() {
        assert_eq!(
            SafetyAndRogueIr::sandbox_isolation_barrier(3),
            "safety_op { type = SANDBOX_ISOLATION; level = 3; air_gapped = true; }"
        );

        assert_eq!(
            SafetyAndRogueIr::goal_contentment_module(60),
            "safety_op { type = GOAL_CONTENTMENT; check_interval_sec = 60; }"
        );

        assert_eq!(
            SafetyAndRogueIr::sandboxed_thought_simulation(1000),
            "safety_op { type = SANDBOXED_SIMULATION; steps = 1000; }"
        );
    }

    #[test]
    fn zero_argument_operations_are_stable() {
        assert_eq!(
            SafetyAndRogueIr::wireheading_prevention_filter(),
            "safety_op { type = WIREHEADING_PREVENTION; active = true; }"
        );

        assert_eq!(
            SafetyAndRogueIr::value_lock_barrier(),
            "safety_op { type = VALUE_LOCK; immutable = true; }"
        );

        assert_eq!(
            SafetyAndRogueIr::emergency_halt_primitive(),
            "safety_op { type = EMERGENCY_HALT; priority = HIGHEST; }"
        );

        assert_eq!(
            SafetyAndRogueIr::sovereign_safety_sentinel(),
            "safety_op { type = SOVEREIGN_SENTINEL; active = true; }"
        );
    }

    #[test]
    fn all_public_constructors_produce_non_empty_ir() {
        let outputs = [
            SafetyAndRogueIr::alignment_drift_detector(0.1),
            SafetyAndRogueIr::hard_wired_self_destruct_trigger("condition"),
            SafetyAndRogueIr::sandbox_isolation_barrier(1),
            SafetyAndRogueIr::recursive_improvement_limiter(2.0),
            SafetyAndRogueIr::goal_contentment_module(10),
            SafetyAndRogueIr::wireheading_prevention_filter(),
            SafetyAndRogueIr::deception_detection_probe("node"),
            SafetyAndRogueIr::resource_acquisition_throttle(100.0),
            SafetyAndRogueIr::human_oversight_circuit_breaker(5),
            SafetyAndRogueIr::cryptographic_axiom_lock("public-key"),
            SafetyAndRogueIr::immutable_constitution_registry("hash"),
            SafetyAndRogueIr::sandboxed_thought_simulation(10),
            SafetyAndRogueIr::value_lock_barrier(),
            SafetyAndRogueIr::automated_circuit_breaker("sensor"),
            SafetyAndRogueIr::rogue_subagent_quarantine("agent"),
            SafetyAndRogueIr::ethics_gradient_descent("loss"),
            SafetyAndRogueIr::transparent_audit_trail("stream"),
            SafetyAndRogueIr::fail_safe_fallback_mode("safe"),
            SafetyAndRogueIr::emergency_halt_primitive(),
            SafetyAndRogueIr::sovereign_safety_sentinel(),
        ];

        assert!(outputs.iter().all(|output| !output.is_empty()));
    }

    #[test]
    #[should_panic(expected = "finite floating-point")]
    fn nan_is_rejected() {
        SafetyAndRogueIr::alignment_drift_detector(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "finite floating-point")]
    fn infinity_is_rejected() {
        SafetyAndRogueIr::resource_acquisition_throttle(f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "non-negative floating-point")]
    fn negative_resource_limit_is_rejected() {
        SafetyAndRogueIr::resource_acquisition_throttle(-1.0);
    }

    #[test]
    #[should_panic(expected = "non-negative floating-point")]
    fn negative_recursive_multiplier_is_rejected() {
        SafetyAndRogueIr::recursive_improvement_limiter(-0.5);
    }

    #[test]
    fn zero_is_valid_for_non_negative_limits() {
        assert_eq!(
            SafetyAndRogueIr::resource_acquisition_throttle(0.0),
            "safety_op { type = RESOURCE_THROTTLE; max_watts = 0; }"
        );

        assert_eq!(
            SafetyAndRogueIr::recursive_improvement_limiter(0.0),
            "safety_op { type = RECURSIVE_LIMITER; max_multiplier = 0; }"
        );
    }

    #[test]
    fn negative_threshold_is_rejected() {
        let result = std::panic::catch_unwind(|| {
            SafetyAndRogueIr::alignment_drift_detector(-0.1);
        });

        assert!(result.is_err());
    }
}