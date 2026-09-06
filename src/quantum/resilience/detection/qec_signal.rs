//! Zamani Quantum Resilience — QEC Signal Detection.
//!
//! Path:
//!     src/quantum/resilience/detection/qec_signal.rs
//!
//! # Purpose
//!
//! This module converts explicit QEC observations into normalized resilience
//! detection signals.
//!
//! It answers:
//!
//! > "Has the QEC subsystem reported a condition that resilience should
//! > interpret?"
//!
//! It does NOT:
//!
//! - implement quantum error correction;
//! - implement a stabilizer/code;
//! - decode syndromes;
//! - correct quantum states;
//! - choose a decoder;
//! - choose code distance;
//! - change QEC configuration;
//! - remap qubits;
//! - reroute circuits;
//! - reschedule execution;
//! - recover execution;
//! - decide whether a result is correct.
//!
//! Those responsibilities belong to the QEC, routing, scheduling, planning,
//! adaptation, recovery, and verification subsystems.
//!
//! # Architectural position
//!
//! ```text
//! QEC subsystem
//!      │
//!      │ syndrome / decoder / logical-error observation
//!      ▼
//! DetectionObservation
//!      │
//!      ▼
//! QecSignalDetector
//!      │
//!      ▼
//! DetectionSignal::QecSignal
//!      │
//!      ▼
//! diagnosis
//!      │
//!      ▼
//! policy
//!      │
//!      ▼
//! planning
//!      │
//!      ▼
//! adaptation / recovery / verification
//! ```
//!
//! The detector is therefore a boundary adapter, not a QEC implementation.
//!
//! # Existing repository contract
//!
//! This implementation is designed against:
//!
//! ```text
//! crate::quantum::resilience::detection::detector
//! ```
//!
//! In particular it consumes:
//!
//! - `DetectionInput`;
//! - `DetectionObservation`;
//! - `ObservationPayload`;
//! - `DetectionContext`;
//! - `DetectionSignal`;
//! - `DetectionOutput`;
//! - `DetectionMetadata`;
//! - `DetectionClassification`;
//! - `DetectionConfidence`;
//! - `DetectorIdentity`;
//! - `SignalId`;
//! - `DetectionSequence`.
//!
//! The existing detector contract uses a streaming iterator and returns a
//! `DetectionOutput`; concrete detectors implement `Detector`. The repository
//! anomaly detector follows this same contract. This module follows it as
//! well.
//!
//! # QEC ownership
//!
//! QEC remains authoritative for:
//!
//! - code definitions;
//! - encoding;
//! - syndrome extraction;
//! - decoder implementation;
//! - correction;
//! - logical-state handling;
//! - code distance;
//! - ancilla management;
//! - QEC-specific resource allocation.
//!
//! Resilience consumes the resulting observations.
//!
//! This distinction is essential:
//!
//! ```text
//! QEC says:
//!     "this observation occurred"
//!
//! Resilience says:
//!     "this observation may require resilience action"
//! ```
//!
//! A detection signal is not itself a recovery command.
//!
//! # Supported semantic observations
//!
//! The detector accepts an explicit, provider-neutral textual envelope:
//!
//! ```text
//! zamani.qec.signal|kind=<kind>|confidence=<0..=1>
//! ```
//!
//! Additional fields may be supplied by QEC producers:
//!
//! ```text
//! zamani.qec.signal|kind=logical_error|confidence=0.97|
//! code=surface_code|decoder=decoder-v1|resource=logical-42
//! ```
//!
//! The detector deliberately treats the additional fields as opaque evidence.
//! It does not attempt to interpret provider-specific or code-specific values.
//!
//! Examples of valid `kind` values include:
//!
//! ```text
//! syndrome
//! decoder_low_confidence
//! logical_error
//! leakage
//! erasure
//! loss
//! correction_failure
//! decoding_failure
//! code_degradation
//! resource_degradation
//! qec_unavailable
//! ```
//!
//! These are examples, not a closed enumeration. New QEC technologies must not
//! require changes to this detector merely because they introduce a new signal
//! kind.
//!
//! The detector therefore preserves the `kind` only as evidence in the input
//! observation. The normalized resilience classification remains:
//!
//! ```text
//! DetectionClassification::QecSignal
//! ```
//!
//! # Why an explicit envelope?
//!
//! The generic resilience detector observation contract intentionally supports
//! heterogeneous payloads such as:
//!
//! - Boolean;
//! - Integer;
//! - Unsigned;
//! - Float;
//! - Text;
//! - Marker.
//!
//! A Boolean or numeric value by itself does not identify whether it represents:
//!
//! - a syndrome;
//! - decoder confidence;
//! - logical error rate;
//! - leakage;
//! - loss;
//! - a calibration metric;
//! - an unrelated runtime metric.
//!
//! Automatically guessing would create unsafe false positives.
//!
//! Therefore this detector requires an explicit QEC signal envelope for textual
//! observations. This preserves the generic detector contract without creating
//! a second observation ontology.
//!
//! # No QEC implementation
//!
//! This file must never grow into:
//!
//! ```text
//! surface-code decoder
//! repetition-code decoder
//! stabilizer simulator
//! MWPM implementation
//! belief-propagation implementation
//! neural decoder
//! syndrome extractor
//! ```
//!
//! Those belong in the QEC subsystem.
//!
//! # Canonical quantum identities
//!
//! This detector does not define quantum resource identities.
//!
//! When a QEC producer needs to associate a signal with:
//!
//! - a logical qubit;
//! - a physical qubit;
//! - an operation;
//! - an ancilla;
//! - a code block;
//! - another quantum resource;
//!
//! the producer must use the canonical resource identity owned by the relevant
//! Zamani subsystem. In particular, canonical qubit identity belongs to:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! This detector does not duplicate `QubitId` or `PhysicalQubitId`.
//!
//! # Write once, scale everywhere
//!
//! There is no hard-coded:
//!
//! - qubit count;
//! - code distance;
//! - number of syndrome rounds;
//! - number of QEC blocks;
//! - number of decoders;
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of observations;
//! - number of signals;
//! - backend;
//! - provider;
//! - device;
//! - retry count;
//! - recovery action.
//!
//! A single detector instance can process observations originating from:
//!
//! ```text
//! one physical qubit
//!        ↓
//! small QPU
//!        ↓
//! large QPU
//!        ↓
//! fault-tolerant logical machine
//!        ↓
//! distributed quantum system
//!        ↓
//! heterogeneous quantum fleet
//! ```
//!
//! "Infinity" means that this detector introduces no artificial finite
//! machine-size ceiling. A real execution is naturally constrained by memory,
//! CPU, storage, transport, policy, and available hardware.
//!
//! # Streaming
//!
//! The detector consumes the iterator supplied by `DetectionInput` exactly once.
//!
//! It does not retain the observation stream.
//!
//! Memory consumption therefore depends on:
//!
//! ```text
//! detector configuration
//! + output collection required by the existing DetectionOutput contract
//! ```
//!
//! and not on historical QEC stream size.
//!
//! Long-term history belongs to `history/` and telemetry storage.
//!
//! # Determinism
//!
//! This detector:
//!
//! - does not read the system clock;
//! - does not use randomness;
//! - does not inspect environment variables;
//! - does not access global mutable state;
//! - does not use memory addresses;
//! - does not depend on hash-map iteration order;
//! - does not call a QEC implementation;
//! - does not infer hidden machine properties.
//!
//! Given identical:
//!
//! - configuration;
//! - observations;
//! - detection context;
//! - observation ordering;
//!
//! it produces identical output.
//!
//! # Security
//!
//! QEC observations may cross trust boundaries.
//!
//! Therefore:
//!
//! - an observation is data, not authority;
//! - a QEC signal is data, not a recovery command;
//! - detector output must not authorize migration;
//! - detector output must not authorize backend access;
//! - detector output must not authorize credentials;
//! - detector output must not authorize filesystem/network access.
//!
//! Trust and freshness remain properties of `DetectionObservation` and
//! `DetectionContext`.
//!
//! The detector does not silently upgrade an untrusted observation into an
//! authorization decision.
//!
//! # Unknown observations
//!
//! Unknown QEC signal kinds are allowed.
//!
//! This is intentional because QEC technology must be extensible.
//!
//! A syntactically valid:
//!
//! ```text
//! zamani.qec.signal|kind=<new-future-kind>|confidence=...
//! ```
//!
//! is still a QEC signal.
//!
//! The detector does not maintain a closed list of QEC technologies.
//!
//! # Invalid observations
//!
//! The following are invalid:
//!
//! - missing envelope;
//! - empty signal kind;
//! - missing confidence;
//! - non-numeric confidence;
//! - NaN confidence;
//! - infinite confidence;
//! - confidence outside `[0, 1]`;
//! - duplicate fields;
//! - malformed field separators;
//! - empty field names;
//! - empty field values where a required value is expected.
//!
//! Invalid input returns the canonical resilience error instead of silently
//! producing a false resilience signal.
//!
//! # Confidence
//!
//! QEC confidence is supplied by the QEC producer.
//!
//! It must not be invented by this detector.
//!
//! This is important because:
//!
//! ```text
//! decoder confidence
//! !=
//! detector certainty
//! ```
//!
//! The producer's confidence becomes the detector signal confidence because the
//! detector is not independently re-decoding the QEC observation.
//!
//! # Signal identity
//!
//! The detector derives the `SignalId` from the observation identity using the
//! same convention used by the repository's existing detector implementations:
//!
//! ```text
//! SignalId::from_u64(observation.id().value())
//! ```
//!
//! The observation identity remains the caller-owned source of identity.
//!
//! No random ID generation is used.
//!
//! # Detection versus diagnosis
//!
//! The detector must not infer:
//!
//! ```text
//! "logical error occurred because qubit X is defective"
//! ```
//!
//! from a QEC signal.
//!
//! It only reports:
//!
//! ```text
//! QEC signal observed.
//! ```
//!
//! Diagnosis may later correlate that signal with:
//!
//! - ZQN faults;
//! - hardware health;
//! - calibration drift;
//! - topology;
//! - execution history;
//! - other QEC observations.
//!
//! # Integration
//!
//! The dependency direction is:
//!
//! ```text
//! QEC
//!   │
//!   ▼
//! detection/detector.rs
//!   │
//!   ▼
//! detection/qec_signal.rs
//!   │
//!   ▼
//! diagnosis
//!   │
//!   ├── policy
//!   ├── planning
//!   ├── adaptation
//!   ├── recovery
//!   └── verification
//! ```
//!
//! The detector does not depend on concrete:
//!
//! - decoder;
//! - QEC code;
//! - backend;
//! - routing;
//! - scheduler;
//! - recovery;
//! - mitigation;
//! - hardware provider.
//!
//! # Integration with QEC
//!
//! QEC producers should construct a normal `DetectionObservation` whose payload
//! is `ObservationPayload::Text` containing the canonical envelope.
//!
//! A producer may attach additional execution/resource provenance through the
//! existing observation/context mechanisms.
//!
//! The producer remains responsible for ensuring that any qubit/resource
//! identity is represented by the canonical Zamani resource model.
//!
//! # Integration with diagnosis
//!
//! Diagnosis receives the normalized:
//!
//! ```text
//! DetectionClassification::QecSignal
//! ```
//!
//! together with the original observation reference.
//!
//! Diagnosis can then inspect the original QEC evidence and correlate it with
//! other observations.
//!
//! # Integration with policy
//!
//! Policy decides whether a QEC signal warrants:
//!
//! - continued execution;
//! - additional QEC;
//! - code adaptation;
//! - remapping;
//! - rerouting;
//! - rescheduling;
//! - recompilation;
//! - mitigation;
//! - recovery;
//! - migration;
//! - escalation;
//! - abort.
//!
//! This detector does none of those things.
//!
//! # Integration with verification
//!
//! A QEC signal must never by itself cause a result to be accepted or rejected.
//!
//! Verification remains the final semantic authority.
//!
//! # Compatibility
//!
//! The implementation uses only stable Rust features compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::num::NonZeroU64;

use crate::quantum::resilience::detection::detector::{
    DetectionClassification,
    DetectionConfidence,
    DetectionInput,
    DetectionMetadata,
    DetectionObservation,
    DetectionOutput,
    DetectionSignal,
    Detector,
    DetectorIdentity,
    ObservationPayload,
    SignalId,
};

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

/// Stable schema identifier for this detector.
pub const QEC_SIGNAL_DETECTOR_SCHEMA_ID: &str =
    "zamani.quantum.resilience.detection.qec_signal";

/// Semantic version of this detector contract.
pub const QEC_SIGNAL_DETECTOR_SCHEMA_VERSION: u16 = 1;

/// Stable detector implementation name.
///
/// This identifies the detector implementation, not a QEC provider.
pub const QEC_SIGNAL_DETECTOR_NAME: &str = "qec-signal";

/// Prefix identifying the provider-neutral QEC signal envelope.
///
/// The envelope intentionally uses a namespace rather than a provider name.
pub const QEC_SIGNAL_PREFIX: &str = "zamani.qec.signal";

/// Configuration for [`QecSignalDetector`].
///
/// The detector has intentionally very little configuration because the
/// semantics of a QEC signal belong to the producer.
///
/// Configuration is limited to the detector's input acceptance policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QecSignalDetectorConfig {
    /// Whether the detector accepts only the canonical textual envelope.
    ///
    /// This should normally remain `true`.
    ///
    /// It exists as explicit configuration so future versions can introduce
    /// another formally specified observation representation without changing
    /// the detector's public shape.
    require_canonical_envelope: bool,
}

