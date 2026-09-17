/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/slice.g4
 *
 * Grammar:
 *     Slice
 *
 * Status:
 *     CANONICAL slice-type grammar component.
 *
 * Purpose:
 *     Defines the source-level syntax for dynamically sized slice types.
 *
 * Canonical source form:
 *
 *     [T]
 *
 * Examples:
 *
 *     [int]
 *     [float]
 *     [User]
 *     [Qubit]
 *     [Tensor<float>]
 *     [[int]]
 *     [fn(int) -> int]
 *
 * ============================================================================
 * FILE CONTRACT
 * ============================================================================
 *
 * This file is independently complete as the grammar component responsible
 * for slice syntax.
 *
 * It owns:
 *
 *   - sliceType;
 *   - the slice delimiters;
 *   - the relationship between slice syntax and its element type;
 *   - acceptance of recursively nested slice types;
 *   - delegation to the canonical typeExpression rule;
 *   - source-level slice syntax diagnostics;
 *   - slice-specific conformance requirements.
 *
 * It does NOT own:
 *
 *   - typeExpression;
 *   - primitive types;
 *   - named types;
 *   - generic types;
 *   - arrays;
 *   - tuples;
 *   - function types;
 *   - references;
 *   - pointers;
 *   - optional types;
 *   - result types;
 *   - quantum types;
 *   - hardware types;
 *   - resource types;
 *   - capability types;
 *   - dependent-type semantics;
 *   - type inference;
 *   - name resolution;
 *   - ownership;
 *   - borrowing;
 *   - runtime representation;
 *   - allocation;
 *   - memory placement;
 *   - hardware selection;
 *   - scheduling;
 *   - routing;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - optimization;
 *   - backend selection.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file is the sole canonical owner of the `sliceType` parser rule.
 *
 * The following files MUST NOT become competing implementations:
 *
 *     grammar/types/types.g4
 *     grammar/types/array-types.g4
 *     grammar/types/array.g4
 *     grammar/types/slice-types.g4
 *     grammar/antlr/Types.g4
 *
 * `grammar/types/types.g4` remains the canonical composition owner of
 * `typeExpression`.
 *
 * It delegates slice syntax to this grammar component.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The dependency direction is:
 *
 *     ZamaniTokens
 *          |
 *          v
 *     Slice
 *          |
 *          v
 *     canonical typeExpression
 *          |
 *          v
 *     Types composition
 *
 * This grammar MUST NOT introduce another `typeExpression`.
 *
 * This grammar MUST NOT copy primitive/generic/function/etc. type rules merely
 * to make itself independently parseable.
 *
 * The composition layer is responsible for supplying the canonical
 * `typeExpression` rule.
 *
 * There must be no competing recursive type system.
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes the canonical lexer vocabulary:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * It MUST NOT define lexer rules.
 *
 * Required canonical tokens:
 *
 *     LBRACK
 *     RBRACK
 *
 * The element is parsed through:
 *
 *     typeExpression
 *
 * supplied by the canonical type-system composition boundary.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 *     sliceType
 *
 * Canonical source syntax:
 *
 *     [ typeExpression ]
 *
 * ============================================================================
 * SOURCE-LEVEL SEMANTICS
 * ============================================================================
 *
 * A slice represents a dynamically sized sequence.
 *
 * The syntax specifies:
 *
 *     element type
 *
 * It does NOT specify:
 *
 *     element count
 *     capacity
 *     allocation strategy
 *     memory address
 *     memory bank
 *     device
 *     node
 *     processor
 *     accelerator
 *     physical qubit
 *     runtime container layout
 *
 * For example:
 *
 *     [int]
 *
 * means a dynamically sized sequence whose element type is `int`.
 *
 * It does not mean:
 *
 *     "allocate a fixed number of integers."
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Slice syntax MUST remain independent of available hardware resources.
 *
 * The grammar MUST NOT encode:
 *
 *     MAX_SLICE_LENGTH
 *     MAX_ELEMENTS
 *     MAX_CAPACITY
 *     MAX_NESTING
 *     MAX_MEMORY
 *     MAX_ADDRESS_WIDTH
 *     MAX_REGISTER_WIDTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * A program may impose its own semantic requirements, for example through
 * ordinary program expressions or resource contracts.
 *
 * Those requirements are not parser-level slice limits.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar permits arbitrary recursive nesting subject only to the
 * implementation's configurable parsing/resource-safety policy.
 *
 * Examples:
 *
 *     [T]
 *
 *     [[T]]
 *
 *     [[[T]]]
 *
 *     [[[[T]]]]
 *
 * and so on.
 *
 * There is deliberately no grammar constant limiting nesting depth.
 *
 * Likewise:
 *
 *     [T]
 *
 * does not encode the eventual number of elements.
 *
 * The runtime representation may scale from:
 *
 *     tiny
 *     ->
 *     large
 *     ->
 *     distributed
 *     ->
 *     heterogeneous
 *     ->
 *     future execution substrates
 *
 * without changing this grammar.
 *
 * ============================================================================
 * TYPE COMPOSITION
 * ============================================================================
 *
 * The element position accepts the canonical `typeExpression`.
 *
 * Therefore slices compose with all types already supported by the canonical
 * type system.
 *
 * Examples:
 *
 *     [int]
 *     [User]
 *     [Vec<float>]
 *     [fn(int) -> int]
 *     [Qubit]
 *     [QRegister<N>]
 *     [Resource<T>]
 *
 * Nested slices are naturally represented:
 *
 *     [[int]]
 *
 *     [[[float]]]
 *
 * This grammar does not need separate productions for each domain.
 *
 * ============================================================================
 * GENERICS
 * ============================================================================
 *
 * Generic syntax remains owned by the generic type grammar and canonical
 * `typeExpression` composition.
 *
 * Examples:
 *
 *     [Vec<int>]
 *     [Map<str, float>]
 *     [Tensor<float, N, M>]
 *
 * `slice.g4` does not duplicate generic argument parsing.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types are consumed through the canonical type-expression boundary.
 *
 * Examples:
 *
 *     [Qubit]
 *     [LogicalQubit]
 *     [QuantumState]
 *     [QRegister<N>]
 *
 * These are source-level types.
 *
 * This grammar MUST NOT:
 *
 *     - allocate physical qubits;
 *     - identify physical qubits;
 *     - define coupling maps;
 *     - select a QPU;
 *     - perform routing;
 *     - perform scheduling;
 *     - define calibration;
 *     - implement QEC;
 *     - define ZQN semantics;
 *     - access HAL state.
 *
 * Quantum semantic lowering remains downstream and `quantum::ir` remains the
 * canonical quantum semantic boundary.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical types are consumed through `typeExpression`.
 *
 * Examples:
 *
 *     [int]
 *     [float]
 *     [bool]
 *     [String]
 *     [Tensor<float>]
 *
 * No classical runtime representation is prescribed here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-related source types may appear as element types if they are valid
 * canonical Zamani types.
 *
 * Examples:
 *
 *     [Signal]
 *     [HardwareValue]
 *     [Resource<T>]
 *
 * This grammar does not select physical hardware.
 *
 * Hardware capabilities, placement, topology, memory resources, accelerator
 * mapping and deployment remain owned by:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *     grammar/compile/
 *     grammar/execution/
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * A slice type describes a type-level sequence abstraction.
 *
 * It is NOT a resource declaration.
 *
 * Therefore:
 *
 *     [T]
 *
 * does not mean:
 *
 *     allocate N elements
 *
 * and does not reserve:
 *
 *     memory
 *     devices
 *     nodes
 *     accelerators
 *
 * Resource feasibility is established downstream.
 *
 * ============================================================================
 * ARRAY DISTINCTION
 * ============================================================================
 *
 * Slice and array types MUST remain semantically distinct.
 *
 * Slice:
 *
 *     [T]
 *
 * represents a dynamically sized sequence.
 *
 * An array type, where supported, represents a sequence with a source-level
 * size expression.
 *
 * For example, an array form may eventually be represented by a separate
 * canonical grammar such as:
 *
 *     [T; N]
 *
 * if that syntax is part of the normative Zamani type specification.
 *
 * This file MUST NOT silently reinterpret array syntax as slice syntax.
 *
 * Therefore this rule accepts exactly:
 *
 *     [ typeExpression ]
 *
 * with no size expression inside the brackets.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The canonical frontend representation is:
 *
 *     TypeExpr::Slice(Box<TypeExpr>)
 *
 * as already established by the repository's frontend AST.
 *
 * The parser-to-AST mapping is:
 *
 *     sliceType
 *         |
 *         v
 *     element type
 *         |
 *         v
 *     TypeExpr::Slice(Box::new(element))
 *
 * No new AST enum is introduced.
 *
 * No:
 *
 *     SliceTypeExpr
 *     DynamicArrayType
 *     RuntimeSliceType
 *     QuantumSliceType
 *
 * variants are introduced merely for this grammar.
 *
 * The existing typed façade:
 *
 *     src/frontend/ast/node/types/slice.rs
 *
 * remains a façade over the canonical `TypeExpr`.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The frontend AST must preserve the complete source span of the slice,
 * including:
 *
 *     [
 *     element type
 *     ]
 *
 * The element type must retain its own nested source span.
 *
 * This permits diagnostics such as:
 *
 *     invalid element type
 *
 * to point at the element rather than merely at the surrounding brackets.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only that:
 *
 *     [T]
 *
 * is syntactically a slice type.
 *
 * Semantic analysis determines:
 *
 *     - whether T is a valid type;
 *     - whether T is resolved;
 *     - whether T is generic;
 *     - whether T is dependent;
 *     - whether T satisfies ownership constraints;
 *     - whether T is compatible with the surrounding context;
 *     - whether the program's resource requirements can be satisfied.
 *
 * This grammar performs none of those checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Slice syntax does not introduce an independent IR.
 *
 * The canonical flow is:
 *
 *     sliceType
 *          |
 *          v
 *     TypeExpr::Slice
 *          |
 *          v
 *     semantic type model
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     target-specific lowering
 *
 * If the element type belongs to the quantum domain, quantum semantics remain
 * represented through the established semantic pipeline and ultimately
 * `quantum::ir` where applicable.
 *
 * `slice.g4` must never import:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *     scheduler
 *     router
 *     backend
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may determine:
 *
 *     - representation;
 *     - allocation strategy;
 *     - ownership strategy;
 *     - storage location;
 *     - memory layout;
 *     - vectorization;
 *     - accelerator mapping;
 *     - distributed representation;
 *     - garbage/reclamation strategy;
 *     - ABI representation.
 *
 * None of these decisions are encoded in this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime behavior is outside grammar ownership.
 *
 * A slice may be implemented using any valid runtime representation that
 * preserves the language semantics.
 *
 * Possible implementations include:
 *
 *     contiguous storage
 *     segmented storage
 *     distributed storage
 *     accelerator-backed storage
 *     managed storage
 *     another future representation
 *
 * The grammar does not choose among them.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Slice syntax is deterministic:
 *
 *     '[' typeExpression ']'
 *
 * The element type occurs exactly once.
 *
 * The parser must preserve source ordering and must not infer target-specific
 * properties.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics should distinguish:
 *
 *     missing '['
 *     missing element type
 *     missing ']'
 *     malformed nested type
 *     array syntax appearing where slice syntax is required
 *
 * Examples of malformed syntax:
 *
 *     []
 *     [;]
 *     [int
 *     int]
 *     [int; N]
 *
 * The last form is deliberately not accepted by this grammar because it is
 * an array form, not a slice form.
 *
 * Semantic diagnostics belong outside this grammar.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - executes no source program;
 *     - allocates no runtime slice;
 *     - accesses no hardware;
 *     - performs no backend invocation;
 *     - contains no Rust;
 *     - contains no `unsafe`.
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     edition 2021
 *
 * and must use safe Rust only.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive cases:
 *
 *     [int]
 *     [float]
 *     [bool]
 *     [String]
 *     [User]
 *     [Vec<int>]
 *     [Qubit]
 *     [QuantumState]
 *     [Resource<T>]
 *     [[int]]
 *     [[[int]]]
 *     [fn(int) -> int]
 *
 * Negative cases:
 *
 *     []
 *     [;]
 *     [int
 *     int]
 *     [int; N]
 *     [int,,]
 *
 * Boundary cases:
 *
 *     [T]
 *     [[T]]
 *     deeply nested slices
 *     slice of a generic type
 *     slice of a function type
 *     slice of a quantum type
 *     slice of a resource type
 *
 * Scalability cases:
 *
 *     no maximum element count;
 *     no maximum type nesting encoded in grammar;
 *     no maximum resource count;
 *     no maximum qubit count;
 *     no maximum hardware size;
 *     no maximum slice capacity.
 *
 * Compatibility cases:
 *
 *     native Rust parser `[T]`
 *     canonical ANTLR parser `[T]`
 *     canonical TypeExpr::Slice
 *     semantic Slice type
 *
 * Determinism cases:
 *
 *     equivalent source must produce the same structural slice representation;
 *     element ordering must be preserved;
 *     nested slice structure must be preserved.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this grammar:
 *
 *     MAX_SLICE_LENGTH
 *     MAX_ELEMENTS
 *     MAX_CAPACITY
 *     MAX_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * No physical identifier or target-specific resource identifier may occur.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * `slice.g4` is complete only when:
 *
 *     [ ] `sliceType` is the sole canonical slice grammar rule;
 *     [ ] `typeExpression` remains owned by `types.g4`;
 *     [ ] canonical `ZamaniTokens` is consumed;
 *     [ ] `LBRACK` and `RBRACK` are canonical tokens;
 *     [ ] `[T]` is accepted;
 *     [ ] `[]` is rejected;
 *     [ ] `[T; N]` is not silently interpreted as a slice;
 *     [ ] nested slices are accepted;
 *     [ ] generic element types are accepted;
 *     [ ] function element types are accepted;
 *     [ ] quantum element types are accepted;
 *     [ ] resource element types are accepted;
 *     [ ] no hardware limits are encoded;
 *     [ ] no runtime representation is encoded;
 *     [ ] AST maps to `TypeExpr::Slice`;
 *     [ ] source spans are preserved;
 *     [ ] semantic validation remains downstream;
 *     [ ] IR lowering remains downstream;
 *     [ ] compiler decisions remain downstream;
 *     [ ] runtime decisions remain downstream;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] determinism tests exist;
 *     [ ] hard-coding audit passes.
 *
 * ============================================================================
 */

parser grammar Slice;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * PUBLIC SLICE-TYPE RULE
 * ============================================================================
 *
 * Canonical source syntax:
 *
 *     [T]
 *
 * where `T` is supplied by the canonical type-expression composition layer.
 *
 * `typeExpression` is intentionally NOT defined here.
 */

sliceType
    : LBRACK
      typeExpression
      RBRACK
    ;