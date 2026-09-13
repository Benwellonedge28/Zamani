/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/imports.g4
 *
 * Grammar:
 *     Imports
 *
 * Purpose:
 *     Canonical source-level grammar for Zamani import declarations.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime callbacks, or unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Imports
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Module / name resolution
 *          |
 *          v
 *     Semantic analysis
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     Classical IR          quantum::ir
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *              optimization
 *                    |
 *                    v
 *            routing / scheduling
 *                    |
 *                    v
 *        hardware / runtime / deployment
 *
 * This grammar owns only the concrete syntax of imports.
 *
 * It does NOT resolve imports.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - import declarations;
 *   - import clauses;
 *   - imported qualified names;
 *   - imported symbol lists;
 *   - import aliases;
 *   - wildcard imports;
 *   - source-qualified imports;
 *   - import source literals;
 *   - import-list separators;
 *   - import-specific syntactic structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical keywords;
 *   - identifiers;
 *   - qualified-name syntax;
 *   - filesystem paths;
 *   - URLs;
 *   - package resolution;
 *   - dependency resolution;
 *   - module discovery;
 *   - registry access;
 *   - downloading;
 *   - caching;
 *   - signature verification;
 *   - symbol resolution;
 *   - visibility checking;
 *   - capability checking;
 *   - type checking;
 *   - effects;
 *   - resources;
 *   - hardware;
 *   - quantum devices;
 *   - QEC;
 *   - ZQN;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - runtime execution;
 *   - canonical IR construction.
 *
 * ============================================================================
 * CRITICAL SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The following source:
 *
 *     import quantum::algorithm;
 *
 * means only:
 *
 *     "the source program contains an import declaration whose target is
 *      the qualified name quantum::algorithm."
 *
 * It does NOT mean:
 *
 *     - open a file named quantum::algorithm;
 *     - access a filesystem;
 *     - contact a package registry;
 *     - select a quantum processor;
 *     - select a number of qubits;
 *     - select a topology;
 *     - allocate hardware;
 *     - load a runtime;
 *     - invoke quantum::ir;
 *     - compile anything.
 *
 * Those decisions belong to downstream semantic/toolchain layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Import syntax is intentionally independent of execution scale.
 *
 * There are no grammar-level limits on:
 *
 *     - number of imports;
 *     - number of imported symbols;
 *     - number of qualified-name segments;
 *     - number of aliases;
 *     - number of source modules;
 *     - number of packages;
 *     - number of compilation units;
 *     - number of domains;
 *     - number of quantum resources;
 *     - number of hardware resources;
 *     - number of execution targets.
 *
 * Repetition uses ANTLR repetition operators rather than finite constants.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_IMPORTS
 *     MAX_MODULES
 *     MAX_IMPORT_DEPTH
 *     MAX_PACKAGES
 *     MAX_TARGETS
 *
 * or equivalent limitations.
 *
 * ============================================================================
 * IMPORT SOURCE MODEL
 * ============================================================================
 *
 * Zamani supports two syntactic categories of import source:
 *
 *   1. symbolic source
 *
 *        import quantum::algorithm;
 *
 *   2. explicit source literal
 *
 *        import {algorithm} from "library";
 *
 * The grammar preserves the distinction.
 *
 * A string literal is opaque syntax here.
 *
 * It may later represent:
 *
 *   - a package coordinate;
 *   - a workspace source;
 *   - an embedded module;
 *   - a generated module;
 *   - a registry coordinate;
 *   - a logical source identifier;
 *   - another compiler-defined source provider.
 *
 * The grammar does not decide which.
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * `identifier` and `qualifiedName` are owned by:
 *
 *     grammar/core/names.g4
 *
 * This grammar imports Names rather than redefining those rules.
 *
 * This is essential because:
 *
 *     math::linear
 *     quantum::ir
 *     hardware::capability
 *
 * must have exactly one canonical qualified-name syntax throughout Zamani.
 *
 * ============================================================================
 * LEXER OWNERSHIP
 * ============================================================================
 *
 * Keyword and punctuation tokens are supplied by the canonical Zamani lexer:
 *
 *     grammar/lexer/tokens.g4
 *
 * Relevant tokens include:
 *
 *     K_IMPORT
 *     K_FROM
 *     K_AS
 *     DOUBLE_COLON
 *     STAR
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     SEMICOLON
 *     STRING_LITERAL
 *     IDENTIFIER
 *
 * This grammar does not redefine any of them.
 *
 * ============================================================================
 * NO FILESYSTEM PATH SEMANTICS
 * ============================================================================
 *
 * Import targets are symbolic language names.
 *
 * This grammar deliberately does not interpret:
 *
 *     /
 *     \
 *     .
 *     ..
 *     C:
 *     file extensions
 *     URLs
 *
 * as filesystem semantics.
 *
 * If Zamani later supports explicit path-based imports, that syntax must be
 * introduced through the canonical path grammar and an explicit import-path
 * construct rather than silently changing the meaning of qualified names.
 *
 * ============================================================================
 * NO TARGET COUPLING
 * ============================================================================
 *
 * An import must never encode:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC identity
 *     QPU identity
 *     qubit count
 *     topology
 *     memory capacity
 *     network topology
 *     deployment node
 *     accelerator count
 *
 * Such information belongs to resource/capability/target semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * These rules:
 *
 *     - perform no I/O;
 *     - perform no network operations;
 *     - inspect no environment variables;
 *     - inspect no hardware;
 *     - access no clock;
 *     - use no randomness;
 *     - invoke no runtime;
 *     - perform no semantic lookup.
 *
 * Therefore parsing is deterministic for a deterministic token stream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The downstream AST should be able to preserve:
 *
 *     - import declaration span;
 *     - import kind;
 *     - target qualified-name segments;
 *     - imported symbol names;
 *     - aliases;
 *     - wildcard marker;
 *     - source literal;
 *     - source spans;
 *     - source ordering.
 *
 * Recommended semantic categories:
 *
 *     SymbolImport
 *     NamedImport
 *     WildcardImport
 *
 * These are AST/semantic concepts, not grammar-owned types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis is responsible for:
 *
 *     - resolving module identities;
 *     - resolving packages;
 *     - resolving symbols;
 *     - checking aliases;
 *     - checking duplicate imports;
 *     - checking visibility;
 *     - detecting dependency cycles;
 *     - determining source-provider semantics;
 *     - validating package/version requirements;
 *     - checking capabilities;
 *     - constructing the module dependency graph.
 *
 * None of those checks belong here.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar never constructs IR.
 *
 * Import declarations are consumed by module/name semantic analysis before
 * program semantics are lowered.
 *
 * Quantum imports must eventually feed quantum semantic lowering and,
 * where quantum computation is involved, the repository's canonical
 * `quantum::ir`.
 *
 * This grammar must never define:
 *
 *     QuantumImportIR
 *     QuantumModuleIR
 *     HardwareImportIR
 *
 * or equivalent duplicate representations.
 *
 * ============================================================================
 * COMPILER / RUNTIME CONTRACT
 * ============================================================================
 *
 * Compiler:
 *
 *     parser
 *       -> AST
 *       -> module resolution
 *       -> semantic analysis
 *       -> canonical semantic IR
 *
 * Runtime:
 *
 *     NO DIRECT DEPENDENCY.
 *
 * Import resolution must happen before runtime execution and must not cause
 * this grammar to depend on runtime APIs.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * IDEs, formatters, documentation tools, dependency analyzers and language
 * servers may consume the parse tree/AST produced from this grammar.
 *
 * The parser must preserve enough source structure for:
 *
 *     - import navigation;
 *     - go-to-definition;
 *     - dependency visualization;
 *     - import sorting;
 *     - unused-import analysis;
 *     - rename operations;
 *     - diagnostics;
 *     - formatting;
 *     - source-preserving transformations.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file is the canonical modular replacement for older module grammar
 * implementations that used obsolete token names such as:
 *
 *     IMPORT
 *     FROM
 *     AS
 *     SEMI
 *
 * The current lexical authority uses:
 *
 *     K_IMPORT
 *     K_FROM
 *     K_AS
 *     SEMICOLON
 *
 * No compatibility aliases are created inside this file.
 *
 * Compatibility belongs to the language-versioning/migration layer.
 *
 * ============================================================================
 */

parser grammar Imports;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Names owns:
 *
 *     identifier
 *     qualifiedName
 *
 * Import syntax consumes those canonical rules.
 */
import Names;


/*
 * ============================================================================
 * 1. PUBLIC IMPORT DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     import <clause>;
 *
 * Examples:
 *
 *     import math;
 *     import math::linear;
 *     import math::linear as linear;
 *     import {Vector, Matrix};
 *     import {Vector as V, Matrix};
 *     import *;
 *
 * Source-qualified forms are also supported:
 *
 *     import {Vector} from "math";
 *     import * from "math";
 *     import * as math from "math";
 *     import math::linear from "math";
 *
 * The terminating semicolon is required.
 *
 * Requiring the terminator gives deterministic declaration boundaries and
 * avoids coupling import parsing to newline/trivia behavior.
 */
importDeclaration
    : K_IMPORT importClause SEMICOLON
    ;


/*
 * ============================================================================
 * 2. IMPORT CLAUSE
 * ============================================================================
 *
 * An import has one of three structural forms:
 *
 *     qualified target
 *     wildcard target
 *     named target list
 *
 * An optional source qualifier may follow any target form where syntactically
 * meaningful.
 */
importClause
    : qualifiedImportClause
    | wildcardImportClause
    | namedImportClause
    ;


/*
 * ============================================================================
 * 3. QUALIFIED IMPORT
 * ============================================================================
 *
 * Examples:
 *
 *     import math;
 *     import math::linear;
 *     import math::linear as linear;
 *     import math::linear from "math";
 *     import math::linear as linear from "math";
 *
 * The source qualifier is syntactic only.
 */
qualifiedImportClause
    : qualifiedName importAlias? importSource?
    ;


/*
 * ============================================================================
 * 4. QUALIFIED IMPORT ALIAS
 * ============================================================================
 *
 * Example:
 *
 *     import quantum::algorithm as algorithm;
 *
 * The imported qualified name remains canonical.
 *
 * The alias is another canonical identifier.
 */
importAlias
    : K_AS identifier
    ;


/*
 * ============================================================================
 * 5. NAMED IMPORT
 * ============================================================================
 *
 * Examples:
 *
 *     import {Vector};
 *     import {Vector, Matrix};
 *     import {Vector as V, Matrix};
 *     import {Vector, Matrix, Tensor,};
 *
 * A trailing comma is accepted.
 *
 * No finite number of imported symbols is imposed.
 */
namedImportClause
    : LBRACE importSpecifierList RBRACE importSource?
    ;


/*
 * ============================================================================
 * 6. IMPORT SPECIFIER LIST
 * ============================================================================
 *
 * The list is non-empty.
 *
 * Empty:
 *
 *     import {};
 *
 * is intentionally rejected by the grammar because it has no syntactic
 * import target and provides no useful source-level meaning.
 *
 * Semantic validation remains responsible for duplicate names and visibility.
 */
importSpecifierList
    : importSpecifier (COMMA importSpecifier)* COMMA?
    ;


/*
 * ============================================================================
 * 7. IMPORT SPECIFIER
 * ============================================================================
 *
 * A named import uses one qualified name with an optional local alias.
 *
 * Examples:
 *
 *     Vector
 *     math::Vector
 *     math::Vector as Vector
 *
 * Keeping the target as a qualifiedName allows the syntax to scale to nested
 * module namespaces without introducing a depth limit.
 */
importSpecifier
    : qualifiedName importAlias?
    ;


