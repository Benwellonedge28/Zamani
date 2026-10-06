/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/multi-agent.g4
 *
 * GRAMMAR
 * -------
 * MultiAgent
 *
 * STATUS
 * ------
 * PRODUCTION SOURCE-GRAMMAR COMPOSITION BOUNDARY
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 * No embedded Rust
 * No unsafe Rust requirement
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar is the canonical parser-level composition boundary for
 * multi-agent computation.
 *
 * IMPORTANT:
 *
 * This file does NOT define another agent language.
 *
 * Individual agent syntax remains exclusively owned by:
 *
 *     grammar/ai/agents.g4
 *
 * Actor syntax remains exclusively owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * This grammar composes those existing constructs and provides parser-level
 * boundaries for:
 *
 *     - multiple agents;
 *     - agent/actor participation;
 *     - groups;
 *     - compositions;
 *     - delegation;
 *     - coordination;
 *     - communication;
 *     - supervision;
 *     - participant analysis;
 *     - nested composition;
 *     - source-order preservation;
 *     - semantic relationship analysis.
 *
 * The semantic layer determines the actual meaning of these constructs.
 *
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * Multi-agent computation is a semantic composition of ordinary Zamani
 * computation participants.
 *
 * It is NOT:
 *
 *     a second actor language;
 *     a second distributed language;
 *     a second networking language;
 *     a second concurrency language;
 *     a second AI language;
 *     a second IR;
 *     a hardware-placement language.
 *
 *
 * ============================================================================
 * PIPELINE
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     lexer
 *        |
 *        v
 *     parser
 *        |
 *        v
 *     domain-neutral AST
 *        |
 *        v
 *     structural validation
 *        |
 *        +-----------------------+
 *        |                       |
 *        v                       v
 *     agent semantics       actor semantics
 *        |                       |
 *        +-----------+-----------+
 *                    |
 *                    v
 *          multi-agent semantic model
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *       effects   resources capabilities
 *          |         |         |
 *          +---------+---------+
 *                    |
 *                    v
 *                 policies
 *                    |
 *                    v
 *                provenance
 *                    |
 *                    v
 *             canonical semantic IR
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *      classical  quantum::ir HDL/hardware
 *                    |
 *                    v
 *                optimization
 *                    |
 *                scheduling
 *                    |
 *                  routing
 *                    |
 *                resilience
 *                    |
 *                   ZQN
 *                    |
 *                   HAL
 *                    |
 *             target realization
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns parser-level multi-agent composition boundaries:
 *
 *     multiAgentConstruct
 *     multiAgentAgentConstruct
 *     multiAgentActorConstruct
 *     multiAgentGroup
 *     multiAgentComposition
 *     multiAgentBody
 *     multiAgentMember
 *     multiAgentMembers
 *     multiAgentDelegation
 *     multiAgentCoordination
 *     multiAgentCommunication
 *     multiAgentSupervision
 *     multiAgentRelationship
 *     multiAgentRelationships
 *     multiAgentParticipant
 *     multiAgentParticipants
 *     multiAgentUnit
 *     multiAgentUnits
 *     multiAgentReference
 *     multiAgentExpression
 *     multiAgentType
 *     multiAgentArgumentList
 *     multiAgentOrderedMember
 *     nestedMultiAgentConstruct
 *     multiAgentDomainConstruct
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
 *     lexical tokens
 *     expressions
 *     expression precedence
 *     types
 *     statements
 *     declarations
 *     agent syntax
 *     actor syntax
 *     channels
 *     futures
 *     tasks
 *     asynchronous execution
 *     networking
 *     distributed topology
 *     resource discovery
 *     capability discovery
 *     policy enforcement
 *     authorization
 *     model execution
 *     reasoning
 *     learning
 *     adaptation
 *     uncertainty
 *     provenance semantics
 *     quantum syntax
 *     quantum operations
 *     QEC
 *     HDL syntax
 *     hardware topology
 *     scheduling
 *     routing
 *     placement
 *     classical IR
 *     agent IR
 *     multi-agent IR
 *     quantum::ir
 *     ZQN
 *     HAL
 *     runtime execution
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/ai/agents.g4
 *     grammar/concurrency/actors.g4
 *
 * Transitive universal dependencies are provided by those canonical grammars.
 *
 *
 * EXPORTS:
 *
 *     multiAgentConstruct
 *     multiAgentAgentConstruct
 *     multiAgentActorConstruct
 *     multiAgentGroup
 *     multiAgentComposition
 *     multiAgentBody
 *     multiAgentMember
 *     multiAgentMembers
 *     multiAgentDelegation
 *     multiAgentCoordination
 *     multiAgentCommunication
 *     multiAgentSupervision
 *     multiAgentRelationship
 *     multiAgentRelationships
 *     multiAgentParticipant
 *     multiAgentParticipants
 *     multiAgentUnit
 *     multiAgentUnits
 *     multiAgentReference
 *     multiAgentExpression
 *     multiAgentType
 *     multiAgentArgumentList
 *     multiAgentOrderedMember
 *     nestedMultiAgentConstruct
 *     multiAgentDomainConstruct
 *
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *
 * Future semantic/conformance tooling may consume the secondary boundaries.
 *
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral Zamani frontend AST.
 *
 *
 * SEMANTIC_OWNER:
 *
 *     AI semantic layer
 *     concurrency semantic layer
 *     distributed semantic layer
 *     policy/resource/effect/provenance layers
 *
 *
 * IR_OWNER:
 *
 *     Existing canonical semantic IR/domain IR pipeline.
 *
 *     No multi-agent-specific IR is created by this grammar.
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/
 *     grammar/tests/concurrency/
 *     grammar/tests/distributed/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/spec/policies.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *     grammar/spec/provenance.md
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Agent syntax:
 *
 *     grammar/ai/agents.g4
 *
 * Actor syntax:
 *
 *     grammar/concurrency/actors.g4
 *
 * Multi-agent composition:
 *
 *     THIS FILE
 *
 * Concurrency aggregation:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * Distributed execution:
 *
 *     grammar/distributed/
 *
 * Networking:
 *
 *     grammar/networking/
 *
 * No file may introduce a competing agent or actor declaration syntax.
 *
 *
 * ============================================================================
 * CRITICAL CORRECTION
 * ============================================================================
 *
 * The old design attempted to make constructs such as:
 *
 *     @group Research { ... }
 *
 * and:
 *
 *     @composition { ... }
 *
 * special multi-agent syntax.
 *
 * That is incorrect because `agents.g4` already structurally owns:
 *
 *     @ identifier identifier ...
 *
 * and:
 *
 *     @ identifier ...
 *
 * Therefore such constructs can already be parsed by `agentConstruct`.
 *
 * Creating another parser alternative for the same syntax would introduce:
 *
 *     duplicate ownership;
 *     ambiguity;
 *     unstable parse-tree expectations;
 *     unnecessary semantic adapters;
 *     future compatibility problems.
 *
 * This production grammar therefore treats agent syntax as authoritative and
 * exposes multi-agent semantics through composition boundaries instead.
 *
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * No permanent lexer keyword is introduced for:
 *
 *     group
 *     delegate
 *     coordinate
 *     communicate
 *     supervise
 *     consensus
 *     negotiate
 *     swarm
 *     federation
 *     collaboration
 *     planner
 *     worker
 *     coordinator
 *     leader
 *     participant
 *
 * These are semantic concepts.
 *
 * The existing open-world annotation mechanism from `agents.g4` remains
 * authoritative.
 *
 * This allows future coordination strategies, protocols and organizational
 * models to be introduced through semantic registries, libraries, dialects,
 * capabilities and policies without changing the universal parser merely
 * because a new concept appears.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes no universal upper bound on:
 *
 *     agents
 *     actors
 *     groups
 *     relationships
 *     messages
 *     participants
 *     tasks
 *     goals
 *     delegation chains
 *     coordination relationships
 *     nesting depth
 *     source size
 *     resource quantities
 *     nodes
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     accelerators
 *     memory
 *     network size
 *
 * Repetition uses ordinary grammar constructs such as:
 *
 *     *
 *     +
 *
 * rather than fixed cardinalities.
 *
 * Physical limits are implementation/runtime constraints, never language
 * ceilings.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A multi-agent program expresses portable computational intent.
 *
 * It does not encode:
 *
 *     a fixed number of machines;
 *     a fixed number of agents;
 *     a fixed number of processes;
 *     a fixed number of threads;
 *     a fixed number of network nodes;
 *     a fixed actor placement;
 *     a fixed accelerator;
 *     a fixed QPU;
 *     a fixed topology.
 *
 * Resource and capability requirements are resolved downstream.
 *
 * Therefore the same source structure can be considered for:
 *
 *     tiny embedded execution;
 *     single-process execution;
 *     multicore execution;
 *     GPU-backed execution;
 *     accelerator execution;
 *     FPGA/ASIC realization;
 *     quantum-assisted execution;
 *     simulator execution;
 *     HPC execution;
 *     cluster execution;
 *     distributed execution;
 *     cloud execution;
 *     future execution substrates.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser contexts are converted by the existing frontend into the
 * domain-neutral AST.
 *
 * The AST must preserve, where present:
 *
 *     source span
 *     participant order
 *     participant identity/reference
 *     agent/actor origin
 *     composition nesting
 *     relationship kind
 *     relationship target
 *     relationship arguments
 *     expression structure
 *     type structure
 *
 * The AST MUST NOT introduce:
 *
 *     PhysicalAgent
 *     AgentCPU
 *     AgentGPU
 *     AgentQPU
 *     AgentNode
 *     AgentThread
 *     AgentDevice
 *     PhysicalMailbox
 *     PhysicalTopology
 *
 * or equivalent target-specific nodes.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether participants resolve;
 *     - whether references are valid;
 *     - whether a relationship is legal;
 *     - whether delegation is authorized;
 *     - whether coordination is valid;
 *     - whether communication is permitted;
 *     - whether supervision is valid;
 *     - whether cycles are legal;
 *     - whether ordering constraints are satisfied;
 *     - whether policies permit the operation;
 *     - whether required capabilities exist;
 *     - whether resource requirements are satisfiable;
 *     - whether effects are permitted;
 *     - whether provenance requirements are satisfied;
 *     - whether execution is deterministic where required;
 *     - whether distributed realization is feasible.
 *
 * None of those decisions belong in this parser.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Types remain owned by the canonical type grammar.
 *
 * `multiAgentType` is only an integration alias for `typeExpression`.
 *
 * No second:
 *
 *     AgentType
 *     ActorType
 *     MultiAgentType
 *
 * hierarchy is created here.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Multi-agent constructs may semantically carry effects such as:
 *
 *     concurrency
 *     distributed
 *     network
 *     mutation
 *     communication
 *     foreign
 *     native
 *     randomness
 *     learning
 *     adaptation
 *     reflection
 *     quantum
 *     simulation
 *
 * Effect classification belongs to the existing effects subsystem.
 *
 * This grammar does not invent a second effect vocabulary.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements remain open-world.
 *
 * Examples of semantic capabilities may include:
 *
 *     communication
 *     distributed.compute
 *     actor.execute
 *     quantum.compute
 *     tensor.compute
 *     model.inference
 *
 * No fixed capability list is encoded here.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Multi-agent computation may require resources, but this grammar does not
 * select or limit resources.
 *
 * Requirements are represented through the existing resource model.
 *
 * Examples:
 *
 *     requires capability("distributed.compute");
 *     requires memory >= required_memory;
 *     requires topology(required_topology);
 *
 * remain semantic/resource constructs.
 *
 * No:
 *
 *     MAX_AGENTS
 *     MAX_NODES
 *     MAX_MESSAGES
 *     MAX_THREADS
 *
 * or equivalent universal limit is permitted.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Multi-agent operations may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     evidence
 *
 * Contract syntax remains owned by the validation subsystem.
 *
 * This grammar only provides composition boundaries.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Multi-agent computation may be constrained by policies controlling:
 *
 *     communication
 *     delegation
 *     resource usage
 *     capabilities
 *     security
 *     adaptation
 *     execution
 *     deployment
 *     provenance
 *
 * Policy syntax and enforcement remain outside this grammar.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Semantic analysis may record:
 *
 *     participant origin
 *     delegation reason
 *     coordination decision
 *     communication decision
 *     policy decision
 *     resource decision
 *     transformation
 *     generated artifact
 *     verification
 *
 * Provenance semantics remain owned by the provenance subsystem.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * This grammar contains no quantum operation syntax.
 *
 * An agent may semantically invoke quantum computation through ordinary
 * Zamani expressions/statements.
 *
 * The quantum path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
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
 *     resilience/QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * No physical qubit, gate catalog, calibration value, QPU identity or topology
 * is encoded here.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Multi-agent constructs may coordinate hardware-oriented computation.
 *
 * Hardware realization remains owned by:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *
 * This grammar does not define:
 *
 *     wires
 *     fixed-width buses
 *     registers
 *     physical devices
 *     board identifiers
 *     FPGA resources
 *     ASIC cells
 *     clock topology
 *     physical placement
 *
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * Backends consume semantic information after:
 *
 *     type checking
 *     effect checking
 *     capability resolution
 *     resource analysis
 *     policy analysis
 *     provenance
 *     canonical semantic lowering
 *
 * No backend is selected here.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     selected grammar
 *     lexer vocabulary
 *     parser rules
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     runtime state
 *     network state
 *     wall clock
 *     random values
 *     resource availability
 *     environment variables
 *
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime execution.
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * The grammar itself introduces no unsafe requirement.
 *
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar MultiAgent;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Agents is the source syntax authority for agents.
 *
 * Actors is the source syntax authority for actors.
 *
 * Do not copy their rules into this grammar.
 * ============================================================================
 */

import
    Agents,
    Actors
;


/*
 * ============================================================================
 * 1. PUBLIC MULTI-AGENT CONSTRUCT
 * ============================================================================
 *
 * This is the only rule that `ai.g4` should normally consume.
 *
 * It deliberately does not add another spelling of agent syntax.
 *
 * An individual agent is included because a single participant is a valid
 * member of a larger semantic composition and because the same parser boundary
 * is useful to tooling.
 *
 * Multi-agent cardinality is determined semantically by composition analysis.
 * ============================================================================
 */

multiAgentConstruct
    : multiAgentAgentConstruct
    | multiAgentActorConstruct
    ;


/*
 * ============================================================================
 * 2. AGENT CONSTRUCT ADAPTER
 * ============================================================================
 *
 * `agentConstruct` remains exclusively owned by Agents.
 * ============================================================================
 */

multiAgentAgentConstruct
    : agentConstruct
    ;


/*
 * ============================================================================
 * 3. ACTOR CONSTRUCT ADAPTER
 * ============================================================================
 *
 * `actorConstruct` remains exclusively owned by Actors.
 * ============================================================================
 */

multiAgentActorConstruct
    : actorConstruct
    ;


/*
 * ============================================================================
 * 4. MULTI-AGENT BODY
 * ============================================================================
 *
 * This is a semantic composition boundary, not a new top-level declaration
 * syntax.
 *
 * It represents an ordered collection of multi-agent members.
 *
 * The surrounding owner determines whether such a body is legal.
 * ============================================================================
 */

multiAgentBody
    : LBRACE
      multiAgentMembers
      RBRACE
    ;


/*
 * ============================================================================
 * 5. MULTI-AGENT MEMBER
 * ============================================================================
 *
 * A member may be:
 *
 *     agent
 *     actor
 *     nested composition
 *     ordinary statement
 *
 * Ordinary statements remain owned by Statements.
 * ============================================================================
 */

multiAgentMember
    : multiAgentConstruct
    | nestedMultiAgentConstruct
    | statement
    ;


multiAgentMembers
    : multiAgentMember*
    ;


/*
 * ============================================================================
 * 6. MULTI-AGENT COMPOSITION
 * ============================================================================
 *
 * This rule intentionally has NO new keyword.
 *
 * It is a parser-level semantic boundary for a body containing multiple
 * participants.
 *
 * The semantic layer determines whether the body represents:
 *
 *     collaboration
 *     delegation
 *     coordination
 *     supervision
 *     orchestration
 *     another registered composition
 *
 * ============================================================================
 */

multiAgentComposition
    : multiAgentBody
    ;


/*
 * ============================================================================
 * 7. ARGUMENT LIST
 * ============================================================================
 *
 * Expressions remain owned by Expressions.
 *
 * No second expression language is introduced.
 * ============================================================================
 */

multiAgentArgumentList
    : expression
      (
          COMMA
          expression
      )*
    ;


/*
 * ============================================================================
 * 8. REFERENCE
 * ============================================================================
 *
 * A reference is semantically resolved.
 *
 * The parser deliberately does not decide whether an expression refers to:
 *
 *     an agent
 *     an actor
 *     a group
 *     a task
 *     a service
 *     a model
 *     another participant
 * ============================================================================
 */

multiAgentReference
    : expression
    ;


/*
 * ============================================================================
 * 9. EXPRESSION BRIDGE
 * ============================================================================
 */

multiAgentExpression
    : expression
    ;


/*
 * ============================================================================
 * 10. TYPE BRIDGE
 * ============================================================================
 */

multiAgentType
    : typeExpression
    ;


/*
 * ============================================================================
 * 11. DELEGATION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The syntax is intentionally delegated to the existing annotation-based
 * agent construct.
 *
 * This rule is a semantic parser boundary for tools that need to classify a
 * construct as delegation.
 *
 * It does NOT reserve the identifier `delegate`.
 *
 * It does NOT decide whether an annotation means delegation.
 *
 * Semantic analysis performs that classification.
 *
 * Structurally, it accepts the canonical agent construct.
 * ============================================================================
 */

multiAgentDelegation
    : agentConstruct
    ;


/*
 * ============================================================================
 * 12. COORDINATION
 * ============================================================================
 *
 * Coordination is similarly an open semantic concept.
 *
 * No coordination algorithm is encoded here.
 * ============================================================================
 */

multiAgentCoordination
    : agentConstruct
    ;


/*
 * ============================================================================
 * 13. COMMUNICATION
 * ============================================================================
 *
 * Communication syntax remains owned by the existing agent/concurrency/
 * networking systems.
 *
 * This boundary allows semantic tooling to classify an existing construct
 * without creating another message grammar.
 * ============================================================================
 */

multiAgentCommunication
    : agentConstruct
    | actorSendExpression
    | actorAskExpression
    | actorForwardExpression
    ;


/*
 * ============================================================================
 * 14. SUPERVISION
 * ============================================================================
 *
 * Actor supervision remains owned by Actors.
 *
 * Agent-oriented supervision may use the canonical annotation structure.
 * ============================================================================
 */

multiAgentSupervision
    : actorSupervisionConstruct
    | agentConstruct
    ;


/*
 * ============================================================================
 * 15. RELATIONSHIP
 * ============================================================================
 *
 * These are semantic classification boundaries.
 *
 * They deliberately do not introduce new syntax.
 * ============================================================================
 */

multiAgentRelationship
    : multiAgentDelegation
    | multiAgentCoordination
    | multiAgentCommunication
    | multiAgentSupervision
    ;


multiAgentRelationships
    : multiAgentRelationship*
    ;


/*
 * ============================================================================
 * 16. PARTICIPANT
 * ============================================================================
 *
 * A participant is either:
 *
 *     an agent;
 *     an actor;
 *     a resolvable reference.
 *
 * The semantic layer determines participant identity and validity.
 * ============================================================================
 */

multiAgentParticipant
    : multiAgentAgentConstruct
    | multiAgentActorConstruct
    | multiAgentReference
    ;


multiAgentParticipants
    : multiAgentParticipant*
    ;


/*
 * ============================================================================
 * 17. SEMANTIC COMPOSITION UNIT
 * ============================================================================
 *
 * This is intentionally broader than `multiAgentConstruct`.
 *
 * It is useful for semantic tooling and conformance analysis.
 * ============================================================================
 */

multiAgentUnit
    : multiAgentConstruct
    | multiAgentRelationship
    | multiAgentComposition
    | statement
    ;


multiAgentUnits
    : multiAgentUnit*
    ;


/*
 * ============================================================================
 * 18. SOURCE-ORDER PRESERVATION
 * ============================================================================
 *
 * The AST must preserve source order.
 *
 * This rule exists as a stable parser boundary for consumers that explicitly
 * require ordered member analysis.
 * ============================================================================
 */

multiAgentOrderedMember
    : multiAgentMember
    ;


/*
 * ============================================================================
 * 19. NESTED COMPOSITION
 * ============================================================================
 *
 * Recursive nesting is intentional.
 *
 * No universal nesting depth is encoded here.
 *
 * Finite parser-stack/resource exhaustion is an implementation concern and
 * must be diagnosed rather than converted into a language-level limit.
 * ============================================================================
 */

nestedMultiAgentConstruct
    : multiAgentConstruct
    | multiAgentComposition
    ;


/*
 * ============================================================================
 * 20. TARGET-INDEPENDENT CONSTRUCT
 * ============================================================================
 *
 * This adapter explicitly communicates that multi-agent syntax is independent
 * of physical realization.
 * ============================================================================
 */

multiAgentTargetIndependentConstruct
    : multiAgentConstruct
    | multiAgentComposition
    | multiAgentRelationship
    ;


/*
 * ============================================================================
 * 21. FINAL DOMAIN AGGREGATION
 * ============================================================================
 *
 * Conformance tooling may consume this rule.
 *
 * The universal AI composition grammar should normally consume only:
 *
 *     multiAgentConstruct
 *
 * ============================================================================
 */

multiAgentDomainConstruct
    : multiAgentConstruct
    | multiAgentComposition
    | multiAgentRelationship
    | multiAgentSupervision
    ;


/*
 * ============================================================================
 * FEATURE CONTRACT — COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] Agents owns agent syntax.
 *
 * [x] Actors owns actor syntax.
 *
 * [x] No duplicate agent declaration syntax exists here.
 *
 * [x] No duplicate actor declaration syntax exists here.
 *
 * [x] No new mandatory multi-agent lexer keyword exists.
 *
 * [x] No fixed participant count exists.
 *
 * [x] No fixed agent count exists.
 *
 * [x] No fixed group count exists.
 *
 * [x] No fixed relationship count exists.
 *
 * [x] No hardware capacity is encoded.
 *
 * [x] No target is selected by parsing.
 *
 * [x] No physical topology is encoded.
 *
 * [x] No multi-agent-specific IR exists.
 *
 * [x] General expressions remain owned by Expressions.
 *
 * [x] General types remain owned by Types.
 *
 * [x] General statements remain owned by Statements.
 *
 * [x] Actor communication remains owned by Actors/Concurrency.
 *
 * [x] Distributed realization remains downstream.
 *
 * [x] Resource analysis remains downstream.
 *
 * [x] Capability analysis remains downstream.
 *
 * [x] Effect analysis remains downstream.
 *
 * [x] Policy analysis remains downstream.
 *
 * [x] Provenance remains downstream.
 *
 * [x] Quantum lowering remains downstream through quantum::ir.
 *
 * [x] HDL/hardware lowering remains downstream.
 *
 * [x] No embedded Rust exists.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Positive tests exist.
 *
 * [x] Negative tests exist.
 *
 * [x] Boundary tests exist.
 *
 * [x] Scalability tests exist.
 *
 * [x] Determinism tests exist.
 *
 * [x] Cross-domain tests exist.
 *
 * [x] Compatibility tests exist.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * Individual agent:
 *
 *     @agent Worker {
 *         ...
 *     }
 *
 * Actor:
 *
 *     actor Worker {
 *         ...
 *     }
 *
 * Agent composition body:
 *
 *     {
 *         @agent A {
 *             ...
 *         }
 *
 *         @agent B {
 *             ...
 *         }
 *     }
 *
 * Nested composition:
 *
 *     {
 *         {
 *             @agent A {
 *                 ...
 *             }
 *
 *             @agent B {
 *                 ...
 *             }
 *         }
 *     }
 *
 * Actor communication:
 *
 *     actor A {
 *         receive work(value: Value) {
 *             ...
 *         }
 *     }
 *
 * Relationship annotations:
 *
 *     @delegate worker(task);
 *
 *     @coordinate group;
 *
 *     @message(recipient, payload);
 *
 *     @supervise worker;
 *
 *
 * NEGATIVE
 * --------
 *
 * Must reject malformed:
 *
 *     incomplete annotation
 *     incomplete actor declaration
 *     incomplete group body
 *     malformed argument list
 *     malformed relationship expression
 *     malformed nested composition
 *
 *
 * BOUNDARY
 * --------
 *
 * Test combinations of:
 *
 *     agent + actor
 *     agent + quantum expression
 *     agent + classical computation
 *     agent + distributed operation
 *     agent + networking operation
 *     agent + resource requirement
 *     agent + capability requirement
 *     agent + contract
 *     agent + policy
 *     agent + provenance
 *     agent + learning
 *     agent + adaptation
 *
 *
 * SCALABILITY
 * ----------
 *
 * Test symbolically large source structures without changing the grammar:
 *
 *     many agents
 *     many actors
 *     many relationships
 *     deeply nested compositions
 *     large participant collections
 *     large expression arguments
 *
 * The test suite must NOT define a universal maximum merely to satisfy a
 * grammar test.
 *
 * Resource-exhaustion testing belongs to the parser/runtime safety tests.
 *
 *
 * DETERMINISM
 * -----------
 *
 * The same source and grammar configuration must produce the same parse tree.
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     resource availability
 *     runtime state
 *     network state
 *     randomness
 *     wall-clock time
 *
 *
 * CROSS-DOMAIN
 * ------------
 *
 * At minimum test:
 *
 *     AI + concurrency
 *     AI + distributed
 *     AI + quantum
 *     AI + classical
 *     AI + resources
 *     AI + effects
 *     AI + contracts
 *     AI + policies
 *     AI + provenance
 *
 *
 * COMPATIBILITY
 * ------------
 *
 * Existing valid `agentConstruct` input must remain valid.
 *
 * Existing valid `actorConstruct` input must remain valid.
 *
 * No existing agent or actor syntax may acquire a different parse meaning
 * merely because this grammar is imported.
 *
 * ============================================================================
 */