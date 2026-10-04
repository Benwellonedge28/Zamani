/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/types/map.g4
* 
* Grammar:
* Map
* 
* Status:
* PRODUCTION-READY FOCUSED MAP-TYPE CLASSIFICATION/VALIDATION DELEGATE
* 
* Compiler baseline:
* Rust 1.97 / Rust 1.97.1
* Rust 2021
* safe Rust only
* no unsafe
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file defines the focused parser-level representation of the canonical
* Zamani associative map type:
* 
* Map<K, V>
* 
* The important architectural rule is:
* 
* Map<K, V>
* 
* is a normal generic type application.
* 
* It is NOT a separate source-level type hierarchy.
* 
* It therefore converges on the existing canonical frontend representation:
* 
* TypeExpr::Generic(
*     TypeExpr::Identifier("Map"),
*     [K, V]
* )
* 
* or its equivalent qualified-base representation when a qualified map
* constructor is used.
* 
* This file exists to give the type subsystem a focused map-specific grammar
* boundary for:
* 
* - parser tooling;
* - structural validation;
* - conformance testing;
* - map-type classification;
* - documentation;
* - future semantic map analysis.
* 
* The ordinary Zamani parser MUST continue to obtain complete type syntax from:
* 
* grammar/types/types.g4
* 
* and generic application syntax from:
* 
* grammar/types/generic.g4
* 
* This file MUST NOT become a competing type-expression grammar.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Canonical source pipeline:
* 
* Zamani source
*      |
*      v
* grammar/antlr/ZamaniLexer.g4
*      |
*      v
* grammar/antlr/ZamaniParser.g4
*      |
*      v
* grammar/types/types.g4
*      |
*      v
* typeExpression
*      |
*      v
* canonical frontend TypeExpr
*      |
*      v
* structural validation
*      |
*      v
* name/type resolution
*      |
*      v
* semantic type model
*      |
*      +-------------------+--------------------+
*      |                   |                    |
*      v                   v                    v
*  classical          quantum semantics     HDL/hardware
*   semantics                |              semantics
*                            v
*                       quantum::ir
*      |                   |                    |
*      +-------------------+--------------------+
*                          |
*                          v
*                   canonical IR / domain IR
*                          |
*                  optimization / lowering
*                          |
*                  routing / scheduling
*                          |
*                   resilience / QEC
*                          |
*                         ZQN
*                          |
*                         HAL
*                          |
*                   target realization
* 
* "map.g4" remains upstream of semantic realization.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - the focused `mapType` parser production;
* - the syntactic requirement for exactly two map type arguments;
* - the key/value positional relationship;
* - the map-type validation/classification boundary;
* - the integration contract between a map-shaped generic application and
*   the canonical `TypeExpr` model.
* 
* THIS FILE DOES NOT OWN:
* 
* - the lexer;
* - keywords;
* - identifiers;
* - typeExpression;
* - typePath;
* - generic application in general;
* - generic declarations;
* - generic parameter bounds;
* - type aliases;
* - type inference;
* - name resolution;
* - type unification;
* - ownership checking;
* - borrow checking;
* - linearity checking;
* - hashing semantics;
* - equality semantics;
* - ordering semantics;
* - collision handling;
* - storage layout;
* - allocation;
* - capacity;
* - memory management;
* - concurrency implementation;
* - distributed placement;
* - sharding;
* - replication;
* - networking;
* - GPU/FPGA/ASIC realization;
* - quantum allocation;
* - routing;
* - scheduling;
* - calibration;
* - QEC implementation;
* - ZQN;
* - HAL;
* - runtime implementation;
* - ABI layout;
* - classical IR;
* - quantum::ir.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/types/types.g4
* grammar/antlr/ZamaniLexer.g4
* 
* CONSUMES:
* 
* typeExpression
* typePath
* LESS_THAN
* GREATER_THAN
* COMMA
* 
* EXPORTS:
* 
* mapType
* mapKeyType
* mapValueType
* 
* AST_OWNER:
* 
* Existing canonical frontend TypeExpr.
* 
* SEMANTIC_OWNER:
* 
* Semantic type system / map semantic resolver.
* 
* IR_OWNER:
* 
* Canonical semantic IR and the appropriate downstream domain IR.
* 
* TEST_OWNER:
* 
* grammar/tests/types/map/
* 
* SPEC_OWNER:
* 
* grammar/spec/type-system.md
* grammar/specification/types.md
* 
* IMPORTANT:
* 
* This file imports "Types".
* 
* "grammar/types/types.g4" MUST NOT import "Map".
* 
* This one-way dependency is deliberate:
* 
* Map
*   |
*   v
* Types
* 
* not:
* 
* Types <-> Map
* 
* The canonical parser therefore remains free of grammar cycles.
* 
* ============================================================================
* TOKEN AUTHORITY
* ============================================================================
* 
* The sole lexical authority is:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* No lexer rules are defined here.
* 
* In particular, this file MUST NOT declare:
* 
* MAP
* MAP_TYPE
* MAP_KW
* LESS
* GREATER
* IDENTIFIER
* 
* or any other lexical token.
* 
* The spelling:
* 
* Map
* 
* is intentionally NOT required to be a dedicated lexer keyword.
* 
* This is consistent with the repository's generic type architecture.
* 
* Lowercase:
* 
* map
* 
* may have an independent lexical meaning as an operation/keyword. It MUST
* NOT be confused with the source type constructor:
* 
* Map
* 
* Lexer case sensitivity therefore remains authoritative.
* 
* ============================================================================
* MAP IS A GENERIC TYPE
* ============================================================================
* 
* The source language meaning is:
* 
* Map<K, V>
* 
* where:
* 
* K = key type
* V = value type
* 
* The grammar does not define what implementation strategy realizes the map.
* 
* Valid semantic realizations may include, depending on the declared semantic
* contract and target capabilities:
* 
* hash-based associative storage
* ordered associative storage
* tree-based storage
* persistent storage
* sparse storage
* concurrent storage
* distributed storage
* accelerator-backed storage
* other future representations
* 
* Those are semantic/backend decisions, not grammar decisions.
* 
* ============================================================================
* SOURCE-LEVEL REPRESENTATION
* ============================================================================
* 
* The canonical source representation is the existing generic TypeExpr:
* 
* TypeExpr::Generic(
*     base,
*     arguments
* )
* 
* For:
* 
* Map<K, V>
* 
* the structural AST is conceptually:
* 
* TypeExpr::Generic(
*     TypeExpr::Identifier(Map),
*     vec![
*         K,
*         V
*     ]
* )
* 
* The exact Rust construction remains owned by the existing AST builder.
* 
* THIS FILE MUST NOT REQUIRE:
* 
* TypeExpr::Map
* MapType
* MapTypeNode
* MapTypeAst
* 
* as new source-AST variants.
* 
* A grammar filename does not imply an AST variant.
* 
* ============================================================================
* QUALIFIED MAP CONSTRUCTORS
* ============================================================================
* 
* The canonical named-type system supports qualified type paths.
* 
* Therefore map-shaped applications may also occur as:
* 
* collections::Map<K, V>
* domain::Map<K, V>
* package::collections::Map<K, V>
* 
* Whether such a constructor is actually the standard associative map type is
* a semantic/name-resolution question.
* 
* The parser must preserve the complete type path.
* 
* It MUST NOT discard namespace information merely because the terminal
* component is named "Map".
* 
* ============================================================================
* GENERIC ARITY
* ============================================================================
* 
* A canonical Map type requires exactly two semantic type arguments:
* 
* Map<K, V>
* 
* Therefore:
* 
* Map<K>
* 
* is structurally incomplete.
* 
* Map<K, V, W>
* 
* is not the canonical two-parameter Map constructor.
* 
* These are NOT global generic-arity limits.
* 
* They are properties of the resolved "Map" type constructor.
* 
* The language MUST continue to permit arbitrary generic arity for unrelated
* constructors through "grammar/types/generic.g4".
* 
* This distinction is essential:
* 
* Map arity = semantic constructor contract
* 
* not:
* 
* global generic arity = fixed language limit
* 
* ============================================================================
* EMPTY ARGUMENTS
* ============================================================================
* 
* The following is invalid:
* 
* Map<>
* 
* The following is invalid:
* 
* Map<,>
* 
* The following is invalid:
* 
* Map<K,>
* 
* when interpreted as the complete focused "mapType" production.
* 
* A trailing comma is therefore not silently converted into an additional
* semantic type argument.
* 
* The canonical generic grammar may have its own trailing-comma policy.
* Whatever policy is selected there, the resulting AST must contain exactly
* the semantic arguments actually written.
* 
* This focused production intentionally requires the canonical two-argument
* Map form without manufacturing placeholder arguments.
* 
* ============================================================================
* RECURSIVE COMPOSITION
* ============================================================================
* 
* Both key and value positions consume the complete canonical "typeExpression".
* 
* Therefore the following forms are structurally representable:
* 
* Map<int, str>
* 
* Map<str, int>
* 
* Map<K, V>
* 
* Map<int, Map<str, float>>
* 
* Map<MapKey, Result<Value, Error>>
* 
* Map<(A, B), [Value; N]>
* 
* Map<Key, Option<Value>>
* 
* Map<Key, Vec<Result<Value, Error>>>
* 
* Map<Key, Tensor<Value>[N, M]>
* 
* Map<Key, QuantumState<State>>
* 
* Map<Key, LogicalQubit>
* 
* Map<Key, hardware::Resource>
* 
* The grammar introduces no artificial nesting limit.
* 
* ============================================================================
* POCO-REAF / SCALABILITY CONTRACT
* ============================================================================
* 
* Map syntax MUST remain independent of physical machine capacity.
* 
* This file MUST NOT encode:
* 
* MAX_MAP_ENTRIES
* MAX_KEY_SIZE
* MAX_VALUE_SIZE
* MAX_MAP_DEPTH
* MAX_MAP_NESTING
* MAX_BUCKETS
* MAX_HASH_WIDTH
* MAX_MEMORY
* MAX_DEVICES
* MAX_NODES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QUBITS
* 
* or equivalent constants.
* 
* The grammar therefore permits source structures whose eventual realization
* may be extremely small or extremely large.
* 
* For example:
* 
* Map<Key, Value>
* 
* has no language-level entry capacity.
* 
* Runtime capacity is determined by:
* 
* semantic requirements
* available resources
* explicit program constraints
* target capabilities
* implementation strategy
* runtime availability
* 
* If the target cannot satisfy the required realization, compilation or
* execution planning must report resource/target infeasibility.
* 
* It must not alter the source type's meaning.
* 
* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* "Map<K,V>" itself does not imply a resource requirement beyond whatever is
* established by the semantic type/runtime model.
* 
* This file does NOT define:
* 
* capacity;
* memory budget;
* storage device;
* node count;
* replication count;
* network bandwidth;
* accelerator assignment.
* 
* Resource requirements remain owned by:
* 
* grammar/resources/
* 
* and the downstream semantic resource model.
* 
* A program may independently express requirements such as:
* 
* requires memory >= required_memory;
* 
* requires capability("distributed.storage");
* 
* requires capability("associative.storage");
* 
* Those requirements are not embedded into Map syntax.
* 
* ============================================================================
* CAPABILITY CONTRACT
* ============================================================================
* 
* The grammar does not require any particular capability merely to parse:
* 
* Map<K,V>
* 
* Semantic analysis may determine that a particular use requires capabilities
* such as:
* 
* associative.storage
* persistent.storage
* distributed.storage
* concurrent.storage
* 
* Such capability identities are semantic/resource declarations and must not
* be converted into fixed grammar keywords.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Declaring or naming:
* 
* Map<K,V>
* 
* produces no runtime effect by itself.
* 
* In particular, a map type does not inherently mean:
* 
* IO
* network
* mutation
* synchronization
* randomness
* 
* Effects arise from operations performed on values of that type.
* 
* For example:
* 
* map.insert(...)
* 
* may have mutation/concurrency effects depending on the semantic type.
* 
* A pure read may have different effects.
* 
* The effect system remains the owner of these distinctions.
* 
* ============================================================================
* OWNERSHIP / LINEARITY CONTRACT
* ============================================================================
* 
* Map syntax does not define ownership policy.
* 
* Key/value ownership is determined by the types:
* 
* Map<linear K, linear V>
* 
* where such syntax is supported by the canonical type grammar.
* 
* The map grammar must preserve the nested type expressions and leave
* ownership/linearity validation to the semantic layer.
* 
* A map must not silently duplicate a value that the semantic type system
* identifies as linear.
* 
* ============================================================================
* KEY SEMANTICS
* ============================================================================
* 
* The parser only establishes:
* 
* K = key type
* 
* It does NOT decide whether K supports:
* 
* equality
* hashing
* ordering
* canonicalization
* serialization
* 
* These are semantic constraints.
* 
* A semantic Map constructor may impose requirements on K through its declared
* type bounds/constraints.
* 
* Therefore:
* 
* Map<SomeUnsupportedKey, Value>
* 
* may be syntactically valid while being rejected later by semantic analysis.
* 
* This is intentional.
* 
* ============================================================================
* VALUE SEMANTICS
* ============================================================================
* 
* The parser only establishes:
* 
* V = value type
* 
* It does not determine:
* 
* mutability;
* ownership;
* layout;
* serialization;
* storage;
* lifetime;
* device placement.
* 
* Those are downstream semantic properties.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Quantum types may syntactically occur in Map positions:
* 
* Map<LogicalQubit, Measurement>
* 
* Map<Qubit, ClassicalValue>
* 
* when the semantic type system permits them.
* 
* This does NOT:
* 
* - allocate physical qubits;
* - identify physical qubit IDs;
* - choose a QPU;
* - inspect hardware topology;
* - perform routing;
* - perform scheduling;
* - perform decomposition;
* - perform calibration;
* - perform QEC;
* - create a quantum IR.
* 
* If quantum computation is involved, downstream semantic processing continues
* through the canonical:
* 
* quantum::ir
* 
* boundary.
* 
* No map-specific quantum IR is permitted.
* 
* ============================================================================
* CLASSICAL INTEGRATION
* ============================================================================
* 
* Map types can be used with:
* 
* scalar types;
* records;
* tuples;
* arrays;
* slices;
* functions;
* generic types;
* symbolic types;
* numerical types;
* tensor types;
* user-defined types.
* 
* Classical representation is selected downstream.
* 
* The grammar must not assume:
* 
* pointer width;
* integer width;
* cache size;
* memory hierarchy;
* processor count.
* 
* ============================================================================
* AI / DATA INTEGRATION
* ============================================================================
* 
* Map is useful for generic data abstractions including:
* 
* metadata;
* sparse associations;
* feature associations;
* schemas;
* graph relationships;
* knowledge structures;
* model configuration.
* 
* The map grammar does not define AI algorithms or data formats.
* 
* It remains a generic type constructor.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* A Map type may appear in software/hardware co-design source where the
* surrounding semantic system permits it.
* 
* Example:
* 
* Map<Address, Data>
* 
* does not by itself mean:
* 
* hardware RAM;
* register file;
* cache;
* associative hardware;
* fixed-width bus;
* finite physical storage.
* 
* Synthesizability and physical realization are downstream questions.
* 
* ============================================================================
* DISTRIBUTED INTEGRATION
* ============================================================================
* 
* Map syntax does not imply:
* 
* local;
* remote;
* replicated;
* sharded;
* partitioned;
* strongly consistent;
* eventually consistent.
* 
* Distributed variants can be represented through separate semantic types,
* constructors, capabilities, policies, or dialects.
* 
* Examples:
* 
* DistributedMap<K,V>
* ReplicatedMap<K,V>
* PersistentMap<K,V>
* 
* do not require modifications to this grammar.
* 
* ============================================================================
* INTEROPERABILITY
* ============================================================================
* 
* Foreign data structures may be mapped to or from Zamani Map semantics by
* interoperability layers.
* 
* This file does not define:
* 
* C++ std::map
* Rust HashMap
* Java Map
* Python dict
* vendor-specific associative structures
* 
* Such representations belong to the appropriate interoperability/ABI layer.
* 
* ============================================================================
* POLICY CONTRACT
* ============================================================================
* 
* Policies may constrain the use or realization of a map.
* 
* Examples include:
* 
* storage policy;
* security policy;
* data-retention policy;
* distribution policy;
* concurrency policy;
* privacy policy.
* 
* This grammar does not implement policies.
* 
* Policy resolution occurs downstream.
* 
* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* The parser must preserve sufficient source structure for the existing source
* map/provenance infrastructure to associate:
* 
* Map
* key type
* value type
* 
* with their original source spans.
* 
* Provenance belongs to the parser/AST/source-map layers.
* 
* This file must not invent a second provenance model.
* 
* ============================================================================
* DIAGNOSTICS
* ============================================================================
* 
* This focused grammar is responsible only for structural syntax diagnostics.
* 
* Examples:
* 
* Map<>
* 
* Map<K>
* 
* Map<,V>
* 
* Map<K,>
* 
* Map<K,V,W>
* 
* may be rejected by the focused map production.
* 
* Semantic diagnostics are downstream:
* 
* unknown key type;
* unknown value type;
* invalid key constraint;
* unsupported key semantics;
* unsatisfied capability;
* insufficient resources;
* unsupported target realization.
* 
* These must not be reported as lexer errors merely because a semantic
* condition is unsatisfied.
* 
* ============================================================================
* ERROR CLASSIFICATION
* ============================================================================
* 
* The implementation should preserve the distinction:
* 
* LEXICAL ERROR
*     invalid tokenization
* 
* SYNTAX ERROR
*     invalid map type structure
* 
* TYPE ERROR
*     invalid key/value type
* 
* CONSTRAINT ERROR
*     map constructor constraints are unsatisfied
* 
* CAPABILITY ERROR
*     required capability unavailable
* 
* RESOURCE ERROR
*     required resources unavailable
* 
* TARGET ERROR
*     no supported realization for the requested semantic contract
* 
* A valid Map type must never be rejected merely because one target lacks
* sufficient resources.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar has no unordered semantic state.
* 
* Key and value order is structurally significant:
* 
* Map<K,V>
* 
* means:
* 
* key = K
* value = V
* 
* and MUST NOT be reordered.
* 
* Any downstream collection used for semantic analysis must preserve
* deterministic observable behavior.
* 
* ============================================================================
* GENERIC INTEGRATION
* ============================================================================
* 
* The general generic application owner remains:
* 
* grammar/types/generic.g4
* 
* It handles generic applications such as:
* 
* Vec<T>
* Result<T,E>
* Option<T>
* Container<A,B,C,...>
* 
* "map.g4" must therefore NOT redefine:
* 
* genericType
* typeArguments
* genericArgumentList
* 
* as a second generic grammar.
* 
* The focused map production only specializes the canonical type structure for
* the semantic Map constructor shape.
* 
* ============================================================================
* CANONICAL TYPE INTEGRATION
* ============================================================================
* 
* The complete type system remains owned by:
* 
* grammar/types/types.g4
* 
* Consequently key/value positions consume:
* 
* typeExpression
* 
* rather than a reduced map-specific type grammar.
* 
* This is necessary so that Map can contain the full range of source-level
* types supported by Zamani.
* 
* ============================================================================
* AST INTEGRATION
* ============================================================================
* 
* The parser/AST builder must lower:
* 
* Map<K,V>
* 
* into the existing generic representation.
* 
* Conceptual form:
* 
* TypeExpr::Generic(
*     TypeExpr::Identifier("Map"),
*     vec![K, V]
* )
* 
* Qualified form:
* 
* domain::Map<K,V>
* 
* becomes the equivalent generic application whose base is the complete
* qualified type path.
* 
* No new Map-specific AST node is required.
* 
* ============================================================================
* SEMANTIC INTEGRATION
* ============================================================================
* 
* Semantic analysis should resolve:
* 
* base constructor
* key type
* value type
* 
* and then validate the Map constructor's declared semantic contract.
* 
* Possible semantic requirements include:
* 
* key equality;
* key hashing;
* ordering;
* ownership;
* lifetime;
* serialization;
* persistence;
* concurrency;
* distribution.
* 
* These are semantic properties, not parser rules.
* 
* ============================================================================
* IR INTEGRATION
* ============================================================================
* 
* There is no Map-specific IR required by this grammar.
* 
* After semantic resolution, the resulting type participates in the ordinary
* canonical IR/type pipeline.
* 
* A backend may select a representation appropriate to:
* 
* CPU
* GPU
* FPGA
* ASIC
* accelerator
* distributed execution
* embedded execution
* future targets
* 
* without changing source-level Map meaning.
* 
* If Map participates in quantum computation, downstream lowering may interact
* with the canonical "quantum::ir", but this grammar never imports or creates
* that IR.
* 
* ============================================================================
* RESOURCE ADAPTATION
* ============================================================================
* 
* Different targets may realize the same semantic Map type differently.
* 
* The source program remains:
* 
* Map<K,V>
* 
* while downstream implementation may choose an appropriate representation
* according to:
* 
* requirements;
* capabilities;
* constraints;
* preferences;
* policies;
* available resources.
* 
* This is a direct application of POCO-REAF.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* The file contains no language-level machine limits.
* 
* In particular, it contains no:
* 
* MAX_MAP_ENTRIES
* MAX_MAP_CAPACITY
* MAX_KEY_WIDTH
* MAX_VALUE_WIDTH
* MAX_HASH_WIDTH
* MAX_MEMORY
* MAX_NODES
* MAX_DEVICES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QUBITS
* 
* It also does not:
* 
* - select a backend;
* - select a device;
* - select a memory hierarchy;
* - encode pointer width;
* - encode integer width;
* - encode hash width;
* - encode node count.
* 
* ============================================================================
* SECURITY AUDIT
* ============================================================================
* 
* This grammar:
* 
* - declares no executable code;
* - performs no I/O;
* - performs no allocation;
* - performs no native calls;
* - performs no FFI;
* - performs no reflection;
* - performs no dynamic execution;
* - does not bypass semantic validation;
* - does not bypass policy validation;
* - does not bypass capability validation.
* 
* The Rust implementation consuming this grammar MUST remain safe Rust and
* MUST NOT require "unsafe".
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Stable source meaning:
* 
* Map<K,V>
* 
* must remain stable across compiler and target versions unless the language
* specification explicitly introduces a breaking change.
* 
* Changing the backend representation of Map is not by itself a source
* compatibility change.
* 
* Adding a new map implementation must not require a new core grammar rule
* when it can be expressed as a distinct named generic type.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Positive structural tests:
* 
* Map<int,str>
* Map<str,int>
* Map<K,V>
* Map<int,Map<str,float>>
* Map<(A,B),Result<T,E>>
* Map<Key,Option<Value>>
* Map<Key,Tensor<Value>[N,M]>
* domain::Map<K,V>
* 
* Negative focused-map tests:
* 
* Map<>
* Map<K>
* Map<,V>
* Map<K,>
* Map<K,V,W>
* 
* Semantic-negative tests belong to the semantic type-test layer:
* 
* invalid key type;
* invalid value type;
* unsatisfied key constraint;
* unavailable capability;
* insufficient resources;
* unsupported realization.
* 
* Scalability tests must use symbolic and progressively larger structures
* without introducing language-level maximum constants.
* 
* ============================================================================
* BOUNDARY TESTS
* ============================================================================
* 
* Required boundaries include:
* 
* nested maps;
* maps containing generic types;
* maps containing dependent types;
* maps containing tensor shapes;
* maps containing quantum types;
* maps containing HDL-related types;
* maps containing distributed types;
* maps containing foreign/interoperability types;
* maps under contracts;
* maps under resource requirements;
* maps under capabilities;
* maps under policies;
* maps generated by macros;
* maps produced by metaprogramming.
* 
* ============================================================================
* CROSS-DOMAIN TEST
* ============================================================================
* 
* A production conformance test should demonstrate that:
* 
* Map<Key,Value>
* 
* survives the complete frontend pipeline:
* 
* source
*   |
*   v
* lexer
*   |
*   v
* parser
*   |
*   v
* TypeExpr::Generic
*   |
*   v
* structural validation
*   |
*   v
* semantic type resolution
*   |
*   v
* capability/resource analysis
*   |
*   v
* canonical IR
*   |
*   v
* target realization
* 
* without requiring a Map-specific AST or IR.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is DONE when:
* 
* [x] it declares no lexer rules;
* [x] it uses the canonical Zamani lexer vocabulary;
* [x] it does not duplicate typeExpression;
* [x] it does not duplicate generic application syntax;
* [x] it imports the canonical Types grammar one-way;
* [x] it defines a focused mapType production;
* [x] key and value positions consume canonical typeExpression;
* [x] Map is represented by existing generic TypeExpr semantics;
* [x] no Map-specific AST variant is required;
* [x] no Map-specific IR is required;
* [x] semantic key constraints remain downstream;
* [x] resource semantics remain downstream;
* [x] capability semantics remain downstream;
* [x] policy semantics remain downstream;
* [x] quantum realization remains downstream;
* [x] HDL realization remains downstream;
* [x] distributed realization remains downstream;
* [x] no physical resource limits are encoded;
* [x] no machine-width assumptions are encoded;
* [x] deterministic key/value ordering is preserved;
* [x] negative syntax cases are defined;
* [x] boundary cases are defined;
* [x] scalability cases are defined;
* [x] compatibility behavior is defined;
* [x] safe Rust remains the implementation requirement.
* 
* The repository integration additionally requires:
* 
* - `grammar/types/types.g4` remains the canonical type composition root;
* - `grammar/types/generic.g4` remains the generic application authority;
* - existing `grammar/types/map-types.g4` is not allowed to become a second
*   competing Map authority;
* - the parser/AST builder maps Map applications to the existing
*   `TypeExpr::Generic` representation;
* - conformance tests verify both ordinary generic parsing and focused
*   `mapType` classification.
* 
* ============================================================================
* CANONICAL GUARANTEE
* ============================================================================
* 
* The fundamental guarantee of this file is:
* 
* Map<K,V>
* 
* describes a source-level associative type contract.
* 
* It does not describe:
* 
* - how many entries exist;
* - where entries are stored;
* - how keys are hashed;
* - how values are represented;
* - which machine executes the program;
* - which memory device is selected;
* - which network node stores an entry;
* - which accelerator is used;
* - which quantum resource is allocated.
* 
* Consequently the same source type can participate in programs intended for
* radically different execution scales without changing its source meaning.
* 
* That separation is required for the Zamani POCO-REAF architecture.
* ============================================================================
  */

parser grammar Map;

options {
tokenVocab = ZamaniLexer;
}

import Types;

/*

* ============================================================================
* FOCUSED MAP TYPE
* ============================================================================
* 
* This is intentionally NOT the canonical general-purpose type-expression
* entry point.
* 
* The ordinary Zamani parser should parse:
* 
* Map<K,V>
* 
* through "Types.typeExpression" and "Generic.genericType".
* 
* "mapType" is the focused production for tooling, conformance, structural
* validation and map-specific parser consumers.
* 
* The terminal name is deliberately matched through the canonical type-path
* machinery rather than a dedicated "MAP" keyword.
* 
* This permits qualified constructors such as:
* 
* collections::Map<K,V>
* 
* to retain their complete source identity.
  */
  mapType
  : mapTypeConstructor
  LESS_THAN
  mapKeyType
  COMMA
  mapValueType
  GREATER_THAN
  ;

/*

* ============================================================================
* MAP CONSTRUCTOR
* ============================================================================
* 
* The constructor is represented using the canonical type-path structure.
* 
* The semantic layer determines whether the resolved constructor is actually
* the standard Map type.
* 
* No source-level namespace is discarded.
  */
  mapTypeConstructor
  : typePath
  ;

/*

* ============================================================================
* KEY TYPE
* ============================================================================
* 
* A map key is a complete Zamani type expression.
  */
  mapKeyType
  : typeExpression
  ;

/*

* ============================================================================
* VALUE TYPE
* ============================================================================
* 
* A map value is a complete Zamani type expression.
  */
  mapValueType
  : typeExpression
  ;