/*
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
 *     Canonical generator parser delegate.
 *
 * Purpose:
 *     Defines the source-level syntax for generator suspension/value
 *     production. Generator declaration structure remains owned by the
 *     canonical function grammar.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust, actions, predicates,
 *     filesystem access, networking, process execution, or hardware access.
 *     The Zamani compiler/frontend/runtime implementation must use safe Rust.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - yieldStatement;
 *   - the optional value attached to a yield;
 *   - generator suspension syntax;
 *   - the parser-level boundary used by generator-aware AST construction.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - function declarations;
 *   - function names;
 *   - parameters;
 *   - generic parameters;
 *   - return-type declarations;
 *   - return statements;
 *   - expressions;
 *   - expression precedence;
 *   - blocks;
 *   - loops;
 *   - async declarations;
 *   - await expressions;
 *   - generator types;
 *   - stream types;
 *   - futures;
 *   - tasks;
 *   - executors;
 *   - scheduling;
 *   - cancellation;
 *   - checkpoint implementation;
 *   - memory layout;
 *   - state-machine lowering;
 *   - ABI;
 *   - hardware;
 *   - resources;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - optimization;
 *   - runtime behavior.
 *
 * ============================================================================
 * CANONICAL LEXICAL CONTRACT
 * ============================================================================
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which composes:
 *
 *     grammar/lexer/tokens.g4
 *
 * Parser grammars therefore consume:
 *
 *     tokenVocab = ZamaniLexer
 *
 * This file deliberately does NOT use:
 *
 *     tokenVocab = ZamaniTokens
 *
 * directly.
 *
 * The canonical yield token is:
 *
 *     YIELD
 *
 * The canonical statement terminator is:
 *
 *     SEMICOLON
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * Generator-ness is a semantic property of a callable containing one or more
 * valid yield statements.
 *
 * This file MUST NOT introduce:
 *
 *     generator fn ...
 *     gen fn ...
 *     generator<T>
 *     stream<T>
 *
 * merely to identify generators.
 *
 * The canonical function declaration remains owned by:
 *
 *     grammar/functions/functions.g4
 *
 * A function containing yield is classified downstream as a generator.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * A yielded value is exactly one canonical Zamani expression.
 *
 * This file MUST NOT redefine:
 *
 *     expression
 *     assignment expressions
 *     binary expressions
 *     unary expressions
 *     calls
 *     indexing
 *     member access
 *     quantum expressions
 *     classical expressions
 *     HDL expressions
 *     hardware expressions
 *     resource expressions
 *
 * The canonical expression grammar owns those constructs.
 *
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 *
 * The canonical statement-composition grammar must admit:
 *
 *     yieldStatement
 *
 * exactly once.
 *
 * The authoritative composition should therefore be conceptually:
 *
 *     statement
 *         : ...
 *         | yieldStatement
 *         | ...
 *         ;
 *
 * The following files MUST NOT remain competing authoritative owners of
 * yieldStatement:
 *
 *     grammar/antlr/Core.g4
 *     grammar/core/compilation-unit.g4
 *     any legacy monolithic parser grammar
 *
 * They may be retained temporarily for migration/documentation purposes, but
 * the production parser must have exactly one yieldStatement rule.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing answers only:
 *
 *     "Is this syntactically a yield statement?"
 *
 * Semantic analysis determines:
 *
 *   - whether yield is legal in the enclosing callable;
 *   - whether the callable is a generator;
 *   - the yielded value's type;
 *   - consistency of yielded types;
 *   - generator result semantics;
 *   - control-flow legality;
 *   - ownership/borrowing across suspension;
 *   - lifetime validity across suspension;
 *   - effect/capability requirements;
 *   - cancellation behavior;
 *   - checkpointability;
 *   - serialization legality;
 *   - distribution legality;
 *   - runtime compatibility.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * GENERATOR RETURN SEMANTICS
 * ============================================================================
 *
 * `yield` and `return` are deliberately separate constructs.
 *
 *     yield value;
 *
 * suspends execution and produces a generator value.
 *
 *     return value;
 *
 * terminates the enclosing callable according to the ordinary return
 * semantics.
 *
 * The exact relationship between:
 *
 *     yielded value type
 *
 * and:
 *
 *     final generator result type
 *
 * belongs to the semantic/type system.
 *
 * This grammar does not introduce a generator-specific return type syntax.
 *
 * ============================================================================
 * ASYNC INTEGRATION
 * ============================================================================
 *
 * Async function declaration syntax is owned by:
 *
 *     grammar/functions/async.g4
 *
 * and:
 *
 *     grammar/functions/functions.g4
 *
 * This file only supplies yield syntax.
 *
 * Therefore:
 *
 *     async fn values() {
 *         yield value;
 *     }
 *
 * may be parsed structurally as an async function containing yield.
 *
 * Semantic analysis determines whether the combination is a valid asynchronous
 * generator.
 *
 * This grammar does NOT introduce:
 *
 *     asyncGenerator
 *     async_generator
 *     generator_async
 *
 * as new language constructs.
 *
 * ============================================================================
 * `yield from`
 * ============================================================================
 *
 * The canonical lexer contains `FROM`, but the current language contract does
 * not establish `yield from` as generator syntax.
 *
 * Therefore this grammar intentionally does NOT accept:
 *
 *     yield from expression;
 *
 * as a special construct.
 *
 * If `yield from` is added in a future language version, it must receive:
 *
 *   - language-specification approval;
 *   - lexical/parser contract;
 *   - AST representation;
 *   - semantic rules;
 *   - generator composition rules;
 *   - diagnostics;
 *   - compatibility policy;
 *   - tests.
 *
 * It must then be integrated without changing the ownership of ordinary yield.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level finite limits on:
 *
 *   - number of generators;
 *   - number of yield statements;
 *   - number of suspension points;
 *   - number of generator calls;
 *   - number of generator compositions;
 *   - number of source declarations;
 *   - number of yielded values;
 *   - yielded-value type complexity;
 *   - generator nesting.
 *
 * Repetition is represented by the surrounding grammar and ordinary recursive
 * language structures.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_GENERATORS
 *     MAX_YIELDS
 *     MAX_SUSPENSIONS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *
 * Practical parser/compiler/runtime resource exhaustion remains an
 * implementation concern and must never be presented as a language semantic
 * limit.
 *
 * ============================================================================
 * RESOURCE / HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Generator syntax describes portable computation.
 *
 * It does NOT select:
 *
 *   - CPU;
 *   - GPU;
 *   - FPGA;
 *   - ASIC;
 *   - QPU;
 *   - device;
 *   - worker;
 *   - thread;
 *   - executor;
 *   - queue;
 *   - memory bank;
 *   - physical qubit;
 *   - network node;
 *   - topology.
 *
 * A generator may later be lowered to any execution strategy supported by the
 * semantic/compiler/runtime stack.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A generator may yield a value whose semantic type is quantum-related:
 *
 *     yield measurement;
 *     yield state;
 *     yield result;
 *
 * The generator grammar does not define the quantum semantics.
 *
 * Quantum meaning continues through:
 *
 *     frontend AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     QEC / resilience
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *         ->
 *     target realization
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HYBRID / DISTRIBUTED / AI INTEGRATION
 * ============================================================================
 *
 * A yielded expression may semantically represent:
 *
 *   - classical data;
 *   - tensors;
 *   - AI/ML results;
 *   - quantum measurements;
 *   - hardware events;
 *   - HDL-facing values;
 *   - accelerator results;
 *   - distributed messages;
 *   - networking values;
 *   - security/cryptographic results;
 *   - future domain values.
 *
 * No domain-specific yield syntax is required.
 *
 * Domain-specific meaning belongs to the corresponding type/semantic subsystem.
 *
 * ============================================================================
 * STATE-MACHINE / RUNTIME BOUNDARY
 * ============================================================================
 *
 * A compiler may lower:
 *
 *     function containing yield
 *
 * into:
 *
 *     generator semantic model
 *         ->
 *     suspension/resumption representation
 *         ->
 *     compiler-generated state machine
 *         ->
 *     target implementation
 *         ->
 *     runtime execution
 *
 * The grammar MUST NOT encode:
 *
 *   - program counters;
 *   - hidden frame fields;
 *   - stack layout;
 *   - heap layout;
 *   - polling;
 *   - scheduling;
 *   - worker assignment;
 *   - executor implementation;
 *   - queue implementation;
 *   - allocation strategy.
 *
 * ============================================================================
 * CHECKPOINT / SERIALIZATION BOUNDARY
 * ============================================================================
 *
 * A yield is a suspension point.
 *
 * It is NOT automatically:
 *
 *   - a durable checkpoint;
 *   - a serialization boundary;
 *   - a restart point;
 *   - a migration point.
 *
 * Those meanings require semantic/runtime contracts.
 *
 * In particular, yield MUST NOT implicitly authorize serialization of arbitrary
 * quantum state, hardware state, borrowed state, or privileged runtime state.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *   - no semantic predicates;
 *   - no embedded target-language actions;
 *   - no runtime queries;
 *   - no hardware queries;
 *   - no random behavior.
 *
 * Identical source, language version, lexer configuration, and parser
 * configuration therefore produce the same generator parse structure.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors are parser/frontend concerns.
 *
 * Examples:
 *
 *     yield value;
 *
 *     yield;
 *
 * are syntactically valid.
 *
 * Examples such as:
 *
 *     yield ;
 *
 * are also structurally represented as an empty yield and are semantically
 * validated according to the language specification.
 *
 * Semantic errors include:
 *
 *   - yield outside a generator-capable context;
 *   - invalid yielded type;
 *   - incompatible yielded types;
 *   - illegal borrow across suspension;
 *   - invalid resource lifetime;
 *   - unsupported async-generator combination;
 *   - unsupported generator checkpointing.
 *
 * These are NOT grammar-level hardware errors.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser/frontend AST adapter should map:
 *
 *     yieldStatement
 *
 * to the domain-neutral AST representation for a yield/suspension statement.
 *
 * Conceptually:
 *
 *     YieldStatement {
 *         value: Option<Expression>,
 *         source_span: SourceSpan
 *     }
 *
 * The exact Rust AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar must not define a second generator AST.
 *
 * The AST must preserve:
 *
 *   - whether a yield has a value;
 *   - the complete yielded expression;
 *   - source span;
 *   - child-expression spans.
 *
 * It must not prematurely decide:
 *
 *   - generator type;
 *   - runtime frame representation;
 *   - executor;
 *   - hardware;
 *   - quantum physical mapping.
 *
 * ============================================================================
 * SEMANTIC / IR CONTRACT
 * ============================================================================
 *
 * The semantic layer consumes the AST yield node.
 *
 * Generator semantics remain distinct from the ordinary expression value.
 *
 * A yielded quantum value is lowered through the existing semantic quantum
 * pipeline and ultimately through canonical:
 *
 *     quantum::ir
 *
 * A yielded classical/hybrid/HDL/data/distributed value is lowered through the
 * appropriate canonical semantic representation.
 *
 * Generator suspension itself is represented by the compiler's generator
 * semantic/lowering machinery rather than by a new grammar-level IR.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler must be able to lower generator suspension without requiring
 * source changes when the target changes.
 *
 * Target-specific decisions may include:
 *
 *   - state-machine representation;
 *   - stack/heap strategy;
 *   - register allocation;
 *   - scheduling;
 *   - placement;
 *   - parallelization;
 *   - accelerator mapping;
 *   - distributed execution;
 *   - quantum routing/scheduling;
 *   - runtime integration.
 *
 * These decisions occur after parsing and semantic analysis.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime support may provide:
 *
 *   - generator creation;
 *   - suspension;
 *   - resumption;
 *   - completion;
 *   - cancellation;
 *   - resource management;
 *   - checkpointing;
 *   - distributed execution;
 *   - observability.
 *
 * None of these are encoded by this grammar.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Formatters, language servers, syntax highlighters, source analyzers, and
 * refactoring tools should identify `yieldStatement` structurally.
 *
 * They must not search source text for `"yield"` as their authoritative
 * representation.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     yield;
 *     yield expression;
 *
 * Optional statement termination follows the repository's existing statement
 * grammar policy.
 *
 * The current canonical lexer spelling is:
 *
 *     yield
 *
 * represented by:
 *
 *     YIELD
 *
 * The current canonical terminator is:
 *
 *     ;
 *
 * represented by:
 *
 *     SEMICOLON
 *
 * No compatibility alias such as `K_YIELD` or `SEMI` is introduced here.
 *
 * Those names belong to an older/different lexical contract and must not be
 * reintroduced merely to make this file compile.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax:
 *
 *     yield;
 *     yield value;
 *     yield expression;
 *     yield call();
 *     yield a + b;
 *     yield measurement;
 *     yield quantum_result;
 *     yield tensor;
 *     yield hardware_event;
 *
 * Nested/control-flow contexts:
 *
 *     if condition {
 *         yield value;
 *     }
 *
 *     while condition {
 *         yield value;
 *     }
 *
 *     for value in source {
 *         yield value;
 *     }
 *
 * Async:
 *
 *     async fn values() {
 *         yield value;
 *     }
 *
 * Domain-neutral:
 *
 *     fn classical() {
 *         yield scalar;
 *     }
 *
 *     fn quantum() {
 *         yield measurement;
 *     }
 *
 *     fn hybrid() {
 *         yield result;
 *     }
 *
 * Negative/semantic fixtures:
 *
 *     - yield in a context where generators are forbidden;
 *     - invalid yielded expression;
 *     - invalid ownership across suspension;
 *     - invalid lifetime across suspension;
 *     - invalid async-generator combination;
 *     - invalid generator return semantics.
 *
 * These semantic cases must be tested outside the grammar itself.
 *
 * Boundary/scalability:
 *
 *     - many yield statements;
 *     - deeply nested control flow;
 *     - large yielded expressions;
 *     - large generic types used by yielded expressions;
 *     - large source programs;
 *     - many generator functions;
 *     - cross-domain generator programs.
 *
 * No fixture may define an artificial maximum as the language limit.
 *
 * Determinism:
 *
 * Identical source must produce equivalent parse trees across repeated runs.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Allowed fixed syntax:
 *
 *     YIELD
 *     SEMICOLON
 *
 * Forbidden hardware/resource assumptions:
 *
 *     CPU counts
 *     GPU counts
 *     FPGA counts
 *     QPU counts
 *     qubit counts
 *     memory sizes
 *     worker counts
 *     thread counts
 *     node counts
 *     device identifiers
 *     topology
 *     vendor names
 *     backend names
 *
 * This file contains no such assumptions.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   1. `yieldStatement` is the sole authoritative yield-statement rule.
 *
 *   2. It consumes canonical `YIELD`.
 *
 *   3. It consumes canonical `SEMICOLON`.
 *
 *   4. It delegates yielded values to canonical `expression`.
 *
 *   5. It contains no lexer rules.
 *
 *   6. It contains no function-declaration grammar.
 *
 *   7. It contains no return grammar.
 *
 *   8. It contains no duplicate generator type grammar.
 *
 *   9. It contains no runtime/compiler implementation.
 *
 *  10. It contains no machine-specific limits.
 *
 *  11. It is integrated exactly once into the canonical statement dispatcher.
 *
 *  12. The legacy competing yield rules are removed from the authoritative
 *      parser path.
 *
 *  13. AST mapping is stable and source-span preserving.
 *
 *  14. Semantic generator classification occurs downstream.
 *
 *  15. Quantum values continue through the canonical `quantum::ir` boundary.
 *
 *  16. Classical, quantum, HDL, hybrid, distributed, AI/data, and future
 *      semantic values require no generator-specific grammar variants.
 *
 *  17. Positive, negative, boundary, scalability, determinism, and
 *      cross-domain tests exist.
 *
 *  18. The implementation remains compatible with Rust 1.97 / 1.97.1 and
 *      requires no unsafe Rust.
 *
 * ============================================================================
 * CANONICAL RULES
 * ============================================================================
 *
 * The complete authoritative syntax in this file is intentionally small:
 *
 *     yieldStatement
 *         : YIELD expression? SEMICOLON?
 *         ;
 *
 * No other generator-specific parser rule is required at this layer.
 *
 * ============================================================================
 */

parser grammar Generators;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * GENERATOR SUSPENSION / VALUE PRODUCTION
 * ============================================================================
 *
 * The expression is optional so that the grammar preserves both:
 *
 *     yield;
 *
 * and:
 *
 *     yield expression;
 *
 * Semantic analysis determines whether an empty yield is valid in a given
 * language context and what its value/type semantics are.
 *
 * The grammar deliberately does not introduce a generator-specific expression
 * hierarchy.
 */
yieldStatement
    : YIELD
      expression?
      SEMICOLON?
    ;