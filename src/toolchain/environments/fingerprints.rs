
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Environment Fingerprints
//!
//! Path: src/toolchain/environments/fingerprints.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Coordinates reproducible environment identity across the Zamani
//! toolchain by reusing the canonical compiler, backend and ZUR
//! fingerprint and environment contracts.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Toolchain-facing environment fingerprint composition.
//! - External-toolchain fingerprint adaptation.
//! - Platform-observation fingerprint composition.
//! - Separation of compilation identity and host observation identity.
//! - Explicit construction of immutable environment fingerprint results.
//!
//! It does not own:
//!
//! - Fingerprint algorithms.
//! - Compilation environment semantics.
//! - Source-language environment semantics.
//! - Grammar interpretation.
//! - Operating-system discovery.
//! - Hardware discovery.
//! - Device enumeration.
//! - Target selection.
//! - Capability verification.
//! - Resource allocation.
//! - Executable discovery.
//! - External process execution.
//! - Artifact fingerprints.
//! - Digital signatures or authentication.
//! - Runtime scheduling or execution.
//!
//! ## Canonical authorities
//!
//! Compilation environment:
//!     crate::compiler::reproducibility::environment
//!
//! Canonical stable fingerprint algorithm:
//!     crate::zuir::fingerprint::stable_hash
//!
//! Backend toolchain fingerprint:
//!     crate::backend::reproducibility
//!
//! External executable validation:
//!     crate::backend::toolchain
//!
//! Platform observations:
//!     crate::zur::platforms::provider
//!
//! Host environment representation:
//!     crate::zur::platforms::host
//!
//! Toolchain configuration:
//!     crate::toolchain::environments::configuration
//!
//! Toolchain discovery:
//!     crate::toolchain::environments::discovery
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! ## Grammar boundary
//!
//! Source-level reproducibility intent is defined by:
//!
//! - grammar/compile/reproducibility.g4
//! - grammar/compile/deterministic-builds.g4
//! - grammar/compile/provenance.g4
//! - grammar/spec/provenance.md
//! - grammar/spec/resources.md
//! - grammar/specification/semantic-model.md
//! - grammar/validation/semantic-boundaries.md
//!
//! This module never parses grammar or independently interprets
//! source-level requirements, capabilities, preferences, constraints,
//! hints or budgets.
//!
//! Canonical semantic and compiler representations must supply
//! their already-validated fingerprint inputs.
//!
//! ## Identity contract
//!
//! Compilation identity and physical environment observation identity
//! are deliberately independent.
//!
//! A portable compilation environment must not acquire an implicit
//! dependency on the host operating system, processor architecture,
//! machine identity, process identity or device inventory.
//!
//! Target-specific identity requires an explicitly supplied,
//! authoritative target fingerprint.
//!
//! Platform observations are retained as separate evidence.
//!
//! ## External toolchain contract
//!
//! External executable discovery and version validation are delegated
//! to backend::toolchain.
//!
//! The backend reproducibility fingerprint is reused.
//!
//! The version-based adapter in this module fingerprints the validated
//! tool identifier and numeric version.
//!
//! It does NOT claim to fingerprint the executable's actual binary bytes.
//! Binary integrity requires separate authoritative content-digest or
//! attestation evidence.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - External toolchains.
//! - Environment facts.
//! - Platform providers.
//! - Computing domains.
//! - Targets.
//! - Processors.
//! - Accelerators.
//! - Quantum resources.
//! - Distributed participants.
//! - Memory.
//! - Storage.
//! - Tensor dimensions.
//!
//! Collections grow according to available resources and Rust's
//! representational constraints.
//!
//! No literal infinite physical capacity is promised.
//!
//! ## Determinism
//!
//! Fingerprint inputs use canonical identifiers and deterministic
//! ordering.
//!
//! No wall-clock timestamps, process identifiers, randomized hashers,
//! memory addresses, implicit environment-variable reads or
//! filesystem enumeration participate in compilation identity.
//!
//! The existing ZUIR stable SHA-256 implementation owns canonical
//! fingerprint encoding.
//!
//! ## Security
//!
//! No filesystem, network, hardware or process I/O is performed.
//!
//! Secret material must never be supplied as an environment fact.
//!
//! Fingerprints establish content identity, not authenticity,
//! authorization, trust or execution safety.
//!
//! ## Integration
//!
//! Upstream:
//!
//! - compiler/reproducibility/environment.rs
//! - backend/reproducibility.rs
//! - backend/toolchain.rs
//! - zur/platforms/provider.rs
//! - zur/platforms/host.rs
//!
//! Downstream:
//!
//! - environments/validation.rs
//! - build/inputs.rs
//! - build/plan.rs
//! - environments/fingerprints.rs consumers
//! - targets/compatibility.rs
//! - environments/reporting.rs, when introduced
//! - deployment/planning.rs
//!
//! This module does not allocate resources or establish feasibility.

use std::collections::BTreeMap;
use std::fmt;

use crate::backend::reproducibility::{
    FingerprintBuilder as BackendFingerprintBuilder,
    ToolchainFingerprint as BackendToolchainFingerprint,
};

use crate::backend::toolchain::ToolchainDescriptor;

use crate::compiler::reproducibility::environment::{
    CompilationEnvironment,
    CompilationEnvironmentBuilder,
    EnvironmentError,
    EnvironmentFact,
    EnvironmentFactImpact,
    EnvironmentKey,
    EnvironmentScope,
    ToolchainIdentity,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::ToolchainResult;

use crate::zur::platforms::host::{
    HostAvailability,
    HostEnvironment,
};

use crate::zur::platforms::provider::PlatformObservation;

use crate::zuir::fingerprint::stable_hash::{
    Fingerprint,
    FingerprintError,
    StableHashBuilder,
};

// ============================================================================
// COMPONENT IDENTITY AND VERSIONING
// ============================================================================

const COMPONENT: &str = "toolchain.environments.fingerprints";

const OPERATION_BUILD: &str =
    "toolchain.environments.fingerprints.build";

const OPERATION_TOOLCHAIN: &str =
    "toolchain.environments.fingerprints.toolchain";

const OPERATION_OBSERVATION: &str =
    "toolchain.environments.fingerprints.observation";

/// Public API version of this environment-fingerprinting adapter.
///
/// This version is independent of the language, grammar, compiler,
/// backend, runtime and artifact versions.
pub const ENVIRONMENT_FINGERPRINT_API_VERSION: u16 = 1;

/// Schema version for the platform-observation fingerprint.
///
/// Increment this version if the observation encoding changes.
pub const PLATFORM_OBSERVATION_SCHEMA_VERSION: u32 = 1;

/// Stable fingerprint domain for platform observations.
pub const PLATFORM_OBSERVATION_FINGERPRINT_DOMAIN: &str =
    "zamani.toolchain.environment.platform-observation.v1";

/// Stable fingerprint domain for version-validated external tools.
pub const VALIDATED_TOOLCHAIN_FINGERPRINT_DOMAIN: &str =
    "zamani.toolchain.environment.validated-toolchain.v1";

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

fn environment_error(
    operation: &'static str,
    source: EnvironmentError,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ValidationFailed,
        "canonical compilation environment construction failed",
    )
    .with_operation(operation)
    .with_context(COMPONENT)
    .with_source(source)
}

fn fingerprint_error(
    operation: &'static str,
    source: FingerprintError,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Internal,
        "canonical environment fingerprint construction failed",
    )
    .with_operation(operation)
    .with_context(COMPONENT)
    .with_source(source)
}

fn invalid_input(
    operation: &'static str,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
    .with_operation(operation)
    .with_context(COMPONENT)
}

// ============================================================================
// PLATFORM OBSERVATION FINGERPRINT
// ============================================================================

/// Fingerprint of a separately obtained platform observation.
///
/// This identity represents the explicitly supplied observation.
/// It is NOT automatically incorporated into portable compilation
/// identity.
///
/// It does not establish provider authenticity, hardware capability,
/// workload feasibility or continued resource availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlatformObservationFingerprint {
    provider_fingerprint: Fingerprint,
}

impl PlatformObservationFingerprint {
    /// Returns the canonical platform-observation digest.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.provider_fingerprint
    }

    /// Returns the canonical hexadecimal representation.
    #[must_use]
    pub fn to_hex(self) -> String {
        self.provider_fingerprint.to_hex()
    }
}

impl fmt::Display for PlatformObservationFingerprint {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{}", self.provider_fingerprint)
    }
}

// ============================================================================
// OPTIONAL FIELD ENCODING
// ============================================================================

fn add_optional_string(
    builder: &mut StableHashBuilder,
    name: &str,
    value: Option<&str>,
) -> ToolchainResult<()> {
    let present_field = format!("{name}.present");

    builder
        .field_bool(
            &present_field,
            value.is_some(),
        )
        .map_err(|source| {
            fingerprint_error(OPERATION_OBSERVATION, source)
        })?;

    if let Some(value) = value {
        builder
            .field_str(name, value)
            .map_err(|source| {
                fingerprint_error(OPERATION_OBSERVATION, source)
            })?;
    }

    Ok(())
}

// ============================================================================
// PLATFORM AVAILABILITY ENCODING
// ============================================================================

fn availability_name(
    availability: HostAvailability,
) -> &'static str {
    match availability {
        HostAvailability::Available => "available",
        HostAvailability::Unavailable => "unavailable",
        HostAvailability::Unknown => "unknown",
        _ => "unrecognized",
    }
}

// ============================================================================
// PLATFORM OBSERVATION FINGERPRINT CONSTRUCTION
// ============================================================================

/// Fingerprints an explicitly supplied canonical platform observation.
///
/// The fingerprint contains:
///
/// - Provider identity.
/// - Host platform identity.
/// - Operating-system identity and version.
/// - Processor architecture identity.
/// - ABI identity and version.
/// - Target-triple metadata.
/// - Canonically ordered facility identifiers and reported availability.
/// - Canonically ordered execution-interface identifiers and availability.
///
/// The following are deliberately excluded:
///
/// - Process identity.
/// - Process identifiers.
/// - Arbitrary provider properties.
/// - Arbitrary host attributes.
/// - Evidence references that may contain sensitive information.
/// - Credentials and secrets.
///
/// These fields require their own explicit semantic and security
/// contracts before they can participate in a fingerprint.
///
/// This function does not perform discovery or provider verification.
pub fn fingerprint_platform_observation(
    observation: &PlatformObservation,
) -> ToolchainResult<PlatformObservationFingerprint> {
    let host: &HostEnvironment = observation.environment();

    let platform = host.platform();

    let mut builder = StableHashBuilder::new(
        PLATFORM_OBSERVATION_FINGERPRINT_DOMAIN,
    )
    .map_err(|source| {
        fingerprint_error(OPERATION_OBSERVATION, source)
    })?;

    builder
        .field_u32(
            "schema_version",
            PLATFORM_OBSERVATION_SCHEMA_VERSION,
        )
        .map_err(|source| {
            fingerprint_error(OPERATION_OBSERVATION, source)
        })?;

    builder
        .field_str(
            "provider_id",
            observation.provider_id().as_str(),
        )
        .map_err(|source| {
            fingerprint_error(OPERATION_OBSERVATION, source)
        })?;

    builder
        .field_str(
            "platform_id",
            platform.id().as_str(),
        )
        .map_err(|source| {
            fingerprint_error(OPERATION_OBSERVATION, source)
        })?;

    add_optional_string(
        &mut builder,
        "operating_system",
        platform.operating_system(),
    )?;

    add_optional_string(
        &mut builder,
        "operating_system_version",
        platform.operating_system_version(),
    )?;

    add_optional_string(
        &mut builder,
        "architecture",
        platform.architecture(),
    )?;

    add_optional_string(
        &mut builder,
        "abi",
        platform.abi(),
    )?;

    add_optional_string(
        &mut builder,
        "abi_version",
        platform.abi_version(),
    )?;

    add_optional_string(
        &mut builder,
        "target_triple",
        platform.target_triple(),
    )?;

    // ------------------------------------------------------------------------
    // Host facilities
    // ------------------------------------------------------------------------

    let facility_count = u64::try_from(host.facility_count())
        .map_err(|_| {
            invalid_input(
                OPERATION_OBSERVATION,
                "host facility count cannot be represented by the fingerprint schema",
            )
        })?;

    builder
        .field_u64("facility_count", facility_count)
        .map_err(|source| {
            fingerprint_error(OPERATION_OBSERVATION, source)
        })?;

    for (identifier, descriptor) in host.facilities() {
        builder
            .field_str(
                "facility.identifier",
                identifier.as_str(),
            )
            .map_err(|source| {
                fingerprint_error(OPERATION_OBSERVATION, source)
            })?;

        builder
            .field_str(
                "facility.availability",
                availability_name(descriptor.availability()),
            )
            .map_err(|source| {
                fingerprint_error(OPERATION_OBSERVATION, source)
            })?;
    }

    // ------------------------------------------------------------------------
    // Execution interfaces
    // ------------------------------------------------------------------------

    let interface_count =
        u64::try_from(host.execution_interface_count())
            .map_err(|_| {
                invalid_input(
                    OPERATION_OBSERVATION,
                    "execution-interface count cannot be represented by the fingerprint schema",
                )
            })?;

    builder
        .field_u64(
            "execution_interface_count",
            interface_count,
        )
        .map_err(|source| {
            fingerprint_error(OPERATION_OBSERVATION, source)
        })?;

    for (identifier, descriptor) in host.execution_interfaces() {
        builder
            .field_str(
                "execution_interface.identifier",
                identifier.as_str(),
            )
            .map_err(|source| {
                fingerprint_error(OPERATION_OBSERVATION, source)
            })?;

        builder
            .field_str(
                "execution_interface.availability",
                availability_name(descriptor.availability()),
            )
            .map_err(|source| {
                fingerprint_error(OPERATION_OBSERVATION, source)
            })?;
    }

    Ok(PlatformObservationFingerprint {
        provider_fingerprint: builder.finish(),
    })
}

// ============================================================================
// EXTERNAL TOOLCHAIN FINGERPRINTING
// ============================================================================

/// Produces a backend-owned fingerprint for an already validated
/// external toolchain descriptor.
///
/// This adapter includes the caller-defined tool identifier and
/// successfully parsed numeric version.
///
/// It deliberately does not claim binary-content verification.
///
/// The backend owns the fingerprint type and fingerprint algorithm.
pub fn fingerprint_validated_toolchain(
    descriptor: &ToolchainDescriptor,
) -> BackendToolchainFingerprint {
    let version = descriptor.version();

    let mut builder = BackendFingerprintBuilder::new(
        VALIDATED_TOOLCHAIN_FINGERPRINT_DOMAIN,
    );

    builder
        .string("toolchain.identifier", descriptor.id())
        .field(
            "toolchain.version.major",
            &version.major.to_be_bytes(),
        )
        .field(
            "toolchain.version.minor",
            &version.minor.to_be_bytes(),
        )
        .field(
            "toolchain.version.patch",
            &version.patch.to_be_bytes(),
        );

    BackendToolchainFingerprint::from_digest(
        builder.finish_digest(),
    )
}

// ============================================================================
// IMMUTABLE ENVIRONMENT FINGERPRINT RESULT
// ============================================================================

/// Immutable toolchain-facing environment fingerprint result.
///
/// The canonical compilation environment remains owned by the compiler.
///
/// Platform observation remains separate and does not silently modify
/// compilation identity.
///
/// The structure is immutable after construction.
#[derive(Debug)]
pub struct EnvironmentFingerprint {
    compilation_environment: CompilationEnvironment,

    platform_observation: Option<PlatformObservationFingerprint>,
}

impl EnvironmentFingerprint {
    /// Returns the canonical compiler-owned compilation environment.
    #[must_use]
    pub const fn compilation_environment(
        &self,
    ) -> &CompilationEnvironment {
        &self.compilation_environment
    }

    /// Returns the canonical compilation fingerprint.
    ///
    /// This is the fingerprint used for the declared compilation
    /// environment identity.
    #[must_use]
    pub const fn compilation_fingerprint(
        &self,
    ) -> Fingerprint {
        self.compilation_environment.fingerprint()
    }

    /// Returns the separately calculated platform observation
    /// fingerprint, if an observation was supplied.
    #[must_use]
    pub const fn platform_observation(
        &self,
    ) -> Option<PlatformObservationFingerprint> {
        self.platform_observation
    }

    /// Returns the environment scope.
    #[must_use]
    pub const fn scope(&self) -> EnvironmentScope {
        self.compilation_environment.scope()
    }

    /// Returns whether the compilation identity is portable.
    #[must_use]
    pub const fn is_portable(&self) -> bool {
        self.compilation_environment.is_portable()
    }

    /// Returns the number of validated external toolchains included
    /// in compilation identity.
    #[must_use]
    pub fn toolchain_count(&self) -> usize {
        self.compilation_environment.toolchain_count()
    }

    /// Returns the number of explicitly supplied environment facts.
    #[must_use]
    pub fn fact_count(&self) -> usize {
        self.compilation_environment.fact_count()
    }

    /// Returns the number of output-affecting environment facts.
    #[must_use]
    pub fn output_affecting_fact_count(&self) -> usize {
        self.compilation_environment
            .output_affecting_fact_count()
    }

    /// Returns the underlying compiler-owned environment fingerprint.
    ///
    /// This is an explicit alias for callers that need the canonical
    /// compiler identity rather than the toolchain wrapper.
    #[must_use]
    pub const fn compiler_environment(
        &self,
    ) -> &CompilationEnvironment {
        &self.compilation_environment
    }
}

// ============================================================================
// ENVIRONMENT FINGERPRINT BUILDER
// ============================================================================

/// Builder for reproducible toolchain environment identities.
///
/// Mandatory compiler, grammar and semantic-model fingerprints are
/// supplied by their authoritative implementations.
///
/// The builder does not discover or invent any of these identities.
pub struct EnvironmentFingerprintBuilder {
    compilation: CompilationEnvironmentBuilder,

    platform_observation: Option<PlatformObservationFingerprint>,
}

impl EnvironmentFingerprintBuilder {
    /// Creates a builder for a portable compilation environment.
    ///
    /// Portable environments must not contain an implicit target
    /// fingerprint.
    #[must_use]
    pub fn portable(
        compiler_fingerprint: Fingerprint,
        grammar_fingerprint: Fingerprint,
        semantic_model_fingerprint: Fingerprint,
    ) -> Self {
        Self {
            compilation: CompilationEnvironment::portable(
                compiler_fingerprint,
                grammar_fingerprint,
                semantic_model_fingerprint,
            ),
            platform_observation: None,
        }
    }

    /// Creates a builder for an explicitly target-specific environment.
    ///
    /// The target fingerprint must originate from the authoritative
    /// target-description or capability subsystem.
    #[must_use]
    pub fn target_specific(
        compiler_fingerprint: Fingerprint,
        grammar_fingerprint: Fingerprint,
        semantic_model_fingerprint: Fingerprint,
        target_fingerprint: Fingerprint,
    ) -> Self {
        Self {
            compilation: CompilationEnvironment::target_specific(
                compiler_fingerprint,
                grammar_fingerprint,
                semantic_model_fingerprint,
            )
            .target_fingerprint(target_fingerprint),

            platform_observation: None,
        }
    }

    /// Adds an explicitly constructed compiler-owned environment fact.
    ///
    /// The fact's impact classification determines whether it
    /// contributes to compilation identity.
    ///
    /// Duplicate fact identities are rejected by the canonical
    /// compiler environment builder.
    pub fn add_fact(
        &mut self,
        fact: EnvironmentFact,
    ) -> ToolchainResult<&mut Self> {
        self.compilation
            .add_fact(fact)
            .map_err(|source| {
                environment_error(OPERATION_BUILD, source)
            })?;

        Ok(self)
    }

    /// Adds a UTF-8 environment fact with an explicit impact category.
    ///
    /// Callers must supply a canonical key and canonical value.
    ///
    /// Secrets must never be included.
    pub fn add_text_fact(
        &mut self,
        key: EnvironmentKey,
        value: impl Into<String>,
        impact: EnvironmentFactImpact,
    ) -> ToolchainResult<&mut Self> {
        self.compilation
            .add_text_fact(key, value, impact)
            .map_err(|source| {
                environment_error(OPERATION_BUILD, source)
            })?;

        Ok(self)
    }

    /// Adds an already verified, backend-owned external toolchain
    /// fingerprint.
    ///
    /// This is the preferred method when a provider can supply an
    /// authoritative content digest or other stronger identity.
    ///
    /// The backend fingerprint is converted into the canonical
    /// compiler fingerprint representation without changing its bytes.
    pub fn add_toolchain_fingerprint(
        &mut self,
        identifier: impl Into<String>,
        fingerprint: BackendToolchainFingerprint,
    ) -> ToolchainResult<&mut Self> {
        let key = EnvironmentKey::new(identifier)
            .map_err(|source| {
                environment_error(OPERATION_TOOLCHAIN, source)
            })?;

        let compiler_fingerprint = Fingerprint::from_bytes(
            *fingerprint.as_bytes(),
        );

        self.compilation
            .add_toolchain(ToolchainIdentity::new(
                key,
                compiler_fingerprint,
            ))
            .map_err(|source| {
                environment_error(OPERATION_TOOLCHAIN, source)
            })?;

        Ok(self)
    }

    /// Adds a successfully discovered and version-validated external
    /// toolchain descriptor.
    ///
    /// The descriptor must originate from the canonical
    /// backend::toolchain discovery subsystem.
    ///
    /// This method fingerprints its declared identifier and parsed
    /// numeric version. It does not verify executable binary integrity.
    pub fn add_validated_toolchain(
        &mut self,
        descriptor: &ToolchainDescriptor,
    ) -> ToolchainResult<&mut Self> {
        if descriptor.id().trim().is_empty() {
            return Err(invalid_input(
                OPERATION_TOOLCHAIN,
                "validated toolchain identifier must not be empty",
            ));
        }

        let fingerprint =
            fingerprint_validated_toolchain(descriptor);

        self.add_toolchain_fingerprint(
            descriptor.id(),
            fingerprint,
        )
    }

    /// Adds a canonical platform observation as separate observational
    /// evidence.
    ///
    /// The platform observation is never automatically inserted into
    /// the compilation fingerprint.
    ///
    /// A second observation cannot silently replace the first one.
    pub fn add_platform_observation(
        &mut self,
        observation: &PlatformObservation,
    ) -> ToolchainResult<&mut Self> {
        if self.platform_observation.is_some() {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    "a platform observation has already been supplied",
                )
                .with_operation(OPERATION_OBSERVATION)
                .with_context(COMPONENT),
            );
        }

        self.platform_observation = Some(
            fingerprint_platform_observation(observation)?,
        );

        Ok(self)
    }

    /// Constructs the immutable environment fingerprint result.
    ///
    /// All canonical compiler environment validation is delegated
    /// to CompilationEnvironmentBuilder.
    ///
    /// A missing target fingerprint, duplicate fact, duplicate
    /// toolchain or fingerprinting failure prevents construction.
    pub fn build(self) -> ToolchainResult<EnvironmentFingerprint> {
        let compilation_environment =
            self.compilation.build().map_err(|source| {
                environment_error(OPERATION_BUILD, source)
            })?;

        Ok(EnvironmentFingerprint {
            compilation_environment,
            platform_observation: self.platform_observation,
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fingerprint(value: u8) -> Fingerprint {
        Fingerprint::from_bytes([value; 32])
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(ENVIRONMENT_FINGERPRINT_API_VERSION, 1);
    }

    #[test]
    fn portable_environment_builds() {
        let result = EnvironmentFingerprintBuilder::portable(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
        )
        .build()
        .expect("portable environment should build");

        assert!(result.is_portable());
        assert_eq!(result.toolchain_count(), 0);
        assert_eq!(result.fact_count(), 0);
        assert!(result.platform_observation().is_none());
    }

    #[test]
    fn identical_inputs_produce_identical_fingerprints() {
        let first = EnvironmentFingerprintBuilder::portable(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
        )
        .build()
        .expect("first environment should build");

        let second = EnvironmentFingerprintBuilder::portable(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
        )
        .build()
        .expect("second environment should build");

        assert_eq!(
            first.compilation_fingerprint(),
            second.compilation_fingerprint(),
        );
    }

    #[test]
    fn different_compiler_inputs_change_identity() {
        let first = EnvironmentFingerprintBuilder::portable(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
        )
        .build()
        .expect("first environment should build");

        let second = EnvironmentFingerprintBuilder::portable(
            fingerprint(4),
            fingerprint(2),
            fingerprint(3),
        )
        .build()
        .expect("second environment should build");

        assert_ne!(
            first.compilation_fingerprint(),
            second.compilation_fingerprint(),
        );
    }

    #[test]
    fn target_specific_environment_requires_explicit_target() {
        let result = EnvironmentFingerprintBuilder {
            compilation: CompilationEnvironmentBuilder::new(
                EnvironmentScope::TargetSpecific,
                fingerprint(1),
                fingerprint(2),
                fingerprint(3),
            ),
            platform_observation: None,
        }
        .build();

        assert!(result.is_err());
    }

    #[test]
    fn target_specific_environment_accepts_explicit_target() {
        let result = EnvironmentFingerprintBuilder::target_specific(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
            fingerprint(4),
        )
        .build()
        .expect("explicit target environment should build");

        assert!(!result.is_portable());
    }

    #[test]
    fn output_affecting_facts_change_identity() {
        let key = EnvironmentKey::new("compiler.optimization")
            .expect("valid environment key");

        let mut first = EnvironmentFingerprintBuilder::portable(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
        );

        first
            .add_text_fact(
                key.clone(),
                "enabled",
                EnvironmentFactImpact::OutputAffecting,
            )
            .expect("fact should be accepted");

        let mut second = EnvironmentFingerprintBuilder::portable(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
        );

        second
            .add_text_fact(
                key,
                "disabled",
                EnvironmentFactImpact::OutputAffecting,
            )
            .expect("fact should be accepted");

        let first = first.build().expect("first environment");
        let second = second.build().expect("second environment");

        assert_ne!(
            first.compilation_fingerprint(),
            second.compilation_fingerprint(),
        );
    }

    #[test]
    fn observational_facts_do_not_change_compilation_identity() {
        let key = EnvironmentKey::new("environment.description")
            .expect("valid environment key");

        let mut first = EnvironmentFingerprintBuilder::portable(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
        );

        first
            .add_text_fact(
                key.clone(),
                "workstation",
                EnvironmentFactImpact::Observational,
            )
            .expect("fact should be accepted");

        let mut second = EnvironmentFingerprintBuilder::portable(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
        );

        second
            .add_text_fact(
                key,
                "embedded-system",
                EnvironmentFactImpact::Observational,
            )
            .expect("fact should be accepted");

        let first = first.build().expect("first environment");
        let second = second.build().expect("second environment");

        assert_eq!(
            first.compilation_fingerprint(),
            second.compilation_fingerprint(),
        );

        assert_eq!(first.fact_count(), 1);
        assert_eq!(second.fact_count(), 1);
    }

    #[test]
    fn observation_is_separate_from_compilation_identity() {
        let environment = EnvironmentFingerprintBuilder::portable(
            fingerprint(1),
            fingerprint(2),
            fingerprint(3),
        )
        .build()
        .expect("environment should build");

        assert!(environment.platform_observation().is_none());

        assert!(environment.is_portable());
    }
}
