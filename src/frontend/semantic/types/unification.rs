
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani frontend semantic analysis — type unification.
//!
//! Path: `src/frontend/semantic/types/unification.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Solve type equality constraints over the canonical `SemanticType` model.
//! This module is independent of the AST, parser, symbol table, hardware,
//! and backend implementations.
//!
//! # Ownership
//!
//! Owns:
//! - inference-variable substitutions;
//! - type-variable dereferencing;
//! - structural type unification;
//! - occurs checking;
//! - transactional rollback on failed unification;
//! - iterative substitution application.
//!
//! Does not own:
//! - AST `TypeExpr` resolution;
//! - variable allocation policy;
//! - generic parameter instantiation;
//! - trait or overload resolution;
//! - effect, capability, resource, or contract checking;
//! - symbolic arithmetic or dependent-type proof solving;
//! - target feasibility or IR lowering.
//!
//! # Integration
//!
//! `types/ty.rs` is the authoritative semantic type representation.
//! `types/inference.rs` should call `TypeUnifier::unify` through the
//! inference-session integration layer.
//! `types/constraints.rs` should handle unresolved symbolic obligations.
//! `types/generics.rs` owns generic instantiation and substitution.
//!
//! # Important invariants
//!
//! - A failed `unify` call leaves the substitution environment unchanged.
//! - A type variable cannot be bound to a type containing itself.
//! - `SemanticType::Error` is not a wildcard and cannot prove compatibility.
//! - Unresolved symbolic dimensions are not assumed equal.
//! - No hardware capacities or fixed domain inventories are encoded here.
//! - Traversal uses explicit work stacks rather than recursive descent.
//! - All ordering and diagnostics are deterministic.
//!
//! A finite process cannot promise unlimited execution. This implementation
//! imposes no language-level hardware ceiling; actual memory and time remain
//! subject to the host environment and any configured compiler budgets.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::ty::{
    Cardinality, FloatFormat, FunctionParameter, IntegerWidth, PrimitiveType,
    QualifiedName, SemanticType, TypeAttribute, TypeLevelValue, TypeParameter,
    TypeVariableId,
};

/// Failure encountered while solving type constraints.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnificationError {
    /// Two semantic types have incompatible structures.
    TypeMismatch {
        left: SemanticType,
        right: SemanticType,
    },

    /// A type variable occurs inside the type it would be bound to.
    OccursCheck {
        variable: TypeVariableId,
        within: SemanticType,
    },

    /// Two constructors have different numbers of arguments.
    ArityMismatch {
        constructor: &'static str,
        left: usize,
        right: usize,
    },

    /// Named types have different qualified names or nominal identities.
    NamedTypeMismatch {
        left: QualifiedName,
        right: QualifiedName,
    },

    /// A symbolic cardinality equality needs the constraint solver.
    UnresolvedCardinalityConstraint {
        left: Cardinality,
        right: Cardinality,
    },

    /// Cardinalities are known to be incompatible.
    CardinalityMismatch {
        left: Cardinality,
        right: Cardinality,
    },

    /// A recursive or inconsistent substitution chain was detected.
    InvalidSubstitutionCycle {
        variable: TypeVariableId,
    },

    /// Error-recovery types cannot establish a valid type equality.
    ErrorTypeParticipates,

    /// A type structure exceeded an explicitly configured operational budget.
    WorkBudgetExceeded {
        budget: usize,
    },

    /// The caller supplied an invalid operational budget.
    InvalidConfiguration,
}

impl fmt::Display for UnificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch { left, right } => {
                write!(f, "cannot unify `{left}` with `{right}`")
            }
            Self::OccursCheck { variable, within } => {
                write!(
                    f,
                    "type variable ?{} occurs in `{within}`",
                    variable.0
                )
            }
            Self::ArityMismatch {
                constructor,
                left,
                right,
            } => write!(
                f,
                "{constructor} arity mismatch: {left} versus {right}"
            ),
            Self::NamedTypeMismatch { left, right } => {
                write!(f, "named type identity mismatch: `{left}` versus `{right}`")
            }
            Self::UnresolvedCardinalityConstraint { left, right } => {
                write!(
                    f,
                    "cardinality equality remains unresolved: `{left}` versus `{right}`"
                )
            }
            Self::CardinalityMismatch { left, right } => {
                write!(
                    f,
                    "incompatible cardinalities: `{left}` versus `{right}`"
                )
            }
            Self::InvalidSubstitutionCycle { variable } => {
                write!(f, "invalid substitution cycle involving ?{}", variable.0)
            }
            Self::ErrorTypeParticipates => {
                f.write_str("an error-recovery type cannot establish type equality")
            }
            Self::WorkBudgetExceeded { budget } => {
                write!(f, "unification work budget exceeded ({budget})")
            }
            Self::InvalidConfiguration => {
                f.write_str("invalid unification configuration")
            }
        }
    }
}

impl Error for UnificationError {}

/// Optional operational protection for untrusted or generated type structures.
///
/// `max_work = None` means no explicit work limit. Production callers
/// processing untrusted input should normally set this from semantic
/// configuration. A budget failure is an incomplete analysis, not a
/// successful type check.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnificationConfig {
    pub max_work: Option<usize>,
}

/// Result of a successful unification.
///
/// `ty` is the fully substituted form of the unified type at the time of
/// return. The substitution environment remains owned by the unifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnificationResult {
    pub ty: SemanticType,
}

/// Transactional unification environment.
///
/// Create one per semantic inference session. Do not share it across
/// concurrent analyses without an explicit synchronization and isolation
/// design.
#[derive(Clone, Debug, Default)]
pub struct TypeUnifier {
    substitutions: BTreeMap<TypeVariableId, SemanticType>,
    config: UnificationConfig,
}

impl TypeUnifier {
    /// Creates a unifier with no explicit work budget.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a unifier with an explicit operational policy.
    pub fn with_config(config: UnificationConfig) -> Result<Self, UnificationError> {
        if config.max_work == Some(0) {
            return Err(UnificationError::InvalidConfiguration);
        }

        Ok(Self {
            substitutions: BTreeMap::new(),
            config,
        })
    }

    /// Returns the configured work budget.
    #[must_use]
    pub fn config(&self) -> UnificationConfig {
        self.config
    }

    /// Returns the current substitution for a variable, if one exists.
    ///
    /// This returns the direct binding, not necessarily a fully normalized
    /// type. Use `resolve` when the fully substituted form is required.
    #[must_use]
    pub fn direct_binding(
        &self,
        variable: TypeVariableId,
    ) -> Option<&SemanticType> {
        self.substitutions.get(&variable)
    }

    /// Returns the number of currently bound variables.
    #[must_use]
    pub fn binding_count(&self) -> usize {
        self.substitutions.len()
    }

    /// Returns whether a variable currently has a binding.
    #[must_use]
    pub fn is_bound(&self, variable: TypeVariableId) -> bool {
        self.substitutions.contains_key(&variable)
    }

    /// Exposes the substitution environment for diagnostics and integration.
    ///
    /// Callers must not mutate this map. All changes must go through this
    /// unifier so occurs checks and transactional behavior remain enforced.
    #[must_use]
    pub fn substitutions(&self) -> &BTreeMap<TypeVariableId, SemanticType> {
        &self.substitutions
    }

    /// Unifies two types atomically.
    ///
    /// If any constraint fails, every binding introduced by this call is
    /// removed and the pre-call environment is preserved.
    pub fn unify(
        &mut self,
        left: &SemanticType,
        right: &SemanticType,
    ) -> Result<SemanticType, UnificationError> {
        let mut inserted = Vec::new();
        let mut work = vec![(left.clone(), right.clone())];
        let mut work_count = 0usize;

        while let Some((left, right)) = work.pop() {
            self.charge_work(&mut work_count)?;

            let left = self.dereference(&left)?;
            let right = self.dereference(&right)?;

            if left == right {
                if matches!(left, SemanticType::Error) {
                    self.rollback(&inserted);
                    return Err(UnificationError::ErrorTypeParticipates);
                }
                continue;
            }

            if matches!(left, SemanticType::Error)
                || matches!(right, SemanticType::Error)
            {
                self.rollback(&inserted);
                return Err(UnificationError::ErrorTypeParticipates);
            }

            match (left, right) {
                (SemanticType::Variable(variable), ty)
                | (ty, SemanticType::Variable(variable)) => {
                    if let Err(error) =
                        self.bind(variable, ty, &mut inserted, &mut work_count)
                    {
                        self.rollback(&inserted);
                        return Err(error);
                    }
                }

                (
                    SemanticType::Primitive(left),
                    SemanticType::Primitive(right),
                ) => {
                    if let Err(error) = unify_primitive(
                        left,
                        right,
                        &mut work,
                    ) {
                        self.rollback(&inserted);
                        return Err(error);
                    }
                }

                (
                    SemanticType::Named {
                        name: left_name,
                        nominal_id: left_id,
                        arguments: left_args,
                    },
                    SemanticType::Named {
                        name: right_name,
                        nominal_id: right_id,
                        arguments: right_args,
                    },
                ) => {
                    if !same_named_identity(
                        &left_name,
                        &left_id,
                        &right_name,
                        &right_id,
                    ) {
                        self.rollback(&inserted);
                        return Err(UnificationError::NamedTypeMismatch {
                            left: left_name,
                            right: right_name,
                        });
                    }

                    if let Err(error) = push_pairs(
                        "named type",
                        left_args,
                        right_args,
                        &mut work,
                    ) {
                        self.rollback(&inserted);
                        return Err(error);
                    }
                }

                (
                    SemanticType::Parameter(left),
                    SemanticType::Parameter(right),
                ) if left == right => {}

                (SemanticType::Tuple(left), SemanticType::Tuple(right)) => {
                    if let Err(error) =
                        push_pairs("tuple", left, right, &mut work)
                    {
                        self.rollback(&inserted);
                        return Err(error);
                    }
                }

                (
                    SemanticType::Array {
                        element: left_element,
                        cardinality: left_cardinality,
                    },
                    SemanticType::Array {
                        element: right_element,
                        cardinality: right_cardinality,
                    },
                ) => {
                    if let Err(error) =
                        unify_cardinality(&left_cardinality, &right_cardinality)
                    {
                        self.rollback(&inserted);
                        return Err(error);
                    }
                    work.push((*left_element, *right_element));
                }

                (SemanticType::Slice(left), SemanticType::Slice(right))
                | (SemanticType::Optional(left), SemanticType::Optional(right))
                | (SemanticType::Quantum(left), SemanticType::Quantum(right))
                | (SemanticType::Linear(left), SemanticType::Linear(right))
                | (SemanticType::Affine(left), SemanticType::Affine(right))
                | (SemanticType::Temporal(left), SemanticType::Temporal(right)) => {
                    work.push((*left, *right));
                }

                (
                    SemanticType::Function {
                        parameters: left_parameters,
                        result: left_result,
                        effects: left_effects,
                        is_async: left_async,
                    },
                    SemanticType::Function {
                        parameters: right_parameters,
                        result: right_result,
                        effects: right_effects,
                        is_async: right_async,
                    },
                ) => {
                    if left_async != right_async
                        || !same_effect_set(&left_effects, &right_effects)
                    {
                        self.rollback(&inserted);
                        return Err(type_mismatch(
                            SemanticType::Function {
                                parameters: left_parameters,
                                result: left_result,
                                effects: left_effects,
                                is_async: left_async,
                            },
                            SemanticType::Function {
                                parameters: right_parameters,
                                result: right_result,
                                effects: right_effects,
                                is_async: right_async,
                            },
                        ));
                    }

                    if left_parameters.len() != right_parameters.len() {
                        self.rollback(&inserted);
                        return Err(UnificationError::ArityMismatch {
                            constructor: "function parameters",
                            left: left_parameters.len(),
                            right: right_parameters.len(),
                        });
                    }

                    for (left, right) in
                        left_parameters.into_iter().zip(right_parameters)
                    {
                        if left.mode != right.mode {
                            self.rollback(&inserted);
                            return Err(type_mismatch(
                                SemanticType::Named {
                                    name: QualifiedName::single("parameter-mode"),
                                    nominal_id: None,
                                    arguments: Vec::new(),
                                },
                                SemanticType::Named {
                                    name: QualifiedName::single("parameter-mode"),
                                    nominal_id: None,
                                    arguments: Vec::new(),
                                },
                            ));
                        }

                        work.push((left.ty, right.ty));
                    }

                    work.push((*left_result, *right_result));
                }

                (
                    SemanticType::Reference {
                        mutable: left_mutable,
                        lifetime: left_lifetime,
                        referent: left_referent,
                    },
                    SemanticType::Reference {
                        mutable: right_mutable,
                        lifetime: right_lifetime,
                        referent: right_referent,
                    },
                ) if left_mutable == right_mutable
                    && left_lifetime == right_lifetime =>
                {
                    work.push((*left_referent, *right_referent));
                }

                (
                    SemanticType::Pointer {
                        mutable: left_mutable,
                        pointee: left_pointee,
                    },
                    SemanticType::Pointer {
                        mutable: right_mutable,
                        pointee: right_pointee,
                    },
                ) if left_mutable == right_mutable => {
                    work.push((*left_pointee, *right_pointee));
                }

                (
                    SemanticType::Result {
                        ok: left_ok,
                        error: left_error,
                    },
                    SemanticType::Result {
                        ok: right_ok,
                        error: right_error,
                    },
                ) => {
                    work.push((*left_ok, *right_ok));
                    work.push((*left_error, *right_error));
                }

                (SemanticType::Union(left), SemanticType::Union(right)) => {
                    if let Err(error) =
                        push_pairs("union", left, right, &mut work)
                    {
                        self.rollback(&inserted);
                        return Err(error);
                    }
                }

                (
                    SemanticType::Intersection(left),
                    SemanticType::Intersection(right),
                ) => {
                    if let Err(error) =
                        push_pairs("intersection", left, right, &mut work)
                    {
                        self.rollback(&inserted);
                        return Err(error);
                    }
                }

                (SemanticType::Never, SemanticType::Never)
                | (SemanticType::Unit, SemanticType::Unit)
                | (SemanticType::SelfType, SemanticType::SelfType) => {}

                (
                    SemanticType::Pi {
                        parameter: left_parameter,
                        parameter_type: left_parameter_type,
                        body: left_body,
                    },
                    SemanticType::Pi {
                        parameter: right_parameter,
                        parameter_type: right_parameter_type,
                        body: right_body,
                    },
                )
                | (
                    SemanticType::Sigma {
                        parameter: left_parameter,
                        parameter_type: left_parameter_type,
                        body: left_body,
                    },
                    SemanticType::Sigma {
                        parameter: right_parameter,
                        parameter_type: right_parameter_type,
                        body: right_body,
                    },
                ) => {
                    if left_parameter != right_parameter {
                        self.rollback(&inserted);
                        return Err(type_mismatch(
                            SemanticType::Parameter(left_parameter),
                            SemanticType::Parameter(right_parameter),
                        ));
                    }
                    work.push((*left_parameter_type, *right_parameter_type));
                    work.push((*left_body, *right_body));
                }

                (
                    SemanticType::Identity {
                        left: left_a,
                        right: left_b,
                    },
                    SemanticType::Identity {
                        left: right_a,
                        right: right_b,
                    },
                ) => {
                    work.push((*left_a, *right_a));
                    work.push((*left_b, *right_b));
                }

                (
                    SemanticType::Associated {
                        base: left_base,
                        member: left_member,
                    },
                    SemanticType::Associated {
                        base: right_base,
                        member: right_member,
                    },
                ) if left_member == right_member => {
                    work.push((*left_base, *right_base));
                }

                (
                    SemanticType::TypeApplication {
                        constructor: left_constructor,
                        arguments: left_arguments,
                    },
                    SemanticType::TypeApplication {
                        constructor: right_constructor,
                        arguments: right_arguments,
                    },
                ) => {
                    if let Err(error) = push_pairs(
                        "type application",
                        left_arguments,
                        right_arguments,
                        &mut work,
                    ) {
                        self.rollback(&inserted);
                        return Err(error);
                    }
                    work.push((*left_constructor, *right_constructor));
                }

                (
                    SemanticType::Extension {
                        namespace: left_namespace,
                        name: left_name,
                        arguments: left_arguments,
                        attributes: left_attributes,
                    },
                    SemanticType::Extension {
                        namespace: right_namespace,
                        name: right_name,
                        arguments: right_arguments,
                        attributes: right_attributes,
                    },
                ) => {
                    if left_namespace != right_namespace
                        || left_name != right_name
                        || left_attributes != right_attributes
                    {
                        self.rollback(&inserted);
                        return Err(type_mismatch(
                            SemanticType::Extension {
                                namespace: left_namespace,
                                name: left_name,
                                arguments: left_arguments,
                                attributes: left_attributes,
                            },
                            SemanticType::Extension {
                                namespace: right_namespace,
                                name: right_name,
                                arguments: right_arguments,
                                attributes: right_attributes,
                            },
                        ));
                    }

                    if let Err(error) = push_pairs(
                        "extension type",
                        left_arguments,
                        right_arguments,
                        &mut work,
                    ) {
                        self.rollback(&inserted);
                        return Err(error);
                    }
                }

                (left, right) => {
                    self.rollback(&inserted);
                    return Err(type_mismatch(left, right));
                }
            }
        }

        match self.resolve(left) {
            Ok(ty) => Ok(ty),
            Err(error) => {
                self.rollback(&inserted);
                Err(error)
            }
        }
    }

    /// Resolves all currently known substitutions in a type.
    ///
    /// The traversal is iterative. Unbound variables remain variables.
    pub fn resolve(
        &self,
        ty: &SemanticType,
    ) -> Result<SemanticType, UnificationError> {
        let mut frames: Vec<(SemanticType, bool, usize)> = vec![(ty.clone(), false, 0)];
        let mut values: Vec<SemanticType> = Vec::new();
        let mut work_count = 0usize;

        while let Some((current, exiting, child_count)) = frames.pop() {
            self.charge_work(&mut work_count)?;

            if !exiting {
                if let SemanticType::Variable(variable) = current {
                    match self.dereference(&SemanticType::Variable(variable))? {
                        SemanticType::Variable(unbound) => {
                            values.push(SemanticType::Variable(unbound));
                            continue;
                        }
                        resolved => {
                            frames.push((resolved, false, 0));
                            continue;
                        }
                    }
                }

                let children = type_children(&current);
                frames.push((current, true, children.len()));

                for child in children.into_iter().rev() {
                    frames.push((child, false, 0));
                }
            } else {
                if values.len() < child_count {
                    return Err(UnificationError::InvalidConfiguration);
                }

                let split = values.len() - child_count;
                let children = values.split_off(split);
                values.push(rebuild_type(current, children)?);
            }
        }

        if values.len() != 1 {
            return Err(UnificationError::InvalidConfiguration);
        }

        Ok(values.pop().expect("exactly one resolved type"))
    }

    /// Removes every substitution from this inference session.
    pub fn clear(&mut self) {
        self.substitutions.clear();
    }

    fn dereference(
        &self,
        ty: &SemanticType,
    ) -> Result<SemanticType, UnificationError> {
        let mut current = ty.clone();
        let mut visited = BTreeSet::new();

        loop {
            let SemanticType::Variable(variable) = current else {
                return Ok(current);
            };

            let Some(next) = self.substitutions.get(&variable) else {
                return Ok(SemanticType::Variable(variable));
            };

            if !visited.insert(variable) {
                return Err(UnificationError::InvalidSubstitutionCycle { variable });
            }

            current = next.clone();
        }
    }

    fn bind(
        &mut self,
        variable: TypeVariableId,
        ty: SemanticType,
        inserted: &mut Vec<TypeVariableId>,
        work_count: &mut usize,
    ) -> Result<(), UnificationError> {
        let ty = self.dereference(&ty)?;

        if ty == SemanticType::Variable(variable) {
            return Ok(());
        }

        if let Some(existing) = self.substitutions.get(&variable) {
            return self.unify_existing_binding(variable, existing.clone(), ty, work_count);
        }

        if let Some(found) = self.occurs(variable, &ty, work_count)? {
            return Err(UnificationError::OccursCheck {
                variable,
                within: found,
            });
        }

        self.substitutions.insert(variable, ty);
        inserted.push(variable);
        Ok(())
    }

    fn unify_existing_binding(
        &mut self,
        variable: TypeVariableId,
        existing: SemanticType,
        incoming: SemanticType,
        _work_count: &mut usize,
    ) -> Result<(), UnificationError> {
        let resolved = self.dereference(&SemanticType::Variable(variable))?;
        if resolved == existing {
            if existing == incoming {
                Ok(())
            } else {
                Err(type_mismatch(existing, incoming))
            }
        } else {
            Err(type_mismatch(existing, incoming))
        }
    }

    fn occurs(
        &self,
        needle: TypeVariableId,
        ty: &SemanticType,
        work_count: &mut usize,
    ) -> Result<Option<SemanticType>, UnificationError> {
        let mut stack = vec![ty.clone()];
        let mut visited_variables = BTreeSet::new();

        while let Some(current) = stack.pop() {
            self.charge_work(work_count)?;

            match current {
                SemanticType::Variable(variable) => {
                    if variable == needle {
                        return Ok(Some(SemanticType::Variable(variable)));
                    }

                    if !visited_variables.insert(variable) {
                        continue;
                    }

                    if let Some(bound) = self.substitutions.get(&variable) {
                        stack.push(bound.clone());
                    }
                }
                other => stack.extend(type_children(&other)),
            }
        }

        Ok(None)
    }

    fn rollback(&mut self, inserted: &[TypeVariableId]) {
        for variable in inserted.iter().rev() {
            self.substitutions.remove(variable);
        }
    }

    fn charge_work(&self, work_count: &mut usize) -> Result<(), UnificationError> {
        *work_count = work_count
            .checked_add(1)
            .ok_or(UnificationError::WorkBudgetExceeded {
                budget: self.config.max_work.unwrap_or(usize::MAX),
            })?;

        if let Some(budget) = self.config.max_work {
            if *work_count > budget {
                return Err(UnificationError::WorkBudgetExceeded { budget });
            }
        }

        Ok(())
    }
}

fn type_mismatch(left: SemanticType, right: SemanticType) -> UnificationError {
    UnificationError::TypeMismatch { left, right }
}

fn same_named_identity(
    left_name: &QualifiedName,
    left_id: &Option<QualifiedName>,
    right_name: &QualifiedName,
    right_id: &Option<QualifiedName>,
) -> bool {
    match (left_id, right_id) {
        (Some(left), Some(right)) => left == right,
        (None, None) => left_name == right_name,
        // Do not silently equate a resolved nominal identity with an
        // unresolved spelling: the declaration resolver must settle identity.
        _ => false,
    }
}

fn same_effect_set(
    left: &[QualifiedName],
    right: &[QualifiedName],
) -> bool {
    let left: BTreeSet<_> = left.iter().collect();
    let right: BTreeSet<_> = right.iter().collect();
    left == right
}

fn push_pairs(
    constructor: &'static str,
    left: Vec<SemanticType>,
    right: Vec<SemanticType>,
    work: &mut Vec<(SemanticType, SemanticType)>,
) -> Result<(), UnificationError> {
    if left.len() != right.len() {
        return Err(UnificationError::ArityMismatch {
            constructor,
            left: left.len(),
            right: right.len(),
        });
    }

    work.extend(left.into_iter().zip(right));
    Ok(())
}

fn unify_cardinality(
    left: &Cardinality,
    right: &Cardinality,
) -> Result<(), UnificationError> {
    if left == right {
        return Ok(());
    }

    match (left, right) {
        (Cardinality::Unspecified, Cardinality::Unspecified) => Ok(()),
        (Cardinality::Known(_), Cardinality::Known(_)) => {
            Err(UnificationError::CardinalityMismatch {
                left: left.clone(),
                right: right.clone(),
            })
        }
        _ => Err(UnificationError::UnresolvedCardinalityConstraint {
            left: left.clone(),
            right: right.clone(),
        }),
    }
}

fn unify_primitive(
    left: PrimitiveType,
    right: PrimitiveType,
    work: &mut Vec<(SemanticType, SemanticType)>,
) -> Result<(), UnificationError> {
    match (left, right) {
        (PrimitiveType::Unit, PrimitiveType::Unit)
        | (PrimitiveType::Boolean, PrimitiveType::Boolean)
        | (PrimitiveType::Character, PrimitiveType::Character)
        | (PrimitiveType::String, PrimitiveType::String)
        | (PrimitiveType::StringSlice, PrimitiveType::StringSlice) => Ok(()),

        (
            PrimitiveType::Integer {
                signedness: left_sign,
                width: left_width,
            },
            PrimitiveType::Integer {
                signedness: right_sign,
                width: right_width,
            },
        ) if left_sign == right_sign && left_width == right_width => Ok(()),

        (PrimitiveType::Float(left), PrimitiveType::Float(right)) => {
            unify_float_format(left, right, work)
        }

        (PrimitiveType::Complex(left), PrimitiveType::Complex(right)) => {
            work.push((*left, *right));
            Ok(())
        }

        (left, right) => Err(type_mismatch(
            SemanticType::Primitive(left),
            SemanticType::Primitive(right),
        )),
    }
}

fn unify_float_format(
    left: FloatFormat,
    right: FloatFormat,
    work: &mut Vec<(SemanticType, SemanticType)>,
) -> Result<(), UnificationError> {
    match (left, right) {
        (FloatFormat::Binary16, FloatFormat::Binary16)
        | (FloatFormat::BFloat16, FloatFormat::BFloat16)
        | (FloatFormat::Binary32, FloatFormat::Binary32)
        | (FloatFormat::Binary64, FloatFormat::Binary64)
        | (FloatFormat::Binary128, FloatFormat::Binary128) => Ok(()),

        (
            FloatFormat::Decimal {
                precision: left_precision,
            },
            FloatFormat::Decimal {
                precision: right_precision,
            },
        ) if left_precision == right_precision => Ok(()),

        (
            FloatFormat::Extension {
                name: left_name,
                parameters: left_parameters,
            },
            FloatFormat::Extension {
                name: right_name,
                parameters: right_parameters,
            },
        ) if left_name == right_name => {
            push_pairs(
                "floating-point extension",
                left_parameters,
                right_parameters,
                work,
            )
        }

        (left, right) => Err(type_mismatch(
            SemanticType::Primitive(PrimitiveType::Float(left)),
            SemanticType::Primitive(PrimitiveType::Float(right)),
        )),
    }
}

/// Extracts immediate semantic-type children without recursive descent.
fn type_children(ty: &SemanticType) -> Vec<SemanticType> {
    match ty {
        SemanticType::Primitive(PrimitiveType::Complex(inner)) => {
            vec![(**inner).clone()]
        }
        SemanticType::Primitive(PrimitiveType::Float(
            FloatFormat::Extension { parameters, .. },
        )) => parameters.clone(),

        SemanticType::Named { arguments, .. }
        | SemanticType::Tuple(arguments)
        | SemanticType::Union(arguments)
        | SemanticType::Intersection(arguments)
        | SemanticType::TypeApplication {
            arguments,
            constructor: _,
        } => {
            if let SemanticType::TypeApplication { constructor, .. } = ty {
                let mut result = vec![(**constructor).clone()];
                result.extend(arguments.clone());
                result
            } else {
                arguments.clone()
            }
        }

        SemanticType::Array { element, .. }
        | SemanticType::Slice(element)
        | SemanticType::Optional(element)
        | SemanticType::Quantum(element)
        | SemanticType::Linear(element)
        | SemanticType::Affine(element)
        | SemanticType::Temporal(element) => vec![(**element).clone()],

        SemanticType::Function {
            parameters,
            result,
            ..
        } => {
            let mut children: Vec<_> =
                parameters.iter().map(|parameter| parameter.ty.clone()).collect();
            children.push((**result).clone());
            children
        }

        SemanticType::Reference { referent, .. } => vec![(**referent).clone()],
        SemanticType::Pointer { pointee, .. } => vec![(**pointee).clone()],

        SemanticType::Result { ok, error } => {
            vec![(**ok).clone(), (**error).clone()]
        }

        SemanticType::Pi {
            parameter_type,
            body,
            ..
        }
        | SemanticType::Sigma {
            parameter_type,
            body,
            ..
        } => vec![(**parameter_type).clone(), (**body).clone()],

        SemanticType::Identity { left, right } => {
            vec![(**left).clone(), (**right).clone()]
        }

        SemanticType::Associated { base, .. } => vec![(**base).clone()],

        SemanticType::Extension { arguments, .. } => arguments.clone(),

        SemanticType::Primitive(_)
        | SemanticType::Parameter(_)
        | SemanticType::Variable(_)
        | SemanticType::Never
        | SemanticType::Unit
        | SemanticType::SelfType
        | SemanticType::Error => Vec::new(),
    }
}

/// Rebuilds a type after its children have been resolved.
///
/// `children` are supplied in the same order returned by `type_children`.
fn rebuild_type(
    ty: SemanticType,
    children: Vec<SemanticType>,
) -> Result<SemanticType, UnificationError> {
    let mut children = children.into_iter();

    let next = |children: &mut std::vec::IntoIter<SemanticType>| {
        children.next().ok_or(UnificationError::InvalidConfiguration)
    };

    let rebuilt = match ty {
        SemanticType::Primitive(PrimitiveType::Complex(_)) => {
            SemanticType::Primitive(PrimitiveType::Complex(Box::new(next(&mut children)?)))
        }

        SemanticType::Primitive(PrimitiveType::Float(
            FloatFormat::Extension { name, .. },
        )) => {
            let parameters = children.collect();
            SemanticType::Primitive(PrimitiveType::Float(FloatFormat::Extension {
                name,
                parameters,
            }))
        }

        SemanticType::Named {
            name,
            nominal_id,
            arguments: _,
        } => SemanticType::Named {
            name,
            nominal_id,
            arguments: children.collect(),
        },

        SemanticType::Tuple(_) => SemanticType::Tuple(children.collect()),

        SemanticType::Array { cardinality, .. } => SemanticType::Array {
            element: Box::new(next(&mut children)?),
            cardinality,
        },

        SemanticType::Slice(_) => SemanticType::Slice(Box::new(next(&mut children)?)),
        SemanticType::Optional(_) => {
            SemanticType::Optional(Box::new(next(&mut children)?))
        }
        SemanticType::Quantum(_) => {
            SemanticType::Quantum(Box::new(next(&mut children)?))
        }
        SemanticType::Linear(_) => {
            SemanticType::Linear(Box::new(next(&mut children)?))
        }
        SemanticType::Affine(_) => {
            SemanticType::Affine(Box::new(next(&mut children)?))
        }
        SemanticType::Temporal(_) => {
            SemanticType::Temporal(Box::new(next(&mut children)?))
        }

        SemanticType::Function {
            parameters,
            effects,
            is_async,
            ..
        } => {
            let mut resolved_parameters = Vec::with_capacity(parameters.len());

            for parameter in parameters {
                resolved_parameters.push(FunctionParameter {
                    name: parameter.name,
                    ty: next(&mut children)?,
                    mode: parameter.mode,
                });
            }

            let result = next(&mut children)?;
            SemanticType::Function {
                parameters: resolved_parameters,
                result: Box::new(result),
                effects,
                is_async,
            }
        }

        SemanticType::Reference {
            mutable, lifetime, ..
        } => SemanticType::Reference {
            mutable,
            lifetime,
            referent: Box::new(next(&mut children)?),
        },

        SemanticType::Pointer { mutable, .. } => SemanticType::Pointer {
            mutable,
            pointee: Box::new(next(&mut children)?),
        },

        SemanticType::Result { .. } => SemanticType::Result {
            ok: Box::new(next(&mut children)?),
            error: Box::new(next(&mut children)?),
        },

        SemanticType::Union(_) => SemanticType::Union(children.collect()),
        SemanticType::Intersection(_) => SemanticType::Intersection(children.collect()),

        SemanticType::Pi { parameter, .. } => SemanticType::Pi {
            parameter,
            parameter_type: Box::new(next(&mut children)?),
            body: Box::new(next(&mut children)?),
        },

        SemanticType::Sigma { parameter, .. } => SemanticType::Sigma {
            parameter,
            parameter_type: Box::new(next(&mut children)?),
            body: Box::new(next(&mut children)?),
        },

        SemanticType::Identity { .. } => SemanticType::Identity {
            left: Box::new(next(&mut children)?),
            right: Box::new(next(&mut children)?),
        },

        SemanticType::Associated { member, .. } => SemanticType::Associated {
            base: Box::new(next(&mut children)?),
            member,
        },

        SemanticType::TypeApplication { .. } => {
            let constructor = Box::new(next(&mut children)?);
            SemanticType::TypeApplication {
                constructor,
                arguments: children.collect(),
            }
        }

        SemanticType::Extension {
            namespace,
            name,
            attributes,
            ..
        } => SemanticType::Extension {
            namespace,
            name,
            arguments: children.collect(),
            attributes,
        },

        leaf => leaf,
    };

    if children.next().is_some() {
        return Err(UnificationError::InvalidConfiguration);
    }

    Ok(rebuilt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::types::ty::{
        IntegerSignedness, IntegerWidth,
    };

    fn variable(id: u64) -> SemanticType {
        SemanticType::Variable(TypeVariableId(id))
    }

    fn int() -> SemanticType {
        SemanticType::integer(
            IntegerSignedness::Signed,
            IntegerWidth::bits("64").expect("valid width"),
        )
    }

    fn named(name: &str) -> SemanticType {
        SemanticType::named(QualifiedName::single(name))
    }

    #[test]
    fn binds_and_resolves_variable() {
        let mut unifier = TypeUnifier::new();
        unifier.unify(&variable(1), &int()).expect("unifies");

        assert_eq!(unifier.resolve(&variable(1)).expect("resolves"), int());
    }

    #[test]
    fn resolves_transitive_bindings() {
        let mut unifier = TypeUnifier::new();
        unifier.unify(&variable(1), &variable(2)).expect("first binding");
        unifier.unify(&variable(2), &int()).expect("second binding");

        assert_eq!(unifier.resolve(&variable(1)).expect("resolves"), int());
    }

    #[test]
    fn rejects_occurs_check() {
        let mut unifier = TypeUnifier::new();
        let recursive = SemanticType::Optional(Box::new(variable(7)));

        assert!(matches!(
            unifier.unify(&variable(7), &recursive),
            Err(UnificationError::OccursCheck { .. })
        ));
        assert_eq!(unifier.binding_count(), 0);
    }

    #[test]
    fn failure_rolls_back_partial_bindings() {
        let mut unifier = TypeUnifier::new();

        let left = SemanticType::Tuple(vec![variable(1), named("Left")]);
        let right = SemanticType::Tuple(vec![int(), named("Right")]);

        assert!(unifier.unify(&left, &right).is_err());
        assert_eq!(unifier.binding_count(), 0);
    }

    #[test]
    fn rejects_incompatible_known_array_lengths() {
        let left = SemanticType::Array {
            element: Box::new(int()),
            cardinality: Cardinality::Known(
                TypeLevelValue::integer("4").expect("valid cardinality"),
            ),
        };
        let right = SemanticType::Array {
            element: Box::new(int()),
            cardinality: Cardinality::Known(
                TypeLevelValue::integer("5").expect("valid cardinality"),
            ),
        };

        assert!(matches!(
            TypeUnifier::new().unify(&left, &right),
            Err(UnificationError::CardinalityMismatch { .. })
        ));
    }

    #[test]
    fn defers_unequal_symbolic_cardinalities() {
        let left = Cardinality::Symbolic(TypeLevelValue::Parameter("N".into()));
        let right = Cardinality::Symbolic(TypeLevelValue::Parameter("M".into()));

        assert!(matches!(
            unify_cardinality(&left, &right),
            Err(UnificationError::UnresolvedCardinalityConstraint { .. })
        ));
    }

    #[test]
    fn error_type_is_not_a_wildcard() {
        assert_eq!(
            TypeUnifier::new().unify(&SemanticType::Error, &int()),
            Err(UnificationError::ErrorTypeParticipates)
        );
    }

    #[test]
    fn work_budget_is_explicit() {
        let mut unifier = TypeUnifier::with_config(UnificationConfig {
            max_work: Some(1),
        })
        .expect("valid configuration");

        assert!(matches!(
            unifier.unify(
                &SemanticType::Tuple(vec![variable(1)]),
                &SemanticType::Tuple(vec![int()])
            ),
            Err(UnificationError::WorkBudgetExceeded { budget: 1 })
        ));
        assert_eq!(unifier.binding_count(), 0);
    }

    #[test]
    fn extension_types_are_compared_without_hardware_assumptions() {
        let ty = SemanticType::Extension {
            namespace: QualifiedName::single("zamani.quantum"),
            name: "logical_register".into(),
            arguments: vec![variable(3)],
            attributes: Vec::<TypeAttribute>::new(),
        };

        let concrete = SemanticType::Extension {
            namespace: QualifiedName::single("zamani.quantum"),
            name: "logical_register".into(),
            arguments: vec![int()],
            attributes: Vec::new(),
        };

        let mut unifier = TypeUnifier::new();
        unifier.unify(&ty, &concrete).expect("extension unifies");
        assert_eq!(unifier.resolve(&ty).expect("resolves"), concrete);
    }
}
