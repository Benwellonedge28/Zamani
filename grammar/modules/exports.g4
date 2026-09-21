/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/modules/exports.g4
* 
* Grammar:
* Exports
* 
* Status:
* CANONICAL MODULE-EXPORT PARSER COMPONENT
* 
* Purpose:
* Define the complete source-level syntax for exporting declarations,
* symbols, aliases, wildcard exports, and re-exports while delegating
* lexical vocabulary, names, semantic resolution, visibility, package
* resolution, dependency resolution, and target realization to their
* canonical owners.
* 
* Language:
* Zamani
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Rust baseline:
* Rust 1.97 / Rust 1.97.1
* 
* Safety:
* This file contains grammar only.
* It contains no Rust actions, semantic predicates, filesystem access,
* network access, runtime callbacks, or unsafe code.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* source
*   |
*   v
* grammar/antlr/ZamaniLexer.g4
*   |
*   v
* grammar/antlr/ZamaniParser.g4
*   |
*   v
* Modules / Exports
*   |
*   v
* domain-neutral frontend AST
*   |
*   v
* module + symbol + visibility analysis
*   |
*   v
* semantic model
*   |
*   +---------------------+----------------------+
*   |                     |                      |
*   v                     v                      v
* classical             quantum::ir          HDL/hardware
*   |                     |                      |
*   +---------------------+----------------------+
*                         |
*                         v
*              optimization / lowering
*                         |
*                routing / scheduling
*                         |
*                 QEC / resilience / ZQN
*                         |
*                        HAL
*                         |
*                         v
*                target realization
* 
* This file is a SOURCE-SYNTAX component only.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - export declarations;
* - export targets;
* - direct symbol exports;
* - qualified symbol exports;
* - named export lists;
* - export aliases;
* - wildcard exports;
* - namespace-style wildcard aliases;
* - source-qualified exports;
* - re-export declarations;
* - export-specific syntactic wrappers.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical tokens;
* - identifiers;
* - qualified-name construction;
* - filesystem paths;
* - package coordinates;
* - module resolution;
* - symbol resolution;
* - visibility rules;
* - dependency solving;
* - package fetching;
* - registry access;
* - trust verification;
* - ABI validation;
* - capability validation;
* - resource validation;
* - hardware discovery;
* - target selection;
* - classical IR;
* - quantum::ir;
* - HDL/hardware IR;
* - QEC;
* - ZQN;
* - routing;
* - scheduling;
* - optimization;
* - runtime execution.
* 
* ============================================================================
* AUTHORITY
* ============================================================================
* 
* Normative language specification:
* 
* grammar/specification/syntax.md
* grammar/specification/semantics.md
* grammar/spec/modules.md
* 
* Grammar architecture:
* 
* grammar/DESIGN.md
* 
* Canonical lexer:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Canonical parser composition:
* 
* grammar/antlr/ZamaniParser.g4
* 
* Canonical lexical composition:
* 
* grammar/lexer/tokens.g4
* 
* Canonical source-level names:
* 
* grammar/core/names.g4
* 
* Qualified-name integration boundary:
* 
* grammar/core/qualified-names.g4
* 
* Module declaration:
* 
* grammar/modules/modules.g4
* 
* Imports:
* 
* grammar/modules/imports.g4
* 
* Packages:
* 
* grammar/modules/packages.g4
* 
* Dependencies:
* 
* grammar/modules/dependencies.g4
* 
* Namespaces:
* 
* grammar/modules/namespaces.g4
* 
* Visibility:
* 
* grammar/modules/visibility.g4
* 
* This file MUST NOT become a competing module/export authority.
* 
* ============================================================================
* LEXER INTEGRATION
* ============================================================================
* 
* The canonical production lexer is:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Therefore this parser grammar consumes:
* 
* tokenVocab = ZamaniLexer;
* 
* It MUST NOT consume the intermediate lexical composition grammar directly.
* 
* Actual canonical token names include:
* 
* EXPORT
* FROM
* AS
* IDENTIFIER
* STRING
* STAR
* DOUBLE_COLON
* LBRACE
* RBRACE
* COMMA
* SEMICOLON
* 
* This file MUST NOT define lexer rules.
* 
* ============================================================================
* NAME INTEGRATION
* ============================================================================
* 
* Qualified-name syntax is owned outside this file.
* 
* The canonical integration boundary is:
* 
* QualifiedNames
* 
* which ultimately delegates qualified-name construction to:
* 
* Names
* 
* Consequently this file MUST NOT define:
* 
* IDENTIFIER (DOUBLE_COLON IDENTIFIER)*
* 
* or an equivalent local qualified-name grammar.
* 
* This prevents:
* 
* grammar drift;
* duplicate definitions;
* incompatible name semantics;
* competing path-depth behavior;
* different source-span structures.
* 
* ============================================================================
* IMPORTANT REPOSITORY INTEGRATION NOTE
* ============================================================================
* 
* The repository currently contains stale K_* token references in some older
* parser components, including the canonical name grammar.
* 
* The canonical lexer defines:
* 
* AS
* 
* rather than:
* 
* K_AS
* 
* This file intentionally uses the actual canonical lexer token:
* 
* AS
* 
* The stale K_* references must be corrected in their owning files as a
* separate repository-wide compatibility/conformance task.
* 
* This file MUST NOT reintroduce K_* aliases merely to hide that discrepancy.
* 
* ============================================================================
* SOURCE-LEVEL EXPORT MODEL
* ============================================================================
* 
* Zamani exports four principal forms:
* 
* 1. Direct export:
* 
*   export foo;
*   export foo::bar;
* 
* 2. Direct export with alias:
* 
*   export foo as publicFoo;
*   export foo::bar as publicBar;
* 
* 3. Named export list:
* 
*   export {foo, bar};
*   export {foo as publicFoo, bar};
* 
* 4. Re-export:
* 
*   export foo from "library";
*   export {foo, bar} from "library";
*   export * from "library";
* 
* Namespace-style wildcard re-export is also supported:
* 
*   export * as namespace from "library";
* 
* This is source syntax only.
* 
* ============================================================================
* EXPORT VERSUS RE-EXPORT
* ============================================================================
* 
* The grammar preserves the difference between:
* 
* export foo;
* 
* and:
* 
* export foo from "source";
* 
* The first exports a declaration/reference from the current module context.
* 
* The second establishes a source-qualified re-export.
* 
* The parser does NOT resolve either target.
* 
* ============================================================================
* SOURCE REFERENCE MODEL
* ============================================================================
* 
* A re-export source may be:
* 
* qualifiedName
* 
* or:
* 
* STRING
* 
* Examples:
* 
* export foo from math::linear;
* 
* export foo from "math";
* 
* export * from quantum::algorithms;
* 
* export * from "quantum-library";
* 
* The grammar preserves the syntactic distinction.
* 
* Semantic/toolchain layers determine whether the source denotes:
* 
* module;
* package;
* workspace source;
* embedded module;
* generated source;
* registry coordinate;
* remote source;
* another supported source provider.
* 
* This grammar never interprets a string as a filesystem path.
* 
* ============================================================================
* WILDCARD MODEL
* ============================================================================
* 
* A wildcard export:
* 
* export *;
* 
* preserves wildcard intent.
* 
* A wildcard re-export:
* 
* export * from "source";
* 
* preserves wildcard re-export intent.
* 
* A namespace-style wildcard re-export:
* 
* export * as namespace from "source";
* 
* preserves:
* 
* wildcard;
* namespace alias;
* source;
* 
* as separate syntactic components.
* 
* The grammar MUST NOT expand wildcard exports.
* 
* Expansion belongs to module/symbol semantic resolution.
* 
* ============================================================================
* ALIAS MODEL
* ============================================================================
* 
* Alias syntax is:
* 
* AS IDENTIFIER
* 
* Examples:
* 
* export foo as bar;
* 
* export foo::bar as baz;
* 
* export {foo as publicFoo};
* 
* export * as namespace from "library";
* 
* Alias legality, collision detection, visibility, and namespace semantics
* belong downstream.
* 
* ============================================================================
* SEMANTIC BOUNDARY
* ============================================================================
* 
* Parsing MUST NOT determine:
* 
* - whether the exported declaration exists;
* - whether the declaration is visible;
* - whether an alias is legal;
* - whether two aliases collide;
* - whether wildcard expansion produces conflicts;
* - whether a re-export source exists;
* - whether a source is trusted;
* - whether a package version is compatible;
* - whether an exported declaration is ABI compatible;
* - whether the export is usable on a target;
* - whether a capability is available;
* - whether a resource is available.
* 
* These belong to semantic/module/package/toolchain layers.
* 
* ============================================================================
* VISIBILITY INTEGRATION
* ============================================================================
* 
* "visibility.g4" owns visibility syntax.
* 
* Export syntax does not redefine:
* 
* public
* private
* protected
* internal
* pub
* 
* Exporting and visibility are related but distinct concepts.
* 
* For example:
* 
* pub fn compute() {}
* 
* export compute;
* 
* may be semantically meaningful depending on the language specification,
* but this grammar does not decide whether the combination is legal.
* 
* ============================================================================
* MODULE INTEGRATION
* ============================================================================
* 
* "grammar/modules/modules.g4" owns module declarations.
* 
* This file owns export declarations.
* 
* "modules.g4" MUST NOT duplicate:
* 
* exportDeclaration
* exportTarget
* exportSpecifier
* exportSource
* 
* A module body should expose exports through the aggregate parser's "item"
* dispatch.
* 
* Conceptually:
* 
* moduleBody
*     -> item*
* 
* and:
* 
* item
*     -> exportDeclaration
* 
* This keeps the export grammar independently maintainable.
* 
* ============================================================================
* IMPORT INTEGRATION
* ============================================================================
* 
* "grammar/modules/imports.g4" owns imports.
* 
* Import and export syntax intentionally remain separate.
* 
* In particular:
* 
* import foo from "source";
* 
* is an import construct.
* 
* export foo from "source";
* 
* is a re-export construct.
* 
* The source-reference syntax may be structurally similar, but ownership
* remains separate to keep AST and semantic boundaries explicit.
* 
* ============================================================================
* PACKAGE INTEGRATION
* ============================================================================
* 
* "grammar/modules/packages.g4" owns package declaration syntax.
* 
* This file does not define package declarations.
* 
* A source such as:
* 
* "quantum-library"
* 
* remains opaque source syntax until package/module resolution.
* 
* ============================================================================
* DEPENDENCY INTEGRATION
* ============================================================================
* 
* "grammar/modules/dependencies.g4" owns dependency declarations.
* 
* An export source can refer to a module/package, but an export does not itself
* declare a dependency policy.
* 
* Dependency graph construction belongs downstream.
* 
* ============================================================================
* NAMESPACE INTEGRATION
* ============================================================================
* 
* A qualified name such as:
* 
* quantum::algorithms::search
* 
* is syntactically just a qualified source-level name.
* 
* It does not imply:
* 
* quantum hardware;
* a QPU;
* physical qubits;
* a filesystem directory;
* a network endpoint;
* a deployment topology.
* 
* Semantic resolution determines what the name denotes.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The frontend AST MUST preserve enough information to distinguish:
* 
* direct export;
* named export list;
* wildcard export;
* namespace wildcard export;
* re-export;
* export alias;
* source reference;
* source kind;
* source spans.
* 
* Recommended semantic-neutral conceptual shape:
* 
* ExportDeclaration {
*     target
*     source?
*     span
* }
* 
* Target variants:
* 
* Path {
*     path
*     alias?
* }
* 
* Named {
*     specifiers[]
* }
* 
* Wildcard {
*     alias?
* }
* 
* Specifier:
* 
* ExportSpecifier {
*     path
*     alias?
*     span
* }
* 
* Source:
* 
* SymbolicSource {
*     qualifiedName
* }
* 
* LiteralSource {
*     literal
* }
* 
* The exact Rust type names belong to:
* 
* src/frontend/ast/
* 
* This grammar MUST NOT introduce another AST.
* 
* ============================================================================
* SOURCE-SPAN CONTRACT
* ============================================================================
* 
* The parser/AST integration MUST preserve source locations for:
* 
* export keyword;
* exported path;
* alias;
* wildcard;
* source keyword;
* source reference;
* complete export declaration.
* 
* This is required for:
* 
* diagnostics;
* IDE tooling;
* formatting;
* refactoring;
* source maps;
* provenance;
* documentation;
* compatibility tooling.
* 
* ============================================================================
* SEMANTIC PIPELINE
* ============================================================================
* 
* The complete downstream flow is:
* 
* export syntax
*      |
*      v
* domain-neutral AST
*      |
*      v
* module/name resolution
*      |
*      v
* visibility validation
*      |
*      v
* package/dependency resolution
*      |
*      v
* semantic model
*      |
*      +----------------------+---------------------+
*      |                      |                     |
*      v                      v                     v
* classical semantics   quantum semantics     HDL/hardware
*                              |
*                              v
*                         quantum::ir
*                              |
*                              v
*                   optimization / lowering
*                              |
*                    routing / scheduling
*                              |
*                     QEC / resilience / ZQN
*                              |
*                             HAL
*                              |
*                              v
*                     target realization
* 
* Exports themselves do not lower directly to quantum::ir, classical IR,
* HDL IR, or hardware IR.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* The grammar remains completely domain-neutral.
* 
* These are all ordinary qualified names:
* 
* quantum::operations
* quantum::measurement
* quantum::algorithms
* quantum::custom_gate
* 
* An export does not:
* 
* - select a QPU;
* - allocate qubits;
* - select physical qubits;
* - select a native gate set;
* - perform decomposition;
* - perform routing;
* - perform scheduling;
* - perform QEC;
* - perform ZQN analysis;
* - select calibration;
* - construct quantum::ir.
* 
* ============================================================================
* CLASSICAL / HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* The same export grammar applies to:
* 
* classical::
* quantum::
* hybrid::
* hdl::
* hardware::
* distributed::
* ai::
* data::
* networking::
* security::
* interoperability::
* dialect::
* 
* No domain-specific export rule is required.
* 
* This means new computational domains can be introduced without changing
* export syntax merely because a new namespace is added.
* 
* ============================================================================
* POCO-REAF / SCALABILITY CONTRACT
* ============================================================================
* 
* There are NO language-level limits here for:
* 
* - number of export declarations;
* - number of export specifiers;
* - number of aliases;
* - number of qualified-name segments;
* - number of modules;
* - number of packages;
* - number of dependencies;
* - number of domains;
* - number of targets;
* - number of devices;
* - number of CPUs;
* - number of cores;
* - number of threads;
* - number of GPUs;
* - number of FPGAs;
* - number of ASICs;
* - number of QPUs;
* - number of qubits;
* - amount of memory;
* - number of nodes;
* - number of accelerators;
* - topology size.
* 
* This grammar MUST NOT contain:
* 
* MAX_EXPORTS
* MAX_EXPORT_SPECIFIERS
* MAX_EXPORT_DEPTH
* MAX_ALIAS_COUNT
* MAX_MODULES
* MAX_PACKAGES
* MAX_DEPENDENCIES
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* MAX_DEVICES
* 
* ANTLR repetition operators express unbounded language cardinality:
* 
* *
* +
* 
* Actual compiler/parser resource exhaustion is an implementation concern.
* 
* Such operational budgets MUST NOT be converted into source-language
* restrictions.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* Forbidden in this file:
* 
* - hardware IDs;
* - device IDs;
* - physical addresses;
* - physical qubit IDs;
* - fixed topology;
* - fixed accelerator counts;
* - fixed memory sizes;
* - fixed CPU/core/thread counts;
* - fixed tensor dimensions;
* - vendor-specific export syntax;
* - backend-specific export syntax;
* - filesystem assumptions;
* - network assumptions.
* 
* Domain names remain ordinary qualified names.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* This grammar contains:
* 
* - no actions;
* - no semantic predicates;
* - no filesystem access;
* - no network access;
* - no package lookup;
* - no registry lookup;
* - no environment lookup;
* - no hardware discovery;
* - no randomness;
* - no clock access;
* - no runtime callbacks.
* 
* For the same token stream and grammar version, syntactic interpretation is
* deterministic.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Export source references are untrusted source input.
* 
* Parsing MUST NOT:
* 
* - open a file;
* - fetch a URL;
* - contact a registry;
* - execute commands;
* - resolve credentials;
* - access environment variables;
* - verify signatures;
* - install dependencies.
* 
* Those responsibilities belong to explicitly authorized downstream tooling.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing source forms preserved by this grammar:
* 
* export foo;
* export foo::bar;
* export foo as bar;
* export foo::bar as bar;
* export {foo, bar};
* export {foo as publicFoo, bar};
* export *;
* export * from "source";
* export {foo, bar} from "source";
* export foo::bar from "source";
* 
* Additional production form:
* 
* export * as namespace from "source";
* 
* This form preserves namespace-style wildcard re-export semantics without
* introducing a second module/export language.
* 
* Compatibility policy:
* 
* - Existing valid syntax remains valid.
* - Token renames are handled by lexer compatibility policy.
* - Semantic changes are handled by semantic-version policy.
* - Deprecated forms must be diagnosed by compatibility tooling rather
*   than silently changing their meaning.
* 
* ============================================================================
* GRAMMAR COMPOSITION
* ============================================================================
* 
* This grammar imports:
* 
* QualifiedNames
* 
* and consumes:
* 
* qualifiedNameReference
* 
* for exported source-level names.
* 
* It therefore does NOT define its own:
* 
* identifier;
* qualifiedName;
* nameSegment;
* nameAlias.
* 
* "AS" is consumed directly here because alias ownership is export-context
* syntax, while the actual identifier remains owned by the canonical name
* grammar.
* 
* ============================================================================
* PRODUCTION FORMS
* ============================================================================
* 
* Direct export:
* 
* export foo;
* 
* Direct qualified export:
* 
* export foo::bar;
* 
* Direct aliased export:
* 
* export foo as bar;
* 
* Named export list:
* 
* export {foo, bar};
* 
* Named export list with aliases:
* 
* export {
*     foo as publicFoo,
*     bar
* };
* 
* Wildcard:
* 
* export *;
* 
* Wildcard re-export:
* 
* export * from "library";
* 
* Namespace wildcard re-export:
* 
* export * as library from "library";
* 
* Named re-export:
* 
* export {foo, bar} from "library";
* 
* Qualified re-export:
* 
* export foo::bar from "library";
* 
* Symbolic source:
* 
* export foo from library::core;
* 
* ============================================================================
* GRAMMAR RULES
* ============================================================================
  */

parser grammar Exports;

options {
tokenVocab = ZamaniLexer;
}

import QualifiedNames;

/*

* ============================================================================
* 1. EXPORT DECLARATION
* ============================================================================
* 
* Complete source-level export declaration.
* 
* A semicolon is required at this grammar boundary.
* 
* This gives module-item parsing an unambiguous declaration terminator and
* avoids making export parsing dependent on newline/trivia behavior.
  */
  exportDeclaration
  : EXPORT
  exportTarget
  exportSourceClause?
  SEMICOLON
  ;

/*

* ============================================================================
* 2. EXPORT TARGET
* ============================================================================
* 
* The target determines what is exported.
* 
* It does not determine semantic visibility or resolution.
  */
  exportTarget
  : exportWildcardTarget
  | exportNamedTarget
  | exportPathTarget
  ;

/*

* ============================================================================
* 3. WILDCARD EXPORT
* ============================================================================
* 
* Forms:
* 
* export *;
* 
* export * from "source";
* 
* export * as namespace from "source";
* 
* The optional alias is syntactic data.
  */
  exportWildcardTarget
  : STAR
  exportWildcardAlias?
  ;

/*

* ============================================================================
* 4. WILDCARD ALIAS
* ============================================================================
* 
* Form:
* 
* export * as namespace ...
* 
* The alias is deliberately restricted to a canonical identifier rather than
* a qualified name because it names the local exported namespace.
  */
  exportWildcardAlias
  : AS
  IDENTIFIER
  ;

/*

* ============================================================================
* 5. PATH EXPORT
* ============================================================================
* 
* Forms:
* 
* export foo;
* export foo::bar;
* export foo as publicFoo;
* export foo::bar as publicBar;
* 
* The path itself is owned by QualifiedNames.
  */
  exportPathTarget
  : qualifiedNameReference
  exportTargetAlias?
  ;

/*

* ============================================================================
* 6. TARGET ALIAS
* ============================================================================
* 
* Context-specific alias wrapper.
* 
* The exported name remains owned by the canonical identifier/name grammar.
  */
  exportTargetAlias
  : AS
  IDENTIFIER
  ;

/*

* ============================================================================
* 7. NAMED EXPORT GROUP
* ============================================================================
* 
* Forms:
* 
* export {foo};
* export {foo, bar};
* export {foo as publicFoo, bar};
* 
* An empty export group is rejected intentionally.
* 
* Empty braces have no useful export semantics and would introduce a
* redundant construct that complicates semantic validation.
  */
  exportNamedTarget
  : LBRACE
  exportSpecifierList
  RBRACE
  ;

/*

* ============================================================================
* 8. EXPORT SPECIFIER LIST
* ============================================================================
* 
* Arbitrarily many specifiers subject only to implementation resources.
* 
* A trailing comma is accepted for formatter-friendly and generated source.
  /
  exportSpecifierList
  : exportSpecifier
  (COMMA exportSpecifier)
  COMMA?
  ;

/*

* ============================================================================
* 9. EXPORT SPECIFIER
* ============================================================================
* 
* Forms:
* 
* foo
* foo as bar
* foo::bar
* foo::bar as baz
* 
* Qualified-name syntax remains owned by QualifiedNames.
  */
  exportSpecifier
  : qualifiedNameReference
  exportSpecifierAlias?
  ;

/*

* ============================================================================
* 10. EXPORT SPECIFIER ALIAS
* ============================================================================
  */
  exportSpecifierAlias
  : AS
  IDENTIFIER
  ;

/*

* ============================================================================
* 11. EXPORT SOURCE CLAUSE
* ============================================================================
* 
* Forms:
* 
* from "source"
* from module::source
* 
* The source is not resolved here.
  */
  exportSourceClause
  : FROM
  exportSourceReference
  ;

/*

* ============================================================================
* 12. EXPORT SOURCE REFERENCE
* ============================================================================
* 
* Two syntactic source classes are preserved:
* 
* qualified symbolic source
* 
* and:
* 
* literal source
* 
* This is intentionally parallel to the module import source model.
  */
  exportSourceReference
  : qualifiedNameReference
  | STRING
  ;

/*

* ============================================================================
* 13. COMPLETE EXPORT TARGET
* ============================================================================
* 
* Stable wrapper for tooling that needs to inspect the target independently
* from the optional source clause.
  */
  exportTargetReference
  : exportTarget
  ;

/*

* ============================================================================
* 14. COMPLETE EXPORT SOURCE
* ============================================================================
* 
* Stable wrapper for tooling and compatibility layers.
  */
  exportSourceReferenceClause
  : exportSourceClause
  ;

/*

* ============================================================================
* 15. NAMED EXPORT ENTRY
* ============================================================================
* 
* Explicit wrapper for tooling that needs to inspect individual named entries.
  */
  exportNamedEntry
  : exportSpecifier
  ;

/*

* ============================================================================
* 16. NAMED EXPORT ENTRY LIST
* ============================================================================
  */
  exportNamedEntryList
  : exportSpecifierList
  ;

/*

* ============================================================================
* 17. SEMANTICALLY NEUTRAL RE-EXPORT WRAPPER
* ============================================================================
* 
* A re-export is an export declaration carrying a source clause.
* 
* This wrapper does not perform source resolution.
  */
  exportReExportDeclaration
  : EXPORT
  exportTarget
  exportSourceClause
  SEMICOLON
  ;

/*

* ============================================================================
* 18. LOCAL EXPORT WRAPPER
* ============================================================================
* 
* A local export is an export declaration without a source clause.
  */
  exportLocalDeclaration
  : EXPORT
  exportTarget
  SEMICOLON
  ;

/*

* ============================================================================
* 19. AST / SEMANTIC INTEGRATION WRAPPERS
* ============================================================================
* 
* These rules deliberately preserve syntax rather than introduce separate
* semantic grammars.
* 
* The frontend may use:
* 
* exportDeclaration
* 
* as the canonical AST entry point.
* 
* The additional wrappers are compatibility/tooling boundaries only.
  */
  exportClause
  : exportTarget
  exportSourceClause?
  ;

/*

* ============================================================================
* 20. ROUND-TRIP INVARIANT
* ============================================================================
* 
* A parser/formatter/printer pipeline should preserve at least:
* 
* export kind;
* exported qualified-name segments;
* alias;
* wildcard;
* wildcard alias;
* source kind;
* source spelling;
* source span.
* 
* Formatting may normalize whitespace and line breaks but MUST NOT silently
* change the semantic category.
* 
* ============================================================================
* 21. POSITIVE CONFORMANCE EXAMPLES
* ============================================================================
* 
* export foo;
* 
* export foo::bar;
* 
* export foo as publicFoo;
* 
* export foo::bar as publicBar;
* 
* export {foo};
* 
* export {foo, bar};
* 
* export {foo as publicFoo, bar};
* 
* export {
* classical::math::Vector,
* quantum::algorithms::search as search,
* hardware::accelerator::compute as compute,
* };
* 
* export *;
* 
* export * from "library";
* 
* export * from library::core;
* 
* export * as library from "library";
* 
* export * as quantum from quantum::library;
* 
* export {foo, bar} from "library";
* 
* export {foo as publicFoo, bar} from library::core;
* 
* export foo::bar from "library";
* 
* export quantum::operations::custom from quantum::library;
* 
* ============================================================================
* 22. NEGATIVE CONFORMANCE EXAMPLES
* ============================================================================
* 
* Missing target:
* 
* export;
* 
* Missing alias:
* 
* export foo as;
* 
* Missing alias identifier:
* 
* export foo as 123;
* 
* Empty named export:
* 
* export {};
* 
* Leading comma:
* 
* export {, foo};
* 
* Duplicate comma:
* 
* export {foo,,bar};
* 
* Missing source:
* 
* export foo from;
* 
* Missing source after wildcard:
* 
* export * from;
* 
* Missing namespace alias:
* 
* export * as from "library";
* 
* Missing closing brace:
* 
* export {foo;
* 
* Missing semicolon:
* 
* export foo
* 
* Whether a no-semicolon declaration form is ever accepted must be decided by
* the normative language specification and applied consistently across module
* declarations. This file intentionally uses an explicit terminator.
* 
* ============================================================================
* 23. BOUNDARY CONFORMANCE
* ============================================================================
* 
* The test suite must cover:
* 
* one exported symbol;
* many exported symbols;
* deeply qualified names;
* long aliases;
* large export lists;
* nested module graphs;
* symbolic sources;
* literal sources;
* wildcard exports;
* wildcard aliases;
* mixed local/re-export declarations;
* all supported source domains.
* 
* No test may turn a test-environment size into a language-level maximum.
* 
* ============================================================================
* 24. SCALABILITY CONFORMANCE
* ============================================================================
* 
* The grammar must remain structurally valid as the program scales across:
* 
* atom-scale computation;
* tiny embedded systems;
* single CPUs;
* multicore systems;
* GPUs;
* FPGAs;
* ASICs;
* quantum processors;
* accelerators;
* distributed systems;
* clusters;
* HPC;
* cloud;
* future execution substrates.
* 
* Export syntax itself remains unchanged.
* 
* Only semantic resolution and target realization change.
* 
* ============================================================================
* 25. CROSS-DOMAIN CONFORMANCE
* ============================================================================
* 
* The following are syntactically ordinary names:
* 
* classical::...
* quantum::...
* hybrid::...
* hdl::...
* hardware::...
* distributed::...
* ai::...
* data::...
* networking::...
* security::...
* interoperability::...
* 
* The export grammar MUST NOT need new alternatives when another computational
* domain is added.
* 
* ============================================================================
* 26. RESOURCE / CAPABILITY SEPARATION
* ============================================================================
* 
* Export syntax MUST NOT be interpreted as resource allocation.
* 
* For example:
* 
* export quantum::operation;
* 
* does NOT mean:
* 
* allocate a QPU;
* allocate qubits;
* choose physical qubits;
* select topology;
* reserve memory;
* select a backend.
* 
* Resource and capability semantics belong downstream.
* 
* ============================================================================
* 27. DETERMINISM TESTS
* ============================================================================
* 
* Given identical:
* 
* source;
* token stream;
* language version;
* grammar version;
* 
* the parser MUST produce equivalent parse structure.
* 
* Parsing MUST NOT depend upon:
* 
* CPU count;
* GPU availability;
* QPU availability;
* filesystem state;
* package registry state;
* network state;
* environment variables;
* wall-clock time;
* randomness;
* deployment topology.
* 
* ============================================================================
* 28. SECURITY TESTS
* ============================================================================
* 
* Source strings must be treated as inert syntax.
* 
* The parser MUST NOT:
* 
* open them;
* normalize them into host paths;
* fetch them;
* execute them;
* resolve credentials;
* invoke package managers.
* 
* Security and trust checks belong downstream.
* 
* ============================================================================
* 29. COMPATIBILITY TESTS
* ============================================================================
* 
* Every stable export form must be tested against:
* 
* lexer output;
* parser output;
* AST lowering;
* symbol resolution;
* visibility checking;
* module resolution;
* package/dependency resolution;
* formatter round-trip;
* diagnostics.
* 
* Compatibility tests must also verify that:
* 
* AS
* 
* remains the canonical lexical token rather than silently introducing:
* 
* K_AS.
* 
* ============================================================================
* 30. HARD-CODING AUDIT
* ============================================================================
* 
* This file contains no:
* 
* MAX_EXPORTS;
* MAX_MODULES;
* MAX_PATH_DEPTH;
* MAX_ALIASES;
* MAX_PACKAGES;
* MAX_DEPENDENCIES;
* MAX_TARGETS;
* MAX_QUBITS;
* MAX_CPUS;
* MAX_CORES;
* MAX_THREADS;
* MAX_GPUS;
* MAX_FPGAS;
* MAX_QPUS;
* MAX_NODES;
* MAX_MEMORY;
* MAX_DEVICES.
* 
* It contains no:
* 
* physical device IDs;
* physical qubit IDs;
* hardware addresses;
* vendor backend names;
* fixed topology;
* fixed accelerator count.
* 
* ============================================================================
* 31. RUST INTEGRATION
* ============================================================================
* 
* This grammar contains no Rust.
* 
* Generated and handwritten Rust integration must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* 
* and must use safe Rust only.
* 
* This grammar does not require:
* 
* unsafe;
* FFI;
* raw pointers;
* target-specific compiler extensions.
* 
* ============================================================================
* 32. IR INTEGRATION
* ============================================================================
* 
* Export declarations are frontend/module metadata.
* 
* They do not directly become:
* 
* ClassicalIR;
* quantum::ir;
* HDLIR;
* HardwareIR;
* RuntimeIR.
* 
* Their semantic effects are resolved before domain-specific lowering.
* 
* In particular, this file MUST NOT introduce:
* 
* ExportIR;
* ModuleIR;
* QuantumExportIR;
* HardwareExportIR;
* 
* as competing intermediate representations.
* 
* ============================================================================
* 33. COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] It is a parser grammar.
* 
* [x] It consumes the canonical ZamaniLexer vocabulary.
* 
* [x] It uses actual canonical token names.
* 
* [x] It does not define lexer rules.
* 
* [x] It delegates qualified-name syntax to QualifiedNames.
* 
* [x] It does not duplicate identifier syntax.
* 
* [x] It supports direct exports.
* 
* [x] It supports qualified exports.
* 
* [x] It supports aliases.
* 
* [x] It supports named export lists.
* 
* [x] It supports trailing commas.
* 
* [x] It supports wildcard exports.
* 
* [x] It supports wildcard namespace aliases.
* 
* [x] It supports symbolic re-export sources.
* 
* [x] It supports literal re-export sources.
* 
* [x] It preserves local-export versus re-export structure.
* 
* [x] It performs no semantic resolution.
* 
* [x] It performs no package resolution.
* 
* [x] It performs no filesystem access.
* 
* [x] It performs no network access.
* 
* [x] It performs no hardware discovery.
* 
* [x] It introduces no machine-size limits.
* 
* [x] It introduces no quantum-size limits.
* 
* [x] It introduces no topology limits.
* 
* [x] It introduces no competing IR.
* 
* [x] It remains domain-neutral.
* 
* [x] It supports POCO-REAF.
* 
* [x] It requires no unsafe Rust.
* 
* [x] It is compatible with the Rust 1.97 / 1.97.1 safe-Rust frontend
* integration contract.
* 
* [x] It has explicit integration contracts with Modules, Imports,
* QualifiedNames, Visibility, Packages, Dependencies, the frontend AST,
* semantic analysis, and downstream compilation.
* 
* ============================================================================
* FINAL ARCHITECTURAL RULE
* ============================================================================
* 
* "export" describes which source-level program symbols form a module's
* externally visible interface.
* 
* It does NOT describe where the program runs.
* 
* Therefore:
* 
* export syntax
*     !=
* module resolution
* 
* module resolution
*     !=
* hardware realization
* 
* hardware realization
*     !=
* language syntax
* 
* Zamani remains:
* 
* Program Once
*      ->
* Compile Once
*      ->
* Run Everywhere
*      ->
* Run Anywhere
*      ->
* Run Forever
* 
* subject to the program's semantics, implementation capabilities, and
* resources available at realization time.
* 
* ============================================================================
  */

parser grammar Exports;

options {
tokenVocab = ZamaniLexer;
}

import QualifiedNames;

/*

* ---
* Canonical export declaration
* ---

*/

exportDeclaration
: EXPORT
exportTarget
exportSourceClause?
SEMICOLON
;

/*

* ---
* Export target
* ---

*/

exportTarget
: exportWildcardTarget
| exportNamedTarget
| exportPathTarget
;

/*

* ---
* Wildcard target
* ---

*/

exportWildcardTarget
: STAR
exportWildcardAlias?
;

exportWildcardAlias
: AS IDENTIFIER
;

/*

* ---
* Qualified/direct target
* ---

*/

exportPathTarget
: qualifiedNameReference
exportTargetAlias?
;

exportTargetAlias
: AS IDENTIFIER
;

/*

* ---
* Named target
* ---

*/

exportNamedTarget
: LBRACE
exportSpecifierList
RBRACE
;

exportSpecifierList
: exportSpecifier
(COMMA exportSpecifier)*
COMMA?
;

exportSpecifier
: qualifiedNameReference
exportSpecifierAlias?
;

exportSpecifierAlias
: AS IDENTIFIER
;

/*

* ---
* Re-export source
* ---

*/

exportSourceClause
: FROM
exportSourceReference
;

exportSourceReference
: qualifiedNameReference
| STRING
;

/*

* ---
* Stable integration wrappers
* ---

*/

exportTargetReference
: exportTarget
;

exportSourceReferenceClause
: exportSourceClause
;

exportNamedEntry
: exportSpecifier
;

exportNamedEntryList
: exportSpecifierList
;

exportReExportDeclaration
: EXPORT
exportTarget
exportSourceClause
SEMICOLON
;

exportLocalDeclaration
: EXPORT
exportTarget
SEMICOLON
;

exportClause
: exportTarget
exportSourceClause?
;