/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Grammar:
 *     ZamaniLexer
 *
 * Status:
 *     CANONICAL PRODUCTION LEXER



// grammar/ZamaniLexer.g4
//
// Zamani language lexer extension.
//
// Authority:
//   grammar/lexer/lexer.g4
//
// Build contract:
//   - grammar/lexer/lexer.g4 must declare: lexer grammar Lexer;
//   - compile with grammar/lexer on ANTLR's library path (-lib grammar/lexer).
//   - the imported Lexer grammar owns identifiers, literals, comments,
//     whitespace, operators, delimiters, and shared lexical fragments.
//   - this file owns only Zamani-reserved words not already defined by Lexer.
//   - do not duplicate a token rule from the imported grammar.
//   - parser grammars should consume this vocabulary through tokenVocab=ZamaniLexer.
//
// This file defines lexical identity only. Types, effects, resource requirements,
// capabilities, policies, contracts, and target feasibility are semantic concerns.
//
// Rust implementation constraint:
//   Rust 1.97 / 1.97.1; no unsafe Rust is required or permitted by this design.

lexer grammar ZamaniLexer;

import Lexer;

// -----------------------------------------------------------------------------
// Universal declarations and module structure
// -----------------------------------------------------------------------------

MODULE          : 'module';
IMPORT          : 'import';
EXPORT          : 'export';
FROM            : 'from';
AS              : 'as';
PUB             : 'pub';
PRIVATE         : 'private';
PROTECTED       : 'protected';
INTERNAL        : 'internal';

NAMESPACE       : 'namespace';
PACKAGE         : 'package';
USE             : 'use';
INCLUDE         : 'include';

CONST           : 'const';
LET             : 'let';
VAR             : 'var';
STATIC          : 'static';
MUT             : 'mut';

FN              : 'fn';
FUNCTION        : 'function';
RETURN          : 'return';
YIELD           : 'yield';
ASYNC           : 'async';
AWAIT           : 'await';
GENERATOR       : 'generator';

STRUCT          : 'struct';
RECORD          : 'record';
ENUM            : 'enum';
UNION           : 'union';
CLASS           : 'class';
INTERFACE       : 'interface';
TRAIT           : 'trait';
IMPL            : 'impl';
EXTENDS         : 'extends';
IMPLEMENTS      : 'implements';
TYPE            : 'type';
TYPEOF          : 'typeof';
WHERE           : 'where';
ASSOCIATED      : 'associated';
OPAQUE          : 'opaque';

NEW             : 'new';
SELF            : 'self';
SUPER           : 'super';
THIS            : 'this';
INIT            : 'init';
DROP            : 'drop';
DEFER           : 'defer';

// -----------------------------------------------------------------------------
// Control flow and pattern matching
// -----------------------------------------------------------------------------

IF              : 'if';
ELSE            : 'else';
THEN            : 'then';
MATCH           : 'match';
CASE            : 'case';
DEFAULT         : 'default';
WHEN            : 'when';
FOR             : 'for';
WHILE           : 'while';
LOOP            : 'loop';
IN              : 'in';
OF              : 'of';
BREAK           : 'break';
CONTINUE        : 'continue';
NEXT            : 'next';
SELECT          : 'select';
SWITCH          : 'switch';

TRY             : 'try';
CATCH           : 'catch';
FINALLY         : 'finally';
THROW           : 'throw';
RAISE           : 'raise';
ERROR           : 'error';
RESULT          : 'result';
OPTION          : 'option';
SOME            : 'some';
NONE            : 'none';
OK              : 'ok';
ERR             : 'err';

// -----------------------------------------------------------------------------
// Types, generic constraints, and value semantics
// -----------------------------------------------------------------------------

BOOL            : 'bool';
BOOLEAN         : 'boolean';
INTEGER         : 'integer';
INT             : 'int';
FLOAT           : 'float';
REAL            : 'real';
DECIMAL         : 'decimal';
RATIONAL        : 'rational';
COMPLEX         : 'complex';
STRING          : 'string';
CHAR            : 'char';
BYTE            : 'byte';
BYTES           : 'bytes';
UNIT            : 'unit';
NEVER           : 'never';
ANY             : 'any';
VOID            : 'void';

GENERIC         : 'generic';
EXTENDS_TYPE    : 'extends_type';
SUPER_TYPE      : 'super_type';
LINEAR          : 'linear';
AFFINE          : 'affine';
OWN             : 'own';
BORROW          : 'borrow';
SHARED          : 'shared';
MOVE            : 'move';
REF             : 'ref';
MAYBE           : 'maybe';
DISTINCT        : 'distinct';
SEALED          : 'sealed';
ABSTRACT        : 'abstract';
FINAL           : 'final';
OVERRIDE        : 'override';
VIRTUAL         : 'virtual';
STATIC_TYPE     : 'static_type';

// -----------------------------------------------------------------------------
// Contracts, validation, assumptions, and proofs
// -----------------------------------------------------------------------------

REQUIRES        : 'requires';
ENSURES         : 'ensures';
INVARIANT       : 'invariant';
PRECONDITION    : 'precondition';
POSTCONDITION   : 'postcondition';
ASSERT          : 'assert';
ASSUME          : 'assume';
GUARANTEE       : 'guarantee';
PROPERTY        : 'property';
PROVE           : 'prove';
PROOF           : 'proof';
VERIFY          : 'verify';
VALIDATE        : 'validate';
REFINE          : 'refine';
SATISFIES       : 'satisfies';
CONTRACT        : 'contract';
SPEC            : 'spec';

// -----------------------------------------------------------------------------
// Effects, capabilities, resources, and portable execution intent
// -----------------------------------------------------------------------------

EFFECT          : 'effect';
EFFECTS         : 'effects';
PURE            : 'pure';
CAPABILITY      : 'capability';
CAPABILITIES    : 'capabilities';
RESOURCE        : 'resource';
RESOURCES       : 'resources';
CONSTRAINT      : 'constraint';
CONSTRAINTS     : 'constraints';
BUDGET          : 'budget';
PREFERENCE      : 'prefer';
PREFER          : 'preference';
HINT            : 'hint';
NEGOTIATE       : 'negotiate';
ALLOW           : 'allow';
FORBID          : 'forbid';
DENY            : 'deny';
PERMIT          : 'permit';
RESTRICT        : 'restrict';
REQUIRE         : 'require';
PROVIDE         : 'provide';
AVAILABLE       : 'available';
OPTIONAL        : 'optional';
MANDATORY       : 'mandatory';
FALLBACK        : 'fallback';
PRIORITY        : 'priority';
PORTABLE        : 'portable';
SPECIALIZE      : 'specialize';
TARGET          : 'target';
BACKEND         : 'backend';
DEPLOY          : 'deploy';
EXECUTE         : 'execute';
EXECUTION       : 'execution';
SCHEDULE        : 'schedule';
PLACEMENT       : 'placement';
TOPOLOGY        : 'topology';
CAPACITY        : 'capacity';

// -----------------------------------------------------------------------------
// Classical, numerical, tensor, and data-oriented computation
// -----------------------------------------------------------------------------

ARRAY           : 'array';
SLICE           : 'slice';
VECTOR          : 'vector';
MATRIX          : 'matrix';
TENSOR          : 'tensor';
SHAPE           : 'shape';
RANK            : 'rank';
INDEX           : 'index';
SHARD           : 'shard';
PARTITION       : 'partition';
STREAM          : 'stream';
ITERATOR        : 'iterator';
COLLECTION      : 'collection';
MAP             : 'map';
SET             : 'set';
GRAPH           : 'graph';
NODE            : 'node';
EDGE            : 'edge';
TABLE           : 'table';
COLUMN          : 'column';
ROW             : 'row';
SCHEMA          : 'schema';
QUERY           : 'query';
TRANSACTION     : 'transaction';
COMMIT          : 'commit';
ROLLBACK        : 'rollback';

// -----------------------------------------------------------------------------
// Knowledge, reasoning, learning, uncertainty, and explanation
// -----------------------------------------------------------------------------

INFER           : 'infer';
DEDUCE          : 'deduce';
REASON          : 'reason';
INDUCE          : 'induce';
ABDUCE          : 'abduce';
LEARN           : 'learn';
ADAPT           : 'adapt';
RETRACT         : 'retract';
EXPLAIN         : 'explain';
EVIDENCE        : 'evidence';
PROVENANCE      : 'provenance';
DECISION        : 'decision';
CONFIDENCE      : 'confidence';
UNCERTAIN       : 'uncertain';
PROBABILITY     : 'probability';
DISTRIBUTION    : 'distribution';
BELIEF          : 'belief';
CAUSE           : 'cause';
CAUSAL          : 'causal';
INTERVENTION    : 'intervention';
COUNTERFACTUAL  : 'counterfactual';
OBSERVATION     : 'observation';
PREMISE         : 'premise';
CONCLUSION      : 'conclusion';
KNOWLEDGE       : 'knowledge';
FACT            : 'fact';
BELIEF_STATE    : 'belief_state';
OBJECTIVE       : 'objective';
FEEDBACK        : 'feedback';
MODEL           : 'model';
TRAIN           : 'train';
INFERRED        : 'inferred';

// -----------------------------------------------------------------------------
// Agents and concurrency. Actor lifecycle remains owned by concurrency specs.
// -----------------------------------------------------------------------------

ACTOR           : 'actor';
AGENT           : 'agent';
SPAWN           : 'spawn';
TASK            : 'task';
TASKS           : 'tasks';
CHANNEL         : 'channel';
MESSAGE         : 'message';
SEND            : 'send';
RECEIVE         : 'receive';
SELECTOR        : 'selector';
PARALLEL        : 'parallel';
CONCURRENT      : 'concurrent';
CONCURRENCY     : 'concurrency';
PAR              : 'par';
RACE            : 'race';
JOIN            : 'join';
CANCEL          : 'cancel';
CANCELLATION    : 'cancellation';
ATOMIC          : 'atomic';
SYNCHRONIZED    : 'synchronized';
LOCK            : 'lock';
UNLOCK          : 'unlock';

// -----------------------------------------------------------------------------
// Classical/quantum hybrid and quantum intent
// -----------------------------------------------------------------------------

QUANTUM         : 'quantum';
QUBIT           : 'qubit';
QUBITS          : 'qubits';
QREGISTER       : 'qregister';
QREG            : 'qreg';
QSTATE          : 'qstate';
CIRCUIT         : 'circuit';
MEASURE         : 'measure';
MEASUREMENT     : 'measurement';
RESET           : 'reset';
BARRIER         : 'barrier';
CHANNEL_Q       : 'quantum_channel';
NOISE           : 'noise';
ENTANGLE        : 'entangle';
CONTROL         : 'control';
HYBRID          : 'hybrid';
CLASSICAL       : 'classical';
COHERENCE       : 'coherence';
OBSERVABLE      : 'observable';
OPERATOR        : 'operator';
UNITARY         : 'unitary';
QEC             : 'qec';
ERROR_CORRECTION: 'error_correction';
DECOMPOSE       : 'decompose';
ROUTE           : 'route';
ROUTING         : 'routing';

// -----------------------------------------------------------------------------
// HDL and hardware description intent
// -----------------------------------------------------------------------------

HDL             : 'hdl';
HARDWARE        : 'hardware';
SIGNAL          : 'signal';
WIRE            : 'wire';
PORT            : 'port';
INPUT           : 'input';
OUTPUT          : 'output';
INOUT           : 'inout';
CLOCK           : 'clock';
RESET_SIGNAL    : 'reset_signal';
EDGE_TRIGGERED  : 'edge_triggered';
COMBINATIONAL   : 'combinational';
SEQUENTIAL      : 'sequential';
PROCESS         : 'process';
COMPONENT       : 'component';
MODULE_INSTANCE : 'instance';
SYNTHESIZE      : 'synthesize';
SYNTHESIS       : 'synthesis';
TIMING          : 'timing';
LATENCY         : 'latency';
THROUGHPUT      : 'throughput';
PIPELINE        : 'pipeline';
VERIFY_HARDWARE : 'verify_hardware';

// -----------------------------------------------------------------------------
// Security, policies, sandboxing, and controlled reflection
// -----------------------------------------------------------------------------

POLICY          : 'policy';
POLICIES        : 'policies';
SANDBOX         : 'sandbox';
TRUST           : 'trust';
AUTHORIZE       : 'authorize';
AUTHORIZATION   : 'authorization';
AUDIT           : 'audit';
ISOLATE         : 'isolate';
ISOLATION       : 'isolation';
REFLECT         : 'reflect';
REFLECTION      : 'reflection';
INTROSPECT      : 'introspect';
METADATA        : 'metadata';
ANNOTATION      : 'annotation';
ATTRIBUTE       : 'attribute';

// -----------------------------------------------------------------------------
// Simulation, resilience, and reproducibility
// -----------------------------------------------------------------------------

SIMULATE        : 'simulate';
SIMULATION      : 'simulation';
DETERMINISTIC   : 'deterministic';
NONDETERMINISTIC: 'nondeterministic';
REPRODUCIBLE    : 'reproducible';
SEED            : 'seed';
RETRY           : 'retry';
RECOVER         : 'recover';
RECOVERY        : 'recovery';
RESILIENT       : 'resilient';
RESILIENCE      : 'resilience';
DEGRADED        : 'degraded';
QUARANTINE      : 'quarantine';
ESCALATE        : 'escalate';

// -----------------------------------------------------------------------------
// Interoperability and foreign boundaries
// -----------------------------------------------------------------------------

FFI             : 'ffi';
ABI             : 'abi';
FOREIGN         : 'foreign';
EXTERN          : 'extern';
LINK            : 'link';
LINKAGE         : 'linkage';
CALLING         : 'calling';
CONVENTION      : 'convention';
NATIVE          : 'native';
INTEROP         : 'interop';
INTEROPERABILITY: 'interoperability';
ENCODE          : 'encode';
DECODE          : 'decode';
SERIALIZE       : 'serialize';
DESERIALIZE     : 'deserialize';

// -----------------------------------------------------------------------------
// Compile-time and controlled code generation
// -----------------------------------------------------------------------------

COMPTIME        : 'comptime';
CONSTEXPR       : 'constexpr';
MACRO           : 'macro';
QUOTE           : 'quote';
UNQUOTE         : 'unquote';
GENERATE        : 'generate';
CODEGEN         : 'codegen';
SYNTAX          : 'syntax';
DIALECT         : 'dialect';
EXTENSION       : 'extension';
VERSION         : 'version';
DEPRECATED      : 'deprecated';
EXPERIMENTAL    : 'experimental';
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file is the ONE and ONLY production lexer entry point for Zamani.
 *
 * It is intentionally a thin orchestration boundary.
 *
 * It does not duplicate lexical rules from grammar/lexer/.
 *
 * All lexical vocabulary is composed through:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose ANTLR grammar name is:
 *
 *     ZamaniTokens
 *
 * The resulting architecture is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniTokens
 *       |
 *       +--------------------------------------------------+
 *       |                                                  |
 *       v                                                  v
 *   lexical components                              parser input
 *       |                                                  |
 *       |                                                  v
 *       |                                        ZamaniParser
 *       |                                                  |
 *       |                    +-----------------------------+
 *       |                    |
 *       |                    +--> Core
 *       |                    +--> Types
 *       |                    +--> Expressions
 *       |                    +--> Declarations
 *       |                    +--> Statements
 *       |                    +--> Functions
 *       |                    +--> Modules
 *       |                    +--> Effects
 *       |                    +--> Memory
 *       |                    +--> Concurrency
 *       |                    +--> Classical
 *       |                    +--> Quantum
 *       |                    +--> Hybrid
 *       |                    +--> HDL
 *       |                    +--> Hardware
 *       |                    +--> Distributed
 *       |                    +--> AI
 *       |                    +--> Data
 *       |                    +--> Networking
 *       |                    +--> Security
 *       |                    +--> Resources
 *       |                    +--> Compile
 *       |                    +--> Execution
 *       |                    +--> Interoperability
 *       |                    +--> Dialects
 *       |                    +--> Macros
 *       |                    +--> Metaprogramming
 *       |
 *       v
 *   Domain-neutral AST
 *       |
 *       v
 *   Semantic analysis
 *       |
 *       +----------------------+-----------------------+
 *       |                      |                       |
 *       v                      v                       v
 *   Classical IR          quantum::ir           HDL/Hardware IR
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                    resilience / QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * ============================================================================
 *
 * IMPORTANT ARCHITECTURAL DISTINCTION
 * ============================================================================
 *
 * A lexer cannot orchestrate parser grammars.
 *
 * Therefore this file:
 *
 *     DOES orchestrate lexical grammar composition.
 *
 * It does NOT:
 *
 *     - import parser grammars;
 *     - parse declarations;
 *     - parse expressions;
 *     - parse quantum circuits;
 *     - parse HDL;
 *     - construct AST nodes;
 *     - perform semantic analysis;
 *     - select targets;
 *     - discover hardware;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - perform ZQN analysis;
 *     - perform calibration;
 *     - execute programs.
 *
 * Parser orchestration belongs exclusively to:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ============================================================================
 *
 * SINGLE-LEXER INVARIANT
 * ============================================================================
 *
 * There MUST be exactly one production Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No other file may be presented to downstream parser generation as the
 * canonical Zamani lexer.
 *
 * In particular, the following MUST NOT become competing production lexers:
 *
 *     grammar/lexer/*.g4
 *     grammar/antlr/*Lexer.g4
 *     src/lexer.rs
 *
 * `grammar/lexer/*.g4` contains modular lexical source.
 *
 * `src/lexer.rs` is the executable Rust frontend implementation/conformance
 * surface.
 *
 * `grammar/antlr/ZamaniLexer.g4` is the canonical ANTLR lexer boundary.
 *
 * ============================================================================
 *
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * All token ownership is delegated to:
 *
 *     grammar/lexer/tokens.g4
 *
 * `ZamaniTokens` is the lexical composition root.
 *
 * It composes the independently owned lexical families:
 *
 *     ZamaniLiterals
 *     ZamaniAnnotations
 *     ZamaniKeywords
 *     ZamaniOperators
 *     ZamaniPunctuation
 *     ZamaniIdentifiers
 *     ZamaniComments
 *     ZamaniWhitespace
 *     ZamaniLexerErrors
 *
 * Literal subfamilies are composed below ZamaniLiterals:
 *
 *     ZamaniNumericLiterals
 *     ZamaniStringLiterals
 *     ZamaniCharacterLiterals
 *     ZamaniBooleanLiterals
 *     ZamaniQuantumLiterals
 *     ZamaniHardwareLiterals
 *     ZamaniDurationLiterals
 *     ZamaniSizeLiterals
 *
 * This file MUST NOT import those leaf grammars directly.
 *
 * Doing so would create multiple lexical dependency paths and would make
 * token ownership harder to audit.
 *
 * ============================================================================
 *
 * TOKEN OWNERSHIP INVARIANT
 * ============================================================================
 *
 * Every token has exactly one lexical owner.
 *
 * Examples:
 *
 *     IDENTIFIER
 *         -> ZamaniIdentifiers
 *
 *     INTEGER
 *         -> ZamaniNumericLiterals
 *
 *     FLOAT
 *         -> ZamaniNumericLiterals
 *
 *     STRING
 *         -> ZamaniStringLiterals
 *
 *     CHAR
 *         -> ZamaniCharacterLiterals
 *
 *     TRUE / FALSE
 *         -> ZamaniBooleanLiterals
 *
 *     QUANTUM_LITERAL
 *         -> ZamaniQuantumLiterals
 *
 *     AT
 *         -> ZamaniAnnotations
 *
 * Operators:
 *         -> ZamaniOperators
 *
 * Punctuation:
 *         -> ZamaniPunctuation
 *
 * Comments:
 *         -> ZamaniComments
 *
 * Whitespace:
 *         -> ZamaniWhitespace
 *
 * Lexical-error sentinels:
 *         -> ZamaniLexerErrors
 *
 * No token may be duplicated here.
 *
 * ============================================================================
 *
 * QUANTUM EXTENSIBILITY
 * ============================================================================
 *
 * This file deliberately contains no finite quantum gate vocabulary.
 *
 * It MUST NOT contain rules such as:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * as a universal gate token set.
 *
 * Quantum operation names remain extensible source-level names.
 *
 * The intended pipeline is:
 *
 *     source operation
 *          |
 *          v
 *     generic AST Operation
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * This lexer must never become the gate-set authority.
 *
 * ============================================================================
 *
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * The lexer imposes NO universal machine-size limits.
 *
 * In particular, this file contains no limits for:
 *
 *     qubits
 *     logical qubits
 *     physical qubits
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
 *     processes
 *     tasks
 *     channels
 *     memory
 *     storage
 *     registers
 *     vector widths
 *     tensor rank
 *     tensor dimensions
 *     network links
 *     topology size
 *     timelines
 *     source size
 *     program size
 *
 * It MUST NOT introduce constants such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_PROGRAM_SIZE
 *
 * Finite lexical syntax is not the same thing as a finite machine limit.
 *
 * Practical implementation limits belong to:
 *
 *     source representation
 *     compiler resources
 *     runtime resources
 *     target capabilities
 *     resource policies
 *     operating environment
 *
 * They must not become universal lexical semantics.
 *
 * ============================================================================
 *
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Hardware names are lexical names.
 *
 * The lexer must not assign physical meaning to:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     node0
 *     device0
 *     accelerator0
 *
 * unless a separate, explicitly versioned lexical construct defines a genuine
 * lexical distinction.
 *
 * Hardware discovery belongs downstream.
 *
 * Physical mapping belongs downstream.
 *
 * Routing belongs downstream.
 *
 * Scheduling belongs downstream.
 *
 * ============================================================================
 *
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Lexical recognition must not determine resource availability.
 *
 * Source such as:
 *
 *     requires capability("quantum.measurement")
 *
 * is merely source syntax.
 *
 * Whether the target actually provides that capability is determined by
 * semantic analysis, resource management, compiler lowering, HAL, scheduling,
 * deployment, or runtime layers.
 *
 * Likewise:
 *
 *     requires qubits >= n
 *
 * must not become a lexer-level machine limit.
 *
 * ============================================================================
 *
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The canonical lexer must preserve sufficient token information for:
 *
 *     parser
 *     AST
 *     diagnostics
 *     source maps
 *     formatter
 *     LSP
 *     IDE tooling
 *     documentation
 *     refactoring
 *     macros
 *     metaprogramming
 *     provenance
 *     compatibility tooling
 *
 * Exact source spelling must remain available where required by the lexical
 * contract.
 *
 * No Unicode normalization, semantic rewriting, hardware discovery, or target
 * specialization is performed here.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     language version
 *     lexical grammar
 *
 * this lexer must produce the same lexical classification.
 *
 * Lexical behavior MUST NOT depend on:
 *
 *     hardware
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     network state
 *     filesystem state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     deployment topology
 *     runtime state
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * This grammar has no target-language actions.
 *
 * It performs no:
 *
 *     filesystem access
 *     network access
 *     command execution
 *     environment inspection
 *     hardware discovery
 *     credential access
 *     runtime execution
 *     backend invocation
 *
 * The lexer is therefore a pure source-processing boundary.
 *
 * ============================================================================
 *
 * ANTLR GENERATION CONTRACT
 * ============================================================================
 *
 * This is a lexer grammar.
 *
 * It must be generated independently of the parser.
 *
 * The parser uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The parser MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * directly.
 *
 * The build system must therefore make the generated:
 *
 *     ZamaniLexer.tokens
 *     ZamaniLexer.interp
 *     generated lexer sources
 *
 * available to parser generation as required by the selected ANTLR toolchain.
 *
 * ============================================================================
 *
 * PARSER INTEGRATION
 * ============================================================================
 *
 * Parser orchestration belongs to:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * `ZamaniParser.g4` is responsible for composing:
 *
 *     Core
 *     Types
 *     Expressions
 *     Declarations
 *     Statements
 *     Functions
 *     Modules
 *     Effects
 *     Memory
 *     Concurrency
 *     Classical
 *     Quantum
 *     Hybrid
 *     HDL
 *     Hardware
 *     Distributed
 *     AI
 *     Data
 *     Networking
 *     Security
 *     Resources
 *     Compile
 *     Execution
 *     Interoperability
 *     Dialects
 *     Macros
 *     Metaprogramming
 *
 * This lexer does not duplicate or import those parser grammars.
 *
 * ============================================================================
 *
 * FRONTEND INTEGRATION
 * ============================================================================
 *
 * The canonical frontend contract is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     canonical IR
 *
 * The lexer must not construct backend-specific AST nodes.
 *
 * In particular, it must not create:
 *
 *     PhysicalQubitNode
 *     QPUNode
 *     CPUNode
 *     GPUNode
 *     FPGAAllocationNode
 *     VendorGateNode
 *     RoutingNode
 *     SchedulingNode
 *
 * ============================================================================
 *
 * QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * The lexer has no direct dependency on quantum::ir.
 *
 * This is intentional.
 *
 * The relationship is:
 *
 *     quantum source
 *          |
 *          v
 *     lexer token
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     generic AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * No lexer token may imply a second quantum IR.
 *
 * ============================================================================
 *
 * CLASSICAL / QUANTUM / HDL / OTHER DOMAINS
 * ============================================================================
 *
 * The lexical vocabulary is shared by all domains.
 *
 * The same lexer must support source containing:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware/software co-design
 *     distributed computation
 *     parallel/HPC computation
 *     AI/ML
 *     data processing
 *     networking
 *     security
 *     scientific computation
 *     embedded computation
 *     accelerator computation
 *     future dialects
 *
 * Domain semantics are parser/semantic/IR concerns.
 *
 * The lexer remains domain-neutral.
 *
 * ============================================================================
 *
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * The generated/executing Zamani frontend is required to target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * This file itself contains no unsafe Rust because it contains no Rust code.
 *
 * ============================================================================
 *
 * ERROR CONTRACT
 * ============================================================================
 *
 * Malformed lexical constructs must not silently become valid tokens.
 *
 * The canonical error grammar:
 *
 *     ZamaniLexerErrors
 *
 * is composed through:
 *
 *     ZamaniTokens
 *
 * Error classification is therefore part of the canonical lexer rather than
 * being implemented by a competing second lexer.
 *
 * The lexer must preserve source spans for diagnostics.
 *
 * It must not:
 *
 *     panic
 *     execute code
 *     silently discard malformed source
 *     invent replacement syntax
 *
 * Ordinary malformed-source reporting is handled by the repository's
 * established diagnostic/error infrastructure.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing stable token names are preserved by this assembly boundary.
 *
 * The lexer therefore retains compatibility with parser grammars that consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Token renames, removals, or ownership changes require the normal language
 * compatibility/versioning process.
 *
 * `grammar/grammar.md` must report implementation status.
 *
 * `grammar/Zamani-Grammar.md` may describe future or historical syntax but
 * cannot silently redefine this lexical authority.
 *
 * ============================================================================
 *
 * GENERATED FILE POLICY
 * ============================================================================
 *
 * This file is source grammar.
 *
 * Generated lexer artifacts MUST NOT become competing hand-maintained
 * authorities.
 *
 * Generated output belongs to the repository's declared generated-artifact
 * policy.
 *
 * If generated files are committed, they must be reproducible from this
 * source and the canonical imported lexical components.
 *
 * ============================================================================
 *
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * `grammar/validation/` must verify at minimum:
 *
 *     - exactly one canonical lexer;
 *     - ZamaniLexer imports ZamaniTokens;
 *     - parser grammars consume ZamaniLexer;
 *     - no parser grammar consumes ZamaniTokens directly;
 *     - no duplicate lexical authority exists;
 *     - every token has one owner;
 *     - lexer imports resolve;
 *     - lexical grammar names match filenames;
 *     - no universal machine-size constants exist;
 *     - no fixed quantum gate vocabulary has been introduced;
 *     - no hardware topology is encoded lexically;
 *     - no semantic actions exist;
 *     - lexical tokenization remains deterministic.
 *
 * ============================================================================
 *
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is the single production lexer entry point.
 *
 * [x] It retains the existing filename.
 *
 * [x] It imports exactly one lexical composition root.
 *
 * [x] All lexical families are delegated to grammar/lexer/.
 *
 * [x] No lexical family is duplicated here.
 *
 * [x] No parser grammar is imported here.
 *
 * [x] Parser orchestration remains in ZamaniParser.g4.
 *
 * [x] Parser grammars consume ZamaniLexer.
 *
 * [x] Quantum syntax remains extensible.
 *
 * [x] No fixed quantum gate set is imposed.
 *
 * [x] No physical qubit mapping is imposed.
 *
 * [x] No machine/resource limits are imposed.
 *
 * [x] No hardware topology is imposed.
 *
 * [x] No backend is selected.
 *
 * [x] No QEC implementation is introduced.
 *
 * [x] No ZQN implementation is introduced.
 *
 * [x] No routing is introduced.
 *
 * [x] No scheduling is introduced.
 *
 * [x] No HAL implementation is introduced.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Rust 1.97 / 1.97.1 integration is documented.
 *
 * [x] Source preservation is documented.
 *
 * [x] Determinism is documented.
 *
 * [x] Compatibility is documented.
 *
 * [x] Validation responsibilities are documented.
 *
 * [x] Downstream integration is defined before implementation.
 *
 * ============================================================================
 *
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "How does Zamani source become the canonical token stream?"
 *
 * It does NOT answer:
 *
 *     "What does the program mean?"
 *
 *     "Which AST node represents it?"
 *
 *     "Which IR realizes it?"
 *
 *     "Which machine executes it?"
 *
 * Those responsibilities belong downstream.
 *
 * The complete architecture remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to actual program semantics and available resources, with no
 * artificial language-level hardware ceiling.
 *
 * ============================================================================
 */

lexer grammar ZamaniLexer;

/*
 * ============================================================================
 * CANONICAL LEXICAL ORCHESTRATION
 * ============================================================================
 *
 * ZamaniTokens is the ONLY lexical composition root.
 *
 * It imports:
 *
 *     ZamaniLiterals
 *     ZamaniAnnotations
 *     ZamaniKeywords
 *     ZamaniOperators
 *     ZamaniPunctuation
 *     ZamaniIdentifiers
 *     ZamaniComments
 *     ZamaniWhitespace
 *     ZamaniLexerErrors
 *
 * and therefore indirectly composes the complete grammar/lexer/ tree.
 *
 * DO NOT add individual lexical imports here.
 *
 * In particular, do NOT add:
 *
 *     ZamaniKeywords
 *     ZamaniOperators
 *     ZamaniIdentifiers
 *     ZamaniNumericLiterals
 *     ZamaniQuantumLiterals
 *     ...
 *
 * separately.
 *
 * Doing so would create multiple lexical dependency paths and can introduce
 * duplicate token definitions and unstable token ownership.
 *
 * ============================================================================
 */

import ZamaniTokens;