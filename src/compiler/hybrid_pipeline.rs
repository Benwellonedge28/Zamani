//! Zamani Compiler — Quantum/Classical Hybrid Pipeline
//!
//! This module defines the production boundary between classical compilation
//! and quantum compilation.
//!
//! The hybrid pipeline does NOT fabricate native assembly or quantum machine
//! instructions. Instead, it produces a deterministic hybrid execution
//! artifact describing:
//!
//!   classical control
//!        |
//!        v
//!   quantum invocation boundary
//!        |
//!        v
//!   quantum kernel
//!        |
//!        v
//!   classical synchronization
//!
//! Target-specific code generation remains the responsibility of the
//! corresponding compiler backends.
//!
//! Design goals:
//! - deterministic artifact generation
//! - explicit classical/quantum target validation
//! - structured errors
//! - no fake backend instructions
//! - explicit synchronization boundaries
//! - bounded metadata
//! - stable serialization
//! - compatibility with existing callers
//! - testable without external quantum or native toolchains

use std::fmt;

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

const MAX_PROFILE_NAME_LENGTH: usize = 256;
const MAX_TARGET_NAME_LENGTH: usize = 256;
const MAX_DESCRIPTION_LENGTH: usize = 4096;
const MAX_MODULE_NAME_LENGTH: usize = 256;

// -----------------------------------------------------------------------------
// Target types
// -----------------------------------------------------------------------------

/// Supported classical target families.
///
/// This enum intentionally represents target families rather than pretending
/// to implement every individual CPU architecture inside the hybrid pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassicalTarget {
    X86_64,
    Aarch64,
    RiscV64,
    Wasm32,
    Wasm64,
    ZamaniPortable,
    Custom(String),
}

impl ClassicalTarget {
    /// Returns the stable target identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::X86_64 => "x86_64",
            Self::Aarch64 => "aarch64",
            Self::RiscV64 => "riscv64",
            Self::Wasm32 => "wasm32",
            Self::Wasm64 => "wasm64",
            Self::ZamaniPortable => "zamani-portable",
            Self::Custom(value) => value.as_str(),
        }
    }

    fn validate(&self) -> Result<(), HybridPipelineError> {
        validate_identifier("classical target", self.as_str(), MAX_TARGET_NAME_LENGTH)
    }
}

/// Supported quantum target families.
///
/// These are interface identifiers. Actual circuit lowering and device
/// emission belong to the quantum backend layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuantumTarget {
    OpenQasm3,
    Qir,
    IonTrap,
    Superconducting,
    NeutralAtom,
    Photonic,
    Custom(String),
}

impl QuantumTarget {
    /// Returns the stable target identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::OpenQasm3 => "openqasm3",
            Self::Qir => "qir",
            Self::IonTrap => "ion-trap",
            Self::Superconducting => "superconducting",
            Self::NeutralAtom => "neutral-atom",
            Self::Photonic => "photonic",
            Self::Custom(value) => value.as_str(),
        }
    }

    fn validate(&self) -> Result<(), HybridPipelineError> {
        validate_identifier("quantum target", self.as_str(), MAX_TARGET_NAME_LENGTH)
    }
}

// -----------------------------------------------------------------------------
// Synchronization
// -----------------------------------------------------------------------------

/// Boundary between classical and quantum execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynchronizationMode {
    /// Classical code invokes a quantum kernel and waits for completion.
    Synchronous,

    /// Quantum execution may continue while classical execution progresses.
    Asynchronous,

    /// Explicit synchronization is required before reading quantum results.
    Barrier,
}

impl SynchronizationMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Synchronous => "synchronous",
            Self::Asynchronous => "asynchronous",
            Self::Barrier => "barrier",
        }
    }
}

// -----------------------------------------------------------------------------
// Pipeline profile
// -----------------------------------------------------------------------------

/// Configuration describing one classical/quantum compilation combination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridCompilationProfile {
    pub profile_name: String,
    pub classical_target: String,
    pub quantum_target: String,
    pub description: String,
    pub synchronization: SynchronizationMode,
}

impl HybridCompilationProfile {
    /// Creates a profile using synchronous execution.
    pub fn new(
        profile_name: impl Into<String>,
        classical_target: impl Into<String>,
        quantum_target: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, HybridPipelineError> {
        let profile = Self {
            profile_name: profile_name.into(),
            classical_target: classical_target.into(),
            quantum_target: quantum_target.into(),
            description: description.into(),
            synchronization: SynchronizationMode::Synchronous,
        };

        profile.validate()?;

        Ok(profile)
    }

    /// Creates a profile with an explicit synchronization mode.
    pub fn with_synchronization(
        profile_name: impl Into<String>,
        classical_target: impl Into<String>,
        quantum_target: impl Into<String>,
        description: impl Into<String>,
        synchronization: SynchronizationMode,
    ) -> Result<Self, HybridPipelineError> {
        let profile = Self {
            profile_name: profile_name.into(),
            classical_target: classical_target.into(),
            quantum_target: quantum_target.into(),
            description: description.into(),
            synchronization,
        };

        profile.validate()?;

        Ok(profile)
    }

    fn validate(&self) -> Result<(), HybridPipelineError> {
        validate_identifier(
            "profile name",
            &self.profile_name,
            MAX_PROFILE_NAME_LENGTH,
        )?;

        validate_identifier(
            "classical target",
            &self.classical_target,
            MAX_TARGET_NAME_LENGTH,
        )?;

        validate_identifier(
            "quantum target",
            &self.quantum_target,
            MAX_TARGET_NAME_LENGTH,
        )?;

        validate_text(
            "profile description",
            &self.description,
            MAX_DESCRIPTION_LENGTH,
        )
    }
}

// -----------------------------------------------------------------------------
// Artifact
// -----------------------------------------------------------------------------

/// A deterministic hybrid compilation artifact.
///
/// This is intentionally an intermediate artifact rather than pretending to
/// be a native executable. Native/quantum backends consume this information
/// and produce their respective target artifacts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridArtifact {
    pub module_name: String,
    pub profile_name: String,
    pub classical_target: String,
    pub quantum_target: String,
    pub synchronization: SynchronizationMode,
    pub manifest: String,
}

impl HybridArtifact {
    /// Serializes the artifact into its stable textual representation.
    pub fn as_bytes(&self) -> &[u8] {
        self.manifest.as_bytes()
    }

    /// Returns the artifact format identifier.
    pub fn format(&self) -> &'static str {
        "zamani-hybrid-v1"
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced by the hybrid compilation pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HybridPipelineError {
    EmptyModuleName,
    InvalidIdentifier {
        field: &'static str,
        reason: String,
    },
    ProfileValidation(String),
}

impl fmt::Display for HybridPipelineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyModuleName => {
                write!(formatter, "hybrid pipeline: module name cannot be empty")
            }

            Self::InvalidIdentifier { field, reason } => {
                write!(
                    formatter,
                    "hybrid pipeline: invalid {}: {}",
                    field, reason
                )
            }

            Self::ProfileValidation(message) => {
                write!(
                    formatter,
                    "hybrid pipeline: invalid compilation profile: {}",
                    message
                )
            }
        }
    }
}

impl std::error::Error for HybridPipelineError {}

// -----------------------------------------------------------------------------
// Orchestrator
// -----------------------------------------------------------------------------

/// Production hybrid compilation orchestrator.
///
/// The orchestrator owns profile validation and artifact construction. It does
/// not duplicate classical or quantum backend implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridPipelineOrchestrator {
    pub profile: HybridCompilationProfile,
}

impl HybridPipelineOrchestrator {
    /// Creates a hybrid pipeline using synchronous execution.
    ///
    /// This constructor is retained for compatibility with the original
    /// `hybrid_pipeline.rs` API.
    ///
    /// Invalid profiles are represented by a panic-free fallback error
    /// boundary through `try_new`; callers requiring validation should use
    /// `try_new`.
    pub fn new(
        profile_name: &str,
        classical_target: &str,
        quantum_target: &str,
        description: &str,
    ) -> Self {
        Self {
            profile: HybridCompilationProfile {
                profile_name: profile_name.to_string(),
                classical_target: classical_target.to_string(),
                quantum_target: quantum_target.to_string(),
                description: description.to_string(),
                synchronization: SynchronizationMode::Synchronous,
            },
        }
    }

    /// Fallible production constructor.
    pub fn try_new(
        profile_name: &str,
        classical_target: &str,
        quantum_target: &str,
        description: &str,
    ) -> Result<Self, HybridPipelineError> {
        Ok(Self {
            profile: HybridCompilationProfile::new(
                profile_name,
                classical_target,
                quantum_target,
                description,
            )?,
        })
    }

    /// Creates an orchestrator from an existing profile.
    pub fn from_profile(
        profile: HybridCompilationProfile,
    ) -> Result<Self, HybridPipelineError> {
        profile.validate()?;

        Ok(Self { profile })
    }

    /// Validates the configured pipeline.
    pub fn validate(&self) -> Result<(), HybridPipelineError> {
        self.profile.validate()
    }

    /// Produces the deterministic hybrid compilation artifact.
    pub fn synthesize(
        &self,
        module_name: &str,
    ) -> Result<HybridArtifact, HybridPipelineError> {
        self.validate()?;

        validate_module_name(module_name)?;

        let manifest = build_manifest(module_name, &self.profile);

        Ok(HybridArtifact {
            module_name: module_name.to_string(),
            profile_name: self.profile.profile_name.clone(),
            classical_target: self.profile.classical_target.clone(),
            quantum_target: self.profile.quantum_target.clone(),
            synchronization: self.profile.synchronization,
            manifest,
        })
    }

    /// Compatibility API used by older callers.
    ///
    /// Unlike the old implementation, this method no longer emits fake CPU or
    /// quantum instructions. It returns the canonical hybrid artifact
    /// manifest.
    pub fn synthesize_hybrid_binary(&self, module_name: &str) -> String {
        match self.synthesize(module_name) {
            Ok(artifact) => artifact.manifest,
            Err(error) => {
                // Compatibility requires a String return type. The error is
                // represented explicitly rather than silently generating an
                // invalid artifact.
                format!("HYBRID_COMPILATION_ERROR: {}", error)
            }
        }
    }

    /// Produces the artifact as bytes.
    pub fn compile(
        &self,
        module_name: &str,
    ) -> Result<Vec<u8>, HybridPipelineError> {
        Ok(self.synthesize(module_name)?.as_bytes().to_vec())
    }

    /// Returns whether the configured classical and quantum targets are
    /// syntactically valid.
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

// -----------------------------------------------------------------------------
// Manifest generation
// -----------------------------------------------------------------------------

fn build_manifest(
    module_name: &str,
    profile: &HybridCompilationProfile,
) -> String {
    let mut output = String::with_capacity(1024);

    output.push_str("ZAMANI-HYBRID-ARTIFACT\n");
    output.push_str("version=1\n");
    output.push_str("format=zamani-hybrid-v1\n");
    output.push_str("module=");
    output.push_str(module_name);
    output.push('\n');

    output.push_str("profile=");
    output.push_str(&profile.profile_name);
    output.push('\n');

    output.push_str("classical_target=");
    output.push_str(&profile.classical_target);
    output.push('\n');

    output.push_str("quantum_target=");
    output.push_str(&profile.quantum_target);
    output.push('\n');

    output.push_str("synchronization=");
    output.push_str(profile.synchronization.as_str());
    output.push('\n');

    output.push_str("description=");
    output.push_str(&escape_manifest_value(&profile.description));
    output.push('\n');

    output.push_str("\n[CLASSICAL_CONTROL]\n");
    output.push_str("role=control\n");
    output.push_str("target=");
    output.push_str(&profile.classical_target);
    output.push('\n');

    output.push_str("\n[QUANTUM_KERNEL]\n");
    output.push_str("role=accelerator\n");
    output.push_str("target=");
    output.push_str(&profile.quantum_target);
    output.push('\n');

    output.push_str("\n[QUANTUM_CLASSICAL_BOUNDARY]\n");
    output.push_str("synchronization=");
    output.push_str(profile.synchronization.as_str());
    output.push('\n');

    output.push_str("\n[BACKEND_REQUIREMENTS]\n");
    output.push_str("classical_backend_required=true\n");
    output.push_str("quantum_backend_required=true\n");
    output.push_str("native_instructions_embedded=false\n");
    output.push_str("quantum_instructions_embedded=false\n");

    output
}

fn escape_manifest_value(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

// -----------------------------------------------------------------------------
// Validation helpers
// -----------------------------------------------------------------------------

fn validate_module_name(name: &str) -> Result<(), HybridPipelineError> {
    if name.trim().is_empty() {
        return Err(HybridPipelineError::EmptyModuleName);
    }

    validate_identifier("module name", name, MAX_MODULE_NAME_LENGTH)
}

fn validate_identifier(
    field: &'static str,
    value: &str,
    max_length: usize,
) -> Result<(), HybridPipelineError> {
    if value.trim().is_empty() {
        return Err(HybridPipelineError::InvalidIdentifier {
            field,
            reason: "value cannot be empty".to_string(),
        });
    }

    if value.len() > max_length {
        return Err(HybridPipelineError::InvalidIdentifier {
            field,
            reason: format!(
                "length {} exceeds maximum {}",
                value.len(),
                max_length
            ),
        });
    }

    if value.chars().any(|character| character == '\0') {
        return Err(HybridPipelineError::InvalidIdentifier {
            field,
            reason: "value contains a NUL character".to_string(),
        });
    }

    if value.contains('\n') || value.contains('\r') {
        return Err(HybridPipelineError::InvalidIdentifier {
            field,
            reason: "value cannot contain newline characters".to_string(),
        });
    }

    Ok(())
}

fn validate_text(
    field: &'static str,
    value: &str,
    max_length: usize,
) -> Result<(), HybridPipelineError> {
    if value.len() > max_length {
        return Err(HybridPipelineError::InvalidIdentifier {
            field,
            reason: format!(
                "length {} exceeds maximum {}",
                value.len(),
                max_length
            ),
        });
    }

    if value.chars().any(|character| character == '\0') {
        return Err(HybridPipelineError::InvalidIdentifier {
            field,
            reason: "value contains a NUL character".to_string(),
        });
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_profile() {
        let profile = HybridCompilationProfile::new(
            "default",
            "x86_64",
            "openqasm3",
            "production hybrid profile",
        )
        .expect("profile should be valid");

        assert_eq!(profile.profile_name, "default");
        assert_eq!(
            profile.synchronization,
            SynchronizationMode::Synchronous
        );
    }

    #[test]
    fn rejects_empty_profile_name() {
        let result = HybridCompilationProfile::new(
            "",
            "x86_64",
            "openqasm3",
            "test",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_classical_target() {
        let result = HybridCompilationProfile::new(
            "profile",
            "",
            "openqasm3",
            "test",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_quantum_target() {
        let result = HybridCompilationProfile::new(
            "profile",
            "x86_64",
            "",
            "test",
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_module_name() {
        let orchestrator = HybridPipelineOrchestrator::new(
            "profile",
            "x86_64",
            "openqasm3",
            "test",
        );

        assert_eq!(
            orchestrator.synthesize(""),
            Err(HybridPipelineError::EmptyModuleName)
        );
    }

    #[test]
    fn produces_deterministic_artifact() {
        let orchestrator = HybridPipelineOrchestrator::try_new(
            "production",
            "x86_64",
            "openqasm3",
            "hybrid compilation",
        )
        .expect("valid orchestrator");

        let first = orchestrator
            .synthesize("example")
            .expect("first synthesis");

        let second = orchestrator
            .synthesize("example")
            .expect("second synthesis");

        assert_eq!(first, second);
    }

    #[test]
    fn artifact_contains_both_targets() {
        let orchestrator = HybridPipelineOrchestrator::try_new(
            "production",
            "aarch64",
            "qir",
            "ARM plus QIR",
        )
        .expect("valid orchestrator");

        let artifact = orchestrator
            .synthesize("program")
            .expect("synthesis should succeed");

        assert!(artifact.manifest.contains("classical_target=aarch64"));
        assert!(artifact.manifest.contains("quantum_target=qir"));
    }

    #[test]
    fn artifact_does_not_claim_to_contain_native_instructions() {
        let orchestrator = HybridPipelineOrchestrator::try_new(
            "production",
            "riscv64",
            "ion-trap",
            "RISC-V plus trapped ion",
        )
        .expect("valid orchestrator");

        let artifact = orchestrator
            .synthesize("program")
            .expect("synthesis should succeed");

        assert!(
            artifact
                .manifest
                .contains("native_instructions_embedded=false")
        );

        assert!(
            artifact
                .manifest
                .contains("quantum_instructions_embedded=false")
        );
    }

    #[test]
    fn asynchronous_profile_is_preserved() {
        let profile = HybridCompilationProfile::with_synchronization(
            "async",
            "x86_64",
            "qir",
            "asynchronous execution",
            SynchronizationMode::Asynchronous,
        )
        .expect("profile should be valid");

        assert_eq!(
            profile.synchronization,
            SynchronizationMode::Asynchronous
        );
    }

    #[test]
    fn barrier_profile_is_preserved() {
        let profile = HybridCompilationProfile::with_synchronization(
            "barrier",
            "x86_64",
            "qir",
            "barrier execution",
            SynchronizationMode::Barrier,
        )
        .expect("profile should be valid");

        assert_eq!(
            profile.synchronization,
            SynchronizationMode::Barrier
        );
    }

    #[test]
    fn compatibility_api_returns_artifact() {
        let orchestrator = HybridPipelineOrchestrator::new(
            "compatibility",
            "x86_64",
            "openqasm3",
            "compatibility test",
        );

        let output = orchestrator.synthesize_hybrid_binary("main");

        assert!(output.contains("ZAMANI-HYBRID-ARTIFACT"));
        assert!(output.contains("module=main"));
    }

    #[test]
    fn invalid_compatibility_profile_returns_explicit_error() {
        let orchestrator = HybridPipelineOrchestrator::new(
            "",
            "x86_64",
            "openqasm3",
            "invalid",
        );

        let output = orchestrator.synthesize_hybrid_binary("main");

        assert!(output.starts_with("HYBRID_COMPILATION_ERROR:"));
    }

    #[test]
    fn compile_returns_bytes() {
        let orchestrator = HybridPipelineOrchestrator::try_new(
            "production",
            "wasm32",
            "qir",
            "portable hybrid compilation",
        )
        .expect("valid orchestrator");

        let bytes = orchestrator
            .compile("main")
            .expect("compilation should succeed");

        assert!(!bytes.is_empty());
        assert_eq!(
            std::str::from_utf8(&bytes)
                .expect("artifact must be UTF-8")
                .lines()
                .next(),
            Some("ZAMANI-HYBRID-ARTIFACT")
        );
    }

    #[test]
    fn target_types_have_stable_names() {
        assert_eq!(ClassicalTarget::X86_64.as_str(), "x86_64");
        assert_eq!(ClassicalTarget::Aarch64.as_str(), "aarch64");
        assert_eq!(ClassicalTarget::RiscV64.as_str(), "riscv64");
        assert_eq!(QuantumTarget::OpenQasm3.as_str(), "openqasm3");
        assert_eq!(QuantumTarget::Qir.as_str(), "qir");
        assert_eq!(QuantumTarget::IonTrap.as_str(), "ion-trap");
    }

    #[test]
    fn manifest_escapes_description_newlines() {
        let orchestrator = HybridPipelineOrchestrator::new(
            "profile",
            "x86_64",
            "qir",
            "line one\nline two",
        );

        let output = orchestrator.synthesize("main");

        assert!(output.is_ok());

        let artifact = output.expect("artifact should exist");

        assert!(artifact.manifest.contains("line one\\nline two"));
    }

    #[test]
    fn profile_validation_is_available_without_compilation() {
        let orchestrator = HybridPipelineOrchestrator::new(
            "production",
            "x86_64",
            "qir",
            "test",
        );

        assert!(orchestrator.is_valid());
    }
}