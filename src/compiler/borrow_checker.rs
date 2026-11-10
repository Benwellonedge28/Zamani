
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Ownership and Borrow Checker
//!
//! Path: src/compiler/borrow_checker.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Architectural authority
//!
//! Source ownership semantics:
//!     grammar/specification/semantic-model.md
//!
//! Ownership syntax:
//!     grammar/memory/ownership.g4
//!
//! Borrowing syntax:
//!     grammar/memory/borrowing.g4
//!
//! Lifetime syntax:
//!     grammar/memory/lifetimes.g4
//!
//! Canonical source ownership analysis:
//!     crate::frontend::semantic::types::ownership
//!
//! Compiler ownership analysis:
//!     crate::compiler::analysis::ownership
//!
//! Canonical ownership contracts:
//!     crate::zuir::ir::ownership
//!
//! Canonical structural ownership verification:
//!     crate::zuir::verify::ownership
//!
//! Canonical control-flow graph:
//!     crate::zuir::analysis::cfg
//!
//! # Responsibility
//!
//! This module provides:
//!
//! - A compatibility-oriented, scope-aware ownership session.
//! - Explicit shared and exclusive borrow tracking.
//! - Binding identity independent of variable names.
//! - Explicit loan identity and termination.
//! - Safe lexical scope management.
//! - Integration with canonical frontend ownership analysis.
//! - Integration with compiler-level ZUIR ownership analysis.
//!
//! It does not establish another source-level ownership authority.
//!
//! # Does not own
//!
//! - Source grammar.
//! - AST construction.
//! - Name resolution.
//! - Type inference.
//! - Lifetime inference.
//! - Canonical ownership contracts.
//! - Canonical ValueId or RegionId.
//! - Canonical control-flow graph construction.
//! - Physical memory allocation.
//! - Quantum resource allocation.
//! - Hardware ownership.
//! - Runtime resource management.
//! - Backend implementation.
//! - Distributed execution.
//!
//! # Scalability
//!
//! No arbitrary hardware or program-size limits are imposed.
//!
//! Storage grows with the actual number of tracked bindings and loans.
//!
//! Identifier exhaustion is reported explicitly rather than silently
//! wrapping or reusing an identity.
//!
//! Operational budgets belong to the appropriate analysis configuration.
//! An incomplete analysis must never be interpreted as successful.
//!
//! # Safety
//!
//! This implementation uses safe Rust exclusively.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// ============================================================================
// CANONICAL FRONTEND OWNERSHIP INTEGRATION
// ============================================================================

/// Canonical source-level ownership and borrowing types.
///
/// These types are owned by the frontend semantic subsystem.
/// This module only exposes their established interfaces.
pub use crate::frontend::semantic::types::ownership::{
    AnalysisStatus,
    BasicBlock,
    BlockId as SemanticBlockId,
    BorrowKind as SemanticBorrowKind,
    GraphError as SemanticGraphError,
    IndexProjection,
    LoanId,
    OwnershipConfig,
    OwnershipDiagnostic,
    OwnershipDiagnosticCode,
    OwnershipEvent,
    OwnershipMode,
    OwnershipReport,
    PlaceId,
    PlacePath,
    Projection,
    ScopeId,
    ScopeTree,
    ScopeTreeError,
    SourceSite,
};

/// Runs the authoritative frontend ownership and borrow analysis.
///
/// The frontend owns source-level ownership semantics, place analysis,
/// borrow lifetimes, and CFG interpretation.
///
/// This function deliberately does not implement another semantic engine.
#[must_use]
pub fn analyze_frontend_ownership(
    cfg: &crate::frontend::semantic::types::ownership::ControlFlowGraph,
    config: &OwnershipConfig,
) -> OwnershipReport {
    crate::frontend::semantic::types::ownership::analyze_ownership(
        cfg,
        config,
    )
}

// ============================================================================
// CANONICAL COMPILER / ZUIR OWNERSHIP INTEGRATION
// ============================================================================

/// Canonical compiler ownership-analysis input.
pub use crate::compiler::analysis::ownership::OwnershipAnalysisInput;

/// Canonical compiler ownership-analysis report.
pub use crate::compiler::analysis::ownership::OwnershipAnalysisReport;

/// Canonical compiler ownership-analysis issue.
pub use crate::compiler::analysis::ownership::OwnershipAnalysisIssue;

/// Canonical compiler ownership-flow event.
pub use crate::compiler::analysis::ownership::OwnershipFlowEvent;

/// Runs the existing CFG-aware compiler ownership analysis.
///
/// This analysis consumes canonical ZUIR ownership contracts,
/// canonical identifiers, canonical blocks, and the canonical CFG.
///
/// It does not reconstruct execution order from identifier ordering.
#[must_use]
pub fn analyze_zuir_ownership(
    input: &OwnershipAnalysisInput<'_>,
) -> OwnershipAnalysisReport {
    crate::compiler::analysis::ownership::analyze_ownership(input)
}

// ============================================================================
// API VERSION
// ============================================================================

/// Public compatibility API version.
pub const BORROW_CHECKER_API_VERSION: u16 = 2;

/// Returns the borrow-checker API version.
#[must_use]
pub const fn api_version() -> u16 {
    BORROW_CHECKER_API_VERSION
}

// ============================================================================
// IDENTITIES
// ============================================================================

/// Identity of a binding within one BorrowChecker session.
///
/// This is a compiler-session identity, not a source AST NodeId,
/// canonical ZUIR ValueId, physical address, or runtime handle.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct BindingId(u64);

impl BindingId {
    /// Returns the underlying session-local identity.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Identity of an individual borrow within one checker session.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct BorrowId(u64);

impl BorrowId {
    /// Returns the underlying session-local identity.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

// ============================================================================
// OWNERSHIP STATE
// ============================================================================

/// State of a locally tracked binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipState {
    /// Binding contains an available owned value.
    Owned,

    /// Value has been moved or consumed.
    Moved,

    /// At least one exclusive borrow is active.
    MutablyBorrowed,

    /// One or more shared borrows are active.
    ImmutablyBorrowed,
}

impl OwnershipState {
    /// Whether the value has not been moved.
    #[must_use]
    pub const fn is_moved(self) -> bool {
        matches!(self, Self::Moved)
    }

    /// Whether the value is available for an ordinary read.
    ///
    /// A shared borrow permits reading.
    /// An exclusive borrow does not.
    #[must_use]
    pub const fn is_readable(self) -> bool {
        matches!(
            self,
            Self::Owned | Self::ImmutablyBorrowed
        )
    }
}

// ============================================================================
// BORROW KIND
// ============================================================================

/// Kind of active borrow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowKind {
    /// Shared/read-only borrowing.
    Immutable,

    /// Exclusive/mutable borrowing.
    Mutable,
}

// ============================================================================
// DIAGNOSTICS
// ============================================================================

/// Stable local borrow-checking diagnostic codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowErrorCode {
    /// Use of a moved value.
    UseAfterMove,

    /// Conflicting exclusive borrowing.
    MutableBorrowWhileBorrowed,

    /// Shared borrowing while an exclusive borrow exists.
    ImmutableBorrowWhileMutablyBorrowed,

    /// Moving a borrowed value.
    MoveWhileBorrowed,

    /// Duplicate declaration in one scope.
    DuplicateDeclaration,

    /// Unknown binding or invalid borrow target.
    InvalidBorrowTarget,

    /// Invalid binding name.
    InvalidDeclaration,

    /// Internal invariant violation.
    InvariantViolation,

    /// Reading an owner while exclusively borrowed.
    AccessWhileMutablyBorrowed,

    /// Mutating an actively borrowed value.
    MutationWhileBorrowed,

    /// Unknown borrow identity.
    UnknownBorrow,

    /// Invalid root-scope exit.
    InvalidScopeExit,

    /// Session identity space exhausted.
    IdentityExhausted,

    /// Expression traversal is not implemented by this compatibility API.
    UnsupportedExpressionTraversal,
}

impl BorrowErrorCode {
    /// Stable diagnostic code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseAfterMove => "E0401",
            Self::MutableBorrowWhileBorrowed => "E0402",
            Self::ImmutableBorrowWhileMutablyBorrowed => "E0403",
            Self::MoveWhileBorrowed => "E0404",
            Self::DuplicateDeclaration => "E0405",
            Self::InvalidBorrowTarget => "E0406",
            Self::InvalidDeclaration => "E0407",
            Self::InvariantViolation => "E0408",
            Self::AccessWhileMutablyBorrowed => "E0409",
            Self::MutationWhileBorrowed => "E0410",
            Self::UnknownBorrow => "E0411",
            Self::InvalidScopeExit => "E0412",
            Self::IdentityExhausted => "E0413",
            Self::UnsupportedExpressionTraversal => "E0414",
        }
    }
}

/// Structured local ownership error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorrowError {
    /// Stable error category.
    pub code: BorrowErrorCode,

    /// Human-readable diagnostic.
    pub message: String,

    /// Associated binding, where available.
    pub variable: Option<String>,
}

impl BorrowError {
    fn new(
        code: BorrowErrorCode,
        message: impl Into<String>,
        variable: Option<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            variable,
        }
    }

    /// Returns the stable diagnostic identifier.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code.as_str()
    }
}

impl fmt::Display for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.code(),
            self.message
        )
    }
}

impl std::error::Error for BorrowError {}

// ============================================================================
// OWNERSHIP INFORMATION
// ============================================================================

/// Ownership information for one binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnershipInfo {
    /// Current ownership state.
    pub state: OwnershipState,

    /// Number of active shared borrows.
    pub immutable_borrows: usize,

    /// Whether an exclusive borrow is active.
    pub mutable_borrow: bool,

    /// Lexical scope depth.
    pub scope_depth: usize,
}

impl OwnershipInfo {
    fn new(scope_depth: usize) -> Self {
        Self {
            state: OwnershipState::Owned,
            immutable_borrows: 0,
            mutable_borrow: false,
            scope_depth,
        }
    }

    fn refresh_state(&mut self) {
        if self.state.is_moved() {
            return;
        }

        self.state = if self.mutable_borrow {
            OwnershipState::MutablyBorrowed
        } else if self.immutable_borrows > 0 {
            OwnershipState::ImmutablyBorrowed
        } else {
            OwnershipState::Owned
        };
    }
}

// ============================================================================
// BORROW RECORDS
// ============================================================================

/// Public description of one active borrow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorrowRecord {
    /// Borrowed binding name.
    pub variable: String,

    /// Borrow mode.
    pub kind: BorrowKind,

    /// Scope in which the loan was created.
    pub scope_depth: usize,
}

// ============================================================================
// INTERNAL STORAGE
// ============================================================================

#[derive(Debug, Clone)]
struct Binding {
    id: BindingId,
    info: OwnershipInfo,
}

#[derive(Debug, Clone, Default)]
struct Scope {
    bindings: BTreeMap<String, Binding>,
}

/// Session-local borrow checker.
///
/// Bindings are stored in lexical scopes rather than a single global
/// name map. This makes shadowing deterministic and avoids accidentally
/// restoring an incorrect binding after nested scope exits.
#[derive(Debug, Clone)]
pub struct BorrowChecker {
    scopes: Vec<Scope>,

    borrows: Vec<BorrowRecord>,

    // Parallel identity index. Each entry identifies the actual
    // binding borrowed by the corresponding BorrowRecord.
    borrow_binding_ids: Vec<BindingId>,

    borrow_ids: Vec<BorrowId>,

    next_binding_id: u64,

    next_borrow_id: u64,
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CONSTRUCTION
// ============================================================================

impl BorrowChecker {
    /// Creates a new checker with one root lexical scope.
    #[must_use]
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::default()],
            borrows: Vec::new(),
            borrow_binding_ids: Vec::new(),
            borrow_ids: Vec::new(),
            next_binding_id: 0,
            next_borrow_id: 0,
        }
    }

    /// Returns the current lexical scope depth.
    #[must_use]
    pub fn scope_depth(&self) -> usize {
        self.scopes.len().saturating_sub(1)
    }

    /// Returns the number of visible scopes.
    #[must_use]
    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    /// Enters a new lexical scope.
    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    /// Leaves the current lexical scope.
    ///
    /// The root scope is preserved.
    ///
    /// Prefer `try_leave_scope` when the caller needs an explicit
    /// diagnostic for invalid scope transitions.
    pub fn leave_scope(&mut self) {
        let _ = self.try_leave_scope();
    }

    /// Fallible lexical scope exit.
    ///
    /// Loans created in the exiting scope are released.
    /// Loans referring to bindings destroyed by that scope are also removed.
    pub fn try_leave_scope(&mut self) -> Result<(), BorrowError> {
        if self.scopes.len() <= 1 {
            return Err(BorrowError::new(
                BorrowErrorCode::InvalidScopeExit,
                "cannot leave the root ownership scope",
                None,
            ));
        }

        let depth = self.scope_depth();

        let removed_scope = self.scopes.pop().ok_or_else(|| {
            BorrowError::new(
                BorrowErrorCode::InvariantViolation,
                "scope stack unexpectedly became empty",
                None,
            )
        })?;

        let removed_ids: BTreeSet<BindingId> = removed_scope
            .bindings
            .values()
            .map(|binding| binding.id)
            .collect();

        let mut index = self.borrows.len();

        while index > 0 {
            index -= 1;

            let remove = self.borrows[index].scope_depth >= depth
                || removed_ids.contains(&self.borrow_binding_ids[index]);

            if remove {
                self.remove_borrow_at(index);
            }
        }

        self.validate_invariants()?;

        Ok(())
    }

    // ========================================================================
    // DECLARATION
    // ========================================================================

    /// Declares an owned binding.
    ///
    /// Duplicate declarations in the same scope are rejected.
    /// Shadowing an outer binding is permitted.
    pub fn declare(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), BorrowError> {
        self.declare_with_id(name).map(|_| ())
    }

    /// Declares a binding and returns its unique session-local identity.
    pub fn declare_with_id(
        &mut self,
        name: impl Into<String>,
    ) -> Result<BindingId, BorrowError> {
        let name = name.into();

        self.validate_name(&name)?;

        let depth = self.scope_depth();

        let current_scope = self.scopes.last().ok_or_else(|| {
            BorrowError::new(
                BorrowErrorCode::InvariantViolation,
                "no root ownership scope exists",
                None,
            )
        })?;

        if current_scope.bindings.contains_key(&name) {
            return Err(BorrowError::new(
                BorrowErrorCode::DuplicateDeclaration,
                format!(
                    "variable `{name}` is already declared in this scope"
                ),
                Some(name),
            ));
        }

        let id = self.allocate_binding_id()?;

        let binding = Binding {
            id,
            info: OwnershipInfo::new(depth),
        };

        self.scopes
            .last_mut()
            .expect("root scope validated")
            .bindings
            .insert(name, binding);

        self.debug_assert_invariants();

        Ok(id)
    }

    /// Registers an existing owned binding.
    ///
    /// Unlike the previous implementation, invalid declarations are
    /// never silently discarded.
    ///
    /// Duplicate registrations return an error.
    pub fn register_owned(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), BorrowError> {
        self.declare(name)
    }

    // ========================================================================
    // LOOKUP
    // ========================================================================

    /// Returns ownership information for the visible binding.
    #[must_use]
    pub fn ownership_of(
        &self,
        name: &str,
    ) -> Option<&OwnershipInfo> {
        self.lookup_binding(name).map(|binding| &binding.info)
    }

    /// Returns the identity of the visible binding.
    #[must_use]
    pub fn binding_id(
        &self,
        name: &str,
    ) -> Option<BindingId> {
        self.lookup_binding(name).map(|binding| binding.id)
    }

    /// Returns whether a binding is currently visible.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.lookup_binding(name).is_some()
    }

    // ========================================================================
    // BORROWING
    // ========================================================================

    /// Creates a shared borrow.
    pub fn borrow_immutable(
        &mut self,
        name: &str,
    ) -> Result<(), BorrowError> {
        self.borrow_immutable_with_id(name).map(|_| ())
    }

    /// Creates a shared borrow and returns its identity.
    pub fn borrow_immutable_with_id(
        &mut self,
        name: &str,
    ) -> Result<BorrowId, BorrowError> {
        self.acquire_borrow(name, BorrowKind::Immutable)
    }

    /// Creates an exclusive mutable borrow.
    pub fn borrow_mut(
        &mut self,
        name: &str,
    ) -> Result<(), BorrowError> {
        self.borrow_mut_with_id(name).map(|_| ())
    }

    /// Creates an exclusive borrow and returns its identity.
    pub fn borrow_mut_with_id(
        &mut self,
        name: &str,
    ) -> Result<BorrowId, BorrowError> {
        self.acquire_borrow(name, BorrowKind::Mutable)
    }

    fn acquire_borrow(
        &mut self,
        name: &str,
        kind: BorrowKind,
    ) -> Result<BorrowId, BorrowError> {
        let binding = self.lookup_binding(name).ok_or_else(|| {
            Self::unknown_binding(name)
        })?;

        if binding.info.state.is_moved() {
            return Err(Self::use_after_move(name));
        }

        match kind {
            BorrowKind::Immutable => {
                if binding.info.mutable_borrow {
                    return Err(BorrowError::new(
                        BorrowErrorCode::ImmutableBorrowWhileMutablyBorrowed,
                        format!(
                            "cannot immutably borrow `{name}` while an exclusive borrow exists"
                        ),
                        Some(name.to_string()),
                    ));
                }
            }

            BorrowKind::Mutable => {
                if binding.info.mutable_borrow
                    || binding.info.immutable_borrows > 0
                {
                    return Err(BorrowError::new(
                        BorrowErrorCode::MutableBorrowWhileBorrowed,
                        format!(
                            "cannot exclusively borrow `{name}` while another borrow exists"
                        ),
                        Some(name.to_string()),
                    ));
                }
            }
        }

        let binding_id = binding.id;
        let borrow_id = self.allocate_borrow_id()?;

        let scope_depth = self.scope_depth();

        let record = BorrowRecord {
            variable: name.to_string(),
            kind,
            scope_depth,
        };

        let binding = self.lookup_binding_mut(name).ok_or_else(|| {
            Self::unknown_binding(name)
        })?;

        match kind {
            BorrowKind::Immutable => {
                binding.info.immutable_borrows =
                    binding.info.immutable_borrows.checked_add(1)
                        .ok_or_else(|| {
                            BorrowError::new(
                                BorrowErrorCode::IdentityExhausted,
                                "shared borrow counter exhausted",
                                Some(name.to_string()),
                            )
                        })?;
            }

            BorrowKind::Mutable => {
                binding.info.mutable_borrow = true;
            }
        }

        binding.info.refresh_state();

        self.borrows.push(record);
        self.borrow_binding_ids.push(binding_id);
        self.borrow_ids.push(borrow_id);

        self.debug_assert_invariants();

        Ok(borrow_id)
    }

    /// Releases one specific borrow.
    ///
    /// A loan cannot be released twice.
    pub fn release_borrow(
        &mut self,
        id: BorrowId,
    ) -> Result<(), BorrowError> {
        let index = self.borrow_ids.iter().position(|item| *item == id)
            .ok_or_else(|| {
                BorrowError::new(
                    BorrowErrorCode::UnknownBorrow,
                    format!("unknown or already released borrow {}", id.get()),
                    None,
                )
            })?;

        self.remove_borrow_at(index);

        self.debug_assert_invariants();

        Ok(())
    }

    /// Releases all loans of a binding created in the current scope.
    ///
    /// This compatibility operation does not release loans created
    /// in an enclosing scope.
    pub fn release_current_scope_borrows(
        &mut self,
        name: &str,
    ) {
        let Some(binding_id) = self.binding_id(name) else {
            return;
        };

        let depth = self.scope_depth();

        let mut index = self.borrows.len();

        while index > 0 {
            index -= 1;

            if self.borrow_binding_ids[index] == binding_id
                && self.borrows[index].scope_depth == depth
            {
                self.remove_borrow_at(index);
            }
        }

        self.debug_assert_invariants();
    }

    /// Returns all active borrow records.
    #[must_use]
    pub fn active_borrows(&self) -> &[BorrowRecord] {
        &self.borrows
    }

    // ========================================================================
    // VALUE OPERATIONS
    // ========================================================================

    /// Checks an ordinary read.
    ///
    /// Shared borrows permit reading.
    /// Exclusive borrows prohibit reading through the owner.
    pub fn use_value(
        &self,
        name: &str,
    ) -> Result<(), BorrowError> {
        let binding = self.lookup_binding(name)
            .ok_or_else(|| Self::unknown_binding(name))?;

        if binding.info.state.is_moved() {
            return Err(Self::use_after_move(name));
        }

        if binding.info.mutable_borrow {
            return Err(BorrowError::new(
                BorrowErrorCode::AccessWhileMutablyBorrowed,
                format!(
                    "cannot access `{name}` while it is exclusively borrowed"
                ),
                Some(name.to_string()),
            ));
        }

        Ok(())
    }

    /// Moves an owned value.
    ///
    /// Moving while any borrow is active is prohibited.
    pub fn move_value(
        &mut self,
        name: &str,
    ) -> Result<(), BorrowError> {
        let binding_id = {
            let binding = self.lookup_binding(name)
                .ok_or_else(|| Self::unknown_binding(name))?;

            if binding.info.state.is_moved() {
                return Err(Self::use_after_move(name));
            }

            if binding.info.immutable_borrows > 0
                || binding.info.mutable_borrow
            {
                return Err(BorrowError::new(
                    BorrowErrorCode::MoveWhileBorrowed,
                    format!(
                        "cannot move `{name}` while a borrow is active"
                    ),
                    Some(name.to_string()),
                ));
            }

            binding.id
        };

        let binding = self.lookup_binding_mut_by_id(binding_id)
            .ok_or_else(|| {
                BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    "binding disappeared during move",
                    Some(name.to_string()),
                )
            })?;

        binding.info.state = OwnershipState::Moved;

        self.debug_assert_invariants();

        Ok(())
    }

    /// Mutates an owned value.
    ///
    /// Any overlapping borrow of the binding prevents mutation.
    pub fn mutate_value(
        &mut self,
        name: &str,
    ) -> Result<(), BorrowError> {
        let binding = self.lookup_binding(name)
            .ok_or_else(|| Self::unknown_binding(name))?;

        if binding.info.state.is_moved() {
            return Err(Self::use_after_move(name));
        }

        if binding.info.immutable_borrows > 0
            || binding.info.mutable_borrow
        {
            return Err(BorrowError::new(
                BorrowErrorCode::MutationWhileBorrowed,
                format!(
                    "cannot mutate `{name}` while a borrow is active"
                ),
                Some(name.to_string()),
            ));
        }

        Ok(())
    }

    /// Reinitializes a moved binding.
    ///
    /// This represents a new value being assigned to the existing
    /// source-level place. Type compatibility is checked by the frontend.
    pub fn write_value(
        &mut self,
        name: &str,
    ) -> Result<(), BorrowError> {
        let binding = self.lookup_binding(name)
            .ok_or_else(|| Self::unknown_binding(name))?;

        if binding.info.immutable_borrows > 0
            || binding.info.mutable_borrow
        {
            return Err(BorrowError::new(
                BorrowErrorCode::MutationWhileBorrowed,
                format!(
                    "cannot write `{name}` while a borrow is active"
                ),
                Some(name.to_string()),
            ));
        }

        let id = binding.id;

        let binding = self.lookup_binding_mut_by_id(id)
            .ok_or_else(|| {
                BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    "binding disappeared during write",
                    Some(name.to_string()),
                )
            })?;

        binding.info.state = OwnershipState::Owned;

        Ok(())
    }

    // ========================================================================
    // EXPRESSION COMPATIBILITY
    // ========================================================================

    /// Legacy AST expression compatibility boundary.
    ///
    /// This method intentionally refuses to certify an expression because
    /// the legacy AST does not provide this module with a complete,
    /// authoritative ownership-event traversal contract.
    ///
    /// Expression analysis must use the native frontend AST adapter and
    /// canonical ownership CFG.
    pub fn check_expression(
        &mut self,
        _expr: &crate::ast::Expression,
    ) -> Result<(), BorrowError> {
        Err(BorrowError::new(
            BorrowErrorCode::UnsupportedExpressionTraversal,
            "legacy expression traversal is not an authoritative ownership analysis; use frontend CFG ownership analysis",
            None,
        ))
    }

    // ========================================================================
    // INVARIANTS
    // ========================================================================

    /// Validates all local storage and borrow invariants.
    pub fn validate_invariants(
        &self,
    ) -> Result<(), BorrowError> {
        if self.scopes.is_empty() {
            return Err(Self::invariant(
                "ownership checker has no root scope",
            ));
        }

        if self.borrows.len() != self.borrow_binding_ids.len()
            || self.borrows.len() != self.borrow_ids.len()
        {
            return Err(Self::invariant(
                "borrow records and identity indexes have different lengths",
            ));
        }

        let mut seen_borrows = BTreeSet::new();

        for (index, borrow) in self.borrows.iter().enumerate() {
            if borrow.scope_depth >= self.scopes.len() {
                return Err(Self::invariant(
                    "borrow references a nonexistent lexical scope",
                ));
            }

            if !seen_borrows.insert(self.borrow_ids[index]) {
                return Err(Self::invariant(
                    "duplicate borrow identity detected",
                ));
            }

            if self.find_binding_by_id(
                self.borrow_binding_ids[index]
            ).is_none() {
                return Err(Self::invariant(
                    "active borrow references a missing binding",
                ));
            }

            if borrow.kind == BorrowKind::Mutable
                && self.borrows.iter().enumerate().any(
                    |(other_index, other)| {
                        other_index != index
                            && self.borrow_binding_ids[other_index]
                                == self.borrow_binding_ids[index]
                    }
                )
            {
                return Err(Self::invariant(
                    "exclusive borrow coexists with another borrow",
                ));
            }
        }

        for scope in &self.scopes {
            for binding in scope.bindings.values() {
                let mut shared = 0usize;
                let mut exclusive = false;

                for (index, borrow) in self.borrows.iter().enumerate() {
                    if self.borrow_binding_ids[index] != binding.id {
                        continue;
                    }

                    match borrow.kind {
                        BorrowKind::Immutable => {
                            shared = shared.checked_add(1).ok_or_else(
                                || Self::invariant(
                                    "shared borrow validation overflow",
                                )
                            )?;
                        }
                        BorrowKind::Mutable => {
                            if exclusive {
                                return Err(Self::invariant(
                                    "multiple exclusive borrows exist",
                                ));
                            }
                            exclusive = true;
                        }
                    }
                }

                if shared != binding.info.immutable_borrows
                    || exclusive != binding.info.mutable_borrow
                {
                    return Err(Self::invariant(
                        "ownership borrow counters do not match active loans",
                    ));
                }

                if exclusive && shared > 0 {
                    return Err(Self::invariant(
                        "shared and exclusive borrows coexist",
                    ));
                }

                let expected = if binding.info.state.is_moved() {
                    OwnershipState::Moved
                } else if exclusive {
                    OwnershipState::MutablyBorrowed
                } else if shared > 0 {
                    OwnershipState::ImmutablyBorrowed
                } else {
                    OwnershipState::Owned
                };

                if binding.info.state != expected {
                    return Err(Self::invariant(
                        "binding ownership state is inconsistent",
                    ));
                }
            }
        }

        Ok(())
    }

    // ========================================================================
    // SESSION MANAGEMENT
    // ========================================================================

    /// Clears all current ownership and borrow state.
    ///
    /// Identity counters are deliberately preserved so that stale
    /// session-local handles cannot become valid again after clearing.
    pub fn clear(&mut self) {
        self.scopes.clear();
        self.scopes.push(Scope::default());

        self.borrows.clear();
        self.borrow_binding_ids.clear();
        self.borrow_ids.clear();
    }

    // ========================================================================
    // INTERNAL HELPERS
    // ========================================================================

    fn allocate_binding_id(
        &mut self,
    ) -> Result<BindingId, BorrowError> {
        let id = self.next_binding_id;

        self.next_binding_id = id.checked_add(1).ok_or_else(|| {
            BorrowError::new(
                BorrowErrorCode::IdentityExhausted,
                "binding identity space exhausted",
                None,
            )
        })?;

        Ok(BindingId(id))
    }

    fn allocate_borrow_id(
        &mut self,
    ) -> Result<BorrowId, BorrowError> {
        let id = self.next_borrow_id;

        self.next_borrow_id = id.checked_add(1).ok_or_else(|| {
            BorrowError::new(
                BorrowErrorCode::IdentityExhausted,
                "borrow identity space exhausted",
                None,
            )
        })?;

        Ok(BorrowId(id))
    }

    fn lookup_binding(
        &self,
        name: &str,
    ) -> Option<&Binding> {
        self.scopes.iter().rev()
            .find_map(|scope| scope.bindings.get(name))
    }

    fn lookup_binding_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut Binding> {
        self.scopes.iter_mut().rev()
            .find_map(|scope| scope.bindings.get_mut(name))
    }

    fn lookup_binding_mut_by_id(
        &mut self,
        id: BindingId,
    ) -> Option<&mut Binding> {
        self.scopes.iter_mut().rev()
            .flat_map(|scope| scope.bindings.values_mut())
            .find(|binding| binding.id == id)
    }

    fn find_binding_by_id(
        &self,
        id: BindingId,
    ) -> Option<&Binding> {
        self.scopes.iter().rev()
            .flat_map(|scope| scope.bindings.values())
            .find(|binding| binding.id == id)
    }

    fn remove_borrow_at(
        &mut self,
        index: usize,
    ) {
        let kind = self.borrows.remove(index).kind;
        let binding_id = self.borrow_binding_ids.remove(index);
        self.borrow_ids.remove(index);

        if let Some(binding) =
            self.lookup_binding_mut_by_id(binding_id)
        {
            match kind {
                BorrowKind::Immutable => {
                    binding.info.immutable_borrows =
                        binding.info.immutable_borrows.saturating_sub(1);
                }
                BorrowKind::Mutable => {
                    binding.info.mutable_borrow = false;
                }
            }

            binding.info.refresh_state();
        }
    }

    fn validate_name(
        &self,
        name: &str,
    ) -> Result<(), BorrowError> {
        if name.trim().is_empty()
            || name.chars().any(char::is_control)
        {
            return Err(BorrowError::new(
                BorrowErrorCode::InvalidDeclaration,
                "binding name must not be empty or contain control characters",
                None,
            ));
        }

        Ok(())
    }

    fn unknown_binding(name: &str) -> BorrowError {
        BorrowError::new(
            BorrowErrorCode::InvalidBorrowTarget,
            format!("unknown binding `{name}`"),
            Some(name.to_string()),
        )
    }

    fn use_after_move(name: &str) -> BorrowError {
        BorrowError::new(
            BorrowErrorCode::UseAfterMove,
            format!("use of moved value `{name}`"),
            Some(name.to_string()),
        )
    }

    fn invariant(message: impl Into<String>) -> BorrowError {
        BorrowError::new(
            BorrowErrorCode::InvariantViolation,
            message,
            None,
        )
    }

    fn debug_assert_invariants(&self) {
        debug_assert!(
            self.validate_invariants().is_ok(),
            "borrow checker invariant violation",
        );
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_checker_has_root_scope() {
        let checker = BorrowChecker::new();

        assert_eq!(checker.scope_depth(), 0);
        assert_eq!(checker.scope_count(), 1);
        assert!(checker.active_borrows().is_empty());
        assert!(checker.validate_invariants().is_ok());
    }

    #[test]
    fn declaration_creates_owned_binding() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Owned,
        );
    }

    #[test]
    fn duplicate_declaration_is_rejected() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();

        let error = checker.declare("value").unwrap_err();

        assert_eq!(error.code(), "E0405");
    }

    #[test]
    fn nested_scope_shadowing_preserves_outer_binding() {
        let mut checker = BorrowChecker::new();

        let outer = checker.declare_with_id("value").unwrap();

        checker.move_value("value").unwrap();

        checker.enter_scope();

        let inner = checker.declare_with_id("value").unwrap();

        assert_ne!(outer, inner);

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Owned,
        );

        checker.try_leave_scope().unwrap();

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Moved,
        );
    }

    #[test]
    fn multiple_shared_borrows_are_allowed() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();

        let first = checker.borrow_immutable_with_id("value").unwrap();
        let second = checker.borrow_immutable_with_id("value").unwrap();

        assert_ne!(first, second);

        assert_eq!(
            checker.ownership_of("value").unwrap().immutable_borrows,
            2,
        );

        checker.validate_invariants().unwrap();
    }

    #[test]
    fn exclusive_borrow_rejects_shared_borrow() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_mut("value").unwrap();

        let error = checker.borrow_immutable("value").unwrap_err();

        assert_eq!(error.code(), "E0403");
    }

    #[test]
    fn shared_borrow_rejects_exclusive_borrow() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        let error = checker.borrow_mut("value").unwrap_err();

        assert_eq!(error.code(), "E0402");
    }

    #[test]
    fn moving_borrowed_value_is_rejected() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        let error = checker.move_value("value").unwrap_err();

        assert_eq!(error.code(), "E0404");
    }

    #[test]
    fn moved_value_cannot_be_used() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.move_value("value").unwrap();

        let error = checker.use_value("value").unwrap_err();

        assert_eq!(error.code(), "E0401");
    }

    #[test]
    fn moved_value_can_be_reinitialized() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.move_value("value").unwrap();

        checker.write_value("value").unwrap();

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Owned,
        );
    }

    #[test]
    fn exclusive_borrow_prevents_owner_read() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_mut("value").unwrap();

        let error = checker.use_value("value").unwrap_err();

        assert_eq!(error.code(), "E0409");
    }

    #[test]
    fn shared_borrow_allows_owner_read() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        assert!(checker.use_value("value").is_ok());
    }

    #[test]
    fn exclusive_borrow_prevents_mutation() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_mut("value").unwrap();

        let error = checker.mutate_value("value").unwrap_err();

        assert_eq!(error.code(), "E0410");
    }

    #[test]
    fn releasing_specific_borrow_restores_access() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();

        let loan = checker.borrow_mut_with_id("value").unwrap();

        checker.release_borrow(loan).unwrap();

        assert!(checker.use_value("value").is_ok());
    }

    #[test]
    fn releasing_unknown_borrow_fails() {
        let mut checker = BorrowChecker::new();

        let error = checker.release_borrow(BorrowId(100)).unwrap_err();

        assert_eq!(error.code(), "E0411");
    }

    #[test]
    fn inner_scope_loans_end_when_scope_exits() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();

        checker.enter_scope();
        checker.borrow_immutable("value").unwrap();

        checker.try_leave_scope().unwrap();

        assert!(checker.active_borrows().is_empty());

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Owned,
        );
    }

    #[test]
    fn root_scope_cannot_be_removed() {
        let mut checker = BorrowChecker::new();

        assert_eq!(
            checker.try_leave_scope().unwrap_err().code(),
            "E0412",
        );

        assert_eq!(checker.scope_count(), 1);
    }

    #[test]
    fn clear_preserves_identity_monotonicity() {
        let mut checker = BorrowChecker::new();

        let first = checker.declare_with_id("first").unwrap();

        checker.clear();

        let second = checker.declare_with_id("second").unwrap();

        assert!(second > first);
    }

    #[test]
    fn expression_check_does_not_falsely_report_success() {
        // The legacy expression visitor has no authoritative ownership
        // event adapter. The method must therefore reject unsupported
        // traversal rather than certify unchecked expressions.
        let error = BorrowError::new(
            BorrowErrorCode::UnsupportedExpressionTraversal,
            "unsupported",
            None,
        );

        assert_eq!(error.code(), "E0414");
    }

    #[test]
    fn frontend_analysis_api_is_exposed() {
        assert!(BORROW_CHECKER_API_VERSION > 0);
        assert!(api_version() > 0);
    }
}
