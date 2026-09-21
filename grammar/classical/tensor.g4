/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/tensor.g4
 *
 * Grammar:
 *     Tensor
 *
 * Status:
 *     CANONICAL CLASSICAL TENSOR-DOMAIN PARSER BOUNDARY
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL TENSOR-DOMAIN SYNTAX BOUNDARY.
 *
 * Tensor computation is part of the single Zamani language. It is not a
 * separate tensor language and it does not create a second expression,
 * identifier, type, indexing, assignment, or IR system.
 *
 * This grammar provides only syntax that is genuinely useful for identifying
 * tensor-domain constructs:
 *
 *     - tensor literals;
 *     - tensor-domain construction/invocation boundaries;
 *     - tensor-domain operation invocation boundaries;
 *     - tensor shape specification;
 *     - tensor dimension lists;
 *     - tensor-domain argument boundaries;
 *     - tensor-domain composition hooks.
 *
 * General language syntax remains owned by its canonical grammar modules.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     Expressions                       Types
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                    Tensor domain
 *                          |
 *                          v
 *                   Frontend AST
 *                          |
 *                          v
 *                 Semantic analysis
 *                          |
 *             +------------+-------------+
 *             |                          |
 *             v                          v
 *       Tensor semantics        resource/capability semantics
 *             |                          |
 *             +------------+-------------+
 *                          |
 *                          v
 *                 Canonical semantic model
 *                          |
 *                          v
 *                     Classical IR
 *                          |
 *                          v
 *                     Optimization
 *                          |
 *                 scheduling / lowering
 *                          |
 *                          v
 *                   Target realization
 *
 * This grammar MUST NOT construct IR.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - tensor literal structure;
 *     - tensor-domain construction syntax boundary;
 *     - tensor-domain operation invocation boundary;
 *     - tensor shape specification syntax;
 *     - tensor dimension-list syntax;
 *     - tensor-domain argument classification hooks.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier spelling;
 *     - keyword recognition;
 *     - general literals;
 *     - general expressions;
 *     - expression precedence;
 *     - arithmetic;
 *     - logical operators;
 *     - comparison operators;
 *     - assignment;
 *     - general function calls;
 *     - member access;
 *     - general indexing;
 *     - general slicing;
 *     - declarations;
 *     - statements;
 *     - blocks;
 *     - generic types;
 *     - tensor type semantics;
 *     - shape validation;
 *     - rank validation;
 *     - dimension compatibility;
 *     - broadcasting semantics;
 *     - tensor storage;
 *     - memory layout;
 *     - allocation;
 *     - vectorization;
 *     - accelerator selection;
 *     - GPU selection;
 *     - FPGA selection;
 *     - CPU selection;
 *     - distributed placement;
 *     - scheduling;
 *     - optimization;
 *     - automatic differentiation;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * AUTHORITY MODEL
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *
 * Identifier/name authority:
 *
 *     grammar/core/names.g4
 *
 * General expression authority:
 *
 *     grammar/expressions/expressions.g4
 *
 * Type authority:
 *
 *     grammar/types/
 *
 * Classical composition:
 *
 *     grammar/classical/classical.g4
 *
 * Universal parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/Zamani.g4
 *
 * AST authority:
 *
 *     src/frontend/ast/
 *
 * Semantic authority:
 *
 *     semantic analysis
 *
 * Canonical classical semantic/IR boundary:
 *
 *     existing classical semantic/IR infrastructure
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * Tensor.g4 MUST NOT become an additional authority for any of these layers.
 *
 * ============================================================================
 *
 * SINGLE-LANGUAGE RULE
 * ============================================================================
 *
 * Tensor syntax is Zamani syntax.
 *
 * There must not be:
 *
 *     tensor language
 *     matrix language
 *     vector language
 *
 * as separate programming languages.
 *
 * Tensor constructs participate in the same:
 *
 *     lexer
 *     parser
 *     AST
 *     semantic analysis
 *     type system
 *     effect system
 *     resource system
 *     capability system
 *     compiler
 *     runtime
 *
 * as every other Zamani domain.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Tensor source describes WHAT tensor computation means.
 *
 * It MUST NOT prescribe:
 *
 *     - which CPU;
 *     - which GPU;
 *     - which FPGA;
 *     - which accelerator;
 *     - which memory bank;
 *     - which SIMD width;
 *     - which register width;
 *     - which number of threads;
 *     - which number of devices;
 *     - which distributed topology;
 *     - which vendor library;
 *     - which BLAS implementation;
 *     - which tensor runtime.
 *
 * A source construct such as:
 *
 *     reshape(A, shape)
 *
 * expresses semantic intent.
 *
 * The implementation may realize it as:
 *
 *     - a view;
 *     - a copy;
 *     - a layout transformation;
 *     - a fused operation;
 *     - a distributed transformation;
 *     - an accelerator operation;
 *     - another semantics-preserving implementation.
 *
 * Such decisions belong downstream.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level finite limit on:
 *
 *     - tensor rank;
 *     - tensor dimensions;
 *     - tensor elements;
 *     - tensor operations;
 *     - tensor arguments;
 *     - nested tensor literals;
 *     - shape components;
 *     - operation depth;
 *     - tensor values;
 *     - tensor instances;
 *     - tensors in a program.
 *
 * In particular, this file MUST NOT introduce:
 *
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSIONS
 *     MAX_TENSOR_ELEMENTS
 *     MAX_TENSOR_SIZE
 *     MAX_SHAPE
 *     MAX_AXIS_COUNT
 *     MAX_TENSOR_OPERATIONS
 *     MAX_TENSOR_ARGUMENTS
 *
 * Structural repetition uses normal ANTLR repetition operators.
 *
 * Practical limits belong to:
 *
 *     - parser resource policies;
 *     - compiler resource policies;
 *     - memory availability;
 *     - runtime availability;
 *     - target capabilities;
 *     - deployment constraints.
 *
 * Those limits are not tensor-language semantics.
 *
 * ============================================================================
 *
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     CPU counts
 *     core counts
 *     thread counts
 *     GPU counts
 *     FPGA counts
 *     accelerator counts
 *     QPU counts
 *     memory capacities
 *     register widths
 *     SIMD widths
 *     vector widths
 *     machine topology
 *     device identifiers
 *     physical addresses
 *     vendor APIs
 *     tensor storage formats
 *     physical placement.
 *
 * It MUST NOT enumerate tensor algorithms as language keywords.
 *
 * Therefore names such as:
 *
 *     reshape
 *     transpose
 *     permute
 *     contract
 *     einsum
 *     reduce
 *     broadcast
 *     concatenate
 *     stack
 *     svd
 *     qr
 *     lu
 *     eigen
 *
 * remain names resolved by the semantic/library/dialect layers.
 *
 * The grammar does not need to change when a new tensor operation is added.
 *
 * ============================================================================
 *
 * EXPRESSION SEPARATION
 * ============================================================================
 *
 * General expressions are owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Therefore this file MUST NOT define another expression hierarchy.
 *
 * In particular, it MUST NOT define:
 *
 *     tensorExpression
 *         -> tensorExpression + tensorExpression
 *
 * or equivalent tensor-local arithmetic precedence.
 *
 * A tensor expression is an ordinary Zamani expression whose semantic type
 * or operation may be tensor-valued.
 *
 * Consequently:
 *
 *     A + B
 *     A * B
 *     f(A)
 *     A[i]
 *     A[i, j]
 *     A.member
 *
 * are parsed through the canonical expression system.
 *
 * Tensor semantics are determined later.
 *
 * ============================================================================
 *
 * TYPE SEPARATION
 * ============================================================================
 *
 * Tensor type syntax belongs to the canonical type grammar.
 *
 * Examples of semantic type forms may include:
 *
 *     Tensor<T>
 *     Tensor<T, Shape>
 *
 * but this file does not define those types.
 *
 * Consumers requiring a tensor type MUST use:
 *
 *     typeExpression
 *
 * and the canonical generic/type infrastructure.
 *
 * Shape compatibility, rank compatibility, element type compatibility and
 * dependent constraints are semantic concerns.
 *
 * ============================================================================
 *
 * INDEXING AND SLICING SEPARATION
 * ============================================================================
 *
 * General indexing and slicing belong to the canonical expression system.
 *
 * Therefore this file MUST NOT define:
 *
 *     tensorIndexExpression
 *     tensorSliceExpression
 *
 * as competing implementations of generic indexing.
 *
 * For example:
 *
 *     A[i]
 *     A[i, j]
 *     A[i, j, k]
 *
 * are ordinary expression/indexing syntax.
 *
 * Whether A is a tensor and whether the indexing is valid are semantic
 * questions.
 *
 * ============================================================================
 *
 * DECLARATION AND ASSIGNMENT SEPARATION
 * ============================================================================
 *
 * Declarations belong to:
 *
 *     grammar/declarations/
 *
 * Assignments belong to:
 *
 *     grammar/expressions/
 *
 * Statements belong to:
 *
 *     grammar/statements/
 *
 * Consequently this file MUST NOT define:
 *
 *     tensorDeclaration
 *     tensorAssignment
 *     tensorStatement
 *
 * as alternate declaration/assignment/statement authorities.
 *
 * Tensor values may occur in ordinary declarations:
 *
 *     let A: Tensor<f64> = ...
 *
 * through the canonical declaration and expression grammars.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * No lexer rules are declared here.
 *
 * No tensor-specific keyword tokens are created here.
 *
 * Canonical lexical tokens remain owned by the canonical lexer.
 *
 * This preserves:
 *
 *     one lexer
 *     one token vocabulary
 *     one identifier system
 *     one keyword authority.
 *
 * ============================================================================
 *
 * IDENTIFIER CONTRACT
 * ============================================================================
 *
 * This grammar MUST use the canonical:
 *
 *     identifier
 *     qualifiedName
 *
 * rules supplied by the core name grammar.
 *
 * It MUST NOT redefine:
 *
 *     IDENTIFIER
 *     identifier
 *     qualifiedName
 *     qualifiedIdentifier
 *
 * This is particularly important for extensible tensor operations.
 *
 * An operation name is source-level name data.
 *
 * Its semantic meaning is resolved after parsing.
 *
 * ============================================================================
 *
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `tensorConstruct` is retained as the stable tensor-domain entry point.
 *
 * It identifies constructs that are structurally tensor-specific.
 *
 * It intentionally does NOT attempt to parse every expression that might
 * eventually have tensor type.
 *
 * ============================================================================
 */

parser grammar Tensor;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC TENSOR CONSTRUCT
 * ============================================================================
 *
 * Stable tensor-domain composition boundary.
 *
 * Tensor consumers may use this rule without taking ownership of general
 * declarations, statements, expressions, or types.
 *
 * ============================================================================
 */

tensorConstruct
    : tensorLiteral
    | tensorConstruction
    | tensorOperationInvocation
    | tensorShapeSpecification
    ;


/*
 * ============================================================================
 * 2. TENSOR LITERAL
 * ============================================================================
 *
 * Tensor literals are recursively structured.
 *
 * Examples:
 *
 *     [1, 2, 3]
 *
 *     [[1, 2], [3, 4]]
 *
 *     [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
 *
 * Rank is determined by semantic analysis.
 *
 * This grammar does NOT impose a maximum rank.
 *
 * ============================================================================
 */

tensorLiteral
    : LBRACKET tensorLiteralElements? RBRACKET
    ;


tensorLiteralElements
    : tensorLiteralElement
      (COMMA tensorLiteralElement)*
      COMMA?
    ;


tensorLiteralElement
    : expression
    | tensorLiteral
    ;


/*
 * ============================================================================
 * 3. TENSOR CONSTRUCTION
 * ============================================================================
 *
 * Tensor construction is intentionally name-based.
 *
 * Examples may include:
 *
 *     tensor(...)
 *     zeros(...)
 *     ones(...)
 *     fill(...)
 *     random(...)
 *     identity(...)
 *
 * None of those names are reserved by this grammar.
 *
 * Semantic analysis determines whether the operation is:
 *
 *     - a canonical tensor constructor;
 *     - a user-defined function;
 *     - a library operation;
 *     - a dialect operation;
 *     - another domain-defined callable.
 *
 * ============================================================================
 */

tensorConstruction
    : qualifiedName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 4. TENSOR OPERATION INVOCATION
 * ============================================================================
 *
 * This is the principal open-world tensor extension point.
 *
 * Examples:
 *
 *     transpose(A)
 *     reshape(A, shape)
 *     contract(A, B)
 *     einsum(specification, A, B)
 *     custom_tensor_operation(A, B)
 *
 * Namespaces are supported through canonical qualified-name syntax:
 *
 *     linalg::transpose(A)
 *     tensor::contract(A, B)
 *     vendor::operation(A)
 *
 * The operation inventory is intentionally not enumerated.
 *
 * ============================================================================
 */

tensorOperationInvocation
    : qualifiedName
      LPAREN
      tensorOperationArgumentList?
      RPAREN
    ;


tensorOperationArgumentList
    : tensorOperationArgument
      (COMMA tensorOperationArgument)*
      COMMA?
    ;


tensorOperationArgument
    : tensorNamedArgument
    | expression
    ;


/*
 * ============================================================================
 * 5. TENSOR NAMED ARGUMENT
 * ============================================================================
 *
 * Named argument syntax is kept local only as an argument-classification
 * boundary.
 *
 * It does not redefine general assignment syntax.
 *
 * Example:
 *
 *     reshape(input: A, shape: S)
 *
 * Semantic analysis determines whether the called operation accepts these
 * names.
 *
 * ============================================================================
 */

tensorNamedArgument
    : identifier
      COLON
      expression
    ;


/*
 * ============================================================================
 * 6. TENSOR SHAPE SPECIFICATION
 * ============================================================================
 *
 * Shape is semantic information.
 *
 * This rule provides a structural tensor-domain boundary for a sequence of
 * dimension expressions.
 *
 * Examples:
 *
 *     <M, N>
 *     <Batch, Height, Width, Channels>
 *     <n, n>
 *     <n * m, k>
 *
 * Dimensions may be arbitrary expressions.
 *
 * No dimension count is hard-coded.
 *
 * ============================================================================
 */

tensorShapeSpecification
    : LESS_THAN
      tensorDimensionList?
      GREATER_THAN
    ;


tensorDimensionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. TENSOR AXIS LIST
 * ============================================================================
 *
 * Axis identity is semantic.
 *
 * It may be:
 *
 *     - positional;
 *     - symbolic;
 *     - computed;
 *     - represented by a compile-time value.
 *
 * No finite number of axes is encoded.
 *
 * ============================================================================
 */

tensorAxisList
    : tensorAxis
      (COMMA tensorAxis)*
      COMMA?
    ;


tensorAxis
    : expression
    ;


/*
 * ============================================================================
 * 8. TENSOR DOMAIN EXPRESSION HOOK
 * ============================================================================
 *
 * A tensor-valued computation is normally just an ordinary Zamani expression.
 *
 * This wrapper exists for domain-oriented consumers that need a stable tensor
 * semantic boundary without creating a second expression grammar.
 *
 * ============================================================================
 */

tensorDomainExpression
    : expression
    ;


/*
 * ============================================================================
 * 9. TENSOR DOMAIN VALUE
 * ============================================================================
 *
 * Tensor values remain ordinary expressions at the universal syntax level.
 *
 * Semantic analysis determines whether the expression has a tensor-valued
 * type.
 *
 * ============================================================================
 */

tensorValue
    : expression
    ;


/*
 * ============================================================================
 * 10. TENSOR SHAPE VALUE
 * ============================================================================
 *
 * A shape may itself be represented by an ordinary expression.
 *
 * This wrapper exists for semantic-domain classification only.
 *
 * ============================================================================
 */

tensorShapeValue
    : expression
    ;


/*
 * ============================================================================
 * 11. TENSOR AXIS VALUE
 * ============================================================================
 *
 * Axis expressions remain ordinary expressions.
 *
 * ============================================================================
 */

tensorAxisValue
    : expression
    ;


/*
 * ============================================================================
 * 12. TENSOR OPERATION NAME
 * ============================================================================
 *
 * Tensor operation names use the canonical name grammar.
 *
 * This rule deliberately does not introduce a tensor-specific identifier
 * system.
 *
 * ============================================================================
 */

tensorOperationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 13. TENSOR CONSTRUCTION NAME
 * ============================================================================
 *
 * Constructor names use exactly the same canonical name representation as
 * operation names.
 *
 * ============================================================================
 */

tensorConstructionName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 14. TENSOR TYPE ARGUMENT BOUNDARY
 * ============================================================================
 *
 * Tensor-specific semantic type parameters remain ordinary type expressions.
 *
 * This rule is a domain wrapper only.
 *
 * ============================================================================
 */

tensorTypeArgument
    : typeExpression
    ;


/*
 * ============================================================================
 * 15. TENSOR TYPE ARGUMENT LIST
 * ============================================================================
 *
 * There is no finite type-argument limit.
 *
 * The canonical type system remains authoritative.
 *
 * ============================================================================
 */

tensorTypeArgumentList
    : tensorTypeArgument
      (COMMA tensorTypeArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 16. TENSOR SEMANTIC OPERATION ARGUMENT
 * ============================================================================
 *
 * Domain consumers may use this boundary when translating tensor operation
 * arguments into the domain-neutral frontend AST.
 *
 * ============================================================================
 */

tensorSemanticArgument
    : tensorNamedArgument
    | expression
    ;


/*
 * ============================================================================
 * 17. TENSOR SEMANTIC ARGUMENT LIST
 * ============================================================================
 */

tensorSemanticArgumentList
    : tensorSemanticArgument
      (COMMA tensorSemanticArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 18. TENSOR DOMAIN CONSTRUCT LIST
 * ============================================================================
 *
 * Provides an unbounded composition point for tensor-domain tooling.
 *
 * This is not a replacement for the universal program grammar.
 *
 * ============================================================================
 */

tensorConstructList
    : tensorConstruct*
    ;


/*
 * ============================================================================
 * 19. SEMANTIC BOUNDARY
 * ============================================================================
 *
 * After parsing, semantic analysis is responsible for determining:
 *
 *     - whether a literal is a tensor;
 *     - tensor rank;
 *     - element type;
 *     - shape;
 *     - dimension compatibility;
 *     - broadcasting;
 *     - contraction compatibility;
 *     - reduction legality;
 *     - indexing legality;
 *     - slicing legality;
 *     - reshape legality;
 *     - permutation legality;
 *     - aliasing;
 *     - ownership;
 *     - effects;
 *     - resource requirements;
 *     - capability requirements;
 *     - numerical semantics.
 *
 * None of those decisions are made here.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * These grammar rules lower into the existing domain-neutral frontend AST.
 *
 * A tensor literal should preserve:
 *
 *     - element ordering;
 *     - nesting;
 *     - source spans;
 *     - literal/source representation;
 *     - expression structure.
 *
 * A tensor operation invocation should preserve:
 *
 *     - operation name;
 *     - namespace/qualification;
 *     - ordered operands/arguments;
 *     - named arguments;
 *     - source spans;
 *     - syntactic attributes available to the frontend.
 *
 * Conceptually:
 *
 *     tensorOperationInvocation
 *             |
 *             v
 *     generic AST Operation
 *             |
 *             v
 *     semantic tensor operation
 *
 * This file MUST NOT introduce a TensorOperation enum containing every
 * algorithm.
 *
 * ============================================================================
 *
 * IR CONTRACT
 * ============================================================================
 *
 * Tensor.g4 MUST NOT directly construct IR.
 *
 * The intended flow is:
 *
 *     tensor syntax
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     classical/data/tensor IR as defined by the compiler architecture
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling / placement / lowering
 *          |
 *          v
 *     target realization
 *
 * If tensor operations interact with quantum computation, the semantic layer
 * determines the appropriate cross-domain representation. Quantum semantics
 * ultimately use the established:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This file MUST NOT create another quantum IR merely because a tensor is used
 * by a quantum program.
 *
 * ============================================================================
 *
 * RESOURCE AND CAPABILITY CONTRACT
 * ============================================================================
 *
 * Tensor syntax does not select physical resources.
 *
 * Requirements such as:
 *
 *     memory requirements;
 *     accelerator capabilities;
 *     distributed execution;
 *     numerical precision;
 *     storage capabilities;
 *     communication requirements;
 *
 * belong to the canonical resource/capability/constraint system.
 *
 * Tensor syntax may therefore be compiled for:
 *
 *     embedded systems;
 *     scalar CPUs;
 *     multicore CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future architectures.
 *
 * The tensor grammar remains unchanged.
 *
 * ============================================================================
 *
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Tensor values may participate in:
 *
 *     classical computation;
 *     scientific computation;
 *     symbolic computation;
 *     AI/ML;
 *     data processing;
 *     distributed computation;
 *     networking;
 *     hardware/software co-design;
 *     hybrid quantum-classical computation.
 *
 * The domain boundaries are semantic.
 *
 * This file does not create separate tensor languages for each domain.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no environment inspection;
 *     - no randomness;
 *     - no runtime callbacks.
 *
 * Parsing therefore depends only on the source token stream and active
 * grammar/language version.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Parsing tensor syntax MUST have no side effects.
 *
 * Tensor operation names are data at parse time.
 *
 * The parser MUST NOT:
 *
 *     - load a tensor library;
 *     - execute a tensor operation;
 *     - access a device;
 *     - access credentials;
 *     - inspect hardware;
 *     - allocate tensor storage;
 *     - contact a network.
 *
 * ============================================================================
 *
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing public grammar name:
 *
 *     Tensor
 *
 * is retained.
 *
 * Existing public tensor-domain entry point:
 *
 *     tensorConstruct
 *
 * is retained.
 *
 * Existing canonical names remain available through:
 *
 *     identifier
 *     qualifiedName
 *
 * Existing universal expression syntax remains authoritative.
 *
 * Existing universal type syntax remains authoritative.
 *
 * No tensor-specific lexer tokens are introduced.
 *
 * This allows existing consumers to migrate from the previous tensor grammar
 * without requiring a second language or token vocabulary.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSIONS
 *     MAX_TENSOR_ELEMENTS
 *     MAX_TENSOR_SIZE
 *     MAX_AXIS_COUNT
 *     MAX_TENSOR_ARGUMENTS
 *     MAX_GPU_COUNT
 *     MAX_CPU_COUNT
 *     MAX_THREAD_COUNT
 *     MAX_ACCELERATOR_COUNT
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *
 * No physical device identifier is encoded.
 *
 * No vendor operation list is encoded.
 *
 * No finite tensor rank is encoded.
 *
 * No finite tensor dimension is encoded.
 *
 * No finite tensor element count is encoded.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 *
 * Tensor literals:
 *
 *     [1]
 *     [1, 2, 3]
 *     [[1, 2], [3, 4]]
 *     [[[1], [2]], [[3], [4]]]
 *
 * Tensor construction:
 *
 *     tensor(shape)
 *     zeros(shape)
 *     ones(shape)
 *     fill(shape, value)
 *
 * Qualified operations:
 *
 *     tensor::reshape(A, shape)
 *     linalg::transpose(A)
 *     linalg::contract(A, B)
 *
 * Named arguments:
 *
 *     reshape(input: A, shape: S)
 *
 * Symbolic dimensions:
 *
 *     <N>
 *     <M, N>
 *     <Batch, Height, Width, Channels>
 *
 * GENERAL EXPRESSION INTEGRATION
 *
 *     A + B
 *     A * B
 *     f(A)
 *     A[i]
 *     A[i, j]
 *
 * These MUST be handled by the canonical expression grammar rather than by
 * tensor-specific expression rules.
 *
 * NEGATIVE TESTS
 *
 * Include:
 *
 *     [
 *     ]
 *     [1,
 *     [[1, 2],]
 *     tensor(
 *     tensor(A,
 *     operation(, A)
 *     operation(A,)
 *
 * and malformed shape specifications such as:
 *
 *     <>
 *     <,>
 *     <N,>
 *
 * where the surrounding language specification disallows an empty or
 * incomplete dimension.
 *
 * SEMANTIC NEGATIVE TESTS
 *
 * These are NOT parser errors when syntactically valid:
 *
 *     reshape(A, incompatible_shape)
 *     contract(A, B)
 *     A[i, j, k]
 *
 * Their legality is determined by semantic analysis.
 *
 * BOUNDARY TESTS
 *
 * Include:
 *
 *     scalar-like tensor
 *     vector-shaped tensor
 *     matrix-shaped tensor
 *     high-rank nested tensor
 *     symbolic dimensions
 *     computed dimensions
 *     very large generated tensor literals
 *     deeply nested tensor expressions
 *     large operation argument lists
 *
 * SCALABILITY TESTS
 *
 * Verify that the grammar imposes no artificial:
 *
 *     rank limit;
 *     dimension-count limit;
 *     element-count limit;
 *     argument-count limit;
 *     operation-count limit;
 *     device-count limit;
 *     machine-size limit.
 *
 * CROSS-DOMAIN TESTS
 *
 * Tensor syntax must be tested with:
 *
 *     classical expressions;
 *     generic types;
 *     functions;
 *     resources;
 *     capabilities;
 *     concurrency;
 *     distributed computation;
 *     AI/data computation;
 *     hybrid quantum-classical programs;
 *     hardware/software co-design.
 *
 * DETERMINISM TESTS
 *
 * Parsing identical token streams under identical grammar/language versions
 * must produce equivalent parse structures.
 *
 * ROUND-TRIP TESTS
 *
 * Where the frontend formatter/source-preservation architecture supports it,
 * tensor syntax must survive:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> source
 *
 * without changing semantic meaning.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * Tensor.g4 is complete when:
 *
 *     [ ] ANTLR accepts the grammar.
 *
 *     [ ] The canonical lexer vocabulary is consumed.
 *
 *     [ ] No lexer rules exist in this parser grammar.
 *
 *     [ ] No tensor-specific keyword tokens are introduced.
 *
 *     [ ] Canonical identifier/name rules are reused.
 *
 *     [ ] Canonical expression syntax is reused.
 *
 *     [ ] Canonical type syntax is reused.
 *
 *     [ ] General indexing/slicing remain owned by expressions.
 *
 *     [ ] Declarations remain owned by declarations.
 *
 *     [ ] Assignments remain owned by expressions/statements.
 *
 *     [ ] Tensor literals are structurally recursive.
 *
 *     [ ] Tensor operation names remain open-world identifiers.
 *
 *     [ ] Qualified tensor operation names are supported.
 *
 *     [ ] Tensor operation algorithms are not hard-coded.
 *
 *     [ ] Tensor shape dimensions are structurally represented.
 *
 *     [ ] Tensor rank is not hard-coded.
 *
 *     [ ] Tensor dimensions are not hard-coded.
 *
 *     [ ] Tensor element counts are not hard-coded.
 *
 *     [ ] Hardware resources are not hard-coded.
 *
 *     [ ] Device placement is not encoded.
 *
 *     [ ] Vendor implementations are not encoded.
 *
 *     [ ] Semantic validation remains downstream.
 *
 *     [ ] AST integration is defined in advance.
 *
 *     [ ] Classical IR integration is defined in advance.
 *
 *     [ ] Cross-domain integration is defined in advance.
 *
 *     [ ] quantum::ir remains the canonical quantum semantic boundary.
 *
 *     [ ] QEC remains outside the grammar.
 *
 *     [ ] ZQN remains outside the grammar.
 *
 *     [ ] Scheduling remains outside the grammar.
 *
 *     [ ] HAL remains outside the grammar.
 *
 *     [ ] Runtime execution remains outside the grammar.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Rust integration remains compatible with Rust 1.97 / 1.97.1.
 *
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */