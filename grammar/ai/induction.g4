/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/induction.g4
 *
 * Grammar:
 *     Induction
 *
 * Status:
 *     PRODUCTION AI-DOMAIN INDUCTION COMPOSITION BOUNDARY
 *
 * Implementation baseline:
 *     Rust 1.97+
 *     Rust edition 2021
 *     Safe Rust only
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the AI-domain composition boundary for INDUCTIVE
 * REASONING.
 *
 * Induction is a reasoning mode in which a program expresses a desired
 * conclusion/target together with source observations, evidence, examples,
 * premises, or other expressions from which the semantic reasoning system may
 * derive a generalized result.
 *
 * IMPORTANT:
 *
 * This file does NOT implement an induction algorithm.
 *
 * It does not select or encode:
 *
 *     theorem provers
 *     rule engines
 *     statistical algorithms
 *     machine-learning algorithms
 *     probabilistic engines
 *     Bayesian implementations
 *     symbolic engines
 *     neural models
 *     quantum algorithms
 *     hardware
 *     accelerators
 *     schedulers
 *     distributed topology
 *     runtime workers
 *     physical devices
 *     backend implementations
 *
 * Those decisions belong downstream.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     universal reasoning
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 *     infer / deduce / reason        induction
 *       |                               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *                domain-neutral AST
 *                       |
 *                       v
 *                structural validation
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *        types       evidence     provenance
 *          |            |             |
 *          +------------+-------------+
 *                       |
 *                       v
 *                semantic reasoning
 *                       |
 *          +------------+-------------+
 *          |            |             |
 *          v            v             v
 *      classical     AI/model      quantum/hybrid
 *          |            |             |
 *          +------------+-------------+
 *                       |
 *                       v
 *                 canonical IR
 *                       |
 *              optimization/lowering
 *                       |
 *                target realization
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     aiInductionConstruct
 *     aiInductionStatement
 *
 * These are AI-domain composition aliases.
 *
 *
 * THIS FILE DOES NOT OWN:
 *
 *     induction algorithms
 *     induction semantics
 *     general reasoning semantics
 *     expressions
 *     expression precedence
 *     identifiers
 *     names
 *     types
 *     patterns
 *     guards
 *     evidence semantics
 *     provenance semantics
 *     contracts
 *     policies
 *     effects
 *     capabilities
 *     resources
 *     learning algorithms
 *     model definitions
 *     model training
 *     adaptation
 *     agents
 *     concurrency
 *     quantum operations
 *     HDL
 *     hardware
 *     distributed execution
 *     networking
 *     FFI
 *     ABI
 *     metaprogramming
 *     IR
 *     runtime execution
 *     backend realization
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be one universal source-level reasoning model.
 *
 * The canonical owner is:
 *
 *     grammar/statements/reason.g4
 *
 * with grammar identity:
 *
 *     ReasonStatements
 *
 * Therefore this file MUST NOT redefine:
 *
 *     reasonStatement
 *     reasoningOperator
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * Induction is an AI-domain semantic/composition concern, not a reason to
 * create another general reasoning grammar.
 *
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * The AI subsystem needs an explicit stable boundary for inductive reasoning
 * so that:
 *
 *     AI
 *       |
 *       +--> induction
 *
 * can be represented without making the universal reasoning grammar depend on
 * AI implementation details.
 *
 * This file therefore acts as a domain adapter.
 *
 * The actual reasoning syntax remains owned by:
 *
 *     grammar/statements/reason.g4
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/statements/reason.g4
 *
 * IMPORTS:
 *
 *     ReasonStatements
 *
 * EXPORTS:
 *
 *     aiInductionConstruct
 *     aiInductionStatement
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
 *     universal reasoning semantic subsystem
 *     plus AI semantic analysis where applicable
 *
 * IR_OWNER:
 *
 *     canonical semantic/domain IR layers
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/induction/
 *     grammar/tests/statements/reasoning/
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
 * The dependency direction is deliberately:
 *
 *     Expressions
 *          ^
 *          |
 *     ReasonStatements
 *          ^
 *          |
 *     Induction
 *          ^
 *          |
 *          AI
 *
 * This file MUST NOT import:
 *
 *     AI
 *     Statements
 *     ZamaniParser
 *
 * Doing so would create circular grammar composition.
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar defines NO lexer rules.
 *
 * Lexer ownership remains:
 *
 *     grammar/lexer/
 *     grammar/antlr/ZamaniLexer.g4
 *
 * If the language eventually introduces a dedicated lexical spelling for
 * induction, that token must be added to the canonical keyword vocabulary.
 *
 * It must NOT be defined inside this parser grammar.
 *
 *
 * ============================================================================
 * IMPORTANT SEMANTIC DISTINCTION
 * ============================================================================
 *
 * Induction is NOT the same thing as learning.
 *
 * Induction may be used to derive a generalized conclusion from observations
 * without training or mutating a persistent model.
 *
 * Learning may consume an inductive result, but:
 *
 *     induction != learning
 *
 * Likewise:
 *
 *     induction != inference
 *
 * and:
 *
 *     induction != adaptation
 *
 * The semantic layer may compose these operations, but they remain distinct
 * concepts.
 *
 *
 * ============================================================================
 * UNIVERSAL REASONING INTEGRATION
 * ============================================================================
 *
 * The universal reasoning subsystem already represents:
 *
 *     infer
 *     deduce
 *     reason
 *
 * using:
 *
 *     reasonStatement
 *
 * The AI induction boundary therefore consumes the same reasoning
 * representation rather than defining an independent parser model.
 *
 * This gives induction access to the same:
 *
 *     target
 *     source
 *     context
 *     expression
 *     source span
 *
 * representation.
 *
 *
 * ============================================================================
 * PUBLIC AI ENTRY
 * ============================================================================
 *
 * AI code sees induction through:
 *
 *     aiInductionConstruct
 *
 * This rule intentionally delegates directly to the universal reasoning
 * representation.
 *
 * No AI-specific syntax tree is created merely because the construct entered
 * through the AI grammar.
 *
 * ============================================================================
 */

parser grammar Induction;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * The universal reasoning grammar is the only semantic syntax dependency.
 *
 * ============================================================================
 */

import
    ReasonStatements
    ;


/*
 * ============================================================================
 * PUBLIC AI INDUCTION CONSTRUCT
 * ============================================================================
 *
 * This is the stable AI-domain composition boundary.
 *
 * The underlying syntax remains the canonical reasoning syntax.
 *
 * Consequently, induction-related semantic intent must be represented by the
 * semantic model rather than by duplicating the entire reasoning grammar.
 *
 * ============================================================================
 */

aiInductionConstruct
    : aiInductionStatement
    ;


/*
 * ============================================================================
 * AI INDUCTION STATEMENT
 * ============================================================================
 *
 * The parser accepts the canonical reasoning statement.
 *
 * Semantic analysis determines whether the selected reasoning operation is
 * inductive.
 *
 * This intentionally prevents an AI-only alternate syntax from competing with
 * the universal reasoning grammar.
 *
 * ============================================================================
 */

aiInductionStatement
    : reasonStatement
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * THIS FILE CREATES NO NEW AST HIERARCHY.
 *
 * In particular, it MUST NOT require:
 *
 *     AIInductionNode
 *     AIInductionExpression
 *     AIInductionIR
 *     AIModelInductionIR
 *
 * solely because the construct was parsed through this adapter.
 *
 * The domain-neutral AST must preserve the canonical reasoning representation
 * supplied by ReasonStatements.
 *
 * At minimum, the semantic representation must be able to retain:
 *
 *     reasoning kind
 *     target
 *     optional source
 *     ordered context
 *     source span
 *
 * If the semantic reasoning system identifies the operation as induction,
 * that information belongs in the semantic representation of the reasoning
 * operation.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     whether the operation is inductive;
 *     what observations/evidence are being generalized;
 *     whether the target is a valid inductive target;
 *     whether the source is a valid inductive source;
 *     whether sufficient evidence exists;
 *     whether the requested generalization is permitted;
 *     whether uncertainty must be represented;
 *     whether confidence/probability metadata applies;
 *     which model or reasoning engine may realize the operation;
 *     which capabilities are required;
 *     which resources are required;
 *     which effects are produced;
 *     which policies apply;
 *     which contracts apply;
 *     what provenance must be recorded;
 *     how the result is lowered.
 *
 * Parser acceptance does NOT imply semantic validity.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Inductive reasoning operates on ordinary Zamani expressions.
 *
 * Therefore it can participate in reasoning over:
 *
 *     scalar values
 *     records
 *     tuples
 *     collections
 *     streams
 *     tensors
 *     graphs
 *     datasets
 *     knowledge values
 *     probabilistic values
 *     uncertain values
 *     model results
 *     classical results
 *     quantum-derived measurements
 *     simulation results
 *     distributed results
 *     hardware observations
 *     future domain values
 *
 * The type system remains the sole authority for type validity.
 *
 * This grammar introduces no AI-specific type system.
 *
 *
 * ============================================================================
 * PATTERN / GUARD INTEGRATION
 * ============================================================================
 *
 * Induction may operate over patterns and guarded observations.
 *
 * Examples conceptually include:
 *
 *     observations matching a pattern
 *
 *     observations satisfying a guard
 *
 *     generalized results constrained by a property
 *
 * Pattern syntax remains owned by:
 *
 *     grammar/expressions/
 *
 * Guard semantics remain owned by the expression/semantic systems.
 *
 * This grammar does not duplicate either subsystem.
 *
 *
 * ============================================================================
 * EVIDENCE CONTRACT
 * ============================================================================
 *
 * Inductive reasoning commonly depends on evidence.
 *
 * Evidence may originate from:
 *
 *     observations
 *     datasets
 *     knowledge
 *     simulations
 *     classical computation
 *     quantum measurements
 *     hardware observations
 *     network results
 *     distributed computation
 *     learned models
 *     external services
 *
 * Evidence is represented by ordinary Zamani expressions and semantic
 * evidence/provenance structures.
 *
 * This grammar does not create an evidence-specific syntax tree.
 *
 *
 * ============================================================================
 * UNCERTAINTY CONTRACT
 * ============================================================================
 *
 * Inductive results may naturally be uncertain.
 *
 * The semantic model may therefore associate:
 *
 *     confidence
 *     probability
 *     likelihood
 *     distribution
 *     belief
 *     interval
 *     evidence strength
 *
 * with an inductive result.
 *
 * This grammar does NOT impose:
 *
 *     a fixed probability precision;
 *     a fixed floating-point representation;
 *     a fixed confidence scale;
 *     a fixed distribution size;
 *     a fixed numerical backend.
 *
 * Such decisions belong to the type and semantic systems.
 *
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Induction may provide information to learning.
 *
 * For example, an inductive result can become:
 *
 *     a candidate rule;
 *     a hypothesis;
 *     a model constraint;
 *     a training signal;
 *     a feature;
 *     an explanation;
 *     a policy input.
 *
 * However:
 *
 *     induction != training
 *
 * Training remains owned by:
 *
 *     grammar/ai/
 *
 * learning/training grammar.
 *
 * This file MUST NOT encode:
 *
 *     neural network architectures;
 *     optimizer names;
 *     batch sizes;
 *     accelerator types;
 *     framework names;
 *     model-specific training syntax.
 *
 *
 * ============================================================================
 * INFERENCE INTEGRATION
 * ============================================================================
 *
 * Induction may feed or consume inference.
 *
 * For example:
 *
 *     observations
 *         |
 *         v
 *     induction
 *         |
 *         v
 *     generalized hypothesis
 *         |
 *         v
 *     inference
 *
 * The inference grammar remains separately owned.
 *
 * No inference grammar is duplicated here.
 *
 *
 * ============================================================================
 * DEDUCTION INTEGRATION
 * ============================================================================
 *
 * Induction and deduction may compose:
 *
 *     observations
 *         |
 *         v
 *     inductive generalization
 *         |
 *         v
 *     premise/rule
 *         |
 *         v
 *     deduction
 *
 * Both remain members of the common reasoning semantic family.
 *
 * This file must not implement a second deduction grammar.
 *
 *
 * ============================================================================
 * CAUSAL REASONING INTEGRATION
 * ============================================================================
 *
 * Induction may operate over:
 *
 *     observations
 *     dependencies
 *     causes
 *     effects
 *     interventions
 *     counterfactual evidence
 *
 * Causal semantics remain owned by the causal reasoning subsystem.
 *
 * This grammar does not reserve causal algorithm names.
 *
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Inductive reasoning may consume or produce knowledge.
 *
 * Conceptual flow:
 *
 *     knowledge
 *        |
 *        v
 *     observations
 *        |
 *        v
 *     induction
 *        |
 *        v
 *     generalized knowledge
 *
 * Knowledge operations such as:
 *
 *     assert
 *     retract
 *     query
 *
 * remain owned by the knowledge subsystem.
 *
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * An inductive result may influence adaptive execution.
 *
 * However, parsing an inductive construct MUST NOT authorize:
 *
 *     self-modification;
 *     unrestricted model mutation;
 *     unrestricted strategy replacement;
 *     resource reallocation;
 *     code generation;
 *     deployment changes.
 *
 * Adaptation requires independent:
 *
 *     effect;
 *     capability;
 *     resource;
 *     policy;
 *     authorization;
 *     provenance
 *
 * analysis.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * The parser does not assign effects.
 *
 * Semantic analysis may derive effects including:
 *
 *     computation
 *     knowledge.read
 *     model.inference
 *     randomness
 *     learning
 *     measurement
 *     network
 *     distributed
 *     simulation
 *     foreign
 *     native
 *     reflection
 *
 * depending on the realization.
 *
 * An inductive statement that is semantically pure must remain capable of
 * being represented without unnecessary effects.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements are semantic and open-ended.
 *
 * Examples include:
 *
 *     capability("reasoning.induction")
 *     capability("reasoning")
 *     capability("knowledge.query")
 *     capability("probabilistic.compute")
 *     capability("model.inference")
 *
 * These are examples of semantic capability names, not a closed catalogue.
 *
 * This grammar MUST NOT enumerate:
 *
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     accelerators
 *     vendors
 *     devices
 *     processor generations
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Induction may require arbitrary:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     accelerator
 *     quantum
 *     distributed
 *     model
 *     data
 *
 * resources.
 *
 * Resource ownership remains under:
 *
 *     grammar/resources/
 *
 * This grammar does not duplicate:
 *
 *     requires
 *     capability
 *     constraint
 *     budget
 *     preference
 *     hint
 *     negotiation
 *
 * syntax.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Induction may be constrained by:
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
 * The parser merely preserves the reasoning operation so that semantic
 * validation can apply the relevant contracts.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Inductive reasoning may be controlled by:
 *
 *     evidence policy
 *     privacy policy
 *     security policy
 *     model policy
 *     execution policy
 *     determinism policy
 *     resource policy
 *     adaptation policy
 *
 * Policy evaluation remains outside the parser.
 *
 * Valid syntax does not imply authorization.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Induction is particularly provenance-sensitive because the semantic result
 * may be generalized from observations.
 *
 * Provenance should therefore be capable of preserving:
 *
 *     source
 *     observations
 *     evidence
 *     derivation
 *     reasoning operation
 *     model
 *     transformation
 *     verification
 *     decision
 *     policy
 *     language version
 *     semantic version
 *
 * The grammar does not implement provenance.
 *
 *
 * ============================================================================
 * EXPLANATION CONTRACT
 * ============================================================================
 *
 * An inductive result may later be explained through the universal explanation
 * and evidence systems.
 *
 * The explanation layer may expose:
 *
 *     contributing observations
 *     evidence
 *     derived hypothesis
 *     confidence
 *     uncertainty
 *     transformations
 *     decisions
 *
 * No explanation syntax is duplicated here.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Induction may consume quantum-derived values.
 *
 * For example:
 *
 *     quantum measurement
 *          |
 *          v
 *     observation
 *          |
 *          v
 *     induction
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * Quantum semantic lowering remains:
 *
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
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
 * This file introduces no quantum-specific IR.
 *
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Induction can participate in hybrid computation:
 *
 *     classical observations
 *          +
 *     quantum measurements
 *          +
 *     learned model results
 *          |
 *          v
 *     inductive reasoning
 *          |
 *          v
 *     classical decision
 *
 * The grammar remains domain-neutral.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Induction may consume:
 *
 *     simulation results
 *     verification results
 *     timing observations
 *     hardware observations
 *     synthesis information
 *     telemetry
 *
 * No hardware-specific syntax is introduced here.
 *
 * In particular, this file does NOT encode:
 *
 *     register widths
 *     bus widths
 *     fixed device counts
 *     fixed FPGA capacities
 *     physical addresses
 *     vendor topology
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Inductive evidence may be collected from:
 *
 *     actors
 *     services
 *     distributed tasks
 *     replicated observations
 *     collective computation
 *
 * Distribution remains owned by:
 *
 *     grammar/concurrency/
 *     grammar/distributed/
 *
 * This file introduces no distributed topology syntax.
 *
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Network-derived observations may participate as ordinary expressions.
 *
 * Network effects, authorization, security and resource requirements remain
 * downstream concerns.
 *
 *
 * ============================================================================
 * FFI / ABI INTEGRATION
 * ============================================================================
 *
 * Foreign-function results may be used as inductive evidence.
 *
 * FFI and ABI remain owned by:
 *
 *     grammar/interoperability/
 *
 * Foreign effects and capabilities remain independently checked.
 *
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Induction may operate over compile-time or reflective values where the
 * semantic system permits it.
 *
 * Compile-time execution and reflection remain owned by:
 *
 *     grammar/metaprogramming/
 *
 * This grammar performs no execution.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * It must depend only on:
 *
 *     token stream
 *     grammar version
 *     parser configuration
 *
 * It must NOT inspect:
 *
 *     hardware
 *     available resources
 *     system time
 *     randomness
 *     network state
 *     filesystem state
 *     runtime state
 *     scheduler state
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no language-level finite limit for:
 *
 *     observations
 *     evidence
 *     premises
 *     source expression size
 *     target expression size
 *     context expressions
 *     models
 *     datasets
 *     reasoning operations
 *     AI constructs
 *     quantum resources
 *     classical resources
 *     distributed resources
 *     hardware resources
 *
 * Repetition and composition are represented by normal grammar structures and
 * semantic values.
 *
 * No grammar-level machine capacity is introduced.
 *
 * "Infinity" therefore means:
 *
 *     no artificial language ceiling.
 *
 * It does not mean that a finite implementation has infinite physical
 * resources.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_INDUCTION_DEPTH
 *     MAX_INDUCTION_STEPS
 *     MAX_OBSERVATIONS
 *     MAX_EVIDENCE
 *     MAX_PREMISES
 *     MAX_HYPOTHESES
 *     MAX_MODELS
 *     MAX_DATASETS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also MUST NOT contain:
 *
 *     fixed hardware identifiers
 *     fixed vendor identifiers
 *     fixed topology
 *     fixed physical addresses
 *     fixed processor counts
 *     fixed quantum operation catalogues
 *     fixed accelerator counts
 *
 *
 * ============================================================================
 * NO APPLICATION-SPECIFIC KEYWORD EXPLOSION
 * ============================================================================
 *
 * This grammar MUST NOT add keywords for individual applications such as:
 *
 *     computer vision
 *     sentiment analysis
 *     robotics
 *     blockchain
 *     payments
 *     administration
 *     legal workflows
 *     virtual reality
 *     augmented reality
 *     individual AI frameworks
 *     individual ML algorithms
 *     individual hardware vendors
 *
 * Those belong to:
 *
 *     libraries
 *     dialects
 *     capabilities
 *     policies
 *     semantic extensions
 *     applications
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The semantic pipeline remains:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     canonical/domain IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing / scheduling
 *       |
 *       v
 *     resilience / recovery
 *       |
 *       v
 *     ZQN / HAL
 *       |
 *       v
 *     target realization
 *
 * Induction MUST NOT create a competing:
 *
 *     InductionIR
 *     AIInductionIR
 *     LearningIR
 *
 * merely because induction occurs in an AI context.
 *
 *
 * ============================================================================
 * RUST / SAFETY CONTRACT
 * ============================================================================
 *
 * This is an ANTLR grammar only.
 *
 * It contains:
 *
 *     no Rust;
 *     no embedded actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime execution.
 *
 * Generated and consuming Rust code must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * No `unsafe` implementation is required or permitted as part of this
 * grammar feature.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The existing reasoning grammar remains authoritative.
 *
 * Therefore this adapter does not change the canonical forms:
 *
 *     infer TARGET;
 *     deduce TARGET;
 *     reason TARGET;
 *
 *     infer TARGET from SOURCE;
 *     deduce TARGET from SOURCE;
 *     reason TARGET from SOURCE;
 *
 *     infer TARGET with (OPTION);
 *     deduce TARGET with (OPTION);
 *     reason TARGET with (OPTION);
 *
 *     infer TARGET from SOURCE with (OPTION);
 *     deduce TARGET from SOURCE with (OPTION);
 *     reason TARGET from SOURCE with (OPTION);
 *
 * Existing source programs remain structurally compatible.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     infer conclusion from observations;
 *
 *     deduce hypothesis from evidence;
 *
 *     reason generalization from dataset;
 *
 *     infer result from quantum_measurements;
 *
 *     reason result from model_output with (confidence);
 *
 *     deduce rule from observations with (evidence, provenance);
 *
 *
 * SEMANTIC POSITIVE:
 *
 *     inductive reasoning over scalar values;
 *     inductive reasoning over collections;
 *     inductive reasoning over tensors;
 *     inductive reasoning over graphs;
 *     inductive reasoning over datasets;
 *     inductive reasoning over probabilistic values;
 *     inductive reasoning over quantum-derived values;
 *     inductive reasoning over distributed results;
 *
 *
 * NEGATIVE:
 *
 *     inference without target;
 *     reasoning without target;
 *     malformed source;
 *     malformed context;
 *     empty context;
 *     trailing context comma;
 *     invalid clause ordering;
 *
 *
 * SCALABILITY:
 *
 *     arbitrarily large observation expressions;
 *     arbitrarily large evidence expressions;
 *     arbitrarily large context expressions;
 *     arbitrary model/data references;
 *     arbitrary symbolic resource requirements downstream;
 *     arbitrary target hardware downstream.
 *
 *
 * DETERMINISM:
 *
 *     identical source/token stream/parser configuration produces equivalent
 *     parse-tree structure.
 *
 *
 * CROSS-DOMAIN:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     AI/model;
 *     data;
 *     concurrency;
 *     distributed;
 *     networking;
 *     FFI;
 *     metaprogramming;
 *     simulation.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It has exactly one AI induction composition boundary.
 *
 * [x] It imports the canonical ReasonStatements grammar.
 *
 * [x] It does not duplicate reasoning syntax.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not define a competing type system.
 *
 * [x] It does not define an AI-specific expression hierarchy.
 *
 * [x] It does not define an induction algorithm.
 *
 * [x] It does not select a model implementation.
 *
 * [x] It does not select hardware.
 *
 * [x] It does not encode resource limits.
 *
 * [x] It does not encode physical topology.
 *
 * [x] It does not create a competing IR.
 *
 * [x] It preserves the domain-neutral AST boundary.
 *
 * [x] It preserves the universal semantic reasoning boundary.
 *
 * [x] It remains compatible with quantum::ir downstream.
 *
 * [x] It remains compatible with classical computation.
 *
 * [x] It remains compatible with hybrid computation.
 *
 * [x] It remains compatible with HDL/hardware computation.
 *
 * [x] It remains compatible with distributed computation.
 *
 * [x] It remains compatible with future computational domains.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It has explicit downstream integration contracts.
 *
 * [ ] AI grammar imports this grammar.
 *
 * [ ] AI grammar exposes aiInductionConstruct exactly once.
 *
 * [ ] AI conformance tests include induction.
 *
 * [ ] Semantic induction tests pass.
 *
 * [ ] Cross-domain induction tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * Induction is a semantic capability, not an application-specific language.
 *
 * The parser should preserve portable intent.
 *
 * The compiler decides how that intent is realized:
 *
 *     symbolic
 *     statistical
 *     probabilistic
 *     learned
 *     classical
 *     quantum-assisted
 *     distributed
 *     hardware-accelerated
 *     simulated
 *     future
 *
 * according to:
 *
 *     types
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     target feasibility
 *
 * That separation is what keeps induction compatible with POCO-REAF.
 *
 * ============================================================================
 */