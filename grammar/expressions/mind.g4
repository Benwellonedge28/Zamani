/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/expressions/mind.g4
 *
 * GRAMMAR
 * -------
 * MindExpressions
 *
 * STATUS
 * ------
 * CANONICAL EXPRESSION-LEVEL COGNITIVE COMPOSITION GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust:
 *     1.97 / 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * Parser:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the EXPRESSION-LEVEL COGNITIVE COMPOSITION boundary.
 *
 * It provides a universal expression form for composing cognitive
 * computation without creating a second programming language.
 *
 * The construct represented here may combine:
 *
 *     reasoning
 *     knowledge
 *     learning
 *     adaptation
 *     uncertainty
 *     evidence
 *     explanation
 *     decision
 *     provenance
 *     policy
 *     constraints
 *     capabilities
 *     resource requirements
 *     model references
 *     memory references
 *     agent/mind references
 *     classical computation
 *     quantum-derived values
 *     hybrid computation
 *     distributed computation
 *     simulation
 *     interoperability
 *
 * This file DOES NOT implement those subsystems.
 *
 * It provides their common expression-level composition boundary.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * There must be ONE language.
 *
 * There must NOT be:
 *
 *     - a separate AI language;
 *     - a separate cognitive language;
 *     - a separate reasoning language;
 *     - a separate learning language;
 *     - a separate quantum-AI language;
 *     - a separate agent language.
 *
 * Instead:
 *
 *     Zamani expression
 *          |
 *          v
 *     cognitive composition
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *     reasoning           knowledge
 *          |                  |
 *          +--------+---------+
 *                   |
 *                   v
 *              learning
 *                   |
 *                   v
 *              adaptation
 *                   |
 *                   v
 *              decision
 *                   |
 *                   v
 *          effects/capabilities/
 *          resources/contracts/
 *          policies/provenance
 *                   |
 *                   v
 *          canonical semantic model
 *
 * Domain-specific realization occurs downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     mindExpression
 *     mindComposition
 *     mindOperation
 *     mindOperationName
 *     mindOperand
 *     mindArgumentList
 *     mindArgument
 *     mindOptionList
 *     mindOption
 *     mindBinding
 *     mindCondition
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - mind declarations;
 *     - mind regions;
 *     - AI model declarations;
 *     - training;
 *     - inference implementation;
 *     - reasoning algorithms;
 *     - knowledge storage;
 *     - memory implementation;
 *     - agent lifecycle;
 *     - actor lifecycle;
 *     - policies;
 *     - contracts;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - provenance storage;
 *     - quantum operations;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - backend selection;
 *     - runtime execution.
 *
 * Those concerns remain owned by their existing subsystems.
 *
 * ============================================================================
 * EXISTING MIND ARCHITECTURE
 * ============================================================================
 *
 * `grammar/ai/mind.g4` remains the owner of:
 *
 *     - mindConstruct
 *     - mindDeclaration
 *     - mindBinding
 *     - mindInvocation
 *     - mindRegion
 *     - mind-local members
 *     - cognitive regions
 *     - mind declarations
 *
 * This file MUST NOT redefine those constructs.
 *
 * The relationship is:
 *
 *     grammar/ai/mind.g4
 *             |
 *             | declaration / region semantics
 *             v
 *        cognitive entity
 *             |
 *             ^
 *             |
 *     grammar/expressions/mind.g4
 *             |
 *             | expression-level operation
 *             v
 *       cognitive computation
 *
 * A mind is therefore not automatically an agent, process, actor, model,
 * database, or runtime.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical identifier grammar
 *     canonical qualified-name grammar
 *     canonical literal grammar
 *     canonical expression-core boundary
 *
 * EXPORTS:
 *
 *     mindExpression
 *     mindComposition
 *     mindOperation
 *
 * CONSUMED_BY:
 *
 *     grammar/expressions/expressions.g4
 *     canonical parser composition
 *     AI semantic integration
 *     hybrid semantic integration
 *     cognitive-expression tests
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     canonical cognitive/semantic analysis layer
 *
 * EFFECT_OWNER:
 *
 *     grammar/effects/ and corresponding semantic effect system
 *
 * CAPABILITY_OWNER:
 *
 *     grammar/resources/ and corresponding capability subsystem
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/ and corresponding resource subsystem
 *
 * CONTRACT_OWNER:
 *
 *     grammar/validation/
 *
 * POLICY_OWNER:
 *
 *     grammar/expressions/policy.g4
 *     grammar/security/
 *     grammar/policies/ when established
 *
 * PROVENANCE_OWNER:
 *
 *     grammar/expressions/provenance.g4
 *     grammar/data/provenance.g4
 *     grammar/compile/provenance.g4
 *     canonical provenance semantic subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     classical IR where applicable
 *     quantum::ir for quantum computation
 *
 * TEST_OWNER:
 *
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/ai/
 *     grammar/tests/hybrid/
 *     grammar/tests/quantum/
 *     grammar/tests/scalability/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/spec/policies.md
 *     grammar/spec/provenance.md
 *     grammar/specification/ai.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * No cognitive-specific lexer vocabulary is required for the operation names
 * below.
 *
 * Operation names remain identifiers and are resolved by semantic registries.
 *
 * Examples:
 *
 *     reason
 *     infer
 *     deduce
 *     learn
 *     adapt
 *     explain
 *     decide
 *     query
 *     assert
 *     retract
 *
 * remain extensible semantic operation names.
 *
 * This avoids forcing every future cognitive capability into the universal
 * lexer.
 *
 * ============================================================================
 * IMPORTANT KEYWORD RULE
 * ============================================================================
 *
 * This grammar deliberately does NOT require a new `MIND` lexer keyword.
 *
 * The existing AI mind grammar already uses annotation-based mind constructs
 * and keeps cognitive vocabulary open.
 *
 * Introducing a second globally reserved `MIND` token solely for this
 * expression grammar would create unnecessary lexical coupling.
 *
 * Therefore the expression-level form uses the existing identifier namespace
 * for its operation names.
 *
 * ============================================================================
 * EXPRESSION-CORE BOUNDARY
 * ============================================================================
 *
 * The canonical expression hierarchy is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * That grammar currently owns:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     rangeExpression
 *     logical expressions
 *     arithmetic expressions
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *     argument-list composition
 *
 * A leaf grammar MUST NOT copy that hierarchy.
 *
 * The production architecture therefore requires a canonical operand boundary:
 *
 *     expressionCore
 *
 * The eventual canonical composition is:
 *
 *     Expressions
 *          |
 *          +--> MindExpressions
 *                    |
 *                    v
 *              expressionCore
 *
 * `expressionCore` must be supplied by the canonical expression-core
 * composition layer.
 *
 * This file deliberately does not reproduce the expression precedence tree.
 *
 * ============================================================================
 * PUBLIC EXPRESSION
 * ============================================================================
 *
 * `mindExpression` is the sole public expression rule owned by this file.
 *
 * A mind expression represents a cognitive computation invocation.
 *
 * Conceptual examples:
 *
 *     mind.reason(problem)
 *     mind.infer(hypothesis, evidence)
 *     mind.learn(model, data)
 *     mind.adapt(strategy, feedback)
 *     mind.explain(decision)
 *     mind.decide(options)
 *
 * The grammar does not reserve any of those operation names.
 *
 * ============================================================================
 * MIND EXPRESSION SHAPE
 * ============================================================================
 *
 * The canonical structural form is:
 *
 *     mindOperationName
 *     (
 *         arguments...
 *     )
 *
 * with optional:
 *
 *     binding
 *     condition
 *     options
 *
 * The semantic model may normalize this to:
 *
 *     MindOperation
 *         namespace
 *         operation
 *         operands
 *         parameters
 *         results
 *         attributes
 *         modifiers
 *         effects
 *         capabilities
 *         resources
 *         contracts
 *         policy
 *         provenance
 *         source
 *
 * This matches the universal operation model.
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * This grammar MUST NOT enumerate:
 *
 *     infer
 *     deduce
 *     reason
 *     learn
 *     adapt
 *     explain
 *     decide
 *     query
 *     assert
 *     retract
 *     predict
 *     plan
 *     reflect
 *     observe
 *
 * as separate grammar alternatives.
 *
 * They remain names.
 *
 * This allows future operations without changing this grammar.
 *
 * The semantic registry determines whether a named operation is:
 *
 *     reasoning
 *     knowledge
 *     learning
 *     adaptation
 *     explanation
 *     decision
 *     evidence processing
 *     uncertainty processing
 *     provenance processing
 *     model execution
 *     hybrid control
 *     another registered semantic operation.
 *
 * ============================================================================
 * NAMESPACE MODEL
 * ============================================================================
 *
 * Qualified operation names are supported:
 *
 *     reasoning.infer
 *     knowledge.query
 *     learning.train
 *     adaptation.apply
 *     explanation.explain
 *     policy.evaluate
 *     quantum.observe
 *     hybrid.control
 *
 * Namespace meaning is semantic.
 *
 * The grammar does not contain a finite namespace catalogue.
 *
 * ============================================================================
 * ARGUMENT MODEL
 * ============================================================================
 *
 * Arguments are expressions.
 *
 * Therefore the cognitive layer can consume:
 *
 *     scalar values
 *     tensors
 *     collections
 *     functions
 *     model values
 *     knowledge queries
 *     measurements
 *     quantum-derived values
 *     hardware observations
 *     distributed values
 *     simulation results
 *     provenance values
 *     policy values
 *     contracts
 *     symbolic values
 *     future domain values
 *
 * without modifying this grammar.
 *
 * ============================================================================
 * OPTION MODEL
 * ============================================================================
 *
 * Options are expressions.
 *
 * The grammar does not define a closed option vocabulary.
 *
 * Semantic options may represent:
 *
 *     strategy
 *     model
 *     evidence
 *     confidence
 *     probability
 *     policy
 *     provenance
 *     resource preference
 *     capability preference
 *     execution mode
 *     simulation
 *     fallback
 *     reproducibility
 *     determinism
 *     adaptation policy
 *
 * New options do not require grammar modification.
 *
 * ============================================================================
 * BINDING MODEL
 * ============================================================================
 *
 * A cognitive operation may optionally bind its result:
 *
 *     mind.reason(problem) as conclusion
 *
 * Binding semantics are downstream.
 *
 * This grammar does not decide:
 *
 *     mutability
 *     ownership
 *     lifetime
 *     storage
 *     register allocation
 *     CPU placement
 *     GPU placement
 *     QPU placement.
 *
 * ============================================================================
 * CONDITION MODEL
 * ============================================================================
 *
 * Cognitive operations may be conditionally enabled:
 *
 *     mind.adapt(strategy) when feedback
 *
 * Conditions are ordinary expressions.
 *
 * There is no second boolean-expression grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only structural validity.
 *
 * Semantic analysis determines:
 *
 *     - operation identity;
 *     - namespace resolution;
 *     - operand types;
 *     - result types;
 *     - generic inference;
 *     - operation availability;
 *     - effect requirements;
 *     - capability requirements;
 *     - resource requirements;
 *     - contract requirements;
 *     - policy applicability;
 *     - provenance requirements;
 *     - determinism;
 *     - authorization;
 *     - target feasibility.
 *
 * A syntactically valid mind expression is NOT automatically executable.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/expressions/reasoning.g4
 *
 * remains the syntax owner for:
 *
 *     reasoningExpression
 *     reasoningOperation
 *     reasoningOperator
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *
 * This file does not duplicate those rules.
 *
 * A semantic operation such as:
 *
 *     mind.reason(...)
 *
 * may resolve to the existing reasoning semantic model.
 *
 * Therefore:
 *
 *     mind.reason
 *          |
 *          v
 *     reasoning semantic operation
 *
 * and not:
 *
 *     mind.reason
 *          |
 *          v
 *     MindReasoningIR
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/expressions/knowledge.g4
 *
 * remains the syntax owner for knowledge expressions.
 *
 * Mind expressions may consume knowledge values:
 *
 *     mind.infer(knowledge.query(pattern))
 *
 * The knowledge subsystem owns:
 *
 *     assertion
 *     retraction
 *     query
 *     lookup
 *     update
 *
 * This file does not duplicate them.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning is an operation, not a fixed grammar catalogue.
 *
 * Examples:
 *
 *     mind.learn(model, data)
 *     mind.learn(model, data, objective)
 *
 * Semantic analysis determines:
 *
 *     algorithm
 *     model
 *     objective
 *     data
 *     resources
 *     capabilities
 *     effects
 *     policy
 *     provenance
 *
 * The grammar does not enumerate:
 *
 *     gradient descent
 *     reinforcement learning
 *     transfer learning
 *     Bayesian learning
 *     evolutionary learning
 *     future learning methods
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Adaptation is controlled computation.
 *
 * A semantic adaptation operation must participate in:
 *
 *     authorization
 *     policy
 *     effects
 *     provenance
 *     validation
 *     resources
 *     capabilities
 *
 * Adaptation MUST NOT imply unrestricted self-modifying execution.
 *
 * Conceptually:
 *
 *     mind.adapt(strategy, feedback)
 *          |
 *          v
 *     authorization
 *          |
 *          v
 *     policy validation
 *          |
 *          v
 *     state/model/strategy change
 *          |
 *          v
 *     provenance
 *          |
 *          v
 *     continued execution
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Mind operations may consume:
 *
 *     uncertain values
 *     probabilities
 *     distributions
 *     confidence
 *     belief
 *     intervals
 *
 * No fixed probability representation is required.
 *
 * No fixed precision is imposed.
 *
 * No finite number of outcomes is encoded.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence is represented by ordinary expression operands.
 *
 * Evidence may originate from:
 *
 *     knowledge
 *     observation
 *     measurement
 *     simulation
 *     model output
 *     classical computation
 *     quantum computation
 *     HDL simulation
 *     hardware observations
 *     distributed services
 *     external data
 *
 * Evidence semantics belong downstream.
 *
 * ============================================================================
 * EXPLANATION / DECISION INTEGRATION
 * ============================================================================
 *
 * Mind operations may produce:
 *
 *     explanations
 *     decisions
 *     reasons
 *     evidence relationships
 *
 * These become semantic records rather than application-specific syntax.
 *
 * Example:
 *
 *     mind.explain(decision)
 *
 * does not require an AI-only explanation runtime.
 *
 * The same semantic mechanism may explain:
 *
 *     AI decisions
 *     compiler decisions
 *     routing decisions
 *     resource decisions
 *     scheduling decisions
 *     security decisions
 *     optimization decisions
 *     hardware mapping decisions.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policy syntax remains owned by:
 *
 *     grammar/expressions/policy.g4
 *
 * and the central policy subsystem.
 *
 * A mind operation may consume policy as an operand:
 *
 *     mind.adapt(strategy, policy)
 *
 * or be constrained semantically by an active policy.
 *
 * This file MUST NOT redefine:
 *
 *     allow
 *     forbid
 *     require
 *     prefer
 *     constrain
 *     fallback
 *
 * as separate grammar constructs.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Mind operations participate in:
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
 * This file only preserves the expression boundary required for semantic
 * validation.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Mind operations may consume or produce provenance:
 *
 *     mind.reason(problem, provenance)
 *     mind.learn(model, data, provenance)
 *     mind.adapt(strategy, feedback, provenance)
 *
 * Provenance semantics remain owned by the central provenance subsystem.
 *
 * The AST must preserve source spans and operand order so provenance can
 * later identify:
 *
 *     source
 *     operation
 *     evidence
 *     decision
 *     transformation
 *     result.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar declares NO concrete effects.
 *
 * Semantic resolution may infer effects such as:
 *
 *     reasoning
 *     knowledge.read
 *     knowledge.write
 *     learning
 *     adaptation
 *     randomness
 *     measurement
 *     network
 *     io
 *     native
 *     foreign
 *     distributed
 *     simulation
 *     reflection
 *     code_generation
 *
 * Effects remain separate from capabilities and resources.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A mind operation may require capabilities such as:
 *
 *     capability("reasoning")
 *     capability("knowledge.query")
 *     capability("learning")
 *     capability("adaptation")
 *     capability("explanation")
 *     capability("probabilistic.compute")
 *     capability("tensor.compute")
 *     capability("quantum.measurement")
 *
 * Capability names are symbolic.
 *
 * This grammar does not enumerate them.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Mind operations may require:
 *
 *     memory
 *     compute
 *     accelerator capacity
 *     model resources
 *     storage
 *     communication
 *     quantum resources
 *     simulation resources
 *
 * No finite capacity is encoded here.
 *
 * Forbidden examples include:
 *
 *     MAX_MINDS
 *     MAX_REASONING
 *     MAX_LEARNING
 *     MAX_MODELS
 *     MAX_EVIDENCE
 *     MAX_CONTEXT
 *     MAX_AGENTS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_DEVICE_COUNT
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Mind expressions may consume quantum results:
 *
 *     mind.reason(measurement)
 *     mind.learn(quantum_features)
 *     mind.decide(quantum_result)
 *
 * They may also participate in hybrid control:
 *
 *     quantum
 *         ->
 *     measurement
 *         ->
 *     mind decision
 *         ->
 *     classical control
 *
 * Quantum semantics remain owned by:
 *
 *     grammar/quantum/
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT define:
 *
 *     physical qubits
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Mind expressions may consume HDL simulation/verification results.
 *
 * They may semantically contribute to hardware/software co-design.
 *
 * They MUST NOT introduce:
 *
 *     signal-width ceilings
 *     register ceilings
 *     device ceilings
 *     topology limits
 *     synthesis implementation details.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Mind operations may execute semantically over:
 *
 *     actors
 *     tasks
 *     services
 *     distributed data
 *     parallel computation
 *
 * The concurrency and distributed subsystems own execution semantics.
 *
 * This file creates no second actor system.
 *
 * ============================================================================
 * AGENT BOUNDARY
 * ============================================================================
 *
 * A mind is not automatically an agent.
 *
 * Agent lifecycle remains owned by:
 *
 *     grammar/ai/agent.g4
 *     grammar/concurrency/
 *
 * A mind expression can be invoked by an agent, but:
 *
 *     mind != actor
 *     mind != process
 *     mind != scheduler
 *
 * ============================================================================
 * FFI / ABI BOUNDARY
 * ============================================================================
 *
 * Mind operations may consume external values returned through FFI.
 *
 * FFI/ABI semantics remain owned by:
 *
 *     grammar/interoperability/
 *
 * Such operations inherit their:
 *
 *     effects
 *     capabilities
 *     security requirements
 *     provenance
 *
 * from the interoperability subsystem.
 *
 * ============================================================================
 * REFLECTION / METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Mind expressions may consume reflection/metaprogramming values.
 *
 * Reflection remains controlled by:
 *
 *     grammar/metaprogramming/
 *
 * A mind expression MUST NOT bypass:
 *
 *     type checking
 *     effect checking
 *     capability checking
 *     policy checking
 *     provenance.
 *
 * ============================================================================
 * SIMULATION BOUNDARY
 * ============================================================================
 *
 * Mind operations may be executed in simulation.
 *
 * Simulation is an execution strategy.
 *
 * It is not a second language.
 *
 * The distinction:
 *
 *     simulation
 *
 * versus:
 *
 *     physical execution
 *
 * is semantic and execution-level, not expression grammar-level.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The same mind expression must remain source-compatible across:
 *
 *     tiny systems
 *     embedded systems
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed infrastructure
 *     cloud
 *     future hardware
 *
 * provided the required semantics can be realized.
 *
 * The grammar itself contains no target-size assumptions.
 *
 * POCO-REAF therefore means:
 *
 *     source meaning
 *          |
 *          v
 *     target-independent semantics
 *          |
 *          v
 *     capability negotiation
 *          |
 *          v
 *     resource negotiation
 *          |
 *          v
 *     specialization
 *          |
 *          v
 *     lowering
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * All collections use unbounded grammar repetition:
 *
 *     *
 *     +
 *
 * No grammar-level finite limit exists for:
 *
 *     operations
 *     arguments
 *     options
 *     bindings
 *     namespace depth
 *     expression complexity
 *     evidence
 *     knowledge
 *     models
 *     resources
 *     capabilities
 *     domains
 *     devices
 *     nodes
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *
 * Actual limits are implementation-resource conditions.
 *
 * The grammar does not promise infinite physical resources.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain none of:
 *
 *     MAX_MIND_*
 *     MAX_REASONING_*
 *     MAX_KNOWLEDGE_*
 *     MAX_LEARNING_*
 *     MAX_ADAPTATION_*
 *     MAX_EVIDENCE_*
 *     MAX_CONTEXT_*
 *     MAX_MODELS_*
 *     MAX_AGENTS_*
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
 *
 * No vendor catalog is allowed.
 *
 * No finite algorithm catalogue is allowed.
 *
 * No application-specific keyword catalogue is allowed.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar cannot select:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     physical qubit 0
 *     node 0
 *     device 0
 *
 * It cannot encode:
 *
 *     fixed topology
 *     fixed register width
 *     fixed memory size
 *     fixed tensor rank
 *     fixed accelerator count.
 *
 * Target realization is downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The domain-neutral AST must preserve:
 *
 *     operation name
 *     namespace
 *     ordered operands
 *     ordered options
 *     optional binding
 *     optional condition
 *     source span
 *     source ordering
 *     nested expression structure
 *
 * Conceptual representation:
 *
 *     MindExpression
 *         operation
 *         namespace
 *         operands[]
 *         options[]
 *         binding?
 *         condition?
 *         source
 *
 * The grammar does not require a target-specific AST.
 *
 * ============================================================================
 * SEMANTIC NORMALIZATION
 * ============================================================================
 *
 * The semantic layer SHOULD normalize a mind expression into the universal
 * operation model:
 *
 *     name
 *     namespace
 *     operands
 *     parameters
 *     results
 *     attributes
 *     modifiers
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     source
 *
 * This is the bridge from cognitive syntax to universal computation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * A mind expression may lower to:
 *
 *     classical IR
 *     tensor/data IR
 *     distributed semantic operations
 *     hardware/HDL semantic operations
 *     quantum::ir
 *     another explicitly-owned domain IR
 *
 * according to semantic resolution.
 *
 * There is no:
 *
 *     MindIR
 *     CognitiveIR
 *     ReasoningIR
 *
 * introduced by this grammar.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics MUST distinguish:
 *
 *     missing operation name
 *     malformed qualified name
 *     missing opening parenthesis
 *     missing closing parenthesis
 *     malformed argument list
 *     malformed binding
 *     malformed condition
 *     malformed option list.
 *
 * Semantic diagnostics MUST distinguish:
 *
 *     unknown operation
 *     unknown namespace
 *     invalid operand type
 *     invalid option
 *     unavailable capability
 *     insufficient resources
 *     forbidden effect
 *     policy violation
 *     contract violation
 *     unavailable model
 *     unavailable knowledge source
 *     unsupported target.
 *
 * Target feasibility failures MUST NOT be reported as parser errors.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source
 *     lexer
 *     grammar
 *     parser configuration.
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     filesystem
 *     network
 *     runtime state
 *     model availability
 *     device availability
 *     random state
 *     wall-clock time.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing MUST NOT:
 *
 *     - execute a mind operation;
 *     - execute a model;
 *     - query knowledge storage;
 *     - access a network;
 *     - inspect hardware;
 *     - load credentials;
 *     - invoke a QPU;
 *     - invoke a simulator;
 *     - load plugins.
 *
 * All such activity belongs downstream.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing `grammar/ai/mind.g4` syntax remains authoritative for mind
 * declarations.
 *
 * This file does not replace or reinterpret those declarations.
 *
 * Existing:
 *
 *     @mind ...
 *
 * constructs remain distinct from expression-level cognitive operations.
 *
 * New semantic cognitive operations should normally be introduced by registry
 * or dialect metadata rather than by editing this grammar.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following structural forms are required once `expressionCore` is bound
 * to the canonical expression hierarchy:
 *
 *     mind.reason(problem)
 *
 *     mind.infer(hypothesis, evidence)
 *
 *     mind.deduce(conclusion, premises)
 *
 *     mind.learn(model, data)
 *
 *     mind.adapt(strategy, feedback)
 *
 *     mind.explain(decision)
 *
 *     mind.decide(options)
 *
 *     mind.query(knowledge)
 *
 *     mind.assert(fact)
 *
 *     mind.retract(fact)
 *
 *     mind.observe(observation)
 *
 *     mind.predict(input)
 *
 *     mind.reflect(state)
 *
 *     mind.reason(
 *         hypothesis,
 *         evidence,
 *         policy
 *     )
 *
 *     mind.learn(
 *         model,
 *         dataset,
 *         objective,
 *         resources
 *     )
 *
 *     mind.adapt(
 *         strategy,
 *         feedback,
 *         policy,
 *         provenance
 *     )
 *
 * CROSS-DOMAIN:
 *
 *     mind.reason(measurement_result)
 *
 *     mind.learn(quantum_features, training_data)
 *
 *     mind.decide(hardware_observation)
 *
 *     mind.explain(resource_decision)
 *
 *     mind.reason(simulation_result)
 *
 *     mind.query(distributed_knowledge)
 *
 *     mind.adapt(classical_strategy, quantum_feedback)
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These must be rejected:
 *
 *     mind
 *
 *     mind.
 *
 *     mind.reason(
 *
 *     mind.reason(,)
 *
 *     mind.reason(value,,value)
 *
 *     mind.reason(value value)
 *
 *     mind.reason(value as)
 *
 * where the malformed form violates the canonical operand grammar.
 *
 * Semantic negatives include:
 *
 *     unknown operation
 *     unavailable capability
 *     forbidden effect
 *     unsatisfied resource requirement
 *     invalid policy
 *     invalid contract
 *
 * These must be reported downstream rather than as syntax failures when the
 * syntax itself is valid.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     simple operation
 *     qualified operation
 *     nested operation
 *     nested ordinary expression
 *     many arguments
 *     many options
 *     conditional operation
 *     bound result
 *     quantum-derived operand
 *     HDL-derived operand
 *     distributed operand
 *     simulation operand
 *     FFI-derived operand
 *     reflection-derived operand
 *     provenance operand
 *     policy operand.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test increasing implementation-supported:
 *
 *     argument counts
 *     option counts
 *     namespace depth
 *     nested operation depth
 *     expression size
 *     source size
 *
 * without declaring a language maximum.
 *
 * The same grammar must represent:
 *
 *     tiny cognitive computations
 *
 * and:
 *
 *     extremely large finite cognitive computations
 *
 * subject only to actual compiler/parser/runtime resources.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     grammar version
 *     lexer configuration
 *     parser configuration
 *
 * MUST produce equivalent parse structure.
 *
 * No hardware, model, network, runtime or resource state may alter parsing.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. `grammar/expressions/expressions.g4`
 *
 *    MUST add `mindExpression` to the canonical primary-expression
 *    composition.
 *
 * 2. The canonical expression-core extraction MUST provide:
 *
 *        expressionCore
 *
 *    as the operand boundary consumed here.
 *
 * 3. The expression composition layer MUST import/compose this grammar
 *    exactly once.
 *
 * 4. No other grammar may define another public `mindExpression`.
 *
 * 5. `grammar/ai/mind.g4` remains the owner of mind declarations and regions.
 *
 * 6. `grammar/expressions/reasoning.g4` remains the owner of reasoning
 *    expression syntax.
 *
 * 7. `grammar/expressions/knowledge.g4` remains the owner of knowledge
 *    expression syntax.
 *
 * 8. `grammar/expressions/policy.g4` remains the owner of policy expression
 *    syntax.
 *
 * 9. `grammar/expressions/provenance.g4` remains the owner of provenance
 *    expression syntax.
 *
 * 10. AI, quantum, HDL, distributed and hardware domains consume the resulting
 *     semantic operation rather than redefining this grammar.
 *
 * 11. The Rust frontend must map the parse structure into the existing
 *     domain-neutral AST.
 *
 * 12. Semantic analysis must attach:
 *
 *        effects
 *        capabilities
 *        resources
 *        contracts
 *        policies
 *        provenance
 *
 *     without modifying source meaning.
 *
 * 13. Quantum computation must lower through:
 *
 *        quantum::ir
 *
 *     and no cognitive-specific quantum IR may be introduced.
 *
 * 14. Classical computation must lower through the canonical classical path.
 *
 * 15. Target realization remains downstream of semantic analysis.
 *
 * ============================================================================
 * REQUIRED COMPOSITION CHANGE
 * ============================================================================
 *
 * The following conceptual change belongs in:
 *
 *     grammar/expressions/expressions.g4
 *
 * Its existing:
 *
 *     primaryExpression
 *
 * rule must eventually contain:
 *
 *     | mindExpression
 *
 * alongside the existing primary forms.
 *
 * It MUST NOT copy the rules from this file into expressions.g4.
 *
 * ============================================================================
 * REQUIRED EXPRESSION-CORE CHANGE
 * ============================================================================
 *
 * The repository currently documents `expressionCore` as an intended
 * integration boundary, but the inspected `expressions.g4` still owns the
 * complete public `expression` hierarchy directly.
 *
 * Before this file is marked fully integrated, establish exactly one shared
 * expression-core boundary.
 *
 * Recommended ownership:
 *
 *     grammar/expressions/core.g4
 *
 * or an equivalent canonical composition grammar.
 *
 * The migration must preserve:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     rangeExpression
 *     logical precedence
 *     arithmetic precedence
 *     prefixExpression
 *     postfixExpression
 *     primaryExpression
 *
 * There must remain exactly one authoritative expression hierarchy.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file must remain a parser grammar:
 *
 *     parser grammar MindExpressions;
 *
 * with:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * It must contain:
 *
 *     no lexer rules
 *     no Rust actions
 *     no semantic predicates
 *     no embedded code
 *     no runtime execution.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar requires no unsafe Rust.
 *
 * The generated parser and consuming implementation must remain compatible
 * with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * Repository code must continue to satisfy:
 *
 *     cargo check --all-targets
 *     cargo test --all-targets
 *     cargo fmt --all -- --check
 *     cargo clippy --all-targets --all-features -- -D warnings
 *
 * subject to the repository's supported feature matrix.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS COMPLETE WHEN:
 *
 * [x] It owns only expression-level cognitive composition.
 *
 * [x] It does not replace grammar/ai/mind.g4.
 *
 * [x] It does not create a second cognitive language.
 *
 * [x] It does not enumerate cognitive algorithms.
 *
 * [x] It does not enumerate application domains.
 *
 * [x] It does not enumerate hardware targets.
 *
 * [x] It contains no universal resource ceilings.
 *
 * [x] It contains no physical hardware identifiers.
 *
 * [x] It contains no quantum gate catalogue.
 *
 * [x] It creates no AI IR.
 *
 * [x] It creates no cognitive IR.
 *
 * [x] It creates no quantum IR.
 *
 * [x] It preserves domain-neutral AST semantics.
 *
 * [x] It integrates effects.
 *
 * [x] It integrates capabilities.
 *
 * [x] It integrates resources.
 *
 * [x] It integrates contracts.
 *
 * [x] It integrates policies.
 *
 * [x] It integrates provenance.
 *
 * [x] It supports classical/quantum/hybrid/HDL/distributed operands.
 *
 * [x] It has no unsafe Rust requirement.
 *
 * [x] It is compatible with Rust 1.97/1.97.1.
 *
 * [ ] Canonical expressionCore has been established.
 *
 * [ ] Expressions.g4 composes mindExpression.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Rust parser conformance succeeds.
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
 * [ ] Cross-domain tests pass.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file provides:
 *
 *     ONE cognitive expression boundary
 *
 * rather than:
 *
 *     ONE cognitive programming language.
 *
 * Its meaning is resolved through the existing Zamani universal model:
 *
 *     VALUE
 *       |
 *     TYPE
 *       |
 *     OPERATION
 *       |
 *     EFFECT
 *       |
 *     CAPABILITY
 *       |
 *     RESOURCE
 *       |
 *     REQUIREMENT
 *       |
 *     CONSTRAINT
 *       |
 *     POLICY
 *       |
 *     CONTRACT
 *       |
 *     EVIDENCE
 *       |
 *     PROVENANCE
 *       |
 *     DECISION
 *       |
 *     SEMANTIC MODEL
 *       |
 *       +------------------+
 *       |                  |
 *       v                  v
 *   classical          quantum::ir
 *       |                  |
 *       +--------+---------+
 *                |
 *                v
 *       optimization/lowering
 *                |
 *         routing/scheduling
 *                |
 *        resilience/QEC/ZQN
 *                |
 *               HAL
 *                |
 *        target realization
 *
 * This is the required boundary for POCO-REAF:
 *
 * source meaning remains stable while realization changes according to
 * available capabilities, resources, policies and target semantics.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */

parser grammar MindExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC EXPRESSION ENTRY
 * ========================================================================== */

mindExpression
    : mindComposition
    ;


/* ============================================================================
 * 2. COGNITIVE COMPOSITION
 * ========================================================================== */

/*
 * The operation name remains open-world.
 *
 * Examples:
 *
 *     mind.reason(...)
 *     mind.infer(...)
 *     mind.learn(...)
 *     mind.adapt(...)
 *     mind.explain(...)
 *     mind.decide(...)
 *
 * The grammar does not reserve those names.
 */
mindComposition
    : mindOperation
    ;


/* ============================================================================
 * 3. OPERATION
 * ========================================================================== */

mindOperation
    : mindOperationName
      LPAREN
      mindArgumentList?
      RPAREN
      mindBinding?
      mindCondition?
      mindOptionList?
    ;


/* ============================================================================
 * 4. OPERATION NAME
 * ========================================================================== */

mindOperationName
    : identifier
      mindOperationNamespaceSuffix*
    ;


/* ============================================================================
 * 5. OPERATION NAMESPACE
 * ========================================================================== */

mindOperationNamespaceSuffix
    : DOT identifier
    ;


/* ============================================================================
 * 6. ARGUMENT LIST
 * ========================================================================== */

mindArgumentList
    : mindArgument
      (
          COMMA
          mindArgument
      )*
      COMMA?
    ;


/* ============================================================================
 * 7. ARGUMENT
 * ========================================================================== */

mindArgument
    : mindOperand
    ;


/* ============================================================================
 * 8. OPERAND INTEGRATION BOUNDARY
 * ========================================================================== */

/*
 * This is deliberately a named integration boundary.
 *
 * It MUST be bound to the canonical expression-core grammar when the
 * expression hierarchy is composed.
 *
 * Do not copy the complete expression grammar here.
 */
mindOperand
    : expressionCore
    ;


/* ============================================================================
 * 9. RESULT BINDING
 * ========================================================================== */

mindBinding
    : AS identifier
    ;


/* ============================================================================
 * 10. CONDITIONAL EXECUTION
 * ========================================================================== */

mindCondition
    : WHEN mindOperand
    ;


/* ============================================================================
 * 11. OPTIONS
 * ========================================================================== */

mindOptionList
    : WITH
      LPAREN
      mindOptionItems?
      RPAREN
    ;


mindOptionItems
    : mindOption
      (
          COMMA
          mindOption
      )*
      COMMA?
    ;


mindOption
    : mindOperand
    ;