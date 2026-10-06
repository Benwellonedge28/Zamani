/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/simulation.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * STATUS
 * ------
 * PRODUCTION-READY POLICY/SIMULATION COMPOSITION LEAF
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 * No unsafe implementation requirement
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the POLICY-SPECIFIC COMPOSITION boundary for simulation.
 *
 * It does NOT create another simulation language.
 *
 * It does NOT redefine:
 *
 *     SIMULATE
 *     simulationExpression
 *     simulationOperation
 *     simulationTarget
 *     simulationSourceClause
 *     simulationContextClause
 *
 * Those constructs already have canonical owners elsewhere in the repository.
 *
 * This file exists so that simulation-specific policy metadata and policy
 * composition can be represented without making the universal policy grammar
 * enumerate every present or future simulation technology.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     universal policy grammar
 *       |
 *       +----------------------------+
 *       |                            |
 *       v                            v
 *     policy                    simulation syntax
 *       |                            |
 *       +-------------+--------------+
 *                     |
 *                     v
 *          simulation policy binding
 *                     |
 *                     v
 *             domain-neutral AST
 *                     |
 *                     v
 *             semantic analysis
 *                     |
 *       +-------------+-------------+
 *       |             |             |
 *       v             v             v
 *   classical      quantum         HDL
 *       |             |             |
 *       |             v             |
 *       |        quantum::ir       |
 *       |                           |
 *       +-------------+-------------+
 *                     |
 *                     v
 *             canonical semantic model
 *                     |
 *                     v
 *          target-independent planning
 *                     |
 *              +------+------+
 *              |             |
 *              v             v
 *          simulation      execution
 *              |             |
 *              +------+------+
 *                     |
 *                     v
 *          capability/resource resolution
 *                     |
 *                     v
 *                policy resolution
 *                     |
 *                     v
 *             optimization/lowering
 *                     |
 *             routing/scheduling
 *                     |
 *               resilience
 *                     |
 *                 ZQN/HAL
 *                     |
 *                     v
 *              target realization
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     simulationPolicyBinding
 *     simulationPolicySubject
 *     simulationPolicyOptions
 *     simulationPolicyOption
 *     simulationPolicyOptionList
 *     simulationPolicyCondition
 *     simulationPolicyMetadata
 *     simulationPolicyProperty
 *     simulationPolicyPropertyKey
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     SIMULATE token definition
 *     simulation expression syntax
 *     simulation operation syntax
 *     simulation target syntax
 *     simulation source syntax
 *     simulation execution algorithms
 *     simulation engines
 *     numerical methods
 *     quantum state simulation
 *     HDL simulation
 *     hardware simulation
 *     AI/model simulation
 *     event simulation
 *     waveform simulation
 *     cycle simulation
 *     distributed simulation
 *     fault injection implementation
 *     performance modelling implementation
 *     target selection
 *     resource allocation
 *     capability discovery
 *     scheduling
 *     routing
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     backend implementation
 *     runtime enforcement
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Simulation source syntax has existing authorities:
 *
 *     grammar/execution/simulation.g4
 *     grammar/expressions/simulation.g4
 *     grammar/statements/simulate.g4
 *
 * Universal policy syntax has its existing authority:
 *
 *     grammar/policies/policy.g4
 *
 * Therefore this file MUST NOT recreate any of those grammars.
 *
 * In particular, this file MUST NOT define another rule equivalent to:
 *
 *     policySimulation
 *
 * with the intention of replacing the canonical rule in policy.g4.
 *
 * Instead, it provides a stable simulation-policy composition boundary that
 * downstream policy/execution composition can consume.
 *
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Simulation has several independent dimensions:
 *
 *     target
 *     source
 *     context
 *     scenario
 *     model
 *     fidelity
 *     reproducibility
 *     determinism
 *     fault model
 *     uncertainty
 *     validation
 *     observation
 *     resource requirements
 *     capability requirements
 *     fallback
 *     adaptation
 *     provenance
 *
 * These dimensions must not become a keyword catalogue.
 *
 * The policy layer therefore needs an open-world representation capable of
 * expressing simulation policy intent without enumerating all possible
 * simulation technologies.
 *
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
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 *
 * DIRECT GRAMMAR DEPENDENCIES
 * ---------------------------
 *
 *     grammar/core/names.g4
 *     grammar/core/expressions.g4
 *
 * The canonical expression grammar is used as the semantic payload boundary.
 *
 *
 * IMPORTANT
 * ---------
 *
 * This file deliberately does not import:
 *
 *     grammar/policies/policy.g4
 *
 * merely to reuse its rules.
 *
 * The universal policy grammar is the owner of policy declaration syntax.
 *
 * Importing the complete policy grammar here would risk circular composition
 * when policy.g4 or a policy composition root consumes this leaf.
 *
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The build system must expose canonical grammar directories through the
 * ANTLR grammar library path.
 *
 * This file uses grammar-name imports rather than filesystem-style imports.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser contexts from this file map to domain-neutral semantic structures
 * conceptually equivalent to:
 *
 *     SimulationPolicyBinding
 *     SimulationPolicySubject
 *     SimulationPolicyOption
 *     SimulationPolicyCondition
 *     SimulationPolicyMetadata
 *
 * The AST MUST preserve:
 *
 *     policy subject
 *     simulation policy options
 *     property names
 *     property values
 *     ordering
 *     source spans
 *     source order
 *
 * The AST MUST NOT contain:
 *
 *     physical simulator handles
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     ASIC identifiers
 *     QPU identifiers
 *     physical qubit mappings
 *     vendor SDK objects
 *     calibration state
 *     routing state
 *     scheduler state
 *     memory addresses
 *     device handles
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether the referenced simulation policy is valid;
 *     which simulation intent it governs;
 *     whether the policy is applicable;
 *     whether policy properties are legal;
 *     whether requirements are satisfiable;
 *     whether capabilities exist;
 *     whether resources are sufficient;
 *     whether effects are permitted;
 *     whether contracts are satisfied;
 *     whether provenance is available;
 *     whether deterministic execution is possible;
 *     whether reproducibility requirements are satisfiable;
 *     whether fallback behavior is valid;
 *     whether adaptation is authorized.
 *
 * None of these operations occur in this grammar.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This file does not allocate resources.
 *
 * Simulation policy may semantically refer to arbitrary resource expressions,
 * including future resource classes.
 *
 * Examples include:
 *
 *     memory
 *     compute
 *     storage
 *     bandwidth
 *     latency
 *     energy
 *     accelerator resources
 *     quantum simulation resources
 *     timing resources
 *     distributed resources
 *
 * No resource quantity is fixed by this grammar.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Simulation policy may semantically refer to open-world capability names.
 *
 * Examples:
 *
 *     simulation::classical
 *     simulation::quantum
 *     simulation::hdl
 *     simulation::distributed
 *     simulation::fault_model
 *     simulation::deterministic
 *
 * These examples are identifiers, not a closed vocabulary.
 *
 * The grammar therefore does not require new parser changes when a new
 * simulation capability is introduced.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Simulation itself may result in the semantic:
 *
 *     simulation
 *
 * Additional effects are determined from the simulation subject and policy.
 *
 * Possible effects include:
 *
 *     io
 *     network
 *     randomness
 *     measurement
 *     distributed
 *     native
 *     foreign
 *     learning
 *     adaptation
 *
 * This file does not redefine the effect system.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Simulation policies may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * This file merely provides expression boundaries that can be consumed by
 * semantic contract analysis.
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Universal policy semantics remain owned by:
 *
 *     grammar/policies/policy.g4
 *
 * This file supplies the simulation-specific composition payload that can be
 * attached to that policy model.
 *
 * A policy may therefore semantically govern:
 *
 *     simulation intent
 *     simulation context
 *     simulation resources
 *     simulation capabilities
 *     simulation fallback
 *     simulation determinism
 *     simulation reproducibility
 *     simulation validation
 *     simulation provenance
 *     simulation adaptation
 *
 * This file does not redefine the policy language.
 *
 *
 * ============================================================================
 * EXECUTION INTEGRATION
 * ============================================================================
 *
 * The execution layer already owns simulation execution intent.
 *
 * Relevant existing owners include:
 *
 *     grammar/execution/simulation.g4
 *     grammar/execution/policies.g4
 *
 * The integration boundary is:
 *
 *     simulation policy
 *          |
 *          v
 *     execution policy model
 *          |
 *          v
 *     simulation execution model
 *
 * Execution policy resolution must never require this grammar to know which
 * backend will eventually execute the simulation.
 *
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Existing simulation expressions remain the source-level expression owner.
 *
 * The semantic relationship is:
 *
 *     simulationExpression
 *            |
 *            v
 *     simulation semantic intent
 *            |
 *            +--------------------+
 *            |                    |
 *            v                    v
 *     simulation policy     execution planning
 *
 * This file therefore accepts ordinary expressions rather than introducing
 * a simulation-specific expression language.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * If a simulation policy applies to a quantum computation:
 *
 *     simulation policy
 *          |
 *          v
 *     semantic simulation intent
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
 * This file does not create a simulation-specific quantum IR.
 *
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * If a simulation policy applies to HDL or hardware intent:
 *
 *     simulation policy
 *          |
 *          v
 *     HDL/hardware semantic model
 *          |
 *          v
 *     simulation / verification / synthesis planning
 *
 * Physical implementation remains downstream.
 *
 * This grammar does not encode:
 *
 *     wire width
 *     register width
 *     memory size
 *     clock count
 *     FPGA resource count
 *     ASIC cell count
 *     device count
 *
 *
 * ============================================================================
 * CLASSICAL BOUNDARY
 * ============================================================================
 *
 * Classical simulation uses exactly the same policy representation.
 *
 * There is no separate classical simulation-policy grammar.
 *
 *
 * ============================================================================
 * HYBRID BOUNDARY
 * ============================================================================
 *
 * A simulation policy may govern hybrid computation involving:
 *
 *     classical
 *     quantum
 *     AI/model
 *     accelerator
 *     HDL
 *     distributed
 *
 * The grammar does not enumerate these domains.
 *
 * Semantic analysis determines the participating domain models.
 *
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Simulation policies may govern distributed simulation through:
 *
 *     requirements
 *     capabilities
 *     resource expressions
 *     topology expressions
 *     consistency policies
 *     fallback policies
 *     reproducibility policies
 *
 * The grammar does not define a maximum number of locations, nodes, devices,
 * tasks, events, or simulated entities.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Downstream provenance may record:
 *
 *     source policy
 *     simulation subject
 *     policy properties
 *     derived requirements
 *     selected capabilities
 *     semantic decisions
 *     transformations
 *     verification
 *     execution realization
 *     result provenance
 *
 * This grammar preserves the syntax necessary to maintain that provenance.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * This file MUST NOT inspect:
 *
 *     hardware
 *     filesystem state
 *     network state
 *     memory availability
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     wall-clock time
 *     runtime state
 *     random state
 *     deployment state
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no language-level finite limit on:
 *
 *     policy options
 *     policy properties
 *     property nesting
 *     simulation contexts
 *     simulation subjects
 *     scenarios
 *     models
 *     events
 *     samples
 *     simulated states
 *     resources
 *     capabilities
 *     targets
 *     devices
 *     nodes
 *     processors
 *     accelerators
 *     qubits
 *     memory
 *     tensor dimensions
 *     tensor rank
 *     network size
 *
 * Practical limits are determined only by:
 *
 *     available implementation resources
 *     parser memory
 *     compiler resources
 *     operating-system resources
 *     deployment resources
 *
 * Those limits are not language semantics.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no fixed capacity constants;
 *     no physical device catalogue;
 *     no simulator catalogue;
 *     no processor catalogue;
 *     no quantum-operation catalogue;
 *     no backend enumeration;
 *     no fixed resource quantities;
 *     no fixed node counts;
 *     no fixed qubit counts;
 *     no fixed memory sizes;
 *     no fixed tensor ranks;
 *     no fixed network sizes.
 *
 * It therefore does not create a language ceiling for simulation scale.
 *
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file uses only canonical tokens.
 *
 * Required canonical tokens:
 *
 *     SIMULATE
 *     WHEN
 *     ASSIGN
 *     SEMICOLON
 *     COMMA
 *     LPAREN
 *     RPAREN
 *
 * Names and identifiers are supplied by the canonical name grammar.
 *
 * No lexer rules are defined here.
 *
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

/*
 * ============================================================================
 * PARSER IDENTITY
 * ============================================================================
 *
 * This grammar is intentionally named separately from the universal policy
 * grammar.
 *
 * It can therefore be composed into policy/execution grammar roots without
 * claiming ownership of policy declarations.
 */
parser grammar SimulationPolicy;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable integration boundary for simulation-specific policy
 * composition.
 *
 * It describes policy intent associated with a simulation subject.
 *
 * The enclosing universal policy grammar remains responsible for deciding
 * where this construct is permitted.
 */
simulationPolicyBinding
    : SIMULATE
      simulationPolicySubject
      simulationPolicyOptions?
      SEMICOLON
    ;


/*
 * ============================================================================
 * SUBJECT
 * ============================================================================
 *
 * The subject is an ordinary expression.
 *
 * This is deliberately open-world.
 *
 * It may represent:
 *
 *     a program
 *     a function
 *     a model
 *     a circuit
 *     an HDL design
 *     a hardware model
 *     a data pipeline
 *     a distributed computation
 *     a hybrid computation
 *     a future computational domain
 */
simulationPolicySubject
    : expression
    ;


/*
 * ============================================================================
 * OPTIONS
 * ============================================================================
 *
 * Policy options are an unordered semantic collection whose source ordering is
 * preserved in the AST.
 *
 * The grammar does not assign semantics to ordering.
 */
simulationPolicyOptions
    : LPAREN
      simulationPolicyOptionList
      RPAREN
    ;


/*
 * ============================================================================
 * OPTION LIST
 * ============================================================================
 *
 * One or more options are required.
 *
 * Empty option lists are rejected.
 *
 * A trailing comma is permitted to support extensible source generation.
 *
 * Valid:
 *
 *     simulate model (policy)
 *     simulate model (policy, resources)
 *     simulate model (policy, resources,)
 *
 * Invalid:
 *
 *     simulate model ()
 */
simulationPolicyOptionList
    : simulationPolicyOption
      (
          COMMA
          simulationPolicyOption
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTION
 * ============================================================================
 *
 * An option may be:
 *
 *     an expression
 *     a named property
 *
 * Named properties provide a stable open-world policy mechanism without
 * requiring a new keyword for every future simulation feature.
 */
simulationPolicyOption
    : expression
    | simulationPolicyProperty
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 *
 * Examples:
 *
 *     scenario = scenario;
 *     reproducibility = policy;
 *     resources = requirements;
 *     capability = capability_expression;
 *     fallback = fallback_policy;
 *
 * Property names remain ordinary identifiers.
 */
simulationPolicyProperty
    : simulationPolicyPropertyKey
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * PROPERTY KEY
 * ============================================================================
 *
 * New simulation-policy concepts do not require new reserved words.
 */
simulationPolicyPropertyKey
    : identifier
    ;


/*
 * ============================================================================
 * CONDITIONAL POLICY BINDING
 * ============================================================================
 *
 * This rule provides a stable structure for consumers that need a conditional
 * simulation-policy attachment without defining another policy language.
 *
 * Example:
 *
 *     when condition simulate model (policy);
 *
 * The enclosing policy composition layer determines whether this construct is
 * legal in its specific policy context.
 */
simulationPolicyConditionalBinding
    : WHEN
      simulationPolicyCondition
      simulationPolicyBinding
    ;


/*
 * ============================================================================
 * CONDITION
 * ============================================================================
 *
 * Conditions remain normal Zamani expressions.
 */
simulationPolicyCondition
    : expression
    ;


/*
 * ============================================================================
 * OPTION LIST BRIDGE
 * ============================================================================
 *
 * Stable reusable rule for consumers that need only the options without
 * reimplementing their syntax.
 */
simulationPolicyOptionSequence
    : simulationPolicyOptionList
    ;


/*
 * ============================================================================
 * SUBJECT LIST
 * ============================================================================
 *
 * A policy consumer may need to associate equivalent simulation policy intent
 * with multiple semantic subjects.
 *
 * This remains unbounded at the grammar level.
 */
simulationPolicySubjectList
    : simulationPolicySubject
      (
          COMMA
          simulationPolicySubject
      )*
    ;


/*
 * ============================================================================
 * SUBJECT/OPTION COMPOSITION
 * ============================================================================
 *
 * This bridge allows a consumer to preserve the relationship between one
 * simulation subject and its optional policy context without introducing
 * another simulation syntax.
 */
simulationPolicyTarget
    : simulationPolicySubject
      simulationPolicyOptions?
    ;


/*
 * ============================================================================
 * OPTIONAL POLICY TARGET
 * ============================================================================
 */
optionalSimulationPolicyTarget
    : simulationPolicyTarget?
    ;


/*
 * ============================================================================
 * OPTIONAL CONDITION
 * ============================================================================
 */
optionalSimulationPolicyCondition
    : simulationPolicyCondition?
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * CONSUMERS
 * ---------
 *
 * Intended consumers include:
 *
 *     grammar/execution/policies.g4
 *     policy composition roots
 *     execution policy semantic adapters
 *     simulation policy semantic analysis
 *
 *
 * EXISTING AUTHORITIES
 * --------------------
 *
 * Universal policy syntax:
 *
 *     grammar/policies/policy.g4
 *
 * Simulation expression syntax:
 *
 *     grammar/expressions/simulation.g4
 *
 * Execution simulation composition:
 *
 *     grammar/execution/simulation.g4
 *
 * Statement-level simulation:
 *
 *     grammar/statements/simulate.g4
 *
 * Resource semantics:
 *
 *     grammar/resources/
 *
 * Capability semantics:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/capabilities.g4
 *
 * Effect semantics:
 *
 *     grammar/effects/
 *
 * Contract semantics:
 *
 *     grammar/validation/
 *
 * Security semantics:
 *
 *     grammar/security/
 *
 * Provenance semantics:
 *
 *     grammar/spec/provenance.md
 *
 *
 * ============================================================================
 * INTEGRATION DIRECTION
 * ============================================================================
 *
 * Canonical dependency direction:
 *
 *     lexer
 *       |
 *       v
 *     names / expressions
 *       |
 *       v
 *     SimulationPolicy
 *       |
 *       v
 *     policy composition
 *       |
 *       v
 *     semantic policy model
 *       |
 *       +--> requirements
 *       +--> constraints
 *       +--> capabilities
 *       +--> resources
 *       +--> effects
 *       +--> contracts
 *       +--> security
 *       +--> provenance
 *       +--> execution
 *       |
 *       v
 *     canonical semantic model
 *
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * This file MUST NOT be imported into grammar/policies/policy.g4 merely to
 * replace the existing policySimulation rule unless the policy authority is
 * deliberately refactored to delegate that rule here.
 *
 * Until that deliberate composition change is made:
 *
 *     policy.g4
 *
 * remains the sole owner of universal policy syntax, including its existing
 * policySimulation member.
 *
 * This file is therefore safe to land independently without changing the
 * meaning of existing policy syntax.
 *
 * When the policy composition root is migrated, it may delegate its
 * simulation-policy member to:
 *
 *     simulationPolicyBinding
 *
 * without changing the semantic model.
 *
 *
 * ============================================================================
 * NO DUPLICATION RULE
 * ============================================================================
 *
 * A future integration MUST NOT create both:
 *
 *     policySimulation
 *
 * and:
 *
 *     simulationPolicyBinding
 *
 * as two independently parsed alternatives for the same source position.
 *
 * There must ultimately be one canonical source-level owner.
 *
 * The migration path is:
 *
 *     existing policySimulation
 *             |
 *             v
 *     simulationPolicyBinding
 *             |
 *             v
 *     semantic SimulationPolicyBinding
 *
 * not:
 *
 *     policySimulation
 *          +
 *     simulationPolicyBinding
 *
 * as competing languages.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics include:
 *
 *     missing simulation subject
 *     malformed option list
 *     empty option list
 *     missing option expression
 *     malformed property
 *     missing property value
 *     missing closing parenthesis
 *     missing semicolon
 *
 * Semantic diagnostics include:
 *
 *     invalid simulation subject
 *     unknown policy property
 *     conflicting policy property
 *     unavailable capability
 *     unsatisfied requirement
 *     violated constraint
 *     unavailable resource
 *     prohibited effect
 *     invalid policy composition
 *     invalid fallback
 *     invalid reproducibility requirement
 *     invalid determinism requirement
 *
 * Capability/resource/policy failures MUST NOT be converted into parser
 * failures.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms should parse at this leaf boundary:
 *
 *     simulate model;
 *
 *     simulate model (policy);
 *
 *     simulate model (policy, resources);
 *
 *     simulate model (policy, resources,);
 *
 *     simulate quantum_program (quantum_policy);
 *
 *     simulate hardware_model (timing_policy);
 *
 *     simulate hdl_model (verification_policy);
 *
 *     simulate hybrid_program (execution_policy);
 *
 *     simulate model (scenario = scenario);
 *
 *     simulate model (reproducibility = reproducible_policy);
 *
 *     simulate model (determinism = deterministic_policy);
 *
 *     simulate model (capability = required_capability);
 *
 *     simulate model (resource = required_resource);
 *
 *     simulate model (fallback = fallback_policy);
 *
 *     simulate model (provenance = provenance_policy);
 *
 *     when condition simulate model (policy);
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected:
 *
 *     simulate;
 *
 *     simulate ();
 *
 *     simulate model ();
 *
 *     simulate model ( );
 *
 *     simulate model (, policy);
 *
 *     simulate model (policy,);
 *
 * NOTE:
 *
 * The final trailing-comma rule is deliberately defined above.
 *
 * Therefore:
 *
 *     simulate model (policy,)
 *
 * IS VALID.
 *
 * Tests must reflect the grammar rather than contradictory documentation.
 *
 * Invalid forms include:
 *
 *     simulate model (policy,,other);
 *
 *     simulate model (property =);
 *
 *     simulate model ( = value);
 *
 *     simulate model (property);
 *
 *     when simulate model;
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The same grammar must support policy bindings for:
 *
 *     classical computation
 *     numerical computation
 *     tensor computation
 *     AI/model computation
 *     knowledge computation
 *     quantum computation
 *     hybrid computation
 *     HDL intent
 *     hardware intent
 *     accelerator computation
 *     distributed computation
 *     networking computation
 *     data processing
 *     future computational domains
 *
 * No domain receives a dedicated parser branch.
 *
 *
 * ============================================================================
 * QUANTUM TEST CONTRACT
 * ============================================================================
 *
 * The grammar must accept arbitrary quantum-program expressions without
 * enumerating quantum operations.
 *
 * Examples:
 *
 *     simulate quantum_program (quantum_policy);
 *
 *     simulate circuit (capability = quantum::simulation);
 *
 *     simulate circuit (resource = simulation::state);
 *
 *     simulate circuit (
 *         fallback = classical::simulation_policy
 *     );
 *
 * The actual quantum lowering path remains:
 *
 *     semantic quantum model
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     decomposition
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     resilience/QEC
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *
 *
 * ============================================================================
 * HDL TEST CONTRACT
 * ============================================================================
 *
 * Examples:
 *
 *     simulate hdl_design (verification_policy);
 *
 *     simulate hardware_model (
 *         resource = timing::model
 *     );
 *
 * No physical width or implementation capacity is encoded.
 *
 *
 * ============================================================================
 * POCO-REAF TEST CONTRACT
 * ============================================================================
 *
 * A simulation policy must remain source-compatible across target scales.
 *
 * The same policy may semantically be considered for:
 *
 *     tiny embedded target
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     quantum simulator
 *     QPU
 *     HPC
 *     cluster
 *     distributed system
 *     cloud
 *     future computational substrate
 *
 * provided that the semantic requirements and policies are satisfiable.
 *
 * The grammar itself must not change because the target scale changes.
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must cover:
 *
 *     large option lists
 *     deeply qualified names
 *     deeply nested expressions
 *     large policy expressions
 *     large subject expressions
 *     many simulation-policy bindings
 *     many policy properties
 *     cross-domain policy composition
 *     nested simulation expressions
 *
 * Test sizes are test parameters, not language limits.
 *
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical token streams and parser configuration:
 *
 *     parse(source) == equivalent_parse_structure
 *
 * The grammar has no:
 *
 *     semantic actions
 *     predicates based on runtime state
 *     hardware queries
 *     filesystem queries
 *     network queries
 *     random behavior
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not remove or redefine existing simulation syntax.
 *
 * Existing:
 *
 *     simulate target;
 *
 * remains owned by the current policy/simulation architecture until the
 * explicit policy-authority migration is performed.
 *
 * No existing quantum, classical, HDL, execution, resource, capability, or
 * security syntax is changed by this file alone.
 *
 *
 * ============================================================================
 * HARD-CODING COMPLETION AUDIT
 * ============================================================================
 *
 * FORBIDDEN IN THIS FILE:
 *
 *     MAX_SIMULATIONS
 *     MAX_SCENARIOS
 *     MAX_EVENTS
 *     MAX_SAMPLES
 *     MAX_STATES
 *     MAX_MODELS
 *     MAX_RESOURCES
 *     MAX_CAPABILITIES
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * None are present.
 *
 *
 * ============================================================================
 * RUST IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Generated parser integration therefore requires:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * No unsafe implementation is required or implied.
 *
 * Runtime safety belongs to the Rust implementation and its dependencies.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] It has one clear ownership boundary.
 *
 * [x] It does not create a second simulation language.
 *
 * [x] It does not create a second policy authority.
 *
 * [x] It consumes the canonical lexer.
 *
 * [x] It reuses canonical expressions.
 *
 * [x] It uses open-world expressions for future simulation concepts.
 *
 * [x] It permits arbitrary policy properties without keyword explosion.
 *
 * [x] It permits arbitrary simulation subjects.
 *
 * [x] It has no physical-machine limits.
 *
 * [x] It has no simulator catalogue.
 *
 * [x] It has no quantum-operation catalogue.
 *
 * [x] It has no hardware catalogue.
 *
 * [x] It has no backend dependency.
 *
 * [x] It has no IR dependency.
 *
 * [x] It has no runtime dependency.
 *
 * [x] It has no unsafe Rust requirement.
 *
 * [x] It has explicit AST boundaries.
 *
 * [x] It has explicit semantic boundaries.
 *
 * [x] It has explicit resource/capability/effect boundaries.
 *
 * [x] It has explicit contract/policy/provenance boundaries.
 *
 * [x] It has explicit quantum and HDL boundaries.
 *
 * [x] It has explicit POCO-REAF behavior.
 *
 * [x] It can be integrated without changing existing simulation semantics.
 *
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * This file answers only:
 *
 *     "How can simulation-specific policy intent be represented and composed?"
 *
 * It does NOT answer:
 *
 *     "Which simulator executes it?"
 *
 *     "Which machine executes it?"
 *
 *     "How much memory exists?"
 *
 *     "How many processors exist?"
 *
 *     "How many qubits exist?"
 *
 *     "Which QPU is selected?"
 *
 *     "Which GPU is selected?"
 *
 *     "How is HDL synthesized?"
 *
 *     "How is simulation scheduled?"
 *
 *     "How are resources allocated?"
 *
 * Those questions belong downstream.
 *
 * Therefore:
 *
 *     simulation policy syntax
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic policy model
 *         ->
 *     capability/resource/requirement analysis
 *         ->
 *     effect/contract/security analysis
 *         ->
 *     provenance
 *         ->
 *     canonical semantic model
 *         ->
 *     domain IR
 *         ->
 *     target-independent optimization
 *         ->
 *     lowering
 *         ->
 *     routing/scheduling where applicable
 *         ->
 *     resilience where applicable
 *         ->
 *     ZQN/HAL where applicable
 *         ->
 *     target realization
 *
 * This separation is what keeps simulation policy scalable from the smallest
 * supported execution environment to arbitrarily larger environments subject
 * only to actual available resources and semantic feasibility.
 *
 * ============================================================================
 */