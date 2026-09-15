/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/experimental.g4
 *
 * Role:
 *     Canonical parser grammar for explicitly opt-in experimental dialects.
 *
 * Baseline:
 *     ANTLR4 parser grammar
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only; no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
 *       |
 *       +--> core::Names
 *       +--> core::Versioning
 *       |
 *       v
 *     ExperimentalDialects
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic validation
 *       |
 *       +--> dialect registry
 *       +--> compatibility
 *       +--> capability resolution
 *       +--> feature/requirement checking
 *       +--> lowering validation
 *       |
 *       v
 *     canonical semantic representations
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representations
 *       +--> other domain IRs
 *       |
 *       v
 *     optimization / routing / scheduling / QEC / ZQN /
 *     resilience / hardware HAL / runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - explicit experimental dialect declaration syntax;
 *   - experimental lifecycle metadata;
 *   - experimental stability/status metadata;
 *   - explicit opt-in metadata;
 *   - experimental feature declarations;
 *   - experimental capability declarations;
 *   - experimental semantic requirements;
 *   - experimental compatibility declarations;
 *   - experimental deprecation/sunset metadata;
 *   - experimental extension descriptors;
 *   - experimental namespace structure;
 *   - experimental dialect metadata and attributes.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - qualified names;
 *   - lexical keywords;
 *   - semantic version comparison;
 *   - package/module resolution;
 *   - plugin loading;
 *   - capability discovery;
 *   - hardware discovery;
 *   - resource allocation;
 *   - target selection;
 *   - topology;
 *   - physical qubit identifiers;
 *   - QubitId;
 *   - PhysicalQubitId;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL/hardware IR;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - simulation;
 *   - runtime execution;
 *   - vendor implementation.
 *
 * ============================================================================
 * FUNDAMENTAL PRINCIPLE
 * ============================================================================
 *
 * Experimental status describes the maturity of a SOURCE-LEVEL DIALECT
 * CONTRACT.
 *
 * It does not describe:
 *
 *   - experimental hardware;
 *   - experimental QPUs;
 *   - experimental GPUs;
 *   - experimental CPUs;
 *   - experimental FPGAs;
 *   - experimental deployment environments.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Dialect identities are arbitrary qualified names.
 *
 * Examples:
 *
 *     quantum::future
 *     quantum::dynamic::preview
 *     hdl::future::synth
 *     hardware::reconfigurable::preview
 *     ai::future::tensor
 *     organization::research::experimental
 *     vendor::domain::preview
 *
 * This grammar deliberately contains no closed enumeration of dialects.
 *
 * Adding a new dialect MUST NOT require changing this grammar.
 *
 * ============================================================================
 * EXPERIMENTAL OPT-IN
 * ============================================================================
 *
 * Canonical source form:
 *
 *     @experimental dialect quantum::future {
 *         ...
 *     }
 *
 * The existing lexer does not define an EXPERIMENTAL token.
 *
 * Therefore the marker is represented structurally as:
 *
 *     AT identifier
 *
 * Semantic analysis MUST validate that the identifier is exactly:
 *
 *     experimental
 *
 * This avoids coupling the experimental grammar to an unnecessary lexer
 * modification while retaining an explicit semantic opt-in boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar contains no finite machine/resource limits.
 *
 * It does not encode:
 *
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_FEATURES
 *     MAX_CAPABILITIES
 *     MAX_EXTENSIONS
 *     MAX_NAMESPACE_DEPTH
 *
 * Practical limits belong to parser/compiler/runtime/resource policy.
 *
 * Therefore:
 *
 *     Program Once
 *       ->
 *     Compile Once
 *       ->
 *     Run Everywhere
 *       ->
 *     Run Anywhere
 *       ->
 *     Run Forever
 *
 * remains the architectural goal.
 *
 * ============================================================================
 * VERSION BOUNDARY
 * ============================================================================
 *
 * Version syntax is consumed from core/versioning.g4.
 *
 * This file MUST NOT redefine:
 *
 *     exactVersion
 *     versionRange
 *     versionConstraint
 *     version comparison
 *     semantic-version ordering
 *     migration semantics
 *
 * Semantic compatibility belongs downstream.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * An experimental quantum dialect may declare semantic capabilities such as:
 *
 *     capability quantum::dynamic_control;
 *
 * but this grammar does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumOperation
 *     QuantumCircuit
 *     topology
 *     calibration
 *     pulse scheduling
 *
 * Quantum constructs eventually lower through semantic analysis into the
 * canonical quantum semantic representation and, where applicable,
 * quantum::ir.
 *
 * ============================================================================
 * QEC / ZQN BOUNDARY
 * ============================================================================
 *
 * Experimental syntax may declare requirements related to QEC or fault-aware
 * execution, but this grammar does not implement QEC or ZQN.
 *
 * QEC remains the owner of error-correction algorithms.
 *
 * ZQN remains the owner of fault/noise semantics.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Experimental dialects MUST NOT turn target-specific facts into permanent
 * source semantics.
 *
 * Forbidden concepts at this layer include:
 *
 *     device = "specific-device"
 *     qpu = 3
 *     qubits = 32
 *     gpu_count = 8
 *     topology = "fixed-grid"
 *
 * If a program requires a capability, it should express a semantic capability
 * or requirement. Hardware realization belongs to target/resource/hardware
 * layers.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This is a pure ANTLR parser grammar:
 *
 *   - no embedded Rust;
 *   - no actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware access;
 *   - no runtime calls;
 *   - no randomness.
 *
 * Identical token streams receive identical syntactic treatment.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *   - source spans;
 *   - experimental marker;
 *   - dialect identity;
 *   - declaration order;
 *   - version-expression structure;
 *   - feature structure;
 *   - capability structure;
 *   - requirement structure;
 *   - lifecycle metadata;
 *   - extension structure;
 *   - original spelling where required for diagnostics/round-tripping.
 *
 * Parsing MUST NOT:
 *
 *   - resolve dialects;
 *   - resolve versions;
 *   - discover capabilities;
 *   - discover hardware;
 *   - allocate resources;
 *   - create IR.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Experimental status may later become semantic metadata attached to a
 * dialect contract or compilation unit.
 *
 * It must never create a second:
 *
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR;
 *   - hardware IR;
 *   - scheduling representation;
 *   - QEC representation;
 *   - ZQN representation;
 *   - runtime representation.
 *
 * ============================================================================
 */

parser grammar ExperimentalDialects;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Versioning;


/* ============================================================================
 * 1. ROOT DECLARATION
 * ========================================================================== */

/*
 * Canonical form:
 *
 *     @experimental dialect quantum::future {
 *         ...
 *     }
 *
 * The marker is intentionally part of this grammar rather than a lexer
 * keyword so that introducing an experimental dialect facility does not
 * require changing the universal lexical vocabulary.
 */
experimentalDialectDeclaration
    : experimentalDialectMarker
      DIALECT
      qualifiedName
      experimentalDialectHeader?
      LBRACE
      experimentalDialectMember*
      RBRACE
    ;


/* ============================================================================
 * 2. EXPERIMENTAL MARKER
 * ========================================================================== */

experimentalDialectMarker
    : AT identifier
    ;


/* ============================================================================
 * 3. HEADER
 * ========================================================================== */

experimentalDialectHeader
    : experimentalDialectHeaderItem*
    ;

experimentalDialectHeaderItem
    : experimentalDialectVersion
    | experimentalDialectStatus
    | experimentalDialectStability
    | experimentalDialectOptIn
    | experimentalDialectMetadata
    | experimentalDialectAttribute
    ;


/* ============================================================================
 * 4. MEMBER DISPATCH
 * ========================================================================== */

experimentalDialectMember
    : experimentalDialectImport
    | experimentalDialectUse
    | experimentalDialectExtends
    | experimentalDialectVersion
    | experimentalDialectStatus
    | experimentalDialectStability
    | experimentalDialectOptIn
    | experimentalDialectFeature
    | experimentalDialectCapability
    | experimentalDialectRequirement
    | experimentalDialectCompatibility
    | experimentalDialectDeprecation
    | experimentalDialectExtension
    | experimentalDialectNamespace
    | experimentalDialectMetadata
    | experimentalDialectAttribute
    ;


/* ============================================================================
 * 5. IMPORT / USE
 * ========================================================================== */

experimentalDialectImport
    : IMPORT qualifiedName experimentalDialectAlias? SEMICOLON
    ;

