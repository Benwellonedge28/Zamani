/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/entrypoints.g4
 *
 * Grammar:
 *     ExecutionEntryPoints
 *
 * Status:
 *     Production-ready source-level execution-entry-point grammar component
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL ENTRY-POINT CONTRACT for Zamani.
 *
 * An entry point identifies a semantically callable computation that may be
 * selected as an externally reachable execution boundary.
 *
 * It describes:
 *
 *     - entry-point identity;
 *     - referenced computation;
 *     - optional signature contract;
 *     - optional alias;
 *     - input/output contract;
 *     - execution requirements;
 *     - capability requirements;
 *     - resource requirements;
 *     - portability requirements;
 *     - target intent;
 *     - lifecycle intent;
 *     - observability intent;
 *     - determinism intent;
 *     - failure/recovery intent;
 *     - execution properties;
 *     - entry-point metadata.
 *
 * This grammar does NOT:
 *
 *     - execute the entry point;
 *     - select a physical machine;
 *     - select a CPU core;
 *     - select a GPU;
 *     - select an FPGA region;
 *     - select a QPU;
 *     - select physical qubits;
 *     - allocate memory;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform optimization;
 *     - perform QEC;
 *     - perform ZQN processing;
 *     - perform calibration;
 *     - perform hardware discovery;
 *     - perform deployment;
 *     - create an execution plan;
 *     - create a runtime implementation;
 *     - create a second quantum IR.
 *
 * ============================================================================
 * FUNDAMENTAL MODEL
 * ============================================================================
 *
 * An entry point is a SOURCE-LEVEL SEMANTIC BOUNDARY.
 *
 * It connects:
 *
 *     source-level callable computation
 *                 |
 *                 v
 *          entry-point intent
 *                 |
 *                 v
 *        semantic validation
 *                 |
 *                 v
 *       canonical semantic model
 *                 |
 *        +--------+--------+
 *        |        |        |
 *        v        v        v
 *     classical quantum::ir HDL/hardware
 *        |        |        |
 *        +--------+--------+
 *                 |
 *                 v
 *        optimization/lowering
 *                 |
 *        +--------+--------+
 *        |        |        |
 *        v        v        v
 *     routing scheduling resilience
 *                          |
 *                          v
 *                         ZQN
 *                          |
 *                          v
 *                         HAL
 *                          |
 *                          v
 *                    target/runtime
 *
 * The entry-point grammar therefore describes WHAT is externally executable,
 * not HOW that execution is physically realized.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Entry points MUST remain portable.
 *
 * A valid entry point may eventually be realized on:
 *
 *     - a tiny embedded target;
 *     - a CPU;
 *     - many CPUs;
 *     - a GPU;
 *     - many GPUs;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum processor;
 *     - a quantum simulator;
 *     - a heterogeneous accelerator;
 *     - an HPC system;
 *     - a distributed system;
 *     - a cloud environment;
 *     - an edge system;
 *     - a future execution architecture.
 *
 * The entry-point grammar MUST NOT encode the physical realization.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Entry-point cardinality is structurally unbounded.
 *
 * The grammar contains no universal limits on:
 *
 *     - number of entry points;
 *     - number of parameters;
 *     - number of return values;
 *     - number of requirements;
 *     - number of capabilities;
 *     - number of resources;
 *     - number of attributes;
 *     - number of policies;
 *     - number of execution targets;
 *     - number of supported domains.
 *
 * Repetition is represented through recursive/iterative grammar structure.
 *
 * Practical limits imposed by:
 *
 *     - parser memory;
 *     - compiler resources;
 *     - runtime resources;
 *     - operating-system limits;
 *     - deployment resources;
 *
 * are implementation/environment limits and MUST NOT become language
 * semantics.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     MAX_ENTRYPOINTS
 *     MAX_PARAMETERS
 *     MAX_ARGUMENTS
 *     MAX_RETURNS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * It MUST NOT encode equivalent hidden limits.
 *
 * Numeric values appearing in expressions are PROGRAM VALUES.
 *
 * For example:
 *
 *     requires qubits >= n;
 *
 * is valid.
 *
 * The value of n is not a language-level resource ceiling.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Portable entry points MUST NOT require:
 *
 *     cpu(0)
 *     gpu(0)
 *     qpu(0)
 *     fpga(0)
 *     node(0)
 *     device(0)
 *     physical_qubit(0)
 *
 * as their universal representation.
 *
 * Physical realization belongs to:
 *
 *     hardware;
 *     resources;
 *     placement;
 *     routing;
 *     scheduling;
 *     HAL;
 *     runtime;
 *     deployment;
 *     target-specific interoperability.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - entry-point declaration syntax;
 *     - entry-point identity;
 *     - entry-point computation reference;
 *     - entry-point alias syntax;
 *     - entry-point signature contract;
 *     - entry-point parameter contract;
 *     - entry-point return contract;
 *     - entry-point execution-policy composition;
 *     - entry-point requirement references;
 *     - entry-point capability references;
 *     - entry-point resource references;
 *     - entry-point portability intent;
 *     - entry-point target intent;
 *     - entry-point lifecycle intent;
 *     - entry-point metadata;
 *     - entry-point properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - types;
 *     - function declarations;
 *     - function bodies;
 *     - modules;
 *     - resource semantics;
 *     - capability semantics;
 *     - scheduling;
 *     - placement;
 *     - synchronization;
 *     - dispatch;
 *     - deployment;
 *     - runtime implementation;
 *     - hardware discovery;
 *     - hardware realization;
 *     - quantum operations;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL/hardware IR;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - optimization;
 *     - calibration;
 *     - HAL.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * An entry point is NOT a function declaration.
 *
 * Function syntax remains owned by:
 *
 *     grammar/functions/
 *
 * Therefore this grammar does not define:
 *
 *     fn
 *     function bodies
 *     generic function implementation
 *     closure implementation
 *     calling-convention implementation
 *
 * Instead, an entry point references an existing callable semantic entity.
 *
 * Conceptually:
 *
 *     fn compute(...) -> T { ... }
 *
 *     entrypoint compute;
 *
 * or:
 *
 *     entrypoint compute(...)
 *         -> T;
 *
 * The semantic analyzer verifies that the referenced callable exists and
 * that an optional entry-point signature agrees with it.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes:
 *
 *     ZamaniLexer
 *
 * and imports only canonical parser-composition grammars:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * EntryPoints MUST NOT import:
 *
 *     Execution
 *     Runtime
 *     Scheduling
 *     Placement
 *     Dispatch
 *     Deployment
 *
 * merely to reuse their concepts.
 *
 * This keeps dependency direction acyclic.
 *
 * Canonical direction:
 *
 *     EntryPoints
 *          |
 *          v
 *     Execution composition
 *
 * NOT:
 *
 *     EntryPoints -> Execution -> EntryPoints
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar ExecutionEntryPoints;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC COMPOSITION CONTRACT
 * ============================================================================
 *
 * `entryPointDeclaration` is the only public declaration entry point owned by
 * this grammar.
 *
 * Execution.g4 consumes this rule.
 *
 * Runtime.g4 consumes this rule indirectly through Execution.
 * ============================================================================
 */

entryPointDeclaration
    : entryPointMarker
      entryPointName
      entryPointTail
    ;


/*
 * ============================================================================
 * 2. CONTEXTUAL ENTRY-POINT MARKER
 * ============================================================================
 *
 * The current lexical authority does not reserve `entrypoint` as a dedicated
 * keyword token.
 *
 * Therefore this grammar deliberately uses IDENTIFIER through the canonical
 * `identifier` parser rule.
 *
 * Semantic analysis MUST validate the exact contextual spelling:
 *
 *     entrypoint
 *
 * This avoids silently modifying the lexical authority merely to introduce
 * this leaf grammar.
 *
 * If `entrypoint` is promoted to a reserved keyword in a future compatible
 * lexical version, this rule may be migrated according to the normal language
 * compatibility process.
 *
 * Until then, this file remains independently usable with the existing
 * ZamaniLexer.
 * ============================================================================
 */

entryPointMarker
    : identifier
    ;


/*
 * ============================================================================
 * 3. ENTRY-POINT NAME
 * ============================================================================
 *
 * Entry points may reference:
 *
 *     local names
 *     qualified names
 *     module-qualified names
 *     domain-qualified names
 *     future semantic namespaces
 *
 * Meaning is resolved semantically.
 * ============================================================================
 */

entryPointName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 4. ENTRY-POINT TAIL
 * ============================================================================
 *
 * An entry point may contain:
 *
 *     - a signature contract;
 *     - an alias;
 *     - a body-less declaration;
 *     - an execution policy block;
 *     - requirements;
 *     - capabilities;
 *     - resources;
 *     - portability intent;
 *     - lifecycle intent;
 *     - generic properties.
 *
 * The entry-point grammar never requires a physical implementation.
 * ============================================================================
 */

entryPointTail
    : entryPointClause*
      entryPointTerminator
    ;


/*
 * ============================================================================
 * 5. TERMINATION
 * ============================================================================
 *
 * A body is intentionally NOT owned here.
 *
 * Entry-point declarations describe an existing callable computation.
 * ============================================================================
 */

entryPointTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 6. ENTRY-POINT CLAUSES
 * ============================================================================
 *
 * Clause order is intentionally not semantically significant.
 *
 * Semantic analysis normalizes clauses into a canonical representation.
 *
 * This allows independently authored entry-point contracts without coupling
 * their source ordering to implementation behavior.
 * ============================================================================
 */

entryPointClause
    : entryPointSignature
    | entryPointAlias
    | entryPointRequirement
    | entryPointCapabilityRequirement
    | entryPointResourceRequirement
    | entryPointPortabilityRequirement
    | entryPointTargetIntent
    | entryPointLifecycleIntent
    | entryPointExecutionProperty
    | entryPointAttribute
    ;


/*
 * ============================================================================
 * 7. SIGNATURE CONTRACT
 * ============================================================================
 *
 * This is a CONTRACT, not a second function declaration.
 *
 * The semantic analyzer must compare this contract with the referenced
 * callable.
 *
 * ============================================================================
 */

entryPointSignature
    : LPAREN
      entryPointParameterList?
      RPAREN
      entryPointReturnClause?
    ;


entryPointParameterList
    : entryPointParameter
      (COMMA entryPointParameter)*
      COMMA?
    ;


entryPointParameter
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
    ;


entryPointReturnClause
    : THIN_ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * 8. ALIAS
 * ============================================================================
 *
 * An alias is a source-level symbolic name.
 *
 * It does not imply a physical endpoint, URL, device, process, or deployment
 * identity.
 * ============================================================================
 */

entryPointAlias
    : AS
      identifier
    ;


/*
 * ============================================================================
 * 9. REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe conditions that must hold for a valid realization.
 *
 * They do not select physical resources.
 *
 * ============================================================================
 */

entryPointRequirement
    : REQUIRES
      expression
    ;


/*
 * ============================================================================
 * 10. CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Capability identity is intentionally open-ended.
 *
 * No finite list such as:
 *
 *     CPU
 *     GPU
 *     QPU
 *     FPGA
 *
 * is encoded here.
 *
 * ============================================================================
 */

entryPointCapabilityRequirement
    : CAPABILITY
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 11. RESOURCE REQUIREMENT
 * ============================================================================
 *
 * Resource expressions remain expressions.
 *
 * Examples:
 *
 *     resource(qubits >= n)
 *     resource(memory >= required_memory)
 *     resource(nodes >= required_nodes)
 *
 * The actual resource interpretation belongs to semantic/resource analysis.
 * ============================================================================
 */

entryPointResourceRequirement
    : RESOURCE
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 12. PORTABILITY REQUIREMENT
 * ============================================================================
 *
 * Portability is semantic intent, not target selection.
 * ============================================================================
 */

entryPointPortabilityRequirement
    : PORTABILITY
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 13. TARGET INTENT
 * ============================================================================
 *
 * `target` here is an ABSTRACT execution-target expression.
 *
 * It MUST NOT be interpreted by the parser as:
 *
 *     CPU number
 *     GPU number
 *     QPU number
 *     FPGA number
 *     node number
 *     device number
 *
 * Semantic target resolution occurs later.
 * ============================================================================
 */

entryPointTargetIntent
    : TARGET
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 14. LIFECYCLE INTENT
 * ============================================================================
 *
 * Lifecycle vocabulary is intentionally open-ended.
 *
 * The grammar does not enumerate a finite runtime state machine.
 *
 * Semantic systems may use states such as:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * without making that list a grammar-level limitation.
 *
 * ============================================================================
 */

entryPointLifecycleIntent
    : entryPointLifecycleMarker
      expression?
    ;


entryPointLifecycleMarker
    : identifier
    ;


/*
 * ============================================================================
 * 15. GENERIC EXECUTION PROPERTY
 * ============================================================================
 *
 * Generic properties provide forward-compatible extension without creating
 * a new keyword for every future execution technology.
 *
 * Example conceptual forms:
 *
 *     deterministic = true
 *     isolation = expression
 *     timeout = expression
 *     observability = expression
 *     recovery = expression
 *     resilience = expression
 *
 * Property meaning is resolved semantically.
 * ============================================================================
 */

entryPointExecutionProperty
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 16. ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * Attributes remain owned by the canonical Core attribute grammar.
 *
 * This adapter intentionally uses the existing generic attribute entry point
 * instead of creating a second attribute language.
 * ============================================================================
 */

entryPointAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 17. OPTIONAL ENTRY-POINT REFERENCE FORM
 * ============================================================================
 *
 * Some execution systems need to refer to an entry point without redeclaring
 * its full contract.
 *
 * The same declaration syntax remains canonical:
 *
 *     entrypoint compute;
 *
 * The semantic analyzer determines whether `compute` is a callable,
 * executable module boundary, quantum kernel, HDL/software boundary, or
 * another legal execution subject.
 *
 * No domain-specific entry-point enum is introduced.
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntax only.
 *
 * Semantic analysis MUST subsequently determine:
 *
 *     - whether entryPointMarker spells "entrypoint";
 *     - whether entryPointName resolves;
 *     - whether the resolved entity is callable/executable;
 *     - whether its visibility permits external entry;
 *     - whether the optional signature matches;
 *     - whether parameter defaults are legal;
 *     - whether the return contract matches;
 *     - whether aliases conflict;
 *     - whether requirements are satisfiable;
 *     - whether capability requirements are supported;
 *     - whether resource requirements are feasible;
 *     - whether portability requirements are compatible;
 *     - whether target intent is legal;
 *     - whether lifecycle intent is compatible;
 *     - whether properties are recognized or intentionally extensible;
 *     - whether duplicate/conflicting clauses exist.
 *
 * The parser MUST NOT perform these checks.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. REQUIREMENT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * These constructs MUST remain semantically distinct.
 *
 * Requirement:
 *
 *     requires qubits >= n
 *
 * Capability:
 *
 *     capability("quantum.mid_circuit_measurement")
 *
 * Resource:
 *
 *     resource(memory >= required_memory)
 *
 * Target intent:
 *
 *     target(capability("quantum"))
 *
 * Portability:
 *
 *     portability(expression)
 *
 * They MUST NOT be collapsed into a physical-device selection.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * An entry point may reference:
 *
 *     classical functions
 *     kernels
 *     services
 *     applications
 *     numerical computations
 *     distributed computations
 *
 * No classical-specific entry-point grammar is required.
 *
 * The same semantic entry-point contract applies.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. QUANTUM INTEGRATION
 * ============================================================================
 *
 * An entry point may reference:
 *
 *     quantum circuits
 *     quantum kernels
 *     quantum functions
 *     hybrid kernels
 *     logical quantum computations
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * The entry-point grammar does NOT enumerate gates.
 *
 * It does NOT enumerate qubits.
 *
 * It does NOT allocate physical qubits.
 *
 * It does NOT perform routing.
 *
 * It does NOT perform QEC.
 *
 * It does NOT create another quantum IR.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * An entry point may reference:
 *
 *     HDL modules
 *     hardware/software co-design boundaries
 *     accelerator kernels
 *     synthesis entry points
 *     simulation entry points
 *     verification entry points
 *
 * Hardware realization remains downstream.
 *
 * This grammar does not encode:
 *
 *     wire widths;
 *     physical FPGA regions;
 *     ASIC instances;
 *     fixed clocks;
 *     fixed memory capacities;
 *     physical pins.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. HYBRID INTEGRATION
 * ============================================================================
 *
 * A single entry point may reference a computation containing:
 *
 *     classical computation
 *     quantum computation
 *     measurement
 *     classical feed-forward
 *     accelerator computation
 *     distributed computation
 *     HDL/hardware execution
 *
 * No separate hybrid entry-point syntax is necessary.
 *
 * Hybrid semantics are determined by the referenced computation and semantic
 * analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed entry points use resource/capability/placement intent rather
 * than fixed node identifiers.
 *
 * Valid semantic examples include:
 *
 *     requires nodes >= n
 *
 *     capability("distributed.execution")
 *
 *     resource(network.bandwidth >= required_bandwidth)
 *
 * No node-count ceiling is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Entry points may reference:
 *
 *     model inference
 *     training
 *     tensor kernels
 *     data pipelines
 *     agents
 *     scientific workloads
 *     distributed learning
 *
 * AI frameworks and vendor APIs remain outside this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. EXECUTION-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The canonical execution composition should become:
 *
 *     Execution
 *         |
 *         +--> EntryPoints
 *         |
 *         +--> ExecutionContext
 *         |
 *         +--> Dispatch
 *         |
 *         +--> Placement
 *         |
 *         +--> Scheduling
 *         |
 *         +--> Synchronization
 *         |
 *         +--> Runtime
 *         |
 *         +--> Deployment
 *
 * `execution.g4` owns composition.
 *
 * `entrypoints.g4` owns entry-point syntax.
 *
 * Neither file should duplicate the other's rules.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. RUNTIME INTEGRATION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The existing:
 *
 *     grammar/execution/runtime.g4
 *
 * currently contains a legacy `runtimeEntryPointDeclaration`.
 *
 * That rule MUST cease to be an independent owner.
 *
 * The required final relationship is:
 *
 *     runtime.g4
 *          |
 *          +--> entryPointDeclaration
 *
 * Runtime remains the owner of:
 *
 *     runtimeEnvironmentDeclaration
 *     runtimeStatement
 *     runtimeExpression
 *     runtimePolicyDeclaration
 *     runtimeRequirementDeclaration
 *     runtimeCapabilityDeclaration
 *
 * EntryPoints owns:
 *
 *     entryPointDeclaration
 *
 * This prevents duplicate entry-point AST contracts.
 *
 * `runtime.g4` MUST NOT import Execution merely to obtain entry points.
 *
 * Instead the canonical `Execution` composition grammar imports both
 * `Runtime` and `ExecutionEntryPoints`.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. DISPATCH INTEGRATION
 * ============================================================================
 *
 * Dispatch remains owned by:
 *
 *     grammar/execution/dispatch.g4
 *
 * Entry-point declaration does not dispatch anything.
 *
 * Conceptual flow:
 *
 *     entrypoint declaration
 *          |
 *          v
 *     semantic entry-point identity
 *          |
 *          v
 *     dispatch intent
 *          |
 *          v
 *     dispatch planning
 *          |
 *          v
 *     runtime/deployment
 *
 * This grammar MUST NOT import Dispatch.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Placement remains owned by:
 *
 *     grammar/execution/placement.g4
 *
 * Entry-point target intent may eventually be lowered into placement intent.
 *
 * This grammar does not duplicate:
 *
 *     locality;
 *     affinity;
 *     anti-affinity;
 *     physical placement;
 *     topology;
 *     routing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling remains owned by:
 *
 *     grammar/execution/scheduling.g4
 *
 * Entry-point properties may express scheduling-related intent through generic
 * semantic properties or through execution context, but this file does not
 * implement a scheduler or scheduling algorithm.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Generic resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * EntryPoints only provides the syntactic attachment point:
 *
 *     resource(expression)
 *
 * Semantic analysis maps the expression into the canonical resource model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Generic capability semantics remain owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/core/capabilities.g4
 *
 * This grammar does not enumerate capabilities.
 *
 * Therefore future capabilities can be introduced without changing this
 * grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. PORTABILITY INTEGRATION
 * ============================================================================
 *
 * Portability semantics remain governed by:
 *
 *     grammar/spec/portability.md
 *
 *     grammar/specification/portability.md
 *
 * Entry-point portability is an intent contract.
 *
 * The compiler determines whether a target can realize it.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. AST CONTRACT
 * ============================================================================
 *
 * The grammar maps conceptually to:
 *
 *     EntryPointDecl
 *
 * with semantic fields equivalent to:
 *
 *     name
 *     subject
 *     alias
 *     parameters
 *     return_type
 *     requirements
 *     capabilities
 *     resources
 *     portability
 *     target_intent
 *     lifecycle
 *     properties
 *     attributes
 *     source_span
 *
 * This is a DOMAIN-NEUTRAL AST contract.
 *
 * The actual Rust AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * No runtime-specific IR is created here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. SEMANTIC LOWERING CONTRACT
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     entryPointDeclaration
 *             |
 *             v
 *     EntryPointDecl
 *             |
 *             v
 *     semantic EntryPointIntent
 *             |
 *             +--------------------------+
 *             |                          |
 *             v                          v
 *     callable resolution        requirement/capability analysis
 *             |                          |
 *             +-------------+------------+
 *                           |
 *                           v
 *                  canonical semantic model
 *                           |
 *             +-------------+-------------+
 *             |             |             |
 *             v             v             v
 *        classical     quantum::ir   HDL/hardware
 *             |             |             |
 *             +-------------+-------------+
 *                           |
 *                           v
 *                 optimization/lowering
 *                           |
 *                  routing/scheduling
 *                           |
 *                 resilience/QEC/ZQN
 *                           |
 *                           v
 *                          HAL
 *                           |
 *                           v
 *                     runtime/deployment
 *
 * The entry-point grammar must not introduce any additional IR boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend MUST preserve source spans for:
 *
 *     - entry-point marker;
 *     - entry-point name;
 *     - alias;
 *     - each parameter;
 *     - parameter type;
 *     - default expression;
 *     - return type;
 *     - every requirement;
 *     - every capability;
 *     - every resource;
 *     - every portability clause;
 *     - target intent;
 *     - lifecycle intent;
 *     - property;
 *     - attribute.
 *
 * This is required for:
 *
 *     diagnostics;
 *     IDE/LSP;
 *     formatting;
 *     refactoring;
 *     provenance;
 *     compatibility analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to ANTLR/parser diagnostics.
 *
 * Semantic errors belong to semantic analysis.
 *
 * Examples:
 *
 * Syntax:
 *
 *     entrypoint ;
 *
 * Semantic:
 *
 *     entrypoint missing_function;
 *
 * Type:
 *
 *     entrypoint compute(x: InvalidType);
 *
 * Resource:
 *
 *     entrypoint compute
 *         requires resource(qubits >= impossible_requirement);
 *
 * Capability:
 *
 *     entrypoint compute
 *         capability("unsupported.capability");
 *
 * Portability:
 *
 *     entrypoint compute
 *         portability(...);
 *
 * The parser MUST NOT probe hardware or perform resource discovery to diagnose
 * these cases.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing this grammar must be deterministic.
 *
 * It MUST NOT depend on:
 *
 *     - current time;
 *     - randomness;
 *     - environment variables;
 *     - hardware;
 *     - available CPUs;
 *     - available GPUs;
 *     - available QPUs;
 *     - network state;
 *     - filesystem state;
 *     - runtime state.
 *
 * Equivalent canonical token streams must produce equivalent parse trees.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no environment access;
 *     - no hardware probing;
 *     - no runtime callbacks;
 *     - no credentials;
 *     - no unsafe Rust.
 *
 * It is therefore a pure source-language parsing boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. RUST CONTRACT
 * ============================================================================
 *
 * This grammar itself contains no Rust implementation code.
 *
 * Generated parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The Zamani implementation MUST use safe Rust only.
 *
 * No `unsafe` block, `unsafe fn`, raw-pointer implementation, or unsafe
 * dependency is required by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces a new modular grammar component without renaming:
 *
 *     Zamani.g4
 *     grammar.md
 *     Zamani-Grammar.md
 *     DESIGN.md
 *
 * Existing execution syntax remains governed by the compatibility system.
 *
 * The existing legacy runtime entry-point rule is migrated into this owner
 * according to:
 *
 *     grammar/compatibility/versions.md
 *     grammar/compatibility/migrations.md
 *     grammar/compatibility/deprecated.md
 *
 * No silent syntax removal is permitted.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. TEST CONTRACT
 * ============================================================================
 *
 * Production conformance tests MUST cover at minimum:
 *
 * --------------------------------------------------------------------------
 * Positive
 * --------------------------------------------------------------------------
 *
 *     entrypoint main;
 *
 *     entrypoint compute();
 *
 *     entrypoint compute(x: int);
 *
 *     entrypoint compute(x: int, y: int) -> int;
 *
 *     entrypoint quantum_kernel;
 *
 *     entrypoint module::kernel;
 *
 *     entrypoint domain::subsystem::kernel;
 *
 *     entrypoint compute as public_compute;
 *
 *     entrypoint compute
 *         requires qubits >= n;
 *
 *     entrypoint compute
 *         capability("quantum.measurement");
 *
 *     entrypoint compute
 *         resource(memory >= required_memory);
 *
 *     entrypoint compute
 *         portability(expression);
 *
 *     entrypoint compute
 *         target(capability_expression);
 *
 *     entrypoint compute
 *         deterministic = true;
 *
 * --------------------------------------------------------------------------
 * Negative
 * --------------------------------------------------------------------------
 *
 *     entrypoint;
 *
 *     entrypoint compute(;
 *
 *     entrypoint compute(x:);
 *
 *     entrypoint compute() ->;
 *
 *     entrypoint compute(x int);
 *
 * --------------------------------------------------------------------------
 * Scalability
 * --------------------------------------------------------------------------
 *
 * Test arbitrary source-level cardinalities without establishing a language
 * maximum:
 *
 *     many entry points;
 *     many parameters;
 *     many clauses;
 *     deeply qualified names;
 *     large expressions;
 *     large requirement sets;
 *     large capability sets.
 *
 * The tests MUST NOT convert a practical test limit into a language limit.
 *
 * --------------------------------------------------------------------------
 * Cross-domain
 * --------------------------------------------------------------------------
 *
 *     classical entry point;
 *     quantum entry point;
 *     hybrid entry point;
 *     HDL entry point;
 *     accelerator entry point;
 *     distributed entry point;
 *     AI entry point;
 *     data-processing entry point;
 *     networking entry point;
 *     future-domain entry point.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_*
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     node count
 *     device count
 *     memory capacity
 *     register width
 *     tensor rank
 *     network size
 *     physical address
 *     physical device identifier
 *     vendor-specific execution primitive.
 *
 * Generic expressions remain program semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It owns entry-point syntax.
 *
 * [x] It has one public entry-point declaration rule.
 *
 * [x] It does not own function declarations.
 *
 * [x] It does not own runtime implementation.
 *
 * [x] It does not own dispatch.
 *
 * [x] It does not own placement.
 *
 * [x] It does not own scheduling.
 *
 * [x] It does not own resource semantics.
 *
 * [x] It does not own capability semantics.
 *
 * [x] It does not own hardware discovery.
 *
 * [x] It does not own quantum operations.
 *
 * [x] It does not create a second quantum IR.
 *
 * [x] It preserves quantum::ir as the canonical quantum boundary.
 *
 * [x] It contains no fixed hardware limits.
 *
 * [x] It contains no semantic actions.
 *
 * [x] It contains no unsafe Rust.
 *
 * [x] It is compatible with Rust 1.97 / 1.97.1 and Rust 2021.
 *
 * [x] It has explicit AST integration.
 *
 * [x] It has explicit semantic integration.
 *
 * [x] It has explicit IR integration.
 *
 * [x] It has explicit runtime integration.
 *
 * [x] It has explicit compiler integration.
 *
 * [x] It has explicit test requirements.
 *
 * [x] It has explicit compatibility requirements.
 *
 * [x] It has explicit scalability requirements.
 *
 * [x] It has an explicit hard-coding audit.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "Which source-level computation is exposed as an execution entry
 *      boundary, and what portable execution intent accompanies it?"
 *
 * It does NOT answer:
 *
 *     "Which machine executes it?"
 *
 *     "Which CPU core executes it?"
 *
 *     "Which GPU executes it?"
 *
 *     "Which QPU executes it?"
 *
 *     "Which physical qubit executes it?"
 *
 *     "Which FPGA region executes it?"
 *
 *     "Which node executes it?"
 *
 *     "How is it routed?"
 *
 *     "How is it scheduled?"
 *
 *     "How is QEC performed?"
 *
 *     "How does ZQN operate?"
 *
 *     "How does HAL communicate with hardware?"
 *
 * Those decisions remain downstream.
 *
 * Therefore:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to the program's semantic requirements and resources actually
 * available to a realization, without artificial language-level hardware
 * ceilings.
 *
 * ============================================================================
 */