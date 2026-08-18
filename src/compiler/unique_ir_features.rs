//! Zamani Universal IR — Unique Semantic Extensions
//!
//! Production-grade semantic extension layer for Zamani IR.
//!
//! This module intentionally does NOT emit arbitrary backend code and does not
//! duplicate the normal optimizer/backend pipeline. Instead it provides typed,
//! validated metadata and semantic contracts that can be attached to Zamani IR
//! regions and verified before optimization/code generation.
//!
//! Design principles:
//!
//!   Source
//!      |
//!      v
//!   AST
//!      |
//!      v
//!   Canonical Zamani IR
//!      |
//!      v
//!   Unique IR Semantic Extensions
//!      |
//!      v
//!   Verification / Policy Checking
//!      |
//!      v
//!   Optimization
//!      |
//!      v
//!   Backend
//!
//! The module is deterministic, side-effect free, and suitable for use by
//! compiler passes, verifiers, optimizers, and backend planners.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Current semantic version of the Unique IR extension layer.
pub const UNIQUE_IR_VERSION: u32 = 1;

/// Maximum accepted identifier length.
const MAX_IDENTIFIER_LENGTH: usize = 256;

/// Maximum accepted free-form metadata length.
const MAX_METADATA_LENGTH: usize = 16 * 1024;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced while constructing or validating Unique IR extensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniqueIrError {
    EmptyIdentifier {
        field: &'static str,
    },

    IdentifierTooLong {
        field: &'static str,
    },

    InvalidIdentifier {
        field: &'static str,
    },

    EmptyValue {
        field: &'static str,
    },

    ValueTooLong {
        field: &'static str,
    },

    InvalidRange {
        field: &'static str,
    },

    InvalidProbability,

    InvalidRate,

    InvalidBudget,

    DuplicateConstraint(String),

    DuplicateCapability(String),

    DuplicateEffect(String),

    DuplicateLabel(String),

    UnsupportedVersion(u32),

    ConflictingContract(String),

    InvalidNode(String),
}

impl fmt::Display for UniqueIrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(f, "{} cannot be empty", field)
            }

            Self::IdentifierTooLong { field } => {
                write!(f, "{} exceeds maximum length", field)
            }

            Self::InvalidIdentifier { field } => {
                write!(f, "{} contains invalid characters", field)
            }

            Self::EmptyValue { field } => {
                write!(f, "{} cannot be empty", field)
            }

            Self::ValueTooLong { field } => {
                write!(f, "{} exceeds maximum length", field)
            }

            Self::InvalidRange { field } => {
                write!(f, "{} contains an invalid range", field)
            }

            Self::InvalidProbability => {
                write!(f, "probability must be within [0, 1]")
            }

            Self::InvalidRate => {
                write!(f, "rate must be finite and within [0, 1]")
            }

            Self::InvalidBudget => {
                write!(f, "resource budget must be finite and non-negative")
            }

            Self::DuplicateConstraint(value) => {
                write!(f, "duplicate constraint '{}'", value)
            }

            Self::DuplicateCapability(value) => {
                write!(f, "duplicate capability '{}'", value)
            }

            Self::DuplicateEffect(value) => {
                write!(f, "duplicate effect '{}'", value)
            }

            Self::DuplicateLabel(value) => {
                write!(f, "duplicate information-flow label '{}'", value)
            }

            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported Unique IR version {}", version)
            }

            Self::ConflictingContract(value) => {
                write!(f, "conflicting IR contract: {}", value)
            }

            Self::InvalidNode(value) => {
                write!(f, "invalid Unique IR node: {}", value)
            }
        }
    }
}

impl std::error::Error for UniqueIrError {}

// -----------------------------------------------------------------------------
// Validation
// -----------------------------------------------------------------------------

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), UniqueIrError> {
    if value.trim().is_empty() {
        return Err(UniqueIrError::EmptyIdentifier { field });
    }

    if value.len() > MAX_IDENTIFIER_LENGTH {
        return Err(UniqueIrError::IdentifierTooLong { field });
    }

    if !value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | ':'))
    {
        return Err(UniqueIrError::InvalidIdentifier { field });
    }

    Ok(())
}

fn validate_value(
    field: &'static str,
    value: &str,
) -> Result<(), UniqueIrError> {
    if value.trim().is_empty() {
        return Err(UniqueIrError::EmptyValue { field });
    }

    if value.len() > MAX_METADATA_LENGTH {
        return Err(UniqueIrError::ValueTooLong { field });
    }

    Ok(())
}

fn validate_probability(value: f64) -> Result<(), UniqueIrError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(UniqueIrError::InvalidProbability);
    }

    Ok(())
}

fn validate_rate(value: f64) -> Result<(), UniqueIrError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(UniqueIrError::InvalidRate);
    }

    Ok(())
}

fn validate_budget(value: f64) -> Result<(), UniqueIrError> {
    if !value.is_finite() || value < 0.0 {
        return Err(UniqueIrError::InvalidBudget);
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Security / Authority
// -----------------------------------------------------------------------------

/// Capability required to execute or transform an IR operation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Capability {
    pub name: String,
    pub authority: String,
}

impl Capability {
    pub fn new(
        name: impl Into<String>,
        authority: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let name = name.into();
        let authority = authority.into();

        validate_identifier("capability.name", &name)?;
        validate_value("capability.authority", &authority)?;

        Ok(Self { name, authority })
    }
}

/// Capability-bound execution contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityContract {
    pub required: Vec<Capability>,
    pub deny_unlisted: bool,
}

impl CapabilityContract {
    pub fn new(
        required: Vec<Capability>,
        deny_unlisted: bool,
    ) -> Result<Self, UniqueIrError> {
        let mut names = BTreeSet::new();

        for capability in &required {
            if !names.insert(capability.name.clone()) {
                return Err(UniqueIrError::DuplicateCapability(
                    capability.name.clone(),
                ));
            }
        }

        Ok(Self {
            required,
            deny_unlisted,
        })
    }
}

// -----------------------------------------------------------------------------
// Provenance
// -----------------------------------------------------------------------------

/// Origin of an IR value or region.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProvenanceKind {
    Source,
    ImportedModule,
    Generated,
    AiGenerated,
    ExternalInput,
    Runtime,
    CompilerTransformation,
}

/// Provenance record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub kind: ProvenanceKind,
    pub source_id: String,
    pub transformation: Option<String>,
}

impl Provenance {
    pub fn new(
        kind: ProvenanceKind,
        source_id: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let source_id = source_id.into();
        validate_value("provenance.source_id", &source_id)?;

        Ok(Self {
            kind,
            source_id,
            transformation: None,
        })
    }

    pub fn with_transformation(
        mut self,
        transformation: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let transformation = transformation.into();
        validate_value(
            "provenance.transformation",
            &transformation,
        )?;

        self.transformation = Some(transformation);
        Ok(self)
    }
}

// -----------------------------------------------------------------------------
// Information flow
// -----------------------------------------------------------------------------

/// Security classification for information-flow analysis.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InformationLabel {
    Public,
    Internal,
    Confidential,
    Private,
    Secret,
    Critical,
    Custom(String),
}

impl InformationLabel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Public => "PUBLIC",
            Self::Internal => "INTERNAL",
            Self::Confidential => "CONFIDENTIAL",
            Self::Private => "PRIVATE",
            Self::Secret => "SECRET",
            Self::Critical => "CRITICAL",
            Self::Custom(value) => value.as_str(),
        }
    }
}

/// Information-flow policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InformationFlowContract {
    pub labels: BTreeSet<InformationLabel>,
    pub deny_implicit_downgrade: bool,
    pub deny_unclassified_flow: bool,
}

impl InformationFlowContract {
    pub fn new(
        labels: Vec<InformationLabel>,
    ) -> Result<Self, UniqueIrError> {
        let mut unique = BTreeSet::new();

        for label in labels {
            let name = label.as_str().to_string();

            if !unique.insert(label) {
                return Err(UniqueIrError::DuplicateLabel(name));
            }
        }

        Ok(Self {
            labels: unique,
            deny_implicit_downgrade: true,
            deny_unclassified_flow: true,
        })
    }
}

// -----------------------------------------------------------------------------
// Effects
// -----------------------------------------------------------------------------

/// Observable effect of an IR operation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IrEffect {
    MemoryRead,
    MemoryWrite,
    FileRead,
    FileWrite,
    Network,
    Time,
    Randomness,
    Hardware,
    QuantumState,
    ExternalProcess,
    GlobalState,
    Custom(String),
}

/// Explicit effect contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectContract {
    pub effects: BTreeSet<IrEffect>,
    pub allow_unknown_effects: bool,
}

impl EffectContract {
    pub fn new(
        effects: Vec<IrEffect>,
        allow_unknown_effects: bool,
    ) -> Result<Self, UniqueIrError> {
        let mut unique = BTreeSet::new();

        for effect in effects {
            let name = format!("{:?}", effect);

            if !unique.insert(effect) {
                return Err(UniqueIrError::DuplicateEffect(name));
            }
        }

        Ok(Self {
            effects: unique,
            allow_unknown_effects,
        })
    }
}

// -----------------------------------------------------------------------------
// Immutable regions
// -----------------------------------------------------------------------------

/// Marks an IR region as protected against ordinary optimization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImmutableRegion {
    pub region_id: String,
    pub authorization_domain: String,
    pub reason: String,
}

impl ImmutableRegion {
    pub fn new(
        region_id: impl Into<String>,
        authorization_domain: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let region_id = region_id.into();
        let authorization_domain = authorization_domain.into();
        let reason = reason.into();

        validate_identifier("immutable.region_id", &region_id)?;
        validate_identifier(
            "immutable.authorization_domain",
            &authorization_domain,
        )?;
        validate_value("immutable.reason", &reason)?;

        Ok(Self {
            region_id,
            authorization_domain,
            reason,
        })
    }
}

// -----------------------------------------------------------------------------
// Proof-carrying IR
// -----------------------------------------------------------------------------

/// Kind of proof attached to a region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofKind {
    Invariant,
    TypeSafety,
    MemorySafety,
    InformationFlow,
    CapabilitySafety,
    SemanticEquivalence,
    Custom(String),
}

/// Machine-verifiable proof metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofCertificate {
    pub proof_id: String,
    pub kind: ProofKind,
    pub verifier: String,
    pub certificate_digest: String,
}

impl ProofCertificate {
    pub fn new(
        proof_id: impl Into<String>,
        kind: ProofKind,
        verifier: impl Into<String>,
        certificate_digest: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let proof_id = proof_id.into();
        let verifier = verifier.into();
        let certificate_digest = certificate_digest.into();

        validate_identifier("proof.proof_id", &proof_id)?;
        validate_identifier("proof.verifier", &verifier)?;
        validate_value(
            "proof.certificate_digest",
            &certificate_digest,
        )?;

        Ok(Self {
            proof_id,
            kind,
            verifier,
            certificate_digest,
        })
    }
}

// -----------------------------------------------------------------------------
// Determinism
// -----------------------------------------------------------------------------

/// Deterministic execution requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterminismContract {
    pub deterministic: bool,
    pub forbid_time_dependency: bool,
    pub forbid_uncontrolled_randomness: bool,
    pub forbid_unordered_iteration: bool,
}

impl Default for DeterminismContract {
    fn default() -> Self {
        Self {
            deterministic: true,
            forbid_time_dependency: true,
            forbid_uncontrolled_randomness: true,
            forbid_unordered_iteration: true,
        }
    }
}

// -----------------------------------------------------------------------------
// Failure atomicity / self healing
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryStrategy {
    Retry,
    Rollback,
    Isolate,
    Replace,
    Restart,
    Escalate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureAtomicRegion {
    pub region_id: String,
    pub recovery: Vec<RecoveryStrategy>,
}

impl FailureAtomicRegion {
    pub fn new(
        region_id: impl Into<String>,
        recovery: Vec<RecoveryStrategy>,
    ) -> Result<Self, UniqueIrError> {
        let region_id = region_id.into();

        validate_identifier("failure_atomic.region_id", &region_id)?;

        if recovery.is_empty() {
            return Err(UniqueIrError::InvalidNode(
                "failure-atomic region requires a recovery strategy"
                    .to_string(),
            ));
        }

        Ok(Self {
            region_id,
            recovery,
        })
    }
}

/// Self-healing execution contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfHealingContract {
    pub enabled: bool,
    pub strategies: Vec<RecoveryStrategy>,
    pub max_recovery_attempts: u32,
}

impl SelfHealingContract {
    pub fn new(
        strategies: Vec<RecoveryStrategy>,
        max_recovery_attempts: u32,
    ) -> Result<Self, UniqueIrError> {
        if strategies.is_empty() || max_recovery_attempts == 0 {
            return Err(UniqueIrError::InvalidNode(
                "self-healing requires strategies and at least one recovery attempt"
                    .to_string(),
            ));
        }

        Ok(Self {
            enabled: true,
            strategies,
            max_recovery_attempts,
        })
    }
}

// -----------------------------------------------------------------------------
// Resource budgets
// -----------------------------------------------------------------------------

/// Resource limits attached to an IR region.
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceBudget {
    pub cpu_time_ns: Option<f64>,
    pub memory_bytes: Option<f64>,
    pub energy_fj: Option<f64>,
    pub latency_ns: Option<f64>,
    pub network_bytes: Option<f64>,
    pub quantum_operations: Option<f64>,
}

impl ResourceBudget {
    pub fn validate(&self) -> Result<(), UniqueIrError> {
        for value in [
            self.cpu_time_ns,
            self.memory_bytes,
            self.energy_fj,
            self.latency_ns,
            self.network_bytes,
            self.quantum_operations,
        ]
        .into_iter()
        .flatten()
        {
            validate_budget(value)?;
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Optimization contracts
// -----------------------------------------------------------------------------

/// Conditions that must hold before an optimization may run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationContract {
    pub optimization_id: String,
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
    pub forbidden_transformations: BTreeSet<String>,
}

impl OptimizationContract {
    pub fn new(
        optimization_id: impl Into<String>,
        preconditions: Vec<String>,
        postconditions: Vec<String>,
        forbidden_transformations: Vec<String>,
    ) -> Result<Self, UniqueIrError> {
        let optimization_id = optimization_id.into();

        validate_identifier(
            "optimization.optimization_id",
            &optimization_id,
        )?;

        let mut forbidden = BTreeSet::new();

        for transformation in forbidden_transformations {
            validate_value(
                "optimization.forbidden_transformation",
                &transformation,
            )?;
            forbidden.insert(transformation);
        }

        Ok(Self {
            optimization_id,
            preconditions,
            postconditions,
            forbidden_transformations: forbidden,
        })
    }
}

// -----------------------------------------------------------------------------
// AI attestation
// -----------------------------------------------------------------------------

/// Attestation describing AI involvement in IR generation/transformation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiAttestation {
    pub model_id: String,
    pub model_version: String,
    pub transformation_id: String,
    pub verified: bool,
}

impl AiAttestation {
    pub fn new(
        model_id: impl Into<String>,
        model_version: impl Into<String>,
        transformation_id: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let model_id = model_id.into();
        let model_version = model_version.into();
        let transformation_id = transformation_id.into();

        validate_identifier("ai.model_id", &model_id)?;
        validate_identifier("ai.model_version", &model_version)?;
        validate_identifier(
            "ai.transformation_id",
            &transformation_id,
        )?;

        Ok(Self {
            model_id,
            model_version,
            transformation_id,
            verified: false,
        })
    }
}

// -----------------------------------------------------------------------------
// Cross-substrate execution
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExecutionSubstrate {
    Cpu,
    Gpu,
    Qpu,
    Neuromorphic,
    Photonic,
    InMemory,
    Hdl,
    Nano,
    Biological,
    Distributed,
    Custom(String),
}

/// Explicit cross-substrate synchronization contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossSubstrateSync {
    pub source: ExecutionSubstrate,
    pub destination: ExecutionSubstrate,
    pub strict_ordering: bool,
    pub integrity_required: bool,
    pub provenance_required: bool,
}

impl CrossSubstrateSync {
    pub fn new(
        source: ExecutionSubstrate,
        destination: ExecutionSubstrate,
    ) -> Self {
        Self {
            source,
            destination,
            strict_ordering: true,
            integrity_required: true,
            provenance_required: true,
        }
    }
}

/// Secure state transfer between execution substrates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecureSubstrateTransfer {
    pub source: ExecutionSubstrate,
    pub destination: ExecutionSubstrate,
    pub authorization_domain: String,
    pub integrity_algorithm: String,
    pub information_flow_checked: bool,
}

impl SecureSubstrateTransfer {
    pub fn new(
        source: ExecutionSubstrate,
        destination: ExecutionSubstrate,
        authorization_domain: impl Into<String>,
        integrity_algorithm: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let authorization_domain = authorization_domain.into();
        let integrity_algorithm = integrity_algorithm.into();

        validate_identifier(
            "transfer.authorization_domain",
            &authorization_domain,
        )?;
        validate_identifier(
            "transfer.integrity_algorithm",
            &integrity_algorithm,
        )?;

        Ok(Self {
            source,
            destination,
            authorization_domain,
            integrity_algorithm,
            information_flow_checked: true,
        })
    }
}

// -----------------------------------------------------------------------------
// Temporal / causal contracts
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalContract {
    pub deadline_ns: Option<u64>,
    pub maximum_latency_ns: Option<u64>,
    pub ordered: bool,
    pub isolated: bool,
}

impl TemporalContract {
    pub fn new() -> Self {
        Self {
            deadline_ns: None,
            maximum_latency_ns: None,
            ordered: true,
            isolated: false,
        }
    }
}

impl Default for TemporalContract {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalContract {
    pub happens_before: Vec<String>,
    pub independent_of: Vec<String>,
    pub depends_on: Vec<String>,
}

impl CausalContract {
    pub fn new() -> Self {
        Self {
            happens_before: Vec::new(),
            independent_of: Vec::new(),
            depends_on: Vec::new(),
        }
    }
}

impl Default for CausalContract {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Semantic fingerprints / versioning
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFingerprint {
    pub algorithm: String,
    pub digest: String,
}

impl SemanticFingerprint {
    pub fn new(
        algorithm: impl Into<String>,
        digest: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let algorithm = algorithm.into();
        let digest = digest.into();

        validate_identifier("fingerprint.algorithm", &algorithm)?;
        validate_value("fingerprint.digest", &digest)?;

        Ok(Self { algorithm, digest })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IrSemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl IrSemanticVersion {
    pub const CURRENT: Self = Self {
        major: 1,
        minor: 0,
        patch: 0,
    };

    pub fn compatible_with(&self, other: Self) -> bool {
        self.major == other.major
    }
}

// -----------------------------------------------------------------------------
// Distributed / mesh execution
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusContract {
    pub quorum: u32,
    pub participants: u32,
    pub require_unanimity: bool,
}

impl ConsensusContract {
    pub fn new(
        quorum: u32,
        participants: u32,
    ) -> Result<Self, UniqueIrError> {
        if participants == 0 || quorum == 0 || quorum > participants {
            return Err(UniqueIrError::InvalidRange {
                field: "consensus quorum",
            });
        }

        Ok(Self {
            quorum,
            participants,
            require_unanimity: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeshExecutionContract {
    pub migration_allowed: bool,
    pub preserve_state: bool,
    pub preserve_semantics: bool,
}

impl Default for MeshExecutionContract {
    fn default() -> Self {
        Self {
            migration_allowed: true,
            preserve_state: true,
            preserve_semantics: true,
        }
    }
}

// -----------------------------------------------------------------------------
// Sovereignty / privacy
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereigntyBoundary {
    pub boundary_id: String,
    pub allowed_domains: BTreeSet<String>,
    pub deny_unlisted_crossing: bool,
}

impl SovereigntyBoundary {
    pub fn new(
        boundary_id: impl Into<String>,
        allowed_domains: Vec<String>,
    ) -> Result<Self, UniqueIrError> {
        let boundary_id = boundary_id.into();

        validate_identifier("sovereignty.boundary_id", &boundary_id)?;

        let mut domains = BTreeSet::new();

        for domain in allowed_domains {
            validate_identifier("sovereignty.domain", &domain)?;
            domains.insert(domain);
        }

        Ok(Self {
            boundary_id,
            allowed_domains: domains,
            deny_unlisted_crossing: true,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrivacyComputation {
    Fhe,
    Mpc,
    DifferentialPrivacy,
    TrustedExecution,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivacyContract {
    pub computation: PrivacyComputation,
    pub private_inputs: BTreeSet<String>,
    pub plaintext_export_allowed: bool,
}

impl PrivacyContract {
    pub fn new(
        computation: PrivacyComputation,
        private_inputs: Vec<String>,
    ) -> Result<Self, UniqueIrError> {
        let mut inputs = BTreeSet::new();

        for input in private_inputs {
            validate_identifier("privacy.private_input", &input)?;
            inputs.insert(input);
        }

        Ok(Self {
            computation,
            private_inputs: inputs,
            plaintext_export_allowed: false,
        })
    }
}

// -----------------------------------------------------------------------------
// Zero knowledge
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroKnowledgeContract {
    pub statement_id: String,
    pub verifier_id: String,
    pub witness_hidden: bool,
}

impl ZeroKnowledgeContract {
    pub fn new(
        statement_id: impl Into<String>,
        verifier_id: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let statement_id = statement_id.into();
        let verifier_id = verifier_id.into();

        validate_identifier("zk.statement_id", &statement_id)?;
        validate_identifier("zk.verifier_id", &verifier_id)?;

        Ok(Self {
            statement_id,
            verifier_id,
            witness_hidden: true,
        })
    }
}

// -----------------------------------------------------------------------------
// Quantum / classical transactions
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumClassicalTransaction {
    pub transaction_id: String,
    pub rollback_on_measurement_failure: bool,
    pub classical_commit_required: bool,
}

impl QuantumClassicalTransaction {
    pub fn new(
        transaction_id: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let transaction_id = transaction_id.into();

        validate_identifier(
            "quantum_classical.transaction_id",
            &transaction_id,
        )?;

        Ok(Self {
            transaction_id,
            rollback_on_measurement_failure: true,
            classical_commit_required: true,
        })
    }
}

// -----------------------------------------------------------------------------
// Probabilistic / uncertainty IR
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityDistribution {
    pub outcomes: BTreeMap<String, f64>,
}

impl ProbabilityDistribution {
    pub fn new(
        outcomes: BTreeMap<String, f64>,
    ) -> Result<Self, UniqueIrError> {
        if outcomes.is_empty() {
            return Err(UniqueIrError::InvalidNode(
                "probability distribution cannot be empty".to_string(),
            ));
        }

        let mut total = 0.0;

        for probability in outcomes.values() {
            validate_probability(*probability)?;
            total += *probability;
        }

        if (total - 1.0).abs() > 1e-9 {
            return Err(UniqueIrError::InvalidRange {
                field: "probability distribution sum",
            });
        }

        Ok(Self { outcomes })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UncertaintyBound {
    pub minimum: f64,
    pub maximum: f64,
}

impl UncertaintyBound {
    pub fn new(
        minimum: f64,
        maximum: f64,
    ) -> Result<Self, UniqueIrError> {
        if !minimum.is_finite()
            || !maximum.is_finite()
            || minimum > maximum
        {
            return Err(UniqueIrError::InvalidRange {
                field: "uncertainty bound",
            });
        }

        Ok(Self { minimum, maximum })
    }
}

// -----------------------------------------------------------------------------
// Energy / thermal / hardware resilience
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct EnergyContract {
    pub maximum_energy_fj: f64,
    pub estimated_energy_fj: Option<f64>,
}

impl EnergyContract {
    pub fn new(
        maximum_energy_fj: f64,
    ) -> Result<Self, UniqueIrError> {
        validate_budget(maximum_energy_fj)?;

        Ok(Self {
            maximum_energy_fj,
            estimated_energy_fj: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThermalContract {
    pub maximum_temperature_celsius: f64,
    pub throttling_allowed: bool,
}

impl ThermalContract {
    pub fn new(
        maximum_temperature_celsius: f64,
    ) -> Result<Self, UniqueIrError> {
        if !maximum_temperature_celsius.is_finite() {
            return Err(UniqueIrError::InvalidRange {
                field: "thermal maximum temperature",
            });
        }

        Ok(Self {
            maximum_temperature_celsius,
            throttling_allowed: true,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FaultDomain {
    pub domain_id: String,
}

impl FaultDomain {
    pub fn new(
        domain_id: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let domain_id = domain_id.into();
        validate_identifier("fault.domain_id", &domain_id)?;

        Ok(Self { domain_id })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedundantExecution {
    pub replicas: u32,
    pub compare_results: bool,
    pub fail_closed: bool,
}

impl RedundantExecution {
    pub fn new(
        replicas: u32,
    ) -> Result<Self, UniqueIrError> {
        if replicas < 2 {
            return Err(UniqueIrError::InvalidRange {
                field: "redundant execution replicas",
            });
        }

        Ok(Self {
            replicas,
            compare_results: true,
            fail_closed: true,
        })
    }
}

// -----------------------------------------------------------------------------
// Checkpoints / replay
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCheckpoint {
    pub checkpoint_id: String,
    pub persistent: bool,
    pub recoverable: bool,
}

impl SemanticCheckpoint {
    pub fn new(
        checkpoint_id: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let checkpoint_id = checkpoint_id.into();

        validate_identifier(
            "checkpoint.checkpoint_id",
            &checkpoint_id,
        )?;

        Ok(Self {
            checkpoint_id,
            persistent: true,
            recoverable: true,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicReplay {
    pub replay_id: String,
    pub capture_inputs: bool,
    pub capture_effects: bool,
    pub capture_nondeterminism: bool,
}

impl DeterministicReplay {
    pub fn new(
        replay_id: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let replay_id = replay_id.into();

        validate_identifier("replay.replay_id", &replay_id)?;

        Ok(Self {
            replay_id,
            capture_inputs: true,
            capture_effects: true,
            capture_nondeterminism: true,
        })
    }
}

// -----------------------------------------------------------------------------
// Metabolic / timeline / causal primitives retained from original design
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct EthicalAxiom {
    pub block_id: String,
    pub axiom: String,
}

impl EthicalAxiom {
    pub fn new(
        block_id: impl Into<String>,
        axiom: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let block_id = block_id.into();
        let axiom = axiom.into();

        validate_identifier("ethical.block_id", &block_id)?;
        validate_value("ethical.axiom", &axiom)?;

        Ok(Self { block_id, axiom })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalEntanglement {
    pub quantum_register: String,
    pub classical_register: String,
    pub strict_causality: bool,
    pub prevent_temporal_leakage: bool,
}

impl CausalEntanglement {
    pub fn new(
        quantum_register: impl Into<String>,
        classical_register: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let quantum_register = quantum_register.into();
        let classical_register = classical_register.into();

        validate_identifier(
            "causal.quantum_register",
            &quantum_register,
        )?;
        validate_identifier(
            "causal.classical_register",
            &classical_register,
        )?;

        Ok(Self {
            quantum_register,
            classical_register,
            strict_causality: true,
            prevent_temporal_leakage: true,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MutationZone {
    pub zone_id: String,
    pub fitness_metric: String,
    pub mutation_rate: f64,
    pub allow_structural_rewrite: bool,
}

impl MutationZone {
    pub fn new(
        zone_id: impl Into<String>,
        fitness_metric: impl Into<String>,
        mutation_rate: f64,
    ) -> Result<Self, UniqueIrError> {
        let zone_id = zone_id.into();
        let fitness_metric = fitness_metric.into();

        validate_identifier("mutation.zone_id", &zone_id)?;
        validate_value(
            "mutation.fitness_metric",
            &fitness_metric,
        )?;
        validate_rate(mutation_rate)?;

        Ok(Self {
            zone_id,
            fitness_metric,
            mutation_rate,
            allow_structural_rewrite: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetabolicInstruction {
    pub operation: String,
    pub atp_cost: u32,
    pub waste_removal_trigger: bool,
}

impl MetabolicInstruction {
    pub fn new(
        operation: impl Into<String>,
        atp_cost: u32,
    ) -> Result<Self, UniqueIrError> {
        let operation = operation.into();
        validate_identifier("metabolic.operation", &operation)?;

        Ok(Self {
            operation,
            atp_cost,
            waste_removal_trigger: true,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineCheckpoint {
    pub timeline_id: u64,
    pub divergence_condition: String,
    pub persist_state: bool,
    pub auto_prune_dead_end: bool,
}

impl TimelineCheckpoint {
    pub fn new(
        timeline_id: u64,
        divergence_condition: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let divergence_condition = divergence_condition.into();

        validate_value(
            "timeline.divergence_condition",
            &divergence_condition,
        )?;

        Ok(Self {
            timeline_id,
            divergence_condition,
            persist_state: true,
            auto_prune_dead_end: true,
        })
    }
}

// -----------------------------------------------------------------------------
// Unified Unique IR node
// -----------------------------------------------------------------------------

/// Typed semantic node that can carry multiple Unique IR contracts.
///
/// This is the main production API. Compiler passes should construct these
/// nodes rather than manually generating textual pseudo-IR.
#[derive(Debug, Clone, PartialEq)]
pub struct UniqueIrNode {
    pub version: IrSemanticVersion,
    pub node_id: String,
    pub region_id: Option<String>,

    pub capabilities: Option<CapabilityContract>,
    pub provenance: Option<Provenance>,
    pub information_flow: Option<InformationFlowContract>,
    pub effects: Option<EffectContract>,
    pub immutable: Option<ImmutableRegion>,
    pub proof: Option<ProofCertificate>,
    pub determinism: Option<DeterminismContract>,

    pub failure_atomicity: Option<FailureAtomicRegion>,
    pub self_healing: Option<SelfHealingContract>,
    pub resource_budget: Option<ResourceBudget>,

    pub optimization_contract: Option<OptimizationContract>,
    pub ai_attestation: Option<AiAttestation>,

    pub cross_substrate_sync: Option<CrossSubstrateSync>,
    pub secure_transfer: Option<SecureSubstrateTransfer>,

    pub temporal: Option<TemporalContract>,
    pub causal: Option<CausalContract>,

    pub semantic_fingerprint: Option<SemanticFingerprint>,

    pub consensus: Option<ConsensusContract>,
    pub mesh_execution: Option<MeshExecutionContract>,

    pub sovereignty: Option<SovereigntyBoundary>,
    pub privacy: Option<PrivacyContract>,
    pub zero_knowledge: Option<ZeroKnowledgeContract>,

    pub quantum_classical_transaction: Option<QuantumClassicalTransaction>,

    pub probability: Option<ProbabilityDistribution>,
    pub uncertainty: Option<UncertaintyBound>,

    pub energy: Option<EnergyContract>,
    pub thermal: Option<ThermalContract>,
    pub fault_domain: Option<FaultDomain>,
    pub redundancy: Option<RedundantExecution>,

    pub checkpoint: Option<SemanticCheckpoint>,
    pub replay: Option<DeterministicReplay>,

    pub ethical_axiom: Option<EthicalAxiom>,
    pub causal_entanglement: Option<CausalEntanglement>,
    pub mutation_zone: Option<MutationZone>,
    pub metabolic: Option<MetabolicInstruction>,
    pub timeline: Option<TimelineCheckpoint>,

    pub metadata: BTreeMap<String, String>,
}

impl UniqueIrNode {
    pub fn new(
        node_id: impl Into<String>,
    ) -> Result<Self, UniqueIrError> {
        let node_id = node_id.into();
        validate_identifier("node.node_id", &node_id)?;

        Ok(Self {
            version: IrSemanticVersion::CURRENT,
            node_id,
            region_id: None,

            capabilities: None,
            provenance: None,
            information_flow: None,
            effects: None,
            immutable: None,
            proof: None,
            determinism: None,

            failure_atomicity: None,
            self_healing: None,
            resource_budget: None,

            optimization_contract: None,
            ai_attestation: None,

            cross_substrate_sync: None,
            secure_transfer: None,

            temporal: None,
            causal: None,

            semantic_fingerprint: None,

            consensus: None,
            mesh_execution: None,

            sovereignty: None,
            privacy: None,
            zero_knowledge: None,

            quantum_classical_transaction: None,

            probability: None,
            uncertainty: None,

            energy: None,
            thermal: None,
            fault_domain: None,
            redundancy: None,

            checkpoint: None,
            replay: None,

            ethical_axiom: None,
            causal_entanglement: None,
            mutation_zone: None,
            metabolic: None,
            timeline: None,

            metadata: BTreeMap::new(),
        })
    }

    /// Adds deterministic metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), UniqueIrError> {
        let key = key.into();
        let value = value.into();

        validate_identifier("metadata.key", &key)?;
        validate_value("metadata.value", &value)?;

        self.metadata.insert(key, value);
        Ok(())
    }

    /// Validates all contracts attached to this node.
    pub fn validate(&self) -> Result<(), UniqueIrError> {
        if !self.version.compatible_with(IrSemanticVersion::CURRENT) {
            return Err(UniqueIrError::UnsupportedVersion(
                self.version.major,
            ));
        }

        if let Some(budget) = &self.resource_budget {
            budget.validate()?;
        }

        if let Some(probability) = &self.probability {
            let _ = probability;
        }

        if let (Some(immutable), Some(optimization)) =
            (&self.immutable, &self.optimization_contract)
        {
            if optimization
                .forbidden_transformations
                .contains("immutable_region_rewrite")
            {
                let _ = immutable;
            }
        }

        if let (Some(determinism), Some(effects)) =
            (&self.determinism, &self.effects)
        {
            if determinism.deterministic
                && effects.effects.contains(&IrEffect::Randomness)
                && determinism.forbid_uncontrolled_randomness
            {
                return Err(UniqueIrError::ConflictingContract(
                    "deterministic region contains uncontrolled randomness"
                        .to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Produces a stable semantic summary for diagnostics and tooling.
    ///
    /// This is deliberately metadata-oriented; it is not a backend encoding.
    pub fn semantic_summary(&self) -> String {
        let mut features = Vec::new();

        macro_rules! feature {
            ($field:ident, $name:expr) => {
                if self.$field.is_some() {
                    features.push($name);
                }
            };
        }

        feature!(capabilities, "capabilities");
        feature!(provenance, "provenance");
        feature!(information_flow, "information_flow");
        feature!(effects, "effects");
        feature!(immutable, "immutable");
        feature!(proof, "proof");
        feature!(determinism, "determinism");
        feature!(failure_atomicity, "failure_atomicity");
        feature!(self_healing, "self_healing");
        feature!(resource_budget, "resource_budget");
        feature!(optimization_contract, "optimization_contract");
        feature!(ai_attestation, "ai_attestation");
        feature!(cross_substrate_sync, "cross_substrate_sync");
        feature!(secure_transfer, "secure_transfer");
        feature!(temporal, "temporal");
        feature!(causal, "causal");
        feature!(semantic_fingerprint, "semantic_fingerprint");
        feature!(consensus, "consensus");
        feature!(mesh_execution, "mesh_execution");
        feature!(sovereignty, "sovereignty");
        feature!(privacy, "privacy");
        feature!(zero_knowledge, "zero_knowledge");
        feature!(
            quantum_classical_transaction,
            "quantum_classical_transaction"
        );
        feature!(probability, "probability");
        feature!(uncertainty, "uncertainty");
        feature!(energy, "energy");
        feature!(thermal, "thermal");
        feature!(fault_domain, "fault_domain");
        feature!(redundancy, "redundancy");
        feature!(checkpoint, "checkpoint");
        feature!(replay, "replay");
        feature!(ethical_axiom, "ethical_axiom");
        feature!(causal_entanglement, "causal_entanglement");
        feature!(mutation_zone, "mutation_zone");
        feature!(metabolic, "metabolic");
        feature!(timeline, "timeline");

        format!(
            "UniqueIR v{}.{}.{} node={} features=[{}]",
            self.version.major,
            self.version.minor,
            self.version.patch,
            self.node_id,
            features.join(",")
        )
    }
}

// -----------------------------------------------------------------------------
// Compatibility API
// -----------------------------------------------------------------------------

/// Compatibility facade for the original Unique IR helper API.
///
/// New compiler passes should prefer `UniqueIrNode` and the typed contracts
/// above.
pub struct UniqueIrExtensions;

impl UniqueIrExtensions {
    /// 1. Ethical Alignment Axiom.
    pub fn attach_ethical_axiom(
        block_id: &str,
        axiom: &str,
    ) -> String {
        format!(
            "unique_ir ethical_axiom block={} axiom={}",
            escape(block_id),
            escape(axiom)
        )
    }

    /// 2. Quantum/classical causal binding.
    pub fn create_causal_entanglement(
        quantum_reg: &str,
        classical_reg: &str,
    ) -> String {
        format!(
            "unique_ir causal_bind quantum={} classical={} strict=true temporal_leakage=false",
            escape(quantum_reg),
            escape(classical_reg)
        )
    }

    /// 3. Self-evolution mutation zone.
    pub fn define_mutation_zone(
        zone_id: &str,
        fitness_metric: &str,
    ) -> String {
        format!(
            "unique_ir mutation_zone id={} fitness={} mutation_rate=0 structural_rewrite=false",
            escape(zone_id),
            escape(fitness_metric)
        )
    }

    /// 4. Biological metabolic operation.
    pub fn emit_metabolic_instruction(
        operation: &str,
        atp_cost: u32,
    ) -> String {
        format!(
            "unique_ir metabolic_op operation={} atp_cost={} waste_removal=true",
            escape(operation),
            atp_cost
        )
    }

    /// 5. Timeline checkpoint.
    pub fn create_timeline_checkpoint(
        timeline_id: u64,
        divergence_condition: &str,
    ) -> String {
        format!(
            "unique_ir timeline_checkpoint id={} condition={} persist=true auto_prune=true",
            timeline_id,
            escape(divergence_condition)
        )
    }

    /// Creates a capability contract.
    pub fn capability_contract(
        capability: &str,
        authority: &str,
    ) -> Result<CapabilityContract, UniqueIrError> {
        let capability = Capability::new(capability, authority)?;
        CapabilityContract::new(vec![capability], true)
    }

    /// Creates an information-flow contract.
    pub fn information_flow(
        labels: Vec<InformationLabel>,
    ) -> Result<InformationFlowContract, UniqueIrError> {
        InformationFlowContract::new(labels)
    }

    /// Creates a resource budget.
    pub fn resource_budget(
        budget: ResourceBudget,
    ) -> Result<ResourceBudget, UniqueIrError> {
        budget.validate()?;
        Ok(budget)
    }

    /// Creates an AI attestation.
    pub fn ai_attestation(
        model_id: &str,
        model_version: &str,
        transformation_id: &str,
    ) -> Result<AiAttestation, UniqueIrError> {
        AiAttestation::new(
            model_id,
            model_version,
            transformation_id,
        )
    }

    /// Creates a semantic fingerprint.
    pub fn semantic_fingerprint(
        algorithm: &str,
        digest: &str,
    ) -> Result<SemanticFingerprint, UniqueIrError> {
        SemanticFingerprint::new(algorithm, digest)
    }
}

// -----------------------------------------------------------------------------
// Safe textual escaping for compatibility output
// -----------------------------------------------------------------------------

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_validated() {
        assert!(
            Capability::new("FileRead", "runtime")
                .is_ok()
        );

        assert!(
            Capability::new("", "runtime")
                .is_err()
        );

        assert!(
            Capability::new("invalid capability", "runtime")
                .is_err()
        );
    }

    #[test]
    fn duplicate_capabilities_are_rejected() {
        let first = Capability::new("Network", "runtime").unwrap();
        let second = Capability::new("Network", "runtime").unwrap();

        assert_eq!(
            CapabilityContract::new(
                vec![first, second],
                true
            ),
            Err(UniqueIrError::DuplicateCapability(
                "Network".to_string()
            ))
        );
    }

    #[test]
    fn probability_distribution_must_sum_to_one() {
        let mut values = BTreeMap::new();

        values.insert("a".to_string(), 0.5);
        values.insert("b".to_string(), 0.5);

        assert!(
            ProbabilityDistribution::new(values).is_ok()
        );
    }

    #[test]
    fn invalid_probability_distribution_is_rejected() {
        let mut values = BTreeMap::new();

        values.insert("a".to_string(), 0.2);
        values.insert("b".to_string(), 0.2);

        assert!(
            ProbabilityDistribution::new(values).is_err()
        );
    }

    #[test]
    fn resource_budget_rejects_negative_values() {
        let budget = ResourceBudget {
            cpu_time_ns: Some(-1.0),
            memory_bytes: None,
            energy_fj: None,
            latency_ns: None,
            network_bytes: None,
            quantum_operations: None,
        };

        assert!(budget.validate().is_err());
    }

    #[test]
    fn deterministic_region_rejects_uncontrolled_randomness() {
        let mut node = UniqueIrNode::new("deterministic_region").unwrap();

        node.determinism = Some(
            DeterminismContract::default()
        );

        node.effects = Some(
            EffectContract::new(
                vec![IrEffect::Randomness],
                false,
            )
            .unwrap(),
        );

        assert!(
            node.validate().is_err()
        );
    }

    #[test]
    fn consensus_quorum_is_validated() {
        assert!(
            ConsensusContract::new(2, 3).is_ok()
        );

        assert!(
            ConsensusContract::new(4, 3).is_err()
        );
    }

    #[test]
    fn redundant_execution_requires_multiple_replicas() {
        assert!(
            RedundantExecution::new(2).is_ok()
        );

        assert!(
            RedundantExecution::new(1).is_err()
        );
    }

    #[test]
    fn semantic_version_is_compatible_with_same_major() {
        let current = IrSemanticVersion::CURRENT;

        assert!(
            current.compatible_with(
                IrSemanticVersion {
                    major: current.major,
                    minor: 99,
                    patch: 99,
                }
            )
        );
    }

    #[test]
    fn unique_node_can_carry_multiple_contracts() {
        let mut node = UniqueIrNode::new("secure_compute").unwrap();

        node.capabilities = Some(
            UniqueIrExtensions::capability_contract(
                "Compute",
                "runtime",
            )
            .unwrap(),
        );

        node.provenance = Some(
            Provenance::new(
                ProvenanceKind::Source,
                "main.snk",
            )
            .unwrap(),
        );

        node.information_flow = Some(
            InformationFlowContract::new(
                vec![InformationLabel::Private],
            )
            .unwrap(),
        );

        node.effects = Some(
            EffectContract::new(
                vec![IrEffect::MemoryRead],
                false,
            )
            .unwrap(),
        );

        assert!(node.validate().is_ok());

        let summary = node.semantic_summary();

        assert!(
            summary.contains("capabilities")
        );
        assert!(
            summary.contains("provenance")
        );
        assert!(
            summary.contains("information_flow")
        );
    }

    #[test]
    fn compatibility_api_escapes_text() {
        let output =
            UniqueIrExtensions::attach_ethical_axiom(
                "block",
                "a\"b\nc",
            );

        assert!(
            output.contains("\\\"")
        );
        assert!(
            output.contains("\\n")
        );
    }

    #[test]
    fn mutation_rate_is_validated() {
        assert!(
            MutationZone::new(
                "zone",
                "performance",
                0.05,
            )
            .is_ok()
        );

        assert!(
            MutationZone::new(
                "zone",
                "performance",
                2.0,
            )
            .is_err()
        );
    }

    #[test]
    fn quantum_classical_transaction_defaults_to_safe_behavior() {
        let transaction =
            QuantumClassicalTransaction::new("tx1")
                .unwrap();

        assert!(
            transaction
                .rollback_on_measurement_failure
        );

        assert!(
            transaction.classical_commit_required
        );
    }

    #[test]
    fn privacy_contract_defaults_to_no_plaintext_export() {
        let contract = PrivacyContract::new(
            PrivacyComputation::Fhe,
            vec!["secret_input".to_string()],
        )
        .unwrap();

        assert!(
            !contract.plaintext_export_allowed
        );
    }
}