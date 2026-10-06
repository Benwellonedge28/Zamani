/**
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/lexer/tokens.g4
 *
 * Grammar:
 *     ZamaniTokens
 *
 * Status:
 *     CANONICAL PRODUCTION LEXICAL VOCABULARY
 *
 * Purpose:
 *     Defines the actual public lexical vocabulary consumed by:
 *
 *         grammar/lexer/lexer.g4
 *             ->
 *         grammar/antlr/ZamaniLexer.g4
 *             ->
 *         grammar/antlr/ZamaniParser.g4
 *
 * Rust baseline:
 *     Rust 1.97+
 *
 * Safety:
 *     No target-language actions.
 *     No unsafe Rust.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - actual keyword tokens;
 *   - actual operator tokens;
 *   - actual punctuation tokens;
 *   - identifiers;
 *   - primitive literals;
 *   - quantum literals;
 *   - hardware/resource literals;
 *   - duration literals;
 *   - size literals;
 *   - comments;
 *   - lexical error tokens;
 *   - lexical fragments required by those tokens.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - parser productions;
 *   - AST nodes;
 *   - semantic analysis;
 *   - type checking;
 *   - effect checking;
 *   - capability negotiation;
 *   - resource negotiation;
 *   - quantum routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - backend selection;
 *   - runtime execution.
 *
 * ============================================================================
 * SINGLE TOKEN AUTHORITY
 * ============================================================================
 *
 * Every emitted lexical token has exactly one definition here.
 *
 * The following files are lexical design partitions/reference owners but are
 * NOT imported by the production lexer composition after this file becomes
 * canonical:
 *
 *   keywords.g4
 *   operators.g4
 *   punctuation.g4
 *   identifiers.g4
 *   literals.g4
 *   numeric-literals.g4
 *   string-literals.g4
 *   character-literals.g4
 *   boolean-literals.g4
 *   quantum-literals.g4
 *   hardware-literals.g4
 *   duration-literals.g4
 *   size-literals.g4
 *   annotations.g4
 *   comments.g4
 *   lexer-errors.g4
 *
 * This prevents duplicate token definitions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains no artificial machine limits.
 *
 * It does NOT define limits for:
 *
 *   qubits
 *   CPUs
 *   cores
 *   threads
 *   GPUs
 *   FPGAs
 *   ASICs
 *   QPUs
 *   accelerators
 *   nodes
 *   devices
 *   memory
 *   storage
 *   registers
 *   register width
 *   tensor rank
 *   tensor dimensions
 *   network size
 *   actor count
 *   process count
 *
 * Numeric and textual values are source values, not machine-capacity limits.
 *
 * ============================================================================
 * QUANTUM SCALABILITY
 * ============================================================================
 *
 * Quantum operation names remain identifiers.
 *
 * The lexer does NOT enumerate:
 *
 *   H
 *   X
 *   Y
 *   Z
 *   CNOT
 *   CX
 *   CZ
 *   SWAP
 *   RX
 *   RY
 *   RZ
 *
 * or future/vendor/custom operations.
 *
 * They are resolved semantically and eventually lowered through:
 *
 *     quantum::ir
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Lexical classification depends only upon:
 *
 *   source text;
 *   lexical grammar;
 *   language/compatibility configuration.
 *
 * It never depends upon:
 *
 *   hardware;
 *   network;
 *   filesystem state;
 *   runtime state;
 *   target availability;
 *   random state;
 *   wall-clock time.
 *
 * ============================================================================
 */

lexer grammar ZamaniTokens;


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

MODULE      : 'module' ;
IMPORT      : 'import' ;
EXPORT      : 'export' ;
USE         : 'use' ;
FROM        : 'from' ;
AS          : 'as' ;
PACKAGE     : 'package' ;

TYPE        : 'type' ;
STRUCT      : 'struct' ;
ENUM        : 'enum' ;
TRAIT       : 'trait' ;
IMPL        : 'impl' ;
CLASS       : 'class' ;
INTERFACE   : 'interface' ;
RECORD      : 'record' ;
UNION       : 'union' ;
ALIAS       : 'alias' ;

SEALED      : 'sealed' ;
PARTIAL     : 'partial' ;
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
WHERE       : 'where' ;


/* ============================================================================
 * CONCURRENCY / CONTROL / EFFECTS
 * ========================================================================== */

ASYNC           : 'async' ;
AWAIT           : 'await' ;
SPAWN           : 'spawn' ;
PARALLEL        : 'parallel' ;

UNSAFE          : 'unsafe' ;

TRY             : 'try' ;
CATCH           : 'catch' ;
FINALLY         : 'finally' ;
THROW           : 'throw' ;
HANDLE          : 'handle' ;

EFFECT          : 'effect' ;
EFFECTS         : 'effects' ;
WITH            : 'with' ;
PURE            : 'pure' ;

IO              : 'io' ;
NETWORK         : 'network' ;
MUTATION        : 'mutation' ;
RANDOMNESS      : 'randomness' ;
NATIVE          : 'native' ;
FOREIGN         : 'foreign' ;
DISTRIBUTED     : 'distributed' ;
MEASUREMENT     : 'measurement' ;
LEARNING        : 'learning' ;
ADAPTATION      : 'adaptation' ;
REFLECTION      : 'reflection' ;
CODE_GENERATION : 'code_generation' ;
SIMULATION      : 'simulation' ;


/* ============================================================================
 * CONTRACTS / VALIDATION
 * ========================================================================== */

REQUIRES        : 'requires' ;
ENSURES         : 'ensures' ;
INVARIANT       : 'invariant' ;
ASSUME          : 'assume' ;
GUARANTEE       : 'guarantee' ;
PROPERTY        : 'property' ;
PRECONDITION    : 'precondition' ;
POSTCONDITION   : 'postcondition' ;
CONTRACT        : 'contract' ;
ASSERT          : 'assert' ;


/* ============================================================================
 * RESOURCES / CAPABILITIES / PORTABILITY
 * ========================================================================== */

RESOURCE        : 'resource' ;
RESOURCES       : 'resources' ;
QUANTITY        : 'quantity' ;

CAPABILITY      : 'capability' ;
CAPABILITIES    : 'capabilities' ;

REQUIREMENT     : 'requirement' ;
REQUIREMENTS    : 'requirements' ;

CONSTRAINT      : 'constraint' ;
CONSTRAINTS     : 'constraints' ;

PREFER          : 'prefer' ;
PREFERENCE      : 'preference' ;
PREFERENCES     : 'preferences' ;

HINT            : 'hint' ;
HINTS           : 'hints' ;

TARGET          : 'target' ;
TARGETS         : 'targets' ;

CAPACITY        : 'capacity' ;
AVAILABILITY    : 'availability' ;
PORTABILITY     : 'portability' ;
SCALABILITY     : 'scalability' ;
PERFORMANCE     : 'performance' ;
LATENCY         : 'latency' ;
THROUGHPUT      : 'throughput' ;
BANDWIDTH       : 'bandwidth' ;
ENERGY          : 'energy' ;
POWER           : 'power' ;
RELIABILITY     : 'reliability' ;
RESILIENCE      : 'resilience' ;
COST            : 'cost' ;

RESERVE         : 'reserve' ;
ACQUIRE         : 'acquire' ;
RELEASE         : 'release' ;
DERIVE          : 'derive' ;
GROUP           : 'group' ;
PROFILE         : 'profile' ;


/* ============================================================================
 * POLICY / EXECUTION CONTROL
 * ========================================================================== */

POLICY          : 'policy' ;
POLICIES        : 'policies' ;

ALLOW           : 'allow' ;
FORBID          : 'forbid' ;
PERMIT          : 'permit' ;
DENY            : 'deny' ;

FALLBACK        : 'fallback' ;
RETRY            : 'retry' ;
RECOVER         : 'recover' ;
ESCALATE        : 'escalate' ;
REJECT          : 'reject' ;
SELECT          : 'select' ;
NEGOTIATE       : 'negotiate' ;

SANDBOX         : 'sandbox' ;
SIMULATE        : 'simulate' ;

DETERMINISTIC   : 'deterministic' ;
REPRODUCIBLE    : 'reproducible' ;


/* ============================================================================
 * REASONING / KNOWLEDGE
 * ========================================================================== */

INFER           : 'infer' ;
DEDUCE          : 'deduce' ;
REASON          : 'reason' ;

PREMISE         : 'premise' ;
PREMISES        : 'premises' ;
CONCLUSION      : 'conclusion' ;

PROVE           : 'prove' ;
VERIFY          : 'verify' ;
VALIDATE        : 'validate' ;

ASSUMPTION      : 'assumption' ;
OBSERVATION     : 'observation' ;
INTERVENTION    : 'intervention' ;
COUNTERFACTUAL  : 'counterfactual' ;

KNOWLEDGE       : 'knowledge' ;
ASSERT_KNOWLEDGE: 'assert_knowledge' ;
RETRACT         : 'retract' ;
QUERY           : 'query' ;

FACT            : 'fact' ;
FACTS           : 'facts' ;
RELATION        : 'relation' ;
RELATIONS       : 'relations' ;


/* ============================================================================
 * LEARNING / PROBABILISTIC COMPUTATION
 * ========================================================================== */

LEARN           : 'learn' ;
ADAPT           : 'adapt' ;
TRAIN           : 'train' ;
PREDICT         : 'predict' ;
EVALUATE        : 'evaluate' ;
FEEDBACK        : 'feedback' ;
MODEL           : 'model' ;
DATASET         : 'dataset' ;
OBJECTIVE       : 'objective' ;
TRANSFER        : 'transfer' ;
REINFORCEMENT   : 'reinforcement' ;
UPDATE          : 'update' ;

UNCERTAIN       : 'uncertain' ;
UNCERTAINTY     : 'uncertainty' ;
PROBABILITY     : 'probability' ;
PROBABILISTIC   : 'probabilistic' ;
DISTRIBUTION    : 'distribution' ;
CONFIDENCE      : 'confidence' ;
BELIEF          : 'belief' ;
LIKELIHOOD      : 'likelihood' ;


/* ============================================================================
 * EVIDENCE / PROVENANCE / EXPLANATION
 * ========================================================================== */

EVIDENCE        : 'evidence' ;
EXPLAIN         : 'explain' ;
EXPLANATION     : 'explanation' ;
PROVENANCE      : 'provenance' ;
SOURCE          : 'source' ;
DERIVATION      : 'derivation' ;

DECISION        : 'decision' ;
DECISIONS       : 'decisions' ;

GENERATED       : 'generated' ;
TRANSFORMED     : 'transformed' ;
VERIFIED        : 'verified' ;

AUDIT           : 'audit' ;
TRACE           : 'trace' ;


/* ============================================================================
 * PATTERNS / GUARDS
 * ========================================================================== */

PATTERN         : 'pattern' ;
PATTERNS        : 'patterns' ;
GUARD           : 'guard' ;
GUARDS          : 'guards' ;


/* ============================================================================
 * QUANTUM
 *
 * Quantum operation names themselves remain IDENTIFIER.
 * ========================================================================== */

QUANTUM         : 'quantum' ;
CIRCUIT         : 'circuit' ;
QUBIT           : 'qubit' ;
APPLY           : 'apply' ;
MEASURE         : 'measure' ;
RESET           : 'reset' ;
BARRIER         : 'barrier' ;
CONTROL         : 'control' ;
ADJOINT         : 'adjoint' ;
INVERSE         : 'inverse' ;
OBSERVE         : 'observe' ;
ENTANGLE        : 'entangle' ;
NOISE           : 'noise' ;
FIDELITY        : 'fidelity' ;
SURFACE         : 'surface' ;
LOGICAL         : 'logical' ;
PARITY          : 'parity' ;
QEC             : 'qec' ;
RESILIENT       : 'resilient' ;
DYNAMIC         : 'dynamic' ;


/* ============================================================================
 * HYBRID / CLASSICAL / ACCELERATION
 * ========================================================================== */

HYBRID          : 'hybrid' ;
CLASSICAL       : 'classical' ;
QUANTUM_CLASSICAL: 'quantum_classical' ;

ACCELERATOR     : 'accelerator' ;
DEVICE          : 'device' ;
HOST            : 'host' ;
SUBMIT          : 'submit' ;
SYNC            : 'sync' ;


/* ============================================================================
 * HDL / HARDWARE
 * ========================================================================== */

HDL             : 'hdl' ;
HARDWARE        : 'hardware' ;

SIGNAL          : 'signal' ;
CLOCK           : 'clock' ;
RESET_SIGNAL    : 'reset_signal' ;
TIMING          : 'timing' ;
SYNTHESIZE      : 'synthesize' ;
DEPLOY          : 'deploy' ;

REGISTER        : 'register' ;
MEMORY          : 'memory' ;

CHANNEL         : 'channel' ;
PORT            : 'port' ;
INPUT           : 'input' ;
OUTPUT          : 'output' ;
INOUT           : 'inout' ;


/* ============================================================================
 * DISTRIBUTED / ACTORS / NETWORK SERVICES
 * ========================================================================== */

NODE            : 'node' ;
NODES           : 'nodes' ;

ACTOR           : 'actor' ;
ACTORS          : 'actors' ;

MESSAGE         : 'message' ;
MESSAGES        : 'messages' ;

SERVICE         : 'service' ;
SERVICES        : 'services' ;

ENDPOINT        : 'endpoint' ;
ENDPOINTS       : 'endpoints' ;

STREAM          : 'stream' ;
STREAMS         : 'streams' ;

DISCOVER        : 'discover' ;


/* ============================================================================
 * AI / SYMBOLIC / NEURAL
 * ========================================================================== */

AI              : 'ai' ;
AGENT           : 'agent' ;
AGENTS          : 'agents' ;

NEURAL          : 'neural' ;
SYMBOLIC        : 'symbolic' ;
NEURAL_SYMBOLIC : 'neural_symbolic' ;
COGNITIVE       : 'cognitive' ;
PLANNING        : 'planning' ;

GENERATION      : 'generation' ;
GENERATIVE      : 'generative' ;


/* ============================================================================
 * DATA / QUERY / INTERCHANGE
 * ========================================================================== */

SCHEMA          : 'schema' ;
SCHEMAS         : 'schemas' ;

GRAPH           : 'graph' ;
GRAPHS          : 'graphs' ;

TABLE           : 'table' ;
TABLES          : 'tables' ;

COLUMN          : 'column' ;
COLUMNS         : 'columns' ;

ROW             : 'row' ;
ROWS            : 'rows' ;

FILTER          : 'filter' ;
MAP             : 'map' ;
REDUCE          : 'reduce' ;

JSON            : 'json' ;
XML             : 'xml' ;
LANGUAGE        : 'language' ;


/* ============================================================================
 * MACROS / METAPROGRAMMING
 * ========================================================================== */

MACRO           : 'macro' ;
EXTERN          : 'extern' ;
COMPILE         : 'compile' ;
COMPTIME        : 'comptime' ;

REFLECTIVE      : 'reflective' ;
INTROSPECT      : 'introspect' ;
GENERATE        : 'generate' ;
QUOTE           : 'quote' ;
SYNTAX          : 'syntax' ;


/* ============================================================================
 * TYPE-SYSTEM QUALIFIERS
 * ========================================================================== */

LINEAR          : 'linear' ;
AFFINE          : 'affine' ;
IMMUTABLE       : 'immutable' ;
INLINE          : 'inline' ;
VOLATILE        : 'volatile' ;

OPTION          : 'option' ;
SOME            : 'some' ;
NONE            : 'none' ;


/* ============================================================================
 * LANGUAGE / LEGACY COMPATIBILITY VOCABULARY
 * ========================================================================== */

MTS             : 'mts' ;
ZAMANI          : 'zamani' ;
SASA            : 'sasa' ;
REMEMBER        : 'remember' ;
RECALL          : 'recall' ;
WISDOM          : 'wisdom' ;

RESULT          : 'Result' ;
NEVER           : 'Never' ;

PI_KEYWORD      : 'Pi' ;
SIGMA_KEYWORD   : 'Sigma' ;


/* ============================================================================
 * BUILT-IN TYPES / BUILT-IN OPERATIONS
 * ========================================================================== */

VOID            : 'void' ;
INT             : 'int' ;
FLOAT_TYPE      : 'float' ;
BOOL_TYPE       : 'bool' ;
STR_TYPE        : 'str' ;
STRING_TYPE     : 'string' ;
CHAR_TYPE       : 'char' ;

PRINT           : 'print' ;
PRINTLN         : 'println' ;
PANIC           : 'panic' ;
LEN             : 'len' ;
SIZEOF          : 'sizeof' ;


/* ============================================================================
 * LOGICAL WORD OPERATORS
 *
 * NOT owns the word "not".
 * BANG owns the symbolic "!". This removes the old duplicate NOT conflict.
 * ========================================================================== */

IS              : 'is' ;
AND             : 'and' ;
OR              : 'or' ;
NOT             : 'not' ;


/* ============================================================================
 * ADDITIONAL LANGUAGE VOCABULARY
 * ========================================================================== */

SYSTEM          : 'system' ;
NANO            : 'nano' ;
PERFORM         : 'perform' ;

OMNIVERSAL      : 'omniversal' ;
ALIGNMENT       : 'alignment' ;
CONTAINMENT     : 'containment' ;

TRUST           : 'trust' ;
SOVEREIGNTY     : 'sovereignty' ;
GOAL            : 'goal' ;
BIONANO         : 'bionano' ;
REALITY         : 'reality' ;

NLP             : 'nlp' ;
ASI             : 'asi' ;
AESI            : 'aesi' ;
ASESI           : 'asesi' ;

ADMIN           : 'admin' ;
PAYMENT         : 'payment' ;
GATEWAY         : 'gateway' ;
GRAPHICS        : 'graphics' ;
VIDEO           : 'video' ;

ADJUST          : 'adjust' ;
VERSIONING      : 'versioning' ;
COPYRIGHT       : 'copyright' ;
NOTICE          : 'notice' ;
LEGAL           : 'legal' ;
ACTION          : 'action' ;
TAILOR          : 'tailor' ;
BUSINESS        : 'business' ;


/* ============================================================================
 * BOOLEAN LITERALS
 * ========================================================================== */

TRUE            : 'true' ;
FALSE           : 'false' ;


/* ============================================================================
 * HARDWARE / RESOURCE LITERALS
 *
 * These rules precede AT, MODULO and HASH so their complete lexical form is
 * emitted as one token.
 * ========================================================================== */

HARDWARE_WIDTH_LITERAL
    : DECIMAL_INTEGER 'bit'
    ;

HARDWARE_BYTE_WIDTH_LITERAL
    : DECIMAL_INTEGER 'byte'
    ;

HARDWARE_ALIGNMENT_LITERAL
    : DECIMAL_INTEGER 'align'
    ;

HARDWARE_ADDRESS_LITERAL
    : '@' HEX_INTEGER
    ;

HARDWARE_REGISTER_INDEX_LITERAL
    : '%' DECIMAL_INTEGER
    ;

HARDWARE_BANK_INDEX_LITERAL
    : '#' DECIMAL_INTEGER
    ;


/* ============================================================================
 * DURATION / SIZE LITERALS
 * ========================================================================== */

DURATION_LITERAL
    : DECIMAL_INTEGER DURATION_UNIT
    ;

SIZE_LITERAL
    : DECIMAL_INTEGER SIZE_UNIT
    ;


/* ============================================================================
 * NUMERIC LITERALS
 *
 * No machine width is encoded here.
 * ========================================================================== */

FLOAT
    : DECIMAL_DIGITS
      (
          '.' DECIMAL_DIGITS
      )
      EXPONENT?
    | DECIMAL_DIGITS EXPONENT
    ;

INTEGER
    : HEX_INTEGER
    | BINARY_INTEGER
    | OCTAL_INTEGER
    | DECIMAL_INTEGER
    ;


/* ============================================================================
 * QUANTUM STATE LITERALS
 *
 * A quantum ket/state literal has an explicit closing '>'.
 *
 * Examples:
 *
 *     |0>
 *     |1>
 *     |psi>
 *
 * A bare "|" remains PIPE.
 * ========================================================================== */

QUANTUM_LITERAL
    : '|' QUANTUM_STATE_BODY '>'
    ;


/* ============================================================================
 * STRING / CHARACTER LITERALS
 * ========================================================================== */

STRING
    : '"' STRING_CHARACTER* '"'
    ;

CHAR
    : '\'' CHARACTER_CONTENT '\''
    ;


/* ============================================================================
 * ANNOTATIONS
 *
 * AT is the sole token for "@". Hardware address literals are recognized
 * earlier when followed by a hexadecimal integer.
 * ========================================================================== */

AT
    : '@'
    ;


/* ============================================================================
 * OPERATORS
 *
 * Longer operators MUST precede their prefixes.
 * ========================================================================== */

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

ASSIGN
    : '='
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

LESS
    : '<'
    ;

GREATER
    : '>'
    ;

BANG
    : '!'
    ;


/* ============================================================================
 * PUNCTUATION
 * ========================================================================== */

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

HASH
    : '#'
    ;


/* ============================================================================
 * COMMENTS
 *
 * Comments precede SLASH and STAR so comment prefixes are recognized before
 * ordinary operators.
 * ========================================================================== */

DOC_LINE_COMMENT
    : '///' ~[\r\n]* -> channel(HIDDEN)
    ;

DOC_BLOCK_COMMENT
    : '/**'
      ( '*' ~[/] | ~'*' )*
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
      ( '*' ~[/] | ~'*' )*
      '*/'
      -> channel(HIDDEN)
    ;


/* ============================================================================
 * LEXICAL ERROR TOKENS
 *
 * These are deliberately explicit rather than a generic catch-all token.
 * ========================================================================== */

UNTERMINATED_DOC_BLOCK_COMMENT
    : '/**'
      ( '*' ~[/] | ~'*' )*
      EOF
    ;

UNTERMINATED_BLOCK_COMMENT
    : '/*'
      ( '*' ~[/] | ~'*' )*
      EOF
    ;

UNTERMINATED_STRING
    : '"'
      ( '\\' . | ~["\\\r\n] )*
      ( '\r' | '\n' | EOF )
    ;

UNTERMINATED_CHARACTER
    : '\''
      ( '\\' . | ~['\\\r\n] )*
      ( '\r' | '\n' | EOF )
    ;


/* ============================================================================
 * IDENTIFIER
 *
 * Domain-specific names remain identifiers.
 *
 * This includes:
 *
 *   quantum operations
 *   AI models
 *   agents
 *   hardware devices
 *   vendor operations
 *   mathematical functions
 *   tensors
 *   HDL components
 *   network services
 *   future computational constructs
 *
 * Identifier length is not artificially bounded.
 * ========================================================================== */

IDENTIFIER
    : IDENTIFIER_START IDENTIFIER_CONTINUE*
    ;


/* ============================================================================
 * WHITESPACE
 * ========================================================================== */

WS
    : [ \t\r\n\u000B\u000C]+ -> channel(HIDDEN)
    ;


/* ============================================================================
 * IDENTIFIER FRAGMENTS
 * ========================================================================== */

fragment IDENTIFIER_START
    : [A-Z]
    | [a-z]
    | '_'
    | '\u0080'..'\uFFFF'
    ;

fragment IDENTIFIER_CONTINUE
    : IDENTIFIER_START
    | [0-9]
    ;


/* ============================================================================
 * INTEGER FRAGMENTS
 * ========================================================================== */

fragment DECIMAL_INTEGER
    : [0-9]+
    ;

fragment DECIMAL_DIGITS
    : [0-9]+
    ;

fragment HEX_INTEGER
    : '0' [xX] HEX_DIGIT+
    ;

fragment BINARY_INTEGER
    : '0' [bB] [01]+
    ;

fragment OCTAL_INTEGER
    : '0' [oO] [0-7]+
    ;

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;


/* ============================================================================
 * FLOAT FRAGMENTS
 * ========================================================================== */

fragment EXPONENT
    : [eE] [+-]? DECIMAL_DIGITS
    ;


/* ============================================================================
 * STRING / CHARACTER FRAGMENTS
 * ========================================================================== */

fragment STRING_CHARACTER
    : ESCAPE_SEQUENCE
    | ~["\\\r\n]
    ;

fragment CHARACTER_CONTENT
    : ESCAPE_SEQUENCE
    | ~['\\\r\n]
    ;

fragment ESCAPE_SEQUENCE
    : '\\'
      (
          'n'
        | 'r'
        | 't'
        | 'b'
        | 'f'
        | '0'
        | '\\'
        | '"'
        | '\''
        | 'u'
        | 'U'
        | 'x'
      )
    ;


/* ============================================================================
 * QUANTUM STATE FRAGMENTS
 * ========================================================================== */

fragment QUANTUM_STATE_BODY
    : QUANTUM_STATE_CHARACTER+
    ;

fragment QUANTUM_STATE_CHARACTER
    : [A-Za-z0-9_]
    | '.'
    | '+'
    | '-'
    | '/'
    | '\\'
    | '\u0080'..'\uFFFF'
    ;


/* ============================================================================
 * DURATION UNITS
 *
 * The lexical unit is deliberately open to the language-defined duration
 * vocabulary without encoding a machine timing limit.
 * ========================================================================== */

fragment DURATION_UNIT
    : 'ns'
    | 'us'
    | 'µs'
    | 'ms'
    | 's'
    | 'min'
    | 'h'
    | 'd'
    ;


/* ============================================================================
 * SIZE UNITS
 * ========================================================================== */

fragment SIZE_UNIT
    : 'B'
    | 'KB'
    | 'KiB'
    | 'MB'
    | 'MiB'
    | 'GB'
    | 'GiB'
    | 'TB'
    | 'TiB'
    | 'PB'
    | 'PiB'
    | 'EB'
    | 'EiB'
    | 'ZB'
    | 'ZiB'
    | 'YB'
    | 'YiB'
    ;


/* ============================================================================
 * FINAL FALLBACK
 *
 * No generic ERROR_CHAR rule is provided.
 *
 * Invalid characters must be surfaced by ANTLR's normal lexical error
 * mechanism rather than silently converted into an apparently valid token.
 * ========================================================================== */