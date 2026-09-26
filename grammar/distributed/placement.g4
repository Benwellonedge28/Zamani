/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/placement.g4
 *
 * Grammar:
 *     DistributedPlacement
 *
 * Status:
 *     PRODUCTION DISTRIBUTED-PLACEMENT COMPOSITION GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No mutable parser-global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the DISTRIBUTED-DOMAIN ENTRY BOUNDARY for placement.
 *
 * It does NOT create a second placement language.
 *
 * The reusable placement-clause vocabulary is owned by:
 *
 *     grammar/resources/placement.g4
 *
 * This file adds only the distributed-domain wrapper needed to associate
 * placement intent with a distributed semantic subject.
 *
 * Conceptually:
 *
 *     distributed placement declaration
 *             |
 *             +--> distributed subject
 *             |
 *             +--> canonical resource placement specification
 *
 * The resulting syntax can express placement intent for:
 *
 *     nodes
 *     processes
 *     services
 *     actors
 *     tasks
 *     channels
 *     messages
 *     replicated computations
 *     partitions
 *     distributed data
 *     distributed quantum workloads
 *     accelerators
 *     hardware/software computations
 *     future distributed computational entities
 *
 * without requiring a new placement grammar for every domain.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP DECISION
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed placement declaration framing;
 *     - distributed placement naming;
 *     - distributed placement subject association;
 *     - distributed placement generic parameters;
 *     - distributed placement composition entry points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic placement clauses;
 *     - placement requirements;
 *     - placement constraints;
 *     - placement preferences;
 *     - placement hints;
 *     - placement policies;
 *     - placement replication properties;
 *     - placement mobility properties;
 *     - placement elasticity properties;
 *     - placement groups;
 *     - placement relations;
 *     - placement target syntax;
 *     - placement resource expressions.
 *
 * Those remain owned by:
 *
 *     grammar/resources/placement.g4
 *
 * This prevents:
 *
 *     distributed/placement.g4
 *              +
 *     resources/placement.g4
 *
 * from becoming two competing placement authorities.
 *
 * ============================================================================
 * PLACEMENT DOMAIN SEPARATION
 * ============================================================================
 *
 * The repository contains several placement-related domains:
 *
 *     grammar/resources/placement.g4
 *         Generic resource-placement intent.
 *
 *     grammar/distributed/placement.g4
 *         Distributed-domain placement association.
 *
 *     grammar/execution/placement.g4
 *         Execution-placement intent.
 *
 *     grammar/hardware/placement.g4
 *         Hardware-domain placement structures.
 *
 * These files MUST NOT become independent semantic placement systems.
 *
 * They are domain-specific syntax boundaries that eventually lower into the
 * common semantic placement model.
 *
 * The dependency direction is:
 *
 *     domain syntax
 *          |
 *          v
 *     common semantic placement model
 *          |
 *          +--> resource analysis
 *          +--> target selection
 *          +--> topology analysis
 *          +--> routing
 *          +--> scheduling
 *          +--> deployment
 *          +--> runtime
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     DistributedPlacement
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> distributed semantic analysis
 *          +--> security analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> resource/placement representation
 *          |
 *          v
 *     optimization
 *          |
 *          +--> placement realization
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     HAL / deployment
 *          |
 *          v
 *     runtime
 *
 * This grammar never constructs, modifies, or selects an IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed placement participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Placement therefore expresses semantic intent rather than a permanent
 * physical machine assignment.
 *
 * The grammar MUST NOT require:
 *
 *     - a fixed number of nodes;
 *     - a fixed number of processes;
 *     - a fixed number of services;
 *     - a fixed number of workers;
 *     - a fixed number of devices;
 *     - a fixed number of CPUs;
 *     - a fixed number of GPUs;
 *     - a fixed number of FPGAs;
 *     - a fixed number of QPUs;
 *     - a fixed number of qubits;
 *     - a fixed memory capacity;
 *     - a fixed cluster size;
 *     - a fixed topology;
 *     - a fixed hostname;
 *     - a fixed IP address;
 *     - a fixed port;
 *     - a fixed cloud provider;
 *     - a fixed region;
 *     - a fixed physical device identifier.
 *
 * ============================================================================
 * UNBOUNDED SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level constants such as:
 *
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_SERVICES
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_PLACEMENTS
 *     MAX_REGIONS
 *     MAX_GROUPS
 *
 * The grammar uses repetition and expressions instead of finite hardware
 * capacities.
 *
 * Therefore the same syntax can describe:
 *
 *     one logical participant
 *     one machine
 *     one accelerator
 *     many machines
 *     large clusters
 *     HPC systems
 *     federated systems
 *     heterogeneous systems
 *     distributed quantum systems
 *     future computational substrates
 *
 * subject only to actual semantic and resource availability.
 *
 * "Infinity" here means:
 *
 *     no artificial language-level finite ceiling.
 *
 * It does not claim that a particular compiler, parser, runtime, operating
 * system, or physical deployment has infinite resources.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode universal limits equivalent to:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Nor may it encode equivalent physical assumptions such as:
 *
 *     exactly 8 CPUs
 *     exactly 32 GPUs
 *     exactly 1024 qubits
 *     exactly 64 GB memory
 *     exactly 32-bit registers
 *     exactly N nodes
 *
 * A number appearing in an expression remains PROGRAM DATA.
 *
 * For example:
 *
 *     placement::replicas = replica_count;
 *
 * is semantic program intent.
 *
 * It is not a compiler-wide maximum.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * The semantic model MUST preserve these distinctions:
 *
 *     REQUIREMENT
 *         Must be satisfied.
 *
 *     CONSTRAINT
 *         Restricts legal realizations.
 *
 *     PREFERENCE
 *         Desirable but non-mandatory.
 *
 *     HINT
 *         Advisory information that may be ignored.
 *
 * These distinctions are inherited from the canonical resource-placement
 * grammar.
 *
 * This file MUST NOT reinterpret one category as another.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Valid portable intent includes concepts such as:
 *
 *     placement::scope = execution_region;
 *
 *     placement::affinity = service_a, service_b;
 *
 *     placement::anti_affinity = replica_group;
 *
 *     placement::target = accelerator;
 *
 *     placement::replicas = replica_count;
 *
 *     placement::migration = migration_policy;
 *
 *     placement::elasticity = workload_size;
 *
 *     placement::latency = latency_budget;
 *
 *     placement::capability = capability_requirement;
 *
 * Physical realization is downstream.
 *
 * The grammar does not decide which machine, node, CPU, GPU, FPGA, QPU,
 * memory bank, network link, or physical qubit satisfies those expressions.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed placement may apply to:
 *
 *     logical quantum workloads;
 *     logical qubits;
 *     logical registers;
 *     distributed circuits;
 *     quantum/classical services;
 *     quantum accelerators;
 *     distributed quantum execution.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     coupling maps
 *     SWAP insertion
 *     pulse placement
 *     calibration
 *     QEC algorithms
 *     ZQN noise models
 *
 * Quantum semantics continue through:
 *
 *     quantum::ir
 *
 * Placement contributes intent only.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Distributed placement may refer semantically to:
 *
 *     hardware classes;
 *     accelerator classes;
 *     memory domains;
 *     compute domains;
 *     interconnect domains;
 *     capability domains.
 *
 * It MUST NOT define:
 *
 *     physical FPGA cells;
 *     ASIC coordinates;
 *     physical pins;
 *     physical registers;
 *     physical memory addresses;
 *     device inventory;
 *     vendor-specific placement coordinates.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Placement is not routing.
 *
 * Placement may express:
 *
 *     locality;
 *     affinity;
 *     anti-affinity;
 *     topology-related requirements;
 *     latency-related intent.
 *
 * It does not construct:
 *
 *     network paths;
 *     packets;
 *     routes;
 *     sockets;
 *     ports;
 *     transport protocols.
 *
 * Those belong to networking/routing/runtime layers.
 *
 * ============================================================================
 * REPLICATION INTEGRATION
 * ============================================================================
 *
 * Replication and placement remain separate semantic concepts.
 *
 * Replication says:
 *
 *     how many logical realizations are required or permitted.
 *
 * Placement says:
 *
 *     what placement relationships those realizations should satisfy.
 *
 * This file therefore does not reimplement:
 *
 *     grammar/distributed/replication.g4
 *
 * Instead, placement properties may refer to replication-related semantic
 * values through the canonical resource-placement property mechanism.
 *
 * ============================================================================
 * MIGRATION / ELASTICITY
 * ============================================================================
 *
 * Migration and elasticity are expressed as placement properties.
 *
 * This grammar does not implement:
 *
 *     migration algorithms;
 *     checkpointing;
 *     state transfer;
 *     failover;
 *     rescheduling;
 *     autoscaling;
 *     load balancing.
 *
 * Those belong to semantic/runtime/deployment systems.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware queries;
 *     - no runtime callbacks;
 *     - no randomness.
 *
 * Parsing depends only on:
 *
 *     - the supplied token stream;
 *     - the selected Zamani language version;
 *     - the imported grammar contracts.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * Frontend AST construction must preserve:
 *
 *     - declaration source span;
 *     - placement name;
 *     - subject expression;
 *     - generic parameter ordering;
 *     - placement-clause ordering;
 *     - nested expression structure;
 *     - source spans of all child constructs.
 *
 * The parser MUST NOT perform:
 *
 *     - name resolution;
 *     - resource discovery;
 *     - capability lookup;
 *     - placement selection;
 *     - target selection.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptually, the frontend AST should contain:
 *
 *     DistributedPlacementDeclaration
 *         name
 *         generic_parameters
 *         subject
 *         specification
 *         source_span
 *
 * The nested placement specification should lower to the same placement
 * semantic representation used by the resource subsystem.
 *
 * This wrapper MUST NOT create a second DistributedPlacementClause hierarchy
 * if the frontend already has a canonical placement-clause representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving the placement declaration name;
 *     - resolving the placement subject;
 *     - validating generic parameters;
 *     - resolving placement properties;
 *     - distinguishing requirements/constraints/preferences/hints;
 *     - checking placement compatibility;
 *     - checking resource availability;
 *     - checking capabilities;
 *     - checking topology requirements;
 *     - checking distributed consistency implications;
 *     - checking replication interactions;
 *     - checking migration legality;
 *     - checking elasticity legality;
 *     - checking quantum placement compatibility;
 *     - checking hardware capability compatibility.
 *
 * None of those checks belong in parser actions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar defines NO IR.
 *
 * Placement semantics must lower into the repository's canonical semantic
 * representation.
 *
 * Quantum-related placement information ultimately integrates with:
 *
 *     quantum::ir
 *
 * rather than creating:
 *
 *     DistributedQuantumPlacementIR
 *
 * or another competing quantum intermediate representation.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The stable public rule remains:
 *
 *     distributedPlacementDeclaration
 *
 * The file remains:
 *
 *     grammar/distributed/placement.g4
 *
 * No filename rename is required.
 *
 * The previous distributed placement body was internally duplicating resource
 * placement constructs. The replacement deliberately preserves the public
 * distributed entry point while delegating clause ownership to
 * ResourcePlacement.
 *
 * ============================================================================
 */

parser grammar DistributedPlacement;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ResourcePlacement is the canonical reusable placement-clause authority.
 *
 * Names and Expressions remain direct dependencies because the distributed
 * wrapper owns its declaration name, generic parameters, and subject.
 *
 * ResourcePlacement itself owns resource-placement expressions and clauses.
 */
import Names, Expressions, ResourcePlacement;


/* ============================================================================
 * 1. PUBLIC DISTRIBUTED PLACEMENT DECLARATION
 * ============================================================================
 *
 * Canonical source shape:
 *
 *     placement Policy {
 *         placement::scope = execution_region;
 *         placement::target = accelerator;
 *     }
 *
 * Subject-associated form:
 *
 *     placement Policy(workload) {
 *         placement::scope = execution_region;
 *     }
 *
 * Generic form:
 *
 *     placement Policy<N> {
 *         placement::replicas = N;
 *     }
 *
 * Generic + subject form:
 *
 *     placement Policy<N>(workload) {
 *         placement::replicas = N;
 *     }
 *
 * The word "placement" remains contextual at this boundary because the
 * repository's canonical lexer does not establish a dedicated distributed
 * placement token.
 *
 * Semantic analysis MUST verify that the first identifier has the contextual
 * spelling required by this construct.
 */
distributedPlacementDeclaration
    : distributedPlacementKeyword
      identifier
      distributedPlacementGenericParameters?
      distributedPlacementSubjectClause?
      resourcePlacementSpecification
    ;


