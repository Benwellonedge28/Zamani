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
 *     CANONICAL HARDWARE-CONTRACT THERMAL INTENT GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     Action-free ANTLR parser grammar.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No I/O.
 *     No hardware discovery.
 *     No runtime execution.
 *     No unsafe implementation requirement.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file defines source-level, target-independent thermal intent.
 *
 * Thermal intent may describe:
 *
 *     - thermal requirements;
 *     - thermal constraints;
 *     - thermal preferences;
 *     - thermal hints;
 *     - thermal properties;
 *     - temperature relationships;
 *     - thermal budgets;
 *     - thermal envelopes;
 *     - thermal profiles;
 *     - logical thermal domains;
 *     - logical thermal states;
 *     - thermal transitions;
 *     - thermal measurements/contracts;
 *     - thermal scaling relationships;
 *     - cooling requirements;
 *     - thermal capability requirements;
 *     - thermal resource requirements;
 *     - thermal assertions;
 *     - extensible thermal metadata.
 *
 * The grammar expresses WHAT thermal property is required or preferred.
 *
 * It does not prescribe HOW a particular target realizes that property.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type analysis
 *          +--> dimensional/unit analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> thermal-contract validation
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     canonical compiler IR
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> placement
 *          +--> routing
 *          +--> synthesis
 *          +--> resilience
 *          |
 *          v
 *     HAL / target realization
 *          |
 *          v
 *     runtime / deployment
 *
 * This grammar therefore remains above physical realization.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hardware thermal declarations;
 *     - thermal contracts;
 *     - thermal requirements;
 *     - thermal constraints;
 *     - thermal preferences;
 *     - thermal hints;
 *     - thermal properties;
 *     - thermal profiles;
 *     - thermal logical domains;
 *     - thermal logical states;
 *     - thermal transitions;
 *     - thermal measurement contracts;
 *     - thermal scaling contracts;
 *     - thermal capability requirements;
 *     - thermal resource requirements;
 *     - thermal assertions;
 *     - thermal metadata;
 *     - target-independent thermal intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - the THERMAL token;
 *     - identifiers;
 *     - qualified names;
 *     - numeric literals;
 *     - unit lexical syntax;
 *     - general expression syntax;
 *     - general type syntax;
 *     - universal resource declarations;
 *     - universal capability declarations;
 *     - power semantics;
 *     - energy semantics;
 *     - timing semantics;
 *     - clock semantics;
 *     - cooling-device implementation;
 *     - sensors;
 *     - physical thermal zones;
 *     - physical device IDs;
 *     - physical placement;
 *     - routing;
 *     - scheduling algorithms;
 *     - optimization algorithms;
 *     - calibration;
 *     - hardware discovery;
 *     - device drivers;
 *     - runtime thermal enforcement;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * 4. CRITICAL OWNERSHIP BOUNDARIES
 * ============================================================================
 *
 * POWER
 * -----
 *
 * Power intent remains owned by:
 *
 *     grammar/hardware/power.g4
 *
 * Thermal grammar may reference power as a property or expression, but must
 * not redefine power contracts.
 *
 *
 * TIMING
 * ------
 *
 * Timing intent remains owned by:
 *
 *     grammar/hardware/timing.g4
 *
 * Thermal grammar may express time-dependent thermal relationships through
 * normal expressions, but it does not create a second timing grammar.
 *
 *
 * RESOURCES
 * ---------
 *
 * Universal resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * Hardware resource integration remains owned by:
 *
 *     grammar/hardware/resources.g4
 *
 * This file only consumes the resource/capability concepts.
 *
 *
 * CAPABILITIES
 * ------------
 *
 * Capability identity remains owned by the canonical capability model.
 *
 * This file may require capabilities such as:
 *
 *     capability("thermal.management")
 *     capability("thermal.monitoring")
 *     capability("thermal.cooling")
 *
 * but does not define their implementation.
 *
 *
 * HDL
 * ---
 *
 * HDL thermal intent remains owned by:
 *
 *     grammar/hdl/physical-intent.g4
 *
 * HDL thermal syntax may lower into the same semantic thermal/resource model.
 *
 *
 * QUANTUM
 * -------
 *
 * Quantum thermal/noise semantics remain downstream.
 *
 * Quantum source ultimately continues through:
 *
 *     quantum::ir
 *          |
 *          v
 *     ZQN / resilience / scheduling
 *
 * This grammar MUST NOT create a second quantum IR.
 *
 * ============================================================================
 * 5. LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST NOT define lexer rules.
 *
 * The canonical parser dependency is:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * `ZamaniLexer` is the production lexer.
 *
 * `ZamaniTokens` is the lexical composition vocabulary and is not the parser's
 * direct token source.
 *
 * The required lexical addition for this grammar is:
 *
 *     THERMAL : 'thermal' ;
 *
 * in:
 *
 *     grammar/lexer/keywords.g4
 *
 * That token is intentionally the only new thermal-specific reserved word.
 *
 * Property names such as:
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
 *
 * remain identifiers.
 *
 * This prevents keyword proliferation.
 *
 * ============================================================================
 * 6. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `hardwareThermalDeclaration` is the only public declaration entry point
 * exported by this grammar.
 *
 * The hardware composition grammar must delegate to it rather than copying
 * thermal rules.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareThermalParser;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 7. PUBLIC DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     thermal contract compute_thermal {
 *         temperature <= maximum_temperature;
 *     }
 *
 *     thermal compute_thermal {
 *         temperature <= maximum_temperature;
 *     }
 *
 *     thermal profile operating_profile {
 *         ambient = ambient_temperature;
 *     }
 *
 * The declaration name is symbolic.
 *
 * It is never interpreted as a physical device identifier by this grammar.
 * ============================================================================
 */

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
 * 8. DECLARATION KIND
 * ============================================================================
 *
 * Only stable structural kinds are reserved here.
 *
 * Additional thermal concepts remain contextual/open-world constructs.
 * ============================================================================
 */

