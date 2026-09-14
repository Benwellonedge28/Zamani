/**
 * Zamani Universal Computing Language
 *
 * File:
 *   grammar/resources/requirements.g4
 *
 * Purpose:
 *   Defines the syntax for declarative resource requirements.
 *
 * Architectural rule:
 *   A requirement describes what a computation needs semantically.
 *   It does NOT select a machine, device, topology, address, placement,
 *   scheduler, backend, compiler, or runtime implementation.
 *
 * Ownership:
 *   This grammar owns the syntactic structure of resource requirements.
 *
 * Non-ownership:
 *   This grammar does not own:
 *     - resource discovery
 *     - hardware discovery
 *     - capability evaluation
 *     - resource allocation
 *     - scheduling
 *     - routing
 *     - placement
 *     - optimization
 *     - compilation
 *     - runtime execution
 *     - quantum IR
 *     - classical IR
 *     - hardware IR
 *     - resource-management policy
 *     - target selection
 *
 * POCO-REAF:
 *   Requirements are portable semantic declarations. A requirement may
 *   describe a minimum capability, a semantic property, a quantitative
 *   bound, a relationship, or a conditional requirement without encoding
 *   a fixed physical machine.
 *
 * Scalability:
 *   There are intentionally no grammar-level limits on:
 *     - number of requirements
 *     - number of resources
 *     - number of dimensions
 *     - number of operands
 *     - number of machines
 *     - number of devices
 *     - number of qubits
 *     - number of CPUs/GPUs/FPGAs
 *     - memory capacity
 *     - network size
 *     - cluster size
 *
 * All physical limits belong to semantic analysis, resource management,
 * compilation, deployment, or runtime capability information.
 *
 * Integration:
 *   resources.g4 should import this grammar and expose requirement rules
 *   from its public resource-declaration surface.
 *
 *   This grammar intentionally delegates value/expression syntax to
 *   resource-expressions.g4. That file is the canonical syntax boundary
 *   for resource expressions.
 *
 * Rust:
 *   This is ANTLR grammar source. It contains no Rust implementation code
 *   and therefore introduces no unsafe Rust. Generated Rust must remain
 *   compatible with the repository's Rust 1.97 / 1.97.1 toolchain.
 *
 * NOTE:
 *   The exact imported grammar name MUST match the actual grammar name
 *   declared by grammar/resources/resource-expressions.g4.
 *   If that grammar is named ResourceExpressions, the import below is
 *   correct. If the repository establishes another canonical grammar name,
 *   only the import name should be changed; the requirement model below
 *   should remain unchanged.
 */

parser grammar Requirements;

import ResourceExpressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A resource requirement is one complete declarative requirement.
 *
 * Examples of semantic forms this structure is intended to support:
 *
 *   requires <resource-expression>
 *
 *   requires <resource-expression> if <resource-expression>
 *
 *   requires all <resource-expression>
 *
 *   requires any <resource-expression>
 *
 *   requires one-of <resource-expression>
 *
 *   requires <resource-expression> where <resource-expression>
 *
 * The concrete vocabulary for "requires", "all", "any", etc. must come
 * from the repository's canonical lexer/keyword layer.
 *
 * This grammar therefore uses the existing resource-expression surface
 * rather than defining a second expression language here.
 */
requirement
    : requirementHead requirementBody?
    ;


/*
 * ============================================================================
 * REQUIREMENT HEAD
 * ============================================================================
 *
 * Requirement heads identify the semantic strength/aggregation model of
 * the requirement.
 *
 * A simple requirement is the default:
 *
 *   requirementHead = requirementKeyword
 *
 * Aggregated requirements allow semantic composition without introducing
 * machine-specific assumptions.
 */
requirementHead
    : requirementKeyword
    | requirementAllKeyword
    | requirementAnyKeyword
    | requirementOneOfKeyword
    ;


/*
 * ============================================================================
 * REQUIREMENT BODY
 * ============================================================================
 *
 * The body is deliberately expression-oriented.
 *
 * Resource expressions are interpreted later by semantic analysis.
 *
 * This prevents this grammar from deciding:
 *
 *   - what "GPU" physically means
 *   - what a quantum processor is
 *   - how many qubits exist
 *   - which topology is available
 *   - which backend is selected
 *   - where an operation executes
 */
requirementBody
    : resourceRequirementExpression
    ;


/*
 * ============================================================================
 * RESOURCE REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * This is the primary integration boundary with resource-expressions.g4.
 *
 * Resource expressions may represent:
 *
 *   - resource identity
 *   - resource class
 *   - capability predicates
 *   - quantitative requirements
 *   - symbolic values
 *   - relationships
 *   - logical combinations
 *   - conditional expressions
 *   - resource dimensions
 *   - portable semantic properties
 *
 * The expression grammar, rather than this grammar, owns expression
 * precedence and expression syntax.
 */
resourceRequirementExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * CONDITIONAL REQUIREMENTS
 * ============================================================================
 *
 * Conditional requirements express semantic dependencies.
 *
 * They do not mean "select hardware X if condition Y".
 *
 * They mean:
 *
 *   "This requirement applies when the semantic condition is true."
 *
 * Runtime/compiler policy determines how the requirement is satisfied.
 *
 * Example conceptual form:
 *
 *   requires <requirement> if <condition>
 *
 * The condition itself is a resource expression so that no second
 * conditional expression language is introduced.
 */
conditionalRequirement
    : requirementHead resourceRequirementExpression
      requirementConditionClause
    ;

requirementConditionClause
    : requirementIfKeyword resourceRequirementExpression
    ;


/*
 * ============================================================================
 * COMPOSED REQUIREMENTS
 * ============================================================================
 *
 * These rules provide explicit semantic grouping.
 *
 * They are intentionally unbounded by fixed counts.
 *
 * A requirement set can therefore scale from one requirement to arbitrarily
 * large programs, subject only to parser/compiler/runtime resources.
 */
requirementSet
    : requirementSetStart requirementSequence? requirementSetEnd
    ;

requirementSequence
    : requirementItem
    | requirementSequence requirementSeparator requirementItem
    ;

requirementItem
    : requirement
    | conditionalRequirement
    ;

requirementSetStart
    : requirementOpen
    ;

requirementSetEnd
    : requirementClose
    ;

requirementSeparator
    : requirementListSeparator
    ;


/*
 * ============================================================================
 * NAMED REQUIREMENTS
 * ============================================================================
 *
 * Names are useful for diagnostics, references, documentation, tooling,
 * configuration, and semantic provenance.
 *
 * The identifier syntax remains owned by the canonical expression/name
 * grammar. This rule does not create a second identifier definition.
 */
namedRequirement
    : requirementNamePrefix resourceRequirementName
      requirementNameAssignment requirementBody
    ;

resourceRequirementName
    : resourceIdentifier
    ;


/*
 * ============================================================================
 * REQUIREMENT GROUPS
 * ============================================================================
 *
 * Groups allow several requirements to be treated as one semantic unit.
 *
 * The group itself does not imply physical co-location or a particular
 * hardware topology.
 */
requirementGroup
    : requirementGroupPrefix
      requirementGroupBody
    ;

requirementGroupBody
    : requirementGroupOpen requirementItemSequence? requirementGroupClose
    ;

requirementItemSequence
    : requirementItem
    | requirementItemSequence requirementSeparator requirementItem
    ;


/*
 * ============================================================================
 * REQUIREMENT MODIFIERS
 * ============================================================================
 *
 * Modifiers are semantic qualifiers, not hardware directives.
 *
 * Examples of concepts that may eventually be represented by modifiers:
 *
 *   mandatory
 *   optional
 *   conditional
 *   inherited
 *   transitive
 *   local
 *   global
 *
 * The actual vocabulary must be established by the language specification
 * and canonical lexer. This grammar deliberately does not invent physical
 * device modifiers.
 */
requirementModifier
    : requirementMandatoryModifier
    | requirementOptionalModifier
    | requirementConditionalModifier
    | requirementInheritedModifier
    | requirementTransitiveModifier
    ;

requirementModifiers
    : requirementModifier
    | requirementModifiers requirementModifier
    ;


/*
 * ============================================================================
 * MODIFIED REQUIREMENTS
 * ============================================================================
 */
modifiedRequirement
    : requirementModifiers requirement
    ;


/*
 * ============================================================================
 * REQUIREMENT DECLARATION
 * ============================================================================
 *
 * This is the integration surface intended for resources.g4.
 *
 * resources.g4 can choose whether a requirement appears:
 *
 *   - directly in a resource declaration
 *   - inside a resource block
 *   - as a module-level requirement
 *   - as part of a target declaration
 *   - inside another domain's resource annotation
 *
 * This grammar does not decide placement.
 */
requirementDeclaration
    : requirement
    | namedRequirement
    | modifiedRequirement
    | requirementGroup
    ;


/*
 * ============================================================================
 * REQUIREMENT LIST
 * ============================================================================
 *
 * Unbounded recursive form deliberately avoids arbitrary maximum counts.
 */
requirementList
    : requirementDeclaration
    | requirementList requirementSeparator requirementDeclaration
    ;


