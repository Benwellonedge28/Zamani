/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/explanations.g4
 *
 * GRAMMAR
 * -------
 * AIExplanations
 *
 * STATUS
 * ------
 * CANONICAL AI EXPLANATION COMPOSITION ADAPTER
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021 edition
 * Safe Rust only
 * No unsafe Rust required
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
 * This file provides the AI-domain composition boundary for explanation
 * requests.
 *
 * It does NOT define a second explanation language.
 *
 * The canonical source-level explanation statement is already owned by:
 *
 *     grammar/statements/explain.g4
 *
 * whose parser grammar identity is:
 *
 *     Explain
 *
 * Therefore this file is deliberately a thin adapter:
 *
 *     Explain
 *         |
 *         v
 *     AIExplanations
 *         |
 *         v
 *     AI
 *
 * The purpose of this boundary is to allow AI computations to consume the
 * same universal explanation semantics used by:
 *
 *     classical computation
 *     quantum computation
 *     HDL
 *     hardware/software co-design
 *     distributed computation
 *     networking
 *     security
 *     resource planning
 *     compilation
 *     optimization
 *     execution
 *     simulation
 *     future computational domains
 *
 * without making explanation an AI-only feature.
 *
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Explanation is UNIVERSAL.
 *
 * An explanation may describe:
 *
 *     an AI result
 *     a learned-model prediction
 *     a reasoning conclusion
 *     a knowledge-derived result
 *     a decision
 *     a causal claim
 *     an uncertainty-bearing result
 *     a classical computation
 *     a quantum result
 *     a measurement
 *     a compiler transformation
 *     an optimization decision
 *     a resource decision
 *     a capability decision
 *     a routing decision
 *     a scheduling decision
 *     a resilience decision
 *     a hardware realization
 *     an HDL transformation
 *     a simulation result
 *     a distributed execution
 *     a networking result
 *     a security decision
 *     a generated artifact
 *     a future-domain computation
 *
 * Consequently this file MUST NOT encode an AI-specific explanation
 * implementation.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY the AI composition boundary:
 *
 *     aiExplanationConstruct
 *     aiExplanationStatement
 *
 * These rules are aliases/adapters around the universal:
 *
 *     explainStatement
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     explain syntax
 *     explanation target syntax
 *     explanation source syntax
 *     explanation context syntax
 *     expression syntax
 *     expression precedence
 *     identifiers
 *     names
 *     qualified names
 *     literals
 *     types
 *     patterns
 *     guards
 *     reasoning syntax
 *     inference syntax
 *     deduction syntax
 *     induction syntax
 *     abduction syntax
 *     knowledge syntax
 *     learning syntax
 *     adaptation syntax
 *     uncertainty syntax
 *     probability syntax
 *     causal syntax
 *     evidence syntax
 *     provenance syntax
 *     decision syntax
 *     policy syntax
 *     contract syntax
 *     effect syntax
 *     capability syntax
 *     resource syntax
 *     model syntax
 *     tensor syntax
 *     dataset syntax
 *     agent syntax
 *     concurrency syntax
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     distributed syntax
 *     networking syntax
 *     FFI syntax
 *     ABI syntax
 *     metaprogramming syntax
 *     dialect syntax
 *     AST implementation
 *     semantic implementation
 *     explanation algorithms
 *     proof generation
 *     model inspection
 *     causal analysis
 *     provenance storage
 *     evidence verification
 *     policy enforcement
 *     capability resolution
 *     resource allocation
 *     target selection
 *     compiler implementation
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative source-level explanation grammar.
 *
 * That authority is:
 *
 *     grammar/statements/explain.g4
 *
 * Grammar identity:
 *
 *     Explain
 *
 * This file MUST NOT redefine:
 *
 *     explainStatement
 *     explanationTarget
 *     explanationSourceClause
 *     explanationContextClause
 *     explanationContextList
 *     explanationContext
 *
 * The universal grammar already defines those rules.
 *
 *
 * ============================================================================
 * WHY THIS ADAPTER EXISTS
 * ============================================================================
 *
 * AI needs to expose explanation as an explicit compositional capability.
 *
 * However, explanation is not intrinsically an AI feature.
 *
 * A separate AI implementation of:
 *
 *     explain
 *
 * would create a second syntax and eventually a second semantic model.
 *
 * That would produce an undesirable architecture:
 *
 *     universal explanation
 *          +
 *     AI explanation
 *
 * instead of:
 *
 *     universal explanation
 *             |
 *             v
 *        AI composition
 *
 * This adapter therefore keeps one language and one explanation model.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                       canonical parser
 *                              |
 *              +---------------+---------------+
 *              |                               |
 *              v                               v
 *        universal statements                  AI
 *              |                               |
 *              v                               v
 *       explainStatement                AIExplanations
 *              |                               |
 *              +---------------+---------------+
 *                              |
 *                              v
 *                     domain-neutral AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *            types          effects          policies
 *                              |
 *                    +---------+---------+
 *                    |         |         |
 *                    v         v         v
 *               evidence   provenance  capabilities
 *                              |
 *                              v
 *                    semantic explanation
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir        HDL/hardware
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                    optimization/lowering
 *                              |
 *                         scheduling
 *                              |
 *                       target realization
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/statements/explain.g4
 *
 * IMPORTS:
 *
 *     Explain
 *
 * EXPORTS:
 *
 *     aiExplanationConstruct
 *     aiExplanationStatement
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     universal explanation semantic subsystem
 *     plus AI semantic analysis where the explained subject is AI-related
 *
 * TYPE_OWNER:
 *
 *     canonical type subsystem
 *
 * EFFECT_OWNER:
 *
 *     canonical effect subsystem
 *
 * CAPABILITY_OWNER:
 *
 *     canonical capability subsystem
 *
 * RESOURCE_OWNER:
 *
 *     canonical resource subsystem
 *
 * CONTRACT_OWNER:
 *
 *     canonical validation/contract subsystem
 *
 * POLICY_OWNER:
 *
 *     canonical policy/security subsystem
 *
 * EVIDENCE_OWNER:
 *
 *     canonical evidence/validation/provenance infrastructure
 *
 * PROVENANCE_OWNER:
 *
 *     canonical provenance subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic/domain IR pipeline
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/explanations/
 *
 *     plus:
 *
 *     grammar/tests/statements/explain/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/specification/
 *
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The dependency direction MUST remain:
 *
 *     Expressions
 *          |
 *          v
 *     Explain
 *          |
 *          v
 *     AIExplanations
 *          |
 *          v
 *     AI
 *
 * More precisely:
 *
 *     grammar/statements/explain.g4
 *              ^
 *              |
 *     grammar/ai/explanations.g4
 *              ^
 *              |
 *     grammar/ai/ai.g4
 *
 * AIExplanations MUST NOT import:
 *
 *     AI
 *
 * AIExplanations MUST NOT import:
 *
 *     Statements
 *
 * because the universal statement composition layer already owns statement
 * integration.
 *
 * This prevents circular parser-grammar dependencies.
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * The canonical lexer remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical explanation keyword is already consumed by:
 *
 *     grammar/statements/explain.g4
 *
 * which uses:
 *
 *     EXPLAIN
 *
 * Therefore this file MUST NOT define or alias:
 *
 *     EXPLAIN
 *     DESCRIBE
 *     CLARIFY
 *     JUSTIFY
 *     WHY
 *     TRANSPARENT
 *
 * merely to enlarge the vocabulary.
 *
 * Explanation mechanisms are semantic concepts, not automatically reserved
 * lexical keywords.
 *
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar AIExplanations;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * UNIVERSAL EXPLANATION IMPORT
 * ============================================================================
 *
 * The canonical explanation statement is owned by:
 *
 *     Explain
 *
 * This import is the only syntax dependency required by this adapter.
 * ============================================================================
 */

import
    Explain
    ;


/*
 * ============================================================================
 * PUBLIC AI EXPLANATION ENTRY
 * ============================================================================
 *
 * An AI explanation construct is exactly the universal explanation statement.
 *
 * No AI-specific syntax is added.
 *
 * This means:
 *
 *     explain prediction;
 *
 *     explain prediction from evidence;
 *
 *     explain model(input);
 *
 *     explain model(input) from training_record
 *         with (provenance, policy);
 *
 * all use the same universal explanation grammar.
 *
 * ============================================================================
 */

aiExplanationConstruct
    : explainStatement
    ;


/*
 * ============================================================================
 * EXPLICIT STATEMENT ALIAS
 * ============================================================================
 *
 * This alias gives the AI composition layer a stable public parser boundary
 * without duplicating the underlying explanation syntax.
 *
 * ============================================================================
 */

aiExplanationStatement
    : explainStatement
    ;


/*
 * ============================================================================
 * AI SUBJECT BOUNDARY
 * ============================================================================
 *
 * AI explanations may semantically describe:
 *
 *     models
 *     predictions
 *     training results
 *     inference results
 *     learned representations
 *     tensor results
 *     datasets
 *     knowledge
 *     reasoning
 *     deductions
 *     hypotheses
 *     decisions
 *     adaptations
 *     uncertainty
 *     causal results
 *     agent decisions
 *     distributed AI results
 *     hybrid classical/quantum results
 *
 * However, all such subjects are ordinary Zamani expressions at parser level.
 *
 * This file therefore deliberately does NOT define:
 *
 *     aiExplanationTarget
 *
 * as a second expression hierarchy.
 *
 * The universal:
 *
 *     explanationTarget
 *
 * remains authoritative.
 *
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Explanation may describe reasoning results:
 *
 *     explain conclusion;
 *
 *     explain decision from evidence;
 *
 *     explain result with (reasoning_context);
 *
 * Reasoning syntax remains owned by:
 *
 *     grammar/statements/reason.g4
 *
 * and AI reasoning composition remains owned by:
 *
 *     grammar/ai/reasoning.g4
 *
 * This adapter does not duplicate either.
 *
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Explanation may describe knowledge-derived values.
 *
 * Knowledge syntax remains owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * and:
 *
 *     grammar/ai/knowledge.g4
 *
 * This adapter treats the result as an ordinary expression.
 *
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Explanation may describe:
 *
 *     learned model results
 *     predictions
 *     training outcomes
 *     evaluation results
 *     learned representations
 *
 * Learning syntax remains owned by the AI learning subsystem.
 *
 * This file does not enumerate:
 *
 *     model architectures
 *     optimizer families
 *     loss functions
 *     training algorithms
 *     reinforcement algorithms
 *     transfer-learning algorithms
 *     hardware accelerators
 *
 * Future learning methods remain semantic/library/dialect capabilities.
 *
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Explanation may target an adaptation decision:
 *
 *     explain adaptation_decision;
 *
 *     explain selected_strategy from adaptation_record;
 *
 * The explanation request itself does NOT authorize adaptation.
 *
 * Adaptation authorization remains governed by:
 *
 *     policies
 *     capabilities
 *     effects
 *     contracts
 *     resources
 *     provenance
 *
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Explanation may consume values containing:
 *
 *     probability
 *     uncertainty
 *     confidence
 *     belief
 *     likelihood
 *     distributions
 *     observations
 *
 * The uncertainty grammar remains the canonical owner of uncertainty syntax.
 *
 * In particular:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * remains independent of this file.
 *
 * This file MUST NOT introduce:
 *
 *     AIProbability
 *     AIConfidence
 *     AIUncertainty
 *
 * as competing syntax categories.
 *
 *
 * ============================================================================
 * CAUSAL INTEGRATION
 * ============================================================================
 *
 * Explanation may target causal results and causal decisions.
 *
 * Examples:
 *
 *     explain causal_result;
 *
 *     explain decision from causal_evidence;
 *
 *     explain prediction with (causal_context);
 *
 * Causal syntax remains owned by the causal subsystem.
 *
 * This adapter does not define:
 *
 *     cause
 *     effect
 *     intervention
 *     observation
 *     counterfactual
 *     dependency
 *
 * merely to support explanation.
 *
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Explanation may consume evidence represented by ordinary expressions.
 *
 * Evidence may originate from:
 *
 *     source data
 *     experiments
 *     observations
 *     measurements
 *     quantum execution
 *     simulation
 *     hardware telemetry
 *     compiler analysis
 *     optimization
 *     verification
 *     distributed execution
 *     networking
 *     AI model execution
 *
 * Evidence validation remains outside this grammar.
 *
 * The existing validation boundary remains authoritative:
 *
 *     grammar/validation/evidence.g4
 *
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Explanation requests may refer to provenance:
 *
 *     explain result from provenance_record;
 *
 *     explain decision with (provenance);
 *
 *     explain model(input) with (provenance);
 *
 * Provenance syntax remains owned by the canonical provenance expression
 * subsystem where applicable.
 *
 * This file does not create a provenance database or provenance record format.
 *
 *
 * ============================================================================
 * DECISION INTEGRATION
 * ============================================================================
 *
 * A decision is represented by an ordinary expression.
 *
 * Explanation can therefore target decisions produced by:
 *
 *     reasoning
 *     optimization
 *     resource negotiation
 *     capability negotiation
 *     routing
 *     scheduling
 *     resilience
 *     security
 *     adaptation
 *     deployment
 *     model inference
 *     distributed execution
 *     hardware lowering
 *
 * There is no AI-specific Decision AST introduced here.
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Explanation may be subject to policy.
 *
 * Example:
 *
 *     explain decision with (policy);
 *
 * The parser accepts the expression.
 *
 * Semantic analysis determines whether the request is:
 *
 *     permitted
 *     restricted
 *     redacted
 *     incomplete
 *     unavailable
 *     prohibited
 *
 * Syntax validity does not imply authorization.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Explanation may occur inside code governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by the canonical validation/contract grammar.
 *
 * This adapter introduces no contract syntax.
 *
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Explanation can be semantically effectful.
 *
 * Depending on implementation and execution context it may require:
 *
 *     computation
 *     provenance access
 *     evidence access
 *     model inspection
 *     simulation inspection
 *     IO
 *     network access
 *     reflection
 *     external service access
 *
 * The grammar neither grants nor infers these effects.
 *
 * Effect analysis remains authoritative.
 *
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Semantic analysis may require capabilities such as:
 *
 *     explanation
 *     evidence.read
 *     provenance.read
 *     model.inspect
 *     simulation.inspect
 *     execution.trace.read
 *
 * Capability names remain open-world semantic identifiers.
 *
 * No fixed capability catalogue is encoded here.
 *
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Explanation may require:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     model resources
 *     simulation resources
 *     provenance resources
 *     evidence resources
 *     accelerator resources
 *     quantum resources
 *
 * The grammar imposes NO universal capacity ceiling.
 *
 * It MUST NOT introduce:
 *
 *     MAX_EXPLANATION_DEPTH
 *     MAX_EVIDENCE
 *     MAX_PROVENANCE
 *     MAX_CONTEXT
 *     MAX_MODELS
 *     MAX_TENSORS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * or equivalent constants.
 *
 * Resource feasibility belongs downstream.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The same source-level explanation construct must remain syntactically valid
 * regardless of the realization scale.
 *
 * Possible realization environments include:
 *
 *     tiny computational substrates
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * The grammar describes explanation intent only.
 *
 * Physical or runtime feasibility is determined by:
 *
 *     semantic analysis
 *     capabilities
 *     resources
 *     policies
 *     effects
 *     contracts
 *     available evidence
 *     available provenance
 *     implementation support
 *     execution context
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Explanation may describe quantum computation without defining quantum syntax.
 *
 * Examples:
 *
 *     explain quantum_result;
 *
 *     explain measurement_result from measurement_record;
 *
 *     explain routing_decision from compilation_record;
 *
 *     explain quantum_result with (provenance);
 *
 * The quantum source grammar remains authoritative for quantum syntax.
 *
 * The canonical semantic boundary remains:
 *
 *     quantum::ir
 *
 * Therefore this file MUST NOT introduce:
 *
 *     quantum gates
 *     qubit identifiers
 *     physical qubits
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * Explanation of a quantum artifact is a semantic relationship to that
 * artifact, not a quantum grammar extension.
 *
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Explanation can describe a hybrid computation:
 *
 *     classical computation
 *          |
 *          v
 *     quantum computation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     classical decision
 *          |
 *          v
 *     explanation
 *
 * No hybrid-specific explanation syntax is required.
 *
 * The semantic model preserves the cross-domain relationship.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Explanation may target:
 *
 *     hardware intent
 *     synthesis decisions
 *     placement decisions
 *     routing decisions
 *     timing decisions
 *     verification results
 *     simulation results
 *     lowering decisions
 *     resource decisions
 *
 * This grammar does not encode:
 *
 *     fixed bus widths
 *     fixed register widths
 *     fixed memory sizes
 *     fixed device counts
 *     physical identifiers
 *     vendor inventories
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Explanation may target:
 *
 *     distributed decisions
 *     task placement
 *     communication decisions
 *     scheduling
 *     recovery
 *     resilience
 *     consistency
 *     service selection
 *
 * The grammar introduces no fixed:
 *
 *     node count
 *     worker count
 *     actor count
 *     task count
 *     channel count
 *     service count
 *
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * Explanation may target artifacts produced by:
 *
 *     FFI
 *     ABI
 *     foreign functions
 *     external data
 *     SQL
 *     JSON
 *     XML
 *     services
 *     external models
 *
 * Interoperability syntax remains owned by:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * This file only consumes the resulting expressions.
 *
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Explanation may target:
 *
 *     generated artifacts
 *     transformations
 *     syntax trees
 *     generated types
 *     compile-time results
 *     reflection results
 *
 * Reflection and metaprogramming remain independently owned.
 *
 * An explanation request does NOT grant:
 *
 *     reflection capability
 *     code-generation capability
 *     code-execution capability
 *
 *
 * ============================================================================
 * SANDBOX / SECURITY INTEGRATION
 * ============================================================================
 *
 * Explanation requests may execute under a sandbox or security policy.
 *
 * The security subsystem determines whether explanation may access:
 *
 *     evidence
 *     provenance
 *     model state
 *     execution traces
 *     network resources
 *     external services
 *     native resources
 *     sensitive information
 *
 * The grammar does not bypass those controls.
 *
 *
 * ============================================================================
 * ADAPTIVE EXECUTION INTEGRATION
 * ============================================================================
 *
 * An explanation may be requested for an adaptive execution decision:
 *
 *     explain selected_strategy from adaptation_record;
 *
 * The explanation does not itself perform:
 *
 *     detect
 *     retry
 *     recover
 *     fallback
 *     adapt
 *
 * Those operations remain owned by execution and policy subsystems.
 *
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Explanation may target simulation artifacts:
 *
 *     explain simulation_result;
 *
 *     explain simulation_result from simulation_record;
 *
 * The grammar does not choose a simulation backend.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates parser contexts only.
 *
 * It MUST NOT introduce an AI-specific explanation AST hierarchy.
 *
 * The AST representation remains the same canonical representation produced
 * by:
 *
 *     explainStatement
 *
 * Conceptually:
 *
 *     ExplanationStatement
 *         target
 *         source?
 *         context*
 *         source_span
 *
 * The AST MUST preserve:
 *
 *     target
 *     optional source
 *     ordered context
 *     source spans
 *     source ordering
 *
 * The AST MUST NOT encode:
 *
 *     AI explanation engine
 *     model family
 *     model provider
 *     hardware target
 *     accelerator
 *     physical QPU
 *     physical qubit
 *     CPU identifier
 *     GPU identifier
 *     FPGA identifier
 *     network node identifier
 *     backend implementation
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A syntactically valid AI explanation construct means only:
 *
 *     an explanation request was expressed in an AI composition context.
 *
 * It does NOT imply:
 *
 *     target exists
 *     target is explainable
 *     evidence exists
 *     provenance exists
 *     explanation mechanism exists
 *     caller is authorized
 *     explanation is complete
 *     explanation is truthful
 *     explanation is deterministic
 *     explanation is reproducible
 *     execution is available
 *
 * Semantic analysis must establish those properties independently.
 *
 * The semantic layer may classify an explanation request as:
 *
 *     available
 *     partially available
 *     unavailable
 *     restricted
 *     prohibited
 *
 * without changing parser behavior.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The explanation target, source and context are ordinary expressions.
 *
 * Their types are determined by the canonical type system.
 *
 * This grammar does not introduce:
 *
 *     AIExplanationType
 *     ExplanationType
 *     EvidenceType
 *     ProvenanceType
 *     DecisionType
 *
 * as replacement type systems.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar declares no effects.
 *
 * Semantic effect analysis may determine that realization requires:
 *
 *     read
 *     compute
 *     IO
 *     network
 *     reflection
 *     model inspection
 *     simulation inspection
 *     provenance access
 *     evidence access
 *
 * The explanation statement MUST NOT silently acquire those effects merely
 * because it parsed successfully.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities are semantic requirements.
 *
 * The implementation may require capabilities such as:
 *
 *     capability("explanation")
 *     capability("evidence.read")
 *     capability("provenance.read")
 *     capability("model.inspect")
 *     capability("execution.trace.read")
 *
 * These are not parser-level hardware commitments.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements are determined downstream.
 *
 * An explanation can consume resources proportional to:
 *
 *     target complexity
 *     evidence volume
 *     provenance volume
 *     model complexity
 *     simulation complexity
 *     trace complexity
 *     requested context
 *
 * These quantities are semantic/runtime values.
 *
 * No fixed maximum is encoded in this grammar.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Explanation may be governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract checking remains independently owned.
 *
 * Explanation itself does not establish that a contract is satisfied.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may:
 *
 *     permit explanation
 *     restrict explanation
 *     redact explanation
 *     require evidence
 *     require provenance
 *     require reproducibility
 *     prohibit sensitive information disclosure
 *     constrain external access
 *     constrain model inspection
 *     constrain reflection
 *
 * Policy evaluation remains downstream.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The explanation request must remain traceable through the frontend.
 *
 * Downstream provenance may associate:
 *
 *     source span
 *     target
 *     source
 *     context
 *     semantic decision
 *     evidence
 *     provenance
 *     explanation artifact
 *     compiler transformation
 *     execution record
 *     language version
 *     grammar version
 *     semantic version
 *
 * This file does not create provenance records.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This adapter creates NO explanation-specific IR.
 *
 * The explanation request remains associated with the canonical semantic
 * representation of its target.
 *
 * Possible downstream representations include:
 *
 *     classical semantic representation
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     execution representation
 *     compiler provenance representation
 *     future domain representation
 *
 * The AI explanation adapter MUST NOT introduce:
 *
 *     AIExplanationIR
 *     ExplanationIR
 *     AIQuantumIR
 *     AIHardwareIR
 *
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * If an explained subject is quantum-derived, the relationship is:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     explanation/provenance consumer
 *
 * The explanation grammar never constructs or modifies quantum::ir.
 *
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * If an explained subject is HDL/hardware-derived:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     HDL/hardware representation
 *       |
 *       v
 *     explanation/provenance consumer
 *
 * This file remains independent of hardware dimensions and topology.
 *
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * The backend may consume explanation intent to produce:
 *
 *     structured explanation data
 *     provenance records
 *     evidence references
 *     diagnostics
 *     transformation traces
 *     optimization explanations
 *     execution explanations
 *     hardware-lowering explanations
 *     quantum-transformation explanations
 *     simulation explanations
 *     distributed-execution explanations
 *
 * Backend behavior is outside this grammar.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are inherited from:
 *
 *     Explain
 *
 * Examples include:
 *
 *     missing EXPLAIN
 *     missing target
 *     malformed FROM clause
 *     malformed WITH clause
 *     missing closing parenthesis
 *     empty context
 *     malformed context list
 *     unexpected clause order
 *     missing statement terminator
 *
 * This adapter MUST NOT introduce AI-specific parser diagnostics for semantic
 * conditions.
 *
 * Semantic diagnostics belong downstream, including:
 *
 *     unknown target
 *     unavailable evidence
 *     unavailable provenance
 *     unsupported explanation mechanism
 *     insufficient capability
 *     insufficient resources
 *     forbidden policy
 *     effect violation
 *     invalid target type
 *     unavailable introspection
 *     unavailable simulation
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * AI-domain conformance tests must accept at least:
 *
 *     explain prediction;
 *
 *     explain prediction from evidence;
 *
 *     explain prediction with (provenance);
 *
 *     explain prediction from evidence with (provenance, policy);
 *
 *     explain model(input);
 *
 *     explain model(input) from training_record;
 *
 *     explain model(input) from training_record
 *         with (provenance, policy);
 *
 *     explain reasoning_result;
 *
 *     explain reasoning_result from reasoning_evidence;
 *
 *     explain knowledge_result;
 *
 *     explain decision;
 *
 *     explain decision from decision_evidence;
 *
 *     explain adaptation_decision from adaptation_record;
 *
 *     explain causal_result from causal_evidence;
 *
 *     explain uncertain_result with (confidence);
 *
 *     explain quantum_result;
 *
 *     explain measurement_result from measurement_record;
 *
 *     explain hybrid_result with (provenance);
 *
 *     explain simulation_result from simulation_record;
 *
 *     explain hardware_result from synthesis_record;
 *
 *     explain distributed_result from execution_record
 *         with (provenance, policy);
 *
 *     explain generated_artifact;
 *
 *     explain transformation_record;
 *
 * All target/source/context expressions remain governed by the canonical
 * expression grammar.
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The AI explanation boundary must reject or diagnose the same malformed
 * explanation syntax rejected by the universal explanation grammar:
 *
 *     explain;
 *
 *     explain from evidence;
 *
 *     explain with (policy);
 *
 *     explain result from;
 *
 *     explain result with;
 *
 *     explain result with ();
 *
 *     explain result with (evidence,);
 *
 *     explain result with (, evidence);
 *
 *     explain result with (evidence,, policy);
 *
 *     explain result with (evidence policy);
 *
 *     explain result with (evidence) from source;
 *
 *     explain result from source with;
 *
 *     explain result from source with ();
 *
 * The AI adapter must not weaken or alter these universal diagnostics.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test combinations of:
 *
 *     AI + reasoning
 *     AI + knowledge
 *     AI + learning
 *     AI + adaptation
 *     AI + uncertainty
 *     AI + causality
 *     AI + evidence
 *     AI + provenance
 *     AI + contracts
 *     AI + policies
 *     AI + classical computation
 *     AI + quantum computation
 *     AI + hybrid computation
 *     AI + HDL/hardware
 *     AI + distributed execution
 *     AI + simulation
 *     AI + metaprogramming
 *     AI + interoperability
 *
 * The parser must continue to treat the explanation subject as an ordinary
 * expression.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * At least one conformance fixture should exercise a complete chain such as:
 *
 *     classical computation
 *          ->
 *     quantum operation
 *          ->
 *     measurement
 *          ->
 *     AI inference
 *          ->
 *     decision
 *          ->
 *     provenance
 *          ->
 *     explanation
 *
 * The explanation grammar must remain unchanged across that chain.
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests must vary source size and expression complexity without
 * turning test dimensions into language-level constants.
 *
 * Test progressively larger:
 *
 *     explanation statements
 *     context expressions
 *     nested expressions
 *     qualified names
 *     model expressions
 *     evidence expressions
 *     provenance expressions
 *
 * The grammar MUST NOT impose a finite maximum on any of these concepts.
 *
 * A parser implementation may have practical resource limits, but such limits
 * belong to implementation/resource configuration rather than language
 * semantics.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source tokens
 *     grammar version
 *     parser configuration
 *
 * this grammar must produce equivalent parse-tree structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     resources
 *     filesystem state
 *     network state
 *     clock
 *     randomness
 *     runtime state
 *     target availability
 *     model availability
 *     evidence availability
 *     provenance availability
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Public rules:
 *
 *     aiExplanationConstruct
 *     aiExplanationStatement
 *
 * are stable AI composition boundaries.
 *
 * The underlying explanation syntax remains versioned by:
 *
 *     grammar/statements/explain.g4
 *
 * Therefore:
 *
 *     changes to universal explanation syntax
 *
 * are compatibility-sensitive.
 *
 * This adapter should normally require no changes when new explanation
 * mechanisms are added semantically.
 *
 * This is intentional.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain:
 *
 *     [x] no machine-capacity constants;
 *     [x] no quantum-capacity constants;
 *     [x] no CPU/GPU/FPGA/QPU enumeration;
 *     [x] no vendor enumeration;
 *     [x] no device identifiers;
 *     [x] no fixed tensor rank;
 *     [x] no fixed network size;
 *     [x] no fixed node count;
 *     [x] no fixed memory size;
 *     [x] no fixed register width;
 *     [x] no parser-level allocation;
 *     [x] no hardware discovery;
 *     [x] no runtime execution;
 *     [x] no embedded Rust;
 *     [x] no unsafe Rust requirement.
 *
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before marking this file DONE:
 *
 *     [ ] grammar/statements/explain.g4 exists and is canonical.
 *
 *     [ ] Explain is the grammar identity of that file.
 *
 *     [ ] Explain exports explainStatement.
 *
 *     [ ] Explain is imported exactly once through this adapter.
 *
 *     [ ] AI imports AIExplanations.
 *
 *     [ ] AI exposes aiExplanationConstruct.
 *
 *     [ ] AI exposes aiExplanationStatement if required by downstream
 *         composition.
 *
 *     [ ] No explanation syntax is duplicated in AI.
 *
 *     [ ] No AI-specific explanation AST is introduced.
 *
 *     [ ] Existing expression grammar remains the expression authority.
 *
 *     [ ] Existing uncertainty grammar remains the uncertainty authority.
 *
 *     [ ] Existing reasoning grammar remains the reasoning authority.
 *
 *     [ ] Existing knowledge grammar remains the knowledge authority.
 *
 *     [ ] Existing evidence validation remains authoritative.
 *
 *     [ ] Provenance remains downstream/semantic.
 *
 *     [ ] Policies remain downstream.
 *
 *     [ ] Effects remain downstream.
 *
 *     [ ] Capabilities remain downstream.
 *
 *     [ ] Resources remain downstream.
 *
 *     [ ] quantum::ir remains the canonical quantum IR.
 *
 *     [ ] No finite scalability limit is introduced.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Generated parser integration is verified using Rust 1.97+.
 *
 *     [ ] Generated parser integration remains safe Rust.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. It is the sole AI-domain explanation composition adapter.
 *
 *     2. It imports the existing universal Explain grammar.
 *
 *     3. It introduces no duplicate explanation syntax.
 *
 *     4. It introduces no duplicate expression hierarchy.
 *
 *     5. It introduces no AI-specific explanation AST.
 *
 *     6. It introduces no explanation-specific IR.
 *
 *     7. It preserves evidence and provenance as downstream semantic concerns.
 *
 *     8. It preserves policy/effect/capability/resource boundaries.
 *
 *     9. It integrates with reasoning, knowledge, learning, adaptation,
 *        uncertainty and causality without owning those syntaxes.
 *
 *    10. It integrates with classical, quantum, hybrid, HDL, hardware,
 *        distributed, networking, simulation, compilation and metaprogramming
 *        through ordinary semantic expressions.
 *
 *    11. It preserves the quantum::ir boundary.
 *
 *    12. It introduces no hardware or machine-size ceiling.
 *
 *    13. It remains deterministic and environment-independent.
 *
 *    14. It requires no embedded Rust or unsafe implementation.
 *
 *    15. Its public rules remain stable even when new explanation mechanisms
 *        are added downstream.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * AI explanation is NOT a second explanation language.
 *
 * The correct architecture is:
 *
 *     universal explain syntax
 *              |
 *              v
 *       universal semantic model
 *              |
 *              +----------------------+
 *              |                      |
 *              v                      v
 *        AI composition          other domains
 *              |                      |
 *              +----------+-----------+
 *                         |
 *                         v
 *                  common provenance
 *                         |
 *                         v
 *                 common policy/effects
 *                         |
 *                         v
 *                 canonical semantic IR
 *
 * Therefore this file should remain intentionally small in executable grammar
 * rules and intentionally strong in architectural contracts.
 *
 * New explanation algorithms, evidence mechanisms, model explanation methods,
 * causal explanation methods, quantum explanation mechanisms, hardware
 * explanation mechanisms and future explanation technologies should normally
 * be added downstream without changing this grammar.
 *
 * ============================================================================
 */