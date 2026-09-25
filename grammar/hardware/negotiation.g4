/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/negotiation.g4
 *
 * Grammar:
 *     ZamaniHardwareNegotiationParser
 *
 * Status:
 *     CANONICAL HARDWARE-REALIZATION NEGOTIATION LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *     Action-free
 *     Predicate-free
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL HARDWARE REALIZATION NEGOTIATION INTENT.
 *
 * Negotiation describes the semantic relationship between:
 *
 *     - program requirements;
 *     - hardware capabilities;
 *     - resource availability;
 *     - hardware constraints;
 *     - target compatibility;
 *     - realization preferences;
 *     - fallback policies;
 *     - admissibility conditions;
 *     - scalable realization alternatives.
 *
 * Negotiation does NOT perform negotiation.
 *
 * It does not:
 *
 *     - discover hardware;
 *     - enumerate devices;
 *     - allocate resources;
 *     - select physical devices;
 *     - route operations;
 *     - schedule execution;
 *     - calibrate hardware;
 *     - execute programs;
 *     - query runtime hardware;
 *     - create quantum::ir;
 *     - implement QEC;
 *     - implement ZQN;
 *     - implement HAL;
 *     - synthesize HDL.
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * FUNDAMENTAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This grammar expresses:
 *
 *     WHAT realization is acceptable
 *
 * rather than:
 *
 *     WHICH physical realization must be used.
 *
 * Therefore:
 *
 *     negotiation != discovery
 *     negotiation != allocation
 *     negotiation != placement
 *     negotiation != routing
 *     negotiation != scheduling
 *     negotiation != execution
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A portable program may express:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     alternatives
 *     fallbacks
 *     resource conditions
 *     target conditions
 *     scalability conditions
 *
 * without encoding:
 *
 *     physical device IDs
 *     CPU IDs
 *     GPU IDs
 *     FPGA coordinates
 *     QPU IDs
 *     physical qubit IDs
 *     physical addresses
 *     vendor-specific allocation
 *     fixed machine topology
 *     fixed memory size
 *     fixed device count
 *     fixed node count
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO universal hardware limits.
 *
 * It MUST NOT define:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_TARGETS
 *     MAX_RESOURCES
 *
 * Quantities are expressions.
 *
 * Collections use unbounded grammar repetition.
 *
 * Actual finite limitations belong to:
 *
 *     semantic analysis
 *     resource resolution
 *     compiler resources
 *     target capabilities
 *     runtime resources
 *     deployment environment
 *     physical hardware
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hardware negotiation declarations;
 *     - negotiation clauses;
 *     - negotiation requirements;
 *     - negotiation constraints;
 *     - negotiation preferences;
 *     - negotiation hints;
 *     - capability conditions;
 *     - resource conditions;
 *     - target compatibility conditions;
 *     - alternative realization intent;
 *     - fallback realization intent;
 *     - conditional realization intent;
 *     - negotiation profiles;
 *     - negotiation contracts;
 *     - negotiation groups;
 *     - negotiation assertions;
 *     - negotiation properties.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - ordinary expressions;
 *     - types;
 *     - general constraints;
 *     - resource declarations;
 *     - target declarations;
 *     - hardware declarations;
 *     - hardware discovery;
 *     - resource discovery;
 *     - resource allocation;
 *     - physical placement;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - synthesis;
 *     - runtime negotiation;
 *     - device drivers;
 *     - quantum operations;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * EXISTING AUTHORITY BOUNDARIES
 * ============================================================================
 *
 * grammar/hardware/hardware.g4
 *     owns hardware composition and dispatch.
 *
 * grammar/hardware/hardware-constraints.g4
 *     owns hardware-specific constraint syntax.
 *
 * grammar/hardware/targets.g4
 *     owns hardware target intent.
 *
 * grammar/compile/target.g4
 *     owns compilation target intent.
 *
 * grammar/resources/
 *     owns generic resource semantics.
 *
 * grammar/core/constraints.g4
 *     owns generic constraint expression semantics.
 *
 * grammar/quantum/
 *     owns quantum source semantics.
 *
 * quantum::ir
 *     remains the canonical quantum semantic boundary.
 *
 * This file MUST NOT reproduce any of those grammars.
 *
 * ============================================================================
 * SEMANTIC DISTINCTIONS
 * ============================================================================
 *
 * REQUIREMENT
 *     A mandatory program need.
 *
 * CONSTRAINT
 *     A mandatory admissibility condition.
 *
 * CAPABILITY
 *     An ability that a realization must or may provide.
 *
 * PREFERENCE
 *     A non-mandatory optimization preference.
 *
 * HINT
 *     Advisory information.
 *
 * ALTERNATIVE
 *     Another semantically acceptable realization strategy.
 *
 * FALLBACK
 *     A realization path permitted when another path cannot satisfy the
 *     required contract.
 *
 * PROPERTY
 *     Open-ended semantic metadata.
 *
 * Negotiation MUST preserve these distinctions.
 *
 * ============================================================================
 * NEGOTIATION MODEL
 * ============================================================================
 *
 * Conceptual semantic pipeline:
 *
 *     source
 *       |
 *       v
 *     parse negotiation intent
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic normalization
 *       |
 *       +--> requirements
 *       +--> capabilities
 *       +--> resources
 *       +--> constraints
 *       +--> preferences
 *       +--> alternatives
 *       +--> fallbacks
 *       |
 *       v
 *     realization candidates
 *       |
 *       v
 *     capability/resource validation
 *       |
 *       v
 *     constraint filtering
 *       |
 *       v
 *     preference ordering
 *       |
 *       v
 *     realization selection
 *       |
 *       +--> routing
 *       +--> scheduling
 *       +--> optimization
 *       +--> QEC
 *       +--> ZQN
 *       +--> HAL
 *
 * The grammar only represents the source portion of this process.
 *
 * ============================================================================
 * DECLARATION FORM
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     negotiation Name {
 *         ...
 *     }
 *
 * The declaration is symbolic.
 *
 * It does not execute.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareNegotiationParser;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * hardware.g4 MUST delegate hardware negotiation declarations here.
 *
 * There must be exactly one owner of hardwareNegotiationDeclaration.
 * ============================================================================
 */

hardwareNegotiationDeclaration
    : hardwareNegotiationAnnotation*
      hardwareNegotiationVisibility?
      hardwareNegotiationModifier*
      NEGOTIATION
      IDENTIFIER
      hardwareNegotiationParameters?
      hardwareNegotiationExtends?
      hardwareNegotiationSpecification?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. VISIBILITY
 * ============================================================================
 */

hardwareNegotiationVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;


/*
 * ============================================================================
 * 3. MODIFIERS
 * ============================================================================
 *
 * These are source declaration properties only.
 *
 * They do not select or reserve physical hardware.
 * ============================================================================
 */

hardwareNegotiationModifier
    : K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


/*
 * ============================================================================
 * 4. ANNOTATIONS
 * ============================================================================
 *
 * Annotation names remain open.
 *
 * Vendor, research, dialect and future metadata therefore do not require
 * permanent universal keywords.
 * ============================================================================
 */

hardwareNegotiationAnnotation
    : AT
      IDENTIFIER
      (
          LPAREN
          hardwareNegotiationAnnotationArguments?
          RPAREN
      )?
    ;


hardwareNegotiationAnnotationArguments
    : hardwareNegotiationAnnotationArgument
      (
          COMMA
          hardwareNegotiationAnnotationArgument
      )*
      COMMA?
    ;


hardwareNegotiationAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | QUANTUM_LITERAL
    | hardwareNegotiationQualifiedName
    | hardwareNegotiationExpression
    ;


/*
 * ============================================================================
 * 5. GENERIC PARAMETERS
 * ============================================================================
 *
 * Negotiation contracts may be parameterized.
 *
 * Example:
 *
 *     negotiation Scalable<C> {
 *         ...
 *     }
 *
 * C is a semantic parameter, not a machine limit.
 * ============================================================================
 */

hardwareNegotiationParameters
    : LT
      hardwareNegotiationParameter
      (
          COMMA
          hardwareNegotiationParameter
      )*
      COMMA?
      GT
    ;


hardwareNegotiationParameter
    : IDENTIFIER
      (
          COLON
          hardwareNegotiationQualifiedName
      )?
      (
          ASSIGN
          hardwareNegotiationExpression
      )?
    ;


/*
 * ============================================================================
 * 6. EXTENSION
 * ============================================================================
 *
 * Negotiation contracts may compose other symbolic negotiation contracts.
 *
 * This is semantic composition, not physical inheritance.
 * ============================================================================
 */

hardwareNegotiationExtends
    : K_EXTENDS
      hardwareNegotiationQualifiedName
      (
          COMMA
          hardwareNegotiationQualifiedName
      )*
    ;


/*
 * ============================================================================
 * 7. NEGOTIATION SPECIFICATION
 * ============================================================================
 */

hardwareNegotiationSpecification
    : LBRACE
      hardwareNegotiationMember*
      RBRACE
    ;


hardwareNegotiationMember
    : hardwareNegotiationAnnotation*
      hardwareNegotiationStatement
    ;


/*
 * ============================================================================
 * 8. NEGOTIATION STATEMENTS
 * ============================================================================
 */

hardwareNegotiationStatement
    : hardwareNegotiationRequirement
    | hardwareNegotiationConstraint
    | hardwareNegotiationCapability
    | hardwareNegotiationResource
    | hardwareNegotiationPreference
    | hardwareNegotiationHint
    | hardwareNegotiationAlternative
    | hardwareNegotiationFallback
    | hardwareNegotiationWhen
    | hardwareNegotiationProfile
    | hardwareNegotiationContract
    | hardwareNegotiationGroup
    | hardwareNegotiationAssertion
    | hardwareNegotiationProperty
    ;


/*
 * ============================================================================
 * 9. REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * It must not be silently converted into a preference.
 * ============================================================================
 */

hardwareNegotiationRequirement
    : K_REQUIRES
      hardwareNegotiationCondition
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. CONSTRAINTS
 * ============================================================================
 *
 * A constraint is mandatory.
 *
 * Hardware constraint syntax remains owned by hardware-constraints.g4.
 *
 * The negotiation grammar references the semantic form rather than
 * reproducing that grammar.
 * ============================================================================
 */

hardwareNegotiationConstraint
    : K_CONSTRAINT
      hardwareNegotiationCondition
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CAPABILITY CONDITIONS
 * ============================================================================
 *
 * Capability names are symbolic.
 *
 * Example:
 *
 *     capability("quantum.measurement");
 *
 * The grammar does not enumerate capabilities.
 * ============================================================================
 */

hardwareNegotiationCapability
    : K_CAPABILITY
      hardwareNegotiationCapabilityReference
      SEMICOLON
    ;


hardwareNegotiationCapabilityReference
    : hardwareNegotiationQualifiedName
    | LPAREN
      hardwareNegotiationExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 12. RESOURCE CONDITIONS
 * ============================================================================
 *
 * Resources are referenced symbolically.
 *
 * Quantities remain expressions.
 * ============================================================================
 */

hardwareNegotiationResource
    : K_RESOURCE
      hardwareNegotiationResourceReference
      (
          hardwareNegotiationComparisonOperator
          hardwareNegotiationExpression
      )?
      SEMICOLON
    ;


hardwareNegotiationResourceReference
    : hardwareNegotiationQualifiedName
    ;


/*
 * ============================================================================
 * 13. PREFERENCES
 * ============================================================================
 *
 * Preferences cannot invalidate a realization by themselves.
 * ============================================================================
 */

hardwareNegotiationPreference
    : K_PREFER
      hardwareNegotiationCondition
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. HINTS
 * ============================================================================
 *
 * Hints are advisory.
 *
 * They must not be treated as mandatory constraints unless semantic
 * processing explicitly promotes them according to the language contract.
 * ============================================================================
 */

hardwareNegotiationHint
    : K_HINT
      hardwareNegotiationCondition
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. ALTERNATIVES
 * ============================================================================
 *
 * Alternatives express semantically acceptable realization choices.
 *
 * They do not identify physical devices.
 * ============================================================================
 */

hardwareNegotiationAlternative
    : K_ALTERNATIVE
      hardwareNegotiationAlternativeSpecification
    ;


hardwareNegotiationAlternativeSpecification
    : hardwareNegotiationAlternativeBody
    | hardwareNegotiationExpression
      SEMICOLON
    ;


hardwareNegotiationAlternativeBody
    : LBRACE
      hardwareNegotiationMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 16. FALLBACK
 * ============================================================================
 *
 * A fallback is used when a primary realization cannot satisfy the contract.
 *
 * The semantic layer decides when fallback is admissible.
 * ============================================================================
 */

hardwareNegotiationFallback
    : K_FALLBACK
      hardwareNegotiationFallbackSpecification
    ;


hardwareNegotiationFallbackSpecification
    : hardwareNegotiationAlternativeBody
    | hardwareNegotiationExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. CONDITIONAL NEGOTIATION
 * ============================================================================
 *
 * Conditional clauses express realization policy without executing code.
 *
 * Example:
 *
 *     when capability.quantum.measurement {
 *         ...
 *     }
 *
 * The condition is semantic data.
 * ============================================================================
 */

hardwareNegotiationWhen
    : K_WHEN
      hardwareNegotiationCondition
      hardwareNegotiationAlternativeBody
    ;


/*
 * ============================================================================
 * 18. PROFILES
 * ============================================================================
 *
 * Profiles provide reusable semantic negotiation policies.
 * ============================================================================
 */

hardwareNegotiationProfile
    : K_PROFILE
      IDENTIFIER
      hardwareNegotiationParameters?
      hardwareNegotiationAlternativeBody
    ;


/*
 * ============================================================================
 * 19. CONTRACTS
 * ============================================================================
 *
 * A contract groups mandatory and advisory realization semantics.
 * ============================================================================
 */

hardwareNegotiationContract
    : K_CONTRACT
      IDENTIFIER
      hardwareNegotiationParameters?
      hardwareNegotiationAlternativeBody
    ;


/*
 * ============================================================================
 * 20. GROUPS
 * ============================================================================
 *
 * Groups provide source-level organization.
 *
 * They do not create physical resource groups.
 * ============================================================================
 */

hardwareNegotiationGroup
    : K_GROUP
      IDENTIFIER
      hardwareNegotiationAlternativeBody
    ;


/*
 * ============================================================================
 * 21. ASSERTIONS
 * ============================================================================
 *
 * Assertions are validation conditions.
 *
 * They do not perform hardware discovery.
 * ============================================================================
 */

hardwareNegotiationAssertion
    : K_ASSERT
      hardwareNegotiationCondition
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * This is the principal extensibility mechanism.
 *
 * Future hardware characteristics should normally be represented as symbolic
 * properties instead of requiring permanent parser keywords.
 *
 * Examples:
 *
 *     topology.connectivity >= required_connectivity;
 *     quantum.dynamic_control == true;
 *     thermal.margin >= required_margin;
 *     reliability.availability >= required_availability;
 *     power.energy_per_operation <= budget;
 *
 * The property namespace is intentionally open.
 * ============================================================================
 */

hardwareNegotiationProperty
    : hardwareNegotiationQualifiedName
      (
          ASSIGN
        | hardwareNegotiationComparisonOperator
      )
      hardwareNegotiationExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. CONDITION
 * ============================================================================
 *
 * Conditions are intentionally open-ended.
 *
 * This permits future resource and capability properties without changing
 * this grammar every time a new computational architecture appears.
 * ============================================================================
 */

hardwareNegotiationCondition
    : hardwareNegotiationConditionOr
    ;


hardwareNegotiationConditionOr
    : hardwareNegotiationConditionAnd
      (
          K_OR
          hardwareNegotiationConditionAnd
      )*
    ;


hardwareNegotiationConditionAnd
    : hardwareNegotiationConditionNot
      (
          K_AND
          hardwareNegotiationConditionNot
      )*
    ;


hardwareNegotiationConditionNot
    : K_NOT
      hardwareNegotiationConditionNot
    | hardwareNegotiationConditionAtom
    ;


hardwareNegotiationConditionAtom
    : LPAREN
      hardwareNegotiationCondition
      RPAREN
    | hardwareNegotiationPredicate
    ;


hardwareNegotiationPredicate
    : hardwareNegotiationExpression
      (
          hardwareNegotiationComparisonOperator
          hardwareNegotiationExpression
      )?
    ;


/*
 * ============================================================================
 * 24. COMPARISON OPERATORS
 * ============================================================================
 *
 * Comparison semantics are resolved downstream.
 *
 * This grammar only represents their syntax.
 * ============================================================================
 */

hardwareNegotiationComparisonOperator
    : K_EQ
    | K_NE
    | K_LT
    | K_LE
    | K_GT
    | K_GE
    ;


/*
 * ============================================================================
 * 25. QUALIFIED NAMES
 * ============================================================================
 *
 * Negotiation must remain open to:
 *
 *     hardware.quantum.measurement
 *     capability.tensor.compute
 *     resource.memory
 *     target.accelerator
 *     topology.connectivity
 *
 * without enumerating every future domain.
 * ============================================================================
 */

hardwareNegotiationQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 26. EXPRESSIONS
 * ============================================================================
 *
 * Negotiation quantities and conditions are expressions.
 *
 * This permits:
 *
 *     input.size
 *     problem.size * element_size
 *     required_memory
 *     qubits >= problem.size
 *     bandwidth >= workload / duration
 *
 * without imposing finite machine limits.
 *
 * The implementation should bind this production to the canonical Zamani
 * expression grammar during composition rather than maintaining a second
 * expression implementation.
 *
 * ============================================================================
 */

hardwareNegotiationExpression
    : hardwareNegotiationPrimary
      (
          hardwareNegotiationExpressionOperator
          hardwareNegotiationPrimary
      )*
    ;


hardwareNegotiationExpressionOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | MODULO
    | LEFT_SHIFT
    | RIGHT_SHIFT
    | AMPERSAND
    | PIPE
    | CARET
    ;


hardwareNegotiationPrimary
    : hardwareNegotiationQualifiedName
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | QUANTUM_LITERAL
    | LPAREN
      hardwareNegotiationExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 27. INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is intentionally self-contained at the grammar-contract level.
 *
 * It has no embedded Rust and therefore introduces no unsafe code.
 *
 * Integration requirements:
 *
 *     1. ZamaniTokens remains the sole lexical vocabulary.
 *
 *     2. The canonical lexer must provide NEGOTIATION.
 *
 *     3. The canonical lexer must provide all K_* tokens referenced here.
 *
 *     4. hardware.g4 delegates hardwareNegotiationDeclaration here.
 *
 *     5. No duplicate hardwareNegotiationDeclaration is created elsewhere.
 *
 *     6. hardware-constraints.g4 remains the authority for standalone
 *        hardware constraint syntax.
 *
 *     7. targets.g4 remains the authority for hardware target declarations.
 *
 *     8. resources/ remains the authority for generic resource semantics.
 *
 *     9. compile/target.g4 remains the authority for compilation-target
 *        semantics.
 *
 *    10. Semantic analysis normalizes all negotiation constructs into the
 *        common requirement/capability/resource/constraint/preference model.
 *
 *    11. Negotiation does not create a second IR.
 *
 *    12. Quantum-related requirements eventually converge on quantum::ir
 *        through the normal quantum semantic pipeline.
 *
 *    13. Hardware realization occurs downstream.
 *
 * ============================================================================
 * 28. AST CONTRACT
 * ============================================================================
 *
 * The grammar requires the following conceptual AST information:
 *
 *     declaration name
 *     visibility
 *     modifiers
 *     annotations
 *     generic parameters
 *     extensions
 *     ordered members
 *     member kind
 *     source spans
 *     expressions
 *     conditions
 *     alternatives
 *     fallbacks
 *     profiles
 *     contracts
 *     groups
 *     properties
 *
 * A domain-neutral AST should represent negotiation as semantic intent.
 *
 * It MUST NOT contain:
 *
 *     PhysicalDeviceId
 *     PhysicalQubitId
 *     CpuId
 *     GpuId
 *     FpgaId
 *     DeviceAddress
 *
 * unless such information is explicitly part of a target-specific downstream
 * representation.
 *
 * ============================================================================
 * 29. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST preserve:
 *
 *     requirement != constraint
 *     constraint != preference
 *     preference != hint
 *     capability != resource
 *     alternative != fallback
 *     target != device
 *
 * It must also preserve symbolic expressions without silently truncating,
 * clamping or narrowing quantities.
 *
 * Unsatisfied mandatory requirements must remain observable as failures.
 *
 * ============================================================================
 * 30. IR CONTRACT
 * ============================================================================
 *
 * This grammar has no direct IR ownership.
 *
 * Negotiation semantics lower into the common semantic/resource/target model.
 *
 * Quantum constructs are lowered through the existing quantum pipeline:
 *
 *     quantum source
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization / routing / scheduling / QEC / ZQN / HAL
 *
 * No second quantum IR is permitted.
 *
 * ============================================================================
 * 31. COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may:
 *
 *     - resolve available capabilities;
 *     - resolve available resources;
 *     - evaluate requirements;
 *     - filter invalid candidates;
 *     - evaluate constraints;
 *     - rank preferences;
 *     - consider alternatives;
 *     - apply fallback policy;
 *     - construct a realization plan.
 *
 * The compiler must not reinterpret a hard requirement as a preference merely
 * because a target is unavailable.
 *
 * ============================================================================
 * 32. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime negotiation is outside this grammar.
 *
 * If runtime adaptation is supported, it must consume a semantic negotiation
 * model produced by compilation/semantic analysis.
 *
 * The runtime may adapt only within the program's declared semantic contract.
 *
 * ============================================================================
 * 33. SECURITY CONTRACT
 * ============================================================================
 *
 * Negotiation syntax must not itself grant permission to:
 *
 *     access devices;
 *     access memory;
 *     access networks;
 *     bypass authorization;
 *     access secrets;
 *     change security policy.
 *
 * Capability requirements describe required abilities.
 *
 * Authorization remains owned by the security subsystem.
 *
 * ============================================================================
 * 34. DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical:
 *
 *     source
 *     language version
 *     semantic environment
 *
 * parsing must produce the same syntax structure.
 *
 * The grammar must not depend on:
 *
 *     hardware discovery order
 *     runtime state
 *     device enumeration order
 *     network timing
 *     scheduler timing
 *     calibration state
 *
 * ============================================================================
 * 35. HARD-CODING AUDIT
 * ============================================================================
 *
 * The following are prohibited as universal semantics:
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
 * Program constants remain valid.
 *
 * For example:
 *
 *     required_memory = input.size * element_size
 *
 * is valid.
 *
 * A universal:
 *
 *     MAX_MEMORY = 64GB
 *
 * is not.
 *
 * ============================================================================
 * 36. TEST CONTRACT
 * ============================================================================
 *
 * The implementation must provide:
 *
 * POSITIVE TESTS
 *     - named negotiation;
 *     - parameterized negotiation;
 *     - requirements;
 *     - constraints;
 *     - capabilities;
 *     - resources;
 *     - preferences;
 *     - hints;
 *     - alternatives;
 *     - fallbacks;
 *     - conditions;
 *     - profiles;
 *     - contracts;
 *     - groups;
 *     - properties;
 *     - symbolic quantities;
 *     - quantum capability requirements;
 *     - classical capability requirements;
 *     - HDL/hardware capability requirements;
 *     - distributed capability requirements.
 *
 * NEGATIVE TESTS
 *     - malformed declaration;
 *     - malformed condition;
 *     - malformed comparison;
 *     - missing member delimiter;
 *     - invalid qualified name;
 *     - invalid generic parameter;
 *     - malformed alternative;
 *     - malformed fallback;
 *     - invalid property assignment.
 *
 * BOUNDARY TESTS
 *     - empty negotiation body;
 *     - deeply nested conditions;
 *     - large member collections;
 *     - large expression trees;
 *     - symbolic values;
 *     - zero where semantically valid;
 *     - arbitrarily large representable quantities.
 *
 * SCALABILITY TESTS
 *     - tiny target;
 *     - single-device realization;
 *     - heterogeneous realization;
 *     - distributed realization;
 *     - dynamically changing resource availability;
 *     - large symbolic resource requirements.
 *
 * DETERMINISM TESTS
 *     - identical source/context produces identical parse structure.
 *
 * COMPATIBILITY TESTS
 *     - existing hardware declarations remain unchanged;
 *     - target declarations retain their ownership;
 *     - resource declarations retain their ownership;
 *     - hardware constraints retain their ownership.
 *
 * ============================================================================
 * 37. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] exactly one hardware negotiation declaration owner exists;
 *     [ ] lexical ownership is defined;
 *     [ ] all referenced tokens exist in canonical vocabulary;
 *     [ ] syntax is unambiguous;
 *     [ ] requirement semantics are defined;
 *     [ ] constraint semantics are defined;
 *     [ ] capability semantics are defined;
 *     [ ] resource semantics are defined;
 *     [ ] preference semantics are defined;
 *     [ ] hint semantics are defined;
 *     [ ] alternative semantics are defined;
 *     [ ] fallback semantics are defined;
 *     [ ] target integration is defined;
 *     [ ] resource integration is defined;
 *     [ ] hardware constraint integration is defined;
 *     [ ] AST contract exists;
 *     [ ] semantic contract exists;
 *     [ ] IR contract exists;
 *     [ ] compiler integration exists;
 *     [ ] runtime boundary exists;
 *     [ ] security boundary exists;
 *     [ ] determinism contract exists;
 *     [ ] hard-coding audit exists;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] Rust 1.97/1.97.1 compatibility is maintained;
 *     [ ] no embedded Rust exists;
 *     [ ] no unsafe code is required.
 *
 * ============================================================================
 * END
 * ============================================================================
 */