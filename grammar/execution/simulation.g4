/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/expressions/simulation.g4
 *
 * Grammar:
 *     SimulationExpressions
 *
 * Status:
 *     PRODUCTION-READY MODULAR EXPRESSION GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the EXPRESSION form of simulation.
 *
 * It provides a composable source-level simulation expression that can be used
 * wherever a normal Zamani expression is permitted.
 *
 * Examples:
 *
 *     simulate model
 *
 *     simulate model with (scenario)
 *
 *     simulate model from source
 *
 *     simulate model from source with (policy, configuration)
 *
 *     simulate quantum_program
 *
 *     simulate hardware_model
 *
 *     simulate ai_model(input)
 *
 *     simulate circuit with (noise_model, execution_policy)
 *
 * Simulation is an EXECUTION / ANALYSIS STRATEGY.
 *
 * It is NOT:
 *
 *     - a second programming language;
 *     - a simulator implementation;
 *     - a classical-only feature;
 *     - a quantum-only feature;
 *     - an HDL-only feature;
 *     - a hardware-specific feature;
 *     - a vendor API;
 *     - a runtime implementation;
 *     - an IR.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     Expressions
 *          |
 *          +--> simulationExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------------+-----------------------+
 *          |                      |                       |
 *          v                      v                       v
 *     classical              quantum                 HDL/hardware
 *     semantics              semantics               semantics
 *                              |
 *                              v
 *                         quantum::ir
 *          |                      |                       |
 *          +----------------------+-----------------------+
 *                                 |
 *                                 v
 *                       canonical semantic model
 *                                 |
 *                       optimization / lowering
 *                                 |
 *                       routing / scheduling
 *                                 |
 *                       resilience / recovery
 *                                 |
 *                       QEC where applicable
 *                                 |
 *                                ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * This file does NOT create any IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     simulationExpression
 *     simulationOperation
 *     simulationTarget
 *     simulationSourceClause
 *     simulationContextClause
 *     simulationOptionList
 *     simulationOption
 *     simulationOperand
 *
 * THIS FILE DOES NOT OWN:
 *
 *     expression
 *     assignmentExpression
 *     conditional precedence
 *     range precedence
 *     logical precedence
 *     arithmetic precedence
 *     unary precedence
 *     postfix precedence
 *     calls
 *     indexing
 *     member access
 *     literals
 *     identifiers
 *     names
 *     types
 *     statements
 *     declarations
 *     simulation statement bodies
 *     simulation blocks
 *     simulation algorithms
 *     numerical solvers
 *     event engines
 *     waveform engines
 *     quantum simulators
 *     HDL simulators
 *     hardware models
 *     AI model runtimes
 *     resource allocation
 *     target selection
 *     capability discovery
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *     AST implementation
 *     semantic implementation
 *     IR implementation
 *
 * ============================================================================
 * CRITICAL SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There are two intentionally different simulation surfaces:
 *
 *     grammar/expressions/simulation.g4
 *         |
 *         +--> expression-level simulation
 *
 *     grammar/statements/simulate.g4
 *         |
 *         +--> statement-level structured simulation
 *
 * They MUST NOT become competing implementations.
 *
 * Expression simulation:
 *
 *     simulate target
 *     simulate target from source
 *     simulate target with (option)
 *
 * Statement simulation:
 *
 *     simulate target {
 *         ...
 *     }
 *
 * The structured simulation statement remains owned by:
 *
 *     grammar/statements/simulate.g4
 *
 * This file MUST NOT reproduce its body/block grammar.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/*
 *     canonical parent expression hierarchy
 *
 * The parent expression composition grammar supplies:
 *
 *     expression
 *
 * and the normal expression/name/type infrastructure.
 *
 * IMPORTANT:
 *
 * This file deliberately DOES NOT import:
 *
 *     Expressions
 *
 * because:
 *
 *     Expressions
 *          -> SimulationExpressions
 *          -> Expressions
 *
 * would create an unnecessary grammar-import cycle.
 *
 * Instead, `expression` is the canonical integration boundary supplied by
 * the expression composition layer.
 *
 * This follows the same leaf-grammar pattern used by other specialized
 * expression grammars.
 *
 * EXPORTS:
 *
 *     simulationExpression
 *     simulationOperation
 *     simulationTarget
 *     simulationSourceClause
 *     simulationContextClause
 *     simulationOptionList
 *     simulationOption
 *
 * CONSUMED BY:
 *
 *     grammar/expressions/expressions.g4
 *
 * AST_OWNER:
 *
 *     frontend AST subsystem.
 *
 * SEMANTIC_OWNER:
 *
 *     execution/simulation semantic subsystem.
 *
 * EFFECT_OWNER:
 *
 *     grammar/effects/
 *     semantic effect analysis
 *
 * CAPABILITY_OWNER:
 *
 *     grammar/resources/
 *     semantic capability analysis
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/
 *     semantic resource analysis
 *
 * CONTRACT_OWNER:
 *
 *     grammar/validation/
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/
 *     grammar/security/
 *     execution policy subsystem
 *
 * PROVENANCE_OWNER:
 *
 *     grammar/spec/provenance.md
 *     semantic provenance subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic IR layers.
 *
 *     Classical simulation:
 *         canonical classical semantic representation.
 *
 *     Quantum simulation:
 *         canonical quantum semantic representation
 *         ->
 *         quantum::ir
 *
 *     HDL simulation:
 *         HDL semantic representation.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/simulation/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/
 *     grammar/specification/
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Required canonical token:
 *
 *     SIMULATE
 *
 * Existing canonical lexical tokens are reused for:
 *
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT create:
 *
 *     SIMULATION
 *     SIMULATE_EXPRESSION
 *     K_SIMULATE
 *     K_FROM
 *     K_WITH
 *
 * or any other local token aliases.
 *
 * All lexical authority remains in:
 *
 *     grammar/lexer/
 *     grammar/antlr/ZamaniLexer.g4
 *
 * ============================================================================
 * OPEN-WORLD SIMULATION MODEL
 * ============================================================================
 *
 * The grammar deliberately does NOT enumerate simulation technologies.
 *
 * It does not contain alternatives for:
 *
 *     classicalSimulator
 *     quantumSimulator
 *     hdlSimulator
 *     gpuSimulator
 *     fpgaSimulator
 *     distributedSimulator
 *     eventSimulator
 *     cycleSimulator
 *     waveformSimulator
 *     numericalSolver
 *     MonteCarloSimulator
 *     vendorSimulator
 *     emulator
 *     hardwareModel
 *
 * Such distinctions belong to:
 *
 *     semantic models
 *     capabilities
 *     dialects
 *     libraries
 *     policies
 *     execution strategies
 *     backend implementations
 *
 * This is essential for long-term language extensibility.
 *
 * ============================================================================
 * UNIVERSAL SIMULATION TARGET
 * ============================================================================
 *
 * The target is a normal Zamani expression.
 *
 * Therefore it can represent:
 *
 *     classical computation
 *     numerical computation
 *     tensor computation
 *     AI/ML models
 *     knowledge systems
 *     quantum programs
 *     quantum circuits
 *     hybrid computation
 *     HDL designs
 *     hardware models
 *     accelerator computation
 *     distributed systems
 *     networked systems
 *     data pipelines
 *     future computational domains
 *
 * Examples:
 *
 *     simulate model
 *
 *     simulate model(input)
 *
 *     simulate circuit
 *
 *     simulate quantum_program
 *
 *     simulate hardware_model
 *
 *     simulate pipeline(data)
 *
 *     simulate distributed_system
 *
 *     simulate reasoning_result
 *
 * The grammar does not determine the domain.
 *
 * Semantic analysis determines the meaning and applicable simulation strategy.
 *
 * ============================================================================
 * SOURCE / INPUT MODEL
 * ============================================================================
 *
 * An optional `from` clause identifies an input/source expression.
 *
 * Examples:
 *
 *     simulate model from input
 *
 *     simulate circuit from initial_state
 *
 *     simulate hardware_model from configuration
 *
 *     simulate algorithm from dataset
 *
 * The source expression is intentionally generic.
 *
 * It is NOT assumed to be:
 *
 *     a file
 *     a dataset
 *     a quantum state
 *     a hardware model
 *     a network
 *     a database
 *     a classical value
 *
 * Semantic analysis determines the interpretation.
 *
 * ============================================================================
 * CONTEXT MODEL
 * ============================================================================
 *
 * An optional `with (...)` clause provides extensible simulation context.
 *
 * Examples:
 *
 *     simulate model with (scenario)
 *
 *     simulate circuit with (noise_model)
 *
 *     simulate model with (policy, resources)
 *
 *     simulate hardware_model with (timing_model, configuration)
 *
 *     simulate quantum_program with (noise, resilience_policy)
 *
 * The option list is intentionally open-ended.
 *
 * No fixed option vocabulary is encoded here.
 *
 * Future semantic concepts can therefore be represented without modifying
 * this grammar merely because a new simulation facility is introduced.
 *
 * ============================================================================
 * OPTION MODEL
 * ============================================================================
 *
 * Each option is a normal Zamani expression.
 *
 * This means options can represent:
 *
 *     values
 *     configuration
 *     policies
 *     resources
 *     capabilities
 *     models
 *     datasets
 *     constraints
 *     contracts
 *     provenance
 *     execution strategies
 *     dialect-specific values
 *
 * No special grammar is required for every future simulation option.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this structurally a simulation expression?"
 *
 * Semantic analysis answers:
 *
 *     "What computation is being simulated?"
 *
 *     "What simulation semantics apply?"
 *
 *     "What effects are produced?"
 *
 *     "What capabilities are required?"
 *
 *     "What resources are required?"
 *
 *     "What policies constrain execution?"
 *
 *     "What contracts must hold?"
 *
 *     "What provenance must be recorded?"
 *
 *     "Which realization is feasible?"
 *
 * This grammar does not answer those questions.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Simulation may carry effects such as:
 *
 *     simulation
 *     randomness
 *     measurement
 *     IO
 *     network
 *     distributed
 *     native
 *     foreign
 *     quantum
 *     learning
 *     adaptation
 *
 * The grammar merely preserves the simulation operation.
 *
 * Effect inference belongs downstream.
 *
 * A semantic implementation may derive:
 *
 *     simulation
 *
 * from the presence of `simulationExpression`.
 *
 * Additional effects come from the simulated target and its context.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * The grammar does not enumerate simulator capabilities.
 *
 * Semantic analysis may derive requirements such as:
 *
 *     capability("simulation")
 *     capability("quantum.simulation")
 *     capability("hdl.simulation")
 *     capability("distributed.simulation")
 *     capability("tensor.compute")
 *
 * These are semantic capability identifiers.
 *
 * They are not grammar-level machine constants.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements belong downstream.
 *
 * Simulation may require resources such as:
 *
 *     memory
 *     compute
 *     accelerator resources
 *     simulation state storage
 *     network resources
 *     quantum simulation resources
 *     timing resources
 *     storage
 *
 * The grammar MUST NOT encode a universal capacity.
 *
 * In particular, this file contains no limits on:
 *
 *     simulation size
 *     model size
 *     scenario count
 *     event count
 *     sample count
 *     state count
 *     qubit count
 *     processor count
 *     GPU count
 *     accelerator count
 *     node count
 *     memory
 *     storage
 *     tensor rank
 *     tensor dimensions
 *     network size
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Simulation expressions may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * The grammar does not evaluate these contracts.
 *
 * Structural contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * Simulation semantics consume the resulting contract model.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Simulation may be constrained by:
 *
 *     execution policies
 *     resource policies
 *     security policies
 *     adaptation policies
 *     deployment policies
 *     reproducibility policies
 *     simulation policies
 *
 * The grammar does not enumerate those policies.
 *
 * Policy ownership remains downstream.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Simulation can affect provenance because a simulation may be used to:
 *
 *     validate
 *     predict
 *     compare
 *     test
 *     verify
 *     reproduce
 *     derive
 *     explain
 *     optimize
 *
 * The parser preserves source structure.
 *
 * Semantic analysis attaches provenance such as:
 *
 *     source
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     reason
 *     evidence
 *     decision
 *     version
 *     timestamp
 *
 * Provenance semantics do not belong in this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *     parser configuration
 *
 * must produce equivalent structural parse trees.
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     memory availability
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     random state
 *     runtime state
 *     deployment state
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *     no environment inspection
 *     no randomness
 *
 * The generated parser is consumed by the Zamani Rust implementation, which
 * remains compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * No unsafe implementation is required.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no artificial finite language-level ceiling on:
 *
 *     expression nesting
 *     simulation nesting
 *     option count
 *     source complexity
 *     target complexity
 *     model complexity
 *     simulation context complexity
 *     argument count
 *     data size
 *     state size
 *     scenario complexity
 *
 * Repetition is represented structurally using ANTLR repetition operators.
 *
 * For example:
 *
 *     simulationOptionList
 *         : simulationOption (COMMA simulationOption)* COMMA?
 *         ;
 *
 * means:
 *
 *     zero/one/many according to syntax,
 *
 * not:
 *
 *     a fixed implementation capacity.
 *
 * Actual limits arise only from:
 *
 *     compiler resources
 *     parser implementation resources
 *     memory
 *     runtime resources
 *     target capabilities
 *     deployment constraints
 *     explicitly declared program requirements
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Simulation is source intent.
 *
 * The same simulation expression may therefore be considered for:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASIC-associated environments
 *     accelerators
 *     quantum simulators
 *     QPUs through simulation workflows
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * without changing this grammar merely because the available realization
 * changes.
 *
 * Program portability is separated from physical feasibility.
 *
 * If a target cannot satisfy the semantic requirements, the compiler/runtime
 * reports that fact rather than changing the meaning of the source program.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A simulation target may contain quantum computation.
 *
 * This grammar does NOT:
 *
 *     enumerate quantum gates;
 *     enumerate qubits;
 *     select physical qubits;
 *     describe coupling maps;
 *     perform routing;
 *     perform scheduling;
 *     select calibration;
 *     implement QEC;
 *     implement ZQN;
 *     implement HAL.
 *
 * The required semantic boundary remains:
 *
 *     simulationExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum model
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
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A simulation target may contain HDL or hardware intent.
 *
 * This file does not define:
 *
 *     bus widths
 *     register widths
 *     device counts
 *     memory sizes
 *     topology sizes
 *     clock limits
 *     pipeline limits
 *     hardware models
 *
 * HDL/hardware semantic grammars remain the owners of their respective
 * structures.
 *
 * ============================================================================
 * AI / REASONING / LEARNING BOUNDARY
 * ============================================================================
 *
 * A simulation target may be:
 *
 *     a learned model
 *     a reasoning operation
 *     a knowledge system
 *     an adaptive computation
 *     a probabilistic computation
 *     a neural-symbolic computation
 *     an agent
 *
 * No AI-specific simulation keyword catalogue is introduced here.
 *
 * Those concepts remain composable through the normal expression system.
 *
 * ============================================================================
 * REPRODUCIBILITY
 * ============================================================================
 *
 * Deterministic/reproducible simulation is a semantic concern.
 *
 * The grammar therefore does not hard-code:
 *
 *     seed syntax
 *     random algorithm
 *     simulator implementation
 *     reproducibility mechanism
 *
 * A program may supply reproducibility configuration through ordinary
 * simulation options.
 *
 * Example:
 *
 *     simulate model with (reproducibility_policy)
 *
 * The semantic layer determines how that policy is realized.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Simulation may participate in adaptive execution.
 *
 * Examples of downstream decisions include:
 *
 *     detect
 *     evaluate
 *     select
 *     retry
 *     recover
 *     fallback
 *     adapt
 *
 * Those are not implemented by this grammar.
 *
 * They belong to:
 *
 *     grammar/execution/
 *     grammar/policies/
 *     semantic execution planning
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should represent this structure as one canonical
 * simulation-expression node, for example conceptually:
 *
 *     SimulationExpression {
 *         target,
 *         source,
 *         options,
 *         source_span
 *     }
 *
 * The exact Rust AST type name remains owned by the AST subsystem.
 *
 * The AST MUST preserve:
 *
 *     operation kind
 *     target expression
 *     optional source expression
 *     ordered options
 *     source locations
 *
 * It MUST NOT contain:
 *
 *     physical simulator handles
 *     backend instances
 *     machine identifiers
 *     hardware allocation
 *     runtime state
 *
 * ============================================================================
 * SEMANTIC MODEL CONTRACT
 * ============================================================================
 *
 * Semantic analysis should transform the AST into a simulation intent model
 * containing, as applicable:
 *
 *     target meaning
 *     simulation strategy
 *     source/input
 *     options
 *     effects
 *     capabilities
 *     resource requirements
 *     contracts
 *     policies
 *     provenance
 *     reproducibility requirements
 *     domain classification
 *
 * The semantic model remains target-independent.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The downstream path is:
 *
 *     SimulationExpression
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic simulation intent
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     classical semantics   quantum semantics
 *          |                    |
 *          |                    v
 *          |                quantum::ir
 *          |                    |
 *          +----------+---------+
 *                     |
 *                     v
 *             canonical semantic model
 *                     |
 *             optimization/lowering
 *                     |
 *             target realization
 *
 * HDL and hardware domains use their canonical semantic boundaries rather than
 * introducing another simulation IR here.
 *
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 *
 * The existing:
 *
 *     grammar/statements/simulate.g4
 *
 * remains the owner of:
 *
 *     simulate target;
 *
 * and:
 *
 *     simulate target {
 *         ...
 *     }
 *
 * It MUST NOT be replaced by this file.
 *
 * This file supplies the expression form:
 *
 *     simulate target
 *
 * and therefore allows simulation intent to occur inside larger expressions.
 *
 * Statement composition must prefer the dedicated simulation statement before
 * the generic expression statement where the source form is ambiguous.
 *
 * This ensures that structured simulation blocks remain owned by
 * statements/simulate.g4.
 *
 * ============================================================================
 * EXPRESSIONS INTEGRATION
 * ============================================================================
 *
 * `grammar/expressions/expressions.g4` must:
 *
 * 1. import:
 *
 *        SimulationExpressions
 *
 * 2. add:
 *
 *        simulationExpression
 *
 *    to the canonical `primaryExpression` alternatives.
 *
 * The required conceptual composition is:
 *
 *     primaryExpression
 *         :
 *             ...
 *           | quantumExpression
 *           | reasoningExpression
 *           | knowledgeExpression
 *           | uncertaintyExpression
 *           | simulationExpression
 *           | queryExpression
 *           | ...
 *
 * The exact ordering should remain stable and should not create a second
 * precedence hierarchy.
 *
 * ============================================================================
 * ROOT PARSER INTEGRATION
 * ============================================================================
 *
 * No direct change is required to:
 *
 *     grammar/Zamani.g4
 *
 * because it imports:
 *
 *     ZamaniParser
 *     ZamaniLexer
 *
 * and ZamaniParser already imports:
 *
 *     Expressions
 *
 * Therefore the dependency chain becomes:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Expressions
 *          |
 *          v
 *     SimulationExpressions
 *
 * ============================================================================
 * STATEMENT DISPATCH INTEGRATION
 * ============================================================================
 *
 * The repository currently has:
 *
 *     grammar/statements/simulate.g4
 *
 * but the universal statement dispatcher must compose it.
 *
 * `grammar/statements/statements.g4` therefore needs:
 *
 *     import StatementsSimulation
 *
 * and:
 *
 *     statement
 *         :
 *             ...
 *           | simulationStatement
 *           | ...
 *         ;
 *
 * This is a separate integration change from this expression file.
 *
 * It prevents the structured simulation statement from being swallowed by the
 * generic expression-statement path.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * No change to the effect grammar is required merely to parse this expression.
 *
 * Semantic analysis should classify simulation as an effect where the language
 * effect model requires it.
 *
 * Additional effects come from the target and context.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * No simulator catalogue belongs in this grammar.
 *
 * Resource and capability requirements are resolved through:
 *
 *     grammar/resources/
 *     semantic capability analysis
 *     semantic resource analysis
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Simulation options may carry policy expressions.
 *
 * Policy interpretation belongs to:
 *
 *     grammar/policies/
 *     grammar/security/
 *     execution policy semantics
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * The expression may appear inside contract conditions.
 *
 * Example conceptual usage:
 *
 *     requires capability("simulation");
 *
 *     ensures simulate(model);
 *
 * Contract validation remains owned by:
 *
 *     grammar/validation/
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Simulation results and transformations can contribute evidence and
 * provenance.
 *
 * Provenance is not encoded as parser state.
 *
 * The semantic/compiler layers attach:
 *
 *     source
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     reason
 *     evidence
 *     decision
 *     version
 *     timestamp
 *
 * ============================================================================
 * NEGATIVE / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level failures include:
 *
 *     missing simulation target
 *     malformed `from` clause
 *     malformed `with` clause
 *     missing closing parenthesis
 *     malformed option list
 *
 * Semantic failures include:
 *
 *     invalid simulation target
 *     unsupported simulation strategy
 *     missing capability
 *     insufficient resources
 *     prohibited effect
 *     violated policy
 *     invalid contract
 *     unsupported target realization
 *
 * A target/resource/capability failure MUST NOT be reported as a syntax error.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces one new expression category:
 *
 *     simulationExpression
 *
 * It does not rename or remove:
 *
 *     expression
 *     primaryExpression
 *     simulationStatement
 *
 * Existing statement-level simulation syntax remains compatible.
 *
 * Existing programs containing:
 *
 *     simulate target;
 *
 * remain owned by:
 *
 *     statements/simulate.g4
 *
 * The expression form is an additive capability.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_SIMULATIONS
 *     MAX_SIMULATION_DEPTH
 *     MAX_SCENARIOS
 *     MAX_EVENTS
 *     MAX_SAMPLES
 *     MAX_STATES
 *     MAX_MODELS
 *     MAX_RESOURCES
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
 * No equivalent hidden capacity constant may be introduced.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * REQUIRED POSITIVE TESTS
 *
 *     simulate model
 *
 *     simulate model from input
 *
 *     simulate model with (scenario)
 *
 *     simulate model from input with (scenario, policy)
 *
 *     simulate quantum_program
 *
 *     simulate hardware_model
 *
 *     simulate model(input)
 *
 *     simulate pipeline(data)
 *
 *     simulate model with (requires_capability)
 *
 *     simulate model with (resource_requirement)
 *
 *     simulate model with (reproducibility_policy)
 *
 *     simulate model with (uncertainty_model)
 *
 *     simulate model with (provenance_context)
 *
 * REQUIRED COMPOSITION TESTS
 *
 *     outer(simulate model)
 *
 *     simulate model + other_value
 *
 *     condition ? simulate model : fallback
 *
 *     collection(simulate model)
 *
 *     function(simulate model)
 *
 *     requires simulate model
 *
 *     ensures simulate model
 *
 *     property(simulate model)
 *
 * REQUIRED CROSS-DOMAIN TESTS
 *
 *     simulate classical_model
 *
 *     simulate quantum_program
 *
 *     simulate hdl_model
 *
 *     simulate hardware_model
 *
 *     simulate ai_model(input)
 *
 *     simulate distributed_system
 *
 *     simulate hybrid_program
 *
 * REQUIRED NEGATIVE TESTS
 *
 *     simulate
 *
 *     simulate from input
 *
 *     simulate model from
 *
 *     simulate model with
 *
 *     simulate model with (
 *
 *     simulate model with (a,)
 *
 *     simulate model from input with (
 *
 * REQUIRED SCALABILITY TESTS
 *
 *     deeply nested expressions;
 *     large option lists;
 *     large source expressions;
 *     large target expressions;
 *     nested simulation expressions;
 *     simulation embedded in generic expressions;
 *     cross-domain expressions.
 *
 * The tests must verify that no grammar-level capacity constant exists.
 *
 * REQUIRED DETERMINISM TEST
 *
 * Parsing the same source with the same grammar and lexer configuration must
 * produce equivalent parse-tree structure.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] `SimulationExpressions` is the sole grammar identity.
 *
 * [ ] `simulationExpression` is the sole public expression entry point.
 *
 * [ ] No lexer rules exist here.
 *
 * [ ] `Expressions` is NOT imported here.
 *
 * [ ] The canonical `expression` rule is reused as the operand boundary.
 *
 * [ ] `simulate` supports an expression target.
 *
 * [ ] optional `from` source is supported.
 *
 * [ ] optional `with (...)` context is supported.
 *
 * [ ] options are open-ended expressions.
 *
 * [ ] no simulation algorithm is enumerated.
 *
 * [ ] no simulator implementation is enumerated.
 *
 * [ ] no hardware capacity is encoded.
 *
 * [ ] no quantum gate catalogue is encoded.
 *
 * [ ] no physical target is encoded.
 *
 * [ ] no IR is created.
 *
 * [ ] no runtime execution exists.
 *
 * [ ] no semantic predicates exist.
 *
 * [ ] deterministic parsing is preserved.
 *
 * [ ] AST integration is defined.
 *
 * [ ] semantic integration is defined.
 *
 * [ ] effect integration is defined.
 *
 * [ ] capability integration is defined.
 *
 * [ ] resource integration is defined.
 *
 * [ ] contract integration is defined.
 *
 * [ ] policy integration is defined.
 *
 * [ ] provenance integration is defined.
 *
 * [ ] statement-level simulation remains owned by
 *     `grammar/statements/simulate.g4`.
 *
 * [ ] `expressions.g4` imports this grammar.
 *
 * [ ] `expressions.g4` adds `simulationExpression` to `primaryExpression`.
 *
 * [ ] `statements.g4` composes `StatementsSimulation`.
 *
 * [ ] `ZamaniParser.g4` continues to compose `Expressions` and `Statements`.
 *
 * [ ] `Zamani.g4` requires no direct modification.
 *
 * [ ] positive tests exist.
 *
 * [ ] negative tests exist.
 *
 * [ ] cross-domain tests exist.
 *
 * [ ] scalability tests exist.
 *
 * [ ] determinism tests exist.
 *
 * [ ] compatibility tests exist.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar SimulationExpressions;

options {
    tokenVocab = ZamaniLexer;
};


/*
 * ============================================================================
 * 1. PUBLIC EXPRESSION
 * ============================================================================
 *
 * The expression-level simulation construct.
 *
 * No semicolon is consumed here.
 *
 * Statement-level termination remains owned by statement grammars.
 */

simulationExpression
    : simulationOperation
    ;


/*
 * ============================================================================
 * 2. SIMULATION OPERATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     simulate target
 *
 *     simulate target from source
 *
 *     simulate target with (option)
 *
 *     simulate target from source with (option, option)
 *
 * `from` and `with` are explicit structural delimiters, so the canonical
 * expression parser can determine where the target ends without introducing a
 * second precedence hierarchy.
 */

simulationOperation
    : SIMULATE
      simulationTarget
      simulationSourceClause?
      simulationContextClause?
    ;


/*
 * ============================================================================
 * 3. TARGET
 * ============================================================================
 *
 * The target is the canonical Zamani expression.
 *
 * This rule deliberately references the parent expression hierarchy.
 *
 * It does not reproduce expression precedence.
 */

simulationTarget
    : simulationOperand
    ;


/*
 * ============================================================================
 * 4. SOURCE
 * ============================================================================
 */

simulationSourceClause
    : FROM
      simulationOperand
    ;


/*
 * ============================================================================
 * 5. CONTEXT
 * ============================================================================
 */

simulationContextClause
    : WITH
      LPAREN
      simulationOptionList?
      RPAREN
    ;


/*
 * ============================================================================
 * 6. OPTION LIST
 * ============================================================================
 *
 * There is no fixed number of options.
 */

simulationOptionList
    : simulationOption
      (
          COMMA
          simulationOption
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. OPTION
 * ============================================================================
 *
 * Every option is an ordinary Zamani expression.
 */

simulationOption
    : simulationOperand
    ;


/*
 * ============================================================================
 * 8. CANONICAL EXPRESSION INTEGRATION BOUNDARY
 * ============================================================================
 *
 * This is intentionally an alias boundary.
 *
 * The expression-composition root supplies the canonical `expression` rule.
 *
 * This grammar does not import `Expressions`.
 *
 * This avoids:
 *
 *     Expressions -> SimulationExpressions -> Expressions
 *
 * while still allowing:
 *
 *     expression
 *         ->
 *     primaryExpression
 *         ->
 *     simulationExpression
 *         ->
 *     simulationOperand
 *         ->
 *     expression
 *
 * The resulting recursion is intentional: a simulation operation is itself an
 * expression and may therefore contain normal expressions as its target,
 * source and options.
 */

simulationOperand
    : expression
    ;


/*
 * ============================================================================
 * END OF GRAMMAR RULES
 * ============================================================================
 */