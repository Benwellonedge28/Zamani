
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani semantic analysis — implicit type coercions.
//!
//! Path: src/frontend/semantic/types/coercions.rs
//!
//! STATUS
//!   New implementation candidate. Freeze after module integration,
//!   specification conformance, and workspace tests pass.
//!
//! PURPOSE
//!   Determine whether a source semantic type can be implicitly converted
//!   to a target semantic type under the registered language rules.
//!
//! AUTHORITY
//!   - grammar/spec/type-system.md
//!   - grammar/specification/types.md
//!   - grammar/specification/semantic-model.md
//!   - grammar/Zamani.g4 and the composed expression/type grammars
//!   - src/frontend/ast/node/types/type_expr.rs
//!   - src/frontend/ast/node/expressions/cast.rs
//!   - src/frontend/semantic/types/ty.rs
//!
//! OWNS
//!   - The implicit-coercion rule interface.
//!   - A deterministic, extensible registry of conversion rules.
//!   - Direct coercion selection and ambiguity detection.
//!   - Explicit handling of incomplete types and operational budgets.
//!   - The semantic plan describing a selected coercion.
//!
//! DOES_NOT_OWN
//!   - Source parsing or AST construction.
//!   - A competing source or semantic type hierarchy.
//!   - Type inference or inference-variable substitution.
//!   - Type unification or trait solving.
//!   - The general assignability relation.
//!   - Explicit-cast execution or runtime conversion.
//!   - Constant evaluation.
//!   - Effect authorization.
//!   - Hardware availability, physical resource allocation, or lowering.
//!
//! INTEGRATION
//!   - ty.rs owns SemanticType.
//!   - compatibility.rs owns type equality/assignability decisions.
//!   - unification.rs owns inference-variable substitutions.
//!   - inference.rs requests coercion decisions after type inference has
//!     established the relevant source and target types.
//!   - traits.rs and domain extensions may provide registered rules.
//!   - diagnostics translate the structured outcomes below into diagnostics.
//!   - expression validation records the selected plan in semantic facts.
//!   - explicit casts are validated separately from implicit coercions.
//!
//! SEMANTIC INVARIANTS
//!   - A conversion is permitted only by an explicitly registered rule.
//!   - Identity is not confused with a conversion.
//!   - Incomplete types cannot prove that a coercion is valid.
//!   - Potentially lossy conversions are rejected by default.
//!   - Runtime-checked conversions require explicit policy permission.
//!   - Equal-ranked applicable rules are ambiguous; registration order
//!     never decides the meaning of a program.
//!   - Conversion chains are not inferred automatically.
//!   - This module plans conversions; it never executes them.
//!   - No hardware capacities or finite domain inventories are encoded.
//!
//! SCALABILITY
//!   No language-level limit is imposed on type arity, numeric width,
//!   computational domain, or target resource capacity.
//!   An optional rule-evaluation budget is an operational safety policy.
//!   Exceeding it is an explicit incomplete-analysis error, never success.
//!
//! FREEZE CRITERIA
//!   - The module is exposed through types/mod.rs.
//!   - Every rule is deterministic, documented, and tested.
//!   - Ambiguity, incomplete types, disallowed conversions, and exhausted
//!     budgets are tested.
//!   - Integration tests cover inference, compatibility, diagnostics,
//!     explicit casts, and semantic-model fact recording.
//!   - The full workspace builds and passes the supported CI checks.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::Arc;

use super::ty::{PrimitiveType, SemanticType};

/// Identifies the semantic context in which a coercion is requested.
///
/// Names are extensible identifiers, not a closed enumeration of all
/// expression kinds or computational domains.
///
/// Examples of possible sites include assignment, argument passing,
/// return values, and operator operands. The authoritative caller supplies
/// the site appropriate to the canonical AST construct.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoercionSite {
    namespace: String,
    name: String,
}

impl CoercionSite {
    /// Constructs a site identifier.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CoercionError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() || name.trim().is_empty() {
            return Err(CoercionError::InvalidConfiguration(
                "coercion-site namespace and name must be non-empty".into(),
            ));
        }

        Ok(Self { namespace, name })
    }

    /// Returns the site namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the site name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Policy for one coercion query.
///
/// This is a semantic policy, not a target-capacity configuration.
///
/// `max_rule_evaluations` limits work performed by one query. It does not
/// restrict the types or programs that the language can express.
///
/// `allow_lossy` should only be enabled when the active normative language
/// profile explicitly permits the corresponding implicit conversion.
/// It must not be enabled merely to make a program compile.
///
/// `allow_runtime_checks` permits a rule to request a runtime check.
/// The rule's check must still be represented in the selected plan and
/// implemented by a downstream component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoercionContext {
    pub site: CoercionSite,
    pub allow_lossy: bool,
    pub allow_runtime_checks: bool,
    pub max_rule_evaluations: Option<usize>,
}

impl CoercionContext {
    /// Creates a conservative context for the supplied semantic site.
    #[must_use]
    pub fn new(site: CoercionSite) -> Self {
        Self {
            site,
            allow_lossy: false,
            allow_runtime_checks: false,
            max_rule_evaluations: None,
        }
    }

    /// Sets whether the active language profile permits lossy coercions.
    #[must_use]
    pub fn with_lossy_coercions(mut self, allow: bool) -> Self {
        self.allow_lossy = allow;
        self
    }

    /// Sets whether runtime-checked coercions may be selected.
    #[must_use]
    pub fn with_runtime_checks(mut self, allow: bool) -> Self {
        self.allow_runtime_checks = allow;
        self
    }

    /// Sets the operational budget for evaluating registered rules.
    ///
    /// `None` means no additional rule-count budget.
    /// `Some(0)` is valid and allows a query to succeed only when it does
    /// not need to evaluate any registered rules.
    #[must_use]
    pub fn with_rule_budget(mut self, budget: Option<usize>) -> Self {
        self.max_rule_evaluations = budget;
        self
    }
}

/// Properties declared by a coercion rule.
///
/// `priority` is a language-defined rule-ranking value: lower values are
/// preferred. It is not a machine size, a type width, or a resource limit.
///
/// If two applicable rules have the same best priority, the result is
/// ambiguous. The registry never silently uses insertion order to break
/// a semantic tie.
///
/// A rule must report whether it may lose information and whether it needs
/// a runtime check. Any introduced effects must be returned explicitly for
/// the semantic effect checker to validate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoercionProperties {
    pub priority: u64,
    pub may_lose_information: bool,
    pub requires_runtime_check: bool,
    pub effects: Vec<super::ty::QualifiedName>,
}

impl CoercionProperties {
    /// Creates a value-preserving coercion that needs no runtime check.
    #[must_use]
    pub fn preserving(priority: u64) -> Self {
        Self {
            priority,
            may_lose_information: false,
            requires_runtime_check: false,
            effects: Vec::new(),
        }
    }

    /// Creates a value-preserving coercion that requires a runtime check.
    #[must_use]
    pub fn checked(priority: u64) -> Self {
        Self {
            priority,
            may_lose_information: false,
            requires_runtime_check: true,
            effects: Vec::new(),
        }
    }

    /// Creates a coercion that may lose information.
    ///
    /// Such a rule is not selected under the default context.
    #[must_use]
    pub fn lossy(priority: u64) -> Self {
        Self {
            priority,
            may_lose_information: true,
            requires_runtime_check: false,
            effects: Vec::new(),
        }
    }

    /// Attaches the effects introduced by this conversion.
    ///
    /// The caller remains responsible for validating those effects against
    /// the function's declared effects and the active authorization policy.
    #[must_use]
    pub fn with_effects(
        mut self,
        effects: Vec<super::ty::QualifiedName>,
    ) -> Self {
        self.effects = effects;
        self
    }
}

/// Outcome returned by a rule when inspecting a source/target pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleMatch {
    /// This rule does not apply to the pair.
    NotApplicable,

    /// This rule applies with the declared properties.
    Applicable(CoercionProperties),

    /// The rule might apply, but available semantic information is
    /// insufficient to decide safely.
    ///
    /// For example, a generic conversion rule might need a trait obligation
    /// or a symbolic constraint to be resolved first.
    Undetermined {
        reason: String,
    },
}

/// A registered coercion rule.
///
/// Rules must be deterministic and side-effect free. They must not perform
/// I/O, inspect hardware, execute source expressions, mutate global state,
/// or use registration order to affect their result.
///
/// A rule may inspect the full canonical SemanticType model. Generic,
/// domain-specific, and future conversion rules can therefore be added
/// without modifying this registry's core implementation.
///
/// Rule implementations should avoid expensive recursive operations on
/// untrusted type structures. Any potentially expensive analysis must use
/// the semantic subsystem's configured work budgets.
pub trait CoercionRule: Send + Sync {
    /// Stable, non-empty rule identifier.
    ///
    /// Use a qualified identifier such as `core.numeric.safe-widening`.
    /// Identifiers are unique within one registry.
    fn id(&self) -> &str;

    /// Determines whether this rule applies.
    ///
    /// `Err` represents a rule implementation failure, not a normal
    /// incompatibility. The registry propagates it rather than silently
    /// treating a broken rule as if it did not exist.
    fn evaluate(
        &self,
        source: &SemanticType,
        target: &SemanticType,
        context: &CoercionContext,
    ) -> Result<RuleMatch, String>;
}

/// A selected direct conversion.
///
/// This describes a semantic conversion request. It does not execute the
/// conversion or prescribe its physical implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoercionPlan {
    pub source: SemanticType,
    pub target: SemanticType,
    pub rule_id: String,
    pub properties: CoercionProperties,
}

/// A reason why a coercion query remains unresolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleIndeterminacy {
    pub rule_id: String,
    pub reason: String,
}

/// The result of querying the implicit-coercion registry.
///
/// `NoCoercion` means no registered rule was proven applicable under the
/// supplied context. The caller should then use `compatibility.rs` to
/// distinguish ordinary incompatibility from a case that requires an
/// explicit cast.
///
/// `Undetermined` is not success. Semantic analysis must resolve the
/// missing information or report an incomplete/invalid analysis.
///
/// `Ambiguous` must be diagnosed rather than resolved by registry order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoercionDecision {
    /// The types are already identical and complete.
    Identity,

    /// Exactly one best-ranked direct conversion was established.
    Convert(CoercionPlan),

    /// No permitted implicit conversion was established.
    NoCoercion,

    /// At least one rule could not safely decide whether it applies.
    Undetermined {
        reasons: Vec<RuleIndeterminacy>,
    },

    /// Multiple applicable rules have the same best priority.
    Ambiguous {
        candidates: Vec<CoercionPlan>,
    },
}

/// Failures of the registry or its operational policy.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CoercionError {
    InvalidConfiguration(String),
    InvalidRuleId,
    DuplicateRuleId(String),

    WorkBudgetExceeded {
        budget: usize,
        evaluated: usize,
    },

    RuleFailure {
        rule_id: String,
        message: String,
    },
}

impl fmt::Display for CoercionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(f, "invalid coercion configuration: {message}")
            }
            Self::InvalidRuleId => {
                f.write_str("coercion rule identifier must be non-empty")
            }
            Self::DuplicateRuleId(id) => {
                write!(f, "duplicate coercion rule identifier `{id}`")
            }
            Self::WorkBudgetExceeded { budget, evaluated } => {
                write!(
                    f,
                    "coercion rule-evaluation budget exceeded: \
                     budget {budget}, already evaluated {evaluated}"
                )
            }
            Self::RuleFailure { rule_id, message } => {
                write!(
                    f,
                    "coercion rule `{rule_id}` failed: {message}"
                )
            }
        }
    }
}

impl Error for CoercionError {}

/// Deterministic registry for direct implicit-coercion rules.
///
/// One registry should be constructed for a language-version and extension
/// configuration, then shared immutably for an analysis session.
///
/// Register rules before concurrent analysis begins. Rule IDs are stored
/// in an ordered map, so evaluation order is stable. The final decision
/// does not depend on that order.
#[derive(Default)]
pub struct CoercionRegistry {
    rules: BTreeMap<String, Arc<dyn CoercionRule>>,
}

impl fmt::Debug for CoercionRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CoercionRegistry")
            .field("rule_ids", &self.rules.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl CoercionRegistry {
    /// Creates an empty registry.
    ///
    /// An empty registry has no non-identity implicit conversions. This
    /// deliberate default avoids inventing language rules that have not
    /// been approved by the normative type-system specification.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of registered rules.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Returns whether the registry contains no rules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Returns registered rule IDs in deterministic order.
    #[must_use]
    pub fn rule_ids(&self) -> Vec<&str> {
        self.rules.keys().map(String::as_str).collect()
    }

    /// Registers a rule implementation.
    pub fn register<R>(&mut self, rule: R) -> Result<(), CoercionError>
    where
        R: CoercionRule + 'static,
    {
        self.register_shared(Arc::new(rule))
    }

    /// Registers a shared rule implementation.
    ///
    /// The registry owns the shared reference. The caller must ensure that
    /// the rule's behavior is deterministic and remains stable after
    /// registration.
    pub fn register_shared(
        &mut self,
        rule: Arc<dyn CoercionRule>,
    ) -> Result<(), CoercionError> {
        let id = rule.id().trim();

        if id.is_empty() {
            return Err(CoercionError::InvalidRuleId);
        }

        let id = id.to_owned();

        if self.rules.contains_key(&id) {
            return Err(CoercionError::DuplicateRuleId(id));
        }

        self.rules.insert(id, rule);
        Ok(())
    }

    /// Resolves one direct implicit-coercion query.
    ///
    /// This method does not automatically compose multiple rules into a
    /// conversion chain. Such composition can change rounding, overflow,
    /// effects, ownership, or domain semantics and must be authorized by
    /// an explicit language rule.
    ///
    /// If any rule returns `Undetermined`, the result remains undetermined.
    /// Selecting a currently applicable candidate while ignoring another
    /// rule that could outrank it would be unsound.
    pub fn resolve(
        &self,
        source: &SemanticType,
        target: &SemanticType,
        context: &CoercionContext,
    ) -> Result<CoercionDecision, CoercionError> {
        // Do not certify unresolved inference variables, error-recovery
        // types, unresolved associated types, or contextual Self types.
        if type_is_incomplete(source) || type_is_incomplete(target) {
            return Ok(CoercionDecision::Undetermined {
                reasons: vec![RuleIndeterminacy {
                    rule_id: "<semantic-type-state>".into(),
                    reason: "source or target type is incomplete".into(),
                }],
            });
        }

        if source == target {
            return Ok(CoercionDecision::Identity);
        }

        let mut evaluated = 0usize;
        let mut candidates = Vec::new();
        let mut unresolved = Vec::new();

        for (id, rule) in &self.rules {
            if let Some(budget) = context.max_rule_evaluations {
                if evaluated >= budget {
                    return Err(CoercionError::WorkBudgetExceeded {
                        budget,
                        evaluated,
                    });
                }
            }

            evaluated = evaluated.saturating_add(1);

            let outcome = rule.evaluate(source, target, context).map_err(
                |message| CoercionError::RuleFailure {
                    rule_id: id.clone(),
                    message,
                },
            )?;

            match outcome {
                RuleMatch::NotApplicable => {}

                RuleMatch::Undetermined { reason } => {
                    unresolved.push(RuleIndeterminacy {
                        rule_id: id.clone(),
                        reason,
                    });
                }

                RuleMatch::Applicable(properties) => {
                    // A policy may forbid a conversion even when a rule
                    // recognizes it. A forbidden conversion is not a
                    // candidate for implicit selection.
                    if properties.may_lose_information && !context.allow_lossy {
                        continue;
                    }

                    if properties.requires_runtime_check
                        && !context.allow_runtime_checks
                    {
                        continue;
                    }

                    candidates.push(CoercionPlan {
                        source: source.clone(),
                        target: target.clone(),
                        rule_id: id.clone(),
                        properties,
                    });
                }
            }
        }

        if !unresolved.is_empty() {
            unresolved.sort_by(|left, right| {
                left.rule_id
                    .cmp(&right.rule_id)
                    .then_with(|| left.reason.cmp(&right.reason))
            });

            return Ok(CoercionDecision::Undetermined {
                reasons: unresolved,
            });
        }

        if candidates.is_empty() {
            return Ok(CoercionDecision::NoCoercion);
        }

        candidates.sort_by(|left, right| {
            left.properties
                .priority
                .cmp(&right.properties.priority)
                .then_with(|| left.rule_id.cmp(&right.rule_id))
        });

        let best_priority = candidates[0].properties.priority;
        let best_count = candidates
            .iter()
            .take_while(|candidate| candidate.properties.priority == best_priority)
            .count();

        if best_count != 1 {
            return Ok(CoercionDecision::Ambiguous {
                candidates: candidates
                    .into_iter()
                    .take(best_count)
                    .collect(),
            });
        }

        // Exactly one candidate has the best priority.
        Ok(CoercionDecision::Convert(candidates.remove(0)))
    }
}

/// Iteratively checks whether a semantic type contains unresolved or
/// error-tainted type information.
///
/// This avoids using the recursive `SemanticType::is_incomplete()`
/// traversal for this particular gate. It does not make every operation
/// in the wider type system stack-independent; callers must still apply
/// the semantic subsystem's configured work/depth policies where needed.
#[must_use]
pub fn type_is_incomplete(ty: &SemanticType) -> bool {
    let mut pending = vec![ty];

    while let Some(current) = pending.pop() {
        match current {
            SemanticType::Error
            | SemanticType::Variable(_)
            | SemanticType::SelfType
            | SemanticType::Associated { .. } => return true,

            SemanticType::Primitive(PrimitiveType::Complex(inner)) => {
                pending.push(inner);
            }

            SemanticType::Primitive(PrimitiveType::Float(
                super::ty::FloatFormat::Extension { parameters, .. },
            )) => {
                pending.extend(parameters.iter());
            }

            SemanticType::Named { arguments, .. }
            | SemanticType::Tuple(arguments)
            | SemanticType::Union(arguments)
            | SemanticType::Intersection(arguments) => {
                pending.extend(arguments.iter());
            }

            SemanticType::Array { element, .. }
            | SemanticType::Slice(element)
            | SemanticType::Optional(element)
            | SemanticType::Quantum(element)
            | SemanticType::Linear(element)
            | SemanticType::Affine(element)
            | SemanticType::Temporal(element) => {
                pending.push(element);
            }

            SemanticType::Function {
                parameters,
                result,
                ..
            } => {
                pending.extend(parameters.iter().map(|parameter| &parameter.ty));
                pending.push(result);
            }

            SemanticType::Reference { referent, .. } => {
                pending.push(referent);
            }

            SemanticType::Pointer { pointee, .. } => {
                pending.push(pointee);
            }

            SemanticType::Result { ok, error } => {
                pending.push(ok);
                pending.push(error);
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
            } => {
                pending.push(parameter_type);
                pending.push(body);
            }

            SemanticType::Identity { left, right } => {
                pending.push(left);
                pending.push(right);
            }

            SemanticType::TypeApplication {
                constructor,
                arguments,
            } => {
                pending.push(constructor);
                pending.extend(arguments.iter());
            }

            SemanticType::Extension { arguments, .. } => {
                pending.extend(arguments.iter());
            }

            SemanticType::Primitive(_)
            | SemanticType::Parameter(_)
            | SemanticType::Never
            | SemanticType::Unit => {}
        }
    }

    false
}

/// A simple exact-type-pair rule.
///
/// This is useful for a specifically approved conversion between two
/// complete semantic types. It deliberately does not implement numeric
/// promotion or infer conversions from names.
///
/// More general rules can implement `CoercionRule` directly and inspect
/// generic arguments, symbolic constraints, domain metadata, or other
/// language-defined semantic properties.
#[derive(Clone, Debug)]
pub struct ExactTypeRule {
    id: String,
    source: SemanticType,
    target: SemanticType,
    properties: CoercionProperties,
}

impl ExactTypeRule {
    /// Constructs an exact source/target conversion rule.
    ///
    /// The registry validates the rule ID when the rule is registered.
    /// The caller is responsible for registering only conversions allowed
    /// by the active language specification.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        source: SemanticType,
        target: SemanticType,
        properties: CoercionProperties,
    ) -> Self {
        Self {
            id: id.into(),
            source,
            target,
            properties,
        }
    }
}

impl CoercionRule for ExactTypeRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn evaluate(
        &self,
        source: &SemanticType,
        target: &SemanticType,
        _context: &CoercionContext,
    ) -> Result<RuleMatch, String> {
        if source == &self.source && target == &self.target {
            Ok(RuleMatch::Applicable(self.properties.clone()))
        } else {
            Ok(RuleMatch::NotApplicable)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::types::ty::PrimitiveType;

    fn site() -> CoercionSite {
        CoercionSite::new("core", "test").expect("valid test site")
    }

    fn boolean() -> SemanticType {
        SemanticType::Primitive(PrimitiveType::Boolean)
    }

    fn string() -> SemanticType {
        SemanticType::Primitive(PrimitiveType::String)
    }

    fn exact_rule(
        id: &str,
        source: SemanticType,
        target: SemanticType,
        priority: u64,
    ) -> ExactTypeRule {
        ExactTypeRule::new(
            id,
            source,
            target,
            CoercionProperties::preserving(priority),
        )
    }

    #[test]
    fn identical_complete_types_need_no_rule() {
        let registry = CoercionRegistry::new();
        let ty = boolean();

        let result = registry
            .resolve(&ty, &ty, &CoercionContext::new(site()))
            .expect("query succeeds");

        assert_eq!(result, CoercionDecision::Identity);
    }

    #[test]
    fn unregistered_conversion_is_not_assumed() {
        let registry = CoercionRegistry::new();

        let result = registry
            .resolve(
                &boolean(),
                &string(),
                &CoercionContext::new(site()),
            )
            .expect("query succeeds");

        assert_eq!(result, CoercionDecision::NoCoercion);
    }

    #[test]
    fn registered_rule_produces_a_direct_plan() {
        let mut registry = CoercionRegistry::new();

        registry
            .register(exact_rule(
                "test.boolean-to-string",
                boolean(),
                string(),
                10,
            ))
            .expect("registration succeeds");

        let result = registry
            .resolve(
                &boolean(),
                &string(),
                &CoercionContext::new(site()),
            )
            .expect("query succeeds");

        match result {
            CoercionDecision::Convert(plan) => {
                assert_eq!(plan.rule_id, "test.boolean-to-string");
                assert_eq!(plan.source, boolean());
                assert_eq!(plan.target, string());
            }
            other => panic!("expected conversion plan, got {other:?}"),
        }
    }

    #[test]
    fn duplicate_rule_ids_are_rejected() {
        let mut registry = CoercionRegistry::new();

        registry
            .register(exact_rule(
                "test.same",
                boolean(),
                string(),
                1,
            ))
            .expect("first registration succeeds");

        let result = registry.register(exact_rule(
            "test.same",
            boolean(),
            string(),
            2,
        ));

        assert!(matches!(
            result,
            Err(CoercionError::DuplicateRuleId(_))
        ));
    }

    #[test]
    fn equal_best_priorities_are_ambiguous() {
        let mut registry = CoercionRegistry::new();

        registry
            .register(exact_rule(
                "test.first",
                boolean(),
                string(),
                10,
            ))
            .expect("registration succeeds");

        registry
            .register(exact_rule(
                "test.second",
                boolean(),
                string(),
                10,
            ))
            .expect("registration succeeds");

        let result = registry
            .resolve(
                &boolean(),
                &string(),
                &CoercionContext::new(site()),
            )
            .expect("query succeeds");

        match result {
            CoercionDecision::Ambiguous { candidates } => {
                assert_eq!(candidates.len(), 2);
                assert!(candidates[0].rule_id < candidates[1].rule_id);
            }
            other => panic!("expected ambiguity, got {other:?}"),
        }
    }

    #[test]
    fn lower_priority_value_wins_when_unique() {
        let mut registry = CoercionRegistry::new();

        registry
            .register(exact_rule(
                "test.lower-preference",
                boolean(),
                string(),
                20,
            ))
            .expect("registration succeeds");

        registry
            .register(exact_rule(
                "test.higher-preference",
                boolean(),
                string(),
                5,
            ))
            .expect("registration succeeds");

        let result = registry
            .resolve(
                &boolean(),
                &string(),
                &CoercionContext::new(site()),
            )
            .expect("query succeeds");

        match result {
            CoercionDecision::Convert(plan) => {
                assert_eq!(plan.rule_id, "test.higher-preference");
            }
            other => panic!("expected selected plan, got {other:?}"),
        }
    }

    #[test]
    fn lossy_conversion_is_rejected_by_default() {
        let mut registry = CoercionRegistry::new();

        registry
            .register(ExactTypeRule::new(
                "test.lossy",
                boolean(),
                string(),
                CoercionProperties::lossy(1),
            ))
            .expect("registration succeeds");

        let result = registry
            .resolve(
                &boolean(),
                &string(),
                &CoercionContext::new(site()),
            )
            .expect("query succeeds");

        assert_eq!(result, CoercionDecision::NoCoercion);
    }

    #[test]
    fn runtime_checked_conversion_requires_permission() {
        let mut registry = CoercionRegistry::new();

        registry
            .register(ExactTypeRule::new(
                "test.checked",
                boolean(),
                string(),
                CoercionProperties::checked(1),
            ))
            .expect("registration succeeds");

        let denied = registry
            .resolve(
                &boolean(),
                &string(),
                &CoercionContext::new(site()),
            )
            .expect("query succeeds");

        assert_eq!(denied, CoercionDecision::NoCoercion);

        let allowed = registry
            .resolve(
                &boolean(),
                &string(),
                &CoercionContext::new(site()).with_runtime_checks(true),
            )
            .expect("query succeeds");

        assert!(matches!(
            allowed,
            CoercionDecision::Convert(CoercionPlan {
                properties: CoercionProperties {
                    requires_runtime_check: true,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn zero_budget_does_not_silently_skip_rules() {
        let mut registry = CoercionRegistry::new();

        registry
            .register(exact_rule(
                "test.rule",
                boolean(),
                string(),
                1,
            ))
            .expect("registration succeeds");

        let result = registry.resolve(
            &boolean(),
            &string(),
            &CoercionContext::new(site()).with_rule_budget(Some(0)),
        );

        assert!(matches!(
            result,
            Err(CoercionError::WorkBudgetExceeded {
                budget: 0,
                evaluated: 0
            })
        ));
    }

    #[test]
    fn incomplete_types_are_not_certified_by_rules() {
        let mut registry = CoercionRegistry::new();

        registry
            .register(exact_rule(
                "test.rule",
                SemanticType::Error,
                string(),
                1,
            ))
            .expect("registration succeeds");

        let result = registry
            .resolve(
                &SemanticType::Error,
                &string(),
                &CoercionContext::new(site()),
            )
            .expect("query returns an explicit decision");

        assert!(matches!(
            result,
            CoercionDecision::Undetermined { .. }
        ));
    }

    #[test]
    fn rule_ids_are_returned_in_stable_order() {
        let mut registry = CoercionRegistry::new();

        registry
            .register(exact_rule(
                "test.z",
                boolean(),
                string(),
                1,
            ))
            .expect("registration succeeds");

        registry
            .register(exact_rule(
                "test.a",
                boolean(),
                string(),
                2,
            ))
            .expect("registration succeeds");

        assert_eq!(
            registry.rule_ids(),
            vec!["test.a", "test.z"]
        );
    }
}
