/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/modules.g4
 *
 * Role:
 *     Canonical parser component for Zamani's module, import, export,
 *     use, and package source syntax.
 *
 * Grammar layer:
 *     Syntax only.
 *
 * Canonical lexical dependency:
 *     grammar/lexer/tokens.g4
 *
 * Parser integration:
 *
 *     ZamaniTokens
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          +--> Modules.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     module/name/visibility/dependency resolution
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware IR
 *          +--> distributed/accelerator IR
 *          |
 *          v
 *     optimization / routing / scheduling / lowering
 *          |
 *          v
 *     runtime / target realization
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no Rust code.
 *     The Zamani compiler/runtime implementation MUST use safe Rust only.
 *     No unsafe Rust is required for this grammar component.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * This file OWNS:
 *
 *   - module declarations;
 *   - module paths;
 *   - inline module bodies;
 *   - imports;
 *   - import paths;
 *   - import aliases;
 *   - import lists;
 *   - import sources;
 *   - exports;
 *   - export paths;
 *   - export aliases;
 *   - export lists;
 *   - export sources;
 *   - use declarations;
 *   - use trees;
 *   - use aliases;
 *   - package declarations;
 *   - package metadata fields;
 *   - source-level module/package attributes when explicitly attached to
 *     these declarations.
 *
 * This file DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - literals;
 *   - general expressions;
 *   - general types;
 *   - functions;
 *   - structs;
 *   - quantum operations;
 *   - HDL operations;
 *   - hardware resources;
 *   - resource discovery;
 *   - filesystem access;
 *   - network access;
 *   - package downloading;
 *   - package installation;
 *   - dependency solving;
 *   - registry access;
 *   - semantic module resolution;
 *   - symbol tables;
 *   - visibility checking;
 *   - cycle detection;
 *   - AST storage;
 *   - canonical IR;
 *   - target selection;
 *   - runtime dispatch.
 *
 * ============================================================================
 *
 * CRITICAL SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar describes source-level module intent.
 *
 * Therefore:
 *
 *     import foo::bar;
 *
 * does NOT mean:
 *
 *     read file foo/bar
 *     contact a registry
 *     access a network
 *     download a package
 *     select a machine
 *     select a backend
 *
 * Likewise:
 *
 *     package foo { ... }
 *
 * does NOT itself mean:
 *
 *     publish package
 *     install package
 *     fetch package
 *     trust package
 *
 * Those operations belong to later compiler/toolchain layers.
 *
 * ============================================================================
 *
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level finite limits on:
 *
 *   - module depth;
 *   - package depth;
 *   - import count;
 *   - export count;
 *   - use-tree depth;
 *   - use-list size;
 *   - package field count;
 *   - identifier count;
 *   - source-module count.
 *
 * It contains no:
 *
 *   MAX_MODULES
 *   MAX_IMPORTS
 *   MAX_EXPORTS
 *   MAX_PATH_DEPTH
 *   MAX_PACKAGE_FIELDS
 *   MAX_NAMESPACES
 *
 * Resource limits are compiler/runtime policy rather than grammar semantics.
 *
 * A sufficiently capable implementation may therefore process arbitrarily
 * large module graphs, subject only to available resources and explicitly
 * configured operational limits.
 *
 * ============================================================================
 *
 * DETERMINISM
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
 * Parsing the same token stream therefore produces the same syntactic
 * structure.
 *
 * ============================================================================
 *
 * DEPENDENCY MODEL
 * ============================================================================
 *
 * Required canonical lexer vocabulary:
 *
 *   K_MODULE
 *   K_IMPORT
 *   K_EXPORT
 *   K_USE
 *   K_FROM
 *   K_AS
 *   K_PACKAGE
 *   K_PUB
 *   K_PUBLIC
 *   K_PRIVATE
 *   K_PROTECTED
 *   K_INTERNAL
 *   IDENTIFIER
 *   STRING_LITERAL
 *   LBRACE
 *   RBRACE
 *   LBRACKET
 *   RBRACKET
 *   LPAREN
 *   RPAREN
 *   COMMA
 *   SEMICOLON
 *   COLON
 *   DOUBLE_COLON
 *   STAR
 *   ASSIGN
 *
 * Delegating parser dependencies:
 *
 *   item
 *   expression
 *
 * These rules are intentionally supplied by the canonical parser rather than
 * duplicated here.
 *
 * ============================================================================
 */

parser grammar Modules;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. MODULE DECLARATIONS
 * ============================================================================
 *
 * Examples:
 *
 *     module math;
 *
 *     module math {
 *         ...
 *     }
 *
 *     module quantum::algorithms;
 *
 *     module quantum::algorithms {
 *         ...
 *     }
 *
 * A module path is a language namespace.
 *
 * It is NOT an operating-system path.
 */

moduleDeclaration
    : moduleVisibility?
      K_MODULE
      modulePath
      (
          SEMICOLON
        | moduleBody
      )
    ;


/**
 * Inline module contents.
 *
 * `item` belongs to the canonical parser.
 *
 * This permits a module to contain any valid Zamani declaration without
 * creating a second module-specific declaration language.
 */
moduleBody
    : LBRACE item* RBRACE
    ;


/**
 * Canonical module path.
 *
 * Examples:
 *
 *     core
 *     core::math
 *     quantum::algorithms::search
 *
 * No fixed depth is encoded.
 */
modulePath
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 2. MODULE VISIBILITY
 * ============================================================================
 *
 * Visibility syntax is owned here only for module-level declarations.
 *
 * The semantic meaning of visibility is resolved later.
 *
 * No filesystem or package-access semantics are implied.
 */

moduleVisibility
    : K_PUB
    | K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;


/* ============================================================================
 * 3. IMPORT DECLARATIONS
 * ============================================================================
 *
 * Supported forms:
 *
 *     import foo;
 *
 *     import foo::bar;
 *
 *     import foo::bar as baz;
 *
 *     import {foo, bar};
 *
 *     import {foo as localFoo, bar};
 *
 *     import * as namespace from "source";
 *
 *     import {foo, bar} from "source";
 *
 *     import foo::bar from "source";
 *
 *     import "source";
 *
 * The source is syntax only.
 */

importDeclaration
    : K_IMPORT
      importClause?
      importSource?
      SEMICOLON?
    ;


/**
 * Import clause.
 *
 * Exactly one of:
 *
 *   - a path;
 *   - a named import list;
 *   - a wildcard namespace import;
 *   - a side-effect-free source-only import handled by the absence of a
 *     clause.
 */
importClause
    : importPath
      importAlias?
    | importSpecifierGroup
    | importWildcard
    ;


/**
 * A qualified imported module/symbol path.
 *
 * Example:
 *
 *     quantum::algorithms::search
 */
importPath
    : modulePath
    ;


/**
 * Local alias for an imported path.
 *
 * Example:
 *
 *     import quantum::algorithms as algorithms;
 */
importAlias
    : K_AS IDENTIFIER
    ;


/**
 * Wildcard namespace import.
 *
 * Example:
 *
 *     import * as quantum;
 *
 * A wildcard is not interpreted as "all physical resources".
 * It is only a source-level namespace selection.
 */
importWildcard
    : STAR
      (K_AS IDENTIFIER)?
    ;


/**
 * Named imports.
 *
 * Examples:
 *
 *     import {foo, bar};
 *
 *     import {foo as localFoo, bar};
 */
importSpecifierGroup
    : LBRACE
      importSpecifierList?
      RBRACE
    ;


/**
 * One named import.
 */
importSpecifier
    : IDENTIFIER
      importAlias?
    ;


/**
 * Arbitrarily large named-import list.
 */
importSpecifierList
    : importSpecifier
      (
          COMMA
          importSpecifier
      )*
      COMMA?
    ;


/**
 * Optional import source.
 *
 * Example:
 *
 *     import {foo} from "library";
 *
 * The string remains uninterpreted syntax.
 */
importSource
    : K_FROM
      STRING_LITERAL
    ;


/* ============================================================================
 * 4. EXPORT DECLARATIONS
 * ============================================================================
 *
 * Supported forms:
 *
 *     export foo;
 *     export foo::bar;
 *     export foo as bar;
 *     export {foo, bar};
 *     export {foo as publicFoo};
 *     export *;
 *     export * from "source";
 *     export {foo, bar} from "source";
 *     export foo::bar from "source";
 *
 * Export resolution and visibility are semantic concerns.
 */

exportDeclaration
    : K_EXPORT
      exportClause
      exportSource?
      SEMICOLON?
    ;


/**
 * Export target.
 */
exportClause
    : STAR
    | exportPath
      exportAlias?
    | exportSpecifierGroup
    ;


/**
 * Exported path.
 */
exportPath
    : modulePath
    ;


/**
 * Alias for an exported path.
 */
exportAlias
    : K_AS IDENTIFIER
    ;


/**
 * Named export group.
 */
exportSpecifierGroup
    : LBRACE
      exportSpecifierList?
      RBRACE
    ;


/**
 * Named export list.
 */
exportSpecifierList
    : exportSpecifier
      (
          COMMA
          exportSpecifier
      )*
      COMMA?
    ;


/**
 * One named export.
 */
exportSpecifier
    : IDENTIFIER
      exportAlias?
    ;


/**
 * Optional re-export source.
 *
 * Example:
 *
 *     export * from "library";
 */
exportSource
    : K_FROM
      STRING_LITERAL
    ;


/* ============================================================================
 * 5. USE DECLARATIONS
 * ============================================================================
 *
 * `use` is represented as a recursive source-level selection tree.
 *
 * Examples:
 *
 *     use foo;
 *
 *     use foo::bar;
 *
 *     use foo::bar as baz;
 *
 *     use foo::*;
 *
 *     use foo::{bar, baz};
 *
 *     use foo::{bar as localBar, baz};
 *
 *     use foo::bar::{x, y};
 *
 *     use foo::{bar::{x, y}, baz};
 *
 * The grammar does not impose a nesting limit.
 */

useDeclaration
    : K_USE
      useTree
      SEMICOLON?
    ;


/**
 * Root use tree.
 */
useTree
    : useTreePath
      useTreeSuffix?
      useTreeAlias?
    ;


/**
 * The path portion of a use tree.
 *
 * The final identifier is retained as a path component; selection is expressed
 * by the optional suffix.
 */
useTreePath
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/**
 * Selection from a use path.
 *
 * Examples:
 *
 *     use foo::*;
 *
 *     use foo::{bar, baz};
 *
 *     use foo::bar::{x, y};
 */
useTreeSuffix
    : DOUBLE_COLON
      (
          STAR
        | LBRACE
          useTreeList?
          RBRACE
      )
    ;


/**
 * Alias for a complete use tree.
 *
 * Example:
 *
 *     use foo::bar as baz;
 */
useTreeAlias
    : K_AS IDENTIFIER
    ;


/**
 * Recursive use selection list.
 */
useTreeList
    : useTreeNode
      (
          COMMA
          useTreeNode
      )*
      COMMA?
    ;


/**
 * One recursive use-tree node.
 *
 * Examples:
 *
 *     foo
 *     foo as bar
 *     foo::bar
 *     foo::{x, y}
 *     foo::*
 */
useTreeNode
    : IDENTIFIER
      useTreeNodeSuffix?
      useTreeAlias?
    ;


/**
 * Recursive suffix of a use-tree node.
 */
useTreeNodeSuffix
    : DOUBLE_COLON
      (
          IDENTIFIER
          (
              DOUBLE_COLON
              IDENTIFIER
          )*
          (
              DOUBLE_COLON
              (
                  STAR
                | LBRACE
                  useTreeList?
                  RBRACE
              )
          )?
        | STAR
        | LBRACE
          useTreeList?
          RBRACE
      )
    ;


/* ============================================================================
 * 6. PACKAGE DECLARATIONS
 * ============================================================================
 *
 * A package declaration describes source-level package metadata.
 *
 * It does NOT perform:
 *
 *     - publication;
 *     - installation;
 *     - dependency resolution;
 *     - registry access;
 *     - network access;
 *     - signature verification.
 *
 * Those belong to the package/toolchain layer.
 *
 * Example:
 *
 *     package my_library {
 *         version: "1.0.0";
 *         license: "MIT";
 *     }
 *
 * Package metadata is deliberately schema-extensible.
 *
 * The grammar does not hard-code a finite list of metadata keys.
 */

packageDeclaration
    : packageVisibility?
      K_PACKAGE
      IDENTIFIER
      packageBody
    ;


/**
 * Package visibility.
 */
packageVisibility
    : K_PUB
    | K_PUBLIC
    | K_PRIVATE
    | K_INTERNAL
    ;


/**
 * Package metadata body.
 */
packageBody
    : LBRACE
      packageField*
      RBRACE
    ;


/**
 * Package field.
 *
 * The key is an identifier and the value is a normal Zamani expression.
 *
 * This keeps package syntax extensible while leaving the package manifest
 * schema to semantic/toolchain validation.
 */
packageField
    : IDENTIFIER
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 7. MODULE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are deliberately kept as a small syntax-level attachment point.
 *
 * The attribute system itself belongs to the canonical attribute grammar.
 *
 * This rule does not interpret attribute names.
 *
 * Example conceptual forms:
 *
 *     #[...]
 *
 * The canonical parser may attach its own attribute rule to module/package
 * declarations. This component therefore does not duplicate that syntax.
 *
 * ============================================================================
 * 8. CANONICAL PATH CONTRACT
 * ============================================================================
 *
 * The module path syntax is intentionally independent of:
 *
 *     filesystem separators;
 *     operating-system paths;
 *     URLs;
 *     package registries;
 *     repository layouts;
 *     physical devices;
 *     deployment topology.
 *
 * Therefore:
 *
 *     foo::bar
 *
 * remains a language namespace path.
 *
 * A resolver may later map it to:
 *
 *     source module
 *     workspace module
 *     generated module
 *     embedded module
 *     package module
 *     remote source
 *     cached module
 *     another compilation unit
 *
 * without changing the grammar.
 *
 * ============================================================================
 * 9. SEMANTIC RESOLUTION BOUNDARY
 * ============================================================================
 *
 * These operations are intentionally absent:
 *
 *     resolve_module()
 *     resolve_import()
 *     resolve_export()
 *     resolve_use()
 *     resolve_package()
 *     load_source()
 *     read_file()
 *     fetch_registry()
 *     download_package()
 *     verify_package()
 *     resolve_dependency()
 *     detect_module_cycle()
 *     check_visibility()
 *
 * The parser produces syntax.
 *
 * The semantic layer is responsible for:
 *
 *     source identity
 *     canonical module identity
 *     dependency graphs
 *     name resolution
 *     visibility
 *     aliases
 *     imports
 *     exports
 *     re-exports
 *     package metadata validation
 *     cycle diagnostics
 *     capability checks
 *     dependency policy
 *
 * ============================================================================
 * 10. CANONICAL AST INTEGRATION
 * ============================================================================
 *
 * The frontend should lower these productions into one canonical module
 * representation rather than creating independent representations for:
 *
 *     import
 *     export
 *     use
 *     package
 *
 * Recommended conceptual nodes:
 *
 *     ModuleDecl
 *     ModulePath
 *     ImportDecl
 *     ImportTarget
 *     ImportSpecifier
 *     ExportDecl
 *     ExportTarget
 *     UseDecl
 *     UseTree
 *     PackageDecl
 *     PackageField
 *
 * These are AST concepts, not grammar-owned data structures.
 *
 * ============================================================================
 * 11. CANONICAL IR INTEGRATION
 * ============================================================================
 *
 * Modules.g4 MUST NOT create or define IR.
 *
 * The pipeline is:
 *
 *     source
 *       |
 *       v
 *     Modules.g4
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic module graph
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       |
 *       v
 *     optimization / routing / scheduling / target lowering
 *
 * In particular, a module declaration must never directly instantiate:
 *
 *     QubitId
 *     PhysicalQubitId
 *     hardware device IDs
 *     schedule IDs
 *     routing structures
 *     QEC structures
 *     ZQN fault objects
 *
 * Those concepts belong to their respective subsystem boundaries.
 *
 * ============================================================================
 * 12. QUANTUM / CLASSICAL / HDL / HARDWARE COMPATIBILITY
 * ============================================================================
 *
 * Module syntax is domain-neutral.
 *
 * Therefore all of the following may be represented as ordinary module
 * namespaces without module-specific machine assumptions:
 *
 *     classical::math
 *     quantum::algorithms
 *     quantum::qec
 *     hdl::rtl
 *     hardware::accelerators
 *     distributed::runtime
 *     ai::models
 *
 * The module grammar does not need separate quantum modules, CPU modules,
 * GPU modules, QPU modules, FPGA modules, or ASIC modules.
 *
 * Domain-specific semantics remain in their domain grammars and semantic
 * layers.
 *
 * ============================================================================
 * 13. RESOURCE / CAPABILITY INDEPENDENCE
 * ============================================================================
 *
 * Module syntax does not encode:
 *
 *     number of CPUs
 *     number of GPUs
 *     number of qubits
 *     memory capacity
 *     topology
 *     network size
 *     device count
 *     register count
 *     machine width
 *     accelerator count
 *
 * A module can therefore remain unchanged while the program is compiled or
 * executed against different available resources.
 *
 * This is required for POCO-REAF:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * ============================================================================
 * 14. COMPATIBILITY
 * ============================================================================
 *
 * Adding a new import/export/package metadata semantic does not require a
 * grammar change unless the source syntax changes.
 *
 * Adding a new backend, accelerator, quantum processor, HDL target, runtime,
 * package registry, deployment environment, or resource type must not require
 * modification of this grammar merely to recognize the existing module syntax.
 *
 * New reserved keywords must be handled by the lexical compatibility policy,
 * not silently introduced here.
 *
 * ============================================================================
 * 15. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no fixed module count;
 *     no fixed path depth;
 *     no fixed import count;
 *     no fixed export count;
 *     no fixed package-field count;
 *     no fixed package size;
 *     no fixed hardware size;
 *     no fixed device count;
 *     no fixed quantum count;
 *     no fixed CPU/GPU/QPU count.
 *
 * Recursive/list productions are bounded only by parser/compiler resources.
 *
 * ============================================================================
 * 16. DETERMINISM / SAFETY
 * ============================================================================
 *
 * No actions, predicates, target-language code, or semantic callbacks are
 * used in this grammar.
 *
 * Consequently this component performs no:
 *
 *     filesystem operation;
 *     network operation;
 *     process execution;
 *     environment mutation;
 *     random operation;
 *     hardware access.
 *
 * Generated Rust integration remains subject to the repository-wide rule:
 *
 *     #![forbid(unsafe_code)]
 *
 * or the equivalent crate-level safety policy.
 *
 * ============================================================================
 */