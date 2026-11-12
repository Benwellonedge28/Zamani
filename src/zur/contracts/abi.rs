
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Execution ABI
//!
//! Path: `src/zur/contracts/abi.rs`
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! ## Purpose
//!
//! Defines the runtime-facing execution Application Binary Interface
//! contract shared by compatible Zamani artifacts, runtime hosts,
//! and runtime services.
//!
//! This module specifies how an already-produced executable entry point
//! is invoked and how values, ownership, errors, and service calls cross
//! the runtime boundary.
//!
//! It does not generate executable code, load artifacts, execute
//! instructions, or establish source-language semantics.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Runtime ABI descriptors.
//! - Runtime ABI symbol identifiers.
//! - ABI type references.
//! - Entry-point declarations.
//! - Calling-convention identifiers.
//! - Invocation-mode declarations.
//! - Parameter and result transfer contracts.
//! - Ownership and lifetime transfer declarations.
//! - Error-transfer declarations.
//! - Runtime service invocation declarations.
//! - Structural ABI validation.
//!
//! ## It does not own
//!
//! - Source-language types or semantic meaning.
//! - AST or semantic-model declarations.
//! - Classical, quantum, HDL, AI, or other domain IR.
//! - Compiler artifacts or artifact manifests.
//! - Backend registration or code generation.
//! - Physical hardware or device capabilities.
//! - Resource discovery, reservation, or allocation.
//! - Compiler planning or target selection.
//! - Service registration or implementation.
//! - Authentication or authorization.
//! - Actual memory ownership enforcement.
//! - Serialization of arbitrary program values.
//! - Foreign-language ABI implementation.
//!
//! Those responsibilities remain with their respective authorities.
//!
//! ## Compatibility
//!
//! ABI version identity is owned by `zur::version::RuntimeAbiVersion`.
//!
//! Compatibility is evaluated using an explicitly supplied
//! `RuntimeAbiCompatibilityPolicy`.
//!
//! Version equality alone does not prove:
//!
//! - Artifact integrity.
//! - Calling-convention implementation.
//! - Type-layout compatibility.
//! - Semantic equivalence.
//! - Resource feasibility.
//! - Provider availability.
//! - Successful execution.
//!
//! The runtime must perform those checks at their respective boundaries.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on:
//!
//! - Entry points.
//! - Parameters.
//! - ABI type references.
//! - Service methods.
//! - Services.
//! - Computational domains.
//! - Providers.
//! - Devices.
//! - Resource quantities.
//!
//! Collections grow subject to available memory and platform limits.
//!
//! This does not promise infinite physical resources.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No raw pointer API is exposed by this module.
//! No unsafe implementation is introduced.
//! No global mutable state is used.
//!
//! ## Determinism
//!
//! ABI descriptors preserve declared parameter order.
//!
//! Identifiers are validated before construction.
//! Structural validation does not inspect the host environment.
//!
//! ## Integration
//!
//! - `zur/error.rs` supplies `ZurError` and `ZurResult`.
//! - `zur/version.rs` supplies ABI version identities and compatibility.
//! - `zur/contracts/mod.rs` exports this module.
//! - `zur/execution/artifact.rs` consumes ABI declarations from artifacts.
//! - `zur/execution/loader.rs` validates ABI compatibility before loading.
//! - `zur/execution/dispatch.rs` resolves the declared invocation path.
//! - `zur/execution/executor.rs` invokes compatible entry points.
//! - `zur/core/service_registry.rs` resolves declared runtime services.
//! - `backend/` remains responsible for generating compatible artifacts.
//! - `compiler/artifacts/` remains responsible for canonical artifacts.
//!
//! This module must not be changed merely because a new computational
//! domain or hardware provider is introduced.

use core::fmt;
use std::collections::BTreeSet;

use crate::zur::error::{ZurError, ZurResult};
use crate::zur::version::{
    Compatibility,
    CompatibilityDecision,
    RuntimeAbiCompatibilityPolicy,
    RuntimeAbiVersion,
};

// =============================================================================
// IDENTIFIER VALIDATION
// =============================================================================

/// Validates a namespaced ABI identifier.
///
/// Format:
///
///     namespace.segment.identifier
///
/// Permitted characters:
///
/// - ASCII lowercase letters.
/// - ASCII digits.
/// - Hyphens.
/// - Underscores.
/// - Dots separating nonempty segments.
///
/// There is no arbitrary identifier-length or namespace-depth ceiling.
fn validate_namespaced_identifier(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(abi_input_error(
            format!("{description} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(abi_input_error(
            format!(
                "{description} must not contain surrounding whitespace"
            ),
        ));
    }

    let mut segment_count = 0usize;

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(abi_input_error(
                format!(
                    "{description} contains an empty namespace segment"
                ),
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        }) {
            return Err(abi_input_error(
                format!(
                    "{description} contains an invalid character"
                ),
            ));
        }

        if segment.starts_with('-')
            || segment.starts_with('_')
            || segment.ends_with('-')
            || segment.ends_with('_')
        {
            return Err(abi_input_error(
                format!(
                    "{description} has an invalid segment boundary"
                ),
            ));
        }

        segment_count += 1;
    }

    if segment_count < 2 {
        return Err(abi_input_error(
            format!(
                "{description} must contain at least two namespace segments"
            ),
        ));
    }

    Ok(())
}

/// Constructs a structured ABI input error.
fn abi_input_error(message: impl Into<String>) -> ZurError {
    ZurError::invalid_input(message)
        .with_component("zur.contracts.abi")
}

// =============================================================================
// ABI IDENTIFIERS
// =============================================================================

/// A validated, namespaced ABI type identity.
///
/// This is an opaque reference to a type contract owned by another
/// authoritative subsystem.
///
/// It does not define the type's source-language meaning, physical
/// representation, layout, or domain-specific behavior.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbiTypeId(String);

impl AbiTypeId {
    /// Constructs a validated ABI type identity.
    pub fn new(value: impl Into<String>) -> ZurResult<Self> {
        let value = value.into();

        validate_namespaced_identifier(
            &value,
            "ABI type identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identity and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for AbiTypeId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A validated runtime ABI calling-convention identifier.
///
/// Calling conventions are open-ended namespaced identifiers rather
/// than a closed inventory of CPU architectures or compiler backends.
///
/// The identifier describes a contract. It does not implement the
/// calling convention.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallingConvention(String);

impl CallingConvention {
    /// Constructs a validated calling-convention identifier.
    pub fn new(value: impl Into<String>) -> ZurResult<Self> {
        let value = value.into();

        validate_namespaced_identifier(
            &value,
            "calling convention",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CallingConvention {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A validated executable or service-method symbol.
///
/// Symbols are intentionally not restricted to the namespaced identifier
/// format because native and foreign toolchains may use different symbol
/// mangling conventions.
///
/// Empty strings, surrounding whitespace, and control characters are
/// rejected. The original spelling is preserved.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbiSymbol(String);

impl AbiSymbol {
    /// Constructs a validated ABI symbol.
    pub fn new(value: impl Into<String>) -> ZurResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(abi_input_error(
                "ABI symbol must not be empty",
            ));
        }

        if value.trim() != value {
            return Err(abi_input_error(
                "ABI symbol must not contain surrounding whitespace",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(abi_input_error(
                "ABI symbol must not contain control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the original symbol spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the symbol and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for AbiSymbol {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// INVOCATION CONTRACTS
// =============================================================================

/// Declares how an ABI entry point is invoked.
///
/// These are invocation mechanics, not computational domains.
///
/// A provider may implement one or more invocation modes. The runtime
/// must verify support before invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InvocationMode {
    /// The invocation completes through one call/return interaction.
    Synchronous,

    /// The invocation may complete asynchronously.
    Asynchronous,

    /// The invocation produces a sequence of results.
    Streaming,

    /// The invocation establishes or uses a persistent session.
    Session,
}

/// Declares how an ABI value is passed across the runtime boundary.
///
/// This is a transfer contract, not a Rust pointer representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PassingMode {
    /// The value is transferred according to its ABI-defined value
    /// representation.
    ByValue,

    /// The receiver obtains temporary read-only access.
    Borrowed,

    /// The receiver obtains temporary mutable access.
    BorrowedMutable,

    /// The receiver obtains an opaque handle governed by another
    /// explicit contract.
    OpaqueHandle,

    /// The receiver obtains a stream-oriented value contract.
    Stream,
}

/// Declares ownership behavior at the ABI boundary.
///
/// The declaration does not itself transfer or enforce ownership.
/// The ABI implementation must enforce the declared behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OwnershipMode {
    /// The caller retains ownership.
    CallerRetains,

    /// The receiver temporarily borrows the value.
    Borrowed,

    /// Ownership transfers to the receiver.
    TransferredToReceiver,

    /// The value is shared under an immutable-sharing contract.
    SharedImmutable,

    /// The value is shared under a separately enforced mutable-sharing
    /// contract.
    SharedMutable,
}

/// Declares the maximum contractual lifetime scope of an ABI value.
///
/// These scopes are logical ABI contracts. They are not Rust lifetimes
/// and must not be used as substitutes for Rust's borrow checker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LifetimeScope {
    /// The value is valid only during one invocation.
    Invocation,

    /// The value may remain valid for the associated session.
    Session,

    /// The value may remain valid for the runtime instance.
    Runtime,

    /// The value is governed by an independently defined external
    /// ownership and lifetime contract.
    External,
}

/// Describes the transfer contract for one ABI value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueContract {
    type_id: AbiTypeId,
    passing: PassingMode,
    ownership: OwnershipMode,
    lifetime: LifetimeScope,
}

impl ValueContract {
    /// Constructs a value-transfer contract.
    ///
    /// The constructor rejects combinations that contradict the
    /// declared transfer mechanics.
    pub fn new(
        type_id: AbiTypeId,
        passing: PassingMode,
        ownership: OwnershipMode,
        lifetime: LifetimeScope,
    ) -> ZurResult<Self> {
        let valid = match passing {
            PassingMode::ByValue => {
                !matches!(
                    ownership,
                    OwnershipMode::Borrowed
                )
            }

            PassingMode::Borrowed => {
                matches!(
                    ownership,
                    OwnershipMode::Borrowed
                        | OwnershipMode::CallerRetains
                )
            }

            PassingMode::BorrowedMutable => {
                matches!(
                    ownership,
                    OwnershipMode::Borrowed
                        | OwnershipMode::CallerRetains
                )
            }

            PassingMode::OpaqueHandle => true,

            PassingMode::Stream => {
                matches!(
                    lifetime,
                    LifetimeScope::Session
                        | LifetimeScope::Runtime
                        | LifetimeScope::External
                )
            }
        };

        if !valid {
            return Err(abi_input_error(
                "ABI value has an inconsistent passing, ownership, or lifetime contract",
            ));
        }

        Ok(Self {
            type_id,
            passing,
            ownership,
            lifetime,
        })
    }

    /// Returns the referenced ABI type identity.
    #[must_use]
    pub fn type_id(&self) -> &AbiTypeId {
        &self.type_id
    }

    /// Returns the passing mode.
    #[must_use]
    pub const fn passing(&self) -> PassingMode {
        self.passing
    }

    /// Returns the ownership mode.
    #[must_use]
    pub const fn ownership(&self) -> OwnershipMode {
        self.ownership
    }

    /// Returns the lifetime scope.
    #[must_use]
    pub const fn lifetime(&self) -> LifetimeScope {
        self.lifetime
    }
}

// =============================================================================
// ENTRY-POINT PARAMETERS
// =============================================================================

/// Describes one named entry-point parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbiParameter {
    name: AbiSymbol,
    contract: ValueContract,
}

impl AbiParameter {
    /// Constructs an entry-point parameter.
    #[must_use]
    pub fn new(
        name: AbiSymbol,
        contract: ValueContract,
    ) -> Self {
        Self {
            name,
            contract,
        }
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &AbiSymbol {
        &self.name
    }

    /// Returns the parameter transfer contract.
    #[must_use]
    pub fn contract(&self) -> &ValueContract {
        &self.contract
    }
}

// =============================================================================
// ERROR TRANSFER
// =============================================================================

/// Declares the ABI mechanism for transferring execution errors.
///
/// The concrete error representation is an ABI type reference.
/// This module does not redefine the canonical ZUR error model.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorTransferContract {
    error_type: AbiTypeId,
    ownership: OwnershipMode,
    lifetime: LifetimeScope,
}

impl ErrorTransferContract {
    /// Constructs an error-transfer contract.
    pub fn new(
        error_type: AbiTypeId,
        ownership: OwnershipMode,
        lifetime: LifetimeScope,
    ) -> ZurResult<Self> {
        if matches!(
            ownership,
            OwnershipMode::Borrowed
                | OwnershipMode::SharedMutable
        ) && lifetime == LifetimeScope::Invocation
        {
            return Err(abi_input_error(
                "error transfer ownership is inconsistent with its lifetime",
            ));
        }

        Ok(Self {
            error_type,
            ownership,
            lifetime,
        })
    }

    /// Returns the error representation type.
    #[must_use]
    pub fn error_type(&self) -> &AbiTypeId {
        &self.error_type
    }

    /// Returns the error ownership contract.
    #[must_use]
    pub const fn ownership(&self) -> OwnershipMode {
        self.ownership
    }

    /// Returns the error lifetime contract.
    #[must_use]
    pub const fn lifetime(&self) -> LifetimeScope {
        self.lifetime
    }
}

// =============================================================================
// ENTRY-POINT DESCRIPTOR
// =============================================================================

/// Declares one runtime-callable executable entry point.
///
/// The parameter vector preserves declaration order because parameter
/// order is part of the calling contract.
///
/// This descriptor does not contain executable bytes, artifact metadata,
/// compiler plans, or source-language semantic declarations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntryPointDescriptor {
    symbol: AbiSymbol,
    calling_convention: CallingConvention,
    invocation_mode: InvocationMode,
    parameters: Vec<AbiParameter>,
    result: Option<ValueContract>,
    errors: ErrorTransferContract,
}

impl EntryPointDescriptor {
    /// Constructs and validates an entry-point descriptor.
    pub fn new(
        symbol: AbiSymbol,
        calling_convention: CallingConvention,
        invocation_mode: InvocationMode,
        parameters: Vec<AbiParameter>,
        result: Option<ValueContract>,
        errors: ErrorTransferContract,
    ) -> ZurResult<Self> {
        let mut names = BTreeSet::new();

        for parameter in &parameters {
            if !names.insert(parameter.name().as_str()) {
                return Err(abi_input_error(
                    format!(
                        "duplicate ABI parameter name `{}`",
                        parameter.name()
                    ),
                ));
            }
        }

        if invocation_mode == InvocationMode::Streaming {
            let has_stream_result = result.as_ref().is_some_and(
                |value| value.passing() == PassingMode::Stream,
            );

            if !has_stream_result {
                return Err(abi_input_error(
                    "streaming entry points require a stream result contract",
                ));
            }
        }

        Ok(Self {
            symbol,
            calling_convention,
            invocation_mode,
            parameters,
            result,
            errors,
        })
    }

    /// Returns the executable symbol.
    #[must_use]
    pub fn symbol(&self) -> &AbiSymbol {
        &self.symbol
    }

    /// Returns the calling convention.
    #[must_use]
    pub fn calling_convention(&self) -> &CallingConvention {
        &self.calling_convention
    }

    /// Returns the invocation mode.
    #[must_use]
    pub const fn invocation_mode(&self) -> InvocationMode {
        self.invocation_mode
    }

    /// Returns parameters in their declared ABI order.
    #[must_use]
    pub fn parameters(&self) -> &[AbiParameter] {
        &self.parameters
    }

    /// Returns the optional result contract.
    #[must_use]
    pub fn result(&self) -> Option<&ValueContract> {
        self.result.as_ref()
    }

    /// Returns the error-transfer contract.
    #[must_use]
    pub fn errors(&self) -> &ErrorTransferContract {
        &self.errors
    }
}

// =============================================================================
// RUNTIME SERVICE INVOCATION
// =============================================================================

/// Declares one runtime service method invocation.
///
/// The service identity and service-contract version are reused from
/// existing ZUR configuration and version authorities.
///
/// This descriptor does not register or implement the service.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceInvocationDescriptor {
    service: crate::zur::config::ServiceKey,
    contract_version: crate::zur::version::ServiceContractVersion,
    method: AbiSymbol,
    invocation_mode: InvocationMode,
    parameters: Vec<AbiParameter>,
    result: Option<ValueContract>,
    errors: ErrorTransferContract,
}

impl ServiceInvocationDescriptor {
    /// Constructs a runtime service invocation descriptor.
    pub fn new(
        service: crate::zur::config::ServiceKey,
        contract_version: crate::zur::version::ServiceContractVersion,
        method: AbiSymbol,
        invocation_mode: InvocationMode,
        parameters: Vec<AbiParameter>,
        result: Option<ValueContract>,
        errors: ErrorTransferContract,
    ) -> ZurResult<Self> {
        let mut names = BTreeSet::new();

        for parameter in &parameters {
            if !names.insert(parameter.name().as_str()) {
                return Err(abi_input_error(
                    format!(
                        "duplicate service parameter name `{}`",
                        parameter.name()
                    ),
                ));
            }
        }

        if invocation_mode == InvocationMode::Streaming {
            let has_stream_result = result.as_ref().is_some_and(
                |value| value.passing() == PassingMode::Stream,
            );

            if !has_stream_result {
                return Err(abi_input_error(
                    "streaming service methods require a stream result contract",
                ));
            }
        }

        Ok(Self {
            service,
            contract_version,
            method,
            invocation_mode,
            parameters,
            result,
            errors,
        })
    }

    /// Returns the service contract identity.
    #[must_use]
    pub fn service(&self) -> &crate::zur::config::ServiceKey {
        &self.service
    }

    /// Returns the service contract version.
    #[must_use]
    pub fn contract_version(
        &self,
    ) -> &crate::zur::version::ServiceContractVersion {
        &self.contract_version
    }

    /// Returns the service method symbol.
    #[must_use]
    pub fn method(&self) -> &AbiSymbol {
        &self.method
    }

    /// Returns the invocation mode.
    #[must_use]
    pub const fn invocation_mode(&self) -> InvocationMode {
        self.invocation_mode
    }

    /// Returns parameters in declared order.
    #[must_use]
    pub fn parameters(&self) -> &[AbiParameter] {
        &self.parameters
    }

    /// Returns the optional result contract.
    #[must_use]
    pub fn result(&self) -> Option<&ValueContract> {
        self.result.as_ref()
    }

    /// Returns the error-transfer contract.
    #[must_use]
    pub fn errors(&self) -> &ErrorTransferContract {
        &self.errors
    }
}

// =============================================================================
// COMPLETE RUNTIME ABI
// =============================================================================

/// Complete runtime-facing execution ABI descriptor.
///
/// This is the canonical ZUR contract for the calling and transfer
/// boundary between an executable artifact and a compatible runtime.
///
/// It deliberately does not contain an artifact manifest or executable
/// payload. Those remain owned by the compiler artifact and backend
/// manifest systems.
///
/// Entry points are stored in declaration order. Duplicate symbols
/// are rejected.
///
/// Runtime service declarations are also stored in declaration order.
/// Their identities and versions are validated by their respective
/// authorities during runtime loading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAbi {
    version: RuntimeAbiVersion,
    entry_points: Vec<EntryPointDescriptor>,
    service_invocations: Vec<ServiceInvocationDescriptor>,
}

impl RuntimeAbi {
    /// Constructs a runtime ABI and validates its structure.
    pub fn new(
        version: RuntimeAbiVersion,
        entry_points: Vec<EntryPointDescriptor>,
        service_invocations: Vec<ServiceInvocationDescriptor>,
    ) -> ZurResult<Self> {
        let mut symbols = BTreeSet::new();

        for entry_point in &entry_points {
            if !symbols.insert(entry_point.symbol().as_str()) {
                return Err(abi_input_error(
                    format!(
                        "duplicate runtime ABI entry-point symbol `{}`",
                        entry_point.symbol()
                    ),
                ));
            }
        }

        Ok(Self {
            version,
            entry_points,
            service_invocations,
        })
    }

    /// Returns the ABI version.
    #[must_use]
    pub fn version(&self) -> &RuntimeAbiVersion {
        &self.version
    }

    /// Returns all entry points in declaration order.
    #[must_use]
    pub fn entry_points(&self) -> &[EntryPointDescriptor] {
        &self.entry_points
    }

    /// Returns all declared runtime service invocations.
    #[must_use]
    pub fn service_invocations(
        &self,
    ) -> &[ServiceInvocationDescriptor] {
        &self.service_invocations
    }

    /// Finds an entry point by its exact symbol.
    #[must_use]
    pub fn entry_point(
        &self,
        symbol: &AbiSymbol,
    ) -> Option<&EntryPointDescriptor> {
        self.entry_points
            .iter()
            .find(|entry| entry.symbol() == symbol)
    }

    /// Evaluates this ABI version against an explicit compatibility
    /// policy.
    ///
    /// This operation does not execute migrations, validate artifact
    /// integrity, or prove that a host implements the declared ABI.
    #[must_use]
    pub fn compatibility(
        &self,
        policy: &RuntimeAbiCompatibilityPolicy,
    ) -> CompatibilityDecision<RuntimeAbiVersion> {
        policy.check(self.version.clone())
    }

    /// Requires direct ABI compatibility.
    ///
    /// A migration-required decision is not treated as directly
    /// compatible. The caller must invoke an explicitly approved
    /// migration authority before accepting a migrated ABI.
    pub fn require_direct_compatibility(
        &self,
        policy: &RuntimeAbiCompatibilityPolicy,
    ) -> ZurResult<()> {
        let decision = self.compatibility(policy);

        if decision.compatibility() == Compatibility::Compatible {
            return Ok(());
        }

        Err(
            ZurError::compatibility(
                format!(
                    "runtime ABI version `{}` is not directly compatible with `{}`: {:?}",
                    decision.supplied(),
                    decision.current(),
                    decision.reason(),
                ),
            )
            .with_component("zur.contracts.abi")
            .with_operation("require_direct_compatibility"),
        )
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn abi_version() -> RuntimeAbiVersion {
        RuntimeAbiVersion::new("1", "0", "0")
            .expect("test ABI version must be valid")
    }

    fn type_id(name: &str) -> AbiTypeId {
        AbiTypeId::new(name)
            .expect("test ABI type identity must be valid")
    }

    fn symbol(name: &str) -> AbiSymbol {
        AbiSymbol::new(name)
            .expect("test ABI symbol must be valid")
    }

    fn convention() -> CallingConvention {
        CallingConvention::new("zamani.native.system")
            .expect("test calling convention must be valid")
    }

    fn value_contract() -> ValueContract {
        ValueContract::new(
            type_id("zamani.core.integer"),
            PassingMode::ByValue,
            OwnershipMode::CallerRetains,
            LifetimeScope::Invocation,
        )
        .expect("test value contract must be valid")
    }

    fn error_contract() -> ErrorTransferContract {
        ErrorTransferContract::new(
            type_id("zamani.runtime.error"),
            OwnershipMode::TransferredToReceiver,
            LifetimeScope::Invocation,
        )
        .expect("test error contract must be valid")
    }

    fn entry_point(name: &str) -> EntryPointDescriptor {
        EntryPointDescriptor::new(
            symbol(name),
            convention(),
            InvocationMode::Synchronous,
            Vec::new(),
            Some(value_contract()),
            error_contract(),
        )
        .expect("test entry point must be valid")
    }

    #[test]
    fn accepts_namespaced_abi_type_ids() {
        assert!(AbiTypeId::new("zamani.core.integer").is_ok());
        assert!(AbiTypeId::new("vendor.extension.custom_type").is_ok());
    }

    #[test]
    fn rejects_unqualified_abi_type_ids() {
        assert!(AbiTypeId::new("integer").is_err());
    }

    #[test]
    fn rejects_empty_or_invalid_calling_conventions() {
        assert!(CallingConvention::new("").is_err());
        assert!(CallingConvention::new("native").is_err());
        assert!(CallingConvention::new("zamani..native").is_err());
        assert!(CallingConvention::new("zamani.native").is_ok());
    }

    #[test]
    fn preserves_native_symbol_spelling() {
        let value = AbiSymbol::new("_zamani_entry.v1")
            .expect("valid native symbol");

        assert_eq!(value.as_str(), "_zamani_entry.v1");
    }

    #[test]
    fn rejects_empty_and_control_character_symbols() {
        assert!(AbiSymbol::new("").is_err());
        assert!(AbiSymbol::new("entry\npoint").is_err());
        assert!(AbiSymbol::new(" entry ").is_err());
    }

    #[test]
    fn rejects_inconsistent_value_transfer_contracts() {
        let invalid = ValueContract::new(
            type_id("zamani.core.integer"),
            PassingMode::Borrowed,
            OwnershipMode::TransferredToReceiver,
            LifetimeScope::Invocation,
        );

        assert!(invalid.is_err());
    }

    #[test]
    fn accepts_consistent_value_transfer_contracts() {
        let valid = ValueContract::new(
            type_id("zamani.core.integer"),
            PassingMode::Borrowed,
            OwnershipMode::Borrowed,
            LifetimeScope::Invocation,
        );

        assert!(valid.is_ok());
    }

    #[test]
    fn rejects_duplicate_parameter_names() {
        let parameter = || {
            AbiParameter::new(
                symbol("value"),
                value_contract(),
            )
        };

        let result = EntryPointDescriptor::new(
            symbol("run"),
            convention(),
            InvocationMode::Synchronous,
            vec![parameter(), parameter()],
            None,
            error_contract(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_entry_point_symbols() {
        let result = RuntimeAbi::new(
            abi_version(),
            vec![
                entry_point("run"),
                entry_point("run"),
            ],
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_distinct_entry_points() {
        let result = RuntimeAbi::new(
            abi_version(),
            vec![
                entry_point("run"),
                entry_point("initialize"),
            ],
            Vec::new(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn preserves_entry_point_declaration_order() {
        let abi = RuntimeAbi::new(
            abi_version(),
            vec![
                entry_point("initialize"),
                entry_point("run"),
            ],
            Vec::new(),
        )
        .expect("test ABI must be valid");

        assert_eq!(
            abi.entry_points()[0].symbol().as_str(),
            "initialize"
        );

        assert_eq!(
            abi.entry_points()[1].symbol().as_str(),
            "run"
        );
    }

    #[test]
    fn resolves_entry_point_by_exact_symbol() {
        let abi = RuntimeAbi::new(
            abi_version(),
            vec![entry_point("run")],
            Vec::new(),
        )
        .expect("test ABI must be valid");

        assert!(
            abi.entry_point(&symbol("run")).is_some()
        );

        assert!(
            abi.entry_point(&symbol("other")).is_none()
        );
    }

    #[test]
    fn streaming_entry_point_requires_stream_result() {
        let result = EntryPointDescriptor::new(
            symbol("stream"),
            convention(),
            InvocationMode::Streaming,
            Vec::new(),
            Some(value_contract()),
            error_contract(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn streaming_entry_point_accepts_stream_result() {
        let stream_result = ValueContract::new(
            type_id("zamani.core.stream"),
            PassingMode::Stream,
            OwnershipMode::TransferredToReceiver,
            LifetimeScope::Session,
        )
        .expect("stream result must be valid");

        let result = EntryPointDescriptor::new(
            symbol("stream"),
            convention(),
            InvocationMode::Streaming,
            Vec::new(),
            Some(stream_result),
            error_contract(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn accepts_large_parameter_collections_without_fixed_ceiling() {
        let parameters = (0..1024)
            .map(|index| {
                AbiParameter::new(
                    symbol(format!("parameter_{index}")),
                    value_contract(),
                )
            })
            .collect();

        let result = EntryPointDescriptor::new(
            symbol("many_parameters"),
            convention(),
            InvocationMode::Synchronous,
            parameters,
            None,
            error_contract(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_duplicate_parameter_names_in_service_calls() {
        let service = crate::zur::config::ServiceKey::new(
            "zamani.runtime.memory",
        )
        .expect("service key must be valid");

        let version =
            crate::zur::version::ServiceContractVersion::new(
                "1",
                "0",
                "0",
            )
            .expect("service contract version must be valid");

        let parameter = || {
            AbiParameter::new(
                symbol("input"),
                value_contract(),
            )
        };

        let result = ServiceInvocationDescriptor::new(
            service,
            version,
            symbol("allocate"),
            InvocationMode::Synchronous,
            vec![parameter(), parameter()],
            None,
            error_contract(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn abi_version_is_checked_through_explicit_policy() {
        use crate::zur::version::{
            RuntimeAbiCompatibilityPolicy,
        };

        let version = abi_version();

        let policy =
            RuntimeAbiCompatibilityPolicy::exact(version.clone())
                .expect("exact compatibility policy must be valid");

        let abi = RuntimeAbi::new(
            version,
            vec![entry_point("run")],
            Vec::new(),
        )
        .expect("test ABI must be valid");

        assert!(
            abi.require_direct_compatibility(&policy).is_ok()
        );
    }
}
