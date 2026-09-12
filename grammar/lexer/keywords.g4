/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/keywords.g4
 *
 * Role:
 *     Canonical keyword vocabulary for the Zamani lexical grammar.
 *
 * Grammar family:
 *     Zamani lexer
 *
 * Intended importer:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Language specification:
 *     grammar/spec/lexical.md
 *
 * Grammar authority:
 *     grammar/spec/grammar-authority.md
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no Rust code and requires no unsafe Rust.
 *     The Zamani compiler implementation MUST remain safe Rust only.
 *
 * ============================================================================
 *
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This file owns ONLY lexical recognition of Zamani's RESERVED keywords.
 *
 * It does NOT own:
 *
 *   - identifiers;
 *   - literals;
 *   - operators;
 *   - punctuation;
 *   - comments;
 *   - whitespace;
 *   - AST construction;
 *   - semantic analysis;
 *   - name resolution;
 *   - type checking;
 *   - effects;
 *   - resource analysis;
 *   - quantum semantics;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware discovery;
 *   - target selection;
 *   - runtime behavior;
 *   - backend selection.
 *
 * A keyword is therefore only a lexical classification.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Keywords MUST NOT encode:
 *
 *   - machine sizes;
 *   - qubit counts;
 *   - CPU counts;
 *   - GPU counts;
 *   - FPGA counts;
 *   - memory capacities;
 *   - topology;
 *   - physical addresses;
 *   - device identifiers;
 *   - native gate sets;
 *   - deployment sizes;
 *   - scheduling decisions.
 *
 * Zamani source describes portable computation and intent.
 *
 * Target-specific realization belongs downstream.
 *
 * ============================================================================
 *
 * KEYWORD POLICY
 * ============================================================================
 *
 * 1. Only genuinely syntactic reserved words belong here.
 *
 * 2. Library functions remain identifiers.
 *
 * 3. Quantum gates remain identifiers unless a spelling is itself a language
 *    keyword.
 *
 * 4. Mathematical functions remain identifiers unless the syntax requires
 *    lexical distinction.
 *
 * 5. Hardware/device names remain identifiers.
 *
 * 6. Vendor operations remain identifiers.
 *
 * 7. Backend names remain identifiers.
 *
 * 8. Resource values remain semantic data rather than keywords.
 *
 * 9. New vocabulary MUST NOT be added merely because a new subsystem exists.
 *
 * 10. Contextual concepts should remain identifiers whenever the parser can
 *     distinguish them structurally.
 *
 * ============================================================================
 *
 * IMPORT CONTRACT
 * ============================================================================
 *
 * ZamaniLexer.g4 should import this grammar:
 *
 *     lexer grammar ZamaniLexer;
 *
 *     import ZamaniKeywords;
 *
 * The assembled lexer remains the token source consumed by parser grammars
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * Existing parser grammars therefore continue to depend on ZamaniLexer rather
 * than directly depending on this vocabulary file.
 *
 * ============================================================================
 *
 * TOKEN-NAME STABILITY
 * ============================================================================
 *
 * The token names below are part of the lexer/parser integration contract.
 *
 * Changing a token name is a compatibility-affecting change even when its
 * spelling remains unchanged.
 *
 * When a keyword spelling changes:
 *
 *   old spelling -> compatibility/deprecation policy
 *   new spelling -> new lexical contract
 *
 * Do not silently reuse a token name for an unrelated spelling.
 *
 * ============================================================================
 */

lexer grammar ZamaniKeywords;


/* ============================================================================
 * CORE DECLARATION / BINDING
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
 * TYPE / DATA DECLARATIONS
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
 * VISIBILITY
 * ========================================================================== */

PUBLIC      : 'public' ;
PUB         : 'pub' ;
PRIVATE     : 'private' ;
PROTECTED   : 'protected' ;
INTERNAL    : 'internal' ;


/* ============================================================================
 * OBJECT / IMPLEMENTATION MODIFIERS
 * ========================================================================== */

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
 * GENERIC CONSTRAINTS
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
 * QUANTUM LANGUAGE PRIMITIVES
 *
 * IMPORTANT:
 *
 * This section deliberately contains language-level quantum syntax words,
 * not an exhaustive quantum operation vocabulary.
 *
 * Gate names such as:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     U
 *     RX
 *     RY
 *     RZ
 *
 * remain identifiers unless separately reserved by a future lexical
 * specification.
 *
 * This keeps the grammar independent of any particular hardware generation.
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
 * QUANTUM SEMANTIC DECLARATION WORDS
 *
 * These are retained only where they already form part of Zamani's language
 * surface. They do not perform semantic work in the lexer.
 * ========================================================================== */

ENTANGLE    : 'entangle' ;
NOISE       : 'noise' ;
FIDELITY    : 'fidelity' ;
SURFACE     : 'surface' ;
CODE        : 'code' ;
LOGICAL     : 'logical' ;
PARITY      : 'parity' ;


/* ============================================================================
 * NANO / AGENT COMPUTATION
 * ========================================================================== */

NANO        : 'nano' ;
AGENT       : 'agent' ;
PERFORM     : 'perform' ;
LEARN       : 'learn' ;
INFER       : 'infer' ;


/* ============================================================================
 * TEMPORAL / ZAMANI LANGUAGE VOCABULARY
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
 *
 * These spellings are case-sensitive and therefore remain distinct from
 * ordinary lower-case identifiers.
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
 * LOGICAL / TYPE-RELATION WORDS
 *
 * These are lexical words because the current grammar uses them as operators
 * or type/semantic relations.
 * ========================================================================== */

IS          : 'is' ;
AND         : 'and' ;
OR          : 'or' ;
NOT         : 'not' ;


/* ============================================================================
 * BUILT-IN TYPE NAMES
 *
 * These remain reserved in the current language surface.
 *
 * The lexical layer does NOT determine:
 *
 *     - width;
 *     - precision;
 *     - representation;
 *     - ABI;
 *     - machine register size;
 *     - target compatibility.
 *
 * Those are semantic/type-system responsibilities.
 * ========================================================================== */

VOID        : 'void' ;
INT         : 'int' ;
FLOAT_TYPE  : 'float' ;
BOOL_TYPE   : 'bool' ;
STR_TYPE    : 'str' ;
STRING_TYPE : 'string' ;
CHAR_TYPE   : 'char' ;


/* ============================================================================
 * CORE BUILT-IN OPERATIONS
 *
 * These are retained only where the existing language surface gives them
 * syntactic significance.
 *
 * General library functions must remain identifiers.
 * ========================================================================== */

PRINT       : 'print' ;
PRINTLN     : 'println' ;
ASSERT      : 'assert' ;
PANIC       : 'panic' ;
LEN         : 'len' ;
SIZEOF      : 'sizeof' ;


/* ============================================================================
 * UNIVERSAL / SYSTEM VOCABULARY
 *
 * These spellings already occur in the existing Zamani language surface.
 *
 * They are lexical names only. They MUST NOT cause hardware discovery,
 * backend selection, simulation, deployment, or runtime activity during
 * lexical analysis.
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
 * Retained for compatibility with the existing language surface.
 *
 * These are not implementations of the named systems.
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
 * BOOLEAN / NULL LITERALS
 *
 * These are keyword-like lexical tokens rather than ordinary identifiers.
 *
 * Their semantic relationship is defined downstream.
 * ========================================================================== */

TRUE        : 'true' ;
FALSE       : 'false' ;

NIL         : 'nil' ;
NULL        : 'null' ;


/* ============================================================================
 * END OF KEYWORD VOCABULARY
 * ============================================================================
 *
 * There are intentionally NO rules here for:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     operators
 *     punctuation
 *     comments
 *     whitespace
 *
 * Those belong to their respective lexical grammar components.
 *
 * In particular, do NOT add quantum gates, mathematical functions, device
 * names, backend names, accelerator names, topology names, or arbitrary API
 * names here merely because they exist elsewhere in the repository.
 *
 * ============================================================================
 */