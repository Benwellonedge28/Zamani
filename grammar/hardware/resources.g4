/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hardware/resources.g4
 *
 * GRAMMAR
 * -------
 * ZamaniHardwareResourcesParser
 *
 * STATUS
 * ------
 * CANONICAL HARDWARE-RESOURCE INTENT LEAF GRAMMAR
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar
 * Rust 2021
 * Rust 1.97 or later
 * Safe Rust only
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no parser actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no allocation;
 *     - no scheduling;
 *     - no routing;
 *     - no runtime execution;
 *     - no unsafe implementation requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the hardware-domain owner for SOURCE-LEVEL HARDWARE RESOURCE
 * INTENT.
 *
 * It describes what a hardware-oriented declaration requires, permits,
 * prefers, exposes, derives, groups, profiles, or constrains.
 *
 * It does NOT select or allocate physical hardware.
 *
 * The central architectural distinction is:
 *
 *     SOURCE INTENT
 *          |
 *          v
 *     semantic resource model
 *          |
 *          v
 *     capability/resource negotiation
 *          |
 *          v
 *     target realization
 *
 * This grammar therefore expresses:
 *
 *     resource kinds
 *     resource references
 *     quantities
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     capabilities
 *     targets
 *     capacities
 *     availability
 *     portability
 *     scalability
 *     performance
 *     latency
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     cost
 *     reservation intent
 *     acquisition intent
 *     release intent
 *     symbolic derivation
 *     resource groups
 *     resource contracts
 *     resource profiles
 *     extensible properties
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     hardwareResourceDeclaration
 *     hardwareResourceKind
 *     hardwareResourceSpecification
 *     hardwareResourceBodyElement
 *     hardwareResourceClause
 *
 *     hardware-specific resource clause composition:
 *
 *         quantity
 *         reference
 *         requirement
 *         constraint
 *         preference
 *         hint
 *         capability
 *         target
 *         capacity
 *         availability
 *         portability
 *         scalability
 *         performance
 *         latency
 *         throughput
 *         bandwidth
 *         energy
 *         power
 *         reliability
 *         resilience
 *         cost
 *         reservation
 *         acquisition
 *         release
 *         derivation
 *         group
 *         contract
 *         profile
 *         property
 *
 * ============================================================================
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical definitions
 *     token spelling
 *     identifier syntax
 *     qualified-name syntax
 *     general expression precedence
 *     general type syntax
 *
 *     universal resource declarations
 *     universal resource expressions
 *     universal capability identity
 *     universal requirement semantics
 *     universal constraint semantics
 *     universal preference semantics
 *     universal hint semantics
 *     resource budgets
 *     resource negotiation
 *     generic policy syntax
 *     generic effects
 *     contracts outside the hardware-resource boundary
 *     provenance
 *
 *     hardware discovery
 *     physical device enumeration
 *     physical device identifiers
 *     physical addresses
 *     physical allocation
 *     placement algorithms
 *     routing
 *     scheduling
 *     optimization
 *     calibration
 *     runtime allocation
 *
 *     quantum operations
 *     quantum state semantics
 *     QEC
 *     ZQN
 *     quantum::ir
 *
 *     HDL behavioral semantics
 *     HDL simulation
 *     HDL synthesis
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Canonical resource expressions:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * Canonical names:
 *
 *     grammar/core/names.g4
 *
 * Imported parser grammars:
 *
 *     ResourceExpressions
 *     Names
 *
 * This file deliberately imports ResourceExpressions rather than creating
 * another resource-specific expression hierarchy.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Primary public rule:
 *
 *     hardwareResourceDeclaration
 *
 * Public reusable rules:
 *
 *     hardwareResourceKind
 *     hardwareResourceSpecification
 *     hardwareResourceBodyElement
 *     hardwareResourceClause
 *
 * Clause rules:
 *
 *     hardwareResourceQuantityClause
 *     hardwareResourceReferenceClause
 *     hardwareResourceRequirementClause
 *     hardwareResourceConstraintClause
 *     hardwareResourcePreferenceClause
 *     hardwareResourceHintClause
 *     hardwareResourceCapabilityClause
 *     hardwareResourceTargetClause
 *     hardwareResourceCapacityClause
 *     hardwareResourceAvailabilityClause
 *     hardwareResourcePortabilityClause
 *     hardwareResourceScalabilityClause
 *     hardwareResourcePerformanceClause
 *     hardwareResourceLatencyClause
 *     hardwareResourceThroughputClause
 *     hardwareResourceBandwidthClause
 *     hardwareResourceEnergyClause
 *     hardwareResourcePowerClause
 *     hardwareResourceReliabilityClause
 *     hardwareResourceResilienceClause
 *     hardwareResourceCostClause
 *     hardwareResourceReservationClause
 *     hardwareResourceAcquisitionClause
 *     hardwareResourceReleaseClause
 *     hardwareResourceDerivationClause
 *     hardwareResourceGroupClause
 *     hardwareResourceContractClause
 *     hardwareResourceProfileClause
 *     hardwareResourcePropertyClause
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/hardware/hardware.g4
 *
 * The Hardware composition root already imports:
 *
 *     ZamaniHardwareResourcesParser
 *
 * and dispatches:
 *
 *     hardwareResourceDeclaration
 *
 * Therefore the grammar name and primary entry rule remain stable.
 *
 * ============================================================================
 * OTHER INTEGRATION BOUNDARIES
 * ============================================================================
 *
 * Universal resource syntax:
 *
 *     grammar/resources/resources.g4
 *
 * Universal resource expressions:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * Resource requirements:
 *
 *     grammar/resources/requirements.g4
 *
 * Resource constraints:
 *
 *     grammar/resources/constraints.g4
 *
 * Resource capabilities:
 *
 *     grammar/resources/capabilities.g4
 *
 * Resource preferences:
 *
 *     grammar/resources/preferences.g4
 *
 * Resource hints:
 *
 *     grammar/resources/hints.g4
 *
 * Resource scalability:
 *
 *     grammar/resources/scalability.g4
 *
 * Resource negotiation:
 *
 *     grammar/resources/negotiation.g4
 *
 * These files own their universal resource semantics.
 *
 * This file only composes those concepts for a hardware-resource declaration.
 *
 * ============================================================================
 * AST OWNER
 * ============================================================================
 *
 * Domain-neutral frontend AST.
 *
 * This grammar does not define Rust AST structures.
 *
 * The AST must preserve:
 *
 *     source span
 *     declaration name
 *     resource kind
 *     attributes
 *     clauses
 *     expressions
 *     qualified names
 *     source ordering
 *
 * ============================================================================
 * SEMANTIC OWNER
 * ============================================================================
 *
 * Compiler/frontend semantic resource-analysis layer.
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     resource-kind resolution
 *     type checking
 *     resource-unit checking
 *     dimensional checking
 *     capability resolution
 *     requirement satisfiability
 *     constraint validation
 *     preference interpretation
 *     target compatibility
 *     portability analysis
 *     scalability analysis
 *     conflict detection
 *     resource dependency analysis
 *     lifetime validation
 *     policy interaction
 *
 * Parsing MUST NOT determine whether a requested resource exists.
 *
 * ============================================================================
 * IR OWNER
 * ============================================================================
 *
 * This grammar owns NO independent IR.
 *
 * Resource intent must be represented by the repository's canonical semantic
 * resource model and subsequently lowered into the appropriate compiler
 * representation.
 *
 * Quantum computation continues through:
 *
 *     quantum::ir
 *
 * Classical computation continues through the canonical classical IR.
 *
 * HDL/hardware intent continues through the repository's canonical
 * hardware/HDL semantic representation.
 *
 * This file MUST NOT introduce:
 *
 *     HardwareResourceIR
 *     HardwareDeviceIR
 *     HardwareCapabilityIR
 *     QuantumHardwareIR
 *
 * as competing universal IRs.
 *
 * ============================================================================
 * TEST OWNER
 * ============================================================================
 *
 * Primary tests:
 *
 *     grammar/tests/hardware/resources/
 *
 * Recommended test files:
 *
 *     declarations.zm
 *     requirements.zm
 *     capabilities.zm
 *     scalability.zm
 *     lifecycle.zm
 *     extensibility.zm
 *     negative.zm
 *     cross-domain.zm
 *
 * ============================================================================
 * SPECIFICATION OWNER
 * ============================================================================
 *
 * Primary specification:
 *
 *     grammar/spec/resources.md
 *
 * Hardware specification:
 *
 *     grammar/spec/hardware.md
 *
 * Portability:
 *
 *     grammar/spec/portability.md
 *
 * Scalability:
 *
 *     grammar/spec/scalability-model.md
 *
 * Repository architecture:
 *
 *     grammar/DESIGN.md
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Hardware resources are expressed as abstract requirements and properties.
 *
 * Source code MUST NOT need to change merely because realization changes
 * from:
 *
 *     tiny embedded hardware
 *     single-core CPU
 *     multicore CPU
 *     many-core system
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     workstation
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud infrastructure
 *     future computational substrate
 *
 * A source program describes intent.
 *
 * The compiler and runtime determine whether and how that intent can be
 * realized on an available target.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO universal resource ceilings in this grammar.
 *
 * This file MUST NOT define:
 *
 *     MAX_RESOURCES
 *     MAX_RESOURCE_GROUPS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_ACCELERATORS
 *     MAX_PORTS
 *     MAX_CONNECTIONS
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_TOPOLOGY_SIZE
 *
 * Resource quantities are expressions.
 *
 * Therefore:
 *
 *     quantity = n;
 *
 *     quantity = workload_size;
 *
 *     quantity = required_memory;
 *
 *     quantity = problem_size * parallelism;
 *
 * are all structurally valid without imposing a language-level upper bound.
 *
 * "Infinity" means:
 *
 *     no artificial finite language-level resource ceiling.
 *
 * It does NOT mean:
 *
 *     infinite physical hardware;
 *     infinite memory;
 *     infinite execution time;
 *     infinite compilation resources.
 *
 * Actual physical/resource limitations are reported by downstream resource
 * analysis and realization.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT select:
 *
 *     physical CPU
 *     physical core
 *     physical thread
 *     physical GPU
 *     physical FPGA
 *     physical ASIC
 *     physical QPU
 *     physical qubit
 *     physical node
 *     physical memory bank
 *     PCI address
 *     machine hostname
 *     vendor serial number
 *     physical bus location
 *
 * Abstract target intent is allowed.
 *
 * Concrete target selection belongs downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY / REQUIREMENT DISTINCTION
 * ============================================================================
 *
 * RESOURCE
 *     An abstract resource category or property.
 *
 * CAPABILITY
 *     An ability supplied by a realization.
 *
 * REQUIREMENT
 *     A condition that must be satisfied.
 *
 * CONSTRAINT
 *     A mandatory condition restricting legal realization.
 *
 * PREFERENCE
 *     Advisory realization guidance.
 *
 * HINT
 *     Non-binding advisory information.
 *
 * CAPACITY
 *     A supplied/observed quantity.
 *
 * AVAILABILITY
 *     A supplied/observed availability condition.
 *
 * TARGET
 *     An abstract realization class or execution context.
 *
 * RESERVATION
 *     Declarative intent that a resource may need reservation.
 *
 * ACQUISITION
 *     Declarative intent that a resource may need acquisition.
 *
 * RELEASE
 *     Declarative intent to relinquish an acquired/reserved logical resource.
 *
 * None of these declarations performs physical allocation.
 *
 * ============================================================================
 * RESOURCE EXPRESSION POLICY
 * ============================================================================
 *
 * All resource values and predicates use:
 *
 *     resourceExpression
 *
 * from:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * ResourceExpressions itself delegates to the canonical:
 *
 *     expression
 *
 * grammar.
 *
 * This provides one expression hierarchy across:
 *
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     execution
 *
 * ============================================================================
 * OPEN-WORLD RESOURCE KINDS
 * ============================================================================
 *
 * Resource kinds are qualified semantic names.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     storage
 *     accelerator
 *     quantum::logical_qubit
 *     quantum::measurement
 *     network::bandwidth
 *     network::latency
 *     accelerator::tensor_compute
 *     custom::future_resource
 *
 * The grammar deliberately does NOT enumerate resource kinds.
 *
 * New resources can therefore be introduced through:
 *
 *     semantic registration
 *     target metadata
 *     capabilities
 *     dialects
 *     libraries
 *     future compiler extensions
 *
 * without changing this universal hardware-resource grammar merely because
 * the resource universe grows.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A hardware resource declaration introduces a LOGICAL resource contract.
 *
 * Examples:
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource qpu_memory: quantum::logical_qubit;
 *
 *     resource accelerator: accelerator {
 *         quantity = workload_size;
 *     };
 *
 * The declaration name is a source-level semantic name.
 *
 * It is not a physical device identifier.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareResourcesParser;

options {
    tokenVocab = ZamaniLexer;
}

import
    ResourceExpressions,
    Names
;


/*
 * ============================================================================
 * 1. HARDWARE RESOURCE DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource accelerator: accelerator {
 *         quantity = workload_size;
 *         requires capability("tensor.compute");
 *     };
 *
 *     resource qpu_memory: quantum::logical_qubit {
 *         requires qubits >= required_qubits;
 *     };
 *
 * A declaration does not allocate anything.
 */
hardwareResourceDeclaration
    : hardwareResourceAttributes*
      RESOURCE
      identifier
      hardwareResourceKind?
      hardwareResourceSpecification?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. RESOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * Their semantic interpretation belongs downstream.
 *
 * Example:
 *
 *     @portable
 *     @domain(hardware)
 *     resource compute;
 */
hardwareResourceAttributes
    : AT
      qualifiedName
      (
          LPAREN
          optionalResourceExpressionList
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 3. RESOURCE KIND
 * ============================================================================
 *
 * Resource kinds are open-world qualified names.
 *
 * Example:
 *
 *     : memory
 *     : accelerator
 *     : quantum::logical_qubit
 *     : network::bandwidth
 */
hardwareResourceKind
    : COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * 4. RESOURCE SPECIFICATION
 * ============================================================================
 *
 * A specification contains zero or more resource clauses.
 *
 * No fixed clause count is imposed.
 */
hardwareResourceSpecification
    : LBRACE
      hardwareResourceBodyElement*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. RESOURCE BODY ELEMENT
 * ============================================================================
 */
hardwareResourceBodyElement
    : hardwareResourceAttributes*
      hardwareResourceClause
    ;


/*
 * ============================================================================
 * 6. RESOURCE CLAUSE DISPATCH
 * ============================================================================
 *
 * This is the only hardware-resource clause dispatcher.
 *
 * Specialized universal resource grammars are consumed through
 * resourceExpression payloads.
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
    | hardwareResourceThroughputClause
    | hardwareResourceBandwidthClause
    | hardwareResourceEnergyClause
    | hardwareResourcePowerClause
    | hardwareResourceReliabilityClause
    | hardwareResourceResilienceClause
    | hardwareResourceCostClause
    | hardwareResourceReservationClause
    | hardwareResourceAcquisitionClause
    | hardwareResourceReleaseClause
    | hardwareResourceDerivationClause
    | hardwareResourceGroupClause
    | hardwareResourceContractClause
    | hardwareResourceProfileClause
    | hardwareResourcePropertyClause
    ;


/*
 * ============================================================================
 * 7. QUANTITY
 * ============================================================================
 *
 * Quantity values remain expressions.
 *
 * Examples:
 *
 *     quantity = n;
 *     quantity = workload_size;
 *     quantity = problem_size * parallelism;
 *     quantity = required_memory;
 *
 * No physical width or numeric ceiling is imposed here.
 */
hardwareResourceQuantityClause
    : QUANTITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. RESOURCE REFERENCE
 * ============================================================================
 *
 * References an abstract/logical resource.
 *
 * This does not identify a physical device.
 */
hardwareResourceReferenceClause
    : USE
      RESOURCE
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. REQUIREMENT
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * Supported intent includes:
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 *     requires resource memory >= required_memory;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("gpu.compute");
 *
 *     requires topology(required_topology);
 *
 * The expression remains syntactically general.
 *
 * Resource/capability interpretation belongs to semantic analysis.
 */
hardwareResourceRequirementClause
    : REQUIRES
      hardwareResourceRequirementSubject
      SEMICOLON
    ;


hardwareResourceRequirementSubject
    : RESOURCE
      qualifiedName
      hardwareResourceRelationExpression?
    | CAPABILITY
      resourceExpression
    | resourceExpression
    ;


hardwareResourceRelationExpression
    : hardwareResourceRelationOperator
      resourceExpression
    ;


/*
 * ============================================================================
 * 10. CONSTRAINT
 * ============================================================================
 *
 * A constraint is mandatory realization intent.
 *
 * Example:
 *
 *     constraint latency <= latency_budget;
 */
hardwareResourceConstraintClause
    : CONSTRAINT
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. PREFERENCE
 * ============================================================================
 *
 * Preferences are not requirements.
 *
 * They may influence target selection and optimization.
 */
hardwareResourcePreferenceClause
    : PREFER
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. HINT
 * ============================================================================
 *
 * Hints are advisory and non-binding.
 */
hardwareResourceHintClause
    : HINT
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. CAPABILITY
 * ============================================================================
 *
 * Capability intent remains open-world.
 *
 * Examples:
 *
 *     capability = quantum::measurement;
 *
 *     capability = tensor::compute;
 *
 *     capability("quantum.measurement");
 */
hardwareResourceCapabilityClause
    : CAPABILITY
      (
          ASSIGN
      )?
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. TARGET
 * ============================================================================
 *
 * Target is an abstract realization class/context.
 *
 * Examples:
 *
 *     target = gpu;
 *
 *     target = accelerator::tensor;
 *
 *     target = quantum;
 *
 * No physical device is selected.
 */
hardwareResourceTargetClause
    : TARGET
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. CAPACITY
 * ============================================================================
 *
 * Capacity represents a value/observation.
 *
 * It is NOT a language maximum.
 */
hardwareResourceCapacityClause
    : CAPACITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. AVAILABILITY
 * ============================================================================
 *
 * Availability represents a value/condition supplied by the relevant
 * compilation/execution context.
 *
 * Parsing does not inspect availability.
 */
hardwareResourceAvailabilityClause
    : AVAILABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. PORTABILITY
 * ============================================================================
 *
 * Portability intent remains symbolic.
 *
 * Example:
 *
 *     portability = portable;
 *
 * or a richer resource expression supplied by the semantic model.
 */
hardwareResourcePortabilityClause
    : PORTABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. SCALABILITY
 * ============================================================================
 *
 * Scaling relationships are symbolic.
 *
 * Examples:
 *
 *     scalability = problem_size;
 *
 *     scalability = workload_size * parallelism;
 *
 *     scalability = available_capacity;
 *
 * No finite scaling limit is encoded.
 */
hardwareResourceScalabilityClause
    : SCALABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. PERFORMANCE
 * ============================================================================
 */
hardwareResourcePerformanceClause
    : PERFORMANCE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. LATENCY
 * ============================================================================
 */
hardwareResourceLatencyClause
    : LATENCY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. THROUGHPUT
 * ============================================================================
 */
hardwareResourceThroughputClause
    : THROUGHPUT
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. BANDWIDTH
 * ============================================================================
 */
hardwareResourceBandwidthClause
    : BANDWIDTH
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. ENERGY
 * ============================================================================
 */
hardwareResourceEnergyClause
    : ENERGY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. POWER
 * ============================================================================
 */
hardwareResourcePowerClause
    : POWER
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. RELIABILITY
 * ============================================================================
 */
hardwareResourceReliabilityClause
    : RELIABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. RESILIENCE
 * ============================================================================
 *
 * Resilience is a semantic property.
 *
 * Runtime states, recovery and fault handling are not parsed here.
 */
hardwareResourceResilienceClause
    : RESILIENCE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 27. COST
 * ============================================================================
 */
hardwareResourceCostClause
    : COST
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 28. RESERVATION INTENT
 * ============================================================================
 *
 * This expresses intent only.
 *
 * It does NOT reserve anything during parsing or compilation.
 *
 * Actual reservation belongs to resource realization/runtime infrastructure.
 */
hardwareResourceReservationClause
    : RESERVE
      RESOURCE
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 29. ACQUISITION INTENT
 * ============================================================================
 *
 * This expresses an abstract acquisition request.
 *
 * It does NOT allocate or acquire a physical resource during parsing.
 */
hardwareResourceAcquisitionClause
    : ACQUIRE
      RESOURCE
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 30. RELEASE INTENT
 * ============================================================================
 *
 * This expresses logical release intent.
 *
 * Runtime ownership and allocation remain outside the grammar.
 */
hardwareResourceReleaseClause
    : RELEASE
      RESOURCE
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 31. DERIVATION
 * ============================================================================
 *
 * Derives a symbolic resource value from program/resource semantics.
 *
 * Example:
 *
 *     derive required_memory = elements * element_size;
 *
 * Derivation does not execute during parsing.
 */
hardwareResourceDerivationClause
    : DERIVE
      identifier
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 32. RESOURCE GROUP
 * ============================================================================
 *
 * Resource groups provide hierarchical organization.
 *
 * Groups are recursively nestable and therefore do not impose a fixed depth.
 *
 * Example:
 *
 *     resource group compute {
 *         quantity = workload_size;
 *
 *         resource group accelerator {
 *             requires capability("tensor.compute");
 *         }
 *     };
 */
hardwareResourceGroupClause
    : RESOURCE
      GROUP
      identifier
      hardwareResourceSpecification
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 33. RESOURCE CONTRACT
 * ============================================================================
 *
 * A contract groups resource intent.
 *
 * It does not allocate or select hardware.
 *
 * Example:
 *
 *     contract compute_contract {
 *         requires memory >= required_memory;
 *         requires capability("tensor.compute");
 *     };
 */
hardwareResourceContractClause
    : CONTRACT
      identifier?
      hardwareResourceSpecification
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 34. RESOURCE PROFILE
 * ============================================================================
 *
 * A profile is a reusable symbolic collection of resource intent.
 *
 * It is not a physical target profile.
 */
hardwareResourceProfileClause
    : PROFILE
      identifier?
      hardwareResourceSpecification
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 35. OPEN-WORLD RESOURCE PROPERTY
 * ============================================================================
 *
 * Standard and future resource properties must not require a new universal
 * keyword merely because a new resource dimension is introduced.
 *
 * Explicit form:
 *
 *     property bandwidth_class = preferred;
 *
 * Open-world form:
 *
 *     bandwidth_class = preferred;
 *
 * The semantic layer determines whether the property is:
 *
 *     standard
 *     dialect-defined
 *     target-defined
 *     experimental
 *     deprecated
 *     invalid
 *
 * Physical realization remains outside this grammar.
 */
hardwareResourcePropertyClause
    : PROPERTY
      qualifiedName
      ASSIGN
      resourceExpression
      SEMICOLON
    | qualifiedName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 36. RELATION OPERATORS
 * ============================================================================
 *
 * Use the repository's canonical lexer vocabulary.
 *
 * These are:
 *
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS
 *     LESS_EQUAL
 *     GREATER
 *     GREATER_EQUAL
 *
 * This file does not define another operator vocabulary.
 */
hardwareResourceRelationOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * 37. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve at least:
 *
 *     HardwareResourceDeclaration
 *     HardwareResourceKind
 *     HardwareResourceAttribute
 *     HardwareResourceQuantity
 *     HardwareResourceReference
 *     HardwareResourceRequirement
 *     HardwareResourceConstraint
 *     HardwareResourcePreference
 *     HardwareResourceHint
 *     HardwareResourceCapability
 *     HardwareResourceTarget
 *     HardwareResourceCapacity
 *     HardwareResourceAvailability
 *     HardwareResourcePortability
 *     HardwareResourceScalability
 *     HardwareResourcePerformance
 *     HardwareResourceLatency
 *     HardwareResourceThroughput
 *     HardwareResourceBandwidth
 *     HardwareResourceEnergy
 *     HardwareResourcePower
 *     HardwareResourceReliability
 *     HardwareResourceResilience
 *     HardwareResourceCost
 *     HardwareResourceReservation
 *     HardwareResourceAcquisition
 *     HardwareResourceRelease
 *     HardwareResourceDerivation
 *     HardwareResourceGroup
 *     HardwareResourceContract
 *     HardwareResourceProfile
 *     HardwareResourceProperty
 *
 * Every AST node must preserve source-span information.
 *
 * Resource expressions must remain structured expression nodes rather than
 * being flattened into strings.
 *
 * Qualified names must remain structured canonical name nodes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     resolve resource names;
 *     resolve resource kinds;
 *     validate declarations;
 *     validate resource expressions;
 *     validate dimensions/units;
 *     classify requirements;
 *     evaluate requirement satisfiability;
 *     validate constraints;
 *     distinguish preferences from requirements;
 *     resolve capabilities;
 *     validate target compatibility;
 *     evaluate portability;
 *     evaluate scalability relationships;
 *     detect conflicts;
 *     preserve provenance;
 *     apply policies where applicable;
 *     produce deterministic diagnostics.
 *
 * Example:
 *
 *     requires qubits >= required_qubits;
 *
 * is syntactically valid even if the current machine has insufficient
 * resources.
 *
 * Insufficient capacity is a semantic/realization result, not a parser error.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing any construct in this file has no execution effect.
 *
 * In particular:
 *
 *     reserve
 *     acquire
 *     release
 *
 * are declarations of intent.
 *
 * They do not perform allocation.
 *
 * If later semantic/runtime layers associate effects with them, those effects
 * must be represented in the repository's existing effect system.
 *
 * This grammar must not invent a second effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability expressions remain symbolic.
 *
 * Examples:
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("gpu.compute");
 *
 *     requires capability("tensor.compute");
 *
 *     capability = quantum::measurement;
 *
 * No capability list is hard-coded here.
 *
 * A new capability must be introducible through semantic registration,
 * capability metadata, dialects or target descriptions.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource values may depend on:
 *
 *     program values
 *     type information
 *     workload size
 *     problem size
 *     symbolic parameters
 *     derived values
 *     other resource expressions
 *     compilation context
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *
 *     quantity = workload_size * element_size;
 *
 *     scalability = problem_size * parallelism;
 *
 * No physical capacity is encoded into this grammar.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policy semantics belong to:
 *
 *     grammar/policies/
 *
 * This grammar may provide resource expressions consumed by policy analysis,
 * but it must not duplicate the policy language.
 *
 * Example semantic flow:
 *
 *     hardware resource intent
 *          |
 *          v
 *     resource semantic model
 *          |
 *          v
 *     policy evaluation
 *          |
 *          v
 *     target/resource negotiation
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Source spans and declaration structure must survive parsing so downstream
 * provenance can record:
 *
 *     source declaration
 *     derived resource requirement
 *     capability requirement
 *     constraint
 *     preference
 *     semantic transformation
 *     realization decision
 *
 * This grammar itself does not create runtime provenance records.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum resources may be expressed symbolically.
 *
 * Examples:
 *
 *     resource logical_qubits: quantum::logical_qubit {
 *         quantity = required_qubits;
 *     };
 *
 *     resource qpu: quantum::qpu {
 *         requires capability("quantum.measurement");
 *         requires capability("quantum.dynamic_control");
 *     };
 *
 * Quantum operations themselves remain owned by:
 *
 *     grammar/quantum/
 *
 * Quantum IR remains:
 *
 *     quantum::ir
 *
 * This grammar must never:
 *
 *     enumerate physical qubits;
 *     select physical qubits;
 *     encode a maximum qubit count;
 *     create a second quantum IR;
 *     encode vendor-specific QPU topology.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical resources may express:
 *
 *     memory
 *     compute
 *     storage
 *     vector/tensor capability
 *     bandwidth
 *     latency
 *     throughput
 *
 * without specifying a particular CPU or memory device.
 *
 * Example:
 *
 *     resource memory: memory {
 *         requires capacity >= required_memory;
 *     };
 *
 * ============================================================================
 * GPU / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Accelerator resource identity remains open-world.
 *
 * Examples:
 *
 *     resource tensor_compute: accelerator::tensor {
 *         requires capability("tensor.compute");
 *         quantity = workload_size;
 *     };
 *
 *     resource accelerator: accelerator {
 *         prefer target = accelerator::tensor;
 *     };
 *
 * Concrete GPU/accelerator selection belongs to target negotiation.
 *
 * ============================================================================
 * FPGA / ASIC INTEGRATION
 * ============================================================================
 *
 * FPGA and ASIC grammars own their hardware-specific declaration semantics.
 *
 * This resource grammar may be consumed alongside them to express:
 *
 *     memory
 *     compute
 *     bandwidth
 *     latency
 *     power
 *     energy
 *     reliability
 *     resilience
 *     capability
 *
 * It must not reproduce FPGA/ASIC implementation syntax.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL behavioral constructs remain owned by:
 *
 *     grammar/hdl/
 *
 * Hardware resources can accompany HDL intent but do not replace:
 *
 *     signals
 *     processes
 *     procedural behavior
 *     timing semantics
 *     simulation semantics
 *     synthesis semantics
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * A hybrid program can use this resource contract to describe requirements
 * shared across:
 *
 *     classical computation
 *     quantum computation
 *     accelerators
 *     hardware
 *     distributed computation
 *     data processing
 *     AI/ML
 *
 * There is one resource-intent model rather than one resource language per
 * domain.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Resource expressions may describe:
 *
 *     nodes
 *     memory
 *     compute
 *     bandwidth
 *     latency
 *     topology
 *     accelerator availability
 *
 * without encoding a maximum node count or network size.
 *
 * Example:
 *
 *     requires nodes >= required_nodes;
 *
 * The actual node set is selected downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar version
 *     selected parser/dialect composition
 *
 * Parsing must NOT depend on:
 *
 *     CPU count
 *     memory availability
 *     GPU availability
 *     FPGA availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *     runtime state
 *
 * Therefore identical source and parser configuration produce the same parse
 * structure.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar is declarative and action-free.
 *
 * It contains:
 *
 *     no Rust;
 *     no unsafe code;
 *     no target-language actions;
 *     no semantic predicates;
 *     no hardware calls;
 *     no process execution;
 *     no filesystem access;
 *     no network access.
 *
 * The consuming implementation must remain compatible with:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable syntax must remain compatible with:
 *
 *     grammar/spec/hardware.md
 *     grammar/spec/resources.md
 *     grammar/spec/portability.md
 *     grammar/spec/scalability-model.md
 *     grammar/compatibility/
 *     grammar/grammar.md
 *
 * Historical/proposed syntax in:
 *
 *     grammar/Zamani-Grammar.md
 *
 * does not automatically become normative.
 *
 * Compatibility aliases must be handled by the canonical compatibility
 * architecture and must not create duplicate token identities.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * The grammar must reject malformed structural forms such as:
 *
 *     resource;
 *     resource : memory;
 *     resource compute {
 *         quantity = ;
 *     };
 *
 *     resource compute {
 *         requires;
 *     };
 *
 *     resource compute {
 *         property bandwidth_class = ;
 *     };
 *
 * Resource insufficiency is NOT a syntax error.
 *
 * Example:
 *
 *     requires memory >= enormous_requirement;
 *
 * remains syntactically valid.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must be represented by the hardware-resource test suite.
 *
 * --------------------------------------------------------------------------
 * Basic declarations
 * --------------------------------------------------------------------------
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource storage: storage;
 *
 *     resource accelerator: accelerator;
 *
 * --------------------------------------------------------------------------
 * Quantities
 * --------------------------------------------------------------------------
 *
 *     resource compute {
 *         quantity = workload_size;
 *     };
 *
 *     resource memory: memory {
 *         quantity = required_memory;
 *     };
 *
 * --------------------------------------------------------------------------
 * Requirements
 * --------------------------------------------------------------------------
 *
 *     resource memory: memory {
 *         requires capacity >= required_memory;
 *     };
 *
 *     resource qpu: quantum::qpu {
 *         requires capability("quantum.measurement");
 *     };
 *
 *     resource accelerator: accelerator {
 *         requires capability("tensor.compute");
 *     };
 *
 * --------------------------------------------------------------------------
 * Constraints
 * --------------------------------------------------------------------------
 *
 *     resource network: network::bandwidth {
 *         constraint bandwidth >= required_bandwidth;
 *     };
 *
 *     resource compute {
 *         constraint latency <= latency_budget;
 *     };
 *
 * --------------------------------------------------------------------------
 * Preferences and hints
 * --------------------------------------------------------------------------
 *
 *     resource compute {
 *         prefer target = accelerator::tensor;
 *         hint preferred_distribution;
 *     };
 *
 * --------------------------------------------------------------------------
 * Scalability
 * --------------------------------------------------------------------------
 *
 *     resource compute {
 *         scalability = problem_size;
 *     };
 *
 *     resource compute {
 *         scalability = workload_size * parallelism;
 *     };
 *
 * --------------------------------------------------------------------------
 * Performance
 * --------------------------------------------------------------------------
 *
 *     resource network: network {
 *         performance = required_performance;
 *         latency = latency_budget;
 *         throughput = required_throughput;
 *         bandwidth = required_bandwidth;
 *     };
 *
 * --------------------------------------------------------------------------
 * Reliability
 * --------------------------------------------------------------------------
 *
 *     resource system: compute {
 *         reliability = required_reliability;
 *         resilience = required_resilience;
 *     };
 *
 * --------------------------------------------------------------------------
 * Derivation
 * --------------------------------------------------------------------------
 *
 *     resource memory: memory {
 *         derive required_memory = elements * element_size;
 *         requires capacity >= required_memory;
 *     };
 *
 * --------------------------------------------------------------------------
 * Grouping
 * --------------------------------------------------------------------------
 *
 *     resource group compute {
 *         quantity = workload_size;
 *
 *         resource group accelerator {
 *             requires capability("tensor.compute");
 *         };
 *     };
 *
 * --------------------------------------------------------------------------
 * Contracts
 * --------------------------------------------------------------------------
 *
 *     resource compute {
 *         contract {
 *             requires capability("compute");
 *             constraint latency <= latency_budget;
 *         };
 *     };
 *
 * --------------------------------------------------------------------------
 * Profiles
 * --------------------------------------------------------------------------
 *
 *     resource compute {
 *         profile scalable {
 *             scalability = problem_size;
 *             prefer target = accelerator;
 *         };
 *     };
 *
 * --------------------------------------------------------------------------
 * Extensible properties
 * --------------------------------------------------------------------------
 *
 *     resource compute {
 *         property scheduling_class = scalable;
 *         vendor_extension = value;
 *     };
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Test malformed structures including:
 *
 *     resource;
 *     resource : memory;
 *     resource compute {
 *         quantity = ;
 *     };
 *     resource compute {
 *         requires;
 *     };
 *     resource compute {
 *         capability = ;
 *     };
 *     resource compute {
 *         target = ;
 *     };
 *     resource compute {
 *         property = ;
 *     };
 *
 * Also verify that physical allocation is not implied by parsing.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must cover:
 *
 *     symbolic quantities;
 *     large supported literals;
 *     deeply nested resource groups within implementation limits;
 *     large numbers of resource clauses;
 *     large numbers of properties;
 *     symbolic resource derivations;
 *     symbolic scalability expressions;
 *     large distributed resource requirements;
 *     large quantum resource requirements;
 *     large accelerator requirements;
 *     heterogeneous resource sets.
 *
 * Tests MUST NOT assert an artificial maximum such as:
 *
 *     64 GB
 *     24 GB
 *     32-bit
 *     8 threads
 *     32 qubits
 *     1024 nodes
 *
 * Such values may appear as ordinary program/test values where semantically
 * appropriate, but they must never become language-level ceilings.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * At minimum, test resource intent alongside:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid quantum-classical computation;
 *     HDL;
 *     accelerators;
 *     AI/ML;
 *     distributed execution;
 *     networking;
 *     simulation;
 *     adaptive execution;
 *     deterministic execution;
 *     contracts;
 *     policies;
 *     provenance.
 *
 * ============================================================================
 * BUILD INTEGRATION
 * ============================================================================
 *
 * Existing hardware composition:
 *
 *     grammar/hardware/hardware.g4
 *
 * already imports:
 *
 *     ZamaniHardwareResourcesParser
 *
 * and dispatches:
 *
 *     hardwareResourceDeclaration
 *
 * Therefore this file replacement preserves that integration boundary.
 *
 * No change to hardware.g4 is required merely to adopt this grammar.
 *
 * ANTLR must make the following grammar library directories available:
 *
 *     grammar/hardware
 *     grammar/resources
 *     grammar/core
 *     grammar/expressions
 *
 * The repository build system owns the exact ANTLR invocation.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * Generated parser code is consumed by the existing Rust frontend.
 *
 * This grammar introduces no Rust implementation requirements.
 *
 * The Rust implementation must:
 *
 *     preserve source spans;
 *     map parse structures into the domain-neutral AST;
 *     perform semantic resource validation downstream;
 *     remain safe Rust;
 *     remain compatible with Rust 1.97 or later.
 *
 * No unsafe Rust is required or permitted by this grammar's design.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS DONE WHEN:
 *
 * [x] It has one canonical grammar name:
 *         ZamaniHardwareResourcesParser
 *
 * [x] Its primary entry point is:
 *         hardwareResourceDeclaration
 *
 * [x] hardware.g4 can import it without changing its public boundary.
 *
 * [x] It consumes the canonical ZamaniLexer vocabulary.
 *
 * [x] It imports ResourceExpressions instead of creating a second resource
 *     expression hierarchy.
 *
 * [x] It imports Names instead of redefining identifier/qualified-name syntax.
 *
 * [x] It contains no local identifier wrapper.
 *
 * [x] It contains no duplicate lexer tokens.
 *
 * [x] It contains no physical-device discovery.
 *
 * [x] It contains no allocation implementation.
 *
 * [x] It contains no routing implementation.
 *
 * [x] It contains no scheduling implementation.
 *
 * [x] It contains no runtime execution.
 *
 * [x] It contains no fixed resource ceilings.
 *
 * [x] It contains no fixed hardware capacities.
 *
 * [x] It contains no fixed qubit count.
 *
 * [x] It contains no fixed CPU/GPU/FPGA/ASIC/device count.
 *
 * [x] It uses open-world resource kinds.
 *
 * [x] It uses symbolic resource expressions.
 *
 * [x] It preserves requirement/constraint/preference/hint distinctions.
 *
 * [x] It preserves resource/capability separation.
 *
 * [x] It preserves target independence.
 *
 * [x] It preserves the quantum::ir boundary.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It defines its AST boundary.
 *
 * [x] It defines its semantic boundary.
 *
 * [x] It defines its IR boundary.
 *
 * [x] It defines its integration boundary.
 *
 * [x] It defines positive, negative, scalability and cross-domain tests.
 *
 * Repository verification still required:
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Hardware composition succeeds.
 *
 * [ ] Canonical parser generation succeeds.
 *
 * [ ] Rust frontend compilation succeeds on Rust 1.97 or later.
 *
 * [ ] Hardware resource tests pass.
 *
 * [ ] Resource-system integration tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file describes WHAT HARDWARE RESOURCES MEAN AT THE SOURCE LEVEL.
 *
 * It does not decide:
 *
 *     where they live;
 *     which machine provides them;
 *     which device implements them;
 *     how they are allocated;
 *     how operations are routed;
 *     how execution is scheduled;
 *     how hardware is calibrated;
 *     how quantum operations are decomposed;
 *     how QEC is performed;
 *     how ZQN is generated;
 *     how HAL realizes the result.
 *
 * The resulting pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     hardwareResourceDeclaration
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     semantic resource model
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     classical intent     quantum intent
 *                              |
 *                              v
 *                         quantum::ir
 *          |
 *          v
 *     target-independent optimization
 *          |
 *          v
 *     resource negotiation
 *          |
 *          v
 *     lowering
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * This is the required boundary for scalable,
 * target-independent hardware resource intent.
 *
 * ============================================================================
 */