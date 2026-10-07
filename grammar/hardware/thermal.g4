/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/thermal.g4
 *
 * Grammar:
 *     ZamaniHardwareThermalParser
 *
 * Status:
 *     CANONICAL / PRODUCTION THERMAL-INTENT GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust edition 2021
 *
 * Safety:
 *     Pure ANTLR4 parser grammar.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No I/O.
 *     No hardware discovery.
 *     No runtime execution.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns source-level thermal intent for hardware-independent
 * computation.
 *
 * Thermal intent describes requirements, constraints, preferences, hints,
 * properties, capabilities, resources, profiles, states, transitions,
 * measurements, scaling relationships, and other thermal metadata.
 *
 * The grammar describes WHAT is required or preferred.
 *
 * It does not describe HOW a target realizes that intent.
 *
 *
 * OWNS
 * ----
 *
 * This file owns:
 *
 *     hardwareThermalDeclaration
 *     hardwareThermalBody
 *     hardwareThermalItem
 *     thermal requirements
 *     thermal constraints
 *     thermal preferences
 *     thermal hints
 *     thermal capability references
 *     thermal resource references
 *     thermal target references
 *     thermal properties
 *     thermal assertions
 *     thermal named sections
 *     thermal transitions
 *
 *
 * DOES NOT OWN
 * ------------
 *
 * This file does NOT own:
 *
 *     lexer tokens
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     generic constraint semantics
 *     generic resource semantics
 *     generic capability semantics
 *     power semantics
 *     energy semantics
 *     timing semantics
 *     reliability semantics
 *     placement
 *     topology
 *     scheduling
 *     optimization
 *     hardware discovery
 *     device IDs
 *     device drivers
 *     runtime thermal control
 *     physical sensors
 *     cooling-device implementation
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Hardware
 *       |
 *       +--> ZamaniHardwareThermalParser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> type analysis
 *       +--> resource analysis
 *       +--> capability analysis
 *       +--> constraint analysis
 *       +--> policy analysis
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       +--> placement
 *       +--> routing
 *       +--> scheduling
 *       +--> resilience
 *       |
 *       v
 *     ZQN / HAL where applicable
 *       |
 *       v
 *     target realization
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Thermal syntax expresses portable intent.
 *
 * It MUST NOT select:
 *
 *     a physical CPU
 *     a physical core
 *     a physical GPU
 *     a physical FPGA
 *     a physical ASIC
 *     a physical QPU
 *     a physical sensor
 *     a physical cooling device
 *     a physical node
 *     a physical machine
 *
 * The same source must remain representable across:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * Target feasibility is determined downstream.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar has NO language-level finite thermal limits.
 *
 * It MUST NOT define:
 *
 *     MAX_TEMPERATURE
 *     MAX_THERMAL_DOMAINS
 *     MAX_THERMAL_STATES
 *     MAX_THERMAL_PROFILES
 *     MAX_THERMAL_SENSORS
 *     MAX_THERMAL_TRANSITIONS
 *     MAX_COOLING_DEVICES
 *     MAX_POWER
 *     MAX_ENERGY
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * Repetition uses ANTLR repetition operators.
 *
 * Quantities are expressions.
 *
 * Therefore quantities may be:
 *
 *     literals
 *     variables
 *     constants
 *     parameters
 *     functions
 *     symbolic expressions
 *     derived values
 *     runtime-provided values
 *     target-provided values
 *
 * Practical implementation limits belong to the compiler/runtime environment,
 * not the language grammar.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Thermal property names remain extensible.
 *
 * The grammar deliberately does NOT enumerate every possible thermal concept.
 *
 * Examples that remain ordinary identifiers:
 *
 *     temperature
 *     ambient
 *     junction
 *     hotspot
 *     gradient
 *     cooling
 *     dissipation
 *     resistance
 *     conductivity
 *     emissivity
 *     heat_flux
 *     thermal_margin
 *     cryogenic_margin
 *     thermal_stability
 *     thermal_noise
 *     thermal_load
 *
 * New thermal concepts therefore do not require new language keywords.
 *
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer
 *
 * The thermal declaration keyword is:
 *
 *     THERMAL
 *
 * `THERMAL` is a canonical lexical token and must be emitted by the production
 * lexer.
 *
 * Other thermal vocabulary remains contextual identifiers unless the language
 * already reserves the spelling for another universal construct.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * All expressions are delegated to the canonical expression composition root:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file MUST NOT define:
 *
 *     arithmetic precedence
 *     logical precedence
 *     calls
 *     indexing
 *     member access
 *     literals
 *     unary operators
 *     conditional expressions
 *     collection expressions
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Names are delegated through the canonical expression/name architecture.
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     nameSegment
 *
 *
 * ============================================================================
 * POWER BOUNDARY
 * ============================================================================
 *
 * Power remains owned by:
 *
 *     grammar/hardware/power.g4
 *
 * Thermal syntax may reference power through expressions or thermal properties,
 * but must not redefine power budgets, power states, or power transitions.
 *
 *
 * ============================================================================
 * TIMING BOUNDARY
 * ============================================================================
 *
 * Timing remains owned by:
 *
 *     grammar/hardware/timing.g4
 *
 * Thermal expressions may depend on time, workload, frequency, or other
 * symbolic timing values through normal expressions.
 *
 * This file does not define a second timing model.
 *
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Resource semantics remain owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/resources.g4
 *
 * Thermal resource references express intent only.
 *
 * Example:
 *
 *     resource thermal.capacity >= required_capacity;
 *
 * The semantic layer determines whether such a resource exists and whether
 * it can satisfy the requirement.
 *
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Capability semantics remain owned by the canonical capability subsystem.
 *
 * Example:
 *
 *     capability thermal.monitoring;
 *
 * or:
 *
 *     requires capability("thermal.cooling");
 *
 * This grammar does not decide whether a capability is available.
 *
 *
 * ============================================================================
 * CONTRACT / POLICY BOUNDARY
 * ============================================================================
 *
 * `requires`, `constraint`, `prefer`, `hint`, and `assert` are source-level
 * thermal intent.
 *
 * Generic contract semantics remain downstream.
 *
 * Policies may later:
 *
 *     permit
 *     prohibit
 *     prioritize
 *     restrict
 *     condition
 *     select
 *     fallback
 *
 * thermal realization.
 *
 * This grammar does not perform policy evaluation.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every thermal construct must remain traceable through the frontend.
 *
 * Downstream semantic representation should preserve:
 *
 *     source span
 *     declaration name
 *     declaration kind
 *     property name
 *     operator
 *     expression
 *     resource reference
 *     capability reference
 *     target reference
 *     section name
 *     transition source
 *     transition target
 *
 * Provenance implementation belongs outside this grammar.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file owns NO thermal-specific IR.
 *
 * Do NOT create:
 *
 *     ThermalIR
 *     HardwareThermalIR
 *     ThermalDeviceIR
 *     ThermalControlIR
 *
 * merely because thermal syntax exists.
 *
 * Thermal intent is lowered through the canonical semantic/IR architecture.
 *
 * Quantum computation continues through:
 *
 *     quantum::ir
 *
 * Thermal requirements may influence quantum resource analysis, scheduling,
 * resilience, or HAL realization, but this grammar never constructs or owns
 * quantum::ir.
 *
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

parser grammar ZamaniHardwareThermalParser;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/* ============================================================================
 * 1. THERMAL DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     thermal name {
 *         ...
 *     }
 *
 *     thermal contract name {
 *         ...
 *     }
 *
 *     thermal profile name {
 *         ...
 *     }
 *
 * `contract` and `profile` are declaration kinds.
 *
 * They are not implemented as separate nested grammars.
 *
 * This prevents duplicated declaration ownership.
 * ========================================================================== */

hardwareThermalDeclaration
    : hardwareThermalAttribute*
      hardwareThermalVisibility?
      hardwareThermalModifier*
      THERMAL
      hardwareThermalDeclarationKind?
      identifier
      hardwareThermalGenericParameters?
      hardwareThermalTargetClause?
      hardwareThermalBody
    ;


/* ============================================================================
 * 2. DECLARATION KIND
 * ========================================================================== */

hardwareThermalDeclarationKind
    : CONTRACT
    | PROFILE
    ;


/* ============================================================================
 * 3. DECLARATION VISIBILITY
 * ========================================================================= */

hardwareThermalVisibility
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/* ============================================================================
 * 4. DECLARATION MODIFIERS
 * ========================================================================= */

hardwareThermalModifier
    : STATIC
    | CONST
    | EXTERN
    | FINAL
    | ABSTRACT
    | SEALED
    | PARTIAL
    ;


/* ============================================================================
 * 5. ATTRIBUTES
 * ============================================================================
 *
 * Attributes use the canonical name and expression systems.
 *
 * Example:
 *
 *     @vendor::thermal(expression)
 *
 * The attribute itself has no runtime meaning at grammar level.
 * ========================================================================== */

hardwareThermalAttribute
    : AT
      qualifiedName
      (
          LPAREN
          expressionList?
          RPAREN
      )?
    ;


/* ============================================================================
 * 6. GENERIC PARAMETERS
 * ============================================================================
 *
 * Thermal intent can depend on arbitrary symbolic parameters.
 *
 * No finite machine scale is represented here.
 * ========================================================================== */

hardwareThermalGenericParameters
    : LESS
      hardwareThermalGenericParameter
      (
          COMMA
          hardwareThermalGenericParameter
      )*
      COMMA?
      GREATER
    ;


hardwareThermalGenericParameter
    : identifier
      (
          COLON
          qualifiedName
      )?
      (
          ASSIGN
          expression
      )?
    ;


/* ============================================================================
 * 7. ABSTRACT TARGET REFERENCE
 * ============================================================================
 *
 * This is a symbolic target reference only.
 *
 * It does not identify a physical machine or device.
 * ========================================================================== */

hardwareThermalTargetClause
    : TARGET
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 8. THERMAL BODY
 * ========================================================================== */

hardwareThermalBody
    : LBRACE
      hardwareThermalItem*
      RBRACE
    ;


/* ============================================================================
 * 9. THERMAL ITEM DISPATCH
 * ============================================================================
 *
 * Every thermal body element has one owner.
 *
 * The ordering is deliberate:
 *
 *     transition
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     resource
 *     target
 *     assertion
 *     property
 *     named section
 *
 * Generic named sections are last so that dedicated constructs remain
 * structurally recognizable.
 * ========================================================================== */

hardwareThermalItem
    : hardwareThermalAttribute*
      (
          hardwareThermalTransition
        | hardwareThermalRequirement
        | hardwareThermalConstraint
        | hardwareThermalPreference
        | hardwareThermalHint
        | hardwareThermalCapabilityRequirement
        | hardwareThermalResourceRequirement
        | hardwareThermalTargetReference
        | hardwareThermalAssertion
        | hardwareThermalProperty
        | hardwareThermalNamedSection
      )
    ;


/* ============================================================================
 * 10. REQUIREMENT
 * ============================================================================
 *
 * Example:
 *
 *     requires temperature <= thermal_limit;
 *
 *     requires capability("thermal.monitoring");
 *
 * Requirement satisfaction is semantic, not syntactic.
 * ========================================================================== */

hardwareThermalRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 11. CONSTRAINT
 * ============================================================================
 *
 * Example:
 *
 *     constraint temperature <= thermal_limit;
 *
 * The generic expression system remains authoritative.
 * ========================================================================== */

hardwareThermalConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 12. PREFERENCE
 * ============================================================================
 *
 * Preferences are advisory optimization intent.
 *
 * They MUST NOT be treated as mandatory constraints by the parser.
 * ========================================================================== */

hardwareThermalPreference
    : PREFER
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 13. HINT
 * ============================================================================
 *
 * Hints are advisory information.
 * ========================================================================== */

hardwareThermalHint
    : HINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 14. CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Examples:
 *
 *     capability thermal.monitoring;
 *
 *     capability thermal.cooling = required_cooling;
 *
 * Capability resolution is downstream.
 * ========================================================================== */

hardwareThermalCapabilityRequirement
    : CAPABILITY
      qualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 15. RESOURCE REQUIREMENT
 * ============================================================================
 *
 * Example:
 *
 *     resource thermal.capacity >= required_capacity;
 *
 * The resource itself is not declared here.
 *
 * Resource ownership remains in the resource subsystem.
 * ========================================================================== */

hardwareThermalResourceRequirement
    : RESOURCE
      qualifiedName
      hardwareThermalComparisonOperator
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 16. TARGET REFERENCE
 * ============================================================================
 *
 * A target is referenced symbolically only.
 * ========================================================================== */

hardwareThermalTargetReference
    : TARGET
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 17. THERMAL PROPERTY
 * ============================================================================
 *
 * Property names remain open-world.
 *
 * Examples:
 *
 *     temperature <= thermal_limit;
 *
 *     ambient = ambient_temperature;
 *
 *     junction <= junction_limit;
 *
 *     hotspot <= hotspot_limit;
 *
 *     cooling >= required_cooling;
 *
 *     dissipation <= thermal_budget;
 *
 * Known universal resource words may also appear as thermal property names
 * where they are already reserved by the language.
 * ========================================================================== */

hardwareThermalProperty
    : hardwareThermalPropertyName
      hardwareThermalPropertyOperator
      expression
      SEMICOLON
    ;


hardwareThermalPropertyName
    : identifier
    | POWER
    | ENERGY
    | CAPACITY
    | AVAILABILITY
    | PERFORMANCE
    | LATENCY
    | THROUGHPUT
    | BANDWIDTH
    | RELIABILITY
    | RESILIENCE
    | SCALABILITY
    ;


/* ============================================================================
 * 18. PROPERTY OPERATOR
 * ============================================================================
 *
 * Assignment and relational operators are sufficient for thermal properties.
 *
 * Semantic interpretation is downstream.
 * ========================================================================== */

hardwareThermalPropertyOperator
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/* ============================================================================
 * 19. COMPARISON OPERATOR
 * ============================================================================
 */

hardwareThermalComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/* ============================================================================
 * 20. ASSERTION
 * ============================================================================
 *
 * Example:
 *
 *     assert temperature <= thermal_limit;
 *
 * This is a source-level assertion.
 *
 * It does not perform hardware control.
 * ========================================================================== */

hardwareThermalAssertion
    : ASSERT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 21. NAMED THERMAL SECTION
 * ============================================================================
 *
 * Thermal concepts evolve over time.
 *
 * Instead of adding a permanent keyword for every possible thermal concept,
 * named sections remain open-world.
 *
 * Examples:
 *
 *     state nominal {
 *         temperature <= nominal_limit;
 *     }
 *
 *     domain processor {
 *         temperature <= processor_limit;
 *     }
 *
 *     scaling workload {
 *         temperature = thermal_model(workload);
 *     }
 *
 *     measurement telemetry {
 *         temperature = measured_temperature;
 *     }
 *
 * The first identifier is interpreted semantically.
 *
 * The grammar does not create a fixed catalogue of thermal section kinds.
 * ========================================================================== */

hardwareThermalNamedSection
    : identifier
      identifier?
      hardwareThermalBody
    ;


/* ============================================================================
 * 22. THERMAL TRANSITION
 * ============================================================================
 *
 * Example:
 *
 *     transition nominal -> throttled {
 *         temperature >= throttle_limit;
 *     }
 *
 * `transition` remains contextual rather than becoming a permanent global
 * keyword.
 *
 * The semantic layer determines whether the leading identifier denotes a
 * transition construct in the current thermal context.
 * ========================================================================== */

hardwareThermalTransition
    : identifier
      identifier
      THIN_ARROW
      identifier
      hardwareThermalBody
    ;


/* ============================================================================
 * 23. DETERMINISM
 * ============================================================================
 *
 * Parsing is purely syntactic.
 *
 * It MUST NOT depend on:
 *
 *     hardware availability
 *     target temperature
 *     runtime state
 *     wall-clock time
 *     network state
 *     filesystem state
 *     random state
 *     scheduler state
 *     device discovery
 *
 * Identical source and identical lexical/parser configuration must produce
 * identical parser structure.
 *
 *
 * ============================================================================
 * 24. SEMANTIC INTEGRATION
 * ============================================================================
 *
 * The frontend must normalize thermal constructs into the existing
 * domain-neutral semantic model.
 *
 * Conceptual mappings:
 *
 *     hardwareThermalDeclaration
 *         -> HardwareThermalDeclaration
 *
 *     hardwareThermalRequirement
 *         -> Requirement
 *
 *     hardwareThermalConstraint
 *         -> Constraint
 *
 *     hardwareThermalPreference
 *         -> Preference
 *
 *     hardwareThermalHint
 *         -> Hint
 *
 *     hardwareThermalCapabilityRequirement
 *         -> CapabilityRequirement
 *
 *     hardwareThermalResourceRequirement
 *         -> ResourceRequirement
 *
 *     hardwareThermalTargetReference
 *         -> TargetReference
 *
 *     hardwareThermalProperty
 *         -> Property
 *
 *     hardwareThermalAssertion
 *         -> Assertion
 *
 *     hardwareThermalNamedSection
 *         -> NamedSection
 *
 *     hardwareThermalTransition
 *         -> Transition
 *
 * The grammar MUST NOT require a thermal-specific Rust AST hierarchy unless
 * the existing AST architecture already has an appropriate generic
 * declaration/property/constraint representation.
 *
 *
 * ============================================================================
 * 25. TYPE INTEGRATION
 * ============================================================================
 *
 * Thermal quantities are expressions.
 *
 * Dimensional correctness belongs to semantic/type analysis.
 *
 * Examples:
 *
 *     temperature <= thermal_limit
 *
 *     cooling >= required_cooling
 *
 *     dissipation <= thermal_budget
 *
 * The grammar does not decide whether the operands have compatible units.
 *
 *
 * ============================================================================
 * 26. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Thermal resource requirements feed:
 *
 *     resource analysis
 *         ->
 *     capability negotiation
 *         ->
 *     execution planning
 *
 * The grammar never performs resource discovery.
 *
 *
 * ============================================================================
 * 27. POWER INTEGRATION
 * ============================================================================
 *
 * Power and thermal intent may coexist:
 *
 *     power <= power_budget;
 *
 *     temperature <= thermal_limit;
 *
 * The semantic/compiler layers may jointly analyze them.
 *
 * This grammar does not duplicate power semantics.
 *
 *
 * ============================================================================
 * 28. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Thermal intent can constrain quantum execution:
 *
 *     requires capability("quantum.measurement");
 *
 *     temperature <= thermal_limit;
 *
 *     resource thermal.capacity >= required_capacity;
 *
 * The pipeline remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *
 * Thermal grammar does not create a quantum representation.
 *
 *
 * ============================================================================
 * 29. HDL INTEGRATION
 * ============================================================================
 *
 * HDL physical intent may ultimately consume the same semantic thermal
 * representation.
 *
 * The relationship is:
 *
 *     HDL thermal intent
 *         ->
 *     semantic thermal/resource contract
 *         ->
 *     hardware compilation
 *
 * This prevents incompatible thermal models from being created for HDL and
 * general hardware source.
 *
 *
 * ============================================================================
 * 30. EXECUTION / HAL INTEGRATION
 * ============================================================================
 *
 * Runtime and HAL layers may use semantic thermal intent to:
 *
 *     inspect capabilities
 *     inspect resources
 *     evaluate thermal state
 *     select schedules
 *     throttle
 *     migrate
 *     reschedule
 *     recover
 *     reject an infeasible realization
 *
 * None of these are parser operations.
 *
 *
 * ============================================================================
 * 31. ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Thermal constraints may participate downstream in:
 *
 *     fallback
 *     retry
 *     recovery
 *     adaptive scheduling
 *     workload shaping
 *     placement selection
 *     target selection
 *
 * The grammar remains unchanged as targets and execution strategies evolve.
 *
 *
 * ============================================================================
 * 32. PROVENANCE
 * ============================================================================
 *
 * Semantic thermal analysis should preserve:
 *
 *     source declaration
 *     source property
 *     source expression
 *     normalized expression
 *     resource resolution
 *     capability resolution
 *     policy decision
 *     target evaluation
 *     selected realization
 *
 * This grammar only preserves syntactic structure needed for that process.
 *
 *
 * ============================================================================
 * 33. SECURITY
 * ============================================================================
 *
 * Thermal syntax does not authorize access to:
 *
 *     sensors
 *     cooling controllers
 *     power controllers
 *     operating-system thermal interfaces
 *     physical hardware
 *
 * Authorization belongs to the security/runtime/HAL layers.
 *
 *
 * ============================================================================
 * 34. HARD-CODING AUDIT
 * ============================================================================
 *
 * No fixed thermal capacities are encoded.
 *
 * No fixed hardware capacities are encoded.
 *
 * No physical device catalogue is encoded.
 *
 * No physical IDs are encoded.
 *
 * No finite list of thermal states is encoded.
 *
 * No finite list of thermal profiles is encoded.
 *
 * No finite list of thermal properties is encoded.
 *
 * No finite number of thermal domains is encoded.
 *
 * No fixed machine size is encoded.
 *
 *
 * ============================================================================
 * 35. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * These forms must parse:
 *
 *     thermal contract basic {
 *         temperature <= thermal_limit;
 *     }
 *
 *     thermal profile nominal {
 *         ambient = ambient_temperature;
 *         junction <= junction_limit;
 *     }
 *
 *     thermal symbolic<limit> {
 *         requires temperature <= limit;
 *     }
 *
 *     thermal capabilities {
 *         capability thermal.monitoring;
 *         requires capability("thermal.cooling");
 *     }
 *
 *     thermal resources {
 *         resource thermal.capacity >= required_capacity;
 *     }
 *
 *     thermal states {
 *         state nominal {
 *             temperature <= nominal_limit;
 *         }
 *
 *         state throttled {
 *             temperature <= throttled_limit;
 *         }
 *
 *         transition nominal -> throttled {
 *             temperature >= throttle_limit;
 *         }
 *     }
 *
 *     thermal scaling {
 *         scaling workload {
 *             temperature = thermal_model(workload);
 *         }
 *     }
 *
 *     thermal measurement {
 *         measurement telemetry {
 *             temperature = measured_temperature;
 *         }
 *     }
 *
 *     thermal joint {
 *         power <= power_budget;
 *         temperature <= thermal_limit;
 *         resource thermal.capacity >= required_capacity;
 *         capability thermal.monitoring;
 *     }
 *
 *
 * ============================================================================
 * 36. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These must be rejected syntactically:
 *
 *     thermal contract broken {
 *         requires;
 *     }
 *
 *     thermal contract broken {
 *         temperature <= ;
 *     }
 *
 *     thermal contract broken {
 *         resource;
 *     }
 *
 *     thermal contract broken {
 *         capability;
 *     }
 *
 *     thermal contract broken {
 *         transition -> state {
 *         }
 *     }
 *
 *
 * ============================================================================
 * 37. SEMANTIC-NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These should parse but may be rejected by semantic analysis:
 *
 *     thermal contract semantic {
 *         temperature <= memory;
 *     }
 *
 *     thermal contract semantic {
 *         cooling >= required_temperature;
 *     }
 *
 *     thermal contract semantic {
 *         resource thermal.unknown >= value;
 *     }
 *
 *     thermal contract semantic {
 *         capability thermal.unknown;
 *     }
 *
 *     thermal contract semantic {
 *         temperature <= target.unsupported_limit;
 *     }
 *
 * Parsing must not confuse semantic infeasibility with syntax failure.
 *
 *
 * ============================================================================
 * 38. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     symbolic quantities
 *     computed quantities
 *     very large numeric values
 *     very small numeric values
 *     zero values
 *     negative expressions where semantically meaningful
 *     deeply qualified names
 *     deeply nested expressions
 *     many properties
 *     many declarations
 *     many sections
 *     many transitions
 *     mixed power/thermal constraints
 *     mixed thermal/resource/capability constraints
 *     classical + quantum + HDL thermal intent
 *
 *
 * ============================================================================
 * 39. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The test suite must increase:
 *
 *     source size
 *     declaration count
 *     property count
 *     section count
 *     transition count
 *     expression complexity
 *
 * without changing this grammar.
 *
 * Test size is a test parameter, not a grammar constant.
 *
 *
 * ============================================================================
 * 40. COMPATIBILITY
 * ============================================================================
 *
 * Existing hardware grammar files remain independently owned:
 *
 *     hardware.g4
 *     power.g4
 *     timing.g4
 *     resources.g4
 *     capabilities.g4
 *     constraints.g4
 *     target-related grammars
 *
 * This file introduces one canonical thermal declaration boundary:
 *
 *     hardwareThermalDeclaration
 *
 * Hardware composition must delegate to this rule.
 *
 * No existing major filename needs to be renamed.
 *
 *
 * ============================================================================
 * 41. BUILD / RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust.
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * The surrounding parser/compiler implementation must use safe Rust.
 *
 * This grammar does not require target-specific Rust behavior.
 *
 *
 * ============================================================================
 * 42. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It imports Expressions.
 *
 *     [ ] It consumes only ZamaniLexer tokens.
 *
 *     [ ] It defines exactly one public thermal declaration entry point.
 *
 *     [ ] Thermal declarations are composed through hardware.g4.
 *
 *     [ ] No local expression grammar exists.
 *
 *     [ ] No local identifier grammar exists.
 *
 *     [ ] No local qualified-name grammar exists.
 *
 *     [ ] No duplicate power grammar exists.
 *
 *     [ ] No duplicate timing grammar exists.
 *
 *     [ ] No duplicate resource grammar exists.
 *
 *     [ ] No duplicate capability grammar exists.
 *
 *     [ ] No physical hardware identity is encoded.
 *
 *     [ ] No hardware capacity ceiling is encoded.
 *
 *     [ ] Thermal property names remain extensible.
 *
 *     [ ] Thermal section names remain extensible.
 *
 *     [ ] Thermal quantities remain symbolic expressions.
 *
 *     [ ] Thermal semantics remain downstream.
 *
 *     [ ] No thermal-specific IR is introduced.
 *
 *     [ ] quantum::ir remains the quantum boundary.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Semantic-negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Generated Rust integration works with Rust 1.97+.
 *
 *
 * ============================================================================
 * FINAL OWNERSHIP INVARIANT
 * ============================================================================
 *
 *     thermal.g4
 *         owns thermal SOURCE SYNTAX
 *
 *     semantic layer
 *         owns thermal MEANING
 *
 *     resource/capability subsystem
 *         owns thermal FEASIBILITY INPUTS
 *
 *     compiler
 *         owns thermal-AWARE REALIZATION
 *
 *     runtime/HAL
 *         owns thermal OBSERVATION/ENFORCEMENT
 *
 *     target
 *         owns PHYSICAL THERMAL REALIZATION
 *
 * These responsibilities must never be collapsed into this grammar.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */