/*
 * ============================================================================
 * Zamani Universal Quantum Grammar
 * ============================================================================
 *
 * File:
 *     grammar/quantum/error-correction.g4
 *
 * Purpose:
 *     Canonical reusable parser fragment for source-level quantum
 *     error-correction (QEC) intent.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar fragment
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file defines SOURCE SYNTAX for declaring and expressing QEC intent.
 *
 * It does NOT implement quantum error correction.
 *
 * The architectural direction is:
 *
 *     Zamani source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     quantum::ir          QEC semantic model
 *                                 |
 *                                 v
 *                         src/quantum/error_correction/
 *                                 |
 *              +------------------+-------------------+
 *              |                  |                   |
 *              v                  v                   v
 *          encoding           syndrome            decoding
 *          correction         extraction          correction
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - QEC declaration syntax;
 *     - QEC policy/intent syntax;
 *     - code-family references;
 *     - encoding intent;
 *     - decoding intent;
 *     - correction intent;
 *     - syndrome intent;
 *     - logical-error detection intent;
 *     - logical-observable intent;
 *     - code-distance requirements;
 *     - measurement-round requirements;
 *     - QEC strategy references;
 *     - decoder strategy references;
 *     - QEC capability requirements;
 *     - QEC constraints and preferences;
 *     - QEC verification intent;
 *     - QEC checkpoint/resume intent;
 *     - QEC semantic extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - QEC algorithms;
 *     - decoder implementations;
 *     - MWPM;
 *     - union-find;
 *     - stabilizer simulation;
 *     - surface-code construction;
 *     - code construction;
 *     - syndrome extraction implementation;
 *     - physical-qubit allocation;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - noise models;
 *     - ZQN;
 *     - hardware discovery;
 *     - resource accounting;
 *     - QecLimits;
 *     - runtime memory allocation;
 *     - runtime workers;
 *     - checkpoint serialization implementation;
 *     - canonical quantum IR;
 *     - backend selection;
 *     - QPU I/O.
 *
 * ============================================================================
 * CRITICAL BOUNDARY
 * ============================================================================
 *
 * A source-level statement such as:
 *
 *     code Surface;
 *
 * means:
 *
 *     "the program has QEC/code semantics associated with the symbolic
 *      code named Surface"
 *
 * It MUST NOT mean:
 *
 *     "use a particular physical topology"
 *
 *     "allocate exactly N physical qubits"
 *
 *     "use device X"
 *
 *     "use a particular vendor"
 *
 *     "use a particular decoder implementation"
 *
 *     "use a particular schedule"
 *
 *     "use a particular calibration"
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * There are deliberately NO grammar-level constants for:
 *
 *     - maximum logical qubits;
 *     - maximum physical qubits;
 *     - maximum code distance;
 *     - maximum syndrome rounds;
 *     - maximum stabilizers;
 *     - maximum decoder nodes;
 *     - maximum decoder edges;
 *     - maximum shots;
 *     - maximum workers;
 *     - maximum memory;
 *     - maximum QEC depth.
 *
 * Numeric values, where meaningful, are expressions.
 *
 * For example:
 *
 *     distance = d;
 *
 * is syntactically valid regardless of the eventual value of d.
 *
 * Whether that value is:
 *
 *     valid;
 *     representable;
 *     available;
 *     affordable;
 *     supported;
 *     safe;
 *
 * is determined by semantic analysis, QecLimits, resource management,
 * capability checking, compilation, scheduling, and runtime.
 *
 * ============================================================================
 * IMPORTANT EXISTING-REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The current quantum grammar already contains:
 *
 *     quantumCodeDeclaration
 *     quantumParityExpression
 *     quantumFidelityExpression
 *     quantumSurfaceExpression
 *
 * The canonical ownership of those QEC-related constructs belongs here.
 *
 * `quantum.g4` MUST delegate to this file rather than define duplicate
 * productions.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. TOP-LEVEL QEC DECLARATION
 * ========================================================================== */

/*
 * A QEC declaration introduces a reusable semantic QEC specification.
 *
 * Examples:
 *
 *     code Surface;
 *
 *     code Surface {
 *         ...
 *     }
 *
 *     code MyCode<T> {
 *         ...
 *     }
 *
 * The name is symbolic.
 *
 * It is not a hardware identifier.
 */
quantumErrorCorrectionDeclaration
    : K_CODE
      identifier
      genericParameters?
      quantumParameterList?
      quantumErrorCorrectionBody?
      SEMICOLON?
    ;


/*
 * QEC declaration body.
 *
 * The body contains declarative QEC properties and semantic extensions.
 */
quantumErrorCorrectionBody
    : blockExpression
    ;


/* ============================================================================
 * 2. CANONICAL CODE DECLARATION
 * ========================================================================== */

/*
 * This rule becomes the canonical owner of the existing quantumCodeDeclaration
 * construct currently present in quantum.g4.
 *
 * DO NOT maintain a second implementation of this rule in quantum.g4.
 */
quantumCodeDeclaration
    : quantumErrorCorrectionDeclaration
    ;


/* ============================================================================
 * 3. QEC CODE REFERENCE
 * ========================================================================== */

/*
 * A code reference is symbolic.
 *
 * Examples:
 *
 *     Surface
 *     Steane
 *     MyLogicalCode
 *     vendor.namespace.Code
 *
 * The grammar does not embed a fixed code catalogue.
 */
quantumErrorCorrectionCodeReference
    : qualifiedName
    ;


/* ============================================================================
 * 4. QEC CODE SELECTION
 * ========================================================================== */

/*
 * Selects a semantic QEC code.
 *
 * Example:
 *
 *     code Surface;
 *
 * The selected code remains an abstract semantic object until lowering.
 */
quantumErrorCorrectionCodeSelection
    : K_CODE
      quantumErrorCorrectionCodeReference
      SEMICOLON
    ;


/* ============================================================================
 * 5. QEC CODE CONFIGURATION
 * ========================================================================== */

/*
 * A QEC configuration is a set of declarative properties.
 *
 * Example:
 *
 *     code Surface {
 *         distance = d;
 *         rounds = rounds;
 *         decoder = decoder;
 *     }
 *
 * Property interpretation belongs to semantic analysis.
 */
quantumErrorCorrectionConfiguration
    : K_CODE
      quantumErrorCorrectionCodeReference
      quantumErrorCorrectionBody
    ;


/* ============================================================================
 * 6. QEC PROPERTY
 * ========================================================================== */

/*
 * Generic property form:
 *
 *     property = expression;
 *
 * This provides forward compatibility without requiring a grammar edit for
 * every new QEC research concept.
 *
 * The semantic layer MUST validate which properties are legal for a particular
 * code, execution model, dialect, or compiler phase.
 */
quantumErrorCorrectionProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * Function-like semantic property:
 *
 *     decoder(strategy);
 *
 *     verification(mode);
 *
 *     checkpoint(policy);
 *
 * The grammar records syntax only.
 */
quantumErrorCorrectionPropertyCall
    : identifier
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 7. QEC BODY ELEMENT
 * ========================================================================== */

quantumErrorCorrectionBodyElement
    : attributes*
      quantumErrorCorrectionElement
    ;


quantumErrorCorrectionElement
    : quantumErrorCorrectionProperty
    | quantumErrorCorrectionPropertyCall
    | quantumErrorCorrectionOperation
    | quantumErrorCorrectionRequirement
    | quantumErrorCorrectionCapability
    | quantumErrorCorrectionConstraint
    | quantumErrorCorrectionPreference
    | quantumErrorCorrectionHint
    | statement
    ;


/* ============================================================================
 * 8. QEC OPERATION
 * ========================================================================== */

/*
 * QEC operations are semantic requests rather than implementations.
 *
 * Example:
 *
 *     error_correct q;
 *
 * The concrete implementation may involve:
 *
 *     syndrome extraction;
 *     decoding;
 *     correction;
 *     logical-frame updates;
 *     measurement processing;
 *     another supported mechanism.
 */
quantumErrorCorrectionOperation
    : identifier
      quantumErrorCorrectionTargetClause?
      SEMICOLON
    ;


quantumErrorCorrectionTargetClause
    : K_ON
      quantumErrorCorrectionTargetList
    ;


quantumErrorCorrectionTargetList
    : quantumErrorCorrectionTarget
      (COMMA quantumErrorCorrectionTarget)*
    ;


quantumErrorCorrectionTarget
    : expression
    ;


/* ============================================================================
 * 9. ENCODING INTENT
 * ========================================================================== */

/*
 * Encoding is intentionally represented as semantic intent.
 *
 * The physical realization is downstream.
 */
quantumEncodingDeclaration
    : identifier
      quantumEncodingTargetClause
      SEMICOLON
    ;


quantumEncodingTargetClause
    : K_ON
      quantumErrorCorrectionTargetList
    ;


/* ============================================================================
 * 10. DECODING INTENT
 * ========================================================================== */

/*
 * Decoder selection is symbolic.
 *
 * The grammar MUST NOT contain a fixed list such as:
 *
 *     mwpm
 *     union_find
 *     decoder_1
 *
 * because new decoders must be addable without changing the core grammar.
 */
quantumDecodingDeclaration
    : identifier
      quantumDecodingTargetClause
      SEMICOLON
    ;


quantumDecodingTargetClause
    : K_ON
      quantumErrorCorrectionTargetList
    ;


/* ============================================================================
 * 11. CORRECTION INTENT
 * ========================================================================== */

/*
 * Correction may eventually be realized through:
 *
 *     physical correction;
 *     Pauli-frame update;
 *     logical-frame update;
 *     feed-forward;
 *     deferred correction;
 *     another backend-supported mechanism.
 *
 * The grammar intentionally does not choose among them.
 */
quantumCorrectionDeclaration
    : identifier
      quantumCorrectionTargetClause
      SEMICOLON
    ;


quantumCorrectionTargetClause
    : K_ON
      quantumErrorCorrectionTargetList
    ;


/* ============================================================================
 * 12. SYNDROME INTENT
 * ========================================================================== */

/*
 * Syndrome information is a semantic artifact.
 *
 * Its concrete representation belongs to the QEC implementation.
 */
quantumSyndromeDeclaration
    : identifier
      quantumSyndromeTargetClause?
      SEMICOLON
    ;


quantumSyndromeTargetClause
    : K_ON
      quantumErrorCorrectionTargetList
    ;


/* ============================================================================
 * 13. CODE DISTANCE
 * ========================================================================== */

/*
 * Distance is a semantic/code property.
 *
 * Examples:
 *
 *     distance = d;
 *
 *     distance = code_distance;
 *
 * The grammar imposes no maximum.
 */
quantumCodeDistanceRequirement
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 14. MEASUREMENT ROUNDS
 * ========================================================================== */

/*
 * Measurement-round counts are expressions.
 *
 * Examples:
 *
 *     rounds = r;
 *
 *     rounds = syndrome_rounds;
 *
 * No fixed upper bound exists in the grammar.
 */
quantumSyndromeRoundRequirement
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 15. LOGICAL ERROR DETECTION
 * ========================================================================== */

/*
 * Logical-error detection is an intent.
 *
 * It does not define a decoder or detector implementation.
 */
quantumLogicalErrorDetection
    : identifier
      quantumLogicalErrorTargetClause?
      SEMICOLON
    ;


quantumLogicalErrorTargetClause
    : K_ON
      quantumErrorCorrectionTargetList
    ;


/* ============================================================================
 * 16. LOGICAL OBSERVABLES
 * ========================================================================== */

/*
 * Logical observables are semantic objects.
 *
 * Their physical realization belongs to QEC lowering.
 */
quantumLogicalObservableDeclaration
    : identifier
      quantumLogicalObservableTargetClause?
      SEMICOLON
    ;


quantumLogicalObservableTargetClause
    : K_ON
      quantumErrorCorrectionTargetList
    ;


/* ============================================================================
 * 17. PARITY
 * ========================================================================== */

/*
 * This becomes the canonical owner of parity-related QEC syntax currently
 * present in quantum.g4.
 */
quantumParityExpression
    : K_PARITY
      LPAREN
      quantumErrorCorrectionTargetList
      RPAREN
    ;


/* ============================================================================
 * 18. FIDELITY
 * ========================================================================== */

/*
 * Fidelity is an expression/measurement concept.
 *
 * No threshold is embedded here.
 *
 * INVALID:
 *
 *     fidelity > 0.95
 *
 * as a grammar-level policy.
 *
 * VALID:
 *
 *     fidelity(...)
 *
 * with policy handled downstream.
 */
quantumFidelityExpression
    : K_FIDELITY
      LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 19. SURFACE / CODE FAMILY INTENT
 * ========================================================================== */

/*
 * Surface-code-like concepts may be named symbolically.
 *
 * This does NOT define physical topology.
 */
quantumSurfaceExpression
    : K_SURFACE
      LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 20. QEC REQUIREMENTS
 * ========================================================================== */

/*
 * QEC requirements express what the program needs.
 *
 * They do not grant capability.
 *
 * They do not select a physical machine.
 */
quantumErrorCorrectionRequirement
    : K_REQUIRES
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 21. QEC CAPABILITIES
 * ========================================================================== */

/*
 * Capability syntax describes required/declared semantic capability.
 *
 * Actual capability authority belongs to the capability subsystem.
 */
quantumErrorCorrectionCapability
    : K_CAPABILITY
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 22. QEC CONSTRAINTS
 * ========================================================================== */

/*
 * Constraints are distinct from requirements and preferences.
 */
quantumErrorCorrectionConstraint
    : identifier
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 23. QEC PREFERENCES
 * ========================================================================== */

/*
 * Preferences must never silently become requirements.
 */
quantumErrorCorrectionPreference
    : identifier
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 24. QEC HINTS
 * ========================================================================== */

/*
 * Hints are advisory only.
 *
 * Compiler/runtime components may ignore them when necessary.
 */
quantumErrorCorrectionHint
    : identifier
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 25. QEC VERIFICATION INTENT
 * ========================================================================== */

/*
 * Verification requests semantic verification.
 *
 * It does not implement verification.
 */
quantumErrorCorrectionVerification
    : identifier
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 26. CHECKPOINT / RESUME INTENT
 * ========================================================================== */

/*
 * QEC checkpoint syntax records a semantic request.
 *
 * It MUST NOT imply that an arbitrary unknown quantum state can always be
 * serialized.
 *
 * Valid checkpoint semantics may depend on:
 *
 *     - classical execution state;
 *     - compiled program state;
 *     - logical checkpoint state;
 *     - measurement boundaries;
 *     - QEC-supported state;
 *     - provider-supported checkpoint semantics.
 */
quantumErrorCorrectionCheckpoint
    : identifier
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 27. QEC POLICY EXPRESSION
 * ========================================================================== */

/*
 * Policy expressions remain symbolic and composable.
 */
quantumErrorCorrectionPolicyExpression
    : expression
    ;


/* ============================================================================
 * 28. QEC TARGET
 * ========================================================================== */

/*
 * QEC can target:
 *
 *     logical qubits;
 *     logical registers;
 *     encoded states;
 *     syndrome streams;
 *     observables;
 *     execution regions;
 *     another semantic QEC object.
 *
 * The target is intentionally an expression.
 */
quantumErrorCorrectionSemanticTarget
    : expression
    ;


/* ============================================================================
 * 29. QEC REGION
 * ========================================================================== */

/*
 * A QEC region groups QEC intent without fixing implementation strategy.
 *
 * Example conceptual form:
 *
 *     code Surface {
 *         ...
 *     }
 */
quantumErrorCorrectionRegion
    : quantumErrorCorrectionDeclaration
    ;


/* ============================================================================
 * 30. QEC EXTENSION POINT
 * ========================================================================== */

/*
 * Future QEC dialects may introduce additional constructs through registered
 * semantic extensions.
 *
 * The grammar does not create a vendor-specific QEC namespace.
 */
quantumErrorCorrectionExtension
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 31. QEC SOURCE ELEMENT
 * ========================================================================== */

/*
 * This is the integration point for quantum.g4.
 */
quantumErrorCorrectionElementRoot
    : quantumErrorCorrectionDeclaration
    | quantumErrorCorrectionCodeSelection
    | quantumErrorCorrectionConfiguration
    | quantumEncodingDeclaration
    | quantumDecodingDeclaration
    | quantumCorrectionDeclaration
    | quantumSyndromeDeclaration
    | quantumLogicalErrorDetection
    | quantumLogicalObservableDeclaration
    | quantumErrorCorrectionVerification
    | quantumErrorCorrectionCheckpoint
    | quantumErrorCorrectionExtension
    | quantumParityExpression
    | quantumFidelityExpression
    | quantumSurfaceExpression
    ;


/* ============================================================================
 * 32. SEMANTIC OWNERSHIP MARKERS
 * ========================================================================== */

/*
 * The following concepts intentionally remain symbolic:
 *
 *     code family
 *     decoder
 *     syndrome representation
 *     correction strategy
 *     logical observable
 *     verification strategy
 *     checkpoint strategy
 *
 * Semantic analysis resolves these against:
 *
 *     QEC subsystem
 *     capabilities
 *     resource policy
 *     target hardware
 *     ZQN
 *     scheduling
 *     routing
 *     resilience
 *
 * No implementation-specific meaning is encoded here.
 */


/* ============================================================================
 * 33. NON-OWNERSHIP GUARANTEE
 * ========================================================================== */

/*
 * This grammar MUST NOT introduce:
 *
 *     MAX_QUBITS
 *     MAX_CODE_DISTANCE
 *     MAX_ROUNDS
 *     MAX_SYNDROME_EVENTS
 *     MAX_DECODER_NODES
 *     MAX_DECODER_EDGES
 *     MAX_MEMORY
 *     MAX_PARALLELISM
 *     MAX_SHOTS
 *
 * Those are runtime/configuration policy concerns.
 *
 * The repository's canonical QecLimits remains the sole production QEC
 * resource-limit authority.
 */


/* ============================================================================
 * 34. IR LOWERING CONTRACT
 * ========================================================================== */

/*
 * Parser output:
 *
 *     parse tree
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic QEC model
 *         |
 *         v
 *     canonical quantum::ir
 *
 * This grammar MUST NOT create:
 *
 *     QecLimits
 *     Decoder
 *     Backend
 *     PhysicalQubit
 *     Schedule
 *     NoiseModel
 *     RoutingPlan
 *
 * or equivalent runtime objects.
 */


/* ============================================================================
 * 35. HARDWARE INDEPENDENCE
 * ========================================================================== */

/*
 * QEC syntax MUST NOT identify:
 *
 *     physical qubit numbers;
 *     chip coordinates;
 *     coupling maps;
 *     backend IDs;
 *     calibration IDs;
 *     pulse IDs;
 *     device addresses;
 *     topology sizes.
 *
 * Such information belongs to hardware/routing/resource lowering.
 */


/* ============================================================================
 * 36. DETERMINISM
 * ========================================================================== */

/*
 * Parsing is deterministic.
 *
 * No semantic action is permitted in this grammar.
 *
 * No:
 *
 *     Rust action;
 *     filesystem access;
 *     network access;
 *     runtime lookup;
 *     hardware discovery;
 *     random selection
 *
 * is permitted here.
 */


/* ============================================================================
 * 37. EXTENSIBILITY
 * ========================================================================== */

/*
 * New QEC algorithms should NOT require modifying this grammar merely because
 * an implementation was added.
 *
 * For example, adding:
 *
 *     a new decoder;
 *     a new code family;
 *     a new syndrome representation;
 *     a new verification backend;
 *
 * should normally require semantic registration rather than a new parser rule.
 */


/* ============================================================================
 * 38. FINAL ARCHITECTURAL CONTRACT
 * ========================================================================== */

/*
 * Zamani source describes:
 *
 *     WHAT computation requires.
 *
 * QEC determines:
 *
 *     HOW errors are detected/corrected.
 *
 * ZQN determines:
 *
 *     WHAT faults/noise exist.
 *
 * Hardware determines:
 *
 *     WHAT resources/capabilities are available.
 *
 * Routing determines:
 *
 *     WHERE computation is physically realized.
 *
 * Scheduling determines:
 *
 *     WHEN operations execute.
 *
 * Resilience determines:
 *
 *     WHETHER and HOW the computation adapts/recoveries.
 *
 * Optimization determines:
 *
 *     WHICH semantically equivalent implementation is preferable.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * Therefore:
 *
 *     grammar
 *        -> syntax
 *        -> AST
 *        -> semantic analysis
 *        -> quantum::ir
 *        -> QEC / ZQN / routing / scheduling / optimization /
 *           resilience / hardware
 *
 * and NEVER:
 *
 *     grammar
 *        -> QEC implementation
 *
 * or:
 *
 *     grammar
 *        -> hardware-specific QEC
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * and:
 *
 *     Zamani: From Atom to Everywhere.
 */