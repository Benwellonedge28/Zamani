/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/modules.g4
 *
 * Grammar:
 *     Modules
 *
 * Purpose:
 *     Canonical parser component for Zamani module declarations.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * This grammar owns ONLY module-declaration syntax.
 *
 *     ZamaniTokens
 *          |
 *          v
 *     canonical parser
 *          |
 *          +--> Names / QualifiedNames
 *          +--> Visibility
 *          +--> ModuleAttributes
 *          |
 *          v
 *     Modules
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     module / namespace / package semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware IR
 *          +--> distributed / accelerator IR
 *          |
 *          v
 *     optimization / routing / scheduling / lowering
 *          |
 *          v
 *     runtime / target realization
 *
 * IMPORTANT:
 *
 * This grammar is a syntax layer.
 *
 * It MUST NOT:
 *
 *     - resolve modules;
 *     - access the filesystem;
 *     - access a package registry;
 *     - access the network;
 *     - select hardware;
 *     - select a backend;
 *     - discover resources;
 *     - allocate resources;
 *     - construct IR;
 *     - construct quantum::ir;
 *     - perform optimization;
 *     - perform routing;
 *     - perform scheduling;
 *     - execute code.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - module declaration syntax;
 *     - module declaration headers;
 *     - module names as module-qualified-name wrappers;
 *     - inline module bodies;
 *     - module body boundaries;
 *     - module nesting syntax through canonical names;
 *     - the syntactic composition of:
 *           visibility
 *           attributes
 *           module name
 *           module body
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - visibility vocabulary;
 *     - attributes;
 *     - imports;
 *     - exports;
 *     - packages;
 *     - dependencies;
 *     - namespaces;
 *     - declarations;
 *     - functions;
 *     - types;
 *     - expressions;
 *     - statements;
 *     - classical computation;
 *     - quantum computation;
 *     - quantum gates;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HDL;
 *     - hardware;
 *     - resources;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - runtime execution;
 *     - filesystem resolution;
 *     - package resolution;
 *     - dependency solving;
 *     - symbol tables;
 *     - semantic validation.
 *
 * ============================================================================
 * CANONICAL OWNERSHIP BOUNDARIES
 * ============================================================================
 *
 * Names:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * own canonical name/qualified-name syntax.
 *
 * Visibility:
 *
 *     grammar/modules/visibility.g4
 *
 * owns:
 *
 *     visibilityModifier
 *
 * Module attributes:
 *
 *     grammar/modules/module-attributes.g4
 *
 * own:
 *
 *     moduleAttributes
 *
 * Imports:
 *
 *     grammar/modules/imports.g4
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
 * This file MUST NOT duplicate any of those syntactic vocabularies.
 *
 * ============================================================================
 * CRITICAL DESIGN RULE
 * ============================================================================
 *
 * A module is a SOURCE-LEVEL COMPILATION / ORGANIZATION UNIT.
 *
 * A module is NOT inherently:
 *
 *     - a filesystem directory;
 *     - a package;
 *     - a process;
 *     - a thread;
 *     - a machine;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a quantum register;
 *     - a physical qubit set;
 *     - a network node;
 *     - a runtime instance;
 *     - a deployment unit.
 *
 * For example:
 *
 *     module quantum::algorithms {
 *         ...
 *     }
 *
 * establishes source-level module syntax.
 *
 * It does NOT mean:
 *
 *     - use a quantum processor;
 *     - allocate a fixed number of qubits;
 *     - use a particular topology;
 *     - use a particular backend;
 *     - allocate a particular amount of memory;
 *     - deploy to a particular machine.
 *
 * Those meanings belong to later semantic, compilation, resource, hardware,
 * scheduling, routing, and runtime layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Module syntax is deliberately independent of machine scale.
 *
 * The grammar contains NO finite limits for:
 *
 *     - number of modules;
 *     - module nesting depth;
 *     - module-name segment count;
 *     - declarations per module;
 *     - imports per module;
 *     - exports per module;
 *     - packages;
 *     - resources;
 *     - devices;
 *     - CPUs;
 *     - cores;
 *     - threads;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *         - QPUs;
 *     - qubits;
 *     - nodes;
 *     - accelerators.
 *
 * There are intentionally no grammar constants such as:
 *
 *     MAX_MODULES
 *     MAX_MODULE_DEPTH
 *     MAX_MODULE_NAME_SEGMENTS
 *     MAX_MODULE_ITEMS
 *     MAX_IMPORTS
 *
 * Repetition is represented structurally with ANTLR repetition operators.
 *
 * Operational limits, where necessary, belong to compiler/resource policy.
 * Such limits MUST NOT become language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs:
 *
 *     - no filesystem I/O;
 *     - no network I/O;
 *     - no environment inspection;
 *     - no clock access;
 *     - no randomness;
 *     - no package lookup;
 *     - no module lookup;
 *     - no symbol lookup;
 *     - no hardware discovery;
 *     - no resource discovery;
 *     - no runtime calls.
 *
 * Therefore the syntactic interpretation of a deterministic token stream is
 * deterministic.
 *
 * ============================================================================
 * SAFETY / RUST
 * ============================================================================
 *
 * This file contains ANTLR grammar only.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no unsafe code.
 *
 * Generated compiler/frontend code is intended for:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The Zamani Rust implementation MUST use safe Rust only.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * Canonical lexical vocabulary:
 *
 *     ZamaniTokens
 *
 * Canonical shared name grammar:
 *
 *     QualifiedNames
 *
 * Canonical visibility grammar:
 *
 *     Visibility
 *
 * Canonical module attribute grammar:
 *
 *     ModuleAttributes
 *
 * The aggregate parser composes this module grammar with the rest of the
 * Zamani grammar.
 *
 * ============================================================================
 */

parser grammar Modules;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ============================================================================
 * DELEGATED GRAMMAR COMPONENTS
 * ============================================================================
 *
 * QualifiedNames:
 *
 *     Owns canonical qualified-name structure.
 *
 * Visibility:
 *
 *     Owns visibility vocabulary.
 *
 * ModuleAttributes:
 *
 *     Owns module-specific attribute syntax.
 *
 * These components MUST remain independent owners.
 *
 * ============================================================================
 */

import QualifiedNames, Visibility, ModuleAttributes;


/*
 * ============================================================================
 * 1. MODULE DECLARATION
 * ============================================================================
 *
 * Canonical forms:
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
 *     pub module math {
 *         ...
 *     }
 *
 *     @experimental
 *     pub module quantum::algorithms {
 *         ...
 *     }
 *
 * The module declaration may have:
 *
 *     - zero or more module attributes;
 *     - one optional visibility modifier;
 *     - one canonical qualified module name;
 *     - either a declaration terminator or an inline body.
 *
 * The semantic layer determines whether a particular combination is legal.
 *
 * This grammar only recognizes the syntax.
 */
moduleDeclaration
    : moduleAttributes?
      visibilityModifier?
      K_MODULE
      moduleName
      moduleDeclarationTail
    ;


/*
 * ============================================================================
 * 2. MODULE DECLARATION TAIL
 * ============================================================================
 *
 * A module declaration is either:
 *
 *     module name;
 *
 * or:
 *
 *     module name {
 *         ...
 *     }
 *
 * The two forms intentionally have different syntactic meaning:
 *
 *     `;`
 *
 *     declares a module without an inline body.
 *
 *     `{ ... }`
 *
 *     establishes an inline module body.
 *
 * A module body does not require a trailing semicolon.
 *
 * This gives deterministic declaration boundaries and avoids making module
 * parsing dependent on newline/trivia behavior.
 */
moduleDeclarationTail
    : SEMICOLON
    | moduleBody
    ;


/*
 * ============================================================================
 * 3. MODULE NAME
 * ============================================================================
 *
 * Module names use the repository's canonical qualified-name grammar.
 *
 * Examples:
 *
 *     math
 *
 *     math::linear
 *
 *     math::linear::matrix
 *
 *     quantum::algorithms
 *
 *     quantum::algorithms::optimization
 *
 *     hardware::accelerators
 *
 *     distributed::services
 *
 * No maximum number of segments is encoded here.
 *
 * This rule is a module-specific wrapper only.
 *
 * It does NOT redefine:
 *
 *     IDENTIFIER
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *
 * The canonical name grammar owns those structures.
 */
