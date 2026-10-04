/**
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * FILE:
 *   grammar/lexer/lexer.g4
 *
 * GRAMMAR:
 *   lexer
 *
 * STATUS:
 *   Canonical lexical implementation candidate.
 *
 * LANGUAGE:
 *   Zamani
 *
 * ANTLR:
 *   ANTLR4
 *
 * RUST COMPILER BASELINE:
 *   Rust 1.97 / 1.97.1
 *
 * RUST EDITION:
 *   2021
 *
 * SAFETY:
 *   The Rust implementation must use safe Rust.
 *   No unsafe Rust is required.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file defines the lexical vocabulary consumed through:
 *
 *   grammar/antlr/ZamaniLexer.g4
 *
 * It recognizes:
 *
 *   - identifiers and keywords;
 *   - literals;
 *   - operators;
 *   - punctuation;
 *   - delimiters;
 *   - annotations;
 *   - comments;
 *   - whitespace;
 *   - Unicode identifiers;
 *   - domain-neutral source constructs.
 *
 * It does not perform parsing, type checking, semantic analysis,
 * hardware discovery, resource allocation, or target selection.
 *
 * ============================================================================
 * 2. OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *   - lexical token recognition;
 *   - canonical token spellings;
 *   - keyword recognition;
 *   - operator recognition;
 *   - punctuation recognition;
 *   - lexical precedence;
 *   - literal boundaries;
 *   - comments and whitespace;
 *   - lexical error-token recognition where supported.
 *
 * DOES NOT OWN:
 *
 *   - AST construction;
 *   - expression precedence;
 *   - statement semantics;
 *   - type semantics;
 *   - resource feasibility;
 *   - capability negotiation;
 *   - execution policies;
 *   - effects;
 *   - contracts;
 *   - provenance semantics;
 *   - quantum IR;
 *   - physical quantum gates;
 *   - hardware topology;
 *   - HDL synthesis;
 *   - backend scheduling;
 *   - runtime execution.
 *
 * ============================================================================
 * 3. INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *   ANTLR4 lexer runtime.
 *
 * EXPORTS:
 *   Canonical lexer token vocabulary.
 *
 * CONSUMED_BY:
 *   grammar/antlr/ZamaniLexer.g4
 *
 *   through:
 *
 *     lexer grammar ZamaniLexer;
 *     import lexer;
 *
 * AST_OWNER:
 *   Existing Rust frontend AST.
 *
 * SEMANTIC_OWNER:
 *   Semantic analysis.
 *
 * IR_OWNER:
 *   Canonical compiler IR.
 *   Quantum operations lower through quantum::ir.
 *
 * TEST_OWNER:
 *   grammar/tests/lexical/
 *
 * SPEC_OWNER:
 *   grammar/specification/
 *   grammar/lexer/
 *
 * COMPATIBILITY_OWNER:
 *   grammar/compatibility/
 *
 * ============================================================================
 * 4. TOKEN NAMING
 * ============================================================================
 *
 * Token names use UPPER_SNAKE_CASE.
 *
 * Parser grammars consume these tokens through ZamaniLexer.
 *
 * Existing Rust TokenType variants require an explicit mapping.
 *
 * No token name is assumed to be a Rust enum variant automatically.
 *
 * ============================================================================
 * 5. SCALABILITY
 * ============================================================================
 *
 * This grammar defines no universal maximum for:
 *
 *   source size
 *   identifier length
 *   numeric precision
 *   number of declarations
 *   number of modules
 *   number of qubits
 *   number of devices
 *   number of processors
 *   memory capacity
 *   tensor rank
 *   register width
 *   hardware topology
 *   network size
 *
 * Implementation resource exhaustion must be reported explicitly by
 * the compiler infrastructure.
 *
 * It must never silently truncate source or alter token meaning.
 *
 * ============================================================================
 * 6. DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Quantum operation names, vendor names, device names, model names,
 * algorithms, and application concepts are identifiers unless the
 * normative language specification explicitly reserves them.
 *
 * New quantum operations must not require editing this lexer.
 *
 * ============================================================================
 * 7. DETERMINISM
 * ============================================================================
 *
 * Lexical results depend only on:
 *
 *   source text;
 *   lexical rules;
 *   language version;
 *   explicitly selected lexical configuration.
 *
 * They must not depend on:
 *
 *   hardware;
 *   filesystem state;
 *   network state;
 *   wall-clock time;
 *   randomness;
 *   runtime state;
 *   target availability.
 *
 * ============================================================================
 * 8. COMPATIBILITY
 * ============================================================================
 *
 * Existing source spellings must not silently change meaning.
 *
 * New reserved words require:
 *
 *   specification;
 *   compatibility review;
 *   Rust lexer mapping;
 *   parser integration;
 *   AST integration where applicable;
 *   positive and negative tests.
 *
 * ============================================================================
 */

