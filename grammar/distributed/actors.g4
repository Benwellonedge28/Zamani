/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Distributed Actor Qualification Grammar
 * ============================================================================
 *
 * File:
 *     grammar/distributed/actors.g4
 *
 * Grammar:
 *     DistributedActors
 *
 * Purpose:
 *     Qualify the canonical Zamani actor model for distributed realization.
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust Edition 2021
 *
 * Safety:
 *     Safe Rust only.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * GENERAL ACTOR AUTHORITY
 * -----------------------
 *
 *     grammar/concurrency/actors.g4
 *
 * owns:
 *
 *     actor declaration
 *     actor state
 *     actor handlers
 *     actor construction/spawn
 *     actor commands
 *     actor messaging
 *     actor lifecycle
 *     actor supervision
 *     actor targets
 *
 *
 * DISTRIBUTED ACTOR AUTHORITY
 * ---------------------------
 *
 * This file owns only the distributed qualification:
 *
 *     distributed actor <canonical actor declaration>
 *
 * It does NOT create a second actor language.
 *
 *
 * DISTRIBUTED DOMAIN AUTHORITY
 * ----------------------------
 *
 *     grammar/distributed/distributed.g4
 *
 * owns distributed-domain composition.
 *
 *
 * RESOURCE AUTHORITY
 * ------------------
 *
 *     grammar/resources/
 *
 * owns:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     resource negotiation
 *     scaling
 *     placement/resource realization
 *
 *
 * EFFECT AUTHORITY
 * ----------------
 *
 *     grammar/effects/
 *
 * owns effect syntax and semantic effect classification.
 *
 *
 * NETWORK AUTHORITY
 * -----------------
 *
 *     grammar/networking/
 *
 * owns network-specific syntax.
 *
 *
 * PLACEMENT AUTHORITY
 * -------------------
 *
 *     grammar/distributed/placement.g4
 *     grammar/resources/placement.g4
 *
 * own placement syntax at their respective semantic boundaries.
 *
 *
 * REPLICATION AUTHORITY
 * ---------------------
 *
 *     grammar/distributed/replication.g4
 *
 * owns replication semantics.
 *
 *
 * QUANTUM AUTHORITY
 * -----------------
 *
 *     grammar/quantum/
 *
 * owns quantum syntax and semantics.
 *
 * Quantum lowering ultimately uses:
 *
 *     quantum::ir
 *
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * A distributed actor is not a different kind of actor.
 *
 * It is the canonical actor model with an additional distributed semantic
 * qualification.
 *
 * Conceptually:
 *
 *     distributed actor
 *          =
 *     distributed qualification
 *          +
 *     ordinary Zamani actor
 *
 * Therefore:
 *
 *     distributed actor Counter { ... }
 *
 * is parsed as:
 *
 *     distributed qualification
 *          +
 *     actor declaration
 *
 * The complete actor body, state, handlers, commands, lifecycle and
 * supervision rules remain owned by `grammar/concurrency/actors.g4`.
 *
 * ============================================================================
 * WHY THIS FILE MUST BE SMALL
 * ============================================================================
 *
 * The previous architecture duplicated:
 *
 *     actor state
 *     actor handlers
 *     actor lifecycle
 *     actor messaging
 *     actor targets
 *     actor supervision
 *     actor spawning
 *
 * in this file.
 *
 * That created two actor grammars:
 *
 *     grammar/concurrency/actors.g4
 *     grammar/distributed/actors.g4
 *
 * Two actor grammars inevitably drift.
 *
 * The production architecture instead has:
 *
 *     ONE actor syntax
 *     ONE actor semantic model
 *     ONE actor AST path
 *     ONE actor concurrency model
 *
 * with distributed qualification applied at the domain boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A distributed actor expresses logical computational intent.
 *
 * It does NOT specify:
 *
 *     machine
 *     CPU
 *     core
 *     thread
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     node
 *     process
 *     network address
 *     transport
 *     scheduler
 *     runtime
 *     cloud provider
 *     physical topology
 *
 * The same logical actor may therefore be realized as:
 *
 *     local execution
 *     asynchronous execution
 *     one process
 *     multiple processes
 *     one node
 *     many nodes
 *     edge execution
 *     cluster execution
 *     HPC execution
 *     cloud execution
 *     heterogeneous execution
 *     accelerator-backed execution
 *     quantum/classical orchestration
 *     future computational substrates
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level capacity limits.
 *
 * In particular, it contains no:
 *
 *     MAX_ACTORS
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_MESSAGES
 *     MAX_REPLICAS
 *     MAX_CHANNELS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *
 * No physical capacity is encoded in this grammar.
 *
 * "Infinity" in the POCO-REAF sense means:
 *
 *     no artificial finite capacity is introduced by the language grammar.
 *
 * Actual limits remain properties of:
 *
 *     source size
 *     compiler resources
 *     semantic feasibility
 *     resource availability
 *     deployment policy
 *     runtime resources
 *     target capabilities
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Distributed qualification does not allocate resources.
 *
 * It does not answer:
 *
 *     where should this actor execute?
 *     how many workers should execute it?
 *     which machine should host it?
 *     which transport should be used?
 *
 * Those questions belong downstream.
 *
 * A distributed actor can participate in universal requirements such as:
 *
 *     requires capability("distributed.actor");
 *     requires capability("message.passing");
 *     requires capability("quantum.measurement");
 *     requires capability("tensor.compute");
 *
 * but this file does not define the resource grammar.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * The spelling:
 *
 *     distributed
 *
 * is intentionally contextual here.
 *
 * The repository's lexical architecture does not require a dedicated
 * DISTRIBUTED lexer token for every distributed-domain concept.
 *
 * This prevents a distributed-only keyword from becoming a new lexical
 * authority.
 *
 * Semantic validation must establish that the contextual marker has the
 * canonical spelling required by this construct.
 *
 * Future distributed concepts remain expressible through existing names,
 * qualified names, attributes, policies, capabilities and dialects.
 *
 * Examples:
 *
 *     distributed::replication
 *     distributed::consistency
 *     distributed::migration
 *     distributed::federation
 *     distributed::locality
 *
 * These are semantic names, not a closed parser keyword inventory.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not create a distributed-actor-specific AST.
 *
 * The parser structure is:
 *
 *     distributedActorDeclaration
 *             |
 *             +--> distributed marker
 *             |
 *             +--> actorDeclaration
 *
 * The existing actor AST representation must remain authoritative for:
 *
 *     actor name
 *     generic parameters
 *     inheritance
 *     members
 *     state
 *     handlers
 *     message contracts
 *     actor body
 *     source spans
 *
 * The distributed qualification is represented by the domain-neutral
 * declaration/attribute/semantic metadata mechanism used by the frontend.
 *
 * The exact representation is an AST/semantic implementation concern and
 * MUST NOT require a runtime-specific type such as:
 *
 *     DistributedActorHandle
 *     RemoteActorHandle
 *     NodeActor
 *     ProcessActor
 *     NetworkActor
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the wrapped declaration is a valid actor;
 *     - whether distributed qualification is valid;
 *     - whether distributed realization is permitted;
 *     - which capabilities are required;
 *     - which effects are produced;
 *     - which resources are required;
 *     - which policies apply;
 *     - whether placement constraints are satisfiable;
 *     - whether replication is legal;
 *     - whether communication semantics are valid;
 *     - whether ownership/lifetime rules remain valid;
 *     - whether the actor can participate in distributed execution;
 *     - whether target realization preserves source semantics.
 *
 * This grammar performs none of those semantic checks.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Distributed qualification does not automatically assign a physical effect.
 *
 * In particular:
 *
 *     distributed actor
 *
 * does not inherently mean:
 *
 *     network
 *     IO
 *     mutation
 *     distributed transport
 *
 * The actor may ultimately be realized using:
 *
 *     local communication
 *     shared memory
 *     runtime messaging
 *     accelerator interconnect
 *     distributed transport
 *     another future mechanism
 *
 * Effect classification belongs to semantic analysis.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Distributed actors may be affected by:
 *
 *     resource policies
 *     security policies
 *     execution policies
 *     placement policies
 *     replication policies
 *     consistency policies
 *     migration policies
 *     resilience policies
 *     adaptation policies
 *     reproducibility policies
 *
 * This file does not define those policies.
 *
 * Policies are resolved through the repository's canonical policy system.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Source locations must remain available for:
 *
 *     distributed marker
 *     actor declaration
 *     actor name
 *     actor members
 *     actor body
 *
 * The frontend must preserve provenance through:
 *
 *     source
 *       ->
 *     parse tree
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic model
 *       ->
 *     canonical IR
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     scheduling
 *       ->
 *     deployment
 *       ->
 *     runtime
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * QUANTUM / CLASSICAL / HDL / AI INTEGRATION
 * ============================================================================
 *
 * A distributed actor remains a normal actor.
 *
 * Therefore its body may contain valid Zamani constructs from:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     networking
 *     memory
 *     effects
 *     resources
 *     validation
 *     policies
 *     metaprogramming
 *
 * No domain-specific actor grammar is created here.
 *
 * If quantum operations occur:
 *
 *     actor semantics
 *          ->
 *     quantum semantic analysis
 *          ->
 *     quantum::ir
 *
 * No distributed-quantum IR is introduced.
 *
 * ============================================================================
 * DISTRIBUTED COMMUNICATION
 * ============================================================================
 *
 * Actor messaging remains owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * and related concurrency/channel grammars.
 *
 * Distributed communication realization remains owned by:
 *
 *     grammar/distributed/communication.g4
 *     grammar/distributed/messaging.g4
 *     grammar/networking/
 *
 * Therefore this file does NOT define:
 *
 *     send
 *     ask
 *     forward
 *     receive
 *     channel
 *     endpoint
 *     route
 *     transport
 *     packet
 *
 * again.
 *
 * ============================================================================
 * PLACEMENT / REPLICATION / TOPOLOGY
 * ============================================================================
 *
 * Distributed qualification does not define physical placement.
 *
 * Placement, replication and topology remain independently owned.
 *
 * This avoids coupling an actor declaration to a finite physical topology.
 *
 * Logical relationships may be described by the existing distributed
 * resource/policy mechanisms.
 *
 * ============================================================================
 * LIFECYCLE / SUPERVISION
 * ============================================================================
 *
 * Actor lifecycle and supervision remain owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * Distributed recovery, migration, replication and fault tolerance remain
 * distributed semantic/runtime concerns.
 *
 * This file does not introduce another lifecycle model.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is independent of:
 *
 *     hardware
 *     node count
 *     processor count
 *     memory availability
 *     accelerator availability
 *     QPU availability
 *     network availability
 *     deployment state
 *     runtime state
 *     wall-clock time
 *     randomness
 *
 * Identical token streams and parser configuration must produce equivalent
 * parser structures.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Distributed qualification does not grant:
 *
 *     network access
 *     filesystem access
 *     native execution
 *     secret access
 *     hardware access
 *     remote execution authority
 *     deployment authority
 *
 * Authorization and capability checking remain downstream.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It:
 *
 *     - uses ZamaniLexer;
 *     - imports the canonical actor grammar;
 *     - contains no lexer rules;
 *     - contains no parser actions;
 *     - contains no semantic predicates;
 *     - contains no embedded Rust;
 *     - performs no I/O;
 *     - performs no hardware inspection;
 *     - performs no resource discovery;
 *     - performs no network access;
 *     - performs no runtime calls.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/concurrency/actors.g4
 *     grammar/core/names.g4
 *     canonical Zamani lexer
 *
 * EXPORTS:
 *
 *     distributedActorConstruct
 *     distributedActorDeclaration
 *     distributedActorKeyword
 *
 * CONSUMED_BY:
 *
 *     grammar/distributed/distributed.g4
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *     existing actor AST/semantic representation
 *
 * SEMANTIC_OWNER:
 *
 *     distributed semantic analysis
 *     concurrency semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR pipeline
 *
 *     quantum::ir
 *     remains the quantum boundary
 *
 * TEST_OWNER:
 *
 *     grammar/tests/distributed/
 *     grammar/tests/concurrency/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/distributed.md
 *     grammar/spec/concurrency.md
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * There is exactly one distributed-actor composition boundary:
 *
 *     distributedActorConstruct
 *
 * Higher-level distributed grammar MUST consume this rule.
 *
 * This grammar must not expose a competing distributed actor root.
 *
 * ============================================================================
 */

parser grammar DistributedActors;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * PUBLIC DISTRIBUTED-ACTOR COMPOSITION
 * ============================================================================
 *
 * The distributed actor construct is deliberately a qualification around
 * the canonical actor declaration.
 *
 * Canonical source form:
 *
 *     distributed actor Counter {
 *         value: Int;
 *     }
 *
 * `actorDeclaration` is imported from:
 *
 *     grammar/concurrency/actors.g4
 *
 * and therefore owns the complete actor grammar.
 */
distributedActorConstruct
    : distributedActorDeclaration
    ;


/*
 * ============================================================================
 * DISTRIBUTED ACTOR DECLARATION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `distributedActorKeyword` is contextual rather than a dedicated lexer
 * token. The canonical lexer must therefore continue to emit the spelling
 * `distributed` as an identifier in this position.
 *
 * The second component is the canonical actor declaration.
 *
 * This guarantees:
 *
 *     distributed actor X { ... }
 *
 * and:
 *
 *     actor X { ... }
 *
 * share exactly the same actor body grammar.
 */
distributedActorDeclaration
    : distributedActorKeyword
      actorDeclaration
    ;


/*
 * ============================================================================
 * CONTEXTUAL DISTRIBUTED MARKER
 * ============================================================================
 *
 * This rule deliberately uses `identifier`.
 *
 * The semantic layer validates that its spelling is exactly:
 *
 *     distributed
 *
 * The grammar therefore does not require a new DISTRIBUTED lexer token.
 *
 * This is consistent with the repository's open-world distributed namespace
 * design and avoids lexical duplication.
 *
 * IMPORTANT:
 *
 * The canonical lexer must NOT reserve `distributed` as an unrelated
 * dedicated keyword if this contextual design is used.
 */
distributedActorKeyword
    : identifier
    ;


/*
 * ============================================================================
 * SEMANTIC / TOOLING ADAPTERS
 * ============================================================================
 *
 * These aliases do not create additional syntax.
 *
 * They give tooling a stable category while retaining one concrete grammar
 * owner.
 */
distributedActorDeclarationConstruct
    : distributedActorDeclaration
    ;


distributedActorSemanticConstruct
    : distributedActorConstruct
    ;


/*
 * ============================================================================
 * NO SECOND ACTOR GRAMMAR
 * ============================================================================
 *
 * The following are intentionally NOT defined here:
 *
 *     actor state
 *     actor handler
 *     actor message
 *     actor send
 *     actor ask
 *     actor forward
 *     actor spawn
 *     actor lifecycle
 *     actor supervision
 *     actor target
 *
 * All remain owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * ============================================================================
 * NO SECOND DISTRIBUTED COMMUNICATION GRAMMAR
 * ============================================================================
 *
 * This file intentionally does not define:
 *
 *     distributed send
 *     distributed receive
 *     distributed channel
 *     distributed endpoint
 *     distributed transport
 *     distributed route
 *
 * Those concepts belong to their existing owners.
 *
 * ============================================================================
 * NO SECOND RESOURCE GRAMMAR
 * ============================================================================
 *
 * This file intentionally does not define:
 *
 *     requires
 *     capability
 *     resource
 *     placement
 *     topology
 *     replication
 *     scaling
 *
 * Those remain owned by their existing resource/distributed grammars.
 *
 * ============================================================================
 * NO PHYSICAL REALIZATION
 * ============================================================================
 *
 * This grammar cannot select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     node
 *     process
 *     thread
 *     network address
 *     transport
 *     scheduler
 *     cloud provider
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Required PASS conditions:
 *
 *     no MAX_ACTORS
 *     no MAX_NODES
 *     no MAX_PROCESSES
 *     no MAX_WORKERS
 *     no MAX_THREADS
 *     no MAX_MESSAGES
 *     no MAX_REPLICAS
 *     no MAX_CHANNELS
 *     no MAX_CPUS
 *     no MAX_GPUS
 *     no MAX_FPGAS
 *     no MAX_ASICS
 *     no MAX_QPUS
 *     no MAX_MEMORY
 *     no MAX_DEVICES
 *     no fixed topology
 *     no physical identifiers
 *     no provider-specific realization
 *     no transport-specific realization
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The number of distributed actors is not restricted by this grammar.
 *
 * The grammar inherits the canonical actor repetition structure from:
 *
 *     grammar/concurrency/actors.g4
 *
 * Therefore actor members, handlers, state fields and nesting remain governed
 * by the canonical actor grammar rather than a distributed-specific limit.
 *
 * Practical limits are implementation/resource conditions only.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors:
 *
 *     malformed distributed marker
 *     missing actor declaration
 *     malformed canonical actor declaration
 *
 * Semantic errors:
 *
 *     marker is not the canonical distributed spelling
 *     distributed qualification is not permitted
 *     unavailable distributed capability
 *     unavailable resource
 *     invalid placement
 *     invalid replication
 *     invalid topology
 *     invalid policy
 *     unsupported target realization
 *
 * This grammar MUST NOT attempt semantic resource discovery.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing ordinary actor syntax remains unchanged because this grammar
 * delegates the actor body to `Actors.actorDeclaration`.
 *
 * Distributed actor syntax is intentionally:
 *
 *     distributed actor ...
 *
 * rather than creating:
 *
 *     distributedReceive
 *     distributedSend
 *     distributedAsk
 *     distributedForward
 *     distributedSpawn
 *
 * This prevents permanent duplication of actor commands.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     distributed actor Counter {
 *         value: Int;
 *     }
 *
 *     distributed actor Counter {
 *         value: Int;
 *
 *         actor receive increment(amount: Int) {
 *             value = value + amount;
 *         }
 *     }
 *
 *     distributed actor Counter extends BaseCounter {
 *         value: Int;
 *     }
 *
 * CROSS-DOMAIN:
 *
 *     distributed actor QuantumWorker {
 *         ...
 *     }
 *
 * where the body contains otherwise-valid quantum syntax.
 *
 *     distributed actor TensorWorker {
 *         ...
 *     }
 *
 * where the body contains otherwise-valid tensor/AI syntax.
 *
 *     distributed actor HardwareWorker {
 *         ...
 *     }
 *
 * where the body contains otherwise-valid hardware/HDL intent.
 *
 * NEGATIVE:
 *
 *     distributed
 *
 *     distributed {
 *     }
 *
 *     distributed actor
 *
 *     distributed actor Counter
 *
 *     distributed actor Counter {
 *
 * where the underlying actor grammar requires additional structure.
 *
 * SEMANTIC NEGATIVES:
 *
 *     distributed actor whose distributed requirements cannot be satisfied
 *
 * must be rejected downstream, not by this parser.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Tests must parameterize:
 *
 *     number of distributed actor declarations
 *     number of actor members
 *     number of handlers
 *     state fields
 *     nesting depth
 *     generic complexity
 *     message relationships
 *
 * No test may define a universal maximum.
 *
 * A stress test may select a finite value for the test environment, but that
 * value is a test parameter and MUST NOT become a language constant.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     token stream
 *     grammar version
 *     parser configuration
 *
 * parsing must be deterministic.
 *
 * ============================================================================
 * ROUND-TRIP TESTS
 * ============================================================================
 *
 * Where formatting support exists:
 *
 *     source
 *       ->
 *     parse
 *       ->
 *     AST
 *       ->
 *     format
 *       ->
 *     parse
 *
 * must preserve:
 *
 *     distributed qualification
 *     actor declaration
 *     actor name
 *     actor members
 *     actor state
 *     handlers
 *     actor commands
 *     nested expressions
 *     source semantics
 *
 * ============================================================================
 * CROSS-FILE INTEGRATION
 * ============================================================================
 *
 * DISTRIBUTED COMPOSITION
 * -----------------------
 *
 *     grammar/distributed/distributed.g4
 *
 * MUST import:
 *
 *     DistributedActors
 *
 * and consume:
 *
 *     distributedActorConstruct
 *
 * It must not define another actor alternative.
 *
 *
 * CONCURRENCY
 * -----------
 *
 *     grammar/concurrency/actors.g4
 *
 * remains the sole actor grammar.
 *
 * This file imports and reuses:
 *
 *     actorDeclaration
 *
 * It must not modify actor semantics merely because an actor is distributed.
 *
 *
 * CONCURRENCY COMPOSITION
 * -----------------------
 *
 *     grammar/concurrency/concurrency.g4
 *
 * continues to consume:
 *
 *     actorConstruct
 *
 * for ordinary actor syntax.
 *
 * It does not need a second distributed actor implementation.
 *
 *
 * DISTRIBUTED PLACEMENT
 * ---------------------
 *
 * Distributed placement remains in:
 *
 *     grammar/distributed/placement.g4
 *
 * and its canonical resource placement dependencies.
 *
 *
 * DISTRIBUTED REPLICATION
 * -----------------------
 *
 * Replication remains in:
 *
 *     grammar/distributed/replication.g4
 *
 *
 * DISTRIBUTED COMMUNICATION
 * -------------------------
 *
 * Communication remains in:
 *
 *     grammar/distributed/communication.g4
 *     grammar/distributed/messaging.g4
 *     grammar/concurrency/channels.g4
 *
 *
 * NETWORKING
 * ----------
 *
 * Network transport and endpoint realization remain in:
 *
 *     grammar/networking/
 *
 *
 * RESOURCES
 * ---------
 *
 * Resource/capability requirements remain in:
 *
 *     grammar/resources/
 *
 *
 * EFFECTS
 * -------
 *
 * Effects remain in:
 *
 *     grammar/effects/
 *
 *
 * SECURITY
 * --------
 *
 * Security/capability/authorization remains in:
 *
 *     grammar/security/
 *
 *
 * QUANTUM
 * -------
 *
 * Quantum operations remain in:
 *
 *     grammar/quantum/
 *
 * and lower through:
 *
 *     quantum::ir
 *
 *
 * HDL / HARDWARE
 * -------------
 *
 * HDL and hardware realization remain in:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 *
 * AST
 * ---
 *
 * No new distributed-actor-specific runtime AST is required.
 *
 * The existing actor AST representation is reused, with distributed
 * qualification preserved by the frontend's domain-neutral semantic metadata.
 *
 *
 * IR
 * --
 *
 * No DistributedActorIR is introduced.
 *
 * No ActorNetworkIR is introduced.
 *
 * No DistributedQuantumIR is introduced.
 *
 * The canonical semantic/IR pipeline remains authoritative.
 *
 * ============================================================================
 * RUST / IMPLEMENTATION SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * The generated Zamani frontend must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust Edition 2021
 *
 * and must use safe Rust only.
 *
 * No `unsafe` implementation is required or permitted by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It has exactly one distributed-actor concrete syntax boundary.
 *
 * [x] It delegates ordinary actor syntax to `concurrency/actors.g4`.
 *
 * [x] It does not duplicate actor state syntax.
 *
 * [x] It does not duplicate actor handler syntax.
 *
 * [x] It does not duplicate actor messaging syntax.
 *
 * [x] It does not duplicate actor lifecycle syntax.
 *
 * [x] It does not duplicate actor supervision syntax.
 *
 * [x] It does not define distributed transport syntax.
 *
 * [x] It does not define physical placement.
 *
 * [x] It does not define resource allocation.
 *
 * [x] It does not define scheduling.
 *
 * [x] It does not define replication algorithms.
 *
 * [x] It does not define networking protocols.
 *
 * [x] It does not define quantum operations.
 *
 * [x] It does not define an actor-specific IR.
 *
 * [x] It does not define a distributed-specific quantum IR.
 *
 * [x] It has no universal resource limits.
 *
 * [x] It has no physical hardware identifiers.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It contains no semantic predicates.
 *
 * [x] It contains no runtime callbacks.
 *
 * [x] It remains deterministic.
 *
 * [ ] Repository lexer confirms `distributed` remains an identifier in the
 *     contextual position.
 *
 * [ ] `Distributed.distributedDeclaration` consumes
 *     `distributedActorConstruct`.
 *
 * [ ] Frontend AST preserves the distributed qualification.
 *
 * [ ] Semantic analysis recognizes distributed actor qualification.
 *
 * [ ] Positive, negative, boundary, scalability and cross-domain tests pass.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 *     ordinary actor:
 *
 *         actorDeclaration
 *
 *     distributed actor:
 *
 *         distributedActorKeyword
 *             +
 *         actorDeclaration
 *
 * Therefore:
 *
 *     distributed actor
 *         !=
 *     second actor language
 *
 * and:
 *
 *     distributed actor
 *         =
 *     canonical actor
 *         +
 *     distributed semantic qualification
 *
 * This is the required production architecture for POCO-REAF.
 *
 * ============================================================================
 */