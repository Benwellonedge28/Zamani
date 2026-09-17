/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/maps.g4
 *
 * Status:
 *     Canonical production-ready map-expression grammar component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar delegate.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of map expressions.
 *
 * Normative forms:
 *
 *     {}
 *     { key: value }
 *     { key1: value1, key2: value2 }
 *     { expression: expression, expression: expression }
 *
 * The canonical language specification defines:
 *
 *     MapExpression ::=
 *         "{"
 *         [ MapEntry { "," MapEntry } ]
 *         "}"
 *
 *     MapEntry ::=
 *         Expression ":" Expression
 *
 * This file is therefore deliberately generic.
 *
 * A map key is a canonical Zamani expression.
 * A map value is a canonical Zamani expression.
 *
 * Consequently map expressions can contain:
 *
 *     classical values
 *     quantum values
 *     hybrid values
 *     HDL expressions
 *     hardware/resource expressions
 *     distributed values
 *     AI/ML values
 *     data expressions
 *     networking expressions
 *     security expressions
 *     compile-time expressions
 *     user-defined expressions
 *     future dialect expressions
 *
 * Semantic analysis determines whether a particular key/value pair is legal.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * This file defines SOURCE SYNTAX ONLY.
 *
 * It does NOT define:
 *
 *     - map key equality;
 *     - hashing;
 *     - ordering;
 *     - uniqueness;
 *     - mutability;
 *     - ownership;
 *     - borrowing;
 *     - allocation;
 *     - storage representation;
 *     - memory layout;
 *     - hashing algorithms;
 *     - tree representation;
 *     - lookup complexity;
 *     - serialization;
 *     - distributed placement;
 *     - replication;
 *     - persistence;
 *     - GPU placement;
 *     - FPGA placement;
 *     - QPU placement;
 *     - physical addresses;
 *     - hardware topology;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - backend lowering;
 *     - runtime representation.
 *
 * Canonical pipeline:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical expression parser
 *          |
 *          v
 *     mapExpression
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic/type/effect/resource analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *     classical IR          quantum::ir          HDL/hardware IR
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                         optimization / lowering
 *                                 |
 *                         routing / scheduling
 *                                 |
 *                         resilience / QEC / ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                          target realization
 *
 * IMPORTANT:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This grammar must never introduce:
 *
 *     MapQuantumIR
 *     QuantumMapAST
 *     HardwareMapIR
 *     QECMapIR
 *     TensorMapIR
 *
 * or equivalent domain-specific duplicate representations.
 *
 * ============================================================================
 * SINGLE RESPONSIBILITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - map-expression delimiters;
 *     - map-entry sequencing;
 *     - key/value separator syntax;
 *     - comma-separated map entries;
 *     - empty-map syntax;
 *     - single-entry maps;
 *     - multi-entry maps;
 *     - arbitrary expression keys;
 *     - arbitrary expression values;
 *     - the public `mapExpression` rule;
 *     - the public `mapEntry` rule;
 *     - the public `mapEntryList` rule.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the canonical `expression` hierarchy;
 *     - primary expressions;
 *     - postfix expressions;
 *     - blocks;
 *     - arrays;
 *     - tuples;
 *     - indexing;
 *     - map types;
 *     - map patterns;
 *     - declarations;
 *     - assignment;
 *     - type checking;
 *     - semantic map implementation;
 *     - hashing;
 *     - key uniqueness;
 *     - ordering;
 *     - allocation;
 *     - resource management;
 *     - hardware selection;
 *     - quantum mapping;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical expression composition layer owns:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     rangeExpression
 *     ...
 *     postfixExpression
 *     primaryExpression
 *
 * This file provides:
 *
 *     mapExpression
 *
 * Conceptually:
 *
 *     primaryExpression
 *         : ...
 *         | mapExpression
 *         | ...
 *         ;
 *
 * This file MUST NOT define:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     postfixExpression
 *     primaryExpression
 *
 * Defining those here would create competing expression hierarchies.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The intended dependency direction is:
 *
 *     canonical expression composition
 *                 |
 *                 v
 *          mapExpression
 *                 |
 *                 +----> expression
 *                 |
 *                 +----> expression
 *
 * Therefore this delegate does not import the canonical expression grammar
 * back into itself.
 *
 * The composition root is responsible for assembling the expression delegates.
 *
 * Do NOT create:
 *
 *     Expression -> Maps -> Expression
 *
 * as an explicit grammar-import cycle.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * The canonical lexical authority supplies:
 *
 *     LBRACE
 *     RBRACE
 *     COLON
 *     COMMA
 *
 * Expression syntax is supplied by the canonical expression grammar.
 *
 * This file therefore MUST NOT redefine:
 *
 *     LBRACE
 *     RBRACE
 *     COLON
 *     COMMA
 *
 * and MUST NOT introduce aliases such as:
 *
 *     MAP_LBRACE
 *     MAP_RBRACE
 *     MAP_COLON
 *     MAP_COMMA
 *
 * ============================================================================
 * NORMATIVE SYNTAX
 * ============================================================================
 *
 * The normative syntax is:
 *
 *     MapExpression ::=
 *         "{"
 *         [ MapEntry { "," MapEntry } ]
 *         "}"
 *
 *     MapEntry ::=
 *         Expression ":" Expression
 *
 * Therefore:
 *
 *     {}
 *
 * is syntactically valid.
 *
 *     {a: b}
 *
 * is syntactically valid.
 *
 *     {a: b, c: d}
 *
 * is syntactically valid.
 *
 *     {f(x): g(y)}
 *
 * is syntactically valid.
 *
 *     {[a, b]: value}
 *
 * is syntactically representable.
 *
 * Whether a particular key is semantically usable as a map key is NOT decided
 * by this grammar.
 *
 * ============================================================================
 * TRAILING COMMAS
 * ============================================================================
 *
 * The normative MapExpression production does NOT contain an optional trailing
 * comma.
 *
 * Therefore:
 *
 *     {a: b}
 *
 * is valid.
 *
 *     {a: b, c: d}
 *
 * is valid.
 *
 *     {a: b,}
 *
 * is NOT valid under this syntax contract.
 *
 * This is intentional.
 *
 * The map grammar must not silently inherit trailing-comma behavior from:
 *
 *     tuple expressions;
 *     function argument lists;
 *     generic argument lists;
 *     attribute argument lists.
 *
 * If trailing commas are introduced into map literals in a future language
 * version, that change must occur through the canonical specification and
 * compatibility/versioning process.
 *
 * ============================================================================
 * EMPTY MAP
 * ============================================================================
 *
 * The canonical syntax permits:
 *
 *     {}
 *
 * as an empty map.
 *
 * This creates an unavoidable syntactic interaction with block expressions,
 * because the language also permits block syntax using `{` and `}`.
 *
 * This ambiguity is NOT solved by creating a second map grammar.
 *
 * The canonical expression-composition layer MUST establish one deterministic
 * interpretation for `{}`.
 *
 * The integration requirement is:
 *
 *     - map-expression syntax is canonical here;
 *     - block-expression syntax remains owned by the block-expression layer;
 *     - the expression composition layer must resolve their shared `{}` form;
 *     - no semantic or runtime heuristic may decide how `{}` parses.
 *
 * The preferred composition policy is to establish a single syntactic
 * disambiguation rule in the canonical expression grammar and apply it
 * consistently everywhere.
 *
 * This file must not contain parser actions, target-language predicates, or
 * runtime callbacks to inspect whether `{}` "looks like" a map.
 *
 * Parsing must remain deterministic from the token stream and grammar version.
 *
 * ============================================================================
 * MAP ENTRY
 * ============================================================================
 *
 * A map entry has exactly two expressions:
 *
 *     key : value
 *
 * Both sides use the canonical `expression` rule.
 *
 * This deliberately permits:
 *
 *     {x: y}
 *     {x + 1: y * 2}
 *     {f(x): g(y)}
 *     {array[i]: value}
 *     {object.field: value}
 *     {condition ? a : b: value}
 *     {key: condition ? a : b}
 *
 * The expression grammar remains responsible for precedence.
 *
 * This file does not reproduce:
 *
 *     additiveExpression
 *     multiplicativeExpression
 *     conditionalExpression
 *     logicalOrExpression
 *     postfixExpression
 *     primaryExpression
 *
 * ============================================================================
 * CONDITIONAL KEY EXPRESSIONS
 * ============================================================================
 *
 * The canonical expression hierarchy contains:
 *
 *     condition ? whenTrue : whenFalse
 *
 * Map entries also contain:
 *
 *     key : value
 *
 * These two uses of COLON must remain unambiguous through the canonical
 * expression grammar.
 *
 * For example:
 *
 *     {condition ? left : right: value}
 *
 * means:
 *
 *     key   = condition ? left : right
 *     value = value
 *
 * when parsed according to the canonical expression precedence.
 *
 * This file must not implement a second conditional-expression grammar merely
 * to handle map keys.
 *
 * If the expression composition layer changes conditional-expression
 * precedence, the map grammar remains unchanged because it consumes the
 * canonical `expression` rule.
 *
 * ============================================================================
 * NESTED MAPS
 * ============================================================================
 *
 * Nested maps are naturally supported because both key and value consume the
 * canonical expression rule.
 *
 * Examples:
 *
 *     {a: {b: c}}
 *
 *     {{a: b}: c}
 *
 *     {a: {b: {c: d}}}
 *
 *     {{a: b}: {c: d}}
 *
 * No nesting-depth constant is permitted.
 *
 * Actual parser-stack or resource limits remain implementation/resource policy.
 *
 * ============================================================================
 * MAPS INSIDE OTHER EXPRESSIONS
 * ============================================================================
 *
 * Map expressions can occur wherever the enclosing expression grammar permits
 * a primary expression.
 *
 * Examples:
 *
 *     f({a: b})
 *
 *     [{a: b}, {c: d}]
 *
 *     ({a: b}, {c: d})
 *
 *     value[{key: value}]
 *
 *     {a: f({b: c})}
 *
 *     condition ? {a: b} : {c: d}
 *
 * The map grammar owns only the map literal itself.
 *
 * ============================================================================
 * MAPS AND POSTFIX OPERATIONS
 * ============================================================================
 *
 * Because `mapExpression` is a primary expression, postfix syntax may operate
 * on a map expression when the semantic type supports it.
 *
 * Examples:
 *
 *     {a: b}[a]
 *
 *     {a: b}.field
 *
 *     {a: b}(argument)
 *
 * The grammar must not decide whether such operations are semantically valid.
 *
 * Indexing remains owned by:
 *
 *     grammar/expressions/indexing.g4
 *
 * Member access remains owned by:
 *
 *     grammar/expressions/member-access.g4
 *
 * Calls remain owned by:
 *
 *     grammar/expressions/calls.g4
 *
 * ============================================================================
 * NO FIXED MAP SIZE
 * ============================================================================
 *
 * There is no universal maximum number of entries.
 *
 * The grammar therefore uses:
 *
 *     mapEntryList
 *         : mapEntry (COMMA mapEntry)*
 *         ;
 *
 * rather than finite alternatives such as:
 *
 *     map1
 *     map2
 *     map4
 *     map8
 *     map16
 *
 * No grammar constants such as:
 *
 *     MAX_MAP_ENTRIES
 *     MAX_MAP_SIZE
 *     MAX_MAP_KEYS
 *     MAX_MAP_DEPTH
 *
 * are permitted.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no universal upper bound on:
 *
 *     map entry count;
 *     key expression size;
 *     value expression size;
 *     nesting depth;
 *     number of maps;
 *     map nesting;
 *     key complexity;
 *     value complexity;
 *     source-file size;
 *     number of expressions;
 *     number of declarations;
 *     number of program operations;
 *     number of resources;
 *     number of nodes;
 *     number of CPUs;
 *     number of cores;
 *     number of threads;
 *     number of GPUs;
 *     number of FPGAs;
 *     number of accelerators;
 *     number of QPUs;
 *     number of logical qubits;
 *     memory capacity;
 *     tensor dimensions;
 *     distributed participants.
 *
 * "Infinity" means:
 *
 *     the language grammar introduces no artificial finite universal limit.
 *
 * It does NOT mean:
 *
 *     a physical implementation possesses infinite memory or compute.
 *
 * Actual resource limitations belong to:
 *
 *     parser resource policy;
 *     compiler resource policy;
 *     available memory;
 *     available storage;
 *     target capabilities;
 *     runtime resources;
 *     deployment policy.
 *
 * Such limitations must not become syntax-level map-size limits.
 *
 * ============================================================================
 * SEMANTIC MAP SIZE
 * ============================================================================
 *
 * A map with one entry:
 *
 *     {a: b}
 *
 * and a map with many entries:
 *
 *     {a: b, c: d, e: f, ...}
 *
 * are syntactically represented by the same structure:
 *
 *     MapExpression
 *         |
 *         +-- MapEntry*
 *
 * The AST collection must be dynamically sized.
 *
 * No fixed-size tuple/array/map representation may be introduced merely to
 * simplify parsing.
 *
 * ============================================================================
 * MAP KEY SEMANTICS
 * ============================================================================
 *
 * The grammar intentionally accepts any canonical expression as a key.
 *
 * Semantic analysis determines whether the key is valid.
 *
 * Examples of possible semantic policies include:
 *
 *     hashable key;
 *     comparable key;
 *     identity key;
 *     symbolic key;
 *     structural key;
 *     ordered key;
 *     user-defined key;
 *     runtime key;
 *     distributed key.
 *
 * This grammar does not select any one of these policies.
 *
 * In particular, the parser MUST NOT reject a key merely because it is not
 * obviously an integer or string.
 *
 * This is essential for a universal language.
 *
 * ============================================================================
 * KEY UNIQUENESS
 * ============================================================================
 *
 * Duplicate keys are NOT a grammar error.
 *
 * For example:
 *
 *     {a: 1, a: 2}
 *
 * is syntactically representable.
 *
 * Whether duplicate keys are:
 *
 *     rejected;
 *     merged;
 *     overwritten;
 *     preserved;
 *     diagnosed;
 *     ordered;
 *     implementation-defined;
 *
 * is a semantic/type-system/library decision.
 *
 * The parser must preserve every entry and its source order.
 *
 * It must never silently discard a duplicate entry.
 *
 * ============================================================================
 * KEY ORDER
 * ============================================================================
 *
 * Source order is part of the syntax tree.
 *
 * This grammar preserves:
 *
 *     entry1
 *     entry2
 *     entry3
 *
 * in source order.
 *
 * Whether the semantic map:
 *
 *     preserves insertion order;
 *     canonicalizes order;
 *     hashes entries;
 *     stores entries in a tree;
 *     distributes entries;
 *
 * is a downstream implementation decision.
 *
 * ============================================================================
 * MAP VALUE SEMANTICS
 * ============================================================================
 *
 * Values are canonical Zamani expressions.
 *
 * They may therefore be:
 *
 *     literals;
 *     identifiers;
 *     calls;
 *     tuples;
 *     arrays;
 *     nested maps;
 *     ranges;
 *     indexing expressions;
 *     quantum expressions;
 *     hybrid expressions;
 *     hardware/resource expressions;
 *     AI/data expressions;
 *     distributed expressions;
 *     compile-time expressions;
 *     future dialect expressions.
 *
 * No special map-value grammar is required.
 *
 * ============================================================================
 * CLASSICAL COMPUTING INTEGRATION
 * ============================================================================
 *
 * Maps may represent:
 *
 *     dictionaries;
 *     associative collections;
 *     lookup structures;
 *     symbol tables;
 *     configuration data;
 *     sparse data;
 *     metadata;
 *     scientific mappings;
 *     application state.
 *
 * The grammar does not prescribe the runtime representation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Maps may syntactically contain quantum expressions.
 *
 * Examples:
 *
 *     {q0: measurement}
 *
 *     {logical_q0: physical_intent}
 *
 *     {state_label: amplitude}
 *
 * These are source expressions only.
 *
 * This grammar does NOT:
 *
 *     - allocate qubits;
 *     - select physical qubits;
 *     - map logical qubits;
 *     - choose a QPU;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - model noise;
 *     - select calibration;
 *     - select a vendor.
 *
 * Any quantum meaning is resolved downstream and eventually integrates with:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HYBRID COMPUTING INTEGRATION
 * ============================================================================
 *
 * A map can contain classical and quantum expressions together:
 *
 *     {classical_key: quantum_value}
 *
 * or:
 *
 *     {quantum_key: classical_value}
 *
 * or:
 *
 *     {key: measure(q)}
 *
 * Whether such combinations are semantically valid belongs to hybrid semantic
 * analysis.
 *
 * The grammar remains domain-neutral.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Map expressions may occur in HDL/hardware constructs wherever a general
 * expression is permitted.
 *
 * The map grammar does not define:
 *
 *     bus widths;
 *     register widths;
 *     pin counts;
 *     memory-bank counts;
 *     FPGA resource counts;
 *     ASIC cell counts;
 *     physical addresses;
 *     device topology.
 *
 * Those belong to hardware/resource semantics.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Maps can represent:
 *
 *     feature metadata;
 *     configuration;
 *     model metadata;
 *     dataset metadata;
 *     symbolic environments;
 *     sparse structures;
 *     parameter associations;
 *     pipeline configuration.
 *
 * Framework-specific semantics must remain outside this grammar.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A map can semantically become:
 *
 *     distributed metadata;
 *     partition mappings;
 *     routing metadata;
 *     node-to-value associations;
 *     service metadata;
 *     replicated state.
 *
 * The grammar does not determine:
 *
 *     node count;
 *     placement;
 *     replication factor;
 *     partition strategy;
 *     consistency model;
 *     network topology.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Map expressions may carry resource or capability data:
 *
 *     {capability_name: requirement}
 *
 *     {resource_name: preference}
 *
 * However, the map grammar does not distinguish:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     implementation decision
 *
 * Those distinctions belong to the resource/capability semantic contracts.
 *
 * ============================================================================
 * OWNERSHIP / BORROWING
 * ============================================================================
 *
 * The parser does not determine:
 *
 *     ownership;
 *     borrowing;
 *     aliasing;
 *     lifetime;
 *     mutability;
 *     move semantics.
 *
 * A map expression simply preserves its key/value expression structure.
 *
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * Keys and values may syntactically contain expressions with effects.
 *
 * For example:
 *
 *     {compute_key(): compute_value()}
 *
 * Effect legality and evaluation order belong to semantic analysis.
 *
 * The grammar preserves source ordering but does not define runtime evaluation
 * scheduling.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing a map must depend only on:
 *
 *     token sequence;
 *     grammar version.
 *
 * It must not depend on:
 *
 *     system time;
 *     randomness;
 *     environment variables;
 *     hardware discovery;
 *     network state;
 *     runtime scheduler state;
 *     resource availability.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The frontend AST must preserve source spans for:
 *
 *     mapExpression;
 *     each mapEntry;
 *     each key expression;
 *     each value expression;
 *     delimiters where source-fidelity requirements require them.
 *
 * This enables diagnostics such as:
 *
 *     duplicate-key diagnostics;
 *     invalid-key-type diagnostics;
 *     invalid-value-type diagnostics;
 *     unsupported map operation diagnostics;
 *
 * without requiring the grammar to perform semantic validation.
 *
 * ============================================================================
 * DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to parsing.
 *
 * Examples:
 *
 *     {
 *     {a}
 *     {a:}
 *     {:b}
 *     {a: b c: d}
 *     {a: b,}
 *     {a: b,, c: d}
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     unhashable key;
 *     incompatible key types;
 *     incompatible value types;
 *     duplicate keys under a uniqueness policy;
 *     unsupported key domain;
 *     resource-incompatible map representation.
 *
 * The parser must not encode those semantic rules.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * The AST must preserve map-entry source order.
 *
 * For:
 *
 *     {a: first(), b: second(), c: third()}
 *
 * the syntax tree must preserve:
 *
 *     a: first()
 *     b: second()
 *     c: third()
 *
 * in exactly that source order.
 *
 * Later semantic phases may establish an evaluation or storage policy, but
 * source ordering must not be destroyed by parsing.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar must lower into the repository's existing domain-neutral AST
 * contract.
 *
 * It must NOT introduce a second map-specific frontend hierarchy merely because
 * this file is modular.
 *
 * The preferred representation is:
 *
 *     mapExpression
 *          |
 *          v
 *     generic collection/map expression representation
 *          |
 *          v
 *     canonical semantic model
 *
 * Where the frontend AST has a dedicated structural node for map literals,
 * that node must be the canonical existing node rather than a new duplicate.
 *
 * If the frontend uses the generic `Operation` representation established for
 * extensible expressions, the semantic operation identity should be something
 * equivalent to:
 *
 *     map.literal
 *
 * with ordered key/value operands or a structurally equivalent attribute
 * representation.
 *
 * The exact AST implementation remains owned by `src/frontend/ast/`.
 *
 * This grammar must not hard-code a Rust AST type.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes:
 *
 *     this is structurally a map expression.
 *
 * Semantic analysis establishes:
 *
 *     what map type it has;
 *     what key type it has;
 *     what value type it has;
 *     whether keys are valid;
 *     whether duplicate keys are legal;
 *     whether evaluation has effects;
 *     what ownership applies;
 *     what capabilities are required;
 *     what resources are required;
 *     what representation is selected.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Map expressions do not create a new universal IR.
 *
 * The semantic map representation may lower into whichever canonical/domain IR
 * is appropriate:
 *
 *     classical IR
 *     data IR
 *     distributed IR
 *     hardware/HDL IR
 *     quantum::ir where the map participates in a quantum semantic construct
 *
 * This grammar does not select the target IR.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler responsibilities include:
 *
 *     type-directed lowering;
 *     representation selection;
 *     optimization;
 *     storage strategy;
 *     constant folding where legal;
 *     specialization where explicitly permitted;
 *     target capability matching;
 *     resource planning.
 *
 * The compiler must not reinterpret map syntax differently on different
 * targets merely because a target has a different map implementation.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime responsibilities may include:
 *
 *     allocation;
 *     lookup;
 *     mutation;
 *     concurrency;
 *     synchronization;
 *     persistence;
 *     distribution;
 *     caching;
 *     serialization.
 *
 * None of these are parser responsibilities.
 *
 * ============================================================================
 * NO HARD-CODED MACHINE LIMITS
 * ============================================================================
 *
 * This file MUST NOT contain universal limits such as:
 *
 *     MAX_MAP_ENTRIES
 *     MAX_MAP_SIZE
 *     MAX_MAP_DEPTH
 *     MAX_KEY_SIZE
 *     MAX_VALUE_SIZE
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_QUBITS
 *
 * Nor may it encode:
 *
 *     32-bit map keys;
 *     64-bit map keys;
 *     fixed hash width;
 *     fixed bucket count;
 *     fixed table size;
 *     fixed node count;
 *     fixed memory capacity.
 *
 * Literal values such as:
 *
 *     {0: "zero"}
 *
 * are ordinary program semantics and are not prohibited.
 *
 * The prohibition applies to universal language/implementation ceilings.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file implements the canonical MapExpression syntax.
 *
 * Existing inline map syntax in `grammar/Zamani.g4` must be replaced by the
 * canonical delegated rule during grammar composition.
 *
 * The old inline production:
 *
 *     mapExpression
 *         : '{' mapEntryList? '}'
 *
 * must not remain as a second competing definition after modular composition.
 *
 * There must be exactly one authoritative `mapExpression` rule in the
 * composed grammar.
 *
 * `grammar/spec/syntax.md` remains the normative syntax authority.
 *
 * `grammar/Zamani-Grammar.md` may describe broader or historical designs but
 * must not silently override this rule.
 *
 * `grammar/grammar.md` records implementation conformance.
 *
 * ============================================================================
 * VALIDATION
 * ============================================================================
 *
 * This grammar should be validated for:
 *
 *     - unreachable rules;
 *     - duplicate rules;
 *     - duplicate token declarations;
 *     - token-vocabulary mismatch;
 *     - grammar-import cycles;
 *     - ambiguity;
 *     - parser nondeterminism;
 *     - missing expression integration;
 *     - source-span coverage;
 *     - AST coverage;
 *     - semantic coverage;
 *     - IR coverage;
 *     - compatibility coverage;
 *     - hard-coded limits.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax cases:
 *
 *     {}
 *     {a: b}
 *     {a: b, c: d}
 *     {a + 1: b * 2}
 *     {f(x): g(y)}
 *     {array[i]: value}
 *     {object.field: value}
 *     {q0: measurement}
 *     {classical_value: quantum_value}
 *     {a: {b: c}}
 *     {{a: b}: c}
 *     {{a: b}: {c: d}}
 *     {condition ? left : right: value}
 *     {key: condition ? left : right}
 *     {a: f({b: c})}
 *
 * Negative syntax cases:
 *
 *     {
 *     {a}
 *     {a:}
 *     {:b}
 *     {a b}
 *     {a: b c: d}
 *     {a: b,}
 *     {a: b,, c: d}
 *     {a: b c}
 *
 * Boundary cases:
 *
 *     {}
 *     {a: b}
 *     nested maps
 *     map as array element
 *     map as tuple element
 *     map as function argument
 *     map used before indexing
 *     map used before member access
 *     map used before a call
 *
 * Scalability cases:
 *
 *     generated maps with increasing entry counts;
 *     generated nested maps;
 *     generated large key expressions;
 *     generated large value expressions;
 *     maps containing arbitrarily large expressions.
 *
 * The scalability tests MUST NOT establish a language-level maximum.
 *
 * Determinism cases:
 *
 *     identical token streams must produce identical syntax structures;
 *     map parsing must not depend on hardware;
 *     map parsing must not depend on resource discovery;
 *     map parsing must not depend on time or randomness.
 *
 * ============================================================================
 * INDEPENDENT COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. It is a parser grammar delegate.
 *     2. It defines no lexer rules.
 *     3. It consumes canonical lexer tokens.
 *     4. It exposes `mapExpression`.
 *     5. It exposes `mapEntryList`.
 *     6. It exposes `mapEntry`.
 *     7. Empty maps are syntactically representable.
 *     8. Single-entry maps are representable.
 *     9. Multi-entry maps are representable.
 *    10. Map keys consume the canonical `expression`.
 *    11. Map values consume the canonical `expression`.
 *    12. No fixed entry count exists.
 *    13. No fixed nesting depth exists.
 *    14. No machine/resource limits exist.
 *    15. Trailing commas remain rejected according to the normative syntax.
 *    16. Duplicate keys are preserved rather than discarded.
 *    17. Source order is preserved.
 *    18. Nested maps are representable.
 *    19. Maps compose with arrays, tuples, calls, indexing and member access.
 *    20. Quantum syntax remains domain-neutral.
 *    21. HDL syntax remains domain-neutral.
 *    22. Hardware/resource realization remains downstream.
 *    23. The canonical `quantum::ir` boundary remains untouched.
 *    24. No second map-specific IR is introduced.
 *    25. AST integration is predetermined.
 *    26. Semantic integration is predetermined.
 *    27. Compiler integration is predetermined.
 *    28. Runtime integration is predetermined.
 *    29. Diagnostics and source spans are preserved.
 *    30. Positive, negative, boundary and scalability tests are defined.
 *    31. Deterministic parsing is maintained.
 *    32. The existing inline map rule is removed from the canonical root
 *        composition so there is only one authoritative map rule.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION NOTE
 * ============================================================================
 *
 * This file is intentionally self-contained at the FEATURE-CONTRACT level,
 * but grammar composition still requires the canonical expression composition
 * layer to expose `mapExpression` from `primaryExpression`.
 *
 * The required composition is conceptually:
 *
 *     primaryExpression
 *         : ...
 *         | mapExpression
 *         | ...
 *         ;
 *
 * The composition layer must also resolve the existing `{}` interaction with
 * block expressions deterministically.
 *
 * No change to this file should be required when unrelated expression features
 * are subsequently added.
 *
 * ============================================================================
 */

parser grammar Maps;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC MAP EXPRESSION
 * ========================================================================== */

/*
 * Complete map literal.
 *
 * Canonical forms:
 *
 *     {}
 *     {key: value}
 *     {key1: value1, key2: value2}
 *
 * No trailing comma is accepted.
 */
mapExpression
    : LBRACE
      mapEntryList?
      RBRACE
    ;


/* ============================================================================
 * MAP ENTRY LIST
 * ========================================================================== */

/*
 * One or more map entries separated by commas.
 *
 * The repetition is intentionally unbounded at the grammar level.
 *
 * There is deliberately no:
 *
 *     mapEntry2
 *     mapEntry4
 *     mapEntry8
 *     mapEntry16
 *
 * or equivalent finite expansion.
 */
mapEntryList
    : mapEntry
      (
          COMMA
          mapEntry
      )*
    ;


/* ============================================================================
 * MAP ENTRY
 * ========================================================================== */

/*
 * One key/value association.
 *
 * Both sides consume the canonical expression rule.
 *
 * The map grammar therefore does not duplicate expression precedence or
 * expression semantics.
 */
mapEntry
    : expression
      COLON
      expression
    ;