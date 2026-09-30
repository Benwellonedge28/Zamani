/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/nano/deployment.g4
 *
 * GRAMMAR IDENTITY
 * ----------------
 * NanoDeployment
 *
 * STATUS
 * ------
 * CANONICAL NANO-DOMAIN DEPLOYMENT-INTENT GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL DEPLOYMENT INTENT SPECIFIC TO THE NANO DOMAIN.
 *
 * It describes how a nano-oriented computation, agent, protocol, material,
 * molecular process, atomic process, sensor, actuator, transformation,
 * simulation, or composed nano workload is intended to become available to
 * an execution environment.
 *
 * This grammar deliberately does NOT implement deployment.
 *
 * It describes:
 *
 *     WHAT nano deployment is intended to accomplish.
 *
 * It does NOT describe:
 *
 *     HOW a particular deployment system realizes it.
 *
 * ============================================================================
 * 2. DOMAIN BOUNDARY
 * ============================================================================
 *
 * Generic deployment is owned by:
 *
 *     grammar/execution/deployment.g4
 *
 * Hardware deployment intent is owned by:
 *
 *     grammar/hardware/deployment.g4
 *
 * Resource placement intent is owned by:
 *
 *     grammar/resources/placement.g4
 *
 * Distributed deployment composition is owned by:
 *
 *     grammar/distributed/deployment.g4
 *
 * Nano capabilities are owned by:
 *
 *     grammar/nano/capabilities.g4
 *
 * Nano agents are owned by:
 *
 *     grammar/nano/agents.g4
 *
 * Nano interactions are owned by:
 *
 *     grammar/nano/interactions.g4
 *
 * Nano protocols are owned by:
 *
 *     grammar/nano/protocols.g4
 *
 * Nano atoms are owned by:
 *
 *     grammar/nano/atoms.g4
 *
 * Nano molecules are owned by:
 *
 *     grammar/nano/molecules.g4
 *
 * Nano materials are owned by:
 *
 *     grammar/nano/materials.g4
 *
 * This file composes with those domains but does not duplicate their syntax.
 *
 * ============================================================================
 * 3. ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                       ZamaniParser
 *                              |
 *                              v
 *                         Nano domain
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *          agents         protocols        deployment
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                     Domain-neutral AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *                              v
 *                      semantic analysis
 *                              |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *    resources            capabilities          portability
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        classical         quantum::ir      HDL/hardware
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                  optimization / lowering
 *                              |
 *                  routing / scheduling
 *                              |
 *                     resilience / QEC
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                      target realization
 *
 * This grammar is PARSER-ONLY.
 *
 * ============================================================================
 * 4. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     nanoDeploymentConstruct
 *     nanoDeploymentAnnotatedConstruct
 *     nanoDeploymentAnnotation
 *     nanoDeploymentDeclaration
 *     nanoDeploymentSubject
 *     nanoDeploymentSubjectClause
 *     nanoDeploymentBody
 *     nanoDeploymentMember
 *     nanoDeploymentArtifact
 *     nanoDeploymentRequirement
 *     nanoDeploymentCapability
 *     nanoDeploymentResource
 *     nanoDeploymentConstraint
 *     nanoDeploymentPreference
 *     nanoDeploymentHint
 *     nanoDeploymentEnvironment
 *     nanoDeploymentTarget
 *     nanoDeploymentPlacement
 *     nanoDeploymentLifecycle
 *     nanoDeploymentAvailability
 *     nanoDeploymentRecovery
 *     nanoDeploymentObservability
 *     nanoDeploymentPolicy
 *     nanoDeploymentParameter
 *     nanoDeploymentProperty
 *     nanoDeploymentPropertyBlock
 *     nanoDeploymentPropertyEntry
 *     nanoDeploymentReference
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical rules;
 *     identifiers;
 *     qualified names;
 *     expression precedence;
 *     general types;
 *     general statements;
 *     generic deployment;
 *     hardware deployment;
 *     resource allocation;
 *     resource discovery;
 *     physical placement algorithms;
 *     routing;
 *     scheduling;
 *     hardware discovery;
 *     device allocation;
 *     networking implementation;
 *     quantum operations;
 *     quantum::ir;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     runtime execution;
 *     provider APIs;
 *     cloud APIs;
 *     container implementation;
 *     orchestration implementation.
 *
 * ============================================================================
 * 5. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     argumentList
 *     statement
 *
 * Those concepts are inherited from the canonical parser composition.
 *
 * Nano deployment-specific syntax is the only syntax owned here.
 *
 * ============================================================================
 * 6. OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Nano deployment MUST remain open-world.
 *
 * The grammar MUST NOT enumerate:
 *
 *     nano devices;
 *     atoms;
 *     molecules;
 *     materials;
 *     sensors;
 *     actuators;
 *     fabrication systems;
 *     execution environments;
 *     vendors;
 *     cloud providers;
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     QPUs;
 *     simulators;
 *     clusters;
 *     node types;
 *     physical communication mechanisms.
 *
 * Domain-specific names remain identifiers or qualified names.
 *
 * Examples:
 *
 *     nano::sensor
 *     nano::fabrication
 *     nano::molecular
 *     vendor::future_device
 *     future::nano::substrate
 *
 * Their meaning is resolved semantically.
 *
 * ============================================================================
 * 7. ANNOTATION MODEL
 * ============================================================================
 *
 * The canonical nano deployment form is annotation-led:
 *
 *     @deployment ...
 *
 * The parser intentionally does not hard-code the spelling "deployment".
 *
 * This follows the open-world approach already used by:
 *
 *     grammar/nano/agents.g4
 *     grammar/nano/protocols.g4
 *     grammar/nano/molecules.g4
 *
 * Semantic analysis determines whether the annotation represents a valid
 * nano deployment construct in the current language/domain context.
 *
 * Examples:
 *
 *     @deployment NanoAgent { ... }
 *
 *     @deployment protocol::Assembly { ... }
 *
 *     @deployment nano::simulation { ... }
 *
 * ============================================================================
 * 8. POCO-REAF
 * ============================================================================
 *
 * Nano deployment participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A deployment declaration therefore describes portable deployment intent.
 *
 * It MUST NOT silently bind the program to:
 *
 *     one CPU;
 *     one GPU;
 *     one FPGA;
 *     one ASIC;
 *     one QPU;
 *     one node;
 *     one device;
 *     one physical qubit;
 *     one memory bank;
 *     one network interface;
 *     one vendor;
 *     one cloud provider.
 *
 * Target-specific deployment is permitted only when it is explicitly part of
 * the program's semantic contract or selected through a target-specific
 * dialect/profile.
 *
 * ============================================================================
 * 9. REQUIREMENT / CAPABILITY / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts are deliberately distinct.
 *
 * REQUIREMENT:
 *
 *     mandatory semantic condition.
 *
 * CAPABILITY:
 *
 *     required or referenced capability.
 *
 * RESOURCE:
 *
 *     required resource property or quantity.
 *
 * CONSTRAINT:
 *
 *     condition restricting legal realizations.
 *
 * PREFERENCE:
 *
 *     advisory optimization preference.
 *
 * HINT:
 *
 *     advisory information that does not change program meaning.
 *
 * The grammar does not decide whether a requirement is satisfiable.
 *
 * Semantic/resource analysis does.
 *
 * ============================================================================
 * 10. NO ARTIFICIAL SCALABILITY CEILINGS
 * ============================================================================
 *
 * This grammar contains NO universal limits on:
 *
 *     deployments;
 *     subjects;
 *     artifacts;
 *     agents;
 *     protocols;
 *     atoms;
 *     molecules;
 *     materials;
 *     resources;
 *     capabilities;
 *     replicas;
 *     instances;
 *     nodes;
 *     devices;
 *     environments;
 *     parameters;
 *     properties;
 *     deployment members.
 *
 * It MUST NOT define:
 *
 *     MAX_NANO_DEPLOYMENTS
 *     MAX_NANO_AGENTS
 *     MAX_NANO_DEVICES
 *     MAX_NANO_RESOURCES
 *     MAX_NANO_INSTANCES
 *     MAX_NANO_REPLICAS
 *     MAX_NANO_NODES
 *     MAX_NANO_MEMORY
 *     MAX_NANO_ENERGY
 *     MAX_NANO_BANDWIDTH
 *     MAX_NANO_DEPTH
 *
 * Collections use parser repetition.
 *
 * Quantities are expressions.
 *
 * Actual limits belong to:
 *
 *     program semantics;
 *     resource analysis;
 *     target capabilities;
 *     runtime resources;
 *     deployment policies;
 *     operating-system constraints;
 *     provider constraints.
 *
 * ============================================================================
 * 11. NO HARD-CODED HARDWARE
 * ============================================================================
 *
 * The grammar MUST NOT encode:
 *
 *     CPU_0
 *     GPU_0
 *     FPGA_0
 *     QPU_0
 *     NODE_0
 *     DEVICE_0
 *     physical_qubit_0
 *     memory_bank_0
 *
 * as language-level deployment resources.
 *
 * A program may contain such names as ordinary program data if a semantic
 * contract explicitly requires them.
 *
 * That does not turn them into grammar-level hardware primitives.
 *
 * ============================================================================
 * 12. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no parser actions;
 *     no semantic predicates;
 *     no embedded Rust;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no resource discovery;
 *     no randomness;
 *     no environment inspection;
 *     no runtime execution.
 *
 * Parsing depends only on:
 *
 *     source tokens;
 *     selected grammar;
 *     parser configuration;
 *     language/dialect configuration explicitly supplied to the parser.
 *
 * ============================================================================
 * 13. SAFETY
 * ============================================================================
 *
 * The grammar contains no Rust code.
 *
 * The Rust implementation consuming this grammar MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 */

parser grammar NanoDeployment;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/*
 * ============================================================================
 * 14. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Exactly one public nano-deployment entry point is exposed.
 *
 * The universal parser/domain dispatcher is responsible for deciding when
 * this construct is legal.
 *
 * ============================================================================
 */

nanoDeploymentConstruct
    : nanoDeploymentAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 15. ANNOTATED DEPLOYMENT
 * ============================================================================
 */

nanoDeploymentAnnotatedConstruct
    : nanoDeploymentAnnotation
      nanoDeploymentDeclaration
    ;


/*
 * ============================================================================
 * 16. DEPLOYMENT ANNOTATION
 * ============================================================================
 *
 * Examples:
 *
 *     @deployment
 *     @nano_deployment
 *     @future::deployment
 *
 * The parser does not assign semantic meaning to the identifier.
 */

nanoDeploymentAnnotation
    : AT identifier
    ;


/*
 * ============================================================================
 * 17. DEPLOYMENT DECLARATION
 * ============================================================================
 *
 * Supported forms:
 *
 *     @deployment Subject;
 *
 *     @deployment Subject { ... }
 *
 *     @deployment Subject = expression;
 *
 *     @deployment Subject(parameters) { ... }
 *
 * The subject is intentionally an expression rather than a closed
 * enumeration of nano entities.
 */

nanoDeploymentDeclaration
    : nanoDeploymentSubject
      nanoDeploymentSubjectClause?
      nanoDeploymentInitializer?
      nanoDeploymentBody?
      SEMI?
    ;


/*
 * ============================================================================
 * 18. DEPLOYMENT SUBJECT
 * ============================================================================
 *
 * A deployment subject may refer to:
 *
 *     nano agent;
 *     protocol;
 *     interaction;
 *     atom;
 *     molecule;
 *     material;
 *     simulation;
 *     transformation;
 *     computation;
 *     pipeline;
 *     function;
 *     module;
 *     hybrid workload;
 *     future nano-domain entity.
 *
 * The semantic layer determines its actual category.
 */

nanoDeploymentSubject
    : qualifiedName
    ;


/*
 * ============================================================================
 * 19. SUBJECT PARAMETERS
 * ============================================================================
 *
 * Parameters remain expressions and therefore do not impose finite arity.
 */

nanoDeploymentSubjectClause
    : LPAREN
      nanoDeploymentArgumentList?
      RPAREN
    ;


nanoDeploymentArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 20. INITIALIZER
 * ============================================================================
 */

nanoDeploymentInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 21. DEPLOYMENT BODY
 * ============================================================================
 */

nanoDeploymentBody
    : LBRACE
      nanoDeploymentMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 22. DEPLOYMENT MEMBER DISPATCH
 * ============================================================================
 *
 * Explicit semantic introducers are preferred over ambiguous identifier-led
 * alternatives.
 *
 * Open-world properties remain available through nanoDeploymentProperty.
 */

nanoDeploymentMember
    : nanoDeploymentArtifact
    | nanoDeploymentRequirement
    | nanoDeploymentCapability
    | nanoDeploymentResource
    | nanoDeploymentConstraint
    | nanoDeploymentPreference
    | nanoDeploymentHint
    | nanoDeploymentEnvironment
    | nanoDeploymentTarget
    | nanoDeploymentPlacement
    | nanoDeploymentLifecycle
    | nanoDeploymentAvailability
    | nanoDeploymentRecovery
    | nanoDeploymentObservability
    | nanoDeploymentPolicy
    | nanoDeploymentParameter
    | nanoDeploymentProperty
    | statement
    ;


/*
 * ============================================================================
 * 23. ARTIFACT
 * ============================================================================
 *
 * The artifact is a semantic reference.
 *
 * Storage, hashing, signing, retrieval, verification and packaging belong
 * outside this grammar.
 */

nanoDeploymentArtifact
    : nanoDeploymentArtifactMarker
      qualifiedName
      nanoDeploymentValue?
      SEMI
    ;


nanoDeploymentArtifactMarker
    : identifier
    ;


/*
 * ============================================================================
 * 24. REQUIREMENT
 * ============================================================================
 *
 * Example:
 *
 *     requires capability("nano.fabrication");
 *
 *     requires memory >= required_memory;
 *
 *     requires resources >= required_resources;
 *
 * The grammar does not impose resource ceilings.
 */

nanoDeploymentRequirement
    : REQUIRES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 25. CAPABILITY
 * ============================================================================
 *
 * Capability names remain open.
 *
 * Examples:
 *
 *     capability("nano.sensing")
 *     capability("nano.molecular")
 *     capability("quantum.measurement")
 *     capability("gpu.compute")
 *
 * The grammar does not enumerate capabilities.
 */

nanoDeploymentCapability
    : CAPABILITY
      qualifiedName
      nanoDeploymentCapabilityValue?
      SEMI
    ;


nanoDeploymentCapabilityValue
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 26. RESOURCE
 * ============================================================================
 *
 * Resource quantities are expressions.
 *
 * Examples:
 *
 *     resource memory = required_memory;
 *
 *     resource processing = required_processing;
 *
 *     resource quantum = required_quantum;
 *
 *     resource energy = required_energy;
 *
 * These are requirements/intent, not allocation.
 */

nanoDeploymentResource
    : nanoDeploymentResourceMarker
      qualifiedName
      nanoDeploymentAssignment
      expression
      SEMI
    ;


nanoDeploymentResourceMarker
    : identifier
    ;


/*
 * ============================================================================
 * 27. CONSTRAINT
 * ============================================================================
 */

nanoDeploymentConstraint
    : CONSTRAINT
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 28. PREFERENCE
 * ============================================================================
 */

nanoDeploymentPreference
    : PREFER
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 29. HINT
 * ============================================================================
 *
 * Hints are intentionally identifier-led because the canonical lexer should
 * not grow a keyword for every future deployment property.
 */

nanoDeploymentHint
    : nanoDeploymentHintMarker
      expression
      SEMI
    ;


nanoDeploymentHintMarker
    : identifier
    ;


/*
 * ============================================================================
 * 30. ENVIRONMENT
 * ============================================================================
 *
 * Environment remains abstract.
 *
 * Examples of semantic names:
 *
 *     embedded
 *     edge
 *     cloud
 *     distributed
 *     quantum
 *     simulator
 *     laboratory
 *     future::environment
 *
 * None is a grammar-level enumeration.
 */

nanoDeploymentEnvironment
    : nanoDeploymentEnvironmentMarker
      expression
      SEMI
    ;


nanoDeploymentEnvironmentMarker
    : identifier
    ;


/*
 * ============================================================================
 * 31. TARGET
 * ============================================================================
 *
 * A target may be an abstract target expression.
 *
 * The semantic target-resolution system determines whether it is compatible.
 */

nanoDeploymentTarget
    : nanoDeploymentTargetMarker
      expression
      SEMI
    ;


nanoDeploymentTargetMarker
    : identifier
    ;


/*
 * ============================================================================
 * 32. PLACEMENT
 * ============================================================================
 *
 * Placement is intent only.
 *
 * Actual placement belongs to:
 *
 *     resources/placement.g4
 *     execution/placement.g4
 *     hardware/placement.g4
 *     distributed/placement.g4
 *
 * depending on semantic domain.
 */

nanoDeploymentPlacement
    : nanoDeploymentPlacementMarker
      expression
      SEMI
    ;


nanoDeploymentPlacementMarker
    : identifier
    ;


/*
 * ============================================================================
 * 33. LIFECYCLE
 * ============================================================================
 *
 * Lifecycle names remain semantic data.
 *
 * No finite lifecycle enumeration is imposed.
 */

nanoDeploymentLifecycle
    : nanoDeploymentLifecycleMarker
      expression
      SEMI
    ;


nanoDeploymentLifecycleMarker
    : identifier
    ;


/*
 * ============================================================================
 * 34. AVAILABILITY
 * ============================================================================
 */

nanoDeploymentAvailability
    : nanoDeploymentAvailabilityMarker
      expression
      SEMI
    ;


nanoDeploymentAvailabilityMarker
    : identifier
    ;


/*
 * ============================================================================
 * 35. RECOVERY
 * ============================================================================
 */

nanoDeploymentRecovery
    : nanoDeploymentRecoveryMarker
      expression
      SEMI
    ;


nanoDeploymentRecoveryMarker
    : identifier
    ;


/*
 * ============================================================================
 * 36. OBSERVABILITY
 * ============================================================================
 */

nanoDeploymentObservability
    : nanoDeploymentObservabilityMarker
      expression
      SEMI
    ;


nanoDeploymentObservabilityMarker
    : identifier
    ;


/*
 * ============================================================================
 * 37. POLICY
 * ============================================================================
 *
 * A policy may be a named expression or structured property block.
 */

nanoDeploymentPolicy
    : nanoDeploymentPolicyMarker
      nanoDeploymentPolicyValue
      SEMI?
    ;


nanoDeploymentPolicyMarker
    : identifier
    ;


nanoDeploymentPolicyValue
    : expression
    | nanoDeploymentPropertyBlock
    ;


/*
 * ============================================================================
 * 38. PARAMETER
 * ============================================================================
 *
 * Parameters may use either:
 *
 *     name: value;
 *
 * or:
 *
 *     name = value;
 *
 * This remains open-world.
 */

nanoDeploymentParameter
    : qualifiedName
      nanoDeploymentAssignment
      expression
      SEMI
    ;


nanoDeploymentAssignment
    : COLON
    | ASSIGN
    ;


/*
 * ============================================================================
 * 39. OPEN DEPLOYMENT PROPERTY
 * ============================================================================
 *
 * This is the principal long-term extension point.
 *
 * Examples:
 *
 *     rollout: policy;
 *     energy: budget;
 *     fidelity: required_fidelity;
 *     fabrication::mode: expression;
 *     future::nano::property: value;
 *
 * New semantic properties do not require new lexer tokens or grammar
 * alternatives.
 */

nanoDeploymentProperty
    : qualifiedName
      nanoDeploymentAssignment
      nanoDeploymentValue
      SEMI
    ;


/*
 * ============================================================================
 * 40. PROPERTY BLOCK
 * ============================================================================
 *
 * Nested property blocks allow arbitrarily structured deployment metadata.
 */

nanoDeploymentPropertyBlock
    : LBRACE
      nanoDeploymentPropertyEntry*
      RBRACE
    ;


nanoDeploymentPropertyEntry
    : qualifiedName
      nanoDeploymentAssignment
      nanoDeploymentValue
      SEMI
    ;


/*
 * ============================================================================
 * 41. DEPLOYMENT VALUE
 * ============================================================================
 *
 * Values reuse the canonical expression language.
 *
 * A nested property block is additionally permitted for structured
 * deployment metadata.
 */

nanoDeploymentValue
    : expression
    | nanoDeploymentPropertyBlock
    ;


/*
 * ============================================================================
 * 42. DEPLOYMENT REFERENCE
 * ============================================================================
 *
 * A deployment may explicitly reference a nano-domain declaration without
 * embedding that declaration inside the deployment grammar.
 *
 * The semantic resolver determines whether the reference denotes:
 *
 *     agent;
 *     protocol;
 *     interaction;
 *     atom;
 *     molecule;
 *     material;
 *     computation;
 *     simulation;
 *     transformation;
 *     future domain entity.
 */

nanoDeploymentReference
    : qualifiedName
    ;