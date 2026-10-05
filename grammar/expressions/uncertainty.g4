/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/uncertainty.g4
 *
 * Grammar:
 *     UncertaintyExpressions
 *
 * Status:
 *     Canonical production expression component.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL UNCERTAINTY VALUE EXPRESSION SYNTAX.
 *
 * The construct represents a value whose interpretation may include:
 *
 *     uncertainty
 *     probability
 *     distribution
 *     confidence
 *     belief
 *     likelihood
 *     evidence
 *     provenance
 *
 * The grammar describes syntax and intent only.
 *
 * It does NOT prescribe:
 *
 *     - floating-point representation;
 *     - arbitrary-precision representation;
 *     - probability implementation;
 *     - probability precision;
 *     - distribution family;
 *     - statistical algorithm;
 *     - inference algorithm;
 *     - sampling algorithm;
 *     - random-number generator;
 *     - machine-learning framework;
 *     - quantum backend;
 *     - hardware target;
 *     - storage engine;
 *     - execution strategy.
 *
 * Those concerns belong to semantic analysis, types, effects, capabilities,
 * resources, policies, execution and target realization.
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
 *     parser
 *       |
 *       v
 *     uncertaintyExpression
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
 *       |                   |
 *       +---------+---------+
 *                 |
 *                 v
 *          effects/capabilities
 *                 |
 *              resources
 *                 |
 *              policies
 *                 |
 *             provenance
 *                 |
 *                 v
 *       canonical semantic model
 *                 |
 *       +---------+----------+----------------+
 *       |                    |                |
 *       v                    v                v
 *   classical            quantum::ir       other domains
 *       |                    |                |
 *       +--------------------+----------------+
 *                            |
 *                            v
 *                    optimization/lowering
 *                            |
 *                     target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
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
 * THIS FILE DOES NOT OWN:
 *
 *     - expression precedence;
 *     - assignment;
 *     - arithmetic;
 *     - logical operators;
 *     - calls;
 *     - indexing;
 *     - member access;
 *     - identifiers;
 *     - literals;
 *     - patterns;
 *     - guards;
 *     - type syntax;
 *     - probability algorithms;
 *     - statistical algorithms;
 *     - inference algorithms;
 *     - learning algorithms;
 *     - reasoning;
 *     - knowledge storage;
 *     - provenance implementation;
 *     - policy implementation;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - runtime execution;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - hardware selection.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/punctuation.g4
 *     grammar/lexer/operators.g4
 *     canonical expression-core boundary
 *
 * The canonical expression boundary supplies:
 *
 *     expression
 *
 * This file deliberately does NOT import the complete Expressions grammar.
 *
 * The dependency direction must remain:
 *
 *     expression composition
 *             |
 *             +----> uncertainty expression
 *
 * and never:
 *
 *     uncertainty expression
 *             |
 *             +----> complete expression composition
 *
 * This prevents a parser-grammar dependency cycle.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public rule:
 *
 *     uncertaintyExpression
 *
 * Supporting rules:
 *
 *     uncertainValueExpression
 *     uncertaintyValue
 *     uncertaintyArgumentList
 *     uncertaintyArgument
 *     uncertaintyNamedArgument
 *     uncertaintyPositionalArgument
 *     uncertaintyFieldName
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/expressions/expressions.g4
 *
 * through:
 *
 *     primaryExpression
 *
 * The canonical integration is:
 *
 *     primaryExpression
 *         :
 *             ...
 *           | uncertaintyExpression
 *           ;
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * Canonical uncertainty vocabulary already provided by the lexical layer:
 *
 *     UNCERTAIN
 *     UNCERTAINTY
 *     PROBABILITY
 *     PROBABILISTIC
 *     DISTRIBUTION
 *     CONFIDENCE
 *     BELIEF
 *     LIKELIHOOD
 *     EVIDENCE
 *     PROVENANCE
 *     SOURCE
 *
 * This grammar consumes those canonical tokens.
 *
 * It MUST NOT invent parser-side aliases such as:
 *
 *     KeywordUncertain
 *     KeywordUncertainty
 *     KeywordProbability
 *     KeywordConfidence
 *
 * The canonical lexer token names are the source of truth.
 *
 * ============================================================================
 * WHY TWO INTRODUCERS
 * ============================================================================
 *
 * The language already reserves both:
 *
 *     uncertain
 *     uncertainty
 *
 * They are allowed to converge on one semantic abstraction.
 *
 * Both forms therefore produce the same conceptual semantic category:
 *
 *     UncertainValue
 *
 * while preserving the source-level spelling for:
 *
 *     diagnostics
 *     formatting
 *     provenance
 *     compatibility tooling
 *     source maps.
 *
 * Canonical forms:
 *
 *     uncertain(value)
 *
 *     uncertainty(value)
 *
 * Both may carry semantic attributes:
 *
 *     uncertain(value, confidence: c)
 *
 *     uncertain(value, probability: p)
 *
 *     uncertain(value, distribution: d)
 *
 *     uncertainty(value, evidence: e)
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Uncertainty metadata must remain extensible.
 *
 * The grammar therefore does NOT enumerate every future statistical property.
 *
 * Supported standardized field names include:
 *
 *     probability
 *     distribution
 *     confidence
 *     belief
 *     likelihood
 *     evidence
 *     provenance
 *     source
 *
 * User/library/provider-defined metadata can use:
 *
 *     identifier: expression
 *
 * without requiring a new universal keyword.
 *
 * This permits future concepts such as:
 *
 *     interval
 *     variance
 *     covariance
 *     entropy
 *     weight
 *     posterior
 *     prior
 *     calibration
 *     reliability
 *
 * without changing this grammar merely because a new semantic property is
 * introduced.
 *
 * ============================================================================
 * CANONICAL SURFACE FORMS
 * ============================================================================
 *
 * Basic:
 *
 *     uncertain(value)
 *
 *     uncertainty(value)
 *
 * Confidence:
 *
 *     uncertain(value, confidence: 0.95)
 *
 * Probability:
 *
 *     uncertain(value, probability: probability_value)
 *
 * Distribution:
 *
 *     uncertainty(value, distribution: distribution_value)
 *
 * Evidence:
 *
 *     uncertainty(value, evidence: evidence_value)
 *
 * Provenance:
 *
 *     uncertainty(value, provenance: provenance_value)
 *
 * Multiple properties:
 *
 *     uncertainty(
 *         value,
 *         probability: probability_value,
 *         confidence: confidence_value,
 *         evidence: evidence_value,
 *         provenance: provenance_value
 *     )
 *
 * Provider/application extension:
 *
 *     uncertainty(
 *         value,
 *         calibration: calibration_value
 *     )
 *
 * The last form remains syntactically open because `calibration` is an
 * ordinary identifier unless the wider language reserves it.
 *
 * ============================================================================
 * VALUE BOUNDARY
 * ============================================================================
 *
 * `uncertaintyValue` is the canonical expression boundary.
 *
 * Therefore the value can eventually be:
 *
 *     identifier
 *     literal
 *     arithmetic expression
 *     tensor expression
 *     function result
 *     model result
 *     knowledge query
 *     reasoning result
 *     learning result
 *     quantum measurement
 *     classical measurement
 *     distributed result
 *     hardware observation
 *     simulation result
 *     another uncertainty expression
 *
 * This grammar does not reproduce expression precedence.
 *
 * The canonical expression system remains responsible for that.
 *
 * ============================================================================
 * ARGUMENT ORDER
 * ============================================================================
 *
 * The first argument is the uncertain value.
 *
 * Additional arguments are metadata/options.
 *
 * The grammar preserves source order.
 *
 * Semantic validation determines:
 *
 *     - duplicate metadata;
 *     - conflicting metadata;
 *     - unsupported metadata;
 *     - incompatible metadata types;
 *     - provider-specific fields.
 *
 * ============================================================================
 * NAMED ARGUMENTS
 * ============================================================================
 *
 * Named uncertainty metadata supports both established Zamani named-argument
 * separators:
 *
 *     :
 *
 * and:
 *
 *     =
 *
 * Examples:
 *
 *     uncertain(x, confidence: c)
 *
 *     uncertain(x, confidence = c)
 *
 * The semantic layer determines whether the two forms are equivalent under
 * the active language/compatibility version.
 *
 * ============================================================================
 * POSITIONAL ARGUMENTS
 * ============================================================================
 *
 * Additional positional arguments are syntactically permitted so that future
 * uncertainty providers can represent structured source-level values without
 * requiring a new grammar.
 *
 * Example:
 *
 *     uncertainty(value, metadata)
 *
 * Their meaning is semantic.
 *
 * A conforming semantic implementation may reject an unsupported positional
 * form even though the syntax is valid.
 *
 * This keeps syntax extensible while preserving semantic validation.
 *
 * ============================================================================
 * FIELD NAMES
 * ============================================================================
 *
 * Standard field names are represented by canonical lexer tokens.
 *
 * Extension fields use IDENTIFIER.
 *
 * This avoids creating a global keyword for every possible uncertainty
 * property.
 *
 * Standard fields:
 *
 *     PROBABILITY
 *     DISTRIBUTION
 *     CONFIDENCE
 *     BELIEF
 *     LIKELIHOOD
 *     EVIDENCE
 *     PROVENANCE
 *     SOURCE
 *
 * ============================================================================
 * NO NUMERICAL ASSUMPTIONS
 * ============================================================================
 *
 * This grammar does NOT require:
 *
 *     probability in [0, 1]
 *
 * because that is semantic validation.
 *
 * It does not impose:
 *
 *     f32
 *     f64
 *     fixed-point
 *     arbitrary precision
 *     symbolic probability
 *     interval probability
 *     logarithmic representation
 *     quantum probability representation
 *
 * The semantic/type system decides the representation.
 *
 * ============================================================================
 * NO FIXED DISTRIBUTION CATALOG
 * ============================================================================
 *
 * This grammar does NOT enumerate:
 *
 *     normal
 *     Gaussian
 *     Bernoulli
 *     binomial
 *     Poisson
 *     categorical
 *     Dirichlet
 *     custom distributions
 *
 * Those names remain identifiers or library-level constructs.
 *
 * A distribution can therefore be supplied as:
 *
 *     distribution: gaussian(...)
 *
 *     distribution: custom_distribution(...)
 *
 *     distribution: model.distribution(...)
 *
 * without modifying this grammar.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * The syntax itself does not declare an effect.
 *
 * Semantic analysis may infer effects depending on the resolved value or
 * metadata, including:
 *
 *     randomness
 *     measurement
 *     learning
 *     network
 *     IO
 *     foreign
 *     distributed
 *     native
 *
 * For example:
 *
 *     uncertainty(measure(q), provenance: source)
 *
 * may involve a measurement effect because `measure(q)` resolves to a quantum
 * measurement.
 *
 * The grammar must not guess this.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Semantic analysis may require capabilities such as:
 *
 *     probabilistic.compute
 *     uncertainty
 *     statistical.compute
 *     quantum.measurement
 *     model.inference
 *     knowledge.read
 *     provenance.record
 *
 * Capability resolution occurs downstream.
 *
 * The grammar never discovers whether a target provides a capability.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * An uncertainty expression can semantically require:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     accelerator resources
 *     quantum resources
 *     model resources
 *
 * Resource requirements belong to:
 *
 *     grammar/resources/
 *
 * They are NOT encoded here.
 *
 * There is deliberately no:
 *
 *     MAX_OUTCOMES
 *     MAX_DISTRIBUTION_SIZE
 *     MAX_SAMPLES
 *     MAX_PROBABILITY_BITS
 *     MAX_CONFIDENCE_BITS
 *     MAX_RANDOM_VARIABLES
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_MEMORY
 *
 * or equivalent constant.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The semantic/type layer determines the type of:
 *
 *     value
 *     probability
 *     distribution
 *     confidence
 *     belief
 *     likelihood
 *     evidence
 *     provenance
 *
 * Conceptually:
 *
 *     uncertain(T)
 *
 * may correspond to a semantic type such as:
 *
 *     Uncertain<T>
 *
 * but this grammar does not define the Rust type or type constructor.
 *
 * The exact type representation belongs to:
 *
 *     grammar/types/
 *     frontend semantic type system
 *
 * The language must remain capable of representing uncertainty over:
 *
 *     scalar values
 *     records
 *     tuples
 *     collections
 *     tensors
 *     functions
 *     symbolic values
 *     quantum-derived values
 *     distributed values
 *     domain-defined values
 *
 * subject to semantic type rules.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Uncertainty expressions may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * For example:
 *
 *     requires probability_value <= threshold
 *
 * or:
 *
 *     ensures confidence(result) >= required_confidence
 *
 * These are semantic relationships.
 *
 * This grammar does not duplicate contract grammar.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may constrain:
 *
 *     allowed uncertainty sources
 *     allowed models
 *     allowed evidence
 *     randomness
 *     provenance
 *     external services
 *     privacy
 *     reproducibility
 *     execution strategies.
 *
 * A policy may reject an otherwise syntactically valid uncertainty operation.
 *
 * Syntax validity does not imply authorization.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Uncertainty metadata may preserve provenance such as:
 *
 *     source
 *     evidence
 *     derivation
 *     model
 *     transformation
 *     verification
 *     version
 *
 * This file only preserves source syntax.
 *
 * Provenance semantics belong to the canonical provenance system.
 *
 * No provider-specific provenance format is encoded here.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Uncertainty expressions may be consumed by:
 *
 *     infer
 *     deduce
 *     reason
 *
 * and may themselves contain:
 *
 *     reasoning results
 *
 * Example:
 *
 *     uncertain(infer hypothesis, confidence: c)
 *
 * The reasoning grammar remains the owner of:
 *
 *     infer
 *     deduce
 *     reason
 *
 * This file must not duplicate those constructs.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Uncertainty may wrap knowledge-derived values:
 *
 *     uncertain(knowledge_value, confidence: c)
 *
 * or:
 *
 *     uncertain(query_result, provenance: source)
 *
 * Knowledge syntax remains owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * or its canonical future knowledge composition boundary.
 *
 * This file must not duplicate:
 *
 *     assert
 *     retract
 *     query
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning may produce uncertain values.
 *
 * Examples:
 *
 *     uncertain(model.predict(input), confidence: confidence)
 *
 *     uncertainty(prediction, distribution: model_distribution)
 *
 * Learning semantics remain owned by the AI/learning subsystem.
 *
 * This file does not enumerate:
 *
 *     neural networks
 *     classifiers
 *     regressors
 *     reinforcement-learning algorithms
 *     transfer-learning algorithms
 *     model vendors.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Uncertainty is particularly important for quantum/classical computation.
 *
 * Examples:
 *
 *     uncertain(measure(q))
 *
 *     uncertain(
 *         measure(q),
 *         probability: p,
 *         provenance: measurement_source
 *     )
 *
 * The grammar does not:
 *
 *     - allocate qubits;
 *     - select physical qubits;
 *     - select a QPU;
 *     - select a simulator;
 *     - select a gate set;
 *     - select topology;
 *     - perform measurement;
 *     - implement probability amplitudes;
 *     - implement QEC.
 *
 * Quantum semantic lowering remains:
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
 *     resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 * No uncertainty-specific quantum IR is introduced.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Uncertainty is a natural bridge between:
 *
 *     classical
 *     AI
 *     quantum
 *     simulation
 *     distributed
 *     hardware
 *
 * Example:
 *
 *     classical computation
 *          |
 *          v
 *     uncertain value
 *          |
 *          v
 *     reasoning
 *          |
 *          v
 *     quantum/classical control
 *
 * The grammar remains domain-neutral.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Uncertainty may semantically describe:
 *
 *     measurement values
 *     sensor values
 *     simulation results
 *     timing uncertainty
 *     hardware reliability information
 *     probabilistic hardware behavior
 *
 * The grammar does not encode:
 *
 *     bus width
 *     register width
 *     clock count
 *     device count
 *     topology
 *     physical timing resolution.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * An uncertain value may originate from:
 *
 *     actor
 *     task
 *     node
 *     service
 *     stream
 *     remote observation.
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/distributed/
 *     grammar/concurrency/
 *     grammar/networking/
 *
 * No node-count limit is introduced.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Foreign data or external systems may provide uncertainty metadata.
 *
 * FFI/ABI, SQL, JSON, XML and other external representations remain owned by
 * the interoperability/dialect subsystems.
 *
 * The uncertainty grammar does not duplicate those grammars.
 *
 * ============================================================================
 * METAPROGRAMMING
 * ============================================================================
 *
 * Reflection or compile-time generation may inspect uncertainty expressions
 * only through the normal AST/metaprogramming boundaries.
 *
 * This grammar performs no compile-time execution.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve, at minimum:
 *
 *     - introducer kind;
 *     - primary value expression;
 *     - ordered additional positional arguments;
 *     - ordered named arguments;
 *     - named-field spelling/token identity;
 *     - separator kind (`:` versus `=`);
 *     - source span;
 *     - source ordering.
 *
 * Conceptual semantic-neutral representation:
 *
 *     UncertaintyExpression {
 *         kind,
 *         value,
 *         arguments,
 *         span
 *     }
 *
 * where:
 *
 *     kind = Uncertain | Uncertainty
 *
 * The AST must NOT contain:
 *
 *     QuantumUncertainty
 *     GPUUncertainty
 *     NeuralUncertainty
 *     HardwareUncertainty
 *
 * as universal syntax categories.
 *
 * The exact Rust structures belong to the frontend AST owner.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the wrapped value is a valid uncertain value;
 *     - the semantic uncertainty type;
 *     - whether metadata is legal;
 *     - probability validity;
 *     - confidence validity;
 *     - distribution compatibility;
 *     - evidence compatibility;
 *     - provenance compatibility;
 *     - duplicate metadata;
 *     - conflicting metadata;
 *     - required capabilities;
 *     - required resources;
 *     - effects;
 *     - policies;
 *     - determinism/reproducibility requirements.
 *
 * The parser performs none of these validations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The canonical lowering path is:
 *
 *     uncertaintyExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic uncertainty value
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *       +--+--------------------+
 *       |                       |
 *       v                       v
 *   classical              quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *             optimization
 *                   |
 *              lowering
 *                   |
 *            target realization
 *
 * The uncertainty grammar MUST NOT introduce:
 *
 *     uncertaintyIR
 *     probabilityIR
 *     quantumUncertaintyIR
 *
 * or any second domain-specific IR.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * Important:
 *
 *     syntax != effect
 *     syntax != capability
 *     syntax != resource
 *     syntax != policy
 *
 * For example:
 *
 *     uncertain(x)
 *
 * does not itself prove that:
 *
 *     randomness
 *
 * is required.
 *
 * The value `x` and its semantic construction determine the actual effects.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * The parser must depend only on:
 *
 *     source token sequence
 *     grammar version
 *     language configuration
 *
 * It must NOT depend on:
 *
 *     time
 *     randomness
 *     hardware availability
 *     filesystem state
 *     network state
 *     target selection
 *     runtime state
 *     scheduler state
 *     installed statistical libraries.
 *
 * Runtime uncertainty may be nondeterministic.
 *
 * That is a semantic/runtime property, not a parser property.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * This grammar must never:
 *
 *     - sample a distribution;
 *     - invoke a random generator;
 *     - contact a service;
 *     - query a database;
 *     - invoke a model;
 *     - inspect hardware;
 *     - invoke a QPU;
 *     - invoke a simulator;
 *     - read credentials;
 *     - execute foreign code.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no universal finite limits on:
 *
 *     uncertainty expressions
 *     metadata count
 *     positional arguments
 *     nested expressions
 *     nested uncertainty
 *     distribution structure
 *     tensor rank
 *     model size
 *     knowledge size
 *     evidence count
 *     provenance depth
 *     quantum resources
 *     CPU resources
 *     GPU resources
 *     FPGA resources
 *     node count
 *     memory capacity
 *
 * Repetition uses:
 *
 *     *
 *     +
 *
 * and not finite machine-dependent counts.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite ceiling.
 *
 * Actual parser/compiler/runtime limits remain implementation resource limits.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_PROBABILITY_BITS
 *     MAX_CONFIDENCE_BITS
 *     MAX_OUTCOMES
 *     MAX_DISTRIBUTION_SIZE
 *     MAX_SAMPLES
 *     MAX_RANDOM_VARIABLES
 *     MAX_UNCERTAINTY_DEPTH
 *     MAX_EVIDENCE
 *     MAX_PROVENANCE
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * None of these concepts is a grammar-level limit.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces a stable expression-level uncertainty constructor.
 *
 * Existing ordinary identifiers named:
 *
 *     uncertain
 *     uncertainty
 *
 * become reserved because the canonical lexer already reserves those spellings.
 *
 * Compatibility handling therefore belongs to:
 *
 *     grammar/compatibility/
 *
 * A compatibility layer must not create alternate lexer tokens.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     uncertain(value)
 *
 *     uncertainty(value)
 *
 *     uncertain(value, confidence: c)
 *
 *     uncertain(value, confidence = c)
 *
 *     uncertain(value, probability: p)
 *
 *     uncertain(value, distribution: d)
 *
 *     uncertain(value, belief: b)
 *
 *     uncertain(value, likelihood: l)
 *
 *     uncertain(value, evidence: e)
 *
 *     uncertain(value, provenance: p)
 *
 *     uncertain(value, source: s)
 *
 *     uncertainty(
 *         value,
 *         probability: p,
 *         confidence: c,
 *         evidence: e,
 *         provenance: source
 *     )
 *
 * EXTENSION:
 *
 *     uncertainty(value, calibration: calibration_data)
 *
 *     uncertainty(value, interval: interval_value)
 *
 *     uncertainty(value, custom_metadata: metadata)
 *
 * NESTED:
 *
 *     uncertain(uncertainty(value))
 *
 *     uncertain(model.predict(input), confidence: model.confidence(input))
 *
 *     uncertain(measure(q), probability: p)
 *
 *     uncertainty(reasoning_result, evidence: knowledge_result)
 *
 * CROSS-DOMAIN:
 *
 *     uncertain(tensor[index])
 *
 *     uncertain(distributed_result)
 *
 *     uncertain(simulation_result)
 *
 *     uncertain(hardware_observation)
 *
 *     uncertain(measurement_result)
 *
 * NEGATIVE:
 *
 *     uncertain()
 *
 *     uncertainty()
 *
 *     uncertain(, value)
 *
 *     uncertain(value,)
 *
 *     uncertain(value, confidence:)
 *
 *     uncertain(value, : confidence)
 *
 *     uncertain(value, confidence)
 *
 *     uncertainty(value, probability)
 *
 *     uncertainty(value, confidence: , evidence: e)
 *
 *     uncertainty(value, confidence: c confidence: d)
 *
 * The semantic test suite must additionally reject invalid probability,
 * confidence, distribution and metadata combinations.
 *
 * Those semantic failures are NOT parser failures.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     uncertainty(integer)
 *     uncertainty(decimal)
 *     uncertainty(string)
 *     uncertainty(tuple)
 *     uncertainty(array)
 *     uncertainty(tensor_expression)
 *     uncertainty(function_call)
 *     uncertainty(quantum_measurement)
 *     uncertainty(knowledge_result)
 *     uncertainty(reasoning_result)
 *     uncertainty(learning_result)
 *     uncertainty(distributed_result)
 *
 * Also test:
 *
 *     nested uncertainty expressions;
 *     arbitrarily many metadata arguments;
 *     arbitrarily deep expression nesting;
 *     Unicode identifiers in extension metadata;
 *     `:` and `=` named-argument compatibility.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The conformance suite should verify:
 *
 *     - no fixed metadata count;
 *     - no fixed expression size;
 *     - no fixed distribution size;
 *     - no fixed evidence count;
 *     - no fixed provenance depth;
 *     - no hardware-dependent parser behavior;
 *     - no probability-precision assumption in syntax;
 *     - no tensor-rank assumption in syntax.
 *
 * Test resources may be finite.
 *
 * The language-level grammar must remain open-ended.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust actions or predicates.
 *
 * Generated parser/compiler integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * This grammar does not require `unsafe`.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/punctuation.g4
 *     grammar/lexer/operators.g4
 *
 * Expression composition:
 *
 *     grammar/expressions/expressions.g4
 *
 * AST:
 *
 *     existing domain-neutral frontend AST
 *
 * Semantic owners:
 *
 *     grammar/types/
 *     grammar/effects/
 *     grammar/resources/
 *     grammar/validation/
 *     grammar/policies/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/quantum/
 *
 * Provenance:
 *
 *     canonical provenance subsystem
 *
 * IR:
 *
 *     canonical semantic model
 *     quantum::ir for quantum lowering
 *
 * Tests:
 *
 *     grammar/tests/expressions/
 *     grammar/tests/semantic/
 *     grammar/tests/ai/
 *     grammar/tests/quantum/
 *     grammar/tests/hybrid/
 *     grammar/tests/scalability/
 *
 * Specification:
 *
 *     grammar/spec/uncertainty.md
 *
 * ============================================================================
 * COMPOSITION REQUIREMENT
 * ============================================================================
 *
 * The expression composition owner must add:
 *
 *     | uncertaintyExpression
 *
 * to `primaryExpression`.
 *
 * It must NOT:
 *
 *     - copy this grammar's rules;
 *     - redefine uncertaintyExpression;
 *     - create a second uncertainty syntax;
 *     - create an AI-specific uncertainty syntax;
 *     - create a quantum-specific uncertainty syntax.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS DONE when:
 *
 * [x] It uses canonical lexer token names.
 *
 * [x] It defines one uncertainty expression boundary.
 *
 * [x] It supports uncertain(value).
 *
 * [x] It supports uncertainty(value).
 *
 * [x] It supports standard uncertainty metadata.
 *
 * [x] It supports extensible identifier-based metadata.
 *
 * [x] It supports `:` and `=` named metadata syntax.
 *
 * [x] It preserves ordered arguments.
 *
 * [x] It imposes no probability representation.
 *
 * [x] It imposes no fixed precision.
 *
 * [x] It imposes no distribution catalog.
 *
 * [x] It imposes no hardware limits.
 *
 * [x] It imposes no quantum limits.
 *
 * [x] It introduces no IR.
 *
 * [x] It introduces no runtime behavior.
 *
 * [x] It introduces no unsafe Rust requirement.
 *
 * [x] It documents AST integration.
 *
 * [x] It documents semantic integration.
 *
 * [x] It documents effect integration.
 *
 * [x] It documents capability integration.
 *
 * [x] It documents resource integration.
 *
 * [x] It documents policy integration.
 *
 * [x] It documents provenance integration.
 *
 * [x] It documents quantum::ir integration.
 *
 * [x] It documents HDL/hardware integration.
 *
 * [x] It documents distributed integration.
 *
 * [x] It documents interoperability.
 *
 * [x] It documents deterministic parsing.
 *
 * [x] It documents security boundaries.
 *
 * [x] It documents positive/negative/boundary/scalability tests.
 *
 * Repository-level completion additionally requires:
 *
 * [ ] `uncertaintyExpression` is integrated exactly once into
 *     `primaryExpression`.
 *
 * [ ] The frontend AST has one corresponding domain-neutral representation.
 *
 * [ ] Semantic analysis has one corresponding uncertainty semantic model.
 *
 * [ ] Type checking handles the resulting uncertainty type.
 *
 * [ ] Effect analysis handles resolved uncertainty-producing operations.
 *
 * [ ] Capability analysis handles resolved uncertainty capabilities.
 *
 * [ ] Resource analysis handles resolved resource requirements.
 *
 * [ ] Policy analysis handles applicable policies.
 *
 * [ ] Provenance preserves uncertainty source/derivation metadata.
 *
 * [ ] Semantic lowering reaches the canonical semantic representation.
 *
 * [ ] Quantum-derived uncertainty reaches `quantum::ir` without a second
 *     quantum uncertainty IR.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar UncertaintyExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * This is the only public uncertainty-expression rule.
 */
uncertaintyExpression
    : uncertainValueExpression
    | uncertaintyValueExpression
    ;


/*
 * ============================================================================
 * UNCERTAIN FORM
 * ============================================================================
 *
 *     uncertain(value)
 *
 *     uncertain(value, confidence: c)
 *
 *     uncertain(value, probability: p)
 */
uncertainValueExpression
    : UNCERTAIN
      LPAREN
      uncertaintyValue
      (
          COMMA
          uncertaintyArgumentList
      )?
      RPAREN
    ;


/*
 * ============================================================================
 * UNCERTAINTY FORM
 * ============================================================================
 *
 *     uncertainty(value)
 *
 *     uncertainty(value, distribution: d)
 */
uncertaintyValueExpression
    : UNCERTAINTY
      LPAREN
      uncertaintyValue
      (
          COMMA
          uncertaintyArgumentList
      )?
      RPAREN
    ;


/*
 * ============================================================================
 * VALUE BOUNDARY
 * ============================================================================
 *
 * `expression` is supplied by the canonical expression composition layer.
 *
 * This rule deliberately does not reproduce the expression hierarchy.
 */
uncertaintyValue
    : expression
    ;


/*
 * ============================================================================
 * ARGUMENT LIST
 * ============================================================================
 *
 * The list is open-ended.
 *
 * No fixed metadata count is encoded.
 */
uncertaintyArgumentList
    : uncertaintyArgument
      (
          COMMA
          uncertaintyArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * ARGUMENT
 * ============================================================================
 *
 * Named metadata is preferred for standardized uncertainty properties.
 *
 * Positional arguments remain syntactically available for extensibility.
 */
uncertaintyArgument
    : uncertaintyNamedArgument
    | uncertaintyPositionalArgument
    ;


/*
 * ============================================================================
 * NAMED ARGUMENT
 * ============================================================================
 *
 * Examples:
 *
 *     confidence: c
 *     confidence = c
 *     probability: p
 *     evidence: e
 *     custom_field: value
 */
uncertaintyNamedArgument
    : uncertaintyFieldName
      (
          COLON
        | ASSIGN
      )
      expression
    ;


/*
 * ============================================================================
 * POSITIONAL ARGUMENT
 * ============================================================================
 *
 * The semantic layer decides which positional metadata forms are meaningful.
 */
uncertaintyPositionalArgument
    : expression
    ;


/*
 * ============================================================================
 * FIELD NAME
 * ============================================================================
 *
 * Standard uncertainty vocabulary uses canonical lexer tokens.
 *
 * IDENTIFIER provides open-world extension without adding new global keywords.
 */
uncertaintyFieldName
    : PROBABILITY
    | DISTRIBUTION
    | CONFIDENCE
    | BELIEF
    | LIKELIHOOD
    | EVIDENCE
    | PROVENANCE
    | SOURCE
    | IDENTIFIER
    ;