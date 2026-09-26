/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/budgets.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Grammar identity:
 *     ResourceBudgets
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the canonical SOURCE-LEVEL RESOURCE-BUDGET grammar.
 *
 * A budget is a declared allowance, envelope, objective boundary, or
 * consumption policy associated with a semantic resource.
 *
 * A budget is NOT:
 *
 *     - a compiler hard limit;
 *     - a hardware maximum;
 *     - a parser maximum;
 *     - a fixed machine capacity;
 *     - a physical device selection;
 *     - a scheduling implementation;
 *     - a runtime allocator;
 *     - a resource inventory;
 *     - a hardware discovery mechanism.
 *
 * Budgets are therefore portable semantic intent.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A budget describes what the program is allowed, expected, or required to
 * consume or remain within.
 *
 * It does NOT select:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     physical qubit 0
 *     node 0
 *     memory bank 0
 *
 * nor does it establish universal machine capacities.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
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
 * It also MUST NOT encode equivalent fixed assumptions such as:
 *
 *     32-bit registers
 *     64 GB RAM
 *     24 GB VRAM
 *     1024 physical qubits
 *     8 CPU cores
 *     16 GPU devices
 *
 * A source-level value such as:
 *
 *     budget memory = required_memory;
 *
 * or:
 *
 *     budget qubits = logical_qubits;
 *
 * is program semantics.
 *
 * It is not a language-level hardware limit.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Budget expressions are deliberately open-ended.
 *
 * The grammar imposes no finite limit on:
 *
 *     number of budgets
 *     number of budget clauses
 *     budget nesting
 *     budget expression complexity
 *     number of resources
 *     number of resource dimensions
 *     number of conditions
 *     number of scopes
 *     number of targets
 *
 * ANTLR repetition and recursive structure represent arbitrary source
 * cardinality subject only to the actual implementation's externally
 * configurable parsing resources.
 *
 * "Infinity" therefore means:
 *
 *     no language-level artificial capacity limit.
 *
 * It does NOT mean that physical hardware or compiler memory is infinite.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     resourceBudgets
 *     resourceBudgetItem
 *     resourceBudgetDeclaration
 *     resourceBudgetSpecification
 *     resourceBudgetClause
 *     resourceBudgetLimitClause
 *     resourceBudgetMinimumClause
 *     resourceBudgetMaximumClause
 *     resourceBudgetTargetClause
 *     resourceBudgetReserveClause
 *     resourceBudgetConsumeClause
 *     resourceBudgetRemainingClause
 *     resourceBudgetScopeClause
 *     resourceBudgetConditionClause
 *     resourceBudgetPriorityClause
 *     resourceBudgetPolicyClause
 *     resourceBudgetPropertyClause
 *     resourceBudgetExpression
 *     resourceBudgetRelation
 *     resourceBudgetOperator
 *     resourceBudgetName
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifier
 *     qualifiedName
 *     literals
 *     operators
 *     general expression precedence
 *     resourceExpression
 *     resource requirements
 *     resource capabilities
 *     resource constraints
 *     resource preferences
 *     resource placement
 *     hardware discovery
 *     scheduling
 *     routing
 *     optimization
 *     runtime allocation
 *     physical device selection
 *     QEC
 *     ZQN
 *     HAL
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser is expected to map:
 *
 *     resourceBudgetDeclaration
 *         ->
 *     ResourceBudgetDeclaration
 *
 *     resourceBudgetClause
 *         ->
 *     ResourceBudgetClause
 *
 *     resourceBudgetLimitClause
 *         ->
 *     ResourceBudgetLimit
 *
 *     resourceBudgetMinimumClause
 *         ->
 *     ResourceBudgetMinimum
 *
 *     resourceBudgetMaximumClause
 *         ->
 *     ResourceBudgetMaximum
 *
 *     resourceBudgetTargetClause
 *         ->
 *     ResourceBudgetTarget
 *
 *     resourceBudgetReserveClause
 *         ->
 *     ResourceBudgetReservation
 *
 *     resourceBudgetConsumeClause
 *         ->
 *     ResourceBudgetConsumption
 *
 *     resourceBudgetConditionClause
 *         ->
 *     ResourceBudgetCondition
 *
 * The AST must preserve source spans and the original resource/budget
 * expressions.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - what resource the budget refers to;
 *     - what quantity/dimension is being budgeted;
 *     - whether the budget is a limit, minimum, target, reservation, or policy;
 *     - whether it is mandatory or advisory;
 *     - whether it is conditional;
 *     - how it composes with requirements and constraints;
 *     - whether it is satisfiable on a selected target;
 *     - whether the value has a valid semantic unit/type.
 *
 * Parsing MUST NOT make those decisions.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE DISTINCTION
 * ============================================================================
 *
 * A budget is not automatically a requirement.
 *
 * Example:
 *
 *     budget latency = latency_budget;
 *
 * merely declares a budget.
 *
 * A containing semantic construct may subsequently make it:
 *
 *     required
 *     constrained
 *     preferred
 *     advisory
 *
 * A budget MUST NOT silently change a preference into a hard constraint.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define a ResourceBudget IR.
 *
 * Budget syntax is lowered into the repository's canonical semantic resource
 * model and then consumed by existing compiler/resource infrastructure.
 *
 * Conceptually:
 *
 *     source
 *       |
 *       v
 *     ResourceBudgetDeclaration
 *       |
 *       v
 *     semantic ResourceBudget
 *       |
 *       +---------------------+
 *       |                     |
 *       v                     v
 *     resource analysis   performance analysis
 *       |                     |
 *       +----------+----------+
 *                  |
 *                  v
 *          canonical semantic model
 *                  |
 *          +-------+--------+
 *          |                |
 *          v                v
 *     classical          quantum::ir
 *          |                |
 *          +-------+--------+
 *                  |
 *             optimization
 *                  |
 *          routing/scheduling
 *                  |
 *              resilience
 *                  |
 *                 ZQN
 *                  |
 *                 HAL
 *
 * No second resource IR and no second quantum IR are introduced here.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Primary integration:
 *
 *     grammar/resources/resources.g4
 *
 * imports:
 *
 *     ResourceBudgets
 *
 * and exposes:
 *
 *     resourceBudgetDeclaration
 *
 * as a resource-domain item/clause.
 *
 * Related grammars:
 *
 *     requirements.g4
 *     constraints.g4
 *     capabilities.g4
 *     preferences.g4
 *     scaling.g4
 *     performance.g4
 *     placement.g4
 *     hardware/*.g4
 *     execution/*.g4
 *
 * Those grammars retain ownership of their respective concepts.
 *
 * ============================================================================
 * IMPORTANT NON-DUPLICATION RULE
 * ============================================================================
 *
 * ResourceExpressions owns:
 *
 *     resourceExpression
 *
 * This file MUST NOT redefine:
 *
 *     resourceExpression
 *     arithmeticExpression
 *     logicalExpression
 *     comparisonExpression
 *     functionCall
 *     indexing
 *     member access
 *     literals
 *
 * All budget values pass through:
 *
 *     resourceBudgetExpression
 *         -> resourceExpression
 *
 * ============================================================================
 */

parser grammar ResourceBudgets;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A budget grammar may contain zero or more budget items.
 *
 * No EOF is consumed here.
 *
 * EOF belongs to the canonical Zamani composition root.
 * ============================================================================
 */

resourceBudgets
    : resourceBudgetItem*
    ;


/*
 * ============================================================================
 * 2. BUDGET ITEM
 * ============================================================================
 */

resourceBudgetItem
    : resourceBudgetDeclaration
    ;


/*
 * ============================================================================
 * 3. BUDGET DECLARATION
 * ============================================================================
 *
 * Simple form:
 *
 *     budget memory = required_memory;
 *
 *     budget qubits = logical_qubits;
 *
 *     budget latency = latency_budget;
 *
 *     budget energy = energy_budget;
 *
 * Structured form:
 *
 *     budget execution {
 *         limit = execution_limit;
 *         reserve = reserved_capacity;
 *         consume = expected_consumption;
 *     };
 *
 * The budget name is semantic data, not a finite enum.
 * ============================================================================
 */

resourceBudgetDeclaration
    : BUDGET
      resourceBudgetName
      resourceBudgetSpecification
    ;


/*
 * ============================================================================
 * 4. BUDGET NAME
 * ============================================================================
 *
 * Budget names are open-world.
 *
 * Examples:
 *
 *     memory
 *     latency
 *     energy
 *     qubits
 *     execution
 *     quantum::error
 *     network::bandwidth
 *     accelerator::time
 *     future::resource
 *
 * No finite list of budget dimensions is encoded.
 * ============================================================================
 */

resourceBudgetName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * 5. BUDGET SPECIFICATION
 * ============================================================================
 *
 * Simple assignment:
 *
 *     budget memory = required_memory;
 *
 * Structured declaration:
 *
 *     budget memory {
 *         limit = required_memory;
 *         reserve = reserved_memory;
 *     };
 *
 * ============================================================================
 */

resourceBudgetSpecification
    : ASSIGN
      resourceBudgetExpression
      SEMICOLON
    | LBRACE
      resourceBudgetClause*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. BUDGET CLAUSE
 * ============================================================================
 *
 * Every clause is independently semantic.
 *
 * ============================================================================
 */

resourceBudgetClause
    : resourceBudgetLimitClause
    | resourceBudgetMinimumClause
    | resourceBudgetMaximumClause
    | resourceBudgetTargetClause
    | resourceBudgetReserveClause
    | resourceBudgetConsumeClause
    | resourceBudgetRemainingClause
    | resourceBudgetScopeClause
    | resourceBudgetConditionClause
    | resourceBudgetPriorityClause
    | resourceBudgetPolicyClause
    | resourceBudgetPropertyClause
    ;


/*
 * ============================================================================
 * 7. LIMIT
 * ============================================================================
 *
 * A limit expresses a maximum permitted semantic quantity.
 *
 * It does NOT establish a universal hardware maximum.
 *
 * Example:
 *
 *     budget execution {
 *         limit = execution_budget;
 *     };
 *
 * ============================================================================
 */

resourceBudgetLimitClause
    : LIMIT
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. MINIMUM
 * ============================================================================
 *
 * A minimum expresses a lower semantic boundary.
 *
 * ============================================================================
 */

resourceBudgetMinimumClause
    : MINIMUM
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. MAXIMUM
 * ============================================================================
 *
 * `maximum` is a value associated with this particular budget declaration.
 *
 * It is NOT:
 *
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CPUS
 *
 * and MUST NOT be interpreted as a universal machine capacity.
 *
 * ============================================================================
 */

resourceBudgetMaximumClause
    : MAXIMUM
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. TARGET
 * ============================================================================
 *
 * A target is an objective/reference value.
 *
 * It is not automatically a hard constraint.
 * ============================================================================
 */

resourceBudgetTargetClause
    : TARGET
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. RESERVE
 * ============================================================================
 *
 * Reservation intent is semantic.
 *
 * Actual reservation is performed downstream by resource-management systems.
 * ============================================================================
 */

resourceBudgetReserveClause
    : RESERVE
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. CONSUME
 * ============================================================================
 *
 * Describes expected or declared consumption.
 *
 * It does not perform allocation.
 * ============================================================================
 */

resourceBudgetConsumeClause
    : CONSUME
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. REMAINING
 * ============================================================================
 *
 * Describes an explicitly modelled remaining allowance.
 *
 * Runtime calculation is downstream.
 * ============================================================================
 */

resourceBudgetRemainingClause
    : REMAINING
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. SCOPE
 * ============================================================================
 *
 * Budget scope identifies the semantic region to which the budget applies.
 *
 * Examples:
 *
 *     scope = program;
 *     scope = function;
 *     scope = quantum_kernel;
 *     scope = execution;
 *     scope = deployment;
 *
 * The grammar does not enumerate scopes.
 * ============================================================================
 */

resourceBudgetScopeClause
    : SCOPE
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. CONDITION
 * ============================================================================
 *
 * Conditional budget application.
 *
 * Example:
 *
 *     condition = workload_size > threshold;
 *
 * The expression remains owned by ResourceExpressions.
 * ============================================================================
 */

resourceBudgetConditionClause
    : CONDITION
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. PRIORITY
 * ============================================================================
 *
 * Priority is policy metadata.
 *
 * The grammar deliberately does not constrain the representation to:
 *
 *     0..10
 *     0..100
 *     integer-only
 *
 * ============================================================================
 */

resourceBudgetPriorityClause
    : PRIORITY
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. POLICY
 * ============================================================================
 *
 * Open-world policy value.
 *
 * Examples:
 *
 *     policy = strict;
 *     policy = best_effort;
 *     policy = adaptive;
 *     policy = quantum::resilience;
 *     policy = future::policy;
 *
 * The policy semantics belong downstream.
 * ============================================================================
 */

resourceBudgetPolicyClause
    : POLICY
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * Future budget dimensions must not require a grammar rewrite.
 *
 * Examples:
 *
 *     thermal_margin = required_margin;
 *
 *     network::egress = egress_budget;
 *
 *     quantum::sampling = sampling_budget;
 *
 *     accelerator::occupancy = occupancy_budget;
 *
 *     future::metric = symbolic_budget;
 *
 * ============================================================================
 */

resourceBudgetPropertyClause
    : resourceBudgetPropertyName
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


resourceBudgetPropertyName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * 19. BUDGET EXPRESSION
 * ============================================================================
 *
 * ResourceExpressions is authoritative.
 *
 * ============================================================================
 */

resourceBudgetExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 20. OPTIONAL BUDGET RELATION
 * ============================================================================
 *
 * Relations are useful when a budget is attached to an explicit comparison.
 *
 * Examples:
 *
 *     latency <= latency_budget
 *     energy <= energy_budget
 *     throughput >= required_throughput
 *
 * The underlying expressions remain canonical resource expressions.
 * ============================================================================
 */

resourceBudgetRelation
    : resourceBudgetExpression
      resourceBudgetOperator
      resourceBudgetExpression
    ;


/*
 * ============================================================================
 * 21. BUDGET OPERATOR
 * ============================================================================
 *
 * These tokens are supplied by ZamaniLexer.
 *
 * No comparison semantics are implemented here.
 * ============================================================================
 */

resourceBudgetOperator
    : EQ_EQ
    | NOT_EQ
    | LE
    | GE
    | LESS
    | GREATER
    ;


/*
 * ============================================================================
 * 22. SEMANTIC EXAMPLES
 * ============================================================================
 *
 * The following are intended source forms:
 *
 *     budget memory = required_memory;
 *
 *     budget qubits = logical_qubits;
 *
 *     budget latency = latency_budget;
 *
 *     budget energy = energy_budget;
 *
 *     budget quantum::error = error_budget;
 *
 *     budget execution {
 *         limit = execution_budget;
 *         reserve = reserved_capacity;
 *         consume = estimated_consumption;
 *         scope = execution;
 *         condition = workload_size > threshold;
 *         priority = execution_priority;
 *         policy = adaptive;
 *     };
 *
 *     budget network::bandwidth {
 *         maximum = available_bandwidth;
 *         target = desired_bandwidth;
 *     };
 *
 *     budget accelerator::memory {
 *         minimum = required_memory;
 *         maximum = available_memory;
 *     };
 *
 * These are semantic examples only.
 *
 * They do not establish any physical capacity.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum programs may declare:
 *
 *     budget qubits = logical_qubits;
 *
 *     budget quantum::time = execution_time_budget;
 *
 *     budget quantum::error = error_budget;
 *
 *     budget quantum::fidelity = minimum_fidelity;
 *
 *     budget quantum::sampling = sampling_budget;
 *
 * The grammar does NOT:
 *
 *     - enumerate gates;
 *     - enumerate QPUs;
 *     - enumerate qubits;
 *     - select physical qubits;
 *     - define QEC;
 *     - define ZQN;
 *     - define routing;
 *     - define pulse scheduling.
 *
 * Quantum semantics continue through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Budgets can describe:
 *
 *     memory
 *     execution time
 *     computation
 *     communication
 *     storage
 *     energy
 *     bandwidth
 *     throughput
 *     cost
 *
 * without imposing a fixed CPU/GPU/register model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware descriptions may use budgets for intent such as:
 *
 *     budget latency = datapath_latency;
 *
 *     budget power = power_budget;
 *
 *     budget energy = energy_budget;
 *
 *     budget memory = required_storage;
 *
 *     budget bandwidth = required_bandwidth;
 *
 * The budget does not select an FPGA, ASIC, process node, physical region,
 * register width, or vendor device.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed programs may express:
 *
 *     budget network::bandwidth = required_bandwidth;
 *
 *     budget network::latency = latency_budget;
 *
 *     budget communication::volume = communication_budget;
 *
 *     budget storage = required_storage;
 *
 * No node-count maximum is encoded.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. AI / TENSOR INTEGRATION
 * ============================================================================
 *
 * Budgets may apply to:
 *
 *     training
 *     inference
 *     tensor memory
 *     communication
 *     accelerator time
 *     energy
 *     latency
 *     throughput
 *
 * Tensor dimensions remain semantic data.
 *
 * No MAX_TENSOR_RANK or fixed accelerator memory is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. RELATIONSHIP WITH REQUIREMENTS
 * ============================================================================
 *
 * These are intentionally different:
 *
 *     requires memory >= required_memory;
 *
 *     budget memory = memory_budget;
 *
 * A requirement says:
 *
 *     this condition must be satisfiable.
 *
 * A budget says:
 *
 *     this allowance/objective exists.
 *
 * Semantic analysis may relate them, but parsing must preserve the distinction.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. RELATIONSHIP WITH CONSTRAINTS
 * ============================================================================
 *
 * These are also distinct:
 *
 *     constraint latency <= latency_budget;
 *
 *     budget latency = latency_budget;
 *
 * The first is a constraint.
 *
 * The second declares a budget.
 *
 * A downstream semantic phase may use the budget as the source of a
 * constraint, depending on the enclosing policy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. RELATIONSHIP WITH PREFERENCES
 * ============================================================================
 *
 * A budget MUST NOT automatically imply:
 *
 *     prefer
 *
 * or:
 *
 *     require
 *
 * The enclosing semantic context determines the strength.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. RESOURCE DISCOVERY
 * ============================================================================
 *
 * This grammar never queries:
 *
 *     CPU count
 *     GPU count
 *     FPGA resources
 *     QPU size
 *     physical memory
 *     network topology
 *     storage capacity
 *
 * Such information is supplied by:
 *
 *     compiler
 *     target description
 *     hardware abstraction layer
 *     runtime
 *     deployment environment
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. SOURCE PORTABILITY
 * ============================================================================
 *
 * A source program containing:
 *
 *     budget memory = required_memory;
 *
 * must remain syntactically valid regardless of whether a target currently
 * has:
 *
 *     very little memory
 *     substantial memory
 *     distributed memory
 *     accelerator memory
 *     unified memory
 *     quantum memory
 *     future memory architecture
 *
 * Whether the target can satisfy the semantic requirement is a downstream
 * question.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. IMPLEMENTATION LIMITS
 * ============================================================================
 *
 * Compiler/runtime implementations may have externally configured resource
 * budgets for:
 *
 *     parsing
 *     AST construction
 *     semantic analysis
 *     compilation
 *     optimization
 *     diagnostics
 *     execution
 *
 * Such implementation budgets MUST NOT be encoded by this grammar.
 *
 * If an implementation budget is exhausted, the compiler must report an
 * implementation/resource diagnostic rather than treating valid source syntax
 * as invalid language syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic actions
 *     no predicates
 *     no embedded Rust
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no random behavior
 *     no mutable global state
 *
 * Equivalent token streams therefore have deterministic grammatical meaning.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. SAFETY
 * ============================================================================
 *
 * This grammar requires no unsafe Rust.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Handwritten Zamani compiler code remains safe Rust only.
 *
 * This file contains no embedded Rust actions or semantic predicates.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Syntax errors:
 *
 *     malformed budget declaration
 *
 * belong to parsing.
 *
 * Semantic errors:
 *
 *     unknown budget type
 *     incompatible budget unit
 *     invalid budget relationship
 *
 * belong to semantic analysis.
 *
 * Resource errors:
 *
 *     target cannot satisfy the budget
 *
 * belong to resource/target analysis.
 *
 * Runtime errors:
 *
 *     actual execution exceeds a runtime policy
 *
 * belong to runtime/resilience systems.
 *
 * The grammar must not collapse these categories.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. COMPATIBILITY
 * ============================================================================
 *
 * Adding new budget property names should normally be backward compatible
 * because property names are open-world.
 *
 * Adding a new semantic interpretation requires:
 *
 *     specification update
 *     AST contract
 *     semantic contract
 *     IR contract if applicable
 *     conformance tests
 *     compatibility documentation
 *
 * Removing or changing the meaning of an existing budget construct requires
 * the normal Zamani compatibility/deprecation process.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     budget memory = required_memory;
 *
 *     budget qubits = logical_qubits;
 *
 *     budget latency = latency_budget;
 *
 *     budget quantum::error = error_budget;
 *
 *     budget execution {
 *         limit = execution_budget;
 *         reserve = reserved_capacity;
 *         consume = estimated_consumption;
 *     };
 *
 *     budget network::bandwidth {
 *         minimum = required_bandwidth;
 *         maximum = available_bandwidth;
 *     };
 *
 * Negative tests MUST include:
 *
 *     missing budget name
 *     missing assignment expression
 *     malformed structured budget
 *     missing semicolon where required
 *     malformed property path
 *
 * Boundary tests MUST include:
 *
 *     empty budget block
 *     deeply nested expressions
 *     long qualified names
 *     many budget declarations
 *     many budget clauses
 *     symbolic quantities
 *     very large numeric literals
 *
 * Scalability tests MUST verify that no grammar-level artificial capacity
 * exists for:
 *
 *     budgets
 *     clauses
 *     resources
 *     resource dimensions
 *     expression complexity
 *
 * Cross-domain tests MUST cover:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     networking
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no MAX_QUBITS
 *     no MAX_CPUS
 *     no MAX_GPUS
 *     no MAX_FPGAS
 *     no MAX_NODES
 *     no MAX_MEMORY
 *     no MAX_THREADS
 *     no MAX_TENSOR_RANK
 *     no MAX_REGISTER_WIDTH
 *     no MAX_NETWORK_SIZE
 *     no MAX_DEVICE_COUNT
 *
 * It also contains no physical device identifiers.
 *
 * All quantities remain expressions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. COMPLETION CRITERIA
 * ============================================================================
 *
 * ResourceBudgets is complete when:
 *
 * [ ] ResourceBudgets is the only owner of budget declaration syntax.
 *
 * [ ] ResourceExpressions remains the only owner of resource expressions.
 *
 * [ ] Names remains the only owner of canonical names.
 *
 * [ ] ZamaniLexer supplies every required token.
 *
 * [ ] No undefined lexer token is referenced.
 *
 * [ ] No EOF is consumed by this imported grammar.
 *
 * [ ] Resources.g4 can import this grammar without creating a competing
 *     resource root.
 *
 * [ ] Hardware grammar can consume the resulting semantic budget.
 *
 * [ ] Execution grammar can consume budget semantics downstream.
 *
 * [ ] Quantum budgets lower through the existing quantum::ir architecture.
 *
 * [ ] No second resource IR is introduced.
 *
 * [ ] No second quantum IR is introduced.
 *
 * [ ] No physical hardware assumptions are encoded.
 *
 * [ ] No fixed resource capacities are encoded.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Rust 1.97 / 1.97.1 generated-parser integration succeeds.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] AST mapping is documented.
 *
 * [ ] Semantic mapping is documented.
 *
 * [ ] IR mapping is documented.
 *
 * [ ] Downstream consumers are documented.
 *
 * [ ] No later file needs to redefine the ownership established here.
 *
 * ============================================================================
 */