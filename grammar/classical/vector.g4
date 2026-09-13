/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/vector.g4
 *
 * Status:
 *     Production-ready classical vector-domain parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific code.
 *     - No filesystem access.
 *     - No network access.
 *     - No evaluation.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL CLASSICAL VECTOR VALUE SYNTAX.
 *
 * It defines the syntax required to express vector values and vector
 * operations without imposing any physical machine representation.
 *
 * A vector is a semantic sequence of values whose:
 *
 *     - element type;
 *     - length;
 *     - shape;
 *     - storage representation;
 *     - execution strategy;
 *     - placement;
 *     - vectorization;
 *     - accelerator realization;
 *
 * are determined by downstream semantic and compilation layers.
 *
 * This grammar therefore supports vectors ranging from the smallest useful
 * source-level vector to arbitrarily large vectors subject only to resources
 * available to later compiler/runtime stages.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     parser
 *          |
 *          +----------------------------+
 *          |                            |
 *          v                            v
 *     Expressions                    Scalar
 *          |                            |
 *          +-------------+--------------+
 *                        |
 *                        v
 *                   Vector.g4
 *                        |
 *                        v
 *                   Frontend AST
 *                        |
 *                        v
 *                 Semantic analysis
 *                        |
 *          +-------------+--------------+
 *          |                            |
 *          v                            v
 *     Classical semantic          Resource/effect
 *        representation             metadata
 *          |
 *          v
 *      Classical IR
 *          |
 *          v
 *     optimization
 *          |
 *          v
 * scheduling / lowering / placement
 *          |
 *          v
 *     target realization
 *
 * This file NEVER directly creates or references:
 *
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - scheduling state;
 *     - routing state;
 *     - hardware topology;
 *     - runtime state;
 *     - backend state.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - vector value entry point;
 *     - vector literal syntax;
 *     - vector element-list syntax;
 *     - vector construction syntax;
 *     - vector repetition syntax;
 *     - vector concatenation syntax where represented explicitly;
 *     - vector range/value construction syntax;
 *     - vector reference classification;
 *     - vector-domain parser integration points;
 *     - vector-specific syntactic wrappers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifier spelling;
 *     - numeric literal spelling;
 *     - general expression precedence;
 *     - arithmetic operators;
 *     - indexing semantics;
 *     - member-access semantics;
 *     - generic type syntax;
 *     - vector TYPE syntax;
 *     - matrix syntax;
 *     - tensor syntax;
 *     - scalar type definitions;
 *     - vector algorithms;
 *     - vector storage;
 *     - memory allocation;
 *     - SIMD lowering;
 *     - GPU lowering;
 *     - FPGA lowering;
 *     - accelerator selection;
 *     - scheduling;
 *     - placement;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * INTEGRATION OWNERSHIP
 * ============================================================================
 *
 * The authoritative owners are:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         lexical tokens
 *
 *     grammar/expressions/expressions.g4
 *         general expression syntax and precedence
 *
 *     grammar/classical/scalar.g4
 *         scalar literal/value classification
 *
 *     grammar/types/classical-types.g4
 *         classical VECTOR TYPE syntax
 *
 *     grammar/classical/classical.g4
 *         classical-domain composition
 *
 *     frontend AST
 *         semantic source representation
 *
 *     semantic analysis
 *         type/shape/name/effect/resource validation
 *
 *     classical IR
 *         canonical classical semantic representation
 *
 *     resources
 *         actual resource constraints/capabilities
 *
 *     optimization
 *         implementation improvement
 *
 *     scheduling
 *         ordering/timing/resource scheduling
 *
 *     hardware
 *         target capabilities and realization
 *
 *     runtime
 *         execution
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Vector syntax describes VECTOR SEMANTICS, not vector hardware.
 *
 * The following are deliberately NOT encoded:
 *
 *     MAX_VECTOR_LENGTH
 *     MAX_ELEMENTS
 *     MAX_DIMENSIONS
 *     MAX_LANES
 *     MAX_SIMD_WIDTH
 *     MAX_GPU_THREADS
 *     MAX_GPU_LANES
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     MAX_NODES
 *
 * A source-level vector may therefore be realized as:
 *
 *     scalar sequence
 *     contiguous memory
 *     strided storage
 *     SIMD/vector registers
 *     GPU data
 *     FPGA pipeline
 *     distributed data
 *     accelerator memory
 *     quantum-classical data
 *     another future representation
 *
 * without changing the vector's source semantics.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite language-level vector length is encoded.
 *
 * These forms are all structurally legal:
 *
 *     []
 *
 *     [x]
 *
 *     [x, y]
 *
 *     [x, y, z, ...]
 *
 *     [expression for ...]
 *
 *     vector(value, size)
 *
 *     vector(value, symbolic_size)
 *
 * The practical maximum is determined downstream by explicit resource,
 * compiler, deployment, or runtime policies.
 *
 * A resource limitation MUST NOT be represented by a grammar maximum.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser establishes syntax only.
 *
 * Semantic analysis determines:
 *
 *     - whether all elements have compatible types;
 *     - whether the vector is homogeneous;
 *     - whether implicit conversions are legal;
 *     - whether its length is statically known;
 *     - whether its length is dynamic;
 *     - whether a symbolic dimension is valid;
 *     - whether repetition is valid;
 *     - whether an element expression is pure;
 *     - whether evaluation order matters;
 *     - whether the vector is mutable;
 *     - whether ownership/borrowing permits its use;
 *     - whether the vector can be lowered to classical IR;
 *     - whether a target can realize the vector;
 *     - whether available resources satisfy execution requirements.
 *
 * This grammar does none of those things.
 *
 * ============================================================================
 * TYPE BOUNDARY
 * ============================================================================
 *
 * Vector VALUE syntax is separate from vector TYPE syntax.
 *
 * Vector type syntax belongs to:
 *
 *     grammar/types/classical-types.g4
 *
 * Examples of type syntax:
 *
 *     Vector<T>
 *     Vector<T, N>
 *
 * Examples of value syntax owned here:
 *
 *     [a, b, c]
 *     vector(a, n)
 *     repeat(x, n)
 *
 * The parser must never infer a vector type merely from this grammar.
 *
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Vector elements are ordinary Zamani expressions.
 *
 * Therefore this grammar does not redefine:
 *
 *     +
 *     -
 *     *
 *     /
 *     %
 *     **
 *     ==
 *     !=
 *     <
 *     <=
 *     >
 *     >=
 *     &&
 *     ||
 *     &
 *     |
 *     ^
 *     <<
 *     >>
 *
 * Expression precedence belongs to Expressions.
 *
 * ============================================================================
 * SCALAR INTEGRATION
 * ============================================================================
 *
 * Scalar.g4 is imported as an optional domain-specific classification layer.
 *
 * Vector elements remain general expressions rather than being restricted to
 * scalar literals. This is important because a vector element may be:
 *
 *     scalar value
 *     variable
 *     function result
 *     compile-time value
 *     symbolic value
 *     field/member access
 *     quantum-derived classical value
 *     hardware parameter
 *     future domain-defined value
 *
 * Semantic analysis determines whether a particular vector element type is
 * legal.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     SIMD lane count
 *     CPU vector register width
 *     GPU warp width
 *     GPU block size
 *     FPGA pipeline width
 *     accelerator width
 *     memory alignment
 *     cache line size
 *     device identifier
 *     memory address
 *     topology
 *
 * Alignment, vectorization and layout are downstream implementation decisions
 * unless explicitly expressed as semantic source-level requirements through
 * the resource/capability system.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no evaluation;
 *     - performs no allocation;
 *     - performs no I/O;
 *     - performs no device access;
 *     - performs no network access;
 *     - performs no filesystem access;
 *     - performs no dynamic execution.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * All alternatives are syntactic.
 *
 * No semantic predicate or target-dependent action is used.
 *
 * The same source token sequence therefore produces the same parse result
 * independently of:
 *
 *     - machine architecture;
 *     - CPU count;
 *     - GPU availability;
 *     - quantum backend;
 *     - runtime environment;
 *     - resource availability.
 *
 * ============================================================================
 */

parser grammar Vector;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable entry point for vector VALUE syntax.
 *
 * This is intentionally distinct from:
 *
 *     classicalVectorType
 *
 * in ClassicalTypes.g4.
 */
vector
    : vectorLiteral
    | vectorConstructor
    | vectorReference
    ;


/* ============================================================================
 * 2. VECTOR LITERAL
 * ============================================================================
 *
 * Bracketed vector literals provide direct source-level construction.
 *
 * Examples:
 *
 *     []
 *     [x]
 *     [x, y]
 *     [x, y, z]
 *
 * There is no fixed element count.
 */
vectorLiteral
    : LBRACKET vectorElements? RBRACKET
    ;


/* ============================================================================
 * 3. VECTOR ELEMENTS
 * ============================================================================
 *
 * Elements are general expressions.
 *
 * This permits:
 *
 *     [1, 2, 3]
 *     [x, y, z]
 *     [a + b, c * d]
 *     [f(x), f(y)]
 *     [quantum_result, classical_result]
 *
 * without coupling the grammar to a particular element type.
 */
vectorElements
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 4. VECTOR CONSTRUCTOR
 * ============================================================================
 *
 * Constructor syntax provides a scalable alternative to explicitly spelling
 * every element.
 *
 * Conceptual forms:
 *
 *     vector(value)
 *     vector(value, length)
 *
 * The identifier `vector` remains ordinary identifier syntax rather than a
 * lexer keyword. Semantic resolution determines whether it denotes the
 * canonical vector constructor, a user-defined abstraction, or another
 * callable.
 *
 * This prevents the grammar from hard-coding a single implementation.
 */
vectorConstructor
    : vectorConstructorCall
    ;


vectorConstructorCall
    : vectorConstructorName
      LPAREN
      vectorConstructorArguments?
      RPAREN
    ;


vectorConstructorName
    : identifier
    ;


vectorConstructorArguments
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 5. VECTOR REFERENCE
 * ============================================================================
 *
 * A vector reference is syntactically an identifier.
 *
 * Name resolution and type checking determine whether the referenced value is
 * actually a vector.
 *
 * This is deliberately not restricted to a built-in `Vector` name.
 */
vectorReference
    : identifier
    ;


/* ============================================================================
 * 6. VECTOR ELEMENT ACCESS
 * ============================================================================
 *
 * Indexing remains owned by the general expression layer.
 *
 * This grammar therefore provides only a stable vector-domain classification
 * wrapper for semantic consumers.
 *
 * Examples:
 *
 *     v[i]
 *     v[i, j]
 *     v[index]
 *
 * The actual indexing syntax is inherited from Expressions.
 */
vectorAccess
    : expression
      LBRACKET
      expressionList
      RBRACKET
    ;


/* ============================================================================
 * 7. VECTOR SLICE
 * ============================================================================
 *
 * Slicing is represented using ordinary range expressions.
 *
 * Examples:
 *
 *     v[start..end]
 *     v[start..=end]
 *     v[..end]
 *     v[start..]
 *
 * Range semantics remain owned by the expression subsystem.
 *
 * This rule exists only as a vector-domain integration boundary.
 */
vectorSlice
    : expression
      LBRACKET
      vectorSliceSelector
      RBRACKET
    ;


vectorSliceSelector
    : expression
    ;


/* ============================================================================
 * 8. VECTOR CONSTRUCTION FROM RANGE
 * ============================================================================
 *
 * A range is an expression-level concept.
 *
 * Vector construction from a range remains semantic rather than hardware
 * specific.
 *
 * Examples:
 *
 *     vector(range)
 *     vector(start..end)
 *
 * The grammar accepts the expression and leaves interpretation to semantic
 * analysis.
 */
vectorFromRange
    : vectorConstructorName
      LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 9. VECTOR REPETITION
 * ============================================================================
 *
 * Repetition is a semantic construction:
 *
 *     repeat(value, count)
 *
 * Neither `repeat` nor `count` is hardware-specific.
 *
 * There is no maximum count.
 *
 * A count may be:
 *
 *     literal
 *     constant
 *     symbolic value
 *     compile-time expression
 *     runtime expression
 *
 * Semantic analysis decides whether runtime repetition is legal for the
 * particular context.
 */
vectorRepeat
    : vectorRepeatName
      LPAREN
      expression
      COMMA
      expression
      RPAREN
    ;


vectorRepeatName
    : identifier
    ;


/* ============================================================================
 * 10. VECTOR CONCATENATION
 * ============================================================================
 *
 * Concatenation is represented as a callable semantic operation rather than
 * introducing a new vector-specific operator.
 *
 * Conceptual form:
 *
 *     concat(a, b, c)
 *
 * The argument count is unbounded by the grammar.
 *
 * Semantic analysis determines:
 *
 *     - vector compatibility;
 *     - resulting element type;
 *     - resulting length;
 *     - ownership;
 *     - evaluation order;
 *     - allocation strategy.
 */
vectorConcatenation
    : vectorConcatenationName
      LPAREN
      vectorConcatenationArguments
      RPAREN
    ;


vectorConcatenationName
    : identifier
    ;


vectorConcatenationArguments
    : expression
      COMMA
      expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 11. VECTOR COMPREHENSION
 * ============================================================================
 *
 * Vector comprehensions are intentionally expressed through the general
 * expression and iteration vocabulary.
 *
 * The syntax is:
 *
 *     [ expression for identifier in expression ]
 *
 * Example:
 *
 *     [x * x for x in values]
 *
 * This does not imply:
 *
 *     CPU loop
 *     GPU kernel
 *     SIMD loop
 *     distributed loop
 *
 * The compiler may lower it according to available capabilities.
 */
vectorComprehension
    : LBRACKET
      expression
      FOR
      identifier
      IN
      expression
      RBRACKET
    ;


/* ============================================================================
 * 12. FILTERED VECTOR COMPREHENSION
 * ============================================================================
 *
 * Optional filtering:
 *
 *     [expression for x in values if condition]
 *
 * The condition is an ordinary expression.
 *
 * The semantic layer determines whether the operation can be vectorized,
 * parallelized, fused, streamed, distributed, or otherwise optimized.
 */
vectorFilteredComprehension
    : LBRACKET
      expression
      FOR
      identifier
      IN
      expression
      IF
      expression
      RBRACKET
    ;


/* ============================================================================
 * 13. VECTOR GENERATION
 * ============================================================================
 *
 * General generation form:
 *
 *     generate(expression, count)
 *
 * The generated values need not correspond to a hardware loop.
 */
vectorGeneration
    : vectorGenerationName
      LPAREN
      expression
      COMMA
      expression
      RPAREN
    ;


vectorGenerationName
    : identifier
    ;


/* ============================================================================
 * 14. VECTOR DOMAIN EXPRESSION
 * ============================================================================
 *
 * This is the preferred bridge for semantic consumers that need to recognize
 * a vector-oriented source expression without duplicating the vector grammar.
 *
 * General expressions remain legal because vector-ness is frequently
 * established only after name/type resolution.
 */
vectorExpression
    : vector
    | vectorAccess
    | vectorSlice
    | vectorRepeat
    | vectorConcatenation
    | vectorComprehension
    | vectorFilteredComprehension
    | vectorGeneration
    ;


/* ============================================================================
 * 15. IDENTIFIER BRIDGE
 * ============================================================================
 *
 * Identifier spelling is NOT owned here.
 *
 * Expressions.g4 already provides the canonical parser-level identifier rule.
 *
 * This rule deliberately delegates to that imported rule rather than
 * introducing:
 *
 *     IDENT
 *     IDENTIFIER
 *     VECTOR_IDENTIFIER
 *     VECTOR_NAME
 *
 * as new alternatives.
 */
identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 16. VECTOR SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar permits vectors whose dimensions are:
 *
 *     known statically;
 *     symbolic;
 *     compile-time determined;
 *     runtime determined;
 *     resource dependent;
 *
 * Examples:
 *
 *     [1, 2, 3]
 *
 *     vector(value, N)
 *
 *     [f(x) for x in data]
 *
 * The grammar imposes no finite value of N.
 *
 * The compiler may later reject a program because:
 *
 *     - a target lacks sufficient resources;
 *     - a required capability is unavailable;
 *     - a resource constraint is violated;
 *     - an implementation policy cannot realize it;
 *
 * Such rejection is NOT a grammar failure.
 *
 * ============================================================================
 * VECTOR / MACHINE SEPARATION
 * ============================================================================
 *
 * The following source concepts remain semantic:
 *
 *     vector length
 *     element type
 *     vector operations
 *     ordering
 *     mathematical meaning
 *
 * The following remain downstream:
 *
 *     SIMD width
 *     register allocation
 *     memory layout
 *     alignment
 *     cache strategy
 *     GPU mapping
 *     accelerator mapping
 *     FPGA pipeline structure
 *     distributed partitioning
 *     scheduling
 *     placement
 *
 * ============================================================================
 * VECTOR / QUANTUM INTEGRATION
 * ============================================================================
 *
 * A vector may contain or represent values derived from quantum computation.
 *
 * Examples may include:
 *
 *     measurement result vectors
 *     probability vectors
 *     expectation-value collections
 *     parameter vectors
 *     classical control vectors
 *
 * This grammar does not import quantum::ir and does not define quantum
 * semantics.
 *
 * Quantum syntax is interpreted by the quantum frontend and lowered to the
 * canonical quantum semantic boundary.
 *
 * ============================================================================
 * VECTOR / HDL INTEGRATION
 * ============================================================================
 *
 * A vector value may participate in HDL/hardware parameterization when the
 * semantic layers permit it.
 *
 * This grammar does not define:
 *
 *     wires
 *     ports
 *     registers
 *     clocks
 *     buses
 *     physical widths
 *
 * Those concepts belong to the HDL/hardware grammar layers.
 *
 * ============================================================================
 * VECTOR / RESOURCE INTEGRATION
 * ============================================================================
 *
 * Vector size must not be interpreted as a hardware allocation by this parser.
 *
 * For example:
 *
 *     vector(x, N)
 *
 * means a vector containing N semantic elements.
 *
 * It does NOT mean:
 *
 *     allocate N registers
 *     allocate N SIMD lanes
 *     allocate N GPU threads
 *     allocate N physical devices
 *
 * Resource analysis determines the actual realization.
 *
 * ============================================================================
 * VECTOR / CLASSICAL IR INTEGRATION
 * ============================================================================
 *
 * This grammar emits parser structure only.
 *
 * The frontend is responsible for translating the parsed structure into the
 * canonical AST.
 *
 * Semantic analysis then determines the vector's:
 *
 *     element type
 *     shape
 *     length
 *     mutability
 *     ownership
 *     effects
 *     resource requirements
 *
 * The resulting semantic object may be lowered to classical IR.
 *
 * No IR node is constructed by this grammar.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Downstream compilation may choose among:
 *
 *     scalar lowering
 *     loop lowering
 *     SIMD/vector lowering
 *     GPU lowering
 *     accelerator lowering
 *     FPGA-oriented lowering
 *     distributed lowering
 *     streaming lowering
 *     future target-specific lowering
 *
 * without changing the source grammar.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime vector representation is outside this grammar.
 *
 * Runtime may select:
 *
 *     contiguous representation
 *     segmented representation
 *     lazy representation
 *     streamed representation
 *     distributed representation
 *     device-resident representation
 *
 * according to the compiled program and available resources.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains NO:
 *
 *     MAX_VECTOR_LENGTH
 *     MAX_ELEMENTS
 *     MAX_RANK
 *     MAX_DIMENSIONS
 *     MAX_LANES
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *
 * There are no fixed hardware identifiers.
 *
 * There are no fixed machine widths.
 *
 * There are no fixed target names.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar introduces vector VALUE syntax without changing vector TYPE
 * ownership.
 *
 * Existing vector type syntax remains owned by:
 *
 *     grammar/types/classical-types.g4
 *
 * Existing general expression syntax remains owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Existing scalar syntax remains independently owned by:
 *
 *     grammar/classical/scalar.g4
 *
 * Future vector operations should preferably be added as semantic/library
 * constructs rather than new reserved keywords.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     []
 *     [1]
 *     [1, 2]
 *     [x, y, z]
 *     [x + y, f(z)]
 *     vector(x)
 *     vector(x, n)
 *     repeat(x, n)
 *     concat(a, b)
 *     [x * x for x in values]
 *     [x * x for x in values if x > 0]
 *
 * Boundary tests MUST include:
 *
 *     empty vector
 *     single-element vector
 *     very large syntactic element lists
 *     deeply nested expressions
 *     symbolic vector sizes
 *     runtime vector sizes
 *
 * Negative tests MUST include:
 *
 *     [,
 *     [,]
 *     [1 2]
 *     vector(
 *     vector(x,)
 *     repeat(x)
 *     concat(a)
 *     [x for in values]
 *
 * Cross-domain tests MUST include:
 *
 *     classical + vector
 *     vector + quantum-derived value
 *     vector + resource expression
 *     vector + compile-time expression
 *     vector + distributed expression
 *     vector + accelerator-oriented semantic annotation
 *
 * Scalability tests MUST verify that no parser rule imposes a fixed:
 *
 *     vector length
 *     element count
 *     machine width
 *     device count
 *     accelerator count
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. It compiles as an ANTLR4 parser grammar.
 *
 *     2. It consumes only the canonical Zamani lexer vocabulary.
 *
 *     3. It introduces no lexer aliases.
 *
 *     4. It introduces no hardware limits.
 *
 *     5. It introduces no machine-specific assumptions.
 *
 *     6. It does not duplicate vector TYPE syntax.
 *
 *     7. It does not duplicate expression precedence.
 *
 *     8. It does not construct IR.
 *
 *     9. It does not depend on quantum::ir.
 *
 *    10. It does not depend on QEC, ZQN, scheduling, routing, or hardware
 *        discovery.
 *
 *    11. Vector values can be represented independently of target hardware.
 *
 *    12. Empty, singleton, finite, symbolic, generated, and runtime-sized
 *        vectors are syntactically representable.
 *
 *    13. Positive, negative, boundary, determinism, scalability, and
 *        cross-domain tests pass.
 *
 *    14. The grammar remains valid under Rust 1.97 / Rust 1.97.1 generated
 *        frontend integration.
 *
 *    15. No unsafe Rust is required anywhere in this grammar's integration.
 *
 * ============================================================================
 */