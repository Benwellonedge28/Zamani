/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/adjoints.g4
 *
 * Grammar:
 *     QuantumAdjoints
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM ADJOINT-MODIFIER GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical parser owner for SOURCE-LEVEL QUANTUM ADJOINT
 * OPERATION MODIFIER SYNTAX.
 *
 * An adjoint represents the semantic transformation:
 *
 *     adjoint(operation)
 *
 * without requiring the grammar to know:
 *
 *     - the operation's implementation;
 *     - its matrix representation;
 *     - its decomposition;
 *     - its native target instruction;
 *     - its physical realization;
 *     - its hardware;
 *     - its qubit count;
 *     - its resource requirements;
 *     - its error-correction implementation.
 *
 * The grammar expresses SOURCE STRUCTURE only.
 *
 * ============================================================================
 * CANONICAL SOURCE FORMS
 * ============================================================================
 *
 * The canonical invocation form is:
 *
 *     apply adjoint(H)(q);
 *
 *     apply adjoint(X)(q);
 *
 *     apply adjoint(U(theta))(q);
 *
 *     apply adjoint(namespace::operation)(q);
 *
 *     apply adjoint(namespace::operation(a, b))(q0, q1);
 *
 *     apply adjoint(custom_operation)(targets);
 *
 * Nested transformations are structurally supported:
 *
 *     apply adjoint(inverse(U))(q);
 *
 *     apply adjoint(control(U))(control, target);
 *
 *     apply adjoint(adjoint(U))(q);
 *
 *     apply adjoint(inverse(control(operation(parameter))))(targets);
 *
 * The semantic layer determines whether each requested transformation is
 * meaningful for the referenced operation.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * An adjoint is an OPERATION TRANSFORMATION, not a gate catalogue.
 *
 * Therefore this file MUST NOT contain:
 *
 *     adjointH
 *     adjointX
 *     adjointY
 *     adjointZ
 *     adjointCNOT
 *     adjointSWAP
 *     adjointRX
 *     adjointRY
 *     adjointRZ
 *
 * or any equivalent finite enumeration.
 *
 * Operation identity remains an open-ended source-level name.
 *
 * Examples:
 *
 *     H
 *     X
 *     custom_gate
 *     library::operation
 *     vendor::operation
 *     future::operation
 *
 * remain names.
 *
 * Their quantum meaning is determined by semantic analysis.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     quantumAdjointModifier
 *     quantumAdjointModifierOperand
 *     quantumAdjointOperationReference
 *     quantumAdjointDesignatorReference
 *     quantumAdjointParameterClause
 *
 * These rules define the structural syntax of an adjoint transformation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - keywords;
 *     - identifiers;
 *     - qualified names;
 *     - generic argument syntax;
 *     - general expressions;
 *     - quantum operation invocation;
 *     - operation target lists;
 *     - qubit declarations;
 *     - quantum registers;
 *     - logical qubits;
 *     - physical qubits;
 *     - measurements;
 *     - reset;
 *     - observables;
 *     - dynamic classical control;
 *     - controlled-operation semantics;
 *     - inverse semantics;
 *     - power semantics;
 *     - gate matrices;
 *     - gate implementations;
 *     - resource allocation;
 *     - capability discovery;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - calibration;
 *     - hardware topology;
 *     - physical mapping;
 *     - HAL;
 *     - runtime execution;
 *     - canonical quantum::ir.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one canonical owner for adjoint modifier syntax.
 *
 * The intended final ownership is:
 *
 *     grammar/quantum/adjoints.g4
 *         |
 *         +--> quantumAdjointModifier
 *         +--> quantumAdjointModifierOperand
 *         +--> quantumAdjointOperationReference
 *
 * `operations.g4` remains the owner of operation invocation.
 *
 * Therefore:
 *
 *     operations.g4
 *         owns:
 *             quantumOperationInvocation
 *             quantumOperationDesignator
 *             quantumOperationTargetClause
 *
 *     adjoints.g4
 *         owns:
 *             quantumAdjointModifier
 *             quantumAdjointModifierOperand
 *
 * This prevents the operation grammar from becoming the owner of every
 * transformation mechanism.
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
 *     parser
 *          |
 *          v
 *     QuantumAdjoints
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> operation resolution
 *          +--> adjoint validity
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> resilience
 *          +--> HAL
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime
 *
 * This file MUST NOT bypass the AST or semantic layers.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Adjoint syntax expresses a mathematical/semantic transformation of an
 * operation.
 *
 * It does NOT select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     physical qubit
 *     physical register
 *     memory bank
 *     device
 *     device ID
 *     vendor
 *     topology
 *     coupling map
 *     pulse
 *     calibration
 *     native instruction
 *     scheduler
 *     router
 *
 * The same source:
 *
 *     apply adjoint(operation)(targets);
 *
 * may therefore be lowered to different implementations on different
 * computational substrates.
 *
 * This is a required part of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO artificial resource limit.
 *
 * There is no:
 *
 *     MAX_ADJOINTS
 *     MAX_ADJOINT_DEPTH
 *     MAX_QUBITS
 *     MAX_TARGETS
 *     MAX_PARAMETERS
 *     MAX_OPERATIONS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_DEVICES
 *     MAX_REGISTER_WIDTH
 *
 * or equivalent language-level ceiling.
 *
 * Recursive modifier composition is represented structurally.
 *
 * Therefore the language does not distinguish:
 *
 *     small adjoint
 *     medium adjoint
 *     large adjoint
 *
 * Nor does it introduce:
 *
 *     adjoint2
 *     adjoint4
 *     adjoint8
 *     adjoint32
 *
 * Any practical limit is an implementation/resource limit, not a language
 * semantic limit.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Its lexical vocabulary is composed through:
 *
 *     grammar/lexer/tokens.g4
 *
 * The adjoint keyword is supplied by the canonical keyword vocabulary:
 *
 *     ADJOINT : 'adjoint'
 *
 * This file MUST NOT define:
 *
 *     ADJOINT
 *     DAGGER
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * or any other lexer token.
 *
 * In particular, `dagger` is NOT silently introduced as a second spelling
 * unless the lexical specification explicitly promotes such a spelling.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical parser dependencies:
 *
 *     Names
 *         qualifiedName
 *         identifier
 *
 *     Expressions
 *         argumentList
 *         expression
 *         genericArgumentSuffix
 *
 * The operation grammar remains responsible for combining the adjoint
 * modifier with the final operation target clause.
 *
 * ============================================================================
 * OPERATION-NAME EXTENSIBILITY
 * ============================================================================
 *
 * This file deliberately accepts open operation names.
 *
 * Valid structural examples include:
 *
 *     H
 *     X
 *     custom_gate
 *     quantum::operation
 *     library::quantum::operation
 *     vendor::operation
 *     future::operation
 *
 * The grammar does not decide whether any of these names actually exist.
 *
 * That belongs to:
 *
 *     name resolution
 *     operation registry
 *     semantic analysis
 *     dialect resolution
 *
 * ============================================================================
 * GENERIC OPERATION REFERENCES
 * ============================================================================
 *
 * Generic arguments are delegated to the existing universal generic argument
 * grammar.
 *
 * Example:
 *
 *     adjoint(operation::<T>)
 *
 * This file MUST NOT create another generic-argument grammar.
 *
 * ============================================================================
 * PARAMETER / TARGET SEPARATION
 * ============================================================================
 *
 * This file handles parameters belonging to the operation being transformed.
 *
 * Example:
 *
 *     apply adjoint(RX(theta))(q);
 *
 * Here:
 *
 *     adjoint
 *         |
 *         v
 *     RX(theta)
 *         |
 *         v
 *     target q
 *
 * The parameter clause belongs to the operation reference.
 *
 * The final target clause belongs to `operations.g4`.
 *
 * This separation is important because:
 *
 *     adjoint(RX(theta))(q)
 *
 * must not be interpreted as:
 *
 *     adjoint(RX)(theta)(q)
 *
 * unless the operation grammar explicitly defines such a structure.
 *
 * ============================================================================
 * NESTED MODIFIERS
 * ============================================================================
 *
 * Adjoint may wrap another operation transformation.
 *
 * Examples:
 *
 *     adjoint(control(U))
 *
 *     adjoint(inverse(U))
 *
 *     adjoint(adjoint(U))
 *
 *     adjoint(control(inverse(U)))
 *
 *     adjoint(inverse(control(operation(theta))))
 *
 * The grammar imposes no finite nesting depth.
 *
 * Semantic analysis determines:
 *
 *     whether the composition is valid;
 *     whether transformations commute;
 *     whether a transformation can be lowered;
 *     whether a decomposition is required.
 *
 * ============================================================================
 * DOUBLE ADJOINT
 * ============================================================================
 *
 * The grammar intentionally permits:
 *
 *     adjoint(adjoint(operation))
 *
 * because the parser should represent source structure rather than perform
 * mathematical simplification.
 *
 * Semantic analysis may establish:
 *
 *     adjoint(adjoint(U)) == U
 *
 * where the operation semantics make that identity applicable.
 *
 * The parser MUST NOT rewrite the source.
 *
 * ============================================================================
 * ADJOINT VS INVERSE
 * ============================================================================
 *
 * This grammar does not treat:
 *
 *     adjoint
 *
 * and:
 *
 *     inverse
 *
 * as syntactic synonyms.
 *
 * They are separate semantic transformations.
 *
 * `operations.g4` / the canonical modifier system owns composition with
 * inverse.
 *
 * Semantic analysis determines the mathematical relationship for a specific
 * operation.
 *
 * For unitary quantum operations, the adjoint may coincide mathematically with
 * the inverse, but that fact MUST NOT be assumed by this parser grammar.
 *
 * ============================================================================
 * ADJOINT VS CONTROL
 * ============================================================================
 *
 * The grammar permits composition:
 *
 *     adjoint(control(U))
 *
 * and:
 *
 *     control(adjoint(U))
 *
 * because source structure is compositional.
 *
 * Semantic analysis determines whether the transformations are equivalent,
 * how they should be represented, and how they should be lowered.
 *
 * The parser MUST preserve modifier ordering.
 *
 * ============================================================================
 * SOURCE ORDER PRESERVATION
 * ============================================================================
 *
 * The parser/AST integration MUST preserve:
 *
 *     - modifier kind;
 *     - modifier nesting;
 *     - operation name;
 *     - qualified-name components;
 *     - generic arguments;
 *     - operation parameter order;
 *     - source spans.
 *
 * This is required for:
 *
 *     diagnostics
 *     formatting
 *     refactoring
 *     IDE/LSP
 *     source maps
 *     provenance
 *     semantic analysis
 *     optimization explanations
 *     compiler diagnostics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define a Rust AST type.
 *
 * The domain-neutral frontend AST remains owned by:
 *
 *     src/frontend/ast/
 *
 * The required semantic information is conceptually:
 *
 *     AdjointModifier {
 *         operand: OperationReference | Modifier,
 *         source_span
 *     }
 *
 * An operation reference conceptually contains:
 *
 *     designator
 *     generic_arguments
 *     parameters
 *     source_span
 *
 * The actual Rust representation MUST remain owned by the frontend AST.
 *
 * There must be no:
 *
 *     QuantumAdjointAst
 *
 * or other competing AST hierarchy introduced solely by this grammar file.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes:
 *
 *     "This source has structurally valid adjoint syntax."
 *
 * Semantic analysis establishes:
 *
 *     "This referenced operation admits an adjoint."
 *
 * Semantic validation must determine, as applicable:
 *
 *     - whether the operation exists;
 *     - whether the operation is callable;
 *     - whether its parameters are valid;
 *     - whether the operation is quantum;
 *     - whether an adjoint is defined;
 *     - whether nested transformations are valid;
 *     - whether operand types are compatible;
 *     - whether effects permit adjoint transformation;
 *     - whether capabilities are sufficient;
 *     - whether resource requirements can be satisfied;
 *     - whether target realization supports the required transformation;
 *     - whether decomposition is required.
 *
 * These MUST NOT become parser predicates.
 *
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * An operation may have semantic effects that affect whether its adjoint is
 * meaningful.
 *
 * Examples include operations involving:
 *
 *     measurement
 *     irreversible reset
 *     external I/O
 *     classical side effects
 *     nondeterministic effects
 *     resource consumption
 *
 * The parser MUST NOT inspect effects.
 *
 * The semantic/effect system decides whether:
 *
 *     adjoint(operation)
 *
 * is legal.
 *
 * ============================================================================
 * MEASUREMENT
 * ============================================================================
 *
 * The grammar permits an operation name syntactically even when the semantic
 * operation ultimately resolves to a measurement-related operation.
 *
 * Whether an adjoint exists is a semantic question.
 *
 * For example:
 *
 *     apply adjoint(measurement_operation)(q);
 *
 * may parse structurally but can be rejected semantically if the operation has
 * no valid adjoint.
 *
 * This separation is intentional.
 *
 * ============================================================================
 * RESET
 * ============================================================================
 *
 * Reset-like operations are treated the same way:
 *
 *     apply adjoint(reset_operation)(q);
 *
 * may be syntactically valid while being semantically invalid.
 *
 * No parser-level special case is introduced.
 *
 * ============================================================================
 * CUSTOM / VENDOR / DIALECT OPERATIONS
 * ============================================================================
 *
 * Custom operations remain open:
 *
 *     apply adjoint(custom::operation)(q);
 *
 * Vendor operations remain names:
 *
 *     apply adjoint(vendor::operation)(q);
 *
 * Dialect resolution remains downstream:
 *
 *     source
 *       |
 *       v
 *     operation name
 *       |
 *       v
 *     dialect / module / library resolution
 *       |
 *       v
 *     semantic operation
 *
 * This means adding a new operation does not require modifying this grammar.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This file does not define IR.
 *
 * A successfully validated construct conceptually becomes:
 *
 *     Adjoint(operation)
 *             |
 *             v
 *     semantic quantum operation
 *             |
 *             v
 *     quantum::ir
 *
 * The canonical quantum IR remains:
 *
 *     quantum::ir
 *
 * There must not be another:
 *
 *     QuantumAdjointIR
 *     FrontendQuantumAdjointIR
 *     GateAdjointIR
 *
 * introduced by this grammar.
 *
 * ============================================================================
 * LOWERING CONTRACT
 * ============================================================================
 *
 * The compiler may realize an adjoint by:
 *
 *     - direct native realization;
 *     - operation metadata;
 *     - decomposition;
 *     - sequence reversal;
 *     - parameter transformation;
 *     - synthesis;
 *     - target-specific lowering.
 *
 * The choice belongs downstream.
 *
 * The source grammar does not encode the realization strategy.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * This grammar does not decide whether an adjoint is available on a target.
 *
 * Semantic/resource analysis may derive requirements such as:
 *
 *     capability("quantum.adjoint")
 *
 * or other canonical capability forms.
 *
 * It may also derive resource requirements.
 *
 * Such requirements MUST remain separate from syntax.
 *
 * A resource failure is not a parser error.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file must never contain:
 *
 *     physical_qubit
 *     qpu0
 *     gpu0
 *     device0
 *     coupling_map
 *     native_gate
 *     calibration
 *     pulse
 *     hardware_width
 *
 * as universal adjoint syntax.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal limits include:
 *
 *     MAX_ADJOINTS
 *     MAX_ADJOINT_DEPTH
 *     MAX_QUBITS
 *     MAX_TARGETS
 *     MAX_PARAMETERS
 *     MAX_OPERATIONS
 *     MAX_DEVICES
 *     MAX_REGISTER_WIDTH
 *     MAX_CIRCUIT_DEPTH
 *
 * Forbidden fixed physical assumptions include:
 *
 *     physical_qubit_0
 *     physical_qubit_1
 *     qpu0
 *     qpu1
 *     device0
 *     device1
 *
 * Program values are not implementation limits.
 *
 * For example:
 *
 *     let depth = user_defined_value;
 *
 * is source data.
 *
 * A compiler constant such as:
 *
 *     MAX_ADJOINT_DEPTH = 32
 *
 * would be an implementation ceiling and MUST NOT become language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no runtime calls;
 *     no randomness;
 *     no environment-dependent parsing.
 *
 * Identical source/token input under the same grammar/version must produce
 * equivalent parse structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem I/O
 *     network I/O
 *     process execution
 *     credential access
 *     hardware access
 *     runtime invocation
 *     backend invocation.
 *
 * No embedded target-language action is permitted.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural parser diagnostics should identify:
 *
 *     - missing `adjoint`;
 *     - missing opening parenthesis;
 *     - missing closing parenthesis;
 *     - missing operation designator;
 *     - malformed nested modifier;
 *     - malformed parameter clause.
 *
 * Semantic diagnostics belong downstream.
 *
 * Relevant existing diagnostic categories include:
 *
 *     ZMN-QUANTUM-INVALID-ADJOINT
 *     ZMN-QUANTUM-INVALID-PARAMETER
 *     ZMN-QUANTUM-INVALID-ARITY
 *     ZMN-QUANTUM-RESOURCE-UNSATISFIED
 *
 * This grammar MUST NOT emit semantic diagnostics directly.
 *
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar QuantumAdjoints;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
;


/*
 * ============================================================================
 * 1. CANONICAL ADJOINT MODIFIER
 * ============================================================================
 *
 * Canonical form:
 *
 *     adjoint(operation)
 *
 * Examples:
 *
 *     adjoint(H)
 *     adjoint(X)
 *     adjoint(U(theta))
 *     adjoint(namespace::operation)
 *     adjoint(custom::operation(parameter))
 *
 * This rule owns the `adjoint(...)` transformation syntax.
 */

quantumAdjointModifier
    : ADJOINT
      LPAREN
      quantumAdjointModifierOperand
      RPAREN
    ;


/*
 * ============================================================================
 * 2. ADJOINT OPERAND
 * ============================================================================
 *
 * An adjoint may wrap:
 *
 *     - an operation reference;
 *     - another operation modifier.
 *
 * This permits composition with the canonical operation transformation system.
 *
 * Examples:
 *
 *     adjoint(H)
 *     adjoint(RX(theta))
 *     adjoint(control(U))
 *     adjoint(inverse(U))
 *     adjoint(adjoint(U))
 *
 * The grammar intentionally does not attempt to simplify nested modifiers.
 */

quantumAdjointModifierOperand
    : quantumAdjointOperationReference
    | quantumOperationModifierReference
    ;


/*
 * ============================================================================
 * 3. OPERATION REFERENCE
 * ============================================================================
 *
 * This rule represents the operation being adjointed.
 *
 * It does NOT invoke the operation.
 *
 * Therefore:
 *
 *     adjoint(U(theta))
 *
 * contains an operation reference.
 *
 * The target list is supplied later by operations.g4:
 *
 *     apply adjoint(U(theta))(q);
 *
 * The separation prevents target syntax from becoming owned by this file.
 */

quantumAdjointOperationReference
    : quantumAdjointDesignatorReference
      quantumAdjointParameterClause?
    ;


/*
 * ============================================================================
 * 4. OPERATION DESIGNATOR
 * ============================================================================
 *
 * Operation identity is an open qualified name.
 *
 * Examples:
 *
 *     H
 *     custom_operation
 *     quantum::operation
 *     library::quantum::operation
 *     vendor::operation
 *     future::operation
 *
 * Generic argument syntax is delegated to the canonical expression/name
 * infrastructure.
 */

quantumAdjointDesignatorReference
    : qualifiedName
      genericArgumentSuffix?
    ;


/*
 * ============================================================================
 * 5. OPERATION PARAMETERS
 * ============================================================================
 *
 * Operation parameters are ordinary Zamani expressions.
 *
 * Examples:
 *
 *     adjoint(RX(theta))
 *
 *     adjoint(U(theta, phi, lambda))
 *
 *     adjoint(operation(f(x), y))
 *
 * The grammar does not impose a parameter count.
 *
 * Semantic operation signatures determine valid arity and parameter types.
 */

quantumAdjointParameterClause
    : LPAREN argumentList? RPAREN
    ;


/*
 * ============================================================================
 * 6. COMPOSABLE OPERATION MODIFIER REFERENCE
 * ============================================================================
 *
 * This rule is the integration point for other operation transformations.
 *
 * It intentionally references the canonical modifier productions rather than
 * creating another complete modifier grammar.
 *
 * Canonical downstream ownership:
 *
 *     controls.g4 / controlled-operation migration
 *         -> control transformation
 *
 *     inverse.g4 or the canonical inverse owner
 *         -> inverse transformation
 *
 *     adjoints.g4
 *         -> adjoint transformation
 *
 * Until those specialized owners are fully separated, the existing
 * `quantumOperationModifier` production in operations.g4 remains the
 * compatibility composition point.
 *
 * The final parser composition MUST expose exactly one canonical production
 * for each modifier kind.
 */

quantumOperationModifierReference
    : quantumOperationModifier
    ;


/*
 * ============================================================================
 * 7. REUSABLE ADJOINT REFERENCE
 * ============================================================================
 *
 * This rule provides a named integration point for parser consumers that need
 * an adjoint transformation without a complete `apply ...` statement.
 *
 * Example:
 *
 *     adjoint(U)
 *
 * The surrounding grammar decides whether the resulting reference is:
 *
 *     - invoked;
 *     - stored;
 *     - passed as an operation value;
 *     - composed with another operation;
 *     - used in metaprogramming;
 *     - lowered into a semantic operation reference.
 */

quantumAdjointOperationReferenceExpression
    : quantumAdjointModifier
    ;


/*
 * ============================================================================
 * 8. SOURCE-LEVEL OPERATION VALUE
 * ============================================================================
 *
 * This rule deliberately aliases the canonical adjoint modifier instead of
 * creating a second semantic representation.
 *
 * It exists as an explicit integration point for future operation-value
 * syntax.
 */

quantumAdjointOperationValue
    : quantumAdjointModifier
    ;


/*
 * ============================================================================
 * 9. STRUCTURAL COMPOSITION CONTRACT
 * ============================================================================
 *
 * The following structures are intentionally supported:
 *
 *     adjoint(H)
 *
 *     adjoint(RX(theta))
 *
 *     adjoint(U(theta, phi, lambda))
 *
 *     adjoint(custom::operation)
 *
 *     adjoint(custom::operation(parameter))
 *
 *     adjoint(control(U))
 *
 *     adjoint(inverse(U))
 *
 *     adjoint(adjoint(U))
 *
 *     adjoint(control(inverse(U)))
 *
 *     adjoint(inverse(control(adjoint(U))))
 *
 * The parser preserves the nesting.
 *
 * It does not simplify:
 *
 *     adjoint(adjoint(U))
 *
 * into:
 *
 *     U
 *
 * nor:
 *
 *     adjoint(inverse(U))
 *
 * into another form.
 *
 * Such transformations belong to semantic analysis/optimization.
 *
 * ============================================================================
 * 10. INVOCATION INTEGRATION
 * ============================================================================
 *
 * `operations.g4` remains the canonical owner of complete operation
 * invocation.
 *
 * The intended composition is:
 *
 *     quantumOperationInvocation
 *         :
 *         quantumOperationCallee
 *         quantumOperationParameterClause?
 *         quantumOperationTargetClause
 *         ;
 *
 * The canonical callee must be capable of consuming:
 *
 *     quantumAdjointModifier
 *
 * Conceptually:
 *
 *     apply adjoint(H)(q);
 *
 * parses as:
 *
 *     APPLY
 *       |
 *       +-- quantumAdjointModifier
 *       |      |
 *       |      +-- ADJOINT
 *       |      +-- H
 *       |
 *       +-- target clause
 *              |
 *              +-- q
 *
 * This file owns the first part.
 *
 * `operations.g4` owns the second part.
 *
 * ============================================================================
 * 11. PARAMETERIZED INVOCATION INTEGRATION
 * ============================================================================
 *
 * Example:
 *
 *     apply adjoint(RX(theta))(q);
 *
 * The intended parse structure is:
 *
 *     apply
 *       |
 *       adjoint
 *          |
 *          RX
 *          |
 *          theta
 *       |
 *       q
 *
 * The parameter:
 *
 *     theta
 *
 * is part of the operation reference.
 *
 * The target:
 *
 *     q
 *
 * is part of the invocation.
 *
 * They MUST remain structurally distinct.
 *
 * ============================================================================
 * 12. CONTROL INTEGRATION
 * ============================================================================
 *
 * The grammar permits:
 *
 *     adjoint(control(U))
 *
 * because the operand is compositional.
 *
 * Conversely:
 *
 *     control(adjoint(U))
 *
 * is owned structurally by the control transformation grammar.
 *
 * The semantic layer must preserve the ordering:
 *
 *     adjoint(control(U))
 *
 * is not automatically equivalent to:
 *
 *     control(adjoint(U))
 *
 * unless the operation semantics establish that equivalence.
 *
 * ============================================================================
 * 13. INVERSE INTEGRATION
 * ============================================================================
 *
 * The grammar permits:
 *
 *     adjoint(inverse(U))
 *
 * because inverse is another operation transformation.
 *
 * Semantic analysis determines the mathematical relationship.
 *
 * No parser-level rewrite is permitted.
 *
 * ============================================================================
 * 14. EXPRESSIONS IN PARAMETERS
 * ============================================================================
 *
 * Parameter expressions are supplied by the canonical expression grammar.
 *
 * Examples:
 *
 *     adjoint(RX(theta))
 *
 *     adjoint(RX(pi / 2))
 *
 *     adjoint(U(theta + phi, f(x), scale)))
 *
 * The adjoint grammar does not duplicate arithmetic, function-call, indexing,
 * member-access, tensor, or other expression syntax.
 *
 * ============================================================================
 * 15. QUANTUM TYPES
 * ============================================================================
 *
 * This file does not define:
 *
 *     Qubit
 *     Qubit[n]
 *     QuantumRegister
 *     LogicalQubit
 *     PhysicalQubit
 *     QuantumState
 *
 * Those remain owned by the quantum/type grammar.
 *
 * Whether the eventual operation target has a valid quantum type is a semantic
 * question.
 *
 * ============================================================================
 * 16. TARGET INTEGRATION
 * ============================================================================
 *
 * This file intentionally does NOT define:
 *
 *     quantumTargetClause
 *     quantumOperationTargetList
 *     qubitReference
 *     quantumRegisterElement
 *     quantumRegisterSlice
 *
 * Complete invocation remains owned by operations.g4 and the quantum reference
 * grammar.
 *
 * Therefore:
 *
 *     adjoint(U)
 *
 * is owned here.
 *
 *     apply adjoint(U)(q0, q1);
 *
 * is composed across:
 *
 *     adjoints.g4
 *     operations.g4
 *
 * ============================================================================
 * 17. RESOURCE INTEGRATION
 * ============================================================================
 *
 * An adjoint may require semantic capabilities or resources.
 *
 * Examples:
 *
 *     capability("quantum.adjoint")
 *
 *     capability("quantum.operation_transformation")
 *
 * The actual capability vocabulary is owned by the resource/capability
 * subsystem.
 *
 * This grammar does not embed capability names as semantic decisions.
 *
 * ============================================================================
 * 18. HARDWARE INTEGRATION
 * ============================================================================
 *
 * An adjoint may be:
 *
 *     - directly supported;
 *     - synthesized;
 *     - decomposed;
 *     - transformed;
 *     - routed;
 *     - scheduled;
 *     - executed through a simulator;
 *     - executed on a QPU;
 *     - lowered to another computational substrate.
 *
 * None of these choices changes source syntax.
 *
 * ============================================================================
 * 19. QUANTUM::IR INTEGRATION
 * ============================================================================
 *
 * After semantic validation:
 *
 *     quantumAdjointModifier
 *             |
 *             v
 *     semantic operation transformation
 *             |
 *             v
 *     quantum::ir
 *
 * The quantum IR must preserve enough information to distinguish:
 *
 *     operation
 *
 * from:
 *
 *     adjoint(operation)
 *
 * until a downstream transformation explicitly chooses to simplify or lower
 * it.
 *
 * ============================================================================
 * 20. OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization may transform:
 *
 *     adjoint(adjoint(U))
 *
 * into:
 *
 *     U
 *
 * where semantics establish the identity.
 *
 * Optimization may also transform:
 *
 *     adjoint(U)
 *
 * into:
 *
 *     a synthesized equivalent sequence.
 *
 * Such transformations MUST occur after parsing and semantic validation.
 *
 * This grammar must never perform them.
 *
 * ============================================================================
 * 21. QEC / ZQN / RESILIENCE INTEGRATION
 * ============================================================================
 *
 * This file has no knowledge of:
 *
 *     QEC code;
 *     code distance;
 *     syndrome extraction;
 *     decoder;
 *     noise model;
 *     leakage;
 *     fault model;
 *     reliability state;
 *     recovery strategy.
 *
 * Those systems may transform the canonical quantum::ir representation after
 * semantic analysis.
 *
 * Adjoint syntax must remain independent of those implementations.
 *
 * ============================================================================
 * 22. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Parser errors:
 *
 *     adjoint
 *     adjoint(
 *     adjoint()
 *     adjoint(
 *     adjoint(U
 *     adjoint(U(
 *     adjoint(U))
 *
 * are structural syntax errors.
 *
 * Semantic errors include:
 *
 *     adjoint(unknown_operation)
 *
 *     adjoint(non_adjointable_operation)
 *
 *     adjoint(measure)
 *
 *     adjoint(reset)
 *
 *     adjoint(operation_with_invalid_parameters)
 *
 * These MUST be rejected by semantic analysis rather than parser predicates.
 *
 * ============================================================================
 * 23. EMPTY PARAMETER CLAUSE
 * ============================================================================
 *
 * The grammar permits:
 *
 *     adjoint(U())
 *
 * structurally because:
 *
 *     argumentList?
 *
 * allows an empty parameter clause.
 *
 * Whether:
 *
 *     U()
 *
 * is semantically meaningful is determined by operation resolution.
 *
 * This follows the same principle used elsewhere in the quantum operation
 * grammar: syntax establishes structure; semantics establishes validity.
 *
 * ============================================================================
 * 24. TRAILING COMMAS
 * ============================================================================
 *
 * Whether trailing commas are accepted inside `argumentList` is delegated to
 * the canonical expression grammar.
 *
 * This file MUST NOT create a second argument-list policy.
 *
 * ============================================================================
 * 25. GENERICITY
 * ============================================================================
 *
 * Generic operation references remain possible through:
 *
 *     genericArgumentSuffix
 *
 * Example:
 *
 *     adjoint(operation::<QubitType>)
 *
 * Generic interpretation is downstream.
 *
 * ============================================================================
 * 26. DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Although this file belongs under:
 *
 *     grammar/quantum/
 *
 * its operation identity remains structurally generic.
 *
 * The semantic resolver decides whether the operation is:
 *
 *     quantum;
 *     logical;
 *     physical-intent;
 *     dialect-defined;
 *     imported;
 *     synthesized;
 *     user-defined.
 *
 * The quantum context supplies the semantic interpretation.
 *
 * ============================================================================
 * 27. FUTURE QUANTUM COMPUTATION
 * ============================================================================
 *
 * The grammar must remain applicable to future quantum models including:
 *
 *     superconducting
 *     trapped-ion
 *     neutral-atom
 *     photonic
 *     spin-based
 *     topological
 *     measurement-based
 *     analog
 *     distributed
 *     networked
 *     hybrid
 *     future quantum substrates
 *
 * No new adjoint syntax should be necessary merely because the physical
 * implementation changes.
 *
 * ============================================================================
 * 28. INTEROPERABILITY
 * ============================================================================
 *
 * External representations such as:
 *
 *     OpenQASM
 *     QIR
 *     other quantum IRs
 *     vendor formats
 *
 * may map their adjoint/dagger constructs into this semantic operation
 * transformation.
 *
 * Interoperability belongs downstream and MUST NOT turn an external format
 * into the Zamani grammar authority.
 *
 * ============================================================================
 * 29. DETERMINISTIC SOURCE MEANING
 * ============================================================================
 *
 * Parsing:
 *
 *     adjoint(U)
 *
 * must not inspect:
 *
 *     available QPUs;
 *     CPU count;
 *     GPU count;
 *     FPGA availability;
 *     runtime state;
 *     network state;
 *     calibration;
 *     current time;
 *     random state.
 *
 * Therefore source parsing remains deterministic and portable.
 *
 * ============================================================================
 * 30. SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Downstream Rust implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and:
 *
 *     no unsafe Rust.
 *
 * Repository Rust crates should enforce this where applicable with:
 *
 *     #![forbid(unsafe_code)]
 *
 * This grammar introduces no requirement for unsafe functionality.
 *
 * ============================================================================
 * 31. TEST CONTRACT — BASIC
 * ============================================================================
 *
 * Required positive syntax tests:
 *
 *     adjoint(H)
 *     adjoint(X)
 *     adjoint(custom_operation)
 *     adjoint(namespace::operation)
 *     adjoint(library::operation)
 *
 * ============================================================================
 * 32. TEST CONTRACT — PARAMETERIZED
 * ============================================================================
 *
 * Required positive syntax tests:
 *
 *     adjoint(RX(theta))
 *
 *     adjoint(RY(theta))
 *
 *     adjoint(RZ(theta))
 *
 *     adjoint(U(theta, phi, lambda))
 *
 *     adjoint(custom::operation(parameter))
 *
 *     adjoint(operation(f(x), y, z))
 *
 * The names above are test examples only.
 *
 * They do not create a fixed operation catalogue.
 *
 * ============================================================================
 * 33. TEST CONTRACT — INVOCATION
 * ============================================================================
 *
 * Required composed tests:
 *
 *     apply adjoint(H)(q);
 *
 *     apply adjoint(X)(q);
 *
 *     apply adjoint(RX(theta))(q);
 *
 *     apply adjoint(U(theta, phi, lambda))(q);
 *
 *     apply adjoint(custom::operation(parameter))(q0, q1);
 *
 * The target list remains owned by operations.g4.
 *
 * ============================================================================
 * 34. TEST CONTRACT — NESTING
 * ============================================================================
 *
 * Required positive tests:
 *
 *     adjoint(adjoint(U))
 *
 *     adjoint(inverse(U))
 *
 *     adjoint(control(U))
 *
 *     adjoint(control(adjoint(U)))
 *
 *     adjoint(inverse(control(U)))
 *
 *     adjoint(control(inverse(adjoint(U))))
 *
 * The nesting depth must not be hard-coded.
 *
 * ============================================================================
 * 35. TEST CONTRACT — SCALABILITY
 * ============================================================================
 *
 * The grammar must be tested with:
 *
 *     one modifier;
 *     multiple nested modifiers;
 *     large generated modifier trees;
 *     long qualified names;
 *     many operation parameters;
 *     large parameter expressions;
 *     large source programs.
 *
 * Test-environment resource limits must never become language limits.
 *
 * ============================================================================
 * 36. TEST CONTRACT — NEGATIVE
 * ============================================================================
 *
 * Required structural rejection cases include:
 *
 *     adjoint
 *
 *     adjoint(
 *
 *     adjoint()
 *
 *     adjoint(U
 *
 *     adjoint(U(
 *
 *     adjoint(U))
 *
 *     adjoint(U
 *
 *     adjoint(control(
 *
 *     adjoint(inverse(
 *
 *     adjoint(control(U)
 *
 * These should fail structurally where the canonical grammar requires a
 * missing delimiter or operand.
 *
 * ============================================================================
 * 37. TEST CONTRACT — SEMANTIC NEGATIVE
 * ============================================================================
 *
 * The following should parse structurally if their names and expressions are
 * syntactically valid:
 *
 *     adjoint(unknown_operation)
 *
 *     adjoint(non_adjointable_operation)
 *
 *     adjoint(measurement_operation)
 *
 *     adjoint(reset_operation)
 *
 * Semantic analysis is responsible for determining whether these are legal.
 *
 * ============================================================================
 * 38. TEST CONTRACT — DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *     parser configuration;
 *
 * the parse structure must be identical.
 *
 * The result must not depend on:
 *
 *     hardware;
 *     QPU availability;
 *     CPU count;
 *     GPU count;
 *     memory size;
 *     network;
 *     filesystem;
 *     wall clock;
 *     randomness.
 *
 * ============================================================================
 * 39. TEST CONTRACT — SOURCE SPANS
 * ============================================================================
 *
 * The frontend integration must preserve source spans for:
 *
 *     adjoint keyword;
 *     opening parenthesis;
 *     operation designator;
 *     generic arguments;
 *     parameter clause;
 *     nested modifier;
 *     closing parenthesis;
 *     complete modifier.
 *
 * This supports diagnostics, IDE tooling, formatting, and provenance.
 *
 * ============================================================================
 * 40. COMPATIBILITY WITH EXISTING OPERATIONS.G4
 * ============================================================================
 *
 * EXISTING STATE
 * -------------
 *
 * `grammar/quantum/operations.g4` currently defines:
 *
 *     quantumOperationModifier
 *     quantumOperationModifierOperand
 *     quantumAdjointOperation
 *
 * including direct ADJOINT alternatives.
 *
 * That is overlapping ownership.
 *
 * FINAL STATE
 * ----------
 *
 * `adjoints.g4` becomes the canonical owner of:
 *
 *     quantumAdjointModifier
 *     quantumAdjointModifierOperand
 *     quantumAdjointOperationReference
 *
 * `operations.g4` remains the canonical owner of operation invocation.
 *
 * During integration, the ADJOINT alternative in the generic modifier rule
 * must delegate to:
 *
 *     quantumAdjointModifier
 *
 * rather than reimplementing:
 *
 *     ADJOINT LPAREN ... RPAREN
 *
 * The old:
 *
 *     quantumAdjointOperation
 *
 * production should become a compatibility alias or be retired through the
 * normal grammar compatibility process if no external consumer requires it.
 *
 * Do NOT silently create two canonical adjoint productions.
 *
 * ============================================================================
 * 41. COMPATIBILITY WITH EXPRESSIONS/QUANTUM.G4
 * ============================================================================
 *
 * `grammar/expressions/quantum.g4` currently exposes quantum-specific
 * expression forms including:
 *
 *     quantumAdjointExpression
 *     quantumAdjointModifier
 *
 * That expression grammar must not become a second canonical adjoint syntax
 * owner.
 *
 * Its eventual integration target is:
 *
 *     quantumAdjointExpression
 *         ->
 *     canonical quantumAdjointModifier
 *
 * The expression layer remains responsible for deciding where an adjoint
 * operation value can occur as an expression.
 *
 * It does not redefine the token sequence.
 *
 * ============================================================================
 * 42. COMPATIBILITY WITH GATES.G4
 * ============================================================================
 *
 * `grammar/quantum/gates.g4` currently has:
 *
 *     adjointModifier
 *
 * as part of its legacy gate-specific modifier vocabulary.
 *
 * That vocabulary must not become another canonical quantum adjoint syntax.
 *
 * Gate syntax should eventually compose through the universal operation
 * transformation model.
 *
 * Standard gate names remain semantic/library vocabulary rather than a reason
 * to create a second adjoint grammar.
 *
 * ============================================================================
 * 43. COMPATIBILITY WITH CONTROLLED-OPERATIONS.G4
 * ============================================================================
 *
 * `grammar/quantum/controlled-operations.g4` references inverse/adjoint
 * concepts but must not own the adjoint token sequence.
 *
 * Controlled-operation syntax may compose with:
 *
 *     quantumAdjointModifier
 *
 * without redefining it.
 *
 * ============================================================================
 * 44. COMPATIBILITY WITH PARAMETERIZED-OPERATIONS.G4
 * ============================================================================
 *
 * Parameterized operation syntax remains the responsibility of the
 * parameterized-operation subsystem.
 *
 * This file only supplies the composition:
 *
 *     adjoint
 *       +
 *     operation reference
 *
 * Example:
 *
 *     adjoint(operation(parameter))
 *
 * The actual parameter expression remains owned by the canonical expression
 * grammar.
 *
 * ============================================================================
 * 45. COMPATIBILITY WITH QUANTUM.G4
 * ============================================================================
 *
 * `grammar/quantum/quantum.g4` remains the quantum composition/orchestration
 * boundary.
 *
 * It must consume the adjoint construct through the canonical specialized
 * production rather than redefining adjoint syntax.
 *
 * The desired dependency direction is:
 *
 *     lexical vocabulary
 *          |
 *          v
 *     names / expressions
 *          |
 *          v
 *     adjoints.g4
 *          |
 *          v
 *     quantum.g4 composition
 *          |
 *          v
 *     Zamani parser
 *
 * ============================================================================
 * 46. NO COMPETING QUANTUM IR
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     QuantumAdjointInstruction
 *     QuantumAdjointOperationIR
 *     AdjointGateIR
 *
 * The only canonical quantum IR boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * 47. NO HARDWARE LIMITS
 * ============================================================================
 *
 * This file has no language-level limits on:
 *
 *     operation count;
 *     parameter count;
 *     target count;
 *     qubit count;
 *     register size;
 *     device count;
 *     modifier depth;
 *     circuit depth;
 *     program size.
 *
 * Practical parser/compilation resource limits are implementation concerns.
 *
 * ============================================================================
 * 48. NO VENDOR LOCK-IN
 * ============================================================================
 *
 * This grammar must remain valid if a new vendor, architecture, simulator,
 * accelerator, quantum technology, or future computational substrate is added.
 *
 * Adding:
 *
 *     vendor::new_operation
 *
 * must not require changing this grammar.
 *
 * ============================================================================
 * 49. NO OPERATION ENUMERATION
 * ============================================================================
 *
 * NEVER add:
 *
 *     adjointOperation
 *         : H
 *         | X
 *         | Y
 *         | Z
 *         | CNOT
 *         | ...
 *
 * Operation identity is open.
 *
 * ============================================================================
 * 50. FEATURE-LIFECYCLE CONTRACT
 * ============================================================================
 *
 * This grammar is the syntax implementation of the adjoint feature.
 *
 * The complete feature lifecycle is:
 *
 *     specification
 *         |
 *         v
 *     AST contract
 *         |
 *         v
 *     adjoints.g4
 *         |
 *         v
 *     parser
 *         |
 *         v
 *     semantic validation
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization/lowering
 *         |
 *         v
 *     conformance tests
 *         |
 *         v
 *     stable feature
 *
 * `Zamani-Grammar.md` must not silently override this contract.
 *
 * ============================================================================
 * 51. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file itself is complete when:
 *
 * [x] It has a single documented ownership boundary.
 *
 * [x] It owns adjoint transformation syntax.
 *
 * [x] It does not own complete operation invocation.
 *
 * [x] It does not own operation targets.
 *
 * [x] It does not own qubit references.
 *
 * [x] It does not own quantum types.
 *
 * [x] It does not own expressions.
 *
 * [x] It does not own generic argument syntax.
 *
 * [x] It does not own lexer tokens.
 *
 * [x] It does not enumerate quantum gates.
 *
 * [x] It supports open operation names.
 *
 * [x] It supports qualified operation names.
 *
 * [x] It supports generic operation references.
 *
 * [x] It supports parameterized operation references.
 *
 * [x] It supports nested transformations.
 *
 * [x] It preserves transformation ordering.
 *
 * [x] It introduces no hardware limits.
 *
 * [x] It introduces no resource limits.
 *
 * [x] It introduces no vendor dependency.
 *
 * [x] It introduces no parser semantic predicates.
 *
 * [x] It introduces no embedded Rust.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It preserves deterministic parsing.
 *
 * [x] It documents AST integration.
 *
 * [x] It documents semantic integration.
 *
 * [x] It documents quantum::ir integration.
 *
 * [x] It documents optimization integration.
 *
 * [x] It documents QEC/ZQN/resilience boundaries.
 *
 * [x] It documents POCO-REAF compatibility.
 *
 * [x] It documents scalability behavior.
 *
 * [x] It documents hard-coding policy.
 *
 * [x] It documents diagnostics.
 *
 * [x] It documents compatibility migration.
 *
 * [ ] `operations.g4` delegates canonical ADJOINT syntax to this file.
 *
 * [ ] `expressions/quantum.g4` delegates its adjoint syntax to this file.
 *
 * [ ] `gates.g4` no longer acts as an independent canonical adjoint owner.
 *
 * [ ] `quantum.g4` composes this rule exactly once.
 *
 * [ ] ANTLR composition succeeds.
 *
 * [ ] Rust parser generation succeeds under the repository's Rust 1.97/1.97.1
 *     toolchain.
 *
 * [ ] Frontend AST conformance succeeds.
 *
 * [ ] Semantic adjoint validation succeeds.
 *
 * [ ] quantum::ir lowering succeeds.
 *
 * [ ] Positive tests succeed.
 *
 * [ ] Negative tests succeed.
 *
 * [ ] Boundary tests succeed.
 *
 * [ ] Scalability tests succeed.
 *
 * [ ] Determinism tests succeed.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The source language says:
 *
 *     "apply the adjoint transformation of this operation"
 *
 * It does NOT say:
 *
 *     "use this particular physical inverse instruction on this particular
 *      machine."
 *
 * Therefore:
 *
 *     adjoint syntax
 *          !=
 *     physical implementation
 *
 *     operation identity
 *          !=
 *     hardware identity
 *
 *     grammar cardinality
 *          !=
 *     hardware capacity
 *
 *     parser validity
 *          !=
 *     resource availability
 *
 * The canonical architecture remains:
 *
 *     Source
 *       |
 *       v
 *     Lexer
 *       |
 *       v
 *     Parser
 *       |
 *       v
 *     Domain-neutral AST
 *       |
 *       v
 *     Semantic analysis
 *       |
 *       v
 *     canonical quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * with no artificial machine-size ceiling introduced by this grammar.
 *
 * ============================================================================
 */