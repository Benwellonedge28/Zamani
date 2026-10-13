
//! Trait declarations, implementation coherence, obligation solving, and
//! associated-type validation for Zamani.
//!
//! This module owns the semantic trait registry and trait-obligation solver.
//! It does not own parsing, AST construction, general type unification,
//! type compatibility, target selection, hardware allocation, or IR lowering.
//!
//! All type-pattern matching, substitution, compatibility, and overlap
//! checks are delegated to the canonical semantic type-system adapter.
//!
//! No fixed hardware limits or unsafe code are used.

use std::collections::{HashMap, HashSet};

use super::ty::{QualifiedName, SemanticName, SemanticType};

/// Bindings for generic parameters during pattern matching and substitution.
///
/// Parameter identity is represented by `SemanticName` in this interface.
/// A future symbol/type-system migration should prefer stable generic-parameter
/// IDs to avoid collisions caused by shadowed parameter names.
pub type TypeBindings = HashMap<SemanticName, SemanticType>;

/// A fully resolved trait obligation.
///
/// For example, `Display` on `String` is represented by a `self_type` of
/// `String` and a trait reference naming `Display`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TraitRef {
    pub name: QualifiedName,
    pub arguments: Vec<SemanticType>,
}

/// A trait reference whose arguments may contain generic parameters.
///
/// The patterns are semantic types, not parser-level `TypeExpr` nodes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TraitRefPattern {
    pub name: QualifiedName,
    pub arguments: Vec<SemanticType>,
}

/// A required associated type declared by a trait.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssociatedTypeRequirement {
    pub name: SemanticName,
}

/// A required method declared by a trait.
///
/// `signature` must be the canonical semantic function type. The adapter
/// decides whether an implementation signature satisfies this requirement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodRequirement {
    pub name: SemanticName,
    pub signature: SemanticType,
}

/// A validated semantic trait declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraitDefinition {
    pub name: QualifiedName,
    pub type_parameters: Vec<SemanticName>,
    pub supertraits: Vec<TraitRefPattern>,
    pub associated_types: Vec<AssociatedTypeRequirement>,
    pub methods: Vec<MethodRequirement>,
}

/// A method supplied by an implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImplementedMethod {
    pub name: SemanticName,
    pub signature: SemanticType,
}

/// An associated type supplied by an implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssociatedTypeDefinition {
    pub name: SemanticName,
    pub value: SemanticType,
}

/// A semantic implementation of a trait for a type pattern.
///
/// `type_parameters` belong to this implementation. They are distinct from
/// the parameters declared by the trait itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraitImplementation {
    pub trait_ref: TraitRefPattern,
    pub target_type: SemanticType,
    pub type_parameters: Vec<SemanticName>,
    pub where_bounds: Vec<TraitRefPattern>,
    pub associated_types: Vec<AssociatedTypeDefinition>,
    pub methods: Vec<ImplementedMethod>,
}

/// Outcome of checking a trait obligation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraitCheck {
    /// A unique implementation satisfies the obligation.
    Satisfied {
        implementation_index: usize,
    },

    /// The available semantic information is insufficient to decide.
    ///
    /// A caller must not treat this result as proof that the obligation holds.
    Deferred {
        reason: String,
    },
}

/// Errors produced by registry operations or obligation solving.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraitError<E> {
    DuplicateTrait {
        name: QualifiedName,
    },
    UnknownTrait {
        name: QualifiedName,
    },
    TraitArgumentArity {
        name: QualifiedName,
        expected: usize,
        actual: usize,
    },
    DuplicateImplementation {
        trait_name: QualifiedName,
    },
    OverlappingImplementations {
        trait_name: QualifiedName,
    },
    IncompleteType {
        context: String,
    },
    NoApplicableImplementation {
        trait_name: QualifiedName,
    },
    AmbiguousImplementation {
        trait_name: QualifiedName,
        candidates: Vec<usize>,
    },
    CyclicObligation {
        trait_name: QualifiedName,
    },
    MissingMethod {
        trait_name: QualifiedName,
        method: SemanticName,
    },
    IncompatibleMethod {
        trait_name: QualifiedName,
        method: SemanticName,
    },
    MissingAssociatedType {
        trait_name: QualifiedName,
        associated_type: SemanticName,
    },
    IncompatibleAssociatedType {
        trait_name: QualifiedName,
        associated_type: SemanticName,
    },
    InvalidDeclaration {
        message: String,
    },
    TypeSystem(E),
}

/// Interface to Zamani's canonical type-system implementation.
///
/// Implement this adapter using the existing type unifier, generic-bound
/// checker, alias normalizer, and compatibility engine. This module
/// intentionally does not implement a second type-unification algorithm.
pub trait TraitTypeSystem {
    type Error;

    /// Returns whether a type is incomplete, unresolved, or otherwise
    /// unsuitable as proof of a successful trait obligation.
    fn is_incomplete(&self, ty: &SemanticType) -> bool;

    /// Match a semantic pattern against an actual type.
    ///
    /// Generic variables in `pattern` are recorded in `bindings`.
    /// Return `Ok(false)` for a valid non-match; return `Err` for a semantic
    /// failure that the caller cannot safely interpret as a non-match.
    fn match_type_pattern(
        &self,
        pattern: &SemanticType,
        actual: &SemanticType,
        bindings: &mut TypeBindings,
    ) -> Result<bool, Self::Error>;

    /// Substitute generic bindings into a semantic type.
    fn substitute_type(
        &self,
        ty: &SemanticType,
        bindings: &TypeBindings,
    ) -> Result<SemanticType, Self::Error>;

    /// Check whether two resolved types are compatible for a required
    /// trait method signature.
    fn method_signatures_compatible(
        &self,
        required: &SemanticType,
        provided: &SemanticType,
    ) -> Result<bool, Self::Error>;

    /// Check compatibility of an associated type with a declared
    /// associated-type constraint, when the language supports one.
    fn associated_types_compatible(
        &self,
        required: &SemanticType,
        provided: &SemanticType,
    ) -> Result<bool, Self::Error>;

    /// Determine whether two implementation patterns could overlap.
    ///
    /// This must be conservative: return `true` whenever disjointness cannot
    /// be established. It is used for coherence validation, not dispatch.
    fn patterns_may_overlap(
        &self,
        left: &SemanticType,
        right: &SemanticType,
    ) -> Result<bool, Self::Error>;
}

/// Identity of an obligation currently being solved.
///
/// Uses semantic equality and hashing rather than formatted diagnostic
/// strings, avoiding collisions and formatting-dependent solver behaviour.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ObligationKey {
    self_type: SemanticType,
    trait_ref: TraitRef,
}

/// Registry and solver for semantic trait declarations and implementations.
#[derive(Clone, Debug, Default)]
pub struct TraitRegistry {
    definitions: HashMap<QualifiedName, TraitDefinition>,
    implementations: Vec<TraitImplementation>,
}

impl TraitRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Look up a trait declaration.
    pub fn definition(&self, name: &QualifiedName) -> Option<&TraitDefinition> {
        self.definitions.get(name)
    }

    /// Iterate over registered trait declarations.
    pub fn definitions(&self) -> impl Iterator<Item = &TraitDefinition> {
        self.definitions.values()
    }

    /// Iterate over registered implementations.
    pub fn implementations(&self) -> impl Iterator<Item = &TraitImplementation> {
        self.implementations.iter()
    }

    /// Register a previously validated trait declaration.
    ///
    /// AST-level checks—duplicate members, malformed generic declarations,
    /// and syntax-specific diagnostics—should happen before this operation.
    pub fn register_trait<E>(
        &mut self,
        definition: TraitDefinition,
    ) -> Result<(), TraitError<E>> {
        if self.definitions.contains_key(&definition.name) {
            return Err(TraitError::DuplicateTrait {
                name: definition.name,
            });
        }

        if has_duplicates(&definition.type_parameters) {
            return Err(TraitError::InvalidDeclaration {
                message: "trait contains duplicate type-parameter names".to_owned(),
            });
        }

        if has_duplicates(
            &definition
                .associated_types
                .iter()
                .map(|item| item.name.clone())
                .collect::<Vec<_>>(),
        ) {
            return Err(TraitError::InvalidDeclaration {
                message: "trait contains duplicate associated-type names".to_owned(),
            });
        }

        if has_duplicates(
            &definition
                .methods
                .iter()
                .map(|item| item.name.clone())
                .collect::<Vec<_>>(),
        ) {
            return Err(TraitError::InvalidDeclaration {
                message: "trait contains duplicate method names".to_owned(),
            });
        }

        for supertrait in &definition.supertraits {
            if has_duplicates(&supertrait.arguments) {
                // Equal type arguments are perfectly legal; do not interpret
                // repeated argument values as duplicate parameters.
                // Arity and parameter validation happen below instead.
            }

            let Some(parent) = self.definitions.get(&supertrait.name) else {
                return Err(TraitError::UnknownTrait {
                    name: supertrait.name.clone(),
                });
            };

            if parent.type_parameters.len() != supertrait.arguments.len() {
                return Err(TraitError::TraitArgumentArity {
                    name: supertrait.name.clone(),
                    expected: parent.type_parameters.len(),
                    actual: supertrait.arguments.len(),
                });
            }
        }

        self.definitions
            .insert(definition.name.clone(), definition);

        Ok(())
    }

    /// Register an implementation after checking its declaration shape
    /// and checking coherence against existing implementations.
    ///
    /// The type-system adapter is responsible for semantic pattern matching
    /// and conservative overlap detection.
    pub fn register_implementation<TS: TraitTypeSystem>(
        &mut self,
        implementation: TraitImplementation,
        types: &TS,
    ) -> Result<(), TraitError<TS::Error>> {
        let Some(definition) = self.definitions.get(&implementation.trait_ref.name) else {
            return Err(TraitError::UnknownTrait {
                name: implementation.trait_ref.name,
            });
        };

        if definition.type_parameters.len() != implementation.trait_ref.arguments.len() {
            return Err(TraitError::TraitArgumentArity {
                name: implementation.trait_ref.name,
                expected: definition.type_parameters.len(),
                actual: implementation.trait_ref.arguments.len(),
            });
        }

        if has_duplicates(&implementation.type_parameters) {
            return Err(TraitError::InvalidDeclaration {
                message: "implementation contains duplicate type-parameter names".to_owned(),
            });
        }

        if has_duplicates(
            &implementation
                .associated_types
                .iter()
                .map(|item| item.name.clone())
                .collect::<Vec<_>>(),
        ) {
            return Err(TraitError::InvalidDeclaration {
                message: "implementation contains duplicate associated-type definitions"
                    .to_owned(),
            });
        }

        if has_duplicates(
            &implementation
                .methods
                .iter()
                .map(|item| item.name.clone())
                .collect::<Vec<_>>(),
        ) {
            return Err(TraitError::InvalidDeclaration {
                message: "implementation contains duplicate method definitions".to_owned(),
            });
        }

        let trait_name = implementation.trait_ref.name.clone();

        for existing in &self.implementations {
            if existing.trait_ref.name != implementation.trait_ref.name {
                continue;
            }

            if existing == &implementation {
                return Err(TraitError::DuplicateImplementation {
                    trait_name,
                });
            }

            let target_overlap = types
                .patterns_may_overlap(&existing.target_type, &implementation.target_type)
                .map_err(TraitError::TypeSystem)?;

            if !target_overlap {
                continue;
            }

            let mut trait_arguments_overlap = true;

            for (left, right) in existing
                .trait_ref
                .arguments
                .iter()
                .zip(&implementation.trait_ref.arguments)
            {
                if !types
                    .patterns_may_overlap(left, right)
                    .map_err(TraitError::TypeSystem)?
                {
                    trait_arguments_overlap = false;
                    break;
                }
            }

            if trait_arguments_overlap {
                return Err(TraitError::OverlappingImplementations {
                    trait_name,
                });
            }
        }

        self.implementations.push(implementation);
        Ok(())
    }

    /// Solve one fully resolved trait obligation.
    ///
    /// `Satisfied` is returned only when a unique applicable implementation
    /// is found and all of its bounds, supertraits, methods, and associated
    /// types pass validation. `Deferred` is not equivalent to success.
    pub fn solve<TS: TraitTypeSystem>(
        &self,
        self_type: &SemanticType,
        trait_ref: &TraitRef,
        types: &TS,
    ) -> Result<TraitCheck, TraitError<TS::Error>> {
        let mut active = HashSet::new();

        self.solve_inner(self_type, trait_ref, types, &mut active)
    }

    fn solve_inner<TS: TraitTypeSystem>(
        &self,
        self_type: &SemanticType,
        trait_ref: &TraitRef,
        types: &TS,
        active: &mut HashSet<ObligationKey>,
    ) -> Result<TraitCheck, TraitError<TS::Error>> {
        if types.is_incomplete(self_type)
            || trait_ref.arguments.iter().any(|ty| types.is_incomplete(ty))
        {
            return Ok(TraitCheck::Deferred {
                reason: "trait obligation contains an incomplete type".to_owned(),
            });
        }

        let Some(definition) = self.definitions.get(&trait_ref.name) else {
            return Err(TraitError::UnknownTrait {
                name: trait_ref.name.clone(),
            });
        };

        if definition.type_parameters.len() != trait_ref.arguments.len() {
            return Err(TraitError::TraitArgumentArity {
                name: trait_ref.name.clone(),
                expected: definition.type_parameters.len(),
                actual: trait_ref.arguments.len(),
            });
        }

        let key = ObligationKey {
            self_type: self_type.clone(),
            trait_ref: trait_ref.clone(),
        };

        if !active.insert(key.clone()) {
            return Err(TraitError::CyclicObligation {
                trait_name: trait_ref.name.clone(),
            });
        }

        let result = self.solve_active(self_type, trait_ref, definition, types, active);

        active.remove(&key);
        result
    }

    fn solve_active<TS: TraitTypeSystem>(
        &self,
        self_type: &SemanticType,
        trait_ref: &TraitRef,
        definition: &TraitDefinition,
        types: &TS,
        active: &mut HashSet<ObligationKey>,
    ) -> Result<TraitCheck, TraitError<TS::Error>> {
        let mut candidates = Vec::new();

        for (index, implementation) in self.implementations.iter().enumerate() {
            if implementation.trait_ref.name != trait_ref.name {
                continue;
            }

            if implementation.trait_ref.arguments.len() != trait_ref.arguments.len() {
                continue;
            }

            let mut bindings = TypeBindings::new();

            let target_matches = types
                .match_type_pattern(
                    &implementation.target_type,
                    self_type,
                    &mut bindings,
                )
                .map_err(TraitError::TypeSystem)?;

            if !target_matches {
                continue;
            }

            let mut arguments_match = true;

            for (pattern, actual) in implementation
                .trait_ref
                .arguments
                .iter()
                .zip(&trait_ref.arguments)
            {
                if !types
                    .match_type_pattern(pattern, actual, &mut bindings)
                    .map_err(TraitError::TypeSystem)?
                {
                    arguments_match = false;
                    break;
                }
            }

            if arguments_match {
                candidates.push((index, bindings));
            }
        }

        if candidates.is_empty() {
            return Err(TraitError::NoApplicableImplementation {
                trait_name: trait_ref.name.clone(),
            });
        }

        if candidates.len() > 1 {
            return Err(TraitError::AmbiguousImplementation {
                trait_name: trait_ref.name.clone(),
                candidates: candidates.iter().map(|(index, _)| *index).collect(),
            });
        }

        let (implementation_index, implementation_bindings) = candidates
            .pop()
            .expect("candidate count was checked above");

        let implementation = &self.implementations[implementation_index];

        // Trait-declaration parameters must be substituted using the actual
        // arguments of the requested trait, not implementation-generic
        // bindings. These are two different parameter scopes.
        let mut trait_bindings = TypeBindings::new();

        for (parameter, argument) in definition
            .type_parameters
            .iter()
            .zip(&trait_ref.arguments)
        {
            trait_bindings.insert(parameter.clone(), argument.clone());
        }

        // An implementation's where-bounds are expressed in the scope of
        // that implementation, so they use implementation_bindings.
        for bound in &implementation.where_bounds {
            let instantiated = substitute_trait_pattern(
                bound,
                &implementation_bindings,
                types,
            )?;

            let bound_self_type = self_type.clone();

            match self.solve_inner(
                &bound_self_type,
                &TraitRef {
                    name: instantiated.name,
                    arguments: instantiated.arguments,
                },
                types,
                active,
            )? {
                TraitCheck::Satisfied { .. } => {}
                TraitCheck::Deferred { reason } => {
                    return Ok(TraitCheck::Deferred { reason });
                }
            }
        }

        // Every declared supertrait must be satisfied for this trait
        // obligation. Supertrait arguments are substituted in the trait's
        // own parameter scope.
        for supertrait in &definition.supertraits {
            let instantiated =
                substitute_trait_pattern(supertrait, &trait_bindings, types)?;

            match self.solve_inner(
                self_type,
                &TraitRef {
                    name: instantiated.name,
                    arguments: instantiated.arguments,
                },
                types,
                active,
            )? {
                TraitCheck::Satisfied { .. } => {}
                TraitCheck::Deferred { reason } => {
                    return Ok(TraitCheck::Deferred { reason });
                }
            }
        }

        self.validate_members(
            definition,
            implementation,
            &trait_bindings,
            &implementation_bindings,
            types,
        )?;

        Ok(TraitCheck::Satisfied {
            implementation_index,
        })
    }

    fn validate_members<TS: TraitTypeSystem>(
        &self,
        definition: &TraitDefinition,
        implementation: &TraitImplementation,
        trait_bindings: &TypeBindings,
        implementation_bindings: &TypeBindings,
        types: &TS,
    ) -> Result<(), TraitError<TS::Error>> {
        for required in &definition.methods {
            let Some(provided) = implementation
                .methods
                .iter()
                .find(|method| method.name == required.name)
            else {
                return Err(TraitError::MissingMethod {
                    trait_name: definition.name.clone(),
                    method: required.name.clone(),
                });
            };

            let required_signature = types
                .substitute_type(&required.signature, trait_bindings)
                .map_err(TraitError::TypeSystem)?;

            let provided_signature = types
                .substitute_type(&provided.signature, implementation_bindings)
                .map_err(TraitError::TypeSystem)?;

            if types.is_incomplete(&required_signature)
                || types.is_incomplete(&provided_signature)
            {
                return Err(TraitError::IncompleteType {
                    context: format!(
                        "method signature in trait {}",
                        definition.name
                    ),
                });
            }

            let compatible = types
                .method_signatures_compatible(&required_signature, &provided_signature)
                .map_err(TraitError::TypeSystem)?;

            if !compatible {
                return Err(TraitError::IncompatibleMethod {
                    trait_name: definition.name.clone(),
                    method: required.name.clone(),
                });
            }
        }

        for required in &definition.associated_types {
            let Some(provided) = implementation
                .associated_types
                .iter()
                .find(|item| item.name == required.name)
            else {
                return Err(TraitError::MissingAssociatedType {
                    trait_name: definition.name.clone(),
                    associated_type: required.name.clone(),
                });
            };

            // This basic requirement checks that the supplied type is usable.
            // If the language adds constrained associated types, the
            // constraint must be represented in the requirement and checked
            // through the canonical type-system adapter.
            if types.is_incomplete(&provided.value) {
                return Err(TraitError::IncompleteType {
                    context: format!(
                        "associated type {} in trait {}",
                        required.name, definition.name
                    ),
                });
            }

            // Keep this hook available for constrained associated types.
            // For unconstrained associated types, no expected type exists
            // against which compatibility could meaningfully be checked.
            let _compatibility_hook = &types.associated_types_compatible;
        }

        Ok(())
    }
}

fn substitute_trait_pattern<TS: TraitTypeSystem>(
    pattern: &TraitRefPattern,
    bindings: &TypeBindings,
    types: &TS,
) -> Result<TraitRefPattern, TraitError<TS::Error>> {
    let mut arguments = Vec::with_capacity(pattern.arguments.len());

    for argument in &pattern.arguments {
        arguments.push(
            types
                .substitute_type(argument, bindings)
                .map_err(TraitError::TypeSystem)?,
        );
    }

    Ok(TraitRefPattern {
        name: pattern.name.clone(),
        arguments,
    })
}

fn has_duplicates<T: Eq>(items: &[T]) -> bool {
    let mut seen = HashSet::with_capacity(items.len());

    items.iter().any(|item| !seen.insert(item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_detection_accepts_distinct_items() {
        assert!(!has_duplicates(&[1, 2, 3]));
    }

    #[test]
    fn duplicate_detection_rejects_repeated_items() {
        assert!(has_duplicates(&[1, 2, 1]));
    }

    #[test]
    fn empty_registry_is_constructible() {
        let registry = TraitRegistry::new();

        assert_eq!(registry.definitions().count(), 0);
        assert_eq!(registry.implementations().count(), 0);
    }
}
