lexer grammar ZamaniOperators;

// =============================================================================
// Zamani Universal Programming Language
// grammar/lexer/operators.g4
//
// PURPOSE
// -------
// Authoritative lexical definitions for Zamani operators.
//
// This grammar contains ONLY operator tokens. It deliberately does not define:
//
//   - keywords;
//   - identifiers;
//   - literals;
//   - punctuation;
//   - comments;
//   - whitespace;
//   - annotations;
//   - AST nodes;
//   - expression precedence;
//   - type semantics;
//   - classical IR;
//   - quantum IR;
//   - QEC;
//   - ZQN;
//   - optimization;
//   - routing;
//   - scheduling;
//   - hardware discovery;
//   - runtime behavior;
//   - machine-specific limits.
//
// ARCHITECTURAL RULE
// ------------------
// Lexer:
//     characters -> operator tokens
//
// Parser:
//     operator tokens -> syntactic expression/operator structure
//
// Semantic analysis:
//     operator structure -> language meaning/type/effect information
//
// IR lowering:
//     semantic operation -> canonical IR
//
// Backends:
//     canonical IR -> target-specific implementation
//
// The lexer therefore MUST NOT decide what an operator means for a CPU,
// GPU, FPGA, ASIC, quantum processor, distributed machine, or future target.
//
// =============================================================================
//
// FILE CONTRACT
// ------------
//
// Owns:
//   - lexical recognition of Zamani operator spellings;
//   - stable operator token names;
//   - maximal-munch ordering where necessary;
//   - operator spellings that are part of the language surface.
//
// Does not own:
//   - operator precedence;
//   - associativity;
//   - overload resolution;
//   - type checking;
//   - implicit conversions;
//   - constant folding;
//   - algebraic laws;
//   - quantum gate semantics;
//   - hardware instructions;
//   - target capabilities;
//   - resource requirements;
//   - machine sizes.
//
// Inputs:
//   - source characters supplied by the canonical Zamani lexer.
//
// Outputs:
//   - operator tokens consumed by parser grammars.
//
// Upstream:
//   - canonical lexer assembly.
//
// Downstream:
//   - expression grammars;
//   - statement/declaration grammars where operators occur;
//   - pattern grammars where applicable;
//   - compile-time expression grammars;
//   - macro/metaprogramming grammars.
//
// AST contract:
//   - parser/AST layers preserve the token kind and source span;
//   - this file creates no AST nodes.
//
// Semantic contract:
//   - token spelling identifies syntax only;
//   - semantic interpretation is downstream.
//
// IR contract:
//   - no direct IR dependency.
//
// Runtime contract:
//   - no runtime dependency.
//
// Scalability contract:
//   - no machine dimensions, capacities, topology, device IDs, or resource
//     counts occur here.
//
// Rust contract:
//   - generated parser/lexer integration targets Rust 1.97 / 1.97.1;
//   - generated/runtime Rust must remain safe Rust;
//   - this grammar itself contains no Rust and introduces no unsafe code.
//
// =============================================================================
// 1. MULTI-CHARACTER OPERATORS
//
// These MUST precede their shorter prefixes in the canonical lexer assembly.
//
// Examples:
//
//   ...  must not become . . .
//   ..=  must not become .. +
//   ..   must not become . .
//   ->   must not become - >
//   =>   must not become = >
//   ::   must not become : :
//   ==   must not become = =
//   !=   must not become ! =
//   <=   must not become < =
//   >=   must not become > =
//   &&   must not become & &
//   ||   must not become | |
//   <<   must not become < <
//   >>   must not become > >
//   +=   must not become + =
//   -=   must not become - =
//   *=   must not become * =
//   /=   must not become / =
//   %=   must not become % =
//   &=   must not become & =
//   |=   must not become | =
//   ^=   must not become ^ =
//   ++   must not become + +
//   --   must not become - -
//   ?.   must not become ? .
//   ??   must not become ? ?
//
// =============================================================================

// -----------------------------------------------------------------------------
// Range / variadic operators
// -----------------------------------------------------------------------------

ELLIPSIS
    : '...'
    ;

DOT_DOT_EQ
    : '..='
    ;

DOT_DOT
    : '..'
    ;

// -----------------------------------------------------------------------------
// Function / type / control-flow arrows
// -----------------------------------------------------------------------------

THIN_ARROW
    : '->'
    ;

FAT_ARROW
    : '=>'
    ;

// -----------------------------------------------------------------------------
// Namespace / path operator
// -----------------------------------------------------------------------------

DOUBLE_COLON
    : '::'
    ;

// -----------------------------------------------------------------------------
// Equality / comparison
// -----------------------------------------------------------------------------

EQUAL_EQUAL
    : '=='
    ;

NOT_EQUAL
    : '!='
    ;

LESS_EQUAL
    : '<='
    ;

GREATER_EQUAL
    : '>='
    ;

// -----------------------------------------------------------------------------
// Short-circuit logical operators
// -----------------------------------------------------------------------------

LOGICAL_AND
    : '&&'
    ;

LOGICAL_OR
    : '||'
    ;

// -----------------------------------------------------------------------------
// Bit-shift operators
// -----------------------------------------------------------------------------

LEFT_SHIFT
    : '<<'
    ;

RIGHT_SHIFT
    : '>>'
    ;

// -----------------------------------------------------------------------------
// Compound assignment operators
// -----------------------------------------------------------------------------

PLUS_ASSIGN
    : '+='
    ;

MINUS_ASSIGN
    : '-='
    ;

STAR_ASSIGN
    : '*='
    ;

SLASH_ASSIGN
    : '/='
    ;

PERCENT_ASSIGN
    : '%='
    ;

AMP_ASSIGN
    : '&='
    ;

PIPE_ASSIGN
    : '|='
    ;

CARET_ASSIGN
    : '^='
    ;

// -----------------------------------------------------------------------------
// Increment / decrement
// -----------------------------------------------------------------------------

INCREMENT
    : '++'
    ;

DECREMENT
    : '--'
    ;

// -----------------------------------------------------------------------------
// Optional / null-propagation operators
// -----------------------------------------------------------------------------

QUESTION_DOT
    : '?.'
    ;

NULL_COALESCE
    : '??'
    ;


// =============================================================================
// 2. SINGLE-CHARACTER OPERATORS
//
// Punctuation characters such as parentheses, braces, brackets, comma,
// semicolon and colon are NOT owned here.
//
// IMPORTANT:
//     DOUBLE_COLON is an operator and therefore belongs here.
//     COLON is punctuation and belongs in punctuation.g4.
//
// Likewise:
//
//     DOT_DOT / ELLIPSIS -> operators
//     DOT                -> punctuation
//
// This distinction prevents lexical ownership collisions.
// =============================================================================

// -----------------------------------------------------------------------------
// Arithmetic
// -----------------------------------------------------------------------------

PLUS
    : '+'
    ;

MINUS
    : '-'
    ;

STAR
    : '*'
    ;

SLASH
    : '/'
    ;

MODULO
    : '%'
    ;

// -----------------------------------------------------------------------------
// Assignment
// -----------------------------------------------------------------------------

ASSIGN
    : '='
    ;

// -----------------------------------------------------------------------------
// Bitwise
// -----------------------------------------------------------------------------

AMPERSAND
    : '&'
    ;

PIPE
    : '|'
    ;

CARET
    : '^'
    ;

TILDE
    : '~'
    ;

// -----------------------------------------------------------------------------
// Comparison
//
// The multi-character <= and >= rules above take precedence over these
// single-character forms in the canonical lexer.
// -----------------------------------------------------------------------------

LESS
    : '<'
    ;

GREATER
    : '>'
    ;

// -----------------------------------------------------------------------------
// Logical / unary
//
// ! is an operator, not punctuation.
// ? is punctuation unless the language later assigns it an operator role.
// The canonical ownership decision is kept here explicit so another lexer
// file cannot accidentally define EXCLAMATION again.
// -----------------------------------------------------------------------------

NOT
    : '!'
    ;


// =============================================================================
// 3. OPERATOR OWNERSHIP NOTES
// =============================================================================
//
// The following characters/tokens intentionally do NOT belong to this file:
//
//     ( )       -> punctuation.g4
//     { }       -> punctuation.g4
//     [ ]       -> punctuation.g4
//     ,         -> punctuation.g4
//     ;         -> punctuation.g4
//     :         -> punctuation.g4
//     .         -> punctuation.g4
//     ?         -> punctuation.g4 unless explicitly promoted by a future
//                  language-version decision
//
// The following are operator tokens despite resembling punctuation:
//
//     ->        -> THIN_ARROW
//     =>        -> FAT_ARROW
//     ::        -> DOUBLE_COLON
//     ..        -> DOT_DOT
//     ..=       -> DOT_DOT_EQ
//     ...       -> ELLIPSIS
//     ?.        -> QUESTION_DOT
//     ??        -> NULL_COALESCE
//
// =============================================================================
//
// 4. LEXICAL MAXIMAL-MUNCH REQUIREMENT
// =============================================================================
//
// The canonical lexer assembly MUST ensure that multi-character operators
// are recognized as complete tokens.
//
// This is especially important for:
//
//     ...    ..=    ..
//     ->     =>     ::
//     ==     !=     <=     >=
//     &&     ||     <<     >>
//     +=     -=     *=     /=     %=
//     &=     |=     ^=
//     ++     --
//     ?.     ??
//
// A parser must never have to reconstruct a compound operator from separate
// character tokens.
//
// For example:
//
//     a >= b
//
// MUST produce:
//
//     IDENTIFIER GREATER_EQUAL IDENTIFIER
//
// rather than:
//
//     IDENTIFIER GREATER ASSIGN IDENTIFIER
//
// Likewise:
//
//     x += y
//
// MUST produce:
//
//     IDENTIFIER PLUS_ASSIGN IDENTIFIER
//
// rather than:
//
//     IDENTIFIER PLUS ASSIGN IDENTIFIER
//
// =============================================================================
//
// 5. PRECEDENCE IS NOT DEFINED HERE
// =============================================================================
//
// This file MUST NOT encode:
//
//     multiplication > addition
//     comparison > equality
//     logical-and > logical-or
//     assignment associativity
//
// Those are parser/semantic contracts.
//
// For example, the parser may define:
//
//     multiplicativeExpression
//     additiveExpression
//     shiftExpression
//     comparisonExpression
//     equalityExpression
//     bitwiseExpression
//     logicalExpression
//     assignmentExpression
//
// without changing this lexer.
//
// This separation allows the language's semantic operator model to evolve
// without changing lexical ownership.
//
// =============================================================================
//
// 6. OPERATOR OVERLOADING
// =============================================================================
//
// This file does not decide whether:
//
//     +
//     -
//     *
//     /
//     %
//     <<
//     >>
//     &
//     |
//     ^
//
// operate on:
//
//     integers
//     floating-point values
//     vectors
//     matrices
//     tensors
//     symbolic values
//     user-defined types
//     hardware values
//     quantum/classical boundary values
//     future extensible types
//
// That belongs to semantic analysis and type checking.
//
// Therefore no target-specific operator meaning is embedded here.
//
// =============================================================================
//
// 7. QUANTUM INTEGRATION
// =============================================================================
//
// Quantum syntax may reuse ordinary operators for expressions:
//
//     +
//     -
//     *
//     /
//     ==
//     !=
//     -> 
//     => 
//
// Quantum-specific semantic operations must be represented by the quantum
// parser/semantic layer and eventually lowered into the canonical
// quantum::ir.
//
// This file must NOT define:
//
//     H
//     X
//     CX
//     measurement semantics
//     qubit counts
//     topology
//     coupling maps
//     physical gate durations
//     calibration data
//     error rates
//
// Those belong to their respective language/semantic/backend layers.
//
// In particular, this lexer must never contain:
//
//     MAX_QUBITS
//     MAX_GATES
//     DEVICE_ID
//     q[0]
//     q[1]
//
// or any equivalent machine-specific restriction.
//
// =============================================================================
//
// 8. HDL / HARDWARE INTEGRATION
// =============================================================================
//
// Hardware-oriented expressions may reuse:
//
//     =
//     +=
//     -=
//     &
//     |
//     ^
//     ~
//     <<
//     >>
//     ==
//     !=
//     <
//     >
//     <=
//     >=
//
// Hardware meaning belongs to HDL/hardware semantic analysis.
//
// This file must not encode:
//
//     register width
//     bus width
//     FPGA family
//     ASIC family
//     clock frequency
//     number of ports
//     number of devices
//     physical address
//     topology
//
// Such properties are represented through semantic declarations, target
// descriptions, capabilities, resources, or compilation context.
//
// =============================================================================
//
// 9. CLASSICAL / NUMERICAL INTEGRATION
// =============================================================================
//
// Arithmetic operators remain syntax-level constructs.
//
// Their semantics can be applied to scalar and aggregate values without
// requiring new lexer tokens for every numerical domain.
//
// For example, a future tensor system should not require:
//
//     TENSOR_PLUS
//     MATRIX_PLUS
//     VECTOR_PLUS
//
// unless there is a genuine syntactic distinction.
//
// The ordinary PLUS token is therefore intentionally reusable.
//
// =============================================================================
//
// 10. FUTURE EXTENSIBILITY
// =============================================================================
//
// New operator spellings may be added only through the language-versioning
// process.
//
// A future extension MUST:
//
//   1. choose an unambiguous spelling;
//   2. define its lexical ownership here;
//   3. define parser usage separately;
//   4. define precedence/associativity separately;
//   5. define semantic meaning separately;
//   6. define AST representation;
//   7. define diagnostics;
//   8. define compatibility behavior;
//   9. add positive/negative/boundary tests;
//  10. verify that no existing operator becomes ambiguous.
//
// Domain-specific extensions should prefer dialect/operator-registration
// mechanisms over permanently hard-coding vendor-specific operators into the
// universal core language.
//
// =============================================================================
//
// 11. DETERMINISM
// =============================================================================
//
// For identical source text and identical language version, the canonical
// lexer must emit the same operator token sequence.
//
// Operator tokenization must not depend on:
//
//     machine size
//     CPU count
//     GPU count
//     FPGA count
//     quantum hardware
//     runtime state
//     network state
//     scheduling
//     calibration
//     backend selection
//
// =============================================================================
//
// 12. ERROR HANDLING
// =============================================================================
//
// This file recognizes valid operator spellings.
//
// Invalid or unsupported operator sequences must be diagnosed by the canonical
// lexer/error infrastructure.
//
// Do not introduce catch-all operator rules here such as:
//
//     OPERATOR : . ;
//
// Such a rule would hide spelling errors and make diagnostics less precise.
//
// =============================================================================
//
// 13. COMPATIBILITY
// =============================================================================
//
// Existing parser consumers already reference named tokens including:
//
//     PLUS
//     MINUS
//     STAR
//     SLASH
//     MODULO
//     PLUS_ASSIGN
//     MINUS_ASSIGN
//     STAR_ASSIGN
//     SLASH_ASSIGN
//
// These token names MUST remain stable during migration.
//
// The existing parser also uses operator precedence independently of lexical
// recognition. Therefore migrating these rules from the monolithic token
// grammar into this modular file must preserve token identity.
//
// =============================================================================
//
// 14. REQUIRED MIGRATION
// =============================================================================
//
// The current grammar/lexer/tokens.g4 contains operator definitions alongside
// many unrelated lexical definitions.
//
// During modularization:
//
//     tokens.g4
//          |
//          +--> keywords.g4
//          +--> identifiers.g4
//          +--> literals.g4
//          +--> operators.g4       <-- this file
//          +--> punctuation.g4
//          +--> comments.g4
//          +--> ...
//
// Operator definitions MUST have exactly ONE canonical lexical owner.
//
// Do NOT temporarily leave duplicate definitions in both:
//
//     tokens.g4
//     operators.g4
//
// because ANTLR will then have competing token definitions and the language
// will no longer have a single lexical authority.
//
// The migration must preserve the public token names consumed by:
//
//     grammar/antlr/ZamaniParser.g4
//     grammar/antlr/Meta.g4
//     grammar/antlr/Types.g4
//     other parser grammars
//     generated parser integration
//
// The repository parser currently consumes PLUS/MINUS and other operator
// tokens, so this migration must be treated as a compatibility-preserving
// lexer refactor, not a token-renaming exercise.
//
// =============================================================================
//
// 15. NO DEPENDENCY ON RUNTIME OR HARDWARE
// =============================================================================
//
// This grammar MUST remain usable when no target exists yet.
//
// It must be possible to lex:
//
//     + - * / %
//     == != <= >=
//     && ||
//     << >>
//     += -= *= /= %=
//     & | ^ ~
//     -> => ::
//     .. ..= ...
//
// without knowing:
//
//     target CPU
//     target GPU
//     target FPGA
//     target ASIC
//     quantum backend
//     node count
//     cluster topology
//     available memory
//     runtime capabilities
//
// =============================================================================
//
// 16. SOURCE COMPATIBILITY
// =============================================================================
//
// Existing valid Zamani programs using the operator spellings above must
// continue to tokenize into the same stable token categories after the
// modularization.
//
// Any intentional language change requires:
//
//     language version update
//     compatibility documentation
//     migration guidance
//     regression tests
//
// =============================================================================
//
// END OF FILE
// =============================================================================