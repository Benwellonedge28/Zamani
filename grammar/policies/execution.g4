/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/execution.g4
 *
 * GRAMMAR
 * -------
 * PolicyExecution
 *
 * STATUS
 * ------
 * CANONICAL POLICY-DOMAIN EXECUTION PAYLOAD GRAMMAR
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
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the EXECUTION-SPECIFIC PAYLOAD of a universal Zamani
 * policy.
 *
 * It does NOT own:
 *
 *     policy declarations
 *     universal policy declarations
 *     universal policy identity
 *     general policy composition
 *     security authorization
 *     resource allocation
 *     capability resolution
 *     effect semantics
 *     scheduling
 *     routing
 *     physical placement
 *     quantum mapping
 *     hardware realization
 *     runtime implementation
 *
 * Its purpose is to provide a stable source-level boundary through which
 * a policy can express execution intent.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The complete relationship is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     grammar/core/policies.g4
 *       |
 *       v
 *     policy semantic model
 *       |
 *       v
 *     grammar/policies/execution.g4
 *       |
 *       v
 *     execution-policy semantic binding
 *       |
 *       +------------------+
 *       |                  |
 *       v                  v
 *     resources         capabilities
 *       |                  |
 *       +--------+---------+
 *                |
 *                v
 *             effects
 *                |
 *                v
 *             contracts
 *                |
 *                v
 *             provenance
 *                |
 *                v
 *        execution planning
 *                |
 *        +-------+--------+
 *        |       |        |
 *        v       v        v
 *     classical quantum  HDL/
 *       IR      ::ir   hardware
 *        |       |        |
 *        +-------+--------+
 *                |
 *                v
 *          optimization
 *                |
 *                v
 *          specialization
 *                |
 *                v
 *        routing/scheduling
 *                |
 *                v
 *       resilience/recovery
 *                |
 *                v
 *              ZQN
 *                |
 *                v
 *              HAL
 *                |
 *                v
 *        target realization
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * UNIVERSAL POLICY AUTHORITY
 * --------------------------
 *
 *     grammar/core/policies.g4
 *
 * owns the universal policy language.
 *
 *
 * POLICY EXECUTION PAYLOAD AUTHORITY
 * ----------------------------------
 *
 *     grammar/policies/execution.g4
 *
 * owns only the execution-specific payload represented by:
 *
 *     policyExecutionExpression
 *     policyExecutionSpecification
 *     policyExecutionClause
 *     policyExecutionAssignment
 *     policyExecutionCondition
 *     policyExecutionRequirement
 *     policyExecutionConstraint
 *     policyExecutionCapability
 *     policyExecutionPreference
 *     policyExecutionSelection
 *     policyExecutionFallback
 *     policyExecutionNegotiation
 *     policyExecutionSimulation
 *     policyExecutionAdaptation
 *     policyExecutionResilience
 *     policyExecutionSandbox
 *     policyExecutionDeterminism
 *     policyExecutionReproducibility
 *     policyExecutionProperty
 *
 *
 * EXECUTION SUBSYSTEM AUTHORITY
 * -----------------------------
 *
 *     grammar/execution/policies.g4
 *
 * owns execution-layer policy attachment, application, binding and execution
 * subsystem integration.
 *
 * This file MUST NOT duplicate those responsibilities.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     policyExecutionExpression
 *     policyExecutionSpecification
 *     policyExecutionClause
 *
 *     policyExecutionAssignment
 *     policyExecutionCondition
 *     policyExecutionRequirement
 *     policyExecutionConstraint
 *     policyExecutionCapability
 *     policyExecutionPreference
 *     policyExecutionSelection
 *     policyExecutionFallback
 *     policyExecutionNegotiation
 *
 *     policyExecutionSimulation
 *     policyExecutionAdaptation
 *     policyExecutionResilience
 *     policyExecutionSandbox
 *
 *     policyExecutionDeterminism
 *     policyExecutionReproducibility
 *
 *     policyExecutionProperty
 *     policyExecutionKey
 *     policyExecutionValue
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *     policyRule
 *     policyRuleCondition
 *     policyRuleAction
 *     policyPermission
 *     policyProhibition
 *     policyPreference
 *     policyFallback
 *     policyComposition
 *
 *     identifiers
 *     qualified names
 *     general expressions
 *     types
 *     capabilities
 *     resources
 *     requirements
 *     constraints
 *     contracts
 *     effects
 *     provenance
 *
 *     physical target selection
 *     hardware discovery
 *     device identity
 *     placement
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/requirements.g4
 *     grammar/core/constraints.g4
 *     grammar/expressions/expressions.g4
 *
 *
 * IMPORTS:
 *
 *     Names
 *     Requirements
 *     Constraints
 *     Expressions
 *
 *
 * LEXER AUTHORITY:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The lexer vocabulary is canonical.
 *
 * This file MUST NOT introduce lexer rules.
 *
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PRIMARY:
 *
 *     policyExecutionExpression
 *     policyExecutionSpecification
 *
 * SECONDARY:
 *
 *     policyExecutionClause
 *     policyExecutionAssignment
 *     policyExecutionCondition
 *     policyExecutionRequirement
 *     policyExecutionConstraint
 *     policyExecutionCapability
 *     policyExecutionPreference
 *     policyExecutionSelection
 *     policyExecutionFallback
 *     policyExecutionNegotiation
 *     policyExecutionSimulation
 *     policyExecutionAdaptation
 *     policyExecutionResilience
 *     policyExecutionSandbox
 *     policyExecutionDeterminism
 *     policyExecutionReproducibility
 *     policyExecutionProperty
 *
 *
 * ============================================================================
 * CONSUMERS
 * ============================================================================
 *
 * Primary:
 *
 *     grammar/core/policies.g4
 *
 * Secondary:
 *
 *     policy semantic analysis
 *     execution-policy semantic analysis
 *     execution planning
 *     resource analysis
 *     capability analysis
 *     effect analysis
 *     contract analysis
 *     provenance analysis
 *     security analysis
 *     quantum semantic analysis
 *     HDL/hardware semantic analysis
 *     distributed execution analysis
 *     simulation analysis
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates ANTLR parser contexts only.
 *
 * The Rust frontend AST MUST preserve:
 *
 *     source span
 *     source order
 *     execution policy clauses
 *     assignment keys
 *     expressions
 *     conditions
 *     requirements
 *     constraints
 *     capabilities
 *     preferences
 *     selections
 *     fallbacks
 *     negotiation intent
 *     simulation intent
 *     adaptation intent
 *     resilience intent
 *     sandbox intent
 *     determinism intent
 *     reproducibility intent
 *     arbitrary policy properties
 *
 * The AST MUST remain domain-neutral.
 *
 * It MUST NOT contain:
 *
 *     physical CPU identifiers
 *     physical GPU identifiers
 *     physical FPGA identifiers
 *     physical ASIC identifiers
 *     physical QPU identifiers
 *     physical qubit mappings
 *     vendor SDK objects
 *     scheduler handles
 *     routing tables
 *     calibration objects
 *     HAL handles
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     policy applicability
 *     policy precedence
 *     policy composition
 *     requirement satisfaction
 *     capability satisfaction
 *     resource feasibility
 *     effect compatibility
 *     contract compatibility
 *     security interaction
 *     adaptation authorization
 *     resilience behavior
 *     simulation feasibility
 *     determinism guarantees
 *     reproducibility guarantees
 *     provenance
 *     target feasibility
 *
 * Parsing does none of these.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Values are ordinary Zamani expressions.
 *
 * This grammar does not introduce:
 *
 *     execution-specific scalar types
 *     hardware-specific types
 *     scheduler-specific types
 *     quantum-specific physical types
 *
 * Type checking remains owned by the canonical type subsystem.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar declares policy intent only.
 *
 * It does not itself create effects.
 *
 * Semantic analysis may determine that an execution policy interacts with:
 *
 *     io
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
 *     code_generation
 *     simulation
 *
 * Effect ownership remains under:
 *
 *     grammar/effects/
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource semantics remain outside this file.
 *
 * Examples of semantic requirements that may be referenced by expressions
 * include:
 *
 *     memory >= required_memory
 *     qubits >= required_qubits
 *     capability("tensor.compute")
 *     topology(required_topology)
 *
 * This grammar does not define:
 *
 *     memory size
 *     qubit count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     node count
 *     thread count
 *     tensor rank
 *     register width
 *     network size
 *
 * Physical feasibility is resolved downstream.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability expressions are symbolic/open-world.
 *
 * Examples:
 *
 *     capability::quantum::measurement
 *     capability::tensor::compute
 *     capability::distributed::collectives
 *     future::capability::new_execution_model
 *
 * The grammar does not decide whether those capabilities exist.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Contracts remain owned by the validation/contract subsystem.
 *
 * Execution policy clauses may reference contract expressions but do not
 * redefine:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * This file is itself a policy payload grammar.
 *
 * It therefore MUST remain subordinate to:
 *
 *     grammar/core/policies.g4
 *
 * It MUST NOT create a second policy declaration mechanism.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Semantic processing SHOULD preserve provenance for:
 *
 *     policy declaration
 *     execution policy clause
 *     policy decision
 *     fallback selection
 *     adaptation
 *     target realization
 *     simulation choice
 *     resilience choice
 *
 * Provenance ownership remains outside this grammar.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file owns NO IR.
 *
 * Policy execution intent may influence:
 *
 *     canonical semantic model
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed execution plan
 *
 * It MUST NOT lower directly to:
 *
 *     machine instructions
 *     quantum gates
 *     physical signals
 *     routing commands
 *     scheduler commands
 *     pulse instructions
 *     QEC operations
 *     device operations
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * If execution policy affects quantum computation:
 *
 *     policy execution payload
 *          |
 *          v
 *     semantic policy model
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * This file MUST NOT introduce a competing quantum IR.
 *
 *
 * ============================================================================
 * HDL/HARDWARE BOUNDARY
 * ============================================================================
 *
 * Execution policy may influence:
 *
 *     synthesis intent
 *     simulation
 *     verification
 *     resource preference
 *     reliability
 *     portability
 *     deployment
 *
 * It does not define:
 *
 *     wire widths
 *     register counts
 *     physical cells
 *     FPGA resource counts
 *     ASIC placement
 *     clock topology
 *
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Execution policy may influence:
 *
 *     distribution
 *     placement intent
 *     replication intent
 *     recovery
 *     retry
 *     fallback
 *     consistency preference
 *     observability
 *
 * It does not define a fixed number of nodes or physical node identities.
 *
 *
 * ============================================================================
 * ADAPTATION SAFETY CONTRACT
 * ============================================================================
 *
 * `adapt` in this file represents POLICY INTENT.
 *
 * It does not authorize unrestricted self-modification.
 *
 * Before adaptation can occur, semantic/runtime layers must validate:
 *
 *     authorization
 *     capability
 *     effects
 *     resources
 *     contracts
 *     policy
 *     provenance
 *
 * This is particularly important for:
 *
 *     model adaptation
 *     execution-plan adaptation
 *     strategy adaptation
 *     resource adaptation
 *     quantum execution adaptation
 *     distributed recovery adaptation
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * `deterministic` is policy intent.
 *
 * It is not a parser-level proof.
 *
 * Semantic analysis must determine whether deterministic behavior is actually
 * achievable for the selected execution realization.
 *
 *
 * ============================================================================
 * REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * `reproducible` is also policy intent.
 *
 * Reproducibility must distinguish:
 *
 *     source reproducibility
 *     semantic reproducibility
 *     compilation reproducibility
 *     artifact reproducibility
 *     execution reproducibility
 *     result reproducibility
 *
 * The grammar only preserves the intent.
 *
 *
 * ============================================================================
 * SIMULATION CONTRACT
 * ============================================================================
 *
 * Simulation is an execution strategy.
 *
 * It may represent:
 *
 *     classical simulation
 *     quantum simulation
 *     HDL simulation
 *     hardware simulation
 *     distributed simulation
 *     AI/model simulation
 *     fault simulation
 *     performance simulation
 *
 * No simulator catalogue is encoded.
 *
 *
 * ============================================================================
 * FALLBACK CONTRACT
 * ============================================================================
 *
 * A fallback identifies an alternative semantic path.
 *
 * It does not perform the fallback.
 *
 * Semantic analysis must determine:
 *
 *     applicability
 *     compatibility
 *     contract preservation
 *     capability satisfaction
 *     resource feasibility
 *     provenance
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar deliberately imposes no finite language-level limits on:
 *
 *     execution clauses
 *     execution properties
 *     policy alternatives
 *     requirements
 *     constraints
 *     capabilities
 *     preferences
 *     fallbacks
 *     nested expressions
 *     qualified-name depth
 *     policy composition
 *
 * Repetition is represented using ANTLR `*` and `+`.
 *
 * Any actual limit is an implementation/resource limit, not a language
 * semantic ceiling.
 *
 * The grammar therefore remains applicable from:
 *
 *     tiny embedded computation
 *     single CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed system
 *     cloud
 *     heterogeneous systems
 *     future computational substrates
 *
 * subject only to actual resource availability and semantic feasibility.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal hardware capacities.
 *
 * It MUST NOT contain:
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
 * It MUST NOT enumerate:
 *
 *     processor models
 *     accelerator models
 *     QPU models
 *     vendor devices
 *     simulator vendors
 *     scheduler implementations
 *     routing algorithms
 *     quantum gate catalogues
 *
 *
 * ============================================================================
 * DETERMINISTIC PARSING
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text
 *     canonical lexer
 *     grammar version
 *     explicit compatibility configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     available hardware
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Semantic diagnostics belong downstream.
 *
 * The implementation MUST distinguish:
 *
 *     syntax error
 *     type error
 *     effect error
 *     resource error
 *     capability error
 *     contract error
 *     policy conflict
 *     authorization error
 *     target-feasibility error
 *     runtime failure
 *
 * For example:
 *
 *     unavailable GPU
 *
 * is NOT a grammar error.
 *
 *     unavailable quantum capability
 *
 * is NOT a grammar error.
 *
 *     unsatisfied memory requirement
 *
 * is NOT a grammar error.
 *
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar PolicyExecution;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Requirements,
    Constraints,
    Expressions
;


/*
 * ============================================================================
 * 1. PUBLIC PAYLOAD ROOT
 * ============================================================================
 *
 * This is the rule that grammar/core/policies.g4 should consume when a policy
 * contains execution-specific intent.
 *
 * The outer policy keyword, policy declaration, and terminating policy
 * statement remain owned by the parent policy grammar.
 *
 * This rule therefore deliberately does NOT consume:
 *
 *     POLICY
 *
 *     or a universal policy terminator.
 *
 * Example conceptual parent syntax:
 *
 *     policy execution {
 *         ...
 *     }
 *
 * The parent grammar determines how this payload is embedded.
 *
 * ============================================================================
 */

policyExecutionExpression
    : policyExecutionSpecification
    | policyExecutionAssignment
    | policyExecutionCondition
    | policyExecutionRequirement
    | policyExecutionConstraint
    | policyExecutionCapability
    | policyExecutionPreference
    | policyExecutionSelection
    | policyExecutionFallback
    | policyExecutionNegotiation
    | policyExecutionSimulation
    | policyExecutionAdaptation
    | policyExecutionResilience
    | policyExecutionSandbox
    | policyExecutionDeterminism
    | policyExecutionReproducibility
    | policyExecutionProperty
    | expression
    ;


/*
 * ============================================================================
 * 2. EXECUTION SPECIFICATION
 * ============================================================================
 *
 * The EXECUTION token establishes the execution-policy namespace without
 * requiring an application-specific keyword catalogue.
 *
 * Canonical form:
 *
 *     execution {
 *         ...
 *     }
 *
 * This is an execution payload, not a new policy declaration.
 *
 * ============================================================================
 */

policyExecutionSpecification
    : EXECUTION
      LBRACE
      policyExecutionClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 3. EXECUTION CLAUSE
 * ============================================================================
 *
 * The body is an ordered, unbounded collection.
 *
 * No fixed number of clauses is imposed.
 * ============================================================================
 */

policyExecutionClause
    : policyExecutionAssignment
    | policyExecutionCondition
    | policyExecutionRequirement
    | policyExecutionConstraint
    | policyExecutionCapability
    | policyExecutionPreference
    | policyExecutionSelection
    | policyExecutionFallback
    | policyExecutionNegotiation
    | policyExecutionSimulation
    | policyExecutionAdaptation
    | policyExecutionResilience
    | policyExecutionSandbox
    | policyExecutionDeterminism
    | policyExecutionReproducibility
    | policyExecutionProperty
    ;


/*
 * ============================================================================
 * 4. GENERIC EXECUTION ASSIGNMENT
 * ============================================================================
 *
 * Generic assignments are the principal open-world extensibility mechanism.
 *
 * Examples:
 *
 *     execution::strategy = strategy;
 *
 *     execution::mode = preferred_mode;
 *
 *     execution::placement = placement_intent;
 *
 *     execution::priority = priority_value;
 *
 *     execution::observability = observability_policy;
 *
 *     future::execution::dimension = value;
 *
 * The grammar does not create a closed catalogue.
 *
 * Semantic analysis decides whether a key is:
 *
 *     standardized
 *     dialect-defined
 *     vendor-defined
 *     experimental
 *     deprecated
 *     unknown
 *
 * ============================================================================
 */

policyExecutionAssignment
    : policyExecutionKey
      ASSIGN
      policyExecutionValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. CONDITIONAL EXECUTION INTENT
 * ============================================================================
 *
 * Conditions are ordinary Zamani expressions.
 *
 * Examples:
 *
 *     when execution::available;
 *
 *     when capability::quantum::measurement;
 *
 *     when workload::large;
 *
 * No condition is evaluated by the parser.
 * ============================================================================
 */

policyExecutionCondition
    : WHEN
      expression
      SEMICOLON
    | WHEN
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. REQUIREMENT
 * ============================================================================
 *
 * This is an execution-policy relationship to requirement intent.
 *
 * Requirement semantics remain owned by Requirements.
 *
 * ============================================================================
 */

policyExecutionRequirement
    : REQUIRES
      requirementExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. CONSTRAINT
 * ============================================================================
 *
 * Constraint semantics remain owned by Constraints.
 *
 * ============================================================================
 */

policyExecutionConstraint
    : CONSTRAINT
      constraintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. CAPABILITY
 * ============================================================================
 *
 * Capability resolution remains downstream.
 *
 * The expression is deliberately open-world.
 * ============================================================================
 */

policyExecutionCapability
    : CAPABILITY
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. PREFERENCE
 * ============================================================================
 *
 * Preferences remain advisory.
 *
 * A preference MUST NOT silently become a hard requirement.
 * ============================================================================
 */

policyExecutionPreference
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. SELECTION
 * ============================================================================
 *
 * Selection is semantic intent.
 *
 * It does NOT select a physical device at parse time.
 * ============================================================================
 */

policyExecutionSelection
    : SELECT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. FALLBACK
 * ============================================================================
 *
 * Fallback expresses an alternative semantic execution path.
 *
 * The fallback subsystem determines the detailed fallback specification.
 * This rule intentionally accepts an expression rather than inventing a
 * second fallback grammar.
 * ============================================================================
 */

policyExecutionFallback
    : FALLBACK
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. NEGOTIATION
 * ============================================================================
 *
 * Negotiation is declarative intent.
 *
 * Actual negotiation occurs after parsing.
 * ============================================================================
 */

policyExecutionNegotiation
    : NEGOTIATE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. SIMULATION
 * ============================================================================
 *
 * Simulation is an execution strategy.
 *
 * The expression may identify:
 *
 *     a simulation intent
 *     a simulation model
 *     a semantic simulation path
 *     a simulation policy
 *
 * No simulator implementation is encoded.
 * ============================================================================
 */

policyExecutionSimulation
    : SIMULATE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. ADAPTATION
 * ============================================================================
 *
 * This expresses controlled execution adaptation intent.
 *
 * It does not authorize self-modification.
 * ============================================================================
 */

policyExecutionAdaptation
    : ADAPT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. RESILIENCE
 * ============================================================================
 *
 * Resilience actions are intentionally open-ended through expressions.
 *
 * The known control outcomes are represented using canonical lexical tokens.
 *
 * Examples:
 *
 *     retry execution::alternative;
 *     recover execution::recovery_path;
 *     escalate execution::supervisory_path;
 *     reject execution::invalid_path;
 *
 * Runtime resilience semantics remain outside this grammar.
 * ============================================================================
 */

policyExecutionResilience
    : policyExecutionRetry
    | policyExecutionRecover
    | policyExecutionEscalate
    | policyExecutionReject
    ;


policyExecutionRetry
    : RETRY
      expression
      SEMICOLON
    ;


policyExecutionRecover
    : RECOVER
      expression
      SEMICOLON
    ;


policyExecutionEscalate
    : ESCALATE
      expression
      SEMICOLON
    ;


policyExecutionReject
    : REJECT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. SANDBOX
 * ============================================================================
 *
 * Sandbox is execution-policy intent.
 *
 * Security semantics remain owned by grammar/security/.
 *
 * The policy may constrain:
 *
 *     effects
 *     capabilities
 *     resources
 *     native operations
 *     foreign operations
 *     network behavior
 *     reflection
 *     adaptation
 *
 * ============================================================================
 */

policyExecutionSandbox
    : SANDBOX
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. DETERMINISM
 * ============================================================================
 *
 * Deterministic execution is expressed as policy intent.
 *
 * Semantic analysis determines whether the complete execution path can
 * satisfy the requested guarantee.
 *
 * ============================================================================
 */

policyExecutionDeterminism
    : DETERMINISTIC
      SEMICOLON
    | DETERMINISTIC
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility is policy intent.
 *
 * The semantic layer distinguishes:
 *
 *     source
 *     semantic
 *     compilation
 *     artifact
 *     execution
 *     result
 *
 * reproducibility.
 *
 * ============================================================================
 */

policyExecutionReproducibility
    : REPRODUCIBLE
      SEMICOLON
    | REPRODUCIBLE
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * Generic properties permit future execution-policy concepts without
 * continually expanding the core grammar.
 *
 * Examples:
 *
 *     execution::observability = detailed;
 *
 *     execution::checkpointing = enabled;
 *
 *     execution::resource_fairness = preferred;
 *
 *     quantum::execution::resilience = desired_level;
 *
 *     distributed::execution::consistency = consistency_policy;
 *
 *     future::execution::feature = value;
 *
 * ============================================================================
 */

policyExecutionProperty
    : policyExecutionKey
      COLON
      policyExecutionValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. EXECUTION KEY
 * ============================================================================
 *
 * Keys are open-world.
 *
 * Qualified names are preferred because they permit independently evolving
 * namespaces without requiring a new keyword for every feature.
 *
 * The identifier alternative exists for compatibility with existing
 * repository grammar patterns.
 * ============================================================================
 */

policyExecutionKey
    : qualifiedName
    | identifier
    ;


/*
 * ============================================================================
 * 21. EXECUTION VALUE
 * ============================================================================
 *
 * Values use the canonical expression grammar.
 *
 * This file does NOT create another expression language.
 * ============================================================================
 */

policyExecutionValue
    : expression
    ;


/*
 * ============================================================================
 * 22. EXECUTION EXPRESSION LIST
 * ============================================================================
 *
 * Unbounded by language semantics.
 * ============================================================================
 */

policyExecutionExpressionList
    : policyExecutionExpression
      (COMMA policyExecutionExpression)*
    ;


/*
 * ============================================================================
 * 23. EXECUTION PROPERTY LIST
 * ============================================================================
 */

policyExecutionPropertyList
    : policyExecutionProperty
      (policyExecutionProperty)*
    ;


/*
 * ============================================================================
 * 24. EXECUTION ASSIGNMENT LIST
 * ============================================================================
 */

policyExecutionAssignmentList
    : policyExecutionAssignment
      (policyExecutionAssignment)*
    ;


/*
 * ============================================================================
 * 25. EXECUTION CONDITION LIST
 * ============================================================================
 */

policyExecutionConditionList
    : policyExecutionCondition
      (policyExecutionCondition)*
    ;


/*
 * ============================================================================
 * 26. EXECUTION POLICY REFERENCE
 * ============================================================================
 *
 * An execution policy reference is represented as an ordinary qualified name.
 *
 * Examples:
 *
 *     execution::portable
 *     execution::deterministic
 *     execution::reproducible
 *     distributed::resilient
 *     quantum::fault_tolerant
 *     future::execution::policy
 *
 * These names are NOT reserved by this grammar.
 * ============================================================================
 */

policyExecutionReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 27. EXECUTION TARGET INTENT
 * ============================================================================
 *
 * Target intent is represented as a property/value relationship rather than
 * physical target selection.
 *
 * Example:
 *
 *     target = preferred_execution_class;
 *
 * The physical target is selected downstream.
 *
 * This rule is intentionally not a new target grammar.
 * ============================================================================
 */

policyExecutionTargetAssignment
    : TARGET
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 28. EXECUTION RESOURCE INTENT
 * ============================================================================
 *
 * Resource intent is delegated through an expression.
 *
 * This rule deliberately does not define quantities or resource categories.
 * ============================================================================
 */

policyExecutionResourceAssignment
    : RESOURCE
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 29. EXECUTION POLICY BINDING PAYLOAD
 * ============================================================================
 *
 * A policy may bind execution intent to a symbolic subject/context.
 *
 * The subject remains an expression.
 *
 * No physical object is identified.
 * ============================================================================
 */

policyExecutionBinding
    : policyExecutionBindingKey
      ASSIGN
      expression
      SEMICOLON
    ;


policyExecutionBindingKey
    : qualifiedName
    ;


/*
 * ============================================================================
 * 30. OPTIONAL EXECUTION PAYLOAD
 * ============================================================================
 */

optionalPolicyExecutionExpression
    : policyExecutionExpression?
    ;


/*
 * ============================================================================
 * 31. INTEGRATION WRAPPER
 * ============================================================================
 *
 * This wrapper provides a stable boundary for parent grammars.
 * ============================================================================
 */

policyExecutionPayload
    : policyExecutionSpecification
    | policyExecutionExpression
    ;


/*
 * ============================================================================
 * 32. INTEGRATION WITH grammar/core/policies.g4
 * ============================================================================
 *
 * The universal policy grammar remains the declaration authority.
 *
 * Its integration should be conceptually:
 *
 *     policyMember
 *         : ...
 *         | policyExecutionPayload
 *         ;
 *
 * OR, where the existing policy grammar already has a universal execution
 * action/member boundary:
 *
 *     policyAction
 *         : ...
 *         | policyExecutionPayload
 *         ;
 *
 * The exact parent rule MUST follow the actual current composition of
 * grammar/core/policies.g4.
 *
 * Do NOT create a second `policyDeclaration`.
 *
 * Do NOT move universal policy ownership here.
 *
 *
 * ============================================================================
 * 33. INTEGRATION WITH grammar/execution/policies.g4
 * ============================================================================
 *
 * grammar/execution/policies.g4 remains responsible for execution-layer
 * application/binding.
 *
 * Its existing rules:
 *
 *     executionPolicy
 *     executionPolicyReference
 *     executionPolicyBinding
 *     executionPolicyApplication
 *     executionPolicyBlock
 *
 * MUST consume semantic representations derived from this file rather than
 * redefining these policy payload rules.
 *
 * Dependency direction:
 *
 *     PolicyExecution
 *          |
 *          v
 *     policy semantic model
 *          |
 *          v
 *     ExecutionPolicies
 *
 * This prevents an ANTLR import cycle.
 *
 *
 * ============================================================================
 * 34. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements and constraints remain owned by:
 *
 *     grammar/resources/
 *
 * and universal requirements/constraints by:
 *
 *     grammar/core/
 *
 * This grammar merely delegates:
 *
 *     requires -> requirementExpression
 *     constraint -> constraintExpression
 *
 * It must never define:
 *
 *     resource count
 *     memory limit
 *     processor count
 *     device count
 *     qubit count
 *
 * as language-level capacities.
 *
 *
 * ============================================================================
 * 35. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability resolution remains downstream.
 *
 * This grammar accepts an expression after CAPABILITY so future capability
 * namespaces remain open.
 *
 * The semantic subsystem resolves:
 *
 *     capability existence
 *     capability version
 *     capability compatibility
 *     capability availability
 *     capability negotiation
 *
 *
 * ============================================================================
 * 36. EFFECT INTEGRATION
 * ============================================================================
 *
 * Execution policy may semantically constrain effects.
 *
 * Example:
 *
 *     sandbox execution::restricted;
 *
 * or through an open property:
 *
 *     execution::effects = permitted_effect_set;
 *
 * Effect ownership remains:
 *
 *     grammar/effects/
 *
 *
 * ============================================================================
 * 37. CONTRACT INTEGRATION
 * ============================================================================
 *
 * Execution policies may influence contracts.
 *
 * The grammar MUST NOT duplicate:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *
 * Contract validation remains downstream.
 *
 *
 * ============================================================================
 * 38. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * The semantic policy model should record:
 *
 *     source policy
 *     execution policy
 *     selected alternative
 *     fallback
 *     adaptation
 *     simulation choice
 *     resilience decision
 *     target feasibility result
 *
 * This allows execution decisions to be explained and reproduced where
 * requested.
 *
 *
 * ============================================================================
 * 39. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Valid conceptual usage includes:
 *
 *     execution {
 *         requires capability::quantum::measurement;
 *         prefer quantum::fault_tolerant;
 *         fallback classical::simulation;
 *         reproducible;
 *     }
 *
 * The policy does not define:
 *
 *     H
 *     X
 *     CNOT
 *     physical qubits
 *     coupling maps
 *     routing
 *     calibration
 *
 * Those remain downstream.
 *
 * The semantic path is:
 *
 *     policy
 *       |
 *       v
 *     execution intent
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization/decomposition/routing/scheduling
 *       |
 *       v
 *     resilience/QEC
 *       |
 *       v
 *     ZQN/HAL
 *
 *
 * ============================================================================
 * 40. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * The same policy can constrain classical execution:
 *
 *     execution {
 *         prefer classical::parallel;
 *         reproducible;
 *     }
 *
 * The policy does not select a particular CPU.
 *
 *
 * ============================================================================
 * 41. HDL/HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same execution policy can influence hardware realization:
 *
 *     execution {
 *         prefer hardware::energy_efficiency;
 *         fallback hardware::simulation;
 *     }
 *
 * Physical realization remains downstream.
 *
 *
 * ============================================================================
 * 42. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Execution policy can express:
 *
 *     execution {
 *         prefer distributed::resilient;
 *         fallback execution::local;
 *     }
 *
 * No node count is encoded.
 *
 *
 * ============================================================================
 * 43. AI / ADAPTIVE COMPUTATION INTEGRATION
 * ============================================================================
 *
 * Execution policy may govern controlled adaptation:
 *
 *     execution {
 *         adapt execution::strategy;
 *         require adaptation::authorized;
 *     }
 *
 * Adaptation must remain subject to:
 *
 *     policy
 *     capability
 *     effect
 *     resource
 *     contract
 *     provenance
 *
 *
 * ============================================================================
 * 44. SIMULATION INTEGRATION
 * ============================================================================
 *
 * Execution policy may express:
 *
 *     simulate quantum::execution;
 *
 *     simulate hardware::execution;
 *
 *     simulate distributed::execution;
 *
 * without enumerating simulator implementations.
 *
 *
 * ============================================================================
 * 45. SECURITY INTEGRATION
 * ============================================================================
 *
 * Sandbox intent may be expressed:
 *
 *     sandbox execution::restricted;
 *
 * But this file does NOT implement:
 *
 *     identity
 *     authorization
 *     credentials
 *     cryptography
 *     trust
 *     security enforcement
 *
 * Those remain security-layer responsibilities.
 *
 *
 * ============================================================================
 * 46. OPEN-WORLD EXTENSION
 * ============================================================================
 *
 * New execution dimensions should preferably use:
 *
 *     qualified names
 *     expressions
 *     properties
 *     capabilities
 *     policies
 *
 * instead of adding a new keyword.
 *
 * Examples that do NOT require grammar changes:
 *
 *     future::execution::strategy
 *     accelerator::execution::mode
 *     quantum::execution::resilience
 *     distributed::execution::consistency
 *     ai::execution::adaptation
 *     vendor::execution::feature
 *
 *
 * ============================================================================
 * 47. COMPATIBILITY
 * ============================================================================
 *
 * This grammar introduces no new lexical spelling.
 *
 * Therefore no keyword reservation is required solely for this file.
 *
 * Existing reserved words remain governed by:
 *
 *     grammar/lexer/keywords.g4
 *
 * Existing token composition remains governed by:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * A future keyword addition MUST NOT be introduced here as a parser-local
 * string literal.
 *
 *
 * ============================================================================
 * 48. NEGATIVE SYNTAX CONTRACT
 * ============================================================================
 *
 * The parser must reject malformed forms such as:
 *
 *     execution {
 *
 *     execution {
 *         requires;
 *     }
 *
 *     execution {
 *         prefer;
 *     }
 *
 *     execution {
 *         capability;
 *     }
 *
 *     execution {
 *         fallback;
 *     }
 *
 *     execution {
 *         when;
 *     }
 *
 *     execution {
 *         key =
 *     }
 *
 *     execution {
 *         requires quantum::measurement
 *     }
 *
 * where the enclosing policy grammar requires the terminating semicolon.
 *
 *
 * ============================================================================
 * 49. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     one clause
 *     many clauses
 *     nested expressions
 *     nested policy properties
 *     deeply qualified names
 *     future namespaces
 *     capability expressions
 *     requirement expressions
 *     constraint expressions
 *     conditional execution
 *     fallback
 *     simulation
 *     adaptation
 *     resilience
 *     deterministic execution
 *     reproducible execution
 *     sandbox intent
 *     mixed classical/quantum execution intent
 *     mixed classical/HDL intent
 *     distributed execution intent
 *     AI/adaptive execution intent
 *
 *
 * ============================================================================
 * 50. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST progressively exercise:
 *
 *     many execution clauses
 *     many policy properties
 *     large requirement expressions
 *     large capability expressions
 *     deep qualified names
 *     large cross-domain policies
 *
 * No test value may be interpreted as a language-level maximum.
 *
 *
 * ============================================================================
 * 51. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical source and identical parser configuration MUST produce equivalent
 * parse-tree structure.
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     target availability
 *     network state
 *     runtime state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *
 *
 * ============================================================================
 * 52. RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded Rust.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * The implementation must use safe Rust only.
 *
 * No unsafe Rust is required by this grammar.
 *
 *
 * ============================================================================
 * 53. HARD-CODING COMPLETION AUDIT
 * ============================================================================
 *
 * Before marking this file complete, verify that it contains no universal
 * finite limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     memory
 *     storage
 *     nodes
 *     devices
 *     tensor rank
 *     register width
 *     network size
 *
 * Also verify that it contains no closed catalogue of:
 *
 *     execution targets
 *     vendors
 *     devices
 *     simulators
 *     schedulers
 *     routing algorithms
 *     quantum operations
 *
 *
 * ============================================================================
 * 54. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] PolicyExecution is the sole owner of policy-domain execution payload
 *     syntax.
 *
 * [ ] grammar/core/policies.g4 remains the universal policy authority.
 *
 * [ ] grammar/execution/policies.g4 remains the execution subsystem adapter.
 *
 * [ ] No third policy language is introduced.
 *
 * [ ] No lexer rules are defined here.
 *
 * [ ] No new keyword is required.
 *
 * [ ] Names owns identifiers and qualified names.
 *
 * [ ] Expressions owns expression syntax.
 *
 * [ ] Requirements owns requirement-expression syntax.
 *
 * [ ] Constraints owns constraint-expression syntax.
 *
 * [ ] Resources remain outside this grammar.
 *
 * [ ] Capabilities remain semantically open-world.
 *
 * [ ] Effects remain outside this grammar.
 *
 * [ ] Contracts remain outside this grammar.
 *
 * [ ] Provenance remains outside this grammar.
 *
 * [ ] Security authorization remains outside this grammar.
 *
 * [ ] Quantum physical realization remains outside this grammar.
 *
 * [ ] HDL physical realization remains outside this grammar.
 *
 * [ ] No physical device identity is encoded.
 *
 * [ ] No universal resource ceiling is encoded.
 *
 * [ ] No quantum gate catalogue is encoded.
 *
 * [ ] No vendor catalogue is encoded.
 *
 * [ ] Future execution concepts can be represented through qualified names
 *     and expressions.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97+ generation succeeds.
 *
 * [ ] No unsafe Rust is required.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * An execution policy describes EXECUTION INTENT.
 *
 * It does not describe a physical machine.
 *
 * Therefore:
 *
 *     execution policy
 *         !=
 *     resource allocation
 *
 *     execution policy
 *         !=
 *     hardware selection
 *
 *     execution policy
 *         !=
 *     quantum routing
 *
 *     execution policy
 *         !=
 *     scheduling
 *
 *     execution policy
 *         !=
 *     backend implementation
 *
 *     execution policy
 *         !=
 *     runtime authorization
 *
 * The compiler and runtime resolve those concerns after parsing.
 *
 * This separation is fundamental to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */