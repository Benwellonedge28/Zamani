/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/antlr/Modules.g4
 *
 * Role:
 *     Canonical modular parser component for Zamani.
 *
 * Architecture:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          +--> Modules.g4
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Name / module / semantic resolution
 *          |
 *          v
 *     Type / effect / capability / resource analysis
 *          |
 *          v
 *     Canonical IR
 *
 * Specification:
 *     grammar/spec/syntax.md
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     The Zamani compiler implementation MUST use safe Rust only.
 *     This grammar contains no Rust code and introduces no unsafe behavior.
 *
 * ============================================================================
 *
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This grammar owns ONLY the concrete syntax of:
 *
 *     - module declarations;
 *     - module paths;
 *     - module bodies;
 *     - imports;
 *     - import targets;
 *     - import specifiers;
 *     - import sources;
 *     - exports;
 *     - export targets;
 *     - export specifiers;
 *     - export sources;
 *     - use declarations;
 *     - use trees;
 *     - use paths;
 *     - package declarations;
 *     - package fields.
 *
 * It does NOT own:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - declarations outside the module system;
 *     - semantic resolution;
 *     - filesystem access;
 *     - package downloading;
 *     - dependency resolution;
 *     - registry access;
 *     - capability resolution;
 *     - visibility checking;
 *     - symbol tables;
 *     - AST storage;
 *     - IR lowering;
 *     - target realization.
 *
 * ============================================================================
 *
 * CRITICAL DESIGN RULE
 * ============================================================================
 *
 * The grammar describes SOURCE STRUCTURE.
 *
 * It must never imply that:
 *
 *     import = filesystem read
 *     import = network request
 *     package = downloaded artifact
 *     module = physical file
 *     path = operating-system path
 *     namespace = backend namespace
 *
 * Those are semantic/toolchain concerns.
 *
 * For example:
 *
 *     import foo::bar;
 *
 * is syntax only.
 *
 * Whether `foo::bar` resolves to:
 *
 *     - a source module;
 *     - an already-loaded module;
 *     - a package;
 *     - an embedded module;
 *     - a generated module;
 *     - a workspace module;
 *     - another compilation unit;
 *
 * belongs to the module resolver.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * There are intentionally no grammar-level limits on:
 *
 *     - module depth;
 *     - number of modules;
 *     - number of imports;
 *     - number of exports;
 *     - number of use entries;
 *     - number of package fields;
 *     - identifier length;
 *     - source size.
 *
 * Resource limits, when required operationally, belong to compiler policy.
 *
 * The grammar therefore does not encode artificial constants such as:
 *
 *     MAX_MODULE_DEPTH
 *     MAX_IMPORTS
 *     MAX_EXPORTS
 *     MAX_PATH_SEGMENTS
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * The productions are deliberately structural.
 *
 * They perform no:
 *
 *     - I/O;
 *     - filesystem access;
 *     - networking;
 *     - environment inspection;
 *     - time access;
 *     - randomness;
 *     - backend discovery;
 *     - package resolution.
 *
 * ============================================================================
 *
 * COMPOSITION
 * ============================================================================
 *
 * This grammar is intended to be imported by the canonical Zamani parser:
 *
 *     parser grammar ZamaniParser;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import Modules;
 *
 * The rules below may reference rules supplied by the importing parser,
 * including:
 *
 *     identifier
 *     expression
 *     item
 *     qualifiedName
 *     stringLiteral
 *
 * This allows the module grammar to remain a focused syntactic component
 * without duplicating the core language grammar.
 *
 * ============================================================================
 */

parser grammar Modules;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. MODULE DECLARATIONS
 * ========================================================================== */

/**
 * A module declaration introduces a source-level namespace/container.
 *
 * Supported forms:
 *
 *     module foo;
 *
 *     module foo {
 *         ...
 *     }
 *
 *     module foo::bar;
 *
 *     module foo::bar {
 *         ...
 *     }
 *
 * A module path is a language-level qualified name.
 *
 * It is NOT an operating-system path.
 */
moduleDeclaration
    : visibility? MODULE modulePath moduleBody?
    ;


/**
 * Contents of an inline module.
 *
 * `item` is supplied by the canonical importing parser.
 *
 * A module therefore contains normal Zamani language items rather than a
 * separate sub-language.
 *
 * This is essential for composability:
 *
 *     module foo {
 *         fn ...
 *         struct ...
 *         trait ...
 *         impl ...
 *         module bar { ... }
 *         ...
 *     }
 *
 * The semantic resolver determines which declarations are visible from which
 * namespace.
 */
moduleBody
    : LBRACE item* RBRACE
    ;


/**
 * A module path is a language namespace path.
 *
 * Examples:
 *
 *     core
 *     core::math
 *     org::zamani::quantum
 *
 * The grammar intentionally imposes no depth limit.
 */
modulePath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 2. IMPORTS
 * ========================================================================== */

/**
 * Import declaration.
 *
 * Forms:
 *
 *     import foo;
 *     import foo::bar;
 *     import foo::bar as baz;
 *     import * as foo from "source";
 *     import {foo, bar} from "source";
 *
 * `importSource` is syntactic metadata only.
 *
 * Resolution is NOT performed by the parser.
 */
importDeclaration
    : IMPORT importTarget importSource? SEMI?
    ;


/**
 * Import targets.
 *
 * A target can be:
 *
 *     qualified module/symbol path
 *     wildcard alias
 *     named import list
 */
importTarget
    : qualifiedName
    | STAR AS identifier
    | LBRACE importSpecifierList RBRACE
    ;


/**
 * Named import list.
 *
 * Examples:
 *
 *     import {foo, bar};
 *
 *     import {foo as localFoo, bar};
 *
 * Trailing commas are accepted consistently with other Zamani lists.
 */
importSpecifierList
    : importSpecifier
      (COMMA importSpecifier)*
      COMMA?
    ;


/**
 * One imported symbol and optional local alias.
 *
 * Examples:
 *
 *     foo
 *     foo as bar
 */
importSpecifier
    : identifier
      (AS identifier)?
    ;


/**
 * Optional external/source qualifier.
 *
 * Example:
 *
 *     import {foo} from "library";
 *
 * The string is not interpreted here.
 *
 * In particular, the grammar does NOT decide whether the string identifies:
 *
 *     - a file;
 *     - a package;
 *     - a registry coordinate;
 *     - an embedded resource;
 *     - a workspace;
 *     - a generated module;
 *     - another source provider.
 */
importSource
    : FROM stringLiteral
    ;


/* ============================================================================
 * 3. EXPORTS
 * ========================================================================== */

/**
 * Export declaration.
 *
 * Forms:
 *
 *     export foo;
 *     export foo::bar;
 *     export {foo, bar};
 *     export {foo as publicFoo};
 *     export *;
 *     export * from "source";
 *     export foo from "source";
 *
 * Export semantics belong to module resolution/visibility analysis.
 */
exportDeclaration
    : EXPORT exportTarget exportSource? SEMI?
    ;


/**
 * Export targets.
 *
 * STAR is intentionally syntactic.
 *
 * Whether wildcard export is permitted in a particular semantic context,
 * whether names collide, and how re-exports are resolved belong downstream.
 */
exportTarget
    : STAR
    | qualifiedName
    | LBRACE exportSpecifierList RBRACE
    ;


/**
 * Named export list.
 */
exportSpecifierList
    : exportSpecifier
      (COMMA exportSpecifier)*
      COMMA?
    ;


/**
 * One exported symbol with an optional public alias.
 */
exportSpecifier
    : identifier
      (AS identifier)?
    ;


/**
 * Optional export source.
 *
 * Example:
 *
 *     export * from "library";
 *
 * The parser preserves the source; it does not resolve it.
 */
exportSource
    : FROM stringLiteral
    ;


/* ============================================================================
 * 4. USE DECLARATIONS
 * ========================================================================== */

/**
 * `use` provides namespace/path import syntax.
 *
 * Supported examples:
 *
 *     use foo;
 *     use foo::bar;
 *     use foo::bar as baz;
 *     use foo::*;
 *     use foo::{bar, baz};
 *     use foo::{bar as localBar, baz};
 *
 * The semantic distinction between `use` and `import` is intentionally left
 * to the canonical semantic/module model.
 */
useDeclaration
    : USE useTree (AS identifier)? SEMI?
    ;


/**
 * A use tree represents a root path followed by optional selection.
 *
 * The recursive structure permits arbitrarily nested named selections:
 *
 *     use foo::{bar, baz};
 *     use foo::{bar::{x, y}, baz};
 *
 * This remains bounded only by available compiler resources.
 */
useTree
    : usePath
    | useTreePath
    ;


/**
 * A structured use path.
 *
 * Examples:
 *
 *     foo::bar::*
 *     foo::{bar, baz}
 *     foo::{bar::{x, y}, z}
 *
 * The recursive `useTreeNode` is deliberately used instead of a flat
 * IDENTIFIER-only representation so the AST can preserve source structure.
 */
useTreePath
    : identifier
      (DOUBLE_COLON identifier)*
      DOUBLE_COLON
      (
          STAR
        | LBRACE useTreeList? RBRACE
      )
    ;


/**
 * A use-list contains one or more selected names.
 *
 * Empty braces are accepted syntactically only where the enclosing parser
 * policy permits them; semantic validation should reject meaningless imports.
 *
 * Keeping this syntactically permissive avoids encoding semantic policy in
 * the parser.
 */
useTreeList
    : useTreeNode
      (COMMA useTreeNode)*
      COMMA?
    ;


/**
 * One recursively selectable use-tree node.
 *
 * Examples:
 *
 *     foo
 *     foo as bar
 *     foo::{x, y}
 *     foo::*
 */
useTreeNode
    : identifier
      (AS identifier)?
      (
          DOUBLE_COLON STAR
        | DOUBLE_COLON LBRACE useTreeList? RBRACE
      )?
    ;


/**
 * Plain use path.
 *
 * Example:
 *
 *     use foo::bar;
 */
usePath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 5. PACKAGE DECLARATIONS
 * ========================================================================== */

/**
 * Package declaration.
 *
 * A package is a source-level declaration.
 *
 * It is deliberately NOT a registry manifest.
 *
 * Package discovery, dependency resolution, signatures, hashes, registries,
 * caching, sandboxing and installation belong to the toolchain.
 */
packageDeclaration
    : visibility? PACKAGE identifier packageBody
    ;


/**
 * Package body.
 *
 * Package fields are syntactic key/value declarations.
 *
 * The grammar deliberately does not hard-code a finite package metadata
 * vocabulary. This permits future toolchain metadata without requiring a
 * parser redesign.
 */
packageBody
    : LBRACE packageField* RBRACE
    ;


/**
 * Package field.
 *
 * Examples:
 *
 *     version: "1.0";
 *     repository: "example";
 *     license: "MIT";
 *
 * Values are normal Zamani expressions so the semantic package layer can
 * enforce its own schema.
 */
packageField
    : identifier COLON expression SEMI?
    ;


/* ============================================================================
 * 6. MODULE-RELATED PATHS
 * ========================================================================== */

/**
 * Canonical qualified name.
 *
 * This production intentionally uses the canonical `identifier` rule rather
 * than introducing a second identifier syntax.
 *
 * Examples:
 *
 *     foo
 *     foo::bar
 *     foo::bar::Baz
 */
qualifiedModuleName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/**
 * Import/export path.
 *
 * Kept as an explicit production so future semantic metadata can distinguish
 * source-level path roles without changing lexical identity.
 */
moduleSymbolPath
    : qualifiedModuleName
    ;


/* ============================================================================
 * 7. SOURCE-LEVEL MODULE METADATA
 * ========================================================================== */

/**
 * Optional module-level declaration metadata.
 *
 * This rule is intentionally NOT automatically attached to `moduleDeclaration`
 * yet. It exists as a stable extension boundary for future canonical syntax
 * if the specification adopts explicit module metadata.
 *
 * Keeping the rule isolated avoids silently changing the accepted language.
 */
moduleMetadata
    : LBRACKET moduleMetadataEntry* RBRACKET
    ;


moduleMetadataEntry
    : identifier
      (ASSIGN expression)?
      COMMA?
    ;


/* ============================================================================
 * 8. SEMANTIC BOUNDARY NOTES
 * ============================================================================
 *
 * The following are intentionally NOT grammar productions:
 *
 *     resolveModule()
 *     resolveImport()
 *     resolveExport()
 *     resolveUse()
 *     loadPackage()
 *     fetchPackage()
 *     readFile()
 *     accessRegistry()
 *     checkVisibility()
 *     detectCycle()
 *     canonicalizePath()
 *     verifySignature()
 *
 * They belong to later compiler/toolchain layers.
 *
 * ============================================================================
 *
 * MODULE GRAPH MODEL
 * ============================================================================
 *
 * Source:
 *
 *     module a {
 *         import b::Thing;
 *         use c::{X, Y};
 *         export Thing;
 *     }
 *
 * Parser result:
 *
 *     ModuleSyntax
 *          |
 *          +-- ImportSyntax
 *          +-- UseSyntax
 *          +-- ExportSyntax
 *          +-- declarations
 *
 * Semantic layer:
 *
 *     ModuleResolver
 *          |
 *          +-- canonical module identity
 *          +-- symbol resolution
 *          +-- visibility
 *          +-- dependency graph
 *          +-- cycle diagnostics
 *          +-- capability policy
 *
 * Lowering:
 *
 *     resolved module
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     ZUIR / canonical IR
 *
 * No module grammar rule may bypass these boundaries.
 *
 * ============================================================================
 *
 * RESOURCE SCALABILITY
 * ============================================================================
 *
 * The grammar intentionally uses recursive structures where the language
 * itself is recursive.
 *
 * It does not impose fixed limits.
 *
 * Implementations may still enforce operational limits such as:
 *
 *     parser recursion policy;
 *     token budget;
 *     source-size budget;
 *     compilation budget;
 *     memory budget;
 *
 * Such limits must be configurable compiler policy, not language syntax.
 *
 * A resource-exhaustion diagnostic must be distinct from a syntax error.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical syntax remains supported:
 *
 *     module foo;
 *     module foo { ... }
 *     import foo::bar;
 *     import foo::bar as baz;
 *     import * as foo from "source";
 *     import {foo, bar} from "source";
 *     export foo;
 *     export *;
 *     export * from "source";
 *     use foo::bar;
 *     use foo::*;
 *     use foo::{bar, baz};
 *
 * No hardware, quantum, mathematical or backend-specific syntax is introduced
 * by this file.
 *
 * ============================================================================
 */