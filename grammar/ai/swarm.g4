/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/swarm.g4
 *
 * GRAMMAR
 * -------
 * AISwarm
 *
 * STATUS
 * ------
 * CANONICAL COLLECTIVE-AGENT / SWARM COMPOSITION GRAMMAR
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
 * This file defines the source-level grammar boundary for scalable
 * collective-agent computation.
 *
 * A swarm is a semantic composition of independently addressable agents
 * and/or agent-like computational participants operating under a shared
 * intent.
 *
 * The swarm abstraction is deliberately independent of:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     node count
 *     device count
 *     thread count
 *     process count
 *     memory capacity
 *     network topology
 *     physical agent count
 *     hardware vendor
 *     runtime implementation
 *
 * The grammar describes collective intent.
 *
 * It does NOT describe:
 *
 *     physical placement
 *     process allocation
 *     thread allocation
 *     device allocation
 *     network routing
 *     hardware topology
 *     scheduler implementation
 *     actor runtime implementation
 *     agent runtime implementation
 *     distributed deployment
 *     model execution
 *     quantum execution
 *     QEC
 *     ZQN
 *     HAL
 *
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * A swarm is NOT a new execution substrate.
 *
 * The semantic relationship is:
 *
 *     swarm
 *       |
 *       +--> agents
 *       +--> actors
 *       +--> tasks
 *       +--> messages
 *       +--> planning
 *       +--> capabilities
 *       +--> resources
 *       +--> policies
 *       +--> effects
 *       +--> contracts
 *       +--> provenance
 *       |
 *       v
 *     domain-neutral semantic model
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL / hardware
 *       +--> distributed
 *       +--> networking
 *       +--> simulation
 *       |
 *       v
 *     execution planning
 *       |
 *       v
 *     target realization
 *
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/ai/agents.g4
 *
 * owns individual agent syntax.
 *
 * Existing:
 *
 *     grammar/concurrency/actors.g4
 *
 * owns actor syntax and lifecycle.
 *
 * Existing:
 *
 *     grammar/ai/planning.g4
 *
 * owns planning intent.
 *
 * Existing:
 *
 *     grammar/distributed/
 *
 * owns distributed computation semantics.
 *
 * A swarm needs a composition boundary that combines those concepts without
 * redefining them.
 *
 * Therefore this file owns only:
 *
 *     collective membership
 *     collective identity
 *     collective body
 *     collective coordination intent
 *     collective delegation intent
 *     collective policy/capability/resource attachments
 *     collective provenance/observability attachments
 *
 *
 * ============================================================================
 * IMPORTANT SYNTAX DECISION
 * ============================================================================
 *
 * The canonical agent grammar is intentionally open-world:
 *
 *     @identifier ...
 *
 * Consequently, a conventional:
 *
 *     @swarm Name { ... }
 *
 * would structurally overlap with:
 *
 *     agentConstruct
 *
 * because `swarm` is itself an identifier.
 *
 * This file therefore uses a distinct structural boundary:
 *
 *     @swarm[member_a, member_b] {
 *         ...
 *     }
 *
 * The opening LBRACKET makes the construct unambiguous relative to the
 * existing agent grammar.
 *
 * `swarm` remains an identifier rather than becoming a permanently reserved
 * application-specific keyword.
 *
 * If a future language-version policy chooses to reserve a dedicated lexical
 * token, that migration belongs to the lexer/compatibility architecture and
 * does NOT require changing the semantic swarm model.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     swarmConstruct
 *     swarmDeclaration
 *     swarmMembership
 *     swarmMember
 *     swarmMemberList
 *     swarmBody
 *     swarmBodyMember
 *     swarmAnnotation
 *     swarmAnnotationTail
 *     swarmInvocation
 *     swarmNamedConstruct
 *     swarmBinding
 *     swarmTypedConstruct
 *     swarmBlockConstruct
 *     swarmReference
 *     swarmExpression
 *     swarmType
 *     swarmObjective
 *     swarmGoal
 *     swarmCoordination
 *     swarmDelegation
 *     swarmCommunication
 *     swarmPolicy
 *     swarmRequirement
 *     swarmCapability
 *     swarmConstraint
 *     swarmPreference
 *     swarmHint
 *     swarmEvidence
 *     swarmProvenance
 *     swarmAdaptation
 *     swarmSimulation
 *     swarmBodyStatement
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical tokens
 *     identifiers
 *     qualified names
 *     general expressions
 *     general types
 *     ordinary statements
 *     agent declarations
 *     actor declarations
 *     actor lifecycle
 *     message transport
 *     channel semantics
 *     task semantics
 *     scheduling
 *     distributed topology
 *     networking
 *     planning algorithms
 *     learning algorithms
 *     reasoning algorithms
 *     adaptation algorithms
 *     model semantics
 *     tensor semantics
 *     quantum operations
 *     quantum topology
 *     QEC
 *     ZQN
 *     HDL syntax
 *     hardware syntax
 *     resource discovery
 *     capability discovery
 *     security enforcement
 *     policy evaluation
 *     provenance storage
 *     runtime execution
 *     target selection
 *     physical placement
 *     classical IR
 *     swarm IR
 *     agent IR
 *     actor IR
 *     quantum IR
 *     backend implementation
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/types/types.g4
 *     grammar/expressions/expressions.g4
 *     grammar/statements/statements.g4
 *     grammar/ai/agents.g4
 *     grammar/concurrency/actors.g4
 *
 * Optional semantic consumers:
 *
 *     grammar/ai/planning.g4
 *     grammar/resources/
 *     grammar/effects/
 *     grammar/validation/
 *     grammar/policies/
 *     grammar/security/
 *     grammar/distributed/
 *     grammar/execution/
 *
 *
 * EXPORTS:
 *
 *     swarmConstruct
 *     swarmDeclaration
 *     swarmMembership
 *     swarmBody
 *     swarmMember
 *     swarmBodyMember
 *     swarmReference
 *     swarmObjective
 *     swarmGoal
 *     swarmCoordination
 *     swarmDelegation
 *     swarmCommunication
 *     swarmPolicy
 *     swarmRequirement
 *     swarmCapability
 *     swarmConstraint
 *     swarmPreference
 *     swarmHint
 *     swarmEvidence
 *     swarmProvenance
 *     swarmAdaptation
 *     swarmSimulation
 *
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *     future AI orchestration composition
 *     planning composition
 *     semantic analysis
 *     conformance tooling
 *
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral frontend AST
 *
 *
 * SEMANTIC_OWNER:
 *
 *     AI collective/swarm semantic layer
 *
 *
 * IR_OWNER:
 *
 *     Existing canonical semantic IR
 *     Existing concurrency/distributed semantic representations
 *     quantum::ir where quantum computation occurs
 *
 *     NO SwarmIR is introduced by this grammar.
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/swarm/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/specification/
 *
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer remains:
 *
 *     ZamaniLexer
 *
 * This grammar deliberately does NOT introduce:
 *
 *     SWARM
 *     AGENT_SWARM
 *     SWARM_MEMBER
 *     SWARM_SIZE
 *     SWARM_NODE
 *
 * as new lexical tokens.
 *
 * The word `swarm` is structurally consumed as an identifier.
 *
 * This keeps the language open-world and prevents every future collective
 * computational abstraction from requiring a new lexer keyword.
 *
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 */

parser grammar AISwarm;

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
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `swarmConstruct` is the single public swarm composition boundary.
 *
 * The distinctive membership bracket makes this construct structurally
 * distinguishable from the open-world agent grammar.
 *
 * Canonical form:
 *
 *     @swarm[agent_a, agent_b] {
 *         ...
 *     }
 *
 * The member list is optional so that an initially empty semantic swarm can
 * be declared and populated by downstream semantic mechanisms where the
 * language specification permits it.
 *
 * ============================================================================
 */

swarmConstruct
    : swarmDeclaration
    ;


/*
 * ============================================================================
 * 2. SWARM DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     @swarm[member_a, member_b] {
 *         ...
 *     }
 *
 * The identifier `swarm` is intentionally parsed as an identifier rather than
 * a reserved keyword.
 *
 * The semantic layer validates the annotation role.
 *
 * ============================================================================
 */

swarmDeclaration
    : AT
      identifier
      swarmMembership
      swarmBody
    ;


/*
 * ============================================================================
 * 3. SWARM MEMBERSHIP
 * ============================================================================
 *
 * Membership is symbolic.
 *
 * It does not imply:
 *
 *     physical process
 *     CPU
 *     GPU
 *     node
 *     device
 *     network endpoint
 *     thread
 *     quantum processor
 *
 * The same symbolic member may eventually be realized through:
 *
 *     local execution
 *     actor execution
 *     distributed execution
 *     accelerator execution
 *     simulation
 *     quantum/classical hybrid execution
 *     another valid execution substrate
 *
 * ============================================================================
 */

swarmMembership
    : LBRACKET
      swarmMemberList?
      RBRACKET
    ;


swarmMemberList
    : swarmMember
      (
          COMMA
          swarmMember
      )*
    ;


/*
 * ============================================================================
 * 4. SWARM MEMBER
 * ============================================================================
 *
 * A member is an ordinary symbolic reference.
 *
 * Qualified names are supported through the canonical expression/name
 * architecture.
 *
 * Examples:
 *
 *     planner
 *     worker
 *     agents::planner
 *     distributed::worker
 *     quantum::controller
 *
 * The parser does not decide what those names denote.
 *
 * ============================================================================
 */

swarmMember
    : swarmReference
    ;


swarmReference
    : expression
    ;


/*
 * ============================================================================
 * 5. SWARM BODY
 * ============================================================================
 *
 * A swarm body contains zero or more semantic members.
 *
 * There is no fixed number of:
 *
 *     goals
 *     objectives
 *     agents
 *     coordination operations
 *     messages
 *     policies
 *     requirements
 *     capabilities
 *     constraints
 *     adaptations
 *     simulations
 *     statements
 *
 * ============================================================================
 */

swarmBody
    : LBRACE
      swarmBodyMember*
      RBRACE
    ;


swarmBodyMember
    : swarmBodyAnnotation
    | swarmGoal
    | swarmObjective
    | swarmCoordination
    | swarmDelegation
    | swarmCommunication
    | swarmPolicy
    | swarmRequirement
    | swarmCapability
    | swarmConstraint
    | swarmPreference
    | swarmHint
    | swarmEvidence
    | swarmProvenance
    | swarmAdaptation
    | swarmSimulation
    | swarmBodyStatement
    ;


/*
 * ============================================================================
 * 6. GENERIC SWARM ANNOTATION
 * ============================================================================
 *
 * Open-world extension point.
 *
 * Examples:
 *
 *     @observe collective_state;
 *     @checkpoint state;
 *     @monitor metrics;
 *
 * Application-specific concepts remain identifiers.
 *
 * The semantic layer determines whether an annotation is recognized.
 *
 * ============================================================================
 */

swarmBodyAnnotation
    : swarmAnnotation
      swarmAnnotationTail
    ;


swarmAnnotation
    : AT
      identifier
    ;


swarmAnnotationTail
    : swarmInvocation
    | swarmNamedConstruct
    | swarmBinding
    | swarmTypedConstruct
    | swarmBlockConstruct
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 7. DIRECT ANNOTATION INVOCATION
 * ============================================================================
 */

swarmInvocation
    : LPAREN
      swarmArgumentList?
      RPAREN
      swarmInvocationTail
    ;


swarmInvocationTail
    : swarmBody
    | SEMICOLON
    ;


swarmArgumentList
    : expression
      (
          COMMA
          expression
      )*
    ;


/*
 * ============================================================================
 * 8. NAMED SWARM CONSTRUCT
 * ============================================================================
 *
 * Examples:
 *
 *     @policy execution { ... }
 *     @checkpoint state;
 *     @strategy strategy;
 *
 * The names remain open-world.
 *
 * ============================================================================
 */

swarmNamedConstruct
    : identifier
      swarmNamedConstructTail
    ;


swarmNamedConstructTail
    : swarmInvocation
    | swarmTypedInitializer
    | swarmTypedConstruct
    | swarmInitializer
    | swarmBody
    | SEMICOLON
    ;


swarmBinding
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


swarmTypedInitializer
    : COLON
      typeExpression
      ASSIGN
      expression
      SEMICOLON
    ;


swarmTypedConstruct
    : COLON
      typeExpression
      swarmTypedTail
    ;


swarmTypedTail
    : swarmBody
    | SEMICOLON
    ;


swarmInitializer
    : ASSIGN
      expression
      SEMICOLON
    ;


swarmBlockConstruct
    : swarmBody
    ;


/*
 * ============================================================================
 * 9. GOALS
 * ============================================================================
 *
 * A goal is an abstract desired semantic condition.
 *
 * No planning algorithm is implied.
 *
 * ============================================================================
 */

swarmGoal
    : identifier
      swarmGoalTail
    ;


swarmGoalTail
    : ASSIGN
      expression
      SEMICOLON
    | COLON
      expression
      SEMICOLON
    | swarmBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 10. OBJECTIVES
 * ============================================================================
 *
 * Objectives describe desired properties of collective execution.
 *
 * Examples:
 *
 *     objective = quality;
 *     objective = minimize(cost);
 *     objective = maximize(throughput);
 *
 * These are expressions, not fixed optimization algorithms.
 *
 * ============================================================================
 */

swarmObjective
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. COORDINATION
 * ============================================================================
 *
 * Coordination describes collective intent.
 *
 * Examples:
 *
 *     @coordinate group;
 *     @coordinate(group);
 *
 * The exact coordination algorithm remains semantic/runtime policy.
 *
 * ============================================================================
 */

swarmCoordination
    : AT
      identifier
      LPAREN
      swarmArgumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. DELEGATION
 * ============================================================================
 *
 * Delegation expresses intent to assign work or responsibility.
 *
 * Example:
 *
 *     @delegate(worker, task);
 *
 * The semantic layer determines:
 *
 *     authorization
 *     capability requirements
 *     resource requirements
 *     policy
 *     scheduling
 *     execution mechanism
 *
 * ============================================================================
 */

swarmDelegation
    : AT
      identifier
      LPAREN
      swarmArgumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. COMMUNICATION
 * ============================================================================
 *
 * Communication is represented as intent.
 *
 * The actual transport belongs to concurrency/networking/distributed
 * subsystems.
 *
 * Example:
 *
 *     @message(receiver, payload);
 *
 * ============================================================================
 */

swarmCommunication
    : AT
      identifier
      LPAREN
      swarmArgumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. POLICY
 * ============================================================================
 *
 * Policies remain semantic policies.
 *
 * This grammar does not evaluate authorization.
 *
 * Examples:
 *
 *     @policy execution { ... }
 *     @policy security { ... }
 *
 * ============================================================================
 */

swarmPolicy
    : AT
      identifier
      identifier
      swarmBody
    | AT
      identifier
      LPAREN
      swarmArgumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. REQUIREMENTS
 * ============================================================================
 *
 * Resource/capability requirements are symbolic.
 *
 * Examples:
 *
 *     @requires(memory >= required_memory);
 *
 *     @requires(capability("tensor.compute"));
 *
 *     @requires(capability("quantum.measurement"));
 *
 *     @requires(topology(required_topology));
 *
 * No physical target is selected here.
 *
 * ============================================================================
 */

swarmRequirement
    : AT
      identifier
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. CAPABILITIES
 * ============================================================================
 *
 * Capabilities remain open-world semantic identifiers.
 *
 * This grammar deliberately does not enumerate:
 *
 *     AI algorithms
 *     accelerator models
 *     vendor features
 *     quantum devices
 *     hardware devices
 *     communication protocols
 *
 * ============================================================================
 */

swarmCapability
    : AT
      identifier
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. CONSTRAINTS
 * ============================================================================
 */

swarmConstraint
    : AT
      identifier
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. PREFERENCES
 * ============================================================================
 */

swarmPreference
    : AT
      identifier
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. HINTS
 * ============================================================================
 */

swarmHint
    : AT
      identifier
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. EVIDENCE
 * ============================================================================
 *
 * Evidence is preserved as semantic information.
 *
 * Verification and trust decisions remain downstream.
 *
 * ============================================================================
 */

swarmEvidence
    : AT
      identifier
      LPAREN
      swarmArgumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. PROVENANCE
 * ============================================================================
 *
 * Provenance remains a semantic/audit concern.
 *
 * The grammar only preserves source structure.
 *
 * ============================================================================
 */

swarmProvenance
    : AT
      identifier
      LPAREN
      swarmArgumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. ADAPTATION
 * ============================================================================
 *
 * Adaptation is controlled semantic evolution.
 *
 * It is NOT unrestricted self-modifying execution.
 *
 * Semantic validation must establish:
 *
 *     authorization
 *     policy
 *     capabilities
 *     effects
 *     resource requirements
 *     provenance
 *     contracts
 *
 * ============================================================================
 */

swarmAdaptation
    : AT
      identifier
      LPAREN
      swarmArgumentList?
      RPAREN
      SEMICOLON
    | AT
      identifier
      swarmBody
    ;


/*
 * ============================================================================
 * 23. SIMULATION
 * ============================================================================
 *
 * Simulation is an execution strategy.
 *
 * It may represent:
 *
 *     classical simulation
 *     quantum simulation
 *     distributed simulation
 *     AI simulation
 *     hardware simulation
 *     fault simulation
 *
 * The grammar does not create a separate simulation language.
 *
 * ============================================================================
 */

swarmSimulation
    : AT
      identifier
      LPAREN
      swarmArgumentList?
      RPAREN
      SEMICOLON
    | AT
      identifier
      swarmBody
    ;


/*
 * ============================================================================
 * 24. ORDINARY ZAMANI STATEMENTS
 * ============================================================================
 *
 * Swarm bodies may contain normal Zamani statements.
 *
 * This is essential for universal composition.
 *
 * A swarm therefore does not become an isolated DSL.
 *
 * ============================================================================
 */

swarmBodyStatement
    : statement
    ;


/*
 * ============================================================================
 * 25. EXPRESSION BRIDGE
 * ============================================================================
 *
 * Explicit named bridge for semantic tooling.
 *
 * ============================================================================
 */

swarmExpression
    : expression
    ;


/*
 * ============================================================================
 * 26. TYPE BRIDGE
 * ============================================================================
 */

swarmType
    : typeExpression
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The AST layer must preserve:
 *
 *     source span
 *     swarm declaration span
 *     annotation name
 *     member order
 *     member references
 *     body order
 *     nesting
 *     expressions
 *     types
 *     annotations
 *     source provenance
 *
 * The AST MUST NOT encode:
 *
 *     physical agent count
 *     CPU assignment
 *     GPU assignment
 *     FPGA assignment
 *     QPU assignment
 *     node assignment
 *     device assignment
 *     network route
 *     process ID
 *     thread ID
 *     hardware vendor
 *     physical topology
 *
 * There is no `SwarmIR` produced by this grammar.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether the swarm annotation is valid;
 *     whether members resolve;
 *     whether members are compatible;
 *     whether duplicate membership is meaningful;
 *     whether coordination is valid;
 *     whether delegation is authorized;
 *     whether communication is permitted;
 *     whether policies apply;
 *     whether capabilities are satisfied;
 *     whether resource requirements are satisfiable;
 *     whether effects are permitted;
 *     whether adaptation is authorized;
 *     whether simulation is valid;
 *     whether contracts hold;
 *     whether provenance is complete enough;
 *     whether the collective computation is portable.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Swarm members are expressions.
 *
 * Their types are determined by the canonical type system.
 *
 * No:
 *
 *     SwarmType
 *     AgentType
 *     ActorType
 *     DeviceType
 *
 * type universe is created here.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Swarm operations may require effects such as:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * This grammar does not define effects.
 *
 * The existing effects subsystem determines the effective effect set.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements are symbolic.
 *
 * Examples include:
 *
 *     capability("agent.reasoning")
 *     capability("distributed.communication")
 *     capability("tensor.compute")
 *     capability("quantum.measurement")
 *
 * New capabilities do not require this grammar to change.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements are semantic expressions.
 *
 * The grammar imposes no fixed requirement quantities.
 *
 * Valid concepts include:
 *
 *     required_memory
 *     required_qubits
 *     required_bandwidth
 *     required_capacity
 *     required_topology
 *
 * Their actual values are resolved downstream.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Swarms may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Those contracts remain owned by the validation/contract subsystem.
 *
 * This grammar does not duplicate contract semantics.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Swarms may be governed by policies covering:
 *
 *     membership
 *     delegation
 *     communication
 *     capabilities
 *     resources
 *     adaptation
 *     simulation
 *     foreign calls
 *     networking
 *     security
 *     deployment
 *
 * Policy evaluation remains downstream.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Semantic provenance may record:
 *
 *     source
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     version
 *     execution context
 *
 * The grammar preserves source structure but does not create provenance
 * records itself.
 *
 *
 * ============================================================================
 * AGENT INTEGRATION
 * ============================================================================
 *
 * Individual agent syntax remains owned by:
 *
 *     grammar/ai/agents.g4
 *
 * This file references agents through expressions and does not redefine:
 *
 *     agentConstruct
 *     agentAnnotation
 *     agentBody
 *     agentMember
 *
 * A semantic member can resolve to an agent declaration.
 *
 *
 * ============================================================================
 * ACTOR / CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * A swarm can be realized through the existing concurrency system.
 *
 * The relationship is:
 *
 *     swarm
 *       |
 *       v
 *     agent intent
 *       |
 *       v
 *     actor/task/message semantics
 *       |
 *       v
 *     scheduler/runtime
 *
 * This file does not define:
 *
 *     actor lifecycle
 *     channel semantics
 *     task scheduling
 *     synchronization
 *     worker allocation
 *
 *
 * ============================================================================
 * PLANNING INTEGRATION
 * ============================================================================
 *
 * Swarms may participate in planning.
 *
 * Planning remains owned by:
 *
 *     grammar/ai/planning.g4
 *
 * This grammar does not duplicate:
 *
 *     planningConstruct
 *     planningGoal
 *     planningAction
 *     planningStep
 *
 * A swarm may contain or reference planning constructs through the normal
 * semantic composition path.
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A swarm may be realized:
 *
 *     locally
 *     across processes
 *     across devices
 *     across nodes
 *     across heterogeneous targets
 *     through simulation
 *
 * The grammar does not specify which realization occurs.
 *
 * Distributed placement belongs downstream.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Swarm members may coordinate computation involving quantum semantics.
 *
 * The grammar does not define:
 *
 *     gates
 *     qubits
 *     physical qubit IDs
 *     topology
 *     routing
 *     calibration
 *     QEC
 *
 * The canonical quantum path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
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
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Swarm coordination can semantically control hardware/software workflows.
 *
 * This grammar does not own:
 *
 *     signal syntax
 *     wire syntax
 *     bus widths
 *     registers
 *     pins
 *     device identifiers
 *     accelerator identifiers
 *     clock constraints
 *     physical topology
 *
 * Hardware/HDL grammars remain authoritative.
 *
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * The grammar does not enumerate:
 *
 *     planning algorithms
 *     optimization algorithms
 *     communication protocols
 *     AI models
 *     learning algorithms
 *     reasoning algorithms
 *     swarm algorithms
 *     consensus algorithms
 *     scheduling algorithms
 *     hardware vendors
 *     accelerators
 *     quantum devices
 *     deployment providers
 *
 * Such concepts are:
 *
 *     libraries
 *     dialects
 *     capabilities
 *     policies
 *     semantic metadata
 *     compiler/runtime strategies
 *
 * This means adding a new swarm algorithm does not require modifying this
 * grammar.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes NO language-level maximum on:
 *
 *     swarm count
 *     member count
 *     nested swarm count
 *     goal count
 *     objective count
 *     coordination count
 *     delegation count
 *     message count
 *     policy count
 *     requirement count
 *     capability count
 *     evidence count
 *     provenance entries
 *     adaptation operations
 *     simulation operations
 *     task count
 *     agent count
 *     actor count
 *     process count
 *     node count
 *     device count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     network size
 *
 * Repetition is expressed using:
 *
 *     *
 *     +
 *     ?
 *
 * and symbolic expressions.
 *
 * "Unbounded" means that the language does not impose an artificial ceiling.
 * Actual implementations remain constrained by available computational
 * resources.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO machine-capacity constants
 *     NO swarm-size constants
 *     NO member-count constants
 *     NO agent-count constants
 *     NO worker-count constants
 *     NO node-count constants
 *     NO device-count constants
 *     NO CPU-count constants
 *     NO GPU-count constants
 *     NO FPGA-count constants
 *     NO QPU-count constants
 *     NO qubit-count constants
 *     NO memory-size constants
 *     NO network-size constants
 *     NO topology-size constants
 *     NO vendor identifiers
 *     NO physical placement identifiers
 *
 * The grammar therefore remains suitable for:
 *
 *     tiny execution
 *     embedded execution
 *     local execution
 *     multicore execution
 *     accelerator execution
 *     quantum execution
 *     HPC execution
 *     distributed execution
 *     cloud execution
 *     future computational substrates
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexer vocabulary
 *     grammar version
 *     parser configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     available resources
 *     runtime state
 *     network state
 *     filesystem state
 *     scheduler state
 *     randomness
 *     wall-clock time
 *
 * The same source and parser configuration must produce equivalent parse
 * structure.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This is a pure ANTLR parser grammar.
 *
 * It contains:
 *
 *     no embedded Rust
 *     no actions
 *     no semantic predicates
 *     no filesystem operations
 *     no network operations
 *     no hardware operations
 *     no runtime execution
 *
 * Generated Rust integration remains compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and requires no unsafe Rust.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should distinguish:
 *
 *     missing @
 *     missing swarm annotation identifier
 *     missing '['
 *     malformed member list
 *     missing ']'
 *     missing swarm body
 *     malformed body construct
 *     missing ';'
 *     malformed invocation
 *
 * Semantic diagnostics are downstream and include:
 *
 *     unknown member
 *     invalid member type
 *     unavailable capability
 *     unsatisfied resource requirement
 *     unauthorized delegation
 *     forbidden communication
 *     invalid policy
 *     invalid adaptation
 *     impossible realization
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite must accept forms such as:
 *
 *     @swarm[] {
 *     }
 *
 *     @swarm[planner] {
 *     }
 *
 *     @swarm[planner, worker] {
 *     }
 *
 *     @swarm[agents::planner, agents::worker] {
 *     }
 *
 *     @swarm[planner, worker] {
 *         @goal result = target;
 *     }
 *
 *     @swarm[planner, worker] {
 *         @coordinate(group);
 *     }
 *
 *     @swarm[planner, worker] {
 *         @delegate(worker, task);
 *     }
 *
 *     @swarm[planner, quantum::controller] {
 *         @requires(capability("quantum.measurement"));
 *     }
 *
 *     @swarm[planner, accelerator::worker] {
 *         @requires(capability("tensor.compute"));
 *     }
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite must reject:
 *
 *     @swarm
 *     @swarm[]
 *     @swarm[
 *     @swarm[planner
 *     @swarm[, planner] {
 *     }
 *     @swarm[planner,] {
 *     }
 *     @swarm[planner worker] {
 *     }
 *
 * NOTE:
 *
 * Whether an empty membership list is semantically legal is a language policy
 * decision. The parser permits it so that dynamic/late-bound collective
 * construction remains possible; semantic validation may reject it in a
 * context where at least one member is required.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     one member
 *     many members
 *     qualified members
 *     expression-based members
 *     nested swarm references
 *     swarm plus agents
 *     swarm plus actors
 *     swarm plus planning
 *     swarm plus contracts
 *     swarm plus policies
 *     swarm plus capabilities
 *     swarm plus resource requirements
 *     swarm plus quantum computation
 *     swarm plus HDL/hardware intent
 *     swarm plus distributed execution
 *     swarm plus simulation
 *     swarm plus provenance
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must progressively exercise:
 *
 *     large member lists
 *     deeply nested bodies
 *     many collective operations
 *     long qualified names
 *     large expressions
 *     large policy bodies
 *     large capability expressions
 *
 * Tests must interpret resource exhaustion as an implementation limitation,
 * not as a language-defined swarm limit.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Verify swarm composition with:
 *
 *     classical
 *     AI
 *     reasoning
 *     learning
 *     adaptation
 *     data
 *     tensor
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     concurrency
 *     distributed
 *     networking
 *     security
 *     simulation
 *     interoperability
 *     metaprogramming
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar must not recreate legacy swarm syntax as duplicate parser
 * branches.
 *
 * Historical swarm constructs belong under:
 *
 *     grammar/compatibility/
 *
 * Compatibility adapters may translate historical syntax into the canonical
 * swarm semantic model.
 *
 * The canonical model remains independent of historical syntax.
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. LEXER
 *
 *    No new lexical token is required.
 *
 *    `swarm` remains an identifier.
 *
 *
 * 2. AI COMPOSITION
 *
 *    grammar/ai/ai.g4 imports:
 *
 *        AISwarm
 *
 *    and exposes:
 *
 *        swarmConstruct
 *
 *    as an AI-domain construct.
 *
 *
 * 3. AGENTS
 *
 *    grammar/ai/agents.g4 remains the sole owner of individual agent syntax.
 *
 *    This file references agents through expressions.
 *
 *
 * 4. ACTORS
 *
 *    grammar/concurrency/actors.g4 remains the sole owner of actor syntax.
 *
 *
 * 5. PLANNING
 *
 *    grammar/ai/planning.g4 remains the sole owner of planning syntax.
 *
 *
 * 6. RESOURCES
 *
 *    grammar/resources/ remains the sole authority for resource negotiation.
 *
 *
 * 7. CAPABILITIES
 *
 *    grammar/core/capabilities.g4 and grammar/resources/capabilities.g4
 *    remain authoritative for capability identity and resource capability
 *    semantics.
 *
 *
 * 8. EFFECTS
 *
 *    grammar/effects/ remains authoritative for effect semantics.
 *
 *
 * 9. POLICIES
 *
 *    grammar/policies/ and grammar/security/ remain authoritative for policy
 *    semantics and enforcement.
 *
 *
 * 10. DISTRIBUTED EXECUTION
 *
 *     grammar/distributed/ owns distributed realization semantics.
 *
 *
 * 11. QUANTUM
 *
 *     quantum source remains owned by grammar/quantum/.
 *
 *     Quantum semantics lower through:
 *
 *         quantum::ir
 *
 *
 * 12. HDL
 *
 *     HDL source remains owned by grammar/hdl/.
 *
 *
 * 13. AST
 *
 *     Existing domain-neutral AST receives the parsed structure.
 *
 *
 * 14. SEMANTIC MODEL
 *
 *     Semantic analysis creates collective intent from the AST.
 *
 *
 * 15. IR
 *
 *     No swarm-specific IR is introduced.
 *
 *     Collective semantics lower through existing:
 *
 *         classical
 *         concurrency
 *         distributed
 *         quantum::ir
 *         HDL/hardware
 *
 *     boundaries as appropriate.
 *
 *
 * 16. RUNTIME
 *
 *     Runtime swarm orchestration may consume the semantic model, but runtime
 *     APIs are not called by this grammar.
 *
 *
 * 17. RUST
 *
 *     The grammar contains no Rust implementation.
 *
 *     Rust 1.97+ compatibility is a downstream build requirement.
 *
 *     Generated Rust must remain safe Rust.
 *
 *     No unsafe implementation is required or permitted by this grammar.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It has one canonical public swarm entry point.
 *
 *     [x] It does not redefine agent syntax.
 *
 *     [x] It does not redefine actor syntax.
 *
 *     [x] It does not redefine planning syntax.
 *
 *     [x] It does not define a second expression system.
 *
 *     [x] It does not define a second type system.
 *
 *     [x] It does not define a swarm-specific IR.
 *
 *     [x] It introduces no hardware limits.
 *
 *     [x] It introduces no fixed swarm/member limits.
 *
 *     [x] It introduces no physical placement assumptions.
 *
 *     [x] It introduces no vendor-specific syntax.
 *
 *     [x] It uses the canonical lexer vocabulary.
 *
 *     [x] It contains no embedded actions.
 *
 *     [x] It contains no semantic predicates.
 *
 *     [x] It requires no unsafe Rust.
 *
 *     [x] It preserves source-level symbolic membership.
 *
 *     [x] It provides an open-world extension mechanism.
 *
 *     [x] It is compatible with AI, concurrency, distributed, quantum, HDL,
 *         hardware, resources, effects, policies and provenance boundaries.
 *
 *     [ ] It is imported by grammar/ai/ai.g4.
 *
 *     [ ] AI conformance tests exist under grammar/tests/ai/swarm/.
 *
 *     [ ] AST integration tests pass.
 *
 *     [ ] Semantic integration tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] ANTLR generation succeeds.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * A swarm is a source-level collective intent.
 *
 * It is NOT:
 *
 *     a hardware topology;
 *     a fixed collection of CPUs;
 *     a fixed collection of GPUs;
 *     a fixed collection of QPUs;
 *     a fixed collection of nodes;
 *     a fixed process group;
 *     a fixed thread group;
 *     a fixed network;
 *     a fixed device set.
 *
 * The source describes:
 *
 *     WHO/WHAT participates
 *     WHAT the collective intends
 *     WHAT it requires
 *     WHAT it prefers
 *     WHAT it is allowed to do
 *
 * The compiler and runtime determine:
 *
 *     WHERE
 *     HOW
 *     WHEN
 *     WITH WHICH AVAILABLE RESOURCES
 *
 * the collective computation is realized.
 *
 * This separation is essential for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */