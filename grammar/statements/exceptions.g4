/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/exceptions.g4
 *
 * STATUS
 * ------
 * CANONICAL EXCEPTION / EXCEPTIONAL-CONTROL-FLOW GRAMMAR COMPONENT
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * This grammar contains:
 *
 *   - no embedded Rust actions;
 *   - no semantic predicates;
 *   - no unsafe Rust;
 *   - no I/O;
 *   - no filesystem access;
 *   - no networking;
 *   - no process execution;
 *   - no hardware discovery;
 *   - no runtime execution;
 *   - no mutable global state;
 *   - no target-specific implementation;
 *   - no machine-size constants;
 *   - no resource-count limits.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE SYNTAX OWNER for Zamani source-level exception
 * constructs.
 *
 * It owns:
 *
 *     throwStatement
 *     tryCatchFinallyStatement
 *     catchClause
 *     catchBinding
 *     finallyClause
 *
 * It does NOT own the universal statement dispatcher.
 *
 * Statement composition is owned by:
 *
 *     grammar/statements/statements.g4
 *
 * Control-flow composition is owned by:
 *
 *     grammar/statements/control-flow.g4
 *
 * This file supplies concrete exception syntax to those composition layers.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * The following rules belong ONLY to this file:
 *
 *     throwStatement
 *     tryCatchFinallyStatement
 *     catchClause
 *     catchBinding
 *     finallyClause
 *
 * This file MUST NOT define:
 *
 *     statement
 *     controlFlowStatement
 *     tryStatement
 *     expression
 *     typeExpression
 *     identifier
 *     blockExpression
 *     statementTerminator
 *
 * Those rules belong to their canonical owners.
 *
 * ============================================================================
 * SHARED RULE CONTRACT
 * ============================================================================
 *
 * This grammar consumes the following composition-provided parser rules:
 *
 *     expression
 *         -> canonical expression grammar
 *
 *     typeExpression
 *         -> grammar/types/types.g4
 *
 *     identifier
 *         -> canonical name grammar
 *
 *     blockExpression
 *         -> grammar/statements/blocks.g4
 *
 *     statementTerminator
 *         -> canonical punctuation/statement grammar
 *
 * These rules are intentionally NOT duplicated here.
 *
 * The assembled production parser is responsible for making these canonical
 * rules visible to this delegate grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical ANTLR lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The parser consumes its vocabulary through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Required lexical tokens for this grammar include:
 *
 *     TRY
 *     CATCH
 *     FINALLY
 *     THROW
 *
 * together with the canonical punctuation tokens:
 *
 *     LPAREN
 *     RPAREN
 *     COLON
 *     SEMICOLON
 *     LBRACE
 *     RBRACE
 *
 * This file MUST NOT define lexer rules.
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
 *     exception parser rules              <-- THIS FILE
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
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> ownership / borrowing
 *          +--> control-flow analysis
 *          +--> resource analysis
 *          +--> exception-flow analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future-domain representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / scheduling / resilience where applicable
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * This file participates ONLY in the source syntax stage.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Exception control flow is a language-level construct.
 *
 * It must work uniformly around:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware/software co-design
 *     embedded computation
 *     accelerator computation
 *     distributed computation
 *     parallel computation
 *     HPC
 *     AI/ML
 *     data processing
 *     networking
 *     security
 *     scientific computation
 *     future computational domains
 *
 * This file MUST NOT define:
 *
 *     quantumTryStatement
 *     cpuTryStatement
 *     gpuTryStatement
 *     fpgaTryStatement
 *     qpuTryStatement
 *     distributedTryStatement
 *     aiTryStatement
 *
 * merely because the contained computation may execute on a particular target.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Exception syntax describes PROGRAM BEHAVIOR.
 *
 * It does not describe the physical machine that realizes that behavior.
 *
 * Consequently this grammar imposes no language-level limit on:
 *
 *     exception nesting
 *     handler count
 *     try-block size
 *     handler-block size
 *     program size
 *     task count
 *     thread count
 *     processor count
 *     node count
 *     device count
 *     accelerator count
 *     qubit count
 *     memory capacity
 *     network size
 *     hardware topology
 *
 * No constants such as the following may appear here:
 *
 *     MAX_CATCHES
 *     MAX_TRY_DEPTH
 *     MAX_HANDLERS
 *     MAX_EXCEPTION_SIZE
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * Practical parser/compiler resource limits, when necessary for host
 * protection, are implementation/resource-policy concerns rather than
 * language semantics.
 *
 * ============================================================================
 * EXCEPTION SEMANTICS BOUNDARY
 * ============================================================================
 *
 * This grammar recognizes source structure only.
 *
 * It does NOT determine:
 *
 *     - what constitutes a runtime exception object;
 *     - exception allocation;
 *     - stack unwinding;
 *     - process termination;
 *     - handler dispatch;
 *     - exception propagation;
 *     - retry policy;
 *     - recovery policy;
 *     - fault mitigation;
 *     - distributed failover;
 *     - hardware recovery;
 *     - quantum error correction;
 *     - ZQN noise modelling;
 *     - scheduling;
 *     - routing;
 *     - backend selection.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * CANONICAL PARSER COMPONENT
 * ============================================================================
 */

parser grammar ExceptionsParser;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * THROW STATEMENT
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     throw expression;
 *
 * Examples:
 *
 *     throw error;
 *     throw make_error();
 *     throw failure;
 *     throw create_error(code);
 *
 * The thrown value is deliberately represented by the canonical `expression`
 * rule rather than a special exception-expression hierarchy.
 *
 * Semantic analysis determines whether the resulting expression value is
 * throwable according to the active Zamani exception/effect model.
 *
 * This keeps exception syntax independent of:
 *
 *     - a fixed exception class hierarchy;
 *     - runtime representation;
 *     - operating system;
 *     - hardware;
 *     - domain;
 *     - backend.
 */
throwStatement
    : THROW expression statementTerminator
    ;


/*
 * ============================================================================
 * TRY / CATCH / FINALLY
 * ============================================================================
 *
 * Canonical forms:
 *
 *     try {
 *         body();
 *     } catch (error) {
 *         recover();
 *     }
 *
 *     try {
 *         body();
 *     } catch (error: ErrorType) {
 *         recover();
 *     }
 *
 *     try {
 *         body();
 *     } finally {
 *         cleanup();
 *     }
 *
 *     try {
 *         body();
 *     } catch (error) {
 *         recover();
 *     } finally {
 *         cleanup();
 *     }
 *
 * One or more catch clauses are permitted.
 *
 * A finally clause is optional when one or more catch clauses are present.
 *
 * A finally clause may also appear without a catch clause.
 *
 * Therefore the grammar accepts exactly:
 *
 *     TRY + CATCH+
 *     TRY + CATCH+ + FINALLY
 *     TRY + FINALLY
 *
 * and rejects:
 *
 *     TRY alone
 *
 * This rule deliberately contains no finite handler count.
 */
tryCatchFinallyStatement
    : TRY blockExpression
      (
          catchClause+
          finallyClause?
        | finallyClause
      )
    ;


/*
 * ============================================================================
 * CATCH CLAUSE
 * ============================================================================
 *
 * Canonical forms:
 *
 *     catch (error) {
 *         recover();
 *     }
 *
 *     catch (error: ErrorType) {
 *         recover();
 *     }
 *
 * A catch clause always has:
 *
 *     catch keyword
 *     parenthesized binding
 *     handler block
 *
 * The optional type annotation is syntactic.
 *
 * Semantic analysis determines:
 *
 *     - whether the type exists;
 *     - whether it is throwable;
 *     - whether it is compatible with the propagated exception;
 *     - whether the handler is unreachable;
 *     - whether the handler is redundant;
 *     - whether catch ordering is valid;
 *     - whether all required exception paths are handled.
 *
 * None of those checks belong in this grammar.
 */
catchClause
    : CATCH LPAREN catchBinding RPAREN blockExpression
    ;


/*
 * ============================================================================
 * CATCH BINDING
 * ============================================================================
 *
 * Canonical forms:
 *
 *     error
 *
 *     error: ErrorType
 *
 * The binding name is an ordinary canonical identifier.
 *
 * The type is the canonical `typeExpression`.
 *
 * IMPORTANT:
 *
 *     typeExpression
 *
 * is the repository's canonical type entry point.
 *
 * This grammar deliberately does NOT use a competing `typeExpr` rule.
 *
 * Scope, ownership, mutability, lifetime and type validity are semantic
 * concerns.
 */
catchBinding
    : identifier
      (COLON typeExpression)?
    ;


/*
 * ============================================================================
 * FINALLY CLAUSE
 * ============================================================================
 *
 * Canonical form:
 *
 *     finally {
 *         cleanup();
 *     }
 *
 * The cleanup region is always a complete canonical block.
 *
 * `finally;` is therefore invalid.
 *
 * The grammar does not decide whether the finally block:
 *
 *     - terminates control flow;
 *     - throws;
 *     - returns;
 *     - performs cleanup;
 *     - releases resources;
 *     - invokes domain operations;
 *     - performs hardware operations.
 *
 * Those are semantic/runtime concerns.
 */
finallyClause
    : FINALLY blockExpression
    ;


/*
 * ============================================================================
 * CONTROL-FLOW INTEGRATION
 * ============================================================================
 *
 * grammar/statements/control-flow.g4 owns the control-flow composition rule.
 *
 * Its stable adapter is:
 *
 *     tryStatement
 *         : tryCatchFinallyStatement
 *         ;
 *
 * The control-flow composition layer also admits:
 *
 *     throwStatement
 *
 * directly.
 *
 * Therefore this file MUST NOT define:
 *
 *     controlFlowStatement
 *     tryStatement
 *     statement
 *
 * The intended relationship is:
 *
 *     statements.g4
 *          |
 *          v
 *     control-flow.g4
 *          |
 *          +--> throwStatement
 *          |
 *          +--> tryStatement
 *                    |
 *                    v
 *             tryCatchFinallyStatement
 *
 * This establishes one owner for every rule.
 */


/*
 * ============================================================================
 * BLOCK INTEGRATION
 * ============================================================================
 *
 * All try/catch/finally bodies use:
 *
 *     blockExpression
 *
 * from:
 *
 *     grammar/statements/blocks.g4
 *
 * No duplicate block grammar is permitted here.
 *
 * Because blockExpression ultimately admits the canonical `statement` rule,
 * exception handlers can contain the entire Zamani statement universe.
 *
 * This includes, where semantically legal:
 *
 *     declarations
 *     assignments
 *     expressions
 *     conditionals
 *     loops
 *     matches
 *     returns
 *     breaks
 *     continues
 *     nested exceptions
 *     classical operations
 *     quantum operations
 *     hybrid operations
 *     HDL operations
 *     hardware-intent operations
 *     distributed operations
 *     AI/data operations
 *     networking operations
 *     future-domain operations
 *
 * No exception-specific block type is required.
 */


/*
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * `throwStatement` consumes:
 *
 *     expression
 *
 * from the canonical expression grammar.
 *
 * This allows:
 *
 *     throw error;
 *     throw make_error();
 *     throw result.error;
 *     throw compute_failure(code);
 *     throw some_expression;
 *
 * without creating:
 *
 *     quantumExceptionExpression
 *     hardwareExceptionExpression
 *     distributedExceptionExpression
 *     aiExceptionExpression
 *
 * or any other domain-specific expression grammar.
 *
 * Domain semantics are resolved after parsing.
 */


/*
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Typed catches consume:
 *
 *     typeExpression
 *
 * from:
 *
 *     grammar/types/types.g4
 *
 * This permits the full canonical Zamani type language to participate in
 * catch annotations without creating a second exception-type grammar.
 *
 * Examples:
 *
 *     catch (error: Error) { ... }
 *     catch (error: IOError) { ... }
 *     catch (error: QuantumExecutionError) { ... }
 *     catch (error: HardwareFailure) { ... }
 *     catch (error: DistributedFailure) { ... }
 *
 * Whether those types are semantically throwable is NOT determined here.
 */


/*
 * ============================================================================
 * SOURCE-LEVEL CONTROL FLOW
 * ============================================================================
 *
 * Exception constructs introduce possible control-flow paths.
 *
 * The semantic/control-flow subsystem may derive:
 *
 *     normal continuation
 *     exceptional continuation
 *     catch-handler entry
 *     finally entry
 *     function exit
 *     rethrow path
 *
 * This grammar merely preserves enough source structure for that analysis.
 *
 * It does not determine:
 *
 *     reachability;
 *     dominance;
 *     exhaustiveness;
 *     propagation;
 *     termination;
 *     cleanup guarantees;
 *     async propagation;
 *     distributed propagation.
 */


/*
 * ============================================================================
 * EFFECT SYSTEM INTEGRATION
 * ============================================================================
 *
 * Throwing and catching may participate in Zamani's effect system.
 *
 * For example, semantic analysis may determine that:
 *
 *     throw error;
 *
 * introduces an exception-related effect.
 *
 * This grammar does not encode that effect.
 *
 * The source construct remains:
 *
 *     THROW expression statementTerminator
 *
 * and the effect system operates downstream.
 *
 * This prevents the grammar from becoming coupled to a particular effect
 * implementation.
 */


/*
 * ============================================================================
 * OWNERSHIP / BORROWING INTEGRATION
 * ============================================================================
 *
 * A catch binding introduces a source-level binding.
 *
 * The grammar records:
 *
 *     identifier
 *
 * but does not determine:
 *
 *     mutability;
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     move semantics;
 *     copy semantics;
 *     resource ownership.
 *
 * Those remain semantic-analysis responsibilities.
 *
 * If the language later adds an explicitly specified binding modifier, that
 * feature must be introduced through the canonical binding/type system rather
 * than by silently changing this rule.
 */


/*
 * ============================================================================
 * CONCURRENCY / ASYNC INTEGRATION
 * ============================================================================
 *
 * Exception syntax remains independent of:
 *
 *     task count
 *     thread count
 *     worker count
 *     process count
 *     queue count
 *     node count
 *     channel count
 *
 * A handler may contain asynchronous or concurrent constructs whenever the
 * surrounding semantic context permits them.
 *
 * Exception propagation across async/concurrent boundaries is semantic/runtime
 * behavior, not parser behavior.
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum operations may appear inside any canonical exception block.
 *
 * Conceptually:
 *
 *     try {
 *         measure(q);
 *         continue_quantum_work();
 *     } catch (error: QuantumExecutionError) {
 *         recover();
 *     } finally {
 *         finalize();
 *     }
 *
 * This grammar has NO knowledge of:
 *
 *     qubit count;
 *     logical-qubit count;
 *     physical-qubit count;
 *     physical mapping;
 *     gate inventory;
 *     topology;
 *     calibration;
 *     pulse implementation;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     backend selection.
 *
 * After parsing and semantic analysis, quantum constructs continue through the
 * repository's canonical quantum boundary:
 *
 *     quantum::ir
 *
 * This grammar does not construct, modify, or duplicate quantum::ir.
 */


/*
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Exception blocks may contain HDL/hardware-intent constructs where the
 * surrounding language semantics permit them.
 *
 * This grammar does not create:
 *
 *     hardwareTry
 *     gpuTry
 *     fpgaTry
 *     asicTry
 *     qpuTry
 *
 * Hardware realization remains downstream.
 *
 * No:
 *
 *     bus width
 *     register count
 *     memory capacity
 *     device ID
 *     clock count
 *     topology
 *     pipeline depth
 *
 * may be encoded as an exception-language limit here.
 */


/*
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed operations may fail and their source-level failure values/types
 * may be represented through ordinary throw/catch constructs.
 *
 * This grammar does not determine:
 *
 *     replication;
 *     retry;
 *     failover;
 *     consistency;
 *     placement;
 *     node selection;
 *     communication topology;
 *     network recovery.
 *
 * Those concerns belong to distributed, resilience, compiler, runtime and
 * deployment subsystems.
 */


/*
 * ============================================================================
 * RESILIENCE / RECOVERY SEPARATION
 * ============================================================================
 *
 * A source-level:
 *
 *     try { ... } catch (...) { ... }
 *
 * is NOT equivalent to an automatic resilience policy.
 *
 * This grammar must never introduce parser-level syntax for:
 *
 *     retry count
 *     backend switching
 *     automatic failover
 *     QEC strategy
 *     mitigation
 *     quarantine
 *     fault diagnosis
 *     scheduler recovery
 *
 * unless those become separately specified language constructs with their own
 * ownership contracts.
 *
 * Explicit exception control flow and infrastructure resilience remain
 * separate concepts.
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Exception syntax does not allocate resources.
 *
 * It does not decide whether the target has:
 *
 *     memory;
 *     processors;
 *     accelerators;
 *     qubits;
 *     QPU capabilities;
 *     network capacity;
 *     FPGA resources;
 *     storage;
 *     communication paths.
 *
 * Such requirements and capabilities are represented through the appropriate
 * resource/capability systems and resolved downstream.
 */


/*
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing exception syntax performs no external operation.
 *
 * This grammar:
 *
 *     - reads no files;
 *     - writes no files;
 *     - opens no network connections;
 *     - executes no processes;
 *     - invokes no runtime;
 *     - discovers no hardware;
 *     - contacts no backend;
 *     - accesses no credentials.
 *
 * It contains no embedded Rust and therefore requires no `unsafe` Rust.
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source token stream
 *     grammar version
 *     lexer vocabulary
 *     parser configuration
 *
 * this grammar must produce the same structural parse.
 *
 * Parsing must not depend on:
 *
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     FPGA availability
 *     deployment topology
 *     scheduler state
 *     runtime state
 *     calibration state.
 */


/*
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar intentionally uses repetition and recursion supplied by the
 * canonical language structure.
 *
 * There is no finite language-level limit on:
 *
 *     catch clauses;
 *     nested try statements;
 *     nested handler blocks;
 *     expression complexity;
 *     source-program size.
 *
 * Examples conceptually supported:
 *
 *     try {
 *         ...
 *     } catch (a) {
 *         ...
 *     } catch (b) {
 *         ...
 *     } catch (c) {
 *         ...
 *     }
 *
 * with arbitrarily many catch clauses subject only to available implementation
 * resources.
 *
 * Likewise:
 *
 *     try {
 *         try {
 *             try {
 *                 ...
 *             } catch (inner) {
 *                 ...
 *             }
 *         } catch (middle) {
 *             ...
 *         }
 *     } catch (outer) {
 *         ...
 *     }
 *
 * has no grammar-defined nesting maximum.
 *
 * Practical parser recursion, memory and input-size protection are external
 * implementation policies and must not become language semantics.
 */


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser/frontend should be capable of identifying source-local syntax
 * errors including:
 *
 *     missing try body;
 *     missing catch binding;
 *     missing ')' after catch binding;
 *     missing catch handler block;
 *     malformed catch type;
 *     missing finally block;
 *     try without catch/finally;
 *     malformed throw expression;
 *     missing throw statement terminator;
 *     malformed nested exception syntax.
 *
 * Examples:
 *
 *     try {
 *         work();
 *     }
 *
 *     catch (error) {
 *         recover();
 *     }
 *
 *     try {
 *     } catch {
 *         recover();
 *     }
 *
 *     try {
 *     } finally;
 *
 *     throw;
 *
 *     throw;
 *
 *     try {
 *     } catch (error {
 *         recover();
 *     }
 *
 *     try {
 *     } catch (error) {
 *
 * Semantic errors MUST NOT be disguised as parser errors.
 *
 * Examples of semantic errors include:
 *
 *     catch type does not exist;
 *     catch type is not throwable;
 *     unreachable handler;
 *     invalid handler ordering;
 *     invalid ownership;
 *     invalid effect;
 *     unavailable capability.
 */


/*
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * This grammar contains no custom parser actions.
 *
 * Error recovery is provided by the repository's parser/error infrastructure.
 *
 * The grammar must not embed:
 *
 *     recovery callbacks;
 *     mutable parser state;
 *     runtime calls;
 *     filesystem operations;
 *     hardware operations.
 *
 * Recovery should allow the frontend to report multiple independent source
 * errors where the configured parser strategy permits it.
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does NOT construct Rust AST values.
 *
 * The frontend AST must preserve enough structure to represent:
 *
 *     ThrowStatement
 *         value
 *         source span
 *
 * and:
 *
 *     TryStatement
 *         body
 *         ordered catch clauses
 *         optional finally body
 *         source span
 *
 * with each catch clause preserving:
 *
 *     binding
 *     optional type
 *     handler body
 *     source span
 *
 * Conceptual representation:
 *
 *     TryStatement {
 *         body,
 *         catches: [
 *             CatchClause {
 *                 binding,
 *                 type?,
 *                 body,
 *             },
 *             ...
 *         ],
 *         finally_body?,
 *         source_span,
 *     }
 *
 * The exact Rust type and node representation remain owned by the frontend
 * AST implementation.
 *
 * Source ordering MUST be preserved.
 *
 * Source spans MUST be preserved.
 *
 * The parser MUST NOT inject target-specific information into the AST.
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis consumes the AST produced from these rules and determines:
 *
 *     - binding scope;
 *     - exception type validity;
 *     - throwable-value validity;
 *     - handler compatibility;
 *     - handler ordering;
 *     - reachability;
 *     - propagation;
 *     - rethrow behavior;
 *     - finally semantics;
 *     - effect requirements;
 *     - ownership requirements;
 *     - capability requirements;
 *     - control-flow properties;
 *     - resource implications.
 *
 * This file must not perform those analyses.
 */


/*
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Exception syntax must first lower through:
 *
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical semantic representation
 *
 * From there, relevant constructs may lower to their appropriate canonical
 * representation.
 *
 * The exception grammar does not introduce:
 *
 *     ExceptionIR
 *     QuantumExceptionIR
 *     HardwareExceptionIR
 *     RuntimeExceptionIR
 *
 * merely for parser convenience.
 *
 * Quantum constructs remain subject to:
 *
 *     quantum::ir
 *
 * as the canonical quantum semantic boundary.
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The following established source forms remain supported:
 *
 *     throw expression;
 *
 *     try {
 *         ...
 *     } catch (identifier) {
 *         ...
 *     }
 *
 *     try {
 *         ...
 *     } catch (identifier: type) {
 *         ...
 *     }
 *
 *     try {
 *         ...
 *     } finally {
 *         ...
 *     }
 *
 *     try {
 *         ...
 *     } catch (identifier) {
 *         ...
 *     } finally {
 *         ...
 *     }
 *
 * The previous modular exception grammar already intended to preserve these
 * forms. This replacement preserves them while correcting the canonical type
 * integration point from `typeExpr` to `typeExpression`.
 *
 * Legacy monolithic grammar definitions must not become competing production
 * authorities.
 *
 * The repository's compatibility layer must determine the transition status of
 * any legacy duplicate rule.
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * ---------------------------------------------------------------------------
 * POSITIVE
 * ---------------------------------------------------------------------------
 *
 *     throw error;
 *
 *     throw make_error();
 *
 *     throw computation_failure(code);
 *
 *     try {
 *         work();
 *     } catch (error) {
 *         recover();
 *     }
 *
 *     try {
 *         work();
 *     } catch (error: Error) {
 *         recover();
 *     }
 *
 *     try {
 *         work();
 *     } finally {
 *         cleanup();
 *     }
 *
 *     try {
 *         work();
 *     } catch (first) {
 *         recover_first();
 *     } catch (second: Error) {
 *         recover_second();
 *     } finally {
 *         cleanup();
 *     }
 *
 *     try {
 *         nested();
 *     } catch (error) {
 *         try {
 *             recover();
 *         } finally {
 *             cleanup();
 *         }
 *     }
 *
 * ---------------------------------------------------------------------------
 * CROSS-DOMAIN
 * ---------------------------------------------------------------------------
 *
 * Handler bodies must be capable of containing ordinary canonical statements
 * from:
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
 *     accelerator
 *     future domains
 *
 * No exception-specific domain grammar is required.
 *
 * ---------------------------------------------------------------------------
 * NEGATIVE
 * ---------------------------------------------------------------------------
 *
 *     try {
 *         work();
 *     }
 *
 *     catch (error) {
 *         recover();
 *     }
 *
 *     try {
 *     } catch {
 *         recover();
 *     }
 *
 *     try {
 *     } finally;
 *
 *     throw;
 *
 *     throw;
 *
 *     throw error
 *
 *     try {
 *     } catch (error {
 *         recover();
 *     }
 *
 *     try {
 *     } catch (error) {
 *
 * ---------------------------------------------------------------------------
 * BOUNDARY
 * ---------------------------------------------------------------------------
 *
 *     one catch clause;
 *     many catch clauses;
 *     catch with type;
 *     catch without type;
 *     finally without catch;
 *     catch plus finally;
 *     deeply nested try constructs;
 *     large handler blocks;
 *     large exception expressions;
 *     large programs containing many exception constructs.
 *
 * ---------------------------------------------------------------------------
 * SCALABILITY
 * ---------------------------------------------------------------------------
 *
 * Tests must verify that no grammar-defined artificial limit exists for:
 *
 *     catch count;
 *     nesting;
 *     handler size;
 *     expression size;
 *     program size.
 *
 * Test infrastructure may impose explicit host-resource limits, but such
 * limits must be external to language semantics.
 *
 * ---------------------------------------------------------------------------
 * DETERMINISM
 * ---------------------------------------------------------------------------
 *
 * Parsing identical source under identical grammar/lexer/parser versions must
 * produce structurally equivalent parse results.
 *
 * ---------------------------------------------------------------------------
 * COMPATIBILITY
 * ---------------------------------------------------------------------------
 *
 * Every stable exception form must be checked against:
 *
 *     canonical ANTLR parser
 *     Rust frontend lexer/parser
 *     frontend AST
 *     semantic analysis
 *     canonical downstream representation
 *
 * Any intentional discrepancy must be explicitly versioned and documented.
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain no:
 *
 *     MAX_CATCHES
 *     MAX_TRY_DEPTH
 *     MAX_HANDLER_COUNT
 *     MAX_EXCEPTION_SIZE
 *     MAX_PROGRAM_SIZE
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * It must contain no:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     node0
 *     device0
 *     physical_qubit0
 *
 * as special language-level resources.
 *
 * Ordinary user identifiers with such spelling remain ordinary identifiers.
 */


/*
 * ============================================================================
 * RUST / IMPLEMENTATION SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * The repository implementation consuming this grammar targets:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Safe Rust only.
 *
 * No `unsafe` Rust is required by this grammar.
 *
 * The grammar must not require generated parser actions containing unsafe
 * operations.
 */


/*
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     [ ] ExceptionsParser is the sole owner of exception productions.
 *     [ ] No duplicate throwStatement is authoritative.
 *     [ ] No duplicate tryCatchFinallyStatement is authoritative.
 *     [ ] No duplicate catchClause is authoritative.
 *     [ ] No duplicate finallyClause is authoritative.
 *     [ ] Canonical token vocabulary is used.
 *     [ ] Canonical `expression` is consumed.
 *     [ ] Canonical `typeExpression` is consumed.
 *     [ ] Canonical `identifier` is consumed.
 *     [ ] Canonical `blockExpression` is consumed.
 *     [ ] Canonical `statementTerminator` is consumed.
 *     [ ] No local duplicate type grammar exists.
 *     [ ] No local duplicate block grammar exists.
 *     [ ] No local duplicate identifier grammar exists.
 *     [ ] No semantic actions exist.
 *     [ ] No semantic predicates exist.
 *     [ ] No unsafe Rust is required.
 *     [ ] No machine-size constants exist.
 *     [ ] No hardware topology is encoded.
 *     [ ] No physical device selection is encoded.
 *     [ ] No quantum gate inventory is encoded.
 *     [ ] No QEC implementation is encoded.
 *     [ ] No ZQN implementation is encoded.
 *     [ ] No routing is encoded.
 *     [ ] No scheduling is encoded.
 *     [ ] No runtime implementation is encoded.
 *     [ ] No second quantum IR is introduced.
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] throwStatement is the sole owner of throw syntax.
 *     [ ] tryCatchFinallyStatement is the sole owner of try syntax.
 *     [ ] catchClause is the sole owner of catch syntax.
 *     [ ] catchBinding is the sole owner of catch-binding syntax.
 *     [ ] finallyClause is the sole owner of finally syntax.
 *
 *     [ ] try/catch parses.
 *     [ ] try/finally parses.
 *     [ ] try/catch/finally parses.
 *     [ ] multiple catches parse.
 *     [ ] typed catches parse.
 *     [ ] nested exception constructs parse.
 *     [ ] throw expressions parse.
 *
 *     [ ] try without catch/finally is rejected.
 *     [ ] malformed catch bindings are rejected.
 *     [ ] malformed finally clauses are rejected.
 *     [ ] malformed throw statements are rejected.
 *
 *     [ ] `typeExpression` is used instead of a competing `typeExpr`.
 *     [ ] canonical `identifier` is used.
 *     [ ] canonical `blockExpression` is used.
 *     [ ] canonical `expression` is used.
 *     [ ] canonical `statementTerminator` is used.
 *
 *     [ ] statements.g4 remains the statement composition owner.
 *     [ ] control-flow.g4 remains the control-flow composition owner.
 *     [ ] no statement dispatcher is duplicated here.
 *     [ ] no tryStatement adapter is duplicated here.
 *
 *     [ ] AST integration is defined.
 *     [ ] semantic integration is defined.
 *     [ ] IR integration is defined.
 *     [ ] diagnostics are defined.
 *     [ ] compatibility is defined.
 *     [ ] positive tests exist.
 *     [ ] negative tests exist.
 *     [ ] boundary tests exist.
 *     [ ] scalability tests exist.
 *     [ ] deterministic parsing is tested.
 *
 *     [ ] no hardware limit is encoded.
 *     [ ] no resource-count limit is encoded.
 *     [ ] no quantum limit is encoded.
 *     [ ] no second quantum IR is introduced.
 *     [ ] quantum::ir remains downstream.
 *     [ ] QEC remains downstream.
 *     [ ] ZQN remains downstream.
 *     [ ] routing remains downstream.
 *     [ ] scheduling remains downstream.
 *     [ ] HAL remains downstream.
 *
 *     [ ] no Rust actions exist.
 *     [ ] no unsafe Rust is required.
 *     [ ] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file defines:
 *
 *     SOURCE-LEVEL EXCEPTION SYNTAX
 *
 * and nothing more.
 *
 * The complete Zamani direction remains:
 *
 *     SOURCE
 *       |
 *       v
 *     LEXER
 *       |
 *       v
 *     PARSER
 *       |
 *       v
 *     DOMAIN-NEUTRAL AST
 *       |
 *       v
 *     SEMANTIC ANALYSIS
 *       |
 *       v
 *     CANONICAL SEMANTIC REPRESENTATION
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL / hardware
 *       +--> distributed
 *       +--> accelerator
 *       +--> future domains
 *       |
 *       v
 *     OPTIMIZATION
 *       |
 *       v
 *     ROUTING / SCHEDULING / LOWERING
 *       |
 *       v
 *     RESILIENCE / QEC / ZQN WHERE APPLICABLE
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     TARGET REALIZATION
 *
 * Exception syntax does not choose the target.
 *
 * Exception syntax does not allocate hardware.
 *
 * Exception syntax does not define physical resources.
 *
 * Exception syntax does not define quantum hardware.
 *
 * Exception syntax does not define QEC.
 *
 * Exception syntax does not define ZQN.
 *
 * Exception syntax does not define scheduling.
 *
 * Exception syntax does not define routing.
 *
 * Exception syntax does not define runtime behavior.
 *
 * It expresses portable program control flow.
 *
 * Therefore the same exception-containing Zamani source remains structurally
 * valid from tiny systems to arbitrarily larger systems, subject to actual
 * available resources and downstream implementation policies.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */