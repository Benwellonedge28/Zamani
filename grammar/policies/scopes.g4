/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/scopes.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * GRAMMAR NAME
 * ------------
 * PolicyScopes
 *
 * STATUS
 * ------
 * CANONICAL POLICY-SCOPE GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 *
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the source-level syntax for POLICY SCOPE declarations and
 * scope clauses.
 *
 * A policy scope identifies the semantic region, applicability domain, or
 * evaluation boundary to which policy intent applies.
 *
 * A scope is NOT:
 *
 *     - a physical machine;
 *     - a processor;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a node;
 *     - a device;
 *     - a thread;
 *     - a process;
 *     - a memory allocation;
 *     - a quantum register;
 *     - a hardware topology;
 *     - a runtime resource reservation.
 *
 * A scope is a SOURCE-LEVEL POLICY BOUNDARY.
 *
 *
 * ARCHITECTURAL POSITION
 * ----------------------
 *
 * The complete policy pipeline is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     PolicyScopes parser rules
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic policy scope
 *       |
 *       +--> policy applicability
 *       +--> policy inheritance
 *       +--> policy composition
 *       +--> policy precedence
 *       +--> policy activation
 *       +--> policy resolution
 *       |
 *       v
 *     semantic policy model
 *       |
 *       +--> resources
 *       +--> capabilities
 *       +--> effects
 *       +--> contracts
 *       +--> security
 *       +--> execution
 *       +--> adaptation
 *       +--> provenance
 *       |
 *       v
 *     target-independent planning
 *       |
 *       v
 *     lowering / scheduling / routing / realization
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     policyScope
 *     policyScopeBody
 *     policyScopeMember
 *
 *     policyScopeReference
 *     policyScopeSelector
 *     policyScopeSelectorList
 *
 *     policyScopeCondition
 *     policyScopePriority
 *     policyScopeMode
 *     policyScopeInheritance
 *     policyScopeComposition
 *     policyScopeProperty
 *
 *     policyScopeTarget
 *     policyScopeBinding
 *
 *     policyScopeExpression
 *     policyScopeArgument
 *     policyScopeArgumentList
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexer rules;
 *     - keyword spelling;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - arithmetic;
 *     - logical precedence;
 *     - types;
 *     - resource expressions;
 *     - capability expressions;
 *     - contracts;
 *     - effects;
 *     - security authorization;
 *     - identities;
 *     - credentials;
 *     - roles;
 *     - permissions;
 *     - prohibitions;
 *     - resource discovery;
 *     - hardware discovery;
 *     - device selection;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - quantum mapping;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime enforcement;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - backend implementation.
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Policy declaration syntax remains owned by:
 *
 *     grammar/policies/policy.g4
 *
 * Policy scope syntax is owned here.
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/requirements.g4
 *
 * Resource constraints remain owned by:
 *
 *     grammar/resources/constraints.g4
 *
 * Capabilities remain owned by:
 *
 *     grammar/resources/capabilities.g4
 *
 * General expressions remain owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Names remain owned by:
 *
 *     grammar/core/names.g4
 *
 * This file MUST NOT recreate any of those grammars.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT LEXER DEPENDENCY
 * -----------------------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required token:
 *
 *     SCOPE
 *
 * The canonical spelling is:
 *
 *     scope
 *
 * `SCOPE` must be introduced exactly once in:
 *
 *     grammar/lexer/keywords.g4
 *
 * and composed through the canonical lexer hierarchy.
 *
 * No lexer rule is defined in this file.
 *
 *
 * PARSER DEPENDENCIES
 * -------------------
 *
 *     Names
 *     Expressions
 *
 * These provide:
 *
 *     qualifiedName
 *     identifier
 *     expression
 *
 *
 * IMPORT CONTRACT
 * ---------------
 *
 * ANTLR grammar-name imports are used.
 *
 * The build system must expose the grammar directories through ANTLR's
 * grammar-library path.
 *
 * This file does NOT use filesystem-style imports.
 *
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * Public parser rules exported by this grammar:
 *
 *     policyScope
 *     policyScopeReference
 *     policyScopeSelector
 *     policyScopeCondition
 *     policyScopeBinding
 *     policyScopeExpression
 *
 *
 * CONSUMERS
 * ---------
 *
 * Primary consumer:
 *
 *     grammar/policies/policy.g4
 *
 * Secondary semantic consumers:
 *
 *     policy semantic analysis
 *     policy resolution
 *     resource analysis
 *     capability analysis
 *     execution planning
 *     security analysis
 *     provenance
 *     compatibility
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser creates ANTLR contexts only.
 *
 * The frontend AST must preserve:
 *
 *     - scope source span;
 *     - scope identity;
 *     - scope selectors;
 *     - scope conditions;
 *     - scope bindings;
 *     - scope mode;
 *     - scope priority;
 *     - inheritance intent;
 *     - composition intent;
 *     - declaration order;
 *     - source provenance.
 *
 * The AST MUST NOT resolve:
 *
 *     - physical devices;
 *     - hardware;
 *     - resources;
 *     - capabilities;
 *     - security identities;
 *     - runtime objects.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A parsed scope becomes a semantic PolicyScope.
 *
 * Conceptually:
 *
 *     PolicyScope {
 *         identity
 *         selectors
 *         conditions
 *         bindings
 *         mode
 *         inheritance
 *         composition
 *         priority
 *         properties
 *         source
 *     }
 *
 * The semantic layer determines:
 *
 *     - what the scope denotes;
 *     - whether the scope exists;
 *     - whether it is visible;
 *     - whether it is applicable;
 *     - whether scopes overlap;
 *     - whether inheritance is legal;
 *     - whether composition is compatible;
 *     - whether precedence is deterministic;
 *     - whether policy conflicts exist.
 *
 * None of these decisions are made by this grammar.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Policy scopes MUST remain target-independent.
 *
 * The same source-level scope may apply to:
 *
 *     embedded execution
 *     CPU execution
 *     multicore execution
 *     GPU execution
 *     FPGA execution
 *     ASIC execution
 *     accelerator execution
 *     quantum execution
 *     simulation
 *     HPC
 *     cluster execution
 *     distributed execution
 *     cloud execution
 *     future execution models
 *
 * Scope syntax MUST NOT contain finite universal capacity limits.
 *
 * In particular, this file MUST NOT encode:
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
 * A scope can refer to symbolic requirements or semantic properties.
 *
 * Physical feasibility is determined downstream.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is deliberately no finite grammar-defined limit on:
 *
 *     scope depth;
 *     selector count;
 *     condition count;
 *     binding count;
 *     property count;
 *     inheritance references;
 *     composition references;
 *     argument count;
 *     scope nesting depth.
 *
 * Repetition is therefore represented by ANTLR `*` or `+`.
 *
 * Practical limits are implementation/resource limits, not language limits.
 *
 * The grammar itself imposes no artificial computational ceiling.
 *
 *
 * ============================================================================
 * 1. POLICY SCOPE
 * ============================================================================
 *
 * Canonical form:
 *
 *     scope name {
 *         ...
 *     }
 *
 * Example:
 *
 *     scope execution {
 *         mode adaptive;
 *     }
 *
 * The identity is a qualified source-level name.
 *
 * The name does not identify a physical target.
 */
policyScope
    : SCOPE
      qualifiedName
      policyScopeBody
    ;


/*
 * ============================================================================
 * 2. POLICY SCOPE BODY
 * ============================================================================
 *
 * A scope may contain zero or more members.
 *
 * There is no fixed number of members.
 */
policyScopeBody
    : LBRACE
      policyScopeMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 3. POLICY SCOPE MEMBER
 * ============================================================================
 *
 * Scope members describe applicability and composition only.
 */
policyScopeMember
    : policyScopeReference
    | policyScopeSelector
    | policyScopeCondition
    | policyScopePriority
    | policyScopeMode
    | policyScopeInheritance
    | policyScopeComposition
    | policyScopeTarget
    | policyScopeBinding
    | policyScopeProperty
    ;


/*
 * ============================================================================
 * 4. SCOPE REFERENCE
 * ============================================================================
 *
 * References another logical policy scope.
 *
 * Example:
 *
 *     use base::execution;
 *
 * The referenced scope remains unresolved until semantic analysis.
 *
 * This grammar does not require the referenced scope to exist.
 */
policyScopeReference
    : USE
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. SCOPE SELECTOR
 * ============================================================================
 *
 * A selector identifies a semantic applicability dimension.
 *
 * Example:
 *
 *     select domain::quantum;
 *
 *     select execution::distributed;
 *
 *     select capability::tensor;
 *
 * The grammar intentionally accepts an open-world qualified name.
 *
 * New domains do not require grammar changes.
 */
policyScopeSelector
    : SELECT
      policyScopeSelectorList
      SEMICOLON
    ;


policyScopeSelectorList
    : policyScopeExpression
      (COMMA policyScopeExpression)*
    ;


/*
 * ============================================================================
 * 6. SCOPE CONDITION
 * ============================================================================
 *
 * A condition determines whether the scope is semantically applicable.
 *
 * Example:
 *
 *     when execution::deterministic;
 *
 * The expression remains owned by the canonical expression grammar.
 */
policyScopeCondition
    : WHEN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. SCOPE PRIORITY
 * ============================================================================
 *
 * Priority is semantic policy metadata.
 *
 * It does NOT establish an implementation-defined integer range.
 *
 * The value is an ordinary expression.
 *
 * Example:
 *
 *     priority 10;
 *
 * or:
 *
 *     priority policy::priority;
 */
policyScopePriority
    : PRIORITY
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. SCOPE MODE
 * ============================================================================
 *
 * Mode is intentionally open-ended.
 *
 * The language does not enumerate every possible execution or policy mode.
 *
 * Examples may include:
 *
 *     mode strict;
 *     mode advisory;
 *     mode adaptive;
 *     mode deterministic;
 *
 * Future modes can be represented without changing this grammar.
 */
policyScopeMode
    : MODE
      policyScopeExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. INHERITANCE
 * ============================================================================
 *
 * A scope may inherit from another logical scope.
 *
 * Example:
 *
 *     inherit execution::base;
 *
 * Inheritance semantics are determined by the policy semantic model.
 *
 * This grammar does not define:
 *
 *     - override precedence;
 *     - conflict resolution;
 *     - authorization;
 *     - physical inheritance.
 */
policyScopeInheritance
    : INHERIT
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. COMPOSITION
 * ============================================================================
 *
 * A scope may compose with other logical scopes.
 *
 * Example:
 *
 *     compose execution::portable, execution::deterministic;
 *
 * Composition is semantic.
 *
 * It is not textual macro expansion.
 */
policyScopeComposition
    : COMPOSE
      policyScopeSelectorList
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. TARGET-CLASSIFICATION BOUNDARY
 * ============================================================================
 *
 * A target reference expresses a logical target category or semantic target
 * identity.
 *
 * It MUST NOT be interpreted as physical device selection.
 *
 * Example:
 *
 *     target quantum::simulator;
 *
 *     target accelerator::compute;
 *
 * The target meaning is resolved downstream.
 */
policyScopeTarget
    : TARGET
      policyScopeExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. SCOPE BINDING
 * ============================================================================
 *
 * Bind a symbolic policy-scope property to an expression.
 *
 * Example:
 *
 *     bind determinism = true;
 *
 *     bind execution::strategy = strategy;
 *
 * The left-hand side is a qualified name.
 */
policyScopeBinding
    : BIND
      qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. SCOPE PROPERTY
 * ============================================================================
 *
 * Generic extensibility mechanism.
 *
 * Example:
 *
 *     property determinism = true;
 *
 *     property deployment::region = region;
 *
 * Property names remain open-world qualified names.
 *
 * This prevents the scope grammar from becoming a catalogue of every future
 * policy concern.
 */
policyScopeProperty
    : PROPERTY
      qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. POLICY SCOPE EXPRESSION
 * ============================================================================
 *
 * Scope-specific values are ordinary Zamani expressions.
 *
 * This rule exists as a stable integration boundary.
 *
 * It does not create another expression language.
 */
policyScopeExpression
    : expression
    ;


/*
 * ============================================================================
 * 15. ARGUMENT BOUNDARY
 * ============================================================================
 *
 * This rule is intentionally available as a stable extension point for
 * semantic scope functions and selectors.
 *
 * It does not impose a fixed arity.
 */
policyScopeArgument
    : expression
    ;


policyScopeArgumentList
    : policyScopeArgument
      (COMMA policyScopeArgument)*
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/policies/policy.g4
 * --------------------------
 *
 * MUST consume:
 *
 *     policyScope
 *
 * when a policy body contains a scope declaration.
 *
 * The policy grammar should therefore eventually contain:
 *
 *     policyMember
 *         : ...
 *         | policyScope
 *         ;
 *
 * Policy.g4 remains the owner of policy declarations.
 *
 * PolicyScopes.g4 remains the owner of scope declarations.
 *
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Scope syntax does not duplicate:
 *
 *     requirementExpression
 *     constraintExpression
 *     capabilityExpression
 *
 * If a scope needs resource intent, the semantic policy/resource model
 * consumes the corresponding resource constructs through the existing
 * resource grammars.
 *
 * A scope therefore does not acquire its own resource language.
 *
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Scope selectors may identify capability namespaces symbolically:
 *
 *     select capability::quantum;
 *     select capability::tensor;
 *
 * This does not perform capability resolution.
 *
 * Capability resolution remains owned by:
 *
 *     grammar/resources/capabilities.g4
 *
 * and downstream semantic analysis.
 *
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Scope syntax does not define effects.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * A policy scope may semantically constrain effects, but the effect vocabulary
 * and effect semantics are not duplicated here.
 *
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Scope syntax does not grant permissions.
 *
 * Security authorization remains owned by:
 *
 *     grammar/security/
 *
 * A scope may identify the semantic applicability of security policy, but
 * authorization is resolved downstream.
 *
 *
 * ============================================================================
 * EXECUTION INTEGRATION
 * ============================================================================
 *
 * Scope metadata may influence:
 *
 *     resource selection
 *     capability negotiation
 *     scheduling
 *     fallback
 *     retry
 *     recovery
 *     simulation
 *     adaptation
 *
 * It MUST NOT directly execute or select physical hardware.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum scopes remain target-neutral.
 *
 * Valid semantic selectors may include names such as:
 *
 *     quantum
 *     quantum::simulation
 *     quantum::measurement
 *     quantum::dynamic_control
 *
 * The grammar does not enumerate:
 *
 *     QPU vendors
 *     gate sets
 *     coupling maps
 *     qubit counts
 *     calibration parameters
 *     physical qubit identifiers.
 *
 * Quantum lowering remains:
 *
 *     semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
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
 *       ->
 *     target.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Scope selectors may identify semantic hardware domains:
 *
 *     hardware::accelerator
 *     hardware::programmable_logic
 *     hdl::simulation
 *     hdl::synthesis
 *
 * They do not encode physical widths, device counts, topology limits,
 * clock frequencies, memory capacities, or vendor-specific architecture.
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Scope selectors may identify distributed semantic domains:
 *
 *     distributed
 *     distributed::cluster
 *     distributed::service
 *     networking::transport
 *
 * The grammar imposes no finite node or topology limits.
 *
 *
 * ============================================================================
 * AI / KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Scope syntax remains domain-neutral.
 *
 * A semantic policy scope may apply to:
 *
 *     reasoning
 *     learning
 *     adaptation
 *     uncertainty
 *     provenance
 *     explanation
 *     agents
 *
 * No AI-specific scope syntax is required.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every PolicyScope AST node must preserve source provenance.
 *
 * Downstream semantic processing may record:
 *
 *     declaration source
 *     inherited scope
 *     composed scope
 *     activation condition
 *     resolution decision
 *     conflict resolution
 *     resulting policy.
 *
 * This grammar does not generate provenance itself.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Scope declarations are versioned through the language/specification system.
 *
 * Semantic changes to:
 *
 *     inheritance
 *     composition
 *     priority
 *     activation
 *     applicability
 *
 * MUST NOT be silently introduced by changing parser behavior.
 *
 * Breaking changes require:
 *
 *     specification update
 *     compatibility metadata
 *     conformance tests
 *     migration documentation.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must identify:
 *
 *     - missing scope name;
 *     - missing scope body;
 *     - malformed selector;
 *     - malformed condition;
 *     - malformed priority;
 *     - malformed inheritance reference;
 *     - malformed binding;
 *     - malformed property;
 *     - missing semicolon;
 *     - malformed qualified name.
 *
 * Semantic diagnostics are separate and include:
 *
 *     - unknown scope;
 *     - cyclic inheritance;
 *     - incompatible composition;
 *     - conflicting scope properties;
 *     - ambiguous applicability;
 *     - invalid scope reference;
 *     - unauthorized semantic use.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser tests should cover:
 *
 *     scope execution {
 *     }
 *
 *     scope execution {
 *         select execution::adaptive;
 *     }
 *
 *     scope quantum::simulation {
 *         select quantum::simulation;
 *         mode deterministic;
 *     }
 *
 *     scope distributed::execution {
 *         inherit execution::base;
 *         compose execution::portable, execution::deterministic;
 *     }
 *
 *     scope hardware::intent {
 *         bind capability::compute = capability;
 *         property deployment::strategy = strategy;
 *     }
 *
 * Negative tests should cover:
 *
 *     missing scope name
 *     missing braces
 *     empty selector
 *     malformed qualified name
 *     missing semicolon
 *     missing binding assignment
 *     malformed property assignment
 *     malformed inheritance reference
 *
 * Scalability tests should verify:
 *
 *     arbitrarily many scope members;
 *     arbitrarily many selectors;
 *     arbitrarily deep qualified names;
 *     arbitrarily many inherited scopes;
 *     arbitrarily many composed scopes;
 *     arbitrarily large expressions;
 *     arbitrarily large policy source units subject only to implementation
 *     resources.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO hardware limits
 *     NO quantum limits
 *     NO CPU limits
 *     NO GPU limits
 *     NO FPGA limits
 *     NO ASIC limits
 *     NO node limits
 *     NO memory limits
 *     NO thread limits
 *     NO tensor-rank limits
 *     NO register-width limits
 *     NO network-size limits
 *     NO device-count limits
 *     NO vendor catalogue
 *     NO gate catalogue
 *     NO fixed target catalogue
 *
 * All extensible semantic identities use qualified names or expressions.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     NO embedded Rust;
 *     NO semantic predicates;
 *     NO parser actions;
 *     NO native calls;
 *     NO filesystem access;
 *     NO network access;
 *     NO hardware access;
 *     NO unsafe Rust.
 *
 * Rust 1.97+ compatibility is therefore determined by the generated parser
 * integration and repository build configuration, not by this grammar
 * containing Rust implementation code.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. It compiles as ANTLR4 parser grammar.
 *
 *     2. It uses only the canonical ZamaniLexer vocabulary.
 *
 *     3. It imports only canonical Names and Expressions grammar boundaries.
 *
 *     4. It does not duplicate policy, resource, capability, effect, security,
 *        or expression syntax.
 *
 *     5. Policy.g4 consumes policyScope.
 *
 *     6. The canonical lexer exports SCOPE, PRIORITY, MODE, INHERIT, COMPOSE,
 *        BIND and the other required tokens exactly once.
 *
 *     7. AST ownership is defined outside this grammar.
 *
 *     8. Semantic ownership is defined outside this grammar.
 *
 *     9. Scope resolution remains target-independent.
 *
 *    10. No finite machine-capacity assumptions exist.
 *
 *    11. Positive, negative, boundary, scalability, compatibility and
 *        cross-domain tests exist.
 *
 *    12. The same scope syntax remains valid independently of whether the
 *        eventual realization is classical, quantum, HDL, heterogeneous,
 *        distributed, simulated, embedded, or future hardware.
 *
 *
 * ============================================================================
 */

parser grammar PolicyScopes;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
;


/*
 * Public rule:
 *
 *     policyScope
 *
 * This is intentionally the only scope declaration root.
 */
policyScope
    : SCOPE
      qualifiedName
      policyScopeBody
    ;


policyScopeBody
    : LBRACE
      policyScopeMember*
      RBRACE
    ;


policyScopeMember
    : policyScopeReference
    | policyScopeSelector
    | policyScopeCondition
    | policyScopePriority
    | policyScopeMode
    | policyScopeInheritance
    | policyScopeComposition
    | policyScopeTarget
    | policyScopeBinding
    | policyScopeProperty
    ;


policyScopeReference
    : USE
      qualifiedName
      SEMICOLON
    ;


policyScopeSelector
    : SELECT
      policyScopeSelectorList
      SEMICOLON
    ;


policyScopeSelectorList
    : policyScopeExpression
      (COMMA policyScopeExpression)*
    ;


policyScopeCondition
    : WHEN
      expression
      SEMICOLON
    ;


policyScopePriority
    : PRIORITY
      expression
      SEMICOLON
    ;


policyScopeMode
    : MODE
      policyScopeExpression
      SEMICOLON
    ;


policyScopeInheritance
    : INHERIT
      qualifiedName
      SEMICOLON
    ;


policyScopeComposition
    : COMPOSE
      policyScopeSelectorList
      SEMICOLON
    ;


policyScopeTarget
    : TARGET
      policyScopeExpression
      SEMICOLON
    ;


policyScopeBinding
    : BIND
      qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


policyScopeProperty
    : PROPERTY
      qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


policyScopeExpression
    : expression
    ;


policyScopeArgument
    : expression
    ;


policyScopeArgumentList
    : policyScopeArgument
      (COMMA policyScopeArgument)*
    ;