
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani semantic constant evaluation
//!
//! Path: `src/frontend/semantic/types/constants.rs`
//! Edition: Rust 2021; minimum repository toolchain: Rust 1.97.
//!
//! ## File contract
//!
//! PURPOSE
//!
//! Evaluate compile-time expressions without silently narrowing values to
//! the host machine's integer widths.
//!
//! AUTHORITY
//!
//! - `grammar/spec/type-system.md`
//! - `grammar/lexer/numeric-literals.md`
//! - `grammar/expressions/literals.g4`
//! - `src/frontend/ast/node/expressions/literal.rs`
//! - `src/frontend/semantic/types/ty.rs`
//!
//! OWNS
//!
//! - Exact arbitrary-magnitude signed integer values.
//! - Exact decimal literal preservation.
//! - Constant-value and evaluation-error representations.
//! - Checked integer arithmetic.
//! - Configurable work accounting.
//! - An adapter contract for expression-graph evaluation.
//!
//! DOES NOT OWN
//!
//! - Parsing or AST construction.
//! - Name or import resolution.
//! - Type inference, unification, or coercion.
//! - Floating-point rounding policy.
//! - Evaluation of arbitrary runtime expressions.
//! - Domain-specific quantum, HDL, tensor, or hardware operations.
//! - Resource allocation, target feasibility, or IR lowering.
//!
//! INTEGRATION
//!
//! `types/inference.rs` and `types/constraints.rs` may request constant
//! evaluation through the public value and evaluator APIs.
//! `types/ty.rs` remains the authority for semantic types and type-level
//! cardinalities. The AST adapter must supply canonical AST node identities,
//! literal spelling, and expression dependencies.
//! `model/facts.rs` should retain the result and its validity/provenance.
//! `diagnostics/` should translate `ConstantError` into canonical diagnostics.
//!
//! SCALABILITY
//!
//! There is no language-defined integer-width ceiling. Integers use dynamically
//! sized base-10^9 limbs. Actual operations remain bounded by available memory,
//! address space, and the explicitly selected evaluation budget.
//!
//! This does not promise infinite memory or infinite execution time.
//!
//! SAFETY AND DETERMINISM
//!
//! This module uses safe Rust only. It does not access global mutable state,
//! hardware, the filesystem, the network, or the environment. Arithmetic
//! checks overflow in intermediate machine-word operations where required.
//!
//! STATUS
//!
//! This is a standalone semantic implementation and integration contract.
//! It becomes production-integrated only after the canonical AST adapter,
//! diagnostics, module exports, and repository conformance tests are wired
//! and verified.

use core::cmp::Ordering;
use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

/// Limb base: each limb stores a value in `[0, LIMB_BASE)`.
///
/// This is an internal representation radix, not a language integer limit.
const LIMB_BASE: u64 = 1_000_000_000;

/// Exact, arbitrary-magnitude signed integer.
///
/// The magnitude is stored least-significant limb first. Zero is always
/// represented with an empty magnitude and a non-negative sign.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BigInt {
    negative: bool,
    limbs: Vec<u32>,
}

impl BigInt {
    /// Construct zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            negative: false,
            limbs: Vec::new(),
        }
    }

    /// Construct one.
    #[must_use]
    pub fn one() -> Self {
        Self {
            negative: false,
            limbs: vec![1],
        }
    }

    /// Construct from an `i128` without overflowing on `i128::MIN`.
    #[must_use]
    pub fn from_i128(value: i128) -> Self {
        let negative = value.is_negative();
        let mut magnitude = value.unsigned_abs();
        let mut limbs = Vec::new();

        while magnitude != 0 {
            limbs.push((magnitude % LIMB_BASE) as u32);
            magnitude /= LIMB_BASE;
        }

        Self::normalized(negative, limbs)
    }

    /// Parse an integer literal using an explicit radix.
    ///
    /// Accepted separators are underscores between digits. A matching
    /// `0b`, `0o`, or `0x` prefix is optional for the corresponding radix.
    /// A leading sign is not accepted here: signs are expression operators
    /// and must be applied by the expression evaluator.
    ///
    /// `radix` must be in `2..=36`.
    pub fn parse_radix(source: &str, radix: u32) -> Result<Self, ConstantError> {
        if !(2..=36).contains(&radix) {
            return Err(ConstantError::InvalidRadix(radix));
        }

        let source = source.trim();
        if source.is_empty() {
            return Err(ConstantError::InvalidIntegerLiteral(
                "empty integer literal".into(),
            ));
        }

        if source.starts_with('+') || source.starts_with('-') {
            return Err(ConstantError::InvalidIntegerLiteral(
                "integer signs must be parsed as expression operators".into(),
            ));
        }

        let digits = match (radix, source.get(..2)) {
            (2, Some(prefix)) if prefix.eq_ignore_ascii_case("0b") => &source[2..],
            (8, Some(prefix)) if prefix.eq_ignore_ascii_case("0o") => &source[2..],
            (16, Some(prefix)) if prefix.eq_ignore_ascii_case("0x") => &source[2..],
            _ => source,
        };

        if digits.is_empty() {
            return Err(ConstantError::InvalidIntegerLiteral(
                "integer literal has no digits".into(),
            ));
        }

        let mut result = Self::zero();
        let mut previous_was_digit = false;
        let mut saw_digit = false;

        for character in digits.chars() {
            if character == '_' {
                if !previous_was_digit {
                    return Err(ConstantError::InvalidIntegerLiteral(
                        "misplaced digit separator".into(),
                    ));
                }
                previous_was_digit = false;
                continue;
            }

            let digit = character.to_digit(radix).ok_or_else(|| {
                ConstantError::InvalidIntegerLiteral(format!(
                    "digit {character:?} is invalid in radix {radix}"
                ))
            })?;

            result.mul_small_assign(radix);
            result.add_small_assign(digit);
            previous_was_digit = true;
            saw_digit = true;
        }

        if !saw_digit || !previous_was_digit {
            return Err(ConstantError::InvalidIntegerLiteral(
                "integer literal must end with a digit".into(),
            ));
        }

        Ok(result)
    }

    /// Return whether the integer is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    /// Return whether the integer is negative.
    #[must_use]
    pub fn is_negative(&self) -> bool {
        self.negative
    }

    /// Return the magnitude's number of base-10^9 limbs.
    ///
    /// This is a representation metric, not the integer's decimal digit count.
    #[must_use]
    pub fn limb_count(&self) -> usize {
        self.limbs.len()
    }

    /// Return the negated integer.
    #[must_use]
    pub fn negated(&self) -> Self {
        if self.is_zero() {
            return self.clone();
        }

        let mut result = self.clone();
        result.negative = !result.negative;
        result
    }

    /// Add two integers.
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        if self.negative == other.negative {
            return Self::normalized(
                self.negative,
                Self::add_magnitudes(&self.limbs, &other.limbs),
            );
        }

        match Self::compare_magnitudes(&self.limbs, &other.limbs) {
            Ordering::Greater => Self::normalized(
                self.negative,
                Self::subtract_magnitudes(&self.limbs, &other.limbs),
            ),
            Ordering::Less => Self::normalized(
                other.negative,
                Self::subtract_magnitudes(&other.limbs, &self.limbs),
            ),
            Ordering::Equal => Self::zero(),
        }
    }

    /// Subtract two integers.
    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.negated())
    }

    /// Multiply two integers.
    #[must_use]
    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }

        let mut result = vec![0_u32; self.limbs.len() + other.limbs.len()];

        for (i, left) in self.limbs.iter().copied().enumerate() {
            let mut carry = 0_u64;

            for (j, right) in other.limbs.iter().copied().enumerate() {
                let index = i + j;
                let current = u64::from(result[index])
                    + u64::from(left) * u64::from(right)
                    + carry;

                result[index] = (current % LIMB_BASE) as u32;
                carry = current / LIMB_BASE;
            }

            let mut index = i + other.limbs.len();

            while carry != 0 {
                if index == result.len() {
                    result.push(0);
                }

                let current = u64::from(result[index]) + carry;
                result[index] = (current % LIMB_BASE) as u32;
                carry = current / LIMB_BASE;
                index += 1;
            }
        }

        Self::normalized(self.negative ^ other.negative, result)
    }

    /// Divide two integers, truncating the quotient toward zero.
    ///
    /// The remainder has the dividend's sign, matching truncating integer
    /// division. Division by zero is reported explicitly.
    pub fn div_rem(&self, divisor: &Self) -> Result<(Self, Self), ConstantError> {
        if divisor.is_zero() {
            return Err(ConstantError::DivisionByZero);
        }

        if self.is_zero() {
            return Ok((Self::zero(), Self::zero()));
        }

        let dividend_magnitude =
            Self::normalized(false, self.limbs.clone());
        let divisor_magnitude =
            Self::normalized(false, divisor.limbs.clone());

        if Self::compare_magnitudes(
            &dividend_magnitude.limbs,
            &divisor_magnitude.limbs,
        ) == Ordering::Less
        {
            return Ok((
                Self::zero(),
                self.clone(),
            ));
        }

        let (mut quotient, mut remainder) =
            Self::divide_magnitudes(&dividend_magnitude, &divisor_magnitude)?;

        quotient.negative = self.negative ^ divisor.negative;
        remainder.negative = self.negative;

        Ok((
            Self::normalized(quotient.negative, quotient.limbs),
            Self::normalized(remainder.negative, remainder.limbs),
        ))
    }

    /// Convert to `i128` only when exactly representable.
    #[must_use]
    pub fn to_i128(&self) -> Option<i128> {
        let mut magnitude = 0_u128;

        for limb in self.limbs.iter().rev().copied() {
            magnitude = magnitude
                .checked_mul(u128::from(LIMB_BASE))?
                .checked_add(u128::from(limb))?;
        }

        if self.negative {
            let min_magnitude = (i128::MAX as u128) + 1;
            if magnitude > min_magnitude {
                return None;
            }
            if magnitude == min_magnitude {
                Some(i128::MIN)
            } else {
                i128::try_from(magnitude).ok().map(|n| -n)
            }
        } else {
            i128::try_from(magnitude).ok()
        }
    }

    /// Convert to `u128` only when exactly representable and non-negative.
    #[must_use]
    pub fn to_u128(&self) -> Option<u128> {
        if self.negative {
            return None;
        }

        let mut value = 0_u128;

        for limb in self.limbs.iter().rev().copied() {
            value = value
                .checked_mul(u128::from(LIMB_BASE))?
                .checked_add(u128::from(limb))?;
        }

        Some(value)
    }

    fn normalized(negative: bool, mut limbs: Vec<u32>) -> Self {
        while limbs.last() == Some(&0) {
            limbs.pop();
        }

        Self {
            negative: negative && !limbs.is_empty(),
            limbs,
        }
    }

    fn mul_small_assign(&mut self, multiplier: u32) {
        if multiplier == 0 || self.is_zero() {
            self.limbs.clear();
            self.negative = false;
            return;
        }

        let mut carry = 0_u64;

        for limb in &mut self.limbs {
            let current =
                u64::from(*limb) * u64::from(multiplier) + carry;
            *limb = (current % LIMB_BASE) as u32;
            carry = current / LIMB_BASE;
        }

        while carry != 0 {
            self.limbs.push((carry % LIMB_BASE) as u32);
            carry /= LIMB_BASE;
        }
    }

    fn add_small_assign(&mut self, addend: u32) {
        let mut carry = u64::from(addend);
        let mut index = 0;

        while carry != 0 {
            if index == self.limbs.len() {
                self.limbs.push(0);
            }

            let current = u64::from(self.limbs[index]) + carry;
            self.limbs[index] = (current % LIMB_BASE) as u32;
            carry = current / LIMB_BASE;
            index += 1;
        }
    }

    fn add_magnitudes(left: &[u32], right: &[u32]) -> Vec<u32> {
        let count = left.len().max(right.len());
        let mut output = Vec::with_capacity(count.saturating_add(1));
        let mut carry = 0_u64;

        for index in 0..count {
            let a = left.get(index).copied().unwrap_or(0);
            let b = right.get(index).copied().unwrap_or(0);
            let current = u64::from(a) + u64::from(b) + carry;

            output.push((current % LIMB_BASE) as u32);
            carry = current / LIMB_BASE;
        }

        if carry != 0 {
            output.push(carry as u32);
        }

        output
    }

    /// Subtract magnitudes, assuming `left >= right`.
    fn subtract_magnitudes(left: &[u32], right: &[u32]) -> Vec<u32> {
        let mut output = Vec::with_capacity(left.len());
        let mut borrow = 0_i64;

        for (index, left_limb) in left.iter().copied().enumerate() {
            let right_limb = i64::from(right.get(index).copied().unwrap_or(0));
            let mut current = i64::from(left_limb) - right_limb - borrow;

            if current < 0 {
                current += LIMB_BASE as i64;
                borrow = 1;
            } else {
                borrow = 0;
            }

            output.push(current as u32);
        }

        output
    }

    fn compare_magnitudes(left: &[u32], right: &[u32]) -> Ordering {
        match left.len().cmp(&right.len()) {
            Ordering::Equal => {
                for (a, b) in left.iter().rev().zip(right.iter().rev()) {
                    match a.cmp(b) {
                        Ordering::Equal => {}
                        unequal => return unequal,
                    }
                }
                Ordering::Equal
            }
            unequal => unequal,
        }
    }

    fn divide_magnitudes(
        dividend: &Self,
        divisor: &Self,
    ) -> Result<(Self, Self), ConstantError> {
        if divisor.is_zero() {
            return Err(ConstantError::DivisionByZero);
        }

        let mut quotient_be = Vec::with_capacity(dividend.limbs.len());
        let mut remainder = Self::zero();

        for limb in dividend.limbs.iter().rev().copied() {
            remainder.limbs.insert(0, limb);
            remainder = Self::normalized(false, remainder.limbs);

            let mut low = 0_u64;
            let mut high = LIMB_BASE - 1;
            let mut best = 0_u32;

            while low <= high {
                let middle = low + (high - low) / 2;
                let product = Self::multiply_magnitude_small(divisor, middle as u32);

                match Self::compare_magnitudes(
                    &product.limbs,
                    &remainder.limbs,
                ) {
                    Ordering::Greater => {
                        if middle == 0 {
                            break;
                        }
                        high = middle - 1;
                    }
                    _ => {
                        best = middle as u32;
                        low = middle + 1;
                    }
                }
            }

            let product = Self::multiply_magnitude_small(divisor, best);
            remainder = Self::normalized(
                false,
                Self::subtract_magnitudes(&remainder.limbs, &product.limbs),
            );
            quotient_be.push(best);
        }

        quotient_be.reverse();

        Ok((
            Self::normalized(false, quotient_be),
            Self::normalized(false, remainder.limbs),
        ))
    }

    fn multiply_magnitude_small(value: &Self, multiplier: u32) -> Self {
        let mut result = value.clone();
        result.negative = false;
        result.mul_small_assign(multiplier);
        result
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.negative, other.negative) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => {
                Self::compare_magnitudes(&self.limbs, &other.limbs)
            }
            (true, true) => {
                Self::compare_magnitudes(&other.limbs, &self.limbs)
            }
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            formatter.write_str("-")?;
        }

        let mut limbs = self.limbs.iter().rev();

        match limbs.next() {
            Some(first) => write!(formatter, "{first}")?,
            None => return formatter.write_str("0"),
        }

        for limb in limbs {
            write!(formatter, "{limb:09}")?;
        }

        Ok(())
    }
}

/// Exact decimal spelling.
///
/// This intentionally does not convert a decimal literal to `f32` or `f64`.
/// The spelling is retained until a language-defined numeric policy chooses
/// an exact decimal, rational, or floating-point interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExactDecimal {
    spelling: String,
}

impl ExactDecimal {
    /// Construct after validating a conservative decimal syntax.
    ///
    /// This checks decimal/exponent structure; the canonical lexer remains
    /// authoritative for source syntax and token boundaries.
    pub fn parse(spelling: impl Into<String>) -> Result<Self, ConstantError> {
        let spelling = spelling.into();
        let trimmed = spelling.trim();

        if trimmed.is_empty() {
            return Err(ConstantError::InvalidDecimalLiteral(
                "empty decimal literal".into(),
            ));
        }

        let unsigned = trimmed
            .strip_prefix('+')
            .or_else(|| trimmed.strip_prefix('-'))
            .unwrap_or(trimmed);

        let (mantissa, exponent) = match unsigned.find(['e', 'E']) {
            Some(index) => {
                let (left, right) = unsigned.split_at(index);
                let exponent = right.get(1..).unwrap_or_default();

                if exponent.is_empty()
                    || exponent.parse::<i64>().is_err()
                {
                    return Err(ConstantError::InvalidDecimalLiteral(
                        "invalid decimal exponent".into(),
                    ));
                }

                (left, Some(exponent))
            }
            None => (unsigned, None),
        };

        let _ = exponent;

        let mut dots = 0_usize;
        let mut digits = 0_usize;
        let mut previous_digit = false;

        for character in mantissa.chars() {
            match character {
                '0'..='9' => {
                    digits += 1;
                    previous_digit = true;
                }
                '_' => {
                    if !previous_digit {
                        return Err(ConstantError::InvalidDecimalLiteral(
                            "misplaced digit separator".into(),
                        ));
                    }
                    previous_digit = false;
                }
                '.' => {
                    dots += 1;
                    previous_digit = false;
                    if dots > 1 {
                        return Err(ConstantError::InvalidDecimalLiteral(
                            "multiple decimal points".into(),
                        ));
                    }
                }
                _ => {
                    return Err(ConstantError::InvalidDecimalLiteral(
                        "invalid character in decimal mantissa".into(),
                    ));
                }
            }
        }

        if digits == 0 || !previous_digit {
            return Err(ConstantError::InvalidDecimalLiteral(
                "decimal mantissa must contain digits and end with a digit"
                    .into(),
            ));
        }

        Ok(Self {
            spelling: trimmed.to_owned(),
        })
    }

    /// Return the preserved spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.spelling
    }
}

/// A constant value known to semantic analysis.
///
/// Domain-specific values should be introduced through a validated extension
/// evaluator rather than by adding a permanent variant for every domain.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConstantValue {
    Integer(BigInt),
    Decimal(ExactDecimal),
    Boolean(bool),
    String(String),
    Character(char),
    Unit,
    Null,
    Tuple(Vec<ConstantValue>),
    Array(Vec<ConstantValue>),
}

impl ConstantValue {
    /// Return the integer value when this is an integer constant.
    #[must_use]
    pub fn as_integer(&self) -> Option<&BigInt> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    /// Return the boolean value when this is a boolean constant.
    #[must_use]
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }
}

/// Errors from constant parsing or evaluation.
///
/// These are semantic failures, not panics. The diagnostics subsystem should
/// map these structured cases to stable diagnostic codes and source labels.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConstantError {
    InvalidRadix(u32),
    InvalidIntegerLiteral(String),
    InvalidDecimalLiteral(String),
    UnsupportedLiteral(String),
    UnsupportedOperation(String),
    TypeMismatch {
        operation: String,
        expected: String,
        actual: String,
    },
    DivisionByZero,
    NonConstantExpression,
    UnknownConstant(String),
    CyclicConstantDependency(Vec<String>),
    WorkBudgetExceeded {
        budget: usize,
        consumed: usize,
    },
    ValueBudgetExceeded {
        budget: usize,
        observed: usize,
    },
    ArithmeticOverflow,
    AdapterFailure(String),
}

impl fmt::Display for ConstantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRadix(radix) => {
                write!(formatter, "invalid integer radix {radix}")
            }
            Self::InvalidIntegerLiteral(message) => {
                write!(formatter, "invalid integer literal: {message}")
            }
            Self::InvalidDecimalLiteral(message) => {
                write!(formatter, "invalid decimal literal: {message}")
            }
            Self::UnsupportedLiteral(name) => {
                write!(formatter, "unsupported constant literal: {name}")
            }
            Self::UnsupportedOperation(name) => {
                write!(formatter, "unsupported constant operation: {name}")
            }
            Self::TypeMismatch {
                operation,
                expected,
                actual,
            } => write!(
                formatter,
                "constant operation {operation} expected {expected}, got {actual}"
            ),
            Self::DivisionByZero => {
                formatter.write_str("division by zero in constant expression")
            }
            Self::NonConstantExpression => {
                formatter.write_str("expression is not a compile-time constant")
            }
            Self::UnknownConstant(name) => {
                write!(formatter, "unknown constant {name}")
            }
            Self::CyclicConstantDependency(path) => {
                write!(formatter, "cyclic constant dependency: {}", path.join(" -> "))
            }
            Self::WorkBudgetExceeded { budget, consumed } => write!(
                formatter,
                "constant evaluation work budget {budget} exceeded at {consumed}"
            ),
            Self::ValueBudgetExceeded { budget, observed } => write!(
                formatter,
                "constant value budget {budget} exceeded by value size {observed}"
            ),
            Self::ArithmeticOverflow => {
                formatter.write_str("internal checked arithmetic overflow")
            }
            Self::AdapterFailure(message) => {
                write!(formatter, "constant expression adapter failed: {message}")
            }
        }
    }
}

impl Error for ConstantError {}

/// Evaluation budget.
///
/// `None` means no explicit policy budget is applied for that dimension.
/// It does not mean that memory, time, or address space is infinite.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConstantEvaluationConfig {
    /// Maximum number of evaluator work steps.
    pub max_steps: Option<usize>,

    /// Maximum total integer limbs or collection elements retained in one
    /// produced value. This is an operational budget, not a language limit.
    pub max_value_units: Option<usize>,
}

impl ConstantEvaluationConfig {
    /// No explicit policy limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_steps: None,
            max_value_units: None,
        }
    }

    /// Construct a configuration with explicit budgets.
    #[must_use]
    pub const fn bounded(
        max_steps: usize,
        max_value_units: usize,
    ) -> Self {
        Self {
            max_steps: Some(max_steps),
            max_value_units: Some(max_value_units),
        }
    }
}

/// Source expression identity.
///
/// Implement this for the canonical AST's `NodeId` in the adapter layer.
pub trait ConstantExpressionId:
    Copy + Ord + fmt::Debug
{
}

impl<T> ConstantExpressionId for T where T: Copy + Ord + fmt::Debug {}

/// The operations that a canonical AST adapter exposes to the evaluator.
///
/// This is an adapter contract, not a replacement AST. `Id` must identify
/// canonical AST nodes. Child edges are references by identity, consistent
/// with Zamani's graph-shaped AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstantExpression<Id> {
    Literal(ConstantValue),
    ConstantName(String),
    Negate(Id),
    LogicalNot(Id),
    Add(Id, Id),
    Subtract(Id, Id),
    Multiply(Id, Id),
    Divide(Id, Id),
    Remainder(Id, Id),
    Equal(Id, Id),
    LessThan(Id, Id),
    LessThanOrEqual(Id, Id),
    LogicalAnd(Id, Id),
    LogicalOr(Id, Id),
    Tuple(Vec<Id>),
    Array(Vec<Id>),
    /// Explicitly marks an expression that is not constant-evaluable.
    NonConstant,
}

/// Adapter implemented by the AST/semantic integration layer.
///
/// Name lookup must use the canonical symbol table. This trait must not be
/// implemented by a second independent symbol table in this file.
pub trait ConstantExpressionHost {
    type Id: ConstantExpressionId;

    /// Retrieve the semantic view for one canonical AST expression.
    fn expression(
        &self,
        id: Self::Id,
    ) -> Result<ConstantExpression<Self::Id>, ConstantError>;

    /// Resolve a constant name to its canonical expression identity.
    fn resolve_constant(
        &self,
        name: &str,
    ) -> Result<Self::Id, ConstantError>;
}

/// Per-invocation constant evaluator.
///
/// An evaluator owns its memoization and dependency stack. It has no global
/// state and must not be shared mutably between concurrent analysis sessions.
pub struct ConstantEvaluator<'a, H: ConstantExpressionHost> {
    host: &'a H,
    config: ConstantEvaluationConfig,
    cache: BTreeMap<H::Id, ConstantValue>,
    active: BTreeSet<H::Id>,
    dependency_names: Vec<String>,
    steps: usize,
}

impl<'a, H: ConstantExpressionHost> ConstantEvaluator<'a, H> {
    /// Create an evaluator for one analysis invocation.
    #[must_use]
    pub fn new(host: &'a H, config: ConstantEvaluationConfig) -> Self {
        Self {
            host,
            config,
            cache: BTreeMap::new(),
            active: BTreeSet::new(),
            dependency_names: Vec::new(),
            steps: 0,
        }
    }

    /// Evaluate a canonical AST expression by identity.
    pub fn evaluate(
        &mut self,
        id: H::Id,
    ) -> Result<ConstantValue, ConstantError> {
        self.evaluate_inner(id)
    }

    /// Return the number of evaluation steps consumed.
    #[must_use]
    pub fn steps_consumed(&self) -> usize {
        self.steps
    }

    /// Clear per-invocation memoization and dependency state.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.active.clear();
        self.dependency_names.clear();
        self.steps = 0;
    }

    fn consume_step(&mut self) -> Result<(), ConstantError> {
        self.steps = self
            .steps
            .checked_add(1)
            .ok_or(ConstantError::ArithmeticOverflow)?;

        if let Some(budget) = self.config.max_steps {
            if self.steps > budget {
                return Err(ConstantError::WorkBudgetExceeded {
                    budget,
                    consumed: self.steps,
                });
            }
        }

        Ok(())
    }

    fn evaluate_inner(
        &mut self,
        id: H::Id,
    ) -> Result<ConstantValue, ConstantError> {
        if let Some(value) = self.cache.get(&id) {
            return Ok(value.clone());
        }

        self.consume_step()?;

        if !self.active.insert(id) {
            return Err(ConstantError::CyclicConstantDependency(
                self.dependency_names.clone(),
            ));
        }

        let expression = self.host.expression(id)?;
        let result = self.evaluate_expression(expression);

        self.active.remove(&id);

        let value = result?;
        self.check_value_budget(&value)?;
        self.cache.insert(id, value.clone());

        Ok(value)
    }

    fn evaluate_expression(
        &mut self,
        expression: ConstantExpression<H::Id>,
    ) -> Result<ConstantValue, ConstantError> {
        use ConstantExpression as E;

        match expression {
            E::Literal(value) => Ok(value),
            E::ConstantName(name) => {
                self.dependency_names.push(name.clone());
                let result = self.host.resolve_constant(&name)
                    .and_then(|id| self.evaluate_inner(id));
                self.dependency_names.pop();
                result
            }
            E::Negate(id) => match self.evaluate_inner(id)? {
                ConstantValue::Integer(value) => {
                    Ok(ConstantValue::Integer(value.negated()))
                }
                other => Err(type_mismatch(
                    "unary -",
                    "integer",
                    constant_kind(&other),
                )),
            },
            E::LogicalNot(id) => match self.evaluate_inner(id)? {
                ConstantValue::Boolean(value) => {
                    Ok(ConstantValue::Boolean(!value))
                }
                other => Err(type_mismatch(
                    "logical not",
                    "boolean",
                    constant_kind(&other),
                )),
            },
            E::Add(left, right) => {
                self.eval_integer_binary(left, right, "addition", |a, b| {
                    Ok(a.add(b))
                })
            }
            E::Subtract(left, right) => {
                self.eval_integer_binary(left, right, "subtraction", |a, b| {
                    Ok(a.sub(b))
                })
            }
            E::Multiply(left, right) => {
                self.eval_integer_binary(left, right, "multiplication", |a, b| {
                    Ok(a.mul(b))
                })
            }
            E::Divide(left, right) => {
                self.eval_integer_binary(left, right, "division", |a, b| {
                    a.div_rem(b).map(|pair| pair.0)
                })
            }
            E::Remainder(left, right) => {
                self.eval_integer_binary(left, right, "remainder", |a, b| {
                    a.div_rem(b).map(|pair| pair.1)
                })
            }
            E::Equal(left, right) => {
                let left = self.evaluate_inner(left)?;
                let right = self.evaluate_inner(right)?;
                Ok(ConstantValue::Boolean(left == right))
            }
            E::LessThan(left, right) => {
                self.eval_integer_compare(left, right, |order| order == Ordering::Less)
            }
            E::LessThanOrEqual(left, right) => {
                self.eval_integer_compare(left, right, |order| order != Ordering::Greater)
            }
            E::LogicalAnd(left, right) => {
                match self.evaluate_inner(left)? {
                    ConstantValue::Boolean(false) => Ok(ConstantValue::Boolean(false)),
                    ConstantValue::Boolean(true) => match self.evaluate_inner(right)? {
                        ConstantValue::Boolean(value) => Ok(ConstantValue::Boolean(value)),
                        other => Err(type_mismatch(
                            "logical and",
                            "boolean",
                            constant_kind(&other),
                        )),
                    },
                    other => Err(type_mismatch(
                        "logical and",
                        "boolean",
                        constant_kind(&other),
                    )),
                }
            }
            E::LogicalOr(left, right) => {
                match self.evaluate_inner(left)? {
                    ConstantValue::Boolean(true) => Ok(ConstantValue::Boolean(true)),
                    ConstantValue::Boolean(false) => match self.evaluate_inner(right)? {
                        ConstantValue::Boolean(value) => Ok(ConstantValue::Boolean(value)),
                        other => Err(type_mismatch(
                            "logical or",
                            "boolean",
                            constant_kind(&other),
                        )),
                    },
                    other => Err(type_mismatch(
                        "logical or",
                        "boolean",
                        constant_kind(&other),
                    )),
                }
            }
            E::Tuple(items) => {
                let mut values = Vec::with_capacity(items.len());
                for item in items {
                    values.push(self.evaluate_inner(item)?);
                }
                Ok(ConstantValue::Tuple(values))
            }
            E::Array(items) => {
                let mut values = Vec::with_capacity(items.len());
                for item in items {
                    values.push(self.evaluate_inner(item)?);
                }
                Ok(ConstantValue::Array(values))
            }
            E::NonConstant => Err(ConstantError::NonConstantExpression),
        }
    }

    fn eval_integer_binary(
        &mut self,
        left: H::Id,
        right: H::Id,
        operation: &str,
        calculate: impl FnOnce(&BigInt, &BigInt) -> Result<BigInt, ConstantError>,
    ) -> Result<ConstantValue, ConstantError> {
        let left = self.evaluate_inner(left)?;
        let right = self.evaluate_inner(right)?;

        match (left, right) {
            (ConstantValue::Integer(a), ConstantValue::Integer(b)) => {
                let result = calculate(&a, &b)?;
                let value = ConstantValue::Integer(result);
                self.check_value_budget(&value)?;
                Ok(value)
            }
            (left, right) => Err(type_mismatch(
                operation,
                "two integers",
                &format!("{} and {}", constant_kind(&left), constant_kind(&right)),
            )),
        }
    }

    fn eval_integer_compare(
        &mut self,
        left: H::Id,
        right: H::Id,
        predicate: impl FnOnce(Ordering) -> bool,
    ) -> Result<ConstantValue, ConstantError> {
        let left = self.evaluate_inner(left)?;
        let right = self.evaluate_inner(right)?;

        match (left, right) {
            (ConstantValue::Integer(a), ConstantValue::Integer(b)) => {
                Ok(ConstantValue::Boolean(predicate(a.cmp(&b))))
            }
            (left, right) => Err(type_mismatch(
                "integer comparison",
                "two integers",
                &format!("{} and {}", constant_kind(&left), constant_kind(&right)),
            )),
        }
    }

    fn check_value_budget(
        &self,
        value: &ConstantValue,
    ) -> Result<(), ConstantError> {
        let observed = value_size_units(value);

        if let Some(budget) = self.config.max_value_units {
            if observed > budget {
                return Err(ConstantError::ValueBudgetExceeded {
                    budget,
                    observed,
                });
            }
        }

        Ok(())
    }
}

fn type_mismatch(
    operation: &str,
    expected: &str,
    actual: &str,
) -> ConstantError {
    ConstantError::TypeMismatch {
        operation: operation.to_owned(),
        expected: expected.to_owned(),
        actual: actual.to_owned(),
    }
}

fn constant_kind(value: &ConstantValue) -> &'static str {
    match value {
        ConstantValue::Integer(_) => "integer",
        ConstantValue::Decimal(_) => "decimal",
        ConstantValue::Boolean(_) => "boolean",
        ConstantValue::String(_) => "string",
        ConstantValue::Character(_) => "character",
        ConstantValue::Unit => "unit",
        ConstantValue::Null => "null",
        ConstantValue::Tuple(_) => "tuple",
        ConstantValue::Array(_) => "array",
    }
}

fn value_size_units(value: &ConstantValue) -> usize {
    match value {
        ConstantValue::Integer(value) => value.limb_count(),
        ConstantValue::Decimal(value) => value.spelling().len(),
        ConstantValue::Boolean(_) | ConstantValue::Unit | ConstantValue::Null => 1,
        ConstantValue::String(value) => value.len(),
        ConstantValue::Character(_) => 1,
        ConstantValue::Tuple(values) | ConstantValue::Array(values) => {
            values.iter().fold(values.len(), |total, value| {
                total.saturating_add(value_size_units(value))
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_arbitrary_magnitude_integer() {
        let value = BigInt::parse_radix(
            "12345678901234567890123456789012345678901234567890",
            10,
        )
        .expect("valid integer");

        assert_eq!(
            value.to_string(),
            "12345678901234567890123456789012345678901234567890"
        );
        assert!(value.to_u128().is_none());
    }

    #[test]
    fn parses_standard_radices_and_separators() {
        assert_eq!(BigInt::parse_radix("0xff", 16).unwrap().to_string(), "255");
        assert_eq!(BigInt::parse_radix("0b1010", 2).unwrap().to_string(), "10");
        assert_eq!(BigInt::parse_radix("0o755", 8).unwrap().to_string(), "493");
        assert_eq!(BigInt::parse_radix("1_000_000", 10).unwrap().to_string(), "1000000");
    }

    #[test]
    fn arithmetic_preserves_magnitude_and_sign() {
        let a = BigInt::parse_radix("999999999999999999999", 10).unwrap();
        let b = BigInt::parse_radix("100000000000000000001", 10).unwrap();

        assert_eq!(a.add(&b).to_string(), "1100000000000000000000");
        assert_eq!(a.sub(&b).to_string(), "899999999999999999998");
        assert_eq!(a.mul(&b).to_string(), "99999999999999999999899999999999999999999");
        assert_eq!(b.negated().to_string(), "-100000000000000000001");
    }

    #[test]
    fn division_and_remainder_are_exact() {
        let a = BigInt::parse_radix("1000000000000000000000007", 10).unwrap();
        let b = BigInt::parse_radix("97", 10).unwrap();
        let (q, r) = a.div_rem(&b).unwrap();

        assert_eq!(q.to_string(), "10309278350515463917525");
        assert_eq!(r.to_string(), "82");
        assert!(BigInt::zero().div_rem(&BigInt::zero()).is_err());
    }

    #[test]
    fn signed_division_truncates_toward_zero() {
        let a = BigInt::from_i128(-17);
        let b = BigInt::from_i128(5);
        let (q, r) = a.div_rem(&b).unwrap();

        assert_eq!(q.to_string(), "-3");
        assert_eq!(r.to_string(), "-2");
    }

    #[test]
    fn converts_i128_min_without_overflow() {
        let value = BigInt::from_i128(i128::MIN);
        assert_eq!(value.to_i128(), Some(i128::MIN));
        assert_eq!(value.to_string(), i128::MIN.to_string());
    }

    #[test]
    fn exact_decimal_preserves_source_precision() {
        let decimal = ExactDecimal::parse("0.123456789012345678901234567890").unwrap();
        assert_eq!(decimal.spelling(), "0.123456789012345678901234567890");
    }

    #[test]
    fn rejects_malformed_integer_separators() {
        assert!(BigInt::parse_radix("_12", 10).is_err());
        assert!(BigInt::parse_radix("12_", 10).is_err());
        assert!(BigInt::parse_radix("1__2", 10).is_err());
    }

    #[test]
    fn rejects_invalid_radix() {
        assert!(matches!(
            BigInt::parse_radix("12", 1),
            Err(ConstantError::InvalidRadix(1))
        ));
    }
}
