
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani semantic type compatibility.
//!
//! STATUS: Implementation contract for the native semantic subsystem.
//!
//! PURPOSE
//!
//! Determine whether an actual resolved semantic type can be used where an
//! expected semantic type is required. This module is deliberately separate
//! from source-level `TypeExpr`, type inference, unification, trait solving,
//! constant evaluation, and target feasibility.
//!
//! INTEGRATION
//!
//! The canonical semantic type in `super::ty` must implement [`TypeShape`].
//! Its `type_view()` method must expose the semantic meaning of a type without
//! erasing nominal identity, generic arguments, symbolic dimensions, or
//! domain qualifiers.
//!
//! This module does not assume that `ty.rs` or a particular semantic type
//! representation already exists. The adapter is the integration boundary.
//!
//! SEMANTIC RULES
//!
//! * Unknown, inferred, error-tainted, and unresolved types are not accepted
//!   as compatible evidence. They produce [`Compatibility::Undetermined`].
//! * Numeric conversions are not implicitly permitted here. The coercion
//!   subsystem must authorize a conversion explicitly.
//! * Array dimensions are compared; they are never silently discarded.
//! * Named types are nominal unless the semantic type system explicitly
//!   represents a structural type.
//! * Hardware availability, device capacity, and physical realization are
//!   outside this module.
//! * Compatibility checks use an iterative worklist rather than recursive
//!   descent, avoiding call-stack growth proportional to type nesting.
//!
//! SCALABILITY
//!
//! There are no language-level limits on tuple arity, generic arity, function
//! arity, array dimensions, or the number of compatibility obligations.
//! Actual execution is necessarily bounded by available memory, processing
//! time, and any explicit compiler operational budget.
//!
//! TYPE GRAPH CONTRACT
//!
//! `TypeShape::type_view()` must expose a finite, acyclic structural view for
//! each comparison. Recursive nominal types should refer to stable nominal
//! identities rather than recursively expanding their definitions here.
//! Recursive alias and type-definition validity belongs to alias/cycle
//! analysis. A violation of this contract is an adapter defect.
//!
//! The returned view must be deterministic and must not change during one
//! comparison.
//!
//! AUTHORITY
//!
//! * `grammar/Zamani.g4` and the composed grammar define syntax.
//! * `grammar/specification/semantic-model.md` defines semantic meaning.
//! * The canonical semantic type model defines resolved type identity.
//! * `types/coercions.rs` owns permitted implicit conversions.
//! * `types/unification.rs` owns inference-variable solving.
//! * `types/traits.rs` owns trait and interface obligations.
//!
//! This file owns compatibility decisions only.
//!
//! Rust: edition 2021, Rust 1.97+, stable, safe Rust only.

use core::fmt;

// =============================================================================
// Semantic type adapter contract
// =============================================================================

/// A resolved semantic type that can be inspected by the compatibility
/// engine.
///
/// Implement this trait for the single canonical semantic type representation
/// owned by `types/ty.rs`.
///
/// Implementations must not map unresolved types to a concrete primitive,
/// must preserve nominal identity, and must not encode target-specific sizes
/// as source-language type identity.
///
/// Structural child collections must remain stable for the duration of a
/// comparison. Structural views must be finite and acyclic. Recursive named
/// types should be represented by nominal identities.
pub trait TypeShape {
    /// Returns a borrowed semantic view of this type.
    fn type_view(&self) -> TypeView<'_, Self>
    where
        Self: Sized;
}

/// A borrowed, domain-neutral view of a resolved semantic type.
///
/// Strings identifying primitives and nominal types must be canonical
/// semantic identities, not merely display strings that may be ambiguous.
#[derive(Clone, Copy, Debug)]
pub enum TypeView<'a, T> {
    /// A primitive semantic type, identified by a canonical key.
    ///
    /// Examples might include language-defined integer, boolean, text, or
    /// floating-point types. The compatibility engine does not infer numeric
    /// widening from these names.
    Primitive(&'a str),

    /// A nominal type with ordered generic arguments.
    Nominal {
        identity: &'a str,
        arguments: &'a [T],
    },

    /// A tuple whose element order is semantically significant.
    Tuple(&'a [T]),

    /// An array with a symbolic or known source-level cardinality.
    Array {
        element: &'a T,
        length: ArrayLength<'a>,
    },

    /// A dynamically sized sequence.
    Slice(&'a T),

    /// A function type.
    Function {
        parameters: &'a [T],
        return_type: &'a T,
    },

    /// A reference or borrow.
    Reference {
        mutable: bool,
        inner: &'a T,
    },

    /// A pointer type.
    Pointer {
        mutable: bool,
        inner: &'a T,
    },

    /// An optional value.
    Optional(&'a T),

    /// A result with success and error types.
    Result {
        ok: &'a T,
        error: &'a T,
    },

    /// The never type.
    Never,

    /// The unit type.
    Unit,

    /// An unresolved inference variable.
    InferenceVariable(u64),

    /// A union type.
    Union(&'a [T]),

    /// An intersection type.
    Intersection(&'a [T]),

    /// A quantum semantic wrapper or qualifier.
    Quantum(&'a T),

    /// A linear semantic wrapper or qualifier.
    Linear(&'a T),

    /// An affine semantic wrapper or qualifier.
    Affine(&'a T),

    /// A temporal/resource-lifetime semantic wrapper or qualifier.
    Temporal(&'a T),

    /// A type whose semantics are not yet resolved.
    Unresolved,

    /// A type tainted by an earlier semantic error.
    Error,

    /// A registered extension type.
    ///
    /// `namespace` and `name` together form its canonical identity. Generic
    /// arguments remain ordered.
    Extension {
        namespace: &'a str,
        name: &'a str,
        arguments: &'a [T],
    },
}

/// A semantic array cardinality.
///
/// Symbolic expressions must be normalized by the constant/constraint
/// subsystem before two differently written expressions can be proven equal.
/// This module intentionally does not evaluate source expressions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrayLength<'a> {
    /// No explicit cardinality is present in the resolved type.
    Unspecified,

    /// A normalized compile-time cardinality, represented canonically.
    Known(&'a str),

    /// A normalized symbolic cardinality.
    Symbolic(&'a str),
}

// =============================================================================
// Public compatibility result and policy
// =============================================================================

/// Result of a type-compatibility check.
///
/// `Undetermined` is intentionally distinct from `Compatible`: unresolved
/// types cannot be used as proof that a program is well typed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compatibility {
    /// The expected type is satisfied under the selected relation.
    Compatible,

    /// The types are known and the relation is not satisfied.
    Incompatible,

    /// More semantic information or a separate solver is required.
    Undetermined,
}

impl Compatibility {
    /// Returns true only when compatibility was positively established.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns true when the result requires further semantic analysis.
    #[must_use]
    pub const fn is_undetermined(self) -> bool {
        matches!(self, Self::Undetermined)
    }
}

/// The relation requested by a caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompatibilityMode {
    /// Require equivalent type structure and identity.
    ///
    /// This mode does not apply subtyping rules.
    Exact,

    /// Check whether an actual value can be used as an expected type.
    ///
    /// This mode includes the language-level bottom/never rule and function
    /// variance, but does not silently authorize numeric or user-defined
    /// conversions.
    Assignable,
}

/// Options that control compatibility semantics.
///
/// This is a semantic policy, not a hardware configuration. It contains no
/// architecture limits. Numeric conversion policy and user-defined coercions
/// intentionally live in `coercions.rs`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompatibilityPolicy {
    /// Exact type equality or assignment compatibility.
    pub mode: CompatibilityMode,

    /// Whether reference and pointer qualifiers must match exactly.
    ///
    /// When true, mutable and immutable references are never interchangeable.
    /// This conservative default should be retained unless the normative
    /// ownership model explicitly authorizes a conversion.
    pub strict_mutability: bool,
}

impl Default for CompatibilityPolicy {
    fn default() -> Self {
        Self {
            mode: CompatibilityMode::Exact,
            strict_mutability: true,
        }
    }
}

impl CompatibilityPolicy {
    /// Creates a strict equivalence policy.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            mode: CompatibilityMode::Exact,
            strict_mutability: true,
        }
    }

    /// Creates a conservative assignment-compatibility policy.
    #[must_use]
    pub const fn assignable() -> Self {
        Self {
            mode: CompatibilityMode::Assignable,
            strict_mutability: true,
        }
    }

    /// Allows the language's reference rules to decide mutability variance.
    ///
    /// This option must only be used when the normative ownership rules have
    /// been implemented and tested. It does not authorize pointer conversions.
    #[must_use]
    pub const fn with_reference_variance(mut self) -> Self {
        self.strict_mutability = false;
        self
    }
}

// =============================================================================
// Internal worklist
// =============================================================================

#[derive(Clone, Copy)]
struct Obligation<'a, T> {
    actual: &'a T,
    expected: &'a T,
    mode: CompatibilityMode,
}

/// Determines whether `actual` satisfies `expected`.
///
/// This function is conservative: it returns `Compatible` only when the
/// relationship is established by this module's rules. Unification variables,
/// unresolved types, and error-tainted types return `Undetermined`.
///
/// The implementation is iterative. Its memory consumption is proportional
/// to the outstanding structural obligations and is limited only by available
/// resources or an explicit budget enforced by the caller.
#[must_use]
pub fn check_compatibility<T: TypeShape>(
    actual: &T,
    expected: &T,
    policy: CompatibilityPolicy,
) -> Compatibility {
    let mut worklist = vec![Obligation {
        actual,
        expected,
        mode: policy.mode,
    }];

    let mut undetermined = false;

    while let Some(obligation) = worklist.pop() {
        let actual_view = obligation.actual.type_view();
        let expected_view = obligation.expected.type_view();

        match compare_views(
            actual_view,
            expected_view,
            obligation.mode,
            policy.strict_mutability,
            &mut worklist,
        ) {
            Compatibility::Compatible => {}
            Compatibility::Incompatible => return Compatibility::Incompatible,
            Compatibility::Undetermined => undetermined = true,
        }
    }

    if undetermined {
        Compatibility::Undetermined
    } else {
        Compatibility::Compatible
    }
}

/// Convenience helper for strict type equivalence.
#[must_use]
pub fn types_equal<T: TypeShape>(left: &T, right: &T) -> Compatibility {
    check_compatibility(left, right, CompatibilityPolicy::exact())
}

/// Convenience helper for assignment compatibility.
#[must_use]
pub fn is_assignable<T: TypeShape>(actual: &T, expected: &T) -> Compatibility {
    check_compatibility(actual, expected, CompatibilityPolicy::assignable())
}

fn compare_views<'a, T: TypeShape>(
    actual: TypeView<'a, T>,
    expected: TypeView<'a, T>,
    mode: CompatibilityMode,
    strict_mutability: bool,
    worklist: &mut Vec<Obligation<'a, T>>,
) -> Compatibility {
    use Compatibility::{Compatible, Incompatible, Undetermined};
    use TypeView as V;

    // A tainted or unresolved type is not evidence of compatibility.
    match (actual, expected) {
        (V::Error, _) | (_, V::Error) | (V::Unresolved, _) | (_, V::Unresolved) => {
            return Undetermined;
        }
        (V::InferenceVariable(left), V::InferenceVariable(right)) if left == right => {
            return Compatible;
        }
        (V::InferenceVariable(_), _) | (_, V::InferenceVariable(_)) => {
            return Undetermined;
        }
        _ => {}
    }

    // The never type is assignable to every expected type because it cannot
    // produce a value. Exact equivalence still requires Never on both sides.
    if mode == CompatibilityMode::Assignable {
        if matches!(actual, V::Never) {
            return Compatible;
        }
        if matches!(expected, V::Never) {
            return Incompatible;
        }
    }

    match (actual, expected) {
        (V::Primitive(a), V::Primitive(e)) => {
            if a == e { Compatible } else { Incompatible }
        }

        (
            V::Nominal {
                identity: a,
                arguments: aa,
            },
            V::Nominal {
                identity: e,
                arguments: ea,
            },
        ) => {
            if a != e || aa.len() != ea.len() {
                return Incompatible;
            }
            enqueue_pairs(aa, ea, mode, worklist);
            Compatible
        }

        (V::Tuple(a), V::Tuple(e)) => {
            if a.len() != e.len() {
                return Incompatible;
            }
            enqueue_pairs(a, e, mode, worklist);
            Compatible
        }

        (
            V::Array {
                element: a,
                length: al,
            },
            V::Array {
                element: e,
                length: el,
            },
        ) => {
            if al != el {
                return Incompatible;
            }
            worklist.push(Obligation {
                actual: a,
                expected: e,
                mode,
            });
            Compatible
        }

        (V::Slice(a), V::Slice(e))
        | (V::Optional(a), V::Optional(e))
        | (V::Quantum(a), V::Quantum(e))
        | (V::Linear(a), V::Linear(e))
        | (V::Affine(a), V::Affine(e))
        | (V::Temporal(a), V::Temporal(e)) => {
            worklist.push(Obligation {
                actual: a,
                expected: e,
                mode,
            });
            Compatible
        }

        (
            V::Function {
                parameters: ap,
                return_type: ar,
            },
            V::Function {
                parameters: ep,
                return_type: er,
            },
        ) => {
            if ap.len() != ep.len() {
                return Incompatible;
            }

            match mode {
                CompatibilityMode::Exact => {
                    enqueue_pairs(ap, ep, mode, worklist);
                }
                CompatibilityMode::Assignable => {
                    // Function assignment is contravariant in parameters:
                    // a function accepting a broader input can stand in for
                    // one expected to accept a narrower input.
                    enqueue_pairs(ep, ap, mode, worklist);
                }
            }

            // Function results are covariant under assignment compatibility.
            worklist.push(Obligation {
                actual: ar,
                expected: er,
                mode,
            });
            Compatible
        }

        (
            V::Reference {
                mutable: am,
                inner: a,
            },
            V::Reference {
                mutable: em,
                inner: e,
            },
        ) => {
            if am != em && (strict_mutability || mode == CompatibilityMode::Exact) {
                return Incompatible;
            }

            // Mutable references are invariant. Immutable references may be
            // covariant only when the caller explicitly enables that policy.
            let child_mode = if am || em {
                CompatibilityMode::Exact
            } else {
                mode
            };

            worklist.push(Obligation {
                actual: a,
                expected: e,
                mode: child_mode,
            });
            Compatible
        }

        (
            V::Pointer {
                mutable: am,
                inner: a,
            },
            V::Pointer {
                mutable: em,
                inner: e,
            },
        ) => {
            if am != em {
                return Incompatible;
            }

            // Pointer variance is not inferred from reference variance.
            worklist.push(Obligation {
                actual: a,
                expected: e,
                mode: CompatibilityMode::Exact,
            });
            Compatible
        }

        (V::Result { ok: ao, error: ae }, V::Result { ok: eo, error: ee }) => {
            worklist.push(Obligation {
                actual: ao,
                expected: eo,
                mode,
            });
            worklist.push(Obligation {
                actual: ae,
                expected: ee,
                mode,
            });
            Compatible
        }

        (V::Never, V::Never) | (V::Unit, V::Unit) => Compatible,

        // A union or intersection may require normalization, subtyping, or
        // proof obligations. Do not invent those rules here. Identical
        // ordered structure can be checked conservatively; all other cases
        // are left for the subtype/constraint solver.
        (V::Union(a), V::Union(e)) | (V::Intersection(a), V::Intersection(e)) => {
            if a.len() != e.len() {
                return Incompatible;
            }
            enqueue_pairs(a, e, mode, worklist);
            Compatible
        }

        (
            V::Extension {
                namespace: an,
                name: ai,
                arguments: aa,
            },
            V::Extension {
                namespace: en,
                name: ei,
                arguments: ea,
            },
        ) => {
            if an != en || ai != ei || aa.len() != ea.len() {
                return Incompatible;
            }
            enqueue_pairs(aa, ea, mode, worklist);
            Compatible
        }

        // Distinct semantic constructors are not interchangeable merely
        // because their display strings or underlying payloads look similar.
        _ => Incompatible,
    }
}

fn enqueue_pairs<'a, T>(
    actual: &'a [T],
    expected: &'a [T],
    mode: CompatibilityMode,
    worklist: &mut Vec<Obligation<'a, T>>,
) {
    // Reverse insertion preserves left-to-right processing when popping from
    // the LIFO worklist. Compatibility is order-independent, but deterministic
    // processing improves reproducibility when instrumented by callers.
    for (actual, expected) in actual.iter().zip(expected.iter()).rev() {
        worklist.push(Obligation {
            actual,
            expected,
            mode,
        });
    }
}

// =============================================================================
// Diagnostics-facing helper
// =============================================================================

/// A stable classification for a failed or incomplete compatibility check.
///
/// The diagnostic layer owns user-facing messages and source labels; this
/// enum supplies machine-readable reasons without coupling this module to AST
/// spans or diagnostic storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompatibilityFailure {
    /// The types are known but not compatible.
    IncompatibleTypes,

    /// One or both types still require inference or resolution.
    UnresolvedType,

    /// Compatibility depends on a constraint or subtype proof.
    RequiresConstraintSolver,

    /// A conversion must be explicitly authorized by the coercion subsystem.
    RequiresExplicitCoercion,
}

impl fmt::Display for CompatibilityFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompatibleTypes => f.write_str("incompatible semantic types"),
            Self::UnresolvedType => f.write_str("semantic type is unresolved"),
            Self::RequiresConstraintSolver => {
                f.write_str("compatibility requires constraint or subtype solving")
            }
            Self::RequiresExplicitCoercion => {
                f.write_str("conversion requires explicit coercion authorization")
            }
        }
    }
}

/// Converts a compatibility result to a conservative failure category.
///
/// A compatible result has no failure category. This helper deliberately
/// does not manufacture source spans or diagnostic codes.
#[must_use]
pub const fn failure_for(
    result: Compatibility,
) -> Option<CompatibilityFailure> {
    match result {
        Compatibility::Compatible => None,
        Compatibility::Incompatible => Some(CompatibilityFailure::IncompatibleTypes),
        Compatibility::Undetermined => Some(CompatibilityFailure::UnresolvedType),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct TestType(TestKind);

    #[derive(Clone, Debug)]
    enum TestKind {
        Primitive(&'static str),
        Nominal(&'static str, Vec<TestType>),
        Tuple(Vec<TestType>),
        Array(Box<TestType>, ArrayLength<'static>),
        Slice(Box<TestType>),
        Function(Vec<TestType>, Box<TestType>),
        Reference(bool, Box<TestType>),
        Pointer(bool, Box<TestType>),
        Optional(Box<TestType>),
        Result(Box<TestType>, Box<TestType>),
        Never,
        Unit,
        Variable(u64),
        Union(Vec<TestType>),
        Intersection(Vec<TestType>),
        Quantum(Box<TestType>),
        Linear(Box<TestType>),
        Affine(Box<TestType>),
        Temporal(Box<TestType>),
        Unresolved,
        Error,
        Extension(&'static str, &'static str, Vec<TestType>),
    }

    impl TypeShape for TestType {
        fn type_view(&self) -> TypeView<'_, Self> {
            match &self.0 {
                TestKind::Primitive(name) => TypeView::Primitive(name),
                TestKind::Nominal(identity, arguments) => TypeView::Nominal {
                    identity,
                    arguments,
                },
                TestKind::Tuple(items) => TypeView::Tuple(items),
                TestKind::Array(element, length) => TypeView::Array {
                    element,
                    length: *length,
                },
                TestKind::Slice(inner) => TypeView::Slice(inner),
                TestKind::Function(parameters, result) => TypeView::Function {
                    parameters,
                    return_type: result,
                },
                TestKind::Reference(mutable, inner) => TypeView::Reference {
                    mutable: *mutable,
                    inner,
                },
                TestKind::Pointer(mutable, inner) => TypeView::Pointer {
                    mutable: *mutable,
                    inner,
                },
                TestKind::Optional(inner) => TypeView::Optional(inner),
                TestKind::Result(ok, error) => TypeView::Result { ok, error },
                TestKind::Never => TypeView::Never,
                TestKind::Unit => TypeView::Unit,
                TestKind::Variable(id) => TypeView::InferenceVariable(*id),
                TestKind::Union(items) => TypeView::Union(items),
                TestKind::Intersection(items) => TypeView::Intersection(items),
                TestKind::Quantum(inner) => TypeView::Quantum(inner),
                TestKind::Linear(inner) => TypeView::Linear(inner),
                TestKind::Affine(inner) => TypeView::Affine(inner),
                TestKind::Temporal(inner) => TypeView::Temporal(inner),
                TestKind::Unresolved => TypeView::Unresolved,
                TestKind::Error => TypeView::Error,
                TestKind::Extension(namespace, name, arguments) => TypeView::Extension {
                    namespace,
                    name,
                    arguments,
                },
            }
        }
    }

    fn primitive(name: &'static str) -> TestType {
        TestType(TestKind::Primitive(name))
    }

    fn named(name: &'static str, arguments: Vec<TestType>) -> TestType {
        TestType(TestKind::Nominal(name, arguments))
    }

    #[test]
    fn exact_primitive_identity_is_required() {
        assert_eq!(
            types_equal(&primitive("i32"), &primitive("i32")),
            Compatibility::Compatible
        );
        assert_eq!(
            types_equal(&primitive("i32"), &primitive("i64")),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn numeric_conversion_is_not_silently_accepted() {
        assert_eq!(
            is_assignable(&primitive("i32"), &primitive("i64")),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn nominal_identity_and_generic_arguments_are_checked() {
        let left = named("Container", vec![primitive("i32")]);
        let same = named("Container", vec![primitive("i32")]);
        let different_argument = named("Container", vec![primitive("i64")]);
        let different_identity = named("Other", vec![primitive("i32")]);

        assert_eq!(types_equal(&left, &same), Compatibility::Compatible);
        assert_eq!(
            types_equal(&left, &different_argument),
            Compatibility::Incompatible
        );
        assert_eq!(
            types_equal(&left, &different_identity),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn tuple_arity_and_order_are_semantic() {
        let pair = TestType(TestKind::Tuple(vec![
            primitive("i32"),
            primitive("bool"),
        ]));
        let reversed = TestType(TestKind::Tuple(vec![
            primitive("bool"),
            primitive("i32"),
        ]));
        let triple = TestType(TestKind::Tuple(vec![
            primitive("i32"),
            primitive("bool"),
            primitive("i32"),
        ]));

        assert_eq!(
            types_equal(&pair, &reversed),
            Compatibility::Incompatible
        );
        assert_eq!(
            types_equal(&pair, &triple),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn array_cardinality_is_not_discarded() {
        let four = TestType(TestKind::Array(
            Box::new(primitive("i32")),
            ArrayLength::Known("4"),
        ));
        let eight = TestType(TestKind::Array(
            Box::new(primitive("i32")),
            ArrayLength::Known("8"),
        ));
        let symbolic = TestType(TestKind::Array(
            Box::new(primitive("i32")),
            ArrayLength::Symbolic("N"),
        ));

        assert_eq!(
            types_equal(&four, &eight),
            Compatibility::Incompatible
        );
        assert_eq!(
            types_equal(&four, &symbolic),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn unresolved_and_error_types_do_not_prove_compatibility() {
        assert_eq!(
            is_assignable(
                &TestType(TestKind::Unresolved),
                &primitive("i32")
            ),
            Compatibility::Undetermined
        );
        assert_eq!(
            is_assignable(
                &TestType(TestKind::Error),
                &primitive("i32")
            ),
            Compatibility::Undetermined
        );
        assert_eq!(
            is_assignable(
                &TestType(TestKind::Variable(1)),
                &primitive("i32")
            ),
            Compatibility::Undetermined
        );
    }

    #[test]
    fn identical_inference_variables_are_compatible_with_themselves() {
        let left = TestType(TestKind::Variable(7));
        let right = TestType(TestKind::Variable(7));

        assert_eq!(types_equal(&left, &right), Compatibility::Compatible);
        assert_eq!(
            types_equal(
                &left,
                &TestType(TestKind::Variable(8))
            ),
            Compatibility::Undetermined
        );
    }

    #[test]
    fn never_is_assignable_but_not_equivalent_to_other_types() {
        let never = TestType(TestKind::Never);
        let integer = primitive("i32");

        assert_eq!(
            is_assignable(&never, &integer),
            Compatibility::Compatible
        );
        assert_eq!(
            types_equal(&never, &integer),
            Compatibility::Incompatible
        );
        assert_eq!(
            is_assignable(&integer, &never),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn mutable_references_are_invariant_by_default() {
        let mutable_i32 = TestType(TestKind::Reference(
            true,
            Box::new(primitive("i32")),
        ));
        let immutable_i32 = TestType(TestKind::Reference(
            false,
            Box::new(primitive("i32")),
        ));

        assert_eq!(
            is_assignable(&mutable_i32, &immutable_i32),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn function_parameter_variance_is_reversed_for_assignment() {
        let accepts_i32 = TestType(TestKind::Function(
            vec![primitive("i32")],
            Box::new(primitive("bool")),
        ));
        let accepts_i64 = TestType(TestKind::Function(
            vec![primitive("i64")],
            Box::new(primitive("bool")),
        ));

        // With invariant primitive types, neither function is assignable to
        // the other. A future subtype relation can be added explicitly.
        assert_eq!(
            is_assignable(&accepts_i32, &accepts_i64),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn empty_and_nonempty_generic_arguments_are_distinguished() {
        let marker = named("Marker", vec![]);
        let parameterized = named("Marker", vec![primitive("i32")]);

        assert_eq!(
            types_equal(&marker, &parameterized),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn extension_types_require_matching_qualified_identity() {
        let first = TestType(TestKind::Extension(
            "domain.alpha",
            "Vector",
            vec![primitive("i32")],
        ));
        let same = TestType(TestKind::Extension(
            "domain.alpha",
            "Vector",
            vec![primitive("i32")],
        ));
        let different_namespace = TestType(TestKind::Extension(
            "domain.beta",
            "Vector",
            vec![primitive("i32")],
        ));

        assert_eq!(types_equal(&first, &same), Compatibility::Compatible);
        assert_eq!(
            types_equal(&first, &different_namespace),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn deep_types_are_compared_iteratively() {
        let mut left = primitive("Base");
        let mut right = primitive("Base");

        for _ in 0..20_000 {
            left = TestType(TestKind::Optional(Box::new(left)));
            right = TestType(TestKind::Optional(Box::new(right)));
        }

        assert_eq!(types_equal(&left, &right), Compatibility::Compatible);
    }

    #[test]
    fn domain_qualifiers_are_not_erased() {
        let quantum = TestType(TestKind::Quantum(Box::new(primitive("Q"))));
        let classical = primitive("Q");

        assert_eq!(
            types_equal(&quantum, &classical),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn resource_qualifiers_remain_distinct() {
        let linear = TestType(TestKind::Linear(Box::new(primitive("Resource"))));
        let affine = TestType(TestKind::Affine(Box::new(primitive("Resource"))));

        assert_eq!(
            types_equal(&linear, &affine),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn result_success_and_error_types_are_both_checked() {
        let left = TestType(TestKind::Result(
            Box::new(primitive("i32")),
            Box::new(primitive("Text")),
        ));
        let wrong_error = TestType(TestKind::Result(
            Box::new(primitive("i32")),
            Box::new(primitive("NetworkError")),
        ));

        assert_eq!(
            types_equal(&left, &wrong_error),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn diagnostics_mapping_is_machine_readable() {
        assert_eq!(
            failure_for(Compatibility::Compatible),
            None
        );
        assert_eq!(
            failure_for(Compatibility::Incompatible),
            Some(CompatibilityFailure::IncompatibleTypes)
        );
        assert_eq!(
            failure_for(Compatibility::Undetermined),
            Some(CompatibilityFailure::UnresolvedType)
        );
    }
}
