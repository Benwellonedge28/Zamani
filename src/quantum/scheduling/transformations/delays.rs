//! Zamani Quantum Scheduling — Explicit Delay Materialization
//!
//! Path:
//!     src/quantum/scheduling/transformations/delays.rs
//!
//! # Purpose
//!
//! This module identifies semantically meaningful idle intervals in a completed
//! quantum schedule and represents them as validated delay-materialization
//! requests.
//!
//! A delay is an explicit representation of an interval during which a
//! schedulable resource is intentionally idle between two scheduled activities.
//!
//! This module answers:
//!
//!     "Where are explicit delays required by the schedule?"
//!
//! It does NOT:
//!
//! - parse Zamani source;
//! - define quantum operations;
//! - define another quantum IR;
//! - define another `QubitId`;
//! - define another `Duration`;
//! - define another `TimePoint`;
//! - perform routing;
//! - perform scheduling;
//! - execute a QPU;
//! - synthesize pulses;
//! - choose a hardware clock;
//! - perform QEC;
//! - perform noise simulation;
//! - mutate hardware state.
//!
//! The canonical semantic timing types remain owned by:
//!
//!     crate::quantum::ir::timing
//!
//! The canonical qubit identities remain owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! The canonical scheduled-operation representation remains owned by:
//!
//!     crate::quantum::ir::scheduling
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::frontend
//!       |
//!       v
//! quantum::ir
//!       |
//!       v
//! optimization
//!       |
//!       v
//! routing
//!       |
//!       v
//! scheduling
//!       |
//!       v
//! canonical Schedule
//!       |
//!       v
//! transformations::delays
//!       |
//!       v
//! DelayMaterializationPlan
//!       |
//!       v
//! IR transformation / lowering
//!       |
//!       v
//! canonical quantum::ir Delay operations
//!       |
//!       v
//! hardware timing realization
//! ```
//!
//! # Critical ownership rule
//!
//! This file does NOT manufacture a new semantic `Delay` operation type.
//!
//! The canonical quantum IR already owns delay semantics.
//!
//! The canonical timing model also owns:
//!
//!     DelaySpec
//!
//! including fixed delay duration semantics.
//!
//! This module merely determines which intervals from an already constructed
//! schedule should become explicit delay operations.
//!
//! # Why materialization is separate from scheduling
//!
//! A scheduler may internally reason about idle time without emitting explicit
//! delay operations.
//!
//! Explicit materialization is a separate transformation because different
//! downstream consumers may require different representations:
//!
//! ```text
//! analysis-only schedule
//!        |
//!        +---- no explicit Delay operation
//!
//! hardware-oriented schedule
//!        |
//!        +---- explicit Delay operation
//!
//! pulse-oriented lowering
//!        |
//!        +---- target-specific idle instruction
//!
//! optimization
//!        |
//!        +---- preserve idle interval as metadata
//! ```
//!
//! Keeping this transformation separate prevents the scheduler from becoming
//! coupled to one particular hardware representation.
//!
//! # Universal-program principle
//!
//! This module contains no machine-size assumptions.
//!
//! There is deliberately no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum delay count;
//! - maximum schedule depth;
//! - fixed channel count;
//! - fixed topology;
//! - fixed timing resolution;
//! - fixed hardware clock;
//! - vendor-specific delay instruction.
//!
//! The same implementation can process schedules for:
//!
//! - one qubit;
//! - a small QPU;
//! - a large QPU;
//! - a fault-tolerant processor;
//! - a multi-chip processor;
//! - a distributed quantum computer;
//! - a quantum network;
//! - a simulator;
//! - a future quantum architecture.
//!
//! "Infinity" means that this module introduces no artificial finite machine-size
//! ceiling. Actual execution remains bounded by available memory, CPU time,
//! target capabilities, and explicit compilation/resource policies.
//!
//! # Resource-oriented semantics
//!
//! Delays are materialized per selected scheduling resource.
//!
//! This is deliberately more general than "delay every qubit".
//!
//! A schedule can contain resources such as:
//!
//! - logical qubits;
//! - physical qubits;
//! - control channels;
//! - acquisition channels;
//! - frames;
//! - generic target resources.
//!
//! The caller chooses the materialization scope.
//!
//! The default implementation is therefore not hard-coded to a particular
//! hardware technology.
//!
//! # Qubit identity
//!
//! This module does not create a replacement qubit identity.
//!
//! Whenever callers select logical or physical qubit resources, the underlying
//! resource representation is the canonical:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! through `crate::quantum::ir::scheduling::ScheduleResource`.
//!
//! This is important because a scheduler must not accidentally introduce a
//! second qubit namespace.
//!
//! # Time semantics
//!
//! All intervals use the canonical:
//!
//!     crate::quantum::ir::timing::TimePoint
//!     crate::quantum::ir::timing::Duration
//!     crate::quantum::ir::timing::TimeInterval
//!
//! No floating-point timing is used.
//!
//! No host wall-clock time is used.
//!
//! No operating-system clock is used.
//!
//! No backend tick conversion is performed here.
//!
//! # Half-open intervals
//!
//! Canonical scheduling intervals use half-open semantics:
//!
//!     [start, end)
//!
//! Therefore:
//!
//!     [0, 10)
//!     [10, 20)
//!
//! are adjacent rather than overlapping.
//!
//! A gap exists only when:
//!
//!     previous_end < next_start
//!
//! A zero-width interval is never materialized as a delay.
//!
//! # Determinism
//!
//! Given identical scheduled operations and identical configuration:
//!
//!     DelayMaterializer::materialize(...)
//!
//! must produce the same ordering and values.
//!
//! No hash-map iteration order is used to define semantic output order.
//!
//! Materialization is deterministic even when input operations are supplied in
//! a different order because this module canonicalizes operations before
//! calculating gaps.
//!
//! # Complexity
//!
//! Let:
//!
//!     N = number of scheduled operations
//!     R = number of resources referenced by those operations
//!
//! The implementation uses sorting plus a single pass over each resource's
//! operations.
//!
//! Its asymptotic behavior is:
//!
//!     O(N log N)
//!
//! in the general case.
//!
//! Memory usage is proportional to the referenced schedule operations and
//! materialized gaps rather than to:
//!
//!     number_of_all_possible_qubits
//!
//! or:
//!
//!     maximum_schedule_time
//!
//! The implementation therefore does not construct a giant time-grid matrix.
//!
//! # Sparse scalability
//!
//! A very large machine may expose an enormous resource universe while a
//! particular program touches only a small subset.
//!
//! This module works from resources actually present in the schedule.
//!
//! It does not enumerate all possible machine resources.
//!
//! # No unsafe
//!
//! Unsafe Rust is forbidden at the module level.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration with `quantum::ir::scheduling`
//!
//! The transformation consumes:
//!
//!     crate::quantum::ir::scheduling::ScheduledOperation
//!     crate::quantum::ir::scheduling::ScheduleResource
//!
//! It does not redefine either type.
//!
//! The returned `DelayRequest` contains:
//!
//! - the selected resource;
//! - the delay interval;
//! - the canonical `DelaySpec`;
//! - optional predecessor operation;
//! - optional successor operation.
//!
//! The predecessor/successor IDs provide provenance for later lowering.
//!
//! # Integration with canonical Delay semantics
//!
//! The canonical IR owns:
//!
//!     crate::quantum::ir::timing::DelaySpec
//!
//! This module constructs:
//!
//!     DelaySpec::fixed(duration)
//!
//! for every concrete materialized gap.
//!
//! No local delay enum is introduced.
//!
//! # Why no fake OperationId is generated
//!
//! A delay is not necessarily an operation from the original semantic program.
//!
//! Inventing an `OperationId` here would create a false semantic identity and
//! could make later verification believe that the delay existed in the source
//! program.
//!
//! Instead, `DelayRequest` uses optional predecessor/successor operation IDs as
//! provenance anchors.
//!
//! The downstream IR transformation is responsible for assigning a legitimate
//! identity if the canonical IR requires one for a newly materialized operation.
//!
//! # Resource scope
//!
//! The caller may select:
//!
//! - all resources;
//! - logical qubits only;
//! - physical qubits only;
//! - all qubit resources;
//! - non-qubit resources;
//! - a custom predicate.
//!
//! The custom predicate is intentionally represented as a generic closure at
//! execution time rather than as a stored function pointer in the result.
//!
//! This keeps the resulting data deterministic and serializable.
//!
//! # Important semantic distinction
//!
//! This transformation finds idle intervals.
//!
//! It does NOT claim that every idle interval must become a physical hardware
//! delay instruction.
//!
//! Hardware may choose to:
//!
//! - omit explicit idle instructions;
//! - represent them as no-op time advancement;
//! - represent them as pulse silence;
//! - represent them as a backend-native delay;
//! - use dynamical decoupling;
//! - combine adjacent delays;
//! - lower them into another target representation.
//!
//! Those decisions belong downstream.
//!
//! # Boundary behavior
//!
//! By default, this module materializes only gaps between scheduled operations.
//!
//! It does not create:
//!
//! - leading delay from time zero;
//! - trailing delay after the last operation;
//!
//! unless the caller explicitly supplies a finite materialization window.
//!
//! This prevents the transformation from inventing an arbitrary global start or
//! end time.
//!
//! A window can be supplied when a target or enclosing schedule explicitly
//! defines such a temporal domain.
//!
//! # Zero duration
//!
//! A zero-duration gap is ignored.
//!
//! This is not an optimization accident. It follows directly from interval
//! semantics: there is no elapsed idle time to materialize.
//!
//! # Overflow safety
//!
//! Canonical `TimeInterval` construction validates interval bounds.
//!
//! This module does not calculate a delay by subtracting arbitrary unsigned
//! values without first proving ordering.
//!
//! For a valid gap:
//!
//!     previous_end < next_start
//!
//! the duration is obtained through checked canonical timing operations.
//!
//! If canonical timing arithmetic reports an error, the transformation returns
//! a structured `DelayMaterializationError` rather than panicking.
//!
//! # Verification
//!
//! The transformation validates:
//!
//! - no malformed input interval is accepted;
//! - operations are processed deterministically;
//! - resource-local intervals are ordered;
//! - only positive gaps are emitted;
//! - emitted delay intervals do not overlap the neighboring operations;
//! - emitted delay duration equals interval duration;
//! - emitted `DelaySpec` exactly represents that duration.
//!
//! # Thread safety
//!
//! The materializer contains no global state.
//!
//! Configuration is immutable.
//!
//! Input schedules are borrowed immutably.
//!
//! Results own their data.
//!
//! This makes the transformation suitable for ordinary Rust concurrent
//! compilation pipelines.
//!
//! # Serialization
//!
//! `DelayRequest` is deliberately data-oriented.
//!
//! It can be serialized by the scheduling serialization layer once that layer
//! defines its canonical schema.
//!
//! This file does not introduce a second serialization format.
//!
//! # Diagnostics
//!
//! `DelayRequest` retains predecessor and successor operation identities so
//! diagnostics can explain:
//!
//!     "resource R was idle between operation A and operation B"
//!
//! rather than merely reporting an unexplained timestamp.
//!
//! # QEC integration
//!
//! QEC scheduling may use this transformation for explicit idle periods between:
//!
//! - syndrome operations;
//! - ancilla operations;
//! - measurements;
//! - rounds;
//! - feedback.
//!
//! No QEC code is hard-coded here.
//!
//! # ZQN integration
//!
//! ZQN may consume materialized idle intervals to estimate:
//!
//! - decoherence;
//! - idle error;
//! - temporal noise;
//! - crosstalk exposure;
//! - fidelity impact.
//!
//! This module does not depend directly on ZQN.
//!
//! # Hardware integration
//!
//! Hardware adapters may consume the resulting `DelaySpec` and interval and
//! convert them to target-native timing.
//!
//! Conversion may require:
//!
//! - timing resolution;
//! - alignment;
//! - sample period;
//! - channel capabilities;
//! - minimum/maximum delay;
//! - provider-specific instruction support.
//!
//! Those constraints are deliberately downstream.
//!
//! # Routing integration
//!
//! Routing is upstream.
//!
//! It determines which physical resources operations occupy.
//!
//! This transformation simply observes those already-resolved resources.
//!
//! The relationship remains:
//!
//!     routing = WHERE?
//!     scheduling = WHEN?
//!     delay materialization = WHICH IDLE INTERVALS SHOULD BECOME EXPLICIT?
//!
//! # Dynamic-circuit integration
//!
//! This transformation only materializes statically known schedule gaps.
//!
//! Runtime-dependent gaps should be represented through the dynamic scheduling
//! subsystem when their duration depends on runtime events.
//!
//! This module never assumes that a statically observed gap remains fixed under
//! runtime branching.
//!
//! # Distributed integration
//!
//! A communication resource may be selected like any other resource.
//!
//! The resulting interval can represent:
//!
//! - communication idle time;
//! - synchronization gaps;
//! - inter-node waiting;
//! - link availability gaps.
//!
//! The distributed subsystem decides whether those gaps should become explicit
//! communication/synchronization operations.
//!
//! # Integration with scheduling configuration
//!
//! The scheduling configuration already provides a delay-materialization policy
//! boundary.
//!
//! This module intentionally does not read global configuration.
//!
//! The caller should pass an explicit `DelayMaterializationConfig` derived from
//! the scheduler configuration.
//!
//! This keeps the transformation deterministic and testable.
//!
//! # Completion criterion
//!
//! This file is complete when:
//!
//! 1. It can consume canonical scheduled operations.
//! 2. It produces deterministic delay requests.
//! 3. It uses canonical timing types.
//! 4. It uses canonical resource/qubit identities.
//! 5. It introduces no machine-size limits.
//! 6. It introduces no hardware assumptions.
//! 7. It does not invent semantic operation identities.
//! 8. It performs checked validation.
//! 9. It is independent of scheduling algorithms.
//! 10. It is independent of hardware providers.
//! 11. It is independent of QEC implementations.
//! 12. It is independent of ZQN.
//! 13. It is independent of runtime state.
//! 14. It contains no unsafe code.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::scheduling::{
    ScheduleResource,
    ScheduledOperation,
};
use crate::quantum::ir::timing::{
    DelaySpec,
    Duration,
    TimeInterval,
    TimePoint,
    TimingError,
};

// =============================================================================
// Public errors
// =============================================================================

/// Errors produced while materializing explicit schedule delays.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelayMaterializationError {
    /// The requested materialization configuration is internally inconsistent.
    InvalidConfiguration {
        /// Explanation of the configuration error.
        message: String,
    },

    /// Canonical schedule timing data could not be interpreted safely.
    InvalidScheduleTiming {
        /// Operation whose timing caused the error.
        operation: Option<crate::quantum::ir::identity::OperationId>,

        /// Human-readable explanation.
        message: String,
    },

    /// Canonical timing arithmetic failed.
    Timing {
        /// Underlying canonical timing error.
        source: TimingError,
    },

    /// An emitted delay interval was invalid.
    InvalidDelayInterval {
        /// Resource associated with the invalid interval.
        resource: ScheduleResource,

        /// Interval start.
        start: TimePoint,

        /// Interval end.
        end: TimePoint,
    },

    /// Internal validation detected a materialization invariant violation.
    VerificationFailed {
        /// Human-readable explanation.
        message: String,
    },
}

impl fmt::Display for DelayMaterializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { message } => {
                write!(formatter, "invalid delay materialization configuration: {message}")
            }
            Self::InvalidScheduleTiming {
                operation,
                message,
            } => {
                if let Some(operation) = operation {
                    write!(
                        formatter,
                        "invalid schedule timing for operation {operation:?}: {message}"
                    )
                } else {
                    write!(formatter, "invalid schedule timing: {message}")
                }
            }
            Self::Timing { source } => {
                write!(formatter, "timing error while materializing delay: {source}")
            }
            Self::InvalidDelayInterval {
                resource,
                start,
                end,
            } => {
                write!(
                    formatter,
                    "invalid delay interval for resource {resource:?}: [{start:?}, {end:?})"
                )
            }
            Self::VerificationFailed { message } => {
                write!(
                    formatter,
                    "delay materialization verification failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for DelayMaterializationError {}

impl From<TimingError> for DelayMaterializationError {
    fn from(source: TimingError) -> Self {
        Self::Timing { source }
    }
}

/// Result type used by this transformation.
pub type DelayMaterializationResult<T> = Result<T, DelayMaterializationError>;

// =============================================================================
// Scope
// =============================================================================

/// Resource scope for explicit delay materialization.
///
/// The scope determines which resources are eligible for delay requests.
///
/// It does not create or enumerate hardware resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DelayResourceScope {
    /// Materialize delays for every resource referenced by scheduled operations.
    All,

    /// Materialize only logical-qubit resources.
    LogicalQubits,

    /// Materialize only physical-qubit resources.
    PhysicalQubits,

    /// Materialize logical and physical qubit resources.
    Qubits,

    /// Materialize only non-qubit resources.
    NonQubitResources,
}

impl DelayResourceScope {
    /// Returns whether the scope accepts the supplied resource.
    #[must_use]
    pub const fn accepts(self, resource: ScheduleResource) -> bool {
        match self {
            Self::All => true,
            Self::LogicalQubits => {
                matches!(resource, ScheduleResource::LogicalQubit(_))
            }
            Self::PhysicalQubits => {
                matches!(resource, ScheduleResource::PhysicalQubit(_))
            }
            Self::Qubits => resource.is_qubit(),
            Self::NonQubitResources => !resource.is_qubit(),
        }
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration controlling explicit delay materialization.
///
/// This configuration is deliberately independent of machine size.
///
/// It contains no fixed qubit/resource/operation limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DelayMaterializationConfig {
    /// Resource scope to materialize.
    resource_scope: DelayResourceScope,

    /// Whether leading idle intervals should be materialized.
    ///
    /// Leading delays require an explicit temporal origin.
    materialize_leading: bool,

    /// Whether trailing idle intervals should be materialized.
    ///
    /// Trailing delays require an explicit temporal end.
    materialize_trailing: bool,

    /// Whether adjacent gaps should remain separate.
    ///
    /// Keeping them separate preserves predecessor/successor provenance.
    preserve_boundaries: bool,
}

impl Default for DelayMaterializationConfig {
    fn default() -> Self {
        Self {
            resource_scope: DelayResourceScope::All,
            materialize_leading: false,
            materialize_trailing: false,
            preserve_boundaries: true,
        }
    }
}

impl DelayMaterializationConfig {
    /// Creates the default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            resource_scope: DelayResourceScope::All,
            materialize_leading: false,
            materialize_trailing: false,
            preserve_boundaries: true,
        }
    }

    /// Sets the resource scope.
    #[must_use]
    pub const fn with_resource_scope(
        mut self,
        scope: DelayResourceScope,
    ) -> Self {
        self.resource_scope = scope;
        self
    }

    /// Enables or disables leading-delay materialization.
    ///
    /// Leading delays require a finite explicit origin supplied to
    /// [`DelayMaterializer::materialize_with_window`].
    #[must_use]
    pub const fn with_leading_delays(mut self, enabled: bool) -> Self {
        self.materialize_leading = enabled;
        self
    }

    /// Enables or disables trailing-delay materialization.
    ///
    /// Trailing delays require a finite explicit end supplied to
    /// [`DelayMaterializer::materialize_with_window`].
    #[must_use]
    pub const fn with_trailing_delays(mut self, enabled: bool) -> Self {
        self.materialize_trailing = enabled;
        self
    }

    /// Enables or disables provenance-preserving boundaries.
    ///
    /// When enabled, adjacent gaps remain associated with their immediate
    /// predecessor/successor operations.
    #[must_use]
    pub const fn with_preserve_boundaries(
        mut self,
        enabled: bool,
    ) -> Self {
        self.preserve_boundaries = enabled;
        self
    }

    /// Returns the selected resource scope.
    #[must_use]
    pub const fn resource_scope(self) -> DelayResourceScope {
        self.resource_scope
    }

    /// Returns whether leading delays are enabled.
    #[must_use]
    pub const fn materialize_leading(self) -> bool {
        self.materialize_leading
    }

    /// Returns whether trailing delays are enabled.
    #[must_use]
    pub const fn materialize_trailing(self) -> bool {
        self.materialize_trailing
    }

    /// Returns whether provenance boundaries are preserved.
    #[must_use]
    pub const fn preserve_boundaries(self) -> bool {
        self.preserve_boundaries
    }

    /// Validates the configuration independently.
    pub fn validate(self) -> DelayMaterializationResult<()> {
        // The current configuration has no mutually exclusive fields.
        //
        // This explicit method exists so future configuration expansion can
        // remain validation-oriented without changing the materializer API.
        Ok(())
    }
}

// =============================================================================
// Delay request
// =============================================================================

/// A validated request to represent one schedule gap explicitly as a delay.
///
/// This is a transformation artifact, not a replacement for the canonical IR
/// `Delay` operation.
///
/// The downstream IR transformation owns creation of any new semantic
/// operation identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelayRequest {
    resource: ScheduleResource,
    interval: TimeInterval,
    specification: DelaySpec,
    predecessor: Option<crate::quantum::ir::identity::OperationId>,
    successor: Option<crate::quantum::ir::identity::OperationId>,
}

impl DelayRequest {
    /// Creates and validates a delay request.
    pub fn new(
        resource: ScheduleResource,
        interval: TimeInterval,
        predecessor: Option<crate::quantum::ir::identity::OperationId>,
        successor: Option<crate::quantum::ir::identity::OperationId>,
    ) -> DelayMaterializationResult<Self> {
        if interval.is_empty() {
            return Err(DelayMaterializationError::InvalidDelayInterval {
                resource,
                start: interval.start(),
                end: interval.end(),
            });
        }

        let duration = interval.duration();

        if duration == Duration::ZERO {
            return Err(DelayMaterializationError::InvalidDelayInterval {
                resource,
                start: interval.start(),
                end: interval.end(),
            });
        }

        let specification = DelaySpec::fixed(duration);

        Ok(Self {
            resource,
            interval,
            specification,
            predecessor,
            successor,
        })
    }

    /// Returns the resource on which the delay is materialized.
    #[must_use]
    pub const fn resource(&self) -> ScheduleResource {
        self.resource
    }

    /// Returns the delay interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns the delay start.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the delay end.
    #[must_use]
    pub const fn end(&self) -> TimePoint {
        self.interval.end()
    }

    /// Returns the exact delay duration.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.interval.duration()
    }

    /// Returns the canonical IR delay specification.
    #[must_use]
    pub fn specification(&self) -> DelaySpec {
        self.specification.clone()
    }

    /// Returns the immediately preceding scheduled operation, if any.
    #[must_use]
    pub const fn predecessor(
        &self,
    ) -> Option<crate::quantum::ir::identity::OperationId> {
        self.predecessor
    }

    /// Returns the immediately following scheduled operation, if any.
    #[must_use]
    pub const fn successor(
        &self,
    ) -> Option<crate::quantum::ir::identity::OperationId> {
        self.successor
    }

    /// Returns whether the delay is bounded by operations on both sides.
    #[must_use]
    pub const fn is_between_operations(&self) -> bool {
        self.predecessor.is_some() && self.successor.is_some()
    }

    /// Returns whether the delay has no predecessor.
    #[must_use]
    pub const fn is_leading(&self) -> bool {
        self.predecessor.is_none() && self.successor.is_some()
    }

    /// Returns whether the delay has no successor.
    #[must_use]
    pub const fn is_trailing(&self) -> bool {
        self.predecessor.is_some() && self.successor.is_none()
    }

    /// Verifies the internal delay invariant.
    pub fn verify(&self) -> DelayMaterializationResult<()> {
        if self.interval.is_empty() {
            return Err(DelayMaterializationError::VerificationFailed {
                message: "materialized delay interval is empty".to_owned(),
            });
        }

        let interval_duration = self.interval.duration();

        match self.specification {
            DelaySpec::Fixed(duration) if duration == interval_duration => {}
            DelaySpec::Fixed(_) => {
                return Err(DelayMaterializationError::VerificationFailed {
                    message:
                        "DelaySpec duration does not equal the materialized interval duration"
                            .to_owned(),
                });
            }
            _ => {
                return Err(DelayMaterializationError::VerificationFailed {
                    message:
                        "materialized schedule gap did not produce a fixed concrete DelaySpec"
                            .to_owned(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Materialization result
// =============================================================================

/// Result of explicit delay discovery/materialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelayMaterializationPlan {
    requests: Vec<DelayRequest>,
}

impl DelayMaterializationPlan {
    /// Creates a plan from already validated requests.
    ///
    /// Requests are sorted into deterministic canonical order.
    pub fn new(
        mut requests: Vec<DelayRequest>,
    ) -> DelayMaterializationResult<Self> {
        requests.sort_by(|left, right| {
            left.start()
                .cmp(&right.start())
                .then_with(|| left.end().cmp(&right.end()))
                .then_with(|| left.resource().cmp(&right.resource()))
                .then_with(|| left.predecessor().cmp(&right.predecessor()))
                .then_with(|| left.successor().cmp(&right.successor()))
        });

        let plan = Self { requests };

        plan.verify()?;

        Ok(plan)
    }

    /// Returns all delay requests in deterministic order.
    #[must_use]
    pub fn requests(&self) -> &[DelayRequest] {
        &self.requests
    }

    /// Returns the number of delay requests.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requests.len()
    }

    /// Returns whether the plan contains no delays.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    /// Returns an iterator over delay requests.
    pub fn iter(&self) -> std::slice::Iter<'_, DelayRequest> {
        self.requests.iter()
    }

    /// Consumes the plan and returns its requests.
    #[must_use]
    pub fn into_requests(self) -> Vec<DelayRequest> {
        self.requests
    }

    /// Returns the total duration represented by all materialized requests.
    ///
    /// This is a diagnostic aggregate only. If several resources are idle at
    /// the same time, their durations are intentionally counted independently.
    pub fn total_resource_idle_time(
        &self,
    ) -> DelayMaterializationResult<Duration> {
        let mut total = Duration::ZERO;

        for request in &self.requests {
            total = total
                .checked_add(request.duration())
                .map_err(DelayMaterializationError::from)?;
        }

        Ok(total)
    }

    /// Verifies every request and pairwise resource-local non-overlap.
    pub fn verify(&self) -> DelayMaterializationResult<()> {
        for request in &self.requests {
            request.verify()?;
        }

        // Requests for different resources cannot conflict with one another
        // merely because their temporal intervals overlap.
        //
        // For the same resource, materialized delays must not overlap.
        let mut previous_by_resource: BTreeMap<
            ScheduleResource,
            &DelayRequest,
        > = BTreeMap::new();

        for request in &self.requests {
            if let Some(previous) =
                previous_by_resource.get(&request.resource())
            {
                if previous.interval().overlaps(request.interval()) {
                    return Err(
                        DelayMaterializationError::VerificationFailed {
                            message: format!(
                                "overlapping materialized delays for resource {:?}",
                                request.resource()
                            ),
                        },
                    );
                }
            }

            previous_by_resource.insert(request.resource(), request);
        }

        Ok(())
    }
}

impl<'a> IntoIterator for &'a DelayMaterializationPlan {
    type Item = &'a DelayRequest;
    type IntoIter = std::slice::Iter<'a, DelayRequest>;

    fn into_iter(self) -> Self::IntoIter {
        self.requests.iter()
    }
}

// =============================================================================
// Materializer
// =============================================================================

/// Stateless explicit-delay materializer.
///
/// All configuration is supplied explicitly.
///
/// No global scheduler state exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DelayMaterializer {
    configuration: DelayMaterializationConfig,
}

impl DelayMaterializer {
    /// Creates a materializer from explicit configuration.
    pub fn new(
        configuration: DelayMaterializationConfig,
    ) -> DelayMaterializationResult<Self> {
        configuration.validate()?;

        Ok(Self { configuration })
    }

    /// Returns the materializer configuration.
    #[must_use]
    pub const fn configuration(
        &self,
    ) -> DelayMaterializationConfig {
        self.configuration
    }

    /// Materializes internal gaps between scheduled operations.
    ///
    /// This is the normal production path.
    ///
    /// No arbitrary global start or end is invented.
    pub fn materialize<I>(
        &self,
        operations: I,
    ) -> DelayMaterializationResult<DelayMaterializationPlan>
    where
        I: IntoIterator<Item = ScheduledOperation>,
    {
        self.materialize_with_window(operations, None, None)
    }

    /// Materializes delays with an optional explicit temporal window.
    ///
    /// `window_start` is required when leading delays are enabled.
    ///
    /// `window_end` is required when trailing delays are enabled.
    ///
    /// The window is not inferred from zero or from a hardware-specific clock.
    pub fn materialize_with_window<I>(
        &self,
        operations: I,
        window_start: Option<TimePoint>,
        window_end: Option<TimePoint>,
    ) -> DelayMaterializationResult<DelayMaterializationPlan>
    where
        I: IntoIterator<Item = ScheduledOperation>,
    {
        self.validate_window(window_start, window_end)?;

        let selected = self.collect_selected_operations(operations);

        let mut by_resource: BTreeMap<
            ScheduleResource,
            Vec<ScheduledOperation>,
        > = BTreeMap::new();

        for operation in selected {
            for resource in operation.resources().iter().copied() {
                if self
                    .configuration
                    .resource_scope()
                    .accepts(resource)
                {
                    by_resource
                        .entry(resource)
                        .or_default()
                        .push(operation.clone());
                }
            }
        }

        let mut requests = Vec::new();

        for (resource, mut resource_operations) in by_resource {
            Self::canonicalize_operations(&mut resource_operations);

            self.materialize_resource_gaps(
                resource,
                &resource_operations,
                window_start,
                window_end,
                &mut requests,
            )?;
        }

        DelayMaterializationPlan::new(requests)
    }

    fn validate_window(
        &self,
        window_start: Option<TimePoint>,
        window_end: Option<TimePoint>,
    ) -> DelayMaterializationResult<()> {
        if self.configuration.materialize_leading() && window_start.is_none() {
            return Err(DelayMaterializationError::InvalidConfiguration {
                message:
                    "leading delay materialization requires an explicit window start"
                        .to_owned(),
            });
        }

        if self.configuration.materialize_trailing() && window_end.is_none() {
            return Err(DelayMaterializationError::InvalidConfiguration {
                message:
                    "trailing delay materialization requires an explicit window end"
                        .to_owned(),
            });
        }

        if let (Some(start), Some(end)) = (window_start, window_end) {
            if start > end {
                return Err(DelayMaterializationError::InvalidConfiguration {
                    message:
                        "materialization window start must not exceed its end"
                            .to_owned(),
                });
            }
        }

        Ok(())
    }

    fn collect_selected_operations<I>(
        &self,
        operations: I,
    ) -> Vec<ScheduledOperation>
    where
        I: IntoIterator<Item = ScheduledOperation>,
    {
        operations.into_iter().collect()
    }

    fn canonicalize_operations(
        operations: &mut [ScheduledOperation],
    ) {
        operations.sort_by(|left, right| {
            left.start()
                .cmp(&right.start())
                .then_with(|| left.end().cmp(&right.end()))
                .then_with(|| left.operation_id().cmp(&right.operation_id()))
        });
    }

    fn materialize_resource_gaps(
        &self,
        resource: ScheduleResource,
        operations: &[ScheduledOperation],
        window_start: Option<TimePoint>,
        window_end: Option<TimePoint>,
        requests: &mut Vec<DelayRequest>,
    ) -> DelayMaterializationResult<()> {
        if operations.is_empty() {
            // There is no operation from which to infer a resource-local idle
            // interval. Even if a global window exists, materializing an idle
            // period for a completely unused resource would invent resource
            // activity that was not present in the schedule.
            return Ok(());
        }

        if self.configuration.materialize_leading() {
            let start = window_start.ok_or_else(|| {
                DelayMaterializationError::InvalidConfiguration {
                    message:
                        "leading materialization enabled without a window start"
                            .to_owned(),
                }
            })?;

            let first = &operations[0];

            if start < first.start() {
                let interval = Self::make_positive_interval(
                    resource,
                    start,
                    first.start(),
                    None,
                    Some(first.operation_id()),
                )?;

                requests.push(DelayRequest::new(
                    resource,
                    interval,
                    None,
                    Some(first.operation_id()),
                )?);
            }
        }

        for pair in operations.windows(2) {
            let previous = &pair[0];
            let next = &pair[1];

            if previous.end() > next.start() {
                return Err(
                    DelayMaterializationError::InvalidScheduleTiming {
                        operation: Some(next.operation_id()),
                        message: format!(
                            "resource {:?} has overlapping operations: {:?} ends after {:?} starts",
                            resource,
                            previous.operation_id(),
                            next.operation_id()
                        ),
                    },
                );
            }

            if previous.end() < next.start() {
                let interval = Self::make_positive_interval(
                    resource,
                    previous.end(),
                    next.start(),
                    Some(previous.operation_id()),
                    Some(next.operation_id()),
                )?;

                requests.push(DelayRequest::new(
                    resource,
                    interval,
                    Some(previous.operation_id()),
                    Some(next.operation_id()),
                )?);
            }
        }

        if self.configuration.materialize_trailing() {
            let end = window_end.ok_or_else(|| {
                DelayMaterializationError::InvalidConfiguration {
                    message:
                        "trailing materialization enabled without a window end"
                            .to_owned(),
                }
            })?;

            let last = operations.last().ok_or_else(|| {
                DelayMaterializationError::InvalidScheduleTiming {
                    operation: None,
                    message:
                        "resource operation collection unexpectedly became empty"
                            .to_owned(),
                }
            })?;

            if last.end() < end {
                let interval = Self::make_positive_interval(
                    resource,
                    last.end(),
                    end,
                    Some(last.operation_id()),
                    None,
                )?;

                requests.push(DelayRequest::new(
                    resource,
                    interval,
                    Some(last.operation_id()),
                    None,
                )?);
            }
        }

        Ok(())
    }

    fn make_positive_interval(
        resource: ScheduleResource,
        start: TimePoint,
        end: TimePoint,
        _predecessor: Option<crate::quantum::ir::identity::OperationId>,
        _successor: Option<crate::quantum::ir::identity::OperationId>,
    ) -> DelayMaterializationResult<TimeInterval> {
        if start >= end {
            return Err(DelayMaterializationError::InvalidDelayInterval {
                resource,
                start,
                end,
            });
        }

        TimeInterval::new(start, end).map_err(|_| {
            DelayMaterializationError::InvalidDelayInterval {
                resource,
                start,
                end,
            }
        })
    }
}

// =============================================================================
// Convenience API
// =============================================================================

/// Materializes internal schedule gaps using the default configuration.
///
/// This is equivalent to:
///
/// ```text
/// DelayMaterializer::new(DelayMaterializationConfig::new())
///     .materialize(operations)
/// ```
pub fn materialize_delays<I>(
    operations: I,
) -> DelayMaterializationResult<DelayMaterializationPlan>
where
    I: IntoIterator<Item = ScheduledOperation>,
{
    DelayMaterializer::new(DelayMaterializationConfig::new())?
        .materialize(operations)
}

/// Materializes schedule gaps using explicit configuration.
pub fn materialize_delays_with_config<I>(
    operations: I,
    configuration: DelayMaterializationConfig,
) -> DelayMaterializationResult<DelayMaterializationPlan>
where
    I: IntoIterator<Item = ScheduledOperation>,
{
    DelayMaterializer::new(configuration)?.materialize(operations)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::identity::OperationId;
    use crate::quantum::ir::scheduling::ScheduleResource;
    use crate::quantum::ir::timing::{
        Duration,
        TimeInterval,
        TimePoint,
    };

    fn operation(
        id: u64,
        start: u128,
        end: u128,
        resource: ScheduleResource,
    ) -> ScheduledOperation {
        ScheduledOperation::new(
            OperationId::new(id),
            TimeInterval::new(
                TimePoint::new(start),
                TimePoint::new(end),
            )
            .expect("test interval must be valid"),
            [resource],
        )
        .expect("test scheduled operation must be valid")
    }

    fn physical_qubit(
        id: u64,
    ) -> ScheduleResource {
        ScheduleResource::PhysicalQubit(
            crate::quantum::ir::qubit::PhysicalQubitId::new(id),
        )
    }

    fn logical_qubit(
        id: u64,
    ) -> ScheduleResource {
        ScheduleResource::LogicalQubit(
            crate::quantum::ir::qubit::QubitId::new(id),
        )
    }

    #[test]
    fn finds_internal_gap() {
        let resource = physical_qubit(0);

        let first = operation(1, 0, 10, resource);
        let second = operation(2, 20, 30, resource);

        let plan = materialize_delays([first, second])
            .expect("materialization must succeed");

        assert_eq!(plan.len(), 1);

        let delay = &plan.requests()[0];

        assert_eq!(delay.resource(), resource);
        assert_eq!(delay.start(), TimePoint::new(10));
        assert_eq!(delay.end(), TimePoint::new(20));
        assert_eq!(
            delay.duration(),
            Duration::from_attoseconds(10)
        );
        assert_eq!(
            delay.predecessor(),
            Some(OperationId::new(1))
        );
        assert_eq!(
            delay.successor(),
            Some(OperationId::new(2))
        );
    }

    #[test]
    fn adjacent_operations_do_not_create_delay() {
        let resource = physical_qubit(0);

        let first = operation(1, 0, 10, resource);
        let second = operation(2, 10, 20, resource);

        let plan = materialize_delays([first, second])
            .expect("materialization must succeed");

        assert!(plan.is_empty());
    }

    #[test]
    fn zero_duration_operations_do_not_create_zero_delays() {
        let resource = physical_qubit(0);

        let first = ScheduledOperation::at(
            OperationId::new(1),
            TimePoint::new(10),
        )
        .expect("zero-duration operation must be valid");

        let second = operation(2, 20, 30, resource);

        let plan = materialize_delays([first, second])
            .expect("materialization must succeed");

        // The zero-duration operation has no resource reservation, so it does
        // not create an artificial resource-local delay boundary.
        assert!(plan.is_empty());
    }

    #[test]
    fn leading_delay_requires_explicit_window() {
        let configuration =
            DelayMaterializationConfig::new()
                .with_leading_delays(true);

        let materializer =
            DelayMaterializer::new(configuration)
                .expect("configuration must be valid");

        let result = materializer.materialize([operation(
            1,
            10,
            20,
            physical_qubit(0),
        )]);

        assert!(matches!(
            result,
            Err(DelayMaterializationError::InvalidConfiguration { .. })
        ));
    }

    #[test]
    fn leading_delay_uses_explicit_window() {
        let configuration =
            DelayMaterializationConfig::new()
                .with_leading_delays(true);

        let materializer =
            DelayMaterializer::new(configuration)
                .expect("configuration must be valid");

        let plan = materializer
            .materialize_with_window(
                [operation(
                    1,
                    10,
                    20,
                    physical_qubit(0),
                )],
                Some(TimePoint::new(0)),
                None,
            )
            .expect("materialization must succeed");

        assert_eq!(plan.len(), 1);

        let delay = &plan.requests()[0];

        assert!(delay.is_leading());
        assert_eq!(delay.start(), TimePoint::new(0));
        assert_eq!(delay.end(), TimePoint::new(10));
        assert_eq!(
            delay.successor(),
            Some(OperationId::new(1))
        );
    }

    #[test]
    fn trailing_delay_uses_explicit_window() {
        let configuration =
            DelayMaterializationConfig::new()
                .with_trailing_delays(true);

        let materializer =
            DelayMaterializer::new(configuration)
                .expect("configuration must be valid");

        let plan = materializer
            .materialize_with_window(
                [operation(
                    1,
                    10,
                    20,
                    physical_qubit(0),
                )],
                None,
                Some(TimePoint::new(30)),
            )
            .expect("materialization must succeed");

        assert_eq!(plan.len(), 1);

        let delay = &plan.requests()[0];

        assert!(delay.is_trailing());
        assert_eq!(delay.start(), TimePoint::new(20));
        assert_eq!(delay.end(), TimePoint::new(30));
        assert_eq!(
            delay.predecessor(),
            Some(OperationId::new(1))
        );
    }

    #[test]
    fn resource_scope_can_select_physical_qubits() {
        let configuration =
            DelayMaterializationConfig::new()
                .with_resource_scope(
                    DelayResourceScope::PhysicalQubits,
                );

        let materializer =
            DelayMaterializer::new(configuration)
                .expect("configuration must be valid");

        let physical = physical_qubit(0);
        let logical = logical_qubit(0);

        let operations = [
            operation(1, 0, 10, physical),
            operation(2, 20, 30, physical),
            operation(3, 0, 10, logical),
            operation(4, 20, 30, logical),
        ];

        let plan = materializer
            .materialize(operations)
            .expect("materialization must succeed");

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan.requests()[0].resource(),
            physical
        );
    }

    #[test]
    fn resource_scope_can_select_logical_qubits() {
        let configuration =
            DelayMaterializationConfig::new()
                .with_resource_scope(
                    DelayResourceScope::LogicalQubits,
                );

        let materializer =
            DelayMaterializer::new(configuration)
                .expect("configuration must be valid");

        let physical = physical_qubit(0);
        let logical = logical_qubit(0);

        let operations = [
            operation(1, 0, 10, physical),
            operation(2, 20, 30, physical),
            operation(3, 0, 10, logical),
            operation(4, 20, 30, logical),
        ];

        let plan = materializer
            .materialize(operations)
            .expect("materialization must succeed");

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan.requests()[0].resource(),
            logical
        );
    }

    #[test]
    fn operations_are_canonicalized_independent_of_input_order() {
        let resource = physical_qubit(0);

        let first = operation(1, 0, 10, resource);
        let second = operation(2, 20, 30, resource);

        let forward = materialize_delays([
            first.clone(),
            second.clone(),
        ])
        .expect("forward materialization must succeed");

        let reverse = materialize_delays([
            second,
            first,
        ])
        .expect("reverse materialization must succeed");

        assert_eq!(forward, reverse);
    }

    #[test]
    fn multiple_resources_are_materialized_independently() {
        let q0 = physical_qubit(0);
        let q1 = physical_qubit(1);

        let operations = [
            operation(1, 0, 10, q0),
            operation(2, 20, 30, q0),
            operation(3, 0, 15, q1),
            operation(4, 25, 40, q1),
        ];

        let plan = materialize_delays(operations)
            .expect("materialization must succeed");

        assert_eq!(plan.len(), 2);

        assert_eq!(plan.requests()[0].resource(), q0);
        assert_eq!(plan.requests()[0].start(), TimePoint::new(10));
        assert_eq!(plan.requests()[0].end(), TimePoint::new(20));

        assert_eq!(plan.requests()[1].resource(), q1);
        assert_eq!(plan.requests()[1].start(), TimePoint::new(15));
        assert_eq!(plan.requests()[1].end(), TimePoint::new(25));
    }

    #[test]
    fn overlapping_operations_are_rejected() {
        let resource = physical_qubit(0);

        let first = operation(1, 0, 20, resource);
        let second = operation(2, 10, 30, resource);

        let result = materialize_delays([first, second]);

        assert!(matches!(
            result,
            Err(DelayMaterializationError::InvalidScheduleTiming { .. })
        ));
    }

    #[test]
    fn plan_verification_rejects_overlapping_delays() {
        let resource = physical_qubit(0);

        let first = DelayRequest::new(
            resource,
            TimeInterval::new(
                TimePoint::new(0),
                TimePoint::new(10),
            )
            .expect("valid interval"),
            Some(OperationId::new(1)),
            Some(OperationId::new(2)),
        )
        .expect("valid delay");

        let second = DelayRequest::new(
            resource,
            TimeInterval::new(
                TimePoint::new(5),
                TimePoint::new(15),
            )
            .expect("valid interval"),
            Some(OperationId::new(2)),
            Some(OperationId::new(3)),
        )
        .expect("valid delay");

        let result = DelayMaterializationPlan::new(vec![
            first,
            second,
        ]);

        assert!(matches!(
            result,
            Err(DelayMaterializationError::VerificationFailed { .. })
        ));
    }

    #[test]
    fn delay_spec_matches_interval_duration() {
        let resource = physical_qubit(0);

        let request = DelayRequest::new(
            resource,
            TimeInterval::new(
                TimePoint::new(100),
                TimePoint::new(250),
            )
            .expect("valid interval"),
            Some(OperationId::new(1)),
            Some(OperationId::new(2)),
        )
        .expect("valid request");

        assert_eq!(
            request.duration(),
            Duration::from_attoseconds(150)
        );

        request
            .verify()
            .expect("delay request must verify");
    }

    #[test]
    fn default_materialization_does_not_invent_leading_or_trailing_time() {
        let resource = physical_qubit(0);

        let operations = [
            operation(1, 100, 110, resource),
        ];

        let plan = materialize_delays(operations)
            .expect("materialization must succeed");

        assert!(plan.is_empty());
    }

    #[test]
    fn empty_input_is_valid_and_empty() {
        let plan = materialize_delays(
            std::iter::empty::<ScheduledOperation>(),
        )
        .expect("empty schedule must be valid");

        assert!(plan.is_empty());
    }

    #[test]
    fn delay_total_resource_idle_time_is_exact() {
        let resource = physical_qubit(0);

        let operations = [
            operation(1, 0, 10, resource),
            operation(2, 20, 30, resource),
            operation(3, 40, 60, resource),
        ];

        let plan = materialize_delays(operations)
            .expect("materialization must succeed");

        assert_eq!(
            plan.total_resource_idle_time()
                .expect("total must be representable"),
            Duration::from_attoseconds(20)
        );
    }
}