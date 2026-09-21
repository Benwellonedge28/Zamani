/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/matrix.g4
 *
 * Status:
 *     Production classical matrix-domain parser component.
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No embedded Rust actions
 *     No semantic predicates
 *     No unsafe implementation
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the MATRIX-SPECIFIC SOURCE-SYNTAX BOUNDARY.
 *
 * Matrix computation is part of the single Zamani language. This file does
 * not create a second matrix language, second expression language, second
 * type system, or matrix-specific IR.
 *
 * Matrix syntax ultimately follows:
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
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------------------+
 *          |                            |
 *          v                            v
 *     classical semantics       resource/capability metadata
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     classical IR / canonical IR
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
 * This file MUST NOT construct IR directly.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - matrix-domain composition;
 *   - matrix literal syntax;
 *   - matrix operation-call composition;
 *   - matrix operation-name composition;
 *   - matrix argument-list composition;
 *   - matrix-domain entry points for the classical grammar;
 *   - source-level matrix constructs that cannot be represented merely as
 *     ordinary generic expression syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - keywords;
 *   - numeric literals;
 *   - strings;
 *   - general expressions;
 *   - expression precedence;
 *   - function calls generally;
 *   - indexing;
 *   - slicing;
 *   - ranges;
 *   - assignment;
 *   - declarations;
 *   - statements;
 *   - control flow;
 *   - general type syntax;
 *   - generic type arguments;
 *   - matrix semantic typing;
 *   - shape validation;
 *   - dimension arithmetic;
 *   - numerical algorithms;
 *   - numerical precision;
 *   - sparse/dense representation;
 *   - storage layout;
 *   - memory allocation;
 *   - vectorization;
 *   - tiling;
 *   - parallelization;
 *   - scheduling;
 *   - placement;
 *   - target selection;
 *   - hardware discovery;
 *   - classical IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General expression syntax belongs to:
 *
 *     grammar/expressions/expressions.g4
 *
 * General indexing/slicing syntax belongs to:
 *
 *     grammar/expressions/indexing.g4
 *
 * General type syntax belongs to:
 *
 *     grammar/types/types.g4
 *
 * Classical type constructors, including the open-world generic form used
 * for Matrix<T, ...>, belong to:
 *
 *     grammar/types/classical.g4
 *
 * General declarations belong to:
 *
 *     grammar/declarations/
 *
 * General statements belong to:
 *
 *     grammar/statements/
 *
 * This file MUST NOT redefine those authorities.
 *
 * ============================================================================
 * MATRIX TYPE MODEL
 * ============================================================================
 *
 * Matrix types are intentionally represented by the canonical generic type
 * system.
 *
 * Examples include:
 *
 *     Matrix<f64>
 *     Matrix<f32, Rows, Cols>
 *     math::Matrix<f64, Rows, Cols>
 *
 * The word "Matrix" is NOT made a special lexer token here.
 *
 * The existing classical type grammar already supports open-world type
 * applications and symbolic/value arguments.
 *
 * Therefore semantic analysis determines whether:
 *
 *     Matrix<T, ...>
 *
 * denotes a matrix semantic type.
 *
 * This prevents this file from creating a competing matrix type system.
 *
 * ============================================================================
 * MATRIX DIMENSIONS
 * ============================================================================
 *
 * Matrix dimensions are semantic values, not parser limits.
 *
 * They may be:
 *
 *     - literals;
 *     - identifiers;
 *     - qualified names;
 *     - compile-time expressions;
 *     - symbolic values;
 *     - dependent values;
 *     - values obtained from other program computations where permitted by
 *       the type system.
 *
 * Examples:
 *
 *     Matrix<f64, N, M>
 *     Matrix<f64, Rows, Cols>
 *     Matrix<f64, N * M, K>
 *
 * This grammar imposes no maximum for:
 *
 *     rows
 *     columns
 *     elements
 *     dimensions
 *     operation count
 *     matrix rank
 *     source size
 *     expression depth
 *
 * Implementation/resource limits remain compiler, runtime, resource-policy,
 * or deployment concerns and MUST NOT become matrix-language semantics.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Matrix syntax describes WHAT computation means.
 *
 * It does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     distributed node
 *     memory bank
 *     SIMD width
 *     vector unit
 *     BLAS implementation
 *     LAPACK implementation
 *     vendor library
 *     device identifier
 *
 * The same source-level matrix computation can therefore be lowered to
 * different target realizations according to available capabilities and
 * resources.
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * Matrix algorithms MUST NOT be encoded as a closed parser enumeration.
 *
 * This file deliberately does NOT contain:
 *
 *     transpose
 *     inverse
 *     determinant
 *     solve
 *     LU
 *     QR
 *     SVD
 *     eigenvalues
 *     Cholesky
 *     FFT
 *     GEMM
 *     BLAS
 *     LAPACK
 *
 * as a finite grammar-level operation list.
 *
 * Such operations may be represented by ordinary source-level operation names
 * and resolved semantically.
 *
 * Examples:
 *
 *     transpose(A)
 *     inverse(A)
 *     solve(A, b)
 *     lu(A)
 *     qr(A)
 *     svd(A)
 *     vendor::operation(A)
 *     user_defined_operation(A, B)
 *
 * The parser records structure.
 *
 * Semantic analysis determines:
 *
 *     - whether the operation exists;
 *     - its signature;
 *     - operand types;
 *     - shape requirements;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - numerical semantics;
 *     - legal lowering.
 *
 * ============================================================================
 * OPERATOR MODEL
 * ============================================================================
 *
 * Matrix arithmetic uses the universal expression grammar.
 *
 * Examples:
 *
 *     A + B
 *     A - B
 *     A * B
 *     A / B
 *     A @ B
 *
 * where a future/operator-specific spelling is defined by the canonical
 * expression grammar.
 *
 * This file MUST NOT recreate operator precedence.
 *
 * Semantic analysis determines whether an operator applied to matrix operands
 * means:
 *
 *     matrix addition;
 *     matrix multiplication;
 *     element-wise operation;
 *     scalar/matrix operation;
 *     another user-defined operation.
 *
 * ============================================================================
 * LITERAL MODEL
 * ============================================================================
 *
 * Matrix literals use structural nesting:
 *
 *     [[1, 2], [3, 4]]
 *
 *     [[a, b], [c, d]]
 *
 *     [[f(i, j), g(i, j)], [h(i, j), k(i, j)]]
 *
 * There is no fixed row or column count.
 *
 * Semantic analysis determines:
 *
 *     - rectangularity;
 *     - element-type compatibility;
 *     - dimensional compatibility;
 *     - whether an empty matrix is legal;
 *     - whether nested structures denote a matrix rather than another
 *       collection type.
 *
 * ============================================================================
 * INDEXING MODEL
 * ============================================================================
 *
 * Matrix indexing is intentionally NOT redefined here.
 *
 * The canonical indexing grammar already supports:
 *
 *     A[i]
 *     A[i, j]
 *     A[i, j, k]
 *     A[start:end]
 *     A[start:end:step]
 *
 * through:
 *
 *     grammar/expressions/indexing.g4
 *
 * Therefore matrix-specific indexing semantics are determined by semantic
 * analysis after the universal postfix/indexing syntax has been parsed.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical token composition is:
 *
 *     grammar/lexer/tokens.g4
 *
 * This file declares NO lexer rules.
 *
 * Existing repository tokens used here include:
 *
 *     IDENTIFIER
 *     LBRACKET
 *     RBRACKET
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * No matrix-specific token is required.
 *
 * In particular, this file MUST NOT define parser-local lexer rules such as:
 *
 *     MATRIX_CONSTRUCTOR
 *     TRANSPOSE_OPERATOR
 *     INVERSE_OPERATOR
 *     ADJOINT_OPERATOR
 *     NEGATE_OPERATOR
 *
 * Those were previously incorrect because this is a parser grammar and the
 * canonical lexical vocabulary already provides IDENTIFIER.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Matrix syntax lowers into the existing domain-neutral frontend AST.
 *
 * This file MUST NOT define a matrix-specific Rust AST.
 *
 * Conceptual mappings:
 *
 *     matrixLiteral
 *         -> generic/domain-neutral literal or collection structure
 *
 *     matrixOperationExpression
 *         -> generic Operation expression
 *
 *     matrixOperationName
 *         -> operation name / qualified operation namespace
 *
 *     matrixArgumentList
 *         -> generic ordered operation operands/arguments
 *
 * The repository's generic operation model remains authoritative:
 *
 *     Operation {
 *         name,
 *         namespace,
 *         operands,
 *         parameters,
 *         results,
 *         attributes,
 *         modifiers,
 *         effects,
 *         capabilities,
 *         source
 *     }
 *
 * Exact Rust field construction belongs to the frontend lowering layer, not
 * this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this structurally valid matrix-domain syntax?"
 *
 * Semantic analysis answers:
 *
 *     "What does this matrix computation mean?"
 *
 * Semantic analysis owns:
 *
 *     - matrix type recognition;
 *     - shape inference;
 *     - shape compatibility;
 *     - dimension arithmetic;
 *     - scalar/matrix compatibility;
 *     - matrix operation resolution;
 *     - overload resolution;
 *     - numerical semantics;
 *     - precision;
 *     - sparsity;
 *     - storage semantics;
 *     - resource requirements;
 *     - capability requirements;
 *     - effects;
 *     - legality of target-specific realization.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Matrix syntax must not turn resources into language limits.
 *
 * A program may express resource intent through the canonical resource and
 * capability grammar, for example:
 *
 *     requires capability("matrix.compute")
 *
 * or a domain-specific resource contract.
 *
 * Whether a target provides the requested capability is not determined here.
 *
 * The decision belongs downstream to:
 *
 *     semantic analysis
 *     resource management
 *     compiler
 *     scheduler
 *     deployment
 *     runtime
 *     HAL
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar depends only on:
 *
 *     - source text;
 *     - the selected grammar version;
 *     - canonical lexer vocabulary;
 *     - canonical imported parser grammars.
 *
 * It MUST NOT depend on:
 *
 *     - hardware;
 *     - available RAM;
 *     - CPU/GPU count;
 *     - target device;
 *     - filesystem state;
 *     - network state;
 *     - randomness;
 *     - wall-clock time;
 *     - environment variables.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 */

parser grammar Matrix;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/*
 * ============================================================================
 * PUBLIC MATRIX COMPOSITION ENTRY
 * ============================================================================
 *
 * This is the only public matrix-domain entry point.
 *
 * It intentionally contains matrix-specific constructs only.
 *
 * General declarations, statements, assignments, indexing, and expressions
 * remain owned by their respective canonical grammars.
 * ============================================================================
 */

matrixConstruct
    : matrixLiteral
    | matrixOperationExpression
    ;


/*
 * ============================================================================
 * MATRIX LITERAL
 * ============================================================================
 *
 * Structural form:
 *
 *     []
 *     [[]]
 *     [[1]]
 *     [[1, 2], [3, 4]]
 *
 * Empty and nested forms are accepted syntactically.
 *
 * Semantic analysis decides whether a particular literal is a valid matrix.
 * ============================================================================
 */

matrixLiteral
    : LBRACKET
      matrixRowList?
      RBRACKET
    ;


/*
 * ============================================================================
 * MATRIX ROW LIST
 * ============================================================================
 *
 * No fixed row count is encoded.
 * ============================================================================
 */

matrixRowList
    : matrixRow
      (
          COMMA
          matrixRow
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * MATRIX ROW
 * ============================================================================
 *
 * A row is structurally a nested expression list.
 * ============================================================================
 */

matrixRow
    : LBRACKET
      matrixElementList?
      RBRACKET
    ;


/*
 * ============================================================================
 * MATRIX ELEMENT LIST
 * ============================================================================
 *
 * Elements use the canonical expression grammar.
 *
 * This permits:
 *
 *     [[1, 2]]
 *     [[a, b]]
 *     [[f(x), g(y)]]
 *     [[i + 1, j * 2]]
 *
 * without introducing a matrix-specific expression language.
 * ============================================================================
 */

matrixElementList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * MATRIX OPERATION EXPRESSION
 * ============================================================================
 *
 * Matrix operations are open-world.
 *
 * The operation name is source-level data and is resolved semantically.
 *
 * Examples:
 *
 *     transpose(A)
 *     inverse(A)
 *     determinant(A)
 *     solve(A, b)
 *     svd(A)
 *     math::transpose(A)
 *     linear_algebra::solve(A, b)
 *     vendor::operation(A)
 *     custom_operation(A, B)
 *
 * This grammar does not establish that any particular operation exists.
 * ============================================================================
 */

matrixOperationExpression
    : matrixOperationName
      LPAREN
      matrixArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * MATRIX OPERATION NAME
 * ============================================================================
 *
 * Qualified names use the canonical identifier token.
 *
 * No namespace-depth limit exists.
 *
 * Examples:
 *
 *     transpose
 *     math::transpose
 *     linear_algebra::decompose
 *     vendor::domain::operation
 *
 * The semantic layer resolves the resulting name.
 * ============================================================================
 */

matrixOperationName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * MATRIX OPERATION ARGUMENTS
 * ============================================================================
 *
 * Arguments are ordinary Zamani expressions.
 *
 * This is intentionally not a second matrix-expression grammar.
 *
 * Examples:
 *
 *     solve(A, b)
 *     reshape(A, rows, cols)
 *     multiply(A, B)
 *     scale(A, alpha)
 *     custom(A[i, j], f(x))
 *
 * Indexing itself is owned by the universal postfix/indexing grammar.
 * ============================================================================
 */

matrixArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * MATRIX TYPE INTEGRATION
 * ============================================================================
 *
 * Matrix types are deliberately NOT redefined here.
 *
 * The canonical type path is:
 *
 *     typeExpression
 *         |
 *         v
 *     classical type application
 *         |
 *         v
 *     Matrix<T, ...>
 *
 * The following source-level type forms are therefore resolved through the
 * existing type grammar rather than a duplicate matrix type grammar:
 *
 *     Matrix<T>
 *     Matrix<T, Rows, Cols>
 *     math::Matrix<T, Rows, Cols>
 *
 * This preserves one type-system authority.
 * ============================================================================
 */


/*
 * ============================================================================
 * MATRIX EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Matrix arithmetic does NOT receive another precedence hierarchy here.
 *
 * Ordinary expressions already own:
 *
 *     +
 *     -
 *     *
 *     /
 *     %
 *     comparisons
 *     logical operations
 *     ranges
 *     calls
 *     indexing
 *     member access
 *     assignments
 *     conditional expressions
 *
 * Therefore:
 *
 *     A + B
 *     A - B
 *     A * B
 *
 * are parsed through the canonical expression grammar.
 *
 * Semantic analysis determines whether the operands are matrices and what the
 * corresponding operation means.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * MATRIX INDEXING INTEGRATION
 * ============================================================================
 *
 * Matrix indexing uses the canonical postfix/indexing grammar.
 *
 * Examples:
 *
 *     A[i]
 *     A[i, j]
 *     A[i, j, k]
 *     A[start:end]
 *     A[start:end:step]
 *
 * No matrix-specific indexing rule is declared here.
 *
 * This avoids a second indexing authority and permits the same syntax to work
 * for vectors, tensors, arrays, quantum registers, data objects, and future
 * indexable domains.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * MATRIX CONSTRUCTION INTEGRATION
 * ============================================================================
 *
 * Named constructors remain ordinary semantic operation names.
 *
 * Examples:
 *
 *     matrix(...)
 *     zeros(...)
 *     ones(...)
 *     identity(...)
 *     diagonal(...)
 *     fill(...)
 *
 * The grammar intentionally does NOT enumerate these names.
 *
 * They are ordinary identifiers consumed by matrixOperationExpression.
 *
 * This permits future/user-defined constructors without modifying the grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NUMERICAL ALGORITHM INTEGRATION
 * ============================================================================
 *
 * Algorithms such as:
 *
 *     transpose
 *     inverse
 *     determinant
 *     solve
 *     LU
 *     QR
 *     SVD
 *     eigenvalue
 *     eigenvector
 *     Cholesky
 *     least_squares
 *
 * are semantic operations, not a finite parser vocabulary.
 *
 * A semantic operation registry/library/dialect may define their meaning.
 *
 * This preserves:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Anywhere
 *     Forever
 *
 * without requiring the grammar to be modified whenever a new matrix
 * algorithm is introduced.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Matrix values and operations remain ordinary Zamani values/operations.
 *
 * Therefore matrix computation can participate in:
 *
 *     classical
 *     hybrid
 *     quantum-classical
 *     AI/ML
 *     data
 *     distributed
 *     networking
 *     hardware/software co-design
 *
 * without introducing domain-specific machine syntax here.
 *
 * For example, a matrix may be used as:
 *
 *     - a classical computation operand;
 *     - a tensor/data object;
 *     - a model parameter;
 *     - a quantum-control parameter where semantically permitted;
 *     - an HDL/hardware-co-design parameter;
 *     - distributed data;
 *     - an accelerator input.
 *
 * Semantic analysis determines whether each use is legal.
 *
 * ============================================================================
 * RESOURCE / HARDWARE INTEGRATION
 * ============================================================================
 *
 * This file does NOT select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     device
 *     memory bank
 *     node
 *     SIMD width
 *     vector width
 *
 * A matrix operation may later be lowered to any suitable realization.
 *
 * Example conceptual pipeline:
 *
 *     matrix operation
 *          |
 *          v
 *     semantic operation
 *          |
 *          v
 *     canonical classical representation
 *          |
 *          v
 *     optimization
 *          |
 *          +---------------------+
 *          |                     |
 *          v                     v
 *     target-independent    resource analysis
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *               target lowering
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *         CPU        GPU       FPGA/ASIC/etc.
 *
 * The grammar does not choose among those targets.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A matrix occurring in a quantum program remains a classical semantic object
 * unless the semantic system explicitly defines a quantum-domain meaning.
 *
 * This grammar MUST NOT create:
 *
 *     quantum matrix IR
 *     quantum matrix gate enum
 *     quantum matrix routing
 *     quantum matrix scheduling
 *
 * Quantum semantics continue through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * where appropriate.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Matrix dimensions and values may participate in hardware/software
 * co-design, but this grammar does not encode:
 *
 *     fixed bus widths
 *     fixed register widths
 *     fixed memory sizes
 *     fixed accelerator counts
 *     fixed pipeline depths
 *     fixed device topology
 *
 * Such properties remain source-level parameters, requirements, capabilities,
 * constraints, or downstream target decisions according to their semantics.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors are structural parser errors.
 *
 * Semantic errors belong downstream.
 *
 * Examples of semantic errors:
 *
 *     non-rectangular matrix literal;
 *     incompatible matrix shapes;
 *     invalid matrix multiplication;
 *     invalid dimension expression;
 *     unknown matrix operation;
 *     incompatible element types;
 *     unsupported numerical semantics;
 *     unavailable required capability.
 *
 * The parser MUST NOT attempt to diagnose these as syntax errors merely
 * because they are semantically invalid.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The following are structurally unbounded:
 *
 *     matrix rows
 *     matrix columns
 *     matrix elements
 *     matrix operation arguments
 *     qualified operation-name depth
 *     nested expressions
 *     symbolic dimensions
 *     matrix operation sequences
 *
 * The grammar contains no:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_ELEMENTS
 *     MAX_MATRIX_SIZE
 *     MAX_DIMENSIONS
 *     MAX_RANK
 *     MAX_OPERATIONS
 *     MAX_ARGUMENTS
 *     MAX_NAMESPACE_DEPTH
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *
 * Resource availability is evaluated downstream.
 *
 * "Infinity" here means that the language grammar itself does not impose an
 * arbitrary finite hardware-oriented ceiling.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This replacement deliberately preserves the existing matrix file path:
 *
 *     grammar/classical/matrix.g4
 *
 * and the primary public rule:
 *
 *     matrixConstruct
 *
 * Existing consumers should therefore integrate through matrixConstruct rather
 * than depending on internal rules.
 *
 * Internal rules are not a substitute for the canonical generic expression,
 * type, declaration, statement, or indexing grammars.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive cases:
 *
 *     []
 *     [[]]
 *     [[1]]
 *     [[1, 2], [3, 4]]
 *     [[a, b], [c, d]]
 *     [[f(x), g(y)], [h(x), k(y)]]
 *
 *     transpose(A)
 *     inverse(A)
 *     solve(A, b)
 *     math::transpose(A)
 *     linear_algebra::solve(A, b)
 *     vendor::domain::operation(A, B)
 *     custom_operation(A, B, C)
 *
 * Required semantic integration cases:
 *
 *     Matrix<f64>
 *     Matrix<f64, Rows, Cols>
 *     math::Matrix<f64, Rows, Cols>
 *
 *     A + B
 *     A - B
 *     A * B
 *     A[i]
 *     A[i, j]
 *     A[i, j:k]
 *
 * Required negative cases:
 *
 *     malformed row delimiters;
 *     malformed argument lists;
 *     missing closing bracket;
 *     missing closing parenthesis;
 *     malformed qualified operation name;
 *     malformed comma placement.
 *
 * Required boundary/scalability cases:
 *
 *     one element;
 *     many rows;
 *     many columns;
 *     symbolic expressions as elements;
 *     symbolic dimensions;
 *     deeply qualified operation names;
 *     large operation argument lists.
 *
 * Required cross-domain cases:
 *
 *     classical matrix + data;
 *     classical matrix + AI;
 *     classical matrix + distributed;
 *     classical matrix + hardware/resource intent;
 *     hybrid program containing matrix operations;
 *     quantum-classical program carrying matrix values through semantic
 *     analysis without introducing a second quantum IR.
 *
 * ============================================================================
 * DETERMINISM TEST
 * ============================================================================
 *
 * Identical source text and identical grammar/token vocabulary must produce
 * identical parser structure regardless of:
 *
 *     CPU;
 *     GPU;
 *     memory size;
 *     available devices;
 *     runtime environment;
 *     hardware topology.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the matrix-domain hard-coding requirements when it contains:
 *
 *     no physical device identifiers;
 *     no hardware capacities;
 *     no fixed resource counts;
 *     no fixed matrix dimensions;
 *     no finite matrix algorithm registry;
 *     no vendor-specific parser tokens;
 *     no fixed SIMD width;
 *     no fixed numerical precision limit;
 *     no fixed memory capacity;
 *     no machine-specific ABI assumption.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is independently complete when:
 *
 *   1. It is a valid ANTLR parser grammar.
 *
 *   2. It consumes the canonical ZamaniLexer vocabulary.
 *
 *   3. It declares no lexer rules.
 *
 *   4. It introduces no new token requirement.
 *
 *   5. It uses IDENTIFIER rather than an invented IDENT token.
 *
 *   6. It does not redefine universal expression precedence.
 *
 *   7. It does not redefine indexing or slicing.
 *
 *   8. It does not redefine declarations.
 *
 *   9. It does not redefine statements or control flow.
 *
 *  10. It does not redefine the universal type system.
 *
 *  11. Matrix types remain represented through canonical generic types.
 *
 *  12. Matrix algorithms remain open-world semantic operations.
 *
 *  13. Matrix literals have no finite row/column limit.
 *
 *  14. Matrix operation argument lists have no finite arity limit.
 *
 *  15. Qualified operation names have no finite namespace-depth limit.
 *
 *  16. No hardware/resource limits are encoded.
 *
 *  17. No target-specific implementation is selected.
 *
 *  18. No second matrix IR is introduced.
 *
 *  19. The AST contract is domain-neutral.
 *
 *  20. The semantic contract is explicitly downstream.
 *
 *  21. Positive, negative, boundary, scalability, compatibility,
 *      determinism, and cross-domain tests are defined.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */