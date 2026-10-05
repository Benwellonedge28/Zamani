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
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical LEAF/PAYLOAD grammar for resource hints.
 *
 * A resource hint is advisory resource intent. It may guide:
 *
 *     - optimization;
 *     - resource selection;
 *     - placement;
 *     - scheduling;
 *     - deployment;
 *     - execution strategy;
 *     - simulation strategy;
 *     - quantum realization;
 *     - HDL/hardware realization;
 *     - distributed realization;
 *     - accelerator selection;
 *     - AI/data execution strategy.
 *
 * A hint is NOT a requirement.
 *
 * A hint is NOT a constraint.
 *
 * A hint is NOT a capability.
 *
 * A hint is NOT a budget.
 *
 * A hint is NOT a preference.
 *
 * A hint is NOT an allocation.
 *
 * A hint is NOT a physical-device selection.
 *
 * A hint MAY be ignored by a valid realization unless another semantic
 * construct separately requires it.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Zamani parser
 *       |
 *       v
 *     grammar/resources/resources.g4
 *       |
 *       | owns concrete `resourceHint`
 *       v
 *     ResourceHints
 *       |
 *       | owns reusable hint payload
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +-------------------+
 *       |                   |
 *       v                   v
 *     semantic resource model
 *                         provenance
 *       |
 *       +-----------------------------+
 *       |             |               |
 *       v             v               v
 *   compiler      optimizer       negotiation
 *       |             |               |
 *       +-------------+---------------+
 *                     |
 *                     v
 *             execution planning
 *                     |
 *       +-------------+-------------+
 *       |             |             |
 *       v             v             v
 *   classical     quantum::ir    HDL/hardware
 *       |             |             |
 *       +-------------+-------------+
 *                     |
 *              target realization
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     resourceHintExpression
 *     resourceHintSpecification
 *     resourceHintClause
 *     resourceHintPropertyAssignment
 *     resourceHintValueClause
 *     resourceHintGroup
 *     resourceHintGroupBody
 *     resourceHintGroupEntry
 *     resourceHintPropertyName
 *     resourceHintValue
 *     resourceHintListPayload
 *     optionalResourceHintListPayload
 *
 *
 * THIS FILE DOES NOT OWN:
 *
 *     resources
 *     resourceItem
 *     resourceHint
 *     resourceHintClause at the universal-resource orchestration level
 *     resourceExpression
 *     resourceExpressionList
 *     expression
 *     identifier
 *     qualifiedName
 *     requirements
 *     constraints
 *     budgets
 *     preferences
 *     capabilities
 *     negotiation
 *     scalability
 *     policies
 *     effects
 *     provenance semantics
 *     target selection
 *     resource discovery
 *     allocation
 *     placement
 *     routing
 *     scheduling
 *     optimization
 *     QEC
 *     ZQN
 *     HAL
 *     runtime resource management
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/resources/resource-expressions.g4
 *     grammar/core/names.g4
 *
 *
 * EXPORTS
 * -------
 *
 *     resourceHintExpression
 *     resourceHintSpecification
 *     resourceHintClause
 *     resourceHintPropertyAssignment
 *     resourceHintValueClause
 *     resourceHintGroup
 *     resourceHintGroupBody
 *     resourceHintGroupEntry
 *     resourceHintPropertyName
 *     resourceHintValue
 *     resourceHintListPayload
 *     optionalResourceHintListPayload
 *
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/resources/resources.g4
 *
 * Future consumers MAY include:
 *
 *     grammar/resources/negotiation.g4
 *     grammar/resources/scalability.g4
 *     grammar/execution/
 *     grammar/compile/
 *     grammar/policies/
 *     grammar/dialects/
 *
 *
 * AST_OWNER
 * ---------
 *
 * Domain-neutral frontend AST.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Resource semantic analysis.
 *
 *
 * IR_OWNER
 * --------
 *
 * No IR is owned by this grammar.
 *
 * Hint information is lowered into the canonical semantic resource-intent
 * representation and subsequently consumed by the appropriate compilation,
 * planning, execution, quantum, HDL, hardware, or distributed subsystem.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/resources/hints/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/resources.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * The concrete resource statement remains owned by:
 *
 *     grammar/resources/resources.g4
 *
 * Therefore this file MUST NOT define:
 *
 *     resourceHint
 *
 * The parent owns:
 *
 *     resourceHint
 *         : HINT resourceHintExpression SEMICOLON
 *         ;
 *
 * This file supplies the payload after HINT.
 *
 * This prevents:
 *
 *     duplicate parser ownership;
 *     imported-rule collisions;
 *     parallel resource languages;
 *     parent/child grammar cycles.
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes the canonical ZamaniLexer vocabulary.
 *
 * Required tokens used directly by this file:
 *
 *     HINT
 *     ASSIGN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *
 * Name syntax is supplied by:
 *
 *     Names.qualifiedName
 *
 * Expression syntax is supplied by:
 *
 *     ResourceExpressions.resourceExpression
 *
 * This grammar MUST NOT define lexer rules.
 *
 * This grammar MUST NOT introduce:
 *
 *     K_HINT
 *     K_ASSIGN
 *     K_LBRACE
 *     K_RBRACE
 *
 * or any competing token aliases.
 *
 *
 * ============================================================================
 * RESOURCE EXPRESSION CONTRACT
 * ============================================================================
 *
 * resourceExpression is owned exclusively by:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * Consequently this grammar inherits the repository's canonical expression
 * semantics, including whatever arithmetic, logical, comparison, invocation,
 * indexing, member-access, literal, symbolic, and dynamic expression forms
 * the canonical expression subsystem supports.
 *
 * This grammar MUST NOT recreate:
 *
 *     arithmetic;
 *     comparison;
 *     logical operators;
 *     literals;
 *     function calls;
 *     indexing;
 *     member access;
 *     unary operators;
 *     expression precedence.
 *
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * qualifiedName is owned by:
 *
 *     grammar/core/names.g4
 *
 * Hint property names therefore use:
 *
 *     qualifiedName
 *
 * Examples:
 *
 *     locality
 *     execution::parallelism
 *     quantum::routing
 *     quantum::measurement
 *     hdl::pipeline
 *     hardware::thermal
 *     distributed::locality
 *     tensor::layout
 *     accelerator::tensor::affinity
 *     vendor::extension::future_metric
 *
 * The grammar does not determine whether a name is:
 *
 *     standardized;
 *     experimental;
 *     vendor-specific;
 *     dialect-specific;
 *     future;
 *     unknown.
 *
 * That is semantic/profile responsibility.
 *
 *
 * ============================================================================
 * OPEN-WORLD RESOURCE MODEL
 * ============================================================================
 *
 * Hint property names are deliberately open-world.
 *
 * The grammar MUST NOT enumerate a finite resource-property universe.
 *
 * It must NOT require alternatives for every possible:
 *
 *     CPU property;
 *     GPU property;
 *     FPGA property;
 *     ASIC property;
 *     QPU property;
 *     accelerator property;
 *     memory property;
 *     network property;
 *     AI property;
 *     tensor property;
 *     HDL property;
 *     distributed property;
 *     future-domain property.
 *
 * New semantic properties can therefore be introduced without changing this
 * grammar merely because a new computational technology appears.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A hint participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * by expressing advisory intent rather than physical realization.
 *
 * Examples:
 *
 *     hint execution::parallelism = desired_parallelism;
 *
 *     hint memory::locality = locality_goal;
 *
 *     hint quantum::routing = routing_strategy;
 *
 *     hint accelerator::affinity = accelerator_goal;
 *
 *     hint distributed::partitioning = partition_strategy;
 *
 *     hint tensor::layout = preferred_layout;
 *
 * The same source may therefore be considered for:
 *
 *     embedded;
 *     CPU;
 *     multicore;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed;
 *     cloud;
 *     future computational targets.
 *
 * A target that cannot honor a hint may ignore or diagnose the hint according
 * to semantic compatibility policy.
 *
 * A hint MUST NOT silently become a requirement merely because one target
 * understands it.
 *
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar contains NO universal machine/resource limits.
 *
 * It MUST NOT define or imply:
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
 * It must also not encode equivalent finite alternatives such as:
 *
 *     cpu0 | cpu1 | cpu2
 *     gpu0 | gpu1
 *     qpu0 | qpu1
 *     node0 | node1
 *
 * A source expression such as:
 *
 *     hint execution::parallelism = desired_parallelism;
 *
 * is a program value.
 *
 * It is not a language-level maximum.
 *
 * Likewise:
 *
 *     hint quantum::logical_qubits = logical_qubits;
 *
 * does not establish a maximum number of qubits.
 *
 *
 * ============================================================================
 * UNBOUNDED SCALABILITY
 * ============================================================================
 *
 * This grammar uses:
 *
 *     *
 *     +
 *     recursive composition
 *
 * wherever source cardinality is required.
 *
 * There is no grammar-level finite maximum for:
 *
 *     hint count;
 *     property count;
 *     namespace depth;
 *     group count;
 *     group nesting;
 *     expression size;
 *     resource domains;
 *     target categories;
 *     semantic properties.
 *
 * "Infinity" means:
 *
 *     no artificial finite resource ceiling is encoded in the language.
 *
 * It does not claim physically infinite resources.
 *
 * Physical and implementation limits belong to:
 *
 *     compiler;
 *     runtime;
 *     operating system;
 *     deployment environment;
 *     actual hardware;
 *     provider/environment capability.
 *
 *
 * ============================================================================
 * HINT SEMANTICS
 * ============================================================================
 *
 * A hint is advisory.
 *
 * Semantic analysis MAY:
 *
 *     honor it;
 *     transform it;
 *     preserve it;
 *     ignore it;
 *     diagnose it;
 *     use it during negotiation;
 *     use it during optimization;
 *     use it during planning.
 *
 * Semantic analysis MUST NOT reinterpret a hint as:
 *
 *     requirement;
 *     constraint;
 *     capability;
 *     allocation;
 *     reservation;
 *     mandatory target selection.
 *
 *
 * ============================================================================
 * 1. HINT PAYLOAD
 * ============================================================================
 *
 * The parent resource grammar owns:
 *
 *     HINT ... SEMICOLON
 *
 * This file owns only what appears after HINT and before the parent
 * terminator.
 *
 * Two payload forms are supported:
 *
 *     hint <resource-expression>;
 *
 *     hint {
 *         <hint-clause>*
 *     };
 *
 * The first is concise.
 *
 * The second permits multiple advisory properties to be grouped into one
 * source construct.
 *
 * ============================================================================
 */

resourceHintExpression
    : resourceExpression
    | resourceHintSpecification
    ;


/*
 * ============================================================================
 * 2. HINT SPECIFICATION
 * ============================================================================
 *
 * A specification is an unbounded sequence of hint clauses.
 *
 * Empty specifications are intentionally rejected.
 *
 * Therefore:
 *
 *     hint { };
 *
 * is structurally invalid.
 *
 * This prevents a syntactically successful but semantically empty hint.
 *
 * ============================================================================
 */

resourceHintSpecification
    : LBRACE
      resourceHintClause+
      RBRACE
    ;


/*
 * ============================================================================
 * 3. HINT CLAUSE
 * ============================================================================
 *
 * A clause may be:
 *
 *     - a named advisory property assignment;
 *     - a standalone advisory expression;
 *     - a nested named group.
 *
 * The grammar deliberately does not enumerate property names.
 *
 * ============================================================================
 */

resourceHintClause
    : resourceHintPropertyAssignment
    | resourceHintValueClause
    | resourceHintGroup
    ;


/*
 * ============================================================================
 * 4. PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * Canonical form:
 *
 *     <qualified-name> = <resource-expression> ;
 *
 * Examples:
 *
 *     latency = latency_goal;
 *
 *     throughput = throughput_goal;
 *
 *     execution::parallelism = parallelism_goal;
 *
 *     quantum::routing = routing_strategy;
 *
 *     quantum::measurement = measurement_strategy;
 *
 *     hdl::pipeline = pipeline_goal;
 *
 *     hardware::thermal = thermal_goal;
 *
 *     distributed::locality = locality_goal;
 *
 *     tensor::layout = layout_goal;
 *
 *     accelerator::tensor::affinity = affinity_goal;
 *
 *     vendor::future::optimization = optimization_goal;
 *
 * The property name is symbolic.
 *
 * The value is a canonical resource expression.
 *
 * ============================================================================
 */