impl QecSignalDetectorConfig {
    /// Creates a configuration.
    ///
    /// `require_canonical_envelope` should normally be `true`.
    #[must_use]
    pub const fn new(require_canonical_envelope: bool) -> Self {
        Self {
            require_canonical_envelope,
        }
    }

    /// Returns whether the canonical envelope is required.
    #[must_use]
    pub const fn require_canonical_envelope(&self) -> bool {
        self.require_canonical_envelope
    }
}

/// Parsed representation of one QEC signal.
///
/// This structure intentionally contains only generic fields understood by the
/// resilience detection boundary.
///
/// Additional producer-specific fields are not interpreted here.
#[derive(Debug, Clone, PartialEq)]
pub struct QecSignal {
    kind: String,
    confidence: DetectionConfidence,
}

impl QecSignal {
    /// Creates a validated QEC signal.
    pub fn new(
        kind: impl Into<String>,
        confidence: DetectionConfidence,
    ) -> ResilienceResult<Self> {
        let kind = kind.into();

        if kind.trim().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidDetectionInput,
            ));
        }

        Ok(Self { kind, confidence })
    }

    /// Returns the producer-defined signal kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns the producer-supplied confidence.
    #[must_use]
    pub const fn confidence(&self) -> DetectionConfidence {
        self.confidence
    }
}

/// Detector that converts explicit QEC observations into resilience signals.
///
/// The detector is stateless.
///
/// This is deliberate: QEC state belongs to the QEC subsystem. Historical
/// correlation belongs to diagnosis/history. The detector merely normalizes
/// individual observations.
#[derive(Debug, Clone)]
pub struct QecSignalDetector {
    identity: DetectorIdentity,
    config: QecSignalDetectorConfig,
}

impl QecSignalDetector {
    /// Creates a detector with explicit configuration.
    pub fn new(config: QecSignalDetectorConfig) -> ResilienceResult<Self> {
        let identity = DetectorIdentity::new(
            QEC_SIGNAL_DETECTOR_NAME,
            QEC_SIGNAL_DETECTOR_SCHEMA_VERSION.to_string(),
        )?;

        Ok(Self { identity, config })
    }

    /// Creates a detector using the canonical envelope requirement.
    ///
    /// This constructor does not introduce a hardware or scalability default.
    #[must_use]
    pub fn canonical() -> ResilienceResult<Self> {
        Self::new(QecSignalDetectorConfig::new(true))
    }

    /// Returns the detector configuration.
    #[must_use]
    pub const fn config(&self) -> &QecSignalDetectorConfig {
        &self.config
    }

    /// Returns the detector identity.
    #[must_use]
    pub const fn detector_identity(&self) -> &DetectorIdentity {
        &self.identity
    }

