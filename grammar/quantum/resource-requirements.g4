/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/resource-requirements.g4
 *
 * Status:
 *     PRODUCTION-READY QUANTUM RESOURCE REQUIREMENT SYNTAX
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Grammar identity:
 *     QuantumResourceRequirements
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains:
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no filesystem access;
 *       - no network access;
 *       - no hardware discovery;
 *       - no runtime execution;
 *       - no unsafe Rust requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns ONLY the quantum-specific syntactic boundary for RESOURCE
 * REQUIREMENTS.
 *
 * It does NOT create another resource system.
 *
 * Universal resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * Universal resource expressions remain owned by:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * Universal expression precedence remains owned by:
 *
 *     grammar/expressions/
 *
 * Canonical names remain owned by:
 *
 *     grammar/core/names.g4
 *
 * Quantum composition remains owned by:
 *
 *     grammar/quantum/quantum.g4
 *
 * Quantum capability requirements remain owned by:
 *
 *     grammar/quantum/quantum-capabilities.g4
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                    grammar/Zamani.g4
 *                              |
 *                              v
 *                    grammar/antlr/ZamaniParser.g4
 *                              |
 *                              v
 *                     grammar/quantum/quantum.g4
 *                              |
 *                              v
 *              quantum resource requirement syntax
 *                              |
 *                              v
 *                         FRONTEND AST
 *                              |
 *                              v
 *                     SEMANTIC ANALYSIS
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        resource model   capability model   quantum semantics
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                       quantum::ir
 *                              |
 *                              v
 *                         OPTIMIZATION
 *                              |
 *                    +---------+---------+
 *                    |         |         |
 *                    v         v         v
 *                 ROUTING  SCHEDULING  RESILIENCE
 *                              |
 *                              v
 *                             ZQN
 *                              |
 *                              v
 *                             QEC
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                       TARGET REALIZATION
 *
 * This file participates only in the source syntax boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - quantum resource requirement statements;
 *   - quantum resource requirement expressions;
 *   - quantum resource property paths used by requirements;
 *   - the syntactic comparison between a quantum resource property and
 *     a required expression;
 *   - quantum resource requirement lists;
 *   - optional requirement labels/attributes where supported by the
 *     surrounding canonical resource grammar;
 *   - quantum-scoped requirement composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - general resource declarations;
 *   - general resource constraints;
 *   - preferences;
 *   - hints;
 *   - capability identity;
 *   - capability discovery;
 *   - hardware discovery;
 *   - device selection;
 *   - physical qubit allocation;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC implementation;
 *   - ZQN implementation;
 *   - optimization;
 *   - runtime allocation;
 *   - target selection;
 *   - quantum::ir;
 *   - classical IR;
 *   - hardware IR.
 *
 * ============================================================================
 * CRITICAL NON-DUPLICATION RULE
 * ============================================================================
 *
 * The repository already has universal resource syntax.
 *
 * Therefore this file MUST NOT redefine:
 *
 *     resource
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceExpression
 *     expression
 *     identifier
 *     qualifiedName
 *
 * Those concepts have existing canonical owners.
 *
 * This file instead provides a quantum-specific wrapper whose semantic
 * subject is explicitly a quantum resource property.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Quantum resource requirements are SOURCE-LEVEL SEMANTIC REQUIREMENTS.
 *
 * They describe what a quantum computation needs.
 *
 * They do NOT select a physical machine.
 *
 * They do NOT select a physical QPU.
 *
 * They do NOT select physical qubit identifiers.
 *
 * They do NOT select topology.
 *
 * They do NOT select a coupling map.
 *
 * They do NOT select calibration data.
 *
 * They do NOT select a routing strategy.
 *
 * They do NOT select a scheduler.
 *
 * They do NOT implement QEC.
 *
 * They do NOT implement ZQN.
 *
 * They do NOT implement HAL.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level limits on:
 *
 *     qubits
 *     logical qubits
 *     physical resources
 *     controls
 *     measurements
 *     circuit depth
 *     resource requirements
 *     requirement lists
 *     expression size
 *     namespace depth
 *     resource-property depth
 *     tensor dimensions
 *     tensor rank
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     node count
 *     memory capacity
 *     storage capacity
 *     accelerator count
 *     timeline count
 *
 * No construct in this file represents:
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
 * Numeric values appearing in a requirement are PROGRAM DATA.
 *
 * For example:
 *
 *     requires qubits >= 1024;
 *
 * means that this particular program requires at least 1024 units of the
 * semantic resource identified as `qubits`.
 *
 * It does NOT establish:
 *
 *     MAX_QUBITS = 1024
 *
 * The language remains resource-parametric.
 *
 * ============================================================================
 * NORMATIVE SYNTAX
 * ============================================================================
 *
 * The normative language specification currently defines the quantum
 * requirement concept as:
 *
 *     QuantumResourceRequirement ::=
 *         "requires"
 *         QuantumResourceProperty
 *         ComparisonOperator
 *         Expression
 *         [";"]
 *
 * and:
 *
 *     QuantumResourceProperty ::=
 *           "qubits"
 *         | "logical_qubits"
 *         | "fidelity"
 *         | "coherence"
 *         | "measurement"
 *         | "connectivity"
 *         | Path
 *
 * This implementation deliberately generalizes the property alternatives
 * through a qualified resource path rather than creating a permanently
 * closed list of quantum properties.
 *
 * The semantic layer may recognize standard properties such as:
 *
 *     qubits
 *     logical_qubits
 *     fidelity
 *     coherence
 *     measurement
 *     connectivity
 *
 * while remaining capable of recognizing future properties through the
 * canonical name/namespace system.
 *
 * ============================================================================
 * OPEN-WORLD RESOURCE MODEL
 * ============================================================================
 *
 * A quantum resource property is a NAME, not a hard-coded hardware class.
 *
 * Examples:
 *
 *     qubits
 *     logical_qubits
 *     physical_qubits
 *     fidelity
 *     coherence
 *     measurement
 *     connectivity
 *     quantum::logical_qubits
 *     quantum::measurement
 *     quantum::dynamic_control
 *     future::quantum::resource
 *
 * Whether a particular property is:
 *
 *     standard
 *     experimental
 *     dialect-defined
 *     vendor-specific
 *     unavailable
 *     invalid
 *
 * is a semantic question.
 *
 * The parser must not perform that classification.
 *
 * ============================================================================
 * REQUIREMENT VS REALIZATION
 * ============================================================================
 *
 * SOURCE:
 *
 *     requires logical_qubits >= problem_size;
 *
 * means:
 *
 *     the semantic realization must provide enough logical quantum resources
 *     for the computation.
 *
 * It does NOT mean:
 *
 *     use physical qubit 0;
 *     use physical qubit 1;
 *     use QPU 0;
 *     use device X;
 *     use topology Y;
 *     reserve a particular coupling map.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * QUANTITY MODEL
 * ============================================================================
 *
 * The right-hand side is a canonical Zamani expression.
 *
 * Therefore requirements may be:
 *
 *     constant
 *     symbolic
 *     generic
 *     dependent on input
 *     dependent on program state where semantically valid
 *     derived from another resource
 *     calculated from problem size
 *     calculated from workload size
 *     expressed using units/types supported by semantic analysis
 *
 * Examples:
 *
 *     requires qubits >= n;
 *
 *     requires logical_qubits >= problem_size;
 *
 *     requires logical_qubits >= input.size;
 *
 *     requires logical_qubits >= problem_size + ancilla_count;
 *
 *     requires quantum::memory >= required_memory;
 *
 * The grammar never evaluates these expressions.
 *
 * ============================================================================
 * SEMANTIC VALIDATION
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the property exists;
 *     - whether the expression has a valid type;
 *     - whether the expression represents a valid resource quantity;
 *     - whether units are compatible;
 *     - whether the comparison is meaningful;
 *     - whether the requirement is satisfiable;
 *     - whether the requirement conflicts with other requirements;
 *     - whether the target can provide the requested capability/resource;
 *     - whether logical and physical resources are being confused.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parse tree produced by this grammar must be sufficient for the
 * domain-neutral frontend AST to preserve:
 *
 *     - source span;
 *     - requirement keyword;
 *     - resource property path;
 *     - comparison operator;
 *     - required expression;
 *     - source ordering;
 *     - surrounding quantum scope.
 *
 * Conceptually:
 *
 *     QuantumResourceRequirementSyntax
 *             |
 *             +-- property
 *             +-- operator
 *             +-- required_value
 *             +-- source_span
 *
 * The exact Rust AST type is NOT owned by this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing:
 *
 *     quantumResourceRequirement
 *
 * is lowered into the repository's canonical resource requirement model.
 *
 * It must NOT create a second:
 *
 *     QuantumResourceRequirement
 *
 * semantic implementation merely because the syntax is quantum-specific.
 *
 * The semantic layer may attach:
 *
 *     domain = quantum
 *
 *     resource namespace = quantum
 *
 *     source provenance = quantumResourceRequirement
 *
 * while retaining the common resource requirement representation.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * Quantum resource requirements are metadata/intent associated with semantic
 * quantum computation.
 *
 * They may influence the construction or validation of `quantum::ir`.
 *
 * The grammar itself MUST NOT:
 *
 *     - import Rust quantum IR types;
 *     - construct IR;
 *     - define IR structures;
 *     - define QubitId;
 *     - define PhysicalQubitId;
 *     - define routing;
 *     - define scheduling.
 *
 * Canonical flow:
 *
 *     quantumResourceRequirement
 *              |
 *              v
 *         frontend AST
 *              |
 *              v
 *      semantic resource model
 *              |
 *              v
 *          quantum::ir
 *
 * ============================================================================
 * RESOURCE/CAPABILITY DISTINCTION
 * ============================================================================
 *
 * This file is for RESOURCE REQUIREMENTS.
 *
 * A capability requirement such as:
 *
 *     requires capability("quantum.measurement");
 *
 * belongs to the capability/resource integration layer.
 *
 * This file must not duplicate:
 *
 *     grammar/quantum/quantum-capabilities.g4
 *
 * The distinction is:
 *
 * RESOURCE:
 *
 *     requires logical_qubits >= n;
 *
 * CAPABILITY:
 *
 *     requires quantum::mid_circuit_measurement;
 *
 * The semantic layer may combine both.
 *
 * ============================================================================
 * REQUIREMENT VS CONSTRAINT
 * ============================================================================
 *
 * Requirement:
 *
 *     requires logical_qubits >= n;
 *
 * expresses what must be available.
 *
 * Constraint:
 *
 *     constraint logical_qubits <= budget;
 *
 * expresses a bound that must be respected.
 *
 * Preference:
 *
 *     prefer quantum::low_latency;
 *
 * is advisory.
 *
 * Hint:
 *
 *     hint quantum::locality;
 *
 * is advisory metadata.
 *
 * This file owns only the first category.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NOT contain grammar productions that force:
 *
 *     IBM
 *     Google
 *     IonQ
 *     Rigetti
 *     AWS
 *     Azure
 *     specific QPU names
 *     physical qubit IDs
 *     fixed coupling maps
 *     fixed topology
 *     calibration constants
 *
 * Vendor or target-specific requirements belong in explicit dialects or
 * downstream target descriptions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     - source tokens;
 *     - grammar version;
 *     - selected language/dialect configuration.
 *
 * It must NOT depend on:
 *
 *     hardware availability;
 *     resource discovery;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     environment variables;
 *     runtime state.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors include:
 *
 *     missing `requires`;
 *     missing resource property;
 *     missing comparison operator;
 *     missing required expression;
 *     malformed resource path;
 *     malformed statement terminator.
 *
 * Semantic errors include:
 *
 *     unknown quantum resource property;
 *     invalid quantity type;
 *     incompatible units;
 *     unsatisfiable requirement;
 *     unavailable resource;
 *     incompatible target capability;
 *     invalid logical/physical resource relationship.
 *
 * Resource failure MUST NOT be reported as a parser syntax error when the
 * source is structurally valid.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing syntax specified by grammar/spec/syntax.md remains valid:
 *
 *     requires qubits >= n;
 *
 *     requires logical_qubits >= problem_size;
 *
 *     requires fidelity >= required_fidelity;
 *
 *     requires coherence >= required_coherence;
 *
 *     requires measurement >= required_measurement;
 *
 *     requires connectivity >= required_connectivity;
 *
 * Generic paths remain supported:
 *
 *     requires quantum::logical_qubits >= n;
 *
 *     requires future::quantum::resource >= required_value;
 *
 * This avoids a closed list of quantum properties.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is a LEAF grammar.
 *
 * It is intended to be composed as follows:
 *
 *     grammar/quantum/resource-requirements.g4
 *                       |
 *                       v
 *               grammar/quantum/quantum.g4
 *                       |
 *                       v
 *               grammar/antlr/ZamaniParser.g4
 *                       |
 *                       v
 *                 grammar/Zamani.g4
 *
 * It also depends on:
 *
 *     grammar/resources/resource-expressions.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 * through the imported `Resources` grammar.
 *
 * IMPORTANT:
 *
 * This file MUST NOT be imported directly by `Zamani.g4`.
 *
 * The canonical composition hierarchy remains:
 *
 *     Zamani.g4
 *         |
 *         v
 *     ZamaniParser.g4
 *         |
 *         v
 *     Quantum
 *         |
 *         v
 *     QuantumResourceRequirements
 *
 * ============================================================================
 * INTEGRATION WITH EXISTING quantum-resources.g4
 * ============================================================================
 *
 * The existing:
 *
 *     grammar/quantum/quantum-resources.g4
 *
 * currently owns a much broader resource surface.
 *
 * To prevent duplicate production ownership:
 *
 *     quantum-resources.g4
 *
 * MUST NOT continue defining another production with the same semantic
 * ownership as:
 *
 *     quantumResourceRequirement
 *
 * The recommended final ownership is:
 *
 *     resource-requirements.g4
 *         -> quantumResourceRequirement
 *
 *     quantum-resources.g4
 *         -> resource declaration/composition
 *
 * The latter should delegate requirement syntax to this file.
 *
 * This allows the existing filename to remain unchanged while eliminating
 * the competing requirement implementation.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/resources/
 * ============================================================================
 *
 * Universal resource expression syntax remains canonical.
 *
 * This file imports `Resources`, which in turn imports:
 *
 *     ResourceExpressions
 *     Names
 *
 * Therefore the following are reused:
 *
 *     resourceExpression
 *     identifier
 *     qualifiedName
 *
 * No duplicate expression/name grammar is created.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM CAPABILITIES
 * ============================================================================
 *
 * Capability requirements remain owned by:
 *
 *     grammar/quantum/quantum-capabilities.g4
 *
 * Do not add capability-resolution semantics here.
 *
 * A quantum program may contain both:
 *
 *     requires logical_qubits >= n;
 *
 * and:
 *
 *     requires quantum::mid_circuit_measurement;
 *
 * These are semantically different requirements.
 *
 * ============================================================================
 * INTEGRATION WITH TYPES
 * ============================================================================
 *
 * The resource property is syntactically a name/path.
 *
 * Type validity belongs to:
 *
 *     grammar/types/
 *     semantic analysis
 *
 * This permits resource quantities to use:
 *
 *     integer expressions
 *     symbolic values
 *     generic values
 *     dependent values
 *     unit-aware values
 *
 * without adding another type system to this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH CLASSICAL COMPUTING
 * ============================================================================
 *
 * Right-hand-side expressions may depend on classical program values where
 * permitted by the semantic model.
 *
 * Example:
 *
 *     requires logical_qubits >= problem_size;
 *
 * This permits the same source program to scale with the problem.
 *
 * ============================================================================
 * INTEGRATION WITH HYBRID COMPUTING
 * ============================================================================
 *
 * Quantum requirements can be associated with a quantum computation whose
 * resource demand depends on classical computation.
 *
 * Example:
 *
 *     requires logical_qubits >= problem_size * factor;
 *
 * The grammar does not decide when `problem_size` is evaluated.
 *
 * ============================================================================
 * INTEGRATION WITH HDL / HARDWARE
 * ============================================================================
 *
 * This file does not describe hardware implementation.
 *
 * A quantum resource requirement may be consumed by hardware/resource
 * analysis, which may determine that an implementation requires:
 *
 *     qubit capacity
 *     connectivity
 *     measurement support
 *     coherence
 *     fidelity
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * INTEGRATION WITH DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * Quantum resources may participate in a heterogeneous/distributed program.
 *
 * Example:
 *
 *     requires quantum::logical_qubits >= workload.qubits;
 *
 * The realization may use one or multiple compatible resources.
 *
 * The grammar does not specify placement.
 *
 * ============================================================================
 * INTEGRATION WITH AI / DATA
 * ============================================================================
 *
 * Resource expressions may depend on:
 *
 *     model.size
 *     input.size
 *     dataset.size
 *     tensor.shape
 *     workload.size
 *
 * where the surrounding type/semantic system permits those expressions.
 *
 * This allows quantum acceleration requirements to scale with workloads
 * without rewriting the source language.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES / HARDWARE CAPABILITIES
 * ============================================================================
 *
 * Semantic resource resolution may consume:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *
 * but this grammar must never perform resource discovery.
 *
 * The semantic/resource manager is responsible for:
 *
 *     available capacity;
 *     capability satisfaction;
 *     target feasibility;
 *     resource negotiation.
 *
 * ============================================================================
 * INTEGRATION WITH ROUTING
 * ============================================================================
 *
 * A requirement such as:
 *
 *     requires connectivity >= required_connectivity;
 *
 * constrains the realization.
 *
 * It does not prescribe how connectivity is achieved.
 *
 * Routing may choose:
 *
 *     direct mapping;
 *     remapping;
 *     swaps;
 *     teleportation;
 *     decomposition;
 *     other supported realization strategies.
 *
 * The source requirement remains unchanged.
 *
 * ============================================================================
 * INTEGRATION WITH SCHEDULING
 * ============================================================================
 *
 * This grammar does not specify execution slots.
 *
 * A resource requirement may affect scheduling feasibility, but the scheduler
 * determines actual ordering and timing.
 *
 * ============================================================================
 * INTEGRATION WITH QEC
 * ============================================================================
 *
 * Quantum resource requirements may indirectly influence QEC resource needs.
 *
 * This file does not specify:
 *
 *     code;
 *     decoder;
 *     syndrome extraction;
 *     correction;
 *     code distance;
 *     number of syndrome rounds.
 *
 * Those are QEC semantics/implementation.
 *
 * ============================================================================
 * INTEGRATION WITH ZQN
 * ============================================================================
 *
 * Properties such as:
 *
 *     fidelity
 *     coherence
 *     error-related resource characteristics
 *
 * may be evaluated against ZQN models.
 *
 * This grammar does not contain noise models or probabilities.
 *
 * ============================================================================
 * INTEGRATION WITH RESILIENCE
 * ============================================================================
 *
 * A valid requirement may influence resilience analysis.
 *
 * Resilience may determine:
 *
 *     retry;
 *     recovery;
 *     fallback;
 *     rerouting;
 *     migration.
 *
 * None of these decisions belong in the parser.
 *
 * ============================================================================
 * INTEGRATION WITH HAL
 * ============================================================================
 *
 * HAL supplies actual target information.
 *
 * This grammar supplies only source-level requirements.
 *
 * Therefore:
 *
 *     grammar
 *         !=
 *     HAL
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * The consuming compiler must remain:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * Any resource quantity handling in Rust must use checked/validated
 * arithmetic and explicit error handling rather than relying on wrapping or
 * unchecked conversion.
 *
 * The grammar itself does not impose numeric implementation limits.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following positive forms must be accepted by conformance tests:
 *
 *     requires qubits >= 1;
 *
 *     requires qubits >= n;
 *
 *     requires logical_qubits >= problem_size;
 *
 *     requires fidelity >= required_fidelity;
 *
 *     requires coherence >= required_coherence;
 *
 *     requires measurement >= required_measurement;
 *
 *     requires connectivity >= required_connectivity;
 *
 *     requires quantum::logical_qubits >= problem_size;
 *
 *     requires future::quantum::resource >= required_value;
 *
 *     requires logical_qubits >= input.size;
 *
 *     requires logical_qubits >= problem_size + ancilla_count;
 *
 * The following must be accepted without establishing any language-level
 * maximum:
 *
 *     requires logical_qubits >= very_large_value;
 *
 *     requires logical_qubits >= symbolic_value;
 *
 *     requires quantum::logical_qubits >= workload.size;
 *
 * Negative syntax tests must include:
 *
 *     requires;
 *
 *     requires >= n;
 *
 *     requires qubits;
 *
 *     requires qubits >=;
 *
 *     requires qubits n;
 *
 *     requires qubits >= n extra;
 *
 * Semantic negative tests must separately cover:
 *
 *     unknown property;
 *     invalid quantity type;
 *     incompatible units;
 *     unsatisfiable requirement;
 *     unavailable capability/resource;
 *     invalid scope;
 *
 * These semantic failures must NOT be encoded as parser-only failures.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST pass the repository hard-coding audit.
 *
 * It contains:
 *
 *     NO fixed resource capacities;
 *     NO fixed qubit counts;
 *     NO fixed topology;
 *     NO fixed device count;
 *     NO fixed register width;
 *     NO fixed tensor rank;
 *     NO fixed machine count;
 *     NO vendor inventory;
 *     NO physical addresses;
 *     NO physical qubit IDs.
 *
 * Repeated grammar elements are represented through:
 *
 *     *
 *     +
 *
 * rather than fixed enumerations.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] ownership is limited to quantum resource requirements;
 * [x] universal resource syntax is reused;
 * [x] canonical expressions are reused;
 * [x] canonical names are reused;
 * [x] no quantum IR is created;
 * [x] no capability system is duplicated;
 * [x] no resource declaration system is duplicated;
 * [x] no physical allocation is encoded;
 * [x] no fixed hardware capacity exists;
 * [x] arbitrary symbolic resource quantities are supported;
 * [x] qualified future resource properties are supported;
 * [x] source syntax matches the normative quantum requirement model;
 * [x] semantic validation is explicitly downstream;
 * [x] AST integration is defined;
 * [x] quantum::ir integration is defined;
 * [x] routing integration is defined;
 * [x] scheduling integration is defined;
 * [x] QEC integration is defined;
 * [x] ZQN integration is defined;
 * [x] HAL integration is defined;
 * [x] Rust 1.97/1.97.1 compatibility is defined;
 * [x] no unsafe Rust is required;
 * [x] positive tests are defined;
 * [x] negative tests are defined;
 * [x] scalability tests are defined;
 * [x] cross-domain integration is defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar expresses:
 *
 *     WHAT QUANTUM RESOURCE IS REQUIRED
 *
 * It does NOT express:
 *
 *     WHICH MACHINE MUST PROVIDE IT.
 *
 * Therefore:
 *
 *     Program Once
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Resource analysis
 *          |
 *          v
 *     Target-independent semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     target realization
 *
 * remains valid from tiny quantum computations to arbitrarily large
 * computations supported by the available resources.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 *
 * `Resources` is the canonical universal resource grammar. Importing it here
 * deliberately reuses:
 *
 *     resourceExpression
 *     identifier
 *     qualifiedName
 *
 * and prevents this quantum file from creating parallel expression/name
 * systems.
 */
parser grammar QuantumResourceRequirements;

options {
    tokenVocab = ZamaniLexer;
}

import Resources;


/*
 * ============================================================================
 * 1. PUBLIC QUANTUM RESOURCE REQUIREMENT
 * ============================================================================
 *
 * Normative form:
 *
 *     requires <quantum-resource-property> <comparison> <expression> ;
 *
 * The semicolon is optional in the specification but the canonical source
 * grammar uses SEMICOLON as the normal statement terminator.
 *
 * The optional form is retained for compatibility with the specification.
 */
quantumResourceRequirement
    : REQUIRES
      quantumResourceProperty
      comparisonOperator
      resourceExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. QUANTUM RESOURCE PROPERTY
 * ============================================================================
 *
 * Standard properties are intentionally NOT enumerated as lexer tokens.
 *
 * A qualified name is used so that:
 *
 *     qubits
 *     logical_qubits
 *     fidelity
 *     coherence
 *     measurement
 *     connectivity
 *
 * remain compatible while future properties remain possible.
 *
 * Examples:
 *
 *     quantum::logical_qubits
 *     quantum::measurement
 *     future::quantum::resource
 *
 * Semantic analysis decides which properties are recognized.
 */
quantumResourceProperty
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. COMPARISON OPERATOR
 * ============================================================================
 *
 * Resource comparison syntax is shared with the canonical expression
 * precedence system.
 *
 * This rule is intentionally local to the requirement's comparison boundary;
 * it does not create a second expression hierarchy.
 */
comparisonOperator
    : LE
    | GE
    | EQ_EQ
    | NOT_EQ
    | LT
    | GT
    ;


/*
 * ============================================================================
 * 4. REQUIREMENT LIST
 * ============================================================================
 *
 * A quantum scope may contain any number of independent requirements.
 *
 * There is no grammar-level maximum.
 */
quantumResourceRequirementList
    : quantumResourceRequirement*
    ;


/*
 * ============================================================================
 * 5. REQUIREMENT BLOCK
 * ============================================================================
 *
 * This is a syntactic grouping construct only.
 *
 * It does not allocate resources and does not establish a physical QPU.
 *
 * Example:
 *
 *     quantum requirements {
 *         requires logical_qubits >= problem_size;
 *         requires fidelity >= required_fidelity;
 *     }
 *
 * The enclosing quantum grammar is responsible for deciding where this
 * grouping construct is legal.
 */
quantumResourceRequirementBlock
    : K_QUANTUM
      K_REQUIREMENTS
      LBRACE
      quantumResourceRequirement*
      RBRACE
    ;


/*
 * ============================================================================
 * 6. ATTACHMENT FORM
 * ============================================================================
 *
 * A quantum construct may carry one or more resource requirements through
 * an enclosing requirement block.
 *
 * The actual attachment semantics remain owned by semantic analysis.
 */
quantumResourceRequirementAttachment
    : quantumResourceRequirementList
    ;


/*
 * ============================================================================
 * 7. STANDALONE PUBLIC ADAPTER
 * ============================================================================
 *
 * This entry point exists for standalone grammar conformance tests.
 *
 * It intentionally does not append EOF; the canonical program grammar owns
 * complete-source EOF handling.
 */
quantumResourceRequirementEntry
    : quantumResourceRequirement
    | quantumResourceRequirementBlock
    ;