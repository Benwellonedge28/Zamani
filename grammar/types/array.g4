
/*
 * ============================================================================
 * Zamani Programming Language
 *
 * File: grammar/types/array.g4
 * Grammar: Array
 * Status: Canonical modular array-type grammar
 *
 * Compiler baseline:
 *   Rust 1.97 / Rust 1.97.1
 *   Rust 2021
 *   Safe Rust only; no unsafe.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * Defines portable, target-independent source syntax for array types.
 *
 * Supported forms:
 *
 *   [T]                    Dynamically sized or context-sized array
 *   [T; N]                 Explicit cardinality
 *   [T; N + M]             Computed cardinality
 *   [T; Rows * Columns]    Symbolic cardinality
 *   [[T; N]; M]            Nested arrays
 *   [Qubit; qubit_count]   Quantum element collection
 *   [Signal; signal_count] Hardware-related element collection
 *
 * This grammar describes source-level types. It does not allocate memory,
 * evaluate cardinalities, select hardware, or impose machine capacities.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *   arrayType
 *   arrayLength
 *
 * DOES NOT OWN:
 *   typeExpression
 *   typeCore
 *   typeValueExpression
 *   identifiers
 *   generic arguments
 *   numeric literals
 *   arithmetic operators
 *   type inference
 *   constant evaluation
 *   dependent-value evaluation
 *   memory allocation
 *   ownership or borrowing
 *   ABI layout
 *   tensor semantics
 *   hardware selection
 *   quantum allocation
 *   quantum routing
 *   QEC, ZQN, or HAL
 *   backend lowering
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate composed by grammar/types/types.g4.
 *
 * The delegating Types grammar owns:
 *
 *   typeExpression
 *   typeValueExpression
 *
 * The rules in this grammar deliberately reuse those canonical rules.
 *
 * This grammar MUST NOT define a second type-expression or value-expression
 * language.
 *
 * The Types grammar MUST import this grammar and MUST NOT retain a competing
 * inline definition of arrayType or arrayLength.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Canonical parser vocabulary:
 *
 *   ZamaniLexer
 *
 * Token ownership:
 *
 *   LBRACKET  -> grammar/lexer/punctuation.g4
 *   RBRACKET  -> grammar/lexer/punctuation.g4
 *   SEMICOLON -> grammar/lexer/punctuation.g4
 *
 * No lexer rules or literal spellings are declared here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Existing canonical representation:
 *
 *   TypeExpr::Array {
 *       element: Box<TypeExpr>,
 *       length: Option<TypeValueExpr>
 *   }
 *
 * Mapping:
 *
 *   [T]
 *       -> Array {
 *              element: T,
 *              length: None
 *          }
 *
 *   [T; N]
 *       -> Array {
 *              element: T,
 *              length: Some(N)
 *          }
 *
 * The parser must preserve source spans for the opening delimiter, element
 * type, optional cardinality, and closing delimiter.
 *
 * This grammar MUST NOT introduce another array AST or IR.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis determines:
 *
 *   - whether the element type is valid;
 *   - whether the cardinality expression is valid;
 *   - whether the cardinality is constant or symbolic;
 *   - whether generic parameters occur in the cardinality;
 *   - whether the array is dynamically sized;
 *   - whether zero cardinality is permitted by the applicable type rules;
 *   - whether cardinality is representable by the selected implementation;
 *   - whether the type satisfies ownership and lifetime rules;
 *   - whether the target can realize the required resources.
 *
 * The grammar MUST NOT:
 *
 *   - evaluate expressions;
 *   - convert cardinalities to usize;
 *   - allocate storage;
 *   - unroll arrays;
 *   - resolve generic parameters;
 *   - impose target-specific limits.
 *
 * ============================================================================
 * PORTABILITY AND SCALABILITY
 * ============================================================================
 *
 * Array syntax is independent of:
 *
 *   CPU, GPU, FPGA, ASIC, QPU, embedded devices, accelerators,
 *   distributed systems, clusters, HPC, and future targets.
 *
 * No grammar-level maximum exists for:
 *
 *   array length;
 *   array nesting;
 *   tensor rank;
 *   number of elements;
 *   number of dimensions;
 *   hardware resources.
 *
 * A nested array is represented compositionally, not by enumerating
 * array1, array2, array3, or array4.
 *
 * Practical parser/compiler safety budgets, if required, belong to explicit
 * implementation policy and must not silently change language semantics.
 *
 * Physical feasibility is evaluated downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *   [Qubit]
 *   [Qubit; N]
 *   [[Qubit; M]; N]
 *
 * These are source-level collections, not physical qubit allocations.
 *
 * Quantum-specific semantic lowering must use the canonical quantum::ir
 * boundary. This grammar does not define a quantum array IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *   [float; N]
 *   [TensorElement; N]
 *   [Signal; N]
 *   [Register<Value>; N]
 *
 * Whether these types are valid is determined by their respective semantic
 * systems. This grammar does not determine physical layout, synthesis,
 * placement, register width, or memory-bank allocation.
 *
 * ============================================================================
 * UBUNTU-DERIVED UNIVERSAL SEMANTICS
 * ============================================================================
 *
 * Array types may be used by reasoning, knowledge, learning, adaptation,
 * probabilistic computation, agents, data processing, and neural-symbolic
 * programs.
 *
 * Those features do not require separate array grammars.
 *
 * Their constraints, effects, capabilities, policies, contracts, evidence,
 * and provenance are handled by their respective semantic owners.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structurally invalid examples:
 *
 *   []
 *   [; N]
 *   [T;]
 *   [T N]
 *   [T; ; N]
 *
 * These must produce parser diagnostics rather than silently recovering into
 * a different valid array type.
 *
 * A syntactically valid but semantically invalid cardinality must produce a
 * semantic diagnostic, not an artificial grammar restriction.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *   [int]
 *   [int; 0]
 *   [int; 1024]
 *   [int; N]
 *   [int; Rows * Columns]
 *   [[float; M]; N]
 *   [Qubit; qubit_count]
 *   [Signal; signal_count]
 *   [Map<Key, Value>; N]
 *
 * Negative:
 *
 *   []
 *   [; N]
 *   [T;]
 *   [T N]
 *   [T; ; N]
 *
 * Boundary:
 *
 *   [T; N + M]
 *   [T; namespace::Dimension]
 *   [T; 2 * N]
 *   [[T; N]; M]
 *   [T; generic_parameter]
 *
 * Scalability:
 *
 *   - deeply nested arrays;
 *   - arbitrarily large source cardinality expressions;
 *   - symbolic cardinalities;
 *   - generic-dependent cardinalities;
 *   - large element types;
 *   - cross-domain element types.
 *
 * Test resource budgets must not become language limits.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *   grammar/types/types.g4
 *   grammar/antlr/ZamaniLexer.g4
 *   grammar/lexer/punctuation.g4
 *
 * EXPORTS:
 *   arrayType
 *   arrayLength
 *
 * CONSUMED_BY:
 *   grammar/types/types.g4
 *
 * AST_OWNER:
 *   Existing frontend TypeExpr
 *
 * SEMANTIC_OWNER:
 *   Canonical type semantic analysis
 *
 * IR_OWNER:
 *   Existing canonical semantic IR
 *
 * QUANTUM_IR_OWNER:
 *   quantum::ir
 *
 * SPEC_OWNER:
 *   grammar/specification/types.md
 *
 * TEST_OWNER:
 *   grammar/tests/types/array/
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * [ ] Types imports Array.
 * [ ] Types has no duplicate arrayType definition.
 * [ ] Types has no duplicate arrayLength definition.
 * [ ] All parser grammars use ZamaniLexer.
 * [ ] ANTLR delegate composition succeeds.
 * [ ] AST mapping matches existing TypeExpr::Array.
 * [ ] Symbolic cardinalities are preserved.
 * [ ] Nested arrays parse correctly.
 * [ ] Positive and negative tests pass.
 * [ ] No machine-capacity constants are introduced.
 * [ ] No unsafe Rust is introduced.
 * [ ] Rust 1.97 and 1.97.1 compatibility is verified.
 *
 * ============================================================================
 */

parser grammar Array;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Public rule:
 *
 *   [T]
 *   [T; N]
 *
 * The element type and cardinality are delegated to the canonical Types
 * grammar. This rule does not define their syntax independently.
 */
arrayType
    : LBRACKET typeExpression arrayLength? RBRACKET
    ;

/*
 * Optional explicit cardinality.
 *
 * The semicolon distinguishes an explicitly cardinalized array from an
 * array whose cardinality is omitted.
 */
arrayLength
    : SEMICOLON typeValueExpression
    ;
