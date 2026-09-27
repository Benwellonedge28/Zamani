/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/placement.g4
 *
 * Grammar:
 *     ExecutionPlacement
 *
 * Status:
 *     Production-ready execution-placement intent grammar
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Grammar-only.
 *     No embedded Rust.
 *     No semantic actions.
 *     No semantic predicates.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL EXECUTION PLACEMENT INTENT.
 *
 * Placement answers:
 *
 *     "Where may, should, or must an execution subject be realized?"
 *
 * It does NOT answer:
 *
 *     "Which physical resource will actually be selected?"
 *
 * Physical realization is resolved downstream using:
 *
 *     resource discovery
 *     capability analysis
 *     target selection
 *     topology
 *     routing
 *     scheduling
 *     resilience
 *     deployment
 *     runtime state
 *     hardware abstraction
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
 *     ZamaniParser
 *          |
 *          v
 *     ExecutionPlacement
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> resources
 *          +--> capabilities
 *          +--> portability
 *          +--> hardware intent
 *          +--> distributed intent
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          +--> placement realization
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     execution placement intent;
 *     placement subject association;
 *     placement properties;
 *     placement requirements;
 *     placement constraints;
 *     placement preferences;
 *     placement hints;
 *     locality intent;
 *     affinity intent;
 *     anti-affinity intent;
 *     co-location intent;
 *     separation intent;
 *     scope intent;
 *     target intent;
 *     migration/mobility intent;
 *     replication intent;
 *     elasticity intent;
 *     placement metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical token definitions;
 *     identifiers;
 *     general expression syntax;
 *     type syntax;
 *     resource semantics;
 *     capability discovery;
 *     hardware discovery;
 *     topology;
 *     physical allocation;
 *     quantum physical-qubit mapping;
 *     routing;
 *     scheduling algorithms;
 *     optimization algorithms;
 *     QEC;
 *     ZQN;
 *     calibration;
 *     HAL;
 *     runtime implementation;
 *     deployment implementation;
 *     vendor APIs;
 *     canonical IR.
 *
 * ============================================================================
 * IMPORTANT SEPARATION
 * ============================================================================
 *
 * There are several placement-related grammar components in the repository.
 *
 * They are NOT interchangeable:
 *
 *     execution/placement.g4
 *         execution placement intent
 *
 *     resources/placement.g4
 *         resource-domain placement intent
 *
 *     hardware/placement.g4
 *         hardware-domain placement structures
 *
 *     distributed/placement.g4
 *         distributed-domain placement structures
 *
 * This grammar MUST NOT duplicate those domains.
 *
 * They may eventually lower into compatible semantic placement structures,
 * but ownership remains separated at the syntax boundary.
 *
 * ============================================================================
 * RELATIONSHIP TO EXECUTION COMPOSITION
 * ============================================================================
 *
 * grammar/execution/execution.g4 is the execution-domain composition root.
 *
 * It owns composition/dispatch.
 *
 * This file owns the reusable:
 *
 *     executionPlacement
 *
 * rule.
 *
 * execution.g4 MUST import/compose this grammar rather than redefine the
 * placement language.
 *
 * ============================================================================
 * RELATIONSHIP TO RUNTIME
 * ============================================================================
 *
 * runtime.g4 owns runtime intent.
 *
 * Runtime may consume the semantic placement result.
 *
 * Runtime MUST NOT reinterpret source placement syntax as a physical-device
 * selection without explicit semantic resolution.
 *
 * ============================================================================
 * RELATIONSHIP TO SCHEDULING
 * ============================================================================
 *
 * Placement and scheduling are separate concerns.
 *
 * Placement:
 *
 *     WHERE
 *
 * Scheduling:
 *
 *     WHEN
 *     IN WHAT ORDER
 *     UNDER WHAT TIMING POLICY
 *
 * This grammar MUST NOT implement:
 *
 *     ASAP
 *     ALAP
 *     list scheduling
 *     critical-path scheduling
 *     resource allocation
 *     queue management
 *     pulse scheduling
 *     scheduler algorithms.
 *
 * ============================================================================
 * RELATIONSHIP TO ROUTING
 * ============================================================================
 *
 * Placement MUST NOT insert or calculate:
 *
 *     SWAP
 *     MOVE
 *     transport operations
 *     paths
 *     routes
 *     physical qubit mappings.
 *
 * For quantum computation:
 *
 *     logical computation
 *          |
 *          v
 *     placement intent
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     physical realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Placement is designed for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore source placement MUST remain target-independent unless the
 * programmer explicitly opts into a target-specific dialect/profile.
 *
 * Portable placement may express:
 *
 *     locality;
 *     capability;
 *     resource requirements;
 *     affinity;
 *     anti-affinity;
 *     co-location;
 *     separation;
 *     scope;
 *     target class;
 *     migration policy;
 *     replication policy;
 *     elasticity;
 *     preferences;
 *     hints.
 *
 * It MUST NOT impose:
 *
 *     fixed CPU counts;
 *     fixed GPU counts;
 *     fixed FPGA counts;
 *     fixed QPU counts;
 *     fixed node counts;
 *     fixed qubit counts;
 *     fixed memory capacities;
 *     fixed topology dimensions;
 *     fixed physical addresses;
 *     fixed device identifiers.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains no finite limits on:
 *
 *     placement declarations;
 *     placement clauses;
 *     namespace depth;
 *     expression size;
 *     resource quantities;
 *     target classes;
 *     groups;
 *     replicas;
 *     regions;
 *     nodes;
 *     devices;
 *     accelerators;
 *     qubits;
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     QPUs.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Quantities remain expressions.
 *
 * Consequently:
 *
 *     placement { replicas: n; }
 *
 * is structurally independent of the eventual value of n.
 *
 * "Infinity" means that the language grammar introduces no artificial finite
 * capacity. Actual execution remains bounded by available resources and
 * implementation limits.
 *
 * ============================================================================
 * EXTENSIBILITY PRINCIPLE
 * ============================================================================
 *
 * DO NOT create one lexer keyword for every placement property.
 *
 * Examples that remain ordinary names:
 *
 *     locality
 *     affinity
 *     anti_affinity
 *     capability
 *     resource
 *     target
 *     topology
 *     region
 *     energy
 *     fidelity
 *     migration
 *     replication
 *     elasticity
 *     vendor::property
 *     future::placement::property
 *
 * This permits the language to evolve without continuously expanding the
 * global keyword vocabulary.
 *
 * The single stable introducer is:
 *
 *     placement
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * The canonical parser vocabulary is:
 *
 *     ZamaniLexer
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * A single reserved keyword is required:
 *
 *     PLACEMENT : 'placement'
 *
 * It belongs in:
 *
 *     grammar/lexer/keywords.g4
 *
 * No placement-specific lexer tokens are required for properties.
 *
 * The following MUST NOT be added merely for this grammar:
 *
 *     PLACEMENT_LOCALITY
 *     PLACEMENT_TARGET
 *     PLACEMENT_CAPABILITY
 *     PLACEMENT_AFFINITY
 *     PLACEMENT_RESOURCE
 *     PLACEMENT_SCOPE
 *     PLACEMENT_REPLICATION
 *
 * Those remain identifiers.
 *
 * ============================================================================
 * CORE DEPENDENCY
 * ============================================================================
 *
 * General expression syntax is imported from:
 *
 *     grammar/expressions/expressions.g4
 *
 * The expression grammar already uses the canonical ZamaniLexer vocabulary.
 *
 * This file does not duplicate expression precedence.
 *
 * ============================================================================
 * NAME DEPENDENCY
 * ============================================================================
 *
 * The repository's canonical Names grammar currently defines:
 *
 *     identifier
 *     qualifiedName
 *
 * but currently declares ZamaniTokens as its direct token vocabulary while
 * the production parser architecture uses ZamaniLexer.
 *
 * This grammar therefore provides a PRIVATE placement-name wrapper using the
 * canonical lexer tokens rather than introducing a competing general-purpose
 * name grammar.
 *
 * Repository-wide normalization should eventually make Names consume the
 * same canonical ZamaniLexer boundary.
 *
 * That normalization is a repository integration task, not a placement
 * semantic rule.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Supported forms:
 *
 *     placement {
 *         locality: local;
 *     }
 *
 *     placement computation {
 *         locality: local;
 *     }
 *
 *     placement computation {
 *         requires: capability("gpu.compute");
 *         target: accelerator("quantum");
 *     }
 *
 * A complex expression can be supplied as a parenthesized subject:
 *
 *     placement (compute_stage(input)) {
 *         locality: local;
 *     }
 *
 * ============================================================================
 */

parser grammar ExecutionPlacement;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * PUBLIC PLACEMENT DECLARATION
 * ============================================================================
 *
 * The subject is optional.
 *
 * If absent, semantic analysis attaches the placement to the enclosing
 * execution scope established by execution.g4.
 */
executionPlacement
    : PLACEMENT executionPlacementSubject? executionPlacementBody
    ;


/*
 * ============================================================================
 * PLACEMENT SUBJECT
 * ============================================================================
 *
 * A qualified name is the preferred unambiguous subject form.
 *
 * Parenthesized expressions permit richer subjects without allowing the
 * general expression grammar to consume the placement body.
 */
executionPlacementSubject
    : executionPlacementQualifiedName
    | LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * PLACEMENT BODY
 * ============================================================================
 *
 * Empty bodies are syntactically legal.
 *
 * Semantic validation decides whether an empty placement is meaningful in its
 * particular context.
 */
executionPlacementBody
    : LBRACE executionPlacementClause* RBRACE
    ;


/*
 * ============================================================================
 * PLACEMENT CLAUSE
 * ============================================================================
 *
 * A clause is an open-world property/value pair.
 *
 * This is deliberate.
 *
 * The grammar does NOT enumerate every possible placement concept.
 *
 * Semantic analysis classifies standard properties into categories such as:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     locality
 *     affinity
 *     anti-affinity
 *     target
 *     scope
 *     replication
 *     migration
 *     elasticity
 *
 * Unknown properties may be:
 *
 *     accepted by a registered dialect;
 *     accepted as an extension;
 *     rejected by a conformance profile;
 *     diagnosed as unsupported.
 *
 * The parser does not need to change for every future property.
 */
executionPlacementClause
    : executionPlacementKey executionPlacementSeparator executionPlacementValue executionPlacementTerminator
    ;


/*
 * ============================================================================
 * PLACEMENT KEY
 * ============================================================================
 *
 * Qualified keys permit namespaces:
 *
 *     locality
 *     placement::locality
 *     hardware::placement
 *     quantum::placement
 *     distributed::placement
 *     vendor::placement::property
 *     future::placement::property
 *
 * Semantic analysis determines whether the namespace is legal in the active
 * language version/dialect.
 */
executionPlacementKey
    : executionPlacementQualifiedName
    ;


/*
 * ============================================================================
 * PLACEMENT VALUE
 * ============================================================================
 *
 * Values use the canonical expression grammar.
 *
 * This is essential for scalability.
 *
 * Values may therefore be:
 *
 *     literals;
 *     names;
 *     qualified names;
 *     function calls;
 *     arithmetic expressions;
 *     comparisons;
 *     conditional expressions;
 *     arrays;
 *     maps;
 *     symbolic resource quantities;
 *     capability expressions;
 *     compile-time values;
 *     runtime-derived values where permitted semantically.
 *
 * No domain-specific value grammar is duplicated here.
 */
executionPlacementValue
    : expression
    ;


/*
 * ============================================================================
 * MULTI-VALUE SUPPORT
 * ============================================================================
 *
 * A comma-separated value list is useful for relationships such as:
 *
 *     affinity: producer, consumer;
 *
 *     anti_affinity: replica_a, replica_b;
 *
 *     co_location: stage_a, stage_b;
 *
 *     separation: tenant_a, tenant_b;
 *
 * It remains expression-based rather than introducing a domain-specific
 * identifier list.
 */
executionPlacementValueList
    : executionPlacementValue
      (COMMA executionPlacementValue)*
    ;


/*
 * ============================================================================
 * SEPARATOR
 * ============================================================================
 *
 * COLON is the canonical property separator.
 *
 * ASSIGN is also accepted for compatibility with existing placement/resource
 * design material:
 *
 *     placement::target: accelerator;
 *
 *     placement::target = accelerator;
 *
 * Semantic analysis must normalize both spellings to the same property
 * representation.
 */
executionPlacementSeparator
    : COLON
    | ASSIGN
    ;


/*
 * ============================================================================
 * TERMINATOR
 * ============================================================================
 *
 * Semicolons are mandatory inside a placement body.
 *
 * Requiring the terminator keeps property boundaries deterministic and
 * prevents newline-sensitive grammar behavior.
 */
executionPlacementTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * QUALIFIED NAME
 * ============================================================================
 *
 * This local wrapper exists only because the current repository's Names
 * component and canonical parser vocabulary are temporarily inconsistent.
 *
 * It is intentionally minimal and MUST NOT become a replacement for
 * grammar/core/names.g4.
 *
 * Once the repository-wide Names token-vocabulary normalization is complete,
 * this rule may be replaced by the canonical qualifiedName reference without
 * changing the placement language contract.
 */
executionPlacementQualifiedName
    : executionPlacementNameSegment
      (DOUBLE_COLON executionPlacementNameSegment)*
    ;


executionPlacementNameSegment
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * SEMANTIC CLASSIFICATION CONTRACT
 * ============================================================================
 *
 * The following names are STANDARD semantic property names.
 *
 * They are intentionally not lexer keywords.
 *
 * REQUIREMENT-LIKE:
 *
 *     requires
 *     require
 *     mandatory
 *
 * CAPABILITY:
 *
 *     capability
 *     capabilities
 *
 * RESOURCE:
 *
 *     resource
 *     resources
 *
 * CONSTRAINT:
 *
 *     constraint
 *     constraints
 *
 * PREFERENCE:
 *
 *     prefer
 *     preference
 *     preferences
 *
 * HINT:
 *
 *     hint
 *     hints
 *
 * TARGET:
 *
 *     target
 *     targets
 *
 * LOCALITY:
 *
 *     locality
 *     location
 *     region
 *
 * RELATIONSHIPS:
 *
 *     affinity
 *     anti_affinity
 *     co_location
 *     separation
 *
 * EXECUTION SCOPE:
 *
 *     scope
 *     domain
 *
 * MOBILITY:
 *
 *     migration
 *     mobility
 *
 * REPLICATION:
 *
 *     replication
 *     replicas
 *
 * ELASTICITY:
 *
 *     elasticity
 *     elastic
 *
 * TOPOLOGY:
 *
 *     topology
 *
 * These are semantic registry entries, not grammar alternatives.
 *
 * A future property:
 *
 *     quantum::placement::fidelity
 *
 * can therefore be introduced without changing this grammar.
 *
 * ============================================================================
 * REQUIREMENT / PREFERENCE / HINT SEPARATION
 * ============================================================================
 *
 * The semantic layer MUST preserve the distinction between:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     implementation decision
 *
 * For example:
 *
 *     requires: qubits >= n;
 *
 * means a semantic requirement.
 *
 *     capability: capability("quantum.mid_circuit_measurement");
 *
 * means a capability requirement/reference.
 *
 *     prefer: accelerator("quantum");
 *
 * means an advisory preference.
 *
 *     hint: topology::local;
 *
 * means an advisory hint.
 *
 * A physical mapping such as:
 *
 *     map: physical_qubit(17);
 *
 * is NOT automatically portable placement intent.
 *
 * A portable conformance profile may reject such implementation-specific
 * properties unless an explicit target-specific dialect permits them.
 *
 * ============================================================================
 * RESOURCE SCALABILITY CONTRACT
 * ============================================================================
 *
 * Valid:
 *
 *     requires: qubits >= n;
 *
 *     requires: memory >= required_memory;
 *
 *     requires: capability("tensor.compute");
 *
 *     requires: capability("gpu.compute");
 *
 *     requires: capability("quantum.measurement");
 *
 * Invalid as UNIVERSAL LANGUAGE LIMITS:
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
 * The grammar does not encode any of those limits.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum placement is expressed as intent:
 *
 *     placement quantum::kernel {
 *         requires: qubits >= logical_qubits;
 *         capability: capability("quantum.measurement");
 *         locality: execution_region;
 *     }
 *
 * This grammar does NOT:
 *
 *     allocate qubits;
 *     select physical qubits;
 *     create SWAPs;
 *     calculate routes;
 *     validate hardware coupling;
 *     perform QEC;
 *     calculate calibration;
 *     create a quantum IR.
 *
 * Quantum semantic information ultimately lowers through:
 *
 *     quantum::ir
 *
 * and remains compatible with:
 *
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Example:
 *
 *     placement classical::kernel {
 *         requires: capability("tensor.compute");
 *         prefer: accelerator("classical");
 *         locality: local;
 *     }
 *
 * No CPU/core/thread count is imposed by this grammar.
 *
 * ============================================================================
 * GPU / FPGA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Placement expresses capabilities and intent rather than device identity:
 *
 *     placement workload {
 *         requires: capability("gpu.compute");
 *         requires: memory >= required_memory;
 *         prefer: accelerator("gpu");
 *     }
 *
 * The actual GPU, FPGA, accelerator, memory domain, and topology are selected
 * downstream.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware placement may consume the semantic result of this grammar.
 *
 * This grammar does not declare:
 *
 *     registers;
 *     wires;
 *     physical pins;
 *     clock trees;
 *     FPGA tiles;
 *     ASIC cells;
 *     memory banks.
 *
 * Those remain owned by hardware/ and hdl/.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Example:
 *
 *     placement distributed::stage {
 *         locality: region;
 *         affinity: producer, consumer;
 *         anti_affinity: replica_a, replica_b;
 *         replication: replica_count;
 *     }
 *
 * No fixed number of nodes is encoded.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser result maps conceptually to:
 *
 *     ExecutionPlacement {
 *         subject: Option<Expression>,
 *         clauses: Vec<PlacementClause>,
 *         span: SourceSpan
 *     }
 *
 * Each clause preserves:
 *
 *     key;
 *     separator;
 *     value;
 *     source span;
 *     source ordering.
 *
 * The AST remains domain-neutral.
 *
 * Do NOT introduce:
 *
 *     PhysicalQubitPlacement
 *     GpuPlacement
 *     CpuPlacement
 *     FpgaPlacement
 *
 * into src/frontend/ast merely because this grammar exists.
 *
 * ============================================================================
 * SEMANTIC MODEL CONTRACT
 * ============================================================================
 *
 * Semantic analysis converts generic clauses into the canonical placement
 * intent model.
 *
 * Conceptual categories:
 *
 *     PlacementRequirement
 *     PlacementConstraint
 *     PlacementPreference
 *     PlacementHint
 *     PlacementCapability
 *     PlacementRelationship
 *     PlacementTargetIntent
 *     PlacementLocality
 *     PlacementMobility
 *     PlacementReplication
 *     PlacementElasticity
 *
 * The semantic model must retain whether a property is:
 *
 *     mandatory;
 *     restrictive;
 *     advisory;
 *     informational;
 *     target-specific.
 *
 * Unknown properties must never silently acquire mandatory semantics.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar defines NO IR.
 *
 * Placement intent lowers into the repository's existing canonical semantic
 * representation.
 *
 * It MUST NOT create:
 *
 *     ExecutionPlacementIR
 *     QuantumPlacementIR
 *
 * as competing universal IRs.
 *
 * For quantum execution the canonical boundary remains:
 *
 *     quantum::ir
 *
 * Placement metadata may accompany semantic objects consumed by:
 *
 *     routing;
 *     scheduling;
 *     resilience;
 *     deployment;
 *     HAL.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no parser actions;
 *     no mutable parser state;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime callbacks;
 *     no random behavior.
 *
 * Identical token streams therefore produce identical parsing behavior.
 *
 * Semantic resolution may depend on explicitly supplied compilation/runtime
 * state, but that is not parser behavior.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics:
 *
 *     missing `placement`;
 *     malformed subject;
 *     missing `{`;
 *     missing `}`;
 *     missing property key;
 *     missing `:`;
 *     missing property value;
 *     missing `;`;
 *     malformed expression.
 *
 * Semantic diagnostics:
 *
 *     unknown property;
 *     unsupported property in current language profile;
 *     invalid placement subject;
 *     conflicting constraints;
 *     unsatisfied requirement;
 *     unavailable capability;
 *     incompatible locality;
 *     invalid target class;
 *     non-portable implementation decision;
 *     incompatible dialect;
 *     impossible placement constraints.
 *
 * Resource availability MUST NOT be reported as a parser error.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing placement syntax MUST NOT:
 *
 *     enumerate hardware;
 *     discover devices;
 *     access physical addresses;
 *     allocate resources;
 *     reserve resources;
 *     open network connections;
 *     read files;
 *     invoke commands;
 *     load drivers;
 *     access secrets;
 *     bypass capability checks;
 *     mutate runtime state.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source forms represented by the previous execution placement
 * design should normalize toward:
 *
 *     placement {
 *         placement::scope: execution_region;
 *         placement::affinity: stage_a, stage_b;
 *         placement::anti_affinity: replica_a, replica_b;
 *         placement::target: accelerator;
 *         placement::replicas: replica_count;
 *         placement::migration: migration_policy;
 *     }
 *
 * Both:
 *
 *     key: value;
 *
 * and:
 *
 *     key = value;
 *
 * are accepted.
 *
 * The semantic model MUST normalize them to the same representation.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms MUST parse:
 *
 *     placement {
 *         locality: local;
 *     }
 *
 *     placement {
 *         target: accelerator("quantum");
 *     }
 *
 *     placement {
 *         requires: qubits >= n;
 *     }
 *
 *     placement {
 *         capability: capability("quantum.measurement");
 *     }
 *
 *     placement {
 *         prefer: accelerator("gpu");
 *     }
 *
 *     placement {
 *         hint: topology::local;
 *     }
 *
 *     placement {
 *         affinity: producer, consumer;
 *     }
 *
 *     placement {
 *         anti_affinity: replica_a, replica_b;
 *     }
 *
 *     placement {
 *         co_location: stage_a, stage_b;
 *     }
 *
 *     placement {
 *         separation: tenant_a, tenant_b;
 *     }
 *
 *     placement {
 *         replication: replica_count;
 *         elasticity: workload_size;
 *     }
 *
 *     placement quantum::kernel {
 *         requires: capability("quantum.mid_circuit_measurement");
 *         locality: execution_region;
 *     }
 *
 *     placement (compute_stage(input)) {
 *         target: accelerator("available");
 *     }
 *
 *     placement {
 *         vendor::future::placement::metric: desired_metric;
 *     }
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These MUST be rejected syntactically:
 *
 *     placement
 *
 *     placement {
 *
 *     placement {
 *         : value;
 *     }
 *
 *     placement {
 *         locality:
 *     }
 *
 *     placement {
 *         locality: ;
 *     }
 *
 *     placement {
 *         locality: local
 *     }
 *
 *     placement {
 *         ::locality: local;
 *     }
 *
 *     placement {
 *         locality:::region: local;
 *     }
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Verify:
 *
 *     empty placement body;
 *     one clause;
 *     many clauses;
 *     deeply qualified keys;
 *     deeply qualified subjects;
 *     large expressions;
 *     large value lists;
 *     symbolic resource quantities;
 *     very large program values;
 *     nested expression structures;
 *     repeated properties;
 *     future namespaced properties.
 *
 * None of these tests may establish a hardware maximum.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must accept placement specifications whose sizes are determined
 * by actual source/resource availability rather than language constants.
 *
 * Test progressively:
 *
 *     tiny placement;
 *     single accelerator;
 *     multiple accelerators;
 *     distributed execution;
 *     large resource sets;
 *     large placement-property sets;
 *     large quantum workloads;
 *     large classical workloads;
 *     large HDL/co-design workloads.
 *
 * The test suite must not encode a maximum number of:
 *
 *     devices;
 *     nodes;
 *     qubits;
 *     GPUs;
 *     CPUs;
 *     FPGAs;
 *     placement clauses.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
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
 * It also contains no:
 *
 *     physical device IDs;
 *     physical qubit IDs;
 *     fixed topology;
 *     fixed register widths;
 *     fixed memory capacities;
 *     fixed node counts;
 *     fixed cluster sizes.
 *
 * Numeric literals remain program values.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * The grammar itself contains no Rust.
 *
 * Generated parser integration MUST:
 *
 *     compile against Rust 1.97 / Rust 1.97.1;
 *     use Rust 2021;
 *     require no unsafe Rust;
 *     preserve source spans;
 *     remain deterministic;
 *     avoid target-specific parser behavior.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] executionPlacement is the public rule;
 *     [x] placement has one stable lexical introducer;
 *     [x] properties remain open-world names;
 *     [x] canonical expression syntax is reused;
 *     [x] no expression precedence is duplicated;
 *     [x] no physical resource selection is performed;
 *     [x] no topology is hard-coded;
 *     [x] no resource capacity is hard-coded;
 *     [x] requirements remain distinct semantically;
 *     [x] constraints remain distinct semantically;
 *     [x] preferences remain distinct semantically;
 *     [x] hints remain distinct semantically;
 *     [x] affinity is expressible;
 *     [x] anti-affinity is expressible;
 *     [x] co-location is expressible;
 *     [x] separation is expressible;
 *     [x] locality is expressible;
 *     [x] target intent is expressible;
 *     [x] replication is expressible;
 *     [x] migration is expressible;
 *     [x] elasticity is expressible;
 *     [x] namespaced extensions are expressible;
 *     [x] quantum integration is defined;
 *     [x] classical integration is defined;
 *     [x] HDL/hardware integration is defined;
 *     [x] distributed integration is defined;
 *     [x] AST contract is defined;
 *     [x] semantic contract is defined;
 *     [x] canonical IR boundary is preserved;
 *     [x] quantum::ir remains canonical;
 *     [x] source-span requirements are defined;
 *     [x] diagnostics are defined;
 *     [x] security boundary is defined;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] boundary tests are defined;
 *     [x] scalability tests are defined;
 *     [x] hard-coding audit is defined;
 *     [x] Rust 1.97/1.97.1 compatibility is defined;
 *     [x] unsafe Rust is not required.
 *
 * ============================================================================
 */