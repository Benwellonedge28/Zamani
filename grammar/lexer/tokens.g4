
/*
 * ============================================================================
 * Zamani Programming Language — Canonical Token Vocabulary
 * File: grammar/lexer/tokens.g4
 * Grammar: ZamaniTokens
 *
 * Contract:
 *   - One authoritative emitted-token vocabulary.
 *   - One canonical identity for each lexical spelling.
 *   - No parser productions, AST construction, or target realization.
 *   - No language-level hardware or resource capacity limits.
 *   - No embedded Rust actions; compiler implementations use safe Rust.
 *
 * Integration:
 *   grammar/antlr/ZamaniLexer.g4
 *       imports lexer
 *   grammar/lexer/lexer.g4
 *       imports ZamaniTokens
 *   parser grammars
 *       use tokenVocab=ZamaniLexer
 *
 * The lexer recognizes syntax only. Types, effects, capabilities, resources,
 * policies, provenance, and domain semantics are checked downstream.
 *
 * Quantum operation names remain identifiers. Quantum semantics ultimately
 * map through the canonical quantum::ir boundary.
 * ============================================================================
 */

lexer grammar ZamaniTokens;

// ============================================================================
// 1. Core declarations, names, modules, and visibility
// ============================================================================

FN              : 'fn';
LET             : 'let';
VAR             : 'var';
MUT             : 'mut';
CONST           : 'const';
RETURN          : 'return';
TYPE            : 'type';
STRUCT          : 'struct';
RECORD          : 'record';
ENUM            : 'enum';
UNION           : 'union';
TRAIT           : 'trait';
IMPL            : 'impl';
CLASS           : 'class';
INTERFACE       : 'interface';
ALIAS           : 'alias';
SEALED          : 'sealed';
PARTIAL         : 'partial';

MODULE          : 'module';
PACKAGE         : 'package';
IMPORT          : 'import';
EXPORT          : 'export';
USE             : 'use';
FROM            : 'from';
AS              : 'as';
WHERE           : 'where';
EXTENDS         : 'extends';
IMPLEMENTS      : 'implements';
NEW             : 'new';
THIS            : 'this';
SELF            : 'self';
SUPER            : 'super';

PUBLIC          : 'public';
PUB             : 'pub';
PRIVATE         : 'private';
PROTECTED       : 'protected';
INTERNAL        : 'internal';
STATIC          : 'static';
OVERRIDE        : 'override';
VIRTUAL         : 'virtual';
ABSTRACT        : 'abstract';
FINAL           : 'final';

// ============================================================================
// 2. Control flow, patterns, functions, and asynchronous execution
// ============================================================================

IF              : 'if';
ELSE            : 'else';
FOR             : 'for';
IN              : 'in';
WHILE           : 'while';
LOOP            : 'loop';
BREAK           : 'break';
CONTINUE        : 'continue';
MATCH           : 'match';
CASE            : 'case';
WHEN            : 'when';
THEN            : 'then';
SWITCH          : 'switch';
YIELD           : 'yield';
TRY             : 'try';
CATCH           : 'catch';
FINALLY         : 'finally';
THROW           : 'throw';
HANDLE          : 'handle';

ASYNC           : 'async';
AWAIT           : 'await';
SPAWN           : 'spawn';
PARALLEL        : 'parallel';
MOVE            : 'move';
WITH            : 'with';
EXTERN          : 'extern';
MACRO           : 'macro';
INLINE          : 'inline';
COMPTIME        : 'comptime';
COMPILE         : 'compile';

// ============================================================================
// 3. Types and type-level constructs
// ============================================================================

LINEAR          : 'linear';
AFFINE          : 'affine';
IMMUTABLE       : 'immutable';
VOLATILE        : 'volatile';
OPTION          : 'option';
SOME            : 'some';
NONE            : 'none';
VOID            : 'void';
INT_TYPE        : 'int';
FLOAT_TYPE      : 'float';
BOOL_TYPE       : 'bool';
STR_TYPE        : 'str';
STRING_TYPE     : 'string';
CHAR_TYPE       : 'char';
NIL             : 'nil';
IS              : 'is';
SIZEOF          : 'sizeof';
LEN             : 'len';

// ============================================================================
// 4. Effects and effect declarations
// ============================================================================

EFFECT          : 'effect';
EFFECTS         : 'effects';
PURE            : 'pure';
IO              : 'io';
NETWORK         : 'network';
MUTATION        : 'mutation';
RANDOMNESS      : 'randomness';
NATIVE          : 'native';
FOREIGN         : 'foreign';
DISTRIBUTED     : 'distributed';
MEASUREMENT     : 'measurement';
LEARNING        : 'learning';
ADAPTATION      : 'adaptation';
REFLECTION      : 'reflection';
CODE_GENERATION : 'code_generation';
SIMULATION      : 'simulation';

// This token describes source vocabulary only. It does not authorize unsafe
// implementation code or bypass compiler/runtime safety policy.
UNSAFE          : 'unsafe';

// ============================================================================
// 5. Contracts, assertions, verification, and refinements
// ============================================================================

REQUIRES        : 'requires';
ENSURES         : 'ensures';
INVARIANT       : 'invariant';
ASSUME          : 'assume';
GUARANTEE       : 'guarantee';
PROPERTY        : 'property';
PRECONDITION    : 'precondition';
POSTCONDITION   : 'postcondition';
CONTRACT        : 'contract';
ASSERT          : 'assert';
PROVE           : 'prove';
VERIFY          : 'verify';
VALIDATE        : 'validate';
REFINE          : 'refine';
REFINEMENT      : 'refinement';

// ============================================================================
// 6. Resources, capabilities, constraints, and portability
// ============================================================================

RESOURCE        : 'resource';
RESOURCES       : 'resources';
QUANTITY        : 'quantity';
CAPABILITY      : 'capability';
CAPABILITIES    : 'capabilities';
REQUIREMENT     : 'requirement';
REQUIREMENTS    : 'requirements';
CONSTRAINT      : 'constraint';
CONSTRAINTS     : 'constraints';
PREFER          : 'prefer';
PREFERENCE      : 'preference';
PREFERENCES     : 'preferences';
HINT            : 'hint';
HINTS           : 'hints';
TARGET          : 'target';
TARGETS         : 'targets';
CAPACITY        : 'capacity';
AVAILABILITY    : 'availability';
PORTABILITY     : 'portability';
SCALABILITY     : 'scalability';
PERFORMANCE     : 'performance';
LATENCY         : 'latency';
THROUGHPUT      : 'throughput';
BANDWIDTH       : 'bandwidth';
ENERGY          : 'energy';
POWER           : 'power';
RELIABILITY     : 'reliability';
RESILIENCE      : 'resilience';
COST            : 'cost';
RESERVE         : 'reserve';
ACQUIRE         : 'acquire';
RELEASE         : 'release';
DERIVE          : 'derive';
GROUP           : 'group';
PROFILE         : 'profile';

// ============================================================================
// 7. Policies, authorization intent, recovery, and reproducibility
// ============================================================================

POLICY          : 'policy';
POLICIES        : 'policies';
ALLOW           : 'allow';
FORBID          : 'forbid';
PERMIT          : 'permit';
DENY            : 'deny';
FALLBACK        : 'fallback';
RETRY            : 'retry';
RECOVER         : 'recover';
ESCALATE        : 'escalate';
REJECT          : 'reject';
SELECT          : 'select';
NEGOTIATE       : 'negotiate';
SANDBOX         : 'sandbox';
SIMULATE        : 'simulate';
DETERMINISTIC   : 'deterministic';
REPRODUCIBLE    : 'reproducible';

// ============================================================================
// 8. Reasoning, knowledge, learning, and evidence
// ============================================================================

INFER           : 'infer';
DEDUCE          : 'deduce';
REASON          : 'reason';
PREMISE         : 'premise';
PREMISES        : 'premises';
CONCLUSION      : 'conclusion';
ASSUMPTION      : 'assumption';
OBSERVATION     : 'observation';
INTERVENTION    : 'intervention';
COUNTERFACTUAL  : 'counterfactual';
KNOWLEDGE       : 'knowledge';
ASSERT_KNOWLEDGE: 'assert_knowledge';
RETRACT         : 'retract';
QUERY           : 'query';
FACT            : 'fact';
FACTS           : 'facts';
RELATION        : 'relation';
RELATIONS       : 'relations';
ANCESTOR        : 'ancestor';

LEARN           : 'learn';
ADAPT            : 'adapt';
TRAIN           : 'train';
PREDICT         : 'predict';
EVALUATE        : 'evaluate';
FEEDBACK        : 'feedback';
MODEL           : 'model';
DATASET         : 'dataset';
OBJECTIVE       : 'objective';
TRANSFER        : 'transfer';
REINFORCEMENT   : 'reinforcement';
UPDATE          : 'update';

UNCERTAIN       : 'uncertain';
UNCERTAINTY     : 'uncertainty';
PROBABILITY     : 'probability';
PROBABILISTIC   : 'probabilistic';
DISTRIBUTION    : 'distribution';
CONFIDENCE      : 'confidence';
BELIEF          : 'belief';
LIKELIHOOD      : 'likelihood';

EVIDENCE        : 'evidence';
EXPLAIN         : 'explain';
EXPLANATION     : 'explanation';
PROVENANCE      : 'provenance';
SOURCE          : 'source';
DERIVATION      : 'derivation';
DECISION        : 'decision';
DECISIONS       : 'decisions';
GENERATED       : 'generated';
TRANSFORMED     : 'transformed';
VERIFIED        : 'verified';
AUDIT           : 'audit';
TRACE           : 'trace';
PATTERN         : 'pattern';
PATTERNS        : 'patterns';
GUARD           : 'guard';
GUARDS          : 'guards';

// ============================================================================
// 9. Quantum and hybrid computing
//
// Do not enumerate gates such as H, X, Y, Z, CNOT, RX, or future operations.
// Operation names are IDENTIFIER tokens; meaning is resolved downstream.
// ============================================================================

QUANTUM         : 'quantum';
CIRCUIT         : 'circuit';
QUBIT           : 'qubit';
APPLY           : 'apply';
MEASURE         : 'measure';
RESET           : 'reset';
BARRIER         : 'barrier';
CONTROL         : 'control';
ADJOINT         : 'adjoint';
INVERSE         : 'inverse';
OBSERVE         : 'observe';
ENTANGLE        : 'entangle';
NOISE           : 'noise';
FIDELITY        : 'fidelity';
SURFACE         : 'surface';
LOGICAL         : 'logical';
PARITY          : 'parity';
QEC             : 'qec';
RESILIENT       : 'resilient';
DYNAMIC         : 'dynamic';

HYBRID          : 'hybrid';
CLASSICAL       : 'classical';
QUANTUM_CLASSICAL: 'quantum_classical';
ACCELERATOR     : 'accelerator';
DEVICE          : 'device';
HOST            : 'host';
SUBMIT          : 'submit';
SYNC            : 'sync';

// ============================================================================
// 10. HDL and hardware intent
// ============================================================================

HDL             : 'hdl';
HARDWARE        : 'hardware';
SIGNAL          : 'signal';
CLOCK           : 'clock';
RESET_SIGNAL    : 'reset_signal';
TIMING          : 'timing';
SYNTHESIZE      : 'synthesize';
DEPLOY          : 'deploy';
REGISTER        : 'register';
MEMORY          : 'memory';
CHANNEL         : 'channel';
PORT            : 'port';
INPUT           : 'input';
OUTPUT          : 'output';
INOUT           : 'inout';

// ============================================================================
// 11. Concurrency, distribution, and networking
// ============================================================================

NODE            : 'node';
NODES           : 'nodes';
ACTOR           : 'actor';
ACTORS          : 'actors';
MESSAGE         : 'message';
MESSAGES        : 'messages';
SERVICE         : 'service';
SERVICES        : 'services';
ENDPOINT        : 'endpoint';
ENDPOINTS       : 'endpoints';
STREAM          : 'stream';
STREAMS         : 'streams';
DISCOVER        : 'discover';

// ============================================================================
// 12. AI and data vocabulary
// ============================================================================

AI              : 'ai';
AGENT           : 'agent';
AGENTS          : 'agents';
NEURAL          : 'neural';
SYMBOLIC        : 'symbolic';
NEURAL_SYMBOLIC : 'neural_symbolic';
COGNITIVE       : 'cognitive';
PLANNING        : 'planning';
GENERATION      : 'generation';
GENERATIVE      : 'generative';

SCHEMA          : 'schema';
SCHEMAS         : 'schemas';
GRAPH           : 'graph';
GRAPHS          : 'graphs';
TABLE           : 'table';
TABLES          : 'tables';
COLUMN          : 'column';
COLUMNS         : 'columns';
ROW             : 'row';
ROWS            : 'rows';
FILTER          : 'filter';
MAP             : 'map';
REDUCE          : 'reduce';
JSON            : 'json';
XML             : 'xml';
LANGUAGE        : 'language';

// ============================================================================
// 13. Reflection, metaprogramming, and syntax generation
// ============================================================================

REFLECTIVE      : 'reflective';
INTROSPECT      : 'introspect';
GENERATE        : 'generate';
QUOTE           : 'quote';
SYNTAX          : 'syntax';

// ============================================================================
// 14. Compatibility vocabulary
//
// Retained for source compatibility with existing parser/AST work. Before
// freezing, verify whether each spelling is normative, contextual, deprecated,
// or should become an ordinary identifier in a future language version.
// Application-specific behavior belongs in libraries, dialects, capabilities,
// and policies rather than in the universal language core.
// ============================================================================

ZAMANI          : 'zamani';
WISDOM          : 'wisdom';
SASA            : 'sasa';
SIGMA            : 'sigma';
PI               : 'pi';
CODE             : 'code';
PERFORM          : 'perform';
RECALL           : 'recall';
REMEMBER         : 'remember';
NANO             : 'nano';
MTS              : 'mts';
PRINT            : 'print';
PRINTLN          : 'println';
PANIC            : 'panic';
OMNIVERSAL       : 'omniversal';
ALIGNMENT        : 'alignment';
CONTAINMENT      : 'containment';
TRUST            : 'trust';
SOVEREIGNTY      : 'sovereignty';
GOAL             : 'goal';
BIONANO          : 'bionano';
REALITY          : 'reality';
NLP              : 'nlp';
ASI              : 'asi';
AESI             : 'aesi';
ASESI            : 'asesi';
ADMIN            : 'admin';
PAYMENT          : 'payment';
GATEWAY          : 'gateway';
GRAPHICS         : 'graphics';
VIDEO            : 'video';
ADJUST           : 'adjust';
VERSIONING       : 'versioning';
COPYRIGHT        : 'copyright';
NOTICE           : 'notice';
LEGAL            : 'legal';
ACTION           : 'action';
TAILOR           : 'tailor';
BUSINESS         : 'business';

// ============================================================================
// 15. Boolean literals
//
// Keep these distinct from identifiers and do not duplicate them in a
// separate imported lexer grammar.
// ============================================================================

TRUE            : 'true';
FALSE           : 'false';

// Unicode symbols with dedicated syntax-level meaning.
SIGMA_SYMBOL    : 'Σ';
PI_SYMBOL       : 'Π';

// ============================================================================
// 16. Source-level resource literals
//
// These represent quantities in source code, not actual machine capacities.
// Their interpretation and feasibility are semantic/runtime concerns.
// ============================================================================

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

DURATION_LITERAL
    : DECIMAL_NUMBER DURATION_UNIT
    ;

SIZE_LITERAL
    : DECIMAL_NUMBER SIZE_UNIT
    ;

fragment DURATION_UNIT
    : 'fs' | 'ps' | 'ns' | 'us' | 'µs' | 'μs'
    | 'ms' | 's' | 'min' | 'h' | 'd' | 'wk'
    ;

fragment SIZE_UNIT
    : 'Kibit' | 'Mibit' | 'Gibit' | 'Tibit' | 'Pibit'
    | 'Eibit' | 'Zibit' | 'Yibit'
    | 'KiB' | 'MiB' | 'GiB' | 'TiB' | 'PiB'
    | 'EiB' | 'ZiB' | 'YiB'
    | 'kbit' | 'Mbit' | 'Gbit' | 'Tbit' | 'Pbit'
    | 'Ebit' | 'Zbit' | 'Ybit'
    | 'bit'
    | 'kB' | 'KB' | 'MB' | 'GB' | 'TB' | 'PB'
    | 'EB' | 'ZB' | 'YB' | 'B' | 'byte'
    ;

// ============================================================================
// 17. Numeric literals
//
// The grammar does not constrain numeric magnitude to a target integer width.
// Numeric separators are lexical; validity of separator placement and numeric
// range is checked by literal validation.
// ============================================================================

FLOAT
    : DECIMAL_DIGITS '.' DECIMAL_DIGITS EXPONENT_PART?
    | DECIMAL_DIGITS EXPONENT_PART
    | '.' DECIMAL_DIGITS EXPONENT_PART?
    ;

INTEGER
    : HEX_INTEGER
    | BINARY_INTEGER
    | OCTAL_INTEGER
    | DECIMAL_INTEGER
    ;

fragment DECIMAL_NUMBER
    : DECIMAL_DIGITS ('.' DECIMAL_DIGITS)? EXPONENT_PART?
    ;

fragment DECIMAL_INTEGER
    : DECIMAL_DIGITS
    ;

fragment DECIMAL_DIGITS
    : [0-9] ([0-9] | '_')*
    ;

fragment EXPONENT_PART
    : [eE] [+-]? DECIMAL_DIGITS
    ;

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;

fragment HEX_INTEGER
    : '0' [xX] HEX_DIGIT (HEX_DIGIT | '_')*
    ;

fragment BINARY_INTEGER
    : '0' [bB] [01] ([01] | '_')*
    ;

fragment OCTAL_INTEGER
    : '0' [oO] [0-7] ([0-7] | '_')*
    ;

// ============================================================================
// 18. Quantum state notation
//
// This recognizes source notation only; it does not allocate or measure qubits.
// Quantum operations themselves remain identifiers.
// ============================================================================

QUANTUM_LITERAL
    : '|' QUANTUM_STATE_CHARACTER+ ('>' | '⟩')
    ;

fragment QUANTUM_STATE_CHARACTER
    : [A-Za-z0-9_]
    | '.'
    | '+'
    | '-'
    | '/'
    | '\\'
    | [\u0080-\uFFFF]
    ;

// ============================================================================
// 19. Strings, characters, and escape sequences
// ============================================================================

STRING
    : '"' STRING_CHARACTER* '"'
    ;

UNTERMINATED_STRING
    : '"' (ESCAPE_SEQUENCE | ~["\\\r\n])* ('\r' | '\n' | EOF)
    ;

fragment STRING_CHARACTER
    : ESCAPE_SEQUENCE
    | ~["\\\r\n]
    ;

CHAR
    : '\'' CHARACTER_CONTENT '\''
    ;

UNTERMINATED_CHARACTER
    : '\'' (ESCAPE_SEQUENCE | ~['\\\r\n])* ('\r' | '\n' | EOF)
    ;

fragment CHARACTER_CONTENT
    : ESCAPE_SEQUENCE
    | ~['\\\r\n]
    ;

fragment ESCAPE_SEQUENCE
    : '\\' (
          'n' | 'r' | 't' | 'b' | 'f' | 'v' | '0'
        | '\\' | '"' | '\''
        | 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
        | 'x' HEX_DIGIT HEX_DIGIT
      )
    ;

// ============================================================================
// 20. Annotations
//
// @name is lexed as AT IDENTIFIER. Annotation names are not hard-coded.
// ============================================================================

AT              : '@';

// ============================================================================
// 21. Operators
//
// Multi-character operators precede their shorter prefixes. ANTLR's longest
// match rule is also relied upon, but ordering is kept explicit for review.
// ============================================================================

ELLIPSIS        : '...';
DOT_DOT_EQ      : '..=';
DOT_DOT         : '..';

THIN_ARROW      : '->';
FAT_ARROW       : '=>';
DOUBLE_COLON    : '::';

EQUAL_EQUAL     : '==';
NOT_EQUAL       : '!=';
LESS_EQUAL      : '<=';
GREATER_EQUAL   : '>=';

LOGICAL_AND     : '&&';
LOGICAL_OR      : '||';
LEFT_SHIFT      : '<<';
RIGHT_SHIFT     : '>>';

PLUS_ASSIGN     : '+=';
MINUS_ASSIGN    : '-=';
STAR_ASSIGN     : '*=';
SLASH_ASSIGN    : '/=';
PERCENT_ASSIGN  : '%=';
AMP_ASSIGN      : '&=';
PIPE_ASSIGN     : '|=';
CARET_ASSIGN    : '^=';

INCREMENT       : '++';
DECREMENT       : '--';
QUESTION_DOT    : '?.';
NULL_COALESCE   : '??';

PLUS            : '+';
MINUS           : '-';
STAR            : '*';
SLASH           : '/';
MODULO          : '%';
ASSIGN          : '=';
AMPERSAND       : '&';
PIPE            : '|';
CARET           : '^';
TILDE           : '~';
LESS            : '<';
GREATER         : '>';
QUESTION_MARK   : '?';
BANG            : '!';

// ============================================================================
// 22. Structural punctuation
// ============================================================================

LPAREN          : '(';
RPAREN          : ')';
LBRACE          : '{';
RBRACE          : '}';
LBRACKET        : '[';
RBRACKET        : ']';
COMMA           : ',';
DOT             : '.';
SEMICOLON       : ';';
COLON           : ':';
HASH            : '#';

// ============================================================================
// 23. Comments
//
// Comments are retained on HIDDEN so tooling can preserve them. Documentation
// comments have distinct token identities for AST/documentation tooling.
// ============================================================================

DOC_LINE_COMMENT
    : '///' ~[\r\n]* -> channel(HIDDEN)
    ;

DOC_BLOCK_COMMENT
    : '/**' ( '*' ~[/] | ~'*' )* '*/' -> channel(HIDDEN)
    ;

LINE_COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;

BLOCK_COMMENT
    : '/*' ( '*' ~[/] | ~'*' )* '*/' -> channel(HIDDEN)
    ;

UNTERMINATED_DOC_BLOCK_COMMENT
    : '/**' ( '*' ~[/] | ~'*' )* EOF
    ;

UNTERMINATED_BLOCK_COMMENT
    : '/*' ( '*' ~[/] | ~'*' )* EOF
    ;

// ============================================================================
// 24. Identifiers
//
// Unicode categories are intentionally broad enough to avoid ASCII-only
// identifiers. The normative Unicode profile must specify normalization,
// bidi controls, joiners, and identifier-security diagnostics.
// ============================================================================

IDENTIFIER
    : IDENTIFIER_START IDENTIFIER_CONTINUE*
    ;

fragment IDENTIFIER_START
    : [A-Z]
    | [a-z]
    | '_'
    | [\u0080-\uFFFF]
    ;

fragment IDENTIFIER_CONTINUE
    : IDENTIFIER_START
    | [0-9]
    ;

// ============================================================================
// 25. Whitespace
//
// Whitespace is hidden from parser rules. Line/column accounting and exact
// source spans remain the lexer/runtime responsibility.
// ============================================================================

WS
    : [ \t\r\n\u000B\u000C\u0085\u00A0\u2028\u2029]+
      -> channel(HIDDEN)
    ;
