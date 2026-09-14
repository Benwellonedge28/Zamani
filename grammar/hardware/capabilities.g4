/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/capabilities.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     No embedded target-language actions.
 *     No semantic execution.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL HARDWARE CAPABILITY DECLARATIONS.
 *
 * A capability describes something an implementation MAY provide and that
 * source code, compilation, resource resolution, or deployment may require,
 * prefer, inspect, or reason about.
 *
 * Examples of capability concepts include:
 *
 *     compute
 *     parallel_execution
 *     vector_execution
 *     tensor_execution
 *     quantum_execution
 *     measurement
 *     dynamic_circuit
 *     hardware_acceleration
 *     memory_coherence
 *     high_bandwidth_memory
 *     programmable_logic
 *     deterministic_execution
 *     floating_point
 *     cryptographic_acceleration
 *     networking
 *
 * Capability names are intentionally open.
 *
 * The grammar MUST NOT enumerate every possible CPU, GPU, FPGA, ASIC,
 * quantum processor, accelerator, future processor, vendor extension,
 * architecture, instruction set, or implementation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - abstract hardware capability declarations;
 *     - capability references;
 *     - capability predicates;
 *     - capability properties;
 *     - capability requirements;
 *     - capability exclusions;
 *     - capability preferences;
 *     - capability guarantees;
 *     - capability dependencies;
 *     - capability relationships;
 *     - capability composition;
 *     - capability inheritance/extension intent;
 *     - capability profiles;
 *     - capability sets;
 *     - capability availability intent;
 *     - capability portability intent;
 *     - capability version intent;
 *     - capability parameters;
 *     - capability metadata;
 *     - capability evidence references;
 *     - abstract capability matching intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - literal definitions;
 *     - general expressions;
 *     - general type definitions;
 *     - hardware module syntax;
 *     - ports;
 *     - wires;
 *     - signals;
 *     - clocks;
 *     - physical device enumeration;
 *     - physical device IDs;
 *     - physical addresses;
 *     - PCI identifiers;
 *     - vendor hardware discovery;
 *     - runtime probing;
 *     - calibration;
 *     - topology discovery;
 *     - routing;
 *     - placement algorithms;
 *     - scheduling;
 *     - optimization;
 *     - resource allocation;
 *     - resource quantities;
 *     - hardware drivers;
 *     - runtime dispatch;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - simulation.
 *
 * ============================================================================
 * CAPABILITY VS RESOURCE
 * ============================================================================
 *
 * CAPABILITY
 *     Describes what an implementation can do or support.
 *
 * RESOURCE
 *     Describes an amount, instance, availability, or consumable/allocatable
 *     computational resource.
 *
 * REQUIREMENT
 *     States what must be true for an implementation to be valid.
 *
 * CONSTRAINT
 *     Restricts legal implementation choices.
 *
 * PREFERENCE
 *     Gives optimization guidance without changing semantic validity.
 *
 * HINT
 *     Gives advisory information which may be ignored.
 *
 * TARGET
 *     Identifies an abstract destination or target class.
 *
 * CAPABILITY IS NOT RESOURCE.
 *
 * For example:
 *
 *     capability quantum.dynamic_circuit
 *
 * says that dynamic-circuit execution is supported.
 *
 * It does NOT mean:
 *
 *     use device X;
 *     allocate N qubits;
 *     reserve a particular processor;
 *     use a particular topology.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Capability declarations describe semantic compatibility requirements rather
 * than temporary physical implementation details.
 *
 * A program can therefore express:
 *
 *     requires capability quantum.measurement;
 *
 * without encoding:
 *
 *     a particular quantum processor;
 *     a fixed number of qubits;
 *     a physical qubit identifier;
 *     a vendor;
 *     a topology;
 *     a device address.
 *
 * Capability resolution happens after parsing.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO fixed machine limits.
 *
 * It does not define:
 *
 *     MAX_CAPABILITIES
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *
 * Capability sets use arbitrary repetition:
 *
 *     *
 *     +
 *
 * Capability values and parameters use expressions.
 *
 * Consequently, the grammar places no artificial upper bound on the number
 * of capabilities, capability members, capability properties, or capability
 * relationships represented by source code.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * lexer/tokens.g4
 *   |
 *   v
 * hardware/capabilities.g4
 *   |
 *   v
 * syntax AST
 *   |
 *   v
 * semantic capability model
 *   |
 *   +------------------------------+
 *   |                              |
 *   v                              v
 * program capability intent    target capability model
 *   |                              |
 *   +---------------+--------------+
 *                   |
 *                   v
 *             capability matching
 *                   |
 *                   v
 *          resource/target resolution
 *                   |
 *          +--------+---------+
 *          |        |         |
 *          v        v         v
 *       routing  scheduling  lowering
 *          |        |         |
 *          +--------+---------+
 *                   |
 *                   v
 *              hardware HAL
 *                   |
 *                   v
 *                runtime
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum capability syntax may describe capabilities such as:
 *
 *     quantum
 *     quantum.measurement
 *     quantum.dynamic_circuit
 *     quantum.mid_circuit_measurement
 *     quantum.reset
 *     quantum.error_correction
 *
 * However:
 *
 *     grammar -> quantum::ir
 *
 * is performed by semantic lowering.
 *
 * This grammar does NOT define quantum operations or create a second quantum
 * intermediate representation.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * QEC remains responsible for error correction.
 *
 * ZQN remains responsible for noise/fault semantics.
 *
 * Hardware capability declarations merely state capability intent.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * hardware.g4 owns hardware structure.
 *
 * devices.g4 owns logical device declarations.
 *
 * resources.g4 owns resource quantities and resource intent.
 *
 * topology.g4 owns abstract topology descriptions.
 *
 * placement.g4 owns placement intent.
 *
 * targets.g4 owns target declarations.
 *
 * This file owns the capability layer shared by those concepts.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Required upstream:
 *
 *     grammar/lexer/tokens.g4
 *
 * Expected stable lexical vocabulary includes capability-oriented keywords
 * such as:
 *
 *     capability
 *     requires
 *     provides
 *     supports
 *     excludes
 *     prefers
 *     guarantees
 *     profile
 *     capability-set
 *
 * Exact keyword tokens are part of the lexical compatibility contract.
 *
 * Common punctuation and literals are also supplied by ZamaniTokens.
 *
 * This parser grammar must never define lexer rules itself.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareCapabilitiesParser;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * The grammar exposes both declarations and standalone capability clauses.
 *
 * This allows hardware declarations, device declarations, targets, and other
 * parser components to consume capability syntax without redefining it.
 * ========================================================================== */

hardwareCapabilityDeclaration
    : hardwareCapabilityModifiers*
      K_CAPABILITY
      hardwareCapabilityName
      hardwareCapabilityParameters?
      hardwareCapabilitySpecification?
      SEMICOLON?
    ;

hardwareCapabilityClause
    : hardwareCapabilityRequirement
    | hardwareCapabilityProvision
    | hardwareCapabilitySupport
    | hardwareCapabilityExclusion
    | hardwareCapabilityPreference
    | hardwareCapabilityGuarantee
    | hardwareCapabilityProfileReference
    | hardwareCapabilitySetReference
    ;


/* ============================================================================
 * 2. MODIFIERS
 * ========================================================================== */

hardwareCapabilityModifiers
    : hardwareCapabilityVisibility
    | hardwareCapabilityAttribute
    ;

hardwareCapabilityVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_INTERNAL
    ;

hardwareCapabilityAttribute
    : AT IDENTIFIER
      (
          LPAREN hardwareCapabilityArgumentList? RPAREN
      )?
    ;


/* ============================================================================
 * 3. CAPABILITY NAME
 *
 * Capability names are open-ended qualified names.
 *
 * This deliberately avoids a closed grammar such as:
 *
 *     cpuCapability
 *     gpuCapability
 *     fpgaCapability
 *     quantumCapability
 *
 * New hardware classes therefore do not require a parser rewrite.
 * ========================================================================== */

hardwareCapabilityName
    : hardwareCapabilityQualifiedName
    ;

hardwareCapabilityQualifiedName
    : IDENTIFIER
      (
          DOT IDENTIFIER
      )*
    ;


/* ============================================================================
 * 4. CAPABILITY PARAMETERS
 *
 * Parameters allow capability descriptions to remain generic.
 *
 * Example:
 *
 *     capability vector.execution<width>;
 *
 * The grammar does not define what "width" means.
 *
 * Semantic analysis interprets the parameter according to the capability
 * schema.
 * ========================================================================== */

hardwareCapabilityParameters
    : LESS
      hardwareCapabilityParameterList?
      GREATER
    ;

hardwareCapabilityParameterList
    : hardwareCapabilityParameter
      (
          COMMA hardwareCapabilityParameter
      )*
    ;

hardwareCapabilityParameter
    : IDENTIFIER
      (
          COLON hardwareCapabilityTypeReference
      )?
      (
          ASSIGN hardwareCapabilityExpression
      )?
    ;

hardwareCapabilityTypeReference
    : hardwareCapabilityQualifiedName
    ;


/* ============================================================================
 * 5. CAPABILITY SPECIFICATION
 * ========================================================================== */

hardwareCapabilitySpecification
    : LBRACE
      hardwareCapabilityBodyElement*
      RBRACE
    ;

hardwareCapabilityBodyElement
    : hardwareCapabilityAttributes*
      hardwareCapabilityMember
    ;

hardwareCapabilityAttributes
    : hardwareCapabilityAttribute
    ;


/* ============================================================================
 * 6. CAPABILITY MEMBERS
 * ========================================================================== */

hardwareCapabilityMember
    : hardwareCapabilityClause
    | hardwareCapabilityProperty
    | hardwareCapabilityParameterDeclaration
    | hardwareCapabilityDependency
    | hardwareCapabilityComposition
    | hardwareCapabilityExtension
    | hardwareCapabilityMetadata
    ;


/* ============================================================================
 * 7. REQUIREMENT
 *
 * A requirement is semantically mandatory.
 *
 * Example:
 *
 *     requires quantum.measurement;
 *
 * A capability requirement does not select a physical device.
 * ========================================================================== */

hardwareCapabilityRequirement
    : K_REQUIRES
      hardwareCapabilityReference
      SEMICOLON
    ;

hardwareCapabilityReference
    : hardwareCapabilityQualifiedName
      hardwareCapabilityArgumentList?
    ;


/* ============================================================================
 * 8. PROVISION
 *
 * "provides" describes a capability exposed by a logical hardware
 * declaration or implementation contract.
 * ========================================================================== */

hardwareCapabilityProvision
    : K_PROVIDES
      hardwareCapabilityReferenceList
      SEMICOLON
    ;

hardwareCapabilityReferenceList
    : hardwareCapabilityReference
      (
          COMMA hardwareCapabilityReference
      )*
    ;


/* ============================================================================
 * 9. SUPPORT
 *
 * "supports" is intentionally distinct from "provides".
 *
 * Semantic analysis determines whether support means:
 *
 *     native support;
 *     emulated support;
 *     delegated support;
 *     optional support;
 *     conditional support.
 *
 * The grammar does not make that determination.
 * ========================================================================== */

hardwareCapabilitySupport
    : K_SUPPORTS
      hardwareCapabilityReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 10. EXCLUSION
 *
 * Exclusion states that a capability must not be assumed or selected as part
 * of a particular capability contract.
 * ========================================================================== */

hardwareCapabilityExclusion
    : K_EXCLUDES
      hardwareCapabilityReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 11. PREFERENCE
 *
 * Preferences are advisory.
 *
 * A preference MUST NOT be interpreted as a semantic requirement merely
 * because it appears in source.
 * ========================================================================== */

hardwareCapabilityPreference
    : K_PREFER
      hardwareCapabilityReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 12. GUARANTEE
 *
 * A guarantee is stronger than ordinary support.
 *
 * The semantic layer determines whether a guarantee is statically verifiable,
 * target-dependent, deployment-dependent, or runtime-verified.
 * ========================================================================== */

hardwareCapabilityGuarantee
    : K_GUARANTEES
      hardwareCapabilityReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 13. PROFILE
 *
 * Profiles group capabilities under a symbolic name.
 *
 * Example:
 *
 *     profile quantum.dynamic;
 *
 * Profiles are not vendor names and do not identify physical hardware.
 * ========================================================================== */

hardwareCapabilityProfileReference
    : K_PROFILE
      hardwareCapabilityQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 14. CAPABILITY SET
 *
 * Capability sets permit arbitrary collections of capabilities.
 * ========================================================================== */

hardwareCapabilitySetReference
    : K_CAPABILITY_SET
      hardwareCapabilityQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 15. DEPENDENCIES
 *
 * A capability may depend upon other abstract capabilities.
 *
 * This represents a semantic relationship, not a runtime dependency graph.
 * ========================================================================== */

hardwareCapabilityDependency
    : K_DEPENDS
      K_ON
      hardwareCapabilityReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 16. COMPOSITION
 *
 * A capability can be described as a composition of other capabilities.
 *
 * This is particularly useful for heterogeneous hardware.
 *
 * Example:
 *
 *     compose accelerator =
 *         compute
 *         + memory.access
 *         + parallel.execution;
 *
 * The exact semantic composition rules are outside ANTLR.
 * ========================================================================== */

hardwareCapabilityComposition
    : K_COMPOSE
      (
          ASSIGN
      )?
      hardwareCapabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 17. EXTENSION
 *
 * Capability extension permits domain-specific capability families without
 * changing the universal grammar.
 * ========================================================================== */

hardwareCapabilityExtension
    : K_EXTENDS
      hardwareCapabilityReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 18. CAPABILITY PARAMETER DECLARATION
 * ========================================================================== */

hardwareCapabilityParameterDeclaration
    : K_PARAMETER
      IDENTIFIER
      (
          COLON
          hardwareCapabilityTypeReference
      )?
      (
          ASSIGN
          hardwareCapabilityExpression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 19. METADATA
 *
 * Metadata is intentionally extensible.
 *
 * Standard semantic metadata can be recognized by semantic analysis without
 * forcing every future metadata key into the lexer.
 * ========================================================================== */

hardwareCapabilityMetadata
    : K_METADATA
      LBRACE
      hardwareCapabilityMetadataEntry*
      RBRACE
    ;

hardwareCapabilityMetadataEntry
    : IDENTIFIER
      ASSIGN
      hardwareCapabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 20. PROPERTY
 *
 * Generic property syntax is the forward-compatibility mechanism.
 *
 * Examples:
 *
 *     native = true;
 *     optional = false;
 *     experimental = true;
 *     version = "1";
 *
 * The grammar does not decide which properties are legal for a capability.
 * ========================================================================== */

hardwareCapabilityProperty
    : IDENTIFIER
      ASSIGN
      hardwareCapabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 21. ARGUMENTS
 * ========================================================================== */

hardwareCapabilityArgumentList
    : hardwareCapabilityArgument
      (
          COMMA hardwareCapabilityArgument
      )*
    ;

hardwareCapabilityArgument
    : hardwareCapabilityNamedArgument
    | hardwareCapabilityExpression
    ;

hardwareCapabilityNamedArgument
    : IDENTIFIER
      ASSIGN
      hardwareCapabilityExpression
    ;


/* ============================================================================
 * 22. EXPRESSIONS
 *
 * IMPORTANT:
 *
 * These expressions are capability expressions, not a second general-purpose
 * expression language.
 *
 * They are deliberately small and structural enough for capability
 * declarations while remaining extensible.
 *
 * If the canonical expression grammar exposes a shared expression rule, the
 * frontend integration layer should map this non-terminal to that canonical
 * rule rather than creating a competing semantic expression representation.
 * ========================================================================== */

hardwareCapabilityExpression
    : hardwareCapabilityConditionalExpression
    ;

hardwareCapabilityConditionalExpression
    : hardwareCapabilityLogicalExpression
      (
          QUESTION
          hardwareCapabilityExpression
          COLON
          hardwareCapabilityExpression
      )?
    ;

hardwareCapabilityLogicalExpression
    : hardwareCapabilityComparisonExpression
      (
          (
              AND
            | OR
          )
          hardwareCapabilityComparisonExpression
      )*
    ;

hardwareCapabilityComparisonExpression
    : hardwareCapabilityAdditiveExpression
      (
          (
              EQUAL
            | NOT_EQUAL
            | LESS_EQUAL
            | GREATER_EQUAL
            | LESS
            | GREATER
          )
          hardwareCapabilityAdditiveExpression
      )?
    ;

hardwareCapabilityAdditiveExpression
    : hardwareCapabilityMultiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          hardwareCapabilityMultiplicativeExpression
      )*
    ;

hardwareCapabilityMultiplicativeExpression
    : hardwareCapabilityUnaryExpression
      (
          (
              STAR
            | SLASH
            | PERCENT
          )
          hardwareCapabilityUnaryExpression
      )*
    ;

hardwareCapabilityUnaryExpression
    : (
          PLUS
        | MINUS
        | NOT
      )*
      hardwareCapabilityPrimaryExpression
    ;

hardwareCapabilityPrimaryExpression
    : hardwareCapabilityLiteral
    | hardwareCapabilityReference
    | hardwareCapabilityCall
    | hardwareCapabilityIndex
    | hardwareCapabilityMemberAccess
    | LPAREN
      hardwareCapabilityExpression
      RPAREN
    ;

hardwareCapabilityCall
    : hardwareCapabilityQualifiedName
      LPAREN
      hardwareCapabilityArgumentList?
      RPAREN
    ;

hardwareCapabilityIndex
    : hardwareCapabilityQualifiedName
      LBRACKET
      hardwareCapabilityExpression
      RBRACKET
    ;

hardwareCapabilityMemberAccess
    : hardwareCapabilityQualifiedName
    ;


/* ============================================================================
 * 23. LITERALS
 *
 * Literal recognition belongs to ZamaniTokens.
 * ========================================================================== */

hardwareCapabilityLiteral
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHARACTER_LITERAL
    | TRUE
    | FALSE
    | NULL
    ;


/* ============================================================================
 * 24. BOOLEAN / SET EXPRESSIONS
 *
 * These aliases make semantic processing easier without introducing
 * machine-specific semantics.
 * ========================================================================== */

hardwareCapabilityPredicate
    : hardwareCapabilityExpression
    ;

hardwareCapabilityPredicateList
    : hardwareCapabilityPredicate
      (
          COMMA
          hardwareCapabilityPredicate
      )*
    ;


/* ============================================================================
 * 25. CAPABILITY DECLARATION BLOCKS
 *
 * Explicit named sets are useful for reusable capability contracts.
 * ========================================================================== */

hardwareCapabilityProfileDeclaration
    : hardwareCapabilityModifiers*
      K_PROFILE
      hardwareCapabilityQualifiedName
      LBRACE
      hardwareCapabilityProfileMember*
      RBRACE
      SEMICOLON?
    ;

hardwareCapabilityProfileMember
    : hardwareCapabilityAttributes*
      (
          hardwareCapabilityClause
        | hardwareCapabilityProperty
        | hardwareCapabilityDependency
        | hardwareCapabilityExtension
      )
    ;


/* ============================================================================
 * 26. CAPABILITY SET DECLARATIONS
 * ========================================================================== */

hardwareCapabilitySetDeclaration
    : hardwareCapabilityModifiers*
      K_CAPABILITY_SET
      hardwareCapabilityQualifiedName
      LBRACE
      hardwareCapabilitySetMember*
      RBRACE
      SEMICOLON?
    ;

hardwareCapabilitySetMember
    : hardwareCapabilityAttributes*
      (
          hardwareCapabilityReference
        | hardwareCapabilityClause
        | hardwareCapabilityProperty
      )
      SEMICOLON?
    ;


/* ============================================================================
 * 27. ABSTRACT MATCHING INTENT
 *
 * Matching is declarative.
 *
 * It does NOT perform discovery or allocation.
 *
 * The semantic/resource layer evaluates these expressions against a capability
 * model supplied by the compilation target or runtime environment.
 * ========================================================================== */

hardwareCapabilityMatchExpression
    : K_MATCH
      hardwareCapabilityReference
      (
          K_AGAINST
          hardwareCapabilityReference
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 28. AVAILABILITY INTENT
 *
 * Availability is an observation of an environment.
 *
 * The grammar records the intent to describe availability but does not query
 * hardware.
 * ========================================================================== */

hardwareCapabilityAvailability
    : K_AVAILABILITY
      hardwareCapabilityReference
      (
          ASSIGN
          hardwareCapabilityExpression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 29. VERSION INTENT
 *
 * Capability versions are symbolic/version expressions.
 *
 * The grammar does not impose a particular versioning scheme.
 * ========================================================================== */

hardwareCapabilityVersion
    : K_VERSION
      ASSIGN
      hardwareCapabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 30. EVIDENCE
 *
 * Evidence may identify where a capability claim originates.
 *
 * Examples:
 *
 *     specification
 *     compiler
 *     target
 *     runtime
 *     provider
 *
 * The grammar does not trust evidence merely because it is syntactically
 * present. Trust and verification belong to semantic/security layers.
 * ========================================================================== */

hardwareCapabilityEvidence
    : K_EVIDENCE
      ASSIGN
      hardwareCapabilityExpression
      SEMICOLON
    ;


/* ============================================================================
 * 31. CAPABILITY CONTRACT
 *
 * This is the reusable high-level contract consumed by devices, hardware
 * targets, accelerators, and compilation contexts.
 * ========================================================================== */

hardwareCapabilityContract
    : LBRACE
      hardwareCapabilityContractMember*
      RBRACE
    ;

hardwareCapabilityContractMember
    : hardwareCapabilityRequirement
    | hardwareCapabilityProvision
    | hardwareCapabilitySupport
    | hardwareCapabilityExclusion
    | hardwareCapabilityPreference
    | hardwareCapabilityGuarantee
    | hardwareCapabilityDependency
    | hardwareCapabilityComposition
    | hardwareCapabilityExtension
    | hardwareCapabilityProfileReference
    | hardwareCapabilitySetReference
    | hardwareCapabilityProperty
    | hardwareCapabilityVersion
    | hardwareCapabilityAvailability
    | hardwareCapabilityEvidence
    ;


/* ============================================================================
 * 32. CAPABILITY COLLECTION
 *
 * Arbitrary capability collections are supported.
 * ========================================================================== */

hardwareCapabilityCollection
    : LBRACKET
      hardwareCapabilityReferenceList?
      RBRACKET
    ;


/* ============================================================================
 * 33. QUALIFIED CAPABILITY PROPERTY PATH
 *
 * Used by semantic tooling when capability properties are hierarchical.
 *
 * Example:
 *
 *     quantum.measurement.readout
 *
 * remains symbolic.
 * ========================================================================== */

hardwareCapabilityPropertyPath
    : IDENTIFIER
      (
          DOT IDENTIFIER
      )*
    ;


/* ============================================================================
 * 34. CAPABILITY ATTRIBUTE ARGUMENTS
 * ========================================================================== */

hardwareCapabilityAttributeArguments
    : hardwareCapabilityArgument
      (
          COMMA
          hardwareCapabilityArgument
      )*
    ;


/* ============================================================================
 * 35. INTEGRATION CONTRACT
 * ============================================================================
 *
 * ANTLR
 * -----
 *
 * This file is a parser grammar and consumes ZamaniTokens.
 *
 * Lexer
 * -----
 *
 * All lexical recognition belongs to:
 *
 *     grammar/lexer/tokens.g4
 *
 * No lexer rules are duplicated here.
 *
 * AST
 * ---
 *
 * The parser produces syntax nodes which the frontend maps to the canonical
 * Zamani AST.
 *
 * The AST should distinguish at least:
 *
 *     CapabilityDeclaration
 *     CapabilityReference
 *     CapabilityRequirement
 *     CapabilityProvision
 *     CapabilitySupport
 *     CapabilityExclusion
 *     CapabilityPreference
 *     CapabilityGuarantee
 *     CapabilityDependency
 *     CapabilityComposition
 *     CapabilityProfile
 *     CapabilitySet
 *     CapabilityProperty
 *     CapabilityPredicate
 *
 * SEMANTIC ANALYSIS
 * -----------------
 *
 * Semantic analysis validates:
 *
 *     - capability existence;
 *     - capability namespace;
 *     - capability parameter types;
 *     - capability relationships;
 *     - requirement satisfiability;
 *     - conflicting requirements;
 *     - invalid exclusions;
 *     - profile composition;
 *     - version compatibility;
 *     - dialect ownership;
 *     - target applicability.
 *
 * This grammar does not perform those checks.
 *
 * RESOURCE INTEGRATION
 * --------------------
 *
 * resources.g4 owns resource quantities and resource requirements.
 *
 * Capability resolution MAY produce resource requirements, but that
 * transformation belongs to semantic analysis.
 *
 * HARDWARE INTEGRATION
 * --------------------
 *
 * hardware.g4 may consume:
 *
 *     hardwareCapabilityContract
 *
 * to describe what a hardware implementation provides or requires.
 *
 * DEVICES INTEGRATION
 * -------------------
 *
 * devices.g4 may reference capability declarations but must not redefine
 * capability syntax.
 *
 * TARGET INTEGRATION
 * ------------------
 *
 * targets.g4 may use capability contracts to express target compatibility.
 *
 * TOPOLOGY INTEGRATION
 * --------------------
 *
 * topology.g4 may require capabilities relevant to links, communication,
 * connectivity, or topology-dependent operations.
 *
 * PLACEMENT INTEGRATION
 * ---------------------
 *
 * placement.g4 consumes resolved capability information.
 *
 * It does not depend on this grammar for placement algorithms.
 *
 * SCHEDULING
 * ----------
 *
 * Scheduling consumes capability information after semantic resolution.
 *
 * This grammar must never schedule an operation.
 *
 * ROUTING
 * -------
 *
 * Routing may use resolved capabilities to determine legal implementations.
 *
 * This grammar never performs routing.
 *
 * OPTIMIZATION
 * ------------
 *
 * Optimization may use capability information to select legal implementations.
 *
 * This grammar never optimizes.
 *
 * HARDWARE HAL
 * ------------
 *
 * The HAL supplies implementation capability information.
 *
 * The HAL MUST NOT depend on parser syntax to discover hardware.
 *
 * The direction is:
 *
 *     source capability intent
 *             |
 *             v
 *     semantic capability model
 *             |
 *             v
 *     HAL capability model
 *
 * not:
 *
 *     parser -> hardware discovery
 *
 * QUANTUM
 * -------
 *
 * Quantum capability declarations may influence compilation feasibility.
 *
 * They do not define quantum operations.
 *
 * Quantum operations lower through the canonical quantum::ir boundary.
 *
 * QEC
 * ---
 *
 * QEC consumes resolved capability information where necessary.
 *
 * This grammar does not define QEC algorithms.
 *
 * ZQN
 * ---
 *
 * ZQN provides noise/fault semantics.
 *
 * Capability syntax may state that an implementation supports a particular
 * fault-tolerance or mitigation capability, but ZQN remains the owner of
 * noise/fault semantics.
 *
 * RUNTIME
 * -------
 *
 * Runtime capability state is supplied by the runtime environment.
 *
 * Parsing must remain independent of runtime state.
 *
 * DISTRIBUTED
 * -----------
 *
 * Distributed execution may consume capability contracts for nodes, links,
 * accelerators, memory domains, communication facilities, or execution
 * environments.
 *
 * No fixed node count is permitted.
 *
 * AI
 * --
 *
 * AI/ML accelerators may expose symbolic capabilities such as:
 *
 *     tensor.execution
 *     matrix.acceleration
 *     mixed_precision
 *     accelerator.execution
 *
 * The grammar must not enumerate a fixed accelerator catalogue.
 *
 * HDL
 * ---
 *
 * HDL constructs may require or provide hardware capabilities.
 *
 * Capability declarations do not define HDL structural connectivity.
 *
 * INTEROPERABILITY
 * ----------------
 *
 * Foreign backends may map external capability descriptions into the
 * semantic capability model.
 *
 * They must not modify the meaning of source syntax.
 *
 * TOOLING
 * -------
 *
 * IDEs, formatters, linters, documentation generators, and capability
 * inspectors may consume the parser AST.
 *
 * Tools must not infer physical hardware solely from capability syntax.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar must be deterministic for the same token stream and
 * grammar version.
 *
 * No runtime state, network access, hardware probing, clock, randomness, or
 * environment-dependent operation is permitted during parsing.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Capability declarations are declarative.
 *
 * They must not:
 *
 *     execute commands;
 *     access files;
 *     access networks;
 *     probe devices;
 *     allocate hardware;
 *     modify runtime state.
 *
 * Any external capability evidence must be validated by trusted semantic or
 * runtime components.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Capability names are namespaced and extensible.
 *
 * Standard capability names require language-version compatibility policy.
 *
 * Vendor/provider/experimental capability names should use qualified
 * namespaces rather than modifying universal grammar productions.
 *
 * Removing or changing a standardized capability keyword is a language
 * compatibility event.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     fixed CPU count;
 *     fixed GPU count;
 *     fixed FPGA count;
 *     fixed ASIC count;
 *     fixed qubit count;
 *     fixed memory capacity;
 *     fixed topology;
 *     fixed device identifier;
 *     fixed hardware address;
 *     fixed vendor;
 *     fixed architecture;
 *     fixed accelerator count;
 *     fixed capability count.
 *
 * Capability collections use arbitrary repetition.
 *
 * Quantities, dimensions, versions, and predicates use symbolic expressions.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] ZamaniTokens provides every referenced token.
 * [ ] No lexer rules are duplicated here.
 * [ ] Capability syntax has one authoritative owner.
 * [ ] devices.g4 consumes this capability contract rather than redefining it.
 * [ ] hardware.g4 consumes this capability contract rather than redefining it.
 * [ ] resources.g4 remains the owner of resource quantities.
 * [ ] targets.g4 remains the owner of target syntax.
 * [ ] topology.g4 remains the owner of topology syntax.
 * [ ] placement.g4 remains the owner of placement syntax.
 * [ ] semantic analysis owns capability validation.
 * [ ] HAL owns runtime/physical capability discovery.
 * [ ] quantum::ir remains the canonical quantum semantic boundary.
 * [ ] no QEC implementation is embedded here.
 * [ ] no ZQN implementation is embedded here.
 * [ ] no routing/scheduling/optimization logic is embedded here.
 * [ ] no physical hardware identifiers are required.
 * [ ] no fixed machine/resource limits exist.
 * [ ] positive parser tests exist.
 * [ ] negative parser tests exist.
 * [ ] boundary/scalability tests exist.
 * [ ] cross-domain tests exist.
 * [ ] deterministic parsing tests exist.
 * [ ] compatibility tests cover capability vocabulary evolution.
 * [ ] generated Rust integration succeeds on Rust 1.97/1.97.1.
 * [ ] generated code is accepted under the repository no-unsafe policy.
 *
 * ============================================================================
 */