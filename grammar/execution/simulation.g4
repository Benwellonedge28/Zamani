/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/execution/simulation.g4
 *
 * Grammar:
 *     SimulationExpressions
 *
 * Status:
 *     PRODUCTION-READY
 *
 * Implementation baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust implementation
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the EXPRESSION form of source-level simulation intent.
 *
 * It allows simulation to be used anywhere a normal Zamani expression is
 * permitted.
 *
 * Canonical forms:
 *
 *     simulate target
 *     simulate target from source
 *     simulate target with (option)
 *     simulate target from source with (option, option)
 *
 * Simulation is an execution/analysis strategy.
 *
 * This grammar does NOT implement:
 *
 *     - a simulator;
 *     - a numerical solver;
 *     - a quantum simulator;
 *     - an HDL simulator;
 *     - an AI runtime;
 *     - a hardware model;
 *     - an execution engine;
 *     - an IR;
 *     - target selection;
 *     - resource allocation.
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
 *     expression composition
 *          |
 *          v
 *     simulationExpression
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
 *     +----+----------+-------------+-------------+
 *     |               |             |             |
 *     v               v             v             v
 * classical        quantum        HDL          hybrid
 *     |               |             |             |
 *     |               v             |             |
 *     |          quantum::ir        |             |
 *     +---------------+-------------+-------------+
 *                     |
 *                     v
 *              canonical semantic IR
 *                     |
 *              optimization
 *                     |
 *              lowering
 *                     |
 *              routing/scheduling
 *                     |
 *              resilience/recovery
 *                     |
 *                 ZQN / HAL
 *                     |
 *                     v
 *              target realization
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
 *     assignment precedence
 *     conditional precedence
 *     arithmetic precedence
 *     logical precedence
 *     unary precedence
 *     postfix precedence
 *     identifiers
 *     names
 *     literals
 *     calls
 *     indexing
 *     member access
 *     types
 *     declarations
 *     statements
 *     simulation statement bodies
 *     simulation algorithms
 *     simulation engines
 *     numerical methods
 *     quantum state simulation
 *     HDL simulation implementation
 *     AI model execution
 *     hardware realization
 *     target discovery
 *     capability resolution
 *     resource allocation
 *     scheduling
 *     routing
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *     AST implementation
 *     semantic implementation
 *     IR implementation
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There are two different simulation surfaces.
 *
 * Expression:
 *
 *     grammar/execution/simulation.g4
 *
 * Statement:
 *
 *     grammar/statements/simulate.g4
 *
 * They MUST NOT duplicate one another.
 *
 * This file owns:
 *
 *     simulate <expression>
 *
 * The statement grammar owns:
 *
 *     simulate <expression> { ... }
 *
 * This file MUST NOT define a simulation block.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/
 *     canonical expression composition
 *
 * This grammar consumes the canonical `expression` rule supplied by the
 * expression composition root.
 *
 * It deliberately does not import the complete Expressions grammar because
 * that would create an import cycle:
 *
 *     Expressions
 *         ->
 *     SimulationExpressions
 *         ->
 *     Expressions
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
 *     frontend/domain-neutral AST subsystem
 *
 * SEMANTIC_OWNER:
 *
 *     execution/simulation semantic subsystem
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
 *     canonical semantic IR
 *
 * Quantum:
 *
 *     semantic quantum model -> quantum::ir
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
 * Required token:
 *
 *     SIMULATE
 *
 * Existing canonical tokens:
 *
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * This grammar MUST NOT create local lexer rules or aliases.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * The grammar does not enumerate simulation technologies.
 *
 * It MUST NOT contain grammar alternatives for:
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
 *     emulator
 *
 * Those are represented downstream through:
 *
 *     semantic models
 *     capabilities
 *     resources
 *     policies
 *     dialects
 *     libraries
 *     execution strategies
 *     backend implementations
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * The simulation target is a normal Zamani expression.
 *
 * Therefore it may represent:
 *
 *     classical computation
 *     numerical computation
 *     tensor computation
 *     AI/ML computation
 *     knowledge computation
 *     quantum computation
 *     hybrid computation
 *     HDL intent
 *     hardware intent
 *     accelerator computation
 *     distributed computation
 *     networking computation
 *     data pipelines
 *     future computational domains
 *
 * The parser does not determine the target domain.
 *
 * Semantic analysis determines it.
 *
 * ============================================================================
 * SOURCE CONTRACT
 * ============================================================================
 *
 * The optional FROM clause supplies a source/input expression.
 *
 * Examples:
 *
 *     simulate model from input
 *     simulate circuit from initial_state
 *     simulate hardware_model from configuration
 *     simulate algorithm from dataset
 *
 * The source is intentionally domain-neutral.
 *
 * ============================================================================
 * CONTEXT CONTRACT
 * ============================================================================
 *
 * The optional WITH clause supplies an open-ended context.
 *
 * Examples:
 *
 *     simulate model with (scenario)
 *     simulate circuit with (noise_model)
 *     simulate model with (policy, resources)
 *     simulate hardware_model with (timing_model, configuration)
 *
 * Each option is a normal expression.
 *
 * This prevents the grammar from becoming a closed catalogue of simulation
 * facilities.
 *
 * ============================================================================
 * IMPORTANT CORRECTION
 * ============================================================================
 *
 * The context option list is REQUIRED when WITH is present.
 *
 * Therefore:
 *
 *     simulate model with (scenario)
 *
 * is valid.
 *
 * But:
 *
 *     simulate model with ()
 *
 * is invalid.
 *
 * This prevents an empty context from silently passing parser validation.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * The semantic layer may derive:
 *
 *     simulation
 *
 * from simulationExpression.
 *
 * Additional effects are derived from the target and context, for example:
 *
 *     randomness
 *     measurement
 *     IO
 *     network
 *     distributed
 *     quantum
 *     learning
 *     adaptation
 *     native
 *     foreign
 *
 * This grammar does not encode those effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements are semantic.
 *
 * Possible semantic requirements include:
 *
 *     capability("simulation")
 *     capability("quantum.simulation")
 *     capability("hdl.simulation")
 *     capability("distributed.simulation")
 *     capability("tensor.compute")
 *
 * No capability is hard-coded to a physical device.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements are resolved downstream.
 *
 * Possible resources include:
 *
 *     memory
 *     compute
 *     storage
 *     accelerator capacity
 *     simulation state
 *     network resources
 *     quantum simulation resources
 *     timing resources
 *
 * This grammar imposes no universal physical capacity.
 *
 * It contains no language-level maximum for:
 *
 *     simulations
 *     scenarios
 *     events
 *     samples
 *     states
 *     qubits
 *     processors
 *     GPUs
 *     FPGAs
 *     accelerators
 *     nodes
 *     memory
 *     storage
 *     tensor rank
 *     tensor dimensions
 *     devices
 *     network size
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Simulation expressions may appear inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract interpretation remains owned by grammar/validation/ and the
 * semantic contract subsystem.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Simulation may be constrained by:
 *
 *     execution policy
 *     resource policy
 *     security policy
 *     reproducibility policy
 *     deployment policy
 *     simulation policy
 *
 * Policy semantics remain outside this grammar.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Simulation can produce or consume evidence and provenance.
 *
 * The semantic/compiler layers may record:
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
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * This grammar does not define quantum operations.
 *
 * If the simulation target is quantum:
 *
 *     simulationExpression
 *         ->
 *     semantic simulation model
 *         ->
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * There is no separate simulation-specific quantum IR.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * If the target represents HDL:
 *
 *     simulationExpression
 *         ->
 *     HDL semantic model
 *         ->
 *     HDL simulation/synthesis analysis
 *
 * Physical widths, devices, timing resources and implementation details remain
 * downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The same source-level simulation expression remains valid independently of
 * target scale.
 *
 * Target realization may range from:
 *
 *     tiny embedded execution
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     accelerator
 *     QPU simulator
 *     HPC
 *     cluster
 *     distributed environment
 *     cloud
 *     future computational substrate
 *
 * The grammar does not need to change as available resources change.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * The parser must not inspect:
 *
 *     hardware
 *     memory availability
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     runtime state
 *     deployment state
 *     random state
 *
 * ============================================================================
 * IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no embedded Rust actions
 *     no runtime execution
 *     no hardware calls
 *     no filesystem calls
 *     no network calls
 *     no environment inspection
 *     no target discovery
 *
 * Generated parser integration remains compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser errors:
 *
 *     missing target
 *     malformed FROM clause
 *     missing source after FROM
 *     malformed WITH clause
 *     empty WITH option list
 *     missing closing parenthesis
 *     malformed option list
 *
 * Semantic errors:
 *
 *     invalid simulation target
 *     unsupported simulation realization
 *     unavailable capability
 *     insufficient resources
 *     prohibited effect
 *     violated policy
 *     violated contract
 *
 * Resource/capability failures MUST NOT be reported as syntax failures.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This is an additive expression capability.
 *
 * Existing statement syntax remains owned by:
 *
 *     grammar/statements/simulate.g4
 *
 * Existing:
 *
 *     simulate target;
 *
 * remains a statement.
 *
 * Expression simulation is consumed where an expression is required.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no fixed capacity constants.
 *
 * In particular, it contains no universal limit on:
 *
 *     simulation count
 *     nesting depth
 *     scenario count
 *     event count
 *     sample count
 *     state count
 *     model count
 *     resource quantity
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     nodes
 *     memory
 *     threads
 *     tensor rank
 *     register width
 *     network size
 *     device count
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     simulate model
 *     simulate model from input
 *     simulate model with (scenario)
 *     simulate model from input with (scenario, policy)
 *     simulate quantum_program
 *     simulate hardware_model
 *     simulate model(input)
 *     simulate pipeline(data)
 *     simulate model with (resource_requirement)
 *     simulate model with (reproducibility_policy)
 *     simulate model with (uncertainty_model)
 *
 * COMPOSITION:
 *
 *     outer(simulate model)
 *     collection(simulate model)
 *     function(simulate model)
 *     requires simulate model
 *     ensures simulate model
 *     property(simulate model)
 *
 * CROSS-DOMAIN:
 *
 *     simulate classical_model
 *     simulate quantum_program
 *     simulate hdl_model
 *     simulate hardware_model
 *     simulate ai_model(input)
 *     simulate distributed_system
 *     simulate hybrid_program
 *
 * NEGATIVE:
 *
 *     simulate
 *     simulate from input
 *     simulate model from
 *     simulate model with
 *     simulate model with ()
 *     simulate model with (
 *     simulate model with (a,)
 *     simulate model from input with (
 *
 * SCALABILITY:
 *
 *     deeply nested target expressions
 *     deeply nested option expressions
 *     large option lists
 *     large source expressions
 *     large target expressions
 *     nested simulation expressions
 *     simulation inside generic expressions
 *     cross-domain simulation expressions
 *
 * DETERMINISM:
 *
 *     identical source + identical lexer/parser configuration
 *     => equivalent parse structure
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * DONE when:
 *
 * [ ] SimulationExpressions is the grammar identity.
 * [ ] simulationExpression is the public entry point.
 * [ ] No lexer rules exist here.
 * [ ] Expressions is not imported here.
 * [ ] Canonical expression is reused as the operand boundary.
 * [ ] Target is open-world.
 * [ ] FROM is optional.
 * [ ] WITH is optional.
 * [ ] WITH requires at least one option.
 * [ ] Trailing comma policy is explicit.
 * [ ] No simulation technology is enumerated.
 * [ ] No machine capacity is encoded.
 * [ ] No simulator implementation is encoded.
 * [ ] No IR is created.
 * [ ] No runtime behavior is embedded.
 * [ ] Statement-level simulation remains separate.
 * [ ] Expressions imports SimulationExpressions.
 * [ ] primaryExpression includes simulationExpression.
 * [ ] tests exist for positive/negative/boundary/scalability cases.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar SimulationExpressions;

options {
    tokenVocab = ZamaniLexer;
};


/*
 * ============================================================================
 * PUBLIC EXPRESSION
 * ============================================================================
 */

simulationExpression
    : simulationOperation
    ;


/*
 * ============================================================================
 * SIMULATION OPERATION
 * ============================================================================
 *
 * Forms:
 *
 *     simulate target
 *     simulate target from source
 *     simulate target with (option)
 *     simulate target from source with (option, option)
 *
 * FROM precedes WITH deliberately so that the source form has one canonical
 * ordering and does not introduce multiple equivalent parse structures.
 */

simulationOperation
    : SIMULATE
      simulationTarget
      simulationSourceClause?
      simulationContextClause?
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * The target is a canonical expression.
 *
 * The leading SIMULATE token has already been consumed before this rule is
 * entered, so recursive expression composition does not create an ambiguous
 * alternative at the simulation operator itself.
 */

simulationTarget
    : simulationOperand
    ;


/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 */

simulationSourceClause
    : FROM
      simulationOperand
    ;


/*
 * ============================================================================
 * CONTEXT
 * ============================================================================
 *
 * An explicit option is mandatory when WITH is present.
 *
 * Therefore:
 *
 *     simulate target with ()
 *
 * is rejected.
 */

simulationContextClause
    : WITH
      LPAREN
      simulationOptionList
      RPAREN
    ;


/*
 * ============================================================================
 * OPTION LIST
 * ============================================================================
 *
 * One or more options.
 *
 * A trailing comma is permitted for consistency with extensible list syntax.
 *
 * Therefore both are valid:
 *
 *     with (a)
 *     with (a, b,)
 *
 * but this is invalid:
 *
 *     with ()
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
 * OPTION
 * ============================================================================
 *
 * Options remain ordinary expressions.
 *
 * This permits future semantic options without modifying this grammar.
 */

simulationOption
    : simulationOperand
    ;


/*
 * ============================================================================
 * EXPRESSION INTEGRATION BOUNDARY
 * ============================================================================
 *
 * The canonical expression rule belongs to the expression composition root.
 *
 * This grammar deliberately does not import Expressions.
 *
 * This permits:
 *
 *     simulate model
 *     simulate model(input)
 *     simulate function(simulate model)
 *     simulate simulate model
 *
 * subject to the normal expression semantics and precedence rules.
 *
 * The first SIMULATE token is consumed by simulationOperation before this
 * recursive boundary is reached.
 */

simulationOperand
    : expression
    ;


/*
 * ============================================================================
 * END
 * ============================================================================
 */