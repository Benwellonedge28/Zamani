/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/scalar.g4
 *
 * Role:
 *     Canonical classical scalar-value parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * Safety:
 *     This file contains grammar only.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No target-specific code.
 *     No hardware access.
 *     No runtime execution.
 *     No unsafe Rust.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SCALAR VALUE BOUNDARY for the classical
 * domain.
 *
 * It provides parser-level classification for values that occupy one source
 * value position, including:
 *
 *     integer literals
 *     floating-point literals
 *     boolean literals
 *     character literals
 *     string literals
 *     nil/null literals
 *     scalar identifier references
 *
 * It also exposes stable parser boundaries for consumers that need to
 * distinguish:
 *
 *     numeric scalar
 *     integral scalar
 *     real scalar
 *     boolean scalar
 *     character scalar
 *     text scalar
 *     null scalar
 *     compile-time scalar candidate
 *
 * These are SYNTACTIC CLASSIFICATIONS.
 *
 * They do not perform:
 *
 *     type inference
 *     type checking
 *     name resolution
 *     constant evaluation
 *     numeric conversion
 *     overflow analysis
 *     precision selection
 *     representation selection
 *     target selection
 *     resource allocation
 *     hardware selection
 *     IR construction
 *     runtime execution
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     grammar/antlr/ZamaniLexer.g4
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Scalar
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural analysis
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   classical semantics   resource semantics   cross-domain semantics
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *                 +------------+------------+
 *                 |            |            |
 *                 v            v            v
 *            Classical IR  quantum::ir  HDL/Hardware IR
 *                 |            |            |
 *                 +------------+------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                       scheduling / routing
 *                              |
 *                       resilience / QEC
 *                              |
 *                              ZQN
 *                              |
 *                              HAL
 *                              |
 *                       target realization
 *
 * Scalar.g4 NEVER constructs IR.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - scalar value classification;
 *     - scalar literal classification;
 *     - scalar identifier-reference classification;
 *     - numeric scalar parser boundaries;
 *     - integral scalar parser boundaries;
 *     - real scalar parser boundaries;
 *     - boolean scalar parser boundaries;
 *     - character scalar parser boundaries;
 *     - text scalar parser boundaries;
 *     - null scalar parser boundaries;
 *     - compile-time scalar candidate boundaries;
 *     - classical scalar integration boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - numeric literal spelling;
 *     - numeric digit syntax;
 *     - numeric bases;
 *     - numeric suffixes;
 *     - string escape syntax;
 *     - character escape syntax;
 *     - identifier spelling;
 *     - keywords;
 *     - operators;
 *     - operator precedence;
 *     - general expressions;
 *     - assignments;
 *     - function calls;
 *     - indexing;
 *     - member access;
 *     - ranges;
 *     - collections;
 *     - tuples;
 *     - arrays;
 *     - vectors;
 *     - matrices;
 *     - tensors;
 *     - type expressions;
 *     - generic syntax;
 *     - declarations;
 *     - statements;
 *     - memory semantics;
 *     - concurrency;
 *     - quantum semantics;
 *     - HDL semantics;
 *     - hardware realization;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime representation.
 *
 * ============================================================================
 *
 * AUTHORITY MODEL
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/lexer/*
 *     grammar/antlr/ZamaniLexer.g4
 *
 * General expression authority:
 *
 *     grammar/expressions/*
 *
 * Type authority:
 *
 *     grammar/types/*
 *
 * Classical-domain composition:
 *
 *     grammar/classical/classical.g4
 *
 * Universal parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * AST authority:
 *
 *     frontend AST
 *
 * Semantic authority:
 *
 *     semantic analysis
 *
 * Canonical quantum semantic authority:
 *
 *     quantum::ir
 *
 * Classical lowering:
 *
 *     canonical classical semantic/IR layers
 *
 * Hardware realization:
 *
 *     hardware / HAL / backend layers
 *
 * ============================================================================
 *
 * SINGLE LEXER CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes ONLY the canonical Zamani lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * It therefore uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT define lexer rules.
 *
 * It MUST NOT create:
 *
 *     SCALAR
 *     SCALAR_INTEGER
 *     SCALAR_FLOAT
 *     SCALAR_IDENTIFIER
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     BOOLEAN_LITERAL
 *     STRING_LITERAL
 *     CHARACTER_LITERAL
 *
 * as replacement tokens.
 *
 * Existing canonical tokens remain authoritative.
 *
 * ============================================================================
 *
 * EXISTING TOKEN CONTRACT
 * ============================================================================
 *
 * This file intentionally consumes the existing canonical tokens:
 *
 *     INTEGER
 *     FLOAT
 *     TRUE
 *     FALSE
 *     CHAR
 *     STRING
 *     NIL
 *     NULL
 *     IDENTIFIER
 *     COMMA
 *     LPAREN
 *     RPAREN
 *
 * No new token is required by this grammar.
 *
 * The token meanings are owned by the canonical lexer.
 *
 * In particular:
 *
 *     INTEGER
 *
 * means source integer syntax only.
 *
 *     FLOAT
 *
 * means source floating-point syntax only.
 *
 * Neither token establishes a machine representation.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Scalar syntax MUST remain independent of the machine on which the program
 * eventually executes.
 *
 * This grammar therefore contains no limits on:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     QPU count
 *     qubit count
 *     register count
 *     register width
 *     SIMD width
 *     memory capacity
 *     cache size
 *     NUMA topology
 *     cluster size
 *     node count
 *     network size
 *     tensor dimensions
 *     tensor rank
 *     scalar precision
 *     scalar bit width
 *     identifier length
 *     literal digit count
 *     scalar argument count
 *
 * A scalar literal such as:
 *
 *     42
 *
 * does NOT intrinsically mean:
 *
 *     i32
 *     i64
 *     u32
 *     u64
 *     usize
 *     machine word
 *     register
 *
 * Likewise:
 *
 *     1.0
 *
 * does NOT intrinsically mean:
 *
 *     f32
 *     f64
 *     IEEE-754 binary32
 *     IEEE-754 binary64
 *
 * Semantic/type analysis determines the language-level meaning and later
 * compilation determines the target representation.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no artificial finite limits.
 *
 * There are deliberately no constructs such as:
 *
 *     MAX_INTEGER_BITS
 *     MAX_FLOAT_BITS
 *     MAX_LITERAL_DIGITS
 *     MAX_SCALAR_ARGUMENTS
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_SCALAR_SIZE
 *
 * Structural repetition belongs to the owning grammar layers.
 *
 * Practical implementation limits are allowed only as explicit implementation
 * or resource policies. They must never silently become language semantics.
 *
 * ============================================================================
 *
 * EXPRESSION SEPARATION
 * ============================================================================
 *
 * Scalar.g4 deliberately does NOT define:
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
 *     !
 *     bitwise operators
 *     calls
 *     indexing
 *     member access
 *     assignment
 *     ranges
 *
 * Those belong to grammar/expressions/.
 *
 * For example:
 *
 *     1 + 2
 *
 * is an expression containing two scalar literals.
 *
 * It is NOT itself a scalar grammar production.
 *
 * Likewise:
 *
 *     -42
 *
 * is an expression involving a scalar literal and a unary operator.
 *
 * Scalar.g4 therefore provides the value leaf:
 *
 *     42
 *
 * while the expression grammar owns:
 *
 *     -42
 *     1 + 2
 *     x * 4
 *     a < b
 *
 * This prevents competing precedence hierarchies.
 *
 * ============================================================================
 *
 * TYPE SEPARATION
 * ============================================================================
 *
 * Scalar VALUE syntax is not scalar TYPE syntax.
 *
 * Examples:
 *
 *     42
 *     1.0
 *     true
 *     'a'
 *     "hello"
 *
 * are source values.
 *
 * Type syntax remains owned by:
 *
 *     grammar/types/
 *
 * This grammar MUST NOT define:
 *
 *     int
 *     float
 *     bool
 *     char
 *     string
 *
 * as type syntax.
 *
 * ============================================================================
 *
 * SEMANTIC CLASSIFICATION
 * ============================================================================
 *
 * The following rules are intentionally syntactic:
 *
 *     numericScalar
 *     integralScalar
 *     realScalar
 *     booleanScalar
 *     characterScalar
 *     textScalar
 *     nullScalar
 *
 * For an identifier:
 *
 *     x
 *
 * the grammar can establish that `x` is an identifier reference.
 *
 * It cannot establish that `x` is:
 *
 *     integer
 *     floating point
 *     boolean
 *     character
 *     string
 *     numeric
 *
 * without name and type resolution.
 *
 * Therefore identifier references remain under:
 *
 *     scalarReference
 *
 * and their semantic type is resolved downstream.
 *
 * ============================================================================
 *
 * NULL / NIL COMPATIBILITY
 * ============================================================================
 *
 * The repository currently retains both:
 *
 *     NIL
 *     NULL
 *
 * as canonical lexer tokens.
 *
 * Scalar.g4 preserves both spellings for compatibility.
 *
 * Whether they are semantically equivalent is NOT decided here.
 *
 * That decision belongs to the type/compatibility specification.
 *
 * This prevents the parser from silently changing an existing compatibility
 * contract.
 *
 * ============================================================================
 *
 * STRING CLASSIFICATION
 * ============================================================================
 *
 * `STRING` is included in scalarLiteral because it is one source-level value
 * occupying one value position.
 *
 * This is a PARSER CATEGORY only.
 *
 * It does NOT assert that strings are numeric scalars.
 *
 * Consequently:
 *
 *     stringLiteral
 *
 * is available through:
 *
 *     textScalar
 *
 * but NOT through:
 *
 *     numericScalar
 *
 * ============================================================================
 *
 * CHARACTER CLASSIFICATION
 * ============================================================================
 *
 * `CHAR` is a source-level character literal.
 *
 * Character encoding and storage representation are semantic/backend concerns.
 *
 * This grammar therefore does not assume:
 *
 *     ASCII
 *     UTF-8 byte width
 *     UTF-16 code-unit width
 *     UTF-32 storage
 *     machine character width
 *
 * ============================================================================
 *
 * NUMERIC CLASSIFICATION
 * ============================================================================
 *
 * Numeric literal spelling is owned by:
 *
 *     grammar/lexer/numeric-literals.g4
 *
 * That grammar already defines:
 *
 *     INTEGER
 *     FLOAT
 *
 * and supports scalable source forms including:
 *
 *     decimal integers
 *     binary integers
 *     octal integers
 *     hexadecimal integers
 *     decimal floating point
 *     exponent notation
 *     digit separators
 *
 * Scalar.g4 does not duplicate any of those lexical rules.
 *
 * ============================================================================
 *
 * PRECISION / EXACTNESS
 * ============================================================================
 *
 * The parser does not convert literal text into a Rust primitive.
 *
 * It does not choose:
 *
 *     i32
 *     i64
 *     u32
 *     u64
 *     usize
 *     f32
 *     f64
 *
 * or any other representation.
 *
 * The frontend must preserve source spelling/source span sufficiently for:
 *
 *     exact diagnostics
 *     deterministic semantic analysis
 *     constant evaluation
 *     reproducible compilation
 *     provenance
 *     later target lowering
 *
 * ============================================================================
 *
 * COMPILE-TIME SCALAR CONTRACT
 * ============================================================================
 *
 * `compileTimeScalar` means:
 *
 *     syntactically eligible to be considered as a scalar compile-time value
 *
 * It does NOT mean:
 *
 *     already evaluated
 *     guaranteed constant
 *     guaranteed pure
 *     guaranteed compile-time executable
 *
 * Semantic/compiler analysis determines whether an identifier actually refers
 * to a compile-time value.
 *
 * Examples:
 *
 *     1024
 *     1.0
 *     true
 *     N
 *     Rows
 *
 * are syntactically eligible.
 *
 * ============================================================================
 *
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Scalar values can be consumed by:
 *
 *     classical computation
 *     quantum parameters
 *     quantum control conditions
 *     HDL parameters
 *     hardware requirements
 *     resource requirements
 *     timing constraints
 *     distributed configuration
 *     AI/data computation
 *     compile-time type/value parameters
 *     interoperability layers
 *
 * Scalar.g4 does not need to know which domain consumes the value.
 *
 * Domain-specific semantics are downstream.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A scalar may appear as a quantum parameter:
 *
 *     angle
 *     phase
 *     coefficient
 *     threshold
 *     probability
 *     symbolic parameter
 *
 * The quantum parser/domain owns the surrounding quantum syntax.
 *
 * Scalar.g4 MUST NOT:
 *
 *     enumerate quantum gates;
 *     define physical qubits;
 *     define qubit limits;
 *     define quantum topology;
 *     define routing;
 *     define scheduling;
 *     define QEC;
 *     define ZQN;
 *     define HAL behavior.
 *
 * Quantum semantic lowering remains:
 *
 *     scalar source value
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Scalar values may parameterize:
 *
 *     hardware-independent widths
 *     timing
 *     resource requirements
 *     generic hardware structures
 *     HDL parameters
 *
 * A source value such as:
 *
 *     1024
 *
 * has no machine-specific interpretation until the owning declaration and
 * semantic layers establish one.
 *
 * Scalar.g4 does not decide whether 1024 means:
 *
 *     a dimension
 *     a timing quantity
 *     a resource quantity
 *     a mathematical value
 *     a parameter
 *
 * ============================================================================
 *
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource grammar owns resource intent.
 *
 * Scalar.g4 only supplies scalar values that resource syntax may consume.
 *
 * Distinguish downstream:
 *
 *     value
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     implementation decision
 *
 * For example, the scalar:
 *
 *     1024
 *
 * does not itself mean:
 *
 *     1024 CPUs
 *     1024 GPUs
 *     1024 qubits
 *     1024 nodes
 *     1024 bytes
 *
 * The surrounding semantic construct establishes the unit and meaning.
 *
 * ============================================================================
 *
 * MEMORY / OWNERSHIP
 * ============================================================================
 *
 * Scalar.g4 contains no ownership or memory-management syntax.
 *
 * It may supply scalar values to:
 *
 *     memory sizes
 *     alignment requirements
 *     region parameters
 *     allocation expressions
 *
 * Memory semantics remain owned by grammar/memory/ and downstream semantic
 * analysis.
 *
 * ============================================================================
 *
 * CONCURRENCY
 * ============================================================================
 *
 * Scalar values may parameterize:
 *
 *     task counts
 *     thresholds
 *     timing
 *     queue limits
 *     partition expressions
 *
 * Scalar.g4 does not define concurrency.
 *
 * No thread/core count is hard-coded here.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * For a fixed:
 *
 *     source token stream
 *     language version
 *     grammar version
 *
 * this grammar has deterministic parse behavior.
 *
 * It contains:
 *
 *     no actions
 *     no semantic predicates
 *     no runtime queries
 *     no environment queries
 *     no hardware queries
 *     no randomness
 *     no time dependence
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - executes no source code;
 *     - performs no evaluation;
 *     - performs no target selection;
 *     - performs no hardware discovery;
 *     - creates no unsafe Rust;
 *     - bypasses no semantic safety checks.
 *
 * Macro and metaprogramming layers remain subject to semantic validation.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The domain-neutral frontend AST should preserve the literal category and
 * source representation.
 *
 * Representative semantic categories are:
 *
 *     integer literal
 *     floating literal
 *     boolean literal
 *     character literal
 *     string literal
 *     null literal
 *     scalar reference
 *
 * The exact Rust AST type remains owned by the frontend AST implementation.
 *
 * Scalar.g4 MUST NOT introduce a second AST hierarchy.
 *
 * ============================================================================
 *
 * CLASSICAL IR CONTRACT
 * ============================================================================
 *
 * Scalar.g4 has no dependency on classical IR.
 *
 * Semantic analysis maps the scalar AST/value into the canonical classical
 * semantic representation.
 *
 * Lowering may subsequently choose:
 *
 *     scalar SSA values
 *     constants
 *     symbolic values
 *     vectorized representations
 *     accelerator representations
 *     distributed representations
 *
 * according to program semantics and available target resources.
 *
 * ============================================================================
 *
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * Scalar.g4 has no dependency on quantum::ir.
 *
 * If a scalar is consumed by a quantum operation, the quantum semantic layer
 * remains responsible for mapping the resulting operation to:
 *
 *     quantum::ir
 *
 * No quantum IR is created here.
 *
 * ============================================================================
 *
 * COMPILER / RUNTIME CONTRACT
 * ============================================================================
 *
 * Compiler and runtime decisions remain downstream.
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     cluster node
 *     device
 *     memory bank
 *     register
 *     physical address
 *
 * POCO-REAF is preserved because source scalar syntax remains target-neutral.
 *
 * ============================================================================
 *
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * The public leaf entry point is:
 *
 *     scalar
 *
 * Additional stable classification boundaries are:
 *
 *     scalarLiteral
 *     scalarReference
 *     numericScalar
 *     integralScalar
 *     realScalar
 *     booleanScalar
 *     characterScalar
 *     textScalar
 *     nullScalar
 *     compileTimeScalar
 *     scalarConstant
 *     scalarConstantOrReference
 *
 * Classical integration boundaries:
 *
 *     classicalScalar
 *     classicalNumericScalar
 *     classicalCompileTimeScalar
 *
 * These names are parser contracts.
 *
 * Their semantic interpretation belongs downstream.
 *
 * ============================================================================
 */

parser grammar Scalar;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PRIMARY SCALAR ENTRY
 * ============================================================================
 *
 * A scalar value is either:
 *
 *     - a scalar literal; or
 *     - a reference whose semantic type may resolve to a scalar.
 *
 * Name resolution and type resolution occur downstream.
 *
 * ============================================================================
 */

scalar
    : scalarLiteral
    | scalarReference
    ;


/*
 * ============================================================================
 * 2. SCALAR LITERALS
 * ============================================================================
 *
 * This rule classifies canonical literal tokens only.
 *
 * Lexical spelling remains owned by grammar/lexer/.
 *
 * ============================================================================
 */

scalarLiteral
    : integerLiteral
    | floatingLiteral
    | booleanLiteral
    | characterLiteral
    | stringLiteral
    | nullLiteral
    ;


/*
 * ============================================================================
 * 3. INTEGER LITERAL
 * ============================================================================
 *
 * INTEGER is the canonical lexer token.
 *
 * It may represent decimal, binary, octal, or hexadecimal source syntax
 * according to grammar/lexer/numeric-literals.g4.
 *
 * No width or signedness is implied here.
 *
 * ============================================================================
 */

integerLiteral
    : INTEGER
    ;


/*
 * ============================================================================
 * 4. FLOATING-POINT LITERAL
 * ============================================================================
 *
 * FLOAT is the canonical lexer token.
 *
 * Precision and representation remain semantic decisions.
 *
 * ============================================================================
 */

floatingLiteral
    : FLOAT
    ;


/*
 * ============================================================================
 * 5. BOOLEAN LITERAL
 * ============================================================================
 *
 * TRUE and FALSE are canonical lexer tokens.
 *
 * ============================================================================
 */

booleanLiteral
    : TRUE
    | FALSE
    ;


/*
 * ============================================================================
 * 6. CHARACTER LITERAL
 * ============================================================================
 */

characterLiteral
    : CHAR
    ;


/*
 * ============================================================================
 * 7. STRING LITERAL
 * ============================================================================
 *
 * STRING is treated as a source-level scalar value category.
 *
 * This does NOT classify strings as numeric scalars.
 *
 * ============================================================================
 */

stringLiteral
    : STRING
    ;


/*
 * ============================================================================
 * 8. NIL / NULL LITERAL
 * ============================================================================
 *
 * Both existing lexical spellings remain accepted.
 *
 * Compatibility/type semantics remain downstream.
 *
 * ============================================================================
 */

nullLiteral
    : NIL
    | NULL
    ;


/*
 * ============================================================================
 * 9. SCALAR REFERENCE
 * ============================================================================
 *
 * This is deliberately only an identifier token.
 *
 * Qualified names, member access, indexing, calls, dereferencing, and other
 * reference forms belong to the canonical expression/name grammar.
 *
 * Semantic name resolution determines whether the referenced entity is a
 * scalar value.
 *
 * ============================================================================
 */

scalarReference
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 10. NUMERIC SCALAR
 * ============================================================================
 *
 * Syntactically numeric scalar literals.
 *
 * An identifier is not included because whether an identifier denotes a
 * numeric value is a semantic/type property.
 *
 * ============================================================================
 */

numericScalar
    : integerLiteral
    | floatingLiteral
    ;


/*
 * ============================================================================
 * 11. INTEGRAL SCALAR
 * ============================================================================
 *
 * Syntactic integer classification only.
 *
 * It does NOT establish:
 *
 *     signedness
 *     width
 *     overflow behavior
 *     machine representation
 *
 * ============================================================================
 */

integralScalar
    : integerLiteral
    ;


/*
 * ============================================================================
 * 12. REAL SCALAR
 * ============================================================================
 *
 * Syntactic floating-point classification only.
 *
 * ============================================================================
 */

realScalar
    : floatingLiteral
    ;


/*
 * ============================================================================
 * 13. BOOLEAN SCALAR
 * ============================================================================
 */

booleanScalar
    : booleanLiteral
    ;


/*
 * ============================================================================
 * 14. CHARACTER SCALAR
 * ============================================================================
 */

characterScalar
    : characterLiteral
    ;


/*
 * ============================================================================
 * 15. TEXT SCALAR
 * ============================================================================
 */

textScalar
    : stringLiteral
    ;


/*
 * ============================================================================
 * 16. NULL SCALAR
 * ============================================================================
 */

nullScalar
    : nullLiteral
    ;


/*
 * ============================================================================
 * 17. COMPILE-TIME SCALAR CANDIDATE
 * ============================================================================
 *
 * This rule means:
 *
 *     syntactically eligible for compile-time scalar analysis.
 *
 * It does not perform compile-time evaluation.
 *
 * An identifier must be resolved by the compiler to determine whether it is
 * actually a compile-time value.
 *
 * ============================================================================
 */

compileTimeScalar
    : scalarLiteral
    | scalarReference
    ;


/*
 * ============================================================================
 * 18. SCALAR CONSTANT
 * ============================================================================
 *
 * Literal syntax is the only syntactic form that is intrinsically constant
 * at the parser level.
 *
 * Named constants are represented as scalarReference and resolved later.
 *
 * ============================================================================
 */

scalarConstant
    : scalarLiteral
    ;


/*
 * ============================================================================
 * 19. SCALAR CONSTANT OR REFERENCE
 * ============================================================================
 *
 * Stable boundary for consumers that accept either a literal constant or a
 * named value.
 *
 * ============================================================================
 */

scalarConstantOrReference
    : scalarConstant
    | scalarReference
    ;


/*
 * ============================================================================
 * 20. CLASSICAL SCALAR DOMAIN BOUNDARY
 * ============================================================================
 *
 * Classical.g4 may consume this rule rather than duplicating scalar syntax.
 *
 * ============================================================================
 */

classicalScalar
    : scalar
    ;


/*
 * ============================================================================
 * 21. CLASSICAL NUMERIC SCALAR DOMAIN BOUNDARY
 * ============================================================================
 */

classicalNumericScalar
    : numericScalar
    ;


/*
 * ============================================================================
 * 22. CLASSICAL COMPILE-TIME SCALAR DOMAIN BOUNDARY
 * ============================================================================
 */

classicalCompileTimeScalar
    : compileTimeScalar
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DIRECT DEPENDENCY
 * -----------------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 *
 * EXISTING TOKENS CONSUMED
 * ------------------------
 *
 *     INTEGER
 *     FLOAT
 *     TRUE
 *     FALSE
 *     CHAR
 *     STRING
 *     NIL
 *     NULL
 *     IDENTIFIER
 *
 * No new lexer token is required.
 *
 *
 * LEXER INTEGRATION
 * -----------------
 *
 * Numeric spelling remains owned by:
 *
 *     grammar/lexer/numeric-literals.g4
 *
 * String spelling remains owned by:
 *
 *     grammar/lexer/string-literals.g4
 *
 * Character spelling remains owned by:
 *
 *     grammar/lexer/character-literals.g4
 *
 * Boolean spelling remains owned by:
 *
 *     grammar/lexer/boolean-literals.g4
 *
 * Identifier spelling remains owned by:
 *
 *     grammar/lexer/identifiers.g4
 *
 * The canonical parser-facing lexer remains:
 *
 *     ZamaniLexer
 *
 *
 * CLASSICAL INTEGRATION
 * ---------------------
 *
 * The canonical classical dispatcher is:
 *
 *     grammar/classical/classical.g4
 *
 * That grammar should import Scalar through its ANTLR grammar-composition
 * mechanism and consume:
 *
 *     classicalScalar
 *     classicalNumericScalar
 *     classicalCompileTimeScalar
 *
 * It MUST NOT redefine:
 *
 *     scalarLiteral
 *     integerLiteral
 *     floatingLiteral
 *     booleanLiteral
 *     characterLiteral
 *     stringLiteral
 *     nullLiteral
 *     scalarReference
 *
 *
 * ROOT PARSER INTEGRATION
 * -----------------------
 *
 * The universal parser composition root is:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Its existing architecture imports:
 *
 *     Classical
 *
 * rather than importing every classical leaf grammar directly.
 *
 * Therefore the intended dependency direction is:
 *
 *     Scalar
 *       |
 *       v
 *     Classical
 *       |
 *       v
 *     ZamaniParser
 *
 * not:
 *
 *     Scalar -> ZamaniParser
 *
 *
 * EXPRESSION INTEGRATION
 * ----------------------
 *
 * grammar/expressions/ owns:
 *
 *     expression
 *     unary expressions
 *     binary expressions
 *     calls
 *     indexing
 *     member access
 *     assignments
 *     ranges
 *     precedence
 *
 * Scalar.g4 intentionally does not import Expressions.
 *
 * This prevents a circular parser dependency:
 *
 *     Expressions -> Scalar
 *     Scalar -> Expressions
 *
 * A general expression can consume the scalar-value boundary through the
 * canonical expression composition layer.
 *
 *
 * TYPE INTEGRATION
 * ----------------
 *
 * grammar/types/ owns type syntax.
 *
 * Scalar.g4 does not import Types.
 *
 * Type/value parameterization may consume the scalar compile-time boundary
 * through the canonical composition architecture, without making Scalar.g4
 * responsible for type parsing.
 *
 *
 * DECLARATION INTEGRATION
 * -----------------------
 *
 * Declarations remain owned by:
 *
 *     grammar/declarations/
 *
 * Scalar.g4 does not define:
 *
 *     let
 *     var
 *     const
 *     parameter declarations
 *     fields
 *     function declarations
 *
 *
 * MEMORY / RESOURCE INTEGRATION
 * -----------------------------
 *
 * Scalar values may be consumed by:
 *
 *     grammar/memory/
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * The surrounding grammar establishes semantic units and meaning.
 *
 *
 * QUANTUM INTEGRATION
 * -------------------
 *
 * Quantum grammar may consume scalar values for:
 *
 *     operation parameters
 *     phase values
 *     coefficients
 *     probabilities
 *     thresholds
 *     classical control conditions
 *
 * The quantum layer remains responsible for:
 *
 *     operation semantics
 *     quantum types
 *     measurement
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *
 * and eventually lowers through:
 *
 *     quantum::ir
 *
 * Scalar.g4 creates no quantum IR.
 *
 *
 * HDL / HARDWARE INTEGRATION
 * --------------------------
 *
 * HDL/hardware grammars may consume scalar values as parameters.
 *
 * Scalar.g4 does not define:
 *
 *     wire widths
 *     register widths
 *     device counts
 *     physical addresses
 *     topology
 *     implementation resources
 *
 * Those meanings belong to the owning semantic domains.
 *
 *
 * AST INTEGRATION
 * ---------------
 *
 * Parser contexts from this grammar are mapped by the frontend into the
 * existing domain-neutral AST/value representation.
 *
 * This file does not create Rust AST objects.
 *
 * No new scalar-specific IR is permitted merely because this grammar exists.
 *
 *
 * CLASSICAL IR INTEGRATION
 * ------------------------
 *
 * Scalar syntax is lowered by semantic analysis into the repository's
 * canonical classical/value representation.
 *
 * Possible downstream realizations include:
 *
 *     constant
 *     symbolic value
 *     SSA value
 *     vectorized value
 *     accelerator value
 *     distributed value
 *
 * without changing the source grammar.
 *
 *
 * QUANTUM IR INTEGRATION
 * ----------------------
 *
 * If a scalar participates in a quantum operation:
 *
 *     scalar syntax
 *         ->
 *     frontend AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *
 * No second quantum IR is introduced.
 *
 *
 * COMPILER / RUNTIME INTEGRATION
 * ------------------------------
 *
 * This file provides source syntax only.
 *
 * Compiler/runtime layers determine:
 *
 *     representation
 *     placement
 *     vectorization
 *     parallelization
 *     accelerator use
 *     device selection
 *     memory placement
 *     scheduling
 *     deployment
 *
 * according to semantics, capabilities, constraints, requirements,
 * preferences, hints, and available resources.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * Integer:
 *
 *     0
 *     42
 *     1_000_000
 *     0b1010
 *     0o755
 *     0xFF
 *
 * Floating:
 *
 *     0.0
 *     1.0
 *     .5
 *     1.
 *     1e10
 *     1.5e-9
 *     .5e2
 *     1.e2
 *
 * Boolean:
 *
 *     true
 *     false
 *
 * Character:
 *
 *     'a'
 *     'λ'
 *     '\n'
 *     '\u03BB'
 *
 * String:
 *
 *     ""
 *     "hello"
 *     "Zamani"
 *
 * Null:
 *
 *     nil
 *     null
 *
 * References:
 *
 *     value
 *     _value
 *     scientific_value
 *
 *
 * CLASSIFICATION
 * --------------
 *
 * The following must parse through their respective boundaries:
 *
 *     numericScalar
 *     integralScalar
 *     realScalar
 *     booleanScalar
 *     characterScalar
 *     textScalar
 *     nullScalar
 *
 *
 * NEGATIVE
 * --------
 *
 * These must not become valid scalar literals through this grammar:
 *
 *     malformed numeric spellings
 *     malformed strings
 *     malformed characters
 *     unterminated literals
 *     invalid identifier spellings
 *
 * Their diagnostics belong to the canonical lexer/parser error architecture.
 *
 *
 * EXPRESSION SEPARATION
 * ---------------------
 *
 * The scalar grammar must NOT independently parse:
 *
 *     1 + 2
 *     -42
 *     x * 4
 *     a < b
 *     f(42)
 *     x[0]
 *     x.field
 *
 * Those belong to the expression grammar.
 *
 *
 * BOUNDARY
 * --------
 *
 * Tests must cover source values substantially larger than common native
 * machine widths to verify that scalar.g4 imposes no artificial width limit.
 *
 * For example, lexical acceptance must not change merely because a numeric
 * literal exceeds:
 *
 *     32 bits
 *     64 bits
 *     host usize
 *
 * Semantic representability is tested downstream.
 *
 *
 * SCALABILITY
 * -----------
 *
 * Tests must verify that the grammar itself contains no fixed:
 *
 *     scalar width
 *     scalar precision
 *     identifier length
 *     literal digit count
 *     resource count
 *
 * Practical parser/resource limits belong to implementation policy.
 *
 *
 * CROSS-DOMAIN
 * -----------
 *
 * At minimum test scalar values consumed as:
 *
 *     classical values
 *     quantum parameters
 *     HDL parameters
 *     hardware-independent requirements
 *     resource values
 *     compile-time dimensions
 *     distributed configuration values
 *
 *
 * DETERMINISM
 * -----------
 *
 * Identical:
 *
 *     source
 *     language version
 *     grammar version
 *
 * must produce identical lexical classification and parse structure.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_*
 *     fixed scalar width
 *     fixed scalar precision
 *     fixed digit count
 *     fixed identifier length
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     node count
 *     memory capacity
 *     register width
 *     SIMD width
 *     topology
 *     device identifier
 *     physical address
 *     vendor-specific instruction
 *
 * ============================================================================
 *
 * RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * The Rust implementation consuming it must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 * No `unsafe` implementation is required for this grammar.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * Scalar.g4 is complete when:
 *
 *     [x] It has one clear ownership responsibility.
 *
 *     [x] It consumes the canonical ZamaniLexer vocabulary.
 *
 *     [x] It does not define lexer tokens.
 *
 *     [x] It does not duplicate numeric/string/character/boolean lexical
 *         syntax.
 *
 *     [x] It does not define expression precedence.
 *
 *     [x] It does not define type syntax.
 *
 *     [x] It does not define declaration syntax.
 *
 *     [x] It does not define collection syntax.
 *
 *     [x] It does not define quantum gate syntax.
 *
 *     [x] It does not define HDL syntax.
 *
 *     [x] It does not define hardware topology.
 *
 *     [x] It does not define resource limits.
 *
 *     [x] It does not construct IR.
 *
 *     [x] It does not construct AST objects.
 *
 *     [x] It does not contain Rust actions.
 *
 *     [x] It does not contain unsafe code.
 *
 *     [x] It preserves existing canonical token names.
 *
 *     [x] It preserves NIL/NULL compatibility.
 *
 *     [x] It provides stable scalar classification boundaries.
 *
 *     [x] It remains target-independent.
 *
 *     [x] It remains compatible with POCO-REAF.
 *
 *     [ ] Classical.g4 imports this grammar through the canonical ANTLR
 *         composition mechanism.
 *
 *     [ ] Scalar-specific conformance tests are wired into grammar/tests/.
 *
 * The final two items are repository integration tasks, not responsibilities
 * of this leaf grammar itself.
 *
 * ============================================================================
 *
 * FINAL INVARIANT
 * ============================================================================
 *
 * Scalar.g4 answers exactly one question:
 *
 *     "Which source-level value forms are scalar-value candidates?"
 *
 * It does NOT answer:
 *
 *     "How wide is the value?"
 *     "Where is it stored?"
 *     "Which machine executes it?"
 *     "Which device receives it?"
 *     "How is it optimized?"
 *     "How is it scheduled?"
 *     "How is it represented physically?"
 *
 * Those questions remain downstream.
 *
 * Therefore the same scalar source syntax can participate in:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     quantum simulators
 *     clusters
 *     HPC systems
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * subject only to the semantics and resources of the actual compilation and
 * execution environment.
 *
 * ============================================================================
 */