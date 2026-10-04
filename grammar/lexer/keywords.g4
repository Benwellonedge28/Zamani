/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/keywords.g4
 *
 * Grammar:
 *     ZamaniKeywords
 *
 * Role:
 *     Canonical lexical vocabulary for RESERVED Zamani keywords.
 *
 * Intended composition:
 *
 *     grammar/lexer/keywords.g4
 *                 |
 *                 v
 *     grammar/lexer/tokens.g4
 *                 |
 *                 v
 *     grammar/antlr/ZamaniLexer.g4
 *                 |
 *                 v
 *              parser
 *
 * Specification:
 *     grammar/spec/lexical.md
 *     grammar/spec/grammar-authority.md
 *
 * Documentation:
 *     grammar/lexer/keywords.md
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no Rust code.
 *     The Zamani implementation is required to use safe Rust only.
 *
 * ============================================================================
 *
 * FILE PURPOSE
 * ============================================================================
 *
 * This file owns ONLY source spellings that are RESERVED lexical keywords.
 *
 * A keyword is a lexical classification.
 *
 * A keyword does NOT itself define:
 *
 *     - AST semantics;
 *     - type semantics;
 *     - resource availability;
 *     - hardware selection;
 *     - target selection;
 *     - execution;
 *     - optimization;
 *     - scheduling;
 *     - routing;
 *     - quantum physical mapping;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime behavior.
 *
 * ============================================================================
 *
 * THIS FILE OWNS
 * ============================================================================
 *
 *     - reserved language words;
 *     - stable keyword token names;
 *     - lexical vocabulary for universal language constructs;
 *     - lexical vocabulary for resource intent;
 *     - lexical vocabulary for effects;
 *     - lexical vocabulary for contracts;
 *     - lexical vocabulary for reasoning;
 *     - lexical vocabulary for knowledge;
 *     - lexical vocabulary for learning/adaptation;
 *     - lexical vocabulary for uncertainty/provenance;
 *     - lexical vocabulary for execution policies;
 *     - lexical vocabulary for quantum language primitives;
 *     - lexical vocabulary for HDL/hardware intent where syntactically needed.
 *
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 *     - IDENTIFIER;
 *     - INTEGER;
 *     - FLOAT;
 *     - STRING;
 *     - CHAR;
 *     - boolean literals;
 *     - null/nil literals;
 *     - operators;
 *     - punctuation;
 *     - comments;
 *     - whitespace;
 *     - Unicode identifier classes;
 *     - interpolation;
 *     - quantum gate catalogs;
 *     - mathematical function catalogs;
 *     - device catalogs;
 *     - vendor catalogs;
 *     - backend catalogs;
 *     - hardware capacities;
 *     - tensor dimensions;
 *     - resource quantities;
 *     - physical topology;
 *     - implementation-specific limits.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This vocabulary contains NO language-level limits for:
 *
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     nodes
 *     devices
 *     memory
 *     storage
 *     registers
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     network size
 *     process count
 *     program size
 *     identifier length
 *     literal magnitude
 *
 * No keyword may encode a physical machine size.
 *
 * Resource quantities and capabilities are semantic information.
 *
 * Example:
 *
 *     requires capability("quantum.measurement");
 *     requires memory >= required_memory;
 *     requires qubits >= required_qubits;
 *
 * are source-level requirements.
 *
 * Whether those requirements can be satisfied is determined downstream.
 *
 * ============================================================================
 *
 * QUANTUM EXTENSIBILITY
 * ============================================================================
 *
 * Quantum operation names are intentionally NOT enumerated here.
 *
 * Do NOT add:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *     U
 *
 * as universal keyword tokens merely because they are known operations.
 *
 * They remain identifiers and are interpreted through the quantum semantic
 * layer and ultimately:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic quantum operation
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 * ============================================================================
 *
 * AI / REASONING EXTENSIBILITY
 * ============================================================================
 *
 * Universal reasoning vocabulary is permitted because reasoning is a language
 * capability rather than an application-specific API.
 *
 * The vocabulary therefore supports concepts such as:
 *
 *     infer
 *     deduce
 *     reason
 *     learn
 *     adapt
 *     assert
 *     retract
 *     query
 *     explain
 *     evidence
 *     provenance
 *     uncertainty
 *     confidence
 *     policy
 *
 * Domain-specific algorithms remain libraries, dialects, models, or semantic
 * capabilities rather than keyword catalogs.
 *
 * ============================================================================
 *
 * COMPATIBILITY POLICY
 * ============================================================================
 *
 * Existing token names are retained where their lexical meaning remains
 * compatible.
 *
 * A token rename or newly reserved spelling is a compatibility-affecting
 * change.
 *
 * Keyword additions MUST be checked against:
 *
 *     grammar/lexer/identifiers.g4
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/keywords.md
 *     grammar/spec/lexical.md
 *     grammar/compatibility/
 *     parser grammars
 *     src/lexer.rs
 *     lexer conformance tests
 *
 * ============================================================================
 *
 * RESERVED-WORD PRINCIPLE
 * ============================================================================
 *
 * A word should become reserved only when lexical reservation provides a
 * genuine language-level benefit.
 *
 * Library/API names should remain identifiers.
 *
 * Domain names should remain identifiers.
 *
 * Hardware names should remain identifiers.
 *
 * Vendor names should remain identifiers.
 *
 * Quantum operation names should remain identifiers.
 *
 * Mathematical functions should remain identifiers.
 *
 * ============================================================================
 */

lexer grammar ZamaniKeywords;


/* ============================================================================
 * CORE DECLARATION / BINDING
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
YIELD           : 'yield' ;


/* ============================================================================
 * MODULE / PACKAGE SYSTEM
 * ========================================================================== */

MODULE          : 'module' ;
IMPORT          : 'import' ;
EXPORT          : 'export' ;
USE             : 'use' ;
FROM            : 'from' ;
AS              : 'as' ;
PACKAGE         : 'package' ;


/* ============================================================================
 * TYPE / DATA DECLARATIONS
 * ========================================================================== */

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


/* ============================================================================
 * VISIBILITY
 * ========================================================================== */

PUBLIC          : 'public' ;
PUB             : 'pub' ;
PRIVATE         : 'private' ;
PROTECTED       : 'protected' ;
INTERNAL        : 'internal' ;


/* ============================================================================
 * OBJECT / IMPLEMENTATION MODIFIERS
 * ========================================================================== */

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


/* ============================================================================
 * GENERICS / TYPE CONSTRAINTS
 * ========================================================================== */

WHERE           : 'where' ;


/* ============================================================================
 * ASYNCHRONOUS / CONCURRENT COMPUTATION
 * ========================================================================== */

ASYNC           : 'async' ;
AWAIT           : 'await' ;
SPAWN           : 'spawn' ;
PARALLEL        : 'parallel' ;


/* ============================================================================
 * ERROR / EXCEPTION HANDLING
 * ========================================================================== */

/*
 * `unsafe` is intentionally not a language feature of the safe-only
 * implementation model. The source spelling is nevertheless reserved so
 * that it cannot silently become an ordinary identifier and accidentally
 * suggest unsupported semantics.
 *
 * Parser/semantic validation must reject unsupported unsafe constructs with
 * a clear diagnostic rather than providing an unsafe execution path.
 */
UNSAFE          : 'unsafe' ;

TRY             : 'try' ;
CATCH           : 'catch' ;
FINALLY         : 'finally' ;
THROW           : 'throw' ;
HANDLE          : 'handle' ;


/* ============================================================================
 * EFFECT SYSTEM
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


/* ============================================================================
 * CONTRACTS / CORRECTNESS
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
 * RESOURCE / CAPABILITY INTENT
 *
 * These words express source-level requirements and constraints.
 *
 * They do NOT describe fixed machine capacities.
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
RETRY           : 'retry' ;
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
 * REASONING
 *
 * These are generic language capabilities.
 *
 * They are deliberately not tied to a particular AI implementation.
 * ========================================================================== */

INFER           : 'infer' ;
DEDUCE          : 'deduce' ;
REASON          : 'reason' ;

PREMISE         : 'premise' ;
PREMISES        : 'premises' ;
CONCLUSION      : 'conclusion' ;

PROVE            : 'prove' ;
VERIFY           : 'verify' ;
VALIDATE        : 'validate' ;

ASSUMPTION      : 'assumption' ;
OBSERVATION     : 'observation' ;
INTERVENTION    : 'intervention' ;
COUNTERFACTUAL  : 'counterfactual' ;


/* ============================================================================
 * KNOWLEDGE
 * ========================================================================== */

KNOWLEDGE       : 'knowledge' ;
ASSERT_KNOWLEDGE
                : 'assert_knowledge' ;
RETRACT         : 'retract' ;
QUERY           : 'query' ;
FACT            : 'fact' ;
FACTS           : 'facts' ;

RELATION        : 'relation' ;
RELATIONS       : 'relations' ;


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

TRANSFER       : 'transfer' ;
REINFORCEMENT  : 'reinforcement' ;

UPDATE          : 'update' ;


/* ============================================================================
 * UNCERTAINTY / PROBABILITY
 *
 * These are semantic vocabulary words.
 *
 * They do not impose a particular numerical representation, precision,
 * floating-point width, tensor rank, or probability implementation.
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
 * EVIDENCE / EXPLANATION / PROVENANCE
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
 * PATTERN MATCHING / GUARDS
 * ========================================================================== */

PATTERN         : 'pattern' ;
PATTERNS        : 'patterns' ;
GUARD           : 'guard' ;
GUARDS          : 'guards' ;


/* ============================================================================
 * QUANTUM LANGUAGE PRIMITIVES
 *
 * Only language-level quantum concepts are reserved.
 *
 * Individual quantum operations remain identifiers.
 * ========================================================================== */

QUANTUM         : 'quantum' ;
CIRCUIT         : 'circuit' ;
QUBIT           : 'Qubit' ;

APPLY           : 'apply' ;
MEASURE        : 'measure' ;
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
 * HYBRID COMPUTATION
 * ========================================================================== */

HYBRID          : 'hybrid' ;
CLASSICAL       : 'classical' ;
QUANTUM_CLASSICAL
                : 'quantum_classical' ;

ACCELERATOR     : 'accelerator' ;
DEVICE          : 'device' ;
HOST            : 'host' ;

SUBMIT          : 'submit' ;
SYNC            : 'sync' ;


/* ============================================================================
 * HDL / HARDWARE INTENT
 *
 * These words describe language-level hardware intent.
 *
 * Physical implementation remains downstream.
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
 * DISTRIBUTED / NETWORKED COMPUTATION
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
 * AI / MODEL COMPOSITION
 *
 * Generic computational concepts only.
 *
 * Application-specific model names remain identifiers.
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
 * DATA / QUERY SEMANTICS
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


/* ============================================================================
 * METAPROGRAMMING / REFLECTION
 * ========================================================================== */

LANGUAGE        : 'language' ;
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
 * TYPE-LEVEL VOCABULARY
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
 * EXISTING ZAMANI TEMPORAL / LANGUAGE VOCABULARY
 * ========================================================================== */

MTS             : 'mts' ;
ZAMANI          : 'zamani' ;
SASA            : 'sasa' ;

REMEMBER        : 'remember' ;
RECALL          : 'recall' ;
WISDOM          : 'wisdom' ;


/* ============================================================================
 * TYPE-LEVEL / BUILT-IN TYPE NAMES
 *
 * These spellings are case-sensitive.
 *
 * Width, precision, representation and ABI are NOT determined here.
 * ========================================================================== */

RESULT          : 'Result' ;
NEVER           : 'Never' ;

PI_KEYWORD      : 'Pi' ;
SIGMA_KEYWORD   : 'Sigma' ;


/* ============================================================================
 * BUILT-IN TYPE SPELLINGS
 *
 * These remain reserved for compatibility with the existing language surface.
 *
 * They do NOT define physical widths.
 * ========================================================================== */

VOID            : 'void' ;
INT             : 'int' ;
FLOAT_TYPE      : 'float' ;
BOOL_TYPE       : 'bool' ;
STR_TYPE        : 'str' ;
STRING_TYPE     : 'string' ;
CHAR_TYPE       : 'char' ;


/* ============================================================================
 * CORE OPERATIONS WITH LANGUAGE-LEVEL SYNTAX
 *
 * General library APIs remain identifiers.
 * ========================================================================== */

PRINT           : 'print' ;
PRINTLN         : 'println' ;
PANIC           : 'panic' ;
LEN             : 'len' ;
SIZEOF          : 'sizeof' ;


/* ============================================================================
 * LOGICAL / RELATIONAL KEYWORDS
 * ========================================================================== */

IS              : 'is' ;
AND             : 'and' ;
OR              : 'or' ;
NOT             : 'not' ;


/* ============================================================================
 * SYSTEM / PORTABILITY VOCABULARY
 * ========================================================================== */

SYSTEM          : 'system' ;
NANO            : 'nano' ;
PERFORM         : 'perform' ;

OMNIVERSAL     : 'omniversal' ;

ALIGNMENT       : 'alignment' ;
CONTAINMENT     : 'containment' ;
TRUST            : 'trust' ;
SOVEREIGNTY     : 'sovereignty' ;
GOAL            : 'goal' ;
BIONANO         : 'bionano' ;
REALITY         : 'reality' ;


/* ============================================================================
 * LEGACY COMPATIBILITY VOCABULARY
 *
 * These names are retained only because they are already part of the current
 * lexical surface.
 *
 * They are NOT universal application semantics.
 *
 * New language features MUST NOT be added here merely because an application
 * needs them.
 *
 * Future compatibility work may deprecate these spellings after parser,
 * semantic, tooling and migration support is available.
 * ========================================================================== */

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
 * END OF KEYWORD VOCABULARY
 * ============================================================================
 *
 * NO rules below this point.
 *
 * The following are deliberately absent:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     TRUE
 *     FALSE
 *     NIL
 *     NULL
 *     operators
 *     punctuation
 *     comments
 *     whitespace
 *
 * Their canonical owners are other lexical grammar components.
 *
 * ============================================================================
 */