/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/simulate.g4
 *
 * Grammar:
 *     StatementsSimulation
 *
 * Status:
 *     PRODUCTION-READY MODULAR STATEMENT GRAMMAR
 *
 * Purpose:
 *     Defines portable source-level simulation intent as a Zamani statement.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, hardware access, runtime calls, or
 *     unsafe implementation requirements.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * canonical lexer
 *      |
 *      v
 * canonical parser
 *      |
 *      v
 * statements/simulate.g4
 *      |
 *      v
 * domain-neutral AST
 *      |
 *      v
 * structural validation
 *      |
 *      v
 * semantic analysis
 *      |
 *      +----------------------+-----------------------+
 *      |                      |                       |
 *      v                      v                       v
 * classical semantics   quantum semantics       HDL/hardware
 *                              |                   semantics
 *                              v
 *                         quantum::ir
 *      |                      |                       |
 *      +----------------------+-----------------------+
 *                             |
 *                             v
 *                    canonical semantic model
 *                             |
 *                 optimization / lowering
 *                             |
 *                  routing / scheduling
 *                             |
 *                 resilience / QEC / ZQN
 *                             |
 *                            HAL
 *                             |
 *                       target realization
 *
 * ============================================================================
 * CORE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the source-level `simulate` statement;
 *     - the simulation target expression;
 *     - the optional simulation body;
 *     - simulation body item composition;
 *     - generic simulation clauses;
 *     - simulation bindings;
 *     - nested simulation statements;
 *     - simulation expression statements;
 *     - source-level simulation intent structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - simulation algorithms;
 *     - numerical solvers;
 *     - event kernels;
 *     - waveform engines;
 *     - random-number generation;
 *     - random seeds as runtime behavior;
 *     - quantum state-vector simulation;
 *     - quantum trajectory simulation;
 *     - HDL simulation;
 *     - classical simulation engines;
 *     - AI model simulation;
 *     - hardware models;
 *     - target discovery;
 *     - resource allocation;
 *     - scheduling algorithms;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - physical device selection;
 *     - vendor APIs;
 *     - backend implementation;
 *     - AST construction;
 *     - semantic execution.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Repository architecture:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/spec/
 *          |
 *          v
 *     grammar/specification/
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     grammar/statements/
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *
 * `grammar/Zamani.g4` remains the complete-language composition root.
 *
 * `grammar/antlr/ZamaniParser.g4` remains the canonical parser composition
 * root.
 *
 * `grammar/statements/` owns statement-level syntax.
 *
 * This file owns only simulation statement syntax.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/*
 *     grammar/expressions/*
 *     canonical Core expression/name/identifier rules
 *
 * EXPORTS:
 *
 *     simulationStatement
 *     simulationDeclaration
 *     simulationTarget
 *     simulationBody
 *     simulationItem
 *     simulationClause
 *     simulationBinding
 *     simulationNestedStatement
 *     simulationExpressionStatement
 *     simulationBlock
 *
 * CONSUMED_BY:
 *
 *     grammar/statements/statements.g4
 *     grammar/statements/statement.g4
 *     grammar/statements/domains.g4
 *     grammar/antlr/ZamaniParser.g4
 *
 * AST_OWNER:
 *
 *     frontend AST subsystem.
 *
 *     This grammar supplies parser structure only.
 *
 * SEMANTIC_OWNER:
 *
 *     execution/simulation semantic subsystem.
 *
 * IR_OWNER:
 *
 *     No IR is created here.
 *
 *     Classical simulation lowers through the canonical classical semantic
 *     representation.
 *
 *     Quantum simulation lowers through semantic quantum representation and,
 *     where quantum computation is involved, through canonical `quantum::ir`.
 *
 *     HDL simulation lowers through the HDL semantic subsystem.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/simulation/
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
 * The canonical lexer already provides:
 *
 *     SIMULATE
 *
 * This grammar therefore MUST NOT define:
 *
 *     K_SIMULATE
 *     SIMULATION
 *     K_SIMULATION
 *
 * or another simulation-specific lexer.
 *
 * Simulation concepts such as:
 *
 *     model
 *     scenario
 *     stimulus
 *     observation
 *     expectation
 *     checkpoint
 *     sampling
 *     duration
 *     timeout
 *     tolerance
 *     seed
 *     initial
 *     termination
 *     waveform
 *     trace
 *     policy
 *     resources
 *     capabilities
 *
 * remain ordinary identifiers unless the language specification later
 * establishes a genuine lexical requirement.
 *
 * This is deliberate.
 *
 * It prevents the simulation subsystem from becoming a closed vocabulary and
 * permits future simulation concepts without modifying the universal parser.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Simulation is an execution/analysis intent, not a list of simulator types.
 *
 * The grammar therefore does NOT enumerate:
 *
 *     classical simulator
 *     quantum simulator
 *     HDL simulator
 *     GPU simulator
 *     FPGA simulator
 *     distributed simulator
 *     event simulator
 *     cycle simulator
 *     numerical solver
 *     waveform engine
 *     vendor simulator
 *     hardware model
 *     model framework
 *
 * Those are semantic properties, capabilities, dialects, libraries, or
 * backend implementations.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Simulation syntax MUST remain independent of physical realization.
 *
 * A source program may therefore describe simulation intent that is later
 * realized using:
 *
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC-associated infrastructure
 *     accelerator
 *     QPU simulator
 *     classical quantum simulator
 *     distributed system
 *     cluster
 *     supercomputer
 *     cloud
 *     embedded target
 *     future computational substrate
 *
 * without requiring the source grammar to be rewritten.
 *
 * This grammar MUST NOT impose language-level limits on:
 *
 *     qubits
 *     simulation states
 *     simulation scenarios
 *     simulation steps
 *     simulation events
 *     observations
 *     checkpoints
 *     samples
 *     variables
 *     models
 *     nested simulations
 *     target count
 *     resource quantity
 *     memory
 *     processors
 *     accelerators
 *     devices
 *     nodes
 *     topology
 *     tensor dimensions
 *     tensor rank
 *
 * Any actual limit is downstream:
 *
 *     compiler resources
 *     runtime resources
 *     target capabilities
 *     resource negotiation
 *     deployment policy
 *     operating-system limits
 *     explicitly requested user constraints
 *
 * ============================================================================
 * IMPORTANT SEMANTIC DISTINCTION
 * ============================================================================
 *
 * `simulate` means:
 *
 *     "perform or request a simulation-oriented realization of this
 *      computation/model/intent"
 *
 * It does NOT mean:
 *
 *     "execute a particular simulator implementation."
 *
 * Therefore:
 *
 *     simulate model;
 *
 * is source intent.
 *
 * Whether that becomes:
 *
 *     - an interpreter;
 *     - a compiled simulator;
 *     - a numerical solver;
 *     - a quantum simulator;
 *     - an HDL simulator;
 *     - a distributed simulation;
 *     - an accelerated simulation;
 *     - a formal/simulation-assisted analysis;
 *
 * is a semantic/compiler decision.
 *
 * ============================================================================
 * UNIVERSAL SIMULATION FORM
 * ============================================================================
 *
 * Canonical minimal form:
 *
 *     simulate target;
 *
 * Canonical structured form:
 *
 *     simulate target {
 *         scenario: scenario_value;
 *         stimulus: input_value;
 *         observe: output_value;
 *         expect: expected_value;
 *     }
 *
 * Clause names are semantic identifiers.
 *
 * They are intentionally NOT keywords.
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * The simulation target is a canonical Zamani expression.
 *
 * Examples:
 *
 *     simulate model;
 *     simulate circuit;
 *     simulate system;
 *     simulate quantum_program;
 *     simulate hardware_model;
 *     simulate pipeline(data);
 *     simulate custom::model;
 *
 * The grammar does not decide whether the target is:
 *
 *     - classical;
 *     - quantum;
 *     - HDL;
 *     - hybrid;
 *     - AI;
 *     - distributed;
 *     - hardware;
 *     - data;
 *     - networking;
 *     - another future domain.
 *
 * Semantic analysis determines that.
 *
 * ============================================================================
 * STRUCTURED SIMULATION BODY
 * ============================================================================
 *
 * The body is an open collection of simulation-intent items.
 *
 * It intentionally permits:
 *
 *     zero or more items.
 *
 * There is no fixed minimum or maximum.
 *
 * This allows:
 *
 *     simulate model {}
 *
 * as a valid syntactic form when the language specification permits an empty
 * simulation configuration.
 *
 * ============================================================================
 * ITEM MODEL
 * ============================================================================
 *
 * A simulation body may contain:
 *
 *     1. named simulation clauses;
 *     2. simulation bindings;
 *     3. nested simulation statements;
 *     4. ordinary expression statements;
 *     5. nested simulation blocks.
 *
 * The grammar does not hard-code the meaning of the clause name.
 *
 * Examples:
 *
 *     stimulus: input;
 *     observe: output;
 *     expect: result;
 *     duration: duration_value;
 *     sampling: sampling_policy;
 *     checkpoint: checkpoint_value;
 *     tolerance: tolerance_value;
 *     policy: simulation_policy;
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * This file imports Expressions because simulation targets, clause values and
 * expression statements must use the canonical Zamani expression language.
 *
 * This file MUST NOT define another expression grammar.
 *
 * ============================================================================
 */

parser grammar StatementsSimulation;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * 1. PUBLIC STATEMENT ENTRY POINT
 * ============================================================================
 *
 * This is the only universal statement rule owned by this file.
 *
 * `statements.g4` should consume:
 *
 *     simulationStatement
 *
 * rather than duplicating this grammar.
 *
 * ============================================================================
 */

simulationStatement
    : simulationDeclaration
    ;


/*
 * ============================================================================
 * 2. SIMULATION DECLARATION
 * ============================================================================
 *
 * Supported forms:
 *
 *     simulate target;
 *
 *     simulate target
 *
 *     simulate target {
 *         ...
 *     }
 *
 * The semicolon is optional when a structured body terminates the statement.
 *
 * A simple simulation statement should normally use:
 *
 *     simulate target;
 *
 * while:
 *
 *     simulate target { ... }
 *
 * provides structured intent.
 *
 * ============================================================================
 */

simulationDeclaration
    : SIMULATE
      simulationTarget
      simulationBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 3. SIMULATION TARGET
 * ============================================================================
 *
 * The target is an ordinary Zamani expression.
 *
 * This intentionally reuses the canonical expression language.
 *
 * ============================================================================
 */

simulationTarget
    : expression
    ;


/*
 * ============================================================================
 * 4. SIMULATION BODY
 * ============================================================================
 *
 * A body contains zero or more simulation items.
 *
 * No universal cardinality is imposed.
 *
 * ============================================================================
 */

simulationBody
    : LBRACE
      simulationItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. SIMULATION ITEM
 * ============================================================================
 *
 * The item dispatcher is intentionally small.
 *
 * Detailed semantics are delegated to the semantic layer.
 *
 * ============================================================================
 */

simulationItem
    : simulationClause
    | simulationBinding
    | simulationNestedStatement
    | simulationExpressionStatement
    | simulationBlock
    ;


/*
 * ============================================================================
 * 6. GENERIC SIMULATION CLAUSE
 * ============================================================================
 *
 * Canonical form:
 *
 *     name: expression;
 *
 * Examples:
 *
 *     stimulus: input;
 *     observe: output;
 *     expect: expected;
 *     model: model_value;
 *     scenario: scenario_value;
 *     duration: duration_value;
 *     timeout: timeout_value;
 *     sampling: sampling_policy;
 *     checkpoint: checkpoint_value;
 *     initial: initial_state;
 *     termination: termination_condition;
 *     tolerance: tolerance_value;
 *     policy: policy_value;
 *     capability: required_capability;
 *     resource: required_resource;
 *     provenance: provenance_value;
 *
 * All names remain identifiers.
 *
 * The semantic subsystem determines whether a particular clause is valid for
 * the selected target and context.
 *
 * ============================================================================
 */

simulationClause
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 7. SIMULATION BINDING
 * ============================================================================
 *
 * Canonical form:
 *
 *     name = expression;
 *
 * Examples:
 *
 *     parameter = value;
 *     duration = required_duration;
 *     tolerance = acceptable_error;
 *     state = initial_state;
 *
 * This is configuration/data syntax.
 *
 * It does not introduce a second variable declaration system.
 *
 * If the semantic meaning is a declaration rather than simulation
 * configuration, semantic analysis must route it to the appropriate existing
 * binding/declaration model.
 *
 * ============================================================================
 */

simulationBinding
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. NESTED SIMULATION
 * ============================================================================
 *
 * Nested simulations permit hierarchical simulation descriptions.
 *
 * Example:
 *
 *     simulate system {
 *         simulate subsystem {
 *             stimulus: input;
 *         }
 *     }
 *
 * This has no grammar-level nesting limit.
 *
 * Semantic analysis may impose context-dependent restrictions where required,
 * but those restrictions are not universal parser limits.
 *
 * ============================================================================
 */

simulationNestedStatement
    : simulationDeclaration
    ;


/*
 * ============================================================================
 * 9. EXPRESSION STATEMENT
 * ============================================================================
 *
 * Simulation operations remain ordinary Zamani expressions.
 *
 * Examples:
 *
 *     drive(signal, value);
 *     observe(signal);
 *     sample(stream);
 *     checkpoint(state);
 *     record(trace);
 *     compare(actual, expected);
 *     reset(model);
 *     collect(result);
 *
 * None of these operation names are reserved by this grammar.
 *
 * This keeps simulation extensible through libraries, semantic operations,
 * dialects and capabilities.
 *
 * ============================================================================
 */

simulationExpressionStatement
    : expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. NESTED STRUCTURED BLOCK
 * ============================================================================
 *
 * A generic nested block is useful for future simulation domains and semantic
 * grouping without introducing another keyword vocabulary.
 *
 * Example:
 *
 *     scenario {
 *         stimulus: input;
 *         observe: output;
 *     }
 *
 * The identifier determines semantic meaning downstream.
 *
 * ============================================================================
 */

simulationBlock
    : identifier
      LBRACE
      simulationItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 11. SIMULATION TARGET COMPOSITION
 * ============================================================================
 *
 * Because simulationTarget delegates to expression, targets may be:
 *
 *     identifiers
 *     qualified names
 *     calls
 *     indexed values
 *     expressions
 *     constructed values
 *     references
 *     domain values
 *     model handles
 *     circuit values
 *     hardware descriptions
 *     symbolic values
 *
 * according to the canonical expression/type systems.
 *
 * This file does not create a simulation-specific target type.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. SIMULATION RESOURCE INTENT
 * ============================================================================
 *
 * Resource clauses are deliberately represented by the generic clause form.
 *
 * Examples:
 *
 *     resource: required_resources;
 *     capability: required_capability;
 *
 * or, where canonical expressions support it:
 *
 *     requirement: resource_requirement;
 *
 * Resource semantics remain owned by:
 *
 *     grammar/resources/
 *     grammar/core/requirements.g4
 *     grammar/core/capabilities.g4
 *     execution semantic analysis
 *
 * This grammar MUST NOT evaluate availability.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 13. EFFECT INTEGRATION
 * ============================================================================
 *
 * Simulation itself may be modeled as an effect by the semantic subsystem.
 *
 * This file does not create an independent effect vocabulary.
 *
 * The path is:
 *
 *     simulationStatement
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     effect analysis
 *          |
 *          v
 *     canonical effect model
 *
 * A simulation may interact with:
 *
 *     IO
 *     randomness
 *     time
 *     networking
 *     distributed execution
 *     external models
 *     native/foreign interfaces
 *     quantum measurement semantics
 *
 * Those effects remain owned by grammar/effects/ and its semantic subsystem.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. POLICY INTEGRATION
 * ============================================================================
 *
 * Simulation policy is semantic intent.
 *
 * A clause such as:
 *
 *     policy: simulation_policy;
 *
 * does not implement policy evaluation.
 *
 * Policy semantics belong to:
 *
 *     grammar/core/policies.g4
 *     grammar/security/
 *     execution policy analysis
 *
 * Policies may govern:
 *
 *     reproducibility
 *     determinism
 *     resource use
 *     permitted effects
 *     sandboxing
 *     fallback
 *     adaptation
 *     target selection
 *     data access
 *     external interaction
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Simulation may produce or consume provenance.
 *
 * Provenance is NOT encoded as a closed parser vocabulary.
 *
 * Examples:
 *
 *     provenance: source_record;
 *     evidence: evidence_record;
 *     record: trace_record;
 *
 * Semantic provenance belongs to the repository-wide provenance model.
 *
 * The simulation grammar only preserves the source structure required by the
 * AST to represent such intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     canonical lexer
 *     canonical parser grammar
 *     selected language/dialect configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     runtime state
 *     network state
 *     filesystem state
 *     current time
 *     randomness
 *     target selection
 *     simulator availability
 *     resource availability
 *
 * Therefore:
 *
 *     identical source + identical grammar configuration
 *
 * MUST produce equivalent parser structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. ERROR BOUNDARY
 * ============================================================================
 *
 * This grammar is responsible only for syntactic errors.
 *
 * Parser diagnostics include:
 *
 *     missing `simulate`
 *     missing target
 *     malformed target expression
 *     malformed clause
 *     malformed binding
 *     malformed block
 *     missing delimiter where required
 *     unexpected token
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     target cannot be simulated
 *     capability unavailable
 *     resource insufficient
 *     policy prohibits simulation
 *     target type incompatible with simulation
 *     simulation effect not permitted
 *     quantum operation unsupported by selected realization
 *     HDL model unavailable
 *     external model inaccessible
 *
 * Such conditions MUST NOT be converted into parser errors.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. DOMAIN INTEGRATION
 * ============================================================================
 *
 * This single statement syntax can describe simulation of:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware/software co-design
 *     AI models
 *     learned systems
 *     data pipelines
 *     distributed computation
 *     networked computation
 *     accelerator computation
 *     nano/molecular computation
 *     future computational domains
 *
 * The grammar does not create domain-specific simulation statements.
 *
 * For example:
 *
 *     simulate classical_model;
 *
 *     simulate quantum_circuit;
 *
 *     simulate hybrid_algorithm;
 *
 *     simulate hardware_model;
 *
 *     simulate distributed_system;
 *
 * are all structurally the same kind of statement.
 *
 * Domain semantics are determined downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. QUANTUM INTEGRATION
 * ============================================================================
 *
 * This file MUST NOT enumerate:
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
 *
 * as simulation-specific grammar alternatives.
 *
 * Quantum operations belong to grammar/quantum/.
 *
 * If a simulation target represents quantum computation, the semantic path is:
 *
 *     simulate target
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     simulation/lowering/optimization
 *          |
 *          v
 *     target realization
 *
 * The simulation grammar does not create a quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. HDL INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/hdl/simulation.g4
 *
 * owns HDL-specific simulation-intent syntax.
 *
 * This file does NOT duplicate that grammar.
 *
 * The distinction is:
 *
 *     statements/simulate.g4
 *         =
 *     universal simulation statement
 *
 *     hdl/simulation.g4
 *         =
 *     HDL-specific simulation constructs
 *
 * An HDL simulation target can therefore be represented by:
 *
 *     simulate hardware_model;
 *
 * while HDL-specific simulation details remain owned by the HDL subsystem.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. EXECUTION INTEGRATION
 * ============================================================================
 *
 * Existing execution grammar owns execution context and execution intent.
 *
 * This file does not duplicate:
 *
 *     executionContext
 *     target selection
 *     placement
 *     scheduling
 *     dispatch
 *     lifecycle
 *     retry
 *     recovery
 *
 * Simulation may consume an execution context downstream.
 *
 * Conceptual pipeline:
 *
 *     simulate target
 *          |
 *          v
 *     simulation intent
 *          |
 *          +----> execution context
 *          |
 *          +----> resource requirements
 *          |
 *          +----> capabilities
 *          |
 *          +----> policies
 *          |
 *          v
 *     execution planning
 *
 * No target is selected here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. RESOURCE SCALABILITY
 * ============================================================================
 *
 * Simulation syntax is intentionally symbolic.
 *
 * It does not contain:
 *
 *     MAX_SIMULATION_STEPS
 *     MAX_SIMULATION_EVENTS
 *     MAX_SAMPLES
 *     MAX_SCENARIOS
 *     MAX_CHECKPOINTS
 *     MAX_MODELS
 *     MAX_TARGETS
 *     MAX_RESOURCES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * No such constants are permitted in this grammar.
 *
 * A source program may express arbitrary resource quantities through ordinary
 * Zamani expressions and semantic resource requirements.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. NO PHYSICAL TARGETING
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     GPU 0
 *     CPU 3
 *     QPU 2
 *     device 7
 *     node 12
 *
 * as special parser constructs.
 *
 * Physical targeting, when explicitly supported by a non-portable dialect,
 * belongs to target/deployment/dialect semantics.
 *
 * The portable simulation statement remains target-neutral.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. NO SIMULATOR ENUMERATION
 * ============================================================================
 *
 * This grammar must never evolve into:
 *
 *     simulator:
 *         classicalSimulator
 *       | quantumSimulator
 *       | hdlSimulator
 *       | gpuSimulator
 *       | ...
 *
 * Such enumeration creates a language ceiling.
 *
 * Instead:
 *
 *     simulate target
 *
 * expresses intent.
 *
 * The semantic/compiler layer determines the realization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * A future simulation concept should normally be expressible without editing
 * this file.
 *
 * For example:
 *
 *     simulate system {
 *         digital_twin: model;
 *         intervention: action;
 *         counterfactual: condition;
 *         uncertainty: distribution;
 *         evidence: observation;
 *         provenance: record;
 *     }
 *
 * No new parser rule is required because these are ordinary identifiers
 * participating in the generic clause structure.
 *
 * A new keyword should only be introduced when lexical distinction is
 * genuinely required by the normative language specification.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. CONTROLLED ADAPTATION
 * ============================================================================
 *
 * Simulation may participate in adaptive execution.
 *
 * For example:
 *
 *     simulate system {
 *         policy: adaptive_policy;
 *         feedback: feedback_source;
 *         fallback: alternative;
 *     }
 *
 * This grammar does not authorize adaptation.
 *
 * Authorization, capability checks, effects, policy checks and provenance
 * belong to semantic analysis and the execution/security subsystems.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility information may be represented through ordinary clauses.
 *
 * Example:
 *
 *     simulate model {
 *         reproducibility: policy;
 *         seed: seed_value;
 *         provenance: provenance_record;
 *     }
 *
 * `seed` is not interpreted here.
 *
 * The grammar does not perform random-number generation and does not guarantee
 * deterministic simulation merely because a seed is syntactically present.
 *
 * Determinism/reproducibility semantics belong downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. SANDBOX / SECURITY
 * ============================================================================
 *
 * Simulation may execute or inspect models that interact with external
 * resources.
 *
 * Security intent may therefore be represented through generic clauses or
 * existing security/policy constructs.
 *
 * This file does NOT implement sandboxing.
 *
 * Security semantics belong to:
 *
 *     grammar/security/
 *     grammar/core/policies.g4
 *     effect analysis
 *     capability analysis
 *     execution policy
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve at least:
 *
 *     keyword/source span
 *     target expression
 *     optional simulation body
 *     ordered simulation items
 *     clause names
 *     clause expressions
 *     binding names
 *     binding expressions
 *     nested simulation statements
 *     expression statements
 *     source locations
 *
 * The AST must NOT be forced to contain:
 *
 *     simulator vendor
 *     physical device
 *     physical qubit
 *     CPU identifier
 *     GPU identifier
 *     FPGA identifier
 *     routing decision
 *     schedule
 *     calibration
 *     QEC configuration
 *     ZQN representation
 *
 * unless a later semantic phase derives those artifacts.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. resolve the simulation target;
 *     2. determine the target's computational domain;
 *     3. validate target type/meaning;
 *     4. interpret simulation clauses;
 *     5. resolve resource requirements;
 *     6. resolve capabilities;
 *     7. resolve effects;
 *     8. validate policies;
 *     9. establish provenance requirements;
 *    10. determine reproducibility requirements;
 *    11. construct the canonical simulation intent;
 *    12. lower to the appropriate semantic/domain representation.
 *
 * This grammar performs none of those operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * The canonical path is:
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
 *     semantic simulation intent
 *       |
 *       +----------------------+---------------------+
 *       |                      |                     |
 *       v                      v                     v
 *   classical              quantum::ir          HDL/hardware
 *       |                      |                     |
 *       +----------------------+---------------------+
 *                              |
 *                              v
 *                     optimization/lowering
 *                              |
 *                    execution planning
 *                              |
 *                     target realization
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces the stable parser rule:
 *
 *     simulationStatement
 *
 * Existing simulation functionality elsewhere in the repository remains
 * independently owned.
 *
 * In particular:
 *
 *     grammar/hdl/simulation.g4
 *
 * MUST NOT be copied into this file.
 *
 * Existing source forms that already use:
 *
 *     simulate target;
 *
 * should map naturally to this statement grammar where the canonical parser
 * composition enables it.
 *
 * If historical simulation syntax conflicts with this form, compatibility
 * handling belongs in the compatibility/dialect layer rather than by
 * contaminating this portable grammar with legacy alternatives.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. INTEGRATION WITH statements.g4
 * ============================================================================
 *
 * `grammar/statements/statements.g4` is the composition owner for the
 * statement subsystem.
 *
 * It should import/compose this grammar exactly once and expose:
 *
 *     simulationStatement
 *
 * through its universal statement dispatcher.
 *
 * Conceptually:
 *
 *     statement
 *         |
 *         +--> bindingStatement
 *         +--> controlFlowStatement
 *         +--> loopStatement
 *         +--> matchStatement
 *         +--> assertionStatement
 *         +--> concurrencyStatement
 *         +--> resourceStatement
 *         +--> effectStatement
 *         +--> domainStatement
 *         +--> simulationStatement
 *         +--> ...
 *
 * This file MUST NOT redefine `statement`.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. INTEGRATION WITH domains.g4
 * ============================================================================
 *
 * `domains.g4` may expose execution-domain constructs.
 *
 * A simulation statement should remain a universal statement rather than
 * becoming a new domain family.
 *
 * Therefore:
 *
 *     simulate quantum_program;
 *
 * does not require:
 *
 *     quantum simulation statement
 *
 * at the universal statement layer.
 *
 * The target's domain is discovered semantically.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. INTEGRATION WITH execution/
 * ============================================================================
 *
 * Simulation intent is consumed by execution planning.
 *
 * The execution subsystem may use:
 *
 *     executionContext
 *     runtime capabilities
 *     resource requirements
 *     execution policies
 *     parallel execution
 *     distributed execution
 *
 * without this grammar importing those implementations.
 *
 * This keeps the dependency direction:
 *
 *     statements/simulate.g4
 *             |
 *             v
 *       semantic model
 *             |
 *             v
 *       execution subsystem
 *
 * rather than:
 *
 *     execution subsystem
 *             |
 *             v
 *       statement grammar
 *
 * which would create architectural coupling.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. INTEGRATION WITH quantum/
 * ============================================================================
 *
 * If the simulation target resolves to quantum computation:
 *
 *     simulation target
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * The simulation statement itself does not create a quantum circuit,
 * enumerate operations, allocate qubits, route qubits, schedule operations,
 * apply QEC, or select a QPU.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. INTEGRATION WITH HDL/
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/hdl/simulation.g4
 *
 * remains the owner of HDL-specific simulation constructs.
 *
 * The universal statement can target an HDL model:
 *
 *     simulate hardware_model;
 *
 * while HDL-specific clauses continue to be interpreted by the HDL semantic
 * subsystem.
 *
 * This prevents two independent HDL simulation grammars from emerging.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. INTEGRATION WITH CLASSICAL/
 * ============================================================================
 *
 * Classical simulation targets are ordinary Zamani expressions.
 *
 * The semantic layer determines whether a target represents:
 *
 *     program
 *     function
 *     model
 *     algorithm
 *     state transition system
 *     numerical model
 *     data pipeline
 *     other classical computation
 *
 * No CPU architecture is assumed.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. INTEGRATION WITH HYBRID/
 * ============================================================================
 *
 * A simulation target may represent a hybrid computation combining:
 *
 *     classical
 *     quantum
 *     accelerator
 *     hardware
 *     AI
 *     distributed
 *     networking
 *
 * The universal syntax remains unchanged.
 *
 * The semantic layer constructs the appropriate hybrid representation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. INTEGRATION WITH AI / LEARNING / REASONING
 * ============================================================================
 *
 * Simulation may be used for:
 *
 *     model evaluation
 *     counterfactual analysis
 *     causal intervention
 *     learned-system testing
 *     policy evaluation
 *     planning
 *     uncertainty analysis
 *     agent evaluation
 *
 * None of these require dedicated simulation keywords.
 *
 * They can be represented through:
 *
 *     target expressions
 *     generic clauses
 *     canonical AI semantic constructs
 *     policies
 *     provenance
 *     evidence
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Simulation can generate evidence for later reasoning or validation.
 *
 * Provenance should be preserved from:
 *
 *     source
 *       |
 *       v
 *     simulation intent
 *       |
 *       v
 *     simulation result
 *       |
 *       v
 *     derived artifact
 *
 * The grammar only preserves the source structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. INTEGRATION WITH CONTRACTS
 * ============================================================================
 *
 * Simulation may be used to evaluate:
 *
 *     requires
 *     ensures
 *     invariant
 *     property
 *     guarantee
 *     assumption
 *
 * The grammar does not duplicate contract syntax.
 *
 * Existing validation/contract grammars remain authoritative.
 *
 * Simulation results may become evidence for contract verification during
 * semantic/verification phases.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. INTEGRATION WITH ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Simulation may be used before or during adaptive execution:
 *
 *     simulate candidate;
 *     evaluate result;
 *     select realization;
 *     execute;
 *
 * This grammar owns only the simulation intent.
 *
 * Selection, fallback, retry and recovery remain execution semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. INTEGRATION WITH SANDBOXING
 * ============================================================================
 *
 * A simulation can be subject to:
 *
 *     effect restrictions
 *     resource restrictions
 *     capability restrictions
 *     filesystem restrictions
 *     network restrictions
 *     native/foreign-call restrictions
 *
 * Those restrictions are expressed and enforced by the existing security,
 * capability and effect systems.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. PERFORMANCE / PARSER SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally uses:
 *
 *     expression
 *     identifier
 *     repetition
 *     structured blocks
 *
 * rather than large closed alternatives.
 *
 * This avoids grammar growth proportional to the number of:
 *
 *     simulators
 *     models
 *     hardware targets
 *     quantum operations
 *     numerical methods
 *     future domains.
 *
 * The grammar imposes no semantic cardinality limits.
 *
 * Implementation-level parser resource limits, if any, are compiler/tooling
 * concerns and MUST NOT be described as language limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_SIMULATION_STEPS
 *     MAX_SIMULATION_EVENTS
 *     MAX_SCENARIOS
 *     MAX_SAMPLES
 *     MAX_CHECKPOINTS
 *     MAX_MODELS
 *     MAX_TARGETS
 *     MAX_RESOURCES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * It contains no:
 *
 *     physical device identifier
 *     physical qubit identifier
 *     vendor backend
 *     simulator vendor
 *     quantum gate inventory
 *     hardware topology
 *     routing decision
 *     scheduling decision
 *     calibration data
 *
 * Numeric values remain ordinary Zamani program expressions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. SECURITY AUDIT
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no runtime execution;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no hardware access;
 *     - performs no process spawning;
 *     - performs no native calls;
 *     - performs no foreign calls;
 *     - performs no random generation;
 *     - performs no environment inspection;
 *     - contains no embedded Rust;
 *     - contains no unsafe Rust.
 *
 * Any security-sensitive simulation operation is checked downstream through
 * effects, capabilities, policies and sandbox semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must be accepted by the parser:
 *
 *     simulate model;
 *
 *     simulate circuit;
 *
 *     simulate hardware_model;
 *
 *     simulate quantum_program;
 *
 *     simulate hybrid_program;
 *
 *     simulate pipeline(data);
 *
 *     simulate custom::model;
 *
 *     simulate model {}
 *
 *     simulate model {
 *         stimulus: input;
 *     }
 *
 *     simulate model {
 *         observe: output;
 *         expect: expected;
 *     }
 *
 *     simulate model {
 *         duration: duration_value;
 *         sampling: sampling_policy;
 *         checkpoint: checkpoint_value;
 *     }
 *
 *     simulate model {
 *         resource: required_resources;
 *         capability: required_capability;
 *     }
 *
 *     simulate model {
 *         simulate subsystem;
 *     }
 *
 *     simulate model {
 *         scenario {
 *             stimulus: input;
 *             observe: output;
 *         }
 *     }
 *
 *     simulate model {
 *         drive(signal, value);
 *         observe(signal);
 *         checkpoint(state);
 *     }
 *
 * The exact semantic validity of individual clauses is a downstream concern.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser must reject malformed forms such as:
 *
 *     simulate;
 *
 *     simulate {
 *     }
 *
 *     simulate model {
 *         : value;
 *     }
 *
 *     simulate model {
 *         name:
 *     }
 *
 *     simulate model {
 *         name =
 *     }
 *
 *     simulate model {
 *         name value;
 *     }
 *
 *     simulate model {
 *         {
 *         }
 *     }
 *
 * where the canonical expression grammar cannot form the missing constructs.
 *
 * Semantic-invalid examples such as:
 *
 *     simulate nonexistent_model;
 *
 * MUST normally be handled downstream rather than converted into syntax
 * errors by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     empty simulation body
 *     one clause
 *     many clauses
 *     nested simulation
 *     nested blocks
 *     deeply nested expressions
 *     qualified targets
 *     target calls
 *     complex target expressions
 *     expression statements
 *     mixed clauses and expressions
 *     mixed nested simulations and clauses
 *     large symbolic resource expressions
 *     quantum targets
 *     HDL targets
 *     hybrid targets
 *     distributed targets
 *     AI/model targets
 *
 * No finite semantic nesting/cardinality limit belongs here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests must verify that the grammar contains no source-language
 * limit on:
 *
 *     number of simulation items
 *     number of clauses
 *     number of nested simulations
 *     number of nested blocks
 *     number of target expressions
 *     expression complexity
 *     resource requirement magnitude
 *     simulation scenarios
 *     observations
 *     checkpoints
 *     samples
 *     model descriptions
 *
 * Large tests should be generated rather than manually enumerating a fixed
 * maximum.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given:
 *
 *     identical source
 *     identical lexer vocabulary
 *     identical grammar version
 *     identical parser configuration
 *
 * parsing must produce equivalent parse-tree structure.
 *
 * The grammar must not contain:
 *
 *     semantic predicates
 *     embedded actions
 *     runtime callbacks
 *     random decisions
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Required integration coverage:
 *
 *     simulation + classical
 *     simulation + quantum
 *     simulation + hybrid
 *     simulation + HDL
 *     simulation + hardware
 *     simulation + AI
 *     simulation + data
 *     simulation + distributed
 *     simulation + networking
 *     simulation + security
 *     simulation + resources
 *     simulation + capabilities
 *     simulation + effects
 *     simulation + contracts
 *     simulation + policies
 *     simulation + provenance
 *     simulation + adaptive execution
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. COMPATIBILITY TEST CONTRACT
 * ============================================================================
 *
 * Verify compatibility with:
 *
 *     existing simulate source forms
 *     canonical lexer token SIMULATE
 *     canonical expression grammar
 *     existing HDL simulation syntax
 *     execution-context syntax
 *     resource requirements
 *     capability requirements
 *     effect syntax
 *     contract syntax
 *     policy syntax
 *
 * Compatibility tests must distinguish:
 *
 *     parser compatibility
 *     semantic compatibility
 *     AST compatibility
 *     IR compatibility
 *     runtime compatibility
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] `StatementsSimulation` is the sole owner of the universal
 *         simulation statement syntax.
 *
 *     [ ] `simulationStatement` is the stable public entry rule.
 *
 *     [ ] The canonical lexer token `SIMULATE` is consumed.
 *
 *     [ ] No simulation-specific lexer token is introduced here.
 *
 *     [ ] The canonical expression grammar is reused.
 *
 *     [ ] No second expression grammar exists here.
 *
 *     [ ] Simulation targets are open-world expressions.
 *
 *     [ ] Simulation clause names remain extensible identifiers.
 *
 *     [ ] Simulation body cardinality is unrestricted by language constants.
 *
 *     [ ] Nested simulation is structurally supported.
 *
 *     [ ] Expression statements are supported.
 *
 *     [ ] Generic nested blocks are supported.
 *
 *     [ ] No simulator vendor is encoded.
 *
 *     [ ] No hardware vendor is encoded.
 *
 *     [ ] No quantum gate inventory is encoded.
 *
 *     [ ] No physical device is encoded.
 *
 *     [ ] No physical qubit is encoded.
 *
 *     [ ] No topology is encoded.
 *
 *     [ ] No routing is encoded.
 *
 *     [ ] No scheduling is encoded.
 *
 *     [ ] No QEC is encoded.
 *
 *     [ ] No ZQN is encoded.
 *
 *     [ ] No HAL is encoded.
 *
 *     [ ] No runtime execution is encoded.
 *
 *     [ ] AST ownership is downstream and documented.
 *
 *     [ ] Semantic ownership is downstream and documented.
 *
 *     [ ] IR ownership is downstream and documented.
 *
 *     [ ] Resource/capability ownership remains downstream.
 *
 *     [ ] Effects remain owned by the canonical effect subsystem.
 *
 *     [ ] Policies remain owned by the canonical policy subsystem.
 *
 *     [ ] Provenance remains owned by the canonical provenance subsystem.
 *
 *     [ ] Existing HDL simulation remains independently owned.
 *
 *     [ ] `statements.g4` can consume `simulationStatement` without this file
 *         needing modification.
 *
 *     [ ] `ZamaniParser.g4` receives it only through the canonical Statements
 *         composition hierarchy.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Rust 1.97 integration succeeds.
 *
 *     [ ] Rust 1.97.1 integration succeeds.
 *
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * The meaning of this grammar is:
 *
 *                         SIMULATE
 *                            |
 *                            v
 *                         TARGET
 *                            |
 *                            v
 *                  optional simulation intent
 *                            |
 *                            v
 *                    DOMAIN-NEUTRAL AST
 *                            |
 *                            v
 *                   SEMANTIC ANALYSIS
 *                            |
 *          +-----------------+------------------+
 *          |                 |                  |
 *          v                 v                  v
 *      classical         quantum::ir       HDL/hardware
 *          |                 |                  |
 *          +-----------------+------------------+
 *                            |
 *                            v
 *                   execution planning
 *                            |
 *                            v
 *                    optimization/lowering
 *                            |
 *                            v
 *                    target realization
 *
 * The statement describes simulation INTENT.
 *
 * It does not describe a simulator implementation.
 *
 * It does not describe physical hardware.
 *
 * It does not describe a fixed machine size.
 *
 * It does not impose a language ceiling.
 *
 * It therefore remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */