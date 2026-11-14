
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Interoperability ABI Adapter
//!
//! Path: src/zur/interop/abi.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Adapts the existing authoritative interoperability ABI contracts
//! into the canonical ZUR runtime execution ABI.
//!
//! This module does not define a competing ABI representation.
//!
//! Source ABI declarations remain owned by:
//!
//!     crate::classical::interoperability::abi
//!
//! Canonical runtime ABI declarations remain owned by:
//!
//!     crate::zur::contracts::abi
//!
//! ## Responsibilities
//!
//! - Adapt foreign ABI declarations.
//! - Resolve foreign calling conventions.
//! - Resolve ABI boundary types.
//! - Preserve parameter declaration order.
//! - Resolve invocation modes.
//! - Resolve error-transfer contracts.
//! - Reject unsupported conversions.
//! - Preserve source ABI information.
//! - Produce validated canonical RuntimeAbi descriptors.
//!
//! ## Non-responsibilities
//!
//! This module does not:
//!
//! - Parse Zamani source.
//! - Construct AST nodes.
//! - Perform semantic analysis.
//! - Define foreign-language semantics.
//! - Define another ABI model.
//! - Calculate physical type layouts.
//! - Select CPU architectures.
//! - Select GPU, FPGA, or QPU devices.
//! - Load dynamic libraries.
//! - Resolve physical memory addresses.
//! - Dereference foreign pointers.
//! - Perform linking.
//! - Execute foreign functions.
//! - Allocate physical resources.
//! - Authenticate foreign providers.
//! - Authorize protected operations.
//! - Implement hardware-specific calling sequences.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Important architectural boundaries
//!
//! ABI declaration != ABI implementation.
//!
//! ABI compatibility != provider compatibility.
//!
//! Provider compatibility != provider availability.
//!
//! Provider availability != resource reservation.
//!
//! Resource reservation != successful execution.
//!
//! A successfully constructed adapter does not establish that a
//! particular machine can execute the foreign callable.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - ABI declarations.
//! - Functions.
//! - Parameters.
//! - Foreign types.
//! - Calling conventions.
//! - ABI profiles.
//! - Computational domains.
//! - Providers.
//! - Hardware devices.
//! - Execution environments.
//!
//! Collections grow according to available resources.
//!
//! No finite machine promises literal infinite computation.
//!
//! ## Safety
//!
//! No unsafe code.
//! No raw-pointer API.
//! No global mutable state.
//! No fabricated execution success.
//! No implicit calling-convention selection.
//! No implicit ABI compatibility.
//!
//! ## Integration
//!
//! Source authority:
//!     classical::interoperability::abi
//!
//! Runtime ABI authority:
//!     zur::contracts::abi
//!
//! Error authority:
//!     zur::error
//!
//! Version authority:
//!     zur::version
//!
//! Execution consumers:
//!     zur::execution
//!
//! Security:
//!     zur::security
//!
//! Canonical external-call IR:
//!     quantum::ir::classical::extern_call

use core::fmt;

use crate::classical::interoperability::abi::{
    AbiBoundaryKind,
    AbiContract,
    AbiFunction,
    AbiTypeRef,
};

use crate::zur::contracts::abi::{
    AbiParameter,
    AbiSymbol,
    AbiTypeId,
    CallingConvention,
    EntryPointDescriptor,
    ErrorTransferContract,
    InvocationMode,
    RuntimeAbi,
    ValueContract,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::version::RuntimeAbiVersion;

// ============================================================================
// ERROR CLASSIFICATION
// ============================================================================

/// Interoperability ABI adaptation error.
///
/// This enum identifies structural and adaptation failures.
/// Operational errors continue to use the canonical ZUR error model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InteropAbiError {
    /// The source ABI contract is structurally invalid.
    InvalidSourceContract,

    /// The source contract is not callable.
    NonCallableContract,

    /// The source contract does not contain callable declarations.
    EmptyCallableContract,

    /// The calling convention cannot be resolved.
    UnresolvedCallingConvention,

    /// A foreign type cannot be resolved.
    UnresolvedType,

    /// The source parameter cannot be represented.
    InvalidParameter,

    /// The source result cannot be represented.
    InvalidResult,

    /// The source function uses unsupported variadic behavior.
    UnsupportedVariadicFunction,

    /// The invocation mode cannot be represented.
    UnsupportedInvocationMode,

    /// The error-transfer contract cannot be resolved.
    UnresolvedErrorTransfer,

    /// The canonical runtime ABI rejected the adapted declaration.
    InvalidRuntimeAbi,
}

impl fmt::Display for InteropAbiError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidSourceContract =>
                "source ABI contract is invalid",

            Self::NonCallableContract =>
                "data-only ABI contracts cannot become callable entry points",

            Self::EmptyCallableContract =>
                "callable ABI contract contains no functions",

            Self::UnresolvedCallingConvention =>
                "foreign calling convention has not been resolved",

            Self::UnresolvedType =>
                "foreign ABI type has not been resolved",

            Self::InvalidParameter =>
                "foreign ABI parameter cannot be adapted",

            Self::InvalidResult =>
                "foreign ABI result cannot be adapted",

            Self::UnsupportedVariadicFunction =>
                "variadic functions are not represented by the canonical ZUR ABI",

            Self::UnsupportedInvocationMode =>
                "foreign invocation mode cannot be represented",

            Self::UnresolvedErrorTransfer =>
                "foreign error-transfer contract has not been resolved",

            Self::InvalidRuntimeAbi =>
                "adapted declaration violates canonical ZUR ABI invariants",
        })
    }
}

impl std::error::Error for InteropAbiError {}

fn adaptation_error(
    kind: InteropAbiError,
) -> ZurError {
    ZurError::invalid_input(kind.to_string())
        .with_component("zur.interop.abi")
        .with_operation("adapt")
}

fn unsupported_error(
    kind: InteropAbiError,
) -> ZurError {
    ZurError::unsupported(kind.to_string())
        .with_component("zur.interop.abi")
        .with_operation("adapt")
}

// ============================================================================
// VALUE DIRECTION
// ============================================================================

/// Identifies the direction in which an ABI value crosses a boundary.
///
/// Direction matters because ownership transfer has different
/// interpretations for parameters and returned values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AbiValueDirection {
    /// A value passed from the Zamani caller to the foreign callable.
    Parameter,

    /// A value returned from the foreign callable to the Zamani caller.
    Result,
}

// ============================================================================
// RESOLUTION CONTRACT
// ============================================================================

/// Resolves foreign ABI declarations into canonical ZUR contracts.
///
/// Implementations belong to the appropriate semantic, compiler,
/// or interoperability integration layer.
///
/// A resolver must not invent compatibility or silently discard
/// meaningful ABI information.
///
/// In particular, it must explicitly resolve:
///
/// - Foreign type identities.
/// - Calling-convention identities.
/// - Passing and ownership contracts.
/// - Lifetime requirements.
/// - Invocation modes.
/// - Error-transfer requirements.
///
/// If a source declaration cannot be represented faithfully,
/// the resolver must return an error.
///
/// This interface is extensible: future calling conventions and
/// foreign languages do not require changes to ZUR itself.
pub trait InteropAbiResolver {

    /// Resolves a foreign calling convention.
    ///
    /// `declared` contains the original spelling from the source
    /// ABI profile.
    ///
    /// The returned identifier must be a validated canonical
    /// ZUR calling-convention identity.
    ///
    /// No architecture or vendor is selected here.
    fn resolve_calling_convention(
        &self,
        declared: &str,
    ) -> ZurResult<CallingConvention>;

    /// Resolves one foreign parameter or result type.
    ///
    /// The resolver must preserve the source type's meaningful
    /// representation, ownership, passing, lifetime and
    /// nullability requirements through the appropriate
    /// authoritative semantic contracts.
    ///
    /// A ValueContract that cannot faithfully represent the
    /// required runtime transfer must be rejected.
    fn resolve_value(
        &self,
        source: &AbiTypeRef,
        direction: AbiValueDirection,
    ) -> ZurResult<ValueContract>;

    /// Resolves the invocation mode of a foreign function.
    ///
    /// This allows different functions within one ABI contract
    /// to have different invocation behavior.
    fn resolve_invocation_mode(
        &self,
        function: &AbiFunction,
    ) -> ZurResult<InvocationMode>;

    /// Resolves the canonical error-transfer contract of a
    /// particular foreign function.
    ///
    /// Errors must not be fabricated or silently converted
    /// into successful results.
    fn resolve_error_transfer(
        &self,
        function: &AbiFunction,
    ) -> ZurResult<ErrorTransferContract>;
}

// ============================================================================
// ADAPTED ABI
// ============================================================================

/// A validated adaptation of an existing foreign ABI contract.
///
/// The original source contract is deliberately retained.
///
/// This prevents the adapter from silently replacing or discarding
/// information that the canonical runtime ABI does not independently
/// represent.
///
/// The canonical RuntimeAbi contains the runtime-facing callable
/// declarations.
///
/// The source contract continues to contain:
///
/// - Original linkage declarations.
/// - Foreign ABI profile information.
/// - Foreign version declarations.
/// - Nullability metadata.
/// - Representation hints.
/// - Effects.
/// - Capability requirements.
/// - Variadic declarations.
/// - Extension properties.
///
/// Consumers requiring those details must continue to consult
/// `source_contract()`.
#[derive(Debug)]
pub struct AdaptedAbi<'a> {
    source_contract: &'a AbiContract,
    runtime_abi: RuntimeAbi,
}

impl<'a> AdaptedAbi<'a> {

    /// Returns the original foreign ABI contract.
    ///
    /// This reference is retained to preserve information that
    /// belongs to the source interoperability authority.
    #[must_use]
    pub fn source_contract(&self) -> &'a AbiContract {
        self.source_contract
    }

    /// Returns the canonical ZUR runtime ABI.
    ///
    /// This is the validated runtime-facing projection.
    #[must_use]
    pub fn runtime_abi(&self) -> &RuntimeAbi {
        &self.runtime_abi
    }

    /// Returns the runtime ABI version.
    #[must_use]
    pub fn version(&self) -> &RuntimeAbiVersion {
        self.runtime_abi.version()
    }

    /// Finds a runtime entry point by its exact symbol.
    #[must_use]
    pub fn entry_point(
        &self,
        symbol: &AbiSymbol,
    ) -> Option<&EntryPointDescriptor> {
        self.runtime_abi.entry_point(symbol)
    }
}

// ============================================================================
// ADAPTER
// ============================================================================

/// Stateless adapter between foreign ABI declarations and ZUR.
///
/// The adapter does not own an ABI registry, a linker, a loader,
/// a runtime instance, or a physical execution provider.
///
/// It produces validated descriptors using explicitly supplied
/// resolution decisions.
#[derive(Debug, Default, Clone, Copy)]
pub struct InteropAbiAdapter;

impl InteropAbiAdapter {

    /// Constructs the stateless adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Adapts a foreign ABI contract into the canonical ZUR runtime ABI.
    ///
    /// # Arguments
    ///
    /// - `source`: Existing authoritative foreign ABI declaration.
    /// - `version`: Canonical ZUR runtime ABI version.
    /// - `resolver`: Explicit semantic and ABI resolution authority.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The source contract is invalid.
    /// - The contract is data-oriented rather than callable.
    /// - The source contract has no functions.
    /// - A calling convention cannot be resolved.
    /// - A parameter or result cannot be resolved.
    /// - An invocation mode cannot be represented.
    /// - An error-transfer contract cannot be resolved.
    /// - A function uses unsupported variadic behavior.
    /// - The resulting runtime ABI violates canonical invariants.
    ///
    /// # Important
    ///
    /// Successful adaptation does not mean that a library is
    /// installed, a symbol is resolved, a provider is authorized,
    /// or a target can execute the foreign function.
    pub fn adapt<'a, R>(
        &self,
        source: &'a AbiContract,
        version: RuntimeAbiVersion,
        resolver: &R,
    ) -> ZurResult<AdaptedAbi<'a>>
    where
        R: InteropAbiResolver + ?Sized,
    {
        // --------------------------------------------------------------------
        // 1. Validate the original authority.
        // --------------------------------------------------------------------

        source.validate().map_err(|error| {
            ZurError::invalid_data(format!(
                "{}: {}",
                InteropAbiError::InvalidSourceContract,
                error,
            ))
            .with_component("zur.interop.abi")
            .with_operation("validate_source")
        })?;

        // --------------------------------------------------------------------
        // 2. Only callable ABI contracts can become entry points.
        // --------------------------------------------------------------------

        if source.boundary_kind() != AbiBoundaryKind::Callable {
            return Err(adaptation_error(
                InteropAbiError::NonCallableContract,
            ));
        }

        if source.functions().is_empty() {
            return Err(adaptation_error(
                InteropAbiError::EmptyCallableContract,
            ));
        }

        // --------------------------------------------------------------------
        // 3. Resolve the calling convention.
        //
        // Source spellings such as "c", "cdecl", "sysv64",
        // or future extension names are not assumed to have
        // identical canonical ZUR identities.
        // --------------------------------------------------------------------

        let convention = resolver
            .resolve_calling_convention(
                source.profile().calling_convention(),
            )
            .map_err(|error| {
                ZurError::compatibility(format!(
                    "{}: {}",
                    InteropAbiError::UnresolvedCallingConvention,
                    error,
                ))
                .with_component("zur.interop.abi")
                .with_operation("resolve_calling_convention")
            })?;

        // --------------------------------------------------------------------
        // 4. Adapt each function.
        //
        // The source declaration order is preserved.
        // No arbitrary parameter or function ceiling is introduced.
        // --------------------------------------------------------------------

        let mut entry_points = Vec::with_capacity(
            source.functions().len(),
        );

        for function in source.functions() {

            // --------------------------------------------------------------
            // 4.1 Variadic functions.
            //
            // The existing canonical ZUR EntryPointDescriptor
            // does not have a variadic contract.
            //
            // Do not silently erase variadic semantics.
            // --------------------------------------------------------------

            if function.is_variadic() {
                return Err(unsupported_error(
                    InteropAbiError::UnsupportedVariadicFunction,
                )
                .with_context(
                    "function",
                    function.name(),
                )
                .unwrap_or_else(|_| {
                    unsupported_error(
                        InteropAbiError::UnsupportedVariadicFunction,
                    )
                }));
            }

            // --------------------------------------------------------------
            // 4.2 Resolve the external symbol.
            // --------------------------------------------------------------

            let symbol = AbiSymbol::new(
                function.symbol(),
            )?;

            // --------------------------------------------------------------
            // 4.3 Resolve parameters.
            //
            // Parameter order is part of the ABI.
            // Do not sort or reorder this collection.
            // --------------------------------------------------------------

            let mut parameters = Vec::with_capacity(
                function.parameters().len(),
            );

            for parameter in function.parameters() {

                let contract = resolver
                    .resolve_value(
                        parameter.value_type(),
                        AbiValueDirection::Parameter,
                    )
                    .map_err(|error| {
                        ZurError::compatibility(format!(
                            "{} for parameter `{}`: {}",
                            InteropAbiError::InvalidParameter,
                            parameter.name(),
                            error,
                        ))
                        .with_component("zur.interop.abi")
                        .with_operation("resolve_parameter")
                    })?;

                let name = AbiSymbol::new(
                    parameter.name(),
                )?;

                parameters.push(
                    AbiParameter::new(
                        name,
                        contract,
                    ),
                );
            }

            // --------------------------------------------------------------
            // 4.4 Resolve the optional result.
            // --------------------------------------------------------------

            let result = match function.result() {

                Some(source_result) => {

                    Some(
                        resolver
                            .resolve_value(
                                source_result,
                                AbiValueDirection::Result,
                            )
                            .map_err(|error| {
                                ZurError::compatibility(format!(
                                    "{} for function `{}`: {}",
                                    InteropAbiError::InvalidResult,
                                    function.name(),
                                    error,
                                ))
                                .with_component("zur.interop.abi")
                                .with_operation("resolve_result")
                            })?
                    )
                }

                None => None,
            };

            // --------------------------------------------------------------
            // 4.5 Resolve invocation mode.
            // --------------------------------------------------------------

            let invocation_mode = resolver
                .resolve_invocation_mode(function)
                .map_err(|error| {
                    ZurError::compatibility(format!(
                        "{} for function `{}`: {}",
                        InteropAbiError::UnsupportedInvocationMode,
                        function.name(),
                        error,
                    ))
                    .with_component("zur.interop.abi")
                    .with_operation("resolve_invocation_mode")
                })?;

            // --------------------------------------------------------------
            // 4.6 Resolve error transfer independently.
            //
            // Each function may have a different error contract.
            // --------------------------------------------------------------

            let errors = resolver
                .resolve_error_transfer(function)
                .map_err(|error| {
                    ZurError::compatibility(format!(
                        "{} for function `{}`: {}",
                        InteropAbiError::UnresolvedErrorTransfer,
                        function.name(),
                        error,
                    ))
                    .with_component("zur.interop.abi")
                    .with_operation("resolve_error_transfer")
                })?;

            // --------------------------------------------------------------
            // 4.7 Construct the canonical entry-point descriptor.
            //
            // Canonical structural validation is authoritative.
            // --------------------------------------------------------------

            let entry_point = EntryPointDescriptor::new(
                symbol,
                convention.clone(),
                invocation_mode,
                parameters,
                result,
                errors,
            )
            .map_err(|error| {
                ZurError::invalid_data(format!(
                    "{} for function `{}`: {}",
                    InteropAbiError::InvalidRuntimeAbi,
                    function.name(),
                    error,
                ))
                .with_component("zur.interop.abi")
                .with_operation("construct_entry_point")
            })?;

            entry_points.push(entry_point);
        }

        // --------------------------------------------------------------------
        // 5. Construct the canonical runtime ABI.
        //
        // RuntimeAbi performs its own structural validation,
        // including duplicate entry-point symbol detection.
        // --------------------------------------------------------------------

        let runtime_abi = RuntimeAbi::new(
            version,
            entry_points,
            Vec::new(),
        )
        .map_err(|error| {
            ZurError::invalid_data(format!(
                "{}: {}",
                InteropAbiError::InvalidRuntimeAbi,
                error,
            ))
            .with_component("zur.interop.abi")
            .with_operation("construct_runtime_abi")
        })?;

        // --------------------------------------------------------------------
        // 6. Preserve both authorities.
        //
        // The source ABI remains accessible.
        // The runtime ABI is the validated execution projection.
        // --------------------------------------------------------------------

        Ok(AdaptedAbi {
            source_contract: source,
            runtime_abi,
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::classical::interoperability::abi::{
        AbiContract,
        AbiFunction,
        AbiLinkage,
        AbiParameter as ForeignParameter,
        AbiProfile,
        AbiTypeRef,
    };

    use crate::zur::contracts::abi::{
        AbiTypeId,
        CallingConvention,
        ErrorTransferContract,
        LifetimeScope,
        OwnershipMode,
        PassingMode,
    };

    use crate::zur::version::RuntimeAbiVersion;

    struct TestResolver;

    impl InteropAbiResolver for TestResolver {

        fn resolve_calling_convention(
            &self,
            declared: &str,
        ) -> ZurResult<CallingConvention> {

            // Test-only semantic normalization.
            // Real integrations must resolve through their
            // canonical calling-convention authority.

            assert!(!declared.is_empty());

            CallingConvention::new(
                "test.native.calling_convention",
            )
        }

        fn resolve_value(
            &self,
            source: &AbiTypeRef,
            _direction: AbiValueDirection,
        ) -> ZurResult<ValueContract> {

            let type_id = AbiTypeId::new(
                source.type_name(),
            )?;

            ValueContract::new(
                type_id,
                PassingMode::ByValue,
                OwnershipMode::CallerRetains,
                LifetimeScope::Invocation,
            )
        }

        fn resolve_invocation_mode(
            &self,
            _function: &AbiFunction,
        ) -> ZurResult<InvocationMode> {

            Ok(InvocationMode::Synchronous)
        }

        fn resolve_error_transfer(
            &self,
            _function: &AbiFunction,
        ) -> ZurResult<ErrorTransferContract> {

            ErrorTransferContract::new(
                AbiTypeId::new(
                    "zamani.runtime.error",
                )?,
                OwnershipMode::TransferredToReceiver,
                LifetimeScope::Invocation,
            )
        }
    }

    fn version() -> RuntimeAbiVersion {
        RuntimeAbiVersion::new(
            "1",
            "0",
            "0",
        )
        .expect("valid runtime ABI version")
    }

    fn source_contract() -> AbiContract {

        let profile = AbiProfile::new(
            "portable",
            "c",
            AbiLinkage::Host,
        )
        .expect("valid ABI profile");

        let mut contract = AbiContract::new(
            "test.foreign",
            AbiBoundaryKind::Callable,
            profile,
        )
        .expect("valid ABI contract");

        let integer = AbiTypeRef::new(
            "zamani.core.integer",
        )
        .expect("valid type");

        let function = AbiFunction::new(
            "add",
            "foreign_add",
            vec![
                ForeignParameter::new(
                    "left",
                    integer.clone(),
                )
                .expect("valid parameter"),

                ForeignParameter::new(
                    "right",
                    integer.clone(),
                )
                .expect("valid parameter"),
            ],
            Some(integer),
        )
        .expect("valid function");

        contract
            .add_function(function)
            .expect("function should be registered");

        contract
    }

    #[test]
    fn adapts_valid_foreign_abi() {

        let source = source_contract();

        let adapter = InteropAbiAdapter::new();

        let adapted = adapter
            .adapt(
                &source,
                version(),
                &TestResolver,
            )
            .expect("adaptation should succeed");

        assert_eq!(
            adapted.runtime_abi().entry_points().len(),
            1,
        );

        assert_eq!(
            adapted.runtime_abi()
                .entry_points()[0]
                .symbol()
                .as_str(),
            "foreign_add",
        );
    }

    #[test]
    fn preserves_original_source_contract() {

        let source = source_contract();

        let adapted = InteropAbiAdapter::new()
            .adapt(
                &source,
                version(),
                &TestResolver,
            )
            .expect("adaptation should succeed");

        assert_eq!(
            adapted.source_contract().name(),
            source.name(),
        );

        assert_eq!(
            adapted.source_contract().functions().len(),
            1,
        );
    }

    #[test]
    fn preserves_parameter_order() {

        let source = source_contract();

        let adapted = InteropAbiAdapter::new()
            .adapt(
                &source,
                version(),
                &TestResolver,
            )
            .expect("adaptation should succeed");

        let parameters = adapted.runtime_abi()
            .entry_points()[0]
            .parameters();

        assert_eq!(
            parameters[0].name().as_str(),
            "left",
        );

        assert_eq!(
            parameters[1].name().as_str(),
            "right",
        );
    }

    #[test]
    fn rejects_data_only_contracts() {

        let profile = AbiProfile::new(
            "portable",
            "c",
            AbiLinkage::Host,
        )
        .expect("valid profile");

        let source = AbiContract::new(
            "test.foreign_data",
            AbiBoundaryKind::Data,
            profile,
        )
        .expect("valid data contract");

        let result = InteropAbiAdapter::new()
            .adapt(
                &source,
                version(),
                &TestResolver,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_callable_contracts() {

        let profile = AbiProfile::new(
            "portable",
            "c",
            AbiLinkage::Host,
        )
        .expect("valid profile");

        let source = AbiContract::new(
            "test.empty",
            AbiBoundaryKind::Callable,
            profile,
        )
        .expect("valid contract");

        let result = InteropAbiAdapter::new()
            .adapt(
                &source,
                version(),
                &TestResolver,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_variadic_functions() {

        let profile = AbiProfile::new(
            "portable",
            "c",
            AbiLinkage::Host,
        )
        .expect("valid profile");

        let mut source = AbiContract::new(
            "test.variadic",
            AbiBoundaryKind::Callable,
            profile,
        )
        .expect("valid contract");

        let function = AbiFunction::new(
            "printf_like",
            "foreign_printf_like",
            Vec::new(),
            None,
        )
        .expect("valid function")
        .with_variadic(true);

        source
            .add_function(function)
            .expect("function registration");

        let result = InteropAbiAdapter::new()
            .adapt(
                &source,
                version(),
                &TestResolver,
            );

        assert!(result.is_err());
    }

    #[test]
    fn preserves_runtime_abi_version() {

        let source = source_contract();

        let expected = version();

        let adapted = InteropAbiAdapter::new()
            .adapt(
                &source,
                expected.clone(),
                &TestResolver,
            )
            .expect("adaptation should succeed");

        assert_eq!(
            adapted.version(),
            &expected,
        );
    }

    #[test]
    fn adapter_has_no_global_state() {

        let first = InteropAbiAdapter::new();
        let second = InteropAbiAdapter::new();

        assert_eq!(first, second);
    }
}
