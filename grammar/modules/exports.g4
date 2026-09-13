/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/exports.g4
 *
 * Role:
 *     Focused parser component for source-level module exports and re-exports.
 *
 * Grammar layer:
 *     Concrete syntax only.
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     This file contains ANTLR grammar only and no Rust code.
 *     Generated/compiler/runtime code for Zamani MUST remain safe Rust.
 *     No `unsafe` Rust is required or implied by this grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - the `export` declaration;
 *   - exported symbol paths;
 *   - named export lists;
 *   - export aliases;
 *   - wildcard exports;
 *   - source-qualified exports/re-exports;
 *   - syntax needed to preserve the source structure of those constructs.
 *
 * THIS FILE DOES NOT OWN
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - general expressions or types;
 *   - declarations being exported;
 *   - module resolution;
 *   - symbol-table construction;
 *   - visibility checking;
 *   - duplicate-name detection;
 *   - dependency/cycle analysis;
 *   - package resolution;
 *   - filesystem or network access;
 *   - package installation or publication;
 *   - capability/resource resolution;
 *   - target selection;
 *   - classical IR;
 *   - quantum::ir;
 *   - HDL/hardware IR;
 *   - optimization, routing, scheduling, or runtime dispatch.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * `export` describes source-level API/module intent.
 *
 * It MUST NOT mean any of the following by itself:
 *
 *   export = write a file
 *   export = publish a package
 *   export = contact a registry
 *   export = load another module
 *   export = select a hardware backend
 *   export = select a quantum device
 *   export = select a CPU/GPU/FPGA/ASIC
 *   export = allocate a resource
 *
 * Resolution and realization occur later:
 *
 *   source
 *      |
 *      v
 *   lexer
 *      |
 *      v
 *   parser / module grammar
 *      |
 *      v
 *   frontend AST
 *      |
 *      v
 *   module + symbol resolution
 *      |
 *      v
 *   semantic model
 *      |
 *      v
 *   canonical IR
 *      |
 *      +--> classical IR
 *      +--> quantum::ir
 *      +--> HDL / hardware IR
 *      +--> distributed / accelerator IR
 *      |
 *      v
 *   optimization / routing / scheduling / lowering
 *      |
 *      v
 *   target / runtime realization
 *
 * This keeps grammar syntax independent of machine size and topology and is
 * required for POCO-REAF.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar deliberately contains no finite language-level limit for:
 *
 *   - number of exported names;
 *   - number of export declarations;
 *   - number of path segments;
 *   - number of named re-export entries;
 *   - source-module graph size;
 *   - package graph size;
 *   - target count;
 *   - device count;
 *   - qubit count;
 *   - CPU/core/thread count;
 *   - memory size;
 *   - accelerator count;
 *   - network size.
 *
 * No constants such as MAX_EXPORTS or MAX_PATH_DEPTH belong here.
 *
 * An implementation MAY impose configurable operational budgets for parsing,
 * memory, diagnostics, compilation time, or recursion. Such budgets are
 * implementation/resource policy, not grammar semantics, and MUST NOT alter
 * the language meaning of a valid source program merely because a target has
 * fewer physical resources.
 *
 * ============================================================================
 * DETERMINISM / PURITY
 * ============================================================================
 *
 * These productions perform no:
 *
 *   - filesystem I/O;
 *   - network I/O;
 *   - environment inspection;
 *   - clock access;
 *   - randomness;
 *   - package lookup;
 *   - hardware discovery;
 *   - backend discovery.
 *
 * The same token stream therefore has the same syntactic interpretation.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar is a parser grammar and consumes the canonical token vocabulary
 * from grammar/lexer/tokens.g4.
 *
 * Required tokens:
 *
 *   K_EXPORT
 *   K_FROM
 *   K_AS
 *   IDENTIFIER
 *   STRING_LITERAL
 *   STAR
 *   LBRACE
 *   RBRACE
 *   COMMA
 *   DOUBLE_COLON
 *   SEMICOLON
 *
 * This file MUST NOT redeclare lexer rules.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical parser is expected to import this grammar component:
 *
 *     parser grammar ZamaniParser;
 *
 *     options {
 *         tokenVocab = ZamaniTokens;
 *     }
 *
 *     import Modules, Exports;
 *
 * `Exports.g4` is intentionally narrow. It references only canonical lexical
 * tokens and its own export productions.
 *
 * IMPORTANT MIGRATION RULE
 *
 * Export productions currently present in other module grammar files must not
 * remain as duplicate definitions once this component becomes canonical.
 *
 * In particular, the following ownership must move here:
 *
 *     exportDeclaration
 *     exportTarget
 *     exportPath
 *     exportAlias
 *     exportSpecifierList
 *     exportSpecifier
 *     exportSource
 *
 * There must be exactly one canonical definition of each rule.
 *
 * ============================================================================
 * EXPORT SEMANTICS
 * ============================================================================
 *
 * Supported source forms include:
 *
 *     export foo;
 *     export foo::bar;
 *     export foo as bar;
 *     export foo::bar as bar;
 *
 *     export {foo, bar};
 *     export {foo as publicFoo, bar};
 *
 *     export *;
 *     export * from "source";
 *
 *     export {foo, bar} from "source";
 *     export foo::bar from "source";
 *
 * The source string is opaque syntax here.
 *
 * ============================================================================
 * SEMANTIC RULES LEFT DOWNSTREAM
 * ============================================================================
 *
 * The parser MUST NOT decide:
 *
 *   - whether an exported name exists;
 *   - whether it is visible from the current module;
 *   - whether an alias collides with another exported name;
 *   - whether wildcard exports create conflicts;
 *   - whether a re-export source exists;
 *   - whether a source is a file, package, registry coordinate, generated
 *     module, workspace module, or another source provider;
 *   - whether a package is trusted;
 *   - whether an export is ABI-compatible;
 *   - whether an exported declaration is usable on a particular target.
 *
 * These checks belong to module/name/visibility/semantic analysis.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve at least:
 *
 *   ExportSyntax {
 *       target,
 *       source,
 *       span
 *   }
 *
 * where target preserves one of:
 *
 *   Wildcard
 *
 *   Path {
 *       path,
 *       alias
 *   }
 *
 *   List {
 *       specifiers
 *   }
 *
 * and each specifier preserves:
 *
 *   path
 *   alias
 *   source span
 *
 * Exact Rust type names belong to the repository AST implementation and are
 * deliberately not encoded in this .g4 file.
 *
 * No grammar rule may construct or reference quantum::ir directly.
 *
 * ============================================================================
 * CANONICAL PATH MODEL
 * ============================================================================
 *
 * Export paths use repeated `DOUBLE_COLON` segments.
 *
 * Therefore:
 *
 *     export quantum::algorithms::search;
 *
 * is a language namespace path.
 *
 * It does NOT imply:
 *
 *     quantum/algorithms/search
 *
 * on any host filesystem.
 *
 * No fixed path depth is encoded.
 *
 * ============================================================================
 * WILDCARD MODEL
 * ============================================================================
 *
 * `export *` and `export * from "source"` preserve wildcard intent.
 *
 * Whether wildcard export is legal, how conflicts are resolved, and which
 * names are visible are semantic-policy questions.
 *
 * The parser must not silently filter or expand wildcard exports.
 *
 * ============================================================================
 * RE-EXPORT MODEL
 * ============================================================================
 *
 * Source-qualified exports are re-export syntax:
 *
 *     export foo from "source";
 *     export {foo, bar} from "source";
 *     export * from "source";
 *
 * The parser stores the source literally.
 *
 * It performs no:
 *
 *     filesystem lookup
 *     package lookup
 *     registry access
 *     network access
 *     signature verification
 *     dependency resolution
 *
 * ============================================================================
 * VERSION / COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding export syntax is a language-versioning change.
 *
 * Existing valid forms must remain valid unless an explicit compatibility
 * policy says otherwise.
 *
 * Version gating belongs to language-version and semantic validation layers,
 * not to external-state-dependent parser actions.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Export source strings are untrusted source text.
 *
 * This grammar performs no interpretation, decoding into filesystem paths,
 * network access, command execution, environment access, or package fetching.
 *
 * Downstream resolver/toolchain code owns:
 *
 *   - canonicalization;
 *   - sandboxing;
 *   - trust policy;
 *   - signatures;
 *   - package integrity;
 *   - registry policy;
 *   - dependency policy.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests:
 *
 *     export foo;
 *     export foo::bar;
 *     export foo as bar;
 *     export foo::bar as bar;
 *     export {foo, bar};
 *     export {foo as publicFoo, bar};
 *     export *;
 *     export * from "source";
 *     export {foo, bar} from "source";
 *     export foo::bar from "source";
 *     export {foo::bar as publicBar};
 *
 * Negative tests:
 *
 *     export;
 *     export as foo;
 *     export foo as;
 *     export { as foo };
 *     export {foo,,bar};
 *     export foo from;
 *
 * `export * as alias;` is intentionally not part of this export grammar.
 * Wildcard exports preserve wildcard semantics rather than inventing an
 * additional namespace-alias interpretation.
 *
 * Boundary/scalability tests:
 *
 *     - one export;
 *     - many exports;
 *     - deeply qualified names;
 *     - large named-export lists;
 *     - large module graphs;
 *     - no artificial MAX_EXPORTS behavior;
 *     - no artificial MAX_PATH_DEPTH behavior.
 *
 * Cross-domain tests:
 *
 *     quantum::...
 *     classical::...
 *     hdl::...
 *     hardware::...
 *     distributed::...
 *     ai::...
 *     networking::...
 *
 * must all be ordinary qualified names at this grammar layer.
 *
 * Determinism tests must parse identical token streams identically.
 *
 * Round-trip tests must preserve path, alias, wildcard, source, and source-span
 * intent through the parser/AST/printer pipeline where supported.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *   MAX_EXPORTS
 *   MAX_PATH_DEPTH
 *   MAX_MODULES
 *   fixed device IDs
 *   fixed CPU/GPU/FPGA/ASIC counts
 *   fixed qubit counts
 *   fixed topology names
 *   hardware addresses
 *   provider-specific identifiers
 *   filesystem assumptions
 *   network assumptions
 *
 * Domain names are intentionally absent from the syntax.
 *
 * `quantum::foo` and `classical::foo` are both qualified language names.
 * Their semantic interpretation belongs downstream.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   1. It compiles as an ANTLR parser grammar using ZamaniTokens.
 *   2. It has one canonical owner for every export rule.
 *   3. The canonical parser imports it without duplicate rule definitions.
 *   4. Export paths and aliases are preserved for the frontend AST.
 *   5. Wildcard intent is preserved.
 *   6. Re-export source information is preserved.
 *   7. No semantic resolution occurs during parsing.
 *   8. No filesystem/network/hardware access occurs during parsing.
 *   9. No finite machine/resource limitation is encoded.
 *  10. Quantum/classical/HDL/hardware/future domains require no grammar
 *      changes merely because new module namespaces are introduced.
 *  11. Rust integration remains compatible with Rust 1.97/1.97.1.
 *  12. The safe-Rust-only repository policy remains intact.
 *  13. Duplicate export rules are removed from the previous owning grammar
 *      component as part of the integration migration.
 *
 * ============================================================================
 */

parser grammar Exports;

options {
    tokenVocab = ZamaniTokens;
}


// ============================================================================
// 1. EXPORT DECLARATION
// ============================================================================

/**
 * Source-level export declaration.
 *
 * A trailing semicolon is accepted for compatibility with Zamani declaration
 * syntax.
 */
exportDeclaration
    : K_EXPORT
      exportTarget
      exportSource?
      SEMICOLON?
    ;


// ============================================================================
// 2. EXPORT TARGET
// ============================================================================

/**
 * One export target.
 *
 * Wildcard:
 *
 *     export *;
 *
 * Qualified/path export:
 *
 *     export foo::bar;
 *
 * Qualified/path export with alias:
 *
 *     export foo::bar as publicBar;
 *
 * Named export group:
 *
 *     export {foo, bar};
 */
exportTarget
    : STAR
    | exportPath exportAlias?
    | LBRACE exportSpecifierList? RBRACE
    ;


// ============================================================================
// 3. QUALIFIED EXPORT PATH
// ============================================================================

/**
 * Source-level namespace/symbol path.
 *
 * No fixed depth is encoded.
 */
exportPath
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


// ============================================================================
// 4. EXPORT ALIAS
// ============================================================================

/**
 * Renames an exported path in the current module's public namespace.
 */
exportAlias
    : K_AS IDENTIFIER
    ;


// ============================================================================
// 5. NAMED EXPORT LIST
// ============================================================================

/**
 * Arbitrarily large named-export list, subject only to implementation
 * resources.
 *
 * Trailing commas are accepted.
 */
exportSpecifierList
    : exportSpecifier
      (COMMA exportSpecifier)*
      COMMA?
    ;


/**
 * One named export.
 *
 * Qualified names are intentionally allowed:
 *
 *     export {
 *         quantum::algorithms::search as search
 *     };
 *
 * No domain-specific productions are necessary.
 */
exportSpecifier
    : exportPath exportAlias?
    ;


// ============================================================================
// 6. SOURCE-QUALIFIED EXPORT / RE-EXPORT
// ============================================================================

/**
 * Optional opaque source qualifier.
 *
 * Examples:
 *
 *     export * from "library";
 *     export {foo, bar} from "library";
 *     export foo::bar from "library";
 *
 * The string is preserved by parsing and interpreted only by later module
 * resolution/toolchain layers.
 */
exportSource
    : K_FROM STRING_LITERAL
    ;