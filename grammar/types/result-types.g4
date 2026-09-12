/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/result-types.g4
 *
 * Grammar:
 *     ResultTypes
 *
 * Role:
 *     Production-ready integration delegate for Result<T, E> type semantics.
 *
 * ============================================================================
 * IMPORTANT ARCHITECTURAL DECISION
 * ============================================================================
 *
 * Result<T, E> is a GENERIC TYPE APPLICATION.
 *
 * Therefore this file MUST NOT duplicate:
 *
 *     genericType
 *     genericArguments
 *     genericArgumentList
 *     genericArgument
 *     typePath
 *     typeExpression
 *
 * Those constructs are owned by the canonical generic/type grammars.
 *
 * This file exists to provide a stable Result-family integration boundary
 * without creating a second generic-type grammar.
 *
 * Canonical surface form:
 *
 *     Result<T, E>
 *
 * Examples:
 *
 *     Result<int, Error>
 *     Result<string, IoError>
 *     Result<Qubit, QuantumError>
 *     Result<Vec<int>, Error>
 *     Result<Option<T>, E>
 *     Result<Result<T, E>, E2>
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the Result type-family integration boundary;
 *   - the parser-level Result type delegation point;
 *   - documentation/tooling identification of Result-family types;
 *   - compatibility naming for the Result type family;
 *   - the explicit architectural statement that Result<T,E> is generic
 *     application syntax rather than a second special-purpose type grammar.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifier spelling;
 *   - the Result identifier token;
 *   - generic application syntax;
 *   - generic argument delimiters;
 *   - generic argument lists;
 *   - generic type/value argument parsing;
 *   - typeExpression;
 *   - typePath;
 *   - Result declarations;
 *   - Result constructors;
 *   - Ok;
 *   - Err;
 *   - pattern matching;
 *   - error propagation;
 *   - exception handling;
 *   - type inference;
 *   - type checking;
 *   - error-type semantics;
 *   - effect semantics;
 *   - ownership;
 *   - borrowing;
 *   - resource allocation;
 *   - quantum allocation;
 *   - physical qubit selection;
 *   - hardware selection;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - canonical quantum IR;
 *   - classical IR;
 *   - runtime representation;
 *   - ABI representation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniTokens
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     Types.typeExpression
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 * generic-types.g4               other type delegates
 *       |
 *       v
 * generic application
 *       |
 *       v
 * Result<T, E>
 *       |
 *       v
 * frontend TypeExpr
 *       |
 *       v
 * semantic type resolution
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 * classical semantics          quantum/resource semantics
 *       |                             |
 *       +--------------+--------------+
 *                      |
 *                      v
 *                 canonical IR
 *
 * ResultTypes is therefore a type-family integration boundary, NOT a second
 * generic parser.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Result<T, E> describes a semantic value that may represent either successful
 * computation or failure.
 *
 * It does NOT describe:
 *
 *     a particular machine;
 *     a particular operating system;
 *     a particular CPU;
 *     a particular GPU;
 *     a particular QPU;
 *     a particular device;
 *     a fixed memory size;
 *     a fixed number of nodes;
 *     a fixed number of qubits;
 *     a fixed network;
 *     a fixed hardware topology.
 *
 * Result<T,E> therefore remains portable across:
 *
 *     embedded systems
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     simulators
 *     accelerators
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar introduces NO fixed limits for:
 *
 *     generic arity;
 *     type depth;
 *     nesting depth;
 *     error-type complexity;
 *     payload type complexity;
 *     program size;
 *     resource count;
 *     machine size;
 *     hardware size;
 *     device count;
 *     node count;
 *     qubit count.
 *
 * Examples such as:
 *
 *     Result<T, E>
 *
 *     Result<Vec<T>, E>
 *
 *     Result<Result<T, E1>, E2>
 *
 *     Result<QuantumState<T>, QuantumError<E>>
 *
 *     Result<Distributed<Resource<T>>, NetworkError>
 *
 * are not restricted by this grammar.
 *
 * Any parser, compiler, memory, security, or deployment limit must be an
 * explicit implementation policy outside the language grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The authoritative lexer is:
 *
 *     grammar/lexer/tokens.g4
 *
 * with grammar name:
 *
 *     ZamaniTokens
 *
 * This file therefore does NOT declare lexer rules.
 *
 * In particular, this file MUST NOT introduce:
 *
 *     K_RESULT
 *
 * merely to recognize Result<T,E>.
 *
 * `Result` remains a normal source-level name unless the language
 * specification deliberately reserves it in a future compatibility-controlled
 * language-version change.
 *
 * This preserves the distinction between:
 *
 *     lexical identity
 *
 * and:
 *
 *     semantic type-family recognition.
 *
 * ============================================================================
 * RESULT NAME
 * ============================================================================
 *
 * The canonical spelling:
 *
 *     Result
 *
 * is resolved through the normal type-name machinery.
 *
 * Consequently:
 *
 *     Result<T, E>
 *
 * is structurally a generic application whose base type is the named type:
 *
 *     Result
 *
 * Semantic analysis determines whether that name refers to the canonical
 * Result type constructor, a user-defined type, an imported type, an alias,
 * or another legal type-level entity.
 *
 * This grammar deliberately does not perform name resolution.
 *
 * ============================================================================
 * GENERIC-TYPES INTEGRATION
 * ============================================================================
 *
 * `grammar/types/generic-types.g4` is the authoritative owner of:
 *
 *     genericType
 *     genericArguments
 *     genericArgumentList
 *     genericArgument
 *     genericTypeArgument
 *     genericValueArgument
 *
 * Therefore this file MUST NOT reproduce those productions.
 *
 * The canonical representation is:
 *
 *     Result<T, E>
 *          |
 *          v
 *     generic application
 *          |
 *          v
 *     TypeExpr::Generic
 *
 * where the semantic layer can identify the base as the Result constructor.
 *
 * ============================================================================
 * RESULT TYPE INTEGRATION POINT
 * ============================================================================
 *
 * The following rule intentionally delegates the complete type expression
 * back to the canonical type grammar:
 *
 *     resultType
 *
 * This rule is an integration hook rather than an independent parser for
 * Result<T,E>.
 *
 * It MUST only be invoked from a composition point where the enclosing
 * grammar has already established the Result-family context.
 *
 * It MUST NOT be inserted as an unrestricted alternative before genericType
 * in a way that creates ambiguous parsing.
 *
 * ============================================================================
 * TYPE EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The authoritative type grammar owns:
 *
 *     typeExpression
 *
 * This file does not redefine it.
 *
 * Therefore this file cannot and must not independently parse:
 *
 *     Result<T,E>
 *
 * from its complete lexical representation.
 *
 * Instead:
 *
 *     Types
 *       |
 *       +--> generic application
 *                |
 *                +--> Result<T,E>
 *
 * and the Result semantic family is identified downstream.
 *
 * ============================================================================
 * WHY THERE IS NO SPECIAL RESULT SYNTAX
 * ============================================================================
 *
 * A tempting implementation would be:
 *
 *     resultType
 *         : RESULT
 *           LESS_THAN
 *           typeExpression
 *           COMMA
 *           typeExpression
 *           GREATER_THAN
 *         ;
 *
 * This is intentionally NOT used.
 *
 * It would:
 *
 *     1. duplicate generic syntax;
 *     2. require a new RESULT lexer token;
 *     3. potentially break identifiers named Result;
 *     4. create competing generic parsing paths;
 *     5. force Types to know implementation-specific constructors;
 *     6. make future generic constructors harder to add;
 *     7. create unnecessary grammar coupling.
 *
 * GenericTypes already solves the syntactic problem.
 *
 * ResultTypes therefore owns the semantic-family boundary rather than
 * duplicating syntax.
 *
 * ============================================================================
 * RESULT ARITY
 * ============================================================================
 *
 * The canonical Result constructor is conventionally:
 *
 *     Result<T, E>
 *
 * where:
 *
 *     T = successful value type
 *     E = error type
 *
 * The grammar MUST NOT hard-code an arbitrary machine limit on generic arity.
 *
 * Arity validation belongs to semantic analysis.
 *
 * If the canonical Result constructor requires exactly two type parameters,
 * semantic analysis reports an error when source code supplies an invalid
 * number of arguments.
 *
 * This distinction is important:
 *
 *     syntax:
 *         generic application
 *
 *     semantics:
 *         canonical Result constructor requires its defined parameter shape
 *
 * ============================================================================
 * VALUE-LEVEL RESULT INTEGRATION
 * ============================================================================
 *
 * The lexer already provides Result-related value vocabulary such as:
 *
 *     Ok
 *     Err
 *
 * These are value-level constructs.
 *
 * ResultTypes does NOT own their syntax.
 *
 * The expression/constructor grammar owns constructs such as:
 *
 *     Ok(value)
 *     Err(error)
 *
 * and semantic analysis determines whether they are compatible with:
 *
 *     Result<T,E>
 *
 * Therefore:
 *
 *     Result<T,E>
 *
 * and:
 *
 *     Ok(value)
 *     Err(error)
 *
 * remain separate grammar responsibilities.
 *
 * ============================================================================
 * ERROR PROPAGATION
 * ============================================================================
 *
 * ResultTypes does NOT own:
 *
 *     ?
 *
 * as an error-propagation operator.
 *
 * This is especially important because `?` is already used by the optional
 * type grammar as the postfix optional-type marker.
 *
 * Any future Result/error-propagation operator must therefore be assigned
 * explicit expression-level ownership and context-sensitive semantics rather
 * than being silently added here.
 *
 * ============================================================================
 * OPTION INTEGRATION
 * ============================================================================
 *
 * `Option<T>` is owned by generic type application.
 *
 * Postfix:
 *
 *     T?
 *
 * is owned by OptionTypes.
 *
 * ResultTypes does not redefine either form.
 *
 * Examples:
 *
 *     Result<Option<T>, E>
 *
 *     Result<T?, E>
 *
 *     Option<Result<T,E>>
 *
 *     Result<Option<Result<T,E>>, E2>
 *
 * are composed from the existing canonical type grammar.
 *
 * Semantic analysis determines whether each combination is legal.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Result types may be recursively nested through the canonical generic
 * machinery.
 *
 * Examples:
 *
 *     Result<Result<T,E1>,E2>
 *
 *     Result<Vec<Result<T,E>>,E2>
 *
 *     Result<Option<Result<T,E>>,E2>
 *
 *     Result<QuantumState<Result<T,E>>,QuantumError>
 *
 * No Result-specific nesting limit is encoded here.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Result may wrap any semantically valid classical type:
 *
 *     Result<int, Error>
 *     Result<string, Error>
 *     Result<Vec<int>, Error>
 *     Result<Matrix<float>, NumericError>
 *
 * This file does not determine:
 *
 *     representation;
 *     memory layout;
 *     tagging;
 *     ABI;
 *     register usage;
 *     calling convention;
 *     exception mechanism.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Result is domain-neutral.
 *
 * Examples may include:
 *
 *     Result<Qubit, QuantumError>
 *
 *     Result<QuantumState<T>, QuantumError>
 *
 *     Result<Circuit, CompilationError>
 *
 *     Result<Measurement, MeasurementError>
 *
 * The grammar does NOT interpret Result failure as:
 *
 *     qubit loss;
 *     decoherence;
 *     QPU failure;
 *     calibration failure;
 *     routing failure;
 *     scheduling failure;
 *     QEC failure;
 *     ZQN fault;
 *     backend failure.
 *
 * Those meanings belong to their respective semantic/runtime layers.
 *
 * In particular, ResultTypes has NO dependency on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     hardware
 *     resilience.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Result may wrap source-level hardware/HDL abstractions when the semantic type
 * system permits them:
 *
 *     Result<HardwareModule, HardwareError>
 *
 *     Result<Signal<T>, HardwareError>
 *
 *     Result<Pipeline<T>, SynthesisError>
 *
 * The grammar does not interpret Result as a physical hardware failure
 * mechanism.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Result may represent the semantic result of distributed operations:
 *
 *     Result<T, NetworkError>
 *
 *     Result<T, ServiceError>
 *
 *     Result<T, RemoteExecutionError>
 *
 * The grammar does not decide whether an Err value means:
 *
 *     network failure;
 *     node failure;
 *     service failure;
 *     timeout;
 *     cancellation;
 *     retry;
 *     rollback;
 *     resilience action.
 *
 * Those are downstream semantics.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Result<T,E> does not allocate resources.
 *
 * For example:
 *
 *     Result<Qubit, QuantumError>
 *
 * does not request a physical qubit.
 *
 * Resource requirements remain owned by:
 *
 *     resource grammar
 *     semantic resource model
 *     hardware abstraction
 *     compiler
 *     runtime
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parser:
 *
 *     recognizes generic type application syntax.
 *
 * AST:
 *
 *     preserves the canonical generic representation.
 *
 * Semantic analysis:
 *
 *     resolves the base type;
 *     identifies the Result constructor;
 *     validates its parameters;
 *     validates success/error compatibility;
 *     performs inference;
 *     performs conversions;
 *     checks ownership/effects where applicable.
 *
 * This file MUST NOT perform those operations.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Result<T,E> MUST NOT create a new AST representation such as:
 *
 *     ResultTypeNode
 *     ResultTypeAst
 *     ResultTypeIr
 *
 * merely because this file exists.
 *
 * The canonical frontend representation remains the repository's existing
 * TypeExpr representation for generic applications.
 *
 * Conceptually:
 *
 *     Result<T,E>
 *         ->
 *     TypeExpr::Generic {
 *         base: Result,
 *         arguments: [T, E]
 *     }
 *
 * The exact Rust AST construction belongs to the parser/AST builder.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * This grammar has NO direct IR dependency.
 *
 * The pipeline remains:
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
 *     TypeExpr
 *       |
 *       v
 *     semantic type resolution
 *       |
 *       v
 *     canonical IR
 *
 * If T or E eventually contains quantum semantics, those semantics are
 * lowered through the canonical quantum semantic boundary.
 *
 * ResultTypes must never construct quantum IR.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages may interpret Result<T,E> for:
 *
 *     control flow;
 *     error propagation;
 *     optimization;
 *     effect analysis;
 *     resource management;
 *     lowering;
 *     ABI generation;
 *     target code generation.
 *
 * None of these are grammar responsibilities.
 *
 * Compiler policies may impose explicit implementation limits such as:
 *
 *     maximum AST size;
 *     maximum compilation memory;
 *     maximum diagnostic count;
 *     maximum recursion depth.
 *
 * Those policies must remain outside this grammar.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * ResultTypes has no runtime dependency.
 *
 * A Result may be represented differently on:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     embedded system
 *     distributed runtime
 *
 * without changing source syntax.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Formatters and IDE tooling should preserve:
 *
 *     Result<T,E>
 *
 * according to the canonical generic formatting policy.
 *
 * Syntax highlighting should classify:
 *
 *     Result
 *
 * according to ordinary type-name resolution unless the language specification
 * explicitly reserves it.
 *
 * Documentation generators may identify canonical Result-family uses after
 * semantic resolution.
 *
 * Refactoring tools must operate on resolved names rather than assuming every
 * identifier spelled `Result` is necessarily the built-in Result constructor.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This design preserves compatibility with the current lexical model because
 * it does NOT require introducing:
 *
 *     K_RESULT
 *
 * Existing source code using Result as a type name remains lexically ordinary.
 *
 * If a future language version reserves Result as a keyword, that change must
 * occur through:
 *
 *     specification
 *     lexer
 *     compatibility policy
 *     migration documentation
 *     compatibility tests
 *
 * and MUST NOT be introduced implicitly by this file.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_RESULT_ARITY
 *     MAX_RESULT_DEPTH
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_ERROR_TYPES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * No machine-specific identifier, topology, device address, physical resource
 * count, or deployment constraint may appear here.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * This delegate must not use:
 *
 *     runtime state;
 *     hardware discovery;
 *     random values;
 *     timestamps;
 *     external resources;
 *     network state;
 *     backend state
 *
 * to determine syntax.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no executable Rust code and no unsafe code.
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and the repository's no-unsafe policy.
 *
 * The grammar must not introduce unsafe FFI, raw pointers, machine addresses,
 * or target-specific behavior.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * ResultTypes itself requires integration tests rather than duplicated parser
 * implementations.
 *
 * POSITIVE:
 *
 *     Result<int, Error>
 *     Result<string, Error>
 *     Result<T, E>
 *     Result<Vec<int>, Error>
 *     Result<Option<T>, E>
 *     Result<T?, E>
 *     Result<Result<T, E1>, E2>
 *     Result<Qubit, QuantumError>
 *     Result<QuantumState<T>, QuantumError>
 *     Result<HardwareResource, HardwareError>
 *     Result<Remote<T>, NetworkError>
 *
 * NEGATIVE / SEMANTIC:
 *
 *     Result<>
 *     Result<T>
 *     Result<T, E, X>
 *
 * when the canonical Result constructor requires exactly two parameters.
 *
 * IMPORTANT:
 *
 * The parser should distinguish syntactic generic validity from semantic
 * Result-arity validity.
 *
 * For example:
 *
 *     Result<T, E, X>
 *
 * is syntactically a generic application.
 *
 * Whether it is semantically a valid Result constructor application is owned
 * by semantic type checking.
 *
 * This distinction prevents grammar-level special casing.
 *
 * BOUNDARY:
 *
 *     deeply nested Result types;
 *     arbitrarily many unrelated generic applications;
 *     very large symbolic type parameters;
 *     large source programs.
 *
 * No test may encode an artificial language maximum as the grammar contract.
 *
 * CROSS-DOMAIN:
 *
 *     classical + Result
 *     quantum + Result
 *     hardware + Result
 *     distributed + Result
 *     hybrid + Result
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] Result-family ownership is explicitly documented.
 *
 * [ ] Generic application remains exclusively owned by GenericTypes.
 *
 * [ ] No generic grammar is duplicated here.
 *
 * [ ] No Result lexer keyword is introduced accidentally.
 *
 * [ ] No second TypeExpr representation is introduced.
 *
 * [ ] No machine/resource limits exist.
 *
 * [ ] No hardware dependency exists.
 *
 * [ ] No quantum::ir dependency exists.
 *
 * [ ] No QEC/ZQN/routing/scheduling dependency exists.
 *
 * [ ] Semantic Result validation remains outside the grammar.
 *
 * [ ] Result value constructors remain outside this type grammar.
 *
 * [ ] Parser composition is deterministic.
 *
 * [ ] Rust 1.97/1.97.1 integration remains supported.
 *
 * [ ] Generated Rust remains compatible with the repository's no-unsafe
 *     policy.
 *
 * [ ] Positive, negative, boundary, cross-domain and compatibility tests exist.
 *
 * [ ] Documentation identifies GenericTypes as the authoritative syntax owner.
 *
 * [ ] No later grammar file needs to redefine Result<T,E>.
 *
 * ============================================================================
 */

parser grammar ResultTypes;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. RESULT TYPE INTEGRATION DELEGATE
 * ============================================================================
 *
 * This rule is intentionally a forwarding boundary.
 *
 * The complete syntax of Result<T,E> is owned by GenericTypes.
 *
 * This delegate therefore consumes the canonical generic type application
 * rule rather than reproducing generic syntax.
 *
 * The enclosing Types grammar must invoke this rule only where a Result-family
 * integration point is explicitly appropriate.
 *
 * Semantic analysis remains responsible for determining whether the generic
 * application's base name denotes the canonical Result constructor.
 */
resultType
    : genericType
    ;