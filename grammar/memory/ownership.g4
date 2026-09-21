/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/memory/ownership.g4
 *
 * STATUS
 * ------
 * Production ownership-domain parser component.
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021 Edition
 * Safe Rust only
 * No unsafe implementation requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the reusable SOURCE-LEVEL SYNTAX for ownership intent.
 *
 * It does not implement ownership semantics.
 *
 * The architectural boundary is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     parser grammar
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic ownership analysis
 *       |
 *       +------------------------------+
 *       |                              |
 *       v                              v
 *     type analysis              lifetime analysis
 *       |                              |
 *       +---------------+--------------+
 *                       |
 *                       v
 *                canonical semantic model
 *                       |
 *                       v
 *                  canonical IR
 *                       |
 *             +---------+----------+
 *             |         |          |
 *             v         v          v
 *          classical  quantum    hardware
 *             |         |          |
 *             +---------+----------+
 *                       |
 *                       v
 *             optimization / lowering
 *                       |
 *                       v
 *                    runtime
 *
 * Ownership syntax is therefore deliberately independent from:
 *
 *     allocation
 *     deallocation
 *     borrowing
 *     lifetime inference
 *     memory placement
 *     physical addresses
 *     resource discovery
 *     hardware selection
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative language specification:
 *
 *     grammar/specification/
 *
 * Memory-domain architecture:
 *
 *     grammar/memory/README.md
 *
 * Canonical combined root:
 *
 *     grammar/Zamani.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Frontend implementation:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *     src/frontend/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 * 1. Ownership modes.
 *
 * 2. Ownership qualifiers.
 *
 * 3. Ownership annotations.
 *
 * 4. Ownership policy references.
 *
 * 5. Ownership requirements.
 *
 * 6. Ownership constraints.
 *
 * 7. Ownership preferences.
 *
 * 8. Ownership hints.
 *
 * 9. Ownership transfer intent syntax.
 *
 * 10. Ownership consumption intent syntax.
 *
 * 11. Ownership place references where a place bridge is required.
 *
 * 12. Open-world ownership-domain names.
 *
 * 13. Ownership-domain directive syntax.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 * It does NOT define:
 *
 * - general identifiers;
 * - general names;
 * - general paths;
 * - expressions;
 * - types;
 * - reference types;
 * - pointer types;
 * - borrow syntax;
 * - lifetime syntax;
 * - allocation;
 * - deallocation;
 * - memory spaces;
 * - memory regions;
 * - memory placement;
 * - resource discovery;
 * - hardware topology;
 * - quantum allocation;
 * - qubit allocation;
 * - scheduling;
 * - routing;
 * - optimization;
 * - QEC;
 * - ZQN;
 * - HAL;
 * - runtime behavior.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Ownership is a SEMANTIC PROPERTY.
 *
 * This file preserves ownership intent in syntax.
 *
 * It must never turn a semantic ownership concept into:
 *
 *     a physical address;
 *     a machine resource;
 *     a hardware identifier;
 *     a register;
 *     a device;
 *     a fixed-size storage object;
 *     a compiler implementation decision.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Nothing in this grammar imposes a universal limit on:
 *
 *     values
 *     owners
 *     references
 *     places
 *     regions
 *     lifetimes
 *     allocations
 *     objects
 *     tasks
 *     processes
 *     devices
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     nodes
 *     memory
 *     storage
 *
 * In particular, this file MUST NOT define:
 *
 *     MAX_OWNERS
 *     MAX_BORROWS
 *     MAX_REFERENCES
 *     MAX_REGIONS
 *     MAX_LIFETIMES
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * A numeric literal in a Zamani program remains program data.
 *
 * A resource limit is a downstream property of:
 *
 *     target capabilities
 *     resource analysis
 *     compilation
 *     scheduling
 *     deployment
 *     runtime availability
 *
 * ============================================================================
 * SEMANTIC DISTINCTIONS
 * ============================================================================
 *
 * The language must distinguish:
 *
 *     ownership requirement
 *     ownership constraint
 *     ownership preference
 *     ownership hint
 *
 * Requirement:
 *
 *     mandatory semantic property.
 *
 * Constraint:
 *
 *     restriction on legal realization.
 *
 * Preference:
 *
 *     desired but non-mandatory property.
 *
 * Hint:
 *
 *     optional implementation guidance which must not change semantics.
 *
 * This grammar preserves those distinctions.
 *
 * ============================================================================
 * OWNERSHIP MODES
 * ============================================================================
 *
 * `linear` and `affine` are foundational ownership modes.
 *
 * Their semantic definitions belong to semantic analysis.
 *
 * Conceptually:
 *
 *     linear
 *         a value has consumption-sensitive ownership semantics.
 *
 *     affine
 *         a value may be consumed at most according to its affine rules.
 *
 * The grammar does not implement either rule.
 *
 * Future ownership systems must not require physical-machine assumptions.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file declares NO lexer rules.
 *
 * The canonical lexer remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar should consume canonical lexer tokens for:
 *
 *     identifiers
 *     punctuation
 *     literals
 *     ownership keywords
 *
 * where such tokens are formally part of the Zamani lexical specification.
 *
 * Open-world extension names use IDENTIFIER-based syntax so that adding a
 * future ownership policy does not require continuously expanding the global
 * keyword inventory.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It intentionally does NOT declare:
 *
 *     grammar Zamani;
 *
 * It intentionally does NOT declare a complete program rule.
 *
 * It is consumed by the memory/parser composition hierarchy.
 *
 * The canonical integration direction is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Ownership
 *          |
 *          v
 *     Memory parser composition
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     frontend AST
 *
 * This file must not be imported directly by the final root if the repository's
 * established parser hierarchy already imports memory.g4.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Allowed:
 *
 *     ownership.g4
 *          |
 *          +--> canonical lexer vocabulary
 *          |
 *          +--> host parser rules through composition
 *
 * Downstream:
 *
 *     ownership syntax
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic ownership analysis
 *          |
 *          v
 *     canonical semantic model / IR
 *
 * Forbidden:
 *
 *     ownership.g4 -> runtime
 *     ownership.g4 -> hardware
 *     ownership.g4 -> scheduler
 *     ownership.g4 -> routing
 *     ownership.g4 -> QEC
 *     ownership.g4 -> ZQN
 *     ownership.g4 -> HAL
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough information for the domain-neutral AST to
 * represent:
 *
 *     ownership mode
 *     ownership qualifier
 *     ownership policy
 *     ownership subject
 *     ownership transfer
 *     ownership requirement
 *     ownership constraint
 *     ownership preference
 *     ownership hint
 *     ownership directive
 *     source span
 *
 * The AST MUST NOT require:
 *
 *     physical memory address
 *     hardware identifier
 *     CPU identifier
 *     GPU identifier
 *     QPU identifier
 *     physical qubit identifier
 *     device-local resource index
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     whether ownership is valid;
 *     whether a value can be moved;
 *     whether a value can be consumed;
 *     whether duplication is legal;
 *     whether dropping is legal;
 *     whether an ownership transfer is legal;
 *     whether an ownership requirement is satisfied;
 *     whether a constraint is satisfiable;
 *     whether a preference can be honored;
 *     whether a hint is applicable.
 *
 * None of those questions should be answered by this grammar.
 *
 * ============================================================================
 * PLACE CONTRACT
 * ============================================================================
 *
 * Ownership often applies to a SOURCE-LEVEL PLACE.
 *
 * A place is not equivalent to an arbitrary expression.
 *
 * Examples:
 *
 *     value
 *     object.field
 *     object.field.other
 *
 * Indexing and more complex place expressions remain owned by the canonical
 * expression/place grammar.
 *
 * This file therefore provides a MINIMAL PLACE BRIDGE rather than recreating
 * the complete expression grammar.
 *
 * The host parser may map this bridge directly to its canonical place AST.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSION PRINCIPLE
 * ============================================================================
 *
 * Ownership must remain extensible.
 *
 * Examples of possible future semantic namespaces include:
 *
 *     ownership::unique
 *     ownership::shared
 *     ownership::region
 *     ownership::capability
 *     ownership::distributed
 *     ownership::persistent
 *     ownership::linear
 *
 * This grammar does not need a separate parser rule for each future concept.
 *
 * The name is syntax.
 *
 * Its meaning belongs to semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar Ownership;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * OWNERSHIP MODE
 * ============================================================================
 *
 * Foundational ownership modes.
 *
 * These rules intentionally use canonical lexical tokens rather than quoted
 * strings so that ownership syntax has one lexical authority.
 */

ownershipMode
    : LINEAR
    | AFFINE
    ;


/*
 * Optional ownership qualifier.
 *
 * The host declaration grammar determines where this qualifier may occur.
 */

ownershipQualifier
    : ownershipMode
    ;


optionalOwnershipQualifier
    : ownershipQualifier?
    ;


/*
 * ============================================================================
 * OWNERSHIP ANNOTATIONS
 * ============================================================================
 *
 * Annotation punctuation is consumed through the canonical lexer vocabulary.
 *
 * The annotation name is intentionally represented by IDENTIFIER-based
 * syntax rather than creating a second annotation system.
 */

ownershipAnnotation
    : AT ownershipAnnotationName
      LPAREN ownershipAnnotationValue RPAREN
    ;


ownershipAnnotationName
    : IDENTIFIER
    ;


ownershipAnnotationValue
    : ownershipMode
    | ownershipQualifiedName
    ;


/*
 * ============================================================================
 * OPEN-WORLD OWNERSHIP NAMES
 * ============================================================================
 *
 * Examples:
 *
 *     unique
 *     shared
 *     region::scoped
 *     capability::restricted
 *
 * Qualification depth is not bounded.
 */

ownershipName
    : IDENTIFIER
    ;


ownershipQualifiedName
    : ownershipName
      (
          DOUBLE_COLON
          ownershipName
      )*
    ;


/*
 * ============================================================================
 * OWNERSHIP SUBJECT
 * ============================================================================
 *
 * A subject is the root of an ownership place.
 *
 * General expression syntax remains outside this grammar.
 */

ownershipSubject
    : IDENTIFIER
    | SELF
    | THIS
    ;


/*
 * ============================================================================
 * OWNERSHIP PLACE
 * ============================================================================
 *
 * Minimal source-level place bridge.
 *
 * Field projection belongs here because ownership frequently attaches to
 * object fields without requiring the ownership grammar to own the complete
 * expression language.
 *
 * Indexing is deliberately NOT reconstructed here. The canonical expression
 * and place grammar owns arbitrary indexing syntax.
 */

ownershipPlace
    : ownershipSubject
      ownershipFieldProjection*
    ;


ownershipFieldProjection
    : DOT
      IDENTIFIER
    ;


/*
 * ============================================================================
 * OWNERSHIP QUALIFIED SUBJECT
 * ============================================================================
 *
 * A qualified name can be used when the semantic ownership domain refers to a
 * named resource/value through a namespace.
 *
 * This is intentionally distinct from ownershipPlace.
 */

ownershipQualifiedSubject
    : ownershipQualifiedName
    ;


/*
 * ============================================================================
 * OWNERSHIP REFERENCE
 * ============================================================================
 *
 * Reusable source-level ownership reference.
 *
 * It may be either:
 *
 *     a local/source place
 *
 * or:
 *
 *     a qualified ownership-domain name.
 */

ownershipReference
    : ownershipPlace
    | ownershipQualifiedSubject
    ;


/*
 * ============================================================================
 * OWNERSHIP DECLARATION QUALIFIER
 * ============================================================================
 *
 * Declaration grammar remains responsible for the declaration itself.
 *
 * This rule provides only the reusable ownership prefix.
 */

ownershipDeclarationQualifier
    : ownershipQualifier
    ;


ownershipBindingPrefix
    : ownershipDeclarationQualifier
    ;


/*
 * ============================================================================
 * OWNERSHIP POLICY
 * ============================================================================
 *
 * Policy names are semantic identifiers.
 *
 * They do not select an allocator, device, memory bank, or hardware target.
 */

ownershipPolicy
    : ownershipQualifiedName
    ;


ownershipPolicyClause
    : ownershipPolicyKeyword
      LPAREN
      ownershipPolicy
      RPAREN
    ;


ownershipPolicyKeyword
    : OWNERSHIP
    ;


/*
 * ============================================================================
 * OWNERSHIP REQUIREMENT
 * ============================================================================
 *
 * Requirement is mandatory semantic intent.
 */

ownershipRequirement
    : ownershipRequirementKeyword
      LPAREN
      ownershipRequirementValue
      RPAREN
    ;


ownershipRequirementKeyword
    : REQUIRES
    ;


ownershipRequirementValue
    : ownershipMode
    | ownershipQualifiedName
    ;


/*
 * ============================================================================
 * OWNERSHIP CONSTRAINT
 * ============================================================================
 *
 * Constraint restricts legal semantic realization.
 */

ownershipConstraint
    : ownershipConstraintKeyword
      LPAREN
      ownershipConstraintValue
      RPAREN
    ;


ownershipConstraintKeyword
    : CONSTRAINT
    ;


ownershipConstraintValue
    : ownershipMode
    | ownershipQualifiedName
    ;


/*
 * ============================================================================
 * OWNERSHIP PREFERENCE
 * ============================================================================
 *
 * Preference is non-mandatory.
 */

ownershipPreference
    : ownershipPreferenceKeyword
      LPAREN
      ownershipPreferenceValue
      RPAREN
    ;


ownershipPreferenceKeyword
    : PREFER
    ;


ownershipPreferenceValue
    : ownershipMode
    | ownershipQualifiedName
    ;


/*
 * ============================================================================
 * OWNERSHIP HINT
 * ============================================================================
 *
 * Hint cannot alter program semantics.
 */

ownershipHint
    : ownershipHintKeyword
      LPAREN
      ownershipHintValue
      RPAREN
    ;


ownershipHintKeyword
    : HINT
    ;


ownershipHintValue
    : ownershipMode
    | ownershipQualifiedName
    ;


/*
 * ============================================================================
 * OWNERSHIP TRANSFER
 * ============================================================================
 *
 * Transfer is SOURCE INTENT.
 *
 * It does not perform a runtime move during parsing.
 *
 * The semantic layer determines whether transfer is legal.
 */

ownershipTransfer
    : ownershipTransferKeyword
      LPAREN
      ownershipReference
      (
          COMMA
          ownershipTransferArgument
      )*
      RPAREN
    ;


ownershipTransferKeyword
    : MOVE
    | TRANSFER
    ;


ownershipTransferArgument
    : ownershipReference
    | ownershipQualifiedName
    ;


/*
 * ============================================================================
 * OWNERSHIP CONSUMPTION
 * ============================================================================
 *
 * Consumption intent is separate from general transfer.
 *
 * Semantic analysis determines whether consumption is legal.
 */

ownershipConsume
    : ownershipConsumeKeyword
      LPAREN
      ownershipReference
      RPAREN
    ;


ownershipConsumeKeyword
    : CONSUME
    ;


/*
 * ============================================================================
 * OWNERSHIP COPY / DUPLICATION INTENT
 * ============================================================================
 *
 * The syntax does not declare that a value is semantically copyable.
 *
 * Type and ownership analysis determine whether the operation is legal.
 */

ownershipCopy
    : ownershipCopyKeyword
      LPAREN
      ownershipReference
      RPAREN
    ;


ownershipCopyKeyword
    : COPY
    ;


/*
 * ============================================================================
 * OWNERSHIP DROP / RELEASE INTENT
 * ============================================================================
 *
 * Dropping ownership is semantic intent.
 *
 * It is not the same as physical memory deallocation.
 *
 * Deallocation belongs to allocation/deallocation grammar.
 */

ownershipDrop
    : ownershipDropKeyword
      LPAREN
      ownershipReference
      RPAREN
    ;


ownershipDropKeyword
    : DROP
    ;


/*
 * ============================================================================
 * OWNERSHIP RELATION
 * ============================================================================
 *
 * Generic relation syntax.
 *
 * Examples:
 *
 *     ownership(x)
 *     ownership(x, y)
 *
 * Interpretation remains semantic.
 */

ownershipRelation
    : ownershipRelationKeyword
      LPAREN
      ownershipReference
      (
          COMMA
          ownershipReference
      )*
      RPAREN
    ;


ownershipRelationKeyword
    : OWNERSHIP
    ;


/*
 * ============================================================================
 * OWNERSHIP DIRECTIVE
 * ============================================================================
 *
 * Generic extensibility point.
 *
 * The directive name is open-world.
 *
 * Examples:
 *
 *     ownership::unique(...)
 *     ownership::region::scoped(...)
 *
 * The first namespace component is syntactic ownership-domain identity.
 */

ownershipDirective
    : OWNERSHIP
      DOUBLE_COLON
      ownershipDirectiveName
      LPAREN
      ownershipDirectiveArguments?
      RPAREN
    ;


ownershipDirectiveName
    : ownershipName
    ;


ownershipDirectiveArguments
    : ownershipDirectiveArgument
      (
          COMMA
          ownershipDirectiveArgument
      )*
    ;


ownershipDirectiveArgument
    : ownershipReference
    | ownershipMode
    | ownershipQualifiedName
    ;


/*
 * ============================================================================
 * OWNERSHIP SPECIFIER
 * ============================================================================
 *
 * This is the reusable union consumed by the memory-domain composition layer.
 *
 * It deliberately contains ownership constructs only.
 */

ownershipConstruct
    : ownershipQualifier
    | ownershipAnnotation
    | ownershipPolicyClause
    | ownershipRequirement
    | ownershipConstraint
    | ownershipPreference
    | ownershipHint
    | ownershipTransfer
    | ownershipConsume
    | ownershipCopy
    | ownershipDrop
    | ownershipRelation
    | ownershipDirective
    ;


/*
 * ============================================================================
 * OWNERSHIP METADATA
 * ============================================================================
 *
 * Metadata remains symbolic.
 *
 * It does not encode implementation limits.
 */

ownershipMetadata
    : ownershipQualifiedName
      (
          ASSIGN
          ownershipMetadataValue
      )?
    ;


ownershipMetadataValue
    : ownershipMode
    | ownershipQualifiedName
    | ownershipReference
    ;


ownershipMetadataList
    : ownershipMetadata
      (
          COMMA
          ownershipMetadata
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * OWNERSHIP POLICY DECLARATION
 * ============================================================================
 *
 * A policy declaration carries source-level policy intent only.
 *
 * The host declaration grammar decides where declarations are legal.
 */

ownershipPolicyDeclaration
    : ownershipPolicyDeclarationKeyword
      ownershipQualifiedName
      (
          LPAREN
          ownershipMetadataList?
          RPAREN
      )?
    ;


ownershipPolicyDeclarationKeyword
    : OWNERSHIP
    ;


/*
 * ============================================================================
 * OWNERSHIP CAPABILITY REFERENCE
 * ============================================================================
 *
 * This is deliberately a NAME, not a hardware capability implementation.
 *
 * Examples:
 *
 *     ownership::linear
 *     ownership::shared
 *     ownership::distributed
 *
 * Capability satisfaction is downstream.
 */

ownershipCapability
    : OWNERSHIP
      DOUBLE_COLON
      ownershipQualifiedName
    ;


/*
 * ============================================================================
 * OWNERSHIP REQUIREMENT LIST
 * ============================================================================
 */

ownershipRequirementList
    : ownershipRequirementValue
      (
          COMMA
          ownershipRequirementValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * OWNERSHIP CONSTRAINT LIST
 * ============================================================================
 */

ownershipConstraintList
    : ownershipConstraintValue
      (
          COMMA
          ownershipConstraintValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * OWNERSHIP PREFERENCE LIST
 * ============================================================================
 */

ownershipPreferenceList
    : ownershipPreferenceValue
      (
          COMMA
          ownershipPreferenceValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * OWNERSHIP HINT LIST
 * ============================================================================
 */

ownershipHintList
    : ownershipHintValue
      (
          COMMA
          ownershipHintValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * memory.g4
 * ----------
 *
 * memory.g4 is the memory-domain composition boundary.
 *
 * It should import/compose Ownership and expose the ownership constructs in
 * contexts where memory syntax permits ownership metadata.
 *
 *
 * declarations/*
 * --------------
 *
 * Declaration grammars own:
 *
 *     variable declarations
 *     fields
 *     parameters
 *     resources
 *     types
 *
 * They may consume:
 *
 *     ownershipDeclarationQualifier
 *     ownershipBindingPrefix
 *
 * They remain responsible for the complete declaration syntax.
 *
 *
 * types/*
 * -------
 *
 * Type grammars own:
 *
 *     typeExpression
 *     referenceType
 *     pointerType
 *     genericType
 *     arrayType
 *     resourceType
 *
 * Ownership grammar MUST NOT redefine those types.
 *
 *
 * borrowing.g4
 * ------------
 *
 * Borrowing owns:
 *
 *     borrow syntax
 *     mutable borrow syntax
 *     borrow-specific lifetime references
 *
 * Ownership grammar does not redefine borrow syntax.
 *
 *
 * lifetimes.g4
 * ------------
 *
 * Lifetime grammar owns canonical lifetime syntax where the repository's
 * language specification establishes it.
 *
 * Ownership grammar must not turn lifetime names into machine timers.
 *
 *
 * allocation.g4
 * -------------
 *
 * Allocation owns:
 *
 *     allocate
 *     reserve
 *     resize
 *     allocation policies
 *
 * Ownership transfer MUST NOT imply allocation.
 *
 *
 * deallocation.g4
 * ---------------
 *
 * Deallocation owns physical/logical storage release intent.
 *
 * ownershipDrop is intentionally distinct from deallocation.
 *
 *
 * resources/*
 * ------------
 *
 * Resource requirements and capabilities are target-independent semantic
 * contracts.
 *
 * Ownership grammar may refer to symbolic ownership capabilities but does not
 * discover or select physical resources.
 *
 *
 * expressions/*
 * --------------
 *
 * The canonical expression grammar owns arbitrary expressions.
 *
 * ownership.g4 MUST NOT reproduce:
 *
 *     arithmetic
 *     indexing
 *     calls
 *     ranges
 *     conditionals
 *     lambdas
 *     comprehensions
 *     tensor expressions
 *     quantum expressions
 *
 * Ownership places are intentionally restricted to the source-level bridge
 * defined above.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Ownership syntax is domain-neutral.
 *
 * The same ownership model may apply to:
 *
 *     classical values
 *     quantum semantic objects
 *     hybrid values
 *     HDL resources
 *     accelerator buffers
 *     distributed objects
 *     tensors
 *     datasets
 *     network resources
 *     security-sensitive objects
 *     future computational resources
 *
 * The grammar must not create separate ownership languages for each domain.
 *
 * Domain-specific ownership semantics are downstream semantic concerns.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Ownership syntax must never create a second quantum IR.
 *
 * A quantum source construct eventually follows:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
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
 *     QEC / resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * Ownership participates in semantic validation; it does not own any of
 * those downstream stages.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Ownership cannot identify:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     memory bank 0
 *     physical qubit 0
 *     hardware address
 *
 * Such information belongs to target realization.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Ownership requirements may participate in the resource/capability system,
 * but the grammar only preserves symbolic intent.
 *
 * The distinction is:
 *
 *     source ownership requirement
 *              |
 *              v
 *       semantic requirement
 *              |
 *              v
 *       capability/resource analysis
 *              |
 *              v
 *       target realization
 *
 * The parser does not determine whether a machine satisfies a requirement.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     grammar version
 *     lexer vocabulary
 *     parser configuration
 *     explicitly selected dialect configuration
 *
 * Parsing must not depend on:
 *
 *     hardware availability
 *     memory capacity
 *     CPU count
 *     GPU count
 *     QPU count
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     environment variables
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify structural errors such as:
 *
 *     malformed ownership qualifier
 *     malformed ownership annotation
 *     malformed ownership policy
 *     malformed ownership directive
 *     missing ownership subject
 *     malformed ownership transfer
 *     malformed ownership argument
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     illegal move
 *     illegal duplication
 *     illegal drop
 *     invalid ownership transfer
 *     unsatisfied ownership requirement
 *     violated ownership constraint
 *     unavailable preferred ownership model
 *
 * The parser must not convert semantic failures into syntax errors merely
 * because they require program-wide analysis.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded target-language actions.
 *
 * It performs no:
 *
 *     filesystem access
 *     network access
 *     hardware access
 *     environment inspection
 *     secret access
 *     allocation
 *     runtime execution
 *
 * Rust integration must remain:
 *
 *     Rust 1.97 / 1.97.1
 *     Edition 2021
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The ownership component requires independent conformance tests.
 *
 * --------------------------------------------------------------------------
 * POSITIVE
 * --------------------------------------------------------------------------
 *
 * Ownership modes:
 *
 *     linear
 *     affine
 *
 * Qualifiers:
 *
 *     linear
 *     affine
 *
 * Qualified names:
 *
 *     unique
 *     shared
 *     region::scoped
 *     capability::restricted
 *     domain::ownership::policy
 *
 * Ownership references:
 *
 *     value
 *     self
 *     this
 *     object.field
 *     object.field.value
 *
 * Transfer:
 *
 *     move(value)
 *     transfer(value)
 *
 * Consumption:
 *
 *     consume(value)
 *
 * Copy intent:
 *
 *     copy(value)
 *
 * Drop intent:
 *
 *     drop(value)
 *
 * --------------------------------------------------------------------------
 * NEGATIVE
 * --------------------------------------------------------------------------
 *
 * Must reject malformed structures such as:
 *
 *     linear(
 *     affine(
 *     ownership::
 *     ownership:::
 *     ::ownership
 *     ownership::policy(
 *     move(
 *     consume(
 *     copy(
 *     drop(
 *     object.
 *     .
 *
 * The exact diagnostics are owned by the frontend diagnostic layer.
 *
 * --------------------------------------------------------------------------
 * BOUNDARY
 * --------------------------------------------------------------------------
 *
 * Tests must include:
 *
 *     deeply qualified ownership names;
 *     deeply projected ownership places;
 *     many ownership metadata entries;
 *     many ownership directives;
 *     large programs;
 *     dynamically sized values;
 *     symbolic resource quantities;
 *
 * No artificial upper bound may be encoded by the grammar.
 *
 * --------------------------------------------------------------------------
 * SCALABILITY
 * --------------------------------------------------------------------------
 *
 * The grammar must remain valid when ownership applies to:
 *
 *     tiny values;
 *     large aggregates;
 *     tensors;
 *     distributed objects;
 *     accelerator buffers;
 *     quantum-associated semantic objects;
 *     HDL resources;
 *     future computational resources.
 *
 * Resource realization is downstream.
 *
 * --------------------------------------------------------------------------
 * CROSS-DOMAIN
 * --------------------------------------------------------------------------
 *
 * Ownership syntax must remain usable with:
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
 *     future domains
 *
 * --------------------------------------------------------------------------
 * DETERMINISM
 * --------------------------------------------------------------------------
 *
 * Identical source, grammar version, and lexical configuration must yield
 * identical parse structure.
 *
 * --------------------------------------------------------------------------
 * ROUND-TRIP
 * --------------------------------------------------------------------------
 *
 * A future formatter/printer must preserve:
 *
 *     ownership mode
 *     ownership qualifier
 *     ownership annotation
 *     policy
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     transfer
 *     consumption
 *     copy
 *     drop
 *     directive
 *     source-level ownership reference
 *
 * without changing semantic intent.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Repository validation must reject ownership grammar additions that introduce
 * machine-dependent limits, including but not limited to:
 *
 *     MAX_OWNERS
 *     MAX_REFERENCES
 *     MAX_LIFETIMES
 *     MAX_REGIONS
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *
 * The presence of a numeric literal in program syntax is not itself a
 * violation. The violation occurs when the grammar turns such a value into a
 * universal implementation limit.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * ownership.g4 is complete when:
 *
 * [ ] It is a valid ANTLR4 parser grammar.
 *
 * [ ] It consumes the canonical Zamani lexer vocabulary.
 *
 * [ ] It defines no lexer rules.
 *
 * [ ] It defines no program root.
 *
 * [ ] It does not duplicate the general expression grammar.
 *
 * [ ] It does not duplicate the type grammar.
 *
 * [ ] It does not duplicate borrow grammar.
 *
 * [ ] It does not duplicate lifetime semantics.
 *
 * [ ] It does not duplicate allocation/deallocation semantics.
 *
 * [ ] Ownership modes are represented independently of physical resources.
 *
 * [ ] Ownership policies are open-world.
 *
 * [ ] Requirements, constraints, preferences, and hints remain distinct.
 *
 * [ ] Ownership transfer is represented as semantic intent.
 *
 * [ ] Consumption is distinct from physical deallocation.
 *
 * [ ] Copy intent does not imply semantic copyability.
 *
 * [ ] Ownership places do not recreate arbitrary expression syntax.
 *
 * [ ] No fixed hardware/resource limit exists.
 *
 * [ ] No physical address exists in the grammar.
 *
 * [ ] No hardware identifier exists in the grammar.
 *
 * [ ] No quantum gate or qubit enumeration exists in the grammar.
 *
 * [ ] No second quantum IR is introduced.
 *
 * [ ] The canonical domain-neutral AST remains the next architectural stage.
 *
 * [ ] Semantic ownership checking remains downstream.
 *
 * [ ] Resource/capability analysis remains downstream.
 *
 * [ ] Compiler lowering remains downstream.
 *
 * [ ] Runtime realization remains downstream.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Round-trip tests exist.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] Rust integration remains compatible with Rust 1.97 / 1.97.1 and
 *     requires no unsafe Rust.
 *
 * ============================================================================
 */