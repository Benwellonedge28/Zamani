/*
 * Zamani Programming Language
 * File: grammar/quantum/controlled-operations.g4
 *
 * Purpose
 * -------
 * Grammar for controlled and conditionally-controlled quantum operations.
 *
 * Architectural ownership
 * -----------------------
 * This file owns the SYNTAX of quantum operation-control modifiers:
 *
 *   - one or more quantum controls
 *   - control polarity
 *   - control state specifications
 *   - control-register/range references
 *   - control-value expressions where the language permits them
 *   - composition of controls with an operation invocation
 *
 * This file does NOT own:
 *
 *   - quantum operation definitions
 *   - ordinary operation invocation
 *   - gate definitions
 *   - parameter declarations
 *   - measurement syntax
 *   - reset syntax
 *   - observables
 *   - quantum registers
 *   - qubit declarations
 *   - logical/physical qubit identity semantics
 *   - hardware topology
 *   - routing
 *   - scheduling
 *   - QEC
 *   - ZQN/noise semantics
 *   - optimization
 *   - runtime dispatch
 *   - hardware discovery
 *   - quantum::ir
 *
 * The parser recognizes syntax only. Semantic analysis must validate
 * whether a controlled operation is legal for the referenced operation,
 * controls, target types, dimensions, arity, modifiers, and execution model.
 *
 * No machine-size assumptions are encoded here.
 *
 * No fixed number of controls is encoded.
 *
 * No fixed number of qubits is encoded.
 *
 * No physical topology is encoded.
 *
 * No Rust unsafe code is involved. This is an ANTLR grammar artifact;
 * the Rust implementation consuming its parse tree must remain safe Rust
 * under Rust 1.97 / 1.97.1.
 *
 * Integration boundary
 * -------------------
 *
 * Source
 *   -> Zamani lexer
 *   -> canonical Zamani parser
 *   -> parse tree
 *   -> syntax AST
 *   -> semantic analysis
 *   -> canonical quantum::ir
 *   -> optimization / routing / scheduling / QEC / ZQN / hardware / runtime
 *
 * This file must never import or depend on quantum::ir directly.
 *
 * Token names below intentionally use the shared lexical concepts expected
 * from the canonical Zamani grammar. If the canonical lexer gives these
 * concepts different token names, token aliases must be established in the
 * lexer/token authority layer rather than changing the semantic ownership
 * of this grammar.
 */

/*
 * --------------------------------------------------------------------------
 * CONTROLLED OPERATION
 * --------------------------------------------------------------------------
 *
 * A controlled operation is represented syntactically as:
 *
 *   control-spec operation-invocation
 *
 * Multiple control-spec elements may be chained:
 *
 *   control q[0] control q[1] operation(...)
 *
 * or represented by one aggregate control clause:
 *
 *   control(q[0], q[1]) operation(...)
 *
 * The exact surface form is intentionally extensible.
 *
 * The semantic layer determines:
 *
 *   - whether the operation accepts controls
 *   - how many controls it accepts
 *   - whether controls may overlap targets
 *   - whether controls must be distinct
 *   - whether controls may be classical
 *   - whether controls are quantum or classical
 *   - whether polarity is supported
 *   - whether a controlled operation maps to a native gate
 *   - whether decomposition is required
 *
 * Therefore no such restrictions belong in this grammar.
 */

controlledOperation
    : controlledOperationPrefix operationInvocation
    ;

/*
 * One or more control prefixes.
 *
 * The '+' quantifier is deliberately unbounded.
 * There is no MAX_CONTROLS or equivalent grammar restriction.
 */
controlledOperationPrefix
    : controlPrefix+
    ;

/*
 * --------------------------------------------------------------------------
 * CONTROL PREFIX
 * --------------------------------------------------------------------------
 *
 * Canonical basic forms:
 *
 *   control q
 *   control q[expr]
 *   control(q, r)
 *
 * Optional polarity/state syntax is supported independently.
 */
controlPrefix
    : CONTROL controlSpec
    ;

/*
 * --------------------------------------------------------------------------
 * CONTROL SPECIFICATION
 * --------------------------------------------------------------------------
 *
 * A control may be:
 *
 *   - a single quantum reference
 *   - a collection/range of quantum references
 *   - a structured control group
 *   - a control with an explicit polarity/state
 *
 * The grammar deliberately does not decide whether a particular reference
 * denotes a logical qubit, physical qubit, register element, symbolic
 * qubit, or another quantum resource. That belongs to semantic analysis.
 */
controlSpec
    : controlStateSpecifier? controlTarget
    ;

/*
 * A single control target or an aggregate control target.
 */
controlTarget
    : quantumReference
    | controlTargetGroup
    ;

/*
 * Aggregate controls permit an arbitrary number of controls without
 * introducing fixed-size machine assumptions.
 */
controlTargetGroup
    : LPAREN controlTargetList? RPAREN
    ;

/*
 * Zero elements are syntactically representable so that diagnostics can
 * distinguish:
 *
 *   control()
 *
 * from malformed token sequences.
 *
 * Semantic validation must reject an empty control set where an actual
 * control is required.
 */
controlTargetList
    : controlTarget (COMMA controlTarget)*
    ;

/*
 * --------------------------------------------------------------------------
 * CONTROL POLARITY / STATE
 * --------------------------------------------------------------------------
 *
 * A controlled operation may be active when a control is:
 *
 *   - logically true / |1>
 *   - logically false / |0>
 *   - explicitly specified by a state expression
 *
 * Polarity is syntax, not hardware behavior.
 */
controlStateSpecifier
    : controlPolarity
    | controlStateExpression
    ;

/*
 * Canonical polarity forms.
 *
 * The language may expose aliases in the lexical layer, but semantic
 * interpretation remains centralized.
 */
controlPolarity
    : POSITIVE
    | NEGATIVE
    ;

/*
 * General control-state expressions allow future quantum-state models
 * without requiring this grammar to be rewritten whenever the state model
 * evolves.
 */
controlStateExpression
    : LBRACKET expression RBRACKET
    ;

/*
 * --------------------------------------------------------------------------
 * QUANTUM REFERENCE
 * --------------------------------------------------------------------------
 *
 * This is intentionally a reference to the canonical quantum-reference
 * grammar/type system.
 *
 * Do NOT define QubitId, PhysicalQubitId, register declarations, or quantum
 * types here.
 *
 * The canonical quantum reference grammar owns those concepts.
 */
quantumReference
    : quantumNameReference
    | quantumIndexedReference
    | quantumSliceReference
    ;

/*
 * Named quantum value/reference.
 */
quantumNameReference
    : qualifiedName
    ;

/*
 * Indexed quantum resource:
 *
 *   q[index]
 *
 * The index is an expression, not a hard-coded integer.
 */
quantumIndexedReference
    : qualifiedName LBRACK expression RBRACK
    ;

/*
 * Quantum range/slice reference.
 *
 * Example conceptual forms:
 *
 *   q[start:end]
 *   q[start:end:step]
 *
 * The exact slice syntax is shared with the canonical indexing/range
 * grammar and must not acquire quantum-specific limits here.
 */
quantumSliceReference
    : qualifiedName LBRACK rangeExpression RBRACK
    ;

/*
 * --------------------------------------------------------------------------
 * OPERATION INVOCATION INTEGRATION
 * --------------------------------------------------------------------------
 *
 * `operationInvocation` is owned by operations.g4.
 *
 * It is referenced here but deliberately not redefined.
 *
 * This prevents:
 *
 *   operations.g4
 *       ^
 *       |
 * controlled-operations.g4
 *
 * from becoming mutually recursive semantic owners.
 *
 * The parser composition layer supplies operationInvocation.
 */

/*
 * --------------------------------------------------------------------------
 * CONTROLLED OPERATION DEFINITIONS
 * --------------------------------------------------------------------------
 *
 * This grammar permits declarations of controlled-operation forms only
 * through the generic operation/gate declaration system.
 *
 * It does NOT introduce a second gate-definition language.
 *
 * A dedicated rule is retained as an integration hook for parsers that
 * distinguish invocation syntax from declaration syntax.
 */
controlledOperationDeclaration
    : CONTROLLED operationDeclaration
    ;

/*
 * `operationDeclaration` is owned by the gate/operation declaration
 * subsystem and must be imported rather than duplicated.
 *
 * If the canonical Zamani grammar does not expose operationDeclaration,
 * this rule should be omitted from the composed parser and controlled
 * operation declarations should remain a semantic modifier on ordinary
 * operation declarations.
 */

/*
 * --------------------------------------------------------------------------
 * CONTROLLED OPERATION MODIFIERS
 * --------------------------------------------------------------------------
 *
 * Controlled operations may be composed with other operation modifiers.
 *
 * The modifier system belongs to the operation layer.
 *
 * This grammar only provides the controlled-operation portion.
 */
controlledOperationModifier
    : controlPrefix
    ;

/*
 * --------------------------------------------------------------------------
 * CONTROLLED OPERATION SEQUENCES
 * --------------------------------------------------------------------------
 *
 * Useful when a circuit contains a sequence of controlled operations.
 *
 * Sequence ownership remains with circuits.g4 / operations.g4.
 */
controlledOperationSequence
    : controlledOperation+
    ;

/*
 * --------------------------------------------------------------------------
 * CONTROLLED OPERATION EXPRESSION
 * --------------------------------------------------------------------------
 *
 * This rule provides an expression-level integration point without
 * assigning execution semantics to the grammar.
 */
controlledOperationExpression
    : controlledOperation
    ;

/*
 * --------------------------------------------------------------------------
 * CONTROL CONDITIONS
 * --------------------------------------------------------------------------
 *
 * A quantum operation can additionally be controlled by a classical
 * condition.
 *
 * IMPORTANT:
 *
 * Classical conditions are NOT owned by this file.
 *
 * This rule is an integration point only.
 *
 * The canonical hybrid / dynamic-circuit grammar owns the actual condition
 * syntax.
 */
classicallyControlledOperation
    : classicalControlPrefix operationInvocation
    ;

classicalControlPrefix
    : IF LPAREN expression RPAREN
    ;

/*
 * --------------------------------------------------------------------------
 * HYBRID CONTROL
 * --------------------------------------------------------------------------
 *
 * Hybrid control permits explicit composition of quantum and classical
 * control.
 *
 * The grammar intentionally permits multiple control layers while semantic
 * validation determines whether a particular combination is meaningful.
 */
hybridControlledOperation
    : hybridControlPrefix+ operationInvocation
    ;

hybridControlPrefix
    : quantumControlPrefix
    | classicalControlPrefix
    ;

quantumControlPrefix
    : CONTROL controlSpec
    ;

/*
 * --------------------------------------------------------------------------
 * CONTROLLED OPERATION GROUP
 * --------------------------------------------------------------------------
 *
 * Parenthesized groups allow control syntax to remain composable without
 * imposing fixed arity.
 */
controlledOperationGroup
    : LPAREN controlledOperationSequence RPAREN
    ;

/*
 * --------------------------------------------------------------------------
 * PARAMETERIZED CONTROLLED OPERATIONS
 * --------------------------------------------------------------------------
 *
 * Parameter ownership remains with parameterized-operations.g4.
 *
 * This rule merely provides the composition point:
 *
 *   control q rotate(theta)
 *
 * The actual parameter binding semantics belong elsewhere.
 */
parameterizedControlledOperation
    : controlledOperation
    ;

/*
 * --------------------------------------------------------------------------
 * INVERSE / ADJOINT CONTROLLED OPERATIONS
 * --------------------------------------------------------------------------
 *
 * Inverse/adjoint syntax is represented as an operation modifier rather
 * than a distinct quantum operation type.
 *
 * The modifier vocabulary is owned by operations.g4 / gates.g4.
 */
adjointControlledOperation
    : operationModifierPrefix controlledOperation
    ;

/*
 * --------------------------------------------------------------------------
 * CONTROLLED CONTROLLED OPERATIONS
 * --------------------------------------------------------------------------
 *
 * Multiple controlPrefix elements already provide arbitrary control depth.
 *
 * No special "double-controlled", "triple-controlled", etc. grammar rules
 * are required.
 *
 * This prevents an artificial ceiling and avoids a growing collection of
 * special-case rules.
 */
multiControlledOperation
    : controlledOperationPrefix operationInvocation
    ;

/*
 * --------------------------------------------------------------------------
 * CONTROLLED OPERATION TARGET INTEGRATION
 * --------------------------------------------------------------------------
 *
 * Targets are owned by operations.g4.
 *
 * This rule exists as a semantic integration point for tooling and AST
 * construction.
 */
controlledOperationApplication
    : controlledOperation
    ;

/*
 * --------------------------------------------------------------------------
 * RANGE EXPRESSION INTEGRATION
 * --------------------------------------------------------------------------
 *
 * This is a reference to the canonical expression/range grammar.
 *
 * Do not duplicate the range grammar here.
 */
rangeExpression
    : expression
    ;

/*
 * --------------------------------------------------------------------------
 * SHARED CORE INTEGRATION
 * --------------------------------------------------------------------------
 *
 * The following names are expected to come from canonical shared grammar
 * components:
 *
 *   expression
 *   qualifiedName
 *   operationInvocation
 *   operationDeclaration
 *
 * They are intentionally not redefined here.
 *
 * Likewise the following lexical concepts must be supplied by the canonical
 * lexer:
 *
 *   CONTROL
 *   CONTROLLED
 *   POSITIVE
 *   NEGATIVE
 *   IF
 *   LPAREN
 *   RPAREN
 *   LBRACK
 *   RBRACK
 *   COMMA
 *
 * The canonical lexer is the authority for spelling, aliases, Unicode
 * handling, reserved-word policy, and token precedence.
 *
 * This file therefore cannot silently create conflicting tokens.
 */