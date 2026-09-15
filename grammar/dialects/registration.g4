/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/registration.g4
 *
 * Role:
 *     Canonical, domain-neutral parser grammar for registering, importing,
 *     composing, describing, versioning, and declaring compatibility for
 *     Zamani language dialects.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL DIALECT REGISTRATION CONTRACT.
 *
 * A dialect is a controlled extension of the Zamani language.
 *
 * This file is deliberately domain-neutral.
 *
 * It may describe dialects for:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware
 *     distributed computing
 *     AI
 *     scientific computing
 *     data processing
 *     networking
 *     cryptography
 *     accelerators
 *     future computational domains
 *
 * without enumerating those domains in this grammar.
 *
 * ============================================================================
 * PRIMARY PRINCIPLE
 * ============================================================================
 *
 * A dialect describes language syntax and/or semantics.
 *
 * A dialect registration MUST NOT become:
 *
 *     a hardware description
 *     a device selector
 *     a topology description
 *     a resource allocator
 *     a scheduler
 *     a router
 *     an optimizer
 *     a QEC implementation
 *     a ZQN implementation
 *     a runtime implementation
 *
 * The source-level dependency direction is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +----> dialect registry
 *       +----> version compatibility
 *       +----> capability resolution
 *       +----> extension validation
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +----> classical IR
 *       +----> quantum::ir
 *       +----> HDL/hardware representation
 *       +----> other domain representations
 *       |
 *       v
 *     optimization / routing / scheduling / lowering
 *       |
 *       v
 *     hardware / runtime
 *
 * There MUST be no reverse dependency from this grammar to those subsystems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialect registration MUST preserve:
 *
 *     Program Once
 *         |
 *         v
 *     Portable Source Semantics
 *         |
 *         v
 *     Compile Once
 *         |
 *         v
 *     Capability / Target Adaptation
 *         |
 *         v
 *     Run Everywhere
 *         |
 *         v
 *     Run Anywhere
 *         |
 *         v
 *     Run Forever
 *
 * A dialect MUST NOT make a temporary hardware characteristic part of the
 * permanent language meaning unless that characteristic is explicitly part
 * of the dialect's semantics.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - generic dialect registration syntax;
 *     - dialect identity references;
 *     - dialect declaration structure;
 *     - dialect registration properties;
 *     - dialect composition references;
 *     - dialect imports;
 *     - dialect aliases;
 *     - dialect requirements;
 *     - dialect capabilities;
 *     - dialect extension descriptors;
 *     - dialect syntax descriptors;
 *     - dialect semantic descriptors;
 *     - dialect lowering descriptors;
 *     - dialect compatibility declarations;
 *     - dialect deprecation declarations;
 *     - dialect registration metadata;
 *     - open-world dialect property representation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified-name syntax;
 *     - lexical tokens;
 *     - version comparison algorithms;
 *     - semantic version interpretation;
 *     - capability discovery;
 *     - capability implementation;
 *     - resource discovery;
 *     - resource allocation;
 *     - hardware discovery;
 *     - hardware topology;
 *     - target selection;
 *     - physical qubit mapping;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - simulation;
 *     - runtime execution;
 *     - vendor implementation.
 *
 * ============================================================================
 * NON-OWNERSHIP IS INTENTIONAL
 * ============================================================================
 *
 * The grammar describes registration syntax.
 *
 * Semantic analysis decides:
 *
 *     - whether a dialect exists;
 *     - whether its version is valid;
 *     - whether it is compatible;
 *     - whether dependencies exist;
 *     - whether requirements can be satisfied;
 *     - whether capabilities are valid;
 *     - whether extensions conflict;
 *     - whether lowering exists;
 *     - whether the dialect may be used.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally contains no finite limits for:
 *
 *     dialect count
 *     registration count
 *     namespace depth
 *     extension count
 *     property count
 *     requirement count
 *     capability count
 *     dependency count
 *     composition count
 *     syntax declarations
 *     semantic declarations
 *     compatibility declarations
 *     dialect members
 *
 * Repetition is represented using:
 *
 *     *
 *     +
 *
 * rather than fixed constants.
 *
 * Therefore the language does NOT define:
 *
 *     MAX_DIALECTS
 *     MAX_EXTENSIONS
 *     MAX_NAMESPACE_DEPTH
 *     MAX_CAPABILITIES
 *     MAX_REQUIREMENTS
 *
 * Actual parser/resource limits remain implementation/resource concerns.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar MUST NOT contain a closed enumeration such as:
 *
 *     dialect
 *         : quantum
 *         | openqasm
 *         | qiskit
 *         | verilog
 *         | ...
 *
 * Such an enumeration would make the language depend on today's known
 * technologies and would violate long-term POCO-REAF extensibility.
 *
 * Dialect identities are symbolic qualified names.
 *
 * Examples include:
 *
 *     quantum::standard
 *     quantum::openqasm
 *     classical::numeric
 *     hdl::rtl
 *     hardware::fpga
 *     ai::tensor
 *     future::computing::extension
 *     organization::domain::dialect
 *     vendor::domain::experimental
 *
 * The grammar does not decide whether those names exist.
 *
 * ============================================================================
 * NAMESPACE PRINCIPLE
 * ============================================================================
 *
 * A dialect identity is symbolic language information.
 *
 * It is NOT:
 *
 *     a filesystem path
 *     a URL
 *     a hardware address
 *     a device identifier
 *     a physical location
 *     a network endpoint
 *     a QPU identifier
 *     a physical qubit identifier
 *
 * ============================================================================
 * VERSION PRINCIPLE
 * ============================================================================
 *
 * Version syntax is represented structurally here.
 *
 * Version interpretation belongs to semantic compatibility infrastructure.
 *
 * This grammar MUST NOT embed:
 *
 *     semantic-version comparison algorithms
 *     compatibility algorithms
 *     migration algorithms
 *
 * A dialect version and Zamani language version are distinct concepts.
 *
 * ============================================================================
 * CAPABILITY PRINCIPLE
 * ============================================================================
 *
 * Capabilities are symbolic requirements/provisions.
 *
 * A capability MUST NOT inherently select a machine.
 *
 * For example:
 *
 *     requires quantum::dynamic_control;
 *
 * means that the semantic capability is required.
 *
 * It does NOT mean:
 *
 *     use device X
 *     use QPU X
 *     use GPU X
 *     use CPU X
 *
 * Capability resolution occurs downstream.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Forbidden source-level registration semantics include assumptions such as:
 *
 *     device = "specific-device"
 *     qpu = 3
 *     qubits = 32
 *     topology = "fixed-grid"
 *
 * unless such data is explicitly part of a separate target/hardware
 * description rather than the portable dialect registration.
 *
 * Dialect registration describes a language contract.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum dialects may use this registration mechanism.
 *
 * This grammar does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumCircuit
 *     GateKind
 *     QuantumOperation
 *
 * It does not import quantum::ir.
 *
 * Quantum dialect semantics eventually lower through:
 *
 *     AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *
 * ============================================================================
 * QEC / ZQN BOUNDARY
 * ============================================================================
 *
 * A dialect may register syntax describing QEC or fault/noise intent.
 *
 * It MUST NOT implement QEC or ZQN here.
 *
 * QEC remains owned by the QEC subsystem.
 *
 * ZQN remains the owner of fault/noise semantics.
 *
 * ============================================================================
 * SCHEDULING / ROUTING BOUNDARY
 * ============================================================================
 *
 * A dialect may declare semantic requirements or capabilities related to:
 *
 *     timing
 *     ordering
 *     dynamic execution
 *     synchronization
 *     placement
 *
 * This grammar does not implement:
 *
 *     scheduling
 *     routing
 *     placement algorithms
 *     resource allocation
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
 *     - no runtime callbacks;
 *     - no hardware callbacks;
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

parser grammar DialectRegistration;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 1. TOP-LEVEL DIALECT REGISTRATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     dialect quantum::example {
 *         ...
 *     }
 *
 * The identity is a symbolic qualified name.
 *
 * It is intentionally not a device identifier.
 */
dialectRegistration
    : DIALECT
      qualifiedName
      dialectRegistrationHeader?
      LBRACE
      dialectRegistrationMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 2. REGISTRATION HEADER
 * ============================================================================
 *
 * The header is intentionally compact.
 *
 * Version, stability, ownership, and compatibility metadata can be expressed
 * using the open property mechanism below.
 *
 * Keeping this structure open avoids forcing every future dialect metadata
 * concept into a globally reserved keyword.
 */
dialectRegistrationHeader
    : dialectRegistrationAnnotation*
    ;


/*
 * ============================================================================
 * 3. REGISTRATION MEMBER DISPATCH
 * ============================================================================
 *
 * The member dispatcher provides deterministic structural ownership.
 *
 * Standardized constructs have dedicated rules.
 *
 * Future metadata can use dialectRegistrationProperty without changing this
 * grammar for every new property.
 */
dialectRegistrationMember
    : dialectRegistrationImport
    | dialectRegistrationUse
    | dialectRegistrationExtends
    | dialectRegistrationRequires
    | dialectRegistrationProvides
    | dialectRegistrationExtension
    | dialectRegistrationCompatibility
    | dialectRegistrationDeprecation
    | dialectRegistrationSyntax
    | dialectRegistrationSemantics
    | dialectRegistrationLowering
    | dialectRegistrationProperty
    | dialectRegistrationAnnotation
    ;


