/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/target.g4
 *
 * Grammar:
 *     CompileTarget
 *
 * Status:
 *     Production compilation-target intent parser grammar
 *
 * Purpose:
 *     Defines target-independent source syntax for expressing:
 *
 *       - execution target intent;
 *       - target families;
 *       - target capabilities;
 *       - target requirements;
 *       - target constraints;
 *       - target preferences;
 *       - target hints;
 *       - target profiles;
 *       - target composition;
 *       - target fallback/compatibility intent;
 *       - target-independent portability declarations.
 *
 * Architectural position:
 *
 *     Source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Parser
 *       |
 *       v
 *     Frontend AST
 *       |
 *       v
 *     Semantic Analysis
 *       |
 *       +--> target resolution
 *       +--> capability checking
 *       +--> resource checking
 *       +--> constraint solving
 *       +--> portability validation
 *       +--> compilation planning
 *       |
 *       v
 *     Canonical IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representations
 *       +--> distributed representations
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / hardware HAL
 *       |
 *       v
 *     runtime / deployment
 *
 * ============================================================================
 * FUNDAMENTAL PRINCIPLE
 * ============================================================================
 *
 * Target syntax describes WHAT kind of execution environment is acceptable.
 *
 * It does NOT describe HOW a particular physical machine must be used.
 *
 * Therefore:
 *
 *     target != device
 *     target != backend
 *     target != topology
 *     target != resource allocation
 *     target != scheduling
 *     target != routing
 *     target != hardware discovery
 *
 * A target declaration may express:
 *
 *     - semantic execution requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - portability requirements;
 *     - deployment intent.
 *
 * Concrete realization is a downstream concern.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A Zamani program MUST NOT need to be rewritten merely because it moves
 * between:
 *
 *     - embedded systems;
 *     - CPUs;
 *     - multicore CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - quantum processors;
 *     - quantum simulators;
 *     - heterogeneous accelerators;
 *     - clusters;
 *     - supercomputers;
 *     - distributed systems;
 *     - cloud environments;
 *     - future execution architectures.
 *
 * Target syntax therefore expresses semantic compatibility and intent.
 *
 * It must not encode temporary physical implementation details as permanent
 * program semantics unless the programmer explicitly makes such a property
 * part of the program's meaning.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO fixed machine limits.
 *
 * It does not define:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TARGETS
 *     MAX_CAPABILITIES
 *     MAX_CONSTRAINTS
 *     MAX_RESOURCES
 *     MAX_TOPOLOGY_SIZE
 *
 * Collections are represented recursively or through repetition.
 *
 * Actual resource availability is evaluated by:
 *
 *     resource model
 *     capability model
 *     hardware HAL
 *     compilation context
 *     scheduler
 *     runtime
 *     deployment system
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no device discovery;
 *     - no compiler-global mutable state;
 *     - no runtime execution.
 *
 * Rust integration target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Generated parser/compiler integration MUST use safe Rust only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - source-level target intent;
 *     - target declarations;
 *     - target families;
 *     - target profiles;
 *     - target requirements;
 *     - target constraints;
 *     - target preferences;
 *     - target hints;
 *     - target capability references;
 *     - target composition;
 *     - target alternatives;
 *     - target fallback intent;
 *     - portability declarations associated with targets.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - ordinary expressions;
 *     - types;
 *     - resource semantics;
 *     - hardware descriptions;
 *     - device discovery;
 *     - topology discovery;
 *     - calibration;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - runtime dispatch;
 *     - backend implementation;
 *     - deployment execution.
 *
 * ============================================================================
 * INTEGRATION OWNERSHIP
 * ============================================================================
 *
 * grammar/compile/compile.g4
 *     owns general compilation intent.
 *
 * grammar/compile/target.g4
 *     owns target intent.
 *
 * grammar/resources/
 *     owns resource requirements, capabilities, constraints and preferences.
 *
 * grammar/hardware/
 *     owns hardware-description syntax.
 *
 * grammar/execution/
 *     owns execution and deployment syntax.
 *
 * grammar/quantum/
 *     owns quantum semantics.
 *
 * quantum::ir
 *     remains the canonical quantum semantic boundary.
 *
 * Target syntax MUST NOT create another quantum representation.
 *
 * ============================================================================
 */

parser grammar CompileTarget;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. TARGET DECLARATION
 * ============================================================================
 *
 * Canonical integration entry point.
 *
 * A target declaration describes acceptable execution intent.
 *
 * Generic form:
 *
 *     target <target-expression>
 *
 * Extended form:
 *
 *     target <target-expression> {
 *         ...
 *     }
 *
 * The actual keyword token is intentionally represented by the canonical
 * lexical vocabulary.
 */
targetDeclaration
    : TARGET targetSpecification
    ;


/*
 * ============================================================================
 * 2. TARGET SPECIFICATION
 * ============================================================================
 *
 * A target specification may contain:
 *
 *     - one target;
 *     - a target set;
 *     - a target family;
 *     - a target profile;
 *     - a target expression;
 *     - a composed target.
 */
targetSpecification
    : targetExpression
    | targetBlock
    ;


/*
 * ============================================================================
 * 3. TARGET BLOCK
 * ============================================================================
 *
 * The block provides extensibility without creating a fixed target schema.
 *
 * New target properties can be introduced through semantic namespaces without
 * changing the grammar merely because a new hardware architecture appears.
 */
targetBlock
    : LBRACE targetEntry* RBRACE
    ;


targetEntry
    : targetRequirement
    | targetConstraint
    | targetPreference
    | targetHint
    | targetCapability
    | targetProfileReference
    | targetProperty
    | targetAlternative
    ;


/*
 * ============================================================================
 * 4. TARGET EXPRESSION
 * ============================================================================
 *
 * A target expression describes target intent.
 *
 * It is deliberately open-ended.
 *
 * The grammar must not contain a finite list such as:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *
 * because future architectures must not require grammar modification.
 *
 * Semantic analysis interprets the target identity.
 */
targetExpression
    : targetAtom
    | targetSet
    | targetUnion
    | targetIntersection
    | targetDifference
    | targetExpressionGroup
    ;


targetAtom
    : identifier
    | qualifiedIdentifier
    | STRING
    | targetCapabilityReference
    ;


targetExpressionGroup
    : LPAREN targetExpression RPAREN
    ;


/*
 * ============================================================================
 * 5. TARGET SET
 * ============================================================================
 *
 * A target set expresses a collection of acceptable targets.
 *
 * There is intentionally no fixed cardinality.
 */
targetSet
    : LBRACKET targetExpressionList? RBRACKET
    ;


targetExpressionList
    : targetExpression
      (COMMA targetExpression)*
    ;


/*
 * ============================================================================
 * 6. TARGET UNION
 * ============================================================================
 *
 * Represents alternative target families.
 *
 * Example semantic meaning:
 *
 *     classical | quantum
 *
 * The exact operator remains part of the canonical lexer contract.
 */
targetUnion
    : targetExpression TARGET_OR targetExpression
    ;


/*
 * ============================================================================
 * 7. TARGET INTERSECTION
 * ============================================================================
 *
 * Represents a target satisfying multiple semantic target properties.
 */
targetIntersection
    : targetExpression TARGET_AND targetExpression
    ;


/*
 * ============================================================================
 * 8. TARGET DIFFERENCE
 * ============================================================================
 *
 * Represents target exclusion at the semantic level.
 *
 * This does not mean physical resource removal.
 */
targetDifference
    : targetExpression TARGET_MINUS targetExpression
    ;


/*
 * ============================================================================
 * 9. TARGET FAMILY
 * ============================================================================
 *
 * A target family describes a semantic class of execution environments.
 *
 * Examples:
 *
 *     classical
 *     quantum
 *     hybrid
 *     hardware
 *     distributed
 *     accelerator
 *
 * The grammar intentionally does not enumerate those names.
 */
targetFamily
    : TARGET_FAMILY targetFamilyExpression
    ;


targetFamilyExpression
    : identifier
    | qualifiedIdentifier
    | STRING
    | targetExpression
    ;


/*
 * ============================================================================
 * 10. TARGET PROFILE
 * ============================================================================
 *
 * A profile is a reusable named collection of target intent.
 *
 * It is not a device configuration.
 *
 * Example conceptual form:
 *
 *     target profile portable_quantum {
 *         ...
 *     }
 */
targetProfile
    : TARGET_PROFILE identifier
      targetProfileBody
    ;


targetProfileBody
    : LBRACE targetProfileEntry* RBRACE
    ;


targetProfileEntry
    : targetRequirement
    | targetConstraint
    | targetPreference
    | targetHint
    | targetCapability
    | targetProperty
    ;


/*
 * ============================================================================
 * 11. TARGET PROFILE REFERENCE
 * ============================================================================
 *
 * References an existing semantic target profile.
 */
targetProfileReference
    : TARGET_PROFILE identifier
    ;


/*
 * ============================================================================
 * 12. TARGET REQUIREMENT
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * Examples:
 *
 *     quantum capability
 *     floating-point capability
 *     required instruction capability
 *     required memory property
 *     required precision
 *
 * A requirement MUST NOT encode a concrete physical device unless the
 * programmer explicitly chooses device identity as part of program semantics.
 */
targetRequirement
    : TARGET_REQUIRE expression SEMICOLON?
    ;


targetRequirementBlock
    : TARGET_REQUIRE LBRACE targetRequirementEntry* RBRACE
    ;


targetRequirementEntry
    : identifier COLON expression SEMICOLON
    | identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 13. TARGET CONSTRAINT
 * ============================================================================
 *
 * Constraints describe legal implementation boundaries.
 *
 * Constraint != target identity.
 *
 * Constraint != scheduling.
 *
 * Constraint != routing.
 *
 * Constraint != hardware discovery.
 */
targetConstraint
    : TARGET_CONSTRAIN targetConstraintExpression SEMICOLON?
    ;


targetConstraintExpression
    : expression
    | targetConstraintBlock
    ;


targetConstraintBlock
    : LBRACE targetConstraintEntry* RBRACE
    ;


targetConstraintEntry
    : identifier comparisonOperator expression SEMICOLON
    | identifier COLON expression SEMICOLON
    | identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 14. TARGET PREFERENCE
 * ============================================================================
 *
 * Preferences are optional.
 *
 * A backend may ignore them.
 *
 * A preference MUST NOT become a hidden semantic requirement.
 */
targetPreference
    : TARGET_PREFER expression SEMICOLON?
    ;


targetPreferenceBlock
    : TARGET_PREFER LBRACE targetPreferenceEntry* RBRACE
    ;


targetPreferenceEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 15. TARGET HINT
 * ============================================================================
 *
 * Hints provide optional implementation guidance.
 *
 * They do not affect program correctness.
 */
targetHint
    : TARGET_HINT expression SEMICOLON?
    ;


targetHintBlock
    : TARGET_HINT LBRACE targetHintEntry* RBRACE
    ;


targetHintEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 16. TARGET CAPABILITY
 * ============================================================================
 *
 * A capability describes a property an acceptable execution environment must
 * expose or may expose.
 *
 * Capability != device.
 */
targetCapability
    : TARGET_CAPABILITY targetCapabilityExpression SEMICOLON?
    ;


targetCapabilityExpression
    : targetCapabilityReference
    | targetCapabilityBlock
    ;


targetCapabilityReference
    : identifier
    | qualifiedIdentifier
    | STRING
    ;


targetCapabilityBlock
    : LBRACE targetCapabilityEntry* RBRACE
    ;


targetCapabilityEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 17. TARGET PROPERTY
 * ============================================================================
 *
 * Generic extensibility point.
 *
 * Properties are semantic data, not grammar-level machine definitions.
 */
targetProperty
    : TARGET_PROPERTY identifier
      (ASSIGN expression)?
      SEMICOLON?
    ;


targetPropertyBlock
    : TARGET_PROPERTY identifier
      LBRACE targetPropertyEntry* RBRACE
    ;


targetPropertyEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 18. TARGET ALTERNATIVE
 * ============================================================================
 *
 * Explicit alternatives are useful for POCO-REAF.
 *
 * Example conceptual meaning:
 *
 *     use quantum when available;
 *     otherwise use a semantically equivalent classical implementation.
 *
 * Semantic equivalence MUST be established downstream.
 */
targetAlternative
    : TARGET_ALTERNATIVE targetAlternativeArm+
    ;


targetAlternativeArm
    : targetExpression
      FAT_ARROW
      targetAlternativeBody
    ;


targetAlternativeBody
    : targetExpression
    | targetBlock
    ;


/*
 * ============================================================================
 * 19. TARGET FALLBACK
 * ============================================================================
 *
 * Fallback expresses a semantic alternative.
 *
 * It does not force a particular hardware backend.
 */
targetFallback
    : TARGET_FALLBACK targetExpression
    ;


targetFallbackChain
    : targetExpression
      (TARGET_FALLBACK targetExpression)*
    ;


/*
 * ============================================================================
 * 20. TARGET COMPATIBILITY
 * ============================================================================
 *
 * Declares compatibility intent between semantic target descriptions.
 *
 * This does not claim that every implementation of one target is compatible
 * with every implementation of another.
 *
 * Semantic analysis must establish compatibility.
 */
targetCompatibility
    : TARGET_COMPATIBLE targetExpression
      (WITH targetExpression)?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 21. TARGET PORTABILITY
 * ============================================================================
 *
 * Explicit portability intent.
 *
 * The language can therefore distinguish:
 *
 *     portable semantics
 *     implementation preference
 *     mandatory implementation property
 */
targetPortability
    : TARGET_PORTABLE targetPortabilityBody?
    ;


targetPortabilityBody
    : targetExpression
    | LBRACE targetPortabilityEntry* RBRACE
    ;


targetPortabilityEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 22. TARGET EXCLUSION
 * ============================================================================
 *
 * Exclusion expresses semantic incompatibility or an explicit forbidden
 * target class.
 *
 * It must not be confused with security policy or hardware quarantine.
 */
targetExclusion
    : TARGET_EXCLUDE targetExpression SEMICOLON?
    ;


/*
 * ============================================================================
 * 23. TARGET RESOURCE INTENT
 * ============================================================================
 *
 * Resource syntax remains intentionally generic.
 *
 * Resource ownership belongs to grammar/resources/.
 *
 * This rule only provides the target-to-resource composition boundary.
 */
targetResourceRequirement
    : TARGET_RESOURCE targetResourceExpression SEMICOLON?
    ;


targetResourceExpression
    : expression
    | targetResourceBlock
    ;


targetResourceBlock
    : LBRACE targetResourceEntry* RBRACE
    ;


targetResourceEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 24. TARGET CAPACITY INTENT
 * ============================================================================
 *
 * Capacity is represented as an expression.
 *
 * Therefore:
 *
 *     8
 *     80
 *     8000
 *     runtime-discovered value
 *
 * are all possible semantic values without grammar changes.
 *
 * The grammar itself imposes no upper bound.
 */
targetCapacity
    : TARGET_CAPACITY expression SEMICOLON?
    ;


targetCapacityConstraint
    : TARGET_CAPACITY targetConstraintExpression SEMICOLON?
    ;


/*
 * ============================================================================
 * 25. TARGET TOPOLOGY INTENT
 * ============================================================================
 *
 * Topology is not described here as a fixed graph.
 *
 * Hardware topology belongs to hardware/topology.g4.
 *
 * This rule permits target-level topology requirements to be expressed as
 * semantic constraints without duplicating topology syntax.
 */
targetTopologyRequirement
    : TARGET_TOPOLOGY expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 26. TARGET PLACEMENT INTENT
 * ============================================================================
 *
 * Placement is a semantic request.
 *
 * Actual placement is owned by compilation/execution/hardware layers.
 */
targetPlacementRequirement
    : TARGET_PLACEMENT expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 27. TARGET PERFORMANCE INTENT
 * ============================================================================
 *
 * Performance properties remain expressions.
 *
 * No fixed unit or threshold is encoded here.
 */
targetPerformanceRequirement
    : TARGET_PERFORMANCE expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 28. TARGET LATENCY INTENT
 * ============================================================================
 */
targetLatencyRequirement
    : TARGET_LATENCY expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 29. TARGET ENERGY INTENT
 * ============================================================================
 */
targetEnergyRequirement
    : TARGET_ENERGY expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 30. TARGET RELIABILITY INTENT
 * ============================================================================
 */
targetReliabilityRequirement
    : TARGET_RELIABILITY expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 31. TARGET SCALABILITY INTENT
 * ============================================================================
 *
 * Scalability is semantic intent.
 *
 * The grammar never encodes a maximum scale.
 */
targetScalabilityRequirement
    : TARGET_SCALABILITY expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 32. TARGET PORTABILITY INTENT
 * ============================================================================
 */
targetPortabilityRequirement
    : TARGET_PORTABILITY expression SEMICOLON?
    ;


/*
 * ============================================================================
 * 33. TARGET KIND
 * ============================================================================
 *
 * Target kind is an open semantic identifier.
 *
 * This avoids a grammar enum that would become obsolete as new computing
 * architectures appear.
 */
targetKind
    : identifier
    | qualifiedIdentifier
    | STRING
    ;


/*
 * ============================================================================
 * 34. TARGET KIND DECLARATION
 * ============================================================================
 */
targetKindDeclaration
    : TARGET_KIND targetKind SEMICOLON?
    ;


/*
 * ============================================================================
 * 35. TARGET NAME
 * ============================================================================
 *
 * Names remain symbolic.
 *
 * A name does not imply a physical device.
 */
targetName
    : identifier
    | qualifiedIdentifier
    | STRING
    ;


/*
 * ============================================================================
 * 36. TARGET REFERENCE
 * ============================================================================
 */
targetReference
    : TARGET_REF targetName
    ;


/*
 * ============================================================================
 * 37. TARGET COMPOSITION
 * ============================================================================
 *
 * Target composition permits heterogeneous execution intent without imposing
 * a fixed number of components.
 *
 * This is important for:
 *
 *     classical + quantum
 *     CPU + GPU
 *     CPU + FPGA
 *     CPU + QPU
 *     GPU + QPU
 *     distributed heterogeneous execution
 *     future architectures
 */
targetComposition
    : TARGET_COMPOSE targetExpressionList
    ;


/*
 * ============================================================================
 * 38. TARGET ROLE
 * ============================================================================
 *
 * Roles describe semantic responsibilities, not physical device identities.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     accelerator
 *     quantum
 *     control
 *     communication
 *
 * The vocabulary remains extensible.
 */
targetRole
    : TARGET_ROLE identifier
    ;


targetRoleBinding
    : targetRole ASSIGN targetExpression
    ;


/*
 * ============================================================================
 * 39. TARGET DOMAIN
 * ============================================================================
 *
 * Domain identifies the computational semantic domain.
 *
 * No finite enum is embedded.
 */
targetDomain
    : TARGET_DOMAIN targetKind
    ;


/*
 * ============================================================================
 * 40. TARGET EXECUTION MODEL
 * ============================================================================
 *
 * Execution model is semantic intent.
 *
 * Actual scheduling/dispatch remains outside this grammar.
 */
targetExecutionModel
    : TARGET_EXECUTION expression
    ;


/*
 * ============================================================================
 * 41. TARGET MEMORY MODEL
 * ============================================================================
 *
 * Memory model belongs semantically to target intent but actual memory
 * realization belongs downstream.
 */
targetMemoryModel
    : TARGET_MEMORY expression
    ;


/*
 * ============================================================================
 * 42. TARGET PRECISION
 * ============================================================================
 */
targetPrecision
    : TARGET_PRECISION expression
    ;


/*
 * ============================================================================
 * 43. TARGET NUMERIC MODEL
 * ============================================================================
 */
targetNumericModel
    : TARGET_NUMERIC expression
    ;


/*
 * ============================================================================
 * 44. TARGET QUANTUM INTENT
 * ============================================================================
 *
 * Quantum target syntax remains backend-independent.
 *
 * This rule may describe semantic quantum capabilities but MUST NOT define:
 *
 *     q[0]
 *     q[1]
 *     MAX_QUBITS
 *     a fixed topology
 *     a physical qubit address
 *
 * Canonical quantum semantics remain owned by grammar/quantum/ and ultimately
 * quantum::ir.
 */
targetQuantum
    : TARGET_QUANTUM targetQuantumBody
    ;


targetQuantumBody
    : expression
    | LBRACE targetQuantumEntry* RBRACE
    ;


targetQuantumEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 45. TARGET CLASSICAL INTENT
 * ============================================================================
 */
targetClassical
    : TARGET_CLASSICAL targetClassicalBody
    ;


targetClassicalBody
    : expression
    | LBRACE targetClassicalEntry* RBRACE
    ;


targetClassicalEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 46. TARGET HARDWARE INTENT
 * ============================================================================
 *
 * This is a reference to hardware semantics, not a hardware description.
 *
 * Hardware structure belongs to grammar/hardware/.
 */
targetHardware
    : TARGET_HARDWARE targetHardwareBody
    ;


targetHardwareBody
    : targetExpression
    | LBRACE targetHardwareEntry* RBRACE
    ;


targetHardwareEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 47. TARGET DISTRIBUTED INTENT
 * ============================================================================
 */
targetDistributed
    : TARGET_DISTRIBUTED targetDistributedBody
    ;


targetDistributedBody
    : expression
    | LBRACE targetDistributedEntry* RBRACE
    ;


targetDistributedEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 48. TARGET ACCELERATOR INTENT
 * ============================================================================
 */
targetAccelerator
    : TARGET_ACCELERATOR targetAcceleratorBody
    ;


targetAcceleratorBody
    : targetExpression
    | LBRACE targetAcceleratorEntry* RBRACE
    ;


targetAcceleratorEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 49. TARGET SECURITY INTENT
 * ============================================================================
 *
 * Security policy itself belongs to grammar/security/.
 *
 * This rule only provides a target-security composition boundary.
 */
targetSecurity
    : TARGET_SECURITY expression
    ;


/*
 * ============================================================================
 * 50. TARGET NETWORK INTENT
 * ============================================================================
 *
 * Network semantics belong to grammar/networking/.
 */
targetNetwork
    : TARGET_NETWORK expression
    ;


/*
 * ============================================================================
 * 51. TARGET DEPLOYMENT INTENT
 * ============================================================================
 *
 * Deployment semantics belong to grammar/execution/.
 *
 * This rule only permits target-level deployment intent to be attached to a
 * compilation target.
 */
targetDeployment
    : TARGET_DEPLOYMENT expression
    ;


/*
 * ============================================================================
 * 52. TARGET ENVIRONMENT
 * ============================================================================
 *
 * Environment refers to a semantic execution context.
 *
 * It does not grant grammar-level access to the host environment.
 */
targetEnvironment
    : TARGET_ENVIRONMENT expression
    ;


/*
 * ============================================================================
 * 53. TARGET VERSION
 * ============================================================================
 *
 * Version constraints are symbolic/version expressions.
 *
 * Compatibility semantics belong downstream.
 */
targetVersion
    : TARGET_VERSION expression
    ;


/*
 * ============================================================================
 * 54. TARGET COMPATIBILITY RANGE
 * ============================================================================
 */
targetCompatibilityRange
    : TARGET_COMPATIBILITY expression
    ;


/*
 * ============================================================================
 * 55. TARGET NEGOTIATION
 * ============================================================================
 *
 * A target may be resolved through capability negotiation.
 *
 * Negotiation itself belongs to semantic compilation/runtime infrastructure.
 */
targetNegotiation
    : TARGET_NEGOTIATE targetExpression
    ;


/*
 * ============================================================================
 * 56. TARGET DISCOVERY REFERENCE
 * ============================================================================
 *
 * This is deliberately only a symbolic reference.
 *
 * The grammar does not perform discovery.
 */
targetDiscoveryReference
    : TARGET_DISCOVER targetName
    ;


/*
 * ============================================================================
 * 57. TARGET RESOLUTION POLICY
 * ============================================================================
 *
 * Resolution policy is semantic compiler intent.
 *
 * It does not select a physical device at parse time.
 */
targetResolutionPolicy
    : TARGET_RESOLUTION expression
    ;


/*
 * ============================================================================
 * 58. TARGET FALLBACK POLICY
 * ============================================================================
 */
targetFallbackPolicy
    : TARGET_FALLBACK_POLICY expression
    ;


/*
 * ============================================================================
 * 59. TARGET ENTRY DISPATCH
 * ============================================================================
 *
 * This is the principal rule imported/used by compile.g4.
 *
 * Keeping target constructs behind one entry point prevents the canonical
 * compilation grammar from depending on every individual target rule.
 */
targetControl
    : targetDeclaration
    | targetProfile
    | targetFamily
    | targetRequirement
    | targetConstraint
    | targetPreference
    | targetHint
    | targetCapability
    | targetAlternative
    | targetFallback
    | targetCompatibility
    | targetPortability
    | targetExclusion
    | targetResourceRequirement
    | targetCapacity
    | targetTopologyRequirement
    | targetPlacementRequirement
    | targetPerformanceRequirement
    | targetLatencyRequirement
    | targetEnergyRequirement
    | targetReliabilityRequirement
    | targetScalabilityRequirement
    | targetPortabilityRequirement
    | targetKindDeclaration
    | targetReference
    | targetComposition
    | targetRoleBinding
    | targetDomain
    | targetExecutionModel
    | targetMemoryModel
    | targetPrecision
    | targetNumericModel
    | targetQuantum
    | targetClassical
    | targetHardware
    | targetDistributed
    | targetAccelerator
    | targetSecurity
    | targetNetwork
    | targetDeployment
    | targetEnvironment
    | targetVersion
    | targetCompatibilityRange
    | targetNegotiation
    | targetDiscoveryReference
    | targetResolutionPolicy
    | targetFallbackPolicy
    ;


/*
 * ============================================================================
 * 60. SHARED GRAMMAR BRIDGES
 * ============================================================================
 *
 * IMPORTANT:
 *
 * These rules are composition contracts.
 *
 * The canonical grammar assembly MUST use the shared definitions from:
 *
 *     grammar/core/
 *     grammar/expressions/
 *     grammar/types/
 *     grammar/resources/
 *
 * This file must never create a second semantic identifier/expression/type
 * system.
 *
 * If the canonical imported grammars already expose these rule names, this
 * grammar MUST consume those rules instead of maintaining local copies.
 */


/*
 * ---------------------------------------------------------------------------
 * Shared expression contract
 * ---------------------------------------------------------------------------
 *
 * `expression` is owned by grammar/expressions/expressions.g4.
 */
targetExpressionBridge
    : expression
    ;


/*
 * ---------------------------------------------------------------------------
 * Shared identifier contract
 * ---------------------------------------------------------------------------
 *
 * `identifier` is owned by grammar/core/names.g4.
 */
targetIdentifierBridge
    : identifier
    ;


/*
 * ---------------------------------------------------------------------------
 * Shared qualified-name contract
 * ---------------------------------------------------------------------------
 *
 * `qualifiedIdentifier` is owned by grammar/core/qualified-names.g4.
 */
targetQualifiedIdentifierBridge
    : qualifiedIdentifier
    ;


/*
 * ---------------------------------------------------------------------------
 * Shared comparison contract
 * ---------------------------------------------------------------------------
 */
targetComparison
    : comparisonOperator expression
    ;


/*
 * ============================================================================
 * 61. TARGET-LEVEL COMPILE UNIT
 * ============================================================================
 *
 * This rule exists for isolated grammar testing.
 *
 * The canonical Zamani parser should NOT use this rule as its program root.
 */
targetCompilationUnit
    : targetControl* EOF
    ;