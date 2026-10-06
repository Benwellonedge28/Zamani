/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/distributions.g4
 *
 * Grammar:
 *     AIDistributions
 *
 * Status:
 *     CANONICAL / PRODUCTION DISTRIBUTION LEAF GRAMMAR
 *
 * Language baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the single parser-level owner of SOURCE-LEVEL DISTRIBUTION
 * SYNTAX used by probabilistic, statistical, scientific, AI, classical,
 * hybrid, quantum-derived, simulation, and future computational domains.
 *
 * The grammar represents distribution INTENT.
 *
 * It does not implement probability mathematics or distribution algorithms.
 *
 *
 * THIS FILE OWNS
 * --------------
 *
 *     distributionConstruct
 *     distributionDeclaration
 *     distributionExpression
 *     distributionReference
 *     distributionArguments
 *     distributionAnnotation
 *     distributionInitializer
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - probability arithmetic;
 *     - probability range validation;
 *     - probability precision;
 *     - random-number generation;
 *     - entropy acquisition;
 *     - sampling algorithms;
 *     - inference algorithms;
 *     - Bayesian inference;
 *     - MCMC;
 *     - variational inference;
 *     - particle methods;
 *     - filtering algorithms;
 *     - optimization;
 *     - automatic differentiation;
 *     - training;
 *     - datasets;
 *     - tensors;
 *     - general AI inference;
 *     - reasoning;
 *     - knowledge;
 *     - learning;
 *     - adaptation;
 *     - uncertainty semantics;
 *     - provenance implementation;
 *     - policy implementation;
 *     - effects;
 *     - capabilities;
 *     - resource allocation;
 *     - hardware selection;
 *     - quantum operation semantics;
 *     - quantum topology;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - backend selection.
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
 *     probabilistic / AI composition
 *       |
 *       v
 *     AIDistributions                 <-- THIS FILE
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +-------------------+
 *       |                   |
 *       v                   v
 *     type analysis     semantic analysis
 *                           |
 *               +-----------+-----------+
 *               |           |           |
 *               v           v           v
 *             effects   capabilities resources
 *                           |
 *                         policies
 *                           |
 *                       provenance
 *                           |
 *                           v
 *                 canonical semantic model
 *                           |
 *             +-------------+-------------+
 *             |             |             |
 *             v             v             v
 *         classical     quantum::ir    other domains
 *             |             |             |
 *             +-------------+-------------+
 *                           |
 *                     optimization
 *                           |
 *                      lowering
 *                           |
 *                  routing/scheduling
 *                           |
 *                 resilience where needed
 *                           |
 *                       target HAL
 *                           |
 *                    target realization
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Distribution syntax MUST have exactly one parser-level owner.
 *
 * Therefore:
 *
 *     grammar/ai/distributions.g4
 *
 * owns distribution syntax.
 *
 * Existing probabilistic grammars MUST consume the rules exported here rather
 * than maintaining a second implementation of distribution syntax.
 *
 * In particular, `grammar/ai/probabilistic.g4` MUST NOT retain a competing:
 *
 *     probabilisticDistributionExpression
 *     qualifiedProbabilisticName
 *     probabilisticDistributionArguments
 *
 * implementation after this file is integrated.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 *
 * The expression grammar supplies:
 *
 *     expression
 *     argumentList
 *
 * The canonical name grammar supplies:
 *
 *     qualifiedName
 *
 * The lexer supplies:
 *
 *     AT
 *     DISTRIBUTION
 *     ASSIGN
 *     LPAREN
 *     RPAREN
 *     SEMICOLON
 *
 * This grammar does not define any lexer rules.
 *
 *
 * EXPORTS
 * -------
 *
 *     distributionConstruct
 *     distributionDeclaration
 *     distributionExpression
 *     distributionReference
 *     distributionArguments
 *     distributionAnnotation
 *     distributionInitializer
 *
 *
 * CONSUMED_BY
 * -----------
 *
 * Primary:
 *
 *     grammar/ai/probabilistic.g4
 *
 * AI composition:
 *
 *     grammar/ai/ai.g4
 *
 * Indirect semantic consumers:
 *
 *     grammar/ai/inference.g4
 *     grammar/ai/training.g4
 *     grammar/ai/models.g4
 *     grammar/ai/datasets.g4
 *     grammar/ai/uncertainty.g4
 *     grammar/ai/differentiation.g4
 *     grammar/expressions/uncertainty.g4
 *     grammar/data/*
 *     grammar/classical/*
 *     grammar/hybrid/*
 *     grammar/quantum/*
 *     grammar/execution/*
 *     grammar/simulation/*
 *
 * These consumers MUST consume the semantic distribution representation
 * rather than creating their own distribution grammar.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser contexts only.
 *
 * The frontend AST owns the actual representation.
 *
 * The semantic AST mapping SHALL preserve at minimum:
 *
 *     - distribution source span;
 *     - distribution name/reference;
 *     - qualification segments;
 *     - argument ordering;
 *     - named arguments;
 *     - positional arguments;
 *     - initializer expression;
 *     - annotation/source form;
 *     - source-language version where applicable.
 *
 * Conceptual mapping:
 *
 *     distributionConstruct
 *         -> DistributionConstruct
 *
 *     distributionDeclaration
 *         -> DistributionDeclaration
 *
 *     distributionExpression
 *         -> DistributionExpression
 *
 *     distributionReference
 *         -> DistributionReference
 *
 *     distributionArguments
 *         -> DistributionArguments
 *
 *     distributionInitializer
 *         -> DistributionInitializer
 *
 * Exact Rust AST type names remain owned by the frontend AST subsystem.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A distribution reference is an unresolved semantic name until semantic
 * analysis determines what it denotes.
 *
 * It may denote:
 *
 *     - a standard library distribution;
 *     - a user-defined distribution;
 *     - a symbolic distribution;
 *     - a provider-defined distribution;
 *     - a scientific distribution;
 *     - a domain-specific distribution;
 *     - a distribution derived from data;
 *     - a distribution generated by a model;
 *     - a distribution derived from quantum measurement;
 *     - a simulation distribution;
 *     - a future distribution representation.
 *
 * This grammar intentionally does NOT decide which interpretation is correct.
 *
 *
 * ============================================================================
 * OPEN-WORLD DISTRIBUTION MODEL
 * ============================================================================
 *
 * The grammar MUST NOT enumerate distribution families.
 *
 * DO NOT add grammar alternatives for:
 *
 *     Normal
 *     Gaussian
 *     Bernoulli
 *     Binomial
 *     Poisson
 *     Uniform
 *     Categorical
 *     Multinomial
 *     Exponential
 *     Gamma
 *     Beta
 *     Dirichlet
 *     LogNormal
 *     Weibull
 *     StudentT
 *     or future distribution families.
 *
 * These are semantic names.
 *
 * A distribution library may introduce any number of distributions without
 * modifying this grammar.
 *
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Distribution names use the canonical Zamani name system.
 *
 * Examples:
 *
 *     Normal
 *
 *     statistics::Normal
 *
 *     scientific::probability::Normal
 *
 *     provider::distribution
 *
 *     user::models::custom_distribution
 *
 * Qualification depth is not bounded by this grammar.
 *
 * The parser does not resolve the name.
 *
 *
 * ============================================================================
 * ARGUMENT CONTRACT
 * ============================================================================
 *
 * Distribution arguments reuse the canonical expression argument list.
 *
 * Therefore arguments may be:
 *
 *     literals;
 *     identifiers;
 *     symbolic expressions;
 *     function calls;
 *     tensor expressions;
 *     model results;
 *     data values;
 *     quantum-derived values;
 *     measurements;
 *     simulation results;
 *     distributed values;
 *     future domain values.
 *
 * The distribution grammar does not create another expression language.
 *
 *
 * ============================================================================
 * NAMED-ARGUMENT CONTRACT
 * ============================================================================
 *
 * Named distribution parameters are handled by the canonical argument
 * grammar.
 *
 * Examples:
 *
 *     distribution(mu: mean, sigma: spread)
 *
 *     distribution(mean = mean_value, scale = scale_value)
 *
 * The distribution grammar does not enumerate parameter names.
 *
 * Therefore:
 *
 *     mean
 *     variance
 *     scale
 *     rate
 *     location
 *     shape
 *     covariance
 *     degrees
 *     concentration
 *
 * remain ordinary semantic names.
 *
 *
 * ============================================================================
 * DECLARATION CONTRACT
 * ============================================================================
 *
 * A distribution declaration establishes a source-level named distribution
 * construct.
 *
 * Canonical shape:
 *
 *     @distribution Name(...);
 *
 * or:
 *
 *     @distribution Name = expression;
 *
 * The meaning of the declaration is determined semantically.
 *
 * The grammar does not decide whether the declaration represents:
 *
 *     - an alias;
 *     - a parameterized distribution;
 *     - a distribution definition;
 *     - a symbolic distribution;
 *     - a provider binding;
 *     - a model-derived distribution;
 *     - another language-defined distribution abstraction.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * A distribution expression represents a value-producing distribution
 * expression.
 *
 * Canonical shapes include:
 *
 *     Normal(mu, sigma)
 *
 *     statistics::Normal(mu, sigma)
 *
 *     custom::distribution(parameters)
 *
 *     distribution_expression
 *
 * The grammar only establishes:
 *
 *     reference + optional arguments
 *
 * Semantic analysis establishes:
 *
 *     whether the referenced entity is actually a distribution.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This grammar does not define a Distribution type.
 *
 * If Zamani's type system exposes a distribution type, its ownership belongs
 * to the canonical type subsystem.
 *
 * Distribution expressions may therefore participate in:
 *
 *     generic types;
 *     tensor types;
 *     function types;
 *     records;
 *     algebraic types;
 *     dependent constraints;
 *     uncertainty types;
 *     model types;
 *     quantum-derived values;
 *     data types.
 *
 * No distribution-specific physical type representation is required here.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Merely parsing or constructing a distribution expression does not imply an
 * execution effect.
 *
 * Semantic analysis may attach effects when the distribution is subsequently
 * used for:
 *
 *     sampling;
 *     stochastic execution;
 *     random generation;
 *     external entropy;
 *     network-backed computation;
 *     native execution;
 *     foreign execution;
 *     adaptation;
 *     simulation.
 *
 * Distribution declaration syntax MUST NOT silently grant such effects.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Distribution syntax itself does not require a physical capability.
 *
 * Semantic consumers may require capabilities such as:
 *
 *     capability("probability");
 *     capability("probabilistic.compute");
 *     capability("randomness");
 *     capability("sampling");
 *     capability("statistical.inference");
 *     capability("symbolic.probability");
 *     capability("tensor.compute");
 *     capability("quantum.measurement");
 *
 * Capability names remain semantic data.
 *
 * The grammar does not enumerate or resolve capabilities.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Distribution syntax imposes NO universal resource limit.
 *
 * Resource requirements may be derived downstream from:
 *
 *     number of variables;
 *     argument expressions;
 *     model size;
 *     sample count;
 *     tensor dimensions;
 *     precision requirements;
 *     execution strategy;
 *     distribution representation;
 *     target capabilities;
 *     requested reproducibility;
 *     requested throughput;
 *     resilience requirements.
 *
 * Those are semantic/resource-analysis concerns.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Distribution expressions may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * without introducing distribution-specific contract syntax.
 *
 * For example, a contract may semantically constrain a distribution-related
 * property without the distribution grammar needing a new rule for every
 * mathematical property.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may control distribution realization.
 *
 * Examples include:
 *
 *     deterministic replay;
 *     permitted randomness sources;
 *     entropy policy;
 *     privacy policy;
 *     numerical policy;
 *     resource policy;
 *     execution policy;
 *     sandbox policy;
 *     reproducibility policy.
 *
 * Policies are owned elsewhere.
 *
 * This grammar only preserves the distribution syntax needed for semantic
 * policy analysis.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Distribution source structure must remain traceable.
 *
 * Provenance may record:
 *
 *     source distribution reference;
 *     parameter expressions;
 *     declaration site;
 *     model/data origin;
 *     transformation;
 *     derivation;
 *     evidence;
 *     semantic resolution;
 *     lowering decisions;
 *     execution realization.
 *
 * This grammar does not implement provenance.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT introduce a distribution-specific IR.
 *
 * The semantic representation must lower through the canonical semantic
 * model.
 *
 * Depending on use, a distribution may ultimately participate in:
 *
 *     classical IR;
 *     tensor/data representation;
 *     AI/model representation;
 *     distributed representation;
 *     quantum::ir;
 *     HDL/hardware representation;
 *     execution plans.
 *
 * The existence of this grammar does not create:
 *
 *     DistributionIR
 *     ProbabilityIR
 *     DistributionQuantumIR
 *     DistributionHardwareIR
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Distribution syntax may consume values originating from quantum computation.
 *
 * Examples:
 *
 *     distribution_from_measurement
 *
 *     quantum::measurement_result
 *
 *     model(quantum_result)
 *
 * The distribution grammar does not define quantum syntax.
 *
 * The canonical path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *
 * No secondary quantum IR is introduced.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A distribution may eventually be realized using:
 *
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU-assisted computation;
 *     simulator;
 *     embedded hardware;
 *     HPC;
 *     cluster;
 *     distributed system;
 *     cloud;
 *     future computational substrate.
 *
 * This grammar does not select any of them.
 *
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backend selection occurs after:
 *
 *     parsing
 *       ->
 *     AST
 *       ->
 *     semantic analysis
 *       ->
 *     type/effect/capability/resource analysis
 *       ->
 *     canonical semantic model
 *       ->
 *     target-aware lowering.
 *
 * A backend may specialize a distribution implementation without changing the
 * source grammar.
 *
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar intentionally has NO artificial finite ceiling for:
 *
 *     - number of distributions;
 *     - number of distribution declarations;
 *     - number of parameters;
 *     - number of arguments;
 *     - qualification depth;
 *     - expression complexity;
 *     - model size;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - sample count;
 *     - variable count;
 *     - worker count;
 *     - device count;
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - accelerator count;
 *     - QPU count;
 *     - node count;
 *     - memory capacity;
 *     - network size.
 *
 * Repetition is represented structurally by ANTLR repetition constructs.
 *
 * Any finite practical parser/compiler limit is an implementation/resource
 * constraint, not a language semantic ceiling.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN:
 *
 *     fixed distribution catalog;
 *     fixed algorithm catalog;
 *     fixed parameter count;
 *     fixed sample count;
 *     fixed tensor rank;
 *     fixed tensor dimension;
 *     fixed CPU count;
 *     fixed GPU count;
 *     fixed FPGA count;
 *     fixed accelerator count;
 *     fixed QPU count;
 *     fixed node count;
 *     fixed memory capacity;
 *     fixed device count;
 *     fixed hardware ID;
 *     vendor-specific parser branch;
 *     backend-specific parser branch.
 *
 * ALSO FORBIDDEN:
 *
 *     semantic actions;
 *     parser-time execution;
 *     random-number generation;
 *     target probing;
 *     hardware probing;
 *     filesystem access;
 *     network access.
 *
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * The only reserved lexical word specifically required for the canonical
 * declaration surface is:
 *
 *     DISTRIBUTION
 *
 * The grammar also permits the annotation spelling to remain extensible by
 * accepting an identifier after AT.
 *
 * This is intentional because future distribution dialects must not require
 * a lexer change merely to introduce a new annotation namespace.
 *
 *
 * IMPORTANT:
 *
 * `@distribution` is accepted explicitly through the canonical DISTRIBUTION
 * token because DISTRIBUTION is a reserved lexical word in the repository.
 *
 * Therefore a generic:
 *
 *     AT identifier
 *
 * alone is insufficient for the canonical `@distribution` spelling.
 *
 * This file corrects that lexical/parser boundary.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source forms supported by probabilistic.g4 must remain representable
 * after migration:
 *
 *     @distribution Name(...);
 *
 *     @distribution Name = expression;
 *
 *     @random x ~ Name(...);
 *
 *     @random x: Type ~ Name(...);
 *
 *     @random x = Name(...);
 *
 * Existing distribution names remain semantic identifiers.
 *
 * No source program should need to change merely because a new distribution
 * implementation is added.
 *
 *
 * ============================================================================
 * MIGRATION CONTRACT
 * ============================================================================
 *
 * `grammar/ai/probabilistic.g4` SHALL:
 *
 * 1. import AIDistributions;
 *
 * 2. replace its local distribution-expression implementation with:
 *
 *        distributionExpression
 *
 * 3. remove:
 *
 *        probabilisticDistributionExpression
 *        probabilisticDistributionArguments
 *        qualifiedProbabilisticName
 *
 *    once all consumers have migrated;
 *
 * 4. keep random-variable, sampling, observation, conditioning, prior,
 *    posterior, likelihood, and inference ownership in probabilistic.g4;
 *
 * 5. use `distributionExpression` wherever a distribution value is expected.
 *
 * This produces a single distribution syntax authority.
 *
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * `grammar/ai/ai.g4` remains the AI composition boundary.
 *
 * It MUST NOT create a second distribution rule.
 *
 * The preferred dependency path is:
 *
 *     AI
 *       |
 *       +--> probabilistic composition
 *                 |
 *                 +--> AIDistributions
 *
 * If the canonical parser directly imports `AIProbabilistic`, then `AI.g4`
 * does not need to independently expose `AIDistributions`.
 *
 * The distribution grammar must not be independently added as a competing
 * top-level parser alternative if that would allow:
 *
 *     ordinary expression
 *
 * and:
 *
 *     distribution expression
 *
 * to enter through competing parser paths.
 *
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * `grammar/expressions/uncertainty.g4` owns uncertainty VALUE expression
 * syntax.
 *
 * It may consume distribution values semantically.
 *
 * It must not duplicate the distribution grammar.
 *
 * Conceptually:
 *
 *     uncertainty(value, distribution: D)
 *
 * where `D` is a semantic distribution value.
 *
 *
 * ============================================================================
 * PROBABILISTIC INTEGRATION
 * ============================================================================
 *
 * `grammar/ai/probabilistic.g4` owns the probabilistic computation domain.
 *
 * It consumes:
 *
 *     distributionExpression
 *
 * for:
 *
 *     random variables;
 *     distribution declarations;
 *     sampling;
 *     priors;
 *     posteriors;
 *     likelihoods;
 *     conditioning;
 *     probabilistic inference.
 *
 * It does NOT redefine distribution syntax.
 *
 *
 * ============================================================================
 * INFERENCE INTEGRATION
 * ============================================================================
 *
 * `grammar/ai/inference.g4` owns general inference.
 *
 * Distribution values may participate in inference as ordinary semantic
 * values.
 *
 * This grammar does not define inference algorithms.
 *
 *
 * ============================================================================
 * TRAINING INTEGRATION
 * ============================================================================
 *
 * Training may use distributions for:
 *
 *     initialization;
 *     stochastic objectives;
 *     priors;
 *     likelihoods;
 *     sampling;
 *     probabilistic models;
 *     uncertainty estimation.
 *
 * Training syntax remains owned by its canonical grammar.
 *
 *
 * ============================================================================
 * DIFFERENTIATION INTEGRATION
 * ============================================================================
 *
 * Distribution parameter expressions may be differentiation subjects.
 *
 * This grammar does not define:
 *
 *     gradients;
 *     Jacobians;
 *     Hessians;
 *     automatic differentiation;
 *     adjoint differentiation;
 *     parameter-shift rules.
 *
 * Such semantics belong downstream.
 *
 *
 * ============================================================================
 * DATA / TENSOR INTEGRATION
 * ============================================================================
 *
 * Distribution arguments may contain data and tensor expressions.
 *
 * No tensor grammar is duplicated here.
 *
 * No tensor rank or dimension is encoded.
 *
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Distribution values may participate in ordinary classical computation.
 *
 * The same source representation remains valid regardless of whether the
 * eventual realization uses:
 *
 *     scalar computation;
 *     vector computation;
 *     tensor computation;
 *     symbolic computation;
 *     numerical computation;
 *     accelerator computation.
 *
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Distribution expressions may bridge:
 *
 *     classical -> quantum-derived data
 *     quantum -> classical probability
 *     AI -> probability
 *     probability -> classical control
 *     simulation -> probability
 *     probability -> simulation
 *
 * These are semantic relationships.
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distribution execution may be distributed.
 *
 * This grammar does not encode:
 *
 *     node counts;
 *     worker counts;
 *     cluster topology;
 *     network addresses;
 *     process IDs.
 *
 * Those belong to distributed/execution/resource subsystems.
 *
 *
 * ============================================================================
 * SECURITY / SANDBOX INTEGRATION
 * ============================================================================
 *
 * A distribution implementation may require:
 *
 *     randomness;
 *     external entropy;
 *     native execution;
 *     foreign calls;
 *     filesystem/data access;
 *     network services.
 *
 * These effects and capabilities must be declared/resolved downstream.
 *
 * Parsing a distribution name does not grant any permission.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Identical:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *     parser configuration;
 *
 * MUST produce the same parse structure.
 *
 * Runtime random seeds, entropy sources, hardware availability, current time,
 * network state, filesystem state, and scheduler state MUST NOT affect parsing.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics SHALL identify malformed syntax such as:
 *
 *     @distribution
 *
 *     @distribution Name(
 *
 *     @distribution = expression;
 *
 *     @distribution Name =
 *
 *     @distribution Name(... malformed ...);
 *
 * Semantic diagnostics belong downstream and may report:
 *
 *     unknown distribution;
 *     unknown distribution namespace;
 *     invalid parameter;
 *     invalid parameter type;
 *     invalid parameter relationship;
 *     unavailable distribution implementation;
 *     unavailable capability;
 *     unsatisfied resource requirement;
 *     unsupported target realization.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms MUST parse:
 *
 *     @distribution Normal;
 *
 *     @distribution Normal(mu, sigma);
 *
 *     @distribution statistics::Normal(mu, sigma);
 *
 *     @distribution custom::distribution(parameter);
 *
 *     @distribution custom::distribution(
 *         mean: mean_value,
 *         scale: scale_value
 *     );
 *
 *     @distribution Normal = expression;
 *
 *     @random x ~ Normal(mu, sigma);
 *
 *     @random x: Real ~ statistics::Normal(mu, sigma);
 *
 *     @random x = custom::distribution(parameter);
 *
 * The random-variable examples are integration tests for probabilistic.g4;
 * the distribution expression itself is owned here.
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following MUST fail syntactically where this grammar is invoked through
 * its declaration boundary:
 *
 *     @distribution;
 *
 *     @distribution = expression;
 *
 *     @distribution Name =;
 *
 *     @distribution Name(
 *
 *     @distribution Name(;
 *
 *     @distribution Name = ;
 *
 * Malformed argument expressions must be rejected by the canonical expression
 * grammar.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     one distribution;
 *     many declarations;
 *     deeply qualified distribution names;
 *     empty argument lists;
 *     large argument lists;
 *     nested argument expressions;
 *     nested distribution expressions;
 *     symbolic parameters;
 *     tensor parameters;
 *     quantum-derived parameters;
 *     distributed values;
 *     simulation results.
 *
 * No finite grammar-level boundary may be introduced.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Verify:
 *
 *     classical -> distribution
 *     distribution -> classical
 *     AI model -> distribution
 *     dataset -> distribution parameter
 *     tensor -> distribution parameter
 *     quantum measurement -> distribution parameter
 *     distribution -> quantum-derived computation
 *     distribution -> simulation
 *     distribution -> distributed execution
 *     distribution -> accelerator execution
 *     distribution -> HDL/hardware realization
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite must verify that distribution syntax remains valid as
 * actual implementation resources increase or decrease.
 *
 * Tests must vary:
 *
 *     declaration count;
 *     qualification depth;
 *     parameter count;
 *     expression complexity;
 *     symbolic expression size;
 *     tensor structure;
 *     model structure;
 *     dataset structure.
 *
 * Tests MUST NOT turn a measured parser/resource limit into a language
 * constant.
 *
 *
 * ============================================================================
 * COMPATIBILITY TEST CONTRACT
 * ============================================================================
 *
 * Verify compatibility with:
 *
 *     grammar/ai/probabilistic.g4
 *     grammar/ai/uncertainty.g4
 *     grammar/expressions/uncertainty.g4
 *     grammar/ai/inference.g4
 *     grammar/ai/training.g4
 *     grammar/ai/models.g4
 *     grammar/ai/datasets.g4
 *     grammar/ai/differentiation.g4
 *     grammar/types/
 *     grammar/data/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/execution/
 *     grammar/resources/
 *     grammar/effects/
 *     grammar/validation/
 *     grammar/policies/
 *     grammar/interoperability/
 *
 *
 * ============================================================================
 * FILE-BY-FILE INTEGRATION CONTRACT
 * ============================================================================
 *
 * `grammar/ai/probabilistic.g4`
 * --------------------------------
 *
 * MUST import:
 *
 *     AIDistributions
 *
 * MUST consume:
 *
 *     distributionExpression
 *
 * MUST remove its competing distribution-expression rules after migration.
 *
 *
 * `grammar/ai/ai.g4`
 * ------------------
 *
 * MUST NOT define distribution syntax.
 *
 * It remains the AI composition layer.
 *
 *
 * `grammar/expressions/expressions.g4`
 * -------------------------------------
 *
 * Remains the sole expression precedence authority.
 *
 * It MUST NOT import this grammar merely to make distribution names valid
 * expressions.
 *
 * Distribution expressions enter ordinary expression contexts through the
 * probabilistic/uncertainty semantic boundaries where appropriate.
 *
 *
 * `grammar/expressions/uncertainty.g4`
 * -------------------------------------
 *
 * May consume distribution values semantically.
 *
 * It MUST NOT redefine distribution references or arguments.
 *
 *
 * `grammar/types/types.g4`
 * ------------------------
 *
 * Owns any future canonical distribution type representation.
 *
 * This file must not create one.
 *
 *
 * `grammar/ai/inference.g4`
 * -------------------------
 *
 * Owns general inference syntax.
 *
 * This file supplies distribution values only.
 *
 *
 * `grammar/ai/training.g4`
 * ------------------------
 *
 * Owns training syntax.
 *
 * Distribution use is represented through expressions/semantic values.
 *
 *
 * `grammar/resources/`
 * --------------------
 *
 * Owns capability/resource requirements.
 *
 * This grammar does not allocate or select resources.
 *
 *
 * `grammar/effects/`
 * ------------------
 *
 * Owns effects.
 *
 * Distribution construction itself does not automatically authorize
 * randomness, native execution, foreign calls, or network access.
 *
 *
 * `grammar/policies/`
 * -------------------
 *
 * Owns policy syntax.
 *
 * Distribution policy remains a semantic concern.
 *
 *
 * `grammar/validation/`
 * ---------------------
 *
 * Owns contracts and validation.
 *
 * Distribution-specific mathematical validity belongs semantic validation,
 * not parser syntax.
 *
 *
 * ============================================================================
 * DEPENDENCY METADATA
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 * EXPORTS:
 *
 *     distributionConstruct
 *     distributionDeclaration
 *     distributionExpression
 *     distributionReference
 *     distributionArguments
 *     distributionAnnotation
 *     distributionInitializer
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/probabilistic.g4
 *     grammar/ai/*
 *     semantic AI/probabilistic layer
 *
 * AST_OWNER:
 *
 *     frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     semantic distribution/probability subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic IR
 *     quantum::ir where quantum computation participates
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/distributions/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/specification/
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE only when:
 *
 *     [ ] It is the sole parser owner of distribution syntax.
 *
 *     [ ] It uses ZamaniLexer.
 *
 *     [ ] It reuses canonical qualified names.
 *
 *     [ ] It reuses canonical expressions.
 *
 *     [ ] It reuses canonical argument lists.
 *
 *     [ ] It defines no lexer rules.
 *
 *     [ ] It defines no expression-precedence hierarchy.
 *
 *     [ ] It defines no second type system.
 *
 *     [ ] It defines no inference algorithm catalog.
 *
 *     [ ] It defines no distribution family catalog.
 *
 *     [ ] It defines no sampling algorithm catalog.
 *
 *     [ ] It defines no random-number generator.
 *
 *     [ ] It defines no hardware catalog.
 *
 *     [ ] It defines no vendor catalog.
 *
 *     [ ] It defines no backend selection.
 *
 *     [ ] It defines no machine-capacity limit.
 *
 *     [ ] It defines no quantum-resource limit.
 *
 *     [ ] It defines no tensor-rank limit.
 *
 *     [ ] It defines no sample-count limit.
 *
 *     [ ] It defines no parser-time execution.
 *
 *     [ ] It contains no semantic predicates.
 *
 *     [ ] It contains no embedded actions.
 *
 *     [ ] It requires no unsafe Rust.
 *
 *     [ ] It remains compatible with Rust 1.97+ generated-parser consumers.
 *
 *     [ ] `@distribution` is accepted through the canonical DISTRIBUTION token.
 *
 *     [ ] extensible annotation identifiers remain possible where intended.
 *
 *     [ ] distribution names remain open-world identifiers.
 *
 *     [ ] distribution parameters remain open-world expressions.
 *
 *     [ ] probabilistic.g4 delegates to this grammar.
 *
 *     [ ] uncertainty grammar does not duplicate this grammar.
 *
 *     [ ] AI composition does not create a second distribution path.
 *
 *     [ ] positive tests pass.
 *
 *     [ ] negative tests pass.
 *
 *     [ ] boundary tests pass.
 *
 *     [ ] cross-domain tests pass.
 *
 *     [ ] scalability tests pass.
 *
 *     [ ] deterministic parsing tests pass.
 *
 *     [ ] compatibility tests pass.
 *
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file defines HOW A DISTRIBUTION IS WRITTEN.
 *
 * It does not define:
 *
 *     WHICH distributions exist;
 *     HOW they are computed;
 *     WHERE they execute;
 *     HOW much hardware exists;
 *     HOW much memory exists;
 *     HOW many processors exist;
 *     HOW many quantum resources exist;
 *     WHICH vendor provides the implementation;
 *     WHICH backend is selected.
 *
 * Therefore:
 *
 *     distribution syntax
 *          !=
 *     distribution implementation
 *
 *     distribution implementation
 *          !=
 *     hardware realization
 *
 *     source portability
 *          !=
 *     guaranteed physical feasibility
 *
 * The source representation remains portable while semantic analysis,
 * capability negotiation, resource analysis, optimization, lowering,
 * scheduling, resilience, and target realization determine how the requested
 * computation can actually be performed.
 *
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar AIDistributions;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * Only declarations are exposed as a standalone distribution construct.
 *
 * `distributionExpression` is intentionally a reusable leaf rule and is not
 * exposed as an unrestricted top-level construct.
 *
 * This prevents an ordinary identifier/expression from accidentally becoming
 * a distribution construct merely because this grammar is imported.
 *
 * ============================================================================
 */

distributionConstruct
    : distributionDeclaration
    ;


/*
 * ============================================================================
 * DISTRIBUTION DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     @distribution Normal;
 *
 *     @distribution Normal(mu, sigma);
 *
 *     @distribution Normal = expression;
 *
 *     @distribution custom::distribution(parameter);
 *
 * The semantic layer determines the exact declaration category.
 *
 * ============================================================================
 */

distributionDeclaration
    : distributionAnnotation
      distributionReference
      distributionArguments?
      distributionInitializer?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * DISTRIBUTION ANNOTATION
 * ============================================================================
 *
 * `distribution` is a canonical reserved lexical token in the repository.
 *
 * Therefore the canonical spelling:
 *
 *     @distribution
 *
 * must explicitly consume DISTRIBUTION.
 *
 * The identifier alternative keeps the annotation boundary extensible for
 * dialect/provider-specific declaration forms without introducing a new
 * parser rule for every future annotation.
 * ============================================================================
 */

distributionAnnotation
    : AT DISTRIBUTION
    | AT identifier
    ;


/*
 * ============================================================================
 * DISTRIBUTION EXPRESSION
 * ============================================================================
 *
 * This is the reusable value-level distribution representation.
 *
 * Examples:
 *
 *     Normal
 *
 *     Normal(mu, sigma)
 *
 *     statistics::Normal(mu, sigma)
 *
 *     custom::distribution(parameter)
 *
 * ============================================================================
 */

distributionExpression
    : distributionReference
      distributionArguments?
    ;


/*
 * ============================================================================
 * DISTRIBUTION REFERENCE
 * ============================================================================
 *
 * Canonical qualified-name ownership remains in grammar/core/names.g4.
 *
 * No distribution-specific name grammar is introduced.
 * ============================================================================
 */

distributionReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * DISTRIBUTION ARGUMENTS
 * ============================================================================
 *
 * The canonical expression grammar owns argumentList.
 *
 * No parameter-count limit is encoded.
 * ============================================================================
 */

distributionArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * DISTRIBUTION INITIALIZER
 * ============================================================================
 *
 * The initializer is an ordinary Zamani expression.
 *
 * It may eventually denote:
 *
 *     a symbolic distribution;
 *     a distribution transformation;
 *     a model result;
 *     a data-derived distribution;
 *     a quantum-derived value;
 *     a simulation result;
 *     another domain value.
 * ============================================================================
 */

distributionInitializer
    : ASSIGN
      expression
    ;