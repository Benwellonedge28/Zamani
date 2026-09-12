lexer grammar ZamaniTokens;

// =============================================================================
// Zamani Universal Programming Language
// grammar/lexer/tokens.g4
//
// Production lexical foundation.
//
// OWNERSHIP
//   This file owns:
//     - concrete token vocabulary;
//     - lexical recognition of keywords;
//     - identifiers;
//     - literals;
//     - operators;
//     - punctuation;
//     - comments and whitespace;
//     - stable token names consumed by parser grammars.
//
// DOES NOT OWN
//   - AST construction;
//   - semantic analysis;
//   - type checking;
//   - quantum IR;
//   - QEC;
//   - ZQN;
//   - optimization;
//   - routing;
//   - scheduling;
//   - hardware discovery;
//   - runtime dispatch;
//   - machine-specific resource limits.
//
// INTEGRATION
//   Zamani.g4 / future parser grammars
//       -> ZamaniTokens token vocabulary
//       -> AST / semantic analysis
//       -> canonical IR
//       -> optimization / routing / scheduling
//       -> hardware / runtime.
//
// SCALABILITY
//   No machine, device, topology, qubit-count, CPU-count, memory-size,
//   accelerator-count, or other physical-resource ceiling is encoded here.
//
// SAFETY
//   This is ANTLR grammar source. Generated Rust code must be built with
//   unsafe code forbidden by the Rust crate policy.
//
// RUST TARGET
//   Rust 1.97 / 1.97.1 for generated/runtime integration.
//
// =============================================================================


// =============================================================================
// 1. RESERVED LANGUAGE KEYWORDS
//
// Keywords are declared before IDENTIFIER so reserved words cannot silently
// become identifiers.
//
// Adding a keyword is a language-versioning operation and must be reflected
// in the language specification and compatibility tests.
//
// Domain-specific concepts are keywords only when they have stable language
// syntax. Hardware names, device IDs, topology names and resource counts do
// NOT belong here.
// =============================================================================

// -----------------------------------------------------------------------------
// Package / module system
// -----------------------------------------------------------------------------

K_PACKAGE      : 'package' ;
K_MODULE       : 'module' ;
K_IMPORT       : 'import' ;
K_EXPORT       : 'export' ;
K_USE          : 'use' ;
K_FROM         : 'from' ;
K_AS           : 'as' ;
K_GLOBAL       : 'global' ;
K_USING        : 'using' ;

// -----------------------------------------------------------------------------
// Visibility / declaration modifiers
// -----------------------------------------------------------------------------

K_PUB          : 'pub' ;
K_PUBLIC       : 'public' ;
K_PRIVATE      : 'private' ;
K_PROTECTED    : 'protected' ;
K_INTERNAL     : 'internal' ;
K_STATIC       : 'static' ;
K_CONST        : 'const' ;
K_LET          : 'let' ;
K_VAR          : 'var' ;
K_VAL          : 'val' ;
K_MUT          : 'mut' ;

K_EXTERN       : 'extern' ;
K_VOLATILE     : 'volatile' ;
K_INLINE       : 'inline' ;
K_FINAL        : 'final' ;
K_SEALED       : 'sealed' ;
K_PARTIAL      : 'partial' ;

// -----------------------------------------------------------------------------
// Functions / control flow
// -----------------------------------------------------------------------------

K_FN           : 'fn' ;
K_RETURN       : 'return' ;

K_IF           : 'if' ;
K_ELSE         : 'else' ;

K_WHILE        : 'while' ;
K_DO           : 'do' ;
K_FOR          : 'for' ;
K_IN           : 'in' ;
K_LOOP         : 'loop' ;

K_BREAK        : 'break' ;
K_CONTINUE     : 'continue' ;

K_MATCH        : 'match' ;
K_CASE         : 'case' ;
K_WHEN         : 'when' ;

K_WITH         : 'with' ;
K_YIELD        : 'yield' ;

// -----------------------------------------------------------------------------
// Types / declarations / object model
// -----------------------------------------------------------------------------

K_TYPE         : 'type' ;
K_STRUCT       : 'struct' ;
K_ENUM         : 'enum' ;
K_TRAIT        : 'trait' ;
K_IMPL         : 'impl' ;
K_CLASS        : 'class' ;
K_INTERFACE    : 'interface' ;
K_RECORD       : 'record' ;

K_EXTENDS      : 'extends' ;
K_IMPLEMENTS   : 'implements' ;
K_PERMITS      : 'permits' ;

K_NEW          : 'new' ;
K_THIS         : 'this' ;
K_SELF         : 'self' ;
K_SUPER        : 'super' ;
K_SELF_TYPE    : 'Self' ;

K_VIRTUAL      : 'virtual' ;
K_OVERRIDE     : 'override' ;
K_ABSTRACT     : 'abstract' ;

// -----------------------------------------------------------------------------
// Async / concurrency / safety
// -----------------------------------------------------------------------------

K_ASYNC        : 'async' ;
K_AWAIT        : 'await' ;
K_SPAWN        : 'spawn' ;

K_TRY          : 'try' ;
K_CATCH        : 'catch' ;
K_FINALLY      : 'finally' ;
K_THROW        : 'throw' ;

K_UNSAFE       : 'unsafe' ;
K_SAFE         : 'safe' ;

// -----------------------------------------------------------------------------
// Effects
// -----------------------------------------------------------------------------

K_EFFECT       : 'effect' ;
K_PERFORM      : 'perform' ;
K_HANDLE       : 'handle' ;

// -----------------------------------------------------------------------------
// Quantum computing
//
// These tokens express language-level quantum concepts.
// They do NOT identify a physical processor or impose a physical topology.
// -----------------------------------------------------------------------------

K_QUANTUM      : 'quantum' ;
K_QUBIT        : 'qubit' ;
K_CIRCUIT      : 'circuit' ;

K_ENTANGLE     : 'entangle' ;
K_NOISE        : 'noise' ;
K_FIDELITY     : 'fidelity' ;
K_SURFACE      : 'surface' ;
K_LOGICAL      : 'logical' ;
K_PARITY       : 'parity' ;
K_CODE         : 'code' ;

// -----------------------------------------------------------------------------
// Nano / Sankofa / temporal-language concepts
// -----------------------------------------------------------------------------

K_NANO         : 'nano' ;
K_AGENT        : 'agent' ;

K_REMEMBER     : 'remember' ;
K_RECALL       : 'recall' ;
K_LEARN        : 'learn' ;
K_INFER        : 'infer' ;
K_WISDOM       : 'wisdom' ;

K_ZAMANI       : 'zamani' ;
K_SASA         : 'sasa' ;
K_ANCESTOR     : 'ancestor' ;

K_LINEAR       : 'linear' ;
K_AFFINE       : 'affine' ;

// -----------------------------------------------------------------------------
// Language / model / compilation concepts
// -----------------------------------------------------------------------------

K_LANGUAGE     : 'language' ;
K_MODEL        : 'model' ;

K_SIMULATE     : 'simulate' ;
K_SYNTHESIZE   : 'synthesize' ;
K_DEPLOY       : 'deploy' ;

// -----------------------------------------------------------------------------
// Existing universal / system vocabulary
// -----------------------------------------------------------------------------

K_OMNIVERSAL   : 'omniversal' ;
K_ALIGNMENT    : 'alignment' ;
K_CONTAINMENT  : 'containment' ;
K_TRUST        : 'trust' ;
K_KNOWLEDGE    : 'knowledge' ;
K_GENERATE     : 'generate' ;
K_GENERATIVE   : 'generative' ;
K_SOVEREIGNTY  : 'sovereignty' ;
K_GOAL         : 'goal' ;
K_BIONANO      : 'bionano' ;
K_REALITY      : 'reality' ;
K_NLP          : 'nlp' ;
K_SYSTEM       : 'system' ;

K_ASI          : 'asi' ;
K_AESI         : 'aesi' ;
K_ASESI        : 'asesi' ;

K_ADMIN        : 'admin' ;
K_PAYMENT      : 'payment' ;
K_GATEWAY      : 'gateway' ;
K_GRAPHICS     : 'graphics' ;
K_VIDEO        : 'video' ;
K_ADJUST       : 'adjust' ;
K_VERSIONING   : 'versioning' ;
K_COPYRIGHT    : 'copyright' ;
K_NOTICE       : 'notice' ;
K_LEGAL        : 'legal' ;
K_ACTION       : 'action' ;
K_TAILOR       : 'tailor' ;
K_BUSINESS     : 'business' ;

// -----------------------------------------------------------------------------
// Built-in operations
// -----------------------------------------------------------------------------

K_PRINT        : 'print' ;
K_PRINTLN      : 'println' ;
K_ASSERT       : 'assert' ;
K_PANIC        : 'panic' ;
K_LEN          : 'len' ;
K_SIZEOF       : 'sizeof' ;

// -----------------------------------------------------------------------------
// Built-in types
// -----------------------------------------------------------------------------

K_VOID         : 'void' ;
K_INT          : 'int' ;
K_FLOAT        : 'float' ;
K_BOOL         : 'bool' ;
K_STR          : 'str' ;
K_STRING       : 'String' ;
K_CHAR         : 'char' ;

// -----------------------------------------------------------------------------
// Boolean / null / algebraic values
// -----------------------------------------------------------------------------

K_TRUE         : 'true' ;
K_FALSE        : 'false' ;
K_NIL          : 'nil' ;
K_NULL         : 'null' ;

K_SOME         : 'Some' ;
K_OK           : 'Ok' ;
K_ERR          : 'Err' ;
K_NEVER        : 'never' ;

// -----------------------------------------------------------------------------
// Logical / type-language keywords
// -----------------------------------------------------------------------------

K_AND          : 'and' ;
K_OR           : 'or' ;
K_NOT          : 'not' ;
K_IS           : 'is' ;

K_PI           : 'Pi' ;


// =============================================================================
// 2. MATHEMATICAL SYMBOL TOKENS
// =============================================================================

PI_SYMBOL
    : '\u03A0' // Π
    ;

SIGMA_SYMBOL
    : '\u03A3' // Σ
    ;


// =============================================================================
// 3. COMPOUND OPERATORS
//
// Longer operators must appear before their prefixes.
// This avoids splitting:
//   ...  ->  =>  ::  ==  !=  <=  >=  &&  ||  <<  >>  += ...
// into shorter tokens.
// =============================================================================

ELLIPSIS
    : '...'
    ;

DOT_DOT_EQ
    : '..='
    ;

DOT_DOT
    : '..'
    ;

THIN_ARROW
    : '->'
    ;

FAT_ARROW
    : '=>'
    ;

DOUBLE_COLON
    : '::'
    ;

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

LOGICAL_AND
    : '&&'
    ;

LOGICAL_OR
    : '||'
    ;

LEFT_SHIFT
    : '<<'
    ;

RIGHT_SHIFT
    : '>>'
    ;

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

INCREMENT
    : '++'
    ;

DECREMENT
    : '--'
    ;

QUESTION_DOT
    : '?.'
    ;

NULL_COALESCE
    : '??'
    ;


// =============================================================================
// 4. QUANTUM STATE LITERALS
//
// These are semantic source literals, not physical qubit identifiers.
//
// The grammar intentionally does not impose:
//   - a maximum number of qubits;
//   - a maximum state-vector dimension;
//   - a fixed register size;
//   - a hardware topology;
//   - a physical device.
// =============================================================================

QUANTUM_LITERAL
    : '|'
      (
          '0'
        | '1'
        | '+'
        | '-'
      )
      '\u27E9'
    ;


// =============================================================================
// 5. MACHINE-INDEPENDENT NUMERIC LITERALS
//
// Separators are permitted only between digits.
//
// Examples:
//
//   0
//   42
//   1_000
//   0xFF
//   0xDEAD_BEEF
//   0b1010_0101
//   0o755
//   3.14159
//   1.0e10
//   1.0f64
//
// No maximum magnitude is imposed here.
// Actual representability belongs to the semantic/type/target layers.
// =============================================================================

FLOAT_LITERAL
    : DIGIT_GROUP
      '.'
      DIGIT_GROUP?
      EXPONENT?
      FLOAT_SUFFIX?

    | '.'
      DIGIT_GROUP
      EXPONENT?
      FLOAT_SUFFIX?

    | DIGIT_GROUP
      EXPONENT
      FLOAT_SUFFIX?

    | DIGIT_GROUP
      FLOAT_SUFFIX
    ;

HEX_INTEGER
    : '0'
      [xX]
      HEX_DIGIT
      ('_'? HEX_DIGIT)*
    ;

BINARY_INTEGER
    : '0'
      [bB]
      BIN_DIGIT
      ('_'? BIN_DIGIT)*
    ;

OCTAL_INTEGER
    : '0'
      [oO]
      OCT_DIGIT
      ('_'? OCT_DIGIT)*
    ;

INTEGER_LITERAL
    : DIGIT_GROUP
      INT_SUFFIX?
    ;


// =============================================================================
// 6. CHARACTER LITERALS
// =============================================================================

CHAR_LITERAL
    : '\''
      (
          ESCAPE_SEQUENCE
        | ~['\\\r\n]
      )
      '\''
    ;


// =============================================================================
// 7. STRING LITERALS
//
// Strings are deliberately lexical values. Encoding, allocation, ownership,
// interning, storage, and runtime representation belong elsewhere.
// =============================================================================

STRING_LITERAL
    : '"'
      (
          ESCAPE_SEQUENCE
        | ~["\\\r\n]
      )*
      '"'
    ;


// =============================================================================
// 8. IDENTIFIERS
//
// The stable baseline remains ASCII identifiers:
//
//   [A-Za-z_][A-Za-z0-9_]*
//
// This preserves compatibility with the existing Zamani lexer while avoiding
// accidental dependence on one ANTLR target's Unicode-regex implementation.
//
// Unicode mathematical/domain symbols that have language meaning are exposed
// through dedicated tokens above.
//
// A future generalized Unicode identifier policy belongs in the lexical
// specification and compatibility process, not in downstream IR/hardware code.
// =============================================================================

IDENTIFIER
    : IDENTIFIER_START
      IDENTIFIER_CONTINUE*
    ;


// =============================================================================
// 9. PUNCTUATION
// =============================================================================

LPAREN
    : '('
    ;

RPAREN
    : ')'
    ;

LBRACE
    : '{'
    ;

RBRACE
    : '}'
    ;

LBRACKET
    : '['
    ;

RBRACKET
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

QUESTION
    : '?'
    ;

EXCLAMATION
    : '!'
    ;

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

PERCENT
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

HASH
    : '#'
    ;

AT
    : '@'
    ;

BACKSLASH
    : '\\'
    ;


// =============================================================================
// 10. COMMENTS
//
// Documentation comments are recognized before ordinary comments so they
// remain distinguishable to tooling.
//
// All comments remain on HIDDEN rather than being discarded. This permits
// source-preserving tooling, documentation extraction, formatting, IDEs,
// diagnostics, and future source-to-source transformations.
// =============================================================================

DOC_LINE_COMMENT
    : '///'
      ~[\r\n]*
      -> channel(HIDDEN)
    ;

DOC_BLOCK_COMMENT
    : '/**'
      .*?
      '*/'
      -> channel(HIDDEN)
    ;

LINE_COMMENT
    : '//'
      ~[\r\n]*
      -> channel(HIDDEN)
    ;

BLOCK_COMMENT
    : '/*'
      .*?
      '*/'
      -> channel(HIDDEN)
    ;


// =============================================================================
// 11. WHITESPACE
//
// Whitespace is syntactic trivia and therefore does not enter the parser's
// default token stream.
// =============================================================================

WHITESPACE
    : [ \t\r\n\f]+
      -> channel(HIDDEN)
    ;


// =============================================================================
// 12. LEXICAL FRAGMENTS
// =============================================================================

fragment DIGIT
    : [0-9]
    ;

fragment DIGIT_GROUP
    : DIGIT
      ('_'? DIGIT)*
    ;

fragment EXPONENT
    : [eE]
      [+-]?
      DIGIT_GROUP
    ;

fragment FLOAT_SUFFIX
    : [fF]
      (
          '16'
        | '32'
        | '64'
        | '128'
      )
    ;

fragment INT_SUFFIX
    : [uUiI]
      (
          '8'
        | '16'
        | '32'
        | '64'
        | '128'
        | '256'
      )

    | [uUiI]
      [sS]
      [iI]
      [zZ]
      [eE]
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

fragment IDENTIFIER_START
    : [a-zA-Z_]
    ;

fragment IDENTIFIER_CONTINUE
    : [a-zA-Z0-9_]
    ;

fragment ESCAPE_SEQUENCE
    : '\\'
      (
          [btnfr"'\\]
        | '0'
        | 'x' HEX_DIGIT HEX_DIGIT
        | 'u' '{' HEX_DIGIT+ '}'
      )
    ;