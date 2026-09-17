/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/calls.g4
 *
 * Status:
 *     Canonical production call-expression component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition.
 *
 * Safety:
 *     - This file contains grammar only.
 *     - No embedded Rust actions.
 *     - No embedded Rust predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No process execution.
 *     - No hardware discovery.
 *     - No runtime execution.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of callable invocation.
 *
 * It is deliberately independent from:
 *
 *     - callable name resolution;
 *     - overload resolution;
 *     - type checking;
 *     - generic inference;
 *     - ownership;
 *     - effects;
 *     - capabilities;
 *     - resource allocation;
 *     - hardware selection;
 *     - quantum operation semantics;
 *     - runtime dispatch.
 *
 * The public integration boundary is:
 *
 *     postfixExpression
 *          |
 *          +--> callSuffix
 *
 * Therefore this file MUST NOT define another postfix-expression hierarchy.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     postfixExpression
 *          |
 *          +--> callSuffix
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> callable resolution
 *          +--> generic inference
 *          +--> overload resolution
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource checking
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--------------------+----------------------+
 *          |                    |                      |
 *          v                    v                      v
 *     classical IR        quantum::ir          HDL/hardware IR
 *          |                    |                      |
 *          +--------------------+----------------------+
 *                               |
 *                               v
 *                     optimization / lowering
 *                               |
 *                     routing / scheduling
 *                               |
 *                     resilience / QEC / ZQN
 *                               |
 *                              HAL
 *                               |
 *                       target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - call suffix syntax;
 *     - invocation parentheses;
 *     - argument-list syntax;
 *     - positional arguments;
 *     - named arguments;
 *     - spread arguments;
 *     - trailing-comma syntax;
 *     - explicit call-site generic arguments;
 *     - source ordering of arguments;
 *     - compatibility aliases for call-argument concepts.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified-name resolution;
 *     - member resolution;
 *     - function declarations;
 *     - method declarations;
 *     - closure declarations;
 *     - lambda declarations;
 *     - function types;
 *     - type definitions;
 *     - type-expression semantics;
 *     - overload resolution;
 *     - generic inference;
 *     - arity validation;
 *     - parameter matching;
 *     - default-argument evaluation;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - hardware;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - runtime execution;
 *     - ABI selection.
 *
 * ============================================================================
 * MODULAR COMPOSITION CONTRACT
 * ============================================================================
 *
 * `postfix.g4` owns:
 *
 *     postfixExpression
 *     postfixPart
 *
 * This file owns:
 *
 *     callSuffix
 *     callTypeArguments
 *     callTypeArgumentList
 *     argumentList
 *     argument
 *
 * The canonical relationship is:
 *
 *     postfixExpression
 *         : primaryExpression postfixPart*
 *         ;
 *
 *     postfixPart
 *         : callSuffix
 *         | indexingSuffix
 *         | memberSuffix
 *         | ...
 *         ;
 *
 * This file MUST NOT define:
 *
 *     postfixExpression
 *
 * and MUST NOT import `postfix.g4`.
 *
 * This prevents:
 *
 *     Calls -> Postfix -> Calls
 *
 * dependency cycles.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The current modular parser ecosystem uses the canonical Zamani lexer
 * vocabulary and existing expression/type grammars consume its token names.
 *
 * This grammar therefore consumes lexer tokens but defines none.
 *
 * Required token categories include:
 *
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     COLON
 *     ASSIGN
 *     ELLIPSIS
 *     DOUBLE_COLON
 *     LESS_THAN
 *     GREATER_THAN
 *
 * The lexer owns their textual spelling.
 *
 * IMPORTANT:
 *
 *     No uppercase lexer rule is declared in this parser grammar.
 *
 * If the repository's lexer-composition migration changes the concrete
 * token-vocabulary filename, that change belongs to the parser composition
 * layer and lexer contract, not to call semantics.
 *
 * ============================================================================
 * OPEN-WORLD CALLABLE MODEL
 * ============================================================================
 *
 * The grammar does not maintain a finite list of callable names.
 *
 * A callable can semantically represent:
 *
 *     function
 *     method
 *     closure
 *     lambda
 *     function value
 *     returned callable
 *     generic callable
 *     trait/interface callable
 *     capability-provided operation
 *     accelerator abstraction
 *     quantum operation
 *     distributed service
 *     hardware abstraction
 *     future computational construct
 *
 * Consequently:
 *
 *     H(q)
 *     custom_operation(q)
 *     accelerator.run(data)
 *     service.request(data)
 *
 * all use the same call syntax.
 *
 * The grammar does not enumerate operations.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A call expresses:
 *
 *     invoke a source-level callable with source-level arguments.
 *
 * It MUST NOT encode a fixed physical implementation such as:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     physical qubit 17
 *     node 3
 *     accelerator 2
 *     memory bank 1
 *
 * Such decisions belong downstream to:
 *
 *     semantic analysis
 *     capability resolution
 *     resource resolution
 *     compilation
 *     placement
 *     routing
 *     scheduling
 *     deployment
 *     runtime dispatch
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately contains NO language-level constants for:
 *
 *     maximum arguments
 *     maximum parameters
 *     maximum generic arguments
 *     maximum call depth
 *     maximum callable count
 *     maximum nesting
 *     maximum devices
 *     maximum qubits
 *     maximum CPUs
 *     maximum GPUs
 *     maximum nodes
 *     maximum accelerators
 *
 * Repetition is represented structurally:
 *
 *     (...)* 
 *     (...)? 
 *
 * Therefore the grammar introduces no artificial finite semantic ceiling.
 *
 * "Infinity" means:
 *
 *     the language grammar imposes no arbitrary finite limit.
 *
 * Actual parser/compiler/runtime limits are operational resource limits and
 * MUST NOT be promoted into language semantics.
 *
 * ============================================================================
 * ARGUMENT ORDER
 * ============================================================================
 *
 * The parser preserves the exact source order of arguments.
 *
 * Example:
 *
 *     f(a, b, c)
 *
 * produces an ordered argument sequence:
 *
 *     a -> b -> c
 *
 * This file does not decide whether an implementation evaluates arguments:
 *
 *     eagerly
 *     lazily
 *     sequentially
 *     concurrently
 *     speculatively
 *
 * Evaluation semantics belong downstream.
 *
 * ============================================================================
 * NAMED ARGUMENTS
 * ============================================================================
 *
 * The existing repository contains both:
 *
 *     name = expression
 *
 * and:
 *
 *     name: expression
 *
 * forms.
 *
 * This grammar accepts both at the syntax level to avoid silently deleting
 * existing source forms during modularization.
 *
 * Semantic/version/compatibility validation decides:
 *
 *     - whether both forms remain stable;
 *     - whether one is deprecated;
 *     - whether the two forms have identical semantics;
 *     - whether migration diagnostics are required.
 *
 * The AST should preserve the delimiter kind when source-preserving tooling
 * or migration diagnostics require it.
 *
 * ============================================================================
 * SPREAD ARGUMENTS
 * ============================================================================
 *
 * Spread syntax:
 *
 *     f(...values)
 *
 * is syntactic expansion intent.
 *
 * The semantic layer determines whether `values` is:
 *
 *     iterable
 *     tuple-like
 *     parameter-pack
 *     operand-pack
 *     quantum operand collection
 *     distributed argument collection
 *     hardware/resource argument collection
 *
 * The grammar does not impose a maximum expansion size.
 *
 * ============================================================================
 * GENERIC CALLS
 * ============================================================================
 *
 * Explicit call-site generic arguments use the established Zamani form:
 *
 *     callable::<T>(value)
 *
 * and:
 *
 *     callable::<T, U>(value, other)
 *
 * The type subsystem owns `typeExpression`.
 *
 * This file only establishes where explicit call-site type arguments occur.
 *
 * The semantic layer determines whether the callable actually accepts them.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Calls may represent source-level quantum operations:
 *
 *     H(q)
 *     measure(q)
 *     reset(q)
 *     custom_operation(q0, q1)
 *     operation(theta)(q)
 *
 * The grammar MUST NOT enumerate:
 *
 *     X
 *     Y
 *     Z
 *     H
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     vendor gates
 *     hardware gates
 *
 * Those are identifiers or semantic operations unless explicitly reserved
 * elsewhere.
 *
 * Quantum lowering remains:
 *
 *     source call
 *         ->
 *     frontend semantic representation
 *         ->
 *     canonical quantum::ir
 *
 * This file MUST NOT introduce:
 *
 *     QuantumGate
 *     PhysicalQubitId
 *     topology
 *     calibration
 *     QEC implementation
 *     ZQN implementation
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same call syntax supports:
 *
 *     classical functions
 *     numerical kernels
 *     tensor operations
 *     HDL semantic operations
 *     hardware abstractions
 *     accelerator operations
 *     distributed services
 *     networking operations
 *     AI/ML operations
 *     cryptographic operations
 *     future domains
 *
 * Examples:
 *
 *     tensor.matmul(a, b)
 *     accelerator.run(kernel, data)
 *     device.configure(config)
 *     service.request(data)
 *     quantum.measure(q)
 *
 * None of these calls selects a concrete machine in this grammar.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a call MUST NEVER execute the call.
 *
 * The parser must not:
 *
 *     - invoke a function;
 *     - execute a command;
 *     - open a file;
 *     - contact a network;
 *     - load a plugin;
 *     - inspect hardware;
 *     - inspect credentials;
 *     - invoke a QPU;
 *     - invoke an FPGA;
 *     - invoke an HDL simulator;
 *     - invoke a compiler backend.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token sequence
 *     active grammar/version
 *
 * It must not depend on:
 *
 *     system time
 *     randomness
 *     environment variables
 *     filesystem contents
 *     network state
 *     installed devices
 *     hardware availability
 *     runtime scheduler state
 *     backend availability
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should conceptually preserve:
 *
 *     CallExpression {
 *         callee,
 *         generic_arguments?,
 *         arguments,
 *         source_span
 *     }
 *
 * Each argument should preserve its source-level category:
 *
 *     Positional(expression)
 *     Named(name, expression, delimiter)
 *     Spread(expression)
 *
 * The exact Rust AST types belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT invent a parallel AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - callable resolution;
 *     - name resolution;
 *     - member resolution;
 *     - overload resolution;
 *     - generic inference;
 *     - explicit generic argument validation;
 *     - arity validation;
 *     - named-parameter validation;
 *     - duplicate named-argument validation;
 *     - positional/named ordering rules;
 *     - spread validation;
 *     - type checking;
 *     - conversions;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - invocation semantics.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not directly create IR.
 *
 * Calls lower through the existing frontend/semantic architecture.
 *
 * For quantum:
 *
 *     call syntax
 *         ->
 *     semantic quantum operation
 *         ->
 *     quantum::ir
 *
 * For classical:
 *
 *     call syntax
 *         ->
 *     semantic callable
 *         ->
 *     classical/canonical IR
 *
 * For HDL/hardware:
 *
 *     call syntax
 *         ->
 *     semantic operation
 *         ->
 *     HDL/hardware representation
 *
 * No domain-specific call IR is introduced here.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may later:
 *
 *     inline
 *     specialize
 *     devirtualize
 *     vectorize
 *     distribute
 *     lower
 *     route
 *     schedule
 *     optimize
 *     map
 *
 * None of those transformations belong in this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * A parsed call is not an executed call.
 *
 * Runtime realization may use:
 *
 *     static dispatch
 *     dynamic dispatch
 *     capability negotiation
 *     resource negotiation
 *     service discovery
 *     hardware abstraction
 *     quantum backend selection
 *
 * without changing source syntax.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * These forms remain structurally representable:
 *
 *     f()
 *     f(x)
 *     f(x, y)
 *     f(x,)
 *     f(x, y,)
 *
 * Named:
 *
 *     f(name = value)
 *     f(name: value)
 *
 * Spread:
 *
 *     f(...values)
 *
 * Explicit generics:
 *
 *     f::<T>(x)
 *     f::<T, U>(x, y)
 *
 * Member calls are naturally represented by:
 *
 *     object.method(x)
 *
 * because member selection belongs to postfix composition and the following
 * `callSuffix` attaches to the selected callable.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics should identify:
 *
 *     - missing `(`;
 *     - missing `)`;
 *     - malformed argument;
 *     - missing argument expression;
 *     - misplaced comma;
 *     - malformed named argument;
 *     - malformed spread argument;
 *     - malformed explicit generic argument list;
 *     - missing `>`;
 *     - unexpected trailing tokens.
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     - unknown callable;
 *     - wrong arity;
 *     - unknown named parameter;
 *     - duplicate named parameter;
 *     - invalid spread;
 *     - invalid generic arguments;
 *     - type mismatch;
 *     - capability failure;
 *     - resource failure.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     f()
 *     f(x)
 *     f(x, y)
 *     f(x,)
 *     f(x, y,)
 *     f(name = value)
 *     f(name: value)
 *     f(...values)
 *     f::<T>(x)
 *     f::<T, U>(x, y)
 *
 * Nested:
 *
 *     f(g(x))
 *     f(a[i])
 *     f(object.field)
 *     f(object.method(x))
 *     f(f(g(h(x))))
 *
 * Mixed:
 *
 *     object.method::<T>(x)[i].field()
 *
 * Domain-neutral:
 *
 *     quantum.measure(q)
 *     accelerator.run(data)
 *     service.request(payload)
 *
 * Negative:
 *
 *     f(
 *     f(x
 *     f(,x)
 *     f(x,,y)
 *     f(name =)
 *     f(name:)
 *     f(... )
 *     f::<>(x)
 *     f::<T(x)
 *
 * Boundary:
 *
 *     zero arguments
 *     one argument
 *     many arguments
 *     many generic arguments
 *     deeply nested call chains
 *
 * Scalability:
 *
 *     no fixed argument count;
 *     no fixed generic arity;
 *     no fixed nesting depth;
 *     no fixed call depth;
 *     no fixed callable count.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden language-level limits:
 *
 *     MAX_ARGUMENTS
 *     MAX_PARAMETERS
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_CALL_DEPTH
 *     MAX_NESTING
 *     MAX_CALLABLES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_NODES
 *
 * None are represented by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] call suffix syntax is canonical;
 *     [ ] postfix.g4 consumes `callSuffix`;
 *     [ ] no Calls -> Postfix import exists;
 *     [ ] no Postfix -> Calls circular import exists;
 *     [ ] arguments preserve source order;
 *     [ ] positional arguments are supported;
 *     [ ] named arguments are supported;
 *     [ ] spread arguments are supported;
 *     [ ] trailing commas are supported;
 *     [ ] explicit call-site generic arguments are supported;
 *     [ ] `typeExpression` is delegated to the type subsystem;
 *     [ ] expression syntax is delegated to the expression subsystem;
 *     [ ] callable semantics are delegated to semantic analysis;
 *     [ ] no hardware limits are encoded;
 *     [ ] no quantum gate inventory is encoded;
 *     [ ] no quantum IR is introduced;
 *     [ ] no runtime execution is possible during parsing;
 *     [ ] AST mapping is documented;
 *     [ ] semantic mapping is documented;
 *     [ ] IR integration is documented;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] compatibility tests exist.
 *
 * ============================================================================
 */

parser grammar Calls;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. CANONICAL CALL SUFFIX
 * ========================================================================== */

/**
 * Invocation suffix consumed by the postfix-expression layer.
 *
 * Examples:
 *
 *     f()
 *     f(x)
 *     f(x, y)
 *     f::<T>(x)
 *
 * The callee itself is NOT owned here.
 *
 * `postfixExpression` supplies the callee/base expression and attaches this
 * suffix to it.
 */
callSuffix
    : callTypeArguments?
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 2. EXPLICIT CALL-SITE TYPE ARGUMENTS
 * ========================================================================== */

/**
 * Explicit generic invocation:
 *
 *     f::<T>(x)
 *     f::<T, U>(x, y)
 *
 * The `typeExpression` rule is supplied by the canonical type grammar.
 */
callTypeArguments
    : DOUBLE_COLON
      LESS_THAN
      callTypeArgumentList
      GREATER_THAN
    ;


/**
 * No fixed generic arity.
 *
 * The type subsystem owns the meaning of every type expression.
 */
callTypeArgumentList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      COMMA?
    ;


/* ============================================================================
 * 3. ARGUMENT LIST
 * ========================================================================== */

/**
 * Non-empty argument list.
 *
 * Empty calls use:
 *
 *     argumentList?
 *
 * in `callSuffix`.
 *
 * A trailing comma is intentionally accepted.
 */
argumentList
    : argument
      (
          COMMA
          argument
      )*
      COMMA?
    ;


/* ============================================================================
 * 4. ARGUMENT
 * ========================================================================== */

/**
 * Argument classification is syntactic.
 *
 * Semantic validation determines whether a particular combination is legal
 * for the selected callable.
 */
argument
    : namedArgument
    | spreadArgument
    | positionalArgument
    ;


/* ============================================================================
 * 5. POSITIONAL ARGUMENT
 * ========================================================================== */

/**
 * Ordinary source expression passed positionally.
 */
positionalArgument
    : expression
    ;


/* ============================================================================
 * 6. NAMED ARGUMENT
 * ========================================================================== */

/**
 * Compatibility forms:
 *
 *     f(name = value)
 *     f(name: value)
 *
 * The semantic/compatibility layer determines the canonical status of each
 * spelling.
 */
namedArgument
    : argumentName ASSIGN expression
    | argumentName COLON expression
    ;


/**
 * Argument names use the canonical identifier vocabulary.
 *
 * This grammar deliberately does not define identifier spelling.
 */
argumentName
    : IDENTIFIER
    ;


/* ============================================================================
 * 7. SPREAD ARGUMENT
 * ========================================================================== */

/**
 * Spread/expansion:
 *
 *     f(...values)
 *
 * Expansion semantics are downstream.
 */
spreadArgument
    : ELLIPSIS
      expression
    ;


/* ============================================================================
 * 8. COMPATIBILITY ALIASES
 * ========================================================================== */

/**
 * Compatibility alias for consumers that historically referred to call
 * arguments through a `callArguments` rule.
 *
 * The alias intentionally does not introduce another grammar.
 */
callArguments
    : LPAREN
      argumentList?
      RPAREN
    ;


/**
 * Compatibility alias for consumers that historically used `arguments`.
 *
 * The canonical public list rule remains `argumentList`.
 */
arguments
    : argumentList
    ;