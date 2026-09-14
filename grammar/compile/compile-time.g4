/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/compile-time.g4
 *
 * Grammar:
 *     CompileTime
 *
 * Status:
 *     Production compilation-control parser grammar
 *
 * Purpose:
 *     Defines source-level compilation-time control constructs.
 *
 * Architectural position:
 *
 *     Source
 *       |
 *       v
 *     Lexer
 *       |
 *       v
 *     Parser
 *       |
 *       v
 *     Frontend AST
 *       |
 *       v
 *     Semantic Analysis
 *       |
 *       +--> compile-time evaluation
 *       +--> specialization
 *       +--> feature selection
 *       +--> conditional compilation
 *       +--> lowering
 *       |
 *       v
 *     Canonical IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> hardware-independent representations
 *       |
 *       v
 *     optimization / routing / scheduling / HAL / runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - compilation-control syntax;
 *   - source-level conditional compilation;
 *   - compile-time feature selection;
 *   - compile-time configuration selection;
 *   - compile-time requirement declarations;
 *   - compile-time target-independent specialization requests;
 *   - compile-time inclusion/exclusion of source branches;
 *   - compilation-time assertions as statement/control constructs;
 *   - compilation-time iteration/control boundaries where explicitly
 *     supported by the language;
 *   - compilation-time annotations/options that affect compilation semantics.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - ordinary expressions;
 *   - expression precedence;
 *   - compile-time expressions;
 *   - ordinary functions;
 *   - compile-time functions;
 *   - macros;
 *   - macro expansion;
 *   - reflection;
 *   - types;
 *   - resources;
 *   - hardware;
 *   - targets;
 *   - execution;
 *   - scheduling;
 *   - routing;
 *   - optimization algorithms;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR;
 *   - QEC;
 *   - ZQN;
 *   - runtime dispatch;
 *   - hardware discovery.
 *
 * IMPORTANT:
 *
 * `grammar/expressions/compile-time.g4` owns expression-level compile-time
 * constructs.
 *
 * `grammar/functions/compile-time-functions.g4` owns compile-time-function
 * declaration integration.
 *
 * This file must therefore NOT redefine those constructs.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Compilation-time syntax describes source intent.
 *
 * It MUST NOT require a particular:
 *
 *   - CPU;
 *   - GPU;
 *   - FPGA;
 *   - ASIC;
 *   - QPU;
 *   - vendor;
 *   - machine topology;
 *   - number of processors;
 *   - number of qubits;
 *   - memory capacity;
 *   - accelerator count;
 *   - physical address;
 *   - deployment location.
 *
 * Target-specific information must be represented by the appropriate
 * target/resource/capability grammar and resolved downstream.
 *
 * Compilation-time syntax therefore remains scalable from the smallest
 * supported execution environment to arbitrarily large environments,
 * subject only to available implementation resources and semantic constraints.
 *
 * ============================================================================
 * RUST
 * ============================================================================
 *
 * Runtime/compiler integration target:
 *
 *   Rust 1.97
 *   Rust 1.97.1
 *
 * Generated parser integration must use safe Rust only.
 *
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no semantic actions;
 *   - no filesystem access;
 *   - no network access;
 *   - no unsafe code;
 *   - no host execution;
 *   - no mutable compiler-global state.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No fixed limits are encoded here.
 *
 * In particular, this grammar contains no:
 *
 *   MAX_TARGETS
 *   MAX_FEATURES
 *   MAX_SPECIALIZATIONS
 *   MAX_CONFIGURATIONS
 *   MAX_ITERATIONS
 *   MAX_DEVICES
 *   MAX_QUBITS
 *   MAX_CORES
 *   MAX_THREADS
 *   MAX_MEMORY
 *
 * Any implementation/evaluation resource budget belongs to compiler policy,
 * not syntax.
 *
 * ============================================================================
 */

parser grammar CompileTime;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. TOP-LEVEL COMPILATION-TIME CONTROL
 * ============================================================================
 *
 * The dispatcher used by the canonical parser should expose:
 *
 *     compileTimeControl
 *
 * as one of the valid compilation/control constructs.
 *
 * The actual source grammar remains composed with the repository's canonical
 * parser rather than replacing it.
 */

compileTimeControl
    : compileTimeConditional
    | compileTimeSelection
    | compileTimeRequirement
    | compileTimeAssertion
    | compileTimeSpecialization
    ;


/* ============================================================================
 * 2. CONDITIONAL COMPILATION
 * ============================================================================
 *
 * Conditional compilation selects source structure according to a condition
 * known to the compilation context.
 *
 * This is deliberately different from an ordinary runtime `if`.
 *
 * The condition itself is represented by the canonical expression grammar.
 *
 * The compiler/semantic layer decides whether the condition is legally
 * evaluable at compilation time.
 *
 * Syntax:
 *
 *     compile if <expression> {
 *         ...
 *     }
 *
 *     compile if <expression> {
 *         ...
 *     } else {
 *         ...
 *     }
 *
 * The exact lexical spelling of the compile-time introducer belongs to the
 * canonical lexer contract.
 *
 * This grammar therefore uses the stable token expected by the assembled
 * Zamani lexer rather than defining a lexer token locally.
 */

compileTimeConditional
    : compileTimeIfPrefix expression blockExpression
      compileTimeElseBranch?
    ;

compileTimeIfPrefix
    : COMPILE IF
    ;

compileTimeElseBranch
    : ELSE blockExpression
    ;


/* ============================================================================
 * 3. ELSE-IF CHAIN
 * ============================================================================
 *
 * Multiple compile-time branches remain unbounded by syntax.
 *
 * No fixed number of alternatives is permitted.
 */

compileTimeConditionalChain
    : compileTimeIfPrefix expression blockExpression
      compileTimeElseIfBranch*
      compileTimeElseBranch?
    ;

compileTimeElseIfBranch
    : ELSE IF expression blockExpression
    ;


/* ============================================================================
 * 4. COMPILE-TIME SELECTION
 * ============================================================================
 *
 * Compile-time selection permits the compiler to choose among source-level
 * alternatives without encoding a machine-specific implementation.
 *
 * Example conceptual form:
 *
 *     compile select expression {
 *         case ... => { ... }
 *         case ... => { ... }
 *         default => { ... }
 *     }
 *
 * The selected value/condition is interpreted by semantic analysis.
 */

compileTimeSelection
    : COMPILE SELECT expression
      LBRACE
      compileTimeSelectionArm+
      RBRACE
    ;

compileTimeSelectionArm
    : compileTimeSelectionPattern
      FAT_ARROW
      blockExpression
      COMMA?
    ;

compileTimeSelectionPattern
    : expression
    | DEFAULT
    ;


/* ============================================================================
 * 5. COMPILE-TIME REQUIREMENT
 * ============================================================================
 *
 * A compilation requirement expresses a semantic prerequisite.
 *
 * It does NOT select a concrete physical machine.
 *
 * Examples of semantic requirements include:
 *
 *     compile require expression;
 *
 * The expression is interpreted by semantic analysis against the compilation
 * context.
 *
 * It may refer to:
 *
 *     language features
 *     compiler capabilities
 *     target capabilities
 *     resource requirements
 *     dialect availability
 *     backend-independent properties
 *
 * It must not be interpreted by this grammar as a device-selection operation.
 */

compileTimeRequirement
    : COMPILE REQUIRE expression SEMI?
    ;


/* ============================================================================
 * 6. COMPILE-TIME ASSERTION
 * ============================================================================
 *
 * This is the statement/control-level compile-time assertion.
 *
 * It is intentionally distinct from:
 *
 *     grammar/expressions/compile-time.g4
 *
 * which owns expression-level compile-time assertion forms.
 *
 * Semantic diagnostics are generated downstream.
 */

compileTimeAssertion
    : COMPILE ASSERT LPAREN expression RPAREN SEMI?
    ;


/* ============================================================================
 * 7. COMPILE-TIME SPECIALIZATION
 * ============================================================================
 *
 * Requests that the compiler consider a specialization.
 *
 * The grammar does NOT specify:
 *
 *     - how specialization is implemented;
 *     - whether specialization happens;
 *     - which optimization algorithm is used;
 *     - which target is selected;
 *     - whether code generation occurs;
 *     - whether the result is cached.
 *
 * Those are compiler semantics.
 *
 * Syntax:
 *
 *     compile specialize(expression);
 */

compileTimeSpecialization
    : COMPILE SPECIALIZE LPAREN expression RPAREN SEMI?
    ;


/* ============================================================================
 * 8. COMPILE-TIME FEATURE SELECTION
 * ============================================================================
 *
 * Feature selection must operate on language/semantic capabilities rather
 * than hard-coded physical machines.
 *
 * Example:
 *
 *     compile feature <expression> { ... }
 *
 * The expression is evaluated by semantic analysis.
 */

compileTimeFeatureSelection
    : COMPILE FEATURE expression blockExpression
      compileTimeFeatureElse?
    ;

compileTimeFeatureElse
    : ELSE blockExpression
    ;


/* ============================================================================
 * 9. COMPILE-TIME CONFIGURATION SELECTION
 * ============================================================================
 *
 * Configuration is intentionally expressed as a semantic value rather than
 * a fixed machine table.
 *
 * This keeps source programs portable across different compilation contexts.
 */

compileTimeConfigurationSelection
    : COMPILE CONFIG expression blockExpression
      compileTimeConfigurationElse?
    ;

compileTimeConfigurationElse
    : ELSE blockExpression
    ;


/* ============================================================================
 * 10. COMPILE-TIME OPTION
 * ============================================================================
 *
 * Compilation options are source-level requests.
 *
 * They are not direct compiler implementation switches.
 *
 * The semantic/compiler layer may:
 *
 *     accept;
 *     reject;
 *     ignore;
 *     lower;
 *     transform;
 *     negotiate
 *
 * the requested option according to language and compilation policy.
 */

compileTimeOption
    : COMPILE OPTION identifier
      (ASSIGN expression)?
      SEMI?
    ;


/* ============================================================================
 * 11. COMPILE-TIME ATTRIBUTE
 * ============================================================================
 *
 * Attribute syntax remains compatible with the canonical attribute system.
 *
 * This rule provides an integration boundary rather than a second attribute
 * grammar.
 */

compileTimeAttribute
    : COMPILE ATTRIBUTE identifier
      (LPAREN argumentList? RPAREN)?
    ;


/* ============================================================================
 * 12. COMPILE-TIME INCLUDE
 * ============================================================================
 *
 * Compile-time source inclusion is intentionally semantic.
 *
 * The grammar does not grant the parser filesystem or network access.
 *
 * The compiler frontend/security layer decides whether an inclusion request
 * is permitted and how its source is resolved.
 *
 * This construct must therefore never imply arbitrary host filesystem access.
 */

compileTimeInclude
    : COMPILE INCLUDE STRING_LITERAL SEMI?
    ;


/* ============================================================================
 * 13. COMPILE-TIME GENERATED REGION
 * ============================================================================
 *
 * A generated region identifies a source region whose contents may be
 * produced or transformed by a compile-time mechanism.
 *
 * Generation itself belongs to metaprogramming/compiler infrastructure.
 */

compileTimeGeneratedRegion
    : COMPILE GENERATE blockExpression
    ;


/* ============================================================================
 * 14. COMPILE-TIME LOOP CONTROL
 * ============================================================================
 *
 * Compile-time iteration is represented as semantic iteration.
 *
 * There is deliberately no maximum iteration count.
 *
 * Resource limits, termination, evaluation budgets, and compiler safety
 * policies belong downstream.
 *
 * The body remains canonical Zamani block syntax.
 */

compileTimeFor
    : COMPILE FOR pattern IN expression blockExpression
    ;


/* ============================================================================
 * 15. COMPILE-TIME MATCH
 * ============================================================================
 *
 * Compile-time matching operates over a semantic expression.
 *
 * It does not imply target-specific dispatch.
 */

compileTimeMatch
    : COMPILE MATCH expression
      LBRACE
      compileTimeMatchArm+
      RBRACE
    ;

compileTimeMatchArm
    : pattern
      (IF expression)?
      FAT_ARROW
      blockExpression
      COMMA?
    ;


/* ============================================================================
 * 16. COMPILE-TIME BLOCK
 * ============================================================================
 *
 * A compile-time block is an integration boundary.
 *
 * It does not introduce a second statement grammar.
 */

compileTimeBlock
    : COMPILE blockExpression
    ;


/* ============================================================================
 * 17. COMPILE-TIME DECLARATION
 * ============================================================================
 *
 * This rule intentionally delegates declaration ownership to the ordinary
 * declaration grammar.
 *
 * Compile-time function declarations belong to:
 *
 *     grammar/functions/compile-time-functions.g4
 *
 * Ordinary declarations remain owned by their respective declaration
 * grammars.
 */

compileTimeDeclaration
    : COMPILE declaration
    ;


/* ============================================================================
 * 18. COMPILE-TIME EXPRESSION BRIDGE
 * ============================================================================
 *
 * Expression-level compile-time constructs belong to:
 *
 *     grammar/expressions/compile-time.g4
 *
 * This rule is only a composition boundary.
 *
 * It MUST NOT reproduce those expression productions here.
 */

compileTimeExpressionBridge
    : expression
    ;


/* ============================================================================
 * 19. COMPILE-TIME CONTROL DISPATCH
 * ============================================================================
 *
 * This rule is the principal integration point consumed by the canonical
 * parser.
 *
 * The canonical parser should expose this rule as one possible source
 * construct where compilation-control syntax is permitted.
 *
 * This rule deliberately does not consume ordinary runtime statements unless
 * the selected construct explicitly contains a canonical block.
 */

compileTimeControlForm
    : compileTimeConditionalChain
    | compileTimeSelection
    | compileTimeRequirement
    | compileTimeAssertion
    | compileTimeSpecialization
    | compileTimeFeatureSelection
    | compileTimeConfigurationSelection
    | compileTimeOption
    | compileTimeAttribute
    | compileTimeInclude
    | compileTimeGeneratedRegion
    | compileTimeFor
    | compileTimeMatch
    ;


/* ============================================================================
 * 20. SEMANTIC BOUNDARY MARKERS
 * ============================================================================
 *
 * These rules document the composition points without embedding semantics.
 *
 * They are intentionally thin.
 */

compileTimeCondition
    : expression
    ;

compileTimeValue
    : expression
    ;

compileTimeConfiguration
    : expression
    ;

compileTimeCapability
    : expression
    ;

compileTimeRequirementExpression
    : expression
    ;


/* ============================================================================
 * 21. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical ownership:
 *
 *     lexer/
 *         owns tokens
 *
 *     core/
 *         owns identifiers, paths, attributes and compilation-unit concepts
 *
 *     types/
 *         owns type syntax
 *
 *     expressions/
 *         owns expression syntax
 *
 *     expressions/compile-time.g4
 *         owns expression-level compile-time constructs
 *
 *     functions/
 *         owns function syntax
 *
 *     functions/compile-time-functions.g4
 *         owns compile-time-function syntax
 *
 *     compile/compile-time.g4
 *         owns compilation-control syntax
 *
 *     compile/target.g4
 *         owns target-selection syntax
 *
 *     compile/optimization.g4
 *         owns optimization-request syntax
 *
 *     resources/
 *         owns resource requirements/capabilities/constraints
 *
 *     execution/
 *         owns execution/deployment syntax
 *
 *     hardware/
 *         owns hardware descriptions
 *
 *     dialects/
 *         owns dialect extension syntax
 *
 * Downstream:
 *
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> compile-time evaluator
 *       +--> feature resolver
 *       +--> specialization
 *       +--> target-independent lowering
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware lowering
 *
 * No grammar -> runtime -> grammar cycle is permitted.
 */


/* ============================================================================
 * 22. HARDWARE-INDEPENDENCE CONTRACT
 * ============================================================================
 *
 * This file must never contain productions equivalent to:
 *
 *     compile for cpu0
 *     compile for gpu0
 *     compile for qpu0
 *     compile for 32 qubits
 *     compile for 64 cores
 *     compile for device X
 *     compile on topology Y
 *
 * A source program may express a semantic requirement through the resource,
 * capability, target, or hardware grammars.
 *
 * Physical realization remains downstream.
 */


/* ============================================================================
 * 23. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Semantic evaluation determinism is not a grammar responsibility.
 *
 * A compile-time construct may therefore be syntactically valid even when
 * semantic analysis later rejects it because it depends on prohibited or
 * nondeterministic external state.
 */


/* ============================================================================
 * 24. SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing these constructs MUST NOT:
 *
 *     - read files;
 *     - write files;
 *     - access networks;
 *     - execute programs;
 *     - inspect credentials;
 *     - inspect environment variables directly;
 *     - discover hardware;
 *     - contact quantum hardware;
 *     - contact cloud providers;
 *     - allocate devices;
 *     - deploy workloads.
 *
 * Those operations require explicit downstream capabilities and security
 * policy.
 */


/* ============================================================================
 * 25. RESOURCE-SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar places no syntactic upper bound on:
 *
 *     - number of compile-time branches;
 *     - number of feature requirements;
 *     - number of configuration alternatives;
 *     - number of generated constructs;
 *     - number of compile-time declarations;
 *     - number of compile-time iterations.
 *
 * Compiler implementation limits, where necessary, must be represented as
 * explicit implementation/resource policy rather than hidden grammar limits.
 */


/* ============================================================================
 * 26. POCO-REAF CONTRACT
 * ============================================================================
 *
 * Compilation-time constructs must preserve:
 *
 *     Program Once
 *          |
 *          v
 *     Portable semantic source
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Target-independent representation
 *          |
 *          v
 *     Run Everywhere / Anywhere
 *          |
 *          v
 *     Resource-aware realization
 *
 * The grammar must not make temporary hardware characteristics part of the
 * permanent source-level meaning of a program.
 */


/* ============================================================================
 * 27. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] Its ownership boundary is documented.
 * [ ] It does not duplicate expression compile-time grammar.
 * [ ] It does not duplicate compile-time function grammar.
 * [ ] It consumes the canonical lexer vocabulary.
 * [ ] It introduces no lexer rules.
 * [ ] It introduces no embedded Rust.
 * [ ] It contains no unsafe operations.
 * [ ] It contains no fixed machine/resource limits.
 * [ ] It contains no physical device assumptions.
 * [ ] It contains no quantum IR.
 * [ ] It contains no classical IR.
 * [ ] It contains no hardware IR.
 * [ ] It does not perform semantic evaluation.
 * [ ] It does not access external resources.
 * [ ] It composes with the canonical parser.
 * [ ] Its AST representation can distinguish every construct.
 * [ ] Semantic analysis can reject invalid compile-time operations.
 * [ ] Compilation-control constructs remain target-independent.
 * [ ] Positive parser tests exist.
 * [ ] Negative parser tests exist.
 * [ ] Boundary tests exist.
 * [ ] Determinism tests exist.
 * [ ] Cross-domain tests exist.
 * [ ] POCO-REAF scalability tests exist.
 */