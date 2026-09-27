/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/exports.g4
 *
 * Grammar:
 *     DialectExports
 *
 * Status:
 *     CANONICAL DIALECT-EXPORT PARSER COMPONENT
 *
 * Purpose:
 *     Define source-level export syntax belonging to a Zamani dialect
 *     declaration without duplicating the ordinary module-export grammar.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * canonical lexer
 *      |
 *      v
 * canonical parser
 *      |
 *      v
 * dialect declaration
 *      |
 *      v
 * DialectExports                         <-- THIS FILE
 *      |
 *      v
 * domain-neutral frontend AST
 *      |
 *      v
 * dialect semantic analysis
 *      |
 *      +--> namespace resolution
 *      +--> symbol resolution
 *      +--> visibility analysis
 *      +--> capability analysis
 *      +--> version compatibility
 *      +--> extension validation
 *      +--> dependency analysis
 *      |
 *      v
 * canonical semantic representation
 *      |
 *      +--> classical representation
 *      +--> quantum::ir
 *      +--> HDL / hardware representation
 *      +--> data / AI / distributed representations
 *      |
 *      v
 * optimization / lowering
 *      |
 *      +--> routing
 *      +--> scheduling
 *      +--> resilience
 *      +--> QEC
 *      +--> ZQN
 *      +--> HAL
 *      |
 *      v
 * target realization
 *
 * This file participates only in source-level parsing.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * This file is NOT the ordinary module-export grammar.
 *
 * Ordinary module exports are owned by:
 *
 *     grammar/modules/exports.g4
 *
 * This file owns only:
 *
 *     exports declared as members of a dialect declaration.
 *
 * Therefore the semantic distinction is:
 *
 *     module export
 *          !=
 *     dialect export
 *
 * even where their surface syntax is intentionally similar.
 *
 * The surrounding grammar context determines which construct is being parsed.
 *
 * This prevents:
 *
 *     grammar/modules/exports.g4
 *
 * and:
 *
 *     grammar/dialects/exports.g4
 *
 * from becoming competing definitions of the same AST concept.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The authority chain is:
 *
 *     grammar/DESIGN.md
 *             |
 *             v
 *     grammar/specification/
 *             |
 *             v
 *     grammar/dialects/exports.g4
 *             |
 *             v
 *     dialect AST contract
 *             |
 *             v
 *     dialect semantic model
 *
 * This file MUST NOT redefine:
 *
 *     module export semantics;
 *     module resolution;
 *     symbol resolution;
 *     visibility semantics;
 *     package resolution;
 *     dependency resolution;
 *     capability resolution;
 *     version resolution;
 *     IR;
 *     runtime behavior.
 *
 * ============================================================================
 * RELATIONSHIP TO EXISTING FILES
 * ============================================================================
 *
 * Existing canonical components:
 *
 *     grammar/core/names.g4
 *         owns source-level names.
 *
 *     grammar/core/qualified-names.g4
 *         owns qualified-name integration.
 *
 *     grammar/modules/exports.g4
 *         owns ordinary module exports.
 *
 *     grammar/dialects/registration.g4
 *         owns the dialect registration/member structure.
 *
 *     grammar/dialects/dialect.g4
 *         owns the public dialect parser boundary.
 *
 *     grammar/dialects/capabilities.g4
 *         owns dialect capability syntax.
 *
 *     grammar/dialects/versioning.g4
 *         owns dialect version syntax.
 *
 *     grammar/dialects/compatibility.g4
 *         owns dialect compatibility syntax.
 *
 * This file composes those contracts rather than redefining them.
 *
 * ============================================================================
 * INTEGRATION REQUIREMENT
 * ============================================================================
 *
 * The dialect registration grammar should import this grammar:
 *
 *     import
 *         Names,
 *         DialectExports,
 *         ...
 *
 * and its member dispatcher should expose:
 *
 *     dialectExportDeclaration
 *
 * as a dialect-registration member.
 *
 * Conceptually:
 *
 *     dialectRegistrationMember
 *         : dialectRegistrationImport
 *         | dialectRegistrationUse
 *         | dialectRegistrationExtends
 *         | dialectRegistrationRequires
 *         | dialectRegistrationProvides
 *         | dialectExportDeclaration
 *         | dialectRegistrationExtension
 *         | ...
 *         ;
 *
 * The exact ordering belongs to registration.g4.
 *
 * This file does not duplicate that dispatcher.
 *
 * ============================================================================
 * WHY A SEPARATE DIALECT EXPORT GRAMMAR EXISTS
 * ============================================================================
 *
 * A dialect can expose language facilities to consumers.
 *
 * Examples include:
 *
 *     export quantum::operation;
 *     export quantum::measurement;
 *     export capability::dynamic_control;
 *     export syntax::operation;
 *     export semantics::measurement;
 *
 * These declarations describe the public surface of a dialect.
 *
 * They do NOT mean:
 *
 *     export a hardware device;
 *     export a physical qubit;
 *     export a GPU;
 *     export a CPU;
 *     export FPGA resources;
 *     export memory;
 *     export a network node;
 *     export a scheduler;
 *     export a runtime object.
 *
 * The semantic layer decides what an exported symbolic member represents.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * This grammar MUST NOT enumerate dialect domains.
 *
 * It must therefore NOT contain constructs such as:
 *
 *     quantumDialect
 *     classicalDialect
 *     HdlDialect
 *     CudaDialect
 *     QiskitDialect
 *     VendorDialectX
 *
 * as closed alternatives.
 *
 * Dialect members are symbolic qualified names.
 *
 * Examples:
 *
 *     quantum::operation
 *     classical::numeric
 *     hdl::rtl
 *     hardware::intent
 *     distributed::collective
 *     ai::tensor
 *     future::computing::facility
 *     organization::research::extension
 *
 * A new domain does not require modification to this file.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialect exports are source-level contracts.
 *
 * They must remain independent of target realization.
 *
 * An exported dialect member MUST NOT encode:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     ASIC identity
 *     QPU identity
 *     physical qubit identity
 *     physical memory address
 *     fixed memory capacity
 *     fixed register width
 *     fixed topology
 *     fixed device count
 *     fixed node count
 *     fixed thread count
 *     fixed accelerator count
 *     fixed hardware placement
 *     calibration state
 *     scheduler selection
 *     routing decisions
 *
 * Therefore dialect exports preserve:
 *
 *     Program Once
 *          |
 *          v
 *     portable dialect contract
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     capability/resource adaptation
 *          |
 *          v
 *     Run Everywhere
 *          |
 *          v
 *     Run Anywhere
 *          |
 *          v
 *     Run Forever
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are no language-level limits for:
 *
 *     export declarations
 *     export specifiers
 *     aliases
 *     qualified-name depth
 *     dialect members
 *     dialects
 *     namespaces
 *     capabilities
 *     extensions
 *     resources
 *     targets
 *     devices
 *     nodes
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     memory
 *     threads
 *
 * Lists use:
 *
 *     *
 *     +
 *
 * rather than fixed capacities.
 *
 * This grammar MUST NOT define:
 *
 *     MAX_DIALECT_EXPORTS
 *     MAX_EXPORTS
 *     MAX_EXPORT_SPECIFIERS
 *     MAX_ALIAS_COUNT
 *     MAX_NAMESPACE_DEPTH
 *     MAX_DIALECT_MEMBERS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICE_COUNT
 *
 * Practical parser/compiler resource exhaustion is an implementation concern,
 * not a source-language semantic restriction.
 *
 * ============================================================================
 * HARD-CODING POLICY
 * ============================================================================
 *
 * Numeric literals are not inherently forbidden.
 *
 * Program semantics may legitimately contain numeric values.
 *
 * What is forbidden is turning current hardware capacity into language
 * restrictions.
 *
 * This file therefore contains no hardware capacity assumptions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no package lookup;
 *     no registry lookup;
 *     no hardware discovery;
 *     no runtime callbacks;
 *     no randomness;
 *     no clock access.
 *
 * Identical token streams and grammar versions receive identical syntactic
 * treatment.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Export source references are untrusted source input.
 *
 * Parsing MUST NOT:
 *
 *     open files;
 *     fetch URLs;
 *     contact registries;
 *     execute commands;
 *     load plugins;
 *     access credentials;
 *     inspect hardware;
 *     resolve devices;
 *     allocate resources.
 *
 * All such operations belong to explicitly authorized downstream systems.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum dialect exports remain symbolic.
 *
 * Examples:
 *
 *     export quantum::operation;
 *     export quantum::measurement;
 *     export quantum::dynamic_control;
 *
 * This grammar does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumCircuit
 *     QuantumOperation
 *     QuantumTopology
 *     Calibration
 *     NoiseModel
 *
 * It does not construct quantum::ir.
 *
 * If an exported facility eventually describes quantum computation, the
 * downstream semantic pipeline remains:
 *
 *     AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * The same export syntax can expose:
 *
 *     classical facilities;
 *     quantum facilities;
 *     hybrid facilities;
 *     HDL facilities;
 *     hardware-intent facilities;
 *     distributed facilities;
 *     AI facilities;
 *     data facilities;
 *     networking facilities;
 *     security facilities;
 *     interoperability facilities;
 *     future facilities.
 *
 * The grammar does not special-case any of them.
 *
 * ============================================================================
 * EXPORT MODEL
 * ============================================================================
 *
 * The dialect export model supports:
 *
 *     1. Direct export
 *
 *         export quantum::operation;
 *
 *     2. Direct export with alias
 *
 *         export quantum::operation as operation;
 *
 *     3. Named export group
 *
 *         export {
 *             quantum::operation,
 *             quantum::measurement
 *         };
 *
 *     4. Named export group with aliases
 *
 *         export {
 *             quantum::operation as operation,
 *             quantum::measurement as measure
 *         };
 *
 *     5. Wildcard export
 *
 *         export *;
 *
 *     6. Re-export from another dialect
 *
 *         export quantum::operation from quantum::base;
 *
 *     7. Re-export wildcard from another dialect
 *
 *         export * from quantum::base;
 *
 *     8. Namespace-style wildcard re-export
 *
 *         export * as quantum from quantum::base;
 *
 *     9. Literal source re-export
 *
 *         export quantum::operation from "external-dialect";
 *
 * The grammar preserves all source information.
 *
 * Semantic analysis determines whether the declarations are legal.
 *
 * ============================================================================
 * EXPORT DOES NOT MEAN IMPLEMENTATION
 * ============================================================================
 *
 * An export declaration does not itself provide:
 *
 *     an implementation;
 *     a backend;
 *     a compiler;
 *     a runtime;
 *     a target;
 *     a hardware mapping;
 *     a QPU;
 *     an FPGA;
 *     a GPU;
 *     a CPU;
 *     a physical resource.
 *
 * It only declares a public source-level surface.
 *
 * ============================================================================
 * IMPORT/EXPORT DISTINCTION
 * ============================================================================
 *
 * This file does not own imports.
 *
 * Dialect imports remain owned by:
 *
 *     grammar/dialects/imports.g4
 *
 * These constructs are intentionally distinct:
 *
 *     import quantum::base;
 *
 * versus:
 *
 *     export quantum::operation from quantum::base;
 *
 * Import resolution and export resolution are separate semantic operations.
 *
 * ============================================================================
 * MODULE EXPORT DISTINCTION
 * ============================================================================
 *
 * Ordinary module exports remain owned by:
 *
 *     grammar/modules/exports.g4
 *
 * For example:
 *
 *     module math {
 *         export linear;
 *     }
 *
 * is a module export.
 *
 * Within a dialect declaration:
 *
 *     dialect quantum::extended {
 *         export quantum::operation;
 *     }
 *
 * is a dialect export.
 *
 * The two constructs can intentionally share surface syntax while retaining
 * distinct AST ownership and semantic meaning through parser context.
 *
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * Canonical qualified-name ownership remains outside this file.
 *
 * This grammar therefore imports:
 *
 *     QualifiedNames
 *
 * and consumes:
 *
 *     qualifiedNameReference
 *
 * rather than redefining:
 *
 *     identifier
 *     simpleName
 *     qualifiedName
 *     qualifiedNameReference
 *
 * This prevents grammar drift.
 *
 * ============================================================================
 */

parser grammar DialectExports;

options {
    tokenVocab = ZamaniLexer;
}

import QualifiedNames;


/*
 * ============================================================================
 * 1. PUBLIC DIALECT EXPORT ENTRY POINT
 * ============================================================================
 *
 * This is the only public export declaration rule owned by this file.
 *
 * It is intended to be consumed by dialect registration/member grammars.
 */
dialectExportDeclaration
    : EXPORT
      dialectExportTarget
      dialectExportSourceClause?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. EXPORT TARGET
 * ============================================================================
 *
 * A dialect export target may be:
 *
 *     a symbolic path;
 *     a named export group;
 *     a wildcard.
 */
dialectExportTarget
    : dialectExportWildcardTarget
    | dialectExportNamedTarget
    | dialectExportPathTarget
    ;


/*
 * ============================================================================
 * 3. PATH EXPORT
 * ============================================================================
 *
 * Examples:
 *
 *     export quantum::operation;
 *     export quantum::operation as operation;
 *
 * The path is always interpreted as symbolic source-level information.
 */
dialectExportPathTarget
    : qualifiedNameReference
      dialectExportTargetAlias?
    ;


/*
 * ============================================================================
 * 4. PATH EXPORT ALIAS
 * ============================================================================
 *
 * Alias syntax remains context-specific to export declarations.
 *
 * The alias itself is an ordinary lexical identifier.
 */
dialectExportTargetAlias
    : AS
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 5. NAMED EXPORT GROUP
 * ============================================================================
 *
 * Examples:
 *
 *     export {
 *         quantum::operation,
 *         quantum::measurement
 *     };
 *
 *     export {
 *         quantum::operation as operation,
 *         quantum::measurement as measure
 *     };
 *
 * Empty groups are intentionally rejected.
 */
dialectExportNamedTarget
    : LBRACE
      dialectExportSpecifierList
      RBRACE
    ;


/*
 * ============================================================================
 * 6. EXPORT SPECIFIER LIST
 * ============================================================================
 *
 * Arbitrary source-level cardinality is permitted.
 *
 * A trailing comma is accepted for generated code and formatter stability.
 */
dialectExportSpecifierList
    : dialectExportSpecifier
      (COMMA dialectExportSpecifier)*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. EXPORT SPECIFIER
 * ============================================================================
 *
 * Examples:
 *
 *     quantum::operation
 *
 *     quantum::operation as operation
 *
 * The grammar preserves the symbolic path and alias independently.
 */
dialectExportSpecifier
    : qualifiedNameReference
      dialectExportSpecifierAlias?
    ;


/*
 * ============================================================================
 * 8. EXPORT SPECIFIER ALIAS
 * ============================================================================
 */
dialectExportSpecifierAlias
    : AS
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 9. WILDCARD EXPORT
 * ============================================================================
 *
 * Examples:
 *
 *     export *;
 *
 *     export * as quantum;
 *
 * The alias is optional syntactic information.
 *
 * Wildcard expansion is NEVER performed by the parser.
 */
dialectExportWildcardTarget
    : STAR
      dialectExportWildcardAlias?
    ;


/*
 * ============================================================================
 * 10. WILDCARD ALIAS
 * ============================================================================
 *
 * The alias identifies the exported namespace in source-level syntax.
 *
 * It is not a namespace resolver.
 */
dialectExportWildcardAlias
    : AS
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 11. EXPORT SOURCE CLAUSE
 * ============================================================================
 *
 * A source clause makes the export a re-export.
 *
 * Examples:
 *
 *     export quantum::operation from quantum::base;
 *
 *     export quantum::operation from "external-dialect";
 *
 * Resolution belongs downstream.
 */
dialectExportSourceClause
    : FROM
      dialectExportSourceReference
    ;


/*
 * ============================================================================
 * 12. EXPORT SOURCE REFERENCE
 * ============================================================================
 *
 * Two source forms are preserved:
 *
 *     symbolic qualified source;
 *     opaque string source.
 *
 * The parser does not decide whether a string is:
 *
 *     a package;
 *     a module;
 *     a registry coordinate;
 *     a generated source;
 *     a remote source;
 *     another provider.
 *
 * Those distinctions belong to semantic/toolchain layers.
 */
dialectExportSourceReference
    : qualifiedNameReference
    | STRING
    ;


/*
 * ============================================================================
 * 13. COMPLETE TARGET WRAPPER
 * ============================================================================
 *
 * Stable integration rule for AST/tooling consumers.
 */
dialectExportTargetReference
    : dialectExportTarget
    ;


/*
 * ============================================================================
 * 14. COMPLETE SOURCE WRAPPER
 * ============================================================================
 */
dialectExportSourceReferenceClause
    : dialectExportSourceClause
    ;


/*
 * ============================================================================
 * 15. NAMED ENTRY WRAPPER
 * ============================================================================
 */
dialectExportNamedEntry
    : dialectExportSpecifier
    ;


/*
 * ============================================================================
 * 16. NAMED ENTRY LIST WRAPPER
 * ============================================================================
 */
dialectExportNamedEntryList
    : dialectExportSpecifierList
    ;


/*
 * ============================================================================
 * 17. LOCAL DIALECT EXPORT
 * ============================================================================
 *
 * A local export has no source clause.
 *
 * Examples:
 *
 *     export quantum::operation;
 *
 *     export {
 *         quantum::operation,
 *         quantum::measurement
 *     };
 */
dialectLocalExportDeclaration
    : EXPORT
      dialectExportTarget
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. DIALECT RE-EXPORT
 * ============================================================================
 *
 * A re-export has a mandatory source clause.
 *
 * Examples:
 *
 *     export quantum::operation from quantum::base;
 *
 *     export * from quantum::base;
 */
dialectReExportDeclaration
    : EXPORT
      dialectExportTarget
      dialectExportSourceClause
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. WILDCARD RE-EXPORT
 * ============================================================================
 *
 * Explicit wrapper for tooling and semantic consumers.
 */
dialectWildcardReExportDeclaration
    : EXPORT
      STAR
      dialectExportWildcardAlias?
      dialectExportSourceClause
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. DIALECT EXPORT PATH
 * ============================================================================
 *
 * Stable wrapper around a symbolic export path.
 */
dialectExportPath
    : qualifiedNameReference
    ;


/*
 * ============================================================================
 * 21. DIALECT EXPORT ALIAS
 * ============================================================================
 *
 * Stable wrapper for tooling.
 */
dialectExportAlias
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 22. DIALECT EXPORT SOURCE
 * ============================================================================
 *
 * Stable wrapper for tooling.
 */
dialectExportSource
    : dialectExportSourceReference
    ;


/*
 * ============================================================================
 * 23. DIALECT EXPORT LIST
 * ============================================================================
 *
 * Stable list wrapper.
 */
dialectExportList
    : dialectExportSpecifierList
    ;


/*
 * ============================================================================
 * 24. OPTIONAL DIALECT EXPORT LIST
 * ============================================================================
 */
optionalDialectExportList
    : dialectExportSpecifierList?
    ;