lexer grammar lexer;

// ============================================================================
// A. WHITESPACE
// ============================================================================

WHITESPACE
    : [ \t\r\n\f]+ -> channel(HIDDEN)
    ;

// ============================================================================
// B. COMMENTS
//
// Documentation comments are distinct from ordinary comments.
// Block comments are non-nesting.
// ============================================================================

DOC_LINE_COMMENT
    : '///' ~[\r\n]* -> channel(HIDDEN)
    ;

DOC_BLOCK_COMMENT
    : '/**' .*? '*/' -> channel(HIDDEN)
    ;

LINE_COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;

BLOCK_COMMENT
    : '/*' .*? '*/' -> channel(HIDDEN)
    ;

// ============================================================================
// C. KEYWORDS — CORE PROGRAM STRUCTURE
// ============================================================================

MODULE          : 'module';
IMPORT          : 'import';
EXPORT          : 'export';
FROM            : 'from';
AS              : 'as';
USE             : 'use';
INCLUDE         : 'include';
NAMESPACE       : 'namespace';
PACKAGE         : 'package';

PUB             : 'pub';
PUBLIC          : 'public';
PRIVATE         : 'private';
PROTECTED       : 'protected';
INTERNAL        : 'internal';

CONST           : 'const';
LET             : 'let';
VAR             : 'var';
MUT             : 'mut';
STATIC          : 'static';

FN              : 'fn';
FUNCTION        : 'function';
RETURN          : 'return';
YIELD           : 'yield';
BREAK           : 'break';
CONTINUE        : 'continue';

IF              : 'if';
ELSE            : 'else';
THEN            : 'then';
FOR             : 'for';
WHILE           : 'while';
LOOP            : 'loop';
IN              : 'in';
MATCH           : 'match';
CASE            : 'case';
WHEN            : 'when';
IS              : 'is';
WHERE           : 'where';

ASYNC           : 'async';
AWAIT           : 'await';
SPAWN           : 'spawn';
TASK            : 'task';
GENERATOR       : 'generator';

TRY             : 'try';
CATCH           : 'catch';
FINALLY         : 'finally';
THROW           : 'throw';
RAISE           : 'raise';
DEFER           : 'defer';

TRUE             : 'true';
FALSE            : 'false';
NULL_LITERAL     : 'null';
NIL              : 'nil';

// ============================================================================
// D. DECLARATIONS AND TYPES
// ============================================================================

TYPE             : 'type';
TYPEOF           : 'typeof';

STRUCT           : 'struct';
RECORD           : 'record';
ENUM             : 'enum';
UNION            : 'union';

CLASS            : 'class';
INTERFACE        : 'interface';
TRAIT            : 'trait';
IMPL             : 'impl';

EXTENDS          : 'extends';
IMPLEMENTS       : 'implements';
OVERRIDE         : 'override';
VIRTUAL          : 'virtual';
ABSTRACT         : 'abstract';

NEW              : 'new';
SELF             : 'self';
SUPER            : 'super';
THIS             : 'this';

INIT             : 'init';
DROP             : 'drop';

OPAQUE           : 'opaque';
ASSOCIATED       : 'associated';
LINEAR           : 'linear';
AFFINE           : 'affine';
MOVE             : 'move';
REF              : 'ref';

DYN              : 'dyn';
STATIC_DISPATCH  : 'static_dispatch';

// ============================================================================
// E. UNIVERSAL SEMANTIC CONSTRUCTS
//
// These tokens express source-level intent.
// Their interpretation belongs to semantic analysis.
// ============================================================================

REQUIRES         : 'requires';
ENSURES          : 'ensures';
INVARIANT        : 'invariant';
ASSUME           : 'assume';
GUARANTEE        : 'guarantee';
PROPERTY         : 'property';
ASSERT           : 'assert';
ASSERTION        : 'assertion';

EFFECT           : 'effect';
HANDLE           : 'handle';
PERFORM          : 'perform';

CAPABILITY       : 'capability';
RESOURCE         : 'resource';
CONSTRAINT       : 'constraint';
PREFERENCE       : 'prefer';
BUDGET           : 'budget';
HINT             : 'hint';
NEGOTIATE        : 'negotiate';

POLICY           : 'policy';
ALLOW            : 'allow';
FORBID           : 'forbid';
PERMIT           : 'permit';
DENY             : 'deny';

EVIDENCE         : 'evidence';
PROVENANCE       : 'provenance';
EXPLAIN          : 'explain';
DECISION         : 'decision';

SANDBOX          : 'sandbox';
SIMULATE         : 'simulate';

REASON           : 'reason';
INFER            : 'infer';
DEDUCE           : 'deduce';

LEARN            : 'learn';
ADAPT            : 'adapt';

KNOWLEDGE        : 'knowledge';
RETRACT          : 'retract';
QUERY            : 'query';

UNCERTAIN        : 'uncertain';
PROBABILITY      : 'probability';
DISTRIBUTION     : 'distribution';
CONFIDENCE       : 'confidence';
BELIEF           : 'belief';

CAUSE            : 'cause';
INTERVENTION     : 'intervention';
COUNTERFACTUAL   : 'counterfactual';

OBSERVE          : 'observe';
VERIFY           : 'verify';
VALIDATE         : 'validate';

POLICY_SCOPE     : 'scope';

// ============================================================================
// F. CLASSICAL COMPUTATION
// ============================================================================

PURE             : 'pure';
IMPURE           : 'impure';

PRINT            : 'print';
PRINTLN          : 'println';

PANIC            : 'panic';
UNREACHABLE      : 'unreachable';

SIZEOF           : 'sizeof';
LEN              : 'len';

// ============================================================================
// G. CONCURRENCY AND DISTRIBUTION
// ============================================================================

ACTOR            : 'actor';
MESSAGE          : 'message';
CHANNEL          : 'channel';
SELECT           : 'select';
SEND             : 'send';
RECEIVE          : 'receive';

CONCURRENT       : 'concurrent';
PARALLEL         : 'parallel';
SEQUENTIAL       : 'sequential';

DISTRIBUTED      : 'distributed';
COLLECTIVE       : 'collective';
CONSENSUS        : 'consensus';

RETRY            : 'retry';
RECOVER          : 'recover';
FALLBACK         : 'fallback';
ESCALATE         : 'escalate';

// ============================================================================
// H. QUANTUM AND HYBRID COMPUTATION
//
// Individual operation names are deliberately not reserved.
// ============================================================================

QUANTUM          : 'quantum';
QUBIT            : 'qubit';
QUBITS           : 'qubits';
CIRCUIT          : 'circuit';

MEASURE          : 'measure';
MEASUREMENT      : 'measurement';
ENTANGLE         : 'entangle';

CLASSICAL        : 'classical';
HYBRID           : 'hybrid';

QUANTUM_STATE    : 'quantum_state';
QUANTUM_CHANNEL  : 'quantum_channel';

NOISE            : 'noise';
QEC              : 'qec';

// ============================================================================
// I. HARDWARE DESCRIPTION AND COMPUTATIONAL RESOURCES
// ============================================================================

HDL              : 'hdl';
HARDWARE         : 'hardware';
DEVICE           : 'device';
ACCELERATOR      : 'accelerator';

SIGNAL           : 'signal';
WIRE             : 'wire';
PORT             : 'port';
CLOCK            : 'clock';
RESET            : 'reset';

INPUT            : 'input';
OUTPUT           : 'output';
INOUT            : 'inout';

REGISTER         : 'register';
MEMORY           : 'memory';
PIPELINE         : 'pipeline';

TIMING           : 'timing';
SYNTHESIZE       : 'synthesize';
SIMULATION       : 'simulation';

TOPOLOGY         : 'topology';
PLACEMENT        : 'placement';
SCHEDULE         : 'schedule';

// ============================================================================
// J. DATA AND INTEROPERABILITY
// ============================================================================

DATA             : 'data';
DATASET          : 'dataset';
SCHEMA           : 'schema';

GRAPH            : 'graph';
NODE             : 'node';
EDGE             : 'edge';

STREAM           : 'stream';
PIPE             : 'pipe';

FOREIGN          : 'foreign';
EXTERN           : 'extern';
FFI              : 'ffi';
ABI              : 'abi';

LINK             : 'link';
CALLING_CONVENTION : 'calling_convention';

// ============================================================================
// K. METAPROGRAMMING
// ============================================================================

MACRO            : 'macro';
QUOTE            : 'quote';
UNQUOTE          : 'unquote';

REFLECT          : 'reflect';
REFLECTION       : 'reflection';

COMPTIME         : 'comptime';
GENERATE         : 'generate';

SYNTAX           : 'syntax';
AST              : 'ast';

// ============================================================================
// L. EFFECTIVE BOOLEAN AND LOGICAL WORDS
// ============================================================================

AND              : 'and';
OR               : 'or';
NOT              : 'not';
XOR              : 'xor';

// ============================================================================
// M. SYMBOLIC LITERALS
//
// Multi-character forms must precede shorter overlapping forms.
// ============================================================================

QUANTUM_BASIS_ZERO
    : '|0⟩'
    ;

QUANTUM_BASIS_ONE
    : '|1⟩'
    ;

QUANTUM_PLUS
    : '|+⟩'
    ;

QUANTUM_MINUS
    : '|-⟩'
    ;

SIGMA_SYMBOL
    : 'Σ'
    ;

PI_SYMBOL
    : 'Π'
    ;

// ============================================================================
// N. NUMERIC LITERALS
//
// Lexical recognition does not determine machine width or target precision.
// Semantic analysis determines numeric interpretation.
// ============================================================================

// Decimal integer with optional digit separators.
INTEGER_LITERAL
    : DIGIT (DIGIT | '_')*
    ;

// Decimal floating-point forms:
//   1.0
//   1.
//   .5
//   1e10
//   1.5e-10

FLOAT_LITERAL
    : DIGIT (DIGIT | '_')* '.' DIGIT (DIGIT | '_')* EXPONENT?
    | DIGIT (DIGIT | '_')* '.' EXPONENT?
    | '.' DIGIT (DIGIT | '_')* EXPONENT?
    | DIGIT (DIGIT | '_')* EXPONENT
    ;

// Binary integer.
BINARY_INTEGER_LITERAL
    : '0' [bB] [01] ( [01] | '_' [01] )*
    ;

// Octal integer.
OCTAL_INTEGER_LITERAL
    : '0' [oO] [0-7] ( [0-7] | '_' [0-7] )*
    ;

// Hexadecimal integer.
HEX_INTEGER_LITERAL
    : '0' [xX] HEX_DIGIT ( HEX_DIGIT | '_' HEX_DIGIT )*
    ;

// ============================================================================
// O. STRING LITERALS
// ============================================================================

// Standard escaped string.
STRING_LITERAL
    : '"' (ESCAPE_SEQUENCE | ~["\\\r\n])* '"'
    ;

// Raw string with arbitrary hash delimiters is intentionally handled by
// a bounded lexical form here. Additional raw-string delimiter variants
// require explicit grammar support and conformance tests.
RAW_STRING_LITERAL
    : 'r"' .*? '"'
    ;

// Multiline string.
MULTILINE_STRING_LITERAL
    : '"""' .*? '"""'
    ;

// ============================================================================
// P. CHARACTER LITERALS
// ============================================================================

CHAR_LITERAL
    : '\'' (ESCAPE_SEQUENCE | ~['\\\r\n]) '\''
    ;

// ============================================================================
// Q. ANNOTATIONS AND ATTRIBUTES
// ============================================================================

AT
    : '@'
    ;

HASH
    : '#'
    ;

// ============================================================================
// R. MULTI-CHARACTER OPERATORS
//
// Longest-match behavior is intentional.
// ============================================================================

ELLIPSIS
    : '...'
    ;

RANGE_INCLUSIVE
    : '..='
    ;

RANGE
    : '..'
    ;

FAT_ARROW
    : '=>'
    ;

THIN_ARROW
    : '->'
    ;

DOUBLE_COLON
    : '::'
    ;

SAFE_MEMBER
    : '?.'
    ;

NULL_COALESCE
    : '??'
    ;

NULL_COALESCE_ASSIGN
    : '??='
    ;

POWER
    : '**'
    ;

POWER_ASSIGN
    : '**='
    ;

SHIFT_LEFT_ASSIGN
    : '<<='
    ;

SHIFT_RIGHT_ASSIGN
    : '>>='
    ;

SHIFT_LEFT
    : '<<'
    ;

SHIFT_RIGHT
    : '>>'
    ;

LESS_EQUAL
    : '<='
    ;

GREATER_EQUAL
    : '>='
    ;

EQUAL
    : '=='
    ;

NOT_EQUAL
    : '!='
    ;

LOGICAL_AND
    : '&&'
    ;

LOGICAL_OR
    : '||'
    ;

PLUS_ASSIGN
    : '+='
    ;

MINUS_ASSIGN
    : '-='
    ;

MULTIPLY_ASSIGN
    : '*='
    ;

DIVIDE_ASSIGN
    : '/='
    ;

MODULO_ASSIGN
    : '%='
    ;

BIT_AND_ASSIGN
    : '&='
    ;

BIT_OR_ASSIGN
    : '|='
    ;

BIT_XOR_ASSIGN
    : '^='
    ;

// ============================================================================
// S. SINGLE-CHARACTER OPERATORS
// ============================================================================

PLUS
    : '+'
    ;

MINUS
    : '-'
    ;

MULTIPLY
    : '*'
    ;

DIVIDE
    : '/'
    ;

MODULO
    : '%'
    ;

ASSIGN
    : '='
    ;

LESS_THAN
    : '<'
    ;

GREATER_THAN
    : '>'
    ;

LOGICAL_NOT
    : '!'
    ;

BIT_AND
    : '&'
    ;

BIT_OR
    : '|'
    ;

BIT_XOR
    : '^'
    ;

BIT_NOT
    : '~'
    ;

QUESTION
    : '?'
    ;

// ============================================================================
// T. DELIMITERS AND PUNCTUATION
// ============================================================================

LEFT_PAREN
    : '('
    ;

RIGHT_PAREN
    : ')'
    ;

LEFT_BRACE
    : '{'
    ;

RIGHT_BRACE
    : '}'
    ;

LEFT_BRACKET
    : '['
    ;

RIGHT_BRACKET
    : ']'
    ;

COMMA
    : ','
    ;

DOT
    : '.'
    ;

SEMICOLON
    : ';'
    ;

COLON
    : ':'
    ;

// ============================================================================
// U. IDENTIFIERS
//
// Unicode identifiers are permitted.
// Identifier normalization and confusable-character security checks belong
// to the frontend's identifier-validation stage.
//
// Keywords appear before IDENTIFIER, so exact reserved spellings receive
// their dedicated tokens.
// ============================================================================

IDENTIFIER
    : ID_START ID_CONTINUE*
    ;

// ============================================================================
// V. INVALID INPUT
//
// This token provides a deterministic catch-all for unsupported characters.
// The Rust frontend must produce a source-spanned lexical diagnostic.
// ============================================================================

ERROR_CHAR
    : .
    ;

// ============================================================================
// W. FRAGMENTS
// ============================================================================

fragment DIGIT
    : [0-9]
    ;

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;

fragment EXPONENT
    : [eE] [+-]? DIGIT (DIGIT | '_')*
    ;

fragment ESCAPE_SEQUENCE
    : '\\' (
          [btnfr"'\\0]
        | 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
        | 'x' HEX_DIGIT HEX_DIGIT
      )
    ;

fragment ID_START
    : [a-zA-Z_]
    | [\p{L}]
    | [\p{Nl}]
    ;

fragment ID_CONTINUE
    : ID_START
    | [0-9]
    | [\p{Mn}]
    | [\p{Mc}]
    | [\p{Nd}]
    | [\p{Pc}]
    ;