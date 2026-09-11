/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Role:
 *     Canonical lexical grammar for Zamani.
 *
 * Parser:
 *     grammar/antlr/Core.g4
 *
 * Specification:
 *     grammar/spec/syntax.md
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Compiler safety:
 *     The Zamani compiler implementation MUST use safe Rust only.
 *     No Rust `unsafe` is required or permitted.
 *
 * ============================================================================
 *
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This lexer defines WHAT characters constitute Zamani tokens.
 *
 * It does NOT define:
 *
 *   - quantum hardware;
 *   - physical qubit limits;
 *   - classical register limits;
 *   - CPU architecture;
 *   - GPU architecture;
 *   - accelerator architecture;
 *   - QPU topology;
 *   - native hardware gate sets;
 *   - memory capacity;
 *   - tensor dimensions;
 *   - simulator limits;
 *   - backend-specific capabilities.
 *
 * Those concerns belong to semantic analysis, resource analysis, canonical
 * IR, optimization, scheduling and target lowering.
 *
 * ============================================================================
 *
 * PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     CoreParser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic/type/effect/resource analysis
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> quantum IR
 *       +--> classical IR
 *       +--> control/data IR
 *       +--> temporal IR
 *       +--> effect/resource metadata
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     target-independent lowering
 *       |
 *       v
 *     target-specific realization
 *
 * ============================================================================
 *
 * DESIGN REQUIREMENTS
 * ============================================================================
 *
 * 1. Longest lexical operators are recognized before their prefixes.
 *
 * 2. Keywords are reserved only where the canonical language specification
 *    declares them reserved.
 *
 * 3. Domain operations are NOT encoded as an exhaustive keyword list.
 *
 *    For example:
 *
 *        H
 *        CNOT
 *        U
 *        custom_gate
 *        vendor_operation
 *
 *    are not inherently lexical keywords.
 *
 *    Quantum operation identity belongs to semantic resolution.
 *
 * 4. Numeric literals do not encode machine widths.
 *
 * 5. Identifiers support Unicode.
 *
 * 6. Unicode normalization is NOT silently performed by the lexer.
 *    If Zamani adopts a normalization policy, it belongs to the canonical
 *    identifier/name-resolution specification.
 *
 * 7. Comments never become semantic source tokens.
 *
 * 8. Documentation comments are preserved on a dedicated hidden channel so
 *    tooling can recover them without making them syntactically significant.
 *
 * 9. Unterminated strings, characters and block comments must be rejected
 *    deterministically by the generated lexer/runtime.
 *
 * 10. The grammar does not use artificial finite limits to achieve scalability.
 *
 * ============================================================================
 */

lexer grammar ZamaniLexer;


/* ============================================================================
 * KEYWORDS — CORE LANGUAGE
 * ========================================================================== */

FN          : 'fn' ;
LET         : 'let' ;
VAR         : 'var' ;
MUT         : 'mut' ;
CONST       : 'const' ;

RETURN      : 'return' ;
IF          : 'if' ;
ELSE        : 'else' ;
FOR         : 'for' ;
IN          : 'in' ;
WHILE       : 'while' ;
LOOP        : 'loop' ;
BREAK       : 'break' ;
CONTINUE    : 'continue' ;
MATCH       : 'match' ;
CASE        : 'case' ;
WHEN        : 'when' ;
YIELD       : 'yield' ;


/* ============================================================================
 * MODULE / PACKAGE SYSTEM
 * ========================================================================== */

MODULE      : 'module' ;
IMPORT      : 'import' ;
EXPORT      : 'export' ;
USE         : 'use' ;
FROM        : 'from' ;
AS          : 'as' ;
PACKAGE     : 'package' ;


/* ============================================================================
 * TYPES / DATA MODEL
 * ========================================================================== */

TYPE        : 'type' ;
STRUCT      : 'struct' ;
ENUM        : 'enum' ;
TRAIT       : 'trait' ;
IMPL        : 'impl' ;
CLASS       : 'class' ;
INTERFACE   : 'interface' ;
RECORD      : 'record' ;


/* ============================================================================
 * VISIBILITY / OBJECT MODEL
 * ========================================================================== */

PUBLIC      : 'public' ;
PUB         : 'pub' ;
PRIVATE     : 'private' ;
PROTECTED   : 'protected' ;
INTERNAL    : 'internal' ;

STATIC      : 'static' ;
OVERRIDE    : 'override' ;
VIRTUAL     : 'virtual' ;
ABSTRACT    : 'abstract' ;
FINAL       : 'final' ;

EXTENDS     : 'extends' ;
IMPLEMENTS  : 'implements' ;

THIS        : 'this' ;
SELF        : 'self' ;
SUPER       : 'super' ;
NEW         : 'new' ;


/* ============================================================================
 * GENERICS / CONSTRAINTS
 * ========================================================================== */

WHERE       : 'where' ;


/* ============================================================================
 * ASYNCHRONOUS / CONCURRENT COMPUTATION
 * ========================================================================== */

ASYNC       : 'async' ;
AWAIT       : 'await' ;
SPAWN       : 'spawn' ;
PARALLEL    : 'parallel' ;


/* ============================================================================
 * SAFETY / ERROR HANDLING
 * ========================================================================== */

UNSAFE      : 'unsafe' ;
TRY         : 'try' ;
CATCH       : 'catch' ;
FINALLY     : 'finally' ;
THROW       : 'throw' ;
HANDLE      : 'handle' ;


/* ============================================================================
 * EFFECT SYSTEM
 * ========================================================================== */

EFFECT      : 'effect' ;
EFFECTS     : 'effects' ;
WITH        : 'with' ;

REQUIRES    : 'requires' ;
ENSURES     : 'ensures' ;
INVARIANT   : 'invariant' ;


/* ============================================================================
 * QUANTUM COMPUTATION
 *
 * IMPORTANT:
 *
 * This lexer intentionally does NOT define a fixed quantum gate vocabulary.
 *
 * `H`, `X`, `CNOT`, `U`, `controlled`, custom operations, symbolic operations,
 * calibrated operations, logical operations and future operations can remain
 * identifiers unless the language specification explicitly makes a particular
 * spelling syntactically reserved.
 *
 * The compiler can therefore support:
 *
 *     apply H to q;
 *     apply controlled(X) from c to t;
 *     apply U(theta, phi, lambda) to q;
 *
 * without coupling the lexical layer to a particular hardware generation.
 * ========================================================================== */

QUANTUM     : 'quantum' ;
CIRCUIT     : 'circuit' ;
QUBIT       : 'Qubit' ;

APPLY       : 'apply' ;
MEASURE     : 'measure' ;
RESET       : 'reset' ;
BARRIER     : 'barrier' ;
CONTROL     : 'control' ;
ADJOINT     : 'adjoint' ;
INVERSE     : 'inverse' ;
OBSERVE     : 'observe' ;


/* ============================================================================
 * NANO / AGENT COMPUTATION
 * ========================================================================== */

NANO        : 'nano' ;
AGENT       : 'agent' ;
PERFORM     : 'perform' ;
LEARN       : 'learn' ;
INFER       : 'infer' ;


/* ============================================================================
 * TEMPORAL / ZAMANI / SANKOFA
 * ========================================================================== */

MTS         : 'mts' ;
ZAMANI      : 'zamani' ;
SASA        : 'sasa' ;

REMEMBER    : 'remember' ;
RECALL      : 'recall' ;
WISDOM      : 'wisdom' ;


/* ============================================================================
 * LANGUAGE / METAPROGRAMMING
 * ========================================================================== */

LANGUAGE    : 'language' ;
MACRO       : 'macro' ;
EXTERN      : 'extern' ;


/* ============================================================================
 * TYPE-LEVEL KEYWORDS
 * ========================================================================== */

RESULT      : 'Result' ;
NEVER       : 'Never' ;

PI_KEYWORD  : 'Pi' ;
SIGMA_KEYWORD
            : 'Sigma' ;


/* ============================================================================
 * TYPE / SEMANTIC MODIFIERS
 * ========================================================================== */

LINEAR      : 'linear' ;
AFFINE      : 'affine' ;
PURE        : 'pure' ;
IMMUTABLE   : 'immutable' ;
INLINE      : 'inline' ;
VOLATILE    : 'volatile' ;

SIM         : 'sim' ;
VECTORIZED  : 'vectorized' ;
GPU         : 'gpu' ;


/* ============================================================================
 * LOGICAL / TYPE RELATION KEYWORDS
 * ========================================================================== */

IS          : 'is' ;
AND         : 'and' ;
OR          : 'or' ;
NOT         : 'not' ;


/* ============================================================================
 * BASIC BUILT-IN TYPE NAMES
 *
 * These are reserved because the current implementation exposes them as
 * lexical keywords. Their semantic representation remains a compiler concern.
 * ========================================================================== */

VOID        : 'void' ;
INT         : 'int' ;
FLOAT_TYPE  : 'float' ;
BOOL_TYPE   : 'bool' ;
STR_TYPE    : 'str' ;
STRING_TYPE : 'string' ;
CHAR_TYPE   : 'char' ;


/* ============================================================================
 * BUILT-IN OPERATION WORDS
 *
 * These are intentionally limited to language primitives whose lexical
 * distinction is useful to the parser.
 *
 * Ordinary library functionality should remain identifiers.
 * ========================================================================== */

PRINT       : 'print' ;
PRINTLN     : 'println' ;
ASSERT      : 'assert' ;
PANIC       : 'panic' ;
LEN         : 'len' ;
SIZEOF      : 'sizeof' ;


/* ============================================================================
 * OMNIVERSAL / SYSTEM DECLARATION VOCABULARY
 *
 * These remain reserved because they already exist in the current Zamani
 * lexical design. Their semantics MUST NOT be implemented by the lexer.
 * ========================================================================== */

OMNIVERSAL  : 'omniversal' ;
SIMULATE    : 'simulate' ;
SYNTHESIZE  : 'synthesize' ;
DEPLOY      : 'deploy' ;

ALIGNMENT   : 'alignment' ;
CONTAINMENT : 'containment' ;
TRUST       : 'trust' ;
KNOWLEDGE   : 'knowledge' ;
GENERATIVE  : 'generative' ;
SOVEREIGNTY : 'sovereignty' ;
GOAL        : 'goal' ;
BIONANO     : 'bionano' ;
REALITY     : 'reality' ;
NLP         : 'nlp' ;
SYSTEM      : 'system' ;


/* ============================================================================
 * ADVANCED SYSTEM VOCABULARY
 *
 * Kept lexically distinct for compatibility with the existing language
 * surface. Implementations must not treat these as backend instructions.
 * ========================================================================== */

ASI         : 'asi' ;
AESI        : 'aesi' ;
ASESI       : 'asesi' ;

ADMIN       : 'admin' ;
PAYMENT     : 'payment' ;
GATEWAY     : 'gateway' ;
GRAPHICS    : 'graphics' ;
VIDEO       : 'video' ;
ADJUST      : 'adjust' ;
VERSIONING  : 'versioning' ;
COPYRIGHT   : 'copyright' ;
NOTICE      : 'notice' ;
LEGAL       : 'legal' ;
ACTION      : 'action' ;
TAILOR      : 'tailor' ;
BUSINESS    : 'business' ;


/* ============================================================================
 * QUANTUM / COMPUTATION SEMANTIC WORDS FROM EXISTING FRONTEND
 * ========================================================================== */

ENTANGLE    : 'entangle' ;
NOISE       : 'noise' ;
FIDELITY    : 'fidelity' ;
SURFACE     : 'surface' ;
CODE        : 'code' ;
LOGICAL     : 'logical' ;
PARITY      : 'parity' ;


/* ============================================================================
 * BOOLEAN / NULL LITERALS
 * ========================================================================== */

TRUE        : 'true' ;
FALSE       : 'false' ;
NIL         : 'nil' ;
NULL        : 'null' ;


/* ============================================================================
 * INTEGER LITERALS
 *
 * Supported forms:
 *
 *     0
 *     42
 *     1_000_000
 *     0xFF
 *     0b1010
 *     0o755
 *
 * No machine-width assumptions are encoded.
 * ========================================================================== */

INTEGER
    : DEC_DIGIT (DEC_DIGIT | '_')*
    | '0' [xX] HEX_DIGIT (HEX_DIGIT | '_')*
    | '0' [bB] BIN_DIGIT (BIN_DIGIT | '_')*
    | '0' [oO] OCT_DIGIT (OCT_DIGIT | '_')*
    ;


/* ============================================================================
 * FLOATING-POINT LITERALS
 *
 * Accepted examples:
 *
 *     1.0
 *     0.5
 *     .5
 *     1e9
 *     1.5e-9
 *     1_000.25
 *
 * Exact range/precision belongs to semantic typing and target lowering.
 * ========================================================================== */

FLOAT
    : DEC_DIGIT (DEC_DIGIT | '_')*
      DOT
      DEC_DIGIT (DEC_DIGIT | '_')*
      EXPONENT?
    | DOT
      DEC_DIGIT (DEC_DIGIT | '_')*
      EXPONENT?
    | DEC_DIGIT (DEC_DIGIT | '_')*
      EXPONENT
    ;

fragment EXPONENT
    : [eE] [+-]? DEC_DIGIT (DEC_DIGIT | '_')*
    ;


/* ============================================================================
 * STRING LITERALS
 *
 * Strings are UTF-8 source text represented through the ANTLR character
 * stream. Runtime/compiler ownership of encoding and allocation belongs to
 * the frontend.
 * ========================================================================== */

STRING
    : '"' (ESCAPE_SEQUENCE | ~["\\\r\n])* '"'
    ;


/* ============================================================================
 * CHARACTER LITERALS
 * ========================================================================== */

CHAR
    : '\'' (ESCAPE_SEQUENCE | ~['\\\r\n]) '\''
    ;


/* ============================================================================
 * ESCAPES
 * ========================================================================== */

fragment ESCAPE_SEQUENCE
    : '\\'
      (
          ['"\\bfnrt]
        | 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
      )
    ;


/* ============================================================================
 * QUANTUM STATE LITERALS
 *
 * Examples:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *
 * This notation is lexical only.
 *
 * It does not imply:
 *
 *     - one physical qubit;
 *     - a particular simulator;
 *     - a particular basis representation;
 *     - a fixed numerical precision.
 * ========================================================================== */

QUANTUM_LITERAL
    : '|' QUANTUM_STATE_SYMBOL '⟩'
    ;

fragment QUANTUM_STATE_SYMBOL
    : '0'
    | '1'
    | '+'
    | '-'
    ;


/* ============================================================================
 * NANO ANNOTATIONS
 *
 * The existing implementation identifies forms such as:
 *
 *     @atom
 *     @molecule
 *
 * The lexical representation is deliberately generic.
 *
 * Semantic interpretation belongs to the annotation system.
 *
 * NOTE:
 * `@identifier` is also a general attribute syntax. The parser can therefore
 * consume NANO_ANNOTATION or AT + IDENTIFIER depending on the canonical
 * compatibility policy.
 * ========================================================================== */

NANO_ANNOTATION
    : '@' ID_START ID_CONTINUE*
    ;


/* ============================================================================
 * MULTI-TIMELINE LITERALS
 *
 * Existing implementation terminology includes MTSLiteral.
 *
 * Rather than hard-code a timestamp grammar into the core lexer, the lexical
 * token captures the complete bracketed literal and semantic validation
 * determines whether the payload is a valid temporal value.
 *
 * Example:
 *
 *     mts[...]
 *
 * The payload remains source text at this layer.
 * ========================================================================== */

MTS_LITERAL
    : 'mts[' MTS_LITERAL_BODY* ']'
    ;

fragment MTS_LITERAL_BODY
    : ~[\r\n\]]
    ;


/* ============================================================================
 * IDENTIFIERS
 *
 * ASCII:
 *
 *     a-z
 *     A-Z
 *     _
 *
 * Unicode BMP characters are also accepted.
 *
 * Digits may not begin an identifier.
 *
 * Unicode supplementary-plane identifier support should be implemented through
 * the canonical Unicode identifier policy in the compiler if required by the
 * final Zamani specification.
 * ========================================================================== */

IDENTIFIER
    : ID_START ID_CONTINUE*
    ;

fragment ID_START
    : [a-zA-Z_]
    | '\u0080'..'\uD7FF'
    | '\uE000'..'\uFFFF'
    ;

fragment ID_CONTINUE
    : ID_START
    | [0-9]
    ;


/* ============================================================================
 * MULTI-CHARACTER OPERATORS
 *
 * IMPORTANT:
 *
 * Longer operators MUST appear before their prefixes.
 * ========================================================================== */

DOT_DOT_EQ      : '..=' ;
DOT_DOT         : '..' ;

PLUS_ASSIGN     : '+=' ;
MINUS_ASSIGN    : '-=' ;
STAR_ASSIGN     : '*=' ;
SLASH_ASSIGN    : '/=' ;

SHIFT_LEFT      : '<<' ;
SHIFT_RIGHT     : '>>' ;

EQ_EQ           : '==' ;
NOT_EQ          : '!=' ;
LE              : '<=' ;
GE              : '>=' ;

AND_AND         : '&&' ;
OR_OR           : '||' ;

DOUBLE_COLON    : '::' ;

FAT_ARROW       : '=>' ;
ARROW           : '->' ;


/* ============================================================================
 * SINGLE-CHARACTER OPERATORS
 * ========================================================================== */

ASSIGN          : '=' ;

PLUS            : '+' ;
MINUS           : '-' ;
STAR            : '*' ;
SLASH           : '/' ;
PERCENT         : '%' ;

NOT_OPERATOR    : '!' ;
TILDE           : '~' ;

AMPERSAND       : '&' ;
PIPE            : '|' ;
CARET           : '^' ;

QUESTION        : '?' ;


/* ============================================================================
 * DELIMITERS
 * ========================================================================== */

LPAREN          : '(' ;
RPAREN          : ')' ;

LBRACE          : '{' ;
RBRACE          : '}' ;

LBRACKET        : '[' ;
RBRACKET        : ']' ;

COMMA           : ',' ;
DOT             : '.' ;
COLON           : ':' ;
SEMICOLON       : ';' ;

AT              : '@' ;

UNDERSCORE      : '_' ;


/* ============================================================================
 * DOCUMENTATION COMMENTS
 *
 * Documentation comments are hidden from the parser but retained in the token
 * stream so IDEs, documentation generators and tooling can recover them.
 *
 * The parser therefore remains independent of documentation syntax.
 * ========================================================================== */

DOC_LINE
    : '///' ~[\r\n]* -> channel(HIDDEN)
    ;

DOC_BLOCK
    : '/**' .*? '*/' -> channel(HIDDEN)
    ;


/* ============================================================================
 * ORDINARY COMMENTS
 * ========================================================================== */

LINE_COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;

BLOCK_COMMENT
    : '/*' .*? '*/' -> channel(HIDDEN)
    ;


/* ============================================================================
 * WHITESPACE
 * ========================================================================== */

WS
    : [ \t\r\n\u000B\u000C]+ -> channel(HIDDEN)
    ;


/* ============================================================================
 * NUMERIC FRAGMENTS
 * ========================================================================== */

fragment DEC_DIGIT
    : [0-9]
    ;

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;

fragment BIN_DIGIT
    : [01]
    ;

fragment OCT_DIGIT
    : [0-7]
    ;