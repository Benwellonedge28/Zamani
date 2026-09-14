/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/resources.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe generated-code integration
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL HARDWARE RESOURCE INTENT.
 *
 * It describes resources that a hardware/software program:
 *
 *     - requires;
 *     - constrains;
 *     - prefers;
 *     - hints about;
 *     - exposes as capabilities;
 *     - targets abstractly;
 *     - groups;
 *     - derives;
 *     - associates with hardware components.
 *
 * It does NOT enumerate or select physical machines.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - hardware resource declarations;
 *   - hardware resource references;
 *   - hardware resource quantities;
 *   - hardware resource requirements;
 *   - hardware resource constraints;
 *   - hardware resource preferences;
 *   - hardware resource hints;
 *   - hardware resource capabilities;
 *   - hardware resource targets;
 *   - hardware resource properties;
 *   - hardware resource groups;
 *   - hardware resource relationships;
 *   - hardware resource expressions;
 *   - resource portability intent;
 *   - resource scalability intent;
 *   - resource performance intent;
 *   - resource latency intent;
 *   - resource energy intent;
 *   - resource reliability intent;
 *   - resource availability intent;
 *   - abstract resource reservation/acquisition/release intent.
 *
 * THIS FILE DOES NOT OWN
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - literals;
 *   - general expressions;
 *   - general types;
 *   - hardware modules;
 *   - ports;
 *   - wires;
 *   - signals;
 *   - clocks;
 *   - physical device discovery;
 *   - physical device identifiers;
 *   - physical addresses;
 *   - topology discovery;
 *   - routing;
 *   - scheduling;
 *   - placement algorithms;
 *   - optimization;
 *   - calibration;
 *   - runtime resource allocation;
 *   - backend selection;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - simulation.
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
 * hardware/resources.g4
 *   |
 *   v
 * frontend AST
 *   |
 *   v
 * semantic analysis
 *   |
 *   +-----------------------------+
 *   |                             |
 *   v                             v
 * hardware resource intent    canonical IR / semantic model
 *   |                             |
 *   +-------------+---------------+
 *                 |
 *                 v
 *          resource resolution
 *                 |
 *                 v
 *       hardware capability model
 *                 |
 *                 v
 *       routing / scheduling /
 *       optimization / lowering
 *                 |
 *                 v
 *          hardware HAL
 *                 |
 *                 v
 *             runtime
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A resource expression is an abstract statement of program intent.
 *
 * For example:
 *
 *     requires resource memory >= workload_size;
 *
 * does NOT mean:
 *
 *     allocate 16 GB;
 *     use machine X;
 *     use address Y;
 *     use device Z.
 *
 * The actual implementation is resolved from:
 *
 *     program semantics
 *     compilation context
 *     target capabilities
 *     available resources
 *     deployment configuration
 *     runtime state
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No fixed machine-size limits occur in this grammar.
 *
 * This grammar contains no:
 *
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *
 * Quantities are expressions.
 *
 * Repetition is represented by ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * rather than fixed-size sequences.
 *
 * Therefore resource requirements can scale according to the program,
 * compilation context, and available machine resources.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * RESOURCE
 *     An abstract computational/hardware resource.
 *
 * REQUIREMENT
 *     A condition necessary for semantic feasibility.
 *
 * CONSTRAINT
 *     A condition implementation must respect.
 *
 * PREFERENCE
 *     An optimization preference which may be traded off.
 *
 * HINT
 *     Advisory information which may be ignored.
 *
 * CAPABILITY
 *     A property supplied by an implementation/environment.
 *
 * TARGET
 *     An abstract compilation/execution destination.
 *
 * AVAILABILITY
 *     A runtime/compilation-context observation.
 *
 * CAPACITY
 *     An implementation-provided quantity.
 *
 * RESERVATION
 *     Abstract intent to obtain resources.
 *
 * None of these concepts may be silently collapsed into another.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareResourcesParser;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC HARDWARE RESOURCE ENTRY POINT
 * ============================================================================
 *
 * A hardware resource declaration introduces an abstract resource contract.
 *
 * Example:
 *
 *     resource compute {
 *         quantity = workload_size;
 *     }
 *
 * The identifier is symbolic.
 *
 * It is NOT:
 *
 *     a device ID;
 *     a PCI address;
 *     a physical core number;
 *     a physical qubit number;
 *     a memory address.
 *
 * ============================================================================
 */

