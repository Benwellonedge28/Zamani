//! Zamani Quantum Resilience — Diagnosis Classifier.
//!
//! Path:
//!     src/quantum/resilience/diagnosis/classifier.rs
//!
//! =============================================================================
//! Purpose
//! =============================================================================
//!
//! This module converts normalized detection classifications into
//! provider-neutral diagnosis findings.
//!
//! It answers:
//!
//!     "What semantic class does this detection signal belong to?"
//!
//! It does NOT answer:
//!
//! - what the physical root cause is;
//! - which qubit is causally responsible;
//! - whether recovery is permitted;
//! - which recovery action should execute;
//! - which backend should be selected;
//! - how routing should change;
//! - how scheduling should change;
//! - how QEC should change;
//! - whether a result is semantically correct.
//!
//! Those responsibilities belong to:
//!
//!     diagnosis/root_cause.rs
//!     diagnosis/localization.rs
//!     diagnosis/confidence.rs
//!     policy/
//!     planning/
//!     adaptation/
//!     recovery/
//!     verification/
//!     quantum::zqn
//!     quantum::hardware
//!     quantum::routing
//!     quantum::scheduling
//!     quantum::qec
//!
//! =============================================================================
//! Architectural position
//! =============================================================================
//!
//!     DetectionOutput
//!           |
//!           v
//!     DetectionSignal
//!           |
//!           v
//!     +---------------------+
//!     | DiagnosisClassifier |
//!     +---------------------+
//!           |
//!           v
//!     DiagnosisFinding
//!           |
//!     +-----+------+----------------+
//!     |            |                |
//!     v            v                v
//! correlation   localization    root-cause
//!     |            |                |
//!     +------------+----------------+
//!                       |
//!                       v
//!                   Diagnosis
//!                       |
//!                       v
//!                    policy
//!                       |
//!                       v
//!                   planning
//!
//! =============================================================================
//! Design principles
//! =============================================================================
//!
//! 1. Write once, scale everywhere.
//!
//! No qubit count, detector count, signal count, backend count, or machine
//! size is encoded in this file.
//!
//! The implementation works with dynamically sized collections supplied by the
//! caller. Concrete execution remains bounded only by the resources and
//! policies supplied by the surrounding system.
//!
//! 2. Classification is not causality.
//!
//! A `HardwareSignal` becomes a hardware diagnosis category, but this module
//! does not claim that the hardware is the root cause.
//!
//! A `QecSignal` becomes a QEC category, but this module does not claim that
//! the decoder, code, ancilla, or physical qubit is defective.
//!
//! 3. Detection confidence is preserved conservatively.
//!
//! This classifier does not manufacture confidence.
//!
//! By default, the detection confidence is propagated unchanged. Optional
//! explicit confidence scaling may be configured by the caller.
//!
//! 4. No hidden thresholds.
//!
//! There is no:
//!
//!     confidence > 0.95
//!
//! or:
//!
//!     confidence < 0.50
//!
//! or any equivalent machine-specific threshold.
//!
//! Confidence thresholds belong to policy and downstream confidence analysis.
//!
//! 5. Determinism.
//!
//! Signals are processed in deterministic order supplied by
//! `DiagnosisRequest`, and duplicate signal identities can be eliminated
//! deterministically.
//!
//! 6. No hidden I/O.
//!
//! This module does not access:
//!
//! - clocks;
//! - filesystem;
//! - network;
//! - environment variables;
//! - hardware;
//! - random generators;
//! - global mutable state.
//!
//! 7. Canonical quantum identity.
//!
//! This module does not define or manipulate:
//!
//!     QubitId
//!     PhysicalQubitId
//!
//! If future classification logic needs resource identity, the canonical
//! types MUST come from:
//!
//!     crate::quantum::ir::qubit
//!
//! Resource localization belongs primarily to `localization.rs`.
//!
//! 8. Canonical fault semantics.
//!
//! ZQN remains authoritative for physical/noise/fault semantics.
//!
//! This module classifies normalized detection signals and does not create a
//! second quantum fault ontology.
//!
//! 9. No recovery.
//!
//! A classifier may say:
//!
//!     "timeout"
//!
//! but it must never say:
//!
//!     "retry now"
//!
//! That decision belongs to policy/planning.
//!
//! 10. Extensibility.
//!
//! New classification rules can be installed through configuration without
//! changing the classifier implementation.
//!
//! =============================================================================
//! Integration contract
//! =============================================================================
//!
//! This file depends only on stable contracts from:
//!
//!     diagnosis/diagnostician.rs
//!     detection/detector.rs
//!     errors/
//!
//! It does not depend on:
//!
//!     correlation.rs
//!     localization.rs
//!     root_cause.rs
//!     confidence.rs
//!     policy/
//!     planning/
//!     recovery/
//!
//! Those modules consume the findings produced here.
//!
//! Dependency direction:
//!
//!     detection::detector
//!             |
//!             v
//!     diagnosis/classifier
//!             |
//!             v
//!     diagnosis/diagnostician
//!             |
//!             +----------+-----------+
//!             |          |           |
//!             v          v           v
//!        correlation localization root_cause
//!             |
//!             v
//!           policy
//!
//! =============================================================================
//! Production requirements
//! =============================================================================
//!
//! This implementation provides:
//!
//! - explicit classification rules;
//! - stable defaults;
//! - deterministic behavior;
//! - duplicate signal protection;
//! - confidence validation;
//! - no fixed resource limits;
//! - no provider-specific branches;
//! - no hardware assumptions;
//! - no recovery side effects;
//! - no unsafe Rust;
//! - unit tests for classification invariants.
//!
//! =============================================================================
//! Rust compatibility
//! =============================================================================
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! Rust 1.97.1 is an appropriate target and includes an LLVM-related
//! miscompilation fix over 1.97.0. 4
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::resilience::detection::detector::DetectionClassification;
use crate::quantum::resilience::diagnosis::diagnostician::{
    ContributorIdentity,
    DiagnosisCategory,
    DiagnosisConfidence,
    DiagnosisContributor,
    DiagnosisFinding,
    DiagnosisRequest,
    EvidenceReference,
};
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for the classifier contract.
pub const CLASSIFIER_SCHEMA_ID: &str =
    "zamani.quantum.resilience.diagnosis.classifier";

/// Semantic version of this classifier contract.
pub const CLASSIFIER_SCHEMA_VERSION: u16 = 1;

/// Stable identity used by the built-in classifier.
pub const DEFAULT_CLASSIFIER_NAME: &str =
    "zamani.diagnosis.classifier";

/// Stable default classifier implementation version.
pub const DEFAULT_CLASSIFIER_VERSION: &str = "1";

// =============================================================================
// Confidence policy
// =============================================================================

/// Controls how detector confidence is transferred to a diagnosis finding.
///
/// Classification must not silently invent statistical confidence. The
/// default therefore preserves detector confidence exactly.
///
/// A caller may explicitly configure conservative scaling when integrating
/// evidence from a detector whose confidence requires a known domain-specific
/// adjustment.
///
/// No probability multiplication or independence assumption is performed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfidencePolicy {
    /// Preserve the detector confidence exactly.
    Propagate,

    /// Multiply detector confidence by an explicit factor in `[0, 1]`.
    ///
    /// This is a deterministic confidence transformation, not a probability
    /// model.
    Scale(f64),
}

impl ConfidencePolicy {
    /// Creates a validated scaling policy.
    pub fn scale(factor: f64) -> ResilienceResult<Self> {
        if !factor.is_finite() || !(0.0..=1.0).contains(&factor) {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        Ok(Self::Scale(factor))
    }

    /// Applies the configured transformation.
    pub fn apply(
        self,
        confidence: DiagnosisConfidence,
    ) -> ResilienceResult<DiagnosisConfidence> {
        match self {
            Self::Propagate => Ok(confidence),

            Self::Scale(factor) => {
                DiagnosisConfidence::new(confidence.value() * factor)
            }
        }
    }
}

// =============================================================================
// Classification rule
// =============================================================================

/// One explicit mapping from detection classification to diagnosis category.
///
/// Rules contain semantic classification only. They do not contain recovery
/// actions, retry counts, backend names, qubit numbers, or hardware limits.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassificationRule {
    detection: DetectionClassification,
    diagnosis: DiagnosisCategory,
    confidence: ConfidencePolicy,
    explanation: Option<String>,
}

impl ClassificationRule {
    /// Creates a classification rule.
    pub fn new(
        detection: DetectionClassification,
        diagnosis: DiagnosisCategory,
        confidence: ConfidencePolicy,
        explanation: Option<String>,
    ) -> ResilienceResult<Self> {
        let explanation = explanation.and_then(|value| {
            let trimmed = value.trim();

            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_owned())
            }
        });

        // Validate scaling at configuration time rather than during the
        // first production diagnosis.
        if let ConfidencePolicy::Scale(factor) = confidence {
            if !factor.is_finite() || !(0.0..=1.0).contains(&factor) {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                ));
            }
        }

        Ok(Self {
            detection,
            diagnosis,
            confidence,
            explanation,
        })
    }

    /// Returns the detection classification matched by this rule.
    #[must_use]
    pub const fn detection(&self) -> DetectionClassification {
        self.detection
    }

    /// Returns the resulting diagnosis category.
    #[must_use]
    pub const fn diagnosis(&self) -> &DiagnosisCategory {
        &self.diagnosis
    }

    /// Returns the confidence transformation.
    #[must_use]
    pub const fn confidence_policy(&self) -> ConfidencePolicy {
        self.confidence
    }

    /// Returns the optional explanation.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }
}

// =============================================================================
// Classifier configuration
// =============================================================================

/// Configuration for [`DiagnosisClassifier`].
///
/// All behavior that may legitimately vary between deployments is explicit.
///
/// There are deliberately no fields for:
///
/// - maximum qubits;
/// - maximum signals;
/// - maximum detectors;
/// - retry count;
/// - fidelity threshold;
/// - provider;
/// - backend name.
///
/// Such values belong to other subsystem contracts.
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosisClassifierConfig {
    rules: BTreeMap<DetectionClassification, ClassificationRule>,
    fallback_category: DiagnosisCategory,
    include_no_condition: bool,
    deduplicate_signals: bool,
}

impl Default for DiagnosisClassifierConfig {
    fn default() -> Self {
        Self::standard()
    }
}

impl DiagnosisClassifierConfig {
    /// Creates an empty configuration.
    ///
    /// An empty configuration is valid because callers may intentionally
    /// provide only selected classification mappings.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            rules: BTreeMap::new(),
            fallback_category: DiagnosisCategory::Unknown,
            include_no_condition: false,
            deduplicate_signals: true,
        }
    }

    /// Creates Zamani's provider-neutral standard classification mapping.
    ///
    /// These are semantic mappings between existing detection categories and
    /// existing diagnosis categories. They are not hardware assumptions.
    #[must_use]
    pub fn standard() -> Self {
        let mut config = Self::empty();

        config
            .insert_standard_rule(
                DetectionClassification::NoCondition,
                DiagnosisCategory::NoCondition,
            );

        config
            .insert_standard_rule(
                DetectionClassification::Anomaly,
                DiagnosisCategory::Anomaly,
            );

        config
            .insert_standard_rule(
                DetectionClassification::Fault,
                DiagnosisCategory::Fault,
            );

        config
            .insert_standard_rule(
                DetectionClassification::Degradation,
                DiagnosisCategory::Resource,
            );

        config
            .insert_standard_rule(
                DetectionClassification::Unavailability,
                DiagnosisCategory::Resource,
            );

        config
            .insert_standard_rule(
                DetectionClassification::Timeout,
                DiagnosisCategory::Timeout,
            );

        config
            .insert_standard_rule(
                DetectionClassification::ExecutionFailure,
                DiagnosisCategory::ExecutionFailure,
            );

        config
            .insert_standard_rule(
                DetectionClassification::QecSignal,
                DiagnosisCategory::Qec,
            );

        config
            .insert_standard_rule(
                DetectionClassification::HardwareSignal,
                DiagnosisCategory::Hardware,
            );

        config
            .insert_standard_rule(
                DetectionClassification::Inconclusive,
                DiagnosisCategory::Unknown,
            );

        config
    }

    fn insert_standard_rule(
        &mut self,
        detection: DetectionClassification,
        diagnosis: DiagnosisCategory,
    ) {
        let rule = ClassificationRule {
            detection,
            diagnosis,
            confidence: ConfidencePolicy::Propagate,
            explanation: None,
        };

        self.rules.insert(detection, rule);
    }

    /// Installs or replaces a rule.
    ///
    /// Replacement is deterministic because each detection classification has
    /// at most one active rule.
    pub fn insert_rule(
        &mut self,
        rule: ClassificationRule,
    ) -> Option<ClassificationRule> {
        self.rules.insert(rule.detection(), rule)
    }

    /// Removes a classification rule.
    pub fn remove_rule(
        &mut self,
        detection: DetectionClassification,
    ) -> Option<ClassificationRule> {
        self.rules.remove(&detection)
    }

    /// Returns the rule for a detection classification.
    #[must_use]
    pub fn rule(
        &self,
        detection: DetectionClassification,
    ) -> Option<&ClassificationRule> {
        self.rules.get(&detection)
    }

    /// Returns all configured rules in deterministic order.
    #[must_use]
    pub fn rules(
        &self,
    ) -> impl Iterator<Item = &ClassificationRule> {
        self.rules.values()
    }

    /// Returns the configured fallback category.
    #[must_use]
    pub const fn fallback_category(&self) -> &DiagnosisCategory {
        &self.fallback_category
    }

    /// Sets the fallback category.
    ///
    /// The fallback is used only when no rule exists for a classification.
    pub fn set_fallback_category(
        &mut self,
        category: DiagnosisCategory,
    ) {
        self.fallback_category = category;
    }

    /// Returns whether `NoCondition` signals are emitted as findings.
    #[must_use]
    pub const fn include_no_condition(&self) -> bool {
        self.include_no_condition
    }

    /// Controls whether `NoCondition` is emitted.
    pub const fn set_include_no_condition(
        &mut self,
        include: bool,
    ) {
        self.include_no_condition = include;
    }

    /// Returns whether duplicate signal IDs are ignored.
    #[must_use]
    pub const fn deduplicate_signals(&self) -> bool {
        self.deduplicate_signals
    }

    /// Controls duplicate signal handling.
    pub const fn set_deduplicate_signals(
        &mut self,
        deduplicate: bool,
    ) {
        self.deduplicate_signals = deduplicate;
    }

    /// Validates the configuration.
    ///
    /// Validation is intentionally structural. Operational policy such as
    /// minimum confidence belongs elsewhere.
    pub fn validate(&self) -> ResilienceResult<()> {
        for rule in self.rules.values() {
            if let ConfidencePolicy::Scale(factor) =
                rule.confidence_policy()
            {
                if !factor.is_finite()
                    || !(0.0..=1.0).contains(&factor)
                {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::InvalidConfiguration,
                    ));
                }
            }

            if let DiagnosisCategory::External(name) =
                rule.diagnosis()
            {
                if name.trim().is_empty() {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::InvalidConfiguration,
                    ));
                }
            }
        }

        if let DiagnosisCategory::External(name) =
            &self.fallback_category
        {
            if name.trim().is_empty() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Diagnosis classifier
// =============================================================================

/// Provider-neutral detection-to-diagnosis classifier.
///
/// This is the concrete implementation intended to be registered with
/// `Diagnostician`.
///
/// It is deliberately stateless apart from its explicit configuration.
///
/// It does not:
///
/// - inspect hardware;
/// - inspect topology;
/// - manipulate qubits;
/// - change the circuit;
/// - select a backend;
/// - execute recovery;
/// - modify QEC;
/// - perform causal inference.
///
/// Those concerns belong to downstream diagnosis and resilience layers.
#[derive(Debug, Clone)]
pub struct DiagnosisClassifier {
    identity: ContributorIdentity,
    config: DiagnosisClassifierConfig,
}

impl DiagnosisClassifier {
    /// Creates the standard Zamani classifier.
    pub fn new(
        version: impl Into<String>,
    ) -> ResilienceResult<Self> {
        Self::with_config(
            version,
            DiagnosisClassifierConfig::standard(),
        )
    }

    /// Creates a classifier with explicit configuration.
    pub fn with_config(
        version: impl Into<String>,
        config: DiagnosisClassifierConfig,
    ) -> ResilienceResult<Self> {
        config.validate()?;

        Ok(Self {
            identity: ContributorIdentity::new(
                DEFAULT_CLASSIFIER_NAME,
                version,
            )?,
            config,
        })
    }

    /// Returns immutable classifier configuration.
    #[must_use]
    pub const fn config(&self) -> &DiagnosisClassifierConfig {
        &self.config
    }

    /// Returns the classifier identity.
    #[must_use]
    pub const fn identity_ref(&self) -> &ContributorIdentity {
        &self.identity
    }

    /// Classifies one detection classification using the current rules.
    ///
    /// No request or hardware context is needed for the semantic mapping.
    pub fn classify(
        &self,
        classification: DetectionClassification,
        confidence: DiagnosisConfidence,
    ) -> ResilienceResult<(
        DiagnosisCategory,
        DiagnosisConfidence,
        Option<String>,
    )> {
        if matches!(
            classification,
            DetectionClassification::NoCondition
        ) && !self.config.include_no_condition()
        {
            return Ok((
                DiagnosisCategory::NoCondition,
                DiagnosisConfidence::zero(),
                None,
            ));
        }

        if let Some(rule) = self.config.rule(classification) {
            let confidence =
                rule.confidence_policy().apply(confidence)?;

            return Ok((
                rule.diagnosis().clone(),
                confidence,
                rule.explanation().map(ToOwned::to_owned),
            ));
        }

        Ok((
            self.config.fallback_category().clone(),
            confidence,
            None,
        ))
    }

    /// Produces a finding for one normalized detection signal.
    ///
    /// This helper intentionally retains the signal as the only evidence.
    /// Correlation and evidence aggregation belong to later diagnosis stages.
    pub fn classify_signal(
        &self,
        signal: &crate::quantum::resilience::detection::detector::DetectionSignal,
    ) -> ResilienceResult<Option<DiagnosisFinding>> {
        if signal.is_no_condition()
            && !self.config.include_no_condition()
        {
            return Ok(None);
        }

        let confidence =
            DiagnosisConfidence::new(signal.confidence().value())?;

        let (category, confidence, explanation) =
            self.classify(signal.classification(), confidence)?;

        if !category.is_actionable()
            && !self.config.include_no_condition()
        {
            return Ok(None);
        }

        let evidence = EvidenceReference::from_signal(signal)?;

        Ok(Some(DiagnosisFinding::new(
            category,
            confidence,
            [evidence],
            self.identity.clone(),
            explanation,
        )))
    }

    /// Classifies a request without modifying contributor state.
    ///
    /// This method is useful for callers that want direct access to the
    /// classifier while still preserving the same behavior as the
    /// `DiagnosisContributor` implementation.
    pub fn classify_request(
        &self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Vec<DiagnosisFinding>> {
        let mut findings = Vec::new();
        let mut seen = BTreeSet::new();

        for output in request.outputs() {
            if output.metadata().sequence() != request.sequence() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::DetectionInconsistent,
                ));
            }

            for signal in output.signals() {
                if signal.sequence() != request.sequence() {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::DetectionInconsistent,
                    ));
                }

                if self.config.deduplicate_signals()
                    && !seen.insert(signal.id())
                {
                    continue;
                }

                if let Some(finding) =
                    self.classify_signal(signal)?
                {
                    findings.push(finding);
                }
            }
        }

        Ok(findings)
    }
}

impl DiagnosisContributor for DiagnosisClassifier {
    fn identity(&self) -> &ContributorIdentity {
        &self.identity
    }

    fn diagnose(
        &mut self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Vec<DiagnosisFinding>> {
        self.classify_request(request)
    }

    fn is_available(&self) -> bool {
        true
    }

    fn reset(&mut self) {
        // The classifier intentionally has no hidden mutable state.
        //
        // Configuration remains unchanged. This makes reset deterministic and
        // safe to call from the diagnostician lifecycle.
    }
}

// =============================================================================
// Standard constructor
// =============================================================================

/// Creates the standard Zamani classifier.
///
/// This convenience function is useful for registries and dependency
/// injection.
pub fn standard_classifier() -> ResilienceResult<DiagnosisClassifier> {
    DiagnosisClassifier::new(DEFAULT_CLASSIFIER_VERSION)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::num::NonZeroU64;

    use crate::quantum::resilience::detection::detector::{
        DetectionConfidence,
        DetectionMetadata,
        DetectionOutput,
        DetectionSequence,
        DetectionSignal,
        DetectorIdentity,
        SignalId,
    };

    fn sequence(value: u64) -> DetectionSequence {
        DetectionSequence::new(
            NonZeroU64::new(value)
                .expect("test sequence must be non-zero"),
        )
    }

    fn signal_id(value: u64) -> SignalId {
        SignalId::new(
            NonZeroU64::new(value)
                .expect("test signal ID must be non-zero"),
        )
    }

    fn signal(
        id: u64,
        classification: DetectionClassification,
        confidence: f64,
    ) -> DetectionSignal {
        DetectionSignal::new(
            signal_id(id),
            DetectorIdentity::new(
                "test.detector",
                "1",
            )
            .expect("test detector identity must be valid"),
            classification,
            DetectionConfidence::new(confidence)
                .expect("test confidence must be valid"),
            None,
            sequence(1),
        )
    }

    fn request(
        signals: Vec<DetectionSignal>,
    ) -> DiagnosisRequest {
        let detector = DetectorIdentity::new(
            "test.detector",
            "1",
        )
        .expect("test detector identity must be valid");

        let metadata =
            DetectionMetadata::new(detector, sequence(1), signals.len() as u64);

        DiagnosisRequest::new(
            crate::quantum::resilience::diagnosis::diagnostician::DiagnosisId::from_u64(
                1,
            )
            .expect("test diagnosis ID must be valid"),
            sequence(1),
            None,
            [DetectionOutput::new(metadata, signals)],
            false,
        )
    }

    #[test]
    fn standard_mapping_classifies_fault() {
        let classifier =
            DiagnosisClassifier::new("1").expect("valid classifier");

        let result = classifier
            .classify(
                DetectionClassification::Fault,
                DiagnosisConfidence::new(0.8)
                    .expect("valid confidence"),
            )
            .expect("classification must succeed");

        assert_eq!(result.0, DiagnosisCategory::Fault);
        assert_eq!(result.1.value(), 0.8);
    }

    #[test]
    fn standard_mapping_classifies_degradation_as_resource() {
        let classifier =
            DiagnosisClassifier::new("1").expect("valid classifier");

        let result = classifier
            .classify(
                DetectionClassification::Degradation,
                DiagnosisConfidence::new(0.7)
                    .expect("valid confidence"),
            )
            .expect("classification must succeed");

        assert_eq!(result.0, DiagnosisCategory::Resource);
    }

    #[test]
    fn standard_mapping_classifies_qec_signal_as_qec() {
        let classifier =
            DiagnosisClassifier::new("1").expect("valid classifier");

        let result = classifier
            .classify(
                DetectionClassification::QecSignal,
                DiagnosisConfidence::new(0.9)
                    .expect("valid confidence"),
            )
            .expect("classification must succeed");

        assert_eq!(result.0, DiagnosisCategory::Qec);
    }

    #[test]
    fn standard_mapping_classifies_hardware_signal_as_hardware() {
        let classifier =
            DiagnosisClassifier::new("1").expect("valid classifier");

        let result = classifier
            .classify(
                DetectionClassification::HardwareSignal,
                DiagnosisConfidence::new(0.9)
                    .expect("valid confidence"),
            )
            .expect("classification must succeed");

        assert_eq!(result.0, DiagnosisCategory::Hardware);
    }

    #[test]
    fn no_condition_is_not_emitted_by_default() {
        let classifier =
            DiagnosisClassifier::new("1").expect("valid classifier");

        let result = classifier
            .classify_signal(
                &signal(
                    1,
                    DetectionClassification::NoCondition,
                    1.0,
                ),
            )
            .expect("classification must succeed");

        assert!(result.is_none());
    }

    #[test]
    fn no_condition_can_be_explicitly_emitted() {
        let mut config =
            DiagnosisClassifierConfig::standard();

        config.set_include_no_condition(true);

        let classifier =
            DiagnosisClassifier::with_config("1", config)
                .expect("valid classifier");

        let result = classifier
            .classify_signal(
                &signal(
                    1,
                    DetectionClassification::NoCondition,
                    1.0,
                ),
            )
            .expect("classification must succeed");

        assert!(result.is_some());

        let finding =
            result.expect("finding should exist");

        assert_eq!(
            finding.category(),
            &DiagnosisCategory::NoCondition
        );
    }

    #[test]
    fn confidence_is_propagated_by_default() {
        let classifier =
            DiagnosisClassifier::new("1").expect("valid classifier");

        let result = classifier
            .classify(
                DetectionClassification::Anomaly,
                DiagnosisConfidence::new(0.731)
                    .expect("valid confidence"),
            )
            .expect("classification must succeed");

        assert_eq!(result.1.value(), 0.731);
    }

    #[test]
    fn confidence_scaling_is_explicit() {
        let mut config =
            DiagnosisClassifierConfig::standard();

        let rule =
            ClassificationRule::new(
                DetectionClassification::Anomaly,
                DiagnosisCategory::Hardware,
                ConfidencePolicy::scale(0.5)
                    .expect("valid scale"),
                Some("explicit test transformation".to_owned()),
            )
            .expect("valid rule");

        config.insert_rule(rule);

        let classifier =
            DiagnosisClassifier::with_config("1", config)
                .expect("valid classifier");

        let result = classifier
            .classify(
                DetectionClassification::Anomaly,
                DiagnosisConfidence::new(0.8)
                    .expect("valid confidence"),
            )
            .expect("classification must succeed");

        assert_eq!(result.0, DiagnosisCategory::Hardware);
        assert_eq!(result.1.value(), 0.4);
        assert_eq!(
            result.2.as_deref(),
            Some("explicit test transformation")
        );
    }

    #[test]
    fn invalid_confidence_scale_is_rejected() {
        assert!(
            ConfidencePolicy::scale(1.1).is_err()
        );

        assert!(
            ConfidencePolicy::scale(-0.1).is_err()
        );

        assert!(
            ConfidencePolicy::scale(f64::NAN).is_err()
        );
    }

    #[test]
    fn duplicate_signals_are_deduplicated() {
        let classifier =
            DiagnosisClassifier::new("1").expect("valid classifier");

        let repeated = signal(
            1,
            DetectionClassification::Fault,
            0.8,
        );

        let findings = classifier
            .classify_request(
                &request(vec![
                    repeated.clone(),
                    repeated,
                ]),
            )
            .expect("classification must succeed");

        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn duplicate_signals_can_be_preserved_when_explicitly_requested() {
        let mut config =
            DiagnosisClassifierConfig::standard();

        config.set_deduplicate_signals(false);

        let classifier =
            DiagnosisClassifier::with_config("1", config)
                .expect("valid classifier");

        let repeated = signal(
            1,
            DetectionClassification::Fault,
            0.8,
        );

        let findings = classifier
            .classify_request(
                &request(vec![
                    repeated.clone(),
                    repeated,
                ]),
            )
            .expect("classification must succeed");

        assert_eq!(findings.len(), 2);
    }

    #[test]
    fn sequence_mismatch_is_rejected() {
        let classifier =
            DiagnosisClassifier::new("1").expect("valid classifier");

        let invalid = DetectionSignal::new(
            signal_id(1),
            DetectorIdentity::new(
                "test.detector",
                "1",
            )
            .expect("valid detector"),
            DetectionClassification::Fault,
            DetectionConfidence::new(0.8)
                .expect("valid confidence"),
            None,
            sequence(2),
        );

        assert!(
            classifier
                .classify_request(&request(vec![invalid]))
                .is_err()
        );
    }

    #[test]
    fn configuration_can_fall_back_to_unknown() {
        let mut config =
            DiagnosisClassifierConfig::empty();

        config.set_fallback_category(
            DiagnosisCategory::Unknown,
        );

        let classifier =
            DiagnosisClassifier::with_config("1", config)
                .expect("valid classifier");

        let result = classifier
            .classify(
                DetectionClassification::Fault,
                DiagnosisConfidence::new(0.5)
                    .expect("valid confidence"),
            )
            .expect("classification must succeed");

        assert_eq!(result.0, DiagnosisCategory::Unknown);
        assert_eq!(result.1.value(), 0.5);
    }

    #[test]
    fn classifier_has_no_fixed_machine_limit() {
        let classifier =
            DiagnosisClassifier::new("1").expect("valid classifier");

        let signals: Vec<_> = (1_u64..=10_000_u64)
            .map(|id| {
                signal(
                    id,
                    DetectionClassification::Fault,
                    0.5,
                )
            })
            .collect();

        let findings = classifier
            .classify_request(&request(signals))
            .expect("classification must succeed");

        assert_eq!(findings.len(), 10_000);
    }
}