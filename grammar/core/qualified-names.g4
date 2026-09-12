/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/qualified-names.g4
 *
 * Purpose:
 *     Canonical integration grammar for qualified source-level references.
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
 *
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file is an INTEGRATION BOUNDARY.
 *
 * It does NOT redefine:
 *
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - path syntax;
 *     - keywords;
 *     - lexical rules;
 *     - Unicode identifier rules;
 *     - filesystem paths;
 *     - hardware locations;
 *     - quantum physical locations.
 *
 * Those responsibilities remain owned by:
 *
 *     lexer
 *         -> lexical tokens
 *
 *     core/names.g4
 *         -> source-level names and qualified names
 *
 *     core/paths.g4
 *         -> logical paths
 *
 * This grammar composes those canonical constructs into stable interfaces
 * consumed by higher-level grammar domains.
 *
 * ============================================================================
 *
 * CORE PRINCIPLE
 * ============================================================================
 *
 * A qualified name is a SOURCE-LEVEL SYMBOLIC REFERENCE.
 *
 * It does not inherently mean:
 *
 *     - filesystem path;
 *     - URL;
 *     - memory address;
 *     - CPU;
 *     - GPU;
 *     - FPGA;
 *     - ASIC;
 *     - quantum processor;
 *     - physical qubit;
 *     - network node;
 *     - deployment target;
 *     - hardware device;
 *     - runtime resource.
 *
 * Interpretation belongs to semantic analysis.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Qualified names must remain independent of target scale.
 *
 * Nothing here imposes limits on:
 *
 *     - number of qualification segments;
 *     - number of namespaces;
 *     - number of modules;
 *     - number of packages;
 *     - number of declarations;
 *     - number of resources;
 *     - number of devices;
 *     - number of qubits;
 *     - number of nodes.
 *
 * Repetition is delegated to the canonical rules in names.g4 and paths.g4.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     - integration-level qualified-name references;
 *     - qualified-name contexts;
 *     - qualified-path references;
 *     - reusable qualified-reference lists;
 *     - explicit distinction between symbolic names and logical paths;
 *     - compatibility wrappers for higher-level grammar consumers.
 *
 * DOES NOT OWN:
 *
 *     - IDENTIFIER;
 *     - keyword definitions;
 *     - lexical normalization;
 *     - simple-name syntax;
 *     - qualifiedName's internal separator syntax;
 *     - filesystem path syntax;
 *     - module resolution;
 *     - package resolution;
 *     - symbol resolution;
 *     - type checking;
 *     - capability checking;
 *     - effect checking;
 *     - quantum IR;
 *     - classical IR;
 *     - hardware discovery;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Names
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *     Paths              higher-level names
 *          |                  |
 *          +--------+---------+
 *                   |
 *                   v
 *          QualifiedNames
 *                   |
 *                   v
 *          domain grammars
 *                   |
 *                   v
 *          semantic analysis
 *                   |
 *          +--------+---------+
 *          |        |         |
 *          v        v         v
 *       Classical Quantum  Hardware
 *          IR        IR       models
 *
 * There must be no reverse dependency from this grammar to:
 *
 *     IR
 *     runtime
 *     hardware
 *     scheduler
 *     router
 *     optimizer
 *     QEC
 *     ZQN
 *
 * ============================================================================
 *
 * IMPORTANT DESIGN DECISION
 * ============================================================================
 *
 * names.g4 already owns:
 *
 *     qualifiedName
 *
 * and paths.g4 already owns:
 *
 *     qualifiedPath
 *
 * Therefore this grammar deliberately DOES NOT redeclare either rule.
 *
 * Doing so would create competing canonical definitions and eventually
 * produce grammar drift.
 *
 * Instead, this grammar imports and composes the canonical rules.
 *
 * ============================================================================
 */

parser grammar QualifiedNames;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Import the canonical syntax owners.
 *
 * Names owns:
 *
 *     identifier
 *     simpleName
 *     qualifiedName
 *     nameList
 *     qualifiedNameList
 *     nameAlias
 *     nameReference
 *     nameReferenceList
 *
 * Paths owns:
 *
 *     path
 *     absolutePath
 *     relativePath
 *     pathSegment
 *     pathList
 *     optionalPath
 *     pathSuffix
 *     qualifiedPath
 *     pathPattern
 *     pathOrPattern
 */
import Names, Paths;


/* ============================================================================
 * CANONICAL QUALIFIED NAME REFERENCE
 * ============================================================================
 *
 * Public integration rule for consumers that need a symbolic qualified name.
 *
 * Examples:
 *
 *     math::linear
 *     math::linear::matrix
 *     quantum::ir
 *     hardware::capability
 *
 * The actual qualification syntax remains owned by Names.
 */
qualifiedNameReference
    : qualifiedName
    ;


/* ============================================================================
 * CANONICAL QUALIFIED PATH REFERENCE
 * ============================================================================
 *
 * Public integration rule for consumers that require a logical path.
 *
 * A logical path is intentionally distinct from a source-level qualified
 * name.
 *
 * Semantic consumers decide what the path refers to.
 */
qualifiedPathReference
    : qualifiedPath
    ;


/* ============================================================================
 * SYMBOLIC REFERENCE
 * ============================================================================
 *
 * A symbolic reference is a source-level name reference.
 *
 * This includes:
 *
 *     simple names;
 *     qualified names;
 *     aliases where permitted by the owning consumer.
 *
 * This rule does not perform lookup.
 */
symbolicReference
    : nameReference
    ;


/* ============================================================================
 * SYMBOLIC REFERENCE LIST
 * ============================================================================
 *
 * Reuses the canonical list owned by Names.
 *
 * No finite cardinality is imposed.
 */
symbolicReferenceList
    : nameReferenceList
    ;


/* ============================================================================
 * QUALIFIED NAME LIST
 * ============================================================================
 *
 * Integration wrapper around Names.qualifiedNameList.
 */
qualifiedReferenceList
    : qualifiedNameList
    ;


/* ============================================================================
 * OPTIONAL QUALIFIED NAME
 * ============================================================================
 *
 * Useful for declarations and constructs whose qualified reference is
 * optional.
 */
optionalQualifiedNameReference
    : qualifiedNameReference?
    ;


/* ============================================================================
 * OPTIONAL QUALIFIED PATH
 * ============================================================================
 */
optionalQualifiedPathReference
    : qualifiedPathReference?
    ;


/* ============================================================================
 * QUALIFIED NAME OR PATH
 * ============================================================================
 *
 * This rule deliberately preserves the distinction in the parse tree:
 *
 *     symbolic qualified name
 *
 * versus:
 *
 *     logical path
 *
 * Semantic analysis determines which interpretation is legal in a given
 * context.
 *
 * The grammar does NOT silently convert one into the other.
 */
qualifiedReference
    : qualifiedNameReference
    | qualifiedPathReference
    ;


/* ============================================================================
 * QUALIFIED REFERENCE LIST
 * ============================================================================
 *
 * A heterogeneous list is intentionally provided only as an integration
 * construct.
 *
 * Domain grammars should prefer the narrower:
 *
 *     qualifiedReferenceList
 *
 * or:
 *
 *     pathList
 *
 * where the language construct has a specific semantic category.
 *
 * This rule is therefore primarily useful to generic infrastructure such as
 * metadata, dialect registration, tooling, and extensibility mechanisms.
 */
qualifiedReferenceList
    : qualifiedReference (COMMA qualifiedReference)*
    ;


/* ============================================================================
 * OPTIONAL QUALIFIED REFERENCE LIST
 * ============================================================================
 */
optionalQualifiedReferenceList
    : qualifiedReferenceList?
    ;


/* ============================================================================
 * QUALIFIED NAME WITH ALIAS
 * ============================================================================
 *
 * Alias syntax is owned by Names.
 *
 * This wrapper gives import/export/module grammar a stable integration rule
 * without duplicating:
 *
 *     qualifiedName AS identifier
 *
 * here.
 */
qualifiedNameWithAlias
    : nameAlias
    ;


/* ============================================================================
 * QUALIFIED NAME OR ALIAS
 * ============================================================================
 *
 * Used where a consumer permits either:
 *
 *     math::linear
 *
 * or:
 *
 *     math::linear as linear
 *
 * Alias legality is still semantic/contextual.
 */
qualifiedNameReferenceOrAlias
    : qualifiedNameReference
    | qualifiedNameWithAlias
    ;


/* ============================================================================
 * QUALIFIED NAME OR ALIAS LIST
 * ============================================================================
 */
qualifiedNameReferenceOrAliasList
    : qualifiedNameReferenceOrAlias
      (COMMA qualifiedNameReferenceOrAlias)*
    ;


/* ============================================================================
 * OPTIONAL QUALIFIED NAME OR ALIAS LIST
 * ============================================================================
 */
optionalQualifiedNameReferenceOrAliasList
    : qualifiedNameReferenceOrAliasList?
    ;


/* ============================================================================
 * PATH REFERENCE LIST
 * ============================================================================
 *
 * Reuses the canonical path list.
 */
pathReferenceList
    : pathList
    ;


/* ============================================================================
 * PATH OR PATTERN REFERENCE
 * ============================================================================
 *
 * Explicitly exposes the distinction between an ordinary logical path and
 * a path pattern.
 *
 * A path pattern does not become an ordinary name merely because it contains
 * identifiers.
 */
pathOrPatternReference
    : pathOrPattern
    ;


/* ============================================================================
 * PATH OR PATTERN REFERENCE LIST
 * ============================================================================
 */
pathOrPatternReferenceList
    : pathOrPatternReference
      (COMMA pathOrPatternReference)*
    ;


/* ============================================================================
 * OPTIONAL PATH OR PATTERN REFERENCE LIST
 * ============================================================================
 */
optionalPathOrPatternReferenceList
    : pathOrPatternReferenceList?
    ;


/* ============================================================================
 * QUALIFIED NAME CONTEXT
 * ============================================================================
 *
 * Generic contextual wrapper.
 *
 * The grammar intentionally does not encode whether the reference denotes:
 *
 *     module
 *     package
 *     namespace
 *     type
 *     function
 *     declaration
 *     resource
 *     capability
 *     dialect
 *     interface
 *     quantum object
 *     hardware object
 *     distributed object
 *
 * The owning semantic subsystem supplies that meaning.
 */
qualifiedNameContext
    : qualifiedNameReference
    ;


/* ============================================================================
 * QUALIFIED PATH CONTEXT
 * ============================================================================
 *
 * Generic logical-path context.
 */
qualifiedPathContext
    : qualifiedPathReference
    ;


/* ============================================================================
 * GENERIC QUALIFIED REFERENCE CONTEXT
 * ============================================================================
 *
 * Used only where a higher-level grammar intentionally accepts either
 * symbolic-name or logical-path syntax.
 */
qualifiedReferenceContext
    : qualifiedReference
    ;