/* ============================================================================
 * 2. CONTEXTUAL PLACEMENT KEYWORD
 * ============================================================================
 *
 * The distributed grammar deliberately does not invent a new lexer token.
 *
 * This keeps the parser compatible with the repository's open-world lexical
 * model and prevents a distributed-only keyword from becoming a second lexical
 * authority.
 *
 * Semantic validation establishes the contextual meaning.
 */
distributedPlacementKeyword
    : identifier
    ;


/* ============================================================================
 * 3. GENERIC PARAMETERS
 * ============================================================================
 *
 * These are semantic parameters, not hardware limits.
 *
 * No finite parameter count is encoded.
 */
distributedPlacementGenericParameters
    : LESS_THAN
      distributedPlacementGenericParameter
      (COMMA distributedPlacementGenericParameter)*
      COMMA?
      GREATER_THAN
    ;


distributedPlacementGenericParameter
    : identifier
      distributedPlacementGenericBound?
      (
          ASSIGN
          expression
      )?
    ;


distributedPlacementGenericBound
    : COLON
      qualifiedName
    ;


/* ============================================================================
 * 4. DISTRIBUTED PLACEMENT SUBJECT
 * ============================================================================
 *
 * Parentheses make subject association structurally unambiguous.
 *
 * Examples:
 *
 *     placement WorkerPolicy(worker) { ... }
 *
 *     placement QuantumPolicy(quantum_workload) { ... }
 *
 *     placement ServicePolicy(service::api) { ... }
 *
 * The subject is an expression rather than a closed domain-specific name.
 */
distributedPlacementSubjectClause
    : LPAREN
      expressionList?
      RPAREN
    ;


/* ============================================================================
 * 5. CANONICAL PLACEMENT SPECIFICATION
 * ============================================================================
 *
 * This is a transparent wrapper around:
 *
 *     ResourcePlacement.resourcePlacementSpecification
 *
 * No second clause grammar is created here.
 */
distributedPlacementSpecification
    : resourcePlacementSpecification
    ;


/* ============================================================================
 * 6. CANONICAL PLACEMENT CLAUSE
 * ============================================================================
 *
 * This transparent wrapper allows distributed consumers to refer to placement
 * clauses through a distributed-domain public name without creating a second
 * semantic clause hierarchy.
 */
distributedPlacementClause
    : resourcePlacementClause
    ;


/* ============================================================================
 * 7. CANONICAL PLACEMENT CLAUSE LIST
 * ============================================================================
 *
 * The list is unbounded at the language level.
 */
distributedPlacementClauseList
    : distributedPlacementClause*
    ;


/* ============================================================================
 * 8. OPTIONAL SPECIFICATION
 * ============================================================================
 *
 * Useful to higher-level distributed grammars that need to attach optional
 * placement intent to another declaration.
 */
optionalDistributedPlacementSpecification
    : distributedPlacementSpecification?
    ;


/* ============================================================================
 * 9. PLACEMENT REFERENCE
 * ============================================================================
 *
 * A placement declaration may be referenced by name by higher-level
 * distributed constructs.
 *
 * This is only structural name syntax.
 *
 * Semantic resolution determines whether the name actually denotes a
 * DistributedPlacementDeclaration.
 */
distributedPlacementReference
    : qualifiedName
    ;


/* ============================================================================
 * 10. PLACEMENT REFERENCE LIST
 * ============================================================================
 *
 * No finite number of placement policies is imposed.
 */
distributedPlacementReferenceList
    : distributedPlacementReference
      (COMMA distributedPlacementReference)*
      COMMA?
    ;


/* ============================================================================
 * 11. SUBJECT LIST
 * ============================================================================
 *
 * Useful for consumers that want to associate the same placement declaration
 * with multiple logical distributed entities.
 *
 * This is a structural helper only.
 */
distributedPlacementSubjectList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 12. SEMANTIC CATEGORY BOUNDARY
 * ============================================================================
 *
 * The following concepts are inherited from ResourcePlacement:
 *
 *     relation
 *     scope
 *     target
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     policy
 *     replication
 *     mobility
 *     elasticity
 *     group
 *     property
 *
 * Distributed placement MUST NOT redefine those categories.
 *
 * Semantic analysis may interpret them in the distributed domain.
 *
 * ============================================================================
 * 13. OPEN-WORLD EXTENSION
 * ============================================================================
 *
 * Because ResourcePlacement uses qualified names and expressions, new
 * distributed placement dimensions can be represented without adding:
 *
 *     - a new lexer keyword;
 *     - a new hardware enumeration;
 *     - a new finite resource table;
 *     - a new machine-size constant.
 *
 * Examples remain semantically expressible through canonical placement
 * properties:
 *
 *     placement::locality
 *     placement::affinity
 *     placement::anti_affinity
 *     placement::co_location
 *     placement::separation
 *     placement::scope
 *     placement::target
 *     placement::capability
 *     placement::replicas
 *     placement::migration
 *     placement::elasticity
 *     placement::latency
 *     placement::bandwidth
 *     placement::reliability
 *     placement::energy
 *
 * The grammar intentionally does not enumerate this property inventory.
 *
 * ============================================================================
 * 14. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Placement values may reference resource expressions through the canonical
 * ResourcePlacement grammar.
 *
 * This permits semantic requirements such as:
 *
 *     placement::resource = required_resources;
 *
 *     placement::capability = capability("tensor.compute");
 *
 *     placement::capability = capability("quantum.measurement");
 *
 *     placement::scope = execution_region;
 *
 * without turning those into physical hardware assignments.
 *
 * Resource availability is resolved downstream.
 *
 * ============================================================================
 * 15. DISTRIBUTED NODE INTEGRATION
 * ============================================================================
 *
 * A placement subject may semantically resolve to an entity declared by:
 *
 *     grammar/distributed/nodes.g4
 *
 * This file does not import or duplicate node declarations.
 *
 * Node identity remains logical.
 *
 * A node declaration does not automatically imply:
 *
 *     physical machine;
 *     CPU;
 *     GPU;
 *     FPGA;
 *     QPU;
 *     hostname;
 *     network address.
 *
 * ============================================================================
 * 16. PROCESS / SERVICE / ACTOR INTEGRATION
 * ============================================================================
 *
 * A placement subject may resolve to entities declared by:
 *
 *     grammar/distributed/processes.g4
 *     grammar/distributed/services.g4
 *     grammar/distributed/actors.g4
 *
 * No declaration grammar is duplicated here.
 *
 * ============================================================================
 * 17. COMMUNICATION INTEGRATION
 * ============================================================================
 *
 * Placement may semantically constrain communication-related locality,
 * affinity, topology requirements, or latency preferences.
 *
 * It does not define:
 *
 *     channels;
 *     messages;
 *     endpoints;
 *     protocols;
 *     routes;
 *     packet formats.
 *
 * Those remain owned by:
 *
 *     distributed/communication.g4
 *     distributed/channels.g4
 *     distributed/messages.g4
 *     networking/*
 *
 * ============================================================================
 * 18. REPLICATION INTEGRATION
 * ============================================================================
 *
 * `placement::replicas` and related placement properties describe placement
 * intent associated with replicated semantic objects.
 *
 * They do not replace:
 *
 *     grammar/distributed/replication.g4
 *
 * Replication semantics remain owned by that grammar and its semantic layer.
 *
 * ============================================================================
 * 19. TOPOLOGY INTEGRATION
 * ============================================================================
 *
 * Placement may refer to topology-related semantic properties.
 *
 * This grammar does not define:
 *
 *     nodes;
 *     edges;
 *     links;
 *     graph traversal;
 *     routes;
 *     topology algorithms.
 *
 * Topology remains downstream/domain-owned.
 *
 * ============================================================================
 * 20. ROUTING INTEGRATION
 * ============================================================================
 *
 * Placement answers:
 *
 *     WHERE / UNDER WHICH LOCATION RELATIONSHIPS?
 *
 * Routing answers:
 *
 *     HOW DOES DATA OR COMPUTATION MOVE?
 *
 * Therefore this grammar contains no routing algorithm and no route syntax.
 *
 * ============================================================================
 * 21. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Placement answers spatial/logical intent.
 *
 * Scheduling answers temporal/resource-order realization.
 *
 * This grammar therefore does not own:
 *
 *     schedules;
 *     clocks;
 *     deadlines;
 *     dispatch order;
 *     critical paths;
 *     resource allocation algorithms.
 *
 * ============================================================================
 * 22. QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * When a placement subject participates in quantum computation:
 *
 *     distributed placement
 *          |
 *          v
 *     semantic placement model
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     routing / scheduling / QEC / ZQN / HAL
 *
 * This file never creates another quantum IR.
 *
 * ============================================================================
 * 23. HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL systems may consume the semantic placement result.
 *
 * They determine:
 *
 *     target realization;
 *     resource mapping;
 *     topology realization;
 *     synthesis;
 *     routing;
 *     physical implementation.
 *
 * This grammar remains hardware-independent.
 *
 * ============================================================================
 * 24. AST COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete independently when:
 *
 *     [x] distributedPlacementDeclaration is stable;
 *     [x] declaration naming is defined;
 *     [x] generic parameters are defined;
 *     [x] subject association is defined;
 *     [x] canonical ResourcePlacement is reused;
 *     [x] no second placement-clause hierarchy is introduced;
 *     [x] no lexical rules are defined;
 *     [x] no semantic predicates are required;
 *     [x] no target-specific actions exist;
 *     [x] no physical resource limits exist.
 *
 * The AST contract is:
 *
 *     DistributedPlacementDeclaration
 *         name
 *         generic_parameters
 *         subject
 *         placement_specification
 *         source_span
 *
 * The nested placement specification is the existing canonical placement
 * representation, not a distributed-only copy.
 *
 * ============================================================================
 * 25. SEMANTIC COMPLETION CONTRACT
 * ============================================================================
 *
 * Semantic analysis must establish:
 *
 *     - declaration identity;
 *     - subject identity;
 *     - generic parameter validity;
 *     - placement property validity;
 *     - requirement/constraint/preference/hint category;
 *     - resource compatibility;
 *     - capability compatibility;
 *     - topology compatibility;
 *     - replication compatibility;
 *     - migration compatibility;
 *     - elasticity compatibility;
 *     - quantum compatibility;
 *     - hardware compatibility;
 *     - portability.
 *
 * ============================================================================
 * 26. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should remain structural.
 *
 * Examples of semantic diagnostics belong downstream:
 *
 *     unknown placement declaration;
 *     unresolved subject;
 *     incompatible placement requirement;
 *     unsatisfied capability;
 *     conflicting placement constraints;
 *     invalid replication/placement combination;
 *     unsupported migration policy;
 *     unsupported target capability.
 *
 * The parser MUST NOT inspect hardware to produce these diagnostics.
 *
 * ============================================================================
 * 27. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance tests must cover:
 *
 *     placement P {}
 *
 *     placement P(workload) {}
 *
 *     placement P<N> {}
 *
 *     placement P<N>(workload) {}
 *
 * and arbitrarily many canonical placement clauses.
 *
 * Tests must verify that there is no grammar-level upper bound on:
 *
 *     placement declarations;
 *     generic parameters;
 *     placement clauses;
 *     expression complexity;
 *     qualified-name depth;
 *     distributed subjects.
 *
 * ============================================================================
 * 28. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also contains no physical device identifiers or topology constants.
 *
 * ============================================================================
 * 29. INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/distributed/distributed.g4
 *     MUST consume:
 *
 *         distributedPlacementDeclaration
 *
 *     and MUST NOT redefine placement syntax.
 *
 * grammar/resources/resources.g4
 *     remains the resource-domain composition authority.
 *
 * grammar/resources/placement.g4
 *     remains the canonical generic placement-clause authority.
 *
 * grammar/distributed/replication.g4
 *     remains the replication authority.
 *
 * grammar/distributed/topology.g4
 *     if/when present, remains the distributed-topology authority.
 *
 * grammar/execution/placement.g4
 *     remains execution-placement syntax and does not get imported here.
 *
 * grammar/hardware/placement.g4
 *     remains hardware-domain placement syntax and does not get imported here.
 *
 * These domains converge semantically rather than by duplicating grammar.
 *
 * ============================================================================
 * 30. BUILD / RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Generated parser integration must satisfy:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * with:
 *
 *     no unsafe Rust;
 *     no embedded target-specific parser actions;
 *     no semantic predicates requiring runtime Rust state.
 *
 * Recommended validation:
 *
 *     cargo +1.97.1 check
 *     cargo +1.97.1 test
 *     cargo +1.97.1 clippy -- -D warnings
 *
 * The grammar itself must remain target-language neutral.
 *
 * ============================================================================
 * 31. FINAL INVARIANTS
 * ============================================================================
 *
 *     ONE distributed placement entry point:
 *
 *         distributedPlacementDeclaration
 *
 *     ONE reusable generic placement-clause authority:
 *
 *         ResourcePlacement
 *
 *     ONE canonical resource semantic model downstream.
 *
 *     ONE canonical quantum IR:
 *
 *         quantum::ir
 *
 *     NO physical hardware limits.
 *
 *     NO hard-coded device counts.
 *
 *     NO routing implementation.
 *
 *     NO scheduling implementation.
 *
 *     NO QEC implementation.
 *
 *     NO ZQN implementation.
 *
 *     NO runtime execution.
 *
 *     NO unsafe Rust.
 *
 *     NO second placement language.
 *
 * ============================================================================
 */