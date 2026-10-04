
/*
 * ============================================================================
 * Zamani Programming Language
 *
 * File: grammar/types/slice.g4
 * Grammar: Slice
 * Status: Canonical modular slice-type grammar
 *
 * Compiler compatibility:
 *   Rust 1.97
 *   Rust 1.97.1
 *   Rust edition 2021
 *   Safe Rust only; no unsafe
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * Defines the canonical, target-independent syntax for slice types.
 *
 * Canonical syntax:
 *
 *     [T]
 *
 * Examples:
 *
 *     [int]
 *     [float]
 *     [User]
 *     [Tensor<float>]
 *     [[int]]
 *     [[[float]]]
 *     [fn(int) -> int]
 *     [Qubit]
 *     [Resource<T>]
 *
 * A slice describes a dynamically sized sequence of elements.
 *
 * This grammar describes type syntax only.
 *
 * It does not allocate memory, determine capacity, evaluate expressions,
 * select hardware, or establish physical resource availability.
 *
 * ============================================================================
 * 2. OWNERSHIP CONTRACT
 * ============================================================================
 *
 * OWNS:
 *
 *     sliceType
 *
 * DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     typeValueExpression
 *     identifiers
 *     qualified names
 *     generic arguments
 *     primitive types
 *     named types
 *     array types
 *     tuple types
 *     function types
 *     reference types
 *     pointer types
 *     optional types
 *     result types
 *     quantum types
 *     hardware types
 *     resource types
 *     capability types
 *     type inference
 *     type resolution
 *     ownership
 *     borrowing
 *     lifetimes
 *     memory allocation
 *     runtime representation
 *     memory placement
 *     ABI layout
 *     target selection
 *     scheduling
 *     routing
 *     optimization
 *     quantum error correction
 *     quantum IR
 *     hardware abstraction
 *
 * ============================================================================
 * 3. CANONICAL AUTHORITY
 * ============================================================================
 *
 * This grammar is the sole owner of the sliceType parser rule.
 *
 * The canonical type composition grammar owns typeExpression.
 *
 * The canonical type composition grammar MUST delegate slice parsing to
 * this grammar rather than defining another sliceType.
 *
 * No competing slice grammar may be introduced in:
 *
 *     grammar/types/types.g4
 *     grammar/types/array.g4
 *     grammar/types/array-types.g4
 *     grammar/types/slice-types.g4
 *     grammar/antlr/Types.g4
 *
 * Existing type grammar files must use the canonical sliceType rule.
 *
 * ============================================================================
 * 4. ARRAY DISTINCTION
 * ============================================================================
 *
 * Slice:
 *
 *     [T]
 *
 * Explicitly sized array:
 *
 *     [T; N]
 *
 * These are different source-level type constructors.
 *
 * The array grammar MUST require its explicit cardinality:
 *
 *     arrayType
 *         : LBRACK typeExpression arrayLength RBRACK
 *         ;
 *
 * The array grammar MUST NOT accept [T] as an alternative representation
 * of a sized array or an unspecified-cardinality array.
 *
 * The type composition grammar MUST distinguish the two forms without
 * relying on semantic guessing.
 *
 * This file does not define arrayType or arrayLength.
 *
 * ============================================================================
 * 5. LEXICAL CONTRACT
 * ============================================================================
 *
 * Grammar kind:
 *
 *     ANTLR parser grammar
 *
 * Canonical lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * Required tokens:
 *
 *     LBRACK
 *     RBRACK
 *
 * These tokens are owned by the canonical punctuation vocabulary.
 *
 * This grammar MUST NOT define lexer rules or literal spellings.
 *
 * No token is introduced merely to represent a slice.
 *
 * ============================================================================
 * 6. COMPOSITION CONTRACT
 * ============================================================================
 *
 * Public rule:
 *
 *     sliceType
 *
 * Required external rule:
 *
 *     typeExpression
 *
 * typeExpression is supplied by the canonical type grammar composition.
 *
 * The element type must be parsed exactly once through typeExpression.
 *
 * This grammar MUST NOT define a second type-expression language.
 *
 * This grammar MUST NOT copy primitive, generic, function, reference,
 * quantum, hardware, or resource type productions.
 *
 * The importing/composition grammar is responsible for resolving the
 * canonical typeExpression dependency.
 *
 * ============================================================================
 * 7. SOURCE-LEVEL SEMANTICS
 * ============================================================================
 *
 * A slice represents a dynamically sized sequence of values of one
 * element type.
 *
 * Its source-level structure is:
 *
 *     Slice(element_type)
 *
 * For example:
 *
 *     [int]
 *
 * represents a slice whose element type is int.
 *
 * It does not prescribe:
 *
 *     element count
 *     capacity
 *     allocation strategy
 *     memory address
 *     memory bank
 *     processor
 *     accelerator
 *     physical qubit
 *     device
 *     node
 *     storage layout
 *
 * A slice may be empty at runtime if permitted by the applicable semantic
 * and runtime rules.
 *
 * Empty slice values are not empty slice types.
 *
 * Therefore:
 *
 *     []
 *
 * is not a valid type expression.
 *
 * ============================================================================
 * 8. AST CONTRACT
 * ============================================================================
 *
 * Canonical frontend representation:
 *
 *     TypeExpr::Slice(Box<TypeExpr>)
 *
 * Mapping:
 *
 *     [T]
 *         |
 *         v
 *     sliceType
 *         |
 *         v
 *     TypeExpr::Slice(Box::new(T))
 *
 * No additional AST variant is introduced.
 *
 * In particular, this grammar MUST NOT introduce:
 *
 *     SliceTypeExpr
 *     DynamicArrayType
 *     RuntimeSliceType
 *     QuantumSliceType
 *
 * The existing frontend typed façade, where present, must delegate to
 * the canonical TypeExpr representation.
 *
 * The AST must preserve:
 *
 *     complete slice source span
 *     opening delimiter span
 *     element type span
 *     closing delimiter span
 *
 * Nested element types must retain their individual source spans.
 *
 * ============================================================================
 * 9. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntax and structure only.
 *
 * Semantic analysis is responsible for:
 *
 *     element-type validity
 *     generic substitution
 *     name resolution
 *     lifetime validation
 *     ownership validation
 *     borrowing validation
 *     mutability validation
 *     unsized-type restrictions
 *     storage compatibility
 *     type compatibility
 *     resource feasibility
 *
 * The grammar MUST NOT:
 *
 *     evaluate expressions
 *     infer element types
 *     resolve identifiers
 *     allocate elements
 *     calculate capacity
 *     convert symbolic values to machine integers
 *     select memory locations
 *     select hardware
 *     impose implementation-specific type limits
 *
 * ============================================================================
 * 10. TYPE COMPOSITION
 * ============================================================================
 *
 * The element position delegates to typeExpression.
 *
 * This allows slices to compose with every type supported by the
 * canonical type system.
 *
 * Examples:
 *
 *     [int]
 *     [User]
 *     [Vec<int>]
 *     [Map<string, float>]
 *     [fn(int) -> bool]
 *     [&T]
 *     [Result<T, E>]
 *     [Tensor<float>]
 *     [Resource<T>]
 *     [Capability<T>]
 *
 * Nested slices:
 *
 *     [[int]]
 *     [[[float]]]
 *     [[Vec<T>]]
 *
 * The grammar does not enumerate nesting levels.
 *
 * ============================================================================
 * 11. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical types are accepted through typeExpression.
 *
 * Examples:
 *
 *     [int]
 *     [float]
 *     [bool]
 *     [string]
 *     [Tensor<float>]
 *
 * This grammar does not prescribe:
 *
 *     numeric representation
 *     integer width
 *     floating-point format
 *     vectorization
 *     memory layout
 *     CPU instruction selection
 *
 * These belong to the semantic and backend layers.
 *
 * ============================================================================
 * 12. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types may be slice element types when valid in the canonical
 * type system.
 *
 * Examples:
 *
 *     [Qubit]
 *     [LogicalQubit]
 *     [QuantumState]
 *     [QRegister<N>]
 *
 * These are source-level type expressions.
 *
 * A slice of Qubit does not allocate physical qubits.
 *
 * This grammar MUST NOT:
 *
 *     allocate physical qubits
 *     identify physical qubits
 *     define coupling maps
 *     choose a quantum processor
 *     route quantum operations
 *     schedule quantum circuits
 *     define calibration
 *     implement error correction
 *     create another quantum IR
 *
 * Canonical quantum integration:
 *
 *     source
 *       |
 *       v
 *     frontend TypeExpr
 *       |
 *       v
 *     semantic quantum type
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 * ============================================================================
 * 13. HDL AND HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-related types may be element types if supported by the
 * canonical type system.
 *
 * Examples:
 *
 *     [Signal]
 *     [Signal<T>]
 *     [HardwareValue]
 *     [Resource<T>]
 *
 * This grammar does not prescribe:
 *
 *     register width
 *     signal width
 *     physical wiring
 *     placement
 *     timing
 *     memory banks
 *     device topology
 *     synthesis
 *     physical implementation
 *
 * Hardware realization belongs to the HDL, hardware, compiler, and
 * backend subsystems.
 *
 * ============================================================================
 * 14. RESOURCE AND CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A slice type describes a type-level sequence abstraction.
 *
 * It is not a resource declaration.
 *
 * The grammar does not reserve:
 *
 *     memory
 *     storage
 *     devices
 *     processors
 *     accelerators
 *     nodes
 *     qubits
 *
 * Resource requirements are handled by the canonical resource system.
 *
 * Capability negotiation and target feasibility remain downstream.
 *
 * ============================================================================
 * 15. UNIVERSAL COMPUTATION INTEGRATION
 * ============================================================================
 *
 * Slices are domain-neutral.
 *
 * They may be used by:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     numerical computation
 *     tensor computation
 *     data processing
 *     reasoning
 *     learning
 *     adaptation
 *     knowledge systems
 *     agents
 *     distributed computation
 *     neural-symbolic computation
 *
 * These domains MUST NOT introduce separate slice-type grammars.
 *
 * Domain-specific constraints, effects, capabilities, contracts,
 * policies, evidence, and provenance are owned by their respective
 * semantic systems.
 *
 * ============================================================================
 * 16. POCO-REAF AND PORTABILITY
 * ============================================================================
 *
 * Slice syntax MUST remain independent of available hardware.
 *
 * A source-level slice must not encode:
 *
 *     a particular processor
 *     a particular accelerator
 *     a particular device
 *     a particular node
 *     a particular memory capacity
 *     a particular physical address width
 *
 * The same source-level type must remain meaningful across supported
 * execution environments.
 *
 * A target that cannot realize a required operation or resource must
 * report a feasibility or capability diagnostic.
 *
 * It must not silently change the source program's meaning.
 *
 * ============================================================================
 * 17. SCALABILITY
 * ============================================================================
 *
 * There is no grammar-level maximum for:
 *
 *     slice element count
 *     slice capacity
 *     slice nesting
 *     element-type complexity
 *     generic composition
 *     target resource count
 *
 * Examples:
 *
 *     [T]
 *     [[T]]
 *     [[[T]]]
 *     [[[[T]]]]
 *
 * and further recursive compositions.
 *
 * Grammar recursion does not establish an unlimited implementation
 * resource guarantee.
 *
 * Parser and compiler resource-safety controls must be explicit,
 * configurable implementation policies.
 *
 * Such controls must not silently become language-level capacity limits.
 *
 * Runtime capacity and physical feasibility are determined downstream.
 *
 * ============================================================================
 * 18. DETERMINISM
 * ============================================================================
 *
 * For a fixed token sequence and fixed canonical vocabulary, slice parsing
 * must produce a deterministic structural result.
 *
 * The parser must preserve:
 *
 *     element type
 *     nested type structure
 *     source ordering
 *     source locations
 *
 * This grammar contains no:
 *
 *     semantic predicates
 *     embedded actions
 *     I/O
 *     source execution
 *     target queries
 *     runtime allocation
 *
 * ============================================================================
 * 19. DIAGNOSTICS
 * ============================================================================
 *
 * Required syntax diagnostics include:
 *
 *     missing opening bracket
 *     missing element type
 *     missing closing bracket
 *     malformed nested type
 *     sized array syntax used where a slice is required
 *
 * Invalid examples:
 *
 *     []
 *     [;]
 *     [int
 *     int]
 *     [int; N]
 *
 * The final example belongs to arrayType, not sliceType.
 *
 * Parser recovery must not silently reinterpret malformed sized-array
 * syntax as a valid slice.
 *
 * Semantic errors must be reported by semantic analysis.
 *
 * ============================================================================
 * 20. SECURITY AND IMPLEMENTATION REQUIREMENTS
 * ============================================================================
 *
 * This file contains grammar definitions only.
 *
 * It:
 *
 *     performs no I/O
 *     executes no source program
 *     evaluates no expression
 *     accesses no filesystem
 *     accesses no network
 *     accesses no hardware
 *     allocates no runtime slice
 *     contains no embedded Rust
 *     requires no unsafe Rust
 *
 * Generated Rust integration must use:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     edition 2021
 *
 * No unsafe Rust is required or permitted by this grammar contract.
 *
 * ============================================================================
 * 21. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     [int]
 *     [float]
 *     [bool]
 *     [string]
 *     [User]
 *     [Vec<int>]
 *     [Map<string, float>]
 *     [Qubit]
 *     [QuantumState]
 *     [Resource<T>]
 *     [[int]]
 *     [[[float]]]
 *     [fn(int) -> int]
 *
 * Negative:
 *
 *     []
 *     [;]
 *     [int
 *     int]
 *     [int; N]
 *     [int,,]
 *
 * Boundary:
 *
 *     [T]
 *     [[T]]
 *     deeply nested slices
 *     generic element types
 *     function element types
 *     reference element types
 *     quantum element types
 *     resource element types
 *
 * Scalability:
 *
 *     no grammar-level maximum element count
 *     no grammar-level maximum capacity
 *     no enumerated nesting levels
 *     no fixed target count
 *     no physical resource limits
 *
 * Determinism:
 *
 *     equivalent type structures produce equivalent AST structures
 *     nested slice ordering is preserved
 *     element source spans are preserved
 *
 * Compatibility:
 *
 *     canonical lexer vocabulary
 *     canonical type composition
 *     existing TypeExpr::Slice
 *     native parser type handling
 *     ANTLR parser type handling
 *
 * ============================================================================
 * 22. INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/types/types.g4
 *     grammar/types/array.g4
 *     grammar/lexer/punctuation.g4
 *     canonical ZamaniLexer vocabulary
 *
 * EXPORTS:
 *
 *     sliceType
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *
 * AST_OWNER:
 *
 *     Existing frontend TypeExpr
 *
 * AST_VARIANT:
 *
 *     TypeExpr::Slice(Box<TypeExpr>)
 *
 * SEMANTIC_OWNER:
 *
 *     Canonical type semantic analysis
 *
 * IR_OWNER:
 *
 *     Existing canonical semantic IR
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/slice/
 *
 * RUST_BASELINE:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * SAFETY:
 *
 *     Safe Rust only
 *
 * ============================================================================
 * 23. COMPLETION CRITERIA
 * ============================================================================
 *
 * [ ] sliceType is the sole canonical slice rule.
 * [ ] ZamaniLexer is the canonical token vocabulary.
 * [ ] LBRACK and RBRACK are canonical lexer tokens.
 * [ ] typeExpression is owned by the type composition grammar.
 * [ ] [T] parses as a slice.
 * [ ] [T; N] parses as an explicitly sized array.
 * [ ] [] is rejected as a type.
 * [ ] Nested slices parse correctly.
 * [ ] Generic element types parse correctly.
 * [ ] Function element types parse correctly.
 * [ ] Quantum element types compose correctly.
 * [ ] Resource element types compose correctly.
 * [ ] AST maps to the existing TypeExpr::Slice.
 * [ ] Source spans are preserved.
 * [ ] Semantic analysis remains downstream.
 * [ ] No physical resource is allocated by parsing.
 * [ ] No hardware capacity is encoded.
 * [ ] No artificial grammar nesting limit exists.
 * [ ] No competing slice grammar exists.
 * [ ] Positive tests pass.
 * [ ] Negative tests pass.
 * [ ] Boundary tests pass.
 * [ ] Scalability tests pass.
 * [ ] Determinism tests pass.
 * [ ] Rust 1.97 compatibility is verified.
 * [ ] Rust 1.97.1 compatibility is verified.
 * [ ] No unsafe Rust is introduced.
 *
 * ============================================================================
 */

parser grammar Slice;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Canonical slice syntax:
 *
 *     [T]
 *
 * The element type is parsed using the canonical typeExpression rule
 * supplied by the type-system composition boundary.
 *
 * Do not add array cardinality syntax here.
 *
 * Do not define typeExpression here.
 */
sliceType
    : LBRACK typeExpression RBRACK
    ;
