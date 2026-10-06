/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/causality.g4
 *
 * Grammar:
 *     AICausality
 *
 * Status:
 *     CANONICAL AI-DOMAIN CAUSAL-COMPUTATION COMPOSITION GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97 or later
 *     Rust edition 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the AI-domain composition boundary for generic causal
 * computation.
 *
 * Causality is treated as a general computational relationship rather than
 * as a particular AI algorithm, causal graph implementation, temporal engine,
 * probability engine, database, simulator, theorem prover, or hardware
 * mechanism.
 *
 * The grammar provides a stable source-level structure for expressing:
 *
 *     - causal relationships;
 *     - causes;
 *     - effects;
 *     - observations;
 *     - interventions;
 *     - counterfactuals;
 *     - causal dependencies;
 *     - causal queries;
 *     - causal assertions;
 *     - causal explanations;
 *     - causal context;
 *     - causal metadata;
 *     - open-world causal extensions.
 *
 * The grammar expresses COMPUTATIONAL INTENT.
 *
 * It does not prescribe how that intent is realized.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     AI
 *          |
 *          v
 *     AICausality                         <-- THIS FILE
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *        types              effects           provenance
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                       causal semantic model
 *                              |
 *              +---------------+---------------+
 *              |               |               |
 *              v               v               v
 *         classical       quantum::ir      other domains
 *              |               |               |
 *              +---------------+---------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                       specialization
 *                              |
 *                    routing / scheduling
 *                              |
 *                    resilience / recovery
 *                              |
 *                         ZQN / HAL
 *                              |
 *                       target realization
 *
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * CAUSALITY IS NOT THE SAME AS TEMPORALITY.
 *
 * A causal relationship may be:
 *
 *     temporal;
 *     logical;
 *     probabilistic;
 *     physical;
 *     computational;
 *     data-derived;
 *     observational;
 *     intervention-based;
 *     counterfactual;
 *     distributed;
 *     quantum-derived;
 *     hardware-derived;
 *     simulation-derived;
 *     domain-defined.
 *
 * Therefore this grammar MUST NOT make causal semantics depend exclusively on
 * the MTS temporal subsystem.
 *
 * Temporal causality is one semantic realization of the general causal model.
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     aiCausalityConstruct
 *     causalStatement
 *     causalExpression
 *     causalOperation
 *     causalOperationName
 *     causalArgumentList
 *     causalArgument
 *     causalNamedArgument
 *     causalContext
 *     causalContextItem
 *     causalRelation
 *     causalRelationOperator
 *     causalObservation
 *     causalIntervention
 *     causalCounterfactual
 *     causalDependency
 *     causalQuery
 *     causalAssertion
 *     causalExplanation
 *
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     - lexer rules;
 *     - keyword spelling;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - expression precedence;
 *     - general expressions;
 *     - general types;
 *     - pattern syntax;
 *     - guards;
 *     - knowledge storage;
 *     - probability mathematics;
 *     - uncertainty mathematics;
 *     - statistical algorithms;
 *     - causal discovery algorithms;
 *     - structural causal models;
 *     - Bayesian networks;
 *     - graph databases;
 *     - theorem provers;
 *     - SAT/SMT solvers;
 *     - machine-learning algorithms;
 *     - neural models;
 *     - reinforcement-learning algorithms;
 *     - temporal storage;
 *     - temporal scheduling;
 *     - simulation engines;
 *     - quantum operation syntax;
 *     - quantum physical mapping;
 *     - QEC;
 *     - HDL syntax;
 *     - hardware realization;
 *     - resource allocation;
 *     - capability discovery;
 *     - policy enforcement;
 *     - provenance storage;
 *     - runtime execution;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - canonical IR;
 *     - quantum::ir;
 *     - ZQN;
 *     - HAL.
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file owns AI-domain causal composition only.
 *
 * It MUST NOT duplicate:
 *
 *     expression
 *     typeExpression
 *     identifier
 *     qualifiedName
 *     statement
 *     reasonStatement
 *     knowledgeExpression
 *     uncertaintyExpression
 *     queryExpression
 *     policyExpression
 *     provenanceExpression
 *
 * Those remain owned by their canonical grammar components.
 *
 *
 * ============================================================================
 * CAUSALITY / REASONING DISTINCTION
 * ============================================================================
 *
 * Generic reasoning answers questions such as:
 *
 *     infer conclusion from evidence;
 *     deduce result from premises;
 *     reason decision from observation;
 *
 * Causal computation additionally represents a directed semantic relationship
 * between computational entities.
 *
 * Therefore:
 *
 *     reasoning != causality
 *
 * although reasoning may consume causal information and causal computation
 * may consume reasoning results.
 *
 * This file must not replace:
 *
 *     grammar/statements/reason.g4
 *
 * and reasoning.g4 must not be made responsible for causal graph semantics.
 *
 *
 * ============================================================================
 * CAUSALITY / TEMPORAL DISTINCTION
 * ============================================================================
 *
 * The repository already contains temporal infrastructure including:
 *
 *     grammar/types/temporal.g4
 *     grammar/memory/temporal.g4
 *     src/toolchain/causality_checker.rs
 *
 * Those components remain valid consumers of causal semantics.
 *
 * They do NOT become the source-language owner of this AI causal grammar.
 *
 * A causal relationship MAY have temporal information, but temporal ordering
 * is not required for every causal relationship.
 *
 * Examples of possible semantic forms include:
 *
 *     causal::cause(source, target)
 *     causal::effect(source, target)
 *     causal::observe(value)
 *     causal::intervene(variable, value)
 *     causal::counterfactual(condition, outcome)
 *     causal::depends_on(value, dependency)
 *
 * The actual semantic interpretation is downstream.
 *
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Causal computation MUST remain extensible.
 *
 * The grammar therefore does NOT enumerate a fixed catalogue of:
 *
 *     causal algorithms;
 *     graph algorithms;
 *     causal estimators;
 *     discovery methods;
 *     intervention methods;
 *     counterfactual solvers;
 *     probability models;
 *     domain ontologies;
 *     scientific theories;
 *     hardware mechanisms.
 *
 * New causal operations can be represented through the generic operation
 * boundary and qualified names.
 *
 * For example:
 *
 *     causal::cause(...)
 *     causal::effect(...)
 *     causal::observe(...)
 *     causal::intervene(...)
 *     causal::counterfactual(...)
 *     causal::depends_on(...)
 *
 * are semantic operation names.
 *
 * Future operations may be introduced without creating a new universal
 * hardware or mathematical ceiling.
 *
 *
 * ============================================================================
 * NAMESPACE CONTRACT
 * ============================================================================
 *
 * The canonical source namespace for the generic causal operation family is:
 *
 *     causal
 *
 * The canonical qualified form is:
 *
 *     causal::operation(...)
 *
 * IMPORTANT:
 *
 * `causal` is intentionally NOT introduced as a global reserved keyword.
 *
 * The repository's open-world name architecture allows semantic namespaces
 * to remain extensible without continually expanding the global keyword set.
 *
 * The semantic layer MUST validate that the first qualification segment of a
 * construct entering this grammar is the canonical causal namespace.
 *
 * This parser-level design avoids:
 *
 *     CAUSAL
 *     CAUSE
 *     EFFECT
 *     INTERVENTION
 *     OBSERVATION
 *     COUNTERFACTUAL
 *     DEPENDENCY
 *
 * becoming mandatory global lexer keywords.
 *
 * This is important for long-term language extensibility.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 *
 * IMPORTS
 * -------
 *
 *     Names
 *     Expressions
 *
 *
 * EXPORTS
 * -------
 *
 *     aiCausalityConstruct
 *     causalStatement
 *     causalExpression
 *     causalOperation
 *     causalOperationName
 *     causalArgumentList
 *     causalArgument
 *     causalNamedArgument
 *     causalContext
 *     causalContextItem
 *     causalRelation
 *     causalRelationOperator
 *     causalObservation
 *     causalIntervention
 *     causalCounterfactual
 *     causalDependency
 *     causalQuery
 *     causalAssertion
 *     causalExplanation
 *
 *
 * CONSUMED BY
 * -----------
 *
 *     grammar/ai/ai.g4
 *
 *
 * AST_OWNER
 * ---------
 *
 * Existing domain-neutral frontend AST.
 *
 * This grammar MUST NOT require an AI-specific AST hierarchy merely because
 * the construct is consumed through the AI domain.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Causal semantic analysis.
 *
 * Semantic analysis integrates with:
 *
 *     type analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract analysis
 *     policy analysis
 *     provenance
 *     temporal semantics
 *     uncertainty/probability semantics
 *     knowledge semantics
 *     reasoning semantics
 *     concurrency semantics
 *     distributed semantics
 *     simulation semantics
 *     quantum semantics
 *     HDL/hardware semantics
 *
 *
 * IR_OWNER
 * --------
 *
 * This grammar owns NO IR.
 *
 * Causal semantics lower into the repository's canonical semantic model.
 *
 * Classical causal computation may lower through the canonical classical
 * representation.
 *
 * Quantum-related causal computation MUST use the canonical quantum semantic
 * boundary and, when quantum operations are represented, must cross:
 *
 *     quantum::ir
 *
 * There MUST NOT be:
 *
 *     causal::ir
 *     ai::causal_ir
 *     quantum_causal_ir
 *
 * merely because causal computation is present.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/ai/causality/
 *
 * Recommended groups:
 *
 *     relation/
 *     observation/
 *     intervention/
 *     counterfactual/
 *     dependency/
 *     query/
 *     assertion/
 *     explanation/
 *     context/
 *     namespace/
 *     quantum/
 *     temporal/
 *     distributed/
 *     scalability/
 *     negative/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/ai.md
 *     grammar/spec/causality.md
 *     grammar/spec/provenance.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *     grammar/spec/policies.md
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * It introduces NO new global tokens.
 *
 * It consumes the existing canonical token vocabulary through:
 *
 *     tokenVocab = ZamaniLexer
 *
 * In particular, the following remain ordinary names unless separately
 * reserved by the canonical lexical specification:
 *
 *     causal
 *     cause
 *     effect
 *     observe
 *     intervene
 *     counterfactual
 *     depends_on
 *     query
 *     assert
 *     explain
 *
 * If any of these words already has a canonical token in the lexer, the
 * semantic/grammar integration must use that canonical token instead of
 * attempting to recreate an identifier token.
 *
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * `qualifiedName` remains owned by Names.
 *
 * A causal operation name is therefore:
 *
 *     qualifiedName
 *
 * and its semantic owner validates:
 *
 *     causal::<operation>
 *
 * or a registered causal extension namespace.
 *
 * This allows future domain extensions without modifying the causal grammar.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * All causal operands are ordinary Zamani expressions.
 *
 * This means causal computation may operate on:
 *
 *     scalars
 *     tuples
 *     records
 *     collections
 *     streams
 *     tensors
 *     datasets
 *     models
 *     knowledge values
 *     uncertain values
 *     probabilistic values
 *     observations
 *     simulation results
 *     classical results
 *     quantum measurements
 *     quantum states where semantically valid
 *     hardware observations
 *     distributed results
 *     future-domain values
 *
 * No AI-specific expression hierarchy is introduced.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file defines NO types.
 *
 * In particular, it must NOT create:
 *
 *     CausalType
 *     CauseType
 *     EffectType
 *     InterventionType
 *     CounterfactualType
 *     CausalGraphType
 *
 * merely to represent causal syntax.
 *
 * Causal values are ordinary Zamani values whose semantic roles are determined
 * downstream.
 *
 * The semantic layer may impose additional requirements such as:
 *
 *     compatible intervention target;
 *     valid observation;
 *     valid counterfactual condition;
 *     compatible cause/effect domains;
 *     valid dependency direction;
 *     supported causal model;
 *
 * without making those concepts parser-level types.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Causal observation is normally observational.
 *
 * Causal intervention is potentially state/model-changing and may therefore
 * carry an appropriate semantic effect.
 *
 * Counterfactual evaluation may require simulation or speculative evaluation.
 *
 * The grammar does not assign effect identities.
 *
 * Semantic analysis must determine effects such as:
 *
 *     observation
 *     mutation
 *     simulation
 *     randomness
 *     network
 *     distributed
 *     quantum
 *     native
 *     foreign
 *     reflection
 *
 * according to the actual operation and semantic implementation.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * The grammar does not require a particular implementation capability.
 *
 * Semantic analysis may derive capabilities such as:
 *
 *     causal.observation
 *     causal.intervention
 *     causal.counterfactual
 *     causal.discovery
 *     causal.inference
 *
 * only when the selected semantic operation actually needs them.
 *
 * These are capability identities, not hardware identifiers.
 *
 * Capability negotiation occurs downstream.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO resource ceiling.
 *
 * In particular it MUST NOT define limits on:
 *
 *     causal nodes
 *     causal edges
 *     observations
 *     interventions
 *     counterfactuals
 *     dependencies
 *     graph depth
 *     graph width
 *     variables
 *     models
 *     samples
 *     datasets
 *     processors
 *     threads
 *     devices
 *     memory
 *     nodes
 *     qubits
 *     tensor dimensions
 *
 * A causal structure may therefore scale according to:
 *
 *     source semantics
 *     representation requirements
 *     declared constraints
 *     compiler resources
 *     runtime resources
 *     target capabilities
 *
 * and not according to a grammar-level finite ceiling.
 *
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Causal constructs may appear inside programs governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * and policy constructs.
 *
 * This grammar does not duplicate those grammars.
 *
 * A semantic causal operation may therefore be:
 *
 *     required;
 *     prohibited;
 *     constrained;
 *     audited;
 *     explained;
 *     provenance-tracked;
 *     authorized;
 *
 * by the universal contract/policy system.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Causal computation should preserve provenance whenever the semantic
 * operation produces a derived causal claim.
 *
 * Provenance may include:
 *
 *     source
 *     observation
 *     intervention
 *     model
 *     derivation
 *     evidence
 *     transformation
 *     verification
 *     policy
 *     decision
 *     source span
 *
 * The grammar does not implement provenance.
 *
 * It only preserves the source structure necessary for semantic analysis to
 * create provenance records.
 *
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AICausality;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
    ;


/*
 * ============================================================================
 * PUBLIC AI ENTRY
 * ============================================================================
 *
 * The AI subsystem consumes causal constructs through this one public rule.
 *
 * ============================================================================
 */

aiCausalityConstruct
    : causalStatement
    | causalExpression
    ;


/*
 * ============================================================================
 * STATEMENT BOUNDARY
 * ============================================================================
 *
 * A causal statement is a causal operation followed by a terminator.
 *
 * The operation itself remains expression-oriented and therefore can be
 * reused by semantic tooling without creating a second statement hierarchy.
 *
 * ============================================================================
 */

causalStatement
    : causalOperation SEMICOLON
    ;


/*
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Causal expressions are calls in the canonical causal namespace.
 *
 * Example semantic forms:
 *
 *     causal::cause(a, b)
 *     causal::effect(a, b)
 *     causal::observe(x)
 *     causal::intervene(x, value)
 *     causal::counterfactual(condition, outcome)
 *     causal::depends_on(result, dependency)
 *
 * The parser intentionally accepts a qualified operation name.
 *
 * Semantic analysis is responsible for validating that the operation belongs
 * to the causal namespace or to an explicitly registered causal extension
 * namespace.
 *
 * ============================================================================
 */

causalExpression
    : causalOperation
    ;


causalOperation
    : causalOperationName
      LEFT_PAREN
      causalArgumentList?
      RIGHT_PAREN
    ;


causalOperationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * ARGUMENTS
 * ============================================================================
 *
 * Causal arguments are ordinary Zamani expressions.
 *
 * Named arguments provide a stable extension point for semantic metadata
 * without forcing new grammar rules whenever a causal engine introduces a
 * new option.
 *
 * ============================================================================
 */

causalArgumentList
    : causalArgument
      (
          COMMA
          causalArgument
      )*
    ;


causalArgument
    : causalNamedArgument
    | expression
    ;


causalNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * SEMANTIC CAUSAL RELATION BOUNDARY
 * ============================================================================
 *
 * The following rules provide stable semantic categories for tooling and
 * AST construction without creating separate parser implementations for each
 * category.
 *
 * They are intentionally based on the open causal operation form.
 *
 * The semantic layer identifies the operation category from its canonical
 * qualified name.
 *
 * ============================================================================
 */

causalRelation
    : causalOperation
    ;


causalObservation
    : causalOperation
    ;


causalIntervention
    : causalOperation
    ;


causalCounterfactual
    : causalOperation
    ;


causalDependency
    : causalOperation
    ;


causalQuery
    : causalOperation
    ;


causalAssertion
    : causalOperation
    ;


causalExplanation
    : causalOperation
    ;


/*
 * ============================================================================
 * CONTEXT
 * ============================================================================
 *
 * Context is deliberately represented as ordinary expressions.
 *
 * Examples:
 *
 *     causal::cause(a, b, context: model)
 *     causal::intervene(x, v, policy: policy)
 *     causal::counterfactual(c, y, evidence: evidence)
 *
 * No fixed context vocabulary is imposed.
 *
 * ============================================================================
 */

causalContext
    : LEFT_PAREN
      causalContextItem
      (
          COMMA
          causalContextItem
      )*
      RIGHT_PAREN
    ;


causalContextItem
    : causalNamedArgument
    | expression
    ;


/*
 * ============================================================================
 * RELATION OPERATOR BOUNDARY
 * ============================================================================
 *
 * Causal relationships are semantically directed.
 *
 * The grammar does not create a special arrow operator because the repository
 * already owns its operator vocabulary and because the semantic relation may
 * be represented through ordinary causal operations.
 *
 * This rule is retained as a stable semantic category for tooling and future
 * canonical relation syntax.
 *
 * ============================================================================
 */

causalRelationOperator
    : qualifiedName
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces contexts only.
 *
 * The frontend AST must preserve:
 *
 *     source span
 *     qualified operation name
 *     ordered arguments
 *     named arguments
 *     syntactic category
 *
 * A semantic normalization may then produce a generic causal operation such
 * as:
 *
 *     CausalOperation {
 *         operation
 *         operands
 *         parameters
 *         attributes
 *         source
 *     }
 *
 * This is an application of the repository's generic operation model.
 *
 * It MUST NOT introduce:
 *
 *     AICausalNode
 *     CauseNode
 *     EffectNode
 *     InterventionNode
 *     CounterfactualNode
 *
 * merely because the source operation was causal.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. resolve the operation name;
 *     2. verify that the operation is in the causal namespace or an explicitly
 *        registered causal extension namespace;
 *     3. classify the operation:
 *
 *            relation
 *            observation
 *            intervention
 *            counterfactual
 *            dependency
 *            query
 *            assertion
 *            explanation
 *
 *        where applicable;
 *     4. type-check all operands;
 *     5. determine effects;
 *     6. determine required capabilities;
 *     7. determine resource requirements;
 *     8. apply contracts;
 *     9. apply policies;
 *    10. attach provenance;
 *    11. normalize into the canonical semantic representation.
 *
 *
 * ============================================================================
 * CANONICAL OPERATION SEMANTICS
 * ============================================================================
 *
 * The following names are RECOMMENDED canonical semantic operations:
 *
 *     causal::cause
 *     causal::effect
 *     causal::observe
 *     causal::intervene
 *     causal::counterfactual
 *     causal::depends_on
 *     causal::query
 *     causal::assert
 *     causal::explain
 *
 * These are NOT a closed universal operation catalogue.
 *
 * They are semantic conventions for the core causal model.
 *
 * Implementations may register additional operations through the semantic
 * extension mechanism.
 *
 *
 * ============================================================================
 * OBSERVATION CONTRACT
 * ============================================================================
 *
 * An observation represents information obtained without necessarily changing
 * the causal system being observed.
 *
 * The grammar itself does not guarantee non-interference.
 *
 * Semantic analysis must determine whether an operation is genuinely
 * observational.
 *
 * An observation may originate from:
 *
 *     data
 *     sensors
 *     simulation
 *     hardware
 *     quantum measurement
 *     distributed state
 *     model output
 *     knowledge
 *     external systems
 *
 *
 * ============================================================================
 * INTERVENTION CONTRACT
 * ============================================================================
 *
 * An intervention represents an explicit causal manipulation or hypothetical
 * manipulation of a variable, state, relation, or model.
 *
 * It may require:
 *
 *     mutation effects;
 *     simulation effects;
 *     authorization;
 *     policy approval;
 *     capability negotiation;
 *     resource requirements;
 *     provenance.
 *
 * An intervention MUST NOT imply a particular physical actuator.
 *
 *
 * ============================================================================
 * COUNTERFACTUAL CONTRACT
 * ============================================================================
 *
 * A counterfactual evaluates a semantic alternative relative to some stated
 * context, observation, model, or intervention.
 *
 * It may require:
 *
 *     speculative execution;
 *     simulation;
 *     reasoning;
 *     uncertainty;
 *     probability;
 *     provenance.
 *
 * The grammar does not require a particular counterfactual algorithm.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * A causal dependency expresses a directed dependency relationship.
 *
 * It MUST NOT automatically mean:
 *
 *     temporal dependency;
 *     data dependency;
 *     memory dependency;
 *     hardware dependency;
 *     thread dependency;
 *     network dependency.
 *
 * Semantic analysis determines the dependency kind.
 *
 * This prevents the causal grammar from incorrectly owning unrelated
 * dependency systems.
 *
 *
 * ============================================================================
 * TEMPORAL INTEGRATION
 * ============================================================================
 *
 * Temporal causality may consume the causal semantic representation.
 *
 * The integration path is:
 *
 *     causal source construct
 *          |
 *          v
 *     causal semantic model
 *          |
 *          +----> temporal semantic analysis
 *          |
 *          +----> memory/MTS semantic analysis
 *          |
 *          +----> concurrency ordering
 *
 * The grammar does NOT import:
 *
 *     grammar/types/temporal.g4
 *
 * or:
 *
 *     grammar/memory/temporal.g4
 *
 * because doing so would make causal syntax depend on a specific temporal
 * representation and would create unnecessary grammar coupling.
 *
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Causal constructs may be consumed by generic reasoning.
 *
 * For example, semantically:
 *
 *     reason conclusion from causal::query(...)
 *
 * is valid when the types and effects permit it.
 *
 * This is represented through the ordinary expression boundary.
 *
 * No direct grammar dependency on:
 *
 *     grammar/statements/reason.g4
 *
 * is required.
 *
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Causal information may be asserted, queried, or derived from knowledge.
 *
 * The causal grammar therefore does NOT import the knowledge grammar.
 *
 * Instead:
 *
 *     causal expression
 *          |
 *          v
 *     ordinary expression / semantic value
 *          |
 *          v
 *     knowledge subsystem
 *
 * avoids a cyclic grammar dependency.
 *
 *
 * ============================================================================
 * UNCERTAINTY / PROBABILITY INTEGRATION
 * ============================================================================
 *
 * Causal operations may consume:
 *
 *     uncertain values;
 *     probabilities;
 *     distributions;
 *     confidence;
 *     evidence;
 *
 * through ordinary expressions.
 *
 * No duplicate probability or uncertainty grammar is created here.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Causal computation may consume:
 *
 *     quantum measurements;
 *     quantum-derived values;
 *     hybrid results;
 *     simulation results.
 *
 * This grammar does NOT import the quantum grammar.
 *
 * The integration is:
 *
 *     causal expression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic model where applicable
 *          |
 *          v
 *     quantum::ir
 *
 * There MUST NOT be a causal-specific quantum IR.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Causal expressions may refer to:
 *
 *     HDL observations;
 *     hardware measurements;
 *     control results;
 *     simulation results;
 *     hardware state;
 *     resource observations.
 *
 * The grammar remains target-independent.
 *
 * No:
 *
 *     register width;
 *     wire width;
 *     device count;
 *     accelerator count;
 *     physical topology;
 *     clock limit;
 *
 * is encoded here.
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Causal relationships may span distributed computation.
 *
 * The causal grammar does not own:
 *
 *     actors;
 *     channels;
 *     nodes;
 *     network topology;
 *     message scheduling;
 *     distributed consensus.
 *
 * Those remain owned by their respective subsystems.
 *
 * Semantic analysis may combine causal relationships with distributed
 * execution semantics.
 *
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Causal operation classification must participate in the universal effect
 * system.
 *
 * Example semantic mappings:
 *
 *     observe
 *         -> observation effect where required
 *
 *     intervene
 *         -> mutation/simulation effect where required
 *
 *     counterfactual
 *         -> simulation/speculation effect where required
 *
 *     query
 *         -> effect determined by the queried resource
 *
 * The grammar itself remains effect-neutral.
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Causal computation participates in the universal:
 *
 *     requirement
 *     capability
 *     constraint
 *     budget
 *     preference
 *     hint
 *
 * system.
 *
 * For example, semantic analysis may derive:
 *
 *     requires capability("causal.counterfactual")
 *
 * or:
 *
 *     requires capability("causal.intervention")
 *
 * without changing the source grammar.
 *
 * No finite capacity is encoded.
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Causal operations may be constrained by policy.
 *
 * In particular, interventions and external observations may be subject to:
 *
 *     authorization;
 *     security policy;
 *     privacy policy;
 *     resource policy;
 *     provenance policy;
 *     execution policy.
 *
 * Policy enforcement belongs downstream.
 *
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * A derived causal claim SHOULD carry provenance sufficient to reconstruct:
 *
 *     what was observed;
 *     what was assumed;
 *     what was intervened upon;
 *     which operation produced the result;
 *     what evidence was used;
 *     what transformation occurred;
 *     which policy governed the operation;
 *     which source construct produced it.
 *
 * This file provides the syntactic structure necessary for that provenance.
 *
 *
 * ============================================================================
 * DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * Causal grammar parsing must be deterministic.
 *
 * Semantic causal execution may or may not be deterministic depending on:
 *
 *     randomness;
 *     external observations;
 *     distributed state;
 *     model behavior;
 *     hardware measurements;
 *     quantum measurements;
 *     simulation policy.
 *
 * Reproducibility belongs to semantic/execution layers.
 *
 * The grammar MUST NOT embed runtime randomness or implementation state.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A causal source construct describes causal intent, not physical realization.
 *
 * The same source can therefore be compiled for:
 *
 *     embedded systems;
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed infrastructure;
 *     cloud;
 *     future computational targets.
 *
 * The grammar imposes no universal target-size ceiling.
 *
 * Actual feasibility is determined downstream by:
 *
 *     semantic requirements;
 *     representation;
 *     capabilities;
 *     resources;
 *     policies;
 *     compiler limits;
 *     runtime limits;
 *     target limits.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PROHIBITED:
 *
 *     MAX_CAUSAL_NODES
 *     MAX_CAUSAL_EDGES
 *     MAX_OBSERVATIONS
 *     MAX_INTERVENTIONS
 *     MAX_COUNTERFACTUALS
 *     MAX_DEPENDENCIES
 *     MAX_CAUSAL_DEPTH
 *     MAX_CAUSAL_WIDTH
 *     MAX_CAUSAL_GRAPH_SIZE
 *     MAX_MODELS
 *     MAX_VARIABLES
 *
 * This grammar contains none of those limits.
 *
 * Numeric literals are permitted only as ordinary source expressions and
 * therefore have program meaning rather than language-capacity meaning.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * The grammar performs no security decision.
 *
 * It MUST NOT:
 *
 *     execute an intervention;
 *     access a device;
 *     inspect a filesystem;
 *     access a network;
 *     mutate runtime state;
 *     invoke foreign code;
 *     access secrets;
 *     discover hardware.
 *
 * Security and authorization are semantic/runtime concerns.
 *
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The parser should report ordinary ANTLR syntax diagnostics for malformed
 * operation calls.
 *
 * Semantic diagnostics should be responsible for:
 *
 *     CAUSAL_NAMESPACE_INVALID
 *     CAUSAL_OPERATION_UNKNOWN
 *     CAUSAL_ARGUMENT_TYPE_MISMATCH
 *     CAUSAL_INTERVENTION_NOT_ALLOWED
 *     CAUSAL_COUNTERFACTUAL_UNSUPPORTED
 *     CAUSAL_CAPABILITY_UNAVAILABLE
 *     CAUSAL_RESOURCE_UNSATISFIED
 *     CAUSAL_POLICY_VIOLATION
 *     CAUSAL_PROVENANCE_REQUIRED
 *     CAUSAL_DEPENDENCY_INVALID
 *
 * These diagnostic identities are semantic contracts, not lexer tokens.
 *
 *
 * ============================================================================
 * NEGATIVE SEMANTIC CASES
 * ============================================================================
 *
 * The following must be rejected semantically even if they are syntactically
 * well-formed:
 *
 *     unrelated::operation(value)
 *
 * when used through aiCausalityConstruct without a registered causal
 * extension namespace.
 *
 * Also reject:
 *
 *     causal::intervene()
 *
 * when intervention semantics require an explicit target/value.
 *
 * Also reject:
 *
 *     causal::counterfactual()
 *
 * when the selected semantic operation requires a condition/context.
 *
 * The exact arity and type requirements belong to the semantic operation
 * registry rather than this grammar.
 *
 *
 * ============================================================================
 * POSITIVE FORMS
 * ============================================================================
 *
 * Canonical semantic examples:
 *
 *     causal::cause(a, b);
 *
 *     causal::effect(a, b);
 *
 *     causal::observe(observation);
 *
 *     causal::intervene(variable, value);
 *
 *     causal::counterfactual(condition, outcome);
 *
 *     causal::depends_on(result, dependency);
 *
 *     causal::query(target);
 *
 *     causal::assert(claim, evidence: evidence);
 *
 *     causal::explain(decision, evidence: evidence);
 *
 * Extended:
 *
 *     causal::intervene(
 *         variable,
 *         value,
 *         policy: intervention_policy
 *     );
 *
 *     causal::counterfactual(
 *         condition,
 *         outcome,
 *         evidence: evidence,
 *         context: model
 *     );
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar tests must verify that the source syntax does not depend on:
 *
 *     causal graph size;
 *     number of observations;
 *     number of interventions;
 *     number of counterfactuals;
 *     number of dependencies;
 *     number of variables;
 *     number of models;
 *     number of targets;
 *     number of execution resources.
 *
 * Tests should include:
 *
 *     one causal relation;
 *     many relations;
 *     nested expressions;
 *     large generated causal operation sequences;
 *     symbolic quantities;
 *     dynamic quantities;
 *     distributed causal values;
 *     quantum-derived observations;
 *     hybrid values;
 *     simulation results.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [x] It has one clear ownership boundary.
 *     [x] It defines no lexer rules.
 *     [x] It defines no hardware limits.
 *     [x] It defines no resource ceilings.
 *     [x] It defines no runtime behavior.
 *     [x] It defines no causal algorithm.
 *     [x] It reuses canonical names.
 *     [x] It reuses canonical expressions.
 *     [x] It creates no duplicate type system.
 *     [x] It creates no duplicate reasoning grammar.
 *     [x] It creates no duplicate knowledge grammar.
 *     [x] It creates no duplicate uncertainty grammar.
 *     [x] It creates no AI-specific IR.
 *     [x] It preserves quantum::ir as the quantum boundary.
 *     [x] It supports open-world causal extensions.
 *     [x] It remains target-independent.
 *     [x] It is compatible with safe Rust implementations.
 *     [x] It is compatible with Rust 1.97+.
 *     [x] It supports POCO-REAF source portability.
 *
 * Repository integration still requires the composition steps documented
 * below.
 *
 *
 * ============================================================================
 * REQUIRED INTEGRATION
 * ============================================================================
 *
 * 1. grammar/ai/ai.g4
 *
 * Add:
 *
 *     AICausality
 *
 * to the AI grammar imports.
 *
 * Add:
 *
 *     | aiCausalityConstruct
 *
 * to `aiConstruct`.
 *
 *
 * 2. grammar/spec/causality.md
 *
 * Create the normative causal semantic specification defining:
 *
 *     relation;
 *     observation;
 *     intervention;
 *     counterfactual;
 *     dependency;
 *     causal query;
 *     causal assertion;
 *     causal explanation;
 *     causal namespaces;
 *     operation registry;
 *     effects;
 *     capabilities;
 *     resources;
 *     provenance;
 *     policies;
 *     diagnostics;
 *     determinism;
 *     compatibility.
 *
 *
 * 3. grammar/spec/ai.md
 *
 * Add the causal subsystem to the AI semantic integration matrix.
 *
 *
 * 4. grammar/tests/ai/causality/
 *
 * Add:
 *
 *     relation/
 *     observation/
 *     intervention/
 *     counterfactual/
 *     dependency/
 *     query/
 *     assertion/
 *     explanation/
 *     namespace/
 *     negative/
 *     scalability/
 *     quantum/
 *     temporal/
 *
 *
 * 5. Semantic operation registry
 *
 * The semantic layer must register the canonical operations:
 *
 *     causal::cause
 *     causal::effect
 *     causal::observe
 *     causal::intervene
 *     causal::counterfactual
 *     causal::depends_on
 *     causal::query
 *     causal::assert
 *     causal::explain
 *
 * This registry must be data/semantic metadata driven rather than a finite
 * grammar catalogue of all future causal algorithms.
 *
 *
 * 6. Existing temporal causality
 *
 * Existing temporal causality infrastructure:
 *
 *     grammar/types/temporal.g4
 *     grammar/memory/temporal.g4
 *     src/toolchain/causality_checker.rs
 *
 * must consume or interoperate with the canonical causal semantic model.
 *
 * `src/toolchain/causality_checker.rs` MUST NOT become the parser or grammar
 * owner.
 *
 *
 * 7. Rust implementation
 *
 * Any implementation associated with this grammar must use:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * No `unsafe` implementation is required or permitted.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * The causal subsystem is:
 *
 *     source intent
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic causal model
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *     classical            quantum
 *                            |
 *                            v
 *                        quantum::ir
 *          |                  |
 *          +--------+---------+
 *                   |
 *                   v
 *            canonical semantic
 *               representation
 *                   |
 *          optimization / lowering
 *                   |
 *             routing / scheduling
 *                   |
 *               resilience
 *                   |
 *                ZQN/HAL
 *                   |
 *             target realization
 *
 * This keeps causality universal, extensible, resource-independent and
 * compatible with POCO-REAF.
 *
 * ============================================================================
 */