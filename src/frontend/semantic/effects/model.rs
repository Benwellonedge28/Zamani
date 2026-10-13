#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Effect Model
//!
//! File: `src/frontend/semantic/effects/model.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Define the canonical, target-independent semantic representation of
//! effects, parameterized effect instances, effect variables, and effect rows.
//!
//! ## Authority
//!
//! - `grammar/specification/semantic-model.md`:
//!   normative semantic meaning.
//! - `grammar/spec/effects.md`:
//!   effect-system rules.
//! - `grammar/effects/effect-sets.g4`:
//!   effect-set syntax.
//! - `grammar/effects/effect-declarations.g4`:
//!   effect-declaration syntax.
//! - `grammar/effects/effect-types.g4`:
//!   effect-qualified type syntax.
//! - `src/frontend/ast/node/effects/effect_kind.rs`:
//!   canonical open-world AST effect identity.
//!
//! Grammar determines which source forms are syntactically valid. This module
//! determines how already-resolved effect information is represented.
//!
//! ## Ownership
//!
//! This file owns:
//! - semantic effect arguments and their structural validation;
//! - parameterized semantic effect instances;
//! - effect variables used by polymorphic effect rows;
//! - normalized effect sets and effect rows;
//! - explicit, deterministic set and row operations.
//!
//! This file does not own:
//! - source parsing or AST construction;
//! - effect declaration or name resolution;
//! - type inference or effect inference;
//! - effect compatibility policy or subtyping;
//! - effect authorization or security policy;
//! - capability evidence or target availability;
//! - resource feasibility or allocation;
//! - contracts, effect handlers, or runtime execution;
//! - domain IR, quantum IR, scheduling, routing, or lowering.
//!
//! ## Integration contract
//!
//! - `effects/inference.rs` constructs and combines effect rows.
//! - `effects/checking.rs` validates declared rows and language-defined
//!   compatibility rules.
//! - `effects/authorization.rs` evaluates permissions separately.
//! - `types/ty.rs` and `types/constraints.rs` remain authoritative for types
//!   and type constraints. `TypeKey` here is only an opaque, resolved type
//!   identity supplied by those modules.
//! - `model/facts.rs` associates effect facts with canonical AST node IDs.
//! - `model/program.rs` exposes finalized semantic results.
//! - `capabilities/` and `resources/` own their respective obligations.
//!
//! This file deliberately does not assume that those consumer modules already
//! exist or invent their APIs. Its data types are self-contained apart from
//! the canonical AST `EffectKind`.
//!
//! ## POCO-REAF and scalability
//!
//! No fixed effect count, argument count, generic arity, domain count,
//! hardware capacity, qubit count, or machine-size ceiling is imposed.
//! Collection growth is bounded only by available resources and the explicit
//! operational budgets enforced by the analysis session.
//!
//! Rust collections and strings are finite representations. Consequently,
//! this module does not claim to represent an actually infinite in-memory
//! value. Symbolic quantities are represented symbolically rather than
//! truncated to a machine capacity.
//!
//! ## Determinism
//!
//! Effect sets use ordered collections. Equivalent instances have stable
//! equality, ordering, and serialization order. No hash-map iteration order,
//! process state, timestamps, or hardware state contribute to identity.
//!
//! ## Validity
//!
//! A well-formed effect identity is not proof that the effect is declared,
//! supported, authorized, or available on a target. Those are separate
//! semantic obligations. Unknown effects must not silently become pure effects.
//!
//! ## Freeze criteria
//!
//! - AST effect identity integration compiles.
//! - Argument and row validation tests pass.
//! - Set operations are deterministic and preserve parameter distinctions.
//! - Effect-variable handling is explicit.
//! - No authorization, target discovery, or resource allocation occurs here.
//! - Workspace checks pass after the parent modules are integrated.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::frontend::ast::node::effects::effect_kind::EffectKind;

/// Schema version for the serialized semantic effect model.
///
/// This is independent of the language edition, AST schema, and compiler
/// release. Increment it when serialized meanings or representation change
/// incompatibly.
pub const SEMANTIC_EFFECT_MODEL_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Shared structural validation
// =============================================================================

/// Errors raised when a semantic effect-model value is structurally malformed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EffectModelError {
    /// A required identifier or textual payload was empty.
    EmptyValue {
        /// Name of the field being validated.
        field: &'static str,
    },

    /// A value contains a NUL character, which is not accepted in identifiers.
    NulCharacter {
        /// Name of the field being validated.
        field: &'static str,
    },

    /// A value contains a forbidden control character.
    ControlCharacter {
        /// Name of the field being validated.
        field: &'static str,
    },

    /// An effect variable has an invalid identifier.
    InvalidVariableName,

    /// An effect variable was declared more than once in one row.
    DuplicateVariable {
        /// The repeated variable name.
        name: String,
    },

    /// A serialized or externally constructed value is internally inconsistent.
    InvalidStructure {
        /// Explanation suitable for diagnostics and tests.
        message: String,
    },
}

impl fmt::Display for EffectModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue { field } => {
                write!(f, "{field} must not be empty")
            }
            Self::NulCharacter { field } => {
                write!(f, "{field} must not contain a NUL character")
            }
            Self::ControlCharacter { field } => {
                write!(f, "{field} must not contain control characters")
            }
            Self::InvalidVariableName => {
                f.write_str("invalid effect-variable name")
            }
            Self::DuplicateVariable { name } => {
                write!(f, "duplicate effect variable `{name}`")
            }
            Self::InvalidStructure { message } => {
                write!(f, "invalid semantic effect structure: {message}")
            }
        }
    }
}

impl Error for EffectModelError {}

fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), EffectModelError> {
    if value.is_empty() {
        return Err(EffectModelError::EmptyValue { field });
    }

    if value.contains('\0') {
        return Err(EffectModelError::NulCharacter { field });
    }

    if value.chars().any(char::is_control) {
        return Err(EffectModelError::ControlCharacter { field });
    }

    Ok(())
}

// =============================================================================
// Opaque semantic keys
// =============================================================================

/// Opaque, resolved semantic type identity used as an effect argument.
///
/// This is intentionally not the canonical type representation. It provides
/// a stable integration boundary without duplicating `types/ty.rs`.
///
/// The type resolver must construct the key from its canonical resolved type
/// representation. Callers must not use source text as a resolved type key
/// unless the language specification explicitly defines that representation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TypeKey(String);

impl TypeKey {
    /// Creates a key from a canonical type identity.
    ///
    /// This validates structure only. It does not resolve or prove the
    /// existence of the referenced type.
    pub fn new(value: impl Into<String>) -> Result<Self, EffectModelError> {
        let value = value.into();
        validate_text("type key", &value)?;
        Ok(Self(value))
    }

    /// Returns the canonical key supplied by the type subsystem.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TypeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Opaque, resolved semantic declaration identity used as an effect argument.
///
/// Name resolution owns the mapping from source-level names to these keys.
/// This wrapper must not be used to claim that a declaration exists without
/// successful resolution.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeclarationKey(String);

impl DeclarationKey {
    /// Creates a key from a canonical declaration identity.
    pub fn new(value: impl Into<String>) -> Result<Self, EffectModelError> {
        let value = value.into();
        validate_text("declaration key", &value)?;
        Ok(Self(value))
    }

    /// Returns the canonical declaration key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeclarationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An opaque symbolic constant identity.
///
/// The constant evaluator owns expression evaluation and overflow behavior.
/// This model retains a symbolic key and never truncates it to a machine-sized
/// integer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConstantKey(String);

impl ConstantKey {
    /// Creates a symbolic constant key.
    pub fn new(value: impl Into<String>) -> Result<Self, EffectModelError> {
        let value = value.into();
        validate_text("constant key", &value)?;
        Ok(Self(value))
    }

    /// Returns the symbolic constant key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConstantKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Effect arguments
// =============================================================================

/// A value supplied to a parameterized effect.
///
/// These variants deliberately describe semantic argument identity rather
/// than implementing a second type system or constant evaluator.
///
/// `Symbolic` preserves an expression identity when its value cannot or should
/// not be evaluated at this phase. It must be a canonical identity supplied
/// by the relevant semantic subsystem, not an arbitrary display string.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EffectArgument {
    /// A resolved type argument.
    Type(TypeKey),

    /// A resolved declaration or value identity.
    Declaration(DeclarationKey),

    /// A symbolic constant identity.
    Constant(ConstantKey),

    /// A symbolic argument identity, for example a value-level parameter
    /// whose value is intentionally not evaluated during effect analysis.
    Symbolic(String),
}

impl EffectArgument {
    /// Creates a symbolic argument after structural validation.
    pub fn symbolic(value: impl Into<String>) -> Result<Self, EffectModelError> {
        let value = value.into();
        validate_text("symbolic effect argument", &value)?;
        Ok(Self::Symbolic(value))
    }

    /// Validates the structural contents of the argument.
    pub fn validate(&self) -> Result<(), EffectModelError> {
        match self {
            Self::Type(key) => validate_text("type key", key.as_str()),
            Self::Declaration(key) => {
                validate_text("declaration key", key.as_str())
            }
            Self::Constant(key) => {
                validate_text("constant key", key.as_str())
            }
            Self::Symbolic(value) => {
                validate_text("symbolic effect argument", value)
            }
        }
    }
}

impl fmt::Display for EffectArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type(key) => write!(f, "type({key})"),
            Self::Declaration(key) => write!(f, "decl({key})"),
            Self::Constant(key) => write!(f, "const({key})"),
            Self::Symbolic(value) => write!(f, "symbolic({value})"),
        }
    }
}

// =============================================================================
// Parameterized effect instances
// =============================================================================

/// One resolved semantic effect occurrence.
///
/// `EffectKind` is the canonical, extensible AST identity. The ordered
/// argument list distinguishes different instantiations of the same effect.
///
/// For example, an effect parameterized by a type or symbolic dimension can
/// have multiple distinct instances without adding new enum variants.
///
/// This structure does not prove that the effect is declared, permitted,
/// implemented, or available on a target.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EffectInstance {
    kind: EffectKind,
    arguments: Vec<EffectArgument>,
}

impl EffectInstance {
    /// Constructs an effect instance from a canonical effect kind and its
    /// already-resolved arguments.
    pub fn new(
        kind: EffectKind,
        arguments: Vec<EffectArgument>,
    ) -> Result<Self, EffectModelError> {
        for argument in &arguments {
            argument.validate()?;
        }

        Ok(Self { kind, arguments })
    }

    /// Constructs an unparameterized effect instance.
    #[must_use]
    pub fn simple(kind: EffectKind) -> Self {
        Self {
            kind,
            arguments: Vec::new(),
        }
    }

    /// Returns the canonical open-world effect identity.
    #[must_use]
    pub fn kind(&self) -> &EffectKind {
        &self.kind
    }

    /// Returns the ordered arguments.
    #[must_use]
    pub fn arguments(&self) -> &[EffectArgument] {
        &self.arguments
    }

    /// Returns whether this instance has no arguments.
    #[must_use]
    pub fn is_unparameterized(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Validates the structure of this instance.
    pub fn validate(&self) -> Result<(), EffectModelError> {
        // Re-validate because serde deserialization can bypass constructors.
        EffectKind::new(self.kind.namespace(), self.kind.name()).map_err(
            |error| EffectModelError::InvalidStructure {
                message: format!("invalid effect kind: {error}"),
            },
        )?;

        for argument in &self.arguments {
            argument.validate()?;
        }

        Ok(())
    }
}

impl fmt::Display for EffectInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;

        if !self.arguments.is_empty() {
            f.write_str("<")?;

            for (index, argument) in self.arguments.iter().enumerate() {
                if index != 0 {
                    f.write_str(", ")?;
                }
                write!(f, "{argument}")?;
            }

            f.write_str(">")?;
        }

        Ok(())
    }
}

// =============================================================================
// Effect variables and polymorphic rows
// =============================================================================

/// An effect-row variable.
///
/// Effect variables represent unknown or generic effect tails. They are not
/// runtime variables and do not identify hardware resources.
///
/// Variable identity is lexical and semantic, not based on memory addresses
/// or allocation order. The inference subsystem is responsible for resolving
/// substitutions and rejecting unbound variables where the language requires
/// a closed effect row.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EffectVariable {
    name: String,
}

impl EffectVariable {
    /// Creates an effect variable with a non-empty identifier.
    ///
    /// The identifier is preserved exactly. Identifier normalization and
    /// language-specific name rules belong to the symbol/type subsystem.
    pub fn new(name: impl Into<String>) -> Result<Self, EffectModelError> {
        let name = name.into();
        validate_text("effect variable", &name)?;

        if name.trim() != name {
            return Err(EffectModelError::InvalidVariableName);
        }

        Ok(Self { name })
    }

    /// Returns the variable's spelling.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for EffectVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}", self.name)
    }
}

/// A normalized, deterministic set of concrete effect instances.
///
/// The set deduplicates identical effect instances, but does not conflate
/// different arguments to the same effect kind.
///
/// This type does not implement subtyping, authorization, or effect
/// equivalence beyond structural identity. Those policies belong to checking.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct EffectSet {
    effects: BTreeSet<EffectInstance>,
}

impl EffectSet {
    /// Creates an empty effect set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a set from effect instances.
    ///
    /// Structurally identical instances are deduplicated. Invalid instances
    /// return an error instead of entering the set.
    pub fn from_effects(
        effects: impl IntoIterator<Item = EffectInstance>,
    ) -> Result<Self, EffectModelError> {
        let mut result = Self::new();

        for effect in effects {
            effect.validate()?;
            result.effects.insert(effect);
        }

        Ok(result)
    }

    /// Inserts one effect instance.
    ///
    /// Returns `true` if the set changed.
    pub fn insert(
        &mut self,
        effect: EffectInstance,
    ) -> Result<bool, EffectModelError> {
        effect.validate()?;
        Ok(self.effects.insert(effect))
    }

    /// Removes one exact effect instance.
    ///
    /// This is structural removal only; it does not mean that an effect has
    /// been handled, authorized, or semantically discharged.
    pub fn remove(&mut self, effect: &EffectInstance) -> bool {
        self.effects.remove(effect)
    }

    /// Returns whether an exact effect instance is present.
    #[must_use]
    pub fn contains(&self, effect: &EffectInstance) -> bool {
        self.effects.contains(effect)
    }

    /// Returns the number of distinct concrete instances.
    #[must_use]
    pub fn len(&self) -> usize {
        self.effects.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    /// Iterates over effects in deterministic order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &EffectInstance> {
        self.effects.iter()
    }

    /// Returns the union of two effect sets.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            effects: self.effects.union(&other.effects).cloned().collect(),
        }
    }

    /// Returns the structural intersection of two effect sets.
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            effects: self
                .effects
                .intersection(&other.effects)
                .cloned()
                .collect(),
        }
    }

    /// Returns the structural difference `self - other`.
    ///
    /// This operation is a data-structure operation only. Whether removing an
    /// effect is valid in a program is decided by the effect checker and
    /// handler semantics.
    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            effects: self
                .effects
                .difference(&other.effects)
                .cloned()
                .collect(),
        }
    }

    /// Returns whether every effect in `self` is structurally present in
    /// `other`.
    ///
    /// This is set inclusion, not language-defined effect subtyping.
    #[must_use]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.effects.is_subset(&other.effects)
    }

    /// Validates every contained instance.
    ///
    /// This is useful after deserialization, since serde can construct values
    /// without calling the public constructors.
    pub fn validate(&self) -> Result<(), EffectModelError> {
        for effect in &self.effects {
            effect.validate()?;
        }

        Ok(())
    }
}

impl IntoIterator for EffectSet {
    type Item = EffectInstance;
    type IntoIter = std::collections::btree_set::IntoIter<EffectInstance>;

    fn into_iter(self) -> Self::IntoIter {
        self.effects.into_iter()
    }
}

/// A normalized effect row: concrete effects plus zero or more open variables.
///
/// The row is an analysis representation. An open row means that additional
/// effects may be supplied by inference or generic substitution; it does not
/// mean that the computation is pure or that unknown effects are authorized.
///
/// Duplicate effect variables are rejected by the constructor. Concrete
/// effects are structurally deduplicated by `EffectSet`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct EffectRow {
    effects: EffectSet,
    variables: BTreeSet<EffectVariable>,
}

impl EffectRow {
    /// Creates a closed, empty effect row.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a closed row from concrete effects.
    pub fn closed(
        effects: impl IntoIterator<Item = EffectInstance>,
    ) -> Result<Self, EffectModelError> {
        Ok(Self {
            effects: EffectSet::from_effects(effects)?,
            variables: BTreeSet::new(),
        })
    }

    /// Creates a row from concrete effects and open variables.
    ///
    /// Duplicate variables are rejected explicitly rather than silently
    /// normalized away, because duplicate binders can indicate a malformed
    /// source or an incorrect inference result.
    pub fn new(
        effects: impl IntoIterator<Item = EffectInstance>,
        variables: impl IntoIterator<Item = EffectVariable>,
    ) -> Result<Self, EffectModelError> {
        let effects = EffectSet::from_effects(effects)?;
        let mut variable_set = BTreeSet::new();

        for variable in variables {
            validate_text("effect variable", variable.name())?;

            if !variable_set.insert(variable.clone()) {
                return Err(EffectModelError::DuplicateVariable {
                    name: variable.name().to_owned(),
                });
            }
        }

        Ok(Self {
            effects,
            variables: variable_set,
        })
    }

    /// Returns the concrete effects in deterministic order.
    #[must_use]
    pub fn effects(&self) -> &EffectSet {
        &self.effects
    }

    /// Iterates over open variables in deterministic order.
    pub fn variables(
        &self,
    ) -> impl ExactSizeIterator<Item = &EffectVariable> {
        self.variables.iter()
    }

    /// Returns whether this row is closed.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.variables.is_empty()
    }

    /// Returns whether this row has no concrete effects or open variables.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty() && self.variables.is_empty()
    }

    /// Returns the number of concrete effects.
    #[must_use]
    pub fn effect_count(&self) -> usize {
        self.effects.len()
    }

    /// Returns the number of distinct row variables.
    #[must_use]
    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }

    /// Returns a row containing the union of concrete effects and variables.
    ///
    /// This is structural composition, not a unification algorithm. If both
    /// rows contain a variable with the same name, the variable is represented
    /// once. The inference subsystem must enforce variable binding and scope
    /// rules before relying on this result.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            effects: self.effects.union(&other.effects),
            variables: self
                .variables
                .union(&other.variables)
                .cloned()
                .collect(),
        }
    }

    /// Validates the row's structural invariants.
    pub fn validate(&self) -> Result<(), EffectModelError> {
        self.effects.validate()?;

        for variable in &self.variables {
            validate_text("effect variable", variable.name())?;
        }

        Ok(())
    }
}

impl fmt::Display for EffectRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;

        let mut needs_separator = false;

        for effect in self.effects.iter() {
            if needs_separator {
                f.write_str(", ")?;
            }
            write!(f, "{effect}")?;
            needs_separator = true;
        }

        for variable in &self.variables {
            if needs_separator {
                f.write_str(", ")?;
            }
            write!(f, "{variable}")?;
            needs_separator = true;
        }

        f.write_str("}")
    }
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn kind(namespace: &str, name: &str) -> EffectKind {
        EffectKind::new(namespace, name)
            .expect("test effect kind must be valid")
    }

    fn effect(namespace: &str, name: &str) -> EffectInstance {
        EffectInstance::simple(kind(namespace, name))
    }

    #[test]
    fn effect_kinds_remain_open_world() {
        let custom = effect("future_domain", "new_effect");
        let quantum = effect("quantum", "measurement");

        assert_ne!(custom, quantum);
        assert_eq!(custom.kind().namespace(), "future_domain");
        assert_eq!(custom.kind().name(), "new_effect");
    }

    #[test]
    fn effect_instances_distinguish_arguments() {
        let kind = kind("domain", "operation");

        let first = EffectInstance::new(
            kind.clone(),
            vec![EffectArgument::symbolic("shape_a").unwrap()],
        )
        .unwrap();

        let second = EffectInstance::new(
            kind,
            vec![EffectArgument::symbolic("shape_b").unwrap()],
        )
        .unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn sets_deduplicate_identical_instances() {
        let item = effect("domain", "effect");
        let set = EffectSet::from_effects([item.clone(), item]).unwrap();

        assert_eq!(set.len(), 1);
    }

    #[test]
    fn set_union_and_difference_are_deterministic() {
        let first = EffectSet::from_effects([
            effect("domain", "a"),
            effect("domain", "b"),
        ])
        .unwrap();

        let second = EffectSet::from_effects([
            effect("domain", "b"),
            effect("domain", "c"),
        ])
        .unwrap();

        let union = first.union(&second);
        let difference = first.difference(&second);

        assert_eq!(union.len(), 3);
        assert_eq!(difference.len(), 1);
        assert!(difference.contains(&effect("domain", "a")));
    }

    #[test]
    fn row_variables_are_explicit() {
        let row = EffectRow::new(
            [effect("domain", "io")],
            [EffectVariable::new("e").unwrap()],
        )
        .unwrap();

        assert!(!row.is_closed());
        assert_eq!(row.effect_count(), 1);
        assert_eq!(row.variable_count(), 1);
    }

    #[test]
    fn duplicate_row_variables_are_rejected() {
        let variable = EffectVariable::new("e").unwrap();

        let result = EffectRow::new(
            [],
            [variable.clone(), variable],
        );

        assert!(matches!(
            result,
            Err(EffectModelError::DuplicateVariable { .. })
        ));
    }

    #[test]
    fn malformed_symbolic_arguments_are_rejected() {
        assert!(EffectArgument::symbolic("").is_err());
        assert!(EffectArgument::symbolic("invalid\0key").is_err());
    }

    #[test]
    fn row_format_is_stable() {
        let row = EffectRow::closed([
            effect("domain", "z"),
            effect("domain", "a"),
        ])
        .unwrap();

        assert_eq!(row.to_string(), "{domain:a, domain:z}");
    }

    #[test]
    fn empty_row_is_closed_and_empty() {
        let row = EffectRow::empty();

        assert!(row.is_closed());
        assert!(row.is_empty());
        assert_eq!(row.to_string(), "{}");
    }
}