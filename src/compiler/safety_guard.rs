//! Zamani Compiler — Automated Safety Guard
//!
//! Coordinated cross-substrate security analysis for compiler instruction
//! streams.
//!
//! The safety guard is intentionally:
//!   - deterministic;
//!   - fail-closed for malformed security-sensitive input;
//!   - independent of a particular backend implementation;
//!   - bounded in memory;
//!   - suitable for use from parallel compilation pipelines;
//!   - auditable through structured security events.
//!
//! The guard does not execute instructions. It only analyzes compiler IR-like
//! instruction identifiers and security signals.
//!
//! Supported compatibility API:
//!   - `GlobalSecurityContext`
//!   - `GlobalSecurityContext::new()`
//!   - `GlobalSecurityContext::record_signal()`
//!   - `GlobalSecurityContext::check_coordinated_threat()`
//!   - `SafetyGuard::new()`
//!   - `SafetyGuard::inspect_with_context()`
//!
//! Production callers should prefer `inspect_with_policy()` and
//! `SecurityAuditEvent` for structured diagnostics.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, RwLock};

/// Maximum number of precursor signals retained by the global context.
///
/// A bounded context prevents an untrusted or malformed compilation stream
/// from causing unbounded memory growth.
pub const DEFAULT_MAX_SIGNALS: usize = 4_096;

/// Maximum length accepted for a substrate identifier.
pub const MAX_SUBSTRATE_LENGTH: usize = 256;

/// Maximum length accepted for an instruction identifier.
pub const MAX_INSTRUCTION_LENGTH: usize = 1_024;

/// Maximum length accepted for a security signal identifier.
pub const MAX_SIGNAL_LENGTH: usize = 256;

/// Maximum number of audit events retained by one context.
pub const DEFAULT_MAX_AUDIT_EVENTS: usize = 8_192;

/// Security decision produced by the safety guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityDecision {
    /// Instruction stream is permitted.
    Allow,

    /// Instruction stream contains a suspicious precursor.
    Warn,

    /// Instruction stream matches a coordinated threat rule.
    Block,
}

impl SecurityDecision {
    pub fn is_allowed(self) -> bool {
        matches!(self, Self::Allow | Self::Warn)
    }

    pub fn is_blocked(self) -> bool {
        matches!(self, Self::Block)
    }
}

/// Severity assigned to an audit event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
    Informational,
    Warning,
    Critical,
}

/// Structured security error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyGuardError {
    InvalidSubstrate(String),
    InvalidInstruction(String),
    InvalidSignal(String),
    ThreatDetected {
        substrate: String,
        instruction: String,
        rule: String,
    },
    ContextLimitExceeded,
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

            Self::ThreatDetected {
                substrate,
                instruction,
                rule,
            } => write!(
                f,
                "coordinated adversarial activity detected: \
                 substrate='{substrate}', instruction='{instruction}', rule='{rule}'"
            ),

            Self::ContextLimitExceeded => {
                write!(f, "security context capacity has been exceeded")
            }

            Self::LockPoisoned => {
                write!(f, "security context lock has been poisoned")
            }
        }
    }
}

impl std::error::Error for SafetyGuardError {}

/// Security rule describing a precursor/follow-up instruction chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedThreatRule {
    pub id: String,
    pub precursor_signal: String,
    pub target_substrate: String,
    pub trigger_instruction: String,
}

impl CoordinatedThreatRule {
    pub fn new(
        id: impl Into<String>,
        precursor_signal: impl Into<String>,
        target_substrate: impl Into<String>,
        trigger_instruction: impl Into<String>,
    ) -> Result<Self, SafetyGuardError> {
        let id = normalize_identifier(&id.into(), MAX_SIGNAL_LENGTH)
            .map_err(SafetyGuardError::InvalidSignal)?;

        let precursor_signal =
            normalize_identifier(&precursor_signal.into(), MAX_SIGNAL_LENGTH)
                .map_err(SafetyGuardError::InvalidSignal)?;

        let target_substrate =
            normalize_identifier(&target_substrate.into(), MAX_SUBSTRATE_LENGTH)
                .map_err(SafetyGuardError::InvalidSubstrate)?;

        let trigger_instruction =
            normalize_identifier(&trigger_instruction.into(), MAX_INSTRUCTION_LENGTH)
                .map_err(SafetyGuardError::InvalidInstruction)?;

        Ok(Self {
            id,
            precursor_signal,
            target_substrate,
            trigger_instruction,
        })
    }

    fn matches(&self, substrate: &str, instruction: &str, signals: &HashSet<String>) -> bool {
        self.target_substrate.eq_ignore_ascii_case(substrate)
            && self.trigger_instruction.eq_ignore_ascii_case(instruction)
            && signals.contains(&self.precursor_signal)
    }
}

/// Default coordinated-attack rules.
///
/// These preserve the behavior of the original implementation while moving
/// the rules into explicit, inspectable data.
fn default_threat_rules() -> Vec<CoordinatedThreatRule> {
    vec![
        CoordinatedThreatRule {
            id: "cross-substrate.shared-bus-exploit".to_string(),
            precursor_signal: "NEUROMORPHIC_BUFFER_PREP".to_string(),
            target_substrate: "QUANTUM".to_string(),
            trigger_instruction: "EXPLOIT_SHARED_BUS".to_string(),
        },
        CoordinatedThreatRule {
            id: "cross-substrate.direct-state-leak".to_string(),
            precursor_signal: "NEUROMORPHIC_CACHE_FLUSH".to_string(),
            target_substrate: "QUANTUM".to_string(),
            trigger_instruction: "DIRECT_STATE_LEAK".to_string(),
        },
    ]
}

/// Structured security audit event.
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

/// Runtime security policy.
#[derive(Debug, Clone)]
pub struct SafetyPolicy {
    /// Maximum number of precursor signals retained.
    pub max_signals: usize,

    /// Maximum number of audit events retained.
    pub max_audit_events: usize,

    /// Whether precursor signals should generate warning events.
    pub audit_precursors: bool,

    /// Threat rules evaluated by the guard.
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
    pub fn validate(&self) -> Result<(), SafetyGuardError> {
        if self.max_signals == 0 || self.max_audit_events == 0 {
            return Err(SafetyGuardError::ContextLimitExceeded);
        }

        for rule in &self.threat_rules {
            if rule.id.is_empty()
                || rule.precursor_signal.is_empty()
                || rule.target_substrate.is_empty()
                || rule.trigger_instruction.is_empty()
            {
                return Err(SafetyGuardError::InvalidSignal(
                    "security rule contains an empty field".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// Shared, bounded security context.
///
/// `GlobalSecurityContext` retains the original mutable API while internally
/// maintaining structured state and audit information.
#[derive(Debug, Clone)]
pub struct GlobalSecurityContext {
    observed_precursor_signals: HashSet<String>,
    audit_events: Vec<SecurityAuditEvent>,
    sequence: u64,
    max_signals: usize,
    max_audit_events: usize,
    threat_rules: Vec<CoordinatedThreatRule>,
}

impl Default for GlobalSecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalSecurityContext {
    /// Creates a context using the production default policy.
    pub fn new() -> Self {
        Self::with_policy(SafetyPolicy::default())
            .expect("default SafetyPolicy must always be valid")
    }

    /// Creates a context from an explicit policy.
    pub fn with_policy(policy: SafetyPolicy) -> Result<Self, SafetyGuardError> {
        policy.validate()?;

        Ok(Self {
            observed_precursor_signals: HashSet::new(),
            audit_events: Vec::new(),
            sequence: 0,
            max_signals: policy.max_signals,
            max_audit_events: policy.max_audit_events,
            threat_rules: policy.threat_rules,
        })
    }

    /// Records a precursor signal.
    ///
    /// Invalid signals are ignored for compatibility with the original
    /// infallible API. New callers should use `try_record_signal()`.
    pub fn record_signal(&mut self, signal: &str) {
        let _ = self.try_record_signal(signal);
    }

    /// Fallible production version of `record_signal`.
    pub fn try_record_signal(&mut self, signal: &str) -> Result<bool, SafetyGuardError> {
        let normalized =
            normalize_identifier(signal, MAX_SIGNAL_LENGTH)
                .map_err(SafetyGuardError::InvalidSignal)?;

        if self.observed_precursor_signals.contains(&normalized) {
            return Ok(false);
        }

        if self.observed_precursor_signals.len() >= self.max_signals {
            return Err(SafetyGuardError::ContextLimitExceeded);
        }

        self.observed_precursor_signals.insert(normalized.clone());

        if self
            .threat_rules
            .iter()
            .any(|rule| rule.precursor_signal == normalized)
        {
            self.push_audit_event(
                SecuritySeverity::Warning,
                SecurityDecision::Warn,
                "GLOBAL".to_string(),
                None,
                None,
                format!("precursor security signal recorded: {normalized}"),
            );
        }

        Ok(true)
    }

    /// Checks whether the current substrate/instruction pair completes a
    /// coordinated threat chain.
    ///
    /// This compatibility method remains infallible.
    pub fn check_coordinated_threat(
        &self,
        current_substrate: &str,
        instruction: &str,
    ) -> bool {
        self.try_check_coordinated_threat(current_substrate, instruction)
            .unwrap_or(true)
    }

    /// Fallible production version of coordinated threat detection.
    pub fn try_check_coordinated_threat(
        &self,
        current_substrate: &str,
        instruction: &str,
    ) -> Result<bool, SafetyGuardError> {
        let substrate =
            normalize_identifier(current_substrate, MAX_SUBSTRATE_LENGTH)
                .map_err(SafetyGuardError::InvalidSubstrate)?;

        let instruction =
            normalize_identifier(instruction, MAX_INSTRUCTION_LENGTH)
                .map_err(SafetyGuardError::InvalidInstruction)?;

        Ok(self
            .threat_rules
            .iter()
            .any(|rule| {
                rule.matches(
                    &substrate,
                    &instruction,
                    &self.observed_precursor_signals,
                )
            }))
    }

    /// Returns all currently observed precursor signals.
    pub fn observed_signals(&self) -> Vec<String> {
        let mut signals: Vec<_> =
            self.observed_precursor_signals.iter().cloned().collect();

        signals.sort();
        signals
    }

    /// Returns the number of observed precursor signals.
    pub fn signal_count(&self) -> usize {
        self.observed_precursor_signals.len()
    }

    /// Returns a snapshot of the security audit log.
    pub fn audit_events(&self) -> &[SecurityAuditEvent] {
        &self.audit_events
    }

    /// Clears transient precursor state.
    ///
    /// This is useful between independent compilation units.
    pub fn reset_precursors(&mut self) {
        self.observed_precursor_signals.clear();
    }

    /// Clears both precursor and audit state.
    pub fn reset(&mut self) {
        self.observed_precursor_signals.clear();
        self.audit_events.clear();
        self.sequence = 0;
    }

    fn push_audit_event(
        &mut self,
        severity: SecuritySeverity,
        decision: SecurityDecision,
        substrate: String,
        instruction: Option<String>,
        rule_id: Option<String>,
        message: String,
    ) {
        self.sequence = self.sequence.wrapping_add(1);

        if self.audit_events.len() >= self.max_audit_events {
            self.audit_events.remove(0);
        }

        self.audit_events.push(SecurityAuditEvent {
            sequence: self.sequence,
            severity,
            decision,
            substrate,
            instruction,
            rule_id,
            message,
        });
    }
}

/// Thread-safe shared security context.
///
/// The compiler can clone this handle and share it between independent
/// backend-analysis workers.
pub type SharedGlobalSecurityContext = Arc<RwLock<GlobalSecurityContext>>;

/// Creates a thread-safe global security context.
pub fn shared_security_context() -> Result<SharedGlobalSecurityContext, SafetyGuardError> {
    Ok(Arc::new(RwLock::new(GlobalSecurityContext::new())))
}

/// Automated safety guard for one compiler substrate.
#[derive(Debug, Clone)]
pub struct SafetyGuard {
    pub substrate_name: String,
}

impl SafetyGuard {
    /// Creates a safety guard for a substrate.
    ///
    /// This compatibility constructor cannot return a `Result`, so invalid
    /// names are rejected by normalizing them to an explicit safe identifier.
    /// Production code should prefer `try_new()`.
    pub fn new(substrate_name: &str) -> Self {
        Self::try_new(substrate_name).unwrap_or_else(|_| Self {
            substrate_name: "INVALID_SUBSTRATE".to_string(),
        })
    }

    /// Fallible constructor.
    pub fn try_new(substrate_name: &str) -> Result<Self, SafetyGuardError> {
        let substrate_name =
            normalize_identifier(substrate_name, MAX_SUBSTRATE_LENGTH)
                .map_err(SafetyGuardError::InvalidSubstrate)?;

        Ok(Self { substrate_name })
    }

    /// Inspects instructions using the default production policy.
    ///
    /// This preserves the original API.
    pub fn inspect_with_context(
        &self,
        instructions: &[String],
        global_ctx: &mut GlobalSecurityContext,
    ) -> Result<(), String> {
        self.inspect_with_context_checked(instructions, global_ctx)
            .map_err(|error| error.to_string())
    }

    /// Structured version of `inspect_with_context`.
    pub fn inspect_with_context_checked(
        &self,
        instructions: &[String],
        global_ctx: &mut GlobalSecurityContext,
    ) -> Result<(), SafetyGuardError> {
        self.inspect_with_policy(instructions, global_ctx)
    }

    /// Production inspection entry point.
    ///
    /// The guard processes instructions sequentially so that a precursor
    /// encountered earlier in the compilation stream can affect a later
    /// instruction in the same stream.
    pub fn inspect_with_policy(
        &self,
        instructions: &[String],
        global_ctx: &mut GlobalSecurityContext,
    ) -> Result<(), SafetyGuardError> {
        for raw_instruction in instructions {
            let instruction =
                normalize_identifier(raw_instruction, MAX_INSTRUCTION_LENGTH)
                    .map_err(SafetyGuardError::InvalidInstruction)?;

            self.record_precursor_if_present(
                &instruction,
                global_ctx,
            )?;

            if let Some(rule) = global_ctx.threat_rules.iter().find(|rule| {
                rule.matches(
                    &self.substrate_name,
                    &instruction,
                    &global_ctx.observed_precursor_signals,
                )
            }) {
                global_ctx.push_audit_event(
                    SecuritySeverity::Critical,
                    SecurityDecision::Block,
                    self.substrate_name.clone(),
                    Some(instruction.clone()),
                    Some(rule.id.clone()),
                    format!(
                        "coordinated cross-substrate threat blocked by rule '{}'",
                        rule.id
                    ),
                );

                return Err(SafetyGuardError::ThreatDetected {
                    substrate: self.substrate_name.clone(),
                    instruction,
                    rule: rule.id.clone(),
                });
            }
        }

        Ok(())
    }

    fn record_precursor_if_present(
        &self,
        instruction: &str,
        global_ctx: &mut GlobalSecurityContext,
    ) -> Result<(), SafetyGuardError> {
        match instruction {
            "PREPARE_SHARED_BUFFER"
                if self.substrate_name.eq_ignore_ascii_case("NEUROMORPHIC") =>
            {
                global_ctx.try_record_signal("NEUROMORPHIC_BUFFER_PREP")?;
            }

            "FLUSH_CACHE_LINES"
                if self.substrate_name.eq_ignore_ascii_case("NEUROMORPHIC") =>
            {
                global_ctx.try_record_signal("NEUROMORPHIC_CACHE_FLUSH")?;
            }

            _ => {}
        }

        Ok(())
    }
}

/// Validates and normalizes a security-sensitive identifier.
///
/// Normalization is intentionally conservative:
/// - leading/trailing whitespace is removed;
/// - empty identifiers are rejected;
/// - embedded control characters are rejected;
/// - excessive input is rejected;
/// - identifiers are converted to uppercase for deterministic rule matching.
fn normalize_identifier(
    value: &str,
    max_length: usize,
) -> Result<String, String> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err("identifier cannot be empty".to_string());
    }

    if trimmed.len() > max_length {
        return Err(format!(
            "identifier exceeds maximum length of {max_length} bytes"
        ));
    }

    if trimmed.chars().any(|character| character.is_control()) {
        return Err("identifier contains a control character".to_string());
    }

    Ok(trimmed.to_ascii_uppercase())
}

/// Convenience helper for checking an instruction stream without manually
/// constructing a guard.
pub fn inspect_instructions(
    substrate: &str,
    instructions: &[String],
    context: &mut GlobalSecurityContext,
) -> Result<(), SafetyGuardError> {
    let guard = SafetyGuard::try_new(substrate)?;
    guard.inspect_with_context_checked(instructions, context)
}

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

        assert!(context.try_record_signal("NEUROMORPHIC_BUFFER_PREP").is_ok());
        assert_eq!(context.signal_count(), 1);
    }

    #[test]
    fn duplicate_signal_is_idempotent() {
        let mut context = GlobalSecurityContext::new();

        assert_eq!(
            context.try_record_signal("NEUROMORPHIC_BUFFER_PREP"),
            Ok(true)
        );

        assert_eq!(
            context.try_record_signal("NEUROMORPHIC_BUFFER_PREP"),
            Ok(false)
        );

        assert_eq!(context.signal_count(), 1);
    }

    #[test]
    fn detects_shared_bus_attack_chain() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_BUFFER_PREP");

        assert!(context.check_coordinated_threat(
            "Quantum",
            "EXPLOIT_SHARED_BUS"
        ));
    }

    #[test]
    fn detects_cache_state_leak_attack_chain() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_CACHE_FLUSH");

        assert!(context.check_coordinated_threat(
            "Quantum",
            "DIRECT_STATE_LEAK"
        ));
    }

    #[test]
    fn unrelated_instruction_is_allowed() {
        let context = GlobalSecurityContext::new();

        assert!(!context.check_coordinated_threat(
            "Quantum",
            "NORMAL_OPERATION"
        ));
    }

    #[test]
    fn wrong_substrate_does_not_trigger_quantum_rule() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_BUFFER_PREP");

        assert!(!context.check_coordinated_threat(
            "CPU",
            "EXPLOIT_SHARED_BUS"
        ));
    }

    #[test]
    fn full_chain_is_blocked() {
        let mut context = GlobalSecurityContext::new();
        let guard = SafetyGuard::try_new("neuromorphic").unwrap();

        let precursor = vec!["PREPARE_SHARED_BUFFER".to_string()];

        guard
            .inspect_with_context_checked(&precursor, &mut context)
            .unwrap();

        let quantum_guard = SafetyGuard::try_new("quantum").unwrap();

        let attack = vec!["EXPLOIT_SHARED_BUS".to_string()];

        let result =
            quantum_guard.inspect_with_context_checked(&attack, &mut context);

        assert!(matches!(
            result,
            Err(SafetyGuardError::ThreatDetected { .. })
        ));
    }

    #[test]
    fn cache_flush_chain_is_blocked() {
        let mut context = GlobalSecurityContext::new();

        let neuromorphic = SafetyGuard::try_new("NEUROMORPHIC").unwrap();

        neuromorphic
            .inspect_with_context_checked(
                &["FLUSH_CACHE_LINES".to_string()],
                &mut context,
            )
            .unwrap();

        let quantum = SafetyGuard::try_new("QUANTUM").unwrap();

        let result = quantum.inspect_with_context_checked(
            &["DIRECT_STATE_LEAK".to_string()],
            &mut context,
        );

        assert!(result.is_err());
    }

    #[test]
    fn instruction_matching_is_case_insensitive() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("neuromorphic_buffer_prep");

        assert!(context.check_coordinated_threat(
            "quantum",
            "exploit_shared_bus"
        ));
    }

    #[test]
    fn invalid_instruction_is_rejected() {
        let mut context = GlobalSecurityContext::new();

        let result = inspect_instructions(
            "Quantum",
            &["\n".to_string()],
            &mut context,
        );

        assert!(matches!(
            result,
            Err(SafetyGuardError::InvalidInstruction(_))
        ));
    }

    #[test]
    fn control_characters_are_rejected() {
        let result = SafetyGuard::try_new("Quantum\n");

        assert!(result.is_err());
    }

    #[test]
    fn audit_event_is_created_for_threat() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_BUFFER_PREP");

        let guard = SafetyGuard::try_new("Quantum").unwrap();

        let _ = guard.inspect_with_context_checked(
            &["EXPLOIT_SHARED_BUS".to_string()],
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
        assert_eq!(context.signal_count(), 1);

        context.reset();

        assert_eq!(context.signal_count(), 0);
        assert!(context.audit_events().is_empty());
    }

    #[test]
    fn shared_context_can_be_created() {
        let context = shared_security_context().unwrap();

        let guard = context.read().unwrap();

        assert_eq!(guard.signal_count(), 0);
    }

    #[test]
    fn custom_rule_can_be_installed() {
        let rule = CoordinatedThreatRule::new(
            "custom-rule",
            "CPU_PRECURSOR",
            "QUANTUM",
            "CUSTOM_QUANTUM_ACTION",
        )
        .unwrap();

        let policy = SafetyPolicy {
            threat_rules: vec![rule],
            ..SafetyPolicy::default()
        };

        let mut context =
            GlobalSecurityContext::with_policy(policy).unwrap();

        context.record_signal("CPU_PRECURSOR");

        assert!(context.check_coordinated_threat(
            "QUANTUM",
            "CUSTOM_QUANTUM_ACTION"
        ));
    }

    #[test]
    fn compatibility_api_returns_string_error() {
        let mut context = GlobalSecurityContext::new();

        context.record_signal("NEUROMORPHIC_BUFFER_PREP");

        let guard = SafetyGuard::new("Quantum");

        let result = guard.inspect_with_context(
            &["EXPLOIT_SHARED_BUS".to_string()],
            &mut context,
        );

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("coordinated adversarial activity detected")
        );
    }
}