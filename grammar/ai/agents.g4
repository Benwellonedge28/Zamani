parser grammar Agents;

options {
    tokenVocab=ZamaniLexer;
}

import Types, Expressions;

/*
 * ============================================================================
 * Zamani AI Agent Grammar
 * ============================================================================
 *
 * File:
 *     grammar/ai/agents.g4
 *
 * Grammar:
 *     Agents
 *
 * Purpose:
 *     Defines syntax for portable AI/agentic computation.
 *
 * Architectural boundary:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     Agents parser
 *       |
 *       v
 *     AST / semantic analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +----> inference
 *       +----> models
 *       +----> training
 *       +----> data
 *       +----> networking
 *       +----> distributed execution
 *       +----> security
 *       +----> resource management
 *       +----> classical execution
 *       +----> quantum execution
 *       +----> hardware/runtime
 *
 * This grammar MUST NOT:
 *
 *   - implement an agent runtime
 *   - define model internals
 *   - define tensor internals
 *   - define dataset internals
 *   - define training algorithms
 *   - define inference algorithms
 *   - define network protocols
 *   - define distributed topology
 *   - define hardware topology
 *   - define scheduling
 *   - define resource capacities
 *   - define quantum operations
 *   - define a second IR
 *   - contain target-specific Rust actions
 *   - contain unsafe code
 *   - impose machine-size limits
 *
 * Scalability principle:
 *
 *     The grammar describes agent intent and structure.
 *
 *     Physical realization is selected later from:
 *
 *       capabilities
 *       requirements
 *       constraints
 *       preferences
 *       resources
 *       targets
 *       runtime state
 *       execution policy
 *
 * Agent-specific annotation names are intentionally NOT encoded as lexer
 * keywords. The semantic layer owns annotation validation and versioning.
 *
 * Examples of semantic roles that may be represented by annotations include:
 *
 *     @agent
 *     @model
 *     @goal
 *     @objective
 *     @policy
 *     @tool
 *     @memory
 *     @observation
 *     @action
 *     @plan
 *     @step
 *     @delegate
 *     @coordinate
 *     @message
 *     @event
 *     @checkpoint
 *     @termination
 *     @requires
 *     @constraint
 *     @preference
 *     @capability
 *
 * The list above is documentation, NOT a hard-coded grammar restriction.
 *
 * ============================================================================
 */


/* ============================================================================
 * Public entry point
 * ============================================================================
 *
 * AI's top-level grammar should expose this rule rather than importing
 * implementation-specific agent rules directly.
 */
agentConstruct
    : agentDeclaration
    | agentBinding
    | agentClause
    ;


/* ============================================================================
 * Agent declaration
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     @agent name {
 *         ...
 *     }
 *
 * Optional type and initializer forms are supported because the semantic
 * layer may represent different kinds of agent declarations.
 */
agentDeclaration
    : NANO_ANNOTATION
      identifier
      agentTypeClause?
      agentInitializer?
      agentBody
    ;


/* ============================================================================
 * Agent binding
 * ============================================================================
 *
 * Allows references such as:
 *
 *     @agent worker = existing_agent;
 *
 * or:
 *
 *     @model model = trained_model;
 *
 * The annotation determines semantic meaning.
 */
agentBinding
    : NANO_ANNOTATION
      identifier
      agentTypeClause?
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * Agent type
 * ============================================================================
 */
agentTypeClause
    : COLON typeExpression
    ;


/* ============================================================================
 * Agent initializer
 * ============================================================================
 */
agentInitializer
    : ASSIGN expression
    ;


/* ============================================================================
 * Agent body
 * ============================================================================
 *
 * An agent can contain an arbitrary number of declarations, clauses, and
 * ordinary Zamani statements.
 *
 * No fixed maximum is imposed.
 */
agentBody
    : LBRACE
      agentMember*
      RBRACE
    ;


/* ============================================================================
 * Agent members
 * ============================================================================
 */
agentMember
    : agentNestedDeclaration
    | agentBinding
    | agentClause
    | statement
    ;


/* ============================================================================
 * Nested declaration
 * ============================================================================
 *
 * Supports structures such as:
 *
 *     @agent parent {
 *         @agent child {
 *             ...
 *         }
 *     }
 *
 * The semantic layer determines whether nesting is valid for the particular
 * annotation and execution model.
 */
agentNestedDeclaration
    : NANO_ANNOTATION
      identifier
      agentTypeClause?
      agentInitializer?
      agentBody
    ;


/* ============================================================================
 * Generic agent clause
 * ============================================================================
 *
 * This is the principal extensibility mechanism.
 *
 * Examples:
 *
 *     @goal solve_problem;
 *
 *     @model reasoning_model;
 *
 *     @tool search_tool;
 *
 *     @memory memory_store;
 *
 *     @objective score = objective_function;
 *
 *     @policy policy_name {
 *         ...
 *     }
 *
 *     @plan execution_plan {
 *         ...
 *     }
 *
 *     @requires requirement_expression;
 *
 *     @constraint constraint_expression;
 *
 *     @preference preference_expression;
 *
 *     @capability capability_expression;
 *
 * The semantic layer validates which annotation forms are legal in a given
 * context.
 */
agentClause
    : NANO_ANNOTATION
      agentClauseTarget?
      agentTypeClause?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Optional named target
 * ============================================================================
 *
 * Examples:
 *
 *     @goal solve
 *     @tool search
 *     @memory memory
 *     @plan plan
 */
agentClauseTarget
    : identifier
    ;


/* ============================================================================
 * Agent expression
 * ============================================================================
 *
 * Provides an explicit public rule for agent-related expression contexts.
 *
 * This does NOT introduce a second expression language.
 */
agentExpression
    : expression
    ;


/* ============================================================================
 * Agent reference
 * ============================================================================
 *
 * References are represented through the canonical identifier rule.
 */
agentReference
    : identifier
    ;


/* ============================================================================
 * Goal / objective expressions
 * ============================================================================
 *
 * These rules provide stable semantic boundaries for AST construction while
 * deliberately delegating expression syntax to the shared expression grammar.
 */
agentGoal
    : NANO_ANNOTATION
      agentClauseTarget?
      ASSIGN
      expression
      SEMICOLON
    ;


agentObjective
    : NANO_ANNOTATION
      agentClauseTarget?
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * Policy
 * ============================================================================
 *
 * Policies describe decision constraints/intents.
 *
 * Actual policy evaluation belongs to semantic/runtime policy infrastructure.
 */
agentPolicy
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      agentBody
    ;


/* ============================================================================
 * Tool declaration/reference
 * ============================================================================
 *
 * A tool is represented semantically. This grammar does not prescribe:
 *
 *     - protocol
 *     - transport
 *     - implementation language
 *     - hardware
 *     - endpoint
 *     - device
 *     - authentication mechanism
 *
 * Those belong to the relevant interoperability/network/security layers.
 */
agentTool
    : NANO_ANNOTATION
      agentClauseTarget?
      agentTypeClause?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Memory declaration/reference
 * ============================================================================
 *
 * This is a semantic reference to memory.
 *
 * It does not define:
 *
 *     RAM size
 *     cache size
 *     storage capacity
 *     persistence mechanism
 *     memory topology
 *
 * Those properties are resolved elsewhere.
 */
agentMemory
    : NANO_ANNOTATION
      agentClauseTarget?
      agentTypeClause?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Observation
 * ============================================================================
 *
 * Observation syntax is intentionally expression-based.
 *
 * An observation can therefore originate from:
 *
 *     classical state
 *     data
 *     sensors
 *     network input
 *     quantum measurement
 *     accelerator output
 *     distributed state
 *     external services
 *
 * without the agent grammar needing to know how that information is produced.
 */
agentObservation
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Action
 * ============================================================================
 *
 * An action represents an intended computation/effect.
 *
 * The actual effect system validates whether the action is permitted.
 */
agentAction
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Plan
 * ============================================================================
 *
 * Plans may contain arbitrarily many steps and nested constructs.
 *
 * There is deliberately no fixed maximum number of:
 *
 *     plans
 *     steps
 *     branches
 *     actions
 *     goals
 *     agents
 */
agentPlan
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      agentBody
    ;


/* ============================================================================
 * Plan step
 * ============================================================================
 *
 * A step is structurally generic.
 *
 * Ordering, dependencies, scheduling, retries, resource allocation and
 * execution semantics are determined downstream.
 */
agentStep
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Delegation
 * ============================================================================
 *
 * Delegation expresses intent to transfer work.
 *
 * It does not select a machine, node, process, network address, or provider.
 */
agentDelegation
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Coordination
 * ============================================================================
 *
 * Coordination may later lower into:
 *
 *     local concurrency
 *     distributed execution
 *     actor systems
 *     message passing
 *     collective execution
 *     workflow execution
 *
 * The grammar remains independent of the implementation.
 */
agentCoordination
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      agentBody
    ;


/* ============================================================================
 * Message reference
 * ============================================================================
 *
 * Message semantics belong to networking/distributed layers.
 */
agentMessage
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Event / lifecycle hook
 * ============================================================================
 *
 * Supports extensible lifecycle semantics without introducing runtime logic
 * into the grammar.
 */
agentEvent
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Checkpoint
 * ============================================================================
 *
 * A checkpoint is a semantic execution concept.
 *
 * IMPORTANT:
 *
 * This grammar does NOT imply that arbitrary quantum state can be serialized.
 *
 * The semantic/runtime layer determines whether the checkpoint represents:
 *
 *     - classical state
 *     - compiled program state
 *     - logical state
 *     - measurement boundary
 *     - QEC-supported state
 *     - reconstructible state
 *     - provider-supported state
 *
 * Unsupported checkpoint forms must be rejected semantically.
 */
agentCheckpoint
    : NANO_ANNOTATION
      agentClauseTarget?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Termination condition
 * ============================================================================
 *
 * Conditions remain ordinary Zamani expressions.
 *
 * No fixed iteration count or resource limit is encoded here.
 */
agentTermination
    : NANO_ANNOTATION
      agentClauseTarget?
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * Capability / requirement / constraint / preference
 * ============================================================================
 *
 * These are syntactically generic because ownership belongs to the universal
 * resource/capability model.
 *
 * They must remain semantically distinct:
 *
 *     requirement != capability
 *     constraint != preference
 *     preference != target
 *
 * Example semantic intent:
 *
 *     @requires quantum;
 *
 * must NOT implicitly mean:
 *
 *     use a particular QPU
 *     use a particular qubit count
 *     use a particular topology
 */
agentRequirement
    : NANO_ANNOTATION
      agentClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


agentConstraint
    : NANO_ANNOTATION
      agentClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


agentPreference
    : NANO_ANNOTATION
      agentClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


agentCapability
    : NANO_ANNOTATION
      agentClauseTarget?
      ASSIGN?
      expression
      SEMICOLON
    ;


/* ============================================================================
 * Agent configuration
 * ============================================================================
 *
 * Generic configuration remains expression-based so that new capabilities can
 * be added without changing the grammar's machine assumptions.
 */
agentConfiguration
    : NANO_ANNOTATION
      agentClauseTarget?
      agentTypeClause?
      agentInitializer?
      (
          agentBody
        | SEMICOLON
      )
    ;


/* ============================================================================
 * Agent member sequence
 * ============================================================================
 *
 * Public helper rule for AST/test infrastructure.
 */
agentMembers
    : agentMember*
    ;


/* ============================================================================
 * Agent body contents
 * ============================================================================
 *
 * Explicit alias useful to AST visitors and grammar tests.
 */
agentContents
    : agentBody
    ;