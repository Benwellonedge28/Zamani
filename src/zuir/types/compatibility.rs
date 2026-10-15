
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR semantic type compatibility
//!
//! Path: `src/zuir/types/compatibility.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Answer semantic type-equivalence, assignment, and conversion-policy
//! queries for ZUIR without coupling type meaning to a physical target.
//!
//! ## AUTHORITY AND OWNERSHIP
//!
//! `types/type_def.rs` owns `TypeDef`, `TypeKind`, `TypeRef`, and the
//! semantic type vocabulary.
//!
//! `schema/types.rs` owns `QualifiedName`.
//!
//! `types/constraints.rs` owns symbolic constraint evaluation.
//!
//! `types/layout.rs` and downstream target analysis own physical
//! representation and ABI compatibility.
//!
//! This module owns only the compatibility queries defined here.
//!
//! ## SEMANTIC CONTRACT
//!
//! - Equal, resolved named type references denote the same nominal type.
//! - Different named types are not interchangeable merely because their
//!   definitions have the same structure.
//! - Anonymous inline types may be compared structurally.
//! - Newtypes preserve nominal identity.
//! - Integer signedness, width, floating/decimal format, function effects,
//!   parameter modes, and domain identity are semantically significant.
//! - Symbolic expressions are not evaluated here.
//! - Unknown or invalid types never prove compatibility.
//! - Assignment is conservative: only established equivalence is accepted
//!   unless a separate, explicit language policy extends this behavior.
//! - Conversion policy is supplied by the caller; this module does not
//!   invent implicit conversions.
//! - Physical layout compatibility is never inferred from semantic
//!   equivalence.
//!
//! ## SCALABILITY
//!
//! Structural comparisons use an explicit worklist. Collections have no
//! universal hardware-size ceiling. Actual work remains bounded by available
//! memory, time, representational limits, and any caller-imposed budget.
//! A budget exhaustion must be reported as `Unknown`, never as success.
//!
//! ## DESERIALIZATION
//!
//! Public data structures may have been deserialized without constructors.
//! Type definitions and qualified names are validated before being trusted.
//!
//! ## FREEZE CRITERIA
//!
//! The parent module must expose this file exactly once. Tests must cover
//! recursive named types, malformed definitions, symbolic types, nominal
//! identity, domain types, and deterministic outcomes.

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::type_def::{
    ConstantArgument, Dimension, FieldDef, FunctionParameter, IntegerWidth,
    ParameterMode, SymbolicExpression, TypeArgument, TypeConstraint, TypeDef,
    TypeKind, TypeRef, VariantDef,
};

use std::collections::BTreeSet;
use std::fmt;

/// Resolves a semantic type name to its authoritative definition.
///
/// Implementations must return definitions from the same coherent semantic
/// environment for the duration of a query. Name resolution and duplicate
/// declaration detection remain the registry's responsibility.
pub trait TypeResolver {
    /// Resolve a qualified semantic type name.
    fn resolve(&self, name: &QualifiedName) -> Option<&TypeDef>;
}

/// The semantic result of a compatibility query.
///
/// `Unknown` is distinct from `Incompatible`: the former means the query
/// could not establish a result from available information; the latter
/// means the known types do not satisfy the requested relation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Compatibility {
    /// Compatibility has been established.
    Compatible,

    /// Compatibility has been disproved.
    Incompatible(CompatibilityReason),

    /// Compatibility could not be established safely.
    Unknown(CompatibilityReason),
}

impl Compatibility {
    /// Whether the query has positively established compatibility.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Whether the query has disproved compatibility.
    #[must_use]
    pub const fn is_incompatible(&self) -> bool {
        matches!(self, Self::Incompatible(_))
    }

    /// Whether the query requires more information or a supported policy.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}

/// Stable, structured reasons for compatibility outcomes.
///
/// These reasons are independent of terminal formatting and diagnostic
/// presentation. Callers may translate them into repository diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompatibilityReason {
    /// A named type could not be found in the supplied environment.
    UnresolvedType {
        name: String,
    },

    /// A resolved definition failed local validation.
    InvalidTypeDefinition {
        name: String,
        detail: String,
    },

    /// The semantic types have different nominal identities.
    NominalIdentityMismatch {
        left: String,
        right: String,
    },

    /// Known semantic properties differ.
    SemanticMismatch {
        detail: String,
    },

    /// A symbolic fact would need evaluation to decide the query.
    UnresolvedSymbolicFact {
        detail: String,
    },

    /// A future or extension-defined type needs an owning compatibility
    /// implementation before the relation can be established.
    UnsupportedTypeRelation {
        detail: String,
    },

    /// The query exhausted its explicitly supplied work budget.
    BudgetExceeded {
        limit: usize,
    },

    /// The selected conversion policy has no answer for this pair.
    ConversionPolicyUnknown {
        source: String,
        destination: String,
    },

    /// The selected conversion policy forbids this conversion.
    ConversionForbidden {
        source: String,
        destination: String,
    },

    /// An explicit conversion is required by policy.
    ExplicitConversionRequired {
        source: String,
        destination: String,
    },
}

impl fmt::Display for CompatibilityReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnresolvedType { name } => {
                write!(f, "unresolved semantic type `{name}`")
            }
            Self::InvalidTypeDefinition { name, detail } => {
                write!(f, "invalid definition of `{name}`: {detail}")
            }
            Self::NominalIdentityMismatch { left, right } => {
                write!(
                    f,
                    "nominal type identity differs: `{left}` versus `{right}`"
                )
            }
            Self::SemanticMismatch { detail } => {
                write!(f, "semantic type mismatch: {detail}")
            }
            Self::UnresolvedSymbolicFact { detail } => {
                write!(f, "unresolved symbolic type fact: {detail}")
            }
            Self::UnsupportedTypeRelation { detail } => {
                write!(f, "unsupported type relation: {detail}")
            }
            Self::BudgetExceeded { limit } => {
                write!(f, "type compatibility work budget exceeded ({limit})")
            }
            Self::ConversionPolicyUnknown {
                source,
                destination,
            } => write!(
                f,
                "no conversion policy for `{source}` to `{destination}`"
            ),
            Self::ConversionForbidden {
                source,
                destination,
            } => write!(
                f,
                "conversion from `{source}` to `{destination}` is forbidden"
            ),
            Self::ExplicitConversionRequired {
                source,
                destination,
            } => write!(
                f,
                "explicit conversion required from `{source}` to `{destination}`"
            ),
        }
    }
}

impl std::error::Error for CompatibilityReason {}

/// Resource policy for a compatibility query.
///
/// The budget counts processed type-reference pairs. It is an operational
/// safeguard, not a semantic limit on the size of a valid type. Use
/// `unlimited()` when the caller deliberately delegates the query's resource
/// use to the process/environment policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityBudget {
    max_pairs: Option<usize>,
}

impl CompatibilityBudget {
    /// Construct a budget that permits at most `max_pairs` processed pairs.
    #[must_use]
    pub const fn limited(max_pairs: usize) -> Self {
        Self {
            max_pairs: Some(max_pairs),
        }
    }

    /// Use no additional pair-count budget.
    ///
    /// This does not promise infinite resources or protect against
    /// environmental memory exhaustion.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { max_pairs: None }
    }

    /// Return the configured pair-count limit, if present.
    #[must_use]
    pub const fn max_pairs(self) -> Option<usize> {
        self.max_pairs
    }
}

impl Default for CompatibilityBudget {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// The result of applying a conversion policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversionDecision {
    /// Source and destination are semantically equivalent.
    Identity,

    /// The language policy permits the conversion implicitly.
    Implicit,

    /// The language permits the conversion only when explicitly requested.
    Explicit,

    /// The conversion is not permitted.
    Forbidden,

    /// The policy cannot decide from the supplied information.
    Unknown,
}

/// Supplies language- or dialect-specific conversion rules.
///
/// Implementations must be deterministic for the same type environment and
/// policy snapshot. The default policy is conservative and does not infer
/// conversions between different types.
pub trait ConversionPolicy {
    /// Classify a conversion from `source` to `destination`.
    fn classify(
        &self,
        source: &TypeRef,
        destination: &TypeRef,
    ) -> ConversionDecision;
}

/// A policy that allows identity conversions only.
///
/// This is the safe default for assignment-like operations. It does not
/// silently permit numeric widening, signedness changes, representation
/// changes, domain conversions, or user-defined conversions.
#[derive(Debug, Clone, Copy, Default)]
pub struct IdentityOnlyConversionPolicy;

impl ConversionPolicy for IdentityOnlyConversionPolicy {
    fn classify(
        &self,
        _source: &TypeRef,
        _destination: &TypeRef,
    ) -> ConversionDecision {
        ConversionDecision::Explicit
    }
}

/// Performs semantic compatibility queries against one type environment.
pub struct CompatibilityChecker<'a, R: TypeResolver + ?Sized> {
    resolver: &'a R,
    budget: CompatibilityBudget,
}

impl<'a, R: TypeResolver + ?Sized> CompatibilityChecker<'a, R> {
    /// Create a checker using the caller's resolver and default budget.
    #[must_use]
    pub fn new(resolver: &'a R) -> Self {
        Self {
            resolver,
            budget: CompatibilityBudget::default(),
        }
    }

    /// Create a checker with an explicit operational budget.
    #[must_use]
    pub fn with_budget(
        resolver: &'a R,
        budget: CompatibilityBudget,
    ) -> Self {
        Self { resolver, budget }
    }

    /// Return the budget used by this checker.
    #[must_use]
    pub const fn budget(&self) -> CompatibilityBudget {
        self.budget
    }

    /// Check semantic equivalence.
    ///
    /// Named types use nominal identity. Anonymous inline types are compared
    /// structurally, with every referenced named type resolved and validated.
    pub fn equivalent(
        &self,
        left: &TypeRef,
        right: &TypeRef,
    ) -> Compatibility {
        let mut pending = vec![(left.clone(), right.clone())];

        // Recursive definitions are legal. Record named pairs already
        // visited to avoid repeatedly traversing recursive references.
        let mut visited_named_pairs: BTreeSet<(QualifiedName, QualifiedName)> =
            BTreeSet::new();

        let mut processed = 0usize;

        while let Some((left_ref, right_ref)) = pending.pop() {
            if let Some(limit) = self.budget.max_pairs {
                if processed >= limit {
                    return Compatibility::Unknown(
                        CompatibilityReason::BudgetExceeded { limit },
                    );
                }
            }

            processed = match processed.checked_add(1) {
                Some(value) => value,
                None => {
                    return Compatibility::Unknown(
                        CompatibilityReason::BudgetExceeded {
                            limit: self.budget.max_pairs.unwrap_or(usize::MAX),
                        },
                    );
                }
            };

            // Resolve and validate even identical named references. Textual
            // equality alone is not proof that a declaration exists.
            if let (TypeRef::Named(left_name), TypeRef::Named(right_name)) =
                (&left_ref, &right_ref)
            {
                if left_name == right_name {
                    if let Err(reason) = self.resolve_validated(left_name) {
                        return Compatibility::Unknown(reason);
                    }
                    continue;
                }

                // Named references are nominal. Structural coincidence
                // cannot erase their distinct declared identities.
                return Compatibility::Incompatible(
                    CompatibilityReason::NominalIdentityMismatch {
                        left: left_name.canonical_string(),
                        right: right_name.canonical_string(),
                    },
                );
            }

            let left_kind = match self.resolve_kind(&left_ref) {
                Ok(kind) => kind,
                Err(reason) => return Compatibility::Unknown(reason),
            };
            let right_kind = match self.resolve_kind(&right_ref) {
                Ok(kind) => kind,
                Err(reason) => return Compatibility::Unknown(reason),
            };

            // Once a named reference is encountered inside an inline type,
            // validate the declaration and use its nominal identity. This
            // also prevents infinite expansion of recursive named graphs.
            if let (Some(left_name), Some(right_name)) = (
                named_origin(&left_ref),
                named_origin(&right_ref),
            ) {
                if !visited_named_pairs.insert((
                    left_name.clone(),
                    right_name.clone(),
                )) {
                    continue;
                }
            }

            match compare_kinds(left_kind, right_kind, &mut pending) {
                KindComparison::Equal => {}
                KindComparison::Different(detail) => {
                    return Compatibility::Incompatible(
                        CompatibilityReason::SemanticMismatch { detail },
                    );
                }
                KindComparison::Unknown(detail) => {
                    return Compatibility::Unknown(
                        CompatibilityReason::UnresolvedSymbolicFact { detail },
                    );
                }
                KindComparison::Unsupported(detail) => {
                    return Compatibility::Unknown(
                        CompatibilityReason::UnsupportedTypeRelation { detail },
                    );
                }
            }
        }

        Compatibility::Compatible
    }

    /// Check assignment compatibility from `source` to `destination`.
    ///
    /// The default core rule accepts only established semantic equivalence.
    /// A language-specific assignment system may layer additional permitted
    /// conversions on top of this query through `ConversionPolicy`.
    pub fn assignable(
        &self,
        source: &TypeRef,
        destination: &TypeRef,
    ) -> Compatibility {
        self.equivalent(source, destination)
    }

    /// Check conversion legality using an explicit policy.
    ///
    /// Equivalence is checked first. If the types differ, the supplied
    /// policy must explicitly classify the conversion. This method never
    /// guesses that a conversion is safe merely because types look similar.
    pub fn conversion<P: ConversionPolicy + ?Sized>(
        &self,
        source: &TypeRef,
        destination: &TypeRef,
        policy: &P,
    ) -> Compatibility {
        match self.equivalent(source, destination) {
            Compatibility::Compatible => return Compatibility::Compatible,
            Compatibility::Unknown(reason) => {
                return Compatibility::Unknown(reason);
            }
            Compatibility::Incompatible(_) => {}
        }

        let source_name = describe_ref(source);
        let destination_name = describe_ref(destination);

        match policy.classify(source, destination) {
            ConversionDecision::Identity | ConversionDecision::Implicit => {
                Compatibility::Compatible
            }
            ConversionDecision::Explicit => Compatibility::Incompatible(
                CompatibilityReason::ExplicitConversionRequired {
                    source: source_name,
                    destination: destination_name,
                },
            ),
            ConversionDecision::Forbidden => Compatibility::Incompatible(
                CompatibilityReason::ConversionForbidden {
                    source: source_name,
                    destination: destination_name,
                },
            ),
            ConversionDecision::Unknown => Compatibility::Unknown(
                CompatibilityReason::ConversionPolicyUnknown {
                    source: source_name,
                    destination: destination_name,
                },
            ),
        }
    }

    fn resolve_validated(
        &self,
        name: &QualifiedName,
    ) -> Result<&'a TypeDef, CompatibilityReason> {
        name.validate().map_err(|error| {
            CompatibilityReason::InvalidTypeDefinition {
                name: name.canonical_string(),
                detail: error.to_string(),
            }
        })?;

        let definition = self.resolver.resolve(name).ok_or_else(|| {
            CompatibilityReason::UnresolvedType {
                name: name.canonical_string(),
            }
        })?;

        definition.validate().map_err(|error| {
            CompatibilityReason::InvalidTypeDefinition {
                name: name.canonical_string(),
                detail: error.to_string(),
            }
        })?;

        Ok(definition)
    }

    fn resolve_kind<'b>(
        &'b self,
        reference: &'b TypeRef,
    ) -> Result<&'b TypeKind, CompatibilityReason>
    where
        'a: 'b,
    {
        match reference {
            TypeRef::Inline(kind) => Ok(kind.as_ref()),
            TypeRef::Named(name) => {
                let definition = self.resolve_validated(name)?;
                Ok(&definition.kind)
            }
        }
    }
}

fn named_origin(reference: &TypeRef) -> Option<&QualifiedName> {
    match reference {
        TypeRef::Named(name) => Some(name),
        TypeRef::Inline(_) => None,
    }
}

fn describe_ref(reference: &TypeRef) -> String {
    match reference {
        TypeRef::Named(name) => name.canonical_string(),
        TypeRef::Inline(kind) => format!("inline::{kind:?}"),
    }
}

enum KindComparison {
    Equal,
    Different(String),
    Unknown(String),
    Unsupported(String),
}

fn compare_kinds(
    left: &TypeKind,
    right: &TypeKind,
    pending: &mut Vec<(TypeRef, TypeRef)>,
) -> KindComparison {
    use TypeKind as K;

    match (left, right) {
        (K::Never, K::Never)
        | (K::Unit, K::Unit)
        | (K::Boolean, K::Boolean)
        | (K::String, K::String)
        | (K::Bytes, K::Bytes) => KindComparison::Equal,

        (
            K::Integer {
                signedness: left_sign,
                width: left_width,
            },
            K::Integer {
                signedness: right_sign,
                width: right_width,
            },
        ) => {
            if left_sign != right_sign {
                return different("integer signedness differs");
            }
            compare_widths(left_width, right_width)
        }

        (K::Float { format: left }, K::Float { format: right })
        | (K::Decimal { format: left }, K::Decimal { format: right }) => {
            if left == right {
                KindComparison::Equal
            } else {
                different("numeric formats differ")
            }
        }

        (K::Tuple { elements: left }, K::Tuple { elements: right }) => {
            compare_type_lists(left, right, pending, "tuple arity differs")
        }

        (
            K::Array {
                element: left_element,
                length: left_length,
            },
            K::Array {
                element: right_element,
                length: right_length,
            },
        ) => {
            match compare_dimensions(left_length, right_length) {
                KindComparison::Equal => {}
                other => return other,
            }
            pending.push((left_element.clone(), right_element.clone()));
            KindComparison::Equal
        }

        (K::Record { fields: left }, K::Record { fields: right }) => {
            compare_fields(left, right, pending)
        }

        (K::Sum { variants: left }, K::Sum { variants: right }) => {
            compare_variants(left, right, pending)
        }

        (
            K::Union {
                alternatives: left,
            },
            K::Union {
                alternatives: right,
            },
        ) => compare_type_lists(
            left,
            right,
            pending,
            "union alternatives differ",
        ),

        (
            K::Reference { target: left },
            K::Reference { target: right },
        ) => {
            if left == right {
                KindComparison::Equal
            } else {
                different("reference target identity differs")
            }
        }

        (
            K::Newtype { underlying: left },
            K::Newtype { underlying: right },
        ) => {
            // A newtype is nominal. The underlying representation does not
            // erase the distinct wrapper's identity.
            if left == right {
                KindComparison::Equal
            } else {
                different("newtype definitions differ")
            }
        }

        (
            K::Opaque {
                constraints: left,
            },
            K::Opaque {
                constraints: right,
            },
        ) => {
            if left == right {
                KindComparison::Equal
            } else {
                KindComparison::Unknown(
                    "opaque type constraints require their owning policy"
                        .to_owned(),
                )
            }
        }

        (
            K::Parameter {
                name: left_name,
                bounds: left_bounds,
            },
            K::Parameter {
                name: right_name,
                bounds: right_bounds,
            },
        ) => {
            if left_name == right_name && left_bounds == right_bounds {
                KindComparison::Equal
            } else {
                KindComparison::Unknown(
                    "generic parameter identity or bounds differ".to_owned(),
                )
            }
        }

        (
            K::Applied {
                constructor: left_constructor,
                arguments: left_arguments,
            },
            K::Applied {
                constructor: right_constructor,
                arguments: right_arguments,
            },
        ) => compare_applications(
            left_constructor,
            left_arguments,
            right_constructor,
            right_arguments,
            pending,
        ),

        (
            K::Function {
                parameters: left_parameters,
                results: left_results,
                effects: left_effects,
                calling_convention: left_convention,
            },
            K::Function {
                parameters: right_parameters,
                results: right_results,
                effects: right_effects,
                calling_convention: right_convention,
            },
        ) => {
            if left_parameters.len() != right_parameters.len() {
                return different("function parameter count differs");
            }
            if left_results.len() != right_results.len() {
                return different("function result count differs");
            }
            if left_effects != right_effects {
                return different("function effect declarations differ");
            }
            if left_convention != right_convention {
                return different("function calling conventions differ");
            }

            for (left_parameter, right_parameter) in
                left_parameters.iter().zip(right_parameters)
            {
                if !parameters_compatible(left_parameter, right_parameter) {
                    return different(
                        "function parameter modes or names differ",
                    );
                }
                pending.push((
                    left_parameter.ty.clone(),
                    right_parameter.ty.clone(),
                ));
            }

            for (left_result, right_result) in
                left_results.iter().zip(right_results)
            {
                pending.push((left_result.clone(), right_result.clone()));
            }

            KindComparison::Equal
        }

        (
            K::Quantum {
                domain_type: left_domain,
                parameters: left_parameters,
            },
            K::Quantum {
                domain_type: right_domain,
                parameters: right_parameters,
            },
        )
        | (
            K::Hardware {
                domain_type: left_domain,
                parameters: left_parameters,
            },
            K::Hardware {
                domain_type: right_domain,
                parameters: right_parameters,
            },
        ) => compare_domain_types(
            left_domain,
            left_parameters,
            right_domain,
            right_parameters,
            pending,
        ),

        (
            K::Dialect {
                dialect: left_dialect,
                type_name: left_name,
                arguments: left_arguments,
            },
            K::Dialect {
                dialect: right_dialect,
                type_name: right_name,
                arguments: right_arguments,
            },
        ) => {
            if left_dialect != right_dialect || left_name != right_name {
                return different("dialect-defined type identity differs");
            }
            compare_arguments(left_arguments, right_arguments, pending)
        }

        // A new TypeKind variant must not become compatible accidentally.
        #[allow(unreachable_patterns)]
        _ => different("type constructors differ or are unsupported"),
    }
}

fn compare_widths(
    left: &IntegerWidth,
    right: &IntegerWidth,
) -> KindComparison {
    match (left, right) {
        (IntegerWidth::Unbounded, IntegerWidth::Unbounded) => {
            KindComparison::Equal
        }
        (IntegerWidth::Fixed(left), IntegerWidth::Fixed(right))
            if left == right =>
        {
            KindComparison::Equal
        }
        (
            IntegerWidth::Symbolic(SymbolicExpression {
                expression: left,
            }),
            IntegerWidth::Symbolic(SymbolicExpression {
                expression: right,
            }),
        ) if left == right => KindComparison::Equal,

        (IntegerWidth::Symbolic(_), _) | (_, IntegerWidth::Symbolic(_)) => {
            KindComparison::Unknown(
                "integer width requires symbolic constraint evaluation"
                    .to_owned(),
            )
        }

        _ => different("integer widths differ"),
    }
}

fn compare_dimensions(
    left: &Dimension,
    right: &Dimension,
) -> KindComparison {
    match (left, right) {
        (Dimension::Known(left), Dimension::Known(right))
            if left == right =>
        {
            KindComparison::Equal
        }
        (Dimension::Dynamic, Dimension::Dynamic) => KindComparison::Equal,
        (
            Dimension::Symbolic(SymbolicExpression {
                expression: left,
            }),
            Dimension::Symbolic(SymbolicExpression {
                expression: right,
            }),
        ) if left == right => KindComparison::Equal,

        (Dimension::Symbolic(_), _) | (_, Dimension::Symbolic(_)) => {
            KindComparison::Unknown(
                "array dimensions require symbolic constraint evaluation"
                    .to_owned(),
            )
        }

        _ => different("array dimensions differ"),
    }
}

fn compare_type_lists(
    left: &[TypeRef],
    right: &[TypeRef],
    pending: &mut Vec<(TypeRef, TypeRef)>,
    length_mismatch: &'static str,
) -> KindComparison {
    if left.len() != right.len() {
        return different(length_mismatch);
    }

    for (left, right) in left.iter().zip(right) {
        pending.push((left.clone(), right.clone()));
    }

    KindComparison::Equal
}

fn compare_fields(
    left: &[FieldDef],
    right: &[FieldDef],
    pending: &mut Vec<(TypeRef, TypeRef)>,
) -> KindComparison {
    if left.len() != right.len() {
        return different("record field count differs");
    }

    for (left_field, right_field) in left.iter().zip(right) {
        if left_field.name != right_field.name
            || left_field.attributes != right_field.attributes
        {
            return different("record field names or attributes differ");
        }
        pending.push((left_field.ty.clone(), right_field.ty.clone()));
    }

    KindComparison::Equal
}

fn compare_variants(
    left: &[VariantDef],
    right: &[VariantDef],
    pending: &mut Vec<(TypeRef, TypeRef)>,
) -> KindComparison {
    if left.len() != right.len() {
        return different("sum type variant count differs");
    }

    for (left_variant, right_variant) in left.iter().zip(right) {
        if left_variant.name != right_variant.name
            || left_variant.attributes != right_variant.attributes
        {
            return different("sum type variant names or attributes differ");
        }

        match compare_fields(
            &left_variant.fields,
            &right_variant.fields,
            pending,
        ) {
            KindComparison::Equal => {}
            other => return other,
        }
    }

    KindComparison::Equal
}

fn compare_applications(
    left_constructor: &QualifiedName,
    left_arguments: &[TypeArgument],
    right_constructor: &QualifiedName,
    right_arguments: &[TypeArgument],
    pending: &mut Vec<(TypeRef, TypeRef)>,
) -> KindComparison {
    if left_constructor != right_constructor {
        return different("generic type constructors differ");
    }

    compare_arguments(left_arguments, right_arguments, pending)
}

fn compare_arguments(
    left: &[TypeArgument],
    right: &[TypeArgument],
    pending: &mut Vec<(TypeRef, TypeRef)>,
) -> KindComparison {
    if left.len() != right.len() {
        return different("type argument count differs");
    }

    for (left_argument, right_argument) in left.iter().zip(right) {
        match (left_argument, right_argument) {
            (TypeArgument::Type(left), TypeArgument::Type(right)) => {
                pending.push((left.clone(), right.clone()));
            }
            (TypeArgument::Constant(left), TypeArgument::Constant(right)) => {
                if !constants_equal(left, right) {
                    return different("generic constant arguments differ");
                }
            }
            (
                TypeArgument::Symbolic(SymbolicExpression {
                    expression: left,
                }),
                TypeArgument::Symbolic(SymbolicExpression {
                    expression: right,
                }),
            ) if left == right => {}
            (TypeArgument::Symbolic(_), _)
            | (_, TypeArgument::Symbolic(_)) => {
                return KindComparison::Unknown(
                    "generic symbolic arguments need constraint evaluation"
                        .to_owned(),
                );
            }
            _ => return different("generic argument kinds differ"),
        }
    }

    KindComparison::Equal
}

fn compare_domain_types(
    left_domain: &QualifiedName,
    left_arguments: &[TypeArgument],
    right_domain: &QualifiedName,
    right_arguments: &[TypeArgument],
    pending: &mut Vec<(TypeRef, TypeRef)>,
) -> KindComparison {
    if left_domain != right_domain {
        return different("computational domain type differs");
    }

    compare_arguments(left_arguments, right_arguments, pending)
}

fn parameters_compatible(
    left: &FunctionParameter,
    right: &FunctionParameter,
) -> bool {
    left.name == right.name && parameter_modes_compatible(left.mode, right.mode)
}

fn parameter_modes_compatible(
    left: ParameterMode,
    right: ParameterMode,
) -> bool {
    left == right
}

fn constants_equal(
    left: &ConstantArgument,
    right: &ConstantArgument,
) -> bool {
    match (left, right) {
        (ConstantArgument::Boolean(left), ConstantArgument::Boolean(right)) => {
            left == right
        }
        (ConstantArgument::Integer(left), ConstantArgument::Integer(right)) => {
            // Preserve literal representation here. Numeric normalization
            // belongs to the canonical constant evaluator.
            left == right
        }
        (ConstantArgument::String(left), ConstantArgument::String(right)) => {
            left == right
        }
        (ConstantArgument::Name(left), ConstantArgument::Name(right)) => {
            left == right
        }
        _ => false,
    }
}

fn different(detail: impl Into<String>) -> KindComparison {
    KindComparison::Different(detail.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::schema::types::QualifiedName;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct TestResolver {
        definitions: BTreeMap<QualifiedName, TypeDef>,
    }

    impl TestResolver {
        fn insert(&mut self, definition: TypeDef) {
            self.definitions.insert(definition.name.clone(), definition);
        }
    }

    impl TypeResolver for TestResolver {
        fn resolve(&self, name: &QualifiedName) -> Option<&TypeDef> {
            self.definitions.get(name)
        }
    }

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("valid test qualified name")
    }

    fn definition(value: &str, kind: TypeKind) -> TypeDef {
        TypeDef::new(name(value), kind).expect("valid test type definition")
    }

    fn named(value: &str) -> TypeRef {
        TypeRef::Named(name(value))
    }

    fn inline(kind: TypeKind) -> TypeRef {
        TypeRef::Inline(Box::new(kind))
    }

    #[test]
    fn identical_named_types_must_resolve() {
        let resolver = TestResolver::default();
        let checker = CompatibilityChecker::new(&resolver);

        assert!(matches!(
            checker.equivalent(&named("core::Missing"), &named("core::Missing")),
            Compatibility::Unknown(
                CompatibilityReason::UnresolvedType { .. }
            )
        ));
    }

    #[test]
    fn identical_resolved_named_types_are_equivalent() {
        let mut resolver = TestResolver::default();
        resolver.insert(definition("core::Flag", TypeKind::Boolean));

        let checker = CompatibilityChecker::new(&resolver);
        assert_eq!(
            checker.equivalent(&named("core::Flag"), &named("core::Flag")),
            Compatibility::Compatible
        );
    }

    #[test]
    fn distinct_named_types_remain_nominal() {
        let mut resolver = TestResolver::default();
        resolver.insert(definition("app::Left", TypeKind::Boolean));
        resolver.insert(definition("app::Right", TypeKind::Boolean));

        let checker = CompatibilityChecker::new(&resolver);
        assert!(matches!(
            checker.equivalent(&named("app::Left"), &named("app::Right")),
            Compatibility::Incompatible(
                CompatibilityReason::NominalIdentityMismatch { .. }
            )
        ));
    }

    #[test]
    fn inline_types_are_compared_structurally() {
        let resolver = TestResolver::default();
        let checker = CompatibilityChecker::new(&resolver);

        assert_eq!(
            checker.equivalent(
                &inline(TypeKind::Boolean),
                &inline(TypeKind::Boolean)
            ),
            Compatibility::Compatible
        );

        assert!(matches!(
            checker.equivalent(
                &inline(TypeKind::Boolean),
                &inline(TypeKind::String)
            ),
            Compatibility::Incompatible(_)
        ));
    }

    #[test]
    fn signedness_is_semantically_significant() {
        use crate::zuir::types::type_def::IntegerSignedness;

        let resolver = TestResolver::default();
        let checker = CompatibilityChecker::new(&resolver);

        let signed = inline(TypeKind::Integer {
            signedness: IntegerSignedness::Signed,
            width: IntegerWidth::Fixed(32),
        });
        let unsigned = inline(TypeKind::Integer {
            signedness: IntegerSignedness::Unsigned,
            width: IntegerWidth::Fixed(32),
        });

        assert!(matches!(
            checker.equivalent(&signed, &unsigned),
            Compatibility::Incompatible(_)
        ));
    }

    #[test]
    fn unresolved_symbolic_width_is_not_proven_equivalent() {
        let resolver = TestResolver::default();
        let checker = CompatibilityChecker::new(&resolver);

        let left = inline(TypeKind::Integer {
            signedness: crate::zuir::types::type_def::IntegerSignedness::Signless,
            width: IntegerWidth::Symbolic(SymbolicExpression {
                expression: "width_a".to_owned(),
            }),
        });
        let right = inline(TypeKind::Integer {
            signedness: crate::zuir::types::type_def::IntegerSignedness::Signless,
            width: IntegerWidth::Fixed(32),
        });

        assert!(matches!(
            checker.equivalent(&left, &right),
            Compatibility::Unknown(
                CompatibilityReason::UnresolvedSymbolicFact { .. }
            )
        ));
    }

    #[test]
    fn assignment_does_not_invent_implicit_conversions() {
        let resolver = TestResolver::default();
        let checker = CompatibilityChecker::new(&resolver);

        assert!(matches!(
            checker.assignable(
                &inline(TypeKind::Boolean),
                &inline(TypeKind::String)
            ),
            Compatibility::Incompatible(_)
        ));
    }

    #[test]
    fn conversion_policy_must_authorize_non_identity_conversion() {
        struct PermitImplicit;

        impl ConversionPolicy for PermitImplicit {
            fn classify(
                &self,
                _source: &TypeRef,
                _destination: &TypeRef,
            ) -> ConversionDecision {
                ConversionDecision::Implicit
            }
        }

        let resolver = TestResolver::default();
        let checker = CompatibilityChecker::new(&resolver);

        assert_eq!(
            checker.conversion(
                &inline(TypeKind::Boolean),
                &inline(TypeKind::String),
                &PermitImplicit
            ),
            Compatibility::Compatible
        );
    }

    #[test]
    fn work_budget_exhaustion_is_unknown_not_success() {
        let resolver = TestResolver::default();
        let checker = CompatibilityChecker::with_budget(
            &resolver,
            CompatibilityBudget::limited(0),
        );

        assert!(matches!(
            checker.equivalent(
                &inline(TypeKind::Boolean),
                &inline(TypeKind::Boolean)
            ),
            Compatibility::Unknown(
                CompatibilityReason::BudgetExceeded { limit: 0 }
            )
        ));
    }
}
