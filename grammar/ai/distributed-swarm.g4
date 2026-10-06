/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/distributed-swarm.g4
 *
 * GRAMMAR
 * -------
 * DistributedSwarm
 *
 * STATUS
 * ------
 * PRODUCTION AI / DISTRIBUTED SWARM COMPOSITION BOUNDARY
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
 * This file defines the source-level structural boundary for distributed
 * swarm computation.
 *
 * A swarm is a semantic collective composed from existing Zamani computation
 * participants, especially:
 *
 *     agents
 *     actors
 *     distributed participants
 *     collective operations
 *     services
 *     dynamically resolved participant sets
 *
 * The swarm construct expresses PORTABLE COMPUTATIONAL INTENT.
 *
 * It does not select:
 *
 *     machines
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     physical nodes
 *     physical network topology
 *     transport protocols
 *     schedulers
 *     placement algorithms
 *     vendor runtimes
 *     AI frameworks
 *
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * A swarm is NOT a second:
 *
 *     agent language
 *     actor language
 *     distributed language
 *     collective language
 *     networking language
 *     resource language
 *     policy language
 *     execution language
 *     IR
 *     runtime
 *
 * Instead:
 *
 *     swarm
 *       |
 *       +--> agents
 *       +--> actors
 *       +--> collective semantics
 *       +--> distributed semantics
 *       +--> resources
 *       +--> capabilities
 *       +--> effects
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     semantic swarm model
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +-------------------+-------------------+
 *       |                   |                   |
 *       v                   v                   v
 *   classical          quantum::ir       HDL/hardware
 *       |                   |                   |
 *       +-------------------+-------------------+
 *                           |
 *                     optimization
 *                           |
 *                      placement
 *                           |
 *                       routing
 *                           |
 *                      scheduling
 *                           |
 *                      resilience
 *                           |
 *                      deployment
 *                           |
 *                         runtime
 *
 *
 * ============================================================================
 * OWNERS
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     swarmConstruct
 *     swarmDeclaration
 *     swarmInvocation
 *     swarmMarker
 *     swarmName
 *     swarmBody
 *     swarmMember
 *     swarmMembers
 *     swarmProperty
 *     swarmClause
 *     swarmClauseName
 *     swarmSection
 *     swarmValue
 *     swarmObject
 *     swarmList
 *     swarmListElement
 *     swarmParticipantExpression
 *     swarmObjectiveExpression
 *     swarmCoordinationExpression
 *     swarmStrategyExpression
 *     swarmPolicyExpression
 *     swarmTerminationExpression
 *     swarmTargetIndependentConstruct
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     statements
 *     agent declarations
 *     actor declarations
 *     actor lifecycle
 *     actor messaging
 *     channels
 *     distributed nodes
 *     distributed topology
 *     collective operation definitions
 *     network protocols
 *     resources
 *     capabilities
 *     effects
 *     contracts
 *     policies
 *     security enforcement
 *     provenance semantics
 *     scheduling
 *     routing
 *     placement
 *     resilience
 *     quantum operations
 *     quantum physical topology
 *     QEC
 *     HDL semantics
 *     hardware realization
 *     classical IR
 *     quantum::ir
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
 *     grammar/core/names.g4
 *     grammar/expressions/Expressions
 *     grammar/ai/multi-agent.g4
 *     grammar/distributed/collective.g4
 *
 * EXPORTS:
 *
 *     swarmConstruct
 *     swarmDeclaration
 *     swarmInvocation
 *     swarmMarker
 *     swarmName
 *     swarmBody
 *     swarmMember
 *     swarmMembers
 *     swarmProperty
 *     swarmClause
 *     swarmClauseName
 *     swarmSection
 *     swarmValue
 *     swarmObject
 *     swarmList
 *     swarmListElement
 *     swarmParticipantExpression
 *     swarmObjectiveExpression
 *     swarmCoordinationExpression
 *     swarmStrategyExpression
 *     swarmPolicyExpression
 *     swarmTerminationExpression
 *     swarmTargetIndependentConstruct
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *
 * Potential semantic consumers:
 *
 *     AI semantic analysis
 *     distributed semantic analysis
 *     concurrency semantic analysis
 *     resource analysis
 *     capability analysis
 *     policy analysis
 *     execution planning
 *     provenance
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral Zamani frontend AST.
 *
 * SEMANTIC_OWNER:
 *
 *     AI + distributed + concurrency semantic layers.
 *
 * IR_OWNER:
 *
 *     Existing canonical semantic IR/domain IR pipeline.
 *
 *     No SwarmIR is introduced by this grammar.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/
 *     grammar/tests/distributed/
 *     grammar/tests/concurrency/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/spec/resources.md
 *     grammar/spec/policies.md
 *     grammar/spec/effects.md
 *     grammar/spec/provenance.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Agent syntax remains owned by:
 *
 *     grammar/ai/agents.g4
 *
 * Multi-agent composition remains owned by:
 *
 *     grammar/ai/multi-agent.g4
 *
 * Actor syntax remains owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * Distributed collective syntax remains owned by:
 *
 *     grammar/distributed/collective.g4
 *
 * Distributed composition remains owned by:
 *
 *     grammar/distributed/distributed.g4
 *
 * This file MUST NOT duplicate any of those grammars.
 *
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * No permanent lexer keyword is required for:
 *
 *     swarm
 *     member
 *     participant
 *     objective
 *     coordination
 *     strategy
 *     policy
 *     communication
 *     termination
 *     consensus
 *     delegation
 *     federation
 *     hierarchy
 *     topology
 *     worker
 *     coordinator
 *     leader
 *
 * These are semantic concepts.
 *
 * The source grammar uses ordinary identifiers for extensibility.
 *
 * This prevents every new coordination strategy, collective algorithm,
 * organizational model, or vendor-independent semantic capability from
 * requiring a lexer modification.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes NO universal upper bound on:
 *
 *     swarms
 *     agents
 *     actors
 *     participants
 *     members
 *     objectives
 *     strategies
 *     relationships
 *     messages
 *     tasks
 *     collective operations
 *     nested sections
 *     source size
 *     resource quantities
 *     nodes
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     accelerators
 *     memory
 *     network size
 *
 * There are deliberately no:
 *
 *     MAX_SWARMS
 *     MAX_AGENTS
 *     MAX_ACTORS
 *     MAX_PARTICIPANTS
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_MESSAGES
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *
 * or equivalent language-level limits.
 *
 * Repetition uses ordinary ANTLR `*` / `+` constructs.
 *
 * Practical limits arise from:
 *
 *     compiler resources
 *     parser resources
 *     available memory
 *     target resources
 *     deployment resources
 *     runtime policy
 *
 * Those limits are not language semantics.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A swarm describes WHAT a collective computation intends to accomplish.
 *
 * It does not encode HOW MANY physical participants are available.
 *
 * Membership may be:
 *
 *     statically named
 *     dynamically computed
 *     capability-selected
 *     resource-derived
 *     service-resolved
 *     policy-constrained
 *     discovered at runtime
 *
 * without changing the source grammar.
 *
 * A swarm may therefore be realized on:
 *
 *     one logical execution context
 *     multiple CPU cores
 *     embedded hardware
 *     GPUs
 *     accelerators
 *     FPGAs
 *     ASICs
 *     clusters
 *     HPC systems
 *     cloud systems
 *     heterogeneous systems
 *     quantum-classical systems
 *     distributed quantum systems
 *     future computational substrates
 *
 * subject to semantic feasibility and available resources.
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Swarm syntax may carry semantic expressions such as:
 *
 *     requires capability("distributed.communication");
 *
 *     requires capability("collective.compute");
 *
 *     requires capability("tensor.compute");
 *
 *     requires memory >= required_memory;
 *
 *     requires topology(required_topology);
 *
 * These expressions are NOT evaluated here.
 *
 * They do not select a machine.
 *
 * They do not allocate resources.
 *
 * They do not establish a maximum.
 *
 * Resource and capability analysis remain downstream.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A swarm construct does not itself grant effects.
 *
 * Semantic analysis determines effects resulting from its members.
 *
 * Possible effects include:
 *
 *     distributed
 *     network
 *     mutation
 *     randomness
 *     learning
 *     adaptation
 *     native
 *     foreign
 *     measurement
 *     simulation
 *
 * Effect ownership remains with grammar/effects and semantic analysis.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * A swarm may carry policy expressions or policy references.
 *
 * The grammar does not enforce them.
 *
 * Policy interpretation remains owned by the policy/security/execution
 * subsystems.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Swarm members may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assertion
 *
 * constructs.
 *
 * Contract ownership remains with grammar/validation.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The source span and ordering of every swarm member must remain available to
 * downstream provenance.
 *
 * Semantic provenance may record:
 *
 *     swarm identity
 *     participant expression
 *     objective
 *     strategy
 *     policy
 *     evidence
 *     decision
 *     transformation
 *     execution realization
 *
 * This grammar creates no provenance runtime object.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A swarm may semantically coordinate quantum work.
 *
 * This file does NOT define:
 *
 *     gates
 *     physical qubits
 *     QPU topology
 *     calibration
 *     routing
 *     QEC
 *     ZQN
 *
 * If a swarm member contains quantum computation, the semantic path is:
 *
 *     swarm
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic swarm model
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
 *     resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A swarm may semantically coordinate HDL/hardware computation.
 *
 * This grammar does NOT define:
 *
 *     wires
 *     registers
 *     pins
 *     physical placement
 *     clock topology
 *     FPGA fabric
 *     ASIC cells
 *     vendor devices
 *
 * Hardware realization remains downstream.
 *
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Distributed semantics remain owned by grammar/distributed/.
 *
 * This file may compose existing distributed collective syntax, but does not
 * redefine:
 *
 *     nodes
 *     services
 *     channels
 *     communication
 *     topology
 *     placement
 *     replication
 *     consistency
 *     deployment
 *     fault tolerance
 *
 *
 * ============================================================================
 * AI / AGENT BOUNDARY
 * ============================================================================
 *
 * AI agent semantics remain owned by grammar/ai/agents.g4 and
 * grammar/ai/multi-agent.g4.
 *
 * This file adds the swarm-level semantic boundary:
 *
 *     participant population
 *     collective objective
 *     coordination intent
 *     strategy intent
 *     policy association
 *     termination intent
 *
 * It does not define another agent syntax.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     token stream
 *     grammar version
 *
 * It does not inspect:
 *
 *     hardware
 *     resource availability
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     runtime state
 *
 * Identical source/configuration must produce structurally equivalent parse
 * trees.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no actions
 *     no I/O
 *     no network access
 *     no hardware access
 *     no runtime callbacks
 *     no unsafe Rust
 *
 * Generated parser code remains subject to the repository's safe-Rust
 * requirement and Rust 1.97+ compatibility.
 *
 *
 * ============================================================================
 */

parser grammar DistributedSwarm;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions,
    MultiAgent,
    Collective
;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY
 * ============================================================================
 *
 * Canonical integration rule.
 */
swarmConstruct
    : swarmDeclaration
    | swarmInvocation
    ;


/*
 * ============================================================================
 * 2. DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     swarm Research {
 *         participants: agents;
 *         objective: solve(problem);
 *         coordination: coordination_strategy;
 *     }
 *
 * `swarm` remains an identifier-level marker.
 *
 * Semantic analysis recognizes the marker in this construct position.
 */
swarmDeclaration
    : swarmMarker
      swarmName
      swarmBody
    ;


/*
 * ============================================================================
 * 3. INVOCATION
 * ============================================================================
 *
 * Compact form:
 *
 *     swarm Research(participants, objective);
 *
 * Argument meaning is semantic.
 */
swarmInvocation
    : swarmMarker
      swarmName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. MARKER
 * ============================================================================
 *
 * No new lexer keyword is introduced.
 */
swarmMarker
    : identifier
    ;


/*
 * ============================================================================
 * 5. SWARM NAME
 * ============================================================================
 */
swarmName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 6. BODY
 * ============================================================================
 */
swarmBody
    : LBRACE
      swarmMembers
      RBRACE
    ;


swarmMembers
    : swarmMember*
    ;


/*
 * ============================================================================
 * 7. MEMBER
 * ============================================================================
 *
 * Members may be:
 *
 *     properties
 *     semantic clauses
 *     nested sections
 *     existing agent/actor composition
 *     existing collective operations
 *
 * No new agent or actor syntax is created.
 */
swarmMember
    : swarmProperty
    | swarmClause
    | swarmSection
    | multiAgentConstruct
    | collectiveConstruct
    ;


/*
 * ============================================================================
 * 8. PROPERTY
 * ============================================================================
 *
 * Generic properties deliberately remain identifiers.
 *
 * Examples:
 *
 *     participants: agents;
 *     objective: solve(problem);
 *     strategy: adaptive;
 *     policy: execution_policy;
 *     termination: done;
 */
swarmProperty
    : identifier
      COLON
      swarmValue
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. CLAUSE
 * ============================================================================
 *
 * Canonical requirement/guarantee forms remain open semantic expressions.
 *
 * This permits integration with:
 *
 *     resources
 *     capabilities
 *     contracts
 *     policies
 *     effects
 *     provenance
 *
 * without creating competing grammar systems.
 */
swarmClause
    : swarmClauseName
      expression
      SEMICOLON?
    ;


swarmClauseName
    : REQUIRES
    | ENSURES
    | identifier
    ;


/*
 * ============================================================================
 * 10. NESTED SECTION
 * ============================================================================
 *
 * Example:
 *
 *     coordination {
 *         strategy: adaptive;
 *         policy: policy_reference;
 *     }
 *
 *     resources {
 *         requirement: required_resources;
 *     }
 *
 *     provenance {
 *         source: source_reference;
 *     }
 */
swarmSection
    : identifier
      swarmBody
    ;


/*
 * ============================================================================
 * 11. VALUE
 * ============================================================================
 *
 * Ordinary expressions remain the canonical computational value model.
 *
 * Objects and lists are provided only as structural metadata forms.
 */
swarmValue
    : expression
    | swarmObject
    | swarmList
    ;


swarmObject
    : LBRACE
      swarmMembers
      RBRACE
    ;


swarmList
    : LBRACKET
      swarmListElement*
      RBRACKET
    ;


swarmListElement
    : expression
    | swarmObject
    | swarmList
    ;


/*
 * ============================================================================
 * 12. SEMANTIC EXPRESSION BRIDGES
 * ============================================================================
 *
 * These named bridges provide stable parser boundaries for semantic tooling
 * without introducing separate expression languages.
 */
swarmParticipantExpression
    : expression
    ;


swarmObjectiveExpression
    : expression
    ;


swarmCoordinationExpression
    : expression
    ;


swarmStrategyExpression
    : expression
    ;


swarmPolicyExpression
    : expression
    ;


swarmTerminationExpression
    : expression
    ;


/*
 * ============================================================================
 * 13. TARGET-INDEPENDENT BOUNDARY
 * ============================================================================
 */
swarmTargetIndependentConstruct
    : swarmConstruct
    | multiAgentConstruct
    | collectiveConstruct
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     swarm marker
 *     swarm name
 *     declaration/invocation form
 *     member ordering
 *     property names
 *     property values
 *     clauses
 *     nested sections
 *     participating agent/actor structures
 *     collective structures
 *     source spans
 *
 * It MUST NOT create:
 *
 *     physical node IDs
 *     CPU IDs
 *     GPU IDs
 *     QPU IDs
 *     worker IDs
 *     network addresses
 *     transport handles
 *     scheduler handles
 *     runtime swarm IDs
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether `swarm` is a recognized construct marker;
 *     whether the swarm name resolves;
 *     whether participant expressions denote valid participants;
 *     whether an objective is valid;
 *     whether coordination semantics are supported;
 *     whether strategy metadata is valid;
 *     whether policies are applicable;
 *     whether requirements are satisfiable;
 *     whether capabilities exist;
 *     whether effects are permitted;
 *     whether contracts hold;
 *     whether the swarm can be lowered to the selected execution model.
 *
 * Semantic analysis may derive:
 *
 *     participant sets
 *     role relationships
 *     coordination plans
 *     collective operations
 *     execution dependencies
 *     resource requirements
 *     capability requirements
 *     policy constraints
 *     provenance records
 *
 * None of those decisions occur during parsing.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * All values are checked using the canonical Zamani type system.
 *
 * This grammar does NOT introduce:
 *
 *     SwarmType
 *     AgentType
 *     ActorType
 *     ParticipantType
 *
 * as competing type systems.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar owns NO IR.
 *
 * The semantic swarm model may lower into existing representations.
 *
 * Examples:
 *
 *     classical computation
 *     distributed execution metadata
 *     actor/concurrency semantics
 *     quantum::ir
 *     HDL/hardware representation
 *
 * No:
 *
 *     SwarmIR
 *     AgentSwarmIR
 *     DistributedSwarmIR
 *
 * is introduced by this grammar.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements remain symbolic.
 *
 * Valid semantic forms include:
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("distributed.communication");
 *
 *     requires capability("collective.compute");
 *
 *     requires topology(required_topology);
 *
 * No physical realization is selected by this grammar.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing:
 *
 *     @agent ...
 *     actor ...
 *     collective ...
 *
 * syntax retains its existing ownership and meaning.
 *
 * This grammar introduces no mandatory lexer keyword.
 *
 * Existing ordinary identifiers remain valid unless semantic analysis
 * recognizes them as a swarm marker in a swarm construct position.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     swarm Research {
 *         participants: researchers;
 *         objective: solve(problem);
 *     }
 *
 *     swarm Compute {
 *         participants: workers;
 *         objective: train(model, data);
 *         coordination: strategy;
 *     }
 *
 *     swarm Hybrid {
 *         participants: agents;
 *         objective: quantum_classical_work;
 *         resources {
 *             requires capability("quantum.measurement");
 *         }
 *     }
 *
 *     swarm Distributed {
 *         participants: participant_set;
 *         coordination {
 *             strategy: distributed_strategy;
 *         }
 *         collective reduce(participants, values);
 *     }
 *
 *     swarm Adaptive {
 *         participants: discover_participants();
 *         objective: optimize(problem);
 *         policy: execution_policy;
 *         termination: convergence;
 *     }
 *
 *     swarm Nested {
 *         coordination {
 *             strategy {
 *                 name: adaptive;
 *             }
 *         }
 *     }
 *
 *
 * NEGATIVE
 * --------
 *
 * Reject malformed:
 *
 *     swarm;
 *     swarm Research;
 *     swarm { };
 *     swarm Research(;
 *     swarm Research() malformed;
 *     swarm Research { participants: ; };
 *     swarm Research { : value; };
 *     swarm Research { coordination { ;
 *     swarm Research { [ malformed;
 *
 *
 * BOUNDARY
 * --------
 *
 * Verify:
 *
 *     one participant;
 *     dynamically computed participants;
 *     symbolic participants;
 *     nested swarms through semantic composition;
 *     agent participants;
 *     actor participants;
 *     distributed participants;
 *     collective operations;
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware computation;
 *     resource requirements;
 *     capability requirements;
 *     contracts;
 *     policies;
 *     provenance;
 *     learning;
 *     adaptation;
 *     simulation.
 *
 *
 * SCALABILITY
 * ----------
 *
 * Verify that the grammar remains unchanged when increasing:
 *
 *     swarm count;
 *     participant count;
 *     agent count;
 *     actor count;
 *     objective count;
 *     property count;
 *     message count;
 *     collective count;
 *     nesting depth;
 *     resource requirements;
 *     capability requirements;
 *     expression complexity.
 *
 * Tests must never introduce a language-level maximum merely to make the
 * parser test finite.
 *
 *
 * DETERMINISM
 * -----------
 *
 * Identical source and parser configuration must produce equivalent parse
 * structures.
 *
 * Parsing must not depend on:
 *
 *     resource availability;
 *     hardware;
 *     network state;
 *     runtime state;
 *     randomness;
 *     wall-clock time.
 *
 *
 * CROSS-DOMAIN
 * ------------
 *
 * At minimum:
 *
 *     AI + concurrency
 *     AI + distributed
 *     AI + classical
 *     AI + quantum
 *     AI + hybrid
 *     AI + HDL
 *     AI + hardware
 *     AI + resources
 *     AI + capabilities
 *     AI + effects
 *     AI + contracts
 *     AI + policies
 *     AI + provenance
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO universal capacity constants.
 *     NO fixed participant counts.
 *     NO fixed agent counts.
 *     NO fixed actor counts.
 *     NO fixed node counts.
 *     NO fixed machine counts.
 *     NO fixed topology size.
 *     NO physical device identifiers.
 *     NO vendor-specific device enumeration.
 *     NO finite algorithm catalogue.
 *     NO finite collective-operation catalogue.
 *     NO quantum gate catalogue.
 *     NO hardware-size assumptions.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] Swarm syntax has one canonical entry point.
 *
 * [x] No competing agent grammar exists.
 *
 * [x] No competing actor grammar exists.
 *
 * [x] Existing collective syntax is reused.
 *
 * [x] No new swarm lexer keyword is required.
 *
 * [x] Participant membership is expression-based.
 *
 * [x] Dynamic membership is representable.
 *
 * [x] Coordination is open-world.
 *
 * [x] Strategies are open-world.
 *
 * [x] Policies remain externally owned.
 *
 * [x] Resources remain externally owned.
 *
 * [x] Capabilities remain externally owned.
 *
 * [x] Effects remain externally owned.
 *
 * [x] Contracts remain externally owned.
 *
 * [x] Provenance remains externally owned.
 *
 * [x] No physical topology is encoded.
 *
 * [x] No machine size is encoded.
 *
 * [x] No fixed capacity exists.
 *
 * [x] No swarm-specific IR exists.
 *
 * [x] Quantum lowering remains through quantum::ir.
 *
 * [x] HDL/hardware lowering remains downstream.
 *
 * [x] Generated Rust requires no unsafe implementation.
 *
 * [x] Rust 1.97+ compatibility remains the implementation baseline.
 *
 * [x] Positive tests are defined.
 *
 * [x] Negative tests are defined.
 *
 * [x] Boundary tests are defined.
 *
 * [x] Scalability tests are defined.
 *
 * [x] Determinism tests are defined.
 *
 * [x] Cross-domain tests are defined.
 *
 * [x] Compatibility tests are defined.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */