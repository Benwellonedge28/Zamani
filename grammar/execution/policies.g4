/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/execution/policies.g4
 *
 * GRAMMAR
 * -------
 * ExecutionPolicies
 *
 * STATUS
 * ------
 * CANONICAL EXECUTION-LAYER POLICY COMPOSITION GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 * No unsafe implementation requirement
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the EXECUTION-LAYER POLICY COMPOSITION boundary.
 *
 * It does NOT define a second policy language.
 *
 * The universal policy authority is:
 *
 *     grammar/core/policies.g4
 *
 * Security-specific policy semantics remain owned by:
 *
 *     grammar/security/
 *
 * Resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * Capability semantics remain owned by:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/capabilities.g4
 *
 * Requirement semantics remain owned by:
 *
 *     grammar/core/requirements.g4
 *
 * Constraint semantics remain owned by:
 *
 *     grammar/core/constraints.g4
 *
 * Execution policies therefore form an adapter/composition layer:
 *
 *     universal policy
 *             |
 *             v
 *     execution policy binding
 *             |
 *             +------------------+
 *             |                  |
 *             v                  v
 *       execution context     execution construct
 *             |                  |
 *             +---------+--------+
 *                       |
 *                       v
 *                semantic policy model
 *                       |
 *                       +--> effects
 *                       +--> capabilities
 *                       +--> requirements
 *                       +--> constraints
 *                       +--> resources
 *                       +--> contracts
 *                       +--> provenance
 *                       +--> security
 *                       +--> resilience
 *                       +--> deployment
 *                       +--> adaptation
 *                       +--> simulation
 *                       |
 *                       v
 *                canonical semantic model
 *                       |
 *             +---------+---------+
 *             |         |         |
 *             v         v         v
 *        classical   quantum::ir  HDL/
 *          IR                  hardware
 *             |         |         |
 *             +---------+---------+
 *                       |
 *                       v
 *                 optimization
 *                       |
 *                lowering/planning
 *                       |
 *             +---------+---------+
 *             |         |         |
 *             v         v         v
 *         placement  routing  scheduling
 *             |         |         |
 *             +---------+---------+
 *                       |
 *                   resilience
 *                       |
 *                       v
 *                      ZQN
 *                       |
 *                       v
 *                      HAL
 *                       |
 *                       v
 *               target realization
 *
 * This file therefore describes execution policy INTENT only.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * grammar/core/policies.g4
 * ------------------------
 *
 * OWNS THE UNIVERSAL POLICY LANGUAGE:
 *
 *     policyDeclaration
 *     policyRule
 *     policyRequirement
 *     policyProhibition
 *     policyPermission
 *     policyPreference
 *     policyObligation
 *     policyFallback
 *     policyComposition
 *     policyBinding
 *     policyTarget
 *     policySubject
 *     policyResource
 *     policyAction
 *     policyCondition
 *     policyDecision
 *     policyOutcome
 *
 * This file MUST NOT redefine those universal constructs.
 *
 *
 * grammar/execution/policies.g4
 * -----------------------------
 *
 * OWNS ONLY:
 *
 *     execution-policy attachment;
 *     execution-policy references;
 *     execution-policy scopes;
 *     execution-policy application;
 *     execution-policy selection;
 *     execution-policy composition;
 *     execution-policy precedence metadata;
 *     execution-policy activation conditions;
 *     execution-policy fallback references;
 *     execution-policy execution-context association;
 *     execution-policy dispatch association;
 *     execution-policy deployment association;
 *     execution-policy adaptation association;
 *     execution-policy resilience association;
 *     execution-policy simulation association;
 *     execution-policy runtime association;
 *     execution-policy metadata.
 *
 * It MUST NOT redefine universal policy members.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Execution is sufficiently broad to require a dedicated composition layer,
 * but execution must not become another policy authority.
 *
 * The distinction is:
 *
 *     core/policies.g4
 *         =
 *     WHAT a policy means structurally
 *
 *     execution/policies.g4
 *         =
 *     WHERE/HOW execution semantics consume an already-defined policy
 *
 * This prevents:
 *
 *     policy syntax duplication
 *     security-policy duplication
 *     resource-policy duplication
 *     deployment-policy duplication
 *     dispatch-policy duplication
 *     resilience-policy duplication
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Execution policies MUST remain portable.
 *
 * They may express:
 *
 *     execution intent
 *     selection intent
 *     preference
 *     constraint
 *     requirement
 *     fallback
 *     resilience intent
 *     reproducibility intent
 *     determinism intent
 *     simulation intent
 *     adaptation intent
 *     deployment intent
 *     dispatch intent
 *     observability intent
 *
 * They MUST NOT require a particular physical realization.
 *
 * A policy may therefore remain unchanged while realization occurs on:
 *
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud system
 *     heterogeneous system
 *     future computational substrate
 *
 * provided semantic requirements and constraints are satisfied.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO language-level finite limit on:
 *
 *     policy references
 *     policy attachments
 *     policy scopes
 *     policy alternatives
 *     policy metadata
 *     policy conditions
 *     policy composition depth
 *     execution contexts
 *     execution constructs
 *     targets
 *     resources
 *     capabilities
 *     nodes
 *     devices
 *     processors
 *     accelerators
 *     qubits
 *     memory
 *     threads
 *     tasks
 *     distributed locations
 *
 * Repetition is represented with ANTLR repetition operators.
 *
 * This file MUST NOT introduce any equivalent indirect limit.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT contain or imply:
 *
 *     MAX_POLICIES
 *     MAX_POLICY_ATTACHMENTS
 *     MAX_EXECUTION_POLICIES
 *     MAX_TARGETS
 *     MAX_RESOURCES
 *     MAX_CAPABILITIES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * No physical resource identity is encoded.
 *
 * This file must never contain constructs such as:
 *
 *     cpu(0)
 *     gpu(0)
 *     qpu(0)
 *     node(0)
 *     device(0)
 *     physical_qubit(0)
 *
 * as universal execution-policy semantics.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Execution policy names and policy properties are open-ended.
 *
 * The grammar must not enumerate every possible future:
 *
 *     execution strategy
 *     processor type
 *     accelerator
 *     scheduler
 *     routing algorithm
 *     resilience algorithm
 *     deployment platform
 *     simulator
 *     quantum architecture
 *     AI runtime
 *     distributed system
 *     network
 *
 * Future semantic capabilities must be expressible through:
 *
 *     qualified names
 *     expressions
 *     policy references
 *     policy metadata
 *     policy composition
 *
 * without changing this grammar merely because a new target or strategy
 * appears.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file is a PARSER grammar.
 *
 * It MUST NOT define lexer rules.
 *
 * It consumes the canonical Zamani lexical vocabulary through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Existing canonical tokens used here include:
 *
 *     POLICY
 *     PREFER
 *     REQUIRE
 *     REQUIREMENT
 *     CONSTRAINT
 *     CAPABILITY
 *     TARGET
 *     RESOURCE
 *     FALLBACK
 *     DETERMINISTIC
 *     REPRODUCIBLE
 *     SIMULATE
 *     ADAPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     SELECT
 *     NEGOTIATE
 *     SANDBOX
 *     ALLOW
 *     FORBID
 *     DENY
 *     ASSIGN
 *     COLON
 *     COMMA
 *     SEMICOLON
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *
 * Existing repository architecture also permits open execution vocabulary
 * through qualified names and expressions.
 *
 * This is important because not every execution concept should become a
 * reserved keyword.
 *
 * ============================================================================
 * NO LOCAL KEYWORD AUTHORITY
 * ============================================================================
 *
 * Do NOT add parser string literals such as:
 *
 *     'execution'
 *     'policy'
 *     'apply'
 *
 * merely to manufacture lexical behavior.
 *
 * Existing canonical keyword tokens are consumed where available.
 *
 * Future lexical additions belong exclusively under:
 *
 *     grammar/lexer/
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT LEXICAL DEPENDENCY
 * -------------------------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 *
 * CORE POLICY DEPENDENCY
 * ---------------------
 *
 * This grammar intentionally does NOT import the complete universal policy
 * grammar as a replacement for the execution adapter.
 *
 * The universal policy grammar is:
 *
 *     grammar/core/policies.g4
 *
 * Its semantic model is consumed downstream.
 *
 * Keeping this file independent prevents execution/policies.g4 from becoming
 * a second owner of policy declarations.
 *
 *
 * EXECUTION DEPENDENCIES
 * ----------------------
 *
 * This file is designed to integrate with:
 *
 *     grammar/execution/execution.g4
 *     grammar/execution/execution-context.g4
 *     grammar/execution/dispatch.g4
 *     grammar/execution/deployment.g4
 *     grammar/execution/adaptive.g4
 *     grammar/execution/recovery.g4
 *     grammar/execution/resilience.g4
 *     grammar/execution/runtime-capabilities.g4
 *     grammar/execution/simulation.g4
 *     grammar/execution/environments.g4
 *
 * It does not import those grammars merely to reuse semantic concepts.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Canonical direction:
 *
 *     lexer
 *       |
 *       v
 *     core policy model
 *       |
 *       v
 *     execution policy adapter
 *       |
 *       +--> execution context
 *       +--> dispatch
 *       +--> deployment
 *       +--> adaptation
 *       +--> resilience
 *       +--> simulation
 *       +--> runtime
 *       |
 *       v
 *     semantic policy model
 *
 * Execution subgrammars may consume the exported rules from this file.
 *
 * This file MUST NOT import execution grammars that themselves consume this
 * file, preventing circular ANTLR imports.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser contexts from this file must map to domain-neutral AST structures
 * equivalent to:
 *
 *     ExecutionPolicyReference
 *     ExecutionPolicyBinding
 *     ExecutionPolicyApplication
 *     ExecutionPolicyScope
 *     ExecutionPolicySelection
 *     ExecutionPolicyComposition
 *     ExecutionPolicyCondition
 *     ExecutionPolicyFallback
 *     ExecutionPolicyMetadata
 *
 * The AST must preserve:
 *
 *     policy reference
 *     binding subject
 *     scope
 *     application mode
 *     conditions
 *     alternatives
 *     ordering
 *     metadata
 *     source span
 *     source order
 *
 * The AST MUST NOT contain:
 *
 *     physical device identifiers
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     QPU identifiers
 *     physical qubit mappings
 *     routing decisions
 *     scheduler state
 *     calibration data
 *     vendor SDK objects
 *     HAL handles
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis resolves:
 *
 *     policy identity
 *     policy scope
 *     policy applicability
 *     policy precedence
 *     policy composition
 *     policy conflicts
 *     requirement satisfaction
 *     capability compatibility
 *     resource feasibility
 *     effect compatibility
 *     contract compatibility
 *     security interaction
 *     resilience interaction
 *     adaptation authorization
 *     deployment interaction
 *     simulation interaction
 *     provenance
 *     determinism
 *     reproducibility
 *
 * This grammar performs none of those operations.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar can reference resource intent but does not define resource
 * quantities or allocation.
 *
 * Example:
 *
 *     policy execution::portable {
 *         ...
 *     }
 *
 * may semantically reference:
 *
 *     resource::memory
 *     resource::compute
 *     resource::bandwidth
 *     resource::latency
 *
 * The resource subsystem determines their meaning and feasibility.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Policies may refer to capabilities.
 *
 * Examples:
 *
 *     capability::quantum::measurement
 *     capability::tensor::compute
 *     capability::distributed::collectives
 *
 * This grammar does not determine whether a capability exists.
 *
 * Capability resolution remains downstream.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Execution policies may govern behavior involving:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * The policy grammar does not redefine effects.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Universal contracts remain owned by:
 *
 *     grammar/validation/
 *
 * Execution policies may refer to contract-related expressions but MUST NOT
 * redefine:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * This grammar does not implement authorization.
 *
 * A policy such as:
 *
 *     allow ...
 *
 * may be consumed by the security subsystem when the semantic context makes
 * it an authorization policy.
 *
 * Security-specific identity, credentials, trust, authorization and
 * enforcement remain outside this file.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Execution policy may govern quantum execution intent:
 *
 *     policy reference
 *     capability requirements
 *     resource requirements
 *     resilience preferences
 *     simulation fallback
 *     reproducibility requirements
 *     execution adaptation
 *
 * It must NOT define:
 *
 *     quantum gates
 *     physical qubits
 *     coupling maps
 *     routing
 *     pulse schedules
 *     calibration
 *     QEC algorithms
 *     ZQN
 *
 * If policy semantics affect quantum computation, the semantic pipeline is:
 *
 *     source
 *       |
 *       v
 *     policy AST
 *       |
 *       v
 *     semantic policy model
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition/routing/scheduling
 *       |
 *       v
 *     resilience/QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * This file never bypasses that boundary.
 *
 * ============================================================================
 * HDL/HARDWARE INTEGRATION
 * ============================================================================
 *
 * Execution policy may govern:
 *
 *     synthesis intent
 *     simulation intent
 *     execution capability
 *     portability
 *     resource preferences
 *     reliability
 *     deployment
 *
 * It does not define:
 *
 *     wire widths
 *     register counts
 *     physical cells
 *     clock topology
 *     FPGA resources
 *     ASIC placement
 *
 * Those remain owned by HDL/hardware semantics.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Execution policies may govern:
 *
 *     placement intent
 *     distribution
 *     replication
 *     recovery
 *     retry
 *     fallback
 *     consistency preferences
 *     observability
 *
 * They do not define:
 *
 *     node count
 *     physical node identity
 *     network topology
 *     scheduler implementation
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Adaptation remains controlled.
 *
 * An execution policy may govern adaptation, but policy attachment does not
 * authorize unrestricted self-modification.
 *
 * Semantic analysis must validate:
 *
 *     capability
 *     effect
 *     policy
 *     contract
 *     provenance
 *     resource requirements
 *
 * before adaptation is permitted.
 *
 * ============================================================================
 * DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * Execution policies may express:
 *
 *     deterministic
 *     reproducible
 *
 * intent using canonical tokens.
 *
 * These are policy constraints/preferences.
 *
 * They do not force a particular implementation.
 *
 * Semantic analysis determines whether the requested guarantees are
 * satisfiable on the selected realization.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Simulation policy remains execution intent.
 *
 * The policy may select or prefer simulation through a policy expression.
 *
 * The grammar does not enumerate:
 *
 *     simulator types
 *     simulator vendors
 *     simulation algorithms
 *     qubit limits
 *     model sizes
 *
 * ============================================================================
 * FALLBACK INTEGRATION
 * ============================================================================
 *
 * Fallback policy expresses alternative semantic paths.
 *
 * It does not itself execute fallback.
 *
 * Example conceptual form:
 *
 *     fallback policy::simulation;
 *
 * or:
 *
 *     fallback execution::portable;
 *
 * The semantic layer determines:
 *
 *     whether the fallback is compatible;
 *     when it applies;
 *     whether it preserves program meaning;
 *     whether it satisfies contracts;
 *     whether required capabilities remain satisfied.
 *
 * ============================================================================
 * POLICY REFERENCE
 * ============================================================================
 *
 * A policy reference is intentionally a qualified name.
 *
 * Examples:
 *
 *     execution::portable
 *     execution::deterministic
 *     execution::reproducible
 *     quantum::fault_tolerant
 *     distributed::resilient
 *     hardware::portable
 *     simulation::fallback
 *
 * The grammar does not reserve these names.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar ExecutionPolicies;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC EXECUTION POLICY ENTRY
 * ============================================================================
 *
 * This is the principal rule consumed by execution-domain grammars.
 *
 * It represents an execution policy attachment/reference.
 *
 * It deliberately does NOT define `policyDeclaration`.
 *
 * Universal policy declarations remain owned by grammar/core/policies.g4.
 * ============================================================================
 */

