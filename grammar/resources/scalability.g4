/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/scalability.g4
 *
 * GRAMMAR
 * -------
 * ResourceScalability
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE-SCALABILITY LEAF GRAMMAR
 *
 * BASELINE
 * --------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * Grammar-only.
 *
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no parser actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware discovery;
 *   - no resource allocation;
 *   - no runtime execution;
 *   - no unsafe Rust requirement.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns the syntax boundary for expressing SCALABILITY INTENT.
 *
 * Scalability describes how computational or resource demand may vary with
 * program-defined quantities, workload properties, data size, execution
 * context, or available capabilities.
 *
 * This grammar deliberately describes RELATIONSHIPS rather than physical
 * machine limits.
 *
 * Examples of valid semantic intent include:
 *
 *     scalability = input_size;
 *     scalability = problem_size * parallelism;
 *
 *     scalability input = input_size;
 *     scalability memory = required_memory;
 *     scalability qubits = logical_qubits;
 *
 *     scalability grows_with input_size;
 *     scalability depends_on workload_size;
 *     scalability bounded_by available_capacity;
 *
 *     scalability {
 *         input = input_size;
 *         memory = tensor_elements * element_size;
 *         qubits = logical_qubits;
 *     }
 *
 * The grammar does not decide what these names mean.
 *
 * Semantic analysis determines whether a named property represents:
 *
 *     - a scaling dimension;
 *     - workload;
 *     - demand;
 *     - capacity;
 *     - availability;
 *     - growth;
 *     - invariance;
 *     - adaptation;
 *     - portability;
 *     - another future scalability concept.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Resources
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     ResourceExpressions          ResourceScalability
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                   domain-neutral AST
 *                          |
 *                          v
 *                  semantic analysis
 *                          |
 *                  resource semantics
 *                          |
 *             canonical semantic model
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *      classical       quantum::ir     HDL/hardware
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *                  optimization/lowering
 *                          |
 *              routing / scheduling / QEC
 *                          |
 *                         ZQN
 *                          |
 *                         HAL
 *                          |
 *                  target realization
 *
 * This grammar does NOT directly depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     optimization
 *     HAL
 *     hardware discovery
 *     runtime allocation.
 *
 * Those systems consume the semantic representation produced after parsing.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     resourceScalabilitySpecification
 *     resourceScalabilityItem
 *     resourceScalabilityDeclaration
 *     resourceScalabilityNamedDeclaration
 *     resourceScalabilityRelationship
 *     resourceScalabilityAssignment
 *     resourceScalabilityGroup
 *     resourceScalabilityGroupItem
 *     resourceScalabilityProperty
 *     resourceScalabilityPropertyPath
 *     resourceScalabilityPropertySegment
 *     resourceScalabilityValue
 *     resourceScalabilityCondition
 *     resourceScalabilityExpressionList
 *     optionalResourceScalabilitySpecification
 *
 * THIS FILE DOES NOT OWN:
 *
 *     expression
 *     resourceExpression
 *     identifier
 *     qualifiedName
 *     resource
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceCapacity
 *     resourceAvailability
 *     resourcePortability
 *     resourceTarget
 *     placement
 *     routing
 *     scheduling
 *     allocation
 *     hardware discovery
 *     quantum::ir
 *     classical IR
 *     HDL IR
 *     QEC
 *     ZQN
 *     HAL
 *     runtime behavior.
 *
 * In particular, this file MUST NOT redefine:
 *
 *     resourceExpression
 *     expression
 *     identifier
 *     qualifiedName
 *
 * ============================================================================
 * 4. DEPENDENCY CONTRACT
 * ============================================================================
 *
 * ResourceExpressions owns the canonical resource-expression boundary.
 *
 * Therefore every scalability value is represented by:
 *
 *     resourceExpression
 *
 * This ensures that scalability does not create a second expression language.
 *
 * Names are intentionally represented through the canonical identifier rule.
 *
 * This grammar does not create a second name vocabulary.
 *
 * ============================================================================
 * 5. LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which consumes the assembled:
 *
 *     grammar/lexer/tokens.g4
 *
 * Actual repository token names used by this grammar include:
 *
 *     SCALABILITY
 *     CAPACITY
 *     AVAILABILITY
 *     PORTABILITY
 *     WITH
 *     WHEN
 *     WHERE
 *     FROM
 *     TO
 *     IN
 *     REQUIRES
 *     ENSURES
 *     INVARIANT
 *
 * together with canonical structural tokens such as:
 *
 *     ASSIGN
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *     DOT
 *     DOUBLE_COLON
 *
 * and the tokens consumed by:
 *
 *     resourceExpression
 *
 * IMPORTANT:
 *
 * This grammar deliberately does NOT reference speculative token names such
 * as:
 *
 *     K_SCALABILITY
 *     K_GROWS
 *     K_BOUNDED
 *     K_BY
 *     K_DEPENDS
 *     K_ON
 *     K_DIMENSION
 *     K_GROWTH
 *     K_RATE
 *     K_FACTOR
 *
 * unless those names are first established by the canonical lexer contract.
 *
 * The existing repository uses ordinary keyword token names such as:
 *
 *     SCALABILITY
 *     WITH
 *
 * so this file uses those canonical names.
 *
 * ============================================================================
 * 6. OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Scalability properties are deliberately open-world.
 *
 * The grammar does not enumerate:
 *
 *     input_size
 *     workload_size
 *     problem_size
 *     memory
 *     qubits
 *     nodes
 *     threads
 *     GPUs
 *     FPGAs
 *     tensor dimensions
 *     accelerator count
 *     timeline count
 *     future resource categories.
 *
 * These are semantic names.
 *
 * Therefore future computing domains can introduce new scalability
 * dimensions without changing this grammar merely because a new resource
 * category was invented.
 *
 * ============================================================================
 * 7. POCO-REAF
 * ============================================================================
 *
 * This grammar participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The source program describes scaling relationships.
 *
 * It does NOT prescribe the physical realization.
 *
 * For example:
 *
 *     scalability = input_size;
 *
 * means that a semantic quantity depends on input size.
 *
 * It does NOT mean:
 *
 *     maximum_input_size = N
 *
 * Likewise:
 *
 *     scalability bounded_by available_capacity;
 *
 * when represented semantically through a named property/value relationship
 * describes dependence on execution-context capacity.
 *
 * It does NOT establish:
 *
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_NODES
 *     MAX_THREADS
 *
 * ============================================================================
 * 8. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce universal limits such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_VECTOR_WIDTH
 *     MAX_DEVICE_COUNT
 *     MAX_ACCELERATORS
 *     MAX_TIMELINES
 *     MAX_SCALE
 *     MAX_DIMENSIONS
 *
 * It also MUST NOT encode equivalent fixed capacities through parser
 * alternatives.
 *
 * Numeric literals inside:
 *
 *     resourceExpression
 *
 * are program values.
 *
 * They are not language-level hardware limits.
 *
 * ============================================================================
 * 9. UNBOUNDED CARDINALITY
 * ============================================================================
 *
 * Repetition is deliberately expressed using:
 *
 *     *
 *
 * rather than finite alternatives.
 *
 * Therefore the language architecture does not impose a maximum number of:
 *
 *     scalability declarations
 *     scaling properties
 *     dimensions
 *     relationships
 *     groups
 *     nested group entries
 *     expressions
 *     qualified-name segments.
 *
 * Practical parser, memory, compiler, or runtime limits remain implementation
 * concerns and are not part of the source-language scalability contract.
 *
 * ============================================================================
 * 10. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * resourceScalabilitySpecification
 *
 * is the reusable public leaf entry point.
 *
 * It represents:
 *
 *     {
 *         scalability-item*
 *     }
 *
 * The source-level introducer belongs to the parent grammar that owns the
 * surrounding declaration/statement.
 *
 * This avoids inventing another global keyword or duplicating the concrete
 * resource statement owned by:
 *
 *     grammar/resources/resources.g4
 *
 * ============================================================================
 */

parser grammar ResourceScalability;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions;


/*
 * ============================================================================
 * 11. PUBLIC SCALABILITY SPECIFICATION
 * ============================================================================
 *
 * Example:
 *
 *     {
 *         scalability = input_size;
 *         scalability memory = required_memory;
 *     }
 *
 * The block may contain zero or more items.
 *
 * ============================================================================
 */

resourceScalabilitySpecification
    : LBRACE
      resourceScalabilityItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 12. SCALABILITY ITEM
 * ============================================================================
 */

resourceScalabilityItem
    : resourceScalabilityDeclaration
    | resourceScalabilityNamedDeclaration
    | resourceScalabilityRelationship
    | resourceScalabilityAssignment
    | resourceScalabilityGroup
    ;


/*
 * ============================================================================
 * 13. PRIMARY SCALABILITY DECLARATION
 * ============================================================================
 *
 * Canonical compact form:
 *
 *     scalability = input_size;
 *
 * This is the basic source-level scaling expression.
 *
 * ============================================================================
 */

resourceScalabilityDeclaration
    : SCALABILITY
      ASSIGN
      resourceScalabilityValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. NAMED SCALABILITY DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     scalability input = input_size;
 *     scalability memory = required_memory;
 *     scalability qubits = logical_qubits;
 *     scalability work = problem_size * parallelism;
 *
 * The property name is intentionally open-world.
 *
 * ============================================================================
 */

