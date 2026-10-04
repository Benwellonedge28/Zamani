/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/generic.g4
 *
 * Grammar:
 *     Generic
 *
 * Status:
 *     CANONICAL GENERIC TYPE-APPLICATION COMPONENT
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the reusable source-level syntax required for applying a
 * generic type constructor to an ordered collection of type arguments.
 *
 * Examples:
 *
 *     Vec<T>
 *     Map<Key, Value>
 *     Result<Value, Error>
 *     Option<T>
 *     Matrix<T>
 *     Register<Qubit>
 *     QuantumState<State>
 *     Buffer<Packet>
 *     Container<Vec<Value>>
 *
 * The generic application syntax is deliberately domain-neutral.
 *
 * Generic application may therefore be used by:
 *
 *     classical
 *     numerical
 *     scientific
 *     AI / ML
 *     data
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     distributed
 *     networking
 *     cryptographic
 *     embedded
 *     future computational domains
 *
 * This grammar does not know what a particular generic constructor means.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The source pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Types
 *       |
 *       v
 *     Generic
 *       |
 *       v
 *     TypeExpr
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type resolution
 *       |
 *       +-----------------------+-----------------------+
 *       |                       |                       |
 *       v                       v                       v
 *   classical              quantum::ir            HDL/hardware
 *   semantics               semantics              semantics
 *       |                       |                       |
 *       +-----------------------+-----------------------+
 *                               |
 *                               v
 *                       canonical semantic IR
 *                               |
 *                       optimization/lowering
 *                               |
 *                       routing/scheduling
 *                               |
 *                       resilience/QEC
 *                               |
 *                              ZQN
 *                               |
 *                              HAL
 *                               |
 *                       target realization
 *
 * Generic syntax is entirely before target realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - generic type-application delimiters;
 *     - generic type-application argument-list syntax;
 *     - generic application argument ordering;
 *     - trailing-comma syntax;
 *     - the reusable generic-application suffix/component;
 *     - syntactic distinction between an application and an empty angle list.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - identifiers;
 *     - typeExpression;
 *     - typeCore;
 *     - primitive types;
 *     - named types;
 *     - type paths;
 *     - tuple types;
 *     - arrays;
 *     - slices;
 *     - references;
 *     - pointers;
 *     - function types;
 *     - optional types;
 *     - Result types;
 *     - quantum types;
 *     - HDL types;
 *     - hardware types;
 *     - resource types;
 *     - dependent/value types;
 *     - generic declarations;
 *     - generic parameter declarations;
 *     - generic bounds;
 *     - where clauses;
 *     - type inference;
 *     - unification;
 *     - substitution;
 *     - specialization;
 *     - monomorphization;
 *     - overload resolution;
 *     - trait/interface resolution;
 *     - constraint solving;
 *     - resource discovery;
 *     - capability negotiation;
 *     - hardware selection;
 *     - quantum allocation;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime representation;
 *     - ABI layout.
 *
 * ============================================================================
 * IMPORTANT ANTLR OWNERSHIP RULE
 * ============================================================================
 *
 * `grammar/types/types.g4` is the public type-composition grammar.
 *
 * It owns:
 *
 *     typeExpression
 *
 * and therefore has visibility of every type category.
 *
 * This file MUST NOT redefine `typeExpression`.
 *
 * It also MUST NOT create a competing complete type grammar merely so that
 * generic arguments can be parsed.
 *
 * The composition relationship is:
 *
 *     Types
 *       |
 *       +--> Generic
 *       |
 *       +--> other type components
 *
 * Therefore `Generic` supplies reusable generic-application productions while
 * `Types` supplies the canonical type-expression context in which generic
 * applications are used.
 *
 * This avoids a circular grammar dependency:
 *
 *     Types -> Generic -> Types
 *
 * which would be architecturally incorrect and difficult for ANTLR to
 * maintain.
 *
 * ============================================================================
 * GENERIC DECLARATION VS APPLICATION
 * ============================================================================
 *
 * This file owns APPLICATION syntax:
 *
 *     Vec<T>
 *     Map<K, V>
 *     Result<T, E>
 *
 * Generic declarations belong to:
 *
 *     grammar/functions/generics.g4
 *
 * Examples of declarations:
 *
 *     <T>
 *     <T extends Numeric>
 *     <T extends Numeric + Ordered>
 *
 * This file MUST NOT define:
 *
 *     functionGenericParameters
 *     functionGenericParameter
 *     functionGenericParameterBounds
 *
 * Those belong to the generic declaration owner.
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * Parser-facing lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical component hierarchy:
 *
 *     grammar/lexer/lexer.g4
 *     grammar/lexer/tokens.g4
 *     grammar/lexer/operators.g4
 *     grammar/lexer/punctuation.g4
 *     grammar/lexer/identifiers.g4
 *     ...
 *
 * This parser grammar defines NO lexer rules.
 *
 * The canonical generic delimiters are:
 *
 *     LESS
 *         <
 *
 *     GREATER
 *         >
 *
 * The following names MUST NOT be introduced here:
 *
 *     LESS_THAN
 *     GREATER_THAN
 *
 * because they would create competing lexical terminology with the current
 * canonical lexer.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The existing frontend generic-type representation is conceptually:
 *
 *     TypeExpr::Generic {
 *         base: Box<TypeExpr>,
 *         arguments: Vec<TypeExpr>,
 *     }
 *
 * Therefore generic application syntax must preserve:
 *
 *     1. the constructor/base type;
 *     2. argument ordering;
 *     3. every argument's source span;
 *     4. the complete nesting structure.
 *
 * This grammar MUST NOT introduce a competing semantic AST such as:
 *
 *     GenericType
 *     GenericArgument
 *     GenericApplication
 *     GenericTypeApplication
 *
 * merely to represent the frontend semantic type.
 *
 * Parser rule names are allowed to use these concepts, but AST lowering must
 * converge on the existing TypeExpr architecture.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "How was this generic application written?"
 *
 * Semantic analysis answers:
 *
 *     "Does the constructor exist?"
 *     "Is it generic?"
 *     "How many parameters does it declare?"
 *     "Are these arguments valid?"
 *     "Do the arguments satisfy declared bounds?"
 *     "Can the application be substituted?"
 *     "Can it be specialized?"
 *
 * For example:
 *
 *     Result<Value, Error>
 *
 * may be syntactically valid even if `Result` is not declared.
 *
 * Unknown constructors are semantic/name-resolution errors, not grammar
 * errors.
 *
 * ============================================================================
 * GENERIC ARITY
 * ============================================================================
 *
 * Generic argument count is NOT bounded by this grammar.
 *
 * Valid examples include:
 *
 *     Container<T>
 *
 *     Container<T, U>
 *
 *     Container<T, U, V>
 *
 *     Container<T, U, V, W, X>
 *
 * and arbitrarily larger source-defined arities.
 *
 * There is deliberately no:
 *
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_GENERIC_ARITY
 *     MAX_TYPE_ARGUMENTS
 *     MAX_GENERIC_DEPTH
 *     MAX_TYPE_DEPTH
 *
 * Practical parser/compiler limits, when necessary for resource protection,
 * belong to explicit implementation policy rather than language semantics.
 *
 * ============================================================================
 * EMPTY APPLICATIONS
 * ============================================================================
 *
 * An empty generic application is syntactically invalid:
 *
 *     Vec<>
 *
 * is rejected.
 *
 * A generic declaration with zero parameters is a different concept and is
 * not an application.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * A trailing comma is supported:
 *
 *     Vec<T,>
 *
 *     Map<K, V,>
 *
 *     Result<T, E,>
 *
 * The trailing comma does NOT represent an additional type argument.
 *
 * The AST therefore contains:
 *
 *     [T]
 *
 * rather than:
 *
 *     [T, Unit]
 *
 * or another artificial placeholder.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Generic applications may be nested:
 *
 *     Vec<Option<T>>
 *
 *     Result<Vec<T>, Error>
 *
 *     Map<Key, Vec<Value>>
 *
 *     Container<Map<Key, Result<Value, Error>>>
 *
 * No semantic nesting ceiling is encoded.
 *
 * The actual parser implementation may enforce an explicit operational
 * resource budget if required for denial-of-service protection, but such a
 * budget is not a language-level type limit.
 *
 * ============================================================================
 * TYPE ARGUMENTS
 * ============================================================================
 *
 * A generic argument is a canonical source-level type expression.
 *
 * Therefore applications may eventually accept:
 *
 *     Vec<T>
 *     Vec<int>
 *     Vec<&T>
 *     Vec<&mut T>
 *     Vec<(A, B)>
 *     Vec<fn(A) -> B>
 *     Vec<Option<T>>
 *     Vec<Result<T, E>>
 *     Vec<QuantumState<T>>
 *     Vec<LogicalQubit>
 *
 * The actual `typeExpression` production remains owned by `Types`.
 *
 * Generic.g4 does not duplicate that production.
 *
 * ============================================================================
 * VALUE / DEPENDENT GENERIC ARGUMENTS
 * ============================================================================
 *
 * This file does NOT silently introduce an ambiguous mixture of:
 *
 *     type arguments
 *
 * and:
 *
 *     arbitrary type-level values
 *
 * merely because dependent types may eventually be supported.
 *
 * Existing `types.g4` has a separate type-level value model.
 *
 * Consequently:
 *
 *     Matrix<T>[Rows, Cols]
 *
 * remains a dependent/value-parameterized type construct.
 *
 * Generic.g4 does not reinterpret:
 *
 *     Matrix<T, N>
 *
 * as a value-parameterized generic unless the canonical frontend type model
 * explicitly defines such arguments.
 *
 * This preserves information and prevents the parser from accepting syntax
 * that the AST cannot represent faithfully.
 *
 * ============================================================================
 * SYMBOLIC SCALE
 * ============================================================================
 *
 * Generic syntax itself is independent of:
 *
 *     memory size;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     ASIC count;
 *     QPU count;
 *     qubit count;
 *     node count;
 *     thread count;
 *     tensor rank;
 *     register width;
 *     network size;
 *     device count.
 *
 * A type such as:
 *
 *     Register<Qubit>
 *
 * does not allocate a register.
 *
 * A type such as:
 *
 *     Tensor<Value>
 *
 * does not determine physical tensor storage.
 *
 * A type such as:
 *
 *     Buffer<Packet>
 *
 * does not select a memory device.
 *
 * Physical realization belongs to semantic/resource/target layers.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Generic types contribute to POCO-REAF by expressing reusable source-level
 * abstractions without binding them to a particular machine realization.
 *
 * The same generic source meaning can therefore participate in:
 *
 *     tiny embedded execution;
 *     CPU execution;
 *     multicore execution;
 *     GPU execution;
 *     FPGA execution;
 *     ASIC realization;
 *     accelerator realization;
 *     quantum execution;
 *     quantum simulation;
 *     HPC execution;
 *     cluster execution;
 *     distributed execution;
 *     cloud execution;
 *     future computational substrates.
 *
 * The grammar does not promise that every target can realize every generic
 * type.
 *
 * Instead:
 *
 *     source meaning
 *          |
 *          v
 *     semantic type
 *          |
 *          v
 *     target/resource feasibility
 *
 * A target that cannot realize the required type or capability must produce a
 * controlled semantic/resource/target diagnostic.
 *
 * It must not require source rewriting merely because the physical target
 * changed.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Generic application is quantum-neutral.
 *
 * Examples:
 *
 *     Register<Qubit>
 *     Register<LogicalQubit>
 *     QuantumState<State>
 *     QuantumOperation<Operation>
 *
 * are source-level type applications.
 *
 * Generic.g4 MUST NOT:
 *
 *     - allocate qubits;
 *     - identify physical qubits;
 *     - select a QPU;
 *     - inspect topology;
 *     - select a gate set;
 *     - perform decomposition;
 *     - perform routing;
 *     - perform scheduling;
 *     - select calibration;
 *     - perform QEC;
 *     - construct ZQN;
 *     - access HAL state.
 *
 * Quantum semantics continue through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Generic application is equally neutral for hardware-oriented types:
 *
 *     Signal<Value>
 *     Register<Value>
 *     Module<Configuration>
 *     Buffer<Data>
 *     Accelerator<Model>
 *
 * The generic grammar does not encode:
 *
 *     bus width;
 *     register count;
 *     clock count;
 *     pipeline depth;
 *     memory capacity;
 *     device count;
 *     physical placement.
 *
 * Those belong to the HDL/hardware/resource semantic layers.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Generic types can represent:
 *
 *     Tensor<Value>
 *     Dataset<Record>
 *     Model<Input, Output>
 *     Distribution<Value>
 *     Probability<Value>
 *     KnowledgeGraph<Node, Edge>
 *
 * No machine-learning algorithm is encoded in the generic grammar.
 *
 * Algorithm selection belongs to operations, libraries, capabilities and
 * semantic analysis.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Generic syntax may contain types whose semantic interpretation involves
 * capabilities or resources:
 *
 *     Resource<T>
 *     Capability<T>
 *     Device<T>
 *     Accelerator<T>
 *
 * However, this grammar does not resolve them.
 *
 * Resource requirements belong to:
 *
 *     grammar/resources/
 *
 * Capability semantics belong to:
 *
 *     grammar/resources/
 *     grammar/security/
 *     semantic analysis
 *
 * Target feasibility belongs downstream.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Generic type application itself introduces no effect.
 *
 * For example:
 *
 *     Network<Packet>
 *
 * does not itself perform network access.
 *
 * Likewise:
 *
 *     Qubit
 *
 * does not itself perform quantum measurement.
 *
 * Effects arise from operations and executable constructs, not merely from
 * the syntactic appearance of a generic type.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Generic applications can occur in declarations governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Generic.g4 does not define those constructs.
 *
 * Validation owns their syntax and semantics.
 *
 * Type checking may use generic substitutions when evaluating contracts.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Generic types can appear inside constructs subject to policies.
 *
 * Policy resolution occurs downstream.
 *
 * Generic.g4 does not:
 *
 *     authorize;
 *     deny;
 *     sandbox;
 *     negotiate;
 *     allocate;
 *     deploy.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Generic application syntax must preserve source spans so that semantic
 * provenance can record:
 *
 *     constructor source;
 *     argument source;
 *     substitutions;
 *     specializations;
 *     generated representations.
 *
 * Generic.g4 itself does not create provenance records.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must preserve spans for:
 *
 *     genericTypeArguments
 *     genericArgumentList
 *     each generic type argument
 *
 * and, where the composed type grammar exposes them:
 *
 *     genericTypeApplication
 *
 * These spans are required for:
 *
 *     diagnostics;
 *     IDE/LSP navigation;
 *     formatting;
 *     refactoring;
 *     provenance;
 *     source maps;
 *     compatibility tooling.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexical configuration;
 *     grammar configuration;
 *
 * generic parsing must produce the same:
 *
 *     argument order;
 *     nesting;
 *     source spans;
 *     parser structure.
 *
 * No generic rule depends on:
 *
 *     time;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     hardware availability;
 *     environment state;
 *     runtime state;
 *     target selection.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors owned by this grammar include:
 *
 *     <>
 *     <, T>
 *     <T, , U>
 *     <T,,>
 *     <T U>
 *     <T,>
 *
 * The final form above is valid when it contains one actual argument:
 *
 *     <T,>
 *
 * Semantic errors remain outside the grammar:
 *
 *     unknown constructor;
 *     wrong declared arity;
 *     duplicate declaration parameters;
 *     unsatisfied bounds;
 *     invalid substitution;
 *     incompatible type argument;
 *     unavailable target representation.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The repository currently contains generic application logic directly in:
 *
 *     grammar/types/types.g4
 *
 * That logic and this reusable component MUST converge on one implementation
 * rather than remaining as competing generic grammars.
 *
 * Migration rule:
 *
 *     Types
 *       |
 *       +--> Generic
 *
 * `types.g4` retains ownership of `typeExpression`.
 *
 * Generic.g4 owns the reusable generic application argument syntax.
 *
 * The migration MUST NOT create:
 *
 *     Types -> Generic -> Types
 *
 * or duplicate `genericArgumentList` in multiple imported grammars.
 *
 * The repository also contains historical references to:
 *
 *     generic-types.g4
 *
 * If such a file exists in a branch/version, it must not become another
 * canonical authority. It should be migrated/deprecated through the
 * compatibility architecture.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical token vocabulary
 *     canonical typeExpression supplied by Types composition
 *
 * EXPORTS:
 *
 *     genericTypeArguments
 *     genericArgumentList
 *     genericTypeArgumentSeparator
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *     future type-composition grammars that explicitly import Generic
 *
 * AST_OWNER:
 *
 *     existing frontend TypeExpr implementation
 *
 * SEMANTIC_OWNER:
 *
 *     type semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic IR/type representation
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/
 *     grammar/types/tests/
 *     repository type-conformance suite
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * The public reusable entry point is:
 *
 *     genericTypeArguments
 *
 * It represents:
 *
 *     < type-expression-list >
 *
 * and deliberately does not include the constructor/base type.
 *
 * This permits the canonical type grammar to retain ownership of:
 *
 *     typePath
 *
 * and:
 *
 *     typeExpression
 *
 * without creating a circular dependency.
 *
 * ============================================================================
 * IMPLEMENTATION NOTE
 * ============================================================================
 *
 * The actual argument production is:
 *
 *     typeExpression
 *
 * and therefore belongs structurally to the composed Types grammar.
 *
 * This file consequently defines the delimiter/list component in terms of the
 * type-expression rule supplied by its composition environment.
 *
 * A grammar integration must compose this file from the type grammar in a
 * direction that makes `typeExpression` available to the imported component.
 *
 * If ANTLR composition rules in the repository do not permit that dependency
 * direction, the repository must retain the equivalent argument-list
 * production in `types.g4` until a lower-level `TypeExpression` grammar is
 * introduced.
 *
 * It is forbidden to solve that limitation by duplicating the complete type
 * system inside this file.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are no grammar-level limits on:
 *
 *     generic argument count;
 *     nesting;
 *     source program count;
 *     declaration count;
 *     type constructor count;
 *     domain count;
 *     target count;
 *     hardware count;
 *     quantum count;
 *     resource count.
 *
 * The grammar uses recursive/repetitive parser constructs rather than finite
 * enumerations.
 *
 * Any implementation resource budget must be:
 *
 *     explicit;
 *     configurable;
 *     diagnosable;
 *     separate from language semantics;
 *     independent of physical target enumeration.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN:
 *
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_GENERIC_ARITY
 *     MAX_GENERIC_DEPTH
 *     MAX_TYPE_DEPTH
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
 * Also forbidden:
 *
 *     vendor-specific type constructors;
 *     fixed hardware inventories;
 *     finite quantum operation inventories;
 *     physical device IDs;
 *     physical topology assumptions.
 *
 * ALLOWED:
 *
 *     language-defined punctuation;
 *     generic delimiters;
 *     source-level type expressions;
 *     explicit compatibility rules;
 *     implementation-level resource policies outside grammar semantics.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * The compiler/frontend consuming it MUST:
 *
 *     - support Rust 1.97;
 *     - support Rust 1.97.1;
 *     - use Rust 2021;
 *     - use safe Rust;
 *     - contain no unsafe blocks;
 *     - contain no unsafe functions;
 *     - preserve source spans;
 *     - preserve deterministic AST construction;
 *     - avoid target-specific parser behavior.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     Vec<T>
 *     Vec<int>
 *     Map<K, V>
 *     Result<T, E>
 *     Vec<T,>
 *     Map<K, V,>
 *     Vec<Option<T>>
 *     Map<K, Result<V, E>>
 *     Register<Qubit>
 *     Register<LogicalQubit>
 *     Tensor<Value>
 *
 * NEGATIVE:
 *
 *     <>
 *     Vec<>
 *     Map<,>
 *     Map<T,, U>
 *     Map<T U>
 *
 * SEMANTIC-ERROR FIXTURES:
 *
 *     Unknown<T>
 *     Result<T>
 *
 * where the declaration environment makes these invalid.
 *
 * The latter must parse and subsequently fail semantic arity checking if
 * `Result` requires two arguments.
 *
 * BOUNDARY:
 *
 *     Vec<(A, B)>
 *     Vec<&T>
 *     Vec<&mut T>
 *     Vec<fn(A) -> B>
 *     Vec<Result<A, B>>
 *     Map<K, Vec<Result<V, E>>>
 *
 * CROSS-DOMAIN:
 *
 *     Register<Qubit>
 *     Tensor<Value>
 *     Signal<Value>
 *     Accelerator<Model>
 *     Buffer<Packet>
 *
 * SCALABILITY:
 *
 *     generated nested generic structures of increasing depth;
 *     generated generic lists of increasing arity;
 *     combinations of nested applications;
 *     combinations across domain-neutral type constructors.
 *
 * DETERMINISM:
 *
 *     identical source produces identical parse structure and source spans.
 *
 * DIAGNOSTICS:
 *
 *     missing closing `>`;
 *     missing argument after comma;
 *     unexpected comma;
 *     malformed nested application.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before marking this component complete:
 *
 * [ ] `LESS` is the canonical `<` token.
 *
 * [ ] `GREATER` is the canonical `>` token.
 *
 * [ ] No `LESS_THAN` token is introduced.
 *
 * [ ] No `GREATER_THAN` token is introduced.
 *
 * [ ] No lexer rule exists in this file.
 *
 * [ ] No generic declaration rules exist in this file.
 *
 * [ ] No generic bound rules exist in this file.
 *
 * [ ] No duplicate `typeExpression` exists in this file.
 *
 * [ ] No duplicate `typePath` exists in this file.
 *
 * [ ] No hardware/resource limits exist in this file.
 *
 * [ ] Generic arity is not artificially bounded.
 *
 * [ ] Generic nesting is not artificially bounded.
 *
 * [ ] Trailing commas are supported.
 *
 * [ ] Empty generic applications are rejected.
 *
 * [ ] Argument order is preserved.
 *
 * [ ] Nested applications are representable.
 *
 * [ ] Source spans are preserved downstream.
 *
 * [ ] Semantic arity checking remains downstream.
 *
 * [ ] Bound checking remains downstream.
 *
 * [ ] Substitution remains downstream.
 *
 * [ ] Specialization remains downstream.
 *
 * [ ] Monomorphization remains downstream.
 *
 * [ ] Quantum semantics remain downstream.
 *
 * [ ] `quantum::ir` remains the canonical quantum IR boundary.
 *
 * [ ] HDL/hardware semantics remain downstream.
 *
 * [ ] Resource/capability resolution remains downstream.
 *
 * [ ] Effects are not fabricated by type syntax.
 *
 * [ ] Policies are not fabricated by type syntax.
 *
 * [ ] Provenance remains a downstream concern.
 *
 * [ ] Rust implementation remains safe Rust.
 *
 * [ ] Rust 1.97/1.97.1 compatibility remains required.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Diagnostics tests exist.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * Generic types are a language-level abstraction.
 *
 * They describe relationships between types.
 *
 * They do not describe physical machines.
 *
 * They do not allocate resources.
 *
 * They do not select targets.
 *
 * They do not encode finite hardware ceilings.
 *
 * They do not create a second type system.
 *
 * They do not create a second quantum IR.
 *
 * They do not create a second generic-declaration system.
 *
 * The invariant is:
 *
 *     generic syntax
 *          |
 *          v
 *     domain-neutral TypeExpr
 *          |
 *          v
 *     semantic type resolution
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     ordinary semantics   quantum semantics
 *                              |
 *                              v
 *                         quantum::ir
 *
 * Generic source meaning remains independent of target size and physical
 * realization.
 *
 * ============================================================================
 */

parser grammar Generic;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * GENERIC TYPE-APPLICATION ARGUMENTS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `typeExpression` is intentionally referenced rather than redefined.
 *
 * The canonical Types composition grammar supplies that rule.
 *
 * This preserves one and only one source-level type-expression authority.
 * ============================================================================
 */

/**
 * Complete generic argument delimiters.
 *
 * Examples:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *     <T,>
 *     <T, U,>
 *
 * Empty lists are rejected because genericArgumentList requires at least one
 * type expression.
 */
genericTypeArguments
    : LESS
      genericArgumentList
      GREATER
    ;


/**
 * Ordered generic type arguments.
 *
 * The final optional comma is syntactic only.
 *
 * Examples:
 *
 *     T
 *     T, U
 *     T, U, V
 *     T,
 *     T, U,
 */
genericArgumentList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      genericTypeArgumentTrailingComma?
    ;


/**
 * Optional trailing comma.
 *
 * Kept as a separate rule so that:
 *
 *     argument count
 *
 * remains:
 *
 *     number of typeExpression occurrences
 *
 * rather than the number of comma-separated tokens.
 */
genericTypeArgumentTrailingComma
    : COMMA
    ;


/*
 * ============================================================================
 * GENERIC APPLICATION SUFFIX
 * ============================================================================
 *
 * This rule is useful to the canonical type-composition grammar when it owns
 * the constructor/base type.
 *
 * Conceptually:
 *
 *     typePath genericTypeApplicationSuffix
 *
 * becomes:
 *
 *     TypeExpr::Generic {
 *         base: typePath,
 *         arguments: [...]
 *     }
 *
 * The base/type-path rule itself remains owned by Types.
 * ============================================================================
 */

genericTypeApplicationSuffix
    : genericTypeArguments
    ;


/*
 * ============================================================================
 * COMPATIBILITY RULE
 * ============================================================================
 *
 * Existing consumers may historically refer to the generic argument
 * production as `genericArguments`.
 *
 * Keep this as a parser-level forwarding rule only.
 *
 * It does not create another semantic representation.
 * ============================================================================
 */

genericArguments
    : genericTypeArguments
    ;