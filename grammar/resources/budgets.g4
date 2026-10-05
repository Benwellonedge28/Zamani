/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/budgets.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Grammar name:
 *     ResourceBudgets
 *
 * Status:
 *     CANONICAL RESOURCE-BUDGET SYNTAX
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the source-level syntax for RESOURCE BUDGETS.
 *
 * A budget describes a portable semantic allowance, envelope, objective,
 * accounting quantity, consumption model, or resource-management boundary.
 *
 * A budget is NOT:
 *
 *     - a physical hardware allocation;
 *     - a device selection;
 *     - a compiler hard limit;
 *     - a universal machine maximum;
 *     - a runtime allocator;
 *     - a hardware-discovery mechanism;
 *     - a scheduling implementation;
 *     - a routing implementation;
 *     - a quantum-routing implementation;
 *     - a QEC implementation;
 *     - a HAL implementation.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Budgets participate in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A budget belongs to PROGRAM INTENT.
 *
 * It may be evaluated against:
 *
 *     compile-time resources;
 *     execution resources;
 *     deployment resources;
 *     runtime resources;
 *     simulation resources;
 *     quantum resources;
 *     classical resources;
 *     accelerator resources;
 *     distributed resources;
 *     networking resources;
 *     future resource domains.
 *
 * This grammar MUST NOT encode a finite machine universe.
 *
 * In particular, this grammar MUST NOT encode:
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
 * or equivalent constants.
 *
 * A source program may legitimately contain a concrete budget value.
 *
 * For example:
 *
 *     budget memory = required_memory;
 *
 * The value is a PROGRAM-SPECIFIC budget.
 *
 * It is not a universal language limit.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no grammar-defined maximum for:
 *
 *     budget count;
 *     clause count;
 *     resource-name depth;
 *     expression complexity;
 *     property depth;
 *     namespace depth;
 *     resource dimensions;
 *     conditions;
 *     scopes;
 *     policies;
 *     targets;
 *     budget groups.
 *
 * Repetition uses ANTLR's unbounded grammar constructs.
 *
 * Practical limits are implementation-resource limits such as:
 *
 *     available memory;
 *     parser configuration;
 *     compiler resources;
 *     operating-system resources;
 *     deployment resources.
 *
 * Such implementation limits are NOT language semantics.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     resourceBudgets
 *     resourceBudgetItem
 *     resourceBudgetDeclaration
 *     resourceBudgetSpecification
 *     resourceBudgetAssignment
 *     resourceBudgetRelation
 *     resourceBudgetBlock
 *     resourceBudgetClause
 *     resourceBudgetNamedClause
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
 *     resourceBudgetRelationOperator
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     identifiers;
 *     qualified names;
 *     general expressions;
 *     arithmetic;
 *     logical expressions;
 *     literals;
 *     operators;
 *     requirements;
 *     capabilities;
 *     constraints;
 *     preferences;
 *     hints;
 *     negotiation;
 *     scalability semantics;
 *     policies;
 *     effects;
 *     contracts;
 *     hardware discovery;
 *     allocation;
 *     placement;
 *     routing;
 *     scheduling;
 *     optimization;
 *     classical IR;
 *     quantum::ir;
 *     HDL IR;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     runtime behavior.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/resources/resource-expressions.g4
 *     grammar/core/names.g4
 *
 *
 * DIRECT IMPORTS
 * --------------
 *
 *     ResourceExpressions
 *     Names
 *
 *
 * EXPORTS
 * -------
 *
 * Primary public rules:
 *
 *     resourceBudgets
 *     resourceBudgetItem
 *     resourceBudgetDeclaration
 *     resourceBudgetSpecification
 *     resourceBudgetClause
 *     resourceBudgetExpression
 *
 *
 * CONSUMED BY
 * -----------
 *
 *     grammar/resources/resources.g4
 *     grammar/declarations/resources.g4
 *     grammar/execution/
 *     grammar/compile/
 *     grammar/hardware/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/hdl/
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser contexts only.
 *
 * The domain-neutral frontend AST owns the actual AST representation.
 *
 * The AST should preserve:
 *
 *     budget identity;
 *     source span;
 *     source ordering;
 *     budget specification kind;
 *     expression structure;
 *     clause ordering;
 *     clause names;
 *     clause values;
 *     grouping;
 *     conditional structure.
 *
 * The AST MUST NOT contain:
 *
 *     physical CPU IDs;
 *     physical GPU IDs;
 *     physical FPGA IDs;
 *     physical QPU IDs;
 *     physical qubit IDs;
 *     memory-bank IDs;
 *     node IDs;
 *     vendor allocation decisions;
 *     routing decisions;
 *     scheduling decisions.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - what resource the budget identifies;
 *     - what dimension or property is being budgeted;
 *     - whether a value is a limit, minimum, target, reservation, etc.;
 *     - whether the budget is hard, soft, advisory, or policy-controlled;
 *     - whether units are compatible;
 *     - whether expressions are type-correct;
 *     - whether budgets conflict;
 *     * whether budgets compose with requirements;
 *     * whether budgets compose with constraints;
 *     * whether budgets compose with preferences;
 *     * whether a target can satisfy them.
 *
 * None of those decisions are made by this parser.
 *
 *
 * ============================================================================
 * RESOURCE MODEL CONTRACT
 * ============================================================================
 *
 * Resource concepts remain separate:
 *
 *     requirement
 *         = what must be satisfied
 *
 *     capability
 *         = what an environment can provide
 *
 *     constraint
 *         = a condition that must hold
 *
 *     preference
 *         = a preferred realization
 *
 *     hint
 *         = advisory information
 *
 *     budget
 *         = an allowance/objective/accounting boundary
 *
 *     negotiation
 *         = resolution of competing requirements and available realization
 *
 * Budgets MUST NOT silently become requirements, constraints, or preferences.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file defines NO resource-budget IR.
 *
 * The pipeline is:
 *
 *     source
 *       |
 *       v
 *     parser context
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic ResourceBudget
 *       |
 *       v
 *     canonical semantic resource model
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 * classical realization    quantum semantic model
 *                               |
 *                               v
 *                           quantum::ir
 *                               |
 *                               v
 *                         optimization
 *                               |
 *                         routing/scheduling
 *                               |
 *                          resilience/QEC
 *                               |
 *                              ZQN
 *                               |
 *                              HAL
 *
 * This grammar MUST NOT introduce a second resource IR.
 *
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum budgets are represented through ordinary resource names and
 * expressions.
 *
 * Examples:
 *
 *     budget quantum::time = execution_budget;
 *
 *     budget quantum::error = error_budget;
 *
 *     budget quantum::fidelity = fidelity_target;
 *
 *     budget quantum::sampling = sampling_budget;
 *
 *     budget quantum::logical_qubits = logical_qubits;
 *
 * The grammar does NOT:
 *
 *     enumerate gates;
 *     enumerate QPUs;
 *     enumerate physical qubits;
 *     define coupling maps;
 *     define calibration;
 *     define QEC;
 *     define routing;
 *     define pulse schedules.
 *
 * Quantum realization remains downstream through:
 *
 *     quantum::ir
 *
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware and HDL budgets remain target-independent.
 *
 * Examples:
 *
 *     budget latency = datapath_latency;
 *
 *     budget power = power_budget;
 *
 *     budget energy = energy_budget;
 *
 *     budget memory = storage_budget;
 *
 *     budget bandwidth = bandwidth_budget;
 *
 * They do not select:
 *
 *     FPGA;
 *     ASIC;
 *     CPU;
 *     GPU;
 *     QPU;
 *     vendor;
 *     board;
 *     pin;
 *     process node;
 *     physical region.
 *
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Distributed programs may budget:
 *
 *     network::bandwidth;
 *     network::latency;
 *     communication::volume;
 *     storage;
 *     execution::time;
 *     service::capacity;
 *
 * No node-count ceiling is encoded.
 *
 *
 * ============================================================================
 * AI / DATA / TENSOR CONTRACT
 * ============================================================================
 *
 * Budgets may describe:
 *
 *     training;
 *     inference;
 *     tensor memory;
 *     accelerator time;
 *     communication;
 *     throughput;
 *     latency;
 *     energy;
 *     storage;
 *     data movement.
 *
 * Tensor rank and dimensions remain semantic values.
 *
 * No tensor-rank or accelerator-capacity ceiling is encoded here.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Declaring a budget does not itself create an effect.
 *
 * Resource consumption may later interact with:
 *
 *     io;
 *     network;
 *     distributed;
 *     measurement;
 *     simulation;
 *     learning;
 *     adaptation;
 *     native;
 *     foreign;
 *
 * through the semantic/effect systems.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * A budget may carry policy metadata.
 *
 * Policy interpretation remains owned by the policy subsystem.
 *
 * This grammar does not authorize:
 *
 *     resource allocation;
 *     privileged execution;
 *     native execution;
 *     network access;
 *     FFI;
 *     reflection.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Budget syntax must remain source-traceable.
 *
 * Downstream provenance may record:
 *
 *     source budget;
 *     normalized budget;
 *     resource-resolution decision;
 *     target feasibility;
 *     optimization decision;
 *     execution realization.
 *
 * The grammar itself creates no provenance records.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no actions;
 *     no runtime calls;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no randomness.
 *
 * Parsing depends only on the token stream and grammar.
 *
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
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * No EOF is consumed here.
 *
 * EOF belongs to the canonical Zamani parser composition root.
 *
 * ============================================================================
 */

resourceBudgets
    : resourceBudgetItem*
    ;


/*
 * ============================================================================
 * RESOURCE BUDGET ITEM
 * ============================================================================
 *
 * This rule intentionally remains small.
 *
 * Future budget forms should extend resourceBudgetDeclaration rather than
 * creating another budget entry point.
 *
 * ============================================================================
 */

resourceBudgetItem
    : resourceBudgetDeclaration
    ;


/*
 * ============================================================================
 * RESOURCE BUDGET DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     budget memory = required_memory;
 *
 *     budget qubits = logical_qubits;
 *
 *     budget latency <= latency_budget;
 *
 *     budget execution {
 *         limit = execution_budget;
 *         reserve = reserved_capacity;
 *         consume = expected_consumption;
 *     };
 *
 * The budget name is open-world.
 *
 * ============================================================================
 */

resourceBudgetDeclaration
    : BUDGET
      qualifiedName
      resourceBudgetSpecification
    ;


/*
 * ============================================================================
 * RESOURCE BUDGET SPECIFICATION
 * ============================================================================
 *
 * A budget has one of three source forms:
 *
 *     assignment
 *     explicit relation
 *     structured clause block
 *
 * ============================================================================
 */

resourceBudgetSpecification
    : resourceBudgetAssignment
    | resourceBudgetRelation
    | resourceBudgetBlock
    ;


/*
 * ============================================================================
 * SIMPLE ASSIGNMENT
 * ============================================================================
 *
 * Examples:
 *
 *     budget memory = required_memory;
 *
 *     budget quantum::time = execution_budget;
 *
 *     budget network::bandwidth = bandwidth_budget;
 *
 * ============================================================================
 */

resourceBudgetAssignment
    : ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * DIRECT RELATIONAL BUDGET
 * ============================================================================
 *
 * Examples:
 *
 *     budget memory >= required_memory;
 *
 *     budget latency <= latency_budget;
 *
 *     budget throughput >= required_throughput;
 *
 * The comparison has semantic meaning only after type and resource analysis.
 *
 * ============================================================================
 */

resourceBudgetRelation
    : resourceBudgetRelationOperator
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * STRUCTURED BUDGET
 * ============================================================================
 *
 * Example:
 *
 *     budget execution {
 *         limit = execution_limit;
 *         minimum = required_capacity;
 *         target = desired_capacity;
 *         reserve = reserved_capacity;
 *         consume = expected_consumption;
 *         remaining = remaining_budget;
 *         scope = execution;
 *         condition = workload_size > threshold;
 *         priority = execution_priority;
 *         policy = adaptive;
 *     };
 *
 * ============================================================================
 */

resourceBudgetBlock
    : LBRACE
      resourceBudgetClause*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * RESOURCE BUDGET CLAUSE
 * ============================================================================
 *
 * The common property form keeps the grammar open to future resource-budget
 * dimensions without requiring a universal keyword for every possible
 * resource.
 *
 * Canonical semantic clauses have dedicated parser rules.
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
    | resourceBudgetNamedClause
    ;


/*
 * ============================================================================
 * LIMIT
 * ============================================================================
 *
 * Example:
 *
 *     limit = execution_limit;
 *
 * This is a budget-local semantic limit.
 *
 * It is NOT a universal machine maximum.
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
 * MINIMUM
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
 * MAXIMUM
 * ============================================================================
 *
 * This is a value for this particular program budget.
 *
 * It MUST NOT be interpreted as a language-level hardware ceiling.
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
 * TARGET
 * ============================================================================
 *
 * Target is an objective value or semantic reference.
 *
 * It does not select a physical target.
 *
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
 * RESERVE
 * ============================================================================
 *
 * Reservation intent only.
 *
 * Actual reservation is downstream.
 *
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
 * CONSUME
 * ============================================================================
 *
 * Declares expected or modelled consumption.
 *
 * It does not perform resource allocation.
 *
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
 * REMAINING
 * ============================================================================
 *
 * Represents a semantic remaining allowance.
 *
 * Runtime computation is downstream.
 *
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
 * SCOPE
 * ============================================================================
 *
 * Scope is an expression rather than a finite enum.
 *
 * Examples:
 *
 *     scope = program;
 *     scope = function;
 *     scope = execution;
 *     scope = deployment;
 *     scope = quantum_kernel;
 *     scope = future::execution_domain;
 *
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
 * CONDITION
 * ============================================================================
 *
 * Example:
 *
 *     condition = workload_size > threshold;
 *
 * The condition is parsed through the canonical expression system.
 *
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
 * PRIORITY
 * ============================================================================
 *
 * Priority remains an expression.
 *
 * The grammar deliberately does NOT impose:
 *
 *     0..10
 *     0..100
 *     integer-only
 *
 * semantics.
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
 * POLICY
 * ============================================================================
 *
 * Policy values remain open-world expressions.
 *
 * Examples:
 *
 *     policy = strict;
 *     policy = adaptive;
 *     policy = best_effort;
 *     policy = execution::adaptive;
 *
 * Policy semantics belong downstream.
 *
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
 * OPEN-WORLD NAMED CLAUSE
 * ============================================================================
 *
 * This is the primary extensibility mechanism.
 *
 * Examples:
 *
 *     thermal_margin = thermal_budget;
 *
 *     accounting::cost = cost_budget;
 *
 *     quantum::sampling = sampling_budget;
 *
 *     accelerator::occupancy = occupancy_budget;
 *
 *     network::egress = egress_budget;
 *
 *     future::resource_metric = symbolic_budget;
 *
 * A future resource dimension does not require a new grammar keyword.
 *
 * ============================================================================
 */

resourceBudgetNamedClause
    : qualifiedName
      ASSIGN
      resourceBudgetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RESOURCE BUDGET EXPRESSION
 * ============================================================================
 *
 * ResourceExpressions is the sole authority for resource expressions.
 *
 * This wrapper exists to provide a stable API for this grammar.
 *
 * It MUST NOT grow its own arithmetic or logical expression implementation.
 *
 * ============================================================================
 */

resourceBudgetExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * RESOURCE BUDGET RELATION OPERATOR
 * ============================================================================
 *
 * Operators are supplied by ZamaniLexer.
 *
 * This rule does not assign semantic meaning.
 *
 * ============================================================================
 */

resourceBudgetRelationOperator
    : EQ_EQ
    | NOT_EQ
    | LE
    | GE
    | LESS
    | GREATER
    ;


/*
 * ============================================================================
 * INTEGRATION EXAMPLES
 * ============================================================================
 *
 * The following are representative valid forms.
 *
 * They are NOT hardware limits.
 *
 * --------------------------------------------------------------------------
 *
 *     budget memory = required_memory;
 *
 * --------------------------------------------------------------------------
 *
 *     budget quantum::logical_qubits = logical_qubits;
 *
 * --------------------------------------------------------------------------
 *
 *     budget latency <= latency_budget;
 *
 * --------------------------------------------------------------------------
 *
 *     budget execution {
 *         limit = execution_limit;
 *         minimum = minimum_capacity;
 *         target = desired_capacity;
 *         reserve = reserved_capacity;
 *         consume = expected_consumption;
 *         remaining = remaining_budget;
 *         scope = execution;
 *         condition = workload_size > threshold;
 *         priority = execution_priority;
 *         policy = adaptive;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     budget network::bandwidth {
 *         minimum = required_bandwidth;
 *         target = preferred_bandwidth;
 *         limit = communication_budget;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     budget quantum::error {
 *         maximum = error_budget;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     budget accelerator::memory {
 *         minimum = required_memory;
 *         maximum = memory_budget;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     budget future::resource::metric = symbolic_value;
 *
 * --------------------------------------------------------------------------
 *
 * No physical hardware is selected by any of these forms.
 *
 *
 * ============================================================================
 * REQUIREMENTS INTEGRATION
 * ============================================================================
 *
 * These constructs remain distinct:
 *
 *     requires memory >= required_memory;
 *
 *     budget memory = memory_budget;
 *
 * The first expresses a requirement.
 *
 * The second expresses a budget.
 *
 * Semantic analysis may compare them, but the parser preserves the distinction.
 *
 *
 * ============================================================================
 * CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * These remain distinct:
 *
 *     constraint latency <= latency_budget;
 *
 *     budget latency = latency_budget;
 *
 * A downstream semantic phase may derive or enforce relationships between
 * them according to the enclosing policy.
 *
 *
 * ============================================================================
 * PREFERENCE INTEGRATION
 * ============================================================================
 *
 * A budget does not automatically become:
 *
 *     prefer
 *
 * or:
 *
 *     require
 *
 * The semantic policy determines its strength.
 *
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability expressions remain capability-system concepts.
 *
 * A budget may reference capability-derived values through:
 *
 *     resourceExpression
 *
 * without defining capability syntax here.
 *
 * Example semantic relationship:
 *
 *     capability::tensor_compute
 *             |
 *             v
 *     resource realization
 *             |
 *             v
 *     budget evaluation
 *
 * This grammar does not duplicate capabilityReference.
 *
 *
 * ============================================================================
 * NEGOTIATION INTEGRATION
 * ============================================================================
 *
 * Budgets become inputs to resource negotiation.
 *
 * Conceptual flow:
 *
 *     budget
 *       |
 *       v
 *     requirements
 *       |
 *       v
 *     constraints
 *       |
 *       v
 *     capabilities
 *       |
 *       v
 *     preferences
 *       |
 *       v
 *     hints
 *       |
 *       v
 *     negotiation
 *       |
 *       v
 *     execution realization
 *
 * Negotiation belongs to:
 *
 *     grammar/resources/negotiation.g4
 *
 * and the corresponding semantic/compiler subsystems.
 *
 *
 * ============================================================================
 * SCALABILITY INTEGRATION
 * ============================================================================
 *
 * `scalability.g4` remains the owner of scalability intent.
 *
 * This file only represents budget values that may participate in scalability
 * analysis.
 *
 * It must not introduce:
 *
 *     maximum scale;
 *     maximum resource count;
 *     maximum topology size;
 *     maximum tensor rank;
 *     maximum quantum size.
 *
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * This grammar never emits quantum::ir directly.
 *
 * Budget information may accompany the semantic quantum program until:
 *
 *     semantic quantum model
 *         ->
 *     quantum::ir
 *
 * Quantum optimization, decomposition, routing, scheduling, resilience,
 * QEC, ZQN, and HAL remain downstream.
 *
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Budget syntax remains independent of HDL implementation.
 *
 * HDL semantic analysis may consume budget information for:
 *
 *     timing;
 *     power;
 *     energy;
 *     storage;
 *     bandwidth;
 *     throughput;
 *     thermal behavior;
 *     reliability.
 *
 * The grammar does not choose physical implementation.
 *
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backend realization may use budget information when selecting or evaluating
 * an execution plan.
 *
 * This grammar does not:
 *
 *     inspect hardware;
 *     choose devices;
 *     allocate devices;
 *     assign physical qubits;
 *     assign CPU cores;
 *     assign GPU devices;
 *     assign FPGA regions;
 *     select network nodes.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must identify:
 *
 *     - missing budget name;
 *     - missing assignment value;
 *     - missing relation value;
 *     - malformed budget block;
 *     - malformed budget clause;
 *     - missing semicolon where required;
 *     - invalid token sequence.
 *
 * Semantic diagnostics, not parser diagnostics, report:
 *
 *     - unknown resource;
 *     - invalid unit;
 *     - incompatible dimensions;
 *     - unsatisfiable budget;
 *     - contradictory budgets;
 *     - impossible target realization;
 *     - policy conflict;
 *     - capability conflict.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical forms preserved by this grammar include:
 *
 *     budget <name> = <expression>;
 *
 *     budget <name> {
 *         ...
 *     };
 *
 * Existing source meaning MUST remain unchanged when this grammar is
 * integrated into the resource composition root.
 *
 * Any incompatible surface change requires:
 *
 *     language-version documentation;
 *     compatibility documentation;
 *     migration guidance;
 *     parser regression tests.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser tests MUST include:
 *
 *     budget memory = required_memory;
 *     budget latency = latency_budget;
 *     budget quantum::logical_qubits = logical_qubits;
 *     budget network::bandwidth = bandwidth_budget;
 *     budget future::resource::metric = symbolic_budget;
 *
 *     budget execution {
 *         limit = execution_limit;
 *         minimum = required_capacity;
 *         target = desired_capacity;
 *         reserve = reserved_capacity;
 *         consume = expected_consumption;
 *         remaining = remaining_budget;
 *         scope = execution;
 *         condition = workload_size > threshold;
 *         priority = execution_priority;
 *         policy = adaptive;
 *     };
 *
 * Positive boundary tests MUST include:
 *
 *     deeply qualified resource names;
 *     symbolic values;
 *     expression-valued budgets;
 *     arbitrarily long clause lists;
 *     nested expression structures;
 *     quantum resource names;
 *     distributed resource names;
 *     accelerator resource names;
 *     future-domain resource names.
 *
 * Negative parser tests MUST include:
 *
 *     budget;
 *     budget = value;
 *     budget memory;
 *     budget memory =;
 *     budget memory {;
 *     budget memory { limit = };
 *     budget memory { unknown_clause };
 *
 * Semantic negative tests MUST include:
 *
 *     incompatible units;
 *     contradictory budget values;
 *     invalid resource dimensions;
 *     impossible constraints;
 *     unavailable capabilities.
 *
 *
 * ============================================================================
 * DETERMINISM TEST
 * ============================================================================
 *
 * Identical source text and identical lexer/parser configuration MUST produce
 * structurally equivalent parser output.
 *
 * This grammar contains no:
 *
 *     actions;
 *     semantic predicates;
 *     I/O;
 *     environment access;
 *     hardware inspection;
 *     random behavior.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO finite resource universe;
 *     NO physical device identifiers;
 *     NO hardware capacities;
 *     NO universal maxima;
 *     NO fixed tensor rank;
 *     NO fixed register width;
 *     NO fixed node count;
 *     NO fixed thread count;
 *     NO fixed memory size;
 *     NO fixed quantum size;
 *     NO target-specific allocation;
 *     NO backend-specific syntax.
 *
 * Resource names and values remain expressions.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. It compiles with the canonical ZamaniLexer.
 *
 *     2. It imports only the canonical resource-expression and name grammars.
 *
 *     3. `resources.g4` consumes `resourceBudgetDeclaration`.
 *
 *     4. No second budget entry point exists elsewhere.
 *
 *     5. Budget syntax does not duplicate requirements, constraints,
 *        preferences, hints, capabilities, or policies.
 *
 *     6. Open-world resource names work without grammar modification.
 *
 *     7. Symbolic resource expressions work without grammar modification.
 *
 *     8. No machine-size constant is encoded.
 *
 *     9. Quantum budgets remain target-independent and cross the
 *        `quantum::ir` boundary only downstream.
 *
 *    10. Classical, HDL, accelerator, distributed, networking, and future
 *        resource domains can consume the same grammar.
 *
 *    11. Positive, negative, boundary, scalability, compatibility, and
 *        determinism tests pass.
 *
 *    12. Generated Rust integration remains compatible with Rust 1.97 /
 *        Rust 1.97.1 and uses no unsafe Zamani implementation code.
 *
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */