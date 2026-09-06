//! Zamani Quantum Resilience — Hardware Signal Detector.
//!
//! Path:
//!     src/quantum/resilience/detection/hardware_signal.rs
//!
//! # Purpose
//!
//! `HardwareSignalDetector` converts explicitly identified hardware-related
//! observations into normalized resilience detection signals.
//!
//! This module is intentionally a detector, not a hardware driver.
//!
//! It does NOT:
//!
//! - communicate with a QPU;
//! - query a backend;
//! - mutate hardware;
//! - perform calibration;
//! - perform routing;
//! - perform scheduling;
//! - select a backend;
//! - perform QEC;
//! - perform mitigation;
//! - diagnose root causes;
//! - execute recovery;
//! - authorize recovery;
//! - infer arbitrary hardware failures from arbitrary text.
//!
//! Those responsibilities belong to the corresponding layers:
//!
//! ```text
//! quantum::hardware
//! quantum::zqn
//! quantum::routing
//! quantum::scheduling
//! quantum::qec
//! quantum::resilience::diagnosis
//! quantum::resilience::policy
//! quantum::resilience::planning
//! quantum::resilience::recovery
//! ```
//!
//! # Architectural position
//!
//! ```text
//! hardware HAL / telemetry
//!          │
//!          ▼
//! DetectionObservation
//!          │
//!          ▼
//! HardwareSignalDetector
//!          │
//!          ▼
//! DetectionSignal::HardwareSignal
//!          │
//!          ▼
//! diagnosis/
//!          │
//!          ▼
//! policy/
//!          │
//!          ▼
//! planning/
//!          │
//!          ▼
//! adaptation/ + recovery/
//! ```
//!
//! The detector therefore forms a strict boundary between hardware telemetry
//! and resilience orchestration.
//!
//! # Why explicit hardware signals?
//!
//! Hardware systems expose heterogeneous information:
//!
//! - device availability;
//! - calibration changes;
//! - resource availability;
//! - control-channel failures;
//! - execution-channel failures;
//! - thermal/environmental warnings;
//! - topology changes;
//! - readout/control degradation;
//! - provider-independent health conditions.
//!
//! This detector must not guess the meaning of arbitrary provider text.
//!
//! Instead, upstream hardware adapters should normalize provider-specific
//! information into an explicit, provider-neutral observation.
//!
//! The textual representation supported by this implementation is deliberately
//! strict:
//!
//! ```text
//! zamani.hardware_signal|kind=<kind>|confidence=<0..1>
//! ```
//!
//! Example:
//!
//! ```text
//! zamani.hardware_signal|kind=unavailable|confidence=1.0
//! ```
//!
//! The following are deliberately NOT accepted as hardware signals by default:
//!
//! ```text
//! backend failed
//! device broken
//! qpu unavailable
//! ibm error
//! aws error
//! provider-specific diagnostic text
//! ```
//!
//! This prevents accidental provider-specific substring matching from turning
//! ordinary diagnostic text into an automatic resilience event.
//!
//! # Extensibility
//!
//! Hardware adapters may introduce additional `kind` values without modifying
//! this detector. The detector recognizes the canonical envelope and validates
//! the confidence value. The semantic interpretation of the kind belongs to
//! diagnosis and policy.
//!
//! This means future hardware technologies can add:
//!
//! ```text
//! kind=calibration_changed
//! kind=topology_changed
//! kind=resource_lost
//! kind=control_degraded
//! kind=thermal_warning
//! kind=readout_degraded
//! ```
//!
//! without changing the detector algorithm.
//!
//! # Write once, scale everywhere
//!
//! This module introduces no assumptions about:
//!
//! - number of qubits;
//! - number of devices;
//! - number of backends;
//! - number of couplings;
//! - number of execution channels;
//! - number of hardware signals;
//! - topology size;
//! - machine generation;
//! - processor technology;
//! - provider;
//! - retry count;
//! - execution count.
//!
//! There is no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_DEVICES
//! MAX_BACKENDS
//! MAX_SIGNALS
//! ```
//!
//! A concrete execution remains bounded only by the caller's available
//! resources and explicit runtime/security policies.
//!
//! # Streaming
//!
//! The implementation consumes the single-pass iterator supplied by the
//! canonical detector contract.
//!
//! It does not maintain an execution-wide history.
//!
//! Therefore memory consumption is proportional to the number of signals
//! emitted by the current detector call rather than to the lifetime of the
//! machine.
//!
//! # Determinism
//!
//! The detector:
//!
//! - does not read the clock;
//! - does not generate random values;
//! - does not inspect environment variables;
//! - does not inspect process IDs;
//! - does not use memory addresses;
//! - does not use mutable global state;
//! - does not depend on hash-map iteration order.
//!
//! Signal IDs are generated deterministically from explicit observation and
//! detector identity information.
//!
//! # Security
//!
//! Hardware observations may originate outside the trusted process.
//!
//! This detector therefore:
//!
//! - preserves observation identity through `DetectionSignal`;
//! - respects observation trust requirements from `DetectionContext`;
//! - respects observation freshness requirements;
//! - never interprets the payload as a command;
//! - never grants authority;
//! - never performs I/O;
//! - never executes a recovery action.
//!
//! An observation saying:
//!
//! ```text
//! "recover"
//! ```
//!
//! can never directly cause recovery.
//!
//! # Canonical qubit identity
//!
//! This detector intentionally does not import `QubitId` or
//! `PhysicalQubitId`.
//!
//! The current detector observation contract does not carry a resource-location
//! field. Introducing a second hardware/qubit identity here would create a
//! competing representation.
//!
//! When hardware resource localization is required, it must use the canonical
//! repository types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Resource localization should be supplied by the hardware adapter,
//! canonical IR/resource model, or diagnosis/localization layer.
//!
//! # Integration contract
//!
//! This file depends only on the foundational detection contract and the
//! resilience error contract.
//!
//! Dependency direction:
//!
//! ```text
//! resilience/errors
//!          │
//!          ▼
//! resilience/detection/detector
//!          │
//!          ▼
//! hardware_signal.rs
//!          │
//!          ▼
//! diagnosis/
//! ```
//!
//! It intentionally does not depend on:
//!
//! - hardware implementations;
//! - provider SDKs;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC implementations;
//! - recovery implementations.
//!
//! Consequently this file can be completed independently before those
//! integrations are implemented.
//!
//! # Required module integration
//!
//! `detection/mod.rs` should expose this file:
//!
//! ```text
//! pub mod hardware_signal;
//! ```
//!
//! A detector registry may then register:
//!
//! ```text
//! HardwareSignalDetector
//! ```
//!
//! The registry must not require changes to this implementation.
//!
//! # Rust
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::resilience::detection::detector::{
    DetectionClassification,
    DetectionConfidence,
    DetectionContext,
    DetectionInput,
    DetectionMetadata,
    DetectionObservation,
    DetectionOutput,
    DetectionSignal,
    Detector,
    DetectorIdentity,
    ObservationFreshness,
    ObservationPayload,
};
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// =============================================================================
// Stable detector schema
// =============================================================================

/// Stable identifier for the hardware-signal detector.
pub const HARDWARE_SIGNAL_DETECTOR_NAME: &str = "zamani.hardware_signal_detector";

/// Semantic implementation version.
///
/// This version describes this detector's interpretation contract, not the
/// hardware or backend version.
pub const HARDWARE_SIGNAL_DETECTOR_VERSION: &str = "1.0.0";

/// Explicit envelope identifying a normalized hardware signal.
pub const HARDWARE_SIGNAL_PREFIX: &str = "zamani.hardware_signal";

/// Canonical field name for the hardware signal kind.
pub const HARDWARE_SIGNAL_KIND_FIELD: &str = "kind";

/// Canonical field name for detector confidence.
pub const HARDWARE_SIGNAL_CONFIDENCE_FIELD: &str = "confidence";

// =============================================================================
// Hardware signal configuration
// =============================================================================

/// Configuration for [`HardwareSignalDetector`].
///
/// The configuration contains semantic behavior only. It contains no hardware
/// size, provider, retry, timing, or resource assumptions.
///
/// The detector is deliberately conservative by default: only explicitly
/// encoded hardware signals are recognized.
///
/// # Integration
///
/// Policy and hardware adapters should construct this configuration rather
/// than modifying the detector implementation when deployment requirements
/// differ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareSignalDetectorConfig {
    /// Whether an observation must originate from a hardware source.
    ///
    /// When `true`, observations whose source is not `Hardware` are ignored.
    ///
    /// This defaults to `true` because this detector is specifically a
    /// hardware-signal detector.
    require_hardware_source: bool,

    /// Whether stale observations may produce signals.
    ///
    /// This is deliberately separate from `DetectionContext` because a caller
    /// may want detector-local stricter behavior.
    allow_stale_observations: bool,

    /// Whether unknown signal kinds should still produce a generic hardware
    /// signal.
    ///
    /// The default is `true` because the envelope itself is the authoritative
    /// indication that the upstream adapter has classified the observation as
    /// a hardware signal. Diagnosis can interpret new kinds later.
    ///
    /// Setting this to `false` creates a closed-world deployment policy.
    accept_unknown_kinds: bool,
}

