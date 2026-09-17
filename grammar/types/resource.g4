/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/resource.g4
 *
 * Status:
 *     Production-ready modular resource-type qualifier grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust code.
 *     Zamani compiler/runtime integration MUST remain safe Rust only.
 *     No `unsafe` Rust is required or permitted by the architecture.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns exactly one language-level concept:
 *
 *     resource
 *
 * as a TYPE QUALIFIER.
 *
 * It does NOT own the complete type expression following the qualifier.
 *
 * Therefore:
 *
 *     resource T
 *
 * is composed as:
 *
 *     resourceTypeQualifier
 *         +
 *     canonical typeExpression
 *
 * The canonical type composition remains owned by:
 *
 *     grammar/types/types.g4
 *
 * This prevents:
 *
 *     types.g4 -> resource.g4 -> types.g4
 *
 * circular parser dependencies.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     typeExpression
 *       |
 *       +--> resourceTypeQualifier
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type/resource analysis
 *       |
 *       +--> resource requirements
 *       +--> capabilities
 *       +--> ownership
 *       +--> constraints
 *       +--> preferences
 *       +--> portability
 *       |
 *       v
 *     canonical semantic model / IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed/data/AI representations
 *       |
 *       v
 *     optimization
 *       |
 *       +--> routing
 *       +--> scheduling
 *       +--> resilience
 *       +--> QEC
 *       +--> ZQN
 *       |
 *       v
 *     HAL / target realization / runtime
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the lexical-to-parser integration of the `resource` type qualifier;
 *     - the canonical parser rule `resourceTypeQualifier`;
 *     - source-level resource qualification syntax;
 *     - resource-qualifier source spans;
 *     - the type-system integration contract for resource qualification.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - complete type expressions;
 *     - identifiers;
 *     - qualified names;
 *     - generic arguments;
 *     - arrays;
 *     - tuples;
 *     - functions;
 *     - references;
 *     - pointers;
 *     - primitive types;
 *     - quantum types;
 *     - classical types;
 *     - HDL types;
 *     - hardware types;
 *     - capability semantics;
 *     - resource allocation;
 *     - resource discovery;
 *     - resource accounting;
 *     - hardware discovery;
 *     - device selection;
 *     - placement;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime allocation;
 *     - ABI layout.
 *
 * ============================================================================
 *
 * CORE SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * `resource` describes SOURCE-LEVEL TYPE INTENT.
 *
 * It does not mean:
 *
 *     allocate now
 *     reserve hardware now
 *     select a device
 *     select a physical qubit
 *     select a CPU
 *     select a GPU
 *     select an FPGA
 *     select a memory bank
 *     select a network node
 *     select a cloud region
 *     select a vendor backend
 *
 * Those decisions belong downstream.
 *
 * The grammar therefore preserves the separation:
 *
 *     semantic requirement
 *         !=
 *     capability
 *         !=
 *     preference
 *         !=
 *     hint
 *         !=
 *     implementation decision
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * This file MUST support:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * It MUST NOT encode implementation limits such as:
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
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_REGISTER_COUNT
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_RESOURCE_COUNT
 *     MAX_GENERIC_ARITY
 *     MAX_NESTING
 *
 * A program may express an actual program-level quantity such as:
 *
 *     N
 *     1024
 *     required_memory
 *     qubit_count
 *
 * without turning that value into a compiler-wide hardware limit.
 *
 * For example, a semantic resource requirement may eventually mean:
 *
 *     requires quantum resources sufficient for N logical qubits
 *
 * without the grammar defining:
 *
 *     N <= 1024
 *
 * or:
 *
 *     MAX_QUBITS = 1024
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexical authority:
 *
 *     grammar/lexer/
 *
 * Canonical composed lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume the assembled vocabulary through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * IMPORTANT:
 *
 * The current canonical keyword vocabulary does not yet expose:
 *
 *     RESOURCE
 *
 * Therefore the resource qualifier requires one corresponding addition to:
 *
 *     grammar/lexer/keywords.g4
 *
 * The required lexical contract is:
 *
 *     RESOURCE : 'resource' ;
 *
 * That token MUST be owned by the keyword grammar.
 *
 * It MUST NOT be defined here.
 *
 * No local lexer rule is permitted in this parser grammar.
 *
 * ============================================================================
 *
 * WHY `resource.g4` DOES NOT PARSE THE INNER TYPE
 * ============================================================================
 *
 * An incorrect implementation would be:
 *
 *     resourceType
 *         : RESOURCE typeExpression
 *         ;
 *
 * That is intentionally NOT used here.
 *
 * `typeExpression` is owned by `types.g4`.
 *
 * Making this file parse the inner type would make the dependency direction:
 *
 *     types.g4
 *         |
 *         v
 *     resource.g4
 *         |
 *         v
 *     types.g4
 *
 * which creates circular grammar composition.
 *
 * The correct architecture is:
 *
 *     types.g4
 *         |
 *         +--> resourceTypeQualifier
 *                    |
 *                    +--> RESOURCE
 *
 * followed by the already-existing:
 *
 *     typePrimary
 *     typePostfix
 *
 * composition.
 *
 * ============================================================================
 *
 * PUBLIC PARSER CONTRACT
 * ============================================================================
 *
 * The ONLY public resource-specific parser rule intended for composition into
 * the canonical type grammar is:
 *
 *     resourceTypeQualifier
 *
 * It recognizes:
 *
 *     resource
 *
 * and nothing more.
 *
 * The enclosing `types.g4` is responsible for composing:
 *
 *     resourceTypeQualifier
 *         +
 *     typePrimary
 *         +
 *     typePostfix*
 *
 * ============================================================================
 *
 * CANONICAL SOURCE FORM
 * ============================================================================
 *
 * After integration with `types.g4`, examples include:
 *
 *     resource T
 *
 *     resource Qubit
 *
 *     resource Memory
 *
 *     resource Accelerator
 *
 *     resource QuantumResource<N>
 *
 *     resource Tensor<T, Shape>
 *
 *     resource Foo::Bar
 *
 *     affine resource Qubit
 *
 *     linear resource Buffer
 *
 * where the latter compositions are legal only if the canonical type
 * qualifier/semantic rules permit them.
 *
 * This file does not decide that legality.
 *
 * ============================================================================
 *
 * OPEN-WORLD RESOURCE MODEL
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     TPU
 *     NPU
 *     DSP
 *     memory
 *     storage
 *     network
 *     accelerator
 *     quantum
 *     optical
 *     molecular
 *     atomic
 *     biological
 *     future_resource
 *
 * as parser alternatives.
 *
 * The resource qualifier is intentionally open-ended.
 *
 * Resource kinds belong to the type/semantic system.
 *
 * This allows future computational substrates to participate without
 * requiring this grammar to be rewritten whenever a new technology appears.
 *
 * ============================================================================
 *
 * RESOURCE TYPE VS RESOURCE REQUIREMENT
 * ============================================================================
 *
 * These concepts must remain distinct.
 *
 * TYPE:
 *
 *     resource T
 *
 * describes a resource-bearing/qualified source type.
 *
 * REQUIREMENT:
 *
 *     requires ...
 *
 * describes an execution/compilation requirement.
 *
 * CAPABILITY:
 *
 *     capability(...)
 *
 * describes an available property.
 *
 * PREFERENCE:
 *
 *     prefer ...
 *
 * describes a non-binding preference.
 *
 * HINT:
 *
 *     hint ...
 *
 * describes optional optimization information.
 *
 * IMPLEMENTATION DECISION:
 *
 *     physical_device(...)
 *
 * belongs downstream.
 *
 * This file must never collapse these concepts into one grammar construct.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Resource qualification may apply to quantum source types.
 *
 * Examples:
 *
 *     resource Qubit
 *
 *     resource LogicalQubit
 *
 *     resource QRegister<N>
 *
 *     resource QuantumState<T>
 *
 * The grammar does NOT:
 *
 *     allocate qubits;
 *     assign physical qubit IDs;
 *     select a QPU;
 *     inspect coupling maps;
 *     perform routing;
 *     schedule operations;
 *     perform calibration;
 *     perform QEC;
 *     interpret noise;
 *     invoke ZQN;
 *     query HAL.
 *
 * Quantum semantic lowering continues through:
 *
 *     quantum::ir
 *
 * There must be no:
 *
 *     ResourceQuantumIR
 *
 * or other second quantum IR introduced by this grammar.
 *
 * ============================================================================
 *
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Resource-qualified classical types may describe source-level ownership or
 * resource-bearing values without selecting a concrete processor.
 *
 * Examples:
 *
 *     resource Buffer
 *
 *     resource Tensor<T, Shape>
 *
 *     resource Memory<T>
 *
 *     resource Vector<T, N>
 *
 * The grammar does not determine:
 *
 *     SIMD width
 *     CPU count
 *     cache level
 *     register allocation
 *     NUMA node
 *     physical address
 *
 * Those belong downstream.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Resource-qualified hardware types may describe semantic hardware intent.
 *
 * Examples:
 *
 *     resource Port
 *
 *     resource Signal
 *
 *     resource MemoryBank
 *
 *     resource Accelerator
 *
 *     resource Interconnect
 *
 * The grammar does not determine:
 *
 *     FPGA family
 *     ASIC family
 *     register width
 *     physical pin
 *     physical port number
 *     clock implementation
 *     device address
 *     placement
 *     routing
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 *
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Resource qualification can participate in distributed source types without
 * encoding a fixed number of nodes.
 *
 * Examples:
 *
 *     resource Node
 *
 *     resource Channel
 *
 *     resource Service
 *
 *     resource DistributedState
 *
 * The grammar does not encode:
 *
 *     node 0
 *     node 1
 *     node count
 *     cluster size
 *     topology
 *
 * ============================================================================
 *
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Resource qualification remains domain-neutral and can therefore compose with:
 *
 *     Model
 *     Tensor<T, Shape>
 *     Dataset
 *     Accelerator
 *     Stream
 *     Pipeline
 *
 * without making CUDA, ROCm, TPU, framework, vendor, or accelerator names
 * part of the universal type grammar.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates NO new standalone AST type.
 *
 * The frontend must integrate the qualifier into the existing domain-neutral
 * type representation.
 *
 * Conceptually:
 *
 *     resource T
 *
 * becomes:
 *
 *     TypeExpr::Resource(inner)
 *
 * or the repository's already-established equivalent resource qualifier
 * representation.
 *
 * The exact Rust enum/field name belongs to the frontend AST implementation,
 * not this grammar.
 *
 * IMPORTANT:
 *
 * Do not create:
 *
 *     ResourceTypeAst
 *
 * solely for this grammar.
 *
 * Do not create:
 *
 *     QuantumResourceAst
 *
 * or:
 *
 *     HardwareResourceAst
 *
 * unless the existing domain-neutral AST architecture independently requires
 * them.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether resource qualification is legal;
 *     what resource abstraction is being named;
 *     whether the inner type is resource-compatible;
 *     whether ownership rules apply;
 *     whether capabilities are required;
 *     whether requirements are satisfiable;
 *     whether the program remains portable;
 *     whether a target can realize the requirement.
 *
 * The parser does none of those things.
 *
 * ============================================================================
 *
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has no direct IR dependency.
 *
 * Resource semantics may eventually become:
 *
 *     resource requirements
 *     capability requirements
 *     ownership metadata
 *     execution constraints
 *     optimization metadata
 *     deployment requirements
 *
 * in the canonical semantic/IR layers.
 *
 * The grammar must never introduce a hardware allocation IR.
 *
 * In particular, it must not introduce:
 *
 *     PhysicalResourceIR
 *     DeviceAllocationIR
 *     ResourcePlacementIR
 *
 * merely because the source uses the word `resource`.
 *
 * ============================================================================
 *
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler consumes resource semantics after parsing.
 *
 * The compiler may:
 *
 *     infer capabilities;
 *     compare requirements with target capabilities;
 *     specialize implementation;
 *     choose a backend;
 *     optimize resource usage;
 *     negotiate target realization;
 *     reject unsatisfied mandatory requirements.
 *
 * These are compiler responsibilities, not grammar responsibilities.
 *
 * ============================================================================
 *
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime resource discovery/allocation remains outside this grammar.
 *
 * Runtime may determine:
 *
 *     currently available resources;
 *     resource health;
 *     resource availability;
 *     dynamic capacity;
 *     placement;
 *     scheduling;
 *     recovery;
 *
 * without changing the source language's lexical or syntactic meaning.
 *
 * ============================================================================
 *
 * RESOURCE SCALABILITY
 * ============================================================================
 *
 * This file deliberately contains no finite resource enumeration.
 *
 * The grammar can therefore represent programs intended for:
 *
 *     one resource;
 *     many resources;
 *     symbolic resource counts;
 *     dynamically discovered resources;
 *     heterogeneous resources;
 *     distributed resources;
 *     quantum resources;
 *     classical resources;
 *     hardware resources;
 *     future computational resources.
 *
 * Practical limits are external implementation/resource constraints.
 *
 * They MUST NOT become grammar limits.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * For a fixed Zamani language version:
 *
 *     resource
 *
 * must always tokenize as:
 *
 *     RESOURCE
 *
 * and parse through:
 *
 *     resourceTypeQualifier
 *
 * independent of:
 *
 *     target hardware;
 *     machine size;
 *     available resources;
 *     runtime state;
 *     scheduling state;
 *     network state;
 *     quantum hardware;
 *     backend selection.
 *
 * ============================================================================
 *
 * DIAGNOSTICS
 * ============================================================================
 *
 * This grammar is intentionally minimal so that diagnostics remain precise.
 *
 * Examples:
 *
 *     resource
 *
 * followed by a valid type position must be diagnosed by the enclosing type
 * grammar as an incomplete type expression.
 *
 * Example:
 *
 *     resource 123
 *
 * must fail because the canonical type grammar does not accept `123` as the
 * required type operand.
 *
 * The resource grammar itself should not attempt to manufacture semantic
 * diagnostics.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no filesystem access;
 *     no network access;
 *     no device access;
 *     no runtime calls;
 *     no resource allocation;
 *     no code execution;
 *     no embedded Rust actions;
 *     no semantic predicates requiring external state.
 *
 * This is important for deterministic parsing and safe Rust integration.
 *
 * ============================================================================
 *
 * PERFORMANCE
 * ============================================================================
 *
 * The production rule is constant complexity:
 *
 *     resourceTypeQualifier
 *         : RESOURCE
 *         ;
 *
 * It does not scale with:
 *
 *     number of resource kinds;
 *     number of devices;
 *     number of qubits;
 *     number of CPUs;
 *     number of GPUs;
 *     number of nodes;
 *     generic arity;
 *     tensor rank.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing resource-related source semantics must be migrated deliberately.
 *
 * The obsolete/over-broad:
 *
 *     grammar/types/resource-types.g4
 *
 * MUST NOT become a second authority.
 *
 * It should either:
 *
 *     1. be reduced to a compatibility/documentation role; or
 *     2. be removed only after repository-wide reference analysis proves it is
 *        unused.
 *
 * It must not independently define a competing `resourceType` language.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser tests must include, through the composed type grammar:
 *
 *     resource T
 *     resource Qubit
 *     resource Buffer
 *     resource Foo::Bar
 *     resource ResourceLikeType
 *     affine resource Qubit
 *     linear resource Buffer
 *
 * where those qualifier combinations are enabled by the canonical type
 * composition.
 *
 * Negative tests must include:
 *
 *     resource
 *     resource 123
 *     resource ()
 *
 * when the surrounding type grammar rejects those operands.
 *
 * Boundary tests must include:
 *
 *     resource T?
 *     resource Foo<...>
 *     resource Foo<T, U, V>
 *     resource Foo::Bar::Baz
 *
 * subject to the canonical type grammar.
 *
 * Scalability tests must verify that no finite resource count is encoded.
 *
 * Determinism tests must verify identical token/parse sequences for identical
 * source text and language version regardless of target machine.
 *
 * Compatibility tests must verify that introducing this qualifier does not
 * change the tokenization of unrelated identifiers or resource-like names.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS REQUIREMENTS:
 *
 *     [x] No MAX_QUBITS
 *     [x] No MAX_CPUS
 *     [x] No MAX_CORES
 *     [x] No MAX_THREADS
 *     [x] No MAX_GPUS
 *     [x] No MAX_FPGAS
 *     [x] No MAX_QPUS
 *     [x] No MAX_NODES
 *     [x] No MAX_DEVICES
 *     [x] No MAX_MEMORY
 *     [x] No fixed topology
 *     [x] No physical device IDs
 *     [x] No physical qubit IDs
 *     [x] No fixed accelerator count
 *     [x] No vendor-specific resource enumeration
 *     [x] No resource allocation
 *     [x] No hardware discovery
 *     [x] No scheduling policy
 *     [x] No routing policy
 *     [x] No QEC implementation
 *     [x] No ZQN implementation
 *     [x] No HAL implementation
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     1. RESOURCE exists in the canonical lexer vocabulary;
 *     2. RESOURCE is owned only by the keyword lexer component;
 *     3. this grammar uses ZamaniLexer as token vocabulary;
 *     4. resourceTypeQualifier is the only resource-specific qualifier rule;
 *     5. types.g4 composes the qualifier;
 *     6. types.g4 does not define a competing resource grammar;
 *     7. no circular grammar dependency exists;
 *     8. no second type AST is introduced;
 *     9. resource semantics are handled downstream;
 *    10. quantum semantics continue through quantum::ir;
 *    11. no hardware limit exists in the grammar;
 *    12. positive/negative/boundary/scalability/determinism tests pass;
 *    13. generated Rust remains compatible with Rust 1.97/1.97.1;
 *    14. no unsafe Rust is introduced;
 *    15. repository-wide grammar/spec/lexer/AST integration is validated.
 *
 * ============================================================================
 */

parser grammar Resource;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * This rule intentionally recognizes ONLY the qualifier.
 *
 * The following is correct:
 *
 *     typeExpression
 *         : typeQualifier* typePrimary typePostfix*
 *         ;
 *
 *     typeQualifier
 *         : ...
 *         | resourceTypeQualifier
 *         ;
 *
 * The following is intentionally forbidden:
 *
 *     resourceTypeQualifier
 *         : RESOURCE typeExpression
 *         ;
 *
 * ============================================================================
 */

resourceTypeQualifier
    : RESOURCE
    ;