/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/planning.g4
 *
 * GRAMMAR
 * -------
 * Planning
 *
 * STATUS
 * ------
 * CANONICAL AI / PLANNING SEMANTIC-COMPOSITION BOUNDARY
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the source-level grammar boundary for portable planning
 * intent in Zamani.
 *
 * Planning is treated as a generic computational capability rather than as a
 * hardware-specific, robotics-specific, agent-only, or AI-application-only
 * language.
 *
 * Planning may be used for:
 *
 *     classical computation
 *     AI systems
 *     agents
 *     actors
 *     distributed systems
 *     workflow execution
 *     resource allocation
 *     quantum-classical orchestration
 *     HDL/hardware workflows
 *     simulation
 *     recovery
 *     adaptive execution
 *     optimization
 *     scheduling intent
 *     scientific computation
 *     future computational domains
 *
 * This grammar describes:
 *
 *     goals
 *     objectives
 *     actions
 *     preconditions
 *     postconditions
 *     planning steps
 *     dependencies
 *     alternatives
 *     constraints
 *     policies
 *     resource requirements
 *     capability requirements
 *     evidence
 *     provenance
 *     adaptation hooks
 *
 * It does NOT implement:
 *
 *     planning algorithms
 *     search algorithms
 *     optimization algorithms
 *     scheduling algorithms
 *     execution
 *     resource discovery
 *     hardware selection
 *     network discovery
 *     actor scheduling
 *     quantum routing
 *     QEC
 *     physical placement
 *     backend selection
 *
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Planning describes INTENT.
 *
 * Execution describes REALIZATION.
 *
 * Therefore:
 *
 *     source
 *       |
 *       v
 *     planning syntax
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic planning model
 *       |
 *       +--> contracts
 *       +--> policies
 *       +--> capabilities
 *       +--> resources
 *       +--> effects
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware model
 *       +--> distributed model
 *       +--> execution plan
 *       |
 *       v
 *     lowering / optimization / scheduling / routing
 *       |
 *       v
 *     target realization
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     planningConstruct
 *     planningDeclaration
 *     planningGoal
 *     planningObjective
 *     planningAction
 *     planningPrecondition
 *     planningPostcondition
 *     planningStep
 *     planningDependency
 *     planningAlternative
 *     planningConstraint
 *     planningRequirement
 *     planningCapability
 *     planningPolicy
 *     planningEvidence
 *     planningProvenance
 *     planningAdaptation
 *     planningBody
 *     planningMember
 *     planningReference
 *     planningExpression
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical vocabulary
 *     identifiers
 *     qualified names
 *     general expressions
 *     general types
 *     contracts as a whole
 *     policies as a whole
 *     resources as a whole
 *     capabilities as a whole
 *     effects
 *     provenance storage
 *     actor syntax
 *     agent syntax
 *     concurrency syntax
 *     networking
 *     distributed topology
 *     quantum operations
 *     quantum IR
 *     HDL syntax
 *     hardware syntax
 *     scheduling implementation
 *     optimization implementation
 *     search algorithms
 *     runtime execution
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/ai/agents.g4
 *     grammar/concurrency/actors.g4
 *     grammar/types/types.g4
 *     grammar/expressions/expressions.g4
 *     grammar/statements/statements.g4
 *
 * EXISTING UNIVERSAL BOUNDARIES CONSUMED:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     statement
 *     blockExpression
 *
 *
 * EXPORTS:
 *
 *     planningConstruct
 *     planningDeclaration
 *     planningGoal
 *     planningObjective
 *     planningAction
 *     planningPrecondition
 *     planningPostcondition
 *     planningStep
 *     planningDependency
 *     planningAlternative
 *     planningConstraint
 *     planningRequirement
 *     planningCapability
 *     planningPolicy
 *     planningEvidence
 *     planningProvenance
 *     planningAdaptation
 *
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *     grammar/ai/agents.g4
 *     future planning-aware semantic composition
 *     conformance tooling
 *
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 *
 * SEMANTIC_OWNER:
 *
 *     planning semantic model
 *
 *
 * IR_OWNER:
 *
 *     canonical semantic IR
 *     downstream domain IRs
 *     quantum::ir where quantum semantics occur
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/planning/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/specification/
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Planning must NOT create:
 *
 *     PlanningIR
 *     AgentPlanningIR
 *     RobotPlanningIR
 *     QuantumPlanningIR
 *     HardwarePlanningIR
 *
 * Planning semantics are represented in the common semantic model and lowered
 * into the appropriate domain representation.
 *
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     A*
 *     Dijkstra
 *     BFS
 *     DFS
 *     SAT
 *     SMT
 *     MILP
 *     PDDL
 *     HTN
 *     reinforcement-learning planner
 *     vendor planner
 *     robot planner
 *     quantum planner
 *     scheduler implementation
 *
 * Such algorithms are semantic capabilities, libraries, dialects, policies,
 * or compiler/runtime implementations.
 *
 * A future planning algorithm therefore does not require a universal grammar
 * change.
 *
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * Existing lexical vocabulary includes planning-relevant tokens such as:
 *
 *     GOAL
 *     OBJECTIVE
 *     ACTION
 *     PRECONDITION
 *     POSTCONDITION
 *     INFER
 *     DEDUCE
 *     REASON
 *     LEARN
 *     ADAPT
 *
 * This grammar consumes those existing tokens.
 *
 * It MUST NOT define them again.
 *
 * It MUST NOT create duplicate lexer tokens.
 *
 *
 * ============================================================================
 * IMPORTANT KEYWORD / IDENTIFIER RULE
 * ============================================================================
 *
 * Planning must not assume that arbitrary semantic names are lexer keywords.
 *
 * Application concepts remain identifiers.
 *
 * Examples:
 *
 *     navigation
 *     manufacturing
 *     quantum_control
 *     recovery
 *     data_pipeline
 *     scientific_workflow
 *     accelerator_selection
 *
 * remain ordinary names unless explicitly reserved by the language
 * specification.
 *
 *
 * ============================================================================
 * PLANNING MODEL
 * ============================================================================
 *
 * A planning declaration represents:
 *
 *     initial context
 *     goals
 *     objectives
 *     available actions
 *     preconditions
 *     postconditions
 *     dependencies
 *     constraints
 *     requirements
 *     policies
 *     evidence
 *     provenance
 *     adaptation intent
 *
 * It does NOT prescribe how a planner finds a solution.
 *
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

