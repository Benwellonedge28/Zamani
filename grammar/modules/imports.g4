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
 * Status:
 *     CANONICAL MODULE IMPORT GRAMMAR
 *
 * Purpose:
 *     Define the complete source-level syntax of Zamani import declarations
 *     while delegating names, lexical tokens, literals, semantic resolution,
 *     package resolution, dependency resolution, and target realization to
 *     their canonical owners.
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
 * Rust safety:
 *     Zamani Rust implementation MUST remain safe Rust.
 *     No unsafe Rust is required by this grammar or its integration contract.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The import grammar participates in the following pipeline:
 *
 *     source
 *        |
 *        v
 *     grammar/lexer/*
 *        |
 *        v
 *     grammar/antlr/ZamaniLexer.g4
 *        |
 *        v
 *     grammar/modules/imports.g4
 *        |
 *        v
 *     grammar/antlr/ZamaniParser.g4
 *        |
 *        v
 *     domain-neutral frontend AST
 *        |
 *        v
 *     module / package / symbol resolution
 *        |
 *        v
 *     semantic analysis
 *        |
 *        +-------------------+-------------------+
 *        |                   |                   |
 *        v                   v                   v
 *     classical          quantum::ir       HDL/hardware
 *        |                   |                   |
 *        +-------------------+-------------------+
 *                            |
 *                            v
 *                     optimization
 *                            |
 *                     routing/scheduling
 *                            |
 *                     resilience/QEC/ZQN
 *                            |
 *                            v
 *                           HAL
 *                            |
 *                            v
 *                     target realization
 *
 * THIS FILE IS A SYNTAX COMPONENT ONLY.
 *
 * It MUST NOT:
 *
 *     - resolve imports;
 *     - access the filesystem;
 *     - access a registry;
 *     - access a network;
 *     - inspect environment variables;
 *     - discover hardware;
 *     - discover resources;
 *     - select a compiler backend;
 *     - select a CPU;
 *     - select a GPU;
 *     - select an FPGA;
 *     - select an ASIC;
 *     - select a QPU;
 *     - allocate qubits;
 *     - allocate memory;
 *     - construct classical IR;
 *     - construct quantum::ir;
 *     - perform QEC;
 *     - perform ZQN analysis;
 *     - perform routing;
 *     - perform scheduling;
 *     - execute code.
 *
 * ============================================================================
 * AUTHORITY AND OWNERSHIP
 * ============================================================================
 *
 * Normative syntax:
 *
 *     grammar/spec/syntax.md
 *     grammar/specification/syntax.md
 *
 * Grammar architecture:
 *
 *     grammar/DESIGN.md
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical composition:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical source-level names:
 *
 *     grammar/core/names.g4
 *
 * Qualified-name integration:
 *
 *     grammar/core/qualified-names.g4
 *
 * Module composition:
 *
 *     grammar/modules/modules.g4
 *
 * Exports:
 *
 *     grammar/modules/exports.g4
 *
 * Packages:
 *
 *     grammar/modules/packages.g4
 *
 * Dependencies:
 *
 *     grammar/modules/dependencies.g4
 *
 * Namespaces:
 *
 *     grammar/modules/namespaces.g4
 *
 * Visibility:
 *
 *     grammar/modules/visibility.g4
 *
 * Module attributes:
 *
 *     grammar/modules/module-attributes.g4
 *
 * This file MUST NOT become an independent syntax authority.
 *
 * ============================================================================
 * THIS FILE OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     - import declarations;
 *     - import clauses;
 *     - direct qualified-symbol imports;
 *     - named-symbol imports;
 *     - wildcard imports;
 *     - import aliases;
 *     - import source qualifiers;
 *     - symbolic import sources;
 *     - literal import sources;
 *     - import-specifier lists;
 *     - import-section syntax;
 *     - import-specific structural wrappers.
 *
 * ============================================================================
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - IDENTIFIER;
 *     - keyword spellings;
 *     - punctuation;
 *     - operators;
 *     - string literal syntax;
 *     - qualified-name syntax;
 *     - path syntax;
 *     - module declarations;
 *     - package declarations;
 *     - dependency declarations;
 *     - namespace declarations;
 *     - exports;
 *     - visibility;
 *     - attributes;
 *     - declarations;
 *     - expressions;
 *     - types;
 *     - semantic resolution;
 *     - package resolution;
 *     - dependency solving;
 *     - symbol resolution;
 *     - visibility checking;
 *     - capability checking;
 *     - resource checking;
 *     - hardware selection;
 *     - target selection;
 *     - IR construction;
 *     - runtime loading.
 *
 * ============================================================================
 * CRITICAL NAME OWNERSHIP
 * ============================================================================
 *
 * `grammar/core/names.g4` is the canonical owner of:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *     nameList
 *     qualifiedNameList
 *     nameAlias
 *     nameReference
 *     nameReferenceList
 *
 * This grammar MUST reuse those rules.
 *
 * It MUST NOT define another version of:
 *
 *     identifier
 *     qualifiedName
 *     nameSegment
 *
 * In particular, this grammar MUST NOT contain:
 *
 *     identifier (DOUBLE_COLON identifier)*
 *
 * or an equivalent duplicate.
 *
 * ============================================================================
 * LEXICAL TOKEN CONTRACT
 * ============================================================================
 *
 * Parser grammars consume the canonical production lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The actual canonical keyword names in the repository are:
 *
 *     IMPORT
 *     FROM
 *     AS
 *
 * NOT:
 *
 *     K_IMPORT
 *     K_FROM
 *     K_AS
 *
 * The canonical string literal token is:
 *
 *     STRING
 *
 * NOT:
 *
 *     STRING_LITERAL
 *
 * Other punctuation/operator tokens are consumed exactly as supplied by
 * ZamaniLexer.
 *
 * This grammar MUST NOT redefine any lexical token.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It imports the canonical name grammar directly:
 *
 *     Names
 *
 * `Names` is used instead of defining another qualified-name grammar here.
 *
 * The dependency direction is:
 *
 *     ZamaniTokens
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Names
 *          |
 *          v
 *     Imports
 *          |
 *          v
 *     Modules / ZamaniParser
 *
 * `grammar/core/qualified-names.g4` remains an integration layer for consumers
 * that require qualified-name/path wrappers. This file does not need to import
 * it because imports require only the canonical `qualifiedName` rule owned by
 * `Names`.
 *
 * This avoids introducing an unnecessary additional grammar dependency path.
 *
 * ============================================================================
 * IMPORT MODEL
 * ============================================================================
 *
 * Zamani supports three source-level import target forms:
 *
 *     1. qualified import
 *     2. named import list
 *     3. wildcard import
 *
 * Examples:
 *
 *     import math;
 *
 *     import math::linear;
 *
 *     import math::linear as linear;
 *
 *     import {Vector, Matrix};
 *
 *     import {Vector as V, Matrix};
 *
 *     import *;
 *
 * Optional source qualification may be used:
 *
 *     import {Vector} from "math";
 *
 *     import * from "math";
 *
 *     import math::linear from "math";
 *
 *     import math::linear as linear from "math";
 *
 * A source qualifier is syntactic data.
 *
 * It does not itself establish:
 *
 *     - a filesystem path;
 *     - a URL;
 *     - a package registry;
 *     - a workspace;
 *     - a remote source;
 *     - a local source;
 *     - a generated source;
 *     - a hardware source.
 *
 * The semantic/toolchain layer determines the meaning of the source reference.
 *
 * ============================================================================
 * SYMBOLIC VERSUS LITERAL SOURCE
 * ============================================================================
 *
 * A source qualifier may be represented as either:
 *
 *     qualifiedName
 *
 * or:
 *
 *     STRING
 *
 * Examples:
 *
 *     from math::linear
 *
 *     from "math"
 *
 * The parser preserves the distinction.
 *
 * Semantic analysis decides whether the source reference denotes:
 *
 *     - a module;
 *     - a package;
 *     - a workspace;
 *     - an embedded module;
 *     - a generated module;
 *     - a registry coordinate;
 *     - another source provider.
 *
 * The grammar does not make that decision.
 *
 * ============================================================================
 * IMPORT TARGET SEMANTICS
 * ============================================================================
 *
 * A direct qualified import:
 *
 *     import quantum::algorithms;
 *
 * represents a source-level symbolic reference.
 *
 * It does NOT mean:
 *
 *     use a QPU;
 *     allocate qubits;
 *     select a backend;
 *     invoke quantum::ir;
 *     load a runtime.
 *
 * Likewise:
 *
 *     import hardware::capability;
 *
 * does not select a physical hardware capability.
 *
 * Hardware capability resolution occurs downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Import syntax is target-independent.
 *
 * There are NO grammar-level limits on:
 *
 *     - number of imports;
 *     - number of imported symbols;
 *     - number of import declarations;
 *     - number of qualified-name segments;
 *     - number of aliases;
 *     - number of modules;
 *     - number of packages;
 *     - number of dependencies;
 *     - number of compilation units;
 *     - number of domains;
 *     - number of quantum resources;
 *     - number of hardware resources;
 *     - number of deployment targets.
 *
 * The grammar MUST NOT introduce:
 *
 *     MAX_IMPORTS
 *     MAX_IMPORT_DEPTH
 *     MAX_IMPORTED_SYMBOLS
 *     MAX_MODULES
 *     MAX_PACKAGES
 *     MAX_DEPENDENCIES
 *     MAX_TARGETS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Practical parser/compiler limits MAY exist as configurable implementation
 * resource budgets. Such limits MUST NOT become source-language semantics.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Import declarations MUST NOT encode:
 *
 *     CPU identity
 *     CPU count
 *     core count
 *     thread count
 *     GPU identity
 *     GPU count
 *     FPGA identity
 *     FPGA count
 *     ASIC identity
 *     QPU identity
 *     physical qubit
 *     qubit count
 *     memory bank
 *     memory capacity
 *     network node
 *     topology
 *     accelerator count
 *     physical address
 *
 * Target realization belongs downstream to:
 *
 *     resources
 *     capabilities
 *     compile
 *     execution
 *     hardware
 *     HAL
 *     scheduling
 *     routing
 *     deployment
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     - actions;
 *     - semantic predicates;
 *     - filesystem access;
 *     - network access;
 *     - registry access;
 *     - environment lookup;
 *     - hardware discovery;
 *     - resource discovery;
 *     - randomness;
 *     - clock access;
 *     - runtime execution.
 *
 * Given the same token stream and grammar version, the parser MUST produce the
 * same syntactic interpretation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for a domain-neutral AST to
 * represent at least:
 *
 *     ImportDeclaration
 *         target
 *         alias
 *         source
 *         source_span
 *
 * Target variants are conceptually:
 *
 *     QualifiedImport
 *     NamedImport
 *     WildcardImport
 *
 * Source variants are conceptually:
 *
 *     NoSource
 *     SymbolicSource
 *     LiteralSource
 *
 * A named import item conceptually contains:
 *
 *     importedName
 *     localAlias?
 *
 * The exact Rust AST type names belong to the frontend AST owner.
 *
 * This grammar MUST NOT create a second module AST or a second quantum AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic/module analysis owns:
 *
 *     - source-provider resolution;
 *     - module identity resolution;
 *     - package resolution;
 *     - dependency resolution;
 *     - symbol resolution;
 *     - alias validation;
 *     - duplicate-import detection;
 *     - visibility checking;
 *     - export checking;
 *     - dependency graph construction;
 *     - dependency cycle detection;
 *     - version compatibility;
 *     - capability requirements;
 *     - resource requirements;
 *     - dialect compatibility;
 *     - interoperability validation.
 *
 * None of these checks belong in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Import declarations do not directly lower to execution IR.
 *
 * Their information is consumed during source/module semantic analysis.
 *
 * The resulting program semantics then follow the repository's canonical path:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *     classical model     quantum semantics    HDL/hardware
 *                              |
 *                              v
 *                         quantum::ir
 *                              |
 *                              v
 *                     optimization/lowering
 *                              |
 *                     routing/scheduling
 *                              |
 *                     QEC/resilience/ZQN
 *                              |
 *                             HAL
 *
 * This grammar MUST NOT define:
 *
 *     QuantumImportIR
 *     ModuleIR
 *     ImportIR
 *     HardwareImportIR
 *
 * or equivalent duplicate IR structures.
 *
 * ============================================================================
 * MODULE INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/modules.g4` owns module declaration syntax.
 *
 * It should consume:
 *
 *     importDeclaration
 *
 * through the canonical aggregate `item`/source-element composition supplied
 * by the parser architecture.
 *
 * `modules.g4` MUST NOT redefine:
 *
 *     importDeclaration
 *     importClause
 *     importSpecifier
 *     importAlias
 *     importSource
 *
 * This keeps imports independently completable.
 *
 * ============================================================================
 * EXPORT INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/exports.g4` owns:
 *
 *     exportDeclaration
 *     exportClause
 *     exportSpecifier
 *     re-export syntax
 *
 * Imports and exports intentionally remain separate grammar owners.
 *
 * A re-export is an export construct, not an import construct, even if it
 * contains source-reference syntax.
 *
 * ============================================================================
 * PACKAGE INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/packages.g4` owns package syntax.
 *
 * This grammar does not define package declarations.
 *
 * A literal source such as:
 *
 *     from "math"
 *
 * remains opaque until semantic/toolchain resolution.
 *
 * It must not be interpreted by this grammar as a filesystem directory.
 *
 * ============================================================================
 * DEPENDENCY INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/dependencies.g4` owns dependency declaration syntax.
 *
 * An import declaration references source-level symbols/modules.
 *
 * A dependency declaration describes source-level dependency requirements.
 *
 * These are related semantic concepts but are not the same syntax and must
 * remain separate.
 *
 * ============================================================================
 * NAMESPACE INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/namespaces.g4` owns namespace declarations.
 *
 * A qualified name used by an import is syntactically neutral.
 *
 * For example:
 *
 *     quantum::algorithms
 *
 * does not imply that `quantum` is:
 *
 *     - a namespace;
 *     - a module;
 *     - a package;
 *     - a directory;
 *     - a domain;
 *     - a hardware resource.
 *
 * Semantic analysis determines its identity.
 *
 * ============================================================================
 * VISIBILITY INTEGRATION
 * ============================================================================
 *
 * Visibility checking belongs to:
 *
 *     grammar/modules/visibility.g4
 *     semantic analysis
 *
 * Imports do not introduce a second visibility vocabulary.
 *
 * The grammar accepts import syntax without determining whether the imported
 * entity is accessible.
 *
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attributes attached to an import declaration, if the language specification
 * permits them, belong to the canonical attribute system.
 *
 * This file deliberately does not introduce a second attribute syntax.
 *
 * If import attributes are standardized later, they must be composed through
 * the canonical attribute grammar rather than adding ad-hoc import-only
 * attribute tokens here.
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * Dialects may extend import semantics only through the established dialect
 * extension mechanism.
 *
 * A dialect MUST NOT silently replace:
 *
 *     importDeclaration
 *
 * with an incompatible second import language.
 *
 * Dialect-specific import forms must be:
 *
 *     - explicitly declared;
 *     - versioned;
 *     - unambiguous;
 *     - mapped to the domain-neutral AST;
 *     - semantically validated;
 *     - compatibility-tested.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Imports may ultimately reference interoperability providers for:
 *
 *     C
 *     C++
 *     Rust
 *     Python
 *     WebAssembly
 *     OpenQASM
 *     QIR
 *     HDL formats
 *     vendor-neutral interfaces
 *     future formats
 *
 * The import grammar remains format-neutral.
 *
 * Interoperability semantics belong to:
 *
 *     grammar/interoperability/
 *
 * and downstream semantic/compiler layers.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum modules are imported exactly like other source-level modules.
 *
 * Example:
 *
 *     import quantum::algorithms;
 *
 * The grammar does not know:
 *
 *     - gate sets;
 *     - qubit counts;
 *     - physical qubits;
 *     - topology;
 *     - QPU identity;
 *     - calibration;
 *     - pulse representation;
 *     - scheduling;
 *     - routing;
 *     - QEC.
 *
 * Once imported declarations are semantically resolved, quantum constructs
 * follow the existing canonical pipeline to:
 *
 *     quantum::ir
 *
 * and then:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     resilience/QEC
 *     ZQN
 *     HAL
 *     target realization
 *
 * No import syntax may create a second quantum semantic boundary.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE / OTHER DOMAINS
 * ============================================================================
 *
 * The same import syntax applies to:
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
 *     interoperability
 *     dialects
 *     macros
 *     metaprogramming
 *     future domains
 *
 * No domain-specific import grammar is required merely because the imported
 * declaration belongs to a different computational domain.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors owned by this grammar include:
 *
 *     import;
 *
 *     import ;
 *
 *     import {};
 *
 *     import { };
 *
 *     import {Vector,};
 *
 *     import math::;
 *
 *     import ::math;
 *
 *     import math::linear as;
 *
 *     import math::linear from;
 *
 *     import * as;
 *
 *     import * from;
 *
 *     import {Vector as};
 *
 *     import {Vector,,Matrix};
 *
 *     import math::linear from "source" extra;
 *
 * Semantic errors NOT owned by this grammar include:
 *
 *     unresolved module;
 *     unresolved symbol;
 *     inaccessible symbol;
 *     duplicate imported binding;
 *     duplicate alias;
 *     dependency cycle;
 *     incompatible package version;
 *     unavailable source provider;
 *     unavailable capability;
 *     unavailable hardware;
 *     insufficient memory;
 *     insufficient qubits;
 *     unsupported target;
 *
 * ============================================================================
 * COMPATIBILITY POLICY
 * ============================================================================
 *
 * Historical token names such as:
 *
 *     K_IMPORT
 *     K_FROM
 *     K_AS
 *     STRING_LITERAL
 *
 * are NOT used because they do not match the current canonical lexical
 * vocabulary.
 *
 * The current repository's canonical lexical spellings are represented by:
 *
 *     IMPORT
 *     FROM
 *     AS
 *     STRING
 *
 * No compatibility aliases are created inside this grammar.
 *
 * Compatibility migrations belong to:
 *
 *     grammar/compatibility/
 *
 * Token-number assumptions are prohibited.
 *
 * ============================================================================
 * WHY REDUNDANT WRAPPERS ARE AVOIDED
 * ============================================================================
 *
 * The previous implementation contained many generic wrappers such as:
 *
 *     importTarget
 *     importTargetList
 *     importAliasList
 *     importSourceList
 *     optionalImportAlias
 *     optionalImportSource
 *     optionalImportSourceReference
 *     namedImportSpecifierList
 *
 * merely to expose variants of already-existing productions.
 *
 * Such wrappers are not independently meaningful language constructs and make
 * ownership harder to audit.
 *
 * This production version keeps only wrappers that have a concrete integration
 * purpose:
 *
 *     importDeclaration
 *     importClause
 *     importSpecifierList
 *     importSpecifier
 *     importSource
 *     importSourceReference
 *     importSection
 *
 * The grammar therefore has a smaller and more stable public surface.
 *
 * ============================================================================
 * SOURCE ORDERING
 * ============================================================================
 *
 * This file does NOT impose a universal rule that all imports must appear
 * before all declarations.
 *
 * Whether a source unit/module requires:
 *
 *     imports first
 *
 * or permits imports to be interleaved with declarations is owned by the
 * canonical source/module composition grammar and language specification.
 *
 * `importSection` exists for contexts that explicitly require an import
 * section.
 *
 * ============================================================================
 * TRAILING COMMA POLICY
 * ============================================================================
 *
 * Named import lists permit a trailing comma:
 *
 *     import {Vector, Matrix,};
 *
 * This is intentionally supported for formatter stability and generated-source
 * friendliness.
 *
 * A list MUST contain at least one import specifier.
 *
 * ============================================================================
 * WILDCARD POLICY
 * ============================================================================
 *
 * Wildcard imports are syntactically valid:
 *
 *     import *;
 *
 *     import * as math;
 *
 *     import * from "math";
 *
 *     import * as math from "math";
 *
 * The grammar does not decide what a wildcard imports.
 *
 * Semantic analysis determines:
 *
 *     - exported-name visibility;
 *     - collision behavior;
 *     - namespace population;
 *     - ambiguity;
 *     - tooling behavior.
 *
 * ============================================================================
 * ALIAS POLICY
 * ============================================================================
 *
 * An alias is syntactically:
 *
 *     AS identifier
 *
 * The alias is always a canonical identifier.
 *
 * This grammar does not allow:
 *
 *     as qualified::alias
 *
 * because aliases bind a local name rather than a qualified path.
 *
 * ============================================================================
 * SOURCE POLICY
 * ============================================================================
 *
 * Source qualifiers have the following syntax:
 *
 *     FROM qualifiedName
 *
 * or:
 *
 *     FROM STRING
 *
 * They cannot be empty.
 *
 * The grammar does not interpret the contents of STRING.
 *
 * In particular, this grammar does not classify:
 *
 *     "foo"
 *     "./foo"
 *     "../foo"
 *     "https://example.org/foo"
 *     "registry:foo"
 *
 * as filesystem, URL, registry, or network forms.
 *
 * Such classification belongs to source-provider/toolchain semantics.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax tests MUST include:
 *
 *     import math;
 *     import math::linear;
 *     import math::linear::matrix;
 *     import math::linear as linear;
 *
 *     import {Vector};
 *     import {Vector, Matrix};
 *     import {Vector as V, Matrix};
 *     import {Vector, Matrix,};
 *
 *     import *;
 *     import * as math;
 *
 *     import {Vector} from "math";
 *     import {Vector as V, Matrix} from "math";
 *     import * from "math";
 *     import * as math from "math";
 *
 *     import math::linear from "math";
 *     import math::linear as linear from "math";
 *
 *     import {Vector} from math::linear;
 *     import * from math::linear;
 *
 * Cross-domain examples MUST include:
 *
 *     import quantum::algorithms;
 *     import classical::linear;
 *     import hardware::capability;
 *     import hdl::components;
 *     import distributed::services;
 *     import ai::models;
 *     import networking::protocols;
 *
 * These examples are syntactically identical regardless of target hardware.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser MUST reject:
 *
 *     import;
 *     import ;
 *     import {};
 *     import { };
 *     import math::;
 *     import ::math;
 *     import math::::linear;
 *     import math::linear as;
 *     import math::linear from;
 *     import math::linear from "";
 *     import * as;
 *     import * from;
 *     import {Vector as};
 *     import {Vector,,Matrix};
 *     import {,Vector};
 *     import {Vector Matrix};
 *     import math::linear extra;
 *     import math::linear from "source" extra;
 *
 * Whether an empty STRING is lexically valid is a lexical concern; the module
 * semantic layer may separately reject an empty source identifier. The syntax
 * grammar itself should not create a special empty-string rule.
 *
 * Therefore the `from ""` example is a semantic validation case if the lexer
 * accepts an empty STRING.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     - one-character names;
 *     - long identifiers;
 *     - deeply qualified names;
 *     - many imported symbols;
 *     - many import declarations;
 *     - aliases;
 *     - trailing commas;
 *     - symbolic source references;
 *     - literal source references;
 *     - wildcard imports;
 *     - cross-domain imports;
 *     - nested module contexts.
 *
 * The grammar MUST NOT establish artificial finite bounds for these cases.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The following must be generated/tested parametrically:
 *
 *     import a::b::c::...;
 *
 *     import {a, b, c, ...};
 *
 *     import module::symbol as local;
 *
 *     import * from source;
 *
 * Test size may be bounded by the available test environment.
 *
 * Such a test bound MUST NOT become a language grammar limit.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical token streams MUST produce identical parse structures.
 *
 * Import parsing MUST NOT depend on:
 *
 *     CPU count;
 *     GPU availability;
 *     QPU availability;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     wall-clock time;
 *     randomness;
 *     resource availability.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_IMPORTS
 *     MAX_IMPORT_DEPTH
 *     MAX_SYMBOLS
 *     MAX_MODULES
 *     MAX_PACKAGES
 *     MAX_DEPENDENCIES
 *     MAX_TARGETS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * Numeric source values, where permitted elsewhere in Zamani, remain program
 * semantics and are not interpreted here.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It is the single grammar owner of import declarations.
 *     [x] It uses the repository's actual IMPORT token.
 *     [x] It uses the repository's actual FROM token.
 *     [x] It uses the repository's actual AS token.
 *     [x] It uses the repository's actual STRING token.
 *     [x] It reuses canonical identifier syntax.
 *     [x] It reuses canonical qualified-name syntax.
 *     [x] It supports direct qualified imports.
 *     [x] It supports named imports.
 *     [x] It supports wildcard imports.
 *     [x] It supports aliases.
 *     [x] It supports symbolic source references.
 *     [x] It supports literal source references.
 *     [x] It supports trailing commas in named imports.
 *     [x] It has no finite import-count limit.
 *     [x] It has no finite qualification-depth limit.
 *     [x] It has no hardware limits.
 *     [x] It has no quantum limits.
 *     [x] It performs no semantic resolution.
 *     [x] It performs no I/O.
 *     [x] It creates no IR.
 *     [x] It creates no quantum-specific IR.
 *     [x] It contains no embedded Rust.
 *     [x] It requires no unsafe Rust.
 *     [x] It preserves source-level symbolic/literal source distinction.
 *     [x] It integrates with modules.g4.
 *     [x] It integrates with ZamaniParser.g4.
 *     [x] It remains compatible with classical, quantum, HDL, hardware,
 *         distributed, AI, networking, security and future domains.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * An import declaration says:
 *
 *     "this source program depends on or references a source-level entity."
 *
 * It does NOT say:
 *
 *     "use this particular machine."
 *
 * It does NOT say:
 *
 *     "use this particular hardware topology."
 *
 * It does NOT say:
 *
 *     "allocate this many resources."
 *
 * It does NOT say:
 *
 *     "select this backend."
 *
 * Therefore imports remain compatible with:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     simulators
 *     heterogeneous systems
 *     clusters
 *     HPC systems
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * The source-level contract remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Anywhere
 *          ->
 *     Forever
 *
 * subject to semantic validity, target capabilities, available resources, and
 * explicitly defined compatibility policy.
 *
 * ============================================================================
 */

parser grammar Imports;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Names is the canonical owner of:
 *
 *     identifier
 *     qualifiedName
 *     nameSegment
 *
 * It uses the repository's shared lexical vocabulary and must not be replaced
 * by an import-specific name grammar.
 */
import Names;


/*
 * ============================================================================
 * PUBLIC IMPORT DECLARATION
 * ============================================================================
 *
 * Every ordinary import is terminated by SEMICOLON.
 *
 * Examples:
 *
 *     import math;
 *     import math::linear;
 *     import {Vector, Matrix};
 *     import *;
 */
importDeclaration
    : IMPORT importClause SEMICOLON
    ;


/*
 * ============================================================================
 * IMPORT CLAUSE
 * ============================================================================
 *
 * Exactly one target form is selected.
 */
importClause
    : qualifiedImportClause
    | namedImportClause
    | wildcardImportClause
    ;


/*
 * ============================================================================
 * QUALIFIED IMPORT
 * ============================================================================
 *
 * Examples:
 *
 *     import math;
 *     import math::linear;
 *     import math::linear as linear;
 *     import math::linear from "math";
 *     import math::linear as linear from "math";
 *     import math::linear from math::source;
 */
qualifiedImportClause
    : qualifiedName importAlias? importSource?
    ;


/*
 * ============================================================================
 * NAMED IMPORT
 * ============================================================================
 *
 * Examples:
 *
 *     import {Vector};
 *     import {Vector, Matrix};
 *     import {Vector as V, Matrix};
 *     import {Vector, Matrix,};
 *
 * The list is non-empty.
 *
 * A trailing comma is accepted.
 */
namedImportClause
    : LBRACE importSpecifierList RBRACE importSource?
    ;


/*
 * ============================================================================
 * NAMED IMPORT SPECIFIER LIST
 * ============================================================================
 */
importSpecifierList
    : importSpecifier (COMMA importSpecifier)* COMMA?
    ;


/*
 * ============================================================================
 * NAMED IMPORT SPECIFIER
 * ============================================================================
 *
 * The imported symbol is a canonical qualified name.
 *
 * The local binding is an optional canonical identifier alias.
 *
 * Examples:
 *
 *     Vector
 *     math::Vector
 *     math::Vector as Vector
 *     math::Vector as V
 */
importSpecifier
    : qualifiedName importAlias?
    ;


/*
 * ============================================================================
 * WILDCARD IMPORT
 * ============================================================================
 *
 * Examples:
 *
 *     import *;
 *     import * as math;
 *     import * from "math";
 *     import * as math from "math";
 *     import * from math::source;
 */
wildcardImportClause
    : STAR importAlias? importSource?
    ;


/*
 * ============================================================================
 * IMPORT ALIAS
 * ============================================================================
 *
 * Alias is deliberately restricted to a single identifier.
 *
 * An alias is a local binding, not a qualified path.
 */
importAlias
    : AS identifier
    ;


/*
 * ============================================================================
 * IMPORT SOURCE
 * ============================================================================
 *
 * Source qualification is either:
 *
 *     FROM qualifiedName
 *
 * or:
 *
 *     FROM STRING
 *
 * The grammar preserves which form was used.
 */
importSource
    : FROM importSourceReference
    ;


/*
 * ============================================================================
 * IMPORT SOURCE REFERENCE
 * ============================================================================
 *
 * A symbolic source remains a canonical qualified name.
 *
 * A literal source remains the canonical STRING token.
 *
 * Neither is interpreted here.
 */
importSourceReference
    : qualifiedName
    | STRING
    ;


/*
 * ============================================================================
 * IMPORT SECTION
 * ============================================================================
 *
 * This rule is a reusable section-level boundary.
 *
 * It does not impose source-order semantics on the whole language.
 *
 * A module/source grammar may choose to require imports before declarations
 * while another explicitly standardized context may permit interleaving.
 */
importSection
    : importDeclaration*
    ;