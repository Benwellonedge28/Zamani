/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/array-types.g4
 *
 * Responsibility:
 *     Defines the source-level syntax of Zamani array and slice types.
 *
 * Architecture:
 *     Focused ANTLR parser-grammar delegate.
 *
 *     This grammar is imported/composed by:
 *
 *         grammar/types/types.g4
 *
 *     It intentionally does NOT define the canonical `typeExpression` rule.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * This file OWNS:
 *
 *     - arrayType
 *     - the distinction between:
 *
 *           [T]
 *
 *       and:
 *
 *           [T; N]
 *
 *     - the syntactic relationship between an element type and an optional
 *       compile-time cardinality expression
 *
 * This file DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - primitive types;
 *     - generic type syntax;
 *     - tuple syntax;
 *     - function types;
 *     - reference/pointer types;
 *     - type expressions generally;
 *     - type-level expression semantics;
 *     - constant evaluation;
 *     - array length validation;
 *     - memory layout;
 *     - alignment;
 *     - ABI representation;
 *     - allocation;
 *     - ownership;
 *     - borrowing;
 *     - runtime bounds;
 *     - machine memory limits;
 *     - vector width;
 *     - SIMD width;
 *     - accelerator dimensions;
 *     - hardware topology;
 *     - quantum resources;
 *     - physical qubit counts;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - hardware discovery;
 *     - runtime dispatch.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Array syntax describes PROGRAM SEMANTICS, not the physical machine.
 *
 * The grammar MUST NOT encode:
 *
 *     MAX_ARRAY_LENGTH
 *     MAX_DIMENSIONS
 *     MAX_ELEMENTS
 *     MAX_MEMORY
 *     MAX_VECTOR_WIDTH
 *     MAX_REGISTER_COUNT
 *     MAX_TENSOR_RANK
 *     MAX_ACCELERATOR_WIDTH
 *
 * or equivalent machine-specific restrictions.
 *
 * A program may therefore describe:
 *
 *     [T; N]
 *
 * where N is:
 *
 *     - a compile-time constant;
 *     - a generic parameter;
 *     - a symbolic dimension;
 *     - a dependent value;
 *     - a value resolved by semantic analysis;
 *     - a target-independent resource parameter.
 *
 * The actual representability of N is determined downstream by semantic
 * analysis, compilation, resource management, and the selected execution
 * environment.
 *
 * ============================================================================
 * ARRAY / SLICE SEMANTICS
 * ============================================================================
 *
 * The grammar recognizes two source forms:
 *
 *     [T]
 *
 * and:
 *
 *     [T; N]
 *
 * `[T]` is the unsized/slice form.
 *
 * `[T; N]` is the explicitly cardinality-parameterized array form.
 *
 * The grammar does NOT decide whether `[T]` becomes:
 *
 *     - a dynamically sized slice;
 *     - a view;
 *     - a borrowed sequence;
 *     - a runtime array abstraction;
 *     - another canonical semantic type.
 *
 * That decision belongs to semantic analysis.
 *
 * Likewise, the grammar does not evaluate N.
 *
 * ============================================================================
 * RECURSION AND MULTIDIMENSIONAL DATA
 * ============================================================================
 *
 * Multidimensional arrays are represented compositionally:
 *
 *     [[T; N]; M]
 *
 *     [[[T; A]; B]; C]
 *
 *     [[T]]
 *
 * No special multidimensional-array syntax is required.
 *
 * This is deliberate:
 *
 *     array(array(T, N), M)
 *
 * is more extensible than introducing a finite-dimensional grammar such as:
 *
 *     array2d
 *     array3d
 *     array4d
 *
 * There is therefore no fixed dimensionality ceiling.
 *
 * ============================================================================
 * GENERIC / SYMBOLIC CARDINALITIES
 * ============================================================================
 *
 * The cardinality expression is delegated to the canonical type-level value
 * expression supplied by the enclosing type grammar.
 *
 * Examples:
 *
 *     [T; N]
 *
 *     [T; size]
 *
 *     [T; N + 1]
 *
 *     [T; Rows * Cols]
 *
 *     [T; MatrixSize]
 *
 *     [T; namespace::Dimension]
 *
 * The array grammar does not determine whether a value is legal as a
 * cardinality. That is a semantic/type-system responsibility.
 *
 * ============================================================================
 * TYPE RECURSION
 * ============================================================================
 *
 * `typeExpression` is supplied by the enclosing Types grammar.
 *
 * This allows arrays to contain arbitrary types recognized by Zamani:
 *
 *     [int]
 *     [float; N]
 *     [Option<T>]
 *     [Result<T, E>; N]
 *     [(A, B); N]
 *     [fn(A) -> B]
 *     [&T]
 *     [Qubit]
 *     [LogicalQubit]
 *     [hardware::Resource]
 *     [distributed::Node]
 *
 * The array grammar does not need to know which domain owns those types.
 *
 * ============================================================================
 * QUANTUM COMPATIBILITY
 * ============================================================================
 *
 * Quantum collections remain source-level types.
 *
 * For example:
 *
 *     [qubit; N]
 *
 * expresses a collection of N qubit-typed values.
 *
 * It does NOT mean:
 *
 *     - allocate physical qubits 0..N;
 *     - use a particular QPU;
 *     - use a fixed topology;
 *     - use a fixed number of hardware qubits;
 *     - bind to a vendor;
 *     - bypass quantum::ir.
 *
 * Semantic lowering must preserve the distinction between a source-level
 * collection and physical resource allocation.
 *
 * ============================================================================
 * HDL / HARDWARE COMPATIBILITY
 * ============================================================================
 *
 * Hardware-related element types may appear inside arrays where permitted by
 * the semantic type system.
 *
 * Examples:
 *
 *     [Signal; N]
 *     [Port<T>; N]
 *     [Register<T>; N]
 *
 * This grammar does not determine hardware placement, synthesis, routing,
 * clocking, physical storage, or implementation technology.
 *
 * ============================================================================
 * IR BOUNDARY
 * ============================================================================
 *
 * The parser produces syntax only.
 *
 * The semantic layer should lower:
 *
 *     [T]
 *
 * and:
 *
 *     [T; N]
 *
 * into the repository's canonical type representation.
 *
 * The grammar MUST NOT introduce an ArrayIR, QuantumArrayIR, HardwareArrayIR,
 * or other parallel intermediate representation.
 *
 * For quantum programs, semantic lowering eventually integrates with the
 * canonical `quantum::ir` boundary rather than creating a second quantum
 * representation here.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Generated parser/frontend integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The generated Zamani compiler/frontend must not require `unsafe`.
 *
 * This grammar itself contains no unsafe operation.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Tokens are supplied by the canonical:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose lexer grammar is:
 *
 *     ZamaniTokens
 *
 * This file therefore MUST NOT declare:
 *
 *     LBRACKET
 *     RBRACKET
 *     SEMICOLON
 *
 * or any other lexer token.
 *
 * ============================================================================
 */

parser grammar ArrayTypes;

options {
    tokenVocab = ZamaniTokens;
}


/**
 * ============================================================================
 * Canonical array/slice type syntax
 * ============================================================================
 *
 * Unsized/slice form:
 *
 *     [T]
 *
 * Sized/value-parameterized form:
 *
 *     [T; N]
 *
 * The optional cardinality is syntactically distinguished by SEMICOLON.
 *
 * Examples:
 *
 *     [int]
 *     [int; 16]
 *     [float; N]
 *     [Qubit; number_of_qubits]
 *     [Matrix<T>; Rows]
 *     [[int; N]; M]
 *
 * There is deliberately no:
 *
 *     MAX_ARRAY_LENGTH
 *     MAX_ARRAY_DEPTH
 *     MAX_ARRAY_DIMENSIONS
 *
 * encoded here.
 */
arrayType
    : LBRACKET
      typeExpression
      arrayLength?
      RBRACKET
    ;


/**
 * ============================================================================
 * Optional array cardinality
 * ============================================================================
 *
 * The semicolon is part of the sized-array syntax:
 *
 *     [T; N]
 *
 * The value expression itself belongs to the enclosing type system.
 *
 * This prevents array-types.g4 from becoming a second expression/type-value
 * grammar.
 */
arrayLength
    : SEMICOLON
      typeValueExpression
    ;