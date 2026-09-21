/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/hdl.g4
 *
 * STATUS
 * ------
 * CANONICAL HDL STATEMENT-COMPOSITION GRAMMAR
 *
 * PURPOSE
 * -------
 * This file owns the statement-level HDL composition boundary for Zamani.
 *
 * It does NOT implement HDL declarations, expressions, types, ports,
 * memories, clocks, pipelines, state machines, processes, or hardware
 * semantics. Those remain owned by grammar/hdl/.
 *
 * This file exists because HDL constructs can occur inside executable /
 * procedural hardware contexts while still needing to participate in the
 * universal Zamani statement pipeline.
 *
 * ============================================================================
 * RUST BASELINE
 * ============================================================================
 *
 * Rust compatibility target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * This grammar contains no Rust actions, predicates, unsafe code, I/O,
 * filesystem access, networking, hardware discovery, runtime execution,
 * randomness, or target-specific implementation.
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
 *     ZamaniParser
 *          |
 *          v
 *     Statements
 *          |
 *          +--> hdlStatement
 *                    |
 *                    v
 *             THIS FILE
 *                    |
 *                    +--> HDL assignment
 *                    +--> HDL process
 *                    +--> HDL always block
 *                    +--> combinational behavior
 *                    +--> sequential behavior
 *                    +--> state-machine behavior
 *                    +--> pipeline behavior
 *                    +--> instance behavior
 *                    +--> generate behavior
 *                    +--> timing behavior
 *                    +--> HDL assertion
 *                    +--> HDL block
 *                    +--> HDL expression statement
 *                    |
 *                    v
 *             grammar/hdl/
 *                    |
 *                    v
 *             domain-neutral AST
 *                    |
 *                    v
 *             semantic analysis
 *                    |
 *                    v
 *             canonical HDL / hardware semantics
 *                    |
 *                    +--> optimization
 *                    +--> synthesis
 *                    +--> scheduling
 *                    +--> placement
 *                    +--> routing
 *                    +--> target lowering
 *                    |
 *                    v
 *             FPGA / ASIC / CPU / GPU / accelerator /
 *             simulator / future target
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     hdlStatement
 *     hdlStatementList
 *     HDL statement-family composition
 *     statement-level delegation into grammar/hdl/
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     HDL lexical tokens
 *     identifiers
 *     qualified names
 *     HDL expressions
 *     HDL types
 *     module declarations
 *     module ports
 *     signals
 *     nets
 *     registers
 *     memories
 *     clocks
 *     resets
 *     interfaces
 *     pipelines
 *     state-machine definitions
 *     timing semantics
 *     synthesis
 *     physical placement
 *     physical routing
 *     scheduling
 *     FPGA selection
 *     ASIC selection
 *     vendor selection
 *     hardware discovery
 *     resource allocation
 *     runtime execution
 *
 * Concrete HDL constructs remain owned by grammar/hdl/.
 *
 * ============================================================================
 * SINGLE-SOURCE-OF-TRUTH RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative hdlStatement rule in the assembled
 * production parser.
 *
 * This file is the owner of that rule at the universal statement layer.
 *
 * Legacy placeholder forms such as:
 *
 *     hdlStatement
 *         : HDL_STATEMENT
 *
 * MUST NOT remain as competing production implementations.
 *
 * HDL_STATEMENT may remain temporarily in migration/reference material, but
 * it must not be the effective production rule.
 *
 * ============================================================================
 * HDL DECLARATION VS HDL STATEMENT
 * ============================================================================
 *
 * HDL declarations and HDL statements are deliberately distinguished.
 *
 * HDL DECLARATIONS include constructs such as:
 *
 *     module
 *     interface
 *     signal
 *     net
 *     register
 *     memory
 *     clock
 *     reset
 *     type
 *     parameter
 *
 * Those remain owned by the HDL declaration/module grammar.
 *
 * HDL STATEMENTS describe behavior or executable/structural actions inside an
 * already established HDL context.
 *
 * Examples include:
 *
 *     assignment
 *     process
 *     always
 *     combinational behavior
 *     sequential behavior
 *     state-machine behavior
 *     pipeline behavior
 *     instance operation
 *     generate operation
 *     timing statement
 *     assertion
 *     nested HDL block
 *     expression statement
 *
 * This distinction prevents the universal statement dispatcher from becoming
 * a second HDL module grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * HDL statements express portable hardware intent.
 *
 * They MUST NOT encode universal machine limits.
 *
 * This file therefore imposes no maximum on:
 *
 *     statements
 *     assignments
 *     processes
 *     state transitions
 *     pipeline stages
 *     instances
 *     generated instances
 *     nested blocks
 *     dimensions
 *     widths
 *     clocks
 *     memories
 *     hardware modules
 *     hardware domains
 *
 * Any finite limitation is an implementation/resource/target concern rather
 * than a grammar-level language limit.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NOT select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     device
 *     vendor
 *     physical pin
 *     physical register
 *     physical memory address
 *     routing path
 *     physical clock network
 *     physical placement
 *
 * Source-level statements describe intent.
 *
 * Target realization is downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A statement may eventually carry or reference semantic requirements such as:
 *
 *     capability("hardware.pipeline")
 *     capability("memory.read")
 *     capability("clocked-sequential-logic")
 *
 * or requirements such as:
 *
 *     latency(...)
 *     throughput(...)
 *     timing(...)
 *     memory(...)
 *
 * This grammar does not determine whether the target satisfies them.
 *
 * Resource discovery and capability evaluation remain downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     source tokens
 *     grammar version
 *     parser configuration
 *     explicitly selected dialect configuration
 *
 * It MUST NOT depend upon:
 *
 *     hardware
 *     runtime state
 *     filesystem state
 *     network state
 *     environment state
 *     randomness
 *     wall-clock time
 *     target availability
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no embedded Rust;
 *     - contains no unsafe Rust;
 *     - contains no semantic predicates;
 *     - performs no I/O;
 *     - performs no hardware discovery;
 *     - performs no runtime execution;
 *     - performs no resource allocation.
 *
 * Rust 1.97 / 1.97.1 compatibility is enforced by the generated frontend and
 * repository build system, not by grammar actions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve the complete syntactic structure required by the
 * frontend AST, including:
 *
 *     source span
 *     statement kind
 *     child statements
 *     expressions
 *     assignments
 *     timing expressions
 *     state/pipeline structure
 *     instance structure
 *     assertions
 *     attributes
 *     modifiers
 *
 * No physical hardware information may be inferred merely from this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Successful parsing means only:
 *
 *     the HDL statement has valid source syntax.
 *
 * It does NOT mean:
 *
 *     the hardware is realizable;
 *     timing is satisfiable;
 *     a target exists;
 *     resources are available;
 *     synthesis succeeds;
 *     placement succeeds;
 *     routing succeeds;
 *     scheduling succeeds;
 *     the design fits an FPGA;
 *     the design fits an ASIC;
 *     a particular device is selected.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * HDL statements must lower through the repository's canonical frontend
 * semantic path into the canonical HDL/hardware representation.
 *
 * This file MUST NOT create:
 *
 *     physical netlists
 *     gate-level implementations
 *     FPGA mappings
 *     ASIC mappings
 *     placement data
 *     routing data
 *     timing closure data
 *     device assignments
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * HDL statements may participate in programs containing:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     distributed computation
 *     AI/ML
 *     data processing
 *     networking
 *     security
 *     accelerators
 *     embedded systems
 *     HPC
 *
 * This file does not create separate languages for those domains.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * HDL statements MUST remain separate from quantum semantic realization.
 *
 * If a hardware design contains quantum behavior, the eventual semantic
 * lowering must preserve the established architecture:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> decomposition
 *       +--> routing
 *       +--> scheduling
 *       +--> QEC / resilience
 *       +--> ZQN
 *       +--> HAL
 *       |
 *       v
 *     target realization
 *
 * This statement grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * The detailed HDL grammar currently lives in:
 *
 *     grammar/hdl/hdl.g4
 *
 * That grammar exposes the concrete HDL constructs consumed below.
 *
 * This file therefore acts as a thin composition layer.
 *
 * The integration owner should import this grammar into:
 *
 *     grammar/statements/statements.g4
 *
 * using:
 *
 *     import HdlStatements;
 *
 * The universal statement dispatcher then exposes:
 *
 *     statement
 *         : ...
 *         | hdlStatement
 *         | ...
 *         ;
 *
 * The universal parser's HDL domain adapter also consumes the same
 * hdlStatement rule.
 *
 * ============================================================================
 * IMPORTANT LEGACY INTEGRATION
 * ============================================================================
 *
 * The current repository also contains an older generic integration point:
 *
 *     hdlStatement
 *         : HDL_STATEMENT
 *
 * in legacy/core parser composition.
 *
 * That placeholder MUST cease to be authoritative.
 *
 * There must be one effective hdlStatement rule after ANTLR composition.
 *
 * ============================================================================
 * CANONICAL HDL DELEGATES
 * ============================================================================
 *
 * The existing grammar/hdl/hdl.g4 already owns concrete productions such as:
 *
 *     hdlAssignment
 *     hdlProcessDeclaration
 *     hdlAlwaysDeclaration
 *     hdlCombinationalDeclaration
 *     hdlSequentialDeclaration
 *     hdlStateMachineDeclaration
 *     hdlPipelineDeclaration
 *     hdlInstanceDeclaration
 *     hdlGenerateDeclaration
 *     hdlTimingDeclaration
 *     hdlAssertion
 *     hdlBlockDeclaration
 *     hdlExpressionStatement
 *
 * This file composes those rules rather than copying them.
 *
 * ============================================================================
 * STATEMENT COMPOSITION
 * ============================================================================
 */

parser grammar HdlStatements;

options {
    tokenVocab = ZamaniLexer;
}

import hdl;

/*
 * ============================================================================
 * CANONICAL HDL STATEMENT
 * ============================================================================
 *
 * This is deliberately a composition rule.
 *
 * No HDL implementation semantics are defined here.
 *
 * Ordering is kept explicit so that concrete HDL statement forms remain
 * discoverable and independently testable.
 *
 * ============================================================================
 */

hdlStatement
    : hdlAssignment
    | hdlProcessDeclaration
    | hdlAlwaysDeclaration
    | hdlCombinationalDeclaration
    | hdlSequentialDeclaration
    | hdlStateMachineDeclaration
    | hdlPipelineDeclaration
    | hdlInstanceDeclaration
    | hdlGenerateDeclaration
    | hdlTimingDeclaration
    | hdlAssertion
    | hdlBlockDeclaration
    | hdlExpressionStatement
    ;

/*
 * ============================================================================
 * HDL STATEMENT LIST
 * ============================================================================
 *
 * A list has no grammar-defined finite cardinality.
 *
 * This is useful for process/block/module composition adapters and for
 * conformance tooling.
 *
 * ============================================================================
 */

hdlStatementList
    : hdlStatement*
    ;

/*
 * ============================================================================
 * NON-EMPTY HDL STATEMENT LIST
 * ============================================================================
 */

hdlStatementListNonEmpty
    : hdlStatement+
    ;

/*
 * ============================================================================
 * OPTIONAL HDL STATEMENT LIST
 * ============================================================================
 */

hdlStatementListOptional
    : hdlStatement*
    ;

/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No hardware/resource limit is encoded.
 *
 * There is no:
 *
 *     MAX_HDL_STATEMENTS
 *     MAX_MODULES
 *     MAX_PROCESSES
 *     MAX_PIPELINE_STAGES
 *     MAX_STATE_COUNT
 *     MAX_INSTANCES
 *     MAX_SIGNALS
 *     MAX_REGISTERS
 *     MAX_MEMORIES
 *     MAX_WIDTH
 *     MAX_CLOCKS
 *     MAX_DEVICES
 *     MAX_FPGAS
 *     MAX_ASICS
 *
 * Numeric values appearing in expressions remain source semantics.
 *
 * ============================================================================
 * COMPLEXITY / SCALABILITY CONTRACT
 * ============================================================================
 *
 * ANTLR repetition is intentionally unbounded at the language level.
 *
 * Practical limits imposed by:
 *
 *     parser memory
 *     parser stack/resource policy
 *     compiler resources
 *     host resources
 *
 * are implementation safeguards and MUST NOT be transformed into language
 * constants.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser/frontend.
 *
 * Semantic diagnostics remain downstream, including:
 *
 *     impossible timing
 *     incompatible clock domains
 *     invalid hardware capability
 *     unsatisfied resource requirement
 *     illegal state transition
 *     invalid pipeline dependency
 *     invalid assignment width
 *     unsupported target capability
 *     impossible synthesis constraint
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid HDL statement syntax remains owned by grammar/hdl/.
 *
 * This file adds a composition boundary rather than changing the semantics of
 * the concrete HDL constructs.
 *
 * Migration must therefore preserve:
 *
 *     hdlAssignment
 *     hdlProcessDeclaration
 *     hdlAlwaysDeclaration
 *     hdlCombinationalDeclaration
 *     hdlSequentialDeclaration
 *     hdlStateMachineDeclaration
 *     hdlPipelineDeclaration
 *     hdlInstanceDeclaration
 *     hdlGenerateDeclaration
 *     hdlTimingDeclaration
 *     hdlAssertion
 *     hdlBlockDeclaration
 *     hdlExpressionStatement
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * At minimum, test one valid example for every alternative:
 *
 *     assignment
 *     process
 *     always
 *     combinational
 *     sequential
 *     state machine
 *     pipeline
 *     instance
 *     generate
 *     timing
 *     assertion
 *     block
 *     expression statement
 *
 * NEGATIVE
 * --------
 *
 * Reject:
 *
 *     malformed assignment
 *     malformed process
 *     malformed always construct
 *     malformed combinational construct
 *     malformed sequential construct
 *     malformed state-machine construct
 *     malformed pipeline construct
 *     malformed instance
 *     malformed generate
 *     malformed timing declaration
 *     malformed assertion
 *     malformed HDL block
 *     malformed HDL expression statement
 *
 * BOUNDARY
 * --------
 *
 * Test:
 *
 *     zero HDL statements
 *     one HDL statement
 *     many HDL statements
 *     deeply nested HDL blocks
 *     many pipeline stages
 *     many state transitions
 *     many generated instances
 *     symbolic widths
 *     symbolic dimensions
 *     symbolic timing expressions
 *
 * SCALABILITY
 * -----------
 *
 * Tests MUST demonstrate that no grammar-level limit exists for:
 *
 *     statement count
 *     process count
 *     state count
 *     transition count
 *     pipeline stage count
 *     instance count
 *     generated-instance count
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Tests should cover HDL used alongside:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     distributed computation
 *     accelerator intent
 *
 * DETERMINISM
 * -----------
 *
 * Repeated parsing of identical input under identical configuration must
 * produce equivalent parse-tree structure.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] HdlStatements is the sole owner of the statement-level hdlStatement
 *         composition rule.
 *
 *     [ ] Concrete HDL syntax remains owned by grammar/hdl/.
 *
 *     [ ] No HDL syntax is duplicated here.
 *
 *     [ ] No lexer rules are defined here.
 *
 *     [ ] No physical hardware limits are defined here.
 *
 *     [ ] No target selection is performed here.
 *
 *     [ ] No synthesis behavior is defined here.
 *
 *     [ ] No placement/routing/scheduling behavior is defined here.
 *
 *     [ ] No IR is constructed here.
 *
 *     [ ] No quantum IR is constructed here.
 *
 *     [ ] quantum::ir remains the canonical quantum semantic boundary.
 *
 *     [ ] The rule is imported by grammar/statements/statements.g4.
 *
 *     [ ] The legacy HDL_STATEMENT placeholder is removed from authoritative
 *         production composition.
 *
 *     [ ] The canonical HDL grammar provides every referenced rule exactly once.
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
 *     [ ] Rust 1.97 / 1.97.1 frontend generation and compilation pass.
 *
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */