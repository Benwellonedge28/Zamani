#![allow(dead_code)]

//! Zamani Compiler — Hindley-Milner Type Inference Engine
//!
//! Production-oriented type inference infrastructure.
//!
//! Responsibilities:
//! - fresh type-variable generation
//! - substitutions
//! - occurs-check
//! - unification
//! - type substitution
//! - type schemes
//! - constraint solving
//! - basic expression inference
//! - deterministic type errors
//! - inference-state reset
//!
//! The implementation deliberately uses the Type variants exposed by the
//! Zamani AST instead of assuming additional AST variants.

use std::collections::HashMap;
use std::fmt;

use crate::ast::{
    Expression,
    FloatWidth,
    IntWidth,
    Literal,
    Type,
};

/// A type variable identifier.
pub type TypeVarId = usize;

/// Hindley-Milner type scheme.
///
/// `Mono(T)` represents a monomorphic type.
///
/// `Poly([a, b], T)` represents:
///
///     forall a b. T
#[derive(Debug, Clone, PartialEq)]
pub enum TypeScheme {
    Mono(Type),
    Poly(Vec<TypeVarId>, Type),
}

impl TypeScheme {
    pub fn mono(ty: Type) -> Self {
        Self::Mono(ty)
    }

    pub fn poly(vars: Vec<TypeVarId>, ty: Type) -> Self {
        Self::Poly(vars, ty)
    }

    pub fn ty(&self) -> &Type {
        match self {
            Self::Mono(ty) | Self::Poly(_, ty) => ty,
        }
    }

    pub fn quantified_vars(&self) -> &[TypeVarId] {
        match self {
            Self::Mono(_) => &[],
            Self::Poly(vars, _) => vars,
        }
    }
}

/// Type constraints generated during inference.
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    /// Two types must be equal.
    Equal(Type, Type),

    /// A type must satisfy an instance requirement.
    Instance(Type, String),
}

/// A structured type-inference error.
///
/// Keeping the error structured makes it possible for the compiler diagnostic
/// subsystem to later convert inference errors into source-aware diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeInferenceError {
    TypeMismatch {
        expected: String,
        found: String,
    },

    OccursCheck {
        variable: TypeVarId,
        ty: String,
    },

    UnknownTypeVariable {
        variable: TypeVarId,
    },

    UnsupportedExpression,

    InvalidConstraint(String),

    UnresolvedTypeVariable(TypeVarId),
}

impl fmt::Display for TypeInferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "type mismatch: expected {}, found {}",
                    expected,
                    found
                )
            }

            Self::OccursCheck { variable, ty } => {
                write!(
                    f,
                    "recursive type detected: T{} occurs in {}",
                    variable,
                    ty
                )
            }

            Self::UnknownTypeVariable { variable } => {
                write!(
                    f,
                    "unknown type variable T{}",
                    variable
                )
            }

            Self::UnsupportedExpression => {
                write!(
                    f,
                    "expression is not supported by the current type-inference phase"
                )
            }

            Self::InvalidConstraint(message) => {
                write!(
                    f,
                    "invalid type constraint: {}",
                    message
                )
            }

            Self::UnresolvedTypeVariable(variable) => {
                write!(
                    f,
                    "unresolved type variable T{}",
                    variable
                )
            }
        }
    }
}

impl std::error::Error for TypeInferenceError {}

/// Production-oriented Hindley-Milner inference engine.
#[derive(Debug, Clone)]
pub struct TypeInferenceEngine {
    /// Identifier assigned to the next fresh type variable.
    next_var_id: TypeVarId,

    /// Current substitution set.
    substitutions: HashMap<TypeVarId, Type>,

    /// Constraints waiting to be solved.
    constraints: Vec<Constraint>,

    /// Maximum number of inference variables allowed in one engine.
    ///
    /// `None` means unlimited.
    max_type_variables: Option<usize>,
}

impl Default for TypeInferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeInferenceEngine {
    /// Create a new inference engine.
    pub fn new() -> Self {
        Self {
            next_var_id: 1,
            substitutions: HashMap::new(),
            constraints: Vec::new(),
            max_type_variables: None,
        }
    }

    /// Create an engine with a type-variable limit.
    pub fn with_limit(
        max_type_variables: usize,
    ) -> Result<Self, TypeInferenceError> {
        if max_type_variables == 0 {
            return Err(
                TypeInferenceError::InvalidConstraint(
                    "type-variable limit must be greater than zero"
                        .to_string(),
                ),
            );
        }

        Ok(Self {
            max_type_variables: Some(max_type_variables),
            ..Self::new()
        })
    }

    /// Generate a fresh type variable.
    ///
    /// Zamani's AST represents generic variables using `Type::Generic`.
    pub fn fresh_var(&mut self) -> Type {
        let id = self.fresh_var_id();
        generic_type(id)
    }

    /// Generate a fresh type-variable identifier.
    pub fn fresh_var_id(&mut self) -> TypeVarId {
        if let Some(limit) = self.max_type_variables {
            if self.next_var_id > limit {
                // Keep the engine deterministic. The public `fresh_var()` API
                // cannot return a Result without breaking callers, so the
                // hard limit is enforced through `try_fresh_var()`.
                return self.next_var_id;
            }
        }

        let id = self.next_var_id;
        self.next_var_id += 1;
        id
    }

    /// Fallible fresh-variable generation.
    pub fn try_fresh_var(
        &mut self,
    ) -> Result<Type, TypeInferenceError> {
        if let Some(limit) = self.max_type_variables {
            if self.next_var_id > limit {
                return Err(
                    TypeInferenceError::InvalidConstraint(
                        format!(
                            "maximum type-variable limit ({}) exceeded",
                            limit
                        ),
                    ),
                );
            }
        }

        Ok(generic_type(self.fresh_var_id()))
    }

    /// Add a type equality constraint.
    pub fn add_constraint(
        &mut self,
        constraint: Constraint,
    ) {
        self.constraints.push(constraint);
    }

    /// Return the number of pending constraints.
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }

    /// Return the number of active substitutions.
    pub fn substitution_count(&self) -> usize {
        self.substitutions.len()
    }

    /// Clear inference state while retaining the variable counter.
    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }

    /// Completely reset the engine.
    pub fn reset(&mut self) {
        self.next_var_id = 1;
        self.substitutions.clear();
        self.constraints.clear();
    }

    /// Resolve a type through the current substitution set.
    pub fn resolve(
        &self,
        ty: &Type,
    ) -> Type {
        self.apply_substitutions(ty)
    }

    /// Obtain the current substitution for a type variable.
    pub fn substitution(
        &self,
        variable: TypeVarId,
    ) -> Option<&Type> {
        self.substitutions.get(&variable)
    }

    /// Unify two types.
    ///
    /// This performs:
    /// - reflexive equality
    /// - Unknown handling
    /// - generic-variable binding
    /// - occurs-check
    /// - recursive substitution
    /// - primitive type checking
    pub fn unify(
        &mut self,
        left: &Type,
        right: &Type,
    ) -> Result<(), TypeInferenceError> {
        let left = self.apply_substitutions(left);
        let right = self.apply_substitutions(right);

        if left == right {
            return Ok(());
        }

        match (&left, &right) {
            // Unknown is the compiler's incomplete/inference type.
            //
            // It is deliberately permissive here. Semantic validation can
            // later reject unresolved Unknown values where required.
            (Type::Unknown, _) | (_, Type::Unknown) => Ok(()),

            // Generic variables.
            _ if is_generic(&left) => {
                let variable = generic_id(&left)
                    .ok_or_else(|| {
                        TypeInferenceError::InvalidConstraint(
                            "invalid generic type representation"
                                .to_string(),
                        )
                    })?;

                self.bind(variable, &right)
            }

            _ if is_generic(&right) => {
                let variable = generic_id(&right)
                    .ok_or_else(|| {
                        TypeInferenceError::InvalidConstraint(
                            "invalid generic type representation"
                                .to_string(),
                        )
                    })?;

                self.bind(variable, &left)
            }

            // Primitive types.
            (Type::Bool, Type::Bool)
            | (Type::String, Type::String)
            | (Type::Int(_), Type::Int(_))
            | (Type::Float(_), Type::Float(_)) => Ok(()),

            // Unit type.
            (Type::Unit, Type::Unit) => Ok(()),

            // Generic type constructors.
            (
                Type::Generic(left_name, left_args),
                Type::Generic(right_name, right_args),
            ) => {
                if left_name != right_name {
                    return Err(
                        TypeInferenceError::TypeMismatch {
                            expected: format!(
                                "{:?}",
                                left
                            ),
                            found: format!(
                                "{:?}",
                                right
                            ),
                        },
                    );
                }

                if left_args.len() != right_args.len() {
                    return Err(
                        TypeInferenceError::TypeMismatch {
                            expected: format!(
                                "{:?}",
                                left
                            ),
                            found: format!(
                                "{:?}",
                                right
                            ),
                        },
                    );
                }

                for (a, b) in
                    left_args.iter().zip(right_args.iter())
                {
                    self.unify(a, b)?;
                }

                Ok(())
            }

            // All remaining type combinations are incompatible.
            _ => Err(
                TypeInferenceError::TypeMismatch {
                    expected: format!("{:?}", left),
                    found: format!("{:?}", right),
                },
            ),
        }
    }

    /// Bind a type variable to a type.
    fn bind(
        &mut self,
        variable: TypeVarId,
        ty: &Type,
    ) -> Result<(), TypeInferenceError> {
        let ty = self.apply_substitutions(ty);

        if is_generic_id(&ty, variable) {
            return Ok(());
        }

        if self.occurs(variable, &ty) {
            return Err(
                TypeInferenceError::OccursCheck {
                    variable,
                    ty: format!("{:?}", ty),
                },
            );
        }

        self.substitutions
            .insert(variable, ty);

        self.normalize_substitutions();

        Ok(())
    }

    /// Occurs-check.
    ///
    /// Prevents invalid recursive types such as:
    ///
    ///     T1 = List<T1>
    pub fn occurs(
        &self,
        variable: TypeVarId,
        ty: &Type,
    ) -> bool {
        let ty = self.apply_substitutions(ty);

        match &ty {
            Type::Generic(_, args) => {
                if let Some(id) = generic_id(&ty) {
                    if id == variable {
                        return true;
                    }
                }

                args.iter()
                    .any(|arg| self.occurs(variable, arg))
            }

            _ => false,
        }
    }

    /// Apply substitutions recursively.
    pub fn apply_substitutions(
        &self,
        ty: &Type,
    ) -> Type {
        match ty {
            Type::Generic(name, args) => {
                if let Some(variable) = generic_id(ty) {
                    if let Some(replacement) =
                        self.substitutions.get(&variable)
                    {
                        if !is_generic_id(
                            replacement,
                            variable,
                        ) {
                            return self.apply_substitutions(
                                replacement,
                            );
                        }
                    }
                }

                let new_args = args
                    .iter()
                    .map(|arg| {
                        self.apply_substitutions(arg)
                    })
                    .collect::<Vec<_>>();

                Type::Generic(
                    name.clone(),
                    new_args,
                )
            }

            _ => ty.clone(),
        }
    }

    /// Solve all pending constraints.
    pub fn solve_constraints(
        &mut self,
    ) -> Result<(), TypeInferenceError> {
        let constraints =
            std::mem::take(&mut self.constraints);

        for constraint in constraints {
            match constraint {
                Constraint::Equal(left, right) => {
                    self.unify(&left, &right)?;
                }

                Constraint::Instance(ty, trait_name) => {
                    self.validate_instance(
                        &ty,
                        &trait_name,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Basic instance checking.
    ///
    /// Trait/class resolution can later be connected to Zamani's type-class
    /// or capability system. Unknown and generic types are intentionally
    /// deferred.
    fn validate_instance(
        &self,
        ty: &Type,
        instance: &str,
    ) -> Result<(), TypeInferenceError> {
        let resolved = self.apply_substitutions(ty);

        if matches!(
            resolved,
            Type::Unknown | Type::Generic(_, _)
        ) {
            return Ok(());
        }

        if instance.is_empty() {
            return Err(
                TypeInferenceError::InvalidConstraint(
                    "instance name cannot be empty"
                        .to_string(),
                ),
            );
        }

        Ok(())
    }

    /// Infer the type of an expression.
    ///
    /// The currently known Zamani AST literal forms are inferred precisely.
    /// Unsupported expression variants return a structured error rather than
    /// silently manufacturing `Unknown`.
    pub fn infer(
        &mut self,
        expr: &Expression,
    ) -> Result<Type, TypeInferenceError> {
        match expr {
            Expression::Literal(_, literal) => {
                self.infer_literal(literal)
            }

            // Until all expression forms are represented in the repository's
            // current AST, don't make assumptions about their structure.
            _ => Err(
                TypeInferenceError::UnsupportedExpression
            ),
        }
    }

    /// Infer a literal.
    pub fn infer_literal(
        &self,
        literal: &Literal,
    ) -> Result<Type, TypeInferenceError> {
        match literal {
            Literal::Integer(_, _) => {
                Ok(Type::Int(IntWidth::I64))
            }

            Literal::Float(_, _) => {
                Ok(Type::Float(FloatWidth::F64))
            }

            Literal::String(_, _) => {
                Ok(Type::String)
            }

            Literal::Boolean(_, _) => {
                Ok(Type::Bool)
            }

            _ => Ok(Type::Unit),
        }
    }

    /// Infer and fully resolve an expression type.
    pub fn infer_resolved(
        &mut self,
        expr: &Expression,
    ) -> Result<Type, TypeInferenceError> {
        let ty = self.infer(expr)?;
        self.solve_constraints()?;
        Ok(self.apply_substitutions(&ty))
    }

    /// Instantiate a polymorphic type scheme with fresh variables.
    pub fn instantiate(
        &mut self,
        scheme: &TypeScheme,
    ) -> Result<Type, TypeInferenceError> {
        match scheme {
            TypeScheme::Mono(ty) => Ok(
                self.apply_substitutions(ty)
            ),

            TypeScheme::Poly(vars, ty) => {
                let mut replacements =
                    HashMap::new();

                for variable in vars {
                    let fresh =
                        self.try_fresh_var()?;

                    replacements.insert(
                        *variable,
                        fresh,
                    );
                }

                Ok(substitute_variables(
                    ty,
                    &replacements,
                ))
            }
        }
    }

    /// Generalize a type over the supplied environment variables.
    ///
    /// The environment is represented as a slice of types so this module
    /// remains independent from the repository's eventual symbol-table type.
    pub fn generalize(
        &self,
        ty: &Type,
        environment: &[Type],
    ) -> TypeScheme {
        let resolved =
            self.apply_substitutions(ty);

        let type_vars =
            collect_type_variables(&resolved);

        let mut environment_vars =
            std::collections::HashSet::new();

        for env_ty in environment {
            environment_vars.extend(
                collect_type_variables(
                    &self.apply_substitutions(env_ty),
                ),
            );
        }

        let quantified = type_vars
            .into_iter()
            .filter(|id| {
                !environment_vars.contains(id)
            })
            .collect::<Vec<_>>();

        if quantified.is_empty() {
            TypeScheme::Mono(resolved)
        } else {
            TypeScheme::Poly(
                quantified,
                resolved,
            )
        }
    }

    /// Normalize all known substitutions.
    fn normalize_substitutions(&mut self) {
        let keys =
            self.substitutions
                .keys()
                .copied()
                .collect::<Vec<_>>();

        for key in keys {
            if let Some(value) =
                self.substitutions.get(&key).cloned()
            {
                let resolved =
                    self.apply_substitutions(&value);

                self.substitutions
                    .insert(key, resolved);
            }
        }
    }
}

/// Construct Zamani's generic type representation.
fn generic_type(
    id: TypeVarId,
) -> Type {
    Type::Generic(
        format!("T{}", id),
        Vec::new(),
    )
}

/// Determine whether a type is represented as a generic variable.
fn is_generic(
    ty: &Type,
) -> bool {
    matches!(
        ty,
        Type::Generic(_, args)
            if args.is_empty()
    )
}

/// Extract a generic variable identifier.
///
/// Only `T<number>` is treated as an inference variable. Other generic
/// constructors remain ordinary generic types.
fn generic_id(
    ty: &Type,
) -> Option<TypeVarId> {
    match ty {
        Type::Generic(name, args)
            if args.is_empty() =>
        {
            name.strip_prefix('T')?
                .parse::<usize>()
                .ok()
        }

        _ => None,
    }
}

/// Determine whether a type is a specific generic variable.
fn is_generic_id(
    ty: &Type,
    variable: TypeVarId,
) -> bool {
    generic_id(ty) == Some(variable)
}

/// Recursively collect type variables.
fn collect_type_variables(
    ty: &Type,
) -> Vec<TypeVarId> {
    let mut variables =
        std::collections::HashSet::new();

    collect_type_variables_into(
        ty,
        &mut variables,
    );

    let mut result =
        variables.into_iter().collect::<Vec<_>>();

    result.sort_unstable();
    result
}

fn collect_type_variables_into(
    ty: &Type,
    output: &mut std::collections::HashSet<TypeVarId>,
) {
    match ty {
        Type::Generic(_, args) => {
            if let Some(id) = generic_id(ty) {
                output.insert(id);
            }

            for arg in args {
                collect_type_variables_into(
                    arg,
                    output,
                );
            }
        }

        _ => {}
    }
}

/// Substitute a set of generic variables.
fn substitute_variables(
    ty: &Type,
    replacements: &HashMap<TypeVarId, Type>,
) -> Type {
    match ty {
        Type::Generic(name, args) => {
            if let Some(id) = generic_id(ty) {
                if let Some(replacement) =
                    replacements.get(&id)
                {
                    return replacement.clone();
                }
            }

            Type::Generic(
                name.clone(),
                args.iter()
                    .map(|arg| {
                        substitute_variables(
                            arg,
                            replacements,
                        )
                    })
                    .collect(),
            )
        }

        _ => ty.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_variables_are_unique() {
        let mut engine =
            TypeInferenceEngine::new();

        let first = engine.fresh_var();
        let second = engine.fresh_var();

        assert_ne!(first, second);
    }

    #[test]
    fn equal_types_unify() {
        let mut engine =
            TypeInferenceEngine::new();

        assert!(
            engine
                .unify(
                    &Type::Bool,
                    &Type::Bool
                )
                .is_ok()
        );
    }

    #[test]
    fn incompatible_types_fail() {
        let mut engine =
            TypeInferenceEngine::new();

        let result = engine.unify(
            &Type::Bool,
            &Type::String,
        );

        assert!(result.is_err());
    }

    #[test]
    fn generic_variable_can_be_bound() {
        let mut engine =
            TypeInferenceEngine::new();

        let variable =
            engine.fresh_var();

        engine
            .unify(
                &variable,
                &Type::Bool,
            )
            .unwrap();

        assert_eq!(
            engine.resolve(&variable),
            Type::Bool
        );
    }

    #[test]
    fn occurs_check_rejects_recursive_type() {
        let mut engine =
            TypeInferenceEngine::new();

        let variable =
            engine.fresh_var();

        let recursive =
            Type::Generic(
                "List".into(),
                vec![variable.clone()],
            );

        let result =
            engine.unify(
                &variable,
                &recursive,
            );

        assert!(matches!(
            result,
            Err(
                TypeInferenceError::OccursCheck {
                    ..
                }
            )
        ));
    }

    #[test]
    fn unknown_is_permissive() {
        let mut engine =
            TypeInferenceEngine::new();

        assert!(
            engine
                .unify(
                    &Type::Unknown,
                    &Type::Bool
                )
                .is_ok()
        );
    }

    #[test]
    fn integer_literal_is_i64() {
        let engine =
            TypeInferenceEngine::new();

        let literal =
            Literal::Integer(
                crate::source_map::Span::dummy(),
                42,
            );

        let result =
            engine.infer_literal(&literal)
                .unwrap();

        assert_eq!(
            result,
            Type::Int(IntWidth::I64)
        );
    }

    #[test]
    fn boolean_literal_is_bool() {
        let engine =
            TypeInferenceEngine::new();

        let literal =
            Literal::Boolean(
                crate::source_map::Span::dummy(),
                true,
            );

        let result =
            engine.infer_literal(&literal)
                .unwrap();

        assert_eq!(
            result,
            Type::Bool
        );
    }

    #[test]
    fn constraints_are_solved() {
        let mut engine =
            TypeInferenceEngine::new();

        let variable =
            engine.fresh_var();

        engine.add_constraint(
            Constraint::Equal(
                variable.clone(),
                Type::String,
            ),
        );

        engine.solve_constraints()
            .unwrap();

        assert_eq!(
            engine.resolve(&variable),
            Type::String
        );
    }

    #[test]
    fn polymorphic_scheme_instantiates() {
        let mut engine =
            TypeInferenceEngine::new();

        let variable =
            engine.fresh_var();

        let variable_id =
            generic_id(&variable)
                .unwrap();

        let scheme =
            TypeScheme::Poly(
                vec![variable_id],
                variable,
            );

        let first =
            engine.instantiate(&scheme)
                .unwrap();

        let second =
            engine.instantiate(&scheme)
                .unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn generalization_finds_free_variables() {
        let mut engine =
            TypeInferenceEngine::new();

        let variable =
            engine.fresh_var();

        let scheme =
            engine.generalize(
                &variable,
                &[],
            );

        assert_eq!(
            scheme.quantified_vars().len(),
            1
        );
    }

    #[test]
    fn reset_clears_state() {
        let mut engine =
            TypeInferenceEngine::new();

        let variable =
            engine.fresh_var();

        engine
            .unify(
                &variable,
                &Type::Bool,
            )
            .unwrap();

        engine.reset();

        assert_eq!(
            engine.substitution_count(),
            0
        );

        assert_eq!(
            engine.constraint_count(),
            0
        );
    }
}