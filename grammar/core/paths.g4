/**
 * Zamani Programming Language
 * ===========================
 *
 * File:
 *     grammar/core/paths.g4
 *
 * Purpose:
 *     Defines the canonical syntactic representation of logical paths used
 *     throughout Zamani.
 *
 * Architectural role:
 *     This grammar owns PATH SYNTAX ONLY.
 *
 *     A path is a language-level symbolic location. It may identify things
 *     such as:
 *
 *       - modules
 *       - packages
 *       - namespaces
 *       - declarations
 *       - members
 *       - resources
 *       - capabilities
 *       - logical interfaces
 *       - semantic entities
 *       - imported/exported names
 *       - qualified symbols
 *       - dialect entities
 *
 *     A path MUST NOT imply:
 *
 *       - a filesystem path
 *       - a physical memory address
 *       - a CPU/core identifier
 *       - a GPU identifier
 *       - an FPGA/ASIC identifier
 *       - a quantum-device identifier
 *       - a physical-qubit identifier
 *       - a network address
 *       - a deployment node
 *       - a machine topology
 *       - a fixed resource count
 *       - a hardware-specific location
 *
 *     Such meanings belong to semantic analysis, target descriptions,
 *     resource models, hardware abstraction, deployment, routing, or
 *     runtime systems.
 *
 * Ownership:
 *     - Path syntax
 *     - Path separators
 *     - Relative path structure
 *     - Absolute/logical path structure
 *     - Path segments
 *     - Path roots
 *     - Parent/current logical path components
 *     - Wildcard path syntax, if enabled by the language contract
 *
 * Does NOT own:
 *     - Identifier lexical definitions
 *     - Qualified-name semantic resolution
 *     - Module resolution
 *     - Package resolution
 *     - Filesystem access
 *     - Network access
 *     - Hardware discovery
 *     - Resource discovery
 *     - Device selection
 *     - Quantum topology
 *     - Physical placement
 *     - Scheduling
 *     - Routing
 *     - Runtime lookup
 *     - Security authorization
 *     - Type checking
 *     - IR construction
 *
 * Scalability:
 *     There are no fixed limits on:
 *
 *       - number of path segments
 *       - path depth
 *       - identifier length
 *       - number of paths in a program
 *       - number of namespaces
 *       - number of modules
 *       - number of packages
 *       - number of logical resources
 *
 *     Practical limits are imposed only by the lexer/parser/runtime
 *     implementation and available resources.
 *
 * Portability:
 *     The grammar intentionally contains no target-specific paths,
 *     addresses, device counts, topology assumptions, or filesystem
 *     separators.
 *
 * Rust:
 *     This grammar contains no embedded Rust actions, predicates, unsafe
 *     code, or target-specific generated-code dependencies.
 *
 * Integration boundary:
 *
 *     source-unit.g4
 *          |
 *          v
 *     paths.g4
 *          |
 *          v
 *     qualified-names.g4
 *          |
 *          v
 *     semantic name/module/resource resolution
 *          |
 *          +----> canonical IR
 *          +----> classical IR
 *          +----> quantum::ir
 *          +----> hardware/resource models
 *
 * IMPORTANT:
 *     This file must remain below semantic resolution and above target
 *     interpretation. It describes syntax, not meaning.
 */

parser grammar Paths;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * --------------------------------------------------------------------------
 * Public entry point
 * --------------------------------------------------------------------------
 *
 * Parses one logical path.
 *
 * This rule is intentionally independent from:
 *
 *     - modules
 *     - packages
 *     - hardware
 *     - quantum
 *     - networking
 *     - filesystem layout
 *
 * Those systems interpret a parsed path later.
 */
path
    : absolutePath
    | relativePath
    ;

/*
 * --------------------------------------------------------------------------
 * Absolute logical paths
 * --------------------------------------------------------------------------
 *
 * An absolute path begins at the language-defined logical root.
 *
 * The root has no inherent relationship to:
 *
 *     /
 *     \
 *     C:
 *     filesystem roots
 *     operating-system roots
 *     network roots
 *     hardware roots
 *
 * The actual root semantics are established by semantic analysis.
 *
 * ROOT_PATH is therefore a language token supplied by the canonical lexer.
 */
absolutePath
    : ROOT_PATH pathSegment (PATH_SEPARATOR pathSegment)*
    ;

/*
 * --------------------------------------------------------------------------
 * Relative logical paths
 * --------------------------------------------------------------------------
 *
 * A relative path is resolved against a semantic context.
 *
 * The context might eventually be:
 *
 *     - module
 *     - package
 *     - namespace
 *     - declaration
 *     - resource namespace
 *     - dialect namespace
 *
 * The grammar does not decide which one.
 */
relativePath
    : relativePathPrefix? pathSegment (PATH_SEPARATOR pathSegment)*
    ;

/*
 * --------------------------------------------------------------------------
 * Relative path prefixes
 * --------------------------------------------------------------------------
 *
 * "."  means current logical scope.
 *
 * ".." means parent logical scope.
 *
 * Their exact semantic interpretation is deliberately deferred to
 * name/module resolution.
 */
relativePathPrefix
    : CURRENT_PATH
    | PARENT_PATH
    | CURRENT_PATH PATH_SEPARATOR PARENT_PATH
    ;

/*
 * --------------------------------------------------------------------------
 * Path segments
 * --------------------------------------------------------------------------
 *
 * A path segment is a symbolic component.
 *
 * IDENTIFIER is supplied by the canonical lexer/identifier grammar.
 *
 * This rule deliberately does not permit arbitrary strings as path
 * components. That prevents paths from silently becoming:
 *
 *     filesystem strings
 *     network URLs
 *     device addresses
 *     arbitrary implementation-defined locations
 *
 * Such syntaxes, if required, must be introduced by their owning domain
 * through an explicit semantic/interop construct rather than weakening
 * the universal logical path model.
 */
pathSegment
    : IDENTIFIER
    ;

/*
 * --------------------------------------------------------------------------
 * Path lists
 * --------------------------------------------------------------------------
 *
 * Used by import/export/resource/etc. grammars where several paths may
 * occur in one construct.
 *
 * No cardinality limit is imposed.
 */
pathList
    : path (COMMA path)*
    ;

/*
 * --------------------------------------------------------------------------
 * Optional path
 * --------------------------------------------------------------------------
 *
 * Useful for grammar components that accept an optional destination,
 * namespace, parent, or source path.
 *
 * Semantic validation remains outside this grammar.
 */
optionalPath
    : path?
    ;

/*
 * --------------------------------------------------------------------------
 * Path suffix
 * --------------------------------------------------------------------------
 *
 * A suffix is one or more logical segments appended to an existing path.
 *
 * This rule is useful when another grammar owns the initial path and this
 * grammar owns its continuation.
 */
pathSuffix
    : PATH_SEPARATOR pathSegment (PATH_SEPARATOR pathSegment)*
    ;

/*
 * --------------------------------------------------------------------------
 * Qualified path
 * --------------------------------------------------------------------------
 *
 * A qualified path is syntactically equivalent to a logical path but is
 * given a separate rule so consumers can retain the distinction in their
 * parse tree.
 *
 * Semantic layers determine whether a qualified path identifies:
 *
 *     module::symbol
 *     package::module
 *     namespace::entity
 *     resource::capability
 *     etc.
 *
 * The grammar does not encode those meanings.
 */
qualifiedPath
    : path
    ;

/*
 * --------------------------------------------------------------------------
 * Path pattern
 * --------------------------------------------------------------------------
 *
 * Path patterns are intentionally separated from ordinary paths.
 *
 * A normal path identifies a logical entity.
 * A path pattern describes a set of possible logical entities.
 *
 * This distinction prevents wildcard matching from accidentally becoming
 * part of ordinary name resolution.
 *
 * WILDCARD_PATH is a lexer-level token representing the language-defined
 * wildcard syntax.
 */
pathPattern
    : pathPatternSegment (PATH_SEPARATOR pathPatternSegment)*
    ;

pathPatternSegment
    : IDENTIFIER
    | WILDCARD_PATH
    ;

/*
 * --------------------------------------------------------------------------
 * Path-or-pattern
 * --------------------------------------------------------------------------
 */
pathOrPattern
    : path
    | pathPattern
    ;