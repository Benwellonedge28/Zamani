
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File: grammar/macros/invocations.g4
 * Grammar: invocations
 *
 * Status: Canonical macro invocation grammar component
 * Compiler: Rust 1.97 / 1.97.1
 * Edition: Rust 2021
 * Safety: Safe Rust only
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * Defines the canonical syntax for invoking Zamani macros.
 *
 * This grammar owns:
 *
 *   - macroPath
 *   - macroInvocation
 *   - macroExpression
 *
 * This grammar does not own:
 *
 *   - lexical tokens
 *   - identifiers
 *   - qualified-name syntax
 *   - ordinary function calls
 *   - argument-list syntax
 *   - general expressions
 *   - macro declarations
 *   - macro parameters
 *   - macro bodies
 *   - macro resolution
 *   - macro expansion
 *   - macro execution
 *   - hygiene implementation
 *   - source-span implementation
 *   - AST implementation
 *   - semantic analysis
 *   - type checking
 *   - capability checking
 *   - resource negotiation
 *   - target selection
 *   - compiler optimization
 *   - quantum::ir
 *   - classical IR
 *   - HDL IR
 *   - hardware realization
 *   - routing
 *   - scheduling
 *   - QEC
 *   - ZQN
 *   - HAL
 *   - runtime execution
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * Canonical lexer
 *      |
 *      v
 * Canonical parser
 *      |
 *      +--> macro invocation
 *      |
 *      v
 * Frontend AST
 *      |
 *      v
 * Name resolution
 *      |
 *      v
 * Macro resolution
 *      |
 *      v
 * Controlled macro expansion
 *      |
 *      v
 * Hygiene and provenance
 *      |
 *      v
 * Semantic analysis
 *      |
 *      v
 * Canonical semantic representation
 *      |
 *      +--> Classical representation
 *      +--> quantum::ir
 *      +--> HDL/hardware representation
 *      +--> Distributed representation
 *      +--> Future domain representations
 *      |
 *      v
 * Optimization and lowering
 *      |
 *      v
 * Routing / Scheduling / Resilience
 *      |
 *      v
 * ZQN / HAL / Backend
 *      |
 *      v
 * Execution
 *
 * Parsing must never execute or expand a macro.
 *
 * ============================================================================
 * 3. POCO-REAF
 * ============================================================================
 *
 * Macro invocation is a portable source-language construct.
 *
 * The same macro invocation syntax must remain meaningful across:
 *
 *   - embedded systems
 *   - classical CPUs
 *   - multicore processors
 *   - GPUs
 *   - FPGAs
 *   - ASICs
 *   - quantum processors
 *   - quantum simulators
 *   - heterogeneous accelerators
 *   - distributed systems
 *   - HPC
 *   - cloud infrastructure
 *   - future computing architectures
 *
 * The grammar MUST NOT require a particular physical target.
 *
 * It MUST NOT encode:
 *
 *   MAX_QUBITS
 *   MAX_CPUS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_THREADS
 *   MAX_TENSOR_RANK
 *   MAX_REGISTER_WIDTH
 *   MAX_NETWORK_SIZE
 *   MAX_DEVICE_COUNT
 *
 * Nor equivalent artificial limits.
 *
 * ============================================================================
 * 4. SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level maximum on:
 *
 *   - macro invocation count
 *   - macro argument count
 *   - namespace depth
 *   - source size
 *   - expression size
 *   - nesting
 *   - expansion output size
 *
 * Actual implementations remain subject to available resources.
 *
 * Implementations MAY use configurable admission controls for:
 *
 *   - source bytes
 *   - tokens
 *   - parser memory
 *   - AST nodes
 *   - expansion steps
 *   - expansion depth
 *   - generated nodes
 *   - compilation time
 *   - compiler memory
 *
 * These are compiler resource policies, not language semantics.
 *
 * A resource policy must not silently become a permanent language limit.
 *
 * ============================================================================
 * 5. LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer is the sole owner of token definitions.
 *
 * Required token vocabulary:
 *
 *   BANG
 *   LPAREN
 *   RPAREN
 *
 * Additional tokens are consumed indirectly by shared parser rules.
 *
 * This file MUST NOT define lexer rules.
 *
 * In particular, it MUST NOT redefine:
 *
 *   BANG
 *   IDENTIFIER
 *   DOUBLE_COLON
 *   COMMA
 *
 * The actual token names must be reconciled with the canonical ZamaniLexer
 * before generating the composed parser.
 *
 * ============================================================================
 * 6. SHARED RULE CONTRACT
 * ============================================================================
 *
 * This grammar consumes the following canonical parser rules:
 *
 *   qualifiedName
 *   argumentList
 *
 * Their ownership remains outside this file.
 *
 * qualifiedName owns:
 *
 *   - identifier segments
 *   - namespace separators
 *   - qualified-name structure
 *   - namespace nesting
 *
 * argumentList owns:
 *
 *   - argument expressions
 *   - argument separators
 *   - trailing-comma policy
 *   - supported argument forms
 *
 * expression grammar owns the general expression hierarchy.
 *
 * No duplicate shared rule definitions are permitted here.
 *
 * ============================================================================
 * 7. CANONICAL INVOCATION SYNTAX
 * ============================================================================
 *
 * Empty invocation:
 *
 *   build!()
 *
 * Single argument:
 *
 *   build!(value)
 *
 * Multiple arguments:
 *
 *   build!(value, size, configuration)
 *
 * Qualified invocation:
 *
 *   math::build!(value)
 *
 * Deeply qualified invocation:
 *
 *   package::module::submodule::build!(value)
 *
 * Macro invocation and ordinary function calls are structurally distinct:
 *
 *   build(value)
 *   build!(value)
 *
 * The exclamation mark identifies macro invocation syntax.
 *
 * It does not imply immediate execution.
 *
 * ============================================================================
 * 8. AST CONTRACT
 * ============================================================================
 *
 * The existing frontend AST remains authoritative.
 *
 * The parser must preserve sufficient information for the canonical macro
 * invocation representation to retain:
 *
 *   - invocation identity
 *   - source span
 *   - source identity
 *   - qualified macro path
 *   - ordered arguments
 *   - argument source locations
 *   - invocation provenance
 *
 * The grammar must not introduce another AST hierarchy.
 *
 * In particular, this file must not require a separate:
 *
 *   MacroInvocationAst
 *   MacroExpressionAst
 *   MacroPathAst
 *
 * The existing canonical frontend representation must be reused.
 *
 * ============================================================================
 * 9. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structural validity only.
 *
 * The following are downstream responsibilities:
 *
 *   - determining whether a macro exists
 *   - determining whether it is accessible
 *   - resolving imported names
 *   - resolving qualified names
 *   - matching arguments to parameters
 *   - checking generic constraints
 *   - checking compile-time capabilities
 *   - checking expansion permissions
 *   - enforcing expansion resource policies
 *   - validating generated syntax
 *   - validating generated semantics
 *   - checking generated resource requirements
 *
 * Unknown macro names may be syntactically valid.
 *
 * For example:
 *
 *   unknown_macro!(value)
 *
 * The parser must not reject a structurally valid invocation merely because
 * its meaning is not yet known.
 *
 * ============================================================================
 * 10. DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * Macro arguments may participate in any supported Zamani domain:
 *
 *   - classical computing
 *   - quantum computing
 *   - hybrid computing
 *   - HDL
 *   - hardware/software co-design
 *   - AI and machine learning
 *   - distributed computing
 *   - networking
 *   - data processing
 *   - scientific computing
 *   - security
 *   - future computing domains
 *
 * This grammar does not enumerate those domains.
 *
 * ============================================================================
 * 11. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Macros may produce quantum source constructs.
 *
 * This grammar MUST NOT:
 *
 *   - enumerate quantum gates
 *   - allocate physical qubits
 *   - impose a qubit maximum
 *   - select a QPU
 *   - select a quantum simulator
 *   - determine native gate availability
 *   - determine physical topology
 *   - perform quantum routing
 *   - perform quantum scheduling
 *   - implement QEC
 *   - implement ZQN
 *   - create a second quantum IR
 *
 * Generated quantum semantics must eventually reach:
 *
 *   quantum::ir
 *
 * ============================================================================
 * 12. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Macro invocation must not implicitly select:
 *
 *   - a processor
 *   - a GPU
 *   - an FPGA
 *   - an ASIC
 *   - a QPU
 *   - a device identifier
 *   - a physical register
 *   - a physical memory bank
 *   - a network node
 *   - a machine topology
 *
 * Portable resource requirements belong to the canonical resource and
 * capability layers.
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * 13. SECURITY AND EXECUTION BOUNDARY
 * ============================================================================
 *
 * Parsing a macro invocation MUST have no external side effects.
 *
 * The parser MUST NOT:
 *
 *   - execute macro code
 *   - read arbitrary files
 *   - write arbitrary files
 *   - access the network
 *   - spawn processes
 *   - inspect hardware
 *   - access secrets
 *   - invoke external compiler plugins
 *   - mutate compiler-global state
 *
 * Macro expansion must be a separate controlled compiler phase.
 *
 * It must be governed by:
 *
 *   - explicit capabilities
 *   - security policy
 *   - deterministic execution rules
 *   - provenance
 *   - hygiene
 *   - resource budgets
 *   - diagnostic attribution
 *
 * ============================================================================
 * 14. DETERMINISM
 * ============================================================================
 *
 * The same canonical token stream must produce the same structural parse.
 *
 * This grammar does not implement deterministic expansion.
 *
 * The downstream macro engine must define deterministic:
 *
 *   - resolution
 *   - argument binding
 *   - expansion ordering
 *   - generated-source identity
 *   - hygiene
 *   - provenance
 *   - diagnostics
 *
 * ============================================================================
 * 15. COMPATIBILITY
 * ============================================================================
 *
 * The established invocation shape is preserved:
 *
 *   macroPath BANG LPAREN argumentList? RPAREN
 *
 * No unnecessary syntax change is introduced.
 *
 * Any future syntax change must include:
 *
 *   - language versioning
 *   - migration rules
 *   - compatibility tests
 *   - negative tests
 *   - diagnostics
 *
 * ============================================================================
 * 16. INTEGRATION OWNERSHIP
 * ============================================================================
 *
 * macros/macros.g4:
 *   Composition boundary.
 *
 * macros/declarations.g4:
 *   Declaration syntax.
 *
 * macros/invocations.g4:
 *   Invocation syntax. This file.
 *
 * macros/expansion.g4:
 *   Explicit expansion-related source syntax, if supported.
 *
 * macros/hygiene.g4:
 *   Explicit hygiene-related source syntax, if supported.
 *
 * expressions/:
 *   Integrates macroExpression into the canonical expression hierarchy.
 *
 * core/:
 *   Owns qualifiedName and shared naming infrastructure.
 *
 * Canonical argument grammar:
 *   Owns argumentList.
 *
 * Frontend AST:
 *   Owns invocation representation and source provenance.
 *
 * Compiler macro engine:
 *   Owns resolution and controlled expansion.
 *
 * Semantic analysis:
 *   Owns validation of expanded syntax.
 *
 * ============================================================================
 * 17. ANTLR COMPOSITION
 * ============================================================================
 *
 * This file is an independently maintained parser grammar component.
 *
 * Its token vocabulary is supplied by ZamaniLexer.
 *
 * Shared parser rules are supplied by the canonical parser composition.
 *
 * The composition/build system must resolve shared rule references without
 * introducing duplicate production ownership.
 *
 * This component must not become a second root grammar.
 *
 * ============================================================================
 * 18. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] Invocation ownership is explicit.
 *   [x] Canonical invocation syntax is preserved.
 *   [x] Qualified paths are delegated.
 *   [x] Arguments are delegated.
 *   [x] Lexer ownership is preserved.
 *   [x] No duplicate expression grammar is introduced.
 *   [x] No hardware limits are introduced.
 *   [x] No fixed macro counts are introduced.
 *   [x] No macro execution is performed during parsing.
 *   [x] Quantum IR ownership remains canonical.
 *   [x] AST ownership remains canonical.
 *   [x] Semantic ownership remains downstream.
 *   [x] Integration contracts are specified.
 *
 * Repository-level completion additionally requires:
 *
 *   - ANTLR generation validation
 *   - canonical token-vocabulary validation
 *   - shared-rule resolution validation
 *   - AST mapping tests
 *   - macro engine integration tests
 *   - positive invocation tests
 *   - negative invocation tests
 *   - boundary tests
 *   - scalability tests
 *   - determinism tests
 *   - compatibility tests
 *   - safe-Rust CI validation
 *
 * ============================================================================
 * END OF ARCHITECTURAL CONTRACT
 * ============================================================================
 */

parser grammar invocations;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * MACRO PATH
 * ============================================================================
 *
 * Ownership:
 *   This rule identifies the path position of a macro invocation.
 *
 * Delegation:
 *   qualifiedName owns the actual name/path syntax.
 *
 * Examples:
 *
 *   build
 *   math::build
 *   package::math::build
 *
 * No namespace-depth limit is introduced.
 *
 * Name existence, visibility and resolution are semantic concerns.
 */

macroPath
    : qualifiedName
    ;

/*
 * ============================================================================
 * MACRO INVOCATION
 * ============================================================================
 *
 * Canonical syntax:
 *
 *   name!()
 *   name!(argument)
 *   name!(argument1, argument2)
 *   module::name!(argument)
 *
 * The optional argument list delegates to the canonical argumentList rule.
 *
 * Empty invocations are valid.
 *
 * A non-empty invocation must conform to argumentList.
 *
 * This rule does not evaluate arguments or execute macros.
 */

macroInvocation
    : macroPath
      BANG
      LPAREN
      argumentList?
      RPAREN
    ;

/*
 * ============================================================================
 * MACRO EXPRESSION
 * ============================================================================
 *
 * Single integration boundary with the canonical expression grammar.
 *
 * The canonical expression grammar must reference macroExpression exactly
 * once in its expression alternatives.
 *
 * This rule must not be duplicated in another macro grammar component.
 */

macroExpression
    : macroInvocation
    ;
