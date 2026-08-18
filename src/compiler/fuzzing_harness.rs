//! Zamani Compiler — Cross-Substrate Fuzzing Harness (CSFH)
//!
//! Production-oriented, deterministic fuzzing infrastructure for discovering
//! malformed, unsupported, or unsafe cross-substrate instruction sequences.
//!
//! The harness deliberately separates:
//!
//! 1. Candidate generation
//! 2. Safety validation
//! 3. Execution/simulation
//! 4. Result classification
//! 5. Campaign accounting
//!
//! It does NOT directly execute arbitrary native, quantum, or neuromorphic
//! instructions. Real substrate execution must be supplied through an
//! explicit `FuzzExecutor` implementation.
//!
//! This keeps the compiler fuzzing layer deterministic, testable, and safe.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::{Duration, Instant};

// -----------------------------------------------------------------------------
// Limits and configuration
// -----------------------------------------------------------------------------

/// Resource limits protecting the compiler from unbounded fuzzing campaigns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuzzLimits {
    /// Maximum number of generated candidates.
    pub max_candidates: usize,

    /// Maximum number of stages in one candidate.
    pub max_stages: usize,

    /// Maximum textual size of one instruction.
    pub max_instruction_bytes: usize,

    /// Maximum execution time permitted for one candidate.
    pub timeout: Duration,
}

impl Default for FuzzLimits {
    fn default() -> Self {
        Self {
            max_candidates: 10_000,
            max_stages: 2,
            max_instruction_bytes: 4096,
            timeout: Duration::from_millis(100),
        }
    }
}

impl FuzzLimits {
    pub fn validate(&self) -> Result<(), FuzzError> {
        if self.max_candidates == 0 {
            return Err(FuzzError::InvalidConfiguration(
                "max_candidates must be greater than zero".to_string(),
            ));
        }

        if self.max_stages == 0 {
            return Err(FuzzError::InvalidConfiguration(
                "max_stages must be greater than zero".to_string(),
            ));
        }

        if self.max_instruction_bytes == 0 {
            return Err(FuzzError::InvalidConfiguration(
                "max_instruction_bytes must be greater than zero".to_string(),
            ));
        }

        if self.timeout.is_zero() {
            return Err(FuzzError::InvalidConfiguration(
                "timeout must be greater than zero".to_string(),
            ));
        }

        Ok(())
    }
}

/// Configuration for a fuzzing campaign.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzConfig {
    pub seed: u64,
    pub limits: FuzzLimits,
    pub include_same_substrate: bool,
    pub include_cross_substrate: bool,
}

impl Default for FuzzConfig {
    fn default() -> Self {
        Self {
            seed: 0x5A4D_414E_49,
            limits: FuzzLimits::default(),
            include_same_substrate: false,
            include_cross_substrate: true,
        }
    }
}

impl FuzzConfig {
    pub fn validate(&self) -> Result<(), FuzzError> {
        self.limits.validate()?;

        if !self.include_same_substrate && !self.include_cross_substrate {
            return Err(FuzzError::InvalidConfiguration(
                "at least one candidate generation mode must be enabled".to_string(),
            ));
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Substrates
// -----------------------------------------------------------------------------

/// Execution substrate represented by a fuzzing candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Substrate {
    Classical,
    Quantum,
    Neuromorphic,
}

impl fmt::Display for Substrate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Classical => write!(formatter, "Classical"),
            Self::Quantum => write!(formatter, "Quantum"),
            Self::Neuromorphic => write!(formatter, "Neuromorphic"),
        }
    }
}

// -----------------------------------------------------------------------------
// Candidate model
// -----------------------------------------------------------------------------

/// One instruction executed against one substrate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzStage {
    pub substrate: Substrate,
    pub instruction: String,
}

/// A complete fuzzing candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzingCandidate {
    pub id: u64,
    pub stages: Vec<FuzzStage>,
}

impl FuzzingCandidate {
    pub fn new(id: u64, stages: Vec<FuzzStage>) -> Result<Self, FuzzError> {
        if stages.is_empty() {
            return Err(FuzzError::InvalidCandidate(
                "candidate must contain at least one stage".to_string(),
            ));
        }

        Ok(Self { id, stages })
    }

    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    pub fn substrates(&self) -> BTreeSet<Substrate> {
        self.stages
            .iter()
            .map(|stage| stage.substrate)
            .collect()
    }
}

// -----------------------------------------------------------------------------
// Candidate validation
// -----------------------------------------------------------------------------

/// Validates a candidate before execution.
pub fn validate_candidate(
    candidate: &FuzzingCandidate,
    limits: &FuzzLimits,
) -> Result<(), FuzzError> {
    limits.validate()?;

    if candidate.stages.is_empty() {
        return Err(FuzzError::InvalidCandidate(
            "candidate contains no stages".to_string(),
        ));
    }

    if candidate.stages.len() > limits.max_stages {
        return Err(FuzzError::LimitExceeded(format!(
            "candidate contains {} stages; limit is {}",
            candidate.stages.len(),
            limits.max_stages
        )));
    }

    for stage in &candidate.stages {
        if stage.instruction.is_empty() {
            return Err(FuzzError::InvalidCandidate(
                "instruction cannot be empty".to_string(),
            ));
        }

        if stage.instruction.len() > limits.max_instruction_bytes {
            return Err(FuzzError::LimitExceeded(format!(
                "instruction exceeds {} bytes",
                limits.max_instruction_bytes
            )));
        }

        if stage.instruction.contains('\0') {
            return Err(FuzzError::InvalidCandidate(
                "instruction contains a NUL byte".to_string(),
            ));
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Result classification
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuzzOutcome {
    /// Candidate was accepted and executed normally.
    Accepted,

    /// Candidate was rejected by the safety/validation layer.
    Rejected,

    /// Candidate caused a detected safety violation.
    SafetyViolation,

    /// Candidate produced an invalid result.
    InvalidResult,

    /// Candidate exceeded the configured execution budget.
    Timeout,

    /// Executor reported an error.
    ExecutionError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzResult {
    pub candidate_id: u64,
    pub outcome: FuzzOutcome,
    pub message: String,
    pub duration_nanos: u128,
}

impl FuzzResult {
    fn new(
        candidate_id: u64,
        outcome: FuzzOutcome,
        message: impl Into<String>,
        duration: Duration,
    ) -> Self {
        Self {
            candidate_id,
            outcome,
            message: message.into(),
            duration_nanos: duration.as_nanos(),
        }
    }
}

// -----------------------------------------------------------------------------
// Executor interface
// -----------------------------------------------------------------------------

/// Result returned by a substrate executor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReport {
    pub success: bool,
    pub safety_violation: bool,
    pub message: String,
}

impl ExecutionReport {
    pub fn accepted(message: impl Into<String>) -> Self {
        Self {
            success: true,
            safety_violation: false,
            message: message.into(),
        }
    }

    pub fn rejected(message: impl Into<String>) -> Self {
        Self {
            success: false,
            safety_violation: false,
            message: message.into(),
        }
    }

    pub fn violation(message: impl Into<String>) -> Self {
        Self {
            success: false,
            safety_violation: true,
            message: message.into(),
        }
    }
}

/// Execution abstraction.
///
/// The compiler supplies this interface; the fuzzing harness never executes
/// arbitrary instructions directly.
pub trait FuzzExecutor {
    fn execute(
        &mut self,
        candidate: &FuzzingCandidate,
    ) -> Result<ExecutionReport, String>;
}

/// Safe default executor.
///
/// It validates candidates but does not execute hardware operations.
#[derive(Debug, Default, Clone, Copy)]
pub struct ValidationOnlyExecutor;

impl FuzzExecutor for ValidationOnlyExecutor {
    fn execute(
        &mut self,
        candidate: &FuzzingCandidate,
    ) -> Result<ExecutionReport, String> {
        if candidate.stages.is_empty() {
            return Ok(ExecutionReport::rejected(
                "candidate contains no executable stages",
            ));
        }

        Ok(ExecutionReport::accepted(
            "candidate validated; no substrate execution performed",
        ))
    }
}

// -----------------------------------------------------------------------------
// Campaign statistics
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FuzzStatistics {
    pub generated: usize,
    pub executed: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub safety_violations: usize,
    pub invalid_results: usize,
    pub timeouts: usize,
    pub execution_errors: usize,
}

impl FuzzStatistics {
    fn record(&mut self, result: &FuzzResult) {
        self.executed += 1;

        match result.outcome {
            FuzzOutcome::Accepted => self.accepted += 1,
            FuzzOutcome::Rejected => self.rejected += 1,
            FuzzOutcome::SafetyViolation => self.safety_violations += 1,
            FuzzOutcome::InvalidResult => self.invalid_results += 1,
            FuzzOutcome::Timeout => self.timeouts += 1,
            FuzzOutcome::ExecutionError => self.execution_errors += 1,
        }
    }
}

// -----------------------------------------------------------------------------
// Campaign report
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzCampaignReport {
    pub seed: u64,
    pub statistics: FuzzStatistics,
    pub results: Vec<FuzzResult>,
}

impl FuzzCampaignReport {
    pub fn finding_count(&self) -> usize {
        self.statistics.safety_violations
            + self.statistics.invalid_results
            + self.statistics.timeouts
            + self.statistics.execution_errors
    }
}

// -----------------------------------------------------------------------------
// Cross-substrate fuzzer
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CrossSubstrateFuzzer {
    pub neuromorphic_pool: Vec<String>,
    pub quantum_pool: Vec<String>,
    pub classical_pool: Vec<String>,
    pub config: FuzzConfig,
}

impl Default for CrossSubstrateFuzzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossSubstrateFuzzer {
    pub fn new() -> Self {
        Self {
            neuromorphic_pool: vec![
                "SPIKE_EMIT".to_string(),
                "MEMBRANE_INTEGRATE".to_string(),
                "PREPARE_SHARED_BUFFER".to_string(),
                "ALLOCATE_SYNAPSE_MEM".to_string(),
            ],
            quantum_pool: vec![
                "RZ(pi/2)".to_string(),
                "HADAMARD".to_string(),
                "CNOT".to_string(),
            ],
            classical_pool: vec![
                "MOV RAX, RDX".to_string(),
            ],
            config: FuzzConfig::default(),
        }
    }

    pub fn with_config(config: FuzzConfig) -> Result<Self, FuzzError> {
        config.validate()?;

        let mut fuzzer = Self::new();
        fuzzer.config = config;
        Ok(fuzzer)
    }

    pub fn generate_candidates(
        &self,
    ) -> Result<Vec<FuzzingCandidate>, FuzzError> {
        self.config.validate()?;

        let mut candidates = Vec::new();
        let mut next_id = 0u64;

        let pools = [
            (Substrate::Neuromorphic, &self.neuromorphic_pool),
            (Substrate::Quantum, &self.quantum_pool),
            (Substrate::Classical, &self.classical_pool),
        ];

        for (first_substrate, first_pool) in &pools {
            for (second_substrate, second_pool) in &pools {
                if first_substrate == second_substrate {
                    if !self.config.include_same_substrate {
                        continue;
                    }
                } else if !self.config.include_cross_substrate {
                    continue;
                }

                for first_instruction in first_pool.iter() {
                    for second_instruction in second_pool.iter() {
                        if candidates.len() >= self.config.limits.max_candidates {
                            return Ok(candidates);
                        }

                        let candidate = FuzzingCandidate::new(
                            next_id,
                            vec![
                                FuzzStage {
                                    substrate: *first_substrate,
                                    instruction: first_instruction.clone(),
                                },
                                FuzzStage {
                                    substrate: *second_substrate,
                                    instruction: second_instruction.clone(),
                                },
                            ],
                        )?;

                        validate_candidate(
                            &candidate,
                            &self.config.limits,
                        )?;

                        candidates.push(candidate);
                        next_id = next_id.wrapping_add(1);
                    }
                }
            }
        }

        Ok(candidates)
    }

    /// Execute a complete fuzzing campaign through an explicit executor.
    pub fn run_campaign<E: FuzzExecutor>(
        &self,
        executor: &mut E,
    ) -> Result<FuzzCampaignReport, FuzzError> {
        let candidates = self.generate_candidates()?;

        let mut statistics = FuzzStatistics {
            generated: candidates.len(),
            ..FuzzStatistics::default()
        };

        let mut results = Vec::with_capacity(candidates.len());

        for candidate in candidates {
            let validation = validate_candidate(
                &candidate,
                &self.config.limits,
            );

            if let Err(error) = validation {
                let result = FuzzResult::new(
                    candidate.id,
                    FuzzOutcome::Rejected,
                    error.to_string(),
                    Duration::ZERO,
                );

                statistics.record(&result);
                results.push(result);
                continue;
            }

            let start = Instant::now();

            let execution = executor.execute(&candidate);
            let elapsed = start.elapsed();

            let result = if elapsed > self.config.limits.timeout {
                FuzzResult::new(
                    candidate.id,
                    FuzzOutcome::Timeout,
                    format!(
                        "execution exceeded {:?}",
                        self.config.limits.timeout
                    ),
                    elapsed,
                )
            } else {
                match execution {
                    Ok(report) if report.safety_violation => FuzzResult::new(
                        candidate.id,
                        FuzzOutcome::SafetyViolation,
                        report.message,
                        elapsed,
                    ),

                    Ok(report) if report.success => FuzzResult::new(
                        candidate.id,
                        FuzzOutcome::Accepted,
                        report.message,
                        elapsed,
                    ),

                    Ok(report) => FuzzResult::new(
                        candidate.id,
                        FuzzOutcome::Rejected,
                        report.message,
                        elapsed,
                    ),

                    Err(error) => FuzzResult::new(
                        candidate.id,
                        FuzzOutcome::ExecutionError,
                        error,
                        elapsed,
                    ),
                }
            };

            statistics.record(&result);
            results.push(result);
        }

        Ok(FuzzCampaignReport {
            seed: self.config.seed,
            statistics,
            results,
        })
    }

    /// Returns a deterministic inventory of available instructions.
    pub fn instruction_inventory(
        &self,
    ) -> BTreeMap<Substrate, BTreeSet<String>> {
        let mut inventory = BTreeMap::new();

        inventory.insert(
            Substrate::Neuromorphic,
            self.neuromorphic_pool.iter().cloned().collect(),
        );

        inventory.insert(
            Substrate::Quantum,
            self.quantum_pool.iter().cloned().collect(),
        );

        inventory.insert(
            Substrate::Classical,
            self.classical_pool.iter().cloned().collect(),
        );

        inventory
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuzzError {
    InvalidConfiguration(String),
    InvalidCandidate(String),
    LimitExceeded(String),
}

impl fmt::Display for FuzzError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid fuzzing configuration: {message}")
            }
            Self::InvalidCandidate(message) => {
                write!(formatter, "invalid fuzzing candidate: {message}")
            }
            Self::LimitExceeded(message) => {
                write!(formatter, "fuzzing limit exceeded: {message}")
            }
        }
    }
}

impl std::error::Error for FuzzError {}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        assert!(FuzzConfig::default().validate().is_ok());
    }

    #[test]
    fn default_fuzzer_generates_cross_substrate_candidates() {
        let fuzzer = CrossSubstrateFuzzer::new();

        let candidates = fuzzer
            .generate_candidates()
            .expect("candidate generation should succeed");

        assert!(!candidates.is_empty());
        assert!(candidates.iter().all(|candidate| candidate.stage_count() == 2));
    }

    #[test]
    fn candidate_ids_are_deterministic() {
        let fuzzer = CrossSubstrateFuzzer::new();

        let first = fuzzer.generate_candidates().unwrap();
        let second = fuzzer.generate_candidates().unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn empty_instruction_is_rejected() {
        let candidate = FuzzingCandidate::new(
            0,
            vec![FuzzStage {
                substrate: Substrate::Classical,
                instruction: String::new(),
            }],
        )
        .unwrap();

        assert!(validate_candidate(
            &candidate,
            &FuzzLimits::default()
        )
        .is_err());
    }

    #[test]
    fn nul_instruction_is_rejected() {
        let candidate = FuzzingCandidate::new(
            0,
            vec![FuzzStage {
                substrate: Substrate::Classical,
                instruction: "MOV\0RAX".to_string(),
            }],
        )
        .unwrap();

        assert!(validate_candidate(
            &candidate,
            &FuzzLimits::default()
        )
        .is_err());
    }

    #[test]
    fn candidate_stage_limit_is_enforced() {
        let candidate = FuzzingCandidate::new(
            0,
            vec![
                FuzzStage {
                    substrate: Substrate::Classical,
                    instruction: "A".to_string(),
                },
                FuzzStage {
                    substrate: Substrate::Quantum,
                    instruction: "B".to_string(),
                },
                FuzzStage {
                    substrate: Substrate::Neuromorphic,
                    instruction: "C".to_string(),
                },
            ],
        )
        .unwrap();

        let limits = FuzzLimits {
            max_stages: 2,
            ..FuzzLimits::default()
        };

        assert!(validate_candidate(&candidate, &limits).is_err());
    }

    #[test]
    fn validation_only_executor_is_safe() {
        let fuzzer = CrossSubstrateFuzzer::new();
        let mut executor = ValidationOnlyExecutor;

        let report = fuzzer
            .run_campaign(&mut executor)
            .expect("campaign should succeed");

        assert_eq!(
            report.statistics.generated,
            report.statistics.executed
        );

        assert_eq!(
            report.statistics.safety_violations,
            0
        );

        assert_eq!(
            report.statistics.execution_errors,
            0
        );
    }

    #[test]
    fn candidate_limit_is_enforced() {
        let config = FuzzConfig {
            limits: FuzzLimits {
                max_candidates: 3,
                ..FuzzLimits::default()
            },
            ..FuzzConfig::default()
        };

        let fuzzer = CrossSubstrateFuzzer::with_config(config).unwrap();
        let candidates = fuzzer.generate_candidates().unwrap();

        assert_eq!(candidates.len(), 3);
    }

    #[test]
    fn inventory_is_deterministic() {
        let fuzzer = CrossSubstrateFuzzer::new();

        let first = fuzzer.instruction_inventory();
        let second = fuzzer.instruction_inventory();

        assert_eq!(first, second);
    }
}