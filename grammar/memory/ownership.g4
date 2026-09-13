/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/ownership.g4
 *
 * Grammar role:
 *     Reusable ownership-syntax parser grammar.
 *
 * Architectural status:
 *     Production ownership-domain syntax foundation.
 *
 * Language:
 *     Zamani
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe implementation permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns ONLY THE SOURCE SYNTAX required to express ownership
 * intent.
 *
 * Ownership semantics are deliberately separated from:
 *
 *     - type checking;
 *     - borrow checking;
 *     - lifetime checking;
 *     - alias analysis;
 *     - allocation;
 *     - deallocation;
 *     - memory placement;
 *     - physical storage;
 *     - resource discovery;
 *     - scheduling;
 *     - hardware selection;
 *     - runtime execution.
 *
 * The parser records what the programmer wrote.
 *
 * Semantic analysis determines whether that ownership intent is valid.
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
 *     Core / Memory parser
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *      ownership.g4          other memory grammars
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *                Frontend AST
 *                     |
 *                     v
 *             semantic analysis
 *                     |
 *          +----------+-----------+
 *          |                      |
 *          v                      v
 *     ownership model       memory/resource model
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              canonical semantic IR
 *                     |
 *          +----------+-----------+
 *          |          |           |
 *          v          v           v
 *       classical   quantum     hardware
 *          IR         IR           IR
 *          |          |           |
 *          +----------+-----------+
 *                     |
 *                     v
 *          optimization / routing /
 *          scheduling / lowering
 *                     |
 *                     v
 *                   runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   1. Ownership mode syntax.
 *
 *   2. Ownership qualifiers:
 *
 *          linear
 *          affine
 *
 *   3. Explicit ownership-policy syntax.
 *
 *   4. Ownership annotations.
 *
 *   5. Ownership subjects/references used by ownership constructs.
 *
 *   6. Ownership projections needed to identify source-level places.
 *
 *   7. Ownership relation syntax.
 *
 *   8. Ownership transfer intent syntax where the language surface explicitly
 *      represents transfer.
 *
 *   9. Ownership-policy arguments and modifiers.
 *
 *  10. Extensible ownership-domain names.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - identifiers in general;
 *   - lexical rules;
 *   - Unicode;
 *   - literals in general;
 *   - general expressions;
 *   - statements;
 *   - declarations;
 *   - type expressions;
 *   - references as types;
 *   - pointer types;
 *   - borrowing;
 *   - lifetimes;
 *   - allocation;
 *   - deallocation;
 *   - memory spaces;
 *   - memory resources;
 *   - memory constraints;
 *   - memory placement;
 *   - physical addresses;
 *   - hardware addresses;
 *   - stack layout;
 *   - heap layout;
 *   - register allocation;
 *   - cache allocation;
 *   - NUMA placement;
 *   - GPU memory;
 *   - accelerator memory;
 *   - distributed-memory placement;
 *   - quantum-resource allocation;
 *   - qubit allocation;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime execution;
 *   - compiler code generation.
 *
 * ============================================================================
 * CANONICAL OWNERSHIP SEMANTICS
 * ============================================================================
 *
 * `linear` and `affine` are semantic ownership qualifiers.
 *
 * This grammar DOES NOT define their complete mathematical/semantic rules.
 *
 * The semantic layer determines:
 *
 *     - whether a value has linear semantics;
 *     - whether a value has affine semantics;
 *     - whether a use consumes a value;
 *     - whether duplication is legal;
 *     - whether dropping is legal;
 *     - whether moving is legal;
 *     - whether ownership may be transferred;
 *     - whether an ownership constraint is satisfied.
 *
 * The parser merely preserves the source-level declaration.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Ownership syntax MUST remain independent of physical machine scale.
 *
 * It therefore MUST NOT encode:
 *
 *     maximum owners;
 *     maximum values;
 *     maximum regions;
 *     maximum references;
 *     maximum allocations;
 *     maximum memory;
 *     pointer width;
 *     address width;
 *     register count;
 *     stack size;
 *     heap size;
 *     number of cores;
 *     number of threads;
 *     number of devices;
 *     number of qubits;
 *     number of nodes;
 *     topology;
 *     physical addresses.
 *
 * Any such limitation belongs to:
 *
 *     target capabilities;
 *     resource analysis;
 *     compilation policy;
 *     scheduling;
 *     runtime policy;
 *     hardware discovery.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical source is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar does NOT declare lexer rules.
 *
 * Existing canonical ownership-related tokens include:
 *
 *     LINEAR
 *     AFFINE
 *     MUT
 *     LET
 *     CONST
 *     VAR
 *     VAL
 *     THIS
 *     SELF
 *     IDENTIFIER
 *
 * Existing punctuation tokens/literals are consumed where appropriate.
 *
 * `owned`, `borrowed`, `move`, `consume`, `copy`, and future ownership-domain
 * vocabulary are intentionally NOT required to become globally reserved
 * keywords merely because this grammar needs to recognize them.
 *
 * Such names can remain ordinary identifiers when represented as extensible
 * ownership-domain operations.
 *
 * ============================================================================
 * IMPORTANT ANTLR INTEGRATION RULE
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It intentionally does not define:
 *
 *     grammar Zamani;
 *
 * and it does not define a second program root.
 *
 * It is intended to be imported/consumed by the memory parser composition
 * layer.
 *
 * The host grammar is responsible for integrating the rules below into:
 *
 *     declarations;
 *     expressions;
 *     statements;
 *     memory operations;
 *     annotations.
 *
 * The host grammar MUST NOT copy these rules.
 *
 * ============================================================================
 * OWNERSHIP QUALIFIERS
 * ============================================================================
 *
 * The foundational ownership modes are:
 *
 *     linear
 *     affine
 *
 * These are the only globally reserved ownership modes currently required by
 * the canonical lexer.
 *
 * Future ownership systems should preferably use the extensible policy form
 * rather than requiring changes to this foundational grammar.
 * ============================================================================
 */

parser grammar Ownership;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * OWNERSHIP MODE
 * ========================================================================== */

/*
 * Core ownership modes.
 *
 * Semantic meaning is determined by semantic analysis.
 */
ownershipMode
    : LINEAR
    | AFFINE
    ;


/*
 * Optional ownership qualifier.
 *
 * Useful when the host grammar accepts ordinary declarations and permits an
 * ownership modifier to precede them.
 */
ownershipQualifier
    : ownershipMode
    ;


/*
 * Zero-or-one ownership qualifier.
 */
optionalOwnershipQualifier
    : ownershipQualifier?
    ;


/* ============================================================================
 * OWNERSHIP ANNOTATION
 * ========================================================================== */

/*
 * Generic ownership annotation.
 *
 * Examples:
 *
 *     @ownership(linear)
 *     @ownership(affine)
 *
 * The annotation syntax remains intentionally small.
 *
 * General annotation ownership remains in the core annotation grammar.
 * This rule exists only for semantic ownership payloads.
 */
ownershipAnnotation
    : '@'
      'ownership'
      '('
      ownershipMode
      ')'
    ;


/*
 * Extensible ownership-domain annotation.
 *
 * This allows future ownership policies without requiring every policy to
 * become a lexer keyword.
 *
 * Examples:
 *
 *     @ownership(policy)
 *     @ownership(unique)
 *     @ownership(consuming)
 *
 * Semantic interpretation belongs downstream.
 */
ownershipNamedAnnotation
    : '@'
      'ownership'
      '('
      ownershipName
      ')'
    ;


/*
 * Ownership annotation accepted by memory-domain composition.
 */
ownershipAnnotationValue
    : ownershipMode
    | ownershipName
    ;


/* ============================================================================
 * OWNERSHIP NAMES
 * ========================================================================== */

/*
 * Extensible ownership-domain name.
 *
 * This is deliberately identifier-based.
 *
 * It prevents the grammar from becoming a closed inventory of ownership
 * technologies or research models.
 */
ownershipName
    : IDENTIFIER
    ;


/*
 * Qualified ownership-domain name.
 *
 * Examples:
 *
 *     linear
 *     affine
 *     experimental::unique
 *     domain::ownership::policy
 *
 * There is no fixed qualification depth.
 */
qualifiedOwnershipName
    : ownershipName
      (
          DOUBLE_COLON
          ownershipName
      )*
    ;


/* ============================================================================
 * OWNERSHIP SUBJECT
 * ========================================================================== */

/*
 * A source-level ownership subject.
 *
 * The subject is intentionally NOT a general expression.
 *
 * General expressions remain owned by expressions.g4.
 */
ownershipSubject
    : ownershipIdentifier
    | SELF
    | THIS
    ;


/*
 * Identifier used as the root of an ownership place.
 */
ownershipIdentifier
    : IDENTIFIER
    ;


/* ============================================================================
 * OWNERSHIP PLACE
 * ============================================================================
 *
 * Ownership analysis operates over source-level places.
 *
 * A place can identify:
 *
 *     x
 *     self
 *     this
 *     x.field
 *     self.field
 *     object.field.subfield
 *
 * Indexing is deliberately represented through a restricted projection form
 * rather than redefining the entire expression grammar.
 *
 * The semantic layer resolves whether the resulting place is actually valid.
 * ========================================================================== */

ownershipPlace
    : ownershipSubject
      ownershipProjection*
    ;


/*
 * Field/member projection.
 */
ownershipFieldProjection
    : DOT
      ownershipIdentifier
    ;


/*
 * Index projection.
 *
 * The index itself is deliberately opaque to this grammar.
 *
 * The host expression grammar remains the canonical owner of complete index
 * expressions.
 *
 * The `ownershipIndexExpression` rule provides a syntactic bridge without
 * trying to recreate arithmetic, calls, ranges, or other expressions here.
 */
ownershipIndexProjection
    : LBRACKET
      ownershipIndexExpression
      RBRACKET
    ;


/*
 * Projection used by an ownership place.
 */
ownershipProjection
    : ownershipFieldProjection
    | ownershipIndexProjection
    ;


/*
 * Opaque ownership index.
 *
 * The host parser/AST layer may replace this syntactic bridge with its
 * canonical expression node.
 *
 * This rule deliberately accepts a qualified source reference rather than
 * creating an alternative expression language.
 */
ownershipIndexExpression
    : ownershipSubject
    | INTEGER
    ;


/* ============================================================================
 * OWNERSHIP DECLARATION QUALIFIERS
 * ========================================================================== */

/*
 * Ownership modifier attached to a declaration by the host grammar.
 *
 * Examples:
 *
 *     linear
 *     affine
 *
 * The declaration itself remains owned by declarations.g4.
 */
ownershipDeclarationQualifier
    : ownershipQualifier
    ;


/*
 * Ownership-qualified binding prefix.
 *
 * This rule is useful to declaration grammar composition.
 *
 * It intentionally does not contain:
 *
 *     variableDeclaration
 *     typeExpression
 *     initializer
 *
 * Those remain owned by their canonical grammars.
 */
ownershipBindingPrefix
    : ownershipQualifier
    ;


/* ============================================================================
 * OWNERSHIP RELATIONS
 * ========================================================================== */

/*
 * Explicit ownership relation.
 *
 * Examples:
 *
 *     ownership(x)
 *     ownership(x, y)
 *
 * The operation is semantic syntax, not runtime execution.
 */
ownershipRelation
    : 'ownership'
      '('
      ownershipPlace
      (
          COMMA
          ownershipPlace
      )*
      ')'
    ;


/*
 * Ownership transfer intent.
 *
 * This form is intentionally domain-qualified instead of introducing a
 * permanently reserved global `move` keyword.
 *
 * Examples:
 *
 *     ownership::transfer(x)
 *     ownership::consume(x)
 *     ownership::move(x)
 *
 * The semantic layer determines whether the operation is legal.
 */
ownershipTransfer
    : ownershipTransferOperation
      '('
      ownershipPlace
      (
          COMMA
          ownershipArgument
      )*
      ')'
    ;


/*
 * Transfer operation namespace.
 *
 * The first component remains fixed because this is the ownership-domain
 * grammar.
 *
 * The operation itself remains extensible.
 */
ownershipTransferOperation
    : 'ownership'
      DOUBLE_COLON
      ownershipName
    ;


/*
 * Additional ownership argument.
 *
 * Ownership operations must not recreate the complete expression grammar.
 *
 * The host parser may attach the canonical expression AST to the argument
 * position after syntactic recognition.
 */
ownershipArgument
    : ownershipPlace
    | ownershipName
    | INTEGER
    | STRING
    ;


/* ============================================================================
 * OWNERSHIP POLICY
 * ========================================================================== */

/*
 * Policy declaration.
 *
 * Examples:
 *
 *     ownership::policy(linear)
 *     ownership::policy(affine)
 *     ownership::policy(custom)
 *
 * The policy has no implicit physical-memory meaning.
 */
ownershipPolicy
    : 'ownership'
      DOUBLE_COLON
      'policy'
      '('
      ownershipPolicyValue
      ')'
    ;


/*
 * Policy value.
 */
ownershipPolicyValue
    : ownershipMode
    | qualifiedOwnershipName
    ;


/*
 * Named ownership policy.
 */
namedOwnershipPolicy
    : 'ownership'
      DOUBLE_COLON
      'policy'
      '('
      qualifiedOwnershipName
      ')'
    ;


/* ============================================================================
 * OWNERSHIP REQUIREMENTS
 * ========================================================================== */

/*
 * Ownership requirement.
 *
 * This is a source-level semantic requirement.
 *
 * It does NOT mean that a physical allocator must satisfy anything.
 */
ownershipRequirement
    : 'ownership'
      DOUBLE_COLON
      'requires'
      '('
      ownershipRequirementValue
      ')'
    ;


ownershipRequirementValue
    : ownershipMode
    | qualifiedOwnershipName
    ;


/* ============================================================================
 * OWNERSHIP CONSTRAINTS
 * ========================================================================== */

/*
 * Ownership constraint.
 *
 * A constraint restricts legal semantic implementations.
 *
 * It is distinct from:
 *
 *     requirement;
 *     preference;
 *     hint.
 */
ownershipConstraint
    : 'ownership'
      DOUBLE_COLON
      'constraint'
      '('
      ownershipConstraintValue
      ')'
    ;


ownershipConstraintValue
    : ownershipMode
    | qualifiedOwnershipName
    ;


/* ============================================================================
 * OWNERSHIP PREFERENCES
 * ========================================================================== */

/*
 * Ownership preference.
 *
 * Preferences must never be interpreted as mandatory semantic requirements.
 */
ownershipPreference
    : 'ownership'
      DOUBLE_COLON
      'prefer'
      '('
      ownershipPreferenceValue
      ')'
    ;


ownershipPreferenceValue
    : ownershipMode
    | qualifiedOwnershipName
    ;


/* ============================================================================
 * OWNERSHIP HINTS
 * ========================================================================== */

/*
 * Ownership implementation hint.
 *
 * Hints are optional compiler/runtime guidance.
 *
 * They do not alter program semantics by themselves.
 */
ownershipHint
    : 'ownership'
      DOUBLE_COLON
      'hint'
      '('
      ownershipHintValue
      ')'
    ;


ownershipHintValue
    : ownershipMode
    | qualifiedOwnershipName
    ;


/* ============================================================================
 * OWNERSHIP DOMAIN DIRECTIVE
 * ========================================================================== */

/*
 * Generic ownership-domain directive.
 *
 * Examples:
 *
 *     ownership::policy(...)
 *     ownership::requires(...)
 *     ownership::constraint(...)
 *     ownership::prefer(...)
 *     ownership::hint(...)
 *
 * Future directives can be represented through the same extensible domain
 * mechanism without modifying the lexer.
 */
ownershipDirective
    : 'ownership'
      DOUBLE_COLON
      ownershipDirectiveName
      '('
      ownershipDirectiveArguments?
      ')'
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
    : ownershipName
    | ownershipPlace
    | INTEGER
    | STRING
    ;


/* ============================================================================
 * OWNERSHIP USE
 * ========================================================================== */

/*
 * Explicit ownership use.
 *
 * This identifies the source-level subject whose ownership semantics are
 * being queried or constrained.
 */
ownershipUse
    : 'ownership'
      '('
      ownershipPlace
      ')'
    ;


/*
 * Ownership-qualified use.
 *
 * Examples:
 *
 *     linear x
 *     affine x
 *
 * This is intended for composition with declaration/parameter grammars.
 */
ownershipQualifiedUse
    : ownershipQualifier
      ownershipPlace
    ;


/* ============================================================================
 * OWNERSHIP ASSERTION
 * ========================================================================== */

/*
 * Ownership assertion.
 *
 * The grammar does not prove the assertion.
 *
 * Semantic analysis must validate it.
 */
ownershipAssertion
    : 'ownership'
      DOUBLE_COLON
      'assert'
      '('
      ownershipAssertionValue
      ')'
    ;


ownershipAssertionValue
    : ownershipMode
    | qualifiedOwnershipName
    | ownershipPlace
    ;


/* ============================================================================
 * OWNERSHIP CONVERSION / ADAPTATION
 * ========================================================================== */

/*
 * Explicit ownership-domain adaptation.
 *
 * This is intentionally syntactic.
 *
 * Semantic analysis must reject conversions that violate the language's
 * ownership model.
 */
ownershipAdaptation
    : 'ownership'
      DOUBLE_COLON
      'adapt'
      '('
      ownershipPlace
      COMMA
      ownershipPolicyValue
      ')'
    ;


/* ============================================================================
 * OWNERSHIP EXTENSION POINT
 * ========================================================================== */

/*
 * Generic extension point for future ownership models.
 *
 * Examples:
 *
 *     ownership::unique(...)
 *     ownership::persistent(...)
 *     ownership::region(...)
 *     ownership::capability(...)
 *     ownership::distributed(...)
 *
 * The grammar does not assign these names any semantics.
 */
ownershipExtension
    : 'ownership'
      DOUBLE_COLON
      qualifiedOwnershipName
      (
          '('
          ownershipDirectiveArguments?
          ')'
      )?
    ;


/* ============================================================================
 * OWNERSHIP CONSTRUCT
 * ========================================================================== */

/*
 * Complete ownership-domain construct.
 *
 * This is the principal entry rule intended for the memory grammar.
 *
 * It permits the host grammar to integrate ownership without duplicating
 * individual ownership productions.
 */
ownershipConstruct
    : ownershipAnnotation
    | ownershipNamedAnnotation
    | ownershipRelation
    | ownershipTransfer
    | ownershipPolicy
    | ownershipRequirement
    | ownershipConstraint
    | ownershipPreference
    | ownershipHint
    | ownershipAssertion
    | ownershipAdaptation
    | ownershipDirective
    | ownershipExtension
    ;


/* ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Nothing in this grammar determines:
 *
 *     ownership validity;
 *     borrow validity;
 *     lifetime validity;
 *     alias validity;
 *     allocation legality;
 *     deallocation legality;
 *     memory placement;
 *     resource availability;
 *     hardware capability;
 *     runtime state.
 *
 * Those belong downstream.
 * ============================================================================
 */