/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/distributed.g4
 *
 * Role:
 *     Canonical modular parser grammar for distributed-computing effect
 *     references and distributed-effect syntax.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime callbacks.
 *     - No unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns ONLY the SYNTAX that identifies distributed-computing
 * effects as a domain-specific namespace.
 *
 * Examples:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::consensus
 *     distributed::replication
 *     distributed::coordination
 *     distributed::remote_execution
 *     distributed::service
 *     distributed::stream
 *     distributed::future::operation
 *
 * The set of distributed effects is OPEN-WORLD.
 *
 * This file therefore deliberately does NOT enumerate a finite catalogue of
 * distributed effects.
 *
 * New distributed effects must be representable without modifying this file.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical name grammar
 *          |
 *          v
 *     effects/distributed.g4       <-- THIS FILE
 *          |
 *          v
 *     frontend AST
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     effect analysis              capability analysis
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                 semantic representation
 *                          |
 *          +---------------+------------------+
 *          |               |                  |
 *          v               v                  v
 *     classical IR    quantum::ir       distributed IR/model
 *                                          |
 *                                          v
 *                              scheduling / routing /
 *                              resilience / runtime
 *
 * This file is syntax-only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed-effect namespace recognition;
 *     - distributed-effect symbolic references;
 *     - distributed-effect member syntax;
 *     - distributed-effect reference wrappers;
 *     - distributed-effect reference lists;
 *     - optional distributed-effect references;
 *     - syntactic distinction between generic effects and distributed effects.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - effect sets;
 *     - generic effect references;
 *     - effect declarations;
 *     - effect handlers;
 *     - effect invocation;
 *     - distributed nodes;
 *     - distributed services;
 *     - network endpoints;
 *     - network protocols;
 *     - messages;
 *     - channels;
 *     - placement;
 *     - replication;
 *     - consistency;
 *     - consensus algorithms;
 *     - fault tolerance;
 *     - distributed scheduling;
 *     - distributed routing;
 *     - distributed runtime execution;
 *     - resource allocation;
 *     - capability discovery;
 *     - hardware discovery;
 *     - quantum semantics;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - optimization.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * Generic effect syntax belongs to:
 *
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effects.g4
 *
 * Canonical names belong to:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * Distributed computation constructs belong to:
 *
 *     grammar/distributed/
 *
 * This file therefore provides DOMAIN-SPECIFIC EFFECT SYNTAX only.
 *
 * It must never become a second distributed-programming grammar.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Distributed effect names are intentionally not enumerated.
 *
 * Valid examples include:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::multicast
 *     distributed::consensus
 *     distributed::replication
 *     distributed::coordination
 *     distributed::migration
 *     distributed::remote_execution
 *     distributed::service
 *     distributed::stream
 *     distributed::checkpoint
 *     distributed::recovery
 *
 * Future extensions may use:
 *
 *     distributed::future::operation
 *     distributed::vendor::operation
 *     distributed::domain::subdomain::operation
 *
 * The grammar does not need to know the meaning of these names.
 *
 * Semantic registration/resolution determines whether a referenced effect
 * exists and what it means.
 *
 * ============================================================================
 * IMPORTANT: NAMESPACE VS SEMANTICS
 * ============================================================================
 *
 * The prefix:
 *
 *     distributed::
 *
 * is a source-level namespace.
 *
 * It does NOT mean:
 *
 *     - a fixed number of machines;
 *     - a cluster;
 *     - a particular network;
 *     - a particular transport protocol;
 *     - a particular node;
 *     - a particular cloud provider;
 *     - a particular topology;
 *     - a particular scheduler;
 *     - a particular distributed algorithm.
 *
 * For example:
 *
 *     distributed::send
 *
 * expresses distributed communication intent.
 *
 * It does NOT select:
 *
 *     TCP
 *     UDP
 *     MPI
 *     RDMA
 *     Ethernet
 *     InfiniBand
 *     a cloud provider
 *     a particular node
 *     a particular address
 *
 * Such decisions belong to downstream capability/resource/target/runtime
 * systems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed effects must remain portable across:
 *
 *     - one machine;
 *     - multiple processes;
 *     - multiple threads;
 *     - multiple devices;
 *     - multiple nodes;
 *     - clusters;
 *     - HPC systems;
 *     - edge systems;
 *     - cloud systems;
 *     - heterogeneous systems;
 *     - quantum-classical distributed systems;
 *     - future distributed architectures.
 *
 * Nothing here imposes a source-level maximum on:
 *
 *     - nodes;
 *     - processes;
 *     - services;
 *     - channels;
 *     - messages;
 *     - endpoints;
 *     - effect references;
 *     - namespace segments;
 *     - effect operations;
 *     - communication relationships.
 *
 * There is deliberately no:
 *
 *     MAX_NODES
 *     MAX_SERVICES
 *     MAX_ENDPOINTS
 *     MAX_MESSAGES
 *     MAX_CHANNELS
 *     MAX_EFFECTS
 *     MAX_DISTRIBUTED_DEPTH
 *
 * or equivalent grammar constant.
 *
 * ============================================================================
 * RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * The following distinctions MUST remain explicit:
 *
 *     effect
 *         !=
 *     capability
 *         !=
 *     resource
 *         !=
 *     constraint
 *         !=
 *     preference
 *         !=
 *     placement
 *         !=
 *     deployment
 *
 * For example:
 *
 *     distributed::consensus
 *
 * does not imply:
 *
 *     N nodes
 *
 * and:
 *
 *     distributed::send
 *
 * does not imply:
 *
 *     node 0 -> node 1
 *
 * Physical or runtime facts are discovered or selected later.
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * This grammar MUST reuse:
 *
 *     identifier
 *     qualifiedName
 *
 * from the canonical name grammar.
 *
 * It MUST NOT define:
 *
 *     IDENTIFIER
 *     distributedIdentifier
 *     distributedName
 *     distributedQualifiedName
 *
 * as independent lexical/name systems.
 *
 * This prevents distributed syntax from drifting away from the language-wide
 * naming rules.
 *
 * ============================================================================
 * SEMANTIC VALIDATION OF THE DISTRIBUTED PREFIX
 * ============================================================================
 *
 * `identifier` is intentionally reused for the first namespace segment.
 *
 * Therefore this grammar accepts the structural form:
 *
 *     identifier :: identifier ...
 *
 * and semantic validation MUST verify that the first segment is exactly:
 *
 *     distributed
 *
 * This is deliberate.
 *
 * The lexer currently owns identifier/keyword classification and the
 * repository's open namespace model treats domain names as ordinary symbolic
 * names rather than hard-coded lexical keyword inventories.
 *
 * Consequently this grammar must not invent a `DISTRIBUTED` keyword token.
 *
 * ============================================================================
 * DISTRIBUTED EFFECT PATH
 * ============================================================================
 *
 * A distributed effect reference has the structural form:
 *
 *     distributed::<member>
 *
 * and may be arbitrarily extended:
 *
 *     distributed::<member>::<member>
 *
 * Examples:
 *
 *     distributed::send
 *     distributed::network::send
 *     distributed::consensus::proposal
 *     distributed::consensus::commit
 *     distributed::future::domain::operation
 *
 * The grammar imposes no maximum qualification depth.
 *
 * ============================================================================
 * GENERIC EFFECT INTEGRATION
 * ============================================================================
 *
 * Generic effect syntax remains owned by:
 *
 *     effects/effect-sets.g4
 *
 * Therefore this file does NOT define:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     optionalEffectSet
 *
 * A generic effect such as:
 *
 *     distributed::send
 *
 * remains a normal qualified effect reference in the generic effect grammar.
 *
 * This file provides the narrower domain-specific parse-tree boundary for
 * consumers that need to distinguish distributed effects before semantic
 * lowering.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     - complete source spelling;
 *     - ordered namespace segments;
 *     - source spans;
 *     - the fact that the reference is in the distributed namespace;
 *     - the complete symbolic path.
 *
 * The AST MUST NOT resolve:
 *
 *     node IDs;
 *     service IDs;
 *     endpoint addresses;
 *     network routes;
 *     deployment targets;
 *     runtime resources.
 *
 * Those belong to semantic analysis and downstream systems.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT create an IR.
 *
 * A distributed effect reference may eventually lower into canonical semantic
 * effect metadata and, where applicable, distributed execution IR.
 *
 * The grammar must never:
 *
 *     - construct a node;
 *     - allocate a channel;
 *     - create a network link;
 *     - select a placement;
 *     - schedule an operation;
 *     - create a resource;
 *     - mutate distributed state.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed effects may coexist with quantum effects.
 *
 * Examples:
 *
 *     distributed::send
 *     quantum::Measurement
 *
 * or semantically:
 *
 *     distributed::remote_quantum_execution
 *
 * This grammar does not import or depend on:
 *
 *     quantum::ir
 *     quantum gates
 *     QEC
 *     ZQN
 *
 * If distributed execution carries quantum computation, semantic lowering
 * determines how the distributed effect interacts with quantum::ir.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Distributed effects may eventually be observed by the resilience subsystem.
 *
 * Examples include:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::consensus
 *     distributed::replication
 *     distributed::recovery
 *
 * However, this grammar does NOT implement resilience.
 *
 * Resilience determines how to react to execution failures or changing
 * conditions after semantic lowering.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * A distributed effect may create scheduling requirements.
 *
 * Example:
 *
 *     distributed::send
 *
 * may imply communication work.
 *
 * This grammar does not decide:
 *
 *     - when communication occurs;
 *     - which link is used;
 *     - what latency exists;
 *     - what ordering is required;
 *     - how operations are scheduled.
 *
 * Scheduling consumes canonical semantic information downstream.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Distributed effects may interact with:
 *
 *     grammar/networking/
 *
 * but this file does not define network protocols.
 *
 * Therefore:
 *
 *     distributed::send
 *
 * does not imply:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *
 * Protocol selection belongs to capability/target/runtime layers.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Distributed effects may execute on:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     embedded devices
 *     heterogeneous devices
 *
 * This grammar must remain unaware of physical hardware.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no runtime calls;
 *     - no I/O;
 *     - no network operations;
 *     - no hardware discovery;
 *     - no randomness.
 *
 * Given the same deterministic token stream, the parse result is deterministic.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Adding a new distributed effect name MUST NOT require modifying this file.
 *
 * For example, adding:
 *
 *     distributed::federation
 *
 * requires semantic registration, not grammar modification.
 *
 * Grammar changes are required only if Zamani changes the syntax of
 * distributed-effect references themselves.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Effect vocabulary versioning belongs to:
 *
 *     semantic effect registry
 *     compatibility layer
 *     dialect/version infrastructure
 *
 * This grammar must not contain a finite versioned list of distributed effect
 * names.
 *
 * ============================================================================
 * TESTING CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::consensus
 *     distributed::replication
 *     distributed::network::send
 *     distributed::consensus::proposal
 *     distributed::future::domain::operation
 *     distributed::vendor::extension::effect
 *
 * Negative:
 *
 *     distributed
 *     distributed::
 *     ::send
 *     distributed:: 
 *     distributed::::send
 *     distributed::send::
 *
 * Semantic-negative:
 *
 *     quantum::Measurement
 *
 * must NOT be classified as a distributed effect.
 *
 *     hardware::device
 *
 * must NOT be classified as a distributed effect.
 *
 *     arbitrary::effect
 *
 * must parse as a generic qualified effect but must not be classified as
 * distributed merely because its syntax resembles a distributed reference.
 *
 * Boundary:
 *
 *     distributed::<very-large-number-of-segments>
 *
 * must have no grammar-defined segment-count ceiling.
 *
 * Large source programs must likewise not encounter a language-level
 * distributed-effect cardinality limit.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_NODES
 *     MAX_SERVICES
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_EFFECTS
 *     MAX_NAMESPACE_DEPTH
 *     MAX_DISTRIBUTED_DEPTH
 *     NODE_0
 *     NODE_1
 *     DEVICE_0
 *     DEVICE_1
 *     fixed network addresses
 *     fixed transport protocols
 *     fixed cluster sizes
 *     fixed topology
 *     fixed cloud provider
 *
 * None of these concepts are represented by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] It compiles as an ANTLR4 parser grammar.
 *     [ ] It uses ZamaniLexer.
 *     [ ] It imports the canonical naming grammar.
 *     [ ] It does not redefine identifier syntax.
 *     [ ] It does not redefine generic effect-set syntax.
 *     [ ] It recognizes distributed-qualified effect references.
 *     [ ] It supports arbitrary namespace depth.
 *     [ ] It has no machine-size limits.
 *     [ ] It has no hardware-specific assumptions.
 *     [ ] It has no runtime actions.
 *     [ ] It has no unsafe code.
 *     [ ] It preserves source spans through the parse tree.
 *     [ ] It has positive tests.
 *     [ ] It has negative tests.
 *     [ ] It has boundary/scalability tests.
 *     [ ] It has cross-domain tests.
 *     [ ] It is imported by the canonical effects aggregate.
 *     [ ] It does not create a circular grammar dependency.
 *     [ ] Its AST integration is documented.
 *     [ ] Its semantic integration is documented.
 *     [ ] Its distributed runtime integration is downstream-only.
 *
 * ============================================================================
 */

parser grammar DistributedEffects;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Names owns:
 *
 *     identifier
 *     qualifiedName
 *
 * No local copy of either rule is permitted.
 */
import Names;


/*
 * ============================================================================
 * 1. DISTRIBUTED EFFECT REFERENCE
 * ============================================================================
 *
 * Canonical domain-specific reference.
 *
 * Examples:
 *
 *     distributed::send
 *     distributed::consensus
 *     distributed::network::send
 *
 * The first segment is structurally an identifier. Semantic analysis verifies
 * that its canonical spelling is `distributed`.
 */
distributedEffectReference
    : distributedEffectPath
    ;


/*
 * ============================================================================
 * 2. DISTRIBUTED EFFECT PATH
 * ============================================================================
 *
 * The path contains:
 *
 *     distributed
 *     ::
 *     one or more effect members
 *
 * No finite number of members is permitted by the grammar.
 */
distributedEffectPath
    : identifier
      DOUBLE_COLON
      distributedEffectMember
      (
          DOUBLE_COLON
          distributedEffectMember
      )*
    ;


/*
 * ============================================================================
 * 3. DISTRIBUTED EFFECT MEMBER
 * ============================================================================
 *
 * Effect members are ordinary Zamani identifiers.
 *
 * The meaning of the member is resolved later.
 *
 * Examples:
 *
 *     send
 *     receive
 *     consensus
 *     replication
 *     network
 *     future
 *     vendor
 */
distributedEffectMember
    : identifier
    ;


/*
 * ============================================================================
 * 4. DISTRIBUTED EFFECT REFERENCE LIST
 * ============================================================================
 *
 * This is a domain-specific list.
 *
 * It is NOT the canonical generic effectReferenceList.
 *
 * It exists only for consumers that explicitly require a list whose members
 * are all syntactically represented as distributed-effect references.
 *
 * Generic effect-set syntax remains owned by effect-sets.g4.
 */
distributedEffectReferenceList
    : distributedEffectReference
      (
          COMMA
          distributedEffectReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 5. OPTIONAL DISTRIBUTED EFFECT REFERENCE
 * ============================================================================
 */
optionalDistributedEffectReference
    : distributedEffectReference?
    ;


/*
 * ============================================================================
 * 6. OPTIONAL DISTRIBUTED EFFECT REFERENCE LIST
 * ============================================================================
 */
optionalDistributedEffectReferenceList
    : distributedEffectReferenceList?
    ;


/*
 * ============================================================================
 * 7. DISTRIBUTED EFFECT MEMBER LIST
 * ============================================================================
 *
 * Useful to semantic/tooling consumers that need to inspect the namespace
 * after the `distributed` prefix.
 *
 * Example:
 *
 *     distributed::consensus::proposal::commit
 *
 * produces the member sequence:
 *
 *     consensus
 *     proposal
 *     commit
 *
 * No finite member count is imposed.
 */
distributedEffectMemberList
    : distributedEffectMember
      (
          DOUBLE_COLON
          distributedEffectMember
      )*
    ;


/*
 * ============================================================================
 * 8. DISTRIBUTED EFFECT PATH AFTER NAMESPACE
 * ============================================================================
 *
 * Integration rule for consumers that have already consumed:
 *
 *     distributed::
 *
 * Example:
 *
 *     distributed::consensus::proposal
 *
 * after consuming `distributed::`, this rule consumes:
 *
 *     consensus::proposal
 */
distributedEffectMemberPath
    : distributedEffectMemberList
    ;


/*
 * ============================================================================
 * 9. DISTRIBUTED EFFECT NAMESPACE
 * ============================================================================
 *
 * This rule gives semantic tooling a named boundary for the domain namespace.
 *
 * The spelling is intentionally represented by the canonical identifier rule.
 *
 * Semantic analysis MUST verify:
 *
 *     identifier text == "distributed"
 *
 * No dedicated DISTRIBUTED lexer keyword is introduced.
 */
distributedEffectNamespace
    : identifier
    ;


/*
 * ============================================================================
 * 10. COMPLETE DISTRIBUTED EFFECT QUALIFIED REFERENCE
 * ============================================================================
 *
 * Explicit decomposition:
 *
 *     namespace
 *     ::
 *     member path
 *
 * This rule is useful to AST builders that need the namespace and member path
 * as separate parse-tree nodes.
 */
distributedEffectQualifiedReference
    : distributedEffectNamespace
      DOUBLE_COLON
      distributedEffectMemberPath
    ;


/*
 * ============================================================================
 * 11. DISTRIBUTED EFFECT REFERENCE ALIAS
 * ============================================================================
 *
 * Alias syntax belongs structurally to the canonical name system, but this
 * wrapper gives distributed-effect tooling an explicit integration point.
 *
 * Example:
 *
 *     distributed::consensus as consensus
 *
 * The alias itself remains an ordinary identifier.
 */
distributedEffectAlias
    : distributedEffectReference
      AS
      identifier
    ;


/*
 * ============================================================================
 * 12. DISTRIBUTED EFFECT REFERENCE OR ALIAS
 * ============================================================================
 */
distributedEffectReferenceOrAlias
    : distributedEffectReference
    | distributedEffectAlias
    ;


/*
 * ============================================================================
 * 13. DISTRIBUTED EFFECT REFERENCE OR ALIAS LIST
 * ============================================================================
 */
distributedEffectReferenceOrAliasList
    : distributedEffectReferenceOrAlias
      (
          COMMA
          distributedEffectReferenceOrAlias
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 14. OPTIONAL DISTRIBUTED EFFECT REFERENCE OR ALIAS LIST
 * ============================================================================
 */
optionalDistributedEffectReferenceOrAliasList
    : distributedEffectReferenceOrAliasList?
    ;