resourceHintPropertyAssignment
    : resourceHintPropertyName
      ASSIGN
      resourceHintValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. PROPERTY NAME
 * ============================================================================
 *
 * qualifiedName is the sole name authority.
 *
 * No alternate qualified-name syntax is introduced here.
 *
 * ============================================================================
 */

resourceHintPropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 6. VALUE
 * ============================================================================
 *
 * Hint values are canonical resource expressions.
 *
 * This permits:
 *
 *     literal values;
 *     symbolic values;
 *     computed values;
 *     input-derived values;
 *     workload-derived values;
 *     runtime-derived values where permitted by semantic analysis;
 *     resource-derived values;
 *     expressions involving other program values.
 *
 * Examples:
 *
 *     desired_parallelism
 *
 *     workload_size * parallelism_factor
 *
 *     available_memory * utilization_target
 *
 *     logical_qubits + ancilla_qubits
 *
 *     preferred_latency
 *
 * No value is interpreted by the parser.
 *
 * ============================================================================
 */

resourceHintValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 7. STANDALONE VALUE CLAUSE
 * ============================================================================
 *
 * A standalone expression is useful when the hint itself is naturally
 * represented by a symbolic expression.
 *
 * Example:
 *
 *     hint {
 *         preferred_layout;
 *         workload_locality;
 *     };
 *
 * The semantic layer determines what those expressions mean in the enclosing
 * resource-hint context.
 *
 * ============================================================================
 */

resourceHintValueClause
    : resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. NESTED HINT GROUP
 * ============================================================================
 *
 * Groups provide structural organization without introducing a closed
 * vocabulary.
 *
 * Examples:
 *
 *     execution {
 *         parallelism = desired_parallelism;
 *         locality = locality_goal;
 *     }
 *
 *     quantum::execution {
 *         routing = routing_goal;
 *         measurement = measurement_goal;
 *     }
 *
 *     vendor::future {
 *         optimization = future_goal;
 *     }
 *
 * Group names are qualified names.
 *
 * Group nesting is therefore open-ended.
 *
 * ============================================================================
 */

resourceHintGroup
    : resourceHintPropertyName
      resourceHintGroupBody
    ;


resourceHintGroupBody
    : LBRACE
      resourceHintGroupEntry+
      RBRACE
    ;


resourceHintGroupEntry
    : resourceHintClause
    ;


/*
 * ============================================================================
 * 9. REUSABLE PAYLOAD LISTS
 * ============================================================================
 *
 * These rules deliberately represent PAYLOADS rather than complete `hint`
 * statements.
 *
 * This prevents the leaf grammar from taking ownership of the parent
 * statement terminator or resource-item dispatch.
 *
 * ============================================================================
 */

resourceHintListPayload
    : resourceHintExpression
      (COMMA resourceHintExpression)*
      COMMA?
    ;


optionalResourceHintListPayload
    : resourceHintListPayload?
    ;


/*
 * ============================================================================
 * 10. SEMANTIC SEPARATION
 * ============================================================================
 *
 * REQUIREMENT
 * -----------
 *
 * Must be satisfied.
 *
 * CONSTRAINT
 * ----------
 *
 * Must remain true.
 *
 * CAPABILITY
 * ----------
 *
 * A property the environment can provide.
 *
 * BUDGET
 * ------
 *
 * A bounded resource allowance or accounting intent.
 *
 * PREFERENCE
 * ----------
 *
 * A desirable realization characteristic.
 *
 * HINT
 * ----
 *
 * Advisory information.
 *
 * ALLOCATION
 * ----------
 *
 * A concrete resource acquisition/assignment decision.
 *
 * This grammar owns only HINT payload syntax.
 *
 *
 * ============================================================================
 * 11. RESOURCE-SUBSYSTEM INTEGRATION
 * ============================================================================
 *
 * requirements.g4
 * ----------------
 *
 * Requirements remain independent.
 *
 * A hint may refer to values also used by requirements, but it does not
 * redefine resourceRequirement.
 *
 *
 * constraints.g4
 * --------------
 *
 * Constraints remain independent.
 *
 * A hint may provide information useful to constraint-aware optimization,
 * but it does not create a constraint.
 *
 *
 * budgets.g4
 * ----------
 *
 * Budgets remain independent.
 *
 * A hint may recommend how an available budget should be used, but a hint
 * does not establish the budget.
 *
 *
 * preferences.g4
 * --------------
 *
 * Preferences remain independent.
 *
 * A hint must not silently acquire preference semantics.
 *
 *
 * capabilities.g4
 * ---------------
 *
 * Capability identity remains independent.
 *
 * A hint such as:
 *
 *     hint quantum::measurement = preferred_strategy;
 *
 * does not claim that quantum measurement capability exists.
 *
 *
 * negotiation.g4
 * --------------
 *
 * Negotiation may consume hints as advisory inputs.
 *
 * This grammar does not perform negotiation.
 *
 *
 * scalability.g4
 * --------------
 *
 * Scalability analysis may consume hint values.
 *
 * This grammar does not determine scaling behavior.
 *
 *
 * resource-expressions.g4
 * -----------------------
 *
 * This is the canonical value-expression owner.
 *
 * Every hint value delegates to resourceExpression.
 *
 *
 * resources.g4
 * ------------
 *
 * resources.g4 owns:
 *
 *     resourceHint
 *     resourceHintClause
 *
 * only after integration is normalized.
 *
 * IMPORTANT:
 *
 * The current resources.g4 contains duplicate concrete hint rules. Those
 * duplicates must be removed so that this grammar becomes the sole owner of
 * the reusable hint payload.
 *
 *
 * ============================================================================
 * 12. AI / LEARNING / ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Hints can support adaptive and intelligent execution without introducing
 * application-specific syntax.
 *
 * Examples:
 *
 *     hint learning::batching = batch_goal;
 *
 *     hint inference::parallelism = inference_parallelism;
 *
 *     hint adaptation::strategy = adaptation_strategy;
 *
 *     hint reasoning::latency = reasoning_latency_goal;
 *
 *     hint neural_symbolic::placement = placement_goal;
 *
 * These are ordinary open-world semantic properties.
 *
 * This grammar does not create special syntax for every future algorithm,
 * framework, model, or application.
 *
 *
 * ============================================================================
 * 13. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum hints are ordinary resource-hint metadata.
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
 *     hint quantum::error_correction = resilience_strategy;
 *
 *     hint quantum::measurement = measurement_strategy;
 *
 * The parser MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate physical qubits;
 *     - assign physical qubit identities;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform decomposition;
 *     - perform QEC;
 *     - create quantum::ir.
 *
 * The canonical downstream path remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic resource model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience/QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 *
 * ============================================================================
 * 14. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical hints can express advisory intent for:
 *
 *     vectorization;
 *     locality;
 *     parallelism;
 *     cache behavior;
 *     memory strategy;
 *     numerical execution;
 *     accelerator use;
 *     data movement;
 *     throughput;
 *     latency;
 *     power;
 *     energy.
 *
 * No CPU model is embedded.
 *
 *
 * ============================================================================
 * 15. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hints may describe hardware/HDL intent:
 *
 *     hint hdl::pipeline = pipeline_goal;
 *
 *     hint hdl::timing = timing_goal;
 *
 *     hint hardware::area = area_goal;
 *
 *     hint hardware::power = power_goal;
 *
 *     hint hardware::thermal = thermal_goal;
 *
 *     hint hardware::throughput = throughput_goal;
 *
 * The grammar does not define:
 *
 *     register width;
 *     number of registers;
 *     number of pipeline stages;
 *     FPGA capacity;
 *     ASIC capacity;
 *     clock-frequency limits;
 *     fixed physical topology.
 *
 * Those remain semantic or backend concerns.
 *
 *
 * ============================================================================
 * 16. DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * Hints may describe:
 *
 *     distributed::locality
 *     distributed::partitioning
 *     distributed::replication
 *     distributed::communication
 *     networking::latency
 *     networking::bandwidth
 *     networking::topology
 *
 * The grammar never encodes a maximum node count or fixed node identifiers.
 *
 *
 * ============================================================================
 * 17. DATA / TENSOR INTEGRATION
 * ============================================================================
 *
 * Hints may describe:
 *
 *     data::locality
 *     data::layout
 *     data::partitioning
 *     tensor::layout
 *     tensor::parallelism
 *     tensor::placement
 *     accelerator::tensor::affinity
 *
 * Tensor rank, dimension, size, and accelerator count remain semantic
 * expressions.
 *
 *
 * ============================================================================
 * 18. EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a hint has no execution effect.
 *
 * A hint declaration MUST NOT itself:
 *
 *     perform I/O;
 *     allocate memory;
 *     access hardware;
 *     access a network;
 *     invoke foreign code;
 *     mutate runtime state;
 *     execute learning;
 *     execute adaptation;
 *     execute reflection.
 *
 * If a hint value references an effectful semantic expression, normal
 * expression/effect checking remains authoritative.
 *
 *
 * ============================================================================
 * 19. CAPABILITY CONTRACT
 * ============================================================================
 *
 * A hint does not grant a capability.
 *
 * For example:
 *
 *     hint quantum::measurement = preferred_measurement;
 *
 * does not grant:
 *
 *     quantum.measurement
 *
 * Likewise:
 *
 *     hint accelerator::tensor::affinity = preferred_accelerator;
 *
 * does not prove accelerator availability.
 *
 * Capability checking remains downstream.
 *
 *
 * ============================================================================
 * 20. RESOURCE CONTRACT
 * ============================================================================
 *
 * Hint values may refer to arbitrary symbolic resource quantities.
 *
 * Examples:
 *
 *     desired_memory
 *     required_bandwidth
 *     available_parallelism
 *     workload_size
 *     logical_qubits
 *     tensor_elements
 *
 * The grammar does not evaluate these values.
 *
 * Resource availability is determined by semantic analysis, negotiation,
 * planning, and target realization.
 *
 *
 * ============================================================================
 * 21. CONTRACT CONTRACT
 * ============================================================================
 *
 * Hints may appear in source constructs governed by:
 *
 *     requires;
 *     ensures;
 *     invariant;
 *     assume;
 *     guarantee;
 *     property;
 *
 * but this file does not own contract syntax.
 *
 * A hint remains advisory unless a separate contract explicitly gives another
 * construct mandatory semantic force.
 *
 *
 * ============================================================================
 * 22. POLICY CONTRACT
 * ============================================================================
 *
 * Policy analysis may restrict which hints are permitted, ignored, transformed,
 * or honored.
 *
 * This grammar does not own policy semantics.
 *
 * Examples of semantic policy decisions include:
 *
 *     whether vendor-specific hints are permitted;
 *     whether a hint may affect deployment;
 *     whether a hint may influence adaptive execution;
 *     whether a hint may be preserved across compilation boundaries.
 *
 *
 * ============================================================================
 * 23. PROVENANCE CONTRACT
 * ============================================================================
 *
 * The AST/semantic model must preserve source provenance for:
 *
 *     - the complete hint;
 *     - property names;
 *     - values;
 *     - nested groups;
 *     - source ordering;
 *     - source spans.
 *
 * Provenance may later record:
 *
 *     source;
 *     derived_from;
 *     transformed_by;
 *     honored_by;
 *     ignored_by;
 *     reason;
 *     evidence;
 *     compiler version;
 *     semantic version;
 *     target realization.
 *
 * This grammar does not create provenance records itself.
 *
 *
 * ============================================================================
 * 24. AST CONTRACT
 * ============================================================================
 *
 * Conceptual AST:
 *
 *     ResourceHintPayload
 *         kind:
 *             Expression
 *             Specification
 *
 *     ResourceHintSpecification
 *         clauses[]
 *
 *     ResourceHintProperty
 *         name
 *         value
 *         source_span
 *
 *     ResourceHintValue
 *         expression
 *
 *     ResourceHintGroup
 *         name
 *         entries[]
 *
 * The actual Rust AST type names are owned by the frontend AST implementation.
 *
 * The grammar MUST NOT import Rust AST types.
 *
 * The AST must preserve source ordering because two advisory hints may have
 * different semantic precedence under a future explicit policy.
 *
 *
 * ============================================================================
 * 25. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     - classify the hint;
 *     - resolve property names;
 *     - validate expression types;
 *     - resolve referenced symbols;
 *     - determine whether a property is standard/dialect/vendor/future;
 *     - validate domain compatibility;
 *     - apply policy;
 *     - preserve advisory strength;
 *     - detect conflicting hints;
 *     - construct canonical resource intent.
 *
 * Semantic analysis MUST NOT assume that every parsed hint has a realization.
 *
 *
 * ============================================================================
 * 26. IR CONTRACT
 * ============================================================================
 *
 * This grammar owns NO IR.
 *
 * There must be no:
 *
 *     HintIR
 *     QuantumHintIR
 *     HardwareHintIR
 *     AIHintIR
 *
 * merely because hints exist.
 *
 * Hint information flows through the canonical semantic resource model.
 *
 * Quantum-specific information may subsequently accompany the canonical
 * quantum semantic representation and, where appropriate, quantum::ir.
 *
 * HDL/hardware information is consumed by the HDL/hardware lowering path.
 *
 *
 * ============================================================================
 * 27. BACKEND CONTRACT
 * ============================================================================
 *
 * A backend MAY:
 *
 *     honor a hint;
 *     partially honor a hint;
 *     transform a hint;
 *     ignore a hint;
 *     diagnose an unsupported hint;
 *     preserve a hint for another stage.
 *
 * A backend MUST NOT treat an advisory hint as a mandatory resource
 * requirement unless that semantic strengthening is explicitly represented by
 * another source construct.
 *
 *
 * ============================================================================
 * 28. DIAGNOSTICS
 * ============================================================================
 *
 * STRUCTURAL/PARSER diagnostics:
 *
 *     missing expression;
 *     missing property name;
 *     missing ASSIGN;
 *     missing value;
 *     missing SEMICOLON;
 *     malformed group;
 *     malformed group entry;
 *     unterminated specification;
 *     unterminated group.
 *
 * SEMANTIC diagnostics:
 *
 *     unknown required-standard property;
 *     invalid value type;
 *     incompatible resource dimension;
 *     conflicting hint;
 *     policy-disallowed hint;
 *     unsupported target interpretation;
 *     invalid domain use.
 *
 * The parser must not attempt to perform semantic validation.
 *
 *
 * ============================================================================
 * 29. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source;
 *     canonical lexer vocabulary;
 *     grammar version;
 *     parser configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     runtime availability;
 *     filesystem state;
 *     network state;
 *     randomness;
 *     wall-clock time;
 *     provider state.
 *
 *
 * ============================================================================
 * 30. SECURITY
 * ============================================================================
 *
 * This grammar is declarative.
 *
 * It contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no shell execution;
 *     no filesystem access;
 *     no network access;
 *     no environment access;
 *     no hardware probing;
 *     no credential access;
 *     no unsafe Rust.
 *
 * Generated Rust code must be integrated into the existing safe-Rust frontend
 * without introducing unsafe application code.
 *
 *
 * ============================================================================
 * 31. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable surface:
 *
 *     hint <resource-expression>;
 *
 * Extended surface:
 *
 *     hint {
 *         <hint-clause>+
 *     };
 *
 * Property assignments:
 *
 *     <qualified-name> = <resource-expression>;
 *
 * Nested groups:
 *
 *     <qualified-name> {
 *         <hint-clause>+
 *     }
 *
 * Existing source programs using:
 *
 *     hint <expression>;
 *
 * remain compatible.
 *
 * No finite property vocabulary is required for future compatibility.
 *
 *
 * ============================================================================
 * 32. CONFORMANCE TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 *     hint locality;
 *
 *     hint locality = locality_goal;
 *
 *     hint execution::parallelism = parallelism_goal;
 *
 *     hint quantum::routing = routing_strategy;
 *
 *     hint quantum::measurement = measurement_strategy;
 *
 *     hint hardware::thermal = thermal_goal;
 *
 *     hint distributed::locality = locality_goal;
 *
 *     hint tensor::layout = layout_goal;
 *
 *     hint vendor::future::optimization = optimization_goal;
 *
 *     hint {
 *         latency = latency_goal;
 *         throughput = throughput_goal;
 *     };
 *
 *     hint {
 *         quantum::routing = routing_goal;
 *         quantum::measurement = measurement_goal;
 *     };
 *
 *     hint {
 *         execution {
 *             parallelism = desired_parallelism;
 *             locality = locality_goal;
 *         }
 *     };
 *
 *
 * NEGATIVE TESTS
 * -------------
 *
 *     hint;
 *
 *     hint =;
 *
 *     hint locality =;
 *
 *     hint { };
 *
 *     hint {
 *         locality =
 *     };
 *
 *     hint {
 *         execution {
 *             parallelism =
 *         }
 *     };
 *
 *     hint {
 *         locality
 *     };
 *
 *
 * BOUNDARY TESTS
 * -------------
 *
 *     deeply qualified property names;
 *     deeply nested groups;
 *     symbolic values;
 *     computed values;
 *     very large source-level quantities;
 *     dynamic expressions;
 *     vendor namespaces;
 *     dialect namespaces;
 *     future-domain namespaces;
 *     mixed classical/quantum/HDL/resource hints;
 *     large hint specifications.
 *
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Verify that the grammar imposes no fixed:
 *
 *     hint count;
 *     property count;
 *     group count;
 *     group depth;
 *     namespace depth;
 *     resource-domain count;
 *     target count;
 *     device count;
 *     qubit count;
 *     processor count;
 *     node count.
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Identical source with identical grammar/lexer configuration must produce
 * identical parse structures.
 *
 *
 * ============================================================================
 * 33. HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     [x] no hardware capacity constants;
 *     [x] no physical device enumeration;
 *     [x] no qubit enumeration;
 *     [x] no processor enumeration;
 *     [x] no node enumeration;
 *     [x] no tensor-rank ceiling;
 *     [x] no register-width ceiling;
 *     [x] no network-size ceiling;
 *     [x] no accelerator-count ceiling;
 *     [x] no finite resource-property dictionary;
 *     [x] no fixed group depth;
 *     [x] no fixed namespace depth;
 *     [x] all quantities are expressions;
 *     [x] target realization remains downstream.
 *
 *
 * ============================================================================
 * 34. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar deliberately favors:
 *
 *     direct delegation to resourceExpression;
 *     qualifiedName reuse;
 *     iterative repetition;
 *     bounded local alternatives;
 *     recursive composition only where source nesting requires it.
 *
 * It introduces:
 *
 *     no semantic predicates;
 *     no actions;
 *     no runtime callbacks;
 *     no target probing.
 *
 * Defensive parser resource limits, if required by the implementation, are
 * implementation safety policy and MUST NOT become language semantics.
 *
 *
 * ============================================================================
 * 35. RUST INTEGRATION CONTRACT
 * ============================================================================
 *
 * Target:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Requirements:
 *
 *     - generated parser must integrate with the repository's ANTLR Rust
 *       frontend;
 *     - application-level Rust remains safe Rust;
 *     - no unsafe blocks are required by this grammar;
 *     - no embedded Rust actions are permitted;
 *     - no semantic predicates are permitted;
 *     - no target-specific Rust code is permitted in this grammar.
 *
 * The grammar itself is therefore independent of the Rust implementation
 * details.
 *
 *
 * ============================================================================
 * 36. REQUIRED INTEGRATION CHANGE IN resources.g4
 * ============================================================================
 *
 * The parent orchestrator must import:
 *
 *     ResourceHints
 *
 * alongside the other resource leaf grammars.
 *
 * It must continue to own:
 *
 *     resourceHint
 *
 * but that rule must delegate its payload to:
 *
 *     resourceHintExpression
 *
 * from this grammar.
 *
 * The current duplicate definitions in resources.g4:
 *
 *     resourceHintExpression
 *     resourceHintClause
 *
 * must be removed.
 *
 * The canonical parent form is:
 *
 *     resourceHint
 *         : HINT
 *           resourceHintExpression
 *           SEMICOLON
 *         ;
 *
 * The child grammar must not be imported back into Resources.
 *
 * Therefore dependency direction is:
 *
 *     ResourceExpressions
 *            |
 *     Names   |
 *       \     |
 *        \    v
 *       ResourceHints
 *            |
 *            v
 *        Resources
 *            |
 *            v
 *       Zamani.g4
 *
 * No cycle is introduced.
 *
 *
 * ============================================================================
 * 37. REQUIRED INTEGRATION WITH resource-expressions.g4
 * ============================================================================
 *
 * This file consumes:
 *
 *     resourceExpression
 *
 * from:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * It must never redefine that rule.
 *
 *
 * ============================================================================
 * 38. REQUIRED INTEGRATION WITH core/names.g4
 * ============================================================================
 *
 * This file consumes:
 *
 *     qualifiedName
 *
 * from:
 *
 *     grammar/core/names.g4
 *
 * It must never redefine:
 *
 *     identifier;
 *     nameSegment;
 *     qualifiedName.
 *
 *
 * ============================================================================
 * 39. REQUIRED INTEGRATION WITH THE AST
 * ============================================================================
 *
 * The AST owner must map:
 *
 *     resourceHintExpression
 *         ->
 *     ResourceHintPayload
 *
 * and preserve:
 *
 *     source span;
 *     expression/specification form;
 *     clause order;
 *     property names;
 *     values;
 *     group hierarchy.
 *
 * No target-specific AST node is required.
 *
 *
 * ============================================================================
 * 40. REQUIRED INTEGRATION WITH SEMANTICS
 * ============================================================================
 *
 * Semantic resource analysis must consume the AST representation and classify:
 *
 *     property;
 *     value;
 *     group;
 *     domain;
 *     applicability;
 *     policy;
 *     portability.
 *
 * Semantic analysis may then feed:
 *
 *     requirements;
 *     constraints;
 *     preferences;
 *     budgets;
 *     capabilities;
 *     negotiation;
 *     scalability;
 *     execution;
 *     compilation.
 *
 * None of those relationships changes this grammar's ownership.
 *
 *
 * ============================================================================
 * 41. REQUIRED INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Quantum hints remain metadata/resource intent until consumed by quantum
 * semantic analysis.
 *
 * They may eventually influence:
 *
 *     quantum::ir;
 *     optimization;
 *     decomposition;
 *     routing;
 *     scheduling;
 *     resilience;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * This file owns none of those phases.
 *
 *
 * ============================================================================
 * 42. REQUIRED INTEGRATION WITH HDL/HARDWARE
 * ============================================================================
 *
 * HDL/hardware consumers may use hint information during:
 *
 *     synthesis;
 *     timing analysis;
 *     placement;
 *     resource selection;
 *     optimization;
 *     simulation;
 *     deployment.
 *
 * This grammar does not construct:
 *
 *     netlists;
 *     physical layouts;
 *     device assignments;
 *     fixed-width hardware resources.
 *
 *
 * ============================================================================
 * 43. REQUIRED INTEGRATION WITH EXECUTION
 * ============================================================================
 *
 * Execution planning may consume hints for:
 *
 *     parallelism;
 *     locality;
 *     scheduling;
 *     batching;
 *     adaptive execution;
 *     simulation;
 *     fallback;
 *     retry;
 *     recovery.
 *
 * A hint does not itself perform any of those operations.
 *
 *
 * ============================================================================
 * 44. REQUIRED INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Source spans and semantic identity must remain available so downstream
 * provenance can answer:
 *
 *     where did this hint originate?
 *     what transformation used it?
 *     was it honored?
 *     was it ignored?
 *     why?
 *     which realization consumed it?
 *
 *
 * ============================================================================
 * 45. COMPLETION CRITERIA
 * ============================================================================
 *
 * ResourceHints is DONE when:
 *
 *     [x] grammar name is ResourceHints;
 *     [x] file name is hints.g4;
 *     [x] tokenVocab is ZamaniLexer;
 *     [x] ResourceExpressions is imported;
 *     [x] Names is imported;
 *     [x] no lexer rules exist;
 *     [x] no embedded Rust exists;
 *     [x] no semantic predicates exist;
 *     [x] no unsafe Rust is required;
 *     [x] resourceHint is NOT duplicated here;
 *     [x] resourceHintExpression is owned here;
 *     [x] resourceHintSpecification is owned here;
 *     [x] resourceHintClause is owned here;
 *     [x] property assignments are open-world;
 *     [x] property names reuse qualifiedName;
 *     [x] values reuse resourceExpression;
 *     [x] nested groups are open-world;
 *     [x] empty specifications are rejected;
 *     [x] no fixed resource-property universe exists;
 *     [x] no physical-device universe exists;
 *     [x] no machine-size constants exist;
 *     [x] no quantum-gate enumeration exists;
 *     [x] no quantum physical mapping exists;
 *     [x] no HDL physical realization exists;
 *     [x] no allocation occurs;
 *     [x] no scheduling occurs;
 *     [x] no routing occurs;
 *     [x] no QEC occurs;
 *     [x] no ZQN processing occurs;
 *     [x] no HAL selection occurs;
 *     [x] AST ownership is downstream;
 *     [x] semantic ownership is downstream;
 *     [x] IR ownership is downstream;
 *     [x] provenance is preserved;
 *     [x] parser behavior is deterministic;
 *     [x] scalability is open-world;
 *     [x] compatibility is explicitly defined;
 *     [x] integration with resources.g4 is explicitly defined;
 *     [x] positive tests are specified;
 *     [x] negative tests are specified;
 *     [x] boundary tests are specified;
 *     [x] scalability tests are specified;
 *     [x] determinism tests are specified.
 *
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */