/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/map-types.g4
 *
 * Grammar:
 *     MapTypes
 *
 * Status:
 *     Production-ready focused map-type grammar delegate.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL SYNTAX of map types.
 *
 * Canonical forms:
 *
 *     Map<K, V>
 *
 * and, where the language's generic/named-type syntax permits qualified names:
 *
 *     collections::Map<K, V>
 *     std::Map<K, V>
 *     domain::Map<K, V>
 *
 * The grammar intentionally treats Map as a generic type application rather
 * than making Map a special machine-dependent built-in type.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the dedicated parser-level map-type production;
 *     - the syntactic requirement for a key type;
 *     - the syntactic requirement for a value type;
 *     - the separator between key and value types;
 *     - the map-type integration boundary with the canonical type expression.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifier spelling;
 *     - generic type syntax in general;
 *     - generic declarations;
 *     - type inference;
 *     - key comparability;
 *     - key hashing;
 *     - ordering semantics;
 *     - collision semantics;
 *     - allocation;
 *     - capacity;
 *     - memory layout;
 *     - storage representation;
 *     - hash-table implementation;
 *     - tree implementation;
 *     - distributed-map implementation;
 *     - concurrent-map implementation;
 *     - GPU/FPGA accelerator implementation;
 *     - quantum storage;
 *     - hardware selection;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - runtime dispatch;
 *     - ABI layout;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * This is a focused parser grammar.
 *
 * It MUST be composed into the canonical type grammar:
 *
 *     grammar/types/types.g4
 *
 * rather than becoming an independent parser entry point.
 *
 * `typeExpression` is deliberately NOT defined here.
 *
 * The enclosing canonical type grammar supplies `typeExpression`.
 *
 * This permits map keys and values to use the complete Zamani type system
 * without duplicating that system.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Map syntax describes a PROGRAM-LEVEL ASSOCIATIVE DATA TYPE.
 *
 * It does not describe a particular implementation.
 *
 * Therefore:
 *
 *     Map<K, V>
 *
 * MUST NOT imply:
 *
 *     - a particular hash-table implementation;
 *     - a particular tree implementation;
 *     - a particular memory layout;
 *     - a particular CPU;
 *     - a particular GPU;
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a particular device;
 *     - a particular node;
 *     - a particular network;
 *     - a particular memory size;
 *     - a particular number of entries;
 *     - a particular number of buckets;
 *     - a particular hash width;
 *     - a particular pointer width;
 *     - a particular cache size.
 *
 * These decisions belong downstream.
 *
 * ============================================================================
 * ABSOLUTE SCALABILITY RULE
 * ============================================================================
 *
 * This grammar contains NO:
 *
 *     MAX_MAP_ENTRIES
 *     MAX_KEY_SIZE
 *     MAX_VALUE_SIZE
 *     MAX_MAP_DEPTH
 *     MAX_MAP_NESTING
 *     MAX_GENERIC_ARITY
 *     MAX_MEMORY
 *     MAX_BUCKETS
 *     MAX_HASH_WIDTH
 *
 * or equivalent fixed physical limits.
 *
 * A map can therefore participate in arbitrarily large semantic structures,
 * subject only to:
 *
 *     - source validity;
 *     - semantic validity;
 *     - compiler resource availability;
 *     - explicit program constraints;
 *     - target representability;
 *     - runtime resource availability.
 *
 * Those limitations MUST NOT be encoded into this grammar.
 *
 * ============================================================================
 * MAP AS A TYPE CONSTRUCTOR
 * ============================================================================
 *
 * Map is intentionally represented using the language's generic type mechanism:
 *
 *     Map<K, V>
 *
 * rather than introducing a dedicated keyword such as:
 *
 *     K_MAP
 *
 * unless the language specification later explicitly reserves Map as a
 * keyword.
 *
 * This is important for extensibility.
 *
 * It permits future map implementations and dialects to define semantic
 * constructors such as:
 *
 *     Map<K, V>
 *     OrderedMap<K, V>
 *     ConcurrentMap<K, V>
 *     DistributedMap<K, V>
 *     SparseMap<K, V>
 *     PersistentMap<K, V>
 *
 * without changing the core map grammar.
 *
 * ============================================================================
 * RECURSIVE COMPOSITION
 * ============================================================================
 *
 * Map keys and values use `typeExpression`.
 *
 * Therefore the grammar supports:
 *
 *     Map<int, str>
 *
 *     Map<str, int>
 *
 *     Map<K, V>
 *
 *     Map<int, Map<str, float>>
 *
 *     Map<MapKey, [Value; N]>
 *
 *     Map<(A, B), Result<T, E>>
 *
 *     Map<Qubit, ClassicalValue>
 *
 *     Map<LogicalQubit, Measurement>
 *
 *     Map<hardware::Resource, Capability>
 *
 *     Map<Node, Map<Key, Value>>
 *
 * No special cases are required for these domains.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum type may syntactically appear as a map key or value when the
 * semantic type system permits it.
 *
 * For example:
 *
 *     Map<Qubit, Measurement>
 *
 * is a SOURCE-LEVEL TYPE EXPRESSION.
 *
 * It does NOT mean:
 *
 *     - assign physical qubit IDs;
 *     - allocate a physical QPU;
 *     - select a topology;
 *     - allocate a fixed number of qubits;
 *     - bypass quantum::ir.
 *
 * Semantic analysis determines whether the particular quantum type is valid
 * as a key/value and how it is represented.
 *
 * If the map participates in quantum computation, its semantic representation
 * eventually flows through the canonical quantum compilation pipeline.
 *
 * This grammar NEVER imports or depends upon:
 *
 *     quantum::ir
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Maps are general-purpose data structures and can contain:
 *
 *     integers
 *     floating-point values
 *     strings
 *     records
 *     tuples
 *     arrays
 *     functions
 *     generic types
 *     user-defined types
 *     domain types
 *
 * The grammar does not impose a classical machine representation.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Map types may be used in source-level hardware/software co-design only where
 * the semantic HDL/type system permits them.
 *
 * This grammar does not imply synthesizability.
 *
 * For example:
 *
 *     Map<Address, Data>
 *
 * is syntactically a map type.
 *
 * Whether it represents:
 *
 *     - synthesizable hardware;
 *     - software storage;
 *     - distributed storage;
 *     - a compiler abstraction;
 *     - an unsupported hardware construct;
 *
 * is a semantic/compiler decision.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A map type does not inherently imply:
 *
 *     - local storage;
 *     - remote storage;
 *     - replication;
 *     - sharding;
 *     - consistency model;
 *     - node placement;
 *     - network transport.
 *
 * Those are separate semantic/resource/distributed concerns.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Maps can be used for metadata, sparse representations, schemas, feature
 * associations, model parameters, configuration structures, and other data
 * abstractions.
 *
 * This grammar does not define:
 *
 *     - tensor semantics;
 *     - dataset semantics;
 *     - model semantics;
 *     - accelerator placement.
 *
 * Those belong to their respective language/domain layers.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must lower through the repository's existing canonical source
 * type representation.
 *
 * Conceptually:
 *
 *     Map<K, V>
 *
 * becomes something equivalent to:
 *
 *     TypeExpr::Map {
 *         key: TypeExpr,
 *         value: TypeExpr
 *     }
 *
 * if the repository's canonical TypeExpr has a dedicated Map variant.
 *
 * If the canonical type system instead represents Map as a generic named
 * application, the AST builder should preserve:
 *
 *     Map
 *
 * as the canonical named type constructor with:
 *
 *     K
 *     V
 *
 * as type arguments.
 *
 * THIS GRAMMAR MUST NOT FORCE A NEW AST DESIGN.
 *
 * In particular, do not create:
 *
 *     MapTypeAst
 *     MapTypeNode
 *     MapTypeIR
 *     QuantumMapIR
 *     HardwareMapIR
 *
 * merely because this grammar has its own file.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - resolving Map;
 *     - resolving K;
 *     - resolving V;
 *     - validating key legality;
 *     - validating value legality;
 *     - determining equality/hash requirements;
 *     - determining ownership semantics;
 *     - determining mutability;
 *     - determining concurrency semantics;
 *     - determining storage strategy;
 *     - determining memory requirements;
 *     - determining whether a map is synthesizable;
 *     - determining whether a map is distributed;
 *     - determining whether a map can cross a quantum/classical boundary;
 *     - determining target representation.
 *
 * The parser performs none of these operations.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Map cardinality is intentionally NOT part of this grammar.
 *
 * Do not introduce:
 *
 *     Map<K, V, N>
 *
 * as an implicit machine-capacity mechanism.
 *
 * If a program semantically requires a capacity, that requirement should be
 * represented through the appropriate resource/constraint/type-level mechanism
 * elsewhere in the language.
 *
 * The absence of a grammar-level map capacity means:
 *
 *     Map<K, V>
 *
 * remains portable across environments with different capacities.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * This file must coexist with:
 *
 *     grammar/types/generic-types.g4
 *
 * without duplicating generic argument syntax.
 *
 * The preferred architecture is:
 *
 *     Map<K, V>
 *       |
 *       +--> named/generic type machinery
 *
 * rather than making Map a grammar-level special case.
 *
 * If `types.g4` recognizes Map through generic named types, this file may be
 * used as a semantic/classification delegate rather than a second generic
 * parser.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Allowed:
 *
 *     ZamaniTokens
 *          |
 *          v
 *     MapTypes
 *          |
 *          v
 *     Types
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic type system
 *          |
 *          v
 *     canonical IR
 *
 * Forbidden:
 *
 *     MapTypes -> AST
 *     MapTypes -> quantum::ir
 *     MapTypes -> QEC
 *     MapTypes -> ZQN
 *     MapTypes -> scheduling
 *     MapTypes -> routing
 *     MapTypes -> hardware HAL
 *     MapTypes -> runtime
 *
 * The grammar remains upstream of those layers.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar uses tokens supplied by:
 *
 *     grammar/lexer/tokens.g4
 *
 * The canonical lexer vocabulary is:
 *
 *     ZamaniTokens
 *
 * No lexer tokens are declared here.
 *
 * Map is deliberately recognized through:
 *
 *     IDENTIFIER
 *
 * rather than requiring a new keyword.
 *
 * This avoids consuming a globally reserved lexical namespace unnecessarily.
 *
 * ============================================================================
 * SYNTAX CONTRACT
 * ============================================================================
 *
 * Canonical map syntax:
 *
 *     Map<K, V>
 *
 * where K and V are arbitrary valid type expressions.
 *
 * The generic constructor name itself is parsed by the canonical named-type
 * grammar.
 *
 * Therefore this delegate owns the semantic parser-level classification:
 *
 *     mapType
 *
 * but does not duplicate:
 *
 *     typePath
 *     genericArguments
 *     typeExpression
 *
 * ============================================================================
 * PRODUCTION RULE
 * ============================================================================
 *
 * The actual rule below deliberately consumes the complete canonical generic
 * spelling:
 *
 *     Map < K , V >
 *
 * while leaving K and V to `typeExpression`.
 *
 * This gives the map grammar an explicit ownership boundary without creating
 * a second generic grammar.
 *
 * ============================================================================
 */

parser grammar MapTypes;

options {
    tokenVocab = ZamaniTokens;
}


/**
 * ============================================================================
 * Map type
 * ============================================================================
 *
 * Canonical:
 *
 *     Map<K, V>
 *
 * The constructor spelling `Map` is matched as an identifier rather than a
 * lexer keyword so that the map abstraction remains compatible with the
 * language's named/generic type system.
 *
 * Key and value types recursively use the enclosing canonical `typeExpression`.
 *
 * No entry-count or capacity parameter is encoded here.
 */
mapType
    : identifier
      LESS_THAN
      typeExpression
      COMMA
      typeExpression
      GREATER_THAN
    ;