/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/simulation.g4
 *
 * Status:
 *     CANONICAL PRODUCTION HDL SIMULATION-INTENT PARSER DELEGATE
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions or semantic predicates.
 *     Zamani-owned Rust implementation MUST use safe Rust only.
 *     No unsafe Rust is permitted.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative authority:
 *
 *     grammar/DESIGN.md
 *           |
 *           v
 *     grammar/spec/hdl.md
 *           |
 *           v
 *     grammar/hdl/simulation.g4
 *           |
 *           v
 *     grammar/hdl/hdl.g4
 *           |
 *           v
 *     grammar/Zamani.g4
 *
 * This file owns SOURCE-LEVEL HDL SIMULATION INTENT.
 *
 * It does NOT own simulation algorithms, numerical solvers, waveform engines,
 * event kernels, random-number generation, hardware models, target discovery,
 * physical devices, synthesis, verification algorithms, or runtime execution.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar provides a portable syntax boundary for describing:
 *
 *     - simulation intent;
 *     - simulation targets;
 *     - simulation scenarios;
 *     - simulation configuration;
 *     - stimuli;
 *     - observations;
 *     - measurements;
 *     - expectations;
 *     - simulation timing;
 *     - checkpoints;
 *     - sampling;
 *     - initial conditions;
 *     - termination conditions;
 *     - simulation policies;
 *     - simulation metadata;
 *     - simulation-specific resource/capability intent.
 *
 * Simulation is treated as an ANALYSIS / EXECUTION INTENT.
 *
 * The grammar does not execute anything.
 *
 * ============================================================================
 * CORE DESIGN PRINCIPLE
 * ============================================================================
 *
 * The language describes:
 *
 *     WHAT should be simulated
 *     WHAT observations matter
 *     WHAT constraints apply
 *     WHAT resources/capabilities are required
 *
 * It does NOT prescribe:
 *
 *     WHICH simulator
 *     WHICH event kernel
 *     WHICH numerical solver
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH FPGA
 *     WHICH QPU
 *     WHICH simulator vendor
 *     WHICH waveform format
 *     WHICH random-number implementation
 *     WHICH physical machine
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A simulation description MUST remain portable.
 *
 * The same source may be interpreted as:
 *
 *     - cycle-accurate simulation;
 *     - event-driven simulation;
 *     - transaction-level simulation;
 *     - behavioral simulation;
 *     - hardware/software co-simulation;
 *     - distributed simulation;
 *     - accelerated simulation;
 *     - FPGA-assisted simulation;
 *     - GPU-accelerated simulation;
 *     - quantum/hybrid simulation;
 *     - formal/simulation-assisted analysis;
 *     - future simulation technology.
 *
 * The grammar MUST NOT require source changes merely because the simulation
 * backend changes.
 *
 * ============================================================================
 * OPEN-WORLD MODEL
 * ============================================================================
 *
 * This file deliberately does NOT enumerate:
 *
 *     - simulator vendors;
 *     - numerical solvers;
 *     - waveform formats;
 *     - random generators;
 *     - hardware models;
 *     - CPU models;
 *     - GPU models;
 *     - FPGA families;
 *     - ASIC technologies;
 *     - QPU families;
 *     - verification engines.
 *
 * These are semantic data, library operations, dialects, capabilities,
 * compiler intrinsics, or downstream implementation choices.
 *
 * ============================================================================
 * NO NEW KEYWORDS
 * ============================================================================
 *
 * The canonical lexer already provides:
 *
 *     SIMULATE
 *
 * Therefore this grammar MUST NOT introduce:
 *
 *     K_SIMULATION
 *     K_SIMULATE
 *     SIMULATION
 *
 * or another HDL-specific lexer.
 *
 * Simulation sub-concepts such as:
 *
 *     stimulus
 *     observation
 *     waveform
 *     scenario
 *     checkpoint
 *     sample
 *     seed
 *     duration
 *     timeout
 *     tolerance
 *     model
 *
 * remain ordinary identifiers unless the language specification later
 * establishes a genuine lexical requirement.
 *
 * This keeps the simulation grammar extensible.
 *
 * ============================================================================
 * LEXER
 * ============================================================================
 */

