/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/statements.g4
 *
 * Status:
 *     Canonical production statement-composition grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar fragment
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No filesystem access.
 *     No networking.
 *     No device discovery.
 *     No runtime execution.
 *     No hardware inspection.
 *     No mutable global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the canonical composition of Zamani statements.
 *
 * It answers:
 *
 *     "What syntactic forms are statements?"
 *
 * It does NOT answer:
 *
 *     "Is this statement semantically valid?"
 *
 * "statement" is intentionally a composition rule. Individual statement
 * families should be owned by their dedicated grammar components when those
 * components exist.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
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
 *     statement composition       <-- THIS FILE
 *       |
 *       +--> declarations
 *       +--> control flow
 *       +--> blocks
 *       +--> bindings
 *       +--> functions
 *       +--> effects
 *       +--> concurrency
 *       +--> quantum
 *       +--> HDL / hardware
 *       +--> distributed execution
 *       +--> domain extensions
 *       +--> expression statements
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       +--> ownership / borrowing
 *       +--> control-flow analysis
 *       |
 *       v
 *     canonical semantic representations
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware IR
 *       +--> control/data-flow representations
 *       +--> resource representation
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / lowering
 *       |
 *       v
 *     target realization
 *       |
 *       v
 *     runtime / hardware
 *
 * This file MUST NOT directly depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     hardware discovery
 *     calibration
 *     runtime
 *     backend selection
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - statement
 *     - the canonical statement-dispatch contract
 *     - statement composition ordering
 *     - the relationship between statement families
 *     - empty statements
 *     - expression-statement admission
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens
 *     - identifiers
 *     - literals
 *     - expression precedence
 *     - type syntax
 *     - block delimiters
 *     - declaration internals
 *     - function internals
 *     - loop internals
 *     - match internals
 *     - quantum semantics
 *     - HDL semantics
 *     - hardware topology
 *     - resource availability
 *     - scheduling
 *     - optimization
 *     - runtime behavior
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is deliberately a dispatcher.
 *
 * Dedicated grammar components own detailed productions.
 *
 * Conceptually:
 *
 *     statement
 *         |
 *         +--> bindingStatement
 *         +--> declarationStatement
 *         +--> controlFlowStatement
 *         +--> blockExpression
 *         +--> effectStatement
 *         +--> concurrencyStatement
 *         +--> domainStatement
 *         +--> expressionStatement
 *         +--> emptyStatement
 *
 * The exact component names are resolved by the repository's modular grammar
 * composition layer.
 *
 * No statement implementation should be copied into this file merely to make
 * the dispatcher self-contained.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * The repository currently has a legacy/monolithic Zamani.g4 containing a
 * statement rule. This modular file is intended to become the canonical
 * modular statement boundary.
 *
 * The integration layer MUST expose exactly one canonical `statement` rule to
 * the assembled parser.
 *
 * It MUST NOT assemble both:
 *
 *     Zamani.g4::statement
 *
 * and:
 *
 *     statements.g4::statement
 *
 * as competing definitions.
 *
 * The migration must therefore select one authoritative parser composition
 * path.
 *
 * This file is the owner once the modular grammar becomes authoritative.
 *
 * ============================================================================
 * STATEMENT CATEGORIES
 * ============================================================================
 *
 * Statements are grouped conceptually rather than by physical target.
 *
 * This is deliberate.
 *
 * A statement does not become a different syntactic category merely because
 * it eventually executes on:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     distributed system
 *     accelerator
 *     embedded system
 *
 * Target-specific realization belongs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CANONICAL STATEMENT ENTRY POINT
 * ============================================================================
 *
 * A statement is one syntactic unit in a source block or compilation context.
 *
 * Ordering of alternatives is intentional:
 *
 *     1. constructs with explicit leading keywords
 *     2. declarations / bindings
 *     3. blocks
 *     4. domain extensions
 *     5. expression statements
 *     6. empty statement
 *
 * The final parser assembly must ensure that imported rules do not create
 * ambiguous alternatives.
 *
 * Dedicated statement rules should use distinctive leading tokens wherever
 * practical.
 */
statement
    : attributedStatement
    | declarationStatement
    | bindingStatement
    | controlFlowStatement
    | blockExpression
    | effectStatement
    | concurrencyStatement
    | domainStatement
    | expressionStatement
    | emptyStatement
    ;


/*
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attributes annotate another statement.
 *
 * Attribute syntax itself is owned by the core/annotation grammar.
 *
 * This rule only establishes the statement-level composition.
 *
 * Examples:
 *
 *     #[inline]
 *     fn compute() { ... }
 *
 *     #[some_attribute(...)]
 *     let value = ...;
 *
 * Semantic validation of attributes is downstream.
 */
attributedStatement
    : annotation+ statement
    ;


/*
 * ============================================================================
 * DECLARATION STATEMENTS
 * ============================================================================
 *
 * Declarations may occur at statement positions where the language permits
 * local declarations.
 *
 * The individual declaration grammar remains authoritative.
 *
 * This rule MUST NOT duplicate declaration syntax.
 *
 * A declaration is syntactically admitted here; visibility, namespace,
 * ownership, generic validity, resource requirements, etc. are semantic
 * concerns.
 */
declarationStatement
    : functionDeclarationStatement
    | typeDeclarationStatement
    | moduleDeclarationStatement
    | importStatement
    | exportStatement
    | effectDeclarationStatement
    | domainDeclarationStatement
    ;


/*
 * ============================================================================
 * BINDING STATEMENTS
 * ============================================================================
 *
 * Local bindings belong to the statement layer because they introduce source
 * scope.
 *
 * Type inference, mutability rules, initialization analysis, ownership,
 * borrowing, lifetime, effect, and resource semantics are NOT decided here.
 */
bindingStatement
    : variableDeclarationStatement
    | constantDeclarationStatement
    ;


/*
 * ============================================================================
 * CONTROL FLOW
 * ============================================================================
 *
 * Control-flow syntax is composed here.
 *
 * Detailed implementations remain owned by their dedicated grammar modules.
 *
 * The statement dispatcher therefore remains stable as the language expands.
 */
controlFlowStatement
    : conditionalStatement
    | loopStatement
    | matchStatement
    | returnStatement
    | breakStatement
    | continueStatement
    | throwStatement
    | tryStatement
    ;


/*
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * Effects are language-level semantics, not hardware operations.
 *
 * A statement may request, perform, handle, or otherwise interact with an
 * effect. Whether the current function/module/capability context permits that
 * effect belongs to semantic analysis.
 */
effectStatement
    : performStatement
    | handleStatement
    ;


/*
 * ============================================================================
 * CONCURRENCY
 * ============================================================================
 *
 * Concurrency constructs remain abstract.
 *
 * The grammar does not encode:
 *
 *     core count
 *     thread count
 *     worker count
 *     queue count
 *     machine topology
 *     processor affinity
 *
 * Such properties belong to resource/capability/target/runtime layers.
 */
concurrencyStatement
    : spawnStatement
    | awaitStatement
    | taskStatement
    | synchronizationStatement
    ;


/*
 * ============================================================================
 * DOMAIN STATEMENTS
 * ============================================================================
 *
 * Zamani is intended to span multiple computational domains.
 *
 * Domain syntax is admitted through stable extension points rather than
 * embedding target-specific implementation assumptions into generic
 * statements.
 *
 * A domain statement can eventually lower to the appropriate semantic IR.
 *
 * Examples of domains include:
 *
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     distributed
 *     accelerator
 *     AI/ML
 *     networking
 *     scientific computing
 *     future domains
 *
 * This dispatcher does not decide which machine executes them.
 */
domainStatement
    : quantumStatement
    | hardwareStatement
    | hdlStatement
    | distributedStatement
    | acceleratorStatement
    | dataStatement
    | aiStatement
    | networkingStatement
    ;


/*
 * ============================================================================
 * EXPRESSION STATEMENTS
 * ============================================================================
 *
 * An expression can occur as a statement when the language's semantic model
 * permits it.
 *
 * Examples:
 *
 *     compute();
 *     x = y;
 *     measure(q);
 *
 * Whether the expression is:
 *
 *     pure
 *     effectful
 *     resource-producing
 *     quantum
 *     hardware-related
 *     asynchronous
 *     distributed
 *
 * is determined downstream.
 *
 * This rule MUST NOT duplicate expression precedence.
 */
expressionStatement
    : expression statementTerminator
    ;


/*
 * ============================================================================
 * EMPTY STATEMENT
 * ============================================================================
 *
 * A standalone terminator is syntactically valid.
 *
 * Whether empty statements are desirable in a particular semantic context is
 * a semantic/lint concern, not a parser scalability concern.
 */
emptyStatement
    : statementTerminator
    ;


/*
 * ============================================================================
 * STATEMENT TERMINATION
 * ============================================================================
 *
 * Statement termination is deliberately centralized.
 *
 * If Zamani eventually supports multiple termination policies (for example,
 * semicolon-required and semicolon-optional source dialects), the lexer/parser
 * integration layer can adapt this rule without requiring every statement
 * production to be rewritten.
 *
 * The canonical current language reference permits semicolon-terminated
 * statements in its existing parser model.
 *
 * Do not silently introduce automatic-semicolon-insertion here without an
 * explicit language specification.
 */
statementTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * DECLARATION ADAPTER CONTRACTS
 * ============================================================================
 *
 * The following rules are named integration points.
 *
 * They MUST be bound by the assembled grammar to the authoritative declaration
 * grammar modules.
 *
 * They are deliberately kept as composition rules rather than implementations.
 */


/*
 * Function declarations.
 *
 * Owned by:
 *
 *     grammar/functions/*
 *
 * Semantic ownership:
 *
 *     frontend/semantic/function analysis
 *
 * Do not add function syntax here.
 */
functionDeclarationStatement
    : functionDeclaration
    ;


/*
 * Type declarations.
 *
 * Owned by:
 *
 *     grammar/declarations/*
 *     grammar/types/*
 *
 * Do not duplicate struct/enum/trait/interface/class syntax here.
 */
typeDeclarationStatement
    : typeDeclaration
    ;


/*
 * Module declarations.
 *
 * Owned by:
 *
 *     grammar/modules/*
 *
 * Module resolution is not parser responsibility.
 */
moduleDeclarationStatement
    : moduleDeclaration
    ;


/*
 * Imports.
 */
importStatement
    : importDeclaration
    ;


/*
 * Exports.
 */
exportStatement
    : exportDeclaration
    ;


/*
 * Effect declarations.
 */
effectDeclarationStatement
    : effectDeclaration
    ;


/*
 * Domain-level declarations.
 *
 * This is a composition boundary only.
 */
domainDeclarationStatement
    : quantumDeclaration
    | hardwareDeclaration
    | hdlDeclaration
    | distributedDeclaration
    | acceleratorDeclaration
    | dataDeclaration
    | aiDeclaration
    ;


/*
 * ============================================================================
 * BINDING ADAPTER CONTRACTS
 * ============================================================================
 */

variableDeclarationStatement
    : variableDeclaration
    ;


constantDeclarationStatement
    : constantDeclaration
    ;


/*
 * ============================================================================
 * CONTROL-FLOW ADAPTER CONTRACTS
 * ============================================================================
 */

conditionalStatement
    : ifStatement
    ;


loopStatement
    : whileStatement
    | doWhileStatement
    | forStatement
    | foreachStatement
    | parallelLoopStatement
    ;


matchStatement
    : matchExpressionStatement
    ;


returnStatement
    : RETURN expression? statementTerminator
    ;


breakStatement
    : BREAK statementTerminator
    ;


continueStatement
    : CONTINUE statementTerminator
    ;


throwStatement
    : THROW expression statementTerminator
    ;


tryStatement
    : tryCatchFinallyStatement
    ;


/*
 * ============================================================================
 * EFFECT ADAPTER CONTRACTS
 * ============================================================================
 */

performStatement
    : PERFORM expression statementTerminator
    ;


handleStatement
    : HANDLE expression blockExpression
    ;


/*
 * ============================================================================
 * CONCURRENCY ADAPTER CONTRACTS
 * ============================================================================
 *
 * These rules deliberately contain no resource counts.
 */

spawnStatement
    : SPAWN expression statementTerminator
    ;


awaitStatement
    : AWAIT expression statementTerminator
    ;


taskStatement
    : taskDeclaration
    ;


synchronizationStatement
    : synchronizationConstruct
    ;


/*
 * ============================================================================
 * DOMAIN ADAPTER CONTRACTS
 * ============================================================================
 *
 * These are syntactic integration points.
 *
 * Quantum:
 *
 *     grammar/quantum/*
 *
 * Hardware:
 *
 *     grammar/hardware/*
 *
 * HDL:
 *
 *     grammar/hdl/*
 *
 * Distributed:
 *
 *     grammar/distributed/*
 *
 * Accelerators:
 *
 *     grammar/classical/*
 *     grammar/hardware/*
 *
 * Data:
 *
 *     grammar/data/*
 *
 * AI:
 *
 *     grammar/ai/*
 *
 * Networking:
 *
 *     grammar/networking/*
 *
 * None of these rules contain machine-specific limits.
 */


/*
 * Quantum statement.
 *
 * Quantum semantics are lowered later to the canonical quantum semantic
 * boundary. This grammar does NOT construct quantum::ir.
 */
quantumStatement
    : quantumOperationStatement
    | quantumMeasurementStatement
    | quantumResetStatement
    | quantumControlStatement
    | quantumCircuitStatement
    ;


hardwareStatement
    : hardwareOperationStatement
    | hardwareResourceStatement
    | hardwareControlStatement
    ;


hdlStatement
    : hdlProcessStatement
    | hdlAssignmentStatement
    | hdlControlStatement
    ;


distributedStatement
    : remoteExecutionStatement
    | messageStatement
    | serviceStatement
    ;


acceleratorStatement
    : acceleratorInvocationStatement
    ;


dataStatement
    : dataOperationStatement
    ;


aiStatement
    : modelOperationStatement
    | trainingStatement
    | inferenceStatement
    ;


networkingStatement
    : networkOperationStatement
    ;


/*
 * ============================================================================
 * BLOCK INTEGRATION
 * ============================================================================
 *
 * blocks.g4 owns blockExpression.
 *
 * This file does NOT redefine:
 *
 *     {
 *         ...
 *     }
 *
 * This is essential to prevent two competing block grammars.
 *
 * Canonical flow:
 *
 *     statement
 *         |
 *         +--> blockExpression
 *                    |
 *                    +--> blockElement*
 *                               |
 *                               +--> statement
 *
 * The resulting recursive relationship is intentional and represents nested
 * source scopes.
 *
 * The grammar assembler must import these rules without creating duplicate
 * definitions.
 */


/*
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * `expression` belongs to the expression grammar.
 *
 * This file does not define:
 *
 *     precedence
 *     associativity
 *     literals
 *     calls
 *     indexing
 *     member access
 *     unary operators
 *     binary operators
 *     lambda syntax
 *     ranges
 *
 * This prevents the statement grammar from becoming a second expression
 * grammar.
 */


/*
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately no grammar-level limits for:
 *
 *     statement count
 *     block count
 *     nesting depth
 *     declarations
 *     expressions
 *     functions
 *     loops
 *     quantum operations
 *     qubits
 *     devices
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     nodes
 *     memory
 *     accelerators
 *     tensor dimensions
 *
 * In particular, this file MUST NOT contain constructs such as:
 *
 *     MAX_STATEMENTS
 *     MAX_BLOCKS
 *     MAX_NESTING
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * Any parser-resource protection must be external and configurable.
 *
 * Examples:
 *
 *     parser resource budget
 *     maximum input bytes
 *     maximum parse time
 *     configurable recursion policy
 *
 * Such limits protect an implementation.
 *
 * They do NOT define the Zamani language.
 */


/*
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Statement syntax represents program intent and control structure.
 *
 * It does not represent a particular machine.
 *
 * Therefore the same statement structure may be lowered differently depending
 * on available capabilities:
 *
 *     Zamani source
 *         |
 *         v
 *     same statement semantics
 *         |
 *         +--> CPU
 *         +--> GPU
 *         +--> FPGA
 *         +--> ASIC
 *         +--> QPU
 *         +--> simulator
 *         +--> distributed system
 *         +--> heterogeneous system
 *         +--> future target
 *
 * The source language does not need to change merely because the target
 * changes.
 */


/*
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum statements are syntax only.
 *
 * This file must never:
 *
 *     allocate qubits
 *     discover hardware
 *     choose physical qubits
 *     select a backend
 *     inspect calibration
 *     route gates
 *     schedule operations
 *     apply QEC
 *     model noise
 *
 * Instead:
 *
 *     grammar
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic quantum representation
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization / routing / scheduling / ZQN / QEC / hardware
 *
 * This preserves the canonical `quantum::ir` boundary.
 */


/*
 * ============================================================================
 * HARDWARE / HDL BOUNDARY
 * ============================================================================
 *
 * Hardware statements describe source-level intent.
 *
 * They do not imply:
 *
 *     a fixed device
 *     a fixed topology
 *     a fixed number of resources
 *     a fixed clock frequency
 *     a fixed register count
 *     a fixed memory size
 *     a fixed bus width
 *
 * Those properties belong to capability/resource/target descriptions.
 */


/*
 * ============================================================================
 * SEMANTIC RESPONSIBILITY
 * ============================================================================
 *
 * After parsing, semantic analysis is responsible for determining:
 *
 *     - scope
 *     - name resolution
 *     - reachability
 *     - definite initialization
 *     - type correctness
 *     - effect correctness
 *     - capability requirements
 *     - resource requirements
 *     - ownership
 *     - borrowing
 *     - lifetime
 *     - quantum validity
 *     - hardware validity
 *     - control-flow validity
 *     - domain interoperability
 *
 * No such checks belong in this grammar.
 */


/*
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should retain:
 *
 *     source file
 *     byte/character span
 *     line
 *     column
 *     expected tokens/rules
 *     actual token
 *     parser context
 *
 * Diagnostic formatting belongs to the parser/frontend diagnostic subsystem,
 * not this grammar.
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this rule MUST depend only on:
 *
 *     source
 *     lexer definition
 *     grammar version
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     hardware topology
 *     calibration
 *     network state
 *     scheduler state
 *     runtime state
 *     backend state
 *     current wall-clock time
 *
 * Identical source and parser configuration must produce equivalent parse
 * structure.
 */


/*
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * New statement families should normally be integrated by:
 *
 *     1. adding the dedicated grammar module;
 *     2. defining its ownership contract;
 *     3. exposing one canonical entry rule;
 *     4. adding that rule to the appropriate composition category here;
 *     5. adding positive/negative/boundary tests;
 *     6. documenting semantic lowering;
 *
 * Do NOT modify unrelated statement productions to accommodate a new domain.
 *
 * This keeps additions localized and prevents cross-domain grammar coupling.
 */


/*
 * ============================================================================
 * FORWARD INTEGRATION CONTRACT
 * ============================================================================
 *
 * The following symbolic rule names represent contracts with other grammar
 * modules:
 *
 *     annotation
 *     functionDeclaration
 *     typeDeclaration
 *     moduleDeclaration
 *     importDeclaration
 *     exportDeclaration
 *     effectDeclaration
 *     quantumDeclaration
 *     hardwareDeclaration
 *     hdlDeclaration
 *     distributedDeclaration
 *     acceleratorDeclaration
 *     dataDeclaration
 *     aiDeclaration
 *     variableDeclaration
 *     constantDeclaration
 *     ifStatement
 *     whileStatement
 *     doWhileStatement
 *     forStatement
 *     foreachStatement
 *     parallelLoopStatement
 *     matchExpressionStatement
 *     tryCatchFinallyStatement
 *     taskDeclaration
 *     synchronizationConstruct
 *     quantumOperationStatement
 *     quantumMeasurementStatement
 *     quantumResetStatement
 *     quantumControlStatement
 *     quantumCircuitStatement
 *     hardwareOperationStatement
 *     hardwareResourceStatement
 *     hardwareControlStatement
 *     hdlProcessStatement
 *     hdlAssignmentStatement
 *     hdlControlStatement
 *     remoteExecutionStatement
 *     messageStatement
 *     serviceStatement
 *     acceleratorInvocationStatement
 *     dataOperationStatement
 *     modelOperationStatement
 *     trainingStatement
 *     inferenceStatement
 *     networkOperationStatement
 *     expression
 *     blockExpression
 *
 * The grammar composition/build layer is responsible for making these rules
 * available exactly once.
 *
 * If the repository ultimately chooses different canonical rule names, those
 * names must be changed in the grammar-composition layer and documented in the
 * grammar authority specification rather than creating duplicate semantic
 * rules.
 */


/*
 * ============================================================================
 * LEGACY MIGRATION CONTRACT
 * ============================================================================
 *
 * The current monolithic grammar contains statement forms directly.
 *
 * During migration:
 *
 *     OLD:
 *         grammar/Zamani.g4
 *             -> statement
 *
 *     NEW:
 *         grammar/statements/statements.g4
 *             -> statement
 *
 * There must be one authoritative implementation.
 *
 * The old rule must eventually be:
 *
 *     removed,
 *     delegated,
 *     or generated from the modular grammar,
 *
 * according to the repository's final grammar-authority policy.
 *
 * It must NOT remain as an independently evolving second grammar.
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file requires at least the following test classes.
 *
 * --------------------------------------------------------------------------
 * Positive
 * --------------------------------------------------------------------------
 *
 *     empty statement
 *     variable declaration
 *     constant declaration
 *     function declaration
 *     type declaration
 *     module declaration
 *     import
 *     export
 *     if
 *     while
 *     do/while
 *     for
 *     foreach
 *     parallel iteration
 *     match
 *     return
 *     break
 *     continue
 *     throw
 *     try/catch/finally
 *     block
 *     expression statement
 *     effect operation
 *     concurrency operation
 *     quantum statement
 *     HDL statement
 *     hardware statement
 *     distributed statement
 *     accelerator statement
 *     data statement
 *     AI statement
 *     networking statement
 *
 * --------------------------------------------------------------------------
 * Negative
 * --------------------------------------------------------------------------
 *
 *     missing terminator
 *     malformed declaration
 *     malformed control flow
 *     unmatched braces
 *     incomplete statement
 *     invalid statement ordering
 *     malformed attribute
 *     malformed expression statement
 *
 * --------------------------------------------------------------------------
 * Boundary
 * --------------------------------------------------------------------------
 *
 *     zero statements
 *     one statement
 *     very many statements
 *     deeply nested blocks
 *     large expressions
 *     large mixed-domain blocks
 *
 * Tests must not encode artificial language limits.
 *
 * --------------------------------------------------------------------------
 * Cross-domain
 * --------------------------------------------------------------------------
 *
 *     classical + quantum
 *     classical + HDL
 *     quantum + HDL
 *     quantum + hardware
 *     quantum + distributed
 *     AI + quantum
 *     AI + hardware
 *     classical + quantum + distributed
 *     classical + quantum + HDL + hardware
 *
 * --------------------------------------------------------------------------
 * Determinism
 * --------------------------------------------------------------------------
 *
 * Parse identical source repeatedly and verify equivalent syntax trees.
 *
 * --------------------------------------------------------------------------
 * Round-trip
 * --------------------------------------------------------------------------
 *
 * Where a canonical source printer exists:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * must preserve semantics.
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     numeric machine limits
 *     fixed resource counts
 *     fixed qubit counts
 *     fixed device identifiers
 *     physical addresses
 *     topology assumptions
 *     backend names
 *     calibration values
 *     timing constants
 *     fixed tensor dimensions
 *     fixed accelerator counts
 *
 * Any implementation limit required for parser safety must be outside the
 * language grammar and externally configurable.
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] `statement` is the single canonical statement-composition entry point.
 *
 * [ ] No individual statement grammar is unnecessarily duplicated here.
 *
 * [ ] Blocks are delegated to blocks.g4.
 *
 * [ ] Expressions are delegated to the expression grammar.
 *
 * [ ] Types are delegated to the type grammar.
 *
 * [ ] Declarations are delegated to declaration grammar.
 *
 * [ ] Functions are delegated to function grammar.
 *
 * [ ] Modules are delegated to module grammar.
 *
 * [ ] Quantum syntax is delegated to quantum grammar.
 *
 * [ ] HDL syntax is delegated to HDL grammar.
 *
 * [ ] Hardware syntax is delegated to hardware grammar.
 *
 * [ ] No quantum::ir dependency exists.
 *
 * [ ] No QEC dependency exists.
 *
 * [ ] No ZQN dependency exists.
 *
 * [ ] No routing/scheduling dependency exists.
 *
 * [ ] No hardware discovery exists.
 *
 * [ ] No runtime dependency exists.
 *
 * [ ] No machine-size constants exist.
 *
 * [ ] No unsafe code exists.
 *
 * [ ] Rust 1.97 / 1.97.1 remains the repository integration baseline.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] The legacy monolithic statement rule has a documented migration path.
 *
 * [ ] The assembled grammar contains exactly one authoritative `statement`
 *     rule.
 *
 * [ ] The parser can represent arbitrarily large source programs subject only
 *     to externally configured implementation/resource limits.
 *
 * [ ] POCO-REAF is preserved because this grammar expresses source semantics
 *     rather than machine topology or capacity.
 *
 * ============================================================================
 */