
#![forbid(unsafe_code)]

//! Resource budgets and checked accounting for the Zamani parser.
//!
//! # Responsibilities
//!
//! This module owns:
//! - parser-specific resource-budget configuration;
//! - stable identities for independently enforceable parser budgets;
//! - structured resource-limit violations;
//! - checked, per-parse resource accounting;
//! - cumulative usage counters and maximum-observed measurements.
//!
//! This module does not own:
//! - lexical recognition or token construction;
//! - parser grammar or token-cursor movement;
//! - AST construction or semantic validation;
//! - type checking, capability resolution, or resource negotiation;
//! - hardware discovery, allocation, scheduling, or execution.
//!
//! # Resource model
//!
//! An unlimited budget introduces no explicit policy limit for its
//! resource. It does not promise infinite memory, address space, or time.
//!
//! A configured budget protects an implementation boundary. It MUST NOT
//! be interpreted as a language-level restriction or a hardware capacity.
//!
//! The same language constructs must remain expressible regardless of
//! the number of qubits, processors, devices, registers, tensor dimensions,
//! cluster nodes, or other resources involved.
//!
//! # Integration contract
//!
//! - `config.rs` from the lexer owns the canonical `ResourceLimit` type.
//! - The parser owns `ParserLimits`, `ParserUsage`, and `ParserBudget`.
//! - `config.rs` in the parser should embed `ParserLimits` rather than
//!   redefining its resource dimensions.
//! - `session.rs` should create one `ParserBudget` per parse invocation.
//! - `token_cursor.rs` should charge token consumption consistently.
//! - Grammar productions should charge AST nodes and cumulative work
//!   before committing resource-consuming operations.
//! - Recursive grammar entry points should observe nesting depth before
//!   descending.
//! - Expression parsing should observe expression depth independently
//!   of general grammar nesting.
//! - `diagnostics.rs` should account for diagnostics before retaining them.
//! - `recovery.rs` must respect the same work and diagnostic budgets.
//! - `api.rs` should expose structured limit failures through the
//!   established parser result and diagnostic contracts.
//!
//! This module intentionally does not depend on parser internals, the AST,
//! any domain grammar, or any execution target.
//!
//! # Counting conventions
//!
//! - Byte counts refer to UTF-8 source bytes.
//! - Token counts follow the parser's documented token-consumption policy.
//! - AST-node and work counts are cumulative within one parse invocation.
//! - Depth and individual-item sizes record their maximum observed values.
//! - Source bytes represent the size of the source input, not an increment.
//! - Failed accounting never modifies the corresponding usage counter.
//!
//! A parser must not claim a budget is enforced merely because the budget
//! exists. The consuming operation must call the corresponding method.

use core::fmt;

pub use super::super::lexer::config::ResourceLimit;

/// Identity of an independently enforceable parser resource budget.
///
/// Variants describe parser implementation work and input dimensions,
/// not computational domains or physical hardware capacities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ParserLimitKind {
    /// UTF-8 bytes in the source input.
    SourceBytes,

    /// Tokens consumed by the parser.
    Tokens,

    /// AST nodes constructed for the parsed input.
    AstNodes,

    /// General grammar-recursion nesting depth.
    NestingDepth,

    /// Expression-parser nesting depth.
    ExpressionDepth,

    /// Diagnostics retained or emitted during parsing.
    Diagnostics,

    /// Maximum UTF-8 byte length of an identifier spelling.
    IdentifierBytes,

    /// Maximum UTF-8 byte length of a numeric literal spelling.
    NumericLiteralBytes,

    /// Maximum UTF-8 byte length of a string literal spelling.
    StringLiteralBytes,

    /// Maximum include/import nesting depth when that feature is
    /// implemented by a higher-level source-loading component.
    IncludeDepth,

    /// Source-level gate or equivalent callable definitions processed.
    GateDefinitions,

    /// Operations processed within source-level gate or callable bodies.
    GateOperations,

    /// Maximum observed source-level register or dimension size.
    RegisterSize,

    /// Maximum number of parameters in one declaration or call.
    ParametersPerItem,

    /// Maximum number of operands in one source-level operation.
    OperandsPerOperation,

    /// Source-level statements processed.
    Statements,

    /// Abstract parser work units charged by grammar implementations.
    TotalWork,
}

impl ParserLimitKind {
    /// Returns the stable machine-readable identifier of this resource.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceBytes => "source_bytes",
            Self::Tokens => "tokens",
            Self::AstNodes => "ast_nodes",
            Self::NestingDepth => "nesting_depth",
            Self::ExpressionDepth => "expression_depth",
            Self::Diagnostics => "diagnostics",
            Self::IdentifierBytes => "identifier_bytes",
            Self::NumericLiteralBytes => "numeric_literal_bytes",
            Self::StringLiteralBytes => "string_literal_bytes",
            Self::IncludeDepth => "include_depth",
            Self::GateDefinitions => "gate_definitions",
            Self::GateOperations => "gate_operations",
            Self::RegisterSize => "register_size",
            Self::ParametersPerItem => "parameters_per_item",
            Self::OperandsPerOperation => "operands_per_operation",
            Self::Statements => "statements",
            Self::TotalWork => "total_work",
        }
    }

    /// Returns the configured budget for this resource.
    #[must_use]
    pub const fn limit(self, limits: &ParserLimits) -> ResourceLimit {
        match self {
            Self::SourceBytes => limits.source_bytes,
            Self::Tokens => limits.tokens,
            Self::AstNodes => limits.ast_nodes,
            Self::NestingDepth => limits.nesting_depth,
            Self::ExpressionDepth => limits.expression_depth,
            Self::Diagnostics => limits.diagnostics,
            Self::IdentifierBytes => limits.identifier_bytes,
            Self::NumericLiteralBytes => limits.numeric_literal_bytes,
            Self::StringLiteralBytes => limits.string_literal_bytes,
            Self::IncludeDepth => limits.include_depth,
            Self::GateDefinitions => limits.gate_definitions,
            Self::GateOperations => limits.gate_operations,
            Self::RegisterSize => limits.register_size,
            Self::ParametersPerItem => limits.parameters_per_item,
            Self::OperandsPerOperation => limits.operands_per_operation,
            Self::Statements => limits.statements,
            Self::TotalWork => limits.total_work,
        }
    }

    const fn is_cumulative(self) -> bool {
        matches!(
            self,
            Self::Tokens
                | Self::AstNodes
                | Self::Diagnostics
                | Self::GateDefinitions
                | Self::GateOperations
                | Self::Statements
                | Self::TotalWork
        )
    }
}

impl fmt::Display for ParserLimitKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Parser-specific resource policy.
///
/// All fields default to [`ResourceLimit::Unlimited`]. This preserves the
/// separation between language semantics and operational policy.
///
/// Applications handling untrusted input SHOULD configure finite budgets
/// appropriate to their environment. No single finite default is correct
/// for every compiler, editor, embedded environment, or build service.
///
/// The values are implementation safeguards, not maximum supported
/// machine sizes or maximum expressible program dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ParserLimits {
    /// Maximum UTF-8 bytes accepted for one source input.
    pub source_bytes: ResourceLimit,

    /// Maximum tokens consumed by one parse invocation.
    pub tokens: ResourceLimit,

    /// Maximum AST nodes constructed by one parse invocation.
    pub ast_nodes: ResourceLimit,

    /// Maximum observed grammar nesting depth.
    pub nesting_depth: ResourceLimit,

    /// Maximum observed expression nesting depth.
    pub expression_depth: ResourceLimit,

    /// Maximum diagnostics retained or emitted.
    pub diagnostics: ResourceLimit,

    /// Maximum UTF-8 byte length of one identifier spelling.
    pub identifier_bytes: ResourceLimit,

    /// Maximum UTF-8 byte length of one numeric literal spelling.
    pub numeric_literal_bytes: ResourceLimit,

    /// Maximum UTF-8 byte length of one string literal spelling.
    pub string_literal_bytes: ResourceLimit,

    /// Maximum include/import nesting depth when applicable.
    ///
    /// Source loading and include-cycle detection belong to the component
    /// that resolves imports. This field does not enable source loading.
    pub include_depth: ResourceLimit,

    /// Maximum source-level gate or equivalent callable definitions.
    pub gate_definitions: ResourceLimit,

    /// Maximum cumulative operations processed in gate/callable bodies.
    pub gate_operations: ResourceLimit,

    /// Maximum observed register or source-level dimension size.
    ///
    /// This is a configurable processing budget, not a fixed language
    /// restriction on the dimensions expressible by a program.
    pub register_size: ResourceLimit,

    /// Maximum parameters in an individual declaration or call.
    pub parameters_per_item: ResourceLimit,

    /// Maximum operands in an individual source-level operation.
    pub operands_per_operation: ResourceLimit,

    /// Maximum cumulative statements processed.
    pub statements: ResourceLimit,

    /// Maximum cumulative abstract parser work units.
    pub total_work: ResourceLimit,
}

impl ParserLimits {
    /// Returns a policy without explicit parser resource ceilings.
    ///
    /// The caller remains responsible for actual resource availability.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            source_bytes: ResourceLimit::Unlimited,
            tokens: ResourceLimit::Unlimited,
            ast_nodes: ResourceLimit::Unlimited,
            nesting_depth: ResourceLimit::Unlimited,
            expression_depth: ResourceLimit::Unlimited,
            diagnostics: ResourceLimit::Unlimited,
            identifier_bytes: ResourceLimit::Unlimited,
            numeric_literal_bytes: ResourceLimit::Unlimited,
            string_literal_bytes: ResourceLimit::Unlimited,
            include_depth: ResourceLimit::Unlimited,
            gate_definitions: ResourceLimit::Unlimited,
            gate_operations: ResourceLimit::Unlimited,
            register_size: ResourceLimit::Unlimited,
            parameters_per_item: ResourceLimit::Unlimited,
            operands_per_operation: ResourceLimit::Unlimited,
            statements: ResourceLimit::Unlimited,
            total_work: ResourceLimit::Unlimited,
        }
    }

    /// Checks a representable usage value without changing any counters.
    #[must_use]
    pub fn check(
        &self,
        resource: ParserLimitKind,
        observed: usize,
    ) -> Result<(), ParserLimitViolation> {
        let limit = resource.limit(self);

        if limit.permits(observed) {
            Ok(())
        } else {
            Err(ParserLimitViolation::exceeded(
                resource, limit, observed,
            ))
        }
    }
}

/// Why a resource-budget operation failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParserLimitFailure {
    /// The proposed usage exceeds the configured budget.
    Exceeded,

    /// A usage counter cannot represent the proposed value.
    CounterOverflow,

    /// The requested accounting operation is incompatible with the
    /// resource's documented counting convention.
    InvalidAccountingOperation,
}

impl fmt::Display for ParserLimitFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exceeded => formatter.write_str("parser resource budget exceeded"),
            Self::CounterOverflow => {
                formatter.write_str("parser resource counter overflow")
            }
            Self::InvalidAccountingOperation => {
                formatter.write_str("invalid parser resource accounting operation")
            }
        }
    }
}

/// Structured failure returned by parser-budget checks and accounting.
///
/// `observed` is `None` when checked arithmetic overflowed. The reported
/// value is never fabricated or saturated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParserLimitViolation {
    resource: ParserLimitKind,
    limit: ResourceLimit,
    observed: Option<usize>,
    failure: ParserLimitFailure,
}

impl ParserLimitViolation {
    const fn exceeded(
        resource: ParserLimitKind,
        limit: ResourceLimit,
        observed: usize,
    ) -> Self {
        Self {
            resource,
            limit,
            observed: Some(observed),
            failure: ParserLimitFailure::Exceeded,
        }
    }

    const fn overflow(
        resource: ParserLimitKind,
        limit: ResourceLimit,
    ) -> Self {
        Self {
            resource,
            limit,
            observed: None,
            failure: ParserLimitFailure::CounterOverflow,
        }
    }

    const fn invalid_operation(
        resource: ParserLimitKind,
        limit: ResourceLimit,
    ) -> Self {
        Self {
            resource,
            limit,
            observed: None,
            failure: ParserLimitFailure::InvalidAccountingOperation,
        }
    }

    /// Resource whose operation failed.
    #[must_use]
    pub const fn resource(self) -> ParserLimitKind {
        self.resource
    }

    /// Configured budget that rejected the operation.
    #[must_use]
    pub const fn limit(self) -> ResourceLimit {
        self.limit
    }

    /// Proposed usage when it is representable.
    #[must_use]
    pub const fn observed(self) -> Option<usize> {
        self.observed
    }

    /// Failure category.
    #[must_use]
    pub const fn failure(self) -> ParserLimitFailure {
        self.failure
    }
}

impl fmt::Display for ParserLimitViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "parser resource '{}' failed: {} (limit: {}, observed: {})",
            self.resource,
            self.failure,
            self.limit,
            self.observed
                .map_or_else(|| "unrepresentable".to_owned(), |n| n.to_string()),
        )
    }
}

impl std::error::Error for ParserLimitViolation {}

/// Resource usage accumulated by one parser invocation.
///
/// Fields are private so callers cannot bypass budget checks by modifying
/// counters directly. A new instance should be created for each parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ParserUsage {
    source_bytes: usize,
    tokens: usize,
    ast_nodes: usize,
    nesting_depth: usize,
    expression_depth: usize,
    diagnostics: usize,
    identifier_bytes: usize,
    numeric_literal_bytes: usize,
    string_literal_bytes: usize,
    include_depth: usize,
    gate_definitions: usize,
    gate_operations: usize,
    register_size: usize,
    parameters_per_item: usize,
    operands_per_operation: usize,
    statements: usize,
    total_work: usize,
}

impl ParserUsage {
    /// Returns the observed value for a resource.
    #[must_use]
    pub const fn get(self, resource: ParserLimitKind) -> usize {
        match resource {
            ParserLimitKind::SourceBytes => self.source_bytes,
            ParserLimitKind::Tokens => self.tokens,
            ParserLimitKind::AstNodes => self.ast_nodes,
            ParserLimitKind::NestingDepth => self.nesting_depth,
            ParserLimitKind::ExpressionDepth => self.expression_depth,
            ParserLimitKind::Diagnostics => self.diagnostics,
            ParserLimitKind::IdentifierBytes => self.identifier_bytes,
            ParserLimitKind::NumericLiteralBytes => self.numeric_literal_bytes,
            ParserLimitKind::StringLiteralBytes => self.string_literal_bytes,
            ParserLimitKind::IncludeDepth => self.include_depth,
            ParserLimitKind::GateDefinitions => self.gate_definitions,
            ParserLimitKind::GateOperations => self.gate_operations,
            ParserLimitKind::RegisterSize => self.register_size,
            ParserLimitKind::ParametersPerItem => self.parameters_per_item,
            ParserLimitKind::OperandsPerOperation => self.operands_per_operation,
            ParserLimitKind::Statements => self.statements,
            ParserLimitKind::TotalWork => self.total_work,
        }
    }

    fn set(&mut self, resource: ParserLimitKind, value: usize) {
        match resource {
            ParserLimitKind::SourceBytes => self.source_bytes = value,
            ParserLimitKind::Tokens => self.tokens = value,
            ParserLimitKind::AstNodes => self.ast_nodes = value,
            ParserLimitKind::NestingDepth => self.nesting_depth = value,
            ParserLimitKind::ExpressionDepth => self.expression_depth = value,
            ParserLimitKind::Diagnostics => self.diagnostics = value,
            ParserLimitKind::IdentifierBytes => self.identifier_bytes = value,
            ParserLimitKind::NumericLiteralBytes => self.numeric_literal_bytes = value,
            ParserLimitKind::StringLiteralBytes => self.string_literal_bytes = value,
            ParserLimitKind::IncludeDepth => self.include_depth = value,
            ParserLimitKind::GateDefinitions => self.gate_definitions = value,
            ParserLimitKind::GateOperations => self.gate_operations = value,
            ParserLimitKind::RegisterSize => self.register_size = value,
            ParserLimitKind::ParametersPerItem => self.parameters_per_item = value,
            ParserLimitKind::OperandsPerOperation => self.operands_per_operation = value,
            ParserLimitKind::Statements => self.statements = value,
            ParserLimitKind::TotalWork => self.total_work = value,
        }
    }
}

/// Checked parser resource accounting.
///
/// This tracker is intentionally independent of parser-session and AST
/// types. Callers decide when a resource-consuming action is committed.
///
/// `charge` is for cumulative resources. `observe_max` records the maximum
/// observed value for an individual dimension. `set_source_bytes` records
/// the size of the input and must be called before parsing begins.
///
/// A rejected operation leaves the previous usage unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserBudget {
    limits: ParserLimits,
    usage: ParserUsage,
}

impl ParserBudget {
    /// Creates a fresh tracker with the supplied immutable policy.
    #[must_use]
    pub const fn new(limits: ParserLimits) -> Self {
        Self {
            limits,
            usage: ParserUsage {
                source_bytes: 0,
                tokens: 0,
                ast_nodes: 0,
                nesting_depth: 0,
                expression_depth: 0,
                diagnostics: 0,
                identifier_bytes: 0,
                numeric_literal_bytes: 0,
                string_literal_bytes: 0,
                include_depth: 0,
                gate_definitions: 0,
                gate_operations: 0,
                register_size: 0,
                parameters_per_item: 0,
                operands_per_operation: 0,
                statements: 0,
                total_work: 0,
            },
        }
    }

    /// Returns the configured policy.
    #[must_use]
    pub const fn limits(&self) -> &ParserLimits {
        &self.limits
    }

    /// Returns a snapshot of current usage.
    #[must_use]
    pub const fn usage(&self) -> ParserUsage {
        self.usage
    }

    /// Checks a value against the configured budget without recording it.
    #[must_use]
    pub fn check(
        &self,
        resource: ParserLimitKind,
        observed: usize,
    ) -> Result<(), ParserLimitViolation> {
        self.limits.check(resource, observed)
    }

    /// Records the complete source size.
    ///
    /// This is a setter, not an additive operation. Call it once per parse
    /// before any token consumption or AST construction.
    pub fn set_source_bytes(
        &mut self,
        bytes: usize,
    ) -> Result<(), ParserLimitViolation> {
        let kind = ParserLimitKind::SourceBytes;
        self.check(kind, bytes)?;
        self.usage.source_bytes = bytes;
        Ok(())
    }

    /// Adds `amount` to a cumulative resource counter.
    ///
    /// Only cumulative resources accept this operation. For depth and
    /// individual-item dimensions, use `observe_max` instead.
    pub fn charge(
        &mut self,
        resource: ParserLimitKind,
        amount: usize,
    ) -> Result<(), ParserLimitViolation> {
        if !resource.is_cumulative() {
            return Err(ParserLimitViolation::invalid_operation(
                resource,
                resource.limit(&self.limits),
            ));
        }

        let current = self.usage.get(resource);
        let next = current.checked_add(amount).ok_or_else(|| {
            ParserLimitViolation::overflow(resource, resource.limit(&self.limits))
        })?;

        self.check(resource, next)?;
        self.usage.set(resource, next);
        Ok(())
    }

    /// Records a high-water mark for a depth or individual-item dimension.
    ///
    /// Shallower or smaller observations do not lower the previously
    /// recorded maximum. Cumulative resources are rejected.
    pub fn observe_max(
        &mut self,
        resource: ParserLimitKind,
        observed: usize,
    ) -> Result<(), ParserLimitViolation> {
        if resource.is_cumulative() || resource == ParserLimitKind::SourceBytes {
            return Err(ParserLimitViolation::invalid_operation(
                resource,
                resource.limit(&self.limits),
            ));
        }

        self.check(resource, observed)?;

        let previous = self.usage.get(resource);
        if observed > previous {
            self.usage.set(resource, observed);
        }

        Ok(())
    }

    /// Resets all usage for a new parse while retaining the same policy.
    ///
    /// Do not reset a tracker while a parser session still relies on the
    /// usage accumulated for its current input.
    pub fn reset(&mut self) {
        self.usage = ParserUsage::default();
    }
}

impl Default for ParserBudget {
    fn default() -> Self {
        Self::new(ParserLimits::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::NonZeroUsize;

    fn bounded(value: usize) -> ResourceLimit {
        ResourceLimit::at_most(value).expect("test budget must be non-zero")
    }

    fn limits_with(kind: ParserLimitKind, value: usize) -> ParserLimits {
        let mut limits = ParserLimits::unlimited();
        let limit = bounded(value);

        match kind {
            ParserLimitKind::SourceBytes => limits.source_bytes = limit,
            ParserLimitKind::Tokens => limits.tokens = limit,
            ParserLimitKind::AstNodes => limits.ast_nodes = limit,
            ParserLimitKind::NestingDepth => limits.nesting_depth = limit,
            ParserLimitKind::ExpressionDepth => limits.expression_depth = limit,
            ParserLimitKind::Diagnostics => limits.diagnostics = limit,
            ParserLimitKind::IdentifierBytes => limits.identifier_bytes = limit,
            ParserLimitKind::NumericLiteralBytes => limits.numeric_literal_bytes = limit,
            ParserLimitKind::StringLiteralBytes => limits.string_literal_bytes = limit,
            ParserLimitKind::IncludeDepth => limits.include_depth = limit,
            ParserLimitKind::GateDefinitions => limits.gate_definitions = limit,
            ParserLimitKind::GateOperations => limits.gate_operations = limit,
            ParserLimitKind::RegisterSize => limits.register_size = limit,
            ParserLimitKind::ParametersPerItem => limits.parameters_per_item = limit,
            ParserLimitKind::OperandsPerOperation => limits.operands_per_operation = limit,
            ParserLimitKind::Statements => limits.statements = limit,
            ParserLimitKind::TotalWork => limits.total_work = limit,
        }

        limits
    }

    #[test]
    fn default_and_unlimited_policies_have_no_explicit_ceilings() {
        assert_eq!(ParserLimits::default(), ParserLimits::unlimited());

        let budget = ParserBudget::default();
        for kind in [
            ParserLimitKind::SourceBytes,
            ParserLimitKind::Tokens,
            ParserLimitKind::AstNodes,
            ParserLimitKind::NestingDepth,
            ParserLimitKind::ExpressionDepth,
            ParserLimitKind::Diagnostics,
            ParserLimitKind::IdentifierBytes,
            ParserLimitKind::NumericLiteralBytes,
            ParserLimitKind::StringLiteralBytes,
            ParserLimitKind::IncludeDepth,
            ParserLimitKind::GateDefinitions,
            ParserLimitKind::GateOperations,
            ParserLimitKind::RegisterSize,
            ParserLimitKind::ParametersPerItem,
            ParserLimitKind::OperandsPerOperation,
            ParserLimitKind::Statements,
            ParserLimitKind::TotalWork,
        ] {
            assert!(budget.check(kind, usize::MAX).is_ok(), "{kind}");
        }
    }

    #[test]
    fn configured_limit_is_inclusive() {
        let budget = ParserBudget::new(limits_with(ParserLimitKind::Tokens, 3));

        assert!(budget.check(ParserLimitKind::Tokens, 3).is_ok());

        let error = budget
            .check(ParserLimitKind::Tokens, 4)
            .expect_err("value above the budget must fail");

        assert_eq!(error.resource(), ParserLimitKind::Tokens);
        assert_eq!(error.observed(), Some(4));
        assert_eq!(error.failure(), ParserLimitFailure::Exceeded);
    }

    #[test]
    fn cumulative_charges_are_checked_before_mutation() {
        let mut budget = ParserBudget::new(limits_with(ParserLimitKind::AstNodes, 3));

        budget.charge(ParserLimitKind::AstNodes, 2).unwrap();

        let error = budget
            .charge(ParserLimitKind::AstNodes, 2)
            .expect_err("the charge must exceed the budget");

        assert_eq!(error.observed(), Some(4));
        assert_eq!(budget.usage().get(ParserLimitKind::AstNodes), 2);
    }

    #[test]
    fn zero_charge_is_valid_and_does_not_change_usage() {
        let mut budget = ParserBudget::default();

        budget.charge(ParserLimitKind::Tokens, 0).unwrap();

        assert_eq!(budget.usage().get(ParserLimitKind::Tokens), 0);
    }

    #[test]
    fn source_size_is_set_not_accumulated() {
        let mut budget = ParserBudget::new(limits_with(ParserLimitKind::SourceBytes, 8));

        budget.set_source_bytes(6).unwrap();
        budget.set_source_bytes(8).unwrap();

        assert_eq!(budget.usage().get(ParserLimitKind::SourceBytes), 8);

        assert!(budget.set_source_bytes(9).is_err());
        assert_eq!(budget.usage().get(ParserLimitKind::SourceBytes), 8);
    }

    #[test]
    fn observed_depth_tracks_the_high_water_mark() {
        let mut budget = ParserBudget::new(limits_with(ParserLimitKind::NestingDepth, 4));

        budget.observe_max(ParserLimitKind::NestingDepth, 2).unwrap();
        budget.observe_max(ParserLimitKind::NestingDepth, 4).unwrap();
        budget.observe_max(ParserLimitKind::NestingDepth, 1).unwrap();

        assert_eq!(budget.usage().get(ParserLimitKind::NestingDepth), 4);

        assert!(budget.observe_max(ParserLimitKind::NestingDepth, 5).is_err());
        assert_eq!(budget.usage().get(ParserLimitKind::NestingDepth), 4);
    }

    #[test]
    fn individual_item_dimensions_use_maximum_observation() {
        let mut budget = ParserBudget::new(limits_with(ParserLimitKind::IdentifierBytes, 12));

        budget.observe_max(ParserLimitKind::IdentifierBytes, 7).unwrap();
        budget.observe_max(ParserLimitKind::IdentifierBytes, 12).unwrap();

        assert_eq!(budget.usage().get(ParserLimitKind::IdentifierBytes), 12);
        assert!(budget.observe_max(ParserLimitKind::IdentifierBytes, 13).is_err());
    }

    #[test]
    fn cumulative_resources_reject_maximum_observation() {
        let mut budget = ParserBudget::default();

        let error = budget
            .observe_max(ParserLimitKind::Tokens, 1)
            .expect_err("tokens are cumulative");

        assert_eq!(error.failure(), ParserLimitFailure::InvalidAccountingOperation);
    }

    #[test]
    fn maximum_resources_reject_cumulative_charges() {
        let mut budget = ParserBudget::default();

        let error = budget
            .charge(ParserLimitKind::NestingDepth, 1)
            .expect_err("nesting depth is a maximum");

        assert_eq!(error.failure(), ParserLimitFailure::InvalidAccountingOperation);
    }

    #[test]
    fn counter_overflow_does_not_modify_usage() {
        let mut budget = ParserBudget::default();

        budget
            .charge(ParserLimitKind::Tokens, usize::MAX)
            .expect("unlimited budget accepts representable usage");

        let error = budget
            .charge(ParserLimitKind::Tokens, 1)
            .expect_err("arithmetic must not wrap");

        assert_eq!(error.failure(), ParserLimitFailure::CounterOverflow);
        assert_eq!(error.observed(), None);
        assert_eq!(budget.usage().get(ParserLimitKind::Tokens), usize::MAX);
    }

    #[test]
    fn diagnostics_are_accounted_as_cumulative_work() {
        let mut budget =
            ParserBudget::new(limits_with(ParserLimitKind::Diagnostics, 1));

        budget.charge(ParserLimitKind::Diagnostics, 1).unwrap();

        assert!(budget.charge(ParserLimitKind::Diagnostics, 1).is_err());
        assert_eq!(budget.usage().get(ParserLimitKind::Diagnostics), 1);
    }

    #[test]
    fn reset_clears_usage_and_preserves_policy() {
        let limits = limits_with(ParserLimitKind::Tokens, 2);
        let mut budget = ParserBudget::new(limits);

        budget.charge(ParserLimitKind::Tokens, 2).unwrap();
        budget.reset();

        assert_eq!(budget.usage(), ParserUsage::default());
        assert_eq!(budget.limits(), &limits);
        assert!(budget.charge(ParserLimitKind::Tokens, 2).is_ok());
        assert!(budget.charge(ParserLimitKind::Tokens, 1).is_err());
    }

    #[test]
    fn every_limit_has_a_stable_nonempty_identifier() {
        for kind in [
            ParserLimitKind::SourceBytes,
            ParserLimitKind::Tokens,
            ParserLimitKind::AstNodes,
            ParserLimitKind::NestingDepth,
            ParserLimitKind::ExpressionDepth,
            ParserLimitKind::Diagnostics,
            ParserLimitKind::IdentifierBytes,
            ParserLimitKind::NumericLiteralBytes,
            ParserLimitKind::StringLiteralBytes,
            ParserLimitKind::IncludeDepth,
            ParserLimitKind::GateDefinitions,
            ParserLimitKind::GateOperations,
            ParserLimitKind::RegisterSize,
            ParserLimitKind::ParametersPerItem,
            ParserLimitKind::OperandsPerOperation,
            ParserLimitKind::Statements,
            ParserLimitKind::TotalWork,
        ] {
            assert!(!kind.as_str().is_empty());
            assert_eq!(kind.to_string(), kind.as_str());
        }
    }

    #[test]
    fn resource_limit_reuses_the_lexer_canonical_type() {
        let limit = ResourceLimit::AtMost(
            NonZeroUsize::new(5).expect("non-zero test budget"),
        );

        let limits = ParserLimits {
            tokens: limit,
            ..ParserLimits::unlimited()
        };

        assert_eq!(limits.tokens, limit);
    }
}
