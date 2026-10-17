
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Foreign Function Call Contracts.
//!
//! Path: src/classical/interoperability/foreign_calls.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Responsibility
//!
//! This module binds validated ABI function declarations to call sites and
//! adapts them to the canonical external-call IR. It does not parse source,
//! resolve Zamani types, load libraries, link binaries, or execute calls.
//!
//! # Canonical ownership
//!
//! - `abi.rs` owns ABI declarations and ABI validation.
//! - The frontend owns syntax, source spans, and name resolution.
//! - Semantic analysis owns type compatibility and effect checking.
//! - `quantum::ir::classical::extern_call` owns the canonical call IR.
//! - Target lowering owns concrete ABI realization.
//! - The runtime owns authorization, symbol resolution, and execution.
//!
//! # Portability and safety
//!
//! No universal argument, result, function, or resource limits are imposed.
//! Actual limits come from explicit policies, available resources, and target
//! capabilities. No foreign function is executed by this module.
//!
//! ABI properties that cannot be faithfully represented by the existing IR
//! must be handled by an explicit adapter; this module never silently claims
//! that an unsupported ABI has been preserved.

use std::collections::BTreeMap;
use std::fmt;

use crate::classical::interoperability::abi::{
    AbiContract,
    AbiError,
    AbiFunction,
    AbiIdentifier,
    AbiLinkage,
    AbiProfile,
};

use crate::quantum::ir::classical::extern_call::{
    ExternalAbi,
    ExternalCall,
    ExternalCallArgument,
    ExternalCallError,
    ExternalCallResultTarget,
    ExternalCapabilityRequirement,
    ExternalEffect,
    ExternalExecutionProperties,
    ExternalLinkage,
    ExternalNamespace,
    ExternalSymbol,
};

use crate::quantum::ir::identity::{
    CapabilityId,
    OperationId,
};

/// Result type for foreign-call declaration and IR adaptation.
pub type ForeignCallResult<T> = Result<T, ForeignCallError>;

/// Errors produced while validating or adapting foreign calls.
///
/// Error categories are stable API. Human-readable messages are explanatory
/// and must not be used for programmatic control flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForeignCallError {
    /// The ABI contract or function declaration is invalid.
    InvalidContract {
        message: String,
    },

    /// No function with the requested declaration name exists.
    FunctionNotFound {
        name: String,
    },

    /// A call has fewer or more arguments than its signature permits.
    ArgumentCountMismatch {
        function: String,
        expected: usize,
        actual: usize,
        variadic: bool,
    },

    /// A call has the wrong number of result destinations.
    ResultCountMismatch {
        function: String,
        expected: usize,
        actual: usize,
    },

    /// A required ABI capability has no explicit canonical IR mapping.
    MissingCapabilityMapping {
        function: String,
        capability: String,
    },

    /// A declaration contains an effect the adapter cannot safely classify.
    InvalidEffectContract {
        function: String,
        message: String,
    },

    /// An ABI feature cannot be faithfully represented by the chosen adapter.
    UnsupportedAbiMapping {
        function: String,
        feature: String,
    },

    /// The canonical IR rejected the constructed external call.
    IrValidationFailed {
        message: String,
    },
}

impl ForeignCallError {
    /// Returns a stable machine-readable error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidContract { .. } => "foreign_call.invalid_contract",
            Self::FunctionNotFound { .. } => "foreign_call.function_not_found",
            Self::ArgumentCountMismatch { .. } => {
                "foreign_call.argument_count_mismatch"
            }
            Self::ResultCountMismatch { .. } => {
                "foreign_call.result_count_mismatch"
            }
            Self::MissingCapabilityMapping { .. } => {
                "foreign_call.missing_capability_mapping"
            }
            Self::InvalidEffectContract { .. } => {
                "foreign_call.invalid_effect_contract"
            }
            Self::UnsupportedAbiMapping { .. } => {
                "foreign_call.unsupported_abi_mapping"
            }
            Self::IrValidationFailed { .. } => {
                "foreign_call.ir_validation_failed"
            }
        }
    }

    /// Converts the error to the shared classical error boundary.
    #[must_use]
    pub fn into_classical_error(self) -> crate::classical::error::ClassicalError {
        use crate::classical::error::ClassicalError;

        match self {
            Self::InvalidContract { message }
            | Self::FunctionNotFound { name: message }
            | Self::ArgumentCountMismatch {
                function: message,
                ..
            }
            | Self::ResultCountMismatch {
                function: message,
                ..
            }
            | Self::MissingCapabilityMapping {
                function: message,
                ..
            }
            | Self::InvalidEffectContract {
                function: message,
                ..
            }
            | Self::UnsupportedAbiMapping {
                function: message,
                ..
            }
            | Self::IrValidationFailed { message } => {
                ClassicalError::validation_failed(format!(
                    "{}: {}",
                    self.code(),
                    message
                ))
            }
        }
    }
}

impl fmt::Display for ForeignCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidContract { message } => {
                write!(f, "invalid foreign-call contract: {message}")
            }
            Self::FunctionNotFound { name } => {
                write!(f, "foreign function not found: {name}")
            }
            Self::ArgumentCountMismatch {
                function,
                expected,
                actual,
                variadic,
            } => {
                write!(
                    f,
                    "foreign function {function} received {actual} arguments; \
                     expected {}{}",
                    if *variadic { "at least " } else { "" },
                    expected
                )
            }
            Self::ResultCountMismatch {
                function,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "foreign function {function} expects {expected} result \
                     destinations, received {actual}"
                )
            }
            Self::MissingCapabilityMapping {
                function,
                capability,
            } => {
                write!(
                    f,
                    "foreign function {function} requires capability \
                     {capability}, but no canonical IR mapping was supplied"
                )
            }
            Self::InvalidEffectContract { function, message } => {
                write!(f, "invalid effects for {function}: {message}")
            }
            Self::UnsupportedAbiMapping { function, feature } => {
                write!(
                    f,
                    "cannot faithfully adapt ABI feature {feature} for {function}"
                )
            }
            Self::IrValidationFailed { message } => {
                write!(f, "canonical external-call IR validation failed: {message}")
            }
        }
    }
}

impl std::error::Error for ForeignCallError {}

impl From<AbiError> for ForeignCallError {
    fn from(error: AbiError) -> Self {
        Self::InvalidContract {
            message: error.to_string(),
        }
    }
}

impl From<ExternalCallError> for ForeignCallError {
    fn from(error: ExternalCallError) -> Self {
        Self::IrValidationFailed {
            message: error.to_string(),
        }
    }
}

/// A validated view of one function inside a canonical ABI contract.
///
/// This is a borrowed declaration, not a second function-signature registry.
/// The source contract remains the owner of the declaration.
#[derive(Debug, Clone, Copy)]
pub struct ForeignFunctionDeclaration<'a> {
    contract: &'a AbiContract,
    profile: &'a AbiProfile,
    function: &'a AbiFunction,
}

impl<'a> ForeignFunctionDeclaration<'a> {
    /// Finds and validates a callable declaration by its ABI function name.
    ///
    /// Contract validation is performed before exposing the binding.
    pub fn resolve(
        contract: &'a AbiContract,
        function_name: &str,
    ) -> ForeignCallResult<Self> {
        contract.validate()?;

        let function = contract
            .functions()
            .iter()
            .find(|candidate| candidate.name() == function_name)
            .ok_or_else(|| ForeignCallError::FunctionNotFound {
                name: function_name.to_owned(),
            })?;

        function.validate()?;

        Ok(Self {
            contract,
            profile: contract.profile(),
            function,
        })
    }

    /// Returns the containing ABI contract.
    #[must_use]
    pub const fn contract(&self) -> &'a AbiContract {
        self.contract
    }

    /// Returns the selected ABI profile.
    #[must_use]
    pub const fn profile(&self) -> &'a AbiProfile {
        self.profile
    }

    /// Returns the validated function declaration.
    #[must_use]
    pub const fn function(&self) -> &'a AbiFunction {
        self.function
    }

    /// Returns the source-level function name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.function.name()
    }

    /// Returns the external symbol spelling.
    #[must_use]
    pub fn symbol(&self) -> &str {
        self.function.symbol()
    }

    /// Validates a call's argument and result counts.
    ///
    /// This does not replace semantic type checking: the frontend or semantic
    /// analyzer must also prove that each operand has a compatible boundary
    /// type and representation.
    pub fn validate_arity(
        &self,
        argument_count: usize,
        result_count: usize,
    ) -> ForeignCallResult<()> {
        let expected_arguments = self.function.parameters().len();
        let variadic = self.function.is_variadic();

        let arguments_valid = if variadic {
            argument_count >= expected_arguments
        } else {
            argument_count == expected_arguments
        };

        if !arguments_valid {
            return Err(ForeignCallError::ArgumentCountMismatch {
                function: self.name().to_owned(),
                expected: expected_arguments,
                actual: argument_count,
                variadic,
            });
        }

        // The current canonical external-call model represents one result
        // destination per returned value. An absent ABI result means zero;
        // a declared result means one destination for the aggregate/value.
        let expected_results = usize::from(self.function.result().is_some());

        if result_count != expected_results {
            return Err(ForeignCallError::ResultCountMismatch {
                function: self.name().to_owned(),
                expected: expected_results,
                actual: result_count,
            });
        }

        Ok(())
    }

    /// Validates the declared effect vocabulary before call construction.
    ///
    /// Empty or unknown effect sets remain conservative. This adapter does
    /// not infer purity from a function name, library, or target.
    pub fn validate_effects(&self) -> ForeignCallResult<()> {
        for effect in self.function.effects() {
            if effect.as_str().trim().is_empty() {
                return Err(ForeignCallError::InvalidEffectContract {
                    function: self.name().to_owned(),
                    message: "effect identifiers must not be empty".to_owned(),
                });
            }
        }

        Ok(())
    }

    /// Builds a canonical external-call IR operation.
    ///
    /// `linkage` and `namespace` are explicit inputs because the canonical IR
    /// and the ABI contract do not necessarily express identical linkage
    /// distinctions. The caller must supply a mapping that preserves the
    /// selected target's linkage semantics.
    ///
    /// `capability_ids` maps ABI capability names to the canonical IR's
    /// capability identities. Every declared capability must be mapped:
    /// none is silently discarded.
    ///
    /// This function constructs IR only. It does not resolve a symbol or call
    /// foreign code.
    pub fn build_ir_call(
        &self,
        operation_id: OperationId,
        arguments: impl IntoIterator<Item = ExternalCallArgument>,
        results: impl IntoIterator<Item = ExternalCallResultTarget>,
        namespace: Option<ExternalNamespace>,
        linkage: ExternalLinkage,
        capability_ids: &BTreeMap<String, CapabilityId>,
    ) -> ForeignCallResult<ExternalCall> {
        self.validate_effects()?;

        let arguments: Vec<_> = arguments.into_iter().collect();
        let results: Vec<_> = results.into_iter().collect();

        self.validate_arity(arguments.len(), results.len())?;

        let symbol = ExternalSymbol::new(self.function.symbol())?;

        let mut call = ExternalCall::new(operation_id, symbol)?
            .with_linkage(linkage);

        if let Some(namespace) = namespace {
            call = call.with_namespace(namespace);
        }

        call = call.with_abi(self.external_abi());

        for argument in arguments {
            call.push_argument(argument);
        }

        for result in results {
            call.push_result(result)?;
        }

        for required in self.function.required_capabilities() {
            let name = required.as_str();

            let capability = capability_ids.get(name).copied().ok_or_else(|| {
                ForeignCallError::MissingCapabilityMapping {
                    function: self.name().to_owned(),
                    capability: name.to_owned(),
                }
            })?;

            call.push_capability(ExternalCapabilityRequirement::required(
                capability,
            ))?;
        }

        call = call.with_execution(self.execution_properties());

        call.validate()?;

        Ok(call)
    }

    /// Creates ABI metadata representable by the canonical external-call IR.
    ///
    /// ABI linkage is intentionally not translated here: the canonical IR
    /// distinguishes linkage categories differently. The caller supplies the
    /// corresponding explicit `ExternalLinkage` to `build_ir_call`.
    pub fn external_abi(&self) -> ExternalAbi {
        let mut abi = ExternalAbi::new();

        let convention = self.profile.calling_convention();

        if !convention.trim().is_empty() {
            // The ABI profile has already validated its identifier.
            if let Ok(convention) = ExternalSymbol::new(convention) {
                abi = abi.with_calling_convention(convention);
            }
        }

        if let Some(version) = self.profile.version() {
            abi = abi.with_version(version.as_str());
        }

        abi
    }

    /// Converts a declared effect set to conservative canonical IR properties.
    ///
    /// A call is considered pure only when its declaration explicitly states
    /// exactly one recognized pure effect. Unknown or combined effects never
    /// become pure by default.
    #[must_use]
    pub fn execution_properties(&self) -> ExternalExecutionProperties {
        let effects = self.function.effects();

        if effects.len() == 1 {
            let effect = effects.iter().next().map(AbiIdentifier::as_str);

            match effect {
                Some("pure") => return ExternalExecutionProperties::pure(),
                Some("read_only") | Some("readonly") => {
                    return ExternalExecutionProperties::conservative()
                        .with_effect(ExternalEffect::ReadOnly);
                }
                Some("stateful") => {
                    return ExternalExecutionProperties::conservative()
                        .with_effect(ExternalEffect::Stateful);
                }
                Some("quantum_state") => {
                    return ExternalExecutionProperties::conservative()
                        .with_effect(ExternalEffect::QuantumState);
                }
                Some("hybrid_state") => {
                    return ExternalExecutionProperties::conservative()
                        .with_effect(ExternalEffect::HybridState);
                }
                Some("nondeterministic") => {
                    return ExternalExecutionProperties::conservative()
                        .with_effect(ExternalEffect::Nondeterministic);
                }
                _ => {}
            }
        }

        ExternalExecutionProperties::conservative()
            .with_effect(ExternalEffect::Unknown)
    }

    /// Checks whether the declared ABI linkage has an explicit adapter.
    ///
    /// This deliberately does not equate static and dynamic linking: the
    /// canonical IR currently has no direct static-vs-dynamic distinction.
    pub fn validate_linkage_mapping(
        &self,
        mapping: &ExternalLinkage,
    ) -> ForeignCallResult<()> {
        match self.profile.linkage() {
            AbiLinkage::Static | AbiLinkage::Dynamic => {
                // A plain library/default linkage does not by itself preserve
                // static-vs-dynamic semantics. Require an explicit opaque
                // mapping for a backend-defined, semantics-preserving adapter.
                if !matches!(mapping, ExternalLinkage::Opaque { .. }) {
                    return Err(ForeignCallError::UnsupportedAbiMapping {
                        function: self.name().to_owned(),
                        feature: match self.profile.linkage() {
                            AbiLinkage::Static => "static linkage",
                            AbiLinkage::Dynamic => "dynamic linkage",
                            _ => unreachable!(),
                        }
                        .to_owned(),
                    });
                }
            }
            AbiLinkage::Host => {
                if !matches!(
                    mapping,
                    ExternalLinkage::Default | ExternalLinkage::Runtime { .. }
                ) {
                    return Err(ForeignCallError::UnsupportedAbiMapping {
                        function: self.name().to_owned(),
                        feature: "host linkage mapping".to_owned(),
                    });
                }
            }
            AbiLinkage::Provider(provider) => {
                if !matches!(
                    mapping,
                    ExternalLinkage::Library { name }
                        if name.as_str() == provider.as_str()
                ) {
                    return Err(ForeignCallError::UnsupportedAbiMapping {
                        function: self.name().to_owned(),
                        feature: "provider linkage mapping".to_owned(),
                    });
                }
            }
            AbiLinkage::Custom(identifier) => {
                if !matches!(
                    mapping,
                    ExternalLinkage::Opaque { name }
                        if name.as_str() == identifier.as_str()
                ) {
                    return Err(ForeignCallError::UnsupportedAbiMapping {
                        function: self.name().to_owned(),
                        feature: "custom linkage mapping".to_owned(),
                    });
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests constructing ABI declarations should live beside the
    // ABI module once its public constructors and test fixtures are frozen.
    //
    // The repository-level test suite must additionally verify:
    // - exact and variadic argument arity;
    // - zero-result and one-result signatures;
    // - missing capability mappings;
    // - effect conservatism for unknown and combined effects;
    // - all linkage variants and rejected lossy mappings;
    // - canonical ExternalCall validation;
    // - very large dynamically supplied argument collections subject to
    //   explicit test budgets rather than language-level ceilings.

    #[test]
    fn unknown_effects_are_not_assumed_pure() {
        // This invariant is exercised by declaration-level integration tests
        // after the ABI fixture constructors are wired into this module.
        assert_eq!(
            ExternalEffect::Unknown.may_have_side_effects(),
            true
        );
    }
}
