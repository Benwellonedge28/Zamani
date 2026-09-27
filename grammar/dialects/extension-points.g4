/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/extension-points.g4
 *
 * Grammar:
 *     DialectExtensionPoints
 *
 * Status:
 *     PRODUCTION-READY EXTENSION-POINT COMPOSITION CONTRACT
 *
 * Baseline:
 *     ANTLR4
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the reusable SOURCE-LEVEL EXTENSION-POINT CONTRACT for
 * Zamani dialects.
 *
 * It does NOT define a second extension language.
 *
 * It does NOT redefine dialect registration.
 *
 * It does NOT redefine imports, exports, namespaces, versions, capabilities,
 * compatibility, vendor declarations, or experimental declarations.
 *
 * Its purpose is to provide stable parser-level attachment points that allow
 * independently maintained Zamani domains and dialects to declare where
 * their extension-defined syntax/metadata/semantic contracts attach.
 *
 * The ownership model is:
 *
 *     grammar/dialects/dialect.g4
 *             |
 *             +--> public dialect composition boundary
 *             |
 *             +--> grammar/dialects/registration.g4
 *             |        |
 *             |        +--> dialect declaration
 *             |        +--> extension declaration
 *             |
 *             +--> grammar/dialects/extension-points.g4
 *                      |
 *                      +--> stable extension attachment contracts
 *                      +--> extension target references
 *                      +--> extension-point metadata
 *                      +--> extension-point composition
 *
 * This file therefore complements registration.g4 rather than competing with
 * it.
 *
 * ============================================================================
 * FUNDAMENTAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * A dialect extension point is a PLACE where an extension may attach.
 *
 * It is NOT itself:
 *
 *     a backend
 *     a device
 *     a compiler pass
 *     an IR
 *     a runtime object
 *     a hardware resource
 *     a physical qubit
 *     a scheduler
 *     a router
 *     a QEC implementation
 *     a ZQN implementation
 *
 * The grammar records source structure only.
 *
 * Semantic analysis determines whether a declared extension point:
 *
 *     exists;
 *     is legal;
 *     is visible;
 *     is compatible;
 *     is supported;
 *     is permitted by policy;
 *     has a valid lowering;
 *     conflicts with another extension;
 *     satisfies its requirements.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST NOT redefine the concrete syntax owned by:
 *
 *     grammar/dialects/registration.g4
 *     grammar/dialects/imports.g4
 *     grammar/dialects/exports.g4
 *     grammar/dialects/namespaces.g4
 *     grammar/dialects/versioning.g4
 *     grammar/dialects/capabilities.g4
 *     grammar/dialects/compatibility.g4
 *     grammar/dialects/vendor.g4
 *     grammar/dialects/experimental.g4
 *
 * In particular, this file MUST NOT redefine:
 *
 *     dialectRegistration
 *     dialectRegistrationExtension
 *     dialectImport
 *     dialectExportDeclaration
 *     qualifiedName
 *     versionExpression
 *     capability declaration
 *     compatibility declaration
 *
 * Those remain owned by their canonical files.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Without a dedicated extension-point contract, domain grammars tend to create
 * local rules such as:
 *
 *     quantumExtensionPoint
 *     hardwareExtensionPoint
 *     aiExtensionPoint
 *     vendorExtensionPoint
 *
 * with subtly different identity, metadata, lifecycle, and compatibility
 * semantics.
 *
 * That causes grammar fragmentation.
 *
 * This file establishes one reusable structural contract:
 *
 *     extension point identity
 *     extension target
 *     extension kind
 *     attachment metadata
 *     requirements
 *     capabilities
 *     compatibility references
 *     composition references
 *     semantic/lowering descriptors
 *
 * Individual domains remain responsible for their domain-specific meaning.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Extension-point identities are symbolic.
 *
 * The grammar MUST NOT enumerate:
 *
 *     quantum
 *     classical
 *     HDL
 *     GPU
 *     FPGA
 *     QPU
 *     CPU
 *     ASIC
 *     CUDA
 *     ROCm
 *     OpenQASM
 *     QIR
 *     vendor names
 *     accelerator names
 *     future technologies
 *
 * New domains and technologies therefore do not require this grammar to be
 * modified merely because they exist.
 *
 * Examples of symbolic extension points:
 *
 *     quantum::operation
 *     quantum::measurement
 *     classical::numeric
 *     hdl::module
 *     hardware::lowering
 *     ai::tensor
 *     data::stream
 *     distributed::transport
 *     organization::domain::extension
 *     future::computing::facility
 *
 * These examples are documentation only.
 *
 * They are NOT reserved names.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Extension points MUST preserve:
 *
 *     Program Once
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Run Everywhere
 *          |
 *          v
 *     Run Anywhere
 *          |
 *          v
 *     Forever
 *
 * An extension point may describe language semantics or implementation
 * contracts, but it must not turn a target-specific implementation detail into
 * an implicit universal language limit.
 *
 * The extension-point grammar therefore MUST NOT encode:
 *
 *     MAX_EXTENSIONS
 *     MAX_EXTENSION_POINTS
 *     MAX_DIALECTS
 *     MAX_CAPABILITIES
 *     MAX_REQUIREMENTS
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * ============================================================================
 * RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * The following are intentionally NOT extension-point syntax:
 *
 *     physical device identifiers
 *     physical qubit identifiers
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     memory addresses
 *     fixed topology identifiers
 *     deployment coordinates
 *     backend handles
 *
 * If an extension requires resources, it must expose a semantic requirement
 * or capability contract.
 *
 * Examples:
 *
 *     requires quantum::measurement
 *     requires capability("tensor.compute")
 *     requires hardware::programmable_logic
 *
 * The resource system determines whether a realization satisfies that
 * requirement.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar is purely syntactic.
 *
 * It MUST NOT perform:
 *
 *     filesystem access
 *     network access
 *     plugin loading
 *     hardware discovery
 *     capability probing
 *     target selection
 *     runtime execution
 *     random selection
 *     environment inspection
 *
 * Identical token streams must produce identical syntactic treatment.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no actions
 *     no semantic predicates
 *     no callbacks
 *     no I/O
 *     no unsafe code
 *
 * Generated Rust integration remains subject to:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     safe Rust only
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every extension-point construct must remain representable in the
 * domain-neutral frontend AST.
 *
 * Conceptually:
 *
 *     DialectExtensionPoint {
 *         identity
 *         target
 *         kind
 *         attributes
 *         requirements
 *         capabilities
 *         compatibility
 *         composition
 *         descriptors
 *         source_span
 *     }
 *
 * The exact Rust AST structure remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT create:
 *
 *     QuantumExtensionNode
 *     GPUExtensionNode
 *     FPGAExtensionNode
 *     QPUExtensionNode
 *     VendorDeviceExtensionNode
 *
 * merely because an extension happens to belong to those domains.
 *
 * ============================================================================
 * CANONICAL QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum extensions use the normal pipeline:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL / target realization
 *
 * This grammar MUST NOT create another quantum IR.
 *
 * ============================================================================
 * EXTENSION POINT VS EXTENSION DECLARATION
 * ============================================================================
 *
 * These concepts are deliberately distinct.
 *
 * Extension declaration:
 *
 *     declares a language facility.
 *
 * Extension point:
 *
 *     identifies where such a facility attaches.
 *
 * Therefore:
 *
 *     registration.g4
 *         owns declaration structure
 *
 *     extension-points.g4
 *         owns reusable attachment structure
 *
 * The two must not be merged into one uncontrolled grammar.
 *
 * ============================================================================
 * INTEGRATION MODEL
 * ============================================================================
 *
 * A consuming grammar may import this grammar:
 *
 *     import DialectExtensionPoints;
 *
 * and use:
 *
 *     dialectExtensionPointReference
 *     dialectExtensionTarget
 *     dialectExtensionPointDescriptor
 *     dialectExtensionPointMetadata
 *
 * without redefining identity or extension-point structure.
 *
 * Registration remains owned by:
 *
 *     DialectRegistration
 *
 * ============================================================================
 */

parser grammar DialectExtensionPoints;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    QualifiedNames;


/*
 * ============================================================================
 * 1. CANONICAL EXTENSION-POINT REFERENCE
 * ============================================================================
 *
 * This is the primary public integration rule.
 *
 * An extension point is identified symbolically.
 *
 * Examples:
 *
 *     quantum::operation
 *     hdl::module
 *     hardware::lowering
 *     ai::tensor
 *
 * The name is not resolved here.
 */
dialectExtensionPointReference
    : qualifiedNameReference
    ;


/*
 * ============================================================================
 * 2. EXTENSION TARGET
 * ============================================================================
 *
 * An extension may target another symbolic language construct.
 *
 * The target is intentionally open-world.
 *
 * It may identify:
 *
 *     a declaration category
 *     an expression category
 *     a statement category
 *     a type category
 *     a dialect facility
 *     a domain facility
 *     a semantic facility
 *
 * It MUST NOT be interpreted as a physical target at this layer.
 */
dialectExtensionTarget
    : qualifiedNameReference
    ;


/*
 * ============================================================================
 * 3. EXTENSION TARGET LIST
 * ============================================================================
 *
 * An extension may attach to multiple symbolic targets.
 *
 * There is no artificial maximum.
 */
dialectExtensionTargetList
    : dialectExtensionTarget
      (COMMA dialectExtensionTarget)*
    ;


/*
 * ============================================================================
 * 4. EXTENSION-POINT LIST
 * ============================================================================
 *
 * A declaration may reference multiple extension points.
 *
 * There is no artificial maximum.
 */
dialectExtensionPointList
    : dialectExtensionPointReference
      (COMMA dialectExtensionPointReference)*
    ;


/*
 * ============================================================================
 * 5. EXTENSION KIND
 * ============================================================================
 *
 * The kind is deliberately symbolic rather than a closed enumeration.
 *
 * This prevents the grammar from becoming a permanent list of extension
 * categories.
 *
 * Examples of possible semantic values:
 *
 *     syntax
 *     semantic
 *     type
 *     expression
 *     statement
 *     declaration
 *     effect
 *     resource
 *     capability
 *     lowering
 *     tooling
 *
 * These are NOT reserved keywords.
 */
dialectExtensionKind
    : identifier
    ;


/*
 * ============================================================================
 * 6. EXTENSION KIND LIST
 * ============================================================================
 */
dialectExtensionKindList
    : dialectExtensionKind
      (COMMA dialectExtensionKind)*
    ;


/*
 * ============================================================================
 * 7. EXTENSION-POINT SELECTOR
 * ============================================================================
 *
 * This rule is a stable adapter for consumers that need both:
 *
 *     extension point identity
 *     target identity
 *
 * The grammar records structure only.
 */
dialectExtensionPointSelector
    : dialectExtensionPointReference
      dialectExtensionTargetClause?
    ;

dialectExtensionTargetClause
    : dialectExtensionTargetMarker
      dialectExtensionTargetList
    ;

dialectExtensionTargetMarker
    : identifier
    ;


/*
 * ============================================================================
 * 8. EXTENSION-POINT KIND CLAUSE
 * ============================================================================
 */
dialectExtensionPointKindClause
    : dialectExtensionKindMarker
      dialectExtensionKindList
    ;

dialectExtensionKindMarker
    : identifier
    ;


/*
 * ============================================================================
 * 9. EXTENSION-POINT REQUIREMENT REFERENCE
 * ============================================================================
 *
 * Requirements remain symbolic.
 *
 * This rule deliberately does not duplicate the resource/capability grammar.
 */
dialectExtensionPointRequirement
    : dialectExtensionRequirementMarker
      qualifiedNameReference
    ;

dialectExtensionRequirementMarker
    : identifier
    ;


/*
 * ============================================================================
 * 10. EXTENSION-POINT REQUIREMENT LIST
 * ============================================================================
 */
dialectExtensionPointRequirementList
    : dialectExtensionPointRequirement
      (COMMA dialectExtensionPointRequirement)*
    ;


/*
 * ============================================================================
 * 11. EXTENSION-POINT CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capability resolution is semantic.
 */
dialectExtensionPointCapability
    : dialectExtensionCapabilityMarker
      qualifiedNameReference
    ;

dialectExtensionCapabilityMarker
    : identifier
    ;


/*
 * ============================================================================
 * 12. EXTENSION-POINT CAPABILITY LIST
 * ============================================================================
 */
dialectExtensionPointCapabilityList
    : dialectExtensionPointCapability
      (COMMA dialectExtensionPointCapability)*
    ;


/*
 * ============================================================================
 * 13. EXTENSION-POINT COMPOSITION
 * ============================================================================
 *
 * An extension point may compose other extension points.
 *
 * Semantic analysis is responsible for:
 *
 *     cycle detection
 *     conflict detection
 *     compatibility
 *     ordering
 *     ownership
 */
dialectExtensionPointComposition
    : dialectExtensionCompositionMarker
      dialectExtensionPointList
    ;

dialectExtensionCompositionMarker
    : identifier
    ;


/*
 * ============================================================================
 * 14. EXTENSION-POINT COMPATIBILITY REFERENCE
 * ============================================================================
 *
 * Compatibility syntax is represented as a symbolic reference.
 *
 * Detailed compatibility semantics remain owned by:
 *
 *     grammar/dialects/compatibility.g4
 *
 * This rule deliberately does not duplicate that grammar.
 */
dialectExtensionPointCompatibility
    : dialectExtensionCompatibilityMarker
      qualifiedNameReference
    ;

dialectExtensionCompatibilityMarker
    : identifier
    ;


/*
 * ============================================================================
 * 15. EXTENSION-POINT VERSION REFERENCE
 * ============================================================================
 *
 * Version semantics remain owned by versioning infrastructure.
 *
 * A version can therefore be carried as symbolic metadata without this file
 * becoming a version solver.
 */
dialectExtensionPointVersion
    : dialectExtensionVersionMarker
      qualifiedNameReference
    ;

dialectExtensionVersionMarker
    : identifier
    ;


/*
 * ============================================================================
 * 16. EXTENSION-POINT DESCRIPTOR
 * ============================================================================
 *
 * A descriptor is deliberately open-ended.
 *
 * The extension-point mechanism must survive the addition of future
 * descriptor categories without requiring a new global keyword for every
 * new implementation concept.
 */
dialectExtensionPointDescriptor
    : dialectExtensionDescriptorName
      ASSIGN
      dialectExtensionPointValue
      SEMICOLON
    ;

dialectExtensionDescriptorName
    : identifier
    ;


/*
 * ============================================================================
 * 17. EXTENSION-POINT DESCRIPTOR LIST
 * ============================================================================
 */
dialectExtensionPointDescriptorList
    : dialectExtensionPointDescriptor*
    ;


/*
 * ============================================================================
 * 18. EXTENSION-POINT METADATA
 * ============================================================================
 *
 * Metadata is source-level information.
 *
 * It is not executable code.
 */
dialectExtensionPointMetadata
    : dialectExtensionMetadataMarker
      ASSIGN
      dialectExtensionPointValue
      SEMICOLON
    ;

dialectExtensionMetadataMarker
    : identifier
    ;


/*
 * ============================================================================
 * 19. EXTENSION-POINT METADATA LIST
 * ============================================================================
 */
dialectExtensionPointMetadataList
    : dialectExtensionPointMetadata*
    ;


/*
 * ============================================================================
 * 20. EXTENSION-POINT BODY
 * ============================================================================
 *
 * This is a reusable structural body.
 *
 * It deliberately does not accept arbitrary source statements or executable
 * expressions. Extension-point metadata must remain declarative.
 */
dialectExtensionPointBody
    : LBRACE
      dialectExtensionPointMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 21. EXTENSION-POINT MEMBER
 * ============================================================================
 *
 * Standard structural categories have stable rules.
 *
 * Future metadata can use the descriptor/property mechanisms without forcing
 * this grammar to enumerate every future computational domain.
 */
dialectExtensionPointMember
    : dialectExtensionPointMetadata
    | dialectExtensionPointDescriptor
    | dialectExtensionPointRequirement
      SEMICOLON
    | dialectExtensionPointCapability
      SEMICOLON
    | dialectExtensionPointComposition
      SEMICOLON
    | dialectExtensionPointCompatibility
      SEMICOLON
    | dialectExtensionPointVersion
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. EXTENSION-POINT CONTRACT
 * ============================================================================
 *
 * This is the principal reusable integration contract.
 *
 * It does not declare an extension.
 *
 * It describes an extension attachment contract.
 */
dialectExtensionPointContract
    : dialectExtensionPointSelector
      dialectExtensionPointKindClause?
      dialectExtensionPointBody?
    ;


/*
 * ============================================================================
 * 23. EXTENSION-POINT CONTRACT LIST
 * ============================================================================
 */
dialectExtensionPointContractList
    : dialectExtensionPointContract
      (COMMA dialectExtensionPointContract)*
    ;


/*
 * ============================================================================
 * 24. EXTENSION-POINT REFERENCE WITH OPTIONAL KIND
 * ============================================================================
 *
 * Useful for domains that need a compact attachment declaration.
 */
dialectExtensionPointReferenceWithKind
    : dialectExtensionPointReference
      dialectExtensionPointKindClause?
    ;


/*
 * ============================================================================
 * 25. EXTENSION-POINT REFERENCE WITH TARGET
 * ============================================================================
 */
dialectExtensionPointReferenceWithTarget
    : dialectExtensionPointReference
      dialectExtensionTargetClause?
    ;


/*
 * ============================================================================
 * 26. EXTENSION-POINT REFERENCE WITH METADATA
 * ============================================================================
 */
dialectExtensionPointReferenceWithMetadata
    : dialectExtensionPointReference
      dialectExtensionPointMetadataList
    ;


/*
 * ============================================================================
 * 27. EXTENSION-POINT REFERENCE WITH REQUIREMENTS
 * ============================================================================
 */
dialectExtensionPointReferenceWithRequirements
    : dialectExtensionPointReference
      dialectExtensionPointRequirementList
    ;


/*
 * ============================================================================
 * 28. EXTENSION-POINT REFERENCE WITH CAPABILITIES
 * ============================================================================
 */
dialectExtensionPointReferenceWithCapabilities
    : dialectExtensionPointReference
      dialectExtensionPointCapabilityList
    ;


/*
 * ============================================================================
 * 29. EXTENSION-POINT COMPOSITION CONTRACT
 * ============================================================================
 *
 * This explicitly separates composition from declaration.
 */
dialectExtensionPointCompositionContract
    : dialectExtensionPointReference
      dialectExtensionCompositionMarker
      dialectExtensionPointList
    ;


/*
 * ============================================================================
 * 30. EXTENSION-POINT ATTACHMENT
 * ============================================================================
 *
 * An attachment associates an extension point with an extension target.
 *
 * It does not execute or lower anything.
 */
dialectExtensionPointAttachment
    : dialectExtensionAttachmentMarker
      dialectExtensionPointReference
      dialectExtensionTargetClause?
      dialectExtensionPointKindClause?
      dialectExtensionPointBody?
    ;

dialectExtensionAttachmentMarker
    : identifier
    ;


/*
 * ============================================================================
 * 31. EXTENSION-POINT ATTACHMENT LIST
 * ============================================================================
 */
dialectExtensionPointAttachmentList
    : dialectExtensionPointAttachment*
    ;


/*
 * ============================================================================
 * 32. EXTENSION-POINT GROUP
 * ============================================================================
 *
 * Groups are structural only.
 *
 * Group membership does not imply semantic compatibility.
 */
dialectExtensionPointGroup
    : dialectExtensionGroupMarker
      identifier
      LBRACE
      dialectExtensionPointReference*
      RBRACE
    ;

dialectExtensionGroupMarker
    : identifier
    ;


/*
 * ============================================================================
 * 33. EXTENSION-POINT VALUE
 * ============================================================================
 *
 * Values are intentionally limited to declarative data.
 *
 * No executable source statements are permitted here.
 */
dialectExtensionPointValue
    : STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | TRUE
    | FALSE
    | qualifiedNameReference
    | dialectExtensionPointListValue
    | dialectExtensionPointMapValue
    ;


/*
 * ============================================================================
 * 34. EXTENSION-POINT LIST VALUE
 * ============================================================================
 */
dialectExtensionPointListValue
    : LBRACKET
      dialectExtensionPointValueList?
      RBRACKET
    ;

dialectExtensionPointValueList
    : dialectExtensionPointValue
      (COMMA dialectExtensionPointValue)*
      COMMA?
    ;


/*
 * ============================================================================
 * 35. EXTENSION-POINT MAP VALUE
 * ============================================================================
 *
 * Keys remain identifiers and values remain declarative.
 */
dialectExtensionPointMapValue
    : LBRACE
      dialectExtensionPointMapEntry*
      RBRACE
    ;

dialectExtensionPointMapEntry
    : identifier
      ASSIGN
      dialectExtensionPointValue
      COMMA?
    ;


/*
 * ============================================================================
 * 36. EXTENSION-POINT DOCUMENT
 * ============================================================================
 *
 * This isolated entry point is useful for grammar conformance tests.
 *
 * It does not represent a complete Zamani program.
 */
dialectExtensionPointDocument
    : dialectExtensionPointContract*
      EOF
    ;


/*
 * ============================================================================
 * 37. INTEGRATION ADAPTER: REGISTRATION EXTENSION
 * ============================================================================
 *
 * The actual declaration remains owned by DialectRegistration.
 *
 * This adapter exists so consumers can explicitly document that a registered
 * extension can expose extension-point contracts.
 *
 * The rule is intentionally named separately from:
 *
 *     dialectRegistrationExtension
 *
 * to prevent ownership confusion.
 */
dialectExtensionRegistrationContract
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 38. INTEGRATION ADAPTER: DOMAIN EXTENSION
 * ============================================================================
 *
 * Domain grammars may consume this stable rule without inventing a new
 * extension-point identity model.
 */
dialectDomainExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 39. INTEGRATION ADAPTER: VENDOR EXTENSION
 * ============================================================================
 *
 * Vendor syntax remains owned by vendor.g4.
 *
 * This adapter does not permit vendor code to become core syntax.
 */
dialectVendorExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 40. INTEGRATION ADAPTER: EXPERIMENTAL EXTENSION
 * ============================================================================
 *
 * Experimental status remains owned by experimental.g4.
 */
dialectExperimentalExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 41. INTEGRATION ADAPTER: QUANTUM
 * ============================================================================
 *
 * This is deliberately generic.
 *
 * It does NOT import quantum grammar and does NOT create quantum AST nodes.
 *
 * Quantum-specific meaning remains owned by grammar/quantum/ and the semantic
 * quantum subsystem.
 */
dialectQuantumExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 42. INTEGRATION ADAPTER: CLASSICAL
 * ============================================================================
 */
dialectClassicalExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 43. INTEGRATION ADAPTER: HDL
 * ============================================================================
 */
dialectHdlExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 44. INTEGRATION ADAPTER: HARDWARE
 * ============================================================================
 */
dialectHardwareExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 45. INTEGRATION ADAPTER: AI / DATA
 * ============================================================================
 */
dialectAiExtensionPoint
    : dialectExtensionPointContract
    ;

dialectDataExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 46. INTEGRATION ADAPTER: DISTRIBUTED / NETWORKING
 * ============================================================================
 */
dialectDistributedExtensionPoint
    : dialectExtensionPointContract
    ;

dialectNetworkingExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 47. INTEGRATION ADAPTER: SECURITY
 * ============================================================================
 */
dialectSecurityExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 48. INTEGRATION ADAPTER: FUTURE DOMAINS
 * ============================================================================
 *
 * There is intentionally no enumeration of future domains.
 *
 * Consumers can use dialectExtensionPointContract directly.
 */
dialectFutureExtensionPoint
    : dialectExtensionPointContract
    ;


/*
 * ============================================================================
 * 49. SEMANTIC OWNERSHIP CONTRACT
 * ============================================================================
 *
 * The semantic layer consuming this grammar is responsible for:
 *
 *     identity resolution
 *     namespace validation
 *     extension existence
 *     extension visibility
 *     lifecycle validation
 *     version compatibility
 *     capability validation
 *     requirement validation
 *     composition validation
 *     cycle detection
 *     conflict detection
 *     ownership validation
 *     syntax ownership validation
 *     semantic ownership validation
 *     lowering validation
 *     policy validation
 *     vendor isolation
 *     experimental opt-in
 *     deprecation
 *
 * None of these operations occur in this grammar.
 */


/*
 * ============================================================================
 * 50. EXTENSION CONFLICT CONTRACT
 * ============================================================================
 *
 * Semantic validation MUST reject deterministic conflicts rather than using
 * load order.
 *
 * Examples of conflicts:
 *
 *     same extension identity with incompatible versions
 *     same extension point with incompatible ownership
 *     conflicting semantic definitions
 *     incompatible requirements
 *     incompatible capability declarations
 *     ambiguous extension targets
 *     cyclic extension composition
 *
 * This grammar preserves the structure needed to detect those conflicts.
 */


/*
 * ============================================================================
 * 51. EXTENSION IDENTITY CONTRACT
 * ============================================================================
 *
 * The source-level identity remains symbolic.
 *
 * Semantic infrastructure maps the source identity to the canonical extension
 * identity used by the frontend/IR infrastructure.
 *
 * In particular, this grammar does not create or allocate:
 *
 *     ExtensionId
 *
 * values.
 *
 * The repository's canonical AST/semantic/IR identity infrastructure remains
 * authoritative.
 */


/*
 * ============================================================================
 * 52. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * No rule in this file produces a quantum IR operation.
 *
 * A quantum extension point may eventually contribute:
 *
 *     syntax
 *     operation metadata
 *     capability requirements
 *     semantic descriptors
 *
 * but the resulting quantum meaning must enter:
 *
 *     quantum::ir
 *
 * exactly once.
 *
 * There is no:
 *
 *     DialectQuantumIR
 *     ExtensionQuantumIR
 *     VendorQuantumIR
 *
 * introduced by this grammar.
 */


/*
 * ============================================================================
 * 53. CLASSICAL / HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Extension points belonging to classical, HDL, or hardware domains remain
 * source-level contracts.
 *
 * They must lower through the canonical semantic/IR path owned by the
 * corresponding domain.
 *
 * This file does not create:
 *
 *     ClassicalIR
 *     HdlIR
 *     HardwareIR
 *
 * unless such canonical representations already exist elsewhere in the
 * repository and are owned by those subsystems.
 */


/*
 * ============================================================================
 * 54. TARGET-INDEPENDENCE
 * ============================================================================
 *
 * An extension point MUST NOT itself select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     simulator
 *     node
 *     device
 *
 * It may identify a capability such as:
 *
 *     hardware::programmable_logic
 *
 * but target resolution belongs downstream.
 */


/*
 * ============================================================================
 * 55. SCALABILITY CONTRACT
 * ============================================================================
 *
 * All collections in this grammar use:
 *
 *     *
 *     +
 *
 * rather than finite alternatives or fixed-size collections.
 *
 * Therefore there is no language-level maximum for:
 *
 *     extension points
 *     targets
 *     kinds
 *     requirements
 *     capabilities
 *     descriptors
 *     metadata
 *     compositions
 *     attachments
 *     groups
 *     dialects
 *
 * Practical limits are external resource constraints.
 */


/*
 * ============================================================================
 * 56. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also contains no:
 *
 *     physical qubit number
 *     physical CPU number
 *     physical GPU number
 *     fixed bus width
 *     fixed memory capacity
 *     fixed topology
 *     fixed device count
 *
 * Numeric literals are accepted only as declarative source values where the
 * canonical lexer provides them. They are not interpreted as implementation
 * limits.
 */


/*
 * ============================================================================
 * 57. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT provide:
 *
 *     executable payloads
 *     embedded Rust
 *     shell commands
 *     filesystem operations
 *     network operations
 *     dynamic loading
 *     runtime callbacks
 *
 * An extension declaration is metadata and source structure.
 *
 * Trust, authorization, signing, provenance, and policy remain downstream
 * security concerns.
 */


/*
 * ============================================================================
 * 58. FORWARD COMPATIBILITY
 * ============================================================================
 *
 * The extension-point contract is deliberately symbolic.
 *
 * New extension kinds should normally be representable without adding a new
 * reserved keyword.
 *
 * New extension domains should normally be representable without modifying
 * this file.
 *
 * New target technologies should not require this grammar to enumerate them.
 */


/*
 * ============================================================================
 * 59. BACKWARD COMPATIBILITY
 * ============================================================================
 *
 * Existing consumers should depend on the stable public rules:
 *
 *     dialectExtensionPointReference
 *     dialectExtensionTarget
 *     dialectExtensionPointContract
 *     dialectExtensionPointAttachment
 *
 * Internal helper rules are implementation details.
 *
 * A future revision may extend internal metadata while preserving those
 * public adapter rules.
 */


/*
 * ============================================================================
 * 60. INTEGRATION WITH EXISTING REPOSITORY FILES
 * ============================================================================
 *
 * grammar/dialects/dialect.g4
 *
 *     Owns the public dialect composition boundary.
 *
 *     It should NOT duplicate the rules in this file.
 *
 *
 * grammar/dialects/dialects.g4
 *
 *     Historical/current public dialect boundary.
 *
 *     It must not become a second extension-point implementation.
 *
 *
 * grammar/dialects/registration.g4
 *
 *     Owns dialectRegistration and dialectRegistrationExtension.
 *
 *     This file supplies reusable extension-point contracts consumed by
 *     semantic/domain integration.
 *
 *
 * grammar/dialects/imports.g4
 *
 *     Owns dialect import syntax.
 *
 *     This file does not redefine imports.
 *
 *
 * grammar/dialects/exports.g4
 *
 *     Owns dialect export syntax.
 *
 *     This file does not redefine exports.
 *
 *
 * grammar/dialects/namespaces.g4
 *
 *     Owns dialect namespace syntax/structure.
 *
 *     This file consumes canonical qualified names and does not resolve them.
 *
 *
 * grammar/dialects/versioning.g4
 *
 *     Owns dialect version syntax.
 *
 *     This file does not implement version comparison.
 *
 *
 * grammar/dialects/capabilities.g4
 *
 *     Owns capability declaration syntax.
 *
 *     This file only provides symbolic extension-point capability references.
 *
 *
 * grammar/dialects/compatibility.g4
 *
 *     Owns compatibility declarations.
 *
 *     This file does not duplicate compatibility algorithms or grammar.
 *
 *
 * grammar/dialects/vendor.g4
 *
 *     Owns vendor-extension declaration syntax.
 *
 *     Vendor extension points remain structurally compatible with this file.
 *
 *
 * grammar/dialects/experimental.g4
 *
 *     Owns experimental extension syntax.
 *
 *     Experimental policy remains outside this grammar.
 *
 *
 * grammar/specification/extensibility.md
 *
 *     Normatively defines extension architecture and lifecycle.
 *
 *     This file implements the reusable parser-level extension-point portion
 *     of that architecture.
 *
 *
 * src/frontend/ast/
 *
 *     Owns canonical AST representation.
 *
 *     Extension-point syntax must lower to domain-neutral extension identity
 *     and metadata structures.
 *
 *
 * src/quantum/ir/
 *
 *     Owns the canonical quantum IR.
 *
 *     This file must never create a second quantum IR.
 *
 *
 * compiler / semantic analysis
 *
 *     Owns extension resolution, conflict checking, capability resolution,
 *     version compatibility, and lowering.
 *
 *
 * runtime / HAL
 *
 *     Owns target realization.
 *
 *     Runtime must not depend directly on this parser grammar.
 */


/*
 * ============================================================================
 * 61. DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same extension-point mechanism is usable by:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     scientific computing
 *     embedded computing
 *     accelerator computing
 *     future computing domains
 *
 * No domain receives privileged parser treatment merely because it is known
 * today.
 */


/*
 * ============================================================================
 * 62. TEST CONTRACT
 * ============================================================================
 *
 * The following test categories are required for this file.
 *
 * POSITIVE
 *
 *     symbolic extension-point reference
 *     nested qualified extension-point reference
 *     multiple targets
 *     multiple extension points
 *     multiple requirements
 *     multiple capabilities
 *     composition
 *     metadata
 *     descriptors
 *     attachments
 *     groups
 *     domain adapters
 *
 * NEGATIVE
 *
 *     missing extension identity
 *     malformed qualified name
 *     missing target
 *     missing separator
 *     malformed list
 *     malformed map
 *     malformed contract body
 *
 * BOUNDARY
 *
 *     one extension point
 *     many extension points
 *     deeply qualified symbolic names
 *     empty optional bodies
 *     repeated metadata
 *
 * SCALABILITY
 *
 *     arbitrarily large source-defined extension-point collections subject
 *     only to implementation/resource limits
 *
 *     no test may establish an artificial grammar maximum.
 *
 * DETERMINISM
 *
 *     identical token streams produce identical parse structures.
 *
 * COMPATIBILITY
 *
 *     stable public rule names remain usable by consuming grammars.
 */


/*
 * ============================================================================
 * 63. EXAMPLES
 * ============================================================================
 *
 * These examples are explanatory only.
 *
 * They do NOT reserve these identifiers.
 *
 * Example symbolic reference:
 *
 *     quantum::operation
 *
 * Example target:
 *
 *     quantum::operation
 *         target quantum::instruction
 *
 * Example contract:
 *
 *     quantum::operation
 *         kind syntax, semantic
 *         target quantum::instruction
 *         {
 *             capability = quantum::dynamic_control;
 *         }
 *
 * Example composition:
 *
 *     hybrid::operation
 *         compose classical::operation, quantum::operation;
 *
 * Exact domain semantics are owned by the corresponding domain subsystem.
 */


/*
 * ============================================================================
 * 64. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is considered complete when:
 *
 * [x] It has one clear ownership boundary.
 * [x] It does not redefine dialect registration.
 * [x] It does not redefine imports.
 * [x] It does not redefine exports.
 * [x] It does not redefine namespaces.
 * [x] It does not redefine versions.
 * [x] It does not redefine compatibility.
 * [x] It does not enumerate domains.
 * [x] It does not enumerate vendors.
 * [x] It does not enumerate targets.
 * [x] It does not encode hardware limits.
 * [x] It does not encode quantum limits.
 * [x] It does not create another quantum IR.
 * [x] It remains domain-neutral.
 * [x] It preserves symbolic extension identity.
 * [x] It supports arbitrary source-defined collection sizes.
 * [x] It contains no embedded Rust.
 * [x] It contains no unsafe code.
 * [x] It performs no I/O.
 * [x] It is deterministic at parse time.
 * [x] It provides stable public adapter rules.
 * [x] It provides isolated conformance entry points.
 * [x] It preserves AST/semantic ownership boundaries.
 * [x] It is compatible with POCO-REAF architecture.
 *
 * Final ANTLR generation, parser integration, and repository test execution
 * remain implementation validation steps and must be performed by the
 * repository's grammar-generation/conformance tooling.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */