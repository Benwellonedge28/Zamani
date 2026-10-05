/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/requirements.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * GRAMMAR NAME
 * ------------
 * Requirements
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE-REQUIREMENT SYNTAX AUTHORITY
 *
 * IMPLEMENTATION BASELINE
 * ------------------------
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
 * This file owns source-level REQUIREMENTS whose subject is the resource
 * realization of a Zamani program.
 *
 * A resource requirement expresses what a program, operation, module,
 * execution plan, domain, or contract requires from an available realization.
 *
 * Examples:
 *
 *     requires qubits >= logical_qubits;
 *     requires memory >= required_memory;
 *     requires nodes >= required_nodes;
 *     requires bandwidth >= required_bandwidth;
 *     requires latency <= latency_budget;
 *
 *     requires capability("quantum.measurement");
 *     requires capability("tensor.compute");
 *
 *     requires capability quantum::measurement;
 *
 *     requires capability quantum::dynamic_control version >= 1.2.0;
 *
 * Resource requirements describe INTENT.
 *
 * They do not:
 *
 *     - select hardware;
 *     - allocate hardware;
 *     - reserve hardware;
 *     - route operations;
 *     - schedule operations;
 *     - perform quantum routing;
 *     - perform QEC;
 *     - select a backend;
 *     - inspect hardware;
 *     - query runtime state;
 *     - execute code.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file exclusively owns:
 *
 *     resourceRequirement
 *     resourceRequirementExpression
 *     resourceCapabilityCall
 *     resourceCapabilityReference
 *     resourceRequirementList
 *     optionalResourceRequirementList
 *     resourceRequirementGroup
 *     resourceRequirementGroupItem
 *     resourceRequirementExpressionList
 *     optionalResourceRequirementExpressionList
 *     resourceRequirementPredicate
 *     resourceRequirementCondition
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     requirement
 *     requirementList
 *     requirementDeclaration
 *
 * Those belong to:
 *
 *     grammar/core/requirements.g4
 *
 * It also does not own:
 *
 *     expression
 *     arithmetic
 *     comparison precedence
 *     logical precedence
 *     identifiers
 *     qualified names
 *     literals
 *     capability declarations
 *     capability identity
 *     capability version syntax
 *     resource declarations
 *     constraints
 *     preferences
 *     hints
 *     budgets
 *     availability
 *     capacity
 *     scalability
 *     policies
 *     effects
 *     contracts
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
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Direct parser dependencies:
 *
 *     grammar/resources/resource-expressions.g4
 *     grammar/core/names.g4
 *     grammar/core/capabilities.g4
 *
 * Lexer boundary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The imported resource-expression grammar owns:
 *
 *     resourceExpression
 *     resourceExpressionList
 *     optionalResourceExpressionList
 *
 * The imported capability grammar owns:
 *
 *     capabilityReference
 *     capabilityExpression
 *     capabilityVersionClause
 *
 * This file MUST NOT duplicate those definitions.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Primary:
 *
 *     resourceRequirement
 *     resourceRequirementExpression
 *
 * Capability-specific:
 *
 *     resourceCapabilityCall
 *     resourceCapabilityReference
 *
 * Collection:
 *
 *     resourceRequirementList
 *     optionalResourceRequirementList
 *     resourceRequirementExpressionList
 *     optionalResourceRequirementExpressionList
 *
 * Reusable semantic boundaries:
 *
 *     resourceRequirementPredicate
 *     resourceRequirementCondition
 *     resourceRequirementGroup
 *     resourceRequirementGroupItem
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary:
 *
 *     grammar/resources/resources.g4
 *
 * Known resource/semantic consumers include:
 *
 *     grammar/validation/requires.g4
 *     grammar/quantum/resource-requirements.g4
 *     grammar/hybrid/hybrid-resources.g4
 *     grammar/hardware/
 *     grammar/hdl/
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/execution/
 *     grammar/compile/
 *
 * Consumers MUST use the public rules exported by this file.
 *
 * They MUST NOT reproduce resource-requirement syntax locally.
 *
 * ============================================================================
 * AST OWNER
 * ============================================================================
 *
 * The grammar creates parser contexts only.
 *
 * The domain-neutral frontend AST owns the resulting representation.
 *
 * The AST must preserve at least:
 *
 *     - complete source span;
 *     - requirement expression;
 *     - expression grouping;
 *     - capability-call structure;
 *     - capability-reference structure;
 *     - argument ordering;
 *     - source ordering.
 *
 * The AST MUST NOT contain:
 *
 *     - physical device IDs;
 *     - physical qubit IDs;
 *     - GPU IDs;
 *     - CPU IDs;
 *     - FPGA IDs;
 *     - memory-bank IDs;
 *     - backend-specific allocation;
 *     - routing decisions;
 *     - scheduling decisions.
 *
 * ============================================================================
 * SEMANTIC OWNER
 * ============================================================================
 *
 * Resource semantic analysis owns:
 *
 *     - requirement classification;
 *     - name resolution;
 *     - capability resolution;
 *     - unit/dimensional validation;
 *     - type checking;
 *     - comparison validity;
 *     - requirement normalization;
 *     - contradiction detection;
 *     - satisfiability;
 *     - target feasibility;
 *     - resource negotiation;
 *     - availability evaluation;
 *     - resource budgeting;
 *     - policy interaction;
 *     - effect interaction.
 *
 * The parser does none of these.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Resource requirements do not directly lower into target instructions.
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     resource requirement AST
 *       |
 *       v
 *     semantic requirement
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 * capability analysis    resource analysis
 *       |                    |
 *       +---------+----------+
 *                 |
 *                 v
 *         execution planning
 *                 |
 *                 +------------------+
 *                 |                  |
 *                 v                  v
 *          classical path       quantum path
 *                                      |
 *                                      v
 *                                  quantum::ir
 *
 * This file does not define an IR.
 *
 * ============================================================================
 * SPEC OWNER
 * ============================================================================
 *
 * Normative resource specification:
 *
 *     grammar/spec/resources.md
 *
 * Normative POCO-REAF specification:
 *
 *     grammar/specification/poco-reaf.md
 *
 * Grammar authority:
 *
 *     grammar/DESIGN.md
 *     grammar/README.md
 *     grammar/grammar.md
 *
 * ============================================================================
 * TEST OWNER
 * ============================================================================
 *
 * Required test areas:
 *
 *     grammar/tests/resources/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/quantum/
 *     grammar/tests/hardware/
 *     grammar/tests/hybrid/
 *     grammar/tests/distributed/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This is a pure ANTLR parser grammar.
 *
 * It contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no resource discovery;
 *     - no runtime execution;
 *     - no randomness.
 *
 * Generated Zamani frontend code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not require unsafe Rust.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Resource requirements are one of the mechanisms that allow a Zamani source
 * program to remain independent of machine scale.
 *
 * The source describes:
 *
 *     WHAT is required.
 *
 * The compiler/resource system determines:
 *
 *     WHERE and HOW that requirement is realized.
 *
 * Therefore:
 *
 *     requires qubits >= logical_qubits;
 *
 * does not select a QPU.
 *
 *     requires memory >= required_memory;
 *
 * does not select a memory device.
 *
 *     requires nodes >= required_nodes;
 *
 * does not select a cluster.
 *
 *     requires capability("tensor.compute");
 *
 * does not select a GPU.
 *
 * This distinction is fundamental to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Resource kinds and capability identities are OPEN-WORLD.
 *
 * This grammar MUST NOT enumerate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     vendor
 *     device model
 *     cloud provider
 *     future processor
 *     future quantum architecture
 *     future accelerator
 *
 * New resource categories and capabilities should normally be introduced
 * through semantic registries, specifications, dialects, or target metadata.
 *
 * The universal resource grammar should not need to change merely because a
 * new machine type appears.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO language-level finite limit on:
 *
 *     - number of requirements;
 *     - number of expressions;
 *     - number of capability arguments;
 *     - number of groups;
 *     - number of nested groups;
 *     - qualified-name depth;
 *     - source program size;
 *     - resource categories;
 *     - resource quantities;
 *     - quantum resource quantities;
 *     - distributed resource quantities;
 *     - tensor resource quantities.
 *
 * ANTLR repetition uses:
 *
 *     *
 *     +
 *
 * rather than finite enumerations.
 *
 * "Infinity" means that the language architecture introduces no artificial
 * machine-capacity ceiling.
 *
 * It does NOT claim that:
 *
 *     physical hardware;
 *     operating systems;
 *     compiler memory;
 *     parser memory;
 *     deployment infrastructure
 *
 * are literally infinite.
 *
 * Those are implementation or target constraints.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce:
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
 * It must also not encode equivalent indirect limits.
 *
 * For example, the grammar MUST NOT contain parser alternatives representing:
 *
 *     exactly 32 CPUs;
 *     exactly 1024 qubits;
 *     exactly 64 GB memory;
 *     exactly 24 GB accelerator memory;
 *     exactly 32-bit registers.
 *
 * A literal such as:
 *
 *     1024
 *
 * is a PROGRAM VALUE when it occurs in:
 *
 *     requires qubits >= 1024;
 *
 * It is never interpreted by this grammar as a universal capacity.
 *
 * ============================================================================
 * RESOURCE REQUIREMENT VS UNIVERSAL REQUIREMENT
 * ============================================================================
 *
 * The repository already has:
 *
 *     grammar/core/requirements.g4
 *
 * That file owns generic requirements such as:
 *
 *     requires quantum::measurement;
 *     requires execution::deterministic;
 *
 * This file owns RESOURCE-SPECIFIC requirements such as:
 *
 *     requires qubits >= logical_qubits;
 *     requires memory >= required_memory;
 *     requires capability("tensor.compute");
 *
 * This file therefore deliberately does NOT define:
 *
 *     requirement
 *     requirementList
 *     requirementDeclaration
 *
 * Doing so would create competing semantic ownership.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * The canonical capability grammar is:
 *
 *     grammar/core/capabilities.g4
 *
 * Its primary capability reference syntax is:
 *
 *     capabilityReference
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     tensor::compute
 *     distributed::collectives
 *     security::trusted_execution
 *
 * A resource requirement may consume this canonical reference through:
 *
 *     resourceCapabilityReference
 *
 * In addition, this resource grammar preserves the convenient resource-level
 * call form:
 *
 *     capability("quantum.measurement")
 *
 * This call form is intentionally a RESOURCE REQUIREMENT syntax bridge.
 *
 * It does not redefine capability identity.
 *
 * The semantic layer must normalize both forms to the same capability
 * requirement representation.
 *
 * ============================================================================
 * CAPABILITY CALL CONTRACT
 * ============================================================================
 *
 * Canonical examples:
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("vendor.feature");
 *
 *     requires capability(namespace::feature);
 *
 *     requires capability(feature_name);
 *
 *     requires capability("quantum.operation", operation);
 *
 * Arguments remain canonical resource expressions.
 *
 * The grammar does not interpret:
 *
 *     string contents;
 *     capability namespace;
 *     capability provider;
 *     capability version;
 *     capability availability.
 *
 * Those belong to semantic analysis.
 *
 * ============================================================================
 * RESOURCE EXPRESSIONS
 * ============================================================================
 *
 * All quantitative and predicate expressions are delegated to:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * which delegates further to the canonical expression grammar.
 *
 * Therefore this file does NOT redefine:
 *
 *     expression
 *     arithmeticExpression
 *     comparisonExpression
 *     logicalExpression
 *     unaryExpression
 *     primaryExpression
 *
 * This guarantees that:
 *
 *     x + y
 *     x * y
 *     x >= y
 *     x <= y
 *     x == y
 *     x && y
 *     x || y
 *     !x
 *
 * have the same underlying expression semantics as the rest of Zamani.
 *
 * ============================================================================
 * SOURCE FORMS
 * ============================================================================
 *
 * Resource requirement:
 *
 *     requires <resource-expression>;
 *
 * Capability-call requirement:
 *
 *     requires capability(<resource-expression-list>);
 *
 * Canonical capability-reference requirement:
 *
 *     requires capability <capability-reference>;
 *
 * Resource requirement group:
 *
 *     requires {
 *         ...
 *     }
 *
 * The grouped form is structural syntax only and does not introduce a second
 * resource requirement semantic model.
 *
 * ============================================================================
 * RESOURCE QUANTITIES
 * ============================================================================
 *
 * Valid semantic quantities may include:
 *
 *     qubits
 *     logical_qubits
 *     physical_qubits
 *     memory
 *     storage
 *     nodes
 *     threads
 *     lanes
 *     bandwidth
 *     throughput
 *     latency
 *     energy
 *     power
 *     tensor_elements
 *     tensor_rank
 *     work
 *     problem_size
 *
 * These names are NOT enumerated by this grammar.
 *
 * They are ordinary expressions.
 *
 * This keeps the language open to future resource categories.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum source may express:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("quantum.mid_circuit_measurement");
 *
 *     requires capability("quantum.dynamic_control");
 *
 *     requires capability("quantum.error_correction");
 *
 * The grammar does not know:
 *
 *     physical qubit topology;
 *     coupling map;
 *     calibration;
 *     routing;
 *     scheduling;
 *     QEC implementation.
 *
 * Semantic requirements may influence the pipeline:
 *
 *     source
 *       ->
 *     semantic quantum model
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
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *
 * No quantum IR is created here.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical programs may express:
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("vector.compute");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("parallel.compute");
 *
 * The grammar does not assume:
 *
 *     fixed CPU count;
 *     fixed register width;
 *     fixed cache size;
 *     fixed vector width.
 *
 * ============================================================================
 * GPU / FPGA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Portable requirements may express:
 *
 *     requires capability("gpu.compute");
 *
 *     requires capability("fpga.compute");
 *
 *     requires capability("accelerator.tensor");
 *
 *     requires capability("accelerator.reconfigurable");
 *
 * The source does not select a device.
 *
 * Target negotiation happens downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Portable distributed requirements may express:
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("distributed.execution");
 *
 *     requires capability("distributed.collectives");
 *
 *     requires capability("networking.low_latency");
 *
 * No maximum node count is encoded.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-oriented programs may express:
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     requires latency <= latency_budget;
 *
 *     requires capability("hardware.streaming");
 *
 *     requires capability("hardware.reconfiguration");
 *
 * This file does not define:
 *
 *     wire widths;
 *     physical cells;
 *     pin counts;
 *     FPGA resource counts;
 *     register widths;
 *     routing;
 *     synthesis;
 *     placement.
 *
 * ============================================================================
 * AI / DATA / TENSOR INTEGRATION
 * ============================================================================
 *
 * Requirements may express:
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
 * Tensor dimensions and rank remain semantic values.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Resource requirements may be consumed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * validation and contract systems.
 *
 * This file owns only the resource requirement syntax.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Resource requirements may be evaluated under policies controlling:
 *
 *     resource selection;
 *     permitted capabilities;
 *     security;
 *     adaptation;
 *     deployment;
 *     execution;
 *     simulation;
 *     fallback.
 *
 * Policies do not change the syntax owned here.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * A resource requirement itself does not perform an effect.
 *
 * Semantic analysis may associate the requirement with an operation whose
 * effects include:
 *
 *     network
 *     distributed
 *     measurement
 *     native
 *     foreign
 *     learning
 *     adaptation
 *     simulation
 *
 * This grammar does not authorize effects.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * The source span and structure of every resource requirement must remain
 * traceable through semantic analysis.
 *
 * Downstream provenance may record:
 *
 *     requirement;
 *     evidence;
 *     capability resolution;
 *     resource decision;
 *     target decision;
 *     fallback;
 *     compilation transformation.
 *
 * The parser does not create those records.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic for a given token stream and parser configuration.
 *
 * This file contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no runtime calls;
 *     no I/O;
 *     no hardware discovery;
 *     no network access;
 *     no randomness.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DEFINITION
 * ============================================================================
 */

parser grammar Requirements;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Capabilities, Names;


/*
 * ============================================================================
 * 1. PRIMARY RESOURCE REQUIREMENT
 * ============================================================================
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
 *     requires nodes >= required_nodes;
 *
 *     requires latency <= latency_budget;
 *
 *     requires bandwidth >= required_bandwidth;
 *
 * ============================================================================
 */

resourceRequirement
    : REQUIRES resourceRequirementExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 2. RESOURCE REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * The expression boundary remains open-world.
 *
 * The capability-call and capability-reference alternatives are kept
 * structurally distinct from the general resource expression so that:
 *
 *     capability(...)
 *
 * and:
 *
 *     capability quantum::measurement
 *
 * can both be represented without redefining the general expression grammar.
 *
 * ============================================================================
 */

resourceRequirementExpression
    : resourceCapabilityCall
    | resourceCapabilityReference
    | resourceExpression
    ;


/*
 * ============================================================================
 * 3. RESOURCE CAPABILITY CALL
 * ============================================================================
 *
 * Resource-specific call form:
 *
 *     capability("tensor.compute")
 *
 *     capability("quantum.measurement")
 *
 *     capability(namespace::feature)
 *
 *     capability(feature_name)
 *
 *     capability("quantum.operation", operation)
 *
 * The arguments are expressions.
 *
 * This permits an open capability namespace without enumerating capability
 * names in this grammar.
 *
 * ============================================================================
 */

resourceCapabilityCall
    : CAPABILITY
      LPAREN
      optionalResourceRequirementExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * 4. CANONICAL CAPABILITY REFERENCE
 * ============================================================================
 *
 * Canonical capability-reference form:
 *
 *     capability quantum::measurement
 *
 *     capability quantum::dynamic_control version >= 1.2.0
 *
 * The capability identity/version grammar is owned by core/capabilities.g4.
 *
 * This wrapper gives resource requirements a stable integration boundary.
 *
 * ============================================================================
 */

resourceCapabilityReference
    : CAPABILITY
      capabilityReference
    ;


/*
 * ============================================================================
 * 5. RESOURCE REQUIREMENT EXPRESSION LIST
 * ============================================================================
 *
 * A capability call may contain zero or more arguments.
 *
 * Empty argument lists remain syntactically representable:
 *
 *     capability()
 *
 * Semantic validation determines whether an empty capability call is valid.
 *
 * No parser-level provider or arity limit is imposed.
 *
 * ============================================================================
 */

resourceRequirementExpressionList
    : resourceExpression
      (COMMA resourceExpression)*
      COMMA?
    ;


optionalResourceRequirementExpressionList
    : resourceRequirementExpressionList?
    ;


/*
 * ============================================================================
 * 6. REQUIREMENT LIST
 * ============================================================================
 *
 * This is specifically a LIST OF RESOURCE REQUIREMENT STATEMENTS.
 *
 * It is intentionally named:
 *
 *     resourceRequirementList
 *
 * and MUST NOT be confused with:
 *
 *     grammar/core/requirements.g4::requirementList
 *
 * ============================================================================
 */

resourceRequirementList
    : resourceRequirement*
    ;


optionalResourceRequirementList
    : resourceRequirementList?
    ;


/*
 * ============================================================================
 * 7. RESOURCE REQUIREMENT GROUP
 * ============================================================================
 *
 * Grouped resource requirements provide a reusable structural boundary for
 * consumers that need multiple requirements under one resource context.
 *
 * Example:
 *
 *     requires {
 *         qubits >= logical_qubits;
 *         memory >= required_memory;
 *         capability("quantum.measurement");
 *     }
 *
 * The group does not create a new semantic resource type.
 *
 * ============================================================================
 */

resourceRequirementGroup
    : REQUIRES
      LBRACE
      resourceRequirementGroupItem*
      RBRACE
    ;


resourceRequirementGroupItem
    : resourceRequirement
    | resourceRequirementGroup
    ;


/*
 * ============================================================================
 * 8. RESOURCE REQUIREMENT PREDICATE
 * ============================================================================
 *
 * Explicit semantic boundary for consumers that need a requirement predicate.
 *
 * The parser delegates to the canonical resource expression.
 *
 * Type checking determines whether the resulting expression is a valid
 * predicate.
 *
 * ============================================================================
 */

resourceRequirementPredicate
    : resourceExpression
    ;


resourceRequirementCondition
    : resourceRequirementPredicate
    ;


/*
 * ============================================================================
 * 9. OPTIONAL CAPABILITY CALL ARGUMENTS
 * ============================================================================
 *
 * This rule is intentionally local to resource capability-call syntax.
 *
 * It does NOT replace:
 *
 *     resourceExpressionList
 *
 * as the canonical resource-expression list authority.
 *
 * The distinction exists because capability(...) is a resource requirement
 * syntax boundary, while arbitrary resource expressions use the canonical
 * resource-expression grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 10. SEMANTIC REQUIREMENT CATEGORIES
 * ============================================================================
 *
 * The following are semantic categories, not grammar enumerations:
 *
 *     compute
 *     memory
 *     storage
 *     bandwidth
 *     latency
 *     throughput
 *     energy
 *     power
 *     reliability
 *     resilience
 *     qubits
 *     nodes
 *     threads
 *     tensor resources
 *     accelerator resources
 *     network resources
 *     hardware resources
 *     simulation resources
 *     AI/data resources
 *     future resources
 *
 * New categories do not require parser changes merely because their names
 * change.
 *
 * ============================================================================
 * 11. REQUIREMENT SATISFIABILITY
 * ============================================================================
 *
 * The parser MUST accept syntactically valid requirements even when their
 * satisfaction is unknown.
 *
 * Example:
 *
 *     requires capability("future.architecture.feature");
 *
 * is syntactically valid.
 *
 * Semantic analysis may classify it as:
 *
 *     satisfied
 *     unsatisfied
 *     unknown
 *     conditional
 *
 * depending on the compilation/execution context.
 *
 * ============================================================================
 * 12. TYPE AND UNIT VALIDATION
 * ============================================================================
 *
 * The grammar does not determine whether:
 *
 *     qubits >= memory
 *
 * is meaningful.
 *
 * Type/unit/resource analysis determines whether both sides of the comparison
 * are compatible.
 *
 * The same principle applies to:
 *
 *     latency
 *     bandwidth
 *     energy
 *     power
 *     throughput
 *     memory
 *     tensor dimensions
 *     topology values
 *     resource quantities.
 *
 * ============================================================================
 * 13. SYMBOLIC RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * Requirements may be symbolic:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= dataset_size * element_size;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires bandwidth >= workload_bandwidth;
 *
 *     requires latency <= latency_budget;
 *
 * This is essential for source portability.
 *
 * The grammar does not evaluate these expressions.
 *
 * ============================================================================
 * 14. RESOURCE REQUIREMENTS MUST NOT SELECT TARGETS
 * ============================================================================
 *
 * Invalid architectural interpretation:
 *
 *     requires GPU 0;
 *
 *     requires QPU 1;
 *
 *     requires CPU 3;
 *
 * Portable resource requirements should instead express intent:
 *
 *     requires capability("gpu.compute");
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("parallel.compute");
 *
 * Physical target selection belongs downstream.
 *
 * ============================================================================
 * 15. RESOURCE REQUIREMENTS MUST NOT ALLOCATE
 * ============================================================================
 *
 * A requirement does not reserve or acquire a resource.
 *
 * These are separate semantic concepts owned by other resource grammars:
 *
 *     reservation
 *     acquisition
 *     allocation
 *     placement
 *     scheduling
 *
 * ============================================================================
 * 16. RESOURCE REQUIREMENTS MUST NOT CREATE HARDWARE LIMITS
 * ============================================================================
 *
 * This is valid:
 *
 *     requires qubits >= 1024;
 *
 * This does not establish:
 *
 *     MAX_QUBITS = 1024
 *
 * Likewise:
 *
 *     requires memory >= 64GB;
 *
 * does not establish:
 *
 *     MAX_MEMORY = 64GB
 *
 * ============================================================================
 * 17. QUANTUM BOUNDARY
 * ============================================================================
 *
 * Resource requirements may influence quantum compilation.
 *
 * They do not directly construct quantum operations.
 *
 * The boundary remains:
 *
 *     resource requirement
 *          |
 *          v
 *     semantic resource model
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * ============================================================================
 * 18. CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical resource requirements participate in the canonical semantic model
 * and may influence:
 *
 *     optimization
 *     parallelization
 *     vectorization
 *     accelerator selection
 *     memory planning
 *     execution planning
 *
 * They do not directly select hardware.
 *
 * ============================================================================
 * 19. HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * HDL and hardware consumers may use:
 *
 *     resourceRequirement
 *
 * without modifying this grammar.
 *
 * Hardware-specific semantic analysis may interpret:
 *
 *     bandwidth
 *     latency
 *     timing
 *     power
 *     energy
 *     capacity
 *     reconfiguration
 *     streaming
 *
 * without adding hardware-specific parser alternatives here.
 *
 * ============================================================================
 * 20. DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Distributed consumers may use:
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("distributed.execution");
 *
 *     requires capability("distributed.collectives");
 *
 *     requires capability("networking.communication");
 *
 * The number of nodes is a semantic value.
 *
 * No finite node limit exists in the grammar.
 *
 * ============================================================================
 * 21. AI / DATA BOUNDARY
 * ============================================================================
 *
 * Resource requirements can express:
 *
 *     requires memory >= model_memory;
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("tensor.acceleration");
 *
 *     requires capability("data.streaming");
 *
 *     requires capability("distributed.training");
 *
 * Model semantics remain in their appropriate domain subsystem.
 *
 * ============================================================================
 * 22. ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Resource requirements may participate in adaptive execution.
 *
 * Example semantic flow:
 *
 *     requirement
 *        |
 *        v
 *     capability/resource evaluation
 *        |
 *        +--> satisfied
 *        |
 *        +--> unavailable
 *        |
 *        +--> conditional
 *        |
 *        +--> fallback
 *        |
 *        v
 *     execution planning
 *
 * This grammar does not implement adaptation.
 *
 * ============================================================================
 * 23. SIMULATION
 * ============================================================================
 *
 * A resource requirement may be evaluated against a simulator when actual
 * hardware is unavailable.
 *
 * Example:
 *
 *     requires capability("quantum.measurement");
 *
 * may be satisfied by a quantum simulator if the semantic execution policy
 * explicitly permits that realization.
 *
 * The grammar does not decide whether simulation is acceptable.
 *
 * ============================================================================
 * 24. PROVENANCE
 * ============================================================================
 *
 * Every requirement must remain traceable to its source.
 *
 * Downstream records may contain:
 *
 *     source span
 *     normalized requirement
 *     capability resolution
 *     resource resolution
 *     target decision
 *     fallback
 *     evidence
 *     policy decision
 *
 * ============================================================================
 * 25. ERROR CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify at minimum:
 *
 *     missing REQUIRES
 *     missing expression
 *     missing capability arguments where syntax requires them
 *     malformed capability reference
 *     missing semicolon
 *     malformed grouped requirement
 *     malformed expression
 *
 * Semantic diagnostics should separately identify:
 *
 *     unknown resource
 *     incompatible resource units
 *     unsatisfied requirement
 *     unavailable capability
 *     incompatible capability version
 *     contradictory requirements
 *     forbidden resource/capability
 *     impossible target realization
 *
 * The parser MUST NOT report a physical feasibility error as a lexical or
 * parser error merely because the current machine cannot satisfy a requirement.
 *
 * ============================================================================
 * 26. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Public rule names in this file are stable API:
 *
 *     resourceRequirement
 *     resourceRequirementExpression
 *     resourceCapabilityCall
 *     resourceCapabilityReference
 *     resourceRequirementList
 *     optionalResourceRequirementList
 *
 * Changes to these rules are compatibility-sensitive.
 *
 * Generic core requirement rules must remain owned by:
 *
 *     grammar/core/requirements.g4
 *
 * A migration must be provided if a public resource rule changes.
 *
 * ============================================================================
 * 27. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source tokens
 *     grammar version
 *     parser configuration
 *
 * this grammar must produce the same parser structure.
 *
 * No environmental state may affect parsing.
 *
 * ============================================================================
 * 28. NEGATIVE TESTS
 * ============================================================================
 *
 * The following categories must be rejected or diagnosed appropriately:
 *
 *     requires;
 *
 *     requires >= memory;
 *
 *     requires capability;
 *
 *     requires capability(;
 *
 *     requires capability("x";
 *
 *     requires qubits >= ;
 *
 *     requires memory >= ;
 *
 *     requires {;
 *
 *     requires { qubits >= n;
 *
 *     requires qubits >= n
 *
 * where the final semicolon is required by the selected syntax.
 *
 * Physical impossibility MUST NOT be confused with parser invalidity.
 *
 * ============================================================================
 * 29. POSITIVE TESTS
 * ============================================================================
 *
 * The conformance suite must cover:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires latency <= latency_budget;
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("quantum.dynamic_control");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("distributed.collectives");
 *
 *     requires capability("future.architecture.feature");
 *
 *     requires capability(namespace::feature);
 *
 *     requires capability(feature_name);
 *
 *     requires capability("quantum.operation", operation);
 *
 *     requires capability quantum::measurement;
 *
 *     requires capability quantum::dynamic_control version >= 1.2.0;
 *
 *     requires {
 *         qubits >= logical_qubits;
 *         memory >= required_memory;
 *         capability("quantum.measurement");
 *     }
 *
 * ============================================================================
 * 30. BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     symbolic quantities
 *     large numeric literals
 *     zero
 *     negative values where expression syntax permits them
 *     computed quantities
 *     nested expressions
 *     qualified capability names
 *     long qualified names
 *     capability arguments
 *     nested resource groups
 *     mixed resource domains
 *     quantum + classical requirements
 *     distributed + networking requirements
 *     AI + accelerator requirements
 *     HDL + hardware requirements
 *
 * ============================================================================
 * 31. SCALABILITY TESTS
 * ============================================================================
 *
 * The suite must test progressively larger:
 *
 *     requirement counts
 *     expression sizes
 *     capability argument counts
 *     grouped requirements
 *     qualified names
 *     symbolic expressions
 *
 * The tests must never convert test sizes into language constants.
 *
 * ============================================================================
 * 32. CROSS-DOMAIN TEST
 * ============================================================================
 *
 * At least one integration fixture must combine:
 *
 *     classical resources
 *     quantum resources
 *     hardware resources
 *     accelerator capabilities
 *     distributed resources
 *     networking capabilities
 *     tensor requirements
 *     execution capabilities
 *
 * in a single source program.
 *
 * The requirement parser must remain domain-neutral.
 *
 * ============================================================================
 * 33. HARD-CODING AUDIT
 * ============================================================================
 *
 * Before marking this file complete, verify that it contains:
 *
 *     [ ] no machine-capacity constants;
 *     [ ] no quantum-capacity constants;
 *     [ ] no CPU/GPU/FPGA/QPU enumeration;
 *     [ ] no vendor enumeration;
 *     [ ] no device IDs;
 *     [ ] no fixed tensor rank;
 *     [ ] no fixed network size;
 *     [ ] no fixed node count;
 *     [ ] no fixed memory size;
 *     [ ] no fixed register width;
 *     [ ] no parser-level resource allocation;
 *     [ ] no hardware inspection;
 *     [ ] no runtime execution;
 *     [ ] no unsafe Rust requirement.
 *
 * ============================================================================
 * 34. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before declaring this file DONE:
 *
 *     [ ] ResourceExpressions imports successfully.
 *     [ ] Capabilities imports successfully.
 *     [ ] Names imports successfully.
 *     [ ] ZamaniLexer provides every referenced token.
 *     [ ] Resources consumes resourceRequirement.
 *     [ ] Validation consumes resourceRequirement.
 *     [ ] Quantum consumers use resourceRequirement rather than redefining it.
 *     [ ] Hardware consumers use resourceRequirement rather than redefining it.
 *     [ ] Hybrid consumers use resourceRequirement rather than redefining it.
 *     [ ] Distributed consumers use resourceRequirement rather than redefining it.
 *     [ ] No generic requirement rule is duplicated here.
 *     [ ] No expression grammar is duplicated here.
 *     [ ] No capability identity grammar is duplicated here.
 *     [ ] AST mapping is documented.
 *     [ ] Semantic mapping is documented.
 *     [ ] Resource resolution is downstream.
 *     [ ] Capability resolution is downstream.
 *     [ ] quantum::ir remains the canonical quantum IR.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Scalability tests exist.
 *     [ ] Determinism tests exist.
 *     [ ] Compatibility tests exist.
 *     [ ] Rust 1.97 compatibility is verified by the repository build.
 *     [ ] Rust 1.97.1 compatibility is verified by the repository build.
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * 35. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. It is the single resource-specific requirement syntax owner.
 *
 *     2. It does not redefine generic core requirements.
 *
 *     3. It consumes the canonical resource-expression grammar.
 *
 *     4. It consumes the canonical capability grammar.
 *
 *     5. It supports symbolic and computed resource quantities.
 *
 *     6. It supports open-world capability requirements.
 *
 *     7. It introduces no machine-capacity ceiling.
 *
 *     8. It introduces no vendor/device catalogue.
 *
 *     9. It does not allocate or select hardware.
 *
 *    10. It preserves source structure for the AST.
 *
 *    11. It integrates with classical, quantum, hybrid, HDL, hardware,
 *        distributed, networking, AI/data, execution and compilation
 *        subsystems through stable grammar interfaces.
 *
 *    12. It preserves the quantum::ir boundary.
 *
 *    13. It has positive, negative, boundary and scalability tests.
 *
 *    14. Generated frontend code remains compatible with Rust 1.97/1.97.1
 *        and safe Rust.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file describes RESOURCE REQUIREMENT INTENT.
 *
 * It does not describe a machine.
 *
 * It does not describe a backend.
 *
 * It does not describe a physical device.
 *
 * It does not describe a fixed machine capacity.
 *
 * It does not decide feasibility.
 *
 * It does not perform allocation.
 *
 * It does not create an IR.
 *
 * It therefore preserves the central Zamani separation:
 *
 *     SOURCE MEANING
 *          |
 *          v
 *     RESOURCE INTENT
 *          |
 *          v
 *     SEMANTIC ANALYSIS
 *          |
 *          v
 *     CAPABILITY / RESOURCE NEGOTIATION
 *          |
 *          v
 *     EXECUTION PLANNING
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     CLASSICAL PATH        QUANTUM PATH
 *                               |
 *                               v
 *                           quantum::ir
 *                               |
 *                               v
 *                     target realization
 *
 * ============================================================================
 */

resourceRequirement
    : REQUIRES resourceRequirementExpression SEMICOLON
    ;

resourceRequirementExpression
    : resourceCapabilityCall
    | resourceCapabilityReference
    | resourceExpression
    ;

resourceCapabilityCall
    : CAPABILITY
      LPAREN
      optionalResourceExpressionList
      RPAREN
    ;

resourceCapabilityReference
    : CAPABILITY
      capabilityReference
    ;

resourceRequirementList
    : resourceRequirement*
    ;

optionalResourceRequirementList
    : resourceRequirementList?
    ;

resourceRequirementExpressionList
    : resourceExpression
      (COMMA resourceExpression)*
      COMMA?
    ;

optionalResourceRequirementExpressionList
    : resourceRequirementExpressionList?
    ;

resourceRequirementGroup
    : REQUIRES
      LBRACE
      resourceRequirementGroupItem*
      RBRACE
    ;

resourceRequirementGroupItem
    : resourceRequirement
    | resourceRequirementGroup
    ;

resourceRequirementPredicate
    : resourceExpression
    ;

resourceRequirementCondition
    : resourceRequirementPredicate
    ;