/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/policies/adaptation.g4
 *
 * Grammar:
 *     PolicyAdaptation
 *
 * Status:
 *     PRODUCTION-READY POLICY LEAF GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021+
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the POLICY-SPECIFIC syntax governing adaptive computation.
 *
 * It does NOT own executable adaptation syntax.
 *
 * Executable adaptation remains owned by:
 *
 *     grammar/statements/adapt.g4
 *     grammar/execution/adaptive.g4
 *     grammar/expressions/expressions.g4
 *
 * This file instead describes what a POLICY permits, constrains, requires,
 * prefers, forbids, or otherwise governs when adaptation is requested.
 *
 * Conceptually:
 *
 *     source adaptation intent
 *             |
 *             v
 *     policy applicability
 *             |
 *             v
 *     adaptation policy
 *             |
 *       +-----+-----+------------------+
 *       |           |                  |
 *       v           v                  v
 *   capability    resource          effect
 *    analysis     analysis          analysis
 *       |           |                  |
 *       +-----------+------------------+
 *                   |
 *                   v
 *              authorization
 *                   |
 *                   v
 *               contracts
 *                   |
 *                   v
 *              provenance
 *                   |
 *                   v
 *          semantic adaptation
 *
 * The policy grammar expresses GOVERNANCE.
 *
 * It does not perform adaptation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *   adapt statement             policy adaptation
 *       |                             |
 *       |                             v
 *       |                    PolicyAdaptation
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *              domain-neutral AST
 *                     |
 *                     v
 *             structural validation
 *                     |
 *        +------------+-------------+
 *        |            |             |
 *        v            v             v
 *    capabilities  resources     effects
 *        |            |             |
 *        +------------+-------------+
 *                     |
 *                     v
 *                  policies
 *                     |
 *                     +--> authorization
 *                     +--> contracts
 *                     +--> provenance
 *                     +--> execution
 *                     +--> security
 *                     +--> deployment
 *                     +--> simulation
 *                     |
 *                     v
 *             canonical semantic model
 *                     |
 *          +----------+----------+
 *          |                     |
 *          v                     v
 *     classical semantics    quantum semantics
 *                                  |
 *                                  v
 *                              quantum::ir
 *                                  |
 *                                  v
 *                           optimization
 *                                  |
 *                         lowering / routing
 *                                  |
 *                             scheduling
 *                                  |
 *                             resilience
 *                                  |
 *                                ZQN
 *                                  |
 *                                HAL
 *                                  |
 *                          target realization
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     policyAdaptation
 *     policyAdaptationTarget
 *     policyAdaptationClause
 *     policyAdaptationSource
 *     policyAdaptationContext
 *     policyAdaptationCondition
 *     policyAdaptationFallback
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     lexer rules
 *     keyword definitions
 *     identifiers
 *     qualified names
 *     general expressions
 *     expression precedence
 *     executable adaptation
 *     learning syntax
 *     reasoning syntax
 *     knowledge syntax
 *     capability definitions
 *     resource definitions
 *     effect definitions
 *     contracts
 *     authorization
 *     identities
 *     credentials
 *     roles
 *     trust
 *     security enforcement
 *     provenance implementation
 *     runtime adaptation
 *     execution scheduling
 *     target selection
 *     hardware selection
 *     quantum operations
 *     quantum::ir
 *     HDL implementation
 *     routing
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one owner for each of the following:
 *
 *     executable adaptation statement
 *         -> grammar/statements/adapt.g4
 *
 *     executable adaptive expression
 *         -> grammar/execution/adaptive.g4
 *
 *     policy declaration/composition
 *         -> grammar/core/policies.g4
 *
 *     generic policy expression
 *         -> grammar/expressions/policy.g4
 *
 *     policy adaptation specialization
 *         -> THIS FILE
 *
 *     capability semantics
 *         -> grammar/core/capabilities.g4
 *
 *     resource semantics
 *         -> grammar/resources/
 *
 *     effect semantics
 *         -> grammar/effects/
 *
 *     contract semantics
 *         -> grammar/validation/
 *
 * This file must never recreate those authorities.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Canonical lexer:
 *
 *     ZamaniLexer
 *
 * Canonical expression grammar:
 *
 *     Expressions
 *
 * Existing policy architecture:
 *
 *     grammar/core/policies.g4
 *
 * Existing policy expression architecture:
 *
 *     grammar/expressions/policy.g4
 *
 * Existing adaptation architecture:
 *
 *     grammar/statements/adapt.g4
 *     grammar/execution/adaptive.g4
 *
 * The dependencies are semantic/compositional dependencies.
 *
 * This file does NOT import:
 *
 *     statements
 *     execution
 *     resources
 *     effects
 *     security
 *     quantum
 *     hardware
 *
 * merely to obtain generic expressions or domain concepts.
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * Intended direction:
 *
 *     Expressions
 *          ^
 *          |
 *     PolicyAdaptation
 *          ^
 *          |
 *     ZamaniPolicies
 *
 * More precisely:
 *
 *     grammar/core/policies.g4
 *              |
 *              +--> PolicyAdaptation
 *              |
 *              +--> Expressions
 *
 * This file may independently import Expressions because Expressions does not
 * import this policy leaf.
 *
 * IMPORTANT:
 *
 * This file must NOT be imported by:
 *
 *     grammar/expressions/expressions.g4
 *
 * unless the repository later establishes an explicitly acyclic adapter
 * composition.
 *
 * Policy adaptation is a policy concern, not a universal expression concern.
 *
 * ============================================================================
 * PUBLIC EXPORT
 * ============================================================================
 *
 * Primary:
 *
 *     policyAdaptation
 *
 * Secondary:
 *
 *     policyAdaptationTarget
 *     policyAdaptationClause
 *     policyAdaptationSource
 *     policyAdaptationContext
 *     policyAdaptationCondition
 *     policyAdaptationFallback
 *
 * The primary rule is the intended integration point for:
 *
 *     grammar/core/policies.g4
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The domain-neutral AST adapter should construct a semantic-neutral policy
 * adaptation node conceptually equivalent to:
 *
 *     PolicyAdaptation {
 *         target,
 *         source?,
 *         context[],
 *         condition?,
 *         fallback?,
 *         source_span
 *     }
 *
 * It MUST NOT contain:
 *
 *     physical CPU identifiers
 *     physical GPU identifiers
 *     physical FPGA identifiers
 *     physical ASIC identifiers
 *     physical QPU identifiers
 *     physical qubit mappings
 *     device topology
 *     coupling maps
 *     routing decisions
 *     scheduler decisions
 *     calibration
 *     QEC layout
 *     vendor backend objects
 *
 * The AST records policy intent.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     whether the policy applies;
 *     whether the adaptation target is valid;
 *     whether adaptation is permitted;
 *     whether required capabilities exist;
 *     whether required resources can satisfy the policy;
 *     whether required effects are permitted;
 *     whether authorization exists;
 *     whether contracts remain satisfied;
 *     whether provenance requirements are satisfied;
 *     whether the requested fallback is valid;
 *     whether the adaptation is deterministic where required;
 *     whether the target realization remains feasible.
 *
 * This file does NOT make those decisions.
 *
 * In particular:
 *
 *     syntax != authorization
 *     syntax != capability availability
 *     syntax != resource availability
 *     syntax != effect approval
 *     syntax != runtime feasibility
 *
 * ============================================================================
 * ADAPTATION SAFETY MODEL
 * ============================================================================
 *
 * The presence of a policy adaptation clause does NOT grant unrestricted
 * mutation.
 *
 * A semantic implementation must preserve this ordering:
 *
 *     adaptation intent
 *          |
 *          v
 *     policy applicability
 *          |
 *          v
 *     authorization
 *          |
 *          v
 *     capability validation
 *          |
 *          v
 *     effect validation
 *          |
 *          v
 *     resource validation
 *          |
 *          v
 *     contract validation
 *          |
 *          v
 *     provenance requirements
 *          |
 *          v
 *     adaptation planning
 *          |
 *          v
 *     execution
 *
 * No parser rule in this file bypasses any of these stages.
 *
 * ============================================================================
 * POLICY / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A policy says what is permitted, required, forbidden, or preferred.
 *
 * A capability says what a realization environment can provide.
 *
 * Therefore:
 *
 *     policy != capability
 *
 * and:
 *
 *     capability availability != authorization
 *
 * A policy adaptation clause may reference capability expressions as ordinary
 * expressions.
 *
 * Capability resolution remains owned by the capability/resource subsystem.
 *
 * ============================================================================
 * POLICY / RESOURCE SEPARATION
 * ============================================================================
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * This grammar does not define:
 *
 *     memory quantities
 *     CPU counts
 *     GPU counts
 *     QPU counts
 *     node counts
 *     thread counts
 *     tensor ranks
 *     network sizes
 *     device counts
 *
 * A policy may refer to resource intent through ordinary expressions and
 * canonical resource constructs.
 *
 * Resource feasibility remains downstream.
 *
 * ============================================================================
 * POLICY / EFFECT SEPARATION
 * ============================================================================
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * Adaptation may semantically involve effects such as:
 *
 *     mutation
 *     learning
 *     reflection
 *     code_generation
 *     randomness
 *     measurement
 *     simulation
 *     network
 *     distributed
 *     native
 *     foreign
 *
 * This grammar does not enumerate effect implementations.
 *
 * ============================================================================
 * POLICY / CONTRACT SEPARATION
 * ============================================================================
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * This file does not redefine:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Policy adaptation may be evaluated in the presence of those contracts, but
 * contract syntax remains outside this file.
 *
 * ============================================================================
 * POLICY / AUTHORIZATION SEPARATION
 * ============================================================================
 *
 * This grammar does not implement authorization.
 *
 * In particular, a syntactically valid adaptation policy does not imply:
 *
 *     identity validity
 *     credential validity
 *     role membership
 *     trust
 *     authorization
 *     sandbox permission
 *
 * Those decisions belong to the security/authorization subsystem.
 *
 * ============================================================================
 * POLICY / PROVENANCE SEPARATION
 * ============================================================================
 *
 * Policy adaptation may require provenance.
 *
 * The semantic provenance system may record:
 *
 *     policy identity
 *     adaptation target
 *     source
 *     context
 *     condition
 *     fallback
 *     authorization decision
 *     capability decision
 *     resource decision
 *     effect decision
 *     contract result
 *     adaptation decision
 *     evidence
 *     transformation
 *     resulting state
 *     verification
 *
 * This grammar merely preserves the source structure needed to construct
 * those records.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Adaptation targets are generic expressions.
 *
 * Therefore this grammar does NOT enumerate:
 *
 *     model
 *     strategy
 *     plan
 *     execution
 *     circuit
 *     tensor
 *     device
 *     hardware
 *     neural network
 *     database
 *     distributed service
 *     quantum program
 *     HDL design
 *     compiler strategy
 *     scheduler strategy
 *
 * Any such entity can be represented by an ordinary expression.
 *
 * New computational domains therefore do not require modifying this file.
 *
 * ============================================================================
 * NO ALGORITHM CATALOGUE
 * ============================================================================
 *
 * This file MUST NOT enumerate adaptation algorithms such as:
 *
 *     gradient
 *     reinforcement
 *     Bayesian
 *     evolutionary
 *     genetic
 *     heuristic
 *     predictive
 *     transfer
 *     fine_tuning
 *     remapping
 *     rerouting
 *     rescheduling
 *     recompilation
 *     specialization
 *
 * Those belong to:
 *
 *     libraries
 *     dialects
 *     semantic capabilities
 *     execution strategies
 *     optimization
 *     runtime implementations
 *
 * ============================================================================
 * CLAUSE MODEL
 * ============================================================================
 *
 * A policy adaptation has:
 *
 *     TARGET
 *     SOURCE?
 *     CONTEXT?
 *     CONDITION?
 *     FALLBACK?
 *
 * Canonical conceptual forms:
 *
 *     adapt TARGET;
 *
 *     adapt TARGET from SOURCE;
 *
 *     adapt TARGET with (CONTEXT);
 *
 *     adapt TARGET from SOURCE with (CONTEXT);
 *
 *     adapt TARGET when CONDITION;
 *
 *     adapt TARGET from SOURCE when CONDITION;
 *
 *     adapt TARGET fallback FALLBACK;
 *
 *     adapt TARGET from SOURCE with (POLICY, EVIDENCE)
 *         when CONDITION
 *         fallback ALTERNATIVE;
 *
 * These are POLICY declarations.
 *
 * They are not executable adaptation statements.
 *
 * ============================================================================
 * CLAUSE ORDER
 * ============================================================================
 *
 * The canonical ordering is:
 *
 *     TARGET
 *     SOURCE?
 *     CONTEXT?
 *     CONDITION?
 *     FALLBACK?
 *
 * This gives the policy subsystem one deterministic source representation.
 *
 * A policy author does not need to remember multiple equivalent orderings.
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * The target is mandatory.
 *
 * It is an ordinary Zamani expression.
 *
 * Examples:
 *
 *     adapt strategy;
 *     adapt model;
 *     adapt execution_plan;
 *     adapt quantum_strategy;
 *     adapt hardware_plan;
 *     adapt distributed_strategy;
 *
 * The grammar does not determine what those expressions mean.
 *
 * ============================================================================
 * SOURCE CONTRACT
 * ============================================================================
 *
 * The optional source identifies information that may influence the governed
 * adaptation.
 *
 * Examples:
 *
 *     feedback
 *     observation
 *     measurement
 *     evidence
 *     model output
 *     simulation result
 *     distributed result
 *     resource observation
 *     performance observation
 *     learned result
 *     inferred result
 *
 * Syntax:
 *
 *     from SOURCE
 *
 * Semantic classification remains downstream.
 *
 * ============================================================================
 * CONTEXT CONTRACT
 * ============================================================================
 *
 * Context supplies additional policy-relevant information.
 *
 * Examples:
 *
 *     policy
 *     capability information
 *     resource information
 *     evidence
 *     provenance
 *     constraints
 *     observations
 *     confidence
 *     execution state
 *
 * Syntax:
 *
 *     with (CONTEXT, CONTEXT, ...)
 *
 * Context entries are ordinary expressions.
 *
 * The list is open-ended.
 *
 * ============================================================================
 * CONDITION CONTRACT
 * ============================================================================
 *
 * A policy adaptation may be conditional:
 *
 *     when CONDITION
 *
 * The condition is an ordinary expression.
 *
 * No second boolean/predicate language is created.
 *
 * ============================================================================
 * FALLBACK CONTRACT
 * ============================================================================
 *
 * A fallback describes a permitted alternative when the governed adaptation
 * cannot proceed under the applicable policy.
 *
 * Syntax:
 *
 *     fallback FALLBACK
 *
 * The fallback is an ordinary expression.
 *
 * The grammar does not decide whether a fallback means:
 *
 *     retry
 *     recover
 *     simulate
 *     use another strategy
 *     select another valid realization
 *     defer
 *     reject
 *
 * Those are semantic policy decisions.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY NEGOTIATION
 * ============================================================================
 *
 * Adaptation policies can be evaluated against:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     resource availability
 *
 * The policy grammar never performs negotiation.
 *
 * The downstream architecture is:
 *
 *     policy intent
 *          |
 *          v
 *     semantic policy
 *          |
 *          +--> capability resolution
 *          |
 *          +--> resource analysis
 *          |
 *          +--> effect analysis
 *          |
 *          +--> authorization
 *          |
 *          +--> contract validation
 *          |
 *          v
 *     adaptation plan
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum adaptation policies may govern:
 *
 *     adaptive quantum execution
 *     dynamic circuit behavior
 *     measurement-driven adaptation
 *     resilience choices
 *     simulator fallback
 *     execution strategy selection
 *     resource/capability requirements
 *
 * Examples:
 *
 *     adapt quantum_strategy from measurement_result;
 *
 *     adapt quantum_strategy from quantum_observation
 *         with (execution_policy);
 *
 * The grammar does NOT enumerate:
 *
 *     quantum gates
 *     physical qubits
 *     coupling maps
 *     routing
 *     calibration
 *     QEC mechanisms
 *     vendor backends
 *
 * If the governed computation becomes quantum computation, the semantic
 * pipeline MUST cross the canonical:
 *
 *     quantum::ir
 *
 * boundary before quantum optimization, decomposition, routing, scheduling,
 * resilience, or physical realization.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical adaptation may govern:
 *
 *     algorithm selection
 *     strategy selection
 *     model selection
 *     execution planning
 *     resource-aware specialization
 *     deterministic fallback
 *
 * The grammar remains identical.
 *
 * No classical-specific adaptation syntax is required.
 *
 * ============================================================================
 * AI / LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning and reasoning remain independent language features.
 *
 * Their results may appear as:
 *
 *     source
 *     context
 *     condition
 *     fallback
 *     target
 *
 * Examples:
 *
 *     adapt model from learning_result;
 *
 *     adapt strategy from inferred_strategy
 *         with (evidence, confidence);
 *
 * No learning algorithm is encoded here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Adaptation policies may govern:
 *
 *     synthesis strategy
 *     simulation strategy
 *     verification strategy
 *     hardware realization intent
 *     accelerator strategy
 *     implementation fallback
 *
 * They do not encode:
 *
 *     register width
 *     bus width
 *     fixed memory capacity
 *     fixed device count
 *     physical topology
 *     placement
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed adaptation may consume:
 *
 *     service observations
 *     node observations
 *     consensus results
 *     performance measurements
 *     failure information
 *     resource information
 *
 * Actor/message/channel/service grammar remains owned by the concurrency and
 * distributed subsystems.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * A simulation result may be supplied as source/context:
 *
 *     adapt strategy from simulation_result;
 *
 * Simulation remains an execution strategy.
 *
 * This file does not create a second language for simulation.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Adaptation policy does not imply:
 *
 *     source rewriting
 *     code generation
 *     recompilation
 *     reflection
 *     self-modifying code
 *
 * Such operations require their own semantic effects, capabilities and
 * policies.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     canonical lexer
 *     grammar
 *     parser configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     memory
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     runtime state
 *     scheduler state
 *     wall-clock time
 *     randomness
 *
 * Policy evaluation may have runtime inputs, but those belong downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes no universal finite ceiling on:
 *
 *     number of policy adaptations
 *     number of policy clauses
 *     number of context entries
 *     expression size
 *     policy nesting
 *     target complexity
 *     source complexity
 *     fallback complexity
 *     program size
 *     resource scale
 *     capability scale
 *     processor scale
 *     accelerator scale
 *     quantum scale
 *     distributed scale
 *     network scale
 *
 * Repetition is open-ended:
 *
 *     (COMMA expression)*
 *
 * There are deliberately no constants such as:
 *
 *     MAX_POLICY_ADAPTATIONS
 *     MAX_ADAPTATION_CONTEXT
 *     MAX_FALLBACKS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * "Infinite scalability" means that the language does not impose an artificial
 * universal finite machine-capacity ceiling. It does not claim physically
 * infinite hardware or unlimited implementation resources.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Policy adaptation must preserve source portability.
 *
 * A source policy can remain unchanged while semantic realization changes
 * according to available:
 *
 *     capabilities
 *     resources
 *     policies
 *     execution environments
 *     targets
 *
 * Therefore the same policy can govern adaptation on:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * provided semantic requirements remain satisfiable.
 *
 * The policy grammar never chooses the physical realization.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no machine-capacity constants;
 *     no device enumeration;
 *     no vendor catalogue;
 *     no quantum-gate catalogue;
 *     no fixed resource classes;
 *     no fixed target list;
 *     no fixed policy-provider list;
 *     no fixed adaptation algorithm list;
 *     no physical topology;
 *     no hardware placement;
 *     no scheduling policy implementation.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file consumes canonical tokens from ZamaniLexer.
 *
 * Required tokens:
 *
 *     ADAPT
 *     FROM
 *     WITH
 *     WHEN
 *     FALLBACK
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *
 * These tokens are already part of the repository's central lexical
 * architecture.
 *
 * This file defines NO lexer rules.
 *
 * In particular, it must not create:
 *
 *     POLICY_ADAPT
 *     ADAPT_POLICY
 *     ADAPTIVE_POLICY
 *     SELF_ADAPT
 *     ADAPTATION_POLICY
 *
 * merely to specialize vocabulary.
 *
 * ============================================================================
 * GRAMMAR TECHNOLOGY
 * ============================================================================
 */

parser grammar PolicyAdaptation;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * This is the single public policy-specific adaptation entry point.
 *
 * The existing policy orchestrator is expected to consume:
 *
 *     policyAdaptation
 *
 * as one of its policy-member alternatives.
 */

policyAdaptation
    : ADAPT
      policyAdaptationTarget
      policyAdaptationClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * The adaptation target is an ordinary Zamani expression.
 *
 * This deliberately avoids domain-specific alternatives.
 */

policyAdaptationTarget
    : expression
    ;


/*
 * ============================================================================
 * CLAUSE DISPATCH
 * ============================================================================
 *
 * Canonical clause order is enforced structurally by the grammar below.
 *
 * The clause family itself remains private.
 */

policyAdaptationClause
    : policyAdaptationSource
    | policyAdaptationContext
    | policyAdaptationCondition
    | policyAdaptationFallback
    ;


/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 *
 * Identifies information that may influence the governed adaptation.
 *
 * Example:
 *
 *     adapt strategy from feedback;
 */

policyAdaptationSource
    : FROM expression
    ;


/*
 * ============================================================================
 * CONTEXT
 * ============================================================================
 *
 * Provides additional adaptation-policy context.
 *
 * At least one context expression is required.
 *
 * The repetition is deliberately open-ended.
 */

policyAdaptationContext
    : WITH
      LPAREN
      expressionList
      RPAREN
    ;


/*
 * ============================================================================
 * CONDITION
 * ============================================================================
 *
 * Conditional applicability.
 *
 * This is intentionally an ordinary expression rather than a separate
 * predicate language.
 */

policyAdaptationCondition
    : WHEN expression
    ;


/*
 * ============================================================================
 * FALLBACK
 * ============================================================================
 *
 * Describes a policy-governed alternative.
 *
 * The semantic layer determines whether the expression represents:
 *
 *     retry
 *     recover
 *     simulation
 *     another strategy
 *     deferred execution
 *     rejection
 *     another valid realization
 *
 * No fallback algorithm is hard-coded here.
 */

policyAdaptationFallback
    : FALLBACK expression
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Existing grammar/core/policies.g4 remains the policy composition authority.
 *
 * It should integrate this leaf through:
 *
 *     policyMember
 *         : ...
 *         | policyAdaptation
 *         ;
 *
 * The parent policy grammar remains responsible for deciding where policy
 * members occur.
 *
 * This file does not define:
 *
 *     policyMember
 *     policyDeclaration
 *     policyBody
 *     policyRule
 *
 * ============================================================================
 * EXECUTABLE ADAPTATION BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT be used as a replacement for:
 *
 *     grammar/statements/adapt.g4
 *
 * The distinction is:
 *
 *     adapt target;
 *
 * at executable statement level:
 *
 *     adaptation intent
 *
 * whereas:
 *
 *     policy {
 *         adapt target from feedback
 *             with (policy_context)
 *             when condition
 *             fallback alternative;
 *     }
 *
 * is:
 *
 *     governance of adaptation intent
 *
 * The semantic layer may associate both with the same canonical adaptation
 * semantic model, while retaining their distinct source provenance.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * This file does not declare an effect.
 *
 * An adaptation policy may govern operations whose semantic effects include:
 *
 *     mutation
 *     learning
 *     reflection
 *     code_generation
 *     randomness
 *     measurement
 *     simulation
 *     network
 *     distributed
 *     native
 *     foreign
 *
 * Effect inference/checking belongs to:
 *
 *     grammar/effects/
 *
 * and the downstream semantic effect system.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability references may appear in any expression position.
 *
 * Examples:
 *
 *     adapt strategy
 *         with (capability("adaptation"));
 *
 *     adapt quantum_strategy
 *         from measurement
 *         with (capability("quantum.measurement"));
 *
 * The capability grammar remains the sole authority for capability syntax.
 *
 * This file does not resolve or authorize capabilities.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource expressions may appear as policy context.
 *
 * Examples:
 *
 *     adapt strategy
 *         with (required_resources);
 *
 *     adapt execution_plan
 *         from resource_observation;
 *
 * The resource subsystem determines:
 *
 *     resource identity
 *     quantity
 *     availability
 *     feasibility
 *     negotiation
 *
 * This grammar does not allocate resources.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contract constructs remain outside this grammar.
 *
 * A policy adaptation can be subject to:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * The validation/contract subsystem owns those constructs.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * An adaptation policy does not grant authorization.
 *
 * The semantic/security layers must independently establish:
 *
 *     identity
 *     credential validity
 *     authorization
 *     trust
 *     sandbox compatibility
 *     permitted effects
 *
 * This prevents:
 *
 *     policy syntax
 *
 * from becoming:
 *
 *     security authority
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * The AST adapter must preserve source spans for:
 *
 *     ADAPT
 *     target
 *     FROM
 *     source
 *     WITH
 *     each context expression
 *     WHEN
 *     condition
 *     FALLBACK
 *     fallback expression
 *     terminating SEMICOLON
 *
 * Semantic provenance may then record:
 *
 *     policy identity
 *     source location
 *     adaptation target
 *     source
 *     context
 *     condition
 *     fallback
 *     policy decision
 *     authorization decision
 *     capability decision
 *     resource decision
 *     effect decision
 *     contract result
 *     realization decision
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * If the adaptation target or context eventually governs quantum computation:
 *
 *     source
 *       |
 *       v
 *     policy semantic model
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * This grammar never introduces another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Policy adaptation may govern:
 *
 *     synthesis strategy
 *     verification strategy
 *     simulation strategy
 *     implementation strategy
 *     accelerator strategy
 *
 * It does not encode:
 *
 *     register width
 *     bus width
 *     device count
 *     memory capacity
 *     topology
 *     physical placement
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Policy adaptation may consume distributed observations and results.
 *
 * It does not own:
 *
 *     actor syntax
 *     message syntax
 *     channel syntax
 *     service syntax
 *     node topology
 *     scheduler implementation
 *
 * Those remain owned by the corresponding subsystems.
 *
 * ============================================================================
 * METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Policy adaptation does not itself permit:
 *
 *     source rewriting
 *     executable rewriting
 *     code generation
 *     recompilation
 *     reflection
 *     compiler mutation
 *
 * If such behavior is requested, the semantic layer must require the
 * corresponding capability, effect and policy authorization.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * STRUCTURAL/PARSER ERRORS
 * ------------------------
 *
 * Invalid:
 *
 *     adapt;
 *
 *     adapt();
 *
 *     adapt( target );
 *
 *     adapt target from;
 *
 *     adapt target with ();
 *
 *     adapt target with (, context);
 *
 *     adapt target with (context,);
 *
 *     adapt target when;
 *
 *     adapt target fallback;
 *
 *     adapt target with (context) from source;
 *
 * The final example is rejected because clause order is canonical:
 *
 *     TARGET
 *     SOURCE?
 *     CONTEXT?
 *     CONDITION?
 *     FALLBACK?
 *
 * SEMANTIC ERRORS
 * ---------------
 *
 * Examples:
 *
 *     adaptation not permitted by policy
 *     missing authorization
 *     unavailable capability
 *     insufficient resources
 *     forbidden effect
 *     violated contract
 *     invalid provenance
 *     invalid target
 *     invalid source
 *     invalid fallback
 *
 * These MUST NOT be reported as parser errors.
 *
 * ============================================================================
 * POSITIVE CONFORMANCE TESTS
 * ============================================================================
 *
 * Minimal:
 *
 *     adapt strategy;
 *
 * Source:
 *
 *     adapt strategy from feedback;
 *
 * Context:
 *
 *     adapt strategy with (policy_context);
 *
 * Source + context:
 *
 *     adapt strategy from feedback
 *         with (policy_context);
 *
 * Conditional:
 *
 *     adapt strategy
 *         when condition;
 *
 * Fallback:
 *
 *     adapt strategy
 *         fallback alternate_strategy;
 *
 * Full:
 *
 *     adapt strategy
 *         from feedback
 *         with (policy_context, evidence, capability_context)
 *         when condition
 *         fallback alternate_strategy;
 *
 * Classical:
 *
 *     adapt execution_strategy
 *         from performance_observation;
 *
 * Quantum:
 *
 *     adapt quantum_strategy
 *         from measurement_result
 *         with (execution_policy);
 *
 * Hybrid:
 *
 *     adapt hybrid_strategy
 *         from quantum_result
 *         with (classical_context);
 *
 * HDL/hardware:
 *
 *     adapt hardware_strategy
 *         from simulation_result
 *         with (verification_result);
 *
 * Distributed:
 *
 *     adapt distributed_strategy
 *         from distributed_observation
 *         with (resource_state);
 *
 * AI/model:
 *
 *     adapt model
 *         from learning_result
 *         with (confidence, evidence);
 *
 * Reasoning:
 *
 *     adapt strategy
 *         from inferred_strategy
 *         with (evidence);
 *
 * ============================================================================
 * NEGATIVE CONFORMANCE TESTS
 * ============================================================================
 *
 * Must reject:
 *
 *     adapt;
 *
 *     adapt from feedback;
 *
 *     adapt strategy from;
 *
 *     adapt strategy with ();
 *
 *     adapt strategy with (, policy);
 *
 *     adapt strategy with (policy,);
 *
 *     adapt strategy when;
 *
 *     adapt strategy fallback;
 *
 *     adapt strategy with (policy) from feedback;
 *
 *     adapt strategy from feedback when condition
 *         with (policy);
 *
 *     adapt strategy from feedback fallback alternative
 *         with (policy);
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Verify interaction with the generic expression system:
 *
 *     adapt choose(strategy_a, strategy_b);
 *
 *     adapt strategy from infer(candidate);
 *
 *     adapt strategy from query(result);
 *
 *     adapt strategy with (capability("adaptation"));
 *
 *     adapt strategy with (required_memory);
 *
 *     adapt strategy when observation > threshold;
 *
 *     adapt strategy fallback simulate(strategy);
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * The same grammar must parse all of the following without domain-specific
 * grammar alternatives:
 *
 *     adapt classical_strategy;
 *
 *     adapt quantum_strategy;
 *
 *     adapt hybrid_strategy;
 *
 *     adapt hdl_strategy;
 *
 *     adapt hardware_strategy;
 *
 *     adapt tensor_strategy;
 *
 *     adapt distributed_strategy;
 *
 *     adapt network_strategy;
 *
 *     adapt simulation_strategy;
 *
 *     adapt future_domain_strategy;
 *
 * This proves that domain growth does not require a new adaptation grammar
 * branch.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The following must remain structurally valid for arbitrary finite source
 * input supported by the implementation:
 *
 *     adapt target;
 *
 *     adapt target from source;
 *
 *     adapt target with (a, b, c, ...);
 *
 * The grammar must not introduce any fixed context count.
 *
 * Practical parser/compiler limits are implementation constraints and must
 * never become language semantics.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parsing the same source with the same:
 *
 *     lexer version
 *     grammar version
 *     parser configuration
 *
 * must produce equivalent parse structure.
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     resource availability
 *     target discovery
 *     filesystem
 *     network
 *     wall-clock time
 *     randomness
 *     runtime state
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no new lexer keyword.
 *
 * It consumes existing canonical spellings:
 *
 *     adapt
 *     from
 *     with
 *     when
 *     fallback
 *
 * Existing executable adaptation syntax remains owned by its existing
 * grammar.
 *
 * Existing generic policy syntax remains valid.
 *
 * If policy adaptation syntax changes in the future, the semantic model and
 * compatibility layer must preserve the distinction between:
 *
 *     policy adaptation
 *
 * and:
 *
 *     executable adaptation.
 *
 * ============================================================================
 * VERSIONING CONTRACT
 * ============================================================================
 *
 * The policy adaptation semantic model should carry:
 *
 *     language version
 *     grammar version
 *     policy version where applicable
 *     semantic model version
 *     compiler version
 *
 * This grammar itself does not inspect versions.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This file contains:
 *
 *     no embedded Rust actions;
 *     no semantic predicates;
 *     no runtime code;
 *     no I/O;
 *     no hardware access;
 *     no network access;
 *     no unsafe code.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021+
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] PolicyAdaptation is the sole grammar identity for policy adaptation.
 *     [ ] The file consumes ZamaniLexer.
 *     [ ] No lexer rules are duplicated.
 *     [ ] No identifier rules are duplicated.
 *     [ ] No general expression rules are duplicated.
 *     [ ] policyAdaptation is the sole public entry point.
 *     [ ] Target is mandatory.
 *     [ ] Source is optional.
 *     [ ] Context is optional.
 *     [ ] Context is open-ended.
 *     [ ] Condition is optional.
 *     [ ] Fallback is optional.
 *     [ ] Clause order is deterministic.
 *     [ ] Executable adaptation remains separately owned.
 *     [ ] Policy declaration remains owned by core/policies.g4.
 *     [ ] Capabilities remain separately owned.
 *     [ ] Resources remain separately owned.
 *     [ ] Effects remain separately owned.
 *     [ ] Contracts remain separately owned.
 *     [ ] Authorization remains separately owned.
 *     [ ] Provenance remains separately owned.
 *     [ ] No adaptation algorithm is enumerated.
 *     [ ] No physical target is encoded.
 *     [ ] No resource ceiling is encoded.
 *     [ ] No hardware capacity is encoded.
 *     [ ] No quantum gate catalogue is encoded.
 *     [ ] quantum::ir remains the canonical quantum IR boundary.
 *     [ ] No second concurrency model is introduced.
 *     [ ] No second execution model is introduced.
 *     [ ] Parsing is deterministic.
 *     [ ] No runtime behavior is embedded.
 *     [ ] Safe Rust integration is preserved.
 *     [ ] Rust 1.97+ compatibility is preserved.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Cross-domain tests exist.
 *     [ ] Scalability tests exist.
 *     [ ] Compatibility tests exist.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "What adaptation behavior does this policy govern?"
 *
 * It does NOT answer:
 *
 *     "How is the adaptation executed?"
 *     "Which algorithm performs it?"
 *     "Which machine performs it?"
 *     "Which processor performs it?"
 *     "Which GPU performs it?"
 *     "Which FPGA performs it?"
 *     "Which QPU performs it?"
 *     "Which qubits are selected?"
 *     "How is it routed?"
 *     "How is it scheduled?"
 *     "How is it error-corrected?"
 *     "How is it realized by the HAL?"
 *
 * Those questions remain downstream.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */