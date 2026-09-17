/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/generic.g4
 *
 * Status:
 *     CANONICAL generic-type APPLICATION grammar.
 *
 * Purpose:
 *     Defines the source-level syntax for applying a generic type constructor
 *     to an ordered sequence of type arguments.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This grammar owns:
 *
 *   - generic type application;
 *   - generic argument delimiters;
 *   - generic argument ordering;
 *   - generic argument lists;
 *   - trailing-comma syntax;
 *   - nested generic applications;
 *   - generic applications whose constructor is a qualified type path.
 *
 * This grammar does NOT own:
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - type paths;
 *   - primitive types;
 *   - tuple types;
 *   - arrays;
 *   - slices;
 *   - references;
 *   - pointers;
 *   - function types;
 *   - optional types;
 *   - result types;
 *   - quantum types;
 *   - hardware types;
 *   - resource types;
 *   - type-level value expressions;
 *   - generic declarations;
 *   - generic parameter declarations;
 *   - generic bounds;
 *   - trait/interface definitions;
 *   - type inference;
 *   - substitution;
 *   - monomorphization;
 *   - specialization;
 *   - overload resolution;
 *   - resource allocation;
 *   - hardware selection;
 *   - quantum allocation;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - optimization;
 *   - canonical IR;
 *   - runtime representation.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * The canonical frontend AST currently represents generic types as:
 *
 *     TypeExpr::Generic {
 *         base: Box<TypeExpr>,
 *         arguments: Vec<TypeExpr>,
 *     }
 *
 * Therefore this grammar deliberately accepts TYPE ARGUMENTS only.
 *
 * A source construct such as:
 *
 *     Vec<int>
 *     Map<String, Value>
 *     Result<T, E>
 *     quantum::Register<Qubit>
 *     Matrix<Vector<float>>
 *
 * maps naturally to the existing TypeExpr::Generic representation.
 *
 * A type-level value such as:
 *
 *     Matrix<float, N>
 *
 * MUST NOT be accepted by this file until the frontend AST has a first-class
 * representation for type/value generic arguments.
 *
 * The current TypeExpr::Generic representation cannot faithfully represent
 * the distinction between:
 *
 *     Matrix<float, N>
 *
 * and:
 *
 *     Matrix<float, SomeType>
 *
 * without introducing an additional semantic representation.
 *
 * This is intentional.
 *
 * The grammar must never silently accept syntax which the canonical AST cannot
 * represent without loss.
 *
 * Type/value parameters remain an integration point for the future dependent
 * type/value model owned by the type-system architecture.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Generic arity is source-defined.
 *
 * There is deliberately NO:
 *
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_GENERIC_DEPTH
 *     MAX_TYPE_DEPTH
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_DEVICE_COUNT
 *
 * An arbitrary number of generic arguments is represented through recursive
 * parser repetition:
 *
 *     argument (COMMA argument)*
 *
 * Any implementation resource limit belongs to an explicit compiler/parser
 * policy, never to the language grammar.
 *
 * "Scale to infinity" therefore means that the language imposes no arbitrary
 * finite semantic ceiling. Actual compilation remains bounded only by the
 * resources and explicit operational policies of the environment.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Generic types may parameterize:
 *
 *     classical types
 *     quantum types
 *     hybrid types
 *     HDL abstractions
 *     hardware abstractions
 *     accelerator abstractions
 *     distributed abstractions
 *     tensors
 *     data structures
 *     networking abstractions
 *     cryptographic abstractions
 *     AI/ML structures
 *     future computational domains
 *
 * This grammar does not know which domain a generic constructor belongs to.
 *
 * Examples:
 *
 *     Vec<int>
 *     Tensor<float>
 *     Register<Qubit>
 *     Buffer<Packet>
 *     Accelerator<Model>
 *     HardwareModule<Configuration>
 *
 * remain syntactically identical at this layer.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser/AST lowering layer must produce:
 *
 *     TypeExpr::Generic {
 *         base,
 *         arguments,
 *     }
 *
 * where:
 *
 *     base
 *         = canonical TypeExpr representing the generic constructor;
 *
 *     arguments
 *         = ordered Vec<TypeExpr>.
 *
 * This file MUST NOT introduce:
 *
 *     GenericType
 *     GenericArgument
 *     GenericTypeArgument
 *     GenericTypeApplication
 *
 * as competing semantic AST structures.
 *
 * Those names may appear as parser-rule names, but they must not become
 * independent frontend semantic nodes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "How was this generic type application written?"
 *
 * Semantic analysis answers:
 *
 *     "Which generic declaration does this constructor refer to?"
 *     "How many parameters does it declare?"
 *     "Are the supplied arguments valid?"
 *     "Do the supplied types satisfy their bounds?"
 *     "What substitutions are required?"
 *
 * Therefore:
 *
 *     Vec<int>
 *
 * is syntactically valid.
 *
 * Whether Vec exists, whether it accepts one argument, and whether int
 * satisfies its constraints are semantic questions.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Generic syntax is quantum-neutral.
 *
 * Examples:
 *
 *     Register<Qubit>
 *     Register<LogicalQubit>
 *     QuantumState<State>
 *     Circuit<Operation>
 *
 * are source-level type applications.
 *
 * This grammar does NOT:
 *
 *     - allocate qubits;
 *     - select physical qubits;
 *     - select a QPU;
 *     - select a vendor;
 *     - inspect topology;
 *     - select a gate set;
 *     - perform decomposition;
 *     - route;
 *     - schedule;
 *     - perform QEC;
 *     - interpret ZQN;
 *     - select calibration;
 *     - access HAL state.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Generic syntax reaches quantum semantics only after frontend AST and
 * semantic type resolution.
 *
 * ============================================================================
 * HARDWARE / RESOURCE CONTRACT
 * ============================================================================
 *
 * Generic applications MUST NOT encode physical realization.
 *
 * Valid:
 *
 *     Buffer<Packet>
 *     Register<Qubit>
 *     Accelerator<Model>
 *     Memory<Value>
 *
 * Invalid as universal language semantics:
 *
 *     GPU0<...>
 *     QPU42<...>
 *     PhysicalQubit17<...>
 *
 * Concrete placement, device identity, topology, resource availability,
 * scheduling, and deployment belong to downstream resource/target layers.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical authority is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose lexer grammar is:
 *
 *     ZamaniTokens
 *
 * Therefore this file MUST use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * It MUST NOT use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The latter is an obsolete/inconsistent contract in the current repository.
 *
 * ============================================================================
 * TYPE-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The canonical public type entry point remains owned by:
 *
 *     grammar/types/types.g4
 *
 * This file owns the generic APPLICATION production.
 *
 * The type composition layer is responsible for combining:
 *
 *     generic
 *     primitive
 *     named
 *     tuple
 *     array
 *     slice
 *     function
 *     reference
 *     pointer
 *     optional
 *     result
 *     quantum
 *     resource
 *     hardware
 *     future type categories
 *
 * into the single public:
 *
 *     typeExpression
 *
 * contract.
 *
 * ============================================================================
 * GENERIC DECLARATION INTEGRATION
 * ============================================================================
 *
 * This file does NOT parse declarations such as:
 *
 *     type Vec<T> = ...
 *
 * or:
 *
 *     fn map<T>(...)
 *
 * Generic declaration syntax belongs to the declaration/function generic
 * grammars.
 *
 * This file only parses USE/APPLICATION syntax:
 *
 *     Vec<T>
 *     Map<K, V>
 *     Result<T, E>
 *
 * ============================================================================
 * GENERIC BOUNDS
 * ============================================================================
 *
 * Bounds are not part of an application.
 *
 * For example:
 *
 *     T: Numeric
 *
 * belongs to generic declaration/constraint syntax.
 *
 * The application:
 *
 *     Vector<T>
 *
 * contains only the supplied argument.
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * The grammar deliberately rejects:
 *
 *     <>
 *
 * because an application without an argument is not a generic application.
 *
 * It accepts:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *     <T,>
 *     <T, U,>
 *
 * A trailing comma is purely syntactic and does not create an additional
 * semantic argument.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Nested generic applications are naturally supported:
 *
 *     Vec<Option<T>>
 *
 *     Result<Vec<T>, Error>
 *
 *     Map<Key, Vec<Value>>
 *
 *     QuantumContainer<Register<LogicalQubit>>
 *
 * No explicit nesting-depth limit exists in this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Generic arguments preserve source order.
 *
 * The canonical AST uses:
 *
 *     Vec<TypeExpr>
 *
 * and therefore argument ordering is semantically significant.
 *
 * No unordered collection is introduced by this grammar.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The parser/AST layer must preserve source spans for:
 *
 *     genericTypeApplication
 *     genericTypeArguments
 *     genericArgumentList
 *     each generic argument
 *
 * This permits diagnostics such as:
 *
 *     wrong generic arity
 *     invalid type argument
 *     unsatisfied bound
 *     unknown generic constructor
 *
 * without requiring grammar changes.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may later perform:
 *
 *     name resolution
 *     generic parameter binding
 *     constraint checking
 *     substitution
 *     specialization
 *     monomorphization
 *     representation selection
 *     optimization
 *
 * None of those operations occur in this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * This grammar has no runtime dependency.
 *
 * Generic type application may eventually affect:
 *
 *     representation
 *     dispatch
 *     specialization
 *     resource requirements
 *
 * but those are determined after semantic analysis.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/types/generic-types.g4
 *
 * must not remain an independent authority.
 *
 * Its generic application rules should be migrated to this file and the old
 * file retained only as a compatibility/deprecation surface until repository
 * references have been migrated.
 *
 * No filename rename is required.
 *
 * ============================================================================
 */

parser grammar Generic;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. GENERIC TYPE APPLICATION
 * ========================================================================== */

/**
 * Generic type application.
 *
 * Examples:
 *
 *     Vec<int>
 *     Option<T>
 *     Result<Value, Error>
 *     Map<Key, Value>
 *     quantum::Register<Qubit>
 *
 * The constructor itself is represented by the canonical type-path rule in
 * the parent type grammar.
 *
 * `typePath` is intentionally not redefined here.
 *
 * The parent type grammar supplies the canonical type-path production.
 *
 * The rule therefore belongs in the final composed type grammar, where the
 * canonical `typePath` and `typeExpression` rules are visible.
 */
genericType
    : typePath
      genericTypeArguments
    ;


/* ============================================================================
 * 2. GENERIC ARGUMENT DELIMITERS
 * ========================================================================== */

/**
 * Generic application delimiters.
 *
 * Empty applications are rejected.
 *
 * Therefore:
 *
 *     Vec<>
 *
 * is syntactically invalid.
 *
 * while:
 *
 *     Vec<T>
 *
 * is valid.
 */
genericTypeArguments
    : LESS_THAN
      genericArgumentList
      GREATER_THAN
    ;


/* ============================================================================
 * 3. ORDERED GENERIC ARGUMENT LIST
 * ========================================================================== */

/**
 * One or more ordered type arguments.
 *
 * The repetition is unbounded by language semantics.
 *
 * A compiler may have configurable operational resource budgets, but those
 * budgets must not be encoded here as grammar-level cardinality limits.
 */
genericArgumentList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      COMMA?
    ;


/* ============================================================================
 * 4. COMPATIBILITY ALIAS
 * ========================================================================== */

/**
 * Compatibility façade for tooling that historically referred to the generic
 * argument delimiter rule as `genericArguments`.
 *
 * It is intentionally a parser-rule alias rather than another implementation.
 */
genericArguments
    : genericTypeArguments
    ;