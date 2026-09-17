/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/result.g4
 *
 * Grammar:
 *     Result
 *
 * Status:
 *     CANONICAL Result-type syntax integration grammar.
 *
 * Purpose:
 *     Defines the source-level syntax for Zamani's canonical:
 *
 *         Result<T, E>
 *
 *     type constructor.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * Result<T, E> is a source-level algebraic result type:
 *
 *     success -> T
 *     failure -> E
 *
 * This grammar owns ONLY the syntactic Result type constructor.
 *
 * It does NOT own:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - generic declarations;
 *     - generic application in general;
 *     - expression-level Ok;
 *     - expression-level Err;
 *     - error propagation;
 *     - exception handling;
 *     - pattern matching;
 *     - type inference;
 *     - type checking;
 *     - name resolution;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - resources;
 *     - hardware;
 *     - quantum devices;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - runtime representation;
 *     - ABI representation.
 *
 * ============================================================================
 * CANONICAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     grammar/lexer/tokens.g4
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     resultType
 *       |
 *       v
 *     frontend TypeExpr::Result(ok, error)
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type resolution
 *       |
 *       v
 *     canonical semantic model / ZUIR
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   classical              quantum::ir          HDL/resource
 *   semantics              semantics             semantics
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                     optimization / lowering
 *                              |
 *                     routing / scheduling
 *                              |
 *                         QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                      target realization
 *
 * Result is therefore a language-level type construct, not a runtime-result
 * implementation.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Result<T,E> contains NO machine-specific assumptions.
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_RESULT_SIZE
 *     MAX_ERROR_SIZE
 *     MAX_RESULT_DEPTH
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *
 * A Result may therefore contain source-level types representing:
 *
 *     tiny values
 *     large data structures
 *     symbolic values
 *     quantum values
 *     distributed values
 *     hardware abstractions
 *     AI/ML values
 *     future computational domains
 *
 * Actual resource limitations are implementation policy and belong outside
 * language syntax.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Valid examples include:
 *
 *     Result<int, Error>
 *     Result<String, IoError>
 *     Result<Vec<T>, Error>
 *     Result<QuantumState, QuantumError>
 *     Result<Measurement, MeasurementError>
 *     Result<HardwareModule, SynthesisError>
 *     Result<Model, TrainingError>
 *     Result<Packet, NetworkError>
 *     Result<T, DistributedError>
 *
 * The grammar does not decide what any error means.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The canonical frontend representation is:
 *
 *     TypeExpr::Result(
 *         Box<TypeExpr>,
 *         Box<TypeExpr>
 *     )
 *
 * where:
 *
 *     first child  = successful-value type T
 *     second child = error type E
 *
 * This grammar MUST NOT introduce:
 *
 *     ResultTypeAst
 *     ResultTypeNode
 *     ResultTypeIr
 *     ResultTypeSemantic
 *
 * as competing representations.
 *
 * The existing frontend ResultType is a typed façade over the canonical
 * TypeExpr::Result representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar establishes:
 *
 *     Result<T,E>
 *
 * with exactly two source-level type arguments.
 *
 * It does NOT establish:
 *
 *     - whether T is legal;
 *     - whether E is legal;
 *     - whether E satisfies an Error-like constraint;
 *     - whether T or E is inferred;
 *     - whether Result participates in effects;
 *     - whether Result participates in ownership;
 *     - how Result is represented;
 *     - how Result is lowered;
 *     - how Result is executed.
 *
 * Those decisions belong to semantic analysis and later compiler stages.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The authoritative lexer is:
 *
 *     grammar/lexer/tokens.g4
 *
 * with:
 *
 *     lexer grammar ZamaniTokens;
 *
 * Parser grammars consume that vocabulary using:
 *
 *     options {
 *         tokenVocab = ZamaniTokens;
 *     }
 *
 * This grammar therefore declares NO lexer rules.
 *
 * Required canonical lexical tokens:
 *
 *     K_RESULT
 *     LESS_THAN
 *     GREATER_THAN
 *     COMMA
 *
 * Type arguments are parsed using the canonical typeExpression rule supplied
 * by the composed type grammar.
 *
 * ============================================================================
 * WHY K_RESULT IS REQUIRED
 * ============================================================================
 *
 * `Result` is a language-level standard type constructor and is already
 * referenced by other repository grammar components through K_RESULT.
 *
 * Therefore recognizing:
 *
 *     Result<T,E>
 *
 * through the canonical lexer is preferable to:
 *
 *     IDENTIFIER < ... >
 *
 * followed by parser-side string comparison.
 *
 * This preserves:
 *
 *     - deterministic lexical classification;
 *     - one spelling authority;
 *     - consistent parser tokens;
 *     - compatibility with existing K_RESULT consumers;
 *     - tooling discoverability.
 *
 * K_RESULT does NOT mean that the grammar is hard-coded to a hardware
 * implementation. Result is a language semantic constructor.
 *
 * ============================================================================
 * RESULT ARITY
 * ============================================================================
 *
 * The canonical Result constructor has exactly two type parameters:
 *
 *     Result<T, E>
 *
 *     T = success type
 *     E = error type
 *
 * Therefore the following are syntactically invalid:
 *
 *     Result<>
 *     Result<T>
 *     Result<T,E,F>
 *
 * and:
 *
 *     Result<T,E>
 *
 * is valid.
 *
 * A trailing comma is accepted:
 *
 *     Result<T,E,>
 *
 * because trailing commas are already part of Zamani's generic/type-list
 * syntax conventions.
 *
 * The trailing comma does not represent an additional semantic argument.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * `grammar/types/generic.g4` remains the general generic-type application
 * grammar.
 *
 * It owns:
 *
 *     genericType
 *     genericTypeArguments
 *     genericArgumentList
 *
 * This file does NOT redefine those general-purpose rules.
 *
 * Result is intentionally specialized because its semantic constructor has a
 * fixed two-parameter shape.
 *
 * Therefore:
 *
 *     Result<T,E>
 *
 * is a Result-specific type constructor whose children are ordinary canonical
 * type expressions.
 *
 * Nested generic constructs remain delegated to typeExpression:
 *
 *     Result<Vec<T>, Error>
 *     Result<Map<K,V>, Error>
 *     Result<Result<T,E1>, E2>
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Result nesting is unrestricted by language semantics.
 *
 * Examples:
 *
 *     Result<Result<T,E1>,E2>
 *
 *     Result<T,Result<E1,E2>>
 *
 *     Result<Vec<Result<T,E>>,Error>
 *
 *     Result<Option<Result<T,E>>,Error>
 *
 *     Result<QuantumState<Result<T,E>>,QuantumError>
 *
 * No Result-specific nesting limit is encoded here.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The parser/AST layer must preserve source spans for:
 *
 *     Result
 *     <
 *     T
 *     ,
 *     E
 *     >
 *     complete Result type
 *
 * and for both child type expressions.
 *
 * This permits diagnostics such as:
 *
 *     invalid Result argument
 *     malformed Result type
 *     invalid nested type
 *     invalid type constructor
 *
 * without requiring grammar changes.
 *
 * ============================================================================
 * VALUE-LEVEL INTEGRATION
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     Ok(...)
 *     Err(...)
 *
 * Those are expression/value constructs.
 *
 * The canonical lexer already has Result-related lexical vocabulary including
 * Ok and Err. They remain separate from the Result type grammar.
 *
 * Conceptually:
 *
 *     type:
 *         Result<T,E>
 *
 *     values:
 *         Ok(value)
 *         Err(error)
 *
 * Semantic analysis establishes whether:
 *
 *     Ok(value)
 *
 * is compatible with:
 *
 *     Result<T,E>
 *
 * and likewise for Err.
 *
 * ============================================================================
 * ERROR PROPAGATION
 * ============================================================================
 *
 * This grammar does NOT own error-propagation syntax.
 *
 * In particular, it must not define a `?` operator.
 *
 * `?` already participates in optional-type syntax and expression-level
 * operators elsewhere in the language.
 *
 * Error propagation is an expression/control-flow concern and belongs to the
 * appropriate expression/statement grammar and semantic system.
 *
 * ============================================================================
 * OPTION INTEGRATION
 * ============================================================================
 *
 * Option remains independently owned by the Option/type grammar.
 *
 * These compositions are valid where their component types are valid:
 *
 *     Result<Option<T>,E>
 *
 *     Option<Result<T,E>>
 *
 *     Result<Option<Result<T,E>>,E2>
 *
 * Result does not duplicate Option syntax.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Result may wrap any valid classical source type:
 *
 *     Result<int,Error>
 *     Result<float,NumericError>
 *     Result<String,IoError>
 *     Result<Vec<int>,CollectionError>
 *     Result<Matrix<float>,LinearAlgebraError>
 *
 * This grammar does not determine representation, ABI, calling convention,
 * stack layout, register allocation, or exception implementation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Result is domain-neutral and may wrap quantum source types:
 *
 *     Result<Qubit,QuantumError>
 *     Result<QuantumState,QuantumError>
 *     Result<Measurement,MeasurementError>
 *     Result<Circuit,CompilationError>
 *
 * This grammar does NOT:
 *
 *     - allocate qubits;
 *     - select physical qubits;
 *     - select a QPU;
 *     - inspect topology;
 *     - route;
 *     - schedule;
 *     - perform QEC;
 *     - interpret ZQN;
 *     - select calibration;
 *     - invoke HAL.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Result may wrap hardware/HDL source abstractions:
 *
 *     Result<HardwareModule,HardwareError>
 *     Result<Signal<T>,SignalError>
 *     Result<Pipeline<T>,SynthesisError>
 *
 * The grammar does not determine physical implementation.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Result may wrap distributed computation types:
 *
 *     Result<T,NetworkError>
 *     Result<T,RemoteExecutionError>
 *     Result<T,ServiceError>
 *
 * This grammar does not determine whether an error triggers:
 *
 *     retry
 *     recovery
 *     cancellation
 *     rollback
 *     migration
 *     replication
 *     escalation
 *
 * Those are execution/resilience semantics.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Result<T,E> describes a value type.
 *
 * It does not allocate resources.
 *
 * For example:
 *
 *     Result<Qubit,QuantumError>
 *
 * does NOT request one physical qubit.
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *     semantic resource analysis
 *     compiler
 *     HAL
 *     runtime
 *
 * ============================================================================
 * TYPE-LEVEL VALUES
 * ============================================================================
 *
 * Result's two arguments are ordinary type expressions.
 *
 * Type-level values must therefore be admitted only when the enclosing
 * canonical type system permits them.
 *
 * This grammar does not independently invent:
 *
 *     Result<T,N>
 *
 * as a value-parameterized Result constructor.
 *
 * If a future language version defines such a construct, the canonical
 * dependent/type-value grammar must define it consistently across all generic
 * constructors.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The order is fixed:
 *
 *     Result<T,E>
 *             ^
 *             |
 *             +-- T = success
 *             +-- E = error
 *
 * No unordered collections are introduced.
 *
 * No alternative parse may swap the two branches.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The parser should report malformed Result syntax through the normal parser
 * diagnostic mechanism.
 *
 * Examples:
 *
 *     Result<>
 *     Result<T>
 *     Result<T,E,F>
 *     Result<T,,E>
 *     Result<T E>
 *
 * Semantic diagnostics are reserved for semantic problems such as:
 *
 *     unknown type
 *     invalid generic argument
 *     unresolved Result constructor
 *     invalid error type
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes the canonical AST representation:
 *
 *     TypeExpr::Result(ok,error)
 *
 * It may subsequently perform:
 *
 *     name resolution
 *     type checking
 *     inference
 *     generic substitution
 *     ownership analysis
 *     effect analysis
 *     representation selection
 *     optimization
 *     lowering
 *
 * This grammar performs none of these operations.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * No runtime dependency exists here.
 *
 * A downstream runtime may represent Result as:
 *
 *     tagged value
 *     sum type
 *     control-flow representation
 *     ABI-defined result
 *     optimized representation
 *     another target-specific form
 *
 * The grammar must remain unaware of that choice.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This is ANTLR grammar source and contains no Rust.
 *
 * Generated Zamani compiler code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and:
 *
 *     #![forbid(unsafe_code)]
 *
 * No unsafe implementation is required by this grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Allowed:
 *
 *     exactly two semantic Result branches.
 *
 * Not allowed:
 *
 *     maximum result payload size
 *     maximum error size
 *     maximum nesting depth
 *     maximum program size
 *     maximum number of machines
 *     maximum number of nodes
 *     maximum number of devices
 *     maximum number of qubits
 *     maximum memory
 *     maximum generic depth
 *
 * The two-parameter shape of Result<T,E> is language semantics, not a physical
 * scalability limit.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/types/result-types.g4
 *
 * already establishes the Result-family architectural intent.
 *
 * This file provides the canonical filename requested for modular composition.
 *
 * The old `result-types.g4` MUST NOT remain a competing implementation.
 *
 * Its Result-family documentation/integration contract should be retained
 * during migration, but its parser rule must ultimately resolve to this
 * canonical Result grammar.
 *
 * No unrelated filename should be renamed.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Result<T,E> has one canonical syntax;
 *     [x] exactly two semantic type arguments are required;
 *     [x] trailing comma is supported;
 *     [x] no generic implementation is duplicated unnecessarily;
 *     [x] canonical ZamaniTokens are consumed;
 *     [x] K_RESULT is provided by the canonical lexer;
 *     [x] TypeExpr::Result remains the AST representation;
 *     [x] no second Result AST exists;
 *     [x] no semantic analysis exists in grammar;
 *     [x] no runtime behavior exists in grammar;
 *     [x] no hardware limits exist;
 *     [x] no resource limits exist;
 *     [x] nested Result types are supported;
 *     [x] quantum use remains domain-neutral;
 *     [x] HDL use remains domain-neutral;
 *     [x] distributed use remains domain-neutral;
 *     [x] source spans remain recoverable;
 *     [x] diagnostics have deterministic structure;
 *     [x] Rust integration requires no unsafe code.
 *
 * ============================================================================
 */

parser grammar Result;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * PUBLIC RESULT TYPE
 * ============================================================================
 *
 * Canonical source form:
 *
 *     Result<T,E>
 *
 * Exactly two type expressions are required.
 *
 * The child expressions are owned by the canonical type-expression grammar.
 *
 * This rule intentionally does not introduce a Result-specific child-type
 * grammar.
 */
resultType
    : K_RESULT
      LESS_THAN
      typeExpression
      COMMA
      typeExpression
      COMMA?
      GREATER_THAN
    ;