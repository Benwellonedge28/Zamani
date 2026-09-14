/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/hardware-constraints.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded target-language actions.
 *     It introduces no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL HARDWARE CONSTRAINT SYNTAX.
 *
 * A hardware constraint describes conditions that a valid hardware
 * realization must satisfy.
 *
 * It is intentionally narrower than the generic constraint system in:
 *
 *     grammar/core/constraints.g4
 *
 * The generic constraint system owns:
 *
 *     - constraint expression structure;
 *     - boolean composition;
 *     - grouping;
 *     - negation;
 *     - relational predicates;
 *     - generic constraint references;
 *     - generic type/bound constraints.
 *
 * This file owns only the hardware-domain integration layer:
 *
 *     - hardware constraint declarations;
 *     - hardware constraint clauses;
 *     - hardware constraint subjects;
 *     - hardware-specific constraint categories;
 *     - hardware resource/capability/target references;
 *     - hardware realization predicates;
 *     - hardware constraint annotations;
 *     - hardware constraint composition hooks.
 *
 * ============================================================================
 * FUNDAMENTAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * THIS FILE DOES NOT CREATE A SECOND CONSTRAINT LANGUAGE.
 *
 * Generic constraint expressions are owned by:
 *
 *     grammar/core/constraints.g4
 *
 * General expressions are owned by the expression subsystem.
 *
 * Names are owned by:
 *
 *     grammar/core/names.g4
 *
 * Capabilities are owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Requirements are owned by:
 *
 *     grammar/core/requirements.g4
 *
 * Resources are owned by:
 *
 *     grammar/resources/
 *
 * Targets are owned by:
 *
 *     grammar/compile/target.g4
 *     grammar/hardware/targets.g4
 *
 * Hardware realization is handled downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *     - hardwareConstraintClause;
 *     - hardwareConstraintDeclaration;
 *     - hardwareConstraintMember;
 *     - hardware constraint classification;
 *     - hardware resource constraint references;
 *     - hardware capability constraint references;
 *     - hardware target constraint references;
 *     - hardware implementation constraint references;
 *     - hardware portability constraint references;
 *     - hardware scalability constraint references;
 *     - hardware performance constraint references;
 *     - hardware reliability constraint references;
 *     - hardware energy constraint references;
 *     - hardware latency constraint references;
 *     - hardware timing constraint references;
 *     - hardware synthesis constraint references;
 *     - hardware interface constraint references;
 *     - hardware constraint annotations;
 *     - hardware-specific constraint wrappers around canonical constraints.
 *
 * THIS FILE DOES NOT OWN
 *
 *     - lexical tokens;
 *     - keywords;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - generic constraint semantics;
 *     - type checking;
 *     - resource allocation;
 *     - resource discovery;
 *     - hardware discovery;
 *     - physical device enumeration;
 *     - physical addresses;
 *     - topology discovery;
 *     - routing;
 *     - placement;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - synthesis;
 *     - device drivers;
 *     - runtime dispatch;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - classical IR;
 *     - HDL IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Hardware constraints exist to preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A constraint describes a property that must hold.
 *
 * It does NOT prescribe a particular implementation unless the program
 * explicitly makes that implementation part of its semantics.
 *
 * Therefore:
 *
 *     requires hardware::quantum;
 *
 * is different from:
 *
 *     where hardware::quantum::dynamic_control == true;
 *
 * and both are different from:
 *
 *     prefer target::quantum;
 *
 * and all are different from:
 *
 *     placement ...
 *
 * The grammar preserves those distinctions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file contains NO:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     MAX_LANES
 *     MAX_PORTS
 *     MAX_RESOURCES
 *
 * No fixed physical topology is encoded.
 *
 * No physical device identifier is required.
 *
 * No hardware address is required.
 *
 * Quantities are represented by expressions and are resolved by semantic
 * analysis against the available compilation/runtime resource context.
 *
 * Repetition is represented using ANTLR's unbounded:
 *
 *     *
 *     +
 *
 * operators.
 *
 * ============================================================================
 * HARDWARE-INDEPENDENT EXAMPLES
 * ============================================================================
 *
 * Valid examples include concepts such as:
 *
 *     constraint scalable_compute {
 *         hardware::compute_capacity >= workload::required_capacity;
 *     }
 *
 *     constraint quantum_execution {
 *         hardware::quantum::measurement == true;
 *     }
 *
 *     constraint dynamic_control {
 *         hardware::quantum::dynamic_control == true;
 *     }
 *
 *     constraint memory {
 *         hardware::memory_capacity >= workload::memory_requirement;
 *     }
 *
 *     constraint latency {
 *         hardware::latency <= application::latency_budget;
 *     }
 *
 * The grammar does not decide whether those constraints can actually be
 * satisfied.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum-related hardware constraints may refer to semantic capabilities:
 *
 *     hardware::quantum::measurement
 *     hardware::quantum::reset
 *     hardware::quantum::dynamic_control
 *     hardware::quantum::mid_circuit_measurement
 *     hardware::quantum::error_correction
 *     hardware::quantum::logical_qubits
 *
 * These are NOT quantum operations.
 *
 * They do NOT create:
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
 * Quantum programs are lowered to that boundary elsewhere.
 *
 * Hardware constraints only express conditions about a realization.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Hardware constraints may refer to:
 *
 *     hardware::clocking
 *     hardware::timing
 *     hardware::pipeline
 *     hardware::memory_interface
 *     hardware::synthesis
 *     hardware::combinational_logic
 *     hardware::sequential_logic
 *
 * They do not define HDL semantics.
 *
 * HDL semantics remain owned by:
 *
 *     grammar/hdl/
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Resource properties are referenced, not defined here.
 *
 * Examples:
 *
 *     resource::memory
 *     resource::compute
 *     resource::bandwidth
 *     resource::latency
 *     resource::energy
 *     resource::reliability
 *
 * This file does not determine:
 *
 *     - how resources are discovered;
 *     - how resources are allocated;
 *     - how resources are scheduled;
 *     - which physical device owns them;
 *     - whether they are local or remote.
 *
 * ============================================================================
 * TARGET BOUNDARY
 * ============================================================================
 *
 * Target references are symbolic.
 *
 * A constraint may refer to:
 *
 *     target::supports_quantum
 *     target::supports_hdl
 *     target::supports_dynamic_control
 *
 * It must not require:
 *
 *     device0
 *     gpu0
 *     qpu7
 *     fpga3
 *
 * Physical target selection belongs downstream.
 *
 * ============================================================================
 * CONSTRAINT VS REQUIREMENT VS CAPABILITY VS PREFERENCE
 * ============================================================================
 *
 * Requirement:
 *
 *     What the program needs.
 *
 * Capability:
 *
 *     What an environment can provide.
 *
 * Constraint:
 *
 *     What must be true for a realization to be valid.
 *
 * Preference:
 *
 *     Which valid realization is preferred.
 *
 * Resource:
 *
 *     Something that can be consumed or provided.
 *
 * Target:
 *
 *     A compilation/execution context.
 *
 * Hint:
 *
 *     Non-mandatory guidance.
 *
 * This file must preserve these distinctions.
 *
 * ============================================================================
 * DECLARATION FORM
 * ============================================================================
 *
 * A named hardware constraint is a reusable source-level constraint object.
 *
 *     constraint Name {
 *         ...
 *     }
 *
 * The declaration is symbolic.
 *
 * It does not execute.
 *
 * It does not query hardware.
 *
 * It does not allocate hardware.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareConstraintsParser;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * These rules are the integration surface consumed by hardware.g4 and any
 * future hardware-domain parser.
 * ============================================================================
 */


/**
 * A complete named hardware constraint declaration.
 *
 * Example:
 *
 *     constraint QuantumExecution {
 *         ...
 *     }
 */
hardwareConstraintDeclaration
    : hardwareConstraintAnnotation*
      hardwareConstraintVisibility?
      hardwareConstraintModifier*
      K_CONSTRAINT
      IDENTIFIER
      hardwareConstraintParameters?
      hardwareConstraintClause?
      LBRACE
      hardwareConstraintMember*
      RBRACE
    ;


/**
 * An inline hardware constraint clause.
 *
 * Example:
 *
 *     requires {
 *         ...
 *     }
 *
 * or a hardware construct that embeds a constraint expression.
 */
hardwareConstraintClause
    : K_CONSTRAINT
      LBRACE
      hardwareConstraintMember*
      RBRACE
    ;


/**
 * A single hardware constraint member.
 */
hardwareConstraintMember
    : hardwareConstraintAnnotation*
      hardwareConstraintStatement
    ;


/*
 * ============================================================================
 * 2. VISIBILITY
 * ============================================================================
 */

hardwareConstraintVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;


/*
 * ============================================================================
 * 3. MODIFIERS
 * ============================================================================
 *
 * Modifiers affect source-level declaration properties only.
 *
 * They do not imply a physical implementation.
 * ============================================================================
 */

hardwareConstraintModifier
    : K_STATIC
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    ;


/*
 * ============================================================================
 * 4. ANNOTATIONS
 * ============================================================================
 *
 * Annotations are metadata.
 *
 * Their semantics belong outside the grammar.
 * ============================================================================
 */

hardwareConstraintAnnotation
    : AT
      IDENTIFIER
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
    ;


hardwareConstraintAnnotationArgument
    : IDENTIFIER
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | hardwareConstraintQualifiedName
    | hardwareConstraintValue
    ;


/*
 * ============================================================================
 * 5. GENERIC PARAMETERS
 * ============================================================================
 *
 * Constraint declarations may be parameterized.
 *
 * This is essential for scalable source programs.
 *
 * Example:
 *
 *     constraint Capacity<C> {
 *         ...
 *     }
 *
 * No concrete capacity is imposed by the grammar.
 * ============================================================================
 */

hardwareConstraintParameters
    : LT
      hardwareConstraintParameter
      (
          COMMA
          hardwareConstraintParameter
      )*
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


/*
 * ============================================================================
 * 6. HARDWARE CONSTRAINT STATEMENTS
 * ============================================================================
 *
 * The actual logical expression structure remains compatible with the
 * canonical constraint model.
 *
 * Hardware-specific forms are wrappers around canonical constraint operands.
 * ============================================================================
 */

hardwareConstraintStatement
    : hardwareConstraintPredicate
      SEMICOLON
    | hardwareConstraintRequirementReference
      SEMICOLON
    | hardwareConstraintCapabilityReference
      SEMICOLON
    | hardwareConstraintResourcePredicate
      SEMICOLON
    | hardwareConstraintTargetPredicate
      SEMICOLON
    | hardwareConstraintPerformancePredicate
      SEMICOLON
    | hardwareConstraintTimingPredicate
      SEMICOLON
    | hardwareConstraintScalabilityPredicate
      SEMICOLON
    | hardwareConstraintPortabilityPredicate
      SEMICOLON
    | hardwareConstraintReliabilityPredicate
      SEMICOLON
    | hardwareConstraintEnergyPredicate
      SEMICOLON
    | hardwareConstraintSynthesisPredicate
      SEMICOLON
    | hardwareConstraintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. GENERIC HARDWARE CONSTRAINT PREDICATE
 * ============================================================================
 *
 * This is the principal open-world form.
 *
 * New hardware properties do not require new grammar rules.
 *
 * Example:
 *
 *     hardware::future_accelerator::capability == true;
 *
 * The property name is symbolic.
 * ============================================================================
 */

hardwareConstraintPredicate
    : hardwareConstraintQualifiedName
      hardwareConstraintComparisonOperator
      hardwareConstraintValue
    ;


/*
 * ============================================================================
 * 8. REQUIREMENT REFERENCES
 * ============================================================================
 *
 * Requirements are referenced, not redefined.
 * ============================================================================
 */

hardwareConstraintRequirementReference
    : K_REQUIRES
      hardwareConstraintQualifiedName
    ;


/*
 * ============================================================================
 * 9. CAPABILITY REFERENCES
 * ============================================================================
 */

hardwareConstraintCapabilityReference
    : K_CAPABILITY
      hardwareConstraintQualifiedName
    ;


/*
 * ============================================================================
 * 10. RESOURCE PREDICATES
 * ============================================================================
 *
 * Resource identity is symbolic.
 *
 * No resource quantity is hard-coded.
 * ============================================================================
 */

hardwareConstraintResourcePredicate
    : hardwareConstraintResourceReference
      hardwareConstraintComparisonOperator
      hardwareConstraintValue
    ;


hardwareConstraintResourceReference
    : K_RESOURCE
      hardwareConstraintQualifiedName
    ;


/*
 * ============================================================================
 * 11. TARGET PREDICATES
 * ============================================================================
 *
 * A target property may be constrained without selecting a physical machine.
 * ============================================================================
 */

hardwareConstraintTargetPredicate
    : K_TARGET
      hardwareConstraintQualifiedName
      (
          hardwareConstraintComparisonOperator
          hardwareConstraintValue
      )?
    ;


/*
 * ============================================================================
 * 12. PERFORMANCE
 * ============================================================================
 */

hardwareConstraintPerformancePredicate
    : K_PERFORMANCE
      hardwareConstraintQualifiedName
      hardwareConstraintComparisonOperator
      hardwareConstraintValue
    ;


/*
 * ============================================================================
 * 13. TIMING
 * ============================================================================
 *
 * Timing values are expressions/literals.
 *
 * No clock frequency or duration limit is encoded here.
 * ============================================================================
 */

hardwareConstraintTimingPredicate
    : K_TIMING
      hardwareConstraintQualifiedName
      hardwareConstraintComparisonOperator
      hardwareConstraintValue
    ;


/*
 * ============================================================================
 * 14. SCALABILITY
 * ============================================================================
 *
 * Scalability is a semantic property.
 *
 * The grammar does not impose a scale.
 * ============================================================================
 */

hardwareConstraintScalabilityPredicate
    : K_SCALABILITY
      hardwareConstraintQualifiedName
      hardwareConstraintComparisonOperator
      hardwareConstraintValue
    ;


/*
 * ============================================================================
 * 15. PORTABILITY
 * ============================================================================
 */

hardwareConstraintPortabilityPredicate
    : K_PORTABILITY
      hardwareConstraintQualifiedName
      hardwareConstraintComparisonOperator
      hardwareConstraintValue
    ;


/*
 * ============================================================================
 * 16. RELIABILITY
 * ============================================================================
 */

hardwareConstraintReliabilityPredicate
    : K_RELIABILITY
      hardwareConstraintQualifiedName
      hardwareConstraintComparisonOperator
      hardwareConstraintValue
    ;


/*
 * ============================================================================
 * 17. ENERGY
 * ============================================================================
 */

hardwareConstraintEnergyPredicate
    : K_ENERGY
      hardwareConstraintQualifiedName
      hardwareConstraintComparisonOperator
      hardwareConstraintValue
    ;


/*
 * ============================================================================
 * 18. SYNTHESIS / IMPLEMENTATION
 * ============================================================================
 */

hardwareConstraintSynthesisPredicate
    : K_SYNTHESIZE
      hardwareConstraintQualifiedName
      (
          hardwareConstraintComparisonOperator
          hardwareConstraintValue
      )?
    ;


/*
 * ============================================================================
 * 19. GENERAL HARDWARE CONSTRAINT EXPRESSION
 * ============================================================================
 *
 * This rule is deliberately open-world.
 *
 * It permits a symbolic subject followed by a canonical comparison.
 *
 * It must not enumerate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     vendor
 *     device family
 *
 * as grammar-level constraint types.
 *
 * Those are semantic classifications.
 * ============================================================================
 */

hardwareConstraintExpression
    : hardwareConstraintValue
      hardwareConstraintComparisonOperator
      hardwareConstraintValue
    ;


/*
 * ============================================================================
 * 20. COMPARISON OPERATORS
 * ============================================================================
 *
 * The lexer is authoritative for token spelling.
 *
 * Assignment '=' is deliberately excluded.
 *
 * Equality uses '=='.
 * ============================================================================
 */

hardwareConstraintComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * 21. CONSTRAINT VALUES
 * ============================================================================
 *
 * This rule intentionally remains small.
 *
 * General expression semantics belong to the expression subsystem.
 *
 * The hardware grammar must not create a second expression language.
 * ============================================================================
 */

hardwareConstraintValue
    : hardwareConstraintQualifiedName
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | hardwareConstraintLiteral
    | hardwareConstraintQualifiedCall
    | hardwareConstraintGroupedValue
    ;


hardwareConstraintLiteral
    : K_TRUE
    | K_FALSE
    | K_NULL
    | K_NIL
    ;


hardwareConstraintGroupedValue
    : LPAREN
      hardwareConstraintValue
      RPAREN
    ;


hardwareConstraintQualifiedCall
    : hardwareConstraintQualifiedName
      LPAREN
      hardwareConstraintArgumentList?
      RPAREN
    ;


hardwareConstraintArgumentList
    : hardwareConstraintValue
      (
          COMMA
          hardwareConstraintValue
      )*
    ;


/*
 * ============================================================================
 * 22. QUALIFIED NAMES
 * ============================================================================
 *
 * Name ownership remains centralized in the core name system.
 *
 * This rule is intentionally structurally compatible with Zamani's
 * qualified-name model:
 *
 *     hardware
 *     hardware::quantum
 *     hardware::quantum::measurement
 *     resource::memory
 *     target::supports_dynamic_control
 *     future::accelerator::capability
 *
 * If the final canonical core parser exposes qualifiedName through parser
 * grammar composition, this rule MUST be replaced by that imported canonical
 * rule rather than maintaining a competing name grammar.
 *
 * Until parser composition is finalized, this adapter preserves the required
 * open-world syntax without enumerating domains.
 * ============================================================================
 */

hardwareConstraintQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 23. DESIGNATED HARDWARE CONSTRAINT CATEGORIES
 * ============================================================================
 *
 * These categories are semantic namespaces rather than machine definitions.
 *
 * They are intentionally represented as qualified names rather than separate
 * device grammars.
 *
 * Examples:
 *
 *     hardware::cpu
 *     hardware::gpu
 *     hardware::fpga
 *     hardware::asic
 *     hardware::quantum
 *     hardware::accelerator
 *
 * The grammar does not require these names to exist.
 *
 * A future domain can introduce:
 *
 *     hardware::photonic
 *     hardware::neuromorphic
 *     hardware::memristive
 *     hardware::biological
 *     hardware::future
 *
 * without changing this grammar.
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. CONSTRAINT SETS
 * ============================================================================
 *
 * A reusable set can group constraints without imposing a fixed cardinality.
 * ============================================================================
 */

hardwareConstraintSet
    : K_CONSTRAINT
      IDENTIFIER
      LBRACE
      hardwareConstraintMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 25. INLINE ASSERTION
 * ============================================================================
 *
 * An assertion is source-level validation intent.
 *
 * It does not execute hardware discovery.
 * ============================================================================
 */

hardwareConstraintAssertion
    : K_ASSERT
      LPAREN
      hardwareConstraintExpression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. DOCUMENTATION / SOURCE CONTRACT
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve at least:
 *
 *     HardwareConstraintDeclaration
 *         name
 *         parameters
 *         members
 *         annotations
 *         visibility
 *         modifiers
 *         source_span
 *
 *     HardwareConstraint
 *         subject
 *         operator
 *         value
 *         category
 *         source_span
 *
 *     HardwareConstraintReference
 *         namespace
 *         name
 *         arguments
 *         source_span
 *
 * The exact Rust structures belong to the AST subsystem.
 *
 * This grammar must never embed Rust types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether referenced names resolve;
 *     - whether values have compatible types;
 *     - whether a resource property exists;
 *     - whether a capability exists;
 *     - whether a target property exists;
 *     - whether the constraint is meaningful;
 *     - whether constraints conflict;
 *     - whether constraints are satisfiable;
 *     - whether a constraint is statically decidable;
 *     - whether evaluation requires runtime information.
 *
 * The parser must accept syntactically valid references even when the
 * capability/property is introduced by a future extension.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * Hardware constraints are lowered into the repository's canonical semantic
 * constraint representation.
 *
 * This grammar MUST NOT define:
 *
 *     - hardware IR;
 *     - classical IR;
 *     - quantum IR;
 *     - QEC IR;
 *     - ZQN IR;
 *
 * Quantum program semantics continue through:
 *
 *     quantum::ir
 *
 * Hardware constraints may accompany those semantic artifacts as
 * realization requirements/constraints.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler flow:
 *
 *     Zamani source
 *         |
 *         v
 *     lexer
 *         |
 *         v
 *     parser
 *         |
 *         v
 *     HardwareConstraint AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     canonical constraint model
 *         |
 *         +--> capability analysis
 *         +--> resource analysis
 *         +--> target analysis
 *         +--> compilation planning
 *         |
 *         v
 *     target realization
 *
 * The compiler may reject an unsatisfied constraint.
 *
 * The grammar itself must not.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may evaluate constraints that depend on runtime-discovered
 * capabilities/resources.
 *
 * This file does not perform that evaluation.
 *
 * Runtime MUST NOT need to parse source merely to discover hardware.
 *
 * Parsed/validated constraint representations should be available in the
 * compiled semantic artifact.
 *
 * ============================================================================
 * HARDWARE HAL INTEGRATION
 * ============================================================================
 *
 * The Hardware HAL supplies discovered facts.
 *
 * Example:
 *
 *     capability available
 *     resource available
 *     timing property
 *     memory capacity
 *     supported execution mode
 *
 * The constraint model consumes those facts.
 *
 * hardware-constraints.g4 does not query the HAL.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum constraint can state:
 *
 *     hardware::quantum::measurement == true;
 *
 *     hardware::quantum::dynamic_control == true;
 *
 *     hardware::quantum::logical_qubits >= required_logical_qubits;
 *
 * This does not create physical qubit references.
 *
 * Routing, placement, scheduling, calibration and execution remain outside
 * this grammar.
 *
 * QEC remains owned by the QEC subsystem.
 *
 * ZQN remains responsible for noise/fault semantics.
 *
 * ============================================================================
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Constraints may influence optimization legality.
 *
 * Example:
 *
 *     hardware::pipeline::supported == true;
 *
 * Optimization determines an implementation satisfying that condition.
 *
 * This grammar does not optimize anything.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Timing constraints may be consumed by scheduling:
 *
 *     hardware::timing::latency <= latency_budget;
 *
 *     hardware::timing::period <= required_period;
 *
 * The grammar does not create schedules.
 *
 * ============================================================================
 * ROUTING / PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Hardware constraints may restrict legal realizations.
 *
 * They must not encode an actual placement or routing solution.
 *
 * For example:
 *
 *     hardware::connectivity::required == connectivity_requirement;
 *
 * is a constraint.
 *
 * A concrete coupling map and route belong downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar has:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no random behavior;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime calls.
 *
 * Equal token streams produce equal parse structures.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Adding a new qualified hardware property MUST NOT require a grammar change.
 *
 * Adding a new physical device MUST NOT require a grammar change.
 *
 * Adding a new accelerator family MUST NOT require a grammar change.
 *
 * Adding a new quantum processor MUST NOT require a grammar change.
 *
 * Adding a new hardware vendor MUST NOT require a grammar change.
 *
 * New reserved keywords require the normal lexer/versioning compatibility
 * process.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_DEVICES
 *     fixed_memory
 *     fixed_topology
 *     fixed_address
 *
 * Symbolic names are permitted:
 *
 *     hardware::quantum::logical_qubits
 *     resource::memory
 *     target::supports_dynamic_control
 *
 * because their values are resolved outside the grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     - simple hardware constraints;
 *     - qualified hardware properties;
 *     - resource constraints;
 *     - capability constraints;
 *     - target constraints;
 *     - timing constraints;
 *     - performance constraints;
 *     - energy constraints;
 *     - reliability constraints;
 *     - portability constraints;
 *     - scalability constraints;
 *     - quantum capability constraints;
 *     - HDL capability constraints;
 *     - generic/future qualified names;
 *     - parameterized constraints;
 *     - repeated constraint members;
 *     - nested qualified names.
 *
 * Negative tests MUST cover:
 *
 *     - assignment used as equality;
 *     - missing constraint subject;
 *     - missing comparison operator;
 *     - malformed qualified names;
 *     - malformed argument lists;
 *     - malformed declarations;
 *     - invalid delimiter placement;
 *     - incomplete expressions.
 *
 * Boundary tests MUST cover:
 *
 *     - zero constraint members;
 *     - one constraint;
 *     - many constraints;
 *     - deeply qualified names;
 *     - deeply nested groups;
 *     - large symbolic expressions;
 *     - large source files;
 *     - generated/future hardware namespaces.
 *
 * Scalability tests MUST verify that the grammar itself imposes no maximum
 * hardware/resource quantity.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] It parses hardware constraint declarations.
 *     [ ] It parses inline hardware constraint clauses.
 *     [ ] It uses ZamaniTokens.
 *     [ ] It contains no lexer rules.
 *     [ ] It contains no Rust actions.
 *     [ ] It contains no unsafe code.
 *     [ ] It contains no physical hardware assumptions.
 *     [ ] It contains no fixed resource limits.
 *     [ ] It contains no fixed qubit limits.
 *     [ ] It contains no fixed topology.
 *     [ ] It does not create a second quantum IR.
 *     [ ] It does not duplicate generic constraint semantics unnecessarily.
 *     [ ] It integrates with the canonical constraint model.
 *     [ ] It supports open-ended hardware namespaces.
 *     [ ] It supports symbolic resource quantities.
 *     [ ] It supports future hardware capabilities.
 *     [ ] It preserves source structure for AST construction.
 *     [ ] It has positive tests.
 *     [ ] It has negative tests.
 *     [ ] It has boundary tests.
 *     [ ] It has scalability tests.
 *     [ ] It has deterministic parsing.
 *     [ ] Its hardware.g4 integration has been completed.
 *     [ ] Its root-parser integration has been completed.
 *
 * ============================================================================
 */