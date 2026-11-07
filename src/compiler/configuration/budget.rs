
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
//! # Purpose
//!
//! Provides configurable operational budgets and shared accounting
//! for one compilation operation.
//!
//! A budget controls compiler work. It does not define language
//! semantics, physical hardware capacity, or runtime allocation.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Budget dimension identities.
//! - Optional operational limits.
//! - Immutable budget configurations.
//! - Shared concurrent accounting.
//! - Atomic single-dimension consumption.
//! - Transactional multi-dimension consumption.
//! - Remaining-budget queries.
//! - Deterministic usage snapshots.
//! - Explicit exhaustion and accounting errors.
//!
//! It does not own:
//!
//! - Source-level resource requirements.
//! - Hardware capabilities.
//! - Physical allocation.
//! - Device discovery.
//! - Runtime scheduling.
//! - Backend implementation.
//! - Semantic validation.
//! - IR definitions.
//! - Security authorization.
//!
//! # Scalability
//!
//! No default operational limits are imposed.
//!
//! An omitted dimension is unlimited by this budget.
//!
//! This does not mean physical resources are infinite.
//!
//! Actual resource availability, operating-system limits, target
//! capabilities, and explicitly configured security policies remain
//! applicable.
//!
//! No universal maximum is established for:
//!
//! - Workers, CPUs, threads, or nodes.
//! - Memory or storage.
//! - GPUs, FPGAs, QPUs, or accelerators.
//! - Qubits or logical registers.
//! - Tensor dimensions or rank.
//! - Modules, functions, or IR operations.
//! - Source or artifact size.
//! - Compiler stages or optimization passes.
//!
//! # Accounting
//!
//! Quantities use u128.
//!
//! This provides a wide, pointer-independent accounting representation.
//! It is not an unlimited mathematical integer.
//!
//! Arithmetic overflow is always reported explicitly.
//!
//! # Concurrency
//!
//! Cloned trackers share one configuration and one accounting ledger.
//!
//! Every consumption operation is synchronized.
//!
//! Failed operations do not partially modify the ledger.
//!
//! # Determinism
//!
//! Dimensions and usage entries are ordered by BTreeMap.
//!
//! Snapshots are deterministic for identical configurations and
//! consumption histories.
//!
//! This module performs no environment, clock, filesystem, network,
//! hardware-discovery, or random-number operations.
//!
//! # Integration
//!
//! configuration/mod.rs:
//!     Exposes this module and its public types.
//!
//! context/compilation_context.rs:
//!     Owns a tracker shared by compilation stages.
//!
//! pipeline/execution.rs:
//!     Accounts for measurable stage work.
//!
//! parallel/:
//!     Shares the same tracker across workers.
//!
//! analysis/:
//!     Accounts for explicitly measured analysis work.
//!
//! optimization/:
//!     Accounts for optimization work.
//!
//! lowering/:
//!     Accounts for lowering work.
//!
//! artifacts/:
//!     Accounts for generated artifact bytes.
//!
//! incremental/:
//!     Includes relevant budget configuration in cache identity.
//!
//! runtime/:
//!     Uses runtime-owned allocation contracts, not this compiler
//!     accounting model.
//!
//! # Freeze criteria
//!
//! This file is ready to freeze after:
//!
//! - Public API compatibility is checked.
//! - Unit tests pass.
//! - Existing context integration passes.
//! - Concurrent accounting tests pass.
//! - Transactional accounting tests pass.
//! - All relevant callers propagate budget errors.
//! - No caller silently treats failed accounting as success.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard};

// ============================================================================
// RESULT
// ============================================================================

/// Result type for compiler budget operations.
pub type BudgetResult<T> = Result<T, BudgetError>;

// ============================================================================
// TEXT VALIDATION
// ============================================================================

fn validate_text(
    value: &str,
    field: &'static str,
) -> BudgetResult<()> {
    if value.trim().is_empty() {
        return Err(BudgetError::InvalidDimension {
            field,
            reason: "value cannot be empty or whitespace-only".into(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(BudgetError::InvalidDimension {
            field,
            reason: "value cannot contain control characters".into(),
        });
    }

    Ok(())
}

// ============================================================================
// DIMENSIONS
// ============================================================================

/// Identifies a measurable compiler-budget dimension.
///
/// The unit is part of the identity.
///
/// Therefore:
///
/// memory / bytes
///
/// and:
///
/// memory / kibibytes
///
/// are distinct accounting dimensions.
///
/// Custom dimensions permit new domains and extensions to introduce
/// additional accounting categories without modifying this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BudgetDimension {
    name: String,
    unit: String,
}

impl BudgetDimension {
    /// Creates a validated custom budget dimension.
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

    /// Source bytes processed.
    #[must_use]
    pub fn input_bytes() -> Self {
        Self::builtin("input_bytes", "bytes")
    }

    /// Memory accounted for by compiler stages.
    ///
    /// This is accounting, not physical allocation.
    #[must_use]
    pub fn memory_bytes() -> Self {
        Self::builtin("memory", "bytes")
    }

    /// Compiler worker slots.
    #[must_use]
    pub fn worker_slots() -> Self {
        Self::builtin("worker_slots", "slots")
    }

    /// Intermediate-representation entities processed.
    #[must_use]
    pub fn ir_entities() -> Self {
        Self::builtin("ir_entities", "entities")
    }

    /// Optimization pass applications.
    #[must_use]
    pub fn pass_applications() -> Self {
        Self::builtin("pass_applications", "applications")
    }

    /// Constant-evaluation work.
    #[must_use]
    pub fn constant_evaluation_work() -> Self {
        Self::builtin("constant_evaluation_work", "work_units")
    }

    /// Diagnostics retained.
    #[must_use]
    pub fn diagnostics() -> Self {
        Self::builtin("diagnostics", "entries")
    }

    /// Generated artifact bytes.
    #[must_use]
    pub fn artifact_bytes() -> Self {
        Self::builtin("artifact_bytes", "bytes")
    }

    /// Compilation wall-clock time.
    ///
    /// The caller supplies elapsed nanoseconds.
    #[must_use]
    pub fn wall_time() -> Self {
        Self::builtin("wall_time", "nanoseconds")
    }

    /// Accounted CPU time.
    ///
    /// The caller supplies elapsed nanoseconds.
    #[must_use]
    pub fn cpu_time() -> Self {
        Self::builtin("cpu_time", "nanoseconds")
    }

    /// Parser work units.
    #[must_use]
    pub fn parser_work() -> Self {
        Self::builtin("parser_work", "work_units")
    }

    /// Optimizer work units.
    #[must_use]
    pub fn optimizer_work() -> Self {
        Self::builtin("optimizer_work", "work_units")
    }

    fn builtin(
        name: &'static str,
        unit: &'static str,
    ) -> Self {
        Self {
            name: name.to_string(),
            unit: unit.to_string(),
        }
    }
}

impl fmt::Display for BudgetDimension {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{} ({})", self.name, self.unit)
    }
}

// ============================================================================
// LIMITS
// ============================================================================

/// Optional upper bound for one operational budget.
///
/// None means that no explicit budget is imposed.
///
/// Some(0) is a valid zero-consumption budget.
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

    /// Creates an explicitly bounded budget.
    #[must_use]
    pub const fn bounded(limit: u128) -> Self {
        Self {
            limit: Some(limit),
        }
    }

    /// Returns the configured limit.
    #[must_use]
    pub const fn value(self) -> Option<u128> {
        self.limit
    }

    /// Returns whether the budget is unlimited.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        self.limit.is_none()
    }

    /// Checks whether an absolute amount is permitted.
    #[must_use]
    pub const fn permits(self, amount: u128) -> bool {
        match self.limit {
            Some(maximum) => amount <= maximum,
            None => true,
        }
    }

    /// Returns the remaining amount after consumption.
    ///
    /// Unlimited budgets return None.
    ///
    /// Bounded budgets return Some(0) when exhausted.
    ///
    /// Saturation here is intentional because remaining capacity
    /// cannot be negative.
    #[must_use]
    pub const fn remaining(
        self,
        consumed: u128,
    ) -> Option<u128> {
        match self.limit {
            Some(maximum) => {
                Some(maximum.saturating_sub(consumed))
            }
            None => None,
        }
    }
}

impl fmt::Display for BudgetLimit {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.limit {
            Some(value) => write!(formatter, "bounded({value})"),
            None => formatter.write_str("unlimited"),
        }
    }
}

// ============================================================================
// COMPILATION BUDGET
// ============================================================================

/// Immutable operational budget configuration.
///
/// Missing dimensions are unlimited.
///
/// This type does not perform accounting.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompilationBudget {
    limits: BTreeMap<BudgetDimension, BudgetLimit>,
}

impl CompilationBudget {
    /// Creates an unlimited budget.
    #[must_use]
    pub fn unlimited() -> Self {
        Self::default()
    }

    /// Creates an empty budget configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a dimension limit.
    #[must_use]
    pub fn with_limit(
        mut self,
        dimension: BudgetDimension,
        limit: BudgetLimit,
    ) -> Self {
        self.limits.insert(dimension, limit);
        self
    }

    /// Sets a limit and returns its previous value.
    pub fn set_limit(
        &mut self,
        dimension: BudgetDimension,
        limit: BudgetLimit,
    ) -> Option<BudgetLimit> {
        self.limits.insert(dimension, limit)
    }

    /// Removes an explicit limit.
    pub fn remove_limit(
        &mut self,
        dimension: &BudgetDimension,
    ) -> Option<BudgetLimit> {
        self.limits.remove(dimension)
    }

    /// Returns a dimension's configured limit.
    ///
    /// Missing dimensions are unlimited.
    #[must_use]
    pub fn limit(
        &self,
        dimension: &BudgetDimension,
    ) -> BudgetLimit {
        self.limits
            .get(dimension)
            .copied()
            .unwrap_or_else(BudgetLimit::unlimited)
    }

    /// Returns all configured limits in deterministic order.
    #[must_use]
    pub fn limits(
        &self,
    ) -> &BTreeMap<BudgetDimension, BudgetLimit> {
        &self.limits
    }

    /// Returns whether no explicit limits exist.
    #[must_use]
    pub fn is_unlimited(&self) -> bool {
        self.limits.is_empty()
    }

    /// Checks proposed consumption without modifying accounting.
    ///
    /// This is a check, not a reservation.
    pub fn check(
        &self,
        dimension: &BudgetDimension,
        consumed: u128,
        additional: u128,
    ) -> BudgetResult<()> {
        let limit = self.limit(dimension);

        // No configured upper bound means no budget exhaustion
        // decision is necessary here.
        //
        // Representability of cumulative accounting is checked by
        // BudgetTracker when consumption is actually recorded.
        let Some(maximum) = limit.value() else {
            return Ok(());
        };

        let attempted = consumed
            .checked_add(additional)
            .ok_or_else(|| BudgetError::AccountingOverflow {
                dimension: dimension.clone(),
            })?;

        if attempted > maximum {
            return Err(BudgetError::Exceeded {
                dimension: dimension.clone(),
                consumed,
                requested: additional,
                attempted,
                limit: maximum,
            });
        }

        Ok(())
    }

    /// Creates a shared accounting tracker.
    #[must_use]
    pub fn tracker(&self) -> BudgetTracker {
        BudgetTracker::new(self.clone())
    }

    /// Returns the number of configured dimensions.
    #[must_use]
    pub fn dimension_count(&self) -> usize {
        self.limits.len()
    }
}

// ============================================================================
// USAGE
// ============================================================================

/// Usage information for one budget dimension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetUsage {
    /// Dimension identity.
    pub dimension: BudgetDimension,

    /// Recorded cumulative consumption.
    pub consumed: u128,

    /// Explicit maximum, if configured.
    pub limit: Option<u128>,

    /// Remaining capacity, if bounded.
    pub remaining: Option<u128>,
}

impl BudgetUsage {
    /// Returns whether the configured budget is exhausted.
    ///
    /// A zero limit is exhausted even before consumption.
    #[must_use]
    pub const fn is_exhausted(&self) -> bool {
        match self.limit {
            Some(maximum) => self.consumed >= maximum,
            None => false,
        }
    }
}

// ============================================================================
// SNAPSHOTS
// ============================================================================

/// Deterministic snapshot of budget usage.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BudgetSnapshot {
    usage: BTreeMap<BudgetDimension, BudgetUsage>,
}

impl BudgetSnapshot {
    /// Returns usage for a dimension.
    #[must_use]
    pub fn usage(
        &self,
        dimension: &BudgetDimension,
    ) -> Option<&BudgetUsage> {
        self.usage.get(dimension)
    }

    /// Returns all usage entries in deterministic order.
    #[must_use]
    pub fn dimensions(
        &self,
    ) -> &BTreeMap<BudgetDimension, BudgetUsage> {
        &self.usage
    }

    /// Returns the number of dimensions in the snapshot.
    #[must_use]
    pub fn dimension_count(&self) -> usize {
        self.usage.len()
    }

    /// Returns whether the snapshot is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.usage.is_empty()
    }

    /// Returns whether any configured budget is exhausted.
    #[must_use]
    pub fn has_exhausted_budget(&self) -> bool {
        self.usage.values().any(BudgetUsage::is_exhausted)
    }

    /// Returns the dimensions whose budgets are exhausted.
    pub fn exhausted_dimensions(
        &self,
    ) -> impl Iterator<Item = &BudgetUsage> {
        self.usage
            .values()
            .filter(|usage| usage.is_exhausted())
    }
}

// ============================================================================
// REQUESTS
// ============================================================================

/// One requested accounting operation.
///
/// Requests are processed atomically by `consume_batch`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetRequest {
    /// Dimension to consume.
    pub dimension: BudgetDimension,

    /// Amount requested.
    pub amount: u128,
}

impl BudgetRequest {
    /// Creates a budget request.
    #[must_use]
    pub fn new(
        dimension: BudgetDimension,
        amount: u128,
    ) -> Self {
        Self {
            dimension,
            amount,
        }
    }
}

// ============================================================================
// INTERNAL LEDGER
// ============================================================================

#[derive(Debug, Default)]
struct BudgetLedger {
    consumed: BTreeMap<BudgetDimension, u128>,
}

// ============================================================================
// TRACKER
// ============================================================================

/// Shared, thread-safe accounting for one compilation.
///
/// Cloning the tracker shares the same accounting ledger.
///
/// This prevents independent compilation workers from each receiving
/// a fresh copy of the same budget.
#[derive(Debug, Clone)]
pub struct BudgetTracker {
    budget: Arc<CompilationBudget>,
    ledger: Arc<Mutex<BudgetLedger>>,
}

impl BudgetTracker {
    /// Creates a tracker from an immutable configuration.
    #[must_use]
    pub fn new(budget: CompilationBudget) -> Self {
        Self {
            budget: Arc::new(budget),
            ledger: Arc::new(Mutex::new(BudgetLedger::default())),
        }
    }

    /// Acquires the ledger lock.
    ///
    /// Poisoned accounting state fails closed.
    fn lock(&self) -> BudgetResult<MutexGuard<'_, BudgetLedger>> {
        self.ledger
            .lock()
            .map_err(|_| BudgetError::LedgerPoisoned)
    }

    /// Returns the immutable configuration.
    #[must_use]
    pub fn budget(&self) -> &CompilationBudget {
        &self.budget
    }

    /// Returns a shared tracker clone.
    #[must_use]
    pub fn shared(&self) -> Self {
        self.clone()
    }

    /// Atomically consumes an amount from one dimension.
    ///
    /// On failure, the ledger remains unchanged.
    pub fn consume(
        &self,
        dimension: BudgetDimension,
        amount: u128,
    ) -> BudgetResult<u128> {
        let mut ledger = self.lock()?;

        let consumed = ledger
            .consumed
            .get(&dimension)
            .copied()
            .unwrap_or(0);

        let updated = consumed
            .checked_add(amount)
            .ok_or_else(|| BudgetError::AccountingOverflow {
                dimension: dimension.clone(),
            })?;

        self.budget.check(
            &dimension,
            consumed,
            amount,
        )?;

        ledger.consumed.insert(dimension, updated);

        Ok(updated)
    }

    /// Atomically consumes several budget dimensions.
    ///
    /// All requests succeed together or none are committed.
    ///
    /// Duplicate dimensions are aggregated before checking.
    ///
    /// An empty batch succeeds without modifying the ledger.
    pub fn consume_batch(
        &self,
        requests: &[BudgetRequest],
    ) -> BudgetResult<BudgetSnapshot> {
        let mut ledger = self.lock()?;

        let mut aggregated =
            BTreeMap::<BudgetDimension, u128>::new();

        for request in requests {
            let entry = aggregated
                .entry(request.dimension.clone())
                .or_insert(0);

            *entry = entry
                .checked_add(request.amount)
                .ok_or_else(|| BudgetError::AccountingOverflow {
                    dimension: request.dimension.clone(),
                })?;
        }

        // Validate every dimension before committing anything.
        let mut updates =
            BTreeMap::<BudgetDimension, u128>::new();

        for (dimension, additional) in aggregated {
            let consumed = ledger
                .consumed
                .get(&dimension)
                .copied()
                .unwrap_or(0);

            let updated = consumed
                .checked_add(additional)
                .ok_or_else(|| BudgetError::AccountingOverflow {
                    dimension: dimension.clone(),
                })?;

            self.budget.check(
                &dimension,
                consumed,
                additional,
            )?;

            updates.insert(dimension, updated);
        }

        // Commit only after all validation succeeds.
        ledger.consumed.extend(updates);

        self.snapshot_locked(&ledger)
    }

    /// Checks whether additional consumption is permitted.
    ///
    /// This does not reserve capacity.
    pub fn check(
        &self,
        dimension: &BudgetDimension,
        additional: u128,
    ) -> BudgetResult<()> {
        let ledger = self.lock()?;

        let consumed = ledger
            .consumed
            .get(dimension)
            .copied()
            .unwrap_or(0);

        let _attempted = consumed
            .checked_add(additional)
            .ok_or_else(|| BudgetError::AccountingOverflow {
                dimension: dimension.clone(),
            })?;

        self.budget.check(
            dimension,
            consumed,
            additional,
        )
    }

    /// Returns cumulative consumption.
    pub fn consumed(
        &self,
        dimension: &BudgetDimension,
    ) -> BudgetResult<u128> {
        let ledger = self.lock()?;

        Ok(ledger
            .consumed
            .get(dimension)
            .copied()
            .unwrap_or(0))
    }

    /// Returns remaining configured capacity.
    ///
    /// None means unlimited.
    pub fn remaining(
        &self,
        dimension: &BudgetDimension,
    ) -> BudgetResult<Option<u128>> {
        let consumed = self.consumed(dimension)?;

        Ok(self.budget.limit(dimension).remaining(consumed))
    }

    /// Returns whether a dimension has reached its limit.
    pub fn is_exhausted(
        &self,
        dimension: &BudgetDimension,
    ) -> BudgetResult<bool> {
        let consumed = self.consumed(dimension)?;

        Ok(match self.budget.limit(dimension).value() {
            Some(maximum) => consumed >= maximum,
            None => false,
        })
    }

    /// Creates a snapshot while the ledger is locked.
    fn snapshot_locked(
        &self,
        ledger: &BudgetLedger,
    ) -> BudgetResult<BudgetSnapshot> {
        let mut usage = BTreeMap::new();

        // Include all configured dimensions, even if consumption is zero.
        for (dimension, limit) in self.budget.limits() {
            let consumed = ledger
                .consumed
                .get(dimension)
                .copied()
                .unwrap_or(0);

            usage.insert(
                dimension.clone(),
                BudgetUsage {
                    dimension: dimension.clone(),
                    consumed,
                    limit: limit.value(),
                    remaining: limit.remaining(consumed),
                },
            );
        }

        // Include custom dimensions that were consumed without
        // an explicit configured limit.
        for (dimension, consumed) in &ledger.consumed {
            usage.entry(dimension.clone()).or_insert_with(|| {
                BudgetUsage {
                    dimension: dimension.clone(),
                    consumed: *consumed,
                    limit: None,
                    remaining: None,
                }
            });
        }

        Ok(BudgetSnapshot { usage })
    }

    /// Returns a deterministic snapshot.
    pub fn snapshot(&self) -> BudgetResult<BudgetSnapshot> {
        let ledger = self.lock()?;

        self.snapshot_locked(&ledger)
    }

    /// Returns the number of dimensions with recorded consumption.
    pub fn tracked_dimension_count(&self) -> BudgetResult<usize> {
        let ledger = self.lock()?;

        Ok(ledger.consumed.len())
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors from budget configuration and accounting.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BudgetError {
    /// Invalid budget dimension.
    InvalidDimension {
        /// Invalid field.
        field: &'static str,

        /// Explanation.
        reason: String,
    },

    /// Explicit budget exceeded.
    Exceeded {
        /// Dimension.
        dimension: BudgetDimension,

        /// Existing consumption.
        consumed: u128,

        /// Additional requested amount.
        requested: u128,

        /// Total attempted consumption.
        attempted: u128,

        /// Configured maximum.
        limit: u128,
    },

    /// Accounting exceeded the u128 representation.
    AccountingOverflow {
        /// Affected dimension.
        dimension: BudgetDimension,
    },

    /// Accounting mutex was poisoned.
    LedgerPoisoned,
}

impl fmt::Display for BudgetError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
                    "compiler budget accounting representation \
                     overflow for {dimension}"
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

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn default_budget_is_unlimited() {
        let budget = CompilationBudget::default();
        let dimension = BudgetDimension::memory_bytes();

        assert!(budget.is_unlimited());
        assert!(budget.limit(&dimension).is_unlimited());
        assert!(budget.check(&dimension, 0, u128::MAX).is_ok());
    }

    #[test]
    fn zero_limit_is_valid() {
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
    fn custom_dimensions_are_supported() {
        let dimension = BudgetDimension::new(
            "future_domain.custom_work",
            "work_units",
        ).unwrap();

        assert_eq!(dimension.name(), "future_domain.custom_work");
        assert_eq!(dimension.unit(), "work_units");
    }

    #[test]
    fn units_are_part_of_dimension_identity() {
        let bytes = BudgetDimension::new("memory", "bytes").unwrap();
        let kibibytes =
            BudgetDimension::new("memory", "kibibytes").unwrap();

        assert_ne!(bytes, kibibytes);
    }

    #[test]
    fn empty_dimension_name_is_rejected() {
        assert!(BudgetDimension::new("", "bytes").is_err());
    }

    #[test]
    fn empty_dimension_unit_is_rejected() {
        assert!(BudgetDimension::new("memory", "").is_err());
    }

    #[test]
    fn control_characters_are_rejected() {
        assert!(
            BudgetDimension::new("memory\ninvalid", "bytes").is_err()
        );
    }

    #[test]
    fn long_dimension_names_are_not_arbitrarily_rejected() {
        let name = format!("domain.{}", "x".repeat(16_384));

        assert!(
            BudgetDimension::new(name, "work_units").is_ok()
        );
    }

    #[test]
    fn configured_budget_is_enforced() {
        let dimension = BudgetDimension::memory_bytes();

        let budget = CompilationBudget::new().with_limit(
            dimension.clone(),
            BudgetLimit::bounded(100),
        );

        assert!(budget.check(&dimension, 20, 30).is_ok());

        assert!(matches!(
            budget.check(&dimension, 80, 21),
            Err(BudgetError::Exceeded {
                attempted: 101,
                limit: 100,
                ..
            })
        ));
    }

    #[test]
    fn tracker_accounts_consumption() {
        let dimension = BudgetDimension::pass_applications();

        let tracker = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(10),
            )
            .tracker();

        assert_eq!(tracker.consume(dimension.clone(), 4).unwrap(), 4);
        assert_eq!(tracker.consume(dimension.clone(), 3).unwrap(), 7);

        assert_eq!(tracker.consumed(&dimension).unwrap(), 7);
        assert_eq!(tracker.remaining(&dimension).unwrap(), Some(3));
    }

    #[test]
    fn failed_consumption_does_not_modify_ledger() {
        let dimension = BudgetDimension::diagnostics();

        let tracker = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(5),
            )
            .tracker();

        tracker.consume(dimension.clone(), 4).unwrap();

        assert!(tracker.consume(dimension.clone(), 2).is_err());

        assert_eq!(tracker.consumed(&dimension).unwrap(), 4);
    }

    #[test]
    fn clones_share_accounting() {
        let dimension = BudgetDimension::worker_slots();

        let tracker = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(8),
            )
            .tracker();

        let second = tracker.shared();

        tracker.consume(dimension.clone(), 5).unwrap();

        assert_eq!(second.consumed(&dimension).unwrap(), 5);

        assert!(second.consume(dimension.clone(), 4).is_err());
    }

    #[test]
    fn concurrent_consumption_respects_shared_limit() {
        let dimension = BudgetDimension::ir_entities();

        let tracker = Arc::new(
            CompilationBudget::new()
                .with_limit(
                    dimension.clone(),
                    BudgetLimit::bounded(100),
                )
                .tracker(),
        );

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
            if worker.join().unwrap().is_ok() {
                successful += 15;
            }
        }

        assert_eq!(successful, 90);
        assert_eq!(tracker.consumed(&dimension).unwrap(), 90);
    }

    #[test]
    fn unlimited_budget_still_detects_accounting_overflow() {
        let dimension = BudgetDimension::optimizer_work();

        let tracker = CompilationBudget::unlimited().tracker();

        tracker.consume(dimension.clone(), u128::MAX).unwrap();

        assert!(matches!(
            tracker.consume(dimension, 1),
            Err(BudgetError::AccountingOverflow { .. })
        ));
    }

    #[test]
    fn snapshots_include_configured_unused_dimensions() {
        let dimension = BudgetDimension::memory_bytes();

        let tracker = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(100),
            )
            .tracker();

        let snapshot = tracker.snapshot().unwrap();

        let usage = snapshot.usage(&dimension).unwrap();

        assert_eq!(usage.consumed, 0);
        assert_eq!(usage.remaining, Some(100));
    }

    #[test]
    fn snapshots_are_deterministic() {
        let first = BudgetDimension::input_bytes();
        let second = BudgetDimension::artifact_bytes();

        let tracker = CompilationBudget::new()
            .with_limit(
                first.clone(),
                BudgetLimit::bounded(100),
            )
            .with_limit(
                second.clone(),
                BudgetLimit::bounded(200),
            )
            .tracker();

        tracker.consume(second, 50).unwrap();
        tracker.consume(first, 20).unwrap();

        let snapshot = tracker.snapshot().unwrap();

        assert_eq!(snapshot.dimension_count(), 2);
    }

    #[test]
    fn unlimited_dimensions_have_no_remaining_capacity_value() {
        let dimension = BudgetDimension::cpu_time();

        let tracker = CompilationBudget::unlimited().tracker();

        tracker.consume(dimension.clone(), 1_000).unwrap();

        assert_eq!(tracker.remaining(&dimension).unwrap(), None);
        assert!(!tracker.is_exhausted(&dimension).unwrap());
    }

    #[test]
    fn zero_budget_is_exhausted_initially() {
        let dimension = BudgetDimension::input_bytes();

        let tracker = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(0),
            )
            .tracker();

        assert!(tracker.is_exhausted(&dimension).unwrap());
    }

    #[test]
    fn bounded_budget_reports_exhaustion() {
        let dimension = BudgetDimension::wall_time();

        let tracker = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(10),
            )
            .tracker();

        tracker.consume(dimension.clone(), 10).unwrap();

        assert!(tracker.is_exhausted(&dimension).unwrap());
    }

    #[test]
    fn batch_consumption_is_atomic() {
        let memory = BudgetDimension::memory_bytes();
        let workers = BudgetDimension::worker_slots();

        let tracker = CompilationBudget::new()
            .with_limit(
                memory.clone(),
                BudgetLimit::bounded(100),
            )
            .with_limit(
                workers.clone(),
                BudgetLimit::bounded(2),
            )
            .tracker();

        let requests = [
            BudgetRequest::new(memory.clone(), 50),
            BudgetRequest::new(workers.clone(), 3),
        ];

        assert!(tracker.consume_batch(&requests).is_err());

        assert_eq!(tracker.consumed(&memory).unwrap(), 0);
        assert_eq!(tracker.consumed(&workers).unwrap(), 0);
    }

    #[test]
    fn batch_consumption_commits_all_dimensions() {
        let memory = BudgetDimension::memory_bytes();
        let workers = BudgetDimension::worker_slots();

        let tracker = CompilationBudget::new()
            .with_limit(
                memory.clone(),
                BudgetLimit::bounded(100),
            )
            .with_limit(
                workers.clone(),
                BudgetLimit::bounded(4),
            )
            .tracker();

        let requests = [
            BudgetRequest::new(memory.clone(), 50),
            BudgetRequest::new(workers.clone(), 2),
        ];

        tracker.consume_batch(&requests).unwrap();

        assert_eq!(tracker.consumed(&memory).unwrap(), 50);
        assert_eq!(tracker.consumed(&workers).unwrap(), 2);
    }

    #[test]
    fn duplicate_batch_dimensions_are_aggregated() {
        let dimension = BudgetDimension::artifact_bytes();

        let tracker = CompilationBudget::new()
            .with_limit(
                dimension.clone(),
                BudgetLimit::bounded(100),
            )
            .tracker();

        let requests = [
            BudgetRequest::new(dimension.clone(), 20),
            BudgetRequest::new(dimension.clone(), 30),
        ];

        tracker.consume_batch(&requests).unwrap();

        assert_eq!(tracker.consumed(&dimension).unwrap(), 50);
    }

    #[test]
    fn batch_overflow_does_not_commit() {
        let dimension = BudgetDimension::optimizer_work();

        let tracker = CompilationBudget::unlimited().tracker();

        let requests = [
            BudgetRequest::new(dimension.clone(), u128::MAX),
            BudgetRequest::new(dimension.clone(), 1),
        ];

        assert!(matches!(
            tracker.consume_batch(&requests),
            Err(BudgetError::AccountingOverflow { .. })
        ));

        assert_eq!(tracker.consumed(&dimension).unwrap(), 0);
    }

    #[test]
    fn replacing_and_removing_limits_works() {
        let dimension = BudgetDimension::artifact_bytes();

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
            budget.remove_limit(&dimension),
            Some(BudgetLimit::bounded(200))
        );

        assert!(budget.limit(&dimension).is_unlimited());
    }

    #[test]
    fn time_dimensions_have_explicit_units() {
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
    fn error_display_is_informative() {
        let error = BudgetError::Exceeded {
            dimension: BudgetDimension::memory_bytes(),
            consumed: 80,
            requested: 30,
            attempted: 110,
            limit: 100,
        };

        let message = error.to_string();

        assert!(message.contains("memory"));
        assert!(message.contains("110"));
        assert!(message.contains("100"));
    }
}
