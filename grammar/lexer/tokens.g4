lexer grammar ZamaniTokens;

// =============================================================================
// Zamani Universal Programming Language
// grammar/lexer/tokens.g4
//
// CANONICAL LEXICAL VOCABULARY
// =============================================================================
//
// PURPOSE
// -------
// This lexer defines the stable lexical vocabulary consumed by the modular
// Zamani parser grammars.
//
// ARCHITECTURAL PIPELINE
//
//     source text
//         |
//         v
//     ZamaniTokens
//         |
//         v
//     modular Zamani parser grammars
//         |
//         v
//     frontend AST
//         |
//         v
//     structural / semantic validation
//         |
//         v
//     semantic model
//         |
//         +--------------------+
//         |                    |
//         v                    v
//     classical IR        quantum::ir
//                              |
//                              v
//                         QEC / ZQN /
//                         optimization /
//                         routing / scheduling
//                              |
//                              v
//                       HAL / target / runtime
//
// OWNERS
// ------
// This file owns:
//
//   * lexical token names;
//   * keyword recognition;
//   * identifiers;
//   * numeric literals;
//   * string and character literals;
//   * quantum source literals;
//   * operators;
//   * punctuation;
//   * comments;
//   * whitespace handling.
//
// THIS FILE DOES NOT OWN
// ----------------------
//
//   * AST construction;
//   * semantic interpretation;
//   * type checking;
//   * resource allocation;
//   * hardware discovery;
//   * hardware topology;
//   * target selection;
//   * qubit placement;
//   * QEC policy;
//   * ZQN semantics;
//   * optimization;
//   * routing;
//   * scheduling;
//   * runtime dispatch;
//   * deployment;
//   * machine-specific limits.
//
// POCO-REAF CONTRACT
// ------------------
//
// Lexical syntax must remain independent of physical execution resources.
//
// There is intentionally NO:
//
//   MAX_QUBITS
//   MAX_CORES
//   MAX_THREADS
//   MAX_GPUS
//   MAX_FPGAS
//   MAX_NODES
//   MAX_MEMORY
//   MAX_DEVICES
//   MAX_REGISTER_SIZE
//   MAX_TENSOR_RANK
//   MAX_PATH_DEPTH
//   MAX_RESOURCE_COUNT
//
// Resource availability is handled downstream.
//
// A parser/lexer implementation may impose configurable operational budgets
// for memory, diagnostics, source size, or execution time. Such budgets are
// implementation policy and must never become language semantics.
//
// SAFETY
// ------
// This is ANTLR grammar source and contains no Rust.
//
// Generated Rust/compiler/runtime integration MUST remain compatible with:
//
//   Rust 1.97
//   Rust 1.97.1
//
// The generated/runtime Rust implementation must not require `unsafe`.
//
// TOKEN NAMING
// ------------
//
// K_*          = reserved language keyword
// *_LITERAL    = lexical literal
// *_ASSIGN     = compound assignment
// *_SYMBOL     = symbolic language token
// fragments    = lexer implementation helpers
//
// Parser grammars consume these token names through:
//
//     options {
//         tokenVocab = ZamaniTokens;
//     }
//
// =============================================================================


// =============================================================================
// 1. PACKAGE / MODULE SYSTEM
// =============================================================================

K_PACKAGE       : 'package' ;
K_MODULE        : 'module' ;
K_IMPORT        : 'import' ;
K_EXPORT        : 'export' ;
K_USE           : 'use' ;
K_FROM          : 'from' ;
K_AS            : 'as' ;
K_GLOBAL        : 'global' ;
K_USING         : 'using' ;


// =============================================================================
// 2. VISIBILITY / DECLARATION MODIFIERS
// =============================================================================

K_PUB           : 'pub' ;
K_PUBLIC        : 'public' ;
K_PRIVATE       : 'private' ;
K_PROTECTED     : 'protected' ;
K_INTERNAL      : 'internal' ;

K_STATIC        : 'static' ;
K_CONST         : 'const' ;
K_LET           : 'let' ;
K_VAR           : 'var' ;
K_VAL           : 'val' ;
K_MUT           : 'mut' ;

K_EXTERN        : 'extern' ;
K_VOLATILE      : 'volatile' ;
K_INLINE        : 'inline' ;
K_FINAL         : 'final' ;
K_SEALED        : 'sealed' ;
K_PARTIAL       : 'partial' ;
K_OVERRIDE      : 'override' ;
K_VIRTUAL       : 'virtual' ;
K_ABSTRACT      : 'abstract' ;


// =============================================================================
// 3. FUNCTIONS / CONTROL FLOW
// =============================================================================

K_FN            : 'fn' ;
K_RETURN        : 'return' ;

K_IF            : 'if' ;
K_ELSE          : 'else' ;

K_WHILE         : 'while' ;
K_DO            : 'do' ;
K_FOR           : 'for' ;
K_IN            : 'in' ;
K_LOOP          : 'loop' ;

K_BREAK         : 'break' ;
K_CONTINUE      : 'continue' ;

K_MATCH         : 'match' ;
K_CASE          : 'case' ;
K_WHEN          : 'when' ;

K_WITH          : 'with' ;
K_YIELD         : 'yield' ;

K_SWITCH        : 'switch' ;
K_THEN          : 'then' ;

K_SPAWN         : 'spawn' ;
K_ASYNC         : 'async' ;
K_AWAIT         : 'await' ;


// =============================================================================
// 4. EXCEPTION / FAILURE CONTROL
// =============================================================================

K_TRY           : 'try' ;
K_CATCH         : 'catch' ;
K_FINALLY       : 'finally' ;
K_THROW         : 'throw' ;


// =============================================================================
// 5. SAFETY / EFFECTS
// =============================================================================

K_UNSAFE        : 'unsafe' ;
K_SAFE          : 'safe' ;

K_EFFECT        : 'effect' ;
K_PERFORM       : 'perform' ;
K_HANDLE        : 'handle' ;


// =============================================================================
// 6. TYPE SYSTEM / DECLARATIONS
// =============================================================================

K_TYPE          : 'type' ;
K_STRUCT        : 'struct' ;
K_ENUM          : 'enum' ;
K_TRAIT         : 'trait' ;
K_IMPL          : 'impl' ;
K_CLASS         : 'class' ;
K_INTERFACE     : 'interface' ;
K_RECORD        : 'record' ;

K_EXTENDS       : 'extends' ;
K_IMPLEMENTS    : 'implements' ;
K_PERMITS       : 'permits' ;

K_NEW           : 'new' ;
K_THIS          : 'this' ;
K_SELF          : 'self' ;
K_SUPER         : 'super' ;
K_SELF_TYPE     : 'Self' ;


// =============================================================================
// 7. GENERIC / TYPE-LEVEL LANGUAGE
// =============================================================================

K_WHERE         : 'where' ;
K_IS            : 'is' ;
K_ASCRIBE       : 'ascribe' ;

K_SOME          : 'Some' ;
K_OK            : 'Ok' ;
K_ERR           : 'Err' ;
K_NEVER         : 'never' ;


// =============================================================================
// 8. BOOLEAN / NULL VALUES
// =============================================================================

K_TRUE          : 'true' ;
K_FALSE         : 'false' ;
K_NIL           : 'nil' ;
K_NULL          : 'null' ;


// =============================================================================
// 9. BUILT-IN TYPES
// =============================================================================

K_VOID          : 'void' ;
K_BOOL          : 'bool' ;
K_INT           : 'int' ;
K_FLOAT         : 'float' ;
K_STR           : 'str' ;
K_STRING        : 'String' ;
K_CHAR          : 'char' ;

K_BYTES         : 'bytes' ;

K_I8            : 'i8' ;
K_I16           : 'i16' ;
K_I32           : 'i32' ;
K_I64           : 'i64' ;
K_I128          : 'i128' ;

K_U8            : 'u8' ;
K_U16           : 'u16' ;
K_U32           : 'u32' ;
K_U64           : 'u64' ;
K_U128          : 'u128' ;

K_F32           : 'f32' ;
K_F64           : 'f64' ;

K_USIZE         : 'usize' ;
K_ISIZE         : 'isize' ;


// =============================================================================
// 10. LOGICAL LANGUAGE WORDS
// =============================================================================

K_AND           : 'and' ;
K_OR            : 'or' ;
K_NOT           : 'not' ;


// =============================================================================
// 11. QUANTUM COMPUTING
//
// These are language concepts, NOT physical-device declarations.
//
// They do not encode:
//
//   * qubit count;
//   * physical qubit ID;
//   * topology;
//   * processor model;
//   * backend;
//   * device address;
//   * calibration;
//   * hardware capacity.
//
// Those belong to later semantic/resource/target layers.
// =============================================================================

K_QUANTUM       : 'quantum' ;
K_QUBIT         : 'qubit' ;
K_CIRCUIT       : 'circuit' ;

K_GATE          : 'gate' ;
K_MEASURE       : 'measure' ;
K_RESET         : 'reset' ;
K_OBSERVABLE    : 'observable' ;
K_STATE         : 'state' ;

K_ENTANGLE      : 'entangle' ;
K_NOISE         : 'noise' ;
K_FIDELITY      : 'fidelity' ;

K_LOGICAL       : 'logical' ;
K_PHYSICAL      : 'physical' ;
K_PARITY        : 'parity' ;
K_CODE          : 'code' ;

K_CONTROL       : 'control' ;
K_CONTROLLED    : 'controlled' ;
K_TARGET        : 'target' ;

K_MID_CIRCUIT   : 'mid_circuit' ;
K_DYNAMIC       : 'dynamic' ;


// =============================================================================
// 12. QUANTUM ERROR CORRECTION / RESILIENCE VOCABULARY
//
// These keywords describe source-level concepts only.
// QEC implementation remains outside the lexer.
// =============================================================================

K_QEC           : 'qec' ;
K_ERROR         : 'error' ;
K_CORRECT       : 'correct' ;
K_DETECT        : 'detect' ;
K_SYNDROME      : 'syndrome' ;
K_FAULT         : 'fault' ;
K_RECOVER       : 'recover' ;
K_RESILIENT     : 'resilient' ;


// =============================================================================
// 13. CLASSICAL / NUMERICAL / MATHEMATICAL VOCABULARY
// =============================================================================

K_VECTOR        : 'vector' ;
K_MATRIX        : 'matrix' ;
K_TENSOR        : 'tensor' ;

K_SYMBOLIC      : 'symbolic' ;
K_NUMERIC       : 'numeric' ;
K_STATISTICS    : 'statistics' ;
K_PROBABILITY   : 'probability' ;

K_DIFF          : 'diff' ;
K_INTEGRAL      : 'integral' ;
K_GRADIENT      : 'gradient' ;
K_HESSIAN       : 'hessian' ;
K_JACOBIAN      : 'jacobian' ;

K_SOLVE         : 'solve' ;
K_MINIMIZE      : 'minimize' ;
K_MAXIMIZE      : 'maximize' ;


// =============================================================================
// 14. HDL / HARDWARE DESCRIPTION
// =============================================================================

K_HDL           : 'hdl' ;
K_HARDWARE      : 'hardware' ;

K_MODULE_HW     : 'hwmodule' ;
K_PORT          : 'port' ;
K_SIGNAL        : 'signal' ;
K_WIRE          : 'wire' ;
K_REGISTER      : 'register' ;

K_CLOCK         : 'clock' ;
K_RESET_SIGNAL  : 'reset_signal' ;

K_COMBINATIONAL : 'combinational' ;
K_SEQUENTIAL    : 'sequential' ;
K_PROCESS       : 'process' ;
K_PIPELINE      : 'pipeline' ;
K_STATE_MACHINE : 'state_machine' ;

K_TIMING        : 'timing' ;
K_EDGE          : 'edge' ;


// =============================================================================
// 15. HARDWARE / TARGET / CAPABILITY LANGUAGE
//
// These are abstract language concepts. They must not encode finite hardware
// limits.
//
// Example:
//
//     requires capability quantum;
//     requires capability tensor;
//     requires resource memory;
//     prefers capability accelerator;
//
// does not select a concrete device here.
// =============================================================================

K_DEVICE        : 'device' ;
K_TARGET        : 'target' ;
K_CAPABILITY    : 'capability' ;
K_CAPABILITIES  : 'capabilities' ;

K_RESOURCE      : 'resource' ;
K_RESOURCES     : 'resources' ;

K_REQUIRE       : 'require' ;
K_REQUIRES      : 'requires' ;

K_CONSTRAINT    : 'constraint' ;
K_CONSTRAINTS   : 'constraints' ;

K_PREFERENCE    : 'preference' ;
K_PREFER        : 'prefer' ;
K_HINT          : 'hint' ;
K_HINTS         : 'hints' ;

K_PLACEMENT     : 'placement' ;
K_TOPOLOGY      : 'topology' ;

K_LATENCY       : 'latency' ;
K_THROUGHPUT    : 'throughput' ;
K_BANDWIDTH     : 'bandwidth' ;
K_ENERGY        : 'energy' ;
K_RELIABILITY   : 'reliability' ;
K_SCALABILITY   : 'scalability' ;
K_PORTABILITY   : 'portability' ;


// =============================================================================
// 16. COMPILATION
// =============================================================================

K_COMPILE       : 'compile' ;
K_COMPILER      : 'compiler' ;
K_FRONTEND      : 'frontend' ;
K_MIDDLEEND     : 'middleend' ;
K_BACKEND       : 'backend' ;

K_LOWER         : 'lower' ;
K_OPTIMIZE      : 'optimize' ;
K_OPTIMIZATION  : 'optimization' ;

K_SPECIALIZE    : 'specialize' ;
K_GENERATE      : 'generate' ;
K_GENERATIVE    : 'generative' ;

K_EMIT          : 'emit' ;
K_COMPILE_TIME  : 'compile_time' ;

K_RUNTIME       : 'runtime' ;
K_EXECUTION     : 'execution' ;


// =============================================================================
// 17. EXECUTION / DEPLOYMENT
// =============================================================================

K_EXECUTE       : 'execute' ;
K_RUN            : 'run' ;
K_DEPLOY        : 'deploy' ;

K_SCHEDULE      : 'schedule' ;
K_SCHEDULING    : 'scheduling' ;

K_DISPATCH      : 'dispatch' ;
K_SYNCHRONIZE   : 'synchronize' ;
K_CANCEL        : 'cancel' ;


// =============================================================================
// 18. CONCURRENCY / PARALLELISM
// =============================================================================

K_PARALLEL      : 'parallel' ;
K_CONCURRENT    : 'concurrent' ;
K_CONCURRENCY   : 'concurrency' ;

K_TASK          : 'task' ;
K_FUTURE        : 'future' ;
K_ACTOR         : 'actor' ;
K_CHANNEL       : 'channel' ;

K_SELECT        : 'select' ;
K_SYNC          : 'sync' ;


// =============================================================================
// 19. DISTRIBUTED COMPUTING
// =============================================================================

K_DISTRIBUTED   : 'distributed' ;
K_NODE          : 'node' ;
K_NODES         : 'nodes' ;
K_SERVICE       : 'service' ;
K_MESSAGE       : 'message' ;
K_MESSAGING     : 'messaging' ;

K_REMOTE        : 'remote' ;
K_REPLICATE     : 'replicate' ;
K_REPLICATION  : 'replication' ;

K_CONSISTENCY   : 'consistency' ;
K_FAULT_TOLERANCE : 'fault_tolerance' ;


// =============================================================================
// 20. AI / MACHINE LEARNING
// =============================================================================

K_AI            : 'ai' ;
K_ML            : 'ml' ;
K_MODEL         : 'model' ;
K_DATASET       : 'dataset' ;
K_TRAIN         : 'train' ;
K_TRAINING      : 'training' ;
K_INFERENCE     : 'inference' ;

K_AGENT         : 'agent' ;
K_LEARN         : 'learn' ;
K_INFER         : 'infer' ;

K_TENSOR_OP     : 'tensor_op' ;
K_AUTODIFF      : 'autodiff' ;


// =============================================================================
// 21. DATA
// =============================================================================

K_DATA          : 'data' ;
K_SCHEMA        : 'schema' ;
K_RECORDS       : 'records' ;
K_STREAM        : 'stream' ;
K_SERIALIZE     : 'serialize' ;
K_DESERIALIZE   : 'deserialize' ;

K_TRANSFORM     : 'transform' ;


// =============================================================================
// 22. NETWORKING
// =============================================================================

K_NETWORK       : 'network' ;
K_NETWORKING    : 'networking' ;
K_ENDPOINT      : 'endpoint' ;
K_PROTOCOL      : 'protocol' ;
K_CONNECTION    : 'connection' ;

K_REQUEST       : 'request' ;
K_RESPONSE      : 'response' ;


// =============================================================================
// 23. SECURITY / CRYPTOGRAPHY
// =============================================================================

K_SECURITY      : 'security' ;
K_PERMISSION    : 'permission' ;
K_PERMISSIONS   : 'permissions' ;

K_IDENTITY      : 'identity' ;
K_TRUST         : 'trust' ;
K_CRYPTO        : 'crypto' ;
K_CRYPTOGRAPHY  : 'cryptography' ;

K_PRIVACY       : 'privacy' ;
K_SECRET        : 'secret' ;
K_PUBLIC_KEY    : 'public_key' ;
K_SIGNATURE     : 'signature' ;


// =============================================================================
// 24. INTEROPERABILITY
// =============================================================================

K_EXTERN_LANG   : 'extern_lang' ;
K_FFI           : 'ffi' ;
K_ABI           : 'abi' ;
K_FOREIGN       : 'foreign' ;

K_C             : 'c' ;
K_CPP           : 'cpp' ;
K_PYTHON        : 'python' ;


// =============================================================================
// 25. METAPROGRAMMING / MACROS
// =============================================================================

K_MACRO         : 'macro' ;
K_META          : 'meta' ;
K_REFLECT       : 'reflect' ;
K_REFLECTION    : 'reflection' ;

K_EXPAND        : 'expand' ;
K_SPECIALIZATION : 'specialization' ;


// =============================================================================
// 26. PLUGINS / LANGUAGE EXTENSIONS
// =============================================================================

K_PLUGIN        : 'plugin' ;
K_DIALECT       : 'dialect' ;
K_EXPERIMENTAL  : 'experimental' ;
K_VENDOR        : 'vendor' ;


// =============================================================================
// 27. EXISTING ZAMANI / SANKOFA VOCABULARY
//
// Retained for compatibility with existing source and Rust lexer vocabulary.
// These names remain lexical constructs; their deeper semantics belong to
// downstream layers.
// =============================================================================

K_NANO          : 'nano' ;
K_REMEMBER      : 'remember' ;
K_RECALL        : 'recall' ;
K_WISDOM        : 'wisdom' ;
K_ZAMANI        : 'zamani' ;
K_SASA          : 'sasa' ;
K_ANCESTOR      : 'ancestor' ;

K_LINEAR        : 'linear' ;
K_AFFINE        : 'affine' ;

K_OMNIVERSAL    : 'omniversal' ;
K_ALIGNMENT     : 'alignment' ;
K_CONTAINMENT   : 'containment' ;
K_KNOWLEDGE     : 'knowledge' ;
K_SOVEREIGNTY   : 'sovereignty' ;
K_GOAL          : 'goal' ;
K_BIONANO       : 'bionano' ;
K_REALITY       : 'reality' ;
K_NLP           : 'nlp' ;
K_SYSTEM        : 'system' ;

K_ASI           : 'asi' ;
K_AESI          : 'aesi' ;
K_ASESI         : 'asesi' ;

K_ADMIN         : 'admin' ;
K_PAYMENT       : 'payment' ;
K_GATEWAY       : 'gateway' ;
K_GRAPHICS      : 'graphics' ;
K_VIDEO         : 'video' ;
K_ADJUST        : 'adjust' ;
K_VERSIONING    : 'versioning' ;
K_COPYRIGHT     : 'copyright' ;
K_NOTICE        : 'notice' ;
K_LEGAL         : 'legal' ;
K_ACTION        : 'action' ;
K_TAILOR        : 'tailor' ;
K_BUSINESS      : 'business' ;


// =============================================================================
// 28. COMMON BUILT-IN OPERATIONS
// =============================================================================

K_PRINT         : 'print' ;
K_PRINTLN       : 'println' ;
K_ASSERT        : 'assert' ;
K_PANIC         : 'panic' ;
K_LEN           : 'len' ;
K_SIZEOF        : 'sizeof' ;


// =============================================================================
// 29. TEMPORAL / MEMORY-LIKE EXISTING VOCABULARY
// =============================================================================

K_MTS           : 'mts' ;


// =============================================================================
// 30. MATHEMATICAL SYMBOL TOKENS
// =============================================================================
//
// Symbolic mathematical constructs remain explicit tokens rather than being
// silently folded into identifiers.

PI_SYMBOL
    : '\u03A0'
    ;

SIGMA_SYMBOL
    : '\u03A3'
    ;

LAMBDA_SYMBOL
    : '\u03BB'
    ;

THETA_SYMBOL
    : '\u03B8'
    ;

MU_SYMBOL
    : '\u03BC'
    ;

DELTA_SYMBOL
    : '\u0394'
    ;

OMEGA_SYMBOL
    : '\u03A9'
    ;

INFINITY_SYMBOL
    : '\u221E'
    ;

FOR_ALL_SYMBOL
    : '\u2200'
    ;

EXISTS_SYMBOL
    : '\u2203'
    ;

ELEMENT_OF_SYMBOL
    : '\u2208'
    ;

NOT_ELEMENT_OF_SYMBOL
    : '\u2209'
    ;

SUBSET_SYMBOL
    : '\u2282'
    ;

SUBSET_EQ_SYMBOL
    : '\u2286'
    ;

SUPERSET_SYMBOL
    : '\u2283'
    ;

SUPERSET_EQ_SYMBOL
    : '\u2287'
    ;

INTERSECTION_SYMBOL
    : '\u2229'
    ;

UNION_SYMBOL
    : '\u222A'
    ;

LOGICAL_NOT_SYMBOL
    : '\u00AC'
    ;

LOGICAL_AND_SYMBOL
    : '\u2227'
    ;

LOGICAL_OR_SYMBOL
    : '\u2228'
    ;

XOR_SYMBOL
    : '\u2295'
    ;

LESS_EQUAL_SYMBOL
    : '\u2264'
    ;

GREATER_EQUAL_SYMBOL
    : '\u2265'
    ;

NOT_EQUAL_SYMBOL
    : '\u2260'
    ;

APPROX_SYMBOL
    : '\u2248'
    ;

PLUS_MINUS_SYMBOL
    : '\u00B1'
    ;

MULTIPLY_SYMBOL
    : '\u00D7'
    ;

DIVIDE_SYMBOL
    : '\u00F7'
    ;

MINUS_SYMBOL
    : '\u2212'
    ;

DOT_PRODUCT_SYMBOL
    : '\u22C5'
    ;

PARTIAL_SYMBOL
    : '\u2202'
    ;

NABLA_SYMBOL
    : '\u2207'
    ;

SQRT_SYMBOL
    : '\u221A'
    ;

INTEGRAL_SYMBOL
    : '\u222B'
    ;

DOUBLE_INTEGRAL_SYMBOL
    : '\u222C'
    ;

TRIPLE_INTEGRAL_SYMBOL
    : '\u222D'
    ;

SUM_SYMBOL
    : '\u2211'
    ;

PRODUCT_SYMBOL
    : '\u220F'
    ;

EMPTY_SET_SYMBOL
    : '\u2205'
    ;

DEGREE_SYMBOL
    : '\u00B0'
    ;

PRIME_SYMBOL
    : '\u2032'
    ;

DOUBLE_PRIME_SYMBOL
    : '\u2033'
    ;

LEFT_ANGLE
    : '\u27E8'
    ;

RIGHT_ANGLE
    : '\u27E9'
    ;


// =============================================================================
// 31. COMPOUND OPERATORS
//
// Longest forms MUST occur before shorter prefixes.
//
// For example:
//
//     ...   before ..
//     ..=   before ..
//     ->    before -
//     >=    before >
//     +=    before +
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

SCOPE_ASSIGN
    : '::='
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

LEFT_SHIFT_ASSIGN
    : '<<='
    ;

RIGHT_SHIFT_ASSIGN
    : '>>='
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

NULL_COALESCE_ASSIGN
    : '??='
    ;

POWER
    : '**'
    ;

POWER_ASSIGN
    : '**='
    ;

ARROW_BI
    : '<->'
    ;

FAT_ARROW_BI
    : '<=>'
    ;


// =============================================================================
// 32. QUANTUM STATE LITERALS
//
// Examples:
//
//     |0⟩
//     |1⟩
//     |+⟩
//     |-⟩
//
// These represent source-level quantum-state syntax.
//
// IMPORTANT:
//
// This token does NOT represent:
//
//     * a physical qubit;
//     * a hardware qubit index;
//     * a fixed state-vector size;
//     * a device;
//     * a backend.
//
// No finite quantum-resource limit is encoded.
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
// 33. NUMERIC LITERALS
//
// Numeric lexical syntax does not impose a maximum numerical magnitude.
//
// Semantic/type layers determine representability.
//
// The lexer therefore does not define:
//
//     MAX_INTEGER
//     MAX_FLOAT
//     MAX_PRECISION
//     MAX_EXPONENT
//
// Operational limits remain implementation/resource policy.
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
      INT_SUFFIX?
    ;

BINARY_INTEGER
    : '0'
      [bB]
      BIN_DIGIT
      ('_'? BIN_DIGIT)*
      INT_SUFFIX?
    ;

OCTAL_INTEGER
    : '0'
      [oO]
      OCT_DIGIT
      ('_'? OCT_DIGIT)*
      INT_SUFFIX?
    ;

INTEGER_LITERAL
    : DIGIT_GROUP
      INT_SUFFIX?
    ;


// =============================================================================
// 34. CHARACTER LITERALS
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
// 35. STRING LITERALS
//
// A lexical string has no prescribed encoding, allocation strategy, ownership,
// interning strategy, or runtime representation at this layer.
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
// 36. IDENTIFIERS
//
// Compatibility baseline:
//
//     [A-Za-z_][A-Za-z0-9_]*
//
// This intentionally matches the existing Zamani lexical baseline rather than
// making downstream compiler behavior depend on a particular target's Unicode
// identifier implementation.
//
// Unicode mathematical/domain symbols with language-level meaning are handled
// by explicit symbolic tokens above.
//
// A future Unicode identifier expansion must be specified as a language-version
// change and tested independently.
// =============================================================================

IDENTIFIER
    : IDENTIFIER_START
      IDENTIFIER_CONTINUE*
    ;


// =============================================================================
// 37. PUNCTUATION
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

AT
    : '@'
    ;

HASH
    : '#'
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

LESS_THAN
    : '<'
    ;

GREATER_THAN
    : '>'
    ;

EQUAL
    : '='
    ;

BACKSLASH
    : '\\'
    ;




// =============================================================================
// 38. COMMENTS
//
// Comments are lexically discarded and therefore cannot influence semantic
// meaning.
//
// Documentation comments remain on a dedicated channel so frontend/tooling
// layers may optionally consume them without making documentation part of
// ordinary syntax.
// =============================================================================

DOC_COMMENT
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
// 39. WHITESPACE
//
// Whitespace is not semantic unless a future grammar rule explicitly introduces
// layout sensitivity. The current language is delimiter-driven.
// =============================================================================

WS
    : [ \t\r\n\u000B\u000C\u0085\u00A0]+
      -> channel(HIDDEN)
    ;


// =============================================================================
// 40. ERROR-RECOVERY TOKEN
//
// Keeping an explicit catch-all token provides deterministic lexical recovery
// and allows the Rust frontend to report a precise offending span.
//
// The token is NOT a valid language construct.
//
// Parser/diagnostic layers must treat ERROR_CHAR as a lexical error.
// =============================================================================

ERROR_CHAR
    : .
    ;


// =============================================================================
// 41. FRAGMENTS
//
// Fragments never produce tokens directly.
// =============================================================================

fragment IDENTIFIER_START
    : [A-Za-z_]
    ;

fragment IDENTIFIER_CONTINUE
    : [A-Za-z0-9_]
    ;

fragment DIGIT
    : [0-9]
    ;

fragment DIGIT_GROUP
    : DIGIT
      ('_'? DIGIT)*
    ;

fragment BIN_DIGIT
    : [01]
    ;

fragment OCT_DIGIT
    : [0-7]
    ;

fragment HEX_DIGIT
    : [0-9A-Fa-f]
    ;

fragment EXPONENT
    : [eE]
      [+-]?
      DIGIT_GROUP
    ;

fragment FLOAT_SUFFIX
    : [fF]
      (
          [0-9]+
      )
    ;

fragment INT_SUFFIX
    : [iu]
      (
          [0-9]+
        | [sS][iI][zZ][eE]
        | [sS][iI][zZ][eE]?
      )
    ;

fragment ESCAPE_SEQUENCE
    : '\\'
      (
          ['"\\nrtbfv0]
        | 'x' HEX_DIGIT HEX_DIGIT
        | 'u' '{' HEX_DIGIT+ '}'
        | 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
        | 'U' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
                  HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
      )
    ;


// =============================================================================
// END OF CANONICAL ZAMANI TOKEN VOCABULARY
// =============================================================================
//
// INTEGRATION CONTRACT
//
// 1. Parser grammars use:
//
//        options {
//            tokenVocab = ZamaniTokens;
//        }
//
// 2. Parser grammars MUST NOT redefine lexer rules.
//
// 3. New domain grammar modules consume this vocabulary rather than creating
//    domain-specific lexers.
//
// 4. Domain names that are not language keywords remain IDENTIFIERs.
//
// 5. Hardware/device/resource quantities are ordinary syntax/data and are not
//    represented as fixed lexer limits.
//
// 6. Quantum syntax does not directly reference quantum::ir.
//
// 7. quantum::ir remains the canonical quantum semantic boundary.
//
// 8. QEC, ZQN, HAL, routing, scheduling, calibration, optimization, resource
//    management, and runtime realization remain downstream.
//
// 9. Rust 1.97/1.97.1 integration is a consumer concern; this lexer introduces
//    no unsafe Rust.
//
// 10. Adding a new hardware architecture must not require adding a token unless
//     the architecture name itself becomes a deliberately reserved language
//     keyword.
//
// 11. Adding more qubits, CPUs, GPUs, FPGAs, nodes, memory, devices, tensors,
//     or other resources requires no lexer modification.
//
// 12. No finite source-level resource limit is encoded.
//
// =============================================================================