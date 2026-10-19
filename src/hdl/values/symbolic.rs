#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Symbolic Values
//!
//! Path: `src/hdl/values/symbolic.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent parameter-dependent HDL values without prematurely evaluating
//! them, guessing missing values, or imposing a fixed hardware capacity.
//!
//! This module provides:
//!
//! - Stable symbolic identities using the canonical `HdlId`.
//! - Explicit dependency declarations for symbolic definitions.
//! - A typed environment for concrete bindings and symbolic definitions.
//! - Deterministic, iterative dependency traversal.
//! - Dependency-cycle detection with a useful cycle path.
//! - Evaluation through a caller-supplied evaluator.
//! - Explicit errors for unresolved symbols, duplicate definitions,
//!   conflicting bindings, cycles, and evaluator failures.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/parameters.g4`
//! - `grammar/hdl/hardware-generics.g4`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/hdl/widths.rs`
//! - `src/hdl/types/logic.rs`
//! - `src/hdl/types/layout.rs`
//!
//! The grammar owns syntax. The frontend owns canonical expressions and
//! semantic name resolution. This module owns the symbolic-value dependency
//! environment and the mechanics of resolving its dependency graph.
//!
//! ## Does not own
//!
//! - A replacement AST, expression parser, or general expression language.
//! - General-purpose name resolution or type inference.
//! - A second HDL width representation.
//! - Arbitrary-precision arithmetic implementation.
//! - Hardware allocation, device discovery, or target feasibility.
//! - Simulation, synthesis, scheduling, placement, or routing.
//! - HDLIR structures, builders, passes, verification, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration
//!
//! The frontend adapter supplies canonical `HdlId` values and extracts
//! dependency identities from already parsed and semantically checked
//! expressions. An evaluator supplied by the semantic/elaboration layer
//! evaluates a definition using the values of its declared dependencies.
//!
//! `src/hdl/widths.rs` remains the owner of concrete `HdlWidth` resolution.
//! This module can be used to resolve parameter dependencies before the
//! caller constructs a `WidthEnvironment`.
//!
//! The generic value type `T` is deliberately supplied by the caller.
//! This module therefore does not invent a competing universal value type.
//!
//! ## Scalability
//!
//! There is no language-level maximum for the number of definitions or
//! dependencies. Ordered standard-library collections provide deterministic
//! traversal. Dependency evaluation uses an explicit stack rather than
//! recursive calls, so deeply nested dependency graphs do not consume the
//! Rust call stack through this module.
//!
//! Execution remains finite and consumes resources proportional to the
//! reachable graph and the values stored in the environment. Callers should
//! apply explicit operational budgets where required.
//!
//! This module does not promise infinite physical memory or computation.
//! It introduces no artificial HDL-specific capacity ceiling.
//!
//! ## Freeze criteria
//!
//! - The canonical `HdlId` remains the sole symbolic identity type.
//! - The frontend supplies dependency information without losing source
//!   expression identity.
//! - Missing bindings are never guessed.
//! - Cycles are reported rather than evaluated indefinitely.
//! - Evaluator failures remain distinguishable from dependency failures.
//! - Repeated identical bindings are idempotent.
//! - Conflicting bindings never overwrite existing values.
//! - Unit, boundary, determinism, and stress tests pass.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::frontend::semantic::domains::hdl::HdlId;

/// Schema version for the symbolic-value environment.
///
/// This version describes this module's API contract. It is independent of
/// the language version, AST schema, and HDLIR version.
pub const HDL_SYMBOLIC_SCHEMA_VERSION: u16 = 1;

/// A symbolic definition referring to a canonical expression identity.
///
/// The expression itself remains owned by the canonical frontend AST or
/// semantic expression representation. This structure records only its
/// identity and the symbols required to evaluate it.
///
/// Dependencies are normalized into deterministic order when the
/// definition is created. Repeated dependencies are removed because a
/// dependency graph needs each referenced identity only once.
///
/// The caller must ensure that the dependency list faithfully represents
/// every external symbolic value needed to evaluate the expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolicValue {
    expression: HdlId,
    dependencies: Vec<HdlId>,
}

impl SymbolicValue {
    /// Create a symbolic definition from its canonical expression identity
    /// and dependency identities.
    ///
    /// An empty dependency list represents an expression that can be
    /// evaluated without other symbolic definitions.
    ///
    /// This constructor does not establish that the expression exists or
    /// that its dependencies are semantically valid. Those checks belong
    /// to the frontend and semantic layer.
    #[must_use]
    pub fn new(
        expression: HdlId,
        dependencies: impl IntoIterator<Item = HdlId>,
    ) -> Self {
        let dependencies: BTreeSet<HdlId> = dependencies.into_iter().collect();

        Self {
            expression,
            dependencies: dependencies.into_iter().collect(),
        }
    }

    /// Return the canonical expression identity.
    #[must_use]
    pub fn expression(&self) -> &HdlId {
        &self.expression
    }

    /// Return the unique dependencies in deterministic order.
    #[must_use]
    pub fn dependencies(&self) -> &[HdlId] {
        &self.dependencies
    }
}

/// Errors encountered while constructing or modifying a symbolic
/// environment.
///
/// These errors concern the environment's integrity. They do not replace
/// source-language diagnostics or semantic type-checking errors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SymbolicEnvironmentError {
    /// A binding conflicts with a different existing value.
    ConflictingBinding {
        /// The symbolic identity being rebound.
        id: HdlId,
    },

    /// A definition conflicts with an existing binding.
    ///
    /// A symbol cannot simultaneously have an authoritative concrete
    /// binding and a separate unevaluated definition in this environment.
    DefinitionConflictsWithBinding {
        /// The symbolic identity.
        id: HdlId,
    },

    /// A second definition attempts to reuse an existing definition identity.
    DuplicateDefinition {
        /// The symbolic identity.
        id: HdlId,
    },
}

impl fmt::Display for SymbolicEnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConflictingBinding { id } => {
                write!(f, "conflicting symbolic value binding for `{id}`")
            }
            Self::DefinitionConflictsWithBinding { id } => {
                write!(
                    f,
                    "symbolic definition `{id}` conflicts with an existing \
                     concrete binding"
                )
            }
            Self::DuplicateDefinition { id } => {
                write!(f, "duplicate symbolic definition for `{id}`")
            }
        }
    }
}

impl Error for SymbolicEnvironmentError {}

/// Errors encountered while resolving a symbolic dependency graph.
///
/// The evaluator's error type remains generic so that callers can preserve
/// their existing expression-evaluation error contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SymbolicResolutionError<E> {
    /// The requested symbol or one of its dependencies has no binding
    /// and no symbolic definition.
    Unresolved {
        /// The identity that could not be resolved.
        id: HdlId,
    },

    /// A dependency cycle prevents evaluation.
    ///
    /// The cycle path begins and ends with the same identity when the
    /// complete path can be constructed.
    DependencyCycle {
        /// The identities participating in the discovered cycle.
        path: Vec<HdlId>,
    },

    /// Evaluation of a definition failed.
    ///
    /// The expression identity and original evaluator error are retained.
    EvaluationFailed {
        /// The expression whose evaluation failed.
        expression: HdlId,

        /// The error returned by the caller's evaluator.
        source: E,
    },

    /// An internal graph traversal invariant was violated.
    ///
    /// This is an implementation error, not an indication that user
    /// source code is semantically invalid.
    InternalInvariantViolation,
}

impl<E: fmt::Display> fmt::Display for SymbolicResolutionError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unresolved { id } => {
                write!(f, "symbolic value `{id}` is unresolved")
            }
            Self::DependencyCycle { path } => {
                f.write_str("symbolic dependency cycle: ")?;

                for (index, id) in path.iter().enumerate() {
                    if index != 0 {
                        f.write_str(" -> ")?;
                    }

                    write!(f, "{id}")?;
                }

                Ok(())
            }
            Self::EvaluationFailed { expression, source } => {
                write!(
                    f,
                    "failed to evaluate symbolic expression `{expression}`: \
                     {source}"
                )
            }
            Self::InternalInvariantViolation => {
                f.write_str("symbolic dependency traversal invariant violated")
            }
        }
    }
}

impl<E> Error for SymbolicResolutionError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EvaluationFailed { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Environment containing concrete symbolic-value bindings and unevaluated
/// symbolic definitions.
///
/// `T` is the caller's concrete value representation. It may be a
/// domain-specific constant, a canonical semantic value, or another
/// explicitly chosen type.
///
/// Concrete values are not interpreted by this environment. In particular,
/// arithmetic, type conversions, overflow policy, and hardware-width
/// semantics remain the responsibility of the supplied evaluator and the
/// existing semantic contracts.
///
/// The ordered maps provide deterministic symbol enumeration and stable
/// dependency traversal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolicEnvironment<T> {
    bindings: BTreeMap<HdlId, T>,
    definitions: BTreeMap<HdlId, SymbolicValue>,
}

impl<T> Default for SymbolicEnvironment<T> {
    fn default() -> Self {
        Self {
            bindings: BTreeMap::new(),
            definitions: BTreeMap::new(),
        }
    }
}

impl<T> SymbolicEnvironment<T> {
    /// Create an empty symbolic environment.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind a symbol to a concrete value.
    ///
    /// Repeating an equal binding is idempotent. A conflicting binding
    /// returns an error and preserves the original value.
    ///
    /// A symbol with an existing symbolic definition cannot be bound
    /// directly. Resolve the definition first, then explicitly construct
    /// the desired concrete environment.
    pub fn bind(
        &mut self,
        id: HdlId,
        value: T,
    ) -> Result<(), SymbolicEnvironmentError>
    where
        T: PartialEq,
    {
        if self.definitions.contains_key(&id) {
            return Err(
                SymbolicEnvironmentError::DefinitionConflictsWithBinding { id },
            );
        }

        match self.bindings.get(&id) {
            Some(existing) if existing == &value => Ok(()),
            Some(_) => Err(SymbolicEnvironmentError::ConflictingBinding { id }),
            None => {
                self.bindings.insert(id, value);
                Ok(())
            }
        }
    }

    /// Register an unevaluated symbolic definition.
    ///
    /// Replacing a definition is deliberately not implicit. A second
    /// definition for the same identity returns an error, even if its
    /// contents happen to be identical.
    ///
    /// Dependencies need not already exist when the definition is
    /// registered. This permits forward references and mutually dependent
    /// declarations to be represented. Missing references and cycles are
    /// reported when resolution is requested.
    pub fn define(
        &mut self,
        id: HdlId,
        definition: SymbolicValue,
    ) -> Result<(), SymbolicEnvironmentError> {
        if self.bindings.contains_key(&id) {
            return Err(
                SymbolicEnvironmentError::DefinitionConflictsWithBinding { id },
            );
        }

        if self.definitions.contains_key(&id) {
            return Err(SymbolicEnvironmentError::DuplicateDefinition { id });
        }

        self.definitions.insert(id, definition);
        Ok(())
    }

    /// Return a concrete binding if one exists.
    #[must_use]
    pub fn get(&self, id: &HdlId) -> Option<&T> {
        self.bindings.get(id)
    }

    /// Return a symbolic definition if one exists.
    #[must_use]
    pub fn definition(&self, id: &HdlId) -> Option<&SymbolicValue> {
        self.definitions.get(id)
    }

    /// Return the number of concrete bindings.
    #[must_use]
    pub fn binding_count(&self) -> usize {
        self.bindings.len()
    }

    /// Return the number of symbolic definitions.
    #[must_use]
    pub fn definition_count(&self) -> usize {
        self.definitions.len()
    }

    /// Return whether the environment has no bindings or definitions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty() && self.definitions.is_empty()
    }

    /// Return concrete bindings in deterministic identity order.
    pub fn bindings(&self) -> impl Iterator<Item = (&HdlId, &T)> {
        self.bindings.iter()
    }

    /// Return definitions in deterministic identity order.
    pub fn definitions(&self) -> impl Iterator<Item = (&HdlId, &SymbolicValue)> {
        self.definitions.iter()
    }

    /// Remove all bindings and definitions.
    ///
    /// This changes only the environment. It does not modify source
    /// expressions, the canonical AST, or semantic declarations.
    pub fn clear(&mut self) {
        self.bindings.clear();
        self.definitions.clear();
    }
}

impl<T: Clone> SymbolicEnvironment<T> {
    /// Resolve a symbol and its transitive dependencies.
    ///
    /// The supplied evaluator receives:
    ///
    /// - The canonical expression identity.
    /// - A map containing exactly the declared dependencies and their
    ///   resolved concrete values.
    ///
    /// Dependencies are evaluated before their dependents. A definition
    /// is evaluated at most once during a single call to this method.
    /// Already bound values are reused without reevaluation.
    ///
    /// The environment is not mutated by resolution. If evaluation fails,
    /// the caller receives the error and the environment remains unchanged.
    ///
    /// The evaluator must implement the actual expression semantics,
    /// including type checks, numeric rules, and checked arithmetic.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolicResolutionError::Unresolved`] for a missing
    /// dependency, [`SymbolicResolutionError::DependencyCycle`] for a
    /// cycle, or [`SymbolicResolutionError::EvaluationFailed`] when the
    /// supplied evaluator rejects an expression.
    pub fn resolve<E, F>(
        &self,
        target: &HdlId,
        mut evaluate: F,
    ) -> Result<T, SymbolicResolutionError<E>>
    where
        F: FnMut(&HdlId, &BTreeMap<HdlId, T>) -> Result<T, E>,
    {
        if let Some(value) = self.bindings.get(target) {
            return Ok(value.clone());
        }

        if !self.definitions.contains_key(target) {
            return Err(SymbolicResolutionError::Unresolved {
                id: target.clone(),
            });
        }

        // Iterative depth-first traversal. The active path detects cycles;
        // the completed set prevents repeated traversal of shared subgraphs.
        //
        // Each frame stores the node being visited and the next dependency
        // index to inspect. This avoids recursive calls for deep graphs.
        struct Frame {
            id: HdlId,
            next_dependency: usize,
        }

        let mut frames = vec![Frame {
            id: target.clone(),
            next_dependency: 0,
        }];

        let mut active = BTreeMap::<HdlId, usize>::new();
        active.insert(target.clone(), 0);

        let mut completed = BTreeSet::<HdlId>::new();
        let mut order = Vec::<HdlId>::new();

        while let Some(frame) = frames.last_mut() {
            let id = frame.id.clone();

            if self.bindings.contains_key(&id) || completed.contains(&id) {
                frames.pop();
                active.remove(&id);
                continue;
            }

            let definition = self
                .definitions
                .get(&id)
                .ok_or_else(|| SymbolicResolutionError::Unresolved {
                    id: id.clone(),
                })?;

            if frame.next_dependency < definition.dependencies.len() {
                let dependency =
                    definition.dependencies[frame.next_dependency].clone();

                frame.next_dependency += 1;

                if self.bindings.contains_key(&dependency)
                    || completed.contains(&dependency)
                {
                    continue;
                }

                if let Some(&cycle_start) = active.get(&dependency) {
                    let mut path: Vec<HdlId> = frames
                        .iter()
                        .skip(cycle_start)
                        .map(|entry| entry.id.clone())
                        .collect();

                    path.push(dependency);

                    return Err(
                        SymbolicResolutionError::DependencyCycle { path },
                    );
                }

                if !self.definitions.contains_key(&dependency) {
                    return Err(SymbolicResolutionError::Unresolved {
                        id: dependency,
                    });
                }

                active.insert(dependency.clone(), frames.len());
                frames.push(Frame {
                    id: dependency,
                    next_dependency: 0,
                });
            } else {
                let finished = frames
                    .pop()
                    .ok_or(
                        SymbolicResolutionError::InternalInvariantViolation,
                    )?;

                active.remove(&finished.id);
                completed.insert(finished.id.clone());
                order.push(finished.id);
            }
        }

        // Evaluate in dependency-first order. Only the dependencies
        // declared by each definition are passed to the evaluator.
        //
        // `resolved` is local to this invocation, so a failed evaluation
        // cannot leave partially computed values in the environment.
        let mut resolved = BTreeMap::<HdlId, T>::new();

        for id in order {
            if self.bindings.contains_key(&id) {
                continue;
            }

            let definition = self
                .definitions
                .get(&id)
                .ok_or(
                    SymbolicResolutionError::InternalInvariantViolation,
                )?;

            let mut dependency_values = BTreeMap::<HdlId, T>::new();

            for dependency in &definition.dependencies {
                let value = self
                    .bindings
                    .get(dependency)
                    .or_else(|| resolved.get(dependency))
                    .ok_or_else(|| {
                        SymbolicResolutionError::InternalInvariantViolation
                    })?;

                dependency_values.insert(dependency.clone(), value.clone());
            }

            let value = evaluate(
                definition.expression(),
                &dependency_values,
            )
            .map_err(|source| {
                SymbolicResolutionError::EvaluationFailed {
                    expression: definition.expression().clone(),
                    source,
                }
            })?;

            resolved.insert(id, value);
        }

        resolved
            .remove(target)
            .ok_or(SymbolicResolutionError::InternalInvariantViolation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(name: &str) -> HdlId {
        HdlId::new(name).expect("test identity must be valid")
    }

    fn definition(
        expression: &str,
        dependencies: &[&str],
    ) -> SymbolicValue {
        SymbolicValue::new(
            id(expression),
            dependencies.iter().map(|name| id(name)),
        )
    }

    fn evaluate_sum(
        expression: &HdlId,
        dependencies: &BTreeMap<HdlId, u64>,
    ) -> Result<u64, String> {
        let base = match expression.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "sum" => 0,
            "double" => 0,
            "constant" => 42,
            other => return Err(format!("unknown test expression `{other}`")),
        };

        if dependencies.is_empty() {
            return Ok(base);
        }

        match expression.as_str() {
            "sum" => dependencies.values().try_fold(0_u64, |total, value| {
                total
                    .checked_add(*value)
                    .ok_or_else(|| "test arithmetic overflow".to_owned())
            }),
            "double" => dependencies
                .values()
                .next()
                .copied()
                .ok_or_else(|| "missing operand".to_owned())?
                .checked_mul(2)
                .ok_or_else(|| "test arithmetic overflow".to_owned()),
            other => Err(format!(
                "unexpected dependencies for test expression `{other}`"
            )),
        }
    }

    #[test]
    fn binding_is_idempotent_for_equal_values() {
        let mut environment = SymbolicEnvironment::new();

        environment.bind(id("WIDTH"), 32_u64).unwrap();
        environment.bind(id("WIDTH"), 32_u64).unwrap();

        assert_eq!(environment.get(&id("WIDTH")), Some(&32));
        assert_eq!(environment.binding_count(), 1);
    }

    #[test]
    fn conflicting_binding_preserves_original_value() {
        let mut environment = SymbolicEnvironment::new();

        environment.bind(id("WIDTH"), 32_u64).unwrap();

        assert_eq!(
            environment.bind(id("WIDTH"), 64_u64),
            Err(SymbolicEnvironmentError::ConflictingBinding {
                id: id("WIDTH"),
            })
        );

        assert_eq!(environment.get(&id("WIDTH")), Some(&32));
    }

    #[test]
    fn definition_cannot_overwrite_binding() {
        let mut environment = SymbolicEnvironment::new();

        environment.bind(id("WIDTH"), 32_u64).unwrap();

        assert_eq!(
            environment.define(
                id("WIDTH"),
                definition("constant", &[])
            ),
            Err(
                SymbolicEnvironmentError::DefinitionConflictsWithBinding {
                    id: id("WIDTH"),
                }
            )
        );
    }

    #[test]
    fn duplicate_definition_is_rejected() {
        let mut environment = SymbolicEnvironment::new();

        environment
            .define(id("A"), definition("one", &[]))
            .unwrap();

        assert_eq!(
            environment.define(id("A"), definition("two", &[])),
            Err(SymbolicEnvironmentError::DuplicateDefinition {
                id: id("A"),
            })
        );
    }

    #[test]
    fn resolves_direct_binding() {
        let mut environment = SymbolicEnvironment::new();

        environment.bind(id("WIDTH"), 128_u64).unwrap();

        assert_eq!(
            environment.resolve(&id("WIDTH"), evaluate_sum),
            Ok(128)
        );
    }

    #[test]
    fn resolves_definition_without_dependencies() {
        let mut environment = SymbolicEnvironment::new();

        environment
            .define(id("VALUE"), definition("constant", &[]))
            .unwrap();

        assert_eq!(
            environment.resolve(&id("VALUE"), evaluate_sum),
            Ok(42)
        );
    }

    #[test]
    fn resolves_dependency_chain() {
        let mut environment = SymbolicEnvironment::new();

        environment.bind(id("BASE"), 8_u64).unwrap();

        environment
            .define(id("DOUBLE"), definition("double", &["BASE"]))
            .unwrap();

        environment
            .define(id("RESULT"), definition("double", &["DOUBLE"]))
            .unwrap();

        assert_eq!(
            environment.resolve(&id("RESULT"), evaluate_sum),
            Ok(32)
        );
    }

    #[test]
    fn resolves_multiple_dependencies_deterministically() {
        let mut environment = SymbolicEnvironment::new();

        environment.bind(id("A"), 10_u64).unwrap();
        environment.bind(id("B"), 20_u64).unwrap();

        environment
            .define(id("TOTAL"), definition("sum", &["B", "A", "A"]))
            .unwrap();

        // Duplicate dependency A is removed by the constructor.
        assert_eq!(
            environment.resolve(&id("TOTAL"), evaluate_sum),
            Ok(30)
        );

        assert_eq!(
            environment
                .definition(&id("TOTAL"))
                .unwrap()
                .dependencies(),
            &[id("A"), id("B")]
        );
    }

    #[test]
    fn unresolved_reference_is_not_guessed() {
        let mut environment = SymbolicEnvironment::<u64>::new();

        environment
            .define(id("RESULT"), definition("double", &["MISSING"]))
            .unwrap();

        assert_eq!(
            environment.resolve(&id("RESULT"), evaluate_sum),
            Err(SymbolicResolutionError::Unresolved {
                id: id("MISSING"),
            })
        );
    }

    #[test]
    fn detects_self_cycle() {
        let mut environment = SymbolicEnvironment::<u64>::new();

        environment
            .define(id("A"), definition("double", &["A"]))
            .unwrap();

        assert_eq!(
            environment.resolve(&id("A"), evaluate_sum),
            Err(SymbolicResolutionError::DependencyCycle {
                path: vec![id("A"), id("A")],
            })
        );
    }

    #[test]
    fn detects_multi_node_cycle() {
        let mut environment = SymbolicEnvironment::<u64>::new();

        environment
            .define(id("A"), definition("double", &["B"]))
            .unwrap();

        environment
            .define(id("B"), definition("double", &["C"]))
            .unwrap();

        environment
            .define(id("C"), definition("double", &["A"]))
            .unwrap();

        assert_eq!(
            environment.resolve(&id("A"), evaluate_sum),
            Err(SymbolicResolutionError::DependencyCycle {
                path: vec![id("A"), id("B"), id("C"), id("A")],
            })
        );
    }

    #[test]
    fn evaluator_error_is_preserved() {
        let mut environment = SymbolicEnvironment::<u64>::new();

        environment
            .define(id("UNKNOWN"), definition("not_registered", &[]))
            .unwrap();

        assert_eq!(
            environment.resolve(&id("UNKNOWN"), evaluate_sum),
            Err(SymbolicResolutionError::EvaluationFailed {
                expression: id("not_registered"),
                source: "unknown test expression `not_registered`".to_owned(),
            })
        );
    }

    #[test]
    fn failed_resolution_does_not_mutate_environment() {
        let mut environment = SymbolicEnvironment::<u64>::new();

        environment
            .define(id("RESULT"), definition("double", &["MISSING"]))
            .unwrap();

        let before = environment.clone();

        let _ = environment.resolve(&id("RESULT"), evaluate_sum);

        assert_eq!(environment, before);
    }

    #[test]
    fn shared_dependency_is_evaluated_once_per_resolution() {
        let mut environment = SymbolicEnvironment::new();

        environment
            .define(id("SHARED"), definition("constant", &[]))
            .unwrap();

        environment
            .define(id("LEFT"), definition("double", &["SHARED"]))
            .unwrap();

        environment
            .define(id("RIGHT"), definition("double", &["SHARED"]))
            .unwrap();

        environment
            .define(id("ROOT"), definition("sum", &["LEFT", "RIGHT"]))
            .unwrap();

        let mut calls = BTreeMap::<HdlId, usize>::new();

        let result = environment.resolve(
            &id("ROOT"),
            |expression, dependencies| {
                *calls.entry(expression.clone()).or_default() += 1;
                evaluate_sum(expression, dependencies)
            },
        );

        assert_eq!(result, Ok(168));
        assert_eq!(calls.get(&id("constant")), Some(&1));
    }

    #[test]
    fn supports_a_deep_dependency_chain_without_recursive_traversal() {
        let mut environment = SymbolicEnvironment::new();

        let count = 2_000_usize;

        environment
            .define(id("NODE_0"), definition("one", &[]))
            .unwrap();

        for index in 1..count {
            let current = format!("NODE_{index}");
            let previous = format!("NODE_{}", index - 1);

            environment
                .define(
                    id(&current),
                    SymbolicValue::new(id("double"), [id(&previous)]),
                )
                .unwrap();
        }

        // This chain deliberately exceeds common recursive traversal depths.
        // Evaluation is iterative; the evaluator detects numeric overflow.
        let result = environment.resolve(
            &id(&format!("NODE_{}", count - 1)),
            |expression, dependencies| -> Result<u64, String> {
                match expression.as_str() {
                    "one" => Ok(1),
                    "double" => dependencies
                        .values()
                        .next()
                        .copied()
                        .ok_or_else(|| "missing operand".to_owned())?
                        .checked_mul(2)
                        .ok_or_else(|| "arithmetic overflow".to_owned()),
                    other => Err(format!("unexpected expression `{other}`")),
                }
            },
        );

        assert!(matches!(
            result,
            Err(SymbolicResolutionError::EvaluationFailed { .. })
        ));
    }

    #[test]
    fn empty_environment_is_reported_as_unresolved() {
        let environment = SymbolicEnvironment::<u64>::new();

        assert_eq!(
            environment.resolve(&id("UNKNOWN"), evaluate_sum),
            Err(SymbolicResolutionError::Unresolved {
                id: id("UNKNOWN"),
            })
        );
    }

    #[test]
    fn clear_removes_bindings_and_definitions() {
        let mut environment = SymbolicEnvironment::new();

        environment.bind(id("WIDTH"), 32_u64).unwrap();
        environment
            .define(id("VALUE"), definition("constant", &[]))
            .unwrap();

        environment.clear();

        assert!(environment.is_empty());
        assert_eq!(environment.binding_count(), 0);
        assert_eq!(environment.definition_count(), 0);
    }
}