/*
 * ============================================================================
 * 25. OPTIONAL DIALECT EXPORT SOURCE
 * ============================================================================
 */
optionalDialectExportSource
    : dialectExportSourceClause?
    ;


/*
 * ============================================================================
 * 26. SYMBOLIC EXPORT REFERENCE
 * ============================================================================
 *
 * This rule is intentionally narrow.
 *
 * It represents a symbolic source-level exported member, not an implementation
 * object.
 */
dialectExportSymbolicReference
    : qualifiedNameReference
    ;


/*
 * ============================================================================
 * 27. SYMBOLIC EXPORT REFERENCE LIST
 * ============================================================================
 */
dialectExportSymbolicReferenceList
    : dialectExportSymbolicReference
      (COMMA dialectExportSymbolicReference)*
    ;


/*
 * ============================================================================
 * 28. DIALECT EXPORT TARGET OR GROUP
 * ============================================================================
 *
 * Integration wrapper.
 */
dialectExportTargetOrGroup
    : dialectExportPathTarget
    | dialectExportNamedTarget
    | dialectExportWildcardTarget
    ;


/*
 * ============================================================================
 * 29. DIALECT EXPORT DECLARATION LIST
 * ============================================================================
 *
 * Useful for parser/tooling integrations that operate over multiple dialect
 * members.
 *
 * No finite cardinality is imposed.
 */
dialectExportDeclarationList
    : dialectExportDeclaration*
    ;


/*
 * ============================================================================
 * 30. AST CONTRACT
 * ============================================================================
 *
 * This grammar produces syntax only.
 *
 * The frontend AST must preserve enough information to distinguish:
 *
 *     local dialect export
 *     dialect re-export
 *     wildcard export
 *     wildcard re-export
 *     named export group
 *     export alias
 *     source reference
 *     symbolic qualified name
 *     complete source span
 *
 * A semantic-neutral conceptual representation is:
 *
 *     DialectExportDeclaration {
 *         target
 *         source?
 *         span
 *     }
 *
 * Target variants:
 *
 *     Path {
 *         path
 *         alias?
 *     }
 *
 *     Named {
 *         specifiers[]
 *     }
 *
 *     Wildcard {
 *         alias?
 *     }
 *
 * Specifier:
 *
 *     DialectExportSpecifier {
 *         path
 *         alias?
 *         span
 *     }
 *
 * Source:
 *
 *     SymbolicSource {
 *         qualified_name
 *     }
 *
 *     LiteralSource {
 *         literal
 *     }
 *
 * The exact Rust AST type is owned by:
 *
 *     src/frontend/ast/
 *
 * or the repository's established frontend AST authority.
 *
 * This grammar MUST NOT create another AST.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The parser/AST layer must preserve source locations for:
 *
 *     EXPORT
 *     exported path
 *     alias
 *     wildcard
 *     FROM
 *     source reference
 *     complete declaration
 *
 * These spans are required for:
 *
 *     diagnostics;
 *     IDE support;
 *     formatting;
 *     refactoring;
 *     documentation;
 *     provenance;
 *     compatibility diagnostics.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - symbol existence;
 *     - dialect existence;
 *     - dialect visibility;
 *     - export visibility;
 *     - alias legality;
 *     - alias collisions;
 *     - wildcard expansion;
 *     - source resolution;
 *     - import/export cycles;
 *     - dependency resolution;
 *     - version compatibility;
 *     - capability validation;
 *     - extension validity;
 *     - namespace validity;
 *     - trust policy;
 *     - deprecation;
 *     - feature-gate validation.
 *
 * The parser MUST NOT perform any of these operations.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY DISTINCTION
 * ============================================================================
 *
 * An exported symbolic member may represent a capability contract.
 *
 * For example:
 *
 *     export quantum::dynamic_control;
 *
 * does not mean:
 *
 *     use a specific QPU;
 *
 * and does not guarantee:
 *
 *     any particular number of qubits.
 *
 * The semantic layer determines what capability contract the symbol denotes.
 *
 * ============================================================================
 * RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * This file contains no source-level hardware capacities.
 *
 * It does not define:
 *
 *     qubit counts;
 *     processor counts;
 *     GPU counts;
 *     FPGA counts;
 *     node counts;
 *     memory sizes;
 *     thread counts;
 *     register widths;
 *     tensor-rank limits;
 *     topology limits.
 *
 * Resource requirements belong to the resource/capability subsystem.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Dialect exports do not directly lower to:
 *
 *     classical IR;
 *     quantum::ir;
 *     HDL IR;
 *     hardware IR.
 *
 * Instead:
 *
 *     dialect export syntax
 *          |
 *          v
 *     dialect semantic model
 *          |
 *          v
 *     domain-specific semantic contracts
 *          |
 *          v
 *     canonical IR where applicable
 *
 * If an exported quantum operation eventually participates in computation:
 *
 *     dialect semantic model
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *
 * The canonical `quantum::ir` boundary remains unchanged.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler should consume dialect-export semantics during:
 *
 *     name resolution;
 *     dialect loading;
 *     symbol visibility analysis;
 *     extension validation;
 *     feature-gate validation;
 *     version compatibility;
 *     capability resolution;
 *     semantic lowering.
 *
 * This grammar does not perform compiler operations.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime systems do not consume raw export syntax as executable instructions.
 *
 * Runtime-relevant meaning is obtained only after:
 *
 *     parse
 *       ->
 *     AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical representation
 *       ->
 *     compilation/lowering.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * The syntax intentionally supports symbolic names from any domain:
 *
 *     classical::
 *     quantum::
 *     hybrid::
 *     hdl::
 *     hardware::
 *     distributed::
 *     ai::
 *     data::
 *     networking::
 *     security::
 *     interoperability::
 *     future::
 *
 * These are examples, not grammar enumerations.
 *
 * Adding a new domain requires no modification to this file.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * Positive examples:
 *
 *     export quantum::operation;
 *
 *     export quantum::operation as operation;
 *
 *     export {
 *         quantum::operation,
 *         quantum::measurement
 *     };
 *
 *     export *;
 *
 *     export * as quantum;
 *
 *     export quantum::operation from quantum::base;
 *
 *     export * from quantum::base;
 *
 *     export * as quantum from quantum::base;
 *
 *     export quantum::operation from "external-dialect";
 *
 * Negative examples:
 *
 *     export;
 *
 *     export {};
 *
 *     export as operation;
 *
 *     export * as quantum::namespace;
 *
 *     export quantum::operation as quantum::alias;
 *
 *     export quantum::operation from;
 *
 *     export {quantum::operation,,quantum::measurement};
 *
 * Boundary examples:
 *
 *     export a;
 *
 *     export a::b;
 *
 *     export a::b::c::d;
 *
 *     export {
 *         a,
 *         b,
 *         c,
 *         d
 *     };
 *
 * The grammar imposes no semantic maximum on these structures.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance tests should verify that source-level cardinality is not
 * artificially restricted.
 *
 * Examples should cover:
 *
 *     one export;
 *     many exports;
 *     many aliases;
 *     deeply qualified symbolic names;
 *     large generated export groups;
 *     repeated dialect composition;
 *     many re-export declarations.
 *
 * Tests must not establish an artificial maximum as language semantics.
 *
 * If parsing fails because compiler resources are exhausted, that must be
 * diagnosed as implementation/resource exhaustion rather than a language
 * cardinality rule.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     grammar version;
 *     token stream;
 *     parser configuration;
 *
 * the resulting parse structure must be identical.
 *
 * No external environment may affect parsing.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing module-export syntax remains owned by:
 *
 *     grammar/modules/exports.g4
 *
 * This file introduces no replacement for that grammar.
 *
 * Existing dialect registration syntax remains owned by:
 *
 *     grammar/dialects/registration.g4
 *
 * Integration should add `dialectExportDeclaration` as a registration member
 * without duplicating the rules contained here.
 *
 * No existing major repository filename needs to be renamed.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors should identify the smallest useful source span.
 *
 * Recommended diagnostics include:
 *
 *     missing export target;
 *     missing closing export group;
 *     missing export specifier;
 *     invalid alias;
 *     missing re-export source;
 *     invalid wildcard alias;
 *     missing semicolon.
 *
 * Semantic diagnostics are outside this file and should distinguish:
 *
 *     unknown exported symbol;
 *     unknown dialect;
 *     duplicate export;
 *     alias collision;
 *     invalid re-export;
 *     incompatible dialect;
 *     unavailable capability;
 *     forbidden extension.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar must avoid:
 *
 *     unnecessary ambiguity;
 *     duplicate alternatives;
 *     hidden semantic predicates;
 *     recursive expansion of wildcard exports;
 *     source lookup during parsing.
 *
 * Wildcard expansion is semantic, not syntactic.
 *
 * The parser must therefore remain independent of the number of symbols
 * eventually exported by a wildcard.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * No export source may trigger:
 *
 *     file access;
 *     network access;
 *     dependency installation;
 *     command execution;
 *     plugin execution;
 *     credential access;
 *     environment inspection.
 *
 * Strings are data at parse time.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Purpose is defined.
 * [x] Ownership is defined.
 * [x] Non-ownership is defined.
 * [x] Canonical name integration is defined.
 * [x] Module-export distinction is defined.
 * [x] Dialect-registration integration is defined.
 * [x] Direct exports are supported.
 * [x] Aliased exports are supported.
 * [x] Named export groups are supported.
 * [x] Wildcard exports are supported.
 * [x] Re-exports are supported.
 * [x] Wildcard re-exports are supported.
 * [x] Namespace wildcard aliases are supported.
 * [x] Symbolic and literal sources are preserved.
 * [x] No dialect enumeration exists.
 * [x] No hardware limits exist.
 * [x] No resource capacities are hard-coded.
 * [x] No backend selection exists.
 * [x] No quantum gate enumeration exists.
 * [x] No second quantum IR exists.
 * [x] No embedded Rust exists.
 * [x] No unsafe code exists.
 * [x] AST contract is defined.
 * [x] Semantic contract is defined.
 * [x] IR boundary is defined.
 * [x] Compiler integration is defined.
 * [x] Runtime boundary is defined.
 * [x] Diagnostics are defined.
 * [x] Security boundary is defined.
 * [x] Determinism requirements are defined.
 * [x] Scalability requirements are defined.
 * [x] Compatibility requirements are defined.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file defines HOW A DIALECT EXPOSES ITS PUBLIC SOURCE-LEVEL SURFACE.
 *
 * It does not define:
 *
 *     what hardware exists;
 *     which target is selected;
 *     how resources are allocated;
 *     how quantum operations are routed;
 *     how schedules are generated;
 *     how QEC is implemented;
 *     how ZQN is executed;
 *     how a runtime executes code.
 *
 * The final direction remains:
 *
 *     DIALECT EXPORT
 *          |
 *          v
 *     DOMAIN-NEUTRAL AST
 *          |
 *          v
 *     SEMANTIC DIALECT MODEL
 *          |
 *          v
 *     CANONICAL SEMANTICS
 *          |
 *          +----------------------+
 *          |          |           |
 *          v          v           v
 *      CLASSICAL   quantum::ir   HDL/HARDWARE
 *          |          |           |
 *          +----------+-----------+
 *                     |
 *                     v
 *              OPTIMIZATION
 *                     |
 *              ROUTING / SCHEDULING
 *                     |
 *               QEC / ZQN
 *                     |
 *                    HAL
 *                     |
 *                     v
 *                TARGETS
 *
 * This preserves the Zamani single-language, open-world, target-independent
 * and POCO-REAF architecture.
 *
 * ============================================================================
 */