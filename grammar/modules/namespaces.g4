/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/namespaces.g4
 *
 * Role:
 *     Canonical parser component for namespace syntax.
 *
 * Grammar layer:
 *     Concrete syntax only.
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, predicates, filesystem
 *     access, network access, hardware discovery, runtime calls, or unsafe
 *     code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniTokens
 *       |
 *       v
 *     core/names.g4
 *       |
 *       v
 *     modules/namespaces.g4          <-- THIS FILE
 *       |
 *       +--> modules.g4
 *       +--> declarations
 *       +--> functions
 *       +--> types
 *       +--> quantum
 *       +--> classical
 *       +--> hdl
 *       +--> hardware
 *       +--> distributed
 *       +--> ai/data
 *       +--> future domains
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic namespace/scope resolution
 *       |
 *       v
 *     canonical semantic model / IR
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware IR
 *       +--> distributed / accelerator IR
 *       |
 *       v
 *     optimization / routing / scheduling / lowering
 *       |
 *       v
 *     runtime / target realization
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * A namespace is a SOURCE-LEVEL NAMING/SCOPE CONCEPT.
 *
 * It is not:
 *
 *     - a filesystem directory;
 *     - a package registry;
 *     - a deployment;
 *     - a process;
 *     - a machine;
 *     - a hardware device;
 *     - a CPU;
 *     - a GPU;
 *     * a QPU;
 *     * a physical qubit;
 *     * a network node;
 *     * a runtime resource.
 *
 * Namespace syntax therefore remains completely independent of the size or
 * architecture of the machine on which the program eventually executes.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - namespace declaration headers;
 *     - namespace paths;
 *     - namespace references;
 *     - namespace aliases;
 *     - namespace qualification syntax;
 *     - syntactic namespace scope identity.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical identifier definitions;
 *     - keyword definitions;
 *     - generic identifier syntax;
 *     - filesystem paths;
 *     - URLs;
 *     - package resolution;
 *     - module resolution;
 *     - symbol resolution;
 *     - declaration semantics;
 *     - visibility semantics;
 *     - import semantics;
 *     - export semantics;
 *     - type semantics;
 *     - expression semantics;
 *     - quantum semantics;
 *     - classical semantics;
 *     - HDL semantics;
 *     - hardware semantics;
 *     - resource semantics;
 *     - capability semantics;
 *     - target selection;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime dispatch.
 *
 * ============================================================================
 * CANONICAL NAME OWNERSHIP
 * ============================================================================
 *
 * `grammar/core/names.g4` owns the structural syntax of names.
 *
 * Therefore this file MUST NOT redefine:
 *
 *     IDENTIFIER
 *     simpleName
 *     identifier
 *     qualifiedName
 *     nameSegment
 *
 * Namespace syntax composes the canonical name grammar.
 *
 * This is critical because namespace names, module names, type names,
 * declaration names, quantum names, hardware names, and resource names must
 * not drift into incompatible syntaxes.
 *
 * ============================================================================
 * NAMESPACE VS MODULE
 * ============================================================================
 *
 * A namespace and a module are related but distinct concepts.
 *
 * Namespace:
 *
 *     logical source-level naming/scope identity.
 *
 * Module:
 *
 *     source-level compilation/organization unit.
 *
 * A module MAY establish or inhabit a namespace.
 *
 * A namespace MUST NOT imply:
 *
 *     - a filesystem location;
 *     - a package;
 *     - a deployment;
 *     - a hardware resource;
 *     - a runtime process.
 *
 * Those relationships are established by semantic/toolchain layers.
 *
 * ============================================================================
 * NAMESPACE VS PACKAGE
 * ============================================================================
 *
 * A package is a distribution/dependency concept.
 *
 * A namespace is a naming/scope concept.
 *
 * Therefore:
 *
 *     package::name
 *
 * is syntactically a qualified name unless semantic analysis determines
 * otherwise.
 *
 * The grammar does not assume that the first segment of a qualified name is
 * a package.
 *
 * ============================================================================
 * NAMESPACE VS FILESYSTEM PATH
 * ============================================================================
 *
 * Namespace paths use language qualification:
 *
 *     ::
 *
 * Examples:
 *
 *     std::collections
 *     quantum::algorithms
 *     hardware::interfaces
 *
 * Filesystem paths belong to:
 *
 *     grammar/core/paths.g4
 *
 * Therefore this grammar does not consume:
 *
 *     /
 *     \
 *     ./
 *     ../
 *     absolute filesystem paths
 *
 * merely because they contain identifiers.
 *
 * ============================================================================
 * NAMESPACE DECLARATION MODEL
 * ============================================================================
 *
 * The canonical declaration form is:
 *
 *     namespace foo;
 *
 * or:
 *
 *     namespace foo::bar;
 *
 * A block-bearing namespace is represented by the aggregate parser:
 *
 *     namespace foo {
 *         ...
 *     }
 *
 * This file owns the namespace declaration HEADER.
 *
 * The aggregate parser owns the surrounding declaration body because the body
 * must contain the canonical Zamani `item` grammar rather than a second
 * namespace-specific declaration language.
 *
 * This avoids a circular grammar dependency.
 *
 * ============================================================================
 * NAMESPACE ALIAS MODEL
 * ============================================================================
 *
 * Namespace aliases are explicitly separated from namespace declarations.
 *
 * Example:
 *
 *     namespace linear = math::linear;
 *
 * The alias creates no new physical resource and performs no module loading.
 *
 * Alias validity, visibility, cycles, and resolution belong to semantic
 * analysis.
 *
 * ============================================================================
 * QUALIFICATION
 * ============================================================================
 *
 * Namespace paths use the canonical qualified-name syntax:
 *
 *     qualifiedName
 *
 * Examples:
 *
 *     core
 *     core::math
 *     quantum::algorithms::search
 *     hardware::accelerator::tensor
 *     distributed::cluster::service
 *
 * No maximum number of segments is encoded.
 *
 * ============================================================================
 * GLOBAL / ROOT NAMESPACE
 * ============================================================================
 *
 * This grammar does not invent a magic root identifier.
 *
 * A root/global namespace is represented by the absence of an explicit
 * namespace qualifier and is interpreted by semantic analysis.
 *
 * The grammar therefore does not reserve names such as:
 *
 *     root
 *     global
 *     universe
 *     world
 *     system
 *
 * merely to represent a root namespace.
 *
 * ============================================================================
 * OPEN / NESTED NAMESPACES
 * ============================================================================
 *
 * Nested namespace declarations are naturally represented through qualified
 * paths:
 *
 *     namespace quantum;
 *
 *     namespace quantum::algorithms;
 *
 *     namespace quantum::algorithms::optimization;
 *
 * No finite nesting depth is encoded.
 *
 * The implementation may impose operational recursion/resource limits, but
 * those limits MUST NOT become language semantics.
 *
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 *
 * Visibility is owned by:
 *
 *     grammar/modules/visibility.g4
 *
 * This file MUST NOT duplicate:
 *
 *     K_PUB
 *     K_PUBLIC
 *     K_PRIVATE
 *     K_PROTECTED
 *     K_INTERNAL
 *
 * If namespace declarations are allowed to carry visibility, the aggregate
 * namespace declaration MUST compose:
 *
 *     visibilityModifier?
 *
 * from visibility.g4.
 *
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attributes are owned by the canonical core attribute grammar.
 *
 * Namespace syntax does not define a second attribute system.
 *
 * ============================================================================
 * IMPORT / EXPORT
 * ============================================================================
 *
 * Import and export syntax remain owned by:
 *
 *     modules/imports.g4
 *     modules/exports.g4
 *
 * Namespace qualification may appear inside import/export paths, but this
 * grammar does not redefine import/export declarations.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser MUST preserve enough information for the frontend AST to retain:
 *
 *     - namespace declaration;
 *     - namespace path;
 *     - individual path segments;
 *     - source spans;
 *     - namespace alias target;
 *     - alias name;
 *     - source spelling.
 *
 * Recommended semantic-neutral representation:
 *
 *     NamespacePath {
 *         segments
 *     }
 *
 *     NamespaceDeclaration {
 *         path,
 *         span
 *     }
 *
 *     NamespaceAlias {
 *         name,
 *         target,
 *         span
 *     }
 *
 * Exact Rust AST types remain owned by the frontend AST.
 *
 * This grammar MUST NOT define Rust structs/enums.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether a namespace exists;
 *     - whether a namespace is declared more than once;
 *     - whether declarations may inhabit it;
 *     - whether a namespace is nested;
 *     - whether aliases are valid;
 *     - whether aliases form cycles;
 *     - whether a namespace is visible;
 *     - whether a referenced namespace is accessible;
 *     - whether a namespace conflicts with a module;
 *     - whether a namespace conflicts with another declaration;
 *     - whether a namespace is imported/exported;
 *     - whether a namespace is package-qualified;
 *     - whether a namespace participates in ABI/linkage.
 *
 * None of those decisions belong in this parser.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Namespace syntax MUST remain identical regardless of:
 *
 *     - CPU count;
 *     - core count;
 *     - thread count;
 *     - GPU count;
 *     - FPGA count;
 *     - ASIC count;
 *     - QPU count;
 *     - qubit count;
 *     - memory capacity;
 *     - accelerator count;
 *     - cluster size;
 *     - network topology;
 *     - deployment topology.
 *
 * Namespace syntax therefore contributes directly to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     - namespace count;
 *     - namespace path depth;
 *     - namespace segment count;
 *     - alias count;
 *     - source-module count;
 *     - declaration count;
 *     - namespace graph size.
 *
 * Do NOT introduce:
 *
 *     MAX_NAMESPACES
 *     MAX_NAMESPACE_DEPTH
 *     MAX_NAMESPACE_SEGMENTS
 *     MAX_NAMESPACE_ALIASES
 *
 * Any operational limits belong to compiler/resource policy.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - filesystem access;
 *     - network access;
 *     - environment inspection;
 *     - clock access;
 *     - randomness;
 *     - package lookup;
 *     - module lookup;
 *     - symbol lookup;
 *     - hardware discovery;
 *     - resource discovery.
 *
 * Identical token streams therefore have deterministic syntactic results.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Semantic errors belong to later validation.
 *
 * Examples of syntax errors:
 *
 *     namespace ;
 *     namespace ::foo;
 *     namespace foo::;
 *     namespace foo = ;
 *
 * Examples of semantic errors:
 *
 *     duplicate namespace declaration
 *     namespace alias cycle
 *     inaccessible namespace
 *     unresolved namespace
 *     conflicting namespace/module ownership
 *
 * The grammar MUST NOT encode semantic predicates for those cases.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar delegate.
 *
 * Canonical lexical vocabulary:
 *
 *     ZamaniTokens
 *
 * Canonical name grammar:
 *
 *     Names
 *
 * The aggregate parser should import this grammar and consume:
 *
 *     namespaceDeclarationHeader
 *     namespaceAliasDeclaration
 *     namespacePath
 *     namespaceReference
 *
 * ============================================================================
 */

parser grammar Namespaces;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. NAMESPACE DECLARATION HEADER
 * ============================================================================
 *
 * Examples:
 *
 *     namespace core;
 *
 *     namespace core::math;
 *
 *     namespace quantum::algorithms;
 *
 *     namespace hardware::accelerators::tensor;
 *
 * The terminating semicolon or body belongs to the aggregate declaration
 * grammar.
 */
namespaceDeclarationHeader
    : K_NAMESPACE
      namespacePath
    ;


/* ============================================================================
 * 2. NAMESPACE PATH
 * ============================================================================
 *
 * Namespace paths intentionally reuse the canonical qualified-name structure.
 *
 * No duplicate `identifier (DOUBLE_COLON identifier)*` production is created
 * here.
 *
 * The canonical name grammar owns the structure.
 */
namespacePath
    : qualifiedName
    ;


/* ============================================================================
 * 3. NAMESPACE REFERENCE
 * ============================================================================
 *
 * A namespace reference is syntactically a qualified name.
 *
 * It is kept as a named wrapper so semantic/domain-specific consumers can
 * distinguish a namespace reference from an arbitrary qualified name without
 * duplicating the underlying syntax.
 */
namespaceReference
    : qualifiedName
    ;


/* ============================================================================
 * 4. NAMESPACE ALIAS
 * ============================================================================
 *
 * Canonical form:
 *
 *     namespace Alias = target::namespace;
 *
 * Examples:
 *
 *     namespace linear = math::linear;
 *
 *     namespace q = quantum::algorithms;
 *
 *     namespace hw = hardware::accelerators;
 *
 * The alias name is a simple identifier.
 *
 * The target is a canonical namespace path.
 *
 * The grammar does not resolve the target.
 */
namespaceAliasDeclaration
    : K_NAMESPACE
      identifier
      ASSIGN
      namespacePath
      SEMICOLON
    ;


/* ============================================================================
 * 5. NAMESPACE PATH LIST
 * ============================================================================
 *
 * Reusable list for grammar components that operate on multiple namespaces.
 *
 * No finite number of namespaces is encoded.
 */
namespacePathList
    : namespacePath
      (
          COMMA
          namespacePath
      )*
    ;


/* ============================================================================
 * 6. OPTIONAL NAMESPACE PATH LIST
 * ============================================================================
 */
optionalNamespacePathList
    : namespacePathList?
    ;