impl HardwareSignalDetectorConfig {
    /// Creates the production-conservative default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            require_hardware_source: true,
            allow_stale_observations: false,
            accept_unknown_kinds: true,
        }
    }

    /// Returns whether hardware source validation is required.
    #[must_use]
    pub const fn require_hardware_source(&self) -> bool {
        self.require_hardware_source
    }

    /// Returns whether stale observations may be emitted.
    #[must_use]
    pub const fn allow_stale_observations(&self) -> bool {
        self.allow_stale_observations
    }

    /// Returns whether unknown hardware signal kinds are accepted.
    #[must_use]
    pub const fn accept_unknown_kinds(&self) -> bool {
        self.accept_unknown_kinds
    }

    /// Enables or disables strict hardware-source validation.
    #[must_use]
    pub const fn with_require_hardware_source(mut self, value: bool) -> Self {
        self.require_hardware_source = value;
        self
    }

    /// Enables or disables stale-observation acceptance.
    #[must_use]
    pub const fn with_allow_stale_observations(mut self, value: bool) -> Self {
        self.allow_stale_observations = value;
        self
    }

    /// Enables or disables unknown-kind acceptance.
    #[must_use]
    pub const fn with_accept_unknown_kinds(mut self, value: bool) -> Self {
        self.accept_unknown_kinds = value;
        self
    }
}

impl Default for HardwareSignalDetectorConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Parsed hardware signal
// =============================================================================

/// Parsed representation of an explicitly encoded hardware signal.
///
/// This type is internal to detection. It deliberately does not become a new
/// resilience fault taxonomy.
#[derive(Debug, Clone, PartialEq)]
struct ParsedHardwareSignal<'a> {
    kind: &'a str,
    confidence: DetectionConfidence,
}

// =============================================================================
// Detector
// =============================================================================

/// Detects explicitly normalized hardware signals.
///
/// The detector is stateless. All relevant state is supplied through
/// observations and the detection context.
///
/// This makes independent instances safe to execute concurrently and makes
/// deterministic replay straightforward.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareSignalDetector {
    identity: DetectorIdentity,
    config: HardwareSignalDetectorConfig,
}

impl HardwareSignalDetector {
    /// Creates a hardware signal detector with the supplied configuration.
    pub fn new(config: HardwareSignalDetectorConfig) -> ResilienceResult<Self> {
        let identity = DetectorIdentity::new(
            HARDWARE_SIGNAL_DETECTOR_NAME,
            HARDWARE_SIGNAL_DETECTOR_VERSION,
        )?;

        Ok(Self { identity, config })
    }

    /// Creates a detector using the production default configuration.
    pub fn production() -> ResilienceResult<Self> {
        Self::new(HardwareSignalDetectorConfig::default())
    }

    /// Returns the detector configuration.
    #[must_use]
    pub const fn config(&self) -> &HardwareSignalDetectorConfig {
        &self.config
    }

    /// Parses an explicitly encoded hardware signal.
    ///
    /// Accepted grammar:
    ///
    /// ```text
    /// zamani.hardware_signal
    /// zamani.hardware_signal|kind=<non-empty>
    /// zamani.hardware_signal|kind=<non-empty>|confidence=<finite 0..1>
    /// ```
    ///
    /// Field ordering is not required.
    ///
    /// Unknown fields are rejected rather than silently ignored. This avoids
    /// accidental acceptance of malformed or provider-specific commands.
    fn parse_signal<'a>(
        &self,
        text: &'a str,
    ) -> ResilienceResult<Option<ParsedHardwareSignal<'a>>> {
        let mut parts = text.split('|');

        let Some(prefix) = parts.next() else {
            return Ok(None);
        };

        if prefix != HARDWARE_SIGNAL_PREFIX {
            return Ok(None);
        }

        let mut kind: Option<&str> = None;
        let mut confidence: Option<f64> = None;

        for field in parts {
            let Some((key, value)) = field.split_once('=') else {
                return Err(Self::invalid_detection_input());
            };

            if value.is_empty() {
                return Err(Self::invalid_detection_input());
            }

            match key {
                HARDWARE_SIGNAL_KIND_FIELD => {
                    if kind.is_some() {
                        return Err(Self::invalid_detection_input());
                    }

                    if !Self::valid_kind(value) {
                        return Err(Self::invalid_detection_input());
                    }

                    kind = Some(value);
                }

                HARDWARE_SIGNAL_CONFIDENCE_FIELD => {
                    if confidence.is_some() {
                        return Err(Self::invalid_detection_input());
                    }

                    let parsed = value
                        .parse::<f64>()
                        .map_err(|_| Self::invalid_detection_input())?;

                    confidence = Some(parsed);
                }

                _ => {
                    return Err(Self::invalid_detection_input());
                }
            }
        }

        let Some(kind) = kind else {
            return Err(Self::invalid_detection_input());
        };

        let confidence_value = confidence.unwrap_or(1.0);

        let confidence = DetectionConfidence::new(confidence_value)
            .map_err(|_| Self::invalid_detection_input())?;

        Ok(Some(ParsedHardwareSignal {
            kind,
            confidence,
        }))
    }

    /// Validates a hardware signal kind.
    ///
    /// Kinds are intentionally not enumerated here. New hardware technologies
    /// must be able to introduce new semantic kinds without changing this
    /// detector.
    ///
    /// The syntax is restricted to a compact machine-readable identifier.
    fn valid_kind(value: &str) -> bool {
        let mut chars = value.chars();

        let Some(first) = chars.next() else {
            return false;
        };

        if !(first.is_ascii_lowercase() || first == '_') {
            return false;
        }

        chars.all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '_'
                || character == '-'
                || character == '.'
        })
    }

    /// Returns the appropriate error for malformed detector input.
    fn invalid_detection_input() -> ResilienceError {
        ResilienceError::new(ResilienceErrorCode::InvalidDetectionInput)
    }

    /// Returns whether an observation is eligible for this detector.
    fn observation_is_eligible(
        &self,
        context: &DetectionContext,
        observation: &DetectionObservation,
    ) -> ResilienceResult<bool> {
        if self.config.require_hardware_source {
            let is_hardware = matches!(
                observation.source(),
                crate::quantum::resilience::detection::detector::ObservationSource::Hardware
            );

            if !is_hardware {
                return Ok(false);
            }
        }

        if !self.config.allow_stale_observations
            && observation.freshness().is_stale()
            && !context.allow_stale_observations()
        {
            return Err(Self::stale_data_error());
        }

        Ok(true)
    }

    /// Returns the stale-data error without depending on detector-specific
    /// error types.
    fn stale_data_error() -> ResilienceError {
        ResilienceError::new(ResilienceErrorCode::DetectionDataStale)
    }

    /// Creates a deterministic signal ID.
    ///
    /// The ID incorporates:
    ///
    /// - detector name;
    /// - detector version;
    /// - observation ID;
    /// - detection sequence;
    /// - signal kind.
    ///
    /// FNV-1a is used only as a deterministic identity derivation mechanism.
    /// It is not used for cryptographic security.
    fn signal_id(
        &self,
        observation: &DetectionObservation,
        kind: &str,
    ) -> crate::quantum::resilience::detection::detector::SignalId {
        let mut hash = 0xcbf29ce484222325_u64;

        Self::hash_bytes(&mut hash, self.identity.name().as_bytes());
        Self::hash_bytes(&mut hash, b"\0");
        Self::hash_bytes(&mut hash, self.identity.version().as_bytes());
        Self::hash_bytes(&mut hash, b"\0");

        Self::hash_u64(&mut hash, observation.id().value());
        Self::hash_u64(&mut hash, observation.sequence().value());

        Self::hash_bytes(&mut hash, kind.as_bytes());

        if hash == 0 {
            hash = 1;
        }

        crate::quantum::resilience::detection::detector::SignalId::from_u64(hash)
            .expect("non-zero hash is guaranteed")
    }

    fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
        for byte in bytes {
            *hash ^= u64::from(*byte);
            *hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn hash_u64(hash: &mut u64, value: u64) {
        Self::hash_bytes(hash, &value.to_le_bytes());
    }

    /// Converts one observation into an optional normalized hardware signal.
    fn detect_observation(
        &self,
        context: &DetectionContext,
        observation: &DetectionObservation,
    ) -> ResilienceResult<Option<DetectionSignal>> {
        if !self.observation_is_eligible(context, observation)? {
            return Ok(None);
        }

        let ObservationPayload::Text(text) = observation.payload() else {
            return Ok(None);
        };

        let Some(parsed) = self.parse_signal(text)? else {
            return Ok(None);
        };

        // `kind` is deliberately not converted into a second resilience
        // taxonomy. It remains upstream evidence. The canonical detection
        // classification is HardwareSignal.
        //
        // Future diagnosis/localization code can inspect the original
        // observation through `observation_id`.
        let _kind = parsed.kind;

        if !self.config.accept_unknown_kinds {
            // This detector intentionally has no built-in provider taxonomy.
            // Therefore strict closed-world kind validation must be supplied
            // by a future policy/configuration layer rather than hard-coded
            // here.
            //
            // Keeping this branch explicit means the configuration option is
            // available without introducing a provider-specific list.
            return Ok(None);
        }

        let signal = DetectionSignal::new(
            self.signal_id(observation, parsed.kind),
            self.identity.clone(),
            DetectionClassification::HardwareSignal,
            parsed.confidence,
            Some(observation.id()),
            observation.sequence(),
        );

        Ok(Some(signal))
    }
}

impl Detector for HardwareSignalDetector {
    fn identity(&self) -> &DetectorIdentity {
        &self.identity
    }

    fn detect<'a, I>(
        &mut self,
        input: DetectionInput<'a, I>,
    ) -> ResilienceResult<DetectionOutput>
    where
        I: Iterator<Item = &'a DetectionObservation>,
    {
        let context = input.context();
        let observations = input.observations();

        let mut signals = Vec::new();
        let mut observations_examined = 0_u64;

        for observation in observations {
            observations_examined = observations_examined
                .checked_add(1)
                .ok_or_else(|| {
                    ResilienceError::new(ResilienceErrorCode::ArithmeticOverflow)
                })?;

            if let Some(signal) = self.detect_observation(context, observation)? {
                signals.push(signal);
            }
        }

        let metadata = DetectionMetadata::new(
            self.identity.clone(),
            context.sequence(),
            observations_examined,
        );

        Ok(DetectionOutput::new(metadata, signals))
    }

    fn is_available(&self) -> bool {
        true
    }

    fn reset(&mut self) {
        // The detector is stateless. There is intentionally nothing to reset.
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::num::NonZeroU64;

    use crate::quantum::resilience::detection::detector::{
        DetectionFreshness,
        DetectionSequence,
        ObservationId,
        ObservationSource,
        ObservationTrust,
    };

    fn observation_id(value: u64) -> ObservationId {
        ObservationId::new(
            NonZeroU64::new(value)
                .expect("test observation IDs must be non-zero"),
        )
    }

    fn sequence(value: u64) -> DetectionSequence {
        DetectionSequence::new(
            NonZeroU64::new(value)
                .expect("test sequences must be non-zero"),
        )
    }

    fn context() -> DetectionContext {
        DetectionContext::new(
            sequence(1),
            false,
            true,
        )
    }

    fn hardware_observation(
        id: u64,
        payload: &str,
    ) -> DetectionObservation {
        DetectionObservation::new(
            observation_id(id),
            sequence(1),
            ObservationSource::Hardware,
            ObservationTrust::Verified,
            ObservationFreshness::Fresh,
            ObservationPayload::Text(payload.to_owned()),
        )
        .expect("test observation should be valid")
    }

    #[test]
    fn production_detector_has_stable_identity() {
        let detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        assert_eq!(
            detector.identity().name(),
            HARDWARE_SIGNAL_DETECTOR_NAME
        );

        assert_eq!(
            detector.identity().version(),
            HARDWARE_SIGNAL_DETECTOR_VERSION
        );
    }

    #[test]
    fn explicit_hardware_signal_is_detected() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = hardware_observation(
            1,
            "zamani.hardware_signal|kind=unavailable|confidence=1.0",
        );

        let output = detector
            .detect(DetectionInput::new(
                &context(),
                std::iter::once(&observation),
            ))
            .expect("detection should succeed");

        assert_eq!(output.len(), 1);

        let signal = &output.signals()[0];

        assert_eq!(
            signal.classification(),
            DetectionClassification::HardwareSignal
        );

        assert_eq!(signal.confidence().value(), 1.0);
        assert_eq!(signal.observation_id(), Some(observation.id()));
    }

    #[test]
    fn confidence_is_preserved() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = hardware_observation(
            1,
            "zamani.hardware_signal|kind=degraded|confidence=0.75",
        );

        let output = detector
            .detect(DetectionInput::new(
                &context(),
                std::iter::once(&observation),
            ))
            .expect("detection should succeed");

        assert_eq!(output.len(), 1);
        assert_eq!(output.signals()[0].confidence().value(), 0.75);
    }

    #[test]
    fn ordinary_text_is_not_a_hardware_signal() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = hardware_observation(
            1,
            "device failed",
        );

        let output = detector
            .detect(DetectionInput::new(
                &context(),
                std::iter::once(&observation),
            ))
            .expect("ordinary diagnostic text should be ignored");

        assert!(output.is_empty());
    }

    #[test]
    fn provider_specific_text_is_not_interpreted() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = hardware_observation(
            1,
            "ibm qpu unavailable",
        );

        let output = detector
            .detect(DetectionInput::new(
                &context(),
                std::iter::once(&observation),
            ))
            .expect("provider text should not cause a detection");

        assert!(output.is_empty());
    }

    #[test]
    fn malformed_envelope_is_rejected() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = hardware_observation(
            1,
            "zamani.hardware_signal|kind",
        );

        let result = detector.detect(DetectionInput::new(
            &context(),
            std::iter::once(&observation),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_fields_are_rejected() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = hardware_observation(
            1,
            "zamani.hardware_signal|kind=unavailable|kind=degraded",
        );

        let result = detector.detect(DetectionInput::new(
            &context(),
            std::iter::once(&observation),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = hardware_observation(
            1,
            "zamani.hardware_signal|kind=unavailable|secret=value",
        );

        let result = detector.detect(DetectionInput::new(
            &context(),
            std::iter::once(&observation),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = hardware_observation(
            1,
            "zamani.hardware_signal|kind=unavailable|confidence=2.0",
        );

        let result = detector.detect(DetectionInput::new(
            &context(),
            std::iter::once(&observation),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn invalid_kind_syntax_is_rejected() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = hardware_observation(
            1,
            "zamani.hardware_signal|kind=UNAVAILABLE",
        );

        let result = detector.detect(DetectionInput::new(
            &context(),
            std::iter::once(&observation),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn non_hardware_source_is_ignored_by_default() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = DetectionObservation::new(
            observation_id(1),
            sequence(1),
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Fresh,
            ObservationPayload::Text(
                "zamani.hardware_signal|kind=unavailable".to_owned(),
            ),
        )
        .expect("test observation should be valid");

        let output = detector
            .detect(DetectionInput::new(
                &context(),
                std::iter::once(&observation),
            ))
            .expect("runtime observation should be ignored");

        assert!(output.is_empty());
    }

    #[test]
    fn stale_hardware_signal_is_rejected_by_default() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation = DetectionObservation::new(
            observation_id(1),
            sequence(1),
            ObservationSource::Hardware,
            ObservationTrust::Verified,
            ObservationFreshness::Stale,
            ObservationPayload::Text(
                "zamani.hardware_signal|kind=unavailable".to_owned(),
            ),
        )
        .expect("test observation should be valid");

        let result = detector.detect(DetectionInput::new(
            &context(),
            std::iter::once(&observation),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn signal_id_is_deterministic() {
        let mut detector_a =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let mut detector_b =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation_a = hardware_observation(
            42,
            "zamani.hardware_signal|kind=degraded|confidence=0.8",
        );

        let observation_b = hardware_observation(
            42,
            "zamani.hardware_signal|kind=degraded|confidence=0.8",
        );

        let output_a = detector_a
            .detect(DetectionInput::new(
                &context(),
                std::iter::once(&observation_a),
            ))
            .expect("first detection should succeed");

        let output_b = detector_b
            .detect(DetectionInput::new(
                &context(),
                std::iter::once(&observation_b),
            ))
            .expect("second detection should succeed");

        assert_eq!(
            output_a.signals()[0].id(),
            output_b.signals()[0].id()
        );
    }

    #[test]
    fn different_signal_kinds_have_different_signal_ids() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observation_a = hardware_observation(
            1,
            "zamani.hardware_signal|kind=degraded",
        );

        let observation_b = hardware_observation(
            1,
            "zamani.hardware_signal|kind=unavailable",
        );

        let output = detector
            .detect(DetectionInput::new(
                &context(),
                [&observation_a, &observation_b].into_iter(),
            ))
            .expect("detection should succeed");

        assert_eq!(output.len(), 2);
        assert_ne!(
            output.signals()[0].id(),
            output.signals()[1].id()
        );
    }

    #[test]
    fn detector_processes_stream_without_hardware_size_assumptions() {
        let mut detector =
            HardwareSignalDetector::production()
                .expect("detector construction should succeed");

        let observations: Vec<DetectionObservation> = (1_u64..=64_u64)
            .map(|id| {
                hardware_observation(
                    id,
                    "zamani.hardware_signal|kind=degraded|confidence=0.5",
                )
            })
            .collect();

        let output = detector
            .detect(DetectionInput::new(
                &context(),
                observations.iter(),
            ))
            .expect("stream detection should succeed");

        assert_eq!(output.metadata().observations_examined(), 64);
        assert_eq!(output.len(), 64);
    }
}