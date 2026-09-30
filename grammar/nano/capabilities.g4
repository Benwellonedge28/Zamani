/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/nano/capabilities.g4
 *
 * GRAMMAR
 * -------
 * NanoCapabilities
 *
 * STATUS
 * ------
 * CANONICAL NANO-DOMAIN CAPABILITY ADAPTER GRAMMAR
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
 * This file owns the SOURCE-LEVEL NANO-DOMAIN USE of the canonical Zamani
 * capability system.
 *
 * It does NOT create a second capability language.
 *
 * Canonical capability identity and generic capability syntax remain owned
 * exclusively by:
 *
 *     grammar/core/capabilities.g4
 *
 * This file provides the nano-domain adapter around that canonical model.
 *
 * It supports capability intent for:
 *
 *     - nano agents;
 *     - atoms;
 *     - molecules;
 *     - materials;
 *     - nano interactions;
 *     - nano protocols;
 *     - nano transformations;
 *     - nano observations;
 *     - nano actions;
 *     - nano fabrication intent;
 *     - nano sensing;
 *     - nano actuation;
 *     - nano coordination;
 *     - nano communication;
 *     - nano simulation;
 *     - nano/classical computation;
 *     - nano/quantum computation;
 *     - nano/HDL/hardware co-design;
 *     - future nano-domain extensions.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                    canonical parser layer
 *                              |
 *                              v
 *                      NanoCapabilities
 *                              |
 *                              v
 *                       Domain-neutral AST
 *                              |
 *                              v
 *                    semantic capability model
 *                              |
 *              +---------------+---------------+
 *              |               |               |
 *              v               v               v
 *        nano semantics   resource analysis  security
 *              |               |               |
 *              +---------------+---------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir        HDL/hardware
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                 optimization / lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                  resilience / QEC / ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                       target realization
 *
 * This file is PARSER-ONLY.
 *
 * It does not:
 *
 *     - discover capabilities;
 *     - allocate resources;
 *     - select devices;
 *     - select vendors;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform optimization;
 *     - perform simulation;
 *     - perform chemistry;
 *     - perform quantum execution;
 *     - perform QEC;
 *     - perform ZQN;
 *     - construct IR;
 *     - execute runtime operations.
 *
 * ============================================================================
 * 3. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Canonical capability ownership:
 *
 *     grammar/core/capabilities.g4
 *
 * This file MUST reuse:
 *
 *     capabilityDeclaration
 *     capabilityReference
 *     capabilityName
 *     capabilityVersionClause
 *     capabilityExpression
 *     capabilityAlias
 *     capabilityRequirementReference
 *     capabilityProvisionReference
 *     capabilityPreferenceReference
 *     capabilityConstraintReference
 *
 * where applicable.
 *
 * This file MUST NOT redefine:
 *
 *     capabilityName
 *     capabilityReference
 *     capabilityVersion
 *     capabilityVersionClause
 *     capabilityExpression
 *     qualifiedName
 *     identifier
 *
 * ============================================================================
 * 4. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     nanoCapabilityConstruct
 *     nanoCapabilityContract
 *     nanoCapabilityContractBody
 *     nanoCapabilityContractMember
 *     nanoCapabilityRequirement
 *     nanoCapabilityPreference
 *     nanoCapabilityConstraint
 *     nanoCapabilityHint
 *     nanoCapabilityProperty
 *     nanoCapabilityReferenceMember
 *     nanoCapabilityCapabilityMember
 *     nanoCapabilityAnnotation
 *     nanoCapabilityExpression
 *     nanoCapabilityReference
 *     nanoCapabilityRequirementReference
 *     nanoCapabilityProvisionReference
 *     nanoCapabilityPreferenceReference
 *     nanoCapabilityConstraintReference
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical rules;
 *     keywords;
 *     identifiers;
 *     qualified names;
 *     generic capability identity;
 *     generic capability version semantics;
 *     generic expression syntax;
 *     generic type syntax;
 *     requirements outside nano capability context;
 *     resources;
 *     hardware discovery;
 *     target selection;
 *     device placement;
 *     routing;
 *     scheduling;
 *     calibration;
 *     quantum::ir;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     runtime execution.
 *
 * ============================================================================
 * 5. OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Nano capability names are deliberately NOT enumerated.
 *
 * Valid semantic identities may include examples such as:
 *
 *     nano::sensing
 *     nano::actuation
 *     nano::molecular_assembly
 *     nano::material_transformation
 *     nano::agent_coordination
 *     nano::interaction
 *     nano::observation
 *     nano::fabrication
 *     nano::simulation
 *     nano::quantum_hybrid
 *
 * These are examples of semantic names only.
 *
 * They are NOT grammar alternatives.
 *
 * Future capabilities such as:
 *
 *     nano::future::capability
 *     molecular::future::interaction
 *     material::new_process
 *     quantum::nano::new_operation
 *
 * remain syntactically representable without changing this file.
 *
 * ============================================================================
 * 6. POCO-REAF
 * ============================================================================
 *
 * Capability syntax describes WHAT a nano-oriented computation requires,
 * provides, prefers, constrains, or references.
 *
 * It does not describe WHICH physical realization must be used.
 *
 * The source language therefore MUST NOT encode:
 *
 *     a particular nano device;
 *     a particular atom;
 *     a particular molecule instance;
 *     a particular material sample;
 *     a particular sensor;
 *     a particular actuator;
 *     a particular CPU;
 *     a particular GPU;
 *     a particular FPGA;
 *     a particular ASIC;
 *     a particular QPU;
 *     a particular node;
 *     a particular memory bank;
 *     a particular physical network;
 *     a particular vendor.
 *
 * The same capability contract may therefore be realized by:
 *
 *     a symbolic implementation;
 *     a classical implementation;
 *     a quantum implementation;
 *     a hybrid implementation;
 *     an embedded implementation;
 *     an accelerator;
 *     an FPGA;
 *     an ASIC;
 *     an HPC system;
 *     a distributed system;
 *     a simulator;
 *     a future computational substrate.
 *
 * ============================================================================
 * 7. NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     MAX_NANO_CAPABILITIES
 *     MAX_NANO_REQUIREMENTS
 *     MAX_NANO_DEVICES
 *     MAX_NANO_AGENTS
 *     MAX_NANO_ATOMS
 *     MAX_NANO_MOLECULES
 *     MAX_NANO_MATERIALS
 *     MAX_NANO_INTERACTIONS
 *     MAX_NANO_PROTOCOLS
 *     MAX_NANO_SENSORS
 *     MAX_NANO_ACTUATORS
 *     MAX_NANO_NODES
 *     MAX_NANO_MEMORY
 *     MAX_NANO_ENERGY
 *     MAX_NANO_BANDWIDTH
 *     MAX_NANO_DEPTH
 *
 * Repeated structures use ANTLR repetition:
 *
 *     *
 *     +
 *
 * The grammar therefore imposes no finite semantic cardinality.
 *
 * "Infinity" means:
 *
 *     no artificial language ceiling.
 *
 * It does NOT mean that physical hardware has infinite resources.
 *
 * Resource exhaustion or infeasibility is evaluated downstream.
 *
 * ============================================================================
 * 8. CAPABILITY / REQUIREMENT / RESOURCE SEPARATION
 * ============================================================================
 *
 * CAPABILITY
 *
 *     What an implementation can provide.
 *
 * REQUIREMENT
 *
 *     What the program requires.
 *
 * CONSTRAINT
 *
 *     What a valid realization must satisfy.
 *
 * PREFERENCE
 *
 *     Which valid realization is preferred.
 *
 * HINT
 *
 *     Advisory implementation information.
 *
 * RESOURCE
 *
 *     A measurable or allocatable execution/resource property.
 *
 * TARGET
 *
 *     An abstract realization context.
 *
 * These concepts MUST remain distinct.
 *
 * ============================================================================
 * 9. CANONICAL DEPENDENCIES
 * ============================================================================
 */

parser grammar NanoCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Capabilities, Expressions, Types;


/*
 * ============================================================================
 * 10. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A nano capability construct is either:
 *
 *     - a nano capability contract;
 *     - a nano capability reference.
 *
 * Generic capability declarations remain owned by Capabilities.
 */

nanoCapabilityConstruct
    : nanoCapabilityContract
    | nanoCapabilityReferenceConstruct
    ;


/*
 * ============================================================================
 * 11. NANO CAPABILITY CONTRACT
 * ============================================================================
 *
 * Example:
 *
 *     capability nano::sensing {
 *         requires nano::observation;
 *         prefer nano::low_energy;
 *         constraint expression;
 *         hint expression;
 *     }
 *
 * The leading capability identity is canonical.
 *
 * The contract body is nano-domain syntax.
 */

nanoCapabilityContract
    : CAPABILITY
      capabilityName
      nanoCapabilityContractBody
      SEMI?
    ;


nanoCapabilityContractBody
    : LBRACE
      nanoCapabilityContractMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 12. CONTRACT MEMBERS
 * ============================================================================
 */

nanoCapabilityContractMember
    : nanoCapabilityRequirement
    | nanoCapabilityPreference
    | nanoCapabilityConstraint
    | nanoCapabilityHint
    | nanoCapabilityProperty
    | nanoCapabilityCapabilityMember
    | nanoCapabilityReferenceMember
    | nanoCapabilityAnnotation
    ;


/*
 * ============================================================================
 * 13. REQUIREMENT
 * ============================================================================
 *
 * Example:
 *
 *     requires nano::sensing;
 *
 *     requires nano::molecular_assembly;
 *
 *     requires capability_expression;
 *
 * Requirement satisfaction is semantic analysis.
 */

nanoCapabilityRequirement
    : REQUIRES
      capabilityExpression
      SEMI
    ;


/*
 * ============================================================================
 * 14. PREFERENCE
 * ============================================================================
 *
 * A preference is NOT a requirement.
 *
 * Example:
 *
 *     prefer nano::low_energy;
 *
 * Failure to honor a preference must not make a semantically valid program
 * syntactically invalid.
 */

nanoCapabilityPreference
    : PREFER
      capabilityExpression
      SEMI
    ;


/*
 * ============================================================================
 * 15. CONSTRAINT
 * ============================================================================
 *
 * A constraint may refer to a capability or to an ordinary expression.
 *
 * Examples:
 *
 *     constraint nano::deterministic;
 *
 *     constraint energy <= budget;
 *
 * The grammar preserves the distinction while semantic analysis determines
 * the meaning.
 */

nanoCapabilityConstraint
    : CONSTRAINT
      nanoCapabilityConstraintExpression
      SEMI
    ;


nanoCapabilityConstraintExpression
    : capabilityExpression
    | expression
    ;


/*
 * ============================================================================
 * 16. HINT
 * ============================================================================
 *
 * Hints are advisory.
 *
 * A compiler may ignore a hint without changing program correctness.
 */

nanoCapabilityHint
    : HINT
      (
          capabilityExpression
        | expression
      )
      SEMI
    ;


/*
 * ============================================================================
 * 17. PROPERTY
 * ============================================================================
 *
 * Open-world properties avoid adding a keyword for every future nano-domain
 * property.
 *
 * Examples:
 *
 *     mode = symbolic;
 *     model = molecular;
 *     energy_budget = budget;
 *     deterministic = true;
 *
 * Property semantics belong downstream.
 */

nanoCapabilityProperty
    : identifier
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 18. CAPABILITY MEMBER
 * ============================================================================
 *
 * A capability contract may explicitly reference another capability.
 *
 * Example:
 *
 *     nano::sensing;
 *
 * This does not create a second capability identity.
 */

nanoCapabilityCapabilityMember
    : capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 19. CAPABILITY REFERENCE MEMBER
 * ============================================================================
 *
 * This named adapter makes the integration contract explicit for consumers
 * such as protocols, agents, materials, atoms, and molecules.
 */

nanoCapabilityReferenceMember
    : capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 20. STANDALONE CAPABILITY REFERENCE
 * ============================================================================
 *
 * A standalone reference is intentionally syntactically minimal.
 *
 * The consuming declaration determines whether the reference represents:
 *
 *     requirement;
 *     provision;
 *     preference;
 *     constraint;
 *     effect;
 *     annotation;
 *     metadata;
 *     another semantic relation.
 */

nanoCapabilityReferenceConstruct
    : nanoCapabilityReference
      SEMI
    ;


nanoCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * 21. EXPLICIT REFERENCE ADAPTERS
 * ============================================================================
 *
 * These aliases allow domain grammars to consume the canonical capability
 * representation without reproducing capability syntax.
 */

nanoCapabilityRequirementReference
    : capabilityRequirementReference
    ;


nanoCapabilityProvisionReference
    : capabilityProvisionReference
    ;


nanoCapabilityPreferenceReference
    : capabilityPreferenceReference
    ;


nanoCapabilityConstraintReference
    : capabilityConstraintReference
    ;


/*
 * ============================================================================
 * 22. CAPABILITY EXPRESSION ADAPTER
 * ============================================================================
 *
 * The canonical capability expression remains authoritative.
 *
 * This alias exists only to provide a stable nano-domain integration name.
 */

nanoCapabilityExpression
    : capabilityExpression
    ;


/*
 * ============================================================================
 * 23. ANNOTATION
 * ============================================================================
 *
 * Nano domains already use open-world annotations in:
 *
 *     grammar/nano/agents.g4
 *     grammar/nano/atoms.g4
 *     grammar/nano/molecules.g4
 *     grammar/nano/materials.g4
 *     grammar/nano/protocols.g4
 *
 * This rule deliberately does not define annotation semantics.
 *
 * The annotation name remains an identifier.
 *
 * Examples:
 *
 *     @capability
 *     @requires
 *     @provides
 *     @observe
 *     @act
 *     @coordinate
 *
 * Semantic analysis determines whether an annotation is valid in context.
 */

nanoCapabilityAnnotation
    : AT
      identifier
      (
          LPAREN
          argumentList?
          RPAREN
      )?
      SEMI
    ;


/*
 * ============================================================================
 * 24. ATOMIC INTEGRATION
 * ============================================================================
 *
 * grammar/nano/atoms.g4 may consume:
 *
 *     nanoCapabilityReference
 *     nanoCapabilityRequirementReference
 *     nanoCapabilityExpression
 *
 * for atom-local capability intent.
 *
 * This file does not define atomic structure.
 *
 * Atomic structure remains owned by:
 *
 *     grammar/nano/atoms.g4
 *
 * Physical atomic meaning remains semantic/domain data.
 */


/*
 * ============================================================================
 * 25. MOLECULAR INTEGRATION
 * ============================================================================
 *
 * grammar/nano/molecules.g4 may consume canonical nano capability references
 * for:
 *
 *     molecular assembly;
 *     molecular transformation;
 *     molecular observation;
 *     molecular simulation;
 *     molecular interaction;
 *     molecular resource requirements.
 *
 * Molecular structure remains owned by:
 *
 *     grammar/nano/molecules.g4
 *
 * This file does not implement chemistry.
 */


/*
 * ============================================================================
 * 26. MATERIAL INTEGRATION
 * ============================================================================
 *
 * grammar/nano/materials.g4 may consume this adapter for:
 *
 *     material transformation;
 *     material observation;
 *     material synthesis;
 *     material simulation;
 *     material processing;
 *     material constraints.
 *
 * Material semantics remain owned by the material semantic subsystem.
 */


/*
 * ============================================================================
 * 27. AGENT INTEGRATION
 * ============================================================================
 *
 * grammar/nano/agents.g4 already supports nano-agent capabilities and
 * requirements.
 *
 * It must reuse:
 *
 *     capabilityReference
 *     capabilityExpression
 *
 * through this adapter rather than creating an independent nano capability
 * identity model.
 *
 * Agent behavior remains owned by NanoAgents.
 *
 * Capability meaning remains owned by semantic analysis.
 */


/*
 * ============================================================================
 * 28. PROTOCOL INTEGRATION
 * ============================================================================
 *
 * grammar/nano/protocols.g4 already contains a
 * nanoProtocolCapabilityDeclaration rule.
 *
 * That rule currently expresses:
 *
 *     CAPABILITY qualifiedName (ASSIGN expression)? SEMI
 *
 * The production architecture should migrate that rule to consume:
 *
 *     capabilityReference
 *
 * from the canonical capability grammar, with this file supplying the
 * nano-domain adapter.
 *
 * The protocol grammar must NOT create a second capability identity model.
 *
 * Recommended integration:
 *
 *     NanoProtocols
 *          |
 *          v
 *     NanoCapabilities
 *          |
 *          v
 *     Capabilities
 *
 * rather than:
 *
 *     NanoProtocols
 *          |
 *          +--> private capability syntax
 *
 * This keeps capability version constraints available to nano protocols.
 */


/*
 * ============================================================================
 * 29. INTERACTION INTEGRATION
 * ============================================================================
 *
 * Future/actual:
 *
 *     grammar/nano/interactions.g4
 *
 * may consume:
 *
 *     nanoCapabilityReference
 *     nanoCapabilityExpression
 *     nanoCapabilityRequirementReference
 *
 * Interaction semantics remain outside this grammar.
 *
 * No physical interaction mechanism is enumerated here.
 */


/*
 * ============================================================================
 * 30. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Nano capability contracts may require classical computation.
 *
 * Example semantic identity:
 *
 *     classical::compute
 *
 * This grammar does not define that capability.
 *
 * The canonical capability registry and classical semantic layer determine
 * its meaning.
 */


/*
 * ============================================================================
 * 31. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Nano capability contracts may refer to quantum capabilities.
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::hybrid
 *
 * This file does not define quantum syntax.
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * No NanoQuantumIR is permitted.
 */


/*
 * ============================================================================
 * 32. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Nano capabilities may be satisfied by:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     embedded system;
 *     future hardware.
 *
 * The source capability remains target-independent.
 *
 * Hardware realization is handled downstream by:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *     compiler planning;
 *     target analysis;
 *     HAL.
 *
 * This grammar must never select:
 *
 *     device 0;
 *     GPU 0;
 *     QPU 0;
 *     FPGA 0;
 *
 * as universal language constructs.
 */


/*
 * ============================================================================
 * 33. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Capability and resource are different semantic categories.
 *
 * Example:
 *
 *     requires nano::sensing;
 *
 * is not equivalent to:
 *
 *     allocate sensor;
 *
 * and:
 *
 *     requires memory >= required_memory;
 *
 * is not equivalent to:
 *
 *     use memory bank 0;
 *
 * Resource realization belongs downstream.
 */


/*
 * ============================================================================
 * 34. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Nano capability contracts may be satisfied by distributed execution.
 *
 * The source grammar does not prescribe:
 *
 *     node count;
 *     process count;
 *     worker count;
 *     network topology;
 *     communication route.
 *
 * Those are realization decisions.
 */


/*
 * ============================================================================
 * 35. SECURITY INTEGRATION
 * ============================================================================
 *
 * A capability reference is NOT authorization.
 *
 * For example:
 *
 *     requires nano::fabrication;
 *
 * expresses semantic intent.
 *
 * It does not grant permission to fabricate anything.
 *
 * Security analysis must independently determine:
 *
 *     authorization;
 *     policy;
 *     trust;
 *     identity;
 *     provenance;
 *     allowed operations.
 */


/*
 * ============================================================================
 * 36. AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the existing domain-neutral
 * AST to represent:
 *
 *     capability identity;
 *     version requirement;
 *     capability expression;
 *     requirement/preference/constraint/hint role;
 *     property expression;
 *     annotations;
 *     source spans;
 *     source ordering.
 *
 * The AST must NOT gain:
 *
 *     physical device IDs;
 *     resource allocation;
 *     hardware state;
 *     runtime capability tokens;
 *     scheduler state;
 *     routing state;
 *     calibration state.
 *
 * Prefer the repository's existing native Capability representation rather
 * than creating NanoCapabilityIR.
 */


/*
 * ============================================================================
 * 37. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     capability existence;
 *     capability namespace resolution;
 *     capability version compatibility;
 *     capability classification;
 *     requirement satisfaction;
 *     capability conflicts;
 *     resource implications;
 *     target matching;
 *     security policy;
 *     domain compatibility;
 *     quantum compatibility;
 *     hardware compatibility;
 *     portability.
 *
 * None of these decisions occur in the parser.
 */


/*
 * ============================================================================
 * 38. IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT create an IR.
 *
 * Correct direction:
 *
 *     nano capability syntax
 *              |
 *              v
 *     domain-neutral AST
 *              |
 *              v
 *     semantic capability model
 *              |
 *       +------+------+
 *       |             |
 *       v             v
 *   classical     quantum::ir
 *       |             |
 *       +------+------+
 *              |
 *              v
 *       target lowering
 *
 * There must be no:
 *
 *     NanoCapabilityIR
 *     NanoQuantumIR
 *     NanoHardwareIR
 *
 * created by this grammar.
 */


/*
 * ============================================================================
 * 39. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source tokens;
 *     grammar version;
 *     parser configuration.
 *
 * It MUST NOT depend on:
 *
 *     hardware;
 *     resource availability;
 *     network state;
 *     filesystem state;
 *     environment variables;
 *     wall-clock time;
 *     randomness;
 *     device discovery;
 *     runtime state.
 */


/*
 * ============================================================================
 * 40. SECURITY / SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no filesystem operations;
 *     no network operations;
 *     no shell execution;
 *     no hardware discovery;
 *     no credentials;
 *     no runtime callbacks;
 *     no unsafe implementation.
 *
 * Generated/integrating Rust code MUST remain:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust
 *
 * Repository Rust policy should continue to enforce:
 *
 *     #![deny(unsafe_code)]
 *
 * where applicable.
 */


/*
 * ============================================================================
 * 41. ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to this grammar.
 *
 * Examples:
 *
 *     capability;
 *     capability nano::;
 *     capability nano::sensing {
 *     requires;
 *     prefer;
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     unknown nano capability;
 *     incompatible capability version;
 *     unsatisfied capability;
 *     conflicting capabilities;
 *     unavailable realization;
 *     unauthorized capability.
 *
 * A hardware/resource shortage must not be reported as a syntax error.
 */


/*
 * ============================================================================
 * 42. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must vary:
 *
 *     capability count;
 *     requirement count;
 *     preference count;
 *     constraint count;
 *     hint count;
 *     property count;
 *     qualified-name depth;
 *     capability-expression size;
 *     version-expression complexity;
 *     nested contract depth where supported;
 *     source-unit size.
 *
 * Tests MUST NOT establish a language maximum.
 *
 * The test harness may use finite test sizes for practical execution, but
 * those sizes are test parameters, not language restrictions.
 */


/*
 * ============================================================================
 * 43. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Examples:
 *
 *     capability nano::sensing;
 *
 *     capability nano::sensing {
 *         requires nano::observation;
 *     }
 *
 *     capability nano::assembly {
 *         requires molecular::assembly;
 *         prefer nano::low_energy;
 *     }
 *
 *     capability nano::quantum {
 *         requires quantum::dynamic_control;
 *     }
 *
 *     capability future::nano::capability {
 *         hint future::implementation;
 *     }
 *
 *     capability nano::distributed {
 *         constraint nano::deterministic;
 *         property mode = distributed;
 *     }
 *
 * ============================================================================
 * 44. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Reject:
 *
 *     capability;
 *
 *     capability nano::;
 *
 *     capability ::nano;
 *
 *     capability nano::sensing {
 *
 *     capability nano::sensing {
 *         requires;
 *     }
 *
 *     capability nano::sensing {
 *         prefer;
 *     }
 *
 *     capability nano::sensing {
 *         constraint;
 *     }
 *
 *     capability nano::sensing {
 *         unknown-property;
 *     }
 *
 * where the final form violates the canonical expression/property grammar.
 */


/*
 * ============================================================================
 * 45. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Verify:
 *
 *     one capability;
 *     many capabilities;
 *     deeply qualified names;
 *     large numeric version components;
 *     long capability expressions;
 *     long property expressions;
 *     nested nano constructs;
 *     capability reuse across domains;
 *     empty contract bodies;
 *     multiple contract members;
 *     arbitrary future capability names.
 *
 * No boundary value becomes a language maximum.
 */


/*
 * ============================================================================
 * 46. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * At minimum test capability references from:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     AI;
 *     data;
 *     networking;
 *     security;
 *     memory;
 *     execution;
 *     interoperability;
 *     nano.
 *
 * The capability identity must remain canonical across all domains.
 */


/*
 * ============================================================================
 * 47. PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same capability source must parse identically regardless of whether
 * eventual realization is:
 *
 *     CPU;
 *     multicore;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     simulator;
 *     accelerator;
 *     embedded;
 *     HPC;
 *     cluster;
 *     distributed;
 *     cloud;
 *     future hardware.
 *
 * Hardware availability must never alter parsing.
 */


/*
 * ============================================================================
 * 48. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must remain free of universal resource constants such as:
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
 * It must also avoid fixed physical identities such as:
 *
 *     CPU_0
 *     GPU_0
 *     FPGA_0
 *     QPU_0
 *     NODE_0
 *     DEVICE_0
 *
 * when used as universal language assumptions.
 *
 * Explicit values appearing in expressions remain valid program data.
 */


/*
 * ============================================================================
 * 49. VENDOR / FRAMEWORK NEUTRALITY
 * ============================================================================
 *
 * This file must not enumerate vendor or framework names as language
 * capabilities.
 *
 * Examples that MUST remain outside this grammar:
 *
 *     CUDA
 *     ROCm
 *     TensorRT
 *     vendor GPU models
 *     vendor QPU models
 *     vendor FPGA families
 *     proprietary nano controllers
 *
 * Such integration belongs to:
 *
 *     interoperability;
 *     dialects;
 *     compiler backends;
 *     HAL;
 *     target-specific implementation.
 */


/*
 * ============================================================================
 * 50. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * This file is independently complete when:
 *
 * [x] It is a parser grammar.
 *
 * [x] It uses ZamaniLexer.
 *
 * [x] It imports canonical Capabilities.
 *
 * [x] It does not redefine capability identity.
 *
 * [x] It does not redefine capability versions.
 *
 * [x] It does not define a closed nano capability enumeration.
 *
 * [x] It has no artificial capacity constants.
 *
 * [x] It is target-independent.
 *
 * [x] It distinguishes capability, requirement, preference, constraint,
 *     and hint syntax.
 *
 * [x] It permits open-world future capability names.
 *
 * [x] It does not select physical devices.
 *
 * [x] It does not allocate resources.
 *
 * [x] It does not perform capability discovery.
 *
 * [x] It does not construct IR.
 *
 * [x] It does not create a second quantum IR.
 *
 * [x] It contains no Rust actions.
 *
 * [x] It contains no unsafe implementation.
 *
 * [x] It is deterministic.
 *
 * [x] It defines AST/semantic/IR integration contracts.
 *
 * [x] It defines cross-domain integration contracts.
 *
 * [x] It defines positive/negative/boundary/scalability tests.
 *
 * [x] It preserves POCO-REAF.
 *
 * Remaining repository-level integration work:
 *
 * [ ] Nano domain composition imports NanoCapabilities.
 *
 * [ ] NanoProtocols consumes the canonical capability reference through
 *     NanoCapabilities instead of maintaining an independent capability
 *     identity grammar.
 *
 * [ ] NanoAgents consumes canonical capability references.
 *
 * [ ] NanoAtoms consumes canonical capability references.
 *
 * [ ] NanoMolecules consumes canonical capability references.
 *
 * [ ] NanoMaterials consumes canonical capability references.
 *
 * [ ] Any future nano/interactions.g4 consumes canonical capability
 *     references.
 *
 * [ ] The universal ZamaniParser exposes the nano capability construct through
 *     the nano/domain dispatcher.
 *
 * [ ] AST lowering uses the existing domain-neutral Capability representation.
 *
 * [ ] Semantic capability resolution is implemented.
 *
 * [ ] Capability/resource distinction is preserved.
 *
 * [ ] Capability/security distinction is preserved.
 *
 * [ ] Cross-domain capability compatibility is tested.
 *
 * [ ] Generated parser compiles under Rust 1.97 / 1.97.1 integration.
 *
 * [ ] Repository-wide safe-Rust/no-unsafe policy passes.
 *
 * ============================================================================
 * 51. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * The nano capability layer answers:
 *
 *     WHAT capability is relevant to this nano computation?
 *
 * It does NOT answer:
 *
 *     WHICH physical device?
 *     WHICH atom instance?
 *     WHICH molecule instance?
 *     WHICH material sample?
 *     WHICH sensor?
 *     WHICH actuator?
 *     WHICH CPU?
 *     WHICH GPU?
 *     WHICH FPGA?
 *     WHICH ASIC?
 *     WHICH QPU?
 *     WHICH node?
 *     WHICH memory bank?
 *     WHICH scheduler?
 *     WHICH router?
 *     WHICH calibration?
 *
 * Therefore the architectural flow remains:
 *
 *     Nano source
 *          |
 *          v
 *     canonical capability syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic capability resolution
 *          |
 *          v
 *     resource / capability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *     classical semantics   quantum::ir
 *          |                  |
 *          +---------+--------+
 *                    |
 *                    v
 *            optimization/lowering
 *                    |
 *             routing/scheduling
 *                    |
 *              resilience/QEC
 *                    |
 *                   ZQN
 *                    |
 *                   HAL
 *                    |
 *             target realization
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */