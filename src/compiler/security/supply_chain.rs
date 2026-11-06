
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler
//!
//! File:
//!     src/compiler/security/supply_chain.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Provide the compiler-side software supply-chain security boundary.
//!
//! This module evaluates package and dependency evidence before artifacts
//! are accepted for trusted compilation, linking, packaging or deployment.
//!
//! It supports:
//!
//! - Direct and transitive dependencies.
//! - Arbitrary package ecosystems.
//! - Arbitrary package source providers.
//! - Package revisions and immutable references.
//! - Content-integrity verification.
//! - Signature verification.
//! - Build-attestation verification.
//! - Builder identity verification.
//! - Vulnerability intelligence.
//! - License-policy evaluation.
//! - Dependency graph validation.
//! - Reproducibility evidence.
//! - Configurable operational budgets.
//! - Deterministic reports.
//! - Explicit incomplete verification.
//! - Fail-closed security decisions.
//!
//! ARCHITECTURAL OWNERSHIP
//!
//! This module owns supply-chain verification orchestration.
//!
//! It does NOT own:
//!
//! - Source grammar or parsing.
//! - Security-language semantics.
//! - Identity authentication.
//! - Authorization.
//! - Cryptographic implementations.
//! - Private-key storage.
//! - Trust-anchor storage.
//! - Artifact representation.
//! - Artifact serialization.
//! - Compilation provenance storage.
//! - Package downloading.
//! - Package installation.
//! - Runtime execution.
//! - Hardware discovery.
//! - Backend implementation.
//!
//! CANONICAL INTEGRATIONS
//!
//! crate::compiler::artifacts::artifact
//!     ArtifactDigest and artifact identity.
//!
//! crate::compiler::artifacts::manifest
//!     Canonical artifact dependency relationships.
//!
//! crate::compiler::artifacts::validation
//!     Artifact validation.
//!
//! crate::compiler::context::provenance_context
//!     Compilation provenance.
//!
//! crate::backend::security
//!     Backend security enforcement.
//!
//! crate::compiler::safety_guard
//!     Instruction-stream security inspection.
//!
//! POCO-REAF
//!
//! Supply-chain decisions are independent of:
//!
//! - CPU architecture.
//! - GPU architecture.
//! - FPGA family.
//! - ASIC implementation.
//! - Quantum processor.
//! - Neuromorphic processor.
//! - Photonic processor.
//! - Distributed topology.
//! - Number of devices.
//! - Number of dependencies.
//! - Number of computing domains.
//!
//! No universal hardware or dependency-count ceiling is imposed.
//!
//! Actual processing is limited by available resources and optional
//! caller-supplied operational budgets.
//!
//! SECURITY PRINCIPLE
//!
//! Package metadata is not proof of authenticity.
//! A checksum is not proof of trust.
//! A signature is not automatically proof of authorization.
//! Provenance is not proof of correctness.
//! An attestation is not automatically trustworthy.
//!
//! Every required property must be independently established.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::compiler::artifacts::artifact::ArtifactDigest;

// ============================================================================
// API VERSION
// ============================================================================

/// Supply-chain API version.
///
/// Independent of the Zamani language, grammar, compiler release,
/// artifact model, and backend ABI versions.
pub const SUPPLY_CHAIN_API_VERSION: u16 = 1;

/// Stable supply-chain report schema identifier.
pub const SUPPLY_CHAIN_REPORT_SCHEMA: &str =
    "zamani.compiler.security.supply-chain";

// ============================================================================
// COMMON VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    description: &'static str,
) -> Result<(), SupplyChainError> {
    if value.is_empty() || value.trim() != value {
        return Err(SupplyChainError::InvalidIdentifier {
            description,
            value: value.to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(SupplyChainError::InvalidIdentifier {
            description,
            value: value.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by supply-chain assessment.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SupplyChainError {
    InvalidIdentifier {
        description: &'static str,
        value: String,
    },

    DuplicatePackage {
        package: String,
    },

    MissingDependency {
        package: String,
        dependency: String,
    },

    InvalidPolicy {
        reason: String,
    },

    BudgetExceeded {
        resource: &'static str,
        requested: usize,
        permitted: usize,
    },

    ArithmeticOverflow,

    ProviderFailure {
        provider: String,
        reason: String,
    },
}

impl fmt::Display for SupplyChainError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier {
                description,
                value,
            } => {
                write!(
                    formatter,
                    "invalid {description}: {value}"
                )
            }

            Self::DuplicatePackage { package } => {
                write!(
                    formatter,
                    "duplicate package identity: {package}"
                )
            }

            Self::MissingDependency {
                package,
                dependency,
            } => {
                write!(
                    formatter,
                    "package {package} references missing dependency {dependency}"
                )
            }

            Self::InvalidPolicy { reason } => {
                write!(
                    formatter,
                    "invalid supply-chain policy: {reason}"
                )
            }

            Self::BudgetExceeded {
                resource,
                requested,
                permitted,
            } => {
                write!(
                    formatter,
                    "supply-chain {resource} budget exceeded: \
                     requested {requested}, permitted {permitted}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "supply-chain accounting arithmetic overflow"
                )
            }

            Self::ProviderFailure {
                provider,
                reason,
            } => {
                write!(
                    formatter,
                    "supply-chain provider {provider} failed: {reason}"
                )
            }
        }
    }
}

impl Error for SupplyChainError {}

pub type SupplyChainResult<T> = Result<T, SupplyChainError>;

// ============================================================================
// PACKAGE IDENTITIES
// ============================================================================

/// Open-world package identity.
///
/// The identifier is ecosystem-neutral.
///
/// Examples:
///
///     cargo:serde
///     npm:some-package
///     pypi:some-package
///     maven:group:artifact
///     custom-ecosystem:package
///
/// These examples are identifiers, not a closed registry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageId(Arc<str>);

impl PackageId {
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> SupplyChainResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "package identity",
        )?;

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PackageId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PACKAGE SOURCES
// ============================================================================

/// Identifies a package source.
///
/// The source identifier is descriptive and does not establish trust.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackageSource {
    provider: Arc<str>,
    location: Arc<str>,
}

impl PackageSource {
    pub fn new(
        provider: impl Into<Arc<str>>,
        location: impl Into<Arc<str>>,
    ) -> SupplyChainResult<Self> {
        let provider = provider.into();
        let location = location.into();

        validate_identifier(
            &provider,
            "package provider",
        )?;

        validate_identifier(
            &location,
            "package source",
        )?;

        Ok(Self {
            provider,
            location,
        })
    }

    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    #[must_use]
    pub fn location(&self) -> &str {
        &self.location
    }
}

// ============================================================================
// SIGNATURE EVIDENCE
// ============================================================================

/// Signature evidence supplied by a package source.
///
/// This is untrusted input.
///
/// Verification is performed by an explicitly registered provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureEnvelope {
    algorithm: Arc<str>,
    key_reference: Arc<str>,
    signature: Arc<[u8]>,
    signed_digest: ArtifactDigest,
}

impl SignatureEnvelope {
    pub fn new(
        algorithm: impl Into<Arc<str>>,
        key_reference: impl Into<Arc<str>>,
        signature: impl Into<Arc<[u8]>>,
        signed_digest: ArtifactDigest,
    ) -> SupplyChainResult<Self> {
        let algorithm = algorithm.into();
        let key_reference = key_reference.into();
        let signature = signature.into();

        validate_identifier(
            &algorithm,
            "signature algorithm",
        )?;

        validate_identifier(
            &key_reference,
            "signature key reference",
        )?;

        if signature.is_empty() {
            return Err(SupplyChainError::InvalidPolicy {
                reason: "empty signature evidence".into(),
            });
        }

        Ok(Self {
            algorithm,
            key_reference,
            signature,
            signed_digest,
        })
    }

    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    #[must_use]
    pub fn key_reference(&self) -> &str {
        &self.key_reference
    }

    #[must_use]
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    #[must_use]
    pub fn signed_digest(&self) -> &ArtifactDigest {
        &self.signed_digest
    }
}

// ============================================================================
// ATTESTATION EVIDENCE
// ============================================================================

/// Build or package attestation reference.
///
/// The payload is opaque and must be verified by an attestation provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttestationEnvelope {
    format: Arc<str>,
    subject: PackageId,
    payload: Arc<[u8]>,
}

impl AttestationEnvelope {
    pub fn new(
        format: impl Into<Arc<str>>,
        subject: PackageId,
        payload: impl Into<Arc<[u8]>>,
    ) -> SupplyChainResult<Self> {
        let format = format.into();
        let payload = payload.into();

        validate_identifier(
            &format,
            "attestation format",
        )?;

        if payload.is_empty() {
            return Err(SupplyChainError::InvalidPolicy {
                reason: "empty attestation evidence".into(),
            });
        }

        Ok(Self {
            format,
            subject,
            payload,
        })
    }

    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    #[must_use]
    pub fn subject(&self) -> &PackageId {
        &self.subject
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

// ============================================================================
// PACKAGE RECORD
// ============================================================================

/// Dependency relationship.
///
/// Dependency versions and constraints are supplied by the package
/// resolution authority. This module does not implement a resolver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDependency {
    package: PackageId,
    version_requirement: Arc<str>,
}

impl PackageDependency {
    pub fn new(
        package: PackageId,
        version_requirement: impl Into<Arc<str>>,
    ) -> SupplyChainResult<Self> {
        let version_requirement = version_requirement.into();

        validate_identifier(
            &version_requirement,
            "dependency version requirement",
        )?;

        Ok(Self {
            package,
            version_requirement,
        })
    }

    #[must_use]
    pub fn package(&self) -> &PackageId {
        &self.package
    }

    #[must_use]
    pub fn version_requirement(&self) -> &str {
        &self.version_requirement
    }
}

/// Complete package evidence record.
///
/// Package metadata is untrusted until evaluated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRecord {
    id: PackageId,
    version: Arc<str>,
    source: Option<PackageSource>,
    immutable_revision: Option<Arc<str>>,
    digest: Option<ArtifactDigest>,
    license: Option<Arc<str>>,
    dependencies: Vec<PackageDependency>,
    signature: Option<SignatureEnvelope>,
    attestation: Option<AttestationEnvelope>,
    declared_builder: Option<Arc<str>>,
    reproducible_build_claim: bool,
}

impl PackageRecord {
    pub fn new(
        id: PackageId,
        version: impl Into<Arc<str>>,
    ) -> SupplyChainResult<Self> {
        let version = version.into();

        validate_identifier(
            &version,
            "package version",
        )?;

        Ok(Self {
            id,
            version,
            source: None,
            immutable_revision: None,
            digest: None,
            license: None,
            dependencies: Vec::new(),
            signature: None,
            attestation: None,
            declared_builder: None,
            reproducible_build_claim: false,
        })
    }

    #[must_use]
    pub fn with_source(
        mut self,
        source: PackageSource,
    ) -> Self {
        self.source = Some(source);
        self
    }

    #[must_use]
    pub fn with_revision(
        mut self,
        revision: impl Into<Arc<str>>,
    ) -> Self {
        self.immutable_revision = Some(revision.into());
        self
    }

    #[must_use]
    pub fn with_digest(
        mut self,
        digest: ArtifactDigest,
    ) -> Self {
        self.digest = Some(digest);
        self
    }

    #[must_use]
    pub fn with_license(
        mut self,
        license: impl Into<Arc<str>>,
    ) -> Self {
        self.license = Some(license.into());
        self
    }

    #[must_use]
    pub fn with_dependency(
        mut self,
        dependency: PackageDependency,
    ) -> Self {
        self.dependencies.push(dependency);
        self
    }

    #[must_use]
    pub fn with_signature(
        mut self,
        signature: SignatureEnvelope,
    ) -> Self {
        self.signature = Some(signature);
        self
    }

    #[must_use]
    pub fn with_attestation(
        mut self,
        attestation: AttestationEnvelope,
    ) -> Self {
        self.attestation = Some(attestation);
        self
    }

    #[must_use]
    pub fn with_builder(
        mut self,
        builder: impl Into<Arc<str>>,
    ) -> Self {
        self.declared_builder = Some(builder.into());
        self
    }

    #[must_use]
    pub fn with_reproducible_build_claim(
        mut self,
        enabled: bool,
    ) -> Self {
        self.reproducible_build_claim = enabled;
        self
    }

    #[must_use]
    pub fn id(&self) -> &PackageId {
        &self.id
    }

    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    #[must_use]
    pub fn source(&self) -> Option<&PackageSource> {
        self.source.as_ref()
    }

    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.immutable_revision.as_deref()
    }

    #[must_use]
    pub fn digest(&self) -> Option<&ArtifactDigest> {
        self.digest.as_ref()
    }

    #[must_use]
    pub fn license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    #[must_use]
    pub fn dependencies(&self) -> &[PackageDependency] {
        &self.dependencies
    }

    #[must_use]
    pub fn signature(&self) -> Option<&SignatureEnvelope> {
        self.signature.as_ref()
    }

    #[must_use]
    pub fn attestation(&self) -> Option<&AttestationEnvelope> {
        self.attestation.as_ref()
    }
}

// ============================================================================
// PROVIDER VERDICTS
// ============================================================================

/// Result of an independent evidence verification operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderVerdict {
    Verified {
        authority: String,
    },

    Rejected {
        reason: String,
    },

    Indeterminate {
        reason: String,
    },
}

impl ProviderVerdict {
    fn authority(&self) -> Option<&str> {
        match self {
            Self::Verified { authority } => {
                Some(authority)
            }
            _ => None,
        }
    }
}

/// Verifies package content against its declared digest.
///
/// Implementations may stream content from external stores.
///
/// They must not claim success without checking the actual bytes.
pub trait PackageContentVerifier: Send + Sync {
    fn verify_content(
        &self,
        package: &PackageRecord,
        digest: &ArtifactDigest,
    ) -> Result<ProviderVerdict, String>;
}

/// Verifies cryptographic signatures and signer trust.
///
/// Implementations own:
///
/// - Cryptographic algorithm validation.
/// - Signature verification.
/// - Key resolution.
/// - Trust-chain validation.
/// - Revocation checks.
/// - Signer identity verification.
pub trait PackageSignatureVerifier: Send + Sync {
    fn verify_signature(
        &self,
        package: &PackageRecord,
        evidence: &SignatureEnvelope,
    ) -> Result<ProviderVerdict, String>;
}

/// Verifies build and package attestations.
///
/// Implementations must validate the attestation subject and claims,
/// builder identity, materials, invocation and applicable trust roots.
pub trait PackageAttestationVerifier: Send + Sync {
    fn verify_attestation(
        &self,
        package: &PackageRecord,
        evidence: &AttestationEnvelope,
    ) -> Result<ProviderVerdict, String>;
}

/// Vulnerability information returned by an external advisory provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VulnerabilityFinding {
    id: String,
    severity: VulnerabilitySeverity,
    affected_range: String,
    description: String,
}

impl VulnerabilityFinding {
    pub fn new(
        id: impl Into<String>,
        severity: VulnerabilitySeverity,
        affected_range: impl Into<String>,
        description: impl Into<String>,
    ) -> SupplyChainResult<Self> {
        let id = id.into();

        validate_identifier(
            &id,
            "vulnerability identifier",
        )?;

        Ok(Self {
            id,
            severity,
            affected_range: affected_range.into(),
            description: description.into(),
        })
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn severity(&self) -> VulnerabilitySeverity {
        self.severity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VulnerabilitySeverity {
    Informational,
    Low,
    Moderate,
    High,
    Critical,
}

/// Retrieves vulnerability information.
///
/// This provider must communicate whether its response is complete
/// for the requested package and advisory data source.
pub trait VulnerabilityProvider: Send + Sync {
    fn query(
        &self,
        package: &PackageRecord,
    ) -> Result<Vec<VulnerabilityFinding>, String>;
}

// ============================================================================
// CONFIGURABLE OPERATIONAL BUDGET
// ============================================================================

/// Optional caller-controlled processing budgets.
///
/// None means that this module imposes no additional explicit budget.
///
/// Actual process capacity remains finite.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SupplyChainBudget {
    pub max_packages: Option<usize>,
    pub max_dependency_edges: Option<usize>,
}

impl SupplyChainBudget {
    pub fn validate(&self) -> SupplyChainResult<()> {
        if self.max_packages == Some(0) {
            return Err(SupplyChainError::InvalidPolicy {
                reason: "package budget cannot be zero".into(),
            });
        }

        if self.max_dependency_edges == Some(0) {
            return Err(SupplyChainError::InvalidPolicy {
                reason: "dependency budget cannot be zero".into(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// SECURITY POLICY
// ============================================================================

/// Configurable supply-chain acceptance policy.
///
/// Security requirements are distinct from preferences.
///
/// No provider is assumed trustworthy by default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplyChainPolicy {
    pub require_package_digest: bool,
    pub require_immutable_revision: bool,
    pub require_package_signature: bool,
    pub require_attestation: bool,
    pub require_declared_builder: bool,
    pub require_reproducible_build_claim: bool,
    pub require_license_information: bool,
    pub allowed_sources: BTreeSet<String>,
    pub allowed_signers: BTreeSet<String>,
    pub allowed_builders: BTreeSet<String>,
    pub allowed_licenses: BTreeSet<String>,
    pub denied_licenses: BTreeSet<String>,
    pub maximum_vulnerability_severity: VulnerabilitySeverity,
    pub fail_on_indeterminate: bool,
    pub budget: SupplyChainBudget,
}

impl Default for SupplyChainPolicy {
    fn default() -> Self {
        Self {
            require_package_digest: true,
            require_immutable_revision: true,
            require_package_signature: true,
            require_attestation: true,
            require_declared_builder: true,
            require_reproducible_build_claim: false,
            require_license_information: true,
            allowed_sources: BTreeSet::new(),
            allowed_signers: BTreeSet::new(),
            allowed_builders: BTreeSet::new(),
            allowed_licenses: BTreeSet::new(),
            denied_licenses: BTreeSet::new(),
            maximum_vulnerability_severity:
                VulnerabilitySeverity::High,
            fail_on_indeterminate: true,
            budget: SupplyChainBudget::default(),
        }
    }
}

impl SupplyChainPolicy {
    pub fn validate(&self) -> SupplyChainResult<()> {
        self.budget.validate()?;

        for source in &self.allowed_sources {
            validate_identifier(source, "allowed source")?;
        }

        for signer in &self.allowed_signers {
            validate_identifier(signer, "allowed signer")?;
        }

        for builder in &self.allowed_builders {
            validate_identifier(builder, "allowed builder")?;
        }

        for license in &self.allowed_licenses {
            validate_identifier(license, "allowed license")?;
        }

        for license in &self.denied_licenses {
            validate_identifier(license, "denied license")?;
        }

        Ok(())
    }
}

// ============================================================================
// FINDINGS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FindingSeverity {
    Informational,
    Warning,
    Deferred,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplyChainFinding {
    pub code: String,
    pub severity: FindingSeverity,
    pub package: Option<PackageId>,
    pub message: String,
}

impl SupplyChainFinding {
    fn new(
        code: impl Into<String>,
        severity: FindingSeverity,
        package: Option<PackageId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            package,
            message: message.into(),
        }
    }
}

// ============================================================================
// REPORT
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupplyChainStatus {
    Accepted,
    Rejected,
    Incomplete,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SupplyChainReport {
    findings: Vec<SupplyChainFinding>,
    topological_order: Vec<PackageId>,
    packages_examined: usize,
    dependency_edges_examined: usize,
}

impl SupplyChainReport {
    #[must_use]
    pub fn findings(&self) -> &[SupplyChainFinding] {
        &self.findings
    }

    #[must_use]
    pub fn topological_order(&self) -> &[PackageId] {
        &self.topological_order
    }

    #[must_use]
    pub fn packages_examined(&self) -> usize {
        self.packages_examined
    }

    #[must_use]
    pub fn dependency_edges_examined(&self) -> usize {
        self.dependency_edges_examined
    }

    #[must_use]
    pub fn status(&self) -> SupplyChainStatus {
        if self.findings.iter().any(|finding| {
            finding.severity == FindingSeverity::Error
        }) {
            SupplyChainStatus::Rejected
        } else if self.findings.iter().any(|finding| {
            finding.severity == FindingSeverity::Deferred
        }) {
            SupplyChainStatus::Incomplete
        } else {
            SupplyChainStatus::Accepted
        }
    }

    #[must_use]
    pub fn is_accepted(&self) -> bool {
        self.status() == SupplyChainStatus::Accepted
    }

    fn push(
        &mut self,
        finding: SupplyChainFinding,
    ) {
        self.findings.push(finding);
    }

    fn defer_or_reject(
        &mut self,
        policy: &SupplyChainPolicy,
        code: &'static str,
        package: Option<PackageId>,
        message: impl Into<String>,
    ) {
        let severity = if policy.fail_on_indeterminate {
            FindingSeverity::Error
        } else {
            FindingSeverity::Deferred
        };

        self.push(SupplyChainFinding::new(
            code,
            severity,
            package,
            message,
        ));
    }
}

// ============================================================================
// VERIFICATION PROVIDERS
// ============================================================================

/// Collection of independently implemented security providers.
///
/// Absence of a provider never implies successful verification.
#[derive(Default)]
pub struct SupplyChainProviders<'a> {
    pub content: Option<&'a dyn PackageContentVerifier>,
    pub signatures: Option<&'a dyn PackageSignatureVerifier>,
    pub attestations: Option<&'a dyn PackageAttestationVerifier>,
    pub vulnerabilities: Option<&'a dyn VulnerabilityProvider>,
}

// ============================================================================
// SUPPLY-CHAIN VERIFIER
// ============================================================================

pub struct SupplyChainVerifier {
    policy: SupplyChainPolicy,
}

impl SupplyChainVerifier {
    pub fn new(
        policy: SupplyChainPolicy,
    ) -> SupplyChainResult<Self> {
        policy.validate()?;

        Ok(Self { policy })
    }

    #[must_use]
    pub fn policy(&self) -> &SupplyChainPolicy {
        &self.policy
    }

    /// Assess a complete resolved package graph.
    ///
    /// The input must contain every dependency referenced by a package.
    ///
    /// Missing dependencies, duplicate identities and cycles are reported.
    ///
    /// No package is downloaded or installed.
    pub fn assess(
        &self,
        packages: &[PackageRecord],
        providers: &SupplyChainProviders<'_>,
    ) -> SupplyChainResult<SupplyChainReport> {
        let mut report = SupplyChainReport::default();

        self.check_budget(packages.len(), &mut report)?;

        let mut index = BTreeMap::new();

        let mut edge_count = 0usize;

        for (position, package) in packages.iter().enumerate() {
            if index
                .insert(package.id().clone(), position)
                .is_some()
            {
                return Err(SupplyChainError::DuplicatePackage {
                    package: package.id().to_string(),
                });
            }

            edge_count = edge_count
                .checked_add(package.dependencies().len())
                .ok_or(SupplyChainError::ArithmeticOverflow)?;
        }

        if let Some(limit) =
            self.policy.budget.max_dependency_edges
        {
            if edge_count > limit {
                return Err(SupplyChainError::BudgetExceeded {
                    resource: "dependency-edge",
                    requested: edge_count,
                    permitted: limit,
                });
            }
        }

        report.packages_examined = packages.len();
        report.dependency_edges_examined = edge_count;

        self.verify_dependency_graph(
            packages,
            &index,
            &mut report,
        )?;

        for package in packages {
            self.verify_package(
                package,
                providers,
                &mut report,
            )?;
        }

        // Stable ordering independent of input order.
        report.findings.sort_by(|left, right| {
            (
                &left.package,
                &left.code,
                left.severity,
                &left.message,
            )
                .cmp(&(
                    &right.package,
                    &right.code,
                    right.severity,
                    &right.message,
                ))
        });

        Ok(report)
    }

    fn check_budget(
        &self,
        count: usize,
        _report: &mut SupplyChainReport,
    ) -> SupplyChainResult<()> {
        if let Some(limit) =
            self.policy.budget.max_packages
        {
            if count > limit {
                return Err(SupplyChainError::BudgetExceeded {
                    resource: "package",
                    requested: count,
                    permitted: limit,
                });
            }
        }

        Ok(())
    }

    // ========================================================================
    // GRAPH VALIDATION
    // ========================================================================

    fn verify_dependency_graph(
        &self,
        packages: &[PackageRecord],
        index: &BTreeMap<PackageId, usize>,
        report: &mut SupplyChainReport,
    ) -> SupplyChainResult<()> {
        let mut indegree = vec![0usize; packages.len()];

        let mut reverse_edges: Vec<Vec<usize>> =
            vec![Vec::new(); packages.len()];

        let mut graph_invalid = false;

        for (package_index, package) in
            packages.iter().enumerate()
        {
            let mut seen = BTreeSet::new();

            for dependency in package.dependencies() {
                if !seen.insert(dependency.package().clone()) {
                    report.push(SupplyChainFinding::new(
                        "SC-GRAPH-DUPLICATE-EDGE",
                        FindingSeverity::Error,
                        Some(package.id().clone()),
                        format!(
                            "duplicate dependency reference: {}",
                            dependency.package()
                        ),
                    ));

                    graph_invalid = true;
                    continue;
                }

                let Some(&dependency_index) =
                    index.get(dependency.package())
                else {
                    report.push(SupplyChainFinding::new(
                        "SC-GRAPH-MISSING-DEPENDENCY",
                        FindingSeverity::Error,
                        Some(package.id().clone()),
                        format!(
                            "dependency {} is missing from the resolved graph",
                            dependency.package()
                        ),
                    ));

                    graph_invalid = true;
                    continue;
                };

                indegree[package_index] =
                    indegree[package_index]
                        .checked_add(1)
                        .ok_or(
                            SupplyChainError::ArithmeticOverflow
                        )?;

                reverse_edges[dependency_index]
                    .push(package_index);
            }
        }

        if graph_invalid {
            return Ok(());
        }

        // Kahn's algorithm.
        //
        // This avoids recursive traversal and therefore avoids recursion
        // depth limits for large dependency graphs.

        let mut ready = BTreeSet::new();

        for (position, degree) in
            indegree.iter().enumerate()
        {
            if *degree == 0 {
                ready.insert(packages[position].id().clone());
            }
        }

        let mut visited = 0usize;

        while let Some(package_id) =
            ready.pop_first()
        {
            let Some(&position) = index.get(&package_id) else {
                continue;
            };

            visited = visited
                .checked_add(1)
                .ok_or(SupplyChainError::ArithmeticOverflow)?;

            report.topological_order.push(package_id);

            for dependent in &reverse_edges[position] {
                indegree[*dependent] =
                    indegree[*dependent]
                        .checked_sub(1)
                        .ok_or(
                            SupplyChainError::ArithmeticOverflow
                        )?;

                if indegree[*dependent] == 0 {
                    ready.insert(
                        packages[*dependent].id().clone()
                    );
                }
            }
        }

        if visited != packages.len() {
            let cyclic_packages: Vec<String> = packages
                .iter()
                .enumerate()
                .filter(|(position, _)| indegree[*position] != 0)
                .map(|(_, package)| package.id().to_string())
                .collect();

            report.push(SupplyChainFinding::new(
                "SC-GRAPH-CYCLE",
                FindingSeverity::Error,
                None,
                format!(
                    "dependency graph contains a cycle involving: {}",
                    cyclic_packages.join(", ")
                ),
            ));
        }

        Ok(())
    }

    // ========================================================================
    // PACKAGE VERIFICATION
    // ========================================================================

    fn verify_package(
        &self,
        package: &PackageRecord,
        providers: &SupplyChainProviders<'_>,
        report: &mut SupplyChainReport,
    ) -> SupplyChainResult<()> {
        self.verify_source(package, report);
        self.verify_revision(package, report);
        self.verify_license(package, report);
        self.verify_builder(package, report);
        self.verify_reproducibility(package, report);

        self.verify_digest(
            package,
            providers.content,
            report,
        );

        self.verify_signature(
            package,
            providers.signatures,
            report,
        );

        self.verify_attestation(
            package,
            providers.attestations,
            report,
        );

        self.verify_vulnerabilities(
            package,
            providers.vulnerabilities,
            report,
        );

        Ok(())
    }

    fn verify_source(
        &self,
        package: &PackageRecord,
        report: &mut SupplyChainReport,
    ) {
        let Some(source) = package.source() else {
            report.defer_or_reject(
                &self.policy,
                "SC-SOURCE-MISSING",
                Some(package.id().clone()),
                "package source information is missing",
            );

            return;
        };

        if !self.policy.allowed_sources.is_empty()
            && !self
                .policy
                .allowed_sources
                .contains(source.provider())
        {
            report.push(SupplyChainFinding::new(
                "SC-SOURCE-NOT-ALLOWED",
                FindingSeverity::Error,
                Some(package.id().clone()),
                format!(
                    "package source provider {} is not allowed",
                    source.provider()
                ),
            ));
        }
    }

    fn verify_revision(
        &self,
        package: &PackageRecord,
        report: &mut SupplyChainReport,
    ) {
        if self.policy.require_immutable_revision
            && package.revision().is_none()
        {
            report.defer_or_reject(
                &self.policy,
                "SC-REVISION-MISSING",
                Some(package.id().clone()),
                "immutable package revision is required",
            );
        }

        if let Some(revision) = package.revision() {
            if revision.trim().is_empty()
                || revision.chars().any(char::is_control)
            {
                report.push(SupplyChainFinding::new(
                    "SC-REVISION-INVALID",
                    FindingSeverity::Error,
                    Some(package.id().clone()),
                    "package revision is invalid",
                ));
            }
        }
    }

    fn verify_license(
        &self,
        package: &PackageRecord,
        report: &mut SupplyChainReport,
    ) {
        let Some(license) = package.license() else {
            if self.policy.require_license_information {
                report.defer_or_reject(
                    &self.policy,
                    "SC-LICENSE-MISSING",
                    Some(package.id().clone()),
                    "package license information is missing",
                );
            }

            return;
        };

        if self.policy.denied_licenses.contains(license) {
            report.push(SupplyChainFinding::new(
                "SC-LICENSE-DENIED",
                FindingSeverity::Error,
                Some(package.id().clone()),
                format!(
                    "license {license} is explicitly denied"
                ),
            ));

            return;
        }

        if !self.policy.allowed_licenses.is_empty()
            && !self.policy.allowed_licenses.contains(license)
        {
            report.push(SupplyChainFinding::new(
                "SC-LICENSE-NOT-ALLOWED",
                FindingSeverity::Error,
                Some(package.id().clone()),
                format!(
                    "license {license} is not in the permitted set"
                ),
            ));
        }
    }

    fn verify_builder(
        &self,
        package: &PackageRecord,
        report: &mut SupplyChainReport,
    ) {
        if self.policy.require_declared_builder
            && package.declared_builder.is_none()
        {
            report.defer_or_reject(
                &self.policy,
                "SC-BUILDER-MISSING",
                Some(package.id().clone()),
                "build identity is missing",
            );

            return;
        }

        if let Some(builder) = &package.declared_builder {
            if !self.policy.allowed_builders.is_empty()
                && !self.policy.allowed_builders.contains(
                    builder.as_ref()
                )
            {
                report.push(SupplyChainFinding::new(
                    "SC-BUILDER-NOT-ALLOWED",
                    FindingSeverity::Error,
                    Some(package.id().clone()),
                    format!(
                        "builder {builder} is not trusted by policy"
                    ),
                ));
            }
        }
    }

    fn verify_reproducibility(
        &self,
        package: &PackageRecord,
        report: &mut SupplyChainReport,
    ) {
        if self.policy.require_reproducible_build_claim
            && !package.reproducible_build_claim
        {
            report.defer_or_reject(
                &self.policy,
                "SC-REPRODUCIBILITY-UNDECLARED",
                Some(package.id().clone()),
                "reproducible-build evidence is required",
            );
        }
    }

    fn verify_digest(
        &self,
        package: &PackageRecord,
        verifier: Option<&dyn PackageContentVerifier>,
        report: &mut SupplyChainReport,
    ) {
        let Some(digest) = package.digest() else {
            if self.policy.require_package_digest {
                report.defer_or_reject(
                    &self.policy,
                    "SC-DIGEST-MISSING",
                    Some(package.id().clone()),
                    "package content digest is required",
                );
            }

            return;
        };

        let Some(verifier) = verifier else {
            report.defer_or_reject(
                &self.policy,
                "SC-DIGEST-VERIFIER-UNAVAILABLE",
                Some(package.id().clone()),
                "no content verifier is registered",
            );

            return;
        };

        match verifier.verify_content(package, digest) {
            Ok(ProviderVerdict::Verified { .. }) => {}

            Ok(ProviderVerdict::Rejected { reason }) => {
                report.push(SupplyChainFinding::new(
                    "SC-DIGEST-REJECTED",
                    FindingSeverity::Error,
                    Some(package.id().clone()),
                    reason,
                ));
            }

            Ok(ProviderVerdict::Indeterminate { reason }) => {
                report.defer_or_reject(
                    &self.policy,
                    "SC-DIGEST-INDETERMINATE",
                    Some(package.id().clone()),
                    reason,
                );
            }

            Err(reason) => {
                report.defer_or_reject(
                    &self.policy,
                    "SC-DIGEST-PROVIDER-FAILURE",
                    Some(package.id().clone()),
                    reason,
                );
            }
        }
    }

    fn verify_signature(
        &self,
        package: &PackageRecord,
        verifier: Option<&dyn PackageSignatureVerifier>,
        report: &mut SupplyChainReport,
    ) {
        let Some(evidence) = package.signature() else {
            if self.policy.require_package_signature {
                report.defer_or_reject(
                    &self.policy,
                    "SC-SIGNATURE-MISSING",
                    Some(package.id().clone()),
                    "package signature is required",
                );
            }

            return;
        };

        let Some(verifier) = verifier else {
            report.defer_or_reject(
                &self.policy,
                "SC-SIGNATURE-VERIFIER-UNAVAILABLE",
                Some(package.id().clone()),
                "no signature verification provider is registered",
            );

            return;
        };

        match verifier.verify_signature(package, evidence) {
            Ok(ProviderVerdict::Verified { authority }) => {
                if !self.policy.allowed_signers.is_empty()
                    && !self.policy.allowed_signers.contains(&authority)
                {
                    report.push(SupplyChainFinding::new(
                        "SC-SIGNER-NOT-ALLOWED",
                        FindingSeverity::Error,
                        Some(package.id().clone()),
                        format!(
                            "signature authority {authority} is not allowed"
                        ),
                    ));
                }
            }

            Ok(ProviderVerdict::Rejected { reason }) => {
                report.push(SupplyChainFinding::new(
                    "SC-SIGNATURE-INVALID",
                    FindingSeverity::Error,
                    Some(package.id().clone()),
                    reason,
                ));
            }

            Ok(ProviderVerdict::Indeterminate { reason }) => {
                report.defer_or_reject(
                    &self.policy,
                    "SC-SIGNATURE-INDETERMINATE",
                    Some(package.id().clone()),
                    reason,
                );
            }

            Err(reason) => {
                report.defer_or_reject(
                    &self.policy,
                    "SC-SIGNATURE-PROVIDER-FAILURE",
                    Some(package.id().clone()),
                    reason,
                );
            }
        }
    }

    fn verify_attestation(
        &self,
        package: &PackageRecord,
        verifier: Option<&dyn PackageAttestationVerifier>,
        report: &mut SupplyChainReport,
    ) {
        let Some(evidence) = package.attestation() else {
            if self.policy.require_attestation {
                report.defer_or_reject(
                    &self.policy,
                    "SC-ATTESTATION-MISSING",
                    Some(package.id().clone()),
                    "build attestation is required",
                );
            }

            return;
        };

        if evidence.subject() != package.id() {
            report.push(SupplyChainFinding::new(
                "SC-ATTESTATION-SUBJECT-MISMATCH",
                FindingSeverity::Error,
                Some(package.id().clone()),
                "attestation subject does not match package identity",
            ));

            return;
        }

        let Some(verifier) = verifier else {
            report.defer_or_reject(
                &self.policy,
                "SC-ATTESTATION-VERIFIER-UNAVAILABLE",
                Some(package.id().clone()),
                "no attestation verification provider is registered",
            );

            return;
        };

        match verifier.verify_attestation(package, evidence) {
            Ok(ProviderVerdict::Verified { .. }) => {}

            Ok(ProviderVerdict::Rejected { reason }) => {
                report.push(SupplyChainFinding::new(
                    "SC-ATTESTATION-INVALID",
                    FindingSeverity::Error,
                    Some(package.id().clone()),
                    reason,
                ));
            }

            Ok(ProviderVerdict::Indeterminate { reason }) => {
                report.defer_or_reject(
                    &self.policy,
                    "SC-ATTESTATION-INDETERMINATE",
                    Some(package.id().clone()),
                    reason,
                );
            }

            Err(reason) => {
                report.defer_or_reject(
                    &self.policy,
                    "SC-ATTESTATION-PROVIDER-FAILURE",
                    Some(package.id().clone()),
                    reason,
                );
            }
        }
    }

    fn verify_vulnerabilities(
        &self,
        package: &PackageRecord,
        provider: Option<&dyn VulnerabilityProvider>,
        report: &mut SupplyChainReport,
    ) {
        let Some(provider) = provider else {
            report.defer_or_reject(
                &self.policy,
                "SC-ADVISORY-PROVIDER-UNAVAILABLE",
                Some(package.id().clone()),
                "vulnerability intelligence is unavailable",
            );

            return;
        };

        match provider.query(package) {
            Ok(findings) => {
                for vulnerability in findings {
                    let severity = vulnerability.severity();

                    if severity >=
                        self.policy.maximum_vulnerability_severity
                    {
                        report.push(SupplyChainFinding::new(
                            "SC-VULNERABILITY-FOUND",
                            FindingSeverity::Error,
                            Some(package.id().clone()),
                            format!(
                                "{}: {}",
                                vulnerability.id(),
                                vulnerability.description
                            ),
                        ));
                    } else {
                        report.push(SupplyChainFinding::new(
                            "SC-VULNERABILITY-REVIEW",
                            FindingSeverity::Warning,
                            Some(package.id().clone()),
                            format!(
                                "{} affects range {}",
                                vulnerability.id(),
                                vulnerability.affected_range
                            ),
                        ));
                    }
                }
            }

            Err(reason) => {
                report.defer_or_reject(
                    &self.policy,
                    "SC-ADVISORY-PROVIDER-FAILURE",
                    Some(package.id().clone()),
                    reason,
                );
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn package(
        name: &str,
    ) -> PackageRecord {
        PackageRecord::new(
            PackageId::new(name).unwrap(),
            "1.0.0",
        )
        .unwrap()
    }

    #[test]
    fn package_identity_is_validated() {
        assert!(PackageId::new("cargo:serde").is_ok());

        assert!(PackageId::new("").is_err());

        assert!(PackageId::new(" invalid ").is_err());
    }

    #[test]
    fn duplicate_package_identities_are_rejected() {
        let verifier = SupplyChainVerifier::new(
            SupplyChainPolicy::default(),
        )
        .unwrap();

        let packages = vec![
            package("cargo:a"),
            package("cargo:a"),
        ];

        assert!(matches!(
            verifier.assess(
                &packages,
                &SupplyChainProviders::default(),
            ),
            Err(SupplyChainError::DuplicatePackage { .. })
        ));
    }

    #[test]
    fn missing_dependencies_are_rejected() {
        let verifier = SupplyChainVerifier::new(
            SupplyChainPolicy::default(),
        )
        .unwrap();

        let dependency = PackageDependency::new(
            PackageId::new("cargo:missing").unwrap(),
            "*",
        )
        .unwrap();

        let packages = vec![
            package("cargo:a").with_dependency(dependency),
        ];

        let report = verifier.assess(
            &packages,
            &SupplyChainProviders::default(),
        )
        .unwrap();

        assert_eq!(
            report.status(),
            SupplyChainStatus::Rejected
        );
    }

    #[test]
    fn dependency_cycles_are_detected() {
        let verifier = SupplyChainVerifier::new(
            SupplyChainPolicy::default(),
        )
        .unwrap();

        let a = PackageId::new("cargo:a").unwrap();
        let b = PackageId::new("cargo:b").unwrap();

        let packages = vec![
            package("cargo:a")
                .with_dependency(
                    PackageDependency::new(b, "*").unwrap(),
                ),

            package("cargo:b")
                .with_dependency(
                    PackageDependency::new(a, "*").unwrap(),
                ),
        ];

        let report = verifier.assess(
            &packages,
            &SupplyChainProviders::default(),
        )
        .unwrap();

        assert_eq!(
            report.status(),
            SupplyChainStatus::Rejected
        );

        assert!(report.findings().iter().any(|finding| {
            finding.code == "SC-GRAPH-CYCLE"
        }));
    }

    #[test]
    fn absent_verification_providers_never_prove_trust() {
        let verifier = SupplyChainVerifier::new(
            SupplyChainPolicy::default(),
        )
        .unwrap();

        let report = verifier.assess(
            &[package("cargo:a")],
            &SupplyChainProviders::default(),
        )
        .unwrap();

        assert!(!report.is_accepted());

        assert_eq!(
            report.status(),
            SupplyChainStatus::Rejected
        );
    }

    #[test]
    fn dependency_budget_is_configurable() {
        let mut policy = SupplyChainPolicy::default();

        policy.require_package_signature = false;
        policy.require_attestation = false;
        policy.require_package_digest = false;
        policy.require_declared_builder = false;
        policy.require_license_information = false;

        policy.budget.max_packages = Some(1);

        let verifier = SupplyChainVerifier::new(policy)
            .unwrap();

        let packages = vec![
            package("cargo:a"),
            package("cargo:b"),
        ];

        assert!(matches!(
            verifier.assess(
                &packages,
                &SupplyChainProviders::default(),
            ),
            Err(SupplyChainError::BudgetExceeded { .. })
        ));
    }

    #[test]
    fn graph_order_is_independent_of_input_order() {
        let verifier = SupplyChainVerifier::new(
            SupplyChainPolicy::default(),
        )
        .unwrap();

        let dependency = PackageId::new("cargo:b").unwrap();

        let a = package("cargo:a")
            .with_dependency(
                PackageDependency::new(dependency, "*").unwrap(),
            );

        let b = package("cargo:b");

        let report = verifier.assess(
            &[a.clone(), b.clone()],
            &SupplyChainProviders::default(),
        ).unwrap();

        let reordered = verifier.assess(
            &[b, a],
            &SupplyChainProviders::default(),
        ).unwrap();

        assert_eq!(
            report.topological_order(),
            reordered.topological_order()
        );
    }
}
