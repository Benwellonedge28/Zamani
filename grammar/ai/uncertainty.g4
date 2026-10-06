/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/uncertainty.g4
 *
 * Grammar:
 *     AIUncertainty
 *
 * Status:
 *     CANONICAL AI-DOMAIN UNCERTAINTY COMPOSITION GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97 or later
 *     Rust 2021 edition
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the AI-domain composition boundary for uncertainty-related
 * computation.
 *
 * IMPORTANT:
 *
 *     This file does NOT own the universal uncertainty expression syntax.
 *
 * The canonical source-level uncertainty syntax is owned by:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * whose grammar identity is:
 *
 *     UncertaintyExpressions
 *
 * This file exists so that the AI domain can explicitly consume uncertainty
 * without creating:
 *
 *     - a second uncertainty language;
 *     - a second uncertainty expression hierarchy;
 *     - a second probability grammar;
 *     - a second probabilistic type system;
 *     - a second evidence grammar;
 *     - a second provenance grammar;
 *     - a second AI-specific IR.
 *
 * The architectural direction is:
 *
 *     AI
 *      |
 *      v
 *     AIUncertainty
 *      |
 *      v
 *     UncertaintyExpressions
 *      |
 *      v
 *     domain-neutral AST
 *      |
 *      v
 *     structural validation
 *      |
 *      +----------------------+----------------------+
 *      |                      |                      |
 *      v                      v                      v
 *    types                 semantics              provenance
 *      |                      |                      |
 *      +----------------------+----------------------+
 *                             |
 *                             v
 *                    canonical semantic model
 *                             |
 *              +--------------+---------------+
 *              |              |               |
 *              v              v               v
 *          classical      quantum::ir      other domains
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Uncertainty is a UNIVERSAL COMPUTATIONAL CONCEPT.
 *
 * It is therefore not inherently restricted to AI.
 *
 * It may occur in:
 *
 *     classical computation
 *     numerical computation
 *     scientific computation
 *     statistical computation
 *     machine learning
 *     reasoning
 *     knowledge systems
 *     probabilistic computation
 *     quantum computation
 *     quantum measurement
 *     hardware observation
 *     sensor data
 *     distributed computation
 *     networking
 *     simulation
 *     reliability analysis
 *     calibration
 *     control systems
 *     optimization
 *     verification
 *     security analysis
 *     future computational domains
 *
 * Consequently:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * owns the source-level construct.
 *
 * This file only establishes its AI-domain integration boundary.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The repository has exactly one canonical source-level owner for the
 * uncertainty expression:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * This file MUST NOT redefine:
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
 * Those rules belong to:
 *
 *     UncertaintyExpressions
 *
 * Any future change to universal uncertainty syntax must therefore be made in
 * the canonical expression grammar rather than copied into this AI adapter.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - AI-domain composition of universal uncertainty expressions;
 *     - the public AI uncertainty parser boundary;
 *     - AI-to-universal uncertainty grammar integration;
 *     - explicit AI grammar dependency on the canonical uncertainty grammar;
 *     - prevention of duplicate AI uncertainty syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - keywords;
 *     - punctuation;
 *     - operators;
 *     - literals;
 *     - identifiers;
 *     - qualified names;
 *     - expression precedence;
 *     - function calls;
 *     - generic expression syntax;
 *     - uncertainty expression syntax;
 *     - probability representation;
 *     - distribution representation;
 *     - statistical algorithms;
 *     - probabilistic algorithms;
 *     - sampling algorithms;
 *     - random-number generation;
 *     - confidence calculation;
 *     - belief calculation;
 *     - likelihood calculation;
 *     - evidence storage;
 *     - provenance implementation;
 *     - AI model semantics;
 *     - learning semantics;
 *     - reasoning semantics;
 *     - inference semantics;
 *     - type semantics;
 *     - effect semantics;
 *     - capability resolution;
 *     - resource allocation;
 *     - policy enforcement;
 *     - security enforcement;
 *     - execution;
 *     - scheduling;
 *     - placement;
 *     - hardware discovery;
 *     - target selection;
 *     - classical IR;
 *     - AI-specific IR;
 *     - quantum IR;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Canonical grammar:
 *
 *     UncertaintyExpressions
 *
 * Canonical public rule:
 *
 *     uncertaintyExpression
 *
 * This file must NOT import:
 *
 *     Expressions
 *
 * merely to obtain uncertainty syntax.
 *
 * The complete expression composition grammar must remain above this leaf
 * boundary.
 *
 * Dependency direction:
 *
 *     Expressions
 *          |
 *          +----> UncertaintyExpressions
 *                         ^
 *                         |
 *                     AIUncertainty
 *
 * AIUncertainty therefore depends on the canonical uncertainty leaf but does
 * not own or replace the expression composition root.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PUBLIC RULE:
 *
 *     aiUncertaintyConstruct
 *
 * The public rule represents:
 *
 *     one AI-domain use of a canonical uncertainty expression.
 *
 * SUPPORTING RULES:
 *
 *     none
 *
 * This is intentional.
 *
 * A composition façade should expose the smallest possible stable surface.
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/ai/ai.g4
 *
 * Expected integration:
 *
 *     import
 *         ...
 *         AIUncertainty
 *     ;
 *
 * and:
 *
 *     aiConstruct
 *         : ...
 *         | aiUncertaintyConstruct
 *         | ...
 *         ;
 *
 * The exact ordering should follow the existing AI composition ordering.
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
 * when AI is already routed through:
 *
 *     AI
 *
 * The intended path is:
 *
 *     ZamaniParser
 *          |
 *          v
 *         AI
 *          |
 *          v
 *     AIUncertainty
 *          |
 *          v
 *     UncertaintyExpressions
 *
 * This keeps the root parser independent of individual AI leaf components.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * It introduces NO tokens.
 *
 * It introduces NO keywords.
 *
 * It introduces NO punctuation.
 *
 * It introduces NO operator definitions.
 *
 * Existing canonical lexical vocabulary remains owned by the repository's
 * lexical layer.
 *
 * Relevant existing uncertainty vocabulary includes concepts represented by
 * the canonical lexical layer such as:
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
 * This file does not assume that every future uncertainty concept requires a
 * keyword.
 *
 * ============================================================================
 * NO KEYWORD EXPLOSION
 * ============================================================================
 *
 * Future uncertainty concepts must normally remain identifiers or semantic
 * library constructs unless there is a demonstrated language-wide reason for
 * introducing a reserved lexical token.
 *
 * Examples that MUST NOT automatically become new core keywords:
 *
 *     interval
 *     variance
 *     covariance
 *     entropy
 *     prior
 *     posterior
 *     calibration
 *     reliability
 *     credibility
 *     dispersion
 *     evidence_strength
 *     uncertainty_budget
 *     confidence_interval
 *
 * Such concepts can be represented by the existing open uncertainty metadata
 * mechanism or by library/domain constructs.
 *
 * This keeps the language extensible without requiring universal grammar
 * changes whenever mathematics or statistical practice gains a new concept.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * The complete expression system owns:
 *
 *     precedence
 *     associativity
 *     calls
 *     indexing
 *     member access
 *     unary operations
 *     binary operations
 *     conditional expressions
 *     literals
 *     identifiers
 *     lambdas
 *     patterns
 *     other expression forms
 *
 * This file does not reproduce any of those mechanisms.
 *
 * The canonical uncertainty expression already accepts a value boundary that
 * can semantically resolve to values such as:
 *
 *     scalar
 *     record
 *     tuple
 *     collection
 *     tensor
 *     model result
 *     knowledge result
 *     reasoning result
 *     learning result
 *     measurement result
 *     quantum-derived result
 *     distributed result
 *     hardware observation
 *     simulation result
 *     domain-defined value
 *
 * The AI layer therefore consumes uncertainty without constraining what kind
 * of computational value may be uncertain.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file defines NO types.
 *
 * In particular, it must not define:
 *
 *     AIUncertainType
 *     AIProbabilityType
 *     AIDistributionType
 *     AIConfidenceType
 *     AIBeliefType
 *
 * or any equivalent competing type family.
 *
 * Universal type semantics remain owned by:
 *
 *     grammar/types/
 *
 * The semantic type system may represent uncertainty conceptually using
 * constructs such as:
 *
 *     Uncertain<T>
 *     Probability<T>
 *     Distribution<T>
 *     Confidence<T>
 *     Belief<T>
 *
 * when such semantic types are standardized.
 *
 * Those names are semantic/type-system concepts, not parser-level types owned
 * by this file.
 *
 * ============================================================================
 * OPEN-WORLD TYPE PRINCIPLE
 * ============================================================================
 *
 * The uncertain value may have any semantically supported type.
 *
 * The grammar does not impose a closed list of:
 *
 *     scalar types
 *     tensor types
 *     model types
 *     quantum types
 *     hardware types
 *     data types
 *     distributed types
 *
 * A future type must therefore be able to participate in uncertainty without
 * requiring this file to be modified merely because the type was introduced.
 *
 * ============================================================================
 * AI SEMANTIC INTEGRATION
 * ============================================================================
 *
 * AI semantic analysis may consume uncertainty in:
 *
 *     inference
 *     reasoning
 *     deduction
 *     induction
 *     abduction
 *     learning
 *     adaptation
 *     knowledge
 *     planning
 *     decisions
 *     agents
 *     model evaluation
 *     model deployment
 *     feedback
 *     probabilistic computation
 *
 * However, those systems remain separate semantic owners.
 *
 * This grammar does not define how uncertainty is consumed.
 *
 * Example semantic flow:
 *
 *     inference result
 *          |
 *          v
 *     uncertaintyExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic uncertainty value
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Uncertainty may participate in:
 *
 *     infer
 *     deduce
 *     reason
 *
 * The reasoning constructs remain owned by their canonical grammar files.
 *
 * This file MUST NOT define:
 *
 *     infer
 *     deduce
 *     reason
 *
 * again.
 *
 * A reasoning result can simply become the value consumed by the canonical
 * uncertainty expression.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Knowledge systems may produce uncertain values.
 *
 * Conceptually:
 *
 *     query result
 *          |
 *          v
 *     uncertain value
 *
 * or:
 *
 *     knowledge assertion
 *          |
 *          v
 *     confidence / belief / likelihood
 *
 * Knowledge syntax remains owned by the canonical knowledge subsystem.
 *
 * This file MUST NOT redefine:
 *
 *     assert
 *     retract
 *     query
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning operations may produce:
 *
 *     uncertain predictions
 *     confidence values
 *     probability distributions
 *     belief states
 *     likelihood values
 *     evidence-derived results
 *
 * Learning syntax remains owned by the canonical learning grammar.
 *
 * This adapter only permits the canonical uncertainty expression to be
 * consumed by the AI composition boundary.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Adaptive execution may use uncertainty as:
 *
 *     observation
 *     trigger
 *     evidence
 *     confidence
 *     decision input
 *     evaluation result
 *
 * Adaptation syntax remains owned by:
 *
 *     grammar/statements/adapt.g4
 *
 * and the AI adaptation composition boundary.
 *
 * This file must not define adaptation syntax.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence may accompany an uncertain value.
 *
 * The canonical expression grammar is responsible for preserving the source
 * structure.
 *
 * Semantic analysis determines:
 *
 *     evidence identity
 *     evidence validity
 *     evidence type
 *     evidence strength
 *     evidence provenance
 *     trust
 *     authorization
 *     compatibility
 *
 * This file does not implement evidence semantics.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Uncertainty may carry or derive provenance.
 *
 * The repository-wide provenance system remains authoritative.
 *
 * There must be no:
 *
 *     AIUncertaintyProvenance
 *
 * competing with the universal provenance representation.
 *
 * Provenance may describe:
 *
 *     source
 *     derivation
 *     transformation
 *     evidence
 *     model
 *     computation
 *     measurement
 *     verification
 *     version
 *     decision
 *
 * This file does not define provenance syntax.
 *
 * ============================================================================
 * CONFIDENCE / BELIEF / LIKELIHOOD
 * ============================================================================
 *
 * These are semantic concepts rather than separate AI grammar languages.
 *
 * The canonical uncertainty expression may carry metadata representing:
 *
 *     confidence
 *     belief
 *     likelihood
 *
 * Their interpretation is determined by semantic analysis.
 *
 * This grammar does not assume:
 *
 *     a particular mathematical interpretation;
 *     a particular scale;
 *     a particular numeric representation;
 *     a particular statistical framework.
 *
 * ============================================================================
 * PROBABILITY CONTRACT
 * ============================================================================
 *
 * This file does not define probability mathematics.
 *
 * It does not require:
 *
 *     a particular probability domain;
 *     a particular numerical encoding;
 *     a particular precision;
 *     a particular arithmetic implementation;
 *     a particular floating-point format;
 *     a particular fixed-point format;
 *     a particular arbitrary-precision implementation;
 *     a particular symbolic representation.
 *
 * Semantic validation determines whether a resolved probability value is
 * valid for the operation in which it participates.
 *
 * For example, domain semantics may establish that a resolved probability must
 * satisfy a mathematical invariant. That validation must not be moved into
 * parser syntax.
 *
 * ============================================================================
 * DISTRIBUTION CONTRACT
 * ============================================================================
 *
 * No distribution family is part of this AI grammar.
 *
 * The grammar must not enumerate:
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
 * as a closed universal grammar catalog.
 *
 * A distribution may instead be supplied by:
 *
 *     library construct
 *     function
 *     model
 *     provider
 *     dialect
 *     semantic value
 *     future implementation
 *
 * This makes the language open to new mathematical and computational
 * representations.
 *
 * ============================================================================
 * UNCERTAINTY REPRESENTATION CONTRACT
 * ============================================================================
 *
 * The source language must remain representation-independent.
 *
 * A semantic uncertainty value may eventually be implemented using:
 *
 *     exact arithmetic
 *     arbitrary precision
 *     interval arithmetic
 *     symbolic mathematics
 *     numerical representation
 *     probability distributions
 *     samples
 *     statistical summaries
 *     quantum amplitudes/probabilities
 *     domain-specific representations
 *     hybrid representations
 *
 * None of these implementation choices belong in this file.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This file declares NO effects.
 *
 * An uncertainty expression may semantically acquire effects from the value or
 * metadata it contains.
 *
 * Possible effects include:
 *
 *     randomness
 *     measurement
 *     learning
 *     adaptation
 *     IO
 *     network
 *     foreign
 *     native
 *     distributed
 *     simulation
 *
 * Example:
 *
 *     uncertain(measurement_result)
 *
 * may semantically involve a measurement effect.
 *
 * The grammar does not infer or encode that effect.
 *
 * The canonical effects subsystem remains authoritative.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * This file declares NO physical capabilities.
 *
 * Semantic analysis may determine that an uncertainty operation requires a
 * capability such as:
 *
 *     probabilistic.compute
 *     statistical.compute
 *     uncertainty
 *     model.inference
 *     knowledge.read
 *     provenance.record
 *     quantum.measurement
 *
 * Capability names remain open-world semantic identifiers.
 *
 * Capability resolution occurs downstream.
 *
 * The grammar never determines whether a target actually provides a
 * capability.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Uncertainty computation may consume arbitrary resources depending on its
 * semantic realization.
 *
 * Possible resource categories include:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     accelerator resources
 *     quantum resources
 *     distributed resources
 *     model resources
 *
 * Resource requirements belong to:
 *
 *     grammar/resources/
 *
 * This file does not allocate or select resources.
 *
 * ============================================================================
 * POCO-REAF SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO language-level capacity ceiling.
 *
 * It must not introduce limits for:
 *
 *     uncertainty values
 *     variables
 *     probability values
 *     distributions
 *     outcomes
 *     samples
 *     evidence
 *     provenance records
 *     model values
 *     tensor dimensions
 *     tensor rank
 *     agents
 *     workers
 *     nodes
 *     devices
 *     accelerators
 *     quantum resources
 *     memory
 *     network size
 *
 * It must not define constants equivalent to:
 *
 *     MAX_UNCERTAINTY_VALUES
 *     MAX_DISTRIBUTIONS
 *     MAX_OUTCOMES
 *     MAX_SAMPLES
 *     MAX_PROBABILITY_BITS
 *     MAX_CONFIDENCE_BITS
 *     MAX_RANDOM_VARIABLES
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * or any equivalent artificial ceiling.
 *
 * "Infinity" in the language architecture means that the grammar introduces no
 * artificial finite ceiling. Actual execution remains constrained only by
 * semantic validity, available resources, target capabilities, policies,
 * implementation limits, physical feasibility and the mathematical properties
 * of the requested computation.
 *
 * ============================================================================
 * DYNAMIC AND SYMBOLIC VALUES
 * ============================================================================
 *
 * Uncertainty metadata values may be:
 *
 *     compile-time known
 *     runtime known
 *     dynamically derived
 *     symbolic
 *     data-dependent
 *     model-dependent
 *     hardware-observed
 *     quantum-derived
 *     distributed
 *
 * This grammar does not require such values to be literal constants.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Uncertainty is particularly important for quantum computation.
 *
 * Quantum-derived uncertainty may arise from:
 *
 *     measurement
 *     probabilistic outcomes
 *     noise
 *     error models
 *     statistical estimation
 *     calibration
 *     simulation
 *     resilience analysis
 *
 * Nevertheless, this file does not define quantum syntax.
 *
 * The architectural boundary remains:
 *
 *     AI uncertainty
 *          |
 *          v
 *     domain-neutral semantic model
 *          |
 *          v
 *     quantum semantic operation
 *          |
 *          v
 *     quantum::ir
 *
 * This file MUST NOT define:
 *
 *     qubit
 *     gate
 *     physical qubit
 *     coupling map
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * Quantum source syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical computation may use uncertainty for:
 *
 *     numerical analysis
 *     statistics
 *     scientific computing
 *     probabilistic algorithms
 *     optimization
 *     simulation
 *     measurement
 *     estimation
 *
 * Classical realization remains downstream.
 *
 * No CPU architecture, register width, floating-point width, vector width or
 * memory capacity is represented here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL systems may produce uncertainty through:
 *
 *     measurement
 *     sensor input
 *     timing analysis
 *     reliability analysis
 *     simulation
 *     calibration
 *     physical observation
 *
 * This file does not define HDL constructs or physical hardware structures.
 *
 * HDL syntax remains owned by:
 *
 *     grammar/hdl/
 *
 * Hardware realization remains owned by:
 *
 *     grammar/hardware/
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed systems may represent uncertainty from:
 *
 *     incomplete observations
 *     distributed measurements
 *     probabilistic decisions
 *     consensus information
 *     network observations
 *     failure estimation
 *     model inference
 *
 * Distribution of computation remains a downstream execution concern.
 *
 * This file does not encode:
 *
 *     node identifiers
 *     node counts
 *     network topology
 *     cluster sizes
 *     worker counts
 *     device placement.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may constrain uncertainty-related computation.
 *
 * Examples include policies governing:
 *
 *     acceptable evidence
 *     allowed data sources
 *     provenance requirements
 *     reproducibility
 *     randomness
 *     external services
 *     privacy
 *     model use
 *     decision thresholds
 *     adaptation
 *     deployment
 *
 * Policy enforcement belongs downstream.
 *
 * Syntax validity does not imply authorization.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Uncertainty values may participate in universal contracts:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Examples of semantic relationships include:
 *
 *     requires confidence_value >= required_confidence;
 *
 *     ensures probability_value <= allowed_probability;
 *
 * The contract grammar remains authoritative for contract syntax.
 *
 * This file must not duplicate:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Uncertainty computation may involve sensitive data, external evidence,
 * model outputs or external services.
 *
 * Security enforcement therefore remains outside this grammar.
 *
 * Security analysis may consider:
 *
 *     capabilities
 *     effects
 *     policies
 *     provenance
 *     trust
 *     authorization
 *     sandboxing
 *     data access
 *
 * This grammar performs none of those operations.
 *
 * ============================================================================
 * SIMULATION CONTRACT
 * ============================================================================
 *
 * The same uncertainty syntax may be used when execution is performed by:
 *
 *     classical simulation
 *     quantum simulation
 *     hardware simulation
 *     distributed simulation
 *     AI simulation
 *     fault simulation
 *     performance simulation
 *
 * Simulation is an execution strategy.
 *
 * It does not create a separate uncertainty language.
 *
 * ============================================================================
 * DIALECT / VENDOR CONTRACT
 * ============================================================================
 *
 * This file remains vendor-neutral.
 *
 * It must not reserve syntax for:
 *
 *     specific AI frameworks
 *     specific probabilistic frameworks
 *     specific statistical libraries
 *     specific quantum providers
 *     specific GPUs
 *     specific accelerators
 *     specific cloud providers
 *     specific model families
 *
 * Vendor and framework integration belongs to:
 *
 *     grammar/dialects/
 *     grammar/interoperability/
 *     grammar/compile/
 *     grammar/execution/
 *     grammar/hardware/
 *
 * Such integrations may consume the canonical semantic uncertainty model.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file introduces NO AI-specific AST node.
 *
 * The parse tree generated by this adapter contains:
 *
 *     aiUncertaintyConstruct
 *
 * wrapping:
 *
 *     uncertaintyExpression
 *
 * AST construction must normalize the wrapper according to the frontend's
 * domain-neutral AST policy.
 *
 * The semantic AST must not permanently encode an artificial distinction such
 * as:
 *
 *     AIUncertaintyExpression
 *
 * when the underlying construct is the universal uncertainty expression.
 *
 * The original source span and source spelling must remain available for:
 *
 *     diagnostics
 *     source maps
 *     formatting
 *     compatibility
 *     provenance
 *     tooling
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis resolves:
 *
 *     value type
 *     uncertainty metadata
 *     metadata types
 *     duplicate metadata
 *     conflicting metadata
 *     mathematical constraints
 *     evidence
 *     provenance
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     authorization
 *
 * None of those decisions are performed by this parser grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * There must be no:
 *
 *     AIUncertaintyIR
 *     ProbabilityIR
 *     DistributionIR
 *
 * created merely because this grammar is in the AI directory.
 *
 * The semantic representation is lowered according to the resolved operation
 * and domain.
 *
 * Possible downstream paths include:
 *
 *     uncertainty
 *          |
 *          +--> classical semantic operation
 *          |
 *          +--> quantum semantic operation
 *          |         |
 *          |         v
 *          |     quantum::ir
 *          |
 *          +--> hardware semantic operation
 *          |
 *          +--> distributed semantic operation
 *          |
 *          +--> AI/model semantic operation
 *          |
 *          +--> other future domain
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE PIPELINE
 * ============================================================================
 *
 * The complete downstream pipeline is:
 *
 *     aiUncertaintyConstruct
 *              |
 *              v
 *     uncertaintyExpression
 *              |
 *              v
 *     domain-neutral AST
 *              |
 *              v
 *     structural validation
 *              |
 *              v
 *     semantic model
 *              |
 *       +------+-------+---------+---------+
 *       |              |         |         |
 *       v              v         v         v
 *     Types         Effects  Capabilities Resources
 *       |              |         |         |
 *       +--------------+---------+---------+
 *                      |
 *                      v
 *                  Contracts
 *                      |
 *                      v
 *                   Policies
 *                      |
 *                      v
 *                 Provenance
 *                      |
 *                      v
 *              canonical semantics
 *
 * This adapter must not bypass that pipeline.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * This file contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no executable code;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no resource discovery;
 *     - no random behavior;
 *     - no runtime execution.
 *
 * For a fixed:
 *
 *     source token stream
 *     lexer version
 *     parser grammar version
 *     enabled dialect configuration
 *
 * the parser structure must be deterministic.
 *
 * ============================================================================
 * SOURCE / RUNTIME SEPARATION
 * ============================================================================
 *
 * This grammar never:
 *
 *     loads a probability model;
 *     loads a dataset;
 *     loads a distribution;
 *     executes sampling;
 *     performs inference;
 *     performs learning;
 *     performs reasoning;
 *     performs measurement;
 *     accesses a QPU;
 *     accesses a GPU;
 *     accesses an FPGA;
 *     accesses an ASIC;
 *     allocates memory;
 *     allocates devices;
 *     starts workers;
 *     communicates over a network.
 *
 * All such behavior belongs downstream.
 *
 * ============================================================================
 * RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97 or later
 *     Rust 2021 edition
 *     safe Rust only
 *
 * No unsafe Rust is required or permitted by this grammar design.
 *
 * Parser generation and frontend integration must not require:
 *
 *     unsafe
 *     FFI from parser actions
 *     native execution from parser actions
 *     filesystem operations
 *     network operations
 *     hardware operations
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical universal uncertainty syntax remains owned by:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Therefore compatibility changes to uncertainty syntax must be made in that
 * canonical owner.
 *
 * This file should normally remain unchanged when:
 *
 *     a new probability representation is introduced;
 *     a new distribution implementation is introduced;
 *     a new AI framework is introduced;
 *     a new model family is introduced;
 *     a new quantum backend is introduced;
 *     a new hardware target is introduced;
 *     a new statistical algorithm is introduced.
 *
 * Such changes should be semantic, library, dialect or backend changes.
 *
 * ============================================================================
 * LEGACY / DUPLICATE-GRAMMAR POLICY
 * ============================================================================
 *
 * If another AI grammar attempts to define:
 *
 *     aiUncertaintyExpression
 *     aiProbabilityExpression
 *     aiDistributionExpression
 *     aiConfidenceExpression
 *     aiBeliefExpression
 *
 * as a competing syntax hierarchy, that grammar is NOT canonical.
 *
 * The canonical AI integration point is:
 *
 *     AIUncertainty.aiUncertaintyConstruct
 *
 * which delegates to:
 *
 *     UncertaintyExpressions.uncertaintyExpression
 *
 * ============================================================================
 * NEGATIVE ARCHITECTURAL REQUIREMENTS
 * ============================================================================
 *
 * This file must never:
 *
 *     - enumerate all probability algorithms;
 *     - enumerate all distribution families;
 *     - enumerate all statistical methods;
 *     - enumerate all ML frameworks;
 *     - enumerate all quantum backends;
 *     - enumerate all hardware devices;
 *     - enumerate all uncertainty metrics;
 *     - encode physical resource limits;
 *     - encode tensor rank limits;
 *     - encode sample-count limits;
 *     - encode outcome-count limits;
 *     - encode precision limits;
 *     - encode floating-point width;
 *     - encode register width;
 *     - encode memory capacity;
 *     - encode CPU count;
 *     - encode GPU count;
 *     - encode FPGA count;
 *     - encode QPU count;
 *     - encode node count;
 *     - encode network topology;
 *     - select physical devices;
 *     - perform runtime computation;
 *     - create an AI-specific IR.
 *
 * ============================================================================
 * POCO-REAF TARGET INDEPENDENCE
 * ============================================================================
 *
 * The same source-level uncertainty construct must remain syntactically valid
 * regardless of whether its eventual realization is:
 *
 *     tiny embedded computation
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     quantum simulator
 *     classical simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud system
 *     future computational target
 *
 * Target feasibility is determined downstream.
 *
 * Therefore:
 *
 *     source syntax
 *
 * describes computational intent while:
 *
 *     semantic analysis
 *     capability negotiation
 *     resource analysis
 *     policy evaluation
 *     compilation
 *     lowering
 *     scheduling
 *     routing
 *     resilience
 *     target realization
 *
 * determine how that intent is realized.
 *
 * ============================================================================
 * FILE-LOCAL COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 * [x] It is a parser grammar.
 *
 * [x] Its grammar identity is AIUncertainty.
 *
 * [x] It imports the canonical UncertaintyExpressions grammar.
 *
 * [x] It exposes exactly one public AI composition rule.
 *
 * [x] aiUncertaintyConstruct delegates to uncertaintyExpression.
 *
 * [x] It defines no duplicate uncertainty syntax.
 *
 * [x] It defines no lexer rules.
 *
 * [x] It defines no keywords.
 *
 * [x] It defines no tokens.
 *
 * [x] It defines no punctuation.
 *
 * [x] It defines no operators.
 *
 * [x] It defines no expression hierarchy.
 *
 * [x] It defines no types.
 *
 * [x] It defines no probability representation.
 *
 * [x] It defines no distribution catalog.
 *
 * [x] It defines no statistical algorithms.
 *
 * [x] It defines no inference algorithms.
 *
 * [x] It defines no learning algorithms.
 *
 * [x] It defines no reasoning syntax.
 *
 * [x] It defines no knowledge syntax.
 *
 * [x] It defines no provenance syntax.
 *
 * [x] It defines no policy syntax.
 *
 * [x] It defines no contract syntax.
 *
 * [x] It defines no effect syntax.
 *
 * [x] It defines no capability syntax.
 *
 * [x] It defines no resource syntax.
 *
 * [x] It defines no hardware syntax.
 *
 * [x] It defines no quantum syntax.
 *
 * [x] It defines no routing syntax.
 *
 * [x] It defines no scheduling syntax.
 *
 * [x] It defines no QEC syntax.
 *
 * [x] It defines no ZQN syntax.
 *
 * [x] It defines no HAL syntax.
 *
 * [x] It creates no AI-specific AST requirement.
 *
 * [x] It creates no AI-specific IR.
 *
 * [x] It introduces no capacity constants.
 *
 * [x] It introduces no target-specific assumptions.
 *
 * [x] It introduces no vendor-specific assumptions.
 *
 * [x] It contains no executable actions.
 *
 * [x] It contains no semantic predicates.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is deterministic at parse time.
 *
 * [x] It preserves the universal uncertainty ownership model.
 *
 * [x] It permits future uncertainty concepts without keyword expansion.
 *
 * [x] It is independent of machine scale.
 *
 * [x] It is compatible with the POCO-REAF architecture.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * The following repository integrations are intentionally external to this
 * file. They are listed here so this file has a complete integration contract
 * and does not need to be reopened merely because another subsystem is
 * subsequently implemented.
 *
 * ---------------------------------------------------------------------------
 * 1. AI COMPOSITION
 * ---------------------------------------------------------------------------
 *
 * File:
 *
 *     grammar/ai/ai.g4
 *
 * Required import:
 *
 *     AIUncertainty
 *
 * Required AI dispatch alternative:
 *
 *     aiUncertaintyConstruct
 *
 * Intended path:
 *
 *     AI.aiConstruct
 *          |
 *          v
 *     AIUncertainty.aiUncertaintyConstruct
 *          |
 *          v
 *     UncertaintyExpressions.uncertaintyExpression
 *
 * Do not import this file directly into the universal root grammar.
 *
 * ---------------------------------------------------------------------------
 * 2. UNIVERSAL EXPRESSION COMPOSITION
 * ---------------------------------------------------------------------------
 *
 * File:
 *
 *     grammar/expressions/expressions.g4
 *
 * Required condition:
 *
 *     uncertaintyExpression
 *
 * must be reachable from the canonical expression composition boundary.
 *
 * This file does not modify that ownership.
 *
 * The expected relationship is:
 *
 *     Expressions
 *          |
 *          v
 *     UncertaintyExpressions
 *
 * ---------------------------------------------------------------------------
 * 3. LEXER
 * ---------------------------------------------------------------------------
 *
 * Files:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/tokens.g4
 *
 * Required condition:
 *
 * uncertainty vocabulary is defined exactly once in the lexical architecture.
 *
 * This file adds nothing to the lexer.
 *
 * ---------------------------------------------------------------------------
 * 4. TYPES
 * ---------------------------------------------------------------------------
 *
 * Directory:
 *
 *     grammar/types/
 *
 * Required semantic integration:
 *
 * uncertainty values must use the universal type system.
 *
 * This file introduces no competing AI type family.
 *
 * ---------------------------------------------------------------------------
 * 5. EFFECTS
 * ---------------------------------------------------------------------------
 *
 * Directory:
 *
 *     grammar/effects/
 *
 * Required semantic integration:
 *
 * effects arising from uncertainty-producing computations must be inferred
 * from resolved semantic operations.
 *
 * This file does not duplicate effect syntax.
 *
 * ---------------------------------------------------------------------------
 * 6. RESOURCES
 * ---------------------------------------------------------------------------
 *
 * Directory:
 *
 *     grammar/resources/
 *
 * Required semantic integration:
 *
 * resource requirements associated with uncertainty computation are resolved
 * downstream.
 *
 * This file does not select or allocate resources.
 *
 * ---------------------------------------------------------------------------
 * 7. CAPABILITIES
 * ---------------------------------------------------------------------------
 *
 * Directory:
 *
 *     grammar/resources/
 *
 * Required semantic integration:
 *
 * capability requirements are resolved from semantic meaning.
 *
 * This file does not grant capabilities.
 *
 * ---------------------------------------------------------------------------
 * 8. CONTRACTS
 * ---------------------------------------------------------------------------
 *
 * Directory:
 *
 *     grammar/validation/
 *
 * Required semantic integration:
 *
 * uncertainty values may participate in universal contracts.
 *
 * Contract syntax remains owned by validation grammar.
 *
 * ---------------------------------------------------------------------------
 * 9. POLICIES
 * ---------------------------------------------------------------------------
 *
 * Policy subsystem:
 *
 *     grammar/policies/
 *
 * and existing policy integration points.
 *
 * Required semantic integration:
 *
 * policy analysis may constrain uncertainty sources, evidence, provenance,
 * randomness, external services and execution.
 *
 * This file performs no policy enforcement.
 *
 * ---------------------------------------------------------------------------
 * 10. PROVENANCE
 * ---------------------------------------------------------------------------
 *
 * Repository-wide provenance subsystem.
 *
 * Required semantic integration:
 *
 * uncertainty-derived values can preserve source and derivation information.
 *
 * No AI-specific provenance model is created.
 *
 * ---------------------------------------------------------------------------
 * 11. QUANTUM
 * ---------------------------------------------------------------------------
 *
 * Directory:
 *
 *     grammar/quantum/
 *
 * Required downstream boundary:
 *
 *     quantum::ir
 *
 * Uncertainty arising from quantum computation must enter quantum semantics
 * through the existing quantum pipeline.
 *
 * This file never introduces a second quantum IR.
 *
 * ---------------------------------------------------------------------------
 * 12. CLASSICAL
 * ---------------------------------------------------------------------------
 *
 * Directory:
 *
 *     grammar/classical/
 *
 * Required condition:
 *
 * classical uncertainty computation consumes the same universal semantic
 * uncertainty model.
 *
 * ---------------------------------------------------------------------------
 * 13. HDL / HARDWARE
 * ---------------------------------------------------------------------------
 *
 * Directories:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * Required condition:
 *
 * hardware-derived uncertainty remains semantic data and does not alter this
 * grammar.
 *
 * ---------------------------------------------------------------------------
 * 14. DISTRIBUTED
 * ---------------------------------------------------------------------------
 *
 * Directory:
 *
 *     grammar/distributed/
 *
 * Required condition:
 *
 * distributed uncertainty remains independent of node and topology limits.
 *
 * ---------------------------------------------------------------------------
 * 15. INTEROPERABILITY
 * ---------------------------------------------------------------------------
 *
 * Directories:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * Required condition:
 *
 * external probabilistic/statistical/AI systems consume or produce the
 * canonical semantic uncertainty representation through explicit adapters.
 *
 * ---------------------------------------------------------------------------
 * 16. AST
 * ---------------------------------------------------------------------------
 *
 * Frontend AST:
 *
 *     domain-neutral representation
 *
 * Required condition:
 *
 * `aiUncertaintyConstruct` must not force an AI-specific semantic AST node
 * where the underlying construct is universally applicable.
 *
 * ---------------------------------------------------------------------------
 * 17. SEMANTIC MODEL
 * ---------------------------------------------------------------------------
 *
 * Required condition:
 *
 * uncertainty is represented once in the canonical semantic model.
 *
 * Consumers may include:
 *
 *     AI
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     distributed
 *     simulation
 *     networking
 *     future domains
 *
 * ---------------------------------------------------------------------------
 * 18. IR
 * ---------------------------------------------------------------------------
 *
 * Required condition:
 *
 * no AI-specific uncertainty IR is introduced solely because this file is
 * located under grammar/ai/.
 *
 * Quantum computation ultimately reaches:
 *
 *     quantum::ir
 *
 * according to the repository's canonical quantum architecture.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This grammar must be tested at three distinct levels.
 *
 * ---------------------------------------------------------------------------
 * A. LEAF TESTS
 * ---------------------------------------------------------------------------
 *
 * The canonical uncertainty grammar must independently accept its supported
 * source forms.
 *
 * Examples include:
 *
 *     uncertain(value)
 *
 *     uncertainty(value)
 *
 *     uncertain(value, confidence: confidence_value)
 *
 *     uncertain(value, probability: probability_value)
 *
 *     uncertainty(value, distribution: distribution_value)
 *
 *     uncertainty(value, evidence: evidence_value)
 *
 *     uncertainty(value, provenance: provenance_value)
 *
 *     uncertainty(
 *         value,
 *         probability: probability_value,
 *         confidence: confidence_value,
 *         evidence: evidence_value,
 *         provenance: provenance_value
 *     )
 *
 * These are tests of:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * rather than duplicated tests of an AI-specific uncertainty language.
 *
 * ---------------------------------------------------------------------------
 * B. AI COMPOSITION TESTS
 * ---------------------------------------------------------------------------
 *
 * Through:
 *
 *     AI
 *
 * verify that the canonical uncertainty expression can participate in AI
 * contexts without requiring a second uncertainty syntax.
 *
 * Representative semantic combinations include:
 *
 *     inference + uncertainty
 *     reasoning + uncertainty
 *     knowledge + uncertainty
 *     learning + uncertainty
 *     adaptation + uncertainty
 *     agent + uncertainty
 *     model + uncertainty
 *     tensor result + uncertainty
 *
 * ---------------------------------------------------------------------------
 * C. CROSS-DOMAIN TESTS
 * ---------------------------------------------------------------------------
 *
 * Verify uncertainty integration with:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware observation
 *     distributed computation
 *     networking
 *     simulation
 *     security
 *     contracts
 *     policies
 *     provenance
 *     resources
 *     capabilities
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * AIUncertainty must NOT make invalid universal uncertainty syntax valid.
 *
 * Invalid examples include malformed calls such as:
 *
 *     uncertain()
 *
 *     uncertainty()
 *
 *     uncertain(value,)
 *
 *     uncertain(value, confidence:)
 *
 *     uncertain(value, : confidence)
 *
 *     uncertain(value, confidence,, probability)
 *
 *     uncertain(value, , confidence)
 *
 *     uncertainty(value, confidence: )
 *
 * provided those forms are rejected by the canonical uncertainty grammar.
 *
 * The AI adapter must not weaken validation.
 *
 * It also must not accept:
 *
 *     aiUncertainty
 *     aiProbability
 *     aiDistribution
 *
 * as special syntax merely because they are AI concepts.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must demonstrate that this adapter introduces no artificial ceiling
 * for:
 *
 *     number of uncertainty expressions
 *     expression nesting
 *     metadata fields
 *     evidence references
 *     provenance references
 *     model values
 *     tensor dimensions
 *     distributed participants
 *     quantum-derived values
 *
 * Scaling must be structural and implementation/resource dependent.
 *
 * The grammar must never be modified merely because a larger supported
 * machine becomes available.
 *
 * ============================================================================
 * PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same source-level uncertainty construct must remain independent of:
 *
 *     CPU model
 *     GPU model
 *     accelerator model
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     cluster size
 *     node count
 *     memory capacity
 *     register width
 *     network topology
 *     vendor
 *     cloud provider
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source text
 *     token stream
 *     lexer version
 *     parser grammar version
 *     dialect configuration
 *
 * the resulting parse structure must be deterministic.
 *
 * No runtime state may affect parsing.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the grammar-level scalability audit when:
 *
 *     no finite universal uncertainty capacity is encoded;
 *
 *     no finite universal sample capacity is encoded;
 *
 *     no finite universal outcome capacity is encoded;
 *
 *     no fixed numerical precision is encoded;
 *
 *     no fixed distribution family is encoded;
 *
 *     no fixed tensor rank is encoded;
 *
 *     no fixed machine capacity is encoded;
 *
 *     no target device is encoded;
 *
 *     no hardware topology is encoded;
 *
 *     no vendor is encoded;
 *
 *     no runtime resource is allocated;
 *
 *     no parser-time computation is performed.
 *
 * ============================================================================
 * MAINTAINABILITY CONTRACT
 * ============================================================================
 *
 * Completing this file must NOT require reopening it merely because another
 * subsystem adds:
 *
 *     a new probability algorithm;
 *     a new distribution;
 *     a new statistical representation;
 *     a new AI model;
 *     a new learning algorithm;
 *     a new reasoning algorithm;
 *     a new evidence source;
 *     a new provenance implementation;
 *     a new quantum backend;
 *     a new hardware target;
 *     a new accelerator;
 *     a new simulator;
 *     a new distributed runtime;
 *     a new cloud provider.
 *
 * Such changes consume the existing semantic uncertainty contract.
 *
 * This is the intended independent-file completion property.
 *
 * ============================================================================
 * CHANGE-OWNERSHIP MATRIX
 * ============================================================================
 *
 * Change:
 *     uncertainty source syntax
 *
 * Owner:
 *     grammar/expressions/uncertainty.g4
 *
 * Change:
 *     uncertainty lexical vocabulary
 *
 * Owner:
 *     grammar/lexer/
 *
 * Change:
 *     expression precedence
 *
 * Owner:
 *     grammar/expressions/
 *
 * Change:
 *     uncertainty type semantics
 *
 * Owner:
 *     grammar/types/
 *
 * Change:
 *     uncertainty effects
 *
 * Owner:
 *     grammar/effects/
 *
 * Change:
 *     uncertainty capabilities
 *
 * Owner:
 *     grammar/resources/
 *
 * Change:
 *     uncertainty resource requirements
 *
 * Owner:
 *     grammar/resources/
 *
 * Change:
 *     uncertainty contracts
 *
 * Owner:
 *     grammar/validation/
 *
 * Change:
 *     uncertainty policies
 *
 * Owner:
 *     grammar/policies/
 *
 * Change:
 *     uncertainty provenance
 *
 * Owner:
 *     universal provenance subsystem
 *
 * Change:
 *     AI composition
 *
 * Owner:
 *     grammar/ai/ai.g4
 *
 * Change:
 *     quantum realization
 *
 * Owner:
 *     quantum semantic pipeline
 *
 * Change:
 *     quantum IR
 *
 * Owner:
 *     quantum::ir
 *
 * Change:
 *     target realization
 *
 * Owner:
 *     compiler/backend/runtime layers
 *
 * This file should not absorb responsibilities from those owners.
 *
 * ============================================================================
 * FINAL INTEGRATION INVARIANT
 * ============================================================================
 *
 * The architectural invariant is:
 *
 *     universal uncertainty syntax
 *                |
 *                v
 *     AI composition boundary
 *                |
 *                v
 *     universal semantic uncertainty
 *                |
 *       +--------+---------+---------+
 *       |        |         |         |
 *       v        v         v         v
 *      AI    classical   quantum   other
 *                         |
 *                         v
 *                     quantum::ir
 *
 * The AI directory therefore consumes uncertainty rather than redefining it.
 *
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar AIUncertainty;

import UncertaintyExpressions;


/*
 * ============================================================================
 * PUBLIC AI UNCERTAINTY COMPOSITION BOUNDARY
 * ============================================================================
 *
 * This rule is intentionally a one-rule adapter.
 *
 * The universal uncertainty syntax remains owned by:
 *
 *     UncertaintyExpressions.uncertaintyExpression
 *
 * No AI-specific syntax is introduced here.
 *
 * ============================================================================
 */

aiUncertaintyConstruct
    : uncertaintyExpression
    ;