/*
 * ============================================================================
 * 4. DIALECT IMPORT
 * ============================================================================
 *
 * Imports another dialect registration contract.
 *
 * Importing a dialect does not:
 *
 *     - select hardware;
 *     - load a backend;
 *     - discover a device;
 *     - allocate resources;
 *     - execute code.
 */
dialectRegistrationImport
    : IMPORT
      qualifiedName
      dialectRegistrationAlias?
      SEMICOLON
    ;

dialectRegistrationAlias
    : AS
      identifier
    ;


/*
 * ============================================================================
 * 5. DIALECT USE
 * ============================================================================
 *
 * `use` activates a registered dialect for the relevant source scope.
 *
 * Semantic analysis decides whether the dialect exists and whether it is
 * compatible with the current source program.
 */
dialectRegistrationUse
    : USE
      qualifiedName
      dialectRegistrationAlias?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. DIALECT COMPOSITION / INHERITANCE
 * ============================================================================
 *
 * A dialect may extend any number of other dialects.
 *
 * There is no grammar-level maximum.
 *
 * Semantic analysis owns:
 *
 *     - cycle detection;
 *     - conflict detection;
 *     - inheritance resolution;
 *     - compatibility;
 *     - feature merging.
 */
dialectRegistrationExtends
    : EXTENDS
      dialectReferenceList
      SEMICOLON
    ;

dialectReferenceList
    : dialectReference
      (COMMA dialectReference)*
    ;

dialectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 7. DIALECT REQUIREMENTS
 * ============================================================================
 *
 * Requirements express semantic prerequisites.
 *
 * Examples:
 *
 *     requires quantum::dynamic_control;
 *     requires hardware::programmable_logic;
 *     requires distributed::messaging;
 *
 * Requirements do not select concrete machines.
 */
dialectRegistrationRequires
    : REQUIRES
      dialectRequirementExpression
      SEMICOLON
    ;

dialectRequirementExpression
    : dialectRequirementAtom
    | dialectRequirementGroup
    ;

dialectRequirementAtom
    : qualifiedName
      dialectRequirementConstraint?
    ;

dialectRequirementConstraint
    : WHERE
      dialectPredicate
    ;

dialectRequirementGroup
    : LBRACE
      dialectRequirementItem*
      RBRACE
    ;

dialectRequirementItem
    : dialectRequirementAtom
      COMMA?
    ;


/*
 * ============================================================================
 * 8. DIALECT PROVISIONS / CAPABILITIES
 * ============================================================================
 *
 * A dialect may declare capabilities it provides.
 *
 * Capability identity is symbolic.
 *
 * Capability availability on a particular machine is NOT decided here.
 */
dialectRegistrationProvides
    : dialectRegistrationKeywordProvides
      dialectCapabilityList
      SEMICOLON
    ;

dialectRegistrationKeywordProvides
    : IDENTIFIER
    ;

dialectCapabilityList
    : dialectCapability
      (COMMA dialectCapability)*
    ;

dialectCapability
    : qualifiedName
      dialectCapabilityVersionConstraint?
    ;

dialectCapabilityVersionConstraint
    : dialectVersionConstraint
    ;


/*
 * ============================================================================
 * 9. DIALECT EXTENSION DECLARATION
 * ============================================================================
 *
 * An extension declares a source-level language facility belonging to the
 * dialect.
 *
 * The name is symbolic and open-ended.
 *
 * The grammar does not decide what the extension means.
 */
dialectRegistrationExtension
    : dialectRegistrationKeywordExtension
      qualifiedName
      dialectExtensionSignature?
      dialectExtensionBody?
      SEMICOLON?
    ;

dialectRegistrationKeywordExtension
    : IDENTIFIER
    ;

dialectExtensionSignature
    : LPAREN
      dialectParameterList?
      RPAREN
    ;

dialectParameterList
    : dialectParameter
      (COMMA dialectParameter)*
    ;

dialectParameter
    : identifier
      (COLON qualifiedName)?
      (ASSIGN dialectValue)?
    ;

dialectExtensionBody
    : LBRACE
      dialectExtensionMember*
      RBRACE
    ;

dialectExtensionMember
    : dialectRegistrationAnnotation
    | dialectRegistrationProperty
    | dialectRegistrationRequires
    | dialectRegistrationProvides
    | dialectRegistrationSyntax
    | dialectRegistrationSemantics
    | dialectRegistrationLowering
    ;


/*
 * ============================================================================
 * 10. SYNTAX DESCRIPTOR
 * ============================================================================
 *
 * Describes source syntax owned by the dialect.
 *
 * This is metadata about syntax.
 *
 * It is NOT a second parser implementation embedded in the registration
 * grammar.
 *
 * Concrete grammar implementation remains part of the dialect grammar files.
 */
dialectRegistrationSyntax
    : dialectRegistrationKeywordSyntax
      dialectDescriptorBody
    ;

dialectRegistrationKeywordSyntax
    : IDENTIFIER
    ;

dialectDescriptorBody
    : LBRACE
      dialectDescriptorItem*
      RBRACE
    ;

dialectDescriptorItem
    : dialectRegistrationProperty
    | dialectRegistrationAnnotation
    ;


/*
 * ============================================================================
 * 11. SEMANTIC DESCRIPTOR
 * ============================================================================
 *
 * Describes semantic ownership and interpretation of registered extensions.
 *
 * This grammar does not implement semantic analysis.
 */
dialectRegistrationSemantics
    : dialectRegistrationKeywordSemantics
      dialectDescriptorBody
    ;

dialectRegistrationKeywordSemantics
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 12. LOWERING DESCRIPTOR
 * ============================================================================
 *
 * A dialect may identify a semantic lowering boundary.
 *
 * The value is symbolic.
 *
 * It MUST NOT embed implementation code or runtime callbacks.
 */
dialectRegistrationLowering
    : dialectRegistrationKeywordLowering
      dialectLoweringBody
    ;

dialectRegistrationKeywordLowering
    : IDENTIFIER
    ;

dialectLoweringBody
    : LBRACE
      dialectDescriptorItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 13. COMPATIBILITY DECLARATION
 * ============================================================================
 *
 * Compatibility is declarative syntax only.
 *
 * Semantic analysis determines whether a compatibility relationship actually
 * holds.
 *
 * Compatibility dimensions may include:
 *
 *     source
 *     AST
 *     semantic
 *     IR
 *     runtime
 *     forward
 *     backward
 *
 * These remain symbolic to avoid hard-coding today's compatibility model.
 */
dialectRegistrationCompatibility
    : dialectRegistrationKeywordCompatibility
      dialectCompatibilityExpression
      SEMICOLON
    ;

dialectRegistrationKeywordCompatibility
    : IDENTIFIER
    ;

dialectCompatibilityExpression
    : dialectCompatibilityAtom
    | dialectCompatibilityGroup
    ;

dialectCompatibilityAtom
    : qualifiedName
      dialectVersionConstraint?
    ;

dialectCompatibilityGroup
    : LBRACE
      dialectCompatibilityItem*
      RBRACE
    ;

dialectCompatibilityItem
    : dialectCompatibilityAtom
      COMMA?
    ;


/*
 * ============================================================================
 * 14. DEPRECATION
 * ============================================================================
 *
 * Deprecation is source-level metadata.
 *
 * It does not remove functionality by itself.
 *
 * Semantic/compiler policy determines:
 *
 *     - warning behavior;
 *     - migration behavior;
 *     - removal release;
 *     - compatibility behavior.
 */
dialectRegistrationDeprecation
    : dialectRegistrationKeywordDeprecation
      dialectDeprecationBody
    ;

dialectRegistrationKeywordDeprecation
    : IDENTIFIER
    ;

dialectDeprecationBody
    : LBRACE
      dialectDescriptorItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 15. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * This is the principal forward-compatibility mechanism.
 *
 * New dialect metadata does not automatically require a new Zamani keyword.
 *
 * Example conceptual forms:
 *
 *     version = "1.0.0";
 *     stability = stable;
 *     owner = organization::domain;
 *     documentation = "....";
 *
 * Unknown properties remain structurally representable and are validated by
 * semantic dialect infrastructure.
 *
 * This prevents grammar churn as the language ecosystem grows.
 */
dialectRegistrationProperty
    : identifier
      ASSIGN
      dialectValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. DIALECT ANNOTATION
 * ============================================================================
 *
 * Annotations attach non-core metadata without creating new global keywords.
 *
 * The annotation name remains symbolic.
 *
 * Semantic analysis determines whether an annotation is legal and what it
 * means.
 */
dialectRegistrationAnnotation
    : AT
      identifier
      dialectAnnotationArguments?
    ;

dialectAnnotationArguments
    : LPAREN
      dialectValueList?
      RPAREN
    ;


/*
 * ============================================================================
 * 17. DIALECT VERSION CONSTRAINT
 * ============================================================================
 *
 * Version syntax is structural.
 *
 * Interpretation is semantic.
 *
 * Examples:
 *
 *     version = "1.0";
 *     >= "1.0";
 *     < "2.0";
 *
 * A dialect can therefore evolve without requiring grammar-level numeric
 * limits.
 */
dialectVersionConstraint
    : dialectVersionOperator
      dialectVersionValue
    ;

dialectVersionOperator
    : ASSIGN
    | EQ
    | LT
    | LE
    | GT
    | GE
    ;

dialectVersionValue
    : STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | identifier
    ;


/*
 * ============================================================================
 * 18. DIALECT PREDICATE
 * ============================================================================
 *
 * Predicates remain symbolic.
 *
 * This rule intentionally does not interpret resource quantities, machine
 * properties, or capabilities.
 */
dialectPredicate
    : dialectPredicateAtom
    | dialectPredicateGroup
    ;

dialectPredicateAtom
    : qualifiedName
      dialectPredicateOperator?
      dialectValue?
    ;

dialectPredicateOperator
    : EQ
    | LT
    | LE
    | GT
    | GE
    ;

dialectPredicateGroup
    : LPAREN
      dialectPredicate
      RPAREN
    ;


/*
 * ============================================================================
 * 19. DIALECT VALUES
 * ============================================================================
 *
 * Values remain syntactic data.
 *
 * Their semantic type is determined by the owning subsystem.
 *
 * This deliberately permits future metadata without requiring a new grammar
 * release for every new kind of dialect property.
 */
dialectValue
    : dialectLiteral
    | qualifiedName
    | dialectList
    | dialectMap
    ;

dialectLiteral
    : STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | TRUE
    | FALSE
    | identifier
    ;

dialectList
    : LBRACKET
      dialectValueList?
      RBRACKET
    ;

dialectValueList
    : dialectValue
      (COMMA dialectValue)*
      COMMA?
    ;

dialectMap
    : LBRACE
      dialectMapEntry*
      RBRACE
    ;

dialectMapEntry
    : identifier
      ASSIGN
      dialectValue
      COMMA?
    ;


/*
 * ============================================================================
 * 20. DIALECT DECLARATION ALIAS
 * ============================================================================
 *
 * Alias is intentionally limited to symbolic source names.
 *
 * It does not create another dialect identity.
 */
dialectAlias
    : identifier
    ;


/*
 * ============================================================================
 * 21. DIALECT REGISTRATION LIST
 * ============================================================================
 *
 * Useful to generic parser integration and tooling.
 *
 * There is no finite list size.
 */
dialectRegistrationList
    : dialectRegistration+
    ;


/*
 * ============================================================================
 * 22. DIALECT REFERENCE LIST WITH OPTIONAL ALIASES
 * ============================================================================
 */
dialectReferenceWithAlias
    : dialectReference
      dialectRegistrationAlias?
    ;

dialectReferenceWithAliasList
    : dialectReferenceWithAlias
      (COMMA dialectReferenceWithAlias)*
    ;


/*
 * ============================================================================
 * 23. OPTIONAL DIALECT REFERENCE
 * ============================================================================
 */
optionalDialectReference
    : dialectReference?
    ;


/*
 * ============================================================================
 * 24. DIALECT REGISTRATION CONTEXT
 * ============================================================================
 *
 * Generic wrapper for consumers that need to distinguish a dialect
 * registration from an ordinary qualified name.
 */
dialectRegistrationContext
    : dialectRegistration
    ;


/*
 * ============================================================================
 * 25. DIALECT IDENTITY
 * ============================================================================
 *
 * Identity is deliberately delegated to canonical Names.
 *
 * No duplicate qualified-name grammar is introduced here.
 */
dialectIdentity
    : qualifiedName
    ;


/*
 * ============================================================================
 * 26. DIALECT PROPERTY KEY
 * ============================================================================
 *
 * Property keys are symbolic names.
 *
 * Semantic validation determines which keys are standard, experimental,
 * vendor-defined, or invalid.
 */
dialectPropertyKey
    : identifier
    ;


/*
 * ============================================================================
 * 27. DIALECT PROPERTY
 * ============================================================================
 *
 * Named separately for AST/tooling consumers.
 */
dialectProperty
    : dialectPropertyKey
      ASSIGN
      dialectValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 28. DIALECT METADATA BLOCK
 * ============================================================================
 *
 * Generic metadata container.
 */
dialectMetadataBlock
    : LBRACE
      dialectProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 29. DIALECT DEPENDENCY
 * ============================================================================
 *
 * A dependency is symbolic.
 *
 * Dependency resolution belongs outside the parser.
 */
dialectDependency
    : dialectReference
      dialectVersionConstraint?
    ;

dialectDependencyList
    : dialectDependency
      (COMMA dialectDependency)*
    ;


/*
 * ============================================================================
 * 30. DIALECT DEPENDENCY DECLARATION
 * ============================================================================
 */
dialectDependencyDeclaration
    : IMPORT
      dialectDependencyList
      SEMICOLON
    ;


/*
 * ============================================================================
 * 31. DIALECT CAPABILITY DECLARATION
 * ============================================================================
 */
dialectCapabilityDeclaration
    : dialectRegistrationKeywordCapability
      dialectCapabilityList
      SEMICOLON
    ;

dialectRegistrationKeywordCapability
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 32. DIALECT REQUIREMENT LIST
 * ============================================================================
 */
dialectRequirementList
    : dialectRequirementExpression
      (COMMA dialectRequirementExpression)*
    ;


/*
 * ============================================================================
 * 33. OPTIONAL REQUIREMENT LIST
 * ============================================================================
 */
optionalDialectRequirementList
    : dialectRequirementList?
    ;


/*
 * ============================================================================
 * 34. OPTIONAL CAPABILITY LIST
 * ============================================================================
 */
optionalDialectCapabilityList
    : dialectCapabilityList?
    ;


/*
 * ============================================================================
 * 35. DIALECT REGISTRATION PROPERTY LIST
 * ============================================================================
 */
dialectRegistrationPropertyList
    : dialectRegistrationProperty*
    ;


/*
 * ============================================================================
 * 36. DIALECT REGISTRATION ANNOTATION LIST
 * ============================================================================
 */
dialectRegistrationAnnotationList
    : dialectRegistrationAnnotation*
    ;


/*
 * ============================================================================
 * 37. DIALECT EXTENSION LIST
 * ============================================================================
 */
dialectExtensionList
    : dialectRegistrationExtension*
    ;


/*
 * ============================================================================
 * 38. DIALECT COMPATIBILITY LIST
 * ============================================================================
 */
dialectCompatibilityList
    : dialectRegistrationCompatibility*
    ;


/*
 * ============================================================================
 * 39. DIALECT MEMBER LIST
 * ============================================================================
 */
dialectRegistrationMemberList
    : dialectRegistrationMember*
    ;


/*
 * ============================================================================
 * 40. EXPLICIT DIALECT REGISTRATION DOCUMENT
 * ============================================================================
 *
 * This rule is useful when a parser entry point needs to parse a sequence of
 * dialect registrations without importing the complete Zamani program grammar.
 */
dialectRegistrationDocument
    : dialectRegistration*
      EOF
    ;