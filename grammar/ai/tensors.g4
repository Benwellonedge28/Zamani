/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/tensors.g4
 *
 * Role:
 *     Production AI/ML tensor-domain parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - Parser grammar only.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime execution.
 *     - No unsafe implementation.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for tensor-domain computation
 * within Zamani's AI / classical / accelerator programming model.
 *
 * Tensor syntax expresses mathematical and computational intent.
 *
 * It deliberately does NOT specify:
 *
 *     - tensor storage;
 *     - memory layout;
 *     - allocation strategy;
 *     - CPU selection;
 *     - GPU selection;
 *     - TPU/NPU selection;
 *     - FPGA selection;
 *     - accelerator selection;
 *     - SIMD width;
 *     - thread count;
 *     - device count;
 *     - cluster size;
 *     - distributed topology;
 *     - numerical kernel implementation;
 *     - BLAS implementation;
 *     - compiler optimization;
 *     - scheduling;
 *     - placement;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     canonical Types              canonical Expressions
 *          |                             |
 *          +--------------+--------------+
 *                         |
 *                         v
 *                     AITensors
 *                         |
 *                         v
 *                    Frontend AST
 *                         |
 *                         v
 *                 Semantic Analysis
 *                         |
 *             +-----------+-----------+
 *             |                       |
 *             v                       v
 *       Tensor semantic model   Resource metadata
 *             |                       |
 *             +-----------+-----------+
 *                         |
 *                         v
 *                   Canonical IR
 *                         |
 *             +-----------+-----------+
 *             |                       |
 *             v                       v
 *       Classical computation    Quantum computation
 *             |                       |
 *             +-----------+-----------+
 *                         |
 *                         v
 *                    Optimization
 *                         |
 *                         v
 *                     Scheduling
 *                         |
 *                         v
 *                  Target realization
 *                         |
 *                         v
 *                       Runtime
 *
 * AITensors MUST NOT construct IR directly.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - tensor declaration syntax;
 *     - tensor literal syntax;
 *     - tensor shape syntax;
 *     - tensor dimension syntax;
 *     - tensor axis syntax;
 *     - tensor indexing syntax;
 *     - tensor slicing syntax;
 *     - tensor transformation syntax;
 *     - tensor contraction syntax;
 *     - tensor reduction syntax;
 *     - tensor permutation syntax;
 *     - tensor reshape syntax;
 *     - tensor broadcast syntax;
 *     - tensor expression composition;
 *     - tensor-domain statement boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - general expressions;
 *     - general statements;
 *     - canonical type definitions;
 *     - generic type declarations;
 *     - memory allocation;
 *     - ownership;
 *     - borrowing;
 *     - numerical algorithms;
 *     - automatic differentiation;
 *     - model architecture;
 *     - training;
 *     - inference;
 *     - datasets;
 *     - accelerator implementation;
 *     - CPU/GPU/NPU/TPU selection;
 *     - hardware discovery;
 *     - resource allocation;
 *     - placement;
 *     - scheduling;
 *     - optimization;
 *     - canonical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Tensor source syntax describes WHAT computation means.
 *
 * It MUST NOT silently encode WHERE or HOW it executes.
 *
 * Valid examples include:
 *
 *     @tensor x: Tensor<T, Shape>;
 *
 *     @tensor weights: Tensor<f32, [Rows, Columns]>;
 *
 *     y = reshape(x, [Batch, Features]);
 *
 *     z = contract(a, b, axes);
 *
 *     result = broadcast(x, targetShape);
 *
 * The syntax does not imply:
 *
 *     - a particular machine;
 *     - a particular accelerator;
 *     - a particular memory capacity;
 *     - a particular SIMD width;
 *     - a particular number of threads;
 *     - a particular number of devices;
 *     - a particular distributed topology.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite machine-oriented limits are encoded here.
 *
 * In particular, this grammar contains no:
 *
 *     MAX_RANK
 *     MAX_DIMENSIONS
 *     MAX_ELEMENTS
 *     MAX_TENSOR_SIZE
 *     MAX_AXES
 *     MAX_BATCH_SIZE
 *     MAX_FEATURES
 *     MAX_CHANNELS
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_THREADS
 *     MAX_NODES
 *
 * Tensor structures are recursive or repeated structurally.
 *
 * Therefore the grammar does not impose a semantic ceiling on:
 *
 *     - tensor rank;
 *     - tensor dimensions;
 *     - number of axes;
 *     - number of tensor declarations;
 *     - number of tensor operations;
 *     - model size;
 *     - data size;
 *     - batch size;
 *     - sequence length;
 *     - distributed scale.
 *
 * Actual limits are governed by:
 *
 *     - parser resource policy;
 *     - semantic validation;
 *     - compiler policy;
 *     - available memory;
 *     - available compute;
 *     - resource management;
 *     - scheduling;
 *     - deployment;
 *     - runtime capabilities.
 *
 * Such limits MUST NOT become grammar constants.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax belongs to the canonical Zamani type grammar.
 *
 * This grammar therefore reuses:
 *
 *     typeExpression
 *
 * rather than defining a second tensor type system.
 *
 * Tensor semantic types may ultimately be represented by existing or future
 * canonical types such as:
 *
 *     Tensor<T>
 *     Tensor<T, Shape>
 *     Tensor<T, D0, D1, ...>
 *
 * The exact semantic validity is determined downstream.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions belong to the canonical expression grammar.
 *
 * This file therefore reuses:
 *
 *     expression
 *
 * for:
 *
 *     - symbolic dimensions;
 *     - axis expressions;
 *     - indices;
 *     - slice bounds;
 *     - tensor values;
 *     - constructor arguments;
 *     - compile-time values;
 *     - runtime values where permitted by semantic analysis.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No lexer rules are declared here.
 *
 * Tensor-domain vocabulary such as:
 *
 *     Tensor
 *     Shape
 *     reshape
 *     broadcast
 *     contract
 *     transpose
 *     reduce
 *     einsum
 *     concatenate
 *     stack
 *     split
 *
 * remains semantically resolvable vocabulary rather than being made into
 * new lexer rules by this file.
 *
 * This preserves:
 *
 *     - library extensibility;
 *     - dialect extensibility;
 *     - vendor independence;
 *     - future compatibility;
 *     - user-defined tensor operations.
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar imports the canonical type and expression grammars.
 *
 * It exposes:
 *
 *     tensorConstruct
 *
 * as the public integration boundary.
 *
 * The AI orchestrator may therefore delegate tensor syntax through:
 *
 *     aiTensorBoundary
 *
 * without reproducing tensor productions.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar does NOT validate:
 *
 *     - shape compatibility;
 *     - rank compatibility;
 *     - dimension positivity;
 *     - broadcast compatibility;
 *     - contraction compatibility;
 *     - axis uniqueness;
 *     - axis range;
 *     - dtype compatibility;
 *     - storage compatibility;
 *     - layout compatibility;
 *     - numerical precision;
 *     - overflow;
 *     - memory availability;
 *     - accelerator availability.
 *
 * Semantic analysis owns those checks.
 *
 * ============================================================================
 *
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Tensor values may participate in:
 *
 *     classical computation
 *     AI/ML
 *     quantum-classical computation
 *     hardware acceleration
 *     distributed computation
 *     scientific computing
 *     future computational domains
 *
 * This grammar does not create separate tensor representations for each
 * domain.
 *
 * A tensor is a semantic value whose realization is selected downstream.
 *
 * ============================================================================
 *
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Tensor syntax may describe mathematical objects used by quantum algorithms,
 * simulation, optimization, or hybrid programs.
 *
 * It does NOT define:
 *
 *     - quantum states;
 *     - physical qubits;
 *     - logical qubits;
 *     - QPU topology;
 *     - gate implementations;
 *     - quantum noise;
 *     - QEC;
 *     - ZQN;
 *     - quantum scheduling.
 *
 * Those belong to the quantum architecture and canonical quantum IR.
 *
 * ============================================================================
 *
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. It parses all tensor-domain constructs it owns.
 *     2. It contains no lexer rules.
 *     3. It contains no Rust actions.
 *     4. It contains no unsafe implementation.
 *     5. It contains no hardware-size constants.
 *     6. It delegates general types to typeExpression.
 *     7. It delegates general expressions to expression.
 *     8. It exposes tensorConstruct as its public boundary.
 *     9. It can be composed into AI.g4 without redefining tensor syntax.
 *    10. Its AST mapping can represent every production without losing source
 *        information.
 *    11. Semantic validation remains downstream.
 *    12. Positive, negative, boundary, scalability and cross-domain tests
 *        exist for the productions owned here.
 *
 * ============================================================================
 */

parser grammar AITensors;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/* ============================================================================
 * 1. PUBLIC TENSOR ENTRY POINT
 * ========================================================================== */

/**
 * Stable parser-facing tensor boundary.
 *
 * AI.g4 and other domain orchestrators should depend on this rule rather than
 * duplicating tensor productions.
 */
tensorConstruct
    : tensorDeclaration
    | tensorExpressionStatement
    | tensorAssignment
    | tensorIndexExpression
    | tensorSliceExpression
    | tensorTransformExpression
    | tensorReductionExpression
    | tensorContractionExpression
    | tensorShapeExpression
    | tensorAxisExpression
    ;


/* ============================================================================
 * 2. TENSOR DECLARATION
 * ========================================================================== */

/**
 * Tensor declarations are source-level declarations.
 *
 * The annotation establishes tensor-domain intent without introducing a new
 * reserved keyword.
 *
 * Examples:
 *
 *     @tensor x: Tensor<f32>;
 *     @tensor weights: Tensor<f32, Shape>;
 *     @tensor x: Tensor<T, [Batch, Features]> = value;
 *
 * Semantic analysis determines whether the type is actually a valid tensor
 * type.
 */
tensorDeclaration
    : tensorAnnotation
      identifier
      tensorTypeAnnotation?
      tensorInitializer?
      SEMICOLON
    ;


/**
 * Generic annotation boundary.
 *
 * The semantic layer MUST normalize and validate the annotation as `@tensor`.
 *
 * This avoids adding a new tensor-specific lexer keyword.
 */
tensorAnnotation
    : NANO_ANNOTATION
    ;


tensorTypeAnnotation
    : COLON
      typeExpression
    ;


tensorInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 3. TENSOR EXPRESSION STATEMENT
 * ========================================================================== */

/**
 * Tensor-domain expression statement.
 */
tensorExpressionStatement
    : tensorExpression
      SEMICOLON
    ;


/**
 * Tensor expression composition.
 *
 * The grammar intentionally does not impose a finite operation depth.
 */
tensorExpression
    : tensorPrimary
    | tensorLiteral
    | tensorConstruction
    | tensorIndexExpression
    | tensorSliceExpression
    | tensorTransformExpression
    | tensorReductionExpression
    | tensorContractionExpression
    | tensorShapeExpression
    | tensorBroadcastExpression
    | tensorElementwiseExpression
    | tensorExpressionInParentheses
    ;


/* ============================================================================
 * 4. TENSOR PRIMARY
 * ========================================================================== */

/**
 * Tensor values may be represented by ordinary expressions.
 *
 * This permits tensors to interoperate with the general Zamani language.
 */
tensorPrimary
    : identifier
    | INTEGER
    | FLOAT
    | STRING
    | QUANTUM_LITERAL
    ;


/* ============================================================================
 * 5. TENSOR CONSTRUCTION
 * ========================================================================== */

/**
 * Tensor constructors are identified semantically rather than through a
 * closed keyword list.
 *
 * Examples:
 *
 *     tensor(...)
 *     zeros(...)
 *     ones(...)
 *     full(...)
 *     random(...)
 *     range(...)
 *
 * Future libraries and dialects can provide additional constructors.
 */
tensorConstruction
    : identifier
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 6. TENSOR LITERALS
 * ========================================================================== */

/**
 * Recursive tensor literal.
 *
 * Examples:
 *
 *     [1, 2, 3]
 *
 *     [[1, 2], [3, 4]]
 *
 *     [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
 *
 * Arbitrary nesting is represented structurally.
 *
 * The grammar does not impose a rank limit.
 *
 * Semantic analysis determines:
 *
 *     - rectangularity;
 *     - element type;
 *     - rank;
 *     - dimensions;
 *     - shape consistency.
 */
tensorLiteral
    : LBRACKET
      tensorLiteralElements?
      RBRACKET
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


/* ============================================================================
 * 7. TENSOR ASSIGNMENT
 * ========================================================================== */

/**
 * Tensor assignment.
 *
 * Assignment semantics remain owned by the general language.
 *
 * This rule establishes only the tensor-domain composition boundary.
 */
tensorAssignment
    : tensorAssignableTarget
      ASSIGN
      tensorExpression
      SEMICOLON
    ;


tensorAssignableTarget
    : identifier
    | tensorIndexExpression
    | tensorSliceExpression
    ;


/* ============================================================================
 * 8. INDEXING
 * ========================================================================== */

/**
 * Rank-independent tensor indexing.
 *
 * Examples:
 *
 *     x[i]
 *     x[i, j]
 *     x[i, j, k]
 *     x[i, j, k, l]
 *
 * No maximum index count is encoded.
 */
tensorIndexExpression
    : tensorIndexBase
      LBRACKET
      tensorIndexList
      RBRACKET
    ;


tensorIndexBase
    : identifier
    | tensorExpressionInParentheses
    ;


tensorIndexList
    : tensorIndex
      (COMMA tensorIndex)*
      COMMA?
    ;


tensorIndex
    : expression
    | tensorSlice
    ;


/* ============================================================================
 * 9. SLICING
 * ========================================================================== */

/**
 * Tensor slicing.
 *
 * Examples:
 *
 *     x[start..end]
 *     x[start..]
 *     x[..end]
 *     x[..]
 *
 * Slice semantics are validated downstream.
 */
tensorSliceExpression
    : tensorIndexBase
      LBRACKET
      tensorSliceList
      RBRACKET
    ;


tensorSliceList
    : tensorSliceItem
      (COMMA tensorSliceItem)*
      COMMA?
    ;


tensorSliceItem
    : tensorSlice
    | expression
    ;


tensorSlice
    : expression
      RANGE_OPERATOR
      expression
    | expression
      RANGE_OPERATOR
    | RANGE_OPERATOR
      expression
    | RANGE_OPERATOR
    ;


/**
 * Range operator is deliberately represented from canonical lexer tokens.
 *
 * The lexer owns DOT; this grammar composes the range operator structurally.
 */
RANGE_OPERATOR
    : DOT DOT
    ;


/* ============================================================================
 * 10. TRANSFORMATIONS
 * ========================================================================== */

/**
 * General tensor transformation boundary.
 *
 * Operation identity remains semantic.
 *
 * Examples:
 *
 *     reshape(x, shape)
 *     transpose(x, axes)
 *     permute(x, axes)
 *     flatten(x)
 *     squeeze(x)
 *     unsqueeze(x)
 *     expand(x, shape)
 *     broadcast(x, shape)
 */
tensorTransformExpression
    : tensorTransformName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorTransformName
    : identifier
    ;


/* ============================================================================
 * 11. BROADCAST
 * ========================================================================== */

/**
 * Explicit broadcast composition.
 *
 * Example:
 *
 *     broadcast(x, [Batch, Features, Channels])
 *
 * Compatibility is semantic, not grammatical.
 */
tensorBroadcastExpression
    : broadcastName
      LPAREN
      expression
      COMMA
      tensorShapeExpression
      RPAREN
    ;


broadcastName
    : identifier
    ;


/* ============================================================================
 * 12. REDUCTIONS
 * ========================================================================== */

/**
 * Tensor reduction.
 *
 * Examples:
 *
 *     reduce(x)
 *     reduce(x, axis)
 *     reduce(x, axes)
 *     reduce(x, axis, initial)
 *
 * The actual reduction operation is semantic.
 */
tensorReductionExpression
    : tensorReductionName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorReductionName
    : identifier
    ;


/* ============================================================================
 * 13. CONTRACTION
 * ========================================================================== */

/**
 * General tensor contraction.
 *
 * Examples:
 *
 *     contract(a, b)
 *
 *     contract(a, b, axes)
 *
 *     einsum(a, b, specification)
 *
 * The grammar preserves arguments; semantic analysis validates contraction
 * compatibility.
 */
tensorContractionExpression
    : tensorContractionName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorContractionName
    : identifier
    ;


/* ============================================================================
 * 14. ELEMENT-WISE OPERATIONS
 * ========================================================================== */

/**
 * Element-wise tensor expression.
 *
 * Examples:
 *
 *     a + b
 *     a - b
 *     a * b
 *     a / b
 *
 * Operator meaning and broadcasting rules are semantic.
 */
tensorElementwiseExpression
    : tensorExpression
      tensorElementwiseOperator
      tensorExpression
    ;


tensorElementwiseOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    ;


/* ============================================================================
 * 15. SHAPE
 * ========================================================================== */

/**
 * Tensor shape expression.
 *
 * Examples:
 *
 *     [N]
 *     [N, M]
 *     [Batch, Height, Width, Channels]
 *     [N * M, K]
 *
 * Shape expressions are symbolic and may remain unresolved until semantic
 * analysis or compilation.
 */
tensorShapeExpression
    : LBRACKET
      tensorDimensionList?
      RBRACKET
    ;


tensorDimensionList
    : tensorDimension
      (COMMA tensorDimension)*
      COMMA?
    ;


tensorDimension
    : expression
    ;


/* ============================================================================
 * 16. AXES
 * ========================================================================== */

/**
 * Tensor axis expressions.
 *
 * Examples:
 *
 *     axis
 *     [axis0, axis1]
 *     [0, 2, 4]
 *
 * There is no fixed axis count.
 */
tensorAxisExpression
    : tensorAxisList
    ;


tensorAxisList
    : LBRACKET
      tensorAxisItem
      (COMMA tensorAxisItem)*
      COMMA?
      RBRACKET
    ;


tensorAxisItem
    : expression
    ;


/* ============================================================================
 * 17. TENSOR EXPRESSIONS IN PARENTHESES
 * ========================================================================== */

/**
 * Parenthesized tensor expression.
 */
tensorExpressionInParentheses
    : LPAREN
      tensorExpression
      RPAREN
    ;


/* ============================================================================
 * 18. TENSOR TYPE REFERENCE
 * ========================================================================== */

/**
 * Tensor type syntax is intentionally delegated to the canonical type grammar.
 *
 * This helper exists only as an integration boundary for semantic consumers
 * that need to distinguish a tensor-domain type position.
 *
 * It MUST NOT become a second type system.
 */
tensorTypeReference
    : typeExpression
    ;


/* ============================================================================
 * 19. SHAPE-AWARE CONSTRUCTION
 * ========================================================================== */

/**
 * Optional explicit tensor construction boundary.
 *
 * Examples:
 *
 *     tensor<T>(shape)
 *     tensor<T, Shape>(value)
 *
 * The parser preserves generic/type information through the canonical type
 * grammar where the composed parser exposes it.
 *
 * Semantic analysis determines whether a particular callable is a tensor
 * constructor.
 */
tensorTypedConstruction
    : typeExpression
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 20. PERMUTATION
 * ========================================================================== */

/**
 * Axis permutation.
 *
 * Example:
 *
 *     permute(x, [2, 0, 1])
 *
 * The permutation's validity is semantic.
 */
tensorPermutationExpression
    : permutationName
      LPAREN
      expression
      COMMA
      tensorAxisExpression
      RPAREN
    ;


permutationName
    : identifier
    ;


/* ============================================================================
 * 21. RESHAPE
 * ========================================================================== */

/**
 * Reshape operation.
 *
 * Example:
 *
 *     reshape(x, [Batch, Features])
 *
 * The semantic layer determines whether the source and target shapes contain
 * compatible element counts.
 */
tensorReshapeExpression
    : reshapeName
      LPAREN
      expression
      COMMA
      tensorShapeExpression
      RPAREN
    ;


reshapeName
    : identifier
    ;


/* ============================================================================
 * 22. TRANSPOSE
 * ========================================================================== */

/**
 * Transpose operation.
 *
 * Example:
 *
 *     transpose(x)
 *
 *     transpose(x, [2, 0, 1])
 */
tensorTransposeExpression
    : transposeName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


transposeName
    : identifier
    ;


/* ============================================================================
 * 23. STACK / CONCATENATION
 * ========================================================================== */

/**
 * Structural tensor composition.
 *
 * Examples:
 *
 *     stack(values, axis)
 *     concatenate(values, axis)
 *
 * These remain identifier-resolved operations rather than lexer keywords.
 */
tensorCompositionExpression
    : tensorCompositionName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorCompositionName
    : identifier
    ;


/* ============================================================================
 * 24. SPLIT
 * ========================================================================== */

/**
 * Tensor splitting.
 *
 * Example:
 *
 *     split(x, sections, axis)
 *
 * The number of resulting values is not fixed by the grammar.
 */
tensorSplitExpression
    : tensorSplitName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorSplitName
    : identifier
    ;


/* ============================================================================
 * 25. DIAGONAL / TRACE / LINEAR ALGEBRA BOUNDARY
 * ========================================================================== */

/**
 * Tensor-compatible mathematical operations remain semantic identifier calls.
 *
 * This allows:
 *
 *     diagonal
 *     trace
 *     determinant
 *     norm
 *     solve
 *     inverse
 *
 * and future operations without changing the lexer.
 */
tensorMathExpression
    : tensorMathName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorMathName
    : identifier
    ;


/* ============================================================================
 * 26. SHAPE QUERY
 * ========================================================================== */

/**
 * Shape/rank/axis queries are semantic calls.
 *
 * Examples:
 *
 *     shape(x)
 *     rank(x)
 *     dimensions(x)
 *
 * No fixed rank is encoded.
 */
tensorShapeQueryExpression
    : tensorShapeQueryName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorShapeQueryName
    : identifier
    ;


/* ============================================================================
 * 27. DTYPE / PRECISION QUERY
 * ========================================================================== */

/**
 * Tensor element-type queries.
 *
 * Examples:
 *
 *     dtype(x)
 *     precision(x)
 *
 * Precision semantics belong to the type system and compiler.
 */
tensorTypeQueryExpression
    : tensorTypeQueryName
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorTypeQueryName
    : identifier
    ;


/* ============================================================================
 * 28. DEVICE-NEUTRAL TENSOR RESOURCE EXPRESSION
 * ========================================================================== */

/**
 * Tensor operations may carry semantic resource metadata through ordinary
 * expressions and the resource/capability system.
 *
 * This grammar does NOT provide:
 *
 *     gpu(0)
 *     cuda_device(3)
 *     tensor_cores(8)
 *     memory(80GB)
 *
 * as implicit tensor semantics.
 *
 * Such requirements belong to the resource/capability grammar.
 *
 * This rule therefore exists only as an explicit semantic boundary.
 */
tensorResourceExpression
    : identifier
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 29. SYMBOLIC DIMENSIONS
 * ========================================================================== */

/**
 * Symbolic dimensions are ordinary expressions.
 *
 * Examples:
 *
 *     N
 *     M
 *     Batch
 *     SequenceLength
 *     N * M
 *     2 * Channels
 *
 * The grammar does not evaluate these expressions.
 */
tensorSymbolicDimension
    : expression
    ;


/* ============================================================================
 * 30. DYNAMIC SHAPE
 * ========================================================================== */

/**
 * Dynamic shape is represented structurally.
 *
 * A dynamic dimension may be represented by an ordinary symbolic expression
 * resolved downstream.
 *
 * No special maximum or sentinel value is required.
 */
tensorDynamicShape
    : tensorShapeExpression
    ;


/* ============================================================================
 * 31. TENSOR REGION
 * ========================================================================== */

/**
 * Tensor computation region.
 *
 * This provides a stable domain boundary without defining a second statement
 * language.
 */
tensorRegion
    : LBRACE
      tensorRegionMember*
      RBRACE
    ;


tensorRegionMember
    : tensorDeclaration
    | tensorExpressionStatement
    | tensorAssignment
    ;


/* ============================================================================
 * 32. TENSOR PROGRAM COMPOSITION
 * ========================================================================== */

/**
 * Multiple tensor operations compose structurally.
 *
 * There is no grammar-level operation-count limit.
 */
tensorProgram
    : tensorConstruct*
    ;


/* ============================================================================
 * 33. SEMANTICALLY NAMED OPERATION BOUNDARY
 * ========================================================================== */

/**
 * Generic tensor operation boundary.
 *
 * This is intentionally broad enough for future tensor algorithms while
 * preserving parser stability.
 */
tensorOperation
    : identifier
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 34. TENSOR VALUE BOUNDARY
 * ========================================================================== */

/**
 * Stable semantic boundary for consumers that only need a tensor value.
 */
tensorValue
    : tensorExpression
    ;


/* ============================================================================
 * 35. TENSOR DIMENSION BOUNDARY
 * ========================================================================== */

/**
 * Stable semantic boundary for consumers that need one symbolic dimension.
 */
tensorDimensionExpression
    : expression
    ;


/* ============================================================================
 * 36. TENSOR AXIS BOUNDARY
 * ========================================================================== */

/**
 * Stable semantic boundary for one or more tensor axes.
 */
tensorAxes
    : tensorAxisList
    ;


/* ============================================================================
 * 37. TENSOR SHAPE BOUNDARY
 * ========================================================================== */

/**
 * Stable semantic boundary for tensor shape.
 */
tensorShape
    : tensorShapeExpression
    ;


/* ============================================================================
 * 38. INTEGRATION NOTES
 * ========================================================================== */

/*
 * Integration with grammar/ai/ai.g4:
 *
 * The AI orchestrator should import this grammar and delegate its existing
 * aiTensorBoundary to tensorConstruct/tensorValue/tensorTypeReference as
 * appropriate.
 *
 * Conceptually:
 *
 *     aiTensorBoundary
 *         : tensorConstruct
 *         | tensorValue
 *         | tensorTypeReference
 *         ;
 *
 * The exact composition must be performed by the owning AI orchestrator,
 * not duplicated here.
 *
 *
 * Integration with grammar/ai/models.g4:
 *
 * Model inputs, outputs, parameters and states may use:
 *
 *     typeExpression
 *
 * and semantic analysis may resolve those types to tensor semantic types.
 *
 * models.g4 MUST NOT import tensor implementation semantics or duplicate
 * tensor shape/indexing rules.
 *
 *
 * Integration with grammar/types:
 *
 * Tensor types belong to the canonical type system.
 *
 * If Tensor<T, Shape> is not yet represented by the canonical type grammar,
 * that type must be added to the canonical type owner. This file MUST NOT
 * create a competing tensor type system merely to compensate.
 *
 *
 * Integration with grammar/classical:
 *
 * Tensor computation may consume or produce vectors, matrices, scalars and
 * other classical values.
 *
 * Tensor grammar owns tensor-specific syntax.
 *
 * Classical grammar owns the canonical classical type/value semantics.
 *
 *
 * Integration with grammar/resources:
 *
 * Tensor resource requirements must be expressed through the resource and
 * capability system.
 *
 * Tensor grammar must not encode device IDs, fixed accelerator counts,
 * memory sizes, topology, or hardware limits.
 *
 *
 * Integration with quantum:
 *
 * Tensor syntax may be used by hybrid quantum/classical programs.
 *
 * It does not replace quantum::ir.
 *
 * Quantum semantic lowering remains the responsibility of the quantum
 * frontend/IR pipeline.
 *
 *
 * Integration with optimization:
 *
 * Tensor operations are optimization inputs, not optimization instructions.
 *
 *
 * Integration with scheduling:
 *
 * Tensor execution ordering and placement are scheduling concerns.
 *
 *
 * Integration with hardware:
 *
 * Tensor hardware realization is target-specific and belongs downstream.
 *
 *
 * Integration with runtime:
 *
 * This grammar never executes tensor operations.
 *
 *
 * Integration with Rust:
 *
 * Rust 1.97 / 1.97.1 consumers must use safe Rust.
 *
 * This grammar itself contains no Rust code and therefore contains no unsafe
 * implementation.
 */


/* ============================================================================
 * 39. NON-OWNERSHIP GUARANTEE
 * ========================================================================== */

/*
 * A future change to:
 *
 *     CPU count
 *     GPU count
 *     accelerator count
 *     memory capacity
 *     device topology
 *     cluster size
 *     tensor engine count
 *     SIMD width
 *     thread count
 *     quantum device size
 *
 * MUST NOT require changes to this grammar merely because the available
 * machine changed.
 *
 * A change is required only when Zamani's LANGUAGE SEMANTICS themselves
 * acquire a genuinely new tensor syntax feature.
 */


/* ============================================================================
 * 40. TEST CONTRACT
 * ========================================================================== */

/*
 * The owning test suite must provide at least:
 *
 * POSITIVE:
 *
 *     @tensor x: Tensor<f32>;
 *
 *     @tensor x: Tensor<f32, [N, M]>;
 *
 *     @tensor x: Tensor<T, [Batch, Sequence, Features]>;
 *
 *     x = [[1, 2], [3, 4]];
 *
 *     x[i];
 *
 *     x[i, j];
 *
 *     x[i, j, k];
 *
 *     x[1..N, 0..M];
 *
 *     reshape(x, [Batch, Features]);
 *
 *     broadcast(x, [Batch, Height, Width, Channels]);
 *
 *     contract(a, b);
 *
 *     einsum(a, b, specification);
 *
 *     transpose(x);
 *
 *     transpose(x, [2, 0, 1]);
 *
 *     reduce(x, axis);
 *
 *     stack(values, axis);
 *
 *     concatenate(values, axis);
 *
 *     split(x, sections, axis);
 *
 *     tensor<N, M>(value);
 *
 *
 * NEGATIVE:
 *
 *     malformed tensor declaration;
 *     missing closing bracket;
 *     missing closing parenthesis;
 *     malformed shape list;
 *     malformed index list;
 *     malformed range;
 *     malformed tensor literal;
 *     missing assignment expression;
 *     invalid separator placement.
 *
 *
 * BOUNDARY:
 *
 *     one-dimensional tensor;
 *     zero/empty structural lists where syntax permits;
 *     deeply nested tensor literals;
 *     very large symbolic shape expressions;
 *     many tensor axes;
 *     many tensor dimensions;
 *     many tensor operations;
 *     deeply nested generic/type expressions.
 *
 *
 * SCALABILITY:
 *
 * The tests must prove there are no grammar constants imposing:
 *
 *     maximum tensor rank;
 *     maximum dimension count;
 *     maximum axis count;
 *     maximum tensor operation count;
 *     maximum model tensor count;
 *     maximum device count.
 *
 *
 * CROSS-DOMAIN:
 *
 *     classical + tensor;
 *     AI + tensor;
 *     quantum + tensor;
 *     quantum + classical + tensor;
 *     tensor + distributed;
 *     tensor + hardware;
 *     tensor + HDL;
 *     AI + quantum + tensor;
 *     AI + hardware + tensor.
 *
 *
 * DETERMINISM:
 *
 * The same source must produce the same parser structure for every invocation.
 *
 *
 * ROUND-TRIP:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> canonical representation
 *       -> source
 *
 * must preserve tensor semantic structure where the repository's AST/printer
 * supports round-trip serialization.
 */


/* ============================================================================
 * 41. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * This file deliberately contains no:
 *
 *     MAX_RANK
 *     MAX_DIMENSIONS
 *     MAX_AXES
 *     MAX_ELEMENTS
 *     MAX_BATCH
 *     MAX_FEATURES
 *     MAX_CHANNELS
 *     MAX_SEQUENCE
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_GPU
 *     MAX_NODES
 *
 * Any future proposed constant of this form requires architectural review.
 *
 * If a limit is necessary for:
 *
 *     parser protection;
 *     denial-of-service protection;
 *     compiler resource management;
 *     runtime resource management;
 *     deployment policy;
 *
 * it belongs to an explicit policy/configuration layer rather than the
 * language grammar.
 */


/* ============================================================================
 * 42. FINAL OWNERSHIP BOUNDARY
 * ========================================================================== */

/*
 * This file answers:
 *
 *     "How can tensor computation be expressed in Zamani source syntax?"
 *
 * It does NOT answer:
 *
 *     "How should a tensor execute?"
 *
 *     "Where should it execute?"
 *
 *     "Which device should execute it?"
 *
 *     "How should memory be allocated?"
 *
 *     "How should the operation be optimized?"
 *
 *     "How should the operation be scheduled?"
 *
 *     "How should it be lowered to a GPU/FPGA/ASIC?"
 *
 *     "How should it interact with quantum hardware?"
 *
 * Those questions belong to downstream semantic, IR, optimization,
 * scheduling, hardware and runtime subsystems.
 *
 * This separation is required for POCO-REAF.
 */