executionPolicy
    : executionPolicyReference
    | executionPolicyBinding
    | executionPolicyApplication
    ;


/*
 * ============================================================================
 * 2. POLICY REFERENCE
 * ============================================================================
 *
 * Canonical reference form:
 *
 *     execution::portable
 *
 *     quantum::fault_tolerant
 *
 *     distributed::resilient
 *
 *     future::execution::policy
 *
 * Qualified-name depth is intentionally unrestricted by the grammar.
 * ============================================================================
 */

executionPolicyReference
    : POLICY qualifiedName
    ;


/*
 * ============================================================================
 * 3. POLICY BINDING
 * ============================================================================
 *
 * Binds an existing policy to an abstract execution subject/context.
 *
 * Examples:
 *
 *     policy execution::portable on computation;
 *
 *     policy execution::deterministic for task;
 *
 *     policy distributed::resilient on service;
 *
 * `on` and `for` are already part of the canonical lexical vocabulary.
 * ============================================================================
 */

executionPolicyBinding
    : POLICY qualifiedName executionPolicyBindingClause+ SEMICOLON
    ;


executionPolicyBindingClause
    : executionPolicyOnClause
    | executionPolicyForClause
    | executionPolicyWithinClause
    | executionPolicyWhenClause
    | executionPolicyUnlessClause
    | executionPolicyTargetClause
    | executionPolicySubjectClause
    | executionPolicyScopeClause
    ;


executionPolicyOnClause
    : ON expression
    ;


executionPolicyForClause
    : FOR expression
    ;


executionPolicyWithinClause
    : WITHIN expression
    ;


executionPolicyWhenClause
    : WHEN expression
    ;


executionPolicyUnlessClause
    : UNLESS expression
    ;


executionPolicyTargetClause
    : TARGET expression
    ;


executionPolicySubjectClause
    : SUBJECT expression
    ;


executionPolicyScopeClause
    : SCOPE expression
    ;


/*
 * ============================================================================
 * 4. POLICY APPLICATION
 * ============================================================================
 *
 * Applies an existing policy to an execution subject using an explicit
 * application block.
 *
 * This form is intentionally generic:
 *
 *     policy execution::portable {
 *         ...
 *     }
 *
 * The body contains EXECUTION POLICY APPLICATION METADATA, not a second
 * universal policy declaration language.
 *
 * Universal policy members must remain owned by core/policies.g4.
 * ============================================================================
 */

executionPolicyApplication
    : POLICY qualifiedName
      LBRACE
      executionPolicyApplicationMember*
      RBRACE
    ;


executionPolicyApplicationMember
    : executionPolicyApplicationClause
    | executionPolicyBindingClause
    | executionPolicyOption
    | executionPolicyProperty
    | executionPolicyFallback
    | executionPolicySelection
    | executionPolicyRequirement
    | executionPolicyPreference
    | executionPolicyConstraint
    | executionPolicyCapability
    ;


 /*
  * ===========================================================================
  * 5. APPLICATION CLAUSE
  * ===========================================================================
  */

executionPolicyApplicationClause
    : executionPolicyApplicationKey
      COLON
      expression
      SEMICOLON
    ;


executionPolicyApplicationKey
    : qualifiedName
    ;


/*
 * ============================================================================
 * 6. EXECUTION POLICY OPTION
 * ============================================================================
 *
 * Options are open-world named execution-policy properties.
 *
 * They do not form a closed catalogue.
 * ============================================================================
 */

executionPolicyOption
    : executionPolicyOptionKey
      executionPolicyAssignmentOperator
      expression
      SEMICOLON
    ;


executionPolicyOptionKey
    : qualifiedName
    ;


executionPolicyAssignmentOperator
    : ASSIGN
    | COLON
    ;


/*
 * ============================================================================
 * 7. EXECUTION POLICY PROPERTY
 * ============================================================================
 *
 * Generic property syntax provides future extensibility without continually
 * expanding the core grammar.
 * ============================================================================
 */

executionPolicyProperty
    : executionPolicyPropertyKey
      COLON
      executionPolicyPropertyValue
      SEMICOLON
    ;


executionPolicyPropertyKey
    : qualifiedName
    ;


executionPolicyPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * 8. FALLBACK
 * ============================================================================
 *
 * FALLBACK is deliberately represented as an expression.
 *
 * The expression may identify:
 *
 *     another policy
 *     an execution strategy
 *     a simulation path
 *     a deployment path
 *     a recovery path
 *     a semantic alternative
 *
 * The semantic layer determines legality.
 * ============================================================================
 */

executionPolicyFallback
    : FALLBACK expression SEMICOLON
    ;


/*
 * ============================================================================
 * 9. POLICY SELECTION
 * ============================================================================
 *
 * Selection describes an execution-policy choice without selecting a physical
 * resource.
 *
 * Example:
 *
 *     select execution::deterministic;
 *
 * The semantic layer resolves compatibility and applicability.
 * ============================================================================
 */

executionPolicySelection
    : SELECT executionPolicySelectionValue SEMICOLON
    ;


executionPolicySelectionValue
    : expression
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT
 * ============================================================================
 *
 * This is an EXECUTION-POLICY reference to requirement intent.
 *
 * It does not redefine resource/core requirement syntax.
 *
 * The expression remains open-world.
 * ============================================================================
 */

executionPolicyRequirement
    : REQUIRE expression SEMICOLON
    ;


/*
 * ============================================================================
 * 11. PREFERENCE
 * ============================================================================
 *
 * Preference remains advisory unless semantic policy explicitly defines a
 * stronger interpretation.
 * ============================================================================
 */

executionPolicyPreference
    : PREFER expression SEMICOLON
    ;


/*
 * ============================================================================
 * 12. CONSTRAINT
 * ============================================================================
 *
 * Constraint remains semantic intent.
 *
 * This grammar does not evaluate it.
 * ============================================================================
 */

executionPolicyConstraint
    : CONSTRAINT expression SEMICOLON
    ;


/*
 * ============================================================================
 * 13. CAPABILITY
 * ============================================================================
 *
 * Capability references are expressions at this execution-policy boundary.
 *
 * Canonical capability resolution remains owned by the capability subsystem.
 * ============================================================================
 */

executionPolicyCapability
    : CAPABILITY expression SEMICOLON
    ;


/*
 * ============================================================================
 * 14. DETERMINISTIC POLICY
 * ============================================================================
 *
 * These rules provide convenient execution-policy forms while preserving the
 * distinction between:
 *
 *     policy intent
 *
 * and:
 *
 *     semantic enforcement.
 *
 * Example:
 *
 *     deterministic;
 *
 * Semantic analysis decides whether the requested guarantee can be satisfied.
 * ============================================================================
 */

executionDeterminismPolicy
    : DETERMINISTIC SEMICOLON
    ;


executionReproducibilityPolicy
    : REPRODUCIBLE SEMICOLON
    ;


/*
 * ============================================================================
 * 15. SIMULATION POLICY
 * ============================================================================
 *
 * SIMULATE is an execution intent.
 *
 * It does not identify a simulator implementation.
 * ============================================================================
 */

executionSimulationPolicy
    : SIMULATE expression SEMICOLON
    ;


/*
 * ============================================================================
 * 16. ADAPTATION POLICY
 * ============================================================================
 *
 * Adaptation is controlled by the semantic policy system.
 *
 * The grammar does not authorize arbitrary self-modification.
 * ============================================================================
 */

executionAdaptationPolicy
    : ADAPT expression SEMICOLON
    ;


/*
 * ============================================================================
 * 17. RESILIENCE POLICY
 * ============================================================================
 *
 * Execution policy may express resilience intent using the canonical
 * execution/resilience vocabulary.
 *
 * These are references/intent only.
 *
 * The actual resilience state machine remains owned by resilience/recovery
 * semantics.
 * ============================================================================
 */

executionResiliencePolicy
    : executionResilienceAction SEMICOLON
    ;


executionResilienceAction
    : RETRY expression
    | RECOVER expression
    | ESCALATE expression
    | REJECT expression
    ;


/*
 * ============================================================================
 * 18. NEGOTIATION
 * ============================================================================
 *
 * Negotiation expresses that execution realization may resolve compatible
 * policy/capability/resource alternatives downstream.
 *
 * It does not perform negotiation during parsing.
 * ============================================================================
 */

executionPolicyNegotiation
    : NEGOTIATE expression SEMICOLON
    ;


/*
 * ============================================================================
 * 19. SANDBOX
 * ============================================================================
 *
 * Sandbox intent is represented here only as execution-policy composition.
 *
 * Security semantics remain owned by grammar/security/.
 * ============================================================================
 */

executionSandboxPolicy
    : SANDBOX expression SEMICOLON
    ;


/*
 * ============================================================================
 * 20. POLICY COMPOSITION
 * ============================================================================
 *
 * Multiple policies may be composed without imposing a finite number.
 *
 * Example:
 *
 *     executionPolicies {
 *         policy execution::portable;
 *         policy execution::deterministic;
 *         policy distributed::resilient;
 *     }
 *
 * This rule is intentionally reusable by execution constructs.
 * ============================================================================
 */

executionPolicies
    : executionPolicyMember*
    ;


executionPolicyMember
    : executionPolicyReference SEMICOLON
    | executionPolicyBinding
    | executionPolicyApplication
    | executionDeterminismPolicy
    | executionReproducibilityPolicy
    | executionSimulationPolicy
    | executionAdaptationPolicy
    | executionResiliencePolicy
    | executionPolicyNegotiation
    | executionSandboxPolicy
    ;


/*
 * ============================================================================
 * 21. POLICY COMPOSITION BLOCK
 * ============================================================================
 *
 * This block does not declare a universal policy.
 *
 * It groups execution-policy applications for an enclosing execution construct.
 * ============================================================================
 */

executionPolicyBlock
    : LBRACE executionPolicyMember* RBRACE
    ;


/*
 * ============================================================================
 * 22. DISPATCH INTEGRATION ADAPTER
 * ============================================================================
 *
 * dispatch.g4 may consume this adapter.
 *
 * This does not import Dispatch here, preventing a circular dependency.
 * ============================================================================
 */

executionDispatchPolicy
    : executionPolicy
    ;


/*
 * ============================================================================
 * 23. DEPLOYMENT INTEGRATION ADAPTER
 * ============================================================================
 */

executionDeploymentPolicy
    : executionPolicy
    ;


/*
 * ============================================================================
 * 24. EXECUTION-CONTEXT INTEGRATION ADAPTER
 * ============================================================================
 */

executionContextPolicy
    : executionPolicy
    ;


/*
 * ============================================================================
 * 25. ADAPTIVE EXECUTION INTEGRATION ADAPTER
 * ============================================================================
 */

executionAdaptivePolicy
    : executionPolicy
    ;


/*
 * ============================================================================
 * 26. RESILIENCE INTEGRATION ADAPTER
 * ============================================================================
 */

executionRecoveryPolicy
    : executionPolicy
    ;


/*
 * ============================================================================
 * 27. SIMULATION INTEGRATION ADAPTER
 * ============================================================================
 */

executionSimulationPolicyBlock
    : executionPolicyBlock
    ;


/*
 * ============================================================================
 * 28. RUNTIME INTEGRATION ADAPTER
 * ============================================================================
 *
 * Runtime policy references remain semantic data.
 *
 * This grammar does not invoke runtime operations.
 * ============================================================================
 */

executionRuntimePolicy
    : executionPolicy
    ;


/*
 * ============================================================================
 * 29. POLICY METADATA
 * ============================================================================
 *
 * Metadata is open-ended.
 *
 * No fixed metadata catalogue is imposed.
 * ============================================================================
 */

executionPolicyMetadata
    : AT qualifiedName
      (ASSIGN expression)?
    ;


/*
 * ============================================================================
 * 30. POLICY REFERENCE LIST
 * ============================================================================
 *
 * Unbounded by language semantics.
 * ============================================================================
 */

executionPolicyReferenceList
    : executionPolicyReference
      (COMMA executionPolicyReference)*
    ;


/*
 * ============================================================================
 * 31. POLICY SELECTION LIST
 * ============================================================================
 */

executionPolicySelectionList
    : executionPolicySelectionValue
      (COMMA executionPolicySelectionValue)*
    ;


/*
 * ============================================================================
 * 32. POLICY ARGUMENTS
 * ============================================================================
 *
 * Policy extensions may carry arbitrary expressions.
 * ============================================================================
 */

executionPolicyArgumentList
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * 33. POLICY INVOCATION
 * ============================================================================
 *
 * A policy extension may be named and parameterized without introducing a
 * fixed execution-strategy catalogue.
 *
 * Example:
 *
 *     policy execution::strategy(argument_a, argument_b);
 *
 * ============================================================================
 */

executionPolicyInvocation
    : POLICY qualifiedName
      LPAREN
      executionPolicyArgumentList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 34. POLICY APPLICATION WITH ARGUMENTS
 * ============================================================================
 */

executionPolicyApplicationInvocation
    : POLICY qualifiedName
      LPAREN
      executionPolicyArgumentList?
      RPAREN
      executionPolicyApplicationSuffix?
      SEMICOLON
    ;


executionPolicyApplicationSuffix
    : ON expression
    | FOR expression
    | WITHIN expression
    ;


/*
 * ============================================================================
 * 35. PORTABILITY POLICY
 * ============================================================================
 *
 * PORTABILITY is represented by the canonical token but remains semantic
 * policy intent rather than a backend selection.
 * ============================================================================
 */

executionPortabilityPolicy
    : PORTABILITY expression SEMICOLON
    ;


/*
 * ============================================================================
 * 36. SCALABILITY POLICY
 * ============================================================================
 *
 * Scalability is semantic intent.
 *
 * It does not introduce a finite scaling limit.
 * ============================================================================
 */

executionScalabilityPolicy
    : SCALABILITY expression SEMICOLON
    ;


/*
 * ============================================================================
 * 37. PERFORMANCE POLICY
 * ============================================================================
 *
 * Performance properties remain expressions.
 * ============================================================================
 */

executionPerformancePolicy
    : PERFORMANCE expression SEMICOLON
    ;


/*
 * ============================================================================
 * 38. RELIABILITY POLICY
 * ============================================================================
 */

executionReliabilityPolicy
    : RELIABILITY expression SEMICOLON
    ;


/*
 * ============================================================================
 * 39. RESOURCE POLICY REFERENCE
 * ============================================================================
 *
 * This is deliberately a reference, not resource syntax.
 * ============================================================================
 */

executionResourcePolicy
    : RESOURCE expression SEMICOLON
    ;


/*
 * ============================================================================
 * 40. TARGET POLICY REFERENCE
 * ============================================================================
 *
 * The target remains abstract.
 * ============================================================================
 */

executionTargetPolicy
    : TARGET expression SEMICOLON
    ;


/*
 * ============================================================================
 * 41. POLICY EXPRESSION
 * ============================================================================
 *
 * This rule provides a stable execution-layer expression boundary.
 *
 * It delegates meaning to the canonical expression grammar.
 * ============================================================================
 */

executionPolicyExpression
    : expression
    ;


/*
 * ============================================================================
 * 42. CANONICAL EXECUTION INTEGRATION
 * ============================================================================
 *
 * Existing execution grammars can consume:
 *
 *     executionPolicy
 *
 * rather than reproducing policy syntax.
 *
 * Recommended integration:
 *
 *     execution.g4
 *         |
 *         +--> executionPolicy
 *
 *     dispatch.g4
 *         |
 *         +--> executionDispatchPolicy
 *
 *     deployment.g4
 *         |
 *         +--> executionDeploymentPolicy
 *
 *     execution-context.g4
 *         |
 *         +--> executionContextPolicy
 *
 *     adaptive.g4
 *         |
 *         +--> executionAdaptivePolicy
 *
 *     recovery.g4
 *         |
 *         +--> executionRecoveryPolicy
 *
 *     runtime.g4
 *         |
 *         +--> executionRuntimePolicy
 *
 * This keeps policy ownership singular.
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. AST / SEMANTIC / IR BOUNDARY
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend must transform these contexts into domain-neutral AST nodes.
 *
 * Recommended semantic structure:
 *
 *     ExecutionPolicyReference
 *     ExecutionPolicyBinding
 *     ExecutionPolicyApplication
 *     ExecutionPolicyCondition
 *     ExecutionPolicyOption
 *     ExecutionPolicyFallback
 *     ExecutionPolicySelection
 *
 * Then:
 *
 *     AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       +--> contract checking
 *       +--> policy checking
 *       +--> provenance
 *       |
 *       v
 *     semantic policy model
 *       |
 *       +--> classical computation
 *       +--> quantum computation
 *       +--> HDL/hardware
 *       +--> distributed computation
 *       +--> AI/ML
 *       +--> networking
 *       +--> simulation
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       |
 *       v
 *     optimization/lowering
 *       |
 *       v
 *     placement/routing/scheduling
 *       |
 *       v
 *     resilience
 *       |
 *       v
 *     ZQN/HAL
 *
 * Policy syntax never directly creates target instructions.
 *
 * ============================================================================
 * 44. SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The frontend must preserve:
 *
 *     policy reference order
 *     binding order
 *     clause order
 *     property order
 *     fallback order
 *     selection order
 *     source spans
 *     grouping
 *
 * Semantic normalization may canonicalize policy sets but must retain enough
 * provenance to explain the transformation.
 *
 * ============================================================================
 * 45. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic with respect to:
 *
 *     source
 *     grammar
 *     lexer
 *     parser configuration
 *
 * This grammar performs:
 *
 *     no hardware discovery
 *     no resource discovery
 *     no network access
 *     no filesystem access
 *     no environment inspection
 *     no runtime execution
 *     no randomness
 *
 * ============================================================================
 * 46. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar cannot grant permission merely by parsing:
 *
 *     allow
 *     permit
 *     sandbox
 *
 * Security authorization remains a semantic/runtime concern.
 *
 * Capability availability remains separate.
 *
 * Resource availability remains separate.
 *
 * Policy permission therefore does not imply:
 *
 *     capability availability
 *
 * and capability availability does not imply:
 *
 *     policy permission.
 *
 * ============================================================================
 * 47. ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify:
 *
 *     malformed policy reference
 *     malformed binding
 *     malformed application
 *     malformed policy option
 *     malformed fallback
 *     malformed selection
 *     malformed policy expression
 *
 * Semantic diagnostics must separately report:
 *
 *     unknown policy
 *     conflicting policies
 *     unsatisfied requirements
 *     unavailable capabilities
 *     incompatible constraints
 *     impossible guarantees
 *     invalid fallback
 *     unauthorized adaptation
 *     non-reproducible execution
 *     target infeasibility
 *
 * The parser must not pretend semantic failures are syntax failures.
 *
 * ============================================================================
 * 48. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar is compatible with the existing universal policy architecture
 * by deliberately not replacing grammar/core/policies.g4.
 *
 * Compatibility responsibility:
 *
 *     grammar/compatibility/
 *
 * governs migration of older execution-policy syntax.
 *
 * Deprecated syntax must not silently change semantic meaning.
 *
 * ============================================================================
 * 49. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must exercise progressively larger:
 *
 *     policy reference lists
 *     policy compositions
 *     policy bindings
 *     nested execution contexts
 *     metadata sets
 *     policy alternatives
 *     expressions
 *     qualified names
 *
 * The test suite must distinguish:
 *
 *     implementation/resource exhaustion
 *
 * from:
 *
 *     language-level rejection.
 *
 * No test may establish an artificial universal machine-size limit.
 *
 * ============================================================================
 * 50. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The execution policy grammar must be testable with:
 *
 *     classical execution
 *     quantum execution
 *     hybrid execution
 *     HDL/hardware execution
 *     AI/ML execution
 *     distributed execution
 *     networking
 *     simulation
 *     embedded execution
 *     accelerator execution
 *
 * The policy grammar itself must remain domain-neutral.
 *
 * ============================================================================
 * 51. QUANTUM SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test semantic policy references involving:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::fault_tolerant
 *     quantum::simulation
 *
 * as open policy/capability names.
 *
 * Do not encode:
 *
 *     a fixed qubit count
 *     a fixed QPU count
 *     a fixed topology
 *     a fixed gate set
 *
 * ============================================================================
 * 52. HARDWARE SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test policy references against abstract requirements such as:
 *
 *     capability::tensor::compute
 *     capability::accelerator::compute
 *     resource::memory
 *     resource::bandwidth
 *     resource::latency
 *
 * Do not encode:
 *
 *     RAM = fixed size
 *     VRAM = fixed size
 *     register = fixed width
 *     device = fixed count
 *
 * ============================================================================
 * 53. DISTRIBUTED TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     distributed::resilient
 *     distributed::portable
 *     distributed::reproducible
 *
 * without requiring a fixed number of nodes.
 *
 * ============================================================================
 * 54. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust code
 *     no actions
 *     no semantic predicates
 *     no unsafe code
 *     no raw pointers
 *     no filesystem access
 *     no network access
 *     no runtime calls
 *
 * Generated Rust integration must remain:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * ============================================================================
 * 55. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [ ] It exists at:
 *         grammar/execution/policies.g4
 *
 * [ ] It is a parser grammar.
 *
 * [ ] It uses:
 *         tokenVocab = ZamaniLexer;
 *
 * [ ] It does not define lexer rules.
 *
 * [ ] It does not redefine universal policy declarations.
 *
 * [ ] It does not duplicate security policy semantics.
 *
 * [ ] It does not duplicate resource semantics.
 *
 * [ ] It does not duplicate capability semantics.
 *
 * [ ] It does not duplicate requirement semantics.
 *
 * [ ] It does not duplicate constraint semantics.
 *
 * [ ] It does not duplicate contract semantics.
 *
 * [ ] It does not allocate resources.
 *
 * [ ] It does not select physical hardware.
 *
 * [ ] It does not select a vendor.
 *
 * [ ] It does not perform routing.
 *
 * [ ] It does not perform scheduling.
 *
 * [ ] It does not perform QEC.
 *
 * [ ] It does not implement ZQN.
 *
 * [ ] It does not implement HAL.
 *
 * [ ] It does not execute policies.
 *
 * [ ] It contains no fixed hardware limits.
 *
 * [ ] It supports open-ended qualified policy references.
 *
 * [ ] It supports policy binding.
 *
 * [ ] It supports policy application.
 *
 * [ ] It supports execution-policy composition.
 *
 * [ ] It supports fallback.
 *
 * [ ] It supports selection.
 *
 * [ ] It supports requirements.
 *
 * [ ] It supports preferences.
 *
 * [ ] It supports constraints.
 *
 * [ ] It supports capability references.
 *
 * [ ] It supports deterministic intent.
 *
 * [ ] It supports reproducibility intent.
 *
 * [ ] It supports simulation intent.
 *
 * [ ] It supports controlled adaptation intent.
 *
 * [ ] It supports resilience intent.
 *
 * [ ] It supports negotiation intent.
 *
 * [ ] It supports sandbox intent.
 *
 * [ ] It provides adapters for execution subgrammars.
 *
 * [ ] It preserves source ordering.
 *
 * [ ] It preserves source spans through the AST contract.
 *
 * [ ] It has positive tests.
 *
 * [ ] It has negative tests.
 *
 * [ ] It has boundary tests.
 *
 * [ ] It has scalability tests.
 *
 * [ ] It has deterministic parsing tests.
 *
 * [ ] It has cross-domain tests.
 *
 * [ ] It has compatibility tests.
 *
 * [ ] Generated Rust remains safe.
 *
 * ============================================================================
 * FINAL NORMATIVE RULE
 * ============================================================================
 *
 * This file is an EXECUTION POLICY ADAPTER.
 *
 * It does not create a second policy language.
 *
 * The authoritative separation is:
 *
 *     core/policies.g4
 *         |
 *         | universal policy structure
 *         v
 *     execution/policies.g4
 *         |
 *         | execution attachment/composition
 *         v
 *     execution semantics
 *         |
 *         +--> capabilities
 *         +--> resources
 *         +--> requirements
 *         +--> constraints
 *         +--> effects
 *         +--> contracts
 *         +--> security
 *         +--> resilience
 *         +--> deployment
 *         +--> adaptation
 *         +--> simulation
 *         |
 *         v
 *     canonical semantic model
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL/hardware representation
 *         |
 *         v
 *     target-independent lowering
 *         |
 *         v
 *     routing / scheduling / resilience
 *         |
 *         v
 *     ZQN / HAL
 *         |
 *         v
 *     target realization
 *
 * A policy expresses governing intent.
 *
 * Execution policy expresses how that governing intent applies to execution.
 *
 * Neither one specifies the physical machine.
 *
 * That separation is mandatory for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */