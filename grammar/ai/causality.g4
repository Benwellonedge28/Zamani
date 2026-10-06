/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/causality.g4
 *
 * Grammar:
 *     AICausality
 *
 * Status:
 *     CANONICAL AI-DOMAIN CAUSAL-COMPUTATION COMPOSITION GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97+
 *     Rust edition 2021
 *     Safe Rust only
 *     No unsafe Rust
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
 * This file defines the AI-domain composition boundary for causal
 * computation.
 *
 * Causality is a semantic computational capability. It is not restricted to
 * machine learning and is not tied to a particular causal graph format,
 * statistical model, temporal engine, database, simulator, theorem prover,
 * hardware architecture, quantum implementation, or runtime.
 *
 * The grammar provides source-level structure for:
 *
 *     causal relationships
 *     causes
 *     effects
 *     observations
 *     interventions
 *     counterfactuals
 *     dependencies
 *     causal queries
 *     causal assertions
 *     causal explanations
 *     causal context
 *     causal metadata
 *     extensible causal operations
 *
 * The grammar describes COMPUTATIONAL INTENT.
 *
 * It does not decide how that intent is executed.
 *
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
 *     ZamaniParser
 *          |
 *          v
 *     AI composition
 *          |
 *          v
 *     AICausality                    <-- THIS FILE
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +------------------+------------------+------------------+
 *          |                  |                  |                  |
 *          v                  v                  v                  v
 *        types             effects          capabilities         resources
 *          |                  |                  |                  |
 *          +------------------+------------------+------------------+
 *                                     |
 *                                     v
 *                             semantic causal model
 *                                     |
 *          +------------------+-------+------------------+
 *          |                  |                          |
 *          v                  v                          v
 *      classical          quantum semantic          other domains
 *                              |
 *                              v
 *                          quantum::ir
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                       routing / scheduling
 *                              |
 *                    resilience / recovery
 *                              |
 *                           ZQN / HAL
 *                              |
 *                       target realization
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     aiCausalityConstruct
 *     causalStatement
 *     causalExpression
 *     causalOperation
 *     causalOperationName
 *     causalArgumentList
 *     causalArgument
 *     causalNamedArgument
 *     causalContext
 *     causalContextItem
 *
 * It also provides stable semantic-category parser boundaries:
 *
 *     causalRelation
 *     causalObservation
 *     causalIntervention
 *     causalCounterfactual
 *     causalDependency
 *     causalQuery
 *     causalAssertion
 *     causalExplanation
 *
 * These category rules deliberately share one operation representation.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     keywords
 *     identifiers
 *     qualified-name syntax
 *     expression precedence
 *     expressions
 *     literals
 *     types
 *     patterns
 *     guards
 *     reasoning
 *     inference
 *     deduction
 *     knowledge storage
 *     knowledge query semantics
 *     uncertainty
 *     probability
 *     distributions
 *     evidence storage
 *     provenance storage
 *     contracts
 *     policies
 *     effects
 *     capabilities
 *     resources
 *     temporal syntax
 *     temporal storage
 *     learning algorithms
 *     adaptation algorithms
 *     causal discovery algorithms
 *     causal inference algorithms
 *     Bayesian networks
 *     graph databases
 *     theorem provers
 *     simulation engines
 *     quantum syntax
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     HDL syntax
 *     hardware syntax
 *     distributed actor syntax
 *     networking syntax
 *     FFI
 *     ABI
 *     runtime execution
 *     target selection
 *     optimization
 *     lowering
 *     canonical IR
 *     quantum::ir
 *     ZQN
 *     HAL
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Generic language facilities remain owned by their canonical grammars.
 *
 * In particular this grammar MUST NOT redefine:
 *
 *     expression
 *     typeExpression
 *     identifier
 *     qualifiedName
 *     reasonStatement
 *     knowledgeExpression
 *     uncertaintyExpression
 *     pattern
 *     guard
 *     requirement
 *     capability
 *     contract
 *     policy
 *     provenance
 *     effect
 *
 * Causal computation consumes those facilities through their public
 * interfaces.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 * IMPORTS:
 *
 *     Names
 *     Expressions
 *
 * EXPORTS:
 *
 *     aiCausalityConstruct
 *     causalStatement
 *     causalExpression
 *     causalOperation
 *     causalOperationName
 *     causalArgumentList
 *     causalArgument
 *     causalNamedArgument
 *     causalContext
 *     causalContextItem
 *     causalRelation
 *     causalObservation
 *     causalIntervention
 *     causalCounterfactual
 *     causalDependency
 *     causalQuery
 *     causalAssertion
 *     causalExplanation
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral frontend AST.
 *
 * SEMANTIC_OWNER:
 *
 *     Causal semantic subsystem.
 *
 * SEMANTIC_CONSUMERS:
 *
 *     type analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     contract analysis
 *     policy analysis
 *     provenance
 *     temporal semantics
 *     reasoning
 *     knowledge
 *     uncertainty
 *     concurrency
 *     distributed semantics
 *     simulation
 *     quantum semantics
 *     HDL/hardware semantics
 *
 * IR_OWNER:
 *
 *     Canonical semantic representation.
 *
 *     Classical causal computation uses the canonical classical path.
 *
 *     Quantum-related computation crosses the canonical:
 *
 *         quantum::ir
 *
 *     boundary.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/causality/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/spec/causality.md
 *     grammar/spec/provenance.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *     grammar/spec/policies.md
 *
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The intended dependency direction is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Names / Expressions
 *          |
 *          v
 *     AICausality
 *          |
 *          v
 *     AI
 *          |
 *          v
 *     Zamani parser composition
 *
 * This grammar MUST NOT import AI itself.
 *
 * This grammar MUST NOT import ZamaniParser.
 *
 * This grammar MUST NOT import the complete Statements grammar.
 *
 * This prevents circular parser dependencies.
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * No new global keyword is required for causal computation.
 *
 * The canonical causal namespace is:
 *
 *     causal
 *
 * Canonical operations are conventionally represented as:
 *
 *     causal::cause(...)
 *     causal::effect(...)
 *     causal::observe(...)
 *     causal::intervene(...)
 *     causal::counterfactual(...)
 *     causal::depends_on(...)
 *     causal::query(...)
 *     causal::assert(...)
 *     causal::explain(...)
 *
 * These names remain ordinary names at the lexical layer unless the canonical
 * lexer independently reserves one of them.
 *
 * This deliberately avoids adding a permanent global catalogue such as:
 *
 *     CAUSAL
 *     CAUSE
 *     EFFECT
 *     OBSERVATION
 *     INTERVENTION
 *     COUNTERFACTUAL
 *     DEPENDENCY
 *
 * merely to support this subsystem.
 *
 * A new causal operation therefore does not normally require a lexer change.
 *
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * `qualifiedName` is the canonical name boundary.
 *
 * `causalOperationName` delegates completely to `qualifiedName`.
 *
 * Semantic analysis determines whether a resolved operation belongs to:
 *
 *     causal
 *
 * or to a registered causal extension namespace.
 *
 * Therefore this grammar intentionally permits:
 *
 *     causal::operation(...)
 *
 * and also permits a syntactically valid qualified name which semantic
 * analysis may subsequently reject because it is not a causal operation.
 *
 * This is preferable to duplicating namespace semantics in the parser.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Every causal operand is an ordinary Zamani expression.
 *
 * Consequently causal operations may consume:
 *
 *     scalar values
 *     tuples
 *     records
 *     collections
 *     streams
 *     tensors
 *     datasets
 *     models
 *     knowledge values
 *     uncertain values
 *     probabilistic values
 *     reasoning results
 *     classical computation results
 *     quantum measurement results
 *     simulation results
 *     distributed results
 *     hardware observations
 *     future-domain values
 *
 * This grammar creates no AI-specific expression hierarchy.
 *
 *
 * ============================================================================
 * STATEMENT / EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Causal computation is represented as an operation expression.
 *
 * A standalone causal operation becomes a statement only through:
 *
 *     causalStatement
 *
 * This gives the same semantic operation a reusable expression boundary.
 *
 * Example:
 *
 *     causal::observe(sensor_value)
 *
 * may be embedded in an expression.
 *
 * Example:
 *
 *     causal::observe(sensor_value);
 *
 * is a causal statement.
 *
 * The grammar does not define assignment or general expression precedence.
 *
 *
 * ============================================================================
 * OPEN-WORLD OPERATION CONTRACT
 * ============================================================================
 *
 * The following operations are canonical semantic conventions:
 *
 *     causal::cause
 *     causal::effect
 *     causal::observe
 *     causal::intervene
 *     causal::counterfactual
 *     causal::depends_on
 *     causal::query
 *     causal::assert
 *     causal::explain
 *
 * They are NOT a closed catalogue.
 *
 * A causal provider, library, scientific domain, simulator, compiler
 * extension, or future computational domain may register additional
 * operations.
 *
 * The parser therefore MUST NOT contain one alternative for every causal
 * algorithm.
 *
 * For example, these remain possible semantic extensions:
 *
 *     causal::discover(...)
 *     causal::estimate(...)
 *     causal::identify(...)
 *     causal::simulate(...)
 *     causal::validate(...)
 *     causal::mechanism(...)
 *
 * without changing this grammar.
 *
 *
 * ============================================================================
 * CAUSAL RELATION CONTRACT
 * ============================================================================
 *
 * A causal relation represents a semantic directed relationship between
 * computational entities.
 *
 * The grammar does not force a graph representation.
 *
 * A semantic implementation may represent causal information as:
 *
 *     relations
 *     graphs
 *     rules
 *     equations
 *     models
 *     constraints
 *     symbolic structures
 *     distributed representations
 *     other future representations
 *
 * A causal relation is not automatically:
 *
 *     temporal dependency
 *     data dependency
 *     memory dependency
 *     hardware dependency
 *     network dependency
 *     thread dependency
 *
 * Those meanings belong to their respective semantic systems.
 *
 *
 * ============================================================================
 * OBSERVATION CONTRACT
 * ============================================================================
 *
 * `causal::observe(...)` represents an observation request.
 *
 * An observation may derive from:
 *
 *     ordinary data
 *     sensor data
 *     simulation
 *     hardware
 *     quantum measurement
 *     distributed state
 *     model output
 *     knowledge
 *     external systems
 *
 * The grammar does not guarantee that an observation is physically
 * non-invasive.
 *
 * The semantic layer determines whether the selected operation is truly
 * observational and what effects it has.
 *
 *
 * ============================================================================
 * INTERVENTION CONTRACT
 * ============================================================================
 *
 * `causal::intervene(...)` represents an explicit causal manipulation or
 * hypothetical manipulation.
 *
 * It may refer to:
 *
 *     variables
 *     states
 *     values
 *     relations
 *     models
 *     simulation parameters
 *     control conditions
 *
 * It does not imply a particular:
 *
 *     actuator
 *     CPU
 *     GPU
 *     QPU
 *     FPGA
 *     sensor
 *     hardware device
 *     physical mechanism
 *
 * Authorization and execution are semantic/runtime concerns.
 *
 *
 * ============================================================================
 * COUNTERFACTUAL CONTRACT
 * ============================================================================
 *
 * `causal::counterfactual(...)` represents evaluation of an alternative
 * condition, intervention, state, or world relative to a specified semantic
 * context.
 *
 * It may consume:
 *
 *     observations
 *     interventions
 *     models
 *     evidence
 *     uncertainty
 *     probability
 *     reasoning
 *     simulation
 *
 * The grammar does not prescribe a counterfactual algorithm.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * `causal::depends_on(...)` represents a causal dependency.
 *
 * It must not be confused with the repository's other dependency concepts.
 *
 * In particular, causal dependency does not automatically mean:
 *
 *     execution dependency
 *     resource dependency
 *     module dependency
 *     memory dependency
 *     data dependency
 *     network dependency
 *     scheduling dependency
 *
 * Semantic analysis determines the intended relation.
 *
 *
 * ============================================================================
 * QUERY CONTRACT
 * ============================================================================
 *
 * `causal::query(...)` represents a request for causal information.
 *
 * Query semantics may be backed by:
 *
 *     causal models
 *     knowledge systems
 *     datasets
 *     symbolic rules
 *     statistical systems
 *     simulation
 *     distributed systems
 *     external providers
 *
 * This grammar does not create a separate query language.
 *
 *
 * ============================================================================
 * ASSERTION CONTRACT
 * ============================================================================
 *
 * `causal::assert(...)` represents a causal claim or causal assertion.
 *
 * The operation may reference:
 *
 *     a claim
 *     evidence
 *     observations
 *     interventions
 *     model information
 *     provenance
 *     confidence
 *     uncertainty
 *
 * General assertion/contract syntax remains owned by the validation system.
 *
 * A causal assertion operation must therefore not be confused with the
 * universal assertion/contract grammar.
 *
 *
 * ============================================================================
 * EXPLANATION CONTRACT
 * ============================================================================
 *
 * `causal::explain(...)` represents a request for a causal explanation.
 *
 * Explanations may be consumed by:
 *
 *     AI tooling
 *     scientific tooling
 *     compiler diagnostics
 *     optimization analysis
 *     resource planning
 *     quantum analysis
 *     hardware analysis
 *     security analysis
 *
 * The grammar does not prescribe the explanation representation.
 *
 *
 * ============================================================================
 * ARGUMENT CONTRACT
 * ============================================================================
 *
 * Causal operations accept zero or more arguments syntactically.
 *
 * Semantic operation metadata determines:
 *
 *     required arity
 *     optional arity
 *     argument roles
 *     argument types
 *     ordering constraints
 *     required metadata
 *     incompatible metadata
 *
 * This allows the grammar to remain stable while semantic operation
 * definitions evolve.
 *
 * A semantic implementation may reject an operation whose arguments do not
 * satisfy its registered contract.
 *
 *
 * ============================================================================
 * NAMED ARGUMENT CONTRACT
 * ============================================================================
 *
 * Named arguments provide extensible causal metadata without introducing a
 * new grammar rule for every future causal property.
 *
 * Canonical examples include:
 *
 *     evidence: e
 *     provenance: p
 *     context: c
 *     model: m
 *     policy: policy_value
 *     confidence: confidence_value
 *
 * The canonical parser representation uses:
 *
 *     identifier ASSIGN expression
 *
 * where the semantic layer determines the meaning of the field.
 *
 * The grammar does not create a fixed metadata catalogue.
 *
 *
 * ============================================================================
 * CONTEXT CONTRACT
 * ============================================================================
 *
 * `causalContext` is a reusable parser boundary for operation contexts.
 *
 * Context items may be:
 *
 *     expressions
 *
 * or:
 *
 *     named arguments
 *
 * This permits context such as:
 *
 *     causal::cause(a, b, context: model);
 *
 *     causal::intervene(x, value, policy: intervention_policy);
 *
 *     causal::counterfactual(condition, outcome, evidence: evidence);
 *
 * Semantic analysis determines which context fields are valid.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST remains domain-neutral.
 *
 * The AST must preserve at least:
 *
 *     source span
 *     qualified operation name
 *     ordered arguments
 *     named argument names
 *     argument expressions
 *     syntactic statement/expression context
 *
 * A semantic normalization may produce a generic operation representation
 * equivalent to:
 *
 *     Operation {
 *         name
 *         namespace
 *         operands
 *         parameters
 *         results
 *         attributes
 *         modifiers
 *         effects
 *         capabilities
 *         source
 *     }
 *
 * The exact AST type is owned by the existing frontend.
 *
 * This grammar MUST NOT introduce:
 *
 *     AICausalNode
 *     CauseNode
 *     EffectNode
 *     ObservationNode
 *     InterventionNode
 *     CounterfactualNode
 *     CausalGraphNode
 *
 * merely because causal syntax was encountered.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. resolve the operation name;
 *     2. validate its causal namespace;
 *     3. resolve registered causal operation metadata;
 *     4. validate argument arity;
 *     5. validate argument roles;
 *     6. validate argument types;
 *     7. normalize named arguments;
 *     8. determine semantic operation category;
 *     9. determine effects;
 *    10. determine capabilities;
 *    11. determine resource requirements;
 *    12. evaluate applicable contracts;
 *    13. evaluate applicable policies;
 *    14. attach evidence/provenance where required;
 *    15. determine determinism/reproducibility properties;
 *    16. normalize into the canonical semantic representation.
 *
 * The parser MUST NOT perform these operations.
 *
 *
 * ============================================================================
 * OPERATION REGISTRY CONTRACT
 * ============================================================================
 *
 * Causal operation semantics should be registered through semantic metadata.
 *
 * The registry should contain information such as:
 *
 *     qualified name
 *     category
 *     argument schema
 *     type requirements
 *     effect requirements
 *     capability requirements
 *     resource requirements
 *     contract requirements
 *     policy requirements
 *     provenance requirements
 *     determinism properties
 *     supported execution modes
 *     supported domain interactions
 *
 * The registry is semantic data.
 *
 * It is NOT a grammar-level finite catalogue.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Causal operands use ordinary Zamani expressions.
 *
 * Causal semantics may therefore operate on values from:
 *
 *     classical computation
 *     numerical computation
 *     tensors
 *     datasets
 *     graphs
 *     knowledge
 *     uncertainty
 *     probability
 *     models
 *     quantum measurements
 *     quantum states where semantically valid
 *     simulation
 *     distributed computation
 *     hardware observations
 *     HDL simulation
 *     future domains
 *
 * Type compatibility is checked by the canonical type system.
 *
 * This grammar defines no causal type hierarchy.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Causal syntax itself is effect-neutral.
 *
 * Semantic analysis may derive effects such as:
 *
 *     observation
 *     mutation
 *     simulation
 *     randomness
 *     network
 *     distributed
 *     measurement
 *     native
 *     foreign
 *     reflection
 *     learning
 *     adaptation
 *
 * according to the actual operation and its resolved implementation.
 *
 * For example:
 *
 *     causal::observe(measurement)
 *
 * may inherit a measurement effect from the operand.
 *
 *     causal::intervene(target, value)
 *
 * may require a mutation or simulation effect.
 *
 * The grammar does not infer either effect.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements are semantic.
 *
 * Possible capabilities include:
 *
 *     causal.observation
 *     causal.intervention
 *     causal.counterfactual
 *     causal.inference
 *     causal.discovery
 *     causal.simulation
 *
 * These are examples of semantic capability identities, not a closed list.
 *
 * The capability system determines whether a realization can provide the
 * requested capability.
 *
 * This grammar does not inspect hardware.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Causal computation may consume arbitrary:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     accelerator resources
 *     distributed resources
 *     quantum resources
 *     model resources
 *     simulation resources
 *
 * Resource requirements belong to the canonical resource system.
 *
 * This file introduces no resource grammar.
 *
 * There is no grammar-level limit on:
 *
 *     causal relations
 *     observations
 *     interventions
 *     counterfactuals
 *     dependencies
 *     variables
 *     models
 *     evidence items
 *     context items
 *     graph size
 *     graph depth
 *     graph width
 *     processors
 *     threads
 *     devices
 *     nodes
 *     qubits
 *     tensor dimensions
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Causal operations may participate in the universal contract model:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * Contract syntax remains owned by the validation subsystem.
 *
 * This grammar does not duplicate contract productions.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Causal computation may be controlled by:
 *
 *     security policy
 *     privacy policy
 *     evidence policy
 *     provenance policy
 *     resource policy
 *     execution policy
 *     model policy
 *     adaptation policy
 *     reproducibility policy
 *
 * Policy enforcement occurs downstream.
 *
 * Parsing a causal operation never grants authorization.
 *
 *
 * ============================================================================
 * EVIDENCE CONTRACT
 * ============================================================================
 *
 * Evidence is represented as ordinary expressions.
 *
 * Causal operations may therefore consume evidence from:
 *
 *     datasets
 *     observations
 *     knowledge
 *     reasoning
 *     simulations
 *     classical computation
 *     quantum measurement
 *     hardware
 *     distributed systems
 *     external sources
 *
 * The evidence subsystem owns evidence semantics.
 *
 * This grammar does not create a second evidence representation.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Causal claims and derived results may require provenance.
 *
 * Provenance may record:
 *
 *     source
 *     observation
 *     intervention
 *     evidence
 *     model
 *     derivation
 *     transformation
 *     verification
 *     policy
 *     decision
 *     version
 *     source span
 *
 * Provenance storage and semantics remain owned by the canonical provenance
 * subsystem.
 *
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Causal computation and generic reasoning are complementary.
 *
 *     reasoning
 *         asks:
 *             what follows from premises/evidence?
 *
 *     causality
 *         represents:
 *             causal relationships and interventions.
 *
 * Causal results may be consumed by:
 *
 *     infer
 *     deduce
 *     reason
 *
 * through ordinary expression composition.
 *
 * This grammar does not import or duplicate:
 *
 *     grammar/statements/reason.g4
 *
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Causal operations may consume or produce knowledge values.
 *
 * For example, semantically:
 *
 *     causal::query(knowledge_value)
 *
 * or:
 *
 *     causal::assert(claim, evidence: knowledge_value)
 *
 * may be valid.
 *
 * The knowledge grammar remains independently owned.
 *
 * This file intentionally does not import the knowledge grammar, preventing
 * unnecessary dependency cycles.
 *
 *
 * ============================================================================
 * UNCERTAINTY / PROBABILITY INTEGRATION
 * ============================================================================
 *
 * Causal operands may contain:
 *
 *     uncertainty expressions
 *     probabilities
 *     distributions
 *     confidence values
 *     beliefs
 *     likelihoods
 *
 * These are consumed through the ordinary expression boundary.
 *
 * No probability or uncertainty syntax is duplicated here.
 *
 * Semantic analysis determines whether uncertainty is valid for a particular
 * causal operation.
 *
 *
 * ============================================================================
 * TEMPORAL INTEGRATION
 * ============================================================================
 *
 * Causality is not equivalent to temporal ordering.
 *
 * A causal relationship MAY contain temporal information, but temporal syntax
 * remains owned by the temporal subsystem.
 *
 * The causal semantic model may therefore be consumed by:
 *
 *     temporal semantics
 *     MTS semantics
 *     memory semantics
 *     concurrency ordering
 *     distributed ordering
 *
 * This grammar does not import temporal grammars.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Causal expressions may consume quantum-derived values, including results of
 * quantum measurement and hybrid computation.
 *
 * Example semantic composition:
 *
 *     causal::observe(measurement_result)
 *
 * or:
 *
 *     causal::counterfactual(
 *         condition,
 *         quantum_derived_outcome,
 *         context: model
 *     )
 *
 * This grammar does NOT define quantum operations.
 *
 * It does NOT define:
 *
 *     qubits
 *     gates
 *     circuits
 *     physical qubits
 *     coupling maps
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *
 * When causal semantics require quantum computation, the canonical boundary is:
 *
 *     causal semantic model
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
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
 * No causal-specific quantum IR is permitted.
 *
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Causal computation can participate in hybrid execution:
 *
 *     classical computation
 *          |
 *          v
 *     causal operation
 *          |
 *          v
 *     quantum operation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     causal/reasoning decision
 *
 * The grammar does not encode the physical execution path.
 *
 * Hybrid orchestration belongs to the semantic and execution layers.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Causal operands may represent:
 *
 *     HDL simulation values
 *     hardware observations
 *     control values
 *     timing observations
 *     sensor values
 *     resource observations
 *     verification results
 *
 * This grammar does not define:
 *
 *     wire widths
 *     register widths
 *     fixed clock counts
 *     fixed device counts
 *     fixed FPGA resources
 *     physical pins
 *     placement
 *     routing
 *     synthesis
 *
 * Hardware realization remains downstream.
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Causal relationships may span:
 *
 *     actors
 *     processes
 *     services
 *     nodes
 *     channels
 *     distributed state
 *     network observations
 *
 * This grammar does not own actor, message, channel, topology, scheduling or
 * consensus syntax.
 *
 * Distributed semantics may attach those meanings after parsing.
 *
 *
 * ============================================================================
 * AI / LEARNING INTEGRATION
 * ============================================================================
 *
 * Causal operations may consume learned models and learning results.
 *
 * Learning algorithms remain outside this grammar.
 *
 * The same causal syntax can therefore work with:
 *
 *     symbolic models
 *     learned models
 *     hybrid models
 *     statistical models
 *     simulation models
 *     domain-defined models
 *
 * No model-family catalogue is encoded.
 *
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * An adaptation operation may semantically use causal information to select
 * another model, strategy, policy, or execution path.
 *
 * Causality does not authorize adaptation.
 *
 * Adaptation remains governed by:
 *
 *     policy
 *     capabilities
 *     effects
 *     contracts
 *     resources
 *     provenance
 *
 * through the existing adaptation subsystem.
 *
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Counterfactual and intervention operations may use simulation.
 *
 * Simulation may represent:
 *
 *     classical systems
 *     quantum systems
 *     hardware
 *     HDL
 *     distributed systems
 *     AI models
 *     future domains
 *
 * Simulation is an execution strategy, not a second source language.
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY NEGOTIATION
 * ============================================================================
 *
 * Source-level causal intent must remain independent of target selection.
 *
 * For example, semantic analysis may derive:
 *
 *     requires capability("causal.counterfactual")
 *
 * or:
 *
 *     requires capability("causal.intervention")
 *
 * but the source grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     device
 *     node
 *     physical qubit
 *
 * Target feasibility is evaluated after semantic analysis.
 *
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Causal syntax MUST NOT encode a finite machine or causal-model universe.
 *
 * In particular, this file contains no limits for:
 *
 *     causal nodes
 *     causal edges
 *     observations
 *     interventions
 *     counterfactuals
 *     dependencies
 *     variables
 *     models
 *     evidence
 *     context
 *     graph depth
 *     graph width
 *     tensor rank
 *     tensor dimensions
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     memory
 *     storage
 *     network size
 *
 * No capacity constants or equivalent finite grammar alternatives are allowed.
 *
 * Numeric literals in source remain program values.
 *
 * For example:
 *
 *     causal::intervene(x, 1024);
 *
 * does not make 1024 a language-level capacity.
 *
 * "Scale to infinity given resources" means:
 *
 *     the source language introduces no artificial universal ceiling.
 *
 * It does not claim that finite physical implementations have infinite
 * resources.
 *
 * Resource exhaustion, target infeasibility, implementation limits and
 * policy restrictions are downstream conditions.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PROHIBITED:
 *
 *     MAX_CAUSAL_NODES
 *     MAX_CAUSAL_EDGES
 *     MAX_OBSERVATIONS
 *     MAX_INTERVENTIONS
 *     MAX_COUNTERFACTUALS
 *     MAX_DEPENDENCIES
 *     MAX_CAUSAL_DEPTH
 *     MAX_CAUSAL_WIDTH
 *     MAX_CAUSAL_GRAPH_SIZE
 *     MAX_MODELS
 *     MAX_VARIABLES
 *     MAX_EVIDENCE
 *
 * Also prohibited are equivalent indirect parser ceilings.
 *
 * This grammar contains none.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     lexical grammar
 *     parser grammar
 *     selected language/compatibility configuration
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *     runtime state
 *     resource availability
 *
 * For identical lexical/parser configuration and identical source text,
 * parsing must be deterministic.
 *
 * Semantic execution may be nondeterministic because of:
 *
 *     randomness
 *     external observations
 *     distributed execution
 *     probabilistic computation
 *     quantum measurement
 *     hardware behavior
 *     model behavior
 *
 * Such nondeterminism belongs to semantic/effect/provenance analysis.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     intervention
 *     observation
 *     device access
 *     filesystem access
 *     network access
 *     foreign invocation
 *     native invocation
 *     secret access
 *     hardware discovery
 *     policy enforcement
 *
 * It contains no embedded Rust actions or semantic predicates.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are limited to syntactic failures.
 *
 * Semantic diagnostics should distinguish:
 *
 *     CAUSAL_NAMESPACE_INVALID
 *     CAUSAL_OPERATION_UNKNOWN
 *     CAUSAL_ARGUMENT_ARITY_INVALID
 *     CAUSAL_ARGUMENT_ROLE_INVALID
 *     CAUSAL_ARGUMENT_TYPE_MISMATCH
 *     CAUSAL_NAMED_ARGUMENT_UNKNOWN
 *     CAUSAL_NAMED_ARGUMENT_DUPLICATE
 *     CAUSAL_INTERVENTION_NOT_ALLOWED
 *     CAUSAL_COUNTERFACTUAL_UNSUPPORTED
 *     CAUSAL_CAPABILITY_UNAVAILABLE
 *     CAUSAL_RESOURCE_UNSATISFIED
 *     CAUSAL_CONTRACT_VIOLATION
 *     CAUSAL_POLICY_VIOLATION
 *     CAUSAL_PROVENANCE_REQUIRED
 *     CAUSAL_EVIDENCE_INVALID
 *     CAUSAL_DEPENDENCY_INVALID
 *     CAUSAL_TARGET_INCOMPATIBLE
 *
 * These are semantic diagnostic identities.
 *
 * They are not lexer tokens.
 *
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AICausality;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
    ;


/*
 * ============================================================================
 * PUBLIC AI ENTRY
 * ============================================================================
 *
 * This is the only public entry into this grammar from AI composition.
 *
 * ============================================================================
 */

aiCausalityConstruct
    : causalStatement
    | causalExpression
    ;


/*
 * ============================================================================
 * CAUSAL STATEMENT
 * ============================================================================
 *
 * A causal operation may appear as a standalone statement.
 *
 * Examples:
 *
 *     causal::cause(a, b);
 *     causal::observe(value);
 *     causal::intervene(variable, value);
 *     causal::counterfactual(condition, outcome);
 *
 * ============================================================================
 */

causalStatement
    : causalOperation SEMICOLON
    ;


/*
 * ============================================================================
 * CAUSAL EXPRESSION
 * ============================================================================
 *
 * A causal operation is also reusable as an expression.
 *
 * ============================================================================
 */

causalExpression
    : causalOperation
    ;


/*
 * ============================================================================
 * CAUSAL OPERATION
 * ============================================================================
 *
 * The parser accepts an open qualified operation name.
 *
 * Semantic validation determines whether the resolved name belongs to the
 * canonical causal namespace or a registered causal extension namespace.
 *
 * ============================================================================
 */

causalOperation
    : causalOperationName
      LEFT_PAREN
      causalArgumentList?
      RIGHT_PAREN
    ;


causalOperationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * ARGUMENT LIST
 * ============================================================================
 *
 * No trailing comma is accepted.
 *
 * The number of arguments is not fixed by this grammar.
 *
 * ============================================================================
 */

causalArgumentList
    : causalArgument
      (
          COMMA
          causalArgument
      )*
    ;


causalArgument
    : causalNamedArgument
    | expression
    ;


/*
 * ============================================================================
 * NAMED ARGUMENT
 * ============================================================================
 *
 * Named causal metadata remains open-world.
 *
 * Example:
 *
 *     evidence: e
 *     provenance: p
 *     context: model
 *
 * The semantic operation registry owns the meaning.
 *
 * ============================================================================
 */

causalNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * REUSABLE CONTEXT
 * ============================================================================
 *
 * This rule is intentionally independent of causalOperation so semantic tools
 * may consume the context boundary without creating another operation syntax.
 *
 * ============================================================================
 */

causalContext
    : LEFT_PAREN
      causalContextItem
      (
          COMMA
          causalContextItem
      )*
      RIGHT_PAREN
    ;


causalContextItem
    : causalNamedArgument
    | expression
    ;


/*
 * ============================================================================
 * SEMANTIC CATEGORY BOUNDARIES
 * ============================================================================
 *
 * These rules do not create independent syntaxes.
 *
 * They all delegate to the canonical causal operation representation.
 *
 * Semantic analysis determines the actual category from the resolved
 * operation metadata.
 * ============================================================================
 */

causalRelation
    : causalOperation
    ;


causalObservation
    : causalOperation
    ;


causalIntervention
    : causalOperation
    ;


causalCounterfactual
    : causalOperation
    ;


causalDependency
    : causalOperation
    ;


causalQuery
    : causalOperation
    ;


causalAssertion
    : causalOperation
    ;


causalExplanation
    : causalOperation
    ;


/*
 * ============================================================================
 * CAUSAL RELATION OPERATOR BOUNDARY
 * ============================================================================
 *
 * The canonical causal representation uses named operations rather than
 * introducing another relation-operator language.
 *
 * This reusable boundary therefore delegates to the canonical name system.
 *
 * It exists for tooling and future source forms, but does not create an
 * additional parser path for causal relations.
 * ============================================================================
 */

causalRelationOperator
    : qualifiedName
    ;


/*
 * ============================================================================
 * EXAMPLES OF CANONICAL SEMANTIC FORMS
 * ============================================================================
 *
 * The following illustrate intended semantic operations:
 *
 *     causal::cause(a, b);
 *     causal::effect(a, b);
 *     causal::observe(observation);
 *     causal::intervene(variable, value);
 *     causal::counterfactual(condition, outcome);
 *     causal::depends_on(result, dependency);
 *     causal::query(target);
 *     causal::assert(claim, evidence: evidence_value);
 *     causal::explain(decision, evidence: evidence_value);
 *
 * Extended:
 *
 *     causal::intervene(
 *         variable,
 *         value,
 *         policy: intervention_policy
 *     );
 *
 *     causal::counterfactual(
 *         condition,
 *         outcome,
 *         evidence: evidence_value,
 *         context: model
 *     );
 *
 * These examples are documentation of semantic conventions.
 *
 * They are not a closed grammar catalogue.
 *
 *
 * ============================================================================
 * COMPLETENESS / INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] AICausality is the unique grammar identity for this AI causal adapter.
 *
 * [ ] tokenVocab is ZamaniLexer.
 *
 * [ ] No lexer rules exist in this file.
 *
 * [ ] Names is the canonical name dependency.
 *
 * [ ] Expressions is the canonical expression dependency.
 *
 * [ ] aiCausalityConstruct is the sole AI-facing causal entry.
 *
 * [ ] causalStatement delegates to causalOperation.
 *
 * [ ] causalExpression delegates to causalOperation.
 *
 * [ ] causalOperation is open-world.
 *
 * [ ] causalOperationName delegates to qualifiedName.
 *
 * [ ] Named arguments use canonical identifier/expression boundaries.
 *
 * [ ] No fixed causal-operation catalogue is encoded.
 *
 * [ ] No application-specific operation catalogue is encoded.
 *
 * [ ] No duplicate reasoning grammar exists.
 *
 * [ ] No duplicate knowledge grammar exists.
 *
 * [ ] No duplicate uncertainty grammar exists.
 *
 * [ ] No duplicate evidence grammar exists.
 *
 * [ ] No duplicate provenance grammar exists.
 *
 * [ ] No duplicate contract grammar exists.
 *
 * [ ] No duplicate policy grammar exists.
 *
 * [ ] No duplicate resource grammar exists.
 *
 * [ ] No duplicate capability grammar exists.
 *
 * [ ] No duplicate type hierarchy exists.
 *
 * [ ] No duplicate expression hierarchy exists.
 *
 * [ ] No causal-specific IR exists.
 *
 * [ ] Quantum semantics retain the quantum::ir boundary.
 *
 * [ ] No physical hardware assumptions exist.
 *
 * [ ] No universal machine-capacity limit exists.
 *
 * [ ] No causal graph capacity limit exists.
 *
 * [ ] No embedded Rust exists.
 *
 * [ ] No unsafe implementation is required.
 *
 * [ ] Rust 1.97+ compatibility is preserved.
 *
 * [ ] Parser behavior is deterministic.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Diagnostics tests exist.
 *
 *
 * ============================================================================
 * REQUIRED REPOSITORY INTEGRATION
 * ============================================================================
 *
 * This file is independently complete at the grammar boundary, but the
 * following repository integration must be performed once.
 *
 *
 * 1. grammar/ai/ai.g4
 *
 * Add the canonical import:
 *
 *     AICausality
 *
 * to the existing AI composition imports.
 *
 * Add:
 *
 *     | aiCausalityConstruct
 *
 * to:
 *
 *     aiConstruct
 *
 * This makes causality reachable from the canonical AI grammar.
 *
 *
 * 2. grammar/spec/causality.md
 *
 * Create the normative semantic specification.
 *
 * It must define:
 *
 *     causal namespace
 *     operation registration
 *     relation semantics
 *     observation semantics
 *     intervention semantics
 *     counterfactual semantics
 *     dependency semantics
 *     query semantics
 *     assertion semantics
 *     explanation semantics
 *     argument roles
 *     named metadata
 *     type rules
 *     effect rules
 *     capability rules
 *     resource rules
 *     contract interaction
 *     policy interaction
 *     evidence interaction
 *     provenance requirements
 *     determinism
 *     compatibility
 *     diagnostics
 *     IR lowering
 *
 *
 * 3. grammar/spec/ai.md
 *
 * Add causality to the AI semantic integration matrix.
 *
 * It must state that causality is an AI composition capability while its
 * underlying semantic model remains reusable by non-AI domains.
 *
 *
 * 4. grammar/tests/ai/causality/
 *
 * Create/maintain:
 *
 *     relation/
 *     observation/
 *     intervention/
 *     counterfactual/
 *     dependency/
 *     query/
 *     assertion/
 *     explanation/
 *     context/
 *     namespace/
 *     negative/
 *     scalability/
 *     quantum/
 *     hybrid/
 *     temporal/
 *     distributed/
 *     provenance/
 *     determinism/
 *
 *
 * 5. Semantic operation registry
 *
 * Register the initial semantic conventions:
 *
 *     causal::cause
 *     causal::effect
 *     causal::observe
 *     causal::intervene
 *     causal::counterfactual
 *     causal::depends_on
 *     causal::query
 *     causal::assert
 *     causal::explain
 *
 * The registry must remain open to additional operations.
 *
 * The grammar must not be regenerated merely because a new semantic causal
 * operation is registered.
 *
 *
 * 6. Evidence integration
 *
 * Causal operations consume evidence through ordinary expressions and the
 * canonical evidence semantic model.
 *
 * No causal-specific evidence AST or evidence store is permitted.
 *
 *
 * 7. Provenance integration
 *
 * Derived causal claims should preserve source provenance according to the
 * canonical provenance specification.
 *
 *
 * 8. Reasoning integration
 *
 * Causal values must remain usable by the canonical reasoning subsystem:
 *
 *     grammar/statements/reason.g4
 *
 * No causal-specific reasoning syntax is required.
 *
 *
 * 9. Knowledge integration
 *
 * Causal values must remain composable with the canonical knowledge subsystem.
 *
 * No causal-specific knowledge storage is required.
 *
 *
 * 10. Uncertainty integration
 *
 * Causal operands must remain ordinary expressions so they can consume the
 * canonical uncertainty subsystem without grammar duplication.
 *
 *
 * 11. Quantum integration
 *
 * Quantum-derived causal operations must lower through the canonical quantum
 * semantic pipeline and ultimately:
 *
 *     quantum::ir
 *
 * No causal quantum IR is permitted.
 *
 *
 * 12. Resource/capability integration
 *
 * Semantic causal operations may produce resource requirements and capability
 * requirements.
 *
 * Those requirements must be represented by the existing resource and
 * capability systems.
 *
 *
 * 13. Effects integration
 *
 * Semantic causal operations must participate in the existing effect system.
 *
 *
 * 14. Contract/policy integration
 *
 * Causal operations must be evaluable under the existing contracts and policy
 * systems.
 *
 *
 * 15. Rust integration
 *
 * The grammar requires no embedded Rust.
 *
 * Generated frontend/compiler implementation must remain:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * No unsafe Rust is required or permitted.
 *
 *
 * ============================================================================
 * REQUIRED TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     causal::cause(a, b);
 *     causal::effect(a, b);
 *     causal::observe(x);
 *     causal::intervene(x, value);
 *     causal::counterfactual(condition, outcome);
 *     causal::depends_on(result, dependency);
 *     causal::query(target);
 *     causal::assert(claim, evidence: evidence_value);
 *     causal::explain(decision, evidence: evidence_value);
 *
 *
 * EXTENDED
 * --------
 *
 *     causal::intervene(
 *         variable,
 *         value,
 *         policy: intervention_policy
 *     );
 *
 *     causal::counterfactual(
 *         condition,
 *         outcome,
 *         evidence: evidence_value,
 *         context: model
 *     );
 *
 *
 * COMPOSITION
 * -----------
 *
 *     causal::observe(measurement_result);
 *
 *     causal::query(knowledge_result);
 *
 *     causal::counterfactual(
 *         uncertain(condition),
 *         result
 *     );
 *
 *     causal::explain(
 *         decision,
 *         provenance: provenance_value
 *     );
 *
 *
 * NEGATIVE SYNTAX
 * ---------------
 *
 * Reject malformed forms such as:
 *
 *     causal::observe(;
 *
 *     causal::observe(x;
 *
 *     causal::observe(x,);
 *
 *     causal::intervene(x,);
 *
 *     causal::query(,x);
 *
 *     causal::cause(x,,y);
 *
 *     causal::operation(x y);
 *
 * These are parser errors.
 *
 *
 * NEGATIVE SEMANTICS
 * ------------------
 *
 * Syntax may succeed while semantic analysis rejects:
 *
 *     unrelated::operation(x);
 *
 * when reached through the causal AI boundary and no causal extension
 * namespace is registered.
 *
 * Semantic analysis may also reject:
 *
 *     causal::intervene()
 *
 *     causal::counterfactual()
 *
 *     causal::unknown_operation(x)
 *
 * according to the registered semantic operation contracts.
 *
 *
 * SCALABILITY
 * ----------
 *
 * Tests must demonstrate that syntax does not depend on:
 *
 *     number of causal relations
 *     number of observations
 *     number of interventions
 *     number of counterfactuals
 *     number of variables
 *     number of models
 *     graph size
 *     graph depth
 *     graph width
 *     number of processors
 *     number of devices
 *     number of qubits
 *     tensor dimensions
 *     machine memory capacity
 *
 * Test generators may produce arbitrarily large valid operation sequences
 * subject only to actual implementation resource availability.
 *
 *
 * CROSS-DOMAIN
 * -----------
 *
 * Required integration cases include:
 *
 *     classical + causal
 *     AI + causal
 *     knowledge + causal
 *     reasoning + causal
 *     uncertainty + causal
 *     quantum measurement + causal
 *     hybrid + causal
 *     HDL simulation + causal
 *     distributed state + causal
 *     hardware observation + causal
 *     simulation + causal
 *     contracts + causal
 *     policies + causal
 *     provenance + causal
 *     resource requirements + causal
 *
 *
 * DETERMINISM
 * -----------
 *
 * Identical source and parser configuration must produce identical parse
 * structure and source spans.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Causality describes semantic intent.
 *
 * It does not describe a physical machine.
 *
 * The final architecture remains:
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
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     causal semantic model
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *   classical            quantum semantic model
 *                            |
 *                            v
 *                        quantum::ir
 *                            |
 *                            v
 *                     optimization
 *                            |
 *                         lowering
 *                            |
 *                   routing / scheduling
 *                            |
 *                    resilience / recovery
 *                            |
 *                         ZQN / HAL
 *                            |
 *                     target realization
 *
 * The same causal source meaning must remain independent of:
 *
 *     machine size
 *     processor count
 *     accelerator count
 *     QPU size
 *     node count
 *     memory capacity
 *     network size
 *     physical topology
 *
 * POCO-REAF is therefore achieved through stable semantics, open-world
 * operations, canonical IR boundaries, explicit resource/capability
 * negotiation, policy-controlled realization, and target-independent source
 * meaning.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */