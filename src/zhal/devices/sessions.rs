
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Device Sessions
//!
//! Path: src/zhal/devices/sessions.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the generic lifecycle-management boundary for
//! provider-managed hardware device sessions.
//!
//! This module validates, adopts, observes, closes and transfers
//! ownership of existing HardwareSession implementations.
//!
//! It supports sessions associated with:
//!
//! - Classical processors.
//! - Memory and storage devices.
//! - GPUs and other accelerators.
//! - Quantum processing units.
//! - Quantum simulators.
//! - FPGA and ASIC devices.
//! - Embedded hardware.
//! - Sensors and actuators.
//! - Remote hardware.
//! - Distributed hardware.
//! - Hybrid computing systems.
//! - Neuromorphic hardware.
//! - Photonic and analog computing.
//! - Future computational technologies.
//!
//! Hardware categories and provider implementations remain
//! open-ended.
//!
//! # Architectural ownership
//!
//! THIS MODULE OWNS:
//!
//! - HardwareSessionHandle.
//! - HardwareSessionState.
//! - Session adoption and validation.
//! - Session lifecycle coordination.
//! - Idempotent close coordination.
//! - Session ownership transfer.
//! - Session lifecycle diagnostics.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - HardwareSession trait.
//! - HardwareProvider trait.
//! - Provider identity.
//! - Provider registration.
//! - Provider initialization or shutdown.
//! - Authentication.
//! - Authorization policy.
//! - Device discovery.
//! - Device inventory.
//! - Capability evaluation.
//! - Resource allocation.
//! - Resource reservations.
//! - Resource accounting.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Domain-specific execution.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//! - Hybrid orchestration.
//! - Runtime cancellation infrastructure.
//!
//! Those responsibilities remain with their existing
//! repository authorities.
//!
//! # Canonical repository authorities
//!
//! Hardware session contract:
//!     crate::zhal::providers::contract::HardwareSession
//!
//! Hardware provider contract:
//!     crate::zhal::providers::contract::HardwareProvider
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Cancellation:
//!     crate::stdlib::context
//!
//! Resource allocation:
//!     Canonical ZUR resource-management authority.
//!
//! Quantum-specific sessions:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Generic device I/O:
//!     crate::stdlib::io::devices
//!
//! # Session ownership
//!
//! A HardwareSessionHandle exclusively owns one provider-managed
//! session.
//!
//! The handle does not clone a physical session or manufacture
//! additional access rights.
//!
//! A session must be transferred explicitly when another subsystem
//! becomes responsible for its lifecycle.
//!
//! # Session closure
//!
//! Successful closure must be observable through the provider's
//! HardwareSession::is_closed implementation.
//!
//! Repeated closure through this handle is harmless.
//!
//! A successful close operation that does not result in a closed
//! session is treated as a provider contract violation.
//!
//! Failed closure is not silently converted into success.
//!
//! The provider remains responsible for its physical cleanup
//! guarantees and failure reporting.
//!
//! # Cancellation
//!
//! This module does not implement another cancellation mechanism.
//!
//! The existing ZhalContext is passed directly to the provider.
//!
//! Session closure is a cleanup operation. Consequently, this
//! module does not reject closure merely because the operation
//! context is already cancelled or past its deadline.
//!
//! The provider decides which essential cleanup operations must
//! continue despite cancellation.
//!
//! # Resource ownership
//!
//! Opening or holding a session does not automatically reserve
//! computational resources.
//!
//! A session does not imply ownership of:
//!
//! - CPU capacity.
//! - Memory capacity.
//! - GPU capacity.
//! - Qubits.
//! - FPGA logic resources.
//! - Storage capacity.
//! - Network bandwidth.
//! - Distributed execution capacity.
//!
//! Resource reservations and allocation remain with ZUR and
//! applicable specialized provider contracts.
//!
//! # Concurrency
//!
//! HardwareSession is Send, not necessarily Sync.
//!
//! This module therefore does not promise concurrent access to
//! an individual session.
//!
//! A handle requires exclusive mutable access for closure.
//!
//! Concurrent operations require an explicit concurrency contract
//! from the specialized provider interface.
//!
//! # POCO-REAF
//!
//! No arbitrary maximum is imposed on:
//!
//! - Number of providers.
//! - Number of devices.
//! - Number of sessions.
//! - Number of processors.
//! - Number of accelerators.
//! - Number of qubits.
//! - Number of hardware categories.
//! - Session identifier lengths.
//! - Device identifier lengths.
//!
//! The module does not maintain a global session collection.
//!
//! Each handle owns only its own session. Aggregate ownership,
//! quotas and operational budgets belong to their respective
//! runtime or lifecycle authorities.
//!
//! Physical execution remains constrained by available resources.
//! Literal infinite physical resources are not promised.
//!
//! # Grammar boundary
//!
//! Relevant grammar and specifications:
//!
//! - grammar/hardware/devices.g4
//! - grammar/hardware/hardware.g4
//! - grammar/effects/hardware.g4
//! - grammar/effects/security.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/semantic-model.md
//! - grammar/validation/semantic-boundaries.md
//!
//! Grammar expresses logical device intent, requirements,
//! capabilities, effects and execution context.
//!
//! This module operates after semantic validation and provider
//! access authorization.
//!
//! It does not parse source code, perform semantic analysis,
//! select physical hardware or allocate resources.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.

use core::fmt;

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareSessionIdentity,
};

use crate::zhal::providers::contract::{
    HardwareSession,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.devices.sessions";

// ============================================================================
// SESSION STATE
// ============================================================================

/// Observable lifecycle state of an adopted hardware session.
///
/// The provider's `HardwareSession::is_closed()` method remains
/// the authoritative source of the underlying physical session
/// state.
///
/// This enum deliberately does not invent intermediate provider
/// states that the existing HardwareSession contract cannot
/// establish.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HardwareSessionState {
    /// The provider reports that the session is not closed.
    Open,

    /// The provider reports that the session is closed.
    Closed,
}

impl HardwareSessionState {
    /// Returns a stable textual state identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }

    /// Returns whether the state represents an open session.
    #[must_use]
    pub const fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }

    /// Returns whether the state represents a closed session.
    #[must_use]
    pub const fn is_closed(self) -> bool {
        matches!(self, Self::Closed)
    }
}

impl fmt::Display for HardwareSessionState {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn session_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

fn invalid_session(
    message: impl Into<String>,
) -> ZhalError {
    session_error(
        ZhalErrorKind::InvalidInput,
        message,
    )
}

fn contract_violation(
    message: impl Into<String>,
) -> ZhalError {
    session_error(
        ZhalErrorKind::ContractViolation,
        message,
    )
}

// ============================================================================
// SESSION HANDLE
// ============================================================================

/// Exclusively owned handle to a provider-managed hardware session.
///
/// The handle provides a common lifecycle boundary around the
/// existing provider-owned `HardwareSession` implementation.
///
/// It does not replace the provider session or implement a
/// universal hardware operation interface.
///
/// # Ownership
///
/// A handle owns exactly one provider-managed session.
///
/// It cannot be cloned because cloning a physical session handle
/// would create ambiguous lifecycle ownership.
///
/// # Lifecycle
///
/// The handle observes the provider's closed-state report.
///
/// Calling `close()` repeatedly after successful closure returns
/// success without invoking the provider again.
///
/// A successful provider close that leaves the session open
/// produces a contract violation.
///
/// # Thread safety
///
/// The handle is Send when its provider session is Send.
///
/// It is not promised to be Sync.
///
/// # Resource semantics
///
/// The handle does not represent a resource reservation,
/// resource allocation or scheduling decision.
pub struct HardwareSessionHandle {
    session: Box<dyn HardwareSession>,
    identity: HardwareSessionIdentity,
    device: HardwareDeviceIdentity,
}

impl HardwareSessionHandle {
    /// Adopts and validates an existing provider-managed session.
    ///
    /// The session must:
    ///
    /// - Have a provider-scoped session identity.
    /// - Refer to a device owned by the same provider.
    /// - Be associated with the exact device returned by its
    ///   `device_identity()` method.
    /// - Report that it is not already closed.
    ///
    /// The provider must already have completed the applicable
    /// authentication, authorization and physical access checks.
    ///
    /// This method does not grant additional permissions.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - InvalidInput for an already-closed session.
    /// - ContractViolation for inconsistent provider/device identity.
    ///
    /// The supplied session is dropped if adoption fails.
    ///
    /// Dropping a session does not guarantee asynchronous physical
    /// cleanup. Providers must define their own safe drop behavior.
    pub fn new(
        session: Box<dyn HardwareSession>,
    ) -> ZhalResult<Self> {
        let identity = session.identity().clone();
        let device = session.device_identity().clone();

        if identity.provider_id() != device.provider_id() {
            return Err(
                contract_violation(
                    "hardware session and device identities belong to different providers",
                )
                .with_session(identity),
            );
        }

        if session.is_closed() {
            return Err(
                invalid_session(
                    "a closed hardware session cannot be adopted as an open session",
                )
                .with_session(identity),
            );
        }

        Ok(Self {
            session,
            identity,
            device,
        })
    }

    /// Returns the immutable provider-scoped session identity.
    #[must_use]
    pub fn identity(&self) -> &HardwareSessionIdentity {
        &self.identity
    }

    /// Returns the immutable associated hardware-device identity.
    #[must_use]
    pub fn device_identity(&self) -> &HardwareDeviceIdentity {
        &self.device
    }

    /// Returns the provider-reported session lifecycle state.
    ///
    /// The provider remains the authority for the actual state.
    #[must_use]
    pub fn state(&self) -> HardwareSessionState {
        if self.session.is_closed() {
            HardwareSessionState::Closed
        } else {
            HardwareSessionState::Open
        }
    }

    /// Returns whether the provider reports that the session is open.
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.state().is_open()
    }

    /// Returns whether the provider reports that the session is closed.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.state().is_closed()
    }

    /// Returns an immutable reference to the provider-managed session.
    ///
    /// Specialized domain adapters can inspect the generic session
    /// identity and lifecycle without taking ownership.
    ///
    /// Domain-specific execution must still use its own authoritative
    /// provider interface.
    #[must_use]
    pub fn session(&self) -> &dyn HardwareSession {
        self.session.as_ref()
    }

    /// Checks that the session remains open.
    ///
    /// This is an observation, not a reservation or guarantee that
    /// the session will remain open after this method returns.
    ///
    /// # Errors
    ///
    /// Returns DeviceUnavailable if the provider reports closure.
    pub fn ensure_open(&self) -> ZhalResult<()> {
        if self.session.is_closed() {
            return Err(
                session_error(
                    ZhalErrorKind::DeviceUnavailable,
                    "the provider-managed hardware session is closed",
                )
                .with_session(self.identity.clone())
                .with_device(self.device.clone()),
            );
        }

        Ok(())
    }

    /// Closes the provider-managed session.
    ///
    /// The provider performs the actual cleanup.
    ///
    /// This method:
    ///
    /// 1. Checks whether the session is already closed.
    /// 2. Delegates closure to the existing provider session.
    /// 3. Preserves provider errors.
    /// 4. Verifies that successful closure is observable.
    ///
    /// Repeated closure after successful closure is harmless.
    ///
    /// The supplied context is passed directly to the provider.
    ///
    /// This method deliberately does not reject a cancelled or
    /// expired context before calling the provider. Essential
    /// cleanup may need to continue after cancellation.
    ///
    /// The provider's existing close contract determines which
    /// cleanup actions can safely proceed.
    ///
    /// # Errors
    ///
    /// Returns a provider error if closure fails.
    ///
    /// Returns ContractViolation if the provider reports success
    /// without reporting that the session has closed.
    pub async fn close(
        &mut self,
        context: &ZhalContext,
    ) -> ZhalResult<()> {
        if self.session.is_closed() {
            return Ok(());
        }

        self.session
            .close(context)
            .await
            .map_err(|error| {
                error
                    .with_session(self.identity.clone())
                    .with_device(self.device.clone())
            })?;

        if !self.session.is_closed() {
            return Err(
                contract_violation(
                    "provider reported successful session closure but the session remains open",
                )
                .with_session(self.identity.clone())
                .with_device(self.device.clone()),
            );
        }

        Ok(())
    }

    /// Consumes the handle and transfers ownership of the original
    /// provider-managed session to the caller.
    ///
    /// After this operation, the caller is responsible for preserving
    /// the provider session lifecycle and invoking its close method.
    ///
    /// This operation does not close the session.
    #[must_use]
    pub fn into_inner(self) -> Box<dyn HardwareSession> {
        self.session
    }

    /// Returns the session identity and associated device identity.
    ///
    /// The original provider-managed session remains owned by the
    /// returned handle.
    #[must_use]
    pub fn identities(
        &self,
    ) -> (
        &HardwareSessionIdentity,
        &HardwareDeviceIdentity,
    ) {
        (
            &self.identity,
            &self.device,
        )
    }
}

impl fmt::Debug for HardwareSessionHandle {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HardwareSessionHandle")
            .field("identity", &self.identity)
            .field("device", &self.device)
            .field("state", &self.state())
            .finish()
    }
}

// ============================================================================
// SESSION VALIDATION
// ============================================================================

/// Validates an existing provider-managed hardware session.
///
/// This function is useful when a provider or adapter needs to
/// validate a session before transferring it to another subsystem.
///
/// It verifies:
///
/// - Session identity is associated with the device provider.
/// - The session is not already closed.
///
/// It does not:
///
/// - Authenticate the caller.
/// - Authorize hardware access.
/// - Verify provider trust.
/// - Reserve hardware resources.
/// - Guarantee future session availability.
///
/// # Errors
///
/// Returns a structured error when the session is inconsistent
/// or already closed.
pub fn validate_session(
    session: &dyn HardwareSession,
) -> ZhalResult<()> {
    let identity = session.identity();
    let device = session.device_identity();

    if identity.provider_id() != device.provider_id() {
        return Err(
            contract_violation(
                "hardware session and associated device have different provider identities",
            )
            .with_session(identity.clone())
            .with_device(device.clone()),
        );
    }

    if session.is_closed() {
        return Err(
            session_error(
                ZhalErrorKind::DeviceUnavailable,
                "the provider-managed hardware session is already closed",
            )
            .with_session(identity.clone())
            .with_device(device.clone()),
        );
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_state_has_stable_identifiers() {
        assert_eq!(
            HardwareSessionState::Open.as_str(),
            "open",
        );

        assert_eq!(
            HardwareSessionState::Closed.as_str(),
            "closed",
        );
    }

    #[test]
    fn session_state_classification_is_consistent() {
        assert!(HardwareSessionState::Open.is_open());
        assert!(!HardwareSessionState::Open.is_closed());

        assert!(HardwareSessionState::Closed.is_closed());
        assert!(!HardwareSessionState::Closed.is_open());
    }
}
