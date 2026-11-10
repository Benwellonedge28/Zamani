
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Production Fuzzing Infrastructure
//!
//! Path: src/compiler/fuzzing_harness.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide extensible, deterministic and resource-aware fuzzing
//! infrastructure for the complete Zamani compiler architecture.
//!
//! Supported testing boundaries include:
//!
//! - Raw source bytes.
//! - UTF-8 source text.
//! - Lexical analysis.
//! - Grammar and parsing.
//! - AST construction.
//! - Semantic validation.
//! - Type and effect validation.
//! - Capability and requirement analysis.
//! - Canonical IR construction.
//! - IR verification.
//! - Compiler transformations.
//! - Optimization.
//! - Lowering.
//! - Backend interfaces.
//! - Linking.
//! - Artifact serialization.
//! - Artifact validation.
//! - Incremental compilation.
//! - Cross-domain integration.
//!
//! # Architectural ownership
//!
//! This file owns:
//!
//! - Fuzzing input and case identities.
//! - Deterministic mutation.
//! - Corpus traversal.
//! - Fuzz-target interfaces.
//! - Execution result classification.
//! - Campaign accounting.
//! - Regression-compatible reports.
//!
//! It does NOT own:
//!
//! - The Zamani grammar.
//! - The lexer.
//! - The parser.
//! - The AST.
//! - Semantic analysis.
//! - ZUIR or domain-specific IR.
//! - Compiler transformations.
//! - Backend implementations.
//! - Physical resource allocation.
//! - Runtime execution.
//!
//! Those responsibilities remain with their canonical owners.
//!
//! # Scalability
//!
//! No universal hardware-capacity limits are introduced.
//!
//! Optional fuzzing budgets describe the amount of work a campaign
//! is permitted to perform. They do not define the capabilities of
//! the Zamani language or its supported computing domains.
//!
//! The campaign processes inputs incrementally rather than requiring
//! the complete corpus or complete result collection in memory.
//!
//! No finite Rust process can consume literally infinite input.
//! Actual capacity is constrained by available resources, configured
//! budgets, address-space limits and operating-system policies.
//!
//! # Determinism
//!
//! For identical:
//!
//! - seed;
//! - input corpus;
//! - configuration;
//! - target implementation;
//! - mutation sequence;
//!
//! the generated mutation sequence is deterministic.
//!
//! Execution duration and externally observable target behavior
//! are not guaranteed to be deterministic.
//!
//! # Security
//!
//! The default implementation never executes arbitrary machine
//! instructions, quantum operations, HDL descriptions or binaries.
//!
//! Target execution is explicitly supplied by the caller.
//!
//! Panic capture works when the process uses unwinding. It cannot
//! recover from process aborts, memory exhaustion, or external
//! process termination.
//!
//! # Integration
//!
//! Compiler composition:
//!     crate::compiler::fuzzing_harness
//!
//! Frontend:
//!     crate::frontend::lexer
//!     crate::frontend::parser
//!     crate::frontend::semantic
//!     crate::frontend::ast
//!
//! IR:
//!     crate::zuir
//!     crate::classical
//!     crate::quantum::ir
//!     crate::hdl
//!
//! Compiler:
//!     crate::compiler::pipeline
//!     crate::compiler::analysis
//!     crate::compiler::optimization
//!     crate::compiler::lowering
//!     crate::compiler::backends
//!     crate::compiler::artifacts
//!     crate::compiler::incremental
//!
//! Resource accounting:
//!     crate::compiler::configuration::budget
//!
//! The harness depends on target interfaces rather than implementing
//! competing versions of those subsystems.

use std::error::Error;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::{Duration, Instant};

// ============================================================================
// API VERSION
// ============================================================================

/// Public fuzzing-harness API version.
pub const FUZZING_HARNESS_API_VERSION: u16 = 2;

// ============================================================================
// DOMAIN IDENTITIES
// ============================================================================

/// Open-world identity of a computing domain.
///
/// Domain identifiers are strings rather than a closed enumeration.
/// New domains therefore do not require modifications to this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuzzDomain(String);

impl FuzzDomain {
    /// Create a validated domain identity.
    pub fn new(value: impl Into<String>) -> Result<Self, FuzzError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FuzzError::InvalidConfiguration(
                "domain identity cannot be empty".into(),
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(FuzzError::InvalidConfiguration(
                "domain identity cannot contain control characters".into(),
            ));
        }

        Ok(Self(value))
    }

    /// Return the domain identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Classical computing.
    pub fn classical() -> Self {
        Self("classical".into())
    }

    /// Quantum computing.
    pub fn quantum() -> Self {
        Self("quantum".into())
    }

    /// Hardware description.
    pub fn hdl() -> Self {
        Self("hdl".into())
    }

    /// Hybrid computing.
    pub fn hybrid() -> Self {
        Self("hybrid".into())
    }

    /// Distributed computing.
    pub fn distributed() -> Self {
        Self("distributed".into())
    }

    /// Artificial intelligence.
    pub fn ai() -> Self {
        Self("ai".into())
    }

    /// Tensor computation.
    pub fn tensor() -> Self {
        Self("tensor".into())
    }

    /// Neuromorphic computing.
    pub fn neuromorphic() -> Self {
        Self("neuromorphic".into())
    }
}

impl fmt::Display for FuzzDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// FUZZ TARGET IDENTITIES
// ============================================================================

/// Identifies the compiler subsystem being fuzzed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuzzTargetId(String);

impl FuzzTargetId {
    /// Create a validated target identity.
    pub fn new(value: impl Into<String>) -> Result<Self, FuzzError> {
        let value = value.into();

        if value.trim().is_empty()
            || value.chars().any(char::is_control)
        {
            return Err(FuzzError::InvalidConfiguration(
                "invalid fuzz-target identity".into(),
            ));
        }

        Ok(Self(value))
    }

    /// Return the target identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FuzzTargetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// RESOURCE LIMITS
// ============================================================================

/// Optional operational limits for one fuzzing campaign.
///
/// `None` means that this configuration imposes no limit on that
/// particular dimension.
///
/// Actual machine and operating-system limits still apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FuzzLimits {
    /// Maximum number of cases.
    pub max_cases: Option<u128>,

    /// Maximum size of an individual input.
    pub max_input_bytes: Option<u128>,

    /// Maximum aggregate input bytes processed.
    pub max_total_input_bytes: Option<u128>,

    /// Maximum measured execution time per case.
    ///
    /// This is a reporting threshold, not an enforceable
    /// cancellation mechanism.
    pub timeout_per_case: Option<Duration>,
}

impl FuzzLimits {
    /// Validate the operational configuration.
    pub fn validate(&self) -> Result<(), FuzzError> {
        if self.max_cases == Some(0) {
            return Err(FuzzError::InvalidConfiguration(
                "max_cases cannot be zero".into(),
            ));
        }

        if self.max_input_bytes == Some(0) {
            return Err(FuzzError::InvalidConfiguration(
                "max_input_bytes cannot be zero".into(),
            ));
        }

        if self.max_total_input_bytes == Some(0) {
            return Err(FuzzError::InvalidConfiguration(
                "max_total_input_bytes cannot be zero".into(),
            ));
        }

        if self.timeout_per_case.is_some_and(Duration::is_zero) {
            return Err(FuzzError::InvalidConfiguration(
                "timeout_per_case cannot be zero".into(),
            ));
        }

        Ok(())
    }
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Mutation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationStrategy {
    /// Flip one or more input bits.
    BitFlip,

    /// Replace an existing byte.
    ByteReplacement,

    /// Insert a byte.
    ByteInsertion,

    /// Delete a byte.
    ByteDeletion,

    /// Duplicate an existing input region.
    Duplication,

    /// Select a strategy deterministically for each mutation.
    Mixed,
}

/// Fuzzing campaign configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzConfig {
    /// Deterministic mutation seed.
    pub seed: u64,

    /// Operational limits.
    pub limits: FuzzLimits,

    /// Number of mutations generated from each corpus entry.
    pub mutations_per_seed: u64,

    /// Selected mutation strategy.
    pub mutation_strategy: MutationStrategy,

    /// Whether to retain malformed UTF-8 inputs.
    pub allow_non_utf8: bool,

    /// Whether executor panics are captured.
    pub capture_panics: bool,
}

impl Default for FuzzConfig {
    fn default() -> Self {
        Self {
            seed: 0x5A4D_414E_49,
            limits: FuzzLimits::default(),
            mutations_per_seed: 0,
            mutation_strategy: MutationStrategy::Mixed,
            allow_non_utf8: true,
            capture_panics: true,
        }
    }
}

impl FuzzConfig {
    /// Validate configuration.
    pub fn validate(&self) -> Result<(), FuzzError> {
        self.limits.validate()
    }
}

// ============================================================================
// FUZZ INPUT
// ============================================================================

/// One owned fuzzing input.
///
/// Raw bytes are preserved because the lexer and parser must be
/// tested against malformed encodings as well as valid source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzInput {
    bytes: Vec<u8>,
}

impl FuzzInput {
    /// Construct a raw-byte input.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Borrow the raw input.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the input length.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Check whether the input is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Interpret input as UTF-8 source text.
    pub fn as_source(&self) -> Result<&str, FuzzError> {
        std::str::from_utf8(&self.bytes)
            .map_err(|error| {
                FuzzError::InvalidInput(
                    format!("input is not valid UTF-8: {error}"),
                )
            })
    }
}

// ============================================================================
// CASE IDENTITY
// ============================================================================

/// Stable identity for a fuzzing case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuzzCaseId(pub u128);

/// One generated test case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzCase {
    /// Stable campaign-local case identity.
    pub id: FuzzCaseId,

    /// Original corpus-entry index.
    pub seed_index: u128,

    /// Mutation index, if this is a mutated input.
    pub mutation_index: Option<u64>,

    /// Input bytes.
    pub input: FuzzInput,
}

// ============================================================================
// TARGET INTERFACE
// ============================================================================

/// Result of running one fuzzing input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetOutcome {
    /// Target accepted the input.
    Accepted,

    /// Target correctly rejected the input.
    Rejected(String),

    /// Target detected an invariant or safety violation.
    SafetyViolation(String),

    /// Target produced an invalid result.
    InvalidResult(String),
}

/// Interface implemented by compiler subsystem fuzz targets.
///
/// The target must not assume that input is valid source text.
///
/// The target owns interpretation and validation of its input.
///
/// Implementations should be deterministic when their underlying
/// compiler subsystem supports deterministic operation.
pub trait FuzzTarget {
    /// Execute the target against one fuzz input.
    fn execute(
        &mut self,
        input: &FuzzInput,
    ) -> Result<TargetOutcome, String>;
}

// ============================================================================
// SOURCE TARGET ADAPTER
// ============================================================================

/// Adapter for testing a source-processing subsystem.
///
/// This permits integration with the actual frontend parser and
/// semantic services without introducing a duplicate parser.
///
/// The supplied callback should invoke the canonical repository API.
pub struct SourceFuzzTarget<F> {
    id: FuzzTargetId,
    processor: F,
}

impl<F> SourceFuzzTarget<F> {
    /// Construct a source target.
    pub fn new(id: FuzzTargetId, processor: F) -> Self {
        Self { id, processor }
    }

    /// Return target identity.
    pub fn id(&self) -> &FuzzTargetId {
        &self.id
    }
}

impl<F> FuzzTarget for SourceFuzzTarget<F>
where
    F: FnMut(&str) -> Result<(), String>,
{
    fn execute(
        &mut self,
        input: &FuzzInput,
    ) -> Result<TargetOutcome, String> {
        let source = match input.as_source() {
            Ok(source) => source,
            Err(error) => {
                return Ok(TargetOutcome::Rejected(
                    error.to_string(),
                ));
            }
        };

        match (self.processor)(source) {
            Ok(()) => Ok(TargetOutcome::Accepted),
            Err(error) => Ok(TargetOutcome::Rejected(error)),
        }
    }
}

// ============================================================================
// RESULT CLASSIFICATION
// ============================================================================

/// Classification of a fuzzing execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuzzOutcome {
    Accepted,
    Rejected,
    SafetyViolation,
    InvalidResult,
    Timeout,
    ExecutionError,
    Panic,
    InvalidInput,
}

/// One execution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzResult {
    pub case_id: FuzzCaseId,
    pub outcome: FuzzOutcome,
    pub message: String,
    pub duration_nanos: u128,
    pub input_bytes: u128,
}

impl FuzzResult {
    fn new(
        case_id: FuzzCaseId,
        outcome: FuzzOutcome,
        message: impl Into<String>,
        duration: Duration,
        input_bytes: usize,
    ) -> Self {
        Self {
            case_id,
            outcome,
            message: message.into(),
            duration_nanos: duration.as_nanos(),
            input_bytes: input_bytes as u128,
        }
    }

    /// Determine whether the result represents a finding.
    pub fn is_finding(&self) -> bool {
        matches!(
            self.outcome,
            FuzzOutcome::SafetyViolation
                | FuzzOutcome::InvalidResult
                | FuzzOutcome::Timeout
                | FuzzOutcome::ExecutionError
                | FuzzOutcome::Panic
        )
    }
}

// ============================================================================
// CAMPAIGN STATISTICS
// ============================================================================

/// Campaign accounting.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FuzzStatistics {
    pub generated: u128,
    pub executed: u128,
    pub accepted: u128,
    pub rejected: u128,
    pub safety_violations: u128,
    pub invalid_results: u128,
    pub timeouts: u128,
    pub execution_errors: u128,
    pub panics: u128,
    pub total_input_bytes: u128,
    pub total_execution_nanos: u128,
}

impl FuzzStatistics {
    fn increment(value: &mut u128) -> Result<(), FuzzError> {
        *value = value.checked_add(1).ok_or(
            FuzzError::AccountingOverflow,
        )?;

        Ok(())
    }

    fn record(&mut self, result: &FuzzResult) -> Result<(), FuzzError> {
        Self::increment(&mut self.executed)?;

        self.total_input_bytes = self
            .total_input_bytes
            .checked_add(result.input_bytes)
            .ok_or(FuzzError::AccountingOverflow)?;

        self.total_execution_nanos = self
            .total_execution_nanos
            .checked_add(result.duration_nanos)
            .ok_or(FuzzError::AccountingOverflow)?;

        match result.outcome {
            FuzzOutcome::Accepted => {
                Self::increment(&mut self.accepted)?;
            }
            FuzzOutcome::Rejected | FuzzOutcome::InvalidInput => {
                Self::increment(&mut self.rejected)?;
            }
            FuzzOutcome::SafetyViolation => {
                Self::increment(&mut self.safety_violations)?;
            }
            FuzzOutcome::InvalidResult => {
                Self::increment(&mut self.invalid_results)?;
            }
            FuzzOutcome::Timeout => {
                Self::increment(&mut self.timeouts)?;
            }
            FuzzOutcome::ExecutionError => {
                Self::increment(&mut self.execution_errors)?;
            }
            FuzzOutcome::Panic => {
                Self::increment(&mut self.panics)?;
            }
        }

        Ok(())
    }

    /// Return the total number of findings.
    pub fn finding_count(&self) -> u128 {
        self.safety_violations
            .saturating_add(self.invalid_results)
            .saturating_add(self.timeouts)
            .saturating_add(self.execution_errors)
            .saturating_add(self.panics)
    }
}

// ============================================================================
// CAMPAIGN REPORT
// ============================================================================

/// Completed campaign summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzCampaignReport {
    pub seed: u64,
    pub target: FuzzTargetId,
    pub statistics: FuzzStatistics,
    pub completed: bool,
}

impl FuzzCampaignReport {
    /// Determine whether the campaign discovered a finding.
    pub fn has_findings(&self) -> bool {
        self.statistics.finding_count() != 0
    }
}

// ============================================================================
// DETERMINISTIC RANDOM GENERATOR
// ============================================================================

/// Small deterministic generator used exclusively for mutation.
///
/// It does not provide cryptographic randomness.
#[derive(Debug, Clone)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(
            0x9E37_79B9_7F4A_7C15,
        );

        let mut value = self.state;

        value = (value ^ (value >> 30))
            .wrapping_mul(0xBF58_476D_1CE4_E5B9);

        value = (value ^ (value >> 27))
            .wrapping_mul(0x94D0_49BB_1331_11EB);

        value ^ (value >> 31)
    }

    fn index(&mut self, length: usize) -> Option<usize> {
        if length == 0 {
            return None;
        }

        Some((self.next_u64() as u128 % length as u128) as usize)
    }
}

// ============================================================================
// MUTATION ENGINE
// ============================================================================

/// Deterministic input mutation engine.
#[derive(Debug, Clone)]
pub struct Mutator {
    rng: DeterministicRng,
    strategy: MutationStrategy,
}

impl Mutator {
    /// Construct a deterministic mutator.
    pub fn new(seed: u64, strategy: MutationStrategy) -> Self {
        Self {
            rng: DeterministicRng::new(seed),
            strategy,
        }
    }

    /// Mutate one input.
    ///
    /// `max_bytes` is an operational input budget.
    /// It is not a language or hardware capacity limit.
    pub fn mutate(
        &mut self,
        input: &FuzzInput,
        max_bytes: Option<u128>,
    ) -> Result<FuzzInput, FuzzError> {
        let mut bytes = input.bytes.clone();

        let strategy = match self.strategy {
            MutationStrategy::Mixed => {
                match self.rng.next_u64() % 5 {
                    0 => MutationStrategy::BitFlip,
                    1 => MutationStrategy::ByteReplacement,
                    2 => MutationStrategy::ByteInsertion,
                    3 => MutationStrategy::ByteDeletion,
                    _ => MutationStrategy::Duplication,
                }
            }
            strategy => strategy,
        };

        match strategy {
            MutationStrategy::BitFlip => {
                if let Some(index) = self.rng.index(bytes.len()) {
                    let bit = (self.rng.next_u64() % 8) as u32;
                    bytes[index] ^= 1u8 << bit;
                } else {
                    bytes.push(self.rng.next_u64() as u8);
                }
            }

            MutationStrategy::ByteReplacement => {
                if let Some(index) = self.rng.index(bytes.len()) {
                    bytes[index] = self.rng.next_u64() as u8;
                } else {
                    bytes.push(self.rng.next_u64() as u8);
                }
            }

            MutationStrategy::ByteInsertion => {
                let new_length = bytes
                    .len()
                    .checked_add(1)
                    .ok_or(FuzzError::AccountingOverflow)?;

                Self::check_size(new_length, max_bytes)?;

                let index = self.rng.index(
                    bytes.len().saturating_add(1),
                ).ok_or(FuzzError::AccountingOverflow)?;

                bytes.insert(index, self.rng.next_u64() as u8);
            }

            MutationStrategy::ByteDeletion => {
                if let Some(index) = self.rng.index(bytes.len()) {
                    bytes.remove(index);
                }
            }

            MutationStrategy::Duplication => {
                if let Some(index) = self.rng.index(bytes.len()) {
                    let byte = bytes[index];

                    let new_length = bytes
                        .len()
                        .checked_add(1)
                        .ok_or(FuzzError::AccountingOverflow)?;

                    Self::check_size(new_length, max_bytes)?;

                    bytes.push(byte);
                }
            }

            MutationStrategy::Mixed => unreachable!(),
        }

        Self::check_size(bytes.len(), max_bytes)?;

        Ok(FuzzInput::new(bytes))
    }

    fn check_size(
        size: usize,
        max_bytes: Option<u128>,
    ) -> Result<(), FuzzError> {
        if max_bytes.is_some_and(|limit| size as u128 > limit) {
            return Err(FuzzError::LimitExceeded(
                "mutation exceeds input-byte budget".into(),
            ));
        }

        Ok(())
    }
}

// ============================================================================
// CAMPAIGN EXECUTION
// ============================================================================

/// Streaming fuzzing campaign.
///
/// Results are delivered to the caller through a callback. The
/// harness does not retain an entire campaign's results in memory.
pub struct FuzzCampaign {
    config: FuzzConfig,
    target: FuzzTargetId,
}

impl FuzzCampaign {
    /// Create a validated campaign.
    pub fn new(
        config: FuzzConfig,
        target: FuzzTargetId,
    ) -> Result<Self, FuzzError> {
        config.validate()?;

        Ok(Self { config, target })
    }

    /// Return campaign configuration.
    pub fn config(&self) -> &FuzzConfig {
        &self.config
    }

    /// Execute a corpus incrementally.
    ///
    /// The iterator may represent an in-memory corpus, file-backed
    /// corpus, generated input stream, or another caller-owned source.
    ///
    /// No complete corpus is collected by this function.
    pub fn run<I, T, O>(
        &self,
        corpus: I,
        target: &mut T,
        mut observe: O,
    ) -> Result<FuzzCampaignReport, FuzzError>
    where
        I: IntoIterator<Item = Vec<u8>>,
        T: FuzzTarget,
        O: FnMut(&FuzzResult),
    {
        let mut statistics = FuzzStatistics::default();

        let mut mutator = Mutator::new(
            self.config.seed,
            self.config.mutation_strategy,
        );

        let mut case_id = 0u128;
        let mut seed_index = 0u128;

        let mut completed = true;

        for seed in corpus {
            if self
                .config
                .limits
                .max_cases
                .is_some_and(|limit| statistics.generated >= limit)
            {
                completed = false;
                break;
            }

            let input = FuzzInput::new(seed);

            Self::check_input(
                &input,
                &self.config,
            )?;

            let case = FuzzCase {
                id: FuzzCaseId(case_id),
                seed_index,
                mutation_index: None,
                input,
            };

            case_id = case_id
                .checked_add(1)
                .ok_or(FuzzError::AccountingOverflow)?;

            Self::execute_case(
                &case,
                target,
                &self.config,
                &mut statistics,
                &mut observe,
            )?;

            for mutation_index in 0..self.config.mutations_per_seed {
                if self
                    .config
                    .limits
                    .max_cases
                    .is_some_and(|limit| statistics.generated >= limit)
                {
                    completed = false;
                    break;
                }

                let mutated = match mutator.mutate(
                    &case.input,
                    self.config.limits.max_input_bytes,
                ) {
                    Ok(input) => input,

                    Err(FuzzError::LimitExceeded(_)) => {
                        continue;
                    }

                    Err(error) => return Err(error),
                };

                let mutated_case = FuzzCase {
                    id: FuzzCaseId(case_id),
                    seed_index,
                    mutation_index: Some(mutation_index),
                    input: mutated,
                };

                case_id = case_id
                    .checked_add(1)
                    .ok_or(FuzzError::AccountingOverflow)?;

                Self::check_input(
                    &mutated_case.input,
                    &self.config,
                )?;

                Self::execute_case(
                    &mutated_case,
                    target,
                    &self.config,
                    &mut statistics,
                    &mut observe,
                )?;
            }

            seed_index = seed_index
                .checked_add(1)
                .ok_or(FuzzError::AccountingOverflow)?;

            if !completed {
                break;
            }
        }

        Ok(FuzzCampaignReport {
            seed: self.config.seed,
            target: self.target.clone(),
            statistics,
            completed,
        })
    }

    fn check_input(
        input: &FuzzInput,
        config: &FuzzConfig,
    ) -> Result<(), FuzzError> {
        if !config.allow_non_utf8
            && std::str::from_utf8(input.as_bytes()).is_err()
        {
            return Err(FuzzError::InvalidInput(
                "non-UTF-8 input is disabled".into(),
            ));
        }

        if config
            .limits
            .max_input_bytes
            .is_some_and(|limit| input.len() as u128 > limit)
        {
            return Err(FuzzError::LimitExceeded(
                "input exceeds configured byte budget".into(),
            ));
        }

        Ok(())
    }