parser grammar HdlSimulation;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * General expressions are owned by the canonical expression grammar.
 *
 * The simulation grammar therefore reuses `expression` instead of inventing
 * a simulation-specific expression language.
 *
 * When composed into the canonical HDL grammar, the HDL composition root MUST
 * make the canonical expression rule available exactly once.
 *
 * ============================================================================
 */

import Expressions;

/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable HDL simulation-intent boundary.
 *
 * hdl.g4 MUST expose this rule through its HDL member composition.
 *
 * ============================================================================
 */

hdlSimulationConstruct
    : hdlSimulationDeclaration
    ;

/*
 * ============================================================================
 * SIMULATION DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     simulate circuit;
 *
 *     simulate circuit {
 *         ...
 *     }
 *
 * The target is an ordinary Zamani expression.
 *
 * Therefore all of the following can remain open-world:
 *
 *     simulate design;
 *     simulate top;
 *     simulate system(model);
 *     simulate accelerator;
 *     simulate quantum_control;
 *     simulate custom::model;
 *
 * Semantic analysis determines whether the target is actually simulatable.
 *
 * ============================================================================
 */

hdlSimulationDeclaration
    : SIMULATE
      hdlSimulationTarget
      hdlSimulationBody?
      SEMICOLON?
    ;

/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * Simulation target identity is semantic.
 *
 * It MUST NOT encode a physical device.
 *
 * ============================================================================
 */

hdlSimulationTarget
    : expression
    ;

/*
 * ============================================================================
 * SIMULATION BODY
 * ============================================================================
 *
 * A simulation body contains zero or more simulation-intent items.
 *
 * There is no fixed number of:
 *
 *     stimuli
 *     observations
 *     expectations
 *     checkpoints
 *     parameters
 *     constraints
 *     scenarios
 *     samples
 *     policies
 *
 * ============================================================================
 */

hdlSimulationBody
    : LBRACE
      hdlSimulationItem*
      RBRACE
    ;

/*
 * ============================================================================
 * SIMULATION ITEM
 * ============================================================================
 *
 * The grammar deliberately uses structural forms instead of reserving a large
 * vocabulary of simulation concepts.
 *
 * This permits future simulation concepts without grammar changes.
 * ============================================================================
 */

hdlSimulationItem
    : hdlSimulationNamedClause
    | hdlSimulationBinding
    | hdlSimulationNestedConstruct
    | hdlSimulationExpressionStatement
    ;

/*
 * ============================================================================
 * NAMED CLAUSE
 * ============================================================================
 *
 * Generic semantic form:
 *
 *     stimulus: expression;
 *     observe: expression;
 *     expect: expression;
 *     duration: expression;
 *     sampling: expression;
 *     checkpoint: expression;
 *     waveform: expression;
 *     tolerance: expression;
 *     seed: expression;
 *     initial: expression;
 *     termination: expression;
 *     timeout: expression;
 *
 * The names are identifiers, not keywords.
 *
 * This is intentional.
 *
 * ============================================================================
 */

hdlSimulationNamedClause
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * BINDINGS
 * ============================================================================
 *
 * Generic compile-time / configuration / scenario binding:
 *
 *     duration = value;
 *     steps = value;
 *     seed = value;
 *     tolerance = value;
 *     sample_period = value;
 *
 * Semantic analysis determines the meaning of the binding.
 *
 * ============================================================================
 */

hdlSimulationBinding
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;

/*
 * ============================================================================
 * NESTED SIMULATION
 * ============================================================================
 *
 * Nested simulation intent is permitted.
 *
 * This supports hierarchical simulation and co-simulation descriptions
 * without requiring a fixed hierarchy depth.
 *
 * ============================================================================
 */

hdlSimulationNestedConstruct
    : hdlSimulationDeclaration
    ;

/*
 * ============================================================================
 * EXPRESSION STATEMENTS
 * ============================================================================
 *
 * Simulation operations remain ordinary Zamani expressions.
 *
 * Examples:
 *
 *     drive(signal, value);
 *     observe(signal);
 *     sample(bus);
 *     checkpoint(state);
 *     compare(actual, expected);
 *     record(trace);
 *     reset(model);
 *
 * None of these operation names are hard-coded.
 *
 * ============================================================================
 */

hdlSimulationExpressionStatement
    : expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * SCENARIO BOUNDARY
 * ============================================================================
 *
 * A scenario is represented as a named simulation target/body rather than a
 * new reserved keyword.
 *
 * This keeps scenario vocabulary open-ended.
 *
 * ============================================================================
 */

hdlSimulationScenario
    : identifier
      hdlSimulationBody
    ;

/*
 * ============================================================================
 * STIMULUS
 * ============================================================================
 *
 * Stimulus is represented by the generic named-clause structure.
 *
 * Canonical semantic interpretation:
 *
 *     stimulus: expression;
 *
 * The expression may represent:
 *
 *     - a signal value;
 *     - a transaction;
 *     - a function call;
 *     - a stream;
 *     - a generated sequence;
 *     - a distributed event;
 *     - a hardware interaction intent.
 *
 * ============================================================================
 */

hdlSimulationStimulus
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * OBSERVATION
 * ============================================================================
 *
 * Observation remains semantic rather than simulator-specific.
 *
 * ============================================================================
 */

hdlSimulationObservation
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * EXPECTATION
 * ============================================================================
 *
 * Expected behavior is represented as an expression.
 *
 * Verification semantics remain owned by the verification subsystem.
 *
 * This rule does NOT replace grammar/hdl/assertions.g4.
 *
 * ============================================================================
 */

hdlSimulationExpectation
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * INITIALIZATION
 * ============================================================================
 *
 * Initial conditions are expressed as ordinary simulation clauses.
 *
 * Example:
 *
 *     initial: state == reset_state;
 *
 * No fixed initialization model is imposed.
 *
 * ============================================================================
 */

hdlSimulationInitialization
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * TIME / SCHEDULING INTENT
 * ============================================================================
 *
 * Simulation may require semantic temporal information:
 *
 *     duration
 *     start
 *     end
 *     step
 *     period
 *     sampling
 *     timeout
 *
 * These remain expressions.
 *
 * This grammar does not implement a clock or timing engine.
 *
 * Clock semantics remain owned by:
 *
 *     grammar/hdl/clocks.g4
 *     grammar/hdl/clocking.g4
 *     grammar/hdl/timing.g4
 *
 * ============================================================================
 */

hdlSimulationTemporalClause
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * SAMPLING
 * ============================================================================
 *
 * Sampling configuration is represented semantically.
 *
 * There is no universal maximum sampling frequency or sample count.
 *
 * ============================================================================
 */

hdlSimulationSamplingClause
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * CHECKPOINT
 * ============================================================================
 *
 * Checkpoint intent may be represented as:
 *
 *     checkpoint: expression;
 *
 * or:
 *
 *     checkpoint(state);
 *
 * The expression grammar handles the operation form.
 *
 * ============================================================================
 */

hdlSimulationCheckpointClause
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * WAVEFORM / TRACE INTENT
 * ============================================================================
 *
 * Waveform and trace formats are NOT hard-coded.
 *
 * Examples:
 *
 *     waveform: trace;
 *     waveform: "format";
 *     trace: signals;
 *
 * Actual serialization is downstream.
 *
 * ============================================================================
 */

hdlSimulationTraceClause
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * RANDOMNESS / REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility metadata may be expressed semantically:
 *
 *     seed: seed_value;
 *     deterministic: true;
 *     reproducibility: policy;
 *
 * This grammar does not implement RNG behavior.
 *
 * The runtime/simulation subsystem owns random-state semantics.
 *
 * ============================================================================
 */

hdlSimulationReproducibilityClause
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Simulation MAY require capabilities or resources.
 *
 * Examples:
 *
 *     requires capability("simulation.event");
 *     requires capability("simulation.acceleration");
 *     requires memory >= required_memory;
 *
 * The actual resource/capability grammar remains owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *
 * This file MUST NOT duplicate those grammars.
 *
 * ============================================================================
 */

hdlSimulationRequirement
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * PREFERENCE / HINT INTEGRATION
 * ============================================================================
 *
 * Simulation may express non-binding implementation guidance through ordinary
 * expressions or the repository's resource/hardware intent syntax.
 *
 * Examples:
 *
 *     prefer: accelerator("simulation");
 *     hint: parallel;
 *
 * These remain semantic data.
 *
 * ============================================================================
 */

hdlSimulationPreference
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * ERROR / TERMINATION CONDITIONS
 * ============================================================================
 *
 * Simulation termination may be expressed through generic clauses:
 *
 *     until: condition;
 *     timeout: duration;
 *     stop: condition;
 *
 * The parser does not evaluate these expressions.
 *
 * ============================================================================
 */

hdlSimulationTermination
    : identifier
      COLON
      expression
      SEMICOLON?
    ;

/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The grammar intentionally provides structure only.
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the target exists;
 *     - whether the target is simulatable;
 *     - whether all referenced signals exist;
 *     - whether stimulus types are compatible;
 *     - whether observations are valid;
 *     - whether expectations are well-typed;
 *     - whether timing constraints are meaningful;
 *     - whether sampling is valid;
 *     - whether checkpoint state is representable;
 *     - whether resource requirements are satisfiable;
 *     - whether required capabilities exist;
 *     - whether the selected simulation strategy is semantically valid;
 *     - whether deterministic execution is requested;
 *     - whether nondeterminism is explicitly permitted.
 *
 * None of these checks belong in this parser grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every simulation construct MUST lower through the existing domain-neutral
 * frontend AST.
 *
 * Conceptual mapping:
 *
 *     hdlSimulationDeclaration
 *         -> generic domain/simulation declaration
 *
 *     hdlSimulationTarget
 *         -> target expression
 *
 *     hdlSimulationBody
 *         -> ordered child construct collection
 *
 *     hdlSimulationNamedClause
 *         -> named semantic clause
 *
 *     hdlSimulationBinding
 *         -> generic binding
 *
 *     hdlSimulationExpressionStatement
 *         -> expression statement
 *
 * No simulator-specific AST hierarchy should be introduced merely because
 * this grammar exists.
 *
 * ============================================================================
 * SOURCE PROVENANCE
 * ============================================================================
 *
 * The AST MUST preserve source spans for:
 *
 *     simulation declaration
 *     simulation target
 *     simulation body
 *     named clause
 *     binding
 *     expression statement
 *
 * This enables diagnostics to identify the exact simulation intent that
 * caused a semantic or resource failure.
 *
 * ============================================================================
 * SEMANTIC MODEL
 * ============================================================================
 *
 * The semantic pipeline is:
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
 *     simulation semantic analysis
 *       |
 *       +--> type analysis
 *       +--> timing analysis
 *       +--> resource analysis
 *       +--> capability analysis
 *       +--> determinism analysis
 *       +--> verification integration
 *       |
 *       v
 *     canonical semantic model / IR
 *       |
 *       +--> simulator
 *       +--> hardware co-simulation
 *       +--> distributed simulation
 *       +--> accelerated simulation
 *       +--> formal/simulation integration
 *       +--> target realization
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Simulation syntax MUST NOT create a quantum simulation IR.
 *
 * If the simulation target contains quantum computation:
 *
 *     simulation source
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     existing quantum::ir
 *          |
 *          v
 *     simulation / optimization / QEC / ZQN / HAL as appropriate
 *
 * The quantum frontend and quantum::ir remain authoritative for quantum
 * semantics.
 *
 * This grammar only expresses the simulation intent.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical models and expressions remain ordinary Zamani constructs.
 *
 * The simulation grammar does not reproduce:
 *
 *     arithmetic
 *     matrices
 *     tensors
 *     numerical algorithms
 *     signal processing
 *
 * Those remain owned by the corresponding classical/expression grammars.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL simulation may target:
 *
 *     modules
 *     interfaces
 *     signals
 *     nets
 *     registers
 *     memories
 *     processes
 *     pipelines
 *     state machines
 *     generated structures
 *
 * Semantic validation determines whether the target is a valid hardware model.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Simulation may cross:
 *
 *     classical
 *         ->
 *     HDL
 *         ->
 *     quantum
 *         ->
 *     classical
 *
 * without creating separate languages or IRs.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A simulation target may represent a distributed system.
 *
 * This grammar imposes no fixed:
 *
 *     node count
 *     process count
 *     channel count
 *     partition count
 *     replication count
 *
 * Distributed semantics remain owned by the distributed subsystem.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Simulation may operate over:
 *
 *     tensors
 *     datasets
 *     models
 *     agents
 *     streams
 *     learned systems
 *
 * AI/data semantics remain owned by their domains.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Networked hardware simulation may model:
 *
 *     endpoints
 *     packets
 *     streams
 *     protocols
 *     latency
 *     bandwidth
 *     failures
 *
 * Networking semantics remain owned by networking/.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing this grammar MUST NOT:
 *
 *     - execute a simulator;
 *     - execute hardware;
 *     - access devices;
 *     - open network connections;
 *     - read files;
 *     - allocate target resources;
 *     - invoke external commands;
 *     - generate code;
 *     - access secrets.
 *
 * `simulate` is source syntax only.
 *
 * Execution is downstream and capability-controlled.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no semantic maximum on:
 *
 *     simulations
 *     nested simulations
 *     simulation items
 *     stimuli
 *     observations
 *     expectations
 *     checkpoints
 *     bindings
 *     scenarios
 *     expressions
 *     model dimensions
 *     signal counts
 *     hardware size
 *     distributed node counts
 *     quantum resource counts
 *
 * ANTLR repetition operators represent arbitrary finite source structures.
 *
 * "Infinity" means no artificial language-level finite ceiling.
 *
 * Physical execution remains bounded by available resources.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_SIMULATIONS
 *     MAX_STIMULI
 *     MAX_OBSERVATIONS
 *     MAX_SAMPLES
 *     MAX_STEPS
 *     MAX_SIGNALS
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * A program MAY contain explicit values such as:
 *
 *     steps = 1000;
 *     samples = 100000;
 *     duration = 1s;
 *
 * Those are program semantics, not language ceilings.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no runtime calls;
 *     no hardware queries;
 *     no randomness.
 *
 * Therefore parsing is deterministic for a fixed:
 *
 *     source
 *     canonical token vocabulary
 *     grammar version
 *
 * Simulation nondeterminism, if explicitly requested by the source semantics,
 * is a downstream runtime concern.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Semantic diagnostics should be able to identify:
 *
 *     - missing simulation target;
 *     - invalid target;
 *     - unknown referenced model;
 *     - invalid stimulus;
 *     - invalid observation;
 *     - invalid expectation;
 *     - incompatible timing;
 *     - invalid sampling;
 *     - unavailable capability;
 *     - insufficient resources;
 *     - unsupported simulation semantics;
 *     - unsupported target realization.
 *
 * Resource failure MUST remain distinguishable from syntax failure.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar uses simple repetition and delegation.
 *
 * It does not perform:
 *
 *     numerical evaluation;
 *     symbolic solving;
 *     simulation;
 *     hardware discovery;
 *     resource discovery.
 *
 * These expensive operations remain downstream.
 *
 * Implementations MAY impose operational parser/elaboration safeguards for
 * denial-of-service protection or resource exhaustion.
 *
 * Such safeguards MUST NOT become language-level semantic limits.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not rename:
 *
 *     grammar/hdl/hdl.g4
 *     grammar/hdl/clocks.g4
 *     grammar/hdl/clocking.g4
 *     grammar/hdl/timing.g4
 *     grammar/hdl/assertions.g4
 *     grammar/hdl/processes.g4
 *     grammar/hdl/sequential.g4
 *     grammar/hdl/combinational.g4
 *     grammar/hdl/memories.g4
 *     grammar/hdl/registers.g4
 *
 * It also does not create a second simulation grammar elsewhere.
 *
 * If an older simulation syntax exists in another domain grammar, that syntax
 * must be classified as:
 *
 *     compatible;
 *     migrated;
 *     deprecated;
 *     or historical
 *
 * rather than silently becoming another authority.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Required HDL composition:
 *
 *     grammar/hdl/hdl.g4
 *          |
 *          +--> import HdlSimulation
 *          |
 *          +--> expose hdlSimulationConstruct
 *
 * The HDL member dispatcher should include:
 *
 *     | hdlSimulationConstruct
 *
 * where simulation is permitted as an HDL member.
 *
 * The universal Zamani root MUST reach HDL through the existing HDL
 * composition path and MUST NOT import HdlSimulation directly.
 *
 * ============================================================================
 * VERIFICATION INTEGRATION
 * ============================================================================
 *
 * Simulation expectations MUST integrate with:
 *
 *     grammar/hdl/assertions.g4
 *
 * where an actual assertion/property construct is required.
 *
 * This file MUST NOT duplicate:
 *
 *     hdlAssertion
 *     hdlAssumption
 *     hdlCoverage
 *     hdlPropertyDeclaration
 *
 * Verification engines remain downstream.
 *
 * ============================================================================
 * CLOCK / TIMING INTEGRATION
 * ============================================================================
 *
 * This file consumes simulation timing intent but does not redefine:
 *
 *     clock declarations;
 *     clock domains;
 *     clock relationships;
 *     timing constraints.
 *
 * Those remain owned by:
 *
 *     grammar/hdl/clocks.g4
 *     grammar/hdl/clocking.g4
 *     grammar/hdl/timing.g4
 *
 * ============================================================================
 * HARDWARE / RESOURCE INTEGRATION
 * ============================================================================
 *
 * Simulation resource requirements belong to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *
 * Examples:
 *
 *     requires capability("simulation.accelerated");
 *     requires memory >= required_memory;
 *
 * No target-specific capacity is encoded here.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * The parser/frontend produces intent only.
 *
 * Downstream runtime/simulation components determine:
 *
 *     simulator;
 *     execution strategy;
 *     numerical engine;
 *     parallelism;
 *     distributed execution;
 *     checkpoint storage;
 *     trace storage;
 *     random source;
 *     hardware acceleration;
 *     result materialization.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive examples:
 *
 *     simulate top;
 *
 *     simulate top {
 *         stimulus: reset;
 *         observe: output;
 *     }
 *
 *     simulate design {
 *         duration: 1000;
 *         sampling: period;
 *         initial: state == reset_state;
 *         termination: done;
 *     }
 *
 *     simulate system {
 *         seed = seed_value;
 *         tolerance = tolerance_value;
 *         waveform: trace;
 *         checkpoint: state;
 *     }
 *
 *     simulate hybrid_model {
 *         stimulus: input_stream;
 *         observe: quantum_result;
 *         expectation: result == expected;
 *     }
 *
 * Negative examples:
 *
 *     simulate;
 *
 *     simulate {
 *     }
 *
 *     simulate top {
 *         stimulus:
 *     }
 *
 *     simulate top {
 *         duration =
 *     }
 *
 *     simulate top {
 *         checkpoint
 *     }
 *
 * Boundary examples:
 *
 *     simulate top {}
 *
 *     simulate x;
 *
 *     simulate a { a: b; }
 *
 *     deeply nested simulation constructs;
 *
 *     arbitrarily many named clauses;
 *
 *     arbitrarily large symbolic expressions;
 *
 *     symbolic timing values;
 *
 *     symbolic resource requirements.
 *
 * Scalability:
 *
 *     no fixed number of simulation items;
 *     no fixed number of stimuli;
 *     no fixed number of observations;
 *     no fixed number of checkpoints;
 *     no fixed number of samples;
 *     no fixed number of simulation targets;
 *     no fixed simulation depth.
 *
 * Determinism:
 *
 *     identical source + identical lexer + identical grammar
 *         =>
 *     identical parse structure.
 *
 * ============================================================================
 * DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] simulation intent has one canonical grammar owner;
 *     [x] simulation execution is downstream;
 *     [x] general expressions are reused;
 *     [x] no simulation-specific lexer exists;
 *     [x] no simulator vendor is hard-coded;
 *     [x] no solver is hard-coded;
 *     [x] no hardware target is hard-coded;
 *     [x] no resource maximum is hard-coded;
 *     [x] simulation configuration is extensible;
 *     [x] stimuli are representable;
 *     [x] observations are representable;
 *     [x] expectations are representable;
 *     [x] timing intent is representable;
 *     [x] checkpoints are representable;
 *     [x] reproducibility metadata is representable;
 *     [x] resource/capability intent integrates downstream;
 *     [x] verification remains separately owned;
 *     [x] clock/timing remain separately owned;
 *     [x] AST integration is defined;
 *     [x] semantic integration is defined;
 *     [x] IR integration is defined;
 *     [x] runtime integration is defined;
 *     [x] quantum integration is defined;
 *     [x] classical integration is defined;
 *     [x] hybrid integration is defined;
 *     [x] distributed integration is defined;
 *     [x] security boundary is defined;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] boundary tests are defined;
 *     [x] scalability tests are defined;
 *     [x] determinism requirements are defined;
 *     [x] safe-Rust requirements are defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 *     SIMULATION SYNTAX
 *          !=
 *     SIMULATION ENGINE
 *          !=
 *     NUMERICAL SOLVER
 *          !=
 *     HARDWARE MODEL
 *          !=
 *     VERIFICATION ENGINE
 *          !=
 *     RESOURCE MANAGER
 *          !=
 *     RUNTIME
 *
 * The grammar describes portable simulation intent.
 *
 * ============================================================================
 */