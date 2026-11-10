
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler Safety Guard
//!
//! Path: src/compiler/safety_guard.rs
//! Rust: 1.97+
//! Edition: 2021
//!
//! # Purpose
//!
//! Provides deterministic, target-independent inspection of compiler
//! instruction streams for explicitly configured security signatures.
//!
//! This module is an inspection component, not a universal security
//! verifier, semantic analyzer, policy engine, sandbox or runtime monitor.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - instruction-stream inspection;
//! - coordinated threat-signature matching;
//! - precursor-signal tracking;
//! - inspection decisions;
//! - structured inspection events;
//! - configurable operational retention budgets.
//!
//! It does not own:
//! - source grammar or parsing;
//! - semantic validation;
//! - canonical IR definitions;
//! - general authorization;
//! - security policy evaluation;
//! - cryptographic verification;
//! - supply-chain verification;
//! - backend code generation;
//! - hardware allocation;
//! - runtime enforcement.
//!
//! # Canonical integration
//!
//! Grammar:
//!     grammar/security/
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Compiler security:
//!     crate::compiler::security
//!
//! IR verification:
//!     crate::compiler::ir
//!     crate::zuir
//!     authoritative domain IR verifiers
//!
//! Backend:
//!     crate::backend
//!
//! Runtime:
//!     crate::runtime
//!
//! # Scalability
//!
//! No universal identifier-length, instruction-count, substrate-count,
//! signal-count or computing-domain ceiling is imposed.
//!
//! Signal and audit retention budgets are operational configuration.
//! Their default values are not language or hardware limits.
//!
//! A caller may configure operational capacity according to available
//! memory and processing resources.
//!
//! No implementation can promise infinite physical memory.
//!
//! # Security limitations
//!
//! A signature match is evidence of a configured pattern, not proof of
//! malicious behavior.
//!
//! Absence of a signature match is not proof of program safety.
//!
//! Mandatory semantic, IR, policy, artifact and supply-chain verification
//! remains the responsibility of the authoritative security subsystem.
//!
//! This module never executes inspected instructions.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock};

// ============================================================================
// 1. OPERATIONAL DEFAULTS
// ============================================================================

/// Default operational signal-retention budget.
///
/// This is configurable and is not a semantic or hardware ceiling.
pub const DEFAULT_MAX_SIGNALS: usize = 4_096;

/// Default operational audit-retention budget.
///
/// This is configurable and is not a universal audit-event ceiling.
pub const DEFAULT_MAX_AUDIT_EVENTS: usize = 8_192;

// ============================================================================
// 2. DECISIONS
// ============================================================================

/// Decision resulting from configured signature inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityDecision {
    /// No configured blocking signature matched.
    Allow,

    /// A configured precursor or warning was observed.
    Warn,

    /// A configured blocking signature matched.
    Block,
}

impl SecurityDecision {
    /// Returns whether the decision permits continued inspection.
    ///
    /// Warning does not mean that the program has been verified safe.
    pub fn is_allowed(self) -> bool {
        matches!(self, Self::Allow | Self::Warn)
    }

    /// Returns whether the decision blocks the inspected operation.
    pub fn is_blocked(self) -> bool {
        matches!(self, Self::Block)
    }
}

/// Severity associated with an inspection event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
    Informational,
    Warning,
    Critical,
}

// ============================================================================
// 3. ERRORS
// ============================================================================

/// Errors encountered during safety inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SafetyGuardError {
    InvalidSubstrate(String),

    InvalidInstruction(String),

    InvalidSignal(String),

    InvalidRule {
        rule: String,
        reason: String,
    },

    DuplicateRule {
        id: String,
    },

    ThreatDetected {
        substrate: String,
        instruction: String,
        rule: String,
    },

    ContextLimitExceeded,

    AuditSequenceExhausted,

    LockPoisoned,
}

impl fmt::Display for SafetyGuardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSubstrate(value) => {
                write!(f, "invalid substrate identifier: {value}")
            }

            Self::InvalidInstruction(value) => {
                write!(f, "invalid instruction identifier: {value}")
            }

            Self::InvalidSignal(value) => {
                write!(f, "invalid security signal: {value}")
            }

            Self::InvalidRule { rule, reason } => {
                write!(f, "invalid security rule '{rule}': {reason}")
            }

            Self::DuplicateRule { id } => {
                write!(f, "duplicate security rule: {id}")
            }

            Self::ThreatDetected {
                substrate,
                instruction,
                rule,
            } => {
                write!(
                    f,
                    "coordinated adversarial pattern detected: \
                     substrate='{substrate}', \
                     instruction='{instruction}', \
                     rule='{rule}'"
                )
            }

            Self::ContextLimitExceeded => {
                write!(f, "configured security context budget exceeded")
            }

            Self::AuditSequenceExhausted => {
                write!(f, "security audit sequence exhausted")
            }

            Self::LockPoisoned => {
                write!(f, "security context lock is poisoned")
            }
        }
    }
}

impl Error for SafetyGuardError {}

// ============================================================================
// 4. IDENTIFIER VALIDATION
// ============================================================================

/// Validates and canonicalizes an inspection identifier.
///
/// No arbitrary maximum identifier length is imposed.
///
/// ASCII case normalization preserves the compatibility behavior of
/// the original implementation.
fn normalize_identifier(
    value: &str,
) -> Result<String, String> {
    let value = value.trim();

    if value.is_empty() {
        return Err("identifier cannot be empty".to_owned());
    }

    if value.chars().any(char::is_control) {
        return Err(
            "identifier contains a control character".to_owned()
        );
    }

    Ok(value.to_ascii_uppercase())
}

// ============================================================================
// 5. COORDINATED THREAT RULES
// ============================================================================

/// Describes a configured precursor/follow-up signature.
///
/// Rules are data-driven. New substrates and instruction identifiers
/// do not require modifications to the compiler composition root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedThreatRule {
    pub id: String,

    pub precursor_signal: String,

    pub target_substrate: String,

    pub trigger_instruction: String,
}

impl CoordinatedThreatRule {
    /// Constructs and validates a coordinated threat rule.
    pub fn new(
        id: impl Into<String>,
        precursor_signal: impl Into<String>,
        target_substrate: impl Into<String>,
        trigger_instruction: impl Into<String>,
    ) -> Result<Self, SafetyGuardError> {
        let id = normalize_identifier(&id.into())
            .map_err(|reason| SafetyGuardError::InvalidRule {
                rule: "unnamed".to_owned(),
                reason,
            })?;

        let precursor_signal =
            normalize_identifier(&precursor_signal.into())
                .map_err(SafetyGuardError::InvalidSignal)?;

        let target_substrate =
            normalize_identifier(&target_substrate.into())
                .map_err(SafetyGuardError::InvalidSubstrate)?;

        let trigger_instruction =
            normalize_identifier(&trigger_instruction.into())
                .map_err(SafetyGuardError::InvalidInstruction)?;

        Ok(Self {
            id,
            precursor_signal,
            target_substrate,
            trigger_instruction,
        })
    }

    fn matches(
        &self,
        substrate: &str,
        instruction: &str,
        signals: &BTreeSet<String>,
    ) -> bool {
        self.target_substrate == substrate
            && self.trigger_instruction == instruction
            && signals.contains(&self.precursor_signal)
    }
}

// ============================================================================
// 6. DEFAULT COMPATIBILITY SIGNATURES
// ============================================================================

/// Returns the legacy cross-substrate signatures.
///
/// These signatures are retained for compatibility with existing backend
/// security tests and existing consumers.
///
/// They are illustrative configured signatures, not a complete security
/// model and not universal declarations that the named operations are
/// inherently malicious.
///
/// Production deployments should supply their own reviewed rules.
fn default_threat_rules() -> Vec<CoordinatedThreatRule> {
    vec![
        CoordinatedThreatRule {
            id: "cross-substrate.shared-bus-exploit".to_owned(),
            precursor_signal: "NEUROMORPHIC_BUFFER_PREP".to_owned(),
            target_substrate: "QUANTUM".to_owned(),
            trigger_instruction: "EXPLOIT_SHARED_BUS".to_owned(),
        },
        CoordinatedThreatRule {
            id: "cross-substrate.direct-state-leak".to_owned(),
            precursor_signal: "NEUROMORPHIC_CACHE_FLUSH".to_owned(),
            target_substrate: "QUANTUM".to_owned(),
            trigger_instruction: "DIRECT_STATE_LEAK".to_owned(),
        },
    ]
}

// ============================================================================
// 7. AUDIT EVENTS
// ============================================================================

/// Structured safety inspection event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityAuditEvent {
    pub sequence: u64,

    pub severity: SecuritySeverity,

    pub decision: SecurityDecision,

    pub substrate: String,

    pub instruction: Option<String>,

    pub rule_id: Option<String>,

    pub message: String,
}

// ============================================================================
// 8. INSPECTION REPORT
// ============================================================================

/// Result of a successfully completed inspection.
///
/// A report records what this guard inspected. It does not certify
/// semantic correctness, IR validity or runtime safety.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafetyInspectionReport {
    /// Number of instructions inspected successfully.
    pub inspected_instructions: usize,

    /// Number of new precursor signals recorded.
    pub recorded_signals: usize,

    /// Inspection decision.
    pub decision: SecurityDecision,

    /// Substrate associated with this inspection.
    pub substrate: String,
}

// ============================================================================
// 9. SAFETY POLICY
// ============================================================================

/// Configuration for inspection and operational retention.
#[derive(Debug, Clone)]
pub struct SafetyPolicy {
    /// Maximum distinct precursor signals retained simultaneously.
    ///
    /// This is an operational budget, not a language-level maximum.
    pub max_signals: usize,

    /// Maximum retained audit events.
    ///
    /// Older events are evicted when this budget is reached.
    pub max_audit_events: usize,

    /// Whether precursor observations generate warning events.
    pub audit_precursors: bool,

    /// Configured coordinated-threat signatures.
    pub threat_rules: Vec<CoordinatedThreatRule>,
}

impl Default for SafetyPolicy {
    fn default() -> Self {
        Self {
            max_signals: DEFAULT_MAX_SIGNALS,
            max_audit_events: DEFAULT_MAX_AUDIT_EVENTS,
            audit_precursors: true,
            threat_rules: default_threat_rules(),
        }
    }
}

