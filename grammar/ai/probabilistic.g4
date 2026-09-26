/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/probabilistic.g4
 *
 * Grammar:
 *     AIProbabilistic
 *
 * Status:
 *     Production probabilistic-computing leaf grammar.
 *
 * Compiler/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust required
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for probabilistic computation.
 *
 * Probabilistic computation is a first-class semantic capability of Zamani,
 * but this grammar deliberately does not turn probability theory, statistical
 * distributions, inference algorithms, or hardware implementations into a
 * closed keyword inventory.
 *
 * The grammar provides source-level structure for:
 *
 *     - probabilistic declarations;
 *     - random-variable declarations;
 *     - distribution expressions;
 *     - sampling;
 *     - observation;
 *     - conditioning;
 *     - likelihood expressions;
 *     - prior/posterior relationships;
 *     - probabilistic transformations;
 *     - inference requests;
 *     - probabilistic models;
 *     - probabilistic regions;
 *     - stochastic computation;
 *     - uncertainty declarations;
 *     - probabilistic resource/capability contracts;
 *     - probabilistic execution policies;
 *     - extensible probabilistic operations.
 *
 * It does NOT implement:
 *
 *     - probability arithmetic;
 *     - random-number generation;
 *     - distribution algorithms;
 *     - Bayesian inference;
 *     - MCMC;
 *     - variational inference;
 *     - particle filtering;
 *     - sequential Monte Carlo;
 *     - exact inference;
 *     - approximate inference;
 *     - symbolic probability manipulation;
 *     - numerical integration;
 *     - statistical estimation;
 *     - optimization;
 *     - automatic differentiation;
 *     - model execution;
 *     - tensor execution;
 *     - random-device selection;
 *     - entropy-source selection;
 *     - hardware selection;
 *     - accelerator selection;
 *     - quantum execution;
 *     - QEC;
 *     - ZQN;
 *     - runtime scheduling.
 *
 * Those responsibilities belong to semantic analysis, canonical IR,
 * optimization, execution, resource analysis, and target-specific lowering.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +---------------------------+
 *          |                           |
 *          v                           v
 *     Types / Expressions          AIProbabilistic
 *          |                           |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                  domain-neutral AST
 *                        |
 *                        v
 *                 semantic analysis
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      classical      quantum       distributed
 *       semantics     semantics      semantics
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                 canonical semantic IR
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *     classical IR   quantum::ir    other IR
 *                        |
 *                        v
 *                 optimization/lowering
 *                        |
 *              scheduling / routing
 *                        |
 *                resilience / QEC
 *                        |
 *                       ZQN
 *                        |
 *                       HAL
 *                        |
 *                target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - probabilistic parser-domain entry point;
 *     - probabilistic declaration syntax;
 *     - random-variable syntax;
 *     - distribution-expression syntax;
 *     - sampling syntax;
 *     - observation syntax;
 *     - conditioning syntax;
 *     - likelihood syntax;
 *     - prior/posterior relationship syntax;
 *     - probabilistic inference-request syntax;
 *     - probabilistic computation regions;
 *     - probabilistic execution-policy boundaries;
 *     - probabilistic resource/capability contracts;
 *     - extensible probabilistic operation structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - statements;
 *     - functions;
 *     - models;
 *     - datasets;
 *     - tensors;
 *     - differentiation;
 *     - training;
 *     - inference implementation;
 *     - optimizer algorithms;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - hardware discovery;
 *     - target selection;
 *     - scheduling;
 *     - routing;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical types:
 *
 *     grammar/types/types.g4
 *
 * Canonical expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Canonical statements:
 *
 *     grammar/statements/statements.g4
 *
 * This grammar intentionally does NOT import:
 *
 *     Models
 *     Datasets
 *     Training
 *     Inference
 *     Differentiation
 *
 * merely to reference their values.
 *
 * Probabilistic values remain ordinary Zamani expressions.
 *
 * This prevents dependency cycles such as:
 *
 *     probabilistic -> inference -> probabilistic
 *     probabilistic -> models -> probabilistic
 *     probabilistic -> datasets -> probabilistic
 *
 * ============================================================================
 * LEXICAL POLICY
 * ============================================================================
 *
 * Probabilistic concepts are semantic vocabulary.
 *
 * The grammar therefore uses the canonical annotation boundary:
 *
 *     AT identifier
 *
 * rather than requiring a new lexer keyword for every probability concept.
 *
 * Examples:
 *
 *     @probabilistic
 *     @random
 *     @distribution
 *     @sample
 *     @observe
 *     @condition
 *     @prior
 *     @posterior
 *     @likelihood
 *     @infer
 *
 * The parser preserves the annotation structure.
 *
 * Semantic analysis determines whether the annotation identifies a registered
 * probabilistic construct.
 *
 * This prevents the lexer from becoming a finite inventory of statistical
 * terminology.
 *
 * ============================================================================
 * IMPORTANT ANNOTATION INVARIANT
 * ============================================================================
 *
 * This grammar deliberately uses:
 *
 *     AT identifier
 *
 * instead of:
 *
 *     NANO_ANNOTATION
 *
 * for its public constructs.
 *
 * The canonical lexer provides both mechanisms, but the explicit:
 *
 *     AT + identifier
 *
 * form makes the annotation boundary structurally visible and consistent with
 * the current inference grammar.
 *
 * It also means the probabilistic grammar does not depend on the internal
 * tokenization shape of an opaque annotation token.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal limits on:
 *
 *     - random variables;
 *     - distributions;
 *     - distribution parameters;
 *     - dimensions;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - observations;
 *     - samples;
 *     - inference variables;
 *     - model variables;
 *     - conditions;
 *     - evidence terms;
 *     - latent variables;
 *     - chains;
 *     - particles;
 *     - states;
 *     - probabilistic operations;
 *     - nested probabilistic regions;
 *     - model size;
 *     - dataset size;
 *     - workers;
 *     - devices;
 *     - accelerators;
 *     - nodes;
 *     - memory;
 *     - quantum resources.
 *
 * There are deliberately NO grammar-level constants such as:
 *
 *     MAX_RANDOM_VARIABLES
 *     MAX_DISTRIBUTIONS
 *     MAX_SAMPLES
 *     MAX_PARTICLES
 *     MAX_STATES
 *     MAX_PARAMETERS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_MODELS
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_QUBITS
 *
 * Repetition is represented structurally.
 *
 * Actual resource exhaustion is an implementation/resource-management issue,
 * not a language-level semantic ceiling.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     GPU 0
 *     CPU 0
 *     QPU 0
 *     device 0
 *     fixed accelerator IDs
 *     fixed memory capacity
 *     fixed tensor width
 *     fixed tensor rank
 *     fixed SIMD width
 *     fixed thread count
 *     fixed node count
 *     fixed cluster size
 *     fixed quantum-resource count
 *     fixed random-source implementation
 *     vendor-specific statistical hardware
 *
 * Program values such as:
 *
 *     1024
 *     1000000
 *     sample_count
 *
 * remain ordinary program semantics.
 *
 * A source-level:
 *
 *     sample_count = 1024
 *
 * is fundamentally different from a compiler rule such as:
 *
 *     MAX_SAMPLES = 1024
 *
 * The latter is forbidden.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Probabilistic source syntax must preserve the distinction between:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *
 * Examples of semantic intent:
 *
 *     requires capability("probabilistic.inference");
 *
 *     requires capability("randomness.entropy");
 *
 *     requires memory >= required_memory;
 *
 *     constrain variance <= tolerance;
 *
 *     prefer deterministic_replay;
 *
 *     hint parallel_sampling;
 *
 * None of these selects a physical device.
 *
 * Resource realization belongs downstream.
 *
 * ============================================================================
 * DISTRIBUTION MODEL
 * ============================================================================
 *
 * A distribution is represented as an extensible semantic expression:
 *
 *     @distribution Normal(mu, sigma)
 *
 *     @distribution custom::distribution(parameters)
 *
 *     @distribution expression
 *
 * The grammar does NOT enumerate:
 *
 *     Normal
 *     Bernoulli
 *     Binomial
 *     Poisson
 *     Gaussian
 *     Uniform
 *     Categorical
 *     Multinomial
 *     Exponential
 *     Gamma
 *     Beta
 *     Dirichlet
 *     etc.
 *
 * Such names are semantic identifiers.
 *
 * This keeps the language open to:
 *
 *     user-defined distributions;
 *     symbolic distributions;
 *     domain distributions;
 *     future probability models;
 *     library distributions;
 *     quantum-induced distributions;
 *     hardware-derived distributions.
 *
 * ============================================================================
 * RANDOM VARIABLE MODEL
 * ============================================================================
 *
 * A random variable declaration establishes source-level probabilistic intent.
 *
 * Examples:
 *
 *     @random x ~ distribution;
 *
 *     @random x: Real ~ distribution;
 *
 *     @random x = distribution;
 *
 * The grammar does not decide:
 *
 *     - whether x is discrete;
 *     - whether x is continuous;
 *     - whether x is finite;
 *     - whether x is tensor-valued;
 *     - whether x is quantum-derived;
 *     - how x is sampled;
 *     - where x is stored.
 *
 * Those are semantic questions.
 *
 * ============================================================================
 * OBSERVATION MODEL
 * ============================================================================
 *
 * Observations express evidence.
 *
 * Examples:
 *
 *     @observe x = value;
 *
 *     @observe(x, value);
 *
 *     @observe likelihood;
 *
 * Observation syntax does not mutate a runtime probability model.
 *
 * Runtime semantics belong downstream.
 *
 * ============================================================================
 * CONDITIONING MODEL
 * ============================================================================
 *
 * Conditioning expresses a semantic relationship between a probabilistic
 * computation and evidence/conditions.
 *
 * Examples:
 *
 *     @condition x given evidence;
 *
 *     @condition(model, evidence);
 *
 *     @condition expression;
 *
 * The grammar does not implement Bayes' theorem.
 *
 * ============================================================================
 * PRIOR / POSTERIOR MODEL
 * ============================================================================
 *
 * Prior and posterior are semantic relationships.
 *
 * They are not tied to a particular inference algorithm.
 *
 * Examples:
 *
 *     @prior theta = distribution;
 *
 *     @posterior theta;
 *
 *     @posterior(theta | evidence);
 *
 * The semantic layer determines the actual probability semantics.
 *
 * ============================================================================
 * LIKELIHOOD MODEL
 * ============================================================================
 *
 * Likelihood syntax describes a probabilistic relationship.
 *
 * Examples:
 *
 *     @likelihood observation given parameters;
 *
 *     @likelihood(model, data);
 *
 * The grammar does not decide whether likelihood evaluation is:
 *
 *     exact;
 *     approximate;
 *     symbolic;
 *     numerical;
 *     sampled;
 *     differentiated;
 *     accelerated.
 *
 * ============================================================================
 * SAMPLING MODEL
 * ============================================================================
 *
 * Sampling is a semantic request.
 *
 * Examples:
 *
 *     @sample x;
 *
 *     @sample distribution;
 *
 *     @sample(distribution, count);
 *
 *     @sample x into samples;
 *
 * The grammar does not impose a maximum sample count.
 *
 * Sampling strategy remains downstream.
 *
 * ============================================================================
 * INFERENCE MODEL
 * ============================================================================
 *
 * The grammar permits an inference REQUEST without enumerating algorithms.
 *
 * Examples:
 *
 *     @probabilistic model { ... }
 *
 *     @infer model;
 *
 *     @infer(model, evidence);
 *
 *     @infer model with configuration;
 *
 * The semantic layer determines whether the requested inference can be
 * realized and which implementation strategy satisfies the program's
 * semantic contract.
 *
 * Algorithm names, if supplied, are ordinary semantic identifiers.
 *
 * ============================================================================
 * STOCHASTIC / DETERMINISTIC SEPARATION
 * ============================================================================
 *
 * A probabilistic program may request deterministic replay or stochastic
 * execution as a semantic policy.
 *
 * The grammar does not itself generate randomness.
 *
 * Therefore:
 *
 *     parsing != sampling
 *
 *     parsing != inference
 *
 *     parsing != random-number generation
 *
 *     parsing != entropy acquisition
 *
 * Runtime nondeterminism MUST NOT influence parsing.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Probabilistic computation may consume values produced by quantum computation.
 *
 * Examples include:
 *
 *     measurement-derived values;
 *     quantum simulation results;
 *     probabilistic hybrid algorithms;
 *     stochastic quantum-classical workflows.
 *
 * This grammar does NOT define:
 *
 *     qubits;
 *     quantum gates;
 *     measurement semantics;
 *     QEC;
 *     physical qubits;
 *     topology;
 *     routing;
 *     scheduling;
 *     ZQN.
 *
 * The downstream path remains:
 *
 *     probabilistic source
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> classical semantics
 *          |
 *          +--> quantum semantics
 *                     |
 *                     v
 *                 quantum::ir
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * This grammar integrates with:
 *
 *     models.g4
 *     datasets.g4
 *     tensors.g4
 *     training.g4
 *     inference.g4
 *     differentiation.g4
 *     pipelines.g4
 *     agents.g4
 *
 * through ordinary expressions, types, and statements.
 *
 * It does not import those grammars merely to duplicate their syntax.
 *
 * For example:
 *
 *     @random weights: Tensor = distribution;
 *
 * can reference the canonical tensor type without defining a second tensor
 * grammar.
 *
 * ============================================================================
 * DIFFERENTIATION INTEGRATION
 * ============================================================================
 *
 * Probabilistic expressions may be differentiated.
 *
 * This grammar does not define:
 *
 *     gradients;
 *     Jacobians;
 *     Hessians;
 *     automatic differentiation;
 *     parameter-shift;
 *     adjoint differentiation.
 *
 * Those remain owned by:
 *
 *     grammar/ai/differentiation.g4
 *
 * and its semantic/lowering layers.
 *
 * A probabilistic expression may therefore become the subject of a
 * differentiation request through ordinary expression composition.
 *
 * ============================================================================
 * TRAINING INTEGRATION
 * ============================================================================
 *
 * Probabilistic models may be used during training.
 *
 * Training syntax remains owned by:
 *
 *     grammar/ai/training.g4
 *
 * This file only describes probabilistic computation and relationships.
 *
 * ============================================================================
 * INFERENCE INTEGRATION
 * ============================================================================
 *
 * Probabilistic inference may participate in AI inference.
 *
 * `inference.g4` owns the general inference-domain composition boundary.
 *
 * This file owns probabilistic semantics inside that boundary.
 *
 * The two grammars MUST NOT become mutually recursive parser grammars.
 *
 * Integration occurs through:
 *
 *     expressions
 *     types
 *     statements
 *     semantic contracts
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Probabilistic programs may eventually lower to:
 *
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     distributed system;
 *     heterogeneous system;
 *     future target.
 *
 * The grammar remains target-independent.
 *
 * Hardware-specific realization belongs to:
 *
 *     hardware/
 *     resources/
 *     compile/
 *     execution/
 *     hdl/
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar must map to domain-neutral AST structures.
 *
 * Recommended semantic AST shape:
 *
 *     ProbabilisticConstruct
 *       {
 *           annotation,
 *           kind,
 *           name?,
 *           type?,
 *           subject?,
 *           distribution?,
 *           operands[],
 *           clauses[],
 *           body?,
 *           source_span
 *       }
 *
 * Recommended semantic kinds:
 *
 *     Declaration
 *     RandomVariable
 *     Distribution
 *     Sample
 *     Observation
 *     Condition
 *     Prior
 *     Posterior
 *     Likelihood
 *     InferenceRequest
 *     ProbabilisticRegion
 *     Requirement
 *     Capability
 *     Constraint
 *     Preference
 *     Hint
 *
 * The exact Rust AST enum/struct names are owned by the frontend AST
 * implementation, not by this grammar.
 *
 * The AST must remain independent of:
 *
 *     LLVM;
 *     MLIR;
 *     QIR;
 *     vendor ML runtimes;
 *     vendor quantum runtimes;
 *     physical topology;
 *     QEC implementation;
 *     hardware IDs.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - validating probabilistic annotations;
 *     - resolving random variables;
 *     - resolving distributions;
 *     - checking distribution parameter compatibility;
 *     - checking variable types;
 *     - validating observations;
 *     - validating conditioning;
 *     - validating prior/posterior relationships;
 *     - validating likelihood relationships;
 *     - validating inference requests;
 *     - validating probabilistic scopes;
 *     - validating stochastic/deterministic policy;
 *     - checking resource requirements;
 *     - checking capabilities;
 *     - checking constraints;
 *     - distinguishing preferences from requirements;
 *     - checking model/data compatibility;
 *     - checking quantum/classical composition;
 *     - checking differentiability where requested;
 *     - checking numerical validity;
 *     - checking portability.
 *
 * The parser performs none of these semantic checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Probabilistic semantic constructs must lower through the repository's
 * canonical semantic representation.
 *
 * Depending on the computation, downstream lowering may involve:
 *
 *     classical IR;
 *     tensor/data representation;
 *     quantum::ir;
 *     distributed representation;
 *     hardware/accelerator representation.
 *
 * No probabilistic grammar-specific IR is introduced here.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source;
 *     lexer;
 *     grammar;
 *     parser configuration.
 *
 * It MUST NOT depend on:
 *
 *     random state;
 *     entropy source;
 *     current time;
 *     hardware;
 *     available accelerators;
 *     network state;
 *     filesystem state;
 *     runtime state;
 *     scheduler state.
 *
 * Identical canonical token streams under identical parser configuration must
 * produce equivalent parse-tree structure.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser errors include:
 *
 *     - malformed probabilistic declaration;
 *     - malformed distribution expression;
 *     - malformed sample expression;
 *     - malformed observation;
 *     - malformed conditioning;
 *     - malformed prior/posterior form;
 *     - malformed likelihood form;
 *     - malformed inference request;
 *     - missing delimiters;
 *     - malformed clauses;
 *     - malformed probabilistic region.
 *
 * Semantic errors remain downstream:
 *
 *     - unknown distribution;
 *     - invalid distribution parameters;
 *     - invalid variable type;
 *     - undefined random variable;
 *     - invalid observation;
 *     - invalid conditioning;
 *     - invalid prior/posterior relationship;
 *     - impossible inference request;
 *     - unavailable capability;
 *     - unsatisfied resource requirement;
 *     - invalid stochastic policy;
 *     - numerical instability;
 *     - unsupported target realization.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no hardware discovery;
 *     - no credential access;
 *     - no random-number generation;
 *     - no runtime execution.
 *
 * Probabilistic source code is data to the parser, not executable parser
 * behavior.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * Generated and consuming implementation code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only.
 *
 * No `unsafe` Rust is required by this grammar.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

parser grammar AIProbabilistic;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/* ============================================================================
 * 1. PUBLIC PROBABILISTIC CONSTRUCT
 * ========================================================================== */

/**
 * The single public entry point for this leaf grammar.
 *
 * All alternatives cross an explicit probabilistic annotation boundary.
 *
 * An ordinary expression is therefore NOT automatically classified as
 * probabilistic syntax.
 */
aiProbabilisticConstruct
    : probabilisticDeclaration
    | randomVariableDeclaration
    | distributionDeclaration
    | sampleConstruct
    | observationConstruct
    | conditioningConstruct
    | priorConstruct
    | posteriorConstruct
    | likelihoodConstruct
    | probabilisticInferenceConstruct
    | probabilisticRegion
    | probabilisticContract
    ;


/* ============================================================================
 * 2. COMMON ANNOTATION
 * ========================================================================== */

/**
 * Probabilistic annotation boundary.
 *
 * Semantic analysis validates the annotation name.
 *
 * The parser intentionally does not enumerate annotation spellings.
 */
probabilisticAnnotation
    : AT
      identifier
    ;


/* ============================================================================
 * 3. GENERIC PROBABILISTIC DECLARATION
 * ========================================================================== */

/**
 * General probabilistic declaration.
 *
 * Examples:
 *
 *     @probabilistic model { ... }
 *
 *     @probabilistic x: Real = expression;
 *
 *     @probabilistic model = expression;
 *
 * The semantic layer determines the declaration's precise role.
 */
probabilisticDeclaration
    : probabilisticAnnotation
      identifier
      probabilisticDeclarationType?
      probabilisticInitializer?
      probabilisticDeclarationBody?
    ;


probabilisticDeclarationType
    : COLON
      typeExpression
    ;


probabilisticInitializer
    : ASSIGN
      expression
    ;


probabilisticDeclarationBody
    : LBRACE
      probabilisticMember*
      RBRACE
    ;


/* ============================================================================
 * 4. RANDOM VARIABLE
 * ========================================================================== */

/**
 * Random-variable declaration.
 *
 * Canonical semantic forms include:
 *
 *     @random x ~ distribution;
 *     @random x: Type ~ distribution;
 *     @random x = distribution;
 *
 * `TILDE` identifies the probabilistic relationship.
 */
randomVariableDeclaration
    : probabilisticAnnotation
      identifier
      randomVariableType?
      randomVariableDefinition
      SEMICOLON?
    ;


randomVariableType
    : COLON
      typeExpression
    ;


randomVariableDefinition
    : TILDE
      probabilisticExpression
    | ASSIGN
      probabilisticExpression
    ;


/* ============================================================================
 * 5. DISTRIBUTION DECLARATION
 * ========================================================================== */

/**
 * Named distribution definition.
 *
 * Distribution names remain ordinary identifiers.
 *
 * Examples:
 *
 *     @distribution Normal(mu, sigma);
 *
 *     @distribution my_distribution = expression;
 *
 *     @distribution custom::family(parameters);
 */
distributionDeclaration
    : probabilisticAnnotation
      identifier
      distributionParameters?
      distributionInitializer?
      SEMICOLON?
    ;


distributionParameters
    : LPAREN
      argumentList?
      RPAREN
    ;


distributionInitializer
    : ASSIGN
      probabilisticExpression
    ;


/* ============================================================================
 * 6. DISTRIBUTION EXPRESSION
 * ========================================================================== */

/**
 * Extensible distribution expression.
 *
 * The grammar does not enumerate probability distributions.
 */
probabilisticDistributionExpression
    : identifier
      probabilisticDistributionArguments?
    | qualifiedProbabilisticName
      probabilisticDistributionArguments?
    ;


probabilisticDistributionArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


qualifiedProbabilisticName
    : identifier
      DOUBLE_COLON
      identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 7. PROBABILISTIC EXPRESSION
 * ========================================================================== */

/**
 * A probabilistic expression is either:
 *
 *     - an explicitly structured probabilistic distribution expression; or
 *     - an ordinary Zamani expression.
 *
 * Ordinary expressions remain ordinary expressions in the AST unless semantic
 * analysis determines that they participate in a probabilistic construct.
 */
probabilisticExpression
    : probabilisticDistributionExpression
    | expression
    ;


/* ============================================================================
 * 8. SAMPLING
 * ========================================================================== */

/**
 * Sampling request.
 *
 * Examples:
 *
 *     @sample x;
 *     @sample distribution;
 *     @sample(distribution, count);
 *
 * The sample count is source data, not a grammar-level capacity.
 */
sampleConstruct
    : probabilisticAnnotation
      sampleSubject
      sampleArguments?
      SEMICOLON?
    ;


sampleSubject
    : identifier
    | probabilisticDistributionExpression
    | expression
    ;


sampleArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 9. OBSERVATION
 * ========================================================================== */

/**
 * Observation/evidence declaration.
 *
 * Examples:
 *
 *     @observe x = value;
 *     @observe(x, value);
 *     @observe expression;
 */
observationConstruct
    : probabilisticAnnotation
      observationPayload
      SEMICOLON?
    ;


observationPayload
    : observationAssignment
    | observationCall
    | expression
    ;


observationAssignment
    : expression
      ASSIGN
      expression
    ;


observationCall
    : LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 10. CONDITIONING
 * ========================================================================== */

/**
 * Conditioning relationship.
 *
 * Examples:
 *
 *     @condition model given evidence;
 *     @condition(model, evidence);
 *     @condition expression;
 *
 * `given` remains semantic vocabulary rather than a required lexer keyword.
 */
conditioningConstruct
    : probabilisticAnnotation
      conditioningPayload
      SEMICOLON?
    ;


conditioningPayload
    : conditioningExpression
    | conditioningCall
    ;


conditioningExpression
    : expression
      probabilisticGivenClause?
    ;


probabilisticGivenClause
    : identifier
      expression
    ;


conditioningCall
    : LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 11. PRIOR
 * ========================================================================== */

/**
 * Prior relationship.
 *
 * Examples:
 *
 *     @prior theta = distribution;
 *     @prior(theta, distribution);
 */
priorConstruct
    : probabilisticAnnotation
      priorPayload
      SEMICOLON?
    ;


priorPayload
    : priorAssignment
    | priorCall
    ;


priorAssignment
    : expression
      ASSIGN
      probabilisticExpression
    ;


priorCall
    : LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 12. POSTERIOR
 * ========================================================================== */

/**
 * Posterior request.
 *
 * Examples:
 *
 *     @posterior theta;
 *     @posterior(theta, evidence);
 */
posteriorConstruct
    : probabilisticAnnotation
      posteriorPayload
      SEMICOLON?
    ;


posteriorPayload
    : expression
    | LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 13. LIKELIHOOD
 * ========================================================================== */

/**
 * Likelihood relationship.
 *
 * Examples:
 *
 *     @likelihood model;
 *     @likelihood(model, data);
 */
likelihoodConstruct
    : probabilisticAnnotation
      likelihoodPayload
      SEMICOLON?
    ;


likelihoodPayload
    : expression
    | LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 14. PROBABILISTIC INFERENCE
 * ========================================================================== */

/**
 * General inference request.
 *
 * Examples:
 *
 *     @infer model;
 *     @infer(model, evidence);
 *     @infer model with configuration;
 *
 * Algorithm names are ordinary identifiers and are not enumerated here.
 */
probabilisticInferenceConstruct
    : probabilisticAnnotation
      probabilisticInferencePayload
      SEMICOLON?
    ;


probabilisticInferencePayload
    : expression
    | LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 15. PROBABILISTIC REGION
 * ========================================================================== */

/**
 * Structured probabilistic computation region.
 *
 * Examples:
 *
 *     @probabilistic {
 *         ...
 *     }
 *
 *     @stochastic {
 *         ...
 *     }
 *
 *     @model {
 *         ...
 *     }
 *
 * The annotation name is semantically resolved.
 */
probabilisticRegion
    : probabilisticAnnotation
      LBRACE
      probabilisticMember*
      RBRACE
    ;


/* ============================================================================
 * 16. PROBABILISTIC MEMBERS
 * ========================================================================== */

/**
 * A probabilistic region can contain:
 *
 *     - random variables;
 *     - distribution declarations;
 *     - sampling;
 *     - observations;
 *     - conditioning;
 *     - priors;
 *     - posteriors;
 *     - likelihoods;
 *     - inference;
 *     - contracts;
 *     - ordinary Zamani statements.
 *
 * Ordinary statements provide the classical/hybrid integration boundary.
 */
probabilisticMember
    : randomVariableDeclaration
    | distributionDeclaration
    | sampleConstruct
    | observationConstruct
    | conditioningConstruct
    | priorConstruct
    | posteriorConstruct
    | likelihoodConstruct
    | probabilisticInferenceConstruct
    | probabilisticContract
    | statement
    ;


/* ============================================================================
 * 17. PROBABILISTIC CONTRACTS
 * ========================================================================== */

/**
 * Resource/capability/constraint/preference/hint contract.
 *
 * Contract names remain extensible.
 */
probabilisticContract
    : probabilisticContractAnnotation
      expression
      SEMICOLON?
    ;


probabilisticContractAnnotation
    : AT
      identifier
    ;


/* ============================================================================
 * 18. EXPLICIT CONTRACT BRIDGES
 * ========================================================================== */

/**
 * These rules provide stable semantic/tooling anchors.
 *
 * They are intentionally not separate lexer vocabularies.
 *
 * Semantic analysis determines whether an annotation represents a requirement,
 * capability, constraint, preference, or hint.
 */

probabilisticRequirement
    : probabilisticAnnotation
      expression
      SEMICOLON?
    ;


probabilisticCapability
    : probabilisticAnnotation
      expression
      SEMICOLON?
    ;


probabilisticConstraint
    : probabilisticAnnotation
      expression
      SEMICOLON?
    ;


probabilisticPreference
    : probabilisticAnnotation
      expression
      SEMICOLON?
    ;


probabilisticHint
    : probabilisticAnnotation
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 19. PROBABILISTIC OPERATIONS
 * ========================================================================== */

/**
 * Extensible probabilistic operation.
 *
 * This is the generic escape hatch for future probability operations without
 * requiring a new grammar keyword for every mathematical or statistical
 * function.
 *
 * Examples:
 *
 *     @probability operation(...);
 *     @expectation(...);
 *     @variance(...);
 *     @entropy(...);
 *     @marginalize(...);
 *     @normalize(...);
 *
 * These names remain semantic identifiers.
 */
probabilisticOperation
    : probabilisticAnnotation
      identifier
      probabilisticOperationPayload?
      SEMICOLON?
    ;


probabilisticOperationPayload
    : argumentListExpression
    | expression
    | probabilisticOperationRegion
    ;


argumentListExpression
    : LPAREN
      argumentList?
      RPAREN
    ;


probabilisticOperationRegion
    : LBRACE
      probabilisticMember*
      RBRACE
    ;


/* ============================================================================
 * 20. CONDITIONAL / PROBABILITY RELATIONSHIPS
 * ========================================================================== */

/**
 * Structured semantic relationship.
 *
 * This preserves an extensible representation for relationships such as:
 *
 *     probability(X | Y)
 *     P(X | Y)
 *     expectation(X)
 *     variance(X)
 *
 * without adding a separate mathematical expression language.
 *
 * The actual probability notation is resolved by the canonical expression
 * grammar and semantic layer.
 */
probabilisticRelationship
    : expression
    ;


/* ============================================================================
 * 21. UNCERTAINTY DECLARATION
 * ========================================================================== */

/**
 * General uncertainty annotation.
 *
 * Examples:
 *
 *     @uncertain x;
 *     @uncertain x = expression;
 *     @uncertain x: Type;
 */
uncertaintyConstruct
    : probabilisticAnnotation
      identifier
      probabilisticDeclarationType?
      probabilisticInitializer?
      SEMICOLON?
    ;


/* ============================================================================
 * 22. PROBABILISTIC VALUE BRIDGE
 * ========================================================================== */

/**
 * Explicit expression bridge for parser composition layers.
 *
 * This rule is deliberately NOT part of aiProbabilisticConstruct.
 *
 * It prevents every ordinary expression from becoming probabilistic syntax.
 */
probabilisticValue
    : expression
    ;


/**
 * Explicit type bridge.
 */
probabilisticType
    : typeExpression
    ;


/**
 * Explicit statement bridge.
 */
probabilisticStatement
    : statement
    ;


/* ============================================================================
 * 23. COMPOSITION / INTEGRATION CONTRACT
 * ============================================================================
 *
 * AI aggregate:
 *
 *     grammar/ai/ai.g4
 *
 * remains the AI composition boundary.
 *
 * This leaf grammar should be imported by the canonical parser composition
 * layer as:
 *
 *     AIProbabilistic
 *
 * where the repository's composition architecture permits direct leaf
 * delegation.
 *
 * IMPORTANT:
 *
 * Do NOT add:
 *
 *     aiProbabilisticConstruct
 *
 * as another unconstrained sibling alternative beside a generic
 * annotation-driven AI rule if that rule can consume the same token prefix.
 *
 * Annotation-led constructs intentionally share:
 *
 *     AT identifier
 *
 * prefixes.
 *
 * Therefore integration must use ONE ownership path.
 *
 * Recommended composition:
 *
 *     canonical parser
 *          |
 *          +--> AI
 *          |
 *          +--> AIProbabilistic
 *
 * or:
 *
 *     AI
 *       |
 *       +--> AIProbabilistic
 *
 * but not both simultaneously.
 *
 * The exact choice must follow the existing canonical parser composition
 * boundary so that there is exactly one parser path for each source construct.
 *
 * ============================================================================
 * 24. AI.G4 INTEGRATION
 * ============================================================================
 *
 * The existing `grammar/ai/ai.g4` is the AI-domain composition boundary.
 *
 * Its role must remain composition rather than duplication.
 *
 * Integration contract:
 *
 *     AI
 *       -> AIProbabilistic
 *
 * only when the aggregate grammar is responsible for importing leaf grammars.
 *
 * If the canonical parser imports leaf grammars directly, AI.g4 does not need
 * to import this file.
 *
 * Do NOT add a second generic:
 *
 *     NANO_ANNOTATION ...
 *
 * alternative solely to expose probabilistic syntax.
 *
 * That would create an annotation-prefix ambiguity.
 *
 * ============================================================================
 * 25. INFERENCE.G4 INTEGRATION
 * ============================================================================
 *
 * The existing:
 *
 *     grammar/ai/inference.g4
 *
 * already owns general inference syntax.
 *
 * Therefore this grammar MUST NOT redefine the inference language.
 *
 * Probabilistic inference is represented here as probabilistic intent.
 *
 * General inference remains owned by Inference.
 *
 * Semantic analysis joins:
 *
 *     inference intent
 *          +
 *     probabilistic semantics
 *
 * without creating a circular grammar dependency.
 *
 * ============================================================================
 * 26. MODELS.G4 INTEGRATION
 * ============================================================================
 *
 * Model names are expressions/identifiers here.
 *
 * This file does not import or duplicate:
 *
 *     AIModels
 *
 * Model declarations remain owned by:
 *
 *     grammar/ai/models.g4
 *
 * A probabilistic model can therefore reference a model value through the
 * ordinary expression/type system.
 *
 * ============================================================================
 * 27. DATASETS.G4 INTEGRATION
 * ============================================================================
 *
 * Dataset values remain ordinary expressions.
 *
 * This file does not duplicate dataset syntax.
 *
 * Probabilistic observations can consume dataset-derived expressions.
 *
 * ============================================================================
 * 28. TRAINING.G4 INTEGRATION
 * ============================================================================
 *
 * Training remains owned by:
 *
 *     grammar/ai/training.g4
 *
 * A training construct may semantically contain probabilistic objectives,
 * priors, likelihoods, stochastic sampling, or posterior computations.
 *
 * The parser grammars must not become mutually recursive.
 *
 * Integration occurs through:
 *
 *     expressions
 *     types
 *     statements
 *     semantic analysis
 *
 * ============================================================================
 * 29. DIFFERENTIATION.G4 INTEGRATION
 * ============================================================================
 *
 * Differentiation remains owned by:
 *
 *     grammar/ai/differentiation.g4
 *
 * Probabilistic expressions may become differentiation subjects.
 *
 * This file does not define:
 *
 *     gradient
 *     Jacobian
 *     Hessian
 *     autodiff
 *     parameter-shift
 *     adjoint differentiation
 *
 * as parser-level mathematical implementations.
 *
 * ============================================================================
 * 30. TENSOR INTEGRATION
 * ============================================================================
 *
 * Tensor syntax remains owned by the canonical type/expression/data/AI tensor
 * grammars.
 *
 * Probabilistic tensor values are represented using ordinary:
 *
 *     typeExpression
 *     expression
 *
 * No tensor rank or dimension ceiling is introduced here.
 *
 * ============================================================================
 * 31. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum values may occur inside:
 *
 *     expressions;
 *     observations;
 *     likelihoods;
 *     probabilistic models;
 *     inference requests;
 *     probabilistic regions.
 *
 * Quantum semantics remain owned by the quantum subsystem.
 *
 * The eventual canonical quantum boundary is:
 *
 *     quantum::ir
 *
 * ============================================================================
 * 32. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Probabilistic computation may be implemented by hardware accelerators or
 * embedded systems.
 *
 * This grammar does not describe:
 *
 *     registers;
 *     physical buses;
 *     memory banks;
 *     device IDs;
 *     FPGA resources;
 *     accelerator topology;
 *     clock frequencies;
 *     fixed widths.
 *
 * Such intent belongs downstream to:
 *
 *     hardware/
 *     hdl/
 *     resources/
 *     compile/
 *     execution/
 *
 * ============================================================================
 * 33. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Probabilistic computation may be distributed.
 *
 * This grammar does not encode:
 *
 *     node count;
 *     worker count;
 *     cluster topology;
 *     network addresses;
 *     physical placement.
 *
 * Distributed realization belongs downstream.
 *
 * ============================================================================
 * 34. AST COMPLETION CONTRACT
 * ============================================================================
 *
 * Before this grammar is marked complete, the frontend AST must have a
 * predetermined structural mapping for every public rule:
 *
 *     aiProbabilisticConstruct
 *         -> ProbabilisticConstruct
 *
 *     probabilisticDeclaration
 *         -> ProbabilisticDeclaration
 *
 *     randomVariableDeclaration
 *         -> RandomVariableDeclaration
 *
 *     distributionDeclaration
 *         -> DistributionDeclaration
 *
 *     sampleConstruct
 *         -> SampleConstruct
 *
 *     observationConstruct
 *         -> ObservationConstruct
 *
 *     conditioningConstruct
 *         -> ConditioningConstruct
 *
 *     priorConstruct
 *         -> PriorConstruct
 *
 *     posteriorConstruct
 *         -> PosteriorConstruct
 *
 *     likelihoodConstruct
 *         -> LikelihoodConstruct
 *
 *     probabilisticInferenceConstruct
 *         -> ProbabilisticInferenceRequest
 *
 *     probabilisticRegion
 *         -> ProbabilisticRegion
 *
 *     probabilisticContract
 *         -> ProbabilisticContract
 *
 *     probabilisticOperation
 *         -> ProbabilisticOperation
 *
 * These are conceptual mappings; exact Rust type names remain owned by the
 * frontend AST implementation.
 *
 * ============================================================================
 * 35. IR COMPLETION CONTRACT
 * ============================================================================
 *
 * Every AST node above must have an established semantic/IR mapping before
 * this feature is considered production complete.
 *
 * Example:
 *
 *     random variable
 *          |
 *          v
 *     semantic random variable
 *          |
 *          v
 *     canonical probabilistic semantic representation
 *          |
 *          +--> classical lowering
 *          +--> tensor/data lowering
 *          +--> quantum::ir where applicable
 *          +--> distributed lowering where applicable
 *
 * There must NOT be:
 *
 *     ProbabilisticIR
 *     ProbabilisticQuantumIR
 *     ProbabilisticDeviceIR
 *
 * created merely because this grammar exists.
 *
 * ============================================================================
 * 36. RESOURCE SEMANTICS
 * ============================================================================
 *
 * Semantic analysis may derive requirements such as:
 *
 *     requires capability("probabilistic.inference")
 *
 *     requires capability("randomness.entropy")
 *
 *     requires capability("distributed.sampling")
 *
 *     requires memory >= required_memory
 *
 *     requires throughput >= required_throughput
 *
 *     requires reliability >= required_reliability
 *
 * These are semantic requirements, not physical allocations.
 *
 * ============================================================================
 * 37. PORTABILITY CONTRACT
 * ============================================================================
 *
 * The same source-level probabilistic program must remain representable
 * independently of target:
 *
 *     embedded;
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU-assisted system;
 *     simulator;
 *     HPC;
 *     cluster;
 *     cloud;
 *     heterogeneous machine;
 *     future architecture.
 *
 * Target realization is determined after semantic analysis.
 *
 * ============================================================================
 * 38. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 *
 *     @random x ~ Normal(mu, sigma);
 *
 *     @random x: Real ~ custom::distribution(parameter);
 *
 *     @distribution Normal(mu, sigma);
 *
 *     @sample x;
 *
 *     @sample Normal(mu, sigma);
 *
 *     @sample(distribution, count);
 *
 *     @observe x = value;
 *
 *     @observe(x, value);
 *
 *     @condition(model, evidence);
 *
 *     @prior theta = distribution;
 *
 *     @posterior theta;
 *
 *     @likelihood(model, data);
 *
 *     @infer(model, evidence);
 *
 *     @probabilistic {
 *         @random x ~ distribution;
 *         @observe x = value;
 *     }
 *
 *     @probabilistic model {
 *         ...
 *     }
 *
 *     @uncertain x = expression;
 *
 *     @probability expectation(expression);
 *
 *
 * NEGATIVE PARSER TESTS
 *
 *     @random;
 *
 *     @random x ~ ;
 *
 *     @observe = ;
 *
 *     @prior = ;
 *
 *     @posterior(;
 *
 *     @likelihood(;
 *
 *     @infer(;
 *
 *     @probabilistic {
 *         malformed
 *     ...
 *
 *
 * SEMANTIC NEGATIVE TESTS
 *
 *     unknown distribution;
 *     undefined random variable;
 *     incompatible distribution parameters;
 *     invalid observation target;
 *     invalid conditioning target;
 *     invalid prior;
 *     invalid posterior;
 *     invalid likelihood;
 *     unsatisfiable resource requirement;
 *     unavailable capability;
 *     invalid stochastic policy.
 *
 *
 * BOUNDARY TESTS
 *
 *     one random variable;
 *     many random variables;
 *     nested probabilistic regions;
 *     deeply nested expressions;
 *     large distribution parameter lists;
 *     large observation sets;
 *     large evidence sets;
 *     many inference variables;
 *     large symbolic expressions;
 *     empty probabilistic regions.
 *
 *
 * CROSS-DOMAIN TESTS
 *
 *     classical -> probabilistic;
 *     probabilistic -> classical;
 *     quantum measurement -> probabilistic;
 *     probabilistic -> quantum-derived computation;
 *     AI model -> probabilistic inference;
 *     probabilistic -> differentiation;
 *     probabilistic -> tensor computation;
 *     probabilistic -> distributed execution;
 *     probabilistic -> accelerator execution;
 *     probabilistic -> HDL/hardware realization.
 *
 *
 * SCALABILITY TESTS
 *
 * Verify that source syntax remains independent of:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     node count;
 *     worker count;
 *     memory capacity;
 *     tensor rank;
 *     tensor dimensions;
 *     number of samples;
 *     number of random variables.
 *
 * Tests may exercise progressively larger programs until actual parser,
 * compiler, or environment resources are exhausted.
 *
 * They MUST NOT encode those exhaustion points as language semantics.
 *
 *
 * DETERMINISM TESTS
 *
 * Identical:
 *
 *     source
 *     lexer configuration
 *     grammar version
 *     parser configuration
 *
 * must produce equivalent parse structure.
 *
 * Runtime random seeds MUST NOT influence parsing.
 *
 *
 * COMPATIBILITY TESTS
 *
 * Verify compatibility with:
 *
 *     lexer;
 *     parser;
 *     frontend AST;
 *     semantic analysis;
 *     canonical IR;
 *     AI composition;
 *     inference;
 *     training;
 *     differentiation;
 *     models;
 *     datasets;
 *     tensor/data layers;
 *     quantum subsystem;
 *     hardware/resource subsystem.
 *
 * ============================================================================
 * 39. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar passes the architectural hard-coding audit only when it
 * contains none of the following as universal language limits:
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
 *     MAX_SAMPLES
 *     MAX_RANDOM_VARIABLES
 *     MAX_DISTRIBUTIONS
 *     MAX_PARTICLES
 *     MAX_STATES
 *     MAX_PARAMETERS
 *
 * It must also contain no:
 *
 *     physical device identifiers;
 *     fixed accelerator selection;
 *     vendor-specific probability engine;
 *     fixed random hardware;
 *     fixed tensor rank;
 *     fixed distribution inventory.
 *
 * ============================================================================
 * 40. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when ALL of the following are true:
 *
 *     [ ] Canonical lexer dependency resolves.
 *
 *     [ ] Types dependency resolves.
 *
 *     [ ] Expressions dependency resolves.
 *
 *     [ ] Statements dependency resolves.
 *
 *     [ ] `identifier` is the canonical identifier rule.
 *
 *     [ ] `typeExpression` is the canonical type rule.
 *
 *     [ ] `expression` is the canonical expression rule.
 *
 *     [ ] `argumentList` is the canonical argument-list rule.
 *
 *     [ ] `statement` is the canonical statement rule.
 *
 *     [ ] No lexer keywords are added merely for probability concepts.
 *
 *     [ ] No fixed distribution inventory exists.
 *
 *     [ ] No fixed inference-algorithm inventory exists.
 *
 *     [ ] No fixed sampler inventory exists.
 *
 *     [ ] No fixed tensor rank exists.
 *
 *     [ ] No fixed sample count exists.
 *
 *     [ ] No hardware limits exist.
 *
 *     [ ] No device IDs are encoded.
 *
 *     [ ] No second type system exists.
 *
 *     [ ] No second expression system exists.
 *
 *     [ ] No second statement system exists.
 *
 *     [ ] No probabilistic IR is introduced by the grammar.
 *
 *     [ ] Quantum lowering remains through `quantum::ir`.
 *
 *     [ ] AI model syntax remains owned by models.g4.
 *
 *     [ ] Dataset syntax remains owned by datasets.g4.
 *
 *     [ ] Training syntax remains owned by training.g4.
 *
 *     [ ] General inference syntax remains owned by inference.g4.
 *
 *     [ ] Differentiation syntax remains owned by differentiation.g4.
 *
 *     [ ] Tensor syntax remains owned by the tensor/type layers.
 *
 *     [ ] Resource realization remains downstream.
 *
 *     [ ] Parser behavior is deterministic.
 *
 *     [ ] No semantic predicates exist.
 *
 *     [ ] No embedded actions exist.
 *
 *     [ ] No runtime execution exists.
 *
 *     [ ] No filesystem/network access exists.
 *
 *     [ ] Safe Rust 1.97/1.97.1 integration is maintained.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] AST mapping is predetermined.
 *
 *     [ ] Semantic mapping is predetermined.
 *
 *     [ ] IR mapping is predetermined.
 *
 *     [ ] Compiler consumers are identified.
 *
 *     [ ] Runtime consumers are identified.
 *
 * ============================================================================
 */