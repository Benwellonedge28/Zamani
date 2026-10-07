/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/existential-types.g4
 *
 * Grammar:
 *     ExistentialTypes
 *
 * Status:
 *     PRODUCTION TYPE-SYSTEM DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the SINGLE OWNER of source-level existential type syntax.
 *
 * An existential type introduces one or more type variables whose concrete
 * identities are hidden from the consumer while their declared constraints
 * remain available to semantic type checking.
 *
 * Canonical forms:
 *
 *     exists T. T
 *
 *     exists T: Numeric. T
 *
 *     exists T: Numeric + Comparable. Container<T>
 *
 *     exists T, U. Pair<T, U>
 *
 *     exists T: Storage, U: Codec<T>. Package<T, U>
 *
 * Existential types are useful for:
 *
 *     opaque abstraction;
 *     hidden implementation types;
 *     package-like values;
 *     abstraction boundaries;
 *     type erasure;
 *     interface/trait-backed values;
 *     plugin boundaries;
 *     capability-bearing abstractions;
 *     heterogeneous collections;
 *     FFI abstraction;
 *     domain-independent implementation hiding.
 *
 * They are a TYPE-SYSTEM feature.
 *
 * They are not:
 *
 *     dynamic typing;
 *     Any;
 *     unrestricted reflection;
 *     target selection;
 *     runtime resource discovery;
 *     hardware allocation;
 *     quantum allocation;
 *     backend-specific representation.
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Existential syntax describes a semantic type relationship.
 *
 * It MUST NOT prescribe how the hidden type is represented or implemented.
 *
 * Therefore:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     existential type semantics
 *       |
 *       v
 *     type checking / constraint solving
 *       |
 *       v
 *     canonical semantic type model
 *       |
 *       v
 *     canonical IR
 *       |
 *       v
 *     target-independent optimization
 *       |
 *       v
 *     target realization
 *
 * This grammar performs none of the semantic steps after parsing.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     existentialType
 *     existentialBinderList
 *     existentialBinder
 *
 * THIS FILE ALSO OWNS:
 *
 *     the source-level relationship:
 *
 *         exists binders. body
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     generic arguments
 *     generic declarations
 *     type bounds
 *     named types
 *     associated types
 *     type classes
 *     dependent types
 *     type-level values
 *     contracts
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     provenance
 *     semantic type checking
 *     type equality
 *     type substitution
 *     existential elimination
 *     type erasure
 *     runtime representation
 *     ABI representation
 *     target lowering
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * SINGLE TYPE-EXPRESSION AUTHORITY
 * ============================================================================
 *
 * This file MUST NOT define another type-expression grammar.
 *
 * Both:
 *
 *     existential bounds
 *
 * and:
 *
 *     existential bodies
 *
 * consume the canonical:
 *
 *     typeExpression
 *
 * supplied by the type-system orchestration grammar.
 *
 * The dependency is therefore:
 *
 *     canonical typeExpression
 *              |
 *              +--------------------+
 *              |                    |
 *              v                    v
 *       existential bound       existential body
 *
 * There MUST NOT be another rule such as:
 *
 *     existentialTypeExpression
 *
 * or:
 *
 *     existentialBoundType
 *
 * which creates a competing type language.
 *
 * ============================================================================
 * TYPE-BOUND AUTHORITY
 * ============================================================================
 *
 * Type bounds are owned by:
 *
 *     grammar/types/bounds.g4
 *
 * That file owns:
 *
 *     typeBoundClause
 *     typeBoundList
 *     typeBound
 *
 * Therefore this file consumes:
 *
 *     typeBoundClause
 *
 * rather than redefining:
 *
 *     :
 *     +
 *     bound expressions
 *
 * This allows:
 *
 *     exists T: Numeric + Comparable. T
 *
 * to use exactly the same bound semantics as ordinary generic parameters.
 *
 * ============================================================================
 * GENERIC PARAMETER SEPARATION
 * ============================================================================
 *
 * An existential binder is NOT a generic declaration.
 *
 * Generic declarations remain owned by the generic/declaration subsystem.
 *
 * For example:
 *
 *     fn identity<T>(value: T) -> T
 *
 * contains a generic parameter.
 *
 * Whereas:
 *
 *     exists T. Container<T>
 *
 * contains an existential binder.
 *
 * They may use the same semantic type-parameter representation internally,
 * but their quantification and scope are different semantic constructs.
 *
 * This file therefore MUST NOT import or redefine generic parameter-list
 * syntax merely to implement existential binders.
 *
 * ============================================================================
 * EXISTENTIAL QUANTIFICATION
 * ============================================================================
 *
 * The source form:
 *
 *     exists T. Body
 *
 * means, semantically:
 *
 *     there exists some type T such that Body is well formed.
 *
 * The concrete identity of T is hidden outside the existential scope.
 *
 * The body may refer to T.
 *
 * The enclosing semantic context may not arbitrarily recover T's concrete
 * identity unless an explicitly defined elimination/projection operation
 * permits doing so.
 *
 * This rule belongs to semantic analysis, not parsing.
 *
 * ============================================================================
 * BINDER LIST
 * ============================================================================
 *
 * Multiple existential variables are permitted:
 *
 *     exists T, U. Pair<T, U>
 *
 *     exists T, U: Relation<T, U>. Package<T, U>
 *
 * Each binder has:
 *
 *     name
 *     optional bound clause
 *
 * The grammar imposes no artificial number of existential binders.
 *
 * Therefore:
 *
 *     exists T, U, V, ...
 *
 * remains expressible subject only to implementation resources.
 *
 * ============================================================================
 * BINDER SCOPE
 * ============================================================================
 *
 * A binder is in scope:
 *
 *     from the end of its declaration
 *     through the existential body
 *
 * For:
 *
 *     exists T, U: Relation<T, U>. Pair<T, U>
 *
 * the semantic binder environment contains:
 *
 *     T
 *     U
 *
 * before validating the body.
 *
 * Bounds may reference earlier binders.
 *
 * Example:
 *
 *     exists T, U: Relation<T, U>. Pair<T, U>
 *
 * If a bound references a later binder, semantic analysis determines whether
 * such forward references are permitted.
 *
 * This grammar does not impose semantic ordering beyond preserving source
 * order.
 *
 * ============================================================================
 * BOUND SEMANTICS
 * ============================================================================
 *
 * The grammar accepts:
 *
 *     exists T: Numeric. T
 *
 *     exists T: Numeric + Comparable. T
 *
 *     exists T: quantum::State. T
 *
 *     exists T: hardware::Accelerator<Model>. T
 *
 * Whether a bound is:
 *
 *     satisfiable;
 *     nominal;
 *     structural;
 *     trait-like;
 *     capability-like;
 *     domain-specific;
 *
 * is determined by semantic analysis.
 *
 * The grammar never maintains a catalogue of valid bounds.
 *
 * ============================================================================
 * TYPE ERASURE
 * ============================================================================
 *
 * Existential types may eventually lower to:
 *
 *     opaque semantic types;
 *     erased representations;
 *     dictionary/interface passing;
 *     witness tables;
 *     tagged representations;
 *     target-specific ABI structures;
 *     other valid implementation strategies.
 *
 * None of these representations belong in this grammar.
 *
 * The source meaning remains:
 *
 *     hidden type + declared constraints + body type
 *
 * ============================================================================
 * DYNAMIC TYPE SEPARATION
 * ============================================================================
 *
 * Existential types MUST NOT silently become dynamic types.
 *
 * In particular:
 *
 *     exists T. T
 *
 * is not equivalent to:
 *
 *     Any
 *
 * and:
 *
 *     exists T: Trait. T
 *
 * is not equivalent to:
 *
 *     dynamic
 *
 * Unless the semantic specification explicitly defines such an equivalence,
 * semantic analysis must preserve the distinction.
 *
 * Type inference failure must never silently synthesize an existential type.
 *
 * ============================================================================
 * SUBSTITUTION
 * ============================================================================
 *
 * Existential variables are bound variables.
 *
 * Semantic substitution must therefore respect lexical scope.
 *
 * For:
 *
 *     exists T. Pair<T, T>
 *
 * substitution inside the existential body must not accidentally replace the
 * bound T with an unrelated outer type parameter.
 *
 * This grammar only preserves the source structure.
 *
 * Capture avoidance, alpha-renaming and substitution are semantic/type-system
 * responsibilities.
 *
 * ============================================================================
 * ALPHA-EQUIVALENCE
 * ============================================================================
 *
 * The following types may be semantically equivalent:
 *
 *     exists T. Box<T>
 *
 *     exists U. Box<U>
 *
 * provided all other semantic information is equivalent.
 *
 * The binder spelling itself must therefore not become the semantic identity
 * of the existential type.
 *
 * Canonical semantic identity must use binding structure rather than relying
 * solely on source identifier spelling.
 *
 * ============================================================================
 * NESTED EXISTENTIALS
 * ============================================================================
 *
 * Existential types may be nested:
 *
 *     exists T. Box<exists U. Pair<T, U>>
 *
 * subject to the surrounding type-system rules.
 *
 * The grammar introduces no nesting ceiling.
 *
 * Semantic analysis is responsible for:
 *
 *     scope;
 *     substitution;
 *     normalization;
 *     equivalence;
 *     elimination;
 *     representation.
 *
 * ============================================================================
 * EXISTENTIALS INSIDE GENERIC APPLICATIONS
 * ============================================================================
 *
 * Existential types may occur wherever the canonical type-expression grammar
 * permits them.
 *
 * Examples:
 *
 *     Vec<exists T. T>
 *
 *     Result<exists T: ErrorType. T, Error>
 *
 *     Model<Input, exists State. State>
 *
 * This file does not special-case generic applications.
 *
 * ============================================================================
 * GENERICS INSIDE EXISTENTIALS
 * ============================================================================
 *
 * Generic applications may occur in existential bounds and bodies.
 *
 * Examples:
 *
 *     exists T: Iterator<Item>. Vec<T>
 *
 *     exists T. Container<Map<String, T>>
 *
 * Generic application syntax remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * ============================================================================
 * ASSOCIATED TYPE INTEGRATION
 * ============================================================================
 *
 * Associated types may appear in bounds and bodies:
 *
 *     exists T: Iterator. T::Item
 *
 *     exists T: Container. Container<T>::Item
 *
 * Associated type syntax remains owned by:
 *
 *     grammar/types/associated.g4
 *
 * This file does not redefine projection syntax.
 *
 * ============================================================================
 * TYPE-CLASS / TRAIT INTEGRATION
 * ============================================================================
 *
 * Existential bounds are compatible with the repository's generic
 * type-class/trait model.
 *
 * Example:
 *
 *     exists T: Comparable. T
 *
 * Whether `Comparable` is a:
 *
 *     trait;
 *     type class;
 *     interface;
 *     structural predicate;
 *     semantic capability;
 *
 * remains a semantic decision.
 *
 * ============================================================================
 * DEPENDENT TYPE INTEGRATION
 * ============================================================================
 *
 * Existential binders may participate in dependent types where permitted by
 * the semantic type system.
 *
 * Example:
 *
 *     exists N: Natural. Vector<N>
 *
 * The grammar does not evaluate N.
 *
 * The type/value distinction remains owned by the dependent-type subsystem.
 *
 * ============================================================================
 * LINEAR / AFFINE INTEGRATION
 * ============================================================================
 *
 * Existential types may hide resource-sensitive types:
 *
 *     exists T: ResourceType. T
 *
 * or:
 *
 *     exists T. Linear<T>
 *
 * Ownership, linearity and affinity are not checked here.
 *
 * They remain semantic responsibilities.
 *
 * This is particularly important for quantum and hardware resources.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Existential types may abstract over quantum semantic types:
 *
 *     exists Q: quantum::State. Q
 *
 *     exists Q: quantum::Operation. Container<Q>
 *
 * This grammar MUST NOT:
 *
 *     allocate physical qubits;
 *     select physical qubits;
 *     identify a QPU;
 *     choose a coupling map;
 *     route operations;
 *     schedule operations;
 *     select calibration;
 *     select QEC;
 *     construct pulses;
 *     access ZQN;
 *     access HAL state.
 *
 * The quantum pipeline remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum type
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
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * No second quantum IR is permitted.
 *
 * ============================================================================
 * CLASSICAL / AI / DATA INTEGRATION
 * ============================================================================
 *
 * Existential abstraction can be used for:
 *
 *     numeric implementations;
 *     tensor implementations;
 *     models;
 *     datasets;
 *     knowledge structures;
 *     inference engines;
 *     probabilistic representations;
 *     agents;
 *     data containers.
 *
 * Examples:
 *
 *     exists T: TensorLike. Model<T>
 *
 *     exists M: Model. M
 *
 *     exists D: Dataset. Processor<D>
 *
 * Application-specific semantics remain outside this grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Existential types may hide hardware-independent implementations:
 *
 *     exists S: SignalLike. Module<S>
 *
 *     exists A: Accelerator. A
 *
 *     exists M: MemoryLike. Buffer<M>
 *
 * The source type does not identify:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator vendor;
 *     register width;
 *     physical topology;
 *     device count;
 *     memory capacity.
 *
 * Hardware realization is downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * An existential type can be constrained by a semantic type representing a
 * capability:
 *
 *     exists T: quantum::Measurable. T
 *
 * This does not mean the grammar performs capability discovery.
 *
 * Runtime/compiler resource requirements remain separate:
 *
 *     requires capability("quantum.measurement")
 *
 *     requires memory >= required_memory
 *
 *     requires topology(required_topology)
 *
 * Those constructs belong to:
 *
 *     grammar/resources/
 *     grammar/core/
 *     grammar/policies/
 *
 * and their semantic systems.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Merely constructing an existential type has no execution effect.
 *
 * This:
 *
 *     exists T: NetworkType. T
 *
 * does not perform networking.
 *
 * This:
 *
 *     exists T: quantum::State. T
 *
 * does not perform quantum measurement.
 *
 * Effect semantics remain owned by:
 *
 *     grammar/effects/
 *
 * and downstream semantic analysis.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Existential types may appear inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * but this file does not own those constructs.
 *
 * Contract checking occurs after parsing and type resolution.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may constrain operations involving existential values.
 *
 * Examples include policies concerning:
 *
 *     FFI;
 *     reflection;
 *     dynamic dispatch;
 *     resource use;
 *     adaptation;
 *     serialization.
 *
 * Policy syntax remains owned by the policy subsystem.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * The parser must preserve source spans for:
 *
 *     exists;
 *     each binder;
 *     each bound;
 *     the existential separator;
 *     the body.
 *
 * This allows downstream provenance to distinguish:
 *
 *     source declaration;
 *     generated type;
 *     macro-generated type;
 *     transformed type;
 *     compatibility migration.
 *
 * This grammar does not create provenance records itself.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Existential types are target-independent source semantics.
 *
 * They must remain representable regardless of whether the eventual program
 * is realized on:
 *
 *     tiny embedded hardware;
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed infrastructure;
 *     cloud;
 *     future computational substrate.
 *
 * No existential grammar rule may encode:
 *
 *     maximum hidden types;
 *     maximum binders;
 *     maximum bounds;
 *     maximum type depth;
 *     maximum quantum resources;
 *     maximum devices;
 *     maximum memory;
 *     maximum processors;
 *     maximum topology size.
 *
 * "Infinity" means that the language imposes no artificial finite semantic
 * ceiling. Actual parser/compiler resources remain implementation realities.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar contains no fixed existential capacity.
 *
 * Specifically, it MUST NOT contain:
 *
 *     MAX_EXISTENTIAL_BINDERS
 *     MAX_EXISTENTIAL_DEPTH
 *     MAX_EXISTENTIAL_BOUNDS
 *     MAX_TYPE_PARAMETERS
 *     MAX_GENERIC_ARITY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Any compiler protection against pathological input must live outside the
 * language grammar and must be:
 *
 *     explicit;
 *     configurable;
 *     deterministic;
 *     diagnosable;
 *     separate from language semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing an existential type must depend only on:
 *
 *     source text;
 *     language version;
 *     lexical configuration;
 *     grammar version.
 *
 * It must not depend on:
 *
 *     target hardware;
 *     runtime state;
 *     available resources;
 *     filesystem state;
 *     network state;
 *     randomness;
 *     wall-clock time.
 *
 * Identical source under identical language configuration must produce the
 * same structural parse and source ordering.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains no lexer rules.
 *
 * The production lexical vocabulary requires one canonical token:
 *
 *     EXISTS
 *
 * spelling:
 *
 *     exists
 *
 * Ownership:
 *
 *     grammar/lexer/keywords.g4
 *     or the repository's canonical keyword component that owns reserved
 *     language vocabulary.
 *
 * There MUST be exactly one emitted token identity for `exists`.
 *
 * Do NOT define:
 *
 *     EXISTENTIAL
 *     EXISTENTIAL_TYPE
 *     EXISTS_TYPE
 *
 * as additional spellings for the same keyword.
 *
 * Existing lexical punctuation consumed here is:
 *
 *     COMMA
 *     COLON
 *     DOT
 *
 * The punctuation remains owned by the canonical lexer.
 *
 * ============================================================================
 * PARSER CONTRACT
 * ============================================================================
 *
 * The canonical type orchestrator:
 *
 *     grammar/types/types.g4
 *
 * must import/delegate to this grammar.
 *
 * It must expose `existentialType` through its existing `typeCore` composition.
 *
 * It must NOT redefine:
 *
 *     existentialType
 *     existentialBinder
 *     existentialBinderList
 *
 * Therefore the integration direction is:
 *
 *     Type
 *       |
 *       +--> ExistentialTypes
 *                    |
 *                    +--> typeExpression
 *                    +--> typeBoundClause
 *
 * This does not create a second type-expression authority.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The current canonical frontend `TypeExpr` does not yet contain an
 * existential variant.
 *
 * Production integration therefore requires one canonical source-level AST
 * representation.
 *
 * Required semantic shape:
 *
 *     TypeExpr::Existential {
 *         binders: Vec<ExistentialBinder>,
 *         body: Box<TypeExpr>,
 *     }
 *
 * where an existential binder contains:
 *
 *     name: TypeParameterName
 *     bounds: Vec<TypeExpr>
 *
 * The exact Rust struct location belongs to:
 *
 *     src/frontend/ast/node/types/
 *
 * The implementation MUST reuse the existing canonical `TypeExpr`,
 * `TypeParameterName`, and type-bound representation rather than creating a
 * parallel existential AST hierarchy.
 *
 * A compatibility adapter may expose a dedicated `ExistentialType` helper
 * structure, but `TypeExpr::Existential` remains the canonical AST identity.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     create a fresh existential binder scope;
 *     bind every existential parameter;
 *     validate bounds;
 *     validate the body under that scope;
 *     prevent variable capture;
 *     preserve binder identity;
 *     support alpha-equivalence;
 *     distinguish existential from universal quantification;
 *     distinguish existential from dynamic typing;
 *     enforce elimination rules;
 *     validate variance where applicable;
 *     preserve ownership/linearity;
 *     preserve effect information;
 *     preserve capability requirements;
 *     preserve resource semantics.
 *
 * Semantic analysis owns:
 *
 *     substitution;
 *     unification;
 *     normalization;
 *     type equality;
 *     constraint solving;
 *     witness handling;
 *     existential elimination.
 *
 * None of those decisions belong here.
 *
 * ============================================================================
 * TYPE-CHECKING CONTRACT
 * ============================================================================
 *
 * The type checker must distinguish:
 *
 *     existential introduction
 *
 * from:
 *
 *     existential elimination.
 *
 * A valid existential package must not expose the hidden concrete type merely
 * because the consumer knows the binder's source spelling.
 *
 * For example:
 *
 *     exists T: Numeric. T
 *
 * does not permit the surrounding context to assume:
 *
 *     T == Int
 *
 * without an explicit semantic witness or elimination rule.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not create IR.
 *
 * The canonical flow is:
 *
 *     TypeExpr::Existential
 *              |
 *              v
 *     semantic existential type
 *              |
 *              v
 *     canonical target-independent IR
 *
 * The IR representation may use:
 *
 *     opaque type;
 *     existential package;
 *     witness representation;
 *     interface dictionary;
 *     erased representation;
 *     extension-defined representation.
 *
 * That representation must remain target-independent until lowering.
 *
 * ============================================================================
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * If the existential body or bounds contain quantum types, semantic lowering
 * may eventually interact with:
 *
 *     quantum::ir
 *
 * but this grammar must never create another quantum IR.
 *
 * For example:
 *
 *     exists Q: quantum::State. Q
 *
 * first becomes a semantic existential type.
 *
 * Only after semantic resolution may its quantum components participate in:
 *
 *     quantum::ir
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     resilience/QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime representation is NOT defined by this grammar.
 *
 * A runtime may implement existential values using:
 *
 *     opaque storage;
 *     erased values;
 *     witness tables;
 *     interface objects;
 *     tagged packages;
 *     specialized representations.
 *
 * The selected representation must preserve the semantic contract.
 *
 * No runtime representation may become part of source-level existential type
 * identity.
 *
 * ============================================================================
 * FFI / ABI CONTRACT
 * ============================================================================
 *
 * Existential types crossing an FFI boundary require an explicit ABI policy.
 *
 * They must not silently become ABI-dependent native pointers or layouts.
 *
 * If an existential value crosses:
 *
 *     C;
 *     C++;
 *     Rust;
 *     foreign ABI;
 *     device ABI;
 *     accelerator ABI;
 *
 * the interoperability subsystem must establish an explicit representation.
 *
 * This grammar only represents the source type.
 *
 * ============================================================================
 * MACRO / METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Macros may generate existential syntax.
 *
 * Generated syntax must be parsed and semantically checked exactly like
 * handwritten syntax.
 *
 * Macros MUST NOT:
 *
 *     bypass existential scope checking;
 *     inject unresolved binders;
 *     bypass type bounds;
 *     bypass ownership checking;
 *     bypass effect checking;
 *     bypass capability checking;
 *     bypass resource checking.
 *
 * Reflection and type-level metaprogramming must operate on the canonical
 * semantic type representation.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors owned by this grammar include:
 *
 *     exists .
 *     exists T
 *     exists T,
 *     exists , T. T
 *     exists T, . T
 *     exists T..T
 *     exists T: . T
 *     exists T: Numeric +. T
 *     exists T: Numeric, . T
 *
 * Valid structural examples include:
 *
 *     exists T. T
 *     exists T: Numeric. T
 *     exists T: Numeric + Comparable. T
 *     exists T, U. Pair<T, U>
 *     exists T, U: Relation<T, U>. Pair<T, U>
 *
 * The semantic layer owns errors such as:
 *
 *     unknown bound;
 *     unsatisfied bound;
 *     invalid existential escape;
 *     invalid existential elimination;
 *     captured type parameter;
 *     invalid type equality;
 *     incompatible existential representation;
 *     unavailable capability;
 *     unavailable resource;
 *     invalid quantum realization.
 *
 * These MUST NOT be converted into parser errors merely because semantic
 * analysis rejects them.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This is a new canonical existential syntax owner.
 *
 * Historical/proposed existential forms must not be duplicated here.
 *
 * In particular, this file MUST NOT simultaneously support competing forms
 * such as:
 *
 *     exists<T>. T
 *     existential<T>. T
 *     some<T>. T
 *     opaque<T>. T
 *
 * unless a future language version explicitly standardizes them as distinct
 * constructs.
 *
 * Compatibility aliases belong under:
 *
 *     grammar/compatibility/
 *
 * and must lower to the same canonical semantic construct if retained.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar ExistentialTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 */

/**
 * Existential type:
 *
 *     exists T. Body
 *
 *     exists T: Bound. Body
 *
 *     exists T, U: Bound. Body
 *
 * The `EXISTS` token is a language-level reserved type-system keyword.
 */
existentialType
    : EXISTS
      existentialBinderList
      DOT
      typeExpression
    ;


/*
 * ============================================================================
 * EXISTENTIAL BINDERS
 * ============================================================================
 */

/**
 * One or more ordered existential binders.
 *
 * Examples:
 *
 *     T
 *
 *     T, U
 *
 *     T: Numeric
 *
 *     T: Numeric, U: Comparable
 */
existentialBinderList
    : existentialBinder
      (COMMA existentialBinder)*
    ;


/**
 * One existentially quantified type variable.
 *
 * The identifier is the source-level binder name.
 *
 * The optional bound clause is owned by grammar/types/bounds.g4.
 */
existentialBinder
    : identifier
      typeBoundClause?
    ;


/*
 * ============================================================================
 * INTEGRATION ADAPTER
 * ============================================================================
 *
 * This named adapter gives semantic tooling one stable grammar-level boundary
 * for existential binders without duplicating generic parameter syntax.
 */
existentialBinderType
    : identifier
    ;