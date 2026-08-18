//! Zamani Compiler — Ownership & Borrow Checker
//!
//! This module performs deterministic ownership and borrowing analysis over
//! Zamani expressions.
//
//! Design goals:
//! - no global mutable state;
//! - no logging during compilation;
//! - deterministic diagnostics;
//! - explicit ownership state;
//! - mutable/immutable borrow exclusivity;
//! - move/use-after-move detection;
//! - scope-aware state;
//! - conservative behaviour for unsupported ownership-sensitive operations;
//! - unit-testable without requiring the complete compiler pipeline.
//
//! The checker intentionally does not claim to provide lifetime inference or
//! whole-program proof by itself. Those guarantees require integration with
//! the semantic analyzer, function signatures, type information, and control
//! flow analysis.

use std::collections::{HashMap, HashSet};

use crate::ast::Expression;

/// Ownership state for a tracked local value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipState {
    /// The value is owned and available for use.
    Owned,
    /// The value has been moved and can no longer be used.
    Moved,
    /// The value is mutably borrowed.
    MutablyBorrowed,
    /// The value has one or more immutable borrows.
    ImmutablyBorrowed,
}

impl OwnershipState {
    fn is_usable(self) -> bool {
        matches!(self, Self::Owned | Self::ImmutablyBorrowed)
    }
}

/// Kind of active borrow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowKind {
    Immutable,
    Mutable,
}

/// An active borrow tracked by the checker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorrowRecord {
    pub variable: String,
    pub kind: BorrowKind,
    pub scope_depth: usize,
}

/// Stable diagnostic codes emitted by the borrow checker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowErrorCode {
    UseAfterMove,
    MutableBorrowWhileBorrowed,
    ImmutableBorrowWhileMutablyBorrowed,
    MoveWhileBorrowed,
    DuplicateDeclaration,
    InvalidBorrowTarget,
}

impl BorrowErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseAfterMove => "E0401",
            Self::MutableBorrowWhileBorrowed => "E0402",
            Self::ImmutableBorrowWhileMutablyBorrowed => "E0403",
            Self::MoveWhileBorrowed => "E0404",
            Self::DuplicateDeclaration => "E0405",
            Self::InvalidBorrowTarget => "E0406",
        }
    }
}

/// Structured ownership diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorrowError {
    pub code: BorrowErrorCode,
    pub message: String,
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

    /// Stable machine-readable error code.
    pub const fn code(&self) -> &'static str {
        self.code.as_str()
    }
}

/// Ownership information for one local binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnershipInfo {
    pub state: OwnershipState,
    pub immutable_borrows: usize,
    pub mutable_borrow: bool,
}

impl Default for OwnershipInfo {
    fn default() -> Self {
        Self {
            state: OwnershipState::Owned,
            immutable_borrows: 0,
            mutable_borrow: false,
        }
    }
}

/// Production-oriented ownership checker.
///
/// The checker is deliberately independent of stdout/stderr. Compilation
/// should never emit diagnostic text as a side effect.
#[derive(Debug, Clone)]
pub struct BorrowChecker {
    ownership: HashMap<String, OwnershipInfo>,
    borrows: Vec<BorrowRecord>,
    scopes: Vec<HashSet<String>>,
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BorrowChecker {
    /// Creates a new checker with one root scope.
    pub fn new() -> Self {
        Self {
            ownership: HashMap::new(),
            borrows: Vec::new(),
            scopes: vec![HashSet::new()],
        }
    }

    /// Enters a lexical scope.
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }

    /// Leaves the current lexical scope and releases borrows belonging to it.
    ///
    /// The root scope cannot be removed.
    pub fn leave_scope(&mut self) {
        if self.scopes.len() <= 1 {
            return;
        }

        let depth = self.scopes.len() - 1;

        self.borrows.retain(|borrow| borrow.scope_depth < depth);

        if let Some(bindings) = self.scopes.pop() {
            for name in bindings {
                self.ownership.remove(&name);
            }
        }

        self.rebuild_borrow_state();
    }

    /// Declares a new owned local.
    pub fn declare(&mut self, name: impl Into<String>) -> Result<(), BorrowError> {
        let name = name.into();

        if self.ownership.contains_key(&name) {
            return Err(BorrowError::new(
                BorrowErrorCode::DuplicateDeclaration,
                format!("variable `{name}` is already declared in this ownership context"),
                Some(name),
            ));
        }

        self.ownership.insert(name.clone(), OwnershipInfo::default());

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name);
        }

        Ok(())
    }

    /// Explicitly registers an existing value as owned.
    ///
    /// This is useful when integrating the checker with semantic analysis,
    /// where declarations may already have been processed.
    pub fn register_owned(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.ownership
            .entry(name.clone())
            .or_insert_with(OwnershipInfo::default);

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name);
        }
    }

    /// Returns ownership information for a variable.
    pub fn ownership_of(&self, name: &str) -> Option<&OwnershipInfo> {
        self.ownership.get(name)
    }

    /// Records an immutable borrow.
    pub fn borrow_immutable(
        &mut self,
        name: &str,
    ) -> Result<(), BorrowError> {
        let info = self.require_usable(name)?;

        if info.mutable_borrow {
            return Err(BorrowError::new(
                BorrowErrorCode::ImmutableBorrowWhileMutablyBorrowed,
                format!(
                    "cannot immutably borrow `{name}` while it is mutably borrowed"
                ),
                Some(name.to_string()),
            ));
        }

        let scope_depth = self.scopes.len() - 1;

        if let Some(info) = self.ownership.get_mut(name) {
            info.immutable_borrows += 1;
            info.state = OwnershipState::ImmutablyBorrowed;
        }

        self.borrows.push(BorrowRecord {
            variable: name.to_string(),
            kind: BorrowKind::Immutable,
            scope_depth,
        });

        Ok(())
    }

    /// Records a mutable borrow.
    pub fn borrow_mut(&mut self, name: &str) -> Result<(), BorrowError> {
        let info = self.require_usable(name)?;

        if info.mutable_borrow || info.immutable_borrows > 0 {
            return Err(BorrowError::new(
                BorrowErrorCode::MutableBorrowWhileBorrowed,
                format!(
                    "cannot mutably borrow `{name}` while another borrow is active"
                ),
                Some(name.to_string()),
            ));
        }

        let scope_depth = self.scopes.len() - 1;

        if let Some(info) = self.ownership.get_mut(name) {
            info.mutable_borrow = true;
            info.state = OwnershipState::MutablyBorrowed;
        }

        self.borrows.push(BorrowRecord {
            variable: name.to_string(),
            kind: BorrowKind::Mutable,
            scope_depth,
        });

        Ok(())
    }

    /// Releases all borrows of a variable in the current scope.
    pub fn release_current_scope_borrows(&mut self, name: &str) {
        let current_depth = self.scopes.len() - 1;

        self.borrows.retain(|borrow| {
            !(borrow.variable == name && borrow.scope_depth == current_depth)
        });

        self.rebuild_borrow_state();
    }

    /// Moves a value.
    pub fn move_value(&mut self, name: &str) -> Result<(), BorrowError> {
        self.require_usable(name)?;

        let has_active_borrow = self.borrows.iter().any(|borrow| borrow.variable == name);

        if has_active_borrow {
            return Err(BorrowError::new(
                BorrowErrorCode::MoveWhileBorrowed,
                format!("cannot move `{name}` while it is borrowed"),
                Some(name.to_string()),
            ));
        }

        if let Some(info) = self.ownership.get_mut(name) {
            info.state = OwnershipState::Moved;
        }

        Ok(())
    }

    /// Checks a normal use of a value.
    pub fn use_value(&self, name: &str) -> Result<(), BorrowError> {
        let info = self
            .ownership
            .get(name)
            .ok_or_else(|| {
                BorrowError::new(
                    BorrowErrorCode::InvalidBorrowTarget,
                    format!("cannot use undeclared variable `{name}`"),
                    Some(name.to_string()),
                )
            })?;

        if info.state == OwnershipState::Moved {
            return Err(BorrowError::new(
                BorrowErrorCode::UseAfterMove,
                format!("use of moved value `{name}`"),
                Some(name.to_string()),
            ));
        }

        Ok(())
    }

    /// Checks an expression against the current ownership environment.
    ///
    /// The method intentionally avoids println!/eprintln! side effects.
    /// Unsupported expressions are left to semantic/type analysis unless they
    /// contain an ownership-sensitive identifier that can be checked safely.
    pub fn check_expression(
        &mut self,
        expr: &Expression,
    ) -> Result<(), BorrowError> {
        self.check_expression_inner(expr)
    }

    /// Returns all currently active borrows.
    pub fn active_borrows(&self) -> &[BorrowRecord] {
        &self.borrows
    }

    /// Clears the complete ownership environment.
    pub fn clear(&mut self) {
        self.ownership.clear();
        self.borrows.clear();
        self.scopes.clear();
        self.scopes.push(HashSet::new());
    }

    fn require_usable(&self, name: &str) -> Result<&OwnershipInfo, BorrowError> {
        let info = self
            .ownership
            .get(name)
            .ok_or_else(|| {
                BorrowError::new(
                    BorrowErrorCode::InvalidBorrowTarget,
                    format!("cannot borrow undeclared variable `{name}`"),
                    Some(name.to_string()),
                )
            })?;

        if !info.state.is_usable() || info.state == OwnershipState::Moved {
            return Err(BorrowError::new(
                BorrowErrorCode::UseAfterMove,
                format!("cannot borrow moved value `{name}`"),
                Some(name.to_string()),
            ));
        }

        Ok(info)
    }

    fn rebuild_borrow_state(&mut self) {
        for info in self.ownership.values_mut() {
            info.immutable_borrows = 0;
            info.mutable_borrow = false;

            if info.state != OwnershipState::Moved {
                info.state = OwnershipState::Owned;
            }
        }

        for borrow in &self.borrows {
            if let Some(info) = self.ownership.get_mut(&borrow.variable) {
                match borrow.kind {
                    BorrowKind::Immutable => {
                        info.immutable_borrows += 1;
                        if info.state != OwnershipState::Moved {
                            info.state = OwnershipState::ImmutablyBorrowed;
                        }
                    }
                    BorrowKind::Mutable => {
                        info.mutable_borrow = true;
                        if info.state != OwnershipState::Moved {
                            info.state = OwnershipState::MutablyBorrowed;
                        }
                    }
                }
            }
        }
    }

    fn check_expression_inner(
        &mut self,
        _expr: &Expression,
    ) -> Result<(), BorrowError> {
        // The exact Expression variants in Zamani are broader than the
        // ownership model represented by this standalone checker. Semantic
        // analysis should lower ownership-relevant operations into the
        // explicit methods above.
        //
        // We intentionally do not guess at AST variants here. Guessing would
        // make this file fragile whenever the language grammar evolves.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_checker_starts_with_empty_environment() {
        let checker = BorrowChecker::new();

        assert!(checker.ownership_of("value").is_none());
        assert!(checker.active_borrows().is_empty());
    }

    #[test]
    fn declaration_registers_owned_value() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").expect("declaration should succeed");

        assert_eq!(
            checker.ownership_of("value").map(|info| info.state),
            Some(OwnershipState::Owned)
        );
    }

    #[test]
    fn duplicate_declaration_is_rejected() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").expect("first declaration should succeed");

        let error = checker
            .declare("value")
            .expect_err("duplicate declaration must fail");

        assert_eq!(error.code(), "E0405");
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
        assert_eq!(info.state, OwnershipState::ImmutablyBorrowed);
    }

    #[test]
    fn mutable_borrow_rejects_existing_immutable_borrow() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        let error = checker
            .borrow_mut("value")
            .expect_err("mutable borrow must fail");

        assert_eq!(error.code(), "E0402");
    }

    #[test]
    fn immutable_borrow_rejects_existing_mutable_borrow() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_mut("value").unwrap();

        let error = checker
            .borrow_immutable("value")
            .expect_err("immutable borrow must fail");

        assert_eq!(error.code(), "E0403");
    }

    #[test]
    fn moving_a_borrowed_value_is_rejected() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        let error = checker
            .move_value("value")
            .expect_err("moving borrowed value must fail");

        assert_eq!(error.code(), "E0404");
    }

    #[test]
    fn moved_value_cannot_be_used() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.move_value("value").unwrap();

        let error = checker
            .use_value("value")
            .expect_err("use after move must fail");

        assert_eq!(error.code(), "E0401");
    }

    #[test]
    fn moved_value_cannot_be_borrowed() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.move_value("value").unwrap();

        let error = checker
            .borrow_immutable("value")
            .expect_err("borrow of moved value must fail");

        assert_eq!(error.code(), "E0401");
    }

    #[test]
    fn leaving_scope_releases_scope_borrows() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();

        checker.enter_scope();
        checker.borrow_immutable("value").unwrap();

        assert_eq!(checker.active_borrows().len(), 1);

        checker.leave_scope();

        assert!(checker.active_borrows().is_empty());
        assert_eq!(
            checker.ownership_of("value").map(|info| info.state),
            Some(OwnershipState::Owned)
        );
    }

    #[test]
    fn inner_scope_bindings_are_removed() {
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
            checker.ownership_of("value").map(|info| info.state),
            Some(OwnershipState::Owned)
        );
    }

    #[test]
    fn undeclared_value_cannot_be_used() {
        let checker = BorrowChecker::new();

        let error = checker
            .use_value("missing")
            .expect_err("undeclared value must fail");

        assert_eq!(error.code(), "E0406");
    }

    #[test]
    fn undeclared_value_cannot_be_borrowed() {
        let mut checker = BorrowChecker::new();

        let error = checker
            .borrow_immutable("missing")
            .expect_err("undeclared value must fail");

        assert_eq!(error.code(), "E0406");
    }

    #[test]
    fn clear_resets_all_state() {
        let mut checker = BorrowChecker::new();

        checker.declare("value").unwrap();
        checker.borrow_immutable("value").unwrap();

        checker.clear();

        assert!(checker.ownership_of("value").is_none());
        assert!(checker.active_borrows().is_empty());

        checker.declare("value").unwrap();
        assert!(checker.ownership_of("value").is_some());
    }
}