hardwareThermalDeclarationKind
    : CONTRACT
    | PROFILE
    ;


/* ============================================================================
 * 9. VISIBILITY
 * ============================================================================
 */

hardwareThermalVisibility
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/* ============================================================================
 * 10. MODIFIERS
 * ============================================================================
 */

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
 * 11. ATTRIBUTES
 * ============================================================================
 *
 * Attribute names remain qualified names.
 *
 * Vendor-specific or technology-specific thermal metadata therefore does not
 * require permanent global keywords.
 * ============================================================================
 */

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
 * 12. GENERIC PARAMETERS
 * ============================================================================
 *
 * Thermal contracts may depend on arbitrary symbolic program parameters.
 *
 * Example:
 *
 *     thermal contract thermal_policy<limit, workload> {
 *         temperature <= limit;
 *     }
 *
 * No finite hardware scale is encoded.
 * ============================================================================
 */

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
          hardwareThermalGenericBound
      )?
      (
          ASSIGN
          expression
      )?
    ;


hardwareThermalGenericBound
    : qualifiedName
    ;


/* ============================================================================
 * 13. ABSTRACT TARGET
 * ============================================================================
 *
 * A target is symbolic.
 *
 * It does not identify a physical machine, device, socket, board, sensor,
 * thermal zone, GPU, QPU, FPGA, CPU, or node.
 * ============================================================================
 */

hardwareThermalTargetClause
    : TARGET
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 14. THERMAL BODY
 * ============================================================================
 */

hardwareThermalBody
    : LBRACE
      hardwareThermalItem*
      RBRACE
    ;


/* ============================================================================
 * 15. THERMAL ITEM DISPATCH
 * ============================================================================
 *
 * Every thermal-specific construct enters through this single dispatcher.
 *
 * The hardware composition grammar must not duplicate these alternatives.
 * ============================================================================
 */

hardwareThermalItem
    : hardwareThermalAttribute*
      (
          hardwareThermalRequirement
        | hardwareThermalConstraint
        | hardwareThermalPreference
        | hardwareThermalHint
        | hardwareThermalCapabilityRequirement
        | hardwareThermalResourceRequirement
        | hardwareThermalTargetReference
        | hardwareThermalProperty
        | hardwareThermalAssertion
        | hardwareThermalProfile
        | hardwareThermalContract
        | hardwareThermalNamedSection
        | hardwareThermalTransition
      )
    ;


/* ============================================================================
 * 16. REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * Example:
 *
 *     requires temperature <= maximum_temperature;
 *
 * or:
 *
 *     requires capability("thermal.monitoring");
 *
 * The grammar does not determine whether a target satisfies it.
 * ============================================================================
 */

hardwareThermalRequirement
    : REQUIRES
      hardwareThermalRequirementExpression
      SEMICOLON
    ;


hardwareThermalRequirementExpression
    : expression
    ;


/* ============================================================================
 * 17. CONSTRAINT
 * ============================================================================
 *
 * A constraint is mandatory for a legal realization.
 * ============================================================================
 */

hardwareThermalConstraint
    : CONSTRAINT
      hardwareThermalConstraintExpression
      SEMICOLON
    ;


hardwareThermalConstraintExpression
    : expression
    ;


/* ============================================================================
 * 18. PREFERENCE
 * ============================================================================
 *
 * A preference is non-mandatory optimization guidance.
 * ============================================================================
 */

hardwareThermalPreference
    : PREFER
      hardwareThermalPreferenceExpression
      SEMICOLON
    ;


hardwareThermalPreferenceExpression
    : expression
    ;


/* ============================================================================
 * 19. HINT
 * ============================================================================
 *
 * A hint is advisory and must not be treated as a semantic requirement.
 * ============================================================================
 */

hardwareThermalHint
    : HINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 20. CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * Example:
 *
 *     capability thermal.monitoring;
 *
 *     capability thermal.cooling = required_cooling;
 *
 * Capability satisfaction is downstream.
 * ============================================================================
 */

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
 * 21. RESOURCE REQUIREMENT
 * ============================================================================
 *
 * This references the canonical resource model.
 *
 * Examples:
 *
 *     resource thermal.capacity >= required_capacity;
 *
 *     resource cooling.capacity >= required_cooling;
 *
 * No resource maximum is encoded.
 * ============================================================================
 */

hardwareThermalResourceRequirement
    : RESOURCE
      qualifiedName
      (
          hardwareThermalComparisonOperator
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 22. TARGET REFERENCE
 * ============================================================================
 *
 * This references an abstract target only.
 * ============================================================================
 */

hardwareThermalTargetReference
    : TARGET
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 23. THERMAL PROPERTY
 * ============================================================================
 *
 * Property names remain open.
 *
 * This is the primary extensibility mechanism for thermal technology.
 *
 * Examples:
 *
 *     temperature <= maximum_temperature;
 *     ambient = ambient_temperature;
 *     junction <= junction_limit;
 *     hotspot <= hotspot_limit;
 *     gradient <= allowed_gradient;
 *     cooling >= required_cooling;
 *     dissipation <= thermal_budget;
 *
 * Reserved lexical words already meaningful to the language may also appear
 * as thermal property names through `hardwareThermalPropertyName`.
 * ============================================================================
 */

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


hardwareThermalPropertyOperator
    : ASSIGN
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/* ============================================================================
 * 24. ASSERTIONS
 * ============================================================================
 *
 * Assertions are source-level contracts.
 *
 * They do not execute thermal management.
 * ============================================================================
 */

hardwareThermalAssertion
    : ASSERT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 25. THERMAL PROFILE
 * ============================================================================
 *
 * Profiles are named collections of thermal intent.
 *
 * Examples:
 *
 *     profile nominal {
 *         temperature <= nominal_limit;
 *     }
 *
 *     profile safe {
 *         temperature <= safe_limit;
 *     }
 *
 * Profile semantics remain downstream.
 * ============================================================================
 */

hardwareThermalProfile
    : PROFILE
      identifier
      hardwareThermalBody
    ;


/* ============================================================================
 * 26. NESTED THERMAL CONTRACT
 * ============================================================================
 *
 * Allows compositional thermal contracts.
 * ============================================================================
 */

hardwareThermalContract
    : CONTRACT
      identifier
      hardwareThermalGenericParameters?
      hardwareThermalBody
    ;


/* ============================================================================
 * 27. CONTEXTUAL THERMAL SECTIONS
 * ============================================================================
 *
 * Thermal technology evolves.
 *
 * Instead of making every future thermal concept a global keyword, the
 * grammar permits named thermal sections.
 *
 * Examples:
 *
 *     domain package {
 *         temperature <= package_limit;
 *     }
 *
 *     state nominal {
 *         temperature <= nominal_limit;
 *     }
 *
 *     measurement junction {
 *         property temperature <= sensor_limit;
 *     }
 *
 *     scaling workload {
 *         temperature = thermal_model(workload);
 *     }
 *
 * The first identifier is interpreted semantically.
 *
 * Stable semantic section kinds may include:
 *
 *     domain
 *     state
 *     measurement
 *     scaling
 *     envelope
 *     cooling
 *     sensor
 *     hotspot
 *     junction
 *     ambient
 *     custom.domain
 *
 * New concepts can therefore be added without continually expanding the
 * global keyword registry.
 *
 * Semantic validation MUST reject unsupported section kinds when appropriate.
 * ============================================================================
 */

hardwareThermalNamedSection
    : identifier
      identifier
      hardwareThermalBody
    ;


/* ============================================================================
 * 28. THERMAL TRANSITION
 * ============================================================================
 *
 * Thermal state transitions are logical relationships.
 *
 * Example:
 *
 *     transition nominal -> throttled {
 *         temperature >= throttle_threshold;
 *     }
 *
 * `transition` is deliberately contextual rather than a global keyword.
 *
 * `->` is the canonical THIN_ARROW operator.
 *
 * This grammar does not implement a thermal controller.
 * ============================================================================
 */

hardwareThermalTransition
    : identifier
      identifier
      THIN_ARROW
      identifier
      hardwareThermalBody
    ;


/* ============================================================================
 * 29. COMPARISON OPERATORS
 * ============================================================================
 *
 * These are canonical lexer tokens.
 *
 * No thermal-specific operators are introduced.
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
 * 30. EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Thermal grammar delegates all expression semantics to the canonical
 * expression grammar.
 *
 * This means the following may be represented symbolically:
 *
 *     fixed values;
 *     variables;
 *     generic parameters;
 *     functions;
 *     derived values;
 *     configuration values;
 *     resource-derived values;
 *     runtime observations;
 *     negotiated values;
 *     mathematical expressions;
 *     conditional expressions;
 *     domain-specific semantic expressions.
 *
 * The thermal grammar does not create another expression language.
 * ============================================================================
 */

hardwareThermalExpression
    : expression
    ;


/* ============================================================================
 * 31. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     1. Resolve thermal declaration names.
 *
 *     2. Resolve thermal property names.
 *
 *     3. Resolve capability references.
 *
 *     4. Resolve resource references.
 *
 *     5. Resolve target references.
 *
 *     6. Validate expression types.
 *
 *     7. Validate physical dimensions where the semantic type system supports
 *        dimensional analysis.
 *
 *     8. Validate temperature quantities.
 *
 *     9. Validate thermal rates and gradients.
 *
 *    10. Validate energy/power relationships where explicitly expressed.
 *
 *    11. Distinguish requirement from constraint.
 *
 *    12. Distinguish preference from requirement.
 *
 *    13. Distinguish hint from preference.
 *
 *    14. Detect contradictory thermal constraints.
 *
 *    15. Preserve unresolved symbolic requirements when target information
 *        is unavailable.
 *
 *    16. Determine target feasibility downstream.
 *
 *    17. Never convert a source value into a compiler-wide maximum.
 *
 *    18. Never infer hardware limits from grammar constructs.
 *
 *    19. Preserve portability metadata.
 *
 *    20. Preserve source spans.
 *
 * ============================================================================
 * 32. DIMENSIONAL SEMANTICS
 * ============================================================================
 *
 * The grammar intentionally does not enumerate units.
 *
 * Thermal expressions may therefore eventually support whatever quantity/unit
 * system the canonical Zamani type/literal system establishes.
 *
 * Examples may include:
 *
 *     temperature <= limit;
 *     temperature <= 300 K;
 *     temperature <= 27 C;
 *     thermal_rate <= rate;
 *     heat <= energy;
 *
 * Unit compatibility is a semantic/type-system responsibility.
 *
 * This file MUST NOT hard-code a finite unit list.
 *
 * ============================================================================
 * 33. POWER / ENERGY RELATIONSHIP
 * ============================================================================
 *
 * Thermal and power semantics are related but not identical.
 *
 * Examples:
 *
 *     power <= power_budget;
 *     energy <= energy_budget;
 *     temperature <= thermal_limit;
 *
 * Power syntax belongs to:
 *
 *     grammar/hardware/power.g4
 *
 * Energy/resource syntax belongs to the corresponding resource model.
 *
 * Thermal syntax may reference those semantic values without redefining them.
 *
 * ============================================================================
 * 34. THERMAL DOMAIN MODEL
 * ============================================================================
 *
 * Thermal domains are logical semantic regions.
 *
 * They may represent:
 *
 *     package
 *     die
 *     subsystem
 *     module
 *     accelerator
 *     memory
 *     interconnect
 *     quantum subsystem
 *     cooling region
 *     logical partition
 *     future thermal abstraction
 *
 * They do NOT imply:
 *
 *     physical coordinates;
 *     physical sensor IDs;
 *     physical heat-sink IDs;
 *     fixed chip topology;
 *     fixed number of domains.
 *
 * ============================================================================
 * 35. THERMAL STATE MODEL
 * ============================================================================
 *
 * State names remain symbolic.
 *
 * The grammar intentionally does not enumerate:
 *
 *     NORMAL
 *     HOT
 *     CRITICAL
 *     COLD
 *     THROTTLED
 *     EMERGENCY
 *
 * as universal language states.
 *
 * A target or dialect may define semantic state vocabularies downstream.
 *
 * ============================================================================
 * 36. THERMAL MEASUREMENT MODEL
 * ============================================================================
 *
 * Measurement sections describe observable contracts.
 *
 * They do not perform measurements.
 *
 * Measurement providers, sensors, sampling rates, telemetry transport, and
 * hardware interfaces remain runtime/HAL responsibilities.
 *
 * ============================================================================
 * 37. THERMAL SCALING MODEL
 * ============================================================================
 *
 * Thermal behavior may depend on arbitrary program/resource scale.
 *
 * Examples:
 *
 *     temperature = f(workload);
 *
 *     heat <= available_cooling(workload);
 *
 *     cooling >= required_cooling(load);
 *
 * The grammar imposes no finite number of scaling points.
 *
 * ============================================================================
 * 38. COOLING
 * ============================================================================
 *
 * Cooling is represented as semantic intent.
 *
 * Examples:
 *
 *     cooling >= required_cooling;
 *
 *     requires capability("thermal.cooling");
 *
 *     prefer cooling_efficiency >= desired_efficiency;
 *
 * The grammar does not select:
 *
 *     fan;
 *     pump;
 *     heatsink;
 *     liquid loop;
 *     cryogenic system;
 *     Peltier device;
 *     physical cooling controller.
 *
 * Such realization belongs downstream.
 *
 * ============================================================================
 * 39. HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Thermal intent must remain valid across:
 *
 *     atom-scale systems;
 *     embedded systems;
 *     microcontrollers;
 *     CPUs;
 *     multicore CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     edge systems;
 *     future architectures.
 *
 * The source describes requirements.
 *
 * The compiler/runtime discovers the realization.
 *
 * ============================================================================
 * 40. POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Thermal syntax MUST preserve this property.
 *
 * The same program may declare:
 *
 *     requires temperature <= thermal_limit;
 *
 * and allow different implementations to satisfy the contract using
 * different:
 *
 *     schedules;
 *     mappings;
 *     frequencies;
 *     workloads;
 *     parallelism;
 *     accelerators;
 *     cooling strategies;
 *     target devices.
 *
 * No physical device is selected by this grammar.
 *
 * ============================================================================
 * 41. ABSOLUTE SCALABILITY RULE
 * ============================================================================
 *
 * This grammar MUST NOT define:
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
 * It must also not encode equivalent finite bounds through parser
 * alternatives.
 *
 * Collections therefore use:
 *
 *     *
 *     +
 *
 * and quantities use expressions.
 *
 * ============================================================================
 * 42. NO PHYSICAL DEVICE IDENTIFIERS
 * ============================================================================
 *
 * This grammar must not establish universal syntax requiring:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     node0
 *     sensor0
 *     thermal_zone0
 *
 * as physical identities.
 *
 * If a target-dependent interoperability dialect intentionally exposes a
 * physical identifier, that construct must be explicitly governed by the
 * target/dialect compatibility model.
 *
 * ============================================================================
 * 43. AST CONTRACT
 * ============================================================================
 *
 * The grammar lowers into the existing domain-neutral frontend AST.
 *
 * Conceptual mapping:
 *
 *     hardwareThermalDeclaration
 *         -> Declaration
 *         -> HardwareThermalContract
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
 *     hardwareThermalProperty
 *         -> Property
 *
 *     hardwareThermalProfile
 *         -> Profile
 *
 *     hardwareThermalNamedSection
 *         -> NamedThermalSection
 *
 *     hardwareThermalTransition
 *         -> ThermalTransition
 *
 * Every AST representation must preserve:
 *
 *     source span;
 *     declaration identity;
 *     semantic category;
 *     expression identity;
 *     attributes;
 *     generic parameters;
 *     target reference.
 *
 * The grammar MUST NOT require a new Rust AST hierarchy solely because this
 * file exists.
 *
 * ============================================================================
 * 44. SEMANTIC MODEL CONTRACT
 * ============================================================================
 *
 * The semantic representation should preserve:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     resource
 *     target
 *     property
 *     profile
 *     state
 *     domain
 *     transition
 *     measurement
 *     scaling
 *
 * These categories MUST remain distinguishable.
 *
 * In particular:
 *
 *     requirement != preference
 *     preference != hint
 *     constraint != implementation decision
 *
 * ============================================================================
 * 45. IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO thermal-specific IR.
 *
 * It must not introduce:
 *
 *     ThermalIR
 *     HardwareThermalIR
 *     ThermalControlIR
 *     ThermalDeviceIR
 *
 * merely because thermal syntax exists.
 *
 * Thermal intent lowers through the repository's canonical semantic/IR
 * boundary and is consumed by the appropriate hardware/resource/compiler
 * representations.
 *
 * ============================================================================
 * 46. COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages may consume thermal intent for:
 *
 *     resource negotiation;
 *     capability resolution;
 *     optimization;
 *     workload shaping;
 *     parallelism decisions;
 *     frequency selection;
 *     scheduling;
 *     placement;
 *     routing;
 *     accelerator selection;
 *     synthesis;
 *     deployment.
 *
 * The grammar itself makes none of these decisions.
 *
 * An unresolved thermal requirement must remain representable until sufficient
 * target information exists.
 *
 * ============================================================================
 * 47. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling may use thermal contracts to choose among valid schedules.
 *
 * For example:
 *
 *     schedule A
 *     schedule B
 *     schedule C
 *
 * may have different thermal consequences.
 *
 * The source grammar does not choose among them.
 *
 * Scheduling remains a downstream responsibility.
 *
 * ============================================================================
 * 48. POWER INTEGRATION
 * ============================================================================
 *
 * Power and thermal contracts may be jointly analyzed.
 *
 * Example:
 *
 *     power <= power_budget;
 *     temperature <= thermal_limit;
 *
 * A compiler may discover that:
 *
 *     lower power
 *     lower utilization
 *     different scheduling
 *     different placement
 *     different cooling
 *
 * satisfy the joint contract.
 *
 * The thermal grammar does not duplicate the power grammar.
 *
 * ============================================================================
 * 49. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum programs may carry thermal intent because real quantum systems can
 * have thermal constraints.
 *
 * The flow remains:
 *
 *     quantum source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC/resilience
 *          +--> ZQN
 *          |
 *          v
 *     HAL
 *
 * Thermal intent participates as a resource/capability constraint.
 *
 * It does not create a quantum thermal IR.
 *
 * ============================================================================
 * 50. HDL INTEGRATION
 * ============================================================================
 *
 * HDL physical intent may express thermal requirements.
 *
 * HDL syntax must lower into the same semantic hardware/resource contract
 * model rather than creating an incompatible thermal language.
 *
 * The relationship is:
 *
 *     HDL thermal intent
 *          |
 *          v
 *     semantic thermal/resource contract
 *          |
 *          v
 *     hardware compiler
 *
 * ============================================================================
 * 51. RUNTIME / HAL INTEGRATION
 * ============================================================================
 *
 * Runtime/HAL may use thermal intent to:
 *
 *     inspect capabilities;
 *     inspect available resources;
 *     monitor target state;
 *     select valid execution policies;
 *     throttle;
 *     migrate;
 *     reschedule;
 *     recover;
 *     reject an unsatisfied realization.
 *
 * Runtime enforcement is NOT parser behavior.
 *
 * ============================================================================
 * 52. DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     dialect set;
 *     lexical configuration;
 *
 * the grammar must produce the same parse structure.
 *
 * Parsing must not depend on:
 *
 *     CPU availability;
 *     GPU availability;
 *     QPU availability;
 *     temperature;
 *     wall-clock time;
 *     runtime state;
 *     environment variables;
 *     network state;
 *     random state;
 *     target discovery.
 *
 * ============================================================================
 * 53. DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics must preserve:
 *
 *     source span;
 *     offending token;
 *     expected syntax;
 *     language version;
 *     dialect context.
 *
 * Semantic diagnostics are downstream and include:
 *
 *     invalid thermal dimension;
 *     incompatible units;
 *     contradictory constraints;
 *     unsatisfied thermal requirement;
 *     unavailable thermal capability;
 *     invalid thermal resource;
 *     invalid target relationship;
 *     unsupported semantic section;
 *     impossible realization.
 *
 * ============================================================================
 * 54. SECURITY
 * ============================================================================
 *
 * Thermal syntax does not grant access to:
 *
 *     sensors;
 *     cooling devices;
 *     power controllers;
 *     physical hardware;
 *     operating-system thermal interfaces.
 *
 * Authorization and device access remain downstream security/runtime
 * responsibilities.
 *
 * Attributes and named thermal sections must not provide a parser-level escape
 * into backend execution.
 *
 * ============================================================================
 * 55. PERFORMANCE
 * ============================================================================
 *
 * The grammar uses:
 *
 *     unbounded repetition;
 *     symbolic expressions;
 *     open-world property names;
 *     open-world contextual sections.
 *
 * It contains no machine-size constants.
 *
 * Parser implementation must remain safe Rust under:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * No unsafe Rust is required.
 *
 * ============================================================================
 * 56. COMPATIBILITY
 * ============================================================================
 *
 * This grammar is additive to the existing hardware architecture.
 *
 * Existing:
 *
 *     grammar/hardware/hardware.g4
 *     grammar/hardware/power.g4
 *     grammar/hardware/timing.g4
 *     grammar/hardware/resources.g4
 *     grammar/hardware/capabilities.g4
 *     grammar/hardware/targets.g4
 *
 * remain separate ownership domains.
 *
 * No existing major filename is renamed.
 *
 * Thermal-specific syntax is introduced through:
 *
 *     THERMAL
 *
 * while most thermal vocabulary remains contextual identifiers.
 *
 * ============================================================================
 * 57. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no maximum thermal temperature;
 *     no maximum number of thermal domains;
 *     no maximum number of thermal states;
 *     no maximum number of sensors;
 *     no maximum number of transitions;
 *     no maximum number of cooling devices;
 *     no maximum power;
 *     no maximum energy;
 *     no maximum CPUs;
 *     no maximum GPUs;
 *     no maximum FPGAs;
 *     no maximum QPUs;
 *     no maximum nodes;
 *     no maximum memory;
 *     no maximum threads;
 *     no maximum tensor rank;
 *     no maximum register width;
 *     no fixed topology;
 *     no fixed device IDs.
 *
 * Any finite numeric value in source is program/contract data.
 *
 * ============================================================================
 * 58. TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     thermal contract basic {
 *         temperature <= thermal_limit;
 *     }
 *
 *     thermal contract symbolic<limit> {
 *         requires temperature <= limit;
 *     }
 *
 *     thermal profile nominal {
 *         ambient = ambient_temperature;
 *         junction <= junction_limit;
 *     }
 *
 *     thermal contract capability {
 *         capability thermal.monitoring;
 *     }
 *
 *     thermal contract resource {
 *         resource cooling.capacity >= required_cooling;
 *     }
 *
 *     thermal contract stateful {
 *         state nominal {
 *             temperature <= nominal_limit;
 *         }
 *     }
 *
 *     thermal contract transition {
 *         transition nominal -> throttled {
 *             temperature >= throttle_limit;
 *         }
 *     }
 *
 *     thermal contract scaling {
 *         scaling workload {
 *             temperature = thermal_model(workload);
 *         }
 *     }
 *
 * Required negative tests:
 *
 *     thermal contract bad {
 *         requires;
 *     }
 *
 *     thermal contract bad {
 *         temperature <= ;
 *     }
 *
 *     thermal contract bad {
 *         resource;
 *     }
 *
 * Required boundary tests:
 *
 *     one thermal property;
 *     many thermal properties;
 *     many profiles;
 *     many domains;
 *     many states;
 *     many transitions;
 *     nested contracts;
 *     deeply nested semantic structures;
 *     symbolic quantities;
 *     arbitrary expression complexity.
 *
 * Required scalability tests:
 *
 *     symbolic thermal limits;
 *     very large numeric values;
 *     very small numeric values;
 *     many thermal sections;
 *     many thermal properties;
 *     many resource references;
 *     many capability references.
 *
 * Required determinism tests:
 *
 *     identical source -> identical token stream;
 *     identical source -> identical parse structure.
 *
 * ============================================================================
 * 59. INTEGRATION COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE is complete when:
 *
 * [x] Canonical parser grammar is used.
 * [x] Canonical ZamaniLexer token vocabulary is consumed.
 * [x] No lexer rules exist here.
 * [x] Thermal declaration entry point exists.
 * [x] Thermal contracts exist.
 * [x] Requirements exist.
 * [x] Constraints exist.
 * [x] Preferences exist.
 * [x] Hints exist.
 * [x] Capability requirements exist.
 * [x] Resource requirements exist.
 * [x] Target references exist.
 * [x] Thermal properties are extensible.
 * [x] Thermal profiles exist.
 * [x] Thermal domains/states can be represented contextually.
 * [x] Thermal transitions exist.
 * [x] Measurement/scaling sections can be represented contextually.
 * [x] Assertions exist.
 * [x] Generic parameters exist.
 * [x] Attributes exist.
 * [x] Expressions are delegated to the canonical expression grammar.
 * [x] No duplicate expression grammar exists.
 * [x] No duplicate resource grammar exists.
 * [x] No duplicate capability grammar exists.
 * [x] No duplicate timing grammar exists.
 * [x] No duplicate power grammar exists.
 * [x] No quantum IR is created.
 * [x] No physical device is selected.
 * [x] No fixed hardware capacity is encoded.
 * [x] No fixed thermal capacity is encoded.
 * [x] No MAX_* constants are encoded.
 * [x] No semantic predicates are used.
 * [x] No embedded Rust is used.
 * [x] No unsafe Rust is required.
 * [x] POCO-REAF is preserved.
 *
 * Downstream completion additionally requires:
 *
 *     AST implementation;
 *     semantic implementation;
 *     resource/capability implementation;
 *     compiler consumers;
 *     runtime/HAL consumers;
 *     conformance tests.
 *
 * ============================================================================
 * 60. FINAL OWNERSHIP INVARIANT
 * ============================================================================
 *
 *     grammar/lexer/
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     hardware.g4
 *          |
 *          +--> thermal.g4
 *          +--> power.g4
 *          +--> timing.g4
 *          +--> resources.g4
 *          +--> capabilities.g4
 *          +--> targets.g4
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic thermal/resource/capability model
 *          |
 *          v
 *     canonical compiler IR
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> placement
 *          +--> routing
 *          +--> synthesis
 *          +--> resilience
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * The fundamental separation is:
 *
 *     THERMAL SYNTAX
 *          !=
 *     THERMAL ANALYSIS
 *          !=
 *     THERMAL MEASUREMENT
 *          !=
 *     THERMAL CONTROL
 *          !=
 *     THERMAL SCHEDULING
 *          !=
 *     PHYSICAL THERMAL REALIZATION
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */