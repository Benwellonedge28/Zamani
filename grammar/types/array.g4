/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/array.g4
 *
 * Status:
 *     Canonical modular array-type grammar.
 *
 * Purpose:
 *     Defines the source-level syntax for Zamani array types while delegating
 *     element-type syntax and type-level value syntax to the canonical Types
 *     grammar.
 *
 * Architecture:
 *
 *     Zamani.g4
 *          |
 *          v
 *     Types
 *          |
 *          +--> Array
 *          |
 *          v
 *     typeExpression
 *          |
 *          v
 *     frontend TypeExpr::Array
 *          |
 *          v
 *     semantic type system
 *          |
 *          v
 *     canonical semantic IR
 *
 * This grammar MUST NOT become a second type system.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file owns:
 *
 *     - arrayType
 *     - arrayLength
 *     - the source-level distinction between:
 *
 *           [T]
 *
 *       and:
 *
 *           [T; N]
 *
 *     - the syntactic association between an element type and an optional
 *       source-level cardinality expression.
 *
 * This file does NOT own:
 *
 *     - lexer definitions;
 *     - identifiers;
 *     - paths;
 *     - primitive types;
 *     - generic types;
 *     - tuples;
 *     - functions;
 *     - references;
 *     - pointers;
 *     - options;
 *     - results;
 *     - quantum types;
 *     - hardware types;
 *     - resource types;
 *     - type-value expression syntax;
 *     - type inference;
 *     - constant evaluation;
 *     - dependent-value evaluation;
 *     - type checking;
 *     - memory allocation;
 *     - ownership;
 *     - borrowing;
 *     - ABI layout;
 *     - runtime representation;
 *     - hardware selection;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - optimization;
 *     - backend selection;
 *     - runtime execution.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate.
 *
 * The canonical composition owner is:
 *
 *     grammar/types/types.g4
 *
 * `Types` owns:
 *
 *     typeExpression
 *     typeValueExpression
 *
 * This grammar intentionally references those canonical rules.
 *
 * ANTLR grammar imports allow delegate rules to resolve rule references
 * supplied/overridden by the delegating grammar. Therefore this file must be
 * composed through `Types`; it must not create a duplicate type-expression
 * grammar merely to become standalone.
 *
 * The root composition is:
 *
 *     Zamani.g4
 *          |
 *          v
 *        Types
 *          |
 *          +--> Array
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Lexer ownership remains:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical lexer vocabulary:
 *
 *     ZamaniTokens
 *
 * This file therefore declares no lexer rules.
 *
 * Required tokens are supplied by the canonical vocabulary:
 *
 *     LBRACKET
 *     RBRACKET
 *     SEMICOLON
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Array syntax maps to the existing canonical frontend representation:
 *
 *     TypeExpr::Array {
 *         element: Box<TypeExpr>,
 *         length: Option<TypeValueExpr>
 *     }
 *
 * Mapping:
 *
 *     [T]
 *         ->
 *     TypeExpr::Array {
 *         element: T,
 *         length: None
 *     }
 *
 *     [T; N]
 *         ->
 *     TypeExpr::Array {
 *         element: T,
 *         length: Some(N)
 *     }
 *
 * This grammar MUST NOT introduce:
 *
 *     ArrayTypeIR
 *     QuantumArrayIR
 *     HardwareArrayIR
 *     RuntimeArrayIR
 *
 * or any second array representation.
 *
 * The repository's typed `ArrayType` façade already wraps the canonical
 * `TypeExpr::Array` representation and preserves symbolic `TypeValueExpr`
 * lengths. The grammar therefore lowers into that existing representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar answers only:
 *
 *     "What array type did the programmer write?"
 *
 * Semantic analysis determines:
 *
 *     - whether the element type is valid;
 *     - whether a length expression is valid;
 *     - whether the length is constant;
 *     - whether the length is symbolic;
 *     - whether the length depends on generic parameters;
 *     - whether the type is dynamically sized;
 *     - whether the type is legal in a particular context;
 *     - whether the required resources are available.
 *
 * The grammar MUST NOT evaluate array lengths.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Array syntax must remain independent of machine capacity.
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_ARRAY_LENGTH
 *     MAX_ARRAY_ELEMENTS
 *     MAX_ARRAY_DIMENSIONS
 *     MAX_TENSOR_RANK
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_QUBITS
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *
 * or equivalent implementation limits.
 *
 * Examples:
 *
 *     [int]
 *     [int; 1024]
 *     [int; N]
 *     [int; Rows * Cols]
 *     [Qubit; number_of_qubits]
 *     [[float; M]; N]
 *
 * are all syntactically valid where their component type/value expressions
 * are valid.
 *
 * A number such as `1024` is program semantics.
 *
 * A compiler-wide restriction such as:
 *
 *     "arrays may never contain more than 1024 elements"
 *
 * is NOT language grammar.
 *
 * Operational parser/compiler safety budgets may exist outside this grammar,
 * but such budgets must be configurable implementation policy and must not
 * alter Zamani language semantics.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Arrays are recursively compositional.
 *
 * Examples:
 *
 *     [T]
 *     [[T]]
 *     [[[T]]]
 *
 *     [T; N]
 *     [[T; N]; M]
 *     [[[T; A]; B]; C]
 *
 * There is no grammar-level dimensionality limit.
 *
 * The grammar does not unroll dimensions into:
 *
 *     array1
 *     array2
 *     array3
 *     array4
 *
 * Instead, an array element may itself be an array because `arrayType`
 * consumes the canonical `typeExpression`.
 *
 * This permits arbitrary nesting subject only to implementation resource
 * availability and explicit downstream compiler policies.
 *
 * ============================================================================
 * TYPE-LEVEL CARDINALITY
 * ============================================================================
 *
 * The cardinality expression is delegated to:
 *
 *     typeValueExpression
 *
 * owned by the canonical type grammar.
 *
 * Examples:
 *
 *     [T; N]
 *     [T; size]
 *     [T; Rows + Cols]
 *     [T; Rows * Cols]
 *     [T; 2 * N]
 *     [T; namespace::Dimension]
 *
 * are represented without converting the source value to a machine-sized
 * integer during parsing.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum element types are permitted because the element is a canonical
 * `typeExpression`.
 *
 * For example:
 *
 *     [Qubit]
 *     [Qubit; N]
 *     [[Qubit; M]; N]
 *
 * describe source-level collections.
 *
 * They do NOT mean:
 *
 *     - physical qubit IDs;
 *     - physical qubit allocation;
 *     - QPU selection;
 *     - coupling topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC implementation;
 *     - ZQN implementation;
 *     - HAL operations.
 *
 * Quantum semantics eventually integrate with the existing canonical
 * `quantum::ir` boundary.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Array element types may represent:
 *
 *     integers
 *     floating-point values
 *     vectors
 *     matrices
 *     tensors
 *     records
 *     user-defined types
 *     symbolic types
 *     resources
 *     capabilities
 *
 * This grammar does not need separate array syntax for each domain.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-related types may appear as array elements where permitted by the
 * semantic type system:
 *
 *     [Signal; N]
 *     [Register<T>; N]
 *     [Port<T>; N]
 *
 * The grammar does not determine:
 *
 *     - FPGA placement;
 *     - ASIC layout;
 *     - register-file size;
 *     - memory-bank selection;
 *     - physical wiring;
 *     - synthesis strategy;
 *     - timing implementation.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Array cardinality may describe semantic resource quantities:
 *
 *     [Resource; N]
 *     [Qubit; required_qubits]
 *     [Node; node_count]
 *
 * This expresses source-level intent only.
 *
 * It does NOT allocate those resources.
 *
 * Resource discovery and realization remain downstream responsibilities.
 *
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 *
 * Array syntax must remain portable across:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     ASIC
 *     embedded systems
 *     distributed systems
 *     HPC systems
 *     edge systems
 *     cloud systems
 *     future computational targets
 *
 * Target realization occurs after semantic analysis.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This grammar must permit the parser to report precise source spans for:
 *
 *     [
 *     ]
 *     ;
 *     element type
 *     cardinality expression
 *
 * Malformed examples:
 *
 *     []
 *     [; N]
 *     [T;]
 *     [T N]
 *     [T; ; N]
 *
 * must be rejected structurally.
 *
 * Semantic errors such as:
 *
 *     [T; negative_value]
 *
 * are NOT grammar errors when the expression itself is syntactically valid.
 *
 * Such legality belongs to semantic/type validation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar is deterministic with respect to the array delimiter:
 *
 *     LBRACKET
 *         typeExpression
 *         [SEMICOLON typeValueExpression]
 *     RBRACKET
 *
 * The semicolon is the sole syntactic discriminator between an unsized source
 * form and an explicitly cardinality-parameterized source form.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no execution;
 *     - performs no allocation based on array cardinality;
 *     - performs no hardware discovery;
 *     - performs no runtime resource allocation;
 *     - contains no Rust;
 *     - contains no unsafe operation.
 *
 * Generated Rust integration must remain:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     edition 2021
 *     stable Rust
 *     no unsafe code
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Owns only array syntax.
 *     [x] Uses canonical ZamaniTokens.
 *     [x] Reuses canonical typeExpression.
 *     [x] Reuses canonical typeValueExpression.
 *     [x] Supports unsized/source-cardinality-omitted arrays.
 *     [x] Supports symbolic cardinalities.
 *     [x] Supports nested arrays.
 *     [x] Supports arbitrary element types.
 *     [x] Does not introduce machine limits.
 *     [x] Does not duplicate TypeExpr.
 *     [x] Does not introduce an array IR.
 *     [x] Does not allocate runtime storage.
 *     [x] Does not select hardware.
 *     [x] Does not implement QEC/ZQN/HAL.
 *     [x] Preserves POCO-REAF.
 *     [x] Is compatible with the existing frontend ArrayType contract.
 *
 * Remaining repository integration is described below and must be performed
 * as part of the composition change, not by modifying this grammar later.
 *
 * ============================================================================
 */

parser grammar Array;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * Canonical array type
 * ============================================================================
 *
 * Unsized/source-cardinality-omitted form:
 *
 *     [T]
 *
 * Explicit cardinality form:
 *
 *     [T; N]
 *
 * The element type is deliberately delegated to the canonical `typeExpression`
 * rule supplied by the Types delegator.
 */
arrayType
    : LBRACKET
      typeExpression
      arrayLength?
      RBRACKET
    ;


/*
 * ============================================================================
 * Explicit cardinality
 * ============================================================================
 *
 * The semicolon distinguishes:
 *
 *     [T]
 *
 * from:
 *
 *     [T; N]
 *
 * `typeValueExpression` is owned by Types and therefore remains a single
 * canonical type-level value grammar for the entire language.
 */
arrayLength
    : SEMICOLON
      typeValueExpression
    ;