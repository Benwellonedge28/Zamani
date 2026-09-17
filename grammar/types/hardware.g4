/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/types/hardware.g4
 *
 * Grammar:
 *     ZamaniHardwareTypesParser
 *
 * Status:
 *     Production-ready modular hardware-type grammar.
 *
 * Purpose:
 *     Defines SOURCE-LEVEL HARDWARE TYPES.
 *
 * This file is part of the canonical type-system composition:
 *
 *     grammar/types/types.g4
 *
 * It defines hardware abstractions that may describe:
 *
 *     - compute resources;
 *     - memory resources;
 *     - accelerators;
 *     - processors;
 *     - devices;
 *     - interconnects;
 *     - interfaces;
 *     - ports;
 *     - capabilities;
 *     - resources;
 *     - topology abstractions;
 *     - timing abstractions;
 *     - power/thermal/reliability abstractions;
 *     - quantum hardware resources;
 *     - classical hardware resources;
 *     - HDL/co-design resources;
 *     - heterogeneous hardware;
 *     - target-independent hardware requirements.
 *
 * It DOES NOT describe:
 *
 *     - physical device allocation;
 *     - physical device IDs;
 *     - physical addresses;
 *     - physical qubit IDs;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC implementation;
 *     - ZQN implementation;
 *     - HAL implementation;
 *     - vendor ABI;
 *     - compiler backend selection;
 *     - runtime behavior.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     hardware type constructors
 *     hardware type arguments
 *     hardware resource types
 *     hardware capability types
 *     hardware device-class types
 *     hardware compute types
 *     hardware memory types
 *     hardware accelerator types
 *     hardware interconnect types
 *     hardware interface types
 *     hardware topology types
 *     hardware timing types
 *     hardware power types
 *     hardware thermal types
 *     hardware reliability types
 *     hardware quantum-resource types
 *     hardware parameterization
 *     hardware type constraints
 *
 * THIS FILE DOES NOT OWN:
 *
 *     hardware declarations
 *     target declarations
 *     physical placement
 *     physical mappings
 *     device discovery
 *     resource allocation
 *     routing
 *     scheduling
 *     optimization
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * Those belong to their existing domains under:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *     grammar/quantum/
 *     grammar/compile/
 *     grammar/execution/
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Hardware types describe WHAT a program requires or can operate upon.
 *
 * They MUST NOT encode a fixed implementation capacity.
 *
 * Therefore this grammar contains no:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_PORTS
 *     MAX_LANES
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TOPOLOGY_SIZE
 *
 * A source program may express:
 *
 *     Hardware<...>
 *     Compute<...>
 *     Memory<...>
 *     Accelerator<...>
 *     Capability<...>
 *     Resource<...>
 *     QuantumDevice<...>
 *
 * with symbolic or runtime-dependent parameters.
 *
 * Example:
 *
 *     Hardware<Compute<Cores>>
 *
 * does not mean that a machine has a particular number of cores.
 *
 * Likewise:
 *
 *     Memory<byte, RequiredMemory>
 *
 * describes a semantic memory requirement/type relationship.
 *
 * Actual availability is resolved downstream.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * These are valid source-level concepts:
 *
 *     Cpu
 *     Gpu
 *     Fpga
 *     Asic
 *     Qpu
 *     Accelerator
 *     Memory
 *     Interconnect
 *
 * but this grammar MUST NOT interpret:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     node0
 *     device0
 *
 * as physical resources.
 *
 * They remain names/data handled by the semantic and deployment layers.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum hardware types describe hardware capabilities and resources only.
 *
 * They do NOT define quantum operations or quantum semantics.
 *
 * Quantum semantic lowering remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     quantum semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar MUST NOT create another quantum IR.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Expected semantic AST categories:
 *
 *     hardwareType
 *         -> TypeExpr::Hardware
 *
 *     hardwareComputeType
 *         -> hardware type semantic node
 *
 *     hardwareMemoryType
 *         -> hardware/resource semantic node
 *
 *     hardwareCapabilityType
 *         -> capability type semantic node
 *
 *     hardwareResourceType
 *         -> resource type semantic node
 *
 *     hardwareQuantumType
 *         -> quantum hardware semantic node
 *
 * Exact Rust representation remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar does not define Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntax answers:
 *
 *     "What hardware type did the programmer write?"
 *
 * Semantic analysis answers:
 *
 *     "What does that hardware type require, provide, or represent?"
 *
 * A hardware type MUST therefore remain distinct from:
 *
 *     resource requirement
 *     capability
 *     preference
 *     constraint
 *     implementation decision
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Rust implementation baseline:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * This grammar contains no Rust actions.
 *
 * Compiler/runtime implementation must remain safe Rust.
 *
 * No `unsafe` is required or permitted by this architecture.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareTypesParser;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `hardwareType` is the only public type-level hardware entry point.
 *
 * `types.g4` consumes this rule.
 *
 * `hardware.g4` MUST NOT import this grammar merely to parse declarations.
 *
 * ============================================================================
 */

hardwareType
    : hardwareTypeQualifier*
      hardwareTypePrimary
      hardwareTypeArgumentList?
      hardwareTypePostfix*
    ;


/* ============================================================================
 * 2. QUALIFIERS
 * ============================================================================
 *
 * Qualifiers describe semantic ownership/resource properties.
 *
 * They do not select physical hardware.
 * ============================================================================
 */

hardwareTypeQualifier
    : LINEAR
    | AFFINE
    ;


/* ============================================================================
 * 3. PRIMARY HARDWARE TYPES
 * ============================================================================
 */

hardwareTypePrimary
    : hardwareNamedType
    | hardwareComputeType
    | hardwareMemoryType
    | hardwareAcceleratorType
    | hardwareDeviceType
    | hardwareInterfaceType
    | hardwarePortType
    | hardwareInterconnectType
    | hardwareTopologyType
    | hardwareCapabilityType
    | hardwareResourceType
    | hardwareTimingType
    | hardwarePowerType
    | hardwareThermalType
    | hardwareReliabilityType
    | hardwareQuantumType
    | hardwareHeterogeneousType
    | hardwareParameterizedType
    | hardwareTypeConstraint
    | hardwareParenthesizedType
    ;


/* ============================================================================
 * 4. POSTFIXES
 * ============================================================================
 *
 * Hardware types can participate in the universal optional/type composition.
 * ============================================================================
 */

hardwareTypePostfix
    : QUESTION_MARK
    ;


/* ============================================================================
 * 5. NAMED HARDWARE TYPES
 * ============================================================================
 *
 * Open-ended names are essential for future architectures.
 *
 * Do NOT create:
 *
 *     CpuType
 *     NvidiaGpuType
 *     IbmQpuType
 *     XilinxFpgaType
 *
 * as an exhaustive universal grammar.
 *
 * Vendor and future architecture names remain semantic names.
 * ============================================================================
 */

hardwareNamedType
    : hardwareQualifiedName
    ;


hardwareQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/* ============================================================================
 * 6. GENERIC HARDWARE TYPE ARGUMENTS
 * ============================================================================
 *
 * Generic arguments may contain:
 *
 *     types
 *     symbolic values
 *     expressions
 *
 * This allows scaling without embedding implementation limits.
 * ============================================================================
 */

hardwareTypeArgumentList
    : LESS_THAN
      hardwareTypeArgument
      (
          COMMA
          hardwareTypeArgument
      )*
      COMMA?
      GREATER_THAN
    ;


hardwareTypeArgument
    : hardwareType
    | hardwareTypeValueExpression
    ;


/* ============================================================================
 * 7. COMPUTE TYPES
 * ============================================================================
 *
 * Compute describes an abstract computational resource.
 *
 * It does not select a physical processor.
 * ============================================================================
 */

hardwareComputeType
    : COMPUTE
      (
          hardwareTypeArgumentList
      )?
    ;


/*
 * Parameterized compute classes remain open.
 *
 * Examples:
 *
 *     Compute
 *     Compute<Cores>
 *     Compute<parallelism>
 *     Compute<Capability>
 *
 * Semantic analysis determines the meaning.
 */


/* ============================================================================
 * 8. MEMORY TYPES
 * ============================================================================
 *
 * Memory describes an abstract memory resource.
 *
 * Capacity is a value expression, not a grammar constant.
 * ============================================================================
 */

hardwareMemoryType
    : MEMORY
      (
          LESS_THAN
          hardwareMemoryTypeArgumentList
          GREATER_THAN
      )?
    ;


hardwareMemoryTypeArgumentList
    : hardwareTypeArgument
      (
          COMMA
          hardwareTypeArgument
      )*
      COMMA?
    ;


/* ============================================================================
 * 9. ACCELERATOR TYPES
 * ============================================================================
 */

hardwareAcceleratorType
    : ACCELERATOR
      (
          hardwareTypeArgumentList
      )?
    ;


/* ============================================================================
 * 10. DEVICE TYPES
 * ============================================================================
 *
 * Device is intentionally abstract.
 *
 * A Device does not mean a particular physical device.
 * ============================================================================
 */

hardwareDeviceType
    : DEVICE
      (
          hardwareTypeArgumentList
      )?
    ;


/* ============================================================================
 * 11. INTERFACE TYPES
 * ============================================================================
 */

hardwareInterfaceType
    : INTERFACE
      (
          hardwareTypeArgumentList
      )?
    ;


/* ============================================================================
 * 12. PORT TYPES
 * ============================================================================
 *
 * Port types represent logical interfaces.
 *
 * Physical pin assignment is downstream.
 * ============================================================================
 */

hardwarePortType
    : PORT
      (
          LESS_THAN
          hardwarePortTypeArguments
          GREATER_THAN
      )?
    ;


hardwarePortTypeArguments
    : hardwareTypeArgument
      (
          COMMA
          hardwareTypeArgument
      )*
      COMMA?
    ;


/* ============================================================================
 * 13. INTERCONNECT TYPES
 * ============================================================================
 */

hardwareInterconnectType
    : INTERCONNECT
      (
          hardwareTypeArgumentList
      )?
    ;


/* ============================================================================
 * 14. TOPOLOGY TYPES
 * ============================================================================
 *
 * Topology here is a TYPE/PROPERTY abstraction.
 *
 * It does not encode a physical topology instance.
 * ============================================================================
 */

hardwareTopologyType
    : TOPOLOGY
      (
          hardwareTypeArgumentList
      )?
    ;


/* ============================================================================
 * 15. CAPABILITY TYPES
 * ============================================================================
 *
 * Capability<T> expresses a type-level capability relationship.
 *
 * It does not perform capability discovery.
 * ============================================================================
 */

hardwareCapabilityType
    : CAPABILITY
      LESS_THAN
      hardwareCapabilityArgument
      GREATER_THAN
    ;


hardwareCapabilityArgument
    : hardwareQualifiedName
    | hardwareType
    | hardwareTypeValueExpression
    ;


/* ============================================================================
 * 16. RESOURCE TYPES
 * ============================================================================
 *
 * Resource<T> represents an abstract resource type.
 *
 * It does not allocate the resource.
 * ============================================================================
 */

hardwareResourceType
    : RESOURCE
      LESS_THAN
      hardwareResourceArgument
      GREATER_THAN
    ;


hardwareResourceArgument
    : hardwareType
    | hardwareQualifiedName
    | hardwareTypeValueExpression
    ;


/* ============================================================================
 * 17. TIMING TYPES
 * ============================================================================
 */

hardwareTimingType
    : TIMING
      (
          hardwareTypeArgumentList
      )?
    ;


/* ============================================================================
 * 18. POWER TYPES
 * ============================================================================
 */

hardwarePowerType
    : POWER
      (
          hardwareTypeArgumentList
      )?
    ;


/* ============================================================================
 * 19. THERMAL TYPES
 * ============================================================================
 */

hardwareThermalType
    : THERMAL
      (
          hardwareTypeArgumentList
      )?
    ;


/* ============================================================================
 * 20. RELIABILITY TYPES
 * ============================================================================
 */

hardwareReliabilityType
    : RELIABILITY
      (
          hardwareTypeArgumentList
      )?
    ;


/* ============================================================================
 * 21. QUANTUM HARDWARE TYPES
 * ============================================================================
 *
 * These are hardware abstractions, NOT quantum operation syntax.
 *
 * Examples:
 *
 *     Qpu
 *     Qpu<QubitCapacity>
 *     QuantumDevice
 *     QuantumDevice<Capability>
 *     QuantumResource<QubitRequirement>
 *
 * No physical qubit count is hard-coded.
 * ============================================================================
 */

hardwareQuantumType
    : QPU
      (
          hardwareTypeArgumentList
      )?
    | QUANTUM_DEVICE
      (
          hardwareTypeArgumentList
      )?
    | QUANTUM_RESOURCE
      LESS_THAN
      hardwareTypeArgument
      GREATER_THAN
    ;


/* ============================================================================
 * 22. HETEROGENEOUS HARDWARE
 * ============================================================================
 *
 * A heterogeneous type can represent a composition of multiple abstract
 * hardware classes.
 *
 * Example:
 *
 *     Heterogeneous<CPU, GPU, QPU>
 *
 * The list has no fixed cardinality.
 * ============================================================================
 */

hardwareHeterogeneousType
    : HETEROGENEOUS
      LESS_THAN
      hardwareHeterogeneousMember
      (
          COMMA
          hardwareHeterogeneousMember
      )*
      COMMA?
      GREATER_THAN
    ;


hardwareHeterogeneousMember
    : hardwareType
    | hardwareQualifiedName
    ;


/* ============================================================================
 * 23. GENERAL PARAMETERIZED HARDWARE TYPE
 * ============================================================================
 *
 * This provides future-proof extension without adding a keyword for every
 * emerging architecture.
 *
 * Example:
 *
 *     Hardware<custom::Architecture, Parameter>
 * ============================================================================
 */

hardwareParameterizedType
    : HARDWARE
      LESS_THAN
      hardwareTypeArgument
      (
          COMMA
          hardwareTypeArgument
      )*
      COMMA?
      GREATER_THAN
    ;


/* ============================================================================
 * 24. TYPE CONSTRAINTS
 * ============================================================================
 *
 * A type constraint describes a relationship between type-level values.
 *
 * It does not perform target validation.
 * ============================================================================
 */

hardwareTypeConstraint
    : HARDWARE_CONSTRAINT
      hardwareTypeConstraintExpression
    ;


hardwareTypeConstraintExpression
    : hardwareTypeConstraintOperand
      (
          hardwareTypeConstraintOperator
          hardwareTypeConstraintOperand
      )*
    ;


hardwareTypeConstraintOperand
    : hardwareQualifiedName
    | hardwareTypeValueExpression
    | hardwareType
    | LPAREN
      hardwareTypeConstraintExpression
      RPAREN
    ;


hardwareTypeConstraintOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/* ============================================================================
 * 25. PARENTHESIZED HARDWARE TYPE
 * ============================================================================
 */

hardwareParenthesizedType
    : LPAREN
      hardwareType
      RPAREN
    ;


/* ============================================================================
 * 26. TYPE-LEVEL VALUE EXPRESSIONS
 * ============================================================================
 *
 * Values may remain symbolic.
 *
 * Examples:
 *
 *     N
 *     required_memory
 *     cores * lanes
 *     workload_size
 *     available_capacity
 *
 * No finite numeric range is established by this grammar.
 * ============================================================================
 */

hardwareTypeValueExpression
    : hardwareTypeValueUnary*
      hardwareTypeValuePrimary
      hardwareTypeValueBinaryPart*
    ;


hardwareTypeValueUnary
    : PLUS
    | MINUS
    ;


hardwareTypeValueBinaryPart
    : hardwareTypeValueOperator
      hardwareTypeValueUnary*
      hardwareTypeValuePrimary
    ;


hardwareTypeValueOperator
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


hardwareTypeValuePrimary
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | IDENTIFIER
    | hardwareQualifiedName
    | LPAREN
      hardwareTypeValueExpression
      RPAREN
    ;


/* ============================================================================
 * 27. SEMANTIC HARDWARE TYPE ALIASES
 * ============================================================================
 *
 * These aliases deliberately use universal hardware categories.
 *
 * They are syntax-level names only.
 * ============================================================================
 */

hardwareProcessorType
    : CPU
      (
          hardwareTypeArgumentList
      )?
    | GPU
      (
          hardwareTypeArgumentList
      )?
    | FPGA
      (
          hardwareTypeArgumentList
      )?
    | ASIC
      (
          hardwareTypeArgumentList
      )?
    ;


hardwareComputeDeviceType
    : hardwareProcessorType
    | hardwareAcceleratorType
    | hardwareQuantumType
    | hardwareDeviceType
    ;


/* ============================================================================
 * 28. OPTIONAL HARDWARE INTERFACE TYPE
 * ============================================================================
 *
 * Logical interface properties can be parameterized without encoding physical
 * pins or buses.
 * ============================================================================
 */

hardwareInterfaceContractType
    : INTERFACE
      LESS_THAN
      hardwareInterfaceContractArgument
      (
          COMMA
          hardwareInterfaceContractArgument
      )*
      COMMA?
      GREATER_THAN
    ;


hardwareInterfaceContractArgument
    : hardwareQualifiedName
    | hardwareType
    | hardwareTypeValueExpression
    ;


/* ============================================================================
 * 29. HARDWARE RESOURCE VECTOR TYPE
 * ============================================================================
 *
 * A resource vector represents a collection of resource requirements.
 *
 * It does not impose a fixed vector length.
 * ============================================================================
 */

hardwareResourceVectorType
    : RESOURCE_VECTOR
      (
          LESS_THAN
          hardwareResourceVectorArgumentList
          GREATER_THAN
      )?
    ;


hardwareResourceVectorArgumentList
    : hardwareTypeArgument
      (
          COMMA
          hardwareTypeArgument
      )*
      COMMA?
    ;


/* ============================================================================
 * 30. HARDWARE CAPABILITY SET TYPE
 * ============================================================================
 */

hardwareCapabilitySetType
    : CAPABILITY_SET
      (
          LESS_THAN
          hardwareCapabilitySetArgumentList
          GREATER_THAN
      )?
    ;


hardwareCapabilitySetArgumentList
    : hardwareCapabilityArgument
      (
          COMMA
          hardwareCapabilityArgument
      )*
      COMMA?
    ;


/* ============================================================================
 * 31. HARDWARE PROPERTY TYPE
 * ============================================================================
 *
 * Generic property types provide future extensibility without permanently
 * expanding the universal keyword set.
 * ============================================================================
 */

hardwarePropertyType
    : HARDWARE_PROPERTY
      LESS_THAN
      hardwareQualifiedName
      GREATER_THAN
    ;


/* ============================================================================
 * 32. TYPE-SYSTEM INTEGRATION CONTRACT
 * ============================================================================
 *
 * `grammar/types/types.g4` should compose this grammar through:
 *
 *     hardwareType
 *
 * and must not create another hardware-type rule with competing semantics.
 *
 * The integration relationship is:
 *
 *     Types.typeExpression
 *             |
 *             +--> hardwareType
 *                       |
 *                       +--> compute
 *                       +--> memory
 *                       +--> accelerator
 *                       +--> device
 *                       +--> interface
 *                       +--> resource
 *                       +--> capability
 *                       +--> quantum hardware
 *                       +--> heterogeneous
 *                       +--> parameterized
 *
 * The resulting AST remains the repository's domain-neutral TypeExpr.
 *
 * ============================================================================
 * RESOURCE/CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Hardware types may be referenced by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * but this file does not import their declaration grammars.
 *
 * This prevents cyclic grammar dependencies.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL syntax remains under:
 *
 *     grammar/hdl/
 *
 * Hardware types can describe the type-level contract of HDL/software
 * co-design entities, but must not reproduce:
 *
 *     always
 *     assign
 *     clocked process
 *     signal behavior
 *     synthesis behavior
 *
 * Those remain HDL semantics.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum hardware types may be consumed by the quantum and hybrid domains.
 *
 * For example:
 *
 *     QuantumResource<QubitRequirement>
 *
 * can express resource intent.
 *
 * It does NOT select:
 *
 *     physical qubit 0
 *     physical qubit 1
 *     coupling map
 *     calibration
 *     gate duration
 *
 * Those belong downstream.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Hardware type semantics are consumed by:
 *
 *     semantic analysis
 *     capability resolution
 *     resource analysis
 *     target analysis
 *     optimization
 *     lowering
 *     scheduling
 *     routing
 *     HAL
 *
 * This grammar does not invoke any of those systems.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For identical:
 *
 *     source
 *     language version
 *     lexical configuration
 *
 * this grammar must produce the same parse structure.
 *
 * It must not depend on:
 *
 *     CPU count
 *     GPU count
 *     QPU availability
 *     memory capacity
 *     filesystem state
 *     network state
 *     scheduler state
 *     calibration state
 *     runtime state
 *     random state
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_CPU
 *     MAX_GPU
 *     MAX_QPU
 *     MAX_MEMORY
 *     MAX_PORTS
 *     MAX_LANES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *
 * Also forbidden:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     physical_qubit_0
 *
 * as special grammar productions.
 *
 * Such names may occur as ordinary identifiers in source programs.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive examples must include:
 *
 *     Hardware<CPU>
 *     Hardware<GPU>
 *     Hardware<QPU>
 *     Compute
 *     Compute<N>
 *     Memory<byte, RequiredMemory>
 *     Accelerator<TensorCompute>
 *     Resource<Memory>
 *     Capability<quantum::measurement>
 *     QPU<RequiredQubits>
 *     QuantumDevice<Capability>
 *     Heterogeneous<CPU, GPU, QPU>
 *     custom::FutureHardware<Parameter>
 *
 * Negative examples must include:
 *
 *     Hardware<>
 *     Resource<>
 *     Capability<>
 *     Heterogeneous<>
 *
 * and malformed generic lists.
 *
 * Boundary tests must include:
 *
 *     deeply qualified names
 *     symbolic dimensions
 *     large literal values
 *     arbitrarily many type arguments
 *     arbitrarily many heterogeneous members
 *     nested hardware types
 *     nested resource/capability types
 *
 * Scalability tests must verify that the grammar itself introduces no finite
 * hardware capacity.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] canonical lexical tokens exist;
 *     [ ] types.g4 consumes hardwareType;
 *     [ ] no competing hardware typeExpression exists;
 *     [ ] AST mapping is defined;
 *     [ ] semantic mapping is defined;
 *     [ ] resource/capability mapping is defined;
 *     [ ] quantum mapping terminates at quantum::ir downstream;
 *     [ ] HDL mapping remains outside this grammar;
 *     [ ] no physical hardware IDs are special syntax;
 *     [ ] no machine limits exist;
 *     [ ] no vendor implementation is hard-coded;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] lexer/parser integration is deterministic;
 *     [ ] Rust integration remains safe Rust 1.97/1.97.1;
 *     [ ] no unsafe code or embedded actions exist.
 *
 * ============================================================================
 */