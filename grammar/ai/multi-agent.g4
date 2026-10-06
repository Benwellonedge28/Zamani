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
 * Rust 1.97.1+
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical source-grammar composition boundary for
 * multi-agent computation in Zamani.
 *
 * It does NOT define a second agent language.
 *
 * It composes the existing canonical:
 *
 *     grammar/ai/agents.g4
 *     grammar/concurrency/actors.g4
 *
 * with the universal Zamani expression/type/statement model already owned by
 * those grammars.
 *
 * The purpose of this file is to provide a stable parser-facing boundary for
 * computations involving multiple logical agents, including:
 *
 *     - agent groups;
 *     - agent composition;
 *     - delegation;
 *     - coordination;
 *     - communication intent;
 *     - supervision relationships;
 *     - collaboration;
 *     - distributed agent execution;
 *     - actor-backed realization;
 *     - agent lifecycle composition;
 *     - shared goals;
 *     - task distribution;
 *     - agent references;
 *     - agent collections;
 *     - multi-agent blocks;
 *     - multi-agent invocations;
 *     - nested multi-agent composition.
 *
 * The grammar deliberately leaves:
 *
 *     - agent identity semantics;
 *     - actor identity;
 *     - message transport;
 *     - networking;
 *     - distributed placement;
 *     - scheduling;
 *     - resource allocation;
 *     - capability resolution;
 *     - policy evaluation;
 *     - authorization;
 *     - model execution;
 *     - learning;
 *     - reasoning;
 *     - adaptation;
 *     - quantum execution;
 *     - hardware realization;
 *     - runtime behavior
 *
 * to their existing owning subsystems.
 *
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Multi-agent computation is a semantic composition of existing Zamani
 * constructs.
 *
 * It is NOT:
 *
 *     multi-agent language
 *     actor language
 *     distributed language
 *     networking language
 *     AI-only runtime
 *     agent-specific IR
 *
 * The intended pipeline is:
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
 *        +-----------------------------+
 *        |                             |
 *        v                             v
 *     agent semantics            actor semantics
 *        |                             |
 *        +-------------+---------------+
 *                      |
 *                      v
 *             semantic multi-agent model
 *                      |
 *          +-----------+-----------+
 *          |           |           |
 *          v           v           v
 *       effects    resources   capabilities
 *          |           |           |
 *          +-----------+-----------+
 *                      |
 *                      v
 *                   policies
 *                      |
 *                      v
 *                 provenance
 *                      |
 *                      v
 *               canonical IR
 *                      |
 *          +-----------+-----------+
 *          |           |           |
 *          v           v           v
 *       classical   quantum::ir   HDL/hardware
 *                      |
 *                      v
 *                 optimization
 *                      |
 *                 scheduling
 *                      |
 *                 routing
 *                      |
 *                 resilience
 *                      |
 *                    ZQN
 *                      |
 *                    HAL
 *                      |
 *               target realization
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY the parser-level multi-agent composition boundary.
 *
 * Specifically it owns:
 *
 *     multiAgentConstruct
 *     multiAgentAgentConstruct
 *     multiAgentActorConstruct
 *     multiAgentMember
 *     multiAgentMembers
 *     multiAgentBody
 *     multiAgentGroup
 *     multiAgentGroupMember
 *     multiAgentComposition
 *     multiAgentCompositionMember
 *     multiAgentDelegation
 *     multiAgentCoordination
 *     multiAgentCommunication
 *     multiAgentSupervision
 *     multiAgentReference
 *     multiAgentExpression
 *     multiAgentType
 *
 * These rules are integration adapters and composition boundaries.
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
 *     ordinary statements
 *     agent syntax
 *     actor syntax
 *     channels
 *     futures
 *     asynchronous execution
 *     networking
 *     distributed topology
 *     resources
 *     capabilities
 *     effects
 *     contracts
 *     policies
 *     provenance
 *     model syntax
 *     training syntax
 *     inference syntax
 *     reasoning syntax
 *     learning syntax
 *     adaptation syntax
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     target selection
 *     scheduling
 *     routing
 *     placement
 *     classical IR
 *     agent IR
 *     multi-agent IR
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Agent syntax is owned exclusively by:
 *
 *     grammar/ai/agents.g4
 *
 * Actor syntax is owned exclusively by:
 *
 *     grammar/concurrency/actors.g4
 *
 * General concurrency composition is owned by:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * Channel syntax is owned by:
 *
 *     grammar/concurrency/channels.g4
 *
 * Networking syntax is owned by:
 *
 *     grammar/networking/
 *
 * Distributed syntax is owned by:
 *
 *     grammar/distributed/
 *
 * This file MUST NOT reproduce those grammars.
 *
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * A single agent grammar is sufficient for an individual agent.
 *
 * Multi-agent computation additionally requires a stable semantic boundary
 * through which the compiler can recognize that several agent/actor constructs
 * participate in one higher-level composition.
 *
 * That boundary is important for:
 *
 *     - semantic validation;
 *     - dependency analysis;
 *     - delegation analysis;
 *     - communication analysis;
 *     - coordination analysis;
 *     - resource analysis;
 *     - capability negotiation;
 *     - effect analysis;
 *     - policy enforcement;
 *     - provenance;
 *     - distributed lowering;
 *     - deterministic execution analysis.
 *
 * None of those semantics are implemented in this grammar.
 *
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Zamani must not require a lexer keyword for every future agent concept.
 *
 * Existing agent syntax intentionally supports:
 *
 *     @identifier
 *
 * Therefore future concepts can be represented structurally without expanding
 * the permanent lexical vocabulary.
 *
 * Examples of semantic roles include:
 *
 *     @agent
 *     @group
 *     @delegate
 *     @coordinate
 *     @message
 *     @observe
 *     @act
 *     @goal
 *     @plan
 *     @supervise
 *     @checkpoint
 *     @learn
 *     @adapt
 *     @reason
 *     @explain
 *     @evidence
 *     @provenance
 *     @requires
 *     @policy
 *
 * These are semantic identifiers.
 *
 * This grammar MUST NOT create permanent lexer keywords for them merely because
 * they are useful agent concepts.
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required lexical primitives are inherited through the imported grammars.
 *
 * This file MUST NOT declare lexer rules.
 *
 * This file MUST NOT introduce:
 *
 *     AGENT
 *     GROUP
 *     DELEGATE
 *     COORDINATE
 *     MESSAGE
 *     SUPERVISE
 *     MULTI_AGENT
 *
 * as mandatory lexer tokens.
 *
 * The canonical annotation form remains:
 *
 *     AT identifier
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/ai/agents.g4
 *     grammar/concurrency/actors.g4
 *
 * The imported agent grammar provides:
 *
 *     agentConstruct
 *     agentBody
 *     agentMember
 *     agentExpression
 *     agentType
 *     agentReference
 *
 * The imported actor grammar provides:
 *
 *     actorConstruct
 *     actorDeclaration
 *     actorSpawnExpression
 *     actorSendExpression
 *     actorAskExpression
 *     actorForwardExpression
 *     actorLifecycleExpression
 *     actorSupervisionConstruct
 *
 * General expressions/types/statements are inherited through those canonical
 * grammars.
 *
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PRIMARY PUBLIC RULE
 * -------------------
 *
 *     multiAgentConstruct
 *
 * This is the only rule that the AI composition grammar should normally
 * consume.
 *
 *
 * SECONDARY PUBLIC RULES
 * ----------------------
 *
 *     multiAgentAgentConstruct
 *     multiAgentActorConstruct
 *     multiAgentGroup
 *     multiAgentComposition
 *     multiAgentDelegation
 *     multiAgentCoordination
 *     multiAgentCommunication
 *     multiAgentSupervision
 *     multiAgentReference
 *     multiAgentExpression
 *     multiAgentType
 *
 * These rules exist for semantic tooling, conformance tests, and domain
 * adapters.
 *
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/ai/ai.g4
 *
 * Recommended AI composition:
 *
 *     import
 *         Types,
 *         Expressions,
 *         Statements,
 *         AIModels,
 *         AIDatasets,
 *         AITensors,
 *         AITraining,
 *         Inference,
 *         MultiAgent,
 *         AIDifferentiable,
 *         AIPipelines,
 *         AIAccelerators,
 *         ModelDeployment
 *     ;
 *
 * AI.g4 should consume:
 *
 *     multiAgentConstruct
 *
 * rather than importing Agents separately in the same composition layer.
 *
 * MultiAgent.g4 owns the composition dependency on Agents.
 *
 *
 * ============================================================================
 * DEPENDENCY-DIRECTION RULE
 * ============================================================================
 *
 * Dependency direction is:
 *
 *     AI
 *      |
 *      v
 *     MultiAgent
 *      |
 *      +----------------+
 *      |                |
 *      v                v
 *   Agents           Actors
 *
 * NOT:
 *
 *     AI -> Agents
 *     AI -> MultiAgent
 *     MultiAgent -> Agents
 *
 * simultaneously.
 *
 * That would risk duplicate imported rule ownership.
 *
 * Therefore the canonical integration is:
 *
 *     AI -> MultiAgent -> Agents
 *                        -> Actors
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST remains domain-neutral.
 *
 * The AST should preserve enough source structure for semantic analysis to
 * distinguish:
 *
 *     agent membership
 *     actor participation
 *     group membership
 *     composition nesting
 *     delegation relationship
 *     coordination relationship
 *     communication relationship
 *     supervision relationship
 *     expressions
 *     types
 *     source locations
 *
 * The AST MUST NOT contain:
 *
 *     AgentCPU
 *     AgentGPU
 *     AgentQPU
 *     AgentNode
 *     AgentDevice
 *     AgentThread
 *     PhysicalAgent
 *     PhysicalMailbox
 *     PhysicalAgentAddress
 *
 * or equivalent target-specific structures.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether a collection actually represents multiple agents;
 *     - whether referenced agents exist;
 *     - whether references are valid;
 *     - whether delegation is permitted;
 *     - whether coordination is valid;
 *     - whether communication is legal;
 *     - whether actor-backed communication is legal;
 *     - whether supervision relationships are valid;
 *     - whether cycles are permitted;
 *     - whether ordering constraints are satisfied;
 *     - whether policies permit the operation;
 *     - whether capabilities exist;
 *     - whether resources satisfy requirements;
 *     - whether effects are permitted;
 *     - whether adaptation is authorized;
 *     - whether provenance requirements are satisfied;
 *     - whether distributed execution is feasible;
 *     - whether the resulting computation is portable.
 *
 * The parser MUST NOT make these decisions.
 *
 *
 * ============================================================================
 * AGENT SEMANTICS
 * ============================================================================
 *
 * Individual agents remain owned by Agents.
 *
 * This file therefore uses:
 *
 *     agentConstruct
 *
 * rather than redefining:
 *
 *     @agent
 *     @goal
 *     @model
 *     @tool
 *     @memory
 *     @plan
 *     @observe
 *     @act
 *
 * This preserves a single agent syntax authority.
 *
 *
 * ============================================================================
 * ACTOR SEMANTICS
 * ============================================================================
 *
 * Actors remain owned by Actors.
 *
 * This file therefore consumes:
 *
 *     actorConstruct
 *
 * rather than defining another:
 *
 *     actor
 *     mailbox
 *     send
 *     receive
 *     spawn
 *     supervise
 *
 * language.
 *
 * An agent MAY be realized by an actor, but:
 *
 *     agent != actor
 *
 * An actor is a concurrency realization model.
 *
 * An agent is a semantic computation participant.
 *
 * Semantic analysis determines whether and how they correspond.
 *
 *
 * ============================================================================
 * MULTI-AGENT CONSTRUCT
 * ============================================================================
 *
 * The primary boundary accepts:
 *
 *     individual agent constructs;
 *     actor constructs;
 *     multi-agent groups;
 *     multi-agent compositions.
 *
 * It does not force every agent to use an actor runtime.
 *
 * This permits implementations such as:
 *
 *     sequential agents
 *     asynchronous agents
 *     actor-backed agents
 *     distributed agents
 *     accelerator-backed agents
 *     quantum-assisted agents
 *     simulator-backed agents
 *     heterogeneous agents
 *     future execution substrates
 *
 * without changing the source grammar.
 *
 *
 * ============================================================================
 * GROUP MODEL
 * ============================================================================
 *
 * A group is a logical collection of agent constructs.
 *
 * The grammar does not assign:
 *
 *     CPU
 *     GPU
 *     QPU
 *     node
 *     process
 *     thread
 *     physical address
 *
 * to any member.
 *
 * Example conceptual structure:
 *
 *     @group Research {
 *         @agent Analyst {
 *             ...
 *         }
 *
 *         @agent Planner {
 *             ...
 *         }
 *     }
 *
 * The actual meaning of `group`, `Research`, `Analyst`, and `Planner` is
 * determined by semantic analysis.
 *
 * Because the base agent grammar is open-world, no new lexer keyword is
 * required for `group`.
 *
 *
 * ============================================================================
 * COMPOSITION MODEL
 * ============================================================================
 *
 * Multi-agent composition is intentionally generic.
 *
 * A composition may contain:
 *
 *     agents
 *     actors
 *     ordinary statements
 *     nested groups
 *     expressions
 *
 * This permits a group to contain computation rather than being a static
 * metadata-only declaration.
 *
 *
 * ============================================================================
 * DELEGATION
 * ============================================================================
 *
 * Delegation is a semantic relationship between computation participants.
 *
 * The grammar exposes:
 *
 *     multiAgentDelegation
 *
 * as a stable parser-facing boundary.
 *
 * It does not define:
 *
 *     task scheduling
 *     load balancing
 *     worker allocation
 *     resource assignment
 *     network transport
 *     authorization
 *
 * Those are downstream concerns.
 *
 * Conceptual form:
 *
 *     @delegate target(task);
 *
 * or:
 *
 *     @delegate target {
 *         ...
 *     }
 *
 * The generic annotation structure remains open-world.
 *
 *
 * ============================================================================
 * COORDINATION
 * ============================================================================
 *
 * Coordination is a semantic relationship among multiple computation
 * participants.
 *
 * The grammar exposes:
 *
 *     multiAgentCoordination
 *
 * without enumerating coordination algorithms.
 *
 * Therefore the language does NOT require grammar additions for:
 *
 *     consensus
 *     voting
 *     leader election
 *     bargaining
 *     negotiation
 *     planning
 *     distributed search
 *     swarm algorithms
 *     future coordination algorithms
 *
 * Those algorithms belong to semantic libraries, dialects, capabilities,
 * policies, or runtime systems.
 *
 *
 * ============================================================================
 * COMMUNICATION
 * ============================================================================
 *
 * Multi-agent communication must reuse the existing communication model.
 *
 * The grammar does not define a new message language.
 *
 * Agent communication may ultimately use:
 *
 *     actor messages
 *     channels
 *     asynchronous communication
 *     networking
 *     distributed messaging
 *     service calls
 *     future communication substrates
 *
 * This file exposes a semantic composition boundary only.
 *
 *
 * ============================================================================
 * SUPERVISION
 * ============================================================================
 *
 * Supervision may be realized through:
 *
 *     actor supervision
 *     distributed supervision
 *     policy-controlled recovery
 *     resilience management
 *     execution recovery
 *
 * The parser does not decide which implementation is used.
 *
 * Existing actor supervision remains authoritative.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Multi-agent syntax MUST NOT encode physical capacities.
 *
 * Forbidden universal constructs include:
 *
 *     run_on_8_agents
 *     use_4_workers
 *     agent_cpu_0
 *     agent_gpu_0
 *     agent_qpu_0
 *     run_on_16_nodes
 *     max_agents = 100
 *     max_workers = 1000
 *
 * These are not language architecture.
 *
 * Resource intent belongs to:
 *
 *     grammar/resources/
 *
 * For example, semantic resource requirements may express:
 *
 *     requires capability("distributed.compute");
 *
 *     requires capability("agent.coordination");
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires topology(required_topology);
 *
 * without fixing the physical realization.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Agent capabilities remain open-world.
 *
 * Examples:
 *
 *     capability("agent.reasoning")
 *     capability("agent.learning")
 *     capability("agent.coordination")
 *     capability("distributed.compute")
 *     capability("network.messaging")
 *     capability("quantum.compute")
 *     capability("tensor.compute")
 *
 * These are capability identities, not grammar keywords.
 *
 * Capability declaration and resolution remain owned by:
 *
 *     grammar/resources/
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Multi-agent computation may produce or require effects such as:
 *
 *     io
 *     network
 *     distributed
 *     mutation
 *     randomness
 *     learning
 *     adaptation
 *     measurement
 *     foreign
 *     native
 *     reflection
 *     simulation
 *
 * Effects are owned by:
 *
 *     grammar/effects/
 *
 * This file does not enumerate or enforce them.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Multi-agent computation may be governed by:
 *
 *     authorization
 *     resource policy
 *     communication policy
 *     delegation policy
 *     adaptation policy
 *     security policy
 *     deployment policy
 *     fallback policy
 *
 * Policy ownership remains outside this grammar.
 *
 *
 * ============================================================================
 * CONTRACTS
 * ============================================================================
 *
 * Multi-agent constructs may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * The contract subsystem remains authoritative.
 *
 * This grammar MUST NOT reproduce contract syntax.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Multi-agent operations must remain source-traceable.
 *
 * Downstream provenance may record:
 *
 *     source
 *     agent
 *     relationship
 *     delegation
 *     coordination
 *     communication
 *     decision
 *     evidence
 *     transformation
 *     policy
 *     execution context
 *     version
 *
 * The grammar only preserves source structure.
 *
 *
 * ============================================================================
 * KNOWLEDGE / REASONING / LEARNING / ADAPTATION
 * ============================================================================
 *
 * Multi-agent computation may use the existing generic semantic capabilities
 * for:
 *
 *     reasoning
 *     inference
 *     deduction
 *     knowledge
 *     querying
 *     learning
 *     adaptation
 *     explanation
 *     evidence
 *     uncertainty
 *
 * This file MUST NOT create:
 *
 *     AgentReasoningIR
 *     AgentLearningIR
 *     AgentKnowledgeIR
 *
 * The same semantic facilities must remain usable by ordinary programs,
 * classical computation, quantum computation, hybrid computation, distributed
 * computation, and other domains.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A multi-agent computation may contain or invoke quantum computation.
 *
 * This file does NOT define:
 *
 *     qubits
 *     gates
 *     circuits
 *     measurements
 *     topology
 *     calibration
 *     routing
 *     QEC
 *
 * Those remain owned by:
 *
 *     grammar/quantum/
 *
 * Quantum semantics must eventually reach:
 *
 *     quantum::ir
 *
 * through the ordinary semantic pipeline.
 *
 * There is no:
 *
 *     MultiAgentQuantumIR
 *
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Multi-agent computation may coordinate:
 *
 *     classical computation
 *     quantum computation
 *     tensor computation
 *     accelerator computation
 *     HDL/hardware intent
 *     distributed computation
 *
 * The multi-agent grammar remains domain-neutral.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Agents may coordinate hardware-oriented computation, but this grammar does
 * not define hardware syntax.
 *
 * It must not encode:
 *
 *     fixed register widths
 *     fixed wire widths
 *     fixed device counts
 *     fixed accelerator counts
 *     fixed FPGA sizes
 *     fixed ASIC resources
 *     fixed memory sizes
 *
 * Hardware intent remains owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Multi-agent execution may be distributed.
 *
 * This grammar does not define:
 *
 *     node IDs
 *     node counts
 *     network topology
 *     placement
 *     routing
 *     consensus implementation
 *     distributed storage
 *
 * Those concerns belong to:
 *
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/resources/
 *
 * A multi-agent source program therefore describes semantic relationships,
 * not physical deployment.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text
 *     lexer vocabulary
 *     grammar version
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     number of agents available at runtime
 *     number of CPUs
 *     number of GPUs
 *     number of QPUs
 *     network state
 *     resource availability
 *     filesystem state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     runtime agent state
 *
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO universal limits on:
 *
 *     agents
 *     groups
 *     nested groups
 *     relationships
 *     messages
 *     goals
 *     tasks
 *     delegations
 *     coordination relationships
 *     supervisors
 *     actors
 *     nodes
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     qubits
 *     memory
 *     threads
 *     tensor dimensions
 *     network size
 *
 * There must be no language constants such as:
 *
 *     MAX_AGENTS
 *     MAX_GROUPS
 *     MAX_DELEGATIONS
 *     MAX_MESSAGES
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_MEMORY
 *
 * Repetition is represented structurally by grammar repetition.
 *
 * Actual implementation limits are compiler/runtime/environment concerns and
 * must never be silently presented as language-level semantic limits.
 *
 *
 * ============================================================================
 * INDEPENDENT FILE COMPLETION
 * ============================================================================
 *
 * This file is independently complete when:
 *
 *     1. `MultiAgent` generates successfully.
 *
 *     2. `multiAgentConstruct` is stable.
 *
 *     3. Agent syntax is owned by Agents.
 *
 *     4. Actor syntax is owned by Actors.
 *
 *     5. No second actor system exists.
 *
 *     6. No second message system exists.
 *
 *     7. No second distributed system exists.
 *
 *     8. No multi-agent-specific IR exists.
 *
 *     9. General expressions come from the canonical expression grammar.
 *
 *    10. General types come from the canonical type grammar.
 *
 *    11. Resource semantics remain target-independent.
 *
 *    12. Capability semantics remain open-world.
 *
 *    13. Effects remain owned by the effects subsystem.
 *
 *    14. Policies remain owned by the policy subsystem.
 *
 *    15. Contracts remain owned by validation/contracts.
 *
 *    16. Provenance remains owned by provenance.
 *
 *    17. Quantum constructs can eventually reach quantum::ir.
 *
 *    18. Hardware realization is not encoded.
 *
 *    19. Distributed realization is not encoded.
 *
 *    20. No physical machine is selected by parsing.
 *
 *    21. No universal capacity ceiling is encoded.
 *
 *    22. The grammar is deterministic.
 *
 *    23. The grammar contains no actions.
 *
 *    24. The grammar contains no semantic predicates.
 *
 *    25. The grammar contains no Rust code.
 *
 *    26. No unsafe Rust is required.
 *
 *    27. Positive tests exist.
 *
 *    28. Negative tests exist.
 *
 *    29. Boundary tests exist.
 *
 *    30. Scalability tests exist.
 *
 *    31. Cross-domain tests exist.
 *
 *    32. Compatibility tests exist.
 *
 *    33. Determinism tests exist.
 *
 *
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
 * Agents owns agent syntax.
 *
 * Actors owns actor syntax.
 *
 * No rules are copied from either grammar.
 *
 * The dependency graph is deliberately:
 *
 *     MultiAgent
 *        |
 *        +----> Agents
 *        |
 *        +----> Actors
 *
 * The AI composition grammar should import MultiAgent rather than importing
 * Agents independently in the same composition layer.
 *
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
 * This is the canonical entry point for all multi-agent grammar consumers.
 *
 * It intentionally composes existing agent and actor constructs.
 */

multiAgentConstruct
    : multiAgentAgentConstruct
    | multiAgentActorConstruct
    | multiAgentGroup
    | multiAgentComposition
    ;


/*
 * ============================================================================
 * 2. INDIVIDUAL AGENT CONSTRUCT
 * ============================================================================
 *
 * Alias boundary.
 *
 * The actual syntax remains owned by Agents.
 */

multiAgentAgentConstruct
    : agentConstruct
    ;


/*
 * ============================================================================
 * 3. ACTOR-BACKED CONSTRUCT
 * ============================================================================
 *
 * Alias boundary.
 *
 * The actual syntax remains owned by Actors.
 */

multiAgentActorConstruct
    : actorConstruct
    ;


/*
 * ============================================================================
 * 4. MULTI-AGENT GROUP
 * ============================================================================
 *
 * Generic structural group.
 *
 * Example:
 *
 *     @group Research {
 *         @agent Analyst {
 *             ...
 *         }
 *
 *         @agent Planner {
 *             ...
 *         }
 *     }
 *
 * `group` is deliberately an identifier, not a lexer keyword.
 *
 * Semantic analysis determines whether the annotation identifies a group
 * construct and what its policy/meaning is.
 *
 * The grammar therefore remains open-world.
 */

multiAgentGroup
    : AT
      identifier
      identifier
      multiAgentGroupTail
    ;


/*
 * ============================================================================
 * 5. GROUP TAIL
 * ============================================================================
 *
 * A group can have:
 *
 *     a block;
 *     a typed declaration;
 *     an initializer;
 *     an invocation;
 *     an empty declaration.
 *
 * The semantic layer determines which structural form is valid for the
 * registered group construct.
 */

multiAgentGroupTail
    : multiAgentBody
    | multiAgentGroupInvocation
    | multiAgentGroupInitializer
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 6. GROUP INVOCATION
 * ============================================================================
 */

multiAgentGroupInvocation
    : LPAREN
      multiAgentArgumentList?
      RPAREN
      multiAgentInvocationTail
    ;


multiAgentInvocationTail
    : multiAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 7. GROUP INITIALIZER
 * ============================================================================
 */

multiAgentGroupInitializer
    : ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. MULTI-AGENT COMPOSITION
 * ============================================================================
 *
 * Composition is intentionally structural.
 *
 * A composition can contain:
 *
 *     agents
 *     actors
 *     nested groups
 *     ordinary statements
 *
 * This permits multi-agent computation to remain ordinary Zamani computation
 * rather than creating a closed AI-only language.
 */

multiAgentComposition
    : AT
      identifier
      multiAgentCompositionTail
    ;


multiAgentCompositionTail
    : multiAgentCompositionInvocation
    | multiAgentBody
    | SEMICOLON
    ;


multiAgentCompositionInvocation
    : LPAREN
      multiAgentArgumentList?
      RPAREN
      multiAgentInvocationTail
    ;


/*
 * ============================================================================
 * 9. MULTI-AGENT BODY
 * ============================================================================
 *
 * Recursive by design.
 *
 * There is no fixed nesting depth encoded in the grammar.
 *
 * The implementation must nevertheless provide controlled resource-exhaustion
 * diagnostics when finite parser/runtime resources are exhausted.
 */

multiAgentBody
    : LBRACE
      multiAgentMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 10. MULTI-AGENT MEMBER
 * ============================================================================
 *
 * Members may be:
 *
 *     agents
 *     actors
 *     nested groups
 *     nested compositions
 *     ordinary Zamani statements
 *
 * This permits future domain integration without changing this grammar.
 */

multiAgentMember
    : multiAgentConstruct
    | statement
    ;


multiAgentMembers
    : multiAgentMember*
    ;


/*
 * ============================================================================
 * 11. ARGUMENT LIST
 * ============================================================================
 *
 * Expressions remain owned by the canonical expression grammar.
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
 * 12. MULTI-AGENT REFERENCE
 * ============================================================================
 *
 * Semantic tooling may use this boundary when resolving:
 *
 *     agent references
 *     actor references
 *     group references
 *     delegation targets
 *     coordination participants
 *     communication participants
 *
 * The actual expression syntax remains canonical.
 */

multiAgentReference
    : expression
    ;


/*
 * ============================================================================
 * 13. MULTI-AGENT EXPRESSION
 * ============================================================================
 *
 * Stable expression bridge.
 */

multiAgentExpression
    : expression
    ;


/*
 * ============================================================================
 * 14. MULTI-AGENT TYPE
 * ============================================================================
 *
 * Stable type bridge.
 *
 * There is deliberately no:
 *
 *     MultiAgentType
 *
 * competing with the Zamani type system.
 */

multiAgentType
    : typeExpression
    ;


/*
 * ============================================================================
 * 15. DELEGATION SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Delegation uses the existing open-world annotation model.
 *
 * Example conceptual forms:
 *
 *     @delegate worker(task);
 *
 *     @delegate worker {
 *         ...
 *     }
 *
 * The parser exposes a stable boundary while semantic analysis verifies that
 * the annotation actually represents delegation.
 *
 * No delegation keyword is required in the lexer.
 */

multiAgentDelegation
    : AT
      identifier
      multiAgentDelegationTail
    ;


multiAgentDelegationTail
    : identifier
      multiAgentDelegationTargetTail
    | multiAgentBody
    | SEMICOLON
    ;


multiAgentDelegationTargetTail
    : LPAREN
      multiAgentArgumentList?
      RPAREN
      multiAgentInvocationTail
    | ASSIGN
      expression
      SEMICOLON
    | multiAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 16. COORDINATION SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This boundary deliberately does not enumerate algorithms.
 *
 * Coordination algorithms remain semantic/library/runtime concepts.
 */

multiAgentCoordination
    : AT
      identifier
      multiAgentCoordinationTail
    ;


multiAgentCoordinationTail
    : identifier
      multiAgentCoordinationTargetTail
    | multiAgentBody
    | SEMICOLON
    ;


multiAgentCoordinationTargetTail
    : LPAREN
      multiAgentArgumentList?
      RPAREN
      multiAgentInvocationTail
    | ASSIGN
      expression
      SEMICOLON
    | multiAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 17. COMMUNICATION SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Communication is deliberately represented as an integration boundary.
 *
 * Actual message syntax remains owned by concurrency/networking/distributed
 * subsystems.
 *
 * This prevents creation of a third message model.
 */

multiAgentCommunication
    : AT
      identifier
      multiAgentCommunicationTail
    ;


multiAgentCommunicationTail
    : expression
      multiAgentCommunicationSuffix
    | multiAgentBody
    | SEMICOLON
    ;


multiAgentCommunicationSuffix
    : LPAREN
      multiAgentArgumentList?
      RPAREN
      multiAgentInvocationTail
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 18. SUPERVISION SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Existing actor supervision remains authoritative.
 *
 * This rule exists so semantic tooling can recognize supervision-oriented
 * multi-agent composition without duplicating actor supervision grammar.
 */

multiAgentSupervision
    : actorSupervisionConstruct
    | AT
      identifier
      multiAgentSupervisionTail
    ;


multiAgentSupervisionTail
    : identifier
      multiAgentSupervisionTargetTail
    | multiAgentBody
    | SEMICOLON
    ;


multiAgentSupervisionTargetTail
    : LPAREN
      multiAgentArgumentList?
      RPAREN
      multiAgentInvocationTail
    | ASSIGN
      expression
      SEMICOLON
    | multiAgentBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 19. RELATIONSHIP COMPOSITION
 * ============================================================================
 *
 * Stable umbrella boundary for semantic tooling.
 *
 * The concrete operation remains structurally open-world.
 */

multiAgentRelationship
    : multiAgentDelegation
    | multiAgentCoordination
    | multiAgentCommunication
    | multiAgentSupervision
    ;


/*
 * ============================================================================
 * 20. RELATIONSHIP COLLECTION
 * ============================================================================
 *
 * No fixed number of relationships is encoded.
 */

multiAgentRelationships
    : multiAgentRelationship*
    ;


/*
 * ============================================================================
 * 21. PARTICIPANT COLLECTION
 * ============================================================================
 *
 * Stable semantic boundary for participant analysis.
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
 * 22. SEMANTIC COMPOSITION UNIT
 * ============================================================================
 *
 * This boundary is useful for semantic analysis without introducing a new IR.
 */

multiAgentUnit
    : multiAgentConstruct
    | multiAgentRelationship
    | statement
    ;


/*
 * ============================================================================
 * 23. SEMANTIC COMPOSITION UNITS
 * ============================================================================
 */

multiAgentUnits
    : multiAgentUnit*
    ;


/*
 * ============================================================================
 * 24. INTEGRATION ALIASES
 * ============================================================================
 *
 * These aliases intentionally contain no new syntax.
 *
 * They give downstream semantic tooling stable names while keeping syntax
 * ownership in the existing canonical grammars.
 */

agentMultiAgentConstruct
    : multiAgentAgentConstruct
    ;


actorMultiAgentConstruct
    : multiAgentActorConstruct
    ;


groupMultiAgentConstruct
    : multiAgentGroup
    ;


relationshipMultiAgentConstruct
    : multiAgentRelationship
    ;


/*
 * ============================================================================
 * 25. DOMAIN-NEUTRAL TARGET BOUNDARY
 * ============================================================================
 *
 * Multi-agent syntax must never select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     node
 *     process
 *     thread
 *     physical device
 *
 * Target selection occurs after semantic analysis.
 *
 * This rule exists solely as an explicit architecture marker for tooling.
 */

multiAgentTargetIndependentConstruct
    : multiAgentConstruct
    ;


/*
 * ============================================================================
 * 26. SOURCE-ORDER PRESERVATION
 * ============================================================================
 *
 * Parser consumers must preserve source ordering in the resulting AST.
 *
 * This rule is intentionally simple and does not perform ordering analysis.
 */

multiAgentOrderedMember
    : multiAgentMember
    ;


/*
 * ============================================================================
 * 27. NESTED COMPOSITION
 * ============================================================================
 *
 * Explicit named boundary for recursive composition.
 */

nestedMultiAgentConstruct
    : multiAgentConstruct
    | multiAgentBody
    ;


/*
 * ============================================================================
 * 28. FINAL PUBLIC AGGREGATION
 * ============================================================================
 *
 * This rule is useful to conformance tooling and should not normally be
 * consumed directly by ZamaniParser.
 */

multiAgentDomainConstruct
    : multiAgentConstruct
    | multiAgentRelationship
    | multiAgentSupervision
    ;