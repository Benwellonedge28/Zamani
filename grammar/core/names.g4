/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/names.g4
 *
 * Role:
 *     Canonical parser-level name syntax for Zamani.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions and requires no `unsafe`.
 *     The Zamani compiler implementation MUST use safe Rust only.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       |  IDENTIFIER
 *       v
 *     parser
 *       |
 *       v
 *     names.g4                         <-- THIS FILE
 *       |
 *       +--> declarations
 *       +--> modules
 *       +--> types
 *       +--> functions
 *       +--> expressions
 *       +--> quantum
 *       +--> hardware
 *       +--> HDL
 *       +--> distributed
 *       +--> AI/data
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     name/module/symbol resolution
 *       |
 *       v
 *     semantic model / canonical IR
 *
 * ============================================================================
 *
 * CORE PRINCIPLE
 * ============================================================================
 *
 * This file defines the STRUCTURAL SYNTAX of names.
 *
 * It answers:
 *
 *     "Does this token sequence form a valid Zamani name?"
 *
 * It does NOT answer:
 *
 *     "What does this name mean?"
 *
 * Meaning belongs to semantic analysis.
 *
 * Therefore this grammar does NOT resolve:
 *
 *     - symbols;
 *     - scopes;
 *     - modules;
 *     - packages;
 *     - types;
 *     - functions;
 *     - variables;
 *     - quantum resources;
 *     - logical qubits;
 *     - physical qubits;
 *     - hardware devices;
 *     - accelerators;
 *     - network endpoints;
 *     - distributed nodes;
 *     - runtime handles;
 *     - resource IDs.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     - source-level identifier references;
 *     - simple names;
 *     - qualified names;
 *     - lists of names;
 *     - lists of qualified names;
 *     - optional aliases attached to name references;
 *     - structural name syntax.
 *
 * DOES NOT OWN:
 *
 *     - lexical identifier characters;
 *     - keyword definitions;
 *     - Unicode normalization;
 *     - scopes;
 *     - symbol tables;
 *     - module resolution;
 *     - package resolution;
 *     - filesystem paths;
 *     - URLs;
 *     - filesystem identifiers;
 *     - hardware addresses;
 *     - physical device identifiers;
 *     - quantum physical-qubit identifiers;
 *     - resource allocation;
 *     - target selection;
 *     - semantic validity;
 *     - type checking;
 *     - capability checking;
 *     - effect checking;
 *     - IR;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * LEXICAL BOUNDARY
 * ============================================================================
 *
 * The canonical lexer owns IDENTIFIER.
 *
 * This grammar MUST NOT recreate:
 *
 *     IDENTIFIER
 *
 * and MUST NOT duplicate:
 *
 *     Unicode identifier rules
 *     ASCII identifier rules
 *     identifier length rules
 *     keyword recognition
 *     Unicode normalization
 *
 * The existing canonical lexer already defines IDENTIFIER, while the
 * dedicated lexer/identifiers.g4 documents the same lexical responsibility.
 *
 * Therefore this grammar consumes:
 *
 *     IDENTIFIER
 *
 * from:
 *
 *     ZamaniLexer
 *
 * ============================================================================
 *
 * RESERVED WORDS
 * ============================================================================
 *
 * Keywords are not identifiers.
 *
 * The lexer decides whether a spelling becomes:
 *
 *     IDENTIFIER
 *
 * or a reserved keyword token.
 *
 * This parser grammar does not duplicate that policy.
 *
 * Consequently:
 *
 *     quantum
 *     module
 *     fn
 *     let
 *     struct
 *
 * are not automatically accepted as ordinary names merely because they are
 * alphabetic strings.
 *
 * If the language later introduces an explicit escaped-name syntax, that
 * mechanism MUST be added at the lexical/core-name boundary rather than
 * silently weakening keyword rules here.
 *
 * ============================================================================
 *
 * QUALIFICATION
 * ============================================================================
 *
 * Zamani currently uses `::` for qualified names in module/type syntax.
 *
 * Examples:
 *
 *     math::linear
 *     math::linear::matrix
 *     quantum::ir
 *     hardware::capability
 *
 * The grammar deliberately does NOT assign meaning to the segments.
 *
 * For example:
 *
 *     quantum::ir
 *
 * is syntactically a qualified name.
 *
 * Whether it refers to:
 *
 *     - a module;
 *     - a namespace;
 *     - a type;
 *     - a declaration;
 *     - an IR component;
 *     - a future domain object
 *
 * is determined later.
 *
 * ============================================================================
 *
 * PATH SEPARATION
 * ============================================================================
 *
 * `names.g4` owns NAME syntax.
 *
 * It does NOT own filesystem/path syntax.
 *
 * Therefore constructs such as:
 *
 *     ./src/foo.zm
 *     ../module.zm
 *     /absolute/path
 *     https://example
 *
 * MUST NOT be represented by this grammar merely because they contain names.
 *
 * Path syntax belongs to:
 *
 *     grammar/core/paths.g4
 *
 * A path may contain names, but a path is not itself a name.
 *
 * ============================================================================
 *
 * MEMBER ACCESS SEPARATION
 * ============================================================================
 *
 * A source expression such as:
 *
 *     object.member
 *
 * contains identifiers but is not itself a qualified name.
 *
 * `.` therefore remains owned by expression/member-access grammar.
 *
 * Similarly:
 *
 *     object.method()
 *
 * is expression syntax, not name syntax.
 *
 * ============================================================================
 *
 * GENERIC ARGUMENT SEPARATION
 * ============================================================================
 *
 * Generic syntax such as:
 *
 *     Vector::of
 *     Matrix::of
 *
 * may use qualified names.
 *
 * However:
 *
 *     Vector<Int>
 *
 * is a type-expression concern.
 *
 * This grammar therefore does not consume `<...>` as part of a name.
 *
 * ============================================================================
 *
 * QUANTUM / HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * There is deliberately no:
 *
 *     QubitName
 *     PhysicalQubitName
 *     LogicalQubitName
 *     GateName
 *     DeviceName
 *     GPUName
 *     FPGAName
 *     CPUName
 *     NodeName
 *     AcceleratorName
 *
 * rule here.
 *
 * All such spellings are ordinary source-level names.
 *
 * Their meaning is assigned by semantic/domain-specific layers.
 *
 * This is essential for POCO-REAF.
 *
 * A program can therefore use:
 *
 *     q
 *     state
 *     backend
 *     accelerator
 *     device
 *     node
 *     matrix
 *
 * without the grammar deciding what physical resource they denote.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * There is NO artificial maximum on:
 *
 *     - identifier length;
 *     - number of qualified segments;
 *     - number of names in a list;
 *     - number of aliases;
 *     - number of declarations;
 *     - number of namespaces;
 *     - number of modules;
 *     - number of resources;
 *     - number of machines;
 *     - number of qubits;
 *     - number of hardware devices.
 *
 * Repetition is therefore represented using:
 *
 *     *
 *
 * rather than finite bounds.
 *
 * Practical limits may exist because of:
 *
 *     - source size;
 *     - memory;
 *     - parser implementation;
 *     - compiler policy;
 *     - operating-system resources;
 *     - deployment resources.
 *
 * Those are implementation/resource constraints and MUST NOT be encoded as
 * language grammar limits.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * These rules contain no:
 *
 *     - semantic predicates;
 *     - embedded actions;
 *     - runtime-dependent decisions;
 *     - target-dependent branches;
 *     - filesystem access;
 *     - network access;
 *     - random behavior.
 *
 * The same token stream therefore produces the same syntactic interpretation.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should preserve:
 *
 *     identifier spelling
 *     qualified segment order
 *     separators
 *     source spans
 *
 * The AST/semantic model may represent:
 *
 *     SimpleName
 *     QualifiedName
 *
 * but must not resolve them during parsing.
 *
 * Resolution belongs to the semantic name-resolution phase.
 *
 * ============================================================================
 *
 * COMPATIBILITY WITH EXISTING REPOSITORY
 * ============================================================================
 *
 * Existing grammar implementations currently contain overlapping concepts:
 *
 *     Core.g4
 *         qualifiedName
 *
 *     Types.g4
 *         typePath
 *
 *     Modules.g4
 *         modulePath
 *
 * This file establishes the canonical shared name syntax.
 *
 * Migration rule:
 *
 *     qualifiedName
 *         -> names.g4
 *
 *     typePath
 *         -> names.g4 + type-specific wrapper
 *
 *     modulePath
 *         -> names.g4 + module-specific wrapper
 *
 * Domain grammars may wrap a canonical name rule when they need a semantic
 * distinction, but they MUST NOT redefine the lexical/name structure.
 *
 * ============================================================================
 *
 * NO CIRCULAR DEPENDENCY
 * ============================================================================
 *
 * names.g4 depends only on:
 *
 *     ZamaniLexer
 *
 * It must never depend on:
 *
 *     Types.g4
 *     Modules.g4
 *     Quantum.g4
 *     Hardware.g4
 *     Runtime
 *     IR
 *
 * Higher-level grammars consume names.g4.
 *
 * ============================================================================
 */

parser grammar Names;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * SIMPLE NAME
 * ============================================================================
 *
 * A simple name is exactly one lexer-approved identifier.
 *
 * Examples:
 *
 *     value
 *     state
 *     matrix
 *     q
 *     algorithm
 *     accelerator
 *
 * The lexical validity of IDENTIFIER is owned by ZamaniLexer.
 */
simpleName
    : IDENTIFIER
    ;


/* ============================================================================
 * IDENTIFIER REFERENCE
 * ============================================================================
 *
 * `identifier` is retained as the canonical parser rule used by existing
 * grammars.
 *
 * This gives existing rules such as:
 *
 *     identifier
 *     qualifiedName
 *
 * a stable migration target without changing their public rule vocabulary.
 */
identifier
    : simpleName
    ;


/* ============================================================================
 * QUALIFIED NAME
 * ============================================================================
 *
 * A qualified name consists of one or more simple names separated by `::`.
 *
 * Examples:
 *
 *     math::linear
 *     math::linear::matrix
 *     quantum::ir
 *     hardware::capability
 *
 * There is intentionally no upper bound on the number of segments.
 */
qualifiedName
    : nameSegment (DOUBLE_COLON nameSegment)*
    ;


/* ============================================================================
 * QUALIFIED NAME SEGMENT
 * ============================================================================
 *
 * A segment is deliberately kept separate from `simpleName`.
 *
 * This provides a stable extension point for future name syntax without
 * changing qualifiedName itself.
 *
 * At present:
 *
 *     nameSegment == simpleName
 *
 * Future language revisions may extend this rule only under an explicit
 * compatibility policy.
 */
nameSegment
    : simpleName
    ;


/* ============================================================================
 * NAME LIST
 * ============================================================================
 *
 * One or more simple names separated by commas.
 *
 * Examples:
 *
 *     a, b, c
 *
 * There is no artificial list-size limit.
 */
nameList
    : identifier (COMMA identifier)*
    ;


/* ============================================================================
 * OPTIONAL NAME LIST
 * ============================================================================
 *
 * A reusable nullable list for grammar components whose surrounding syntax
 * determines whether an empty list is meaningful.
 *
 * This rule is deliberately explicit instead of forcing every consumer to
 * reproduce:
 *
 *     identifier (COMMA identifier)*
 *
 * `nameList` itself remains non-empty.
 */
optionalNameList
    : nameList?
    ;


/* ============================================================================
 * QUALIFIED NAME LIST
 * ============================================================================
 *
 * One or more qualified names separated by commas.
 *
 * Examples:
 *
 *     math::Vector, math::Matrix
 *     cpu::capability, gpu::capability, quantum::capability
 *
 * The grammar imposes no finite number of entries.
 */
qualifiedNameList
    : qualifiedName (COMMA qualifiedName)*
    ;


/* ============================================================================
 * OPTIONAL QUALIFIED NAME LIST
 * ============================================================================
 */
optionalQualifiedNameList
    : qualifiedNameList?
    ;


/* ============================================================================
 * NAME ALIAS
 * ============================================================================
 *
 * Generic source-level alias syntax:
 *
 *     original as alias
 *
 * Examples:
 *
 *     math::linear as linear
 *     accelerator::tensor as tensor
 *
 * This rule owns only the syntax.
 *
 * It does not determine whether an alias is legal in a particular context.
 */
nameAlias
    : qualifiedName AS identifier
    ;


/* ============================================================================
 * NAME OR ALIAS
 * ============================================================================
 *
 * A consumer that accepts either:
 *
 *     name
 *
 * or:
 *
 *     name as alias
 *
 * can use this shared rule.
 */
nameReference
    : qualifiedName
    | nameAlias
    ;


/* ============================================================================
 * NAME REFERENCE LIST
 * ============================================================================
 *
 * Reusable comma-separated list of name references.
 */
nameReferenceList
    : nameReference (COMMA nameReference)*
    ;


/* ============================================================================
 * OPTIONAL NAME REFERENCE LIST
 * ============================================================================
 */
optionalNameReferenceList
    : nameReferenceList?
    ;