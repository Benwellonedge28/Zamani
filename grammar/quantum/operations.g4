/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/operations.g4
 *
 * Role:
 *     Canonical reusable parser fragment for quantum-operation syntax.
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE SYNTAX of quantum operation invocation and
 * operation-level composition.
 *
 * It is deliberately a PARSER fragment.
 *
 * It does not implement quantum operations.
 *
 * It does not contain:
 *
 *     - Rust;
 *     - semantic actions;
 *     - hardware calls;
 *     - simulator calls;
 *     - IR construction;
 *     - backend selection;
 *     - resource allocation;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantum operation statements;
 *     - operation invocation;
 *     - operation designators;
 *     - operation arguments;
 *     - operation target lists;
 *     - operation modifiers;
 *     - controlled operation syntax;
 *     - adjoint operation syntax;
 *     - inverse operation syntax;
 *     - operation composition syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer vocabulary;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - general types;
 *     - qubit declarations;
 *     - register declarations;
 *     - gate declarations;
 *     - gate matrices;
 *     - gate definitions;
 *     - measurements;
 *     - reset;
 *     - observables;
 *     - dynamic-circuit control;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware;
 *     - topology;
 *     - calibration;
 *     - runtime execution;
 *     - canonical quantum IR.
 *
 * ============================================================================
 * CANONICAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +---- quantum/operations.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +---- name resolution
 *          +---- type checking
 *          +---- effect checking
 *          +---- capability checking
 *          +---- resource analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +---- quantum::ir
 *          |
 *          +---- optimization
 *          +---- routing
 *          +---- scheduling
 *          +---- QEC
 *          +---- ZQN
 *          +---- resilience
 *          +---- hardware HAL
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     execution
 *
 * This grammar MUST NOT bypass the AST/semantic boundary.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A quantum operation describes WHAT the program intends to do.
 *
 * It does not describe HOW a particular machine must implement it.
 *
 * Therefore this file MUST NOT encode:
 *
 *     - maximum operation count;
 *     - maximum target count;
 *     - maximum control count;
 *     - maximum parameter count;
 *     - maximum circuit depth;
 *     - maximum qubit count;
 *     - maximum register count;
 *     - physical qubit numbers;
 *     - topology;
 *     - coupling maps;
 *     - device IDs;
 *     - vendor IDs;
 *     - native gate sets;
 *     - pulse durations;
 *     - calibration;
 *     - backend selection;
 *     - simulator size.
 *
 * There are no finite cardinality bounds in this grammar.
 *
 * The practical limits are determined by:
 *
 *     source size;
 *     parser resources;
 *     compiler resources;
 *     semantic constraints;
 *     available hardware;
 *     runtime resources.
 *
 * Those are NOT language-level limits.
 *
 * ============================================================================
 * LEXICAL INTEGRATION
 * ============================================================================
 *
 * The current canonical Zamani keyword vocabulary already owns:
 *
 *     APPLY
 *     CONTROL
 *     ADJOINT
 *     INVERSE
 *     MEASURE
 *     RESET
 *     BARRIER
 *     OBSERVE
 *
 * This grammar consumes those existing tokens.
 *
 * It intentionally does NOT introduce:
 *
 *     K_APPLY
 *     K_CONTROL
 *     K_ADJOINT
 *     K_INVERSE
 *     TO
 *     POWER_OPERATOR
 *     SEMI
 *
 * or any other duplicate vocabulary.
 *
 * Gate names remain ordinary identifiers/qualified names.
 *
 * Therefore all of these remain possible without modifying this file:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     SWAP
 *     custom_gate
 *     library::operation
 *     vendor::operation
 *     future::operation
 *
 * The semantic layer determines what each name means.
 *
 * ============================================================================
 * OPERATION SURFACE
 * ============================================================================
 *
 * Canonical operation invocation:
 *
 *     apply H(q);
 *
 *     apply X(q);
 *
 *     apply CNOT(control, target);
 *
 * Parameterized operation:
 *
 *     apply RX(theta)(q);
 *
 * Multiple parameters:
 *
 *     apply U(theta, phi, lambda)(q);
 *
 * Multiple targets:
 *
 *     apply SWAP(q0, q1);
 *
 * Qualified operation:
 *
 *     apply library::operation(q);
 *
 * Generic operation:
 *
 *     apply operation<T>(q);
 *
 * Controlled operation:
 *
 *     apply control(X)(control, target);
 *
 * Adjoint:
 *
 *     apply adjoint(U)(q);
 *
 * Inverse:
 *
 *     apply inverse(U)(q);
 *
 * Modifiers may be nested:
 *
 *     apply control(adjoint(U))(control, target);
 *
 * No finite nesting limit is encoded.
 *
 * ============================================================================
 * IMPORTANT SYNTAX DECISION
 * ============================================================================
 *
 * The current lexer does not reserve `to` as a quantum keyword.
 *
 * Therefore the canonical operation form is:
 *
 *     apply OPERATION(TARGETS);
 *
 * rather than:
 *
 *     apply OPERATION to TARGETS;
 *
 * This avoids introducing a special quantum keyword solely for operation
 * syntax and keeps the lexer stable.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. CANONICAL QUANTUM OPERATION STATEMENT
 * ========================================================================== */

/*
 * Examples:
 *
 *     apply H(q);
 *
 *     apply X(q);
 *
 *     apply CNOT(q0, q1);
 *
 *     apply RX(theta)(q);
 *
 *     apply library::operation(q);
 *
 * The final semicolon belongs to the surrounding language punctuation model.
 */
quantumOperationStatement
    : APPLY quantumOperationInvocation SEMICOLON
    ;


/* ============================================================================
 * 2. OPERATION INVOCATION
 * ========================================================================== */

/*
 * Two syntactic forms are intentionally supported:
 *
 *     operation(targets)
 *
 * and:
 *
 *     operation(arguments)(targets)
 *
 * This permits:
 *
 *     apply H(q);
 *
 * and:
 *
 *     apply RX(theta)(q);
 *
 * without requiring the parser to know whether H/RX is a built-in gate.
 */
quantumOperationInvocation
    : quantumOperationDesignator
      quantumOperationArguments?
      quantumOperationTargetClause
    ;


/* ============================================================================
 * 3. OPERATION DESIGNATOR
 * ========================================================================== */

/*
 * An operation is named structurally.
 *
 * The name may be:
 *
 *     simple;
 *     qualified;
 *     generic.
 *
 * Meaning is resolved later.
 */
quantumOperationDesignator
    : qualifiedName
      quantumOperationTypeArguments?
    ;


/* ============================================================================
 * 4. GENERIC OPERATION ARGUMENTS
 * ========================================================================== */

/*
 * Delegates generic syntax to the canonical type/generic grammar.
 *
 * Example:
 *
 *     operation<Qubit>(q)
 *
 * or another semantically valid generic operation form.
 */
quantumOperationTypeArguments
    : genericArguments
    ;


/* ============================================================================
 * 5. VALUE/PARAMETER ARGUMENTS
 * ========================================================================== */

/*
 * Operation parameters are ordinary Zamani expressions.
 *
 * Examples:
 *
 *     theta
 *
 *     theta + phi
 *
 *     parameter
 *
 *     configuration.angle
 *
 *     compile_time_value
 *
 * No parameter count is hard-coded.
 */
quantumOperationArguments
    : LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 6. TARGET CLAUSE
 * ========================================================================== */

/*
 * Targets are ordinary expressions.
 *
 * This is important because:
 *
 *     q
 *
 *     q[i]
 *
 *     q[start .. end]
 *
 *     register_view
 *
 *     expression-derived selections
 *
 * can all be represented without creating a second quantum reference grammar.
 *
 * The semantic layer determines whether an expression is actually a valid
 * quantum target.
 */
quantumOperationTargetClause
    : LPAREN quantumOperationTargetList? RPAREN
    ;


quantumOperationTargetList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 7. OPERATION MODIFIER
 * ========================================================================== */

/*
 * Existing language-level quantum modifiers are recursively composable.
 *
 * There is intentionally no:
 *
 *     MAX_CONTROL_DEPTH
 *
 *     MAX_MODIFIER_DEPTH
 *
 *     MAX_TARGETS
 *
 * The semantic layer determines whether a particular modifier combination is
 * meaningful.
 */
quantumOperationModifier
    : CONTROL LPAREN quantumOperationModifierBody RPAREN
    | ADJOINT LPAREN quantumOperationModifierBody RPAREN
    | INVERSE LPAREN quantumOperationModifierBody RPAREN
    ;


quantumOperationModifierBody
    : quantumOperationDesignator
    | quantumOperationModifier
    ;


/* ============================================================================
 * 8. MODIFIED OPERATION INVOCATION
 * ========================================================================== */

/*
 * This rule is deliberately separate from the canonical operation statement
 * so the parser can represent modifiers without introducing special gate
 * vocabularies.
 *
 * Examples:
 *
 *     apply control(X)(control, target);
 *
 *     apply adjoint(U)(q);
 *
 *     apply inverse(U)(q);
 *
 *     apply control(adjoint(U))(control, target);
 */
quantumModifiedOperationInvocation
    : quantumOperationModifier
      quantumOperationArguments?
      quantumOperationTargetClause
    ;


/* ============================================================================
 * 9. EXTENDED OPERATION INVOCATION
 * ========================================================================== */

/*
 * Public integration rule for callers that accept either an ordinary operation
 * or a modified operation.
 */
quantumExtendedOperationInvocation
    : quantumModifiedOperationInvocation
    | quantumOperationInvocation
    ;


/* ============================================================================
 * 10. CONTROLLED OPERATION
 * ========================================================================== */

/*
 * A controlled operation is expressed using the existing CONTROL token.
 *
 * Example:
 *
 *     apply control(X)(control, target);
 *
 * Multiple controls can be represented as multiple target expressions:
 *
 *     apply control(X)(c0, c1, target);
 *
 * The semantic operation signature determines which operands are controls and
 * which are operation targets.
 *
 * The grammar does NOT assume:
 *
 *     one control;
 *     two controls;
 *     three controls;
 *
 * or any other finite control count.
 */
quantumControlledOperation
    : CONTROL LPAREN quantumOperationDesignator RPAREN
    ;


/* ============================================================================
 * 11. ADJOINT OPERATION
 * ========================================================================== */

/*
 * Adjoint is a semantic operation transformation.
 *
 * The grammar records the source intent.
 *
 * The semantic layer must determine whether the referenced operation admits
 * an adjoint.
 */
quantumAdjointOperation
    : ADJOINT LPAREN quantumOperationDesignator RPAREN
    ;


/* ============================================================================
 * 12. INVERSE OPERATION
 * ========================================================================== */

/*
 * Inverse is kept separate from ADJOINT because semantic layers may distinguish
 * the concepts for particular operation classes.
 *
 * The grammar does not decide whether the two have equivalent semantics.
 */
quantumInverseOperation
    : INVERSE LPAREN quantumOperationDesignator RPAREN
    ;


/* ============================================================================
 * 13. OPERATION COMPOSITION
 * ========================================================================== */

/*
 * Operation composition is represented structurally as a sequence of
 * operation expressions.
 *
 * It does NOT perform scheduling.
 *
 * It does NOT specify hardware timing.
 *
 * It does NOT select a topology.
 *
 * The semantic/IR layer establishes the actual dependency representation.
 */
quantumOperationComposition
    : LPAREN
      quantumOperationExpression
      (COMMA quantumOperationExpression)*
      COMMA?
      RPAREN
    ;


/* ============================================================================
 * 14. OPERATION EXPRESSION
 * ========================================================================== */

/*
 * This is an expression-level operation abstraction.
 *
 * It is intentionally name-based and extensible.
 */
quantumOperationExpression
    : quantumOperationDesignator
    | quantumControlledOperation
    | quantumAdjointOperation
    | quantumInverseOperation
    | quantumOperationComposition
    ;


/* ============================================================================
 * 15. OPERATION REFERENCE
 * ========================================================================== */

/*
 * Explicit reusable alias for semantic consumers.
 *
 * This rule contains no semantic resolution.
 */
quantumOperationReference
    : quantumOperationDesignator
    ;


/* ============================================================================
 * 16. OPERATION TARGET
 * ========================================================================== */

/*
 * Reusable target wrapper.
 *
 * The expression grammar remains the owner of indexing/member access/ranges.
 */
quantumOperationTarget
    : expression
    ;


/* ============================================================================
 * 17. OPERATION TARGET LIST
 * ========================================================================== */

/*
 * Alias for consumers that want an explicitly named operation target list.
 *
 * No finite target count is imposed.
 */
quantumOperationTargets
    : quantumOperationTarget
      (COMMA quantumOperationTarget)*
      COMMA?
    ;


/* ============================================================================
 * 18. OPERATION APPLICATION CORE
 * ========================================================================== */

/*
 * This rule excludes the terminating semicolon so it can be reused by:
 *
 *     quantum blocks;
 *     generated statement lists;
 *     future operation wrappers;
 *     dialect extensions.
 */
quantumOperationApplication
    : APPLY quantumExtendedOperationInvocation
    ;


/* ============================================================================
 * 19. OPERATION STATEMENT WITH ATTRIBUTES
 * ========================================================================== */

/*
 * Attributes remain owned by the canonical attribute grammar.
 *
 * This rule only establishes their attachment point.
 */
quantumAttributedOperationStatement
    : attributes*
      quantumOperationStatement
    ;


/* ============================================================================
 * 20. OPERATION COMPOSITION CONTRACT
 * ========================================================================== */

/*
 * Composition is semantic intent.
 *
 * For example:
 *
 *     compose(A, B, C)
 *
 * MUST NOT mean:
 *
 *     execute immediately on hardware;
 *
 *     insert fixed delays;
 *
 *     choose a coupling map;
 *
 *     allocate physical qubits.
 *
 * The semantic layer converts composition into the canonical representation
 * consumed by quantum::ir and later compiler stages.
 *
 * If the language exposes a user-level `compose` operation, it remains an
 * ordinary operation name and does not require a new lexer keyword.
 */


/* ============================================================================
 * 21. REPETITION / POWER CONTRACT
 * ========================================================================== */

/*
 * IMPORTANT:
 *
 * There is deliberately no POWER_OPERATOR token here because the current
 * canonical operator lexer does not define one.
 *
 * Repetition/power is therefore represented through ordinary operation
 * semantics rather than inventing another lexical token.
 *
 * For example, a future/core library may expose:
 *
 *     power(operation, exponent)
 *
 * or:
 *
 *     repeat(operation, count)
 *
 * and the operation remains syntactically valid because the designator and
 * argument grammar are generic.
 *
 * The semantic layer determines:
 *
 *     - whether the operation is repeatable;
 *     - whether the exponent is valid;
 *     - whether repetition is exact;
 *     - whether exponentiation has mathematical meaning;
 *     - whether the target supports it.
 *
 * This keeps operations.g4 independent from a particular algebraic notation.
 */


/* ============================================================================
 * 22. CLASSICAL CONTROL BOUNDARY
 * ========================================================================== */

/*
 * Classical conditions are NOT owned here.
 *
 * Dynamic-circuit syntax belongs to:
 *
 *     quantum/mid-circuit-control.g4
 *
 * or the canonical statement/control-flow grammar.
 *
 * That separation prevents this file from duplicating:
 *
 *     if
 *     match
 *     measurement-result branching
 *     classical control-flow
 *
 * Operation syntax only supplies the operation that may be controlled.
 */


/* ============================================================================
 * 23. MEASUREMENT BOUNDARY
 * ========================================================================== */

/*
 * Measurement is NOT implemented as a special operation here.
 *
 * The canonical:
 *
 *     MEASURE
 *
 * token belongs to the measurement grammar.
 *
 * This prevents:
 *
 *     operations.g4
 *
 * from becoming a second owner of:
 *
 *     measurement semantics;
 *     measurement destinations;
 *     measurement bases;
 *     measurement result types.
 */


/* ============================================================================
 * 24. RESET BOUNDARY
 * ========================================================================== */

/*
 * RESET is similarly owned by the reset grammar.
 *
 * operations.g4 does not redefine:
 *
 *     reset
 *     reset targets
 *     reset semantics
 *     state initialization semantics
 */


/* ============================================================================
 * 25. GATE CATALOGUE BOUNDARY
 * ========================================================================== */

/*
 * DO NOT add rules such as:
 *
 *     H      : ...;
 *     X      : ...;
 *     Y      : ...;
 *     Z      : ...;
 *     CNOT   : ...;
 *     RX     : ...;
 *     RY     : ...;
 *     RZ     : ...;
 *
 * here.
 *
 * Gate names are semantic names.
 *
 * This is essential for:
 *
 *     - custom gates;
 *     - library gates;
 *     - future gates;
 *     - vendor operations;
 *     - dialect operations;
 *     - synthesized operations;
 *     - target-specific operations.
 *
 * The grammar therefore remains stable as quantum technology evolves.
 */


/* ============================================================================
 * 26. HARDWARE INDEPENDENCE
 * ========================================================================== */

/*
 * An operation target is NOT a physical allocation.
 *
 * For example:
 *
 *     apply CNOT(q0, q1);
 *
 * does NOT mean:
 *
 *     use physical qubit 0;
 *     use physical qubit 1;
 *     use a specific coupling edge;
 *     use a specific QPU;
 *     use a specific calibration;
 *     use a fixed duration.
 *
 * Instead:
 *
 *     q0 / q1
 *
 * are semantic quantum resources.
 *
 * Routing and hardware realization happen later.
 */


/* ============================================================================
 * 27. QUANTUM IR BOUNDARY
 * ========================================================================== */

/*
 * The parser MUST NOT construct quantum::ir.
 *
 * The intended lowering is:
 *
 *     quantumOperationStatement
 *          |
 *          v
 *     frontend AST node
 *          |
 *          v
 *     resolved operation
 *          |
 *          v
 *     canonical quantum::ir
 *
 * After this boundary:
 *
 *     optimization
 *     routing
 *     scheduling
 *     ZQN
 *     QEC
 *     resilience
 *     hardware HAL
 *
 * may consume the semantic representation.
 *
 * operations.g4 MUST NOT import or depend on Rust quantum IR implementation
 * modules.
 */


/* ============================================================================
 * 28. QEC BOUNDARY
 * ========================================================================== */

/*
 * QEC is not implemented in this grammar.
 *
 * A semantic operation may eventually require or interact with QEC, but the
 * grammar does not decide:
 *
 *     code family;
 *     distance;
 *     decoder;
 *     syndrome extraction;
 *     logical-to-physical mapping;
 *     correction strategy.
 *
 * Those remain QEC responsibilities.
 */


/* ============================================================================
 * 29. ZQN BOUNDARY
 * ========================================================================== */

/*
 * Noise is not encoded into operation syntax.
 *
 * ZQN owns:
 *
 *     fault classification;
 *     noise channels;
 *     correlated faults;
 *     leakage;
 *     loss;
 *     erasure;
 *     calibration/noise semantics.
 *
 * An operation may eventually be analyzed against ZQN information, but this
 * parser file does not define that information.
 */


/* ============================================================================
 * 30. ROUTING BOUNDARY
 * ========================================================================== */

/*
 * Operations refer to logical/source-level targets.
 *
 * Routing determines physical realization later.
 *
 * Therefore operations.g4 must never acquire rules such as:
 *
 *     operationOnQubit0
 *     operationOnQubit1
 *     nearestNeighborOperation
 *     couplingEdgeOperation
 *
 * or equivalent fixed-topology constructs.
 */


/* ============================================================================
 * 31. SCHEDULING BOUNDARY
 * ========================================================================== */

/*
 * Source ordering is not physical timing.
 *
 * This file does not encode:
 *
 *     gate duration;
 *     pulse duration;
 *     clock cycles;
 *     alignment slots;
 *     hardware barriers;
 *     scheduling policy.
 *
 * Scheduling consumes semantic operations later.
 */


/* ============================================================================
 * 32. OPTIMIZATION BOUNDARY
 * ========================================================================== */

/*
 * This grammar does not decide:
 *
 *     cancellation;
 *     peephole optimization;
 *     gate fusion;
 *     T-count reduction;
 *     decomposition;
 *     synthesis;
 *     layout optimization.
 *
 * These remain compiler/optimization responsibilities.
 */


/* ============================================================================
 * 33. RESILIENCE BOUNDARY
 * ========================================================================== */

/*
 * Runtime resilience may later decide to:
 *
 *     retry;
 *     reroute;
 *     reschedule;
 *     recompile;
 *     reoptimize;
 *     change QEC;
 *     mitigate;
 *     switch backend;
 *     quarantine a resource.
 *
 * None of those decisions belongs in this grammar.
 *
 * The source operation must remain semantically stable while realization
 * changes around it.
 */


/* ============================================================================
 * 34. AST CONTRACT
 * ========================================================================== */

/*
 * The frontend AST must preserve:
 *
 *     - operation name;
 *     - qualified-name segments;
 *     - generic arguments;
 *     - operation arguments;
 *     - target expressions;
 *     - modifier nesting;
 *     - source locations/spans;
 *     - source attributes.
 *
 * It MUST NOT silently replace these with:
 *
 *     physical gate;
 *     physical qubit;
 *     device;
 *     topology;
 *     calibration;
 *     backend.
 */


/* ============================================================================
 * 35. SEMANTIC VALIDATION CONTRACT
 * ========================================================================== */

/*
 * Semantic analysis MUST validate:
 *
 *     1. operation name resolution;
 *     2. generic argument validity;
 *     3. parameter types;
 *     4. target types;
 *     5. target cardinality;
 *     6. target aliasing;
 *     7. control/target overlap;
 *     8. operation reversibility;
 *     9. adjoint legality;
 *    10. inverse legality;
 *    11. operation ownership;
 *    12. quantum resource lifetime;
 *    13. linear/affine constraints;
 *    14. capability requirements;
 *    15. resource requirements;
 *    16. dynamic execution requirements;
 *    17. dialect compatibility.
 *
 * None of these checks should be encoded as hardware-specific parser rules.
 */


/* ============================================================================
 * 36. ERROR HANDLING CONTRACT
 * ========================================================================== */

/*
 * Syntax errors belong to parser diagnostics.
 *
 * Semantic errors belong to semantic diagnostics.
 *
 * Capability/resource errors belong to capability/resource analysis.
 *
 * Hardware realization failures belong downstream.
 *
 * Do not turn backend-specific failures into parser errors.
 *
 * For example:
 *
 *     apply exotic_operation(q);
 *
 * should remain syntactically valid even if a particular backend later cannot
 * execute it.
 *
 * The compiler may then:
 *
 *     decompose;
 *     synthesize;
 *     emulate;
 *     route;
 *     reject;
 *
 * according to semantic and target policy.
 */


/* ============================================================================
 * 37. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Given:
 *
 *     identical source;
 *     identical language version;
 *     identical lexer vocabulary;
 *
 * parsing must produce the same syntactic structure.
 *
 * This grammar contains no:
 *
 *     random behavior;
 *     runtime state;
 *     hardware queries;
 *     filesystem queries;
 *     network queries;
 *     semantic predicates;
 *     embedded Rust actions.
 */


/* ============================================================================
 * 38. NO MACHINE-SIZE ASSUMPTIONS
 * ========================================================================== */

/*
 * The following MUST NEVER appear in this file:
 *
 *     MAX_QUBITS
 *     MAX_TARGETS
 *     MAX_CONTROLS
 *     MAX_OPERATIONS
 *     MAX_PARAMETERS
 *     MAX_DEPTH
 *     MAX_REGISTERS
 *     MAX_DEVICES
 *     MAX_BACKENDS
 *     DEVICE_0
 *     DEVICE_1
 *     QUBIT_0
 *     QUBIT_1
 *
 * nor equivalent hidden finite bounds.
 *
 * All repetition uses unbounded grammar constructs.
 */


/* ============================================================================
 * 39. FUTURE EXTENSIBILITY
 * ========================================================================== */

/*
 * New quantum operations do NOT require changes here merely because a new
 * operation has been invented.
 *
 * For example:
 *
 *     future_gate
 *
 * is already a valid operation designator.
 *
 * This is intentional.
 *
 * The language therefore does not need a grammar release every time quantum
 * hardware introduces a new native operation.
 *
 * A new syntax keyword should be introduced only when there is a genuine
 * language-level syntactic need.
 */


/* ============================================================================
 * 40. DIALECT INTEGRATION
 * ========================================================================== */

/*
 * Dialects may define additional semantic operation names.
 *
 * A dialect should preferably provide:
 *
 *     namespace/name
 *
 * or another canonical qualified name rather than modifying this grammar for
 * every dialect operation.
 *
 * This preserves:
 *
 *     core grammar stability;
 *     vendor independence;
 *     forward compatibility;
 *     POCO-REAF.
 */


/* ============================================================================
 * 41. CROSS-DOMAIN INTEGRATION
 * ========================================================================== */

/*
 * Classical:
 *
 *     operation parameters are ordinary expressions.
 *
 * Quantum:
 *
 *     targets may resolve to quantum resources.
 *
 * HDL:
 *
 *     operation-like constructs must remain owned by HDL grammar where their
 *     semantics are hardware-specific.
 *
 * Distributed:
 *
 *     remote operation semantics belong to distributed execution grammar.
 *
 * AI:
 *
 *     accelerator/model operations remain names resolved by their owning
 *     semantic domain.
 *
 * Security:
 *
 *     authorization/capability semantics remain outside this parser.
 *
 * Resource system:
 *
 *     requirements are semantic metadata, not parser-level hardware limits.
 */


/* ============================================================================
 * 42. TEST CONTRACT
 * ========================================================================== */

/*
 * Positive tests MUST include at least:
 *
 *     apply H(q);
 *     apply X(q);
 *     apply CNOT(q0, q1);
 *     apply RX(theta)(q);
 *     apply U(theta, phi, lambda)(q);
 *     apply library::operation(q);
 *     apply control(X)(control, target);
 *     apply adjoint(U)(q);
 *     apply inverse(U)(q);
 *     apply control(adjoint(U))(control, target);
 *
 * Negative tests MUST include:
 *
 *     missing operation name;
 *     missing target clause;
 *     malformed argument list;
 *     malformed generic arguments;
 *     missing closing parenthesis;
 *     missing semicolon;
 *     malformed modifier nesting.
 *
 * Boundary tests MUST include:
 *
 *     zero-parameter operation;
 *     one parameter;
 *     many parameters;
 *     one target;
 *     many targets;
 *     deeply nested modifiers;
 *     long qualified names;
 *     symbolic target expressions;
 *     large operation sequences.
 *
 * Scalability tests MUST verify that no grammar-level limit exists on:
 *
 *     target count;
 *     operation count;
 *     modifier nesting;
 *     parameter count;
 *     circuit size.
 *
 * Cross-domain tests MUST include:
 *
 *     classical parameter + quantum target;
 *     symbolic parameter + quantum target;
 *     generic operation + quantum target;
 *     quantum operation inside hybrid control flow.
 */


/* ============================================================================
 * 43. ROUND-TRIP CONTRACT
 * ========================================================================== */

/*
 * Where the repository provides a canonical formatter/printer:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve the operation's intended semantics.
 *
 * In particular it must preserve:
 *
 *     operation identity;
 *     argument order;
 *     target order;
 *     modifier nesting;
 *     generic arguments.
 */


/* ============================================================================
 * 44. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is considered COMPLETE only when:
 *
 *     [ ] canonical lexer tokens are used;
 *     [ ] no duplicate lexer vocabulary exists;
 *     [ ] no hardware-specific gate catalogue exists;
 *     [ ] no fixed machine/resource limit exists;
 *     [ ] operation names remain extensible;
 *     [ ] targets remain source-level expressions;
 *     [ ] parameter expressions remain delegated to the expression grammar;
 *     [ ] generic arguments remain delegated to canonical generic grammar;
 *     [ ] measurement is not duplicated;
 *     [ ] reset is not duplicated;
 *     [ ] QEC is not duplicated;
 *     [ ] ZQN is not duplicated;
 *     [ ] routing is not duplicated;
 *     [ ] scheduling is not duplicated;
 *     [ ] optimization is not duplicated;
 *     [ ] hardware discovery is not duplicated;
 *     [ ] canonical quantum::ir remains downstream;
 *     [ ] AST preservation requirements are satisfied;
 *     [ ] semantic validation requirements are documented;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] cross-domain tests exist;
 *     [ ] round-trip tests exist where the formatter is available;
 *     [ ] Rust integration remains compatible with 1.97/1.97.1;
 *     [ ] generated/compiler code remains safe Rust;
 *     [ ] no unsafe Rust is required.
 *
 * ============================================================================
 */