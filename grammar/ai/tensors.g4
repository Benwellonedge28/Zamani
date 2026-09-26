/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/tensors.g4
 *
 * Grammar:
 *     AITensors
 *
 * Status:
 *     CANONICAL AI / TENSOR LEAF PARSER GRAMMAR
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This is a pure ANTLR parser grammar.
 *
 *     - no embedded Rust actions
 *     - no semantic predicates
 *     - no unsafe code
 *     - no filesystem access
 *     - no network access
 *     - no environment inspection
 *     - no hardware discovery
 *     - no runtime execution
 *     - no target-specific implementation
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns source-level tensor syntax used by:
 *
 *     - AI / ML
 *     - classical numerical computing
 *     - scientific computing
 *     - data processing
 *     - accelerator-oriented computation
 *     - hybrid classical/quantum programs
 *     - future computational domains
 *
 * Tensor syntax describes COMPUTATIONAL STRUCTURE and INTENT.
 *
 * It does not describe the physical machine that realizes that computation.
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
 *     ZamaniParser
 *          |
 *          v
 *     AI / tensor composition
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+----------------------+
 *     |                           |
 *     v                           v
 * tensor semantics       resource/capability semantics
 *     |                           |
 *     +-------------+-------------+
 *                   |
 *                   v
 *             canonical semantic IR
 *                   |
 *          +--------+---------+
 *          |                  |
 *          v                  v
 *      classical          quantum::ir
 *          |                  |
 *          +--------+---------+
 *                   |
 *                   v
 *              optimization
 *                   |
 *                   v
 *              scheduling
 *                   |
 *                   v
 *             target lowering
 *                   |
 *                   v
 *                runtime
 *
 * This grammar MUST NOT bypass the AST/semantic/IR boundaries.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - tensor-domain declarations;
 *     - tensor-domain computation regions;
 *     - tensor literals;
 *     - tensor shapes;
 *     - tensor dimensions;
 *     - tensor axes;
 *     - tensor indexing;
 *     - tensor slicing;
 *     - tensor transformation syntax;
 *     - tensor reduction syntax;
 *     - tensor contraction syntax;
 *     - tensor broadcasting syntax;
 *     - tensor composition syntax;
 *     - tensor operation syntax;
 *     - tensor-specific structural boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - general expressions;
 *     - general statements;
 *     - general types;
 *     - generic type arguments;
 *     - ownership;
 *     - borrowing;
 *     - memory allocation;
 *     - device selection;
 *     - hardware discovery;
 *     - scheduling;
 *     - optimization;
 *     - compiler implementation;
 *     - runtime implementation;
 *     - AI model semantics;
 *     - datasets;
 *     - training;
 *     - inference;
 *     - automatic differentiation;
 *     - quantum semantics;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - calibration;
 *     - HAL.
 *
 * ============================================================================
 * SINGLE SOURCE OF TYPE TRUTH
 * ============================================================================
 *
 * General type syntax is owned by:
 *
 *     grammar/types/
 *
 * This grammar therefore consumes:
 *
 *     typeExpression
 *
 * rather than defining another Tensor type grammar.
 *
 * Examples of semantic types that may be represented by the canonical type
 * system include:
 *
 *     Tensor<T>
 *     Tensor<T, Shape>
 *     Tensor<T, D0, D1, ...>
 *
 * The semantic type system determines whether a particular type expression
 * denotes a tensor.
 *
 * This grammar does NOT declare a competing:
 *
 *     tensorType
 *
 * hierarchy.
 *
 * ============================================================================
 * SINGLE SOURCE OF EXPRESSION TRUTH
 * ============================================================================
 *
 * General expressions are owned by:
 *
 *     grammar/expressions/
 *
 * This grammar therefore consumes:
 *
 *     expression
 *
 * for:
 *
 *     - dimensions;
 *     - axis values;
 *     - index expressions;
 *     - slice bounds;
 *     - tensor operation arguments;
 *     - symbolic shapes;
 *     - initializers;
 *     - compile-time values;
 *     - runtime values.
 *
 * Tensor syntax must not recreate arithmetic, logical, comparison, call,
 * conditional, lambda, or other universal expression precedence here.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The sole lexer authority remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Tensor grammar does NOT declare lexer rules.
 *
 * Tensor concepts such as:
 *
 *     tensor
 *     reshape
 *     transpose
 *     broadcast
 *     reduce
 *     contract
 *     einsum
 *
 * are intentionally represented by identifiers rather than a closed keyword
 * vocabulary.
 *
 * This allows:
 *
 *     library operations
 *     user-defined operations
 *     dialect operations
 *     future tensor algorithms
 *     vendor-neutral extensions
 *
 * without continuously modifying the lexer.
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * Zamani must not enumerate every tensor algorithm in its grammar.
 *
 * Therefore this grammar does NOT define a closed list such as:
 *
 *     reshape
 *     transpose
 *     matmul
 *     convolution
 *     fft
 *     svd
 *     ...
 *
 * as mandatory language keywords.
 *
 * Instead, operation identity is represented structurally:
 *
 *     identifier(...)
 *
 * and semantic analysis resolves whether that operation is:
 *
 *     - a tensor intrinsic;
 *     - a library operation;
 *     - a user-defined operation;
 *     - a dialect operation;
 *     - a future extension;
 *     - an ordinary callable.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Tensor syntax must support:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Forever
 *
 * subject to the actual semantic capabilities and available resources.
 *
 * Tensor syntax MUST NOT encode:
 *
 *     MAX_TENSOR_RANK
 *     MAX_DIMENSIONS
 *     MAX_AXES
 *     MAX_ELEMENTS
 *     MAX_TENSOR_SIZE
 *     MAX_BATCH_SIZE
 *     MAX_FEATURES
 *     MAX_CHANNELS
 *     MAX_SEQUENCE_LENGTH
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_NODES
 *
 * or equivalent universal limits.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * All structurally repeatable tensor constructs use:
 *
 *     *
 *     +
 *
 * rather than a finite alternative list.
 *
 * Tensor rank is therefore not encoded as:
 *
 *     rank1
 *     rank2
 *     rank3
 *     ...
 *
 * Likewise, axis count, dimension count, argument count, tensor-operation
 * count, tensor nesting depth, and declaration count are not assigned
 * language-level maxima.
 *
 * Practical limits may exist in:
 *
 *     - parser resource policies;
 *     - compiler resource policies;
 *     - semantic validation;
 *     - available memory;
 *     - target capabilities;
 *     - runtime resources;
 *     - deployment policy.
 *
 * Such limits are not tensor-language semantics.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * This grammar does not select hardware.
 *
 * Source may express resource/capability intent through the canonical
 * resource grammar.
 *
 * Examples of semantic intent:
 *
 *     requires memory >= required_memory
 *     requires capability("tensor.compute")
 *     prefers capability("accelerated.tensor.compute")
 *
 * This file must never introduce:
 *
 *     gpu(0)
 *     device(3)
 *     cuda_device(0)
 *     tensor_core(8)
 *     memory(24GB)
 *
 * as intrinsic tensor realization syntax.
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Tensor syntax may participate in quantum algorithms, simulation,
 * optimization, hybrid computation, and quantum-classical workflows.
 *
 * This grammar does NOT define:
 *
 *     - qubits;
 *     - quantum gates;
 *     - physical qubits;
 *     - quantum topology;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - calibration.
 *
 * Quantum semantics continue through:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Tensor expressions may be consumed by hardware/software co-design.
 *
 * This grammar does not define:
 *
 *     - register width;
 *     - bus width;
 *     - memory-bank count;
 *     - accelerator count;
 *     - SIMD width;
 *     - pipeline depth;
 *     - physical topology.
 *
 * Such properties are target/resource semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source text;
 *     - canonical lexer;
 *     - parser grammar;
 *     - selected language/dialect configuration.
 *
 * Parsing must not depend on:
 *
 *     - hardware;
 *     - available accelerators;
 *     - memory capacity;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - randomness;
 *     - environment variables;
 *     - runtime state.
 *
 * ============================================================================
 * PUBLIC COMPOSITION BOUNDARY
 * ============================================================================
 *
 * The canonical public tensor entry point is:
 *
 *     tensorConstruct
 *
 * AI.g4 and other approved domain composition grammars should consume this
 * boundary rather than reproducing tensor productions.
 *
 * ============================================================================
 */

parser grammar AITensors;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC TENSOR COMPOSITION
 * ============================================================================
 *
 * `tensorConstruct` is the only public leaf-domain composition boundary.
 *
 * The alternatives are intentionally structural and mutually distinguishable
 * where practical.
 *
 * General expressions are NOT included as a catch-all alternative.
 *
 * This prevents every ordinary Zamani expression from being reclassified as
 * a tensor construct.
 * ============================================================================
 */

tensorConstruct
    : tensorDeclaration
    | tensorRegion
    | tensorExpressionStatement
    | tensorAssignment
    ;


/*
 * ============================================================================
 * 2. TENSOR DECLARATION
 * ============================================================================
 *
 * Tensor declarations are annotation-led so tensor-domain declarations are
 * distinguishable from ordinary declarations without adding a new lexer
 * keyword.
 *
 * The canonical lexer supplies NANO_ANNOTATION.
 *
 * Semantic analysis must validate that the annotation denotes the tensor
 * domain. The grammar intentionally does not hard-code an ever-growing list
 * of annotation names.
 *
 * Examples:
 *
 *     @tensor x: Tensor<f32>;
 *
 *     @tensor weights: Tensor<f32, [Rows, Columns]>;
 *
 *     @tensor x: Tensor<T, [Batch, Features]> = value;
 *
 * ============================================================================
 */

tensorDeclaration
    : NANO_ANNOTATION
      identifier
      tensorTypeClause?
      tensorInitializer?
      SEMICOLON
    ;


tensorTypeClause
    : COLON
      typeExpression
    ;


tensorInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 3. TENSOR REGION
 * ============================================================================
 *
 * A tensor region is a tensor-domain structural boundary.
 *
 * It reuses ordinary statement syntax rather than defining another statement
 * language.
 *
 * This permits tensor computation to compose with:
 *
 *     bindings
 *     control flow
 *     function calls
 *     concurrency
 *     effects
 *     resource contracts
 *     classical computation
 *     hybrid computation
 *
 * ============================================================================
 */

tensorRegion
    : LBRACE
      statement*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. TENSOR EXPRESSION STATEMENT
 * ============================================================================
 *
 * Tensor-specific expressions can be explicitly terminated as statements.
 *
 * ============================================================================
 */

tensorExpressionStatement
    : tensorExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. TENSOR ASSIGNMENT
 * ============================================================================
 *
 * Assignment uses the canonical ASSIGN token.
 *
 * The target is deliberately restricted to tensor-aware structural targets
 * instead of recreating universal assignment syntax.
 * ============================================================================
 */

tensorAssignment
    : tensorAssignableTarget
      ASSIGN
      tensorExpression
      SEMICOLON
    ;


tensorAssignableTarget
    : identifier
      tensorAccessSuffix*
    ;


/*
 * ============================================================================
 * 6. TENSOR EXPRESSION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This grammar does not recreate Zamani's complete expression precedence.
 *
 * Tensor expressions are formed from tensor-specific primaries and canonical
 * expression values.
 *
 * Arithmetic, logical, comparison, calls, conditionals, lambdas, etc. remain
 * owned by Expressions.
 *
 * ============================================================================
 */

tensorExpression
    : tensorPrimary
      tensorPostfix*
    ;


tensorPrimary
    : tensorLiteral
    | tensorOperationCall
    | tensorIdentifier
    | tensorParenthesized
    ;


tensorIdentifier
    : identifier
    ;


tensorParenthesized
    : LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 7. TENSOR POSTFIX
 * ============================================================================
 *
 * Postfix tensor operations are structural tensor access.
 *
 * No maximum number of postfix operations is encoded.
 * ============================================================================
 */

tensorPostfix
    : tensorAccessSuffix
    | tensorSliceSuffix
    ;


tensorAccessSuffix
    : LBRACKET
      tensorIndexList
      RBRACKET
    ;


tensorSliceSuffix
    : LBRACKET
      tensorSliceList
      RBRACKET
    ;


/*
 * ============================================================================
 * 8. INDEXING
 * ============================================================================
 *
 * An index is an ordinary Zamani expression.
 *
 * Therefore indices may be:
 *
 *     i
 *     i + offset
 *     f(x)
 *     dimension
 *     symbolic expression
 *     runtime expression
 *
 * There is no fixed number of indices.
 * ============================================================================
 */

tensorIndexList
    : tensorIndex
      (COMMA tensorIndex)*
      COMMA?
    ;


tensorIndex
    : expression
    ;


/*
 * ============================================================================
 * 9. SLICING
 * ============================================================================
 *
 * A slice is deliberately expressed with DOT DOT directly.
 *
 * No RANGE_OPERATOR lexer/parser token is introduced here.
 *
 * This is important because AITensors is a parser grammar and therefore must
 * not introduce lexer rules.
 *
 * The exact DOT token is supplied by ZamaniLexer.
 *
 * ============================================================================
 */

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
    : tensorSliceBound?
      DOT
      DOT
      tensorSliceBound?
    ;


tensorSliceBound
    : expression
    ;


/*
 * ============================================================================
 * 10. TENSOR LITERALS
 * ============================================================================
 *
 * Tensor literals are recursively structural.
 *
 * Examples:
 *
 *     [1, 2, 3]
 *
 *     [[1, 2], [3, 4]]
 *
 *     [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
 *
 * There is no grammar-level rank limit.
 *
 * Semantic analysis is responsible for:
 *
 *     - rectangularity;
 *     - element type;
 *     - shape;
 *     - rank;
 *     - broadcasting;
 *     - numeric validity.
 *
 * ============================================================================
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
    : tensorLiteral
    | expression
    ;


/*
 * ============================================================================
 * 11. GENERIC TENSOR OPERATION CALL
 * ============================================================================
 *
 * This is the principal open-world tensor operation boundary.
 *
 * Examples:
 *
 *     reshape(x, [N, M])
 *     transpose(x)
 *     broadcast(x, shape)
 *     reduce(x, axis)
 *     contract(a, b)
 *     einsum(a, b, specification)
 *     matmul(a, b)
 *     convolution(input, kernel)
 *     fft(x)
 *     custom_tensor_operation(x)
 *
 * Operation names remain identifiers.
 *
 * Semantic analysis determines whether the call is tensor-domain specific.
 *
 * ============================================================================
 */

tensorOperationCall
    : tensorOperationName
      LPAREN
      tensorOperationArguments?
      RPAREN
    ;


tensorOperationName
    : identifier
    ;


tensorOperationArguments
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. SHAPE EXPRESSION
 * ============================================================================
 *
 * Shape expressions are structural lists of symbolic or concrete dimensions.
 *
 * Examples:
 *
 *     [N]
 *     [N, M]
 *     [Batch, Sequence, Features]
 *     [N * M, K]
 *     [dynamic_dimension]
 *
 * The grammar does not evaluate shape expressions.
 *
 * ============================================================================
 */

tensorShape
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


/*
 * ============================================================================
 * 13. AXIS EXPRESSION
 * ============================================================================
 *
 * Examples:
 *
 *     axis
 *     [axis0, axis1]
 *     [0, 2, 4]
 *
 * There is no maximum axis count.
 * ============================================================================
 */

tensorAxes
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


/*
 * ============================================================================
 * 14. TRANSFORMATION BOUNDARY
 * ============================================================================
 *
 * Transformation identity remains open-world.
 *
 * Examples:
 *
 *     reshape(x, shape)
 *     transpose(x, axes)
 *     permute(x, axes)
 *     flatten(x)
 *     squeeze(x)
 *     expand_dims(x, axis)
 *
 * These are all represented through the generic tensor operation boundary.
 *
 * This rule exists as a semantic naming boundary for tooling and AST
 * classification; it does not enumerate operation names.
 * ============================================================================
 */

tensorTransform
    : tensorOperationCall
    ;


/*
 * ============================================================================
 * 15. REDUCTION BOUNDARY
 * ============================================================================
 *
 * Examples:
 *
 *     reduce(x)
 *     reduce(x, axis)
 *     reduce(x, axes)
 *     reduce(x, axis, initial)
 *
 * Reduction identity is semantic.
 * ============================================================================
 */

tensorReduction
    : tensorOperationCall
    ;


/*
 * ============================================================================
 * 16. CONTRACTION BOUNDARY
 * ============================================================================
 *
 * Examples:
 *
 *     contract(a, b)
 *     contract(a, b, axes)
 *     einsum(a, b, specification)
 *
 * Semantic analysis validates contraction compatibility.
 * ============================================================================
 */

tensorContraction
    : tensorOperationCall
    ;


/*
 * ============================================================================
 * 17. BROADCAST BOUNDARY
 * ============================================================================
 *
 * Examples:
 *
 *     broadcast(x, shape)
 *     broadcast_to(x, shape)
 *
 * Broadcast compatibility is semantic.
 * ============================================================================
 */

tensorBroadcast
    : tensorOperationCall
    ;


/*
 * ============================================================================
 * 18. ELEMENT-WISE COMPOSITION BOUNDARY
 * ============================================================================
 *
 * Element-wise arithmetic belongs primarily to the canonical expression
 * grammar.
 *
 * This rule exists only as a semantic tensor-domain boundary for consumers
 * that need to identify an expression as participating in tensor arithmetic.
 *
 * It intentionally uses canonical expression operands.
 *
 * ============================================================================
 */

tensorElementwise
    : expression
    ;


/*
 * ============================================================================
 * 19. TENSOR VALUE BOUNDARY
 * ============================================================================
 *
 * This is a stable bridge for domain orchestrators.
 *
 * It does not create a second expression language.
 * ============================================================================
 */

tensorValue
    : tensorExpression
    ;


/*
 * ============================================================================
 * 20. TENSOR TYPE BOUNDARY
 * ============================================================================
 *
 * Tensor type semantics belong to Types.
 *
 * This rule is a bridge only.
 * ============================================================================
 */

tensorTypeReference
    : typeExpression
    ;


/*
 * ============================================================================
 * 21. TENSOR DIMENSION BOUNDARY
 * ============================================================================
 */

tensorDimensionExpression
    : expression
    ;


/*
 * ============================================================================
 * 22. TENSOR AXIS BOUNDARY
 * ============================================================================
 */

tensorAxisExpression
    : expression
    ;


/*
 * ============================================================================
 * 23. TENSOR SHAPE BOUNDARY
 * ============================================================================
 */

tensorShapeExpression
    : tensorShape
    ;


/*
 * ============================================================================
 * 24. TENSOR INDEX BOUNDARY
 * ============================================================================
 */

tensorIndexExpression
    : tensorIdentifier
      tensorAccessSuffix
    ;


/*
 * ============================================================================
 * 25. TENSOR SLICE BOUNDARY
 * ============================================================================
 */

tensorSliceExpression
    : tensorIdentifier
      tensorSliceSuffix
    ;


/*
 * ============================================================================
 * 26. TENSOR PROGRAM
 * ============================================================================
 *
 * A tensor program is an unbounded structural sequence.
 *
 * It deliberately reuses the tensor construct boundary rather than defining
 * another program root.
 * ============================================================================
 */

tensorProgram
    : tensorConstruct*
    ;


/*
 * ============================================================================
 * 27. TENSOR COMPUTATION REGION
 * ============================================================================
 *
 * Alias retained as a semantic naming boundary.
 *
 * The canonical structural representation is tensorRegion.
 * ============================================================================
 */

tensorComputationRegion
    : tensorRegion
    ;


/*
 * ============================================================================
 * 28. SYMBOLIC DIMENSION
 * ============================================================================
 *
 * A symbolic dimension is simply an ordinary expression.
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
 * Evaluation is downstream.
 * ============================================================================
 */

tensorSymbolicDimension
    : expression
    ;


/*
 * ============================================================================
 * 29. DYNAMIC SHAPE
 * ============================================================================
 *
 * Dynamic dimensions are represented by ordinary expressions.
 *
 * There is no sentinel value and no machine-specific maximum.
 * ============================================================================
 */

tensorDynamicShape
    : tensorShape
    ;


/*
 * ============================================================================
 * 30. RESOURCE-AWARE TENSOR BOUNDARY
 * ============================================================================
 *
 * Resource requirements are owned by grammar/resources/.
 *
 * This bridge deliberately does not introduce tensor-specific device syntax.
 *
 * It exists so a domain composition layer can associate tensor semantics with
 * an already-parsed canonical resource expression.
 *
 * ============================================================================
 */

tensorResourceBoundary
    : expression
    ;


/*
 * ============================================================================
 * 31. CAPABILITY-AWARE TENSOR BOUNDARY
 * ============================================================================
 *
 * Capability semantics are owned by the canonical capability/resource system.
 * ============================================================================
 */

tensorCapabilityBoundary
    : expression
    ;


/*
 * ============================================================================
 * 32. PORTABILITY BOUNDARY
 * ============================================================================
 *
 * Portability is semantic.
 *
 * The tensor grammar merely provides a stable bridge for consumers that need
 * to attach portable computation metadata.
 * ============================================================================
 */

tensorPortabilityBoundary
    : expression
    ;


/*
 * ============================================================================
 * 33. DOMAIN-NEUTRAL CONSTRUCTION BOUNDARY
 * ============================================================================
 *
 * Tensor construction remains callable/open-world.
 *
 * Examples:
 *
 *     tensor(...)
 *     zeros(...)
 *     ones(...)
 *     full(...)
 *     random(...)
 *     arange(...)
 *     from_data(...)
 *
 * No constructor is a mandatory keyword.
 * ============================================================================
 */

tensorConstruction
    : tensorOperationCall
    ;


/*
 * ============================================================================
 * 34. INDEX TARGET
 * ============================================================================
 *
 * Kept deliberately small so assignment does not recreate the universal
 * l-value system.
 * ============================================================================
 */

tensorIndexTarget
    : tensorIdentifier
      tensorAccessSuffix
    ;


/*
 * ============================================================================
 * 35. SLICE TARGET
 * ============================================================================
 */

tensorSliceTarget
    : tensorIdentifier
      tensorSliceSuffix
    ;


/*
 * ============================================================================
 * 36. TENSOR MEMBER ACCESS BOUNDARY
 * ============================================================================
 *
 * Member-access syntax itself belongs to Expressions.
 *
 * This bridge permits semantic tooling to classify a tensor-related expression
 * without defining a second member-access grammar.
 * ============================================================================
 */

tensorMemberAccess
    : expression
    ;


/*
 * ============================================================================
 * 37. TENSOR CALL BOUNDARY
 * ============================================================================
 *
 * Calls remain canonical expression syntax.
 * ============================================================================
 */

tensorCall
    : tensorOperationCall
    ;


/*
 * ============================================================================
 * 38. TENSOR ARGUMENT BOUNDARY
 * ============================================================================
 */

tensorArgumentList
    : tensorOperationArguments
    ;


/*
 * ============================================================================
 * 39. TENSOR AXIS LIST BOUNDARY
 * ============================================================================
 */

tensorAxesList
    : tensorAxisList
    ;


/*
 * ============================================================================
 * 40. TENSOR DIMENSION LIST BOUNDARY
 * ============================================================================
 */

tensorDimensions
    : tensorDimensionList
    ;


/*
 * ============================================================================
 * 41. TENSOR SHAPE LIST BOUNDARY
 * ============================================================================
 */

tensorShapes
    : tensorShape
    ;


/*
 * ============================================================================
 * 42. SEMANTIC OPERATION BOUNDARY
 * ============================================================================
 *
 * This rule is intentionally open.
 *
 * It allows future tensor operations without modifying this grammar simply
 * because a new mathematical or AI algorithm exists.
 * ============================================================================
 */

tensorOperation
    : tensorOperationCall
    ;


/*
 * ============================================================================
 * 43. FUTURE-PROOF EXTENSION BOUNDARY
 * ============================================================================
 *
 * Explicit annotation-led tensor extensions can be introduced by a higher
 * domain grammar.
 *
 * This grammar does not silently accept arbitrary annotations as tensor
 * operations.
 * ============================================================================
 */

tensorExtension
    : NANO_ANNOTATION
      identifier
      tensorExtensionArguments?
      tensorExtensionBody?
    ;


tensorExtensionArguments
    : LPAREN
      tensorOperationArguments?
      RPAREN
    ;


tensorExtensionBody
    : tensorRegion
    | expression
    ;


/*
 * ============================================================================
 * 44. INTEGRATION WITH AI.G4
 * ============================================================================
 *
 * The canonical AI composition layer is:
 *
 *     grammar/ai/ai.g4
 *
 * It should import:
 *
 *     AITensors
 *
 * when tensor leaf integration is enabled.
 *
 * The AI dispatcher should expose tensor syntax through:
 *
 *     tensorConstruct
 *
 * rather than copying any of the rules in this file.
 *
 * Conceptually:
 *
 *     AI
 *       |
 *       +--> AI-specific constructs
 *       |
 *       +--> AITensors.tensorConstruct
 *
 * AITensors MUST NOT import AI.
 *
 * This keeps dependency direction:
 *
 *     leaf
 *       ->
 *     domain dispatcher
 *       ->
 *     universal parser
 *
 * rather than:
 *
 *     tensor
 *       ->
 *     AI
 *       ->
 *     tensor
 *
 * ============================================================================
 * 45. INTEGRATION WITH TYPES
 * ============================================================================
 *
 * This file imports:
 *
 *     Types
 *
 * and consumes:
 *
 *     typeExpression
 *
 * The Types grammar remains the sole owner of:
 *
 *     - generic type syntax;
 *     - type arguments;
 *     - references;
 *     - arrays;
 *     - dependent types;
 *     - canonical tensor type representation.
 *
 * If Tensor<T, Shape> is not currently implemented semantically, that work
 * belongs in the canonical type subsystem.
 *
 * DO NOT add a competing tensor type grammar here merely to compensate.
 *
 * ============================================================================
 * 46. INTEGRATION WITH EXPRESSIONS
 * ============================================================================
 *
 * This file imports:
 *
 *     Expressions
 *
 * and consumes:
 *
 *     expression
 *
 * Expressions remains the sole owner of:
 *
 *     - arithmetic precedence;
 *     - logical precedence;
 *     - comparisons;
 *     - function calls;
 *     - member access;
 *     - unary operations;
 *     - conditional expressions;
 *     - lambdas;
 *     - ranges where universally defined;
 *     - universal indexing where universally defined.
 *
 * Tensor-specific syntax only adds domain structure that cannot be represented
 * by ordinary expression composition.
 *
 * ============================================================================
 * 47. INTEGRATION WITH CLASSICAL
 * ============================================================================
 *
 * Tensor values may interoperate with:
 *
 *     scalar
 *     vector
 *     matrix
 *     numerical
 *     symbolic
 *     scientific
 *     signal-processing
 *
 * semantics.
 *
 * This grammar does not duplicate those classical types or operations.
 *
 * ============================================================================
 * 48. INTEGRATION WITH DATA
 * ============================================================================
 *
 * Dataset and data semantics remain owned by:
 *
 *     grammar/data/
 *
 * A tensor may be produced from or consumed by data constructs through
 * ordinary types, expressions, functions, and semantic analysis.
 *
 * ============================================================================
 * 49. INTEGRATION WITH MODELS
 * ============================================================================
 *
 *     grammar/ai/models.g4
 *
 * may consume:
 *
 *     tensorTypeReference
 *     tensorValue
 *
 * but MUST NOT duplicate tensor declarations, shape syntax, indexing, or
 * transformation syntax.
 *
 * ============================================================================
 * 50. INTEGRATION WITH TRAINING
 * ============================================================================
 *
 *     grammar/ai/training.g4
 *
 * may consume tensor values and tensor types through:
 *
 *     expression
 *     typeExpression
 *     tensor semantic references
 *
 * Training algorithms remain downstream semantic/compiler concerns.
 *
 * ============================================================================
 * 51. INTEGRATION WITH INFERENCE
 * ============================================================================
 *
 *     grammar/ai/inference.g4
 *
 * may consume tensor values and model inputs/outputs through canonical
 * expressions and types.
 *
 * ============================================================================
 * 52. INTEGRATION WITH DIFFERENTIATION
 * ============================================================================
 *
 *     grammar/ai/differentiation.g4
 *
 * owns differentiation semantics.
 *
 * Tensor differentiation syntax must not create a separate differentiation
 * language here.
 *
 * Tensor values may be differentiation operands.
 *
 * ============================================================================
 * 53. INTEGRATION WITH ACCELERATORS
 * ============================================================================
 *
 *     grammar/ai/ai-accelerators.g4
 *
 * owns accelerator intent.
 *
 * This file must never define:
 *
 *     GPU identifiers
 *     TPU identifiers
 *     NPU identifiers
 *     device indices
 *     tensor-core counts
 *     accelerator counts
 *     memory capacity
 *
 * Accelerator suitability is determined through:
 *
 *     resources
 *     capabilities
 *     hardware abstraction
 *     compiler lowering
 *     scheduling
 *     runtime
 *
 * ============================================================================
 * 54. INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Tensor requirements use the canonical:
 *
 *     grammar/resources/
 *
 * subsystem.
 *
 * Distinguish:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *
 * Do not collapse these into tensor syntax.
 *
 * ============================================================================
 * 55. INTEGRATION WITH HARDWARE
 * ============================================================================
 *
 * Hardware realization is owned by:
 *
 *     grammar/hardware/
 *
 * and downstream target infrastructure.
 *
 * Tensor syntax remains target-independent.
 *
 * ============================================================================
 * 56. INTEGRATION WITH DISTRIBUTED COMPUTATION
 * ============================================================================
 *
 * Tensor operations may be distributed over:
 *
 *     tasks
 *     workers
 *     processes
 *     nodes
 *     devices
 *
 * but this file does not specify the physical distribution.
 *
 * No node count, worker count, or device count is hard-coded.
 *
 * ============================================================================
 * 57. INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Tensor computation may participate in:
 *
 *     quantum simulation;
 *     quantum optimization;
 *     hybrid algorithms;
 *     tensor-network computation;
 *     variational algorithms;
 *     quantum/classical data processing.
 *
 * Tensor syntax does not replace:
 *
 *     quantum::ir
 *
 * or define quantum semantics.
 *
 * ============================================================================
 * 58. INTEGRATION WITH HDL
 * ============================================================================
 *
 * Tensor operations may be lowered into hardware accelerators or HDL-backed
 * implementations.
 *
 * This grammar does not define:
 *
 *     registers
 *     buses
 *     fixed widths
 *     physical memories
 *     clock topology
 *     pipeline depth
 *
 * Those are downstream hardware semantics.
 *
 * ============================================================================
 * 59. INTEGRATION WITH HYBRID
 * ============================================================================
 *
 * Hybrid programs may compose:
 *
 *     tensor
 *       ->
 *     classical
 *       ->
 *     quantum
 *       ->
 *     measurement
 *       ->
 *     tensor
 *
 * without introducing a second tensor or hybrid IR.
 *
 * ============================================================================
 * 60. AST CONTRACT
 * ============================================================================
 *
 * Every tensor construct must preserve enough source structure for the
 * domain-neutral AST to retain:
 *
 *     - source span;
 *     - declaration identity;
 *     - type expression;
 *     - initializer;
 *     - tensor literal structure;
 *     - operation name;
 *     - operation arguments;
 *     - index expressions;
 *     - slice expressions;
 *     - shape expressions;
 *     - axis expressions;
 *     - annotations;
 *     - extension information.
 *
 * The grammar MUST NOT require target-specific AST nodes such as:
 *
 *     GPUTensorNode
 *     CUDATensorNode
 *     TensorCoreNode
 *     TPUNode
 *     PhysicalTensorNode
 *
 * ============================================================================
 * 61. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis owns:
 *
 *     - tensor type recognition;
 *     - element-type validation;
 *     - shape inference;
 *     - rank inference;
 *     - dimension compatibility;
 *     - broadcasting rules;
 *     - indexing validity;
 *     - slice validity;
 *     - contraction validity;
 *     - reduction validity;
 *     - operation resolution;
 *     - differentiability;
 *     - effect checking;
 *     - ownership/lifetime;
 *     - resource requirements;
 *     - capability requirements;
 *     - portability analysis.
 *
 * ============================================================================
 * 62. IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT create:
 *
 *     TensorIR
 *     AITensorIR
 *     GPU Tensor IR
 *     Accelerator Tensor IR
 *
 * merely because tensor syntax exists.
 *
 * Tensor semantics must lower through the repository's canonical semantic/IR
 * architecture.
 *
 * Classical tensor operations may lower through the canonical classical
 * representation.
 *
 * Quantum-related semantics must ultimately use:
 *
 *     quantum::ir
 *
 * where applicable.
 *
 * ============================================================================
 * 63. COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may determine:
 *
 *     - specialization;
 *     - layout;
 *     - tiling;
 *     - fusion;
 *     - vectorization;
 *     - parallelization;
 *     - accelerator mapping;
 *     - distributed partitioning;
 *     - memory placement;
 *     - target lowering.
 *
 * None of those decisions belong in this parser grammar.
 *
 * ============================================================================
 * 64. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may determine:
 *
 *     - available resources;
 *     - execution placement;
 *     - dynamic allocation;
 *     - scheduling;
 *     - device availability;
 *     - recovery;
 *     - resource negotiation.
 *
 * The runtime must not need to parse tensor grammar directly.
 *
 * ============================================================================
 * 65. ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to parser diagnostics.
 *
 * Semantic errors belong to semantic analysis.
 *
 * Examples of syntax errors:
 *
 *     missing ']'
 *     missing ')'
 *     malformed comma placement
 *     malformed slice
 *     malformed declaration
 *
 * Examples of semantic errors:
 *
 *     incompatible shapes
 *     invalid broadcast
 *     invalid contraction
 *     invalid axis
 *     invalid dtype
 *     unavailable capability
 *     insufficient resources
 *
 * The parser must not attempt to determine the latter.
 *
 * ============================================================================
 * 66. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar intentionally contains NO:
 *
 *     MAX_RANK
 *     MAX_DIMENSIONS
 *     MAX_AXES
 *     MAX_ELEMENTS
 *     MAX_TENSOR_SIZE
 *     MAX_BATCH
 *     MAX_FEATURES
 *     MAX_CHANNELS
 *     MAX_SEQUENCE
 *     MAX_PARAMETERS
 *     MAX_LAYERS
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_NODES
 *
 * It also contains no physical identifiers such as:
 *
 *     gpu0
 *     gpu1
 *     device0
 *     qpu0
 *     cpu0
 *
 * A numeric literal in a program remains program data.
 *
 * For example:
 *
 *     [1024, 1024]
 *
 * may be a legitimate program shape.
 *
 * It must NOT be interpreted as a language-wide tensor limit.
 *
 * ============================================================================
 * 67. SECURITY CONTRACT
 * ============================================================================
 *
 * The grammar contains no executable actions.
 *
 * It therefore cannot:
 *
 *     - execute tensor operations;
 *     - allocate memory;
 *     - access files;
 *     - access networks;
 *     - inspect devices;
 *     - access credentials;
 *     - invoke accelerators.
 *
 * Generated parser consumers must remain safe Rust.
 *
 * ============================================================================
 * 68. RUST CONTRACT
 * ============================================================================
 *
 * This grammar is language-runtime independent, but the repository frontend
 * consuming it must support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and:
 *
 *     #![forbid(unsafe_code)]
 *
 * No unsafe Rust is required for this grammar.
 *
 * ============================================================================
 * 69. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing stable tensor syntax should retain its meaning unless explicitly
 * deprecated through the language compatibility process.
 *
 * Historical or proposed tensor syntax in:
 *
 *     grammar/Zamani-Grammar.md
 *
 * does not automatically become legal syntax.
 *
 * Implementation status is reported by:
 *
 *     grammar/grammar.md
 *
 * Normative architecture remains governed by:
 *
 *     grammar/DESIGN.md
 *
 * ============================================================================
 * 70. TEST CONTRACT
 * ============================================================================
 *
 * The owning test suite must cover:
 *
 * ---------------------------------------------------------------------------
 * POSITIVE
 * ---------------------------------------------------------------------------
 *
 *     @tensor x: Tensor<f32>;
 *
 *     @tensor x: Tensor<f32, [N, M]>;
 *
 *     @tensor x: Tensor<T, [Batch, Sequence, Features]>;
 *
 *     @tensor x: Tensor<T, [N * M, K]>;
 *
 *     @tensor x: Tensor<T, [dynamic_dimension]>;
 *
 *     @tensor x: Tensor<T, [N, M]> = value;
 *
 *     x = [1, 2, 3];
 *
 *     x = [[1, 2], [3, 4]];
 *
 *     x[i];
 *
 *     x[i, j];
 *
 *     x[i, j, k];
 *
 *     x[start..end];
 *
 *     x[start..];
 *
 *     x[..end];
 *
 *     x[..];
 *
 *     reshape(x, [N, M]);
 *
 *     transpose(x);
 *
 *     transpose(x, [2, 0, 1]);
 *
 *     broadcast(x, shape);
 *
 *     reduce(x, axis);
 *
 *     contract(a, b);
 *
 *     einsum(a, b, specification);
 *
 *     matmul(a, b);
 *
 *     custom_tensor_operation(x);
 *
 * ---------------------------------------------------------------------------
 * NEGATIVE
 * ---------------------------------------------------------------------------
 *
 *     malformed declaration
 *     missing type delimiter
 *     missing semicolon
 *     missing ']'
 *     missing ')'
 *     malformed comma list
 *     malformed slice
 *     malformed operation call
 *
 * ---------------------------------------------------------------------------
 * BOUNDARY
 * ---------------------------------------------------------------------------
 *
 *     one dimension
 *     many dimensions
 *     many axes
 *     symbolic dimensions
 *     dynamic dimensions
 *     nested tensor literals
 *     nested tensor operations
 *     deeply composed expressions
 *
 * ---------------------------------------------------------------------------
 * SCALABILITY
 * ---------------------------------------------------------------------------
 *
 * Generated tests must vary:
 *
 *     tensor rank
 *     dimension count
 *     axis count
 *     operation count
 *     nesting depth
 *     declaration count
 *
 * without asserting a language-defined maximum.
 *
 * ---------------------------------------------------------------------------
 * CROSS-DOMAIN
 * ---------------------------------------------------------------------------
 *
 *     AI + tensor
 *     classical + tensor
 *     quantum + tensor
 *     hybrid + tensor
 *     tensor + distributed
 *     tensor + hardware
 *     tensor + HDL
 *     tensor + resources
 *     tensor + capabilities
 *
 * ---------------------------------------------------------------------------
 * DETERMINISM
 * ---------------------------------------------------------------------------
 *
 * Identical source and identical grammar configuration must yield identical
 * parse structure.
 *
 * ============================================================================
 * 71. DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] AITensors remains the canonical tensor leaf grammar.
 *
 *     [ ] tensorConstruct is the stable public composition boundary.
 *
 *     [ ] Types remains the sole general type owner.
 *
 *     [ ] Expressions remains the sole general expression owner.
 *
 *     [ ] Statements remains the sole general statement owner.
 *
 *     [ ] ZamaniLexer remains the sole lexical authority.
 *
 *     [ ] No lexer rule exists in this parser grammar.
 *
 *     [ ] No parser-level RANGE_OPERATOR token is created.
 *
 *     [ ] Tensor rank is structurally unbounded.
 *
 *     [ ] Dimension count is structurally unbounded.
 *
 *     [ ] Axis count is structurally unbounded.
 *
 *     [ ] Tensor operation count is structurally unbounded.
 *
 *     [ ] Tensor declaration count is structurally unbounded.
 *
 *     [ ] No physical machine limit is encoded.
 *
 *     [ ] No accelerator/device identity is encoded.
 *
 *     [ ] No vendor-specific tensor syntax is required.
 *
 *     [ ] Tensor operations remain open-world.
 *
 *     [ ] Tensor type syntax is delegated to Types.
 *
 *     [ ] Tensor general expression syntax is delegated to Expressions.
 *
 *     [ ] Tensor statement composition is delegated to Statements.
 *
 *     [ ] AI.g4 imports/composes AITensors.
 *
 *     [ ] models.g4 does not duplicate tensor syntax.
 *
 *     [ ] training.g4 does not duplicate tensor syntax.
 *
 *     [ ] inference.g4 does not duplicate tensor syntax.
 *
 *     [ ] differentiation.g4 does not duplicate tensor syntax.
 *
 *     [ ] resources/ owns resource semantics.
 *
 *     [ ] hardware/ owns hardware realization.
 *
 *     [ ] quantum::ir remains the canonical quantum semantic boundary.
 *
 *     [ ] No TensorIR is introduced merely by this grammar.
 *
 *     [ ] AST mapping is documented and implemented.
 *
 *     [ ] Semantic validation is implemented downstream.
 *
 *     [ ] IR lowering is implemented downstream.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Rust consumers remain Rust 1.97 / 1.97.1 compatible.
 *
 *     [ ] Rust consumers remain safe Rust with no unsafe code.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 *     Tensor syntax describes tensor computation.
 *
 *     It does not describe the machine.
 *
 *     Types describe tensor type semantics.
 *
 *     Expressions describe universal expression semantics.
 *
 *     Resources describe resource intent.
 *
 *     Capabilities describe target capabilities.
 *
 *     Compiler infrastructure determines realization.
 *
 *     Runtime infrastructure determines execution.
 *
 *     quantum::ir remains the canonical quantum semantic boundary.
 *
 * Therefore the tensor language can scale from:
 *
 *     tiny
 *        ->
 *     embedded
 *        ->
 *     CPU
 *        ->
 *     multicore
 *        ->
 *     GPU
 *        ->
 *     FPGA
 *        ->
 *     ASIC
 *        ->
 *     accelerator
 *        ->
 *     QPU
 *        ->
 *     HPC
 *        ->
 *     distributed
 *        ->
 *     cloud
 *        ->
 *     future computational targets
 *
 * without changing the tensor source language merely because the available
 * hardware becomes larger, smaller, different, or previously unknown.
 *
 * ============================================================================
 */