hardwareResourceDeclaration
    : hardwareResourceAttributes*
      K_RESOURCE
      IDENTIFIER
      hardwareResourceType?
      hardwareResourceSpecification?
      SEMICOLON?
    ;


/* ============================================================================
 * 2. RESOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attributes remain syntactic metadata.
 *
 * Their interpretation belongs to semantic analysis.
 *
 * ============================================================================
 */

hardwareResourceAttributes
    : AT IDENTIFIER
      (
          LPAREN hardwareResourceAttributeArguments? RPAREN
      )?
    ;

hardwareResourceAttributeArguments
    : hardwareResourceAttributeArgument
      (
          COMMA hardwareResourceAttributeArgument
      )*
    ;

hardwareResourceAttributeArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | hardwareResourceQualifiedName
    | hardwareResourceExpression
    ;


/* ============================================================================
 * 3. RESOURCE TYPE
 * ============================================================================
 *
 * Resource types are open through qualified names.
 *
 * This deliberately avoids a finite grammar such as:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     MEMORY
 *
 * because future resource kinds must not require a grammar rewrite.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     accelerator
 *     quantum.logical_qubit
 *     vendor.extension.resource
 *
 * ============================================================================
 */

hardwareResourceType
    : hardwareResourceQualifiedName
    ;


/* ============================================================================
 * 4. RESOURCE SPECIFICATION
 * ============================================================================
 */

hardwareResourceSpecification
    : LBRACE
      hardwareResourceBodyElement*
      RBRACE
    ;


/* ============================================================================
 * 5. RESOURCE BODY
 * ============================================================================
 */

hardwareResourceBodyElement
    : hardwareResourceAttributes*
      hardwareResourceClause
    ;


/* ============================================================================
 * 6. RESOURCE CLAUSES
 * ============================================================================
 */

hardwareResourceClause
    : hardwareResourceQuantityClause
    | hardwareResourceReferenceClause
    | hardwareResourceRequirementClause
    | hardwareResourceConstraintClause
    | hardwareResourcePreferenceClause
    | hardwareResourceHintClause
    | hardwareResourceCapabilityClause
    | hardwareResourceTargetClause
    | hardwareResourceCapacityClause
    | hardwareResourceAvailabilityClause
    | hardwareResourcePortabilityClause
    | hardwareResourceScalabilityClause
    | hardwareResourcePerformanceClause
    | hardwareResourceLatencyClause
    | hardwareResourceEnergyClause
    | hardwareResourceReliabilityClause
    | hardwareResourceReservationClause
    | hardwareResourceAcquisitionClause
    | hardwareResourceReleaseClause
    | hardwareResourceGroupClause
    | hardwareResourceDerivationClause
    | hardwareResourcePropertyClause
    ;


/* ============================================================================
 * 7. RESOURCE QUANTITY
 * ============================================================================
 *
 * Quantities are expressions.
 *
 * This is essential for scalability.
 *
 * Valid examples include:
 *
 *     quantity = n;
 *     quantity = problem_size;
 *     quantity = required_memory;
 *     quantity = workload * lanes;
 *     quantity = available_capacity;
 *
 * The grammar imposes no numerical upper bound.
 *
 * ============================================================================
 */

hardwareResourceQuantityClause
    : K_QUANTITY
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 8. RESOURCE REFERENCE
 * ============================================================================
 *
 * References a previously declared abstract resource.
 *
 * ============================================================================
 */

hardwareResourceReferenceClause
    : K_RESOURCE
      hardwareResourceQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 9. REQUIREMENT
 * ============================================================================
 *
 * A requirement is semantically mandatory.
 *
 * Example:
 *
 *     requires resource memory >= required_memory;
 *
 * The grammar does not determine whether the requirement is satisfiable.
 *
 * ============================================================================
 */

hardwareResourceRequirementClause
    : K_REQUIRES
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 10. CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts legal implementations.
 *
 * It is distinct from a requirement.
 *
 * ============================================================================
 */

hardwareResourceConstraintClause
    : K_CONSTRAINT
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 11. PREFERENCE
 * ============================================================================
 *
 * A preference is advisory optimization intent.
 *
 * It MUST NOT silently become a hard requirement.
 *
 * ============================================================================
 */

hardwareResourcePreferenceClause
    : K_PREFERENCE
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 12. HINT
 * ============================================================================
 *
 * A hint is advisory.
 *
 * Implementations may ignore it when necessary.
 *
 * ============================================================================
 */

hardwareResourceHintClause
    : K_HINT
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 13. CAPABILITY
 * ============================================================================
 *
 * This represents source-level capability intent/reference.
 *
 * Actual capability discovery belongs to the hardware abstraction layer.
 *
 * ============================================================================
 */

hardwareResourceCapabilityClause
    : K_CAPABILITY
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 14. TARGET
 * ============================================================================
 *
 * A target is symbolic.
 *
 * It does not require a physical machine identifier.
 *
 * Examples:
 *
 *     target = cpu;
 *     target = accelerator;
 *     target = quantum;
 *     target = heterogeneous;
 *
 * Concrete target resolution occurs downstream.
 *
 * ============================================================================
 */

hardwareResourceTargetClause
    : K_TARGET
      ASSIGN
      hardwareResourceQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 15. CAPACITY
 * ============================================================================
 *
 * Capacity is an implementation/context property.
 *
 * It is not a source-level fixed machine limit.
 *
 * ============================================================================
 */

hardwareResourceCapacityClause
    : K_CAPACITY
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 16. AVAILABILITY
 * ============================================================================
 *
 * Availability may be supplied by compilation or runtime context.
 *
 * ============================================================================
 */

hardwareResourceAvailabilityClause
    : K_AVAILABILITY
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 17. PORTABILITY
 * ============================================================================
 *
 * Portability is semantic intent.
 *
 * It does not identify a backend.
 *
 * ============================================================================
 */

hardwareResourcePortabilityClause
    : K_PORTABLE
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 18. SCALABILITY
 * ============================================================================
 *
 * The scaling relationship is represented by an expression.
 *
 * No finite scaling catalogue is embedded in the grammar.
 *
 * Examples:
 *
 *     scalability = problem_size;
 *     scalability = problem_size * lanes;
 *     scalability = symbolic_scaling;
 *
 * ============================================================================
 */

hardwareResourceScalabilityClause
    : K_SCALABILITY
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 19. PERFORMANCE
 * ============================================================================
 */

hardwareResourcePerformanceClause
    : K_PERFORMANCE
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 20. LATENCY
 * ============================================================================
 *
 * Latency is semantic intent.
 *
 * Actual timing and scheduling remain outside this grammar.
 *
 * ============================================================================
 */

hardwareResourceLatencyClause
    : K_LATENCY
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 21. ENERGY
 * ============================================================================
 */

hardwareResourceEnergyClause
    : K_ENERGY
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 22. RELIABILITY
 * ============================================================================
 */

hardwareResourceReliabilityClause
    : K_RELIABILITY
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 23. RESERVATION INTENT
 * ============================================================================
 *
 * Reservation is intent.
 *
 * The actual reservation belongs to resource management/deployment/runtime.
 *
 * ============================================================================
 */

hardwareResourceReservationClause
    : K_RESERVE
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 24. ACQUISITION INTENT
 * ============================================================================
 *
 * This does not allocate a physical resource during parsing.
 *
 * ============================================================================
 */

hardwareResourceAcquisitionClause
    : K_ACQUIRE
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 25. RELEASE INTENT
 * ============================================================================
 */

hardwareResourceReleaseClause
    : K_RELEASE
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 26. RESOURCE GROUP
 * ============================================================================
 *
 * A group contains an arbitrary number of resource clauses.
 *
 * There is no fixed group size.
 *
 * ============================================================================
 */

hardwareResourceGroupClause
    : K_RESOURCE
      K_GROUP
      IDENTIFIER
      LBRACE
      hardwareResourceBodyElement*
      RBRACE
    ;


/* ============================================================================
 * 27. RESOURCE DERIVATION
 * ============================================================================
 *
 * A resource value may be derived from program semantics.
 *
 * Example:
 *
 *     resource required_memory = workload_size * element_size;
 *
 * ============================================================================
 */

hardwareResourceDerivationClause
    : K_RESOURCE
      IDENTIFIER
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 28. RESOURCE PROPERTY
 * ============================================================================
 *
 * Extensible property syntax.
 *
 * This permits future resource attributes without requiring every new
 * semantic property to become a lexer keyword.
 *
 * Semantic validation determines whether a property is:
 *
 *     standard;
 *     dialect-defined;
 *     target-defined;
 *     experimental;
 *     invalid.
 *
 * ============================================================================
 */

hardwareResourcePropertyClause
    : IDENTIFIER
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 29. RESOURCE EXPRESSION
 * ============================================================================
 *
 * CRITICAL ARCHITECTURAL RULE:
 *
 * This grammar does not define another expression language.
 *
 * Resource expressions must eventually map to the canonical Zamani
 * expression grammar.
 *
 * This local boundary exists so resource-specific semantic analysis can
 * identify expression-bearing positions without creating a second AST
 * expression hierarchy.
 *
 * ============================================================================
 */

hardwareResourceExpression
    : expression
    ;


/* ============================================================================
 * 30. QUALIFIED RESOURCE NAME
 * ============================================================================
 *
 * Resource namespaces are open-ended.
 *
 * Examples:
 *
 *     memory
 *     compute.cpu
 *     accelerator.gpu
 *     quantum.logical_qubit
 *     vendor.domain.resource
 *
 * ============================================================================
 */

hardwareResourceQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/* ============================================================================
 * 31. RESOURCE REQUIREMENT SHORT FORM
 * ============================================================================
 *
 * Convenience form for hardware declarations that already have a resource
 * context.
 *
 * ============================================================================
 */

hardwareResourceRequirement
    : K_REQUIRES
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 32. RESOURCE CONSTRAINT SHORT FORM
 * ============================================================================
 */

hardwareResourceConstraint
    : K_CONSTRAINT
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 33. RESOURCE PREFERENCE SHORT FORM
 * ============================================================================
 */

hardwareResourcePreference
    : K_PREFERENCE
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 34. RESOURCE HINT SHORT FORM
 * ============================================================================
 */

hardwareResourceHint
    : K_HINT
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 35. RESOURCE CAPABILITY SHORT FORM
 * ============================================================================
 */

hardwareResourceCapability
    : K_CAPABILITY
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 36. RESOURCE TARGET SHORT FORM
 * ============================================================================
 */

hardwareResourceTarget
    : K_TARGET
      ASSIGN
      hardwareResourceQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 37. RESOURCE CONTRACT
 * ============================================================================
 *
 * A contract groups resource intent.
 *
 * ============================================================================
 */

hardwareResourceContract
    : K_RESOURCE
      IDENTIFIER
      LBRACE
      hardwareResourceBodyElement*
      RBRACE
    ;


/* ============================================================================
 * 38. RESOURCE SET
 * ============================================================================
 *
 * Allows multiple independent resource declarations.
 *
 * ============================================================================
 */

hardwareResourceSet
    : K_RESOURCES
      LBRACE
      hardwareResourceSetElement*
      RBRACE
    ;

hardwareResourceSetElement
    : hardwareResourceAttributes*
      hardwareResourceDeclaration
    | hardwareResourceRequirement
    | hardwareResourceConstraint
    | hardwareResourcePreference
    | hardwareResourceHint
    | hardwareResourceCapability
    | hardwareResourceTarget
    ;


/* ============================================================================
 * 39. RESOURCE ASSOCIATION
 * ============================================================================
 *
 * Associates a resource with an abstract hardware component.
 *
 * This is semantic association, not physical placement.
 *
 * Example:
 *
 *     resource memory associated_with accelerator;
 *
 * ============================================================================
 */

hardwareResourceAssociation
    : K_RESOURCE
      hardwareResourceQualifiedName
      K_ASSOCIATED
      K_WITH
      hardwareResourceQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 40. RESOURCE DEPENDENCY
 * ============================================================================
 *
 * Expresses a semantic dependency between resource contracts.
 *
 * It does not perform scheduling.
 *
 * ============================================================================
 */

hardwareResourceDependency
    : K_RESOURCE
      hardwareResourceQualifiedName
      K_DEPENDS
      K_ON
      hardwareResourceQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 41. RESOURCE PROVISION
 * ============================================================================
 *
 * Expresses that an abstract hardware declaration provides a resource.
 *
 * ============================================================================
 */

hardwareResourceProvision
    : K_PROVIDES
      K_RESOURCE
      hardwareResourceQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 42. RESOURCE CONSUMPTION
 * ============================================================================
 *
 * Expresses source-level consumption intent.
 *
 * Actual allocation is downstream.
 *
 * ============================================================================
 */

hardwareResourceConsumption
    : K_USES
      K_RESOURCE
      hardwareResourceQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 43. RESOURCE SCALING CONTRACT
 * ============================================================================
 *
 * Represents how resource demand relates to a symbolic problem parameter.
 *
 * Example:
 *
 *     resource scaling {
 *         demand = problem_size * lanes;
 *     }
 *
 * No fixed number of scaling dimensions is imposed.
 *
 * ============================================================================
 */

hardwareResourceScalingClause
    : K_SCALING
      LBRACE
      hardwareResourceScalingItem*
      RBRACE
    ;

hardwareResourceScalingItem
    : IDENTIFIER
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 44. RESOURCE PORTABILITY CONTRACT
 * ============================================================================
 */

hardwareResourcePortabilityContract
    : K_PORTABILITY
      LBRACE
      hardwareResourcePortabilityItem*
      RBRACE
    ;

hardwareResourcePortabilityItem
    : IDENTIFIER
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 45. RESOURCE PERFORMANCE CONTRACT
 * ============================================================================
 */

hardwareResourcePerformanceContract
    : K_PERFORMANCE
      LBRACE
      hardwareResourcePerformanceItem*
      RBRACE
    ;

hardwareResourcePerformanceItem
    : IDENTIFIER
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 46. RESOURCE CONSTRAINT BLOCK
 * ============================================================================
 */

hardwareResourceConstraintBlock
    : K_CONSTRAINT
      LBRACE
      hardwareResourceConstraintItem*
      RBRACE
    ;

hardwareResourceConstraintItem
    : hardwareResourceExpression
      SEMICOLON
    | IDENTIFIER
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 47. RESOURCE REQUIREMENT BLOCK
 * ============================================================================
 */

hardwareResourceRequirementBlock
    : K_REQUIRES
      LBRACE
      hardwareResourceRequirementItem*
      RBRACE
    ;

hardwareResourceRequirementItem
    : K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    | IDENTIFIER
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 48. RESOURCE PREFERENCE BLOCK
 * ============================================================================
 */

hardwareResourcePreferenceBlock
    : K_PREFERENCE
      LBRACE
      hardwareResourcePreferenceItem*
      RBRACE
    ;

hardwareResourcePreferenceItem
    : K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    | IDENTIFIER
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 49. RESOURCE CAPABILITY BLOCK
 * ============================================================================
 */

hardwareResourceCapabilityBlock
    : K_CAPABILITY
      LBRACE
      hardwareResourceCapabilityItem*
      RBRACE
    ;

hardwareResourceCapabilityItem
    : K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    | IDENTIFIER
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 50. RESOURCE OBSERVATION
 * ============================================================================
 *
 * Runtime observations are represented syntactically as values/expressions.
 *
 * This grammar does not claim that an observation is authoritative.
 *
 * ============================================================================
 */

hardwareResourceObservation
    : K_RESOURCE
      hardwareResourceQualifiedName
      K_OBSERVED
      ASSIGN
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 51. RESOURCE VALIDATION MARKER
 * ============================================================================
 *
 * This expresses source-level validation intent only.
 *
 * Actual validation belongs to semantic analysis and resource validation.
 *
 * ============================================================================
 */

hardwareResourceValidation
    : K_VALIDATE
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 52. RESOURCE FALLBACK
 * ============================================================================
 *
 * Allows an implementation to express an alternative resource strategy
 * without encoding a specific physical machine.
 *
 * ============================================================================
 */

hardwareResourceFallback
    : K_FALLBACK
      K_RESOURCE
      hardwareResourceExpression
      SEMICOLON
    ;


/* ============================================================================
 * 53. RESOURCE EXTENSION
 * ============================================================================
 *
 * Future dialects can attach additional resource semantics through the
 * existing dialect/extension infrastructure.
 *
 * ============================================================================
 */

hardwareResourceExtension
    : IDENTIFIER
      DOUBLE_COLON
      IDENTIFIER
      hardwareResourceExpression?
      SEMICOLON
    ;


/* ============================================================================
 * 54. RESOURCE NAME / SYMBOLIC REFERENCE
 * ============================================================================
 */

hardwareResourceName
    : IDENTIFIER
    | hardwareResourceQualifiedName
    ;


/* ============================================================================
 * 55. RESOURCE VALUE
 * ============================================================================
 *
 * Resource values intentionally use the canonical expression grammar.
 *
 * ============================================================================
 */

hardwareResourceValue
    : hardwareResourceExpression
    ;


/* ============================================================================
 * 56. RESOURCE CONTRACT ITEM
 * ============================================================================
 */

hardwareResourceContractItem
    : hardwareResourceAttributes*
      hardwareResourceClause
    | hardwareResourceRequirement
    | hardwareResourceConstraint
    | hardwareResourcePreference
    | hardwareResourceHint
    | hardwareResourceCapability
    | hardwareResourceTarget
    ;


/* ============================================================================
 * 57. INTEGRATION CONTRACT
 * ============================================================================
 *
 * LEXER
 * -----
 *
 * This parser consumes ZamaniTokens.
 *
 * Required lexical vocabulary includes the resource vocabulary used by this
 * file, including the resource/requirement/capability/constraint/preference
 * family and their associated punctuation.
 *
 * The lexer owns spelling and tokenization.
 *
 *
 * CORE GRAMMAR
 * ------------
 *
 * `expression`, identifier semantics, general attributes, and shared source
 * constructs must come from the canonical grammar architecture.
 *
 * This file must not create a competing expression AST.
 *
 *
 * HARDWARE GRAMMAR
 * ----------------
 *
 * `grammar/hardware/hardware.g4` owns hardware modules, ports, signals,
 * wires, clocks, timing, processes, memories, pipelines, and hardware
 * composition.
 *
 * This file supplies reusable hardware-resource intent to that grammar.
 *
 *
 * UNIVERSAL RESOURCE GRAMMAR
 * --------------------------
 *
 * `grammar/resources/resources.g4` owns the language-wide resource model.
 *
 * Hardware resource syntax must map to that semantic model rather than
 * defining incompatible resource concepts.
 *
 *
 * QUANTUM
 * -------
 *
 * Quantum resource declarations may refer to concepts such as logical
 * qubits, quantum memory, measurement capacity, or other quantum resources.
 *
 * This file must NOT define quantum gates, quantum operations, circuits,
 * physical-qubit allocation, QEC, or ZQN semantics.
 *
 * Quantum computation remains represented by the canonical `quantum::ir`.
 *
 *
 * HARDWARE HAL
 * ------------
 *
 * The HAL supplies actual hardware capabilities and availability.
 *
 * This grammar never discovers hardware.
 *
 *
 * RESOURCE MANAGEMENT
 * -------------------
 *
 * Resource-management components evaluate:
 *
 *     requirement
 *     constraint
 *     preference
 *     capability
 *     availability
 *     capacity
 *
 * against actual execution resources.
 *
 *
 * COMPILER
 * --------
 *
 * Compilation may lower resource intent into:
 *
 *     target requirements;
 *     capability queries;
 *     allocation requests;
 *     scheduling constraints;
 *     routing constraints;
 *     optimization objectives.
 *
 *
 * RUNTIME
 * -------
 *
 * Runtime consumes resolved resource decisions.
 *
 * Runtime must never depend on this parser grammar directly.
 *
 *
 * SCHEDULING
 * ----------
 *
 * Latency/performance/timing information is intent.
 *
 * Scheduling owns actual ordering, timing, synchronization, and resource
 * reservation decisions.
 *
 *
 * ROUTING / PLACEMENT
 * -------------------
 *
 * Resource syntax does not perform physical placement.
 *
 * Physical realization belongs downstream.
 *
 *
 * OPTIMIZATION
 * ------------
 *
 * Preferences and performance objectives may be consumed by optimization.
 *
 * Optimization must not reinterpret preferences as mandatory requirements.
 *
 *
 * QEC / ZQN
 * ---------
 *
 * No QEC algorithm belongs here.
 *
 * No noise model belongs here.
 *
 * ZQN may provide resource-relevant fault information downstream.
 *
 *
 * AST CONTRACT
 * ------------
 *
 * The frontend AST should preserve:
 *
 *     resource kind
 *     symbolic name
 *     quantity expression
 *     requirement/constraint/preference/hint category
 *     capability intent
 *     target intent
 *     property namespace
 *     source span
 *     annotations
 *     provenance
 *
 * It must not resolve physical devices during parsing.
 *
 *
 * SEMANTIC CONTRACT
 * -----------------
 *
 * Semantic analysis must determine:
 *
 *     - whether resource names exist;
 *     - whether expressions have valid types;
 *     - whether requirements are meaningful;
 *     - whether constraints are valid;
 *     - whether preferences are legal;
 *     - whether capability references are valid;
 *     - whether target references are valid;
 *     - whether resource contracts conflict.
 *
 * The parser only establishes syntax.
 *
 *
 * SCALABILITY CONTRACT
 * --------------------
 *
 * This file MUST NOT introduce:
 *
 *     fixed resource counts;
 *     fixed hardware counts;
 *     fixed memory sizes;
 *     fixed core counts;
 *     fixed accelerator counts;
 *     fixed device identifiers;
 *     fixed topology;
 *     fixed addresses.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Intentionally absent:
 *
 *     MAX_RESOURCES
 *     MAX_MEMORY
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *
 * Quantities are expressions.
 *
 * Therefore physical scale remains a semantic/resource-resolution concern.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic for a fixed token stream and grammar version.
 *
 * Resource availability, capability discovery, scheduling, and allocation
 * are intentionally NOT parser concerns and therefore do not affect parsing.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no runtime allocation;
 *     - contains no embedded target-language actions.
 *
 * Generated Rust integration must preserve the repository-wide no-unsafe
 * policy.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Changes to keyword tokens are language-version changes.
 *
 * Changes to resource semantics are semantic-versioning/API concerns even
 * when syntax remains compatible.
 *
 * Unknown resource properties should be handled according to the dialect and
 * compatibility policy rather than silently changing standard semantics.
 *
 * ============================================================================
 */