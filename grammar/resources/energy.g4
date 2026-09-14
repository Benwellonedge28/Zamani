/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/energy.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the canonical SOURCE-SYNTAX CONTRACT for energy-related
 * resource intent.
 *
 * Energy is treated as a RESOURCE PROPERTY / RESOURCE OBJECTIVE.
 *
 * This grammar describes semantic intent such as:
 *
 *     energy requirement
 *     energy constraint
 *     energy preference
 *     energy hint
 *     energy budget
 *     energy estimate
 *     energy bound
 *     energy rate
 *     energy per operation
 *     energy per unit of work
 *     energy efficiency
 *     energy objective
 *     energy measurement intent
 *     energy accounting scope
 *     energy derivation
 *     energy relationship
 *
 * It does NOT describe:
 *
 *     a particular CPU
 *     a particular GPU
 *     a particular QPU
 *     a particular FPGA
 *     a particular ASIC
 *     a physical power rail
 *     a hardware address
 *     a fixed energy unit
 *     a fixed energy limit
 *     a fixed machine size
 *     a fixed number of operations
 *     a fixed number of devices
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *       lexer
 *          |
 *          v
 *       parser
 *          |
 *          v
 *     energy syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical resource semantics
 *          |
 *          +-------------------+-------------------+------------------+
 *          |                   |                   |                  |
 *          v                   v                   v                  v
 *     compilation         optimization        scheduling          runtime
 *          |                   |                   |                  |
 *          +-------------------+-------------------+------------------+
 *                                      |
 *                                      v
 *                               hardware / HAL
 *
 * This file MUST NOT create or represent IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - energy resource syntax;
 *     - energy quantities;
 *     - energy budgets;
 *     - energy bounds;
 *     - energy rates;
 *     - energy intensity;
 *     - energy efficiency;
 *     - energy objectives;
 *     - energy preferences;
 *     - energy constraints;
 *     - energy hints;
 *     - energy estimates;
 *     - energy measurement intent;
 *     - energy accounting scope;
 *     - energy relationships;
 *     - energy derivations;
 *     - energy properties;
 *     - energy metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general resource declarations;
 *     - general requirements;
 *     - general constraints;
 *     - general preferences;
 *     - general hints;
 *     - general resource expressions;
 *     - general identifiers;
 *     - general expressions;
 *     - physical energy measurement;
 *     - hardware discovery;
 *     - calibration;
 *     - scheduling algorithms;
 *     - optimization algorithms;
 *     - hardware allocation;
 *     - runtime accounting;
 *     - power management;
 *     - battery management;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - simulation;
 *     - provider APIs.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Energy syntax MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Energy intent belongs to the PROGRAM SEMANTICS only when the programmer
 * explicitly expresses it.
 *
 * Otherwise, measured or estimated energy is an execution-context property.
 *
 * The grammar therefore MUST NOT encode:
 *
 *     MAX_ENERGY
 *     MIN_ENERGY
 *     MAX_POWER
 *     MAX_DEVICES
 *     MAX_OPERATIONS
 *     MAX_CORES
 *     MAX_QUBITS
 *     MAX_GPUS
 *     MAX_FPGAS
 *
 * or equivalent constants.
 *
 * All quantities are expressions.
 *
 * Examples:
 *
 *     energy = workload_energy;
 *
 *     energy <= energy_budget;
 *
 *     energy_per_operation <= operation_budget;
 *
 *     energy_efficiency >= required_efficiency;
 *
 * The actual numerical meaning, unit compatibility, feasibility, and
 * hardware realization are semantic/runtime concerns.
 *
 * ============================================================================
 * UNIT INDEPENDENCE
 * ============================================================================
 *
 * No finite energy-unit vocabulary is hard-coded here.
 *
 * Units MAY be represented by expressions or symbolic metadata and are
 * validated by semantic analysis.
 *
 * This permits future units and domain-specific measurement systems without
 * requiring a grammar rewrite.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * This grammar is intentionally reusable by:
 *
 *     resources/resources.g4
 *     hardware/resources.g4
 *     hardware/targets.g4
 *     hardware/cpu.g4
 *     hardware/gpu.g4
 *     hardware/fpga.g4
 *     hardware/quantum-device.g4
 *     quantum/quantum-resources.g4
 *     hybrid/hybrid-resources.g4
 *     compile/target.g4
 *     execution/runtime-capabilities.g4
 *
 * Domain grammars MUST NOT redefine universal energy semantics.
 *
 * A domain grammar may specialize syntax around its own construct and then
 * lower into the canonical resource-energy semantic representation.
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCY
 * ============================================================================
 *
 * This grammar depends only on:
 *
 *     Zamani lexer vocabulary
 *     general Zamani expressions
 *
 * It MUST NOT import:
 *
 *     quantum grammar
 *     hardware grammar
 *     scheduling grammar
 *     optimization grammar
 *     runtime grammar
 *     QEC grammar
 *     ZQN grammar
 *
 * This keeps:
 *
 *     grammar -> semantic analysis -> IR -> execution
 *
 * acyclic.
 *
 * ============================================================================
 */

parser grammar ZamaniResourceEnergyParser;

options {
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * 1. PUBLIC INTEGRATION BOUNDARY
 * ============================================================================
 *
 * This is the single rule that resources/resources.g4 should consume.
 *
 * Existing resources.g4 currently owns a rule named:
 *
 *     resourceEnergyClause
 *
 * That existing implementation must be replaced by delegation to this rule
 * after this grammar is integrated.
 *
 * The rule name is intentionally retained so downstream resource grammar
 * consumers do not need to know the internal decomposition of energy syntax.
 * ============================================================================
 */

resourceEnergyClause
    : K_ENERGY
      energySpecification
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. ENERGY SPECIFICATION
 * ============================================================================
 *
 * Energy may be expressed as:
 *
 *     a direct expression;
 *     a structured specification;
 *     a named energy property;
 *     a scoped energy declaration.
 *
 * The structured form is preferred for extensibility.
 * ============================================================================
 */

energySpecification
    : energyExpressionSpecification
    | energyBlockSpecification
    ;


/*
 * ============================================================================
 * 3. DIRECT ENERGY EXPRESSION
 * ============================================================================
 *
 * Examples:
 *
 *     energy workload_energy;
 *     energy required_energy;
 *
 * The meaning of the expression is determined by semantic analysis.
 *
 * No fixed numeric range is imposed.
 * ============================================================================
 */

energyExpressionSpecification
    : expression
    ;


/*
 * ============================================================================
 * 4. STRUCTURED ENERGY SPECIFICATION
 * ============================================================================
 *
 * Example:
 *
 *     energy {
 *         value = workload_energy;
 *         unit = energy_unit;
 *         scope = program_scope;
 *         mode = estimate;
 *     }
 *
 * Property names are identifiers rather than a closed grammar vocabulary.
 *
 * This intentionally allows future energy properties without requiring a
 * lexer redesign for every new property.
 * ============================================================================
 */

energyBlockSpecification
    : LBRACE
      energyItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. ENERGY ITEM
 * ============================================================================
 */

energyItem
    : energyProperty
    | energyRelationship
    | energyRequirement
    | energyConstraint
    | energyPreference
    | energyHint
    | energyMeasurement
    | energyDerivation
    ;


/*
 * ============================================================================
 * 6. GENERIC ENERGY PROPERTY
 * ============================================================================
 *
 * Example:
 *
 *     value = workload_energy;
 *
 *     unit = joule;
 *
 *     scope = execution;
 *
 *     mode = estimate;
 *
 * The semantic layer determines whether a property is valid and what its
 * type means.
 * ============================================================================
 */

energyProperty
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. ENERGY RELATIONSHIP
 * ============================================================================
 *
 * Relationships express comparisons or derived conditions without embedding
 * machine-specific thresholds into the grammar.
 *
 * Examples:
 *
 *     total <= budget;
 *
 *     per_operation < target;
 *
 *     efficiency >= minimum_efficiency;
 *
 *     measured == expected;
 * ============================================================================
 */

energyRelationship
    : IDENTIFIER
      energyComparisonOperator
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. ENERGY REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * Example:
 *
 *     require = energy_budget;
 *
 * The exact interpretation is performed downstream.
 * ============================================================================
 */

energyRequirement
    : K_REQUIRES
      IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. ENERGY CONSTRAINT
 * ============================================================================
 *
 * A constraint must be respected by every valid realization.
 * ============================================================================
 */

energyConstraint
    : K_CONSTRAINT
      IDENTIFIER
      energyComparisonOperator
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. ENERGY PREFERENCE
 * ============================================================================
 *
 * A preference is advisory.
 *
 * It MUST NOT be treated as a hard semantic requirement merely because it
 * mentions energy.
 * ============================================================================
 */

energyPreference
    : K_PREFERENCE
      IDENTIFIER
      energyComparisonOperator
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. ENERGY HINT
 * ============================================================================
 *
 * A hint provides advisory information to compilation/runtime layers.
 * ============================================================================
 */

energyHint
    : K_HINT
      IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. ENERGY MEASUREMENT
 * ============================================================================
 *
 * Measurement intent is different from a compile-time requirement.
 *
 * Examples:
 *
 *     measure = runtime;
 *
 *     measure = hardware;
 *
 *     measure = estimated;
 *
 *     measure = modeled;
 *
 * The grammar does not prescribe the measurement mechanism.
 * ============================================================================
 */

energyMeasurement
    : K_MEASURE
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. ENERGY DERIVATION
 * ============================================================================
 *
 * Defines a relationship whose value may be calculated from other program or
 * resource properties.
 *
 * Example:
 *
 *     derive_energy = operation_count * energy_per_operation;
 *
 * This is an expression-level relationship, not an execution algorithm.
 * ============================================================================
 */

energyDerivation
    : K_DERIVE
      IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. COMPARISON OPERATORS
 * ============================================================================
 *
 * These operators are deliberately represented as syntax only.
 *
 * The semantic layer determines:
 *
 *     dimensional compatibility;
 *     unit compatibility;
 *     precision;
 *     uncertainty;
 *     measurement semantics;
 *     estimate semantics.
 * ============================================================================
 */

energyComparisonOperator
    : ASSIGN
    | EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * 15. SEMANTIC PROPERTY CATEGORIES
 * ============================================================================
 *
 * The following comments document reserved semantic categories without
 * hard-coding them into the lexer.
 *
 * Canonical semantic properties may include:
 *
 *     value
 *     budget
 *     minimum
 *     maximum
 *     target
 *     estimate
 *     measured
 *     projected
 *     per_operation
 *     per_instruction
 *     per_byte
 *     per_element
 *     per_sample
 *     per_qubit
 *     per_gate
 *     per_shot
 *     per_cycle
 *     per_task
 *     per_transaction
 *     rate
 *     efficiency
 *     intensity
 *     scope
 *     source
 *     confidence
 *     uncertainty
 *     unit
 *     model
 *     mode
 *     accounting
 *
 * These names are intentionally NOT lexer-level keywords.
 *
 * This prevents energy.g4 from creating a finite vocabulary that would
 * prevent future resource models from extending Zamani.
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. ENERGY SCOPE
 * ============================================================================
 *
 * Scope is represented through a normal expression/property rather than a
 * fixed enumeration.
 *
 * This permits scopes such as:
 *
 *     program
 *     module
 *     function
 *     task
 *     operation
 *     kernel
 *     circuit
 *     execution
 *     deployment
 *     device
 *     resource
 *     custom
 *
 * The semantic layer validates scope names and compatibility.
 * ============================================================================
 */

energyScopeExpression
    : expression
    ;


/*
 * ============================================================================
 * 17. ENERGY UNIT EXPRESSION
 * ============================================================================
 *
 * Units are intentionally expressions rather than fixed grammar literals.
 *
 * This prevents the language from being tied to a finite set of units.
 * ============================================================================
 */

energyUnitExpression
    : expression
    ;


/*
 * ============================================================================
 * 18. ENERGY VALUE EXPRESSION
 * ============================================================================
 *
 * Energy values may depend on:
 *
 *     program parameters;
 *     workload;
 *     input size;
 *     problem size;
 *     runtime context;
 *     resource capacity;
 *     execution measurements;
 *     symbolic expressions.
 *
 * No fixed numeric domain is imposed here.
 * ============================================================================
 */

energyValueExpression
    : expression
    ;


/*
 * ============================================================================
 * 19. ENERGY BUDGET EXPRESSION
 * ============================================================================
 *
 * A budget is a policy/resource condition.
 *
 * It is not automatically interpreted as a physical battery capacity.
 * ============================================================================
 */

energyBudgetExpression
    : expression
    ;


/*
 * ============================================================================
 * 20. ENERGY RATE EXPRESSION
 * ============================================================================
 *
 * Rate may represent an energy-related quantity over another dimension.
 *
 * The semantic layer determines the dimensional relationship.
 * ============================================================================
 */

energyRateExpression
    : expression
    ;


/*
 * ============================================================================
 * 21. ENERGY EFFICIENCY EXPRESSION
 * ============================================================================
 *
 * Efficiency is intentionally generic.
 *
 * Examples include:
 *
 *     useful_work / energy
 *     throughput / energy
 *     operations / energy
 *     result_quality / energy
 *
 * The grammar does not prescribe one definition.
 * ============================================================================
 */

energyEfficiencyExpression
    : expression
    ;


/*
 * ============================================================================
 * 22. ENERGY INTENSITY EXPRESSION
 * ============================================================================
 */

energyIntensityExpression
    : expression
    ;


/*
 * ============================================================================
 * 23. ENERGY ESTIMATE EXPRESSION
 * ============================================================================
 */

energyEstimateExpression
    : expression
    ;


/*
 * ============================================================================
 * 24. ENERGY UNCERTAINTY EXPRESSION
 * ============================================================================
 */

energyUncertaintyExpression
    : expression
    ;


/*
 * ============================================================================
 * 25. ENERGY CONFIDENCE EXPRESSION
 * ============================================================================
 */

energyConfidenceExpression
    : expression
    ;


/*
 * ============================================================================
 * 26. ENERGY PROPERTY CLASSIFICATION
 * ============================================================================
 *
 * These semantic categories are documented here so the AST/semantic layer
 * can distinguish them even though the source grammar remains extensible.
 *
 * The parser must not force a finite list of energy properties.
 *
 * Canonical semantic classification:
 *
 *     absolute energy
 *     energy budget
 *     energy bound
 *     energy rate
 *     energy intensity
 *     energy efficiency
 *     estimated energy
 *     measured energy
 *     projected energy
 *     normalized energy
 *     energy objective
 *     energy preference
 *     energy hint
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. RESOURCE-SCALE INDEPENDENCE
 * ============================================================================
 *
 * No grammar rule in this file refers to:
 *
 *     qubit count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     node count
 *     device count
 *     memory size
 *     topology size
 *     machine size
 *
 * Energy expressions may nevertheless depend on any such semantic quantity
 * when another part of the language exposes it.
 *
 * Example:
 *
 *     energy <= f(workload_size, available_capacity)
 *
 * remains valid without this grammar knowing what machine provides the
 * capacity.
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum grammar may attach energy intent to:
 *
 *     circuits
 *     operations
 *     gates
 *     measurements
 *     shots
 *     logical operations
 *     execution regions
 *
 * It MUST NOT redefine resourceEnergyClause.
 *
 * quantum/quantum-resources.g4 should delegate universal energy semantics to
 * the resource layer.
 *
 * The grammar itself does not know:
 *
 *     QPU topology
 *     gate implementation
 *     pulse implementation
 *     calibration
 *     physical energy consumption.
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical constructs may produce energy-relevant expressions for:
 *
 *     operations
 *     kernels
 *     functions
 *     tasks
 *     data movement
 *     memory activity
 *     accelerator work
 *
 * The semantic layer determines the actual energy model.
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. HDL INTEGRATION
 * ============================================================================
 *
 * HDL constructs may associate energy intent with:
 *
 *     modules
 *     processes
 *     clocks
 *     pipelines
 *     memories
 *     interfaces
 *     hardware operations
 *
 * energy.g4 does not define:
 *
 *     voltage
 *     transistor count
 *     physical power rails
 *     placement
 *     clock-tree implementation.
 *
 * Those belong to hardware/HDL semantic layers.
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware grammars may provide:
 *
 *     measured energy
 *     modeled energy
 *     estimated energy
 *     power characteristics
 *
 * Those are execution/target facts rather than universal source semantics.
 *
 * A target may satisfy a source-level energy requirement or reject it during
 * capability/resource analysis.
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler pipeline should interpret the parsed construct approximately
 * as:
 *
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic validation
 *       |
 *       v
 *     canonical resource intent
 *       |
 *       +--> target-independent compilation
 *       |
 *       +--> target capability negotiation
 *       |
 *       +--> optimization
 *       |
 *       +--> scheduling
 *       |
 *       +--> hardware/resource selection
 *
 * energy.g4 must never directly invoke any of these systems.
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may use:
 *
 *     measured energy
 *     energy budget
 *     energy telemetry
 *     execution energy
 *     resource energy state
 *
 * Runtime MUST NOT require the grammar to know how energy is measured.
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization may consume energy preferences or objectives.
 *
 * A preference MUST NOT silently become a hard constraint.
 *
 * Optimization may trade:
 *
 *     energy
 *     latency
 *     throughput
 *     fidelity
 *     reliability
 *     cost
 *
 * according to compiler policy.
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling may consume energy-related objectives or constraints.
 *
 * Scheduling determines ordering/timing/resource realization.
 *
 * energy.g4 does not implement scheduling.
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Resilience may observe energy-related resource degradation or constraints.
 *
 * It may use that information when deciding among recovery/adaptation
 * actions.
 *
 * This grammar does not define resilience policy.
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. SECURITY
 * ============================================================================
 *
 * Energy expressions MUST be treated as untrusted source input by the
 * frontend.
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     code execution
 *     environment inspection
 *     device access.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic for the same token stream.
 *
 * No semantic predicate or embedded action is used.
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no machine limits;
 *     no numeric limits;
 *     no finite energy-unit catalogue;
 *     no device IDs;
 *     no topology;
 *     no hardware addresses;
 *     no provider names;
 *     no fixed resource count.
 *
 * Expressions carry all scalable quantities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     1. ANTLR accepts it with tokenVocab = Zamani.
 *
 *     2. resourceEnergyClause is the single universal energy grammar boundary.
 *
 *     3. resources/resources.g4 delegates its energy rule here.
 *
 *     4. No duplicate universal energy grammar exists elsewhere.
 *
 *     5. Quantum/hardware/hybrid energy grammars specialize rather than
 *        redefine universal semantics.
 *
 *     6. The resulting AST can preserve:
 *            value
 *            unit
 *            scope
 *            budget
 *            bound
 *            estimate
 *            measurement
 *            efficiency
 *            intensity
 *            uncertainty
 *            confidence
 *            preference
 *            constraint
 *            hint
 *            derivation
 *        without requiring grammar changes.
 *
 *     7. No fixed resource or machine size is encoded.
 *
 *     8. Positive, negative, boundary, deterministic and cross-domain tests
 *        pass.
 *
 *     9. Rust 1.97 / 1.97.1 builds the generated parser integration without
 *        unsafe Rust.
 *
 *    10. The semantic layer, not this grammar, owns dimensional validation,
 *        unit compatibility, physical measurement and feasibility.
 *
 * ============================================================================
 */