experimentalDialectUse
    : USE
      qualifiedName
      experimentalDialectAlias?
      dialectVersionConstraint?
      SEMICOLON
    ;

experimentalDialectAlias
    : AS identifier
    ;


/* ============================================================================
 * 6. COMPOSITION
 * ========================================================================== */

experimentalDialectExtends
    : EXTENDS experimentalDialectReferenceList SEMICOLON
    ;

experimentalDialectReferenceList
    : experimentalDialectReference
      (COMMA experimentalDialectReference)*
    ;

experimentalDialectReference
    : qualifiedName dialectVersionConstraint?
    ;


/* ============================================================================
 * 7. VERSION
 * ========================================================================== */

/*
 * Version structure is delegated to core/versioning.g4.
 */
experimentalDialectVersion
    : VERSION versionExpression SEMICOLON
    ;


/* ============================================================================
 * 8. STATUS
 * ========================================================================== */

/*
 * Status remains symbolic and open-world.
 *
 * Semantic analysis determines which lifecycle/status values are recognized.
 */
experimentalDialectStatus
    : STATUS identifier SEMICOLON
    ;


/* ============================================================================
 * 9. STABILITY
 * ========================================================================== */

experimentalDialectStability
    : STABILITY identifier SEMICOLON
    ;


/* ============================================================================
 * 10. EXPLICIT OPT-IN
 * ========================================================================== */

/*
 * Opt-in is deliberately represented as policy metadata.
 *
 * Examples:
 *
 *     opt_in true;
 *     opt_in preview;
 *     opt_in explicit;
 *
 * Semantic analysis decides which policy values are legal.
 */
experimentalDialectOptIn
    : OPT_IN experimentalDialectOptInValue SEMICOLON
    ;

experimentalDialectOptInValue
    : identifier
    | TRUE
    | FALSE
    ;


/* ============================================================================
 * 11. FEATURES
 * ========================================================================== */

experimentalDialectFeature
    : FEATURE
      identifier
      experimentalDialectFeatureType?
      experimentalDialectFeatureConstraint*
      SEMICOLON
    ;

experimentalDialectFeatureType
    : COLON qualifiedName
    ;

experimentalDialectFeatureConstraint
    : WHERE experimentalDialectPredicate
    ;


/* ============================================================================
 * 12. CAPABILITIES
 * ========================================================================== */

experimentalDialectCapability
    : CAPABILITY
      qualifiedName
      experimentalDialectCapabilityBody?
      SEMICOLON
    ;

experimentalDialectCapabilityBody
    : LBRACE
      experimentalDialectCapabilityMember*
      RBRACE
    ;

experimentalDialectCapabilityMember
    : experimentalDialectRequirement
    | experimentalDialectMetadata
    | experimentalDialectAttribute
    ;


/* ============================================================================
 * 13. REQUIREMENTS
 * ========================================================================== */

experimentalDialectRequirement
    : REQUIRES experimentalDialectRequirementExpression SEMICOLON
    ;

experimentalDialectRequirementExpression
    : experimentalDialectRequirementTerm
      (
          experimentalDialectLogicalOperator
          experimentalDialectRequirementTerm
      )*
    ;

experimentalDialectRequirementTerm
    : experimentalDialectRequirementAtom
    | LPAREN experimentalDialectRequirementExpression RPAREN
    ;

experimentalDialectRequirementAtom
    : qualifiedName dialectVersionConstraint?
    | experimentalDialectCapabilityReference
    ;

experimentalDialectCapabilityReference
    : CAPABILITY qualifiedName
    ;

experimentalDialectLogicalOperator
    : AND
    | OR
    ;


/* ============================================================================
 * 14. COMPATIBILITY
 * ========================================================================== */

experimentalDialectCompatibility
    : COMPATIBLE
      WITH
      experimentalDialectReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 15. DEPRECATION / SUNSET
 * ========================================================================== */

/*
 * Lifecycle metadata is descriptive.
 *
 * It does not itself remove functionality, reject execution, or change
 * runtime behavior.
 */
experimentalDialectDeprecation
    : DEPRECATED
      experimentalDialectDeprecationBody?
      SEMICOLON
    ;

experimentalDialectDeprecationBody
    : LBRACE
      experimentalDialectLifecycleItem*
      RBRACE
    ;

experimentalDialectLifecycleItem
    : experimentalDialectLifecycleKey
      ASSIGN
      experimentalDialectValue
      SEMICOLON
    ;

experimentalDialectLifecycleKey
    : identifier
    ;


/* ============================================================================
 * 16. EXTENSIONS
 * ========================================================================== */

/*
 * An extension is a source-level semantic facility.
 *
 * It is not a runtime callback, plugin invocation, hardware instruction, or
 * backend selection.
 */
experimentalDialectExtension
    : EXTENSION
      qualifiedName
      experimentalDialectExtensionSignature?
      experimentalDialectExtensionConstraint*
      experimentalDialectExtensionBody?
      SEMICOLON?
    ;

experimentalDialectExtensionSignature
    : LPAREN
      experimentalDialectParameterList?
      RPAREN
    ;

experimentalDialectParameterList
    : experimentalDialectParameter
      (COMMA experimentalDialectParameter)*
    ;

experimentalDialectParameter
    : identifier
      experimentalDialectParameterType?
      experimentalDialectDefaultValue?
    ;

experimentalDialectParameterType
    : COLON qualifiedName
    ;

experimentalDialectDefaultValue
    : ASSIGN experimentalDialectValue
    ;

experimentalDialectExtensionConstraint
    : WHERE experimentalDialectPredicate
    ;

experimentalDialectExtensionBody
    : LBRACE
      experimentalDialectExtensionMember*
      RBRACE
    ;

experimentalDialectExtensionMember
    : experimentalDialectFeature
    | experimentalDialectCapability
    | experimentalDialectRequirement
    | experimentalDialectCompatibility
    | experimentalDialectMetadata
    | experimentalDialectAttribute
    ;


/* ============================================================================
 * 17. NAMESPACES
 * ========================================================================== */

/*
 * Namespace depth is intentionally unbounded by grammar design.
 */
experimentalDialectNamespace
    : NAMESPACE
      qualifiedName
      LBRACE
      experimentalDialectMember*
      RBRACE
    ;


/* ============================================================================
 * 18. METADATA
 * ========================================================================== */

experimentalDialectMetadata
    : METADATA
      identifier
      (ASSIGN experimentalDialectValue)?
      SEMICOLON
    ;


/* ============================================================================
 * 19. ATTRIBUTES
 * ========================================================================== */

experimentalDialectAttribute
    : AT
      identifier
      experimentalDialectAttributeArguments?
    ;

experimentalDialectAttributeArguments
    : LPAREN
      experimentalDialectValueList?
      RPAREN
    ;


/* ============================================================================
 * 20. PREDICATES
 * ========================================================================== */

/*
 * Predicate syntax is structural only.
 *
 * It cannot:
 *
 *   - execute code;
 *   - inspect hardware;
 *   - access files;
 *   - access networks;
 *   - invoke runtime services.
 *
 * Operator precedence:
 *
 *     NOT
 *       >
 *     AND
 *       >
 *     OR
 */
experimentalDialectPredicate
    : experimentalDialectPredicateOr
    ;

experimentalDialectPredicateOr
    : experimentalDialectPredicateAnd
      (OR experimentalDialectPredicateAnd)*
    ;

experimentalDialectPredicateAnd
    : experimentalDialectPredicateUnary
      (AND experimentalDialectPredicateUnary)*
    ;

experimentalDialectPredicateUnary
    : NOT experimentalDialectPredicateUnary
    | experimentalDialectPredicatePrimary
    ;

experimentalDialectPredicatePrimary
    : LPAREN experimentalDialectPredicateOr RPAREN
    | qualifiedName
    | qualifiedName
      experimentalDialectComparisonOperator
      experimentalDialectValue
    ;

experimentalDialectComparisonOperator
    : EQ
    | NEQ
    | LT
    | LTE
    | GT
    | GTE
    ;


/* ============================================================================
 * 21. STRUCTURAL VALUES
 * ========================================================================== */

experimentalDialectValue
    : identifier
    | qualifiedName
    | STRING
    | INTEGER
    | FLOAT
    | TRUE
    | FALSE
    | NIL
    | NULL
    | experimentalDialectListValue
    | experimentalDialectMapValue
    ;

experimentalDialectListValue
    : LBRACKET
      experimentalDialectValueList?
      RBRACKET
    ;

experimentalDialectValueList
    : experimentalDialectValue
      (COMMA experimentalDialectValue)*
      COMMA?
    ;

experimentalDialectMapValue
    : LBRACE
      experimentalDialectMapEntry*
      RBRACE
    ;

experimentalDialectMapEntry
    : identifier
      COLON
      experimentalDialectValue
      COMMA?
    ;


/* ============================================================================
 * 22. INTEGRATION WRAPPERS
 * ========================================================================== */

/*
 * These wrappers provide stable names for AST/semantic integration without
 * redefining canonical core concepts.
 */

experimentalDialectIdentity
    : qualifiedName
    ;

experimentalDialectVersionConstraint
    : dialectVersionConstraint
    ;

experimentalDialectFeatureName
    : identifier
    ;

experimentalDialectCapabilityName
    : qualifiedName
    ;

experimentalDialectRequirementName
    : qualifiedName
    ;

experimentalDialectExtensionName
    : qualifiedName
    ;


/* ============================================================================
 * 23. INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * Aggregate parser integration:
 *
 *     1. Import ExperimentalDialects from the dialect grammar composition
 *        layer.
 *
 *     2. Add:
 *
 *            experimentalDialectDeclaration
 *
 *        to the dialect-declaration dispatcher.
 *
 *     3. Keep ordinary dialect declarations owned by Dialects/Registration.
 *
 *     4. Do not add a new lexer keyword merely for this grammar.
 *
 *     5. Map the parse tree to an AST equivalent to:
 *
 *            ExperimentalDialectDeclaration {
 *                marker,
 *                identity,
 *                header,
 *                members,
 *                span
 *            }
 *
 *     6. Semantic analysis MUST validate:
 *
 *            marker == "experimental"
 *
 *        because the lexical representation intentionally uses:
 *
 *            AT identifier
 *
 *     7. Semantic analysis MUST enforce explicit opt-in policy.
 *
 *     8. Version compatibility is resolved through the canonical versioning
 *        subsystem.
 *
 *     9. Dialect identity and dependencies are resolved through the dialect
 *        registry.
 *
 *    10. Capabilities are resolved by capability analysis.
 *
 *    11. Resource and hardware suitability are resolved downstream.
 *
 *    12. Quantum constructs introduced by an experimental dialect must
 *        eventually cross the canonical semantic boundary and, where
 *        applicable, quantum::ir.
 *
 *    13. QEC, ZQN, routing, scheduling, optimization, resilience, hardware
 *        HAL, and runtime remain downstream consumers.
 *
 *    14. Generated Rust must remain safe Rust and compile with Rust 1.97 /
 *        Rust 1.97.1.
 *
 * ============================================================================
 * NON-DEPENDENCIES
 * ============================================================================
 *
 * This grammar MUST NOT import:
 *
 *     quantum
 *     hardware
 *     scheduling
 *     routing
 *     optimization
 *     QEC
 *     ZQN
 *     resilience
 *     runtime
 *     classical IR
 *     quantum::ir
 *
 * Those systems consume semantic output; they are not parser dependencies.
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] ANTLR generation succeeds.
 *
 *   [ ] ZamaniLexer provides every referenced token.
 *
 *   [ ] Names resolves identifier/qualifiedName.
 *
 *   [ ] Versioning resolves versionExpression and dialectVersionConstraint.
 *
 *   [ ] No canonical name rule is duplicated.
 *
 *   [ ] No canonical version grammar is duplicated.
 *
 *   [ ] The aggregate parser can import this grammar.
 *
 *   [ ] The aggregate parser dispatches experimentalDialectDeclaration.
 *
 *   [ ] AST mapping preserves source spans and declaration structure.
 *
 *   [ ] Semantic analysis validates the exact experimental marker.
 *
 *   [ ] Explicit opt-in policy is enforced semantically.
 *
 *   [ ] Experimental status cannot silently affect stable dialect semantics.
 *
 *   [ ] Hardware/resource facts are not encoded as portable dialect identity.
 *
 *   [ ] No finite scalability constants exist in this grammar.
 *
 *   [ ] No quantum IR is defined here.
 *
 *   [ ] No runtime/hardware access exists here.
 *
 *   [ ] Positive tests pass.
 *
 *   [ ] Negative tests pass.
 *
 *   [ ] Boundary tests pass.
 *
 *   [ ] Cross-domain tests pass.
 *
 *   [ ] Scalability tests pass subject only to available resources.
 *
 *   [ ] Determinism tests pass.
 *
 *   [ ] Round-trip tests preserve semantic structure.
 *
 *   [ ] Rust 1.97 / Rust 1.97.1 integration builds with safe Rust only.
 *
 * ============================================================================
 * TEST VECTORS
 * ============================================================================
 *
 * VALID:
 *
 *     @experimental dialect quantum::future {
 *     }
 *
 *     @experimental dialect quantum::future {
 *         version 1.0.0;
 *         status preview;
 *         stability experimental;
 *         opt_in true;
 *         feature dynamic_control;
 *         capability quantum::dynamic_control;
 *         requires quantum::dynamic_control;
 *     }
 *
 *     @experimental dialect hdl::future::synth {
 *         feature parameterized_pipeline;
 *         capability hardware::reconfigurable_logic;
 *     }
 *
 *     @experimental dialect organization::research::future {
 *         extends organization::base;
 *         import organization::shared;
 *
 *         namespace future {
 *             feature new_semantics;
 *         }
 *     }
 *
 *     @experimental dialect quantum::future {
 *         requires quantum::dynamic_control
 *             and quantum::mid_circuit_control;
 *     }
 *
 *     @experimental dialect future::computing {
 *         feature adaptive;
 *         feature distributed;
 *         feature heterogeneous;
 *
 *         capability quantum::dynamic_control;
 *         capability hardware::reconfigurable_logic;
 *         capability distributed::messaging;
 *     }
 *
 * INVALID STRUCTURAL FORMS:
 *
 *     dialect quantum::future {
 *     }
 *
 *     @experimental {
 *     }
 *
 *     @experimental dialect {
 *     }
 *
 *     @experimental dialect quantum::future {
 *         version;
 *     }
 *
 *     @experimental dialect quantum::future {
 *         feature;
 *     }
 *
 *     @experimental dialect quantum::future {
 *         capability;
 *     }
 *
 *     @experimental dialect quantum::future {
 *         requires;
 *     }
 *
 * SEMANTICALLY INVALID:
 *
 *     @preview dialect quantum::future { }
 *
 *     // marker is not `experimental`
 *
 *     @experimental dialect quantum::future {
 *         ...
 *     }
 *
 *     // rejected if explicit opt-in policy is not satisfied
 *
 *     @experimental dialect quantum::future {
 *         ...
 *     }
 *
 *     // rejected if a dependency cycle exists
 *
 *     @experimental dialect quantum::future {
 *         ...
 *     }
 *
 *     // rejected if declared compatibility is unsatisfiable
 *
 * HARDWARE-INDEPENDENCE TEST:
 *
 *     @experimental dialect quantum::future {
 *         requires quantum::dynamic_control;
 *         capability quantum::dynamic_control;
 *     }
 *
 * This must NOT imply:
 *
 *     - a particular QPU;
 *     - a particular number of qubits;
 *     - a topology;
 *     - a calibration;
 *     - a device identifier.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_EXPERIMENTAL_DIALECTS
 *     MAX_FEATURES
 *     MAX_CAPABILITIES
 *     MAX_REQUIREMENTS
 *     MAX_EXTENSIONS
 *     MAX_NAMESPACE_DEPTH
 *     MAX_VERSION_COMPONENT
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *
 * Forbidden target-specific source semantics:
 *
 *     device = "..."
 *     qpu = ...
 *     gpu = ...
 *     cpu = ...
 *     qubits = ...
 *     topology = ...
 *     physical_qubit = ...
 *
 * Practical limits remain compiler/resource policy rather than grammar
 * semantics.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Its generated Rust integration MUST:
 *
 *     - use Rust 1.97 or Rust 1.97.1;
 *     - use Rust 2021;
 *     - use safe Rust only;
 *     - contain no unsafe blocks;
 *     - contain no unsafe functions;
 *     - contain no unsafe traits;
 *     - contain no unsafe implementations.
 *
 * ============================================================================
 */