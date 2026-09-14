/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/hardware-dialects.g4
 *
 * Role:
 *     Production parser grammar for open-world HDL/hardware dialect
 *     declarations and references.
 *
 * Language architecture:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     HardwareDialects
 *       |
 *       v
 *     frontend AST
 *       |
 *       +--> name resolution
 *       +--> dialect registry
 *       +--> version compatibility
 *       +--> capability analysis
 *       +--> requirement analysis
 *       +--> type/effect/resource analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> HDL/hardware IR
 *       +--> quantum::ir where quantum semantics exist
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     scheduling / routing / lowering
 *       |
 *       v
 *     hardware HAL / runtime
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
 * A hardware dialect is a SOURCE-LEVEL LANGUAGE CONTRACT.
 *
 * It describes a family of hardware-oriented syntax and semantic extensions
 * without binding a Zamani program to one physical machine.
 *
 * This grammar therefore owns:
 *
 *     - hardware dialect declarations;
 *     - hardware dialect references;
 *     - dialect imports;
 *     - dialect aliases;
 *     - dialect composition;
 *     - dialect extension declarations;
 *     - dialect feature declarations;
 *     - dialect capability requirements;
 *     - dialect implementation contracts;
 *     - dialect compatibility declarations;
 *     - dialect-scoped metadata.
 *
 * This grammar does NOT own:
 *
 *     - hardware discovery;
 *     - physical devices;
 *     - boards;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - QPUs;
 *     - physical addresses;
 *     - topology discovery;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - timing realization;
 *     - synthesis;
 *     - backend selection;
 *     - runtime execution;
 *     - resource allocation;
 *     - vendor APIs;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A hardware dialect MUST NOT make a source program depend on a particular
 * physical realization unless that physical dependency is explicitly part of
 * the program's declared semantic contract.
 *
 * Consequently this grammar contains NO fixed limits for:
 *
 *     - modules;
 *     - dialects;
 *     - dialect members;
 *     - features;
 *     - capabilities;
 *     - resources;
 *     - devices;
 *     - CPUs;
 *     - cores;
 *     - GPUs;
 *     - FPGA resources;
 *     - ASIC resources;
 *     - quantum devices;
 *     - qubits;
 *     - ports;
 *     - buses;
 *     - nodes;
 *     - topology size;
 *     - hierarchy depth.
 *
 * Any practical limit comes from:
 *
 *     source size
 *     parser implementation
 *     memory
 *     compiler policy
 *     target resources
 *     deployment resources
 *
 * and MUST NOT be represented as a grammar-level machine limit.
 *
 * ============================================================================
 * OPEN-WORLD DIALECT MODEL
 * ============================================================================
 *
 * The grammar deliberately does NOT contain:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     X86
 *     ARM
 *     RISCV
 *     NVIDIA
 *     AMD
 *     Intel
 *     Xilinx
 *     etc.
 *
 * as an exhaustive dialect enumeration.
 *
 * Such names may occur as ordinary qualified identifiers when a particular
 * dialect or target model defines them.
 *
 * This permits future technologies to be represented without modifying this
 * grammar.
 *
 * Examples:
 *
 *     hardware::rtl
 *     hardware::fpga
 *     hardware::asic
 *     hardware::future::reconfigurable
 *     organization::accelerator::v1
 *     vendor::extension::experimental
 *
 * Their existence, compatibility and realizability are semantic concerns.
 *
 * ============================================================================
 * SEMANTIC / PHYSICAL SEPARATION
 * ============================================================================
 *
 * The following are intentionally different concepts:
 *
 *     dialect
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     target
 *     resource
 *     implementation
 *
 * For example:
 *
 *     requires hardware::streaming
 *
 * does NOT mean:
 *
 *     use device X
 *
 * Likewise:
 *
 *     capability hardware::pipeline
 *
 * does NOT mean:
 *
 *     allocate N pipeline stages
 *
 * Physical realization belongs to later compilation and execution layers.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A hardware dialect may describe syntax that eventually contains quantum
 * semantics, but this grammar MUST NOT define quantum IR.
 *
 * It MUST NOT reference:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumOperation
 *     QuantumCircuit
 *
 * or any Rust implementation type.
 *
 * If a dialect construct lowers to quantum semantics, the semantic frontend
 * is responsible for producing the canonical representation and, where
 * applicable, quantum::ir.
 *
 * The grammar therefore remains independent of:
 *
 *     src/quantum/ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     resilience
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Shared lexical concepts are imported from:
 *
 *     grammar/core/qualified-names.g4
 *     grammar/core/paths.g4
 *     grammar/core/versioning.g4
 *
 * The canonical lexer owns lexical spelling.
 *
 * This grammar does not create lexer rules.
 *
 * Required canonical tokens for final parser assembly are:
 *
 *     HARDWARE
 *     DIALECT
 *     IMPORT
 *     USE
 *     AS
 *     EXTENDS
 *     IMPLEMENTS
 *     REQUIRES
 *     ENSURES
 *     WHERE
 *
 * plus the canonical identifier, literal and punctuation vocabulary.
 *
 * If HARDWARE or DIALECT is not yet present in the canonical lexer, those
 * tokens MUST be added there before this grammar is assembled.
 *
 * They MUST NOT be declared here.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime calls;
 *     - no unsafe code.
 *
 * ============================================================================
 */

parser grammar HardwareDialects;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only public root rule owned by this file.
 *
 * Parser composition may invoke:
 *
 *     hardwareDialectDeclaration
 *
 * directly, or dispatch to it from the canonical hardware/HDL declaration
 * dispatcher.
 */

hardwareDialectDeclaration
    : hardwareDialectHeader
      LBRACE
      hardwareDialectMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 2. DIALECT HEADER
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     hardware dialect hardware::rtl {
 *         ...
 *     }
 *
 * The keyword sequence is represented using canonical lexical tokens.
 *
 * No vendor or machine identity is required.
 */

hardwareDialectHeader
    : HARDWARE DIALECT qualifiedName
      hardwareDialectHeaderClause*
    ;


/*
 * ============================================================================
 * 3. HEADER CLAUSES
 * ============================================================================
 *
 * Header clauses remain declarative.
 *
 * They do not select a physical target.
 */

hardwareDialectHeaderClause
    : hardwareDialectVersionClause
    | hardwareDialectExtendsClause
    | hardwareDialectImplementsClause
    | hardwareDialectRequiresClause
    | hardwareDialectEnsuresClause
    | hardwareDialectAttribute
    ;


/*
 * ============================================================================
 * 4. VERSION CLAUSE
 * ============================================================================
 *
 * Version interpretation belongs to the canonical language-version /
 * semantic-version subsystem.
 *
 * This grammar accepts a structural version expression instead of embedding
 * version comparison rules.
 */

hardwareDialectVersionClause
    : AT qualifiedName
    ;


/*
 * ============================================================================
 * 5. DIALECT EXTENSION
 * ============================================================================
 *
 * Composition is open-ended.
 *
 * No maximum inheritance/composition depth exists at grammar level.
 *
 * Cycle detection belongs to semantic analysis.
 */

hardwareDialectExtendsClause
    : EXTENDS hardwareDialectReferenceList
    ;

hardwareDialectReferenceList
    : hardwareDialectReference
      (COMMA hardwareDialectReference)*
    ;


/*
 * ============================================================================
 * 6. IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * `implements` describes a semantic contract.
 *
 * It does NOT mean:
 *
 *     execute on this device
 *
 * It does NOT select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *
 * Those concerns belong to target resolution.
 */

hardwareDialectImplementsClause
    : IMPLEMENTS hardwareDialectReferenceList
    ;


/*
 * ============================================================================
 * 7. REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe semantic prerequisites.
 *
 * They are not target selectors.
 *
 * Example:
 *
 *     requires hardware::streaming;
 *
 *     requires {
 *         hardware::pipeline,
 *         hardware::reconfigurable
 *     };
 */

hardwareDialectRequiresClause
    : REQUIRES hardwareDialectRequirement
    ;

hardwareDialectRequirement
    : hardwareDialectRequirementAtom
    | hardwareDialectRequirementSet
    ;

hardwareDialectRequirementAtom
    : qualifiedName
    ;

hardwareDialectRequirementSet
    : LBRACE
      hardwareDialectRequirementItem*
      RBRACE
    ;

hardwareDialectRequirementItem
    : hardwareDialectRequirementAtom
      COMMA?
    ;


/*
 * ============================================================================
 * 8. GUARANTEES
 * ============================================================================
 *
 * Guarantees describe semantic properties promised by the dialect contract.
 *
 * Performance, energy and reliability guarantees MUST NOT be inferred from
 * this syntax alone.
 *
 * Their semantics belong to the resource/performance contract system.
 */

hardwareDialectEnsuresClause
    : ENSURES hardwareDialectGuarantee
    ;

hardwareDialectGuarantee
    : hardwareDialectValue
    | hardwareDialectValueSet
    ;

hardwareDialectValueSet
    : LBRACE
      hardwareDialectValueItem*
      RBRACE
    ;

hardwareDialectValueItem
    : hardwareDialectValue
      COMMA?
    ;


/*
 * ============================================================================
 * 9. DIALECT MEMBERS
 * ============================================================================
 */

hardwareDialectMember
    : hardwareDialectImport
    | hardwareDialectUse
    | hardwareDialectExtendsClause
    | hardwareDialectImplementsClause
    | hardwareDialectRequiresClause
    | hardwareDialectEnsuresClause
    | hardwareDialectFeature
    | hardwareDialectCapability
    | hardwareDialectRequirementDeclaration
    | hardwareDialectConstraintDeclaration
    | hardwareDialectPreferenceDeclaration
    | hardwareDialectResourceDeclaration
    | hardwareDialectTargetDeclaration
    | hardwareDialectImplementation
    | hardwareDialectExtension
    | hardwareDialectCompatibility
    | hardwareDialectNamespace
    | hardwareDialectAttribute
    ;


/*
 * ============================================================================
 * 10. IMPORT
 * ============================================================================
 *
 * Importing a dialect imports its language contract.
 *
 * It does not import:
 *
 *     hardware state
 *     device state
 *     calibration
 *     resources
 *     runtime state
 */

hardwareDialectImport
    : IMPORT qualifiedName
      hardwareDialectAlias?
      SEMICOLON
    ;

hardwareDialectAlias
    : AS identifier
    ;


/*
 * ============================================================================
 * 11. USE
 * ============================================================================
 *
 * `use` activates a dialect within the applicable source scope.
 *
 * Resolution is semantic.
 */

hardwareDialectUse
    : USE qualifiedName
      hardwareDialectAlias?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. FEATURE
 * ============================================================================
 *
 * A feature describes a named semantic/language facility.
 *
 * It is not synonymous with a physical hardware resource.
 */

hardwareDialectFeature
    : hardwareDialectFeatureKeyword
      identifier
      hardwareDialectFeatureType?
      hardwareDialectFeatureConstraint?
      SEMICOLON
    ;

hardwareDialectFeatureKeyword
    : identifier
    ;

hardwareDialectFeatureType
    : COLON qualifiedName
    ;

hardwareDialectFeatureConstraint
    : WHERE hardwareDialectPredicate
    ;


/*
 * ============================================================================
 * 13. CAPABILITY
 * ============================================================================
 *
 * Capability declarations identify semantic capabilities.
 *
 * Availability is resolved outside the grammar.
 */

hardwareDialectCapability
    : hardwareDialectCapabilityKeyword
      qualifiedName
      hardwareDialectCapabilityBody?
      SEMICOLON
    ;

hardwareDialectCapabilityKeyword
    : identifier
    ;

hardwareDialectCapabilityBody
    : LBRACE
      hardwareDialectCapabilityMember*
      RBRACE
    ;

hardwareDialectCapabilityMember
    : hardwareDialectAttribute
    | hardwareDialectValueDeclaration
    ;


/*
 * ============================================================================
 * 14. REQUIREMENT DECLARATION
 * ============================================================================
 *
 * This separates the declaration of a requirement from a header-level
 * `requires` clause.
 */

hardwareDialectRequirementDeclaration
    : hardwareDialectRequirementKeyword
      qualifiedName
      hardwareDialectRequirementBody?
      SEMICOLON
    ;

hardwareDialectRequirementKeyword
    : identifier
    ;

hardwareDialectRequirementBody
    : LBRACE
      hardwareDialectRequirementMember*
      RBRACE
    ;

hardwareDialectRequirementMember
    : hardwareDialectAttribute
    | hardwareDialectValueDeclaration
    ;


/*
 * ============================================================================
 * 15. CONSTRAINT
 * ============================================================================
 *
 * Constraints express restrictions on valid realization.
 *
 * They do not themselves perform target selection.
 */

hardwareDialectConstraintDeclaration
    : hardwareDialectConstraintKeyword
      qualifiedName
      hardwareDialectConstraintBody?
      SEMICOLON
    ;

hardwareDialectConstraintKeyword
    : identifier
    ;

hardwareDialectConstraintBody
    : LBRACE
      hardwareDialectConstraintMember*
      RBRACE
    ;

hardwareDialectConstraintMember
    : hardwareDialectPredicate
      SEMICOLON
    | hardwareDialectAttribute
    ;


/*
 * ============================================================================
 * 16. PREFERENCE
 * ============================================================================
 *
 * Preferences are explicitly weaker than requirements.
 *
 * A compiler may satisfy or reject them according to the compilation policy
 * without changing source semantics.
 */

hardwareDialectPreferenceDeclaration
    : hardwareDialectPreferenceKeyword
      qualifiedName
      hardwareDialectPreferenceBody?
      SEMICOLON
    ;

hardwareDialectPreferenceKeyword
    : identifier
    ;

hardwareDialectPreferenceBody
    : LBRACE
      hardwareDialectPreferenceMember*
      RBRACE
    ;

hardwareDialectPreferenceMember
    : hardwareDialectPredicate
      SEMICOLON
    | hardwareDialectAttribute
    ;


/*
 * ============================================================================
 * 17. RESOURCE DECLARATION
 * ============================================================================
 *
 * A resource declaration names a logical resource class.
 *
 * It does NOT allocate resources.
 *
 * It does NOT specify a fixed quantity unless that quantity is explicitly
 * part of the source-level semantic contract.
 */

hardwareDialectResourceDeclaration
    : hardwareDialectResourceKeyword
      qualifiedName
      hardwareDialectResourceBody?
      SEMICOLON
    ;

hardwareDialectResourceKeyword
    : identifier
    ;

hardwareDialectResourceBody
    : LBRACE
      hardwareDialectResourceMember*
      RBRACE
    ;

hardwareDialectResourceMember
    : hardwareDialectValueDeclaration
    | hardwareDialectPredicate SEMICOLON
    | hardwareDialectAttribute
    ;


/*
 * ============================================================================
 * 18. TARGET DECLARATION
 * ============================================================================
 *
 * A target declaration represents a logical target contract.
 *
 * It MUST NOT be interpreted as a physical device selection by the parser.
 *
 * A target may be:
 *
 *     abstract
 *     imported
 *     generated
 *     supplied externally
 *     resolved later
 *
 * Target realization belongs to compilation and hardware layers.
 */

hardwareDialectTargetDeclaration
    : hardwareDialectTargetKeyword
      qualifiedName
      hardwareDialectTargetBody?
      SEMICOLON
    ;

hardwareDialectTargetKeyword
    : identifier
    ;

hardwareDialectTargetBody
    : LBRACE
      hardwareDialectTargetMember*
      RBRACE
    ;

hardwareDialectTargetMember
    : hardwareDialectCapability
    | hardwareDialectRequirementDeclaration
    | hardwareDialectConstraintDeclaration
    | hardwareDialectPreferenceDeclaration
    | hardwareDialectResourceDeclaration
    | hardwareDialectAttribute
    ;


/*
 * ============================================================================
 * 19. IMPLEMENTATION
 * ============================================================================
 *
 * An implementation declaration describes a semantic implementation contract.
 *
 * It does not expose physical addresses or hidden backend state.
 */

hardwareDialectImplementation
    : hardwareDialectImplementationKeyword
      qualifiedName
      hardwareDialectImplementationBody?
      SEMICOLON
    ;

hardwareDialectImplementationKeyword
    : identifier
    ;

hardwareDialectImplementationBody
    : LBRACE
      hardwareDialectImplementationMember*
      RBRACE
    ;

hardwareDialectImplementationMember
    : hardwareDialectCapability
    | hardwareDialectRequirementDeclaration
    | hardwareDialectConstraintDeclaration
    | hardwareDialectResourceDeclaration
    | hardwareDialectAttribute
    ;


/*
 * ============================================================================
 * 20. EXTENSION
 * ============================================================================
 *
 * Extensions allow future HDL concepts without modifying the core grammar.
 *
 * This is essential for POCO-REAF and long-term language evolution.
 */

hardwareDialectExtension
    : hardwareDialectExtensionKeyword
      qualifiedName
      hardwareDialectExtensionParameters?
      hardwareDialectExtensionBody?
      SEMICOLON
    ;

hardwareDialectExtensionKeyword
    : identifier
    ;

hardwareDialectExtensionParameters
    : LPAREN
      hardwareDialectParameterList?
      RPAREN
    ;

hardwareDialectParameterList
    : hardwareDialectParameter
      (COMMA hardwareDialectParameter)*
      COMMA?
    ;

hardwareDialectParameter
    : identifier
      (COLON qualifiedName)?
      (ASSIGN hardwareDialectValue)?
    ;

hardwareDialectExtensionBody
    : LBRACE
      hardwareDialectExtensionMember*
      RBRACE
    ;

hardwareDialectExtensionMember
    : hardwareDialectFeature
    | hardwareDialectCapability
    | hardwareDialectRequirementDeclaration
    | hardwareDialectConstraintDeclaration
    | hardwareDialectPreferenceDeclaration
    | hardwareDialectResourceDeclaration
    | hardwareDialectTargetDeclaration
    | hardwareDialectImplementation
    | hardwareDialectAttribute
    ;


/*
 * ============================================================================
 * 21. COMPATIBILITY
 * ============================================================================
 *
 * Compatibility is represented structurally.
 *
 * Semantic analysis owns:
 *
 *     - version compatibility;
 *     - feature compatibility;
 *     - migration;
 *     - deprecation;
 *     - conflict resolution.
 */

hardwareDialectCompatibility
    : hardwareDialectCompatibilityKeyword
      hardwareDialectCompatibilityExpression
      SEMICOLON
    ;

hardwareDialectCompatibilityKeyword
    : identifier
    ;

hardwareDialectCompatibilityExpression
    : hardwareDialectReference
    | hardwareDialectCompatibilitySet
    ;

hardwareDialectCompatibilitySet
    : LBRACE
      hardwareDialectCompatibilityItem*
      RBRACE
    ;

hardwareDialectCompatibilityItem
    : hardwareDialectReference
      COMMA?
    ;


/*
 * ============================================================================
 * 22. NAMESPACE
 * ============================================================================
 *
 * Namespace hierarchy is unbounded at grammar level.
 *
 * It is a source-level naming construct, not a hardware hierarchy.
 */

hardwareDialectNamespace
    : hardwareDialectNamespaceKeyword
      qualifiedName
      LBRACE
      hardwareDialectMember*
      RBRACE
    ;

hardwareDialectNamespaceKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 23. DIALECT REFERENCE
 * ============================================================================
 *
 * A dialect reference is always a logical name.
 *
 * It is never inherently:
 *
 *     a filesystem path
 *     a URL
 *     a memory address
 *     a physical device ID
 *     a board ID
 *     a CPU ID
 *     a GPU ID
 *     an FPGA ID
 *     an ASIC ID
 */

hardwareDialectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 24. VALUE DECLARATIONS
 * ============================================================================
 *
 * Values are structural at grammar level.
 *
 * Type checking and semantic interpretation belong to the frontend.
 */

hardwareDialectValueDeclaration
    : hardwareDialectValueKeyword
      identifier
      (COLON qualifiedName)?
      (ASSIGN hardwareDialectValue)?
      SEMICOLON
    ;

hardwareDialectValueKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 25. PREDICATES
 * ============================================================================
 *
 * The grammar provides structural predicate expressions.
 *
 * Semantic analysis determines:
 *
 *     - types;
 *     - operators;
 *     - capability meaning;
 *     - resource meaning;
 *     - validity.
 */

hardwareDialectPredicate
    : hardwareDialectPredicateAtom
    | LPAREN hardwareDialectPredicate RPAREN
    ;

hardwareDialectPredicateAtom
    : hardwareDialectValue
      hardwareDialectComparisonOperator?
      hardwareDialectValue?
    ;

hardwareDialectComparisonOperator
    : ASSIGN
    | LT
    | GT
    ;


/*
 * ============================================================================
 * 26. VALUES
 * ============================================================================
 *
 * Values deliberately reuse canonical lexical literals.
 *
 * No width-specific literal grammar exists here.
 */

hardwareDialectValue
    : identifier
    | qualifiedName
    | STRING
    | INTEGER
    | FLOAT
    | TRUE
    | FALSE
    ;


/*
 * ============================================================================
 * 27. ATTRIBUTES
 * ============================================================================
 *
 * Attribute semantics belong to the canonical attribute subsystem.
 *
 * This grammar only recognizes their structural form.
 */

hardwareDialectAttribute
    : AT identifier
      hardwareDialectAttributeArguments?
    ;

hardwareDialectAttributeArguments
    : LPAREN
      hardwareDialectAttributeArgumentList?
      RPAREN
    ;

hardwareDialectAttributeArgumentList
    : hardwareDialectAttributeArgument
      (COMMA hardwareDialectAttributeArgument)*
      COMMA?
    ;

hardwareDialectAttributeArgument
    : identifier ASSIGN hardwareDialectValue
    | hardwareDialectValue
    ;


/*
 * ============================================================================
 * 28. SPECIALIZATION / PARAMETERS
 * ============================================================================
 *
 * Generic specialization is source-level elaboration.
 *
 * It does not represent a physical resource count unless the semantic
 * contract explicitly makes that value part of the program's meaning.
 */

hardwareDialectSpecialization
    : LT hardwareDialectSpecializationArgumentList GT
    ;

hardwareDialectSpecializationArgumentList
    : hardwareDialectSpecializationArgument
      (COMMA hardwareDialectSpecializationArgument)*
      COMMA?
    ;

hardwareDialectSpecializationArgument
    : identifier ASSIGN hardwareDialectValue
    | hardwareDialectValue
    ;


/*
 * ============================================================================
 * 29. OPEN-WORLD DIALECT DECLARATIONS
 * ============================================================================
 *
 * A dialect may contain nested declarations through extension and namespace
 * constructs.
 *
 * The grammar intentionally does not enumerate future hardware abstractions.
 *
 * Consequently new classes such as:
 *
 *     reconfigurable fabric
 *     neuromorphic accelerator
 *     optical processor
 *     photonic interconnect
 *     cryogenic controller
 *     future accelerator
 *
 * can be represented through qualified names and dialect extensions without
 * changing this grammar.
 */


/*
 * ============================================================================
 * 30. INTEGRATION CONTRACT
 * ============================================================================
 *
 * OWNED BY THIS FILE:
 *
 *     hardware dialect syntax
 *
 * CONSUMED FROM OTHER GRAMMARS:
 *
 *     identifier
 *     qualifiedName
 *     canonical literals
 *
 * OWNED BY CANONICAL LEXER:
 *
 *     HARDWARE
 *     DIALECT
 *     IMPORT
 *     USE
 *     AS
 *     EXTENDS
 *     IMPLEMENTS
 *     REQUIRES
 *     ENSURES
 *     WHERE
 *     punctuation
 *     identifiers
 *     literals
 *
 * DOWNSTREAM:
 *
 *     frontend AST
 *     semantic analysis
 *     dialect registry
 *     capability analysis
 *     resource analysis
 *     compilation
 *
 * HARDWARE LAYER:
 *
 *     hardware dialect information may eventually be resolved against
 *     hardware capabilities, but this grammar performs no such resolution.
 *
 * HDL:
 *
 *     HardwareDialects is complementary to HardwareModules.
 *
 * HardwareModules owns:
 *
 *     HDL module composition.
 *
 * HardwareDialects owns:
 *
 *     HDL/hardware language-extension contracts.
 *
 * They MUST NOT duplicate module semantics.
 *
 * QUANTUM:
 *
 * Hardware dialects may contain constructs that eventually lower to quantum
 * semantics, but quantum::ir remains the canonical quantum semantic boundary.
 *
 * QEC:
 *
 *     no direct dependency.
 *
 * ZQN:
 *
 *     no direct dependency.
 *
 * SCHEDULING:
 *
 *     no direct dependency.
 *
 * ROUTING:
 *
 *     no direct dependency.
 *
 * OPTIMIZATION:
 *
 *     no direct dependency.
 *
 * RESILIENCE:
 *
 *     no direct dependency.
 *
 * RUNTIME:
 *
 *     no direct dependency.
 */


/*
 * ============================================================================
 * 31. OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * This file MUST NOT be changed later merely because:
 *
 *     - a new CPU appears;
 *     - a new GPU appears;
 *     - a new FPGA family appears;
 *     - a new ASIC process appears;
 *     - a new QPU appears;
 *     - a new vendor appears;
 *     - a new topology appears;
 *     - a machine gets more resources;
 *     - a future hardware technology is introduced.
 *
 * Such changes belong to:
 *
 *     dialect registry
 *     capability registry
 *     semantic analysis
 *     target description
 *     hardware HAL
 *     compiler lowering
 *
 * The grammar changes only when the LANGUAGE syntax itself changes.
 */


/*
 * ============================================================================
 * 32. SCALABILITY INVARIANTS
 * ============================================================================
 *
 * There are intentionally no grammar constants such as:
 *
 *     MAX_DIALECTS
 *     MAX_FEATURES
 *     MAX_CAPABILITIES
 *     MAX_RESOURCES
 *     MAX_TARGETS
 *     MAX_IMPLEMENTATIONS
 *     MAX_NESTING
 *     MAX_HARDWARE_TYPES
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_QUANTA
 *
 * Repetition is represented structurally through:
 *
 *     *
 *     +
 *
 * and recursively composable rules.
 *
 * Resource availability is therefore external to syntax.
 */


/*
 * ============================================================================
 * 33. DETERMINISM
 * ============================================================================
 *
 * No semantic predicates or parser actions are used.
 *
 * The same source and same canonical lexer vocabulary must therefore produce
 * the same parse structure.
 *
 * Semantic nondeterminism, target discovery, scheduling decisions and
 * resource selection MUST NOT be implemented here.
 */


/*
 * ============================================================================
 * 34. SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     dynamic loading
 *     hardware access
 *     runtime execution
 *
 * Any imported dialect is resolved by the appropriate secure module/package
 * subsystem after parsing.
 */


/*
 * ============================================================================
 * 35. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] canonical lexer vocabulary is available;
 * [ ] Names import resolves identifier/qualifiedName;
 * [ ] canonical parser can import this grammar;
 * [ ] no duplicate hardware-dialect root exists elsewhere;
 * [ ] dialect names remain open-world;
 * [ ] no physical device is selected by parsing;
 * [ ] no resource maximum is encoded;
 * [ ] no topology is encoded;
 * [ ] no vendor enumeration is encoded;
 * [ ] no quantum::ir types are referenced;
 * [ ] no QEC types are referenced;
 * [ ] no ZQN types are referenced;
 * [ ] no scheduling types are referenced;
 * [ ] no routing types are referenced;
 * [ ] no runtime types are referenced;
 * [ ] semantic analysis owns validation;
 * [ ] capability resolution owns availability;
 * [ ] target lowering owns physical realization;
 * [ ] Rust generation remains compatible with Rust 1.97/1.97.1;
 * [ ] generated implementation uses safe Rust only;
 * [ ] positive parser tests exist;
 * [ ] negative parser tests exist;
 * [ ] boundary tests exist;
 * [ ] future/unknown dialect names parse without grammar modification;
 * [ ] large dialect compositions do not encounter artificial grammar limits;
 * [ ] cross-domain integration tests pass.
 */