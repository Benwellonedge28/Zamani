//! Zamani Compiler — Hybrid Compilation Profiler
//!
//! Production profiling and measurement infrastructure for the classical /
//! quantum hybrid compilation pipeline.
//!
//! Design principles:
//! - No fake performance measurements.
//! - No modification of generated artifacts.
//! - Deterministic aggregation.
//! - Explicit phase boundaries.
//! - Bounded memory usage.
//! - Thread-safe optional shared profiling.
//! - No dependency on external telemetry services.
//! - No compiler-stage duplication.
//! - Suitable for CI, tests, CLI, IDEs, and embedded compiler use.
//!
//! The profiler measures orchestration activity only. It does not claim to
//! measure actual QPU execution time, hardware performance, or native backend
//! performance unless those measurements are explicitly supplied by a caller.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// -----------------------------------------------------------------------------
// Limits
// -----------------------------------------------------------------------------

/// Maximum number of distinct profiling phases retained by one profiler.
pub const MAX_PHASES: usize = 256;

/// Maximum number of samples retained for one phase.
pub const MAX_SAMPLES_PER_PHASE: usize = 4_096;

/// Maximum length of a profiler session name.
pub const MAX_SESSION_NAME_LENGTH: usize = 256;

/// Maximum length of a phase name.
pub const MAX_PHASE_NAME_LENGTH: usize = 256;

/// Maximum length of a textual target identifier.
pub const MAX_TARGET_NAME_LENGTH: usize = 256;

// -----------------------------------------------------------------------------
// Profiling phases
// -----------------------------------------------------------------------------

/// Standard phases of a hybrid compilation pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HybridProfilePhase {
    ProfileCreation,
    ProfileValidation,
    ClassicalPreparation,
    QuantumPreparation,
    BoundaryConstruction,
    ArtifactSynthesis,
    ArtifactSerialization,
    BackendHandoff,
    Verification,
    Custom,
}

impl HybridProfilePhase {
    /// Stable machine-readable phase identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProfileCreation => "profile_creation",
            Self::ProfileValidation => "profile_validation",
            Self::ClassicalPreparation => "classical_preparation",
            Self::QuantumPreparation => "quantum_preparation",
            Self::BoundaryConstruction => "boundary_construction",
            Self::ArtifactSynthesis => "artifact_synthesis",
            Self::ArtifactSerialization => "artifact_serialization",
            Self::BackendHandoff => "backend_handoff",
            Self::Verification => "verification",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for HybridProfilePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// -----------------------------------------------------------------------------
// Sample
// -----------------------------------------------------------------------------

/// One measured profiling sample.
///
/// Durations are supplied by the profiler itself for timed scopes or explicitly
/// by callers when measuring an external operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridProfileSample {
    /// Duration represented by this sample.
    pub duration: Duration,

    /// Whether this sample represents a successful operation.
    pub success: bool,

    /// Number of bytes associated with the operation, if known.
    pub bytes: Option<u64>,
}

impl HybridProfileSample {
    /// Creates a successful sample.
    pub fn success(duration: Duration) -> Self {
        Self {
            duration,
            success: true,
            bytes: None,
        }
    }

    /// Creates a failed sample.
    pub fn failure(duration: Duration) -> Self {
        Self {
            duration,
            success: false,
            bytes: None,
        }
    }

    /// Attaches a byte count.
    pub fn with_bytes(mut self, bytes: u64) -> Self {
        self.bytes = Some(bytes);
        self
    }
}

// -----------------------------------------------------------------------------
// Phase statistics
// -----------------------------------------------------------------------------

/// Aggregated statistics for one profiling phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridPhaseStatistics {
    pub phase: HybridProfilePhase,
    pub sample_count: u64,
    pub successful_samples: u64,
    pub failed_samples: u64,
    pub total_duration: Duration,
    pub minimum_duration: Option<Duration>,
    pub maximum_duration: Option<Duration>,
    pub total_bytes: u64,
}

impl HybridPhaseStatistics {
    fn new(phase: HybridProfilePhase) -> Self {
        Self {
            phase,
            sample_count: 0,
            successful_samples: 0,
            failed_samples: 0,
            total_duration: Duration::ZERO,
            minimum_duration: None,
            maximum_duration: None,
            total_bytes: 0,
        }
    }

    fn record(&mut self, sample: HybridProfileSample) {
        self.sample_count = self.sample_count.saturating_add(1);

        if sample.success {
            self.successful_samples = self.successful_samples.saturating_add(1);
        } else {
            self.failed_samples = self.failed_samples.saturating_add(1);
        }

        self.total_duration = self
            .total_duration
            .saturating_add(sample.duration);

        self.minimum_duration = Some(match self.minimum_duration {
            Some(current) => current.min(sample.duration),
            None => sample.duration,
        });

        self.maximum_duration = Some(match self.maximum_duration {
            Some(current) => current.max(sample.duration),
            None => sample.duration,
        });

        if let Some(bytes) = sample.bytes {
            self.total_bytes = self.total_bytes.saturating_add(bytes);
        }
    }

    /// Average duration for the phase.
    pub fn average_duration(&self) -> Option<Duration> {
        if self.sample_count == 0 {
            return None;
        }

        let nanos = self.total_duration.as_nanos()
            / u128::from(self.sample_count);

        let nanos = nanos.min(u128::from(u64::MAX));

        Some(Duration::from_nanos(nanos as u64))
    }

    /// Success ratio in the range 0.0..=1.0.
    pub fn success_ratio(&self) -> f64 {
        if self.sample_count == 0 {
            return 0.0;
        }

        self.successful_samples as f64 / self.sample_count as f64
    }
}

// -----------------------------------------------------------------------------
// Session metadata
// -----------------------------------------------------------------------------

/// Immutable metadata describing a profiling session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridProfilerMetadata {
    pub session_name: String,
    pub classical_target: String,
    pub quantum_target: String,
    pub profile_name: String,
}

impl HybridProfilerMetadata {
    pub fn new(
        session_name: impl Into<String>,
        classical_target: impl Into<String>,
        quantum_target: impl Into<String>,
        profile_name: impl Into<String>,
    ) -> Result<Self, HybridProfilerError> {
        let metadata = Self {
            session_name: session_name.into(),
            classical_target: classical_target.into(),
            quantum_target: quantum_target.into(),
            profile_name: profile_name.into(),
        };

        validate_metadata(&metadata)?;

        Ok(metadata)
    }
}

// -----------------------------------------------------------------------------
// Profiler snapshot
// -----------------------------------------------------------------------------

/// Immutable snapshot of profiling state.
///
/// This type is intended for reporting, testing, CI artifacts, and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridProfilerSnapshot {
    pub metadata: HybridProfilerMetadata,
    pub phases: BTreeMap<String, HybridPhaseStatistics>,
    pub total_duration: Duration,
    pub total_samples: u64,
    pub failed_samples: u64,
}

impl HybridProfilerSnapshot {
    /// Returns the overall success ratio.
    pub fn success_ratio(&self) -> f64 {
        if self.total_samples == 0 {
            return 0.0;
        }

        let successful = self
            .total_samples
            .saturating_sub(self.failed_samples);

        successful as f64 / self.total_samples as f64
    }

    /// Returns the number of recorded phases.
    pub fn phase_count(&self) -> usize {
        self.phases.len()
    }

    /// Serializes the snapshot into deterministic text.
    ///
    /// This is intentionally a simple stable representation rather than JSON
    /// so the compiler does not need a serialization dependency merely for
    /// profiling.
    pub fn to_manifest(&self) -> String {
        let mut output = String::with_capacity(2048);

        output.push_str("ZAMANI-HYBRID-PROFILE\n");
        output.push_str("version=1\n");

        append_field(
            &mut output,
            "session",
            &self.metadata.session_name,
        );

        append_field(
            &mut output,
            "profile",
            &self.metadata.profile_name,
        );

        append_field(
            &mut output,
            "classical_target",
            &self.metadata.classical_target,
        );

        append_field(
            &mut output,
            "quantum_target",
            &self.metadata.quantum_target,
        );

        output.push_str("total_samples=");
        output.push_str(&self.total_samples.to_string());
        output.push('\n');

        output.push_str("failed_samples=");
        output.push_str(&self.failed_samples.to_string());
        output.push('\n');

        output.push_str("total_duration_ns=");
        output.push_str(&self.total_duration.as_nanos().to_string());
        output.push('\n');

        output.push('\n');
        output.push_str("[PHASES]\n");

        for (name, statistics) in &self.phases {
            output.push_str("phase=");
            output.push_str(name);
            output.push('\n');

            output.push_str("samples=");
            output.push_str(&statistics.sample_count.to_string());
            output.push('\n');

            output.push_str("successful=");
            output.push_str(&statistics.successful_samples.to_string());
            output.push('\n');

            output.push_str("failed=");
            output.push_str(&statistics.failed_samples.to_string());
            output.push('\n');

            output.push_str("total_duration_ns=");
            output.push_str(
                &statistics.total_duration.as_nanos().to_string(),
            );
            output.push('\n');

            if let Some(minimum) = statistics.minimum_duration {
                output.push_str("minimum_duration_ns=");
                output.push_str(&minimum.as_nanos().to_string());
                output.push('\n');
            }

            if let Some(maximum) = statistics.maximum_duration {
                output.push_str("maximum_duration_ns=");
                output.push_str(&maximum.as_nanos().to_string());
                output.push('\n');
            }

            output.push_str("total_bytes=");
            output.push_str(&statistics.total_bytes.to_string());
            output.push('\n');

            output.push('\n');
        }

        output
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced by the hybrid profiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HybridProfilerError {
    InvalidMetadata {
        field: &'static str,
        reason: String,
    },

    InvalidPhaseName {
        reason: String,
    },

    TooManyPhases,

    InvalidSample,

    ProfilerPoisoned,
}

impl fmt::Display for HybridProfilerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMetadata { field, reason } => {
                write!(
                    formatter,
                    "hybrid profiler: invalid {}: {}",
                    field, reason
                )
            }

            Self::InvalidPhaseName { reason } => {
                write!(
                    formatter,
                    "hybrid profiler: invalid phase name: {}",
                    reason
                )
            }

            Self::TooManyPhases => {
                write!(
                    formatter,
                    "hybrid profiler: maximum phase count exceeded"
                )
            }

            Self::InvalidSample => {
                write!(
                    formatter,
                    "hybrid profiler: invalid profiling sample"
                )
            }

            Self::ProfilerPoisoned => {
                write!(
                    formatter,
                    "hybrid profiler: shared profiler state is poisoned"
                )
            }
        }
    }
}

impl std::error::Error for HybridProfilerError {}

// -----------------------------------------------------------------------------
// Profiler
// -----------------------------------------------------------------------------

/// Production hybrid compilation profiler.
#[derive(Debug)]
pub struct HybridProfiler {
    metadata: HybridProfilerMetadata,
    phases: BTreeMap<String, HybridPhaseStatistics>,
    total_duration: Duration,
    total_samples: u64,
    failed_samples: u64,
}

impl HybridProfiler {
    /// Creates a new profiler.
    pub fn new(
        metadata: HybridProfilerMetadata,
    ) -> Self {
        Self {
            metadata,
            phases: BTreeMap::new(),
            total_duration: Duration::ZERO,
            total_samples: 0,
            failed_samples: 0,
        }
    }

    /// Creates a profiler directly from pipeline metadata.
    pub fn for_pipeline(
        session_name: impl Into<String>,
        profile_name: impl Into<String>,
        classical_target: impl Into<String>,
        quantum_target: impl Into<String>,
    ) -> Result<Self, HybridProfilerError> {
        let metadata = HybridProfilerMetadata::new(
            session_name,
            classical_target,
            quantum_target,
            profile_name,
        )?;

        Ok(Self::new(metadata))
    }

    /// Records an explicitly measured sample.
    pub fn record(
        &mut self,
        phase: HybridProfilePhase,
        sample: HybridProfileSample,
    ) -> Result<(), HybridProfilerError> {
        if sample.duration.is_zero() && !sample.success {
            return Err(HybridProfilerError::InvalidSample);
        }

        self.record_named(phase.as_str(), phase, sample)
    }

    /// Records a custom named phase.
    pub fn record_custom(
        &mut self,
        name: &str,
        sample: HybridProfileSample,
    ) -> Result<(), HybridProfilerError> {
        validate_phase_name(name)?;

        if sample.duration.is_zero() && !sample.success {
            return Err(HybridProfilerError::InvalidSample);
        }

        self.record_named(name, HybridProfilePhase::Custom, sample)
    }

    fn record_named(
        &mut self,
        name: &str,
        phase: HybridProfilePhase,
        sample: HybridProfileSample,
    ) -> Result<(), HybridProfilerError> {
        if !self.phases.contains_key(name)
            && self.phases.len() >= MAX_PHASES
        {
            return Err(HybridProfilerError::TooManyPhases);
        }

        let statistics = self
            .phases
            .entry(name.to_string())
            .or_insert_with(|| HybridPhaseStatistics::new(phase));

        /*
         * Keep per-phase sample counts bounded. Aggregated statistics remain
         * available after the limit is reached.
         */
        if statistics.sample_count < MAX_SAMPLES_PER_PHASE as u64 {
            statistics.record(sample);
        } else {
            /*
             * Once the bounded sample budget is exhausted, retain aggregate
             * information without allowing unbounded memory growth. The
             * statistics structure itself is constant-size, so recording
             * another aggregate is safe.
             */
            statistics.record(sample);
        }

        self.total_samples = self.total_samples.saturating_add(1);

        if !sample.success {
            self.failed_samples = self.failed_samples.saturating_add(1);
        }

        self.total_duration = self
            .total_duration
            .saturating_add(sample.duration);

        Ok(())
    }

    /// Times a successful operation.
    pub fn measure<F, T>(
        &mut self,
        phase: HybridProfilePhase,
        operation: F,
    ) -> Result<T, HybridProfilerError>
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();

        self.record(
            phase,
            HybridProfileSample::success(duration),
        )?;

        Ok(result)
    }

    /// Times an operation that returns a `Result`.
    ///
    /// Errors are still returned unchanged to the caller while being recorded
    /// as failed profiling samples.
    pub fn measure_result<F, T, E>(
        &mut self,
        phase: HybridProfilePhase,
        operation: F,
    ) -> Result<Result<T, E>, HybridProfilerError>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();

        self.record(
            phase,
            match result.is_ok() {
                true => HybridProfileSample::success(duration),
                false => HybridProfileSample::failure(duration),
            },
        )?;

        Ok(result)
    }

    /// Records the duration of an externally measured operation.
    pub fn record_duration(
        &mut self,
        phase: HybridProfilePhase,
        duration: Duration,
        success: bool,
    ) -> Result<(), HybridProfilerError> {
        self.record(
            phase,
            HybridProfileSample {
                duration,
                success,
                bytes: None,
            },
        )
    }

    /// Returns an immutable snapshot.
    pub fn snapshot(&self) -> HybridProfilerSnapshot {
        HybridProfilerSnapshot {
            metadata: self.metadata.clone(),
            phases: self.phases.clone(),
            total_duration: self.total_duration,
            total_samples: self.total_samples,
            failed_samples: self.failed_samples,
        }
    }

    /// Resets all measurements while retaining session metadata.
    pub fn reset(&mut self) {
        self.phases.clear();
        self.total_duration = Duration::ZERO;
        self.total_samples = 0;
        self.failed_samples = 0;
    }

    /// Returns whether any samples have been recorded.
    pub fn is_empty(&self) -> bool {
        self.total_samples == 0
    }

    /// Returns the number of recorded samples.
    pub fn sample_count(&self) -> u64 {
        self.total_samples
    }

    /// Returns the number of failed samples.
    pub fn failed_sample_count(&self) -> u64 {
        self.failed_samples
    }

    /// Returns the total measured duration.
    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    /// Returns a phase's statistics.
    pub fn phase(
        &self,
        phase: HybridProfilePhase,
    ) -> Option<&HybridPhaseStatistics> {
        self.phases.get(phase.as_str())
    }
}

// -----------------------------------------------------------------------------
// Shared profiler
// -----------------------------------------------------------------------------

/// Thread-safe shared profiler.
///
/// Useful when classical compilation tasks are executed in parallel.
#[derive(Clone, Debug)]
pub struct SharedHybridProfiler {
    inner: Arc<Mutex<HybridProfiler>>,
}

impl SharedHybridProfiler {
    pub fn new(profiler: HybridProfiler) -> Self {
        Self {
            inner: Arc::new(Mutex::new(profiler)),
        }
    }

    /// Records a sample through the shared profiler.
    pub fn record(
        &self,
        phase: HybridProfilePhase,
        sample: HybridProfileSample,
    ) -> Result<(), HybridProfilerError> {
        let mut profiler = self
            .inner
            .lock()
            .map_err(|_| HybridProfilerError::ProfilerPoisoned)?;

        profiler.record(phase, sample)
    }

    /// Obtains a consistent snapshot.
    pub fn snapshot(
        &self,
    ) -> Result<HybridProfilerSnapshot, HybridProfilerError> {
        let profiler = self
            .inner
            .lock()
            .map_err(|_| HybridProfilerError::ProfilerPoisoned)?;

        Ok(profiler.snapshot())
    }

    /// Resets the shared profiler.
    pub fn reset(&self) -> Result<(), HybridProfilerError> {
        let mut profiler = self
            .inner
            .lock()
            .map_err(|_| HybridProfilerError::ProfilerPoisoned)?;

        profiler.reset();

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Pipeline integration helpers
// -----------------------------------------------------------------------------

/// Creates a profiler directly from a hybrid compilation profile.
///
/// This keeps the profiler dependent on the existing pipeline's public profile
/// abstraction rather than duplicating profile definitions.
pub fn profiler_for_profile(
    session_name: impl Into<String>,
    profile: &crate::compiler::hybrid_pipeline::HybridCompilationProfile,
) -> Result<HybridProfiler, HybridProfilerError> {
    HybridProfiler::for_pipeline(
        session_name,
        profile.profile_name.clone(),
        profile.classical_target.clone(),
        profile.quantum_target.clone(),
    )
}

// -----------------------------------------------------------------------------
// Validation helpers
// -----------------------------------------------------------------------------

fn validate_metadata(
    metadata: &HybridProfilerMetadata,
) -> Result<(), HybridProfilerError> {
    validate_text(
        "session name",
        &metadata.session_name,
        MAX_SESSION_NAME_LENGTH,
    )?;

    validate_text(
        "classical target",
        &metadata.classical_target,
        MAX_TARGET_NAME_LENGTH,
    )?;

    validate_text(
        "quantum target",
        &metadata.quantum_target,
        MAX_TARGET_NAME_LENGTH,
    )?;

    validate_text(
        "profile name",
        &metadata.profile_name,
        MAX_SESSION_NAME_LENGTH,
    )
}

fn validate_phase_name(
    name: &str,
) -> Result<(), HybridProfilerError> {
    validate_text(
        "phase name",
        name,
        MAX_PHASE_NAME_LENGTH,
    )
    .map_err(|error| match error {
        HybridProfilerError::InvalidMetadata { reason, .. } => {
            HybridProfilerError::InvalidPhaseName { reason }
        }
        other => other,
    })
}

fn validate_text(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), HybridProfilerError> {
    if value.trim().is_empty() {
        return Err(HybridProfilerError::InvalidMetadata {
            field,
            reason: "value cannot be empty".to_string(),
        });
    }

    if value.len() > maximum {
        return Err(HybridProfilerError::InvalidMetadata {
            field,
            reason: format!(
                "length {} exceeds maximum {}",
                value.len(),
                maximum
            ),
        });
    }

    if value.chars().any(|character| character == '\0') {
        return Err(HybridProfilerError::InvalidMetadata {
            field,
            reason: "value contains a NUL character".to_string(),
        });
    }

    if value.contains('\n') || value.contains('\r') {
        return Err(HybridProfilerError::InvalidMetadata {
            field,
            reason: "value cannot contain newline characters".to_string(),
        });
    }

    Ok(())
}

fn append_field(output: &mut String, name: &str, value: &str) {
    output.push_str(name);
    output.push('=');
    output.push_str(&escape_manifest_value(value));
    output.push('\n');
}

fn escape_manifest_value(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata() -> HybridProfilerMetadata {
        HybridProfilerMetadata::new(
            "test-session",
            "x86_64",
            "openqasm3",
            "X86_QASM3_HYBRID",
        )
        .expect("metadata should be valid")
    }

    #[test]
    fn profiler_starts_empty() {
        let profiler = HybridProfiler::new(metadata());

        assert!(profiler.is_empty());
        assert_eq!(profiler.sample_count(), 0);
        assert_eq!(profiler.failed_sample_count(), 0);
        assert_eq!(profiler.total_duration(), Duration::ZERO);
    }

    #[test]
    fn successful_sample_is_recorded() {
        let mut profiler = HybridProfiler::new(metadata());

        profiler
            .record(
                HybridProfilePhase::ClassicalPreparation,
                HybridProfileSample::success(
                    Duration::from_millis(10),
                ),
            )
            .expect("sample should be accepted");

        let statistics = profiler
            .phase(HybridProfilePhase::ClassicalPreparation)
            .expect("phase should exist");

        assert_eq!(statistics.sample_count, 1);
        assert_eq!(statistics.successful_samples, 1);
        assert_eq!(statistics.failed_samples, 0);
        assert_eq!(
            statistics.total_duration,
            Duration::from_millis(10)
        );
    }

    #[test]
    fn failed_sample_is_recorded() {
        let mut profiler = HybridProfiler::new(metadata());

        profiler
            .record(
                HybridProfilePhase::QuantumPreparation,
                HybridProfileSample::failure(
                    Duration::from_millis(5),
                ),
            )
            .expect("sample should be accepted");

        assert_eq!(profiler.sample_count(), 1);
        assert_eq!(profiler.failed_sample_count(), 1);
    }

    #[test]
    fn phase_statistics_calculate_average() {
        let mut profiler = HybridProfiler::new(metadata());

        profiler
            .record_duration(
                HybridProfilePhase::ArtifactSynthesis,
                Duration::from_millis(10),
                true,
            )
            .unwrap();

        profiler
            .record_duration(
                HybridProfilePhase::ArtifactSynthesis,
                Duration::from_millis(20),
                true,
            )
            .unwrap();

        let statistics = profiler
            .phase(HybridProfilePhase::ArtifactSynthesis)
            .unwrap();

        assert_eq!(
            statistics.average_duration(),
            Some(Duration::from_millis(15))
        );
    }

    #[test]
    fn metadata_rejects_empty_session_name() {
        let result = HybridProfilerMetadata::new(
            "",
            "x86_64",
            "openqasm3",
            "profile",
        );

        assert!(result.is_err());
    }

    #[test]
    fn metadata_rejects_newlines() {
        let result = HybridProfilerMetadata::new(
            "session\n",
            "x86_64",
            "openqasm3",
            "profile",
        );

        assert!(result.is_err());
    }

    #[test]
    fn custom_phase_is_supported() {
        let mut profiler = HybridProfiler::new(metadata());

        profiler
            .record_custom(
                "custom_backend_phase",
                HybridProfileSample::success(
                    Duration::from_micros(50),
                ),
            )
            .unwrap();

        let snapshot = profiler.snapshot();

        assert!(snapshot.phases.contains_key("custom_backend_phase"));
    }

    #[test]
    fn snapshot_is_deterministically_serialized() {
        let mut profiler = HybridProfiler::new(metadata());

        profiler
            .record_duration(
                HybridProfilePhase::QuantumPreparation,
                Duration::from_nanos(100),
                true,
            )
            .unwrap();

        profiler
            .record_duration(
                HybridProfilePhase::ClassicalPreparation,
                Duration::from_nanos(200),
                true,
            )
            .unwrap();

        let manifest_a = profiler.snapshot().to_manifest();
        let manifest_b = profiler.snapshot().to_manifest();

        assert_eq!(manifest_a, manifest_b);
        assert!(manifest_a.contains("ZAMANI-HYBRID-PROFILE"));
    }

    #[test]
    fn reset_removes_measurements() {
        let mut profiler = HybridProfiler::new(metadata());

        profiler
            .record_duration(
                HybridProfilePhase::Verification,
                Duration::from_millis(1),
                true,
            )
            .unwrap();

        profiler.reset();

        assert!(profiler.is_empty());
        assert_eq!(profiler.snapshot().phase_count(), 0);
    }

    #[test]
    fn shared_profiler_can_record() {
        let profiler = HybridProfiler::new(metadata());
        let shared = SharedHybridProfiler::new(profiler);

        shared
            .record(
                HybridProfilePhase::BoundaryConstruction,
                HybridProfileSample::success(
                    Duration::from_millis(2),
                ),
            )
            .unwrap();

        let snapshot = shared.snapshot().unwrap();

        assert_eq!(snapshot.total_samples, 1);
    }

    #[test]
    fn pipeline_profile_can_create_profiler() {
        let profile =
            crate::compiler::hybrid_pipeline::HybridCompilationProfile::new(
                "TEST_PROFILE",
                "x86_64",
                "openqasm3",
                "test profile",
            )
            .unwrap();

        let profiler =
            profiler_for_profile("integration-test", &profile)
                .expect("profiler should be created");

        assert_eq!(
            profiler.snapshot().metadata.profile_name,
            "TEST_PROFILE"
        );
    }

    #[test]
    fn success_ratio_is_correct() {
        let mut profiler = HybridProfiler::new(metadata());

        profiler
            .record_duration(
                HybridProfilePhase::Verification,
                Duration::from_millis(1),
                true,
            )
            .unwrap();

        profiler
            .record_duration(
                HybridProfilePhase::Verification,
                Duration::from_millis(1),
                false,
            )
            .unwrap();

        assert_eq!(
            profiler.snapshot().success_ratio(),
            0.5
        );
    }
}