/*
 * ============================================================================
 * SEMANTIC KEYWORD BOUNDARY
 * ============================================================================
 *
 * The following rules are intentionally aliases over canonical lexer tokens.
 *
 * They provide stable grammar-level names without allowing individual
 * resource files to invent their own keyword vocabulary.
 *
 * IMPORTANT:
 *   The token names below are integration placeholders unless they already
 *   exist in the repository's canonical lexer.
 *
 *   When integrating into the repository, these aliases MUST be mapped to
 *   the actual canonical tokens from lexer/tokens.g4 / lexer/keywords.g4.
 *
 *   They must NOT be implemented as independent lexer tokens in this file.
 * ============================================================================
 */


/*
 * Requirement declaration keyword.
 */
requirementKeyword
    : REQUIREMENT_KEYWORD
    ;


/*
 * Requirement aggregation keywords.
 */
requirementAllKeyword
    : REQUIREMENT_ALL_KEYWORD
    ;

requirementAnyKeyword
    : REQUIREMENT_ANY_KEYWORD
    ;

requirementOneOfKeyword
    : REQUIREMENT_ONE_OF_KEYWORD
    ;


/*
 * Conditional keyword.
 */
requirementIfKeyword
    : REQUIREMENT_IF_KEYWORD
    ;


/*
 * Named requirement syntax.
 */
requirementNamePrefix
    : REQUIREMENT_NAME_KEYWORD
    ;

requirementNameAssignment
    : REQUIREMENT_ASSIGN_OPERATOR
    ;


/*
 * Group syntax.
 */
requirementGroupPrefix
    : REQUIREMENT_GROUP_KEYWORD
    ;

requirementGroupOpen
    : REQUIREMENT_GROUP_OPEN
    ;

requirementGroupClose
    : REQUIREMENT_GROUP_CLOSE
    ;


/*
 * List syntax.
 */
requirementOpen
    : REQUIREMENT_OPEN
    ;

requirementClose
    : REQUIREMENT_CLOSE
    ;

requirementListSeparator
    : REQUIREMENT_SEPARATOR
    ;


/*
 * Modifiers.
 */
requirementMandatoryModifier
    : REQUIREMENT_MANDATORY_KEYWORD
    ;

requirementOptionalModifier
    : REQUIREMENT_OPTIONAL_KEYWORD
    ;

requirementConditionalModifier
    : REQUIREMENT_CONDITIONAL_KEYWORD
    ;

requirementInheritedModifier
    : REQUIREMENT_INHERITED_KEYWORD
    ;

requirementTransitiveModifier
    : REQUIREMENT_TRANSITIVE_KEYWORD
    ;


/*
 * ============================================================================
 * RESOURCE EXPRESSION / NAME BOUNDARY
 * ============================================================================
 *
 * These aliases deliberately defer expression and identifier ownership.
 *
 * The imported ResourceExpressions grammar must expose these public rules.
 *
 * No resource-specific primitive type is defined here.
 */
resourceIdentifier
    : identifier
    ;


/*
 * ============================================================================
 * END OF REQUIREMENTS GRAMMAR
 * ============================================================================
 *
 * Architectural invariants:
 *
 * 1. No machine size is encoded here.
 * 2. No hardware vendor is encoded here.
 * 3. No device ID is encoded here.
 * 4. No physical address is encoded here.
 * 5. No topology is encoded here.
 * 6. No qubit count is encoded here.
 * 7. No CPU/core/thread count is encoded here.
 * 8. No accelerator count is encoded here.
 * 9. No memory capacity is encoded here.
 * 10. No network size is encoded here.
 * 11. No scheduler policy is encoded here.
 * 12. No routing policy is encoded here.
 * 13. No allocation policy is encoded here.
 * 14. No runtime discovery is encoded here.
 * 15. No quantum IR is duplicated here.
 * 16. No classical IR is duplicated here.
 * 17. No hardware IR is duplicated here.
 *
 * The semantic pipeline is:
 *
 *   Source
 *      |
 *      v
 *   Lexer
 *      |
 *      v
 *   Requirements parser
 *      |
 *      v
 *   AST / syntax model
 *      |
 *      v
 *   Semantic requirement model
 *      |
 *      +--------------------+
 *      |                    |
 *      v                    v
 *   capability          constraint /
 *   evaluation          requirement solving
 *      |                    |
 *      +---------+----------+
 *                |
 *                v
 *          compilation context
 *                |
 *                v
 *       scheduling / routing /
 *       optimization / HAL
 *                |
 *                v
 *             runtime
 *
 * Resource requirements are therefore declarative inputs to the compiler
 * and runtime ecosystem, not implementation decisions.
 */