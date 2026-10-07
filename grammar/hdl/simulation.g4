/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hdl/simulation.g4
 *
 * GRAMMAR
 * -------
 * HdlSimulation
 *
 * STATUS
 * ------
 * CANONICAL HDL SIMULATION-INTENT PARSER DELEGATE
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021 edition
 * Safe Rust only
 *
 * No unsafe Rust is required or permitted by the implementation contract.
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
 *     HDL composition
 *          |
 *          v
 *     hdlSimulationConstruct
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     semantic simulation model       HDL semantics
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                 execution planning
 *                          |
 *              +-----------+-----------+
 *              |                       |
 *              v                       v
 *          simulation              verification
 *              |                       |
 *              +-----------+-----------+
 *                          |
 *                          v
 *                    synthesis / co-design
 *                          |
 *                          v
 *                   hardware realization
 *
 * This file owns SOURCE-LEVEL HDL SIMULATION INTENT ONLY.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative architectural authority:
 *
 *     grammar/DESIGN.md
 *
 * HDL specification:
 *
 *     grammar/spec/hdl.md
 *
 * HDL composition:
 *
 *     grammar/hdl/hdl.g4
 *
 * Universal simulation execution semantics:
 *
 *     grammar/execution/simulation.g4
 *
 * Universal simulation statement syntax:
 *
 *     grammar/statements/simulate.g4
 *
 * Universal simulation expression syntax:
 *
 *     grammar/expressions/simulation.g4
 *
 * Simulation policy composition:
 *
 *     grammar/policies/simulation.g4
 *
 * Simulation effect composition:
 *
 *     grammar/effects/simulation.g4
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     hdlSimulationConstruct
 *     hdlSimulationDeclaration
 *     hdlSimulationTarget
 *     hdlSimulationBody
 *     hdlSimulationItem
 *     hdlSimulationNamedClause
 *     hdlSimulationResourceClause
 *     hdlSimulationPolicyClause
 *     hdlSimulationNestedConstruct
 *     hdlSimulationExpressionStatement
 *
 * It owns only the HDL-specific structural boundary required to express
 * simulation intent in an HDL context.
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 * This file does NOT own:
 *
 *     - lexer rules;
 *     - token spelling;
 *     - identifiers;
 *     - names;
 *     - general expressions;
 *     - general statements;
 *     - generic simulation expressions;
 *     - generic simulation statements;
 *     - simulation algorithms;
 *     - event kernels;
 *     - numerical solvers;
 *     - waveform engines;
 *     - random-number generators;
 *     - hardware models;
 *     - target discovery;
 *     - target selection;
 *     - simulator selection;
 *     - vendor selection;
 *     - CPU selection;
 *     - GPU selection;
 *     - FPGA selection;
 *     - ASIC selection;
 *     - accelerator selection;
 *     - QPU selection;
 *     - physical device selection;
 *     - physical placement;
 *     - routing;
 *     - scheduling;
 *     - resource allocation;
 *     - verification algorithms;
 *     - formal verification;
 *     - synthesis;
 *     - timing closure;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There are several simulation-related grammar files in Zamani, but they have
 * deliberately different ownership:
 *
 *     grammar/expressions/simulation.g4
 *         universal simulation expression syntax
 *
 *     grammar/statements/simulate.g4
 *         universal simulation statement syntax
 *
 *     grammar/execution/simulation.g4
 *         execution-level simulation expression semantics
 *
 *     grammar/hdl/simulation.g4
 *         HDL-specific simulation-intent composition
 *
 *     grammar/policies/simulation.g4
 *         simulation-policy composition
 *
 *     grammar/effects/simulation.g4
 *         simulation-effect composition
 *
 * This file MUST NOT become another universal simulation language.
 *
 * ============================================================================
 * OPEN-WORLD / POCO-REAF CONTRACT
 * ============================================================================
 *
 * Simulation targets are expressions.
 *
 * Therefore this grammar does NOT enumerate:
 *
 *     simulators
 *     engines
 *     solvers
 *     waveform formats
 *     hardware models
 *     CPU models
 *     GPU models
 *     FPGA families
 *     ASIC families
 *     accelerator families
 *     QPU families
 *     vendors
 *     device types
 *     node counts
 *     memory sizes
 *     qubit counts
 *     thread counts
 *     tensor ranks
 *     network sizes
 *
 * No universal physical capacity is encoded here.
 *
 * The same source-level HDL simulation intent can therefore be considered
 * for implementations ranging from a very small environment to the largest
 * implementation supported by the available resources and toolchain.
 *
 * Source portability does not imply physical feasibility.
 *
 * If a target cannot satisfy the semantic requirements, that is a semantic,
 * capability, resource, policy, or target-feasibility diagnostic — NOT a
 * grammar failure.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical vocabulary:
 *
 *     ZamaniLexer
 *
 * This file introduces NO lexer rules and NO token aliases.
 *
 * Canonical tokens consumed here include:
 *
 *     SIMULATE
 *     FROM
 *     WITH
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     POLICY
 *
 * These tokens are owned by the canonical lexical layer.
 *
 * Simulation concepts such as:
 *
 *     stimulus
 *     observation
 *     expectation
 *     initialization
 *     checkpoint
 *     sampling
 *     waveform
 *     trace
 *     timeout
 *     tolerance
 *     seed
 *     duration
 *     scenario
 *     model
 *
 * are intentionally NOT reserved here.
 *
 * They remain identifiers or ordinary expressions unless another authoritative
 * language layer gives them a distinct lexical meaning.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT IMPORT
 * -------------
 *
 *     Expressions
 *
 * This provides the canonical:
 *
 *     expression
 *
 * rule.
 *
 * No simulation-specific expression grammar is recreated here.
 *
 * No import of the HDL composition root is permitted because that would create
 * a composition cycle:
 *
 *     HDL -> HdlSimulation -> HDL
 *
 * Instead, HDL composition imports this grammar.
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * The single stable public entry point is:
 *
 *     hdlSimulationConstruct
 *
 * HDL composition MUST consume that entry point.
 *
 * Consumers MUST NOT reproduce the rules in this file.
 *
 * ============================================================================
 */

parser grammar HdlSimulation;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Example:
 *
 *     simulate design;
 *
 *     simulate design {
 *         stimulus: input;
 *         observe: output;
 *         expect: output == expected;
 *     }
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
 * Canonical HDL simulation forms:
 *
 *     simulate target;
 *
 *     simulate target {
 *         ...
 *     }
 *
 * The terminating semicolon is optional when a braced body is present.
 *
 * A semicolon remains accepted for consistency with source-level statement
 * syntax:
 *
 *     simulate target {};
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
 * The target is an ordinary Zamani expression.
 *
 * This deliberately permits open-world semantic targets such as:
 *
 *     design
 *     top
 *     model
 *     subsystem
 *     system(model)
 *     accelerator
 *     custom::model
 *
 * The parser does not decide whether the expression denotes a valid HDL
 * simulation target.
 *
 * Semantic analysis performs that validation.
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
 * The body has no fixed cardinality.
 *
 * There is no language-level maximum for:
 *
 *     stimuli
 *     observations
 *     expectations
 *     checkpoints
 *     clauses
 *     nested simulations
 *     expressions
 *
 * Actual implementation limits are controlled by available resources and
 * implementation policy rather than grammar constants.
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
 * Item ownership is deliberately divided:
 *
 *     named clause
 *         generic HDL simulation metadata/intent
 *
 *     resource clause
 *         contextual resource/capability/constraint intent
 *
 *     policy clause
 *         contextual policy attachment
 *
 *     nested construct
 *         hierarchical/co-simulation composition
 *
 *     expression statement
 *         simulation operation expressed through the normal Zamani expression
 *         system
 *
 * No simulator-specific operation list is embedded here.
 *
 * ============================================================================
 */

hdlSimulationItem
    : hdlSimulationNamedClause
    | hdlSimulationResourceClause
    | hdlSimulationPolicyClause
    | hdlSimulationNestedConstruct
    | hdlSimulationExpressionStatement
    ;


/*
 * ============================================================================
 * GENERIC NAMED CLAUSE
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     name: expression;
 *
 * Examples:
 *
 *     stimulus: input;
 *     observe: output;
 *     expect: output == expected;
 *     initial: reset_state;
 *     duration: run_duration;
 *     sampling: sample_period;
 *     checkpoint: state;
 *     waveform: trace;
 *     tolerance: tolerance_value;
 *     seed: seed_value;
 *     termination: condition;
 *     timeout: timeout_value;
 *
 * The identifier is deliberately open-world.
 *
 * This rule does NOT create a closed list of simulation concepts.
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
 * RESOURCE / CAPABILITY / CONSTRAINT / PREFERENCE / HINT CLAUSES
 * ============================================================================
 *
 * These are contextual adapters.
 *
 * They do NOT redefine the universal resource system.
 *
 * They allow HDL simulation intent to participate directly in the repository's
 * canonical resource/capability model.
 *
 * Examples:
 *
 *     requires capability("simulation.event");
 *     requires capability("simulation.acceleration");
 *     requires memory >= required_memory;
 *
 *     constraint topology(requirements);
 *
 *     prefer accelerator;
 *
 *     hint parallel;
 *
 * The semantic layer determines:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *
 * and their actual effect on planning.
 *
 * ============================================================================
 */

hdlSimulationResourceClause
    : REQUIRES
      expression
      SEMICOLON?
    | CONSTRAINT
      expression
      SEMICOLON?
    | PREFER
      expression
      SEMICOLON?
    | HINT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * POLICY CLAUSE
 * ============================================================================
 *
 * This is a contextual attachment point only.
 *
 * It does NOT define policy semantics.
 *
 * Universal policy ownership remains with:
 *
 *     grammar/policies/
 *
 * Simulation-specific policy composition remains with:
 *
 *     grammar/policies/simulation.g4
 *
 * Example:
 *
 *     policy deterministic_execution;
 *
 *     policy simulation_policy;
 *
 * The referenced expression/name is resolved semantically.
 *
 * ============================================================================
 */

hdlSimulationPolicyClause
    : POLICY
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * NESTED SIMULATION
 * ============================================================================
 *
 * Nested simulation permits hierarchical simulation intent and co-simulation
 * descriptions without imposing a fixed hierarchy depth.
 *
 * Example:
 *
 *     simulate top {
 *         simulate subsystem {
 *             observe: signal;
 *         }
 *     }
 *
 * Nesting depth is not a language constant.
 *
 * ============================================================================
 */

hdlSimulationNestedConstruct
    : hdlSimulationDeclaration
    ;


/*
 * ============================================================================
 * EXPRESSION STATEMENT
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
 * None of these operation names are reserved by this grammar.
 *
 * They may be:
 *
 *     library functions;
 *     dialect operations;
 *     semantic intrinsics;
 *     compiler operations;
 *     hardware operations;
 *     future operations.
 *
 * Their meaning is determined downstream.
 *
 * ============================================================================
 */

hdlSimulationExpressionStatement
    : expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser contexts only.
 *
 * The domain-neutral frontend AST should represent the construct conceptually
 * as:
 *
 *     HdlSimulationIntent {
 *         target,
 *         items,
 *         source_span
 *     }
 *
 * where items are represented by semantic-neutral structures such as:
 *
 *     SimulationNamedClause
 *     SimulationResourceIntent
 *     SimulationPolicyAttachment
 *     SimulationNestedIntent
 *     SimulationExpression
 *
 * The AST MUST preserve:
 *
 *     - source span;
 *     - source order;
 *     - target expression;
 *     - clause names;
 *     - clause values;
 *     - nested structure;
 *     - resource intent;
 *     - policy references;
 *     - expression structure.
 *
 * The AST MUST NOT contain:
 *
 *     simulator handles;
 *     device handles;
 *     memory addresses;
 *     physical locations;
 *     physical qubit mappings;
 *     vendor SDK objects;
 *     scheduler state;
 *     routing state;
 *     runtime state.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the target denotes a valid HDL simulation subject;
 *     - what simulation model is required;
 *     - which HDL semantics apply;
 *     - whether clauses are meaningful;
 *     - whether named clauses conflict;
 *     - whether expressions have valid types;
 *     - which effects are produced;
 *     - which capabilities are required;
 *     - which resources are required;
 *     - which constraints apply;
 *     - which preferences may be honored;
 *     - which policies govern execution;
 *     - whether contracts are satisfied;
 *     - whether provenance is available;
 *     - whether deterministic/reproducible execution is requested;
 *     - whether the selected target is feasible.
 *
 * Parsing alone MUST NOT claim any of these properties.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar introduces no effect.
 *
 * Semantic analysis may classify a simulation intent with effects such as:
 *
 *     simulation
 *     measurement
 *     io
 *     network
 *     randomness
 *     distributed
 *     native
 *     foreign
 *
 * according to the actual semantic operation.
 *
 * Effect ownership remains with:
 *
 *     grammar/effects/
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces no physical capability.
 *
 * Capabilities are semantic and open-world.
 *
 * Examples include:
 *
 *     simulation::classical
 *     simulation::hdl
 *     simulation::event
 *     simulation::accelerated
 *     simulation::distributed
 *
 * These are examples of semantic identities, not a closed enumeration.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements are expressions.
 *
 * No physical resource ceiling is represented here.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *     requires compute >= required_compute;
 *     requires capability("simulation.acceleration");
 *
 * The actual feasibility calculation belongs downstream.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies can govern:
 *
 *     - simulator selection;
 *     - execution mode;
 *     - determinism;
 *     - reproducibility;
 *     - resource selection;
 *     - fallback;
 *     - security;
 *     - sandboxing;
 *     - adaptation;
 *     - provenance;
 *     - distributed execution.
 *
 * This grammar only records a policy attachment.
 *
 * It does not evaluate policy.
 *
 * ============================================================================
 * CONTRACT / VERIFICATION INTEGRATION
 * ============================================================================
 *
 * Verification assertions remain owned by:
 *
 *     grammar/hdl/assertions.g4
 *
 * This file MUST NOT redefine:
 *
 *     hdlAssertion
 *     hdlPropertyDeclaration
 *     hdlAssumption
 *     hdlCoverage
 *
 * Simulation expectations may be represented as named simulation clauses:
 *
 *     expect: condition;
 *
 * but their verification meaning is determined by semantic analysis.
 *
 * If a source construct requires an actual HDL verification property, the
 * semantic layer must route it to the HDL verification model rather than
 * creating a second assertion grammar here.
 *
 * ============================================================================
 * TIMING INTEGRATION
 * ============================================================================
 *
 * Timing, clocks and clocking remain owned by their dedicated HDL grammars.
 *
 * This file may carry timing-related simulation intent through generic
 * expressions:
 *
 *     duration: duration_value;
 *     sampling: period;
 *     timeout: timeout_value;
 *
 * It does NOT define:
 *
 *     clock;
 *     clock tree;
 *     physical frequency;
 *     PLL;
 *     timing closure;
 *     clock-domain implementation.
 *
 * Those concerns remain downstream.
 *
 * ============================================================================
 * QUANTUM / HYBRID INTEGRATION
 * ============================================================================
 *
 * HDL simulation may participate in hybrid computation.
 *
 * The grammar does not define quantum syntax.
 *
 * If an expression refers to quantum semantics, the normal semantic pipeline
 * determines the quantum representation and it MUST converge through:
 *
 *     quantum::ir
 *
 * before target-specific quantum realization.
 *
 * HDL simulation does not create a second quantum IR.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Simulation may be realized as:
 *
 *     local;
 *     parallel;
 *     distributed;
 *     accelerated;
 *     heterogeneous;
 *     co-simulation;
 *     future execution modes.
 *
 * These are semantic possibilities, not parser-level enumerations.
 *
 * No node-count or device-count limit is encoded.
 *
 * ============================================================================
 * REPRODUCIBILITY / DETERMINISM
 * ============================================================================
 *
 * Simulation metadata may express:
 *
 *     deterministic: true;
 *     reproducible: policy;
 *     seed: seed_value;
 *
 * through generic named clauses.
 *
 * This grammar does not implement randomness.
 *
 * The semantic/runtime layers must distinguish:
 *
 *     source identity;
 *     semantic identity;
 *     compiler-input identity;
 *     artifact reproducibility;
 *     execution reproducibility;
 *     deterministic result guarantees.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Simulation intent participates in the universal provenance model.
 *
 * Provenance may preserve:
 *
 *     source;
 *     simulation target;
 *     selected policies;
 *     resource requirements;
 *     capability requirements;
 *     transformations;
 *     optimization decisions;
 *     lowering decisions;
 *     execution realization;
 *     verification results.
 *
 * This grammar does not create a second provenance system.
 *
 * ============================================================================
 * LOWERING CONTRACT
 * ============================================================================
 *
 * The semantic lowering path is:
 *
 *     HdlSimulationIntent
 *          |
 *          v
 *     canonical simulation semantic model
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     HDL simulation       hybrid/quantum intent
 *          |                    |
 *          v                    v
 *     HDL/hardware IR       quantum::ir
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *             execution planning
 *                    |
 *                    v
 *             optimization
 *                    |
 *                    v
 *          lowering/specialization
 *                    |
 *                    v
 *          routing/scheduling
 *                    |
 *                    v
 *          resilience/recovery
 *                    |
 *                    v
 *                 ZQN/HAL
 *                    |
 *                    v
 *             target realization
 *
 * This file does NOT own any of those downstream transformations.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * The implementation MUST distinguish them from:
 *
 *     type errors;
 *     effect errors;
 *     capability errors;
 *     resource errors;
 *     policy errors;
 *     contract errors;
 *     provenance errors;
 *     target-feasibility errors;
 *     backend errors;
 *     runtime errors.
 *
 * In particular:
 *
 *     unavailable simulation resource
 *
 * MUST NOT be reported as:
 *
 *     invalid HDL syntax.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no fixed limits for:
 *
 *     simulation items;
 *     nested simulations;
 *     expression size;
 *     target complexity;
 *     resource requirements;
 *     capability sets;
 *     policy references;
 *     HDL model size;
 *     simulation scenarios.
 *
 * Repetition is expressed using ANTLR repetition operators rather than
 * language-level constants.
 *
 * Practical limits belong to:
 *
 *     parser implementation;
 *     compiler configuration;
 *     available memory;
 *     execution resources;
 *     target capabilities.
 *
 * Such implementation limits MUST NOT become language semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST be deterministic for the same token stream.
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no target discovery;
 *     - no runtime calls;
 *     - no random behavior;
 *     - no filesystem access;
 *     - no network access.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical source forms remain supported:
 *
 *     simulate target;
 *
 *     simulate target {
 *         clause: expression;
 *     }
 *
 * The grammar deliberately does not reserve simulation-specific clause names.
 *
 * New clause names can therefore be introduced as identifiers without changing
 * this grammar, provided their lexical spelling is not already reserved by the
 * language.
 *
 * Changes to:
 *
 *     SIMULATE
 *     FROM
 *     WITH
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     POLICY
 *
 * are lexical compatibility changes and must be handled by the canonical
 * lexical compatibility process.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. grammar/hdl/hdl.g4
 *
 *    MUST import:
 *
 *        HdlSimulation
 *
 *    and MUST expose:
 *
 *        hdlSimulationConstruct
 *
 *    from its HDL statement/source dispatch.
 *
 *
 * 2. grammar/antlr/ZamaniLexer.g4
 *
 *    remains the sole lexer consumed by this grammar.
 *
 *
 * 3. grammar/expressions/expressions.g4
 *
 *    remains the sole owner of `expression`.
 *
 *
 * 4. grammar/execution/simulation.g4
 *
 *    remains the owner of universal execution-level simulation expression
 *    syntax.
 *
 *
 * 5. grammar/statements/simulate.g4
 *
 *    remains the owner of universal source-level simulation statement syntax.
 *
 *
 * 6. grammar/effects/simulation.g4
 *
 *    remains the simulation-effect composition boundary.
 *
 *
 * 7. grammar/policies/simulation.g4
 *
 *    remains the simulation-policy composition boundary.
 *
 *
 * 8. grammar/hdl/assertions.g4
 *
 *    remains the HDL verification-property owner.
 *
 *
 * 9. Semantic analysis
 *
 *    consumes the domain-neutral AST and attaches:
 *
 *        types
 *        effects
 *        capabilities
 *        resources
 *        contracts
 *        policies
 *        provenance
 *
 *
 * 10. IR
 *
 *     HDL simulation does not create a competing universal IR.
 *
 *     Hybrid quantum content converges through:
 *
 *         quantum::ir
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required tests belong under the repository's grammar conformance structure,
 * with HDL simulation cases grouped under the HDL/simulation ownership.
 *
 * Minimum categories:
 *
 *     positive
 *     negative
 *     boundary
 *     scalability
 *     compatibility
 *     determinism
 *     cross-domain
 *     diagnostics
 *
 * Positive examples MUST cover:
 *
 *     simulate design;
 *     simulate design {};
 *     simulate design {
 *         stimulus: input;
 *     };
 *     simulate design {
 *         observe: output;
 *         expect: output == expected;
 *     };
 *     simulate design {
 *         requires capability("simulation.event");
 *         constraint topology;
 *         prefer accelerator;
 *         hint parallel;
 *         policy deterministic_execution;
 *     };
 *     nested simulations;
 *     expression-based simulation operations.
 *
 * Negative examples MUST cover:
 *
 *     simulate;
 *     simulate {};
 *     simulate design with ();
 *     malformed named clauses;
 *     malformed resource clauses;
 *     malformed policy clauses;
 *     unterminated simulation bodies.
 *
 * Boundary tests MUST cover:
 *
 *     empty body;
 *     deeply nested simulations;
 *     very large clause lists;
 *     very large expressions;
 *     qualified targets;
 *     target expressions;
 *     mixed HDL and simulation constructs.
 *
 * Scalability tests MUST verify that no language-level capacity constant is
 * required for larger source structures.
 *
 * Cross-domain tests MUST cover, where supported:
 *
 *     HDL + classical;
 *     HDL + quantum;
 *     HDL + hybrid;
 *     HDL + distributed;
 *     HDL + resource requirements;
 *     HDL + policies;
 *     HDL + contracts;
 *     HDL + provenance.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] HdlSimulation is the sole parser grammar for HDL simulation intent.
 *
 *     [ ] hdlSimulationConstruct is the sole public entry point.
 *
 *     [ ] hdl.g4 imports HdlSimulation.
 *
 *     [ ] hdl.g4 dispatches hdlSimulationConstruct.
 *
 *     [ ] The canonical ZamaniLexer is used.
 *
 *     [ ] No lexer rules exist here.
 *
 *     [ ] The canonical expression rule is reused.
 *
 *     [ ] No second expression language exists here.
 *
 *     [ ] Simulation-specific operation names are not hard-coded.
 *
 *     [ ] Simulator vendors are not hard-coded.
 *
 *     [ ] Hardware vendors are not hard-coded.
 *
 *     [ ] Physical devices are not hard-coded.
 *
 *     [ ] Resource capacities are not hard-coded.
 *
 *     [ ] Node/device/thread/qubit/memory limits are not hard-coded.
 *
 *     [ ] Verification syntax is not duplicated.
 *
 *     [ ] Universal simulation syntax is not duplicated.
 *
 *     [ ] Universal simulation execution semantics remain downstream.
 *
 *     [ ] Resource/capability semantics remain downstream.
 *
 *     [ ] Policy semantics remain downstream.
 *
 *     [ ] Effects remain downstream.
 *
 *     [ ] Contracts remain downstream.
 *
 *     [ ] Provenance remains downstream.
 *
 *     [ ] Quantum semantics converge through quantum::ir where applicable.
 *
 *     [ ] No Rust implementation requires unsafe.
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
 *     [ ] Diagnostics distinguish syntax from semantic/resource failures.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file describes HDL SIMULATION INTENT.
 *
 * It does not describe a simulator.
 *
 * It does not allocate resources.
 *
 * It does not select hardware.
 *
 * It does not select a vendor.
 *
 * It does not perform verification.
 *
 * It does not perform synthesis.
 *
 * It does not execute anything.
 *
 * It does not establish physical limits.
 *
 * The same source-level intent must therefore remain meaningful across
 * different simulation implementations and hardware scales, subject to
 * semantic feasibility and available resources.
 *
 * ============================================================================
 */