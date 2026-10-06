/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/policies.g4
 *
 * GRAMMAR
 * -------
 * AIPolicies
 *
 * STATUS
 * ------
 * CANONICAL AI-DOMAIN POLICY COMPOSITION BOUNDARY
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97 or later
 * Rust edition 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the AI-domain composition boundary for policy semantics.
 *
 * It does NOT create a second policy language.
 *
 * It connects AI-domain semantic consumers to Zamani's canonical universal
 * policy expression grammar.
 *
 * AI policies may govern or constrain:
 *
 *     inference
 *     reasoning
 *     deduction
 *     induction
 *     abduction
 *     knowledge operations
 *     learning
 *     adaptation
 *     feedback
 *     uncertainty
 *     model execution
 *     agents
 *     multi-agent computation
 *     simulation
 *     reproducibility
 *     provenance
 *     evidence
 *     explainability
 *     resource requirements
 *     capability requirements
 *     effects
 *     contracts
 *     security
 *     interoperability
 *     classical execution
 *     quantum execution
 *     hybrid execution
 *     distributed execution
 *     accelerator execution
 *     future computational domains
 *
 * The policy grammar remains universal and open-world.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                           Zamani source
 *                                |
 *                                v
 *                              lexer
 *                                |
 *                                v
 *                              parser
 *                                |
 *              +-----------------+------------------+
 *              |                                    |
 *              v                                    v
 *      universal policy syntax                 AI domain
 *              |                                    |
 *              v                                    v
 *      PolicyExpressions                       AIPolicies
 *              |                                    |
 *              +----------------+-------------------+
 *                               |
 *                               v
 *                    domain-neutral frontend AST
 *                               |
 *                               v
 *                       structural validation
 *                               |
 *          +--------------------+---------------------+
 *          |          |         |         |           |
 *          v          v         v         v           v
 *        types     effects  resources  contracts  provenance
 *          |          |         |         |           |
 *          +----------+---------+---------+-----------+
 *                               |
 *                               v
 *                       semantic policy model
 *                               |
 *          +--------------------+----------------------+
 *          |                    |                      |
 *          v                    v                      v
 *       classical          quantum::ir          HDL/hardware
 *          |                    |                      |
 *          +--------------------+----------------------+
 *                               |
 *                         optimization
 *                               |
 *                       lowering / planning
 *                               |
 *                     routing / scheduling
 *                               |
 *                         resilience
 *                               |
 *                           ZQN / HAL
 *                               |
 *                        target realization
 *
 * This file defines NONE of the downstream realization mechanisms.
 *
 * ============================================================================
 * SINGLE-AUTHORITY POLICY RULE
 * ============================================================================
 *
 * The canonical universal policy expression grammar is:
 *
 *     grammar/expressions/policy.g4
 *
 * Grammar:
 *
 *     PolicyExpressions
 *
 * Public entry point:
 *
 *     policyExpression
 *
 * This file MUST consume that grammar.
 *
 * It MUST NOT redefine:
 *
 *     policyExpression
 *     policyBody
 *     policyClause
 *     policyDirective
 *     policyDirectiveName
 *     policyDirectiveArguments
 *     policyCondition
 *     policyScope
 *     policyValue
 *
 * This is critical because policy syntax is language-wide.
 *
 * AI is a consumer of policy semantics, not the owner of a second policy
 * syntax.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY the AI-domain parser boundary:
 *
 *     aiPolicyConstruct
 *
 * The rule identifies a canonical policy expression as an AI-domain policy
 * integration point.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer tokens
 *     keywords
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     statements
 *     generic policies
 *     security policies
 *     authorization
 *     permissions
 *     prohibitions
 *     requirements
 *     constraints
 *     capabilities
 *     resources
 *     effects
 *     contracts
 *     provenance
 *     evidence
 *     reasoning
 *     learning
 *     adaptation
 *     agents
 *     actors
 *     concurrency
 *     model declarations
 *     tensor declarations
 *     dataset declarations
 *     inference syntax
 *     quantum operations
 *     HDL syntax
 *     hardware topology
 *     physical devices
 *     target selection
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime enforcement
 *     policy conflict resolution
 *     policy evaluation algorithms
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/expressions/policy.g4
 *         PolicyExpressions
 *
 * INDIRECTLY CONSUMED THROUGH THE CANONICAL POLICY MODEL:
 *
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *     grammar/resources/
 *     grammar/effects/
 *     grammar/validation/
 *     grammar/provenance/
 *     grammar/security/
 *     grammar/execution/
 *     grammar/concurrency/
 *
 * No physical target grammar is a dependency of this file.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public parser rule:
 *
 *     aiPolicyConstruct
 *
 * The rule is intentionally small because the actual policy syntax has one
 * canonical owner.
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary semantic consumers:
 *
 *     AI semantic analysis
 *     AI policy validation
 *     AI execution planning
 *     AI adaptation analysis
 *     AI agent analysis
 *     AI inference analysis
 *     AI learning analysis
 *     AI provenance analysis
 *
 * The universal policy grammar remains reachable independently through:
 *
 *     grammar/expressions/expressions.g4
 *
 * ============================================================================
 * AST OWNER
 * ============================================================================
 *
 * This grammar owns NO Rust AST structure.
 *
 * The domain-neutral frontend AST owns the actual policy representation.
 *
 * The AST should preserve, where applicable:
 *
 *     policy identity
 *     selector
 *     directives
 *     directive arguments
 *     bindings
 *     conditions
 *     source span
 *     source order
 *     lexical provenance
 *     enclosing AI construct
 *
 * The AST MUST NOT contain:
 *
 *     GPU identifiers
 *     CPU identifiers
 *     QPU identifiers
 *     physical qubit identifiers
 *     physical topology
 *     scheduler state
 *     routing state
 *     calibration state
 *     vendor implementation details
 *
 * ============================================================================
 * SEMANTIC OWNER
 * ============================================================================
 *
 * AI policy semantics belong to the semantic policy subsystem.
 *
 * This grammar only establishes the parser boundary.
 *
 * Semantic analysis determines:
 *
 *     whether a policy applies to an AI construct;
 *     whether its directives are known;
 *     whether directive arguments are valid;
 *     whether the policy conflicts with another policy;
 *     whether a policy is authorized;
 *     whether requirements can be satisfied;
 *     whether capabilities are available;
 *     whether effects are permitted;
 *     whether contracts remain satisfiable;
 *     whether adaptation is permitted;
 *     whether an AI agent may perform an operation;
 *     whether learning may occur;
 *     whether external resources may be used;
 *     whether provenance is required;
 *     whether reproducibility requirements can be met.
 *
 * Parser acceptance MUST NOT be treated as policy authorization.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define an AI-specific policy IR.
 *
 * The semantic policy model is the canonical intermediate representation
 * boundary for policy meaning.
 *
 * Policy semantics may subsequently influence:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic models
 *     distributed execution plans
 *     networking plans
 *     security authorization
 *     runtime execution plans
 *
 * Policy information MUST remain semantically attached to the relevant
 * operation, region, declaration, execution plan, or deployment intent.
 *
 * This file MUST NOT create:
 *
 *     AIPolicyIR
 *     NeuralPolicyIR
 *     AgentPolicyIR
 *     QuantumPolicyIR
 *
 * merely because a policy is consumed by an AI subsystem.
 *
 * ============================================================================
 * AI POLICY MODEL
 * ============================================================================
 *
 * AI policies may semantically govern:
 *
 *     inference
 *     reasoning
 *     learning
 *     adaptation
 *     knowledge
 *     uncertainty
 *     evidence
 *     explanation
 *     decisions
 *     agents
 *     model execution
 *     data access
 *     simulation
 *     reproducibility
 *     resource selection
 *     capability requirements
 *     effect permissions
 *     security
 *     provenance
 *
 * The grammar deliberately does NOT enumerate these as policy keywords.
 *
 * Instead, policy directives remain open-world identifiers or qualified names
 * through the canonical PolicyExpressions grammar.
 *
 * Examples of possible semantic policy directives include:
 *
 *     learning.require
 *     learning.forbid
 *     inference.prefer
 *     inference.constrain
 *     adaptation.allow
 *     agent.authorize
 *     provenance.require
 *     explanation.require
 *     evidence.require
 *     simulation.prefer
 *     quantum.fallback
 *     resource.require
 *     execution.retry
 *
 * New policy namespaces MUST NOT require modification of this grammar.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * The grammar MUST remain open-world.
 *
 * It MUST NOT enumerate application-specific AI concepts such as:
 *
 *     image recognition
 *     sentiment analysis
 *     robotics
 *     payment processing
 *     legal processing
 *     medical processing
 *     administration
 *     blockchain
 *     virtual reality
 *     augmented reality
 *     individual vendor models
 *     individual model architectures
 *
 * Such concepts belong to:
 *
 *     libraries
 *     dialects
 *     schemas
 *     capabilities
 *     semantic registries
 *     application code
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer tokens.
 *
 * It consumes the public vocabulary exported by:
 *
 *     ZamaniLexer
 *
 * through the imported PolicyExpressions grammar.
 *
 * No AI-specific policy keyword is required merely because a new AI feature
 * is introduced.
 *
 * Policy directives remain extensible through identifiers and qualified
 * identifiers.
 *
 * This prevents a permanent lexer-keyword catalogue from growing with every
 * future policy mechanism.
 *
 * ============================================================================
 * AI POLICY ENTRY POINT
 * ============================================================================
 *
 * The sole public rule is:
 *
 *     aiPolicyConstruct
 *
 * It delegates directly to:
 *
 *     policyExpression
 *
 * This means:
 *
 *     AI policy syntax
 *          |
 *          v
 *     canonical policy expression
 *          |
 *          v
 *     universal policy semantics
 *
 * There is deliberately no second policy body or directive grammar here.
 * ============================================================================
 */

parser grammar AIPolicies;

options {
    tokenVocab = ZamaniLexer;
}

import PolicyExpressions;


/*
 * ============================================================================
 * PUBLIC AI POLICY BOUNDARY
 * ============================================================================
 *
 * An AI policy is a canonical Zamani policy expression consumed in an AI
 * semantic context.
 *
 * The surrounding AI semantic model determines what AI construct the policy
 * governs.
 *
 * Examples of semantic consumers include:
 *
 *     model
 *     inference
 *     learning
 *     adaptation
 *     agent
 *     reasoning
 *     pipeline
 *     simulation
 *     deployment
 *
 * The parser does not encode those relationships because doing so would
 * duplicate the ownership of the corresponding AI grammars.
 */
aiPolicyConstruct
    : policyExpression
    ;


/*
 * ============================================================================
 * AI POLICY REFERENCE BOUNDARY
 * ============================================================================
 *
 * This rule is intentionally an alias to the canonical policy construct.
 *
 * It exists so future AI semantic/composition grammars can depend on a named
 * AI-policy boundary without importing or depending directly on the complete
 * generic policy implementation.
 *
 * It does NOT create another policy syntax.
 */
aiPolicyReference
    : aiPolicyConstruct
    ;


/*
 * ============================================================================
 * SEMANTIC CONTEXT CONTRACT
 * ============================================================================
 *
 * The same policy syntax may be consumed by multiple AI constructs.
 *
 * Examples:
 *
 *     policy {
 *         learning.require;
 *     }
 *
 *     policy {
 *         inference.prefer;
 *     }
 *
 *     policy {
 *         adaptation.forbid;
 *     }
 *
 * The semantic layer determines the enclosing construct and validates whether
 * the policy directive is applicable.
 *
 * The parser does not infer applicability.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Reasoning policies may govern:
 *
 *     inference strategy
 *     evidence requirements
 *     confidence requirements
 *     explanation requirements
 *     reproducibility
 *     provenance
 *     permitted knowledge sources
 *     resource usage
 *     fallback behavior
 *
 * Reasoning syntax remains owned by the reasoning grammars.
 *
 * This file only supplies the policy boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning policies may govern:
 *
 *     training permission
 *     model update permission
 *     data-source requirements
 *     evidence requirements
 *     resource requirements
 *     capability requirements
 *     reproducibility
 *     provenance
 *     adaptation
 *     external service use
 *     simulation
 *     validation
 *
 * Learning syntax remains owned by the learning subsystem.
 *
 * No learning algorithm is enumerated here.
 *
 * Therefore future algorithms do not require grammar changes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Adaptation is a controlled semantic operation.
 *
 * A policy may permit, prohibit, constrain, require, or otherwise govern
 * adaptation.
 *
 * The semantic pipeline is:
 *
 *     adaptation request
 *          |
 *          v
 *     policy evaluation
 *          |
 *          v
 *     authorization
 *          |
 *          v
 *     capability/resource/effect validation
 *          |
 *          v
 *     provenance
 *          |
 *          v
 *     adaptation realization
 *
 * This grammar does NOT permit adaptation to bypass policy merely because
 * the source parses.
 *
 * The adaptation operation itself remains owned by its existing grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AGENT INTEGRATION
 * ============================================================================
 *
 * AI agents are semantically governed by AI policies, but actor syntax and
 * lifecycle remain owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * Agent syntax remains owned by:
 *
 *     grammar/ai/agents.g4
 *
 * This file MUST NOT create:
 *
 *     agentPolicyActor
 *     policyActor
 *     policyAgent
 *
 * or another actor model.
 *
 * The semantic relationship is:
 *
 *     AI agent
 *          |
 *          v
 *     actor/concurrency model
 *          |
 *          v
 *     policy evaluation
 *          |
 *          v
 *     permitted execution
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * AI policies may semantically consume universal requirements and capabilities.
 *
 * Examples:
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("quantum.measurement");
 *
 *     requires memory >= required_memory;
 *
 *     requires qubits >= required_qubits;
 *
 *     requires topology(required_topology);
 *
 * These are semantic requirements.
 *
 * This grammar does not define physical resources.
 *
 * It does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     node
 *     memory bank
 *     network path
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Policies may constrain operations carrying effects such as:
 *
 *     io
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     quantum
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * Effect ownership remains in:
 *
 *     grammar/effects/
 *
 * This file MUST NOT create an AI-specific effect system.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * AI policies may interact semantically with:
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
 *     grammar/statements/contract.g4
 *
 * and its established validation/function/distributed integration points.
 *
 * This file MUST NOT redefine contract syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Policy decisions affecting AI computation may require provenance.
 *
 * Relevant information can include:
 *
 *     policy identity
 *     directive identity
 *     policy scope
 *     policy source span
 *     policy version
 *     decision
 *     decision reason
 *     evidence
 *     verification
 *     applicable capabilities
 *     applicable resources
 *     transformations
 *
 * Provenance ownership remains with the repository's provenance subsystem.
 *
 * Policy syntax must not create a second provenance model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * EVIDENCE / EXPLANATION INTEGRATION
 * ============================================================================
 *
 * AI policies may require:
 *
 *     evidence
 *     confidence
 *     explanation
 *     decision records
 *     provenance
 *     verification
 *
 * These are semantic consumers.
 *
 * The grammar does not define:
 *
 *     evidence algorithms
 *     confidence algorithms
 *     explanation algorithms
 *     proof systems
 *     attribution algorithms
 *
 * Such mechanisms remain open-world and implementation-independent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Policies may constrain or prefer uncertainty handling.
 *
 * They may semantically govern:
 *
 *     confidence
 *     probability
 *     distributions
 *     belief
 *     uncertainty propagation
 *     decision thresholds
 *     evidence requirements
 *
 * This file does not define a particular probabilistic model.
 *
 * No finite probability/distribution vocabulary is embedded here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * AI policies may govern simulation as an execution strategy.
 *
 * Simulation may concern:
 *
 *     classical computation
 *     AI computation
 *     quantum computation
 *     hybrid computation
 *     hardware
 *     distributed execution
 *     fault scenarios
 *     performance
 *
 * Simulation grammar and execution semantics remain owned elsewhere.
 *
 * This file only permits policy semantics to govern them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * AI policies may affect hybrid quantum/classical execution.
 *
 * The quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT introduce:
 *
 *     quantum operation syntax
 *     physical qubit identifiers
 *     physical topology
 *     coupling maps
 *     calibration data
 *     routing decisions
 *     scheduling decisions
 *     QEC implementation
 *     ZQN representation
 *     HAL representation
 *
 * The policy path is:
 *
 *     AI policy
 *          |
 *          v
 *     semantic policy model
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     downstream realization
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * AI policies may govern hardware/software co-design or hardware-backed AI.
 *
 * They may semantically constrain:
 *
 *     capabilities
 *     resources
 *     timing intent
 *     simulation
 *     verification
 *     synthesis intent
 *     deployment
 *     reliability
 *     power/thermal policies
 *
 * The grammar does not define hardware topology or physical allocation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DISTRIBUTED / NETWORKING BOUNDARY
 * ============================================================================
 *
 * AI policies may govern:
 *
 *     distributed agents
 *     message exchange
 *     network access
 *     distributed learning
 *     distributed inference
 *     data locality
 *     service selection
 *     retry/recovery
 *     consistency requirements
 *
 * Existing concurrency, distributed, and networking grammars remain the
 * owners of their respective source syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * AI policy syntax MUST remain distinct from security enforcement.
 *
 * Security policy ownership remains with:
 *
 *     grammar/security/policies.g4
 *
 * Authorization semantics remain downstream.
 *
 * AI policies may consume security-related policy decisions, but this grammar
 * does not redefine:
 *
 *     identities
 *     credentials
 *     roles
 *     authorization
 *     cryptographic mechanisms
 *     trust
 *     audit enforcement
 *
 * The semantic relationship is:
 *
 *     AI policy
 *          |
 *          v
 *     universal policy model
 *          |
 *          v
 *     security authorization
 *          |
 *          v
 *     execution decision
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * METAPROGRAMMING / REFLECTION BOUNDARY
 * ============================================================================
 *
 * Policies may govern:
 *
 *     reflection
 *     introspection
 *     compile-time execution
 *     code generation
 *     dynamic adaptation
 *
 * Such operations remain owned by:
 *
 *     grammar/metaprogramming/
 *     grammar/macros/
 *     grammar/effects/
 *
 * Policy evaluation remains semantic.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO target-specific assumptions.
 *
 * An AI policy must remain expressible regardless of whether its eventual
 * realization uses:
 *
 *     tiny hardware
 *     embedded systems
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed infrastructure
 *     cloud infrastructure
 *     future computational hardware.
 *
 * The source policy describes intent, constraints, requirements, preferences,
 * permissions, prohibitions, and other semantic policy information.
 *
 * The compiler/runtime determines whether a realization is feasible.
 *
 * A policy MUST NOT force the source program to name a physical implementation
 * merely to achieve execution.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level limits for:
 *
 *     policies
 *     policy clauses
 *     directives
 *     directive arguments
 *     policy nesting
 *     policy composition
 *     AI agents
 *     models
 *     datasets
 *     inference operations
 *     learning operations
 *     adaptation operations
 *     resources
 *     capabilities
 *     devices
 *     processors
 *     accelerators
 *     QPUs
 *     nodes
 *     threads
 *     tensor rank
 *     memory capacity
 *     network size
 *     topology size
 *
 * This file introduces no finite cardinality constants.
 *
 * Practical limits are implementation/resource-policy concerns.
 *
 * "Unbounded" means that the language grammar does not impose an artificial
 * semantic ceiling. It does not claim that finite hardware has infinite
 * resources.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains NO:
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
 * or equivalent fixed-capacity mechanisms.
 *
 * It also contains no:
 *
 *     vendor identifiers
 *     physical device identifiers
 *     processor-specific policy syntax
 *     fixed topology
 *     fixed accelerator count
 *     fixed model count
 *     fixed agent count
 *     fixed policy count
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     lexer configuration
 *     parser grammar
 *     parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     runtime state
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *     scheduler state
 *     resource availability
 *     target selection.
 *
 * Identical source under identical language/parser configuration must produce
 * equivalent parser structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions;
 *     no embedded Rust predicates;
 *     no unsafe code;
 *     no filesystem operations;
 *     no network operations;
 *     no environment inspection;
 *     no hardware access;
 *     no runtime execution;
 *     no dynamic evaluation.
 *
 * The generated Rust parser is consumed by the safe Rust frontend.
 *
 * Rust 1.97 or later is therefore an implementation baseline, not a grammar
 * dependency.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics are inherited from the canonical policy grammar.
 *
 * This file must not introduce a second diagnostic vocabulary.
 *
 * Semantic diagnostics may include:
 *
 *     AI policy not applicable to enclosing construct
 *     unknown policy directive
 *     invalid policy directive arguments
 *     conflicting policies
 *     unsatisfied requirement
 *     unavailable capability
 *     forbidden effect
 *     unauthorized adaptation
 *     unauthorized model access
 *     invalid agent policy
 *     invalid resource policy
 *     invalid provenance requirement
 *     invalid evidence requirement
 *     invalid execution policy
 *
 * These diagnostics belong to semantic validation rather than ANTLR parsing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * The canonical policy syntax should be accepted through:
 *
 *     aiPolicyConstruct
 *
 * Examples include semantically AI-oriented policy directives such as:
 *
 *     policy {
 *         learning.require;
 *     }
 *
 *     policy {
 *         inference.prefer;
 *     }
 *
 *     policy {
 *         adaptation.forbid;
 *     }
 *
 *     policy {
 *         agent.authorize;
 *     }
 *
 *     policy {
 *         provenance.require;
 *     }
 *
 *     policy {
 *         explanation.require;
 *     }
 *
 *     policy {
 *         resource.require(capability("tensor.compute"));
 *     }
 *
 *     policy {
 *         execution.retry;
 *     }
 *
 * These examples rely on the canonical policy expression grammar.
 *
 * NEGATIVE
 * --------
 *
 * Invalid canonical policy syntax must remain invalid when entered through
 * this AI boundary.
 *
 * Examples:
 *
 *     policy
 *
 *     policy {
 *
 *     policy {
 *         ;
 *     }
 *
 * malformed directive syntax
 *
 * malformed argument syntax
 *
 * The AI adapter MUST NOT make invalid generic policy syntax valid.
 *
 * BOUNDARY
 * --------
 *
 * Test policies that govern:
 *
 *     inference
 *     learning
 *     adaptation
 *     agents
 *     reasoning
 *     provenance
 *     evidence
 *     uncertainty
 *     simulation
 *     quantum/classical execution
 *     distributed execution
 *     hardware capability requirements
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Test policy consumption with:
 *
 *     classical computation
 *     AI computation
 *     quantum computation
 *     hybrid computation
 *     HDL/hardware intent
 *     distributed computation
 *     networking
 *     simulation
 *
 * SCALABILITY
 * -----------
 *
 * Test:
 *
 *     arbitrarily many policy clauses
 *     arbitrarily many directives
 *     arbitrarily large directive argument expressions
 *     deeply qualified directive names
 *     policy composition
 *     nested semantic contexts
 *
 * without introducing language-level finite limits.
 *
 * DETERMINISM
 * -----------
 *
 * Repeated parsing of identical source under identical parser configuration
 * must produce equivalent parse structure and source locations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces no new lexer token.
 *
 * Existing policy syntax remains owned by PolicyExpressions.
 *
 * Existing security policy syntax remains owned by:
 *
 *     grammar/security/policies.g4
 *
 * Existing policy statements remain owned by:
 *
 *     grammar/statements/policy.g4
 *
 * Therefore adding this AI adapter must not change the meaning of existing
 * policy source.
 *
 * Future policy directives may be introduced through the canonical policy
 * subsystem without changing this file, provided they remain representable by
 * PolicyExpressions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. UNIVERSAL POLICY SYNTAX
 *
 *     grammar/expressions/policy.g4
 *
 *     PolicyExpressions
 *            |
 *            +--> policyExpression
 *            |
 *            v
 *     AIPolicies
 *            |
 *            +--> aiPolicyConstruct
 *
 *
 * 2. AI COMPOSITION
 *
 *     grammar/ai/ai.g4
 *
 * already imports:
 *
 *     Expressions
 *
 * Expressions already composes:
 *
 *     PolicyExpressions
 *
 * Therefore ai.g4 MUST NOT also import AIPolicies merely to make policy
 * syntax reachable.
 *
 * Doing so would create a redundant grammar dependency and potentially
 * duplicate the policy composition path.
 *
 * The canonical production path remains:
 *
 *     AI
 *       |
 *       v
 *     Expressions
 *       |
 *       v
 *     PolicyExpressions
 *       |
 *       v
 *     policyExpression
 *
 * AIPolicies is an explicit AI-domain adapter for:
 *
 *     isolated AI grammar tests
 *     AI semantic composition
 *     AI conformance tooling
 *     AI policy analysis
 *     future AI leaf grammars that need a named policy boundary
 *
 *
 * 3. AI AGENTS
 *
 *     grammar/ai/agents.g4
 *
 * owns agent syntax.
 *
 *     grammar/concurrency/actors.g4
 *
 * owns actor lifecycle and actor syntax.
 *
 * AIPolicies owns neither.
 *
 *
 * 4. RESOURCES
 *
 * Resource requirements and capabilities remain owned by:
 *
 *     grammar/resources/
 *
 * AIPolicies consumes them semantically.
 *
 *
 * 5. EFFECTS
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * AIPolicies consumes effect information semantically.
 *
 *
 * 6. CONTRACTS
 *
 * Contract syntax remains owned by:
 *
 *     grammar/statements/contract.g4
 *     grammar/functions/contracts.g4
 *     grammar/distributed/contracts.g4
 *
 * AIPolicies does not duplicate it.
 *
 *
 * 7. PROVENANCE
 *
 * Provenance remains owned by:
 *
 *     grammar/provenance/
 *
 * AIPolicies provides semantic policy information that provenance may record.
 *
 *
 * 8. SECURITY
 *
 * Security policy syntax remains owned by:
 *
 *     grammar/security/policies.g4
 *
 * AIPolicies does not replace or duplicate security policy syntax.
 *
 *
 * 9. EXECUTION
 *
 * AI execution policy is consumed by:
 *
 *     grammar/execution/
 *
 * Runtime selection, retry, recovery, fallback, scheduling, and deployment
 * remain downstream concerns.
 *
 *
 * 10. QUANTUM
 *
 * AI policy semantics may influence quantum execution, but the quantum
 * frontend remains:
 *
 *     quantum source
 *         ->
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * No quantum implementation details enter this grammar.
 *
 *
 * 11. HDL / HARDWARE
 *
 * AI policy semantics may influence hardware realization, but:
 *
 *     hardware capability
 *     topology
 *     placement
 *     timing
 *     synthesis
 *     routing
 *
 * remain outside this grammar.
 *
 *
 * 12. ROOT GRAMMAR
 *
 * No change to:
 *
 *     grammar/Zamani.g4
 *
 * is required solely to add this adapter because the root reaches AI and
 * expression syntax through the existing parser composition architecture.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SPECIFICATION CONTRACT
 * ============================================================================
 *
 * Normative policy semantics should be documented by:
 *
 *     grammar/spec/ai.md
 *
 * and the universal policy specification.
 *
 * AI policy integration should document:
 *
 *     applicable scopes
 *     policy inheritance
 *     policy precedence
 *     policy composition
 *     conflict handling
 *     authorization
 *     resource interaction
 *     capability interaction
 *     effect interaction
 *     contract interaction
 *     provenance requirements
 *     reproducibility requirements
 *     adaptation authorization
 *
 * These semantics must not be encoded as parser-specific behavior.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 * [x] It has exactly one AI-specific public policy boundary.
 *
 * [x] Generic policy syntax has one canonical owner.
 *
 * [x] No policy syntax is duplicated here.
 *
 * [x] No security policy syntax is duplicated here.
 *
 * [x] No agent or actor syntax is duplicated here.
 *
 * [x] No AI algorithm is hard-coded.
 *
 * [x] No hardware capability is hard-coded.
 *
 * [x] No machine capacity is hard-coded.
 *
 * [x] No quantum topology is hard-coded.
 *
 * [x] No target selection occurs here.
 *
 * [x] No Rust actions or unsafe code are embedded.
 *
 * [x] Policy directives remain open-world.
 *
 * [x] AI semantic consumers can reference the boundary.
 *
 * [x] Existing universal policy parsing remains unchanged.
 *
 * [x] The file remains compatible with Rust 1.97 or later generated-parser
 *     integration.
 *
 * [x] Positive, negative, boundary, scalability, cross-domain, and
 *     determinism tests are defined.
 *
 * [x] The policy boundary remains target-independent and POCO-REAF compliant.
 *
 * ============================================================================
 */