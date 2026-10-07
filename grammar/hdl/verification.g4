/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hdl/verification.g4
 *
 * GRAMMAR
 * -------
 * HdlVerification
 *
 * STATUS
 * ------
 * CANONICAL PRODUCTION HDL VERIFICATION COMPOSITION GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 or later
 * Rust 2021
 * Safe Rust only
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/specification/
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/hdl/verification.g4
 *          |
 *          +--> grammar/hdl/assertions.g4
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic verification model
 *          |
 *          v
 *     canonical hardware semantic representation
 *          |
 *          +--> simulation
 *          +--> formal verification
 *          +--> coverage
 *          +--> synthesis checks
 *          +--> timing analysis
 *          +--> optimization
 *          +--> target lowering
 *
 * This file is a parser-composition boundary.
 *
 * It is NOT:
 *
 *     - a verification engine;
 *     - a temporal-logic solver;
 *     - a simulator;
 *     - a formal-verification backend;
 *     - a coverage engine;
 *     - a solver interface;
 *     - a vendor language;
 *     - a hardware-target description language.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the canonical HDL verification declaration boundary.
 *
 * It composes the authoritative assertion/property grammar with additional
 * verification constructs whose source syntax genuinely belongs at the HDL
 * verification boundary.
 *
 * The grammar represents PORTABLE VERIFICATION INTENT.
 *
 * It deliberately does not select:
 *
 *     - a simulator;
 *     - a formal solver;
 *     - a model checker;
 *     - a coverage engine;
 *     - a verification vendor;
 *     - a hardware device;
 *     - an FPGA;
 *     - an ASIC;
 *     - a CPU;
 *     - a GPU;
 *     - a QPU;
 *     - a particular target topology.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Assertion/property syntax is owned by:
 *
 *     grammar/hdl/assertions.g4
 *
 * Therefore this file MUST NOT redefine:
 *
 *     hdlAssertion
 *     hdlAssertionBody
 *     hdlImmediateAssertion
 *     hdlPropertyAssertion
 *     hdlPropertyDeclaration
 *     hdlPropertyExpression
 *     hdlPropertyReference
 *     hdlPropertySpecification
 *     hdlVerificationLabel
 *     hdlVerificationArgumentList
 *     hdlVerificationQualifier
 *
 * Those rules are consumed from HdlAssertions.
 *
 * This file owns only the verification COMPOSITION boundary and constructs
 * that are not already owned by HdlAssertions.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     hdlVerificationDeclaration
 *     hdlVerificationItem
 *     hdlAssumption
 *     hdlCoverage
 *
 * This file may also expose small verification composition rules needed by
 * the HDL composition root.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     keywords
 *     identifiers
 *     literals
 *     punctuation
 *     expressions
 *     types
 *     attributes
 *     contracts
 *     generic requirements
 *     capabilities
 *     resources
 *     effects
 *     policies
 *     provenance
 *     clocks
 *     clock declarations
 *     resets
 *     timing declarations
 *     modules
 *     ports
 *     signals
 *     nets
 *     registers
 *     memories
 *     processes
 *     combinational behavior
 *     sequential behavior
 *     state machines
 *     pipelines
 *     generation
 *     synthesis
 *     placement
 *     routing
 *     scheduling
 *     quantum operations
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar consumes the single canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No lexer rules are permitted here.
 *
 * Required canonical verification tokens are:
 *
 *     ASSERT
 *     ASSUME
 *     COVER
 *     PROPERTY
 *
 * ASSERT, ASSUME and PROPERTY already belong to the canonical keyword layer.
 *
 * COVER MUST be added to the canonical keyword/token registry before this
 * grammar is generated:
 *
 *     COVER : 'cover' ;
 *
 * The addition MUST be made in the canonical lexical authority rather than
 * being hidden inside this grammar.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Verification constructs must remain extensible.
 *
 * This grammar therefore MUST NOT enumerate:
 *
 *     - SAT solvers;
 *     - SMT solvers;
 *     - model checkers;
 *     - theorem provers;
 *     - simulators;
 *     - waveform engines;
 *     - vendor tools;
 *     - FPGA vendors;
 *     - ASIC technologies;
 *     - fixed temporal-logic catalogs;
 *     - fixed coverage algorithms.
 *
 * Such concepts are represented downstream through semantic metadata,
 * capabilities, policies, dialects, interoperability layers, or backend
 * configuration.
 *
 * ============================================================================
 * ASSERTION OWNERSHIP
 * ============================================================================
 *
 * Assertions remain owned by:
 *
 *     grammar/hdl/assertions.g4
 *
 * Therefore:
 *
 *     hdlAssertion
 *
 * is imported rather than redefined.
 *
 * This prevents the repository from having multiple meanings for:
 *
 *     assert(...)
 *     assert property(...)
 *
 * ============================================================================
 * PROPERTY OWNERSHIP
 * ============================================================================
 *
 * Named property declarations remain owned by:
 *
 *     grammar/hdl/assertions.g4
 *
 * Examples:
 *
 *     property ready(v, r) = v && r;
 *
 *     property stable(a, b) = a == b;
 *
 * Property declaration syntax MUST NOT be duplicated here.
 *
 * ============================================================================
 * ASSUMPTION SEMANTICS
 * ============================================================================
 *
 * An assumption constrains the verification environment/model.
 *
 * It does NOT prove that the implementation satisfies the assumption.
 *
 * Canonical forms:
 *
 *     assume property(condition);
 *
 *     assume property(condition)
 *         clock(clk);
 *
 *     assume property(condition)
 *         disable(reset);
 *
 * Verification qualifiers are consumed from HdlAssertions.
 *
 * Semantic analysis determines:
 *
 *     - whether the assumption is valid;
 *     - what environment it constrains;
 *     - whether the referenced names exist;
 *     - whether types are compatible;
 *     - whether clocking is legal;
 *     - whether disable semantics are legal;
 *     - whether the assumption is compatible with its enclosing HDL context.
 *
 * ============================================================================
 * COVERAGE SEMANTICS
 * ============================================================================
 *
 * Coverage describes behavior that verification should observe.
 *
 * Canonical form:
 *
 *     cover property(condition);
 *
 * Optional verification qualifiers are supported:
 *
 *     cover property(condition)
 *         clock(clk);
 *
 *     cover property(condition)
 *         disable(reset);
 *
 * Coverage is semantically distinct from assertion.
 *
 * Assertion:
 *
 *     implementation MUST satisfy the property.
 *
 * Coverage:
 *
 *     verification SHOULD determine whether the property/behavior can occur
 *     or is exercised according to the selected verification semantics.
 *
 * The grammar does not decide whether coverage is:
 *
 *     - simulation coverage;
 *     - formal cover;
 *     - runtime observation;
 *     - symbolic exploration;
 *     - another supported verification mode.
 *
 * ============================================================================
 * PROPERTY SPECIFICATION
 * ============================================================================
 *
 * Property specifications are owned by HdlAssertions.
 *
 * This file intentionally consumes:
 *
 *     hdlPropertySpecification
 *
 * rather than reproducing:
 *
 *     LPAREN hdlExpression RPAREN
 *
 * or property-reference syntax.
 *
 * This guarantees that assertion and coverage constructs use exactly the same
 * property syntax.
 *
 * ============================================================================
 * VERIFICATION QUALIFIERS
 * ============================================================================
 *
 * Qualifiers are owned by HdlAssertions.
 *
 * The current canonical qualifier boundary supports constructs such as:
 *
 *     clock(...)
 *     disable(...)
 *
 * through the existing extensible qualifier representation.
 *
 * This file MUST NOT create a second qualifier grammar.
 *
 * ============================================================================
 * TEMPORAL SEMANTICS
 * ============================================================================
 *
 * This file deliberately does NOT enumerate a fixed temporal language.
 *
 * It does not define universal parser-level alternatives for:
 *
 *     always
 *     eventually
 *     until
 *     next
 *     throughout
 *     implication
 *     repetition
 *     strong
 *     weak
 *
 * Temporal semantics belong to the semantic verification layer.
 *
 * If a future temporal construct becomes stable Zamani syntax, it MUST first
 * receive:
 *
 *     - normative specification;
 *     - lexical contract, if lexical reservation is required;
 *     - grammar contract;
 *     - AST contract;
 *     - semantic contract;
 *     - verification/IR contract;
 *     - compatibility decision;
 *     - positive tests;
 *     - negative tests;
 *     - boundary tests;
 *     - scalability tests.
 *
 * ============================================================================
 * LABELS
 * ============================================================================
 *
 * Verification labels are owned by HdlAssertions.
 *
 * Examples:
 *
 *     ready_check: assert(...);
 *
 *     protocol_check:
 *         assume property(...);
 *
 *     progress:
 *         cover property(...);
 *
 * Labels are source-level identity/metadata.
 *
 * They do not identify:
 *
 *     - physical resources;
 *     - solver instances;
 *     - devices;
 *     - threads;
 *     - simulator processes.
 *
 * ============================================================================
 * VERIFICATION ARGUMENTS
 * ============================================================================
 *
 * Assertion/property verification arguments remain owned by HdlAssertions.
 *
 * The verification grammar does not hard-code:
 *
 *     severity
 *     message
 *     solver
 *     backend
 *     waveform
 *     report
 *     engine
 *
 * as universal syntax.
 *
 * Backend-specific metadata belongs in semantic metadata, policies, dialects,
 * or interoperability layers.
 *
 * ============================================================================
 * CONTRACT / POLICY INTEGRATION
 * ============================================================================
 *
 * Verification is compatible with the repository-wide:
 *
 *     contracts
 *     requirements
 *     capabilities
 *     resources
 *     effects
 *     policies
 *     provenance
 *
 * systems.
 *
 * This grammar does not import those grammars merely because verification may
 * consume their semantic information.
 *
 * Semantic analysis is responsible for determining relationships such as:
 *
 *     verification requirement
 *         |
 *         v
 *     capability resolution
 *         |
 *         v
 *     resource feasibility
 *         |
 *         v
 *     policy authorization
 *         |
 *         v
 *     verification planning
 *
 * Resource/capability failures MUST NOT become syntax errors.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Verification syntax itself is declarative.
 *
 * Parsing it does not perform:
 *
 *     simulation
 *     measurement
 *     I/O
 *     network access
 *     solver execution
 *     code generation
 *     native execution
 *
 * If a downstream verification realization has effects, those effects are
 * attached by semantic analysis and execution planning.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Verification constructs may participate in provenance.
 *
 * Semantic provenance may record:
 *
 *     source property
 *     source assertion
 *     source assumption
 *     source coverage goal
 *     derived verification obligation
 *     verification transformation
 *     verification result
 *     evidence
 *     tool/backend identity
 *     policy
 *     version
 *
 * The parser itself remains deterministic and side-effect free.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser contexts only.
 *
 * Semantic lowering MUST use the existing domain-neutral frontend AST.
 *
 * Conceptual mappings:
 *
 *     hdlAssertion
 *         -> canonical verification assertion node
 *
 *     hdlAssumption
 *         -> canonical verification assumption node
 *
 *     hdlCoverage
 *         -> canonical verification coverage node
 *
 *     hdlPropertyDeclaration
 *         -> canonical verification property declaration node
 *
 * This grammar MUST NOT create:
 *
 *     VerificationAst
 *     HdlVerificationAst
 *     FormalAst
 *     CoverageAst
 *
 * as parallel AST hierarchies.
 *
 * Exact Rust AST ownership remains outside grammar/.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - name resolution;
 *     - property resolution;
 *     - property parameter binding;
 *     - expression typing;
 *     - clock validation;
 *     - disable-condition validation;
 *     - assertion context validation;
 *     - assumption context validation;
 *     - coverage context validation;
 *     - verification-mode resolution;
 *     - capability checking;
 *     - resource checking;
 *     - policy checking;
 *     - contract interaction;
 *     - provenance;
 *     - verification backend selection;
 *     - elaboration.
 *
 * None of these operations occur in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar owns NO IR.
 *
 * It MUST NOT introduce:
 *
 *     VerificationIR
 *     AssertionIR
 *     CoverageIR
 *     FormalIR
 *     SolverIR
 *
 * solely to represent parser syntax.
 *
 * Verification semantics must enter the repository's established canonical
 * semantic/IR pipeline.
 *
 * For hardware verification:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic verification model
 *       |
 *       v
 *     canonical hardware representation / IR
 *       |
 *       +--> simulation
 *       +--> formal verification
 *       +--> coverage
 *       +--> synthesis checks
 *       +--> timing analysis
 *       +--> target lowering
 *
 * ============================================================================
 * QUANTUM / HYBRID INTEGRATION
 * ============================================================================
 *
 * HDL verification expressions may reference quantum or hybrid values where
 * the enclosing semantic domain permits them.
 *
 * This grammar does not define:
 *
 *     qubit syntax;
 *     gate syntax;
 *     quantum measurements;
 *     QEC;
 *     quantum routing;
 *     quantum scheduling;
 *     quantum target selection.
 *
 * Quantum meaning remains owned by the quantum subsystem.
 *
 * If verification semantics reference quantum operations, the semantic path
 * remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *
 * ============================================================================
 * HDL INTEGRATION
 * ============================================================================
 *
 * grammar/hdl/hdl.g4 is the HDL composition root.
 *
 * It MUST import:
 *
 *     HdlVerification
 *
 * and its declaration dispatch MUST use:
 *
 *     hdlVerificationDeclaration
 *
 * rather than directly importing/redeclaring individual verification
 * constructs.
 *
 * The HDL composition root therefore owns composition, not verification
 * syntax.
 *
 * ============================================================================
 * SEQUENTIAL / COMBINATIONAL INTEGRATION
 * ============================================================================
 *
 * grammar/hdl/sequential.g4 and grammar/hdl/combinational.g4 MUST NOT create
 * independent HDL assertion languages.
 *
 * If they need assertion support, they must consume the canonical
 * hdlAssertion rule through grammar composition.
 *
 * They MUST NOT redefine:
 *
 *     hdlAssertion
 *     hdlAssumption
 *     hdlCoverage
 *     hdlPropertyDeclaration
 *
 * ============================================================================
 * TIMING INTEGRATION
 * ============================================================================
 *
 * Verification qualifiers such as:
 *
 *     clock(...)
 *     disable(...)
 *
 * do not replace:
 *
 *     grammar/hdl/clocks.g4
 *     grammar/hdl/clocking.g4
 *     grammar/hdl/timing.g4
 *
 * Those grammars remain authoritative for their respective hardware/timing
 * declarations.
 *
 * Verification only records the logical relationship required by a property.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * grammar/hdl/simulation.g4 owns simulation syntax.
 *
 * This grammar does not execute simulation.
 *
 * Semantic analysis may lower:
 *
 *     assertion
 *     assumption
 *     coverage
 *
 * into simulation instrumentation when permitted by the semantic model.
 *
 * ============================================================================
 * SYNTHESIS INTEGRATION
 * ============================================================================
 *
 * Verification constructs may influence synthesis through semantic metadata,
 * constraints, validation, or implementation policy.
 *
 * This grammar does not synthesize hardware.
 *
 * It does not select:
 *
 *     cells
 *     LUTs
 *     DSP blocks
 *     BRAMs
 *     physical registers
 *     physical pins
 *
 * ============================================================================
 * RESOURCE SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits on:
 *
 *     verification declarations
 *     assertions
 *     assumptions
 *     coverage goals
 *     properties
 *     property parameters
 *     qualifier count
 *     verification arguments
 *     nested source structure
 *     hierarchy depth
 *     expression size
 *     identifier size
 *     source-unit size
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_ASSERTIONS
 *     MAX_PROPERTIES
 *     MAX_ASSUMPTIONS
 *     MAX_COVERAGE
 *     MAX_PROPERTY_PARAMETERS
 *     MAX_QUALIFIERS
 *     MAX_VERIFICATION_ARGUMENTS
 *     MAX_CYCLES
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *     MAX_TRACE_LENGTH
 *     MAX_SOLVERS
 *     MAX_VERIFICATION_THREADS
 *     MAX_WAVEFORM_SIZE
 *
 * or equivalent semantic ceilings.
 *
 * Repetition is represented structurally using ANTLR repetition operators.
 *
 * Actual limits, if required for denial-of-service protection or compiler
 * operation, MUST be external configurable resource policies and MUST NOT
 * change the language's meaning.
 *
 * ============================================================================
 * "INFINITY" INTERPRETATION
 * ============================================================================
 *
 * POCO-REAF scalability means:
 *
 *     no artificial language-level ceiling.
 *
 * It does NOT mean:
 *
 *     infinite memory;
 *     infinite parser stack;
 *     infinite compilation time;
 *     infinite solver capacity;
 *     infinite simulation capacity;
 *     infinite hardware.
 *
 * The same source semantics remain valid as the available resources grow,
 * subject to explicit semantic feasibility and policy decisions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text
 *     canonical lexer vocabulary
 *     grammar version
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     target discovery
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     runtime state
 *     solver availability
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no semantic predicates;
 *     - no native calls;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no solver execution.
 *
 * Rust implementations consuming the grammar MUST remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     #![deny(unsafe_code)]
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing verification syntax MUST NOT:
 *
 *     - execute a solver;
 *     - launch a simulator;
 *     - invoke a vendor tool;
 *     - access a file;
 *     - access a network;
 *     - inspect physical hardware;
 *     - load arbitrary code.
 *
 * Such operations belong to controlled downstream compiler/toolchain
 * boundaries.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics should identify:
 *
 *     - malformed assumption;
 *     - malformed coverage declaration;
 *     - missing PROPERTY;
 *     - malformed property specification;
 *     - malformed qualifier;
 *     - malformed argument list;
 *     - missing statement terminator.
 *
 * Semantic diagnostics should distinguish:
 *
 *     - undefined property;
 *     - undefined property parameter;
 *     - invalid expression type;
 *     - invalid clock;
 *     - invalid disable condition;
 *     - invalid verification context;
 *     - unsupported verification mode;
 *     - unavailable verification capability;
 *     - insufficient resources;
 *     - policy rejection;
 *     - unsupported target/backend.
 *
 * Syntax and semantic/resource/backend failures MUST remain distinct.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical syntax is preserved:
 *
 *     assert(condition);
 *     assert property(condition);
 *     property name = expression;
 *
 * This file adds/normalizes the verification composition forms:
 *
 *     assume property(condition);
 *     cover property(condition);
 *
 * ASSUME is already part of the canonical keyword vocabulary.
 *
 * COVER is a new reserved word and therefore requires the coordinated lexical
 * registry update described below.
 *
 * No legacy K_* verification tokens are accepted by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following hold:
 *
 *     [x] It is a parser grammar.
 *     [x] Grammar name matches HdlVerification.
 *     [x] It consumes ZamaniLexer.
 *     [x] It imports HdlAssertions.
 *     [x] It owns verification composition only.
 *     [x] It does not duplicate assertion/property syntax.
 *     [x] It has no K_VERIFY dependency.
 *     [x] It has no K_ASSERT dependency.
 *     [x] It has no K_ASSUME dependency.
 *     [x] It has no K_COVER dependency.
 *     [x] It introduces no backend enumeration.
 *     [x] It introduces no hardware-size limit.
 *     [x] It introduces no fixed verification limit.
 *     [x] It introduces no second AST.
 *     [x] It introduces no second IR.
 *     [x] It performs no execution.
 *     [x] It is deterministic.
 *     [x] It remains target-independent.
 *     [x] It is compatible with safe Rust toolchain integration.
 *
 * Repository integration is complete only after the coordinated changes
 * described in the integration contract below are applied.
 * ============================================================================
 */

parser grammar HdlVerification;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Assertion/property syntax is owned by HdlAssertions.
 */
import HdlAssertions;

/*
 * ============================================================================
 * PUBLIC VERIFICATION DECLARATION
 * ============================================================================
 *
 * This is the ONLY verification declaration boundary that the HDL composition
 * root should consume.
 *
 * Assertions and property declarations are imported from HdlAssertions.
 *
 * Assumptions and coverage are owned here.
 * ============================================================================
 */

hdlVerificationDeclaration
    : hdlPropertyDeclaration
    | hdlAssertion
    | hdlAssumption
    | hdlCoverage
    ;

/*
 * ============================================================================
 * VERIFICATION ITEM
 * ============================================================================
 *
 * This reusable boundary permits callers that need a sequence of verification
 * constructs without inventing another verification AST.
 * ============================================================================
 */

hdlVerificationItem
    : hdlVerificationDeclaration
    ;

/*
 * ============================================================================
 * ASSUMPTION
 * ============================================================================
 *
 * Canonical:
 *
 *     assume property(condition);
 *
 *     assume property(condition) clock(clk);
 *
 *     assume property(condition) disable(reset);
 *
 * Labels are inherited from HdlAssertions.
 * Property syntax is inherited from HdlAssertions.
 * ============================================================================
 */

hdlAssumption
    : hdlVerificationLabel?
      ASSUME
      PROPERTY
      hdlPropertySpecification
      hdlVerificationQualifier*
      SEMICOLON
    ;

/*
 * ============================================================================
 * COVERAGE
 * ============================================================================
 *
 * Canonical:
 *
 *     cover property(condition);
 *
 *     cover property(condition) clock(clk);
 *
 *     cover property(condition) disable(reset);
 *
 * Coverage uses exactly the same property specification and qualifier
 * boundaries as assertion/assumption syntax.
 * ============================================================================
 */

hdlCoverage
    : hdlVerificationLabel?
      COVER
      PROPERTY
      hdlPropertySpecification
      hdlVerificationQualifier*
      SEMICOLON
    ;