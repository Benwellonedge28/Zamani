
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Symbol Table
//!
//! Canonical deterministic storage for symbols discovered during semantic
//! analysis.
//!
//! ## Purpose
//!
//! Store source-level symbol records and support exact-scope lookup,
//! lexical lookup over a caller-supplied scope chain, identity lookup,
//! deterministic iteration, removal, and duplicate detection.
//!
//! ## Architectural boundaries
//!
//! This module owns:
//! - symbol-record storage;
//! - indexing symbols by declaration identity;
//! - indexing symbols by `(scope, namespace, name)`;
//! - duplicate identity and duplicate-name detection;
//! - deterministic storage and iteration;
//! - consistency-preserving insertion and removal.
//!
//! This module does not own:
//! - AST declarations or AST node identities;
//! - semantic ID allocation;
//! - scope creation, nesting, or visibility rules;
//! - declaration collection or import processing;
//! - name-resolution policy;
//! - type inference or type compatibility;
//! - effects, capabilities, resources, contracts, or domain validation;
//! - target selection, hardware allocation, or IR lowering.
//!
//! ## Integration contract
//!
//! `symbols/declarations.rs` collects declarations and inserts records.
//! `symbols/scopes.rs` owns scope creation and supplies lookup scope chains.
//! `symbols/resolution.rs` performs name resolution using the lookup methods.
//! `symbols/imports.rs` manages imported and exported declarations.
//! `symbols/cycles.rs` analyzes declaration dependencies, not storage indexes.
//!
//! The `Id`, `ScopeId`, and `T` parameters are supplied by the semantic
//! subsystem. In particular, the table does not define a competing semantic
//! ID allocator or AST representation.
//!
//! Namespaces are open strings rather than a closed list of language domains.
//! Their spelling and meaning are governed by the semantic and language
//! specifications. A namespace identifies a name category; it is not a
//! computational-domain or hardware-capacity limit.
//!
//! ## Determinism
//!
//! The indexes use `BTreeMap`. Iteration is ordered by scope ID, namespace,
//! and name. Identity iteration is ordered by symbol ID. This provides
//! deterministic ordering for a fixed set of keys, independently of insertion
//! order.
//!
//! Diagnostic ordering remains the responsibility of the diagnostics layer.
//!
//! ## Scalability and operational limits
//!
//! This module defines no fixed maximum number of symbols, scopes, domains,
//! declarations, or resource requirements. Storage grows as records are
//! inserted and is limited by available resources and the representational
//! limits of the chosen ID types and allocator.
//!
//! A caller that needs an operational budget must enforce it through the
//! semantic configuration/session layer and report budget exhaustion
//! explicitly. It must not interpret an incomplete table as complete.
//!
//! ## Mutation and error handling
//!
//! Insertion is atomic with respect to the table's logical indexes: rejected
//! insertions do not change the table. Duplicate names are rejected within
//! the same scope and namespace. The same spelling may occur in another
//! namespace or scope.
//!
//! The table does not itself decide whether shadowing is legal. That is a
//! language rule enforced by the scope and declaration layers.
//!
//! No global mutable state, interior mutability, or unsafe code is used.

use core::fmt;
use std::collections::BTreeMap;

/// A namespace key for distinguishing categories of source-level names.
///
/// The value is open-ended so language extensions can add namespaces without
/// editing this module. Namespace spelling and compatibility are validated by
/// the semantic layer that constructs the key.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SymbolNamespace(String);

impl SymbolNamespace {
    /// Construct a namespace key.
    ///
    /// Empty and whitespace-only namespaces are rejected. The supplied value
    /// is otherwise preserved exactly; normalization belongs to the
    /// language-defined name-resolution layer.
    pub fn new(value: impl Into<String>) -> Result<Self, SymbolTableError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(SymbolTableError::EmptyNamespace);
        }

        Ok(Self(value))
    }

    /// Return the namespace's exact stored spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SymbolNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The key identifying one symbol within a declaring scope and namespace.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SymbolKey<ScopeId> {
    scope: ScopeId,
    namespace: SymbolNamespace,
    name: String,
}

/// A stored semantic symbol record.
///
/// `Id` is the semantic declaration identity, `ScopeId` is the ID of the
/// declaring scope, and `T` is the declaration payload owned by the semantic
/// subsystem.
///
/// Key fields are private so callers cannot mutate them without updating the
/// table's indexes. Use the accessors to inspect them and `value_mut` to update
/// the payload without changing symbol identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolRecord<Id, ScopeId, T> {
    id: Id,
    scope: ScopeId,
    namespace: SymbolNamespace,
    name: String,
    value: T,
}

impl<Id, ScopeId, T> SymbolRecord<Id, ScopeId, T> {
    /// Create a record for insertion into a symbol table.
    ///
    /// This constructor validates the name and namespace. It does not insert
    /// the record or establish that its ID or scope is unique.
    pub fn new(
        id: Id,
        scope: ScopeId,
        namespace: SymbolNamespace,
        name: impl Into<String>,
        value: T,
    ) -> Result<Self, SymbolTableError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(SymbolTableError::EmptyName);
        }

        Ok(Self {
            id,
            scope,
            namespace,
            name,
            value,
        })
    }

    /// Return the stable semantic identity supplied by the caller.
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Return the declaring scope identity.
    pub fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Return the name namespace.
    pub fn namespace(&self) -> &SymbolNamespace {
        &self.namespace
    }

    /// Return the exact stored source-level name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the symbol payload.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Mutably access the payload without changing the table's indexes.
    ///
    /// Mutating the payload must not change the record's ID, scope, namespace,
    /// or name. Those fields are intentionally not exposed mutably.
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Consume the record and return its payload.
    pub fn into_value(self) -> T {
        self.value
    }
}

/// Errors returned by symbol-table operations.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SymbolTableError {
    /// A symbol name was empty or consisted only of whitespace.
    EmptyName,

    /// A namespace was empty or consisted only of whitespace.
    EmptyNamespace,

    /// Another symbol already uses this ID.
    DuplicateId,

    /// Another symbol already uses this name in this scope and namespace.
    DuplicateName,

    /// The requested symbol identity does not exist.
    UnknownId,

    /// The requested key does not identify a symbol in the specified scope.
    UnknownName,

    /// A requested operation would violate a table invariant.
    InvariantViolation,
}

impl fmt::Display for SymbolTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => f.write_str("symbol name must not be empty"),
            Self::EmptyNamespace => {
                f.write_str("symbol namespace must not be empty")
            }
            Self::DuplicateId => {
                f.write_str("symbol identity is already registered")
            }
            Self::DuplicateName => {
                f.write_str("symbol name already exists in this scope and namespace")
            }
            Self::UnknownId => {
                f.write_str("symbol identity is not registered")
            }
            Self::UnknownName => {
                f.write_str("symbol name is not registered in this scope")
            }
            Self::InvariantViolation => {
                f.write_str("symbol table index invariant was violated")
            }
        }
    }
}

impl std::error::Error for SymbolTableError {}

/// A deterministic symbol store.
///
/// `ScopeId` and `Id` must have a total ordering so that indexes and iteration
/// remain deterministic. The table does not allocate or reinterpret either ID.
#[derive(Clone, Debug)]
pub struct SymbolTable<Id, ScopeId, T> {
    /// Primary index: declaring scope, namespace, and name.
    by_key: BTreeMap<SymbolKey<ScopeId>, Id>,

    /// Record index: declaration identity to complete symbol record.
    by_id: BTreeMap<Id, SymbolRecord<Id, ScopeId, T>>,
}

impl<Id, ScopeId, T> Default for SymbolTable<Id, ScopeId, T>
where
    Id: Ord + Clone,
    ScopeId: Ord + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Id, ScopeId, T> SymbolTable<Id, ScopeId, T>
where
    Id: Ord + Clone,
    ScopeId: Ord + Clone,
{
    /// Create an empty symbol table.
    pub fn new() -> Self {
        Self {
            by_key: BTreeMap::new(),
            by_id: BTreeMap::new(),
        }
    }

    /// Return the number of registered symbols.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// Return whether the table contains no symbols.
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// Insert a symbol without replacing an existing record.
    ///
    /// The operation checks both identity and scoped-name uniqueness before
    /// changing either index. A rejected insertion leaves the table unchanged.
    pub fn insert(
        &mut self,
        record: SymbolRecord<Id, ScopeId, T>,
    ) -> Result<(), SymbolTableError> {
        let key = SymbolKey {
            scope: record.scope.clone(),
            namespace: record.namespace.clone(),
            name: record.name.clone(),
        };

        if self.by_id.contains_key(&record.id) {
            return Err(SymbolTableError::DuplicateId);
        }

        if self.by_key.contains_key(&key) {
            return Err(SymbolTableError::DuplicateName);
        }

        // No fallible operations occur between these two index updates.
        // Both keys have been checked for absence above.
        let id = record.id.clone();
        self.by_key.insert(key, id.clone());
        self.by_id.insert(id, record);

        Ok(())
    }

    /// Look up a symbol by declaration identity.
    pub fn get(&self, id: &Id) -> Option<&SymbolRecord<Id, ScopeId, T>> {
        self.by_id.get(id)
    }

    /// Mutably access a symbol by declaration identity.
    ///
    /// Only the payload is mutable, preserving both index invariants.
    pub fn get_mut(
        &mut self,
        id: &Id,
    ) -> Option<&mut SymbolRecord<Id, ScopeId, T>> {
        self.by_id.get_mut(id)
    }

    /// Look up a name in exactly one scope and namespace.
    pub fn lookup_exact(
        &self,
        scope: &ScopeId,
        namespace: &SymbolNamespace,
        name: &str,
    ) -> Option<&SymbolRecord<Id, ScopeId, T>> {
        let key = SymbolKey {
            scope: scope.clone(),
            namespace: namespace.clone(),
            name: name.to_owned(),
        };

        let id = self.by_key.get(&key)?;
        self.by_id.get(id)
    }

    /// Resolve a name using a scope chain supplied by the scope subsystem.
    ///
    /// `scopes` must be ordered from innermost to outermost. The first match
    /// wins, allowing lexical shadowing when the language's scope rules permit
    /// it. This method does not construct, validate, or own that scope chain.
    pub fn lookup_in_scopes<'a, I>(
        &'a self,
        scopes: I,
        namespace: &SymbolNamespace,
        name: &str,
    ) -> Option<&'a SymbolRecord<Id, ScopeId, T>>
    where
        I: IntoIterator<Item = &'a ScopeId>,
        ScopeId: 'a,
        Id: 'a,
        T: 'a,
    {
        for scope in scopes {
            if let Some(record) = self.lookup_exact(scope, namespace, name) {
                return Some(record);
            }
        }

        None
    }

    /// Return all symbols in one scope in deterministic namespace/name order.
    pub fn in_scope<'a>(
        &'a self,
        scope: &ScopeId,
    ) -> impl Iterator<Item = &'a SymbolRecord<Id, ScopeId, T>> + 'a {
        self.by_id
            .values()
            .filter(move |record| &record.scope == scope)
    }

    /// Iterate over every record in deterministic symbol-ID order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &SymbolRecord<Id, ScopeId, T>> {
        self.by_id.values()
    }

    /// Remove a symbol by ID and return its payload.
    ///
    /// Both indexes are updated together. If the secondary index is
    /// inconsistent, the operation returns an error instead of silently
    /// leaving an orphaned index entry.
    pub fn remove(&mut self, id: &Id) -> Result<T, SymbolTableError> {
        let record = self
            .by_id
            .get(id)
            .ok_or(SymbolTableError::UnknownId)?;

        let key = SymbolKey {
            scope: record.scope.clone(),
            namespace: record.namespace.clone(),
            name: record.name.clone(),
        };

        match self.by_key.get(&key) {
            Some(indexed_id) if indexed_id == id => {}
            _ => return Err(SymbolTableError::InvariantViolation),
        }

        self.by_key.remove(&key);
        let record = self
            .by_id
            .remove(id)
            .ok_or(SymbolTableError::InvariantViolation)?;

        Ok(record.value)
    }

    /// Remove every symbol declared in a scope.
    ///
    /// Scope lifecycle and whether a scope may be discarded are owned by
    /// `symbols/scopes.rs`. This method only removes the scope's records.
    ///
    /// The returned payloads follow deterministic symbol-ID order.
    pub fn remove_scope(
        &mut self,
        scope: &ScopeId,
    ) -> Result<Vec<T>, SymbolTableError> {
        let ids: Vec<Id> = self
            .by_id
            .values()
            .filter(|record| &record.scope == scope)
            .map(|record| record.id.clone())
            .collect();

        // Verify the entire removal set before mutating either index.
        for id in &ids {
            let record = self
                .by_id
                .get(id)
                .ok_or(SymbolTableError::InvariantViolation)?;

            let key = SymbolKey {
                scope: record.scope.clone(),
                namespace: record.namespace.clone(),
                name: record.name.clone(),
            };

            if self.by_key.get(&key) != Some(id) {
                return Err(SymbolTableError::InvariantViolation);
            }
        }

        let mut removed = Vec::with_capacity(ids.len());

        for id in ids {
            let record = self
                .by_id
                .remove(&id)
                .ok_or(SymbolTableError::InvariantViolation)?;

            let key = SymbolKey {
                scope: record.scope.clone(),
                namespace: record.namespace.clone(),
                name: record.name.clone(),
            };

            self.by_key.remove(&key);
            removed.push(record.value);
        }

        Ok(removed)
    }

    /// Check the consistency of both internal indexes.
    ///
    /// Intended for tests, debug assertions, and integration validation.
    /// This is a linear-time check and should not be run automatically on
    /// every production lookup.
    pub fn validate_invariants(&self) -> Result<(), SymbolTableError> {
        if self.by_id.len() != self.by_key.len() {
            return Err(SymbolTableError::InvariantViolation);
        }

        for (id, record) in &self.by_id {
            if id != &record.id {
                return Err(SymbolTableError::InvariantViolation);
            }

            let key = SymbolKey {
                scope: record.scope.clone(),
                namespace: record.namespace.clone(),
                name: record.name.clone(),
            };

            if self.by_key.get(&key) != Some(id) {
                return Err(SymbolTableError::InvariantViolation);
            }
        }

        for (key, id) in &self.by_key {
            let record = self
                .by_id
                .get(id)
                .ok_or(SymbolTableError::InvariantViolation)?;

            if record.scope != key.scope
                || record.namespace != key.namespace
                || record.name != key.name
            {
                return Err(SymbolTableError::InvariantViolation);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Table = SymbolTable<u64, u64, &'static str>;

    fn namespace(value: &str) -> SymbolNamespace {
        SymbolNamespace::new(value).expect("valid namespace")
    }

    fn record(
        id: u64,
        scope: u64,
        ns: &str,
        name: &str,
        value: &'static str,
    ) -> SymbolRecord<u64, u64, &'static str> {
        SymbolRecord::new(id, scope, namespace(ns), name, value)
            .expect("valid symbol record")
    }

    #[test]
    fn inserts_and_resolves_exact_symbol() {
        let mut table = Table::new();
        table
            .insert(record(1, 10, "value", "answer", "integer"))
            .expect("insert");

        assert_eq!(
            table
                .lookup_exact(&10, &namespace("value"), "answer")
                .map(SymbolRecord::value),
            Some(&"integer")
        );
        assert_eq!(table.len(), 1);
        table.validate_invariants().expect("consistent indexes");
    }

    #[test]
    fn rejects_duplicate_identity_without_mutation() {
        let mut table = Table::new();
        table
            .insert(record(1, 10, "value", "first", "a"))
            .expect("insert");

        assert_eq!(
            table.insert(record(1, 10, "value", "second", "b")),
            Err(SymbolTableError::DuplicateId)
        );

        assert_eq!(table.len(), 1);
        assert!(table.lookup_exact(&10, &namespace("value"), "second").is_none());
        table.validate_invariants().expect("consistent indexes");
    }

    #[test]
    fn rejects_duplicate_name_in_same_scope_and_namespace() {
        let mut table = Table::new();
        table
            .insert(record(1, 10, "value", "item", "first"))
            .expect("insert");

        assert_eq!(
            table.insert(record(2, 10, "value", "item", "second")),
            Err(SymbolTableError::DuplicateName)
        );

        assert_eq!(table.len(), 1);
        table.validate_invariants().expect("consistent indexes");
    }

    #[test]
    fn permits_same_name_in_different_scopes_and_namespaces() {
        let mut table = Table::new();
        table
            .insert(record(1, 10, "value", "item", "local"))
            .expect("insert");
        table
            .insert(record(2, 11, "value", "item", "outer"))
            .expect("insert");
        table
            .insert(record(3, 10, "type", "item", "named type"))
            .expect("insert");

        assert_eq!(table.len(), 3);
        table.validate_invariants().expect("consistent indexes");
    }

    #[test]
    fn lexical_lookup_prefers_innermost_scope() {
        let mut table = Table::new();
        table
            .insert(record(1, 10, "value", "item", "outer"))
            .expect("insert");
        table
            .insert(record(2, 11, "value", "item", "inner"))
            .expect("insert");

        let scopes = [&11, &10];
        assert_eq!(
            table
                .lookup_in_scopes(scopes, &namespace("value"), "item")
                .map(SymbolRecord::value),
            Some(&"inner")
        );
    }

    #[test]
    fn missing_name_does_not_match_another_namespace() {
        let mut table = Table::new();
        table
            .insert(record(1, 10, "type", "Thing", "type declaration"))
            .expect("insert");

        assert!(table
            .lookup_exact(&10, &namespace("value"), "Thing")
            .is_none());
    }

    #[test]
    fn remove_updates_both_indexes() {
        let mut table = Table::new();
        table
            .insert(record(1, 10, "value", "item", "payload"))
            .expect("insert");

        assert_eq!(table.remove(&1), Ok("payload"));
        assert!(table.get(&1).is_none());
        assert!(table.lookup_exact(&10, &namespace("value"), "item").is_none());
        assert!(table.is_empty());
        table.validate_invariants().expect("consistent indexes");
    }

    #[test]
    fn remove_scope_removes_only_that_scopes_records() {
        let mut table = Table::new();
        table
            .insert(record(1, 10, "value", "first", "a"))
            .expect("insert");
        table
            .insert(record(2, 10, "value", "second", "b"))
            .expect("insert");
        table
            .insert(record(3, 11, "value", "third", "c"))
            .expect("insert");

        let removed = table.remove_scope(&10).expect("remove scope");
        assert_eq!(removed.len(), 2);
        assert_eq!(table.len(), 1);
        assert!(table.get(&3).is_some());
        table.validate_invariants().expect("consistent indexes");
    }

    #[test]
    fn rejects_empty_names_and_namespaces() {
        assert!(matches!(
            SymbolNamespace::new("  "),
            Err(SymbolTableError::EmptyNamespace)
        ));

        assert!(matches!(
            SymbolRecord::new(1_u64, 1_u64, namespace("value"), " ", "x"),
            Err(SymbolTableError::EmptyName)
        ));
    }

    #[test]
    fn payload_mutation_preserves_indexes() {
        let mut table = Table::new();
        table
            .insert(record(1, 10, "value", "item", "before"))
            .expect("insert");

        *table.get_mut(&1).expect("record").value_mut() = "after";

        assert_eq!(
            table
                .lookup_exact(&10, &namespace("value"), "item")
                .map(SymbolRecord::value),
            Some(&"after")
        );
        table.validate_invariants().expect("consistent indexes");
    }

    #[test]
    fn deterministic_iteration_is_independent_of_insert_order() {
        let mut left = Table::new();
        let mut right = Table::new();

        left.insert(record(2, 10, "value", "b", "b")).expect("insert");
        left.insert(record(1, 10, "value", "a", "a")).expect("insert");

        right.insert(record(1, 10, "value", "a", "a")).expect("insert");
        right.insert(record(2, 10, "value", "b", "b")).expect("insert");

        let left_ids: Vec<_> = left.iter().map(SymbolRecord::id).copied().collect();
        let right_ids: Vec<_> = right.iter().map(SymbolRecord::id).copied().collect();

        assert_eq!(left_ids, right_ids);
    }
}
