/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/postfix.g4
 *
 * Status:
 *     Canonical production postfix-expression composition contract.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the postfix-expression layer of Zamani expressions.
 *
 * Postfix expressions are expressions whose meaning is extended by one or
 * more source-level postfix operations.
 *
 * Conceptually:
 *
 *     primary
 *        |
 *        +--> call
 *        +--> indexing
 *        +--> member access
 *        +--> qualified/member selection
 *        +--> optional access
 *        +--> postfix operators
 *        |
 *        +--> call
 *        +--> indexing
 *        +--> member access
 *        +--> ...
 *
 * The repetition is intentionally open-ended.
 *
 * There is NO language-level maximum for:
 *
 *     postfix operations
 *     call depth
 *     index depth
 *     member depth
 *     namespace/member chain length
 *     argument count
 *     generic argument count
 *     collection dimensions
 *     tensor rank
 *     qubit count
 *     resource count
 *     device count
 *     machine size
 *
 * "Infinity" means that this grammar introduces no artificial finite
 * language-level limit. Actual parser/compiler/runtime resource limits remain
 * implementation and deployment concerns.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     expression
 *          |
 *          v
 *     postfixExpression
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
 *          v
 *     canonical semantic representation
 *          |
 *          +---------------------+-----------------------+
 *          |                     |                       |
 *          v                     v                       v
 *     classical IR          quantum::ir            HDL/hardware IR
 *          |                     |                       |
 *          +---------------------+-----------------------+
 *                                |
 *                                v
 *                    optimization / lowering
 *                                |
 *                    routing / scheduling
 *                                |
 *                    resilience / QEC / ZQN
 *                                |
 *                               HAL
 *                                |
 *                         target realization
 *
 * IMPORTANT:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT introduce:
 *
 *     - a quantum AST;
 *     - a quantum postfix IR;
 *     - a gate enumeration;
 *     - physical qubit identifiers;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC implementation;
 *     - ZQN implementation;
 *     - HAL implementation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - postfix-expression composition;
 *     - postfix chaining;
 *     - postfix operation ordering;
 *     - the public postfixExpression rule;
 *     - the postfixPart dispatch boundary;
 *     - member-selection syntax at the postfix level;
 *     - optional/member postfix syntax where defined by Zamani;
 *     - postfix increment/decrement syntax where supported;
 *     - compatibility aliases for the old postfix rule names;
 *     - integration boundaries for call and indexing components.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifiers;
 *     - qualified-name lexical spelling;
 *     - primary expressions;
 *     - literals;
 *     - arithmetic precedence;
 *     - logical precedence;
 *     - assignment precedence;
 *     - conditional expressions;
 *     - range expressions;
 *     - function declarations;
 *     - lambda declarations;
 *     - type definitions;
 *     - semantic type checking;
 *     - overload resolution;
 *     - name resolution;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capability checking;
 *     - resource allocation;
 *     - hardware selection;
 *     - quantum operation semantics;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - runtime execution;
 *     - ABI selection.
 *
 * ============================================================================
 * SINGLE EXPRESSION-HIERARCHY AUTHORITY
 * ============================================================================
 *
 * The canonical expression hierarchy is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * during the current repository migration.
 *
 * The intended canonical public hierarchy is:
 *
 *     expression
 *       -> assignmentExpression
 *       -> conditionalExpression
 *       -> rangeExpression
 *       -> logicalOrExpression
 *       -> logicalAndExpression
 *       -> bitwiseOrExpression
 *       -> bitwiseXorExpression
 *       -> bitwiseAndExpression
 *       -> equalityExpression
 *       -> relationalExpression
 *       -> shiftExpression
 *       -> additiveExpression
 *       -> multiplicativeExpression
 *       -> prefixExpression
 *       -> postfixExpression
 *       -> primaryExpression
 *
 * This file MUST NOT define another `expression` rule.
 *
 * This file MUST NOT define another precedence hierarchy.
 *
 * ============================================================================
 * MODULAR COMPOSITION RULE
 * ============================================================================
 *
 * The repository contains specialized expression contracts including:
 *
 *     grammar/expressions/calls.g4
 *     grammar/expressions/indexing.g4
 *     grammar/expressions/lambdas.g4
 *
 * Those files own their specialized syntax.
 *
 * However, the current versions of calls.g4 and indexing.g4 refer back to
 * generic expression contracts. Directly importing them here while this file
 * is itself imported by the canonical expression grammar would create a
 * parser dependency cycle.
 *
 * Therefore this file establishes explicit integration boundaries:
 *
 *     postfixExpression
 *         |
 *         +--> postfixCall
 *         +--> postfixIndex
 *         +--> postfixMember
 *         +--> postfixUpdate
 *
 * The final composition layer binds those boundaries to the canonical
 * specialized implementations.
 *
 * The names and contracts below are intentionally stable.
 *
 * No duplicate complete expression grammar is embedded here.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * All tokens come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This parser grammar defines NO lexer rules.
 *
 * It MUST NOT define:
 *
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     DOT
 *     DOUBLE_COLON
 *     COMMA
 *     QUESTION
 *     COLON
 *     PLUS_PLUS
 *     MINUS_MINUS
 *
 * locally.
 *
 * The canonical lexer remains the single lexical authority.
 *
 * ============================================================================
 * POSTFIX SEMANTIC MODEL
 * ============================================================================
 *
 * A postfix chain:
 *
 *     base a b c d
 *
 * is syntactically an ordered sequence:
 *
 *     base
 *       -> a
 *       -> b
 *       -> c
 *       -> d
 *
 * The AST MUST preserve this order.
 *
 * Semantic analysis determines what each operation means.
 *
 * For example:
 *
 *     value[index]
 *
 * may semantically mean:
 *
 *     array indexing
 *     tensor indexing
 *     map lookup
 *     string indexing
 *     memory access
 *     symbolic access
 *     quantum-register selection
 *     hardware-resource selection
 *     user-defined indexing
 *
 * The grammar does not choose among those meanings.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Postfix syntax is target independent.
 *
 * This grammar does not select:
 *
 *     CPU
 *     CPU core
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     physical qubit
 *     memory bank
 *     accelerator
 *     network node
 *     topology
 *     device identifier
 *
 * For example:
 *
 *     accelerator.run(data)
 *
 * is source-level invocation syntax.
 *
 * It does NOT mean:
 *
 *     use accelerator 0
 *
 * or:
 *
 *     use a fixed number of accelerator resources.
 *
 * Resource realization belongs downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum operations may appear through ordinary expression/postfix syntax.
 *
 * Examples:
 *
 *     H(q)
 *     measure(q)
 *     reset(q)
 *     circuit.apply(operation, q)
 *     operation(parameters)(q)
 *
 * The grammar does NOT enumerate:
 *
 *     X
 *     Y
 *     Z
 *     H
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     ...
 *
 * Those names remain identifiers unless the canonical lexical specification
 * explicitly reserves them.
 *
 * A postfix expression involving quantum values remains an ordinary
 * domain-neutral source expression.
 *
 * Semantic lowering may subsequently produce:
 *
 *     quantum::ir
 *
 * which then participates in:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     QEC
 *     resilience
 *     ZQN
 *     HAL
 *
 * No physical realization is encoded here.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same postfix mechanism is valid for:
 *
 *     classical values;
 *     vectors;
 *     matrices;
 *     tensors;
 *     streams;
 *     records;
 *     services;
 *     distributed objects;
 *     hardware abstractions;
 *     HDL-related semantic objects;
 *     accelerators;
 *     quantum objects;
 *     future computational domains.
 *
 * Examples:
 *
 *     tensor[i, j]
 *     matrix[row][column]
 *     service.request(data)
 *     accelerator.run(kernel)
 *     device.configure(config)
 *     signal.value
 *
 * No target-specific interpretation occurs in this file.
 *
 * ============================================================================
 * PRIMARY EXPRESSION CONTRACT
 * ============================================================================
 *
 * `primaryExpression` is owned by the foundational expression grammar.
 *
 * It may represent:
 *
 *     identifiers
 *     literals
 *     parenthesized expressions
 *     lambdas
 *     constructed values
 *     future expression atoms
 *
 * This file consumes it but does not redefine it.
 *
 * ============================================================================
 * POSTFIX OPERATION ORDER
 * ============================================================================
 *
 * The order of postfix operations is semantically significant.
 *
 * Example:
 *
 *     a[i].field(x)[j]
 *
 * MUST remain structurally equivalent to:
 *
 *     a
 *       -> [i]
 *       -> .field
 *       -> (x)
 *       -> [j]
 *
 * The parser MUST NOT reorder these operations.
 *
 * ============================================================================
 * MEMBER ACCESS
 * ============================================================================
 *
 * A member access is a postfix operation:
 *
 *     value.field
 *
 * Member names are resolved semantically.
 *
 * This grammar does not know whether the member is:
 *
 *     a field
 *     method
 *     property
 *     associated item
 *     capability
 *     hardware abstraction
 *     quantum operation
 *     distributed service
 *     future language construct
 *
 * ============================================================================
 * QUALIFIED / NAMESPACE ACCESS
 * ============================================================================
 *
 * Qualified names and namespace paths remain owned by the canonical name/path
 * grammar.
 *
 * This file does not create a second qualified-name syntax.
 *
 * A qualified/member separator such as:
 *
 *     ::
 *
 * may participate in source-level selection where the canonical language
 * contract permits it.
 *
 * Its exact semantic interpretation is determined by name resolution.
 *
 * ============================================================================
 * OPTIONAL MEMBER ACCESS
 * ============================================================================
 *
 * If the canonical lexer exposes `QUESTION` and `DOT`, optional member
 * selection may be represented as:
 *
 *     value?.field
 *
 * This is syntactic structure only.
 *
 * Nullability/optional semantics belong to type and semantic analysis.
 *
 * If optional member access is not enabled by the active language version,
 * compatibility/version validation rejects it downstream.
 *
 * ============================================================================
 * POSTFIX UPDATE
 * ============================================================================
 *
 * If the canonical language exposes:
 *
 *     PLUS_PLUS
 *     MINUS_MINUS
 *
 * postfix update syntax may be represented as:
 *
 *     value++
 *     value--
 *
 * Whether a value is mutable, assignable, linear, affine, atomic, quantum,
 * hardware-backed, or otherwise writable is a semantic question.
 *
 * This grammar does not determine whether the operation is legal.
 *
 * ============================================================================
 * CALL INTEGRATION
 * ============================================================================
 *
 * Calls are owned by:
 *
 *     grammar/expressions/calls.g4
 *
 * The canonical call contract includes:
 *
 *     f()
 *     f(x)
 *     f(x, y)
 *     f(x,)
 *     f(name = value)
 *     f(name: value)
 *     f(...values)
 *     f::<T>(x)
 *
 * This file treats a call as a postfix operation.
 *
 * It does not duplicate the argument grammar.
 *
 * The final parser composition must bind:
 *
 *     postfixCall
 *
 * to the canonical call implementation.
 *
 * ============================================================================
 * INDEXING INTEGRATION
 * ============================================================================
 *
 * Indexing is owned by:
 *
 *     grammar/expressions/indexing.g4
 *
 * Canonical forms include:
 *
 *     value[i]
 *     value[i, j]
 *     value[i][j]
 *     tensor[i, j, k]
 *     value[start:end]
 *     value[start:end:step]
 *
 * This file treats indexing as a postfix operation.
 *
 * It does not define collection dimensionality.
 *
 * ============================================================================
 * NO FIXED ARITY
 * ============================================================================
 *
 * The following are all intentionally open:
 *
 *     postfix chain length
 *     argument count
 *     generic argument count
 *     index count
 *     member chain length
 *     namespace depth
 *
 * There are no grammar constants such as:
 *
 *     MAX_POSTFIX
 *     MAX_ARGUMENTS
 *     MAX_INDEXES
 *     MAX_MEMBER_DEPTH
 *     MAX_CALL_DEPTH
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     token sequence
 *     active grammar/version contract
 *
 * It MUST NOT depend on:
 *
 *     system time
 *     randomness
 *     environment variables
 *     filesystem contents
 *     network state
 *     hardware discovery
 *     installed devices
 *     runtime scheduler state
 *     compiler backend availability
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a postfix expression MUST NOT execute the referenced operation.
 *
 * For example:
 *
 *     system.run(command)
 *
 * is syntax only.
 *
 * The parser MUST NOT:
 *
 *     execute commands;
 *     open files;
 *     contact networks;
 *     load plugins;
 *     inspect devices;
 *     access credentials;
 *     invoke hardware;
 *     invoke a quantum backend;
 *     invoke an HDL simulator;
 *     invoke a compiler backend.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every postfix operation MUST lower into the domain-neutral frontend AST.
 *
 * Conceptually:
 *
 *     PostfixExpression {
 *         base,
 *         operations,
 *         source_span
 *     }
 *
 * where operations preserve source order.
 *
 * Conceptual operation forms include:
 *
 *     Call { ... }
 *     Index { ... }
 *     Member { ... }
 *     QualifiedMember { ... }
 *     OptionalMember { ... }
 *     PostfixUpdate { ... }
 *
 * The exact Rust representation belongs to:
 *
 *     src/frontend/ast/
 *
 * and MUST NOT be invented by this grammar.
 *
 * Source spans must cover the complete syntactic construct and retain the
 * spans necessary for diagnostics and source-preserving tooling.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     name resolution;
 *     member resolution;
 *     overload resolution;
 *     callable resolution;
 *     generic inference;
 *     argument binding;
 *     indexability checking;
 *     range checking;
 *     mutability checking;
 *     ownership checking;
 *     borrowing;
 *     lifetime validation;
 *     effect checking;
 *     capability checking;
 *     resource checking;
 *     quantum legality;
 *     hardware capability requirements;
 *     distributed placement requirements.
 *
 * Syntax alone MUST NOT decide these properties.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not directly produce IR.
 *
 * The downstream chain is:
 *
 *     postfix syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic model
 *          |
 *          +-------------------------+
 *          |            |            |
 *          v            v            v
 *     classical IR  quantum::ir  HDL/hardware IR
 *
 * Quantum postfix operations MUST NOT create a separate quantum IR.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may subsequently:
 *
 *     inline
 *     specialize
 *     devirtualize
 *     vectorize
 *     distribute
 *     fuse
 *     lower
 *     route
 *     schedule
 *     map
 *     optimize
 *
 * Such transformations MUST NOT be encoded into this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * A parsed postfix expression is not runtime execution.
 *
 * Runtime realization may use:
 *
 *     static dispatch
 *     dynamic dispatch
 *     capability negotiation
 *     resource negotiation
 *     distributed service resolution
 *     hardware abstraction
 *     quantum backend selection
 *
 * without changing source syntax.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical postfix layer must represent existing source constructs such
 * as:
 *
 *     value
 *     value()
 *     value(x)
 *     value(x, y)
 *     value(x,)
 *     value[index]
 *     value[i, j]
 *     value.field
 *     value.field(x)
 *     value[index].field
 *     value()[index]
 *     value[index](x)
 *
 * and, where enabled by the language contract:
 *
 *     value?.field
 *     value++
 *     value--
 *     namespace::value
 *     namespace::value(x)
 *
 * Compatibility policy for individual spellings is owned by:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser should report structural errors through the normal ANTLR parser
 * error mechanism consumed by the Zamani frontend.
 *
 * This grammar MUST NOT embed diagnostic side effects.
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples of syntactic errors include:
 *
 *     value[
 *     value.
 *     value(
 *     value(,
 *     value[index
 *     value..field
 *
 * Examples of semantic errors include:
 *
 *     indexing a non-indexable value;
 *     selecting a nonexistent member;
 *     calling a non-callable value;
 *     assigning to an immutable value;
 *     using an invalid quantum operand;
 *     violating an effect;
 *     lacking a required capability.
 *
 * ============================================================================
 * PERFORMANCE / SCALABILITY
 * ============================================================================
 *
 * The grammar uses iterative repetition rather than a recursively nested
 * postfix rule for the common chain:
 *
 *     primary postfixPart*
 *
 * This makes the grammar structurally appropriate for long postfix chains
 * without introducing an artificial finite bound.
 *
 * Implementation-specific parser-stack, token-buffer, memory, and time
 * limits remain implementation concerns.
 *
 * They MUST NOT become language semantics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_POSTFIX
 *     MAX_ARGUMENTS
 *     MAX_INDEXES
 *     MAX_MEMBER_DEPTH
 *     MAX_CALL_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_VECTOR_WIDTH
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *
 * It contains none of these concepts as language limits.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The corresponding conformance suite must test at least:
 *
 * POSITIVE
 *
 *     x
 *     x()
 *     x(a)
 *     x(a, b)
 *     x(a,)
 *     x[0]
 *     x[i, j]
 *     x[i][j]
 *     x.field
 *     x.field()
 *     x[i].field(j)[k]
 *     f(...args)
 *     H(q)
 *     measure(q)
 *     accelerator.run(data)
 *
 * CHAINING
 *
 *     a()[i].b(c)[j].d()
 *
 *     f()(x)(y)[z].field()
 *
 * BOUNDARY / SCALABILITY
 *
 *     arbitrarily long postfix chains;
 *     arbitrarily many arguments;
 *     arbitrarily many indexes;
 *     arbitrarily deep member paths;
 *     arbitrarily many generic arguments.
 *
 * NEGATIVE
 *
 *     x(
 *     x[
 *     x.
 *     x(,)
 *     x(,a)
 *     x[a
 *
 * The test suite MUST NOT establish an artificial maximum chain length.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] postfixExpression is the single canonical postfix entry point;
 *     [ ] no second expression hierarchy exists here;
 *     [ ] no lexer rules exist here;
 *     [ ] no semantic actions exist here;
 *     [ ] no embedded Rust exists here;
 *     [ ] no unsafe implementation dependency exists;
 *     [ ] call syntax has a defined integration boundary;
 *     [ ] indexing syntax has a defined integration boundary;
 *     [ ] member syntax is defined here;
 *     [ ] postfix update syntax is version-gated/compatible;
 *     [ ] source order is preserved;
 *     [ ] arbitrary postfix chains are representable;
 *     [ ] no hardware limits are encoded;
 *     [ ] no quantum gate inventory is encoded;
 *     [ ] AST mapping is predetermined;
 *     [ ] semantic ownership is predetermined;
 *     [ ] IR ownership is predetermined;
 *     [ ] quantum lowering terminates at quantum::ir;
 *     [ ] compiler/runtime ownership is predetermined;
 *     [ ] diagnostics ownership is predetermined;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] the final ANTLR composition binds the integration boundaries.
 *
 * ============================================================================
 */

parser grammar Postfix;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. CANONICAL PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * A postfix expression is one primary expression followed by zero or more
 * postfix operations.
 *
 * This is intentionally iterative:
 *
 *     primaryExpression postfixPart*
 *
 * rather than:
 *
 *     postfixExpression
 *         : postfixExpression postfixPart
 *         | primaryExpression
 *         ;
 *
 * The iterative form gives the parser generator a clear postfix boundary and
 * avoids introducing unnecessary left-recursive structure at this level.
 */
postfixExpression
    : primaryExpression postfixPart*
    ;


/* ============================================================================
 * 2. POSTFIX DISPATCH
 * ========================================================================== */

/*
 * The alternatives are syntactic categories, not semantic categories.
 *
 * The parser does not decide whether an operation is:
 *
 *     method invocation
 *     quantum operation
 *     hardware operation
 *     distributed operation
 *     collection access
 *
 * Those meanings are resolved downstream.
 */
postfixPart
    : postfixCall
    | postfixIndex
    | postfixMember
    | postfixOptionalMember
    | postfixQualifiedMember
    | postfixUpdate
    ;


/* ============================================================================
 * 3. CALL INTEGRATION BOUNDARY
 * ========================================================================== */

/*
 * This boundary intentionally has its own name.
 *
 * The current repository's calls.g4 owns the detailed argument grammar.
 *
 * During final parser composition, `postfixCall` MUST be bound to that
 * canonical call implementation.
 *
 * The compatibility implementation below mirrors the canonical call syntax
 * so this file remains independently understandable and structurally
 * complete while the parser-composition migration is in progress.
 *
 * The argument semantics remain downstream.
 */
postfixCall
    : callTypeArguments?
      LPAREN
      postfixArgumentList?
      RPAREN
    ;


postfixArgumentList
    : postfixArgument
      (
          COMMA
          postfixArgument
      )*
      COMMA?
    ;


postfixArgument
    : postfixNamedArgument
    | postfixSpreadArgument
    | postfixPositionalArgument
    ;


postfixPositionalArgument
    : expression
    ;


postfixNamedArgument
    : identifier
      (
          ASSIGN
        | COLON
      )
      expression
    ;


postfixSpreadArgument
    : ELLIPSIS
      expression
    ;


/*
 * Explicit call-site generic arguments.
 *
 * These remain syntactic.
 *
 * Type validity is semantic.
 */
callTypeArguments
    : DOUBLE_COLON
      LESS_THAN
      callTypeArgumentList
      GREATER_THAN
    ;


callTypeArgumentList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      COMMA?
    ;


/* ============================================================================
 * 4. INDEX INTEGRATION BOUNDARY
 * ========================================================================== */

/*
 * Indexing remains a postfix operation.
 *
 * This local composition rule deliberately delegates the meaning of the
 * contents to the canonical expression rule.
 *
 * It supports:
 *
 *     x[i]
 *     x[i, j]
 *     x[i][j]
 *     x[start:end]
 *     x[start:end:step]
 *
 * The number of indexes is not fixed.
 */
postfixIndex
    : LBRACKET postfixIndexArgumentList? RBRACKET
    ;


postfixIndexArgumentList
    : postfixIndexArgument
      (
          COMMA
          postfixIndexArgument
      )*
    ;


postfixIndexArgument
    : postfixIndexRange
    | expression
    ;


postfixIndexRange
    : expression?
      COLON
      expression?
      (
          COLON
          expression?
      )?
    ;


/* ============================================================================
 * 5. MEMBER ACCESS
 * ========================================================================== */

/*
 * Ordinary member selection:
 *
 *     value.field
 *
 * The identifier is resolved semantically.
 */
postfixMember
    : DOT
      identifier
    ;


/* ============================================================================
 * 6. OPTIONAL MEMBER ACCESS
 * ========================================================================== */

/*
 * Optional member selection:
 *
 *     value?.field
 *
 * This is syntactic structure only.
 *
 * Nullability/optional propagation is a semantic/type-system responsibility.
 *
 * If QUESTION is not enabled by the active language version, compatibility
 * validation rejects the construct.
 */
postfixOptionalMember
    : QUESTION
      DOT
      identifier
    ;


/* ============================================================================
 * 7. QUALIFIED MEMBER / ASSOCIATED SELECTION
 * ========================================================================== */

/*
 * Qualified selection:
 *
 *     value::member
 *
 * This rule is deliberately limited to the postfix separator itself.
 *
 * The complete namespace/qualified-name system remains owned by the canonical
 * names/path grammar.
 *
 * The semantic layer decides whether `::` denotes:
 *
 *     namespace selection
 *     associated item selection
 *     type-associated item
 *     module member
 *     dialect extension
 *     another language-defined selection
 */
postfixQualifiedMember
    : DOUBLE_COLON
      identifier
    ;


/* ============================================================================
 * 8. POSTFIX UPDATE
 * ========================================================================== */

/*
 * Postfix update is represented only when the canonical lexer provides the
 * corresponding tokens.
 *
 * The semantic layer determines:
 *
 *     mutability
 *     assignability
 *     ownership
 *     borrowing
 *     atomicity
 *     effect legality
 *     resource legality
 *
 * The grammar does not impose those restrictions.
 */
postfixUpdate
    : PLUS_PLUS
    | MINUS_MINUS
    ;


/* ============================================================================
 * 9. COMPATIBILITY ALIASES
 * ========================================================================== */

/*
 * Existing parser/tooling code may refer to `callExpression`.
 *
 * Preserve the name as a compatibility boundary without creating another
 * implementation.
 */
callExpression
    : postfixCall
    ;


/*
 * Existing tooling may refer to `indexExpression`.
 *
 * Preserve the alias without creating another indexing hierarchy.
 */
indexExpression
    : postfixIndex
    ;


/*
 * Existing tooling may refer to `memberExpression`.
 *
 * This alias represents ordinary member selection only.
 */
memberExpression
    : postfixMember
    ;


/* ============================================================================
 * 10. SOURCE-ORDER CONTRACT
 * ========================================================================== */

/*
 * The following:
 *
 *     a[i].field(x)[j]
 *
 * MUST be represented structurally as:
 *
 *     primary(a)
 *     index(i)
 *     member(field)
 *     call(x)
 *     index(j)
 *
 * The parser MUST NOT:
 *
 *     reorder operations;
 *     flatten semantically;
 *     perform overload resolution;
 *     perform constant folding;
 *     perform call dispatch;
 *     perform index resolution.
 *
 * Those actions belong downstream.
 */


/* ============================================================================
 * 11. CALL / INDEX / MEMBER COMPOSITION EXAMPLES
 * ========================================================================== */

/*
 * Valid structural examples:
 *
 *     value
 *
 *     value()
 *
 *     value(x)
 *
 *     value(x, y)
 *
 *     value(x,)
 *
 *     value(name = x)
 *
 *     value(name: x)
 *
 *     value(...items)
 *
 *     value::<T>(x)
 *
 *     value[i]
 *
 *     value[i, j]
 *
 *     value[i][j]
 *
 *     value[start:end]
 *
 *     value[start:end:step]
 *
 *     value.field
 *
 *     value.field()
 *
 *     value?.field
 *
 *     value::member
 *
 *     value[i].field(x)[j]
 *
 *     value()[i].field()
 *
 * The exact semantic legality of each construct is determined downstream.
 */


/* ============================================================================
 * 12. OPEN-WORLD CALLABILITY
 * ========================================================================== */

/*
 * A postfix call does not require a statically named function.
 *
 * Examples:
 *
 *     f(x)
 *     make_function()(x)
 *     values[i](x)
 *     object.method(x)
 *     callable.field(x)
 *
 * The callable may eventually resolve to:
 *
 *     function
 *     method
 *     closure
 *     lambda
 *     generic callable
 *     user-defined call capability
 *     quantum operation abstraction
 *     accelerator abstraction
 *     distributed service abstraction
 *     future callable domain
 *
 * The grammar intentionally does not enumerate callable kinds.
 */


/* ============================================================================
 * 13. OPEN-WORLD INDEXABILITY
 * ========================================================================== */

/*
 * An index operation does not imply a particular data structure.
 *
 * Possible semantic meanings include:
 *
 *     array
 *     vector
 *     matrix
 *     tensor
 *     map
 *     slice
 *     string
 *     memory
 *     quantum register
 *     symbolic object
 *     user-defined indexable abstraction
 *
 * The grammar does not encode a fixed collection hierarchy.
 */


/* ============================================================================
 * 14. QUANTUM EXAMPLES
 * ========================================================================== */

/*
 * The following are syntactically ordinary postfix expressions:
 *
 *     H(q)
 *
 *     measure(q)
 *
 *     reset(q)
 *
 *     circuit.apply(operation, q)
 *
 *     circuit.operations[i](q)
 *
 *     operation(parameters)(q)
 *
 * No gate table is embedded here.
 *
 * No physical qubit numbering is embedded here.
 *
 * No topology is embedded here.
 *
 * No qubit limit is embedded here.
 *
 * Semantic analysis determines whether a source operation is quantum and
 * lowers it to the canonical quantum::ir.
 */


/* ============================================================================
 * 15. HARDWARE / HDL EXAMPLES
 * ========================================================================== */

/*
 * These remain target-independent syntax:
 *
 *     accelerator.run(data)
 *
 *     device.configure(config)
 *
 *     signal.value
 *
 *     memory[address]
 *
 *     kernel(arguments)
 *
 * The parser does not determine whether the implementation uses:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     embedded device
 *     remote service
 *     distributed node
 *     future accelerator
 *
 * Target realization belongs downstream.
 */


/* ============================================================================
 * 16. SEMANTIC BOUNDARY
 * ========================================================================== */

/*
 * This grammar deliberately accepts syntactically structured postfix forms
 * without trying to answer semantic questions.
 *
 * Examples:
 *
 *     x.field
 *     x()
 *     x[i]
 *
 * can only be interpreted after:
 *
 *     name resolution
 *     type resolution
 *     callable resolution
 *     member resolution
 *     indexability checking
 *
 * This separation is necessary for POCO-REAF.
 */


/* ============================================================================
 * 17. AST LOWERING CONTRACT
 * ========================================================================== */

/*
 * Conceptual AST:
 *
 *     PostfixExpression {
 *         base: Expression,
 *         operations: [PostfixOperation],
 *         source_span: Span
 *     }
 *
 * Conceptual operations:
 *
 *     Call {
 *         generic_arguments,
 *         arguments,
 *         source_span
 *     }
 *
 *     Index {
 *         indexes,
 *         source_span
 *     }
 *
 *     Member {
 *         name,
 *         source_span
 *     }
 *
 *     OptionalMember {
 *         name,
 *         source_span
 *     }
 *
 *     QualifiedMember {
 *         name,
 *         source_span
 *     }
 *
 *     PostfixUpdate {
 *         operator,
 *         source_span
 *     }
 *
 * These are conceptual contracts only.
 *
 * The actual Rust AST remains owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar must not introduce Rust AST implementation types.
 */


/* ============================================================================
 * 18. CANONICAL QUANTUM IR BOUNDARY
 * ========================================================================== */

/*
 * A quantum call/index/member expression follows:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum operation/value
 *       |
 *       v
 *     quantum::ir
 *
 * Never:
 *
 *     parser
 *       |
 *       v
 *     custom quantum postfix IR
 *
 * This preserves the repository's canonical quantum::ir boundary.
 */


/* ============================================================================
 * 19. RESOURCE / CAPABILITY BOUNDARY
 * ========================================================================== */

/*
 * Postfix syntax may eventually reference values representing:
 *
 *     capabilities
 *     resources
 *     devices
 *     accelerators
 *     memories
 *     services
 *     quantum resources
 *
 * The syntax itself does not select physical resources.
 *
 * Semantic/resource analysis determines:
 *
 *     required capabilities
 *     resource requirements
 *     constraints
 *     preferences
 *     placement intent
 *
 * Compilation/runtime determine realization.
 */


/* ============================================================================
 * 20. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Given identical:
 *
 *     source tokens
 *     grammar version
 *
 * the syntactic parse must be deterministic.
 *
 * No postfix parse may depend on:
 *
 *     time
 *     randomness
 *     environment
 *     hardware
 *     filesystem
 *     network
 *     runtime scheduler
 *     installed compiler backends
 */


/* ============================================================================
 * 21. NO EXECUTION CONTRACT
 * ========================================================================== */

/*
 * These are syntax:
 *
 *     system.run(...)
 *     device.configure(...)
 *     accelerator.execute(...)
 *     qpu.measure(...)
 *
 * Parsing them MUST NOT execute anything.
 *
 * The parser has no:
 *
 *     filesystem access
 *     network access
 *     subprocess access
 *     credential access
 *     hardware access
 *     runtime access
 *
 * and contains no embedded actions or predicates.
 */


/* ============================================================================
 * 22. SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * These forms are intentionally unbounded by language semantics:
 *
 *     a.b.c.d.e.f...
 *
 *     f()()()()...
 *
 *     a[0][1][2]...
 *
 *     a[i, j, k, ...]
 *
 *     f(a, b, c, ...)
 *
 *     f::<T, U, V, ...>(...)
 *
 * The grammar introduces no finite maximum.
 *
 * Parser implementation limits are not language semantics.
 */


/* ============================================================================
 * 23. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * Forbidden universal language limits include:
 *
 *     MAX_POSTFIX
 *     MAX_CALLS
 *     MAX_ARGUMENTS
 *     MAX_INDEXES
 *     MAX_MEMBERS
 *     MAX_CHAIN
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_VECTOR_WIDTH
 *     MAX_DEVICES
 *
 * None are encoded by this grammar.
 */


/* ============================================================================
 * 24. TEST CONTRACT
 * ========================================================================== */

/*
 * Positive:
 *
 *     x
 *     x()
 *     x(a)
 *     x(a, b)
 *     x(a,)
 *     x(name = value)
 *     x(name: value)
 *     x(...values)
 *     x::<T>(value)
 *     x[i]
 *     x[i, j]
 *     x[i][j]
 *     x[start:end]
 *     x[start:end:step]
 *     x.field
 *     x.field()
 *     x?.field
 *     x::member
 *
 * Composition:
 *
 *     x[i].field(a)[j]
 *
 *     make()(x)[i].field()
 *
 *     namespace::factory::<T>(x)[i].run()
 *
 * Quantum:
 *
 *     H(q)
 *     measure(q)
 *     reset(q)
 *     circuit.apply(operation, q)
 *
 * Hardware/HDL:
 *
 *     accelerator.run(data)
 *     device.configure(config)
 *     signal.value
 *     memory[address]
 *
 * Negative:
 *
 *     x(
 *     x[
 *     x.
 *     x(,)
 *     x(,a)
 *     x[a
 *
 * Boundary:
 *
 *     arbitrarily long postfix chains;
 *     arbitrarily many arguments;
 *     arbitrarily many indexes;
 *     arbitrarily many generic arguments;
 *     arbitrarily deep member selection.
 *
 * The conformance suite MUST NOT define an artificial upper bound as a
 * language rule.
 */


/* ============================================================================
 * 25. INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * Upstream:
 *
 *     primaryExpression
 *
 * is supplied by the foundational expression grammar.
 *
 * Downstream:
 *
 *     prefixExpression
 *
 * consumes:
 *
 *     postfixExpression
 *
 * The complete expression grammar therefore composes:
 *
 *     prefixExpression
 *         -> postfixExpression
 *         -> primaryExpression postfixPart*
 *
 * Specialized contracts:
 *
 *     calls.g4
 *         -> call syntax
 *
 *     indexing.g4
 *         -> indexing syntax
 *
 *     identifiers/name grammar
 *         -> identifier/name syntax
 *
 *     types grammar
 *         -> typeExpression
 *
 *     lexer
 *         -> tokens
 *
 * This file remains the postfix composition boundary.
 */


/* ============================================================================
 * 26. MIGRATION CONTRACT
 * ========================================================================== */

/*
 * The current repository contains inline postfix rules in the broader
 * expression grammar.
 *
 * During modularization:
 *
 *     old inline postfix rules
 *              |
 *              v
 *     this Postfix.postfixExpression
 *
 * The old definitions must not remain independently authoritative.
 *
 * Likewise, calls.g4 and indexing.g4 must remain specialized contracts rather
 * than becoming competing public expression hierarchies.
 *
 * No source-level semantic feature should be silently removed during this
 * migration.
 */


/* ============================================================================
 * 27. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete when:
 *
 *     [x] It has one canonical postfixExpression entry point.
 *     [x] It has no competing expression hierarchy.
 *     [x] It contains no lexer rules.
 *     [x] It contains no embedded Rust actions.
 *     [x] It contains no semantic predicates.
 *     [x] It performs no execution.
 *     [x] It performs no hardware discovery.
 *     [x] It imposes no machine/resource limits.
 *     [x] It permits arbitrary postfix chaining.
 *     [x] It preserves postfix operation order.
 *     [x] It defines member postfix syntax.
 *     [x] It defines call integration.
 *     [x] It defines index integration.
 *     [x] It defines optional-member integration.
 *     [x] It defines qualified-member integration.
 *     [x] It preserves compatibility aliases.
 *     [x] It documents the AST contract.
 *     [x] It documents the semantic contract.
 *     [x] It documents the IR contract.
 *     [x] It preserves the quantum::ir boundary.
 *     [x] It documents compiler/runtime integration.
 *     [x] It documents deterministic parsing.
 *     [x] It documents safe-Rust integration requirements.
 *     [x] It documents positive/negative/boundary/scalability tests.
 *
 * Remaining repository integration work is deliberately assigned to the
 * composition layer rather than hidden inside this file.
 */