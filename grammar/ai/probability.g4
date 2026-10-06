/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/ai/probability.g4
* 
* Grammar:
* AIProbability
* 
* Status:
* CANONICAL AI-DOMAIN PROBABILITY COMPOSITION GRAMMAR
* 
* Compiler baseline:
* Rust 1.97 or later
* Rust 2021 edition
* Safe Rust only
* No unsafe Rust
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the focused AI-domain composition boundary for probability
* semantics.
* 
* IMPORTANT:
* 
* This file does NOT own the universal expression system.
* 
* This file does NOT own universal uncertainty syntax.
* 
* This file does NOT own probability mathematics.
* 
* This file does NOT own statistical algorithms.
* 
* This file does NOT own distribution implementations.
* 
* This file does NOT own random-number generation.
* 
* This file does NOT own inference implementation.
* 
* This file does NOT own quantum probability implementation.
* 
* This file does NOT own runtime execution.
* 
* The architectural purpose of this file is to provide a small, stable,
* extensible probability-language boundary that can be consumed by the
* existing probabilistic AI grammar without creating another probability
* language.
* 
* The intended architecture is:
* 
* Zamani source
*      |
*      v
* ZamaniLexer
*      |
*      v
* canonical parser
*      |
*      +-------------------------------+
*      |                               |
*      v                               v
* universal expressions           AIProbability
*      |                               |
*      +---------------+---------------+
*                      |
*                      v
*              domain-neutral AST
*                      |
*                      v
*              structural validation
*                      |
*         +------------+-------------+
*         |            |             |
*         v            v             v
*       types       semantics      provenance
*         |            |             |
*         +------------+-------------+
*                      |
*                      v
*             canonical semantic model
*                      |
*         +------------+-------------+
*         |            |             |
*         v            v             v
*     classical    quantum::ir    other domains
*         |            |             |
*         +------------+-------------+
*                      |
*                      v
*            optimization/lowering
*                      |
*              target realization
* 
* ============================================================================
* ARCHITECTURAL PRINCIPLE
* ============================================================================
* 
* Probability is a UNIVERSAL COMPUTATIONAL CONCEPT.
* 
* It may occur in:
* 
* classical computation
* scientific computation
* statistics
* machine learning
* reasoning
* knowledge systems
* simulation
* distributed computation
* reliability analysis
* control systems
* networking
* hardware observation
* quantum computation
* quantum measurement
* quantum error analysis
* hybrid computation
* future computational domains
* 
* Consequently, this file must remain domain-neutral at the semantic boundary
* while still being located under AI because it is currently consumed by the
* AI probabilistic subsystem.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - AI-domain probability composition;
* - probability-intent source constructs;
* - probability relationship syntax;
* - probability expectation syntax;
* - conditional probability syntax;
* - probability observation syntax;
* - probability assertion syntax;
* - probability metadata composition;
* - probability-domain integration with the existing probabilistic grammar;
* - the public AIProbability parser boundary.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexer rules;
* - keyword spelling;
* - identifiers;
* - literals;
* - operators;
* - expression precedence;
* - general expressions;
* - function calls;
* - indexing;
* - member access;
* - type expressions;
* - uncertainty expression syntax;
* - distribution catalogues;
* - distribution algorithms;
* - statistical algorithms;
* - Bayesian algorithms;
* - Monte Carlo algorithms;
* - MCMC algorithms;
* - variational inference;
* - particle filtering;
* - random-number generation;
* - entropy-source selection;
* - model execution;
* - tensor execution;
* - learning implementation;
* - inference implementation;
* - reasoning implementation;
* - knowledge storage;
* - provenance implementation;
* - policy enforcement;
* - resource negotiation;
* - capability discovery;
* - hardware discovery;
* - device selection;
* - target selection;
* - scheduling;
* - routing;
* - quantum operation syntax;
* - quantum physical mapping;
* - QEC;
* - ZQN;
* - HAL;
* - runtime execution;
* - probabilistic IR;
* - AI-specific IR.
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* The repository already has:
* 
* grammar/expressions/uncertainty.g4
* 
* as the canonical source-level uncertainty expression owner.
* 
* That file owns:
* 
* uncertaintyExpression
* uncertainValueExpression
* uncertaintyValue
* uncertaintyArgumentList
* uncertaintyArgument
* uncertaintyNamedArgument
* uncertaintyPositionalArgument
* uncertaintyFieldName
* 
* This file MUST NOT redefine any of those rules.
* 
* The repository also has:
* 
* grammar/ai/probabilistic.g4
* 
* That file currently contains broad probabilistic constructs.
* 
* The production architecture must converge on:
* 
* AIProbabilistic
*      |
*      +--> AIProbability
*      |
*      +--> other probabilistic constructs
* 
* rather than allowing probability rules to be duplicated in both files.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/antlr/ZamaniLexer.g4
* canonical Zamani expression grammar
* canonical Zamani type grammar
* canonical uncertainty expression grammar
* 
* Direct grammar dependency:
* 
* Expressions
* 
* is intentionally used because probability operands are ordinary Zamani
* expressions.
* 
* The grammar must never create a competing expression hierarchy.
* 
* ============================================================================
* EXPORT CONTRACT
* ============================================================================
* 
* PUBLIC RULE:
* 
* aiProbabilityConstruct
* 
* PUBLIC SPECIALIZED RULES:
* 
* probabilityConstruct
* probabilityValue
* probabilityRelationship
* probabilityExpectation
* probabilityCondition
* probabilityObservation
* probabilityAssertion
* 
* The public surface is deliberately small.
* 
* ============================================================================
* CONSUMED BY
* ============================================================================
* 
* Primary consumer:
* 
* grammar/ai/probabilistic.g4
* 
* Required composition:
* 
* AIProbabilistic
*      |
*      +--> AIProbability
* 
* "grammar/ai/ai.g4" should normally reach probability through:
* 
* AI
*   |
*   v
* AIProbabilistic
*   |
*   v
* AIProbability
* 
* This file MUST NOT be imported directly by the root parser when the existing
* AI composition path already reaches it through AIProbabilistic.
* 
* ============================================================================
* ROOT-PARSER INTEGRATION
* ============================================================================
* 
* The intended parser path is:
* 
* grammar/Zamani.g4
*      |
*      v
* canonical parser
*      |
*      v
* AI
*      |
*      v
* AIProbabilistic
*      |
*      v
* AIProbability
* 
* There must be exactly one parser path for every probability construct.
* 
* The root parser must not independently import:
* 
* AIProbability
* 
* when AIProbabilistic already imports it.
* 
* This prevents duplicate alternatives and ambiguous parser ownership.
* 
* ============================================================================
* LEXER CONTRACT
* ============================================================================
* 
* This file contains NO lexer rules.
* 
* It introduces NO new tokens.
* 
* Existing canonical lexical vocabulary includes:
* 
* PROBABILITY
* PROBABILISTIC
* DISTRIBUTION
* CONFIDENCE
* BELIEF
* LIKELIHOOD
* UNCERTAIN
* UNCERTAINTY
* EVIDENCE
* PROVENANCE
* OBSERVATION
* INFER
* 
* No new probability keyword is required by this grammar.
* 
* The canonical lexer already reserves:
* 
* probability
* 
* Therefore:
* 
* @probability
* 
* is structurally:
* 
* AT PROBABILITY
* 
* rather than:
* 
* AT IDENTIFIER
* 
* This distinction is intentional.
* 
* ============================================================================
* NO KEYWORD EXPLOSION
* ============================================================================
* 
* Probability theory contains an open-ended vocabulary.
* 
* This file MUST NOT introduce reserved keywords for concepts such as:
* 
* prior
* posterior
* event
* outcome
* sample
* expectation
* variance
* covariance
* entropy
* evidence_strength
* density
* mass
* hazard
* odds
* odds_ratio
* calibration
* interval
* quantile
* percentile
* moment
* cumulant
* 
* unless a separate language-wide lexical decision explicitly establishes
* such a word as a Zamani keyword.
* 
* Such concepts remain representable as identifiers, qualified names,
* functions, library constructs, or dialect-defined semantic vocabulary.
* 
* ============================================================================
* EXPRESSION CONTRACT
* ============================================================================
* 
* All probability operands are ordinary Zamani expressions.
* 
* This file does NOT define:
* 
* expression
* primaryExpression
* unaryExpression
* binaryExpression
* callExpression
* indexingExpression
* memberExpression
* lambdaExpression
* matchExpression
* 
* Those belong to:
* 
* grammar/expressions/expressions.g4
* 
* Probability syntax consumes the canonical:
* 
* expression
* 
* boundary.
* 
* This permits probability to operate on:
* 
* scalar values
* tuples
* records
* arrays
* tensors
* model outputs
* dataset values
* knowledge results
* reasoning results
* learning results
* quantum measurements
* simulation results
* hardware observations
* distributed results
* future domain values
* 
* without changing this grammar.
* 
* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* This file defines NO probability type.
* 
* In particular, it MUST NOT define:
* 
* ProbabilityType
* AIProbabilityType
* ScalarProbabilityType
* QuantumProbabilityType
* 
* as competing universal types.
* 
* The universal type system remains authoritative.
* 
* Semantic types may represent concepts such as:
* 
* Probability<T>
* Distribution<T>
* Uncertain<T>
* 
* but their exact representation belongs to:
* 
* grammar/types/
* frontend semantic type analysis
* 
* This grammar only describes source structure.
* 
* ============================================================================
* PROBABILITY VALUE MODEL
* ============================================================================
* 
* A probability value is semantically open.
* 
* It may be represented by:
* 
* numeric value
* symbolic expression
* exact value
* interval
* algebraic representation
* arbitrary-precision representation
* rational representation
* logarithmic representation
* domain-specific probability object
* quantum-derived probability
* distribution-derived value
* model-derived value
* future representation
* 
* The grammar imposes none of those representations.
* 
* ============================================================================
* MATHEMATICAL VALIDATION BOUNDARY
* ============================================================================
* 
* The parser MUST NOT validate mathematical probability invariants.
* 
* For example, parser syntax MUST NOT enforce:
* 
* 0 <= p <= 1
* 
* It MUST NOT enforce:
* 
* sum(p_i) = 1
* 
* It MUST NOT enforce:
* 
* finite(p)
* 
* It MUST NOT enforce:
* 
* nonnegative(p)
* 
* Those are semantic/type/domain validation responsibilities.
* 
* This separation is necessary because probability may be represented
* symbolically, exactly, approximately, interval-wise, or through a
* domain-specific semantic object.
* 
* ============================================================================
* DISTRIBUTION CONTRACT
* ============================================================================
* 
* Distribution names are expressions/identifiers.
* 
* This grammar does NOT enumerate:
* 
* Gaussian
* Normal
* Bernoulli
* Binomial
* Poisson
* Uniform
* Categorical
* Multinomial
* Gamma
* Beta
* Dirichlet
* custom distributions
* 
* A distribution can therefore be:
* 
* gaussian(mu, sigma)
* 
* custom::distribution(parameters)
* 
* model.distribution(parameters)
* 
* distribution_expression
* 
* without changing this grammar.
* 
* ============================================================================
* CONDITIONAL PROBABILITY
* ============================================================================
* 
* Conditional probability is represented explicitly.
* 
* Canonical form:
* 
* @probability event given condition;
* 
* The operands remain ordinary expressions.
* 
* The grammar does not encode Bayes' theorem.
* 
* Semantic analysis determines:
* 
* whether the condition is meaningful;
* whether the event is compatible;
* whether the probability is defined;
* whether evidence is sufficient;
* whether the requested operation can be realized.
* 
* ============================================================================
* PROBABILITY RELATIONSHIP
* ============================================================================
* 
* A general relationship form is supported:
* 
* @probability event;
* 
* @probability event given condition;
* 
* @probability(event);
* 
* @probability(event, given: condition);
* 
* The explicit forms are semantically equivalent only where the language
* version and semantic rules declare them equivalent.
* 
* ============================================================================
* EXPECTATION
* ============================================================================
* 
* Expectation is represented as a probability-domain operation rather than a
* fixed mathematical implementation.
* 
* Canonical form:
* 
* @probability expectation(expression);
* 
* The word "expectation" remains an ordinary identifier.
* 
* This deliberately avoids adding an EXPECTATION lexer keyword.
* 
* It permits:
* 
* expectation(...)
* 
* to remain a library or semantic operation as well.
* 
* The parser therefore treats the operation structurally rather than
* enumerating mathematical functions.
* 
* ============================================================================
* OBSERVATION
* ============================================================================
* 
* Probability may be related to an observed value:
* 
* @probability event observed value;
* 
* or:
* 
* @probability(event, observation: value);
* 
* The exact semantic interpretation belongs downstream.
* 
* This file does not redefine the universal observation grammar.
* 
* ============================================================================
* ASSERTION
* ============================================================================
* 
* Probability assertions can be represented through:
* 
* @probability event = value;
* 
* or:
* 
* @probability(event, value);
* 
* The grammar preserves the relationship.
* 
* Semantic validation determines whether the right-hand side is a valid
* probability representation.
* 
* ============================================================================
* METADATA
* ============================================================================
* 
* Probability metadata is intentionally open.
* 
* Standard semantic fields can include:
* 
* distribution
* confidence
* belief
* likelihood
* evidence
* provenance
* source
* 
* The grammar does not require every future probability property to become a
* lexer keyword.
* 
* Metadata is represented through ordinary named arguments.
* 
* ============================================================================
* ARGUMENT MODEL
* ============================================================================
* 
* Probability operations accept an open-ended argument list.
* 
* There is no universal fixed argument count.
* 
* This supports future mathematical and computational forms without changing
* the grammar.
* 
* Trailing commas are deliberately rejected in probability argument lists
* unless the canonical argument grammar explicitly establishes trailing-comma
* compatibility.
* 
* ============================================================================
* PROBABILITY DECLARATION
* ============================================================================
* 
* This grammar does not introduce a second variable declaration system.
* 
* A probability construct may reference a previously declared value:
* 
* @probability event;
* 
* Variable declarations remain owned by the universal declaration grammar.
* 
* ============================================================================
* SAMPLING INTEGRATION
* ============================================================================
* 
* Sampling syntax remains owned by:
* 
* grammar/ai/probabilistic.g4
* 
* or another canonical sampling owner.
* 
* This file may provide probability expressions that sampling consumes.
* 
* It MUST NOT redefine:
* 
* sample
* random
* random-variable declaration
* 
* ============================================================================
* INFERENCE INTEGRATION
* ============================================================================
* 
* General inference remains owned by:
* 
* grammar/ai/inference.g4
* 
* Probability constructs may become inference inputs or results.
* 
* This file does not define:
* 
* Bayesian inference
* exact inference
* approximate inference
* variational inference
* MCMC
* particle filtering
* sequential Monte Carlo
* 
* ============================================================================
* LEARNING INTEGRATION
* ============================================================================
* 
* Probability may participate in:
* 
* learning objectives
* likelihoods
* predictions
* uncertainty estimation
* model evaluation
* adaptive decisions
* 
* Learning syntax remains owned by the learning/training subsystem.
* 
* ============================================================================
* REASONING INTEGRATION
* ============================================================================
* 
* Probability may participate in:
* 
* infer
* deduce
* reason
* 
* Reasoning grammar remains authoritative for those constructs.
* 
* Example:
* 
* @probability infer(hypothesis);
* 
* is structurally an ordinary expression operand.
* 
* This file does not redefine reasoning syntax.
* 
* ============================================================================
* KNOWLEDGE INTEGRATION
* ============================================================================
* 
* Probability may annotate or evaluate knowledge-derived values.
* 
* Examples:
* 
* @probability query_result;
* 
* @probability query_result given evidence;
* 
* Knowledge syntax remains owned by the knowledge subsystem.
* 
* ============================================================================
* UNCERTAINTY INTEGRATION
* ============================================================================
* 
* Universal uncertainty syntax remains owned by:
* 
* grammar/expressions/uncertainty.g4
* 
* This file may consume uncertainty expressions through:
* 
* expression
* 
* It must not import and redefine:
* 
* uncertaintyExpression
* 
* The semantic relationship is:
* 
* probability
*      |
*      +--> uncertainty
* 
* without creating a second uncertainty language.
* 
* ============================================================================
* CONFIDENCE / BELIEF / LIKELIHOOD
* ============================================================================
* 
* These remain semantic concepts.
* 
* This file does not establish:
* 
* confidence scale;
* belief interpretation;
* likelihood implementation;
* probability precision.
* 
* Such semantics are determined by type and semantic analysis.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Probability syntax does not automatically imply a particular effect.
* 
* Semantic analysis may derive:
* 
* randomness
* measurement
* learning
* IO
* network
* distributed
* native
* foreign
* 
* depending on the resolved operands and implementation.
* 
* For example:
* 
* @probability measure(q);
* 
* may involve quantum measurement.
* 
* The parser does not infer this effect.
* 
* ============================================================================
* CAPABILITY CONTRACT
* ============================================================================
* 
* Semantic analysis may derive capabilities such as:
* 
* probabilistic.compute
* probabilistic.inference
* statistical.compute
* randomness.entropy
* quantum.measurement
* model.inference
* provenance.record
* 
* Capability resolution is downstream.
* 
* The grammar does not inspect the target machine.
* 
* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* Probability computation may require:
* 
* compute
* memory
* storage
* communication
* accelerator resources
* quantum resources
* model resources
* 
* These belong to:
* 
* grammar/resources/
* 
* No resource allocation is performed by this grammar.
* 
* There are NO constants for:
* 
* maximum samples
* maximum outcomes
* maximum variables
* maximum distribution size
* maximum probability precision
* maximum tensor rank
* maximum qubits
* maximum CPUs
* maximum GPUs
* maximum FPGAs
* maximum nodes
* maximum memory
* maximum devices
* 
* ============================================================================
* CONTRACT INTEGRATION
* ============================================================================
* 
* Probability expressions may participate in universal contracts:
* 
* requires
* ensures
* invariant
* assume
* guarantee
* property
* 
* The contract grammar remains authoritative.
* 
* This file does not redefine contract syntax.
* 
* ============================================================================
* POLICY INTEGRATION
* ============================================================================
* 
* Policies may constrain:
* 
* randomness
* evidence
* provenance
* reproducibility
* external data
* model use
* distribution use
* sampling strategy
* execution strategy
* 
* Policy enforcement remains downstream.
* 
* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* Probability results may carry provenance describing:
* 
* source
* derivation
* evidence
* model
* measurement
* transformation
* verification
* version
* decision
* 
* Provenance semantics remain owned by the canonical provenance subsystem.
* 
* This grammar does not create an AI-specific provenance representation.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* Every public construct must map to the existing domain-neutral AST model.
* 
* Conceptual structure:
* 
* ProbabilityConstruct {
*     operation,
*     operands,
*     condition?,
*     metadata?,
*     source_span
* }
* 
* The exact Rust type is owned by the frontend AST implementation.
* 
* The AST MUST NOT contain:
* 
* GPUProbability
* CPUProbability
* QuantumProbability
* BayesianProbabilityIR
* VendorProbability
* 
* as universal syntax categories.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis determines:
* 
* - probability value validity;
* - operand compatibility;
* - conditional relationship validity;
* - observation validity;
* - metadata validity;
* - type compatibility;
* - probability invariants;
* - distribution compatibility;
* - evidence compatibility;
* - provenance requirements;
* - effects;
* - capabilities;
* - resources;
* - contracts;
* - policies;
* - determinism/reproducibility requirements.
* 
* The parser performs none of these checks.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar creates NO IR.
* 
* The canonical lowering path is:
* 
* probability syntax
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic probability model
*      |
*      v
* canonical semantic representation
*      |
*      +----------------------+
*      |                      |
*      v                      v
*  classical              quantum::ir
*      |                      |
*      +----------+-----------+
*                 |
*                 v
*            optimization
*                 |
*              lowering
*                 |
*           target realization
* 
* There must be no:
* 
* ProbabilityIR
* AIProbabilityIR
* QuantumProbabilityIR
* 
* introduced by this grammar.
* 
* ============================================================================
* QUANTUM CONTRACT
* ============================================================================
* 
* Probability may arise from:
* 
* quantum measurement
* quantum simulation
* quantum channels
* noise models
* resilience analysis
* hybrid algorithms
* 
* This grammar does not define any quantum operation.
* 
* The downstream path remains:
* 
* source
*   |
*   v
* domain-neutral AST
*   |
*   v
* semantic quantum model
*   |
*   v
* quantum::ir
*   |
*   v
* optimization
*   |
*   v
* decomposition
*   |
*   v
* routing
*   |
*   v
* scheduling
*   |
*   v
* resilience / QEC / ZQN
*   |
*   v
* HAL
*   |
*   v
* target
* 
* No probability-specific quantum IR is created.
* 
* ============================================================================
* HDL / HARDWARE CONTRACT
* ============================================================================
* 
* Probability may describe:
* 
* sensor uncertainty
* timing uncertainty
* hardware reliability
* fault likelihood
* simulation output
* probabilistic hardware behavior
* 
* The grammar does not encode:
* 
* register width
* bus width
* device count
* clock frequency
* physical topology
* memory capacity
* 
* Hardware realization remains downstream.
* 
* ============================================================================
* DISTRIBUTED CONTRACT
* ============================================================================
* 
* Probability may be computed across:
* 
* tasks
* actors
* services
* nodes
* clusters
* heterogeneous systems
* future distributed systems
* 
* This grammar does not encode node counts, worker counts, or topology.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing is deterministic.
* 
* The parser may depend only on:
* 
* source tokens
* grammar version
* parser configuration
* enabled dialect configuration
* 
* It must not depend on:
* 
* wall-clock time
* random state
* hardware availability
* filesystem state
* network state
* runtime state
* scheduler state
* installed probability libraries
* available GPUs
* available QPUs
* 
* Runtime stochasticity is a semantic/runtime property.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Parsing must never:
* 
* sample a distribution;
* invoke a random generator;
* acquire entropy;
* contact a service;
* query a database;
* inspect hardware;
* load a model;
* execute foreign code;
* invoke a QPU;
* invoke a simulator.
* 
* ============================================================================
* SAFE-RUST CONTRACT
* ============================================================================
* 
* This grammar contains:
* 
* no embedded Rust;
* no actions;
* no semantic predicates;
* no filesystem access;
* no network access;
* no hardware access;
* no runtime execution.
* 
* Generated parser integration must remain compatible with:
* 
* Rust 1.97 or later
* Rust 2021
* safe Rust only.
* 
* No unsafe Rust is required.
* 
* ============================================================================
* POCO-REAF / SCALABILITY CONTRACT
* ============================================================================
* 
* This grammar imposes no language-level finite ceiling on:
* 
* probability expressions
* probability operands
* metadata fields
* nested expressions
* distribution parameters
* evidence
* observations
* variables
* models
* datasets
* tensor dimensions
* tensor rank
* samples
* nodes
* workers
* devices
* accelerators
* qubits
* memory
* 
* "Infinity" means:
* 
* no artificial language-level finite capacity.
* 
* It does NOT mean an implementation has infinite physical memory or infinite
* parser stack capacity.
* 
* Actual exhaustion is handled as an implementation/resource condition.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file MUST contain none of:
* 
* MAX_PROBABILITY_BITS
* MAX_PROBABILITY_PRECISION
* MAX_OUTCOMES
* MAX_EVENTS
* MAX_SAMPLES
* MAX_RANDOM_VARIABLES
* MAX_DISTRIBUTIONS
* MAX_PARAMETERS
* MAX_EVIDENCE
* MAX_OBSERVATIONS
* MAX_TENSOR_RANK
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_DEVICE_COUNT
* 
* It also must not contain:
* 
* physical device identifiers;
* vendor probability engines;
* fixed distribution inventories;
* fixed random-source implementations;
* fixed tensor dimensions;
* fixed machine capacities.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* This file adds a focused probability composition boundary.
* 
* It MUST NOT invalidate existing ordinary expressions except where the
* canonical lexer has already reserved:
* 
* probability
* 
* Existing source code using "probability" as an ordinary identifier is
* therefore already subject to the lexical compatibility rules of the
* repository.
* 
* Compatibility handling belongs to:
* 
* grammar/compatibility/
* 
* This grammar must not create alternative lexer tokens to work around that
* issue.
* 
* ============================================================================
* FILE-LEVEL DEPENDENCY DECLARATION
* ============================================================================
* 
* DEPENDS_ON:
* 
* ZamaniLexer
* Expressions
* 
* EXPORTS:
* 
* aiProbabilityConstruct
* probabilityConstruct
* probabilityValue
* probabilityRelationship
* probabilityExpectation
* probabilityCondition
* probabilityObservation
* probabilityAssertion
* 
* CONSUMED_BY:
* 
* AIProbabilistic
* 
* AST_OWNER:
* 
* domain-neutral frontend AST
* 
* SEMANTIC_OWNER:
* 
* canonical probability semantic model
* 
* TYPE_OWNER:
* 
* canonical type system
* 
* EFFECT_OWNER:
* 
* canonical effect analysis
* 
* CAPABILITY_OWNER:
* 
* canonical capability analysis
* 
* RESOURCE_OWNER:
* 
* canonical resource analysis
* 
* CONTRACT_OWNER:
* 
* grammar/validation/
* 
* POLICY_OWNER:
* 
* grammar/policies/
* grammar/security/
* 
* PROVENANCE_OWNER:
* 
* canonical provenance subsystem
* 
* IR_OWNER:
* 
* canonical semantic representation
* quantum::ir for quantum computation
* 
* TEST_OWNER:
* 
* grammar/tests/ai/
* grammar/tests/probability/
* grammar/tests/parser/
* grammar/tests/semantic/
* grammar/tests/quantum/
* grammar/tests/hybrid/
* grammar/tests/scalability/
* grammar/tests/portability/
* grammar/tests/negative/
* grammar/tests/boundary/
* 
* SPEC_OWNER:
* 
* grammar/spec/ai.md
* grammar/spec/uncertainty.md
* grammar/spec/resources.md
* grammar/spec/effects.md
* grammar/spec/provenance.md
* grammar/spec/policies.md
* grammar/specification/poco-reaf.md
* 
* ============================================================================
* INTEGRATION WITH EXISTING PROBABILISTIC.G4
* ============================================================================
* 
* "grammar/ai/probabilistic.g4" currently contains broad probabilistic syntax.
* 
* The canonical ownership after integration must be:
* 
* AIProbabilistic
*      |
*      +--> AIProbability
*      |
*      +--> random-variable constructs
*      +--> sampling constructs
*      +--> observation constructs not owned here
*      +--> conditioning constructs not owned here
*      +--> inference requests
*      +--> probabilistic regions
* 
* Probability-specific rules MUST NOT remain duplicated in
* "probabilistic.g4".
* 
* In particular, the following concepts must have one owner:
* 
* probability relationship
* probability expectation
* probability assertion
* probability conditional form
* 
* The existing probabilistic grammar should delegate to:
* 
* aiProbabilityConstruct
* 
* rather than reproducing these productions.
* 
* ============================================================================
* INTEGRATION WITH AI.G4
* ============================================================================
* 
* "grammar/ai/ai.g4" remains the AI composition boundary.
* 
* AI.g4 should continue to import:
* 
* AIProbabilistic
* 
* It should NOT import AIProbability directly when AIProbabilistic already
* provides the composition path.
* 
* This maintains one parser path:
* 
* AI
*   |
*   v
* AIProbabilistic
*   |
*   v
* AIProbability
* 
* ============================================================================
* INTEGRATION WITH EXPRESSIONS.G4
* ============================================================================
* 
* No modification to the expression precedence ladder is required for this
* AI-domain file.
* 
* Probability operands are ordinary:
* 
* expression
* 
* values.
* 
* Universal source-level probability values that eventually need to appear as
* first-class expressions should be added through the canonical expression
* subsystem, not by making this AI file a second expression root.
* 
* If a future universal probability expression is required, the correct
* location is:
* 
* grammar/expressions/
* 
* with this file consuming it through a public expression boundary.
* 
* ============================================================================
* INTEGRATION WITH UNCERTAINTY.G4
* ============================================================================
* 
* "grammar/expressions/uncertainty.g4" remains the sole owner of:
* 
* uncertaintyExpression
* 
* Probability may be an uncertainty metadata value:
* 
* uncertain(value, probability: p)
* 
* or a probability construct may consume an uncertainty result:
* 
* @probability uncertain_value;
* 
* No duplicate uncertainty production is permitted here.
* 
* ============================================================================
* INTEGRATION WITH TYPES.G4
* ============================================================================
* 
* Probability values use:
* 
* typeExpression
* 
* only through semantic/type analysis.
* 
* This file does not add:
* 
* probabilityType
* 
* to the parser type hierarchy.
* 
* A semantic implementation may eventually standardize:
* 
* Probability<T>
* 
* as a generic semantic type while keeping the parser grammar open.
* 
* ============================================================================
* INTEGRATION WITH RESOURCES
* ============================================================================
* 
* Probability semantic operations may produce resource requirements.
* 
* Example semantic intent:
* 
* requires capability("probabilistic.compute");
* 
* requires capability("randomness.entropy");
* 
* requires memory >= required_memory;
* 
* The grammar does not perform resource negotiation.
* 
* ============================================================================
* INTEGRATION WITH EFFECTS
* ============================================================================
* 
* Probability operations may resolve to effects including:
* 
* randomness
* measurement
* learning
* network
* distributed
* foreign
* native
* 
* The effect is determined by semantic resolution.
* 
* ============================================================================
* INTEGRATION WITH PROVENANCE
* ============================================================================
* 
* Probability results may preserve:
* 
* source
* evidence
* derivation
* model
* transformation
* measurement
* verification
* version
* 
* Provenance remains a shared semantic subsystem.
* 
* ============================================================================
* INTEGRATION WITH QUANTUM
* ============================================================================
* 
* Quantum-derived probability remains target-neutral at this boundary.
* 
* Example:
* 
* @probability measure(q);
* 
* The quantum semantic path remains:
* 
* semantic model
*      |
*      v
* quantum::ir
*      |
*      v
* optimization
*      |
*      v
* routing
*      |
*      v
* scheduling
*      |
*      v
* resilience / QEC / ZQN
*      |
*      v
* HAL
* 
* This file does not define any physical quantum mapping.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* POSITIVE PARSER TESTS
* ---
* 
* @probability event;
* 
* @probability(event);
* 
* @probability event given condition;
* 
* @probability(event, given: condition);
* 
* @probability event observed observation;
* 
* @probability(event, observation: observation_value);
* 
* @probability event = probability_value;
* 
* @probability(event, probability_value);
* 
* @probability expectation(expression);
* 
* @probability(expectation(expression));
* 
* @probability event given condition with (evidence);
* 
* @probability(
*     event,
*     given: condition,
*     evidence: evidence_value,
*     provenance: source
* );
* 
* POSITIVE CROSS-DOMAIN TESTS
* ---
* 
* @probability measure(q);
* 
* @probability model.predict(input);
* 
* @probability query_result given evidence;
* 
* @probability tensor[index];
* 
* @probability simulation_result;
* 
* @probability distributed_result;
* 
* @probability hardware_observation;
* 
* NEGATIVE PARSER TESTS
* ---
* 
* @probability;
* 
* @probability();
* 
* @probability(;
* 
* @probability event given;
* 
* @probability event =;
* 
* @probability(event,);
* 
* @probability(event, given:);
* 
* @probability(event, : condition);
* 
* @probability event given condition given other;
* 
* @probability event observed;
* 
* @probability(event, observation:);
* 
* @probability(expectation();
* 
* @probability event with ();
* 
* Semantic tests must separately reject:
* 
* invalid probability values;
* incompatible event/condition types;
* invalid distributions;
* invalid evidence;
* unavailable capabilities;
* unsatisfied resources;
* policy violations;
* contract violations.
* 
* Those are semantic failures, not grammar failures.
* 
* ============================================================================
* SCALABILITY TESTS
* ============================================================================
* 
* Test progressively larger:
* 
* expressions;
* conditional relationships;
* metadata lists;
* nested probability expressions;
* distribution expressions;
* evidence structures;
* provenance structures;
* model references;
* tensor expressions;
* quantum-derived expressions.
* 
* Do not encode a maximum.
* 
* Verify that parsing does not depend on:
* 
* CPU count;
* GPU count;
* FPGA count;
* QPU count;
* node count;
* worker count;
* memory capacity;
* tensor rank;
* distribution count;
* sample count.
* 
* ============================================================================
* BOUNDARY TESTS
* ============================================================================
* 
* Test:
* 
* scalar event;
* tuple event;
* record event;
* collection event;
* tensor event;
* model event;
* quantum event;
* distributed event;
* simulation event;
* hardware observation.
* 
* Test nested expressions:
* 
* @probability uncertain(value);
* 
* @probability expectation(uncertain(value));
* 
* @probability measure(q) given uncertain(condition);
* 
* ============================================================================
* DETERMINISM TESTS
* ============================================================================
* 
* Identical:
* 
* source
* lexer version
* grammar version
* parser configuration
* 
* must produce equivalent parse-tree structure.
* 
* Runtime random state must not influence parsing.
* 
* ============================================================================
* COMPATIBILITY TESTS
* ============================================================================
* 
* Verify:
* 
* existing probabilistic constructs;
* existing uncertainty expressions;
* existing inference constructs;
* existing model expressions;
* existing tensor expressions;
* existing quantum expressions;
* existing distributed expressions;
* existing resource expressions.
* 
* The new probability boundary must not introduce a second parser route.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* FILE-LOCAL COMPLETION
* ---
* 
* [x] Uses canonical ZamaniLexer.
* 
* [x] Introduces no lexer rules.
* 
* [x] Introduces no new keywords.
* 
* [x] Uses ordinary Zamani expressions for operands.
* 
* [x] Does not redefine uncertaintyExpression.
* 
* [x] Does not define a second type system.
* 
* [x] Does not define probability mathematics.
* 
* [x] Does not enumerate distributions.
* 
* [x] Does not enumerate inference algorithms.
* 
* [x] Does not enumerate sampling algorithms.
* 
* [x] Does not select hardware.
* 
* [x] Does not allocate resources.
* 
* [x] Does not execute code.
* 
* [x] Does not create an IR.
* 
* [x] Does not contain embedded Rust.
* 
* [x] Does not require unsafe Rust.
* 
* [x] Contains no finite machine/resource constants.
* 
* [x] Provides explicit AST integration.
* 
* [x] Provides explicit semantic integration.
* 
* [x] Provides explicit effect integration.
* 
* [x] Provides explicit capability integration.
* 
* [x] Provides explicit resource integration.
* 
* [x] Provides explicit contract integration.
* 
* [x] Provides explicit policy integration.
* 
* [x] Provides explicit provenance integration.
* 
* [x] Provides explicit quantum::ir integration.
* 
* [x] Provides explicit scalability rules.
* 
* REPOSITORY COMPLETION
* ---
* 
* [ ] Add this grammar to the ANTLR source set.
* 
* [ ] Import AIProbability from AIProbabilistic.
* 
* [ ] Replace duplicated probability productions in probabilistic.g4 with
* aiProbabilityConstruct delegation.
* 
* [ ] Verify AI.g4 reaches probability exactly once.
* 
* [ ] Add frontend AST mapping.
* 
* [ ] Add semantic probability model.
* 
* [ ] Add type validation.
* 
* [ ] Add effect analysis.
* 
* [ ] Add capability analysis.
* 
* [ ] Add resource analysis.
* 
* [ ] Add contract validation.
* 
* [ ] Add policy validation.
* 
* [ ] Add provenance propagation.
* 
* [ ] Add classical lowering.
* 
* [ ] Verify quantum-derived probability reaches quantum::ir.
* 
* [ ] Add parser tests.
* 
* [ ] Add semantic tests.
* 
* [ ] Add cross-domain tests.
* 
* [ ] Add negative tests.
* 
* [ ] Add boundary tests.
* 
* [ ] Add scalability tests.
* 
* [ ] Add determinism tests.
* 
* [ ] Add compatibility tests.
* 
* [ ] Verify generated parser compatibility with Rust 1.97 or later.
* 
* [ ] Verify no unsafe Rust is introduced.
* 
* ============================================================================
* GRAMMAR
* ============================================================================
  */

parser grammar AIProbability;

options {
tokenVocab = ZamaniLexer;
}

import
Expressions
;

/*

* ============================================================================
* PUBLIC AI PROBABILITY ENTRY
* ============================================================================
* 
* Every probability-domain construct enters through this rule.
* 
* The semicolon is intentionally owned here because these constructs are
* statement-like AI-domain declarations/operations.
  */
  aiProbabilityConstruct
  : probabilityConstruct
  ;

/*

* ============================================================================
* PROBABILITY CONSTRUCT
* ============================================================================
* 
* Supported source forms:
* 
* @probability event;
* 
* @probability(event);
* 
* @probability event given condition;
* 
* @probability(event, given: condition);
* 
* @probability event observed observation;
* 
* @probability(event, observation: observation);
* 
* @probability event = probabilityValue;
* 
* @probability(event, probabilityValue);
* 
* @probability expectation(expression);
* 
* @probability(expectation(expression));
* 
* The grammar preserves intent.
* 
* Mathematical interpretation is semantic.
  */
  probabilityConstruct
  : AT PROBABILITY
  (
  probabilityExpectation
  | probabilityRelationship
  | probabilityObservation
  | probabilityAssertion
  )
  SEMICOLON
  ;

/*

* ============================================================================
* PROBABILITY RELATIONSHIP
* ============================================================================
* 
* Basic:
* 
* @probability event
* 
* Conditional:
* 
* @probability event given condition
* 
* Call-style:
* 
* @probability(event)
* 
* Metadata:
* 
* @probability(
*     event,
*     given: condition,
*     evidence: evidence
* )
* 
* The first expression is always the subject/event.
  /
  probabilityRelationship
  : probabilityValue
  (
  probabilityGivenClause
  probabilityMetadataClause
  | probabilityArgumentClause
  )?
  ;

/*

* ============================================================================
* CALL-STYLE PROBABILITY ARGUMENTS
* ============================================================================
* 
* @probability(event)
* 
* @probability(event, given: condition)
* 
* @probability(event, observation: value)
* 
* @probability(event, probability: p)
* 
* @probability(event, metadata: value)
* 
* An empty argument list is deliberately rejected.
  */
  probabilityArgumentClause
  : LPAREN
  probabilityArgumentList
  RPAREN
  ;

/*

* ============================================================================
* ARGUMENT LIST
* ============================================================================
* 
* Open-ended.
* 
* There is no finite argument count.
  /
  probabilityArgumentList
  : probabilityArgument
  (
  COMMA
  probabilityArgument
  )
  ;

/*

* ============================================================================
* ARGUMENT
* ============================================================================
* 
* Named arguments are preferred for semantic metadata.
  */
  probabilityArgument
  : probabilityNamedArgument
  | probabilityValue
  ;

/*

* ============================================================================
* NAMED ARGUMENT
* ============================================================================
* 
* Standard fields:
* 
* given
* observation
* probability
* distribution
* confidence
* belief
* likelihood
* evidence
* provenance
* source
* 
* The field name remains open through IDENTIFIER.
  */
  probabilityNamedArgument
  : probabilityFieldName
  (
  COLON
  | ASSIGN
  )
  expression
  ;

/*

* ============================================================================
* FIELD NAME
* ============================================================================
* 
* No closed probability metadata inventory is required.
* 
* Existing reserved words are accepted where they already exist.
* 
* IDENTIFIER keeps the system extensible.
  */
  probabilityFieldName
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

/*

* ============================================================================
* GIVEN CLAUSE
* ============================================================================
* 
* Conditional probability:
* 
* P(event | condition)
* 
* is represented textually by:
* 
* @probability event given condition;
* 
* or:
* 
* @probability(event, given: condition);
* 
* The word "given" is intentionally NOT a new keyword.
* 
* It is represented as an open identifier-based metadata field:
* 
* given: condition
* 
* for call-style syntax.
* 
* The infix form uses a dedicated structural marker:
* 
* GIVEN
* 
* only if the canonical lexer already provides it.
* 
* To avoid requiring a new keyword, the production form is:
* 
* @probability event given: condition;
* 
* Therefore this grammar uses the canonical named-argument form rather than
* introducing another reserved word.
  /
  probabilityGivenClause
  : LPAREN
  probabilityGivenArgument
  (
  COMMA
  probabilityArgument
  )
  RPAREN
  ;

/*

* ============================================================================
* GIVEN ARGUMENT
* ============================================================================
* 
* given: condition
* 
* "given" remains an ordinary identifier.
  */
  probabilityGivenArgument
  : IDENTIFIER
  (
  COLON
  | ASSIGN
  )
  expression
  ;

/*

* ============================================================================
* METADATA CLAUSE
* ============================================================================
* 
* Additional metadata remains open-ended.
  */
  probabilityMetadataClause
  : COMMA
  probabilityArgument
  ;

/*

* ============================================================================
* OBSERVATION
* ============================================================================
* 
* Canonical form:
* 
* @probability(event, observation: observed_value);
* 
* The grammar deliberately uses metadata rather than introducing a second
* observation language.
  */
  probabilityObservation
  : probabilityValue
  LPAREN
  probabilityObservationArgumentList
  RPAREN
  ;

/*

* ============================================================================
* OBSERVATION ARGUMENT LIST
* ============================================================================
  /
  probabilityObservationArgumentList
  : probabilityArgument
  (
  COMMA
  probabilityArgument
  )
  ;

/*

* ============================================================================
* ASSERTION
* ============================================================================
* 
* Canonical form:
* 
* @probability(event = p);
* 
* or:
* 
* @probability(event, p);
* 
* The semantic layer determines whether "p" is a valid probability.
  */
  probabilityAssertion
  : LPAREN
  probabilityValue
  ASSIGN
  probabilityValue
  RPAREN
  ;

/*

* ============================================================================
* EXPECTATION
* ============================================================================
* 
* Canonical forms:
* 
* @probability expectation(expression);
* 
* @probability(expectation(expression));
* 
* "expectation" remains an identifier.
* 
* This avoids adding a universal EXPECTATION keyword merely for one
* mathematical operation.
  */
  probabilityExpectation
  : expectationCall
  ;

expectationCall
: IDENTIFIER
LPAREN
expression
RPAREN
;

/*

* ============================================================================
* PROBABILITY VALUE
* ============================================================================
* 
* Every probability subject/value is an ordinary Zamani expression.
* 
* This is the critical extensibility boundary.
  */
  probabilityValue
  : expression
  ;