/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/returns.g4
 *
 * Status:
 *     Canonical production grammar for return statements.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     unsafe code forbidden by the consuming Rust crate
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SOLE GRAMMATICAL OWNER of the source-level `return`
 * statement.
 *
 * It defines exactly the syntax required to express:
 *
 *     return;
 *
 * and:
 *
 *     return expression;
 *
 * The grammar intentionally does not decide whether either form is
 * semantically valid in the surrounding callable.
 *
 * Semantic validation belongs to the frontend semantic-analysis layer.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     ReturnStatements parser grammar       <-- THIS FILE
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> return-type validation
 *          +--> control-flow validation
 *          +--> ownership / borrowing
 *          +--> effect validation
 *          +--> capability validation
 *          +--> resource validation
 *          |
 *          v
 *     canonical semantic representations
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> hybrid IR
 *          +--> HDL / hardware IR
 *          +--> future domain IRs
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / lowering
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime / hardware
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - returnStatement
 *     - the optional-return-expression distinction
 *     - return-statement termination
 *     - return-specific parser composition
 *     - the return grammar integration boundary
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions
 *     - keyword spelling
 *     - identifier syntax
 *     - expression precedence
 *     - expression syntax
 *     - function declarations
 *     - callable return types
 *     - control-flow semantic analysis
 *     - type checking
 *     - ownership / borrowing
 *     - effects
 *     - capabilities
 *     - resources
 *     - classical IR
 *     - quantum::ir
 *     - QEC
 *     - ZQN
 *     - optimization
 *     - routing
 *     - scheduling
 *     - hardware discovery
 *     - calibration
 *     - runtime execution
 *     - backend selection
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative `returnStatement` rule in the
 * assembled Zamani parser.
 *
 * The legacy rule currently present in:
 *
 *     grammar/statements/statements.g4
 *
 * must be removed from that file when the modular statement grammar becomes
 * authoritative.
 *
 * The old rule:
 *
 *     returnStatement
 *         : RETURN expression? ';'
 *         ;
 *
 * is NOT authoritative because:
 *
 *     1. `RETURN` does not match the canonical lexer vocabulary;
 *     2. it duplicates ownership;
 *     3. it prevents clean modular composition;
 *     4. it couples statement composition to an obsolete token contract.
 *
 * This file replaces that ownership.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/lexer/tokens.g4
 *
 * It owns:
 *
 *     K_RETURN
 *     SEMICOLON
 *
 * This parser grammar MUST NOT redefine either token.
 *
 * The canonical source spelling is:
 *
 *     return
 *
 * represented by:
 *
 *     K_RETURN
 *
 * Statement termination is represented by:
 *
 *     SEMICOLON
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Return values consume the canonical `expression` rule from:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file MUST NOT define:
 *
 *     expression
 *     assignmentExpression
 *     binaryExpression
 *     unaryExpression
 *     literalExpression
 *     quantumExpression
 *     hardwareExpression
 *     HDL expression syntax
 *
 * That separation is essential.
 *
 * A return expression may therefore eventually contain any expression
 * supported by Zamani, including expressions whose semantic value belongs to:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL / hardware computation
 *     accelerator computation
 *     distributed computation
 *     AI / ML computation
 *     data computation
 *     networking
 *     future computational domains
 *
 * This grammar does not need to know those domains.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Return syntax describes source-level control-flow intent.
 *
 * It does NOT encode:
 *
 *     processor count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     qubit count
 *     register count
 *     memory capacity
 *     node count
 *     topology
 *     device identifier
 *     hardware address
 *     queue size
 *     scheduling slot
 *     pulse duration
 *     calibration value
 *     backend
 *     vendor
 *
 * Therefore:
 *
 *     return value;
 *
 * has the same syntactic meaning regardless of the eventual execution
 * substrate.
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A return expression may contain a quantum-related expression.
 *
 * Example:
 *
 *     return measure(q);
 *
 * The grammar does not decide whether `measure(q)` is:
 *
 *     - a quantum measurement;
 *     - a simulator operation;
 *     - a hybrid result;
 *     - a future quantum abstraction.
 *
 * The expression parser represents syntax.
 *
 * Semantic analysis determines the meaning.
 *
 * The resulting quantum semantics may eventually enter the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar MUST NOT construct or depend directly on quantum::ir.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * The returned expression may likewise represent:
 *
 *     hardware values
 *     HDL values
 *     resource handles
 *     accelerator results
 *     distributed values
 *
 * No hardware-specific return syntax is necessary merely because the
 * returned value is hardware-related.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * CONTROL-FLOW CONTRACT
 * ============================================================================
 *
 * A return statement terminates the current callable's source-level control
 * flow.
 *
 * This grammar only establishes its syntactic form.
 *
 * Semantic analysis determines:
 *
 *     - which callable contains the return;
 *     - the declared callable return type;
 *     - whether a value is required;
 *     - whether a value is forbidden;
 *     - whether the expression type matches;
 *     - whether control-flow permits the return;
 *     - whether effects/capabilities permit the operation;
 *     - whether ownership/borrowing rules permit the returned value.
 *
 * ============================================================================
 * BARE RETURN
 * ============================================================================
 *
 * Canonical form:
 *
 *     return;
 *
 * Grammar representation:
 *
 *     K_RETURN SEMICOLON
 *
 * This corresponds to the AST representation:
 *
 *     value = None
 *
 * ============================================================================
 * VALUE RETURN
 * ============================================================================
 *
 * Canonical form:
 *
 *     return expression;
 *
 * Grammar representation:
 *
 *     K_RETURN expression SEMICOLON
 *
 * This corresponds to:
 *
 *     value = Some(expression-node-id)
 *
 * The grammar does not restrict expression size, type, domain, or resource
 * requirements.
 *
 * ============================================================================
 * TERMINATION
 * ============================================================================
 *
 * The current Zamani grammar uses explicit semicolon termination.
 *
 * Therefore this file deliberately does NOT implement:
 *
 *     automatic semicolon insertion
 *     newline-sensitive termination
 *     optional terminators
 *     dialect-specific terminators
 *
 * Such changes would be language-level compatibility changes and must be
 * introduced through the language specification and compatibility process.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Examples of syntax errors owned by this parser boundary:
 *
 *     return
 *     return value
 *     return ;
 *
 * The first two are incomplete under the current explicit-terminator
 * language contract.
 *
 * Examples of semantic errors NOT owned here:
 *
 *     return value;
 *     // value has the wrong type
 *
 *     return value;
 *     // callable is declared void
 *
 *     return;
 *     // callable requires a value
 *
 *     return quantum_value;
 *     // quantum value cannot be returned by this callable
 *
 * Those belong to semantic analysis.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar:
 *
 *     - contains no semantic predicates;
 *     - contains no embedded target code;
 *     - contains no mutable state;
 *     - contains no randomness;
 *     - contains no timestamps;
 *     - contains no I/O;
 *     - contains no hardware inspection.
 *
 * The same token sequence must produce the same parse structure.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No artificial limits are encoded.
 *
 * In particular, this file contains no:
 *
 *     MAX_RETURN_VALUES
 *     MAX_RETURN_EXPRESSION_SIZE
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * A return statement contains at most one optional expression because that
 * is the language's semantic shape of a return statement, not because of a
 * machine limitation.
 *
 * The expression itself may contain arbitrarily large structures subject to
 * parser/compiler resource policies and available resources.
 *
 * Parser recursion/stack protections, if required, are implementation
 * policies and MUST NOT be encoded as source-language cardinality limits.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     hardware access
 *     device discovery
 *     runtime execution
 *
 * No embedded Rust actions are permitted.
 *
 * Generated Rust integration MUST remain compatible with:
 *
 *     #![forbid(unsafe_code)]
 *
 * and the repository's Rust 1.97 / 1.97.1 baseline.
 *
 * ============================================================================
 * SERIALIZATION / AST CONTRACT
 * ============================================================================
 *
 * This grammar emits parser structure only.
 *
 * The frontend parser/AST builder maps:
 *
 *     return;
 *
 * to:
 *
 *     ReturnStatement {
 *         value: None,
 *     }
 *
 * and:
 *
 *     return expression;
 *
 * to:
 *
 *     ReturnStatement {
 *         value: Some(expression_node_id),
 *     }
 *
 * The expression node remains owned by the enclosing AST graph.
 *
 * This grammar does not introduce AST fields.
 *
 * ============================================================================
 * INCREMENTAL COMPILATION
 * ============================================================================
 *
 * Return syntax is structurally independent from semantic information.
 *
 * Semantic analysis may therefore associate information with the resulting
 * AST NodeId, including:
 *
 *     enclosing callable
 *     resolved return type
 *     control-flow facts
 *     effects
 *     capabilities
 *     ownership
 *     resource requirements
 *
 * None of those fields belong in this grammar.
 *
 * ============================================================================
 * DIALECT / VERSIONING CONTRACT
 * ============================================================================
 *
 * `return` is a core language construct.
 *
 * Dialects MUST NOT silently redefine its meaning.
 *
 * A dialect may extend surrounding expression syntax, but the core structural
 * contract remains:
 *
 *     return;
 *
 *     return expression;
 *
 * Any future syntax such as:
 *
 *     return from label;
 *     return defer expression;
 *     return with metadata;
 *
 * requires an explicit language-version/specification decision and must not
 * be smuggled into this file as an implementation convenience.
 *
 * ============================================================================
 * INTEGRATION GRAPH
 * ============================================================================
 *
 *     grammar/lexer/tokens.g4
 *              |
 *              | K_RETURN, SEMICOLON
 *              v
 *     grammar/statements/returns.g4
 *              |
 *              | expression
 *              v
 *     grammar/expressions/expressions.g4
 *              |
 *              v
 *     frontend parser
 *              |
 *              v
 *     src/frontend/ast/node/statements/return_stmt.rs
 *              |
 *              v
 *     semantic analysis
 *              |
 *              +--> callable return-type checking
 *              +--> control-flow analysis
 *              +--> ownership/effects/capabilities
 *              |
 *              v
 *     canonical semantic representations
 *              |
 *              +--> classical IR
 *              +--> quantum::ir
 *              +--> hybrid IR
 *              +--> HDL/hardware IR
 *              |
 *              v
 *     optimization / routing / scheduling / lowering
 *
 * There is intentionally no:
 *
 *     returns.g4 -> quantum::ir
 *     returns.g4 -> QEC
 *     returns.g4 -> ZQN
 *     returns.g4 -> scheduling
 *     returns.g4 -> hardware
 *     returns.g4 -> runtime
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     return;
 *     return value;
 *     return compute();
 *     return a + b;
 *     return if condition { value } else { other };
 *     return measure(q);
 *     return hardware_result;
 *     return distributed_result;
 *
 * Negative:
 *
 *     return
 *     return value
 *     return;
 *     // where the parser input is incomplete as applicable
 *
 *     return value value;
 *
 *     return;
 *     extra-token
 *
 * Boundary:
 *
 *     return <very large valid expression>;
 *
 *     return <deeply composed expression>;
 *
 *     return <large quantum/classical/hybrid expression>;
 *
 * The grammar itself must not introduce artificial limits for these cases.
 *
 * Cross-domain:
 *
 *     classical function returning a classical value
 *     classical function returning a quantum measurement result
 *     hybrid function returning a classical result
 *     hardware/HDL computation returning a value
 *     distributed computation returning a result
 *     accelerator computation returning a result
 *
 * Round-trip:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter/serializer
 *       -> parser
 *
 * must preserve the distinction between:
 *
 *     return;
 *
 * and:
 *
 *     return expression;
 *
 * ============================================================================
 * IMPLEMENTATION
 * ============================================================================
 */

parser grammar ReturnStatements;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * The canonical expression grammar owns `expression`.
 *
 * Importing it here prevents return statements from creating a second
 * expression grammar.
 */
import Expressions;


/*
 * ============================================================================
 * CANONICAL RETURN STATEMENT
 * ============================================================================
 *
 * Exactly two structural forms are accepted:
 *
 *     return;
 *
 *     return expression;
 *
 * The expression is optional, but the terminator is mandatory.
 *
 * This form is deliberately compact and deterministic.
 */
returnStatement
    : K_RETURN expression? SEMICOLON
    ;