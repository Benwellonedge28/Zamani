/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/generators.g4
 *
 * Grammar:
 *     Generators
 *
 * Status:
 *     Production-ready parser-grammar component.
 *
 * Purpose:
 *     Defines the source-level syntax owned by Zamani's generator model.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani source describes portable computation and intent.
 *
 * Generator syntax therefore describes:
 *
 *     - suspension;
 *     - production of a value;
 *     - resumption points;
 *     - generator-compatible control flow.
 *
 * It MUST NOT describe:
 *
 *     - CPU count;
 *     - thread count;
 *     - memory capacity;
 *     - device identity;
 *     - GPU count;
 *     - FPGA count;
 *     - QPU count;
 *     - qubit count;
 *     - physical topology;
 *     - scheduling policy;
 *     - runtime worker count;
 *     - deployment topology.
 *
 * There are no grammar-level limits on:
 *
 *     - number of yield statements;
 *     - number of generator functions;
 *     - number of suspension points;
 *     - nesting depth;
 *     - generator composition;
 *     - generator calls;
 *     - source size.
 *
 * Any implementation limits belong to compiler/runtime resource policies,
 * never to the language grammar.
 *
 * ============================================================================
 * IMPORTANT LANGUAGE DESIGN DECISION
 * ============================================================================
 *
 * Zamani currently has:
 *
 *     yield
 *
 * as a language construct, but does not establish a separate `generator`
 * keyword or a dedicated generator type constructor in the canonical type
 * grammar.
 *
 * Consequently:
 *
 *     fn produce() {
 *         yield value;
 *     }
 *
 * is syntactically a function declaration whose body contains a generator
 * suspension operation.
 *
 * Semantic analysis determines that the containing function is a generator.
 *
 * This is deliberate.
 *
 * This grammar MUST NOT invent:
 *
 *     generator fn ...
 *     gen fn ...
 *     generator<T>
 *     stream<T>
 *
 * as alternative language constructs merely to give generators a dedicated
 * syntax. Such constructs require a language-specification decision first.
 *
 * If a future Zamani version introduces an explicit generator declaration
 * keyword or first-class generator type, that feature must be added through a
 * versioned language change and integrated with the canonical function/type
 * grammars rather than silently changing this file's meaning.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - yield statement syntax;
 *     - generator yield-value syntax;
 *     - the parser-level generator suspension boundary;
 *     - generator-specific syntax fragments consumed by function/statement
 *       composition;
 *     - generator grammar integration contracts.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ordinary function declarations;
 *     - function names;
 *     - function parameters;
 *     - generic parameter declarations;
 *     - function return-type syntax;
 *     - function types;
 *     - ordinary expressions;
 *     - expression precedence;
 *     - blocks;
 *     - ordinary statements;
 *     - async function declarations;
 *     - async execution;
 *     - futures;
 *     - tasks;
 *     - executors;
 *     - channels;
 *     - scheduling;
 *     - cancellation;
 *     - generator state-machine lowering;
 *     - runtime generator frames;
 *     - allocation strategy;
 *     - memory layout;
 *     - calling conventions;
 *     - ABI;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - hardware discovery;
 *     - hardware selection;
 *     - optimization.
 *
 * Function declarations remain owned by:
 *
 *     grammar/functions/functions.g4
 *
 * Function types remain owned by:
 *
 *     grammar/types/function-types.g4
 *
 * Expressions remain owned by:
 *
 *     grammar/expressions/
 *
 * Blocks/statements remain owned by:
 *
 *     grammar/statements/
 *
 * Async declaration syntax remains owned by:
 *
 *     grammar/functions/async.g4
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     grammar/lexer/tokens.g4
 *                    |
 *                    v
 *               Generators
 *                    |
 *                    +--------------------+
 *                    |                    |
 *                    v                    v
 *              Expressions          Statements
 *                    |                    |
 *                    +---------+----------+
 *                              |
 *                              v
 *                         Functions
 *                              |
 *                              v
 *                             AST
 *                              |
 *                              v
 *                      Semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        classical IR     quantum::ir     effect/resource IR
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                optimization / scheduling /
 *                routing / hardware / runtime
 *
 * This file MUST NOT introduce reverse dependencies from grammar into:
 *
 *     IR
 *     runtime
 *     hardware
 *     scheduling
 *     optimization
 *     QEC
 *     ZQN
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Generator source expresses the computation's suspension/production
 * semantics, not the machine that will execute it.
 *
 * The same generator can therefore participate in:
 *
 *     - embedded execution;
 *     - CPU execution;
 *     - multicore execution;
 *     - accelerator execution;
 *     - distributed execution;
 *     - cloud execution;
 *     - heterogeneous execution;
 *     - future execution environments.
 *
 * Generator syntax MUST NOT require a particular:
 *
 *     - executor;
 *     - scheduler;
 *     - thread pool;
 *     - worker count;
 *     - memory model;
 *     - hardware architecture.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Grammar:
 *
 *     ZamaniTokens
 *
 * This grammar MUST NOT define lexer tokens.
 *
 * The relevant canonical tokens include:
 *
 *     K_YIELD
 *     SEMI
 *
 * `K_FROM` exists in the canonical lexical vocabulary but is NOT interpreted
 * here as a generator-specific `yield from` construct.
 *
 * A future `yield from` feature requires an explicit language specification
 * decision and semantic contract.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate grammar.
 *
 * The composed parser supplies the canonical:
 *
 *     expression
 *
 * rule from the expression subsystem.
 *
 * This grammar deliberately does not redefine expression syntax.
 *
 * The composed parser should import/use this grammar alongside the canonical
 * expression and statement grammars.
 *
 * Example composition:
 *
 *     parser grammar ZamaniParser;
 *
 *     options {
 *         tokenVocab = ZamaniTokens;
 *     }
 *
 *     import
 *         Core,
 *         Expressions,
 *         Statements,
 *         Functions,
 *         Generators;
 *
 * The exact root-composition file is determined by the repository's canonical
 * parser assembly.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing determines:
 *
 *     "Is this syntactically a yield statement?"
 *
 * Semantic analysis determines:
 *
 *     - whether the containing function is a generator;
 *     - whether yield is legal in the current function;
 *     - the yielded value's type;
 *     - whether the generator's yield type is consistent;
 *     - whether control can resume after yield;
 *     - whether all paths satisfy generator rules;
 *     - whether a generator is async;
 *     - whether the generator may cross an effect boundary;
 *     - whether the generator is compatible with ownership/borrowing rules;
 *     - whether the generator may be distributed;
 *     - whether its state is serializable;
 *     - whether checkpointing is legal;
 *     - whether cancellation is supported;
 *     - whether a runtime executor can execute it.
 *
 * None of those semantic decisions belong in this grammar.
 *
 * ============================================================================
 * RUNTIME / COMPILER BOUNDARY
 * ============================================================================
 *
 * A yield may eventually lower into:
 *
 *     source function
 *         ->
 *     generator semantic model
 *         ->
 *     suspension/resumption representation
 *         ->
 *     compiler-generated state machine
 *         ->
 *     target-specific implementation
 *         ->
 *     runtime executor
 *
 * The grammar MUST NOT encode the generated state machine.
 *
 * In particular, it must not encode:
 *
 *     - hidden frame fields;
 *     - program counters;
 *     - heap allocation;
 *     - stack allocation;
 *     - executor implementation;
 *     - polling strategy;
 *     - thread assignment.
 *
 * ============================================================================
 * QUANTUM / HYBRID BOUNDARY
 * ============================================================================
 *
 * A generator may produce values associated with:
 *
 *     - classical computation;
 *     - quantum computation;
 *     - measurements;
 *     - hardware events;
 *     - distributed messages;
 *     - data streams;
 *     - accelerator results.
 *
 * The generator grammar does not define those domains.
 *
 * For example, a semantic layer may eventually accept:
 *
 *     fn measurements() {
 *         yield measurement;
 *     }
 *
 * or:
 *
 *     async fn results() {
 *         yield result;
 *     }
 *
 * The meaning of `measurement`, `result`, quantum values, hardware values,
 * resources, and effects is determined by their respective semantic systems.
 *
 * The generator grammar must never manufacture a second quantum representation.
 *
 * Canonical quantum semantic representation remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains grammar only.
 *
 * No target-language actions, predicates, embedded source code, filesystem
 * access, networking, process execution, or unsafe operations are permitted.
 *
 * Generated parser/runtime integration targets:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The Rust implementation must use safe Rust only.
 *
 * ============================================================================
 */


/**
 * Parser delegate for generator-specific syntax.
 *
 * Generator-ness is semantic: a function containing one or more valid
 * `yieldStatement` nodes is classified downstream as a generator.
 */
parser grammar Generators;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC GENERATOR SYNTAX
 * ============================================================================
 *
 * The primary generator construct is:
 *
 *     yield
 *
 * or:
 *
 *     yield expression
 *
 * followed optionally by a statement terminator.
 *
 * Examples:
 *
 *     yield;
 *
 *     yield value;
 *
 *     yield compute(value);
 *
 *     yield measurement;
 *
 *     yield quantum_result;
 *
 * No fixed value type is encoded here.
 * ============================================================================
 */

yieldStatement
    : K_YIELD yieldValue? SEMI?
    ;


/* ============================================================================
 * 2. YIELD VALUE
 * ============================================================================
 *
 * A yield value is an ordinary Zamani expression.
 *
 * This is intentionally delegated to the canonical expression grammar.
 *
 * Generator syntax therefore automatically inherits support for:
 *
 *     literals
 *     identifiers
 *     calls
 *     arithmetic
 *     comparisons
 *     logical operations
 *     quantum expressions
 *     classical expressions
 *     hardware/resource expressions
 *     future expression forms
 *
 * without this grammar needing to duplicate them.
 * ============================================================================
 */

yieldValue
    : expression
    ;


/* ============================================================================
 * 3. EMPTY YIELD
 * ============================================================================
 *
 * `yield;` is syntactically valid.
 *
 * Whether an empty yield is semantically valid is determined downstream.
 *
 * Possible semantic interpretations include:
 *
 *     - unit-like generator value;
 *     - notification/suspension;
 *     - control-only suspension;
 *     - language-version-specific generator semantics.
 *
 * The parser must not choose among those interpretations.
 * ============================================================================
 */


/* ============================================================================
 * 4. GENERATOR YIELD EXPRESSION BOUNDARY
 * ============================================================================
 *
 * This rule provides a stable parser-level name for tooling and AST adapters.
 *
 * It intentionally delegates to the canonical yield statement.
 *
 * Consumers that need to identify generator suspension points should prefer
 * this rule's AST/source-span boundary rather than searching source text for
 * the string "yield".
 * ============================================================================
 */

generatorYield
    : yieldStatement
    ;


/* ============================================================================
 * 5. GENERATOR VALUE PRODUCTION
 * ============================================================================
 *
 * A generator yield produces zero or one source-level expression value.
 *
 * There is deliberately no grammar-level restriction such as:
 *
 *     exactly one type
 *     exactly N values
 *     maximum yield size
 *
 * Tuple, record, array, quantum, classical, hardware, or other values remain
 * ordinary expressions.
 *
 * Example:
 *
 *     yield (a, b);
 *
 * represents one expression whose value may itself be a tuple.
 *
 * The semantic type system determines the resulting generator value type.
 * ============================================================================
 */

generatorYieldValue
    : yieldValue
    ;


/* ============================================================================
 * 6. GENERATOR FUNCTION CLASSIFICATION BOUNDARY
 * ============================================================================
 *
 * This rule is intentionally a semantic marker rather than a second function
 * declaration grammar.
 *
 * A normal function declaration remains owned by:
 *
 *     grammar/functions/functions.g4
 *
 * The semantic analyser classifies a function as a generator when its body
 * contains generator suspension nodes.
 *
 * This prevents a second competing function syntax such as:
 *
 *     generator fn ...
 *
 * from being introduced accidentally.
 *
 * No parser rule in this file should duplicate `functionDeclaration`.
 * ============================================================================
 */

generatorFunctionBoundary
    : yieldStatement
    ;


/* ============================================================================
 * 7. NESTED GENERATOR EXPRESSIONS
 * ============================================================================
 *
 * Nested expressions remain ordinary expression syntax.
 *
 * For example:
 *
 *     yield transform(value);
 *
 *     yield condition ? a : b;
 *
 *     yield compute(x + y);
 *
 * are parsed by:
 *
 *     yieldStatement
 *         ->
 *     yieldValue
 *         ->
 *     expression
 *
 * The generator grammar does not define expression precedence.
 * ============================================================================
 */


/* ============================================================================
 * 8. GENERATOR COMPOSITION
 * ============================================================================
 *
 * A generator may semantically consume another generator through ordinary
 * function calls, iteration constructs, channels, or future stream abstractions.
 *
 * Example:
 *
 *     yield next(source);
 *
 * The grammar does not introduce a dedicated generator invocation syntax.
 *
 * This preserves POCO-REAF and avoids coupling generators to a particular
 * runtime iteration model.
 * ============================================================================
 */


/* ============================================================================
 * 9. ASYNC-GENERATOR COMPATIBILITY
 * ============================================================================
 *
 * An async function may contain yield:
 *
 *     async fn values() {
 *         yield value;
 *     }
 *
 * The existing Zamani language vocabulary already distinguishes:
 *
 *     async
 *     await
 *     yield
 *
 * Async declaration syntax belongs to:
 *
 *     grammar/functions/async.g4
 *
 * Generator yield syntax belongs here.
 *
 * Semantic analysis determines whether:
 *
 *     async + yield
 *
 * forms a valid async generator.
 *
 * This grammar does not introduce:
 *
 *     async-generator
 *     async_generator
 *     generator_async
 *
 * keywords or types.
 * ============================================================================
 */


/* ============================================================================
 * 10. YIELD AND CONTROL FLOW
 * ============================================================================
 *
 * Yield may syntactically occur wherever the canonical statement dispatcher
 * permits `yieldStatement`.
 *
 * Semantic analysis must determine whether the containing control-flow
 * context permits suspension.
 *
 * Examples requiring semantic validation include:
 *
 *     if condition {
 *         yield value;
 *     }
 *
 *     while condition {
 *         yield value;
 *     }
 *
 *     match value {
 *         ...
 *     }
 *
 * The grammar does not attempt to encode control-flow-sensitive generator
 * validity.
 * ============================================================================
 */


/* ============================================================================
 * 11. YIELD AND RESOURCE / HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Nothing here requires:
 *
 *     a CPU;
 *     a GPU;
 *     an FPGA;
 *     an ASIC;
 *     a QPU;
 *     a cluster;
 *     a network;
 *     a particular memory size;
 *     a fixed number of workers.
 *
 * A generator can therefore be lowered differently for different targets while
 * preserving its source semantics.
 * ============================================================================
 */


/* ============================================================================
 * 12. QUANTUM GENERATORS
 * ============================================================================
 *
 * Quantum generator semantics are downstream.
 *
 * The following is syntactically possible if `measurement` or `state` is a
 * valid Zamani expression:
 *
 *     yield measurement;
 *
 *     yield state;
 *
 * No quantum-specific token is introduced here.
 *
 * No physical qubit is referenced here.
 *
 * No quantum register size is imposed here.
 *
 * No backend topology is encoded here.
 *
 * Any resulting quantum computation is lowered through the canonical quantum
 * semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 */


/* ============================================================================
 * 13. DISTRIBUTED GENERATORS
 * ============================================================================
 *
 * Distributed generator behavior is semantic/runtime territory.
 *
 * The grammar does not introduce:
 *
 *     node;
 *     worker;
 *     shard;
 *     replica;
 *     partition;
 *
 * into generator syntax.
 *
 * Such concepts belong to the distributed/resource/execution grammars where
 * their ownership can be defined independently.
 * ============================================================================
 */


/* ============================================================================
 * 14. CANCELLATION
 * ============================================================================
 *
 * Cancellation is not generator syntax.
 *
 * A generator may eventually participate in:
 *
 *     cancellation;
 *     timeout;
 *     shutdown;
 *     backpressure;
 *     resource revocation.
 *
 * Those are concurrency/runtime semantics.
 *
 * This grammar must not invent a generator-specific cancellation keyword.
 * ============================================================================
 */


/* ============================================================================
 * 15. CHECKPOINTING / RESUMPTION
 * ============================================================================
 *
 * A yield is a source-level suspension point.
 *
 * It is NOT automatically a durable checkpoint.
 *
 * Semantic/runtime layers must distinguish:
 *
 *     - ordinary suspension;
 *     - resumable execution;
 *     - restartable execution;
 *     - durable checkpoint;
 *     - serializable generator state;
 *     - reconstructible state;
 *     - provider/runtime-specific state.
 *
 * In particular, a generator yield must never be interpreted as permission to
 * serialize arbitrary quantum state.
 *
 * ============================================================================
 */


/* ============================================================================
 * 16. DETERMINISM
 * ============================================================================
 *
 * Parsing the same source with the same language version and canonical token
 * vocabulary must produce the same parse structure.
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no target-language actions;
 *     - no runtime lookups;
 *     - no hardware queries;
 *     - no random behavior.
 *
 * Therefore generator syntax remains deterministic.
 * ============================================================================
 */


/* ============================================================================
 * 17. ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples:
 *
 *     yield
 *
 * is syntactically valid.
 *
 * Examples of semantic errors include:
 *
 *     yield value
 *
 * inside a context where yielding is forbidden.
 *
 * Other semantic errors include:
 *
 *     - incompatible yield types;
 *     - yield from a non-generator context;
 *     - illegal borrow across suspension;
 *     - illegal resource lifetime across suspension;
 *     - invalid async-generator combination;
 *     - unsupported checkpoint semantics.
 *
 * These MUST NOT be converted into grammar-level hardware restrictions.
 * ============================================================================
 */


/* ============================================================================
 * 18. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve at least:
 *
 *     YieldStatement
 *         value: Option<Expression>
 *         source_span
 *
 * The grammar itself does not construct the AST.
 *
 * AST construction belongs to the parser/frontend adapter.
 *
 * The AST must preserve the distinction between:
 *
 *     yield;
 *
 * and:
 *
 *     yield expression;
 *
 * without prematurely deciding the final generator type.
 * ============================================================================
 */


/* ============================================================================
 * 19. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis should derive a generator model from the containing
 * function and its yield points.
 *
 * Required semantic checks include:
 *
 *     1. A yield must occur in a generator-compatible function context.
 *
 *     2. Yield expressions must be type checked.
 *
 *     3. All reachable yield values must satisfy the generator's inferred or
 *        declared yield contract.
 *
 *     4. Empty yields must be validated against the language's unit/control
 *        semantics.
 *
 *     5. Suspension must interact correctly with ownership and borrowing.
 *
 *     6. Suspension must interact correctly with effects.
 *
 *     7. Suspension must interact correctly with resource lifetimes.
 *
 *     8. Async generators must satisfy async/concurrency rules.
 *
 *     9. Quantum values must retain their semantic identity and must not be
 *        lowered into generator-specific fake quantum objects.
 *
 *    10. Generator semantics must be preserved through IR lowering.
 *
 * ============================================================================
 */


/* ============================================================================
 * 20. IR CONTRACT
 * ============================================================================
 *
 * The grammar MUST NOT define a generator IR.
 *
 * Instead:
 *
 *     parser
 *         ->
 *     frontend AST
 *         ->
 *     semantic generator model
 *         ->
 *     canonical compiler IR
 *
 * The exact IR representation may include concepts such as:
 *
 *     suspension point;
 *     resume edge;
 *     produced value;
 *     generator state;
 *     control-flow continuation.
 *
 * Those belong to the compiler/semantic IR architecture, not this grammar.
 *
 * For quantum programs:
 *
 *     generator syntax
 *         ->
 *     AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *
 * where applicable.
 *
 * ============================================================================
 */


/* ============================================================================
 * 21. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may lower generator semantics to:
 *
 *     - stack state;
 *     - heap state;
 *     - compiler-generated frames;
 *     - resumable tasks;
 *     - async executors;
 *     - distributed execution;
 *     - target-specific mechanisms.
 *
 * None of those choices are encoded here.
 *
 * ============================================================================
 */


/* ============================================================================
 * 22. TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling should use the parser/AST representation rather than searching raw
 * source text for "yield".
 *
 * Required tooling compatibility includes:
 *
 *     - formatter;
 *     - syntax highlighter;
 *     - IDE parser;
 *     - diagnostics;
 *     - source navigation;
 *     - AST inspection;
 *     - documentation generation;
 *     - semantic indexing;
 *     - refactoring.
 *
 * Source spans must be retained for:
 *
 *     K_YIELD
 *     yield value
 *     complete yield statement
 *
 * ============================================================================
 */


/* ============================================================================
 * 23. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing Zamani source using:
 *
 *     yield
 *     yield expression
 *
 * must remain parseable unless a documented language-version policy explicitly
 * changes the construct.
 *
 * The legacy grammar currently defines:
 *
 *     yieldStatement
 *         : YIELD expression? SEMI?
 *         ;
 *
 * The modular grammar replaces that ownership with:
 *
 *     yieldStatement
 *         : K_YIELD yieldValue? SEMI?
 *         ;
 *
 * The semantic meaning remains compatible.
 *
 * ============================================================================
 */


/* ============================================================================
 * 24. MIGRATION FROM LEGACY GRAMMAR
 * ============================================================================
 *
 * The legacy:
 *
 *     grammar/antlr/Core.g4
 *
 * currently owns a yield statement.
 *
 * During modular grammar migration:
 *
 *     Core.g4
 *
 * MUST NOT retain a second independent `yieldStatement` definition alongside
 * this grammar.
 *
 * The canonical modular owner becomes:
 *
 *     grammar/functions/generators.g4
 *
 * The root statement dispatcher should reference the canonical rule supplied
 * by this grammar.
 *
 * This prevents two competing definitions of `yield`.
 *
 * ============================================================================
 */


/* ============================================================================
 * 25. NO GENERATOR KEYWORD
 * ============================================================================
 *
 * The canonical lexer currently provides:
 *
 *     K_YIELD
 *
 * but does not provide:
 *
 *     K_GENERATOR
 *
 * This file therefore deliberately does not reference a nonexistent token.
 *
 * Adding a generator keyword later would require coordinated updates to:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/specification/*
 *     grammar/functions/generators.g4
 *     grammar/functions/functions.g4
 *     AST
 *     semantic analysis
 *     compatibility tests
 *     documentation
 *
 * It must not be introduced implicitly here.
 *
 * ============================================================================
 */


/* ============================================================================
 * 26. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_YIELDS
 *     MAX_GENERATORS
 *     MAX_GENERATOR_DEPTH
 *     MAX_GENERATOR_VALUES
 *     MAX_RESUMPTIONS
 *     MAX_THREADS
 *     MAX_TASKS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_MEMORY
 *
 * Repetition is represented through ANTLR grammar composition and is therefore
 * bounded only by implementation/parser resource policies.
 *
 * ============================================================================
 */


/* ============================================================================
 * 27. SECURITY AUDIT
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no network access;
 *     - accesses no filesystem;
 *     - executes no user code;
 *     - invokes no runtime services;
 *     - contains no target-language actions;
 *     - contains no unsafe operations.
 *
 * Semantic/runtime layers must separately validate resource and security
 * effects of generator execution.
 *
 * ============================================================================
 */


/* ============================================================================
 * 28. PRODUCTION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] `parser grammar Generators;` compiles with the canonical token
 *         vocabulary.
 *
 *     [ ] No lexer token is duplicated here.
 *
 *     [ ] `yieldStatement` has exactly one canonical owner.
 *
 *     [ ] `yieldValue` delegates to the canonical expression grammar.
 *
 *     [ ] No function declaration grammar is duplicated.
 *
 *     [ ] No function type grammar is duplicated.
 *
 *     [ ] No block grammar is duplicated.
 *
 *     [ ] No semantic generator state machine is encoded.
 *
 *     [ ] No hardware/resource limits are encoded.
 *
 *     [ ] Existing `yield` source remains compatible.
 *
 *     [ ] The legacy Core grammar is migrated so that it does not define a
 *         competing yield rule.
 *
 *     [ ] AST construction has a stable YieldStatement boundary.
 *
 *     [ ] Semantic analysis can classify containing functions as generators.
 *
 *     [ ] Async + yield can be handled by the semantic layer.
 *
 *     [ ] Quantum values remain governed by the canonical quantum semantics
 *         and `quantum::ir`.
 *
 *     [ ] Parser behavior is deterministic.
 *
 *     [ ] Rust integration remains compatible with Rust 1.97/1.97.1 and
 *         safe-Rust-only policy.
 *
 * ============================================================================
 */