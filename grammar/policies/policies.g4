/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/policies.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * GRAMMAR NAME
 * ------------
 * Policies
 *
 * STATUS
 * ------
 * CANONICAL POLICY-DOMAIN COMPOSITION / ORCHESTRATION GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 * No unsafe Rust
 *
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION AUTHORITY for grammar/policies/.
 *
 * It does not define another policy language.
 *
 * It does not duplicate policy syntax owned by:
 *
 *     policy.g4
 *     constitution.g4
 *     scopes.g4
 *     requirements.g4
 *     constraints.g4
 *     permissions.g4
 *     prohibitions.g4
 *     preferences.g4
 *     fallbacks.g4
 *     adaptation.g4
 *     execution.g4
 *     security.g4
 *     resource.g4
 *     deployment.g4
 *     simulation.g4
 *     provenance.g4
 *
 * Instead, this grammar composes those grammars into one stable policy-domain
 * boundary that can be consumed by ZamaniParser.g4.
 *
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                        ZamaniParser
 *                              |
 *                              v
 *                         Policies
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *           Policy       Constitution       Policy leaves
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                    domain-neutral AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      contracts           resources           effects
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                       semantic policies
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *       security          execution          provenance
 *                              |
 *                              v
 *                     target-independent plan
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir        HDL/hardware
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                         routing / scheduling
 *                              |
 *                        resilience / recovery
 *                              |
 *                            ZQN
 *                              |
 *                            HAL
 *                              |
 *                       target realization
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one composition authority for grammar/policies/.
 *
 * That authority is THIS FILE.
 *
 * Individual files below grammar/policies/ are LEAF or DOMAIN-ADAPTER
 * grammars. They must not independently become universal policy roots.
 *
 * The hierarchy is:
 *
 *     grammar/policies/policies.g4
 *                         |
 *          +--------------+--------------+
 *          |                             |
 *          v                             v
 *     policy.g4                    constitution.g4
 *          |
 *          +-------------------------------+
 *          |        |        |       |      |
 *          v        v        v       v      v
 *      requirements constraints permissions ...
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     policySystem
 *     policyElement
 *     policyDeclarationElement
 *     constitutionElement
 *
 * These are composition rules.
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
 *     constitutionDeclaration
 *     constitutionBody
 *
 * Those remain owned by their respective leaf grammars.
 *
 * It also does not own:
 *
 *     lexer rules
 *     tokens
 *     keywords
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     requirements
 *     constraints
 *     capabilities
 *     resources
 *     permissions
 *     prohibitions
 *     preferences
 *     fallbacks
 *     adaptation
 *     execution
 *     security
 *     deployment
 *     simulation
 *     provenance
 *     contracts
 *     effects
 *     quantum operations
 *     hardware
 *     runtime behavior
 *     IR definitions
 *     target selection
 *     resource allocation
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Policy declaration authority:
 *
 *     grammar/policies/policy.g4
 *
 * Constitutional governance authority:
 *
 *     grammar/policies/constitution.g4
 *
 * Policy scope:
 *
 *     grammar/policies/scopes.g4
 *
 * Requirement adapter:
 *
 *     grammar/policies/requirements.g4
 *
 * Constraint adapter:
 *
 *     grammar/policies/constraints.g4
 *
 * Permission adapter:
 *
 *     grammar/policies/permissions.g4
 *
 * Prohibition adapter:
 *
 *     grammar/policies/prohibitions.g4
 *
 * Preference adapter:
 *
 *     grammar/policies/preferences.g4
 *
 * Fallback adapter:
 *
 *     grammar/policies/fallbacks.g4
 *
 * Adaptation adapter:
 *
 *     grammar/policies/adaptation.g4
 *
 * Execution adapter:
 *
 *     grammar/policies/execution.g4
 *
 * Security adapter:
 *
 *     grammar/policies/security.g4
 *
 * Resource adapter:
 *
 *     grammar/policies/resource.g4
 *
 * Deployment adapter:
 *
 *     grammar/policies/deployment.g4
 *
 * Simulation adapter:
 *
 *     grammar/policies/simulation.g4
 *
 * Provenance adapter:
 *
 *     grammar/policies/provenance.g4
 *
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * All imports are ANTLR grammar-name imports.
 *
 * They are NOT filesystem imports.
 *
 * The build system MUST make the complete grammar source tree available to
 * ANTLR's grammar-library path.
 *
 * ============================================================================
 */

parser grammar Policies;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * POLICY-DOMAIN COMPOSITION IMPORTS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This list is the composition manifest for grammar/policies/.
 *
 * Adding a new policy-domain file requires adding its grammar to this
 * composition manifest.
 *
 * Removing a policy-domain file requires removing it here and updating the
 * specification/conformance metadata.
 *
 * A policy leaf MUST NOT bypass this composition boundary in ZamaniParser.
 * ============================================================================
 */

import
    Policy,
    Constitution,
    PolicyScopes,
    PolicyRequirements,
    PolicyConstraints,
    PolicyPermissions,
    Prohibitions,
    PolicyPreferences,
    PolicyFallbacks,
    PolicyAdaptation,
    PolicyExecution,
    PolicySecurity,
    PolicyResource,
    PolicyDeployment,
    SimulationPolicy,
    ProvenancePolicy
;


/*
 * ============================================================================
 * 1. CANONICAL POLICY-DOMAIN ENTRY POINT
 * ============================================================================
 *
 * `policySystem` is the only public entry point that the universal parser
 * needs for the policy domain.
 *
 * A policy-domain source element is either:
 *
 *     policy declaration
 *
 * or:
 *
 *     constitutional governance declaration
 *
 * Specialized policy constructs are NOT independently admitted here.
 *
 * For example, this file does NOT allow:
 *
 *     policyPermission
 *     policyRequirement
 *     policySimulation
 *     policyAdaptation
 *
 * as independent top-level source elements.
 *
 * Those constructs belong inside their owning policy/constitution structures.
 *
 * This prevents policy fragments from becoming accidental second-class
 * top-level language declarations.
 */

policySystem
    : policyElement
    ;


/*
 * ============================================================================
 * 2. POLICY ELEMENT
 * ============================================================================
 *
 * Stable composition boundary for ZamaniParser.
 */

policyElement
    : policyDeclarationElement
    | constitutionElement
    ;


/*
 * ============================================================================
 * 3. ORDINARY POLICY
 * ============================================================================
 *
 * The actual declaration syntax remains owned by policy.g4.
 */

policyDeclarationElement
    : policyDeclaration
    ;


/*
 * ============================================================================
 * 4. CONSTITUTION
 * ============================================================================
 *
 * The actual constitution syntax remains owned by constitution.g4.
 *
 * A constitution is governance metadata/intent.
 *
 * It is not executable code.
 *
 * It does not select physical hardware.
 *
 * It does not allocate resources.
 *
 * It does not perform authorization itself.
 */

constitutionElement
    : constitutionDeclaration
    ;


/*
 * ============================================================================
 * 5. COMPOSITION GUARANTEE
 * ============================================================================
 *
 * The following rule is intentionally NOT defined:
 *
 *     policyMember
 *
 * The reason is critical:
 *
 *     policyMember
 *
 * belongs to policy.g4.
 *
 * This orchestrator must not duplicate or redefine the policy body's member
 * dispatch.
 *
 * Instead:
 *
 *     Policies
 *         |
 *         +--> Policy
 *                  |
 *                  +--> policyMember
 *                           |
 *                           +--> PolicyRequirements
 *                           +--> PolicyConstraints
 *                           +--> PolicyPermissions
 *                           +--> ...
 *
 *
 * ============================================================================
 * 6. CONSTITUTION COMPOSITION GUARANTEE
 * ============================================================================
 *
 * The constitution grammar owns its internal member dispatch.
 *
 * Therefore this file deliberately does not define:
 *
 *     constitutionMember
 *
 * or any constitutional clause.
 *
 *
 * ============================================================================
 * 7. POLICY LEAF ORCHESTRATION CONTRACT
 * ============================================================================
 *
 * The imported policy leaves have the following architectural roles.
 *
 *
 * Policy
 * ------
 *
 * Owns:
 *
 *     policy declaration
 *     policy body
 *     policy member dispatch
 *     generic policy rule
 *     policy-level composition
 *
 *
 * Constitution
 * ------------
 *
 * Owns:
 *
 *     constitutional declaration
 *     constitutional principles
 *     constitutional invariants
 *     foundational governance
 *     amendment relationships
 *     constitutional precedence
 *
 *
 * PolicyScopes
 * ------------
 *
 * Owns:
 *
 *     scope syntax
 *     scope references
 *     applicability boundaries
 *
 *
 * PolicyRequirements
 * ------------------
 *
 * Owns the policy adapter for universal requirements.
 *
 *
 * PolicyConstraints
 * -----------------
 *
 * Owns the policy adapter for universal constraints.
 *
 *
 * PolicyPermissions
 * -----------------
 *
 * Owns policy permission syntax.
 *
 * It does not own security authorization.
 *
 *
 * Prohibitions
 * ------------
 *
 * Owns policy prohibition syntax.
 *
 *
 * PolicyPreferences
 * -----------------
 *
 * Owns advisory policy preferences.
 *
 *
 * PolicyFallbacks
 * ---------------
 *
 * Owns declarative policy fallback relationships.
 *
 *
 * PolicyAdaptation
 * ----------------
 *
 * Owns policy governance of adaptive computation.
 *
 * It does not implement executable adaptation.
 *
 *
 * PolicyExecution
 * ---------------
 *
 * Owns execution-specific policy payloads.
 *
 *
 * PolicySecurity
 * --------------
 *
 * Owns policy/security composition.
 *
 * It does not replace the security subsystem.
 *
 *
 * PolicyResource
 * --------------
 *
 * Owns policy/resource composition.
 *
 *
 * PolicyDeployment
 * ----------------
 *
 * Owns policy/deployment composition.
 *
 *
 * SimulationPolicy
 * ----------------
 *
 * Owns simulation-specific policy payloads.
 *
 * It does not own the general `simulate` statement.
 *
 *
 * ProvenancePolicy
 * ----------------
 *
 * Owns structured provenance policy payloads.
 *
 * It does not own the universal provenance model.
 *
 *
 * ============================================================================
 * 8. OWNERSHIP GRAPH
 * ============================================================================
 *
 *                     Policies
 *                         |
 *             +-----------+-----------+
 *             |                       |
 *             v                       v
 *          Policy                Constitution
 *             |                       |
 *             |                       +--> foundational governance
 *             |
 *             +--> scopes
 *             +--> requirements
 *             +--> constraints
 *             +--> permissions
 *             +--> prohibitions
 *             +--> preferences
 *             +--> fallbacks
 *             +--> adaptation
 *             +--> execution
 *             +--> security
 *             +--> resources
 *             +--> deployment
 *             +--> simulation
 *             +--> provenance
 *             |
 *             v
 *        semantic policy model
 *
 *
 * ============================================================================
 * 9. UNIVERSAL SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar establishes NO target-specific semantic model.
 *
 * Policy information flows into the common semantic representation:
 *
 *     policy intent
 *          |
 *          +--> requirements
 *          +--> constraints
 *          +--> capabilities
 *          +--> resources
 *          +--> effects
 *          +--> contracts
 *          +--> security
 *          +--> provenance
 *          |
 *          v
 *     semantic policy model
 *
 * The semantic policy model can then govern:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     interoperability
 *     simulation
 *     deployment
 *     future domains
 *
 * No policy grammar rule may assume that a domain is finite.
 *
 *
 * ============================================================================
 * 10. POCO-REAF INVARIANT
 * ============================================================================
 *
 * Policies express intent and governance.
 *
 * They MUST NOT encode a universal machine-size model.
 *
 * In particular, this grammar introduces no limits on:
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
 *     logical qubits
 *     nodes
 *     processes
 *     tasks
 *     channels
 *     devices
 *     memory
 *     storage
 *     registers
 *     vector width
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     topology size
 *
 * A policy may reference symbolic requirements such as:
 *
 *     capability("quantum.measurement")
 *
 *     capability("tensor.compute")
 *
 *     memory >= required_memory
 *
 *     qubits >= required_qubits
 *
 *     topology(required_topology)
 *
 * without defining a maximum.
 *
 *
 * ============================================================================
 * 11. OPEN-WORLD DOMAIN CONTRACT
 * ============================================================================
 *
 * The policy layer MUST remain open-world.
 *
 * New semantic capabilities, resource classes, execution mechanisms,
 * accelerators, quantum technologies, hardware technologies, AI models,
 * distributed mechanisms, or future computational domains MUST NOT require
 * a new policy keyword merely because they are new.
 *
 * They should normally be represented through:
 *
 *     qualified names
 *     expressions
 *     capabilities
 *     requirements
 *     constraints
 *     properties
 *     dialects
 *     metadata
 *
 * This is essential to POCO-REAF.
 *
 *
 * ============================================================================
 * 12. APPLICATION-SPECIFIC KEYWORD PROHIBITION
 * ============================================================================
 *
 * The policy orchestrator MUST NOT grow a keyword catalog for application
 * domains.
 *
 * The following concepts, for example, must remain semantic/library/dialect
 * concepts rather than becoming policy keywords merely because an application
 * uses them:
 *
 *     computer vision
 *     sentiment analysis
 *     robotics
 *     blockchain
 *     VR
 *     AR
 *     payment systems
 *     administrative operations
 *     legal workflows
 *
 * A policy may govern such systems through generic:
 *
 *     capabilities
 *     resources
 *     effects
 *     contracts
 *     security rules
 *     provenance
 *     requirements
 *     constraints
 *
 * without adding application-specific grammar vocabulary.
 *
 *
 * ============================================================================
 * 13. AI / REASONING INTEGRATION
 * ============================================================================
 *
 * Policy is deliberately capable of governing generic intelligent computation
 * without making AI a special policy language.
 *
 * Existing semantic capabilities may include:
 *
 *     reasoning
 *     inference
 *     deduction
 *     learning
 *     adaptation
 *     knowledge access
 *     uncertainty
 *     evidence
 *     explanation
 *     decision provenance
 *
 * Policy governance may therefore constrain these operations through:
 *
 *     capabilities
 *     effects
 *     requirements
 *     contracts
 *     permissions
 *     prohibitions
 *     provenance
 *     security
 *
 * No AI algorithm catalogue is introduced here.
 *
 *
 * ============================================================================
 * 14. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Policy syntax remains independent of quantum implementation.
 *
 * Policy may govern semantic identities such as:
 *
 *     quantum::execute
 *     quantum::measure
 *     quantum::prepare
 *     quantum::adapt
 *     quantum::error_correction
 *
 * These are symbolic semantic identities.
 *
 * This grammar does NOT enumerate:
 *
 *     gates
 *     physical qubits
 *     hardware topology
 *     calibration
 *     routing
 *     scheduling
 *
 * The canonical downstream boundary remains:
 *
 *     policy
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 *
 * ============================================================================
 * 15. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Policies may govern HDL/hardware intent through symbolic expressions,
 * capabilities, resources, effects and contracts.
 *
 * This grammar does not impose:
 *
 *     register widths
 *     bus widths
 *     device counts
 *     memory capacities
 *     clock counts
 *     topology sizes
 *     pipeline depths
 *
 * Physical realization remains outside grammar/policies/.
 *
 *
 * ============================================================================
 * 16. DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Policy may govern:
 *
 *     messages
 *     services
 *     channels
 *     endpoints
 *     transport
 *     discovery
 *     distributed execution
 *     consistency
 *     recovery
 *
 * No universal node/network limit is introduced.
 *
 *
 * ============================================================================
 * 17. EFFECT INTEGRATION
 * ============================================================================
 *
 * Policy does not redefine effects.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * Policy may govern effects through the existing policy/effect adapters.
 *
 * This permits governance of:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native calls
 *     foreign calls
 *     distributed behavior
 *     quantum measurement
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 *
 * ============================================================================
 * 18. CONTRACT INTEGRATION
 * ============================================================================
 *
 * Policy does not redefine:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract semantics remain owned by:
 *
 *     grammar/validation/
 *
 * Policy may reference or govern contracts through its dedicated adapter.
 *
 *
 * ============================================================================
 * 19. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Every policy declaration and constitutional declaration must remain
 * traceable to its source location.
 *
 * Downstream semantic processing may record:
 *
 *     source
 *     policy identity
 *     constitutional identity
 *     inherited policy
 *     applied policy
 *     conflict
 *     resolution
 *     evidence
 *     decision
 *     transformation
 *     version
 *
 * Provenance semantics remain outside this grammar.
 *
 *
 * ============================================================================
 * 20. PRECEDENCE INTEGRATION
 * ============================================================================
 *
 * This file does not encode numeric policy priority.
 *
 * Constitutional precedence, policy precedence, conflict resolution and
 * inheritance are semantic concerns.
 *
 * The grammar merely makes the relevant declarations reachable.
 *
 * This avoids creating artificial finite precedence ranges.
 *
 *
 * ============================================================================
 * 21. CIRCULAR-DEPENDENCY PROHIBITION
 * ============================================================================
 *
 * The dependency direction MUST remain:
 *
 *     Policies
 *       |
 *       +--> Policy / Constitution / policy leaves
 *       |
 *       v
 *     semantic policy model
 *
 * No leaf grammar may import Policies.
 *
 * No security grammar may import Policies merely to obtain policy syntax.
 *
 * No resource grammar may import Policies merely to obtain policy syntax.
 *
 * No execution grammar may import Policies merely to obtain policy syntax.
 *
 * Domain consumers consume semantic policy nodes after parsing.
 *
 *
 * ============================================================================
 * 22. AST CONTRACT
 * ============================================================================
 *
 * This grammar does not define Rust AST structs.
 *
 * It guarantees stable parser boundaries:
 *
 *     policyElement
 *         |
 *         +--> policyDeclaration
 *         +--> constitutionDeclaration
 *
 * AST construction belongs to the canonical AST/frontend implementation.
 *
 * Policy AST nodes must remain domain-neutral.
 *
 * They MUST NOT directly encode:
 *
 *     LLVM
 *     MLIR
 *     QIR
 *     physical qubit maps
 *     vendor topology
 *     calibration
 *     QEC implementation
 *     scheduling implementation
 *     backend-specific device IDs
 *
 *
 * ============================================================================
 * 23. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis consumes policy nodes and resolves:
 *
 *     scope
 *     requirements
 *     constraints
 *     capabilities
 *     resources
 *     permissions
 *     prohibitions
 *     preferences
 *     fallbacks
 *     adaptation governance
 *     execution governance
 *     security governance
 *     deployment governance
 *     simulation governance
 *     provenance governance
 *
 * Semantic analysis is responsible for determining whether those constructs
 * are:
 *
 *     valid
 *     satisfiable
 *     compatible
 *     authorized
 *     available
 *     applicable
 *     conflicting
 *
 * Parsing alone MUST NOT make those claims.
 *
 *
 * ============================================================================
 * 24. RESOURCE CONTRACT
 * ============================================================================
 *
 * No finite resource model is embedded here.
 *
 * Resource availability is determined downstream.
 *
 * A source policy may therefore remain unchanged while the semantic planner
 * considers targets with different available resources.
 *
 * Failure of a particular target must be represented as a feasibility or
 * capability result, not as a grammar limitation.
 *
 *
 * ============================================================================
 * 25. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST be deterministic for the same:
 *
 *     source text
 *     lexer configuration
 *     parser grammar version
 *     dialect configuration
 *
 * This grammar performs:
 *
 *     no runtime selection
 *     no hardware discovery
 *     no network access
 *     no filesystem access
 *     no random choice
 *     no environment inspection
 *
 *
 * ============================================================================
 * 26. SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded target-language actions.
 *
 * It requires no unsafe Rust.
 *
 * Rust integration MUST remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021+
 *
 * Safety is an implementation concern of the generated-parser integration;
 * this grammar itself performs no unsafe operation.
 *
 *
 * ============================================================================
 * 27. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * At minimum, the parser composition must accept:
 *
 *     policy example {
 *         requires capability("compute");
 *     }
 *
 *     policy execution {
 *         prefer execution::deterministic;
 *     }
 *
 *     policy quantum {
 *         requires capability("quantum.measurement");
 *     }
 *
 *     policy hardware {
 *         requires capability("accelerator.compute");
 *     }
 *
 *     policy ai {
 *         permit reasoning::infer;
 *     }
 *
 *     constitution core {
 *         invariant program::meaning;
 *     }
 *
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Policy composition must support semantic references to:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     interoperability
 *     simulation
 *     deployment
 *     future domains
 *
 *
 * NEGATIVE
 * --------
 *
 * The policy orchestrator must reject structurally invalid source such as:
 *
 *     policy;
 *
 *     constitution;
 *
 *     policy {}
 *     // only if policy.g4 requires a valid policy identity/body
 *
 *     unknown_policy_fragment;
 *
 * when such fragments are not legal top-level Zamani source elements.
 *
 *
 * BOUNDARY
 * --------
 *
 * Test:
 *
 *     multiple policies
 *     multiple constitutions where semantic rules permit them
 *     nested policy expressions
 *     deeply qualified semantic names
 *     large policy bodies
 *     many requirements
 *     many constraints
 *     many permissions
 *     many prohibitions
 *     many preferences
 *     many fallbacks
 *     cross-domain policy references
 *     policy + quantum
 *     policy + HDL
 *     policy + AI
 *     policy + distributed
 *     policy + security
 *
 *
 * SCALABILITY
 * ----------
 *
 * The grammar introduces no artificial limits on:
 *
 *     number of policies
 *     number of constitutions
 *     policy-member count
 *     scope count
 *     requirement count
 *     constraint count
 *     permission count
 *     resource count
 *     capability count
 *     policy nesting
 *     qualified-name depth
 *     target count
 *     device count
 *     node count
 *     qubit count
 *     CPU count
 *     GPU count
 *     memory capacity
 *
 * Practical parser/compiler limits may exist as configurable implementation
 * safeguards, but they are not language semantics and MUST NOT be represented
 * by grammar constants.
 *
 *
 * ============================================================================
 * 28. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new policy leaf should normally require:
 *
 *     1. create the leaf grammar;
 *     2. define its ownership contract;
 *     3. add its grammar name to this import list;
 *     4. connect it to the owning semantic policy model;
 *     5. add conformance tests.
 *
 * Existing policy syntax must not be silently reinterpreted merely because a
 * new policy leaf is added.
 *
 * Deprecated policy constructs must be handled through the compatibility
 * subsystem rather than by silently changing this orchestrator.
 *
 *
 * ============================================================================
 * 29. INTEGRATION WITH ZamaniParser.g4
 * ============================================================================
 *
 * This is the ONLY direct universal-parser integration required.
 *
 * ZamaniParser.g4 MUST import:
 *
 *     Policies
 *
 * alongside the other domain composition grammars.
 *
 * The universal parser should then expose:
 *
 *     policyElement
 *
 * through source-unit composition.
 *
 * Recommended structure:
 *
 *     import
 *         Core,
 *         Types,
 *         Expressions,
 *         ...
 *         Policies
 *     ;
 *
 *
 *     sourceElement
 *         : documentationElement
 *         | attributeElement
 *         | pragmaElement
 *         | packageElement
 *         | moduleElement
 *         | importElement
 *         | exportElement
 *         | declarationElement
 *         | statementElement
 *         | policyElement
 *         | domainElement
 *         ;
 *
 *
 * IMPORTANT:
 *
 * Zamani.g4 itself should NOT import Policies directly.
 *
 * Its existing architecture is correct:
 *
 *     Zamani.g4
 *         |
 *         +--> ZamaniParser
 *         +--> ZamaniLexer
 *
 *
 * ============================================================================
 * 30. INTEGRATION WITH policy.g4
 * ============================================================================
 *
 * policy.g4 currently contains several local implementations that overlap
 * with the dedicated policy leaf grammars.
 *
 * To make this orchestrator genuinely authoritative, policy.g4 MUST be
 * reduced to policy declaration/body/member composition.
 *
 * Its import section should consume the specialized policy grammars required
 * by its `policyMember` alternatives.
 *
 * In particular, the local implementations of specialized constructs must
 * not remain competing authorities where a dedicated leaf already exists.
 *
 * Examples include:
 *
 *     policyPermission
 *     policyRequirement
 *     policyConstraint
 *     policySimulation
 *     policyAdaptation
 *     policyProvenance
 *
 * where their dedicated leaf grammar already owns the corresponding syntax.
 *
 * The resulting ownership should be:
 *
 *     Policies
 *       |
 *       v
 *     Policy
 *       |
 *       +--> specialized policy leaves
 *
 *
 * ============================================================================
 * 31. INTEGRATION WITH CONSTITUTION
 * ============================================================================
 *
 * constitution.g4 remains the sole authority for:
 *
 *     constitutionDeclaration
 *     constitutionBody
 *     constitutionMember
 *
 * Policies merely composes it.
 *
 * No constitutional syntax is duplicated here.
 *
 *
 * ============================================================================
 * 32. INTEGRATION WITH SECURITY
 * ============================================================================
 *
 * Policy syntax and security authorization remain separate.
 *
 * The relationship is:
 *
 *     policy permission/prohibition
 *              |
 *              v
 *     semantic policy model
 *              |
 *              v
 *     security authorization
 *
 * `Policies` MUST NOT import security grammar merely to reinterpret security
 * authorization as policy syntax.
 *
 *
 * ============================================================================
 * 33. INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Policy resource requirements and capability references ultimately resolve
 * through the universal resource/capability model.
 *
 * The policy grammar never discovers or allocates hardware.
 *
 *
 * ============================================================================
 * 34. INTEGRATION WITH EXECUTION
 * ============================================================================
 *
 * Policy execution syntax is delegated to PolicyExecution.
 *
 * Runtime execution remains owned by:
 *
 *     grammar/execution/
 *
 * Policy governance and executable behavior therefore remain distinct.
 *
 *
 * ============================================================================
 * 35. INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * No direct import of quantum leaf grammars belongs here.
 *
 * Quantum consumers receive policy semantics after universal parsing and
 * semantic analysis.
 *
 * This prevents policies from becoming a quantum-specific grammar.
 *
 *
 * ============================================================================
 * 36. INTEGRATION WITH AI
 * ============================================================================
 *
 * No direct import of AI leaf grammars belongs here.
 *
 * AI policy intent is represented through generic policy semantics and the
 * existing policy adapters.
 *
 * This keeps the policy language universal.
 *
 *
 * ============================================================================
 * 37. INTEGRATION WITH HDL/HARDWARE
 * ============================================================================
 *
 * No physical hardware grammar belongs here.
 *
 * Policy references hardware through:
 *
 *     capability
 *     requirement
 *     resource
 *     constraint
 *     preference
 *     policy expression
 *
 *
 * ============================================================================
 * 38. INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Every policy and constitutional AST node must preserve source provenance.
 *
 * The provenance subsystem may subsequently record:
 *
 *     source span
 *     declaration identity
 *     inheritance
 *     amendment
 *     resolution
 *     conflict
 *     evidence
 *     decision
 *     semantic transformation
 *
 *
 * ============================================================================
 * 39. INTEGRATION WITH COMPILATION
 * ============================================================================
 *
 * Policies participate in target-independent compilation.
 *
 * They may affect:
 *
 *     specialization
 *     optimization
 *     resource planning
 *     target negotiation
 *     execution planning
 *     deployment
 *     resilience
 *
 * They must not directly select a backend in the grammar.
 *
 *
 * ============================================================================
 * 40. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It is the sole composition authority for grammar/policies/.
 *
 *     [x] It owns only composition rules.
 *
 *     [x] It introduces no duplicate policy syntax.
 *
 *     [x] It introduces no lexer rules.
 *
 *     [x] It introduces no hardware limits.
 *
 *     [x] It introduces no quantum gate catalogue.
 *
 *     [x] It introduces no AI algorithm catalogue.
 *
 *     [x] It introduces no target-selection implementation.
 *
 *     [x] It introduces no runtime behavior.
 *
 *     [x] It introduces no unsafe Rust.
 *
 *     [x] It is open-world.
 *
 *     [x] It provides a stable `policySystem` entry point.
 *
 *     [x] It provides a stable `policyElement` integration rule.
 *
 *     [x] It composes ordinary policies and constitutions.
 *
 *     [x] It composes all policy-domain leaf grammars.
 *
 *     [ ] policy.g4 has been reduced to its proper composition/member role.
 *
 *     [ ] ZamaniParser.g4 imports Policies.
 *
 *     [ ] ZamaniParser.g4 exposes policyElement from sourceElement.
 *
 *     [ ] ANTLR generation succeeds.
 *
 *     [ ] Generated Rust parser compiles on Rust 1.97+.
 *
 *     [ ] Positive conformance tests pass.
 *
 *     [ ] Negative conformance tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *
 * ============================================================================
 * FINAL OWNERSHIP RULE
 * ============================================================================
 *
 * The resulting architecture is:
 *
 *     grammar/Zamani.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             v
 *     grammar/policies/policies.g4
 *             |
 *       +-----+------------------------------+
 *       |                                    |
 *       v                                    v
 *    policy.g4                         constitution.g4
 *       |                                    |
 *       +----------------+-------------------+
 *                        |
 *                        v
 *                  policy leaf grammars
 *                        |
 *                        v
 *                 semantic policy model
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *       resources     effects      contracts
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                semantic realization
 *                        |
 *             +----------+----------+
 *             |                     |
 *             v                     v
 *       classical IR            quantum::ir
 *             |                     |
 *             +----------+----------+
 *                        |
 *                 target-independent
 *                    optimization
 *                        |
 *                lowering/routing/
 *                  scheduling/
 *                    resilience
 *                        |
 *                     ZQN/HAL
 *                        |
 *                    realization
 *
 *
 * This is the required composition boundary for POCO-REAF policy governance.
 *
 * ============================================================================
 */