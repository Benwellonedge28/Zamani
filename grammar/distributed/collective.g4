/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/collective.g4
 *
 * Grammar:
 *     Collective
 *
 * Status:
 *     Production distributed collective-computation grammar
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Rust integration:
 *     Rust 1.97+
 *     Edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the source-level structural syntax for collective
 * computation.
 *
 * A collective is a target-independent computational intent involving a
 * logically defined participant set.
 *
 * The grammar deliberately does NOT enumerate collective algorithms.
 *
 * Therefore names such as:
 *
 *     broadcast
 *     scatter
 *     gather
 *     reduce
 *     all_reduce
 *     all_gather
 *     all_to_all
 *     scan
 *     barrier
 *     exchange
 *     custom.collective
 *     vendor.collective
 *     future.collective
 *
 * are ordinary semantic operation names.
 *
 * Adding a new collective operation MUST NOT require changing this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * This grammar describes:
 *
 *     WHAT collective computation is requested.
 *
 * It does NOT describe:
 *
 *     WHERE it executes;
 *     WHICH machine executes it;
 *     WHICH process executes it;
 *     WHICH CPU/GPU/FPGA/ASIC/QPU executes it;
 *     WHICH transport is used;
 *     WHICH network is used;
 *     WHICH physical topology is used;
 *     WHICH routing algorithm is used;
 *     WHICH scheduler is used;
 *     WHICH implementation algorithm is used;
 *     WHICH resources are allocated.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     collectiveConstruct
 *     collectiveDeclaration
 *     collectiveInvocation
 *     collectiveBody
 *     collectiveMember
 *     collectiveProperty
 *     collectiveSection
 *     collectiveRequirementClause
 *     collectiveContractClause
 *     collectivePolicyClause
 *     collectiveCapabilityClause
 *     collectiveResourceClause
 *     collectiveEffectClause
 *     collectiveProvenanceClause
 *     collectiveEvidenceClause
 *     collectiveAttributeAttachment
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     expression precedence
 *     attributes
 *     types
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     contracts
 *     provenance semantics
 *     messages
 *     channels
 *     actors
 *     services
 *     network protocols
 *     topology realization
 *     placement
 *     routing
 *     scheduling
 *     replication
 *     consistency algorithms
 *     fault-tolerance implementation
 *     quantum operations
 *     physical qubits
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical parser dependencies:
 *
 *     Names
 *     Expressions
 *     Attributes
 *
 * Canonical lexer:
 *
 *     ZamaniLexer
 *
 * This grammar does not define lexer rules.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * The only public distributed entry point owned here is:
 *
 *     collectiveConstruct
 *
 * The parent distributed grammar consumes that rule.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Collective operation names are qualified names.
 *
 * The grammar does not contain alternatives such as:
 *
 *     broadcast
 *     scatter
 *     gather
 *     reduce
 *     barrier
 *     all_reduce
 *
 * as syntax.
 *
 * This keeps the collective language open to:
 *
 *     standard operations
 *     user-defined operations
 *     library operations
 *     dialect operations
 *     vendor operations
 *     experimental operations
 *     future operations
 *
 * Semantic analysis determines whether an operation is known, supported,
 * deprecated, authorized, or realizable.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No language-level capacity is encoded.
 *
 * This grammar contains no limits for:
 *
 *     participants
 *     groups
 *     processes
 *     nodes
 *     workers
 *     replicas
 *     values
 *     arguments
 *     properties
 *     nested sections
 *     collectives
 *     resources
 *     memory
 *     bandwidth
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     network size
 *     topology size
 *
 * ANTLR repetition is used where collections are structurally unbounded.
 *
 * Actual limits are implementation/resource constraints and must remain
 * outside language semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A collective source program expresses logical intent.
 *
 * The same source may be considered for realization on:
 *
 *     tiny systems
 *     embedded systems
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
 * without changing the collective grammar merely because target capacity
 * changes.
 *
 * ============================================================================
 * SYNTAX MODEL
 * ============================================================================
 *
 * Declaration/block form:
 *
 *     collective broadcast {
 *         participants: workers;
 *         value: data;
 *     }
 *
 * Invocation form:
 *
 *     collective broadcast(participants, data);
 *
 * Operation names are semantic data.
 *
 * ============================================================================
 * ATTRIBUTE MODEL
 * ============================================================================
 *
 * Generic attributes are owned by:
 *
 *     grammar/core/attributes.g4
 *
 * This grammar reuses:
 *
 *     optionalAttributes
 *
 * It does not recreate attribute syntax.
 *
 * ============================================================================
 * EXPRESSION MODEL
 * ============================================================================
 *
 * All values, participant sets, topology requirements, reducers, policies,
 * resource expressions, capability expressions and other semantic values use
 * the canonical `expression` grammar.
 *
 * This grammar does not create a second expression language.
 *
 * ============================================================================
 */

parser grammar Collective;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions,
    Attributes
;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable integration boundary consumed by:
 *
 *     grammar/distributed/distributed.g4
 *
 * and other grammars that explicitly compose distributed collective syntax.
 */
collectiveConstruct
    : collectiveDeclaration
    | collectiveInvocation
    ;


/*
 * ============================================================================
 * 2. COLLECTIVE DECLARATION
 * ============================================================================
 *
 * Block form:
 *
 *     collective broadcast {
 *         participants: workers;
 *         value: data;
 *     }
 *
 * The operation name is a qualified name and is therefore open-world.
 *
 * The body is mandatory.
 *
 * This makes declaration syntax structurally distinct from invocation syntax:
 *
 *     declaration -> { ... }
 *     invocation  -> ( ... ) ;
 */
collectiveDeclaration
    : COLLECTIVE
      collectiveOperationName
      collectiveAttributeAttachment?
      collectiveBody
    ;


/*
 * ============================================================================
 * 3. COLLECTIVE INVOCATION
 * ============================================================================
 *
 * Invocation:
 *
 *     collective broadcast(participants, data);
 *
 * An empty argument list is valid because some collective operations may have
 * no explicit source arguments and obtain their semantic inputs from context.
 */
collectiveInvocation
    : COLLECTIVE
      collectiveOperationName
      collectiveAttributeAttachment?
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. OPERATION NAME
 * ============================================================================
 *
 * Qualified names remain open-world:
 *
 *     broadcast
 *     distributed::broadcast
 *     custom::reduce
 *     vendor::collective
 *     future::collective::operation
 *
 * The semantic layer resolves meaning.
 */
collectiveOperationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 5. ATTRIBUTE ATTACHMENT
 * ============================================================================
 *
 * Delegates completely to the canonical attribute grammar.
 */
collectiveAttributeAttachment
    : optionalAttributes
    ;


/*
 * ============================================================================
 * 6. COLLECTIVE BODY
 * ============================================================================
 */
collectiveBody
    : LBRACE
      collectiveMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. COLLECTIVE MEMBER
 * ============================================================================
 *
 * Each member has a structurally distinguishable first token:
 *
 *     identifier -> property or nested section
 *     REQUIRES   -> requirement
 *     CONTRACT   -> contract
 *     REQUIRES/ENSURES/... -> contract item when inside contract
 *     POLICY     -> policy
 *     CAPABILITY -> capability
 *     RESOURCE   -> resource
 *     EFFECT/EFFECTS -> effect
 *     PROVENANCE -> provenance
 *     EVIDENCE   -> evidence
 *
 * This avoids the former generic "identifier identifier" ambiguity.
 */
collectiveMember
    : collectiveProperty
    | collectiveSection
    | collectiveRequirementClause
    | collectiveContractClause
    | collectivePolicyClause
    | collectiveCapabilityClause
    | collectiveResourceClause
    | collectiveEffectClause
    | collectiveProvenanceClause
    | collectiveEvidenceClause
    ;


/*
 * ============================================================================
 * 8. NAMED PROPERTY
 * ============================================================================
 *
 * Examples:
 *
 *     participants: workers;
 *     value: data;
 *     result: output;
 *     reducer: combine;
 *     root: coordinator;
 *     topology: required_topology;
 *
 * Property names are identifiers, not a closed keyword catalogue.
 *
 * Therefore future semantic properties do not require grammar changes.
 */
collectiveProperty
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. NESTED NAMED SECTION
 * ============================================================================
 *
 * Examples:
 *
 *     topology {
 *         preference: hierarchical;
 *     }
 *
 *     metadata {
 *         source: model;
 *     }
 *
 *     implementation {
 *         strategy: custom;
 *     }
 *
 * The section name is semantic data.
 */
collectiveSection
    : identifier
      collectiveBody
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability("distributed.collective");
 *     requires capability("collective.reduce");
 *     requires memory >= required_memory;
 *     requires bandwidth >= required_bandwidth;
 *     requires topology(required_topology);
 *
 * The expression is not evaluated by the parser.
 *
 * Resource and capability resolution are downstream responsibilities.
 */
collectiveRequirementClause
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CONTRACTS
 * ============================================================================
 *
 * A collective may carry local correctness contracts.
 *
 * Examples:
 *
 *     requires ...
 *     ensures ...
 *     invariant ...
 *     assume ...
 *     guarantee ...
 *     property ...
 *
 * A contract block keeps contract membership structurally isolated from
 * ordinary collective properties.
 */
collectiveContractClause
    : CONTRACT
      LBRACE
      collectiveContractItem*
      RBRACE
    | collectiveContractItem
    ;


collectiveContractItem
    : REQUIRES
      expression
      SEMICOLON
    | ENSURES
      expression
      SEMICOLON
    | INVARIANT
      expression
      SEMICOLON
    | ASSUME
      expression
      SEMICOLON
    | GUARANTEE
      expression
      SEMICOLON
    | PROPERTY
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. POLICIES
 * ============================================================================
 *
 * Policies govern allowed/preferred execution behavior.
 *
 * They do not select a physical target.
 *
 * Block form:
 *
 *     policy {
 *         allow capability("collective.communication");
 *         forbid effect("network");
 *         prefer topology(preferred_topology);
 *     }
 *
 * Compact form:
 *
 *     policy policy_reference;
 *
 * Semantic policy interpretation belongs downstream.
 */
collectivePolicyClause
    : POLICY
      LBRACE
      collectivePolicyItem*
      RBRACE
    | POLICY
      expression
      SEMICOLON
    | collectivePolicyItem
    ;


collectivePolicyItem
    : ALLOW
      expression
      SEMICOLON
    | FORBID
      expression
      SEMICOLON
    | PERMIT
      expression
      SEMICOLON
    | DENY
      expression
      SEMICOLON
    | PREFER
      expression
      SEMICOLON
    | FALLBACK
      expression
      SEMICOLON
    | RETRY
      expression
      SEMICOLON
    | RECOVER
      expression
      SEMICOLON
    | ESCALATE
      expression
      SEMICOLON
    | REJECT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. CAPABILITIES
 * ============================================================================
 *
 * Capabilities describe realizable computational abilities.
 *
 * Examples:
 *
 *     capability("collective.communication");
 *     capability("collective.reduce");
 *     capability("network.multicast");
 *
 * The capability identity is an expression and is not enumerated here.
 */
collectiveCapabilityClause
    : CAPABILITY
      expression
      SEMICOLON
    | CAPABILITIES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. RESOURCES
 * ============================================================================
 *
 * Resources describe source-level requirements or resource intent.
 *
 * Examples:
 *
 *     resource memory >= required_memory;
 *     resource bandwidth >= required_bandwidth;
 *     resource participants;
 *
 * This grammar never translates such expressions into fixed capacities.
 */
collectiveResourceClause
    : RESOURCE
      expression
      SEMICOLON
    | RESOURCES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. EFFECTS
 * ============================================================================
 *
 * Effects describe observable computational behavior.
 *
 * Examples may refer to:
 *
 *     distributed communication
 *     networking
 *     mutation
 *     measurement
 *     foreign execution
 *     simulation
 *     learning
 *     adaptation
 *
 * The effect system remains the authoritative semantic owner.
 */
collectiveEffectClause
    : EFFECT
      expression
      SEMICOLON
    | EFFECTS
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. PROVENANCE
 * ============================================================================
 *
 * Provenance records source-level origin/derivation intent.
 *
 * The parser only preserves structure.
 */
collectiveProvenanceClause
    : PROVENANCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. EVIDENCE
 * ============================================================================
 *
 * Evidence can be attached to a collective requirement, contract, policy or
 * decision without introducing an AI-specific grammar.
 */
collectiveEvidenceClause
    : EVIDENCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should represent:
 *
 *     CollectiveOperation
 *         operation_name
 *         attributes
 *         form
 *         members
 *         arguments
 *         source_span
 *
 * where:
 *
 *     form = declaration | invocation
 *
 * Members should retain source order.
 *
 * The AST must preserve:
 *
 *     operation-name segments
 *     argument ordering
 *     property ordering
 *     section nesting
 *     contract ordering
 *     policy ordering
 *     requirement ordering
 *     capability expressions
 *     resource expressions
 *     effect expressions
 *     provenance expressions
 *     evidence expressions
 *     source spans
 *
 * The grammar does not create physical participant IDs or resource handles.
 *
 * ============================================================================
 * 19. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     operation lookup
 *     operation version compatibility
 *     participant-set validity
 *     participant type compatibility
 *     value type compatibility
 *     result type compatibility
 *     reducer validity
 *     identity-element requirements
 *     associativity requirements
 *     commutativity requirements
 *     ordering requirements
 *     determinism requirements
 *     reproducibility requirements
 *     synchronization requirements
 *     consistency requirements
 *     fault/recovery requirements
 *     capability availability
 *     resource sufficiency
 *     policy authorization
 *     contract validity
 *     provenance validation
 *     target feasibility
 *
 * None of those decisions occur during parsing.
 *
 * ============================================================================
 * 20. RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Requirements are symbolic.
 *
 * The parser accepts expressions such as:
 *
 *     requires capability("collective.communication");
 *     requires memory >= required_memory;
 *     requires bandwidth >= required_bandwidth;
 *     requires topology(required_topology);
 *
 * The compiler later resolves:
 *
 *     requirement
 *         ->
 *     capability/resource negotiation
 *         ->
 *     execution planning
 *         ->
 *     target realization
 *
 * A failed requirement is a semantic feasibility diagnostic, not a parser
 * failure.
 *
 * ============================================================================
 * 21. NETWORKING CONTRACT
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     RDMA
 *     MPI
 *     InfiniBand
 *     vendor fabrics
 *     future transports
 *
 * Networking grammars and semantic analysis own transport realization.
 *
 * A collective operation may therefore remain unchanged while its realization
 * changes between environments.
 *
 * ============================================================================
 * 22. CONCURRENCY CONTRACT
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     actor lifecycle
 *     task scheduling
 *     channels
 *     locks
 *     futures
 *     async execution
 *
 * Existing concurrency grammars remain authoritative.
 *
 * A participant expression may semantically resolve to an actor, process,
 * task, service, device abstraction or other logical execution entity.
 *
 * ============================================================================
 * 23. QUANTUM CONTRACT
 * ============================================================================
 *
 * Collective computation may operate on:
 *
 *     classical values
 *     tensors
 *     measurement results
 *     logical quantum data
 *     hybrid values
 *     accelerator values
 *
 * Quantum-specific meaning is resolved downstream.
 *
 * The lowering path remains:
 *
 *     collective source
 *         ->
 *     domain-neutral AST
 *         ->
 *     distributed/quantum semantic analysis
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     resilience / QEC
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *
 * This grammar does not define physical qubits, gate sets, calibration,
 * coupling maps, QEC codes or quantum hardware.
 *
 * ============================================================================
 * 24. CLASSICAL / AI / DATA CONTRACT
 * ============================================================================
 *
 * Collective syntax is intentionally domain-neutral.
 *
 * It can therefore support semantic uses including:
 *
 *     distributed tensor computation
 *     model synchronization
 *     gradient aggregation
 *     parameter exchange
 *     distributed inference
 *     knowledge aggregation
 *     scientific reduction
 *     data aggregation
 *     probabilistic aggregation
 *     collective reasoning
 *
 * No AI framework, numerical library or application vocabulary is embedded.
 *
 * ============================================================================
 * 25. HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * A collective may eventually lower to:
 *
 *     CPU execution
 *     GPU execution
 *     FPGA fabric
 *     ASIC logic
 *     accelerator infrastructure
 *     hardware communication engines
 *     quantum-classical infrastructure
 *     future computational substrates
 *
 * Hardware realization is downstream.
 *
 * ============================================================================
 * 26. IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * It MUST NOT introduce:
 *
 *     CollectiveIR
 *     DistributedIR
 *     DistributedQuantumIR
 *     CollectiveQuantumIR
 *
 * as competing canonical representations.
 *
 * Collective semantic information must enter the repository's canonical
 * semantic/IR pipeline.
 *
 * Classical computation follows the classical IR path.
 *
 * Quantum computation follows:
 *
 *     quantum::ir
 *
 * HDL/hardware semantics follow the established hardware representation.
 *
 * ============================================================================
 * 27. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Source provenance must remain available for:
 *
 *     diagnostics
 *     reproducibility
 *     explainability
 *     optimization decisions
 *     resource decisions
 *     target decisions
 *     policy decisions
 *     distributed execution decisions
 *
 * The grammar preserves structure and source spans; provenance semantics are
 * downstream.
 *
 * ============================================================================
 * 28. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar version
 *     explicitly selected language configuration
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     target availability
 *     resources
 *     network state
 *     filesystem state
 *     runtime state
 *     time
 *     randomness
 *
 * Identical source and lexical/parser configuration must produce equivalent
 * parser structure.
 *
 * ============================================================================
 * 29. SECURITY / SAFETY CONTRACT
 * ============================================================================
 *
 * Parsing grants no:
 *
 *     network access
 *     filesystem access
 *     credential access
 *     hardware access
 *     deployment authority
 *     foreign-function authority
 *
 * Security and authorization are semantic/runtime concerns.
 *
 * The grammar contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no actions
 *     no runtime callbacks
 *     no unsafe code
 *
 * ============================================================================
 * 30. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover structural errors such as:
 *
 *     missing `collective`
 *     missing operation name
 *     missing body
 *     malformed invocation
 *     malformed property
 *     malformed contract
 *     malformed policy
 *     malformed capability clause
 *     malformed resource clause
 *     malformed effect clause
 *     malformed provenance clause
 *     malformed evidence clause
 *
 * Semantic diagnostics cover:
 *
 *     unknown operation
 *     unsupported operation
 *     invalid participant set
 *     invalid reducer
 *     incompatible types
 *     missing capability
 *     insufficient resources
 *     forbidden policy
 *     invalid contract
 *     unavailable realization
 *
 * ============================================================================
 * 31. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing parent integration remains:
 *
 *     grammar/distributed/distributed.g4
 *         ->
 *     import Collective
 *         ->
 *     collectiveConstruct
 *
 * The public rule `collectiveConstruct` is intentionally preserved.
 *
 * The `collective` spelling becomes a reserved lexical word because a
 * production distributed parser requires an unambiguous structural marker.
 *
 * This is a compatibility-affecting lexical change and must be recorded in
 * grammar/compatibility/.
 *
 * Programs that previously used `collective` as an ordinary identifier must
 * migrate that identifier to another name or use the repository's explicit
 * compatibility mechanism.
 *
 * No other collective algorithm names are reserved.
 *
 * ============================================================================
 * 32. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no fixed participant count
 *     no fixed node count
 *     no fixed process count
 *     no fixed worker count
 *     no fixed device count
 *     no fixed memory capacity
 *     no fixed bandwidth
 *     no fixed topology size
 *     no fixed collective algorithm list
 *     no fixed transport list
 *     no fixed CPU/GPU/FPGA/QPU catalogue
 *     no physical identifiers
 *     no hardware addresses
 *     no target-specific constants
 *
 * ============================================================================
 * 33. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     collective broadcast {
 *         participants: workers;
 *         value: data;
 *     }
 *
 *     collective reduce {
 *         participants: workers;
 *         value: values;
 *         reducer: reducer;
 *     }
 *
 *     collective custom::reduce {
 *         participants: group;
 *         value: values;
 *         reducer: custom::combine;
 *     }
 *
 *     collective future::collective::operation {
 *         participants: computed_group;
 *         value: tensor;
 *     }
 *
 *     collective broadcast(workers, data);
 *
 *     collective all_reduce(workers, values);
 *
 *     collective reduce {
 *         participants: workers;
 *         value: values;
 *
 *         requires capability("collective.reduce");
 *         requires memory >= required_memory;
 *         requires topology(required_topology);
 *
 *         contract {
 *             requires values_are_valid;
 *             ensures result_is_valid;
 *         }
 *
 *         policy {
 *             allow capability("distributed.collective");
 *             prefer topology(preferred_topology);
 *         }
 *
 *         provenance source_record;
 *         evidence validation_record;
 *     }
 *
 * ATTRIBUTES:
 *
 *     collective broadcast @collective.metadata {
 *         participants: workers;
 *         value: data;
 *     }
 *
 * NEGATIVE:
 *
 *     collective;
 *     collective { };
 *     collective broadcast;
 *     collective broadcast(;
 *     collective broadcast(workers;
 *     collective broadcast() malformed;
 *     collective broadcast {
 *         participants workers;
 *     }
 *
 * BOUNDARY:
 *
 *     one participant expression;
 *     computed participant sets;
 *     deeply qualified operation names;
 *     large argument lists;
 *     large property sets;
 *     deeply nested sections;
 *     large symbolic resource expressions;
 *     custom/vendor/future operation names.
 *
 * SCALABILITY:
 *
 *     progressively larger participant expressions;
 *     progressively larger argument lists;
 *     progressively deeper qualified names;
 *     progressively larger member sets;
 *     progressively deeper metadata sections.
 *
 * The language itself must not define a finite ceiling.
 *
 * DETERMINISM:
 *
 *     identical source + identical lexer/parser configuration
 *         ->
 *     equivalent parse structure.
 *
 * ============================================================================
 * 34. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] collectiveConstruct remains the stable public entry point.
 *     [x] `collective` has an unambiguous lexical token.
 *     [x] operation names are open-world qualified names.
 *     [x] participant sets use canonical expressions.
 *     [x] values use canonical expressions.
 *     [x] attributes use canonical Attributes grammar.
 *     [x] requirements remain symbolic.
 *     [x] contracts remain symbolic.
 *     [x] policies remain symbolic.
 *     [x] capabilities remain symbolic.
 *     [x] resources remain symbolic.
 *     [x] effects remain symbolic.
 *     [x] provenance remains symbolic.
 *     [x] evidence remains symbolic.
 *     [x] no collective algorithm catalogue exists.
 *     [x] no hardware catalogue exists.
 *     [x] no transport catalogue exists.
 *     [x] no physical topology is encoded.
 *     [x] no capacity limit is encoded.
 *     [x] no duplicate expression grammar exists.
 *     [x] no duplicate attribute grammar exists.
 *     [x] no IR is created.
 *     [x] no runtime decision is made.
 *     [x] no Rust action exists.
 *     [x] no unsafe implementation is required.
 *     [x] Rust 1.97+ integration remains possible.
 *
 * Repository-level gates:
 *
 *     [ ] ANTLR generation succeeds.
 *     [ ] Generated Rust parser builds on Rust 1.97+.
 *     [ ] Safe-Rust policy passes.
 *     [ ] Distributed composition tests pass.
 *     [ ] Positive parser tests pass.
 *     [ ] Negative parser tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Scalability tests pass within available resources.
 *     [ ] Determinism tests pass.
 *     [ ] AST mapping tests pass.
 *     [ ] Semantic integration tests pass.
 *
 * ============================================================================
 */