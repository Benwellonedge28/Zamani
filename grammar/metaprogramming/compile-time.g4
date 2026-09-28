/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/compile-time.g4
 *
 * Grammar:
 *     CompileTime
 *
 * Status:
 *     Production parser-composition unit
 *
 * Purpose:
 *     Provide the authoritative compile-time metaprogramming composition
 *     boundary required by grammar/metaprogramming/metaprogramming.g4.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                        canonical lexer
 *                              |
 *                              v
 *                      canonical Zamani parser
 *                              |
 *                              v
 *                         canonical AST
 *                              |
 *                    +---------+---------+
 *                    |                   |
 *                    v                   v
 *             ordinary semantics   metaprogramming
 *                                        |
 *                    +-------------------+-------------------+
 *                    |                   |                   |
 *                    v                   v                   v
 *               compile-time        generation          reflection
 *                    |                   |                   |
 *                    +-------------------+-------------------+
 *                                        |
 *                                        v
 *                                  specialization
 *                                        |
 *                                        v
 *                              canonical semantic model
 *                                        |
 *                 +----------------------+----------------------+
 *                 |                      |                      |
 *                 v                      v                      v
 *             classical             quantum::ir          HDL/hardware
 *                 |                      |                      |
 *                 +----------------------+----------------------+
 *                                        |
 *                                        v
 *                              optimization / lowering
 *                                        |
 *                              routing / scheduling
 *                                        |
 *                                  resilience / QEC
 *                                        |
 *                                       ZQN
 *                                        |
 *                                       HAL
 *                                        |
 *                                 target realization
 *
 * This grammar defines syntax/composition only.
 *
 * It MUST NOT:
 *
 *   - execute compile-time code;
 *   - evaluate expressions;
 *   - construct AST objects;
 *   - construct IR;
 *   - construct quantum::ir;
 *   - perform optimization;
 *   - perform specialization algorithms;
 *   - perform target selection;
 *   - discover hardware;
 *   - inspect the compiler host;
 *   - access the filesystem;
 *   - access the network;
 *   - access credentials;
 *   - invoke processes;
 *   - invoke GPUs;
 *   - invoke FPGAs;
 *   - invoke QPUs;
 *   - perform routing;
 *   - perform scheduling;
 *   - perform QEC;
 *   - implement ZQN;
 *   - implement runtime behavior.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * The repository already contains:
 *
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * which owns explicit compile-time execution syntax.
 *
 * The repository also contains:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * which expects the following integration contracts:
 *
 *     compileTimeDeclarationCore
 *     compileTimeExpressionCore
 *     compileTimeStatementCore
 *
 * Those contracts previously had no single authoritative owner.
 *
 * This file closes that gap.
 *
 * It deliberately does NOT duplicate the compile-time execution grammar.
 *
 * Instead:
 *
 *     compile-time.g4
 *          |
 *          +--> compile-time-execution.g4
 *          |
 *          +--> canonical expression boundary
 *          |
 *          +--> canonical declaration/item boundary
 *          |
 *          +--> canonical statement boundary
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser composition authority:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Metaprogramming composition authority:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Compile-time execution syntax:
 *
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * Generation syntax:
 *
 *     grammar/metaprogramming/generation.g4
 *
 * Reflection syntax:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * Specialization syntax:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * Canonical AST:
 *
 *     frontend AST subsystem
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the compile-time metaprogramming composition boundary;
 *   - compileTimeDeclarationCore;
 *   - compileTimeExpressionCore;
 *   - compileTimeStatementCore;
 *   - explicit integration of compile-time execution with the broader
 *     metaprogramming subsystem;
 *   - phase-boundary naming for compile-time syntax;
 *   - compile-time source-category dispatch.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - keywords;
 *   - identifiers;
 *   - paths;
 *   - ordinary expressions;
 *   - expression precedence;
 *   - ordinary statements;
 *   - declarations;
 *   - types;
 *   - patterns;
 *   - functions;
 *   - macros;
 *   - macro hygiene;
 *   - reflection;
 *   - source generation;
 *   - specialization algorithms;
 *   - compilation algorithms;
 *   - resource allocation;
 *   - hardware discovery;
 *   - target selection;
 *   - AST implementation;
 *   - IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - runtime execution.
 *
 * ============================================================================
 * CRITICAL TOKEN POLICY
 * ============================================================================
 *
 * This file intentionally uses NO private lexer vocabulary.
 *
 * In particular it does NOT introduce:
 *
 *     COMPTIME
 *     COMPILE_TIME
 *     CTIME
 *     META_COMPTIME
 *     GENERATE
 *
 * as local tokens.
 *
 * The repository's existing compile-time execution contract explicitly uses
 * the canonical CONST vocabulary.
 *
 * Generation already uses the canonical SYNTHESIZE token.
 *
 * Any future keyword must first be introduced through the canonical lexer and
 * language specification. It must never be invented inside this grammar.
 *
 * ============================================================================
 * COMPILE-TIME PHASE MODEL
 * ============================================================================
 *
 * Zamani distinguishes:
 *
 *     source parsing
 *          |
 *          v
 *     AST construction
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     authorized compile-time computation
 *          |
 *          v
 *     generated/transformed source
 *          |
 *          v
 *     ordinary semantic validation again
 *          |
 *          v
 *     canonical semantic model
 *
 * The grammar does not execute anything at any stage.
 *
 * ============================================================================
 * PHASE SAFETY
 * ============================================================================
 *
 * A compile-time construct MUST NOT automatically acquire capabilities merely
 * because it is syntactically located in a compile-time context.
 *
 * In particular:
 *
 *     compile-time syntax
 *
 * does NOT automatically grant:
 *
 *     filesystem access
 *     network access
 *     environment access
 *     credential access
 *     process execution
 *     hardware access
 *     device discovery
 *     QPU access
 *     GPU access
 *     FPGA access
 *     random access
 *     wall-clock access
 *
 * Capability and effect authorization remains a semantic/compiler concern.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Compile-time computation MUST preserve the distinction between:
 *
 *     program meaning
 *
 * and:
 *
 *     properties of the machine compiling the program.
 *
 * Compile-time syntax MUST NOT encode universal assumptions about:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     QPU count
 *     qubit capacity
 *     memory capacity
 *     storage capacity
 *     register width
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     network size
 *     node count
 *     topology
 *     device identifiers
 *
 * A compile-time program may compute values representing such quantities when
 * those quantities are explicit program semantics or explicitly authorized
 * resource/capability information.
 *
 * It must not silently convert an available host capability into a permanent
 * source requirement.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no finite language-level limit on:
 *
 *     compile-time expressions
 *     compile-time declarations
 *     compile-time statements
 *     nested compile-time constructs
 *     generated structures
 *     compile-time sequence length
 *     source size
 *     type size
 *     expression size
 *     declaration count
 *     specialization count
 *     generated item count
 *
 * Repetition is therefore represented with ANTLR repetition operators rather
 * than artificial finite alternatives.
 *
 * Compiler resource budgets remain implementation policy.
 *
 * A compiler may enforce:
 *
 *     memory budgets
 *     CPU budgets
 *     cancellation
 *     evaluation budgets
 *     recursion protection
 *     generated-output budgets
 *
 * without changing the language grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Compile-time execution determinism is semantic.
 *
 * The compiler must determine whether a compile-time computation is:
 *
 *     pure
 *     deterministic
 *     reproducible
 *     effectful
 *     externally dependent
 *     capability dependent
 *
 * The grammar does not make those determinations.
 *
 * ============================================================================
 * GENERATED-SOURCE SAFETY
 * ============================================================================
 *
 * If compile-time computation produces source structure, that structure MUST
 * return to the ordinary Zamani pipeline.
 *
 * Required conceptual path:
 *
 *     compile-time computation
 *              |
 *              v
 *        generated source
 *              |
 *              v
 *            lexer
 *              |
 *              v
 *            parser
 *              |
 *              v
 *          canonical AST
 *              |
 *              v
 *       semantic validation
 *              |
 *              v
 *      canonical semantic model
 *
 * Generated source MUST NOT bypass:
 *
 *     type checking
 *     name resolution
 *     effect checking
 *     capability checking
 *     resource checking
 *     ownership checking
 *     security checking
 *     provenance checking
 *     domain validation
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Compile-time metaprogramming may create, transform, or inspect source that
 * eventually represents quantum computation.
 *
 * It MUST NOT create a second quantum IR.
 *
 * The required path remains:
 *
 *     compile-time source transformation
 *              |
 *              v
 *        canonical Zamani AST
 *              |
 *              v
 *       quantum semantic analysis
 *              |
 *              v
 *          quantum::ir
 *              |
 *              v
 *       optimization
 *              |
 *              v
 *           routing
 *              |
 *              v
 *         scheduling
 *              |
 *              v
 *         QEC / resilience
 *              |
 *              v
 *             ZQN
 *              |
 *              v
 *             HAL
 *
 * This grammar MUST NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *
 * or any other closed gate universe.
 *
 * Quantum operations remain data-driven and semantically extensible.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Compile-time constructs may manipulate canonical:
 *
 *     values
 *     expressions
 *     types
 *     functions
 *     declarations
 *     generic structures
 *     arrays
 *     vectors
 *     matrices
 *     tensors
 *
 * They do not create a second classical language or mathematical type system.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Compile-time computation may parameterize HDL and hardware source.
 *
 * Examples of valid semantic intent include:
 *
 *     generated width
 *     generated pipeline structure
 *     generated module structure
 *     generated interfaces
 *     generated state machines
 *
 * Actual:
 *
 *     FPGA selection
 *     ASIC selection
 *     physical placement
 *     timing closure
 *     routing
 *     device assignment
 *
 * remains downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Compile-time code may consume resource/capability information only through
 * the canonical semantic/resource systems.
 *
 * The following concepts remain distinct:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     implementation decision
 *
 * This grammar does not redefine those categories.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Each exposed rule must preserve enough parse information for the frontend
 * to construct the canonical AST.
 *
 * Required source information includes:
 *
 *     source span
 *     source ordering
 *     nesting
 *     syntactic category
 *     explicit compile-time intent
 *     embedded expression structure
 *     embedded statement structure
 *     embedded declaration structure
 *
 * The ANTLR parse tree is not the canonical AST.
 *
 * No independent MetaAST is created by this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     phase legality
 *     name resolution
 *     type checking
 *     effect checking
 *     capability checking
 *     resource checking
 *     compile-time evaluability
 *     determinism
 *     provenance
 *     security policy
 *     recursion/expansion policy
 *     generated-source validation
 *     compatibility
 *
 * Syntax alone does not establish any of those properties.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates ZERO IR.
 *
 * It must never directly produce:
 *
 *     ClassicalInstruction
 *     QuantumGate
 *     Qubit
 *     PhysicalQubit
 *     HDLInstruction
 *     HardwareInstruction
 *     ScheduleOperation
 *     RoutingOperation
 *     QECOperation
 *     ZQNFault
 *
 * Compile-time computation is a source/semantic transformation facility.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Implementation baseline:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Required:
 *
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no semantic predicates
 *     no embedded executable code
 *     no host-language callbacks
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not rename:
 *
 *     compile-time-execution.g4
 *     metaprogramming.g4
 *     generation.g4
 *     reflection.g4
 *     specialization.g4
 *
 * Existing compile-time execution syntax remains owned by
 * compile-time-execution.g4.
 *
 * This file supplies the missing composition contracts required by
 * metaprogramming.g4.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests must include:
 *
 *     const { ... }
 *     const expression
 *     const name = expression
 *     synthesize { ... }
 *     compile-time declarations
 *     compile-time statements
 *     compile-time expressions
 *     nested compile-time structures
 *     generated classical structures
 *     generated quantum structures
 *     generated HDL structures
 *
 * Negative tests must include:
 *
 *     COMPTIME
 *     COMPILE_TIME
 *     unknown compile-time keyword
 *     malformed compile-time block
 *     malformed compile-time binding
 *     malformed compile-time declaration
 *     malformed compile-time expression
 *     malformed compile-time statement
 *
 * The `COMPTIME` negative case is intentional because the current repository
 * contract explicitly does not define that token.
 *
 * Boundary tests must include:
 *
 *     minimal compile-time expression
 *     minimal compile-time block
 *     empty/invalid blocks where prohibited
 *     deeply nested compile-time constructs
 *     long compile-time sequences
 *     long generated structures
 *     deeply qualified names
 *     large generic structures
 *
 * Scalability tests must verify that no finite grammar maximum exists.
 *
 * Determinism tests must verify identical source produces identical parse
 * structure under identical grammar/lexer configuration.
 *
 * Compatibility tests must verify this grammar composes with:
 *
 *     metaprogramming.g4
 *     compile-time-execution.g4
 *     generation.g4
 *     reflection.g4
 *     specialization.g4
 *     canonical ZamaniParser
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No language-level resource ceiling may be introduced here.
 *
 * The following are explicitly prohibited as grammar limits:
 *
 *     MAX_COMPTIME_OPERATIONS
 *     MAX_COMPTIME_DEPTH
 *     MAX_GENERATED_ITEMS
 *     MAX_GENERATED_TYPES
 *     MAX_GENERATED_EXPRESSIONS
 *     MAX_SPECIALIZATIONS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Numeric literals remain ordinary program data.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] it has exactly one ownership responsibility;
 *     [x] it does not duplicate compile-time execution syntax;
 *     [x] it uses canonical lexer vocabulary;
 *     [x] it defines compileTimeDeclarationCore;
 *     [x] it defines compileTimeExpressionCore;
 *     [x] it defines compileTimeStatementCore;
 *     [x] it composes with compile-time-execution.g4;
 *     [x] it preserves canonical expression/declaration/statement ownership;
 *     [x] it creates no AST implementation;
 *     [x] it creates no IR;
 *     [x] it creates no quantum IR;
 *     [x] it creates no hardware model;
 *     [x] it creates no resource limits;
 *     [x] it requires no unsafe Rust;
 *     [x] it defines positive/negative/boundary/scalability contracts;
 *     [x] it documents integration with the canonical parser.
 *
 * ============================================================================
 */

parser grammar CompileTime;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. COMPILE-TIME DECLARATION CORE
 * ============================================================================
 *
 * This is the rule consumed by:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * It is deliberately a dispatcher.
 *
 * The detailed execution syntax remains owned by:
 *
 *     compile-time-execution.g4
 *
 * No compile-time declaration grammar is duplicated here.
 * ============================================================================
 */

compileTimeDeclarationCore
    : compileTimeExecution
    ;


/* ============================================================================
 * 2. COMPILE-TIME EXPRESSION CORE
 * ============================================================================
 *
 * The current repository deliberately does not use a private COMPTIME token.
 *
 * The canonical compile-time vocabulary is CONST.
 *
 * This rule therefore establishes the expression-level compile-time boundary
 * as:
 *
 *     const expression
 *
 * The embedded expression remains the canonical Zamani expression.
 *
 * This does NOT execute the expression.
 *
 * Semantic analysis decides whether the expression is legal and evaluable
 * during compilation.
 * ============================================================================
 */

compileTimeExpressionCore
    : CONST expression
    ;


/* ============================================================================
 * 3. COMPILE-TIME STATEMENT CORE
 * ============================================================================
 *
 * Explicit compile-time execution already owns its execution form in:
 *
 *     compile-time-execution.g4
 *
 * This adapter makes that form available through the composition contract
 * expected by metaprogramming.g4.
 * ============================================================================
 */

compileTimeStatementCore
    : compileTimeExecution
    ;


/* ============================================================================
 * 4. EXPLICIT COMPILE-TIME VALUE
 * ============================================================================
 *
 * Named integration boundary for tooling.
 *
 * It does not introduce a new value grammar.
 * ============================================================================
 */

compileTimeValue
    : CONST expression
    ;


/* ============================================================================
 * 5. COMPILE-TIME SOURCE TRANSFORMATION BOUNDARY
 * ============================================================================
 *
 * Source generation remains owned by generation.g4.
 *
 * This rule is intentionally a semantic bridge rather than a duplicate
 * generation grammar.
 *
 * The canonical generation entry point is `generation`.
 * ============================================================================
 */

compileTimeTransformation
    : generation
    ;


/* ============================================================================
 * 6. COMPILE-TIME EXECUTION BOUNDARY
 * ============================================================================
 *
 * Named integration point for compiler tooling.
 * ============================================================================
 */

compileTimeExecutionBoundary
    : compileTimeExecution
    ;


/* ============================================================================
 * 7. COMPILE-TIME GENERATED DECLARATION BOUNDARY
 * ============================================================================
 *
 * The detailed generated-declaration syntax is already owned by
 * compile-time-execution.g4.
 * ============================================================================
 */

compileTimeGeneratedDeclarationBoundary
    : compileTimeGeneratedDeclaration
    ;


/* ============================================================================
 * 8. COMPILE-TIME BLOCK BOUNDARY
 * ============================================================================
 *
 * The block syntax remains canonical.
 *
 * This rule deliberately does not define a second block grammar.
 * ============================================================================
 */

compileTimeBlock
    : CONST blockExpression
    ;


/* ============================================================================
 * 9. COMPILE-TIME SEQUENCE BOUNDARY
 * ============================================================================
 *
 * Sequence syntax is owned by compile-time-execution.g4.
 *
 * No finite sequence length is encoded.
 * ============================================================================
 */

compileTimeSequence
    : CONST compileTimeExecutionSequence
    ;


/* ============================================================================
 * 10. COMPILE-TIME NAME BOUNDARY
 * ============================================================================
 *
 * Names remain ordinary Zamani names.
 * ============================================================================
 */

compileTimeName
    : identifier
    ;


/* ============================================================================
 * 11. COMPILE-TIME TYPE BOUNDARY
 * ============================================================================
 *
 * Types remain canonical.
 * ============================================================================
 */

compileTimeType
    : typeExpression
    ;


/* ============================================================================
 * 12. COMPILE-TIME PATTERN BOUNDARY
 * ============================================================================
 *
 * Patterns remain canonical.
 * ============================================================================
 */

compileTimePattern
    : pattern
    ;


/* ============================================================================
 * 13. COMPILE-TIME EXPRESSION BOUNDARY
 * ============================================================================
 *
 * The actual expression language remains owned by expressions/.
 * ============================================================================
 */

compileTimeOperand
    : expression
    ;


/* ============================================================================
 * 14. COMPILE-TIME ITEM BOUNDARY
 * ============================================================================
 *
 * Generated/inspected declarations use the canonical item grammar.
 * ============================================================================
 */

compileTimeItem
    : item
    ;


/* ============================================================================
 * 15. COMPILE-TIME STATEMENT BOUNDARY
 * ============================================================================
 *
 * Statements remain canonical Zamani statements.
 * ============================================================================
 */

compileTimeStatement
    : statement
    ;


/* ============================================================================
 * 16. COMPILE-TIME ATTRIBUTE BOUNDARY
 * ============================================================================
 *
 * Attributes remain owned by the canonical attribute grammar.
 * ============================================================================
 */

compileTimeAttribute
    : attribute
    ;


/* ============================================================================
 * 17. COMPILE-TIME GENERIC BOUNDARY
 * ============================================================================
 *
 * Generic parameters remain canonical.
 * ============================================================================
 */

compileTimeGenericParameters
    : genericParameters
    ;


/* ============================================================================
 * 18. COMPILE-TIME PARAMETER BOUNDARY
 * ============================================================================
 *
 * Parameter syntax remains canonical.
 * ============================================================================
 */

compileTimeParameterList
    : parameterList
    ;


/* ============================================================================
 * 19. COMPILE-TIME ARGUMENT BOUNDARY
 * ============================================================================
 *
 * Argument syntax remains canonical.
 * ============================================================================
 */

compileTimeArgumentList
    : argumentList
    ;


/* ============================================================================
 * 20. COMPILE-TIME CAPABILITY/RESOURCE BOUNDARY
 * ============================================================================
 *
 * Capability and resource semantics belong to their canonical domains.
 *
 * This grammar intentionally does not define their syntax.
 *
 * Semantic analysis determines whether a compile-time computation may use
 * particular capabilities.
 * ============================================================================
 */

compileTimeResourceBoundary
    : expression
    ;


/* ============================================================================
 * 21. COMPILE-TIME PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Provenance is represented downstream through source spans, AST metadata,
 * semantic provenance, and compiler artifacts.
 *
 * This grammar does not invent a private provenance syntax.
 * ============================================================================
 */

compileTimeProvenanceBoundary
    : attribute
    ;


/* ============================================================================
 * 22. COMPILE-TIME DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * Compile-time computation may produce source for any supported Zamani domain.
 *
 * This grammar deliberately does not enumerate:
 *
 *     classical
 *     quantum
 *     HDL
 *     AI
 *     distributed
 *     networking
 *     accelerator
 *
 * because the generated/consumed structure remains ordinary Zamani syntax.
 *
 * Domain ownership stays with the corresponding domain grammar.
 * ============================================================================
 */


/* ============================================================================
 * 23. NO HARDWARE REALIZATION
 * ============================================================================
 *
 * There are deliberately no rules such as:
 *
 *     compileOnGpu
 *     compileOnQpu
 *     compileOnFpga
 *     compileOnCpu
 *     useDevice
 *     physicalQubit
 *
 * Hardware realization is downstream.
 * ============================================================================
 */


/* ============================================================================
 * 24. NO FIXED CAPACITY
 * ============================================================================
 *
 * The grammar intentionally uses canonical expressions and recursive/
 * unbounded ANTLR structures.
 *
 * It does not encode machine capacities.
 * ============================================================================
 */


/* ============================================================================
 * 25. INTEGRATION SUMMARY
 * ============================================================================
 *
 * Required composition:
 *
 *     ZamaniParser
 *          |
 *          v
 *     Metaprogramming
 *          |
 *          v
 *     CompileTime
 *          |
 *          +--> CompileTimeExecution
 *          |
 *          +--> canonical expression
 *          +--> canonical declaration/item
 *          +--> canonical statement
 *          +--> canonical type
 *          +--> canonical pattern
 *
 * The compile-time grammar is therefore a composition boundary rather than
 * a second compile-time language.
 *
 * ============================================================================
 */