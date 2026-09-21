/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/aliases.g4
 *
 * Grammar:
 *     ModuleAliases
 *
 * Status:
 *     CANONICAL MODULE-ALIAS GRAMMAR COMPONENT
 *
 * Purpose:
 *     Define source-level aliases for module identities without duplicating
 *     import aliases, namespace aliases, type aliases, dependency aliases,
 *     or general name syntax.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust safety:
 *     This grammar contains no Rust code and no embedded actions.
 *     Zamani's Rust implementation MUST remain safe Rust.
 *     No unsafe Rust is required for this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The module-alias grammar participates in:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     ModuleAliases
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     module/name semantic analysis
 *          |
 *          v
 *     resolved module graph
 *          |
 *          v
 *     semantic program model
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir       HDL/hardware
 *                              |
 *                              v
 *                        optimization
 *                              |
 *                        routing/scheduling
 *                              |
 *                         QEC/resilience
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                      target realization
 *
 * THIS FILE IS SYNTAX ONLY.
 *
 * It MUST NOT:
 *
 *     - resolve modules;
 *     - resolve packages;
 *     - resolve symbols;
 *     - access the filesystem;
 *     - access a registry;
 *     - access a network;
 *     - inspect environment variables;
 *     - discover hardware;
 *     - discover resources;
 *     - select a backend;
 *     - select a CPU;
 *     - select a GPU;
 *     - select an FPGA;
 *     - select an ASIC;
 *     - select a QPU;
 *     - allocate memory;
 *     - allocate qubits;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - perform ZQN analysis;
 *     - construct quantum::ir;
 *     - execute code.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - module alias declaration syntax;
 *     - module alias binding syntax;
 *     - module alias target syntax;
 *     - module-alias-specific structural wrappers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - import aliases;
 *     - export aliases;
 *     - namespace aliases;
 *     - dependency aliases;
 *     - type aliases;
 *     - lexical tokens;
 *     - module declarations;
 *     - package declarations;
 *     - dependency declarations;
 *     - import declarations;
 *     - export declarations;
 *     - namespace declarations;
 *     - visibility semantics;
 *     - package resolution;
 *     - module resolution;
 *     - symbol resolution;
 *     - semantic alias validation;
 *     - AST storage;
 *     - IR;
 *     - target realization.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Language architecture:
 *
 *     grammar/DESIGN.md
 *
 * Normative syntax:
 *
 *     grammar/spec/syntax.md
 *     grammar/specification/syntax.md
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical identifiers/names:
 *
 *     grammar/core/names.g4
 *
 * Qualified-name integration:
 *
 *     grammar/core/qualified-names.g4
 *
 * Module declaration:
 *
 *     grammar/modules/modules.g4
 *
 * Imports:
 *
 *     grammar/modules/imports.g4
 *
 * Exports:
 *
 *     grammar/modules/exports.g4
 *
 * Namespaces:
 *
 *     grammar/modules/namespaces.g4
 *
 * Dependencies:
 *
 *     grammar/modules/dependencies.g4
 *
 * Type aliases:
 *
 *     grammar/declarations/aliases.g4
 *
 * The existence of this file MUST NOT make any historical or proposed syntax
 * in grammar/Zamani-Grammar.md automatically legal.
 *
 * ============================================================================
 * LEXICAL VOCABULARY
 * ============================================================================
 *
 * This parser grammar consumes the canonical production lexer:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Canonical tokens used here are:
 *
 *     MODULE
 *     ALIAS
 *     ASSIGN
 *     SEMICOLON
 *     IDENTIFIER
 *
 * No K_* compatibility token is used here.
 *
 * In particular, this file MUST NOT use:
 *
 *     K_MODULE
 *     K_ALIAS
 *
 * and MUST NOT redefine any lexer token.
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * `grammar/core/names.g4` owns canonical name structure.
 *
 * This file MUST reuse:
 *
 *     identifier
 *     qualifiedName
 *
 * rather than defining:
 *
 *     identifier (DOUBLE_COLON identifier)*
 *
 * locally.
 *
 * This prevents the module system from acquiring a second name grammar.
 *
 * ============================================================================
 * MODULE ALIAS MODEL
 * ============================================================================
 *
 * A module alias gives a local source-level name to another module identity.
 *
 * Canonical form:
 *
 *     module alias local = target::module;
 *
 * Examples:
 *
 *     module alias math = math::linear;
 *
 *     module alias q = quantum::algorithms;
 *
 *     module alias hw = hardware::accelerators;
 *
 *     module alias tensor =
 *         libraries::numerical::tensor;
 *
 * The alias is a source-level symbolic binding.
 *
 * It does NOT:
 *
 *     - copy the target module;
 *     - create a second module;
 *     - create a second namespace;
 *     - load the target;
 *     - instantiate a package;
 *     - allocate resources;
 *     - select hardware;
 *     - select a backend;
 *     - create a quantum IR;
 *     - create an HDL implementation.
 *
 * ============================================================================
 * WHY `module alias`
 * ============================================================================
 *
 * Module aliases are deliberately syntactically distinct from:
 *
 *     import ... as ...;
 *
 *     namespace ... = ...;
 *
 *     type ... = ...;
 *
 *     dependency ... as ...;
 *
 * This distinction prevents a single generic alias construct from becoming
 * ambiguous across unrelated semantic owners.
 *
 * Import aliases rename imported bindings.
 *
 * Namespace aliases rename namespace references.
 *
 * Type aliases create type-level relationships.
 *
 * Dependency aliases rename dependency bindings.
 *
 * Module aliases name a module identity locally.
 *
 * ============================================================================
 * MODULE ALIAS DECLARATION
 * ============================================================================
 *
 * The declaration is deliberately small:
 *
 *     module alias <identifier> = <qualified-name>;
 *
 * The target is a symbolic qualified name.
 *
 * A literal string is intentionally NOT accepted here.
 *
 * Therefore:
 *
 *     module alias math = math::linear;
 *
 * is valid.
 *
 * Whereas:
 *
 *     module alias math = "math/linear";
 *
 * is not valid module-alias syntax.
 *
 * Literal source references belong to import/package/source-resolution
 * constructs where their semantics are explicitly defined.
 *
 * ============================================================================
 * CANONICAL RULES
 * ============================================================================
 */

parser grammar ModuleAliases;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 1. MODULE ALIAS DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     module alias <local-name> = <module-path>;
 *
 * Examples:
 *
 *     module alias math = math::linear;
 *
 *     module alias q = quantum::algorithms;
 *
 *     module alias hw = hardware::accelerators;
 *
 * No fixed number of declarations is imposed.
 */
moduleAliasDeclaration
    : MODULE
      ALIAS
      moduleAliasName
      ASSIGN
      moduleAliasTarget
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. MODULE ALIAS NAME
 * ============================================================================
 *
 * The alias binding is intentionally a simple identifier.
 *
 * A qualified alias binding such as:
 *
 *     module alias math::linear = ...
 *
 * is rejected.
 *
 * This keeps alias identity distinct from the target module identity and
 * avoids introducing a second namespace-binding model.
 */
moduleAliasName
    : identifier
    ;


/*
 * ============================================================================
 * 3. MODULE ALIAS TARGET
 * ============================================================================
 *
 * The target is a canonical qualified name.
 *
 * Examples:
 *
 *     math
 *
 *     math::linear
 *
 *     quantum::algorithms
 *
 *     hardware::accelerators::tensor
 *
 * No finite path depth is encoded.
 */
moduleAliasTarget
    : qualifiedName
    ;


/*
 * ============================================================================
 * 4. REUSABLE MODULE ALIAS LIST
 * ============================================================================
 *
 * Some aggregate grammars may need to process a sequence of module aliases.
 *
 * The list is non-empty when present.
 *
 * There is no language-level maximum.
 */
moduleAliasList
    : moduleAliasDeclaration+
    ;


/*
 * ============================================================================
 * 5. OPTIONAL MODULE ALIAS LIST
 * ============================================================================
 *
 * Explicit nullable wrapper for composition grammars.
 */
optionalModuleAliasList
    : moduleAliasList?
    ;


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Successful parsing establishes only:
 *
 *     local module alias syntax is structurally valid.
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the target module exists;
 *     - whether the target resolves;
 *     - whether the target is a module;
 *     - whether the alias is already bound;
 *     - whether the alias conflicts with a declaration;
 *     - whether the alias conflicts with an import;
 *     - whether the alias conflicts with a namespace;
 *     - whether the alias conflicts with a package;
 *     - whether the alias conflicts with another module alias;
 *     - whether the target is visible;
 *     - whether the alias is allowed in the current scope;
 *     - whether the alias introduces a cycle;
 *     - whether the alias crosses an incompatible package boundary;
 *     - whether version constraints are satisfied;
 *     - whether dialect compatibility is satisfied.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve at least:
 *
 *     ModuleAliasDeclaration {
 *         alias_name,
 *         target,
 *         source_span
 *     }
 *
 * Conceptually:
 *
 *     alias_name
 *         = Identifier
 *
 *     target
 *         = QualifiedName
 *
 * The exact Rust type names belong to the frontend AST owner.
 *
 * This grammar MUST NOT define a Rust AST type.
 *
 * It MUST NOT create:
 *
 *     ModuleAliasAst
 *     ModuleAliasIR
 *     QuantumModuleAliasIR
 *     HardwareModuleAliasIR
 *
 * or equivalent duplicate representations.
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The existing canonical source-level module AST is:
 *
 *     src/frontend/ast/node/program/module.rs
 *
 * It is a structural module container and intentionally does not contain
 * resolved alias state.
 *
 * Therefore this grammar's parsed alias declaration should initially remain
 * structural AST data.
 *
 * Alias resolution belongs to semantic analysis.
 *
 * The existing declarations compatibility boundary:
 *
 *     src/frontend/ast/node/declarations/module.rs
 *
 * MUST NOT become a second owner for module aliases.
 *
 * If module aliases require a dedicated AST node, that node belongs in the
 * canonical frontend AST declaration/path system rather than in this grammar.
 *
 * ============================================================================
 * NAME/IMPORT/EXPORT INTEGRATION
 * ============================================================================
 *
 * Module aliases must remain distinct from the following existing constructs.
 *
 * IMPORT:
 *
 *     import math::linear as linear;
 *
 * This creates/renames an imported binding.
 *
 * MODULE ALIAS:
 *
 *     module alias linear = math::linear;
 *
 * This creates a source-level module alias.
 *
 * NAMESPACE ALIAS:
 *
 *     namespace linear = math::linear;
 *
 * This creates a namespace alias.
 *
 * EXPORT ALIAS:
 *
 *     export math::linear as linear;
 *
 * This renames an exported interface member.
 *
 * TYPE ALIAS:
 *
 *     type Linear = math::Linear;
 *
 * This creates a type-level alias.
 *
 * DEPENDENCY ALIAS:
 *
 *     dependency math::linear as linear;
 *
 * This creates a dependency binding.
 *
 * These constructs MUST NOT be merged into one parser rule merely because
 * they contain the word `alias`.
 *
 * ============================================================================
 * MODULE INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/modules.g4` owns module declarations and module bodies.
 *
 * It should import this grammar component and expose:
 *
 *     moduleAliasDeclaration
 *
 * through the canonical module/source-item composition.
 *
 * `modules.g4` MUST NOT redefine:
 *
 *     moduleAliasDeclaration
 *     moduleAliasName
 *     moduleAliasTarget
 *
 * This file is the sole syntax owner of those rules.
 *
 * ============================================================================
 * IMPORT INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/imports.g4` remains the sole owner of import aliases.
 *
 * This file MUST NOT import or reuse `importAlias`.
 *
 * Import aliases are binding aliases for imported symbols/modules, whereas
 * module aliases are explicit module-identity aliases.
 *
 * ============================================================================
 * EXPORT INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/exports.g4` remains the sole owner of export aliases.
 *
 * This file MUST NOT define:
 *
 *     exportAlias
 *
 * or duplicate export syntax.
 *
 * ============================================================================
 * NAMESPACE INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/namespaces.g4` remains the sole owner of:
 *
 *     namespaceAliasDeclaration
 *
 * Module aliases and namespace aliases must remain structurally distinguishable
 * so semantic analysis can maintain one canonical namespace/module model.
 *
 * ============================================================================
 * DEPENDENCY INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/dependencies.g4` remains the sole owner of dependency
 * aliases.
 *
 * A module alias MUST NOT become a dependency declaration by implication.
 *
 * For example:
 *
 *     module alias q = quantum::algorithms;
 *
 * does not mean:
 *
 *     dependency quantum::algorithms;
 *
 * Dependency resolution remains the responsibility of dependency semantics.
 *
 * ============================================================================
 * PACKAGE INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/packages.g4` owns package declarations and package identity
 * syntax.
 *
 * Module aliases may point into package-qualified semantic module identities,
 * but this grammar does not resolve package membership.
 *
 * A qualified name is intentionally opaque at this level.
 *
 * ============================================================================
 * VISIBILITY INTEGRATION
 * ============================================================================
 *
 * Visibility semantics remain owned by:
 *
 *     grammar/modules/visibility.g4
 *
 * This grammar does not introduce:
 *
 *     public module alias ...
 *
 *     private module alias ...
 *
 *     protected module alias ...
 *
 * merely to duplicate the repository visibility system.
 *
 * If visibility of module aliases is standardized, the canonical declaration
 * composition layer should attach the existing visibility syntax rather than
 * creating another visibility grammar here.
 *
 * ============================================================================
 * VERSIONING / COMPATIBILITY
 * ============================================================================
 *
 * Module aliases are language syntax and therefore participate in the normal
 * language-version compatibility system.
 *
 * Semantic version checks belong downstream.
 *
 * The parser must not inspect:
 *
 *     compiler version;
 *     package version;
 *     module version;
 *     registry state;
 *     filesystem state.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Module alias syntax contains no machine-specific information.
 *
 * There are no limits on:
 *
 *     - number of module aliases;
 *     - number of modules;
 *     - number of packages;
 *     - number of module graph nodes;
 *     - number of module graph edges;
 *     - qualified-name depth;
 *     - number of source files;
 *     - number of compilation units;
 *     - number of domains;
 *     - number of quantum modules;
 *     - number of hardware modules.
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_MODULE_ALIASES
 *     MAX_MODULES
 *     MAX_ALIAS_DEPTH
 *     MAX_MODULE_DEPTH
 *     MAX_PACKAGES
 *     MAX_DEPENDENCIES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * Practical parser/compiler resource limits may exist, but they must be
 * configurable implementation/resource policies rather than language limits.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Module aliases are purely symbolic.
 *
 * They MUST NOT encode:
 *
 *     CPU identifiers;
 *     GPU identifiers;
 *     FPGA identifiers;
 *     ASIC identifiers;
 *     QPU identifiers;
 *     physical qubit identifiers;
 *     memory-bank identifiers;
 *     network-node identifiers;
 *     device addresses;
 *     topology;
 *     core counts;
 *     thread counts;
 *     accelerator counts.
 *
 * For example:
 *
 *     module alias quantum = quantum::algorithms;
 *
 * is portable source-level organization.
 *
 * It does not select:
 *
 *     a QPU;
 *     a physical qubit;
 *     a coupling map;
 *     a calibration;
 *     a backend.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The target may eventually contain modules associated with:
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
 *     embedded
 *     HPC
 *     accelerator
 *     future domains
 *
 * The grammar does not enumerate those domains.
 *
 * Therefore:
 *
 *     module alias q = quantum::algorithms;
 *
 *     module alias h = hdl::pipeline;
 *
 *     module alias a = ai::training;
 *
 *     module alias d = distributed::runtime;
 *
 * all use exactly the same syntax.
 *
 * Adding a new domain does not require modifying this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A module alias such as:
 *
 *     module alias q = quantum::algorithms;
 *
 * is only source organization.
 *
 * It does NOT create or modify:
 *
 *     quantum::ir
 *
 * It does not select a gate set, QPU, qubit topology, calibration, QEC
 * strategy, scheduler, or routing strategy.
 *
 * The eventual semantic path remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic module/name resolution
 *       |
 *       v
 *     quantum semantics
 *       |
 *       v
 *     quantum::ir
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
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A module alias such as:
 *
 *     module alias accelerator = hardware::accelerators;
 *
 * does not select:
 *
 *     an FPGA;
 *     an ASIC;
 *     a device;
 *     a memory bank;
 *     a physical interconnect;
 *     a clock;
 *     a pin;
 *     a bus address.
 *
 * Those decisions belong to downstream hardware/resource/compiler layers.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no registry access;
 *     - no environment access;
 *     - no hardware discovery;
 *     - no resource discovery;
 *     - no randomness;
 *     - no clock access.
 *
 * Identical token streams must produce identical parse structures for the same
 * grammar version.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The parser/AST integration must preserve source spans for:
 *
 *     `module`
 *     `alias`
 *     alias name
 *     `=`
 *     target qualified name
 *     terminating semicolon
 *     complete declaration
 *
 * This allows diagnostics to identify:
 *
 *     invalid alias;
 *     unresolved target;
 *     alias collision;
 *     alias cycle;
 *     visibility violation;
 *     incompatible module identity.
 *
 * Source spans remain source locations only.
 *
 * They must never be interpreted as physical hardware/resource locations.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors should naturally identify malformed forms such as:
 *
 *     module alias;
 *
 *     module alias = math::linear;
 *
 *     module alias math;
 *
 *     module alias math =;
 *
 *     module alias math = ;
 *
 *     module alias math = math::linear
 *
 *     module alias math::linear = other::module;
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples of downstream-only errors:
 *
 *     module alias math = missing::module;
 *
 *     module alias math = math::linear;
 *     module alias math = other::module;
 *
 *     module alias a = b;
 *     module alias b = a;
 *
 * The grammar must parse these structures where syntactically valid and allow
 * semantic analysis to report the appropriate error.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     module alias math = math::linear;
 *
 *     module alias q = quantum::algorithms;
 *
 *     module alias hw = hardware::accelerators;
 *
 *     module alias tensor = libraries::numerical::tensor;
 *
 *     module alias classical = classical::numeric;
 *
 *     module alias distributed = distributed::runtime;
 *
 *     module alias ai = ai::training;
 *
 * NEGATIVE:
 *
 *     module alias;
 *
 *     module alias = math::linear;
 *
 *     module alias math;
 *
 *     module alias math =;
 *
 *     module alias math = ;
 *
 *     module alias math = math::;
 *
 *     module alias math::linear = other::module;
 *
 *     module alias math = "math";
 *
 *     module alias math = 123;
 *
 *     module alias math = @invalid;
 *
 * BOUNDARY:
 *
 *     one segment target;
 *     deeply qualified target;
 *     Unicode-valid identifier where supported by the canonical lexer;
 *     long alias names subject only to implementation resource policy;
 *     large sequences of aliases;
 *     aliases in nested module scopes where modules.g4 permits them.
 *
 * SCALABILITY:
 *
 *     zero aliases;
 *     one alias;
 *     many aliases;
 *     deeply nested module graphs;
 *     large module graphs;
 *     large qualified names;
 *     no MAX_MODULE_ALIASES;
 *     no MAX_MODULE_DEPTH;
 *     no MAX_NAME_SEGMENTS.
 *
 * CROSS-DOMAIN:
 *
 *     classical::...
 *     quantum::...
 *     hdl::...
 *     hardware::...
 *     distributed::...
 *     ai::...
 *     data::...
 *     networking::...
 *     security::...
 *
 * must all be treated as ordinary qualified names.
 *
 * DETERMINISM:
 *
 *     identical token stream -> identical parse structure.
 *
 * ROUND-TRIP:
 *
 *     parse -> AST -> formatter/printer -> parse
 *
 * must preserve:
 *
 *     alias name;
 *     target;
 *     declaration identity;
 *     source intent.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_MODULE_ALIASES
 *     MAX_MODULES
 *     MAX_MODULE_DEPTH
 *     MAX_PACKAGE_COUNT
 *     MAX_DEPENDENCY_COUNT
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *
 * It contains no:
 *
 *     device IDs;
 *     hardware addresses;
 *     physical topology;
 *     vendor-specific module identifiers;
 *     backend-specific syntax.
 *
 * ============================================================================
 * SECURITY AUDIT
 * ============================================================================
 *
 * Alias targets are untrusted source-level names.
 *
 * Parsing performs no:
 *
 *     filesystem lookup;
 *     network request;
 *     registry request;
 *     package download;
 *     code execution;
 *     environment lookup;
 *     dynamic loading.
 *
 * Semantic/module resolution is responsible for applying:
 *
 *     trust policy;
 *     visibility policy;
 *     package policy;
 *     dependency policy;
 *     sandbox policy;
 *     capability policy.
 *
 * ============================================================================
 * PERFORMANCE / RESOURCE CONTRACT
 * ============================================================================
 *
 * The grammar uses only ordinary ANTLR parser constructs:
 *
 *     repetition;
 *     rule references;
 *     token matching.
 *
 * It contains no embedded loops, actions, predicates, or unbounded semantic
 * computation.
 *
 * Any resource budget for parsing must be supplied by the parser/toolchain
 * infrastructure rather than encoded as a language constant.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It compiles as an ANTLR parser grammar.
 * [ ] It consumes the canonical ZamaniLexer vocabulary.
 * [ ] It imports the canonical Names grammar.
 * [ ] It defines exactly one owner for module-alias syntax.
 * [ ] It does not redefine identifier syntax.
 * [ ] It does not redefine qualified-name syntax.
 * [ ] It does not duplicate import aliases.
 * [ ] It does not duplicate export aliases.
 * [ ] It does not duplicate namespace aliases.
 * [ ] It does not duplicate dependency aliases.
 * [ ] It does not duplicate type aliases.
 * [ ] It introduces no semantic predicates.
 * [ ] It performs no I/O.
 * [ ] It introduces no hardware assumptions.
 * [ ] It introduces no resource limits.
 * [ ] It preserves source-level alias and target structure.
 * [ ] Its AST contract is documented before AST implementation.
 * [ ] Its semantic contract is documented.
 * [ ] Its module-graph integration is documented.
 * [ ] Its cross-domain integration is documented.
 * [ ] Its quantum::ir boundary is documented.
 * [ ] Its HDL/hardware boundary is documented.
 * [ ] Positive tests exist.
 * [ ] Negative tests exist.
 * [ ] Boundary tests exist.
 * [ ] Scalability tests exist.
 * [ ] Determinism tests exist.
 * [ ] Round-trip tests exist.
 * [ ] Compatibility tests exist.
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */