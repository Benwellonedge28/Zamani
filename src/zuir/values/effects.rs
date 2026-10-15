
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR effect values
//!
//! Canonical path: `src/zuir/values/effects.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Represent effect identities, parameterized effect instances, effect-row
//! variables, and normalized effect rows in the canonical ZUIR value model.
//!
//! OWNS
//!
//! - Open-world, namespace-qualified effect identities.
//! - Resolved references used as effect arguments.
//! - Parameterized effect instances.
//! - Deterministically ordered effect rows.
//! - Local structural validation and configurable validation budgets.
//! - Explicit row-combination errors.
//!
//! DOES_NOT_OWN
//!
//! - Source parsing, grammar rules, or AST construction.
//! - Effect declaration lookup, name resolution, or inference.
//! - Type inference, type compatibility, or type definitions.
//! - Effect authorization, security policy, or capability evidence.
//! - Resource feasibility, physical allocation, or target discovery.
//! - Operation execution, handlers, scheduling, routing, or lowering.
//! - Source diagnostics rendering or whole-program serialization.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/types/type_def.rs`
//! - `src/frontend/semantic/effects/model.rs`
//! - `src/frontend/ast/node/effects/effect_kind.rs`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/effects/effect-sets.g4`
//! - `grammar/effects/effect-declarations.g4`
//! - `grammar/effects/effect-types.g4`
//!
//! ARCHITECTURAL BOUNDARY
//!
//! The frontend resolves source effect names before conversion to ZUIR.
//! This module deliberately does not import AST or semantic-model types.
//! A semantic-to-ZUIR adapter must perform that conversion explicitly.
//!
//! INTEGRATION
//!
//! - `ids.rs` owns TypeId, SymbolId, and ValueId.
//! - `values/mod.rs` declares and exports this module.
//! - `ir/effects.rs`, if introduced, owns operation/function effect attachment
//!   and effect-declaration semantics at the IR-node level.
//! - Semantic analysis owns effect inference and source-level checking.
//! - The verifier checks effect declarations, row compatibility, and operation
//!   contracts using the registered dialect definitions.
//! - Authorization and capability analysis remain separate.
//! - Serialization validates decoded values before they are trusted.
//!
//! SCALABILITY
//!
//! There is no closed list of effect kinds and no universal maximum effect
//! count, argument count, namespace depth, or hardware capacity.
//! Collections grow dynamically. Optional validation budgets are explicit
//! operational safeguards, not language-semantic limits.
//!
//! Finite memory, address space, execution time, and identity domains remain
//! real constraints. Exhaustion or budget violations must be reported rather
//! than silently truncating data.
//!
//! DETERMINISM
//!
//! Effect identities and arguments have deterministic ordering.
//! Effect rows use BTreeSet so equality and iteration do not depend on hash
//! randomization or insertion order.
//!
//! SEMANTIC BOUNDARY
//!
//! A structurally valid effect does not establish that it is declared,
//! authorized, supported by a target, or executable. Those decisions belong
//! to their respective semantic and compilation stages.
//!
//! FREEZE CRITERIA
//!
//! - No AST or frontend semantic-model dependency.
//! - No closed effect-kind enumeration.
//! - No implicit effect removal or meaning-changing normalization.
//! - Explicit handling of incompatible open row variables.
//! - Deserialized values can be revalidated.
//! - Configurable budgets fail explicitly and deterministically.
//! - Unit, serialization, integration, and workspace tests pass.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::zuir::ids::{SymbolId, TypeId, ValueId};

/// Serialized schema version for the ZUIR effect-value model.
///
/// This is independent of the language version, compiler release, and
/// semantic-model schema. Change it deliberately when serialized meaning
/// changes incompatibly.
pub const ZUIR_EFFECTS_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Validation errors and budgets
// =============================================================================

/// Structural validation errors for ZUIR effect values.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum EffectValidationError {
    /// A required textual component is empty.
    EmptyText {
        /// Field being validated.
        field: &'static str,
    },

    /// A textual component contains whitespace or a control character.
    InvalidText {
        /// Field being validated.
        field: &'static str,
    },

    /// A textual component contains a NUL character.
    NulCharacter {
        /// Field being validated.
        field: &'static str,
    },

    /// A validation counter exceeded the representable counter domain.
    CounterOverflow,

    /// The configured effect-count budget was exceeded.
    EffectBudgetExceeded {
        /// Number of effects encountered.
        actual: usize,
        /// Configured maximum, if any.
        limit: usize,
    },

    /// The configured argument-count budget was exceeded.
    ArgumentBudgetExceeded {
        /// Number of arguments encountered.
        actual: usize,
        /// Configured maximum, if any.
        limit: usize,
    },

    /// The configured text-byte budget was exceeded.
    TextBudgetExceeded {
        /// Number of text bytes encountered.
        actual: usize,
        /// Configured maximum, if any.
        limit: usize,
    },

    /// Two distinct open row variables cannot be combined by this row model.
    ConflictingRowTails {
        /// First row-tail variable.
        left: String,
        /// Second row-tail variable.
        right: String,
    },
}

impl fmt::Display for EffectValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { field } => {
                write!(f, "{field} must not be empty")
            }
            Self::InvalidText { field } => {
                write!(f, "{field} contains whitespace or a control character")
            }
            Self::NulCharacter { field } => {
                write!(f, "{field} must not contain a NUL character")
            }
            Self::CounterOverflow => {
                f.write_str("effect validation counter overflowed")
            }
            Self::EffectBudgetExceeded { actual, limit } => {
                write!(f, "effect budget exceeded: {actual} effects, limit {limit}")
            }
            Self::ArgumentBudgetExceeded { actual, limit } => {
                write!(
                    f,
                    "effect argument budget exceeded: {actual} arguments, limit {limit}"
                )
            }
            Self::TextBudgetExceeded { actual, limit } => {
                write!(
                    f,
                    "effect text budget exceeded: {actual} bytes, limit {limit}"
                )
            }
            Self::ConflictingRowTails { left, right } => {
                write!(
                    f,
                    "cannot combine open effect rows with distinct tails `{left}` and `{right}`"
                )
            }
        }
    }
}

impl Error for EffectValidationError {}

/// Optional operational limits for validating effect rows.
///
/// `None` means that this particular budget is not imposed by this value
/// object. A compiler or service handling untrusted input should supply
/// explicit limits through its compilation policy.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EffectValidationBudget {
    /// Maximum number of distinct effect instances.
    pub max_effects: Option<usize>,

    /// Maximum total number of effect arguments.
    pub max_arguments: Option<usize>,

    /// Maximum total UTF-8 bytes in effect names, row variables, and textual
    /// argument payloads.
    pub max_text_bytes: Option<usize>,
}

impl EffectValidationBudget {
    /// Create a budget with no local limits.
    ///
    /// This is useful for trusted in-memory values. It does not mean that
    /// actual allocation or execution resources are infinite.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_effects: None,
            max_arguments: None,
            max_text_bytes: None,
        }
    }

    /// Create a budget with explicit limits.
    #[must_use]
    pub const fn new(
        max_effects: Option<usize>,
        max_arguments: Option<usize>,
        max_text_bytes: Option<usize>,
    ) -> Self {
        Self {
            max_effects,
            max_arguments,
            max_text_bytes,
        }
    }
}

fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), EffectValidationError> {
    if value.is_empty() {
        return Err(EffectValidationError::EmptyText { field });
    }

    if value.contains('\0') {
        return Err(EffectValidationError::NulCharacter { field });
    }

    if value.chars().any(|ch| ch.is_whitespace() || ch.is_control()) {
        return Err(EffectValidationError::InvalidText { field });
    }

    Ok(())
}

fn add_count(
    current: usize,
    amount: usize,
) -> Result<usize, EffectValidationError> {
    current
        .checked_add(amount)
        .ok_or(EffectValidationError::CounterOverflow)
}

fn check_budget(
    actual: usize,
    limit: Option<usize>,
    make_error: impl FnOnce(usize, usize) -> EffectValidationError,
) -> Result<(), EffectValidationError> {
    if let Some(limit) = limit {
        if actual > limit {
            return Err(make_error(actual, limit));
        }
    }

    Ok(())
}

// =============================================================================
// Effect identity
// =============================================================================

/// An open-world, namespace-qualified effect identity.
///
/// Identity consists only of `namespace` and `name`. It does not encode
/// hardware, implementation, version, authorization, or target availability.
///
/// Names are preserved as supplied. This type does not perform alias
/// resolution, case folding, or source-name parsing.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct EffectId {
    namespace: String,
    name: String,
}

impl EffectId {
    /// Construct a structurally valid effect identity.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, EffectValidationError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_text("effect namespace", &namespace)?;
        validate_text("effect name", &name)?;

        Ok(Self { namespace, name })
    }

    /// Return the namespace exactly as stored.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the effect name exactly as stored.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return a display-oriented qualified form.
    ///
    /// Use the structured identity for equality and lookup. This string is
    /// for display and diagnostics; it is not a canonical parser format.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }

    /// Revalidate this value, including values created by deserialization.
    pub fn validate(&self) -> Result<(), EffectValidationError> {
        validate_text("effect namespace", &self.namespace)?;
        validate_text("effect name", &self.name)
    }
}

impl fmt::Display for EffectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

// =============================================================================
// Effect arguments
// =============================================================================

/// A validated opaque key for a resolved constant or symbolic constant.
///
/// The key must be produced by the canonical constant/evaluation subsystem.
/// This wrapper does not parse or evaluate a source expression.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct EffectConstantKey(String);

impl EffectConstantKey {
    /// Construct a constant key from a canonical, non-empty identity.
    pub fn new(value: impl Into<String>) -> Result<Self, EffectValidationError> {
        let value = value.into();
        validate_text("effect constant key", &value)?;
        Ok(Self(value))
    }

    /// Return the key as stored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Revalidate a value obtained from an untrusted source.
    pub fn validate(&self) -> Result<(), EffectValidationError> {
        validate_text("effect constant key", &self.0)
    }
}

/// A resolved argument to a parameterized effect.
///
/// The variants reference canonical ZUIR identities where possible. Opaque
/// keys are reserved for symbolic constants and extension-defined identities;
/// they are not substitutes for ordinary type, symbol, or value IDs.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum EffectArgument {
    /// A resolved canonical type identity.
    Type(TypeId),

    /// A resolved symbol identity.
    Symbol(SymbolId),

    /// A resolved IR value identity.
    Value(ValueId),

    /// A canonical constant or symbolic constant identity.
    Constant(EffectConstantKey),

    /// A symbolic argument key supplied by the relevant semantic subsystem.
    ///
    /// This is not source text to be reparsed.
    Symbolic(String),

    /// An extension-defined argument identity.
    ///
    /// The extension registry owns the payload's meaning and validation.
    Extension {
        /// Extension namespace.
        namespace: String,
        /// Canonical opaque key within that namespace.
        key: String,
    },
}

impl EffectArgument {
    /// Construct a symbolic argument after validating its key.
    pub fn symbolic(value: impl Into<String>) -> Result<Self, EffectValidationError> {
        let value = value.into();
        validate_text("symbolic effect argument", &value)?;
        Ok(Self::Symbolic(value))
    }

    /// Construct an extension-defined argument.
    pub fn extension(
        namespace: impl Into<String>,
        key: impl Into<String>,
    ) -> Result<Self, EffectValidationError> {
        let namespace = namespace.into();
        let key = key.into();

        validate_text("effect argument extension namespace", &namespace)?;
        validate_text("effect argument extension key", &key)?;

        Ok(Self::Extension { namespace, key })
    }

    /// Validate textual payloads, including those deserialized without
    /// invoking checked constructors.
    pub fn validate(&self) -> Result<(), EffectValidationError> {
        match self {
            Self::Type(_) | Self::Symbol(_) | Self::Value(_) => Ok(()),
            Self::Constant(key) => key.validate(),
            Self::Symbolic(value) => {
                validate_text("symbolic effect argument", value)
            }
            Self::Extension { namespace, key } => {
                validate_text("effect argument extension namespace", namespace)?;
                validate_text("effect argument extension key", key)
            }
        }
    }

    fn text_bytes(&self) -> Result<usize, EffectValidationError> {
        match self {
            Self::Type(_) | Self::Symbol(_) | Self::Value(_) => Ok(0),
            Self::Constant(key) => Ok(key.as_str().len()),
            Self::Symbolic(value) => Ok(value.len()),
            Self::Extension { namespace, key } => {
                add_count(namespace.len(), key.len())
            }
        }
    }
}

// =============================================================================
// Effect instances
// =============================================================================

/// One effect kind instantiated with an ordered list of resolved arguments.
///
/// The same effect with different arguments is represented as a distinct
/// instance. Whether those instances may coexist in a particular effect row
/// is a semantic question, not decided here.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct EffectInstance {
    effect: EffectId,
    arguments: Vec<EffectArgument>,
}

impl EffectInstance {
    /// Construct a parameterized effect instance.
    pub fn new(
        effect: EffectId,
        arguments: Vec<EffectArgument>,
    ) -> Result<Self, EffectValidationError> {
        effect.validate()?;

        for argument in &arguments {
            argument.validate()?;
        }

        Ok(Self { effect, arguments })
    }

    /// Construct an effect instance with no arguments.
    ///
    /// The identity must still be structurally validated before an
    /// externally supplied instance is trusted.
    #[must_use]
    pub fn simple(effect: EffectId) -> Self {
        Self {
            effect,
            arguments: Vec::new(),
        }
    }

    /// Return the effect identity.
    #[must_use]
    pub fn effect(&self) -> &EffectId {
        &self.effect
    }

    /// Return the ordered effect arguments.
    #[must_use]
    pub fn arguments(&self) -> &[EffectArgument] {
        &self.arguments
    }

    /// Return whether this instance has no arguments.
    #[must_use]
    pub fn is_unparameterized(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Validate the complete local representation.
    pub fn validate(&self) -> Result<(), EffectValidationError> {
        self.effect.validate()?;

        for argument in &self.arguments {
            argument.validate()?;
        }

        Ok(())
    }

    fn text_bytes(&self) -> Result<usize, EffectValidationError> {
        let mut total = add_count(
            self.effect.namespace().len(),
            self.effect.name().len(),
        )?;

        for argument in &self.arguments {
            total = add_count(total, argument.text_bytes()?)?;
        }

        Ok(total)
    }
}

// =============================================================================
// Effect-row variables
// =============================================================================

/// An open-world effect-row variable.
///
/// This identifies an unresolved row tail in polymorphic effect notation.
/// It does not resolve or infer the variable.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct EffectVariable(String);

impl EffectVariable {
    /// Construct a row variable from a canonical identity.
    pub fn new(value: impl Into<String>) -> Result<Self, EffectValidationError> {
        let value = value.into();
        validate_text("effect-row variable", &value)?;
        Ok(Self(value))
    }

    /// Return the variable identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Revalidate a value obtained through deserialization.
    pub fn validate(&self) -> Result<(), EffectValidationError> {
        validate_text("effect-row variable", &self.0)
    }
}

impl fmt::Display for EffectVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Effect rows
// =============================================================================

/// A normalized effect row.
///
/// `effects` is a deterministic set: inserting the exact same instance twice
/// does not change the row. Different parameterizations remain distinct.
///
/// `tail` is `None` for a closed row. `Some(variable)` represents an open row
/// whose remaining effects are described by that variable.
///
/// This model permits one open tail. Combining rows with two distinct open
/// tails returns an explicit error rather than dropping either variable.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectRow {
    effects: BTreeSet<EffectInstance>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    tail: Option<EffectVariable>,
}

impl EffectRow {
    /// Construct a closed, empty effect row.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Construct a closed row from effect instances.
    ///
    /// Exact duplicate instances are normalized by set insertion. This does
    /// not merge distinct parameterizations or declare them compatible.
    pub fn closed(
        effects: impl IntoIterator<Item = EffectInstance>,
    ) -> Result<Self, EffectValidationError> {
        let mut row = Self::empty();

        for effect in effects {
            effect.validate()?;
            row.effects.insert(effect);
        }

        Ok(row)
    }

    /// Construct an open row with a row variable.
    pub fn open(
        effects: impl IntoIterator<Item = EffectInstance>,
        tail: EffectVariable,
    ) -> Result<Self, EffectValidationError> {
        tail.validate()?;
        let mut row = Self::closed(effects)?;
        row.tail = Some(tail);
        Ok(row)
    }

    /// Return the normalized effect set.
    #[must_use]
    pub fn effects(&self) -> &BTreeSet<EffectInstance> {
        &self.effects
    }

    /// Return the open-row tail, if present.
    #[must_use]
    pub fn tail(&self) -> Option<&EffectVariable> {
        self.tail.as_ref()
    }

    /// Return whether this row has no explicit effects and is closed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty() && self.tail.is_none()
    }

    /// Return whether the row is closed.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.tail.is_none()
    }

    /// Return the number of distinct explicit effect instances.
    #[must_use]
    pub fn len(&self) -> usize {
        self.effects.len()
    }

    /// Return whether the explicit effect set is empty.
    ///
    /// An empty explicit set may still have an open tail; use `is_empty()` to
    /// determine whether the whole row is empty.
    #[must_use]
    pub fn explicit_effects_are_empty(&self) -> bool {
        self.effects.is_empty()
    }

    /// Add one explicit effect instance.
    ///
    /// Exact duplicates are ignored by set semantics. The row tail is
    /// preserved.
    pub fn insert(
        &mut self,
        effect: EffectInstance,
    ) -> Result<bool, EffectValidationError> {
        effect.validate()?;
        Ok(self.effects.insert(effect))
    }

    /// Remove an exact effect instance.
    ///
    /// Returns whether the instance was present. No other parameterization
    /// is removed, and the row tail is preserved.
    pub fn remove(&mut self, effect: &EffectInstance) -> bool {
        self.effects.remove(effect)
    }

    /// Combine two rows while preserving their explicit effects and tails.
    ///
    /// - A closed row combined with an open row keeps the open tail.
    /// - Two rows with the same tail keep that tail.
    /// - Two rows with distinct tails fail explicitly.
    ///
    /// This is structural set union, not a substitute for language-specific
    /// effect subtyping, authorization, or row unification.
    pub fn union(&self, other: &Self) -> Result<Self, EffectValidationError> {
        let tail = match (&self.tail, &other.tail) {
            (None, None) => None,
            (Some(left), None) => Some(left.clone()),
            (None, Some(right)) => Some(right.clone()),
            (Some(left), Some(right)) if left == right => Some(left.clone()),
            (Some(left), Some(right)) => {
                return Err(EffectValidationError::ConflictingRowTails {
                    left: left.as_str().to_owned(),
                    right: right.as_str().to_owned(),
                });
            }
        };

        let effects = self
            .effects
            .union(&other.effects)
            .cloned()
            .collect::<BTreeSet<_>>();

        let row = Self { effects, tail };
        row.validate()?;
        Ok(row)
    }

    /// Validate a row without local operational limits.
    pub fn validate(&self) -> Result<(), EffectValidationError> {
        self.validate_with(&EffectValidationBudget::unlimited())
    }

    /// Validate this row and its arguments under explicit operational budgets.
    ///
    /// Revalidation is required for deserialized data because Serde can
    /// construct values without calling the checked constructors.
    pub fn validate_with(
        &self,
        budget: &EffectValidationBudget,
    ) -> Result<(), EffectValidationError> {
        if let Some(tail) = &self.tail {
            tail.validate()?;
        }

        let effect_count = self.effects.len();
        check_budget(
            effect_count,
            budget.max_effects,
            |actual, limit| EffectValidationError::EffectBudgetExceeded {
                actual,
                limit,
            },
        )?;

        let mut argument_count = 0usize;
        let mut text_bytes = 0usize;

        if let Some(tail) = &self.tail {
            text_bytes = add_count(text_bytes, tail.as_str().len())?;
        }

        for effect in &self.effects {
            effect.validate()?;

            argument_count = add_count(argument_count, effect.arguments().len())?;
            text_bytes = add_count(text_bytes, effect.text_bytes()?)?;

            check_budget(
                argument_count,
                budget.max_arguments,
                |actual, limit| EffectValidationError::ArgumentBudgetExceeded {
                    actual,
                    limit,
                },
            )?;

            check_budget(
                text_bytes,
                budget.max_text_bytes,
                |actual, limit| EffectValidationError::TextBudgetExceeded {
                    actual,
                    limit,
                },
            )?;
        }

        // Check even an empty row's tail text against the configured budget.
        check_budget(
            text_bytes,
            budget.max_text_bytes,
            |actual, limit| EffectValidationError::TextBudgetExceeded {
                actual,
                limit,
            },
        )?;

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn effect(namespace: &str, name: &str) -> EffectInstance {
        EffectInstance::simple(
            EffectId::new(namespace, name).expect("valid test effect identity"),
        )
    }

    #[test]
    fn accepts_new_effect_names_without_changing_an_enum() {
        let first = effect("quantum", "measurement");
        let future = effect("future.domain", "new_interaction");

        let row = EffectRow::closed([first, future]).expect("valid row");

        assert_eq!(row.len(), 2);
        assert!(row.is_closed());
    }

    #[test]
    fn rejects_empty_or_invalid_identity_components() {
        assert!(matches!(
            EffectId::new("", "read"),
            Err(EffectValidationError::EmptyText { .. })
        ));

        assert!(matches!(
            EffectId::new("core", "read write"),
            Err(EffectValidationError::InvalidText { .. })
        ));

        assert!(matches!(
            EffectId::new("core\0", "read"),
            Err(EffectValidationError::NulCharacter { .. })
        ));
    }

    #[test]
    fn rows_are_deterministic_and_deduplicate_exact_instances() {
        let a = effect("core", "read");
        let b = effect("network", "send");

        let left = EffectRow::closed([a.clone(), b.clone(), a.clone()])
            .expect("valid row");
        let right = EffectRow::closed([b, a]).expect("valid row");

        assert_eq!(left, right);
        assert_eq!(left.len(), 2);
    }

    #[test]
    fn parameterized_effects_preserve_argument_identity() {
        let effect_id = EffectId::new("domain", "operate").expect("valid ID");
        let a = EffectInstance::new(
            effect_id.clone(),
            vec![EffectArgument::symbolic("dimension.alpha").expect("valid key")],
        )
        .expect("valid instance");
        let b = EffectInstance::new(
            effect_id,
            vec![EffectArgument::symbolic("dimension.beta").expect("valid key")],
        )
        .expect("valid instance");

        let row = EffectRow::closed([a, b]).expect("valid row");

        assert_eq!(row.len(), 2);
    }

    #[test]
    fn union_preserves_one_open_tail() {
        let tail = EffectVariable::new("row.r").expect("valid tail");
        let open = EffectRow::open([effect("core", "read")], tail.clone())
            .expect("valid open row");
        let closed = EffectRow::closed([effect("core", "write")])
            .expect("valid closed row");

        let combined = open.union(&closed).expect("compatible tails");

        assert_eq!(combined.tail(), Some(&tail));
        assert_eq!(combined.len(), 2);
    }

    #[test]
    fn union_rejects_distinct_open_tails() {
        let left = EffectRow::open(
            [],
            EffectVariable::new("row.left").expect("valid tail"),
        )
        .expect("valid row");
        let right = EffectRow::open(
            [],
            EffectVariable::new("row.right").expect("valid tail"),
        )
        .expect("valid row");

        assert!(matches!(
            left.union(&right),
            Err(EffectValidationError::ConflictingRowTails { .. })
        ));
    }

    #[test]
    fn validation_budget_fails_explicitly() {
        let row = EffectRow::closed([
            effect("core", "read"),
            effect("core", "write"),
        ])
        .expect("valid row");

        let budget = EffectValidationBudget::new(Some(1), None, None);

        assert!(matches!(
            row.validate_with(&budget),
            Err(EffectValidationError::EffectBudgetExceeded {
                actual: 2,
                limit: 1
            })
        ));
    }

    #[test]
    fn text_budget_counts_utf8_bytes() {
        let row = EffectRow::closed([effect("core", "read")])
            .expect("valid row");
        let budget = EffectValidationBudget::new(None, None, Some(1));

        assert!(matches!(
            row.validate_with(&budget),
            Err(EffectValidationError::TextBudgetExceeded { .. })
        ));
    }

    #[test]
    fn revalidation_catches_invalid_deserialized_payloads() {
        let argument = EffectArgument::Symbolic("bad value".to_owned());

        assert!(matches!(
            argument.validate(),
            Err(EffectValidationError::InvalidText { .. })
        ));
    }

    #[test]
    fn symbolic_and_extension_arguments_validate() {
        let symbolic = EffectArgument::symbolic("shape.dimension").unwrap();
        symbolic.validate().unwrap();

        let extension =
            EffectArgument::extension("future.extension", "opaque-key").unwrap();
        extension.validate().unwrap();

        assert!(EffectArgument::symbolic("").is_err());
        assert!(EffectArgument::extension("future.extension", "").is_err());
    }

    #[test]
    fn serde_round_trip_preserves_a_row() {
        let row = EffectRow::open(
            [effect("quantum", "measurement"), effect("core", "read")],
            EffectVariable::new("row.e").expect("valid tail"),
        )
        .expect("valid row");

        let encoded = serde_json::to_string(&row).expect("serialize row");
        let decoded: EffectRow =
            serde_json::from_str(&encoded).expect("deserialize row");

        decoded.validate().expect("revalidate decoded row");
        assert_eq!(decoded, row);
    }
}
