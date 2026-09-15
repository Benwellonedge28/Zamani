/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/namespaces.g4
 *
 * Role:
 *     Canonical parser grammar for namespace structure inside Zamani dialects.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     ANTLR4
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * This file owns the STRUCTURAL SYNTAX of dialect namespaces.
 *
 * It does NOT own:
 *
 *     lexical identifiers
 *     qualified-name syntax
 *     module resolution
 *     package resolution
 *     filesystem paths
 *     URLs
 *     hardware topology
 *     hardware discovery
 *     resource allocation
 *     target selection
 *     runtime execution
 *     canonical IR
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     optimization
 *     resilience
 *
 * Dependency direction:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     core::Names
 *          |
 *          v
 *     dialects::namespaces
 *          |
 *          v
 *     dialect AST
 *          |
 *          v
 *     semantic namespace resolution
 *          |
 *          +----> dialect registry
 *          +----> module/package resolution
 *          +----> capability resolution
 *          +----> extension resolution
 *          |
 *          v
 *     canonical semantic representation
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * A namespace is a symbolic organizational scope.
 *
 * A namespace MUST NOT inherently mean:
 *
 *     machine
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     physical qubit
 *     memory region
 *     network node
 *     process
 *     deployment location
 *
 * Those meanings belong to semantic/domain/target layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Namespace syntax must preserve:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 *
 * Namespace identity therefore remains symbolic and portable.
 *
 * A namespace such as:
 *
 *     quantum::algorithms
 *
 * does not imply a particular:
 *
 *     QPU
 *     qubit count
 *     topology
 *     backend
 *     scheduler
 *     runtime
 *
 * ============================================================================
 * OPEN-WORLD REQUIREMENT
 * ============================================================================
 *
 * The grammar MUST NOT enumerate known namespaces.
 *
 * Forbidden:
 *
 *     namespace
 *         : quantum
 *         | classical
 *         | hardware
 *         | gpu
 *         | ...
 *
 * Instead, namespace identities are formed from the canonical `qualifiedName`
 * rule supplied by `core/names.g4`.
 *
 * This allows:
 *
 *     quantum::...
 *     classical::...
 *     hdl::...
 *     hardware::...
 *     distributed::...
 *     ai::...
 *     future::...
 *     vendor::...
 *     organization::...
 *
 * without modifying this grammar.
 *
 * ============================================================================
 * NAME OWNERSHIP
 * ============================================================================
 *
 * `core/names.g4` owns:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *     nameList
 *     qualifiedNameList
 *
 * This file consumes those rules.
 *
 * It MUST NOT redefine them.
 *
 * ============================================================================
 * NAMESPACE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - namespace declarations;
 *     - nested namespace structure;
 *     - namespace-qualified declaration regions;
 *     - namespace imports/references at the namespace layer;
 *     - namespace aliases where explicitly supported;
 *     - namespace attributes;
 *     - namespace metadata;
 *     - namespace member boundaries;
 *     - namespace-scoped dialect structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - module semantics;
 *     - package semantics;
 *     - symbol resolution;
 *     - scope lookup;
 *     - visibility semantics;
 *     - dialect registration itself;
 *     - dialect version compatibility;
 *     - capability discovery;
 *     - hardware capabilities;
 *     - resource requirements;
 *     - physical placement;
 *     - quantum resources;
 *     - classical resources;
 *     - runtime namespaces.
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * There is intentionally no maximum for:
 *
 *     namespace depth
 *     namespace members
 *     namespace declarations
 *     aliases
 *     imports
 *     attributes
 *     metadata
 *     declarations inside namespaces
 *
 * Repetition uses `*` or `+`.
 *
 * The grammar does not define:
 *
 *     MAX_NAMESPACE_DEPTH
 *     MAX_NAMESPACES
 *     MAX_NAMESPACE_MEMBERS
 *
 * Practical limits are implementation/resource constraints only.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * No:
 *
 *     embedded Rust
 *     semantic predicates
 *     actions
 *     filesystem access
 *     network access
 *     hardware access
 *     runtime calls
 *     randomness
 *
 * are used here.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors are reported by ANTLR's parser infrastructure.
 *
 * Semantic errors such as:
 *
 *     duplicate namespace
 *     cyclic namespace relationship
 *     illegal namespace visibility
 *     unresolved namespace
 *     conflicting namespace alias
 *
 * are NOT parser errors.
 *
 * They belong to semantic analysis.
 *
 * ============================================================================
 */

parser grammar DialectNamespaces;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 1. NAMESPACE DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     namespace quantum::algorithms {
 *         ...
 *     }
 *
 * The qualified name is symbolic.
 */
dialectNamespaceDeclaration
    : NAMESPACE qualifiedName
      dialectNamespaceHeader?
      LBRACE
      dialectNamespaceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 2. NAMESPACE HEADER
 * ============================================================================
 *
 * Header metadata is intentionally separate from namespace members.
 *
 * This allows semantic tooling to preserve source structure without assigning
 * target-specific meaning to the namespace.
 */
dialectNamespaceHeader
    : dialectNamespaceHeaderItem*
    ;

dialectNamespaceHeaderItem
    : dialectNamespaceAttribute
    | dialectNamespaceMetadata
    | dialectNamespaceAlias
    ;


/*
 * ============================================================================
 * 3. NAMESPACE MEMBERS
 * ============================================================================
 *
 * A namespace may contain nested namespaces and symbolic references.
 *
 * Domain declarations remain owned by their respective grammars.
 *
 * Therefore this grammar does not duplicate:
 *
 *     functions
 *     types
 *     quantum operations
 *     HDL modules
 *     hardware declarations
 *     data declarations
 *
 * Consumers may wrap those declarations inside a namespace through the
 * higher-level dialect/program grammar.
 */
dialectNamespaceMember
    : dialectNamespaceDeclaration
    | dialectNamespaceImport
    | dialectNamespaceUse
    | dialectNamespaceAlias
    | dialectNamespaceAttribute
    | dialectNamespaceMetadata
    ;


/*
 * ============================================================================
 * 4. NAMESPACE IMPORT
 * ============================================================================
 *
 * Imports a symbolic namespace.
 *
 * Import resolution is outside this grammar.
 *
 * Importing a namespace MUST NOT perform filesystem/network/runtime actions
 * during parsing.
 */
dialectNamespaceImport
    : IMPORT qualifiedName
      dialectNamespaceImportAlias?
      SEMICOLON
    ;

dialectNamespaceImportAlias
    : AS identifier
    ;


/*
 * ============================================================================
 * 5. NAMESPACE USE
 * ============================================================================
 *
 * `use` introduces a namespace reference into the enclosing lexical context.
 *
 * It does not itself resolve a symbol.
 */
dialectNamespaceUse
    : USE qualifiedName
      dialectNamespaceUseAlias?
      SEMICOLON
    ;

dialectNamespaceUseAlias
    : AS identifier
    ;


/*
 * ============================================================================
 * 6. NAMESPACE ALIAS
 * ============================================================================
 *
 * Namespace aliases are source-level symbolic aliases.
 *
 * Example:
 *
 *     alias quantum::linear as linear;
 *
 * The exact semantic meaning of the alias is determined downstream.
 */
dialectNamespaceAlias
    : ALIAS qualifiedName
      AS identifier
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. NAMESPACE ATTRIBUTE
 * ============================================================================
 *
 * Attributes are metadata attached to a namespace.
 *
 * They remain symbolic and open-ended.
 *
 * Example:
 *
 *     @portable
 *     @experimental
 *     @domain(quantum)
 *
 * Whether an attribute is recognized is a semantic/tooling concern.
 */
dialectNamespaceAttribute
    : AT identifier
      dialectNamespaceAttributeArguments?
    ;

dialectNamespaceAttributeArguments
    : LPAREN
      dialectNamespaceValueList?
      RPAREN
    ;


/*
 * ============================================================================
 * 8. NAMESPACE METADATA
 * ============================================================================
 *
 * Metadata is deliberately open-world.
 *
 * Example:
 *
 *     metadata owner = "organization";
 *
 * New metadata properties do not require this grammar to enumerate them.
 */
dialectNamespaceMetadata
    : METADATA identifier
      dialectNamespaceMetadataValue?
      SEMICOLON
    ;

dialectNamespaceMetadataValue
    : ASSIGN dialectNamespaceValue
    ;


/*
 * ============================================================================
 * 9. NAMESPACE VALUE
 * ============================================================================
 *
 * Namespace metadata values are syntax-level values.
 *
 * Semantic interpretation belongs to the owner of the metadata key.
 */
dialectNamespaceValue
    : identifier
    | qualifiedName
    | STRING
    | INTEGER
    | FLOAT
    | TRUE
    | FALSE
    | NIL
    | NULL
    | dialectNamespaceListValue
    | dialectNamespaceMapValue
    ;


/*
 * ============================================================================
 * 10. LIST VALUE
 * ============================================================================
 */
dialectNamespaceListValue
    : LBRACKET
      dialectNamespaceValueList?
      RBRACKET
    ;

dialectNamespaceValueList
    : dialectNamespaceValue
      (COMMA dialectNamespaceValue)*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. MAP VALUE
 * ============================================================================
 *
 * Metadata maps remain syntactic.
 *
 * No machine/resource semantics are implied.
 */
dialectNamespaceMapValue
    : LBRACE
      dialectNamespaceMapEntry*
      RBRACE
    ;

dialectNamespaceMapEntry
    : identifier
      COLON
      dialectNamespaceValue
      COMMA?
    ;


/*
 * ============================================================================
 * 12. NAMESPACE PATH
 * ============================================================================
 *
 * This rule provides a named integration boundary for consumers that need to
 * distinguish a namespace path from an arbitrary qualified name in the AST.
 *
 * It intentionally delegates the actual syntax to `qualifiedName`.
 */
dialectNamespacePath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 13. NAMESPACE PATH LIST
 * ============================================================================
 */
dialectNamespacePathList
    : dialectNamespacePath
      (COMMA dialectNamespacePath)*
    ;


/*
 * ============================================================================
 * 14. OPTIONAL NAMESPACE PATH LIST
 * ============================================================================
 */
optionalDialectNamespacePathList
    : dialectNamespacePathList?
    ;


/*
 * ============================================================================
 * 15. NAMESPACE MEMBER LIST
 * ============================================================================
 */
dialectNamespaceMemberList
    : dialectNamespaceMember*
    ;


/*
 * ============================================================================
 * 16. NAMESPACE DOCUMENT
 * ============================================================================
 *
 * Useful for tooling that parses namespace-only fragments.
 *
 * The normal Zamani program parser should use its own compilation-unit entry
 * point and embed namespace declarations where appropriate.
 */
dialectNamespaceDocument
    : dialectNamespaceDeclaration*
      EOF
    ;


/*
 * ============================================================================
 * 17. NAMESPACE DECLARATION LIST
 * ============================================================================
 */
dialectNamespaceDeclarationList
    : dialectNamespaceDeclaration+
    ;


/*
 * ============================================================================
 * 18. NAMESPACE REFERENCE
 * ============================================================================
 *
 * A namespace reference is structurally just a canonical qualified name.
 */
dialectNamespaceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 19. NAMESPACE REFERENCE LIST
 * ============================================================================
 */
dialectNamespaceReferenceList
    : dialectNamespaceReference
      (COMMA dialectNamespaceReference)*
    ;


/*
 * ============================================================================
 * 20. OPTIONAL NAMESPACE REFERENCE LIST
 * ============================================================================
 */
optionalDialectNamespaceReferenceList
    : dialectNamespaceReferenceList?
    ;


/*
 * ============================================================================
 * 21. NAMESPACE QUALIFIER
 * ============================================================================
 *
 * Named wrapper for consumers that need an explicit namespace qualifier AST
 * node.
 */
dialectNamespaceQualifier
    : qualifiedName
    ;


/*
 * ============================================================================
 * 22. NAMESPACE QUALIFIER LIST
 * ============================================================================
 */
dialectNamespaceQualifierList
    : dialectNamespaceQualifier
      (COMMA dialectNamespaceQualifier)*
    ;


/*
 * ============================================================================
 * 23. NAMESPACE QUALIFIED MEMBER REFERENCE
 * ============================================================================
 *
 * This is intentionally represented by canonical qualified-name syntax.
 *
 * Example:
 *
 *     quantum::algorithms::grover
 *
 * The grammar does not determine whether the final segment represents:
 *
 *     type
 *     function
 *     extension
 *     capability
 *     operation
 *     value
 *     module
 *     namespace
 *
 * Semantic resolution determines that.
 */
dialectNamespaceQualifiedMember
    : qualifiedName
    ;


/*
 * ============================================================================
 * 24. NAMESPACE QUALIFIED MEMBER LIST
 * ============================================================================
 */
dialectNamespaceQualifiedMemberList
    : dialectNamespaceQualifiedMember
      (COMMA dialectNamespaceQualifiedMember)*
    ;


/*
 * ============================================================================
 * 25. OPTIONAL QUALIFIED MEMBER LIST
 * ============================================================================
 */
optionalDialectNamespaceQualifiedMemberList
    : dialectNamespaceQualifiedMemberList?
    ;