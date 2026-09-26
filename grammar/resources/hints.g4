/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/hints.g4
 *
 * GRAMMAR
 * -------
 * ResourceHints
 *
 * BASELINE
 * --------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE-HINT LEAF GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical SOURCE-SYNTAX owner for resource hints.
 *
 * A hint is advisory information supplied to downstream compilation,
 * optimization, placement, scheduling, deployment, or runtime policy layers.
 *
 * A hint:
 *
 *     MAY improve a realization;
 *     MAY be ignored;
 *     MUST NOT silently become a requirement;
 *     MUST NOT silently become a constraint;
 *     MUST NOT silently become a capability;
 *     MUST NOT select a physical resource merely because it was parsed.
 *
 * The grammar describes the developer's advisory intent.
 *
 * It does NOT perform:
 *
 *     resource discovery;
 *     hardware discovery;
 *     allocation;
 *     placement;
 *     routing;
 *     scheduling;
 *     optimization;
 *     compilation;
 *     runtime execution;
 *     QEC;
 *     ZQN processing;
 *     HAL selection;
 *     target probing.
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
 *     Resources
 *          |
 *          v
 *     ResourceHints
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic resource model
 *          |
 *     +----+---------+-------------+----------------+
 *     |              |             |                |
 *     v              v             v                v
 *  compiler      optimizer     scheduler        runtime
 *     |              |             |                |
 *     +--------------+-------------+----------------+
 *                            |
 *                            v
 *                    target realization
 *
 * The grammar remains upstream of physical realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Resource hints are part of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Hints MUST therefore remain target-independent unless the programmer
 * explicitly expresses a target-specific semantic property.
 *
 * Examples:
 *
 *     hint locality = "near";
 *
 *     hint execution::parallelism = desired_parallelism;
 *
 *     hint memory::placement = preferred_memory_domain;
 *
 *     hint quantum::routing = routing_strategy;
 *
 *     hint accelerator::affinity = accelerator_preference;
 *
 *     hint vendor::future::optimization = optimization_value;
 *
 * None of these statements establishes:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     FPGA 0
 *     physical qubit 17
 *     node 4
 *     memory bank 2
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define universal limits such as:
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
 * It MUST also not introduce indirect grammar-level equivalents such as:
 *
 *     exactly 8 CPUs
 *     exactly 32 threads
 *     exactly 1024 qubits
 *     exactly 64 GB memory
 *     exactly 32-bit registers
 *
 * Numeric expressions appearing in hints are PROGRAM VALUES.
 *
 * For example:
 *
 *     hint parallelism = 1024;
 *
 * is a program-level advisory value.
 *
 * It does NOT mean:
 *
 *     MAX_THREADS = 1024
 *
 * Physical availability is resolved after parsing.
 *
 * ============================================================================
 * UNBOUNDED SCALABILITY
 * ============================================================================
 *
 * The grammar deliberately uses:
 *
 *     *
 *     +
 *     recursive composition
 *
 * wherever arbitrary source cardinality is required.
 *
 * There is no language-level maximum for:
 *
 *     hints;
 *     hint clauses;
 *     hint properties;
 *     property namespace depth;
 *     hint groups;
 *     hint metadata;
 *     expressions;
 *     targets;
 *     resources;
 *     resource domains;
 *     nested hint structures.
 *
 * "Infinity" in the POCO-REAF requirement means:
 *
 *     no artificial hardware/resource ceiling is encoded by this grammar.
 *
 * It does NOT claim physically infinite memory, compute, or execution time.
 *
 * Actual implementation limits belong to the compiler, runtime, operating
 * system, deployment environment, and physical target.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     resourceHint
 *     resourceHintExpression
 *     resourceHintSpecification
 *     resourceHintClause
 *     resourceHintPropertyAssignment
 *     resourceHintProperty
 *     qualifiedResourceHintName
 *     resourceHintNameSegment
 *     resourceHintValue
 *     resourceHintCondition
 *     resourceHintConditionValue
 *     resourceHintScope
 *     resourceHintScopeValue
 *     resourceHintTarget
 *     resourceHintTargetValue
 *     resourceHintMetadata
 *     resourceHintMetadataName
 *     resourceHintGroup
 *     resourceHintGroupBody
 *     resourceHintGroupEntry
 *     resourceHintList
 *     optionalResourceHintList
 *
 * THIS FILE DOES NOT OWN:
 *
 *     resourceItem
 *     resourceExpression
 *     resourceExpressionList
 *     expression
 *     identifier
 *     qualifiedName
 *     requirement semantics
 *     constraint semantics
 *     preference semantics
 *     capability semantics
 *     resource discovery
 *     target selection
 *     placement
 *     routing
 *     scheduling
 *     optimization
 *     hardware discovery
 *     QEC
 *     ZQN
 *     HAL
 *     runtime resource management
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/resources/resources.g4
 *     owns the resource-domain composition boundary.
 *
 * It MUST import this grammar:
 *
 *     import ResourceHints, ...;
 *
 * and MUST delegate:
 *
 *     resourceHint
 *     resourceHintExpression
 *     resourceHintClause
 *
 * to this grammar.
 *
 * resources.g4 MUST NOT redefine those rules.
 *
 * grammar/resources/resource-expressions.g4
 *     owns:
 *
 *     resourceExpression
 *     resourceExpressionList
 *
 * Every hint value and condition in this file ultimately consumes that
 * canonical expression architecture.
 *
 * grammar/core/names.g4
 *     owns:
 *
 *     identifier
 *     qualifiedName
 *
 * This file does not duplicate identifier syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should represent a hint structurally, for example:
 *
 *     ResourceHint {
 *         expression / clauses,
 *         properties,
 *         conditions,
 *         scope,
 *         target,
 *         metadata,
 *         source_span
 *     }
 *
 * Exact AST type names are owned by the frontend AST contract.
 *
 * This grammar deliberately does not require a Rust AST type.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST preserve the distinction:
 *
 *     requirement != constraint != capability != preference != hint
 *
 * A hint:
 *
 *     MAY be ignored;
 *     MAY influence optimization;
 *     MAY influence placement;
 *     MAY influence scheduling;
 *     MAY influence deployment;
 *     MAY influence runtime policy;
 *
 * but MUST NOT strengthen itself into a mandatory condition merely because a
 * backend understands it.
 *
 * Unknown hint properties may be:
 *
 *     accepted as open-world metadata;
 *     diagnosed according to the language compatibility policy;
 *     rejected only when a semantic/profile policy explicitly requires it.
 *
 * The parser itself does not decide whether a hint is known.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT introduce a ResourceHintIR.
 *
 * Hint information must flow through the repository's canonical semantic
 * resource model and the existing compilation/resource pipeline.
 *
 * In particular:
 *
 *     quantum hints
 *         -> semantic resource model
 *         -> quantum::ir/resource metadata as appropriate
 *
 * do NOT create:
 *
 *     quantum_hint_ir
 *
 * or another parallel quantum intermediate representation.
 *
 * Likewise, HDL/hardware hints remain metadata or semantic resource intent
 * until consumed by their owning downstream layers.
 *
 * ============================================================================
 * RUST / SAFETY CONTRACT
 * ============================================================================
 *
 * This is an ANTLR parser grammar.
 *
 * It contains:
 *
 *     no Rust code;
 *     no embedded actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no environment access;
 *     no hardware access;
 *     no unsafe Rust.
 *
 * The Rust implementation consuming this grammar targets:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * and MUST remain safe Rust.
 *
 * ============================================================================
 */

parser grammar ResourceHints;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * 1. TOP-LEVEL HINT
 * ============================================================================
 *
 * Canonical source form:
 *
 *     hint <resource-expression>;
 *
 * Examples:
 *
 *     hint locality;
 *     hint locality = "near";
 *     hint execution::parallelism = desired_parallelism;
 *     hint quantum::routing = routing_strategy;
 *
 * The entire payload is represented through the canonical resource-expression
 * system or the reusable hint specification form below.
 *
 * ============================================================================
 */

resourceHint
    : HINT
      resourceHintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. HINT EXPRESSION
 * ============================================================================
 *
 * Two forms are intentionally supported:
 *
 *     hint <canonical-expression>;
 *
 * and:
 *
 *     hint {
 *         <hint-clause>;
 *         <hint-clause>;
 *     };
 *
 * The block form allows a hint to carry multiple related advisory properties
 * without creating a new mini-language.
 *
 * ============================================================================
 */

resourceHintExpression
    : resourceExpression
    | resourceHintSpecification
    ;


/*
 * ============================================================================
 * 3. REUSABLE HINT SPECIFICATION
 * ============================================================================
 *
 * A hint specification is an arbitrary sequence of hint clauses.
 *
 * There is no fixed number of clauses.
 *
 * ============================================================================
 */