parser grammar Planning;

options {
    tokenVocab = ZamaniLexer;
}

import
    Types,
    Expressions,
    Statements,
    Agents,
    Actors
;


/*
 * ============================================================================
 * 1. PLANNING COMPOSITION
 * ============================================================================
 *
 * The public planning boundary.
 *
 * A planning construct can be a complete declaration or an individual
 * planning semantic declaration.
 */
planningConstruct
    : planningDeclaration
    | planningGoal
    | planningObjective
    | planningAction
    | planningPrecondition
    | planningPostcondition
    | planningStep
    | planningDependency
    | planningAlternative
    | planningConstraint
    | planningRequirement
    | planningCapability
    | planningPolicy
    | planningEvidence
    | planningProvenance
    | planningAdaptation
    ;


/*
 * ============================================================================
 * 2. PLANNING DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     plan Name {
 *         ...
 *     }
 *
 * The existing lexer deliberately does not require a PLAN keyword.
 *
 * Therefore the production grammar uses an existing structural declaration
 * boundary rather than adding another global keyword solely for planning.
 *
 * A planning declaration is represented by:
 *
 *     @plan Name { ... }
 *
 * where `plan` is interpreted semantically.
 *
 * This keeps planning extensible while avoiding another permanent lexical
 * reservation.
 */
planningDeclaration
    : AT
      identifier
      identifier
      planningBody
    ;


/*
 * ============================================================================
 * 3. PLANNING BODY
 * ============================================================================
 *
 * Planning bodies are unbounded at the language level.
 *
 * Actual parser/resource limits belong to implementation configuration.
 */
planningBody
    : LBRACE
      planningMember*
      RBRACE
    ;


planningMember
    : planningGoal
    | planningObjective
    | planningAction
    | planningPrecondition
    | planningPostcondition
    | planningStep
    | planningDependency
    | planningAlternative
    | planningConstraint
    | planningRequirement
    | planningCapability
    | planningPolicy
    | planningEvidence
    | planningProvenance
    | planningAdaptation
    | agentConstruct
    | actorConstruct
    | statement
    ;


/*
 * ============================================================================
 * 4. GOALS
 * ============================================================================
 *
 * Goals describe desired semantic states.
 *
 * The grammar does not require a fixed goal representation.
 */
planningGoal
    : GOAL
      planningGoalTarget
      planningGoalTail
    ;


planningGoalTarget
    : identifier
    | qualifiedName
    | expression
    ;


planningGoalTail
    : SEMI
    | ASSIGN
      expression
      SEMI
    | COLON
      typeExpression
      SEMI
    | blockExpression
    ;


/*
 * ============================================================================
 * 5. OBJECTIVES
 * ============================================================================
 *
 * Objectives may represent optimization preferences, priorities, utility,
 * quality criteria, or other semantic objectives.
 *
 * Their actual mathematical meaning is semantic-layer responsibility.
 */
planningObjective
    : OBJECTIVE
      planningReference
      planningObjectiveTail
    ;


planningObjectiveTail
    : SEMI
    | ASSIGN
      expression
      SEMI
    | COLON
      typeExpression
      SEMI
    | blockExpression
    ;


/*
 * ============================================================================
 * 6. ACTIONS
 * ============================================================================
 *
 * An action represents an abstract operation that may become executable after
 * planning and semantic validation.
 *
 * The action name remains open-ended.
 */
planningAction
    : ACTION
      identifier
      planningActionTail
    ;


planningActionTail
    : LPAREN
      argumentList?
      RPAREN
      planningActionContinuation
    | ASSIGN
      expression
      SEMI
    | COLON
      typeExpression
      planningActionContinuation
    | blockExpression
    | SEMI
    ;


planningActionContinuation
    : blockExpression
    | SEMI
    ;


/*
 * ============================================================================
 * 7. PRECONDITIONS
 * ============================================================================
 *
 * Preconditions constrain action or plan applicability.
 *
 * They are semantic conditions, not runtime assertions by themselves.
 */
planningPrecondition
    : PRECONDITION
      planningCondition
      SEMI?
    ;


planningCondition
    : expression
    ;


/*
 * ============================================================================
 * 8. POSTCONDITIONS
 * ============================================================================
 *
 * Postconditions describe expected resulting conditions.
 */
planningPostcondition
    : POSTCONDITION
      planningCondition
      SEMI?
    ;


/*
 * ============================================================================
 * 9. STEPS
 * ============================================================================
 *
 * Steps are intentionally generic.
 *
 * A step may reference:
 *
 *     an action
 *     an expression
 *     another plan
 *     an agent
 *     an actor
 *     a capability
 *     a future semantic operation
 */
planningStep
    : identifier
      planningStepTail
    ;


planningStepTail
    : COLON
      typeExpression
      planningStepBody
    | ASSIGN
      expression
      planningStepBody
    | LPAREN
      argumentList?
      RPAREN
      planningStepBody
    | planningStepBody
    | SEMI
    ;


planningStepBody
    : blockExpression
    | SEMI
    ;


/*
 * ============================================================================
 * 10. DEPENDENCIES
 * ============================================================================
 *
 * Dependencies are represented structurally instead of requiring a dedicated
 * dependency vocabulary.
 *
 * This keeps the model usable for:
 *
 *     action dependencies
 *     data dependencies
 *     resource dependencies
 *     capability dependencies
 *     temporal dependencies
 *     causal dependencies
 *     distributed dependencies
 */
planningDependency
    : identifier
      identifier
      planningDependencyTail
    ;


planningDependencyTail
    : SEMI
    | ASSIGN
      expression
      SEMI
    | blockExpression
    ;


/*
 * ============================================================================
 * 11. ALTERNATIVES
 * ============================================================================
 *
 * Alternatives represent multiple valid semantic realization choices.
 *
 * The planner or downstream optimizer determines which alternative is chosen.
 */
planningAlternative
    : identifier
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 12. CONSTRAINTS
 * ============================================================================
 *
 * Planning constraints must remain symbolic.
 *
 * No physical resource count is fixed here.
 */
planningConstraint
    : identifier
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 13. RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * Resource requirements are represented through ordinary expressions.
 *
 * Examples of semantic intent:
 *
 *     requires memory >= required_memory
 *     requires qubits >= required_qubits
 *     requires topology(required_topology)
 *
 * This grammar does not resolve those expressions.
 */
planningRequirement
    : identifier
      LPAREN
      argumentList?
      RPAREN
      SEMI
    | identifier
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 14. CAPABILITY REQUIREMENTS
 * ============================================================================
 *
 * Capability names remain open-world.
 *
 * Examples:
 *
 *     capability("tensor.compute")
 *     capability("quantum.measurement")
 *     capability("distributed.consensus")
 *
 * are expressions rather than finite grammar enumerations.
 */
planningCapability
    : identifier
      LPAREN
      argumentList?
      RPAREN
      SEMI
    ;


/*
 * ============================================================================
 * 15. POLICY BINDINGS
 * ============================================================================
 *
 * Policy syntax itself is owned by the policy subsystem.
 *
 * Planning only establishes a composition boundary.
 */
planningPolicy
    : identifier
      planningPolicyTail
    ;


planningPolicyTail
    : expression
      SEMI
    | blockExpression
    | SEMI
    ;


/*
 * ============================================================================
 * 16. EVIDENCE
 * ============================================================================
 *
 * Evidence can support:
 *
 *     goals
 *     actions
 *     assumptions
 *     decisions
 *     planning choices
 *     learned models
 *     runtime adaptations
 *
 * Evidence semantics belong to provenance/evidence subsystems.
 */
planningEvidence
    : identifier
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 17. PROVENANCE
 * ============================================================================
 *
 * Provenance is preserved structurally but stored and interpreted downstream.
 */
planningProvenance
    : identifier
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 18. ADAPTATION
 * ============================================================================
 *
 * Adaptation is controlled semantic change.
 *
 * It is NOT unrestricted self-modifying execution.
 *
 * Authorization, policy, effects, provenance, and capability checks happen
 * downstream.
 */
planningAdaptation
    : ADAPT
      planningAdaptationTarget
      planningAdaptationTail
    ;


planningAdaptationTarget
    : identifier
    | qualifiedName
    | expression
    ;


planningAdaptationTail
    : SEMI
    | ASSIGN
      expression
      SEMI
    | blockExpression
    ;


/*
 * ============================================================================
 * 19. REFERENCES
 * ============================================================================
 *
 * Planning references are deliberately open-world.
 */
planningReference
    : identifier
    | qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 20. EXPRESSION BRIDGE
 * ============================================================================
 *
 * Planning never creates a second expression grammar.
 */
planningExpression
    : expression
    ;


/*
 * ============================================================================
 * 21. SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Planning constructs may semantically interact with:
 *
 *     Types
 *     Effects
 *     Capabilities
 *     Resources
 *     Contracts
 *     Policies
 *     Provenance
 *     Agents
 *     Actors
 *     Concurrency
 *     Distributed execution
 *     Classical computation
 *     Quantum computation
 *     HDL/hardware
 *     Simulation
 *     Interoperability
 *
 * This file only establishes syntax.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Planning values use the universal:
 *
 *     typeExpression
 *
 * Planning does not define:
 *
 *     PlanType
 *     GoalType
 *     ActionType
 *     RobotPlanType
 *     QuantumPlanType
 *
 * as competing universal type systems.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Planning itself is not assigned a fixed implementation effect by this
 * grammar.
 *
 * A semantic planner may require effects such as:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     learning
 *     adaptation
 *     distributed
 *     simulation
 *     foreign
 *     native
 *     reflection
 *
 * Effect analysis remains owned by grammar/effects/ and the semantic layer.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Planning may require capabilities such as:
 *
 *     planner.search
 *     optimizer.solve
 *     tensor.compute
 *     quantum.measurement
 *     distributed.execute
 *     simulation.run
 *
 * The grammar does not enumerate these capabilities.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Planning can express symbolic resource requirements.
 *
 * No fixed machine capacity is encoded.
 *
 * In particular, this file contains no:
 *
 *     maximum agent count
 *     maximum action count
 *     maximum plan depth
 *     maximum resource count
 *     maximum node count
 *     maximum CPU count
 *     maximum GPU count
 *     maximum FPGA count
 *     maximum QPU count
 *     maximum qubit count
 *     maximum memory size
 *     maximum tensor rank
 *
 * Repetition is represented by grammar repetition.
 *
 * Physical feasibility is determined downstream.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Planning can participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Existing contract grammars remain the owners of the full contract system.
 *
 * Planning conditions must therefore be compatible with the universal
 * expression/validation model.
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Planning decisions can be constrained by policies concerning:
 *
 *     authorization
 *     resources
 *     capabilities
 *     adaptation
 *     networking
 *     security
 *     deployment
 *     simulation
 *     foreign calls
 *     native execution
 *
 * Policy evaluation is not performed by ANTLR.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Planning must preserve source provenance.
 *
 * Downstream records may contain:
 *
 *     source
 *     claim
 *     evidence
 *     derivation
 *     decision
 *     transformation
 *     version
 *     execution context
 *
 * The grammar itself does not create timestamps or runtime state.
 *
 *
 * ============================================================================
 * AGENT INTEGRATION
 * ============================================================================
 *
 * Planning can occur inside agents:
 *
 *     @agent Researcher {
 *         ...
 *     }
 *
 * Agent syntax remains owned by:
 *
 *     grammar/ai/agents.g4
 *
 * Planning must not redefine agent declarations.
 *
 *
 * ============================================================================
 * ACTOR INTEGRATION
 * ============================================================================
 *
 * Planning can coordinate actor-based execution.
 *
 * Actor syntax remains owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * This grammar does not create:
 *
 *     planner actors
 *     planning threads
 *     planning processes
 *     planning devices
 *
 * A semantic planner may eventually lower planning actions into actor,
 * task, asynchronous, distributed, classical, quantum, or hardware-backed
 * execution.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Planning may select or describe quantum computation semantically.
 *
 * It must NOT define:
 *
 *     quantum gates
 *     physical qubits
 *     coupling maps
 *     calibration
 *     routing
 *     decomposition
 *     QEC
 *     ZQN
 *     HAL
 *
 * Quantum computation crosses:
 *
 *     planning semantic model
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * The canonical quantum IR remains `quantum::ir`.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Planning may describe hardware realization intent but does not own:
 *
 *     wires
 *     registers
 *     pins
 *     fixed widths
 *     device identifiers
 *     board identifiers
 *     FPGA families
 *     ASIC structures
 *     physical topology
 *
 * Those remain hardware/HDL concerns.
 *
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Planning is compatible with adaptive execution:
 *
 *     detect
 *     evaluate
 *     select
 *     retry
 *     recover
 *     fallback
 *     adapt
 *
 * The execution subsystem decides how these are realized.
 *
 * Planning only expresses the source-level intent and constraints.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * The grammar must not inspect:
 *
 *     hardware
 *     available resources
 *     network state
 *     filesystem state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     runtime state
 *
 * Planning algorithm nondeterminism, if permitted by a future semantic
 * subsystem, must be represented explicitly and must not alter parser
 * behavior.
 *
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file is a pure ANTLR parser grammar.
 *
 * It contains:
 *
 *     no Rust code
 *     no actions
 *     no semantic predicates
 *     no unsafe operations
 *     no filesystem access
 *     no network access
 *     no hardware probing
 *     no runtime execution
 *
 * Rust 1.97+ compatibility is therefore established at generated-parser
 * integration rather than by embedding Rust in this grammar.
 *
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar uses unbounded structural repetition:
 *
 *     planningMember*
 *
 * and:
 *
 *     ( ... )*
 *
 * rather than finite enumerations.
 *
 * This permits the same source language model to describe plans containing:
 *
 *     very few steps
 *     many steps
 *     nested plans
 *     distributed workflows
 *     heterogeneous execution
 *     arbitrarily large symbolic resource descriptions
 *
 * subject only to actual implementation resources.
 *
 * "Infinity" therefore means:
 *
 *     no language-imposed artificial ceiling.
 *
 * It does not claim infinite physical memory, execution time, storage,
 * bandwidth, or compute.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_AGENTS
 *     MAX_PLANS
 *     MAX_GOALS
 *     MAX_ACTIONS
 *     MAX_STEPS
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * It must not encode:
 *
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     QPU identifiers
 *     physical qubit identifiers
 *     vendor-specific planner algorithms
 *     fixed topology
 *     fixed scheduler
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must identify:
 *
 *     malformed planning declarations
 *     malformed goals
 *     malformed objectives
 *     malformed actions
 *     malformed preconditions
 *     malformed postconditions
 *     malformed steps
 *     malformed adaptation constructs
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     unknown goal
 *     impossible precondition
 *     invalid action
 *     unsatisfied capability
 *     unavailable resource
 *     forbidden policy
 *     invalid contract
 *     invalid type
 *     invalid effect
 *     invalid provenance
 *     unsupported target realization
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following semantic shapes must be testable:
 *
 *     @plan Research {
 *         goal solve_problem;
 *     }
 *
 *     @plan Compute {
 *         objective minimize_cost;
 *         action acquire_data;
 *         precondition data_available;
 *         postcondition result_available;
 *     }
 *
 *     @plan Hybrid {
 *         goal result;
 *         action execute;
 *         precondition capability("quantum.measurement");
 *         postcondition result_ready;
 *     }
 *
 *     @plan Adaptive {
 *         goal stable_result;
 *         adapt strategy;
 *     }
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Tests must reject malformed constructs such as:
 *
 *     @plan {
 *     @plan Name
 *     goal;
 *     action;
 *     precondition;
 *     postcondition
 *
 * according to the exact syntax accepted by the final lexical/parser
 * composition.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Required cross-domain tests include:
 *
 *     planning + classical
 *     planning + quantum
 *     planning + hybrid
 *     planning + HDL
 *     planning + hardware
 *     planning + AI
 *     planning + agents
 *     planning + actors
 *     planning + distributed execution
 *     planning + networking
 *     planning + simulation
 *     planning + contracts
 *     planning + policies
 *     planning + resources
 *     planning + capabilities
 *     planning + provenance
 *     planning + interoperability
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must cover:
 *
 *     empty planning body where semantically permitted
 *     single-step plan
 *     deeply nested plan
 *     many independent actions
 *     many dependencies
 *     symbolic resource expressions
 *     symbolic capability expressions
 *     large qualified names
 *     large expression trees
 *     cross-domain plans
 *
 * No test may encode a universal maximum as part of language semantics.
 *
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Planning syntax must follow the repository compatibility architecture.
 *
 * Deprecated spellings must not be recreated here as duplicate lexer tokens.
 *
 * Compatibility aliases belong under:
 *
 *     grammar/compatibility/
 *
 * and are resolved before or during semantic compatibility processing.
 *
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/ai/ai.g4`
 * ============================================================================
 *
 * `AI` should become the composition owner of Planning.
 *
 * Required integration:
 *
 *     AI
 *       |
 *       +--> Planning
 *       +--> Agents
 *       +--> Models
 *       +--> Datasets
 *       +--> Tensors
 *       +--> Training
 *       +--> Inference
 *       +--> ...
 *
 * To avoid duplicate ownership, `ai.g4` should import `Planning` and expose:
 *
 *     planningConstruct
 *
 * through `aiConstruct`.
 *
 * The existing direct `Agents` import may remain only if `AI` directly
 * exposes `agentConstruct` independently. If Planning imports Agents and the
 * ANTLR build reports duplicate imported rule ownership, the canonical fix is
 * to make `AI` import `Planning` and remove its redundant direct `Agents`
 * import, provided all existing agent consumers continue to receive
 * `agentConstruct` through the planning composition.
 *
 * This integration choice must be verified by ANTLR generation rather than
 * assumed.
 *
 *
 * ============================================================================
 * INTEGRATION WITH ROOT PARSER
 * ============================================================================
 *
 * The root parser must eventually expose:
 *
 *     planningConstruct
 *
 * through the domain-neutral construct dispatch.
 *
 * The root grammar must not duplicate planning rules.
 *
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * Parser contexts map into the existing domain-neutral AST.
 *
 * Suggested semantic node:
 *
 *     PlanningConstruct
 *
 * containing source-preserved structures for:
 *
 *     goals
 *     objectives
 *     actions
 *     conditions
 *     dependencies
 *     constraints
 *     requirements
 *     capabilities
 *     policies
 *     evidence
 *     provenance
 *     adaptation
 *
 * The AST must not encode:
 *
 *     planner implementation
 *     search algorithm
 *     CPU/GPU/QPU
 *     physical node
 *     physical qubit
 *     vendor runtime
 *
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic validation must resolve:
 *
 *     names
 *     types
 *     expressions
 *     action applicability
 *     goal satisfiability
 *     preconditions
 *     postconditions
 *     dependencies
 *     requirements
 *     capabilities
 *     contracts
 *     policies
 *     effects
 *     provenance
 *
 * Planning algorithm selection occurs only after semantic validation.
 *
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * Planning has no independent universal IR.
 *
 * A semantic plan can lower into:
 *
 *     canonical semantic IR
 *
 * and from there into:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed execution representation
 *     runtime execution plan
 *
 * depending on semantic content.
 *
 *
 * ============================================================================
 * TEST LOCATION
 * ============================================================================
 *
 * Recommended:
 *
 *     grammar/tests/ai/planning/
 *
 * With:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     compatibility/
 *     determinism/
 *     cross-domain/
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] It compiles as an ANTLR parser grammar.
 *
 * [ ] It uses the canonical ZamaniLexer.
 *
 * [ ] It imports only existing grammar authorities.
 *
 * [ ] It introduces no duplicate lexer tokens.
 *
 * [ ] It introduces no second expression grammar.
 *
 * [ ] It introduces no second type grammar.
 *
 * [ ] It introduces no second agent grammar.
 *
 * [ ] It introduces no second actor grammar.
 *
 * [ ] It introduces no planning-specific IR.
 *
 * [ ] It introduces no target-specific syntax.
 *
 * [ ] It introduces no physical hardware assumptions.
 *
 * [ ] It introduces no universal resource ceilings.
 *
 * [ ] It contains no Rust actions.
 *
 * [ ] It contains no unsafe code.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass within available implementation resources.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] `AI` composition exposes planning without creating duplicate rule
 *     ownership.
 *
 * [ ] Root parser integration exposes planning exactly once.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Planning is an expression of computational intent.
 *
 * It is not:
 *
 *     a robot language
 *     a workflow-only language
 *     an agent-only language
 *     a scheduler language
 *     a quantum language
 *     a hardware language
 *
 * It is a universal semantic capability that can coordinate any of those
 * domains while preserving the POCO-REAF property:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 *
 * subject to actual resource, capability, policy, compatibility, and physical
 * feasibility constraints.
 *
 * ============================================================================
 */