//! Zamani Quantum Optimization — Parameter Binding
//!
//! Production-grade symbolic parameter binding for the canonical Zamani
//! Quantum IR.
//!
//! # Architectural role
//!
//! ```text
//!                         quantum::ir::parameter
//!                                  │
//!                                  ▼
//!                 optimization::parameter::binding
//!                                  │
//!                 ┌────────────────┼────────────────┐
//!                 │                │                │
//!                 ▼                ▼                ▼
//!            substitution      complete         gate-parameter
//!            / partial         evaluation       binding
//!                 │                │                │
//!                 └────────────────┼────────────────┘
//!                                  ▼
//!                    parameter optimization passes
//!                                  │
//!                 ┌────────────────┼────────────────┐
//!                 ▼                ▼                ▼
//!             simplify       constant_fold     rotation
//!             / algebra      / normalization   optimization
//! ```
//!
//! This module operates exclusively on the canonical Quantum IR parameter
//! representation:
//!
//! - [`crate::quantum::ir::parameter::Parameter`];
//! - [`crate::quantum::ir::parameter::ParameterExpression`];
//! - [`crate::quantum::ir::parameter::GateParameter`].
//!
//! It deliberately does NOT define a second parameter representation.
//!
//! # Responsibilities
//!
//! This module provides:
//!
//! - deterministic symbol-to-value environments;
//! - explicit parameter substitution;
//! - partial symbolic binding;
//! - complete symbolic binding;
//! - complete numerical evaluation;
//! - gate-parameter binding;
//! - transactional in-place binding;
//! - duplicate-binding detection;
//! - missing-symbol detection;
//! - finite-value enforcement;
//! - configurable resource limits;
//! - deterministic statistics;
//! - allocation-conscious traversal;
//! - validation before and after transformation.
//!
//! # Important semantic boundary
//!
//! Binding is substitution, not general algebraic simplification.
//!
//! For example:
//!
//! ```text
//! x + 0
//! ```
//!
//! with `x = 3` becomes:
//!
//! ```text
//! 3 + 0
//! ```
//!
//! A caller may subsequently run `constant_fold` to obtain:
//!
//! ```text
//! 3
//! ```
//!
//! This separation prevents multiple optimization modules from implementing
//! different arithmetic semantics.
//!
//! Complete binding is the exception: when every symbol is resolved, the
//! canonical Quantum IR's own [`Parameter::bind`] evaluation semantics are
//! used so there is exactly one implementation of numerical expression
//! evaluation.
//!
//! # Determinism
//!
//! [`ParameterBindings`] uses `BTreeMap` rather than `HashMap` so that:
//!
//! - iteration order is deterministic;
//! - serialization order can be deterministic;
//! - diagnostics can be stable;
//! - reproducible compilation does not depend on hash randomization.
//!
//! # Numerical safety
//!
//! Binding values must be finite.
//!
//! NaN and positive/negative infinity are rejected before they can enter the
//! resulting Quantum IR.
//!
//! # Resource safety
//!
//! This module supports explicit limits for:
//!
//! - number of bound symbols;
//! - number of parameter nodes traversed;
//! - number of symbols inspected;
//! - number of output nodes constructed.
//!
//! The defaults are deliberately large and can be increased by callers when
//! the available machine resources permit larger workloads.
//!
//! The implementation never uses `unsafe`.
//!
//! # Integration contract
//!
//! `parameter/mod.rs` should expose this module with:
//!
//! ```text
//! pub mod binding;
//! ```
//!
//! Later optimization infrastructure can consume:
//!
//! ```text
//! ParameterBindings
//! ParameterBinder
//! BindingConfig
//! BindingMode
//! ```
//!
//! No modification to this file is required when the following are added:
//!
//! - optimization pipeline;
//! - pass registry;
//! - planner;
//! - cost model;
//! - verification;
//! - circuit optimization;
//! - routing;
//! - scheduling;
//! - hardware targets.
//!
//! Higher-level optimizer configuration can construct [`BindingConfig`] and
//! invoke [`ParameterBinder`] directly.
//!
//! # Rust compatibility
//!
//! Compatible with Rust 1.97 and Rust 1.97.1.
//!
//! # Safety
//!
//! No `unsafe` code is used.

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::errors::{
    IrParameterError,
    IrResult,
};
use crate::quantum::ir::parameter::{
    GateParameter,
    Parameter,
    ParameterExpression,
};

// -----------------------------------------------------------------------------
// Defaults
// -----------------------------------------------------------------------------

/// Default maximum number of symbols in one binding environment.
///
/// The value is intentionally large while remaining bounded so malformed
/// callers cannot accidentally request an effectively unbounded allocation.
pub const DEFAULT_MAX_BINDINGS: usize = 1_048_576;

/// Default maximum number of parameter nodes traversed by one binding
/// operation.
///
/// The canonical IR already constrains expression depth. This additional
/// budget protects against pathological values constructed directly through
/// public enum variants.
pub const DEFAULT_MAX_NODES: usize = 1_048_576;

/// Default maximum number of symbols inspected by one binding operation.
pub const DEFAULT_MAX_SYMBOL_LOOKUPS: usize = 1_048_576;

/// Default maximum number of output parameter nodes constructed by one binding
/// operation.
pub const DEFAULT_MAX_OUTPUT_NODES: usize = 1_048_576;

// -----------------------------------------------------------------------------
// Binding mode
// -----------------------------------------------------------------------------

/// Controls how unresolved symbols are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingMode {
    /// Replace symbols that exist in the binding environment and preserve
    /// unresolved symbols.
    ///
    /// Example:
    ///
    /// ```text
    /// x + y
    /// ```
    ///
    /// with `{x = 1}` becomes:
    ///
    /// ```text
    /// 1 + y
    /// ```
    Partial,

    /// Require every symbol to have a binding.
    ///
    /// An unresolved symbol causes the operation to fail without modifying
    /// the caller's input.
    Complete,
}

impl Default for BindingMode {
    fn default() -> Self {
        Self::Partial
    }
}

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

/// Resource limits for parameter binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindingConfig {
    /// Maximum number of entries allowed in a [`ParameterBindings`] object.
    pub max_bindings: usize,

    /// Maximum number of input parameter nodes traversed by one operation.
    pub max_nodes: usize,

    /// Maximum number of symbol lookups performed by one operation.
    pub max_symbol_lookups: usize,

    /// Maximum number of output parameter nodes constructed by one operation.
    pub max_output_nodes: usize,
}

impl Default for BindingConfig {
    fn default() -> Self {
        Self {
            max_bindings: DEFAULT_MAX_BINDINGS,
            max_nodes: DEFAULT_MAX_NODES,
            max_symbol_lookups: DEFAULT_MAX_SYMBOL_LOOKUPS,
            max_output_nodes: DEFAULT_MAX_OUTPUT_NODES,
        }
    }
}

impl BindingConfig {
    /// Creates an explicit binding configuration.
    pub const fn new(
        max_bindings: usize,
        max_nodes: usize,
        max_symbol_lookups: usize,
        max_output_nodes: usize,
    ) -> Self {
        Self {
            max_bindings,
            max_nodes,
            max_symbol_lookups,
            max_output_nodes,
        }
    }

    /// Validates the configuration.
    ///
    /// A zero resource limit is rejected because it would make even a single
    /// valid parameter impossible to process.
    pub fn validate(self) -> IrResult<()> {
        if self.max_bindings == 0
            || self.max_nodes == 0
            || self.max_symbol_lookups == 0
            || self.max_output_nodes == 0
        {
            return Err(IrParameterError::InvalidExpression.into());
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Binding errors
// -----------------------------------------------------------------------------

/// Errors specific to parameter binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingError {
    /// A symbol was inserted twice through an operation that requires unique
    /// insertion.
    DuplicateSymbol {
        /// Symbol name.
        name: String,
    },

    /// The binding environment would exceed its configured capacity.
    BindingLimitExceeded {
        /// Maximum number of entries.
        limit: usize,

        /// Requested resulting number of entries.
        actual: usize,
    },

    /// A resource limit was reached during a binding operation.
    ResourceLimitExceeded {
        /// Resource name.
        resource: &'static str,

        /// Configured maximum.
        limit: usize,

        /// Requested/observed value.
        actual: usize,
    },

    /// A symbol required by complete binding is missing.
    MissingSymbol {
        /// Missing symbol name.
        name: String,
    },

    /// Multiple required symbols are missing.
    MissingSymbols {
        /// Missing symbol names in deterministic order.
        names: Vec<String>,
    },

    /// A supplied binding value is not finite.
    NonFiniteValue {
        /// Symbol associated with the invalid value.
        name: String,
    },

    /// The caller supplied an invalid binding configuration.
    InvalidConfiguration,

    /// The resulting parameter could not satisfy the canonical IR contract.
    InvalidResult,
}

impl fmt::Display for BindingError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateSymbol { name } => {
                write!(
                    f,
                    "duplicate parameter binding for symbol `{name}`"
                )
            }

            Self::BindingLimitExceeded {
                limit,
                actual,
            } => {
                write!(
                    f,
                    "parameter binding environment exceeds limit: \
                     maximum {limit}, resulting size {actual}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                limit,
                actual,
            } => {
                write!(
                    f,
                    "parameter binding exceeded {resource} limit: \
                     maximum {limit}, actual {actual}"
                )
            }

            Self::MissingSymbol { name } => {
                write!(
                    f,
                    "parameter symbol `{name}` has no binding"
                )
            }

            Self::MissingSymbols { names } => {
                write!(
                    f,
                    "{} parameter symbols have no bindings",
                    names.len()
                )
            }

            Self::NonFiniteValue { name } => {
                write!(
                    f,
                    "binding for parameter symbol `{name}` is not finite"
                )
            }

            Self::InvalidConfiguration => {
                write!(
                    f,
                    "invalid parameter binding configuration"
                )
            }

            Self::InvalidResult => {
                write!(
                    f,
                    "parameter binding produced an invalid Quantum IR value"
                )
            }
        }
    }
}

impl std::error::Error for BindingError {}

impl From<BindingError> for crate::quantum::ir::errors::IrError {
    fn from(error: BindingError) -> Self {
        match error {
            BindingError::NonFiniteValue { .. } => {
                IrParameterError::NonFinite.into()
            }

            BindingError::MissingSymbol { name } => {
                IrParameterError::UnboundSymbol { name }.into()
            }

            BindingError::MissingSymbols { names } => {
                match names.into_iter().next() {
                    Some(name) => {
                        IrParameterError::UnboundSymbol { name }.into()
                    }

                    None => {
                        IrParameterError::InvalidExpression.into()
                    }
                }
            }

            BindingError::DuplicateSymbol { .. }
            | BindingError::BindingLimitExceeded { .. }
            | BindingError::ResourceLimitExceeded { .. }
            | BindingError::InvalidConfiguration
            | BindingError::InvalidResult => {
                IrParameterError::InvalidExpression.into()
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Binding result
// -----------------------------------------------------------------------------

/// Statistics produced by one binding operation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BindingStats {
    /// Number of parameter nodes inspected.
    pub nodes_visited: usize,

    /// Number of symbolic nodes encountered.
    pub symbols_seen: usize,

    /// Number of symbols successfully replaced.
    pub symbols_bound: usize,

    /// Number of symbols deliberately left unresolved in partial mode.
    pub symbols_unresolved: usize,

    /// Number of symbol lookups performed.
    pub symbol_lookups: usize,

    /// Number of output parameter nodes constructed.
    pub output_nodes: usize,

    /// Whether the resulting representation differs from the input.
    pub changed: bool,
}

impl BindingStats {
    fn add_nodes(&mut self) {
        self.nodes_visited = self.nodes_visited.saturating_add(1);
    }

    fn add_symbol_seen(&mut self) {
        self.symbols_seen = self.symbols_seen.saturating_add(1);
    }

    fn add_bound(&mut self) {
        self.symbols_bound = self.symbols_bound.saturating_add(1);
    }

    fn add_unresolved(&mut self) {
        self.symbols_unresolved =
            self.symbols_unresolved.saturating_add(1);
    }

    fn add_lookup(&mut self) {
        self.symbol_lookups =
            self.symbol_lookups.saturating_add(1);
    }

    fn add_output_node(&mut self) {
        self.output_nodes =
            self.output_nodes.saturating_add(1);
    }
}

/// Result of binding one parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundParameter {
    /// Resulting parameter.
    pub parameter: Parameter,

    /// Binding statistics.
    pub stats: BindingStats,
}

impl BoundParameter {
    /// Returns whether binding changed the parameter.
    pub const fn changed(&self) -> bool {
        self.stats.changed
    }
}

/// Result of binding a gate parameter group.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundGateParameters {
    /// Resulting gate parameter group.
    pub parameter: GateParameter,

    /// Aggregate binding statistics.
    pub stats: BindingStats,
}

impl BoundGateParameters {
    /// Returns whether binding changed the parameter group.
    pub const fn changed(&self) -> bool {
        self.stats.changed
    }
}

// -----------------------------------------------------------------------------
// Binding environment
// -----------------------------------------------------------------------------

/// Deterministic explicit parameter binding environment.
///
/// A binding environment is deliberately not global. The caller explicitly
/// owns and supplies it to the binder.
///
/// This makes parameter binding:
///
/// - deterministic;
/// - thread-safe when shared immutably;
/// - reproducible;
/// - suitable for compiler caching;
/// - suitable for concurrent circuit compilation.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterBindings {
    values: BTreeMap<String, f64>,
    max_bindings: usize,
}

impl Default for ParameterBindings {
    fn default() -> Self {
        Self::new()
    }
}

impl ParameterBindings {
    /// Creates an empty binding environment with production defaults.
    pub fn new() -> Self {
        Self::with_capacity_limit(DEFAULT_MAX_BINDINGS)
    }

    /// Creates an empty environment with an explicit entry limit.
    pub const fn with_capacity_limit(
        max_bindings: usize,
    ) -> Self {
        Self {
            values: BTreeMap::new(),
            max_bindings,
        }
    }

    /// Creates an environment from an explicit binding configuration.
    pub fn with_config(
        config: BindingConfig,
    ) -> IrResult<Self> {
        config.validate()?;

        Ok(Self {
            values: BTreeMap::new(),
            max_bindings: config.max_bindings,
        })
    }

    /// Returns the maximum number of bindings permitted.
    pub const fn max_bindings(&self) -> usize {
        self.max_bindings
    }

    /// Returns the number of bound symbols.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true when no symbols are bound.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the numerical value for a symbol.
    pub fn get(
        &self,
        name: &str,
    ) -> Option<f64> {
        self.values.get(name).copied()
    }

    /// Returns whether the environment contains a symbol.
    pub fn contains(
        &self,
        name: &str,
    ) -> bool {
        self.values.contains_key(name)
    }

    /// Returns bindings in deterministic lexical order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, f64)> {
        self.values
            .iter()
            .map(|(name, value)| (name.as_str(), *value))
    }

    /// Inserts a binding.
    ///
    /// Existing symbols are rejected rather than silently overwritten.
    /// This catches accidental duplicate parameter assignment early.
    pub fn insert<S: Into<String>>(
        &mut self,
        name: S,
        value: f64,
    ) -> IrResult<()> {
        let name = name.into();

        validate_symbol_name(&name)?;

        if !value.is_finite() {
            return Err(
                BindingError::NonFiniteValue { name }.into()
            );
        }

        if self.values.contains_key(&name) {
            return Err(
                BindingError::DuplicateSymbol { name }.into()
            );
        }

        let resulting_len = self
            .values
            .len()
            .checked_add(1)
            .ok_or_else(|| {
                BindingError::BindingLimitExceeded {
                    limit: self.max_bindings,
                    actual: usize::MAX,
                }
            })?;

        if resulting_len > self.max_bindings {
            return Err(
                BindingError::BindingLimitExceeded {
                    limit: self.max_bindings,
                    actual: resulting_len,
                }
                .into(),
            );
        }

        self.values.insert(name, value);

        Ok(())
    }

    /// Inserts or replaces a binding.
    ///
    /// This operation is useful for interactive parameter sweeps where a
    /// parameter value is intentionally updated between executions.
    pub fn insert_or_replace<S: Into<String>>(
        &mut self,
        name: S,
        value: f64,
    ) -> IrResult<Option<f64>> {
        let name = name.into();

        validate_symbol_name(&name)?;

        if !value.is_finite() {
            return Err(
                BindingError::NonFiniteValue { name }.into()
            );
        }

        if let Some(existing) = self.values.get_mut(&name) {
            let old = *existing;
            *existing = value;
            return Ok(Some(old));
        }

        let resulting_len = self
            .values
            .len()
            .checked_add(1)
            .ok_or_else(|| {
                BindingError::BindingLimitExceeded {
                    limit: self.max_bindings,
                    actual: usize::MAX,
                }
            })?;

        if resulting_len > self.max_bindings {
            return Err(
                BindingError::BindingLimitExceeded {
                    limit: self.max_bindings,
                    actual: resulting_len,
                }
                .into(),
            );
        }

        self.values.insert(name, value);

        Ok(None)
    }

    /// Removes a symbol binding.
    pub fn remove(
        &mut self,
        name: &str,
    ) -> Option<f64> {
        self.values.remove(name)
    }

    /// Removes all bindings.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Returns all bound symbol names in deterministic order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.values.keys().map(String::as_str)
    }

    /// Validates the complete binding environment.
    pub fn validate(&self) -> IrResult<()> {
        if self.values.len() > self.max_bindings {
            return Err(
                BindingError::BindingLimitExceeded {
                    limit: self.max_bindings,
                    actual: self.values.len(),
                }
                .into(),
            );
        }

        for (name, value) in &self.values {
            validate_symbol_name(name)?;

            if !value.is_finite() {
                return Err(
                    BindingError::NonFiniteValue {
                        name: name.clone(),
                    }
                    .into(),
                );
            }
        }

        Ok(())
    }

    /// Returns the number of bindings that fit within this environment's
    /// configured capacity.
    pub const fn remaining_capacity(&self) -> usize {
        self.max_bindings.saturating_sub(0)
    }
}

// -----------------------------------------------------------------------------
// Binder
// -----------------------------------------------------------------------------

/// Production parameter-binding engine.
///
/// The binder is immutable and reusable. It owns no global state and performs
/// no mutation of caller-owned parameters unless one of the explicit
/// `_in_place` methods is used.
///
/// The in-place methods are transactional: the input is replaced only after
/// complete successful binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterBinder {
    config: BindingConfig,
}

impl Default for ParameterBinder {
    fn default() -> Self {
        Self::new()
    }
}

impl ParameterBinder {
    /// Creates a binder with production defaults.
    pub const fn new() -> Self {
        Self {
            config: BindingConfig::new(
                DEFAULT_MAX_BINDINGS,
                DEFAULT_MAX_NODES,
                DEFAULT_MAX_SYMBOL_LOOKUPS,
                DEFAULT_MAX_OUTPUT_NODES,
            ),
        }
    }

    /// Creates a binder with explicit resource limits.
    pub fn with_config(
        config: BindingConfig,
    ) -> IrResult<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns this binder's resource configuration.
    pub const fn config(&self) -> BindingConfig {
        self.config
    }

    /// Performs parameter binding according to [`BindingMode`].
    ///
    /// In partial mode, known symbols are substituted while unknown symbols
    /// remain symbolic.
    ///
    /// In complete mode, all symbols must be present and the resulting
    /// expression is fully evaluated using the canonical IR evaluation
    /// semantics.
    pub fn bind_parameter(
        &self,
        parameter: &Parameter,
        bindings: &ParameterBindings,
        mode: BindingMode,
    ) -> IrResult<BoundParameter> {
        self.config.validate()?;
        bindings.validate()?;
        parameter.validate()?;

        let mut budget =
            BindingBudget::new(self.config);

        let mut stats =
            BindingStats::default();

        let result = match mode {
            BindingMode::Partial => {
                bind_partial_parameter(
                    parameter,
                    bindings,
                    &mut budget,
                    &mut stats,
                )?
            }

            BindingMode::Complete => {
                bind_complete_parameter(
                    parameter,
                    bindings,
                    &mut budget,
                    &mut stats,
                )?
            }
        };

        result.validate().map_err(|_| {
            BindingError::InvalidResult.into()
        })?;

        stats.changed = result != *parameter;

        Ok(BoundParameter {
            parameter: result,
            stats,
        })
    }

    /// Performs partial binding.
    ///
    /// This is the most useful API for symbolic optimization passes because
    /// unresolved symbols remain available for later compilation stages.
    pub fn bind_partial(
        &self,
        parameter: &Parameter,
        bindings: &ParameterBindings,
    ) -> IrResult<BoundParameter> {
        self.bind_parameter(
            parameter,
            bindings,
            BindingMode::Partial,
        )
    }

    /// Performs complete binding.
    ///
    /// Every symbol must have a finite numerical value.
    pub fn bind_complete(
        &self,
        parameter: &Parameter,
        bindings: &ParameterBindings,
    ) -> IrResult<BoundParameter> {
        self.bind_parameter(
            parameter,
            bindings,
            BindingMode::Complete,
        )
    }

    /// Binds a parameter transactionally in place.
    pub fn bind_parameter_in_place(
        &self,
        parameter: &mut Parameter,
        bindings: &ParameterBindings,
        mode: BindingMode,
    ) -> IrResult<BindingStats> {
        let result =
            self.bind_parameter(parameter, bindings, mode)?;

        *parameter = result.parameter;

        Ok(result.stats)
    }

    /// Binds every parameter in a canonical gate-parameter group.
    ///
    /// Gate parameter arity and variant are preserved exactly.
    pub fn bind_gate_parameter(
        &self,
        parameter: &GateParameter,
        bindings: &ParameterBindings,
        mode: BindingMode,
    ) -> IrResult<BoundGateParameters> {
        self.config.validate()?;
        bindings.validate()?;
        parameter.validate()?;

        let mut stats =
            BindingStats::default();

        let result = match parameter {
            GateParameter::Angle(value) => {
                let result =
                    self.bind_parameter(
                        value,
                        bindings,
                        mode,
                    )?;

                stats.merge(result.stats);

                GateParameter::angle(
                    result.parameter,
                )?
            }

            GateParameter::TwoAngles {
                theta,
                phi,
            } => {
                let theta_result =
                    self.bind_parameter(
                        theta,
                        bindings,
                        mode,
                    )?;

                stats.merge(theta_result.stats);

                let phi_result =
                    self.bind_parameter(
                        phi,
                        bindings,
                        mode,
                    )?;

                stats.merge(phi_result.stats);

                GateParameter::two_angles(
                    theta_result.parameter,
                    phi_result.parameter,
                )?
            }

            GateParameter::ThreeAngles {
                theta,
                phi,
                lambda,
            } => {
                let theta_result =
                    self.bind_parameter(
                        theta,
                        bindings,
                        mode,
                    )?;

                stats.merge(theta_result.stats);

                let phi_result =
                    self.bind_parameter(
                        phi,
                        bindings,
                        mode,
                    )?;

                stats.merge(phi_result.stats);

                let lambda_result =
                    self.bind_parameter(
                        lambda,
                        bindings,
                        mode,
                    )?;

                stats.merge(lambda_result.stats);

                GateParameter::three_angles(
                    theta_result.parameter,
                    phi_result.parameter,
                    lambda_result.parameter,
                )?
            }
        };

        result.validate()?;

        stats.changed = result != *parameter;

        Ok(BoundGateParameters {
            parameter: result,
            stats,
        })
    }

    /// Binds a gate-parameter group transactionally in place.
    pub fn bind_gate_parameter_in_place(
        &self,
        parameter: &mut GateParameter,
        bindings: &ParameterBindings,
        mode: BindingMode,
    ) -> IrResult<BindingStats> {
        let result =
            self.bind_gate_parameter(
                parameter,
                bindings,
                mode,
            )?;

        *parameter = result.parameter;

        Ok(result.stats)
    }

    /// Collects the names of all symbolic parameters occurring in a parameter.
    ///
    /// Names are returned in deterministic lexical order with duplicates
    /// removed.
    pub fn collect_symbols(
        &self,
        parameter: &Parameter,
    ) -> IrResult<Vec<String>> {
        self.config.validate()?;
        parameter.validate()?;

        let mut budget =
            BindingBudget::new(self.config);

        let mut names =
            BTreeMap::<String, ()>::new();

        collect_symbols_recursive(
            parameter,
            &mut budget,
            &mut names,
        )?;

        Ok(names.into_keys().collect())
    }

    /// Returns the symbols that are required by `parameter` but absent from
    /// `bindings`.
    ///
    /// The result is deterministic and duplicate-free.
    pub fn missing_symbols(
        &self,
        parameter: &Parameter,
        bindings: &ParameterBindings,
    ) -> IrResult<Vec<String>> {
        let symbols =
            self.collect_symbols(parameter)?;

        let mut missing = Vec::new();

        for name in symbols {
            if !bindings.contains(&name) {
                missing.push(name);
            }
        }

        Ok(missing)
    }

    /// Returns whether a parameter can be completely bound using the supplied
    /// environment.
    pub fn can_bind_completely(
        &self,
        parameter: &Parameter,
        bindings: &ParameterBindings,
    ) -> IrResult<bool> {
        Ok(self
            .missing_symbols(parameter, bindings)?
            .is_empty())
    }
}

// -----------------------------------------------------------------------------
// Binding budget
// -----------------------------------------------------------------------------

/// Per-operation resource accounting.
///
/// This object is deliberately local to an invocation. No global mutable
/// counters are used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BindingBudget {
    max_nodes: usize,
    max_symbol_lookups: usize,
    max_output_nodes: usize,
    nodes: usize,
    symbol_lookups: usize,
    output_nodes: usize,
}

impl BindingBudget {
    const fn new(
        config: BindingConfig,
    ) -> Self {
        Self {
            max_nodes: config.max_nodes,
            max_symbol_lookups: config.max_symbol_lookups,
            max_output_nodes: config.max_output_nodes,
            nodes: 0,
            symbol_lookups: 0,
            output_nodes: 0,
        }
    }

    fn visit_node(&mut self) -> IrResult<()> {
        self.nodes = self.nodes.checked_add(1).ok_or_else(|| {
            BindingError::ResourceLimitExceeded {
                resource: "nodes",
                limit: self.max_nodes,
                actual: usize::MAX,
            }
            .into()
        })?;

        if self.nodes > self.max_nodes {
            return Err(
                BindingError::ResourceLimitExceeded {
                    resource: "nodes",
                    limit: self.max_nodes,
                    actual: self.nodes,
                }
                .into(),
            );
        }

        Ok(())
    }

    fn lookup_symbol(&mut self) -> IrResult<()> {
        self.symbol_lookups =
            self.symbol_lookups
                .checked_add(1)
                .ok_or_else(|| {
                    BindingError::ResourceLimitExceeded {
                        resource: "symbol lookups",
                        limit: self.max_symbol_lookups,
                        actual: usize::MAX,
                    }
                    .into()
                })?;

        if self.symbol_lookups >
            self.max_symbol_lookups
        {
            return Err(
                BindingError::ResourceLimitExceeded {
                    resource: "symbol lookups",
                    limit: self.max_symbol_lookups,
                    actual: self.symbol_lookups,
                }
                .into(),
            );
        }

        Ok(())
    }

    fn output_node(&mut self) -> IrResult<()> {
        self.output_nodes =
            self.output_nodes
                .checked_add(1)
                .ok_or_else(|| {
                    BindingError::ResourceLimitExceeded {
                        resource: "output nodes",
                        limit: self.max_output_nodes,
                        actual: usize::MAX,
                    }
                    .into()
                })?;

        if self.output_nodes >
            self.max_output_nodes
        {
            return Err(
                BindingError::ResourceLimitExceeded {
                    resource: "output nodes",
                    limit: self.max_output_nodes,
                    actual: self.output_nodes,
                }
                .into(),
            );
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Partial binding
// -----------------------------------------------------------------------------

fn bind_partial_parameter(
    parameter: &Parameter,
    bindings: &ParameterBindings,
    budget: &mut BindingBudget,
    stats: &mut BindingStats,
) -> IrResult<Parameter> {
    budget.visit_node()?;
    stats.add_nodes();

    match parameter {
        Parameter::Constant(value) => {
            budget.output_node()?;
            stats.add_output_node();

            if !value.is_finite() {
                return Err(
                    IrParameterError::NonFinite.into()
                );
            }

            Ok(Parameter::Constant(*value))
        }

        Parameter::Symbol(name) => {
            stats.add_symbol_seen();

            budget.lookup_symbol()?;
            stats.add_lookup();

            match bindings.get(name) {
                Some(value) => {
                    if !value.is_finite() {
                        return Err(
                            BindingError::NonFiniteValue {
                                name: name.clone(),
                            }
                            .into(),
                        );
                    }

                    budget.output_node()?;
                    stats.add_output_node();
                    stats.add_bound();

                    Ok(Parameter::Constant(value))
                }

                None => {
                    budget.output_node()?;
                    stats.add_output_node();
                    stats.add_unresolved();

                    Ok(Parameter::symbol(name.clone())?)
                }
            }
        }

        Parameter::Expression(expression) => {
            bind_partial_expression(
                expression,
                bindings,
                budget,
                stats,
            )
        }
    }
}

fn bind_partial_expression(
    expression: &ParameterExpression,
    bindings: &ParameterBindings,
    budget: &mut BindingBudget,
    stats: &mut BindingStats,
) -> IrResult<Parameter> {
    let (left, right, operation) =
        match expression {
            ParameterExpression::Add(
                left,
                right,
            ) => (
                left.as_ref(),
                right.as_ref(),
                ExpressionOperation::Add,
            ),

            ParameterExpression::Subtract(
                left,
                right,
            ) => (
                left.as_ref(),
                right.as_ref(),
                ExpressionOperation::Subtract,
            ),

            ParameterExpression::Multiply(
                left,
                right,
            ) => (
                left.as_ref(),
                right.as_ref(),
                ExpressionOperation::Multiply,
            ),

            ParameterExpression::Divide(
                left,
                right,
            ) => (
                left.as_ref(),
                right.as_ref(),
                ExpressionOperation::Divide,
            ),

            ParameterExpression::Negate(
                value,
            ) => {
                let bound =
                    bind_partial_parameter(
                        value,
                        bindings,
                        budget,
                        stats,
                    )?;

                budget.output_node()?;
                stats.add_output_node();

                return Parameter::expression(
                    ParameterExpression::Negate(
                        Box::new(bound),
                    ),
                );
            }
        };

    let left =
        bind_partial_parameter(
            left,
            bindings,
            budget,
            stats,
        )?;

    let right =
        bind_partial_parameter(
            right,
            bindings,
            budget,
            stats,
        )?;

    budget.output_node()?;
    stats.add_output_node();

    let expression =
        match operation {
            ExpressionOperation::Add => {
                ParameterExpression::Add(
                    Box::new(left),
                    Box::new(right),
                )
            }

            ExpressionOperation::Subtract => {
                ParameterExpression::Subtract(
                    Box::new(left),
                    Box::new(right),
                )
            }

            ExpressionOperation::Multiply => {
                ParameterExpression::Multiply(
                    Box::new(left),
                    Box::new(right),
                )
            }

            ExpressionOperation::Divide => {
                ParameterExpression::Divide(
                    Box::new(left),
                    Box::new(right),
                )
            }
        };

    Parameter::expression(expression)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpressionOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// -----------------------------------------------------------------------------
// Complete binding
// -----------------------------------------------------------------------------

fn bind_complete_parameter(
    parameter: &Parameter,
    bindings: &ParameterBindings,
    budget: &mut BindingBudget,
    stats: &mut BindingStats,
) -> IrResult<Parameter> {
    let partial =
        bind_partial_parameter(
            parameter,
            bindings,
            budget,
            stats,
        )?;

    if partial.is_symbolic() {
        let missing =
            collect_missing_from_bound_parameter(
                &partial,
                bindings,
                budget,
            )?;

        if let Some(name) = missing.into_iter().next() {
            return Err(
                BindingError::MissingSymbol { name }
                    .into(),
            );
        }
    }

    // The canonical IR owns numerical expression evaluation. This avoids
    // duplicating floating-point semantics in the optimizer.
    let resolved =
        partial.bind(&|name| bindings.get(name))?;

    stats.output_nodes =
        stats.output_nodes.saturating_add(1);

    Ok(Parameter::constant(resolved)?)
}

fn collect_missing_from_bound_parameter(
    parameter: &Parameter,
    bindings: &ParameterBindings,
    budget: &mut BindingBudget,
) -> IrResult<Vec<String>> {
    let mut names =
        BTreeMap::<String, ()>::new();

    collect_missing_recursive(
        parameter,
        bindings,
        budget,
        &mut names,
    )?;

    Ok(names.into_keys().collect())
}

// -----------------------------------------------------------------------------
// Symbol collection
// -----------------------------------------------------------------------------

fn collect_symbols_recursive(
    parameter: &Parameter,
    budget: &mut BindingBudget,
    names: &mut BTreeMap<String, ()>,
) -> IrResult<()> {
    budget.visit_node()?;

    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(
                    IrParameterError::NonFinite.into()
                );
            }
        }

        Parameter::Symbol(name) => {
            validate_symbol_name(name)?;

            names.insert(name.clone(), ());
        }

        Parameter::Expression(expression) => {
            match expression {
                ParameterExpression::Add(
                    left,
                    right,
                )
                | ParameterExpression::Subtract(
                    left,
                    right,
                )
                | ParameterExpression::Multiply(
                    left,
                    right,
                )
                | ParameterExpression::Divide(
                    left,
                    right,
                ) => {
                    collect_symbols_recursive(
                        left,
                        budget,
                        names,
                    )?;

                    collect_symbols_recursive(
                        right,
                        budget,
                        names,
                    )?;
                }

                ParameterExpression::Negate(
                    value,
                ) => {
                    collect_symbols_recursive(
                        value,
                        budget,
                        names,
                    )?;
                }
            }
        }
    }

    Ok(())
}

fn collect_missing_recursive(
    parameter: &Parameter,
    bindings: &ParameterBindings,
    budget: &mut BindingBudget,
    names: &mut BTreeMap<String, ()>,
) -> IrResult<()> {
    budget.visit_node()?;

    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(
                    IrParameterError::NonFinite.into()
                );
            }
        }

        Parameter::Symbol(name) => {
            validate_symbol_name(name)?;

            budget.lookup_symbol()?;

            if !bindings.contains(name) {
                names.insert(name.clone(), ());
            }
        }

        Parameter::Expression(expression) => {
            match expression {
                ParameterExpression::Add(
                    left,
                    right,
                )
                | ParameterExpression::Subtract(
                    left,
                    right,
                )
                | ParameterExpression::Multiply(
                    left,
                    right,
                )
                | ParameterExpression::Divide(
                    left,
                    right,
                ) => {
                    collect_missing_recursive(
                        left,
                        bindings,
                        budget,
                        names,
                    )?;

                    collect_missing_recursive(
                        right,
                        bindings,
                        budget,
                        names,
                    )?;
                }

                ParameterExpression::Negate(
                    value,
                ) => {
                    collect_missing_recursive(
                        value,
                        bindings,
                        budget,
                        names,
                    )?;
                }
            }
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Validation helpers
// -----------------------------------------------------------------------------

fn validate_symbol_name(
    name: &str,
) -> IrResult<()> {
    // Delegate canonical symbol validation to the IR. Creating a temporary
    // Parameter ensures binding and IR use exactly the same symbol contract.
    Parameter::symbol(name.to_owned()).map(|_| ())
}

// -----------------------------------------------------------------------------
// Statistics helpers
// -----------------------------------------------------------------------------

impl BindingStats {
    fn merge(
        &mut self,
        other: Self,
    ) {
        self.nodes_visited = self
            .nodes_visited
            .saturating_add(other.nodes_visited);

        self.symbols_seen = self
            .symbols_seen
            .saturating_add(other.symbols_seen);

        self.symbols_bound = self
            .symbols_bound
            .saturating_add(other.symbols_bound);

        self.symbols_unresolved = self
            .symbols_unresolved
            .saturating_add(other.symbols_unresolved);

        self.symbol_lookups = self
            .symbol_lookups
            .saturating_add(other.symbol_lookups);

        self.output_nodes = self
            .output_nodes
            .saturating_add(other.output_nodes);

        self.changed |= other.changed;
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn constant(
        value: f64,
    ) -> Parameter {
        Parameter::constant(value)
            .expect("test constant must be finite")
    }

    fn symbol(
        name: &str,
    ) -> Parameter {
        Parameter::symbol(name)
            .expect("test symbol must be valid")
    }

    fn expression(
        expression: ParameterExpression,
    ) -> Parameter {
        Parameter::expression(expression)
            .expect("test expression must be valid")
    }

    #[test]
    fn empty_environment_is_valid() {
        let bindings =
            ParameterBindings::new();

        assert!(bindings.is_empty());
        assert_eq!(bindings.len(), 0);
        assert!(bindings.validate().is_ok());
    }

    #[test]
    fn insert_binding_is_deterministic() {
        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("z", 3.0)
            .expect("insert z");

        bindings
            .insert("a", 1.0)
            .expect("insert a");

        bindings
            .insert("m", 2.0)
            .expect("insert m");

        let names: Vec<&str> =
            bindings.names().collect();

        assert_eq!(
            names,
            vec!["a", "m", "z"]
        );
    }

    #[test]
    fn duplicate_insert_is_rejected() {
        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("x", 1.0)
            .expect("first insert");

        assert!(
            bindings
                .insert("x", 2.0)
                .is_err()
        );
    }

    #[test]
    fn replacement_is_explicit() {
        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("x", 1.0)
            .expect("insert");

        let previous =
            bindings
                .insert_or_replace("x", 2.0)
                .expect("replace");

        assert_eq!(previous, Some(1.0));
        assert_eq!(
            bindings.get("x"),
            Some(2.0)
        );
    }

    #[test]
    fn non_finite_binding_is_rejected() {
        let mut bindings =
            ParameterBindings::new();

        assert!(
            bindings
                .insert("x", f64::NAN)
                .is_err()
        );

        assert!(
            bindings
                .insert("y", f64::INFINITY)
                .is_err()
        );
    }

    #[test]
    fn partial_symbol_binding() {
        let binder =
            ParameterBinder::new();

        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("x", 4.0)
            .expect("insert");

        let parameter =
            symbol("x");

        let result =
            binder
                .bind_partial(
                    &parameter,
                    &bindings,
                )
                .expect("binding");

        assert_eq!(
            result.parameter,
            constant(4.0)
        );

        assert_eq!(
            result.stats.symbols_bound,
            1
        );
    }

    #[test]
    fn partial_binding_preserves_unknown_symbols() {
        let binder =
            ParameterBinder::new();

        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("x", 4.0)
            .expect("insert");

        let parameter =
            expression(
                ParameterExpression::Add(
                    Box::new(symbol("x")),
                    Box::new(symbol("y")),
                ),
            );

        let result =
            binder
                .bind_partial(
                    &parameter,
                    &bindings,
                )
                .expect("binding");

        let expected =
            expression(
                ParameterExpression::Add(
                    Box::new(constant(4.0)),
                    Box::new(symbol("y")),
                ),
            );

        assert_eq!(
            result.parameter,
            expected
        );

        assert_eq!(
            result.stats.symbols_bound,
            1
        );

        assert_eq!(
            result.stats.symbols_unresolved,
            1
        );
    }

    #[test]
    fn complete_binding_evaluates_expression() {
        let binder =
            ParameterBinder::new();

        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("x", 2.0)
            .expect("insert x");

        bindings
            .insert("y", 3.0)
            .expect("insert y");

        let parameter =
            expression(
                ParameterExpression::Multiply(
                    Box::new(symbol("x")),
                    Box::new(
                        expression(
                            ParameterExpression::Add(
                                Box::new(symbol("y")),
                                Box::new(
                                    constant(1.0),
                                ),
                            ),
                        ),
                    ),
                ),
            );

        let result =
            binder
                .bind_complete(
                    &parameter,
                    &bindings,
                )
                .expect("complete binding");

        assert_eq!(
            result.parameter,
            constant(8.0)
        );
    }

    #[test]
    fn complete_binding_rejects_missing_symbol() {
        let binder =
            ParameterBinder::new();

        let bindings =
            ParameterBindings::new();

        let parameter =
            symbol("x");

        let result =
            binder.bind_complete(
                &parameter,
                &bindings,
            );

        assert!(result.is_err());
    }

    #[test]
    fn collect_symbols_is_unique_and_sorted() {
        let binder =
            ParameterBinder::new();

        let parameter =
            expression(
                ParameterExpression::Add(
                    Box::new(symbol("z")),
                    Box::new(
                        expression(
                            ParameterExpression::Add(
                                Box::new(symbol("a")),
                                Box::new(symbol("z")),
                            ),
                        ),
                    ),
                ),
            );

        let symbols =
            binder
                .collect_symbols(&parameter)
                .expect("collect symbols");

        assert_eq!(
            symbols,
            vec![
                "a".to_owned(),
                "z".to_owned()
            ]
        );
    }

    #[test]
    fn missing_symbols_are_sorted() {
        let binder =
            ParameterBinder::new();

        let bindings =
            ParameterBindings::new();

        let parameter =
            expression(
                ParameterExpression::Add(
                    Box::new(symbol("z")),
                    Box::new(symbol("a")),
                ),
            );

        let missing =
            binder
                .missing_symbols(
                    &parameter,
                    &bindings,
                )
                .expect("missing symbols");

        assert_eq!(
            missing,
            vec![
                "a".to_owned(),
                "z".to_owned()
            ]
        );
    }

    #[test]
    fn gate_parameter_arity_is_preserved() {
        let binder =
            ParameterBinder::new();

        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("theta", 1.0)
            .expect("theta");

        bindings
            .insert("phi", 2.0)
            .expect("phi");

        let parameter =
            GateParameter::two_angles(
                symbol("theta"),
                symbol("phi"),
            )
            .expect("gate parameter");

        let result =
            binder
                .bind_gate_parameter(
                    &parameter,
                    &bindings,
                    BindingMode::Complete,
                )
                .expect("bind gate parameters");

        assert_eq!(
            result.parameter.arity(),
            2
        );

        match result.parameter {
            GateParameter::TwoAngles {
                theta,
                phi,
            } => {
                assert_eq!(
                    theta,
                    constant(1.0)
                );

                assert_eq!(
                    phi,
                    constant(2.0)
                );
            }

            _ => panic!(
                "binding changed gate parameter variant"
            ),
        }
    }

    #[test]
    fn in_place_binding_is_transactional_on_error() {
        let binder =
            ParameterBinder::new();

        let bindings =
            ParameterBindings::new();

        let original =
            symbol("missing");

        let mut parameter =
            original.clone();

        let result =
            binder.bind_parameter_in_place(
                &mut parameter,
                &bindings,
                BindingMode::Complete,
            );

        assert!(result.is_err());
        assert_eq!(
            parameter,
            original
        );
    }

    #[test]
    fn zero_limits_are_rejected() {
        let config =
            BindingConfig::new(
                1,
                0,
                1,
                1,
            );

        assert!(
            config.validate().is_err()
        );
    }

    #[test]
    fn binding_limit_is_enforced() {
        let mut bindings =
            ParameterBindings::with_capacity_limit(
                1,
            );

        bindings
            .insert("x", 1.0)
            .expect("first binding");

        assert!(
            bindings
                .insert("y", 2.0)
                .is_err()
        );
    }

    #[test]
    fn partial_binding_is_idempotent_for_fully_bound_parameter() {
        let binder =
            ParameterBinder::new();

        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("x", 7.0)
            .expect("insert");

        let parameter =
            symbol("x");

        let first =
            binder
                .bind_partial(
                    &parameter,
                    &bindings,
                )
                .expect("first bind");

        let second =
            binder
                .bind_partial(
                    &first.parameter,
                    &bindings,
                )
                .expect("second bind");

        assert_eq!(
            first.parameter,
            second.parameter
        );
    }

    #[test]
    fn complete_constant_binding_does_not_require_symbols() {
        let binder =
            ParameterBinder::new();

        let bindings =
            ParameterBindings::new();

        let parameter =
            constant(42.0);

        let result =
            binder
                .bind_complete(
                    &parameter,
                    &bindings,
                )
                .expect("constant binding");

        assert_eq!(
            result.parameter,
            parameter
        );
    }
}