resourceHintSpecification
    : LBRACE
      resourceHintClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. HINT CLAUSE
 * ============================================================================
 *
 * Hint clauses remain open-world.
 *
 * The grammar does not enumerate every possible optimization dimension.
 *
 * Examples:
 *
 *     latency = desired_latency;
 *     throughput = desired_throughput;
 *     locality = locality_goal;
 *     scope = execution_scope;
 *     when = workload_size > threshold;
 *     target = target_category;
 *     metadata::origin = "developer";
 *     quantum::routing = routing_strategy;
 *     accelerator::affinity = affinity_goal;
 *
 * ============================================================================
 */

resourceHintClause
    : resourceHintPropertyAssignment
    | resourceHintCondition
    | resourceHintScope
    | resourceHintTarget
    | resourceHintMetadata
    | resourceHintGroup
    ;


/*
 * ============================================================================
 * 5. GENERIC PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * This is the primary extensibility mechanism.
 *
 * It intentionally does NOT enumerate:
 *
 *     latency
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     locality
 *     affinity
 *     parallelism
 *     placement
 *     routing
 *     memory
 *     portability
 *     scalability
 *
 * Those remain semantic property names.
 *
 * This prevents the grammar from becoming an ever-growing dictionary of
 * resource concepts.
 *
 * ============================================================================
 */

resourceHintPropertyAssignment
    : resourceHintProperty
      ASSIGN
      resourceHintValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. HINT PROPERTY NAME
 * ============================================================================
 *
 * Open-world property paths support arbitrary namespace depth.
 *
 * Examples:
 *
 *     locality
 *     latency
 *     performance::latency
 *     execution::parallelism
 *     quantum::routing
 *     quantum::measurement
 *     accelerator::tensor::affinity
 *     vendor::future::metric
 *
 * DOT and DOUBLE_COLON are both accepted because the resource-expression
 * architecture already distinguishes member/path syntax from qualified names.
 *
 * The parser does not determine whether a property is standard, experimental,
 * vendor-specific, or unknown.
 *
 * ============================================================================
 */

resourceHintProperty
    : qualifiedResourceHintName
    ;


qualifiedResourceHintName
    : resourceHintNameSegment
      (
          DOT resourceHintNameSegment
        | DOUBLE_COLON resourceHintNameSegment
      )*
    ;


resourceHintNameSegment
    : identifier
    ;


/*
 * ============================================================================
 * 7. HINT VALUE
 * ============================================================================
 *
 * Values are canonical resource expressions.
 *
 * Therefore hints automatically inherit the existing Zamani expression
 * architecture rather than creating another expression language.
 *
 * Values may therefore represent:
 *
 *     constants;
 *     identifiers;
 *     arithmetic;
 *     comparisons;
 *     logical expressions;
 *     function calls;
 *     indexing;
 *     member access;
 *     collections;
 *     domain-specific expressions;
 *     computed quantities;
 *     symbolic values.
 *
 * Semantic analysis determines whether a value is appropriate for the hint.
 *
 * ============================================================================
 */

resourceHintValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 8. CONDITIONAL HINT
 * ============================================================================
 *
 * A conditional hint expresses when advisory information is applicable.
 *
 * Example:
 *
 *     hint {
 *         when = workload_size > threshold;
 *         execution::parallelism = desired_parallelism;
 *     };
 *
 * The condition is declarative.
 *
 * The parser does not evaluate it.
 *
 * ============================================================================
 */

resourceHintCondition
    : HINT
      resourceHintConditionValue
    ;


resourceHintConditionValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 9. SCOPE
 * ============================================================================
 *
 * Scope identifies the semantic region to which a hint applies.
 *
 * It is represented by an expression rather than a hard-coded list of:
 *
 *     CPU
 *     GPU
 *     QPU
 *     FPGA
 *     node
 *     memory bank
 *
 * This allows future domains to participate without changing this grammar.
 *
 * ============================================================================
 */

resourceHintScope
    : resourceHintScopeName
      ASSIGN
      resourceHintScopeValue
      SEMICOLON
    ;


resourceHintScopeName
    : identifier
    ;


resourceHintScopeValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 10. TARGET ADVISORY
 * ============================================================================
 *
 * A hint may identify an abstract target category.
 *
 * It must not be interpreted by the parser as a physical device selection.
 *
 * Examples:
 *
 *     target = cpu;
 *     target = gpu;
 *     target = quantum;
 *     target = accelerator;
 *     target = future::accelerator;
 *
 * The actual target remains a downstream semantic/compiler decision.
 *
 * ============================================================================
 */

resourceHintTarget
    : resourceHintTargetName
      ASSIGN
      resourceHintTargetValue
      SEMICOLON
    ;


resourceHintTargetName
    : identifier
    ;


resourceHintTargetValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 11. METADATA
 * ============================================================================
 *
 * Metadata may carry advisory provenance or tool-facing information.
 *
 * Examples:
 *
 *     metadata::origin = "developer";
 *     metadata::confidence = confidence_value;
 *     vendor::extension::name = extension_value;
 *
 * Metadata MUST NOT automatically change the semantic strength of a hint.
 *
 * ============================================================================
 */

resourceHintMetadata
    : resourceHintMetadataName
      ASSIGN
      resourceHintValue
      SEMICOLON
    ;


resourceHintMetadataName
    : qualifiedResourceHintName
    ;


/*
 * ============================================================================
 * 12. HINT GROUP
 * ============================================================================
 *
 * A named group allows related hints to be carried together.
 *
 * Example:
 *
 *     group = optimization {
 *         latency = latency_goal;
 *         throughput = throughput_goal;
 *         energy = energy_goal;
 *     };
 *
 * The grammar deliberately keeps the group name open-world.
 *
 * ============================================================================
 */

resourceHintGroup
    : resourceHintGroupName
      resourceHintGroupBody
      SEMICOLON
    ;


resourceHintGroupName
    : identifier
    ;


resourceHintGroupBody
    : LBRACE
      resourceHintGroupEntry*
      RBRACE
    ;


resourceHintGroupEntry
    : resourceHintClause
    ;


/*
 * ============================================================================
 * 13. HINT LIST
 * ============================================================================
 *
 * Reusable list contracts are provided for consumers that need to compose
 * hints inside another resource grammar.
 *
 * ============================================================================
 */

resourceHintList
    : resourceHint
      resourceHint*
    ;


optionalResourceHintList
    : resourceHintList?
    ;


/*
 * ============================================================================
 * 14. SEMANTIC BOUNDARIES
 * ============================================================================
 *
 * HINT
 * ----
 *
 * Advisory information.
 *
 * REQUIREMENT
 * -----------
 *
 * Mandatory capability/resource/property.
 *
 * CONSTRAINT
 * ----------
 *
 * Condition that valid realization must satisfy.
 *
 * PREFERENCE
 * ----------
 *
 * Optimization preference that may be traded against other preferences.
 *
 * CAPABILITY
 * ----------
 *
 * Property supplied by a resource/target/environment.
 *
 * IMPLEMENTATION DECISION
 * -----------------------
 *
 * Concrete downstream realization such as:
 *
 *     physical CPU;
 *     physical GPU;
 *     physical QPU;
 *     physical qubit;
 *     FPGA resource;
 *     memory bank;
 *     network node.
 *
 * This file owns only HINT syntax.
 *
 * ============================================================================
 * 15. OPEN-WORLD DOMAIN SUPPORT
 * ============================================================================
 *
 * The same hint syntax can describe advisory information for:
 *
 *     classical computing;
 *     quantum computing;
 *     hybrid computing;
 *     HDL;
 *     hardware;
 *     AI;
 *     tensors;
 *     distributed computing;
 *     networking;
 *     storage;
 *     memory;
 *     accelerators;
 *     future computational domains.
 *
 * Examples:
 *
 *     hint classical::vectorization = vectorization_goal;
 *
 *     hint quantum::routing = routing_goal;
 *
 *     hint quantum::measurement = measurement_strategy;
 *
 *     hint hdl::pipeline = pipeline_goal;
 *
 *     hint hardware::thermal = thermal_goal;
 *
 *     hint ai::accelerator = accelerator_goal;
 *
 *     hint distributed::locality = locality_goal;
 *
 *     hint networking::latency = latency_goal;
 *
 * No domain-specific finite keyword list is required.
 *
 * ============================================================================
 * 16. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum hints remain semantic metadata.
 *
 * Examples:
 *
 *     hint quantum::routing = routing_strategy;
 *
 *     hint quantum::placement = placement_strategy;
 *
 *     hint quantum::parallelism = parallelism_goal;
 *
 *     hint quantum::fidelity = fidelity_goal;
 *
 *     hint quantum::error_correction = qec_strategy;
 *
 * The parser MUST NOT:
 *
 *     enumerate quantum gates;
 *     select physical qubits;
 *     allocate qubits;
 *     perform routing;
 *     perform scheduling;
 *     perform QEC;
 *     construct quantum::ir.
 *
 * The canonical semantic pipeline remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic resource model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
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
 * ============================================================================
 * 17. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware hints describe intent.
 *
 * Examples:
 *
 *     hint hdl::pipeline = pipeline_goal;
 *
 *     hint hdl::timing = timing_goal;
 *
 *     hint hardware::memory_locality = locality_goal;
 *
 *     hint hardware::throughput = throughput_goal;
 *
 *     hint hardware::power = power_goal;
 *
 * The grammar does not impose:
 *
 *     register width;
 *     number of registers;
 *     number of pipeline stages;
 *     number of FPGA resources;
 *     memory capacity;
 *     clock frequency;
 *     physical topology.
 *
 * Such properties remain expressions and downstream semantic constraints.
 *
 * ============================================================================
 * 18. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     hint distributed::locality = locality_goal;
 *
 *     hint distributed::replication = replication_goal;
 *
 *     hint distributed::partitioning = partition_goal;
 *
 *     hint distributed::communication = communication_goal;
 *
 * No fixed node/process/thread count is encoded.
 *
 * ============================================================================
 * 19. AI / TENSOR INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     hint ai::batching = batch_goal;
 *
 *     hint tensor::layout = layout_goal;
 *
 *     hint tensor::parallelism = tensor_parallelism_goal;
 *
 *     hint accelerator::tensor::affinity = affinity_goal;
 *
 * Tensor rank, tensor dimension, accelerator count, and memory size remain
 * semantic values rather than parser limits.
 *
 * ============================================================================
 * 20. PORTABILITY
 * ============================================================================
 *
 * Hints are portable only to the extent that their semantic meaning can be
 * interpreted by a target.
 *
 * A compiler MAY:
 *
 *     honor a hint;
 *     transform a hint;
 *     ignore a hint;
 *     diagnose an unsupported hint;
 *     preserve a hint for a later compilation stage.
 *
 * A hint MUST NOT make a portable program non-portable merely because a
 * particular target cannot honor the advisory information, unless the
 * programmer has separately expressed a requirement or constraint.
 *
 * ============================================================================
 * 21. DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar is deterministic with respect to:
 *
 *     source;
 *     lexer vocabulary;
 *     grammar version;
 *     parser configuration.
 *
 * This grammar performs no:
 *
 *     hardware probing;
 *     resource discovery;
 *     random selection;
 *     wall-clock evaluation;
 *     environment lookup;
 *     backend selection.
 *
 * ============================================================================
 * 22. DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics should identify:
 *
 *     missing HINT;
 *     missing expression;
 *     malformed property path;
 *     missing ASSIGN;
 *     missing value;
 *     missing SEMICOLON;
 *     malformed hint group;
 *     malformed nested expression.
 *
 * Semantic diagnostics belong downstream and may identify:
 *
 *     unknown hint property;
 *     unsupported hint;
 *     conflicting hints;
 *     invalid hint value type;
 *     invalid hint scope;
 *     unsupported target-specific hint;
 *     hint incorrectly used where a requirement/constraint is required.
 *
 * ============================================================================
 * 23. SECURITY
 * ============================================================================
 *
 * Hint syntax MUST NOT provide an execution escape hatch.
 *
 * A hint must never cause the parser to:
 *
 *     execute code;
 *     access files;
 *     access credentials;
 *     contact a network;
 *     probe hardware;
 *     invoke a compiler backend;
 *     allocate resources.
 *
 * Any expression embedded in a hint is still subject to the normal semantic,
 * effect, capability, ownership, and security analysis pipeline.
 *
 * ============================================================================
 * 24. PERFORMANCE
 * ============================================================================
 *
 * This grammar intentionally avoids:
 *
 *     large finite keyword alternatives;
 *     enumerations of hardware;
 *     enumerations of capabilities;
 *     enumerations of optimization dimensions;
 *     target-specific alternatives.
 *
 * Property namespace depth is recursive/repetitive rather than bounded.
 *
 * Hint lists and groups are unbounded by language design.
 *
 * Compiler/parser resource exhaustion limits, where required for defensive
 * implementation, belong to the parser/compiler safety policy rather than
 * this language grammar.
 *
 * ============================================================================
 * 25. COMPATIBILITY
 * ============================================================================
 *
 * The stable top-level syntax is:
 *
 *     hint <resource-expression>;
 *
 * The reusable block form is:
 *
 *     hint {
 *         <hint-clause>*
 *     };
 *
 * Existing resource grammars must delegate hint ownership to this grammar.
 *
 * No alternative hint grammar should be introduced under:
 *
 *     core/
 *     hardware/
 *     execution/
 *     compile/
 *     quantum/
 *     hdl/
 *
 * unless it is explicitly an interoperability/dialect grammar.
 *
 * ============================================================================
 * 26. CONFORMANCE TEST CONTRACT
 * ============================================================================
 *
 * Positive cases MUST include:
 *
 *     hint locality;
 *
 *     hint locality = locality_goal;
 *
 *     hint execution::parallelism = parallelism_goal;
 *
 *     hint quantum::routing = routing_strategy;
 *
 *     hint hardware::memory_locality = locality_goal;
 *
 *     hint distributed::locality = locality_goal;
 *
 *     hint ai::accelerator = accelerator_goal;
 *
 *     hint {
 *         latency = latency_goal;
 *         throughput = throughput_goal;
 *     };
 *
 *     hint {
 *         when = workload_size > threshold;
 *         execution::parallelism = desired_parallelism;
 *     };
 *
 *     hint vendor::future::metric = future_value;
 *
 * Negative cases MUST include:
 *
 *     hint;
 *
 *     hint =;
 *
 *     hint locality =;
 *
 *     hint { };
 *
 * where the language policy requires at least one meaningful hint clause in
 * block form.
 *
 * Boundary/scalability cases MUST include:
 *
 *     one hint;
 *     many hints;
 *     deeply nested property namespaces;
 *     deeply nested expressions;
 *     large symbolic quantities;
 *     large hint groups;
 *     large hint lists;
 *     mixed classical/quantum/HDL/hardware hints;
 *     unknown future property names;
 *     vendor namespaces;
 *     dialect namespaces.
 *
 * Portability tests MUST verify that:
 *
 *     hint != requirement
 *     hint != constraint
 *     hint != capability
 *     hint != preference
 *     hint != physical allocation
 *
 * ============================================================================
 * 27. HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     no MAX_QUBITS;
 *     no MAX_CPUS;
 *     no MAX_GPUS;
 *     no MAX_FPGAS;
 *     no MAX_NODES;
 *     no MAX_MEMORY;
 *     no MAX_THREADS;
 *     no MAX_TENSOR_RANK;
 *     no MAX_REGISTER_WIDTH;
 *     no MAX_NETWORK_SIZE;
 *     no MAX_DEVICE_COUNT;
 *     no fixed physical device IDs;
 *     no fixed qubit IDs;
 *     no fixed topology;
 *     no fixed machine size;
 *     no fixed accelerator count;
 *     no fixed timeline count;
 *     no fixed process count.
 *
 * PASS:
 *
 *     all quantities are expressions;
 *     all property namespaces are open-world;
 *     all cardinalities are unbounded by grammar design;
 *     target realization is downstream.
 *
 * ============================================================================
 * 28. COMPLETION CRITERIA
 * ============================================================================
 *
 * ResourceHints is complete when:
 *
 *     [x] grammar identity is unique;
 *     [x] token vocabulary is canonical;
 *     [x] resourceExpression is reused;
 *     [x] identifier/name syntax is reused;
 *     [x] hint syntax has one owner;
 *     [x] top-level hint syntax is represented;
 *     [x] reusable hint specification is represented;
 *     [x] generic properties are open-world;
 *     [x] nested namespaces are unbounded;
 *     [x] values are canonical expressions;
 *     [x] hints remain advisory;
 *     [x] requirements remain distinct;
 *     [x] constraints remain distinct;
 *     [x] preferences remain distinct;
 *     [x] capabilities remain distinct;
 *     [x] physical allocation remains downstream;
 *     [x] quantum integration is semantic;
 *     [x] HDL integration is semantic;
 *     [x] hardware integration is target-independent;
 *     [x] distributed integration is target-independent;
 *     [x] AI integration is framework-neutral;
 *     [x] no universal hardware limits are encoded;
 *     [x] no Rust actions exist;
 *     [x] no unsafe Rust is required;
 *     [x] deterministic parsing is preserved;
 *     [x] diagnostics are defined;
 *     [x] security boundary is defined;
 *     [x] scalability boundary is defined;
 *     [x] compatibility boundary is defined;
 *     [x] test contract is defined.
 *
 * Repository integration is complete only after resources.g4 removes its
 * duplicate ownership of:
 *
 *     resourceHint
 *     resourceHintExpression
 *     resourceHintClause
 *
 * and imports ResourceHints.
 *
 * ============================================================================
 */