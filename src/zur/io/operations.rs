
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — I/O Operations
//!
//! Path: src/zur/io/operations.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # PURPOSE
//!
//! Provides the common operational lifecycle coordinator for ZUR I/O.
//!
//! This module coordinates:
//!
//! - Operation registration.
//! - Operation identity.
//! - Operation lifecycle.
//! - Cooperative cancellation observation.
//! - Progress accounting.
//! - Successful completion.
//! - Failure reporting.
//! - Timeout reporting.
//! - Operation snapshots.
//! - Explicit removal of completed operations.
//! - Concurrent access to operation records.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! This module owns I/O operation lifecycle coordination.
//!
//! It does NOT own:
//!
//! - I/O effect semantics.
//! - Source-language operation definitions.
//! - Filesystem implementation.
//! - Console implementation.
//! - Byte-stream implementation.
//! - Device communication.
//! - Device discovery.
//! - Hardware allocation.
//! - Network transport.
//! - Distributed execution.
//! - Scheduling.
//! - Async executors.
//! - Resource reservation.
//! - Memory allocation.
//! - Security authorization.
//! - Provider registration.
//! - Provider lifecycle.
//! - Compiler planning.
//! - IR definitions.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! # CANONICAL INTEGRATION
//!
//! Runtime errors:
//!     crate::zur::error
//!
//! Operation identities:
//!     crate::zur::ids
//!
//! Cancellation:
//!     crate::stdlib::context
//!
//! Streams:
//!     crate::zur::io::streams
//!
//! Host I/O:
//!     crate::zur::io::host
//!
//! Devices:
//!     crate::zur::io::devices
//!
//! Providers:
//!     crate::zur::contracts::provider
//!
//! Distributed transport:
//!     crate::distributed
//!
//! # IMPORTANT INVARIANTS
//!
//! 1. Every tracked operation has a caller-supplied canonical identity.
//! 2. Duplicate operation identities are rejected.
//! 3. An operation can complete only once.
//! 4. Terminal states cannot transition back to running.
//! 5. Cancellation is cooperative.
//! 6. Cancellation does not claim that external work stopped.
//! 7. Successful provider results are preserved.
//! 8. Provider failures remain failures.
//! 9. Submission is not completion.
//! 10. Progress is descriptive, not proof of resource reservation.
//! 11. Removing a record does not cancel its external operation.
//! 12. No operation is automatically reported as successful.
//!
//! # SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Operations.
//! - Providers.
//! - Streams.
//! - Devices.
//! - Transfers.
//! - Data volumes.
//! - Concurrent sessions.
//! - Computational domains.
//!
//! Storage grows according to available memory.
//!
//! Progress accounting uses u128 saturating counters.
//!
//! Saturation never rejects an otherwise successful I/O operation.
//!
//! Literal infinite physical resources are not promised.
//!
//! # SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No raw pointers.
//! No static mutable state.
//! No global operation registry.
//! No implicit provider discovery.
//! No fabricated completion.
//! No independent cancellation-token implementation.
//!
//! # ASYNCHRONOUS EXECUTION
//!
//! This module does not implement an executor.
//!
//! Its asynchronous operation wrapper accepts an existing future.
//!
//! The future is driven by the caller's execution environment.
//!
//! Creating an operation or future does not guarantee execution.

use core::fmt;
use core::future::Future;
use core::time::Duration;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::stdlib::context::OperationContext;

use crate::zur::error::{
    ErrorKind,
    ZurError,
    ZurResult,
};

use crate::zur::ids::RuntimeOperationId;

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

/// Stable ZUR I/O operations component identifier.
pub const COMPONENT: &str = "zur.io.operations";

/// Current ZUR I/O operations API version.
pub const API_VERSION: &str = "1.0.0";

// ============================================================================
// ERROR IDENTIFIERS
// ============================================================================

const DUPLICATE_OPERATION_CODE: &str =
    "zur.io.operations.duplicate";

const INVALID_OPERATION_CODE: &str =
    "zur.io.operations.invalid";

const INVALID_TRANSITION_CODE: &str =
    "zur.io.operations.invalid_transition";

const REGISTRY_POISON_CODE: &str =
    "zur.io.operations.registry_failure";

const INVALID_OPERATION_NAME_CODE: &str =
    "zur.io.operations.invalid_name";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn operation_error(
    kind: ErrorKind,
    code: &'static str,
    message: impl Into<String>,
) -> ZurError {
    // All codes above are reviewed static identifiers.
    ZurError::with_code(
        kind,
        code,
        message,
    )
    .expect("reviewed static ZUR error code must remain valid")
    .with_component(COMPONENT)
}

/// Creates an invalid-transition error.
fn transition_error(
    message: impl Into<String>,
) -> ZurError {
    operation_error(
        ErrorKind::Lifecycle,
        INVALID_TRANSITION_CODE,
        message,
    )
}

/// Creates a cancellation error.
fn cancellation_error(
    context: &OperationContext,
) -> ZurError {
    let reason = context
        .cancellation_reason()
        .unwrap_or_else(|| {
            "cancellation requested".to_owned()
        });

    ZurError::cancelled(reason)
        .with_component(COMPONENT)
}

// ============================================================================
// OPERATION STATUS
// ============================================================================

/// Actual lifecycle status of an I/O operation.
///
/// This is an operational state, not a semantic requirement,
/// capability, preference, constraint or feasibility result.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
#[non_exhaustive]
pub enum OperationStatus {
    /// Operation is executing or awaiting its execution future.
    Running,

    /// Operation completed successfully.
    Completed,

    /// Operation failed.
    Failed,

    /// Operation was cancelled.
    Cancelled,

    /// Operation exceeded an applicable deadline.
    TimedOut,
}

impl OperationStatus {
    /// Returns whether this status represents a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Failed
                | Self::Cancelled
                | Self::TimedOut
        )
    }

    /// Returns the stable textual status.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
        }
    }
}

// ============================================================================
// OPERATION FAILURE
// ============================================================================

/// Immutable diagnostic information about an operation failure.
///
/// The original ZurError remains available to the caller when
/// an operation fails.
///
/// The snapshot stores its public diagnostic representation
/// without attempting to clone its causal error chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationFailure {
    kind: ErrorKind,
    code: String,
    message: String,
}

impl OperationFailure {
    /// Creates a diagnostic snapshot from an actual ZUR error.
    #[must_use]
    pub fn from_error(
        error: &ZurError,
    ) -> Self {
        Self {
            kind: error.kind(),
            code: error.code().as_str().to_owned(),
            message: error.message().to_owned(),
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the stable error code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// TRANSFER ACCOUNTING
// ============================================================================

/// Descriptive transfer accounting for one operation.
///
/// The counter is independent of whether the underlying transfer
/// is local, remote, device-backed or provider-managed.
///
/// Saturation never affects execution.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
)]
pub struct OperationProgress {
    transferred: u128,
    saturated: bool,
}

impl OperationProgress {
    /// Creates empty progress accounting.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            transferred: 0,
            saturated: false,
        }
    }

    /// Returns the observed transferred amount.
    ///
    /// When saturated, the value is a lower bound.
    #[must_use]
    pub const fn transferred(&self) -> u128 {
        self.transferred
    }

    /// Returns whether the counter saturated.
    #[must_use]
    pub const fn is_saturated(&self) -> bool {
        self.saturated
    }

    /// Records an actual successful transfer.
    ///
    /// This method never rejects the underlying operation.
    fn record(
        &mut self,
        amount: u128,
    ) {
        match self.transferred.checked_add(amount) {
            Some(total) => {
                self.transferred = total;
            }

            None => {
                self.transferred = u128::MAX;
                self.saturated = true;
            }
        }
    }
}

// ============================================================================
// OPERATION SNAPSHOT
// ============================================================================

/// Immutable snapshot of an I/O operation.
///
/// A snapshot describes observed operational state.
///
/// It does not prove that external hardware has released
/// resources or that remote work has stopped.
#[derive(Debug, Clone)]
pub struct OperationSnapshot {
    id: RuntimeOperationId,
    name: String,
    status: OperationStatus,
    started_at: Instant,
    finished_at: Option<Instant>,
    progress: OperationProgress,
    failure: Option<OperationFailure>,
}

impl OperationSnapshot {
    /// Returns the canonical operation identity.
    #[must_use]
    pub fn id(&self) -> &RuntimeOperationId {
        &self.id
    }

    /// Returns the caller-supplied operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the observed lifecycle status.
    #[must_use]
    pub const fn status(&self) -> OperationStatus {
        self.status
    }

    /// Returns whether the operation is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }

    /// Returns the operation start instant.
    #[must_use]
    pub const fn started_at(&self) -> Instant {
        self.started_at
    }

    /// Returns the optional completion instant.
    #[must_use]
    pub const fn finished_at(&self) -> Option<Instant> {
        self.finished_at
    }

    /// Returns the elapsed operation duration.
    ///
    /// Running operations are measured up to the time of this call.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.finished_at
            .unwrap_or_else(Instant::now)
            .saturating_duration_since(self.started_at)
    }

    /// Returns observed transfer accounting.
    #[must_use]
    pub const fn progress(&self) -> OperationProgress {
        self.progress
    }

    /// Returns the optional failure diagnostic.
    #[must_use]
    pub fn failure(&self) -> Option<&OperationFailure> {
        self.failure.as_ref()
    }
}

// ============================================================================
// SHARED OPERATION RECORD
// ============================================================================

type SharedRecord = Arc<Mutex<OperationSnapshot>>;

/// Obtains an operation record while recovering a poisoned mutex.
///
/// A poisoned mutex indicates that another thread panicked while
/// holding the lock.
///
/// The record contains only lifecycle metadata, not partially
/// transferred external resources. Recovering the inner value
/// preserves diagnostic access without introducing unsafe code.
fn lock_record(
    record: &SharedRecord,
) -> std::sync::MutexGuard<'_, OperationSnapshot> {
    record
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

// ============================================================================
// OPERATION HANDLE
// ============================================================================

/// Handle to one tracked I/O operation.
///
/// Cloning the handle shares the same operation record.
///
/// All clones observe the same lifecycle state.
///
/// The operation context is the canonical cancellation authority.
///
/// A handle does not itself terminate an external provider operation.
#[derive(Clone)]
pub struct OperationHandle {
    record: SharedRecord,
    context: OperationContext,
}

impl OperationHandle {
    /// Returns the canonical operation identity.
    #[must_use]
    pub fn id(&self) -> RuntimeOperationId {
        lock_record(&self.record).id.clone()
    }

    /// Returns the current operation status.
    #[must_use]
    pub fn status(&self) -> OperationStatus {
        lock_record(&self.record).status
    }

    /// Returns whether the operation is terminal.
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        self.status().is_terminal()
    }

    /// Returns an immutable operation snapshot.
    #[must_use]
    pub fn snapshot(&self) -> OperationSnapshot {
        lock_record(&self.record).clone()
    }

    /// Returns the shared operation context.
    #[must_use]
    pub const fn context(&self) -> &OperationContext {
        &self.context
    }

    /// Checks the canonical cancellation signal.
    ///
    /// If cancellation was requested, the operation is marked
    /// cancelled before the error is returned.
    ///
    /// This does not forcibly interrupt an in-flight operation.
    pub fn check_cancelled(&self) -> ZurResult<()> {
        if !self.context.is_cancelled() {
            return Ok(());
        }

        let error = cancellation_error(&self.context);

        self.mark_terminal(
            OperationStatus::Cancelled,
            None,
        )?;

        Err(error)
    }

    /// Records actual transfer progress.
    ///
    /// The supplied amount must represent data actually accepted
    /// or received by the underlying I/O provider.
    ///
    /// Planned, requested or reserved bytes must not be recorded
    /// as completed transfers.
    pub fn record_progress(
        &self,
        amount: u128,
    ) -> ZurResult<()> {
        let mut record = lock_record(&self.record);

        if record.status.is_terminal() {
            return Err(transition_error(
                "cannot update progress after operation termination",
            ));
        }

        record.progress.record(amount);

        Ok(())
    }

    /// Marks the operation successfully completed.
    ///
    /// Completion is recorded only when the caller has observed
    /// actual successful completion from the underlying service.
    pub fn complete(&self) -> ZurResult<()> {
        self.mark_terminal(
            OperationStatus::Completed,
            None,
        )
    }

    /// Marks the operation failed using its actual error.
    ///
    /// The original error remains owned by the caller.
    ///
    /// The operation snapshot stores diagnostic information.
    pub fn fail(
        &self,
        error: &ZurError,
    ) -> ZurResult<()> {
        let status = match error.kind() {
            ErrorKind::Cancelled => OperationStatus::Cancelled,
            ErrorKind::TimedOut => OperationStatus::TimedOut,
            _ => OperationStatus::Failed,
        };

        self.mark_terminal(
            status,
            Some(OperationFailure::from_error(error)),
        )
    }

    /// Marks an operation cancelled after an actual cancellation
    /// outcome has been observed.
    ///
    /// A cancellation request alone does not prove that external
    /// work has stopped.
    pub fn mark_cancelled(
        &self,
    ) -> ZurResult<()> {
        let error = cancellation_error(&self.context);

        self.mark_terminal(
            OperationStatus::Cancelled,
            Some(OperationFailure::from_error(&error)),
        )
    }

    /// Marks an operation timed out after its deadline authority
    /// has established an actual timeout.
    ///
    /// This method does not implement deadline detection.
    pub fn mark_timed_out(
        &self,
        message: impl Into<String>,
    ) -> ZurResult<()> {
        let error = ZurError::timed_out(message)
            .with_component(COMPONENT);

        self.mark_terminal(
            OperationStatus::TimedOut,
            Some(OperationFailure::from_error(&error)),
        )
    }

    /// Runs an existing asynchronous operation future and tracks
    /// its actual outcome.
    ///
    /// This method does not create or drive an executor.
    ///
    /// The future is not polled if cancellation has already
    /// been requested.
    ///
    /// Successful results are preserved even if cancellation
    /// is requested while the provider operation is in flight.
    ///
    /// This is important because cancellation is cooperative
    /// and cannot retroactively invalidate actual provider results.
    ///
    /// # Errors
    ///
    /// Returns the original operation error.
    pub async fn run<F, T>(
        &self,
        future: F,
    ) -> ZurResult<T>
    where
        F: Future<Output = ZurResult<T>>,
    {
        self.check_cancelled()?;

        match future.await {
            Ok(value) => {
                self.complete()?;
                Ok(value)
            }

            Err(error) => {
                self.fail(&error)?;
                Err(error)
            }
        }
    }

    /// Performs a terminal lifecycle transition.
    ///
    /// A terminal operation cannot transition again.
    fn mark_terminal(
        &self,
        status: OperationStatus,
        failure: Option<OperationFailure>,
    ) -> ZurResult<()> {
        if !status.is_terminal() {
            return Err(transition_error(
                "requested lifecycle transition is not terminal",
            ));
        }

        let mut record = lock_record(&self.record);

        if record.status.is_terminal() {
            return Err(transition_error(
                "operation has already reached a terminal state",
            ));
        }

        record.status = status;
        record.finished_at = Some(Instant::now());
        record.failure = failure;

        Ok(())
    }
}

impl fmt::Debug for OperationHandle {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let record = lock_record(&self.record);

        formatter
            .debug_struct("OperationHandle")
            .field("id", &record.id)
            .field("name", &record.name)
            .field("status", &record.status)
            .field("progress", &record.progress)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// OPERATION REGISTRY
// ============================================================================

/// Explicitly constructed I/O operation registry.
///
/// The registry stores lifecycle records only.
///
/// It does not own the underlying streams, devices, providers,
/// remote operations or execution resources.
///
/// Multiple registries can exist independently.
///
/// There is no process-global registry.
#[derive(Clone, Default)]
pub struct IoOperationRegistry {
    records: Arc<Mutex<BTreeMap<RuntimeOperationId, SharedRecord>>>,
}

impl IoOperationRegistry {
    /// Creates an empty operation registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers and begins an I/O operation.
    ///
    /// The identity is supplied by the canonical ZUR identity
    /// authority.
    ///
    /// The operation name is open-world metadata.
    ///
    /// No operation-name inventory or computational-domain
    /// enumeration is maintained here.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The name is empty.
    /// - The name contains control characters.
    /// - The identity already exists in this registry.
    pub fn begin(
        &self,
        id: RuntimeOperationId,
        name: impl Into<String>,
        context: OperationContext,
    ) -> ZurResult<OperationHandle> {
        let name = name.into();

        if name.trim().is_empty()
            || name.chars().any(char::is_control)
        {
            return Err(operation_error(
                ErrorKind::InvalidInput,
                INVALID_OPERATION_NAME_CODE,
                "operation name must be non-empty and contain no control characters",
            ));
        }

        if context.is_cancelled() {
            return Err(cancellation_error(&context));
        }

        let snapshot = OperationSnapshot {
            id: id.clone(),
            name,
            status: OperationStatus::Running,
            started_at: Instant::now(),
            finished_at: None,
            progress: OperationProgress::new(),
            failure: None,
        };

        let record = Arc::new(Mutex::new(snapshot));

        let mut records = self
            .records
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if records.contains_key(&id) {
            return Err(operation_error(
                ErrorKind::Validation,
                DUPLICATE_OPERATION_CODE,
                "operation identity already exists in this registry",
            ));
        }

        records.insert(
            id,
            Arc::clone(&record),
        );

        Ok(OperationHandle {
            record,
            context,
        })
    }

    /// Returns a snapshot of a registered operation.
    ///
    /// Returns `None` if the identity is not registered.
    #[must_use]
    pub fn get(
        &self,
        id: &RuntimeOperationId,
    ) -> Option<OperationSnapshot> {
        let record = {
            let records = self
                .records
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            records.get(id).cloned()
        }?;

        Some(lock_record(&record).clone())
    }

    /// Returns the number of currently retained records.
    ///
    /// This is an observed registry size, not a configured
    /// maximum or capacity limit.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len()
    }

    /// Returns whether the registry currently contains no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns snapshots of all retained operations.
    ///
    /// The returned collection is a point-in-time observation.
    ///
    /// Ordering follows the canonical identity ordering.
    #[must_use]
    pub fn snapshots(&self) -> Vec<OperationSnapshot> {
        let records: Vec<SharedRecord> = self
            .records
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .values()
            .cloned()
            .collect();

        records
            .iter()
            .map(|record| lock_record(record).clone())
            .collect()
    }

    /// Returns the number of non-terminal operations.
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.records
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .values()
            .filter(|record| {
                !lock_record(record).status.is_terminal()
            })
            .count()
    }

    /// Removes one terminal operation record.
    ///
    /// A running operation cannot be removed.
    ///
    /// Removal affects registry observability only.
    ///
    /// It does not cancel an operation, close a stream,
    /// terminate a device connection or release external resources.
    ///
    /// Existing handles continue to own their shared records.
    pub fn remove_terminal(
        &self,
        id: &RuntimeOperationId,
    ) -> ZurResult<Option<OperationSnapshot>> {
        let mut records = self
            .records
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let Some(record) = records.get(id) else {
            return Ok(None);
        };

        if !lock_record(record).status.is_terminal() {
            return Err(transition_error(
                "cannot remove a running operation",
            ));
        }

        let removed = records.remove(id);

        Ok(removed.map(|record| {
            lock_record(&record).clone()
        }))
    }

    /// Removes all terminal records.
    ///
    /// Active operations are retained.
    ///
    /// The number of removed records is returned.
    ///
    /// This method is an explicit retention-management operation.
    /// It is never performed implicitly during I/O execution.
    pub fn remove_terminal_records(&self) -> usize {
        let mut records = self
            .records
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let terminal_ids: Vec<RuntimeOperationId> = records
            .iter()
            .filter_map(|(id, record)| {
                if lock_record(record).status.is_terminal() {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();

        let removed = terminal_ids.len();

        for id in terminal_ids {
            records.remove(&id);
        }

        removed
    }
}

impl fmt::Debug for IoOperationRegistry {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("IoOperationRegistry")
            .field("retained", &self.len())
            .field("active", &self.active_count())
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::ids::{
        IdNamespace,
        RuntimeOperationId,
    };

    fn operation_id(
        sequence: u8,
    ) -> RuntimeOperationId {
        RuntimeOperationId::new(
            IdNamespace::new(
                "zamani.zur.io.operation",
            )
            .expect("valid namespace"),
            vec![sequence],
        )
        .expect("valid operation identity")
    }

    fn context() -> OperationContext {
        OperationContext::builder()
            .with_component(COMPONENT)
            .expect("valid component")
            .with_operation("io.test")
            .expect("valid operation")
            .build()
    }

    fn registry() -> IoOperationRegistry {
        IoOperationRegistry::new()
    }

    // ------------------------------------------------------------------------
    // Status
    // ------------------------------------------------------------------------

    #[test]
    fn running_is_not_terminal() {
        assert!(
            !OperationStatus::Running.is_terminal()
        );
    }

    #[test]
    fn every_terminal_status_is_terminal() {
        assert!(
            OperationStatus::Completed.is_terminal()
        );

        assert!(
            OperationStatus::Failed.is_terminal()
        );

        assert!(
            OperationStatus::Cancelled.is_terminal()
        );

        assert!(
            OperationStatus::TimedOut.is_terminal()
        );
    }

    // ------------------------------------------------------------------------
    // Registry
    // ------------------------------------------------------------------------

    #[test]
    fn creates_empty_registry() {
        let registry = registry();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert_eq!(registry.active_count(), 0);
    }

    #[test]
    fn registers_operation() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.stream.read",
                context(),
            )
            .unwrap();

        assert_eq!(
            handle.status(),
            OperationStatus::Running,
        );

        assert_eq!(registry.len(), 1);
        assert_eq!(registry.active_count(), 1);
    }

    #[test]
    fn rejects_duplicate_operation_identity() {
        let registry = registry();

        registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        let result = registry.begin(
            operation_id(1),
            "io.write",
            context(),
        );

        assert!(result.is_err());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn rejects_empty_operation_name() {
        let registry = registry();

        assert!(
            registry
                .begin(
                    operation_id(1),
                    "",
                    context(),
                )
                .is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_operation_name() {
        let registry = registry();

        assert!(
            registry
                .begin(
                    operation_id(1),
                    "io\nread",
                    context(),
                )
                .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Lifecycle
    // ------------------------------------------------------------------------

    #[test]
    fn completes_operation() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        handle.complete().unwrap();

        assert_eq!(
            handle.status(),
            OperationStatus::Completed,
        );

        assert_eq!(registry.active_count(), 0);
    }

    #[test]
    fn cannot_complete_operation_twice() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        handle.complete().unwrap();

        assert!(handle.complete().is_err());
    }

    #[test]
    fn cannot_update_progress_after_completion() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        handle.complete().unwrap();

        assert!(
            handle.record_progress(1).is_err()
        );
    }

    #[test]
    fn clones_share_lifecycle_state() {
        let registry = registry();

        let original = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        let cloned = original.clone();

        original.complete().unwrap();

        assert_eq!(
            cloned.status(),
            OperationStatus::Completed,
        );
    }

    // ------------------------------------------------------------------------
    // Failure
    // ------------------------------------------------------------------------

    #[test]
    fn records_actual_failure() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        let error = ZurError::internal(
            "test provider failure",
        );

        handle.fail(&error).unwrap();

        let snapshot = handle.snapshot();

        assert_eq!(
            snapshot.status(),
            OperationStatus::Failed,
        );

        let failure = snapshot.failure().unwrap();

        assert_eq!(
            failure.message(),
            "test provider failure",
        );
    }

    #[test]
    fn classifies_cancellation_error() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        let error = ZurError::cancelled(
            "provider cancelled",
        );

        handle.fail(&error).unwrap();

        assert_eq!(
            handle.status(),
            OperationStatus::Cancelled,
        );
    }

    #[test]
    fn classifies_timeout_error() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        let error = ZurError::timed_out(
            "operation deadline exceeded",
        );

        handle.fail(&error).unwrap();

        assert_eq!(
            handle.status(),
            OperationStatus::TimedOut,
        );
    }

    #[test]
    fn records_explicit_timeout() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        handle
            .mark_timed_out(
                "deadline established by caller",
            )
            .unwrap();

        assert_eq!(
            handle.status(),
            OperationStatus::TimedOut,
        );
    }

    // ------------------------------------------------------------------------
    // Progress
    // ------------------------------------------------------------------------

    #[test]
    fn records_transfer_progress() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.stream.copy",
                context(),
            )
            .unwrap();

        handle.record_progress(100).unwrap();
        handle.record_progress(200).unwrap();

        assert_eq!(
            handle.snapshot().progress().transferred(),
            300,
        );
    }

    #[test]
    fn progress_counter_saturates_without_failing() {
        let mut progress = OperationProgress::new();

        progress.record(u128::MAX);
        progress.record(1);

        assert_eq!(
            progress.transferred(),
            u128::MAX,
        );

        assert!(progress.is_saturated());
    }

    // ------------------------------------------------------------------------
    // Snapshots and retention
    // ------------------------------------------------------------------------

    #[test]
    fn obtains_registered_snapshot() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        let snapshot = registry
            .get(&handle.id())
            .unwrap();

        assert_eq!(
            snapshot.name(),
            "io.read",
        );

        assert_eq!(
            snapshot.status(),
            OperationStatus::Running,
        );
    }

    #[test]
    fn snapshots_report_elapsed_time() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        assert!(
            handle.snapshot().elapsed()
                <= Duration::from_secs(10)
        );
    }

    #[test]
    fn cannot_remove_running_operation() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        assert!(
            registry
                .remove_terminal(&handle.id())
                .is_err()
        );

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn removes_terminal_operation() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        handle.complete().unwrap();

        let removed = registry
            .remove_terminal(&handle.id())
            .unwrap();

        assert!(removed.is_some());
        assert!(registry.is_empty());

        // Existing handles retain their own shared record.
        assert_eq!(
            handle.status(),
            OperationStatus::Completed,
        );
    }

    #[test]
    fn removes_only_terminal_records() {
        let registry = registry();

        let first = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        let second = registry
            .begin(
                operation_id(2),
                "io.write",
                context(),
            )
            .unwrap();

        first.complete().unwrap();

        assert_eq!(
            registry.remove_terminal_records(),
            1,
        );

        assert_eq!(registry.len(), 1);

        assert_eq!(
            second.status(),
            OperationStatus::Running,
        );
    }

    // ------------------------------------------------------------------------
    // Cancellation
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_uncancelled_operation_context() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        assert!(
            handle.check_cancelled().is_ok()
        );
    }

    // ------------------------------------------------------------------------
    // Independent registries
    // ------------------------------------------------------------------------

    #[test]
    fn registries_have_independent_state() {
        let first = registry();
        let second = registry();

        first
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        assert_eq!(first.len(), 1);
        assert_eq!(second.len(), 0);
    }

    #[test]
    fn cloned_registry_shares_records() {
        let first = registry();
        let second = first.clone();

        let handle = first
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        assert_eq!(second.len(), 1);

        handle.complete().unwrap();

        assert_eq!(
            second.active_count(),
            0,
        );
    }

    // ------------------------------------------------------------------------
    // Debugging
    // ------------------------------------------------------------------------

    #[test]
    fn registry_debug_is_available() {
        let registry = registry();

        let representation = format!("{registry}");

        assert!(
            representation.contains(
                "IoOperationRegistry"
            )
        );
    }

    #[test]
    fn operation_handle_debug_is_available() {
        let registry = registry();

        let handle = registry
            .begin(
                operation_id(1),
                "io.read",
                context(),
            )
            .unwrap();

        let representation = format!("{handle}");

        assert!(
            representation.contains(
                "OperationHandle"
            )
        );
    }
}