/*
 * ============================================================================
 * 8. WILDCARD IMPORT
 * ============================================================================
 *
 * Examples:
 *
 *     import *;
 *     import * as math;
 *     import * from "math";
 *     import * as math from "math";
 *
 * Wildcard semantics are NOT determined by this grammar.
 *
 * In particular, this grammar does not decide whether wildcard import means:
 *
 *     - all public declarations;
 *     - all names in a namespace;
 *     - all exports;
 *     - all symbols from a source provider.
 *
 * That is a semantic/module-system policy.
 */
wildcardImportClause
    : STAR importAlias? importSource?
    ;


/*
 * ============================================================================
 * 9. IMPORT SOURCE
 * ============================================================================
 *
 * Example:
 *
 *     from "math";
 *
 * The string literal is intentionally opaque.
 *
 * The grammar does not interpret it as:
 *
 *     - filesystem path;
 *     - URL;
 *     - package registry coordinate;
 *     - device address;
 *     - hardware identifier.
 *
 * The module/source resolver assigns that meaning.
 */
importSource
    : K_FROM importSourceLiteral
    ;


/*
 * ============================================================================
 * 10. IMPORT SOURCE LITERAL
 * ============================================================================
 *
 * This wrapper exists only to give import syntax an explicit parse-tree node.
 *
 * STRING_LITERAL remains owned by the canonical lexer.
 *
 * No string lexical syntax is duplicated here.
 */
importSourceLiteral
    : STRING_LITERAL
    ;


/*
 * ============================================================================
 * 11. IMPORT TARGET
 * ============================================================================
 *
 * Generic integration wrapper.
 *
 * This rule is useful to downstream grammar composition when a construct needs
 * to accept an import target without caring which concrete import form was
 * selected.
 */
importTarget
    : qualifiedImportTarget
    | wildcardImportTarget
    | namedImportTarget
    ;


/*
 * ============================================================================
 * 12. QUALIFIED IMPORT TARGET
 * ============================================================================
 *
 * This is deliberately separated from `qualifiedImportClause`.
 *
 * `qualifiedImportClause` owns the complete declaration-level form.
 *
 * `qualifiedImportTarget` owns only the target itself.
 */
qualifiedImportTarget
    : qualifiedName
    ;


/*
 * ============================================================================
 * 13. WILDCARD IMPORT TARGET
 * ============================================================================
 */
wildcardImportTarget
    : STAR
    ;


/*
 * ============================================================================
 * 14. NAMED IMPORT TARGET
 * ============================================================================
 *
 * This wrapper exposes a named target without the surrounding `import`
 * keyword or source qualifier.
 */
namedImportTarget
    : LBRACE importSpecifierList RBRACE
    ;


/*
 * ============================================================================
 * 15. IMPORT TARGET LIST
 * ============================================================================
 *
 * Generic reusable list for tooling or higher-level grammar composition.
 */
importTargetList
    : importTarget (COMMA importTarget)*
    ;


/*
 * ============================================================================
 * 16. IMPORT SOURCE LIST
 * ============================================================================
 *
 * This is intentionally provided as a reusable syntax boundary for future
 * language constructs that may associate multiple source descriptors with
 * import declarations.
 *
 * It does NOT imply that multiple sources are currently legal in a normal
 * `importDeclaration`.
 */
importSourceList
    : importSource (COMMA importSource)*
    ;


/*
 * ============================================================================
 * 17. IMPORT ALIAS LIST
 * ============================================================================
 *
 * Generic alias list for tooling/composition.
 *
 * Normal imports do not use this rule directly because aliases are attached
 * to their imported target.
 */
importAliasList
    : importAlias (COMMA importAlias)*
    ;


/*
 * ============================================================================
 * 18. OPTIONAL IMPORT SOURCE
 * ============================================================================
 *
 * Integration wrapper.
 */
optionalImportSource
    : importSource?
    ;


/*
 * ============================================================================
 * 19. OPTIONAL IMPORT ALIAS
 * ============================================================================
 *
 * Integration wrapper.
 */
optionalImportAlias
    : importAlias?
    ;


/*
 * ============================================================================
 * 20. IMPORT DECLARATION LIST
 * ============================================================================
 *
 * No finite number of imports is imposed.
 *
 * This rule is useful where a module grammar wants to explicitly represent
 * a contiguous import section.
 *
 * The top-level module grammar may instead use `importDeclaration*` directly.
 */
importDeclarationList
    : importDeclaration+
    ;


/*
 * ============================================================================
 * 21. OPTIONAL IMPORT DECLARATION LIST
 * ============================================================================
 */
optionalImportDeclarationList
    : importDeclarationList?
    ;


/*
 * ============================================================================
 * 22. IMPORT SECTION
 * ============================================================================
 *
 * A section may contain zero or more imports.
 *
 * This is a composition rule, not a declaration of module ordering semantics.
 *
 * Whether imports must precede declarations is owned by the module/source
 * grammar.
 */
importSection
    : importDeclaration*
    ;


/*
 * ============================================================================
 * 23. NON-EMPTY IMPORT SECTION
 * ============================================================================
 */
nonEmptyImportSection
    : importDeclaration+
    ;


/*
 * ============================================================================
 * 24. IMPORT SOURCE NAME
 * ============================================================================
 *
 * A source may be represented by a language-level qualified name through
 * `importSourceName`.
 *
 * This is intentionally separate from `importSourceLiteral`.
 *
 * Example:
 *
 *     import math::linear;
 *
 * uses the symbolic import target.
 *
 * An explicit:
 *
 *     from "math"
 *
 * uses `importSourceLiteral`.
 *
 * The two forms must not be conflated in the AST.
 */
importSourceName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 25. IMPORT SOURCE REFERENCE
 * ============================================================================
 *
 * Generic source reference preserving whether the source was expressed as
 * a symbolic name or a literal.
 *
 * This is a syntax-only distinction.
 */
importSourceReference
    : importSourceName
    | importSourceLiteral
    ;


/*
 * ============================================================================
 * 26. OPTIONAL IMPORT SOURCE REFERENCE
 * ============================================================================
 */
optionalImportSourceReference
    : importSourceReference?
    ;


/*
 * ============================================================================
 * 27. NAMED IMPORT SPECIFIER LIST
 * ============================================================================
 *
 * Explicit alias-aware wrapper.
 *
 * This rule exists so tooling can refer directly to the list structure without
 * reconstructing it from the declaration rule.
 */
namedImportSpecifierList
    : importSpecifierList
    ;


/*
 * ============================================================================
 * 28. IMPORT SPECIFIER WITH SOURCE
 * ============================================================================
 *
 * Reusable syntax wrapper for:
 *
 *     symbol from "source"
 *
 * This is not itself a complete declaration.
 */
importSpecifierWithSource
    : importSpecifier importSource
    ;


/*
 * ============================================================================
 * 29. WILDCARD IMPORT WITH SOURCE
 * ============================================================================
 *
 * Reusable syntax wrapper for:
 *
 *     * from "source"
 *
 * This is not itself a complete declaration.
 */
wildcardImportWithSource
    : STAR importSource
    ;


/*
 * ============================================================================
 * 30. QUALIFIED IMPORT WITH SOURCE
 * ============================================================================
 *
 * Reusable syntax wrapper for:
 *
 *     qualifiedName from "source"
 *
 * This is not itself a complete declaration.
 */
qualifiedImportWithSource
    : qualifiedName importSource
    ;


/*
 * ============================================================================
 * 31. QUALIFIED IMPORT WITH ALIAS AND SOURCE
 * ============================================================================
 *
 * Reusable syntax wrapper for:
 *
 *     qualifiedName as identifier from "source"
 */
qualifiedImportWithAliasAndSource
    : qualifiedName importAlias importSource
    ;


/*
 * ============================================================================
 * 32. WILDCARD IMPORT WITH ALIAS AND SOURCE
 * ============================================================================
 *
 * Reusable syntax wrapper for:
 *
 *     * as identifier from "source"
 */
wildcardImportWithAliasAndSource
    : STAR importAlias importSource
    ;


/*
 * ============================================================================
 * 33. SEMANTIC BOUNDARY DOCUMENTATION
 * ============================================================================
 *
 * The following operations are intentionally absent:
 *
 *     resolveImport
 *     resolveModule
 *     resolvePackage
 *     loadImport
 *     fetchImport
 *     openImport
 *     readImport
 *     installPackage
 *     downloadPackage
 *     verifyPackage
 *     selectBackend
 *     selectDevice
 *     allocateResource
 *
 * They belong to later compiler/toolchain layers.
 *
 * ============================================================================
 * MODULE GRAPH BOUNDARY
 * ============================================================================
 *
 * Given:
 *
 *     import quantum::algorithms::search as search;
 *
 * the parser produces syntax equivalent to:
 *
 *     ImportDeclaration
 *       Target:
 *         QualifiedName
 *           quantum
 *           algorithms
 *           search
 *       Alias:
 *         search
 *
 * The semantic layer may subsequently determine:
 *
 *     source module
 *     package
 *     workspace dependency
 *     embedded module
 *     generated module
 *
 * without changing the grammar.
 *
 * ============================================================================
 * SCALABILITY BOUNDARY
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_IMPORTS
 *     MAX_IMPORT_DEPTH
 *     MAX_SYMBOLS
 *     MAX_MODULES
 *     MAX_PACKAGES
 *     MAX_TARGETS
 *     MAX_QUANTUM_RESOURCES
 *     MAX_HARDWARE_RESOURCES
 *
 * Any operational parser/compiler limits must be configurable implementation
 * policy rather than source-language grammar constants.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Imports can name entities from:
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
 *
 * No domain-specific import grammar is created here.
 *
 * For example:
 *
 *     import quantum::algorithm;
 *
 * and:
 *
 *     import hardware::capability;
 *
 * have exactly the same syntactic import model.
 *
 * Their semantic interpretation belongs to their respective owning domains.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * This grammar does not know what a quantum import contains.
 *
 * It must not define:
 *
 *     QuantumModule
 *     QuantumImportIR
 *     QubitImport
 *     GateImport
 *
 * Quantum syntax is lowered through the repository's normal frontend and
 * semantic pipeline and ultimately uses canonical `quantum::ir` where
 * appropriate.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware imports identify source-level symbols only.
 *
 * They do not select physical hardware.
 *
 * Hardware discovery, capabilities, topology, placement and target selection
 * remain owned by the hardware abstraction/resource/compiler layers.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * There is intentionally no runtime dependency.
 *
 * Runtime availability cannot alter the grammar accepted by the parser.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This `.g4` file contains no Rust code.
 *
 * The Rust frontend generated/integrated from the grammar must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * under the repository's safe-Rust policy.
 *
 * No grammar action, semantic predicate, or generated integration contract in
 * this file requires `unsafe`.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] It consumes the canonical Zamani lexer vocabulary.
 *   [x] It uses K_IMPORT rather than obsolete IMPORT.
 *   [x] It uses K_FROM rather than obsolete FROM.
 *   [x] It uses K_AS rather than obsolete AS.
 *   [x] It uses SEMICOLON rather than obsolete SEMI.
 *   [x] It reuses canonical `identifier`.
 *   [x] It reuses canonical `qualifiedName`.
 *   [x] It does not redefine identifier syntax.
 *   [x] It does not redefine qualified-name syntax.
 *   [x] It does not perform module resolution.
 *   [x] It does not access files or networks.
 *   [x] It does not select hardware.
 *   [x] It does not create IR.
 *   [x] It contains no machine-size constants.
 *   [x] It contains no qubit limits.
 *   [x] It contains no device limits.
 *   [x] It contains no topology assumptions.
 *   [x] It contains no embedded Rust.
 *   [x] It requires no unsafe code.
 *   [x] It supports arbitrarily repeated imports within available resources.
 *   [x] It supports arbitrarily deep qualified names within available
 *       parser/compiler resources.
 *   [x] It preserves symbolic-vs-literal import-source structure.
 *   [x] It can be integrated into the canonical Zamani parser without
 *       redefining shared lexical/name rules.
 *
 * ============================================================================
 */