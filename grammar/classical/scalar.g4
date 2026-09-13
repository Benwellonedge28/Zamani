/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/scalar.g4
 *
 * Role:
 *     Classical scalar-domain parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No target-specific code.
 *     No unsafe Rust.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL CLASSICAL SCALAR DOMAIN.
 *
 * A scalar is a single computational value rather than a collection,
 * sequence, vector, matrix, tensor, distributed object, quantum register,
 * hardware resource, or physical device.
 *
 * This grammar provides stable parser boundaries for scalar values while
 * delegating lexical representation and general expression semantics to
 * their authoritative grammar layers.
 *
 * Scalar values may participate in:
 *
 *     classical computation
 *     quantum-classical control
 *     HDL parameterization
 *     hardware parameterization
 *     distributed computation
 *     AI/data computation
 *     compile-time computation
 *     resource expressions
 *     metadata
 *     configuration
 *     generic/value parameters
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     expressions                 types
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *               classical domain
 *                     |
 *                     v
 *                 Scalar.g4
 *                     |
 *                     v
 *                 frontend AST
 *                     |
 *                     v
 *             semantic analysis
 *                     |
 *          +----------+-----------+
 *          |                      |
 *          v                      v
 *     classical IR          constant/value IR
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *               optimization
 *                     |
 *                     v
 *             lowering / scheduling
 *                     |
 *                     v
 *              target realization
 *
 * This file NEVER directly creates IR.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - classical scalar-domain parser boundaries;
 *     - scalar literal classification;
 *     - scalar literal alternatives;
 *     - scalar literal groups;
 *     - scalar constant/value composition;
 *     - scalar identifier references;
 *     - scalar compile-time value boundaries;
 *     - scalar-domain syntactic wrappers;
 *     - scalar-domain parser integration points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifier spelling;
 *     - numeric literal spelling;
 *     - string literal spelling;
 *     - character literal spelling;
 *     - boolean literal spelling;
 *     - operators;
 *     - arithmetic precedence;
 *     - comparison precedence;
 *     - logical expressions;
 *     - general expressions;
 *     - assignments;
 *     - type definitions;
 *     - scalar type declarations;
 *     - vectors;
 *     - matrices;
 *     - tensors;
 *     - arrays;
 *     - quantum state;
 *     - quantum registers;
 *     - hardware resources;
 *     - hardware topology;
 *     - execution placement;
 *     - scheduling;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime representation.
 *
 * ============================================================================
 *
 * NON-OWNERSHIP / IMPORTANT SEPARATION
 * ============================================================================
 *
 * The following layers remain authoritative:
 *
 *     grammar/lexer/*
 *         lexical spelling
 *
 *     grammar/expressions/*
 *         general expression syntax and operator precedence
 *
 *     grammar/types/*
 *         type syntax
 *
 *     grammar/classical/classical.g4
 *         classical-domain composition
 *
 *     frontend / AST
 *         semantic source representation
 *
 *     semantic analysis
 *         meaning and legality
 *
 *     classical IR
 *         canonical classical representation
 *
 *     quantum::ir
 *         canonical quantum representation
 *
 *     optimization
 *         implementation improvement
 *
 *     scheduling
 *         ordering, timing and resource scheduling
 *
 *     hardware
 *         hardware capabilities and realization
 *
 *     runtime
 *         execution
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Scalar syntax describes VALUE SEMANTICS.
 *
 * It does NOT prescribe:
 *
 *     CPU count
 *     CPU architecture
 *     register count
 *     register width
 *     SIMD width
 *     GPU count
 *     GPU lane count
 *     accelerator count
 *     memory capacity
 *     cache size
 *     NUMA topology
 *     cluster size
 *     network topology
 *     device identifier
 *     physical address
 *     ABI representation
 *     storage layout
 *
 * For example:
 *
 *     42
 *
 * does not mean:
 *
 *     i32
 *     i64
 *     one machine register
 *     one CPU word
 *
 * unless semantic/type resolution explicitly establishes that meaning.
 *
 * Likewise:
 *
 *     1.0
 *
 * does not inherently mean:
 *
 *     IEEE-754 binary32
 *     IEEE-754 binary64
 *     one hardware floating-point register
 *
 * Representation is determined downstream.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO artificial finite limits.
 *
 * In particular, it does not define:
 *
 *     MAX_INTEGER_DIGITS
 *     MAX_FLOAT_DIGITS
 *     MAX_LITERAL_LENGTH
 *     MAX_SCALAR_COUNT
 *     MAX_EXPRESSION_DEPTH
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_PRECISION
 *     MAX_SCALE
 *     MAX_BITS
 *     MAX_WIDTH
 *
 * Repetition and recursion are structural.
 *
 * Any implementation limit must belong to an explicit:
 *
 *     parser resource policy
 *     compiler resource policy
 *     semantic validation policy
 *     target capability
 *     resource manager
 *     runtime environment
 *
 * Such limits must never silently become language semantics.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It MUST NOT define lexer rules.
 *
 * The canonical lexer already owns:
 *
 *     INTEGER
 *     FLOAT
 *     TRUE
 *     FALSE
 *     NIL
 *     NULL
 *     STRING
 *     CHAR
 *     IDENTIFIER
 *
 * among other tokens.
 *
 * Numeric literal syntax, Unicode identifiers and literal spelling therefore
 * remain outside this parser grammar.
 *
 * ============================================================================
 *
 * IMPORTANT TOKEN CONSISTENCY RULE
 * ============================================================================
 *
 * The canonical lexer token is:
 *
 *     IDENTIFIER
 *
 * This file intentionally uses IDENTIFIER directly.
 *
 * It does NOT introduce:
 *
 *     IDENT
 *     ID
 *     NAME
 *     SCALAR_IDENTIFIER
 *
 * or any other token alias.
 *
 * This prevents the scalar grammar from perpetuating token-name divergence
 * between grammar layers.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Scalar literals are PRIMARY VALUES.
 *
 * Arithmetic such as:
 *
 *     1 + 2
 *     x * 4
 *     a / b
 *     -42
 *     x < y
 *     a && b
 *
 * belongs to the authoritative expression grammar.
 *
 * This file therefore does NOT redefine:
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
 *
 * This avoids duplicate precedence hierarchies and parser ambiguity.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * A scalar VALUE is not the same thing as a scalar TYPE.
 *
 * Examples:
 *
 *     42
 *     3.14159
 *     true
 *     'a'
 *     "hello"
 *
 * are values.
 *
 * Their types are determined through the type system.
 *
 * Therefore this grammar does not define:
 *
 *     int
 *     float
 *     bool
 *     char
 *     string
 *
 * as scalar types.
 *
 * Type syntax remains owned by:
 *
 *     grammar/types/*
 *
 * ============================================================================
 *
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser establishes syntactic categories only.
 *
 * Semantic analysis determines:
 *
 *     - literal type;
 *     - numeric domain;
 *     - precision;
 *     - signedness;
 *     - exactness;
 *     - overflow policy;
 *     - underflow policy;
 *     - NaN/Infinity policy;
 *     - character validity;
 *     - string encoding semantics;
 *     - constant-folding eligibility;
 *     - compile-time evaluability;
 *     - coercion;
 *     - promotion;
 *     - conversion;
 *     - target representation.
 *
 * The parser must never silently make these decisions.
 *
 * ============================================================================
 *
 * EXACTNESS / PRECISION CONTRACT
 * ============================================================================
 *
 * Literal spelling must be preserved by the frontend.
 *
 * The grammar does not convert:
 *
 *     INTEGER
 *
 * into a Rust integer.
 *
 * It does not convert:
 *
 *     FLOAT
 *
 * into f32 or f64.
 *
 * It does not choose:
 *
 *     arbitrary precision
 *     fixed precision
 *     decimal precision
 *     machine precision
 *
 * Those decisions belong to semantic analysis and compilation.
 *
 * ============================================================================
 *
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Scalar values may be consumed by:
 *
 *     classical computation
 *     quantum control expressions
 *     quantum parameters
 *     hardware parameters
 *     HDL parameters
 *     timing expressions
 *     resource constraints
 *     capability requirements
 *     distributed control
 *     AI/data computation
 *     compile-time evaluation
 *
 * The scalar grammar therefore remains domain-neutral.
 *
 * A scalar value does NOT imply a classical-only execution target.
 *
 * ============================================================================
 *
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no embedded code;
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no evaluation;
 *     - performs no dynamic code execution;
 *     - performs no target selection;
 *     - performs no hardware access.
 *
 * Literal interpretation must occur in controlled compiler layers.
 *
 * ============================================================================
 */

parser grammar Scalar;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `scalar` is the stable public entry point for the classical scalar domain.
 *
 * It represents a scalar VALUE, not a scalar TYPE.
 *
 * ============================================================================
 */

scalar
    : scalarLiteral
    | scalarReference
    ;


/* ============================================================================
 * 2. SCALAR LITERAL
 * ============================================================================
 *
 * A scalar literal is a source-level literal whose semantic value occupies
 * one scalar value position.
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


/* ============================================================================
 * 3. INTEGER LITERAL
 * ============================================================================
 *
 * The lexical representation is owned by ZamaniLexer.
 *
 * This rule performs only parser-level classification.
 *
 * Examples accepted by the canonical lexer include forms such as:
 *
 *     0
 *     42
 *     1_000_000
 *     0xFF
 *     0b1010
 *     0o755
 *
 * No width is implied.
 *
 * ============================================================================
 */

integerLiteral
    : INTEGER
    ;


/* ============================================================================
 * 4. FLOATING-POINT LITERAL
 * ============================================================================
 *
 * The lexer owns the spelling.
 *
 * Examples include:
 *
 *     1.0
 *     0.5
 *     .5
 *     1e9
 *     1.5e-9
 *     1_000.25
 *
 * Precision and representation remain semantic decisions.
 *
 * ============================================================================
 */

floatingLiteral
    : FLOAT
    ;


/* ============================================================================
 * 5. BOOLEAN LITERAL
 * ============================================================================
 *
 * Boolean semantics are independent of machine representation.
 *
 * The parser does not decide whether the target represents a boolean using:
 *
 *     one bit
 *     one byte
 *     one word
 *     a predicate register
 *     a vector mask
 *
 * ============================================================================
 */

booleanLiteral
    : TRUE
    | FALSE
    ;


/* ============================================================================
 * 6. CHARACTER LITERAL
 * ============================================================================
 *
 * Character lexical spelling is owned by the lexer.
 *
 * Semantic validation belongs to the frontend/type system.
 *
 * This grammar deliberately does not assume:
 *
 *     ASCII
 *     UTF-8 byte width
 *     UTF-16 code-unit width
 *     UTF-32 representation
 *     fixed machine character size
 *
 * ============================================================================
 */

characterLiteral
    : CHAR
    ;


/* ============================================================================
 * 7. STRING LITERAL
 * ============================================================================
 *
 * A string is accepted here as a scalar source value because it occupies one
 * source-level scalar value position.
 *
 * This does NOT mean that strings are mathematically scalar values.
 *
 * The distinction is:
 *
 *     parser scalar-value category
 *
 * versus:
 *
 *     semantic numeric scalar category.
 *
 * Semantic analysis retains the actual type.
 *
 * ============================================================================
 */

stringLiteral
    : STRING
    ;


/* ============================================================================
 * 8. NULL-LIKE LITERALS
 * ============================================================================
 *
 * `nil` and `null` are both currently lexical literal spellings.
 *
 * Their semantic equivalence or distinction belongs to the type system.
 *
 * This grammar deliberately does not decide whether they mean:
 *
 *     nullable reference
 *     optional value
 *     unit-like value
 *     absence
 *     sentinel
 *     invalid value
 *
 * ============================================================================
 */

nullLiteral
    : NIL
    | NULL
    ;


/* ============================================================================
 * 9. SCALAR REFERENCE
 * ============================================================================
 *
 * A scalar reference is a source identifier used where semantic analysis may
 * establish that the referenced value is scalar.
 *
 * The grammar does not resolve the name.
 *
 * The identifier may refer to:
 *
 *     local variable
 *     parameter
 *     constant
 *     module item
 *     imported value
 *     compile-time value
 *     generated value
 *     future domain-defined scalar
 *
 * ============================================================================
 */

scalarReference
    : IDENTIFIER
    ;


/* ============================================================================
 * 10. NUMERIC SCALAR
 * ============================================================================
 *
 * Stable integration point for consumers that require a scalar known
 * syntactically to be numeric.
 *
 * This is deliberately limited to literal numeric syntax.
 *
 * An identifier may also resolve to a numeric scalar, but that is a semantic
 * property and therefore belongs to semantic analysis.
 *
 * ============================================================================
 */

numericScalar
    : integerLiteral
    | floatingLiteral
    ;


/* ============================================================================
 * 11. INTEGRAL SCALAR
 * ============================================================================
 *
 * This is a syntactic classification only.
 *
 * It does not imply:
 *
 *     signedness
 *     width
 *     precision
 *     representation
 *     overflow behavior
 *
 * ============================================================================
 */

integralScalar
    : integerLiteral
    ;


/* ============================================================================
 * 12. REAL-LIKE SCALAR
 * ============================================================================
 *
 * A floating literal is syntactically floating-point.
 *
 * Semantic analysis determines its actual numeric domain.
 *
 * ============================================================================
 */

realScalar
    : floatingLiteral
    ;


/* ============================================================================
 * 13. BOOLEAN SCALAR
 * ============================================================================
 */

booleanScalar
    : booleanLiteral
    ;


/* ============================================================================
 * 14. CHARACTER SCALAR
 * ============================================================================
 */

characterScalar
    : characterLiteral
    ;


/* ============================================================================
 * 15. TEXT SCALAR
 * ============================================================================
 *
 * This parser-level classification is intentionally separate from the
 * language's type system.
 *
 * ============================================================================
 */

textScalar
    : stringLiteral
    ;


/* ============================================================================
 * 16. NULL SCALAR
 * ============================================================================
 */

nullScalar
    : nullLiteral
    ;


/* ============================================================================
 * 17. COMPILE-TIME SCALAR VALUE
 * ============================================================================
 *
 * This rule establishes the parser boundary used by type-level and
 * compile-time facilities.
 *
 * It deliberately accepts:
 *
 *     literal
 *     identifier
 *
 * but does not evaluate them.
 *
 * Examples:
 *
 *     N
 *     Rows
 *     Cols
 *     1024
 *     1.0
 *     true
 *
 * Whether the referenced value is compile-time constant is semantic.
 *
 * ============================================================================
 */

compileTimeScalar
    : scalarLiteral
    | scalarReference
    ;


/* ============================================================================
 * 18. SCALAR VALUE LIST
 * ============================================================================
 *
 * This rule is provided for consumers such as:
 *
 *     scalar parameter lists
 *     compile-time argument lists
 *     value-level generic arguments
 *     configuration values
 *
 * It does not define function-call argument syntax.
 *
 * ============================================================================
 */

scalarValueList
    : scalar
      (COMMA scalar)*
      COMMA?
    ;


/* ============================================================================
 * 19. NUMERIC SCALAR LIST
 * ============================================================================
 *
 * No finite arity is encoded.
 *
 * ============================================================================
 */

numericScalarList
    : numericScalar
      (COMMA numericScalar)*
      COMMA?
    ;


/* ============================================================================
 * 20. COMPILE-TIME SCALAR LIST
 * ============================================================================
 */

compileTimeScalarList
    : compileTimeScalar
      (COMMA compileTimeScalar)*
      COMMA?
    ;


/* ============================================================================
 * 21. SCALAR VALUE GROUP
 * ============================================================================
 *
 * Parentheses belong to the expression/type layers when they affect
 * precedence or type structure.
 *
 * This rule exists only as a semantic grouping boundary for consumers that
 * explicitly need a scalar grouping.
 *
 * ============================================================================
 */

scalarGroup
    : LPAREN scalar RPAREN
    ;


/* ============================================================================
 * 22. SCALAR VALUE WITH GROUPING
 * ============================================================================
 *
 * This is intentionally NOT recursive expression parsing.
 *
 * It is only:
 *
 *     scalar
 *
 * or:
 *
 *     (scalar)
 *
 * Arithmetic such as `(x + y)` remains owned by the expression grammar.
 *
 * ============================================================================
 */

scalarPrimary
    : scalar
    | scalarGroup
    ;


/* ============================================================================
 * 23. SCALAR CONSTANT
 * ============================================================================
 *
 * A scalar constant is syntactically represented by a scalar literal.
 *
 * Whether an identifier denotes a constant is semantic.
 *
 * ============================================================================
 */

scalarConstant
    : scalarLiteral
    ;


/* ============================================================================
 * 24. SCALAR CONSTANT OR REFERENCE
 * ============================================================================
 *
 * This is useful to compile-time consumers without allowing general runtime
 * expressions to leak into the scalar grammar.
 *
 * ============================================================================
 */

scalarConstantOrReference
    : scalarConstant
    | scalarReference
    ;


/* ============================================================================
 * 25. SCALAR DOMAIN ENTRY FOR CLASSICAL COMPOSITION
 * ============================================================================
 *
 * Classical.g4 can consume this rule as the scalar-domain boundary.
 *
 * It should not duplicate scalarLiteral alternatives.
 *
 * ============================================================================
 */

classicalScalar
    : scalar
    ;


/* ============================================================================
 * 26. NUMERIC CLASSICAL SCALAR ENTRY
 * ============================================================================
 *
 * Stable entry point for numerical classical-domain consumers.
 *
 * ============================================================================
 */

classicalNumericScalar
    : numericScalar
    ;


/* ============================================================================
 * 27. COMPILE-TIME CLASSICAL SCALAR ENTRY
 * ============================================================================
 *
 * Stable entry point for compile-time/value-parameter consumers.
 *
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
 * Direct dependencies:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical tokens consumed:
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
 * No other grammar file is required to understand or generate this grammar.
 *
 * This makes Scalar.g4 independently completable.
 *
 * ============================================================================
 *
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * grammar/classical/classical.g4 should import:
 *
 *     Scalar
 *
 * and consume:
 *
 *     classicalScalar
 *     classicalNumericScalar
 *     classicalCompileTimeScalar
 *
 * It MUST NOT recreate:
 *
 *     integerLiteral
 *     floatingLiteral
 *     booleanLiteral
 *     characterLiteral
 *     stringLiteral
 *     nullLiteral
 *     scalarReference
 *
 * ============================================================================
 *
 * TYPE INTEGRATION
 * ============================================================================
 *
 * grammar/types/* may consume scalar-related value boundaries where a
 * type-level value is permitted.
 *
 * Scalar.g4 does NOT import Types.g4.
 *
 * This direction is intentional:
 *
 *     scalar
 *        |
 *        v
 *     type/value consumer
 *
 * rather than:
 *
 *     scalar <-> types
 *
 * This prevents a circular parser dependency.
 *
 * ============================================================================
 *
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The general expression grammar may use:
 *
 *     scalar
 *
 * as a primary-value category.
 *
 * Scalar.g4 does NOT import or redefine the general expression grammar.
 *
 * Therefore expression precedence remains centralized.
 *
 * ============================================================================
 *
 * AST INTEGRATION
 * ============================================================================
 *
 * Parser output must map to the existing frontend AST representation.
 *
 * Recommended semantic categories:
 *
 *     IntegerLiteral
 *     FloatingLiteral
 *     BooleanLiteral
 *     CharacterLiteral
 *     StringLiteral
 *     NullLiteral
 *     ScalarReference
 *
 * The grammar itself must not construct Rust AST objects.
 *
 * ============================================================================
 *
 * CLASSICAL IR INTEGRATION
 * ============================================================================
 *
 * Scalar.g4 does not depend on classical IR.
 *
 * The frontend/semantic layer maps scalar syntax into the canonical
 * classical/value representation.
 *
 * Literal representation must preserve enough source information to support:
 *
 *     exactness
 *     diagnostics
 *     constant evaluation
 *     deterministic lowering
 *     reproducibility
 *     provenance
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Scalar.g4 has NO dependency on quantum::ir.
 *
 * Classical scalar values may nevertheless be consumed by quantum syntax,
 * for example as gate parameters or classical control conditions.
 *
 * The dependency direction remains:
 *
 *     scalar syntax
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum frontend
 *          |
 *          v
 *     quantum::ir
 *
 * Scalar.g4 MUST NOT import quantum::ir or quantum parser semantics.
 *
 * ============================================================================
 *
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Scalar syntax is hardware-independent.
 *
 * A scalar parameter may eventually become:
 *
 *     CPU value
 *     GPU value
 *     FPGA parameter
 *     ASIC parameter
 *     QPU control parameter
 *     HDL constant
 *
 * The grammar must not distinguish those representations.
 *
 * ============================================================================
 *
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Scalar values may be used as:
 *
 *     symbolic dimensions
 *     thresholds
 *     timing values
 *     resource constraints
 *     capability parameters
 *     performance parameters
 *
 * Resource semantics remain outside this file.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Parsing the same token stream with the same grammar version must produce
 * the same parse structure.
 *
 * There are:
 *
 *     no actions
 *     no predicates
 *     no environment queries
 *     no target queries
 *     no runtime queries
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing literal spellings remain delegated to the canonical lexer.
 *
 * Adding future scalar semantic types must not require changing this grammar
 * when the value syntax remains one of the existing literal categories.
 *
 * New literal syntax requires a lexer/specification change first.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_*
 *     fixed scalar widths
 *     fixed precision
 *     fixed number of scalar arguments
 *     fixed identifier length
 *     fixed literal digit count
 *     machine architecture
 *     hardware identifier
 *     device identifier
 *     topology
 *     memory size
 *     CPU count
 *     GPU count
 *     QPU count
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     0
 *     42
 *     1_000_000
 *     0xFF
 *     0b1010
 *     0o755
 *     1.0
 *     .5
 *     1e9
 *     1.5e-9
 *     true
 *     false
 *     'a'
 *     "hello"
 *     nil
 *     null
 *     value
 *     _value
 *     scientific_value
 *
 * Negative:
 *
 *     malformed numeric literals
 *     malformed character literals
 *     malformed strings
 *     unterminated literals
 *     invalid token spellings
 *
 * Boundary:
 *
 *     very long integer literal
 *     very long floating literal
 *     very long identifier
 *     deeply nested scalar grouping within compiler resource policy
 *
 * Cross-domain:
 *
 *     classical scalar used as quantum parameter
 *     scalar used as HDL parameter
 *     scalar used as resource expression
 *     scalar used as compile-time dimension
 *     scalar used in distributed configuration
 *
 * Determinism:
 *
 *     identical source -> identical token classification and parse tree
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. It consumes only canonical lexer tokens.
 *
 *     2. It compiles independently as an ANTLR parser grammar.
 *
 *     3. It contains no embedded target-language code.
 *
 *     4. It contains no artificial machine/resource limits.
 *
 *     5. It does not duplicate general expression precedence.
 *
 *     6. It does not duplicate type syntax.
 *
 *     7. It does not duplicate lexical syntax.
 *
 *     8. It exposes stable scalar-domain parser entry points.
 *
 *     9. Classical.g4 can compose it without recreating scalar rules.
 *
 *    10. Type/value consumers can use its compile-time scalar boundary
 *        without importing the general expression grammar.
 *
 *    11. Semantic analysis can distinguish literal categories without
 *        requiring parser changes for hardware targets.
 *
 *    12. Quantum, HDL, hardware, distributed and future domains can consume
 *        scalar values without introducing machine-specific scalar syntax.
 *
 *    13. Positive, negative, boundary, cross-domain and determinism tests
 *        pass.
 *
 * ============================================================================
 */