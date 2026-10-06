/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/feedback.g4
 *
 * GRAMMAR
 * -------
 * AIFeedback
 *
 * STATUS
 * ------
 * CANONICAL AI-DOMAIN FEEDBACK COMPOSITION GRAMMAR
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97 or later
 * Rust 2021
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
 * This file owns the canonical AI-domain source syntax for explicitly
 * supplying feedback to a computational target.
 *
 * Feedback is a universal computational signal.
 *
 * It may originate from:
 *
 *     - an observed result;
 *     - an evaluation;
 *     - a measurement;
 *     - a human or external evaluator;
 *     - a learned model;
 *     - a simulation;
 *     - a classical computation;
 *     - a quantum computation;
 *     - a hybrid computation;
 *     - a distributed computation;
 *     - hardware observations;
 *     - resource observations;
 *     - policy evaluation;
 *     - verification;
 *     - evidence;
 *     - another Zamani computation;
 *     - a future computational domain.
 *
 * The grammar describes the STRUCTURE of feedback intent.
 *
 * It does not decide:
 *
 *     - how feedback is generated;
 *     - whether feedback is trustworthy;
 *     - whether feedback is positive or negative;
 *     - whether feedback is numerical;
 *     - whether feedback is probabilistic;
 *     - whether feedback changes a model;
 *     - whether feedback changes a strategy;
 *     - whether feedback triggers adaptation;
 *     - whether feedback triggers learning;
 *     - whether feedback changes execution;
 *     - whether feedback changes quantum execution;
 *     - whether feedback changes hardware;
 *     - whether feedback is authorized;
 *     - whether feedback is safe;
 *     - whether feedback is accepted;
 *     - whether feedback is persisted.
 *
 * Those are semantic, policy, effect, provenance, execution and runtime
 * responsibilities.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                         canonical parser
 *                              |
 *                              v
 *                              AI
 *                              |
 *                              v
 *                        AIFeedback
 *                              |
 *                              v
 *                    feedbackConstruct
 *                              |
 *                              v
 *                    domain-neutral AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *            types          effects        provenance
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                       semantic feedback
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      learning           adaptation          evaluation
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                     capabilities/resources
 *                              |
 *                              v
 *                            policy
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *         classical       quantum::ir      other domain IR
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                         specialization
 *                              |
 *                           lowering
 *                              |
 *                     routing / scheduling
 *                              |
 *                       resilience / QEC
 *                              |
 *                            ZQN
 *                              |
 *                            HAL
 *                              |
 *                         target runtime
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Feedback is DATA and SEMANTIC INTENT.
 *
 * It is not an algorithm catalogue.
 *
 * It is not a hardware-selection mechanism.
 *
 * It is not an implicit authorization to mutate anything.
 *
 * It is not equivalent to learning.
 *
 * It is not equivalent to adaptation.
 *
 * It is an input that semantic systems may use for:
 *
 *     evaluation
 *     learning
 *     adaptation
 *     control
 *     optimization
 *     resilience
 *     decision making
 *     verification
 *
 * A feedback value therefore remains an ordinary Zamani expression.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     feedbackConstruct
 *     feedbackTarget
 *     feedbackSourceClause
 *     feedbackContextClause
 *     feedbackContextList
 *     feedbackContext
 *
 * These rules form the AI-domain feedback composition boundary.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - the FEEDBACK lexer token;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - expression precedence;
 *     - literals;
 *     - types;
 *     - learning syntax;
 *     - training syntax;
 *     - adaptation syntax;
 *     - reasoning syntax;
 *     - inference syntax;
 *     - knowledge syntax;
 *     - query syntax;
 *     - uncertainty syntax;
 *     - probability syntax;
 *     - explanation syntax;
 *     - evidence syntax;
 *     - provenance syntax;
 *     - contract syntax;
 *     - policy syntax;
 *     - capability syntax;
 *     - resource syntax;
 *     - actor syntax;
 *     - concurrency syntax;
 *     - distributed syntax;
 *     - quantum syntax;
 *     - HDL syntax;
 *     - hardware syntax;
 *     - simulation syntax;
 *     - effect syntax;
 *     - AST implementation;
 *     - semantic implementation;
 *     - model implementation;
 *     - learning algorithms;
 *     - adaptation algorithms;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one AI-domain grammar boundary for the feedback
 * statement.
 *
 * This file owns:
 *
 *     feedbackConstruct
 *
 * It does NOT own the universal:
 *
 *     statement
 *
 * rule.
 *
 * The universal statement grammar remains responsible for ordinary
 * statement-level composition.
 *
 * ============================================================================
 * WHY FEEDBACK IS A SEPARATE CONSTRUCT
 * ============================================================================
 *
 * The repository already uses feedback as a semantic concept in:
 *
 *     learning
 *     adaptation
 *     quantum resilience
 *     hybrid execution
 *     classical control
 *     evaluation
 *
 * Giving feedback an explicit source-level construct allows a program to
 * communicate feedback without forcing it to masquerade as:
 *
 *     learn
 *     adapt
 *     reason
 *     assert
 *     query
 *
 * However, feedback MUST NOT create separate semantic systems for each domain.
 *
 * All feedback ultimately becomes a common semantic feedback value/event/
 * relationship as determined by the semantic layer.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Direct parser dependencies:
 *
 *     grammar/expressions/expressions.g4
 *     grammar/core/names.g4
 *
 * Grammar names:
 *
 *     Expressions
 *     Names
 *
 * Lexer dependency:
 *
 *     ZamaniLexer
 *
 * The expression grammar owns:
 *
 *     expression
 *
 * and the names grammar owns canonical naming constructs.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PUBLIC:
 *
 *     feedbackConstruct
 *
 * PRIVATE:
 *
 *     feedbackTarget
 *     feedbackSourceClause
 *     feedbackContextClause
 *     feedbackContextList
 *     feedbackContext
 *
 * The public rule is intended for:
 *
 *     grammar/ai/ai.g4
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar contains NO lexer rules.
 *
 * The canonical lexical authority already defines:
 *
 *     FEEDBACK : 'feedback' ;
 *
 * in:
 *
 *     grammar/lexer/keywords.g4
 *
 * Therefore this file MUST NOT define:
 *
 *     FEEDBACK
 *     Feedback
 *     feedbackKeyword
 *     AI_FEEDBACK
 *
 * or another spelling for the same keyword.
 *
 * The lexical pipeline remains:
 *
 *     grammar/lexer/keywords.g4
 *             |
 *             v
 *     grammar/lexer/tokens.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniLexer.g4
 *             |
 *             v
 *          ZamaniLexer
 *             |
 *             v
 *        AIFeedback
 *
 * ============================================================================
 * SURFACE SYNTAX
 * ============================================================================
 *
 * Canonical minimal form:
 *
 *     feedback TARGET;
 *
 * Canonical source form:
 *
 *     feedback TARGET from SOURCE;
 *
 * Canonical contextual form:
 *
 *     feedback TARGET with (CONTEXT);
 *
 * Canonical combined form:
 *
 *     feedback TARGET from SOURCE with (CONTEXT, CONTEXT);
 *
 * Examples:
 *
 *     feedback model;
 *
 *     feedback model from outcome;
 *
 *     feedback controller from measurement;
 *
 *     feedback strategy from evaluation;
 *
 *     feedback model from observation with (confidence);
 *
 *     feedback strategy from reward with (policy, provenance);
 *
 *     feedback execution from measurement with (resource_state);
 *
 *     feedback quantum_strategy from quantum_result with (evidence);
 *
 * The grammar deliberately does not reserve:
 *
 *     model
 *     controller
 *     strategy
 *     outcome
 *     measurement
 *     evaluation
 *     observation
 *     reward
 *     confidence
 *     policy
 *     provenance
 *
 * as feedback-specific identifiers.
 *
 * They are ordinary expressions.
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * The target is mandatory.
 *
 * Therefore:
 *
 *     feedback;
 *
 * is invalid.
 *
 * The target is an ordinary Zamani expression.
 *
 * It may semantically denote:
 *
 *     - a model;
 *     - a strategy;
 *     - a controller;
 *     - an execution;
 *     - a decision;
 *     - a policy;
 *     - a knowledge value;
 *     - a learned value;
 *     - a quantum computation;
 *     - a classical computation;
 *     - a hybrid computation;
 *     - an agent;
 *     - an actor;
 *     - a hardware intent;
 *     - a distributed computation;
 *     - a future-domain value.
 *
 * This grammar does not determine which interpretation applies.
 *
 * ============================================================================
 * SOURCE CONTRACT
 * ============================================================================
 *
 * The optional `from` clause identifies the source of feedback.
 *
 * Examples:
 *
 *     feedback model from outcome;
 *     feedback strategy from observation;
 *     feedback controller from measurement;
 *     feedback execution from simulation_result;
 *     feedback model from evaluation;
 *
 * The source is an ordinary expression.
 *
 * It may represent:
 *
 *     - scalar data;
 *     - structured data;
 *     - tensor data;
 *     - probability;
 *     - distribution;
 *     - confidence;
 *     - evidence;
 *     - measurement;
 *     - query result;
 *     - reasoning result;
 *     - learned result;
 *     - simulation result;
 *     - execution result;
 *     - hardware observation;
 *     - resource observation;
 *     - distributed result;
 *     - another feedback value.
 *
 * No feedback data representation is hard-coded here.
 *
 * ============================================================================
 * CONTEXT CONTRACT
 * ============================================================================
 *
 * The optional `with` clause supplies additional feedback context.
 *
 * Examples:
 *
 *     feedback model with (confidence);
 *
 *     feedback strategy from evaluation with (evidence);
 *
 *     feedback controller from measurement with
 *         (confidence, provenance);
 *
 * Context entries are ordinary expressions.
 *
 * The grammar does not enumerate context types.
 *
 * ============================================================================
 * CLAUSE ORDER
 * ============================================================================
 *
 * Canonical order:
 *
 *     TARGET
 *     SOURCE?
 *     CONTEXT?
 *
 * Valid:
 *
 *     feedback target;
 *
 *     feedback target from source;
 *
 *     feedback target with (context);
 *
 *     feedback target from source with (context);
 *
 * Invalid:
 *
 *     feedback target with (context) from source;
 *
 * This provides one canonical source representation.
 *
 * ============================================================================
 * CONTEXT LIST
 * ============================================================================
 *
 * A context list contains one or more expressions.
 *
 * There is no fixed context-count ceiling.
 *
 * Valid:
 *
 *     with (confidence)
 *
 *     with (confidence, evidence)
 *
 *     with (confidence, evidence, provenance)
 *
 * Invalid:
 *
 *     with ()
 *
 *     with (a,)
 *
 *     with (,a)
 *
 *     with (a,,b)
 *
 * A trailing comma is deliberately rejected.
 *
 * ============================================================================
 * NO FEEDBACK ALGORITHM CATALOGUE
 * ============================================================================
 *
 * This grammar MUST NOT enumerate:
 *
 *     reward
 *     loss
 *     score
 *     gradient
 *     human
 *     reinforcement
 *     evaluation
 *     classification
 *     regression
 *     ranking
 *     Bayesian
 *     heuristic
 *     evolutionary
 *
 * as feedback grammar alternatives.
 *
 * Such concepts are values, algorithms, libraries, dialects, semantic
 * capabilities, or policies.
 *
 * The source grammar remains open-world.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Feedback can be consumed by learning.
 *
 * Example:
 *
 *     feedback model from evaluation;
 *
 *     learn model from feedback;
 *
 * The two operations remain semantically distinct:
 *
 *     feedback
 *         |
 *         v
 *     learning input
 *
 * `feedback.g4` MUST NOT define:
 *
 *     learn
 *     train
 *     fit
 *     fine_tune
 *     reinforcement
 *
 * syntax.
 *
 * The canonical learning statement remains owned by:
 *
 *     grammar/statements/learn.g4
 *
 * and AI learning composition remains owned by:
 *
 *     grammar/ai/learning.g4
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Feedback can drive controlled adaptation.
 *
 * Example:
 *
 *     feedback strategy from evaluation;
 *
 *     adapt strategy from feedback;
 *
 * Feedback does not itself imply adaptation.
 *
 * Adaptation remains owned by:
 *
 *     grammar/statements/adapt.g4
 *
 * and its corresponding semantic system.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Feedback may be derived from:
 *
 *     infer
 *     deduce
 *     reason
 *
 * results.
 *
 * This grammar does not import or duplicate reasoning syntax.
 *
 * Example semantic flow:
 *
 *     reason
 *       |
 *       v
 *     result
 *       |
 *       v
 *     feedback
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Feedback may consume:
 *
 *     query results
 *     knowledge values
 *     facts
 *     evidence
 *
 * through ordinary expressions.
 *
 * Knowledge syntax remains owned by the existing knowledge subsystem.
 *
 * No knowledge grammar is duplicated here.
 *
 * ============================================================================
 * UNCERTAINTY / PROBABILITY INTEGRATION
 * ============================================================================
 *
 * Feedback may contain:
 *
 *     probability
 *     distribution
 *     confidence
 *     uncertainty
 *     belief
 *     likelihood
 *
 * values.
 *
 * The grammar does not impose:
 *
 *     numeric precision;
 *     floating-point width;
 *     probability representation;
 *     distribution size;
 *     tensor rank;
 *     confidence representation.
 *
 * Those are type and semantic concerns.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Feedback can be accompanied by evidence:
 *
 *     feedback model from observation with (evidence);
 *
 * Evidence semantics remain owned by the evidence/provenance subsystem.
 *
 * The grammar does not create:
 *
 *     FeedbackEvidence
 *
 * as a second evidence model.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Feedback may be important for reproducibility, auditing and learning
 * lineage.
 *
 * Semantic provenance may record:
 *
 *     feedback source
 *     target
 *     context
 *     originating computation
 *     evidence
 *     evaluation
 *     timestamp
 *     version
 *     authorization
 *     policy
 *     transformation
 *
 * This grammar only preserves the syntactic structure needed for downstream
 * provenance.
 *
 * It does not define a second provenance system.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing has no effects.
 *
 * Semantic feedback may involve effects such as:
 *
 *     learning
 *     mutation
 *     io
 *     network
 *     distributed
 *     measurement
 *     simulation
 *     foreign
 *     native
 *
 * depending on the resolved source and target.
 *
 * Feedback itself does NOT automatically imply any particular effect.
 *
 * Effect ownership remains with:
 *
 *     grammar/effects/
 *
 * and the semantic effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Feedback may require capabilities such as:
 *
 *     capability("feedback")
 *     capability("learning")
 *     capability("evaluation")
 *     capability("measurement")
 *     capability("data.read")
 *     capability("model.update")
 *     capability("quantum.measurement")
 *
 * These are semantic capability identities.
 *
 * This grammar does not create a closed capability catalogue.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Feedback may consume arbitrary resources.
 *
 * Examples include:
 *
 *     compute
 *     memory
 *     storage
 *     network
 *     accelerator
 *     quantum resources
 *     distributed resources
 *     data resources
 *
 * Resource requirements are determined downstream.
 *
 * This grammar MUST NOT define limits for:
 *
 *     feedback values
 *     feedback events
 *     context entries
 *     models
 *     datasets
 *     tensors
 *     tensor rank
 *     parameters
 *     workers
 *     threads
 *     nodes
 *     devices
 *     accelerators
 *     qubits
 *     memory
 *     network size
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Feedback may be governed by generic contracts:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax is not duplicated here.
 *
 * Semantic validation determines:
 *
 *     - whether feedback is valid;
 *     - whether its source satisfies required conditions;
 *     - whether its target accepts the feedback;
 *     - whether guarantees remain valid.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Feedback may be controlled by policies governing:
 *
 *     - source trust;
 *     - data access;
 *     - privacy;
 *     - model updates;
 *     - adaptation;
 *     - network use;
 *     - external input;
 *     - authorization;
 *     - reproducibility;
 *     - provenance;
 *     - resource use.
 *
 * A syntactically valid feedback construct does not imply authorization.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Feedback input may originate outside the program.
 *
 * Therefore the semantic/runtime layer MUST treat feedback according to the
 * applicable trust and security policy.
 *
 * This grammar performs no:
 *
 *     - network access;
 *     - file access;
 *     - external input;
 *     - authentication;
 *     - authorization;
 *     - sandbox escape;
 *     - model mutation;
 *     - hardware access.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Feedback may originate from quantum computation.
 *
 * Example:
 *
 *     feedback controller from measurement_result;
 *
 * or:
 *
 *     feedback model from quantum_result with (confidence);
 *
 * The grammar does not define quantum operations.
 *
 * If the semantic operation produces or consumes quantum computation, the
 * canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * The path is:
 *
 *     feedback
 *         |
 *         v
 *     domain-neutral semantic operation
 *         |
 *         v
 *     quantum semantic operation
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     decomposition
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     QEC / resilience
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL
 *
 * This grammar never handles:
 *
 *     physical qubit identifiers
 *     physical coupling maps
 *     calibration
 *     routing
 *     scheduling
 *     QEC implementation
 *     backend selection.
 *
 * ============================================================================
 * HYBRID CONTRACT
 * ============================================================================
 *
 * Feedback is particularly important for hybrid computation.
 *
 * A semantic flow may be:
 *
 *     quantum measurement
 *           |
 *           v
 *     classical value
 *           |
 *           v
 *     feedback
 *           |
 *           v
 *     classical decision
 *           |
 *           v
 *     adaptation / execution
 *
 * The grammar remains target-neutral throughout.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Feedback may consume hardware observations or simulation results.
 *
 * Example:
 *
 *     feedback controller from sensor_result;
 *
 *     feedback hardware_policy from timing_observation;
 *
 * The grammar does not encode:
 *
 *     register width
 *     wire width
 *     physical port count
 *     FPGA resource count
 *     ASIC cell count
 *     device identifier
 *     clock frequency
 *     physical address.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Feedback may originate from distributed execution:
 *
 *     feedback model from distributed_evaluation;
 *
 *     feedback strategy from cluster_observation;
 *
 * Distribution remains owned by the distributed/concurrency/execution
 * subsystems.
 *
 * No worker, process, replica, node or partition count is encoded here.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION CONTRACT
 * ============================================================================
 *
 * Feedback is one possible input to adaptive execution.
 *
 * Semantic flow:
 *
 *     observation
 *          |
 *          v
 *       feedback
 *          |
 *          v
 *       evaluate
 *          |
 *          v
 *       policy
 *          |
 *          v
 *       select / retry / recover / adapt
 *
 * The existing resilience model remains downstream.
 *
 * States such as:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * are NOT parser alternatives here.
 *
 * Likewise outcomes such as:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * remain semantic/runtime concepts.
 *
 * ============================================================================
 * REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Runtime feedback may be nondeterministic because its source may involve:
 *
 *     sensors
 *     network input
 *     concurrent execution
 *     randomized computation
 *     quantum measurement
 *     external evaluation.
 *
 * Reproducibility controls therefore belong to:
 *
 *     execution
 *     provenance
 *     policy
 *     runtime
 *
 * and are not hard-coded into this grammar.
 *
 * A reproducible feedback record should be able to retain sufficient
 * provenance to identify:
 *
 *     source
 *     target
 *     context
 *     input version
 *     originating computation
 *     semantic/toolchain version
 *     relevant execution configuration.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser creates parser contexts only.
 *
 * The domain-neutral frontend AST owns the semantic representation.
 *
 * The AST should preserve:
 *
 *     - source span;
 *     - feedback target;
 *     - optional feedback source;
 *     - ordered context expressions;
 *     - statement boundary;
 *     - source ordering.
 *
 * The AST MUST NOT contain:
 *
 *     - GPU identifiers;
 *     - CPU identifiers;
 *     - QPU identifiers;
 *     - physical qubit identifiers;
 *     - device allocation;
 *     - routing;
 *     - scheduling;
 *     - backend selection;
 *     - hardware topology.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - target resolution;
 *     - source resolution;
 *     - context resolution;
 *     - type compatibility;
 *     - feedback validity;
 *     - evidence validity;
 *     - confidence semantics;
 *     - provenance;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - contracts;
 *     - policies;
 *     - authorization;
 *     - reproducibility;
 *     - whether learning may consume the feedback;
 *     - whether adaptation may consume the feedback;
 *     - whether execution may consume the feedback.
 *
 * Parser acceptance MUST NOT depend on current hardware availability.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Feedback is lowered through the domain-neutral semantic model.
 *
 * Possible downstream realizations include:
 *
 *     classical computation
 *     AI learning
 *     AI inference
 *     adaptation
 *     distributed execution
 *     hybrid execution
 *     quantum computation
 *     HDL/hardware control
 *     simulation
 *     future computational domains.
 *
 * If quantum computation is involved, the quantum portion must enter:
 *
 *     quantum::ir
 *
 * before quantum-specific optimization and realization.
 *
 * There is no:
 *
 *     FeedbackIR
 *     AIFeedbackIR
 *     QuantumFeedbackIR
 *
 * introduced by this grammar.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Feedback syntax is independent of machine scale.
 *
 * The same source:
 *
 *     feedback model from evaluation;
 *
 * can remain syntactically identical when realized on:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore CPUs
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
 *     future computational substrates.
 *
 * The grammar imposes no physical machine-size ceiling.
 *
 * "Scale to infinity given available resources" means:
 *
 *     no artificial finite machine or data capacity is encoded by this
 *     grammar.
 *
 * Actual feasibility is determined by:
 *
 *     target capabilities
 *     available resources
 *     compiler resources
 *     runtime resources
 *     policies
 *     contracts
 *     physical feasibility.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains NO fixed capacity constants.
 *
 * It does not define:
 *
 *     maximum feedback count
 *     maximum context count
 *     maximum model count
 *     maximum dataset size
 *     maximum tensor rank
 *     maximum parameter count
 *     maximum thread count
 *     maximum worker count
 *     maximum node count
 *     maximum accelerator count
 *     maximum device count
 *     maximum qubit count
 *     maximum memory
 *     maximum network size
 *
 * It also contains no:
 *
 *     fixed device IDs
 *     fixed CPU IDs
 *     fixed GPU IDs
 *     fixed QPU IDs
 *     fixed physical topology.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Feedback sources and context values are open-world.
 *
 * New feedback-producing systems do not require a grammar change if their
 * values can be represented by ordinary Zamani expressions.
 *
 * This includes future:
 *
 *     sensors
 *     models
 *     evaluators
 *     quantum systems
 *     hardware systems
 *     distributed systems
 *     simulation systems
 *     scientific systems
 *     computational substrates.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical spelling is:
 *
 *     feedback
 *
 * No alternative spellings are introduced here.
 *
 * Existing source constructs such as:
 *
 *     adapt strategy from feedback;
 *
 * remain valid because `feedback` can be an ordinary identifier only where
 * lexical context permits it. Since `feedback` is already a reserved keyword,
 * semantic/parser consumers that require a value named `feedback` should use
 * the appropriate canonical binding/reference mechanism rather than assuming
 * that the keyword is an identifier.
 *
 * This is intentional:
 *
 *     `feedback` as a keyword denotes the feedback construct;
 *
 * while feedback data values should normally be bound to ordinary identifiers,
 * for example:
 *
 *     let feedback_value = observation;
 *     adapt strategy from feedback_value;
 *
 * This prevents keyword/identifier ambiguity.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Supported feedback source must survive:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter/printer
 *       ->
 *     parser
 *
 * while preserving:
 *
 *     feedback operation
 *     target
 *     source
 *     context order
 *     statement boundary.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical:
 *
 *     source text
 *     lexer version
 *     grammar version
 *     parser configuration
 *     enabled dialect configuration
 *
 * this grammar must produce equivalent parse-tree structure.
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     resources
 *     filesystem state
 *     network state
 *     runtime state
 *     scheduler state
 *     wall-clock time
 *     randomness.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no Rust actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *     no unsafe Rust.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * STRUCTURAL PARSER ERRORS include:
 *
 *     feedback
 *
 *     feedback;
 *
 *     feedback from source;
 *
 *     feedback with (context);
 *
 *     feedback target from;
 *
 *     feedback target with;
 *
 *     feedback target with ();
 *
 *     feedback target with (a,);
 *
 *     feedback target with (,a);
 *
 *     feedback target with (a,,b);
 *
 *     feedback target with (a) from source;
 *
 *     feedback target from source from other;
 *
 * SEMANTIC ERRORS include:
 *
 *     target does not accept feedback
 *     source type is incompatible
 *     feedback is unauthorized
 *     required capability unavailable
 *     required resource unavailable
 *     policy violated
 *     contract violated
 *     provenance requirement unsatisfied
 *     evidence invalid
 *     feedback rejected by target semantics
 *
 * Semantic errors MUST NOT be encoded as parser alternatives.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 *     feedback model;
 *
 *     feedback model from outcome;
 *
 *     feedback strategy from evaluation;
 *
 *     feedback controller from measurement;
 *
 *     feedback model with (confidence);
 *
 *     feedback model from observation with (evidence);
 *
 *     feedback strategy from reward with (policy, provenance);
 *
 *     feedback execution from measurement with (confidence, evidence);
 *
 *     feedback quantum_strategy from quantum_result with (provenance);
 *
 *     feedback distributed_strategy from distributed_result;
 *
 *     feedback hardware_controller from simulation_result;
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     feedback
 *
 *     feedback;
 *
 *     feedback from source;
 *
 *     feedback with (context);
 *
 *     feedback target from;
 *
 *     feedback target with;
 *
 *     feedback target with ();
 *
 *     feedback target with (,);
 *
 *     feedback target with (a,);
 *
 *     feedback target with (,a);
 *
 *     feedback target with (a,,b);
 *
 *     feedback target with (a) from source;
 *
 *     feedback target from source from other;
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Target:
 *
 *     feedback identifier;
 *
 *     feedback qualified.name;
 *     feedback object.member;
 *     feedback collection[index];
 *     feedback call(argument);
 *     feedback nested_expression;
 *
 * Source:
 *
 *     feedback target from identifier;
 *     feedback target from call(argument);
 *     feedback target from measurement_result;
 *     feedback target from simulation_result;
 *     feedback target from distributed_result;
 *     feedback target from quantum_result;
 *
 * Context:
 *
 *     feedback target with (policy);
 *     feedback target with (evidence);
 *     feedback target with (confidence, provenance);
 *     feedback target with (policy, evidence, provenance);
 *
 * CROSS-DOMAIN TESTS
 * -----------------
 *
 *     feedback classical_model from classical_evaluation;
 *
 *     feedback quantum_strategy from quantum_result;
 *
 *     feedback hybrid_controller from measurement_result;
 *
 *     feedback hardware_strategy from simulation_result;
 *
 *     feedback distributed_model from distributed_evaluation;
 *
 *     feedback ai_model from inference_result;
 *
 *     feedback execution_plan from resource_observation;
 *
 * LEARNING TESTS
 * -------------
 *
 *     feedback model from evaluation;
 *     learn model from feedback_value;
 *
 * ADAPTATION TESTS
 * ---------------
 *
 *     feedback strategy from evaluation;
 *     adapt strategy from feedback_value;
 *
 * QUANTUM TESTS
 * -------------
 *
 *     feedback controller from measurement_result;
 *
 *     feedback model from quantum_result with (confidence);
 *
 * HYBRID TESTS
 * ------------
 *
 *     feedback classical_controller from quantum_measurement;
 *
 *     feedback quantum_strategy from classical_evaluation;
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Increase:
 *
 *     target expression complexity;
 *     source expression complexity;
 *     context count;
 *     expression nesting;
 *     statement count;
 *     program size;
 *
 * without changing this grammar.
 *
 * No finite machine-size constant is introduced.
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * Verify that:
 *
 *     feedback model from evaluation;
 *
 * has identical source syntax independent of:
 *
 *     CPU
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
 *     future target.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Parse identical feedback source repeatedly and verify equivalent:
 *
 *     operation
 *     target
 *     source
 *     context order
 *     source spans.
 *
 * COMPATIBILITY TESTS
 * -------------------
 *
 * Verify that:
 *
 *     adapt strategy from feedback_value;
 *
 * remains valid.
 *
 * Verify that:
 *
 *     mind.adapt(strategy, feedback_value)
 *
 * remains an expression.
 *
 * Verify that learning, adaptation and feedback remain separate semantic
 * operations.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. AI COMPOSITION
 * -----------------
 *
 * `grammar/ai/ai.g4` must:
 *
 *     import AIFeedback
 *
 * and add:
 *
 *     feedbackConstruct
 *
 * to:
 *
 *     aiConstruct
 *
 * exactly once.
 *
 * Conceptually:
 *
 *     aiConstruct
 *         : ...
 *         | feedbackConstruct
 *         | ...
 *         ;
 *
 * No duplicate feedback production should be added to `ai.g4`.
 *
 * 2. ROOT PARSER
 * --------------
 *
 * The canonical root parser must continue to reach AI through the existing
 * AI composition boundary.
 *
 * `grammar/Zamani.g4` must NOT import this leaf grammar directly if the
 * existing root architecture already reaches AI through `AI`.
 *
 * 3. STATEMENT COMPOSITION
 * ------------------------
 *
 * This file intentionally does NOT own the universal `statement` rule.
 *
 * If AI-domain constructs are dispatched through the AI parser boundary,
 * `feedbackConstruct` is consumed there.
 *
 * If the repository's statement dispatcher requires every statement to be
 * independently registered, the dispatcher must route through the AI boundary
 * rather than duplicating:
 *
 *     FEEDBACK ...
 *
 * in another grammar.
 *
 * 4. LEXER
 * --------
 *
 * No lexer changes are required for the keyword itself because the repository
 * already owns:
 *
 *     FEEDBACK : 'feedback' ;
 *
 * in:
 *
 *     grammar/lexer/keywords.g4
 *
 * 5. LEARNING
 * -----------
 *
 * `grammar/ai/learning.g4` remains the AI learning composition boundary.
 *
 * Feedback is consumed as an input/result of learning through semantic
 * composition or ordinary expressions.
 *
 * No learning production is duplicated here.
 *
 * 6. ADAPTATION
 * -------------
 *
 * `grammar/statements/adapt.g4` remains the canonical adaptation statement
 * owner.
 *
 * Feedback may feed adaptation:
 *
 *     feedback strategy from evaluation;
 *     adapt strategy from feedback_value;
 *
 * No adaptation grammar is duplicated here.
 *
 * 7. EFFECTS
 * ----------
 *
 * Feedback semantics must integrate with:
 *
 *     grammar/effects/learning.g4
 *     grammar/effects/adaptation.g4
 *     shared effect analysis
 *
 * This file does not define effect syntax.
 *
 * 8. RESOURCES / CAPABILITIES
 * ---------------------------
 *
 * Feedback semantic analysis integrates with:
 *
 *     grammar/resources/
 *
 * and capability resolution.
 *
 * This file does not perform resource negotiation.
 *
 * 9. CONTRACTS
 * -----------
 *
 * Generic contracts remain owned by:
 *
 *     grammar/validation/
 *
 * Feedback does not create another contract language.
 *
 * 10. POLICY
 * ----------
 *
 * Feedback policy remains owned by:
 *
 *     grammar/security/
 *     grammar/policies/
 *
 * where applicable.
 *
 * 11. PROVENANCE
 * -------------
 *
 * Feedback lineage integrates with the canonical provenance system.
 *
 * Relevant existing semantic consumers include:
 *
 *     learning provenance
 *     adaptation provenance
 *     execution provenance
 *     quantum resilience provenance.
 *
 * This grammar does not define provenance syntax.
 *
 * 12. QUANTUM
 * ----------
 *
 * Quantum-derived feedback remains ordinary source data at this boundary.
 *
 * If semantic lowering requires quantum computation:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * No AI-specific quantum IR is introduced.
 *
 * 13. RUNTIME
 * -----------
 *
 * Runtime feedback processing is downstream.
 *
 * The parser must never:
 *
 *     wait for feedback;
 *     read feedback;
 *     evaluate feedback;
 *     update a model;
 *     trigger adaptation;
 *     inspect hardware;
 *     inspect resources.
 *
 * ============================================================================
 * FILE-LEVEL DEPENDENCY DECLARATION
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     Expressions
 *     Names
 *     ZamaniLexer
 *
 * EXPORTS:
 *
 *     feedbackConstruct
 *
 * CONSUMED_BY:
 *
 *     AI
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     canonical semantic feedback model
 *
 * EFFECT_OWNER:
 *
 *     shared effect semantic subsystem
 *
 * CAPABILITY_OWNER:
 *
 *     shared capability subsystem
 *
 * RESOURCE_OWNER:
 *
 *     shared resource subsystem
 *
 * CONTRACT_OWNER:
 *
 *     grammar/validation/
 *
 * POLICY_OWNER:
 *
 *     grammar/security/
 *     grammar/policies/
 *
 * PROVENANCE_OWNER:
 *
 *     canonical provenance subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     quantum::ir for quantum computation
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/effects/
 *     grammar/tests/resources/
 *     grammar/tests/capabilities/
 *     grammar/tests/provenance/
 *     grammar/tests/policies/
 *     grammar/tests/quantum/
 *     grammar/tests/hybrid/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/spec/effects.md
 *     grammar/spec/resources.md
 *     grammar/spec/provenance.md
 *     grammar/spec/policies.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * FILE-LOCAL COMPLETION
 * ---------------------
 *
 * [x] Parser grammar.
 * [x] Grammar name is AIFeedback.
 * [x] No lexer rules.
 * [x] Uses canonical ZamaniLexer.
 * [x] Uses canonical FEEDBACK token.
 * [x] No duplicate keyword.
 * [x] No duplicate expression hierarchy.
 * [x] Target is mandatory.
 * [x] Source is optional.
 * [x] Context is optional.
 * [x] Canonical clause order is fixed.
 * [x] Empty context is rejected.
 * [x] Trailing context comma is rejected.
 * [x] Multiple context values are supported.
 * [x] No algorithm catalogue.
 * [x] No application catalogue.
 * [x] No hardware catalogue.
 * [x] No quantum-operation catalogue.
 * [x] No resource-size limit.
 * [x] No machine-size limit.
 * [x] No parser-time execution.
 * [x] No semantic predicates.
 * [x] No embedded Rust.
 * [x] No unsafe Rust dependency.
 * [x] AST remains domain-neutral.
 * [x] Semantic analysis remains downstream.
 * [x] Effects remain downstream.
 * [x] Capabilities remain downstream.
 * [x] Resources remain downstream.
 * [x] Contracts remain downstream.
 * [x] Policies remain downstream.
 * [x] Provenance remains downstream.
 * [x] IR remains downstream.
 * [x] Quantum computation uses quantum::ir.
 * [x] Target realization remains downstream.
 *
 * REPOSITORY COMPLETION
 * ---------------------
 *
 * [ ] Add this file to the ANTLR grammar source set.
 *
 * [ ] Import AIFeedback from grammar/ai/ai.g4.
 *
 * [ ] Add feedbackConstruct to AI.aiConstruct exactly once.
 *
 * [ ] Verify the root parser reaches AI through its existing composition
 *     boundary.
 *
 * [ ] Add parser conformance tests.
 *
 * [ ] Add semantic feedback tests.
 *
 * [ ] Add learning/feedback integration tests.
 *
 * [ ] Add adaptation/feedback integration tests.
 *
 * [ ] Add quantum/hybrid feedback tests.
 *
 * [ ] Add provenance/evidence tests.
 *
 * [ ] Add capability/resource tests.
 *
 * [ ] Add policy/security tests.
 *
 * [ ] Add scalability tests.
 *
 * [ ] Add determinism tests.
 *
 * [ ] Add round-trip formatter tests.
 *
 * [ ] Verify generated Rust remains safe Rust and compatible with Rust 1.97
 *     or later.
 *
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar AIFeedback;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    Names
    ;

/*
 * ============================================================================
 * PUBLIC FEEDBACK CONSTRUCT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     feedback TARGET;
 *
 *     feedback TARGET from SOURCE;
 *
 *     feedback TARGET with (CONTEXT);
 *
 *     feedback TARGET from SOURCE with (CONTEXT, CONTEXT);
 *
 * The semicolon belongs to the statement boundary.
 * ============================================================================
 */

feedbackConstruct
    : FEEDBACK
      feedbackTarget
      feedbackSourceClause?
      feedbackContextClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * FEEDBACK TARGET
 * ============================================================================
 *
 * The target is an ordinary Zamani expression.
 *
 * No feedback-specific target type is introduced.
 * ============================================================================
 */

feedbackTarget
    : expression
    ;

/*
 * ============================================================================
 * FEEDBACK SOURCE
 * ============================================================================
 *
 * The source identifies the value/event/result from which feedback originates.
 *
 * It remains an ordinary expression.
 * ============================================================================
 */

feedbackSourceClause
    : FROM
      expression
    ;

/*
 * ============================================================================
 * FEEDBACK CONTEXT
 * ============================================================================
 *
 * Context is optional but, when present, must contain at least one value.
 * ============================================================================
 */

feedbackContextClause
    : WITH
      LPAREN
      feedbackContextList
      RPAREN
    ;

/*
 * ============================================================================
 * FEEDBACK CONTEXT LIST
 * ============================================================================
 *
 * Open-ended repetition.
 *
 * There is deliberately no finite context count.
 *
 * A trailing comma is rejected.
 * ============================================================================
 */

feedbackContextList
    : feedbackContext
      (
          COMMA
          feedbackContext
      )*
    ;

/*
 * ============================================================================
 * FEEDBACK CONTEXT
 * ============================================================================
 *
 * Context values are ordinary expressions.
 *
 * No closed feedback vocabulary is introduced.
 * ============================================================================
 */

feedbackContext
    : expression
    ;