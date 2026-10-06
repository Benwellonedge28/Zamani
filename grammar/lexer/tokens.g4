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
 *     Single authoritative emitted-token grammar for the Zamani language.
 *
 * Architecture:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer.g4
 *       |
 *       v
 *     lexer/lexer.g4
 *       |
 *       v
 *     ZamaniTokens
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic model
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 *     classical                      quantum::ir
 *       |                               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *               optimization/lowering
 *                       |
 *                       v
 *              routing/scheduling/etc.
 *                       |
 *                       v
 *                 target realization
 *
 * Rust baseline:
 *     Rust 1.97+
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     No embedded Rust actions.
 *     No unsafe Rust.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - canonical emitted token names;
 *   - keyword token spellings;
 *   - operator token spellings;
 *   - punctuation token spellings;
 *   - identifiers;
 *   - primitive literals;
 *   - numeric literals;
 *   - quantum literals;
 *   - hardware/resource literals;
 *   - duration literals;
 *   - size literals;
 *   - annotations;
 *   - comments;
 *   - whitespace;
 *   - lexical-error categories;
 *   - lexical fragments required by the above.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - parser productions;
 *   - AST construction;
 *   - type semantics;
 *   - name resolution;
 *   - effect semantics;
 *   - capability negotiation;
 *   - resource negotiation;
 *   - policies;
 *   - contracts;
 *   - provenance semantics;
 *   - quantum routing;
 *   - quantum scheduling;
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
 * Every emitted lexical spelling has exactly one canonical token identity.
 *
 * The following files remain lexical design/reference partitions:
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
 *   unicode.g4
 *
 * They must not create a second production token authority.
 *
 * Their definitions are reconciled into this grammar before production
 * generation.
 *
 * ============================================================================
 * TOKEN CANONICALIZATION
 * ============================================================================
 *
 * The production token identity is canonicalized as follows:
 *
 *     "->"  -> THIN_ARROW
 *     "&"   -> AMPERSAND
 *     "|"   -> PIPE
 *     "?"   -> QUESTION_MARK
 *
 * The following historical/internal duplicate identities must not be emitted:
 *
 *     Arrow
 *     BitAnd
 *     BitOr
 *     Question
 *
 * Compatibility code may temporarily recognize legacy Rust enum names, but
 * generated lexical streams must contain only the canonical identities.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO artificial language-level limits.
 *
 * It does not define limits for:
 *
 *   qubits
 *   quantum registers
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
 *   processes
 *   actors
 *   memory
 *   storage
 *   registers
 *   register width
 *   tensor rank
 *   tensor dimensions
 *   network size
 *   topology size
 *   source size
 *   identifier length
 *   literal magnitude
 *
 * Numeric values represent source values.
 *
 * Resource feasibility belongs downstream:
 *
 *     semantic analysis
 *         ->
 *     resource analysis
 *         ->
 *     capability negotiation
 *         ->
 *     execution planning
 *         ->
 *     target realization
 *
 * ============================================================================
 * QUANTUM SCALABILITY
 * ============================================================================
 *
 * Quantum operation names are identifiers.
 *
 * This grammar intentionally does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CX
 *     CNOT
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * or any other finite physical gate catalogue.
 *
 * Built-in, custom, parameterized, vendor, decomposed and future operations
 * remain lexically representable through IDENTIFIER.
 *
 * Their semantics eventually cross:
 *
 *     quantum::ir
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Lexical validity is target-independent.
 *
 * The same source vocabulary must remain usable for:
 *
 *   embedded systems
 *   CPUs
 *   multicore systems
 *   GPUs
 *   FPGAs
 *   ASICs
 *   accelerators
 *   QPUs
 *   simulators
 *   HPC systems
 *   clusters
 *   distributed systems
 *   cloud systems
 *   future execution substrates
 *
 * Target feasibility is not lexical validity.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Lexical classification depends only on:
 *
 *   source characters;
 *   lexical grammar;
 *   explicitly selected language/compatibility version.
 *
 * It must not depend on:
 *
 *   hardware;
 *   filesystem state;
 *   network state;
 *   runtime state;
 *   wall-clock time;
 *   random state;
 *   target availability;
 *   resource availability.
 *
 * ============================================================================
 */

lexer grammar ZamaniTokens;


/* ============================================================================
 * CORE DECLARATION / CONTROL KEYWORDS
 * ========================================================================== */

FN              : 'fn' ;
LET             : 'let' ;
VAR             : 'var' ;
MUT             : 'mut' ;
CONST           : 'const' ;
RETURN          : 'return' ;

IF              : 'if' ;
ELSE            : 'else' ;
FOR             : 'for' ;
IN              : 'in' ;
WHILE           : 'while' ;
LOOP            : 'loop' ;
BREAK           : 'break' ;
CONTINUE        : 'continue' ;
MATCH           : 'match' ;
CASE            : 'case' ;
WHEN            : 'when' ;
THEN            : 'then' ;
YIELD           : 'yield' ;
SWITCH          : 'switch' ;

MODULE          : 'module' ;
IMPORT          : 'import' ;
EXPORT          : 'export' ;
USE             : 'use' ;
FROM            : 'from' ;
AS              : 'as' ;
PACKAGE         : 'package' ;

TYPE            : 'type' ;
STRUCT          : 'struct' ;
ENUM            : 'enum' ;
TRAIT           : 'trait' ;
IMPL            : 'impl' ;
CLASS           : 'class' ;
INTERFACE       : 'interface' ;
RECORD          : 'record' ;
UNION           : 'union' ;
ALIAS           : 'alias' ;

SEALED          : 'sealed' ;
PARTIAL         : 'partial' ;

PUBLIC          : 'public' ;
PUB             : 'pub' ;
PRIVATE         : 'private' ;
PROTECTED       : 'protected' ;
INTERNAL        : 'internal' ;

STATIC          : 'static' ;
OVERRIDE        : 'override' ;
VIRTUAL         : 'virtual' ;
ABSTRACT        : 'abstract' ;
FINAL           : 'final' ;

EXTENDS         : 'extends' ;
IMPLEMENTS      : 'implements' ;
THIS            : 'this' ;
SELF            : 'self' ;
SUPER           : 'super' ;
NEW             : 'new' ;
WHERE           : 'where' ;


/* ============================================================================
 * CONCURRENCY / ASYNCHRONY / CONTROL
 * ========================================================================== */

ASYNC           : 'async' ;
AWAIT           : 'await' ;
SPAWN           : 'spawn' ;
PARALLEL        : 'parallel' ;

TRY             : 'try' ;
CATCH           : 'catch' ;
FINALLY         : 'finally' ;
THROW           : 'throw' ;
HANDLE          : 'handle' ;


/* ============================================================================
 * EFFECTS
 * ========================================================================== */

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

UNSAFE          : 'unsafe' ;


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

PROVE           : 'prove' ;
VERIFY          : 'verify' ;
VALIDATE        : 'validate' ;
REFINE          : 'refine' ;
REFINEMENT      : 'refinement' ;


/* ============================================================================
 * RESOURCE / CAPABILITY / PORTABILITY VOCABULARY
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

ANCESTOR        : 'ancestor' ;


/* ============================================================================
 * LEARNING / ADAPTATION
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

MOVE            : 'move' ;


/* ============================================================================
 * UNCERTAINTY / PROBABILISTIC COMPUTATION
 * ========================================================================== */

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
QUANTUM_CLASSICAL
                : 'quantum_classical'
                ;

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
 * DISTRIBUTED / ACTORS / SERVICES
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
 * TYPE SYSTEM
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
 * LANGUAGE / COMPATIBILITY VOCABULARY
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

NIL             : 'nil' | 'null' ;

CODE            : 'code' ;


/* ============================================================================
 * BUILT-IN TYPES / BUILT-IN OPERATIONS
 * ========================================================================== */

VOID            : 'void' ;
INT             : 'int' ;
FLOAT_TYPE      : 'float' ;
BOOL_TYPE       : 'bool' ;

STR_TYPE        : 'str' ;
STRING_TYPE     : 'string' | 'String' ;
CHAR_TYPE       : 'char' ;

PRINT           : 'print' ;
PRINTLN         : 'println' ;
PANIC           : 'panic' ;
LEN             : 'len' ;
SIZEOF          : 'sizeof' ;


/* ============================================================================
 * LOGICAL WORD OPERATORS
 *
 * "not" is a word operator.
 * "!" is BANG.
 * They are distinct source spellings and therefore distinct token kinds.
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
 * UNICODE SYMBOLIC KEYWORDS / MATHEMATICAL SYMBOLS
 * ========================================================================== */

SIGMA_SYMBOL    : 'Σ' ;
PI_SYMBOL       : 'Π' ;


/* ============================================================================
 * HARDWARE / RESOURCE LITERALS
 *
 * These rules precede AT, MODULO and HASH because their complete lexical
 * forms must be recognized before their individual punctuation components.
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
 * DURATION LITERALS
 *
 * Units remain open through the lexical unit vocabulary rather than imposing
 * any physical timing limitation.
 * ========================================================================== */

DURATION_LITERAL
    : DURATION_NUMBER DURATION_UNIT
    ;

DURATION_NUMBER
    : DECIMAL_DIGITS
      ('.' DECIMAL_DIGITS)?
      DURATION_EXPONENT?
    ;

fragment DURATION_EXPONENT
    : [eE] [+-]? DECIMAL_DIGITS
    ;

fragment DURATION_UNIT
    : 'fs'
    | 'ps'
    | 'ns'
    | 'us'
    | 'µs'
    | 'μs'
    | 'ms'
    | 's'
    | 'min'
    | 'h'
    | 'd'
    | 'wk'
    ;


/* ============================================================================
 * SIZE / DATA QUANTITY LITERALS
 *
 * Both byte-oriented and bit-oriented units are supported.
 * ========================================================================== */

SIZE_LITERAL
    : SIZE_NUMBER SIZE_UNIT
    ;

SIZE_NUMBER
    : DECIMAL_DIGITS
      ('.' DECIMAL_DIGITS)?
      SIZE_EXPONENT?
    ;

fragment SIZE_EXPONENT
    : [eE] [+-]? DECIMAL_DIGITS
    ;

fragment SIZE_UNIT
    : 'Kibit'
    | 'Mibit'
    | 'Gibit'
    | 'Tibit'
    | 'Pibit'
    | 'Eibit'
    | 'Zibit'
    | 'Yibit'
    | 'KiB'
    | 'MiB'
    | 'GiB'
    | 'TiB'
    | 'PiB'
    | 'EiB'
    | 'ZiB'
    | 'YiB'
    | 'kbit'
    | 'Mbit'
    | 'Gbit'
    | 'Tbit'
    | 'Pbit'
    | 'Ebit'
    | 'Zbit'
    | 'Ybit'
    | 'bit'
    | 'kB'
    | 'KB'
    | 'MB'
    | 'GB'
    | 'TB'
    | 'PB'
    | 'EB'
    | 'ZB'
    | 'YB'
    | 'B'
    | 'byte'
    ;


/* ============================================================================
 * NUMERIC LITERALS
 *
 * The lexer recognizes mathematical/source magnitude.
 *
 * It does NOT constrain the value to a target machine integer width.
 * ========================================================================== */

FLOAT
    : DECIMAL_DIGITS
      '.'
      DECIMAL_DIGITS
      EXPONENT_PART?
    | DECIMAL_DIGITS
      EXPONENT_PART
    | '.'
      DECIMAL_DIGITS
      EXPONENT_PART?
    ;

INTEGER
    : HEX_INTEGER
    | BINARY_INTEGER
    | OCTAL_INTEGER
    | DECIMAL_INTEGER
    ;

fragment DECIMAL_INTEGER
    : DECIMAL_DIGITS
    ;

fragment DECIMAL_DIGITS
    : DECIMAL_DIGIT
      (DECIMAL_DIGIT | '_')*
      DECIMAL_DIGIT?
    ;

fragment EXPONENT_PART
    : [eE]
      [+-]?
      DECIMAL_DIGITS
    ;

fragment DECIMAL_DIGIT
    : [0-9]
    ;

fragment DECIMAL_DIGIT_OR_SEPARATOR
    : [0-9_]
    ;

fragment BIN_DIGIT
    : [01]
    ;

fragment OCT_DIGIT
    : [0-7]
    ;

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;

fragment HEX_INTEGER
    : '0' [xX]
      HEX_DIGIT
      (HEX_DIGIT | '_')*
      HEX_DIGIT?
    ;

fragment BINARY_INTEGER
    : '0' [bB]
      BIN_DIGIT
      (BIN_DIGIT | '_')*
      BIN_DIGIT?
    ;

fragment OCTAL_INTEGER
    : '0' [oO]
      OCT_DIGIT
      (OCT_DIGIT | '_')*
      OCT_DIGIT?
    ;


/* ============================================================================
 * QUANTUM STATE LITERALS
 *
 * Supported closing delimiters:
 *
 *     >
 *     ⟩
 *
 * Both represent source-level ket notation.
 *
 * A bare "|" remains PIPE.
 *
 * Quantum operations remain identifiers.
 * ========================================================================== */

QUANTUM_LITERAL
    : '|'
      QUANTUM_STATE_BODY
      ('>' | '⟩')
    ;

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
 * STRING LITERALS
 * ========================================================================== */

STRING
    : '"'
      STRING_CHARACTER*
      '"'
    ;

UNTERMINATED_STRING
    : '"'
      (ESCAPE_SEQUENCE | ~["\\\r\n])*
      ( '\r' | '\n' | EOF )
    ;

fragment STRING_CHARACTER
    : ESCAPE_SEQUENCE
    | ~["\\\r\n]
    ;

fragment STRING_TERMINATOR
    : '"'
    ;


/* ============================================================================
 * CHARACTER LITERALS
 * ========================================================================== */

CHAR
    : '\''
      CHARACTER_CONTENT
      '\''
    ;

UNTERMINATED_CHARACTER
    : '\''
      (ESCAPE_SEQUENCE | ~['\\\r\n])*
      ( '\r' | '\n' | EOF )
    ;

fragment CHARACTER_CONTENT
    : ESCAPE_SEQUENCE
    | ~['\\\r\n]
    ;

fragment CHARACTER
    : CHARACTER_CONTENT
    ;

fragment CHARACTER_ESCAPE
    : ESCAPE_SEQUENCE
    ;


/* ============================================================================
 * ESCAPE / UNICODE FRAGMENTS
 * ========================================================================== */

fragment ESCAPE_SEQUENCE
    : '\\'
      (
          'n'
        | 'r'
        | 't'
        | 'b'
        | 'f'
        | 'v'
        | '0'
        | '\\'
        | '"'
        | '\''
        | 'u' UNICODE_ESCAPE
        | 'U' UNICODE_LONG_ESCAPE
        | 'x' HEX_ESCAPE
      )
    ;

fragment UNICODE_ESCAPE
    : HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
    ;

fragment UNICODE_LONG_ESCAPE
    : HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
      HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
    ;

fragment HEX_ESCAPE
    : HEX_DIGIT HEX_DIGIT
    ;


/* ============================================================================
 * ANNOTATIONS
 *
 * AT is the single canonical token for "@", except where a longer
 * HARDWARE_ADDRESS_LITERAL has already matched.
 *
 * Nano/domain annotation names are NOT individual token kinds.
 *
 * Example:
 *
 *     @atom
 *
 * becomes:
 *
 *     AT IDENTIFIER
 *
 * This keeps the language extensible.
 * ========================================================================== */

AT
    : '@'
    ;


/* ============================================================================
 * OPERATORS
 *
 * Longer operators precede their prefixes.
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

QUESTION_MARK
    : '?'
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
 * Comments are hidden from the parser but remain available to tooling.
 * ========================================================================== */

DOC_LINE_COMMENT
    : '///'
      ~[\r\n]*
      -> channel(HIDDEN)
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
 * LEXICAL ERROR CATEGORIES
 *
 * There is intentionally NO universal catch-all ERROR_CHAR rule.
 *
 * Unknown source characters must be surfaced through the canonical lexer
 * diagnostic mechanism rather than silently becoming a valid token.
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


/* ============================================================================
 * IDENTIFIERS
 *
 * Identifiers remain open-ended.
 *
 * They cover:
 *
 *   quantum operation names
 *   AI model names
 *   tensor names
 *   mathematical functions
 *   HDL components
 *   hardware devices
 *   vendor operations
 *   network services
 *   domain names
 *   future computational constructs
 *
 * No artificial identifier-length limit is encoded.
 * ========================================================================== */

IDENTIFIER
    : IDENTIFIER_START
      IDENTIFIER_CONTINUE*
    ;

fragment IDENTIFIER_START
    : [A-Z]
    | [a-z]
    | '_'
    | UNICODE_IDENTIFIER_START
    ;

fragment IDENTIFIER_CONTINUE
    : IDENTIFIER_START
    | [0-9]
    | UNICODE_IDENTIFIER_CONTINUE
    ;


/* ============================================================================
 * UNICODE IDENTIFIER SUPPORT
 *
 * These fragments intentionally avoid imposing a fixed identifier-length
 * ceiling.
 *
 * Source decoding must first guarantee valid Unicode scalar values.
 * ========================================================================== */

fragment UNICODE_IDENTIFIER_START
    : '\u0080'..'\uFFFF'
    ;

fragment UNICODE_IDENTIFIER_CONTINUE
    : '\u0080'..'\uFFFF'
    ;

fragment UNICODE_LETTER
    : '\u0080'..'\uFFFF'
    ;

fragment UNICODE_LETTER_NUMBER
    : '\u2160'..'\u2188'
    ;

fragment UNICODE_DECIMAL_DIGIT
    : '\u0660'..'\u0669'
    ;

fragment UNICODE_NONSPACING_MARK
    : '\u0300'..'\u036F'
    ;

fragment UNICODE_SPACING_COMBINING_MARK
    : '\u0900'..'\u0903'
    ;

fragment UNICODE_COMBINING_MARK
    : '\u20D0'..'\u20FF'
    ;

fragment UNICODE_CONNECTOR_PUNCTUATION
    : '\u005F'
    ;

fragment ZAMANI_LINE_FEED
    : '\u000A'
    ;

fragment ZAMANI_CARRIAGE_RETURN
    : '\u000D'
    ;

fragment ZAMANI_LINE_TERMINATOR
    : ZAMANI_LINE_FEED
    | ZAMANI_CARRIAGE_RETURN
    ;

fragment UNICODE_LINE_SEPARATOR
    : '\u2028'
    ;

fragment UNICODE_PARAGRAPH_SEPARATOR
    : '\u2029'
    ;


/* ============================================================================
 * WHITESPACE
 * ========================================================================== */

WS
    : [ \t\r\n\u000B\u000C]+
      -> channel(HIDDEN)
    ;


/* ============================================================================
 * FINAL ARCHITECTURAL INVARIANTS
 * ============================================================================
 *
 * 1. One source spelling has one canonical emitted token identity.
 *
 * 2. One emitted token has one lexical owner.
 *
 * 3. Domain grammars do not create competing lexical authorities.
 *
 * 4. Quantum operations are identifiers, not a finite lexer catalogue.
 *
 * 5. Hardware scale is not encoded in lexical rules.
 *
 * 6. Resource availability is not lexical validity.
 *
 * 7. Tokenization is deterministic and target-independent.
 *
 * 8. Rust implementation remains safe.
 *
 * 9. Rust 1.97+ is supported.
 *
 * 10. Parser and AST layers receive canonical tokens.
 *
 * 11. Semantic interpretation remains downstream.
 *
 * 12. Classical and quantum computation share the same lexical foundation.
 *
 * 13. HDL and hardware syntax share the same lexical foundation.
 *
 * 14. AI, data, distributed and networking constructs share the same lexical
 *     foundation.
 *
 * 15. Future domains can use IDENTIFIER and extensible syntax without
 *     requiring a universal token catalogue.
 *
 * 16. No MAX_* machine-capacity constants exist in this grammar.
 *
 * ============================================================================
 */


OPTIONAL       : 'optional' ;
REQUIRED       : 'required' ;
NULLABLE       : 'nullable' ;
MUTABLE        : 'mutable' ;
TRANSIENT      : 'transient' ;
SENSITIVE      : 'sensitive' ;
DEPRECATED     : 'deprecated' ;

DEFAULT        : 'default' ;
COMPUTED       : 'computed' ;
BY             : 'by' ;

KEY            : 'key' ;
PRIMARY        : 'primary' ;
UNIQUE         : 'unique' ;
ALTERNATE      : 'alternate' ;
NATURAL        : 'natural' ;
CANDIDATE      : 'candidate' ;

INDEX          : 'index' ;

CHECK          : 'check' ;
VALIDATION     : 'validation' ;
INTEGRITY      : 'integrity' ;

RELATION       : 'relation' ;
CARDINALITY    : 'cardinality' ;

PARTITION      : 'partition' ;
RANGE          : 'range' ;
DOMAIN         : 'domain' ;
ADAPTIVE       : 'adaptive' ;
AUTOMATIC      : 'automatic' ;

ORDER          : 'order' ;

EVOLVE         : 'evolve' ;
VERSION        : 'version' ;
ADD            : 'add' ;
FIELD          : 'field' ;
REMOVE         : 'remove' ;
RENAME         : 'rename' ;
TO             : 'to' ;
ALTER          : 'alter' ;
NONNULLABLE    : 'nonnull' ;

ENCODING       : 'encoding' ;