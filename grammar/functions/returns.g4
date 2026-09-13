/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/returns.g4
 *
 * Grammar:
 *     Returns
 *
 * Role:
 *     Canonical reusable parser grammar for callable return-type clauses.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     Grammar integration MUST remain compatible with the repository's
 *     safe-Rust policy. No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the concrete syntax of a callable return-type attachment:
 *
 *     -> TypeExpression
 *
 * It is deliberately a small parser delegate.
 *
 * Example:
 *
 *     fn add(a: Int, b: Int) -> Int {
 *         a + b
 *     }
 *
 *     fn measure(q: Qubit) -> Measurement {
 *         ...
 *     }
 *
 *     fn transform<T>(value: T) -> T {
 *         ...
 *     }
 *
 *     fn build() {
 *         ...
 *     }
 *
 * The final example has no return clause. Absence of a return clause is
 * handled by the surrounding callable declaration.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - functionReturnClause;
 *     - the syntactic return-type attachment marker `->`;
 *     - attachment of exactly one canonical typeExpression to that marker.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - type expressions;
 *     - function declarations;
 *     - function names;
 *     - parameters;
 *     - generic parameters;
 *     - where clauses;
 *     - effects;
 *     - contracts;
 *     - function bodies;
 *     - return statements;
 *     - expression syntax;
 *     - type checking;
 *     - type inference;
 *     - overload resolution;
 *     - generic substitution;
 *     - ABI selection;
 *     - calling conventions;
 *     - register allocation;
 *     - memory layout;
 *     - resource allocation;
 *     - hardware selection;
 *     - quantum allocation;
 *     - physical qubit assignment;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL LANGUAGE CONTRACT
 * ============================================================================
 *
 * The canonical syntax specification defines:
 *
 *     FunctionDeclaration ::=
 *         ["async"]
 *         "fn"
 *         IDENT
 *         [GenericParameters]
 *         "(" [ParameterList] ")"
 *         ["->" TypeExpression]
 *         [WhereClause]
 *         BlockExpression ;
 *
 * Therefore the return clause is:
 *
 *     ReturnClause ::= "->" TypeExpression ;
 *
 * This file implements that reusable syntactic component.
 *
 * The return clause is OPTIONAL at the function-declaration level, but the
 * rule itself represents the complete clause and therefore always contains
 * both:
 *
 *     THIN_ARROW
 *     typeExpression
 *
 * The surrounding function grammar decides whether the clause is present.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing establishes only:
 *
 *     `-> TypeExpression`
 *
 * Semantic analysis determines:
 *
 *     - whether the return type exists;
 *     - whether the return type is well-formed;
 *     - whether it is resolvable;
 *     - whether generic parameters are valid;
 *     - whether the body returns compatible values;
 *     - whether every control-flow path satisfies the return contract;
 *     - whether `void`/unit/never semantics apply;
 *     - whether an inferred return type is permitted when the clause is absent;
 *     - whether the function is recursive;
 *     - whether async return semantics are valid;
 *     - whether generator return semantics are valid;
 *     - whether effect/capability constraints are satisfied;
 *     - whether a quantum return type is semantically legal;
 *     - whether a hardware/accelerator return type is supported;
 *     - whether the return type is ABI-compatible with an external function;
 *     - whether the return type can be lowered to a target.
 *
 * None of those semantic decisions belong in this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve the return clause structurally.
 *
 * Conceptually:
 *
 *     ReturnClause {
 *         span,
 *         arrow_span,
 *         type
 *     }
 *
 * The exact Rust AST type is owned by the frontend/AST layer.
 *
 * This grammar MUST NOT define or duplicate that Rust representation.
 *
 * The AST should retain source spans so diagnostics can identify:
 *
 *     - the `->` marker;
 *     - the return type;
 *     - the complete return clause.
 *
 * ============================================================================
 * TYPE SYSTEM CONTRACT
 * ============================================================================
 *
 * `typeExpression` is supplied by the canonical type grammar.
 *
 * This file MUST NOT redefine:
 *
 *     typeExpression
 *     primitiveType
 *     compositeType
 *     genericType
 *     functionType
 *     quantumType
 *     hardwareType
 *     resourceType
 *     referenceType
 *     typePath
 *
 * This prevents a second, incompatible type system from emerging inside
 * function grammar.
 *
 * Any supported type expression may therefore become a return type when the
 * type system and language version permit it.
 *
 * This permits future and cross-domain types such as:
 *
 *     Int
 *     Result<Value, Error>
 *     Vec<T>
 *     Tensor<T, N>
 *     Qubit
 *     QuantumState<T>
 *     Circuit<T>
 *     HardwareBuffer<T>
 *     Stream<T>
 *     Future<T>
 *     DistributedValue<T>
 *
 * without modifying this grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A return type describes the semantic result of a computation.
 *
 * It MUST NOT describe how that result is physically represented.
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     CPU registers
 *     GPU registers
 *     SIMD width
 *     memory addresses
 *     memory capacity
 *     device identifiers
 *     QPU identifiers
 *     physical qubits
 *     quantum topology
 *     FPGA resources
 *     ASIC resources
 *     node identifiers
 *     network locations
 *     ABI-specific register classes
 *     calling-convention-specific locations
 *
 * For example:
 *
 *     -> Qubit
 *
 * expresses a source-level type.
 *
 * It does NOT mean:
 *
 *     -> physical qubit 0
 *
 * and:
 *
 *     -> Buffer<T>
 *
 * does not specify a particular memory device or capacity.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO fixed machine/resource limits.
 *
 * It MUST NOT contain:
 *
 *     MAX_RETURN_TYPES
 *     MAX_RETURN_PARAMETERS
 *     MAX_NESTING
 *     MAX_TYPE_DEPTH
 *     MAX_QUANTUM_RETURN_SIZE
 *     MAX_TENSOR_RANK
 *     MAX_HARDWARE_WIDTH
 *     MAX_MEMORY
 *     MAX_QUBITS
 *
 * Recursive and nested type expressions are governed by the canonical type
 * grammar and by explicit compiler/resource policies rather than constants
 * embedded in this file.
 *
 * Example:
 *
 *     -> Result<Vec<Matrix<T>>, Error>
 *
 * must be syntactically representable without this grammar imposing an
 * artificial structural ceiling.
 *
 * If an implementation has a parser-stack, memory, time, or recursion safety
 * limit, that is an implementation/resource policy and MUST NOT be encoded as
 * a language-level grammar limit.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes tokens supplied by the canonical lexer vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * In particular:
 *
 *     THIN_ARROW
 *
 * represents:
 *
 *     ->
 *
 * This file MUST NOT declare lexer rules.
 *
 * The lexer owns token spelling and lexical recognition.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser delegate.
 *
 * The canonical composed parser imports this grammar and supplies the
 * `typeExpression` rule through the canonical Types grammar.
 *
 * Conceptually:
 *
 *     parser grammar Functions;
 *
 *     options {
 *         tokenVocab = ZamaniTokens;
 *     }
 *
 *     import Returns;
 *
 *     functionDeclaration
 *         : ...
 *           functionReturnClause?
 *           ...
 *         ;
 *
 * The exact root grammar may compose the delegates through a different
 * dependency arrangement, but there MUST be one authoritative
 * definition of:
 *
 *     functionReturnClause
 *
 * ============================================================================
 * IMPORT / DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Direct syntactic dependency:
 *
 *     canonical lexer
 *          |
 *          v
 *     THIN_ARROW
 *          |
 *          v
 *     Returns
 *          |
 *          v
 *     function declarations / callable declarations
 *
 * Type dependency:
 *
 *     canonical Types grammar
 *          |
 *          v
 *     typeExpression
 *          |
 *          v
 *     functionReturnClause
 *
 * This is a parser-level composition dependency.
 *
 * It is NOT a dependency on the Rust type-system implementation.
 *
 * ============================================================================
 * NON-CIRCULAR ARCHITECTURE
 * ============================================================================
 *
 * Correct direction:
 *
 *     lexer
 *       |
 *       v
 *     parser delegates
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic/type analysis
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 * classical IR          quantum::ir
 *       |                    |
 *       +---------+----------+
 *                 |
 *                 v
 *        optimization / routing /
 *        scheduling / QEC / ZQN /
 *        hardware realization
 *                 |
 *                 v
 *              runtime
 *
 * This file MUST NOT depend on:
 *
 *     AST implementation
 *     semantic analysis
 *     classical IR
 *     quantum::ir
 *     QEC
 *     ZQN
 *     optimization
 *     routing
 *     scheduling
 *     hardware HAL
 *     runtime
 *
 * In particular:
 *
 *     returns.g4 -> quantum::ir
 *
 * is forbidden.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/functions.g4` currently contains a
 * `functionReturnClause` rule.
 *
 * That duplicate rule MUST be removed from `functions.g4` when this file is
 * integrated.
 *
 * After integration, `functions.g4` should consume:
 *
 *     functionReturnClause?
 *
 * without redefining it.
 *
 * This establishes:
 *
 *     functions.g4
 *         owns function declaration structure
 *
 *     returns.g4
 *         owns return clause syntax
 *
 *     types/*
 *         owns type-expression syntax
 *
 * This separation allows each component to evolve independently.
 *
 * ============================================================================
 * OTHER DECLARATION INTEGRATION
 * ============================================================================
 *
 * Other callable/declaration grammars may consume the same rule.
 *
 * Examples include:
 *
 *     declarations/traits.g4
 *     declarations/implementations.g4
 *     functions/foreign-functions.g4
 *     interoperability/foreign-functions.g4
 *
 * They MUST reuse:
 *
 *     functionReturnClause
 *
 * rather than define variants such as:
 *
 *     traitReturnClause
 *     implementationReturnClause
 *     foreignReturnClause
 *
 * unless a future language specification explicitly establishes genuinely
 * different syntax.
 *
 * Semantic differences between declarations are handled by semantic analysis.
 *
 * ============================================================================
 * DECLARATION VS RETURN STATEMENT
 * ============================================================================
 *
 * This file owns:
 *
 *     fn f() -> Type
 *
 * It does NOT own:
 *
 *     return expression;
 *
 * Return statements belong to:
 *
 *     grammar/statements/returns.g4
 *
 * The distinction is important:
 *
 *     functionReturnClause
 *
 * describes the callable's declared result type.
 *
 *     returnStatement
 *
 * describes control flow from a function body.
 *
 * Semantic analysis connects the two.
 *
 * ============================================================================
 * ABSENCE OF RETURN CLAUSE
 * ============================================================================
 *
 * The following is syntactically valid when the surrounding function grammar
 * permits it:
 *
 *     fn work() {
 *         ...
 *     }
 *
 * This grammar does not create an explicit token or AST node for the absence
 * of a return clause.
 *
 * The surrounding function declaration represents the optionality:
 *
 *     functionReturnClause?
 *
 * Semantic analysis decides whether omission means:
 *
 *     - inferred return type;
 *     - unit/void;
 *     - context-dependent return type;
 *     - generator-specific semantics;
 *     - another language-defined rule.
 *
 * The grammar MUST NOT silently assign a semantic return type.
 *
 * ============================================================================
 * VOID / UNIT / NEVER
 * ============================================================================
 *
 * Types such as:
 *
 *     void
 *     never
 *
 * are lexical/type-system concepts.
 *
 * This file does not special-case them.
 *
 * Therefore:
 *
 *     fn f() -> void { ... }
 *
 * and:
 *
 *     fn fail() -> never { ... }
 *
 * are handled through the canonical `typeExpression` rule.
 *
 * Their semantics belong to the type/control-flow systems.
 *
 * This prevents return grammar from becoming coupled to one particular
 * representation of absence or non-returning computation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum return types are syntactically ordinary type expressions.
 *
 * Examples may include:
 *
 *     fn prepare() -> Qubit { ... }
 *
 *     fn measure(q: Qubit) -> Measurement { ... }
 *
 *     fn execute() -> QuantumState<T> { ... }
 *
 *     fn build() -> Circuit<Operation> { ... }
 *
 * This grammar does NOT determine:
 *
 *     - physical qubit assignment;
 *     - logical-to-physical mapping;
 *     - gate availability;
 *     - topology;
 *     - calibration;
 *     - noise;
 *     - fidelity;
 *     - QEC;
 *     - ZQN;
 *     - scheduling;
 *     - execution provider.
 *
 * Those concerns remain downstream.
 *
 * The canonical `quantum::ir` remains the semantic quantum boundary.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Return types may represent values associated with:
 *
 *     classical computation
 *     numerical computation
 *     tensor computation
 *     accelerator computation
 *     HDL/software interfaces
 *     hardware abstractions
 *     distributed computation
 *     networking
 *     cryptography
 *     AI/ML
 *     future domains
 *
 * This grammar remains domain-neutral because all such meanings are expressed
 * through the canonical type system.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic return types are naturally supported through `typeExpression`.
 *
 * Examples:
 *
 *     fn identity<T>(x: T) -> T {
 *         x
 *     }
 *
 *     fn make<T>() -> Option<T> {
 *         ...
 *     }
 *
 *     fn transform<T, U>(x: T) -> Result<U, Error> {
 *         ...
 *     }
 *
 * Generic declaration syntax belongs to:
 *
 *     grammar/functions/generics.g4
 *
 * or the appropriate canonical generic-declaration grammar.
 *
 * This file only consumes the resulting type expression.
 *
 * ============================================================================
 * FUNCTION-TYPE INTEGRATION
 * ============================================================================
 *
 * A callable may return another callable where the canonical type grammar
 * permits function types.
 *
 * Example:
 *
 *     fn factory() -> fn(Int) -> Int {
 *         ...
 *     }
 *
 * The exact ambiguity and nesting rules are owned by:
 *
 *     grammar/types/function-types.g4
 *
 * This file MUST NOT duplicate function-type syntax.
 *
 * ============================================================================
 * FOREIGN / FFI INTEGRATION
 * ============================================================================
 *
 * Foreign functions may use:
 *
 *     functionReturnClause
 *
 * but ABI compatibility is not a grammar concern.
 *
 * For example:
 *
 *     extern fn external_value() -> ExternalType;
 *
 * The grammar establishes only the source syntax.
 *
 * Semantic/interoperability layers determine:
 *
 *     - ABI;
 *     - calling convention;
 *     - representation;
 *     - ownership;
 *     - layout;
 *     - safety;
 *     - target compatibility.
 *
 * ============================================================================
 * ASYNC INTEGRATION
 * ============================================================================
 *
 * `async` is not owned by this file.
 *
 * An async function may still use:
 *
 *     functionReturnClause
 *
 * The semantic/runtime layer determines whether:
 *
 *     -> T
 *
 * means a direct T, future-like result, task result, or another language-level
 * asynchronous semantic representation.
 *
 * This grammar does not rewrite the return type.
 *
 * ============================================================================
 * GENERATOR INTEGRATION
 * ============================================================================
 *
 * Generator syntax is owned elsewhere.
 *
 * A generator may have a return clause if the language version permits it.
 *
 * The semantic generator model determines the distinction between:
 *
 *     yielded values
 *
 * and:
 *
 *     final function result.
 *
 * This grammar remains unaware of that distinction.
 *
 * ============================================================================
 * EFFECT / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Effects and capabilities are separate syntax/semantic concerns.
 *
 * Conceptually:
 *
 *     fn compute() -> Result
 *         effect ...
 *         ...
 *
 * The return grammar ends at `typeExpression`.
 *
 * It MUST NOT consume or define effect syntax.
 *
 * This prevents:
 *
 *     returns.g4 -> effects.g4
 *
 * from becoming a hard dependency when the function declaration can instead
 * compose both independently.
 *
 * ============================================================================
 * RESOURCE / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Return types must never silently become resource requirements.
 *
 * For example:
 *
 *     -> Qubit
 *
 * does not mean:
 *
 *     allocate a physical qubit now.
 *
 * Likewise:
 *
 *     -> Tensor<Float, N>
 *
 * does not imply:
 *
 *     - a particular accelerator;
 *     - a fixed memory size;
 *     - a fixed vector width;
 *     - a fixed number of processing elements.
 *
 * Resource requirements are established by the resource/capability/semantic
 * layers.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * The grammar must permit deterministic diagnostics for malformed clauses.
 *
 * Invalid examples include:
 *
 *     fn f() -> { ... }
 *
 *     fn f() -> ;
 *
 *     fn f() -> ,
 *
 *     fn f() -> ) { ... }
 *
 *     fn f() -> -> Int { ... }
 *
 *     fn f() -> Int Float { ... }
 *
 * The parser/frontend diagnostic layer owns:
 *
 *     - source spans;
 *     - error codes;
 *     - diagnostic severity;
 *     - human-readable messages;
 *     - recovery behavior;
 *     - localization;
 *     - machine-readable diagnostic output.
 *
 * This grammar must not silently:
 *
 *     - invent a missing type;
 *     - discard `->`;
 *     - consume an unrelated expression as the type;
 *     - convert malformed syntax into a different declaration.
 *
 * ============================================================================
 * ERROR RECOVERY CONTRACT
 * ============================================================================
 *
 * ANTLR's parser recovery may be used by the frontend, but recovery MUST NOT
 * alter the canonical AST as though malformed source were valid source.
 *
 * A recovered parse must retain enough source information for the frontend
 * diagnostics layer to report the original syntax error.
 *
 * Error recovery policy belongs to the parser/frontend infrastructure rather
 * than this small delegate grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical:
 *
 *     source bytes
 *     language version
 *     lexer configuration
 *     parser configuration
 *
 * this rule must produce the same parse structure.
 *
 * It must not depend on:
 *
 *     - hardware;
 *     - runtime device state;
 *     - network state;
 *     - scheduler state;
 *     - calibration;
 *     - random numbers;
 *     - environment variables.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical spelling:
 *
 *     ->
 *
 * is represented by:
 *
 *     THIN_ARROW
 *
 * If a future language version introduces another return syntax, it MUST NOT
 * be silently added here.
 *
 * The language-version specification must first define:
 *
 *     - the new syntax;
 *     - ambiguity behavior;
 *     - AST representation;
 *     - semantic meaning;
 *     - diagnostics;
 *     - compatibility policy;
 *     - migration rules;
 *     - tests.
 *
 * Only then may this grammar be extended.
 *
 * Existing syntax:
 *
 *     -> TypeExpression
 *
 * must remain stable unless an explicit language-version change says otherwise.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must contain:
 *
 *     NO machine-size constants
 *     NO resource-count constants
 *     NO qubit limits
 *     NO CPU limits
 *     NO GPU limits
 *     NO FPGA limits
 *     NO memory limits
 *     NO topology assumptions
 *     NO device identifiers
 *     NO vendor identifiers
 *     NO ABI assumptions
 *     NO runtime dependencies
 *
 * The only fixed syntax in this file is the language-semantic punctuation:
 *
 *     ->
 *
 * That is a genuine language syntax requirement, not hardware hard-coding.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover at least:
 *
 *     fn f() -> Int { ... }
 *     fn f() -> Float { ... }
 *     fn f() -> Bool { ... }
 *     fn f() -> Result<Value, Error> { ... }
 *     fn f<T>() -> T { ... }
 *     fn f() -> Vec<T> { ... }
 *     fn f() -> Qubit { ... }
 *     fn f() -> QuantumState<T> { ... }
 *     fn f() -> Tensor<Float, N> { ... }
 *
 * Nested type expressions must be tested where supported:
 *
 *     -> Result<Vec<Option<T>>, Error>
 *
 * Negative tests MUST cover:
 *
 *     missing type after ->
 *     duplicate ->
 *     malformed type expression
 *     invalid delimiter after return type
 *     return clause in an invalid syntactic position
 *
 * Boundary tests MUST cover:
 *
 *     shortest valid return clause;
 *     deeply nested supported type syntax;
 *     large generic type expressions;
 *     long qualified type paths;
 *     many source declarations using return clauses.
 *
 * Scalability tests MUST verify that no artificial return-type count,
 * machine-size, qubit-count, or hardware-size restriction originates here.
 *
 * Cross-domain tests MUST include return types used by:
 *
 *     classical functions;
 *     quantum functions;
 *     hybrid functions;
 *     HDL/hardware interfaces;
 *     distributed functions;
 *     accelerator functions;
 *     AI/data functions.
 *
 * Determinism tests must parse identical source identically.
 *
 * Round-trip tests must preserve:
 *
 *     ->
 *
 * and the complete type-expression structure.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Language servers, formatters, syntax highlighters, documentation tools,
 * source analyzers, and refactoring tools should identify:
 *
 *     functionReturnClause
 *
 * as a structural parser node.
 *
 * Tooling must obtain semantic type information from the AST/type system,
 * not from assumptions embedded in this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * `grammar/functions/returns.g4` is COMPLETE only when:
 *
 * 1. It is a parser-only delegate grammar.
 *
 * 2. It uses the canonical lexer vocabulary.
 *
 * 3. It defines exactly one owned return-clause rule:
 *
 *        functionReturnClause
 *
 * 4. The rule is:
 *
 *        THIN_ARROW typeExpression
 *
 * 5. It does not redefine `typeExpression`.
 *
 * 6. It does not redefine lexer tokens.
 *
 * 7. It does not depend on AST, IR, runtime, hardware, QEC, or ZQN.
 *
 * 8. `functions/functions.g4` consumes this rule rather than redefining it.
 *
 * 9. Other callable declaration grammars can reuse this rule.
 *
 * 10. Optionality remains owned by the enclosing callable declaration.
 *
 * 11. No hardware/resource limits are encoded.
 *
 * 12. No artificial nesting or arity limits are encoded.
 *
 * 13. Diagnostics remain deterministic.
 *
 * 14. The rule is compatible with the canonical syntax specification.
 *
 * 15. Positive, negative, boundary, scalability, cross-domain, determinism,
 *     and round-trip tests exist.
 *
 * 16. The generated parser remains compatible with Rust 1.97 / 1.97.1
 *     integration and the repository's no-unsafe requirement.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * One source-level callable result:
 *
 *     -> TypeExpression
 *
 * has one syntactic meaning.
 *
 * Its physical realization may vary across:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     embedded system
 *     cluster
 *     supercomputer
 *     QPU
 *     simulator
 *     distributed system
 *     future computational substrate
 *
 * without changing this grammar.
 *
 * The grammar describes the program.
 *
 * The type system determines meaning.
 *
 * The IR represents canonical semantics.
 *
 * The compiler determines realization.
 *
 * The runtime supplies available resources.
 *
 * ============================================================================
 */

parser grammar Returns;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * CANONICAL FUNCTION RETURN CLAUSE
 * ========================================================================== */

/**
 * Function return-type attachment.
 *
 * Canonical form:
 *
 *     -> TypeExpression
 *
 * Examples:
 *
 *     -> Int
 *     -> Result<Value, Error>
 *     -> Qubit
 *     -> Tensor<Float, N>
 *
 * The surrounding callable declaration owns optionality:
 *
 *     functionReturnClause?
 *
 * This rule itself always requires both the arrow and the type expression.
 */
functionReturnClause
    : THIN_ARROW
      typeExpression
    ;