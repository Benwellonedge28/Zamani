/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/hardware.g4
 *
 * Status:
 *     Production-ready modular parser grammar for hardware-related effects.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no unsafe code;
 *       - no filesystem access;
 *       - no network access;
 *       - no hardware discovery;
 *       - no runtime calls;
 *       - no target-specific execution logic.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE SYNTAX for hardware-related computational effects.
 *
 * It allows Zamani programs to express that an operation:
 *
 *     - interacts with hardware;
 *     - requires a hardware capability;
 *     - produces a hardware-related effect;
 *     - requires a class of hardware behavior;
 *     - optionally describes hardware-independent realization constraints;
 *     - distinguishes requirements from preferences and hints;
 *     - associates hardware effects with effect operations.
 *
 * It deliberately does NOT select or identify physical hardware.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       ZamaniTokens
 *                              |
 *                              v
 *                     parser grammars
 *                              |
 *                              v
 *                  hardware effect syntax
 *                              |
 *                              v
 *                         frontend AST
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       effect analysis  capability analysis  resource analysis
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                  canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       classical IR      quantum::ir       HDL/hardware IR
 *                              |
 *                              v
 *                   optimization / routing /
 *                   scheduling / resilience
 *                              |
 *                              v
 *                       target lowering
 *                              |
 *                              v
 *                    hardware abstraction
 *                              |
 *                              v
 *                           runtime
 *
 * The grammar is syntax-only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hardware-effect clauses;
 *     - hardware-effect declarations;
 *     - hardware-effect operation requirements;
 *     - hardware-effect requirement expressions;
 *     - hardware-effect capability references at the effect boundary;
 *     - hardware-effect requirement groups;
 *     - hardware-effect preferences;
 *     - hardware-effect hints;
 *     - hardware-effect semantic annotations;
 *     - hardware-effect realization intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general effect declarations;
 *     - general effect sets;
 *     - capability declarations;
 *     - capability identity definitions;
 *     - resource declarations;
 *     - resource allocation;
 *     - hardware device declarations;
 *     - hardware topology;
 *     - hardware placement;
 *     - physical addresses;
 *     - device IDs;
 *     - CPU IDs;
 *     - GPU IDs;
 *     - FPGA IDs;
 *     - ASIC IDs;
 *     - QPU IDs;
 *     - qubit IDs;
 *     - qubit counts;
 *     - core counts;
 *     - thread counts;
 *     - memory capacities;
 *     - accelerator counts;
 *     - calibration;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - hardware discovery;
 *     - runtime dispatch;
 *     - backend selection;
 *     - resilience.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * Hardware effects describe COMPUTATIONAL INTENT.
 *
 * They do not describe an implementation-specific machine.
 *
 * Therefore this file must remain valid for:
 *
 *     - a microscopic device;
 *     - an embedded processor;
 *     - a CPU;
 *     - a multicore CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum processor;
 *     - a simulator;
 *     - an accelerator;
 *     - a heterogeneous system;
 *     - a distributed system;
 *     - a cluster;
 *     - a supercomputer;
 *     - a cloud system;
 *     - future computational substrates.
 *
 * No source-level hardware capacity is encoded.
 *
 * In particular, this grammar MUST NOT contain:
 *
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *
 * or equivalent fixed machine limits.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * Hardware effect:
 *
 *     What hardware-related interaction is part of the computation?
 *
 * Capability:
 *
 *     What can an execution environment provide?
 *
 * Resource:
 *
 *     What resource may be required or consumed?
 *
 * Constraint:
 *
 *     What conditions must a realization satisfy?
 *
 * Preference:
 *
 *     Which valid realization is preferred?
 *
 * Hint:
 *
 *     Which implementation strategy may be beneficial without changing
 *     semantic correctness?
 *
 * These concepts must remain distinct.
 *
 * Example:
 *
 *     hardware effect "accelerator::compute"
 *
 * MUST NOT mean:
 *
 *     use GPU 0
 *     use 4 GPUs
 *     use 64 cores
 *     use device X
 *     use PCI address Y
 *     use topology Z
 *
 * Such decisions belong downstream.
 *
 * ============================================================================
 * OPEN-WORLD HARDWARE MODEL
 * ============================================================================
 *
 * Hardware domains are open-ended.
 *
 * The grammar therefore does NOT enumerate:
 *
 *     cpu
 *     gpu
 *     fpga
 *     asic
 *     qpu
 *     dsp
 *     tpu
 *     neuromorphic
 *     photonic
 *
 * as a closed set.
 *
 * They are represented through ordinary qualified names.
 *
 * Examples:
 *
 *     hardware::compute
 *     hardware::memory
 *     hardware::clock
 *     accelerator::tensor
 *     accelerator::vector
 *     quantum::control
 *     fpga::reconfiguration
 *     future::photonic::operation
 *     custom::hardware::effect
 *
 * New hardware domains therefore do not require grammar modification.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     device identity;
 *     backend identity;
 *     vendor identity;
 *     physical location;
 *     topology;
 *     machine size;
 *     deployment location.
 *
 * If a target-specific fact is required, the semantic/compiler layers resolve
 * it through:
 *
 *     capability model;
 *     resource model;
 *     target description;
 *     compilation context;
 *     hardware abstraction layer;
 *     runtime discovery.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Hardware effects may reference quantum capabilities:
 *
 *     quantum::measurement
 *     quantum::reset
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *
 * This file does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     calibration
 *     QEC
 *     ZQN
 *     quantum::ir
 *
 * If the effect eventually requires quantum semantics, downstream semantic
 * lowering may produce the canonical:
 *
 *     quantum::ir
 *
 * representation.
 *
 * This grammar never constructs quantum::ir directly.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Hardware effects may refer to HDL-related computational capabilities:
 *
 *     hardware::clocked_logic
 *     hardware::sequential_logic
 *     hardware::combinational_logic
 *     hardware::pipeline
 *     hardware::reconfiguration
 *
 * This file does not define:
 *
 *     ports;
 *     wires;
 *     registers;
 *     clocks;
 *     processes;
 *     state machines;
 *     RTL;
 *     synthesis;
 *     placement;
 *     routing.
 *
 * Those belong to the HDL/hardware grammar and compiler layers.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - has no semantic predicates;
 *     - has no actions;
 *     - has no external state;
 *     - performs no I/O;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no runtime dispatch;
 *     - performs no random operations.
 *
 * Given the same token stream, parsing is deterministic.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Core supplies:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     whereClause
 *
 * Types supplies:
 *
 *     typeExpression
 *     parameterList
 *
 * Expressions supplies:
 *
 *     expression
 *     argumentList
 *     blockExpression
 *
 * Capabilities supplies:
 *
 *     capabilityReference
 *     capabilityVersionClause
 *
 * This file MUST NOT redefine those concepts.
 *
 * ============================================================================
 */

parser grammar HardwareEffect;

options {
    tokenVocab = ZamaniTokens;
}

import Core, Types, Expressions, Capabilities;


/*
 * ============================================================================
 * 1. HARDWARE EFFECT DECLARATION
 * ============================================================================
 *
 * Hardware effects are effect declarations whose semantic domain is associated
 * with hardware interaction.
 *
 * The hardware domain remains open-ended.
 *
 * Examples:
 *
 *     hardwareEffect hardware::compute;
 *
 *     hardwareEffect accelerator::tensor;
 *
 *     hardwareEffect quantum::control;
 *
 *     hardwareEffect future::photonic::operation;
 *
 * The parser records the name.
 *
 * Semantic analysis determines whether that name denotes a valid hardware
 * effect and what it means.
 */

hardwareEffectDeclaration
    : attributes?
      visibility?
      HARDWARE_EFFECT
      qualifiedName
      genericParameters?
      hardwareEffectSignature?
      hardwareEffectBody?
      SEMI?
    ;


/*
 * ============================================================================
 * 2. HARDWARE EFFECT SIGNATURE
 * ============================================================================
 */

hardwareEffectSignature
    : LPAREN
      parameterList?
      RPAREN
      returnType?
    ;


/*
 * ============================================================================
 * 3. HARDWARE EFFECT BODY
 * ============================================================================
 *
 * The body may contain an arbitrary number of operations.
 *
 * No operation-count limit is encoded.
 */

hardwareEffectBody
    : LBRACE
      hardwareEffectOperation*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. HARDWARE EFFECT OPERATION
 * ============================================================================
 *
 * An operation describes one hardware-related computational interaction.
 *
 * Example:
 *
 *     hardwareEffect accelerator::compute {
 *         fn execute(input: Tensor) -> Tensor;
 *     }
 *
 * The declaration is not an implementation.
 */

hardwareEffectOperation
    : attributes?
      visibility?
      ASYNC?
      FN
      identifier
      genericParameters?
      LPAREN
      parameterList?
      RPAREN
      returnType?
      hardwareEffectRequirements?
      whereClause?
      SEMI?
    ;


/*
 * ============================================================================
 * 5. HARDWARE EFFECT REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe semantic conditions for realizing the operation.
 *
 * They do not select hardware.
 */

hardwareEffectRequirements
    : REQUIRES
      hardwareRequirementExpression
    ;


/*
 * ============================================================================
 * 6. HARDWARE REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * The expression supports:
 *
 *     conjunction;
 *     alternatives;
 *     grouping;
 *     capability requirements;
 *     named hardware properties.
 *
 * The grammar deliberately does not decide satisfiability.
 */

hardwareRequirementExpression
    : hardwareRequirementOr
    ;


hardwareRequirementOr
    : hardwareRequirementAnd
      (
          OR
          hardwareRequirementAnd
      )*
    ;


hardwareRequirementAnd
    : hardwareRequirementPrimary
      (
          AND
          hardwareRequirementPrimary
      )*
    ;


hardwareRequirementPrimary
    : hardwareCapabilityRequirement
    | hardwarePropertyRequirement
    | hardwareRequirementGroup
    ;


hardwareRequirementGroup
    : LPAREN
      hardwareRequirementExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 7. HARDWARE CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Capability identity is delegated to the canonical capability grammar.
 *
 * Examples:
 *
 *     hardware::clocked_logic
 *     hardware::reconfigurable_logic
 *     accelerator::tensor
 *     quantum::dynamic_control
 *     future::photonic::operation
 *
 * No hardware type is enumerated here.
 */

hardwareCapabilityRequirement
    : capabilityReference
      capabilityVersionClause?
    ;


/*
 * ============================================================================
 * 8. HARDWARE PROPERTY REQUIREMENT
 * ============================================================================
 *
 * A property requirement describes a semantic property rather than a
 * particular machine.
 *
 * Example:
 *
 *     hardware::deterministic_execution
 *
 *     hardware::low_latency
 *
 *     hardware::fault_tolerant
 *
 * The property identity is intentionally open-ended.
 *
 * Numeric comparison semantics belong to semantic/resource analysis.
 */

hardwarePropertyRequirement
    : qualifiedName
      hardwarePropertyOperator?
      expression?
    ;


hardwarePropertyOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_EQUAL
    | GREATER_EQUAL
    | LESS_THAN
    | GREATER_THAN
    ;


/*
 * ============================================================================
 * 9. HARDWARE EFFECT CLAUSE
 * ============================================================================
 *
 * Attaches a hardware-effect requirement to an enclosing declaration.
 *
 * Example:
 *
 *     with hardware effects {
 *         hardware::compute,
 *         accelerator::tensor
 *     }
 *
 * The list remains open-ended.
 */

hardwareEffectClause
    : WITH
      HARDWARE_EFFECTS
      LBRACE
      hardwareEffectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 10. HARDWARE EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Trailing commas are accepted.
 *
 * No finite maximum is encoded.
 */

hardwareEffectReferenceList
    : hardwareEffectReference
      (
          COMMA
          hardwareEffectReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. HARDWARE EFFECT REFERENCE
 * ============================================================================
 *
 * A reference identifies an effect without declaring it.
 *
 * It does not imply:
 *
 *     device;
 *     backend;
 *     topology;
 *     resource quantity;
 *     placement.
 */

hardwareEffectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 12. HARDWARE EFFECT REQUIREMENT CLAUSE
 * ============================================================================
 *
 * Explicit requirement syntax:
 *
 *     requires hardware effects {
 *         hardware::compute,
 *         accelerator::tensor,
 *     }
 *
 * This expresses semantic requirements only.
 */

hardwareEffectRequirementClause
    : REQUIRES
      HARDWARE_EFFECTS
      LBRACE
      hardwareEffectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 13. HARDWARE EFFECT PREFERENCE
 * ============================================================================
 *
 * Preferences do not affect semantic correctness.
 *
 * A compiler may ignore a preference when no suitable implementation exists.
 */

hardwareEffectPreference
    : PREFER
      hardwarePreferenceExpression
    ;


hardwarePreferenceExpression
    : hardwarePreferenceTerm
      (
          COMMA
          hardwarePreferenceTerm
      )*
      COMMA?
    ;


hardwarePreferenceTerm
    : qualifiedName
      (
          EQUAL_EQUAL
          expression
      )?
    ;


/*
 * ============================================================================
 * 14. HARDWARE EFFECT HINT
 * ============================================================================
 *
 * Hints are advisory implementation information.
 *
 * A hint MUST NOT change program semantics.
 */

hardwareEffectHint
    : HINT
      hardwareHintExpression
    ;


hardwareHintExpression
    : qualifiedName
      (
          EQUAL_EQUAL
          expression
      )?
    ;


/*
 * ============================================================================
 * 15. HARDWARE EFFECT POLICY
 * ============================================================================
 *
 * Policy groups provide a stable syntax boundary for source-level hardware
 * realization intent.
 *
 * Example:
 *
 *     hardware policy {
 *         requires hardware effects {
 *             accelerator::tensor
 *         }
 *
 *         prefer accelerator::vector;
 *
 *         hint hardware::parallel;
 *     }
 *
 * This does not perform target selection.
 */

hardwareEffectPolicy
    : HARDWARE_POLICY
      LBRACE
      hardwareEffectPolicyItem*
      RBRACE
    ;


hardwareEffectPolicyItem
    : hardwareEffectRequirementClause
    | hardwareEffectPreference
    | hardwareEffectHint
    ;


/*
 * ============================================================================
 * 16. HARDWARE EFFECT ANNOTATION
 * ============================================================================
 *
 * An annotation provides metadata without defining execution semantics.
 *
 * The annotation value remains an expression because metadata interpretation
 * belongs downstream.
 */

hardwareEffectAnnotation
    : AT
      qualifiedName
      (
          LPAREN
          argumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 17. HARDWARE EFFECT CONTRACT
 * ============================================================================
 *
 * A contract groups requirements, preferences and hints.
 *
 * Requirements affect validity.
 * Preferences affect realization choice.
 * Hints are advisory.
 */

hardwareEffectContract
    : HARDWARE_CONTRACT
      LBRACE
      hardwareEffectContractItem*
      RBRACE
    ;


hardwareEffectContractItem
    : hardwareEffectRequirementClause
    | hardwareEffectPreference
    | hardwareEffectHint
    | hardwareEffectAnnotation
    ;


/*
 * ============================================================================
 * 18. HARDWARE EFFECT RESOURCE REFERENCE
 * ============================================================================
 *
 * This is deliberately a NAME reference only.
 *
 * It does not allocate or count resources.
 *
 * Examples:
 *
 *     hardware::compute_unit
 *     accelerator::memory
 *     quantum::logical_qubit
 *     hardware::storage
 *
 * Resource quantities and allocation are owned by the resource subsystem.
 */

hardwareEffectResourceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 19. HARDWARE EFFECT RESOURCE REQUIREMENT
 * ============================================================================
 *
 * Resource requirements may be expressed structurally.
 *
 * Example:
 *
 *     resource hardware::compute_unit
 *
 * The actual resource model determines:
 *
 *     quantity;
 *     capacity;
 *     availability;
 *     allocation;
 *     lifetime;
 *     ownership.
 *
 * No fixed capacity is encoded here.
 */

hardwareEffectResourceRequirement
    : RESOURCE
      hardwareEffectResourceReference
      hardwareResourceQuantity?
    ;


hardwareResourceQuantity
    : expression
    ;


/*
 * ============================================================================
 * 20. HARDWARE EFFECT RESOURCE CLAUSE
 * ============================================================================
 */

hardwareEffectResourceClause
    : REQUIRES
      RESOURCE
      LBRACE
      hardwareEffectResourceRequirement*
      RBRACE
    ;


/*
 * ============================================================================
 * 21. HARDWARE EFFECT TARGET REFERENCE
 * ============================================================================
 *
 * This rule represents a symbolic target requirement.
 *
 * It MUST NOT be interpreted as a physical device identifier by the grammar.
 *
 * Example:
 *
 *     target hardware::accelerator
 *
 * A semantic target resolver determines whether the requested target class
 * exists and which realization can satisfy it.
 */

hardwareEffectTargetReference
    : qualifiedName
    ;


hardwareEffectTargetClause
    : TARGET
      hardwareEffectTargetReference
    ;


/*
 * ============================================================================
 * 22. HARDWARE EFFECT CONSTRAINT
 * ============================================================================
 *
 * Constraints describe conditions a realization must satisfy.
 *
 * Example:
 *
 *     constrain hardware::latency <= value
 *
 * The grammar only records syntax.
 */

hardwareEffectConstraint
    : CONSTRAINT
      hardwareConstraintExpression
    ;


hardwareConstraintExpression
    : qualifiedName
      (
          hardwarePropertyOperator
          expression
      )?
    ;


/*
 * ============================================================================
 * 23. HARDWARE EFFECT REQUIREMENT GROUP
 * ============================================================================
 *
 * Combines the different hardware-intent categories without conflating them.
 *
 * The semantic layer must preserve the distinction between:
 *
 *     requirement
 *     resource
 *     target
 *     constraint
 *     preference
 *     hint
 */

hardwareEffectRequirementGroup
    : HARDWARE_REQUIREMENTS
      LBRACE
      hardwareEffectRequirementItem*
      RBRACE
    ;


hardwareEffectRequirementItem
    : hardwareEffectRequirementClause
    | hardwareEffectResourceClause
    | hardwareEffectTargetClause
    | hardwareEffectConstraint
    | hardwareEffectPreference
    | hardwareEffectHint
    ;


/*
 * ============================================================================
 * 24. HARDWARE EFFECT OPERATION CONTRACT
 * ============================================================================
 *
 * Associates all supported hardware realization metadata with one operation.
 */

hardwareEffectOperationContract
    : HARDWARE_CONTRACT
      LBRACE
      hardwareEffectRequirementItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 25. HARDWARE EFFECT REFERENCE WITH QUALIFICATION
 * ============================================================================
 *
 * Explicit convenience rule for consumers requiring one symbolic hardware
 * effect reference.
 */

singleHardwareEffectReference
    : hardwareEffectReference
    ;


/*
 * ============================================================================
 * 26. HARDWARE EFFECT SET
 * ============================================================================
 *
 * A source-level collection of hardware effect references.
 *
 * The collection is intentionally unbounded at the grammar level.
 */

hardwareEffectSet
    : LBRACE
      hardwareEffectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 27. OPTIONAL HARDWARE EFFECT SET
 * ============================================================================
 */

optionalHardwareEffectSet
    : hardwareEffectSet?
    ;


/*
 * ============================================================================
 * 28. HARDWARE EFFECT BODY ITEM
 * ============================================================================
 *
 * This is the canonical integration point for future hardware-effect body
 * extensions.
 */

hardwareEffectBodyItem
    : hardwareEffectOperation
    | hardwareEffectContract
    | hardwareEffectPolicy
    | hardwareEffectAnnotation
    ;


/*
 * ============================================================================
 * 29. HARDWARE EFFECT DECLARATION BODY
 * ============================================================================
 *
 * Kept separate so future extensions do not require changing the declaration
 * header.
 */

hardwareEffectDeclarationBody
    : LBRACE
      hardwareEffectBodyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 30. HARDWARE EFFECT REQUIREMENT EXPRESSION WITH GROUPING
 * ============================================================================
 *
 * Explicit named boundary for semantic analysis.
 */

hardwareRequirementGroupExpression
    : LPAREN
      hardwareRequirementExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 31. HARDWARE EFFECT CAPABILITY LIST
 * ============================================================================
 *
 * Capability references remain owned by Capabilities.
 */

hardwareCapabilityReferenceList
    : capabilityReference
      (
          COMMA
          capabilityReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 32. HARDWARE EFFECT CAPABILITY CLAUSE
 * ============================================================================
 */

hardwareCapabilityClause
    : REQUIRES
      CAPABILITIES
      LBRACE
      hardwareCapabilityReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 33. HARDWARE EFFECT IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * This is intentionally declarative.
 *
 * It does not contain implementation code.
 */

hardwareImplementationContract
    : IMPLEMENTATION
      HARDWARE_CONTRACT
      LBRACE
      hardwareEffectRequirementItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 34. HARDWARE EFFECT ADAPTER REFERENCE
 * ============================================================================
 *
 * Adapters are symbolic references only.
 *
 * Actual adapters belong to the hardware abstraction/runtime layers.
 */

hardwareAdapterReference
    : qualifiedName
    ;


hardwareAdapterClause
    : ADAPTER
      hardwareAdapterReference
    ;


/*
 * ============================================================================
 * 35. HARDWARE EFFECT EXECUTION MODE
 * ============================================================================
 *
 * Execution mode is a semantic name, not a fixed hardware architecture.
 *
 * Examples:
 *
 *     hardware::local
 *     hardware::remote
 *     hardware::accelerated
 *     hardware::distributed
 *     hardware::simulated
 *
 * The implementation decides how a mode is realized.
 */

hardwareExecutionMode
    : EXECUTION
      qualifiedName
    ;


/*
 * ============================================================================
 * 36. HARDWARE EFFECT PORTABILITY CONTRACT
 * ============================================================================
 *
 * Portability is semantic.
 *
 * A portability declaration does not guarantee that every target supports the
 * requested effect. Capability analysis determines whether a realization is
 * possible.
 */

hardwarePortabilityClause
    : PORTABLE
      hardwarePortabilityExpression
    ;


hardwarePortabilityExpression
    : qualifiedName
      (
          COMMA
          qualifiedName
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 37. HARDWARE EFFECT FAILURE POLICY
 * ============================================================================
 *
 * This syntax describes a declarative failure-policy reference only.
 *
 * It does not implement retries, recovery, rollback, quarantine or backend
 * switching.
 *
 * Those responsibilities remain with resilience/runtime layers.
 */

hardwareFailurePolicyReference
    : FAILURE
      POLICY
      qualifiedName
    ;


/*
 * ============================================================================
 * 38. HARDWARE EFFECT OBSERVABILITY REFERENCE
 * ============================================================================
 *
 * Observability is referenced symbolically.
 *
 * Telemetry collection and interpretation belong downstream.
 */

hardwareObservabilityReference
    : OBSERVABILITY
      qualifiedName
    ;


/*
 * ============================================================================
 * 39. COMPLETE HARDWARE EFFECT SPECIFICATION
 * ============================================================================
 *
 * This rule provides one stable aggregate boundary for parser clients.
 *
 * It intentionally contains only syntax.
 */

hardwareEffectSpecification
    : hardwareEffectDeclaration
    | hardwareEffectClause
    | hardwareEffectRequirementClause
    | hardwareEffectPolicy
    | hardwareEffectContract
    ;


/*
 * ============================================================================
 * 40. INTEGRATION ROOT
 * ============================================================================
 *
 * The aggregate grammar may import this grammar and use:
 *
 *     hardwareEffectSpecification
 *
 * as the hardware-effect entry point.
 *
 * No generated Rust code is embedded here.
 */

hardwareEffect
    : hardwareEffectSpecification
    ;