/*
 * ============================================================================
 * Zamani Universal Quantum Grammar
 * ============================================================================
 *
 * File:
 *     grammar/quantum/circuits.g4
 *
 * Role:
 *     Canonical parser fragment for quantum-circuit structure and composition.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - quantum circuit declarations;
 *   - quantum circuit signatures;
 *   - circuit parameters;
 *   - circuit generic parameters;
 *   - circuit bodies;
 *   - circuit-local structural elements;
 *   - circuit composition;
 *   - circuit invocation;
 *   - circuit references;
 *   - circuit specialization syntax;
 *   - circuit instantiation syntax;
 *   - circuit inheritance/composition syntax where supported;
 *   - circuit attributes;
 *   - circuit-level metadata;
 *   - circuit-level semantic annotations;
 *   - circuit-level classical/quantum interface declarations;
 *   - circuit-level input/output declarations;
 *   - circuit-level reusable composition;
 *   - circuit-level recursion syntax where permitted by semantic analysis;
 *   - circuit-level genericity;
 *   - circuit-level symbolic resource requirements.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - paths;
 *   - generic type definitions;
 *   - general expressions;
 *   - general statements;
 *   - quantum gate definitions;
 *   - quantum gate vocabulary;
 *   - quantum operation application;
 *   - controlled-operation semantics;
 *   - parameterized-operation semantics;
 *   - measurement;
 *   - reset;
 *   - observables;
 *   - quantum states;
 *   - QEC algorithms;
 *   - QEC decoders;
 *   - ZQN noise models;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware topology;
 *   - physical allocation;
 *   - calibration;
 *   - backend selection;
 *   - resource discovery;
 *   - runtime dispatch;
 *   - simulation;
 *   - quantum::ir implementation.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +---- circuits.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +---- names
 *          +---- types
 *          +---- generics
 *          +---- effects
 *          +---- capabilities
 *          +---- resources
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +---- quantum::ir
 *          |
 *          +---- optimization
 *          +---- routing
 *          +---- scheduling
 *          +---- ZQN
 *          +---- QEC
 *          +---- resilience
 *          +---- hardware HAL
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     execution
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A circuit describes reusable computation.
 *
 * A circuit MUST NOT encode:
 *
 *   - fixed QPU size;
 *   - fixed qubit count;
 *   - fixed register count;
 *   - fixed topology;
 *   - fixed physical qubit IDs;
 *   - fixed device IDs;
 *   - fixed vendor;
 *   - fixed gate durations;
 *   - fixed pulse durations;
 *   - fixed connectivity;
 *   - fixed processor count;
 *   - fixed simulator size;
 *   - fixed backend;
 *   - fixed scheduling decisions.
 *
 * Circuit extent may be symbolic, inferred, parameterized, runtime-provided,
 * or constrained by semantic/resource analysis.
 *
 * ============================================================================
 * IMPORTANT ANTLR INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is a PARSER FRAGMENT.
 *
 * It intentionally does not declare:
 *
 *     grammar ...
 *
 * and does not declare:
 *
 *     lexer grammar ...
 *
 * It is composed into the canonical Zamani parser.
 *
 * Shared rules/tokens expected from the canonical grammar include:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     statement
 *     block
 *     parameterList
 *     genericParameters
 *     whereClause
 *     typeExpr
 *     argumentList
 *     attributes
 *
 * Shared lexical tokens expected include the canonical equivalents of:
 *
 *     K_CIRCUIT
 *     K_CALL
 *     K_INSTANTIATE
 *     K_COMPOSE
 *     K_INPUT
 *     K_OUTPUT
 *     K_INOUT
 *     K_AS
 *     K_WITH
 *     K_WHERE
 *
 * plus the canonical punctuation tokens.
 *
 * If the lexer currently represents these as literal keywords rather than
 * K_* tokens, the canonical lexer must establish the tokens centrally.
 *
 * No circuit-specific lexer is permitted.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. CIRCUIT DECLARATION
 * ========================================================================== */

/*
 * Canonical forms:
 *
 *     circuit Bell {
 *         ...
 *     }
 *
 *     circuit Bell(a: Qubit, b: Qubit) {
 *         ...
 *     }
 *
 *     circuit QFT<T>(register: QubitRegister<T>) {
 *         ...
 *     }
 *
 * The declaration does not allocate hardware.
 */
quantumCircuitDeclaration
    : K_CIRCUIT
      identifier
      genericParameters?
      quantumCircuitParameterList?
      quantumCircuitInterface?
      whereClause?
      quantumCircuitBody
    ;


/* ============================================================================
 * 2. CIRCUIT PARAMETER LIST
 * ========================================================================== */

/*
 * Circuit parameters are semantic parameters.
 *
 * They may describe:
 *
 *     qubits
 *     registers
 *     classical values
 *     angles
 *     symbolic values
 *     resource expressions
 *     configuration values
 *
 * No fixed machine size is implied.
 */
quantumCircuitParameterList
    : LPAREN parameterList? RPAREN
    ;


/* ============================================================================
 * 3. CIRCUIT INTERFACE
 * ========================================================================== */

/*
 * Circuit interfaces explicitly distinguish quantum inputs/outputs from
 * classical inputs/outputs.
 *
 * Example:
 *
 *     circuit Teleport(
 *         input source: Qubit,
 *         input target: Qubit,
 *         output result: Bit
 *     ) {
 *         ...
 *     }
 *
 * The exact semantic types are resolved by the type system.
 */
quantumCircuitInterface
    : quantumCircuitInterfaceItem+
    ;


quantumCircuitInterfaceItem
    : quantumCircuitInput
    | quantumCircuitOutput
    | quantumCircuitInOut
    ;


quantumCircuitInput
    : K_INPUT
      identifier
      COLON
      typeExpr
    ;


quantumCircuitOutput
    : K_OUTPUT
      identifier
      COLON
      typeExpr
    ;


quantumCircuitInOut
    : K_INOUT
      identifier
      COLON
      typeExpr
    ;


/* ============================================================================
 * 4. CIRCUIT BODY
 * ========================================================================== */

/*
 * A circuit body is structurally a block of circuit elements.
 *
 * The body does not itself define the semantics of operations.
 *
 * Operation syntax is delegated to:
 *
 *     operations.g4
 *
 * Gate definitions are delegated to:
 *
 *     gates.g4
 *
 * Measurements are delegated to:
 *
 *     measurement.g4
 *
 * Reset is delegated to:
 *
 *     reset.g4
 *
 * Controlled operations are delegated to:
 *
 *     controlled-operations.g4
 */
quantumCircuitBody
    : LBRACE
      quantumCircuitElement*
      RBRACE
    ;


quantumCircuitElement
    : quantumCircuitLocalDeclaration
    | quantumCircuitInvocation
    | quantumCircuitComposition
    | quantumCircuitReturn
    | quantumCircuitControl
    | quantumCircuitAnnotation
    | quantumCircuitResourceClause
    | quantumCircuitStatement
    ;


/* ============================================================================
 * 5. CIRCUIT-LOCAL DECLARATIONS
 * ========================================================================== */

/*
 * Local declarations are deliberately generic.
 *
 * The actual quantum declaration forms belong to their respective grammar
 * fragments.
 *
 * This prevents circuits.g4 from becoming a second quantum type system.
 */
quantumCircuitLocalDeclaration
    : quantumCircuitDeclarationReference
    | quantumCircuitClassicalDeclaration
    ;


quantumCircuitDeclarationReference
    : declaration
    ;


quantumCircuitClassicalDeclaration
    : statement
    ;


/* ============================================================================
 * 6. CIRCUIT INVOCATION
 * ========================================================================== */

/*
 * A circuit can be reused as a semantic computation.
 *
 * Example:
 *
 *     call Bell(q0, q1);
 *
 * The target machine is not selected here.
 */
quantumCircuitInvocation
    : K_CALL
      quantumCircuitReference
      quantumCircuitArgumentList?
      quantumCircuitBinding?
      SEMI
    ;


quantumCircuitReference
    : qualifiedName
    ;


quantumCircuitArgumentList
    : LPAREN
      argumentList?
      RPAREN
    ;


quantumCircuitBinding
    : K_AS
      identifier
    ;


/* ============================================================================
 * 7. CIRCUIT COMPOSITION
 * ========================================================================== */

/*
 * Composition expresses reusable circuit structure.
 *
 * Example:
 *
 *     compose {
 *         call Preparation(q);
 *         call Algorithm(q);
 *         call MeasurementStage(q);
 *     }
 *
 * The composition is semantic.
 *
 * Routing and scheduling happen later.
 */
quantumCircuitComposition
    : K_COMPOSE
      LBRACE
      quantumCircuitCompositionElement*
      RBRACE
    ;


quantumCircuitCompositionElement
    : quantumCircuitInvocation
    | quantumCircuitComposition
    | quantumCircuitStatement
    ;


/* ============================================================================
 * 8. CIRCUIT RETURN
 * ========================================================================== */

/*
 * Circuit return describes semantic outputs.
 *
 * It does not imply serialization of arbitrary quantum state.
 *
 * This distinction is important for POCO-REAF and for compatibility with
 * physical quantum execution.
 */
quantumCircuitReturn
    : K_RETURN
      quantumCircuitReturnValue?
      SEMI
    ;


quantumCircuitReturnValue
    : expression
    ;


/* ============================================================================
 * 9. CIRCUIT CONTROL
 * ========================================================================== */

/*
 * Circuit-level classical control may contain:
 *
 *     if
 *     loops
 *     match
 *     dynamic execution
 *
 * The detailed control-flow grammar remains owned by the general statement
 * and control-flow grammar.
 */
quantumCircuitControl
    : quantumCircuitConditional
    | quantumCircuitLoop
    ;


quantumCircuitConditional
    : K_IF
      expression
      quantumCircuitBody
      (K_ELSE quantumCircuitBody)?
    ;


quantumCircuitLoop
    : K_FOR
      identifier
      K_IN
      expression
      quantumCircuitBody
    ;


/* ============================================================================
 * 10. CIRCUIT STATEMENT EXTENSION POINT
 * ========================================================================== */

/*
 * This rule is intentionally narrow.
 *
 * Quantum operations must be introduced through operations.g4 rather than
 * duplicated here.
 */
quantumCircuitStatement
    : statement
    ;


/* ============================================================================
 * 11. CIRCUIT ANNOTATIONS
 * ========================================================================== */

/*
 * Circuit-level annotations carry metadata/intent.
 *
 * They do not directly select a backend.
 */
quantumCircuitAnnotation
    : attributes+
    ;


/* ============================================================================
 * 12. CIRCUIT RESOURCE INTENT
 * ========================================================================== */

/*
 * Circuit-level requirements may express semantic resource needs.
 *
 * Examples:
 *
 *     resources {
 *         requires quantum;
 *         requires capability;
 *     }
 *
 * These declarations are interpreted later by resource/capability analysis.
 *
 * They MUST NOT encode physical allocation.
 */
quantumCircuitResourceClause
    : K_WITH
      K_RESOURCES
      LBRACE
      quantumCircuitResourceItem*
      RBRACE
    ;


quantumCircuitResourceItem
    : quantumCircuitRequirement
    | quantumCircuitConstraint
    | quantumCircuitPreference
    | quantumCircuitHint
    ;


quantumCircuitRequirement
    : K_REQUIRES
      expression
      SEMI
    ;


quantumCircuitConstraint
    : K_CONSTRAINT
      expression
      SEMI
    ;


quantumCircuitPreference
    : K_PREFER
      expression
      SEMI
    ;


quantumCircuitHint
    : K_HINT
      expression
      SEMI
    ;


/* ============================================================================
 * 13. CIRCUIT GENERIC SPECIALIZATION
 * ========================================================================== */

/*
 * Generic circuits allow the same source definition to operate over
 * different semantic types/extents.
 *
 * Example:
 *
 *     circuit Algorithm<N>(q: QubitRegister<N>) {
 *         ...
 *     }
 *
 * N is not a machine maximum.
 */
quantumCircuitSpecialization
    : K_INSTANTIATE
      quantumCircuitReference
      quantumCircuitSpecializationArguments
    ;


quantumCircuitSpecializationArguments
    : LT
      quantumCircuitSpecializationArgument
      (COMMA quantumCircuitSpecializationArgument)*
      GT
    ;


quantumCircuitSpecializationArgument
    : expression
    | typeExpr
    ;


/* ============================================================================
 * 14. CIRCUIT EXPRESSION
 * ========================================================================== */

/*
 * A circuit may be referenced as a first-class semantic value where the
 * type system permits it.
 */
quantumCircuitExpression
    : quantumCircuitReference
    | quantumCircuitSpecialization
    ;


/* ============================================================================
 * 15. CIRCUIT REUSABILITY
 * ========================================================================== */

/*
 * Explicit reusable declaration/reference.
 *
 * This provides a stable syntax boundary for future circuit libraries.
 */
quantumCircuitReferenceExpression
    : quantumCircuitExpression
    ;


/* ============================================================================
 * 16. CIRCUIT COMPOSITION EXPRESSION
 * ========================================================================== */

/*
 * Functional composition:
 *
 *     compose(A, B, C)
 *
 * Composition is semantic.
 *
 * It does not imply a particular physical execution order beyond the
 * semantics established by the composed operations.
 */
quantumCircuitCompositionExpression
    : K_COMPOSE
      LPAREN
      quantumCircuitExpression
      (COMMA quantumCircuitExpression)+
      RPAREN
    ;


/* ============================================================================
 * 17. CIRCUIT REPEAT / ITERATION
 * ========================================================================== */

/*
 * Circuit repetition is expression-based.
 *
 * The count is NOT restricted to a fixed integer literal.
 *
 * Example:
 *
 *     repeat(circuit, n)
 *
 * where n may be:
 *
 *     constant
 *     generic
 *     runtime-derived
 *     symbolic
 *     resource-constrained
 */
quantumCircuitRepeatExpression
    : K_REPEAT
      LPAREN
      quantumCircuitExpression
      COMMA
      expression
      RPAREN
    ;


/* ============================================================================
 * 18. CIRCUIT CONDITIONAL EXPRESSION
 * ========================================================================== */

/*
 * Conditional circuit selection.
 */
quantumCircuitConditionalExpression
    : K_IF
      expression
      quantumCircuitExpression
      (K_ELSE quantumCircuitExpression)?
    ;


/* ============================================================================
 * 19. CIRCUIT PIPELINE / CHAINING
 * ========================================================================== */

/*
 * Circuit chaining provides an explicit structural composition mechanism.
 *
 * Example:
 *
 *     Preparation |> Algorithm |> Readout
 *
 * The operator itself must be defined by the canonical expression grammar.
 *
 * This rule therefore only references expression-level composition where
 * available and does not invent a new lexer token here.
 */
quantumCircuitPipelineExpression
    : quantumCircuitExpression
      (PIPE
       quantumCircuitExpression)+
    ;


/* ============================================================================
 * 20. CIRCUIT METADATA
 * ========================================================================== */

/*
 * Metadata belongs to source-level circuit description.
 *
 * It must not contain hidden machine assumptions.
 */
quantumCircuitMetadata
    : attributes
    ;


/* ============================================================================
 * 21. CIRCUIT IDENTIFICATION
 * ========================================================================== */

/*
 * Circuit identity is semantic/name-based.
 *
 * Device IDs are explicitly forbidden at this layer.
 */
quantumCircuitIdentity
    : quantumCircuitReference
    ;


/* ============================================================================
 * 22. CIRCUIT EXTENSION
 * ========================================================================== */

/*
 * Named circuits may be extended only through semantic language mechanisms.
 *
 * This rule intentionally uses generic inheritance/composition syntax rather
 * than introducing a hardware-specific circuit relationship.
 */
quantumCircuitExtension
    : K_EXTENDS
      quantumCircuitReference
    ;


/* ============================================================================
 * 23. CIRCUIT DECLARATION WITH EXTENSION
 * ========================================================================== */

/*
 * Extended declaration form.
 */
quantumCircuitExtendedDeclaration
    : K_CIRCUIT
      identifier
      genericParameters?
      quantumCircuitParameterList?
      quantumCircuitExtension?
      quantumCircuitInterface?
      whereClause?
      quantumCircuitBody
    ;


/* ============================================================================
 * 24. CIRCUIT SIGNATURE
 * ========================================================================== */

/*
 * Signature is useful to AST/semantic tooling without exposing implementation
 * details.
 */
quantumCircuitSignature
    : quantumCircuitReference
      genericParameters?
      quantumCircuitParameterList?
      quantumCircuitInterface?
    ;


/* ============================================================================
 * 25. CIRCUIT DECLARATION UNION
 * ========================================================================== */

quantumCircuitDeclarationForm
    : quantumCircuitDeclaration
    | quantumCircuitExtendedDeclaration
    ;


/* ============================================================================
 * 26. CIRCUIT MODULE-LEVEL ENTRY
 * ========================================================================== */

/*
 * Canonical quantum module integration point.
 */
quantumCircuit
    : quantumCircuitDeclarationForm
    ;


/* ============================================================================
 * 27. SEMANTICALLY EMPTY / STRUCTURAL CIRCUIT
 * ========================================================================== */

/*
 * Empty circuits are syntactically valid.
 *
 * Semantic validation may reject them where a computation is required.
 *
 * This is deliberate: syntax must not encode semantic policy.
 */
quantumEmptyCircuit
    : K_CIRCUIT
      identifier
      genericParameters?
      quantumCircuitParameterList?
      quantumCircuitInterface?
      quantumCircuitBody
    ;


/* ============================================================================
 * 28. CIRCUIT COMPOSITION LIST
 * ========================================================================== */

quantumCircuitCompositionList
    : quantumCircuitExpression
      (COMMA quantumCircuitExpression)*
    ;


/* ============================================================================
 * 29. CIRCUIT PROGRAM ELEMENT
 * ========================================================================== */

quantumCircuitProgramElement
    : quantumCircuit
    | quantumCircuitInvocation
    | quantumCircuitComposition
    | quantumCircuitStatement
    ;


/* ============================================================================
 * 30. ARCHITECTURAL INVARIANTS
 * ========================================================================== */

/*
 * The following are architectural invariants rather than semantic actions:
 *
 * 1. No fixed circuit width.
 * 2. No fixed circuit depth.
 * 3. No fixed number of operations.
 * 4. No fixed number of circuit parameters.
 * 5. No fixed number of circuit inputs.
 * 6. No fixed number of circuit outputs.
 * 7. No fixed number of nested circuits.
 * 8. No fixed number of composition levels.
 * 9. No fixed number of generic parameters.
 * 10. No fixed machine topology.
 * 11. No fixed device.
 * 12. No fixed physical qubit identifiers.
 * 13. No fixed gate duration.
 * 14. No fixed scheduler.
 * 15. No fixed backend.
 *
 * Any practical parser/resource limitation belongs to infrastructure,
 * configuration, resource policy, or runtime protection—not language semantics.
 */


/* ============================================================================
 * 31. IR BOUNDARY
 * ========================================================================== */

/*
 * The parser must produce AST nodes representing concepts such as:
 *
 *     CircuitDeclaration
 *     CircuitSignature
 *     CircuitParameter
 *     CircuitInterface
 *     CircuitBody
 *     CircuitInvocation
 *     CircuitComposition
 *     CircuitSpecialization
 *     CircuitResourceIntent
 *
 * These AST nodes are NOT quantum::ir.
 *
 * Semantic lowering subsequently converts valid quantum constructs into the
 * canonical quantum::ir representation.
 *
 * circuits.g4 MUST NEVER define:
 *
 *     QuantumCircuit struct
 *     QuantumGate struct
 *     Qubit struct
 *     PhysicalQubit struct
 *     CircuitId implementation
 *     OperationId implementation
 *
 * Those belong to the language frontend/semantic/IR layers.
 */


/* ============================================================================
 * 32. SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * This grammar contains no semantic maximum for:
 *
 *     circuit count
 *     circuit depth
 *     operation count
 *     qubit count
 *     register count
 *     parameter count
 *     nesting depth
 *     composition count
 *     generic arity
 *
 * Resource exhaustion must be handled by:
 *
 *     parser limits
 *     compiler limits
 *     resource policies
 *     runtime policies
 *     cancellation
 *     allocation failure
 *
 * and MUST NOT be represented as arbitrary language maximums.
 */


/* ============================================================================
 * 33. SECURITY CONTRACT
 * ========================================================================== */

/*
 * Circuit grammar must not grant:
 *
 *     filesystem access
 *     network access
 *     device access
 *     backend access
 *     calibration access
 *     shell execution
 *
 * merely by parsing a circuit.
 *
 * Such capabilities belong to explicit effect/capability systems.
 */


/* ============================================================================
 * 34. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Given the same:
 *
 *     source
 *     lexer configuration
 *     grammar version
 *
 * parsing must produce the same parse structure.
 *
 * Circuit grammar must not depend on:
 *
 *     current time
 *     machine topology
 *     available QPU
 *     random hardware selection
 *     calibration state
 *     runtime discovery
 */


/* ============================================================================
 * 35. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Legacy:
 *
 *     quantum circuit IDENTIFIER(...) block
 *
 * must remain representable.
 *
 * Existing circuit syntax from the monolithic Zamani.g4 must be migrated into
 * this canonical circuit grammar rather than silently deleted.
 *
 * Older aliases may be retained by compatibility grammar where required.
 *
 * New syntax must not make existing valid programs ambiguous.
 */


/*
 * END OF circuits.g4
 */