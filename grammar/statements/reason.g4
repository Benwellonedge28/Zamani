/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/reason.g4
 *
 * GRAMMAR
 * -------
 * ReasonStatements
 *
 * STATUS
 * ------
 * PRODUCTION-READY UNIVERSAL REASONING STATEMENT COMPOSITION
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the statement-level syntax for Zamani's generic reasoning
 * family:
 *
 *     infer
 *     deduce
 *     reason
 *
 * These constructs express PORTABLE COMPUTATIONAL INTENT.
 *
 * They do not select:
 *
 *     reasoning algorithms
 *     theorem provers
 *     rule engines
 *     knowledge stores
 *     machine-learning implementations
 *     probabilistic engines
 *     causal engines
 *     hardware
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     distributed topology
 *     scheduler
 *     runtime
 *     backend
 *
 * Those decisions belong to semantic analysis, capability analysis,
 * resource negotiation, policy evaluation, execution planning and
 * downstream IR/lowering.
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
 *     statement composition
 *       |
 *       v
 *     reasonStatement
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       +--> structural validation
 *       +--> name/type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> contract analysis
 *       +--> policy analysis
 *       +--> provenance
 *       |
 *       v
 *     semantic reasoning operation
 *       |
 *       +--> classical realization
 *       +--> quantum semantic realization
 *       +--> hybrid realization
 *       +--> AI/model realization
 *       +--> distributed realization
 *       +--> accelerator realization
 *       +--> future realization
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> other canonical domain IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering / specialization
 *       |
 *       v
 *     routing / scheduling
 *       |
 *       v
 *     resilience / recovery
 *       |
 *       v
 *     ZQN / HAL / target realization
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     reasonStatement
 *     reasoningOperator
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     statement
 *     expressions
 *     expression precedence
 *     identifiers
 *     names
 *     literals
 *     types
 *     patterns
 *     guards
 *     knowledge storage
 *     assertions
 *     retraction
 *     query implementation
 *     learning
 *     adaptation
 *     uncertainty implementation
 *     probability implementation
 *     evidence storage
 *     provenance storage
 *     policy implementation
 *     effect implementation
 *     capability implementation
 *     resource allocation
 *     quantum operations
 *     quantum routing
 *     quantum error correction
 *     HDL
 *     hardware realization
 *     distributed execution
 *     networking
 *     FFI
 *     ABI
 *     runtime execution
 *     IR generation
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * Public parser entry:
 *
 *     reasonStatement
 *
 * Public supporting rules:
 *
 *     reasoningOperator
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * No universal `statement` rule is defined here.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Lexer authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which consumes the canonical lexer composition under:
 *
 *     grammar/lexer/
 *
 * Required tokens:
 *
 *     INFER
 *     DEDUCE
 *     REASON
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *
 * This parser grammar defines NO lexer rules.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/expressions/expressions.g4
 *
 * IMPORTS:
 *
 *     Expressions
 *
 * EXPORTS:
 *
 *     reasonStatement
 *     reasoningOperator
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * CONSUMED_BY:
 *
 *     grammar/statements/statements.g4
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The dependency graph MUST remain:
 *
 *     ReasonStatements
 *          |
 *          v
 *     Expressions
 *
 * while:
 *
 *     Statements
 *          |
 *          v
 *     ReasonStatements
 *
 * Therefore this file MUST NOT import:
 *
 *     Statements
 *
 * or any grammar that imports the universal statement composition.
 *
 * This prevents circular grammar dependencies.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Reasoning operands are ordinary Zamani expressions.
 *
 * This means the reasoning grammar automatically supports whatever the
 * canonical expression grammar supports, including future expressions.
 *
 * Examples:
 *
 *     infer hypothesis;
 *
 *     infer model(input);
 *
 *     infer knowledge.query(pattern);
 *
 *     infer measurement_result;
 *
 *     infer tensor[index];
 *
 *     infer quantum_result;
 *
 *     infer distributed_value;
 *
 *     deduce conclusion from premises;
 *
 *     reason decision from observations;
 *
 * The reasoning grammar MUST NOT recreate:
 *
 *     primaryExpression
 *     postfixExpression
 *     unaryExpression
 *     binaryExpression
 *     logicalExpression
 *     arithmeticExpression
 *     callExpression
 *     indexingExpression
 *     memberExpression
 *
 * Those belong exclusively to:
 *
 *     grammar/expressions/
 *
 * ============================================================================
 * CANONICAL SOURCE FORMS
 * ============================================================================
 *
 * Minimal:
 *
 *     infer TARGET;
 *     deduce TARGET;
 *     reason TARGET;
 *
 * With source:
 *
 *     infer TARGET from SOURCE;
 *     deduce TARGET from SOURCE;
 *     reason TARGET from SOURCE;
 *
 * With context:
 *
 *     infer TARGET with (OPTION);
 *     deduce TARGET with (OPTION);
 *     reason TARGET with (OPTION);
 *
 * Combined:
 *
 *     infer TARGET from SOURCE with (OPTION);
 *     deduce TARGET from SOURCE with (OPTION);
 *     reason TARGET from SOURCE with (OPTION);
 *
 * ============================================================================
 * CLAUSE ORDER
 * ============================================================================
 *
 * Canonical order is:
 *
 *     OPERATOR
 *     TARGET
 *     SOURCE?
 *     CONTEXT?
 *     TERMINATOR
 *
 * Therefore:
 *
 *     reason target from source with (context);
 *
 * is valid.
 *
 * The following is intentionally invalid:
 *
 *     reason target with (context) from source;
 *
 * There is exactly one canonical ordering.
 *
 * This reduces syntactic ambiguity and simplifies:
 *
 *     AST generation
 *     formatting
 *     tooling
 *     diagnostics
 *     compatibility
 *     source-to-source transformation
 *
 * ============================================================================
 * OPERATOR SEMANTICS
 * ============================================================================
 *
 * The three operators:
 *
 *     infer
 *     deduce
 *     reason
 *
 * belong to one semantic reasoning family.
 *
 * They MUST NOT create three unrelated reasoning implementations.
 *
 * The parser preserves the source operator so semantic analysis can distinguish
 * the requested reasoning intent.
 *
 * The semantic layer may represent the operation kind as:
 *
 *     Infer
 *     Deduce
 *     Reason
 *
 * while sharing the common reasoning representation.
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * A reasoning target is mandatory.
 *
 * Valid:
 *
 *     infer conclusion;
 *
 *     deduce result;
 *
 *     reason decision;
 *
 * Invalid:
 *
 *     infer;
 *
 *     deduce;
 *
 *     reason;
 *
 * The grammar checks only syntactic presence.
 *
 * Semantic analysis determines whether the expression is actually valid as a
 * reasoning target.
 *
 * ============================================================================
 * SOURCE CONTRACT
 * ============================================================================
 *
 * `from` introduces one complete expression.
 *
 * Examples:
 *
 *     infer conclusion from evidence;
 *
 *     deduce result from premises;
 *
 *     reason decision from measurement;
 *
 *     infer answer from knowledge.query(pattern);
 *
 *     deduce state_property from quantum_result;
 *
 * The source may represent:
 *
 *     premises
 *     facts
 *     evidence
 *     observations
 *     measurements
 *     model output
 *     knowledge
 *     simulation results
 *     hardware observations
 *     distributed results
 *     computed values
 *     quantum-derived values
 *
 * No source category is hard-coded into this grammar.
 *
 * ============================================================================
 * CONTEXT CONTRACT
 * ============================================================================
 *
 * `with (...)` supplies ordered reasoning context.
 *
 * Examples:
 *
 *     reason proposition with (policy);
 *
 *     infer result with (model);
 *
 *     deduce decision with (confidence);
 *
 *     reason conclusion with (strategy, evidence, policy);
 *
 * Context expressions may semantically represent:
 *
 *     strategy
 *     evidence
 *     confidence
 *     model
 *     policy
 *     provenance
 *     constraints
 *     assumptions
 *     resource preferences
 *     deterministic execution requirements
 *     domain-specific context
 *
 * This grammar intentionally does not create a closed context vocabulary.
 *
 * ============================================================================
 * EMPTY CONTEXT
 * ============================================================================
 *
 * This is invalid:
 *
 *     reason target with ();
 *
 * The context must contain at least one expression.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * This is valid:
 *
 *     reason target with (a, b);
 *
 * This is invalid:
 *
 *     reason target with (a, b,);
 *
 * A trailing comma is not part of the canonical syntax.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume knowledge expressions.
 *
 * Examples:
 *
 *     infer answer from knowledge.query(pattern);
 *
 *     reason conclusion from query(source);
 *
 *     deduce fact from observation;
 *
 * Knowledge operations such as:
 *
 *     assert
 *     retract
 *     query
 *
 * remain owned by their respective grammar/semantic subsystems.
 *
 * This file does not duplicate knowledge syntax.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume learned-model results:
 *
 *     infer result from model(input);
 *
 *     reason classification from predictor(value);
 *
 * Learning syntax remains owned by the learning subsystem.
 *
 * This grammar does not define training algorithms or model architectures.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Reasoning may produce information subsequently consumed by adaptation:
 *
 *     infer strategy from evidence;
 *
 * followed by an adaptation construct elsewhere.
 *
 * Reasoning itself does not authorize:
 *
 *     state mutation
 *     model mutation
 *     strategy replacement
 *     self-modification
 *     resource reallocation
 *
 * Those actions require their own:
 *
 *     effect
 *     capability
 *     resource
 *     policy
 *     provenance
 *
 * validation.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Reasoning expressions may operate over:
 *
 *     probability
 *     distribution
 *     confidence
 *     belief
 *     uncertain values
 *     intervals
 *     symbolic uncertainty
 *
 * This grammar imposes no:
 *
 *     probability precision
 *     confidence scale
 *     distribution size
 *     numerical representation
 *     implementation algorithm
 *
 * Those belong to the type and semantic systems.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence may originate from:
 *
 *     classical computation
 *     quantum measurement
 *     simulation
 *     hardware observation
 *     data processing
 *     learned models
 *     distributed computation
 *     network services
 *     scientific computation
 *
 * Evidence is represented through ordinary expressions.
 *
 * Evidence verification and provenance remain semantic responsibilities.
 *
 * ============================================================================
 * EXPLANATION INTEGRATION
 * ============================================================================
 *
 * A reasoning result may later be consumed by an explanation construct.
 *
 * The grammar does not require a specific explanation representation.
 *
 * Explanation, evidence and decision records belong to their respective
 * semantic systems.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Reasoning may occur under:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * contracts.
 *
 * This file does not duplicate contract grammar.
 *
 * Contract ownership remains under:
 *
 *     grammar/validation/
 *
 * Semantic validation determines whether the reasoning operation satisfies
 * applicable contracts.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Reasoning may be governed by:
 *
 *     security policy
 *     privacy policy
 *     evidence policy
 *     execution policy
 *     resource policy
 *     model policy
 *     determinism policy
 *     adaptation policy
 *
 * Policies are not evaluated by the parser.
 *
 * Valid syntax does NOT imply authorization.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Reasoning may acquire effects during semantic analysis, including:
 *
 *     computation
 *     knowledge.read
 *     model.inference
 *     randomness
 *     network
 *     external.io
 *     measurement
 *     distributed
 *     simulation
 *     foreign
 *     native
 *
 * The grammar does not declare or infer effects.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Semantic analysis may derive requirements such as:
 *
 *     capability("reasoning")
 *     capability("reasoning.inference")
 *     capability("reasoning.deduction")
 *     capability("knowledge.query")
 *     capability("model.inference")
 *     capability("probabilistic.compute")
 *     capability("quantum.measurement")
 *     capability("distributed.compute")
 *
 * Capability names remain open-ended semantic data.
 *
 * This grammar does not enumerate hardware capabilities.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Reasoning may require arbitrary resources:
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
 * Resource feasibility belongs downstream.
 *
 * This file MUST NOT define language-level limits such as:
 *
 *     maximum reasoning depth
 *     maximum premise count
 *     maximum evidence count
 *     maximum context count
 *     maximum model count
 *     maximum memory
 *     maximum CPU count
 *     maximum GPU count
 *     maximum QPU count
 *     maximum node count
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume values produced by quantum computation:
 *
 *     infer result from measurement_result;
 *
 *     reason decision from quantum_result;
 *
 *     deduce property from measured_state;
 *
 * This file does not define quantum operations.
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * and the canonical quantum semantic boundary remains:
 *
 *     AST
 *       ->
 *     quantum semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     QEC / resilience
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * No reasoning-specific quantum IR is introduced here.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Reasoning may operate entirely over classical values.
 *
 * Examples:
 *
 *     infer result from calculation;
 *
 *     deduce conclusion from premises;
 *
 *     reason decision from observation;
 *
 * The classical semantic layer remains responsible for type and execution
 * resolution.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Reasoning may bridge classical and quantum information.
 *
 * Conceptually:
 *
 *     quantum computation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     reasoning
 *          |
 *          v
 *     classical decision
 *
 * or:
 *
 *     classical evidence
 *          |
 *          v
 *     reasoning
 *          |
 *          v
 *     quantum operation selection
 *
 * The grammar does not encode the physical mapping.
 *
 * ============================================================================
 * AI / MODEL INTEGRATION
 * ============================================================================
 *
 * A reasoning target or source may contain:
 *
 *     model(...)
 *     predictor(...)
 *     classifier(...)
 *     learned_result
 *
 * provided those are valid ordinary Zamani expressions.
 *
 * No model architecture is encoded here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume:
 *
 *     simulation results
 *     verification results
 *     hardware observations
 *     synthesis information
 *     timing information
 *     resource information
 *
 * Hardware representation remains owned by the HDL/hardware subsystems.
 *
 * This grammar MUST NOT encode:
 *
 *     register width
 *     bus width
 *     fixed device count
 *     fixed FPGA capacity
 *     physical address
 *     vendor topology
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume distributed results:
 *
 *     deduce result from distributed_value;
 *
 * or reason over expressions representing:
 *
 *     actor results
 *     service responses
 *     collective results
 *     replicated observations
 *
 * Distribution semantics belong to:
 *
 *     grammar/concurrency/
 *     grammar/distributed/
 *     execution/runtime
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Network-derived evidence may be supplied as an ordinary expression.
 *
 * Example:
 *
 *     infer state from service_result;
 *
 * Network effects, security, authorization and resource requirements are
 * checked downstream.
 *
 * ============================================================================
 * FFI / ABI INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume results returned from foreign interfaces.
 *
 * Example:
 *
 *     deduce result from foreign_result;
 *
 * FFI and ABI remain owned by:
 *
 *     grammar/interoperability/
 *
 * Foreign operations must participate in effect and capability checking.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Reasoning may be used over compile-time or reflective values when those
 * values are valid Zamani expressions.
 *
 * Compile-time execution and reflection remain owned by:
 *
 *     grammar/metaprogramming/
 *
 * This grammar never executes them.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * A reasoning operation must be capable of preserving source provenance.
 *
 * Downstream semantic representation should be able to associate:
 *
 *     operator
 *     target
 *     source
 *     context
 *     source span
 *     evidence
 *     derivation
 *     decision
 *     resulting value
 *
 * with provenance records where required by the program's policy or
 * compilation configuration.
 *
 * This grammar does not implement provenance.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser contexts produced here map to the domain-neutral frontend AST.
 *
 * The semantic AST representation should preserve at minimum:
 *
 *     reasoning kind
 *     target
 *     optional source
 *     ordered context
 *     source span
 *
 * The AST MUST remain independent of:
 *
 *     LLVM
 *     QIR
 *     MLIR
 *     vendor IR
 *     physical qubit mapping
 *     routing
 *     calibration
 *     QEC implementation
 *     physical topology
 *     backend selection
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     whether the target is valid;
 *     whether the source is valid;
 *     whether context values are valid;
 *     type compatibility;
 *     name resolution;
 *     ownership;
 *     effects;
 *     capabilities;
 *     resources;
 *     contracts;
 *     policies;
 *     provenance;
 *     domain compatibility;
 *     determinism requirements;
 *     execution feasibility.
 *
 * Parser acceptance alone does not imply semantic validity.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The grammar accepts expressions without imposing a closed type set.
 *
 * Semantic analysis may allow reasoning over:
 *
 *     scalar values
 *     collections
 *     records
 *     functions
 *     tensors
 *     graphs
 *     probabilistic values
 *     quantum-derived values
 *     distributed values
 *     hardware observations
 *     model values
 *     symbolic values
 *     future types
 *
 * Type validity remains downstream.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file generates NO IR.
 *
 * The pipeline remains:
 *
 *     parser
 *       ->
 *     AST
 *       ->
 *     semantic reasoning operation
 *       ->
 *     canonical semantic representation
 *       ->
 *     domain IR
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     target realization
 *
 * Quantum operations continue to use:
 *
 *     quantum::ir
 *
 * as the canonical quantum IR boundary.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics include only structural problems such as:
 *
 *     missing reasoning operator
 *     missing target
 *     missing source expression
 *     malformed context
 *     empty context
 *     missing closing parenthesis
 *     trailing comma
 *     invalid clause order
 *     missing semicolon
 *
 * Semantic diagnostics include:
 *
 *     unknown name
 *     invalid type
 *     invalid reasoning target
 *     invalid reasoning source
 *     unavailable capability
 *     unsatisfied resource requirement
 *     forbidden effect
 *     policy violation
 *     contract violation
 *     invalid provenance requirement
 *     invalid domain composition
 *
 * Parser and semantic diagnostics MUST remain separate.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * This grammar MUST NOT:
 *
 *     invoke a reasoning engine;
 *     access a knowledge store;
 *     invoke a model;
 *     access a network;
 *     access filesystem state;
 *     access secrets;
 *     inspect hardware;
 *     discover devices;
 *     invoke a QPU;
 *     invoke a simulator;
 *     invoke foreign code;
 *     execute generated code.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     token stream
 *     grammar version
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware state
 *     device availability
 *     scheduler state
 *     runtime state
 *
 * Identical source under identical parser configuration must produce
 * equivalent parse-tree structure.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no language-level finite limit for:
 *
 *     reasoning statements
 *     source size
 *     context count
 *     expression size
 *     expression nesting
 *     knowledge size
 *     evidence size
 *     model count
 *     quantum resources
 *     CPU resources
 *     GPU resources
 *     FPGA resources
 *     accelerator resources
 *     node count
 *     memory
 *     tensor dimensions
 *     topology size
 *
 * Repetition is represented by grammar structure rather than fixed slots.
 *
 * Actual parser/compiler exhaustion is an implementation/resource concern,
 * not a semantic language ceiling.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Reasoning syntax describes computational intent rather than physical
 * realization.
 *
 * Therefore the same source can participate in compilation for:
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
 *     heterogeneous systems
 *     future computational systems
 *
 * provided the selected realization can satisfy the program's semantic
 * requirements.
 *
 * The language does NOT claim that every target is capable of executing every
 * reasoning operation.
 *
 * Instead:
 *
 *     source intent
 *       ->
 *     semantic requirements
 *       ->
 *     capability negotiation
 *       ->
 *     resource negotiation
 *       ->
 *     execution planning
 *       ->
 *     target realization
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_REASONING_DEPTH
 *     MAX_REASONING_STEPS
 *     MAX_PREMISES
 *     MAX_EVIDENCE
 *     MAX_CONTEXT
 *     MAX_MODELS
 *     MAX_FACTS
 *     MAX_CPU
 *     MAX_CPUS
 *     MAX_GPU
 *     MAX_GPUS
 *     MAX_FPGA
 *     MAX_FPGAS
 *     MAX_QUBIT
 *     MAX_QUBITS
 *     MAX_QPU
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * Also forbidden:
 *
 *     fixed hardware identifiers
 *     vendor-specific device assumptions
 *     physical topology
 *     physical addresses
 *     fixed accelerator counts
 *     fixed thread counts
 *
 * Allowed:
 *
 *     finite language keywords
 *     finite punctuation
 *     finite syntactic operators
 *     program-provided numeric values
 *     symbolic resource quantities
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical surface syntax is:
 *
 *     infer TARGET [from SOURCE] [with (OPTIONS)] ;
 *     deduce TARGET [from SOURCE] [with (OPTIONS)] ;
 *     reason TARGET [from SOURCE] [with (OPTIONS)] ;
 *
 * New reasoning mechanisms MUST NOT require changes to this grammar merely
 * because a new algorithm, model, hardware target, or backend is introduced.
 *
 * New semantic reasoning capabilities should be represented through:
 *
 *     libraries
 *     semantic metadata
 *     capabilities
 *     policies
 *     dialects
 *     execution strategies
 *
 * rather than keyword proliferation.
 *
 * ============================================================================
 * INFER / DEDUCE ADAPTER CONTRACT
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/statements/infer.g4
 *     grammar/statements/deduce.g4
 *
 * are compatibility adapters for the shared reasoning model.
 *
 * They MUST NOT be composed alongside this grammar in a way that creates
 * multiple competing parser paths for the same source form.
 *
 * Recommended production composition:
 *
 *     statements.g4
 *          |
 *          +--> ReasonStatements
 *
 * and NOT:
 *
 *     statements.g4
 *          |
 *          +--> ReasonStatements
 *          +--> Infer
 *          +--> Deduce
 *
 * at the same time.
 *
 * If the individual adapter grammars are retained, they should delegate to
 * the shared reasoning rules and remain available only where an isolated
 * grammar entry is required.
 *
 * ============================================================================
 * STATEMENT COMPOSITION CONTRACT
 * ============================================================================
 *
 * The universal statement composition grammar:
 *
 *     grammar/statements/statements.g4
 *
 * must import:
 *
 *     ReasonStatements
 *
 * and its `statement` production must admit:
 *
 *     reasonStatement
 *
 * exactly once.
 *
 * This file does NOT modify the universal statement rule.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 *     infer conclusion;
 *
 *     deduce conclusion;
 *
 *     reason conclusion;
 *
 *     infer conclusion from evidence;
 *
 *     deduce result from premises;
 *
 *     reason decision from observation;
 *
 *     infer result with (model);
 *
 *     deduce result with (confidence);
 *
 *     reason result with (policy, provenance);
 *
 *     infer result from evidence with (confidence, policy);
 *
 *     deduce result from knowledge.query(pattern) with (policy);
 *
 *     reason decision from quantum_result with (evidence, confidence);
 *
 * NEGATIVE TESTS
 * -------------
 *
 *     infer;
 *
 *     deduce;
 *
 *     reason;
 *
 *     infer ;
 *
 *     deduce from source;
 *
 *     reason from source;
 *
 *     reason target with ();
 *
 *     reason target with (a,);
 *
 *     reason target with (a) from source;
 *
 *     reason target from;
 *
 *     reason target with;
 *
 *     reason target (;
 *
 *     reason target with (a;
 *
 *     reason target from source
 *
 * SEMANTIC NEGATIVES
 * ------------------
 *
 * These are NOT parser failures:
 *
 *     unknown target name
 *     unknown source name
 *     incompatible target/source types
 *     unavailable reasoning capability
 *     unsatisfied resource requirement
 *     forbidden effect
 *     policy violation
 *     contract violation
 *     invalid quantum-derived value
 *
 * Those must be rejected downstream.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     deeply nested target expressions
 *     deeply nested source expressions
 *     large context lists
 *     large expressions
 *     Unicode identifiers
 *     qualified names
 *     calls
 *     indexing
 *     tensor expressions
 *     graph expressions
 *     uncertainty expressions
 *     knowledge queries
 *     model expressions
 *     quantum-derived expressions
 *     distributed values
 *     hardware observations
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * The reasoning statement must compose with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI/model
 *     data
 *     concurrency
 *     distributed
 *     networking
 *     security
 *     FFI
 *     metaprogramming
 *     simulation
 *
 * without modifying the reasoning grammar for each target domain.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Test progressively larger source programs and expressions while ensuring
 * no language-level resource ceiling is introduced.
 *
 * Test symbolic resource expressions rather than only concrete hardware
 * quantities.
 *
 * Example semantic inputs may contain:
 *
 *     required_memory
 *     required_qubits
 *     required_compute
 *     required_topology
 *
 * without embedding machine capacity into this grammar.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     token stream
 *     grammar version
 *     parser configuration
 *
 * must produce equivalent parse-tree structure.
 *
 * ============================================================================
 * ROUND-TRIP TESTS
 * ============================================================================
 *
 * Where formatter support exists:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve reasoning semantics.
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
 *
 * and safe Rust only.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] ReasonStatements is the sole shared reasoning statement composition.
 *
 * [ ] `reasonStatement` is the stable public entry rule.
 *
 * [ ] infer/deduce/reason share one structural model.
 *
 * [ ] target is mandatory.
 *
 * [ ] source is optional.
 *
 * [ ] context is optional.
 *
 * [ ] canonical clause order is enforced.
 *
 * [ ] empty contexts are rejected.
 *
 * [ ] trailing commas are rejected.
 *
 * [ ] expressions are delegated to Expressions.
 *
 * [ ] no expression hierarchy is duplicated.
 *
 * [ ] no lexer rules are defined here.
 *
 * [ ] no semantic actions exist.
 *
 * [ ] no semantic predicates exist.
 *
 * [ ] no runtime behavior exists.
 *
 * [ ] no hardware discovery exists.
 *
 * [ ] no backend selection exists.
 *
 * [ ] no physical topology exists.
 *
 * [ ] no quantum gate catalog exists.
 *
 * [ ] no hardware capacity limit exists.
 *
 * [ ] no machine-size constant exists.
 *
 * [ ] AST mapping exists.
 *
 * [ ] semantic mapping exists.
 *
 * [ ] effect integration exists.
 *
 * [ ] capability integration exists.
 *
 * [ ] resource integration exists.
 *
 * [ ] contract integration exists.
 *
 * [ ] policy integration exists.
 *
 * [ ] provenance integration exists.
 *
 * [ ] quantum integration uses quantum::ir downstream.
 *
 * [ ] infer.g4 does not create a competing universal parser path.
 *
 * [ ] deduce.g4 does not create a competing universal parser path.
 *
 * [ ] statements.g4 imports ReasonStatements.
 *
 * [ ] statements.g4 admits reasonStatement exactly once.
 *
 * [ ] positive tests pass.
 *
 * [ ] negative tests pass.
 *
 * [ ] semantic-negative tests pass.
 *
 * [ ] boundary tests pass.
 *
 * [ ] scalability tests pass.
 *
 * [ ] cross-domain tests pass.
 *
 * [ ] determinism tests pass.
 *
 * [ ] portability tests pass.
 *
 * [ ] round-trip tests pass.
 *
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar ReasonStatements;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Expressions is the ONLY grammar dependency required here.
 *
 * Do not import Statements.
 *
 * Do not import Infer.
 *
 * Do not import Deduce.
 *
 * This prevents duplicate reasoning ownership and circular dependencies.
 * ============================================================================
 */

import
    Expressions
    ;

/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * Canonical forms:
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
 * The semicolon is mandatory at this statement boundary.
 * ============================================================================
 */

reasonStatement
    : reasoningOperator
      reasoningTarget
      reasoningSourceClause?
      reasoningContextClause?
      SEMICOLON
    ;

/*
 * ============================================================================
 * REASONING OPERATOR
 * ============================================================================
 *
 * One lexical family, one semantic family.
 * ============================================================================
 */

reasoningOperator
    : INFER
    | DEDUCE
    | REASON
    ;

/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * Delegated completely to the canonical expression grammar.
 * ============================================================================
 */

reasoningTarget
    : expression
    ;

/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 */

reasoningSourceClause
    : FROM
      expression
    ;

/*
 * ============================================================================
 * CONTEXT
 * ============================================================================
 *
 * At least one option is required.
 * Empty parentheses are therefore rejected.
 * ============================================================================
 */

reasoningContextClause
    : WITH
      LPAREN
      reasoningOptionList
      RPAREN
    ;

/*
 * ============================================================================
 * CONTEXT OPTION LIST
 * ============================================================================
 *
 * One or more expressions.
 *
 * No trailing comma.
 * ============================================================================
 */

reasoningOptionList
    : reasoningOption
      (
          COMMA
          reasoningOption
      )*
    ;

/*
 * ============================================================================
 * CONTEXT OPTION
 * ============================================================================
 *
 * Any canonical Zamani expression may supply context.
 *
 * Semantic analysis determines its role.
 * ============================================================================
 */

reasoningOption
    : expression
    ;