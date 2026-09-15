/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Grammar:
 *     Metaprogramming
 *
 * Status:
 *     Production parser-composition unit
 *
 * Rust implementation target:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no target-language actions and requires no Rust
 *     `unsafe`. All compiler implementation code consuming this grammar MUST
 *     remain safe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the COMMON METAPROGRAMMING COMPOSITION BOUNDARY.
 *
 * It does not attempt to implement the macro engine, evaluator, reflection
 * engine, specialization engine, source generator, compiler, optimizer,
 * scheduler, hardware layer, runtime, quantum IR, QEC, ZQN, or simulator.
 *
 * The grammar is responsible only for recognizing metaprogramming constructs
 * and connecting them to the repository's canonical parser rules.
 *
 * The semantic pipeline remains:
 *
 *     Zamani source
 *          |
 *          v
 *       lexer
 *          |
 *          v
 *       parser
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> macro expansion
 *          +--> compile-time evaluation
 *          +--> reflection
 *          +--> specialization
 *          +--> generation
 *          |
 *          v
 *     canonical semantic representation / IR
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> other domain IRs
 *          |
 *          v
 *     optimization
 *          |
 *     routing / scheduling / lowering
 *          |
 *     hardware abstraction
 *          |
 *     runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the metaprogramming composition entry point;
 *   - the distinction between metaprogramming declaration and metaprogramming
 *     expression/statement contexts;
 *   - composition of macro syntax;
 *   - composition of compile-time syntax;
 *   - composition of generation syntax;
 *   - composition of reflection syntax;
 *   - composition of specialization syntax;
 *   - the shared dispatch boundary used by the canonical parser.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer tokens;
 *   - identifiers;
 *   - paths;
 *   - expressions;
 *   - statements;
 *   - declarations;
 *   - types;
 *   - patterns;
 *   - generic parameters;
 *   - ordinary functions;
 *   - macro expansion semantics;
 *   - macro hygiene;
 *   - compile-time evaluation;
 *   - reflection semantics;
 *   - specialization algorithms;
 *   - generated-code semantics;
 *   - resource discovery;
 *   - hardware discovery;
 *   - backend selection;
 *   - target selection;
 *   - classical IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - runtime dispatch.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file MUST NOT become a second complete parser.
 *
 * Shared language constructs MUST be imported/composed from their canonical
 * owners.
 *
 * In particular, this file MUST NOT define local replacements for:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     statement
 *     item
 *     declaration
 *     typeExpression
 *     pattern
 *     block
 *     genericParameters
 *     parameterList
 *     argumentList
 *     attribute
 *     visibilityModifier
 *
 * The canonical parser composition layer owns those rules.
 *
 * ============================================================================
 * SIBLING FILE OWNERSHIP
 * ============================================================================
 *
 *     macros/
 *         owns macro declaration/invocation syntax.
 *
 *     compile-time-execution.g4
 *         owns explicit compile-time execution syntax.
 *
 *     generation.g4
 *         owns source-generation syntax.
 *
 *     reflection.g4
 *         owns reflection-request syntax.
 *
 *     specialization.g4
 *         owns specialization-request syntax.
 *
 * This file composes those facilities.
 *
 * If one of those sibling files requires a new production, that production
 * belongs in that sibling file rather than being copied here.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Metaprogramming must preserve:
 *
 *     Program Once
 *          |
 *          v
 *     portable source semantics
 *          |
 *          v
 *     deterministic / explicitly-authorized transformation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     target adaptation
 *
 * Metaprogramming syntax MUST NOT require:
 *
 *     a fixed CPU count
 *     a fixed GPU count
 *     a fixed FPGA count
 *     a fixed QPU count
 *     a fixed qubit count
 *     a fixed memory size
 *     a fixed register count
 *     a fixed topology
 *     a fixed device ID
 *     a fixed vendor
 *     a fixed backend
 *     a fixed timing grid
 *     a fixed physical address
 *
 * Resource requirements and capabilities are expressed through the repository's
 * resource/capability/target systems and resolved downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No language-level maximum is encoded here.
 *
 * In particular, there is no:
 *
 *     MAX_MACROS
 *     MAX_PARAMETERS
 *     MAX_GENERATED_ITEMS
 *     MAX_SPECIALIZATIONS
 *     MAX_REFLECTION_DEPTH
 *     MAX_COMPILE_TIME_OPERATIONS
 *     MAX_TYPES
 *     MAX_EXPRESSIONS
 *     MAX_QUbits
 *     MAX_DEVICES
 *
 * Compiler resource budgets, recursion protection, cancellation, timeouts,
 * memory admission, expansion budgets, and execution limits are implementation
 * policy. They MUST NOT become grammar semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Whether a metaprogram executes deterministically is a semantic/compiler
 * property.
 *
 * The grammar MUST NOT implicitly grant access to:
 *
 *     filesystem
 *     network
 *     environment
 *     credentials
 *     subprocesses
 *     hardware
 *     devices
 *     clocks
 *     random sources
 *
 * A compiler may expose explicitly authorized capabilities through semantic
 * analysis, provenance, sandboxing, and policy.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a metaprogramming construct MUST NEVER execute it.
 *
 * Generated source MUST pass through normal:
 *
 *     parsing
 *     validation
 *     name resolution
 *     type checking
 *     capability checking
 *     effect checking
 *     semantic validation
 *
 * before entering the canonical compiler pipeline.
 *
 * A generated quantum construct must ultimately enter the same quantum semantic
 * pipeline as handwritten quantum source. It must not create a parallel
 * quantum representation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every metaprogramming construct must preserve:
 *
 *     source span
 *     source ordering
 *     syntactic identity
 *     nesting
 *     explicit meta intent
 *     attributes
 *     names/paths
 *     argument structure
 *
 * The ANTLR parse tree is not the canonical AST.
 *
 * The frontend owns conversion from this parser representation to the
 * repository's canonical AST.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * In particular:
 *
 *     grammar/metaprogramming/
 *
 * MUST NOT create:
 *
 *     QuantumGate
 *     Qubit
 *     PhysicalQubit
 *     ClassicalInstruction
 *     HardwareInstruction
 *     ScheduleOperation
 *     ZQN fault
 *     QEC operation
 *
 * Generated source is reintroduced into the normal semantic pipeline.
 *
 * Quantum generated source ultimately lowers through `quantum::ir`.
 *
 * ============================================================================
 * RUST SAFETY
 * ============================================================================
 *
 * This grammar has no embedded Rust actions.
 *
 * Rust 1.97 / 1.97.1 implementations consuming it MUST use safe Rust.
 *
 * The absence of `unsafe` in this grammar is intentional.
 *
 * ============================================================================
 */

parser grammar Metaprogramming;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. TOP-LEVEL METAPROGRAMMING DISPATCH
 * ============================================================================
 *
 * The canonical parser should invoke `metaprogrammingDeclaration`,
 * `metaprogrammingExpression`, or `metaprogrammingStatement` at the
 * appropriate syntactic locations.
 *
 * This file deliberately does not decide where those entry points are legal.
 * The canonical parser owns that integration.
 */

metaprogrammingDeclaration
    : macroDeclaration
    | compileTimeDeclaration
    | generationDeclaration
    | reflectionDeclaration
    | specializationDeclaration
    ;


/*
 * Expression-level metaprogramming is intentionally a dispatcher.
 *
 * Detailed syntax remains in its owning component.
 */
metaprogrammingExpression
    : macroExpression
    | compileTimeExpression
    | generationExpression
    | reflectionExpression
    | specializationExpression
    ;


/*
 * Statement-level metaprogramming is also a dispatcher.
 */
metaprogrammingStatement
    : macroStatement
    | compileTimeStatement
    | generationStatement
    | reflectionStatement
    | specializationStatement
    ;


/* ============================================================================
 * 2. MACRO COMPOSITION
 * ============================================================================
 *
 * Macro syntax belongs to the macro grammar.
 *
 * This file only establishes the integration point.
 */

macroDeclaration
    : macroDeclarationCore
    ;

macroExpression
    : macroInvocationCore
    ;

macroStatement
    : macroInvocationStatementCore
    ;


/*
 * These names are integration contracts for the canonical macro grammar.
 *
 * They prevent this file from copying macro syntax and thereby creating two
 * competing definitions.
 *
 * The implementation of those rules belongs to grammar/macros/.
 */


/* ============================================================================
 * 3. COMPILE-TIME EXECUTION COMPOSITION
 * ============================================================================
 *
 * Compile-time execution syntax belongs to:
 *
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * That component must expose these semantic syntax categories.
 */

compileTimeDeclaration
    : compileTimeDeclarationCore
    ;

compileTimeExpression
    : compileTimeExpressionCore
    ;

compileTimeStatement
    : compileTimeStatementCore
    ;


/* ============================================================================
 * 4. SOURCE GENERATION COMPOSITION
 * ============================================================================
 *
 * Generation syntax belongs to:
 *
 *     grammar/metaprogramming/generation.g4
 */

generationDeclaration
    : generationDeclarationCore
    ;

generationExpression
    : generationExpressionCore
    ;

generationStatement
    : generationStatementCore
    ;


/* ============================================================================
 * 5. REFLECTION COMPOSITION
 * ============================================================================
 *
 * Reflection syntax belongs to:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * Runtime reflection and compile-time reflection remain semantically distinct.
 * The grammar only preserves their explicit source form.
 */

reflectionDeclaration
    : reflectionDeclarationCore
    ;

reflectionExpression
    : reflectionExpressionCore
    ;

reflectionStatement
    : reflectionStatementCore
    ;


/* ============================================================================
 * 6. SPECIALIZATION COMPOSITION
 * ============================================================================
 *
 * Specialization syntax belongs to:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * Specialization is an optimization/compilation concern after semantic
 * validation. It must never change the source program's portable semantics.
 */

specializationDeclaration
    : specializationDeclarationCore
    ;

specializationExpression
    : specializationExpressionCore
    ;

specializationStatement
    : specializationStatementCore
    ;


/* ============================================================================
 * 7. CANONICAL METAPROGRAMMING VALUE
 * ============================================================================
 *
 * A metaprogram may manipulate source structure, values, types, patterns,
 * declarations, or metadata.
 *
 * This grammar does not invent a second type/value hierarchy.
 *
 * The canonical AST/semantic layer determines the actual meta-value category.
 *
 * The following rule exists only as a composition contract for components
 * needing a shared meta-value boundary.
 */

metaValue
    : expression
    | typeExpression
    | pattern
    | qualifiedName
    ;


/* ============================================================================
 * 8. SOURCE-STRUCTURE BOUNDARY
 * ============================================================================
 *
 * Generated syntax must be represented using canonical Zamani syntax.
 *
 * No embedded ANTLR grammar language is accepted here.
 *
 * A language-definition facility may describe a grammar as data, but that
 * description belongs to the language-definition/dialect subsystem.
 */

metaSource
    : expression
    | statement
    | item
    ;


/* ============================================================================
 * 9. QUOTED SOURCE BOUNDARY
 * ============================================================================
 *
 * Quotation/splicing syntax is owned by the macro/generation subsystem.
 *
 * These aliases exist only as explicit integration contracts.
 *
 * They intentionally do not define another quotation language.
 */

metaQuote
    : metaQuoteCore
    ;

metaSplice
    : metaSpliceCore
    ;


/* ============================================================================
 * 10. META IDENTIFIERS
 * ============================================================================
 *
 * Names remain ordinary Zamani names.
 *
 * This is critical for:
 *
 *     hygiene
 *     source maps
 *     diagnostics
 *     deterministic name resolution
 *     IDE tooling
 *     refactoring
 *
 * The grammar does not invent compiler-specific identifier syntax.
 */

metaName
    : identifier
    ;

metaPath
    : qualifiedName
    ;


/* ============================================================================
 * 11. META ATTRIBUTES
 * ============================================================================
 *
 * Attributes are owned by the canonical attribute grammar.
 *
 * Metaprogramming may consume them, but does not redefine their syntax.
 */

metaAttribute
    : attribute
    ;


/* ============================================================================
 * 12. META TYPE BOUNDARY
 * ============================================================================
 *
 * Types remain owned by the canonical type grammar.
 *
 * This allows metaprogramming to manipulate types without establishing a
 * second type system.
 */

metaType
    : typeExpression
    ;


/* ============================================================================
 * 13. META PATTERN BOUNDARY
 * ============================================================================
 *
 * Patterns remain owned by the canonical pattern grammar.
 */

metaPattern
    : pattern
    ;


/* ============================================================================
 * 14. META BLOCK BOUNDARY
 * ============================================================================
 *
 * Blocks remain canonical language blocks.
 *
 * There is no separate "meta block language".
 */

metaBlock
    : block
    ;


/* ============================================================================
 * 15. META EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Metaprogramming may consume ordinary expressions.
 *
 * Expression precedence and expression semantics remain owned by the
 * canonical expression grammar.
 */

metaExpression
    : expression
    ;


/* ============================================================================
 * 16. META STATEMENT BOUNDARY
 * ============================================================================
 *
 * Statements remain canonical statements.
 */

metaStatement
    : statement
    ;


/* ============================================================================
 * 17. META DECLARATION BOUNDARY
 * ============================================================================
 *
 * Generated declarations must be canonical Zamani declarations/items.
 */

metaDeclaration
    : item
    ;


/* ============================================================================
 * 18. META GENERIC BOUNDARY
 * ============================================================================
 *
 * Generic syntax remains owned by the canonical type/function/declaration
 * grammar.
 */

metaGenericParameters
    : genericParameters
    ;


/* ============================================================================
 * 19. META PARAMETER BOUNDARY
 * ============================================================================
 */

metaParameterList
    : parameterList
    ;


/* ============================================================================
 * 20. META ARGUMENT BOUNDARY
 * ============================================================================
 */

metaArgumentList
    : argumentList
    ;


/* ============================================================================
 * 21. PORTABLE RESOURCE BOUNDARY
 * ============================================================================
 *
 * Metaprogramming may produce or inspect resource requirements only through
 * the canonical resource/capability syntax.
 *
 * This grammar deliberately does NOT define:
 *
 *     qubit counts
 *     CPU counts
 *     GPU counts
 *     FPGA counts
 *     memory sizes
 *     device identifiers
 *     topology
 *     addresses
 *
 * Those are owned by grammar/resources and grammar/hardware where appropriate.
 */

metaResourceExpression
    : expression
    ;


/* ============================================================================
 * 22. TARGET-INDEPENDENT META INTENT
 * ============================================================================
 *
 * A metaprogram may produce target-independent source.
 *
 * Target-specific specialization must remain an explicit downstream semantic
 * operation and must not become an implicit parser-side machine dependency.
 */

metaTargetIndependentSource
    : metaSource
    ;


/* ============================================================================
 * 23. GENERATED-SOURCE REENTRY CONTRACT
 * ============================================================================
 *
 * Generated source MUST re-enter the normal parser/semantic pipeline.
 *
 * There is intentionally no direct:
 *
 *     meta -> IR
 *
 * production.
 *
 * The required architecture is:
 *
 *     meta syntax
 *         |
 *         v
 *     canonical AST
 *         |
 *         v
 *     semantic validation
 *         |
 *         v
 *     generated canonical AST
 *         |
 *         v
 *     semantic IR
 *
 * This preserves the same semantic guarantees for generated and handwritten
 * source.
 */

generatedSource
    : metaSource
    ;


/* ============================================================================
 * 24. CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Generated source may describe any domain supported by Zamani:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     future dialects
 *
 * This grammar does not introduce domain-specific duplicates.
 *
 * For example, generated quantum source must use the normal quantum grammar
 * and ultimately lower through quantum::ir.
 */

crossDomainGeneratedSource
    : generatedSource
    ;


/* ============================================================================
 * 25. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file deliberately contains no:
 *
 *     qubit
 *     physical qubit
 *     logical qubit
 *     quantum gate
 *     circuit
 *     QEC
 *     ZQN
 *
 * productions.
 *
 * A metaprogram may generate quantum source, but the generated source must
 * enter the ordinary quantum frontend.
 *
 * Consequently:
 *
 *     metaprogramming
 *          |
 *          v
 *     generated Zamani quantum syntax
 *          |
 *          v
 *     quantum frontend
 *          |
 *          v
 *     quantum::ir
 *
 * No alternate quantum representation is permitted.
 */


/* ============================================================================
 * 26. HARDWARE / HDL INTEGRATION CONTRACT
 * ============================================================================
 *
 * A metaprogram may generate HDL or hardware-oriented Zamani source.
 *
 * This grammar does not define:
 *
 *     ports
 *     wires
 *     clocks
 *     physical resources
 *     topology
 *     device IDs
 *
 * Those remain owned by their respective grammar components.
 */


/* ============================================================================
 * 27. DISTRIBUTED INTEGRATION CONTRACT
 * ============================================================================
 *
 * A metaprogram may generate distributed source.
 *
 * Node count, deployment topology, placement, network resources, and runtime
 * capacity are never encoded as grammar-level finite limits here.
 */


/* ============================================================================
 * 28. AI / DATA INTEGRATION CONTRACT
 * ============================================================================
 *
 * Generated AI/data source uses canonical:
 *
 *     types
 *     expressions
 *     functions
 *     tensors
 *     data
 *     resource
 *     accelerator
 *
 * grammars.
 *
 * No fixed tensor/resource/device maximum belongs here.
 */


/* ============================================================================
 * 29. DIALECT INTEGRATION CONTRACT
 * ============================================================================
 *
 * Dialects extend Zamani through the dialect subsystem.
 *
 * Metaprogramming MUST NOT silently create a dialect merely by generating
 * source text.
 *
 * Dialect registration, versioning, capability declarations, compatibility,
 * and vendor/experimental policy remain owned by grammar/dialects.
 */


/* ============================================================================
 * 30. SECURITY INTEGRATION CONTRACT
 * ============================================================================
 *
 * Meta operations that require effects are validated against the canonical
 * effect/capability/security model.
 *
 * Syntax alone MUST NOT imply permission.
 *
 * In particular:
 *
 *     quote
 *     generation
 *     reflection
 *     specialization
 *     compile-time execution
 *
 * do not imply filesystem/network/process/device privileges.
 */


/* ============================================================================
 * 31. RESOURCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * The following distinction is mandatory:
 *
 *     semantic requirement
 *     capability requirement
 *     hard constraint
 *     soft preference
 *     implementation hint
 *
 * Metaprogramming MUST preserve those distinctions.
 *
 * A generated resource request must remain portable unless the program
 * explicitly declares a semantically meaningful target constraint.
 */


/* ============================================================================
 * 32. COMPILER INTEGRATION CONTRACT
 * ============================================================================
 *
 * Compiler responsibilities after parsing:
 *
 *     1. Build canonical AST.
 *     2. Resolve names.
 *     3. Resolve macro/meta constructs.
 *     4. Validate permissions/effects.
 *     5. Evaluate authorized compile-time computation.
 *     6. Expand/generate source structures.
 *     7. Revalidate generated structures.
 *     8. Type-check generated structures.
 *     9. Lower to canonical semantic IR.
 *    10. Continue through normal optimization/lowering.
 *
 * No compiler phase may assume that parser acceptance means semantic validity.
 */


/* ============================================================================
 * 33. RUNTIME INTEGRATION CONTRACT
 * ============================================================================
 *
 * Runtime does not depend directly on this grammar.
 *
 * Runtime receives compiled semantic artifacts.
 *
 * This is essential to POCO-REAF:
 *
 *     source syntax
 *         !=
 *     runtime hardware configuration
 *
 * The grammar therefore has no runtime hardware dependency.
 */


/* ============================================================================
 * 34. TOOLING CONTRACT
 * ============================================================================
 *
 * IDEs, formatters, language servers, diagnostics, source maps, and
 * documentation generators must be able to distinguish metaprogramming
 * constructs by parse-tree context.
 *
 * They must not need to execute metaprograms merely to parse or format them.
 *
 * Source locations must survive:
 *
 *     macro invocation
 *     expansion
 *     generation
 *     specialization
 *     reflection
 *
 * through compiler-generated provenance metadata.
 */


/* ============================================================================
 * 35. ERROR-DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors are owned by the parser/lexer.
 *
 * The semantic compiler owns:
 *
 *     unknown macro
 *     invalid macro context
 *     forbidden compile-time effect
 *     unauthorized reflection
 *     invalid specialization
 *     generated-code type failure
 *     generated-code capability failure
 *     expansion-cycle failure
 *     resource-policy violation
 *
 * The grammar MUST NOT encode these semantic errors as parser hacks.
 */


/* ============================================================================
 * 36. SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar intentionally contains no finite:
 *
 *     arity limits
 *     nesting limits
 *     generated-item limits
 *     specialization limits
 *     reflection limits
 *     macro-count limits
 *     domain-count limits
 *     resource-count limits
 *
 * The parser implementation may of course be constrained by available memory,
 * input size, stack strategy, cancellation, and compiler policy.
 *
 * Such operational limits are not language semantics.
 */


/* ============================================================================
 * 37. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical source bytes, lexer configuration, grammar version, and
 * parser configuration, parsing must produce the same syntactic structure.
 *
 * Any nondeterminism introduced by:
 *
 *     macro evaluation
 *     reflection
 *     external resources
 *     compilation environment
 *
 * must be handled after parsing by explicit semantic/effect/provenance policy.
 */


/* ============================================================================
 * 38. VERSIONING CONTRACT
 * ============================================================================
 *
 * This grammar must evolve through the language-version and compatibility
 * mechanisms rather than silently changing the meaning of an existing
 * production.
 *
 * New metaprogramming facilities should normally be introduced by:
 *
 *     sibling grammar component
 *         ->
 *     explicit composition rule
 *         ->
 *     semantic implementation
 *         ->
 *     compatibility tests
 *
 * rather than by adding unrelated behavior here.
 */


/* ============================================================================
 * 39. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   [ ] It contains no duplicate canonical language rules.
 *   [ ] It contains no machine-size constants.
 *   [ ] It contains no hardware assumptions.
 *   [ ] It contains no quantum-machine assumptions.
 *   [ ] It contains no Rust actions.
 *   [ ] It contains no unsafe implementation requirement.
 *   [ ] It contains no compile-time execution implementation.
 *   [ ] It contains no macro-expansion implementation.
 *   [ ] It contains no reflection implementation.
 *   [ ] It contains no specialization algorithm.
 *   [ ] It composes all metaprogramming sibling grammars.
 *   [ ] All referenced shared rules have exactly one canonical owner.
 *   [ ] Generated source re-enters the normal semantic pipeline.
 *   [ ] Quantum output ultimately uses quantum::ir.
 *   [ ] Resource constraints remain separate from syntax.
 *   [ ] Parser behavior remains deterministic.
 *   [ ] Negative tests exist for invalid composition.
 *   [ ] Cross-domain tests exist.
 *   [ ] POCO-REAF scalability tests exist.
 *
 * ============================================================================
 */