moduleName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 4. MODULE BODY
 * ============================================================================
 *
 * A module body contains the canonical Zamani `item` grammar.
 *
 * This is critical.
 *
 * Modules MUST NOT define a second declaration language.
 *
 * The body therefore delegates declaration ownership to the aggregate parser.
 *
 * Depending on the final parser assembly, `item` may contain constructs such
 * as:
 *
 *     imports
 *     exports
 *     nested modules
 *     declarations
 *     functions
 *     types
 *     quantum declarations
 *     classical declarations
 *     HDL declarations
 *     hardware declarations
 *     distributed declarations
 *     AI/data declarations
 *     future language constructs
 *
 * This allows one module abstraction to contain all Zamani computational
 * domains without the module grammar needing to know those domains.
 *
 * The module grammar therefore remains future-proof.
 */
moduleBody
    : LBRACE
      item*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. MODULE DECLARATION HEADER
 * ============================================================================
 *
 * Reusable header boundary.
 *
 * This rule intentionally excludes:
 *
 *     - the body;
 *     - the terminating semicolon.
 *
 * It is useful to aggregate tools that need to recognize a module header
 * independently of its contents.
 *
 * Example:
 *
 *     pub module quantum::algorithms
 *
 * The actual declaration is completed by `moduleDeclaration`.
 */
moduleDeclarationHeader
    : moduleAttributes?
      visibilityModifier?
      K_MODULE
      moduleName
    ;


/*
 * ============================================================================
 * 6. MODULE BODY ITEM BOUNDARY
 * ============================================================================
 *
 * This named wrapper exists so parser tooling can refer to the semantic
 * boundary between the module container and the canonical declaration item.
 *
 * It does NOT create another declaration grammar.
 */
moduleBodyItem
    : item
    ;


/*
 * ============================================================================
 * 7. MODULE BODY ITEM LIST
 * ============================================================================
 *
 * Zero or more canonical Zamani items.
 *
 * There is no finite item count.
 *
 * Resource exhaustion is an implementation concern, not a grammar semantic.
 */
moduleBodyItems
    : moduleBodyItem*
    ;


/*
 * ============================================================================
 * 8. EXPLICIT MODULE BODY
 * ============================================================================
 *
 * Named structural wrapper used by tools that need a parse-tree node for the
 * complete body independently of `moduleDeclaration`.
 *
 * This is equivalent in meaning to `moduleBody` and deliberately delegates
 * each item to the canonical parser.
 *
 * The rule is retained as a stable integration point.
 */
inlineModuleBody
    : LBRACE
      moduleBodyItems
      RBRACE
    ;


/*
 * ============================================================================
 * 9. MODULE PATH COMPATIBILITY WRAPPER
 * ============================================================================
 *
 * Existing downstream grammar/tooling may historically refer to a module path
 * as `modulePath`.
 *
 * The old implementation duplicated:
 *
 *     IDENTIFIER (DOUBLE_COLON IDENTIFIER)*
 *
 * That duplication is intentionally removed.
 *
 * `modulePath` is now only a semantic/syntactic wrapper around the canonical
 * qualified-name grammar.
 *
 * New code SHOULD prefer `moduleName`.
 *
 * Existing parser tooling may use this wrapper during migration.
 */
modulePath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 10. MODULE REFERENCE
 * ============================================================================
 *
 * A module reference is a canonical qualified name used in a module-specific
 * syntactic position.
 *
 * This rule does not resolve the module.
 *
 * Examples:
 *
 *     math
 *
 *     math::linear
 *
 *     quantum::algorithms
 *
 *     hardware::accelerators
 *
 * Semantic resolution determines what the referenced name denotes.
 */
moduleReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 11. MODULE DECLARATION CONTRACT
 * ============================================================================
 *
 * The parser produces enough structure for the frontend AST to preserve:
 *
 *     - module attributes;
 *     - explicit visibility;
 *     - module-name segments;
 *     - source spans;
 *     - source ordering;
 *     - whether the declaration has a body;
 *     - body contents;
 *     - exact source spelling where the AST requires provenance.
 *
 * The grammar does NOT define the Rust AST.
 *
 * Recommended semantic-neutral shape:
 *
 *     ModuleDeclaration
 *         attributes
 *         visibility
 *         name
 *         body
 *
 * Exact AST structures belong to the frontend.
 *
 * ============================================================================
 * 12. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - module identity;
 *     - module uniqueness;
 *     - module nesting semantics;
 *     - module/namespace relationships;
 *     - module/package relationships;
 *     - visibility validation;
 *     - import/export resolution;
 *     - symbol resolution;
 *     - dependency graph construction;
 *     - dependency-cycle detection;
 *     - package resolution;
 *     - module accessibility;
 *     - declaration ownership;
 *     - module attributes' meaning;
 *     - compilation-unit rules;
 *     - target-independent semantic validation.
 *
 * None of these checks belong in this grammar.
 *
 * ============================================================================
 * 13. IMPORT / EXPORT INTEGRATION
 * ============================================================================
 *
 * Imports remain owned by:
 *
 *     grammar/modules/imports.g4
 *
 * Exports remain owned by:
 *
 *     grammar/modules/exports.g4
 *
 * This file therefore does NOT define:
 *
 *     importDeclaration
 *     importClause
 *     importSpecifier
 *     exportDeclaration
 *     exportClause
 *     exportSpecifier
 *
 * Those constructs become members of `item` through the aggregate parser.
 *
 * This prevents duplicated ownership and prevents modules.g4 from becoming a
 * monolithic grammar.
 *
 * ============================================================================
 * 14. PACKAGE INTEGRATION
 * ============================================================================
 *
 * Package syntax remains owned by:
 *
 *     grammar/modules/packages.g4
 *
 * A package may semantically contain or organize modules, but that relationship
 * is not encoded as a filesystem or deployment assumption here.
 *
 * This grammar does not define:
 *
 *     packageDeclaration
 *     packageBody
 *     packageMetadataField
 *
 * ============================================================================
 * 15. DEPENDENCY INTEGRATION
 * ============================================================================
 *
 * Dependency syntax remains owned by:
 *
 *     grammar/modules/dependencies.g4
 *
 * Module declarations therefore do not contain a duplicated dependency
 * language.
 *
 * The semantic/package layer may later construct:
 *
 *     package/module dependency graph
 *
 * from the independently parsed constructs.
 *
 * ============================================================================
 * 16. NAMESPACE INTEGRATION
 * ============================================================================
 *
 * Namespace syntax remains owned by:
 *
 *     grammar/modules/namespaces.g4
 *
 * A module name may participate in a namespace relationship, but this grammar
 * does not decide that relationship.
 *
 * In particular:
 *
 *     module quantum::algorithms;
 *
 * does not itself mean that `quantum` is a package, namespace, filesystem
 * directory, or hardware domain.
 *
 * Semantic analysis determines the declaration's relationships.
 *
 * ============================================================================
 * 17. VISIBILITY INTEGRATION
 * ============================================================================
 *
 * Visibility is owned by:
 *
 *     grammar/modules/visibility.g4
 *
 * Therefore this file deliberately does NOT contain:
 *
 *     moduleVisibility
 *
 * with duplicated alternatives such as:
 *
 *     K_PUB
 *     K_PUBLIC
 *     K_PRIVATE
 *     K_PROTECTED
 *     K_INTERNAL
 *
 * The module declaration consumes:
 *
 *     visibilityModifier?
 *
 * from the canonical visibility grammar.
 *
 * This establishes exactly one visibility vocabulary throughout Zamani.
 *
 * ============================================================================
 * 18. ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * Module attributes are owned by:
 *
 *     grammar/modules/module-attributes.g4
 *
 * Therefore this file does not redefine:
 *
 *     @name
 *     @qualified::name
 *     attribute argument syntax
 *
 * The declaration consumes:
 *
 *     moduleAttributes?
 *
 * from the canonical module-attribute grammar.
 *
 * ============================================================================
 * 19. NAME INTEGRATION
 * ============================================================================
 *
 * Module names consume:
 *
 *     qualifiedName
 *
 * from the canonical name grammar.
 *
 * This means all Zamani domains can share the same structural name rules:
 *
 *     classical::math
 *     quantum::algorithms
 *     hardware::accelerators
 *     distributed::services
 *     ai::models
 *
 * No module-specific identifier syntax is introduced.
 *
 * ============================================================================
 * 20. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum module is ordinary module syntax.
 *
 * For example:
 *
 *     module quantum::algorithms {
 *         ...
 *     }
 *
 * This grammar does not know:
 *
 *     - qubits;
 *     - logical qubits;
 *     - physical qubits;
 *     - gates;
 *     - circuits;
 *     - QEC;
 *     - ZQN;
 *     - backend topology;
 *     - calibration;
 *     - scheduling;
 *     - routing.
 *
 * Quantum declarations inside the module are supplied by the canonical
 * `item` grammar and eventually lower through the repository's canonical
 * quantum semantic boundary, `quantum::ir`.
 *
 * Modules.g4 MUST NOT create a quantum-specific module IR.
 *
 * ============================================================================
 * 21. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical declarations use the same module mechanism.
 *
 * A module can contain classical functions, values, types, numerical
 * computation, concurrency, and other constructs through `item`.
 *
 * This file does not need classical-specific module syntax.
 *
 * ============================================================================
 * 22. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware declarations can inhabit ordinary Zamani modules.
 *
 * For example, the module grammar does not need separate constructs such as:
 *
 *     hardwareModule
 *     quantumModule
 *     gpuModule
 *     fpgaModule
 *     distributedModule
 *
 * unless a future language specification establishes genuinely different
 * source semantics.
 *
 * This keeps the module abstraction domain-independent.
 *
 * ============================================================================
 * 23. POCO-REAF HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Nothing in this file identifies:
 *
 *     - a device;
 *     - a processor;
 *     - a backend;
 *     - a topology;
 *     - a memory size;
 *     - a qubit count;
 *     - a node count;
 *     - an accelerator count.
 *
 * Consequently:
 *
 *     module computation;
 *
 * remains valid regardless of whether the eventual target is:
 *
 *     - an embedded machine;
 *     - one CPU;
 *     - many CPUs;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a simulator;
 *     - a heterogeneous system;
 *     - a cluster;
 *     - a cloud deployment;
 *     - a future architecture.
 *
 * ============================================================================
 * 24. NO FILESYSTEM SEMANTICS
 * ============================================================================
 *
 * A module name is a language name.
 *
 * It is not a filesystem path.
 *
 * Therefore:
 *
 *     module foo::bar;
 *
 * MUST NOT be interpreted by this grammar as:
 *
 *     foo/bar
 *
 * or:
 *
 *     foo/bar.zm
 *
 * Filesystem resolution belongs to compiler/toolchain infrastructure.
 *
 * The grammar does not perform filesystem resolution.
 *
 * ============================================================================
 * 25. NO PACKAGE / REGISTRY SEMANTICS
 * ============================================================================
 *
 * This grammar never contacts:
 *
 *     - package registries;
 *     - dependency servers;
 *     - remote repositories;
 *     - network services.
 *
 * A module declaration is therefore deterministic and offline-capable at the
 * grammar level.
 *
 * ============================================================================
 * 26. NO TARGET COUPLING
 * ============================================================================
 *
 * Module syntax cannot select:
 *
 *     - CPU;
 *     - GPU;
 *     - FPGA;
 *     - ASIC;
 *     - QPU;
 *     - accelerator;
 *     - node;
 *     - topology;
 *     - deployment.
 *
 * Target requirements belong to the appropriate resource/capability/target
 * grammar and semantic layers.
 *
 * ============================================================================
 * 27. ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors handled by this grammar include:
 *
 *     module;
 *
 *     module ::foo;
 *
 *     module foo::;
 *
 *     module foo {
 *
 *     module foo { };
 *
 * where the final case is invalid if the body form is followed by a token that
 * cannot begin the next canonical item.
 *
 * Semantic errors NOT handled here include:
 *
 *     duplicate module;
 *     unresolved module;
 *     inaccessible module;
 *     module/package conflict;
 *     module/namespace conflict;
 *     dependency cycle;
 *     invalid visibility;
 *     invalid attribute;
 *     invalid declaration in a module;
 *     invalid quantum operation;
 *     invalid hardware requirement.
 *
 * The parser must not use semantic predicates to implement those checks.
 *
 * ============================================================================
 * 28. AST / IR BOUNDARY
 * ============================================================================
 *
 * The flow is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     Modules
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
 *       +--> HDL / hardware representation
 *       +--> distributed / accelerator representation
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     hardware/runtime
 *
 * Modules.g4 MUST NOT bypass this architecture.
 *
 * ============================================================================
 * 29. TOOLING CONTRACT
 * ============================================================================
 *
 * The parse tree / AST must preserve enough structure for:
 *
 *     - module navigation;
 *     - go-to-definition;
 *     - symbol lookup;
 *     - module hierarchy visualization;
 *     - dependency analysis;
 *     - import analysis;
 *     - export analysis;
 *     - refactoring;
 *     - rename operations;
 *     - documentation extraction;
 *     - formatting;
 *     - source-preserving transformations;
 *     - diagnostics;
 *     - IDE/language-server support.
 *
 * This grammar must therefore preserve source structure rather than resolving
 * it during parsing.
 *
 * ============================================================================
 * 30. COMPATIBILITY
 * ============================================================================
 *
 * This version deliberately removes duplicated module-system ownership from the
 * historical implementation.
 *
 * The following historical rules are intentionally NOT retained as independent
 * definitions:
 *
 *     moduleVisibility
 *     importDeclaration
 *     importClause
 *     exportDeclaration
 *     exportClause
 *     packageDeclaration
 *     packageBody
 *     packageField
 *
 * They belong to their dedicated grammar components.
 *
 * The compatibility wrapper:
 *
 *     modulePath
 *
 * is retained as a thin qualified-name wrapper so existing parser/tooling
 * integration can migrate without recreating a second path grammar.
 *
 * New code SHOULD use:
 *
 *     moduleName
 *
 * for module declarations and:
 *
 *     qualifiedName
 *
 * for general name syntax.
 *
 * ============================================================================
 * 31. SCALABILITY AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_MODULES
 *     MAX_MODULE_DEPTH
 *     MAX_MODULE_ITEMS
 *     MAX_IMPORTS
 *     MAX_EXPORTS
 *     MAX_NAMESPACES
 *     MAX_QUANTUM_RESOURCES
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *
 * No fixed machine topology is represented.
 *
 * No fixed resource capacity is represented.
 *
 * No finite source-level module graph limit is represented.
 *
 * ============================================================================
 * 32. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] It parses canonical module declarations.
 *     [ ] It supports qualified module names.
 *     [ ] It supports arbitrary qualified-name depth permitted by resources.
 *     [ ] It composes canonical visibility syntax.
 *     [ ] It composes canonical module attributes.
 *     [ ] It delegates body contents to canonical `item`.
 *     [ ] It does not redefine imports.
 *     [ ] It does not redefine exports.
 *     [ ] It does not redefine packages.
 *     [ ] It does not redefine dependencies.
 *     [ ] It does not redefine namespaces.
 *     [ ] It does not redefine identifiers.
 *     [ ] It does not redefine qualified-name syntax.
 *     [ ] It contains no machine-size limits.
 *     [ ] It contains no hardware identifiers.
 *     [ ] It performs no I/O.
 *     [ ] It performs no semantic resolution.
 *     [ ] It creates no IR.
 *     [ ] It creates no quantum-specific IR.
 *     [ ] It contains no Rust actions.
 *     [ ] It requires no unsafe Rust.
 *     [ ] It integrates with the aggregate Zamani parser.
 *     [ ] It passes positive parser tests.
 *     [ ] It passes negative parser tests.
 *     [ ] It passes nested-module tests.
 *     [ ] It passes qualified-name scalability tests.
 *     [ ] It passes cross-domain module tests.
 *     [ ] It passes deterministic parsing tests.
 *
 * ============================================================================
 * END
 * ============================================================================
 */