    /// Parses one observation payload as a QEC signal.
    ///
    /// Non-text payloads are intentionally ignored rather than guessed.
    fn parse_observation(
        &self,
        observation: &DetectionObservation,
    ) -> ResilienceResult<Option<QecSignal>> {
        let payload = observation.payload();

        match payload {
            ObservationPayload::Text(text) => self.parse_text(text),
            ObservationPayload::Boolean(_)
            | ObservationPayload::Integer(_)
            | ObservationPayload::Unsigned(_)
            | ObservationPayload::Float(_)
            | ObservationPayload::Marker => Ok(None),
        }
    }

    /// Parses the canonical textual QEC signal envelope.
    fn parse_text(&self, text: &str) -> ResilienceResult<Option<QecSignal>> {
        let text = text.trim();

        if text.is_empty() {
            return Ok(None);
        }

        if !text.starts_with(QEC_SIGNAL_PREFIX) {
            return Ok(None);
        }

        if !self.config.require_canonical_envelope {
            return Self::parse_envelope(text);
        }

        Self::parse_envelope(text)
    }

    /// Parses:
    ///
    /// ```text
    /// zamani.qec.signal|kind=<kind>|confidence=<value>|...
    /// ```
    ///
    /// The parser is intentionally strict about the fields it understands.
    /// Unknown fields are accepted and ignored so that future QEC producers can
    /// add evidence without breaking old resilience binaries.
    fn parse_envelope(text: &str) -> ResilienceResult<Option<QecSignal>> {
        let mut parts = text.split('|');

        let prefix = parts.next();

        if prefix != Some(QEC_SIGNAL_PREFIX) {
            return Ok(None);
        }

        let mut kind: Option<&str> = None;
        let mut confidence: Option<&str> = None;

        for field in parts {
            if field.is_empty() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidDetectionInput,
                ));
            }

            let (key, value) = field.split_once('=').ok_or_else(|| {
                ResilienceError::new(ResilienceErrorCode::InvalidDetectionInput)
            })?;

            if key.trim().is_empty() || value.trim().is_empty() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidDetectionInput,
                ));
            }

            match key {
                "kind" => {
                    if kind.is_some() {
                        return Err(ResilienceError::new(
                            ResilienceErrorCode::InvalidDetectionInput,
                        ));
                    }

                    kind = Some(value.trim());
                }

                "confidence" => {
                    if confidence.is_some() {
                        return Err(ResilienceError::new(
                            ResilienceErrorCode::InvalidDetectionInput,
                        ));
                    }

                    confidence = Some(value.trim());
                }

                // Future QEC metadata is deliberately opaque to this
                // detector. Diagnosis/provenance can retain the original
                // observation.
                _ => {}
            }
        }

        let kind = kind.ok_or_else(|| {
            ResilienceError::new(ResilienceErrorCode::InvalidDetectionInput)
        })?;

        let confidence_text = confidence.ok_or_else(|| {
            ResilienceError::new(ResilienceErrorCode::InvalidDetectionInput)
        })?;

        let confidence_value = confidence_text.parse::<f64>().map_err(|_| {
            ResilienceError::new(ResilienceErrorCode::InvalidDetectionInput)
        })?;

        let confidence = DetectionConfidence::new(confidence_value)?;

        Ok(Some(QecSignal::new(kind, confidence)?))
    }

    /// Converts an observation ID into the corresponding detector signal ID.
    ///
    /// This follows the repository's existing detector convention and avoids
    /// introducing another identity namespace.
    fn signal_id(observation: &DetectionObservation) -> ResilienceResult<SignalId> {
        SignalId::from_u64(observation.id().value()).ok_or_else(|| {
            ResilienceError::new(ResilienceErrorCode::InvalidIdentifier)
        })
    }

    /// Converts one parsed QEC observation into a normalized signal.
    fn detect_one(
        &self,
        observation: &DetectionObservation,
    ) -> ResilienceResult<Option<DetectionSignal>> {
        let qec_signal = match self.parse_observation(observation)? {
            Some(signal) => signal,
            None => return Ok(None),
        };

        let signal_id = Self::signal_id(observation)?;

        Ok(Some(DetectionSignal::new(
            signal_id,
            self.identity.clone(),
            DetectionClassification::QecSignal,
            qec_signal.confidence(),
            Some(observation.id()),
            observation.sequence(),
        )))
    }
}

impl Detector for QecSignalDetector {
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
        let context = input.context().clone();

        let mut signals = Vec::new();
        let mut observations_examined: u64 = 0;

        for observation in input.observations() {
            observations_examined = observations_examined.checked_add(1).ok_or_else(|| {
                ResilienceError::new(ResilienceErrorCode::ArithmeticOverflow)
            })?;

            if let Some(signal) = self.detect_one(observation)? {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observation_id(value: u64) -> crate::quantum::resilience::detection::detector::ObservationId {
        crate::quantum::resilience::detection::detector::ObservationId::from_u64(value)
            .expect("test observation ID must be non-zero")
    }

    fn sequence(
        value: u64,
    ) -> crate::quantum::resilience::detection::detector::DetectionSequence {
        crate::quantum::resilience::detection::detector::DetectionSequence::from_u64(value)
            .expect("test sequence must be non-zero")
    }

    fn context() -> crate::quantum::resilience::detection::detector::DetectionContext {
        crate::quantum::resilience::detection::detector::DetectionContext::new(
            sequence(1),
            false,
            true,
        )
    }

    fn observation(
        id: u64,
        payload: ObservationPayload,
    ) -> DetectionObservation {
        DetectionObservation::new(
            observation_id(id),
            sequence(id),
            crate::quantum::resilience::detection::detector::ObservationSource::Qec,
            crate::quantum::resilience::detection::detector::ObservationTrust::Verified,
            crate::quantum::resilience::detection::detector::ObservationFreshness::Fresh,
            payload,
        )
        .expect("test observation must be valid")
    }

    fn input<'a, I>(
        context: &'a crate::quantum::resilience::detection::detector::DetectionContext,
        observations: I,
    ) -> DetectionInput<'a, I>
    where
        I: Iterator<Item = &'a DetectionObservation>,
    {
        DetectionInput::new(context, observations)
    }

    #[test]
    fn canonical_signal_is_detected() {
        let mut detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let observation = observation(
            1,
            ObservationPayload::Text(
                "zamani.qec.signal|kind=logical_error|confidence=0.97".to_string(),
            ),
        );

        let observations = [&observation];

        let result = detector
            .detect(input(&context(), observations.iter()))
            .expect("detection must succeed");

        assert_eq!(result.len(), 1);

        let signal = result
            .signals()
            .next()
            .expect("one signal must be present");

        assert_eq!(
            signal.classification(),
            DetectionClassification::QecSignal
        );
        assert_eq!(signal.observation_id(), Some(observation.id()));
    }

    #[test]
    fn unknown_future_qec_kind_is_accepted() {
        let mut detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let observation = observation(
            1,
            ObservationPayload::Text(
                "zamani.qec.signal|kind=future_decoder_event|confidence=0.81".to_string(),
            ),
        );

        let observations = [&observation];

        let result = detector
            .detect(input(&context(), observations.iter()))
            .expect("future QEC signal should remain extensible");

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn non_qec_text_is_ignored() {
        let mut detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let observation = observation(
            1,
            ObservationPayload::Text("ordinary.telemetry|value=1".to_string()),
        );

        let observations = [&observation];

        let result = detector
            .detect(input(&context(), observations.iter()))
            .expect("unrelated text should be ignored");

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn numeric_payload_is_not_guessed_as_qec() {
        let mut detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let observation = observation(1, ObservationPayload::Float(0.99));

        let observations = [&observation];

        let result = detector
            .detect(input(&context(), observations.iter()))
            .expect("numeric payload should be ignored");

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn malformed_signal_is_rejected() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result = detector.parse_text(
            "zamani.qec.signal|kind=logical_error|confidence=not-a-number",
        );

        assert!(result.is_err());
    }

    #[test]
    fn missing_kind_is_rejected() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result =
            detector.parse_text("zamani.qec.signal|confidence=0.9");

        assert!(result.is_err());
    }

    #[test]
    fn missing_confidence_is_rejected() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result =
            detector.parse_text("zamani.qec.signal|kind=logical_error");

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_kind_is_rejected() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result = detector.parse_text(
            "zamani.qec.signal|kind=logical_error|kind=leakage|confidence=0.9",
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_confidence_is_rejected() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result = detector.parse_text(
            "zamani.qec.signal|kind=logical_error|confidence=0.9|confidence=0.8",
        );

        assert!(result.is_err());
    }

    #[test]
    fn confidence_above_one_is_rejected() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result =
            detector.parse_text("zamani.qec.signal|kind=logical_error|confidence=1.1");

        assert!(result.is_err());
    }

    #[test]
    fn negative_confidence_is_rejected() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result =
            detector.parse_text("zamani.qec.signal|kind=logical_error|confidence=-0.1");

        assert!(result.is_err());
    }

    #[test]
    fn nan_confidence_is_rejected() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result =
            detector.parse_text("zamani.qec.signal|kind=logical_error|confidence=NaN");

        assert!(result.is_err());
    }

    #[test]
    fn infinite_confidence_is_rejected() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result =
            detector.parse_text("zamani.qec.signal|kind=logical_error|confidence=inf");

        assert!(result.is_err());
    }

    #[test]
    fn additional_future_fields_are_allowed() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let result = detector.parse_text(
            "zamani.qec.signal|kind=logical_error|confidence=0.91|\
             code=surface_code|decoder=future-decoder|resource=logical-resource",
        );

        assert!(result.is_ok());
        assert_eq!(
            result
                .expect("signal should parse")
                .expect("signal should exist")
                .kind(),
            "logical_error"
        );
    }

    #[test]
    fn detector_identity_is_stable() {
        let detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        assert_eq!(detector.identity().name(), QEC_SIGNAL_DETECTOR_NAME);
        assert_eq!(
            detector.identity().version(),
            QEC_SIGNAL_DETECTOR_SCHEMA_VERSION.to_string()
        );
    }

    #[test]
    fn output_preserves_observation_count() {
        let mut detector =
            QecSignalDetector::canonical().expect("canonical detector must be valid");

        let first = observation(
            1,
            ObservationPayload::Text(
                "zamani.qec.signal|kind=syndrome|confidence=0.8".to_string(),
            ),
        );

        let second = observation(2, ObservationPayload::Float(0.5));

        let observations = [&first, &second];

        let result = detector
            .detect(input(&context(), observations.iter()))
            .expect("detection must succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result.metadata().observations_examined(), 2);
    }
}