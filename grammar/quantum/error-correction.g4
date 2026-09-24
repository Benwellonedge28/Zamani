/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/error-correction.g4
 *
 * Grammar:
 *     QuantumErrorCorrection
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM ERROR-CORRECTION SYNTAX COMPONENT
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * Single syntax owner for quantum error-correction (QEC) intent.
 *
 * This grammar describes portable semantic intent. It does not implement
 * codes, decoders, syndrome extraction, correction, simulation, QEC runtime,
 * ZQN, routing, scheduling, calibration, HAL, or physical hardware mapping.
 *
 * Canonical pipeline:
 *
 *   source -> ZamaniLexer -> parser -> domain-neutral AST
 *          -> semantic analysis -> canonical quantum::ir
 *          -> QEC/ZQN/resilience/optimization/routing/scheduling
 *          -> HAL/target lowering -> runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *   - symbolic QEC/code declarations and configuration;
 *   - QEC operation/intent syntax;
 *   - encoding, syndrome, decoding and correction intent;
 *   - logical-error and logical-observable intent;
 *   - QEC requirements, capabilities, constraints, preferences and hints;
 *   - distance, rounds, accuracy and logical-error-rate property syntax;
 *   - verification/checkpoint/resume intent;
 *   - open-world QEC extension points.
 *
 * DOES NOT OWN:
 *   - lexer vocabulary;
 *   - identifiers/names/expressions/types;
 *   - generic quantum operation invocation;
 *   - measurement/reset/feed-forward;
 *   - channel/noise syntax;
 *   - QEC algorithms or decoder implementations;
 *   - resource accounting or QecLimits;
 *   - physical qubits, topology, routing, scheduling or calibration;
 *   - ZQN or canonical quantum::ir implementation.
 *
 * ============================================================================
 * LEXICAL COMPATIBILITY
 * ============================================================================
 *
 * The previous file used non-canonical K_* names. This version uses the
 * canonical ZamaniLexer vocabulary. No lexer rules or aliases are introduced.
 *
 * Canonical tokens used here:
 *   CODE, REQUIRES, CAPABILITY, WITH,
 *   ASSIGN, LPAREN, RPAREN, LBRACE, RBRACE, COMMA, SEMICOLON.
 *
 * SURFACE, PARITY and FIDELITY are intentionally retained as expression
 * entry points because they are existing Zamani lexer tokens.
 *
 * ============================================================================
 * POCO-REAF / UNBOUNDED GRAMMAR CARDINALITY
 * ============================================================================
 *
 * No language-level maximum is encoded for logical/physical qubits, code
 * distance, syndrome rounds, stabilizers, checks, decoder data, shots,
 * workers, memory, targets, parameters, nesting or QEC declarations.
 *
 * The grammar MUST NOT introduce universal limits such as:
 *
 *   MAX_QUBITS, MAX_CPUS, MAX_GPUS, MAX_FPGAS, MAX_NODES, MAX_MEMORY,
 *   MAX_THREADS, MAX_TENSOR_RANK, MAX_REGISTER_WIDTH, MAX_NETWORK_SIZE,
 *   MAX_DEVICE_COUNT, MAX_CODE_DISTANCE, MAX_SYNDROME_ROUNDS.
 *
 * Values such as distance or rounds are expressions and therefore may be
 * symbolic. Physical feasibility is decided downstream from source parsing.
 *
 * ============================================================================
 */

parser grammar QuantumErrorCorrection;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions, Types;


/*
 * ============================================================================
 * 1. CODE DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *   code Surface;
 *   code vendor::logical_code with { distance = d; rounds = r; };
 *
 * The referenced code is semantic data, not a device selection.
 */
quantumErrorCorrectionDeclaration
    : CODE quantumErrorCorrectionCodeReference
      quantumErrorCorrectionConfiguration?
      SEMICOLON?
    ;


quantumErrorCorrectionCodeReference
    : qualifiedName
      genericArgumentSuffix?
    ;


quantumErrorCorrectionConfiguration
    : WITH LBRACE
      quantumErrorCorrectionConfigurationEntry*
      RBRACE
    ;


quantumErrorCorrectionConfigurationEntry
    : quantumErrorCorrectionProperty
    | quantumErrorCorrectionPropertyCall
    ;


quantumErrorCorrectionProperty
    : identifier ASSIGN expression SEMICOLON?
    ;


quantumErrorCorrectionPropertyCall
    : identifier LPAREN argumentList? RPAREN SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. QEC INTENT
 * ============================================================================
 *
 * QEC action names remain ordinary identifiers. This avoids a closed list of
 * algorithms and allows future research and dialect extensions without
 * changing the base grammar.
 *
 * Example:
 *
 *   encode on logical_state;
 *   syndrome_extract on code;
 *   decode on syndrome;
 *   correct on logical_state;
 *
 * The target introducer is intentionally structural rather than a new QEC
 * keyword.
 */
quantumErrorCorrectionOperation
    : identifier
      quantumErrorCorrectionTargetClause?
      SEMICOLON
    ;


quantumErrorCorrectionTargetClause
    : identifier
      quantumErrorCorrectionTargetList
    ;


quantumErrorCorrectionTargetList
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * 3. SEMANTIC QEC INTENT CATEGORIES
 * ============================================================================
 *
 * These provide stable parser/AST boundaries while leaving the actual
 * operation meaning open to semantic resolution.
 */
quantumEncodingIntent
    : identifier quantumErrorCorrectionTargetClause? SEMICOLON
    ;


quantumSyndromeIntent
    : identifier quantumErrorCorrectionTargetClause? SEMICOLON
    ;


quantumDecodingIntent
    : identifier quantumErrorCorrectionTargetClause? SEMICOLON
    ;


quantumCorrectionIntent
    : identifier quantumErrorCorrectionTargetClause? SEMICOLON
    ;


quantumLogicalErrorDetectionIntent
    : identifier quantumErrorCorrectionTargetClause? SEMICOLON
    ;


quantumLogicalObservableIntent
    : identifier quantumErrorCorrectionTargetClause? SEMICOLON
    ;


/*
 * ============================================================================
 * 4. REQUIREMENTS / CAPABILITIES
 * ============================================================================
 *
 * These are source contracts. They do not perform capability discovery or
 * allocate resources.
 *
 * Examples:
 *
 *   requires qubits >= n;
 *   requires capability("quantum.error_correction");
 */
quantumErrorCorrectionRequirement
    : REQUIRES expression SEMICOLON?
    ;


quantumErrorCorrectionCapabilityRequirement
    : REQUIRES CAPABILITY LPAREN argumentList? RPAREN SEMICOLON?
    ;


quantumErrorCorrectionCapability
    : CAPABILITY LPAREN argumentList? RPAREN SEMICOLON?
    ;


/*
 * ============================================================================
 * 5. CONSTRAINTS / PREFERENCES / HINTS
 * ============================================================================
 *
 * Keep these semantically distinct:
 *
 *   requirement = mandatory program condition;
 *   constraint  = condition on a valid realization;
 *   preference  = advisory ordering/choice preference;
 *   hint        = advisory implementation information.
 *
 * None is allowed to silently become a physical device selection.
 */
quantumErrorCorrectionConstraint
    : identifier expression SEMICOLON?
    ;


quantumErrorCorrectionPreference
    : identifier expression SEMICOLON?
    ;


quantumErrorCorrectionHint
    : identifier expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. CODE DISTANCE / ROUNDS / ACCURACY
 * ============================================================================
 *
 * These are semantic property forms, not compiler limits.
 *
 * Examples:
 *
 *   distance = d;
 *   rounds = syndrome_rounds;
 *   accuracy = target_accuracy;
 *   logical_error_rate = epsilon;
 */
quantumCodeDistanceRequirement
    : identifier ASSIGN expression SEMICOLON?
    ;


quantumSyndromeRoundRequirement
    : identifier ASSIGN expression SEMICOLON?
    ;


quantumErrorCorrectionAccuracyRequirement
    : identifier ASSIGN expression SEMICOLON?
    ;


quantumLogicalErrorRateRequirement
    : identifier ASSIGN expression SEMICOLON?
    ;


quantumErrorCorrectionAssertion
    : identifier ASSIGN expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 7. STRATEGY / DECODER / CODE REFERENCES
 * ============================================================================
 *
 * No MWPM, union-find, surface code, repetition code, or vendor
 * implementation is enumerated.
 *
 * New algorithms remain ordinary semantic references.
 */
quantumErrorCorrectionStrategyReference
    : qualifiedName
      genericArgumentSuffix?
    ;


quantumErrorCorrectionDecoderReference
    : qualifiedName
      genericArgumentSuffix?
    ;


quantumErrorCorrectionCodeFamilyReference
    : qualifiedName
      genericArgumentSuffix?
    ;


quantumErrorCorrectionNoiseModelReference
    : qualifiedName
      genericArgumentSuffix?
    ;


/*
 * ============================================================================
 * 8. VERIFICATION / CHECKPOINT / RESUME
 * ============================================================================
 *
 * These describe semantic boundaries.
 *
 * They do not guarantee that an arbitrary quantum state is serializable or
 * that a target supports a particular checkpoint mechanism.
 */
quantumErrorCorrectionVerification
    : identifier LPAREN argumentList? RPAREN SEMICOLON?
    ;


quantumErrorCorrectionCheckpoint
    : identifier LPAREN argumentList? RPAREN SEMICOLON?
    ;


quantumErrorCorrectionResume
    : identifier LPAREN argumentList? RPAREN SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. POLICY / EXTENSION
 * ============================================================================
 */
quantumErrorCorrectionPolicyExpression
    : expression
    ;


quantumErrorCorrectionPolicy
    : identifier ASSIGN
      quantumErrorCorrectionPolicyExpression
      SEMICOLON?
    ;


quantumErrorCorrectionExtension
    : qualifiedName
      LPAREN argumentList? RPAREN
      quantumErrorCorrectionTargetClause?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. EXISTING QEC EXPRESSIONS
 * ============================================================================
 *
 * These preserve the existing Zamani surface while replacing obsolete K_*
 * aliases with canonical lexer tokens.
 */
quantumParityExpression
    : PARITY
      LPAREN
      quantumErrorCorrectionTargetList
      RPAREN
    ;


quantumFidelityExpression
    : FIDELITY
      LPAREN
      expression
      RPAREN
    ;


quantumSurfaceExpression
    : SURFACE
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 11. DECLARATION BODY
 * ============================================================================
 *
 * Deliberately declarative.
 *
 * Generic full-language statements are not admitted here because the
 * universal statement grammar owns general statement composition.
 */
quantumErrorCorrectionBody
    : LBRACE
      quantumErrorCorrectionBodyElement*
      RBRACE
    ;


quantumErrorCorrectionBodyElement
    : quantumErrorCorrectionProperty
    | quantumErrorCorrectionPropertyCall
    | quantumErrorCorrectionRequirement
    | quantumErrorCorrectionCapabilityRequirement
    | quantumErrorCorrectionCapability
    | quantumCodeDistanceRequirement
    | quantumSyndromeRoundRequirement
    | quantumErrorCorrectionAccuracyRequirement
    | quantumLogicalErrorRateRequirement
    | quantumErrorCorrectionAssertion
    | quantumErrorCorrectionPolicy
    | quantumErrorCorrectionVerification
    | quantumErrorCorrectionCheckpoint
    | quantumErrorCorrectionResume
    | quantumErrorCorrectionOperation
    ;


/*
 * ============================================================================
 * 12. PUBLIC STANDALONE ENTRY
 * ============================================================================
 *
 * Intended for isolated grammar/conformance tests.
 *
 * quantum.g4 should import this grammar and dispatch only the productions
 * that it owns at the composition boundary.
 */
quantumErrorCorrection
    : quantumErrorCorrectionDeclaration
    | quantumErrorCorrectionBody
    | quantumEncodingIntent
    | quantumSyndromeIntent
    | quantumDecodingIntent
    | quantumCorrectionIntent
    | quantumLogicalErrorDetectionIntent
    | quantumLogicalObservableIntent
    | quantumErrorCorrectionRequirement
    | quantumErrorCorrectionCapabilityRequirement
    | quantumErrorCorrectionVerification
    | quantumErrorCorrectionCheckpoint
    | quantumErrorCorrectionResume
    | quantumErrorCorrectionExtension
    | quantumParityExpression
    | quantumFidelityExpression
    | quantumSurfaceExpression
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser contexts MUST map into the existing domain-neutral frontend AST.
 *
 * Do not create a parser-specific QEC AST hierarchy.
 *
 * Minimum semantic payload:
 *
 *   - source span;
 *   - symbolic code/strategy/decoder references;
 *   - ordered arguments/properties;
 *   - target expressions;
 *   - intent kind;
 *   - requirement/constraint/preference/hint class.
 *
 * Semantic analysis owns:
 *
 *   - name resolution;
 *   - type/parameter checking;
 *   - code/decoder compatibility;
 *   - syndrome/correction validity;
 *   - noise/channel compatibility;
 *   - resource requirements;
 *   - capability requirements;
 *   - satisfiability;
 *   - target feasibility.
 *
 * ============================================================================
 * CANONICAL IR CONTRACT
 * ============================================================================
 *
 * QEC syntax lowers through:
 *
 *     canonical quantum::ir
 *
 * This file MUST NOT introduce:
 *
 *     QecIr
 *     QuantumErrorCorrectionIR
 *     DecoderIR
 *     SyndromeIR
 *     CodeIR
 *
 * or another competing quantum intermediate representation.
 *
 * QEC-specific semantic information may be represented by the canonical
 * semantic model and consumed by the existing QEC lowering pipeline.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * quantum.g4:
 *   Import QuantumErrorCorrection and delegate QEC productions here.
 *   Do not redefine QEC productions already owned by this file.
 *
 * operations.g4:
 *   Owns ordinary "apply operation(...)" syntax.
 *   This file does not duplicate it.
 *
 * channels.g4 / noise.g4:
 *   Own channel/noise syntax.
 *   This file only permits symbolic references to models when needed by QEC
 *   semantic analysis.
 *
 * measurement.g4 / reset.g4:
 *   Own measurement/reset syntax.
 *
 * classical-feedforward.g4:
 *   Owns classical feed-forward syntax.
 *
 * resource/capability grammars:
 *   Remain canonical for general resource/capability contracts.
 *
 * semantic QEC subsystem:
 *   Resolves codes, strategies, decoders, parameters, targets, capabilities,
 *   resources, noise compatibility and satisfiability.
 *
 * QEC implementation:
 *   Performs encoding, syndrome extraction, decoding, correction and logical
 *   state management.
 *
 * ZQN:
 *   Owns fault/noise semantics and concrete channel representations.
 *
 * routing/scheduling/HAL:
 *   Determine physical realization only after semantic lowering.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax failures are parser diagnostics with source spans.
 *
 * Semantic failures should distinguish:
 *
 *   - unresolved QEC code;
 *   - unresolved strategy/decoder;
 *   - invalid property;
 *   - invalid parameter;
 *   - invalid target;
 *   - unsatisfied capability;
 *   - unsatisfied resource requirement;
 *   - incompatible code/decoder;
 *   - incompatible code/noise model;
 *   - unsatisfiable constraint;
 *   - unsupported target realization;
 *   - unsupported dialect extension.
 *
 * Semantic failures MUST NOT be disguised as grammar failures.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS:
 *
 *   - no K_* aliases;
 *   - no fixed code-family enumeration;
 *   - no fixed decoder enumeration;
 *   - no fixed QEC algorithm enumeration;
 *   - no fixed noise/channel enumeration;
 *   - no hardware identifiers;
 *   - no physical topology;
 *   - no physical qubit mapping;
 *   - no resource maxima;
 *   - no finite target/parameter/depth limits;
 *   - no second expression language;
 *   - no second type language;
 *   - no second quantum IR;
 *   - no embedded Rust actions;
 *   - no unsafe requirement.
 *
 * A programmer may write:
 *
 *     distance = 17;
 *
 * because that is program data.
 *
 * The compiler MUST NOT interpret 17 as a universal maximum or minimum.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *   code Surface;
 *   code vendor::logical_code;
 *   code custom::code with { distance = d; rounds = r; };
 *   encode on logical_state;
 *   syndrome_extract on code;
 *   decode on syndrome;
 *   correct on logical_state;
 *   requires qubits >= n;
 *   requires capability("quantum.error_correction");
 *
 * NEGATIVE:
 *
 *   malformed qualified name;
 *   malformed configuration;
 *   missing delimiters;
 *   malformed argument list;
 *   malformed assignment;
 *   malformed target list.
 *
 * SEMANTIC-NEGATIVE:
 *
 *   unknown code;
 *   incompatible decoder;
 *   invalid distance/round value;
 *   unsupported capability;
 *   unsatisfied resource requirement;
 *   incompatible noise model.
 *
 * BOUNDARY / SCALABILITY:
 *
 *   - symbolic quantities;
 *   - very large representable quantities;
 *   - arbitrarily long property lists;
 *   - arbitrarily long target lists;
 *   - arbitrarily long argument lists;
 *   - deeply qualified names;
 *   - nested expressions;
 *   - many QEC declarations;
 *   - large mixed quantum/classical programs.
 *
 * No test establishes an artificial universal maximum.
 *
 * DETERMINISM:
 *
 *   Identical source plus identical semantic environment produces identical
 *   parse structure and diagnostics.
 *
 * COMPATIBILITY:
 *
 *   Canonical CODE/SURFACE/PARITY/FIDELITY spellings remain lexer-owned.
 *   K_* aliases are not part of this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * FILE-LOCAL:
 *
 *   [x] one canonical parser grammar declaration;
 *   [x] tokenVocab = ZamaniLexer;
 *   [x] canonical token names only;
 *   [x] open-world code/decoder/strategy references;
 *   [x] no hardware limits;
 *   [x] no QEC implementation;
 *   [x] no competing quantum IR;
 *   [x] reusable semantic boundaries;
 *   [x] cross-domain ownership documented;
 *   [x] safe-Rust integration documented.
 *
 * REPOSITORY-LEVEL:
 *
 *   [ ] quantum.g4 imports and dispatches this component without duplicate
 *       QEC productions;
 *   [ ] frontend AST mapping exists;
 *   [ ] semantic QEC validation exists;
 *   [ ] canonical quantum::ir lowering exists;
 *   [ ] QEC implementation consumes the semantic contract;
 *   [ ] positive/negative/boundary/scalability/determinism tests exist;
 *   [ ] ANTLR generation and repository-wide grammar validation pass.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */