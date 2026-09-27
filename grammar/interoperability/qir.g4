/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/qir.g4
 *
 * Grammar:
 *     Qir
 *
 * Status:
 *     CANONICAL QIR INTEROPERABILITY CONTRACT GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical Zamani SOURCE-LEVEL GRAMMAR DELEGATE for QIR
 * interoperability.
 *
 * It does NOT implement the QIR specification itself.
 *
 * It does NOT parse arbitrary LLVM IR.
 *
 * It does NOT create a QIR-specific semantic IR.
 *
 * It does NOT replace:
 *
 *     quantum::ir
 *
 * The ownership boundary is:
 *
 *     Zamani source
 *          |
 *          v
 *     Zamani lexer
 *          |
 *          v
 *     Zamani parser
 *          |
 *          v
 *     QIR interoperability contract
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic interoperability validation
 *          |
 *          v
 *     canonical quantum::ir
 *          |
 *          v
 *     QIR representability analysis
 *          |
 *          v
 *     QIR exporter/importer
 *          |
 *          v
 *     LLVM/QIR representation
 *
 * For imported QIR:
 *
 *     QIR / LLVM representation
 *          |
 *          v
 *     QIR-specific external parser
 *          |
 *          v
 *     QIR format representation
 *          |
 *          v
 *     QIR validation
 *          |
 *          v
 *     canonical quantum::ir
 *
 * The external QIR representation never becomes Zamani's canonical IR.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * QIR is an interoperability/target representation.
 *
 * It is NOT:
 *
 *     - Zamani source syntax;
 *     - Zamani AST;
 *     - Zamani semantic IR;
 *     - canonical quantum IR;
 *     - a hardware description;
 *     - a QPU topology description;
 *     - a routing representation;
 *     - a scheduling representation;
 *     - a QEC representation;
 *     - a ZQN representation;
 *     - a runtime implementation.
 *
 * The canonical Zamani quantum boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - QIR interoperability declarations;
 *     - QIR source/import/export intent;
 *     - QIR format identity;
 *     - QIR version requirements;
 *     - QIR profile requirements;
 *     - QIR capability requirements;
 *     - QIR resource requirements;
 *     - QIR compatibility requirements;
 *     - QIR representability contracts;
 *     - QIR provenance metadata;
 *     - QIR entry-point intent;
 *     - QIR output-contract intent;
 *     - QIR dynamic-resource capability declarations;
 *     - QIR semantic-preservation policy;
 *     - QIR conversion policy;
 *     - QIR format-specific attributes;
 *     - QIR target-independent interoperability metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - LLVM lexical syntax;
 *     - LLVM IR grammar;
 *     - QIR binary/bitcode parsing;
 *     - QIR runtime implementation;
 *     - QIR QIS implementation;
 *     - QIR gate-set enumeration;
 *     - quantum operation semantics;
 *     - quantum type semantics;
 *     - qubit allocation;
 *     - physical qubit identifiers;
 *     - hardware topology;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - provider APIs;
 *     - device discovery;
 *     - hardware execution;
 *     - canonical quantum::ir.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpr
 *     parameterList
 *     argumentList
 *     attribute
 *     source spans
 *     general resource semantics
 *     general capability semantics
 *     general ABI semantics
 *
 * Those belong to their canonical grammar owners.
 *
 * QIR-specific semantics are constrained to this interoperability boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * QIR interoperability MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore this grammar MUST describe:
 *
 *     WHAT QIR compatibility is required
 *
 * rather than:
 *
 *     WHICH QIR-compatible machine must execute it.
 *
 * Valid semantic intent includes concepts equivalent to:
 *
 *     requires capability("qir.base_profile")
 *     requires capability("qir.dynamic_qubit_management")
 *     requires capability("qir.adaptive")
 *     requires qubits >= n
 *     requires results >= m
 *
 * The grammar MUST NOT impose:
 *
 *     MAX_QUBITS
 *     MAX_RESULTS
 *     MAX_OPERATIONS
 *     MAX_BLOCKS
 *     MAX_FUNCTIONS
 *     MAX_MODULES
 *     MAX_PARAMETERS
 *     MAX_PROFILE_SIZE
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * or equivalent language-level limits.
 *
 * ============================================================================
 * QIR VERSIONING
 * ============================================================================
 *
 * QIR version is semantic interoperability metadata.
 *
 * The grammar intentionally does not enumerate a permanent closed list of
 * QIR versions.
 *
 * Examples may include:
 *
 *     qir version 2.0
 *
 *     qir version required_version
 *
 *     qir version "2.0"
 *
 * Concrete compatibility is checked downstream.
 *
 * This prevents the grammar from becoming obsolete merely because the QIR
 * specification gains a future compatible version.
 *
 * ============================================================================
 * QIR PROFILES
 * ============================================================================
 *
 * QIR profiles describe supported subsets/capabilities.
 *
 * Examples include:
 *
 *     base_profile
 *     adaptive
 *     future_profile
 *     vendor_profile
 *
 * The grammar does NOT enumerate them.
 *
 * Profile identity is semantic data.
 *
 * This is important because QIR profiles are intended to describe coherent
 * capability subsets rather than force every backend to support all QIR
 * functionality. QIR's official specification likewise treats profiles as
 * target capability subsets.
 *
 * ============================================================================
 * QIR GATE / INSTRUCTION POLICY
 * ============================================================================
 *
 * QIR does not become a Zamani gate grammar.
 *
 * This file MUST NOT contain:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     SWAP
 *     vendor gates
 *
 * as a closed grammar enumeration.
 *
 * QIR instruction semantics are resolved by the external QIR implementation
 * and the Zamani QIR adapter.
 *
 * Zamani quantum operations remain open-ended.
 *
 * ============================================================================
 * CANONICAL QUANTUM IR
 * ============================================================================
 *
 * The only canonical Zamani quantum semantic boundary is:
 *
 *     quantum::ir
 *
 * Therefore:
 *
 *     Zamani quantum source
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     QIR representability analysis
 *          |
 *          v
 *     QIR
 *
 * and:
 *
 *     QIR
 *          |
 *          v
 *     QIR representation
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     quantum::ir
 *
 * There must never be:
 *
 *     Zamani
 *       -> QirIR
 *       -> quantum::ir
 *
 * as a second long-lived semantic hierarchy.
 *
 * ============================================================================
 * RESOURCE MODEL
 * ============================================================================
 *
 * QIR source contracts may describe requirements such as:
 *
 *     qubits >= n
 *     results >= m
 *     capability("qir.base_profile")
 *
 * These are requirements, not hardware limits.
 *
 * A QIR Base Profile artifact may require concrete resource metadata because
 * the external QIR representation requires it.
 *
 * That fact MUST NOT become a Zamani source-language maximum.
 *
 * The distinction is:
 *
 *     QIR artifact requirement
 *             !=
 *     Zamani language limit
 *
 * ============================================================================
 * DYNAMIC RESOURCE MANAGEMENT
 * ============================================================================
 *
 * QIR can represent static and, where the applicable profile/extensions
 * permit it, dynamic resource management.
 *
 * The grammar therefore represents this as a capability/contract:
 *
 *     dynamic qubit management
 *     dynamic result management
 *
 * rather than assuming one model universally.
 *
 * ============================================================================
 * MEASUREMENT / OUTPUT
 * ============================================================================
 *
 * QIR measurement and output semantics must remain explicit.
 *
 * This grammar can declare output contracts and interoperability policy.
 *
 * It does NOT define:
 *
 *     measurement physics;
 *     probability semantics;
 *     result storage implementation;
 *     backend output transport.
 *
 * Those belong to quantum semantics, QIR validation, runtime, and target
 * adapters.
 *
 * ============================================================================
 * PROFILE REPRESENTABILITY
 * ============================================================================
 *
 * A Zamani program may be more expressive than a selected QIR profile.
 *
 * Therefore the correct pipeline is:
 *
 *     Zamani semantic program
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     profile representability analysis
 *          |
 *      +---+---+
 *      |       |
 *      v       v
 *   supported unsupported
 *      |       |
 *      v       v
 *    emit    diagnostic
 *
 * The grammar MUST NOT silently downgrade unsupported semantics.
 *
 * ============================================================================
 * LOSSLESS CONVERSION
 * ============================================================================
 *
 * QIR interoperability must distinguish:
 *
 *     lossless
 *     explicitly approximate
 *     unsupported
 *     invalid
 *
 * A conversion MUST NOT silently:
 *
 *     drop operations;
 *     drop measurements;
 *     drop classical dependencies;
 *     drop controls;
 *     change qubit ordering;
 *     change result ordering;
 *     change parameter meaning;
 *     change timing semantics;
 *     change resource requirements.
 *
 * unless the applicable semantic contract explicitly permits that behavior.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * QIR conversion should preserve provenance where supported.
 *
 * Provenance may include:
 *
 *     source span;
 *     source operation;
 *     source symbol;
 *     semantic operation;
 *     quantum::ir operation;
 *     exported QIR symbol;
 *     imported QIR symbol;
 *     QIR version;
 *     QIR profile.
 *
 * This file only declares provenance intent.
 *
 * It does not implement source maps.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing QIR interoperability syntax MUST NEVER:
 *
 *     - execute LLVM;
 *     - execute QIR;
 *     - load bitcode;
 *     - load a dynamic library;
 *     - open a network connection;
 *     - discover hardware;
 *     - submit a quantum job;
 *     - access credentials;
 *     - invoke a provider;
 *     - execute a QIS function.
 *
 * External QIR processing belongs downstream to explicitly authorized
 * compiler/adapter/runtime components.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no runtime execution.
 *
 * Parse structure therefore depends only on source text and the canonical
 * Zamani token vocabulary.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar uses repetition and symbolic expressions instead of finite
 * enumerations.
 *
 * There is no grammar-level maximum on:
 *
 *     declarations;
 *     imports;
 *     exports;
 *     profiles;
 *     capabilities;
 *     requirements;
 *     attributes;
 *     operations;
 *     parameters;
 *     qubits;
 *     results;
 *     resources;
 *     functions;
 *     modules;
 *     metadata.
 *
 * Practical implementation limits belong to compiler/runtime resource policy.
 *
 * ============================================================================
 */

parser grammar Qir;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Types,
    Expressions
;


/*
 * ============================================================================
 * 1. PUBLIC QIR ENTRY
 * ============================================================================
 *
 * This is the reusable entry point consumed by the interoperability
 * composition grammar.
 *
 * It is intentionally NOT a compilation-unit root.
 */

qirInteropItem
    : qirInteropDeclaration
    | qirImportDeclaration
    | qirExportDeclaration
    ;


/*
 * ============================================================================
 * 2. QIR INTEROPERABILITY DECLARATION
 * ============================================================================
 *
 * Canonical source-level form:
 *
 *     extern language qir {
 *         ...
 *     }
 *
 * The spelling "qir" remains an identifier/qualified name rather than a
 * permanently reserved Zamani keyword.
 *
 * This is deliberate:
 *
 *     QIR is an external format identity.
 *
 * Future external formats must not require a core lexer change merely to
 * become interoperable with Zamani.
 */

qirInteropDeclaration
    : attribute*
      EXTERN
      LANGUAGE
      qirFormatIdentity
      qirContractBody
    ;


qirFormatIdentity
    : qualifiedName
    | stringLiteral
    ;


/*
 * ============================================================================
 * 3. QIR CONTRACT BODY
 * ============================================================================
 */

qirContractBody
    : LBRACE
      qirContractItem*
      RBRACE
    ;


qirContractItem
    : qirVersionClause
    | qirProfileClause
    | qirCapabilityClause
    | qirRequirementClause
    | qirResourceClause
    | qirCompatibilityClause
    | qirRepresentabilityClause
    | qirConversionClause
    | qirEntryPointClause
    | qirOutputClause
    | qirDynamicResourceClause
    | qirProvenanceClause
    | qirAttributeClause
    ;


/*
 * ============================================================================
 * 4. VERSION
 * ============================================================================
 *
 * Examples:
 *
 *     version = 2.0;
 *     version = required_version;
 *     version = "2.0";
 *
 * The grammar does not enumerate versions.
 */

qirVersionClause
    : VERSIONING
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. PROFILE
 * ============================================================================
 *
 * Examples:
 *
 *     profile = base_profile;
 *     profile = "base_profile";
 *     profile = target_profile;
 *
 * Profile names remain semantic identifiers.
 */

qirProfileClause
    : PROFILE
      ASSIGN
      qirProfileSet
      SEMICOLON
    ;


qirProfileSet
    : qirProfile
      (COMMA qirProfile)*
    ;


qirProfile
    : qualifiedName
    | stringLiteral
    ;


/*
 * ============================================================================
 * 6. CAPABILITIES
 * ============================================================================
 *
 * Examples:
 *
 *     requires capability("qir.base_profile");
 *
 *     requires capability("qir.adaptive");
 *
 * Capability resolution is downstream.
 */

qirCapabilityClause
    : REQUIRES
      CAPABILITY
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are semantic requirements, not machine limits.
 */

qirRequirementClause
    : REQUIRES
      qirRequirementExpression
      SEMICOLON
    ;


qirRequirementExpression
    : expression
    | LBRACE
      qirRequirementList?
      RBRACE
    ;


qirRequirementList
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * 8. RESOURCE CONTRACT
 * ============================================================================
 *
 * Examples:
 *
 *     resource qubits {
 *         requires n;
 *     }
 *
 *     resource results {
 *         requires result_count;
 *     }
 *
 * The resource identity is symbolic.
 */

qirResourceClause
    : RESOURCE
      qirResourceIdentity
      qirResourceBody
    ;


qirResourceIdentity
    : qualifiedName
    | stringLiteral
    ;


qirResourceBody
    : LBRACE
      qirResourceItem*
      RBRACE
    ;


qirResourceItem
    : REQUIRES expression SEMICOLON
    | PREFER expression SEMICOLON
    | CONSTRAINT expression SEMICOLON
    | HINT expression SEMICOLON
    | CAPABILITY LPAREN expression RPAREN SEMICOLON
    | PROPERTY identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 9. COMPATIBILITY
 * ============================================================================
 */

qirCompatibilityClause
    : 'compatible'
      WITH
      qirCompatibilityTarget
      qirCompatibilityBody?
      SEMICOLON?
    ;


qirCompatibilityTarget
    : qualifiedName
    | stringLiteral
    | expression
    ;


qirCompatibilityBody
    : LBRACE
      qirCompatibilityItem*
      RBRACE
    ;


qirCompatibilityItem
    : VERSIONING ASSIGN expression SEMICOLON
    | CAPABILITY ASSIGN expression SEMICOLON
    | PROPERTY identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 10. REPRESENTABILITY
 * ============================================================================
 *
 * This declares how strictly the conversion must preserve semantics.
 */

qirRepresentabilityClause
    : 'representability'
      ASSIGN
      qirRepresentabilityMode
      SEMICOLON
    ;


qirRepresentabilityMode
    : 'lossless'
    | 'exact'
    | 'approximate'
    | 'reject'
    | qualifiedName
    | stringLiteral
    ;


/*
 * ============================================================================
 * 11. CONVERSION POLICY
 * ============================================================================
 *
 * Conversion policy is declarative.
 *
 * It does not perform conversion during parsing.
 */

qirConversionClause
    : 'conversion'
      qirConversionBody
    ;


qirConversionBody
    : LBRACE
      qirConversionItem*
      RBRACE
    ;


qirConversionItem
    : 'import' qirConversionPolicy SEMICOLON
    | 'export' qirConversionPolicy SEMICOLON
    | 'on_loss' qirLossPolicy SEMICOLON
    | PROPERTY identifier ASSIGN expression SEMICOLON
    ;


qirConversionPolicy
    : qualifiedName
    | stringLiteral
    | expression
    ;


qirLossPolicy
    : 'reject'
    | 'diagnose'
    | 'allow'
    | qualifiedName
    | stringLiteral
    ;


/*
 * ============================================================================
 * 12. ENTRY POINT
 * ============================================================================
 *
 * QIR entry-point metadata is represented semantically.
 *
 * This grammar does not require a fixed function name.
 */

qirEntryPointClause
    : 'entry'
      qirEntryPointBody
    ;


qirEntryPointBody
    : LBRACE
      qirEntryPointItem*
      RBRACE
    ;


qirEntryPointItem
    : 'name' ASSIGN expression SEMICOLON
    | 'signature' ASSIGN typeExpr SEMICOLON
    | CAPABILITY ASSIGN expression SEMICOLON
    | PROPERTY identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 13. OUTPUT CONTRACT
 * ============================================================================
 *
 * Output labels, result identity and schema are interoperability metadata.
 */

qirOutputClause
    : 'output'
      qirOutputBody
    ;


qirOutputBody
    : LBRACE
      qirOutputItem*
      RBRACE
    ;


qirOutputItem
    : 'schema' ASSIGN expression SEMICOLON
    | 'labeling' ASSIGN expression SEMICOLON
    | 'ordering' ASSIGN expression SEMICOLON
    | 'result' ASSIGN expression SEMICOLON
    | PROPERTY identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 14. DYNAMIC RESOURCE MANAGEMENT
 * ============================================================================
 *
 * These are capabilities/contracts, not assumptions.
 */

qirDynamicResourceClause
    : 'dynamic'
      qirDynamicResourceBody
    ;


qirDynamicResourceBody
    : LBRACE
      qirDynamicResourceItem*
      RBRACE
    ;


qirDynamicResourceItem
    : 'qubits' ASSIGN qirBooleanIntent SEMICOLON
    | 'results' ASSIGN qirBooleanIntent SEMICOLON
    | CAPABILITY ASSIGN expression SEMICOLON
    | PROPERTY identifier ASSIGN expression SEMICOLON
    ;


qirBooleanIntent
    : TRUE
    | FALSE
    | expression
    ;


/*
 * ============================================================================
 * 15. PROVENANCE
 * ============================================================================
 */

qirProvenanceClause
    : 'provenance'
      qirProvenanceBody
    ;


qirProvenanceBody
    : LBRACE
      qirProvenanceItem*
      RBRACE
    ;


qirProvenanceItem
    : 'source' ASSIGN expression SEMICOLON
    | 'symbol' ASSIGN expression SEMICOLON
    | VERSIONING ASSIGN expression SEMICOLON
    | PROFILE ASSIGN expression SEMICOLON
    | PROPERTY identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 16. QIR ATTRIBUTES
 * ============================================================================
 *
 * QIR/LLVM attributes are represented as semantic metadata.
 *
 * This grammar does not attempt to recreate LLVM's complete attribute
 * language.
 */

qirAttributeClause
    : PROPERTY
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. QIR IMPORT
 * ============================================================================
 *
 * Import establishes an interoperability relationship.
 *
 * It does NOT read files or execute QIR.
 *
 * Example:
 *
 *     import qir from "artifact.qir";
 *
 * The source string is metadata. Resolution belongs downstream.
 */

qirImportDeclaration
    : attribute*
      IMPORT
      qirFormatIdentity
      FROM
      qirSourceReference
      qirImportBody?
      SEMICOLON?
    ;


qirSourceReference
    : stringLiteral
    | qualifiedName
    ;


qirImportBody
    : LBRACE
      qirImportItem*
      RBRACE
    ;


qirImportItem
    : AS identifier
    | VERSIONING ASSIGN expression
    | PROFILE ASSIGN qirProfileSet
    | PROPERTY identifier ASSIGN expression
    ;


/*
 * ============================================================================
 * 18. QIR EXPORT
 * ============================================================================
 *
 * Example:
 *
 *     export qir to target;
 *
 * Actual emission occurs downstream.
 */

qirExportDeclaration
    : attribute*
      EXPORT
      qirFormatIdentity
      'to'
      qirExportTarget
      qirExportBody?
      SEMICOLON?
    ;


qirExportTarget
    : stringLiteral
    | qualifiedName
    ;


qirExportBody
    : LBRACE
      qirExportItem*
      RBRACE
    ;


qirExportItem
    : VERSIONING ASSIGN expression SEMICOLON
    | PROFILE ASSIGN qirProfileSet SEMICOLON
    | CAPABILITY ASSIGN expression SEMICOLON
    | REQUIRES expression SEMICOLON
    | PROPERTY identifier ASSIGN expression SEMICOLON
    ;


/*
 * ============================================================================
 * 19. PROFILE-SCOPED CONTRACT
 * ============================================================================
 *
 * Allows a QIR interoperability declaration to express different contracts
 * without creating separate grammar families for every QIR profile.
 */

qirProfileContract
    : PROFILE qirProfile
      LBRACE
      qirContractItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 20. PUBLIC COMPOSITION CONTRACT
 * ============================================================================
 *
 * The interoperability composition grammar should integrate:
 *
 *     qirInteropItem
 *
 * through this reusable rule.
 *
 * The QIR grammar itself does not become the universal parser root.
 */

qir
    : qirInteropItem
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the frontend AST to retain:
 *
 *     - QIR format identity;
 *     - import/export direction;
 *     - source/target identity;
 *     - version expression;
 *     - profile expressions;
 *     - capability requirements;
 *     - resource requirements;
 *     - compatibility constraints;
 *     - representability mode;
 *     - conversion policy;
 *     - entry-point metadata;
 *     - output metadata;
 *     - dynamic-resource policy;
 *     - provenance;
 *     - arbitrary interoperability attributes;
 *     - source spans.
 *
 * This grammar MUST NOT create a QIR-specific AST hierarchy that becomes
 * canonical.
 *
 * The frontend AST remains owned by:
 *
 *     src/frontend/ast/
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. Resolve the format identity.
 *     2. Verify that the requested format is QIR.
 *     3. Resolve QIR version constraints.
 *     4. Resolve profile requirements.
 *     5. Resolve capability requirements.
 *     6. Validate resource requirements.
 *     7. Validate compatibility constraints.
 *     8. Determine whether conversion can be lossless.
 *     9. Reject unsupported semantic loss unless explicitly permitted.
 *    10. Preserve source provenance.
 *    11. Normalize QIR interoperability metadata.
 *    12. Lower through canonical quantum::ir.
 *
 * Semantic analysis, not parsing, determines whether:
 *
 *     "qir"
 *
 * identifies a supported QIR representation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * The direction MUST be:
 *
 *     Zamani source
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     QIR adapter
 *
 * and for import:
 *
 *     QIR
 *          |
 *          v
 *     QIR external representation
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * This file MUST NOT introduce:
 *
 *     QirIR
 *     QirProgram
 *     QirCircuitIR
 *     QirQuantumIR
 *
 * as canonical semantic types.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The grammar integrates with the existing Rust QIR adapter:
 *
 *     src/quantum/hardware/adapters/qir.rs
 *
 * That adapter already establishes the intended direction:
 *
 *     canonical Quantum IR
 *            |
 *            v
 *     QIR adapter
 *            |
 *            v
 *     QIR v2 / LLVM representation
 *
 * Therefore this grammar MUST NOT attempt to reproduce the adapter's
 * instruction mapping.
 *
 * The compiler is responsible for:
 *
 *     - representability checking;
 *     - profile validation;
 *     - QIR version compatibility;
 *     - operation lowering;
 *     - measurement lowering;
 *     - output metadata generation;
 *     - deterministic QIR generation;
 *     - diagnostics.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime execution is outside this grammar.
 *
 * A QIR artifact may ultimately be consumed by:
 *
 *     QIR-compatible compiler;
 *     QIR-compatible backend;
 *     simulator;
 *     emulator;
 *     quantum runtime;
 *     future target.
 *
 * This grammar MUST NOT assume which one.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     QIR is LLVM-based and may carry classical interoperability metadata.
 *     Classical semantics remain owned by the classical/compiler pipeline.
 *
 * Quantum:
 *
 *     quantum::ir remains canonical.
 *
 * HDL:
 *
 *     QIR interoperability may coexist with HDL/hardware contracts but does
 *     not absorb HDL syntax.
 *
 * Hardware:
 *
 *     QIR capabilities describe requirements. Hardware discovery and target
 *     selection remain downstream.
 *
 * Distributed:
 *
 *     QIR artifacts may participate in distributed execution, but node
 *     placement remains outside this grammar.
 *
 * AI/data:
 *
 *     QIR can participate in larger hybrid programs without making QIR the
 *     semantic authority for those domains.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural errors belong to the parser.
 *
 * Examples:
 *
 *     extern language;
 *     extern language qir {
 *     import qir;
 *     export qir;
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     unsupported QIR profile;
 *     incompatible QIR version;
 *     unsupported capability;
 *     non-representable Zamani operation;
 *     forbidden semantic loss;
 *     invalid QIR resource contract.
 *
 * Resource failures must remain distinct from syntax failures.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     - QIR import;
 *     - QIR export;
 *     - QIR version;
 *     - symbolic version;
 *     - profile selection;
 *     - multiple profiles;
 *     - capability requirements;
 *     - resource requirements;
 *     - compatibility;
 *     - representability;
 *     - conversion policy;
 *     - entry-point metadata;
 *     - output metadata;
 *     - dynamic resource policy;
 *     - provenance;
 *     - arbitrary QIR properties;
 *     - qualified QIR identities;
 *     - future profile names.
 *
 * Negative tests MUST cover:
 *
 *     - incomplete declarations;
 *     - malformed import/export;
 *     - missing source/target;
 *     - malformed profile lists;
 *     - malformed resource contracts;
 *     - malformed conversion policies;
 *     - invalid attribute structure.
 *
 * Semantic negative tests MUST separately cover:
 *
 *     - unsupported QIR version;
 *     - unsupported profile;
 *     - unavailable capability;
 *     - impossible resource requirement;
 *     - non-representable quantum semantics;
 *     - forbidden lossy conversion.
 *
 * Boundary tests MUST cover:
 *
 *     - empty contract;
 *     - one profile;
 *     - many profiles;
 *     - deeply qualified format identity;
 *     - large metadata sets;
 *     - symbolic resource expressions;
 *     - large capability sets;
 *     - nested conversion metadata.
 *
 * Scalability tests MUST verify:
 *
 *     - no finite grammar limits;
 *     - no fixed profile count;
 *     - no fixed capability count;
 *     - no fixed resource count;
 *     - no fixed metadata count;
 *     - no fixed QIR operation count.
 *
 * Determinism tests MUST verify that identical source produces identical parse
 * structure and diagnostics under identical grammar/token versions.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden as universal language limits:
 *
 *     MAX_QUBITS
 *     MAX_RESULTS
 *     MAX_QIR_OPERATIONS
 *     MAX_PROFILES
 *     MAX_CAPABILITIES
 *     MAX_RESOURCES
 *     MAX_MODULES
 *     MAX_FUNCTIONS
 *
 * Also forbidden:
 *
 *     physical_qubit_0
 *     qpu_0
 *     qpu_1
 *     GPU_0
 *     CPU_0
 *     fixed_device
 *     fixed_topology
 *
 * as universal QIR language semantics.
 *
 * Program-level constants remain valid:
 *
 *     requires qubits >= n
 *     required_num_qubits = n
 *
 * when those values are part of program/interoperability semantics.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no embedded Rust.
 *
 * The Rust implementation consuming it MUST remain:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust
 *     #![deny(unsafe_code)]
 *
 * Parsing QIR must never require unsafe Rust.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete independently when:
 *
 *     [x] canonical parser-grammar boundary defined
 *     [x] canonical lexer dependency defined
 *     [x] QIR ownership defined
 *     [x] non-ownership defined
 *     [x] QIR version extensibility defined
 *     [x] profile extensibility defined
 *     [x] capability model defined
 *     [x] resource model defined
 *     [x] import/export defined
 *     [x] representability defined
 *     [x] conversion policy defined
 *     [x] entry-point metadata defined
 *     [x] output metadata defined
 *     [x] dynamic-resource policy defined
 *     [x] provenance defined
 *     [x] AST contract defined
 *     [x] semantic contract defined
 *     [x] quantum::ir contract defined
 *     [x] compiler integration defined
 *     [x] runtime integration defined
 *     [x] diagnostics defined
 *     [x] positive tests defined
 *     [x] negative tests defined
 *     [x] boundary tests defined
 *     [x] scalability tests defined
 *     [x] determinism defined
 *     [x] hard-coding audit defined
 *     [x] safe-Rust requirement defined
 *
 * The file is then ready for integration into the interoperability dispatcher.
 *
 * ============================================================================
 */