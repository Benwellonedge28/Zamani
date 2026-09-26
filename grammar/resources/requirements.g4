/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/requirements.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Grammar identity:
 *     Requirements
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Grammar-only.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No filesystem access.
 *     No network access.
 *     No hardware access.
 *     No runtime execution.
 *     No unsafe Rust requirement.
 *
 * ============================================================================
 * STATUS
 * ============================================================================
 *
 * CANONICAL RESOURCE-REQUIREMENT GRAMMAR
 *
 * This file is the single source-level owner of universal resource
 * requirement syntax.
 *
 * It is composed by:
 *
 *     grammar/resources/resources.g4
 *
 * and may be consumed by domain grammars such as:
 *
 *     grammar/hardware/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/hdl/
 *     grammar/execution/
 *     grammar/compile/
 *
 * Domain grammars MUST NOT create competing general-purpose resource
 * requirement grammars.
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
 *     parser
 *          |
 *          v
 *     Requirements
 *          |
 *          v
 *     ResourceExpressions
 *          |
 *          v
 *     canonical expression grammar
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     resource semantics             capability semantics
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                  canonical semantic model
 *                          |
 *             +------------+------------+
 *             |            |            |
 *             v            v            v
 *         classical     quantum::ir   HDL/hardware
 *             |            |            |
 *             +------------+------------+
 *                          |
 *                          v
 *                     optimization
 *                          |
 *                 +--------+--------+
 *                 |        |        |
 *                 v        v        v
 *              routing scheduling resilience
 *                 |        |        |
 *                 +--------+--------+
 *                          |
 *                         ZQN
 *                          |
 *                         HAL
 *                          |
 *                  target realization
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * A resource requirement describes WHAT a program requires.
 *
 * It does not decide WHICH physical resource will satisfy it.
 *
 * Examples:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability(
 *         "quantum.mid_circuit_measurement",
 *         operation
 *     );
 *
 * The compiler, resource system, runtime, HAL, and deployment infrastructure
 * determine how those requirements are satisfied.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar is part of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Requirements therefore remain:
 *
 *     - target-independent;
 *     - resource-parametric;
 *     - capability-driven;
 *     - open-world;
 *     - scalable;
 *     - semantically declarative.
 *
 * The grammar MUST NOT encode today's hardware as tomorrow's language limit.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define or imply:
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
 * It also MUST NOT encode equivalent fixed limits such as:
 *
 *     exactly 32 CPUs
 *     exactly 1024 qubits
 *     exactly 64 GB memory
 *     exactly 24 GB VRAM
 *     exactly 32-bit registers
 *
 * A numeric literal inside a requirement is a PROGRAM VALUE.
 *
 * For example:
 *
 *     requires qubits >= 1024;
 *
 * is valid portable source syntax.
 *
 * The grammar MUST NOT interpret 1024 as a universal maximum.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     resourceRequirement
 *     resourceRequirementExpression
 *     resourceCapabilityCall
 *     resourceRequirementList
 *     optionalResourceRequirementList
 *     requirement
 *     requirementList
 *
 * THIS FILE DOES NOT OWN:
 *
 *     expression
 *     arithmetic
 *     comparison precedence
 *     logical precedence
 *     identifiers
 *     qualified names
 *     literals
 *     types
 *     resource declarations
 *     resource constraints
 *     preferences
 *     hints
 *     resource discovery
 *     hardware discovery
 *     allocation
 *     placement
 *     routing
 *     scheduling
 *     optimization
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *     physical device selection
 *     quantum::ir
 *     classical IR
 *     HDL/hardware IR
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * ResourceExpressions owns the resource-expression composition boundary.
 *
 * It imports the canonical expression grammar.
 *
 * Therefore this file MUST consume:
 *
 *     resourceExpression
 *
 * rather than defining:
 *
 *     expression
 *     arithmeticExpression
 *     comparisonExpression
 *     logicalExpression
 *     unaryExpression
 *     primaryExpression
 *
 * itself.
 *
 * Names are imported explicitly because requirement-related semantic names
 * may be needed by downstream consumers and because canonical name syntax
 * must remain centralized.
 *
 * ============================================================================
 */

parser grammar Requirements;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * 1. CANONICAL RESOURCE REQUIREMENT
 * ============================================================================
 *
 * A resource requirement is mandatory semantic intent.
 *
 * Canonical form:
 *
 *     requires <resource-expression>;
 *
 * Examples:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires latency <= latency_budget;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("quantum.measurement");
 *
 * The trailing semicolon belongs to the requirement statement.
 *
 * This rule deliberately does not own EOF.
 *
 * EOF belongs to the canonical root grammar:
 *
 *     grammar/Zamani.g4
 *
 * ============================================================================
 */

resourceRequirement
    : REQUIRES resourceRequirementExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 2. REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * A requirement expression normally uses the canonical resource-expression
 * grammar.
 *
 * The capability-call alternative exists because CAPABILITY is a reserved
 * lexical token in Zamani and therefore:
 *
 *     capability("tensor.compute")
 *
 * cannot safely be treated as an ordinary identifier-based function call
 * once CAPABILITY has been tokenized as a keyword.
 *
 * This is a syntactic bridge only.
 *
 * Capability identity and capability satisfaction remain semantic concerns.
 * ============================================================================
 */

resourceRequirementExpression
    : resourceCapabilityCall
    | resourceExpression
    ;


/*
 * ============================================================================
 * 3. CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Canonical portable examples:
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("gpu.compute");
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability(
 *         "quantum.mid_circuit_measurement"
 *     );
 *
 *     requires capability(
 *         "quantum.operation",
 *         operation
 *     );
 *
 * The capability namespace is OPEN-WORLD.
 *
 * This grammar intentionally does not enumerate:
 *
 *     CPU capabilities
 *     GPU capabilities
 *     FPGA capabilities
 *     QPU capabilities
 *     vendor capabilities
 *     future accelerator capabilities
 *
 * Capability names are semantic data.
 *
 * ============================================================================
 */

resourceCapabilityCall
    : CAPABILITY
      LPAREN
      optionalResourceExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * 4. REQUIREMENT LIST
 * ============================================================================
 *
 * This rule provides a reusable composition boundary for consumers that need
 * several requirements as a syntactic collection.
 *
 * Cardinality is intentionally unbounded at the language level.
 *
 * There is no:
 *
 *     MAX_REQUIREMENTS
 *
 * and no fixed number of requirements is implied.
 *
 * Practical limits belong to compiler, runtime, deployment, or configured
 * resource budgets rather than to the language grammar.
 *
 * ============================================================================
 */

resourceRequirementList
    : resourceRequirement*
    ;


/*
 * ============================================================================
 * 5. OPTIONAL REQUIREMENT LIST
 * ============================================================================
 *
 * This wrapper is useful for resource declarations, profiles, contracts,
 * target specifications, and domain-specific resource sections.
 *
 * It does not introduce a second requirement representation.
 * ============================================================================
 */

optionalResourceRequirementList
    : resourceRequirementList?
    ;


/*
 * ============================================================================
 * 6. GENERIC REQUIREMENT ALIAS
 * ============================================================================
 *
 * `requirement` is a stable semantic-category wrapper.
 *
 * It intentionally resolves to the canonical resource requirement rather
 * than defining another requirement statement.
 *
 * Domain grammars may therefore consume:
 *
 *     requirement
 *
 * when they need a generic requirement category while preserving a single
 * concrete syntax owner.
 * ============================================================================
 */

requirement
    : resourceRequirement
    ;


/*
 * ============================================================================
 * 7. GENERIC REQUIREMENT LIST
 * ============================================================================
 *
 * Generic consumers may use this rule without creating another requirement
 * grammar.
 * ============================================================================
 */

requirementList
    : requirement*
    ;


/*
 * ============================================================================
 * 8. RESOURCE REQUIREMENT EXPRESSION LIST
 * ============================================================================
 *
 * ResourceExpressions already owns the canonical expression-list syntax.
 *
 * This rule is deliberately NOT redefined here.
 *
 * The imported rule:
 *
 *     resourceExpressionList
 *
 * is the canonical owner.
 *
 * The optional form used by resourceCapabilityCall is also provided by the
 * imported ResourceExpressions grammar:
 *
 *     optionalResourceExpressionList
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 9. SEMANTIC DISTINCTION
 * ============================================================================
 *
 * The parser preserves the syntactic category:
 *
 *     resourceRequirement
 *
 * Semantic analysis determines the actual requirement meaning.
 *
 * A requirement may represent:
 *
 *     quantitative requirement
 *     qualitative requirement
 *     capability requirement
 *     property requirement
 *     performance requirement
 *     latency requirement
 *     throughput requirement
 *     bandwidth requirement
 *     memory requirement
 *     compute requirement
 *     quantum resource requirement
 *     classical resource requirement
 *     HDL/hardware requirement
 *     distributed requirement
 *     networking requirement
 *     AI/data requirement
 *     resilience requirement
 *
 * No finite semantic enumeration belongs in this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 10. REQUIREMENT VS CAPABILITY
 * ============================================================================
 *
 * These concepts are deliberately not conflated.
 *
 * REQUIREMENT:
 *
 *     requires qubits >= logical_qubits;
 *
 * CAPABILITY REQUIREMENT:
 *
 *     requires capability("quantum.measurement");
 *
 * RESOURCE PROPERTY REQUIREMENT:
 *
 *     requires memory >= required_memory;
 *
 * PERFORMANCE REQUIREMENT:
 *
 *     requires latency <= latency_budget;
 *
 * The semantic layer decides whether the requirement is satisfiable and what
 * target realization can satisfy it.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 11. REQUIREMENT VS PREFERENCE
 * ============================================================================
 *
 * This grammar intentionally owns mandatory requirements only.
 *
 * It does NOT define:
 *
 *     prefer
 *     hint
 *
 * Those belong to their respective resource grammar components.
 *
 * This separation prevents an advisory preference from accidentally becoming
 * a mandatory requirement.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. REQUIREMENT VS CONSTRAINT
 * ============================================================================
 *
 * A requirement expresses what must be available or satisfied.
 *
 * A constraint expresses a restriction on valid realization.
 *
 * They may use the same underlying resource-expression system but remain
 * distinct semantic categories.
 *
 * Constraint syntax belongs to:
 *
 *     grammar/resources/constraints.g4
 *
 * Requirement syntax belongs here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 13. RESOURCE QUANTITIES
 * ============================================================================
 *
 * Quantities are expressions.
 *
 * Therefore all of the following remain valid semantic possibilities:
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 *     requires tensor_rank >= required_rank;
 *
 *     requires lanes >= desired_parallelism;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires bandwidth >= required_bandwidth;
 *
 * The grammar does not impose a maximum on any of these quantities.
 *
 * Dimensional validity, units, overflow policy, precision, and target
 * feasibility belong to semantic/type/resource analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum-specific requirement grammars may wrap this universal requirement
 * grammar.
 *
 * They MUST NOT redefine:
 *
 *     resourceRequirement
 *     resourceRequirementExpression
 *     resourceCapabilityCall
 *     resourceExpression
 *
 * A quantum-specific property can therefore use the same universal form:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("quantum.mid_circuit_measurement");
 *
 *     requires capability("quantum.dynamic_control");
 *
 * The quantum frontend then lowers semantic quantum meaning through the
 * established:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar does not create a quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical domains may use the same requirement grammar:
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("vector.compute");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("parallel.compute");
 *
 * No CPU/core/thread count is encoded as a language-level maximum.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. GPU / FPGA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Target-independent source may express:
 *
 *     requires capability("gpu.compute");
 *
 *     requires capability("fpga.compute");
 *
 *     requires capability("accelerator.tensor");
 *
 *     requires capability("accelerator.reconfigurable");
 *
 * The grammar does not enumerate vendors, models, device IDs, or fixed
 * capacities.
 *
 * Target discovery and matching belong downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed programs may express:
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("distributed.execution");
 *
 *     requires capability("distributed.collectives");
 *
 *     requires capability("networking.low_latency");
 *
 * The grammar does not define a maximum number of nodes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware grammars may consume the generic requirement category.
 *
 * Examples:
 *
 *     requires capability("hardware.reconfiguration");
 *
 *     requires capability("hardware.streaming");
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     requires latency <= latency_budget;
 *
 * Hardware width, topology, timing, physical resources, and implementation
 * feasibility remain downstream semantic concerns.
 *
 * This grammar therefore does not contain constructs such as:
 *
 *     wire [31:0]
 *     MAX_REGISTER_WIDTH
 *     MAX_FPGA_RESOURCES
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. AI / DATA INTEGRATION
 * ============================================================================
 *
 * AI/data programs may express:
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("tensor.acceleration");
 *
 *     requires capability("distributed.training");
 *
 *     requires capability("data.streaming");
 *
 * Tensor rank, tensor dimensions, accelerator count, and available memory
 * remain semantic/resource values rather than grammar-level limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * The capability argument is deliberately an expression rather than a fixed
 * parser enumeration.
 *
 * This permits:
 *
 *     capability("quantum.measurement")
 *     capability("quantum.measurement", mode)
 *     capability("vendor.feature")
 *     capability(namespace::feature)
 *     capability(feature_name)
 *
 * subject to the canonical expression grammar.
 *
 * This design allows future capabilities to be introduced without changing
 * the universal requirement grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. NO PHYSICAL RESOURCE SELECTION
 * ============================================================================
 *
 * This grammar MUST NOT introduce source-level physical allocation such as:
 *
 *     CPU 0
 *     GPU 3
 *     physical_qubit 17
 *     node 42
 *     memory_bank 2
 *
 * Such realization-specific constructs, when genuinely necessary, belong to
 * explicitly target-specific or interoperability grammars and must not be
 * confused with portable resource requirements.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. NO TARGET LIMITS
 * ============================================================================
 *
 * A requirement such as:
 *
 *     requires qubits >= 1024;
 *
 * means:
 *
 *     the semantic workload requires at least 1024 qubits.
 *
 * It does NOT mean:
 *
 *     Zamani supports only 1024 qubits.
 *
 * Likewise:
 *
 *     requires memory >= required_memory;
 *
 * does not establish any universal memory maximum.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. SOURCE SPANS
 * ============================================================================
 *
 * Every requirement construct must preserve its complete source span through
 * the parser/AST boundary.
 *
 * At minimum, semantic diagnostics must be able to identify:
 *
 *     REQUIRES keyword
 *     requirement expression
 *     capability call, when present
 *     argument expressions
 *     terminating semicolon
 *
 * The exact source-span type belongs to the frontend AST contract.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. AST CONTRACT
 * ============================================================================
 *
 * This grammar maps conceptually to:
 *
 *     ResourceRequirement
 *         expression
 *         source_span
 *
 * and, when applicable:
 *
 *     CapabilityRequirement
 *         capability
 *         arguments
 *         source_span
 *
 * The grammar MUST NOT depend on concrete Rust AST type names.
 *
 * The AST must preserve enough information to distinguish:
 *
 *     ordinary resource requirement
 *
 * from:
 *
 *     capability requirement
 *
 * without losing the original expression structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     type checking
 *     unit/dimension checking
 *     capability identity
 *     capability availability
 *     resource feasibility
 *     resource negotiation
 *     target compatibility
 *     portability analysis
 *     diagnostics
 *
 * The parser MUST NOT attempt to determine whether:
 *
 *     qubits
 *     memory
 *     latency
 *     bandwidth
 *     nodes
 *     capability(...)
 *
 * can actually be satisfied.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. IR CONTRACT
 * ============================================================================
 *
 * Requirements do not create a second resource IR inside this grammar.
 *
 * After semantic analysis, requirement information is represented by the
 * repository's canonical semantic/resource model and then consumed by the
 * appropriate compiler/IR layers.
 *
 * Quantum requirements ultimately integrate with:
 *
 *     quantum::ir
 *
 * They do not create:
 *
 *     quantum_requirement_ir
 *
 * or another competing quantum representation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. ERROR CLASSIFICATION
 * ============================================================================
 *
 * Syntax errors:
 *
 *     requires;
 *     requires ();
 *     requires capability(;
 *
 * are parser errors.
 *
 * Semantic errors such as:
 *
 *     requires qubits >= -1;
 *
 * where the semantic type/domain makes the value invalid, belong to semantic
 * analysis.
 *
 * Capability-unsatisfied errors such as:
 *
 *     requires capability("quantum.measurement");
 *
 * on a target without that capability are resource/capability diagnostics,
 * not syntax errors.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no embedded actions;
 *     no random behavior;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime callbacks.
 *
 * Parsing is therefore determined entirely by the input token stream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. COMPATIBILITY
 * ============================================================================
 *
 * Existing source form:
 *
 *     requires <resource-expression>;
 *
 * remains the canonical requirement form.
 *
 * Existing consumers of `resourceRequirement` continue to use that rule.
 *
 * The production change is ownership consolidation:
 *
 *     requirements.g4
 *         owns resourceRequirement
 *
 *     resources.g4
 *         composes resourceRequirement
 *
 * rather than:
 *
 *     resources.g4
 *         redefining resourceRequirement
 *
 * This prevents competing definitions of the same semantic category.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. INTEGRATION WITH resources.g4
 * ============================================================================
 *
 * `grammar/resources/resources.g4` MUST import this grammar:
 *
 *     import ResourceExpressions, Names, Requirements;
 *
 * Its `resourceItem` rule should continue to expose:
 *
 *     resourceRequirement
 *
 * but `resources.g4` MUST remove its local definitions of:
 *
 *     resourceRequirement
 *     resourceRequirementExpression
 *     resourceCapabilityCall
 *
 * because those rules are owned here.
 *
 * The resulting composition is:
 *
 *     Resources
 *         |
 *         +--> Requirements
 *         |       |
 *         |       +--> ResourceExpressions
 *         |
 *         +--> ResourceExpressions
 *         |
 *         +--> Names
 *
 * This gives one owner per grammar concept.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. INTEGRATION WITH DOMAIN GRAMMARS
 * ============================================================================
 *
 * Domain grammars should consume the canonical requirement rule.
 *
 * Examples:
 *
 *     hardware
 *         -> resourceRequirement
 *
 *     quantum
 *         -> resourceRequirement
 *
 *     hybrid
 *         -> resourceRequirement
 *
 *     distributed
 *         -> resourceRequirement
 *
 *     AI
 *         -> resourceRequirement
 *
 *     HDL
 *         -> resourceRequirement
 *
 * Domain-specific wrappers are permitted.
 *
 * Example:
 *
 *     quantumResourceRequirement
 *         : resourceRequirement
 *         ;
 *
 * Such wrappers must add semantic context rather than duplicate the
 * requirement grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax cases:
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires latency <= latency_budget;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("gpu.compute");
 *
 *     requires capability(
 *         "quantum.operation",
 *         operation
 *     );
 *
 *     requires workload_size * element_size <= available_memory;
 *
 * Boundary/scalability cases:
 *
 *     requires qubits >= 0;
 *
 *     requires qubits >= 1;
 *
 *     requires qubits >= 1024;
 *
 *     requires qubits >= very_large_symbolic_value;
 *
 *     requires capability("a");
 *
 *     requires capability("vendor.feature", argument);
 *
 *     many sequential requirements;
 *
 *     deeply nested resource expressions;
 *
 *     large capability argument lists;
 *
 *     large source files containing many requirements.
 *
 * Negative syntax cases:
 *
 *     requires;
 *
 *     requires ;
 *
 *     requires capability;
 *
 *     requires capability(;
 *
 *     requires capability();
 *
 * Note:
 *
 *     Empty capability argument lists are syntactically accepted by the
 *     grammar if the repository chooses to treat capability() as a valid
 *     open-world call. Semantic analysis SHOULD reject it when a capability
 *     identity is required.
 *
 * Therefore, if the language specification requires a capability identity,
 * semantic validation—not a machine-size grammar restriction—must reject
 * capability().
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. HARD-CODING AUDIT
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
 * It contains no finite enumeration of:
 *
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     accelerators
 *     vendors
 *     machines
 *     nodes
 *     qubits
 *     memories
 *     tensor ranks
 *     network sizes
 *
 * Resource quantities remain expressions.
 *
 * Capability identities remain open-world expressions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a requirement must never:
 *
 *     execute a capability;
 *     query hardware;
 *     access the network;
 *     access the filesystem;
 *     invoke a vendor API;
 *     execute a runtime operation;
 *     allocate a resource.
 *
 * All such operations belong to later, explicitly controlled compiler/runtime
 * layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Generated parser integration must:
 *
 *     compile with Rust 1.97 / Rust 1.97.1;
 *     use safe Rust;
 *     require no unsafe blocks;
 *     require no unsafe functions;
 *     require no unsafe traits;
 *     preserve parser source spans;
 *     preserve deterministic behavior.
 *
 * No grammar construct may require an unsafe Rust implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] grammar identity is Requirements;
 *     [x] canonical ZamaniLexer vocabulary is consumed;
 *     [x] no placeholder REQUIREMENT_* tokens exist;
 *     [x] resourceExpression remains owned by ResourceExpressions;
 *     [x] identifier/name syntax remains centrally owned;
 *     [x] resourceRequirement has one owner;
 *     [x] capability requirements are supported;
 *     [x] capability identity is open-world;
 *     [x] arbitrary capability arguments are supported;
 *     [x] resource quantities remain expressions;
 *     [x] no physical hardware limit is encoded;
 *     [x] no target is selected by the parser;
 *     [x] no resource is allocated by the parser;
 *     [x] no quantum IR is duplicated;
 *     [x] quantum requirements can flow toward quantum::ir;
 *     [x] classical requirements are supported;
 *     [x] HDL/hardware requirements are supported;
 *     [x] distributed requirements are supported;
 *     [x] AI/data requirements are supported;
 *     [x] source spans are specified;
 *     [x] AST mapping is specified;
 *     [x] semantic mapping is specified;
 *     [x] IR integration is specified;
 *     [x] resources.g4 integration is specified;
 *     [x] domain integration is specified;
 *     [x] positive tests are specified;
 *     [x] negative tests are specified;
 *     [x] boundary tests are specified;
 *     [x] scalability tests are specified;
 *     [x] determinism is specified;
 *     [x] no-unsafe integration is specified.
 *
 * ============================================================================
 * END OF REQUIREMENTS GRAMMAR
 * ============================================================================
 */