/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/paths.g4
 *
 * Purpose:
 *     Canonical parser-level syntax for logical paths.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime callbacks.
 *     - No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       +--> IDENTIFIER
 *       +--> DOT
 *       +--> DOT_DOT
 *       +--> SLASH
 *       +--> STAR
 *       +--> COMMA
 *       |
 *       v
 *     core/names.g4
 *       |
 *       +--> identifier
 *       +--> nameSegment
 *       +--> qualifiedName
 *       |
 *       v
 *     core/paths.g4
 *       |
 *       +--> logical path syntax
 *       +--> absolute path syntax
 *       +--> relative path syntax
 *       +--> path lists
 *       +--> path patterns
 *       |
 *       v
 *     semantic resolution
 *       |
 *       +--> module/package resolution
 *       +--> import/export resolution
 *       +--> resource/capability resolution
 *       +--> deployment interpretation
 *       +--> domain-specific resolution
 *       |
 *       v
 *     canonical semantic model / IR
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * A path is a SOURCE-LEVEL LOGICAL LOCATION.
 *
 * A path MUST NOT inherently mean:
 *
 *     - a host filesystem location;
 *     - an operating-system path;
 *     - a memory address;
 *     - a CPU identifier;
 *     - a GPU identifier;
 *     - an FPGA identifier;
 *     - an ASIC identifier;
 *     - a QPU identifier;
 *     - a physical qubit;
 *     - a network endpoint;
 *     - a deployment node;
 *     - a scheduler placement;
 *     - a hardware topology location.
 *
 * Those meanings belong to semantic analysis, interoperability, deployment,
 * resource management, hardware abstraction, routing, scheduling, or runtime
 * systems.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical path structure;
 *     - path segments;
 *     - absolute logical paths;
 *     - relative logical paths;
 *     - current-scope path components;
 *     - parent-scope path components;
 *     - path separators;
 *     - path lists;
 *     - optional paths;
 *     - path suffixes;
 *     - path patterns;
 *     - logical path/reference composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifier lexical spelling;
 *     - Unicode identifier classification;
 *     - keyword recognition;
 *     - identifier normalization;
 *     - qualified-name syntax;
 *     - filesystem access;
 *     - URL parsing;
 *     - module resolution;
 *     - package resolution;
 *     - symbol resolution;
 *     - type checking;
 *     - resource allocation;
 *     - hardware discovery;
 *     - physical placement;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - optimization;
 *     - runtime lookup;
 *     - authorization.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexical vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * This parser therefore uses:
 *
 *     tokenVocab = ZamaniTokens
 *
 * Identifier syntax is owned by:
 *
 *     grammar/core/names.g4
 *
 * This file MUST NOT redefine IDENTIFIER.
 *
 * Structural punctuation and operators remain lexer-owned.
 *
 * In particular:
 *
 *     DOT
 *     DOT_DOT
 *     SLASH
 *     STAR
 *     COMMA
 *
 * are consumed here but are not redefined here.
 *
 * ============================================================================
 * NAME / PATH SEPARATION
 * ============================================================================
 *
 * A qualified source-level name and a logical path are distinct concepts.
 *
 * Qualified name:
 *
 *     quantum::ir
 *
 * Logical path:
 *
 *     quantum/ir
 *
 * The parser does not decide what either ultimately denotes.
 *
 * This distinction is important because:
 *
 *     core/names.g4
 *
 * owns:
 *
 *     identifier
 *     qualifiedName
 *
 * while this file owns:
 *
 *     path
 *     qualifiedPath
 *
 * Higher-level grammars may explicitly choose one or the other.
 *
 * ============================================================================
 * PATH SEPARATOR
 * ============================================================================
 *
 * Logical paths use:
 *
 *     /
 *
 * represented by the canonical lexer token:
 *
 *     SLASH
 *
 * This is a parser-level use of the existing operator token.
 *
 * The same token may have arithmetic meaning in expression grammars.
 *
 * That is not a lexical conflict because lexical meaning and parser context
 * are separate concerns.
 *
 * ============================================================================
 * ABSOLUTE PATHS
 * ============================================================================
 *
 * An absolute logical path begins with:
 *
 *     /
 *
 * followed by one or more logical segments.
 *
 * Example:
 *
 *     /system/core
 *     /quantum/operations
 *     /hardware/capabilities
 *
 * The leading slash does NOT imply:
 *
 *     Unix filesystem root;
 *     Windows drive root;
 *     operating-system storage;
 *     physical memory;
 *     network root.
 *
 * Semantic analysis determines the meaning of the logical root.
 *
 * ============================================================================
 * RELATIVE PATHS
 * ============================================================================
 *
 * Relative paths are resolved against a semantic context.
 *
 * Examples:
 *
 *     module
 *     module/child
 *     ./module
 *     ../module
 *     module/../other
 *
 * The parser recognizes the structural form only.
 *
 * It does not perform normalization.
 *
 * ============================================================================
 * CURRENT / PARENT COMPONENTS
 * ============================================================================
 *
 * DOT_DOT:
 *
 *     ..
 *
 * represents a parent logical scope component.
 *
 * DOT:
 *
 *     .
 *
 * represents a current logical scope component when used as a complete path
 * component.
 *
 * Their exact meaning is established by semantic resolution.
 *
 * ============================================================================
 * PATH SEGMENTS
 * ============================================================================
 *
 * A normal path segment is a canonical source-level identifier.
 *
 * This means:
 *
 *     identifier
 *
 * is reused from Names rather than recreated here.
 *
 * Special structural segments are:
 *
 *     .
 *     ..
 *
 * They are not identifiers.
 *
 * ============================================================================
 * PATH PATTERNS
 * ============================================================================
 *
 * Path patterns are intentionally distinct from ordinary paths.
 *
 * A wildcard path segment uses the existing STAR token:
 *
 *     *
 *
 * Example:
 *
 *     /quantum/*
 *     /hardware/*/capability
 *
 * The wildcard is syntax only.
 *
 * Pattern expansion, matching, authorization, discovery, and resolution are
 * semantic/tooling responsibilities.
 *
 * A wildcard MUST NOT be interpreted as:
 *
 *     - all physical devices;
 *     - all CPUs;
 *     - all GPUs;
 *     - all QPUs;
 *     - all nodes;
 *     - all qubits;
 *
 * merely because it appears in a domain-specific context.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally imposes no language-level limits on:
 *
 *     - path depth;
 *     - identifier length;
 *     - number of path segments;
 *     - number of path lists;
 *     - number of path patterns;
 *     - number of modules;
 *     - number of namespaces;
 *     - number of packages;
 *     - number of resources;
 *     - number of machines;
 *     - number of nodes;
 *     - number of qubits;
 *     - number of CPUs;
 *     - number of GPUs;
 *     - number of FPGAs;
 *     - number of accelerators.
 *
 * Repetition is expressed using ANTLR repetition operators.
 *
 * Any practical limits are implementation/resource budgets rather than
 * language semantics.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_PATH_DEPTH
 *     MAX_PATH_SEGMENTS
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_PATHS
 *     MAX_MODULES
 *     MAX_NAMESPACES
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *
 * It MUST NOT encode concrete physical locations.
 *
 * Examples that do NOT belong in this grammar:
 *
 *     q[0]
 *     gpu0
 *     cpu0
 *     device0
 *     node0
 *     physical_qubit_17
 *
 * Those are ordinary identifiers or downstream target-specific data when
 * they are legal at all.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve path structure without resolving it.
 *
 * Recommended conceptual model:
 *
 *     Path
 *       kind:
 *           absolute
 *           relative
 *       components[]
 *       source_span
 *
 *     PathComponent
 *       kind:
 *           identifier
 *           current
 *           parent
 *           wildcard
 *       source_span
 *
 * The exact AST type names belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT depend on the AST implementation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis may interpret a Path as:
 *
 *     module path
 *     package path
 *     namespace path
 *     resource path
 *     capability path
 *     dialect path
 *     logical deployment path
 *     logical data path
 *     interoperability path
 *     domain-specific logical location
 *
 * Semantic analysis is responsible for:
 *
 *     - scope resolution;
 *     - path normalization;
 *     - existence checking;
 *     - visibility checking;
 *     - authorization;
 *     - module/package resolution;
 *     - resource interpretation;
 *     - target interpretation.
 *
 * The parser performs none of those operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Paths do not constitute an IR.
 *
 * Depending on semantic context, a path may lower into:
 *
 *     symbol references;
 *     module references;
 *     package references;
 *     resource references;
 *     capability references;
 *     deployment metadata;
 *     interoperability metadata.
 *
 * Quantum paths MUST NOT create a second quantum IR.
 *
 * Quantum semantic constructs ultimately remain subject to:
 *
 *     quantum::ir
 *
 * and its established downstream pipeline.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may resolve logical paths against:
 *
 *     source modules;
 *     package graphs;
 *     semantic namespaces;
 *     capability registries;
 *     resource descriptions;
 *     deployment contexts.
 *
 * They MUST NOT assume that a logical path is a filesystem path unless an
 * explicit interoperability/file-system construct says so.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may resolve path-bearing semantic objects only after
 * compilation has established their meaning.
 *
 * This grammar never performs runtime lookup.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * The same path syntax is reusable by:
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
 *     resources
 *     effects
 *     dialects
 *     interoperability
 *     macros
 *     metaprogramming
 *
 * Domain grammars MUST NOT create independent generic path grammars.
 *
 * If a domain needs a special path meaning, it should wrap:
 *
 *     path
 *
 * or:
 *
 *     pathOrPattern
 *
 * and define the semantics downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no external state;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime callbacks;
 *     - no random behavior.
 *
 * Therefore path parsing depends only on the token stream.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * Every path and path component must remain source-span traceable through the
 * frontend AST.
 *
 * This grammar does not calculate source offsets itself.
 *
 * Source-span construction belongs to the lexer/frontend infrastructure.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing consumers should migrate from ad-hoc path rules to these canonical
 * rules.
 *
 * In particular, consumers MUST NOT define independent versions of:
 *
 *     identifier / path
 *     pathSegment
 *     absolutePath
 *     relativePath
 *
 * where the generic logical path model is intended.
 *
 * Context-specific wrappers are permitted:
 *
 *     modulePath
 *         : path
 *         ;
 *
 *     resourcePath
 *         : path
 *         ;
 *
 *     capabilityPath
 *         : path
 *         ;
 *
 * These wrappers provide semantic context without duplicating syntax.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust.
 *
 * Generated Rust integration MUST:
 *
 *     - compile on Rust 1.97 / 1.97.1;
 *     - use safe Rust;
 *     - require no unsafe;
 *     - preserve source spans;
 *     - remain deterministic;
 *     - avoid machine-specific assumptions.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     foo
 *     foo/bar
 *     foo/bar/baz
 *     /foo
 *     /foo/bar
 *     ./foo
 *     ../foo
 *     foo/./bar
 *     foo/../bar
 *     /quantum/operations
 *     /hardware/capabilities
 *
 * Path lists:
 *
 *     foo, bar
 *     foo/bar, baz/qux
 *
 * Path patterns:
 *
 *     *
 *     foo/*
 *     /foo/*/bar
 *
 * Negative tests MUST include:
 *
 *     /
 *     foo/
 *     /foo/
 *     foo//bar
 *     foo///bar
 *     ./ 
 *     ../
 *     foo/./
 *     foo/../
 *
 * where the language's semantic/path policy requires a non-empty terminal
 * component.
 *
 * Also test malformed separators and invalid lexical tokens.
 *
 * Boundary tests MUST include:
 *
 *     - one-segment paths;
 *     - deeply nested paths;
 *     - very long identifiers;
 *     - large path lists;
 *     - repeated current/parent components;
 *     - large path patterns.
 *
 * Scalability tests MUST demonstrate that no fixed path depth or cardinality
 * exists in the grammar.
 *
 * Compatibility tests MUST verify that:
 *
 *     names.g4
 *     qualified-names.g4
 *     modules/
 *     imports/exports
 *     resources/
 *     hardware/
 *     interoperability/
 *
 * all consume the same canonical path representation.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] canonical ZamaniTokens vocabulary is used;
 *     [x] Names is reused rather than identifier syntax being duplicated;
 *     [x] logical paths are distinct from qualified names;
 *     [x] absolute paths are defined;
 *     [x] relative paths are defined;
 *     [x] current-scope components are defined;
 *     [x] parent-scope components are defined;
 *     [x] path segments are defined;
 *     [x] path lists are defined;
 *     [x] optional paths are defined;
 *     [x] path suffixes are defined;
 *     [x] qualifiedPath compatibility rule is defined;
 *     [x] path patterns are defined;
 *     [x] pathOrPattern is defined;
 *     [x] no invented ROOT_PATH token is required;
 *     [x] no invented PATH_SEPARATOR token is required;
 *     [x] no invented CURRENT_PATH token is required;
 *     [x] no invented PARENT_PATH token is required;
 *     [x] no invented WILDCARD_PATH token is required;
 *     [x] no filesystem semantics are embedded;
 *     [x] no hardware limits are embedded;
 *     [x] no quantum physical limits are embedded;
 *     [x] no runtime behavior is embedded;
 *     [x] semantic resolution remains downstream;
 *     [x] AST integration is predetermined;
 *     [x] IR integration is predetermined;
 *     [x] Rust 1.97/1.97.1 compatibility is specified;
 *     [x] unsafe Rust is not required;
 *     [x] positive tests are specified;
 *     [x] negative tests are specified;
 *     [x] boundary tests are specified;
 *     [x] scalability tests are specified;
 *     [x] compatibility tests are specified.
 *
 * ============================================================================
 */

parser grammar Paths;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * Reuse the canonical source-level name syntax.

 * Names owns:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *
 * Paths owns logical path syntax.
 *
 * This import prevents paths.g4 from becoming a second identifier grammar.
 */
import Names;


/*
 * ============================================================================
 * PATH
 * ============================================================================
 *
 * Canonical public entry point.
 *
 * A path is either:
 *
 *     absolute
 *     relative
 *
 * There is no separate filesystem-path grammar here.
 */
path
    : absolutePath
    | relativePath
    ;


/*
 * ============================================================================
 * ABSOLUTE PATH
 * ============================================================================
 *
 * Examples:
 *
 *     /foo
 *     /foo/bar
 *     /quantum/operations
 *
 * The leading slash is a logical root marker.
 *
 * It has no inherent operating-system meaning.
 */
absolutePath
    : SLASH pathSegment (SLASH pathSegment)*
    ;


/*
 * ============================================================================
 * RELATIVE PATH
 * ============================================================================
 *
 * Examples:
 *
 *     foo
 *     foo/bar
 *     ./foo
 *     ../foo
 *     foo/../bar
 *
 * A relative path is resolved by semantic analysis against an appropriate
 * logical context.
 */
relativePath
    : relativePathPrefix? relativePathComponent
      (SLASH relativePathComponent)*
    ;


/*
 * ============================================================================
 * RELATIVE PATH PREFIX
 * ============================================================================
 *
 * Current scope:
 *
 *     .
 *
 * Parent scope:
 *
 *     ..
 *
 * These are structural path components, not identifiers.
 */
relativePathPrefix
    : DOT
    | DOT_DOT
    ;


/*
 * ============================================================================
 * RELATIVE PATH COMPONENT
 * ============================================================================
 *
 * A component may be:
 *
 *     identifier
 *     .
 *     ..
 *
 * This permits paths such as:
 *
 *     foo/./bar
 *     foo/../bar
 *
 * without performing normalization in the parser.
 */
relativePathComponent
    : pathSegment
    | DOT
    | DOT_DOT
    ;


/*
 * ============================================================================
 * PATH SEGMENT
 * ============================================================================
 *
 * Reuses the canonical identifier rule from Names.
 *
 * This is deliberately NOT:
 *
 *     IDENTIFIER
 *
 * directly, because the canonical parser-level name contract belongs to
 * core/names.g4.
 */
pathSegment
    : identifier
    ;


/*
 * ============================================================================
 * PATH LIST
 * ============================================================================
 *
 * One or more logical paths.
 *
 * No finite cardinality is imposed.
 */
pathList
    : path (COMMA path)*
    ;


/*
 * ============================================================================
 * OPTIONAL PATH
 * ============================================================================
 */
optionalPath
    : path?
    ;


/*
 * ============================================================================
 * PATH SUFFIX
 * ============================================================================
 *
 * A suffix begins with a separator and contains one or more normal segments.
 *
 * Example:
 *
 *     /bar/baz
 *
 * when attached to an existing logical path.
 *
 * This rule deliberately does not accept an empty suffix.
 */
pathSuffix
    : SLASH pathSegment (SLASH pathSegment)*
    ;


/*
 * ============================================================================
 * QUALIFIED PATH
 * ============================================================================
 *
 * Compatibility/integration wrapper.
 *
 * The repository's qualified-names.g4 expects this public rule.
 *
 * It does NOT mean qualifiedName.
 *
 * A qualified path is still a logical path.
 */
qualifiedPath
    : path
    ;


/*
 * ============================================================================
 * PATH PATTERN
 * ============================================================================
 *
 * A path pattern describes a logical set rather than one necessarily unique
 * logical path.
 *
 * Examples:
 *
 *     *
 *     foo/*
 *     /foo/*/bar
 *
 * STAR is reused from the canonical operator vocabulary.
 *
 * The parser does not perform pattern expansion.
 */
pathPattern
    : pathPatternSegment
      (SLASH pathPatternSegment)*
    ;


/*
 * ============================================================================
 * PATH PATTERN SEGMENT
 * ============================================================================
 *
 * Ordinary identifier:
 *
 *     foo
 *
 * Wildcard:
 *
 *     *
 *
 * The wildcard has no inherent domain-specific meaning.
 */
pathPatternSegment
    : pathSegment
    | STAR
    ;


/*
 * ============================================================================
 * PATH OR PATTERN
 * ============================================================================
 *
 * Explicitly preserves the distinction between:
 *
 *     path
 *
 * and:
 *
 *     pathPattern
 *
 * in the parse tree.
 */
pathOrPattern
    : path
    | pathPattern
    ;