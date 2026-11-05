
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation Budgets.
//!
//! Path:
//!     src/compiler/configuration/budget.rs
//!
//! Rust:
//!     Rust 1.97+
//!     Edition 2021
//!
//! Safety:
//!     Safe Rust only. Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! This module defines configurable operational budgets for compiler work.
//!
//! It allows compiler stages to account for resource consumption without
//! imposing universal language-level or hardware-capacity limits.
//!
//! Examples of compiler work that may be budgeted include:
//!
//! - Source bytes processed.
//! - Intermediate-representation entities processed.
//! - Memory accounted for by a compiler stage.
//! - Worker slots used by a compilation operation.
//! - Optimization-pass applications.
//! - Constant-evaluation work.
//! - Diagnostics retained.
//! - Artifact bytes generated.
//! - Compilation wall-clock time.
//! - Stage-specific work units.
//! - Extension-defined resource dimensions.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Compiler budget dimensions.
//! - Optional operational limits.
//! - Immutable budget configuration.
//! - Shared accounting across compiler stages.
//! - Checked consumption.
//! - Remaining-budget queries.
//! - Deterministic usage snapshots.
//! - Explicit budget-exhaustion errors.
//!
//! This module does not own:
//!
//! - Source-level Zamani resource-budget semantics.
//! - Semantic requirements, capabilities, constraints, or preferences.
//! - Hardware discovery or target capability detection.
//! - Physical memory allocation.
//! - Device allocation or placement.
//! - Thread scheduling or worker creation.
//! - Runtime resource management.
//! - IR definitions or IR verification.
//! - Optimization legality.
//! - Backend implementation.
//! - Source parsing or expression evaluation.
//! - Security authorization.
//!
//! # POCO-REAF and scalability
//!
//! An absent budget means that this configuration imposes no explicit
//! compiler budget for that dimension.
//!
//! It does not promise infinite physical memory, execution time, or
//! representational capacity.
//!
//! Actual host resources, operating-system limits, process limits, target
//! capabilities, and security policies remain applicable.
//!
//! A budget is an operational policy, not a language restriction.
//!
//! No universal maximum is defined here for:
//!
//! - CPUs, cores, threads, workers, devices, or nodes.
//! - Memory, storage, or address-space capacity.
//! - Qubits or logical qubits.
//! - GPUs, FPGAs, QPUs, or accelerators.
//! - Tensor rank or tensor dimensions.
//! - IR operations, blocks, functions, or modules.
//! - Source length or program size.
//! - Generated artifact size.
//! - Optimization passes.
//! - Compiler stages.
//!
//! # Enforcement contract
//!
//! This module accounts for work only when a caller reports that work.
//!
//! It does not automatically intercept allocations, threads, I/O, or CPU
//! instructions.
//!
//! Every enforcing compiler stage must:
//!
//! 1. Identify the budget dimension it consumes.
//! 2. Measure the relevant work using the documented unit.
//! 3. Account for cumulative consumption.
//! 4. Stop or recover according to its own stage contract when exhaustion
//!    occurs.
//! 5. Return an explicit error or diagnostic.
//! 6. Never silently truncate a program or claim successful completion.
//!
//! A stage must not treat an unknown measurement as zero.
//!
//! # Determinism
//!
//! Budget dimensions are stored in ordered maps.
//!
//! Identical budget configurations and identical consumption sequences
//! produce identical snapshots and exhaustion decisions.
//!
//! This module performs no filesystem, network, environment, clock,
//! hardware-discovery, or random-number operations.
//!
//! # Concurrency
//!
//! `BudgetTracker` is cloneable and shares one accounting ledger.
//!
//! Clones therefore account against the same configured budget.
//!
//! Consumption is checked and committed while holding the ledger lock.
//! Concurrent callers cannot independently consume the same remaining
//! budget.
//!
//! # Numeric representation
//!
//! Budget quantities use `u128` so accounting is not tied to the host's
//! pointer width.
//!
//! `u128` is a representation type, not a universal Zamani resource limit.
//! Arithmetic overflow is reported explicitly rather than wrapping or
//! silently saturating.
//!
//! Time quantities are represented in nanoseconds when using the built-in
//! time dimensions.
//!
//! # Integration
//!
//! - `src/compiler/mod.rs`: expose `configuration`.
//! - `src/compiler/configuration.rs`: expose this module.
//! - Compiler API/session: construct a `BudgetTracker` for a compilation.
//! - Pipeline execution: share tracker clones with stages.
//! - Parallel compilation: share the same tracker across worker tasks.
//! - Optimization: account for pass applications and analysis work.
//! - IR processing: account for IR entities and processing work.
//! - Diagnostics: account for retained diagnostic entries.
//! - Artifact generation: account for generated artifact bytes.
//! - Incremental compilation: include budget configuration in cache identity
//!   whenever a budget can change whether or how an artifact is produced.
//! - Backend integration: report generated bytes and other measurable work.
//! - Runtime integration: use runtime-owned resource contracts rather than
//!   treating compiler budgets as runtime allocation limits.
//!
//! The module intentionally does not depend on any of those subsystems.
//!
//! # Freeze criteria
//!
//! This file is ready to freeze when:
//!
//! - Its unit tests pass.
//! - The compiler configuration module exposes it.
//! - Compiler stages use its shared accounting contract.
//! - No stage silently ignores a required budget failure.
//! - Budget configuration is included in relevant cache fingerprints.
//! - Existing domain-specific budget contracts remain authoritative for
//!   their own subsystems.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

/// Result type for compiler budget operations.
pub type BudgetResult<T> = Result<T, BudgetError>;

/// Validates a budget dimension name or unit.
///
/// Names must be non-empty and must not contain control characters.
///
/// No fixed string-length limit is imposed.
fn validate_text(value: &str, field: &'static str) -> BudgetResult<()> {
    if value.trim().is_empty() {
        return Err(BudgetError::InvalidDimension {
            field,
            reason: "value cannot be empty or whitespace-only".to_string(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(BudgetError::InvalidDimension {
            field,
            reason: "value cannot contain control characters".to_string(),
        });
    }

    Ok(())
}

/// Identifies one measurable compiler-budget dimension.
///
/// The name identifies what is being accounted for.
///
/// The unit identifies the measurement unit.
///
/// Including the unit in the dimension identity prevents accidental
/// comparison of quantities expressed in incompatible units.
///
/// Examples:
///
/// - `memory` / `bytes`
/// - `artifact` / `bytes`
/// - `workers` / `slots`
/// - `wall_time` / `nanoseconds`
/// - `ir_entities` / `entities`
///
/// Custom dimensions allow new compiler stages and extensions to introduce
/// additional measurable work without modifying this type.
///
/// A dimension is an accounting identity. It is not proof that a particular
/// resource is available or physically allocated.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BudgetDimension {
    name: String,
    unit: String,
}

impl BudgetDimension {
    /// Creates a new budget dimension.
    ///
    /// Both the name and unit must be non-empty and contain no control
    /// characters.
    pub fn new(
        name: impl Into<String>,
        unit: impl Into<String>,
    ) -> BudgetResult<Self> {
        let name = name.into();
        let unit = unit.into();

        validate_text(&name, "dimension name")?;
        validate_text(&unit, "dimension unit")?;

        Ok(Self { name, unit })
    }

    /// Returns the dimension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the measurement unit.
    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Source bytes processed by a compiler stage.
    #[must_use]
    pub fn input_bytes() -> Self {
        Self::builtin("input_bytes", "bytes")
    }

    /// Memory accounted for by a compiler stage.
    ///
    /// This is accounting, not a physical allocator or operating-system
    /// memory limit.
    #[must_use]
    pub fn memory_bytes() -> Self {
        Self::builtin("memory", "bytes")
    }

    /// Worker slots used by a compiler operation.
    ///
    /// This is a budget dimension, not a universal worker-count ceiling.
    #[must_use]
    pub fn worker_slots() -> Self {
        Self::builtin("worker_slots", "slots")
    }

    /// Intermediate-representation entities processed.
    #[must_use]
    pub fn ir_entities() -> Self {
        Self::builtin("ir_entities", "entities")
    }

    /// Optimization-pass applications.
    #[must_use]
    pub fn pass_applications() -> Self {
        Self::builtin("pass_applications", "applications")
    }

    /// Constant-evaluation work units.
    #[must_use]
    pub fn constant_evaluation_work() -> Self {
        Self::builtin("constant_evaluation_work", "work_units")
    }

    /// Diagnostic entries retained.
    #[must_use]
    pub fn diagnostics() -> Self {
        Self::builtin("diagnostics", "entries")
    }

    /// Generated or serialized artifact bytes.
    #[must_use]
    pub fn artifact_bytes() -> Self {
        Self::builtin("artifact_bytes", "bytes")
    }

    /// Compilation wall-clock time.
    ///
    /// Values are nanoseconds.
    #[must_use]
    pub fn wall_time() -> Self {
        Self::builtin("wall_time", "nanoseconds")
    }

    /// CPU time accounted for by a compiler stage.
    ///
    /// Values are nanoseconds.
    #[must_use]
    pub fn cpu_time() -> Self {
        Self::builtin("cpu_time", "nanoseconds")
    }

    /// Parser work units.
    ///
    /// The parser defines what constitutes one work unit.
    #[must_use]
    pub fn parser_work() -> Self {
        Self::builtin("parser_work", "work_units")
    }

    /// Optimizer work units.
    ///
    /// The optimizer defines what constitutes one work unit.
    #[must_use]
    pub fn optimizer_work() -> Self {
        Self::builtin("optimizer_work", "work_units")
    }

    /// Creates a built-in dimension.
    ///
    /// Built-in values are constants in meaning, not capacity limits.
    fn builtin(name: &'static str, unit: &'static str) -> Self {
        Self {
            name: name.to_string(),
            unit: unit.to_string(),
        }
    }
}

impl fmt::Display for BudgetDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ({})", self.name, self.unit)
    }
}

/// One optional upper bound for a budget dimension.
///
/// `None` means no explicit limit is imposed by this budget.
///
/// `Some(0)` is a valid limit and permits zero consumption.
///
/// A bounded value is an operational configuration value, not a language
/// semantic limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BudgetLimit {
    limit: Option<u128>,
}

impl BudgetLimit {
    /// Creates an unlimited budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { limit: None }
    }

    /// Creates a budget with an explicit maximum.
    #[must_use]
    pub const fn bounded(limit: u128) -> Self {
        Self {
            limit: Some(limit),
        }
    }

    /// Returns the configured limit.
    ///
    /// `None` represents no explicit limit.
    #[must_use]
    pub const fn value(self) -> Option<u128> {
        self.limit
    }

    /// Returns whether the budget has no explicit limit.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        self.limit.is_none()
    }

    /// Returns whether the supplied amount is permitted.
    #[must_use]
    pub const fn permits(self, amount: u128) -> bool {
        match self.limit {
            Some(limit) => amount <= limit,
            None => true,
        }
    }

    /// Returns the remaining amount after the supplied consumption.
    ///
    /// An unlimited budget returns `None`.
    ///
    /// A bounded budget returns `Some(0)` when fully consumed.
    #[must_use]
    pub const fn remaining(self, consumed: u128) -> Option<u128> {
        match self.limit {
            Some(limit) => Some(limit.saturating_sub(consumed)),
            None => None,
        }
    }
}

impl fmt::Display for BudgetLimit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.limit {
            Some(limit) => write!(formatter, "bounded({limit})"),
            None => formatter.write_str("unlimited"),
        }
    }
}

/// Immutable configuration for compiler operational budgets.
///
/// An omitted dimension is unlimited.
///
/// Dimensions are stored in an ordered map for deterministic behavior.
///
/// This type does not perform accounting. Use `BudgetTracker` to account
/// for consumption across compiler stages and concurrent workers.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompilationBudget {
    limits: BTreeMap<BudgetDimension, BudgetLimit>,
}

impl CompilationBudget {
    /// Creates a configuration with no explicit limits.
    #[must_use]
    pub fn unlimited() -> Self {
        Self::default()
    }

    /// Creates an empty configuration with the supplied dimension limits.
    ///
    /// This is equivalent to `unlimited()`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a dimension limit.
    ///
    /// Replacing a dimension is explicit and deterministic.
    #[must_use]
    pub fn with_limit(
        mut self,
        dimension: BudgetDimension,
        limit: BudgetLimit,
    ) -> Self {
        self.limits.insert(dimension, limit);
        self
    }

    /// Sets or replaces a dimension limit in place.
    ///
    /// Returns the previous limit, if one existed.
    pub fn set_limit(
        &mut self,
        dimension: BudgetDimension,
        limit: BudgetLimit,
    ) -> Option<BudgetLimit> {
        self.limits.insert(dimension, limit)
    }

    /// Removes an explicit limit.
    ///
    /// The removed dimension becomes unlimited.
    pub fn remove_limit(
        &mut self,
        dimension: &BudgetDimension,
    ) -> Option<BudgetLimit> {
        self.limits.remove(dimension)
    }

    /// Returns the configured limit for a dimension.
    ///
    /// A missing dimension is unlimited.
    #[must_use]
    pub fn limit(&self, dimension: &BudgetDimension) -> BudgetLimit {
        self.limits
            .get(dimension)
            .copied()
            .unwrap_or_else(BudgetLimit::unlimited)
    }

    /// Returns a read-only view of all explicitly configured limits.
    #[must_use]
    pub fn limits(&self) -> &BTreeMap<BudgetDimension, BudgetLimit> {
        &self.limits
    }

    /// Returns whether this configuration contains no explicit limits.
    #[must_use]
    pub fn is_unlimited(&self) -> bool {
        self.limits.is_empty()
    }

    /// Checks a measured amount without changing accounting state.
    pub fn check(
        &self,
        dimension: &BudgetDimension,
        consumed: u128,
        additional: u128,
    ) -> BudgetResult<()> {
        let attempted = consumed
            .checked_add(additional)
            .ok_or_else(|| BudgetError::AccountingOverflow {
                dimension: dimension.clone(),
            })?;

        let limit = self.limit(dimension);

        if limit.permits(attempted) {
            return Ok(());
        }

        Err(BudgetError::Exceeded {
            dimension: dimension.clone(),
            consumed,
            requested: additional,
            attempted,
            limit: limit.value().unwrap_or_default(),
        })
    }

    /// Creates a new shared accounting tracker for this configuration.
    ///
    /// The configuration is cloned into the tracker and cannot be changed
    /// after tracking begins.
    #[must_use]
    pub fn tracker(&self) -> BudgetTracker {
        BudgetTracker::new(self.clone())
    }
}

/// A deterministic snapshot of consumption for one dimension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetUsage {
    /// Dimension being measured.
    pub dimension: BudgetDimension,

    /// Amount already consumed.
    pub consumed: u128,

    /// Configured maximum, if bounded.
    pub limit: Option<u128>,

    /// Remaining amount, if bounded.
    ///
    /// A value of `Some(0)` means the budget is fully consumed.
    /// `None` means the dimension is unlimited.
    pub remaining: Option<u128>,
}

impl BudgetUsage {
    /// Returns whether the configured limit has been reached.
    ///
    /// An unlimited budget is never considered exhausted.
    #[must_use]
    pub const fn is_exhausted(&self) -> bool {
        match self.limit {
            Some(limit) => self.consumed >= limit,
            None => false,
        }
    }
}

/// A deterministic snapshot of all tracked compiler budget usage.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BudgetSnapshot {
    usage: BTreeMap<BudgetDimension, BudgetUsage>,
}

impl BudgetSnapshot {
    /// Returns the usage entry for a dimension.
    #[must_use]
    pub fn usage(&self, dimension: &BudgetDimension) -> Option<&BudgetUsage> {
        self.usage.get(dimension)
    }

    /// Returns all dimensions that have been consumed.
    #[must_use]
    pub fn dimensions(&self) -> &BTreeMap<BudgetDimension, BudgetUsage> {
        &self.usage
    }

    /// Returns the number of dimensions with recorded consumption.
    #[must_use]
    pub fn dimension_count(&self) -> usize {
        self.usage.len()
    }

    /// Returns whether no consumption has been recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.usage.is_empty()
    }
}

/// Internal shared accounting state.
#[derive(Debug, Default)]
struct BudgetLedger {
    consumed: BTreeMap<BudgetDimension, u128>,
}

/// Shared, thread-safe accounting for a compilation.
///
/// Cloning a tracker shares the same ledger and configuration.
///
/// This is important for parallel compilation: independent workers must
/// not each receive a fresh copy of the entire budget.
#[derive(Debug, Clone)]
pub struct BudgetTracker {
    budget: Arc<CompilationBudget>,
    ledger: Arc<Mutex<BudgetLedger>>,
}

impl BudgetTracker {
    /// Creates a tracker from an immutable budget configuration.
    #[must_use]
    pub fn new(budget: CompilationBudget) -> Self {
        Self {
            budget: Arc::new(budget),
            ledger: Arc::new(Mutex::new(BudgetLedger::default())),
        }
    }

    /// Returns the immutable budget configuration.
    #[must_use]
    pub fn budget(&self) -> &CompilationBudget {
        &self.budget
    }

    /// Returns a clone that shares the same accounting ledger.
    #[must_use]
    pub fn shared(&self) -> Self {
        self.clone()
    }

    /// Records consumption against a dimension.
    ///
    /// The update is atomic with respect to other tracker clones.
    ///
    /// If the requested consumption would exceed the configured limit,
    /// the ledger is not modified.
    ///
    /// If accounting arithmetic overflows, the ledger is not modified.
    pub fn consume(
        &self,
        dimension: BudgetDimension,
        amount: u128,
    ) -> BudgetResult<u128> {
        let mut ledger = self
            .ledger
            .lock()
            .map_err(|_| BudgetError::LedgerPoisoned)?;

        let consumed = ledger
            .consumed
            .get(&dimension)
            .copied()
            .unwrap_or(0);

        self.budget.check(&dimension, consumed, amount)?;

        let updated = consumed
            .checked_add(amount)
            .ok_or_else(|| BudgetError::AccountingOverflow {
                dimension: dimension.clone(),
            })?;

        ledger.consumed.insert(dimension, updated);

        Ok(updated)
    }

    /// Checks whether additional consumption would be permitted.
    ///
    /// This operation does not reserve capacity. Another concurrent caller
    /// may consume the remaining budget after this method returns.
    ///
    /// Use `consume` for an atomic check-and-account operation.
    pub fn check(
        &self,
        dimension: &BudgetDimension,
        additional: u128,
    ) -> BudgetResult<()> {
        let ledger = self
            .ledger
            .lock()
            .map_err(|_| BudgetError::LedgerPoisoned)?;

        let consumed = ledger
            .consumed
            .get(dimension)
            .copied()
            .unwrap_or(0);

        self.budget.check(dimension, consumed, additional)
    }

    /// Returns the amount already consumed for a dimension.
    pub fn consumed(
        &self,
        dimension: &BudgetDimension,
    ) -> BudgetResult<u128> {
        let ledger = self
            .ledger
            .lock()
            .map_err(|_| BudgetError::LedgerPoisoned)?;

        Ok(ledger
            .consumed
            .get(dimension)
            .copied()
            .unwrap_or(0))
    }

    /// Returns the remaining configured budget for a dimension.
    ///
    /// An unlimited dimension returns `None`.
    pub fn remaining(
        &self,
        dimension: &BudgetDimension,
    ) -> BudgetResult<Option<u128>> {
        let consumed = self.consumed(dimension)?;

        Ok(self.budget.limit(dimension).remaining(consumed))
    }

    /// Returns whether a dimension has reached its configured limit.
    ///
    /// An unlimited dimension is never exhausted.
    pub fn is_exhausted(
        &self,
        dimension: &BudgetDimension,
    ) -> BudgetResult<bool> {
        let consumed = self.consumed(dimension)?;
        let limit = self.budget.limit(dimension);

        Ok(match limit.value() {
            Some(maximum) => consumed >= maximum,
            None => false,
        })
    }

    /// Returns a deterministic snapshot of all dimensions with recorded
    /// consumption.
    pub fn snapshot(&self) -> BudgetResult<BudgetSnapshot> {
        let ledger = self
            .ledger
            .lock()
            .map_err(|_| BudgetError::LedgerPoisoned)?;

        let mut usage = BTreeMap::new();

        for (dimension, consumed) in &ledger.consumed {
            let limit = self.budget.limit(dimension).value();

            let remaining = limit.map(|maximum| {
                maximum.saturating_sub(*consumed)
            });

            usage.insert(
                dimension.clone(),
                BudgetUsage {
                    dimension: dimension.clone(),
                    consumed: *consumed,
                    limit,
                    remaining,
                },
            );
        }

        Ok(BudgetSnapshot { usage })
    }

    /// Returns the total number of dimensions with recorded consumption.
    pub fn tracked_dimension_count(&self) -> BudgetResult<usize> {
        let ledger = self
            .ledger
            .lock()
            .map_err(|_| BudgetError::LedgerPoisoned)?;

        Ok(ledger.consumed.len())
    }
}

/// Errors produced by compiler budget configuration and accounting.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BudgetError {
    /// A dimension name or unit was invalid.
    InvalidDimension {
        /// The invalid field.
        field: &'static str,

        /// Explanation of the invalid value.
        reason: String,
    },

    /// A requested amount would exceed a configured limit.
    Exceeded {
        /// Dimension that exceeded its budget.
        dimension: BudgetDimension,

        /// Amount already consumed.
        consumed: u128,

        /// Additional amount requested.
        requested: u128,

        /// Total amount that would have been consumed.
        attempted: u128,

        /// Configured maximum.
        limit: u128,
    },

    /// Accounting arithmetic exceeded the representation range.
    AccountingOverflow {
        /// Dimension whose accounting overflowed.
        dimension: BudgetDimension,
    },

    /// The shared accounting mutex was poisoned.
    ///
    /// The tracker refuses to continue because it cannot establish that
    /// its accounting state is trustworthy.
    LedgerPoisoned,
}

impl fmt::Display for BudgetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimension { field, reason } => {
                write!(
                    formatter,
                    "invalid compiler budget {field}: {reason}"
                )
            }

            Self::Exceeded {
                dimension,
                consumed,
                requested,
                attempted,
                limit,
            } => {
                write!(
                    formatter,
                    "compiler budget exceeded for {dimension}: \
                     consumed {consumed}, requested {requested}, \
                     attempted {attempted}, configured limit {limit}"
                )
            }

            Self::AccountingOverflow { dimension } => {
                write!(
                    formatter,
                    "compiler budget accounting overflow for {dimension}"
                )
            }

            Self::LedgerPoisoned => {
                formatter.write_str(
                    "compiler budget accounting ledger is poisoned"
                )
            }
        }
    }
}

impl Error for BudgetError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn default_budget_is_unlimited() {
        let budget = CompilationBudget::default();

        assert!(budget.is_unlimited());

        let dimension = BudgetDimension::memory_bytes();

        assert!(budget.limit(&dimension).is_unlimited());
        assert!(budget.check(&dimension, 0, u128::MAX).is_ok());
    }

    #[test]
    fn explicit_zero_budget_is_valid() {
        let dimension = BudgetDimension::input_bytes();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(0),
        );

        assert!(budget.check(&dimension, 0, 0).is_ok());

        assert!(matches!(
            budget.check(&dimension, 0, 1),
            Err(BudgetError::Exceeded { .. })
        ));
    }

    #[test]
    fn permits_consumption_within_limit() {
        let dimension = BudgetDimension::artifact_bytes();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(100),
        );

        assert!(budget.check(&dimension, 20, 30).is_ok());
    }

    #[test]
    fn rejects_consumption_above_limit() {
        let dimension = BudgetDimension::memory_bytes();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(100),
        );

        let error = budget
            .check(&dimension, 80, 21)
            .expect_err("budget must be exceeded");

        assert!(matches!(
            error,
            BudgetError::Exceeded {
                consumed: 80,
                requested: 21,
                attempted: 101,
                limit: 100,
                ..
            }
        ));
    }

    #[test]
    fn missing_dimension_is_unlimited() {
        let budget = CompilationBudget::unlimited();
        let dimension = BudgetDimension::worker_slots();

        assert!(budget.check(&dimension, 0, u128::MAX).is_ok());
    }

    #[test]
    fn dimensions_are_extensible() {
        let dimension = BudgetDimension::new(
            "future_domain.custom_work",
            "work_units",
        )
        .expect("custom dimension should be valid");

        assert_eq!(dimension.name(), "future_domain.custom_work");
        assert_eq!(dimension.unit(), "work_units");
    }

    #[test]
    fn dimension_units_are_part_of_identity() {
        let bytes = BudgetDimension::new("memory", "bytes")
            .expect("valid dimension");

        let kibibytes = BudgetDimension::new("memory", "kibibytes")
            .expect("valid dimension");

        assert_ne!(bytes, kibibytes);
    }

    #[test]
    fn rejects_empty_dimension_name() {
        let result = BudgetDimension::new("", "bytes");

        assert!(matches!(
            result,
            Err(BudgetError::InvalidDimension { .. })
        ));
    }

    #[test]
    fn rejects_empty_dimension_unit() {
        let result = BudgetDimension::new("memory", "");

        assert!(matches!(
            result,
            Err(BudgetError::InvalidDimension { .. })
        ));
    }

    #[test]
    fn rejects_control_characters() {
        let result = BudgetDimension::new(
            "memory\ninvalid",
            "bytes",
        );

        assert!(matches!(
            result,
            Err(BudgetError::InvalidDimension { .. })
        ));
    }

    #[test]
    fn does_not_impose_an_arbitrary_name_length_limit() {
        let long_name = "domain.".to_string() + &"x".repeat(16_384);

        let dimension = BudgetDimension::new(
            long_name.clone(),
            "work_units",
        )
        .expect("long valid names must not be rejected by a fixed ceiling");

        assert_eq!(dimension.name(), long_name);
    }

    #[test]
    fn tracker_accounts_consumption() {
        let dimension = BudgetDimension::pass_applications();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(10),
        );

        let tracker = budget.tracker();

        assert_eq!(
            tracker.consume(dimension.clone(), 4).unwrap(),
            4
        );

        assert_eq!(
            tracker.consume(dimension.clone(), 3).unwrap(),
            7
        );

        assert_eq!(tracker.consumed(&dimension).unwrap(), 7);
        assert_eq!(tracker.remaining(&dimension).unwrap(), Some(3));
    }

    #[test]
    fn failed_consumption_does_not_modify_ledger() {
        let dimension = BudgetDimension::diagnostics();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(5),
        );

        let tracker = budget.tracker();

        tracker
            .consume(dimension.clone(), 4)
            .expect("initial consumption should succeed");

        assert!(matches!(
            tracker.consume(dimension.clone(), 2),
            Err(BudgetError::Exceeded { .. })
        ));

        assert_eq!(tracker.consumed(&dimension).unwrap(), 4);
    }

    #[test]
    fn tracker_clones_share_the_same_ledger() {
        let dimension = BudgetDimension::worker_slots();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(8),
        );

        let first = budget.tracker();
        let second = first.shared();

        first
            .consume(dimension.clone(), 5)
            .expect("first consumption");

        assert_eq!(second.consumed(&dimension).unwrap(), 5);

        assert!(matches!(
            second.consume(dimension.clone(), 4),
            Err(BudgetError::Exceeded { .. })
        ));
    }

    #[test]
    fn concurrent_consumption_cannot_exceed_shared_limit() {
        let dimension = BudgetDimension::ir_entities();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(100),
        );

        let tracker = Arc::new(budget.tracker());

        let mut workers = Vec::new();

        for _ in 0..10 {
            let tracker = Arc::clone(&tracker);
            let dimension = dimension.clone();

            workers.push(thread::spawn(move || {
                tracker.consume(dimension, 15)
            }));
        }

        let mut successful = 0_u128;

        for worker in workers {
            if worker
                .join()
                .expect("worker must not panic")
                .is_ok()
            {
                successful += 15;
            }
        }

        assert_eq!(successful, 90);
        assert_eq!(tracker.consumed(&dimension).unwrap(), 90);
    }

    #[test]
    fn accounting_overflow_is_reported() {
        let dimension = BudgetDimension::optimizer_work();

        let budget = CompilationBudget::unlimited();
        let tracker = budget.tracker();

        tracker
            .consume(dimension.clone(), u128::MAX)
            .expect("maximum representable amount");

        assert!(matches!(
            tracker.consume(dimension, 1),
            Err(BudgetError::AccountingOverflow { .. })
        ));
    }

    #[test]
    fn snapshot_is_deterministic_and_contains_usage() {
        let first_dimension = BudgetDimension::input_bytes();
        let second_dimension = BudgetDimension::artifact_bytes();

        let budget = CompilationBudget::new()
            .with_limit(
                first_dimension.clone(),
                BudgetLimit::bounded(100),
            )
            .with_limit(
                second_dimension.clone(),
                BudgetLimit::bounded(200),
            );

        let tracker = budget.tracker();

        tracker
            .consume(second_dimension.clone(), 50)
            .expect("artifact accounting");

        tracker
            .consume(first_dimension.clone(), 20)
            .expect("input accounting");

        let snapshot = tracker.snapshot().unwrap();

        assert_eq!(snapshot.dimension_count(), 2);

        assert_eq!(
            snapshot.usage(&first_dimension).unwrap().remaining,
            Some(80)
        );

        assert_eq!(
            snapshot.usage(&second_dimension).unwrap().remaining,
            Some(150)
        );
    }

    #[test]
    fn unlimited_budget_has_no_remaining_amount() {
        let dimension = BudgetDimension::cpu_time();
        let tracker = CompilationBudget::unlimited().tracker();

        tracker
            .consume(dimension.clone(), 1_000)
            .expect("unlimited consumption");

        assert_eq!(tracker.remaining(&dimension).unwrap(), None);
        assert!(!tracker.is_exhausted(&dimension).unwrap());
    }

    #[test]
    fn bounded_budget_reports_exhaustion() {
        let dimension = BudgetDimension::wall_time();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(10),
        );

        let tracker = budget.tracker();

        tracker
            .consume(dimension.clone(), 10)
            .expect("consume complete budget");

        assert!(tracker.is_exhausted(&dimension).unwrap());
    }

    #[test]
    fn replacing_a_limit_is_explicit() {
        let dimension = BudgetDimension::memory_bytes();

        let mut budget = CompilationBudget::new();

        assert_eq!(
            budget.set_limit(
                dimension.clone(),
                BudgetLimit::bounded(100),
            ),
            None
        );

        assert_eq!(
            budget.set_limit(
                dimension.clone(),
                BudgetLimit::bounded(200),
            ),
            Some(BudgetLimit::bounded(100))
        );

        assert_eq!(
            budget.limit(&dimension),
            BudgetLimit::bounded(200)
        );
    }

    #[test]
    fn removing_a_limit_makes_dimension_unlimited() {
        let dimension = BudgetDimension::artifact_bytes();

        let mut budget = CompilationBudget::new();

        budget.set_limit(
            dimension.clone(),
            BudgetLimit::bounded(100),
        );

        assert_eq!(
            budget.remove_limit(&dimension),
            Some(BudgetLimit::bounded(100))
        );

        assert!(budget.limit(&dimension).is_unlimited());
    }

    #[test]
    fn time_dimensions_use_explicit_nanosecond_units() {
        assert_eq!(
            BudgetDimension::wall_time().unit(),
            "nanoseconds"
        );

        assert_eq!(
            BudgetDimension::cpu_time().unit(),
            "nanoseconds"
        );
    }

    #[test]
    fn budget_error_has_a_useful_display_message() {
        let dimension = BudgetDimension::memory_bytes();

        let error = BudgetError::Exceeded {
            dimension,
            consumed: 80,
            requested: 30,
            attempted: 110,
            limit: 100,
        };

        assert!(error.to_string().contains("memory"));
        assert!(error.to_string().contains("110"));
        assert!(error.to_string().contains("100"));
    }
}