impl SafetyPolicy {
    /// Validates the complete policy.
    pub fn validate(&self) -> Result<(), SafetyGuardError> {
        if self.max_signals == 0 || self.max_audit_events == 0 {
            return Err(SafetyGuardError::ContextLimitExceeded);
        }

        let mut identifiers = BTreeSet::new();

        for rule in &self.threat_rules {
            let id = normalize_identifier(&rule.id)
                .map_err(|reason| SafetyGuardError::InvalidRule {
                    rule: rule.id.clone(),
                    reason,
                })?;

            if !identifiers.insert(id.clone()) {
                return Err(SafetyGuardError::DuplicateRule { id });
            }

            for (field, value) in [
                ("precursor signal", &rule.precursor_signal),
                ("target substrate", &rule.target_substrate),
                ("trigger instruction", &rule.trigger_instruction),
            ] {
                normalize_identifier(value).map_err(|reason| {
                    SafetyGuardError::InvalidRule {
                        rule: rule.id.clone(),
                        reason: format!("{field}: {reason}"),
                    }
                })?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// 10. GLOBAL SECURITY CONTEXT
// ============================================================================

/// Mutable state associated with a security inspection scope.
///
/// A context should be scoped to one logical compilation/security-analysis
/// unit. Do not share precursor state between unrelated programs.
#[derive(Debug, Clone)]
pub struct GlobalSecurityContext {
    observed_precursor_signals: BTreeSet<String>,

    audit_events: Vec<SecurityAuditEvent>,

    sequence: u64,

    max_signals: usize,

    max_audit_events: usize,

    audit_precursors: bool,

    threat_rules: Vec<CoordinatedThreatRule>,
}

impl Default for GlobalSecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalSecurityContext {
    /// Constructs the default compatibility policy without an
    /// unnecessary fallible initialization path.
    pub fn new() -> Self {
        let policy = SafetyPolicy::default();

        Self {
            observed_precursor_signals: BTreeSet::new(),
            audit_events: Vec::new(),
            sequence: 0,
            max_signals: policy.max_signals,
            max_audit_events: policy.max_audit_events,
            audit_precursors: policy.audit_precursors,
            threat_rules: policy.threat_rules,
        }
    }

    /// Constructs a context using an explicitly configured policy.
    pub fn with_policy(
        policy: SafetyPolicy,
    ) -> Result<Self, SafetyGuardError> {
        policy.validate()?;

        Ok(Self {
            observed_precursor_signals: BTreeSet::new(),
            audit_events: Vec::new(),
            sequence: 0,
            max_signals: policy.max_signals,
            max_audit_events: policy.max_audit_events,
            audit_precursors: policy.audit_precursors,
            threat_rules: policy.threat_rules,
        })
    }

    /// Compatibility API.
    ///
    /// Invalid input is ignored for compatibility. Production code should
    /// use try_record_signal and handle its Result.
    pub fn record_signal(&mut self, signal: &str) {
        let _ = self.try_record_signal(signal);
    }

    /// Records a precursor signal.
    ///
    /// Duplicate signals are idempotent.
    pub fn try_record_signal(
        &mut self,
        signal: &str,
    ) -> Result<bool, SafetyGuardError> {
        let normalized = normalize_identifier(signal)
            .map_err(SafetyGuardError::InvalidSignal)?;

        if self.observed_precursor_signals.contains(&normalized) {
            return Ok(false);
        }

        if self.observed_precursor_signals.len() >= self.max_signals {
            return Err(SafetyGuardError::ContextLimitExceeded);
        }

        // Check sequence availability before mutating observable state.
        let needs_audit = self.audit_precursors
            && self.threat_rules.iter().any(|rule| {
                rule.precursor_signal == normalized
            });

        if needs_audit && self.sequence == u64::MAX {
            return Err(SafetyGuardError::AuditSequenceExhausted);
        }

        self.observed_precursor_signals
            .insert(normalized.clone());

        if needs_audit {
            self.push_audit_event(
                SecuritySeverity::Warning,
                SecurityDecision::Warn,
                "GLOBAL".to_owned(),
                None,
                None,
                format!(
                    "configured precursor signal observed: {normalized}"
                ),
            )?;
        }

        Ok(true)
    }

    /// Compatibility API.
    ///
    /// Invalid identifiers conservatively return true.
    /// Production code should use try_check_coordinated_threat.
    pub fn check_coordinated_threat(
        &self,
        current_substrate: &str,
        instruction: &str,
    ) -> bool {
        self.try_check_coordinated_threat(
            current_substrate,
            instruction,
        )
        .unwrap_or(true)
    }

    /// Checks configured signatures using validated identifiers.
    pub fn try_check_coordinated_threat(
        &self,
        current_substrate: &str,
        instruction: &str,
    ) -> Result<bool, SafetyGuardError> {
        let substrate = normalize_identifier(current_substrate)
            .map_err(SafetyGuardError::InvalidSubstrate)?;

        let instruction = normalize_identifier(instruction)
            .map_err(SafetyGuardError::InvalidInstruction)?;

        Ok(self.threat_rules.iter().any(|rule| {
            rule.matches(
                &substrate,
                &instruction,
                &self.observed_precursor_signals,
            )
        }))
    }

    /// Returns deterministically ordered precursor signals.
    pub fn observed_signals(&self) -> Vec<String> {
        self.observed_precursor_signals.iter().cloned().collect()
    }

    /// Returns the number of distinct precursor signals.
    pub fn signal_count(&self) -> usize {
        self.observed_precursor_signals.len()
    }

    /// Returns retained audit events in insertion order.
    pub fn audit_events(&self) -> &[SecurityAuditEvent] {
        &self.audit_events
    }

    /// Clears precursor state but retains audit history.
    pub fn reset_precursors(&mut self) {
        self.observed_precursor_signals.clear();
    }

    /// Clears precursor state and retained audit history.
    pub fn reset(&mut self) {
        self.observed_precursor_signals.clear();
        self.audit_events.clear();
        self.sequence = 0;
    }

    /// Adds an audit event using checked sequence advancement.
    fn push_audit_event(
        &mut self,
        severity: SecuritySeverity,
        decision: SecurityDecision,
        substrate: String,
        instruction: Option<String>,
        rule_id: Option<String>,
        message: String,
    ) -> Result<(), SafetyGuardError> {
        let sequence = self.sequence
            .checked_add(1)
            .ok_or(SafetyGuardError::AuditSequenceExhausted)?;

        self.sequence = sequence;

        // Bounded retention is an explicit operational policy.
        // It does not impose a limit on the number of inspections.
        if self.audit_events.len() >= self.max_audit_events {
            self.audit_events.remove(0);
        }

        self.audit_events.push(SecurityAuditEvent {
            sequence,
            severity,
            decision,
            substrate,
            instruction,
            rule_id,
            message,
        });

        Ok(())
    }
}

// ============================================================================
// 11. SHARED CONTEXT
// ============================================================================

/// Thread-safe shared context handle.
///
/// Prefer independent contexts for independent compilation units.
/// Sharing a context deliberately shares precursor state.
pub type SharedGlobalSecurityContext =
    Arc<RwLock<GlobalSecurityContext>>;

/// Creates a shared default context.
pub fn shared_security_context(
) -> Result<SharedGlobalSecurityContext, SafetyGuardError> {
    Ok(Arc::new(RwLock::new(
        GlobalSecurityContext::new()
    )))
}

// ============================================================================
// 12. SAFETY GUARD
// ============================================================================

/// Instruction-stream inspection boundary for one substrate.
///
/// Substrate identifiers are open-world strings rather than a closed
/// enumeration of classical, quantum, HDL or future computing domains.
#[derive(Debug, Clone)]
pub struct SafetyGuard {
    pub substrate_name: String,
}

impl SafetyGuard {
    /// Compatibility constructor.
    ///
    /// Validation is deferred until inspection because this constructor
    /// cannot return a Result.
    ///
    /// New production callers should prefer try_new.
    pub fn new(substrate_name: &str) -> Self {
        Self {
            substrate_name: substrate_name.to_owned(),
        }
    }

    /// Constructs a guard using a validated substrate identifier.
    pub fn try_new(
        substrate_name: &str,
    ) -> Result<Self, SafetyGuardError> {
        let substrate_name = normalize_identifier(substrate_name)
            .map_err(SafetyGuardError::InvalidSubstrate)?;

        Ok(Self { substrate_name })
    }

    /// Legacy compatibility inspection method.
    pub fn inspect_with_context(
        &self,
        instructions: &[String],
        global_ctx: &mut GlobalSecurityContext,
    ) -> Result<(), String> {
        self.inspect_with_context_checked(
            instructions,
            global_ctx,
        )
        .map_err(|error| error.to_string())
    }

    /// Structured compatibility inspection method.
    pub fn inspect_with_context_checked(
        &self,
        instructions: &[String],
        global_ctx: &mut GlobalSecurityContext,
    ) -> Result<(), SafetyGuardError> {
        self.inspect_with_policy(instructions, global_ctx)
    }

    /// Inspects an instruction stream and returns a structured report.
    ///
    /// Processing is sequential. A precursor encountered earlier in the
    /// stream can therefore affect a subsequent instruction.
    ///
    /// No instruction is executed or transformed.
    pub fn inspect_with_report(
        &self,
        instructions: &[String],
        global_ctx: &mut GlobalSecurityContext,
    ) -> Result<SafetyInspectionReport, SafetyGuardError> {
        let substrate = normalize_identifier(&self.substrate_name)
            .map_err(SafetyGuardError::InvalidSubstrate)?;

        let mut recorded_signals = 0usize;
        let mut inspected = 0usize;

        for raw_instruction in instructions {
            let instruction = normalize_identifier(raw_instruction)
                .map_err(SafetyGuardError::InvalidInstruction)?;

            // Preserve the existing cross-substrate compatibility behavior.
            let precursor = self.precursor_for_instruction(
                &substrate,
                &instruction,
            );

            if let Some(signal) = precursor {
                if global_ctx.try_record_signal(signal)? {
                    recorded_signals = recorded_signals
                        .checked_add(1)
                        .ok_or(SafetyGuardError::ContextLimitExceeded)?;
                }
            }

            inspected = inspected
                .checked_add(1)
                .ok_or(SafetyGuardError::ContextLimitExceeded)?;

            if let Some(rule) = global_ctx.threat_rules.iter().find(
                |rule| {
                    rule.matches(
                        &substrate,
                        &instruction,
                        &global_ctx.observed_precursor_signals,
                    )
                },
            ) {
                global_ctx.push_audit_event(
                    SecuritySeverity::Critical,
                    SecurityDecision::Block,
                    substrate.clone(),
                    Some(instruction.clone()),
                    Some(rule.id.clone()),
                    format!(
                        "configured coordinated threat signature matched: {}",
                        rule.id
                    ),
                )?;

                return Err(SafetyGuardError::ThreatDetected {
                    substrate,
                    instruction,
                    rule: rule.id.clone(),
                });
            }
        }

        Ok(SafetyInspectionReport {
            inspected_instructions: inspected,
            recorded_signals,
            decision: SecurityDecision::Allow,
            substrate,
        })
    }

    /// Compatibility inspection entry point.
    ///
    /// A successful return means only that no configured blocking
    /// signature matched. It does not prove program safety.
    pub fn inspect_with_policy(
        &self,
        instructions: &[String],
        global_ctx: &mut GlobalSecurityContext,
    ) -> Result<(), SafetyGuardError> {
        self.inspect_with_report(
            instructions,
            global_ctx,
        )?;

        Ok(())
    }

    /// Maps the two existing legacy precursor signatures.
    ///
    /// These are compatibility signatures, not universal semantic rules.
    /// New signatures should be introduced through SafetyPolicy.
    fn precursor_for_instruction<'a>(
        &self,
        substrate: &str,
        instruction: &str,
    ) -> Option<&'a str> {
        if substrate == "NEUROMORPHIC" {
            match instruction {
                "PREPARE_SHARED_BUFFER" => {
                    Some("NEUROMORPHIC_BUFFER_PREP")
                }

                "FLUSH_CACHE_LINES" => {
                    Some("NEUROMORPHIC_CACHE_FLUSH")
                }

                _ => None,
            }
        } else {
            None
        }
    }
}

// ============================================================================
// 13. CONVENIENCE API
// ============================================================================

/// Inspects an instruction stream using an existing security context.
pub fn inspect_instructions(
    substrate: &str,
    instructions: &[String],
    context: &mut GlobalSecurityContext,
) -> Result<(), SafetyGuardError> {
    let guard = SafetyGuard::try_new(substrate)?;

    guard.inspect_with_policy(
        instructions,
        context,
    )
}

// ============================================================================
// 14. TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_context_is_empty() {
        let context = GlobalSecurityContext::new();

        assert_eq!(context.signal_count(), 0);
        assert!(context.observed_signals().is_empty());
    }

