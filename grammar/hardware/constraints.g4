/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/constraints.g4
 *
 * Grammar:
 *     ZamaniHardwareConstraintsParser
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     Action-free ANTLR4 parser grammar.
 *     No embedded Rust.
 *     No embedded target-language actions.
 *     No unsafe code.
 *
 * ============================================================================
 * STATUS
 * ============================================================================
 *
 * Canonical hardware-domain constraint grammar.
 *
 * This file supersedes the architectural ownership of:
 *
 *     grammar/hardware/hardware-constraints.g4
 *
 * The old file must NOT remain a second implementation of these rules.
 * It should become a compatibility/deprecation facade or be retired only
 * after repository-wide references have been migrated.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines target-independent HARDWARE CONSTRAINT SYNTAX.
 *
 * A hardware constraint states a condition that must hold for a hardware
 * realization to be considered semantically valid.
 *
 * It describes PORTABLE INTENT.
 *
 * It does NOT select, discover, allocate, route, schedule, calibrate,
 * synthesize, or control physical hardware.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - hardwareConstraintDeclaration
 *   - hardwareConstraintClause
 *   - hardwareConstraintBody
 *   - hardwareConstraintMember
 *   - hardware constraint classification
 *   - hardware resource constraint references
 *   - hardware capability constraint references
 *   - hardware target constraint references
 *   - hardware topology constraint references
 *   - hardware placement constraint references
 *   - hardware timing constraint references
 *   - hardware performance constraint references
 *   - hardware scalability constraint references
 *   - hardware portability constraint references
 *   - hardware reliability constraint references
 *   - hardware energy/power constraint references
 *   - hardware synthesis constraints
 *   - hardware interface constraints
 *   - hardware implementation-intent constraints
 *   - generic hardware constraint properties
 *   - hardware constraint annotations
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - keywords;
 *   - identifiers;
 *   - generic expressions;
 *   - universal constraint semantics;
 *   - universal type constraints;
 *   - resource discovery;
 *   - resource allocation;
 *   - target discovery;
 *   - target selection;
 *   - physical device enumeration;
 *   - physical device identifiers;
 *   - physical addresses;
 *   - routing;
 *   - placement realization;
 *   - scheduling;
 *   - optimization;
 *   - synthesis;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL IR;
 *   - HAL;
 *   - runtime dispatch;
 *   - device drivers.
 *
 * ============================================================================
 * AUTHORITY MODEL
 * ============================================================================
 *
 * Generic constraint composition belongs to:
 *
 *     grammar/core/constraints.g4
 *
 * Generic requirements belong to:
 *
 *     grammar/core/requirements.g4
 *
 * Generic capabilities belong to:
 *
 *     grammar/core/capabilities.g4
 *
 * Resource-domain constraints belong to:
 *
 *     grammar/resources/
 *
 * Hardware constraints belong here.
 *
 * Hardware realization belongs downstream.
 *
 * The canonical composition root remains:
 *
 *     grammar/Zamani.g4
 *
 * This file must never become another root grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Hardware constraints MUST describe properties of a valid realization,
 * rather than today's hardware inventory.
 *
 * Valid semantic concepts include:
 *
 *     requires resource::memory >= required_memory;
 *     requires resource::compute >= required_compute;
 *     capability::quantum::measurement == true;
 *     target::supports_hdl == true;
 *     topology::connectivity >= required_connectivity;
 *     timing::latency <= latency_budget;
 *
 * They MUST NOT encode universal machine limits such as:
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
 * A concrete numeric value in source code is allowed when it is PROGRAM
 * SEMANTICS.
 *
 * A compiler-wide artificial maximum is not.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / CAPABILITY / PREFERENCE
 * ============================================================================
 *
 * These concepts are deliberately different:
 *
 * REQUIREMENT:
 *     What the program needs.
 *
 * CAPABILITY:
 *     What a target/environment can provide.
 *
 * CONSTRAINT:
 *     A condition that must hold.
 *
 * PREFERENCE:
 *     A preferred realization among valid realizations.
 *
 * HINT:
 *     Non-mandatory implementation guidance.
 *
 * IMPLEMENTATION DECISION:
 *     A concrete realization chosen downstream.
 *
 * This grammar owns CONSTRAINT syntax.
 *
 * It must not collapse these concepts into one grammar construct.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Hardware technology continuously evolves.
 *
 * Therefore hardware properties remain symbolic wherever possible.
 *
 * Examples:
 *
 *     hardware::future_accelerator::supports_feature == true;
 *     hardware::vendor_extension::property >= required_value;
 *
 * A new device capability must not require a new Zamani keyword merely
 * because a new technology appears.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * All quantities are expressions.
 *
 * No grammar-level quantity limits exist.
 *
 * The grammar therefore scales structurally from:
 *
 *     one resource
 *     one processing element
 *     one qubit
 *     one accelerator
 *
 * through arbitrarily large resource sets permitted by the semantic model,
 * compiler, runtime, and available hardware.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum hardware constraints may describe:
 *
 *     quantum measurement capability
 *     reset capability
 *     dynamic control
 *     mid-circuit measurement
 *     logical-qubit capability
 *     fault-tolerance capability
 *     error-correction capability
 *     coherence requirements
 *     fidelity requirements
 *     topology requirements
 *
 * They do NOT define quantum operations.
 *
 * They do NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumCircuit
 *     QuantumOperation
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Hardware constraints may constrain HDL realization properties such as:
 *
 *     timing
 *     clocking
 *     pipeline depth
 *     interface properties
 *     synthesis requirements
 *     combinational/sequential behavior
 *
 * They do not own HDL syntax or behavioral semantics.
 *
 * HDL remains owned by:
 *
 *     grammar/hdl/
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * This grammar references resources.
 *
 * It does not define how resources are:
 *
 *     discovered
 *     allocated
 *     reserved
 *     scheduled
 *     measured
 *     released
 *     physically mapped
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * TARGET BOUNDARY
 * ============================================================================
 *
 * Targets are symbolic.
 *
 * This grammar may express:
 *
 *     target::supports_quantum
 *     target::supports_hdl
 *     target::supports_dynamic_control
 *
 * It must not require physical identities such as:
 *
 *     gpu0
 *     qpu7
 *     fpga3
 *     cpu0
 *
 * unless such an identity is explicitly part of a target-specific dialect
 * and is therefore outside the portable core contract.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted declaration maps to the domain-neutral frontend AST.
 *
 * Conceptual mapping:
 *
 *     hardwareConstraintDeclaration
 *         -> ConstraintDeclaration
 *
 *     hardwareConstraintClause
 *         -> ConstraintClause
 *
 *     hardwareConstraintPredicate
 *         -> ConstraintPredicate
 *
 *     hardwareConstraintReference
 *         -> SymbolicReference
 *
 *     hardwareConstraintValue
 *         -> Expression
 *
 * The AST must preserve:
 *
 *     source span
 *     declaration name
 *     annotations
 *     classification
 *     subject/reference
 *     operator
 *     value expression
 *     nesting
 *
 * No hardware vendor, topology, QEC, routing, calibration, or HAL object
 * should be embedded in the generic frontend AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     type checking
 *     dimensional/unit checking
 *     capability validation
 *     resource validation
 *     portability validation
 *     constraint satisfiability analysis
 *     target-context validation
 *
 * The parser MUST NOT decide whether a constraint can be satisfied.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO hardware-specific IR.
 *
 * The lowering path is:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic constraint model
 *       |
 *       v
 *     canonical compiler IR / semantic contracts
 *       |
 *       +--> resource analysis
 *       +--> capability analysis
 *       +--> optimization
 *       +--> routing
 *       +--> scheduling
 *       +--> synthesis
 *       +--> HAL
 *       +--> runtime
 *
 * Quantum constraints eventually interact with quantum::ir through the
 * semantic/compiler pipeline. This grammar must never create a second
 * quantum IR.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The existing grammar/hardware/hardware.g4 already has hardware constraint
 * dispatch.
 *
 * The integration must therefore:
 *
 *   1. retain hardwareConstraintDeclaration as the public rule name;
 *   2. remove duplicate implementations of that rule from hardware.g4;
 *   3. compose this grammar into the hardware grammar;
 *   4. keep Zamani.g4 as the composition root;
 *   5. retain grammar/core/constraints.g4 as the generic constraint authority;
 *   6. retain grammar/resources/constraints.g4 for resource-domain semantics;
 *   7. migrate hardware-constraints.g4 to compatibility/deprecated status.
 *
 * No second hardware constraint dispatcher is permitted.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareConstraintsParser;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC DECLARATION
 * ============================================================================
 *
 * Named reusable hardware constraint.
 *
 * Example:
 *
 *     constraint ScalableCompute {
 *         resource::compute >= workload::required_compute;
 *     }
 *
 * ============================================================================
 */

hardwareConstraintDeclaration
    : hardwareConstraintAnnotation*
      hardwareConstraintVisibility?
      hardwareConstraintModifier*
      K_CONSTRAINT
      IDENTIFIER
      hardwareConstraintParameters?
      hardwareConstraintBody
    ;


/* ============================================================================
 * 2. INLINE HARDWARE CONSTRAINT CLAUSE
 * ============================================================================
 *
 * Used by hardware declarations that embed a constraint.
 *
 * Example:
 *
 *     constraint {
 *         resource::memory >= required_memory;
 *     }
 *
 * ============================================================================
 */

hardwareConstraintClause
    : K_CONSTRAINT
      hardwareConstraintBody
    ;


/* ============================================================================
 * 3. BODY
 * ============================================================================
 */

hardwareConstraintBody
    : LBRACE
      hardwareConstraintMember*
      RBRACE
    ;


/* ============================================================================
 * 4. MEMBERS
 * ============================================================================
 */

hardwareConstraintMember
    : hardwareConstraintAnnotation*
      (
          hardwareConstraintRequirement
        | hardwareConstraintCapability
        | hardwareConstraintResource
        | hardwareConstraintTarget
        | hardwareConstraintTopology
        | hardwareConstraintPlacement
        | hardwareConstraintTiming
        | hardwareConstraintPerformance
        | hardwareConstraintScalability
        | hardwareConstraintPortability
        | hardwareConstraintReliability
        | hardwareConstraintPower
        | hardwareConstraintEnergy
        | hardwareConstraintSynthesis
        | hardwareConstraintInterface
        | hardwareConstraintImplementation
        | hardwareConstraintPredicate
        | hardwareConstraintProperty
        | hardwareConstraintNested
      )
    ;


/* ============================================================================
 * 5. VISIBILITY
 * ============================================================================
 */

hardwareConstraintVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;


/* ============================================================================
 * 6. DECLARATION MODIFIERS
 * ============================================================================
 */

hardwareConstraintModifier
    : K_STATIC
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


/* ============================================================================
 * 7. ANNOTATIONS
 * ============================================================================
 *
 * Annotations remain metadata.
 *
 * Their semantic interpretation belongs outside the parser.
 */

hardwareConstraintAnnotation
    : AT
      hardwareConstraintQualifiedName
      (
          LPAREN
          hardwareConstraintAnnotationArguments?
          RPAREN
      )?
    ;

hardwareConstraintAnnotationArguments
    : hardwareConstraintAnnotationArgument
      (
          COMMA
          hardwareConstraintAnnotationArgument
      )*
      COMMA?
    ;

hardwareConstraintAnnotationArgument
    : hardwareConstraintValue
    | hardwareConstraintQualifiedName
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | CHAR_LITERAL
    ;


/* ============================================================================
 * 8. GENERIC PARAMETERS
 * ============================================================================
 *
 * Constraint declarations may be parameterized.
 *
 * No fixed number of parameters is imposed.
 */

hardwareConstraintParameters
    : LT
      hardwareConstraintParameter
      (
          COMMA
          hardwareConstraintParameter
      )*
      COMMA?
      GT
    ;

hardwareConstraintParameter
    : IDENTIFIER
      (
          COLON
          hardwareConstraintQualifiedName
      )?
      (
          ASSIGN
          hardwareConstraintValue
      )?
    ;


/* ============================================================================
 * 9. REQUIREMENT
 * ============================================================================
 *
 * A requirement identifies a condition needed by the hardware realization.
 *
 * It remains distinct from a raw constraint predicate.
 */

hardwareConstraintRequirement
    : K_REQUIRES
      hardwareConstraintCondition
      SEMICOLON
    ;


/* ============================================================================
 * 10. CAPABILITY
 * ============================================================================
 *
 * Capability checks describe properties that the realization must expose.
 *
 * Examples:
 *
 *     capability::quantum::measurement == true;
 *     capability::tensor::compute == true;
 *
 * ============================================================================
 */

hardwareConstraintCapability
    : K_CAPABILITY
      hardwareConstraintReference
      (
          hardwareConstraintRelation
          hardwareConstraintValue
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 11. RESOURCE
 * ============================================================================
 *
 * Resource constraints are symbolic.
 *
 * Examples:
 *
 *     resource::memory >= required_memory;
 *     resource::compute >= required_compute;
 *     resource::bandwidth >= required_bandwidth;
 *
 * No machine capacity is encoded by the grammar.
 */

hardwareConstraintResource
    : K_RESOURCE
      hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 12. TARGET
 * ============================================================================
 *
 * Target constraints describe target classes or properties.
 *
 * They do not select a physical machine.
 */

hardwareConstraintTarget
    : K_TARGET
      hardwareConstraintReference
      (
          hardwareConstraintRelation
          hardwareConstraintValue
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 13. TOPOLOGY
 * ============================================================================
 *
 * Topology is a semantic requirement.
 *
 * Actual topology discovery and routing are downstream.
 */

hardwareConstraintTopology
    : K_TOPOLOGY
      (
          hardwareConstraintReference
      )?
      hardwareConstraintBody
    ;

hardwareConstraintTopologyMember
    : hardwareConstraintPredicate
    | hardwareConstraintProperty
    ;


/* ============================================================================
 * 14. PLACEMENT
 * ============================================================================
 *
 * Placement constraints express relationships such as affinity or locality.
 *
 * They do not perform placement.
 */

hardwareConstraintPlacement
    : K_PLACEMENT
      (
          hardwareConstraintReference
      )?
      hardwareConstraintBody
    ;

hardwareConstraintPlacementMember
    : hardwareConstraintPredicate
    | hardwareConstraintProperty
    ;


/* ============================================================================
 * 15. TIMING
 * ============================================================================
 *
 * Timing quantities are expressions.
 *
 * No clock or timing maximum is hard-coded.
 */

hardwareConstraintTiming
    : K_TIMING
      hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 16. PERFORMANCE
 * ============================================================================
 */

hardwareConstraintPerformance
    : K_PERFORMANCE
      hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 17. SCALABILITY
 * ============================================================================
 *
 * This is a semantic property, not a fixed size.
 */

hardwareConstraintScalability
    : K_SCALABILITY
      hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 18. PORTABILITY
 * ============================================================================
 */

hardwareConstraintPortability
    : K_PORTABILITY
      hardwareConstraintReference
      (
          hardwareConstraintRelation
          hardwareConstraintValue
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 19. RELIABILITY
 * ============================================================================
 *
 * Reliability is expressed as a property contract.
 *
 * This grammar does not implement resilience, QEC, ZQN, telemetry,
 * recovery, or fault injection.
 */

hardwareConstraintReliability
    : K_RELIABILITY
      hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 20. POWER
 * ============================================================================
 */

hardwareConstraintPower
    : K_POWER
      hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 21. ENERGY
 * ============================================================================
 */

hardwareConstraintEnergy
    : K_ENERGY
      hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 22. SYNTHESIS
 * ============================================================================
 *
 * Synthesis constraints describe intent.
 *
 * They do not invoke synthesis.
 */

hardwareConstraintSynthesis
    : K_SYNTHESIS
      hardwareConstraintReference
      (
          hardwareConstraintRelation
          hardwareConstraintValue
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 23. INTERFACE
 * ============================================================================
 */

hardwareConstraintInterface
    : K_INTERFACE
      hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 24. IMPLEMENTATION-INTENT CONSTRAINT
 * ============================================================================
 *
 * This is deliberately symbolic.
 *
 * It allows a dialect or target-specific semantic layer to describe an
 * implementation property without forcing that property into the portable
 * hardware grammar.
 */

hardwareConstraintImplementation
    : K_IMPLEMENTATION
      hardwareConstraintReference
      (
          hardwareConstraintRelation
          hardwareConstraintValue
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 25. GENERAL PREDICATE
 * ============================================================================
 *
 * This is the principal open-world extension mechanism.
 *
 * Example:
 *
 *     hardware::future_device::feature >= required_feature;
 *
 * New technologies therefore do not require permanent core grammar keywords.
 */

hardwareConstraintPredicate
    : hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 26. GENERIC PROPERTY
 * ============================================================================
 *
 * A property may be attached to a constraint body without introducing a
 * new language keyword.
 */

hardwareConstraintProperty
    : hardwareConstraintReference
      ASSIGN
      hardwareConstraintValue
      SEMICOLON
    ;


/* ============================================================================
 * 27. NESTED CONSTRAINT
 * ============================================================================
 *
 * Nested named/grouped constraints allow hierarchical contracts without
 * imposing any maximum nesting depth.
 */

hardwareConstraintNested
    : K_CONSTRAINT
      IDENTIFIER?
      hardwareConstraintBody
    ;


/* ============================================================================
 * 28. CONDITION
 * ============================================================================
 *
 * Logical composition is represented structurally.
 *
 * Semantic precedence remains owned by the canonical constraint/expression
 * model.
 */

hardwareConstraintCondition
    : hardwareConstraintPredicateExpression
      (
          hardwareConstraintLogicalOperator
          hardwareConstraintPredicateExpression
      )*
    ;

hardwareConstraintPredicateExpression
    : LPAREN
      hardwareConstraintCondition
      RPAREN
    | hardwareConstraintReference
      hardwareConstraintRelation
      hardwareConstraintValue
    | hardwareConstraintReference
    ;


/* ============================================================================
 * 29. RELATION
 * ============================================================================
 *
 * Assignment is deliberately retained because existing Zamani hardware
 * property syntax uses:
 *
 *     property = expression;
 *
 * Relational operators remain distinct from assignment at semantic analysis.
 *
 * The semantic layer MUST reject assignment where only a comparison is
 * meaningful.
 */

hardwareConstraintRelation
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;


/* ============================================================================
 * 30. LOGICAL OPERATORS
 * ============================================================================
 */

hardwareConstraintLogicalOperator
    : LOGICAL_AND
    | LOGICAL_OR
    | K_AND
    | K_OR
    ;


/* ============================================================================
 * 31. SYMBOLIC REFERENCES
 * ============================================================================
 *
 * Qualified names permit open-world domain expansion:
 *
 *     hardware::compute
 *     hardware::quantum::measurement
 *     resource::memory
 *     target::supports_quantum
 *     topology::connectivity
 *
 * The parser does not interpret the meaning of the name.
 */

hardwareConstraintReference
    : hardwareConstraintQualifiedName
    ;

hardwareConstraintQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/* ============================================================================
 * 32. VALUES
 * ============================================================================
 *
 * Values are deliberately routed through expressions.
 *
 * This permits:
 *
 *     literals
 *     variables
 *     generic parameters
 *     symbolic values
 *     arithmetic
 *     function results
 *     compile-time values
 *     unit-aware values
 *     resource-derived values
 *
 * without imposing a hardware-size limit.
 *
 * The production integration should bind hardwareConstraintValue to the
 * canonical expression nonterminal exported by the expression subsystem.
 */

hardwareConstraintValue
    : hardwareConstraintExpression
    ;


/*
 * ============================================================================
 * 33. EXPRESSION BRIDGE
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This is a compatibility bridge only.
 *
 * It is intentionally small.
 *
 * The canonical repository integration must resolve this bridge to the
 * universal expression grammar rather than growing a second hardware
 * expression language.
 *
 * The hardware expression may contain arbitrary semantic identifiers and
 * representable literals. Numeric range/precision is not limited here.
 * ============================================================================
 */

hardwareConstraintExpression
    : hardwareConstraintPrimary
      hardwareConstraintExpressionSuffix*
    ;

hardwareConstraintExpressionSuffix
    : DOT
      IDENTIFIER
    | LBRACKET
      hardwareConstraintExpression
      RBRACKET
    | LPAREN
      hardwareConstraintArgumentList?
      RPAREN
    ;

hardwareConstraintPrimary
    : hardwareConstraintQualifiedName
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | K_TRUE
    | K_FALSE
    | LPAREN
      hardwareConstraintExpression
      RPAREN
    ;

hardwareConstraintArgumentList
    : hardwareConstraintExpression
      (
          COMMA
          hardwareConstraintExpression
      )*
      COMMA?
    ;


/* ============================================================================
 * 34. HARDWARE-SPECIFIC SEMANTIC PROPERTY VOCABULARY
 * ============================================================================
 *
 * These are DOCUMENTED semantic categories, not a closed parser enumeration.
 *
 * Implementations may recognize properties such as:
 *
 *     compute_capacity
 *     memory_capacity
 *     memory_bandwidth
 *     latency
 *     throughput
 *     frequency
 *     period
 *     power
 *     energy
 *     reliability
 *     availability
 *     durability
 *     fault_rate
 *     failure_rate
 *     mtbf
 *     mttr
 *     recovery_time
 *     recovery_point
 *     resilience
 *     redundancy
 *     replication
 *     fault_tolerance
 *     error_correction
 *     coherence
 *     fidelity
 *     topology
 *     connectivity
 *     distance
 *     degree
 *     pipeline
 *     clocking
 *     synthesis
 *     technology
 *     architecture
 *
 * They intentionally remain identifiers/qualified names whenever possible.
 *
 * This prevents the language from becoming closed to future hardware
 * technologies.
 *
 * ============================================================================
 */


/* ============================================================================
 * 35. INTEGRATION ASSERTIONS
 * ============================================================================
 *
 * The following conceptual invariants MUST hold in the integrated grammar:
 *
 *   - one canonical hardware constraint declaration;
 *   - one canonical generic constraint model;
 *   - one canonical expression model;
 *   - one canonical resource model;
 *   - one canonical capability model;
 *   - one canonical target model;
 *   - no duplicate quantum IR;
 *   - no physical device enumeration in the portable grammar;
 *   - no fixed hardware capacity;
 *   - no vendor-specific mandatory keywords;
 *   - no target-specific machine assumptions.
 *
 * ============================================================================
 * 36. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when all of the following are true:
 *
 * [ ] ANTLR grammar generation succeeds.
 * [ ] Rust 1.97 / 1.97.1 generated parser builds.
 * [ ] No unsafe Rust is introduced.
 * [ ] No duplicate hardware constraint rule remains authoritative.
 * [ ] hardware.g4 dispatches hardwareConstraintDeclaration exactly once.
 * [ ] Zamani.g4 remains the composition root.
 * [ ] core/constraints.g4 remains generic constraint authority.
 * [ ] resources/constraints.g4 remains resource-domain authority.
 * [ ] canonical expressions are used by semantic integration.
 * [ ] source spans are preserved by the AST layer.
 * [ ] requirement/capability/constraint/preference distinctions survive AST.
 * [ ] no fixed hardware capacities exist.
 * [ ] symbolic future hardware properties parse.
 * [ ] quantum capability constraints parse.
 * [ ] HDL-related constraints parse.
 * [ ] distributed hardware constraints parse.
 * [ ] arbitrary resource expressions parse.
 * [ ] nested constraints have no artificial depth limit.
 * [ ] positive tests exist.
 * [ ] negative tests exist.
 * [ ] boundary tests exist.
 * [ ] scalability tests exist.
 * [ ] determinism tests exist.
 * [ ] compatibility tests exist.
 * [ ] hard-coding audit passes.
 *
 * ============================================================================
 * 37. REQUIRED CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * The following semantic forms MUST be representable:
 *
 *     constraint ScalableCompute {
 *         resource::compute >= workload::required_compute;
 *     }
 *
 *     constraint QuantumMeasurement {
 *         capability::quantum::measurement == true;
 *     }
 *
 *     constraint Memory {
 *         resource::memory >= required_memory;
 *     }
 *
 *     constraint Timing {
 *         timing::latency <= application::latency_budget;
 *     }
 *
 *     constraint Topology {
 *         topology::connectivity >= required_connectivity;
 *     }
 *
 *     constraint FutureHardware {
 *         hardware::future_accelerator::feature == true;
 *     }
 *
 * These examples do not establish hardware maximums.
 *
 * ============================================================================
 */