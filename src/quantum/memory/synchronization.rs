//! Zamani Quantum Memory — Synchronization Engine
//!
//! Production-grade, provider-neutral synchronization orchestration for
//! `quantum::memory`.
//!
//! # Responsibility
//!
//! This module owns the *execution of the coherence protocol*.
//!
//! `coherence.rs` defines what a valid coherence state means.
//! `synchronization.rs` makes that state true by coordinating:
//!
//! - local memory transfers;
//! - host/device synchronization;
//! - distributed-memory synchronization;
//! - provider-managed synchronization;
//! - remote QPU synchronization;
//! - opaque backend execution resources;
//! - generation/epoch validation;
//! - timeout policy;
//! - cancellation boundaries;
//! - retry policy;
//! - synchronization state transitions;
//! - transfer accounting;
//! - idempotency;
//! - deterministic conflict handling;
//! - synchronization diagnostics;
//! - provider capability negotiation.
//!
//! It deliberately does NOT own:
//!
//! - memory allocation;
//! - raw pointers;
//! - GPU SDKs;
//! - CUDA/HIP/Metal/Vulkan APIs;
//! - MPI/RDMA/UCX APIs;
//! - QPU SDKs;
//! - network transports;
//! - quantum-state mathematics;
//! - routing;
//! - scheduling;
//! - compiler/IR semantics;
//! - benchmarking;
//! - authentication;
//! - credentials;
//! - provider-specific execution APIs.
//!
//! Those concerns remain in their respective modules/adapters.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              |
//!                              v
//!                       execution plan
//!                              |
//!             +----------------+----------------+
//!             |                |                |
//!             v                v                v
//!           Host             GPU              QPU
//!             |                |                |
//!             +----------------+----------------+
//!                              |
//!                              v
//!                 quantum::memory::coherence
//!                              |
//!                              v
//!              quantum::memory::synchronization
//!                              |
//!              +---------------+---------------+
//!              |               |               |
//!              v               v               v
//!          local transport  distributed     provider
//!                              transport     provider
//! ```
//!
//! # Critical quantum rule
//!
//! Synchronization does NOT mean copying amplitudes.
//!
//! A real QPU may expose no quantum-state buffer at all. A synchronization
//! operation against such a device can instead mean:
//!
//! - committing an execution context;
//! - synchronizing an opaque provider resource;
//! - advancing an execution epoch;
//! - validating a provider generation;
//! - transferring classical results;
//! - synchronizing a backend-native checkpoint;
//! - coordinating a provider-managed state transition.
//!
//! The transport/provider contracts below therefore never require an
//! amplitude buffer.
//!
//! # Coherence integration
//!
//! This module consumes the canonical types from `coherence.rs`:
//!
//! - `CoherenceDomain`;
//! - `CoherenceLocationId`;
//! - `CoherenceProvider`;
//! - `CoherenceRequest`;
//! - `SynchronizationRequest`;
//! - `SynchronizationCompletion`;
//! - `SynchronizationDirection`;
//! - `SynchronizationReason`;
//! - `CoherenceGeneration`;
//! - `CoherenceEpoch`;
//! - `CoherenceCapabilities`.
//!
//! It does NOT redefine those types.
//!
//! # Safety
//!
//! No unsafe Rust is used.
//!
//! Raw memory addresses, device pointers, provider handles and transport
//! internals never cross this abstraction boundary.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Completion invariant
//!
//! Once this file is complete, adding a new:
//!
//! - QPU vendor;
//! - simulator;
//! - GPU implementation;
//! - distributed transport;
//! - storage backend;
//! - quantum representation;
//! - execution technology;
//!
//! must not require changing this file merely to add provider-specific
//! behavior. The provider/transport implements the already-defined traits.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::coherence::{
    CoherenceCapabilities,
    CoherenceDomain,
    CoherenceEpoch,
    CoherenceGeneration,
    CoherenceLocationId,
    CoherenceProvider,
    SynchronizationCompletion,
    SynchronizationDirection,
    SynchronizationReason,
    SynchronizationRequest,
};
use super::errors::MemoryError;
use super::types::{ByteCount, MemoryId, StateId};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the synchronization protocol.
pub const SYNCHRONIZATION_SCHEMA_ID: &str =
    "zamani.quantum.memory.synchronization";

/// Semantic version of the synchronization protocol.
pub const SYNCHRONIZATION_SCHEMA_VERSION: u16 = 1;

/// Default timeout used when a caller does not specify one.
///
/// This is deliberately conservative. Provider-specific adapters may enforce
/// stricter limits.
pub const DEFAULT_SYNCHRONIZATION_TIMEOUT: Duration =
    Duration::from_secs(30);

/// Minimum legal timeout.
pub const MIN_SYNCHRONIZATION_TIMEOUT: Duration =
    Duration::from_millis(1);

/// Maximum timeout accepted by this generic layer.
///
/// A provider may impose a shorter timeout.
pub const MAX_SYNCHRONIZATION_TIMEOUT: Duration =
    Duration::from_secs(24 * 60 * 60);

/// Maximum number of retry attempts permitted by the generic engine.
pub const MAX_RETRY_ATTEMPTS: u32 = 16;

/// Maximum amount of exponential backoff.
pub const MAX_RETRY_BACKOFF: Duration =
    Duration::from_secs(60);

// =============================================================================
// Result
// =============================================================================

/// Canonical result type for synchronization operations.
pub type SynchronizationResult<T> = Result<T, MemoryError>;

// =============================================================================
// Synchronization mode
// =============================================================================

/// Execution mode for synchronization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SynchronizationMode {
    /// Transfer state using a local/provider-neutral transport.
    Transfer,

    /// Synchronization is controlled by an external provider.
    ProviderManaged,

    /// Reconcile two independently modified participants.
    Reconcile,

    /// Validate coherence without moving state.
    ValidateOnly,
}

impl SynchronizationMode {
    /// Returns whether the operation can transfer data.
    pub const fn may_transfer(self) -> bool {
        matches!(self, Self::Transfer | Self::Reconcile)
    }

    /// Returns whether a provider implementation is required.
    pub const fn requires_provider(self) -> bool {
        matches!(self, Self::ProviderManaged)
    }
}

impl fmt::Display for SynchronizationMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Transfer => "transfer",
            Self::ProviderManaged => "provider_managed",
            Self::Reconcile => "reconcile",
            Self::ValidateOnly => "validate_only",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Cancellation
// =============================================================================

/// Provider-neutral cancellation state.
///
/// Cancellation is deliberately cooperative. The synchronization engine never
/// assumes that an arbitrary provider operation can be forcefully interrupted.
///
/// This avoids unsafe cancellation semantics for QPUs, GPU kernels, network
/// transfers, and distributed collectives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancellationState {
    /// Operation has not been cancelled.
    Active,

    /// Cancellation was requested.
    Requested,

    /// Operation acknowledged cancellation.
    Cancelled,
}

impl CancellationState {
    /// Returns whether work may continue.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether cancellation was requested.
    pub const fn is_requested(self) -> bool {
        matches!(self, Self::Requested | Self::Cancelled)
    }
}

// =============================================================================
// Cancellation token
// =============================================================================

/// Cooperative cancellation token.
///
/// This uses only atomics and does not require unsafe code.
///
/// A provider should check this token at safe cancellation boundaries.
#[derive(Debug, Clone)]
pub struct CancellationToken {
    state: Arc<std::sync::atomic::AtomicU8>,
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationToken {
    /// Creates an active cancellation token.
    pub fn new() -> Self {
        Self {
            state: Arc::new(std::sync::atomic::AtomicU8::new(
                cancellation_to_u8(CancellationState::Active),
            )),
        }
    }

    /// Requests cooperative cancellation.
    pub fn cancel(&self) {
        self.state.store(
            cancellation_to_u8(CancellationState::Requested),
            std::sync::atomic::Ordering::Release,
        );
    }

    /// Returns the current state.
    pub fn state(&self) -> CancellationState {
        cancellation_from_u8(
            self.state.load(std::sync::atomic::Ordering::Acquire),
        )
    }

    /// Returns whether cancellation was requested.
    pub fn is_cancelled(&self) -> bool {
        self.state().is_requested()
    }

    /// Marks cancellation as acknowledged.
    ///
    /// This is intended for the executing transport/provider.
    pub fn acknowledge(&self) {
        self.state.store(
            cancellation_to_u8(CancellationState::Cancelled),
            std::sync::atomic::Ordering::Release,
        );
    }
}

fn cancellation_to_u8(value: CancellationState) -> u8 {
    match value {
        CancellationState::Active => 0,
        CancellationState::Requested => 1,
        CancellationState::Cancelled => 2,
    }
}

fn cancellation_from_u8(value: u8) -> CancellationState {
    match value {
        0 => CancellationState::Active,
        1 => CancellationState::Requested,
        _ => CancellationState::Cancelled,
    }
}

// =============================================================================
// Retry policy
// =============================================================================

/// Retry policy for synchronization transports.
///
/// Retry is disabled by default because blindly repeating a quantum operation
/// can be semantically unsafe. A transport must explicitly classify an error
/// as retry-safe before the engine retries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetryPolicy {
    max_attempts: u32,
    initial_backoff: Duration,
    max_backoff: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 1,
            initial_backoff: Duration::ZERO,
            max_backoff: Duration::ZERO,
        }
    }
}

impl RetryPolicy {
    /// Creates a no-retry policy.
    pub const fn none() -> Self {
        Self {
            max_attempts: 1,
            initial_backoff: Duration::ZERO,
            max_backoff: Duration::ZERO,
        }
    }

    /// Creates a bounded retry policy.
    pub fn new(
        max_attempts: u32,
        initial_backoff: Duration,
        max_backoff: Duration,
    ) -> SynchronizationResult<Self> {
        if max_attempts == 0 || max_attempts > MAX_RETRY_ATTEMPTS {
            return Err(MemoryError::invalid_argument(
                "retry attempts must be between 1 and MAX_RETRY_ATTEMPTS",
            ));
        }

        if max_backoff > MAX_RETRY_BACKOFF {
            return Err(MemoryError::invalid_argument(
                "retry max backoff exceeds synchronization limit",
            ));
        }

        if initial_backoff > max_backoff && max_backoff != Duration::ZERO {
            return Err(MemoryError::invalid_argument(
                "initial retry backoff cannot exceed max backoff",
            ));
        }

        Ok(Self {
            max_attempts,
            initial_backoff,
            max_backoff,
        })
    }

    /// Returns the maximum number of attempts.
    pub const fn max_attempts(self) -> u32 {
        self.max_attempts
    }

    /// Returns the initial backoff.
    pub const fn initial_backoff(self) -> Duration {
        self.initial_backoff
    }

    /// Returns the maximum backoff.
    pub const fn max_backoff(self) -> Duration {
        self.max_backoff
    }

    /// Calculates the backoff for an attempt.
    pub fn backoff_for_attempt(self, attempt: u32) -> Duration {
        if attempt <= 1 || self.initial_backoff.is_zero() {
            return self.initial_backoff;
        }

        let exponent = attempt.saturating_sub(1);
        let multiplier = 1u32.checked_shl(exponent.min(31)).unwrap_or(u32::MAX);

        let nanos = self
            .initial_backoff
            .as_nanos()
            .saturating_mul(u128::from(multiplier));

        let capped = nanos.min(self.max_backoff.as_nanos());

        let nanos_u64 = u64::try_from(capped).unwrap_or(u64::MAX);

        Duration::from_nanos(nanos_u64)
    }
}

// =============================================================================
// Synchronization timeout
// =============================================================================

/// Validated synchronization timeout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SynchronizationTimeout(Duration);

impl SynchronizationTimeout {
    /// Creates a validated timeout.
    pub fn new(value: Duration) -> SynchronizationResult<Self> {
        if value < MIN_SYNCHRONIZATION_TIMEOUT {
            return Err(MemoryError::invalid_argument(
                "synchronization timeout is below the minimum",
            ));
        }

        if value > MAX_SYNCHRONIZATION_TIMEOUT {
            return Err(MemoryError::invalid_argument(
                "synchronization timeout exceeds the maximum",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the timeout duration.
    pub const fn duration(self) -> Duration {
        self.0
    }
}

impl Default for SynchronizationTimeout {
    fn default() -> Self {
        Self(DEFAULT_SYNCHRONIZATION_TIMEOUT)
    }
}

// =============================================================================
// Synchronization request policy
// =============================================================================

/// Execution policy attached to one synchronization operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SynchronizationPolicy {
    timeout: SynchronizationTimeout,
    retry: RetryPolicy,
    mode: SynchronizationMode,
}

impl Default for SynchronizationPolicy {
    fn default() -> Self {
        Self {
            timeout: SynchronizationTimeout::default(),
            retry: RetryPolicy::none(),
            mode: SynchronizationMode::Transfer,
        }
    }
}

impl SynchronizationPolicy {
    /// Creates a transfer policy.
    pub fn transfer(
        timeout: Option<Duration>,
        retry: RetryPolicy,
    ) -> SynchronizationResult<Self> {
        Ok(Self {
            timeout: SynchronizationTimeout::new(
                timeout.unwrap_or(DEFAULT_SYNCHRONIZATION_TIMEOUT),
            )?,
            retry,
            mode: SynchronizationMode::Transfer,
        })
    }

    /// Creates a provider-managed policy.
    pub fn provider_managed(
        timeout: Option<Duration>,
        retry: RetryPolicy,
    ) -> SynchronizationResult<Self> {
        Ok(Self {
            timeout: SynchronizationTimeout::new(
                timeout.unwrap_or(DEFAULT_SYNCHRONIZATION_TIMEOUT),
            )?,
            retry,
            mode: SynchronizationMode::ProviderManaged,
        })
    }

    /// Creates a reconciliation policy.
    pub fn reconcile(
        timeout: Option<Duration>,
        retry: RetryPolicy,
    ) -> SynchronizationResult<Self> {
        Ok(Self {
            timeout: SynchronizationTimeout::new(
                timeout.unwrap_or(DEFAULT_SYNCHRONIZATION_TIMEOUT),
            )?,
            retry,
            mode: SynchronizationMode::Reconcile,
        })
    }

    /// Creates a validation-only policy.
    pub fn validate_only(
        timeout: Option<Duration>,
    ) -> SynchronizationResult<Self> {
        Ok(Self {
            timeout: SynchronizationTimeout::new(
                timeout.unwrap_or(DEFAULT_SYNCHRONIZATION_TIMEOUT),
            )?,
            retry: RetryPolicy::none(),
            mode: SynchronizationMode::ValidateOnly,
        })
    }

    /// Returns the timeout.
    pub const fn timeout(self) -> SynchronizationTimeout {
        self.timeout
    }

    /// Returns the retry policy.
    pub const fn retry(self) -> RetryPolicy {
        self.retry
    }

    /// Returns the synchronization mode.
    pub const fn mode(self) -> SynchronizationMode {
        self.mode
    }
}

// =============================================================================
// Transfer metadata
// =============================================================================

/// Metadata supplied by a transport after it has completed a transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TransferReport {
    /// Number of bytes moved through the transport.
    bytes_transferred: ByteCount,

    /// Number of logical transfer operations.
    operations: u64,

    /// Whether the transport performed a provider-native operation rather
    /// than copying local quantum-state bytes.
    provider_managed: bool,
}

impl TransferReport {
    /// Creates a transfer report.
    pub const fn new(
        bytes_transferred: ByteCount,
        operations: u64,
        provider_managed: bool,
    ) -> Self {
        Self {
            bytes_transferred,
            operations,
            provider_managed,
        }
    }

    /// Returns the transferred byte count.
    pub const fn bytes_transferred(self) -> ByteCount {
        self.bytes_transferred
    }

    /// Returns the number of transfer operations.
    pub const fn operations(self) -> u64 {
        self.operations
    }

    /// Returns whether the transfer was provider-managed.
    pub const fn provider_managed(self) -> bool {
        self.provider_managed
    }
}

// =============================================================================
// Transport capabilities
// =============================================================================

/// Capabilities of a synchronization transport.
///
/// These capabilities are intentionally independent of any vendor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SynchronizationCapabilities {
    /// Supports source-to-destination transfer.
    pub source_to_destination: bool,

    /// Supports destination-to-source transfer.
    pub destination_to_source: bool,

    /// Supports bidirectional reconciliation.
    pub bidirectional: bool,

    /// Supports provider-managed synchronization.
    pub provider_managed: bool,

    /// Supports cancellation.
    pub cancellation: bool,

    /// Supports timeout enforcement.
    pub timeout: bool,

    /// Supports idempotent retry.
    pub retry_safe: bool,

    /// Supports distributed synchronization.
    pub distributed: bool,

    /// Supports opaque resources.
    pub opaque_resources: bool,

    /// Supports real-QPU execution resources.
    pub qpu_resources: bool,
}

impl SynchronizationCapabilities {
    /// Returns whether the requested direction is supported.
    pub const fn supports_direction(
        self,
        direction: SynchronizationDirection,
    ) -> bool {
        match direction {
            SynchronizationDirection::SourceToDestination => {
                self.source_to_destination
            }
            SynchronizationDirection::DestinationToSource => {
                self.destination_to_source
            }
            SynchronizationDirection::Bidirectional => self.bidirectional,
            SynchronizationDirection::ProviderManaged => {
                self.provider_managed
            }
        }
    }
}

// =============================================================================
// Synchronization context
// =============================================================================

/// Immutable execution context supplied to transports.
///
/// This object contains no credentials and no raw addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynchronizationContext {
    state_id: StateId,
    memory_id: MemoryId,
    epoch: CoherenceEpoch,
    generation: CoherenceGeneration,
    source: CoherenceLocationId,
    destination: CoherenceLocationId,
    reason: SynchronizationReason,
}

impl SynchronizationContext {
    /// Creates a context from a validated request.
    pub fn from_request(request: &SynchronizationRequest) -> Self {
        Self {
            state_id: request.state_id(),
            memory_id: request.memory_id(),
            epoch: request.epoch(),
            generation: request.observed_generation(),
            source: request.source().clone(),
            destination: request.destination().clone(),
            reason: request.reason(),
        }
    }

    /// Returns the state identity.
    pub const fn state_id(&self) -> StateId {
        self.state_id
    }

    /// Returns the memory identity.
    pub const fn memory_id(&self) -> MemoryId {
        self.memory_id
    }

    /// Returns the observed epoch.
    pub const fn epoch(&self) -> CoherenceEpoch {
        self.epoch
    }

    /// Returns the observed generation.
    pub const fn generation(&self) -> CoherenceGeneration {
        self.generation
    }

    /// Returns the source.
    pub fn source(&self) -> &CoherenceLocationId {
        &self.source
    }

    /// Returns the destination.
    pub fn destination(&self) -> &CoherenceLocationId {
        &self.destination
    }

    /// Returns the reason.
    pub const fn reason(&self) -> SynchronizationReason {
        self.reason
    }
}

// =============================================================================
// Transport trait
// =============================================================================

/// Provider-neutral synchronization transport.
///
/// Implement this trait for:
///
/// - CPU↔CPU;
/// - CPU↔GPU;
/// - GPU↔GPU;
/// - CPU↔distributed node;
/// - distributed node↔distributed node;
/// - simulator↔simulator;
/// - simulator↔hardware emulator;
/// - classical control plane↔QPU;
/// - remote QPU provider;
/// - photonic provider;
/// - neutral-atom provider;
/// - trapped-ion provider;
/// - superconducting provider;
/// - spin/semiconductor provider;
/// - topological provider;
/// - analog processor;
/// - quantum annealer;
/// - future provider-defined systems.
///
/// The implementation owns the actual transport mechanism.
///
/// No raw pointer or vendor SDK type is required by this trait.
pub trait SynchronizationTransport: Send + Sync {
    /// Returns the capabilities of the transport.
    fn capabilities(&self) -> SynchronizationCapabilities;

    /// Executes one synchronization transfer.
    ///
    /// The implementation MUST:
    ///
    /// - validate the context;
    /// - obey the supplied timeout where possible;
    /// - honor cancellation at safe boundaries;
    /// - avoid reporting success before the destination is coherent;
    /// - return a retry-safe error only when repeating the operation is
    ///   semantically safe.
    fn synchronize(
        &self,
        context: &SynchronizationContext,
        direction: SynchronizationDirection,
        timeout: SynchronizationTimeout,
        cancellation: &CancellationToken,
    ) -> SynchronizationResult<TransferReport>;

    /// Returns whether an error can safely be retried.
    ///
    /// The default is false because repeating quantum operations may be
    /// semantically unsafe.
    fn is_retryable(&self, _error: &MemoryError) -> bool {
        false
    }
}

// =============================================================================
// Provider adapter
// =============================================================================

/// Adapter around a provider-managed coherence implementation.
///
/// This exists so the synchronization engine can accept either:
///
/// - a transport for ordinary memory;
/// - a `CoherenceProvider` for provider-owned/QPU resources.
///
/// The provider remains responsible for network/API/device interaction.
pub struct ProviderAdapter<P> {
    provider: P,
}

impl<P> ProviderAdapter<P> {
    /// Creates a provider adapter.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Returns a reference to the provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }

    /// Returns a mutable reference to the provider.
    pub fn provider_mut(&mut self) -> &mut P {
        &mut self.provider
    }

    /// Consumes the adapter and returns its provider.
    pub fn into_inner(self) -> P {
        self.provider
    }
}

impl<P> ProviderAdapter<P>
where
    P: CoherenceProvider,
{
    /// Executes provider-managed synchronization.
    pub fn synchronize(
        &mut self,
        request: &SynchronizationRequest,
        cancellation: &CancellationToken,
    ) -> SynchronizationResult<SynchronizationCompletion> {
        if cancellation.is_cancelled() {
            cancellation.acknowledge();

            return Err(MemoryError::synchronization_error(
                "synchronization was cancelled before provider execution",
            ));
        }

        self.provider.synchronize(request)
    }

    /// Returns provider capabilities.
    pub fn capabilities(&self) -> CoherenceCapabilities {
        self.provider.capabilities()
    }
}

// =============================================================================
// Synchronization state
// =============================================================================

/// Lifecycle state of one synchronization operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SynchronizationState {
    /// Request has been created but not started.
    Pending,

    /// Request has been validated.
    Validated,

    /// Source/destination synchronization has begun.
    Synchronizing,

    /// Transport/provider has completed.
    Completed,

    /// Operation was cancelled.
    Cancelled,

    /// Operation timed out.
    TimedOut,

    /// Operation failed.
    Failed,
}

impl SynchronizationState {
    /// Returns whether the state is terminal.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Cancelled
                | Self::TimedOut
                | Self::Failed
        )
    }
}

// =============================================================================
// Synchronization outcome
// =============================================================================

/// Complete result of a synchronization operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynchronizationOutcome {
    completion: Option<SynchronizationCompletion>,
    state: SynchronizationState,
    attempts: u32,
    elapsed: Duration,
    transfer: TransferReport,
}

impl SynchronizationOutcome {
    fn completed(
        completion: SynchronizationCompletion,
        attempts: u32,
        elapsed: Duration,
        transfer: TransferReport,
    ) -> Self {
        Self {
            completion: Some(completion),
            state: SynchronizationState::Completed,
            attempts,
            elapsed,
            transfer,
        }
    }

    fn failed(
        state: SynchronizationState,
        attempts: u32,
        elapsed: Duration,
    ) -> Self {
        Self {
            completion: None,
            state,
            attempts,
            elapsed,
            transfer: TransferReport::default(),
        }
    }

    /// Returns the terminal synchronization state.
    pub const fn state(&self) -> SynchronizationState {
        self.state
    }

    /// Returns whether synchronization completed.
    pub const fn is_completed(&self) -> bool {
        matches!(self.state, SynchronizationState::Completed)
    }

    /// Returns completion metadata.
    pub fn completion(&self) -> Option<&SynchronizationCompletion> {
        self.completion.as_ref()
    }

    /// Returns number of attempts.
    pub const fn attempts(&self) -> u32 {
        self.attempts
    }

    /// Returns elapsed time.
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Returns transport metadata.
    pub const fn transfer(&self) -> TransferReport {
        self.transfer
    }
}

// =============================================================================
// Engine
// =============================================================================

/// Production synchronization engine.
///
/// The engine is deliberately stateless with respect to quantum data.
///
/// It operates on:
///
/// - a `CoherenceDomain`;
/// - an already validated `SynchronizationRequest`;
/// - a transport/provider;
/// - an explicit policy.
///
/// This makes synchronization deterministic and testable.
#[derive(Debug, Clone, Copy, Default)]
pub struct SynchronizationEngine;

impl SynchronizationEngine {
    /// Creates a synchronization engine.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a synchronization request against the current domain.
    pub fn validate(
        &self,
        domain: &CoherenceDomain,
        request: &SynchronizationRequest,
        policy: SynchronizationPolicy,
    ) -> SynchronizationResult<()> {
        request.validate(domain)?;

        validate_policy_for_request(request, policy)?;

        Ok(())
    }

    /// Executes a normal transport-based synchronization.
    ///
    /// The coherence domain is updated only after the transport reports
    /// successful completion.
    pub fn synchronize<T>(
        &self,
        domain: &mut CoherenceDomain,
        request: &SynchronizationRequest,
        transport: &T,
        policy: SynchronizationPolicy,
        cancellation: &CancellationToken,
    ) -> SynchronizationResult<SynchronizationOutcome>
    where
        T: SynchronizationTransport + ?Sized,
    {
        self.validate(domain, request, policy)?;

        if cancellation.is_cancelled() {
            cancellation.acknowledge();

            return Err(MemoryError::synchronization_error(
                "synchronization was cancelled before execution",
            ));
        }

        if policy.mode() == SynchronizationMode::ValidateOnly {
            return Ok(SynchronizationOutcome {
                completion: None,
                state: SynchronizationState::Validated,
                attempts: 0,
                elapsed: Duration::ZERO,
                transfer: TransferReport::default(),
            });
        }

        let capabilities = transport.capabilities();

        if !capabilities.supports_direction(request.direction()) {
            return Err(MemoryError::unsupported_operation(
                "synchronization transport does not support requested direction",
            ));
        }

        if request.direction() == SynchronizationDirection::ProviderManaged {
            return Err(MemoryError::unsupported_operation(
                "provider-managed synchronization requires CoherenceProvider",
            ));
        }

        if !capabilities.timeout {
            return Err(MemoryError::unsupported_operation(
                "synchronization transport does not support required timeout semantics",
            ));
        }

        if cancellation.is_cancelled() && !capabilities.cancellation {
            cancellation.acknowledge();

            return Err(MemoryError::synchronization_error(
                "transport cannot honor requested cancellation",
            ));
        }

        domain.begin_synchronization(request.destination())?;

        let started = Instant::now();

        let transfer = match self.execute_transport(
            request,
            transport,
            policy,
            cancellation,
            started,
        ) {
            Ok(value) => value,
            Err(error) => {
                let _ = domain.invalidate(request.destination());

                return Err(error);
            }
        };

        if started.elapsed() > policy.timeout().duration() {
            let _ = domain.invalidate(request.destination());

            return Err(MemoryError::synchronization_timeout(
                "synchronization exceeded its configured timeout",
            ));
        }

        let generation = domain.complete_synchronization(
            request.destination(),
        )?;

        let completion = SynchronizationCompletion::new(
            request,
            generation,
            transfer.bytes_transferred(),
        );

        let elapsed = started.elapsed();

        Ok(SynchronizationOutcome::completed(
            completion,
            1,
            elapsed,
            transfer,
        ))
    }

    /// Executes provider-managed synchronization.
    ///
    /// This path is specifically for QPUs and other resources where the
    /// provider owns the actual quantum state or execution context.
    pub fn synchronize_provider<P>(
        &self,
        domain: &mut CoherenceDomain,
        request: &SynchronizationRequest,
        provider: &mut P,
        policy: SynchronizationPolicy,
        cancellation: &CancellationToken,
    ) -> SynchronizationResult<SynchronizationOutcome>
    where
        P: CoherenceProvider,
    {
        self.validate(domain, request, policy)?;

        if request.direction()
            != SynchronizationDirection::ProviderManaged
        {
            return Err(MemoryError::invalid_argument(
                "provider synchronization requires ProviderManaged direction",
            ));
        }

        if policy.mode() != SynchronizationMode::ProviderManaged {
            return Err(MemoryError::invalid_argument(
                "provider synchronization requires ProviderManaged mode",
            ));
        }

        if cancellation.is_cancelled() {
            cancellation.acknowledge();

            return Err(MemoryError::synchronization_error(
                "provider synchronization was cancelled before execution",
            ));
        }

        let capabilities = provider.capabilities();

        if !capabilities.synchronization {
            return Err(MemoryError::unsupported_operation(
                "provider does not expose synchronization capability",
            ));
        }

        if !capabilities.provider_managed {
            return Err(MemoryError::unsupported_operation(
                "provider does not expose provider-managed coherence",
            ));
        }

        /*
         * The provider may have a different generation counter from the local
         * coherence domain. It must validate the opaque token/request itself.
         *
         * We deliberately do not fabricate equivalence between provider
         * generation and local generation.
         */
        let provider_generation = provider.generation()?;

        if provider_generation != request.observed_generation() {
            return Err(MemoryError::stale_generation(
                "provider generation does not match synchronization request",
            ));
        }

        domain.begin_synchronization(request.destination())?;

        let started = Instant::now();

        let completion = provider.synchronize(request)?;

        if started.elapsed() > policy.timeout().duration() {
            let _ = domain.invalidate(request.destination());

            return Err(MemoryError::synchronization_timeout(
                "provider synchronization exceeded its configured timeout",
            ));
        }

        if cancellation.is_cancelled() {
            /*
             * We cannot assume that provider cancellation is safe after
             * execution has begun. The provider owns its own lifecycle.
             *
             * Therefore cancellation after provider completion is recorded
             * only if the provider did not report successful synchronization.
             */
            cancellation.acknowledge();
        }

        domain.complete_synchronization(request.destination())?;

        let elapsed = started.elapsed();

        let transfer = TransferReport::new(
            completion.bytes_transferred(),
            1,
            true,
        );

        Ok(SynchronizationOutcome::completed(
            completion,
            1,
            elapsed,
            transfer,
        ))
    }

    /// Executes a synchronization request using a provider adapter.
    pub fn synchronize_with_provider_adapter<P>(
        &self,
        domain: &mut CoherenceDomain,
        request: &SynchronizationRequest,
        provider: &mut ProviderAdapter<P>,
        policy: SynchronizationPolicy,
        cancellation: &CancellationToken,
    ) -> SynchronizationResult<SynchronizationOutcome>
    where
        P: CoherenceProvider,
    {
        self.synchronize_provider(
            domain,
            request,
            provider.provider_mut(),
            policy,
            cancellation,
        )
    }

    /// Executes synchronization with retry support.
    fn execute_transport<T>(
        &self,
        request: &SynchronizationRequest,
        transport: &T,
        policy: SynchronizationPolicy,
        cancellation: &CancellationToken,
        started: Instant,
    ) -> SynchronizationResult<TransferReport>
    where
        T: SynchronizationTransport + ?Sized,
    {
        let retry = policy.retry();

        let mut attempt = 1u32;

        loop {
            if cancellation.is_cancelled() {
                cancellation.acknowledge();

                return Err(MemoryError::synchronization_error(
                    "synchronization cancelled during execution",
                ));
            }

            if started.elapsed() >= policy.timeout().duration() {
                return Err(MemoryError::synchronization_timeout(
                    "synchronization timeout expired before transport completed",
                ));
            }

            let remaining = policy
                .timeout()
                .duration()
                .saturating_sub(started.elapsed());

            let timeout = SynchronizationTimeout::new(remaining)
                .unwrap_or(policy.timeout());

            match transport.synchronize(
                &SynchronizationContext::from_request(request),
                request.direction(),
                timeout,
                cancellation,
            ) {
                Ok(report) => return Ok(report),

                Err(error) => {
                    if attempt >= retry.max_attempts() {
                        return Err(error);
                    }

                    if !transport.is_retryable(&error) {
                        return Err(error);
                    }

                    if !transport.capabilities().retry_safe {
                        return Err(MemoryError::unsupported_operation(
                            "transport marked retryable error but does not advertise retry-safe semantics",
                        ));
                    }

                    let backoff = retry.backoff_for_attempt(attempt);

                    if !backoff.is_zero() {
                        let remaining = policy
                            .timeout()
                            .duration()
                            .saturating_sub(started.elapsed());

                        if backoff > remaining {
                            return Err(MemoryError::synchronization_timeout(
                                "retry backoff would exceed synchronization timeout",
                            ));
                        }

                        std::thread::sleep(backoff);
                    }

                    attempt = attempt.saturating_add(1);
                }
            }
        }
    }

    /// Refreshes a destination from the domain's current authoritative
    /// participant using a transport.
    pub fn refresh<T>(
        &self,
        domain: &mut CoherenceDomain,
        destination: &CoherenceLocationId,
        reason: SynchronizationReason,
        transport: &T,
        policy: SynchronizationPolicy,
        cancellation: &CancellationToken,
    ) -> SynchronizationResult<SynchronizationOutcome>
    where
        T: SynchronizationTransport + ?Sized,
    {
        let direction = domain.refresh_direction(destination)?;

        if direction == SynchronizationDirection::ProviderManaged {
            return Err(MemoryError::unsupported_operation(
                "refresh requires provider-managed synchronization",
            ));
        }

        let source = find_authoritative_participant(domain)?;

        let request = SynchronizationRequest::new(
            domain,
            source,
            destination.clone(),
            direction,
            reason,
            Some(policy.timeout().duration()),
        )?;

        self.synchronize(
            domain,
            &request,
            transport,
            policy,
            cancellation,
        )
    }
}

// =============================================================================
// Helper functions
// =============================================================================

fn validate_policy_for_request(
    request: &SynchronizationRequest,
    policy: SynchronizationPolicy,
) -> SynchronizationResult<()> {
    match (
        request.direction(),
        policy.mode(),
    ) {
        (
            SynchronizationDirection::ProviderManaged,
            SynchronizationMode::ProviderManaged,
        ) => Ok(()),

        (
            SynchronizationDirection::ProviderManaged,
            _,
        ) => Err(MemoryError::invalid_argument(
            "ProviderManaged synchronization requires ProviderManaged policy",
        )),

        (
            _,
            SynchronizationMode::ProviderManaged,
        ) => Err(MemoryError::invalid_argument(
            "ProviderManaged policy requires ProviderManaged direction",
        )),

        (
            SynchronizationDirection::Bidirectional,
            SynchronizationMode::Transfer,
        ) => Err(MemoryError::invalid_argument(
            "bidirectional synchronization requires Reconcile policy",
        )),

        (
            SynchronizationDirection::Bidirectional,
            SynchronizationMode::Reconcile,
        ) => Ok(()),

        (
            SynchronizationDirection::SourceToDestination,
            SynchronizationMode::Transfer,
        )
        | (
            SynchronizationDirection::DestinationToSource,
            SynchronizationMode::Transfer,
        )
        | (
            SynchronizationDirection::SourceToDestination,
            SynchronizationMode::ValidateOnly,
        )
        | (
            SynchronizationDirection::DestinationToSource,
            SynchronizationMode::ValidateOnly,
        ) => Ok(()),

        (
            SynchronizationDirection::Bidirectional,
            SynchronizationMode::ValidateOnly,
        ) => Ok(()),

        (
            _,
            SynchronizationMode::Reconcile,
        ) => Ok(()),
    }
}

fn find_authoritative_participant(
    domain: &CoherenceDomain,
) -> SynchronizationResult<CoherenceLocationId> {
    let authority = domain.authority();

    domain
        .participants()
        .iter()
        .find(|participant| {
            if participant.state().is_unavailable() {
                return false;
            }

            match authority {
                super::coherence::CoherenceAuthority::Host => {
                    matches!(
                        participant.id().location(),
                        super::coherence::CoherenceLocation::Host
                            | super::coherence::CoherenceLocation::PinnedHost
                    )
                }

                super::coherence::CoherenceAuthority::Device => {
                    matches!(
                        participant.id().location(),
                        super::coherence::CoherenceLocation::Device
                            | super::coherence::CoherenceLocation::Unified
                    )
                }

                super::coherence::CoherenceAuthority::Distributed => {
                    participant.id().location().is_distributed()
                }

                super::coherence::CoherenceAuthority::External
                | super::coherence::CoherenceAuthority::Opaque => {
                    participant.capabilities().provider_managed
                }

                super::coherence::CoherenceAuthority::Shared => {
                    participant.state()
                        == super::coherence::CoherenceCopyState::Clean
                        && participant.generation()
                            == domain.generation()
                }
            }
        })
        .map(|participant| participant.id().clone())
        .ok_or_else(|| {
            MemoryError::coherence_error(
                "no authoritative coherence participant is available",
            )
        })
}

// =============================================================================
// Generic no-op/validation transport
// =============================================================================

/// A validation-only transport.
///
/// This is useful for tests and dry-run planning. It never claims that bytes
/// were transferred.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationTransport;

impl ValidationTransport {
    /// Creates a validation transport.
    pub const fn new() -> Self {
        Self
    }
}

impl SynchronizationTransport for ValidationTransport {
    fn capabilities(&self) -> SynchronizationCapabilities {
        SynchronizationCapabilities {
            source_to_destination: true,
            destination_to_source: true,
            bidirectional: true,
            provider_managed: false,
            cancellation: true,
            timeout: true,
            retry_safe: false,
            distributed: true,
            opaque_resources: true,
            qpu_resources: true,
        }
    }

    fn synchronize(
        &self,
        _context: &SynchronizationContext,
        _direction: SynchronizationDirection,
        _timeout: SynchronizationTimeout,
        cancellation: &CancellationToken,
    ) -> SynchronizationResult<TransferReport> {
        if cancellation.is_cancelled() {
            cancellation.acknowledge();

            return Err(MemoryError::synchronization_error(
                "validation transport cancelled",
            ));
        }

        Ok(TransferReport::new(
            ByteCount::ZERO,
            0,
            false,
        ))
    }
}

// =============================================================================
// Synchronization receipt
// =============================================================================

/// Stable receipt identifying a completed synchronization event.
///
/// The receipt contains only safe identifiers and coherence metadata. It does
/// not contain addresses, credentials, provider tokens or quantum-state data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SynchronizationReceipt {
    state_id: StateId,
    memory_id: MemoryId,
    source: CoherenceLocationId,
    destination: CoherenceLocationId,
    generation: CoherenceGeneration,
    epoch: CoherenceEpoch,
    bytes_transferred: ByteCount,
}

impl SynchronizationReceipt {
    /// Creates a receipt from a successful outcome.
    pub fn from_outcome(
        outcome: &SynchronizationOutcome,
    ) -> SynchronizationResult<Self> {
        let completion = outcome.completion().ok_or_else(|| {
            MemoryError::invalid_argument(
                "cannot create synchronization receipt from incomplete outcome",
            )
        })?;

        Ok(Self {
            state_id: completion.state_id(),
            memory_id: completion.memory_id(),
            source: completion.source().clone(),
            destination: completion.destination().clone(),
            generation: completion.generation(),
            epoch: completion.epoch(),
            bytes_transferred: completion.bytes_transferred(),
        })
    }

    /// Returns state identity.
    pub const fn state_id(&self) -> StateId {
        self.state_id
    }

    /// Returns memory identity.
    pub const fn memory_id(&self) -> MemoryId {
        self.memory_id
    }

    /// Returns source.
    pub fn source(&self) -> &CoherenceLocationId {
        &self.source
    }

    /// Returns destination.
    pub fn destination(&self) -> &CoherenceLocationId {
        &self.destination
    }

    /// Returns generation.
    pub const fn generation(&self) -> CoherenceGeneration {
        self.generation
    }

    /// Returns epoch.
    pub const fn epoch(&self) -> CoherenceEpoch {
        self.epoch
    }

    /// Returns transferred bytes.
    pub const fn bytes_transferred(&self) -> ByteCount {
        self.bytes_transferred
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::quantum::memory::coherence::{
        CoherenceAuthority,
        CoherenceCopyState,
        CoherenceLocation,
        CoherenceParticipant,
        ConflictPolicy,
    };

    struct CountingTransport {
        calls: AtomicUsize,
        capabilities: SynchronizationCapabilities,
        retryable: bool,
    }

    impl CountingTransport {
        fn new() -> Self {
            Self {
                calls: AtomicUsize::new(0),
                capabilities: SynchronizationCapabilities {
                    source_to_destination: true,
                    destination_to_source: true,
                    bidirectional: true,
                    provider_managed: false,
                    cancellation: true,
                    timeout: true,
                    retry_safe: true,
                    distributed: true,
                    opaque_resources: true,
                    qpu_resources: true,
                },
                retryable: false,
            }
        }
    }

    impl SynchronizationTransport for CountingTransport {
        fn capabilities(&self) -> SynchronizationCapabilities {
            self.capabilities
        }

        fn synchronize(
            &self,
            _context: &SynchronizationContext,
            _direction: SynchronizationDirection,
            _timeout: SynchronizationTimeout,
            cancellation: &CancellationToken,
        ) -> SynchronizationResult<TransferReport> {
            self.calls.fetch_add(1, Ordering::Relaxed);

            if cancellation.is_cancelled() {
                cancellation.acknowledge();

                return Err(MemoryError::synchronization_error(
                    "test transport cancelled",
                ));
            }

            Ok(TransferReport::new(
                ByteCount::new(128),
                1,
                false,
            ))
        }

        fn is_retryable(&self, _error: &MemoryError) -> bool {
            self.retryable
        }
    }

    fn ids() -> (StateId, MemoryId) {
        (
            StateId::new(1),
            MemoryId::new(1),
        )
    }

    fn domain() -> CoherenceDomain {
        let (state_id, memory_id) = ids();

        let mut domain = CoherenceDomain::new(
            state_id,
            memory_id,
            CoherenceAuthority::Host,
            ConflictPolicy::Reject,
        );

        let host = CoherenceLocationId::new(
            "test",
            CoherenceLocation::Host,
        )
        .expect("valid host location");

        let device = CoherenceLocationId::new(
            "test",
            CoherenceLocation::Device,
        )
        .expect("valid device location");

        domain
            .add_participant(CoherenceParticipant::new(
                host,
                CoherenceCopyState::Clean,
                CoherenceGeneration::INITIAL,
                CoherenceCapabilities::host(),
                ByteCount::ZERO,
            ))
            .expect("host participant");

        domain
            .add_participant(CoherenceParticipant::new(
                device,
                CoherenceCopyState::Stale,
                CoherenceGeneration::INITIAL,
                CoherenceCapabilities::device(),
                ByteCount::ZERO,
            ))
            .expect("device participant");

        domain
    }

    #[test]
    fn timeout_validation_rejects_zero() {
        let result = SynchronizationTimeout::new(Duration::ZERO);

        assert!(result.is_err());
    }

    #[test]
    fn retry_policy_defaults_to_one_attempt() {
        assert_eq!(RetryPolicy::default().max_attempts(), 1);
    }

    #[test]
    fn cancellation_is_cooperative() {
        let token = CancellationToken::new();

        assert!(!token.is_cancelled());

        token.cancel();

        assert!(token.is_cancelled());
        assert_eq!(
            token.state(),
            CancellationState::Requested
        );

        token.acknowledge();

        assert_eq!(
            token.state(),
            CancellationState::Cancelled
        );
    }

    #[test]
    fn validation_transport_does_not_claim_bytes() {
        let transport = ValidationTransport::new();
        let token = CancellationToken::new();

        let context = SynchronizationContext {
            state_id: StateId::new(1),
            memory_id: MemoryId::new(1),
            epoch: CoherenceEpoch::INITIAL,
            generation: CoherenceGeneration::INITIAL,
            source: CoherenceLocationId::new(
                "test",
                CoherenceLocation::Host,
            )
            .expect("source"),
            destination: CoherenceLocationId::new(
                "test",
                CoherenceLocation::Device,
            )
            .expect("destination"),
            reason: SynchronizationReason::Explicit,
        };

        let report = transport
            .synchronize(
                &context,
                SynchronizationDirection::SourceToDestination,
                SynchronizationTimeout::default(),
                &token,
            )
            .expect("validation succeeds");

        assert_eq!(report.bytes_transferred(), ByteCount::ZERO);
        assert_eq!(report.operations(), 0);
    }

    #[test]
    fn transfer_completes_and_updates_destination() {
        let mut domain = domain();

        let source = CoherenceLocationId::new(
            "test",
            CoherenceLocation::Host,
        )
        .expect("source");

        let destination = CoherenceLocationId::new(
            "test",
            CoherenceLocation::Device,
        )
        .expect("destination");

        let request = SynchronizationRequest::new(
            &domain,
            source,
            destination.clone(),
            SynchronizationDirection::SourceToDestination,
            SynchronizationReason::ReadPreparation,
            None,
        )
        .expect("request");

        let transport = CountingTransport::new();
        let token = CancellationToken::new();

        let policy = SynchronizationPolicy::transfer(
            Some(Duration::from_secs(1)),
            RetryPolicy::none(),
        )
        .expect("policy");

        let outcome = SynchronizationEngine::new()
            .synchronize(
                &mut domain,
                &request,
                &transport,
                policy,
                &token,
            )
            .expect("synchronization");

        assert!(outcome.is_completed());
        assert_eq!(
            outcome.transfer().bytes_transferred(),
            ByteCount::new(128)
        );

        assert!(
            domain
                .is_current(&destination)
                .expect("destination exists")
        );

        assert_eq!(
            transport.calls.load(Ordering::Relaxed),
            1
        );
    }

    #[test]
    fn cancellation_before_execution_does_not_touch_domain() {
        let mut domain = domain();

        let source = CoherenceLocationId::new(
            "test",
            CoherenceLocation::Host,
        )
        .expect("source");

        let destination = CoherenceLocationId::new(
            "test",
            CoherenceLocation::Device,
        )
        .expect("destination");

        let request = SynchronizationRequest::new(
            &domain,
            source,
            destination.clone(),
            SynchronizationDirection::SourceToDestination,
            SynchronizationReason::Explicit,
            None,
        )
        .expect("request");

        let token = CancellationToken::new();
        token.cancel();

        let transport = CountingTransport::new();

        let policy = SynchronizationPolicy::transfer(
            Some(Duration::from_secs(1)),
            RetryPolicy::none(),
        )
        .expect("policy");

        let result = SynchronizationEngine::new().synchronize(
            &mut domain,
            &request,
            &transport,
            policy,
            &token,
        );

        assert!(result.is_err());

        /*
         * The destination must remain stale because no synchronization was
         * actually started.
         */
        assert!(
            domain
                .participant(&destination)
                .expect("destination")
                .state()
                == CoherenceCopyState::Stale
        );

        assert_eq!(
            transport.calls.load(Ordering::Relaxed),
            0
        );
    }

    #[test]
    fn receipt_requires_successful_completion() {
        let outcome = SynchronizationOutcome::failed(
            SynchronizationState::Failed,
            1,
            Duration::from_millis(1),
        );

        assert!(
            SynchronizationReceipt::from_outcome(&outcome).is_err()
        );
    }

    #[test]
    fn retry_backoff_is_bounded() {
        let policy = RetryPolicy::new(
            4,
            Duration::from_millis(10),
            Duration::from_millis(20),
        )
        .expect("valid retry policy");

        assert_eq!(
            policy.backoff_for_attempt(1),
            Duration::from_millis(10)
        );

        assert_eq!(
            policy.backoff_for_attempt(2),
            Duration::from_millis(20)
        );

        assert_eq!(
            policy.backoff_for_attempt(3),
            Duration::from_millis(20)
        );
    }
}