    #[test]
    fn records_precursor_signal() {
        let mut context = GlobalSecurityContext::new();

        assert_eq!(
            context.try_record_signal(
                "NEUROMORPHIC_BUFFER_PREP"
            ),
            Ok(true)
        );

        assert_eq!(context.signal_count(), 1);
    }

    #[test]
    fn duplicate_signal_is_idempotent() {
        let mut context = GlobalSecurityContext::new();

        assert_eq!(
            context.try_record_signal("FIRST_SIGNAL"),
            Ok(true)
        );

        assert_eq!(
            context.try_record_signal("FIRST_SIGNAL"),
            Ok(false)
        );

        assert_eq!(context.signal_count(), 1);
    }

    #[test]
    fn detects_shared_bus_signature() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_BUFFER_PREP");

        assert!(context.check_coordinated_threat(
            "Quantum",
            "EXPLOIT_SHARED_BUS"
        ));
    }

    #[test]
    fn detects_cache_state_signature() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_CACHE_FLUSH");

        assert!(context.check_coordinated_threat(
            "Quantum",
            "DIRECT_STATE_LEAK"
        ));
    }

    #[test]
    fn unrelated_instruction_is_not_blocked() {
        let context = GlobalSecurityContext::new();

        assert!(!context.check_coordinated_threat(
            "Quantum",
            "NORMAL_OPERATION"
        ));
    }

    #[test]
    fn incorrect_substrate_does_not_match() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_BUFFER_PREP");

        assert!(!context.check_coordinated_threat(
            "CPU",
            "EXPLOIT_SHARED_BUS"
        ));
    }

    #[test]
    fn full_cross_substrate_chain_is_blocked() {
        let mut context = GlobalSecurityContext::new();

        let precursor = SafetyGuard::try_new(
            "NEUROMORPHIC"
        ).expect("valid substrate");

        precursor.inspect_with_policy(
            &[String::from("PREPARE_SHARED_BUFFER")],
            &mut context,
        ).expect("precursor should be recorded");

        let quantum = SafetyGuard::try_new(
            "QUANTUM"
        ).expect("valid substrate");

        let result = quantum.inspect_with_policy(
            &[String::from("EXPLOIT_SHARED_BUS")],
            &mut context,
        );

        assert!(matches!(
            result,
            Err(SafetyGuardError::ThreatDetected { .. })
        ));
    }

    #[test]
    fn cache_flush_chain_is_blocked() {
        let mut context = GlobalSecurityContext::new();

        let precursor = SafetyGuard::try_new(
            "NEUROMORPHIC"
        ).expect("valid substrate");

        precursor.inspect_with_policy(
            &[String::from("FLUSH_CACHE_LINES")],
            &mut context,
        ).expect("precursor should be recorded");

        let quantum = SafetyGuard::try_new(
            "QUANTUM"
        ).expect("valid substrate");

        assert!(quantum.inspect_with_policy(
            &[String::from("DIRECT_STATE_LEAK")],
            &mut context,
        ).is_err());
    }

    #[test]
    fn matching_is_case_insensitive() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("neuromorphic_buffer_prep");

        assert!(context.check_coordinated_threat(
            "quantum",
            "exploit_shared_bus"
        ));
    }

    #[test]
    fn rejects_control_characters() {
        assert!(matches!(
            SafetyGuard::try_new("bad\nsubstrate"),
            Err(SafetyGuardError::InvalidSubstrate(_))
        ));

        let mut context = GlobalSecurityContext::new();

        assert!(matches!(
            context.try_record_signal("bad\0signal"),
            Err(SafetyGuardError::InvalidSignal(_))
        ));

        assert!(matches!(
            context.try_check_coordinated_threat(
                "QUANTUM",
                "bad\ninstruction"
            ),
            Err(SafetyGuardError::InvalidInstruction(_))
        ));
    }

    #[test]
    fn accepts_identifiers_without_arbitrary_length_ceiling() {
        let identifier = "A".repeat(16_384);

        let guard = SafetyGuard::try_new(&identifier);

        assert!(guard.is_ok());
    }

    #[test]
    fn audit_event_is_created_for_blocked_signature() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_BUFFER_PREP");

        let guard = SafetyGuard::try_new("Quantum")
            .expect("valid substrate");

        let _ = guard.inspect_with_policy(
            &[String::from("EXPLOIT_SHARED_BUS")],
            &mut context,
        );

        assert!(context.audit_events().iter().any(|event| {
            event.decision == SecurityDecision::Block
                && event.severity == SecuritySeverity::Critical
        }));
    }

    #[test]
    fn reset_clears_context() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_BUFFER_PREP");

        context.reset();

        assert_eq!(context.signal_count(), 0);
        assert!(context.audit_events().is_empty());
    }

    #[test]
    fn shared_context_can_be_created() {
        let context = shared_security_context()
            .expect("shared context should be created");

        let guard = context.read()
            .expect("context lock should be available");

        assert_eq!(guard.signal_count(), 0);
    }

    #[test]
    fn custom_rules_are_supported() {
        let rule = CoordinatedThreatRule::new(
            "custom.cross-domain-rule",
            "CUSTOM_PRECURSOR",
            "CUSTOM_ACCELERATOR",
            "CUSTOM_UNSAFE_OPERATION",
        ).expect("valid custom rule");

        let policy = SafetyPolicy {
            threat_rules: vec![rule],
            ..SafetyPolicy::default()
        };

        let mut context = GlobalSecurityContext::with_policy(
            policy
        ).expect("valid custom policy");

        context.try_record_signal(
            "CUSTOM_PRECURSOR"
        ).expect("signal recorded");

        let guard = SafetyGuard::try_new(
            "CUSTOM_ACCELERATOR"
        ).expect("valid custom substrate");

        assert!(matches!(
            guard.inspect_with_policy(
                &[String::from("CUSTOM_UNSAFE_OPERATION")],
                &mut context,
            ),
            Err(SafetyGuardError::ThreatDetected { .. })
        ));
    }

    #[test]
    fn duplicate_rule_identifiers_are_rejected() {
        let first = CoordinatedThreatRule::new(
            "duplicate",
            "FIRST",
            "CPU",
            "ACTION_ONE",
        ).expect("valid rule");

        let second = CoordinatedThreatRule::new(
            "DUPLICATE",
            "SECOND",
            "GPU",
            "ACTION_TWO",
        ).expect("valid rule");

        let policy = SafetyPolicy {
            threat_rules: vec![first, second],
            ..SafetyPolicy::default()
        };

        assert!(matches!(
            GlobalSecurityContext::with_policy(policy),
            Err(SafetyGuardError::DuplicateRule { .. })
        ));
    }

    #[test]
    fn configurable_operational_budget_is_enforced() {
        let policy = SafetyPolicy {
            max_signals: 1,
            max_audit_events: 1,
            ..SafetyPolicy::default()
        };

        let mut context = GlobalSecurityContext::with_policy(
            policy
        ).expect("valid operational budget");

        assert_eq!(
            context.try_record_signal("FIRST"),
            Ok(true)
        );

        assert!(matches!(
            context.try_record_signal("SECOND"),
            Err(SafetyGuardError::ContextLimitExceeded)
        ));

        assert_eq!(context.signal_count(), 1);
    }

    #[test]
    fn zero_operational_capacity_is_rejected() {
        let policy = SafetyPolicy {
            max_signals: 0,
            ..SafetyPolicy::default()
        };

        assert!(matches!(
            GlobalSecurityContext::with_policy(policy),
            Err(SafetyGuardError::ContextLimitExceeded)
        ));
    }

    #[test]
    fn precursor_auditing_can_be_disabled() {
        let policy = SafetyPolicy {
            audit_precursors: false,
            ..SafetyPolicy::default()
        };

        let mut context = GlobalSecurityContext::with_policy(
            policy
        ).expect("valid policy");

        context.try_record_signal(
            "NEUROMORPHIC_BUFFER_PREP"
        ).expect("record signal");

        assert!(context.audit_events().is_empty());
    }

    #[test]
    fn inspection_returns_structured_report() {
        let mut context = GlobalSecurityContext::new();

        let guard = SafetyGuard::try_new(
            "CPU"
        ).expect("valid substrate");

        let report = guard.inspect_with_report(
            &[
                String::from("LOAD"),
                String::from("STORE"),
            ],
            &mut context,
        ).expect("inspection should succeed");

        assert_eq!(report.inspected_instructions, 2);
        assert_eq!(report.recorded_signals, 0);
        assert_eq!(report.decision, SecurityDecision::Allow);
        assert_eq!(report.substrate, "CPU");
    }

    #[test]
    fn invalid_instruction_is_rejected() {
        let mut context = GlobalSecurityContext::new();

        let result = inspect_instructions(
            "Quantum",
            &[String::from("\n")],
            &mut context,
        );

        assert!(matches!(
            result,
            Err(SafetyGuardError::InvalidInstruction(_))
        ));
    }

    #[test]
    fn compatibility_constructor_defers_validation() {
        let guard = SafetyGuard::new("bad\nsubstrate");

        let mut context = GlobalSecurityContext::new();

        assert!(matches!(
            guard.inspect_with_policy(&[], &mut context),
            Err(SafetyGuardError::InvalidSubstrate(_))
        ));
    }

    #[test]
    fn compatibility_api_returns_string_error() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_BUFFER_PREP");

        let guard = SafetyGuard::new("Quantum");

        let result = guard.inspect_with_context(
            &[String::from("EXPLOIT_SHARED_BUS")],
            &mut context,
        );

        assert!(result.is_err());

        assert!(
            result.unwrap_err().contains(
                "coordinated adversarial pattern detected"
            )
        );
    }
}
