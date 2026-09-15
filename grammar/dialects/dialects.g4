/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/dialects.g4
 *
 * Role:
 *     Canonical parser grammar for Zamani's universal source-level dialect
 *     system.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
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
 *       |
 *       +--> Dialects                    <-- THIS FILE
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> dialect registry
 *       +--> version compatibility
 *       +--> capability resolution
 *       +--> effect/type analysis
 *       +--> resource/requirement analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware semantic model
 *       +--> control/data/temporal IR
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
 *   - universal dialect declaration syntax;
 *   - dialect identity syntax;
 *   - dialect imports;
 *   - dialect aliases;
 *   - dialect composition;
 *   - dialect extension declarations;
 *   - dialect feature declarations;
 *   - capability declarations/references;
 *   - semantic requirement declarations;
 *   - compatibility declarations;
 *   - version constraints as structural syntax;
 *   - dialect namespaces;
 *   - dialect-scoped attributes;
 *   - implementation-neutral dialect metadata;
 *   - source-level dialect activation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical identifiers;
 *   - lexical keywords;
 *   - semantic version comparison;
 *   - package resolution;
 *   - filesystem resolution;
 *   - plugin loading;
 *   - capability discovery;
 *   - hardware discovery;
 *   - hardware topology;
 *   - physical qubit identifiers;
 *   - resource allocation;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - simulation;
 *   - runtime execution;
 *   - canonical IR;
 *   - quantum::ir;
 *   - vendor implementation code.
 *
 * ============================================================================
 * FUNDAMENTAL RULE
 * ============================================================================
 *
 * A dialect is a SOURCE-LEVEL LANGUAGE/SEMANTIC EXTENSION CONTRACT.
 *
 * A dialect MUST NOT inherently select:
 *
 *   - a machine;
 *   - a processor;
 *   - a QPU;
 *   - a GPU;
 *   - an FPGA;
 *   - an ASIC;
 *   - a device;
 *   - a physical qubit;
 *   - a topology;
 *   - a backend;
 *   - a calibration;
 *   - a scheduler;
 *   - a runtime.
 *
 * A dialect can state semantic requirements and capabilities.
 *
 * The actual realization is determined later by:
 *
 *   semantic analysis
 *   capability resolution
 *   resource analysis
 *   compilation
 *   optimization
 *   routing
 *   scheduling
 *   hardware abstraction
 *   runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialects must preserve:
 *
 *   Program Once
 *   Compile Once
 *   Run Everywhere
 *   Run Anywhere
 *   Run Forever
 *
 * Therefore the grammar contains NO finite machine/resource limits.
 *
 * There is no:
 *
 *   MAX_DIALECTS
 *   MAX_FEATURES
 *   MAX_CAPABILITIES
 *   MAX_EXTENSIONS
 *   MAX_TARGETS
 *   MAX_NODES
 *   MAX_QUBITS
 *   MAX_DEVICES
 *
 * Practical limits are imposed only by available resources and implementation
 * policy, never by this grammar.
 *
 * ============================================================================
 * OPEN-WORLD MODEL
 * ============================================================================
 *
 * Dialect names are arbitrary qualified names.
 *
 * Examples:
 *
 *   quantum::standard
 *   quantum::openqasm
 *   quantum::future::dynamic
 *   hardware::fpga
 *   hardware::future::reconfigurable
 *   ai::tensor
 *   distributed::consensus
 *   vendor::example::quantum
 *   organization::domain::extension
 *
 * No closed enumeration of dialects is permitted here.
 *
 * Adding a new dialect MUST NOT require modifying this grammar.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *   QubitId
 *   PhysicalQubitId
 *   GateKind
 *   QuantumOperation
 *   QuantumCircuit
 *   topology
 *   calibration
 *   pulse schedule
 *
 * It only describes source syntax.
 *
 * Quantum dialect constructs that represent actual computation are lowered
 * later into the canonical quantum semantic representation and, where
 * appropriate, quantum::ir.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *   - no embedded Rust;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware access;
 *   - no runtime calls;
 *   - no unsafe code.
 *
 * ============================================================================
 */

parser grammar Dialects;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/* ============================================================================
 * 1. TOP-LEVEL DIALECT DECLARATION
 * ========================================================================== */

/*
 * Canonical form:
 *
 *     dialect quantum::standard {
 *         ...
 *     }
 *
 * Version and metadata are optional and remain implementation-neutral.
 */
dialectDeclaration
    : DIALECT qualifiedName dialectHeader? LBRACE dialectMember* RBRACE
    ;


/* ============================================================================
 * 2. DIALECT HEADER
 * ========================================================================== */

dialectHeader
    : dialectHeaderItem*
    ;

dialectHeaderItem
    : dialectVersionDeclaration
    | dialectAttribute
    | dialectStatusDeclaration
    | dialectMetadataDeclaration
    ;


/* ============================================================================
 * 3. DIALECT MEMBERS
 * ========================================================================== */

dialectMember
    : dialectImportDeclaration
    | dialectUseDeclaration
    | dialectExtendsDeclaration
    | dialectFeatureDeclaration
    | dialectCapabilityDeclaration
    | dialectRequirementDeclaration
    | dialectExtensionDeclaration
    | dialectCompatibilityDeclaration
    | dialectConflictDeclaration
    | dialectNamespaceDeclaration
    | dialectMetadataDeclaration
    | dialectAttribute
    ;


/* ============================================================================
 * 4. IMPORT
 * ========================================================================== */

/*
 * Imports another dialect contract.
 *
 * This does not load:
 *
 *   hardware
 *   runtime
 *   plugins
 *   devices
 *
 * Resolution is semantic/toolchain responsibility.
 */
dialectImportDeclaration
    : IMPORT qualifiedName dialectAlias? SEMICOLON
    ;

dialectAlias
    : AS identifier
    ;


/* ============================================================================
 * 5. USE / ACTIVATION
 * ========================================================================== */

/*
 * Activates a dialect for the enclosing source context.
 *
 * The grammar does not decide whether the dialect exists.
 */
dialectUseDeclaration
    : USE qualifiedName dialectAlias? dialectVersionConstraint? SEMICOLON
    ;


/* ============================================================================
 * 6. COMPOSITION / INHERITANCE
 * ========================================================================== */

/*
 * Dialects may compose other dialect contracts.
 *
 * There is no finite depth limit.
 *
 * Semantic analysis must detect:
 *
 *   - cycles;
 *   - incompatible inherited features;
 *   - conflicting definitions;
 *   - capability conflicts.
 */
dialectExtendsDeclaration
    : EXTENDS dialectReferenceList SEMICOLON
    ;

dialectReferenceList
    : dialectReference (COMMA dialectReference)*
    ;

dialectReference
    : qualifiedName dialectVersionConstraint?
    ;


/* ============================================================================
 * 7. VERSION DECLARATION
 * ========================================================================== */

/*
 * Version syntax is structural.
 *
 * Semantic version interpretation belongs to the compatibility subsystem.
 *
 * Examples:
 *
 *     version "1.0.0";
 *     version "2026.1";
 *
 * A dialect version is metadata, not a hardware version.
 */
dialectVersionDeclaration
    : VERSION STRING SEMICOLON
    ;


/* ============================================================================
 * 8. VERSION CONSTRAINT
 * ========================================================================== */

/*
 * The grammar accepts an open structural constraint expression.
 *
 * Examples:
 *
 *     version >= "1.0.0"
 *     version ^ "2.0.0"
 *     version ~ "3.1"
 *     version "1.2.3"
 *
 * Exact comparison semantics are owned by semantic compatibility analysis.
 */
dialectVersionConstraint
    : VERSION dialectVersionOperator STRING
    ;

dialectVersionOperator
    : ASSIGN
    | EQ
    | NEQ
    | LT
    | LTE
    | GT
    | GTE
    | CARET
    | TILDE
    ;


/* ============================================================================
 * 9. DIALECT STATUS
 * ========================================================================== */

/*
 * Status is descriptive metadata only.
 *
 * It does not determine whether a dialect can execute.
 */
dialectStatusDeclaration
    : STATUS dialectStatus SEMICOLON
    ;

dialectStatus
    : identifier
    ;


/* ============================================================================
 * 10. DIALECT FEATURES
 * ========================================================================== */

/*
 * A feature describes a language/semantic facility.
 *
 * It must not be confused with a physical hardware feature.
 *
 * Examples:
 *
 *     feature dynamic_circuits;
 *     feature mid_circuit_measurement;
 *     feature tensor_operations;
 *     feature hardware_description;
 */
dialectFeatureDeclaration
    : FEATURE identifier dialectFeatureType?
      dialectFeatureConstraint*
      SEMICOLON
    ;

dialectFeatureType
    : COLON qualifiedName
    ;

dialectFeatureConstraint
    : WHERE dialectPredicate
    ;


/* ============================================================================
 * 11. CAPABILITIES
 * ========================================================================== */

/*
 * Capabilities describe what a dialect contract exposes or expects.
 *
 * They do not perform capability discovery.
 */
dialectCapabilityDeclaration
    : CAPABILITY qualifiedName dialectCapabilityBody? SEMICOLON
    ;

dialectCapabilityBody
    : LBRACE dialectCapabilityMember* RBRACE
    ;

dialectCapabilityMember
    : dialectAttribute
    | dialectRequirementDeclaration
    | dialectMetadataDeclaration
    ;


/* ============================================================================
 * 12. REQUIREMENTS
 * ========================================================================== */

/*
 * Requirements are semantic prerequisites.
 *
 * They are NOT target selections.
 *
 * Example:
 *
 *     requires quantum::dynamic_control;
 *
 * does not mean:
 *
 *     use device X;
 *
 * It only states a semantic requirement.
 */
dialectRequirementDeclaration
    : REQUIRES dialectRequirementExpression SEMICOLON
    ;

dialectRequirementExpression
    : dialectRequirementTerm
      (dialectLogicalOperator dialectRequirementTerm)*
    ;

dialectRequirementTerm
    : dialectRequirementAtom
    | LPAREN dialectRequirementExpression RPAREN
    ;

dialectRequirementAtom
    : qualifiedName dialectVersionConstraint?
    | dialectCapabilityReference
    ;

dialectCapabilityReference
    : CAPABILITY qualifiedName
    ;

dialectLogicalOperator
    : AND
    | OR
    ;


/* ============================================================================
 * 13. EXTENSIONS
 * ========================================================================== */

/*
 * A dialect extension introduces a source-level semantic construct.
 *
 * The declaration is intentionally structural.
 *
 * The extension's meaning is defined by the dialect registry/semantic layer,
 * not by this universal grammar.
 */
dialectExtensionDeclaration
    : EXTENSION qualifiedName
      dialectExtensionSignature?
      dialectExtensionConstraint*
      dialectExtensionBody?
      SEMICOLON?
    ;

dialectExtensionSignature
    : LPAREN dialectParameterList? RPAREN
    ;

dialectParameterList
    : dialectParameter (COMMA dialectParameter)*
    ;

dialectParameter
    : identifier dialectParameterType?
      dialectDefaultValue?
    ;

dialectParameterType
    : COLON qualifiedName
    ;

dialectDefaultValue
    : ASSIGN dialectValue
    ;

dialectExtensionConstraint
    : WHERE dialectPredicate
    ;

dialectExtensionBody
    : LBRACE dialectExtensionMember* RBRACE
    ;

dialectExtensionMember
    : dialectFeatureDeclaration
    | dialectCapabilityDeclaration
    | dialectRequirementDeclaration
    | dialectMetadataDeclaration
    | dialectAttribute
    ;


/* ============================================================================
 * 14. COMPATIBILITY
 * ========================================================================== */

/*
 * Compatibility declarations describe relationships.
 *
 * They do not implement compatibility checking.
 *
 * Semantic analysis owns:
 *
 *   - version comparison;
 *   - feature compatibility;
 *   - migration;
 *   - deprecation;
 *   - conflict handling.
 */
dialectCompatibilityDeclaration
    : COMPATIBLE WITH dialectReferenceList SEMICOLON
    ;


/* ============================================================================
 * 15. CONFLICTS
 * ========================================================================== */

/*
 * Explicit conflict declarations allow a dialect to state that two semantic
 * contracts cannot be simultaneously selected.
 *
 * The grammar does not decide whether the conflict actually exists.
 */
dialectConflictDeclaration
    : CONFLICTS WITH dialectReferenceList SEMICOLON
    ;


/* ============================================================================
 * 16. NAMESPACES
 * ========================================================================== */

/*
 * Namespace nesting is unbounded by grammar design.
 */
dialectNamespaceDeclaration
    : NAMESPACE qualifiedName
      LBRACE
      dialectMember*
      RBRACE
    ;


/* ============================================================================
 * 17. METADATA
 * ========================================================================== */

/*
 * Metadata is intentionally open.
 *
 * Unknown metadata is syntactically representable and semantically validated
 * by the dialect registry.
 */
dialectMetadataDeclaration
    : METADATA identifier
      (ASSIGN dialectValue)?
      SEMICOLON
    ;


/* ============================================================================
 * 18. ATTRIBUTES
 * ========================================================================== */

dialectAttribute
    : AT identifier
      dialectAttributeArguments?
    ;

dialectAttributeArguments
    : LPAREN dialectValueList? RPAREN
    ;


/* ============================================================================
 * 19. PREDICATES
 * ========================================================================== */

/*
 * Predicate syntax is deliberately structural.
 *
 * It must not execute code.
 *
 * It must not inspect hardware.
 */
dialectPredicate
    : dialectPredicateAtom
    | LPAREN dialectPredicate RPAREN
    | NOT dialectPredicate
    | dialectPredicate AND dialectPredicate
    | dialectPredicate OR dialectPredicate
    ;

dialectPredicateAtom
    : qualifiedName
    | qualifiedName comparisonOperator dialectValue
    ;

comparisonOperator
    : EQ
    | NEQ
    | LT
    | LTE
    | GT
    | GTE
    ;


/* ============================================================================
 * 20. VALUES
 * ========================================================================== */

/*
 * Values remain syntactic.
 *
 * Their semantic type is determined later.
 */
dialectValue
    : identifier
    | qualifiedName
    | STRING
    | INTEGER
    | FLOAT
    | TRUE
    | FALSE
    | NIL
    | NULL
    | dialectListValue
    | dialectMapValue
    ;

dialectListValue
    : LBRACKET dialectValueList? RBRACKET
    ;

dialectValueList
    : dialectValue (COMMA dialectValue)* COMMA?
    ;

dialectMapValue
    : LBRACE dialectMapEntry* RBRACE
    ;

dialectMapEntry
    : identifier COLON dialectValue COMMA?
    ;


/* ============================================================================
 * 21. DIALECT EXTENSION PARAMETERS
 * ========================================================================== */

dialectExtensionParameterList
    : dialectParameter (COMMA dialectParameter)*
    ;


/* ============================================================================
 * 22. UNIVERSAL DIALECT DECLARATION LIST
 * ========================================================================== */

/*
 * Reusable list for parser integration.
 */
dialectDeclarationList
    : dialectDeclaration*
    ;


/* ============================================================================
 * 23. OPTIONAL DIALECT DECLARATION
 * ========================================================================== */

optionalDialectDeclaration
    : dialectDeclaration?
    ;


/* ============================================================================
 * 24. DIALECT REFERENCE LIST
 * ========================================================================== */

dialectNameList
    : qualifiedName (COMMA qualifiedName)*
    ;


/* ============================================================================
 * 25. TARGET-NEUTRAL REQUIREMENT GROUP
 * ========================================================================== */

/*
 * This rule deliberately models requirements rather than target selection.
 *
 * Example:
 *
 *     requires {
 *         quantum::measurement;
 *         quantum::dynamic_control;
 *     }
 *
 * The actual machine capable of satisfying those requirements is selected
 * later.
 */
dialectRequirementGroup
    : REQUIRES LBRACE dialectRequirementItem* RBRACE
    ;

dialectRequirementItem
    : qualifiedName dialectVersionConstraint? SEMICOLON
    ;


/* ============================================================================
 * 26. PROVIDES
 * ========================================================================== */

/*
 * `provides` describes semantic capabilities supplied by a dialect.
 *
 * It does not mean that a physical machine currently provides them.
 */
dialectProvidesDeclaration
    : PROVIDES dialectQualifiedNameList SEMICOLON
    ;

dialectQualifiedNameList
    : qualifiedName (COMMA qualifiedName)*
    ;


/* ============================================================================
 * 27. DIALECT CONTRACT
 * ========================================================================== */

/*
 * A reusable contract can state both requirements and provided capabilities.
 */
dialectContract
    : dialectContractItem*
    ;

dialectContractItem
    : dialectVersionDeclaration
    | dialectExtendsDeclaration
    | dialectRequirementDeclaration
    | dialectRequirementGroup
    | dialectCapabilityDeclaration
    | dialectProvidesDeclaration
    | dialectCompatibilityDeclaration
    | dialectConflictDeclaration
    ;


/* ============================================================================
 * 28. RESERVED EXTENSION POINT
 * ========================================================================== */

/*
 * This rule exists so future dialect metadata can be introduced through
 * dialect-specific semantic registration without modifying this grammar for
 * every new domain.
 */
dialectCustomMember
    : identifier dialectCustomArguments? dialectCustomBody? SEMICOLON?
    ;

dialectCustomArguments
    : LPAREN dialectValueList? RPAREN
    ;

dialectCustomBody
    : LBRACE dialectMember* RBRACE
    ;