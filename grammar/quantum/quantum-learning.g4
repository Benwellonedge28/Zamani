/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/quantum/quantum-learning.g4
 *
 * GRAMMAR
 * -------
 * QuantumLearning
 *
 * STATUS
 * ------
 * CANONICAL QUANTUM-LEARNING COMPOSITION GRAMMAR
 *
 * LANGUAGE / IMPLEMENTATION BASELINE
 * -----------------------------------
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
 * PURPOSE
 * ============================================================================
 *
 * This file defines the quantum-learning DOMAIN COMPOSITION boundary.
 *
 * It does NOT define a second learning language.
 *
 * It does NOT define a second quantum language.
 *
 * It does NOT define a second quantum operation grammar.
 *
 * It does NOT define a quantum-learning IR.
 *
 * It does NOT define target hardware.
 *
 * It does NOT define physical qubits.
 *
 * It does NOT define a QPU.
 *
 * It does NOT define a simulator implementation.
 *
 * It does NOT enumerate machine-learning algorithms.
 *
 * It does NOT enumerate quantum algorithms.
 *
 * Instead, it composes the already canonical:
 *
 *     learning intent
 *     +
 *     quantum operation intent
 *
 * into one explicit source-level domain boundary.
 *
 * The canonical architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     universal learning             quantum operations
 *     Learn.learnStatement           QuantumOperations
 *          |                              |
 *          +--------------+---------------+
 *                         |
 *                         v
 *                QuantumLearning
 *                         |
 *                         v
 *                domain-neutral AST
 *                         |
 *                         v
 *                structural validation
 *                         |
 *             +-----------+-----------+
 *             |           |           |
 *             v           v           v
 *           types      effects    capabilities
 *             |           |           |
 *             +-----------+-----------+
 *                         |
 *                         v
 *                    resources
 *                         |
 *                         v
 *                      policies
 *                         |
 *                         v
 *                    contracts
 *                         |
 *                         v
 *                    provenance
 *                         |
 *                         v
 *                 semantic quantum model
 *                         |
 *                         v
 *                     quantum::ir
 *                         |
 *             +-----------+-----------+
 *             |           |           |
 *             v           v           v
 *        optimization  decomposition  lowering
 *                         |
 *                         v
 *                    routing
 *                         |
 *                         v
 *                    scheduling
 *                         |
 *                         v
 *                 resilience / QEC
 *                         |
 *                         v
 *                        ZQN
 *                         |
 *                         v
 *                        HAL
 *                         |
 *                         v
 *                  target realization
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Quantum learning is a semantic composition of:
 *
 *     learning
 *     quantum computation
 *     classical computation
 *     data/model state
 *     resource requirements
 *     capabilities
 *     effects
 *     policies
 *     contracts
 *     provenance
 *
 * It is NOT a new computational substrate.
 *
 * Therefore this grammar intentionally avoids creating:
 *
 *     QuantumLearningIR
 *     QMLIR
 *     QuantumTrainingIR
 *     LearningQuantumIR
 *     QuantumModelIR
 *     QuantumOptimizerIR
 *
 * The semantic layer determines which canonical domain representations are
 * required.
 *
 * If quantum computation is present, the quantum portion MUST ultimately
 * cross:
 *
 *     quantum::ir
 *
 * There is exactly one canonical quantum IR boundary.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This file participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * by describing source-level intent rather than target realization.
 *
 * A quantum-learning construct must remain independent of:
 *
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     quantum simulator
 *     distributed system
 *     HPC system
 *     cloud provider
 *     device identifier
 *     physical qubit
 *     physical register
 *     coupling map
 *     topology
 *     calibration
 *     pulse implementation
 *     routing algorithm
 *     scheduler
 *     vendor instruction
 *
 * Those concerns are resolved downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar establishes NO universal finite capacity.
 *
 * It contains no:
 *
 *     MAX_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *     MAX_QUANTUM_LEARNING_OPERATIONS
 *     MAX_LEARNING_OPERATIONS
 *     MAX_MODELS
 *     MAX_DATASETS
 *     MAX_PARAMETERS
 *     MAX_TENSORS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_CONTROLS
 *     MAX_TARGETS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_CIRCUIT_WIDTH
 *     MAX_QPUS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *
 * or equivalent hidden ceilings.
 *
 * Repetition is represented structurally with ANTLR repetition operators.
 *
 * "Scale to infinity" means:
 *
 *     the language grammar does not impose a finite machine-derived ceiling.
 *
 * It does NOT mean that physical execution resources are infinite.
 *
 * Actual feasibility is determined by:
 *
 *     compiler resources
 *     target resources
 *     capabilities
 *     policies
 *     execution context
 *     physical constraints
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     quantumLearningConstruct
 *     quantumLearningRegion
 *     quantumLearningMarker
 *     quantumLearningItem
 *
 * THIS FILE OWNS THE COMPOSITION BOUNDARY ONLY.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     learnStatement
 *     learnClause
 *     learnFromClause
 *     learnWithClause
 *     learnArgumentList
 *     learning algorithms
 *     model declarations
 *     dataset declarations
 *     training declarations
 *     inference
 *     reasoning
 *     adaptation
 *     knowledge
 *     query
 *     probability
 *     uncertainty
 *     provenance
 *     policy
 *     contracts
 *     capabilities
 *     resources
 *     effects
 *
 *     quantumOperationStatement
 *     quantumOperation
 *     quantumOperationApplication
 *     quantumOperationSpecifier
 *     quantumOperationTargetClause
 *     quantumOperationTargetList
 *     quantumOperationTarget
 *
 *     quantum kernels
 *     quantum circuits
 *     quantum registers
 *     qubits
 *     logical qubits
 *     physical qubits
 *     measurement
 *     reset
 *     noise
 *     QEC
 *     routing
 *     scheduling
 *     calibration
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The source-level `learn` syntax has exactly one owner:
 *
 *     grammar/statements/learn.g4
 *
 * Grammar:
 *
 *     Learn
 *
 * Public rule:
 *
 *     learnStatement
 *
 * Quantum operation invocation has exactly one owner:
 *
 *     grammar/quantum/operations.g4
 *
 * Grammar:
 *
 *     QuantumOperations
 *
 * Public rules include:
 *
 *     quantumOperationStatement
 *     quantumOperation
 *     quantumOperationApplication
 *
 * This file MUST delegate to those rules.
 *
 * It MUST NOT reproduce either syntax.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/statements/learn.g4
 *     grammar/quantum/operations.g4
 *     grammar/core/names.g4
 *
 * Transitively consumed through those grammars:
 *
 *     canonical expressions
 *     canonical identifiers
 *     canonical qualified names
 *     canonical lexical vocabulary
 *     canonical learning syntax
 *     canonical quantum operation syntax
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports grammar NAMES rather than filesystem paths.
 *
 * Required ANTLR imports:
 *
 *     Learn
 *     QuantumOperations
 *     Names
 *
 * The grammar build must therefore expose the corresponding grammar
 * directories in its ANTLR import/search path.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PUBLIC:
 *
 *     quantumLearningConstruct
 *
 * PRIVATE:
 *
 *     quantumLearningRegion
 *     quantumLearningMarker
 *     quantumLearningItem
 *
 * No other public semantic owner is introduced.
 *
 * ============================================================================
 * CONSUMER CONTRACT
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/quantum/quantum.g4
 *
 * The quantum composition root should import:
 *
 *     QuantumLearning
 *
 * and expose:
 *
 *     quantumLearningConstruct
 *
 * as the quantum-learning domain boundary.
 *
 * The universal parser composition layer must then make that boundary
 * reachable from the appropriate source positions.
 *
 * This file MUST NOT import:
 *
 *     ZamaniParser
 *
 * and MUST NOT reverse the grammar dependency direction.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser contexts only.
 *
 * It does NOT define Rust AST structures.
 *
 * The frontend AST must preserve enough structure to distinguish:
 *
 *     ordinary learning
 *
 * from:
 *
 *     explicitly quantum-learning composition
 *
 * where the source uses the explicit region.
 *
 * Conceptually:
 *
 *     QuantumLearningRegion {
 *         marker,
 *         items,
 *         source_span
 *     }
 *
 * The AST remains DOMAIN-NEUTRAL.
 *
 * It must not require:
 *
 *     QuantumLearningModel
 *     QuantumTrainingModel
 *     QuantumOptimizerModel
 *     QPUModel
 *     GPUModel
 *
 * merely because a source construct occurs inside this region.
 *
 * The semantic layer determines the actual domain composition.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     whether the quantum-learning marker is valid;
 *     whether learning occurs;
 *     whether quantum computation occurs;
 *     what values cross the quantum/classical boundary;
 *     whether learning consumes quantum results;
 *     whether quantum operations consume learned values;
 *     whether measurement is required;
 *     whether classical feed-forward is involved;
 *     whether the operation is valid;
 *     whether model/data types are compatible;
 *     whether quantum operands are valid;
 *     whether effects are valid;
 *     whether capabilities are available;
 *     whether resources are sufficient;
 *     whether policies authorize execution;
 *     whether contracts are satisfied;
 *     whether provenance is required;
 *     whether deterministic/reproducible execution is requested;
 *     whether the computation can cross domain boundaries;
 *     whether the semantic operation can be lowered.
 *
 * Parser acceptance MUST NOT imply semantic validity.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Quantum learning may semantically involve combinations of effects such as:
 *
 *     learning
 *     quantum
 *     measurement
 *     randomness
 *     mutation
 *     simulation
 *     distributed
 *     network
 *     foreign
 *     native
 *     IO
 *
 * This file does NOT define an effect taxonomy.
 *
 * Existing effect grammars remain authoritative.
 *
 * The semantic layer infers or validates effects from the actual operation.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Quantum learning may require open-world capabilities such as:
 *
 *     quantum.operation
 *     quantum.measurement
 *     quantum.dynamic_control
 *     quantum.parameterized_operation
 *     quantum.logical_qubit
 *     learning
 *     tensor.compute
 *     model.compute
 *     accelerator.compute
 *     distributed.compute
 *
 * These names are examples of semantic capability identifiers.
 *
 * This grammar does NOT enumerate or require a closed capability catalogue.
 *
 * Capability availability is resolved downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements are NOT allocated or validated by this grammar.
 *
 * Semantic analysis may derive requirements involving:
 *
 *     qubits
 *     logical qubits
 *     quantum memory
 *     classical memory
 *     tensor resources
 *     accelerator resources
 *     execution time
 *     communication
 *     storage
 *     energy
 *     resilience
 *     distributed resources
 *     other future resources
 *
 * Symbolic requirements remain valid.
 *
 * For example, semantic layers may interpret:
 *
 *     required quantum width
 *
 * from source-level expressions without imposing a universal maximum.
 *
 * This file MUST NOT encode a target capacity.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Quantum-learning constructs may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax is owned by the universal validation/contract subsystem.
 *
 * This grammar does not duplicate contract syntax.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Quantum-learning may be governed by policies involving:
 *
 *     data access
 *     model modification
 *     quantum execution
 *     measurement
 *     randomness
 *     network access
 *     foreign calls
 *     resource consumption
 *     adaptation
 *     reproducibility
 *     security
 *     deployment
 *
 * Policy syntax remains owned by:
 *
 *     grammar/policies/
 *
 * and the existing security/policy subsystem.
 *
 * This file does not enforce policies.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Quantum learning is a high-value provenance boundary.
 *
 * Semantic provenance may preserve:
 *
 *     source program
 *     source span
 *     learning subject
 *     learning source
 *     quantum operation identity
 *     quantum operands
 *     measurement results
 *     model identity
 *     data identity
 *     semantic transformations
 *     optimization decisions
 *     decomposition
 *     routing
 *     scheduling
 *     target realization
 *     capability decisions
 *     resource decisions
 *     policy decisions
 *     verification results
 *
 * This grammar does not create provenance records.
 *
 * It only preserves source structure required by downstream provenance.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This file MUST NOT define an IR.
 *
 * The canonical quantum path is:
 *
 *     quantum-learning source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum-learning model
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> HAL
 *
 * If the learning portion is purely classical, it may lower through the
 * canonical classical/data representations instead.
 *
 * A mixed computation may lower through multiple domain representations, but
 * there remains only one canonical quantum IR for its quantum component.
 *
 * ============================================================================
 * HYBRID CONTRACT
 * ============================================================================
 *
 * Quantum learning is inherently capable of hybrid execution.
 *
 * Typical semantic flows include:
 *
 *     classical data
 *          |
 *          v
 *     learning
 *          |
 *          v
 *     parameter values
 *          |
 *          v
 *     quantum operation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     classical result
 *          |
 *          v
 *     learning update
 *
 * or:
 *
 *     learned model
 *          |
 *          v
 *     quantum parameterization
 *          |
 *          v
 *     quantum computation
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     loss / objective
 *          |
 *          v
 *     learning update
 *
 * The grammar does not encode these algorithms.
 *
 * The semantic model determines the dependency graph.
 *
 * ============================================================================
 * OPEN-WORLD LEARNING CONTRACT
 * ============================================================================
 *
 * This file MUST NOT enumerate:
 *
 *     gradient descent
 *     stochastic gradient descent
 *     Adam
 *     RMSProp
 *     variational algorithms
 *     quantum neural networks
 *     kernel methods
 *     reinforcement methods
 *     evolutionary methods
 *     transformers
 *     neural networks
 *     support-vector methods
 *     future algorithms
 *
 * Such concepts remain:
 *
 *     identifiers
 *     expressions
 *     values
 *     libraries
 *     dialects
 *     declarations
 *     semantic providers
 *     capability providers
 *
 * This allows new computational techniques to be introduced without changing
 * the universal grammar.
 *
 * ============================================================================
 * OPEN-WORLD QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum operation names remain open-world through:
 *
 *     QuantumOperations.quantumOperationStatement
 *
 * Therefore this file MUST NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *     vendor gates
 *     simulator gates
 *     future gates
 *
 * New quantum operations remain semantic declarations, library operations,
 * dialect operations, or capability-provided operations.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Parsing MUST NOT depend on:
 *
 *     QPU availability
 *     GPU availability
 *     CPU count
 *     memory availability
 *     network state
 *     model availability
 *     dataset availability
 *     runtime state
 *     scheduler state
 *     randomness
 *     current time
 *
 * Semantic execution may be nondeterministic when explicitly permitted by
 * the language's effect, policy, and reproducibility systems.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing this grammar MUST NOT:
 *
 *     load a model
 *     load a dataset
 *     contact a QPU
 *     contact a simulator
 *     allocate a device
 *     execute quantum operations
 *     execute learning
 *     access the filesystem
 *     access the network
 *     invoke FFI
 *     invoke native code
 *     modify persistent state
 *     inspect credentials
 *
 * All such behavior belongs to authorized downstream compiler/runtime
 * components.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This is a pure ANTLR parser grammar.
 *
 * It contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no target-language actions;
 *     no runtime calls;
 *     no unsafe code;
 *     no filesystem access;
 *     no networking;
 *     no hardware access.
 *
 * Generated/consuming Rust code MUST remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and the repository's safe-Rust policy.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical learning syntax remains valid because this grammar
 * delegates to:
 *
 *     Learn.learnStatement
 *
 * Existing canonical quantum operation syntax remains valid because this
 * grammar delegates to:
 *
 *     QuantumOperations.quantumOperationStatement
 *
 * Therefore this file must not alter the meaning of:
 *
 *     learn model;
 *     learn model from data;
 *     learn model with (...);
 *     learn model from data with (...);
 *
 * or:
 *
 *     apply operation(q);
 *     apply operation(parameter)(q);
 *
 * Existing quantum operation names remain ordinary semantic names.
 *
 * ============================================================================
 * CONTEXTUAL MARKER CONTRACT
 * ============================================================================
 *
 * The explicit region uses:
 *
 *     quantum_learning { ... }
 *
 * but `quantum_learning` is intentionally NOT introduced as a global lexer
 * keyword.
 *
 * The parser accepts an identifier in the marker position:
 *
 *     quantumLearningMarker
 *         : identifier
 *         ;
 *
 * Semantic validation MUST require the canonical spelling:
 *
 *     quantum_learning
 *
 * This avoids expanding the global keyword vocabulary merely to create one
 * domain composition boundary.
 *
 * If a future language-version policy promotes the spelling to a dedicated
 * token, the semantic meaning remains unchanged and the compatibility layer
 * must handle the lexical transition.
 *
 * ============================================================================
 * REGION CONTRACT
 * ============================================================================
 *
 * Canonical explicit source form:
 *
 *     quantum_learning {
 *         learn model from data;
 *         apply operation(parameter)(q);
 *     }
 *
 * The region means:
 *
 *     the enclosed source constructs are intentionally composed as a
 *     quantum-learning semantic region.
 *
 * It does NOT mean:
 *
 *     use a particular QPU
 *     use a particular simulator
 *     use a particular algorithm
 *     use a particular optimizer
 *     allocate a fixed number of qubits
 *     allocate a fixed number of GPUs
 *     allocate a fixed number of workers
 *
 * ============================================================================
 * REGION ITEM OWNERSHIP
 * ============================================================================
 *
 * The region contains only constructs whose syntax is already independently
 * owned:
 *
 *     Learn.learnStatement
 *
 * and:
 *
 *     QuantumOperations.quantumOperationStatement
 *
 * This deliberately avoids importing the universal `statement` composition
 * root here.
 *
 * Reason:
 *
 *     QuantumLearning
 *          ->
 *     specialized quantum/learning leaves
 *
 * while:
 *
 *     universal Statements
 *          ->
 *     canonical statement composition
 *
 * must remain separate dependency layers.
 *
 * This prevents a grammar cycle such as:
 *
 *     QuantumLearning
 *          ->
 *     Statements
 *          ->
 *     Domains
 *          ->
 *     QuantumLearning
 *
 * ============================================================================
 * WHY A REGION EXISTS
 * ============================================================================
 *
 * A plain:
 *
 *     learn model from quantum_result;
 *
 * is already valid universal learning syntax.
 *
 * A plain:
 *
 *     apply operation(q);
 *
 * is already valid quantum syntax.
 *
 * The explicit:
 *
 *     quantum_learning { ... }
 *
 * boundary is therefore NOT required for basic functionality.
 *
 * It exists to provide a stable source-level composition boundary for:
 *
 *     tooling
 *     semantic analysis
 *     domain diagnostics
 *     provenance
 *     domain capability analysis
 *     domain-specific verification
 *     quantum-learning conformance tests
 *
 * without creating a second language.
 *
 * ============================================================================
 * EXAMPLE SOURCE SHAPES
 * ============================================================================
 *
 * The following are STRUCTURAL examples only.
 *
 * They do not establish built-in algorithms or operations.
 *
 * ---------------------------------------------------------------------------
 *
 *     quantum_learning {
 *         learn model from data;
 *     }
 *
 * ---------------------------------------------------------------------------
 *
 *     quantum_learning {
 *         learn model from observations;
 *         apply operation(theta)(q);
 *     }
 *
 * ---------------------------------------------------------------------------
 *
 *     quantum_learning {
 *         learn parameters from training_data;
 *         apply parameterized_operation(parameters)(q);
 *     }
 *
 * ---------------------------------------------------------------------------
 *
 *     quantum_learning {
 *         apply prepare_state(state)(q);
 *         apply operation(parameters)(q);
 *         learn model from measurement_result;
 *     }
 *
 * ---------------------------------------------------------------------------
 *
 * The semantic layer determines whether these constructs actually form a
 * valid quantum-learning computation.
 *
 * ============================================================================
 * INVALID ASSUMPTIONS
 * ============================================================================
 *
 * This grammar MUST NOT imply that:
 *
 *     every learn operation is quantum;
 *     every quantum operation is learning;
 *     every measurement is a learning signal;
 *     every model is quantum;
 *     every model must run on a QPU;
 *     every quantum computation requires a learning model;
 *     every quantum-learning computation requires a physical QPU;
 *     every quantum-learning algorithm is built into the language.
 *
 * These are semantic/application decisions.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors are restricted to malformed source structure.
 *
 * Examples:
 *
 *     quantum_learning {
 *     quantum_learning ;
 *     quantum_learning {
 *         learn;
 *     }
 *
 *     quantum_learning {
 *         apply;
 *     }
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     unknown learning subject
 *     invalid model/data relationship
 *     invalid quantum operand
 *     unavailable capability
 *     insufficient resources
 *     unsupported quantum operation
 *     invalid measurement dependency
 *     invalid quantum/classical boundary
 *     policy violation
 *     contract violation
 *     unsupported target realization
 *
 * Resource or capability failure MUST NOT become parser failure.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * TEST_OWNER
 * ----------
 *
 * Recommended:
 *
 *     grammar/tests/quantum/quantum-learning/
 *
 * Required groups:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     cross-domain/
 *     compatibility/
 *     round-trip/
 *
 * ============================================================================
 * POSITIVE TEST REQUIREMENTS
 * ============================================================================
 *
 * At minimum:
 *
 *     quantum_learning {
 *         learn model;
 *     }
 *
 *     quantum_learning {
 *         learn model from data;
 *     }
 *
 *     quantum_learning {
 *         learn model from data with (
 *             objective = objective
 *         );
 *     }
 *
 *     quantum_learning {
 *         apply operation(q);
 *     }
 *
 *     quantum_learning {
 *         apply operation(parameter)(q);
 *     }
 *
 *     quantum_learning {
 *         learn parameters from observations;
 *         apply operation(parameters)(q);
 *     }
 *
 *     quantum_learning {
 *         apply operation(parameters)(q);
 *         learn model from measurement_result;
 *     }
 *
 *     quantum_learning {
 *         learn model from quantum_result;
 *         apply future::operation(model)(q);
 *     }
 *
 * Tests must use open-world names rather than implying a fixed algorithm or
 * gate catalogue.
 *
 * ============================================================================
 * NEGATIVE TEST REQUIREMENTS
 * ============================================================================
 *
 * At minimum:
 *
 *     quantum_learning
 *
 *     quantum_learning ;
 *
 *     quantum_learning {
 *
 *     quantum_learning {
 *         learn;
 *     }
 *
 *     quantum_learning {
 *         apply;
 *     }
 *
 *     quantum_learning {
 *         apply operation(
 *     }
 *
 *     quantum_learning {
 *         learn model from;
 *     }
 *
 *     quantum_learning {
 *         learn model with (a,,b);
 *     }
 *
 * Exact diagnostics remain owned by the canonical delegated grammar where
 * applicable.
 *
 * ============================================================================
 * CONTEXTUAL MARKER TESTS
 * ============================================================================
 *
 * Because `quantum_learning` is contextual rather than globally reserved,
 * semantic validation must distinguish:
 *
 *     quantum_learning { ... }
 *
 * from:
 *
 *     some_other_identifier { ... }
 *
 * The latter MUST NOT silently acquire quantum-learning semantics.
 *
 * The parser may recognize both structurally if the surrounding grammar
 * permits the form; semantic validation owns the distinction.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Required integration coverage:
 *
 *     quantum learning + classical expressions
 *     quantum learning + learning
 *     quantum learning + data
 *     quantum learning + uncertainty
 *     quantum learning + probability
 *     quantum learning + reasoning
 *     quantum learning + knowledge
 *     quantum learning + measurement
 *     quantum learning + dynamic control
 *     quantum learning + hybrid computation
 *     quantum learning + distributed execution
 *     quantum learning + accelerator computation
 *     quantum learning + simulation
 *     quantum learning + contracts
 *     quantum learning + capabilities
 *     quantum learning + resources
 *     quantum learning + policies
 *     quantum learning + provenance
 *     quantum learning + reproducibility
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must demonstrate that the grammar has no artificial ceiling on:
 *
 *     learning operations
 *     quantum operations
 *     parameters
 *     targets
 *     nested regions
 *     program size
 *     source expression complexity
 *
 * Test generators may use large values.
 *
 * Those values MUST NOT become language constants.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer configuration
 *     parser grammar
 *     dialect configuration
 *
 * parsing must produce equivalent parse structures.
 *
 * Parsing must not depend on:
 *
 *     machine size
 *     QPU availability
 *     GPU availability
 *     model availability
 *     data availability
 *     current time
 *     randomness
 *     scheduler state
 *     network state
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Formatter/AST round-trip tooling should preserve:
 *
 *     quantum-learning region identity
 *     source order
 *     learning statement structure
 *     quantum operation structure
 *     operation parameters
 *     quantum targets
 *     nested source expressions
 *
 * No semantic information may be silently discarded during parse/format
 * round-trips.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST remain free of:
 *
 *     MAX_QUBITS
 *     MAX_QPUS
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
 *     fixed model counts
 *     fixed dataset counts
 *     fixed parameter counts
 *     fixed worker counts
 *     fixed quantum-operation counts
 *     fixed circuit widths
 *     fixed circuit depths
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] It is an ANTLR4 parser grammar.
 *
 * [ ] Grammar name is QuantumLearning.
 *
 * [ ] tokenVocab is ZamaniLexer.
 *
 * [ ] Learn is imported from the canonical learning grammar.
 *
 * [ ] QuantumOperations is imported from the canonical quantum-operation
 *     grammar.
 *
 * [ ] Names is imported only for the contextual marker's identifier boundary.
 *
 * [ ] `learnStatement` is not redefined.
 *
 * [ ] Quantum operation syntax is not redefined.
 *
 * [ ] No learning algorithm catalogue exists.
 *
 * [ ] No quantum gate catalogue exists.
 *
 * [ ] No quantum-learning-specific IR exists.
 *
 * [ ] quantum::ir remains the only canonical quantum IR.
 *
 * [ ] No target hardware is encoded.
 *
 * [ ] No physical qubit allocation is encoded.
 *
 * [ ] No machine-size limit is encoded.
 *
 * [ ] No resource allocation occurs during parsing.
 *
 * [ ] No capability discovery occurs during parsing.
 *
 * [ ] No policy enforcement occurs during parsing.
 *
 * [ ] No provenance records are created during parsing.
 *
 * [ ] No embedded Rust exists.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Rust 1.97+ compatibility is preserved.
 *
 * [ ] Existing `learn` syntax remains delegated to Learn.
 *
 * [ ] Existing `apply operation(...)` syntax remains delegated to
 *     QuantumOperations.
 *
 * [ ] Explicit quantum_learning regions are parseable.
 *
 * [ ] Malformed regions are rejected structurally.
 *
 * [ ] Semantic validation owns contextual marker validation.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Round-trip tests exist.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ANTLR DECLARATION
 * ============================================================================
 */

parser grammar QuantumLearning;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL IMPORTS
 * ============================================================================
 *
 * Learn
 * ----
 *
 * Owns the universal `learn` statement.
 *
 * QuantumOperations
 * -----------------
 *
 * Owns quantum operation invocation.
 *
 * Names
 * -----
 *
 * Owns canonical identifier syntax.
 *
 * ============================================================================
 */

import
    Learn,
    QuantumOperations,
    Names
;


/*
 * ============================================================================
 * 1. PUBLIC QUANTUM-LEARNING CONSTRUCT
 * ============================================================================
 *
 * This is the ONLY public rule owned by this grammar.
 *
 * It supports two source-level forms:
 *
 *     a canonical learning statement
 *
 * or:
 *
 *     an explicit quantum-learning region.
 *
 * The first preserves ordinary learning compatibility.
 *
 * The second provides an explicit semantic composition boundary.
 *
 * ============================================================================
 */

quantumLearningConstruct
    : learnStatement
    | quantumLearningRegion
    ;


/*
 * ============================================================================
 * 2. EXPLICIT QUANTUM-LEARNING REGION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     quantum_learning {
 *         ...
 *     }
 *
 * The marker is contextual rather than a globally reserved keyword.
 *
 * Semantic validation must verify its spelling.
 *
 * ============================================================================
 */

quantumLearningRegion
    : quantumLearningMarker
      LBRACE
      quantumLearningItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 3. CONTEXTUAL MARKER
 * ============================================================================
 *
 * This rule deliberately consumes an ordinary identifier.
 *
 * It does NOT introduce a new lexer token.
 *
 * Semantic analysis must require:
 *
 *     identifier text == "quantum_learning"
 *
 * This keeps the lexical vocabulary open and avoids unnecessary global
 * reservation.
 *
 * ============================================================================
 */

quantumLearningMarker
    : identifier
    ;


/*
 * ============================================================================
 * 4. REGION ITEMS
 * ============================================================================
 *
 * Each item delegates to an independently-owned grammar.
 *
 * Learning:
 *
 *     Learn.learnStatement
 *
 * Quantum operations:
 *
 *     QuantumOperations.quantumOperationStatement
 *
 * No universal statement grammar is imported here.
 *
 * ============================================================================
 */

quantumLearningItem
    : learnStatement
    | quantumOperationStatement
    ;


/*
 * ============================================================================
 * END OF GRAMMAR
 * ============================================================================
 *
 * FINAL ARCHITECTURAL INVARIANTS
 * ------------------------------
 *
 * 1. One universal learn syntax:
 *
 *        Learn.learnStatement
 *
 * 2. One quantum operation syntax:
 *
 *        QuantumOperations.quantumOperationStatement
 *
 * 3. One canonical quantum IR:
 *
 *        quantum::ir
 *
 * 4. Quantum-learning is a composition domain, not a second language.
 *
 * 5. Learning algorithms remain open-world.
 *
 * 6. Quantum operations remain open-world.
 *
 * 7. Hardware realization remains downstream.
 *
 * 8. Resource availability remains downstream.
 *
 * 9. Capability negotiation remains downstream.
 *
 * 10. Policy enforcement remains downstream.
 *
 * 11. Contract checking remains downstream.
 *
 * 12. Provenance remains downstream.
 *
 * 13. No physical machine capacity is encoded.
 *
 * 14. No MAX_* language ceilings exist.
 *
 * 15. Parsing is deterministic and target-independent.
 *
 * 16. No embedded Rust exists.
 *
 * 17. No unsafe Rust is required.
 *
 * 18. Rust 1.97+ remains the implementation baseline.
 *
 * 19. Existing universal learning syntax remains compatible.
 *
 * 20. Existing quantum operation syntax remains compatible.
 *
 * 21. Explicit quantum-learning composition is represented without creating
 *     another global keyword.
 *
 * 22. No quantum-learning-specific IR is created.
 *
 * 23. Quantum computation ultimately crosses quantum::ir.
 *
 * 24. Semantic analysis owns domain interpretation.
 *
 * 25. Compiler/runtime layers own realization.
 *
 * ============================================================================
 */