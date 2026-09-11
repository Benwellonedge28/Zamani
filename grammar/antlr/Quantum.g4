/**
 * ============================================================================
 * Zamani Quantum Syntax
 * ============================================================================
 *
 * File:
 *     grammar/antlr/Quantum.g4
 *
 * Role:
 *     Reusable parser grammar for Zamani quantum source constructs.
 *
 * Architecture:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          +---- Quantum.g4
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     semantic / type / effect / resource analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +---- optimization
 *          +---- routing
 *          +---- scheduling
 *          +---- ZQN
 *          +---- benchmarking
 *          +---- hardware lowering
 *          +---- simulator lowering
 *          |
 *          v
 *     execution target
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This grammar describes QUANTUM SOURCE INTENT.
 *
 * It MUST NOT encode:
 *
 *   - a maximum number of qubits;
 *   - a maximum number of classical bits;
 *   - a fixed register width;
 *   - a fixed gate set;
 *   - a vendor;
 *   - a QPU;
 *   - a QPU topology;
 *   - physical qubit identifiers;
 *   - a processor architecture;
 *   - a pulse implementation;
 *   - a calibration implementation;
 *   - a simulator implementation;
 *   - a routing algorithm;
 *   - a scheduling algorithm;
 *   - a noise model;
 *   - a QEC decoder;
 *   - a backend;
 *   - an execution provider.
 *
 * A source operation such as:
 *
 *     apply H to q[0];
 *
 * describes intent.
 *
 * Whether H is:
 *
 *     - a primitive;
 *     - a composite operation;
 *     - a user-defined operation;
 *     - an intrinsic;
 *     - decomposed;
 *     - synthesized;
 *     - simulated;
 *     - mapped to a QPU;
 *     - translated to another computational substrate
 *
 * is determined AFTER parsing.
 *
 * ============================================================================
 * IMPORTANT
 * ============================================================================
 *
 * This file intentionally contains parser rules only.
 *
 * It must be imported by the canonical Zamani parser:
 *
 *     grammar ZamaniParser;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This file must NOT declare:
 *
 *     lexer grammar ...
 *     grammar Zamani;
 *     token rules
 *     fragment lexer rules
 *     EOF entry points
 *
 * Lexical ownership belongs exclusively to:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical source-program ownership belongs to:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Semantic quantum ownership belongs downstream to the frontend and:
 *
 *     quantum::ir
 *
 * Rust implementation baseline:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     stable Rust
 *     no unsafe Rust
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. QUANTUM TOP-LEVEL DECLARATIONS
 * ========================================================================== */

/**
 * QuantumDeclaration
 *
 * Supports both the explicit circuit form and the general quantum block form.
 *
 * Examples:
 *
 *     quantum circuit Bell {
 *         ...
 *     }
 *
 *     quantum {
 *         ...
 *     }
 *
 * The parser does not require a fixed number of qubits.
 */
quantumDeclaration
    : QUANTUM quantumDeclarationBody
    ;

quantumDeclarationBody
    : CIRCUIT identifier
      genericParameters?
      quantumParameterClause?
      whereClause?
      blockExpression

    | blockExpression
    ;


/* ============================================================================
 * 2. QUANTUM PARAMETERS
 * ========================================================================== */

/**
 * Quantum parameters are ordinary source-level parameters.
 *
 * Their semantic type may be:
 *
 *     qubit
 *     register
 *     classical value
 *     parameter
 *     tensor
 *     state
 *     user-defined type
 *     future domain-specific type
 *
 * The grammar deliberately does not constrain the representation.
 */
quantumParameterClause
    : LPAREN parameterList? RPAREN
    ;


/* ============================================================================
 * 3. QUANTUM BLOCK
 * ========================================================================== */

quantumBlock
    : LBRACE quantumBlockElement* RBRACE
    ;

quantumBlockElement
    : attribute
    | quantumStatement
    | statement
    ;


/* ============================================================================
 * 4. QUANTUM STATEMENTS
 * ========================================================================== */

quantumStatement
    : quantumOperationStatement
    | quantumMeasurementStatement
    | quantumResetStatement
    | quantumBarrierStatement
    | quantumSynchronizeStatement
    | quantumAllocateStatement
    | quantumReleaseStatement
    | quantumDeclarationStatement
    ;


/* ============================================================================
 * 5. QUANTUM OPERATION
 * ========================================================================== */

/**
 * General quantum operation:
 *
 *     apply operation-expression to target-list;
 *
 * Examples:
 *
 *     apply H to q[0];
 *     apply X to q[1];
 *     apply U(theta, phi, lambda) to q[0];
 *     apply controlled(X) from q[0] to q[1];
 *
 * IMPORTANT:
 *
 * There is deliberately NO:
 *
 *     'H'
 *     'X'
 *     'CNOT'
 *     'T'
 *     'S'
 *     'SWAP'
 *     ...
 *
 * gate inventory here.
 *
 * Gate/operation names are expressions resolved semantically.
 */
quantumOperationStatement
    : APPLY quantumOperation targetSpecification quantumTerminator?
    ;

quantumOperation
    : quantumOperationReference
    | quantumOperationExpression
    ;

quantumOperationReference
    : identifier
      genericArguments?
    ;

quantumOperationExpression
    : expression
    ;


/* ============================================================================
 * 6. TARGET SPECIFICATION
 * ========================================================================== */

/**
 * Unary/multi-target operation:
 *
 *     apply H to q[0];
 *
 * Controlled or relational operation:
 *
 *     apply controlled(X) from q[0] to q[1];
 *
 * The grammar does not impose an arity limit.
 */
targetSpecification
    : TO quantumTargetList
    | FROM quantumTargetList TO quantumTargetList
    ;

quantumTargetList
    : quantumTarget
      (COMMA quantumTarget)*
    ;

quantumTarget
    : expression
    ;


/* ============================================================================
 * 7. QUANTUM MEASUREMENT
 * ========================================================================== */

/**
 * Measurement is a semantic operation.
 *
 * Examples:
 *
 *     measure q[0];
 *     measure q[0] -> result;
 *
 * The result type and classical destination are semantic concerns.
 */
quantumMeasurementStatement
    : MEASURE quantumTargetList
      quantumMeasurementDestination?
      quantumTerminator?
    ;

quantumMeasurementDestination
    : THIN_ARROW expression
    ;


/* ============================================================================
 * 8. RESET
 * ========================================================================== */

quantumResetStatement
    : RESET quantumTargetList
      quantumTerminator?
    ;


/* ============================================================================
 * 9. BARRIER
 * ========================================================================== */

/**
 * A barrier is a semantic ordering constraint.
 *
 * It is NOT a hardware instruction.
 */
quantumBarrierStatement
    : BARRIER quantumTargetList?
      quantumTerminator?
    ;


/* ============================================================================
 * 10. SYNCHRONIZATION
 * ========================================================================== */

/**
 * Synchronization expresses a semantic ordering requirement.
 *
 * The backend determines how this requirement is realized.
 */
quantumSynchronizeStatement
    : SYNCHRONIZE quantumTargetList?
      quantumTerminator?
    ;


/* ============================================================================
 * 11. ABSTRACT QUANTUM RESOURCE ALLOCATION
 * ========================================================================== */

/**
 * These forms describe abstract resource intent.
 *
 * They do not allocate physical hardware resources during parsing.
 *
 * Examples:
 *
 *     allocate q;
 *     allocate q : Qubit;
 *
 * Resource availability and limits belong to semantic/resource analysis.
 */
quantumAllocateStatement
    : ALLOCATE
      identifier
      (COLON typeExpression)?
      quantumTerminator?
    ;


/* ============================================================================
 * 12. ABSTRACT RESOURCE RELEASE
 * ========================================================================== */

quantumReleaseStatement
    : RELEASE
      expression
      quantumTerminator?
    ;


/* ============================================================================
 * 13. QUANTUM LOCAL DECLARATIONS
 * ========================================================================== */

/**
 * This rule provides a syntactic extension point for quantum declarations
 * without forcing the parser to enumerate every future quantum abstraction.
 *
 * The semantic registry determines whether the declaration is meaningful.
 */
quantumDeclarationStatement
    : quantumNamedDeclaration
    ;

quantumNamedDeclaration
    : QUANTUM
      identifier
      (
          genericParameters?
          quantumParameterClause?
          whereClause?
          blockExpression
        | ASSIGN expression
      )
      quantumTerminator?
    ;


/* ============================================================================
 * 14. QUANTUM LITERALS
 * ========================================================================== */

/**
 * Canonical basis-state literal:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *
 * The lexer emits:
 *
 *     PIPE
 *     ZERO / ONE / PLUS / MINUS
 *     KET_CLOSE
 *
 * rather than a specialized finite QUANTUM_LITERAL token.
 *
 * This prevents the lexer from hard-coding a closed quantum literal universe.
 */
quantumLiteral
    : PIPE quantumBasis KET_CLOSE
    ;

quantumBasis
    : ZERO
    | ONE
    | PLUS
    | MINUS
    ;


/* ============================================================================
 * 15. QUANTUM STATE EXPRESSIONS
 * ========================================================================== */

/**
 * Quantum state syntax remains compositional.
 *
 * This permits future state representations to be represented through normal
 * expressions and types instead of adding one lexer token per state.
 */
quantumStateExpression
    : quantumLiteral
    | expression
    ;


/* ============================================================================
 * 16. QUANTUM TYPE EXPRESSIONS
 * ========================================================================== */

/**
 * The parser recognizes the canonical primitive QUBIT type.
 *
 * More sophisticated quantum types remain ordinary type expressions and are
 * validated by the semantic/type system.
 *
 * Therefore the grammar does NOT contain:
 *
 *     QReg[32]
 *     QReg[1024]
 *     physical-qubit-0
 *     IBM-Q...
 *
 * or any other machine-size assumption.
 */
quantumTypeExpression
    : QUBIT
    | namedType
    | genericType
    | arrayType
    | tupleType
    | referenceType
    ;


/* ============================================================================
 * 17. QUANTUM REGIONS
 * ========================================================================== */

/**
 * A quantum region is an explicitly scoped region of quantum computation.
 *
 * It is structurally represented here; lifetime, purity, effects, resource
 * ownership and legality are semantic properties.
 */
quantumRegion
    : QUANTUM blockExpression
    ;


/* ============================================================================
 * 18. CONTROLLED OPERATIONS
 * ========================================================================== */

/**
 * Controlled operations are represented generically.
 *
 * Examples:
 *
 *     controlled(X)
 *     controlled(U(theta))
 *
 * No finite number of control qubits is encoded.
 */
controlledOperation
    : CONTROLLED
      LPAREN quantumOperation RPAREN
    ;


/* ============================================================================
 * 19. ADJOINT / INVERSE OPERATIONS
 * ========================================================================== */

/**
 * Adjoint/inverse is semantic rather than hardware-specific.
 */
adjointOperation
    : ADJOINT
      LPAREN quantumOperation RPAREN
    ;


/* ============================================================================
 * 20. POWER / REPETITION
 * ========================================================================== */

/**
 * Operation powers/repetition are represented structurally.
 *
 * Resource feasibility belongs to semantic/resource analysis.
 */
quantumPowerOperation
    : POWER
      LPAREN quantumOperation COMMA expression RPAREN
    ;


/* ============================================================================
 * 21. COMPOSITION
 * ========================================================================== */

/**
 * Generic composition:
 *
 *     compose(A, B, C)
 *
 * This is intentionally not expanded into a finite grammar of gate names.
 */
quantumComposition
    : COMPOSE
      LPAREN quantumOperationList? RPAREN
    ;

quantumOperationList
    : quantumOperation
      (COMMA quantumOperation)*
    ;


/* ============================================================================
 * 22. CONTROL FLOW INSIDE QUANTUM PROGRAMS
 * ========================================================================== */

/**
 * Quantum programs may contain ordinary language control flow.
 *
 * Whether a classical condition may control a quantum operation is a semantic
 * question involving types, effects, timing, measurement dependencies and the
 * target execution model.
 */
quantumConditional
    : IF expression quantumBlock
      (ELSE IF expression quantumBlock)*
      (ELSE quantumBlock)?
    ;


/* ============================================================================
 * 23. QUANTUM FEEDBACK
 * ========================================================================== */

/**
 * Measurement-driven feedback:
 *
 *     if result {
 *         ...
 *     }
 *
 * The grammar permits ordinary expressions as predicates.
 *
 * Semantic analysis determines whether the expression is:
 *
 *     - compile-time;
 *     - classical runtime;
 *     - measurement-derived;
 *     - temporal;
 *     - probabilistic;
 *     - target-dependent.
 */
quantumFeedback
    : IF expression quantumBlock
    ;


/* ============================================================================
 * 24. PARAMETRIC QUANTUM OPERATIONS
 * ========================================================================== */

/**
 * Parameters are ordinary expressions.
 *
 * Example:
 *
 *     apply rotation(theta) to q[0];
 *
 * No assumption is made about the representation of theta.
 */
quantumParameterizedOperation
    : quantumOperationReference
      LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 25. OPERATION MODIFIERS
 * ========================================================================== */

/**
 * Modifiers compose semantically.
 *
 * Example:
 *
 *     controlled(adjoint(U(theta)))
 *
 * The parser preserves nesting.
 */
quantumOperationModifier
    : CONTROLLED
    | ADJOINT
    ;


/* ============================================================================
 * 26. QUANTUM OPERATION COMPOSITION
 * ========================================================================== */

/**
 * Recursive composition allows arbitrary operation trees subject to the
 * concrete source grammar.
 *
 * Example:
 *
 *     controlled(adjoint(U(theta)))
 *
 * Semantic validation determines whether such composition is legal.
 */
quantumOperationTree
    : quantumOperationAtom
    | controlledOperation
    | adjointOperation
    | quantumPowerOperation
    | quantumComposition
    ;

quantumOperationAtom
    : identifier
      (
          genericArguments?
          LPAREN argumentList? RPAREN
      )?
    ;


/* ============================================================================
 * 27. GENERIC ARGUMENTS
 * ========================================================================== */

genericArguments
    : LESS_THAN typeExpression
      (COMMA typeExpression)*
      GREATER_THAN
    ;


/* ============================================================================
 * 28. QUANTUM EXECUTION INTENT
 * ========================================================================== */

/**
 * Execution intent remains abstract.
 *
 * This grammar does NOT accept:
 *
 *     backend = IBM
 *     qpu = ibm_fez
 *     topology = ...
 *     device = ...
 *
 * Those belong to deployment/configuration/target layers.
 */
quantumExecutionRegion
    : EXECUTE quantumBlock
    ;


/* ============================================================================
 * 29. QUANTUM SIMULATION INTENT
 * ========================================================================== */

/**
 * Simulation is an execution strategy, not part of quantum semantics.
 *
 * If the source language eventually exposes simulation intent, it is represented
 * as a semantic request and resolved downstream.
 */
quantumSimulationRegion
    : SIMULATE quantumBlock
    ;


/* ============================================================================
 * 30. ABSTRACT NOISE INTENT
 * ========================================================================== */

/**
 * Noise is represented abstractly.
 *
 * A noise model is NOT defined here.
 *
 * ZQN owns noise semantics downstream.
 */
quantumNoiseRegion
    : NOISE quantumBlock
    ;


/* ============================================================================
 * 31. ABSTRACT ERROR-CORRECTION REGION
 * ========================================================================== */

/**
 * Error correction belongs to semantic quantum/QEC layers.
 *
 * The parser can preserve an abstract declaration without hard-coding:
 *
 *     surface-code distance
 *     code family
 *     decoder
 *     physical layout
 *     syndrome-extraction circuit
 *
 * Those are downstream decisions.
 */
quantumErrorCorrectionRegion
    : ERROR_CORRECTION quantumBlock
    ;


/* ============================================================================
 * 32. ABSTRACT LOGICAL REGION
 * ========================================================================== */

/**
 * Logical computation is distinct from physical realization.
 */
quantumLogicalRegion
    : LOGICAL quantumBlock
    ;


/* ============================================================================
 * 33. ENTANGLEMENT
 * ========================================================================== */

/**
 * Entanglement is a semantic relationship.
 *
 * No fixed arity is encoded.
 */
quantumEntangleStatement
    : ENTANGLE quantumTargetList
      quantumTerminator?
    ;


/* ============================================================================
 * 34. QUANTUM CHANNEL / COMMUNICATION INTENT
 * ========================================================================== */

/**
 * Channel syntax expresses communication intent.
 *
 * Transport mechanism belongs downstream.
 */
quantumChannelStatement
    : CHANNEL
      identifier
      (
          LPAREN parameterList? RPAREN
      )?
      blockExpression
    ;


/* ============================================================================
 * 35. DISTRIBUTED QUANTUM INTENT
 * ========================================================================== */

/**
 * A distributed quantum region may span arbitrary logical resources.
 *
 * Placement and communication routing are downstream concerns.
 */
quantumDistributedRegion
    : DISTRIBUTED quantumBlock
    ;


/* ============================================================================
 * 36. QUANTUM TIMING INTENT
 * ========================================================================== */

/**
 * Timing syntax is semantic timing information, not a hardware clock model.
 */
quantumTimingAnnotation
    : TIMING
      LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 37. QUANTUM RESOURCE CONSTRAINT INTENT
 * ========================================================================== */

/**
 * Resource constraints describe requirements, not actual allocation.
 *
 * Example:
 *
 *     require resource(expression);
 *
 * The resource subsystem decides feasibility.
 */
quantumResourceRequirement
    : REQUIRE RESOURCE
      LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 38. QUANTUM CAPABILITY REQUIREMENT
 * ========================================================================== */

/**
 * Capabilities are semantic requirements.
 *
 * The grammar does not know which target provides them.
 */
quantumCapabilityRequirement
    : REQUIRE CAPABILITY
      LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 39. QUANTUM ANNOTATIONS
 * ========================================================================== */

/**
 * Quantum annotations use the common Zamani annotation syntax.
 *
 * Examples:
 *
 *     @logical
 *     @unitary
 *     @variational
 *
 * The annotation itself has no semantic authority.
 */
quantumAnnotation
    : AT identifier
      (LPAREN argumentList? RPAREN)?
    ;


/* ============================================================================
 * 40. QUANTUM PROGRAM ELEMENT
 * ========================================================================== */

/**
 * Canonical reusable quantum element.
 */
quantumElement
    : attribute
    | quantumAnnotation
    | quantumStatement
    | quantumConditional
    | quantumFeedback
    | quantumRegion
    | quantumExecutionRegion
    | quantumSimulationRegion
    | quantumNoiseRegion
    | quantumErrorCorrectionRegion
    | quantumLogicalRegion
    | quantumDistributedRegion
    | quantumChannelStatement
    | quantumResourceRequirement
    | quantumCapabilityRequirement
    | statement
    ;


/* ============================================================================
 * 41. QUANTUM TERMINATOR
 * ========================================================================== */

/**
 * Semicolon remains optional where the canonical source syntax permits it.
 */
quantumTerminator
    : SEMI
    ;


/* ============================================================================
 * 42. QUANTUM PROGRAM BODY
 * ========================================================================== */

quantumProgramBody
    : LBRACE quantumElement* RBRACE
    ;


/* ============================================================================
 * 43. QUANTUM CIRCUIT
 * ========================================================================== */

/**
 * Circuit is one quantum computational model.
 *
 * It is NOT the definition of every quantum program.
 *
 * The canonical IR supports broader computational models including dynamic,
 * analog, Hamiltonian, annealing, QUBO, fermionic, bosonic, continuous-
 * variable, measurement-based, tensor-network, logical and distributed
 * computation.
 */
quantumCircuitDeclaration
    : CIRCUIT identifier
      genericParameters?
      quantumParameterClause?
      whereClause?
      quantumProgramBody
    ;


/* ============================================================================
 * 44. QUANTUM OPERATION DECLARATION
 * ========================================================================== */

/**
 * User-defined operations.
 *
 * Example:
 *
 *     operation oracle(x) {
 *         ...
 *     }
 *
 * The semantic layer determines:
 *
 *     - arity;
 *     - purity;
 *     - reversibility;
 *     - unitary properties;
 *     - measurement effects;
 *     - resource requirements;
 *     - decomposability.
 */
quantumOperationDeclaration
    : OPERATION identifier
      genericParameters?
      quantumParameterClause?
      whereClause?
      quantumProgramBody
    ;


/* ============================================================================
 * 45. QUANTUM FUNCTION DECLARATION
 * ========================================================================== */

/**
 * Quantum functions reuse ordinary Zamani function syntax and semantic
 * checking. This prevents the quantum subsystem from creating a second
 * function language.
 */
quantumFunctionDeclaration
    : ASYNC?
      FN identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      quantumProgramBody
    ;


/* ============================================================================
 * 46. QUANTUM MODEL DECLARATION
 * ========================================================================== */

/**
 * Abstract model declaration.
 *
 * This provides a syntactic envelope for semantic models such as:
 *
 *     circuit
 *     Hamiltonian
 *     annealing
 *     QUBO
 *     variational
 *     measurement-based
 *     tensor-network
 *     continuous-variable
 *     fermionic
 *     bosonic
 *
 * The parser deliberately does not enumerate every model as a separate
 * hardware-specific language.
 */
quantumModelDeclaration
    : MODEL identifier
      genericParameters?
      quantumParameterClause?
      whereClause?
      quantumProgramBody
    ;


/* ============================================================================
 * 47. QUANTUM OBSERVABLE / RESULT INTENT
 * ========================================================================== */

/**
 * Observable expressions are ordinary expressions.
 *
 * Their mathematical/physical validity belongs to semantic analysis.
 */
quantumObservable
    : OBSERVABLE expression
    ;


/* ============================================================================
 * 48. VARIATIONAL / PARAMETRIC PROGRAM INTENT
 * ========================================================================== */

/**
 * Variational computation remains source-level intent.
 *
 * Optimizer selection belongs downstream.
 */
quantumVariationalRegion
    : VARIATIONAL
      quantumProgramBody
    ;


/* ============================================================================
 * 49. QUANTUM SAMPLING INTENT
 * ========================================================================== */

quantumSamplingRegion
    : SAMPLE
      quantumProgramBody
    ;


/* ============================================================================
 * 50. QUANTUM PROGRAM COMPOSITION
 * ========================================================================== */

/**
 * Composition is structural and unlimited by grammar.
 *
 * Concrete execution limits are imposed by explicit resource policies.
 */
quantumComposeStatement
    : COMPOSE
      quantumTargetList
      quantumTerminator?
    ;


/* ============================================================================
 * 51. CANONICAL QUANTUM OPERATION ROOT
 * ========================================================================== */

/**
 * This is the principal semantic source construct.
 *
 * Everything below it is intentionally generic.
 */
quantumOperationRoot
    : quantumOperationTree
    ;


/* ============================================================================
 * 52. EXTENSIBILITY BOUNDARY
 * ========================================================================== */

/**
 * Named quantum facilities are identifiers rather than lexer keywords.
 *
 * This is intentional.
 *
 * A future quantum abstraction should normally NOT require changing:
 *
 *     ZamaniLexer.g4
 *     Quantum.g4
 *     ZamaniParser.g4
 *
 * merely because a new operation/model/backend exists.
 *
 * Instead:
 *
 *     source identifier
 *          |
 *          v
 *     semantic registry / resolver
 *          |
 *          v
 *     canonical quantum semantic operation
 *          |
 *          v
 *     quantum::ir
 *
 * This is what prevents language growth from becoming an ever-growing
 * hardware-specific keyword list.
 */


/* ============================================================================
 * 53. INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Quantum.g4 depends on parser rules owned by ZamaniParser.g4:
 *
 *     identifier
 *     genericParameters
 *     whereClause
 *     parameterList
 *     returnType
 *     blockExpression
 *     statement
 *     expression
 *     typeExpression
 *     namedType
 *     genericType
 *     arrayType
 *     tupleType
 *     referenceType
 *     attribute
 *     argumentList
 *
 * It depends on lexical tokens owned by ZamaniLexer.g4:
 *
 *     QUANTUM
 *     CIRCUIT
 *     APPLY
 *     TO
 *     FROM
 *     MEASURE
 *     RESET
 *     BARRIER
 *     SYNCHRONIZE
 *     ALLOCATE
 *     RELEASE
 *     ENTANGLE
 *     CONTROLLED
 *     ADJOINT
 *     POWER
 *     COMPOSE
 *     EXECUTE
 *     SIMULATE
 *     NOISE
 *     ERROR_CORRECTION
 *     LOGICAL
 *     CHANNEL
 *     DISTRIBUTED
 *     TIMING
 *     REQUIRE
 *     RESOURCE
 *     CAPABILITY
 *     OPERATION
 *     MODEL
 *     OBSERVABLE
 *     VARIATIONAL
 *     SAMPLE
 *
 * plus ordinary punctuation/operator tokens.
 *
 * The exact token inventory MUST be synchronized with ZamaniLexer.g4.
 */


/* ============================================================================
 * 54. SEMANTIC OWNERSHIP
 * ========================================================================== */

/**
 * Quantum.g4 OWNS:
 *
 *     concrete quantum source structure
 *
 * Quantum.g4 DOES NOT OWN:
 *
 *     qubit allocation
 *     physical mapping
 *     gate decomposition
 *     routing
 *     scheduling
 *     calibration
 *     noise
 *     QEC
 *     simulation
 *     hardware execution
 *     optimization
 *     benchmarking
 *     target capabilities
 *
 * Ownership:
 *
 *     parser
 *         -> source structure
 *
 *     frontend AST
 *         -> structural representation
 *
 *     semantic analysis
 *         -> meaning / validity
 *
 *     quantum::ir
 *         -> canonical quantum semantic representation
 *
 *     optimization
 *         -> semantics-preserving transformation
 *
 *     routing
 *         -> placement / connectivity realization
 *
 *     scheduling
 *         -> temporal realization
 *
 *     ZQN
 *         -> noise / fault / calibration semantics
 *
 *     hardware
 *         -> target realization
 */


/* ============================================================================
 * 55. SCALE CONTRACT
 * ========================================================================== */

/**
 * There are intentionally no grammar constants such as:
 *
 *     MAX_QUBITS
 *     MAX_QREG
 *     MAX_GATES
 *     MAX_DEPTH
 *     MAX_OPERANDS
 *     MAX_CLASSICAL_BITS
 *
 * Repetition operators are therefore used throughout:
 *
 *     *
 *     +
 *
 * rather than finite enumerations.
 *
 * "Infinity" means that the language introduces no artificial finite machine
 * ceiling.
 *
 * Every actual compilation/execution remains bounded by explicit:
 *
 *     resource policy
 *     process limits
 *     memory availability
 *     target capabilities
 *     execution environment
 *
 * Those constraints must never be silently encoded into this grammar.
 */


/* ============================================================================
 * 56. DETERMINISM CONTRACT
 * ========================================================================== */

/**
 * The parser must be deterministic for a given token stream.
 *
 * No semantic registry, hardware database, network service, filesystem,
 * environment variable, random source or backend is consulted by this grammar.
 *
 * The same source/token stream must produce the same parse tree.
 */


/* ============================================================================
 * 57. SECURITY CONTRACT
 * ========================================================================== */

/**
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     dynamic library loading
 *     hardware access
 *     credential access
 *
 * Parsing is a pure source-to-structure operation.
 */


/* ============================================================================
 * 58. RUST SAFETY CONTRACT
 * ========================================================================== */

/**
 * ANTLR-generated/parser integration code must be maintained under the
 * repository's Rust safety policy:
 *
 *     Rust 1.97 / 1.97.1
 *     Rust 2021
 *     #![forbid(unsafe_code)]
 *
 * Quantum.g4 itself contains no Rust implementation code and therefore
 * introduces no unsafe operations.
 */


/* ============================================================================
 * 59. TEST CONTRACT
 * ========================================================================== */

/**
 * This grammar must be exercised by at least the following conformance
 * categories:
 *
 * VALID:
 *
 *     quantum {}
 *     quantum circuit Bell {}
 *     quantum {
 *         apply H to q[0];
 *     }
 *
 *     quantum {
 *         apply controlled(X) from q[0] to q[1];
 *     }
 *
 *     quantum {
 *         apply U(theta, phi, lambda) to q[0];
 *     }
 *
 *     quantum {
 *         measure q[0];
 *     }
 *
 *     quantum {
 *         reset q[0];
 *     }
 *
 *     quantum {
 *         apply user_defined_operation(x) to target;
 *     }
 *
 *     quantum {
 *         apply controlled(adjoint(U(theta))) to target;
 *     }
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *
 * INVALID:
 *
 *     quantum circuit Bell(32 qubits) {}
 *
 *     // if the language does not define that syntax
 *
 *     apply H to q[0]
 *         // missing quantum/source context where required
 *
 *     measure
 *         // missing target
 *
 *     apply H to
 *         // incomplete target specification
 *
 *     apply controlled(
 *         // unterminated operation
 *
 * SCALE:
 *
 *     arbitrarily many targets
 *     arbitrarily many operations
 *     arbitrarily nested operation expressions
 *     arbitrarily large source-level indices
 *     arbitrarily large parameter expressions
 *
 * The test harness, not the grammar, determines practical test-resource
 * limits.
 */


/* ============================================================================
 * 60. ANTLR COMPOSITION CONTRACT
 * ========================================================================== */

/**
 * This file must be included by the canonical parser build.
 *
 * Conceptually:
 *
 *     grammar ZamaniParser;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     // Core parser rules
 *     ...
 *
 *     // Quantum parser rules from this file
 *     ...
 *
 * The build system must generate exactly one canonical Zamani parser.
 *
 * Quantum.g4 must never be generated as an independent language.
 */