resourceScalabilityNamedDeclaration
    : SCALABILITY
      resourceScalabilityProperty
      ASSIGN
      resourceScalabilityValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. RELATIONSHIP FORM
 * ============================================================================
 *
 * Examples:
 *
 *     scalability grows_with input_size;
 *     scalability depends_on workload_size;
 *
 * Here:
 *
 *     grows_with
 *     depends_on
 *
 * are ordinary source-level identifiers.
 *
 * They are NOT global reserved keywords.
 *
 * This is deliberate:
 *
 *     - it avoids expanding the global lexer unnecessarily;
 *     - it keeps the grammar open-world;
 *     - semantic analysis can define the standard relationship vocabulary;
 *     - dialects can introduce additional relationships without requiring
 *       parser rewrites.
 *
 * The `WITH` token remains canonical and is used structurally.
 *
 * ============================================================================
 */

resourceScalabilityRelationship
    : SCALABILITY
      resourceScalabilityProperty
      WITH
      resourceScalabilityValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * This form is useful when a parent block has already established a
 * scalability context.
 *
 * Example:
 *
 *     scalability {
 *         input = input_size;
 *         memory = required_memory;
 *         qubits = logical_qubits;
 *     }
 *
 * ============================================================================
 */

resourceScalabilityAssignment
    : resourceScalabilityProperty
      ASSIGN
      resourceScalabilityValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. SCALABILITY GROUP
 * ============================================================================
 *
 * Groups permit arbitrarily nested semantic organization.
 *
 * Examples:
 *
 *     scalability {
 *         input = input_size;
 *
 *         memory {
 *             working = required_memory;
 *             persistent = dataset_size;
 *         }
 *     }
 *
 * Group names are open-world.
 *
 * The grammar does not decide whether a group represents:
 *
 *     memory
 *     compute
 *     quantum
 *     network
 *     tensor
 *     distributed
 *     hardware
 *     AI
 *     future domain.
 *
 * ============================================================================
 */

resourceScalabilityGroup
    : resourceScalabilityProperty
      LBRACE
      resourceScalabilityGroupItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 18. GROUP ITEM
 * ============================================================================
 */

resourceScalabilityGroupItem
    : resourceScalabilityAssignment
    | resourceScalabilityGroup
    | resourceScalabilityRelationship
    ;


/*
 * ============================================================================
 * 19. OPEN SCALABILITY PROPERTY
 * ============================================================================
 *
 * A property may be qualified.
 *
 * Examples:
 *
 *     input
 *     workload
 *     memory
 *     quantum::qubits
 *     tensor::elements
 *     hardware::capacity
 *     distributed::nodes
 *     future::resource::scale
 *
 * No maximum namespace depth is encoded.
 *
 * The grammar uses DOUBLE_COLON as the canonical qualified-name separator.
 *
 * ============================================================================
 */

resourceScalabilityProperty
    : resourceScalabilityPropertyPath
    ;


resourceScalabilityPropertyPath
    : resourceScalabilityPropertySegment
      (
          DOT resourceScalabilityPropertySegment
        | DOUBLE_COLON resourceScalabilityPropertySegment
      )*
    ;


resourceScalabilityPropertySegment
    : identifier
    ;


/*
 * ============================================================================
 * 20. VALUE
 * ============================================================================
 *
 * Every value is delegated to the canonical resource expression.
 *
 * This means scalability can use:
 *
 *     symbolic values
 *     arithmetic
 *     comparisons
 *     logical expressions
 *     calls
 *     indexing
 *     ranges
 *     properties
 *     dynamic expressions
 *     typed values
 *     future expression forms.
 *
 * The scalability grammar does not need to be changed merely because the
 * expression language gains another valid expression form.
 *
 * ============================================================================
 */

resourceScalabilityValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 21. CONDITION
 * ============================================================================
 *
 * A condition is also a canonical resource expression.
 *
 * Semantic analysis determines whether the expression is boolean/predicate
 * compatible.
 *
 * ============================================================================
 */

resourceScalabilityCondition
    : resourceExpression
    ;


/*
 * ============================================================================
 * 22. EXPRESSION LIST
 * ============================================================================
 *
 * Unbounded list of scalability values.
 *
 * No finite number of dimensions, factors, operands, or relationships is
 * encoded here.
 * ============================================================================
 */

resourceScalabilityExpressionList
    : resourceScalabilityValue
      (COMMA resourceScalabilityValue)*
      COMMA?
    ;


/*
 * ============================================================================
 * 23. OPTIONAL SPECIFICATION
 * ============================================================================
 */

optionalResourceScalabilitySpecification
    : resourceScalabilitySpecification?
    ;


/*
 * ============================================================================
 * 24. SEMANTIC VOCABULARY
 * ============================================================================
 *
 * The following concepts are intentionally SEMANTIC categories, not parser
 * alternatives:
 *
 *     growth
 *     dependency
 *     bound
 *     capacity
 *     availability
 *     adaptation
 *     invariance
 *     portability
 *     workload
 *     demand
 *     resource scaling
 *     computational scaling
 *     data scaling
 *     parallel scaling
 *     quantum scaling
 *     hardware scaling
 *     distributed scaling.
 *
 * Standard semantic property names may include:
 *
 *     grows_with
 *     depends_on
 *     bounded_by
 *     independent_of
 *     adapts_to
 *     preserves
 *     portable_across
 *
 * These names are intentionally not all reserved lexer keywords.
 *
 * This keeps the universal grammar extensible.
 *
 * Semantic validation is responsible for:
 *
 *     - recognizing standardized properties;
 *     - validating their expected value category;
 *     - validating their arity;
 *     - validating whether they are requirements, constraints, preferences,
 *       hints, or descriptive relationships;
 *     - validating compatibility with other resource contracts.
 *
 * ============================================================================
 * 25. RESOURCE SEMANTIC SEPARATION
 * ============================================================================
 *
 * Scalability must remain distinct from:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     availability
 *     capacity
 *     target selection
 *     placement
 *     allocation.
 *
 * For example:
 *
 *     requires qubits >= logical_qubits;
 *
 * is a REQUIREMENT.
 *
 * A scalability expression:
 *
 *     scalability qubits = logical_qubits;
 *
 * describes how a semantic quantity scales.
 *
 * A capability:
 *
 *     requires capability("quantum.measurement");
 *
 * describes a target capability requirement.
 *
 * A preference:
 *
 *     prefer latency <= latency_budget;
 *
 * describes advisory intent.
 *
 * A hardware placement decision such as:
 *
 *     physical_qubit(...)
 *
 * belongs downstream and MUST NOT be inferred merely from this grammar.
 *
 * ============================================================================
 * 26. CAPACITY AND AVAILABILITY
 * ============================================================================
 *
 * Scalability may refer to expressions representing execution-context
 * capacity or availability.
 *
 * Examples:
 *
 *     scalability capacity = available_capacity;
 *     scalability memory = available_memory;
 *     scalability nodes = available_nodes;
 *
 * The parser does not inspect those quantities.
 *
 * Their actual values come from downstream resource/capability discovery.
 *
 * This preserves POCO-REAF because the same source-level scaling intent can
 * be evaluated against different execution environments.
 *
 * ============================================================================
 * 27. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum scalability is represented semantically.
 *
 * Examples:
 *
 *     scalability qubits = logical_qubits;
 *     scalability ancilla = required_ancilla;
 *     scalability circuit_depth = workload_depth;
 *
 * No physical qubit identifiers are encoded.
 *
 * No finite qubit limit is encoded.
 *
 * No quantum gate enumeration is encoded.
 *
 * No topology is encoded.
 *
 * No calibration data is encoded.
 *
 * The downstream path remains:
 *
 *     scalability syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic resource model
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * 28. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical scaling may describe:
 *
 *     workload
 *     data
 *     operations
 *     memory demand
 *     parallelism
 *     vectorization
 *     tensor dimensions
 *     algorithmic scale.
 *
 * Examples:
 *
 *     scalability work = problem_size;
 *     scalability memory = element_count * element_size;
 *     scalability parallelism = workload_size;
 *
 * These are expressions, not machine limits.
 *
 * ============================================================================
 * 29. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-oriented scaling may describe:
 *
 *     ports
 *     lanes
 *     pipeline work
 *     memory demand
 *     data width
 *     replicated structures
 *     accelerator demand
 *     interconnect demand.
 *
 * Example:
 *
 *     scalability lanes = workload_width;
 *
 * This does NOT define a universal FPGA/ASIC width.
 *
 * A hardware backend later determines what realization satisfies the
 * semantic intent.
 *
 * ============================================================================
 * 30. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed programs may describe:
 *
 *     scalability nodes = required_nodes;
 *     scalability partitions = partition_count;
 *     scalability communication = communication_volume;
 *
 * No fixed number of nodes is encoded.
 *
 * No node identifiers are encoded.
 *
 * No topology is encoded.
 *
 * Placement and deployment remain downstream.
 *
 * ============================================================================
 * 31. AI / DATA INTEGRATION
 * ============================================================================
 *
 * AI/data workloads may describe:
 *
 *     dataset size
 *     batch size
 *     model size
 *     tensor elements
 *     parameter count
 *     inference workload
 *     training workload
 *     memory demand.
 *
 * The grammar does not encode:
 *
 *     a particular framework;
 *     a particular accelerator;
 *     a particular GPU;
 *     a particular tensor-rank maximum.
 *
 * ============================================================================
 * 32. ADAPTATION
 * ============================================================================
 *
 * Adaptation is represented as a semantic relationship rather than a parser
 * algorithm.
 *
 * A future standardized property may express concepts such as:
 *
 *     adapts_to
 *     scales_with
 *     bounded_by
 *     available_under
 *
 * without requiring a fixed vocabulary in this grammar.
 *
 * Semantic analysis determines whether adaptation is permitted and what
 * semantic invariants must be preserved.
 *
 * ============================================================================
 * 33. SEMANTIC PRESERVATION
 * ============================================================================
 *
 * A scalable realization MUST preserve the semantics of the source program.
 *
 * Scaling may change:
 *
 *     placement
 *     parallelism
 *     decomposition
 *     scheduling
 *     routing
 *     memory strategy
 *     accelerator usage
 *     distributed realization
 *     quantum physical realization
 *
 * while preserving the source-level computation's defined semantics.
 *
 * This grammar only exposes the source intent required by that analysis.
 *
 * ============================================================================
 * 34. AST CONTRACT
 * ============================================================================
 *
 * The grammar must lower into the existing domain-neutral frontend AST.
 *
 * Conceptually:
 *
 *     resourceScalabilityDeclaration
 *         ->
 *     scalability intent node
 *
 *     resourceScalabilityNamedDeclaration
 *         ->
 *     named scalability property
 *
 *     resourceScalabilityRelationship
 *         ->
 *     scalability relationship
 *
 *     resourceScalabilityAssignment
 *         ->
 *     scalability property/value
 *
 *     resourceScalabilityGroup
 *         ->
 *     nested scalability scope/group
 *
 * No new AST hierarchy should be created solely because this grammar is
 * modularized.
 *
 * Exact Rust structures belong to:
 *
 *     src/frontend/ast/
 *
 * ============================================================================
 * 35. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving scalability property names;
 *     - checking expression types;
 *     - determining relationship kinds;
 *     - distinguishing requirements from preferences and hints;
 *     - validating resource units/dimensions;
 *     - validating resource availability references;
 *     - validating portability;
 *     - validating domain compatibility;
 *     - detecting contradictory scalability contracts;
 *     - constructing canonical scalability semantics.
 *
 * The parser does none of these things.
 *
 * ============================================================================
 * 36. IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Scalability semantics are attached to the canonical semantic model and
 * lowered into the appropriate downstream representation.
 *
 * For quantum workloads:
 *
 *     semantic scalability
 *         ->
 *     quantum::ir metadata/requirements
 *
 * where appropriate.
 *
 * For classical workloads:
 *
 *     semantic scalability
 *         ->
 *     classical compilation/resource metadata.
 *
 * For HDL/hardware:
 *
 *     semantic scalability
 *         ->
 *     hardware/HDL lowering metadata.
 *
 * There must not be a second "scalability IR".
 *
 * ============================================================================
 * 37. COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler/resource infrastructure may use scalability semantics for:
 *
 *     - target selection;
 *     - resource feasibility;
 *     - specialization;
 *     - parallelization;
 *     - vectorization;
 *     - decomposition;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - distributed execution planning;
 *     - accelerator selection;
 *     - quantum realization.
 *
 * Those are downstream implementation decisions.
 *
 * The source grammar remains independent of the selected machine.
 *
 * ============================================================================
 * 38. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may evaluate scaling-related semantic expressions against
 * runtime context when the language semantics permit dynamic evaluation.
 *
 * The parser MUST NOT:
 *
 *     - query hardware;
 *     - inspect runtime capacity;
 *     - allocate resources;
 *     - choose a device;
 *     - schedule work;
 *     - perform negotiation.
 *
 * ============================================================================
 * 39. DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     grammar version
 *     canonical lexer vocabulary
 *     explicitly selected dialect configuration.
 *
 * Parsing must not depend on:
 *
 *     hardware availability
 *     runtime state
 *     network state
 *     filesystem state
 *     randomness
 *     wall-clock time
 *     compiler scheduling.
 *
 * ============================================================================
 * 40. SOURCE SPANS
 * ============================================================================
 *
 * Every parsed scalability construct must remain traceable to its source
 * range through the normal ANTLR token/context infrastructure.
 *
 * Downstream AST construction must preserve spans for:
 *
 *     property names
 *     relationship names
 *     values
 *     groups
 *     complete scalability declarations.
 *
 * This supports:
 *
 *     diagnostics
 *     IDE/LSP tooling
 *     formatting
 *     source maps
 *     compatibility tooling
 *     provenance.
 *
 * ============================================================================
 * 41. DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics are structural.
 *
 * Examples:
 *
 *     scalability = ;
 *     scalability input = ;
 *     scalability input;
 *     scalability { input = ; }
 *     scalability { }
 *
 * Semantic diagnostics are downstream.
 *
 * Examples:
 *
 *     unknown standardized scalability property;
 *     invalid scaling value type;
 *     incompatible scaling relationship;
 *     contradictory scalability contract;
 *     unavailable required capacity;
 *     non-portable implementation request.
 *
 * The parser must not silently turn semantic errors into successful
 * realizations.
 *
 * ============================================================================
 * 42. SECURITY
 * ============================================================================
 *
 * Scalability syntax must remain declarative and side-effect free.
 *
 * It must not:
 *
 *     execute code;
 *     execute commands;
 *     access secrets;
 *     access credentials;
 *     inspect hardware;
 *     contact providers;
 *     allocate resources;
 *     modify the filesystem;
 *     access the network.
 *
 * ============================================================================
 * 43. COMPATIBILITY
 * ============================================================================
 *
 * This file preserves the existing public concept:
 *
 *     scalability
 *
 * while removing dependency on speculative `K_*` token names.
 *
 * The existing concrete resource declaration owned by:
 *
 *     grammar/resources/resources.g4
 *
 * remains the owner of:
 *
 *     resourceScalabilityClause
 *
 * and therefore this file MUST NOT define that same rule.
 *
 * A parent grammar may import this grammar and delegate a block payload to:
 *
 *     resourceScalabilitySpecification
 *
 * without modifying this file.
 *
 * This is intentional independent-file integration.
 *
 * ============================================================================
 * 44. INTEGRATION WITH resources.g4
 * ============================================================================
 *
 * Current canonical ownership remains:
 *
 *     resources.g4
 *         owns resourceScalabilityClause
 *
 *     scalability.g4
 *         owns resourceScalabilitySpecification
 *
 * This avoids duplicate rule ownership.
 *
 * When a block form is exposed by resources.g4, the parent may import:
 *
 *     ResourceScalability
 *
 * and delegate its block payload to:
 *
 *     resourceScalabilitySpecification
 *
 * The parent must not recreate the internal scalability rules.
 *
 * Conversely, this file must never redefine:
 *
 *     resourceScalabilityClause
 *
 * ============================================================================
 * 45. INTEGRATION WITH resource-expressions.g4
 * ============================================================================
 *
 * This file imports:
 *
 *     ResourceExpressions
 *
 * and consumes:
 *
 *     resourceExpression
 *
 * It therefore inherits future expression-language improvements without
 * requiring this grammar to duplicate arithmetic, comparison, logical,
 * indexing, invocation, or literal syntax.
 *
 * ============================================================================
 * 46. INTEGRATION WITH requirements.g4
 * ============================================================================
 *
 * Requirements remain owned by:
 *
 *     grammar/resources/requirements.g4
 *
 * Scalability does not redefine:
 *
 *     resourceRequirement
 *     resourceRequirementExpression
 *
 * A requirement may refer semantically to a scalability expression, but
 * requirement classification remains downstream.
 *
 * ============================================================================
 * 47. INTEGRATION WITH constraints.g4
 * ============================================================================
 *
 * Constraints remain owned by:
 *
 *     grammar/resources/constraints.g4
 *
 * Scalability does not redefine constraint syntax.
 *
 * A constraint may consume scalability-derived semantic values after parsing.
 *
 * ============================================================================
 * 48. INTEGRATION WITH preferences.g4
 * ============================================================================
 *
 * Preferences remain owned by:
 *
 *     grammar/resources/preferences.g4
 *
 * Scalability does not redefine preference syntax.
 *
 * A preference may refer to scalability properties without changing the
 * ownership of preference semantics.
 *
 * ============================================================================
 * 49. INTEGRATION WITH portability.g4
 * ============================================================================
 *
 * Portability remains owned by:
 *
 *     grammar/resources/portability.g4
 *
 * This grammar may provide scalability values that portability analysis
 * consumes, but it does not duplicate portability rules.
 *
 * ============================================================================
 * 50. DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * The same scalability grammar applies to:
 *
 *     embedded computing
 *     classical CPU computing
 *     multicore computing
 *     GPU computing
 *     FPGA computing
 *     ASIC-oriented computation
 *     accelerator computing
 *     quantum computing
 *     quantum simulation
 *     hybrid computing
 *     AI/ML
 *     tensor computing
 *     distributed computing
 *     HPC
 *     cloud execution
 *     future computational substrates.
 *
 * No domain gets a private scalability language.
 *
 * ============================================================================
 * 51. NO HARDWARE IDENTIFIERS
 * ============================================================================
 *
 * This grammar does not encode:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     node0
 *     device0
 *     physical_qubit0
 *
 * Such strings, if lexically valid identifiers, remain names.
 *
 * Their physical interpretation belongs downstream.
 *
 * ============================================================================
 * 52. NO FIXED SCALE
 * ============================================================================
 *
 * The language supports scaling from:
 *
 *     one operation
 *     one resource
 *     one processor
 *     one accelerator
 *     one qubit
 *
 * through arbitrarily larger semantic workloads, subject to the actual
 * resources available to the implementation.
 *
 * "Infinity" here means:
 *
 *     no artificial finite ceiling is encoded by this grammar.
 *
 * It does NOT claim physically infinite:
 *
 *     memory
 *     compute
 *     devices
 *     qubits
 *     execution time
 *     network capacity.
 *
 * ============================================================================
 * 53. TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax cases MUST include:
 *
 *     scalability = input_size;
 *
 *     scalability = problem_size * parallelism;
 *
 *     scalability input = input_size;
 *
 *     scalability memory = required_memory;
 *
 *     scalability qubits = logical_qubits;
 *
 *     scalability grows_with input_size;
 *
 *     scalability depends_on workload_size;
 *
 *     scalability {
 *         input = input_size;
 *         memory = required_memory;
 *         qubits = logical_qubits;
 *     }
 *
 *     scalability {
 *         memory {
 *             working = working_memory;
 *             persistent = dataset_size;
 *         }
 *     }
 *
 *     scalability hardware::memory = required_memory;
 *
 *     scalability tensor::elements = tensor_size;
 *
 *     scalability quantum::logical_qubits = logical_qubits;
 *
 * Negative syntax cases MUST include:
 *
 *     scalability = ;
 *
 *     scalability input = ;
 *
 *     scalability input;
 *
 *     scalability = input_size
 *
 *     scalability { input = ; }
 *
 *     scalability { input }
 *
 *     scalability input = , other;
 *
 * Boundary cases MUST include:
 *
 *     one scalability item;
 *     arbitrarily many scalability items;
 *     deeply nested scalability groups;
 *     deeply qualified property names;
 *     arbitrarily long expression lists;
 *     symbolic scaling quantities;
 *     large numeric program values;
 *     dynamic resource expressions.
 *
 * Scalability tests MUST verify:
 *
 *     no finite number of dimensions is encoded;
 *     no finite number of properties is encoded;
 *     no finite nesting depth is specified by language semantics;
 *     no hardware cardinality is encoded;
 *     no resource capacity is hard-coded.
 *
 * Determinism tests MUST parse identical source identically.
 *
 * Compatibility tests MUST verify that:
 *
 *     resources.g4
 *     resource-expressions.g4
 *     requirements.g4
 *     constraints.g4
 *     preferences.g4
 *     portability.g4
 *
 * do not introduce competing definitions of this grammar's owned rules.
 *
 * ============================================================================
 * 54. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_VECTOR_WIDTH
 *     MAX_DEVICE_COUNT
 *     MAX_ACCELERATORS
 *     MAX_TIMELINES
 *
 * Forbidden indirect forms include fixed alternatives such as:
 *
 *     one | two | four | eight
 *
 * when used to represent universal hardware capacity.
 *
 * Numeric values are permitted only as source expressions.
 *
 * This file contains no hardware capacity constant.
 *
 * ============================================================================
 * 55. PERFORMANCE
 * ============================================================================
 *
 * The grammar favors:
 *
 *     bounded local alternatives;
 *     canonical expression delegation;
 *     iterative repetition;
 *     recursive grouping only where source nesting requires it.
 *
 * It does not introduce semantic predicates or runtime actions.
 *
 * Any implementation-level parser stack or memory limits are outside the
 * language semantics.
 *
 * ============================================================================
 * 56. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] canonical parser grammar is used;
 *     [x] canonical ZamaniLexer vocabulary is consumed;
 *     [x] no speculative K_* lexer tokens are required;
 *     [x] ResourceExpressions remains the sole owner of resourceExpression;
 *     [x] identifier syntax is not duplicated;
 *     [x] scalability has a single leaf grammar;
 *     [x] resourceScalabilityClause remains owned by Resources;
 *     [x] block scalability has an independent public entry point;
 *     [x] properties are open-world;
 *     [x] qualified properties are supported;
 *     [x] nested groups are supported;
 *     [x] values reuse resourceExpression;
 *     [x] no finite scalability cardinality is encoded;
 *     [x] no hardware capacity is encoded;
 *     [x] no quantum gate enumeration exists;
 *     [x] no physical resource selection exists;
 *     [x] no placement/routing/scheduling logic exists;
 *     [x] no QEC/ZQN/HAL logic exists;
 *     [x] POCO-REAF remains target-independent;
 *     [x] source-span preservation is specified;
 *     [x] diagnostics are separated from semantic validation;
 *     [x] AST ownership remains domain-neutral;
 *     [x] canonical semantic/IR lowering remains downstream;
 *     [x] Rust integration requires no unsafe;
 *     [x] positive/negative/boundary/scalability/determinism tests are defined.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */