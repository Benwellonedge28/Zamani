/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/resource.g4
 *
 * GRAMMAR IDENTITY
 * ----------------
 * ResourceIntent
 *
 * STATUS
 * ------
 * CANONICAL LEAF / COMPONENT RESOURCE GRAMMAR
 *
 * PURPOSE
 * -------
 * This file defines the singular, reusable resource-intent component used by
 * grammar/resources/resources.g4.
 *
 * IMPORTANT:
 *
 *     resource.g4 is NOT the resource orchestrator.
 *
 * The orchestration boundary remains:
 *
 *     grammar/resources/resources.g4
 *
 * This file MUST therefore be imported by Resources and MUST NOT introduce
 * another competing resource grammar hierarchy.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                         Zamani parser
 *                              |
 *                              v
 *                  grammar/resources/resources.g4
 *                              |
 *                              v
 *                  grammar/resources/resource.g4
 *                              |
 *                              v
 *                       ResourceIntent
 *                              |
 *                              v
 *                         Frontend AST
 *                              |
 *                              v
 *                    Semantic Resource Model
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      requirements        capabilities        constraints
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    Canonical Semantic Model
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical           quantum::ir       HDL/hardware
 *          |                                       |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                 +------------+------------+
 *                 |            |            |
 *                 v            v            v
 *              routing     scheduling   resilience
 *                 |            |            |
 *                 +------------+------------+
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS ONLY:
 *
 *     resourceIntent
 *     resourceIntentName
 *     resourceIntentKind
 *     resourceIntentQualifier
 *     resourceIntentArguments
 *     resourceIntentArgument
 *     resourceIntentProperty
 *     resourceIntentPropertyValue
 *
 * These rules provide the reusable singular resource-intent representation
 * needed by the concrete Resources grammar.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     resources
 *     resourceItem
 *     resourceDeclaration
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceTarget
 *     resourceGroup
 *     resourceProfile
 *     resourceContract
 *     resourceExpression
 *     expression
 *     identifier
 *     qualifiedName
 *     type syntax
 *     hardware discovery
 *     resource allocation
 *     physical placement
 *     routing
 *     scheduling
 *     optimization
 *     QEC
 *     ZQN
 *     HAL
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *
 * Those responsibilities remain with their existing canonical owners.
 *
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * The parent grammar:
 *
 *     grammar/resources/resources.g4
 *
 * imports this grammar.
 *
 * Conceptually:
 *
 *     parser grammar Resources;
 *
 *     import
 *         ResourceIntent,
 *         ResourceExpressions,
 *         Names;
 *
 * The parent grammar remains responsible for deciding where resourceIntent
 * appears in the complete resource language.
 *
 * This component deliberately does NOT contain:
 *
 *     program
 *     sourceUnit
 *     resources
 *     resourceItem
 *
 * and therefore cannot become an accidental orchestrator.
 *
 * ============================================================================
 * EXPRESSION AND NAME OWNERSHIP
 * ============================================================================
 *
 * Names remain owned by:
 *
 *     grammar/core/names.g4
 *
 * Expressions remain owned by:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * This file merely consumes those canonical rules.
 *
 * No private:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     arithmeticExpression
 *
 * grammar is introduced here.
 *
 * ============================================================================
 * OPEN-WORLD RESOURCE MODEL
 * ============================================================================
 *
 * Resource kinds are intentionally OPEN-WORLD.
 *
 * The grammar does not enumerate:
 *
 *     cpu
 *     gpu
 *     fpga
 *     asic
 *     qpu
 *     accelerator
 *     memory
 *     storage
 *     network
 *     node
 *     device
 *
 * or any future resource category.
 *
 * A resource kind is represented by a canonical name.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     accelerator
 *     quantum::logical_qubit
 *     quantum::physical_qubit
 *     tensor::compute
 *     future::resource
 *
 * This permits new resource classes without modifying this grammar merely
 * because a new hardware technology appears.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Resource intent participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The source program describes semantic resource intent.
 *
 * It does NOT select a physical realization by default.
 *
 * Therefore:
 *
 *     resource compute;
 *
 * does not mean:
 *
 *     CPU 0
 *
 *     resource quantum::logical_qubit;
 *
 * does not mean:
 *
 *     physical qubit 0
 *
 *     resource accelerator::tensor;
 *
 * does not mean:
 *
 *     GPU device 0
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * NO ARTIFICIAL RESOURCE LIMITS
 * ============================================================================
 *
 * This grammar introduces NO language-level limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     accelerators
 *     nodes
 *     devices
 *     memory
 *     storage
 *     registers
 *     vector widths
 *     tensor dimensions
 *     tensor rank
 *     network links
 *     timelines
 *     processes
 *     tasks
 *
 * It MUST NOT introduce:
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
 * Numeric values appearing in a resource intent are program values.
 *
 * For example:
 *
 *     resource memory {
 *         capacity = required_memory;
 *     }
 *
 * expresses a semantic relationship.
 *
 * The grammar never converts a numeric value into a universal hardware limit.
 *
 * ============================================================================
 * RESOURCE IDENTITY
 * ============================================================================
 *
 * A resource intent name identifies a logical resource concept.
 *
 * It MUST NOT automatically imply:
 *
 *     physical device identity
 *     physical address
 *     physical core number
 *     physical qubit number
 *     cloud instance identifier
 *     PCI identifier
 *     vendor device identifier
 *
 * Those concepts belong to target-specific realization layers.
 *
 * ============================================================================
 * RESOURCE KIND
 * ============================================================================
 *
 * A resource kind is a qualified semantic name.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     compute::parallel
 *     quantum::logical_qubit
 *     quantum::measurement
 *     accelerator::tensor
 *     network::bandwidth
 *
 * The grammar does not decide whether a named kind is available.
 *
 * Semantic analysis resolves its meaning.
 *
 * ============================================================================
 * QUALIFIERS
 * ============================================================================
 *
 * Qualifiers are optional semantic metadata attached to a resource intent.
 *
 * They do not perform allocation.
 *
 * Examples of conceptual qualifiers:
 *
 *     resource compute: parallel;
 *
 *     resource qubits: quantum::logical_qubit;
 *
 *     resource memory: memory;
 *
 * The exact semantic interpretation belongs downstream.
 *
 * ============================================================================
 * ARGUMENTS
 * ============================================================================
 *
 * Resource arguments are expressions.
 *
 * Therefore they may depend on:
 *
 *     constants
 *     variables
 *     generic parameters
 *     input sizes
 *     type information
 *     symbolic values
 *     runtime values where supported
 *
 * Examples:
 *
 *     resource tensor: tensor::compute(workload.size);
 *
 *     resource memory: memory(required_memory);
 *
 *     resource qubits: quantum::logical_qubit(problem.size);
 *
 * No machine-size constant is introduced.
 *
 * ============================================================================
 * PROPERTIES
 * ============================================================================
 *
 * Resource properties are intentionally generic.
 *
 * This component does not create a closed list such as:
 *
 *     cpu_count
 *     gpu_count
 *     qubit_count
 *     memory_gb
 *
 * Instead, property names remain semantic names and values remain canonical
 * expressions.
 *
 * This allows future resource properties without changing the grammar.
 *
 * Examples:
 *
 *     capacity = required_memory
 *     quantity = workload_size
 *     precision = required_precision
 *     topology = required_topology
 *     latency = latency_budget
 *     bandwidth = required_bandwidth
 *
 * The semantic layer determines whether a property is meaningful for the
 * selected resource kind.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST map to the domain-neutral frontend AST.
 *
 * The AST representation should preserve at least:
 *
 *     resource name
 *     resource kind
 *     qualifiers
 *     arguments
 *     properties
 *     source spans
 *
 * The AST MUST NOT contain:
 *
 *     physical allocation
 *     backend-specific device handles
 *     scheduler state
 *     routing state
 *     calibration state
 *     QEC state
 *     ZQN state
 *     HAL handles
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     resolving resource names
 *     resolving resource kinds
 *     validating property applicability
 *     validating argument types
 *     validating expressions
 *     checking resource relationships
 *     checking capability relationships
 *     checking requirements
 *     checking constraints
 *     preserving preferences as preferences
 *     preserving hints as hints
 *
 * This grammar does none of those computations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * ResourceIntent MUST lower into the repository's canonical semantic resource
 * representation.
 *
 * It MUST NOT create:
 *
 *     ResourceIR
 *     ResourceIntentIR
 *     QuantumResourceIR
 *
 * merely because the syntax occurs in this file.
 *
 * For quantum programs, resource information required by quantum semantics
 * ultimately participates in the existing canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * The resource grammar MUST NOT create a second quantum IR.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical resource intent may describe:
 *
 *     compute
 *     memory
 *     storage
 *     parallelism
 *     vectorization
 *     numerical capability
 *     accelerator capability
 *     latency
 *     throughput
 *
 * Actual CPU/core/register/cache realization is downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum resource intent may describe semantic resources such as:
 *
 *     quantum::logical_qubit
 *     quantum::physical_qubit
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::error_correction
 *     quantum::coherence
 *
 * This file does not determine:
 *
 *     physical qubit numbering
 *     coupling topology
 *     gate decomposition
 *     pulse scheduling
 *     calibration
 *     physical routing
 *
 * The downstream path remains:
 *
 *     resource intent
 *          |
 *          v
 *     semantic analysis
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
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     physical realization
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware resources remain abstract at this layer.
 *
 * Resource intent may identify:
 *
 *     compute
 *     memory
 *     interconnect
 *     accelerator
 *     reconfigurable hardware
 *
 * HDL remains responsible for describing hardware structure.
 *
 * Resource intent does not replace:
 *
 *     ports
 *     signals
 *     nets
 *     registers
 *     timing
 *     pipelines
 *     state machines
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Resource intent may describe:
 *
 *     distributed execution
 *     communication
 *     bandwidth
 *     locality
 *     replication
 *     availability
 *     storage
 *
 * It does not impose a fixed node count.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Resource intent may depend on:
 *
 *     tensor shape
 *     model size
 *     dataset size
 *     batch size
 *     workload size
 *     accelerator capability
 *
 * Values remain expressions rather than fixed implementation limits.
 *
 * ============================================================================
 * RESOURCE LIFECYCLE
 * ============================================================================
 *
 * This component does not implement:
 *
 *     acquisition
 *     reservation
 *     allocation
 *     release
 *     migration
 *     recovery
 *
 * Those constructs are owned by the appropriate higher-level resource
 * grammar components and semantic/runtime systems.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing of ResourceIntent depends only on:
 *
 *     source text
 *     lexer
 *     grammar version
 *     parser composition
 *     explicitly selected dialect
 *
 * It MUST NOT depend on:
 *
 *     hardware discovery
 *     runtime state
 *     wall-clock time
 *     random values
 *     filesystem state
 *     network state
 *     environment state
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This is an ANTLR grammar component.
 *
 * It contains:
 *
 *     no embedded Rust
 *     no parser actions
 *     no semantic predicates
 *     no unsafe code
 *     no filesystem access
 *     no network access
 *     no hardware access
 *
 * The Rust frontend consuming the generated parser remains compatible with:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and requires Safe Rust only.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing resource syntax owned by Resources remains authoritative.
 *
 * This component must be additive and compositional.
 *
 * It must not redefine an existing rule with the same semantic ownership.
 *
 * In particular, it does not redefine:
 *
 *     Resource
 *     resource
 *     resources
 *     resourceItem
 *     resourceDeclaration
 *
 * This prevents collision with:
 *
 *     grammar/types/resource.g4
 *
 * and the concrete:
 *
 *     grammar/resources/resources.g4
 *
 * grammar.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Syntax errors are handled by the canonical ANTLR parser error strategy.
 *
 * This component must not embed custom error actions.
 *
 * Semantic errors such as:
 *
 *     unknown resource kind
 *     invalid property
 *     invalid argument type
 *     unsatisfied capability
 *
 * belong downstream to semantic diagnostics.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar intentionally uses repetition and recursive name paths.
 *
 * There is no fixed maximum for:
 *
 *     resource arguments
 *     resource properties
 *     qualified-name depth
 *     nested resource expressions
 *
 * Physical resource availability is not a grammar concern.
 *
 * "Infinity" means unbounded by this language component, subject only to the
 * finite resources of the implementation executing the compiler/parser.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the resource hard-coding policy when:
 *
 *     no machine-size constant is introduced;
 *     no physical device number is required;
 *     no vendor is enumerated;
 *     no architecture is mandatory;
 *     no resource count is bounded;
 *     no memory capacity is bounded;
 *     no register width is bounded;
 *     no topology size is bounded;
 *     no device count is bounded.
 *
 * Program values remain legal:
 *
 *     resource memory: memory(required_memory);
 *     resource qubits: quantum::logical_qubit(problem_size);
 *
 * because they are semantic program data.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is a leaf/component grammar.
 * [x] It is not an orchestrator.
 * [x] It does not create a competing Resources grammar.
 * [x] It does not create a competing Resource type grammar.
 * [x] It does not redefine resourceItem.
 * [x] It does not redefine resourceDeclaration.
 * [x] It does not define resource-expression syntax.
 * [x] It consumes canonical names.
 * [x] It consumes canonical expressions.
 * [x] Resource kinds are open-world.
 * [x] Resource properties are open-world.
 * [x] Resource arguments are expressions.
 * [x] No hardware capacity is hard-coded.
 * [x] No device identity is hard-coded.
 * [x] No vendor is hard-coded.
 * [x] No quantum gate is hard-coded.
 * [x] No physical qubit is hard-coded.
 * [x] No CPU count is hard-coded.
 * [x] No GPU count is hard-coded.
 * [x] No FPGA count is hard-coded.
 * [x] No node count is hard-coded.
 * [x] No memory size is hard-coded.
 * [x] No register width is hard-coded.
 * [x] No tensor rank is hard-coded.
 * [x] No network size is hard-coded.
 * [x] No device count is hard-coded.
 * [x] AST integration is defined.
 * [x] Semantic integration is defined.
 * [x] Canonical IR integration is defined.
 * [x] quantum::ir remains canonical.
 * [x] HDL/hardware integration is defined.
 * [x] Classical integration is defined.
 * [x] Distributed integration is defined.
 * [x] AI/data integration is defined.
 * [x] Safe Rust requirement is defined.
 * [x] No unsafe Rust is required.
 * [x] Deterministic parsing is preserved.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 *
 * This grammar intentionally contains only the singular resource-intent
 * component.
 *
 * The parent Resources grammar decides where resourceIntent is legal.
 *
 * ============================================================================
 */

parser grammar ResourceIntent;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * RESOURCE INTENT
 * ============================================================================
 *
 * A resource intent is a symbolic resource description.
 *
 * It consists of:
 *
 *     a resource name
 *     an optional semantic kind
 *     optional arguments
 *     optional properties
 *
 * Examples at the conceptual level:
 *
 *     compute
 *
 *     memory: memory
 *
 *     qubits: quantum::logical_qubit
 *
 *     accelerator: tensor::compute(workload_size)
 *
 *     memory: memory {
 *         capacity = required_memory;
 *     }
 *
 * This rule does not decide whether the resource is required, preferred,
 * available, reserved, acquired, or allocated. Those meanings belong to
 * their respective higher-level resource constructs.
 *
 * ============================================================================
 */

resourceIntent
    : resourceIntentName
      resourceIntentKind?
      resourceIntentArguments?
      resourceIntentPropertyBlock?
    ;


/*
 * ============================================================================
 * RESOURCE NAME
 * ============================================================================
 *
 * The name identifies the logical resource intent.
 *
 * Names are resolved by semantic analysis.
 *
 * ============================================================================
 */

resourceIntentName
    : identifier
    ;


/*
 * ============================================================================
 * RESOURCE KIND
 * ============================================================================
 *
 * Resource kinds are qualified names.
 *
 * The grammar intentionally does not enumerate resource kinds.
 *
 * ============================================================================
 */

resourceIntentKind
    : COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * RESOURCE ARGUMENTS
 * ============================================================================
 *
 * Arguments are canonical resource expressions.
 *
 * This permits values derived from program semantics rather than fixed
 * hardware assumptions.
 *
 * ============================================================================
 */

resourceIntentArguments
    : LPAREN
      resourceIntentArgumentList?
      RPAREN
    ;


resourceIntentArgumentList
    : resourceIntentArgument
      (COMMA resourceIntentArgument)*
    ;


resourceIntentArgument
    : resourceExpression
    ;


/*
 * ============================================================================
 * RESOURCE PROPERTY BLOCK
 * ============================================================================
 *
 * Properties are generic semantic metadata.
 *
 * This rule deliberately does not enumerate properties such as:
 *
 *     memory
 *     latency
 *     bandwidth
 *     quantity
 *     capacity
 *
 * because resource kinds are open-world.
 *
 * ============================================================================
 */

resourceIntentPropertyBlock
    : LBRACE
      resourceIntentProperty*
      RBRACE
    ;


resourceIntentProperty
    : resourceIntentPropertyName
      ASSIGN
      resourceIntentPropertyValue
      SEMICOLON
    ;


resourceIntentPropertyName
    : identifier
    ;


resourceIntentPropertyValue
    : resourceExpression
    ;