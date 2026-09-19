/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/assertions.g4
 *
 * STATUS
 * ------
 * CANONICAL PRODUCTION GRAMMAR
 *
 * OWNER
 * -----
 * Statement-level runtime assertions.
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar.
 *
 * RUST IMPLEMENTATION BASELINE
 * ----------------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * SAFETY
 * ------
 * This grammar:
 *
 *   - contains no embedded Rust actions;
 *   - contains no semantic predicates;
 *   - contains no unsafe Rust;
 *   - performs no I/O;
 *   - performs no filesystem access;
 *   - performs no networking;
 *   - performs no process execution;
 *   - performs no hardware discovery;
 *   - performs no runtime execution;
 *   - contains no mutable global state;
 *   - contains no machine-specific constants.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE AUTHORITATIVE SYNTAX OWNER for ordinary
 * statement-level assertions.
 *
 * Canonical forms:
 *
 *     assert(condition);
 *
 *     assert(condition, explanation);
 *
 * An assertion consists of:
 *
 *     ASSERT
 *     (
 *         condition
 *         optional explanation
 *     )
 *     statement terminator
 *
 * This grammar describes only source syntax.
 *
 * It does NOT determine:
 *
 *     - whether the condition is true;
 *     - whether the condition is statically provable;
 *     - when the condition is evaluated;
 *     - whether evaluation has side effects;
 *     - whether assertions are enabled;
 *     - whether assertions are optimized;
 *     - whether an assertion is removed;
 *     - whether failure aborts execution;
 *     - whether failure is recoverable;
 *     - how diagnostics are represented;
 *     - how assertions lower to IR;
 *     - which target executes the assertion;
 *     - which hardware executes the assertion.
 *
 * Those concerns belong downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     assertionStatement
 *     assertionCondition
 *     assertionExplanation
 *
 * and only the syntax required to compose those rules.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer tokens
 *     keyword spellings
 *     punctuation token definitions
 *     expression precedence
 *     expression syntax
 *     type syntax
 *     statement composition
 *     blocks
 *     declarations
 *     function declarations
 *     function return types
 *     contracts
 *     invariants
 *     compile-time assertions
 *     formal proofs
 *     semantic analysis
 *     diagnostics
 *     AST implementation
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     calibration
 *     HAL
 *     target selection
 *     resource discovery
 *     capability discovery
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative statement-level assertion entry:
 *
 *     assertionStatement
 *
 * This file owns that rule.
 *
 * grammar/statements/statements.g4 owns the universal:
 *
 *     statement
 *
 * composition rule.
 *
 * statements.g4 MUST import this grammar and reference:
 *
 *     assertionStatement
 *
 * It MUST NOT reproduce:
 *
 *     ASSERT LPAREN ...
 *
 * or otherwise duplicate assertion syntax.
 *
 * ============================================================================
 * CURRENT REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The repository's statement composition grammar currently imports:
 *
 *     AssertionsParser
 *
 * and composes:
 *
 *     assertionStatement
 *
 * This file therefore retains the parser grammar name:
 *
 *     AssertionsParser
 *
 * Do NOT rename it merely for stylistic reasons.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * The production parser consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The canonical lexical system is responsible for providing:
 *
 *     ASSERT
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     SEMICOLON
 *
 * The existing keyword registry defines:
 *
 *     ASSERT : 'assert' ;
 *
 * This grammar MUST NOT redefine ASSERT.
 *
 * The lexer composition architecture distinguishes:
 *
 *     ZamaniTokens
 *
 * from the production lexer:
 *
 *     ZamaniLexer
 *
 * Parser grammars consume:
 *
 *     ZamaniLexer
 *
 * and MUST NOT switch independently to:
 *
 *     ZamaniTokens
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * whose canonical parser grammar name is:
 *
 *     Expressions
 *
 * This file imports Expressions so that it is independently composable.
 *
 * This grammar MUST NOT define another:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     rangeExpression
 *     logical expression
 *     arithmetic expression
 *     postfix expression
 *     primary expression
 *     quantum expression
 *     hardware expression
 *     HDL expression
 *
 * hierarchy.
 *
 * Assertion conditions and explanations both use the canonical expression
 * boundary.
 *
 * Consequently, assertion syntax remains reusable for expressions originating
 * from:
 *
 *     classical computation
 *     numerical computation
 *     symbolic computation
 *     quantum/classical computation
 *     hybrid computation
 *     HDL/co-design
 *     hardware abstractions
 *     distributed computation
 *     AI/ML
 *     data processing
 *     networking
 *     accelerators
 *     scientific computing
 *     future computational domains
 *
 * ============================================================================
 * PUNCTUATION INTEGRATION
 * ============================================================================
 *
 * Statement termination and structural punctuation are owned by:
 *
 *     grammar/core/punctuation.g4
 *
 * whose parser grammar name is:
 *
 *     Punctuation
 *
 * This file imports Punctuation because it consumes:
 *
 *     statementTerminator
 *
 * and therefore does not create a private termination rule.
 *
 * The canonical termination contract is currently:
 *
 *     statementTerminator
 *         : SEMICOLON
 *         ;
 *
 * Therefore:
 *
 *     assert(condition);
 *
 * is the canonical terminated form.
 *
 * Automatic semicolon insertion is NOT introduced here.
 *
 * ============================================================================
 * CANONICAL ASSERTION SYNTAX
 * ============================================================================
 *
 * The authoritative grammar is:
 *
 *     assertionStatement
 *         : ASSERT LPAREN assertionCondition
 *           assertionExplanation?
 *           RPAREN statementTerminator
 *         ;
 *
 * Therefore exactly these structural forms are accepted:
 *
 *     assert(condition);
 *
 *     assert(condition, explanation);
 *
 * There is:
 *
 *     exactly one condition;
 *
 *     zero or one explanation.
 *
 * ============================================================================
 * WHY THE EXPLANATION IS AN EXPRESSION
 * ============================================================================
 *
 * The explanation is deliberately not restricted to STRING.
 *
 * Examples:
 *
 *     assert(condition, "message");
 *
 *     assert(condition, diagnostic);
 *
 *     assert(condition, format_error(context));
 *
 *     assert(condition, diagnostic_value);
 *
 * The semantic layer determines which expression types are valid as
 * explanations.
 *
 * This avoids coupling the parser to one diagnostic representation.
 *
 * ============================================================================
 * NO VARIADIC ASSERTION ARGUMENTS
 * ============================================================================
 *
 * The canonical grammar deliberately accepts:
 *
 *     assert(condition);
 *
 *     assert(condition, explanation);
 *
 * but not:
 *
 *     assert(condition, explanation, extra);
 *
 * If structured diagnostic arguments are required in the future, that is a
 * language-design change and must go through:
 *
 *     specification
 *         ->
 *     AST contract
 *         ->
 *     semantic contract
 *         ->
 *     canonical grammar
 *         ->
 *     tests
 *         ->
 *     compatibility decision
 *
 * It MUST NOT be introduced accidentally by changing `?` to `*`.
 *
 * ============================================================================
 * ASSERTION CONDITION
 * ============================================================================
 *
 * The condition is an ordinary canonical Zamani expression.
 *
 * This grammar intentionally does not create:
 *
 *     booleanExpression
 *
 * as a special assertion-only hierarchy.
 *
 * Whether the expression is a valid assertion predicate is determined by
 * semantic analysis.
 *
 * This permits the language to evolve its predicate semantics without
 * fragmenting the expression grammar.
 *
 * ============================================================================
 * ASSERTION EXPLANATION
 * ============================================================================
 *
 * The optional explanation is represented as:
 *
 *     assertionExplanation
 *         : COMMA expression
 *         ;
 *
 * Keeping the comma inside this rule makes the assertion's two-part structure
 * explicit:
 *
 *     condition
 *     optional explanation
 *
 * It also prevents an arbitrary comma-separated argument list from silently
 * becoming assertion syntax.
 *
 * ============================================================================
 * COMPILE-TIME ASSERTION SEPARATION
 * ============================================================================
 *
 * Compile-time assertion syntax is owned by:
 *
 *     grammar/expressions/compile-time.g4
 *
 * through:
 *
 *     compileTimeAssertionExpression
 *
 * This file MUST NOT redefine compile-time assertion syntax.
 *
 * These are different source constructs:
 *
 *     assert(condition);
 *
 *     comptime assert(condition);
 *
 * Their semantic implementations may share infrastructure downstream, but
 * their grammar ownership remains distinct.
 *
 * ============================================================================
 * CONTRACT / INVARIANT SEPARATION
 * ============================================================================
 *
 * Function/module/declaration contracts are separate concepts.
 *
 * Examples include:
 *
 *     requires(...)
 *     ensures(...)
 *     invariant(...)
 *
 * This file MUST NOT absorb those constructs.
 *
 * An assertion is an executable/checkable statement.
 *
 * A contract expresses a semantic obligation associated with another language
 * entity.
 *
 * ============================================================================
 * FORMAL VERIFICATION SEPARATION
 * ============================================================================
 *
 * An assertion can provide a predicate that a verification subsystem may
 * consume.
 *
 * It does NOT itself constitute a mathematical proof.
 *
 * Proof obligations, theorem proving, verification directives, and formal
 * proof syntax remain owned by their appropriate language/specification
 * layers.
 *
 * ============================================================================
 * CONTROL-FLOW INTEGRATION
 * ============================================================================
 *
 * This grammar establishes only the syntax of a return-independent assertion
 * statement.
 *
 * The statement dispatcher determines where assertions may syntactically
 * occur.
 *
 * Semantic analysis determines contextual validity.
 *
 * Examples of downstream semantic questions include:
 *
 *     - is the assertion reachable?
 *     - is the predicate type valid?
 *     - are required effects available?
 *     - are required capabilities available?
 *     - are referenced values in scope?
 *     - does the enclosing execution model permit the assertion?
 *
 * None of those decisions belong here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Successful parsing must provide enough information for the frontend AST
 * builder to preserve:
 *
 *     - assertion source span;
 *     - condition expression;
 *     - optional explanation expression;
 *     - source ordering.
 *
 * Conceptual representation:
 *
 *     AssertionStatement {
 *         condition,
 *         explanation?,
 *         source_span
 *     }
 *
 * The exact Rust AST type is owned by the frontend AST subsystem.
 *
 * This grammar MUST NOT:
 *
 *     - define Rust structs;
 *     - define Rust enums;
 *     - construct AST objects;
 *     - allocate AST storage;
 *     - evaluate expressions.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis consumes the parsed assertion and determines:
 *
 *     - condition validity;
 *     - condition type;
 *     - explanation validity;
 *     - explanation type;
 *     - name resolution;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - control-flow legality;
 *     - execution policy;
 *     - failure semantics.
 *
 * A syntactically valid assertion may therefore still be semantically invalid.
 *
 * Example:
 *
 *     assert(unknown_name);
 *
 * is structurally valid but may fail name resolution.
 *
 * Likewise:
 *
 *     assert(non_boolean_value);
 *
 * may be syntactically valid while failing the language's semantic predicate
 * rules.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The downstream pipeline remains:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical semantic representation
 *       ->
 *     domain IR
 *       ->
 *     optimization
 *       ->
 *     routing / scheduling / lowering
 *       ->
 *     target realization
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * An assertion may contain an expression whose semantic origin is quantum or
 * hybrid computation.
 *
 * Examples:
 *
 *     assert(measurement_result);
 *
 *     assert(measure(q) == expected);
 *
 *     assert(classical_condition);
 *
 * The assertion grammar does NOT determine whether a referenced operation is:
 *
 *     quantum
 *     classical
 *     hybrid
 *     simulator-specific
 *     hardware-specific
 *     vendor-specific
 *
 * Semantic analysis determines that meaning.
 *
 * If the computation is quantum, the established downstream path remains:
 *
 *     AST
 *       ->
 *     quantum semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     QEC / resilience / ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * This grammar MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - enumerate qubits;
 *     - assign physical qubits;
 *     - define QEC;
 *     - define ZQN;
 *     - perform routing;
 *     - perform scheduling;
 *     - inspect calibration;
 *     - inspect hardware;
 *     - create a second quantum IR.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE / AI / DATA INTEGRATION
 * ============================================================================
 *
 * The same assertion syntax is reusable for:
 *
 *     classical values;
 *     numerical invariants;
 *     symbolic expressions;
 *     tensors;
 *     AI/model values;
 *     data-processing results;
 *     distributed state;
 *     accelerator results;
 *     hardware state;
 *     HDL simulation properties;
 *     hybrid results.
 *
 * Do NOT introduce:
 *
 *     cpuAssert
 *     gpuAssert
 *     fpgaAssert
 *     qpuAssert
 *     hdlAssert
 *     distributedAssert
 *     acceleratorAssert
 *
 * merely because the expression eventually executes on a particular target.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INDEPENDENCE
 * ============================================================================
 *
 * Assertion syntax has no dependency on physical resources.
 *
 * This file MUST NOT encode:
 *
 *     MAX_ASSERTIONS
 *     MAX_ASSERTION_DEPTH
 *     MAX_EXPRESSION_SIZE
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_ACCELERATORS
 *
 * It MUST NOT encode:
 *
 *     physical device identifiers;
 *     physical qubit identifiers;
 *     hardware addresses;
 *     fixed topology;
 *     fixed memory-bank identifiers;
 *     fixed accelerator counts.
 *
 * Resource requirements and capabilities belong to the resource/hardware
 * semantic layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The assertion language is target independent.
 *
 * The same source syntax can be preserved across:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASIC-oriented systems
 *     QPUs
 *     simulators
 *     accelerators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational architectures
 *
 * The grammar does not select any of them.
 *
 * The assertion expresses a property of the computation.
 *
 * This supports:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * without turning current hardware capabilities into language-level limits.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is no grammar-defined maximum for:
 *
 *     number of assertions;
 *     assertion expression size;
 *     expression nesting;
 *     block nesting;
 *     source-unit size;
 *     program size;
 *     number of quantum operations;
 *     number of qubits;
 *     number of processors;
 *     number of GPUs;
 *     number of nodes;
 *     amount of memory;
 *     number of accelerators.
 *
 * Grammar repetition and expression composition remain structural.
 *
 * "Infinity" here means:
 *
 *     no artificial finite machine limit is encoded by this grammar.
 *
 * Actual exhaustion is governed by available implementation resources and
 * explicit compiler/parser resource policies.
 *
 * Such implementation limits MUST NOT be silently converted into language
 * grammar limits.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing an assertion depends only on:
 *
 *     - source;
 *     - lexer tokenization;
 *     - active grammar;
 *     - parser configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     - system time;
 *     - randomness;
 *     - environment state;
 *     - filesystem state;
 *     - network state;
 *     - hardware availability;
 *     - CPU count;
 *     - GPU count;
 *     - QPU availability;
 *     - runtime state;
 *     - scheduler state.
 *
 * No embedded actions or predicates are used.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing:
 *
 *     assert(system.run(command));
 *
 * must only construct syntax.
 *
 * The parser MUST NOT:
 *
 *     - execute the command;
 *     - access the filesystem;
 *     - contact the network;
 *     - inspect hardware;
 *     - access secrets;
 *     - execute external processes.
 *
 * Any later evaluation is governed by semantic, compiler, and runtime
 * security policies.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors include malformed structures such as:
 *
 *     assert;
 *     assert();
 *     assert(;
 *     assert(, explanation);
 *     assert(condition;
 *     assert(condition,);
 *     assert(condition, explanation, extra);
 *     assert(condition) trailing;
 *
 * The parser/frontend diagnostic subsystem owns:
 *
 *     - diagnostic wording;
 *     - source-span rendering;
 *     - recovery strategy;
 *     - error aggregation;
 *     - IDE diagnostics.
 *
 * This grammar only defines the structural boundary.
 *
 * ============================================================================
 * IMPORTANT WHITESPACE RULE
 * ============================================================================
 *
 * Whitespace is lexically insignificant unless the canonical lexer specifies
 * otherwise.
 *
 * Therefore:
 *
 *     assert(condition);
 *
 * and:
 *
 *     assert ( condition ) ;
 *
 * may tokenize identically if the lexer treats whitespace as insignificant.
 *
 * The grammar MUST NOT reject valid whitespace merely for formatting reasons.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical forms are:
 *
 *     assert(condition);
 *
 *     assert(condition, explanation);
 *
 * Existing legacy assertion syntax must be checked against the repository's
 * compatibility policy before being removed.
 *
 * In particular, if an older grammar accepted:
 *
 *     assert expression;
 *
 * without parentheses, its status must be explicitly recorded as:
 *
 *     preserved
 *     migrated
 *     deprecated
 *     or removed.
 *
 * It must not disappear accidentally as a side effect of modularization.
 *
 * ============================================================================
 * DIALECT RULE
 * ============================================================================
 *
 * Dialects MUST NOT silently redefine the core meaning of:
 *
 *     assert
 *
 * A dialect requiring additional assertion syntax must introduce an explicit
 * extension through the dialect/feature lifecycle:
 *
 *     proposal
 *       ->
 *     specification
 *       ->
 *     AST contract
 *       ->
 *     semantic contract
 *       ->
 *     grammar
 *       ->
 *     tests
 *       ->
 *     compatibility decision.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE SYNTAX
 * ---------------
 *
 *     assert(true);
 *
 *     assert(false);
 *
 *     assert(condition);
 *
 *     assert(value == expected);
 *
 *     assert(condition, "message");
 *
 *     assert(condition, diagnostic);
 *
 *     assert(condition, format_error(context));
 *
 *     assert(computation());
 *
 *     assert((nested_expression));
 *
 *     assert(a + b == c);
 *
 *     assert(measurement_result);
 *
 *     assert(measure(q) == expected);
 *
 *     assert(distributed_result);
 *
 *     assert(hardware_result);
 *
 *     assert(accelerator_result);
 *
 * NEGATIVE SYNTAX
 * ---------------
 *
 *     assert;
 *
 *     assert();
 *
 *     assert(;
 *
 *     assert(, explanation);
 *
 *     assert(condition;
 *
 *     assert(condition,);
 *
 *     assert(condition, explanation, extra);
 *
 *     assert(condition) trailing;
 *
 *     assert((condition);
 *
 *     assert(condition));
 *
 * SEMANTIC NEGATIVES
 * ------------------
 *
 * These are NOT grammar errors merely because they may be semantically
 * invalid:
 *
 *     assert(unknown_name);
 *
 *     assert(non_predicate_value);
 *
 *     assert(invalid_domain_value);
 *
 *     assert(incompatible_diagnostic);
 *
 * Semantic analysis owns those diagnostics.
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Test:
 *
 *     - empty explanation expressions where the expression grammar permits
 *       them or rejects them;
 *     - deeply nested conditions;
 *     - deeply nested explanations;
 *     - large arithmetic expressions;
 *     - large symbolic expressions;
 *     - large tensor expressions;
 *     - large hybrid expressions;
 *     - large quantum-derived expressions;
 *     - large HDL/hardware expressions;
 *     - many assertions in one block;
 *     - many assertions in one source unit.
 *
 * No finite language-level ceiling may be encoded.
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Generated tests should vary:
 *
 *     assertion count;
 *     expression size;
 *     expression depth;
 *     block size;
 *     program size;
 *
 * without modifying this grammar.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Verify assertion syntax can occur around expressions involving:
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
 *     accelerators
 *     scientific computing
 *
 * where independently legal.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Repeated parsing of identical source under identical parser configuration
 * must produce equivalent parse-tree structure.
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * The same assertion source must remain syntactically identical regardless of
 * target descriptions or resource availability.
 *
 * ROUND-TRIP TESTS
 * ----------------
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter/printer
 *       ->
 *     parser
 *
 * must preserve:
 *
 *     condition;
 *     optional explanation;
 *     statement structure.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * REQUIRED:
 *
 *     no fixed resource counts;
 *     no fixed machine dimensions;
 *     no fixed assertion count;
 *     no fixed expression size;
 *     no fixed expression depth;
 *     no device identifiers;
 *     no topology;
 *     no backend selection;
 *     no vendor-specific assertion syntax.
 *
 * Forbidden examples include:
 *
 *     MAX_ASSERTIONS
 *     MAX_ASSERTION_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_ACCELERATORS
 *
 * Numeric values appearing inside an assertion expression remain ordinary
 * program data.
 *
 * Example:
 *
 *     assert(value < 1024);
 *
 * contains a program-level constant.
 *
 * It does NOT establish:
 *
 *     MAX_VALUE = 1024
 *
 * for the language implementation.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] Parser grammar name remains `AssertionsParser`.
 *
 * [ ] `tokenVocab = ZamaniLexer` is used.
 *
 * [ ] `ASSERT` is supplied by the canonical lexer.
 *
 * [ ] `LPAREN` is supplied by the canonical lexer.
 *
 * [ ] `RPAREN` is supplied by the canonical lexer.
 *
 * [ ] `COMMA` is supplied by the canonical lexer.
 *
 * [ ] `SEMICOLON` is supplied by the canonical lexer.
 *
 * [ ] `Expressions` is imported.
 *
 * [ ] `Punctuation` is imported.
 *
 * [ ] `expression` comes from the canonical expression grammar.
 *
 * [ ] `statementTerminator` comes from the canonical punctuation grammar.
 *
 * [ ] `assertionStatement` is the sole authoritative assertion entry.
 *
 * [ ] `assertionCondition` is the sole condition wrapper owned here.
 *
 * [ ] `assertionExplanation` is the sole explanation wrapper owned here.
 *
 * [ ] No duplicate `assertionWithExplanation` entry exists.
 *
 * [ ] No duplicate `statement` rule exists.
 *
 * [ ] No duplicate expression hierarchy exists.
 *
 * [ ] No compile-time assertion is duplicated.
 *
 * [ ] No contract syntax is duplicated.
 *
 * [ ] No formal-proof syntax is duplicated.
 *
 * [ ] No target-specific assertion syntax exists.
 *
 * [ ] No quantum gate list exists.
 *
 * [ ] No physical qubit identifiers exist.
 *
 * [ ] No machine-size constants exist.
 *
 * [ ] No resource cardinality limits exist.
 *
 * [ ] No hardware discovery occurs.
 *
 * [ ] No runtime execution occurs.
 *
 * [ ] No embedded Rust actions exist.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Rust 1.97 / 1.97.1 remains the implementation baseline.
 *
 * [ ] `statements.g4` imports `AssertionsParser`.
 *
 * [ ] `statements.g4` reaches `assertionStatement` exactly once.
 *
 * [ ] The frontend AST has a corresponding assertion representation.
 *
 * [ ] Semantic analysis validates the condition.
 *
 * [ ] Semantic analysis validates the optional explanation.
 *
 * [ ] IR lowering is downstream.
 *
 * [ ] `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * [ ] QEC remains downstream.
 *
 * [ ] ZQN remains downstream.
 *
 * [ ] routing remains downstream.
 *
 * [ ] scheduling remains downstream.
 *
 * [ ] HAL remains downstream.
 *
 * [ ] positive tests exist.
 *
 * [ ] negative tests exist.
 *
 * [ ] semantic-negative tests exist.
 *
 * [ ] boundary tests exist.
 *
 * [ ] scalability tests exist.
 *
 * [ ] cross-domain tests exist.
 *
 * [ ] determinism tests exist.
 *
 * [ ] portability tests exist.
 *
 * [ ] round-trip tests exist.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     1. There is exactly one authoritative assertionStatement rule.
 *
 *     2. Both canonical forms parse:
 *
 *            assert(condition);
 *            assert(condition, explanation);
 *
 *     3. Invalid multi-explanation forms do not parse.
 *
 *     4. The canonical expression grammar supplies both expressions.
 *
 *     5. The canonical punctuation grammar supplies statement termination.
 *
 *     6. The canonical lexer supplies all tokens.
 *
 *     7. statements.g4 composes this grammar without duplicating it.
 *
 *     8. The AST preserves condition, optional explanation, and source span.
 *
 *     9. Semantic analysis owns predicate/type/context validation.
 *
 *    10. Compiler/IR layers own assertion lowering.
 *
 *    11. Runtime owns assertion-failure behavior.
 *
 *    12. Quantum behavior remains downstream of semantic analysis.
 *
 *    13. `quantum::ir` remains the canonical quantum boundary.
 *
 *    14. No hardware/resource topology is encoded.
 *
 *    15. No artificial scalability limit is encoded.
 *
 *    16. No unsafe Rust is introduced.
 *
 *    17. Rust 1.97 / 1.97.1 compatibility remains intact.
 *
 *    18. Positive, negative, boundary, scalability, cross-domain,
 *        determinism, portability, compatibility, and round-trip tests pass.
 *
 * ============================================================================
 * CANONICAL PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar AssertionsParser;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Expressions is the canonical expression grammar.
 *
 * Punctuation is the canonical statement-termination grammar.
 *
 * These are explicit dependencies so this grammar remains independently
 * composable and does not rely accidentally on transitive imports from
 * statements.g4.
 */
import
    Expressions,
    Punctuation
    ;

/*
 * ============================================================================
 * ASSERTION STATEMENT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     assert(condition);
 *
 *     assert(condition, explanation);
 *
 * The explanation is optional but may occur at most once.
 *
 * The statement terminator is mandatory.
 *
 * This is the ONLY assertion statement entry point in this grammar.
 */
assertionStatement
    : ASSERT
      LPAREN
      assertionCondition
      assertionExplanation?
      RPAREN
      statementTerminator
    ;

/*
 * ============================================================================
 * ASSERTION CONDITION
 * ============================================================================
 *
 * Uses the canonical Zamani expression grammar.
 *
 * Semantic analysis determines whether the resulting expression is a valid
 * assertion predicate.
 */
assertionCondition
    : expression
    ;

/*
 * ============================================================================
 * ASSERTION EXPLANATION
 * ============================================================================
 *
 * The explanation consists of exactly one comma followed by exactly one
 * canonical expression.
 *
 * Therefore:
 *
 *     assert(condition, explanation);
 *
 * is valid.
 *
 * while:
 *
 *     assert(condition, explanation, extra);
 *
 * is not accepted by this grammar.
 *
 * The semantic layer determines which expression types are valid as diagnostic
 * explanations.
 */
assertionExplanation
    : COMMA expression
    ;