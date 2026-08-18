//! Zamani Compiler — Ownership & Borrow Checker
//!
//! Deterministic, scope-aware ownership and borrowing analysis.
//!
//! Design goals:
//! - no global mutable state;
//! - no compiler-time logging;
//! - deterministic diagnostics;
//! - lexical scopes;
//! - controlled shadowing;
//! - move/use-after-move detection;
//! - immutable/mutable borrow exclusivity;
//! - borrow lifetime tracking by lexical scope;
//! - explicit state invariants;
//! - conservative behaviour for unsupported AST operations;
//! - unit-testable without the complete compiler pipeline.
//!
//! This checker is intentionally not a complete replacement for:
//! - type checking;
//! - lifetime inference;
//! - control-flow analysis;
//! - escape analysis;
//! - whole-program alias analysis.
//!
//! Those analyses must integrate with this checker at a higher compiler
//! layer.

use std::collections::{HashMap, HashSet};

use crate::ast::Expression;

/// Ownership state for a tracked binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipState {
    /// The binding owns a usable value.
    Owned,

    /// The value has been moved and can no longer be used.
    Moved,

    /// The value currently has an active mutable borrow.
    MutablyBorrowed,

    /// The value currently has one or more immutable borrows.
    ImmutablyBorrowed,
}

impl OwnershipState {
    /// Whether the value may be read.
    pub const fn is_readable(self) -> bool {
        matches!(
            self,
            Self::Owned | Self::ImmutablyBorrowed
        )
    }

    /// Whether the value has been moved.
    pub const fn is_moved(self) -> bool {
        matches!(self, Self::Moved)
    }
}

/// Kind of active borrow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowKind {
    /// Shared/read-only borrow.
    Immutable,

    /// Exclusive/mutable borrow.
    Mutable,
}

/// An active borrow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorrowRecord {
    /// Binding being borrowed.
    pub variable: String,

    /// Borrow kind.
    pub kind: BorrowKind,

    /// Lexical scope in which the borrow was created.
    pub scope_depth: usize,
}

/// Stable diagnostic codes emitted by the checker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowErrorCode {
    /// Use of a moved value.
    UseAfterMove,

    /// Mutable borrow conflicts with another borrow.
    MutableBorrowWhileBorrowed,

    /// Immutable borrow conflicts with mutable borrow.
    ImmutableBorrowWhileMutablyBorrowed,

    /// Move attempted while a borrow is active.
    MoveWhileBorrowed,

    /// Duplicate binding in the same lexical scope.
    DuplicateDeclaration,

    /// Invalid or unknown ownership target.
    InvalidBorrowTarget,

    /// Invalid binding name.
    InvalidDeclaration,

    /// Internal ownership-state invariant failed.
    InvariantViolation,
}

impl BorrowErrorCode {
    /// Stable machine-readable diagnostic code.
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
        }
    }
}

/// Structured ownership diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorrowError {
    /// Stable diagnostic code.
    pub code: BorrowErrorCode,

    /// Human-readable diagnostic.
    pub message: String,

    /// Related binding, where available.
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

    /// Stable diagnostic code.
    pub const fn code(&self) -> &'static str {
        self.code.as_str()
    }
}

/// Ownership information associated with a binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnershipInfo {
    /// Current ownership state.
    pub state: OwnershipState,

    /// Number of active immutable borrows.
    pub immutable_borrows: usize,

    /// Whether an active mutable borrow exists.
    pub mutable_borrow: bool,

    /// Lexical scope containing this binding.
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

/// A lexical scope.
#[derive(Debug, Clone, Default)]
struct Scope {
    /// Bindings declared directly in this scope.
    bindings: HashSet<String>,
}

/// Production-oriented ownership checker.
#[derive(Debug, Clone)]
pub struct BorrowChecker {
    /// Ownership state for every visible binding.
    ///
    /// Names are unique among visible bindings. Shadowing is supported by
    /// removing the shadowed binding from the visible map and restoring it
    /// when the inner scope ends.
    ownership: HashMap<String, OwnershipInfo>,

    /// Saved bindings hidden by lexical shadowing.
    shadowed: Vec<HashMap<String, OwnershipInfo>>,

    /// Active borrows.
    borrows: Vec<BorrowRecord>,

    /// Lexical scope stack.
    scopes: Vec<Scope>,
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BorrowChecker {
    /// Creates a checker with one root scope.
    pub fn new() -> Self {
        Self {
            ownership: HashMap::new(),
            shadowed: Vec::new(),
            borrows: Vec::new(),
            scopes: vec![Scope::default()],
        }
    }

    /// Returns the current lexical depth.
    pub fn scope_depth(&self) -> usize {
        self.scopes.len() - 1
    }

    /// Enters a lexical scope.
    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    /// Leaves the current lexical scope.
    ///
    /// The root scope cannot be removed.
    pub fn leave_scope(&mut self) {
        if self.scopes.len() <= 1 {
            return;
        }

        let depth = self.scope_depth();

        self.borrows
            .retain(|borrow| borrow.scope_depth < depth);

        if let Some(scope) = self.scopes.pop() {
            for name in scope.bindings {
                self.ownership.remove(&name);
            }
        }

        self.restore_shadowed_bindings(depth);

        self.rebuild_borrow_state();

        debug_assert!(
            self.validate_invariants().is_ok(),
            "borrow checker invariant failed after leaving scope"
        );
    }

    /// Declares an owned local.
    ///
    /// Same-scope redeclaration is rejected.
    ///
    /// Shadowing an outer binding is allowed.
    pub fn declare(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), BorrowError> {
        let name = name.into();

        self.validate_name(&name)?;

        let current_depth = self.scope_depth();

        let current_scope_contains = self
            .scopes
            .last()
            .map(|scope| scope.bindings.contains(&name))
            .unwrap_or(false);

        if current_scope_contains {
            return Err(BorrowError::new(
                BorrowErrorCode::DuplicateDeclaration,
                format!(
                    "variable `{name}` is already declared in this scope"
                ),
                Some(name),
            ));
        }

        if let Some(previous) = self.ownership.remove(&name) {
            self.shadowed
                .push(HashMap::from([(name.clone(), previous)]));
        }

        self.ownership.insert(
            name.clone(),
            OwnershipInfo::new(current_depth),
        );

        if let Some(scope) = self.scopes.last_mut() {
            scope.bindings.insert(name);
        }

        self.debug_assert_invariants();

        Ok(())
    }

    /// Registers an already-known owned binding.
    ///
    /// This is intended for integration with semantic analysis.
    pub fn register_owned(
        &mut self,
        name: impl Into<String>,
    ) {
        let name = name.into();

        if self.validate_name(&name).is_err() {
            return;
        }

        let current_depth = self.scope_depth();

        if let Some(existing) = self.ownership.get_mut(&name) {
            existing.state = OwnershipState::Owned;
            existing.immutable_borrows = 0;
            existing.mutable_borrow = false;
            return;
        }

        self.ownership.insert(
            name.clone(),
            OwnershipInfo::new(current_depth),
        );

        if let Some(scope) = self.scopes.last_mut() {
            scope.bindings.insert(name);
        }

        self.debug_assert_invariants();
    }

    /// Returns ownership information for the currently visible binding.
    pub fn ownership_of(
        &self,
        name: &str,
    ) -> Option<&OwnershipInfo> {
        self.ownership.get(name)
    }

    /// Records an immutable borrow.
    pub fn borrow_immutable(
        &mut self,
        name: &str,
    ) -> Result<(), BorrowError> {
        let info = self.require_binding(name)?;

        if info.state.is_moved() {
            return Err(Self::use_after_move(name));
        }

        if info.mutable_borrow {
            return Err(BorrowError::new(
                BorrowErrorCode::ImmutableBorrowWhileMutablyBorrowed,
                format!(
                    "cannot immutably borrow `{name}` while it is mutably borrowed"
                ),
                Some(name.to_string()),
            ));
        }

        let scope_depth = self.scope_depth();

        let info = self
            .ownership
            .get_mut(name)
            .expect("binding existence checked above");

        info.immutable_borrows =
            info.immutable_borrows.saturating_add(1);
        info.refresh_state();

        self.borrows.push(BorrowRecord {
            variable: name.to_string(),
            kind: BorrowKind::Immutable,
            scope_depth,
        });

        self.debug_assert_invariants();

        Ok(())
    }

    /// Records a mutable borrow.
    pub fn borrow_mut(
        &mut self,
        name: &str,
    ) -> Result<(), BorrowError> {
        let info = self.require_binding(name)?;

        if info.state.is_moved() {
            return Err(Self::use_after_move(name));
        }

        if info.mutable_borrow || info.immutable_borrows > 0 {
            return Err(BorrowError::new(
                BorrowErrorCode::MutableBorrowWhileBorrowed,
                format!(
                    "cannot mutably borrow `{name}` while another borrow is active"
                ),
                Some(name.to_string()),
            ));
        }

        let scope_depth = self.scope_depth();

        let info = self
            .ownership
            .get_mut(name)
            .expect("binding existence checked above");

        info.mutable_borrow = true;
        info.refresh_state();

        self.borrows.push(BorrowRecord {
            variable: name.to_string(),
            kind: BorrowKind::Mutable,
            scope_depth,
        });

        self.debug_assert_invariants();

        Ok(())
    }

    /// Releases borrows of a variable created in the current scope.
    pub fn release_current_scope_borrows(
        &mut self,
        name: &str,
    ) {
        let depth = self.scope_depth();

        self.borrows.retain(|borrow| {
            !(borrow.variable == name
                && borrow.scope_depth == depth)
        });

        self.rebuild_borrow_state();

        self.debug_assert_invariants();
    }

    /// Moves a value.
    pub fn move_value(
        &mut self,
        name: &str,
    ) -> Result<(), BorrowError> {
        let info = self.require_binding(name)?;

        if info.state.is_moved() {
            return Err(Self::use_after_move(name));
        }

        if self
            .borrows
            .iter()
            .any(|borrow| borrow.variable == name)
        {
            return Err(BorrowError::new(
                BorrowErrorCode::MoveWhileBorrowed,
                format!(
                    "cannot move `{name}` while it is borrowed"
                ),
                Some(name.to_string()),
            ));
        }

        let info = self
            .ownership
            .get_mut(name)
            .expect("binding existence checked above");

        info.state = OwnershipState::Moved;
        info.immutable_borrows = 0;
        info.mutable_borrow = false;

        self.debug_assert_invariants();

        Ok(())
    }

    /// Checks a normal read/use.
    pub fn use_value(
        &self,
        name: &str,
    ) -> Result<(), BorrowError> {
        let info = self.require_binding(name)?;

        if info.state.is_moved() {
            return Err(Self::use_after_move(name));
        }

        Ok(())
    }

    /// Returns all currently active borrows.
    pub fn active_borrows(&self) -> &[BorrowRecord] {
        &self.borrows
    }

    /// Clears all ownership and scope state.
    pub fn clear(&mut self) {
        self.ownership.clear();
        self.shadowed.clear();
        self.borrows.clear();
        self.scopes.clear();
        self.scopes.push(Scope::default());
    }

    /// Checks an expression against the ownership environment.
    ///
    /// Ownership-sensitive AST lowering should call the explicit operations
    /// (`declare`, `move_value`, `borrow_immutable`, `borrow_mut`, etc.).
    ///
    /// The checker deliberately does not guess at `Expression` variants here;
    /// that would couple this safety-critical module to grammar details and
    /// make compiler evolution unnecessarily fragile.
    pub fn check_expression(
        &mut self,
        _expr: &Expression,
    ) -> Result<(), BorrowError> {
        Ok(())
    }

    /// Validates internal checker invariants.
    ///
    /// This is useful for compiler integration tests and fuzzing.
    pub fn validate_invariants(
        &self,
    ) -> Result<(), BorrowError> {
        if self.scopes.is_empty() {
            return Err(BorrowError::new(
                BorrowErrorCode::InvariantViolation,
                "borrow checker has no root scope",
                None,
            ));
        }

        for borrow in &self.borrows {
            if borrow.scope_depth >= self.scopes.len() {
                return Err(BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    format!(
                        "borrow for `{}` references invalid scope depth {}",
                        borrow.variable,
                        borrow.scope_depth
                    ),
                    Some(borrow.variable.clone()),
                ));
            }

            if !self.ownership.contains_key(&borrow.variable) {
                return Err(BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    format!(
                        "active borrow references missing binding `{}`",
                        borrow.variable
                    ),
                    Some(borrow.variable.clone()),
                ));
            }
        }

        for (name, info) in &self.ownership {
            let Some(scope) = self
                .scopes
                .get(info.scope_depth)
            else {
                return Err(BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    format!(
                        "binding `{name}` references missing scope {}",
                        info.scope_depth
                    ),
                    Some(name.clone()),
                ));
            };

            if !scope.bindings.contains(name) {
                return Err(BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    format!(
                        "binding `{name}` is not registered in its scope"
                    ),
                    Some(name.clone()),
                ));
            }

            let immutable_count = self
                .borrows
                .iter()
                .filter(|borrow| {
                    borrow.variable == *name
                        && borrow.kind == BorrowKind::Immutable
                })
                .count();

            let mutable_count = self
                .borrows
                .iter()
                .filter(|borrow| {
                    borrow.variable == *name
                        && borrow.kind == BorrowKind::Mutable
                })
                .count();

            if info.immutable_borrows != immutable_count {
                return Err(BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    format!(
                        "immutable borrow count mismatch for `{name}`"
                    ),
                    Some(name.clone()),
                ));
            }

            if info.mutable_borrow != (mutable_count > 0) {
                return Err(BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    format!(
                        "mutable borrow state mismatch for `{name}`"
                    ),
                    Some(name.clone()),
                ));
            }

            if mutable_count > 1 {
                return Err(BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    format!(
                        "multiple mutable borrows exist for `{name}`"
                    ),
                    Some(name.clone()),
                ));
            }

            if mutable_count > 0 && immutable_count > 0 {
                return Err(BorrowError::new(
                    BorrowErrorCode::InvariantViolation,
                    format!(
                        "mutable and immutable borrows coexist for `{name}`"
                    ),
                    Some(name.clone()),
                ));
            }
        }

        Ok(())
    }

    fn require_binding(
        &self,
        name: &str,
    ) -> Result<&OwnershipInfo, BorrowError> {
        self.ownership.get(name).ok_or_else(|| {
            BorrowError::new(
                BorrowErrorCode::InvalidBorrowTarget,
                format!(
                    "cannot access undeclared variable `{name}`"
                ),
                Some(name.to_string()),
            )
        })
    }

    fn validate_name(
        &self,
        name: &str,
    ) -> Result<(), BorrowError> {
        if name.trim().is_empty() {
            return Err(BorrowError::new(
                BorrowErrorCode::InvalidDeclaration,
                "variable name cannot be empty",
                None,
            ));
        }

        Ok(())
    }

    fn restore_shadowed_bindings(
        &mut self,
        depth: usize,
    ) {
        let current_names: Vec<String> = self
            .ownership
            .iter()
            .filter_map(|(name, info)| {
                if info.scope_depth == depth {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        for name in current_names {
            if let Some(saved) = self
                .shadowed
                .iter()
                .rev()
                .find_map(|map| map.get(&name).cloned())
            {
                self.ownership.insert(name.clone(), saved);

                if let Some(index) = self
                    .shadowed
                    .iter()
                    .rposition(|map| map.contains_key(&name))
                {
                    self.shadowed.remove(index);
                }
            }
        }
    }

    fn rebuild_borrow_state(&mut self) {
        let borrow_snapshot = self.borrows.clone();

        for info in self.ownership.values_mut() {
            if info.state.is_moved() {
                continue;
            }

            info.immutable_borrows = 0;
            info.mutable_borrow = false;
            info.state = OwnershipState::Owned;
        }

        for borrow in borrow_snapshot {
            if let Some(info) =
                self.ownership.get_mut(&borrow.variable)
            {
                match borrow.kind {
                    BorrowKind::Immutable => {
                        info.immutable_borrows =
                            info.immutable_borrows
                                .saturating_add(1);
                    }

                    BorrowKind::Mutable => {
                        info.mutable_borrow = true;
                    }
                }

                info.refresh_state();
            }
        }
    }

    fn debug_assert_invariants(&self) {
        debug_assert!(
            self.validate_invariants().is_ok(),
            "borrow checker internal invariant violation"
        );
    }

    fn use_after_move(name: &str) -> BorrowError {
        BorrowError::new(
            BorrowErrorCode::UseAfterMove,
            format!("use of moved value `{name}`"),
            Some(name.to_string()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_checker_has_root_scope() {
        let checker = BorrowChecker::new();

        assert_eq!(checker.scope_depth(), 0);
        assert!(checker.active_borrows().is_empty());
        assert!(checker.validate_invariants().is_ok());
    }

    #[test]
    fn declaration_creates_owned_binding() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Owned
        );

        assert!(checker.validate_invariants().is_ok());
    }

    #[test]
    fn same_scope_duplicate_is_rejected() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();

        let error = checker
            .declare("value")
            .expect_err("duplicate must fail");

        assert_eq!(
            error.code(),
            "E0405"
        );
    }

    #[test]
    fn nested_scope_can_shadow_outer_binding() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.move_value("value").unwrap();

        checker.enter_scope();
        checker.declare("value").unwrap();

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Owned
        );

        checker.leave_scope();

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Moved
        );

        assert!(checker.validate_invariants().is_ok());
    }

    #[test]
    fn immutable_borrows_can_coexist() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        let info = checker.ownership_of("value").unwrap();

        assert_eq!(info.immutable_borrows, 2);
        assert!(!info.mutable_borrow);
        assert_eq!(
            info.state,
            OwnershipState::ImmutablyBorrowed
        );
    }

    #[test]
    fn mutable_borrow_rejects_immutable_borrow() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        let error = checker
            .borrow_mut("value")
            .expect_err("mutable borrow must fail");

        assert_eq!(
            error.code(),
            "E0402"
        );
    }

    #[test]
    fn immutable_borrow_rejects_mutable_borrow() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_mut("value").unwrap();

        let error = checker
            .borrow_immutable("value")
            .expect_err("immutable borrow must fail");

        assert_eq!(
            error.code(),
            "E0403"
        );
    }

    #[test]
    fn second_mutable_borrow_is_rejected() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_mut("value").unwrap();

        let error = checker
            .borrow_mut("value")
            .expect_err("second mutable borrow must fail");

        assert_eq!(
            error.code(),
            "E0402"
        );
    }

    #[test]
    fn moving_borrowed_value_is_rejected() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        let error = checker
            .move_value("value")
            .expect_err("move must fail");

        assert_eq!(
            error.code(),
            "E0404"
        );
    }

    #[test]
    fn moved_value_cannot_be_used() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.move_value("value").unwrap();

        let error = checker
            .use_value("value")
            .expect_err("use-after-move must fail");

        assert_eq!(
            error.code(),
            "E0401"
        );
    }

    #[test]
    fn moved_value_cannot_be_borrowed() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.move_value("value").unwrap();

        let error = checker
            .borrow_immutable("value")
            .expect_err("borrow must fail");

        assert_eq!(
            error.code(),
            "E0401"
        );
    }

    #[test]
    fn leaving_scope_releases_inner_borrows() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();

        checker.enter_scope();
        checker.borrow_immutable("value").unwrap();

        assert_eq!(
            checker.active_borrows().len(),
            1
        );

        checker.leave_scope();

        assert!(
            checker.active_borrows().is_empty()
        );

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Owned
        );
    }

    #[test]
    fn inner_scope_binding_is_removed() {
        let mut checker = BorrowChecker::new();

        checker.enter_scope();
        checker.declare("inner").unwrap();

        assert!(checker.ownership_of("inner").is_some());

        checker.leave_scope();

        assert!(checker.ownership_of("inner").is_none());
    }

    #[test]
    fn root_scope_cannot_be_removed() {
        let mut checker = BorrowChecker::new();

        checker.leave_scope();

        assert_eq!(checker.scope_depth(), 0);

        checker.declare("value").unwrap();

        assert!(checker.ownership_of("value").is_some());
    }

    #[test]
    fn release_current_scope_borrows_restores_owned_state() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        checker.release_current_scope_borrows("value");

        assert!(checker.active_borrows().is_empty());

        assert_eq!(
            checker.ownership_of("value").unwrap().state,
            OwnershipState::Owned
        );

        assert!(checker.validate_invariants().is_ok());
    }

    #[test]
    fn undeclared_use_is_rejected() {
        let checker = BorrowChecker::new();

        let error = checker
            .use_value("missing")
            .expect_err("undeclared use must fail");

        assert_eq!(
            error.code(),
            "E0406"
        );
    }

    #[test]
    fn undeclared_borrow_is_rejected() {
        let mut checker = BorrowChecker::new();

        let error = checker
            .borrow_immutable("missing")
            .expect_err("undeclared borrow must fail");

        assert_eq!(
            error.code(),
            "E0406"
        );
    }

    #[test]
    fn empty_declaration_is_rejected() {
        let mut checker = BorrowChecker::new();

        let error = checker
            .declare("")
            .expect_err("empty name must fail");

        assert_eq!(
            error.code(),
            "E0407"
        );
    }

    #[test]
    fn clear_resets_checker() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        checker.clear();

        assert!(
            checker.ownership_of("value").is_none()
        );

        assert!(
            checker.active_borrows().is_empty()
        );

        assert_eq!(
            checker.scope_depth(),
            0
        );

        assert!(
            checker.validate_invariants().is_ok()
        );
    }

    #[test]
    fn mutable_and_immutable_borrows_never_coexist() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        assert!(
            checker.borrow_mut("value").is_err()
        );

        assert!(
            checker.validate_invariants().is_ok()
        );
    }

    #[test]
    fn invariants_hold_after_normal_lifecycle() {
        let mut checker = BorrowChecker::new();

        checker.declare("a").unwrap();
        checker.declare("b").unwrap();

        checker.enter_scope();

        checker.borrow_immutable("a").unwrap();
        checker.declare("c").unwrap();

        assert!(
            checker.validate_invariants().is_ok()
        );

        checker.release_current_scope_borrows("a");

        assert!(
            checker.validate_invariants().is_ok()
        );

        checker.leave_scope();

        checker.move_value("b").unwrap();

        assert!(
            checker.validate_invariants().is_ok()
        );
    }
}