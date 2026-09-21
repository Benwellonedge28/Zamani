/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/memory.g4
 *
 * Grammar:
 *     Memory
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     CANONICAL MEMORY-DOMAIN FOUNDATION
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical foundational grammar for source-level memory
 * syntax in Zamani.
 *
 * It defines the stable memory vocabulary consumed by:
 *
 *     allocation.g4
 *     deallocation.g4
 *     ownership.g4
 *     borrowing.g4
 *     lifetimes.g4
 *     shared-memory.g4
 *     distributed-memory.g4
 *     memory-constraints.g4
 *
 * It is intentionally target-independent and open-ended.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani source describes:
 *
 *     WHAT memory semantics are required
 *     WHAT memory intent is expressed
 *     WHAT properties must hold
 *     WHAT capabilities are needed
 *     WHAT constraints apply
 *     WHAT preferences are desired
 *
 * It does NOT prescribe:
 *
 *     WHICH physical address
 *     WHICH memory bank
 *     WHICH NUMA node
 *     WHICH GPU
 *     WHICH accelerator
 *     WHICH QPU
 *     WHICH memory controller
 *     WHICH cache
 *     WHICH physical page
 *     WHICH allocator implementation
 *     WHICH device
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar deliberately imposes no universal limits on:
 *
 *     memory size
 *     allocation count
 *     region count
 *     lifetime count
 *     reference count
 *     memory-space count
 *     address width
 *     object count
 *     buffer count
 *     device count
 *     accelerator count
 *     distributed-node count
 *     process count
 *     task count
 *
 * There are no language constants such as:
 *
 *     MAX_MEMORY
 *     MAX_ALLOCATIONS
 *     MAX_REGIONS
 *     MAX_REFERENCES
 *     MAX_LIFETIMES
 *     MAX_MEMORY_SPACES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_ADDRESS_BITS
 *
 * Practical implementation limits remain implementation/resource policy.
 *
 * They must never silently become language-level semantic limits.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Lexer authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Lexical composition:
 *
 *     grammar/lexer/
 *
 * Canonical names:
 *
 *     grammar/core/names.g4
 *
 * Canonical expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Canonical types:
 *
 *     grammar/types/types.g4
 *
 * Universal resources:
 *
 *     grammar/resources/
 *
 * Memory specialization:
 *
 *     grammar/memory/
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The root parser already composes Memory together with the universal
 * expression, type, core, resource, and domain grammars.
 *
 * This file therefore intentionally does NOT import sibling universal
 * grammars. It consumes their canonical rules through the composed parser.
 *
 * ============================================================================
 * IMPORTANT ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * ANTLR imports produce one composed grammar. The canonical Zamani parser
 * composition root imports:
 *
 *     Core
 *     Types
 *     Expressions
 *     ...
 *     Memory
 *     ...
 *
 * Therefore this grammar may reference canonical rules such as:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     typeExpression
 *
 * without redefining them here.
 *
 * This is intentional.
 *
 * The following rules MUST NOT be reintroduced in Memory:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *
 * because their ownership belongs to the universal grammar layers.
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     memoryConstruct
 *     memoryStatement
 *     memoryOperationStatement
 *     memoryExpression
 *     memoryDeclaration
 *     memoryBinding
 *     memoryAnnotation
 *     memoryAnnotationArguments
 *     memoryOperation
 *     memoryQualifiedName
 *     memoryPath
 *     memoryArgumentList
 *     memoryArgument
 *     memoryNamedArgument
 *     memoryPlace
 *     memoryPlaceBase
 *     memoryPlaceSuffix
 *     memoryMemberSuffix
 *     memoryIndexSuffix
 *     memoryDereferenceSuffix
 *     parenthesizedMemoryPlace
 *     memoryTypeAnnotation
 *     memoryOwnershipQualifier
 *     memoryBorrow
 *     memoryLifetimePrefix
 *     memoryLifetime
 *     memoryLifetimeClause
 *     memoryRegion
 *     memoryRegionReference
 *     memorySpace
 *     memorySpaceClause
 *     memoryResource
 *     memoryResourceClause
 *     memoryRequirement
 *     memoryRequirementExpressionList
 *     memoryConstraint
 *     memoryConstraintKeyword
 *     memoryConstraintExpressionList
 *     memoryPreference
 *     memoryPreferenceKeyword
 *     memoryPreferenceExpressionList
 *     memoryHint
 *     memoryHintKeyword
 *     memoryHintExpressionList
 *     memoryPolicy
 *     memoryPolicyClause
 *     memoryIntent
 *     memoryRegionClause
 *     memoryResourceSpecification
 *     memoryResourceValue
 *     memoryQualification
 *     memoryExtensionOperation
 *     memoryExpressionList
 *     memoryTarget
 *     memoryBound
 *     memoryRange
 *     memoryExtensionMetadata
 *     memoryPolicyBundle
 *     memorySpecification
 *     memoryDomainExtension
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical definitions
 *     identifiers
 *     qualified-name implementation
 *     expression precedence
 *     type-system implementation
 *     ownership checking
 *     borrow checking
 *     lifetime inference
 *     allocation algorithms
 *     deallocation algorithms
 *     garbage collection
 *     reference counting
 *     memory placement
 *     resource discovery
 *     hardware discovery
 *     topology
 *     scheduling
 *     routing
 *     optimization
 *     QEC
 *     ZQN
 *     resilience
 *     HAL
 *     quantum::ir
 *     classical IR
 *     HDL/hardware IR
 *     runtime execution
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough syntax information for the domain-neutral
 * frontend AST to represent, where applicable:
 *
 *     operation name
 *     qualified path
 *     operation arguments
 *     named arguments
 *     memory place
 *     memory space
 *     memory region
 *     ownership mode
 *     borrow marker
 *     lifetime reference
 *     type expression
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     policies
 *     extension metadata
 *     source span
 *
 * The parser does not decide whether any construct is semantically valid.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether a memory operation exists;
 *     whether its arguments are valid;
 *     whether a memory place is valid;
 *     whether ownership is valid;
 *     whether borrowing is valid;
 *     whether lifetimes are compatible;
 *     whether a memory space is supported;
 *     whether a region relationship is valid;
 *     whether a resource requirement is satisfiable;
 *     whether a constraint is satisfiable;
 *     whether a preference can be honored;
 *     whether a hint is meaningful;
 *     how memory intent lowers to canonical semantic representations.
 *
 * None of these decisions occur in this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Memory syntax may occur in quantum-classical programs.
 *
 * It MUST NOT create a quantum memory IR.
 *
 * The required architecture remains:
 *
 *     Zamani source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     QEC / ZQN
 *          |
 *          v
 *     HAL
 *
 * Memory grammar remains below the source/semantic boundary.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * The following distinctions are mandatory:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     resource
 *     implementation decision
 *
 * For example:
 *
 *     requires memory::capacity >= required_capacity
 *
 * expresses intent.
 *
 * It does not select:
 *
 *     memory bank 0
 *     NUMA node 0
 *     GPU 0
 *     physical address 0x...
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Memory operation names, memory spaces, regions, policies, resource names,
 * and extension metadata are represented through qualified names or canonical
 * expressions rather than finite enumerations.
 *
 * This permits future memory technologies to be added without changing this
 * foundation merely because a new technology has appeared.
 *
 * Examples:
 *
 *     memory::allocate(...)
 *     memory::release(...)
 *     memory::shared(...)
 *     memory::distributed(...)
 *     memory::persistent(...)
 *     memory::remote(...)
 *     memory::accelerator(...)
 *     memory::future::technology(...)
 *     vendor::memory::extension(...)
 *
 * Semantic registration determines whether such a name is meaningful.
 *
 * ============================================================================
 */

parser grammar Memory;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC MEMORY ENTRY POINT
 * ============================================================================
 *
 * This is the canonical generic memory entry point.
 *
 * Specialized memory grammars must compose through this vocabulary instead of
 * creating another generic memory language.
 *
 * ============================================================================
 */

memoryConstruct
    : memoryStatement
    | memoryExpression
    | memoryAnnotation
    ;


/*
 * ============================================================================
 * 2. MEMORY STATEMENT
 * ============================================================================
 */

memoryStatement
    : memoryOperationStatement
    ;


memoryOperationStatement
    : memoryOperation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 3. MEMORY EXPRESSION
 * ============================================================================
 *
 * A memory operation can appear in expression position.
 *
 * Whether a particular operation is expression-producing is a semantic/type
 * decision, not a parsing decision.
 *
 * ============================================================================
 */

memoryExpression
    : memoryOperation
    ;


/*
 * ============================================================================
 * 4. MEMORY DECLARATION INTEGRATION
 * ============================================================================
 *
 * This is a reusable memory-declaration fragment.
 *
 * The universal declaration grammar remains responsible for deciding where
 * declarations are legal in a complete source unit.
 *
 * ============================================================================
 */

memoryDeclaration
    : memoryBinding
    ;


memoryBinding
    : memoryOwnershipQualifier?
      memoryPlace
      memoryTypeAnnotation?
      memoryLifetimeClause?
      memorySpaceClause?
    ;


/*
 * ============================================================================
 * 5. MEMORY ANNOTATIONS
 * ============================================================================
 *
 * Annotation syntax uses the canonical AT token and canonical qualified names.
 *
 * ============================================================================
 */

memoryAnnotation
    : AT
      memoryQualifiedName
      memoryAnnotationArguments?
    ;


memoryAnnotationArguments
    : LPAREN
      memoryArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 6. OPEN-WORLD MEMORY OPERATION
 * ============================================================================
 *
 * Operation identity is data.
 *
 * Do NOT replace this with a closed enumeration such as:
 *
 *     ALLOCATE | RELEASE | SHARE | DEVICE | NUMA | CACHE
 *
 * Future operations must be representable without modifying this foundation.
 *
 * ============================================================================
 */

memoryOperation
    : memoryQualifiedName
      LPAREN
      memoryArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 7. MEMORY QUALIFIED NAME
 * ============================================================================
 *
 * Canonical identifier and qualified-name syntax remains owned by
 * core/names.g4.
 *
 * This wrapper gives memory-specific consumers a stable memory-domain name.
 *
 * ============================================================================
 */

memoryQualifiedName
    : qualifiedName
    ;


memoryPath
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * 8. MEMORY ARGUMENTS
 * ============================================================================
 *
 * General expression semantics remain owned by grammar/expressions/.
 *
 * Named arguments are retained here because memory operations frequently need
 * semantic property names such as:
 *
 *     extent
 *     count
 *     place
 *     region
 *     space
 *     policy
 *
 * These are identifiers, not a closed keyword list.
 *
 * ============================================================================
 */

memoryArgumentList
    : memoryArgument
      (COMMA memoryArgument)*
      COMMA?
    ;


memoryArgument
    : memoryNamedArgument
    | expression
    ;


memoryNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 9. MEMORY PLACE
 * ============================================================================
 *
 * A memory place is a source-level semantic location.
 *
 * It is NOT a physical address.
 *
 * Examples:
 *
 *     value
 *     object.field
 *     values[index]
 *     namespace::value
 *     object.field[index]
 *
 * ============================================================================
 */

memoryPlace
    : memoryPlaceBase
      memoryPlaceSuffix*
    ;


memoryPlaceBase
    : memoryQualifiedName
    | parenthesizedMemoryPlace
    ;


memoryPlaceSuffix
    : memoryMemberSuffix
    | memoryIndexSuffix
    | memoryDereferenceSuffix
    ;


memoryMemberSuffix
    : DOT
      identifier
    ;


memoryIndexSuffix
    : LBRACKET
      expressionList
      RBRACKET
    ;


memoryDereferenceSuffix
    : STAR
    ;


parenthesizedMemoryPlace
    : LPAREN
      memoryPlace
      RPAREN
    ;


/*
 * ============================================================================
 * 10. MEMORY TYPE ANNOTATION
 * ============================================================================
 *
 * The canonical type grammar owns typeExpression.
 *
 * This file only provides the memory-specific attachment point.
 *
 * ============================================================================
 */

memoryTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 11. OWNERSHIP QUALIFIER
 * ============================================================================
 *
 * Existing canonical lexical tokens are reused.
 *
 * The grammar does not interpret them.
 *
 * ============================================================================
 */

memoryOwnershipQualifier
    : LINEAR
    | AFFINE
    ;


/*
 * ============================================================================
 * 12. BORROW
 * ============================================================================
 *
 * This reusable syntax is retained for borrowing.g4 and related composition.
 *
 * Semantic borrow checking remains outside the grammar.
 *
 * ============================================================================
 */

memoryBorrow
    : AMPERSAND
      memoryLifetimePrefix?
      MUT?
      memoryPlace
    ;


memoryLifetimePrefix
    : APOSTROPHE
      identifier
    ;


/*
 * ============================================================================
 * 13. LIFETIME
 * ============================================================================
 *
 * A lifetime is a symbolic source-level name.
 *
 * It is NOT:
 *
 *     a clock duration
 *     a number of cycles
 *     a physical retention period
 *     a scheduler interval
 *
 * ============================================================================
 */

memoryLifetime
    : APOSTROPHE
      identifier
    ;


memoryLifetimeClause
    : memoryLifetime
    ;


/*
 * ============================================================================
 * 14. MEMORY REGION
 * ============================================================================
 *
 * A memory region is semantic.
 *
 * It does not imply a physical heap, bank, NUMA node, page, cache, or device.
 *
 * ============================================================================
 */

memoryRegion
    : memoryRegionReference
    ;


memoryRegionReference
    : memoryQualifiedName
    ;


memoryRegionClause
    : IN
      memoryRegion
    ;


/*
 * ============================================================================
 * 15. MEMORY SPACE
 * ============================================================================
 *
 * Memory-space identities remain open-world.
 *
 * Examples:
 *
 *     memory::local
 *     memory::shared
 *     memory::device
 *     memory::persistent
 *     memory::remote
 *     memory::distributed
 *     future::memory::space
 *
 * ============================================================================
 */

memorySpace
    : memoryQualifiedName
    ;


memorySpaceClause
    : memorySpace
    ;


/*
 * ============================================================================
 * 16. MEMORY RESOURCE
 * ============================================================================
 *
 * This is a symbolic memory-resource reference.
 *
 * It does not duplicate the universal resources grammar.
 *
 * ============================================================================
 */

memoryResource
    : memoryQualifiedName
    ;


memoryResourceClause
    : memoryResource
    ;


memoryResourceSpecification
    : memoryResourceClause
      memoryResourceValue?
    ;


memoryResourceValue
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 17. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * The canonical REQUIRES token is retained.
 *
 * ============================================================================
 */

memoryRequirement
    : REQUIRES
      memoryRequirementExpressionList
    ;


memoryRequirementExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 18. CONSTRAINTS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The existing grammar incorrectly used REQUIRES for this category.
 *
 * CONSTRAINT is already a canonical Zamani keyword and is therefore used here.
 *
 * ============================================================================
 */

memoryConstraint
    : memoryConstraintKeyword
      memoryConstraintExpressionList
    ;


memoryConstraintKeyword
    : CONSTRAINT
    ;


memoryConstraintExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 19. PREFERENCES
 * ============================================================================
 *
 * PREFER is the canonical lexical token.
 *
 * A preference is not a correctness requirement.
 *
 * ============================================================================
 */

memoryPreference
    : memoryPreferenceKeyword
      memoryPreferenceExpressionList
    ;


memoryPreferenceKeyword
    : PREFER
    ;


memoryPreferenceExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 20. HINTS
 * ============================================================================
 *
 * HINT is advisory.
 *
 * Ignoring a hint must not change program semantics.
 *
 * ============================================================================
 */

memoryHint
    : memoryHintKeyword
      memoryHintExpressionList
    ;


memoryHintKeyword
    : HINT
    ;


memoryHintExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 21. MEMORY POLICY
 * ============================================================================
 *
 * Policy identity remains open-world.
 *
 * WITH is the existing generic policy/metadata composition token.
 *
 * ============================================================================
 */

memoryPolicy
    : memoryQualifiedName
    ;


memoryPolicyClause
    : WITH
      memoryPolicy
    ;


/*
 * ============================================================================
 * 22. MEMORY INTENT
 * ============================================================================
 *
 * This is the reusable bundle of memory semantics that can be attached to
 * allocation, shared-memory, declarations, resources, and future domains.
 *
 * The four semantic categories remain structurally distinct:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * ============================================================================
 */

memoryIntent
    : memoryOwnershipQualifier?
      memoryTypeAnnotation?
      memoryLifetimeClause?
      memorySpaceClause?
      memoryRegionClause?
      memoryResourceSpecification*
      memoryRequirement*
      memoryConstraint*
      memoryPreference*
      memoryHint*
      memoryPolicyClause?
    ;


/*
 * ============================================================================
 * 23. MEMORY QUALIFICATION
 * ============================================================================
 *
 * General-purpose symbolic qualification.
 * ============================================================================
 */

memoryQualification
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * 24. MEMORY EXTENSION OPERATION
 * ============================================================================
 *
 * Generic extension point for future memory technologies and dialects.
 *
 * Examples:
 *
 *     vendor::memory::operation(...)
 *     future::memory::operation(...)
 *
 * ============================================================================
 */

memoryExtensionOperation
    : memoryQualifiedName
      LPAREN
      memoryArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 25. MEMORY EXPRESSION LIST
 * ============================================================================
 *
 * This reusable list remains available to existing memory consumers and
 * annotations.
 *
 * The general expression grammar remains authoritative for expression syntax.
 *
 * ============================================================================
 */

memoryExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 26. MEMORY TARGET
 * ============================================================================
 *
 * A memory target is semantic source-level intent.
 *
 * It does not represent a physical address or physical device.
 *
 * ============================================================================
 */

memoryTarget
    : memoryPlace
    | memorySpace
    | memoryRegion
    ;


/*
 * ============================================================================
 * 27. MEMORY BOUND
 * ============================================================================
 *
 * A memory bound is an arbitrary expression.
 *
 * It may be:
 *
 *     constant
 *     symbolic
 *     generic
 *     runtime-derived
 *     resource-derived
 *     computed
 *
 * No finite numeric limit is encoded.
 *
 * ============================================================================
 */

memoryBound
    : expression
    ;


/*
 * ============================================================================
 * 28. MEMORY RANGE
 * ============================================================================
 *
 * Range endpoints are semantic expressions.
 *
 * ============================================================================
 */

memoryRange
    : memoryBound
      DOT_DOT
      memoryBound
    | memoryBound
      DOT_DOT_EQ
      memoryBound
    ;


/*
 * ============================================================================
 * 29. MEMORY EXTENSION METADATA
 * ============================================================================
 *
 * Metadata is intentionally open-world and source-level.
 *
 * ============================================================================
 */

memoryExtensionMetadata
    : AT
      memoryQualifiedName
      (
          LPAREN
          memoryExpressionList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 30. MEMORY POLICY BUNDLE
 * ============================================================================
 *
 * There is deliberately ONE definition of this rule.
 *
 * It keeps semantic categories distinct while allowing them to be composed
 * by specialized memory grammars.
 *
 * ============================================================================
 */

memoryPolicyBundle
    : memoryRequirement*
      memoryConstraint*
      memoryPreference*
      memoryHint*
      memoryPolicyClause?
    ;


/*
 * ============================================================================
 * 31. COMPOSABLE MEMORY SPECIFICATION
 * ============================================================================
 *
 * This is the preferred generic specification boundary for:
 *
 *     allocation
 *     shared memory
 *     distributed memory
 *     accelerator memory
 *     persistent memory
 *     future memory domains
 *
 * ============================================================================
 */

memorySpecification
    : memoryIntent
      memoryPolicyBundle
    ;


/*
 * ============================================================================
 * 32. MEMORY-DOMAIN EXTENSION
 * ============================================================================
 *
 * A future memory-domain operation can be represented without adding another
 * closed list of parser alternatives.
 *
 * Semantic registration determines whether the extension is valid.
 *
 * ============================================================================
 */

memoryDomainExtension
    : memoryQualifiedName
      (
          LPAREN
          memoryArgumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * allocation.g4
 * --------------
 *
 * May consume:
 *
 *     memoryOperation
 *     memoryArgument
 *     memoryArgumentList
 *     memoryNamedArgument
 *     memoryPlace
 *     memoryQualifiedName
 *     memoryTypeAnnotation
 *     memoryLifetime
 *     memorySpace
 *     memorySpaceClause
 *
 *
 * deallocation.g4
 * ----------------
 *
 * May consume:
 *
 *     memoryPlace
 *     memoryQualifiedName
 *     memoryOperation
 *
 *
 * ownership.g4
 * -------------
 *
 * May consume:
 *
 *     memoryPlace
 *     memoryOwnershipQualifier
 *     memorySpecification
 *
 *
 * borrowing.g4
 * -------------
 *
 * May consume:
 *
 *     memoryBorrow
 *     memoryPlace
 *     memoryLifetimePrefix
 *     memoryOperation
 *
 *
 * lifetimes.g4
 * -------------
 *
 * May consume:
 *
 *     memoryLifetime
 *     memoryLifetimeClause
 *
 *
 * shared-memory.g4
 * ----------------
 *
 * May consume:
 *
 *     memoryQualifiedName
 *     memoryPlace
 *     memoryArgument
 *     memoryArgumentList
 *     memoryRequirement
 *     memoryConstraint
 *     memoryPreference
 *     memoryHint
 *     memoryPolicy
 *     memorySpecification
 *
 *
 * distributed-memory.g4
 * ---------------------
 *
 * May consume:
 *
 *     memoryQualifiedName
 *     memoryPlace
 *     memoryTarget
 *     memorySpecification
 *
 *
 * memory-constraints.g4
 * ---------------------
 *
 * May consume:
 *
 *     memoryPlace
 *     memorySpace
 *     memoryRequirement
 *     memoryConstraint
 *     memoryPreference
 *     memoryHint
 *
 *
 * resources/
 * ----------
 *
 * Memory-specific resource intent must lower into the universal resource model.
 *
 * This file must not create a second resource model.
 *
 *
 * types/
 * ------
 *
 * memoryTypeAnnotation consumes the canonical typeExpression rule.
 *
 * This file must not define a second type system.
 *
 *
 * expressions/
 * ------------
 *
 * memoryArgument and memoryBound consume canonical expression syntax.
 *
 * This file must not define expression precedence.
 *
 *
 * core/names.g4
 * -------------
 *
 * memoryQualifiedName consumes canonical qualifiedName syntax.
 *
 * This file must not redefine identifier or qualifiedName.
 *
 * ============================================================================
 * CANONICAL DOWNSTREAM PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Memory parse tree
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 * ownership/type analysis   resource/capability analysis
 *       |                         |
 *       +------------+------------+
 *                    |
 *                    v
 *             canonical semantic model
 *                    |
 *          +---------+----------+----------------+
 *          |                    |                |
 *          v                    v                v
 *     classical            quantum::ir      HDL/hardware
 *          |                    |                |
 *          +--------------------+---------------+
 *                               |
 *                               v
 *                     optimization / lowering
 *                               |
 *                     routing / scheduling
 *                               |
 *                     resilience / QEC / ZQN
 *                               |
 *                              HAL
 *                               |
 *                       target realization
 *                               |
 *                            runtime
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no environment inspection
 *     no hardware discovery
 *     no randomness
 *     no runtime execution
 *
 * Given the same source and grammar/version, parsing must be deterministic.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains no Rust code and therefore contains no unsafe Rust.
 *
 * The consuming implementation must remain compatible with:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_MEMORY
 *     MAX_ALLOCATIONS
 *     MAX_REGIONS
 *     MAX_LIFETIMES
 *     MAX_REFERENCES
 *     MAX_MEMORY_SPACES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_THREADS
 *
 * Numeric memory quantities remain program expressions.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] There is exactly one canonical Memory grammar.
 *
 * [x] Existing filename memory.g4 is retained.
 *
 * [x] Duplicate memoryPlace rules are removed.
 *
 * [x] Duplicate memoryBound rules are removed.
 *
 * [x] Duplicate memoryPolicyBundle rules are removed.
 *
 * [x] The canonical IDENTIFIER vocabulary is used indirectly through
 *     core/names.g4 rather than redefining identifier.
 *
 * [x] General expression syntax is not duplicated.
 *
 * [x] General type syntax is not duplicated.
 *
 * [x] Memory operations remain open-world.
 *
 * [x] Memory spaces remain open-world.
 *
 * [x] Memory regions remain open-world.
 *
 * [x] Requirements, constraints, preferences, and hints remain distinct.
 *
 * [x] CONSTRAINT uses the canonical CONSTRAINT token.
 *
 * [x] PREFER uses the canonical PREFER token.
 *
 * [x] HINT uses the canonical HINT token.
 *
 * [x] No physical address is required.
 *
 * [x] No machine topology is encoded.
 *
 * [x] No resource-count ceiling is encoded.
 *
 * [x] No quantum IR is introduced.
 *
 * [x] No QEC implementation is introduced.
 *
 * [x] No ZQN implementation is introduced.
 *
 * [x] No scheduling is introduced.
 *
 * [x] No routing is introduced.
 *
 * [x] No hardware is selected.
 *
 * [x] No runtime dependency is introduced.
 *
 * [x] Rust implementation remains safe Rust.
 *
 * [x] Rust 1.97 / 1.97.1 remains the implementation baseline.
 *
 * [x] Existing specialized memory grammars have predetermined integration
 *     points.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar defines:
 *
 *     WHAT memory-related syntax means structurally.
 *
 * It does not define:
 *
 *     HOW memory is physically realized.
 *
 * Therefore:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * remains compatible with memory systems ranging from tiny embedded
 * environments through heterogeneous, distributed, accelerator, quantum-
 * classical, HPC, cloud, and future computational architectures, subject to
 * actual semantic requirements and available resources.
 *
 * ============================================================================
 */