/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/exceptions.g4
 *
 * Status:
 *     Canonical production grammar for exception/control-flow statements.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar component.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No unsafe code.
 *     No I/O.
 *     No filesystem access.
 *     No networking.
 *     No device discovery.
 *     No hardware inspection.
 *     No runtime execution.
 *     No mutable global state.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the concrete syntax of Zamani exception-oriented statements.
 *
 * It defines:
 *
 *     - throwStatement
 *     - tryCatchFinallyStatement
 *     - catchClause
 *     - finallyClause
 *     - catchBinding
 *
 * The surrounding statement dispatcher is owned by:
 *
 *     grammar/statements/statements.g4
 *
 * That file connects:
 *
 *     controlFlowStatement
 *         -> throwStatement
 *         -> tryStatement
 *
 * and:
 *
 *     tryStatement
 *         -> tryCatchFinallyStatement
 *
 * This file therefore MUST NOT redefine:
 *
 *     statement
 *     controlFlowStatement
 *     tryStatement
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - exception statement syntax;
 *     - throw syntax;
 *     - try syntax;
 *     - catch syntax;
 *     - finally syntax;
 *     - typed catch bindings;
 *     - catch-clause ordering;
 *     - the requirement that a try construct have a handler or finalizer;
 *     - source-level exception-control-flow structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer keywords;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - blocks;
 *     - functions;
 *     - modules;
 *     - effects;
 *     - capabilities;
 *     - resource limits;
 *     - runtime exception objects;
 *     - exception allocation;
 *     - stack unwinding;
 *     - process termination;
 *     - recovery;
 *     - retry policy;
 *     - resilience policy;
 *     - quantum error correction;
 *     - ZQN fault modelling;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - hardware discovery;
 *     - backend selection.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     Zamani lexer
 *       |
 *       v
 *     this parser component
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
 *       +--> ownership / borrowing
 *       +--> control-flow analysis
 *       +--> resource analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware IR
 *       +--> distributed representations
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
 *     runtime
 *
 * This grammar is therefore purely syntactic.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * Exception syntax must describe program semantics rather than machine
 * characteristics.
 *
 * This grammar contains no limits for:
 *
 *     - exception nesting;
 *     - catch-clause count;
 *     - try-block size;
 *     - program size;
 *     - qubit count;
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - accelerator count;
 *     - node count;
 *     - thread count;
 *     - memory capacity;
 *     - device count;
 *     - topology;
 *     - physical addresses.
 *
 * Therefore:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * remains a downstream realization property rather than a grammar limitation.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * with the keyword vocabulary including:
 *
 *     TRY
 *     CATCH
 *     FINALLY
 *     THROW
 *
 * This file MUST NOT declare lexer rules.
 *
 * The parser consumes the assembled ZamaniLexer vocabulary through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Required external parser rules:
 *
 *     expression
 *     typeExpr
 *     blockExpression
 *     identifier
 *
 * `expression` belongs to the expression grammar.
 *
 * `typeExpr` belongs to the type grammar.
 *
 * `blockExpression` belongs to:
 *
 *     grammar/statements/blocks.g4
 *
 * `identifier` belongs to the canonical name/identifier grammar.
 *
 * This file deliberately does not duplicate any of those grammars.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The legacy monolithic grammar currently contains:
 *
 *     throwStatement
 *         : 'throw' expression ';'
 *         ;
 *
 * and:
 *
 *     tryCatchFinally
 *         : 'try' block catchClause* finallyClause?
 *         ;
 *
 * with:
 *
 *     catchClause
 *         : 'catch' '(' IDENTIFIER (':' typeExpr)? ')' block
 *         ;
 *
 * This modular component preserves that source-level syntax while making the
 * exception grammar independently owned and reusable.
 *
 * The modular grammar additionally makes the important structural constraint
 * explicit:
 *
 *     try MUST have at least one catch clause OR a finally clause.
 *
 * Thus:
 *
 *     try { ... }
 *
 * is invalid.
 *
 * This prevents silently accepting an exception construct that has no handler
 * and no cleanup semantics.
 *
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
 * Canonical form:
 *
 *     throw expression;
 *
 * Examples:
 *
 *     throw error;
 *
 *     throw make_error(code);
 *
 *     throw computation_failure;
 *
 * The expression is intentionally unrestricted at grammar level.
 *
 * Semantic analysis determines whether the resulting value is a valid
 * throwable/error value in the active language model.
 *
 * This keeps the grammar independent of any particular exception hierarchy.
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
 *         body
 *     } catch (error) {
 *         handler
 *     }
 *
 *     try {
 *         body
 *     } catch (error: ErrorType) {
 *         handler
 *     }
 *
 *     try {
 *         body
 *     } finally {
 *         cleanup
 *     }
 *
 *     try {
 *         body
 *     } catch (error) {
 *         handler
 *     } finally {
 *         cleanup
 *     }
 *
 * One or more catch clauses may occur.
 *
 * A finally clause is optional.
 *
 * However, at least one of:
 *
 *     catchClause
 *     finallyClause
 *
 * is mandatory.
 *
 * This prevents an empty exception-control construct:
 *
 *     try { ... }
 *
 * from being accepted.
 *
 * No finite number of catch clauses is encoded.
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
 * The canonical binding forms are:
 *
 *     catch (error) { ... }
 *
 *     catch (error: SomeError) { ... }
 *
 * The identifier is retained for compatibility with the existing Zamani
 * grammar.
 *
 * The type annotation is optional.
 *
 * The grammar does not determine:
 *
 *     - whether the type exists;
 *     - whether the type is throwable;
 *     - whether the type is reachable;
 *     - whether the catch is redundant;
 *     - whether the handler is exhaustive;
 *     - whether catch ordering is semantically valid.
 *
 * Those are semantic-analysis responsibilities.
 */
catchClause
    : CATCH LPAREN catchBinding RPAREN blockExpression
    ;


/*
 * ============================================================================
 * CATCH BINDING
 * ============================================================================
 *
 * Canonical form:
 *
 *     identifier
 *
 * or:
 *
 *     identifier : type
 *
 * Example:
 *
 *     catch (error) { ... }
 *
 *     catch (error: IOError) { ... }
 *
 * The binding introduces a handler-local name.
 *
 * Scope, ownership, lifetime, mutability and type validity are semantic
 * responsibilities.
 */
catchBinding
    : identifier
      (COLON typeExpr)?
    ;


/*
 * ============================================================================
 * FINALLY CLAUSE
 * ============================================================================
 *
 * Canonical form:
 *
 *     finally {
 *         cleanup
 *     }
 *
 * The block is always required.
 *
 * The grammar intentionally does not allow:
 *
 *     finally;
 *
 * because cleanup/control-flow semantics require an explicit statement block.
 */
finallyClause
    : FINALLY blockExpression
    ;


/*
 * ============================================================================
 * STATEMENT-DISPATCH INTEGRATION
 * ============================================================================
 *
 * grammar/statements/statements.g4 already owns the statement dispatcher.
 *
 * Its control-flow composition is conceptually:
 *
 *     controlFlowStatement
 *         : conditionalStatement
 *         | loopStatement
 *         | matchStatement
 *         | returnStatement
 *         | breakStatement
 *         | continueStatement
 *         | throwStatement
 *         | tryStatement
 *         ;
 *
 * and:
 *
 *     tryStatement
 *         : tryCatchFinallyStatement
 *         ;
 *
 * Therefore this file supplies the missing concrete rules:
 *
 *     throwStatement
 *     tryCatchFinallyStatement
 *     catchClause
 *     finallyClause
 *
 * It MUST NOT redefine:
 *
 *     controlFlowStatement
 *     tryStatement
 *     statement
 *
 * This prevents circular ownership.
 *
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
 * This is important because exception handlers must be able to contain the
 * complete Zamani statement language.
 *
 * For example:
 *
 *     try {
 *         classical_work();
 *
 *         quantum_work();
 *
 *         hardware_work();
 *
 *         distributed_work();
 *     } catch (error) {
 *         recover();
 *     } finally {
 *         cleanup();
 *     }
 *
 * No domain-specific exception block is required.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * `throwStatement` accepts the canonical:
 *
 *     expression
 *
 * grammar.
 *
 * This means the grammar does not create special categories such as:
 *
 *     quantumExceptionExpression
 *     hardwareExceptionExpression
 *     distributedExceptionExpression
 *
 * Exception values are ordinary language values whose semantic validity is
 * determined later.
 *
 * This permits future domains to participate without modifying this grammar.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * A catch binding may optionally carry:
 *
 *     : typeExpr
 *
 * Example:
 *
 *     catch (error: QuantumExecutionError) {
 *         recover();
 *     }
 *
 * Whether a type is:
 *
 *     throwable
 *     compatible
 *     abstract
 *     concrete
 *     reachable
 *     redundant
 *     exhaustive
 *
 * is not a parser concern.
 *
 * Type checking owns those decisions.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for semantic analysis to determine:
 *
 *     - try-body;
 *     - ordered catch clauses;
 *     - each catch binding;
 *     - optional catch type;
 *     - handler body;
 *     - optional finally body;
 *     - source locations.
 *
 * Conceptual AST shape:
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
 * Conceptual throw shape:
 *
 *     ThrowStatement {
 *         value,
 *         source_span,
 *     }
 *
 * The exact Rust AST representation belongs to the frontend.
 *
 * This grammar MUST NOT define Rust structures or semantic actions.
 *
 * ============================================================================
 * CONTROL-FLOW CONTRACT
 * ============================================================================
 *
 * Exception constructs introduce control-flow edges.
 *
 * Semantic/control-flow analysis may derive:
 *
 *     normal continuation
 *     exceptional continuation
 *     handler entry
 *     finally execution
 *     function exit
 *
 * This grammar only preserves source structure.
 *
 * It does not decide:
 *
 *     - whether a throw is reachable;
 *     - whether a catch handles it;
 *     - whether finally always executes;
 *     - whether a handler rethrows;
 *     - whether control flow terminates;
 *     - whether an exception crosses an async boundary.
 *
 * ============================================================================
 * EFFECT SYSTEM INTEGRATION
 * ============================================================================
 *
 * Throwing and catching may interact with Zamani's effect system.
 *
 * However, this grammar must not embed effect rules.
 *
 * For example:
 *
 *     fn compute() with effects { ... } {
 *         throw error;
 *     }
 *
 * Whether throwing requires a particular effect is determined by semantic
 * effect checking.
 *
 * The parser must remain independent of the effect implementation.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Exception syntax must remain independent of:
 *
 *     thread count
 *     task count
 *     worker count
 *     processor count
 *     queue count
 *     node count
 *
 * A handler may contain asynchronous or concurrent constructs if those constructs
 * are legal in the surrounding semantic context.
 *
 * The grammar does not encode any resource ceiling.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum operations may occur inside try/catch/finally blocks.
 *
 * Example:
 *
 *     try {
 *         measure(q);
 *     } catch (error: QuantumExecutionError) {
 *         recover();
 *     }
 *
 * The grammar does NOT decide:
 *
 *     - number of qubits;
 *     - logical/physical mapping;
 *     - QPU topology;
 *     - calibration;
 *     - gate implementation;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - backend selection.
 *
 * After parsing and semantic analysis, quantum constructs continue toward the
 * canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This file must never construct or duplicate quantum::ir.
 *
 * ============================================================================
 * RESILIENCE / RECOVERY INTEGRATION
 * ============================================================================
 *
 * A source-level exception handler is NOT the same thing as Zamani's resilience
 * subsystem.
 *
 * This grammar only describes explicit programmer-authored control flow.
 *
 * It must not encode:
 *
 *     retry counts
 *     recovery policies
 *     backend switching
 *     QEC strategy
 *     fault diagnosis
 *     mitigation strategy
 *     quarantine policy
 *     scheduler recovery
 *
 * Those remain owned by the appropriate resilience/compiler/runtime systems.
 *
 * An explicit:
 *
 *     try { ... } catch (...) { ... }
 *
 * therefore does not imply automatic retry or hardware recovery.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * A handler may contain hardware or HDL statements where those constructs are
 * legal.
 *
 * This grammar does not create separate:
 *
 *     cpuTryStatement
 *     gpuTryStatement
 *     fpgaTryStatement
 *     asicTryStatement
 *     qpuTryStatement
 *
 * Exception control flow is language-level semantics.
 *
 * Target-specific implementation is downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Remote or distributed operations may eventually report failures represented
 * by values/types handled by catch clauses.
 *
 * This grammar does not decide:
 *
 *     network failure semantics;
 *     retry policy;
 *     replication;
 *     consistency;
 *     node selection;
 *     placement;
 *     failover.
 *
 * Those belong to distributed/runtime/resilience layers.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Exception syntax performs no external operation.
 *
 * Parsing this grammar cannot:
 *
 *     access files;
 *     access networks;
 *     execute processes;
 *     inspect hardware;
 *     discover devices;
 *     invoke runtimes;
 *     contact backends.
 *
 * Any runtime behavior associated with an exception is downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *     parser configuration
 *
 * parsing must produce the same structural result.
 *
 * The grammar has no:
 *
 *     time dependence
 *     hardware dependence
 *     network dependence
 *     scheduler dependence
 *     backend dependence
 *     calibration dependence
 *     device dependence
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite language-level limits are encoded here.
 *
 * The grammar permits arbitrary repetition of catch clauses:
 *
 *     catchClause+
 *
 * rather than a fixed number such as:
 *
 *     catchClause?
 *     catchClause?
 *
 * or an enumerated maximum.
 *
 * Nested try statements are naturally supported because:
 *
 *     blockExpression
 *
 * can contain statements and therefore another try statement.
 *
 * Consequently there is no grammar-defined maximum for:
 *
 *     nesting depth
 *     handler count
 *     program size
 *     handler size
 *
 * Practical parser memory/stack limits are implementation/resource policies,
 * not language semantics.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * The parser/frontend should be able to report source-local diagnostics for:
 *
 *     1. missing try block;
 *     2. missing catch binding;
 *     3. missing closing ')' in catch;
 *     4. missing catch handler block;
 *     5. malformed catch type;
 *     6. missing finally block;
 *     7. try without catch/finally;
 *     8. malformed throw expression;
 *     9. missing statement terminator after throw;
 *    10. malformed nested exception statement.
 *
 * No diagnostic implementation is embedded here.
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * ANTLR's parser recovery remains responsible for recovering from malformed
 * input.
 *
 * This grammar must not contain custom parser actions or embedded recovery code.
 *
 * Recovery must preserve the possibility of reporting multiple independent
 * source errors in one compilation.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing valid forms preserved by this grammar include:
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
 * Existing legacy syntax must remain parseable during migration.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests must include:
 *
 *     throw error;
 *
 *     throw make_error();
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
 *         if condition {
 *             try {
 *                 nested();
 *             } catch (error) {
 *                 recover();
 *             }
 *         }
 *     } finally {
 *         cleanup();
 *     }
 *
 * Cross-domain tests must include handlers containing:
 *
 *     classical operations
 *     quantum operations
 *     hybrid operations
 *     HDL operations
 *     hardware operations
 *     distributed operations
 *     accelerator operations
 *     AI/data operations
 *
 * Negative tests must include:
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
 *     throw ;
 *
 *     throw error
 *
 *     try {
 *     } catch (error
 *
 *     try {
 *     } catch (error) {
 *
 * Boundary tests must include:
 *
 *     deeply nested try/catch/finally constructs;
 *
 *     very large handler blocks;
 *
 *     many catch clauses;
 *
 *     long exception expressions;
 *
 *     large programs containing many exception constructs.
 *
 * The test suite must verify that no grammar-defined artificial maximum exists.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_CATCHES
 *     MAX_TRY_DEPTH
 *     MAX_EXCEPTION_SIZE
 *     MAX_HANDLERS
 *     MAX_PROGRAM_SIZE
 *     MAX_THREADS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * No physical-machine property belongs in this file.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] It parses every supported throw form.
 *     [ ] It parses try/catch.
 *     [ ] It parses try/finally.
 *     [ ] It parses try/catch/finally.
 *     [ ] It parses multiple catch clauses.
 *     [ ] It preserves legacy valid syntax.
 *     [ ] It rejects try without catch/finally.
 *     [ ] It rejects malformed catch bindings.
 *     [ ] It rejects malformed finally clauses.
 *     [ ] It integrates with statements.g4 without redefining its dispatcher.
 *     [ ] It uses the canonical ZamaniLexer vocabulary.
 *     [ ] It uses the canonical expression grammar.
 *     [ ] It uses the canonical type grammar.
 *     [ ] It uses the canonical block grammar.
 *     [ ] It contains no duplicated expression/type/block grammar.
 *     [ ] It contains no Rust actions.
 *     [ ] It contains no unsafe code.
 *     [ ] It performs no I/O.
 *     [ ] It contains no hardware assumptions.
 *     [ ] It contains no resource-count assumptions.
 *     [ ] It contains no finite scalability constants.
 *     [ ] It does not construct IR.
 *     [ ] It does not depend on quantum::ir.
 *     [ ] It does not depend on QEC.
 *     [ ] It does not depend on ZQN.
 *     [ ] It does not depend on routing.
 *     [ ] It does not depend on scheduling.
 *     [ ] It does not depend on runtime implementation.
 *     [ ] Parser output remains deterministic.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */