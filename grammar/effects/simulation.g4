/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/simulation.g4
 *
 * Grammar:
 *     SimulationEffects
 *
 * Status:
 *     CANONICAL / PRODUCTION EFFECT-LAYER SIMULATION BOUNDARY
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no runtime calls;
 *       - no filesystem access;
 *       - no network access;
 *       - no hardware discovery;
 *       - no resource allocation;
 *       - no randomness;
 *       - no unsafe code.
 *
 * Generated Zamani Rust code MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and the Zamani Rust implementation MUST NOT require unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the EFFECT-LAYER boundary for simulation-related
 * computation.
 *
 * It does NOT define the general source-level simulation language.
 *
 * Source-level simulation syntax is owned by the appropriate domain and
 * execution grammars, including:
 *
 *     grammar/statements/simulate.g4
 *     grammar/hdl/simulation.g4
 *     grammar/execution/
 *     grammar/quantum/
 *     grammar/hybrid/
 *
 * This file exists so that simulation can participate in Zamani's universal
 * effect system without creating a second simulation language.
 *
 * The central architectural distinction is:
 *
 *     simulation syntax
 *         !=
 *     simulation effect identity
 *         !=
 *     simulation semantics
 *         !=
 *     simulation implementation
 *
 * This grammar owns only the second boundary.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/spec/effects.md
 *          |
 *          v
 *     grammar/effects/effects.g4
 *          |
 *          +--> generic effect declarations
 *          +--> generic effect sets
 *          +--> generic effect operations
 *          +--> effect handling
 *          |
 *          v
 *     THIS FILE
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic effect analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future domain representation
 *
 * This file MUST remain above semantic realization.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     simulationEffect
 *     simulationEffectReference
 *     simulationEffectReferenceList
 *     simulationEffectGroup
 *     nonEmptySimulationEffectGroup
 *
 * These rules provide a stable effect-layer boundary for simulation identity.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the `simulate` statement;
 *     - simulation expressions;
 *     - simulation targets;
 *     - simulation bodies;
 *     - simulation scenarios;
 *     - simulation stimuli;
 *     - simulation observations;
 *     - simulation checkpoints;
 *     - simulation timing syntax;
 *     - waveform syntax;
 *     - event-kernel syntax;
 *     - solver syntax;
 *     - numerical integration syntax;
 *     - random-number generation;
 *     - deterministic execution syntax;
 *     - simulation configuration language;
 *     - simulation algorithms;
 *     - hardware models;
 *     - quantum gates;
 *     - physical qubits;
 *     - physical devices;
 *     - simulator vendors;
 *     - simulator implementations;
 *     - target selection;
 *     - resource allocation;
 *     - capability discovery;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - canonical IR.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Source-level simulation constructs remain owned by their respective
 * grammars.
 *
 *     grammar/statements/simulate.g4
 *         -> general `simulate` statement syntax
 *
 *     grammar/hdl/simulation.g4
 *         -> HDL simulation intent syntax
 *
 *     grammar/execution/
 *         -> execution/simulation semantics and composition
 *
 *     grammar/quantum/
 *         -> quantum simulation-related source semantics
 *
 *     grammar/hybrid/
 *         -> hybrid simulation composition
 *
 *     grammar/effects/effect-sets.g4
 *         -> generic effect references and effect sets
 *
 *     grammar/effects/effect-operations.g4
 *         -> generic effect operation invocation
 *
 *     grammar/effects/effect-declarations.g4
 *         -> effect declarations
 *
 *     THIS FILE
 *         -> simulation-effect classification boundary
 *
 * No competing simulation syntax authority may be introduced by this file.
 *
 * ============================================================================
 * EFFECT / SIMULATION DISTINCTION
 * ============================================================================
 *
 * A source construct such as:
 *
 *     simulate model;
 *
 * is source-level execution intent.
 *
 * A semantic effect such as:
 *
 *     simulation::run
 *
 * describes observable computational behavior associated with simulation.
 *
 * This file provides the structural boundary through which such effect
 * identities can participate in the generic effect system.
 *
 * The grammar does NOT decide whether a qualified name actually denotes a
 * simulation effect.
 *
 * Semantic registration and effect analysis make that determination.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Simulation effects are intentionally open-world.
 *
 * Examples of valid semantic identities include:
 *
 *     simulation::run
 *     simulation::event
 *     simulation::model
 *     simulation::observe
 *     simulation::sample
 *     simulation::checkpoint
 *     simulation::co_simulation
 *     simulation::distributed
 *     simulation::accelerated
 *     simulation::quantum
 *     simulation::hybrid
 *     simulation::hardware
 *     simulation::formal
 *     simulation::custom::future_effect
 *
 * These names are examples of semantic identities, not a closed grammar
 * catalogue.
 *
 * This file MUST NOT enumerate every possible simulation effect.
 *
 * Future simulation technologies must be representable without changing this
 * grammar merely because a new effect identity is introduced.
 *
 * ============================================================================
 * NO APPLICATION-SPECIFIC KEYWORD EXPLOSION
 * ============================================================================
 *
 * Simulation concepts such as:
 *
 *     model
 *     solver
 *     waveform
 *     event
 *     scenario
 *     stimulus
 *     observation
 *     checkpoint
 *     sampling
 *     tolerance
 *     seed
 *     duration
 *     timeout
 *     trace
 *
 * remain semantic data or ordinary identifiers unless another authoritative
 * grammar establishes a genuine language-wide lexical requirement.
 *
 * This file therefore introduces NO simulation-specific lexer tokens.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexer authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical token vocabulary:
 *
 *     ZamaniLexer
 *
 * This grammar contains no lexer rules.
 *
 * It MUST NOT define:
 *
 *     SIMULATION
 *     SIMULATE
 *     K_SIMULATION
 *     K_SIMULATE
 *
 * or any replacement token.
 *
 * If the canonical lexer exposes `SIMULATE`, that token remains owned by the
 * canonical lexical layer and is consumed by the source-level simulation
 * grammar, not by this effect boundary.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * This file depends only on the canonical naming grammar and lexer vocabulary.
 *
 * The preferred dependency is:
 *
 *     Names
 *
 * because the effect identity is represented as a canonical qualified name.
 *
 * This avoids importing the full expression or statement grammar and prevents
 * unnecessary grammar dependency cycles.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar.
 *
 * It MUST NOT become an ANTLR lexer grammar.
 *
 * It MUST NOT define:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     argumentList
 *     statement
 *     block
 *     effectOperation
 *     effectSet
 *
 * Those constructs have existing owners.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `simulationEffect` is the stable public entry point.
 *
 * It intentionally delegates to the canonical qualified-name grammar.
 *
 * Example:
 *
 *     simulation::run
 *
 * Example:
 *
 *     simulation::quantum
 *
 * Example:
 *
 *     simulation::distributed::event
 *
 * Example:
 *
 *     future::simulation::new_model
 *
 * Whether a name belongs to the simulation effect family is a semantic
 * question, not a parsing question.
 *
 * ============================================================================
 * EFFECT REFERENCE
 * ============================================================================
 *
 * A reference identifies a simulation-related effect without invoking it.
 *
 * The reference is represented by a canonical qualified name.
 *
 * No finite list of simulation effects is encoded here.
 *
 * ============================================================================
 * EFFECT REFERENCE LIST
 * ============================================================================
 *
 * The list permits arbitrary source-level cardinality through repetition.
 *
 * No universal maximum is imposed.
 *
 * A trailing comma is accepted consistently with the effect-set boundary.
 *
 * ============================================================================
 * EFFECT GROUP
 * ============================================================================
 *
 * The group is a structural adapter for consumers that need an explicit
 * simulation-effect collection.
 *
 * It does NOT redefine generic effect-set semantics.
 *
 * For example:
 *
 *     {
 *         simulation::run,
 *         simulation::observe
 *     }
 *
 * remains a collection of effect identities.
 *
 * Canonical semantic normalization determines set semantics.
 *
 * ============================================================================
 * EMPTY GROUP
 * ============================================================================
 *
 * `simulationEffectGroup` permits an empty group.
 *
 * `nonEmptySimulationEffectGroup` requires at least one effect reference.
 *
 * This distinction is useful for consumers that need to distinguish:
 *
 *     no explicit simulation effects
 *
 * from:
 *
 *     explicitly provided simulation-effect collection.
 *
 * The semantic layer determines whether that distinction has behavioral
 * significance in a particular context.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Simulation effect syntax expresses semantic intent.
 *
 * It MUST NOT encode a particular realization.
 *
 * For example:
 *
 *     simulation::run
 *
 * MUST NOT imply:
 *
 *     a particular simulator;
 *     a particular CPU;
 *     a particular GPU;
 *     a particular FPGA;
 *     a particular ASIC;
 *     a particular QPU;
 *     a particular accelerator;
 *     a particular cluster;
 *     a particular node;
 *     a particular memory device;
 *     a particular event kernel;
 *     a particular numerical solver;
 *     a particular vendor.
 *
 * The same semantic effect may be realized by:
 *
 *     - a tiny embedded implementation;
 *     - a CPU implementation;
 *     - a multicore implementation;
 *     - a GPU implementation;
 *     - an FPGA implementation;
 *     - an accelerator;
 *     - a distributed system;
 *     - an HPC system;
 *     - a quantum simulator;
 *     - a hybrid simulator;
 *     - a hardware/software co-simulation environment;
 *     - a future computational substrate.
 *
 * Source portability does not guarantee physical feasibility.
 *
 * If a realization cannot satisfy the semantic requirements, the compiler or
 * runtime must report that condition rather than silently changing program
 * meaning.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no language-level finite limit on:
 *
 *     - simulation-effect references;
 *     - effect groups;
 *     - qualified-name depth;
 *     - namespaces;
 *     - simulation domains;
 *     - semantic effect identities;
 *     - program size;
 *     - simulation model size;
 *     - simulation state size;
 *     - simulation duration;
 *     - simulation sample count;
 *     - simulation event count;
 *     - machine size;
 *     - CPU count;
 *     * GPU count;
 *     * FPGA count;
 *     * accelerator count;
 *     * QPU count;
 *     * qubit count;
 *     * node count;
 *     * memory capacity;
 *     * network size.
 *
 * This grammar MUST contain no language-level constants representing such
 * limits.
 *
 * In particular, it MUST NOT introduce any maximum-count mechanism for
 * simulation or hardware.
 *
 * Practical parser, compiler, runtime, operating-system, and hardware limits
 * are implementation/resource constraints and are outside this grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     - the token stream;
 *     - the active grammar;
 *     - the language version;
 *     - the imported grammar versions.
 *
 * Parsing MUST NOT depend on:
 *
 *     - simulator discovery;
 *     - hardware discovery;
 *     - resource availability;
 *     - network state;
 *     - runtime state;
 *     - wall-clock time;
 *     - random state;
 *     - environment state.
 *
 * Deterministic parsing is therefore independent of execution realization.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing proves only structural validity.
 *
 * Semantic analysis determines:
 *
 *     - whether the referenced effect exists;
 *     - whether it is registered as a simulation effect;
 *     - whether the effect is accessible;
 *     - whether its semantic version is compatible;
 *     - what effect parameters mean;
 *     - what capabilities are required;
 *     - what resources are required;
 *     - which policies apply;
 *     - which contracts apply;
 *     - whether simulation is permitted;
 *     - whether simulation is deterministic;
 *     - whether simulation is reproducible;
 *     - whether simulation crosses a domain boundary;
 *     - whether simulation involves classical computation;
 *     - whether simulation involves quantum computation;
 *     - whether simulation involves HDL/hardware;
 *     - whether simulation involves distributed execution;
 *     - whether the requested realization is feasible.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A simulation-related operation may itself produce other effects.
 *
 * For example, semantic analysis may determine that a simulation operation
 * additionally involves:
 *
 *     IO
 *     randomness
 *     network
 *     distributed
 *     measurement
 *     quantum
 *     foreign
 *     native
 *     mutation
 *     resource acquisition
 *
 * Such relationships are semantic effect relationships.
 *
 * This grammar MUST NOT hard-code them.
 *
 * Effect inference, effect propagation, effect inclusion, effect handling,
 * and effect discharge belong to semantic analysis.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements remain outside this grammar.
 *
 * Semantic analysis may determine that an effect requires capabilities such as:
 *
 *     simulation.execution
 *     simulation.event
 *     simulation.acceleration
 *     simulation.distributed
 *     simulation.quantum
 *     simulation.hardware
 *     simulation.deterministic
 *
 * These are semantic capability identities, not parser-level hardware
 * selections.
 *
 * The grammar must not encode a particular device or implementation.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Simulation may require arbitrary resources.
 *
 * Examples of semantic requirements include:
 *
 *     memory;
 *     compute;
 *     storage;
 *     accelerator capacity;
 *     communication;
 *     execution time;
 *     simulation state;
 *     result storage.
 *
 * The resource subsystem determines feasibility.
 *
 * This file MUST NOT define:
 *
 *     memory sizes;
 *     processor counts;
 *     device counts;
 *     fixed simulation-state sizes;
 *     fixed event counts;
 *     fixed sample counts.
 *
 * Resource quantities belong to the resource model and may be symbolic or
 * derived from program data.
 *
 * ============================================================================
 * REQUIREMENT CONTRACT
 * ============================================================================
 *
 * A simulation effect may be subject to semantic requirements such as:
 *
 *     requires capability("simulation.execution");
 *     requires capability("simulation.quantum");
 *     requires memory >= required_memory;
 *     requires topology(required_topology);
 *
 * These forms are owned by the appropriate requirement/resource grammar.
 *
 * This file supplies no duplicate requirement syntax.
 *
 * ============================================================================
 * CONSTRAINT CONTRACT
 * ============================================================================
 *
 * Simulation realization may be constrained by:
 *
 *     numerical conditions;
 *     timing requirements;
 *     reproducibility requirements;
 *     security constraints;
 *     resource constraints;
 *     topology constraints;
 *     domain compatibility;
 *     accuracy requirements;
 *     correctness conditions.
 *
 * Constraint syntax remains owned by the universal validation/resource/policy
 * subsystem.
 *
 * This file does not implement constraint evaluation.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Simulation effects may be governed by:
 *
 *     execution policies;
 *     security policies;
 *     resource policies;
 *     reproducibility policies;
 *     sandbox policies;
 *     deployment policies;
 *     adaptation policies.
 *
 * Policy ownership remains outside this file.
 *
 * Semantic policy analysis may reject an otherwise syntactically valid
 * simulation effect.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Simulation-related behavior may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * These constructs belong to the validation/contract subsystem.
 *
 * This file does not duplicate their syntax.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Simulation effects must remain traceable through the normal provenance
 * system.
 *
 * Semantic provenance may preserve:
 *
 *     source span;
 *     effect identity;
 *     declaration origin;
 *     transformation history;
 *     model origin;
 *     evidence;
 *     policy decisions;
 *     capability decisions;
 *     resource decisions;
 *     compiler transformations;
 *     execution realization;
 *     verification results.
 *
 * This file does not define a second provenance system.
 *
 * ============================================================================
 * EXPLANATION / DECISION CONTRACT
 * ============================================================================
 *
 * Simulation can participate in the universal explanation and decision model.
 *
 * Examples include:
 *
 *     why a simulation strategy was selected;
 *     why a capability was required;
 *     why a resource realization was rejected;
 *     why a simulation was moved to another execution substrate;
 *     why a deterministic mode was selected;
 *     why a fallback realization was chosen.
 *
 * Such explanations are semantic/tooling data.
 *
 * They do not belong in the parser grammar.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a simulation effect MUST NOT execute a simulation.
 *
 * It MUST NOT:
 *
 *     - load a model;
 *     - open a file;
 *     - connect to a network;
 *     - access hardware;
 *     - allocate simulation resources;
 *     - execute native code;
 *     - execute foreign code;
 *     - access secrets;
 *     - mutate external state.
 *
 * Security, sandboxing, authorization, and capability enforcement happen
 * downstream.
 *
 * ============================================================================
 * SIMULATION / EXECUTION BOUNDARY
 * ============================================================================
 *
 * Simulation is an execution strategy or analysis mode.
 *
 * It is not a second programming language.
 *
 * The semantic pipeline may therefore be:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic simulation intent
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical realization
 *       +--> quantum realization
 *       +--> HDL/hardware realization
 *       +--> distributed realization
 *       +--> accelerator realization
 *       +--> future realization
 *
 * The simulation effect itself does not select the realization.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum simulation remains a domain semantic concern.
 *
 * This file does NOT define:
 *
 *     qubits;
 *     gates;
 *     quantum states;
 *     coupling maps;
 *     physical qubits;
 *     QEC;
 *     routing;
 *     scheduling;
 *     calibration;
 *     QPU selection.
 *
 * A simulation-related quantum computation may eventually lower through:
 *
 *     semantic model
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     QEC / resilience
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL / target realization
 *
 * This file remains above that boundary.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical simulation may lower through the canonical semantic model into
 * the appropriate classical representation.
 *
 * This file does not define:
 *
 *     instructions;
 *     registers;
 *     processor models;
 *     vector widths;
 *     cache sizes;
 *     core counts.
 *
 * Those belong downstream.
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL simulation syntax remains owned by:
 *
 *     grammar/hdl/simulation.g4
 *
 * The effect layer may classify semantic simulation behavior associated with
 * HDL execution.
 *
 * This file does not redefine:
 *
 *     signals;
 *     ports;
 *     clocks;
 *     timing;
 *     event controls;
 *     waveforms;
 *     synthesis;
 *     verification.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * Hybrid computation may combine:
 *
 *     classical simulation;
 *     quantum simulation;
 *     hardware simulation;
 *     AI/model simulation;
 *     distributed simulation.
 *
 * The hybrid semantic layer determines how these domains compose.
 *
 * This file contributes only the simulation-effect boundary.
 *
 * ============================================================================
 * AI / REASONING INTEGRATION
 * ============================================================================
 *
 * Simulation results may feed:
 *
 *     inference;
 *     reasoning;
 *     learning;
 *     adaptation;
 *     uncertainty;
 *     evidence;
 *     explanation;
 *     decision records.
 *
 * AI grammar remains responsible for AI-specific source constructs.
 *
 * This file does not define AI syntax.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Simulation may be distributed across arbitrary execution resources.
 *
 * The semantic model may express:
 *
 *     distributed simulation;
 *     synchronization;
 *     partitioning;
 *     communication;
 *     fault handling;
 *     reproducibility.
 *
 * This file does not encode:
 *
 *     node identifiers;
 *     fixed node counts;
 *     cluster sizes;
 *     topology;
 *     machine placement.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * A simulation may participate in adaptive execution.
 *
 * Semantic execution may select among valid realizations based on:
 *
 *     capabilities;
 *     resources;
 *     policy;
 *     resilience;
 *     reproducibility;
 *     performance;
 *     correctness.
 *
 * Such selection is performed after parsing.
 *
 * The grammar does not encode a particular fallback strategy.
 *
 * ============================================================================
 * REPRODUCIBILITY
 * ============================================================================
 *
 * Simulation may be required to produce reproducible results.
 *
 * Reproducibility semantics may involve:
 *
 *     deterministic execution;
 *     explicit randomness;
 *     controlled seeds;
 *     model version;
 *     input provenance;
 *     environment description;
 *     compiler version;
 *     dialect version;
 *     execution policy.
 *
 * This grammar does not define random generators or seed types.
 *
 * Reproducibility is a semantic/compiler/runtime contract.
 *
 * ============================================================================
 * REFLECTION / METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Simulation may be inspected, generated, transformed, or configured through
 * the metaprogramming subsystem.
 *
 * Reflection remains owned by:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * Code generation remains owned by:
 *
 *     grammar/metaprogramming/generation.g4
 *
 * This file MUST NOT import or redefine those syntaxes.
 *
 * If reflection or code generation is used with simulation, semantic effect
 * analysis may associate the corresponding effects.
 *
 * ============================================================================
 * FOREIGN / NATIVE INTEGRATION
 * ============================================================================
 *
 * A simulation implementation may eventually use native or foreign
 * functionality.
 *
 * Such use is represented through the existing:
 *
 *     grammar/effects/native.g4
 *     grammar/effects/foreign.g4
 *     interoperability/
 *
 * This file does not create an alternate FFI boundary.
 *
 * Semantic analysis must preserve the additional effects, capabilities,
 * policies, and provenance introduced by such crossings.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST lower through the existing domain-neutral frontend AST.
 *
 * Conceptual mapping:
 *
 *     simulationEffect
 *         ->
 *     simulation-effect semantic reference
 *
 *     simulationEffectReference
 *         ->
 *     canonical qualified effect identity
 *
 *     simulationEffectReferenceList
 *         ->
 *     ordered source collection of effect identities
 *
 *     simulationEffectGroup
 *         ->
 *     grouped semantic effect references
 *
 * No simulator-specific AST hierarchy is introduced here.
 *
 * This file MUST NOT create AST types such as:
 *
 *     SimulationEngine
 *     SimulationBackend
 *     SimulatorDevice
 *     QubitSimulator
 *     HardwareSimulator
 *     CpuSimulator
 *     GpuSimulator
 *     PhysicalSimulationTarget
 *
 * Such implementation concepts belong downstream.
 *
 * ============================================================================
 * AST SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The normal ANTLR token/context mechanism must preserve source locations.
 *
 * The frontend must be able to associate:
 *
 *     simulationEffect
 *     simulationEffectReference
 *     simulationEffectReferenceList
 *     simulationEffectGroup
 *
 * with their source spans.
 *
 * No second source-location system may be introduced.
 *
 * ============================================================================
 * SEMANTIC NORMALIZATION
 * ============================================================================
 *
 * Semantic analysis should normalize effect identities according to the
 * canonical qualified-name and effect identity model.
 *
 * For collections:
 *
 *     simulation::run
 *     simulation::run
 *
 * may normalize to one semantic identity when ordinary effect-set semantics
 * apply.
 *
 * Source ordering may still be retained for diagnostics and formatting.
 *
 * ============================================================================
 * NAME RESOLUTION
 * ============================================================================
 *
 * Name resolution determines:
 *
 *     - namespace validity;
 *     - imports;
 *     - aliases;
 *     - visibility;
 *     - declarations;
 *     - version compatibility;
 *     - semantic effect classification.
 *
 * This grammar deliberately does not resolve names.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define an IR.
 *
 * The canonical lowering direction is:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic effect model
 *       |
 *       v
 *     canonical semantic representation / ZUIR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> accelerator representation
 *       +--> future domain representation
 *
 * Simulation effect syntax must never directly construct quantum::ir,
 * hardware IR, or a simulator-specific IR.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * If semantic analysis determines that a simulation represents quantum
 * computation, the resulting quantum computation may lower to:
 *
 *     quantum::ir
 *
 * This grammar does not define that IR and does not bypass semantic analysis.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * If semantic analysis determines that a simulation concerns HDL or hardware,
 * lowering proceeds through the corresponding semantic/domain representation.
 *
 * This grammar does not define:
 *
 *     physical devices;
 *     pin assignments;
 *     clock frequencies;
 *     transistor counts;
 *     fabrication technology;
 *     vendor implementation.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages consuming this grammar must preserve:
 *
 *     effect identity
 *     source provenance
 *     semantic relationships
 *     requirements
 *     capabilities
 *     resources
 *     constraints
 *     policies
 *     contracts
 *
 * Optimization may change implementation strategy only when observable
 * semantic behavior remains valid.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime execution is outside the parser.
 *
 * A runtime may:
 *
 *     resolve a simulation implementation;
 *     acquire permitted resources;
 *     execute a simulator;
 *     execute a distributed simulation;
 *     execute an accelerated simulation;
 *     recover from failures;
 *     record provenance;
 *     produce observations.
 *
 * None of these actions may occur while parsing.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level malformed syntax includes constructs such as:
 *
 *     simulation::
 *     ::simulation
 *     simulation:::
 *     simulation::
 *     {
 *     { simulation::run, }
 *
 * depending on the canonical qualified-name grammar.
 *
 * Semantic diagnostics include:
 *
 *     unknown simulation effect;
 *     unresolved simulation effect;
 *     inaccessible simulation effect;
 *     incompatible effect version;
 *     invalid effect context;
 *     unavailable capability;
 *     unavailable resource;
 *     policy violation;
 *     contract violation;
 *     unsupported realization;
 *     invalid domain composition;
 *     prohibited native/foreign crossing;
 *     non-reproducible realization where reproducibility is required.
 *
 * Semantic errors MUST NOT be implemented as parser predicates.
 *
 * ============================================================================
 * SECURITY DIAGNOSTICS
 * ============================================================================
 *
 * The parser MUST accept structurally valid effect identities even when a
 * particular environment later refuses them.
 *
 * For example:
 *
 *     simulation::custom::operation
 *
 * may parse successfully while semantic analysis reports:
 *
 *     unknown effect
 *
 * or:
 *
 *     capability unavailable
 *
 * or:
 *
 *     policy prohibits execution
 *
 * This separation is required for extensibility and portability.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing generic effect references remain valid.
 *
 * Existing source-level simulation syntax remains governed by its existing
 * owners.
 *
 * This file does not replace:
 *
 *     grammar/statements/simulate.g4
 *     grammar/hdl/simulation.g4
 *
 * It also does not change the meaning of existing generic effect operations.
 *
 * A future simulation effect identity can be introduced through semantic
 * registration without changing this grammar.
 *
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * Domain-specific simulation dialects may extend simulation semantics through
 * the dialect system.
 *
 * Examples include:
 *
 *     quantum simulation;
 *     HDL simulation;
 *     distributed simulation;
 *     accelerator simulation;
 *     scientific simulation;
 *     hybrid simulation;
 *     future simulation technology.
 *
 * Dialects MUST NOT require modifications to this generic effect grammar
 * merely to add new effect identities.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     simulation::run
 *     simulation::observe
 *     simulation::event
 *     simulation::quantum
 *     simulation::hybrid
 *     simulation::hardware
 *     simulation::distributed
 *     simulation::custom::operation
 *     future::simulation::operation
 *
 * Lists:
 *
 *     simulation::run,
 *     simulation::observe
 *
 * Groups:
 *
 *     {
 *         simulation::run,
 *         simulation::observe
 *     }
 *
 * Empty group:
 *
 *     {
 *     }
 *
 * Non-empty group:
 *
 *     {
 *         simulation::run
 *     }
 *
 * Qualified names:
 *
 *     simulation::domain::subdomain::operation
 *
 * Cross-domain identities:
 *
 *     quantum::simulation
 *     hdl::simulation
 *     distributed::simulation
 *     accelerator::simulation
 *
 * must remain structurally representable.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Tests must reject malformed qualified-name structure, including:
 *
 *     simulation::
 *     ::simulation
 *     simulation:::
 *
 * where rejected by the canonical Names grammar.
 *
 * Invalid collection structure must also be tested according to the canonical
 * comma/list rules.
 *
 * Semantic negative tests must separately verify:
 *
 *     unknown effect;
 *     unavailable capability;
 *     unavailable resource;
 *     prohibited policy;
 *     incompatible effect;
 *
 * These must be tested downstream rather than by modifying this grammar.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests must include:
 *
 *     - empty effect group;
 *     - one effect;
 *     - multiple effects;
 *     - repeated effects;
 *     - trailing comma;
 *     - deeply qualified names;
 *     - large effect collections;
 *     - cross-domain effect identities;
 *     - future namespace identities;
 *     - simulation associated with quantum semantics;
 *     - simulation associated with HDL semantics;
 *     - simulation associated with distributed semantics;
 *     - simulation associated with AI/model semantics.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Generated tests must verify that no grammar-level bound exists on:
 *
 *     - number of effect references;
 *     - number of effect groups;
 *     - qualified-name depth;
 *     - number of namespaces;
 *     - program size.
 *
 * Tests should generate increasingly large valid inputs until external
 * implementation/resource limits are reached.
 *
 * Such implementation limits MUST NOT be encoded as grammar constants.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical token streams parsed with the same grammar/version must produce
 * equivalent parse structures.
 *
 * Parsing must not query:
 *
 *     hardware;
 *     filesystem;
 *     network;
 *     simulator registry;
 *     runtime;
 *     clock;
 *     randomness source.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The effect boundary must be tested with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     distributed
 *     networking
 *     data
 *     accelerator
 *     future dialect
 *
 * No domain may require this grammar to enumerate its complete simulation
 * vocabulary.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/effects.g4
 * ============================================================================
 *
 * `effects.g4` remains the generic effect composition root.
 *
 * This file MUST NOT become a replacement for that composition root.
 *
 * The generic effect grammar already supports open-world effect identities and
 * generic effect operations.
 *
 * Therefore no change to the generic effect model is required merely to add
 * this boundary.
 *
 * Semantic tooling may associate:
 *
 *     simulation::* 
 *
 * effect identities with the simulation effect family.
 *
 * That classification belongs to the semantic registry.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/effect-operations.g4
 * ============================================================================
 *
 * Generic operation invocation remains owned by:
 *
 *     grammar/effects/effect-operations.g4
 *
 * For example:
 *
 *     perform simulation::run(model)
 *
 * is structurally parsed through the generic effect-operation path.
 *
 * This file MUST NOT redefine:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *     performEffectOperation
 *
 * This prevents duplicate invocation syntax.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/effect-sets.g4
 * ============================================================================
 *
 * Generic effect sets remain owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * This file does not redefine generic effect-set semantics.
 *
 * `simulationEffectGroup` exists only as a specialized structural adapter for
 * consumers that explicitly need simulation-effect grouping.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/statements/simulate.g4
 * ============================================================================
 *
 * The statement grammar remains the owner of:
 *
 *     simulate <target>
 *
 * and related statement syntax.
 *
 * It may semantically associate such a construct with one or more simulation
 * effects.
 *
 * This file must not be imported merely to parse a `simulate` statement.
 *
 * This separation prevents two competing simulation grammars.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/hdl/simulation.g4
 * ============================================================================
 *
 * HDL simulation intent remains owned by:
 *
 *     grammar/hdl/simulation.g4
 *
 * That grammar already describes simulation targets, bodies, named clauses,
 * bindings, stimuli, observations, timing intent, checkpoints, and related
 * HDL simulation structure.
 *
 * This file supplies only the effect-layer boundary.
 *
 * The HDL simulation grammar must not duplicate simulation-effect identity
 * syntax.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/execution/
 * ============================================================================
 *
 * Execution semantics may consume the semantic simulation-effect model to
 * determine:
 *
 *     execution mode;
 *     realization strategy;
 *     fallback;
 *     retry;
 *     recovery;
 *     reproducibility;
 *     observability.
 *
 * Execution grammars remain responsible for source-level execution constructs.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/quantum/
 * ============================================================================
 *
 * Quantum simulation remains a quantum semantic concern.
 *
 * This file provides no quantum operation grammar.
 *
 * If a simulation contains quantum computation, semantic lowering may target:
 *
 *     quantum::ir
 *
 * after semantic analysis.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/hybrid/
 * ============================================================================
 *
 * Hybrid semantics may compose:
 *
 *     classical simulation
 *     quantum simulation
 *     HDL simulation
 *     AI/model simulation
 *     distributed simulation
 *
 * This file remains independent of the hybrid implementation.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/resources/
 * ============================================================================
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * Simulation feasibility is checked after parsing.
 *
 * No machine-size requirement is encoded here.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/security/
 * ============================================================================
 *
 * Security may restrict simulation through:
 *
 *     capabilities;
 *     authorization;
 *     sandbox;
 *     policies;
 *     audit;
 *     provenance.
 *
 * This grammar does not bypass or duplicate those controls.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/validation/
 * ============================================================================
 *
 * Validation may establish:
 *
 *     preconditions;
 *     postconditions;
 *     invariants;
 *     properties;
 *     guarantees;
 *     evidence requirements.
 *
 * This grammar only provides the effect identity boundary.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/metaprogramming/
 * ============================================================================
 *
 * Reflection, compile-time evaluation, quotation, syntax-tree processing,
 * and generation remain owned by the metaprogramming subsystem.
 *
 * Simulation metadata may be inspected or generated there, but this file
 * does not create a metaprogramming dependency.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/interoperability/
 * ============================================================================
 *
 * Foreign simulation implementations may cross the FFI/ABI boundary.
 *
 * Such crossings must remain visible to semantic effect analysis and security
 * policy.
 *
 * This grammar does not create an alternate FFI syntax.
 *
 * ============================================================================
 * INTEGRATION WITH ROOT GRAMMAR
 * ============================================================================
 *
 * `grammar/Zamani.g4` MUST remain the universal composition root.
 *
 * This file SHOULD NOT be imported directly into the root merely because it
 * exists.
 *
 * The generic effect composition path remains the normal root integration.
 *
 * This specialized file is available to effect-aware semantic tooling,
 * specialized effect composition, conformance tests, and future modular
 * composition where its public rules are explicitly required.
 *
 * This avoids unnecessary root-level grammar coupling.
 *
 * ============================================================================
 * FILE-LEVEL DEPENDENCY METADATA
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/effects/effect-sets.g4 [semantic integration]
 *     grammar/effects/effect-operations.g4 [semantic integration]
 *
 * EXPORTS:
 *
 *     simulationEffect
 *     simulationEffectReference
 *     simulationEffectReferenceList
 *     simulationEffectGroup
 *     nonEmptySimulationEffectGroup
 *
 * CONSUMED_BY:
 *
 *     effect-aware semantic tooling
 *     simulation effect conformance tests
 *     specialized effect composition
 *     future execution/effect adapters
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     effect semantic analysis
 *     execution semantic analysis
 *     simulation semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic representation / ZUIR
 *     domain IR selected after semantic analysis
 *     quantum::ir when quantum semantics require it
 *
 * TEST_OWNER:
 *
 *     grammar/tests/effects/simulation/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/effects.md
 *     grammar/specification/effects.md where that specification layer exists
 *
 * SOURCE_SYNTAX_OWNERS:
 *
 *     grammar/statements/simulate.g4
 *     grammar/hdl/simulation.g4
 *     grammar/execution/
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/
 *     grammar/security/
 *
 * PROVENANCE_OWNER:
 *
 *     grammar/spec/provenance.md
 *     semantic provenance implementation
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file deliberately contains no:
 *
 *     machine capacity;
 *     simulator capacity;
 *     hardware capacity;
 *     fixed event count;
 *     fixed model size;
 *     fixed sample count;
 *     fixed state count;
 *     fixed processor count;
 *     fixed accelerator count;
 *     fixed device count;
 *     fixed qubit count;
 *     fixed node count;
 *     fixed memory size.
 *
 * It contains no language-level constants corresponding to implementation
 * capacity.
 *
 * Numeric values, when appearing in user programs, remain program data and
 * are not language limits.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It compiles as an ANTLR4 parser grammar.
 *
 *     [ ] It uses the canonical Zamani lexer vocabulary.
 *
 *     [ ] It introduces no lexer rules.
 *
 *     [ ] It imports only the canonical naming foundation required by its
 *         rules.
 *
 *     [ ] It does not define `simulate` statement syntax.
 *
 *     [ ] It does not duplicate HDL simulation syntax.
 *
 *     [ ] It does not define simulator algorithms.
 *
 *     [ ] It does not define numerical solvers.
 *
 *     [ ] It does not define event kernels.
 *
 *     [ ] It does not define waveform formats.
 *
 *     [ ] It does not define hardware models.
 *
 *     [ ] It does not define quantum gates.
 *
 *     [ ] It does not define physical qubits.
 *
 *     [ ] It does not define QEC.
 *
 *     [ ] It does not define ZQN.
 *
 *     [ ] It does not define routing.
 *
 *     [ ] It does not define scheduling.
 *
 *     [ ] It does not define resources.
 *
 *     [ ] It does not define capabilities.
 *
 *     [ ] It does not define policies.
 *
 *     [ ] It does not define contracts.
 *
 *     [ ] It does not define provenance.
 *
 *     [ ] It does not define runtime execution.
 *
 *     [ ] It does not define target selection.
 *
 *     [ ] It remains open-world.
 *
 *     [ ] It permits arbitrary qualified effect identities.
 *
 *     [ ] It permits arbitrary effect-list cardinality through grammar
 *         repetition.
 *
 *     [ ] It permits arbitrary qualified-name depth subject to the canonical
 *         name grammar.
 *
 *     [ ] It preserves source structure for semantic analysis.
 *
 *     [ ] It preserves the domain-neutral AST boundary.
 *
 *     [ ] It preserves the canonical semantic IR boundary.
 *
 *     [ ] It preserves quantum::ir as the quantum IR boundary.
 *
 *     [ ] It remains independent from simulator implementation.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Compatibility tests pass.
 *
 *     [ ] Rust-generated frontend integration succeeds on Rust 1.97.
 *
 *     [ ] Rust-generated frontend integration succeeds on Rust 1.97.1.
 *
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar SimulationEffects;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 1. SIMULATION EFFECT
 * ============================================================================
 *
 * Stable public entry point.
 *
 * The effect identity itself is deliberately open-world.
 */

simulationEffect
    : simulationEffectReference
    ;


/*
 * ============================================================================
 * 2. SIMULATION EFFECT REFERENCE
 * ============================================================================
 *
 * A simulation effect is represented by the canonical qualified-name model.
 *
 * Examples:
 *
 *     simulation::run
 *     simulation::observe
 *     simulation::event
 *     simulation::quantum
 *     simulation::distributed
 *     future::simulation::operation
 *
 * Semantic analysis determines whether the resolved identity actually denotes
 * a simulation effect.
 */

simulationEffectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. SIMULATION EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Zero or more references are represented by repetition at the consumer
 * boundary.
 *
 * The list itself is non-empty.
 *
 * A trailing comma is accepted consistently with the specialized effect
 * collection conventions.
 */

simulationEffectReferenceList
    : simulationEffectReference
      (
          COMMA
          simulationEffectReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. SIMULATION EFFECT GROUP
 * ============================================================================
 *
 * Structural grouping only.
 *
 * This does not redefine generic effect-set semantics.
 *
 * It is useful for specialized consumers that need an explicit simulation
 * effect collection.
 */

simulationEffectGroup
    : LBRACE
      simulationEffectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 5. NON-EMPTY SIMULATION EFFECT GROUP
 * ============================================================================
 *
 * Structural variant requiring at least one simulation-effect reference.
 */

nonEmptySimulationEffectGroup
    : LBRACE
      simulationEffectReferenceList
      RBRACE
    ;