    fn execute_case<T, O>(
        case: &FuzzCase,
        target: &mut T,
        config: &FuzzConfig,
        statistics: &mut FuzzStatistics,
        observe: &mut O,
    ) -> Result<(), FuzzError>
    where
        T: FuzzTarget,
        O: FnMut(&FuzzResult),
    {
        if config
            .limits
            .max_total_input_bytes
            .is_some_and(|limit| {
                statistics.total_input_bytes
                    .saturating_add(case.input.len() as u128)
                    > limit
            })
        {
            return Err(FuzzError::LimitExceeded(
                "aggregate input-byte budget exhausted".into(),
            ));
        }

        FuzzStatistics::increment(&mut statistics.generated)?;

        let start = Instant::now();

        let execution = if config.capture_panics {
            match catch_unwind(AssertUnwindSafe(|| {
                target.execute(&case.input)
            })) {
                Ok(result) => result,
                Err(_) => {
                    let result = FuzzResult::new(
                        case.id,
                        FuzzOutcome::Panic,
                        "fuzz target panicked",
                        start.elapsed(),
                        case.input.len(),
                    );

                    statistics.record(&result)?;
                    observe(&result);

                    return Ok(());
                }
            }
        } else {
            target.execute(&case.input)
        };

        let elapsed = start.elapsed();

        let (outcome, message) = match execution {
            Ok(TargetOutcome::Accepted) => (
                FuzzOutcome::Accepted,
                "target accepted input".into(),
            ),

            Ok(TargetOutcome::Rejected(message)) => (
                FuzzOutcome::Rejected,
                message,
            ),

            Ok(TargetOutcome::SafetyViolation(message)) => (
                FuzzOutcome::SafetyViolation,
                message,
            ),

            Ok(TargetOutcome::InvalidResult(message)) => (
                FuzzOutcome::InvalidResult,
                message,
            ),

            Err(message) => (
                FuzzOutcome::ExecutionError,
                message,
            ),
        };

        let timed_out = config
            .limits
            .timeout_per_case
            .is_some_and(|limit| elapsed > limit);

        let result = if timed_out {
            FuzzResult::new(
                case.id,
                FuzzOutcome::Timeout,
                "execution exceeded the configured time threshold",
                elapsed,
                case.input.len(),
            )
        } else {
            FuzzResult::new(
                case.id,
                outcome,
                message,
                elapsed,
                case.input.len(),
            )
        };

        statistics.record(&result)?;
        observe(&result);

        Ok(())
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Fuzzing infrastructure errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuzzError {
    InvalidConfiguration(String),
    InvalidInput(String),
    LimitExceeded(String),
    AccountingOverflow,
}

impl fmt::Display for FuzzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(f, "invalid fuzzing configuration: {message}")
            }

            Self::InvalidInput(message) => {
                write!(f, "invalid fuzzing input: {message}")
            }

            Self::LimitExceeded(message) => {
                write!(f, "fuzzing budget exceeded: {message}")
            }

            Self::AccountingOverflow => {
                f.write_str("fuzzing accounting overflow")
            }
        }
    }
}

impl Error for FuzzError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct AcceptingTarget;

    impl FuzzTarget for AcceptingTarget {
        fn execute(
            &mut self,
            _input: &FuzzInput,
        ) -> Result<TargetOutcome, String> {
            Ok(TargetOutcome::Accepted)
        }
    }

    struct PanicTarget;

    impl FuzzTarget for PanicTarget {
        fn execute(
            &mut self,
            _input: &FuzzInput,
        ) -> Result<TargetOutcome, String> {
            panic!("intentional fuzzing panic");
        }
    }

    #[test]
    fn default_configuration_is_valid() {
        assert!(FuzzConfig::default().validate().is_ok());
    }

    #[test]
    fn domain_identity_is_open_world() {
        let domain = FuzzDomain::new("future_computing_domain")
            .unwrap();

        assert_eq!(
            domain.as_str(),
            "future_computing_domain"
        );
    }

    #[test]
    fn deterministic_mutation_replays() {
        let input = FuzzInput::new(
            b"fn main() {}".to_vec(),
        );

        let mut first = Mutator::new(
            42,
            MutationStrategy::Mixed,
        );

        let mut second = Mutator::new(
            42,
            MutationStrategy::Mixed,
        );

        for _ in 0..100 {
            assert_eq!(
                first.mutate(&input, None).unwrap(),
                second.mutate(&input, None).unwrap(),
            );
        }
    }

    #[test]
    fn empty_inputs_are_supported() {
        let input = FuzzInput::new(Vec::new());

        assert!(input.is_empty());
        assert_eq!(input.as_bytes(), b"");
    }

    #[test]
    fn malformed_utf8_is_preserved() {
        let input = FuzzInput::new(vec![0xff, 0xfe]);

        assert!(input.as_bytes().len() == 2);
        assert!(input.as_source().is_err());
    }

    #[test]
    fn source_target_rejects_invalid_utf8() {
        let mut target = SourceFuzzTarget::new(
            FuzzTargetId::new("frontend.parser").unwrap(),
            |_| Ok(()),
        );

        let result = target.execute(
            &FuzzInput::new(vec![0xff]),
        ).unwrap();

        assert!(matches!(
            result,
            TargetOutcome::Rejected(_)
        ));
    }

    #[test]
    fn campaign_streams_results() {
        let campaign = FuzzCampaign::new(
            FuzzConfig::default(),
            FuzzTargetId::new("test.target").unwrap(),
        ).unwrap();

        let mut target = AcceptingTarget;
        let mut observed = 0u128;

        let report = campaign.run(
            vec![
                b"one".to_vec(),
                b"two".to_vec(),
                b"three".to_vec(),
            ],
            &mut target,
            |_| observed += 1,
        ).unwrap();

        assert_eq!(report.statistics.generated, 3);
        assert_eq!(report.statistics.accepted, 3);
        assert_eq!(observed, 3);
        assert!(report.completed);
    }

    #[test]
    fn mutation_campaign_uses_configured_seed() {
        let config = FuzzConfig {
            mutations_per_seed: 5,
            seed: 123,
            ..FuzzConfig::default()
        };

        let campaign = FuzzCampaign::new(
            config,
            FuzzTargetId::new("mutation.test").unwrap(),
        ).unwrap();

        let mut target = AcceptingTarget;

        let report = campaign.run(
            vec![b"zamani".to_vec()],
            &mut target,
            |_| {},
        ).unwrap();

        assert_eq!(report.statistics.generated, 6);
        assert_eq!(report.statistics.accepted, 6);
    }

    #[test]
    fn campaign_case_limit_is_enforced() {
        let config = FuzzConfig {
            limits: FuzzLimits {
                max_cases: Some(2),
                ..FuzzLimits::default()
            },
            ..FuzzConfig::default()
        };

        let campaign = FuzzCampaign::new(
            config,
            FuzzTargetId::new("limited.test").unwrap(),
        ).unwrap();

        let mut target = AcceptingTarget;

        let report = campaign.run(
            vec![
                b"a".to_vec(),
                b"b".to_vec(),
                b"c".to_vec(),
            ],
            &mut target,
            |_| {},
        ).unwrap();

        assert_eq!(report.statistics.generated, 2);
        assert!(!report.completed);
    }

    #[test]
    fn panic_is_classified() {
        let campaign = FuzzCampaign::new(
            FuzzConfig::default(),
            FuzzTargetId::new("panic.test").unwrap(),
        ).unwrap();

        let mut target = PanicTarget;

        let report = campaign.run(
            vec![b"input".to_vec()],
            &mut target,
            |_| {},
        ).unwrap();

        assert_eq!(report.statistics.panics, 1);
        assert_eq!(report.statistics.executed, 1);
    }

    #[test]
    fn zero_case_limit_is_rejected() {
        let config = FuzzConfig {
            limits: FuzzLimits {
                max_cases: Some(0),
                ..FuzzLimits::default()
            },
            ..FuzzConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn input_budget_is_enforced() {
        let config = FuzzConfig {
            limits: FuzzLimits {
                max_input_bytes: Some(2),
                ..FuzzLimits::default()
            },
            ..FuzzConfig::default()
        };

        let campaign = FuzzCampaign::new(
            config,
            FuzzTargetId::new("input.test").unwrap(),
        ).unwrap();

        let mut target = AcceptingTarget;

        assert!(campaign.run(
            vec![b"larger".to_vec()],
            &mut target,
            |_| {},
        ).is_err());
    }
}
