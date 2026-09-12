/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/identifiers.g4
 *
 * Role:
 *     Canonical expression-layer integration for identifiers and qualified
 *     names.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions.
 *     No unsafe code is used or required.
 *     Rust components integrating this grammar MUST remain compatible with
 *     Rust 1.97 / Rust 1.97.1 and MUST enforce:
 *
 *         #![forbid(unsafe_code)]
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source text
 *         |
 *         v
 *     ZamaniLexer
 *         |
 *         |-- IDENTIFIER
 *         |
 *         v
 *     core/names.g4
 *         |
 *         |-- identifier
 *         |-- simpleName
 *         |-- qualifiedName
 *         |
 *         v
 *     expressions/identifiers.g4       <-- THIS FILE
 *         |
 *         |-- identifierExpression
 *         |-- qualifiedNameExpression
 *         |
 *         v
 *     expressions/expressions.g4
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic name resolution
 *         |
 *         v
 *     semantic model / canonical IR
 *
 * ============================================================================
 *
 * CORE OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the expression-level interpretation of a canonical simple identifier;
 *   - the expression-level interpretation of a canonical qualified name;
 *   - the bridge between core name syntax and expression syntax;
 *   - stable expression rule names used by expressions.g4;
 *   - preservation of the distinction between:
 *
 *         identifier expression
 *         qualified-name expression
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical identifier spelling;
 *   - IDENTIFIER token definition;
 *   - Unicode identifier rules;
 *   - Unicode normalization;
 *   - keyword definitions;
 *   - reserved-word policy;
 *   - scopes;
 *   - symbol tables;
 *   - name resolution;
 *   - namespaces;
 *   - modules;
 *   - packages;
 *   - filesystem paths;
 *   - URLs;
 *   - member access;
 *   - method calls;
 *   - generic arguments;
 *   - types;
 *   - type checking;
 *   - constants;
 *   - quantum resources;
 *   - logical qubits;
 *   - physical qubits;
 *   - hardware devices;
 *   - accelerators;
 *   - distributed nodes;
 *   - runtime handles;
 *   - resource allocation;
 *   - target selection;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - simulation;
 *   - canonical IR.
 *
 * ============================================================================
 *
 * SINGLE SOURCE OF TRUTH
 * ============================================================================
 *
 * Identifier lexical syntax is owned by:
 *
 *     grammar/lexer/identifiers.g4
 *
 * The canonical lexer assembles that lexical contract into:
 *
 *     IDENTIFIER
 *
 * Parser-level name syntax is owned by:
 *
 *     grammar/core/names.g4
 *
 * That grammar provides the canonical:
 *
 *     simpleName
 *     identifier
 *     qualifiedName
 *
 * rules.
 *
 * This file MUST consume those rules rather than redefining them.
 *
 * In particular, this file MUST NOT introduce:
 *
 *     IDENT
 *
 * or another identifier token.
 *
 * The repository's canonical token is:
 *
 *     IDENTIFIER
 *
 * ============================================================================
 *
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * An expression grammar needs to distinguish syntactically between:
 *
 *     value
 *
 * and:
 *
 *     namespace::value
 *
 * without taking ownership of name semantics.
 *
 * The distinction is useful to the expression parser because:
 *
 *     value
 *
 * can begin an ordinary identifier expression, while:
 *
 *     namespace::value
 *
 * can begin a qualified-name expression.
 *
 * The parser does not determine whether a qualified name denotes:
 *
 *     - a variable;
 *     - a function;
 *     - a type;
 *     - a module;
 *     - a namespace;
 *     - a constant;
 *     - a quantum operation;
 *     - a hardware capability;
 *     - an accelerator;
 *     - a distributed service;
 *     - a future language entity.
 *
 * That determination belongs to semantic analysis.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Identifier expressions are target-independent.
 *
 * The grammar does not encode:
 *
 *     - machine size;
 *     - processor count;
 *     - core count;
 *     - thread count;
 *     - GPU count;
 *     - FPGA count;
 *     - QPU count;
 *     - qubit count;
 *     - memory capacity;
 *     - register count;
 *     - vector width;
 *     - tensor dimensions;
 *     - network size;
 *     - cluster size;
 *     - hardware topology;
 *     - device identifiers;
 *     - physical addresses;
 *     - deployment topology.
 *
 * The same source-level name syntax therefore remains valid when the program
 * is compiled or executed on different classes or scales of machine.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no finite limits.
 *
 * There is deliberately no maximum for:
 *
 *     - identifier length;
 *     - qualified-name depth;
 *     - number of names in a surrounding expression;
 *     - number of expressions;
 *     - number of declarations;
 *     - number of modules;
 *     - number of namespaces;
 *     - number of resources;
 *     - number of devices;
 *     - number of qubits;
 *     - number of execution targets.
 *
 * Repetition limits, if any, are imposed by actual implementation/resource
 * availability rather than by source grammar.
 *
 * "Infinity" therefore means that the language imposes no artificial finite
 * machine-scale ceiling here; actual compilation and execution remain bounded
 * by available computational resources and explicitly defined implementation
 * policies.
 *
 * ============================================================================
 *
 * DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * Names are intentionally domain-neutral.
 *
 * All of the following are syntactically ordinary names:
 *
 *     q
 *     state
 *     H
 *     CNOT
 *     matrix
 *     tensor
 *     gpu
 *     fpga
 *     accelerator
 *     device
 *     node
 *     backend
 *     capability
 *     resource
 *     model
 *
 * Their meaning is assigned downstream.
 *
 * This prevents the expression grammar from becoming coupled to a fixed
 * vocabulary of quantum gates, hardware vendors, accelerators, processors,
 * devices, or future computing architectures.
 *
 * ============================================================================
 *
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum names remain ordinary names at this grammar layer.
 *
 * Examples:
 *
 *     q
 *     logical_q
 *     physical_q
 *     register
 *     H
 *     CNOT
 *     measure
 *     backend
 *
 * Whether a name denotes a quantum entity is determined downstream.
 *
 * This file MUST NOT:
 *
 *     - allocate qubits;
 *     - identify physical qubits;
 *     - select a QPU;
 *     - infer topology;
 *     - construct quantum::ir;
 *     - perform routing;
 *     - perform scheduling;
 *     - invoke QEC;
 *     - interpret ZQN faults.
 *
 * Quantum syntax eventually lowers through the repository's canonical semantic
 * boundary:
 *
 *     frontend AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *
 * ============================================================================
 *
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware-related names remain unresolved syntax.
 *
 * For example:
 *
 *     gpu::tensor
 *     fpga::kernel
 *     accelerator::matrix
 *     device::capability
 *
 * are syntactically name expressions.
 *
 * This file does not determine whether:
 *
 *     gpu
 *
 * identifies an actual GPU, a capability namespace, a library namespace, a
 * symbolic resource, or something else.
 *
 * Hardware discovery and target selection belong downstream.
 *
 * ============================================================================
 *
 * MEMBER ACCESS BOUNDARY
 * ============================================================================
 *
 * A qualified name is NOT the same thing as member access.
 *
 * These forms remain distinct:
 *
 *     module::value
 *
 * and:
 *
 *     object.value
 *
 * `::` in this file delegates to canonical qualified-name syntax.
 *
 * `.` remains owned by expression/member-access grammar.
 *
 * This prevents:
 *
 *     namespace::symbol
 *
 * from being accidentally conflated with:
 *
 *     object.member
 *
 * ============================================================================
 *
 * PATH BOUNDARY
 * ============================================================================
 *
 * A qualified name is not a filesystem path.
 *
 * This file therefore does not consume:
 *
 *     ./module
 *     ../module
 *     /absolute/path
 *     https://example
 *
 * Path syntax belongs to:
 *
 *     grammar/core/paths.g4
 *
 * A path may contain names, but the two concepts remain architecturally
 * separate.
 *
 * ============================================================================
 *
 * TYPE BOUNDARY
 * ============================================================================
 *
 * A qualified name can appear inside type syntax, but this file does not own
 * type expressions.
 *
 * For example:
 *
 *     math::Vector
 *
 * may be used by type grammar.
 *
 * Whether it is a type is determined by the type grammar and semantic
 * analysis.
 *
 * Generic syntax such as:
 *
 *     math::Vector<Int>
 *
 * remains owned by the type-expression grammar.
 *
 * ============================================================================
 *
 * FUNCTION / CALL BOUNDARY
 * ============================================================================
 *
 * This file identifies the name portion of an expression.
 *
 * It does not own calls.
 *
 * For example:
 *
 *     compute
 *
 * is represented by:
 *
 *     identifierExpression
 *
 * while:
 *
 *     compute(x)
 *
 * is represented by the expression grammar as an identifier expression
 * followed by a call suffix.
 *
 * The call grammar remains responsible for:
 *
 *     (...)
 *
 * and expression argument lists.
 *
 * ============================================================================
 *
 * LEXICAL KEYWORD BOUNDARY
 * ============================================================================
 *
 * Keyword recognition belongs to the canonical lexer.
 *
 * This grammar does not attempt to determine whether a source spelling is a
 * keyword.
 *
 * Consequently, if:
 *
 *     quantum
 *
 * is lexed as QUANTUM rather than IDENTIFIER, this grammar does not convert it
 * back into an identifier.
 *
 * The same applies to all reserved language keywords.
 *
 * This prevents parser-level name syntax from weakening the language's
 * reserved-word policy.
 *
 * ============================================================================
 *
 * UNICODE BOUNDARY
 * ============================================================================
 *
 * Unicode identifier character classes are owned by:
 *
 *     grammar/lexer/identifiers.g4
 *
 * This file consumes the resulting IDENTIFIER token only.
 *
 * It therefore does not duplicate:
 *
 *     - Unicode categories;
 *     - combining marks;
 *     - connector punctuation;
 *     - Unicode normalization;
 *     - case folding;
 *     - source encoding.
 *
 * The lexer preserves source spelling.
 *
 * Semantic/name-resolution layers may later apply the language's explicit
 * normalization or security policy.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should preserve:
 *
 *     - the original identifier token text;
 *     - qualified segment order;
 *     - source locations;
 *     - token boundaries;
 *     - syntactic distinction between simple and qualified names.
 *
 * The frontend AST may lower these rules into representations such as:
 *
 *     IdentifierExpression
 *     QualifiedNameExpression
 *
 * or an equivalent canonical expression representation.
 *
 * The parser MUST NOT resolve the name.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining what a parsed name means.
 *
 * Possible semantic categories include:
 *
 *     variable
 *     constant
 *     function
 *     type
 *     module
 *     namespace
 *     quantum operation
 *     quantum resource
 *     hardware resource
 *     capability
 *     distributed endpoint
 *     AI model
 *     data object
 *     compiler entity
 *     runtime entity
 *     user-defined entity
 *
 * This grammar deliberately does not enumerate these meanings.
 *
 * ============================================================================
 *
 * IR CONTRACT
 * ============================================================================
 *
 * This file produces parser structure only.
 *
 * It does not construct:
 *
 *     classical IR
 *     quantum::ir
 *     hardware IR
 *     scheduling IR
 *     resource IR
 *     runtime objects
 *
 * Name expressions are lowered only after parsing and semantic resolution.
 *
 * The quantum pipeline remains:
 *
 *     syntax
 *         ->
 *     frontend AST
 *         ->
 *     semantic resolution
 *         ->
 *     quantum::ir
 *
 * ============================================================================
 *
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use the resulting expression nodes for:
 *
 *     - symbol resolution;
 *     - overload resolution;
 *     - type inference;
 *     - capability resolution;
 *     - effect analysis;
 *     - constant evaluation;
 *     - generic resolution;
 *     - resource requirement analysis;
 *     - target-independent lowering.
 *
 * None of those operations are performed here.
 *
 * ============================================================================
 *
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * The runtime MUST NOT depend on this grammar for runtime name resolution.
 *
 * Runtime entities may receive stable identifiers produced by later compiler
 * stages, but runtime resource/device identity must not leak back into this
 * parser grammar.
 *
 * This preserves:
 *
 *     source semantics
 *         !=
 *     current runtime resource identity
 *
 * which is necessary for POCO-REAF.
 *
 * ============================================================================
 *
 * TOOLING CONTRACT
 * ============================================================================
 *
 * IDEs, formatters, language servers, linters, documentation tools, and
 * refactoring tools may use these parser rules to identify expression-level
 * names.
 *
 * Tooling should preserve source spelling unless an explicit refactoring or
 * formatting operation requests a change.
 *
 * No tool should infer hardware, quantum, or runtime meaning solely from these
 * parser rules.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * These rules contain:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment-dependent behavior;
 *     - no random behavior;
 *     - no target-dependent branches.
 *
 * The same token stream therefore receives the same syntactic interpretation.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing expression grammars may already refer to:
 *
 *     identifier
 *     qualifiedName
 *     identifierExpression
 *     qualifiedNameExpression
 *
 * The migration contract is:
 *
 *     IDENT
 *         -> IDENTIFIER
 *
 *     local identifier rule
 *         -> core/names.g4 identifier
 *
 *     local qualified-name implementation
 *         -> core/names.g4 qualifiedName
 *
 * This file provides the expression-specific wrappers without changing the
 * canonical core name contract.
 *
 * ============================================================================
 *
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     lexer/identifiers.g4
 *             |
 *             v
 *        ZamaniLexer
 *             |
 *             v
 *        core/names.g4
 *             |
 *             v
 * expressions/identifiers.g4
 *             |
 *             v
 * expressions/expressions.g4
 *
 * The reverse direction is forbidden.
 *
 * In particular:
 *
 *     names.g4
 *         MUST NOT depend on expressions/identifiers.g4
 *
 * and:
 *
 *     expressions/identifiers.g4
 *         MUST NOT depend on types, quantum, hardware, runtime, IR, or
 *         target-specific grammars.
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar imports the canonical Names parser grammar.
 *
 * Consumers should import this grammar rather than recreating its rules.
 *
 * The canonical token vocabulary remains:
 *
 *     ZamaniLexer
 *
 * ============================================================================
 */

parser grammar ExpressionIdentifiers;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/* ============================================================================
 * IDENTIFIER EXPRESSION
 * ============================================================================
 *
 * A simple identifier used as an expression.
 *
 * Examples:
 *
 *     value
 *     state
 *     q
 *     matrix
 *     tensor
 *     backend
 *     accelerator
 *
 * The actual identifier syntax comes from:
 *
 *     Names.identifier
 *
 * The lexer token comes from:
 *
 *     ZamaniLexer.IDENTIFIER
 *
 * No semantic interpretation is performed here.
 */
identifierExpression
    : identifier
    ;


/* ============================================================================
 * QUALIFIED NAME EXPRESSION
 * ============================================================================
 *
 * A canonical qualified name used as an expression.
 *
 * Examples:
 *
 *     math::pi
 *     math::linear::solve
 *     quantum::operation
 *     hardware::capability
 *     accelerator::kernel
 *
 * The structure is inherited from:
 *
 *     Names.qualifiedName
 *
 * This file does not determine whether the name identifies a module, symbol,
 * function, type, operation, capability, resource, or any other semantic
 * entity.
 */
qualifiedNameExpression
    : qualifiedName
    ;


/* ============================================================================
 * NAME EXPRESSION
 * ============================================================================
 *
 * Shared expression entry point for a name.
 *
 * This rule intentionally preserves the distinction between:
 *
 *     simple identifier
 *
 * and:
 *
 *     qualified name
 *
 * so downstream AST construction can retain source-level structure.
 */
nameExpression
    : identifierExpression
    | qualifiedNameExpression
    ;