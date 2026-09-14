/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/gpu.g4
 *
 * Purpose:
 *     GPU-specific semantic grammar for portable hardware/software
 *     co-design.
 *
 * Architectural position:
 *
 *     Source
 *       ↓
 *     Lexer / Parser
 *       ↓
 *     Zamani AST
 *       ↓
 *     Semantic analysis
 *       ↓
 *     Capability / requirement checking
 *       ↓
 *     Canonical IR
 *       ↓
 *     Optimization
 *       ↓
 *     Scheduling
 *       ↓
 *     Hardware abstraction / target lowering
 *       ↓
 *     Runtime
 *
 * This grammar describes GPU intent and requirements.
 *
 * It does NOT describe:
 *   - a particular physical GPU;
 *   - a vendor;
 *   - a device identifier;
 *   - a fixed number of GPUs;
 *   - a fixed number of compute units;
 *   - a fixed number of streaming multiprocessors;
 *   - a fixed warp/wavefront width;
 *   - a fixed memory capacity;
 *   - a fixed clock rate;
 *   - a fixed topology;
 *   - a fixed PCIe address;
 *   - a fixed launch configuration;
 *   - a fixed deployment;
 *   - a runtime-discovered hardware fact.
 *
 * POCO-REAF:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A source program should express what computation requires rather than
 * encoding the characteristics of today's GPU hardware.
 *
 * Rust compatibility:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar introduces no Rust unsafe requirements.
 *
 * Ownership:
 *     This file owns GPU-specific SOURCE SYNTAX.
 *
 * Non-ownership:
 *     Hardware discovery, GPU inventory, physical placement, scheduling,
 *     memory allocation, kernel compilation, driver interaction, runtime
 *     dispatch, optimization, and device selection belong elsewhere.
 *
 * Canonical boundary:
 *
 *     grammar → AST → semantic model → canonical IR
 *
 * GPU grammar must never become a second IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * FILE CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *
 * - GPU computation intent.
 * - GPU execution-domain declarations.
 * - GPU capability requirements.
 * - GPU resource requirements.
 * - GPU portability constraints.
 * - GPU execution preferences.
 * - GPU execution hints.
 * - GPU memory-space intent.
 * - GPU parallelism intent.
 * - GPU synchronization intent.
 * - GPU kernel intent.
 * - GPU host/device interaction intent.
 * - GPU accelerator interoperability intent.
 *
 * DOES NOT OWN
 * -------------
 *
 * - Generic hardware declarations.
 * - Generic resource declarations.
 * - Generic target declarations.
 * - Generic placement.
 * - Generic scheduling.
 * - Generic execution dispatch.
 * - Device discovery.
 * - Device enumeration.
 * - Vendor-specific hardware.
 * - Physical addresses.
 * - Physical topology.
 * - Driver APIs.
 * - CUDA/HIP/OpenCL/Vulkan/Metal implementation syntax.
 * - Backend-specific binary formats.
 * - GPU optimization algorithms.
 * - Runtime behavior.
 *
 * Those concerns are represented by:
 *
 *     hardware/*.g4
 *     resources/*.g4
 *     compile/*.g4
 *     execution/*.g4
 *     interoperability/*.g4
 *     dialects/*.g4
 *
 * and interpreted downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Expected imported/shared parser rules:
 *
 *     hardware.g4
 *     resources.g4
 *     capabilities.g4
 *     targets.g4
 *     hardware-constraints.g4
 *     core/capabilities.g4
 *     core/requirements.g4
 *     core/constraints.g4
 *     core/hints.g4
 *     types/types.g4
 *     expressions/expressions.g4
 *
 * IMPORTANT:
 *
 * ANTLR grammar composition must be decided by the authoritative
 * Zamani.g4 composition layer. This file therefore intentionally defines
 * GPU rules without assuming that every repository grammar file is directly
 * imported into this file.
 *
 * The final composition layer is responsible for wiring these rules into
 * the parser.
 *
 * This prevents circular grammar dependencies such as:
 *
 *     gpu.g4 → hardware.g4 → gpu.g4
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GPU DOMAIN ROOT
 * ============================================================================
 *
 * A GPU construct may describe one of several semantic forms:
 *
 *     kernel
 *     execution
 *     capability requirement
 *     resource requirement
 *     memory intent
 *     parallelism intent
 *     synchronization
 *     interoperability
 *     constraint
 *     preference
 *     hint
 *
 * The grammar intentionally does not force a physical device.
 * ============================================================================
 */

gpuDeclaration
    : gpuKernelDeclaration
    | gpuExecutionDeclaration
    | gpuRequirementDeclaration
    | gpuCapabilityDeclaration
    | gpuMemoryDeclaration
    | gpuParallelismDeclaration
    | gpuSynchronizationDeclaration
    | gpuInteropDeclaration
    | gpuConstraintDeclaration
    | gpuPreferenceDeclaration
    | gpuHintDeclaration
    ;


/*
 * ============================================================================
 * GPU KERNELS
 * ============================================================================
 *
 * A kernel is a semantic unit of accelerator computation.
 *
 * A kernel does not imply:
 *
 *     CUDA kernel
 *     HIP kernel
 *     OpenCL kernel
 *     SPIR-V kernel
 *     Vulkan compute shader
 *     Metal kernel
 *
 * Those are backend representations.
 *
 * Zamani source describes the computation and its requirements.
 * ============================================================================
 */

gpuKernelDeclaration
    : 'gpu' 'kernel' identifier
      gpuKernelParameters?
      gpuKernelAttributes?
      block
    ;

gpuKernelParameters
    : '(' parameterList? ')'
    ;

gpuKernelAttributes
    : '[' gpuKernelAttribute* ']'
    ;

gpuKernelAttribute
    : gpuKernelParallelismAttribute
    | gpuKernelMemoryAttribute
    | gpuKernelSynchronizationAttribute
    | gpuKernelCapabilityAttribute
    | gpuKernelRequirementAttribute
    ;

gpuKernelParallelismAttribute
    : 'parallel' ':' gpuParallelismExpression
    ;

gpuKernelMemoryAttribute
    : 'memory' ':' gpuMemoryExpression
    ;

gpuKernelSynchronizationAttribute
    : 'synchronization' ':' gpuSynchronizationExpression
    ;

gpuKernelCapabilityAttribute
    : 'capability' ':' gpuCapabilityExpression
    ;

gpuKernelRequirementAttribute
    : 'requires' ':' gpuRequirementExpression
    ;


/*
 * ============================================================================
 * GPU EXECUTION
 * ============================================================================
 *
 * Execution describes intent, not a concrete launch configuration.
 *
 * INVALID conceptual model:
 *
 *     launch on GPU 0 with 80 SMs and 1024 threads
 *
 * VALID conceptual model:
 *
 *     execute using gpu
 *     execute with parallelism(data)
 *     execute requiring shared-memory capability
 *
 * The runtime/compiler chooses the actual mapping.
 * ============================================================================
 */

gpuExecutionDeclaration
    : 'execute' 'on' 'gpu'
      gpuExecutionBody
    ;

gpuExecutionBody
    : block
    | gpuExecutionSpecification
    ;

gpuExecutionSpecification
    : '{'
      gpuExecutionClause*
      '}'
    ;

gpuExecutionClause
    : gpuRequirementClause
    | gpuCapabilityClause
    | gpuResourceClause
    | gpuParallelismClause
    | gpuMemoryClause
    | gpuSynchronizationClause
    | gpuConstraintClause
    | gpuPreferenceClause
    | gpuHintClause
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements are semantic obligations.
 *
 * They are NOT:
 *
 *     device selectors
 *     vendor selectors
 *     fixed resource allocations
 *
 * ============================================================================
 */

gpuRequirementDeclaration
    : 'gpu' 'requires' gpuRequirementExpression
    ;

gpuRequirementClause
    : 'requires' gpuRequirementExpression
    ;

gpuRequirementExpression
    : gpuRequirementAtom
    | gpuRequirementExpression gpuRequirementOperator gpuRequirementExpression
    | '(' gpuRequirementExpression ')'
    ;

gpuRequirementOperator
    : 'and'
    | 'or'
    ;

gpuRequirementAtom
    : 'compute'
    | 'parallel'
    | 'general'
    | 'vector'
    | 'matrix'
    | 'tensor'
    | 'floating'
    | 'integer'
    | 'atomic'
    | 'shared-memory'
    | 'local-memory'
    | 'global-memory'
    | 'unified-memory'
    | 'cooperative-execution'
    | 'asynchronous-copy'
    | 'synchronization'
    | 'subgroup-operations'
    | 'dynamic-parallelism'
    | 'image-processing'
    | 'ray-processing'
    | 'machine-learning'
    | 'cryptographic-computation'
    | 'general-purpose-acceleration'
    | qualifiedName
    ;


/*
 * ============================================================================
 * CAPABILITIES
 * ============================================================================
 *
 * Capability describes what an execution environment must provide or may
 * provide.
 *
 * Capability is intentionally separate from requirement.
 * ============================================================================
 */

gpuCapabilityDeclaration
    : 'gpu' 'capability' gpuCapabilityExpression
    ;

gpuCapabilityClause
    : 'capability' gpuCapabilityExpression
    ;

gpuCapabilityExpression
    : gpuCapabilityAtom
    | gpuCapabilityExpression gpuCapabilityOperator gpuCapabilityExpression
    | '(' gpuCapabilityExpression ')'
    ;

gpuCapabilityOperator
    : 'and'
    | 'or'
    ;

gpuCapabilityAtom
    : 'compute'
    | 'parallel'
    | 'vector'
    | 'matrix'
    | 'tensor'
    | 'atomic'
    | 'shared-memory'
    | 'local-memory'
    | 'global-memory'
    | 'unified-memory'
    | 'subgroup'
    | 'cooperative'
    | 'asynchronous-execution'
    | 'asynchronous-memory'
    | 'fast-math'
    | 'precise-math'
    | 'double-precision'
    | 'extended-precision'
    | 'image'
    | 'ray-tracing'
    | 'machine-learning'
    | 'cryptography'
    | 'interoperability'
    | qualifiedName
    ;


/*
 * ============================================================================
 * MEMORY
 * ============================================================================
 *
 * Memory-space names describe SEMANTIC memory behavior.
 *
 * They do not prescribe:
 *
 *     exact byte capacity
 *     physical address
 *     bank count
 *     cache size
 *     cache topology
 *     vendor-specific memory hierarchy
 *
 * ============================================================================
 */

gpuMemoryDeclaration
    : 'gpu' 'memory' gpuMemoryExpression
    ;

gpuMemoryClause
    : 'memory' gpuMemoryExpression
    ;

gpuMemoryExpression
    : gpuMemorySpace
    | gpuMemorySpace '(' gpuMemoryAttributeList? ')'
    ;

gpuMemorySpace
    : 'private'
    | 'local'
    | 'shared'
    | 'global'
    | 'constant'
    | 'unified'
    | 'managed'
    | 'host'
    | 'device'
    | 'read-only'
    | 'write-only'
    | 'read-write'
    | qualifiedName
    ;

gpuMemoryAttributeList
    : gpuMemoryAttribute (',' gpuMemoryAttribute)*
    ;

gpuMemoryAttribute
    : 'coherent'
    | 'cached'
    | 'uncached'
    | 'persistent'
    | 'streaming'
    | 'atomic'
    | 'synchronized'
    | 'portable'
    | 'preferred'
    | 'required'
    ;


/*
 * ============================================================================
 * PARALLELISM
 * ============================================================================
 *
 * Parallelism expresses the structure of the computation rather than the
 * number of physical execution lanes.
 *
 * The actual number of workers is selected downstream from:
 *
 *     workload
 *     target capabilities
 *     runtime availability
 *     scheduler decisions
 *     resource constraints
 *
 * ============================================================================
 */

gpuParallelismDeclaration
    : 'gpu' 'parallelism' gpuParallelismExpression
    ;

gpuParallelismClause
    : 'parallelism' gpuParallelismExpression
    ;

gpuParallelismExpression
    : gpuParallelismKind
    | gpuParallelismKind '(' gpuParallelismAttributeList? ')'
    ;

gpuParallelismKind
    : 'data'
    | 'task'
    | 'vector'
    | 'matrix'
    | 'tensor'
    | 'pipeline'
    | 'hierarchical'
    | 'cooperative'
    | 'independent'
    | 'synchronized'
    | qualifiedName
    ;

gpuParallelismAttributeList
    : gpuParallelismAttribute (',' gpuParallelismAttribute)*
    ;

gpuParallelismAttribute
    : 'dynamic'
    | 'adaptive'
    | 'portable'
    | 'scalable'
    | 'balanced'
    | 'coalesced'
    | 'deterministic'
    | 'throughput'
    | 'latency'
    | 'energy'
    | 'reliability'
    | gpuParallelismParameter
    ;

gpuParallelismParameter
    : identifier '=' expression
    ;


/*
 * ============================================================================
 * SYNCHRONIZATION
 * ============================================================================
 */

gpuSynchronizationDeclaration
    : 'gpu' 'synchronization' gpuSynchronizationExpression
    ;

gpuSynchronizationClause
    : 'synchronization' gpuSynchronizationExpression
    ;

gpuSynchronizationExpression
    : gpuSynchronizationKind
    | gpuSynchronizationKind '(' gpuSynchronizationAttributeList? ')'
    ;

gpuSynchronizationKind
    : 'none'
    | 'local'
    | 'group'
    | 'subgroup'
    | 'device'
    | 'memory'
    | 'atomic'
    | 'barrier'
    | 'fence'
    | 'event'
    | 'signal'
    | 'wait'
    | qualifiedName
    ;

gpuSynchronizationAttributeList
    : gpuSynchronizationAttribute (',' gpuSynchronizationAttribute)*
    ;

gpuSynchronizationAttribute
    : 'ordered'
    | 'unordered'
    | 'relaxed'
    | 'acquire'
    | 'release'
    | 'acq-rel'
    | 'sequential'
    | 'scoped'
    | 'portable'
    ;


/*
 * ============================================================================
 * RESOURCES
 * ============================================================================
 *
 * Resource declarations are intentionally abstract.
 *
 * Valid:
 *
 *     requires compute capability
 *     prefer accelerator execution
 *     requires memory behavior
 *
 * Invalid as a portable grammar contract:
 *
 *     gpu count = 4
 *     gpu id = 0
 *     sm count = 80
 *     warp size = 32
 *     memory = 24GB
 *
 * Physical resource facts belong to runtime/hardware descriptions.
 * ============================================================================
 */

gpuResourceClause
    : 'resource' gpuResourceExpression
    ;

gpuResourceExpression
    : gpuResourceKind
    | gpuResourceKind '(' gpuResourceAttributeList? ')'
    ;

gpuResourceKind
    : 'compute'
    | 'memory'
    | 'bandwidth'
    | 'throughput'
    | 'latency'
    | 'energy'
    | 'reliability'
    | 'capacity'
    | 'parallelism'
    | 'availability'
    | 'interconnect'
    | qualifiedName
    ;

gpuResourceAttributeList
    : gpuResourceAttribute (',' gpuResourceAttribute)*
    ;

gpuResourceAttribute
    : 'required'
    | 'preferred'
    | 'minimum'
    | 'maximum'
    | 'target'
    | 'portable'
    | 'elastic'
    | 'scalable'
    | 'adaptive'
    | gpuResourceValueAttribute
    ;

gpuResourceValueAttribute
    : identifier '=' expression
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints constrain valid implementations without selecting a device.
 * ============================================================================
 */

gpuConstraintDeclaration
    : 'gpu' 'constraint' gpuConstraintExpression
    ;

gpuConstraintClause
    : 'constraint' gpuConstraintExpression
    ;

gpuConstraintExpression
    : gpuConstraintAtom
    | gpuConstraintExpression gpuConstraintOperator gpuConstraintExpression
    | '(' gpuConstraintExpression ')'
    ;

gpuConstraintOperator
    : 'and'
    | 'or'
    ;

gpuConstraintAtom
    : gpuConstraintName
      gpuConstraintComparator
      expression
    ;

gpuConstraintName
    : 'memory'
    | 'bandwidth'
    | 'latency'
    | 'throughput'
    | 'energy'
    | 'reliability'
    | 'precision'
    | 'parallelism'
    | 'availability'
    | 'portability'
    | qualifiedName
    ;

gpuConstraintComparator
    : '='
    | '!='
    | '<'
    | '<='
    | '>'
    | '>='
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory optimization guidance.
 *
 * A preference must never change the semantic meaning of the program.
 * ============================================================================
 */

gpuPreferenceDeclaration
    : 'gpu' 'prefer' gpuPreferenceExpression
    ;

gpuPreferenceClause
    : 'prefer' gpuPreferenceExpression
    ;

gpuPreferenceExpression
    : gpuPreferenceKind
    | gpuPreferenceKind '(' gpuPreferenceAttributeList? ')'
    ;

gpuPreferenceKind
    : 'throughput'
    | 'latency'
    | 'energy'
    | 'reliability'
    | 'portability'
    | 'determinism'
    | 'precision'
    | 'memory-locality'
    | 'parallelism'
    | 'asynchronous-execution'
    | qualifiedName
    ;

gpuPreferenceAttributeList
    : gpuPreferenceAttribute (',' gpuPreferenceAttribute)*
    ;

gpuPreferenceAttribute
    : identifier '=' expression
    ;


/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints are weaker than requirements and constraints.
 *
 * A backend may ignore a hint.
 *
 * A hint must never make a program semantically invalid merely because a
 * target cannot honor it.
 * ============================================================================
 */

gpuHintDeclaration
    : 'gpu' 'hint' gpuHintExpression
    ;

gpuHintClause
    : 'hint' gpuHintExpression
    ;

gpuHintExpression
    : gpuHintKind
    | gpuHintKind '(' gpuHintAttributeList? ')'
    ;

gpuHintKind
    : 'parallel'
    | 'coalesce'
    | 'vectorize'
    | 'tile'
    | 'fuse'
    | 'cache'
    | 'prefetch'
    | 'pipeline'
    | 'unroll'
    | 'asynchronous'
    | 'occupancy'
    | 'locality'
    | 'precision'
    | 'throughput'
    | 'latency'
    | qualifiedName
    ;

gpuHintAttributeList
    : gpuHintAttribute (',' gpuHintAttribute)*
    ;

gpuHintAttribute
    : identifier '=' expression
    ;


/*
 * ============================================================================
 * HOST / DEVICE INTEROPERABILITY
 * ============================================================================
 *
 * These constructs express semantic movement or invocation boundaries.
 *
 * Actual transfer mechanisms are selected by lowering/runtime.
 * ============================================================================
 */

gpuInteropDeclaration
    : 'gpu' 'interop' gpuInteropExpression
    ;

gpuInteropClause
    : 'interop' gpuInteropExpression
    ;

gpuInteropExpression
    : gpuInteropKind
    | gpuInteropKind '(' gpuInteropAttributeList? ')'
    ;

gpuInteropKind
    : 'host'
    | 'device'
    | 'host-to-device'
    | 'device-to-host'
    | 'shared'
    | 'mapped'
    | 'zero-copy'
    | 'asynchronous'
    | 'synchronous'
    | 'stream'
    | 'event'
    | qualifiedName
    ;

gpuInteropAttributeList
    : gpuInteropAttribute (',' gpuInteropAttribute)*
    ;

gpuInteropAttribute
    : identifier '=' expression
    ;


/*
 * ============================================================================
 * TARGET NEUTRALITY
 * ============================================================================
 *
 * This grammar deliberately does NOT provide source-level syntax for:
 *
 *     vendor = NVIDIA
 *     vendor = AMD
 *     device = RTX...
 *     device_id = ...
 *     architecture = ...
 *     sm_count = ...
 *     compute_units = ...
 *     warp_size = ...
 *     wavefront_size = ...
 *     vram = ...
 *     clock = ...
 *
 * Such facts may exist in target descriptions or runtime-discovered
 * capabilities.
 *
 * If a program genuinely requires a target property, it should express that
 * requirement through the generic capability/resource/constraint mechanisms.
 *
 * Vendor-specific extensions belong under:
 *
 *     grammar/dialects/
 *     grammar/interoperability/
 *
 * and must remain explicitly versioned and namespaced.
 * ============================================================================
 */


/*
 * ============================================================================
 * IDENTIFIERS / SHARED RULE CONTRACT
 * ============================================================================
 *
 * These rules are expected to be provided by the authoritative grammar
 * composition layer.
 *
 * They are referenced here rather than duplicated.
 *
 * Expected shared rules:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     block
 *     parameterList
 *
 * This avoids creating duplicate identifier/expression definitions and
 * therefore prevents parser divergence.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC LOWERING CONTRACT
 * ============================================================================
 *
 * Parser output:
 *
 *     GPU syntax nodes
 *
 * Semantic analysis converts them into domain-neutral concepts such as:
 *
 *     ExecutionIntent
 *     CapabilityRequirement
 *     ResourceRequirement
 *     ResourceConstraint
 *     ExecutionPreference
 *     ExecutionHint
 *     MemoryIntent
 *     ParallelismIntent
 *     SynchronizationIntent
 *     AcceleratorBoundary
 *
 * The compiler then lowers those concepts into the appropriate canonical IR.
 *
 * This file must NOT introduce:
 *
 *     GpuIR
 *     GpuGate
 *     GpuQubit
 *     GpuDevice
 *     GpuRuntimeState
 *     GpuScheduler
 *
 * as competing canonical semantic representations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * REPOSITORY INTEGRATION
 * ============================================================================
 *
 * grammar/hardware/gpu.g4
 *        │
 *        ├── grammar/hardware/hardware.g4
 *        │
 *        ├── grammar/hardware/resources.g4
 *        │
 *        ├── grammar/hardware/capabilities.g4
 *        │
 *        ├── grammar/hardware/targets.g4
 *        │
 *        ├── grammar/resources/requirements.g4
 *        │
 *        ├── grammar/resources/constraints.g4
 *        │
 *        ├── grammar/core/capabilities.g4
 *        │
 *        ├── grammar/core/requirements.g4
 *        │
 *        └── grammar/core/constraints.g4
 *
 * Downstream:
 *
 *        AST
 *         ↓
 *     Semantic Analyzer
 *         ↓
 *     Resource Model
 *         ↓
 *     Compiler
 *         ↓
 *     Optimization
 *         ↓
 *     Scheduling
 *         ↓
 *     Hardware HAL
 *         ↓
 *     Runtime
 *
 * The grammar itself does not call or depend on runtime APIs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * GPU and quantum computing may coexist in a hybrid program.
 *
 * This file does NOT define:
 *
 *     qubits
 *     quantum gates
 *     quantum circuits
 *     QEC
 *     ZQN
 *
 * Quantum semantics remain owned by:
 *
 *     grammar/quantum/
 *
 * and ultimately lower through the canonical quantum::ir boundary.
 *
 * GPU syntax may express generic accelerator execution around classical
 * portions of a hybrid program, but must not redefine quantum semantics.
 *
 * Example semantic relationship:
 *
 *     classical computation
 *          ↓
 *     GPU acceleration intent
 *          ↓
 *     quantum operation
 *
 * is represented through separate domains and integrated by the hybrid
 * semantic layer.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * GPU source constructs must not redefine:
 *
 *     clocks
 *     wires
 *     registers
 *     ports
 *     processes
 *     timing
 *     state machines
 *
 * Those belong to:
 *
 *     grammar/hdl/
 *
 * GPU hardware implementation details may eventually be emitted by an HDL
 * backend, but that is a compiler/lowering responsibility.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A GPU execution intent may ultimately be mapped onto:
 *
 *     one accelerator
 *     multiple accelerators
 *     multiple nodes
 *     heterogeneous nodes
 *     remote resources
 *
 * This grammar intentionally does not encode the number of devices.
 *
 * Distributed placement belongs to:
 *
 *     grammar/distributed/
 *     grammar/resources/
 *     grammar/execution/
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * GPU access may be subject to security/capability policies.
 *
 * This grammar does not define authentication, authorization, identities,
 * credentials, or trust.
 *
 * Those concerns belong to:
 *
 *     grammar/security/
 *
 * The semantic layer may combine GPU requirements with security requirements.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar is unbounded with respect to machine scale.
 *
 * It contains no language-level constants for:
 *
 *     GPU count
 *     compute-unit count
 *     execution-lane count
 *     warp count
 *     wavefront count
 *     register count
 *     memory capacity
 *     cache capacity
 *     bandwidth
 *     device count
 *     topology size
 *
 * Numeric expressions may exist where the language semantics genuinely
 * require numeric values. Such expressions are not interpreted as grammar
 * limits.
 *
 * Resource availability is evaluated downstream.
 *
 * Therefore:
 *
 *     tiny machine
 *          ↓
 *     same source semantics
 *          ↓
 *     larger machine
 *
 * is a valid execution-scaling model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * GPU declarations must not depend on:
 *
 *     hardware discovery
 *     current runtime state
 *     device enumeration
 *     network state
 *     compiler timing
 *
 * The same source must produce the same parse structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser errors.
 *
 * Examples:
 *
 *     gpu kernel
 *     gpu requires
 *     execute on
 *
 * are syntactically incomplete.
 *
 * Semantic errors are NOT encoded in this grammar.
 *
 * Examples:
 *
 *     requesting an unavailable capability
 *     requesting an incompatible memory model
 *     requesting mutually exclusive execution properties
 *
 * are semantic/resource-analysis concerns.
 *
 * This separation is required for good diagnostics and future compatibility.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * FORBIDDEN GRAMMAR PATTERNS
 * ============================================================================
 *
 * The following must never be introduced into this file:
 *
 *     MAX_GPU = ...
 *     MAX_GPUS = ...
 *     GPU_COUNT = ...
 *     MAX_COMPUTE_UNITS = ...
 *     MAX_THREADS = ...
 *     MAX_BLOCKS = ...
 *     MAX_WARPS = ...
 *     MAX_MEMORY = ...
 *
 * Nor should equivalent restrictions be hidden in grammar cardinality.
 *
 * For example, do NOT replace:
 *
 *     gpuRequirementAtom*
 *
 * with a finite machine-derived count.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * VENDOR EXTENSION CONTRACT
 * ============================================================================
 *
 * Vendor-specific GPU capabilities may be represented through qualified
 * names:
 *
 *     vendor.namespace.capability
 *
 * Such extensions must:
 *
 *     1. be explicitly registered;
 *     2. be versioned;
 *     3. remain namespaced;
 *     4. not alter core Zamani semantics;
 *     5. not make portable programs depend on one vendor;
 *     6. be validated by the dialect system.
 *
 * This permits future hardware innovation without changing this file for
 * every new GPU architecture.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPILATION CONTRACT
 * ============================================================================
 *
 * Compilation may lower:
 *
 *     gpu kernel
 *         →
 *     accelerator computation
 *
 *     gpu parallelism
 *         →
 *     parallel execution metadata
 *
 *     gpu memory
 *         →
 *     memory-space intent
 *
 *     gpu requires
 *         →
 *     capability requirement
 *
 *     gpu resource
 *         →
 *     resource requirement
 *
 *     gpu constraint
 *         →
 *     implementation constraint
 *
 *     gpu prefer
 *         →
 *     optimization preference
 *
 *     gpu hint
 *         →
 *     optimization hint
 *
 * None of these transformations may require modifying the source program
 * merely because a target has a different GPU size or architecture.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime is responsible for determining:
 *
 *     available GPU resources
 *     available capabilities
 *     current device health
 *     current resource availability
 *     queue state
 *     execution placement
 *     dispatch configuration
 *     memory allocation
 *     backend implementation
 *
 * Runtime must consume semantic information produced downstream from this
 * grammar.
 *
 * This grammar must never query runtime state.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     gpu kernel compute() {}
 *     gpu requires compute
 *     gpu requires parallel and tensor
 *     gpu capability shared-memory
 *     gpu memory shared
 *     gpu parallelism data
 *     gpu synchronization group
 *     execute on gpu {}
 *
 * Required scalable tests:
 *
 *     no GPU count
 *     no fixed compute-unit count
 *     no fixed thread count
 *     no fixed memory size
 *     no device identifier
 *
 * Required negative tests:
 *
 *     gpu requires
 *     gpu capability
 *     gpu memory
 *     gpu parallelism
 *     execute on
 *
 * Required semantic-boundary tests:
 *
 *     GPU requirements must not become device selection.
 *     GPU capabilities must not become physical inventory.
 *     GPU memory semantics must not become fixed capacities.
 *     GPU preferences must not alter program semantics.
 *     GPU hints must remain optional.
 *
 * Required cross-domain tests:
 *
 *     classical + gpu
 *     quantum + gpu
 *     hybrid + gpu
 *     hdl + gpu
 *     distributed + gpu
 *     ai + gpu
 *     data + gpu
 *     gpu + security
 *
 * Required determinism tests:
 *
 *     identical source → identical parse tree
 *
 * Required round-trip tests:
 *
 *     source
 *       →
 *     parser
 *       →
 *     AST
 *       →
 *     printer
 *       →
 *     parser
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] GPU syntax has one clearly defined owner.
 * [ ] GPU syntax composes through authoritative Zamani.g4.
 * [ ] Shared identifiers are not duplicated.
 * [ ] Shared expressions are not duplicated.
 * [ ] GPU syntax creates no competing IR.
 * [ ] GPU requirements are distinct from capabilities.
 * [ ] GPU capabilities are distinct from resources.
 * [ ] GPU resources are distinct from constraints.
 * [ ] Constraints are distinct from preferences.
 * [ ] Preferences are distinct from hints.
 * [ ] Memory spaces are semantic, not physical inventories.
 * [ ] Parallelism is semantic, not a fixed hardware configuration.
 * [ ] Synchronization is semantic, not vendor-specific.
 * [ ] Device selection is downstream.
 * [ ] Hardware discovery is downstream.
 * [ ] Scheduling is downstream.
 * [ ] Optimization is downstream.
 * [ ] Runtime dispatch is downstream.
 * [ ] Vendor extensions are namespaced.
 * [ ] Quantum semantics remain outside this file.
 * [ ] QEC remains outside this file.
 * [ ] ZQN remains outside this file.
 * [ ] HDL semantics remain outside this file.
 * [ ] No fixed GPU count exists.
 * [ ] No fixed compute-unit count exists.
 * [ ] No fixed thread count exists.
 * [ ] No fixed memory capacity exists.
 * [ ] No fixed topology exists.
 * [ ] No fixed device ID exists.
 * [ ] No machine-specific assumption is hidden in repetition limits.
 * [ ] Rust integration requires no unsafe code.
 * [ ] Positive parser tests exist.
 * [ ] Negative parser tests exist.
 * [ ] Boundary tests exist.
 * [ ] Scalability tests exist.
 * [ ] Cross-domain tests exist.
 * [ ] Determinism tests exist.
 * [ ] Round-trip tests exist.
 *
 * ============================================================================
 */