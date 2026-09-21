/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/namespaces.g4
 *
 * Grammar:
 *     Namespaces
 *
 * Status:
 *     CANONICAL MODULE NAMESPACE PARSER COMPONENT
 *
 * Purpose:
 *     Defines source-level namespace declarations, namespace paths,
 *     namespace references, and namespace aliases.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no Rust actions, predicates, unsafe code,
 *     filesystem access, network access, hardware discovery, runtime calls,
 *     or target-specific behavior.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     Namespaces
 *        |
 *        v
 *     canonical frontend AST
 *        |
 *        v
 *     namespace / scope / symbol analysis
 *        |
 *        v
 *     semantic model
 *        |
 *        +-------------------+-------------------+-------------------+
 *        |                   |                   |                   |
 *        v                   v                   v                   v
 *     classical          quantum::ir       HDL/hardware       other domains
 *        |                   |                   |                   |
 *        +-------------------+-------------------+-------------------+
 *                            |
 *                            v
 *                    optimization/lowering
 *                            |
 *                    routing/scheduling
 *                            |
 *                     resilience/QEC/ZQN
 *                            |
 *                            v
 *                           HAL
 *                            |
 *                            v
 *                    target realization
 *
 * Namespace syntax is therefore upstream of all target-specific realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - namespace declaration syntax;
 *   - namespace declaration headers;
 *   - namespace paths;
 *   - namespace references;
 *   - namespace aliases;
 *   - namespace alias targets;
 *   - reusable namespace path lists;
 *   - namespace-specific syntactic wrappers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifier lexical syntax;
 *   - keyword definitions;
 *   - punctuation/token definitions;
 *   - generic qualified-name syntax;
 *   - filesystem paths;
 *   - URLs;
 *   - packages;
 *   - dependencies;
 *   - imports;
 *   - exports;
 *   - visibility;
 *   - module resolution;
 *   - symbol resolution;
 *   - scope construction;
 *   - type checking;
 *   - resource allocation;
 *   - capability discovery;
 *   - hardware discovery;
 *   - target selection;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution.
 *
 * ============================================================================
 * CANONICAL LEXICAL AUTHORITY
 * ============================================================================
 *
 * The parser consumes the production lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * ZamaniLexer itself composes the canonical lexical vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Parser grammars MUST NOT invent alternate token names.
 *
 * In particular, this file deliberately uses:
 *
 *     NAMESPACE
 *     AS
 *     ASSIGN
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     COMMA
 *     SEMICOLON
 *
 * and does NOT use historical names such as:
 *
 *     K_NAMESPACE
 *     K_AS
 *
 * `NAMESPACE` must be added to the canonical keyword vocabulary in:
 *
 *     grammar/lexer/keywords.g4
 *
 * This is a lexer integration requirement, not a second lexical authority.
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * The canonical structural name grammar is:
 *
 *     grammar/core/names.g4
 *
 * It owns:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *     nameList
 *     qualifiedNameList
 *
 * This file MUST reuse `qualifiedName`.
 *
 * It MUST NOT redefine:
 *
 *     identifier (DOUBLE_COLON identifier)*
 *
 * or any equivalent qualified-name production.
 *
 * This keeps namespaces, modules, types, declarations, capabilities,
 * resources, quantum operations, hardware entities, and future domains
 * structurally compatible.
 *
 * ============================================================================
 * NAMESPACE SEMANTICS
 * ============================================================================
 *
 * A namespace is a source-level symbolic naming/scope concept.
 *
 * A namespace is NOT inherently:
 *
 *     - a filesystem directory;
 *     - a package;
 *     - a repository;
 *     - a deployment;
 *     - a process;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a physical qubit;
 *     - a memory bank;
 *     - a network node;
 *     - a runtime resource.
 *
 * Semantic analysis determines relationships among namespaces, modules,
 * packages, declarations, scopes, imports, exports, and visibility.
 *
 * ============================================================================
 * NAMESPACE AND MODULE SEPARATION
 * ============================================================================
 *
 * A module is a source-level compilation/organization unit.
 *
 * A namespace is a source-level naming/scope identity.
 *
 * They may be related semantically, but neither is defined as the other.
 *
 * Therefore:
 *
 *     module quantum::algorithms;
 *
 * does not make `quantum` automatically a:
 *
 *     package
 *     filesystem directory
 *     deployment
 *     hardware domain
 *
 * Likewise:
 *
 *     namespace quantum::algorithms;
 *
 * does not allocate or select quantum hardware.
 *
 * ============================================================================
 * NAMESPACE AND PACKAGE SEPARATION
 * ============================================================================
 *
 * A qualified name such as:
 *
 *     organization::library::math
 *
 * is syntactically just a qualified name.
 *
 * Whether the leading segment denotes:
 *
 *     namespace
 *     package
 *     module
 *     organization
 *
 * is determined semantically.
 *
 * This grammar does not guess.
 *
 * ============================================================================
 * NAMESPACE AND FILESYSTEM PATH SEPARATION
 * ============================================================================
 *
 * Namespace qualification uses:
 *
 *     DOUBLE_COLON
 *
 * Example:
 *
 *     quantum::algorithms::search
 *
 * Filesystem paths are owned elsewhere.
 *
 * This grammar does NOT consume or define:
 *
 *     /
 *     \
 *     ./
 *     ../
 *
 * merely because they appear in a source reference.
 *
 * ============================================================================
 * NAMESPACE DECLARATION FORMS
 * ============================================================================
 *
 * A namespace declaration has the canonical form:
 *
 *     namespace foo;
 *
 * or:
 *
 *     namespace foo::bar;
 *
 * A block namespace has the form:
 *
 *     namespace foo {
 *         ...
 *     }
 *
 * This file owns the namespace header and body boundary.
 *
 * The contents of the body belong to the canonical `item` grammar.
 *
 * This is important: namespaces must not create a second declaration language.
 *
 * ============================================================================
 * NAMESPACE ALIAS FORMS
 * ============================================================================
 *
 * A namespace alias has the canonical form:
 *
 *     namespace linear = math::linear;
 *
 *     namespace q = quantum::algorithms;
 *
 *     namespace hw = hardware::accelerators;
 *
 * The alias name is a simple identifier.
 *
 * The alias target is a canonical qualified name.
 *
 * Alias resolution is semantic.
 *
 * ============================================================================
 * QUALIFICATION
 * ============================================================================
 *
 * Examples:
 *
 *     core
 *     core::math
 *     quantum::algorithms
 *     quantum::algorithms::optimization
 *     hardware::accelerators::tensor
 *     distributed::services::transport
 *     ai::models::training
 *
 * There is intentionally no finite qualification depth.
 *
 * ============================================================================
 * ROOT / GLOBAL NAMESPACE
 * ============================================================================
 *
 * This grammar does not invent a magic root identifier.
 *
 * The absence of an explicit namespace qualifier represents the surrounding
 * lexical/module scope.
 *
 * Names such as:
 *
 *     root
 *     global
 *     universe
 *     world
 *
 * are ordinary identifiers unless the language specification assigns them
 * another meaning.
 *
 * ============================================================================
 * OPEN-WORLD DOMAIN MODEL
 * ============================================================================
 *
 * Namespace syntax is intentionally domain-neutral.
 *
 * These are all ordinary qualified names:
 *
 *     classical::math
 *     quantum::algorithms
 *     hybrid::workflow
 *     hdl::pipeline
 *     hardware::accelerator
 *     distributed::service
 *     ai::model
 *     data::tensor
 *     networking::protocol
 *     security::crypto
 *     future::domain
 *
 * Adding another computational domain MUST NOT require changing this grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Namespace syntax is independent of:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     accelerator count
 *     node count
 *     cluster size
 *     network topology
 *     device count
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Namespace syntax describes symbolic program organization.
 *
 * Target realization remains downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits for:
 *
 *     namespace declarations;
 *     namespace path depth;
 *     namespace path segment count;
 *     aliases;
 *     namespace members;
 *     module count;
 *     declaration count;
 *     program size.
 *
 * Do NOT introduce:
 *
 *     MAX_NAMESPACES
 *     MAX_NAMESPACE_DEPTH
 *     MAX_NAMESPACE_SEGMENTS
 *     MAX_NAMESPACE_ALIASES
 *     MAX_NAMESPACE_MEMBERS
 *     MAX_MODULES
 *
 * Any practical limit is an implementation/resource constraint.
 *
 * It must not become a language-level semantic ceiling.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access;
 *     network access;
 *     package lookup;
 *     dependency resolution;
 *     symbol lookup;
 *     environment inspection;
 *     hardware discovery;
 *     resource discovery;
 *     target selection;
 *     randomness;
 *     wall-clock inspection.
 *
 * Identical token streams therefore produce equivalent syntactic structures.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser/syntax errors include:
 *
 *     namespace;
 *     namespace ::foo;
 *     namespace foo::;
 *     namespace foo =;
 *     namespace = foo;
 *     namespace foo {
 *
 * Semantic errors belong downstream:
 *
 *     duplicate namespace;
 *     unresolved alias target;
 *     alias cycle;
 *     invalid namespace/module relationship;
 *     visibility violation;
 *     inaccessible namespace;
 *     conflicting declarations;
 *     invalid package relationship.
 *
 * The parser MUST NOT encode those semantic checks through predicates.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must be able to preserve:
 *
 *     namespace declaration;
 *     namespace path;
 *     individual path segments;
 *     alias name;
 *     alias target;
 *     declaration form;
 *     source spans;
 *     source ordering;
 *     source spelling where required for diagnostics/provenance.
 *
 * Recommended semantic-neutral concepts:
 *
 *     NamespacePath {
 *         segments
 *     }
 *
 *     NamespaceDeclaration {
 *         path,
 *         body,
 *         span
 *     }
 *
 *     NamespaceAlias {
 *         name,
 *         target,
 *         span
 *     }
 *
 * Exact Rust AST types remain owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar does not define Rust types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - namespace identity;
 *     - namespace uniqueness;
 *     - namespace nesting;
 *     - namespace/module relationships;
 *     - namespace/package relationships;
 *     - scope construction;
 *     - alias resolution;
 *     - alias-cycle detection;
 *     - visibility;
 *     - accessibility;
 *     - symbol lookup;
 *     - declaration ownership;
 *     - import/export relationships.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Namespaces may contain or organize:
 *
 *     classical declarations;
 *     quantum declarations;
 *     hybrid declarations;
 *     HDL declarations;
 *     hardware declarations;
 *     distributed declarations;
 *     AI declarations;
 *     data declarations;
 *     networking declarations;
 *     security declarations;
 *     interoperability declarations;
 *     dialect declarations;
 *     future domain declarations.
 *
 * The namespace grammar does not need to know which domain a declaration
 * belongs to.
 *
 * The canonical `item` grammar determines that.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Namespace syntax MUST NOT encode:
 *
 *     physical qubits;
 *     physical QPU IDs;
 *     gate sets;
 *     topology;
 *     routing;
 *     scheduling;
 *     calibration;
 *     QEC;
 *     ZQN.
 *
 * For example:
 *
 *     namespace quantum::algorithms {
 *         ...
 *     }
 *
 * only establishes source-level organization.
 *
 * Quantum declarations inside the namespace eventually follow the established:
 *
 *     source
 *       ->
 *     frontend AST
 *       ->
 *     semantic quantum representation
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     QEC/resilience/ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * pipeline.
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same namespace mechanism applies to:
 *
 *     classical::...
 *     hdl::...
 *     hardware::...
 *
 * There is no need for:
 *
 *     classicalNamespace
 *     quantumNamespace
 *     hdlNamespace
 *     gpuNamespace
 *     qpuNamespace
 *
 * because those would fragment the language.
 *
 * ============================================================================
 * IMPORT / EXPORT INTEGRATION
 * ============================================================================
 *
 * Imports are owned by:
 *
 *     grammar/modules/imports.g4
 *
 * Exports are owned by:
 *
 *     grammar/modules/exports.g4
 *
 * Namespace qualification may occur inside import/export constructs through
 * the canonical `qualifiedName`.
 *
 * This grammar does not redefine imports or exports.
 *
 * ============================================================================
 * MODULE INTEGRATION
 * ============================================================================
 *
 * Module syntax is owned by:
 *
 *     grammar/modules/modules.g4
 *
 * Modules may establish or inhabit namespaces semantically.
 *
 * This file does not assume that every namespace is a module or every module
 * is a namespace.
 *
 * ============================================================================
 * VISIBILITY INTEGRATION
 * ============================================================================
 *
 * Visibility is owned by:
 *
 *     grammar/modules/visibility.g4
 *
 * This grammar therefore does not duplicate:
 *
 *     public
 *     pub
 *     private
 *     protected
 *     internal
 *
 * If namespace declarations are eventually permitted to carry visibility,
 * the aggregate declaration should compose `visibilityModifier?` from the
 * canonical visibility grammar rather than creating another vocabulary.
 *
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Namespace attributes are NOT defined here.
 *
 * Attributes belong to the canonical attribute/module-attribute grammar.
 *
 * This prevents a second attribute language from being created specifically
 * for namespaces.
 *
 * ============================================================================
 * DIALects
 * ============================================================================
 *
 * `grammar/dialects/namespaces.g4` is a separate dialect-specific namespace
 * grammar.
 *
 * It MUST NOT replace or redefine this module namespace grammar.
 *
 * Dialect namespace syntax must eventually map to the same domain-neutral
 * namespace/name semantics.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Namespace declarations do not directly create:
 *
 *     ClassicalIR
 *     quantum::ir
 *     HDLIR
 *     HardwareIR
 *     RuntimeIR
 *
 * Namespace information is frontend/module metadata consumed by semantic
 * analysis and symbol/module resolution.
 *
 * A namespace MUST NOT become a competing intermediate representation.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The parser/AST integration must preserve enough information for:
 *
 *     diagnostics;
 *     formatter;
 *     source maps;
 *     IDE/LSP;
 *     go-to-definition;
 *     rename;
 *     symbol navigation;
 *     documentation generation;
 *     refactoring;
 *     provenance;
 *     compatibility tooling.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Namespace strings are source syntax.
 *
 * They MUST NOT be:
 *
 *     opened as filesystem paths;
 *     fetched as URLs;
 *     interpreted as commands;
 *     resolved against credentials;
 *     used to perform network requests;
 *     used to probe hardware.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust.
 *
 * The consuming frontend must compile under:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and use safe Rust only.
 *
 * No `unsafe` block, raw pointer, FFI requirement, or target-specific Rust
 * extension is introduced by this grammar.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Namespaces;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * Namespace declaration
 * ============================================================================
 *
 * Supports both:
 *
 *     namespace foo;
 *
 * and:
 *
 *     namespace foo {
 *         ...
 *     }
 *
 * The semicolon form is intentionally distinct from the block form.
 *
 * The block body is delegated to the canonical `item` grammar.
 *
 * This prevents namespaces from inventing a second declaration language.
 */
namespaceDeclaration
    : namespaceDeclarationHeader SEMICOLON
    | namespaceDeclarationHeader LBRACE item* RBRACE
    ;


/*
 * ============================================================================
 * Namespace declaration header
 * ============================================================================
 *
 * Header only; no body or terminator.
 *
 * This rule is intentionally reusable by aggregate grammar components.
 */
namespaceDeclarationHeader
    : NAMESPACE namespacePath
    ;


/*
 * ============================================================================
 * Namespace path
 * ============================================================================
 *
 * Canonical namespace qualification is exactly the canonical qualified-name
 * syntax.
 *
 * Examples:
 *
 *     core
 *     core::math
 *     quantum::algorithms
 *     hardware::accelerators::tensor
 */
namespacePath
    : qualifiedName
    ;


/*
 * ============================================================================
 * Namespace reference
 * ============================================================================
 *
 * Named wrapper for consumers that need an explicit namespace-reference node.
 *
 * It remains structurally identical to `qualifiedName`.
 */
namespaceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * Namespace alias
 * ============================================================================
 *
 * Canonical form:
 *
 *     namespace linear = math::linear;
 *
 *     namespace q = quantum::algorithms;
 *
 * The alias name is a single identifier.
 *
 * The target is a canonical namespace path.
 *
 * No alias is resolved during parsing.
 */
namespaceAliasDeclaration
    : NAMESPACE identifier ASSIGN namespacePath SEMICOLON
    ;


/*
 * ============================================================================
 * Namespace path list
 * ============================================================================
 *
 * Reusable non-empty list.
 *
 * There is no finite list-size limit.
 */
namespacePathList
    : namespacePath
      (COMMA namespacePath)*
    ;


/*
 * ============================================================================
 * Optional namespace path list
 * ============================================================================
 *
 * Explicit nullable wrapper for consumers that need an optional list.
 */
optionalNamespacePathList
    : namespacePathList?
    ;


/*
 * ============================================================================
 * Namespace declaration list
 * ============================================================================
 *
 * Useful to grammar/tooling consumers parsing namespace-only fragments.
 *
 * The normal Zamani compilation-unit parser should use the canonical `item`
 * entry point rather than treating this as a second root grammar.
 */
namespaceDeclarationList
    : namespaceDeclaration+
    ;


/*
 * ============================================================================
 * Namespace-only document
 * ============================================================================
 *
 * Tooling/conformance entry point.
 *
 * This is NOT the language's program root.
 */
namespaceDocument
    : namespaceDeclaration*
      EOF
    ;