/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum-dialects.g4
 *
 * Role:
 *     Canonical parser grammar for quantum-domain dialect declarations,
 *     references, composition, imports, extensions, feature declarations,
 *     compatibility declarations, and dialect-scoped source constructs.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * A quantum dialect is a SOURCE-LEVEL LANGUAGE EXTENSION CONTRACT.
 *
 * It describes how a quantum-domain syntax/semantic extension is identified,
 * referenced, composed, versioned, and attached to source code.
 *
 * It does NOT define:
 *
 *     - quantum hardware;
 *     - QPU models;
 *     - physical qubits;
 *     - topology;
 *     - connectivity;
 *     - calibration;
 *     - pulse schedules;
 *     - gate durations;
 *     - backend selection;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC algorithms;
 *     - ZQN noise models;
 *     - resilience decisions;
 *     - simulator implementation;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantum dialect declaration syntax;
 *     - quantum dialect identity syntax;
 *     - dialect references;
 *     - dialect inheritance/composition syntax;
 *     - dialect import/use syntax;
 *     - dialect extension declarations;
 *     - dialect feature declarations;
 *     - dialect capability references;
 *     - dialect compatibility declarations;
 *     - dialect-scoped declarations;
 *     - dialect attributes;
 *     - dialect requirements;
 *     - dialect implementation-neutral metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified-name syntax;
 *     - lexical tokens;
 *     - semantic version interpretation;
 *     - capability registry;
 *     - capability discovery;
 *     - resource allocation;
 *     - target selection;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - hardware;
 *     - runtime;
 *     - vendor implementations.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Names
 *          |
 *          v
 *     QuantumDialects
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> dialect registry
 *          +--> version compatibility
 *          +--> capability resolution
 *          +--> requirement analysis
 *          +--> type/effect analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> quantum::ir
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> resilience
 *          +--> hardware HAL
 *          |
 *          v
 *     target lowering
 *
 * This dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialects extend language semantics without binding source programs to a
 * particular machine.
 *
 * A dialect reference MUST NOT inherently identify:
 *
 *     - a device;
 *     - a QPU;
 *     - a vendor;
 *     - a physical qubit;
 *     - a topology;
 *     - a calibration;
 *     - a native gate set;
 *     - a backend;
 *     - a scheduler.
 *
 * Dialect syntax describes a language/semantic contract.
 *
 * Realization is determined later by:
 *
 *     semantic analysis
 *     capability resolution
 *     target/resource analysis
 *     compilation
 *     scheduling
 *     routing
 *     hardware/runtime layers
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level finite limits on:
 *
 *     - dialect count;
 *     - dialect inheritance depth;
 *     - dialect feature count;
 *     - dialect extension count;
 *     - capability count;
 *     - namespace depth;
 *     - declarations;
 *     - requirements;
 *     - implementations;
 *     - target architectures.
 *
 * Repetition uses:
 *
 *     *
 *     +
 *
 * rather than machine-specific constants.
 *
 * Practical limits imposed by:
 *
 *     memory
 *     source size
 *     parser implementation
 *     operating system
 *     compiler policy
 *     deployment resources
 *
 * are implementation/resource constraints and MUST NOT become language
 * grammar limits.
 *
 * ============================================================================
 * QUANTUM::IR BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT reference:
 *
 *     crate::quantum::ir
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumOperation
 *     QuantumCircuit
 *
 * It produces source syntax only.
 *
 * Semantic lowering later determines which dialect constructs contribute to
 * the canonical quantum semantic representation and, where applicable,
 * quantum::ir.
 *
 * ============================================================================
 * OPEN-WORLD DIALECT MODEL
 * ============================================================================
 *
 * Dialects are intentionally open-ended.
 *
 * The grammar MUST NOT contain a closed list such as:
 *
 *     quantumDialect
 *         : OPENQASM
 *         | IBM
 *         | QISKIT
 *         | ...
 *
 * New dialects must be representable without changing this grammar.
 *
 * Examples:
 *
 *     quantum::standard
 *     quantum::openqasm
 *     quantum::future::dynamic
 *     organization::quantum::extension
 *     vendor::domain::experimental
 *
 * Their validity and compatibility are semantic concerns.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required token additions for final integration:
 *
 *     DIALECT : 'dialect' ;
 *
 * The current lexer already owns:
 *
 *     QUANTUM
 *     IMPORT
 *     EXPORT
 *     EXTENDS
 *     IMPLEMENTS
 *     WITH
 *     REQUIRES
 *     WHERE
 *     AS
 *     AT
 *     IDENTIFIER
 *     STRING
 *     INTEGER
 *     DOUBLE_COLON
 *     ASSIGN
 *     COMMA
 *     DOT
 *     COLON
 *     SEMICOLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *
 * `DIALECT` must become a canonical reserved token rather than being
 * represented as an arbitrary identifier.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime calls;
 *     - no unsafe code.
 *
 * Generated Rust must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * ============================================================================
 */

parser grammar QuantumDialects;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/* ============================================================================
 * 1. QUANTUM DIALECT DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     quantum dialect quantum::example {
 *         ...
 *     }
 *
 * The namespace/name is semantic data, not a hardware identifier.
 */

quantumDialectDeclaration
    : QUANTUM DIALECT qualifiedName
      quantumDialectHeader?
      LBRACE
      quantumDialectMember*
      RBRACE
    ;


/* ============================================================================
 * 2. DIALECT HEADER
 * ============================================================================
 *
 * Header metadata remains implementation-neutral.
 *
 * Version information should be represented through the canonical versioning
 * integration at the parser-composition layer rather than introducing a
 * second version grammar here.
 */

quantumDialectHeader
    : quantumDialectAttribute*
    ;


/* ============================================================================
 * 3. DIALECT MEMBER
 * ========================================================================== */

quantumDialectMember
    : quantumDialectImport
    | quantumDialectUse
    | quantumDialectExtends
    | quantumDialectFeature
    | quantumDialectCapability
    | quantumDialectRequirement
    | quantumDialectExtension
    | quantumDialectCompatibility
    | quantumDialectNamespace
    | quantumDialectDeclaration
    | quantumDialectAttribute
    ;


/* ============================================================================
 * 4. DIALECT IMPORT
 * ============================================================================
 *
 * Imports another dialect contract.
 *
 * Importing a dialect does NOT import:
 *
 *     hardware
 *     runtime
 *     backend
 *     device state
 */

quantumDialectImport
    : IMPORT qualifiedName
      quantumDialectAlias?
      SEMICOLON
    ;

quantumDialectAlias
    : AS identifier
    ;


/* ============================================================================
 * 5. DIALECT USE
 * ============================================================================
 *
 * `use` activates a dialect contract in the source scope.
 *
 * Semantic analysis determines whether the referenced dialect exists and
 * whether its version/capability contract is compatible.
 */

quantumDialectUse
    : USE qualifiedName
      quantumDialectAlias?
      SEMICOLON
    ;


/* ============================================================================
 * 6. DIALECT INHERITANCE / COMPOSITION
 * ============================================================================
 *
 * A dialect may extend one or more other dialects.
 *
 * There is no finite inheritance-depth limit.
 *
 * Semantic analysis is responsible for:
 *
 *     - cycle detection;
 *     - conflict resolution;
 *     - compatibility;
 *     - feature inheritance;
 *     - capability inheritance.
 */

quantumDialectExtends
    : EXTENDS quantumDialectReferenceList
      SEMICOLON
    ;

quantumDialectReferenceList
    : quantumDialectReference
      (COMMA quantumDialectReference)*
    ;


/* ============================================================================
 * 7. DIALECT REFERENCE
 * ============================================================================
 */

quantumDialectReference
    : qualifiedName
    ;


/* ============================================================================
 * 8. DIALECT FEATURE
 * ============================================================================
 *
 * A feature is a named language/semantic facility.
 *
 * It is NOT a hardware feature.
 *
 * Examples:
 *
 *     feature dynamic_control;
 *     feature mid_circuit_measurement;
 *     feature parameterized_operations;
 */

quantumDialectFeature
    : quantumDialectFeatureKeyword
      identifier
      quantumDialectFeatureBody?
      SEMICOLON
    ;

quantumDialectFeatureKeyword
    : IDENTIFIER
    ;

quantumDialectFeatureBody
    : quantumDialectFeatureType?
      quantumDialectFeatureConstraint?
    ;

quantumDialectFeatureType
    : COLON qualifiedName
    ;

quantumDialectFeatureConstraint
    : WHERE quantumDialectPredicate
    ;


/* ============================================================================
 * 9. DIALECT CAPABILITY
 * ============================================================================
 *
 * A dialect may name capabilities that its syntax/semantics expects.
 *
 * Capability identity remains an open qualified name.
 *
 * This grammar does NOT decide whether a capability is available.
 */

quantumDialectCapability
    : quantumDialectCapabilityKeyword
      qualifiedName
      SEMICOLON
    ;

quantumDialectCapabilityKeyword
    : IDENTIFIER
    ;


/* ============================================================================
 * 10. DIALECT REQUIREMENT
 * ============================================================================
 *
 * Requirements describe semantic prerequisites.
 *
 * They do not select hardware.
 */

quantumDialectRequirement
    : REQUIRES
      quantumDialectRequirementExpression
      SEMICOLON
    ;

quantumDialectRequirementExpression
    : quantumDialectRequirementAtom
    | quantumDialectRequirementGroup
    ;

quantumDialectRequirementAtom
    : qualifiedName
    ;

quantumDialectRequirementGroup
    : LBRACE
      quantumDialectRequirementList?
      RBRACE
    ;

quantumDialectRequirementList
    : quantumDialectRequirementAtom
      (COMMA quantumDialectRequirementAtom)*
      COMMA?
    ;


/* ============================================================================
 * 11. DIALECT EXTENSION
 * ============================================================================
 *
 * Extensions allow a dialect to introduce named source-level constructs.
 *
 * The grammar does NOT prescribe what those constructs mean.
 */

quantumDialectExtension
    : quantumDialectExtensionKeyword
      qualifiedName
      quantumDialectExtensionParameters?
      quantumDialectExtensionBody?
      SEMICOLON
    ;

quantumDialectExtensionKeyword
    : IDENTIFIER
    ;

quantumDialectExtensionParameters
    : LPAREN
      quantumDialectParameterList?
      RPAREN
    ;

quantumDialectParameterList
    : quantumDialectParameter
      (COMMA quantumDialectParameter)*
    ;

quantumDialectParameter
    : identifier
      (COLON qualifiedName)?
      (ASSIGN quantumDialectValue)?
    ;

quantumDialectExtensionBody
    : LBRACE
      quantumDialectBodyItem*
      RBRACE
    ;

quantumDialectBodyItem
    : quantumDialectAttribute
    | quantumDialectFeature
    | quantumDialectCapability
    | quantumDialectRequirement
    ;


/* ============================================================================
 * 12. DIALECT COMPATIBILITY
 * ============================================================================
 *
 * Compatibility declarations describe relationships between dialects.
 *
 * They do not determine compatibility.
 *
 * Semantic analysis owns:
 *
 *     - version compatibility;
 *     - feature compatibility;
 *     - conflict detection;
 *     - migration policy.
 */

quantumDialectCompatibility
    : quantumDialectCompatibilityKeyword
      quantumDialectCompatibilityExpression
      SEMICOLON
    ;

quantumDialectCompatibilityKeyword
    : IDENTIFIER
    ;

quantumDialectCompatibilityExpression
    : quantumDialectReference
    | quantumDialectCompatibilityGroup
    ;

quantumDialectCompatibilityGroup
    : LBRACE
      quantumDialectReferenceList?
      RBRACE
    ;


/* ============================================================================
 * 13. DIALECT NAMESPACE
 * ============================================================================
 *
 * A namespace creates a source-level grouping boundary.
 *
 * It does NOT create a hardware namespace.
 */

quantumDialectNamespace
    : quantumDialectNamespaceKeyword
      qualifiedName
      LBRACE
      quantumDialectMember*
      RBRACE
    ;

quantumDialectNamespaceKeyword
    : IDENTIFIER
    ;


/* ============================================================================
 * 14. DIALECT-SCOPED DECLARATION
 * ============================================================================
 *
 * This is deliberately open.
 *
 * The canonical declaration dispatcher decides which concrete Zamani
 * declaration is legal in a dialect body.
 *
 * The rule exists as an integration boundary rather than duplicating the
 * complete language declaration grammar.
 */

quantumDialectScopedDeclaration
    : quantumDialectDeclaration
    ;


/* ============================================================================
 * 15. DIALECT ATTRIBUTE
 * ============================================================================
 *
 * Attribute ownership remains separate from dialect semantics.
 *
 * This minimal structural form permits dialect metadata without introducing
 * a second annotation language.
 *
 * The semantic attribute subsystem validates attribute names and values.
 */

quantumDialectAttribute
    : AT identifier
      quantumDialectAttributeArguments?
    ;

quantumDialectAttributeArguments
    : LPAREN
      quantumDialectValueList?
      RPAREN
    ;

quantumDialectValueList
    : quantumDialectValue
      (COMMA quantumDialectValue)*
      COMMA?
    ;


/* ============================================================================
 * 16. DIALECT VALUES
 * ============================================================================
 *
 * Values are intentionally structural.
 *
 * Semantic typing belongs to the frontend type/attribute system.
 */

quantumDialectValue
    : identifier
    | qualifiedName
    | STRING
    | INTEGER
    | quantumDialectList
    | quantumDialectMap
    ;

quantumDialectList
    : LBRACKET
      quantumDialectValueList?
      RBRACKET
    ;

quantumDialectMap
    : LBRACE
      quantumDialectMapEntry*
      RBRACE
    ;

quantumDialectMapEntry
    : quantumDialectMapKey
      COLON
      quantumDialectValue
      COMMA?
    ;

quantumDialectMapKey
    : identifier
    | STRING
    ;


/* ============================================================================
 * 17. DIALECT PREDICATE
 * ============================================================================
 *
 * This is intentionally structural.
 *
 * Capability satisfaction, feature availability and semantic compatibility
 * are NOT parser responsibilities.
 */

quantumDialectPredicate
    : quantumDialectPredicateAtom
    | quantumDialectPredicateGroup
    ;

quantumDialectPredicateAtom
    : qualifiedName
    ;

quantumDialectPredicateGroup
    : LBRACE
      quantumDialectPredicateList?
      RBRACE
    ;

quantumDialectPredicateList
    : quantumDialectPredicateAtom
      (COMMA quantumDialectPredicateAtom)*
      COMMA?
    ;


/* ============================================================================
 * 18. DIALECT REFERENCE LIST
 * ============================================================================ */

quantumDialectReferenceSet
    : LBRACE
      quantumDialectReferenceList?
      RBRACE
    ;


/* ============================================================================
 * 19. DIALECT IMPLEMENTATION-NEUTRAL DECLARATION
 * ============================================================================
 *
 * This provides a stable place for future dialect metadata without embedding
 * provider-specific syntax.
 */

quantumDialectMetadata
    : quantumDialectMetadataKeyword
      quantumDialectValue
      SEMICOLON
    ;

quantumDialectMetadataKeyword
    : IDENTIFIER
    ;


/* ============================================================================
 * 20. DIALECT CONTRACT
 * ============================================================================
 *
 * A compact reusable representation of a dialect contract.
 *
 * This is useful for parser compositions that need to accept:
 *
 *     identity
 *     inheritance
 *     features
 *     requirements
 *     capabilities
 *
 * without selecting a backend.
 */

quantumDialectContract
    : quantumDialectReference
      quantumDialectContractBody?
    ;

quantumDialectContractBody
    : LBRACE
      quantumDialectContractMember*
      RBRACE
    ;

quantumDialectContractMember
    : quantumDialectFeature
    | quantumDialectCapability
    | quantumDialectRequirement
    | quantumDialectCompatibility
    | quantumDialectAttribute
    ;


/* ============================================================================
 * 21. DIALECT ACTIVATION
 * ============================================================================
 *
 * Activation is a source-level declaration.
 *
 * It does not mean:
 *
 *     load device
 *     select backend
 *     allocate QPU
 *     allocate qubits
 *     change scheduler
 *
 * Those decisions remain downstream.
 */

quantumDialectActivation
    : USE quantumDialectReference
      SEMICOLON
    ;


/* ============================================================================
 * 22. DIALECT EXTENSION REFERENCE
 * ============================================================================
 *
 * References an extension without defining its implementation.
 */

quantumDialectExtensionReference
    : qualifiedName
    ;


/* ============================================================================
 * 23. DIALECT FEATURE REFERENCE
 * ============================================================================
 */

quantumDialectFeatureReference
    : qualifiedName
    ;


/* ============================================================================
 * 24. DIALECT CAPABILITY REFERENCE
 * ============================================================================
 *
 * Kept as a separate rule for AST/domain ownership even though its structural
 * syntax is the canonical qualified name.
 */

quantumDialectCapabilityReference
    : qualifiedName
    ;


/* ============================================================================
 * 25. DIALECT COMPATIBILITY REFERENCE
 * ============================================================================
 */

quantumDialectCompatibilityReference
    : qualifiedName
    ;


/* ============================================================================
 * 26. DIALECT IDENTITY
 * ============================================================================
 *
 * Dialect identity is a qualified semantic name.
 */

quantumDialectIdentity
    : qualifiedName
    ;


/* ============================================================================
 * 27. DIALECT IDENTITY LIST
 * ============================================================================
 */

quantumDialectIdentityList
    : quantumDialectIdentity
      (COMMA quantumDialectIdentity)*
      COMMA?
    ;


/* ============================================================================
 * 28. DIALECT FEATURE LIST
 * ============================================================================
 */

quantumDialectFeatureReferenceList
    : quantumDialectFeatureReference
      (COMMA quantumDialectFeatureReference)*
      COMMA?
    ;


/* ============================================================================
 * 29. DIALECT CAPABILITY LIST
 * ============================================================================
 */

quantumDialectCapabilityReferenceList
    : quantumDialectCapabilityReference
      (COMMA quantumDialectCapabilityReference)*
      COMMA?
    ;


/* ============================================================================
 * 30. DIALECT EXTENSION LIST
 * ============================================================================
 */

quantumDialectExtensionReferenceList
    : quantumDialectExtensionReference
      (COMMA quantumDialectExtensionReference)*
      COMMA?
    ;


/* ============================================================================
 * 31. DIALECT COMPATIBILITY LIST
 * ============================================================================
 */

quantumDialectCompatibilityReferenceList
    : quantumDialectCompatibilityReference
      (COMMA quantumDialectCompatibilityReference)*
      COMMA?
    ;