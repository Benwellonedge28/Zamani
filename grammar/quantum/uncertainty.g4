/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/uncertainty.g4
 *
 * Grammar:
 *     QuantumUncertainty
 *
 * Status:
 *     CANONICAL QUANTUM-DOMAIN UNCERTAINTY INTEGRATION GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97 or later
 *     Rust 2021 edition
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the QUANTUM-DOMAIN INTEGRATION BOUNDARY for uncertainty.
 *
 * IMPORTANT:
 *
 *     This file does NOT define a second uncertainty syntax.
 *
 * The universal source-level uncertainty syntax is owned exclusively by:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * whose parser grammar is:
 *
 *     UncertaintyExpressions
 *
 * This file therefore adapts that universal construct for quantum semantic
 * composition without redefining:
 *
 *     uncertaintyExpression
 *     uncertainValueExpression
 *     uncertaintyValue
 *     uncertaintyArgumentList
 *     uncertaintyArgument
 *     uncertaintyNamedArgument
 *     uncertaintyPositionalArgument
 *     uncertaintyFieldName
 *
 * The architectural direction is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical expression grammar
 *       |
 *       v
 *     uncertaintyExpression
 *       |
 *       v
 *     QuantumUncertainty
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +-------------------------------+
 *       |               |               |
 *       v               v               v
 *     classical      quantum::ir      other domains
 *                       |
 *                       v
 *                optimization
 *                       |
 *                decomposition
 *                       |
 *                    routing
 *                       |
 *                  scheduling
 *                       |
 *                 QEC/resilience
 *                       |
 *                      ZQN
 *                       |
 *                      HAL
 *                       |
 *                 target realization
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *
 * Normative quantum specification:
 *
 *     grammar/spec/quantum.md
 *
 * Normative uncertainty expression specification:
 *
 *     grammar/spec/ai.md
 *     grammar/spec/types.md
 *     grammar/spec/effects.md
 *
 * Canonical lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical universal uncertainty expression:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Canonical quantum expression composition:
 *
 *     grammar/expressions/quantum.g4
 *
 * Canonical quantum-domain composition:
 *
 *     grammar/quantum/quantum.g4
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * This file creates NO competing AST and NO competing IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     quantumUncertaintyExpression
 *     quantumUncertaintyConstruct
 *
 * These are quantum-domain integration wrappers only.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     uncertaintyExpression
 *     uncertainValueExpression
 *     uncertaintyValue
 *     uncertaintyArgumentList
 *     uncertaintyArgument
 *     uncertaintyNamedArgument
 *     uncertaintyPositionalArgument
 *     uncertaintyFieldName
 *
 * Those productions belong exclusively to:
 *
 *     UncertaintyExpressions
 *
 * THIS FILE ALSO DOES NOT OWN:
 *
 *     identifiers
 *     names
 *     expressions
 *     expression precedence
 *     function calls
 *     arithmetic
 *     probability mathematics
 *     distributions
 *     confidence mathematics
 *     evidence semantics
 *     provenance semantics
 *     quantum operations
 *     measurements
 *     quantum states
 *     quantum types
 *     QEC
 *     routing
 *     scheduling
 *     calibration
 *     hardware topology
 *     physical qubit allocation
 *     backend selection
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one owner of universal uncertainty syntax:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Quantum-specific uncertainty must therefore be expressed as:
 *
 *     quantumUncertaintyExpression
 *         : uncertaintyExpression
 *         ;
 *
 * and never by copying the universal uncertainty productions.
 *
 * This prevents competing syntaxes such as:
 *
 *     quantumUncertainty(...)
 *
 *     quantumProbability(...)
 *
 *     quantumConfidence(...)
 *
 * from becoming parallel grammar systems.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Grammar:
 *
 *     UncertaintyExpressions
 *
 * Public imported rule:
 *
 *     uncertaintyExpression
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public rules:
 *
 *     quantumUncertaintyExpression
 *     quantumUncertaintyConstruct
 *
 * ============================================================================
 * CONSUMED_BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/expressions/quantum.g4
 *
 * Recommended integration:
 *
 *     import QuantumUncertainty;
 *
 * followed by:
 *
 *     quantumExpression
 *         : ...
 *         | quantumUncertaintyExpression
 *         | ...
 *         ;
 *
 * This permits uncertainty to appear as a quantum-domain expression without
 * changing the universal expression language.
 *
 * ============================================================================
 * ROOT-PARSER INTEGRATION
 * ============================================================================
 *
 * This file MUST NOT be imported directly by:
 *
 *     grammar/Zamani.g4
 *
 * or:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * merely because quantum uncertainty exists.
 *
 * The intended composition is:
 *
 *     ZamaniParser
 *          |
 *          v
 *     Expressions
 *          |
 *          v
 *     QuantumExpressions
 *          |
 *          v
 *     QuantumUncertainty
 *          |
 *          v
 *     UncertaintyExpressions
 *
 * This maintains a single universal expression hierarchy.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * It introduces:
 *
 *     no keywords
 *     no operators
 *     no punctuation
 *     no literals
 *     no identifiers
 *     no quantum gate tokens
 *     no uncertainty-specific token aliases
 *
 * The canonical lexical vocabulary remains owned by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Relevant uncertainty vocabulary is already represented by the canonical
 * uncertainty grammar/lexer contract, including concepts such as:
 *
 *     uncertain
 *     uncertainty
 *     probability
 *     probabilistic
 *     distribution
 *     confidence
 *     belief
 *     likelihood
 *     evidence
 *     provenance
 *     source
 *
 * This file does not require additional lexical vocabulary.
 *
 * ============================================================================
 * NO KEYWORD EXPLOSION
 * ============================================================================
 *
 * Quantum uncertainty must remain open-ended.
 *
 * The grammar MUST NOT introduce dedicated keywords for every possible
 * uncertainty concept.
 *
 * Examples that remain semantic/library concepts include:
 *
 *     variance
 *     covariance
 *     entropy
 *     fidelity
 *     purity
 *     calibration
 *     reliability
 *     interval
 *     posterior
 *     prior
 *     likelihood
 *     credibility
 *     dispersion
 *     error_bound
 *     uncertainty_budget
 *
 * New mathematical or scientific concepts should normally be represented by
 * ordinary identifiers, expressions, types, libraries, dialects, or semantic
 * metadata.
 *
 * ============================================================================
 * PUBLIC RULE: QUANTUM UNCERTAINTY EXPRESSION
 * ============================================================================
 *
 * A quantum uncertainty expression is exactly the universal uncertainty
 * expression viewed in a quantum semantic context.
 *
 * Examples of valid semantic inputs include:
 *
 *     uncertain(measurement_result)
 *
 *     uncertainty(measure(q))
 *
 *     uncertain(
 *         measurement_result,
 *         confidence: confidence_value
 *     )
 *
 *     uncertainty(
 *         result,
 *         probability: probability_value
 *     )
 *
 *     uncertainty(
 *         quantum_result,
 *         distribution: distribution_value
 *     )
 *
 *     uncertainty(
 *         result,
 *         evidence: evidence_value,
 *         provenance: provenance_value
 *     )
 *
 * The quantum semantics are determined downstream.
 *
 * ============================================================================
 */

parser grammar QuantumUncertainty;

options {
    tokenVocab = ZamaniLexer;
}

import UncertaintyExpressions;


/*
 * ============================================================================
 * 1. PUBLIC QUANTUM UNCERTAINTY EXPRESSION
 * ============================================================================
 *
 * This is the primary quantum-expression integration point.
 *
 * It deliberately delegates completely to the universal uncertainty owner.
 */
quantumUncertaintyExpression
    : uncertaintyExpression
    ;


/*
 * ============================================================================
 * 2. QUANTUM UNCERTAINTY CONSTRUCT
 * ============================================================================
 *
 * This façade exists for quantum-domain composition grammars that distinguish
 * between expressions and domain constructs.
 *
 * It does not introduce another syntax layer.
 */
quantumUncertaintyConstruct
    : quantumUncertaintyExpression
    ;


/*
 * ============================================================================
 * VALUE CONTRACT
 * ============================================================================
 *
 * The underlying uncertainty expression may semantically wrap any value
 * supported by the universal expression/type system.
 *
 * Quantum examples include:
 *
 *     quantum measurement results
 *     state-derived values
 *     observable values
 *     expectation values
 *     sampling results
 *     statistical estimates
 *     circuit results
 *     simulator results
 *     QEC-related observations
 *     resilience observations
 *     hybrid classical/quantum results
 *
 * This grammar does not enumerate these as a closed list.
 *
 * A future quantum semantic value must automatically be able to participate
 * in uncertainty if its type and semantics permit it.
 *
 * ============================================================================
 * QUANTUM MEASUREMENT INTEGRATION
 * ============================================================================
 *
 * Measurement syntax remains owned by:
 *
 *     grammar/quantum/measurement.g4
 *
 * This file does not redefine measurement.
 *
 * A measurement result may become an uncertainty value through ordinary
 * expression composition.
 *
 * Conceptually:
 *
 *     measure q
 *         |
 *         v
 *     measurement result
 *         |
 *         v
 *     uncertainty(measurement_result)
 *
 * The grammar does not decide whether the measurement itself is:
 *
 *     destructive
 *     non-destructive
 *     mid-circuit
 *     observable-based
 *     basis-based
 *     sampled
 *     hardware-backed
 *     simulator-backed
 *
 * Those semantics belong to the measurement and semantic systems.
 *
 * ============================================================================
 * QUANTUM STATE INTEGRATION
 * ============================================================================
 *
 * Quantum state syntax remains owned by:
 *
 *     grammar/quantum/quantum-states.g4
 *
 * or the repository's currently designated canonical quantum-state owner.
 *
 * This file does not redefine:
 *
 *     quantumStateExpression
 *     quantumStateReference
 *     quantumStateConstructor
 *     superposition
 *     mixture
 *     tensor/product
 *
 * A quantum state may participate in uncertainty when the semantic/type
 * system permits that relationship.
 *
 * ============================================================================
 * OBSERVABLE INTEGRATION
 * ============================================================================
 *
 * Observable syntax remains owned by:
 *
 *     grammar/quantum/observables.g4
 *
 * An observable-derived value may participate in uncertainty without requiring
 * an observable-specific uncertainty grammar.
 *
 * For example, the semantic structure may represent:
 *
 *     uncertainty(expectation_value)
 *
 * without this file knowing whether the observable is:
 *
 *     named
 *     constructed
 *     parameterized
 *     library-provided
 *     vendor-provided
 *     future-defined
 *
 * ============================================================================
 * QUANTUM OPERATION INTEGRATION
 * ============================================================================
 *
 * Operation syntax remains owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * and:
 *
 *     grammar/expressions/quantum.g4
 *
 * This file must never enumerate operations such as:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * or any future operation.
 *
 * Operation identity remains open-ended.
 *
 * ============================================================================
 * QUANTUM-CLASSICAL / HYBRID INTEGRATION
 * ============================================================================
 *
 * Uncertainty is intentionally usable at the quantum/classical boundary.
 *
 * Conceptual flow:
 *
 *     quantum operation
 *          |
 *          v
 *     quantum result
 *          |
 *          v
 *     uncertainty(...)
 *          |
 *          v
 *     classical reasoning / decision
 *
 * Conversely, classical uncertainty may influence quantum computation:
 *
 *     uncertain(parameter)
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum parameter/control
 *
 * This file does not define hybrid control syntax.
 *
 * Hybrid control remains owned by:
 *
 *     grammar/hybrid/
 *     grammar/quantum/quantum-classical.g4
 *     grammar/quantum/mid-circuit-control.g4
 *
 * according to their existing ownership boundaries.
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * Quantum uncertainty may consume AI-derived values:
 *
 *     uncertainty(model_prediction)
 *
 * and AI may consume quantum-derived uncertainty:
 *
 *     uncertainty(quantum_result)
 *
 * This is intentionally achieved through the universal uncertainty expression
 * rather than through an AI-specific quantum uncertainty grammar.
 *
 * Existing AI uncertainty integration remains owned by:
 *
 *     grammar/ai/uncertainty.g4
 *
 * That file and this file are peer domain adapters.
 *
 * Neither owns the universal uncertainty syntax.
 *
 * Architecture:
 *
 *                 UncertaintyExpressions
 *                         |
 *              +----------+----------+
 *              |                     |
 *              v                     v
 *      AIUncertainty         QuantumUncertainty
 *              |                     |
 *              v                     v
 *             AI                  Quantum
 *
 * ============================================================================
 * PROBABILITY INTEGRATION
 * ============================================================================
 *
 * Probability semantics remain outside this grammar.
 *
 * The grammar does not impose:
 *
 *     a numerical representation;
 *     floating-point width;
 *     fixed-point width;
 *     arbitrary-precision implementation;
 *     symbolic representation;
 *     interval representation;
 *     sampling representation;
 *     hardware representation.
 *
 * Semantic validation may impose mathematical invariants appropriate to the
 * resolved probability type.
 *
 * The parser must not turn those semantic constraints into machine-size
 * constants.
 *
 * ============================================================================
 * DISTRIBUTION INTEGRATION
 * ============================================================================
 *
 * No distribution family is enumerated here.
 *
 * The grammar does not hard-code:
 *
 *     Gaussian
 *     Bernoulli
 *     binomial
 *     Poisson
 *     categorical
 *     Dirichlet
 *     uniform
 *     custom
 *
 * or any finite list.
 *
 * A distribution remains a semantic value that may be supplied by:
 *
 *     library
 *     provider
 *     model
 *     dialect
 *     symbolic expression
 *     future implementation
 *
 * ============================================================================
 * CONFIDENCE / BELIEF / LIKELIHOOD
 * ============================================================================
 *
 * These remain universal uncertainty metadata/semantic concepts.
 *
 * Quantum-specific interpretation may include:
 *
 *     confidence in an estimated result
 *     confidence in a sampled outcome
 *     belief in a state/model
 *     likelihood of an observed result
 *
 * This file does not define their mathematical semantics.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence syntax remains owned by the universal evidence/provenance
 * subsystem.
 *
 * Quantum evidence may originate from:
 *
 *     measurement
 *     repeated observations
 *     simulation
 *     calibration data
 *     verification
 *     classical computation
 *     external scientific data
 *     distributed execution
 *
 * The parser merely preserves the source expression.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Quantum uncertainty may carry provenance describing:
 *
 *     source
 *     measurement
 *     circuit
 *     operation
 *     transformation
 *     simulation
 *     execution
 *     backend-independent derivation
 *     verification
 *     model
 *     evidence
 *     version
 *
 * Provenance semantics remain owned by the repository-wide provenance system.
 *
 * This file must not define a quantum-specific provenance model.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar introduces no effects itself.
 *
 * Effects are derived semantically from the underlying expression.
 *
 * Possible effects include:
 *
 *     measurement
 *     randomness
 *     simulation
 *     learning
 *     IO
 *     network
 *     distributed
 *     foreign
 *     native
 *
 * For example:
 *
 *     uncertainty(measure(q))
 *
 * may require a measurement effect because the nested expression resolves to
 * a measurement operation.
 *
 * The parser must not infer that effect merely from syntax.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements are semantic.
 *
 * Examples may include:
 *
 *     capability("quantum.measurement")
 *     capability("quantum.sampling")
 *     capability("probabilistic.compute")
 *     capability("statistical.compute")
 *     capability("quantum.observable")
 *     capability("provenance.record")
 *
 * This file performs no capability discovery.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Quantum uncertainty may require resources derived from the semantic
 * operation.
 *
 * Examples include:
 *
 *     quantum resources
 *     classical compute
 *     memory
 *     storage
 *     samples
 *     communication
 *     accelerator resources
 *     simulation resources
 *
 * No finite resource capacity is encoded in this grammar.
 *
 * In particular, this file contains no:
 *
 *     MAX_QUBITS
 *     MAX_SAMPLES
 *     MAX_OUTCOMES
 *     MAX_DISTRIBUTION_SIZE
 *     MAX_PROBABILITY_BITS
 *     MAX_CONFIDENCE_BITS
 *     MAX_VARIABLES
 *     MAX_STATE_DIMENSION
 *     MAX_TENSOR_RANK
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_QPUS
 *
 * or equivalent limit.
 *
 * Resource availability is resolved downstream.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This grammar defines no quantum uncertainty type.
 *
 * It must not introduce competing types such as:
 *
 *     QuantumUncertain<T>
 *     QuantumProbability<T>
 *     QuantumDistribution<T>
 *     QuantumConfidence<T>
 *
 * The universal type system determines whether an uncertainty value is
 * represented conceptually as something such as:
 *
 *     Uncertain<T>
 *     Probability<T>
 *     Distribution<T>
 *     Confidence<T>
 *     Belief<T>
 *
 * without requiring this grammar to define those types.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Quantum uncertainty may participate in universal contracts:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * Example semantic intent:
 *
 *     requires confidence(result) >= required_confidence
 *
 * The contract grammar owns the contract syntax.
 *
 * This file must not duplicate it.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may constrain:
 *
 *     measurement uncertainty
 *     acceptable confidence
 *     allowed evidence
 *     allowed randomness
 *     reproducibility
 *     provenance
 *     privacy
 *     external data
 *     simulation
 *     backend-independent execution
 *
 * A policy may reject an otherwise syntactically valid expression.
 *
 * Syntax validity is therefore not authorization or feasibility.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Uncertainty may be consumed by adaptive execution.
 *
 * Conceptual flow:
 *
 *     uncertainty(result)
 *          |
 *          v
 *     evaluate
 *          |
 *          v
 *     policy / contract
 *          |
 *          v
 *     select
 *          |
 *          +----> ACCEPT
 *          +----> DEGRADED_ACCEPT
 *          +----> RETRY
 *          +----> RECOVER
 *          +----> ESCALATE
 *          +----> REJECT
 *
 * Resilience states remain owned by the execution/resilience subsystem.
 *
 * This grammar does not define runtime recovery behavior.
 *
 * ============================================================================
 * DETERMINISM AND REPRODUCIBILITY
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * This grammar must not inspect:
 *
 *     hardware availability
 *     QPU availability
 *     simulator state
 *     random seeds
 *     filesystem state
 *     network state
 *     wall-clock time
 *     environment variables
 *     device calibration
 *     target selection
 *
 * Reproducibility requirements belong to semantic/execution/provenance
 * systems.
 *
 * An uncertainty expression may represent stochastic computation without
 * making parsing stochastic.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar introduces no universal finite ceiling.
 *
 * Source-level uncertainty expressions may contain:
 *
 *     arbitrary semantic nesting supported by implementation resources;
 *     arbitrary metadata cardinality;
 *     arbitrary expression structure;
 *     arbitrary quantum-derived values;
 *     arbitrary result structures.
 *
 * ANTLR repetition and recursive expression structure provide source-level
 * extensibility.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite ceiling.
 *
 * It does NOT mean:
 *
 *     infinite physical memory;
 *     infinite parser memory;
 *     infinite execution time;
 *     infinite QPU capacity;
 *     infinite storage.
 *
 * Actual finite limitations belong to:
 *
 *     compiler resources
 *     runtime resources
 *     target resources
 *     capability availability
 *     deployment policy
 *     execution environment
 *
 * Therefore the same source program can remain unchanged while the compiler
 * specializes it for different available resources.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     source construct kind
 *     underlying uncertainty expression
 *     argument ordering
 *     metadata ordering
 *     source span
 *     source spelling where compatibility/provenance requires it
 *     nested quantum expression structure
 *
 * The AST MUST NOT contain:
 *
 *     physical qubit IDs
 *     physical device IDs
 *     backend names
 *     topology assignments
 *     calibration records
 *     routing decisions
 *     scheduling decisions
 *     QEC implementation details
 *     ZQN implementation details
 *     HAL implementation details
 *
 * merely because a quantum uncertainty expression was parsed.
 *
 * The semantic layer may subsequently derive such information where needed.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     whether the underlying value is valid;
 *     whether it has a quantum-related type;
 *     whether measurement results are valid;
 *     whether probability metadata is mathematically valid;
 *     whether distribution metadata is compatible;
 *     whether confidence metadata has the required type;
 *     whether evidence is valid;
 *     whether provenance requirements are satisfied;
 *     whether effects are permitted;
 *     whether capabilities are available;
 *     whether resources are sufficient;
 *     whether policies permit execution;
 *     whether the expression can cross into quantum::ir.
 *
 * None of these are parser-level decisions.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO quantum IR.
 *
 * The semantic pipeline remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *
 * Quantum uncertainty may contribute semantic information to the canonical
 * quantum representation when the resolved operation is quantum.
 *
 * It must not create:
 *
 *     QuantumUncertaintyIR
 *     QuantumProbabilityIR
 *     QuantumDistributionIR
 *     QuantumMeasurementUncertaintyIR
 *
 * as competing IR families.
 *
 * If the canonical quantum::ir needs uncertainty/probability metadata, that
 * metadata belongs to the existing semantic/IR schema rather than a new
 * parser-specific IR.
 *
 * ============================================================================
 * QEC / RESILIENCE / ZQN BOUNDARY
 * ============================================================================
 *
 * Uncertainty may be consumed downstream by:
 *
 *     QEC
 *     resilience
 *     ZQN
 *     verification
 *     mitigation
 *     statistical analysis
 *
 * This grammar does not define those algorithms.
 *
 * In particular, it does not define:
 *
 *     syndrome decoding
 *     noise models
 *     readout mitigation
 *     error extrapolation
 *     recovery
 *     retry
 *     backend switching
 *
 * Those belong downstream.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     physical qubit
 *     readout channel
 *     detector
 *     memory bank
 *     topology
 *     vendor
 *     calibration
 *
 * Target realization occurs only after semantic analysis and canonical IR.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * External formats such as:
 *
 *     OpenQASM
 *     QIR
 *     vendor formats
 *     simulator formats
 *     scientific data formats
 *
 * may map external uncertainty/measurement information into the canonical
 * semantic model.
 *
 * They do not become alternative Zamani uncertainty grammars.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics are inherited from:
 *
 *     UncertaintyExpressions
 *
 * This file adds no independent syntactic error category.
 *
 * Semantic diagnostics may include:
 *
 *     invalid quantum uncertainty value
 *     invalid quantum result type
 *     invalid measurement-derived uncertainty
 *     incompatible probability representation
 *     invalid distribution
 *     invalid confidence value
 *     insufficient capability
 *     insufficient resources
 *     prohibited effect
 *     policy violation
 *     contract violation
 *     unavailable quantum capability
 *
 * These must be produced by semantic/validation/resource/policy layers rather
 * than encoded as parser alternatives.
 *
 * ============================================================================
 * NEGATIVE-SPACE CONTRACT
 * ============================================================================
 *
 * The following constructs MUST NOT be added to this file:
 *
 *     quantumUncertaintyLiteral
 *     quantumProbabilityLiteral
 *     quantumDistributionLiteral
 *     quantumConfidenceLiteral
 *     quantumEvidenceLiteral
 *     quantumProvenanceLiteral
 *
 * unless a future language specification establishes a genuinely distinct
 * universal syntax and assigns this file explicit ownership.
 *
 * Likewise, do not add:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     QPU identifiers
 *     physical qubit identifiers
 *     fixed-width probability representations
 *     fixed sample counts
 *     target-specific hardware assumptions
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing universal uncertainty syntax remains authoritative.
 *
 * Therefore existing forms such as:
 *
 *     uncertain(value)
 *     uncertainty(value)
 *     uncertain(value, confidence: c)
 *     uncertain(value, probability: p)
 *     uncertainty(value, distribution: d)
 *     uncertainty(value, evidence: e)
 *     uncertainty(value, provenance: p)
 *
 * remain governed by:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * This file does not alter their syntax.
 *
 * ============================================================================
 * REQUIRED INTEGRATION CHANGES
 * ============================================================================
 *
 * The following integration changes are required for production use.
 *
 * 1. grammar/expressions/quantum.g4
 *
 *    Import:
 *
 *        QuantumUncertainty
 *
 *    and add:
 *
 *        | quantumUncertaintyExpression
 *
 *    to the canonical quantum-expression dispatch.
 *
 * 2. grammar/expressions/uncertainty.g4
 *
 *    Its token vocabulary should be aligned with the repository's canonical
 *    lexer:
 *
 *        tokenVocab = ZamaniLexer;
 *
 *    rather than maintaining a competing ZamaniTokens vocabulary.
 *
 * 3. grammar/ai/uncertainty.g4
 *
 *    Must consume the same canonical:
 *
 *        UncertaintyExpressions
 *
 *    and must not create an alternate uncertainty implementation.
 *
 * 4. grammar/expressions/expressions.g4
 *
 *    Must continue to own the universal expression hierarchy and must not
 *    directly duplicate quantum uncertainty syntax.
 *
 * 5. grammar/quantum/quantum.g4
 *
 *    Does NOT need to own uncertainty syntax.
 *
 *    Quantum uncertainty reaches the quantum domain through the canonical
 *    expression composition path.
 *
 * 6. grammar/quantum/measurement.g4
 *
 *    Remains the owner of measurement statement syntax.
 *
 *    It must not import this file merely to implement measurement.
 *
 * 7. grammar/quantum/operations.g4
 *
 *    Remains the owner of quantum operation syntax.
 *
 * 8. grammar/quantum/observables.g4
 *
 *    Remains the owner of observable syntax.
 *
 * 9. grammar/quantum/quantum-states.g4
 *
 *    Remains the owner of quantum-state syntax.
 *
 * 10. semantic quantum frontend
 *
 *     Must map the resulting AST into the existing canonical quantum semantic
 *     model and, where applicable, quantum::ir.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive integration tests include:
 *
 *     uncertain(measurement_result)
 *
 *     uncertainty(measurement_result)
 *
 *     uncertain(result, confidence: confidence_value)
 *
 *     uncertain(result, probability: probability_value)
 *
 *     uncertain(result, distribution: distribution_value)
 *
 *     uncertain(result, evidence: evidence_value)
 *
 *     uncertain(result, provenance: provenance_value)
 *
 *     uncertainty(
 *         measurement_result,
 *         confidence: confidence_value,
 *         evidence: evidence_value,
 *         provenance: provenance_value
 *     )
 *
 *     uncertainty(quantum_result)
 *
 *     uncertainty(observable_result)
 *
 *     uncertainty(simulation_result)
 *
 *     uncertainty(hybrid_result)
 *
 *     uncertainty(model_result)
 *
 * Required cross-domain tests include:
 *
 *     quantum -> uncertainty -> classical decision
 *
 *     classical -> uncertainty -> quantum parameter
 *
 *     measurement -> uncertainty -> reasoning
 *
 *     measurement -> uncertainty -> learning
 *
 *     quantum -> uncertainty -> adaptive execution
 *
 *     quantum -> uncertainty -> contract
 *
 *     quantum -> uncertainty -> policy
 *
 *     quantum -> uncertainty -> provenance
 *
 * Required open-world tests include:
 *
 *     uncertainty(result, custom_metadata: value)
 *
 *     uncertainty(result, future_property: value)
 *
 *     uncertainty(result, provider::property: value)
 *
 * where such qualified metadata is supported by the canonical uncertainty
 * expression contract.
 *
 * Required semantic-negative tests include:
 *
 *     invalid uncertainty type
 *
 *     invalid probability
 *
 *     incompatible distribution
 *
 *     invalid confidence
 *
 *     invalid measurement-derived value
 *
 *     unavailable quantum capability
 *
 *     insufficient resources
 *
 *     prohibited effect
 *
 *     policy violation
 *
 *     contract violation
 *
 * These are semantic tests, not parser hacks.
 *
 * Required scalability tests include:
 *
 *     many uncertainty metadata fields
 *     deeply composed uncertainty expressions
 *     large symbolic quantum result structures
 *     large measurement-derived result sets
 *     large hybrid expressions
 *     large provenance structures
 *
 * No scalability test may establish a universal maximum.
 *
 * Required determinism tests include:
 *
 *     identical source -> identical token sequence
 *     identical source -> equivalent parse tree
 *     identical source -> equivalent AST
 *
 * under the same language and dialect configuration.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_*
 *     fixed qubit count
 *     fixed sample count
 *     fixed probability width
 *     fixed distribution size
 *     fixed confidence precision
 *     fixed tensor rank
 *     fixed memory capacity
 *     fixed CPU count
 *     fixed GPU count
 *     fixed QPU count
 *     fixed topology
 *     physical device ID
 *     vendor-specific operation catalogue
 *     target-specific syntax
 *
 * The only structural repetition is grammar composition.
 *
 * ============================================================================
 * RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This ANTLR grammar contains no embedded Rust code.
 *
 * The generated parser/runtime integration must remain compatible with:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * This grammar must not require:
 *
 *     unsafe
 *     FFI-based parser actions
 *     target-specific code generation
 *     runtime callbacks
 *     hardware access
 *
 * during parsing.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE as an independent grammar component when:
 *
 * [x] It has exactly one parser grammar identity.
 *
 * [x] It uses the canonical Zamani lexer vocabulary.
 *
 * [x] It imports the canonical universal uncertainty grammar.
 *
 * [x] It defines no duplicate uncertainty syntax.
 *
 * [x] It owns only quantum uncertainty integration wrappers.
 *
 * [x] It introduces no new uncertainty keywords.
 *
 * [x] It introduces no new quantum gate catalogue.
 *
 * [x] It introduces no physical hardware assumptions.
 *
 * [x] It introduces no resource limits.
 *
 * [x] It introduces no target-specific limits.
 *
 * [x] It introduces no second AST.
 *
 * [x] It introduces no second quantum IR.
 *
 * [x] It preserves the quantum::ir boundary.
 *
 * [x] It preserves universal uncertainty semantics.
 *
 * [x] It preserves open-world uncertainty metadata.
 *
 * [x] It supports quantum/classical boundary use.
 *
 * [x] It supports AI/quantum composition through shared semantics.
 *
 * [x] It has explicit ownership.
 *
 * [x] It has explicit dependency direction.
 *
 * [x] It has explicit AST/semantic/type/effect/capability/resource contracts.
 *
 * [x] It has explicit policy/provenance contracts.
 *
 * [x] It has explicit integration instructions.
 *
 * [x] It has explicit scalability requirements.
 *
 * [x] It has an explicit hard-coding audit.
 *
 * [x] It has an explicit Rust safety contract.
 *
 * [ ] grammar/expressions/quantum.g4 imports QuantumUncertainty.
 *
 * [ ] canonical expression composition includes quantumUncertaintyExpression.
 *
 * [ ] expressions/uncertainty.g4 uses ZamaniLexer consistently.
 *
 * [ ] ANTLR generation succeeds for the complete grammar graph.
 *
 * [ ] Rust parser generation succeeds under the supported Rust toolchain.
 *
 * [ ] positive integration tests pass.
 *
 * [ ] negative semantic tests pass.
 *
 * [ ] boundary tests pass.
 *
 * [ ] scalability tests pass.
 *
 * [ ] determinism tests pass.
 *
 * [ ] cross-domain quantum/classical tests pass.
 *
 * [ ] quantum::ir lowering tests pass where applicable.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * There is ONE universal uncertainty language construct.
 *
 * There may be MANY domains that consume it.
 *
 * Therefore:
 *
 *     Universal uncertainty
 *              |
 *       +------+------+
 *       |             |
 *       v             v
 *     AI          Quantum
 *       |             |
 *       +------+------+
 *              |
 *              v
 *       shared semantic model
 *              |
 *              v
 *       domain-specific lowering
 *
 * The quantum domain never needs a separate uncertainty language.
 *
 * This is essential for POCO-REAF:
 *
 *     source meaning remains stable;
 *     domain semantics remain composable;
 *     target realization remains downstream;
 *     resource availability remains external;
 *     future hardware does not require a new uncertainty grammar.
 *
 * ============================================================================
 */