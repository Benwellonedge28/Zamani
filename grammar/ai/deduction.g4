/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/ai/deduction.g4
 *
 * Grammar:
 *     Deduction
 *
 * Status:
 *     CANONICAL AI / GENERIC REASONING LEAF GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1 or later
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of generic deductive reasoning.
 *
 * Deduction is a universal computational capability that may be used by:
 *
 *     - AI / machine learning;
 *     - symbolic reasoning;
 *     - scientific computing;
 *     - verification;
 *     - security analysis;
 *     - compiler analysis;
 *     - data processing;
 *     - classical computation;
 *     - quantum-classical hybrid computation;
 *     - distributed computation;
 *     - hardware/HDL verification;
 *     - future computational domains.
 *
 * The grammar describes the STRUCTURE of a deduction.
 *
 * It does not implement:
 *
 *     - theorem proving;
 *     - a particular inference engine;
 *     - a particular logic;
 *     - SAT/SMT solving;
 *     - constraint solving;
 *     - model execution;
 *     - machine learning;
 *     - probability calculation;
 *     - evidence verification;
 *     - provenance storage;
 *     - policy evaluation;
 *     - resource discovery;
 *     - hardware selection;
 *     - quantum routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     Zamani parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+---------+----------+----------+----------+
 *     |              |          |          |          |
 *     v              v          v          v          v
 *   types         effects   capabilities resources contracts
 *     |              |          |          |          |
 *     +--------------+----------+----------+----------+
 *                            |
 *                            v
 *                    reasoning semantics
 *                            |
 *                            v
 *                     canonical semantic IR
 *                            |
 *             +--------------+---------------+
 *             |                              |
 *             v                              v
 *       classical IR                     quantum::ir
 *             |                              |
 *             +--------------+---------------+
 *                            |
 *                            v
 *                   optimization/lowering
 *                            |
 *                            v
 *                    execution planning
 *                            |
 *                            v
 *                       target runtime
 *
 * Deduction is therefore a source-level construct, not an execution backend.
 *
 * ============================================================================
 * FILE CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *     - canonical Zamani lexer vocabulary through ZamaniLexer;
 *     - Types;
 *     - Expressions.
 *
 * EXPORTS:
 *     - deductionConstruct;
 *     - deductionDeclaration;
 *     - deductionInvocation;
 *     - deductionStatement;
 *     - deductionExpression;
 *     - deductionBody;
 *     - deductionPremise;
 *     - deductionConclusion;
 *     - deductionEvidence;
 *     - deductionAssumption;
 *     - deductionObservation;
 *     - deductionRule;
 *     - deductionCondition;
 *     - deductionMetadata.
 *
 * CONSUMED_BY:
 *     - grammar/ai/ai.g4;
 *     - grammar/ai/reasoning.g4;
 *     - grammar/ai/inference.g4 where integration is explicitly required;
 *     - grammar/statements/ where deduction is promoted as a statement;
 *     - grammar/expressions/ where deduction is promoted as an expression;
 *     - root composition through grammar/Zamani.g4.
 *
 * AST_OWNER:
 *     - src/frontend/ast/ and the repository's canonical frontend AST owner.
 *
 * SEMANTIC_OWNER:
 *     - AI/reasoning semantic analysis;
 *     - universal semantic analysis where deduction is used outside AI.
 *
 * IR_OWNER:
 *     - canonical semantic IR;
 *     - downstream classical IR or quantum::ir as appropriate.
 *
 * TEST_OWNER:
 *     - grammar/tests/ai/;
 *     - grammar/tests/semantic/;
 *     - grammar/tests/boundary/;
 *     - grammar/tests/scalability/.
 *
 * SPEC_OWNER:
 *     - grammar/spec/ai.md;
 *     - grammar/spec/semantics.md;
 *     - grammar/spec/provenance.md;
 *     - grammar/spec/resources.md;
 *     - grammar/spec/effects.md;
 *     - grammar/spec/type-system.md.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - deduction syntax;
 *     - deduction declarations;
 *     - deduction invocations;
 *     - deduction premises;
 *     - deduction conclusions;
 *     - deduction assumptions;
 *     - deduction observations;
 *     - deduction evidence references;
 *     - deduction rules;
 *     - deduction conditions;
 *     - deduction-local metadata;
 *     - deduction-local provenance references;
 *     - deduction-local explanation references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - general statements;
 *     - assertions;
 *     - contracts;
 *     - resources;
 *     - capabilities;
 *     - effects;
 *     - policies;
 *     - provenance semantics;
 *     - probability;
 *     - uncertainty;
 *     - learning;
 *     - inference;
 *     - induction;
 *     - abduction;
 *     - models;
 *     - datasets;
 *     - tensors;
 *     - agents;
 *     - actors;
 *     - quantum operations;
 *     - HDL constructs;
 *     - hardware topology;
 *     - target selection.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It composes:
 *
 *     grammar/lexer/
 *
 * This file MUST NOT define lexer rules.
 *
 * Existing lexical vocabulary consumed here includes:
 *
 *     DEDUCE
 *     PREMISE
 *     PREMISES
 *     CONCLUSION
 *     PROVE
 *     VERIFY
 *     VALIDATE
 *     ASSUMPTION
 *     OBSERVATION
 *     EVIDENCE
 *     EXPLAIN
 *     ASSERT
 *     FROM
 *     WHEN
 *     WHERE
 *     AS
 *     AT
 *     ASSIGN
 *     COLON
 *     COMMA
 *     SEMICOLON
 *     LEFT_PAREN / RIGHT_PAREN or their canonical parser-facing aliases
 *     LBRACE / RBRACE or their canonical parser-facing aliases
 *
 * The grammar MUST use the repository's actual canonical token names.
 *
 * In particular, this file does not introduce:
 *
 *     DEDUCTION
 *     DEDUCTION_BEGIN
 *     DEDUCTION_RULE
 *     PREMISE_EXPRESSION
 *     CONCLUSION_EXPRESSION
 *
 * as new lexer tokens.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions remain owned by:
 *
 *     grammar/expressions/
 *
 * This grammar consumes:
 *
 *     expression
 *
 * Expressions may therefore represent:
 *
 *     values;
 *     variables;
 *     calls;
 *     model outputs;
 *     tensor values;
 *     data values;
 *     quantum measurement results;
 *     classical computations;
 *     symbolic expressions;
 *     hardware-independent values;
 *     user-defined semantic objects.
 *
 * Deduction does not create a second expression language.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * General types remain owned by:
 *
 *     grammar/types/
 *
 * Deduction does not define a deduction-specific type system.
 *
 * Typed deduction declarations consume:
 *
 *     typeExpression
 *
 * Semantic analysis determines:
 *
 *     - whether a conclusion has an expected type;
 *     - whether premises are compatible;
 *     - whether a rule is applicable;
 *     - whether a proof/result is valid;
 *     - whether conversion is required.
 *
 * ============================================================================
 * OPEN-WORLD REASONING CONTRACT
 * ============================================================================
 *
 * Deduction MUST remain open-world.
 *
 * The grammar MUST NOT enumerate:
 *
 *     every logical operator;
 *     every theorem;
 *     every proof system;
 *     every solver;
 *     every inference engine;
 *     every mathematical domain;
 *     every scientific rule;
 *     every AI algorithm.
 *
 * The grammar represents deductive STRUCTURE.
 *
 * Concrete reasoning systems are selected through semantic models, libraries,
 * capabilities, policies, dialects, and implementations.
 *
 * ============================================================================
 * GENERIC DEDUCTION MODEL
 * ============================================================================
 *
 * The semantic shape is:
 *
 *     premises
 *        |
 *        v
 *     assumptions
 *        |
 *        v
 *     rules / conditions
 *        |
 *        v
 *     evidence / observations
 *        |
 *        v
 *     derivation
 *        |
 *        v
 *     conclusion
 *
 * A deduction may therefore express:
 *
 *     deduce conclusion from premise;
 *
 * or a structured form:
 *
 *     deduce result {
 *         premise ...
 *         assumption ...
 *         evidence ...
 *         rule ...
 *         conclusion ...
 *     }
 *
 * Exact semantic validity is determined downstream.
 *
 * ============================================================================
 * DECLARATION VERSUS INVOCATION
 * ============================================================================
 *
 * Declaration:
 *
 *     deduce Name {
 *         ...
 *     }
 *
 * Invocation:
 *
 *     deduce(expression);
 *
 * The declaration and invocation forms are intentionally distinguishable.
 *
 * A declaration requires a body.
 *
 * An invocation requires parentheses.
 *
 * This prevents arbitrary:
 *
 *     deduce identifier ...
 *
 * forms from becoming ambiguous with unrelated declarations.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

deductionConstruct
    : deductionDeclaration
    | deductionInvocation
    | deductionStatement
    | deductionExpression
    ;


/* ============================================================================
 * DEDUCTION INTRODUCER
 * ============================================================================
 *
 * DEDUCE is the unique lexical introducer for this feature.
 *
 * This is deliberate.
 *
 * Do NOT use:
 *
 *     AT identifier
 *
 * as the generic deduction introducer.
 *
 * That would conflict with annotations and unrelated AI constructs.
 *
 * The lexical token DEDUCE already exists in the canonical vocabulary.
 */

deductionIntroducer
    : DEDUCE
    ;


/* ============================================================================
 * DECLARATION
 * ============================================================================
 *
 * Canonical declaration:
 *
 *     deduce Name {
 *         ...
 *     }
 *
 * Optional type information is permitted:
 *
 *     deduce Name: Result {
 *         ...
 *     }
 *
 * Optional parameters:
 *
 *     deduce Name(a, b) {
 *         ...
 *     }
 *
 * The semantic layer determines whether the declaration is a valid reusable
 * deductive rule, derivation, theorem-like object, or reasoning procedure.
 */

deductionDeclaration
    : deductionIntroducer
      identifier
      deductionDeclarationParameters?
      deductionDeclarationType?
      deductionBody
    ;


deductionDeclarationParameters
    : LEFT_PAREN
      deductionParameterList?
      RIGHT_PAREN
    ;


deductionParameterList
    : deductionParameter
      (COMMA deductionParameter)*
    ;


deductionParameter
    : identifier
    | identifier COLON typeExpression
    | identifier ASSIGN expression
    ;


deductionDeclarationType
    : COLON
      typeExpression
    ;


/* ============================================================================
 * INVOCATION
 * ============================================================================
 *
 * Invocation is expression-oriented:
 *
 *     deduce(value);
 *
 *     deduce(premise, evidence);
 *
 *     deduce(premise, conclusion);
 *
 *     deduce(rule, value, evidence);
 *
 * The semantic model determines the meaning of each argument.
 *
 * No finite argument count is imposed.
 */

deductionInvocation
    : deductionIntroducer
      LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
      SEMICOLON?
    ;


deductionArgumentList
    : deductionArgument
      (COMMA deductionArgument)*
    ;


deductionArgument
    : deductionNamedArgument
    | expression
    ;


deductionNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * STATEMENT FORM
 * ============================================================================
 *
 * A direct deduction statement permits:
 *
 *     deduce conclusion from premise;
 *
 * Multiple premises may be represented by a structured premise list.
 *
 * The grammar intentionally uses expressions as semantic values.
 */

deductionStatement
    : deductionIntroducer
      deductionStatementPayload
      SEMICOLON?
    ;


deductionStatementPayload
    : deductionDirectDerivation
    | deductionStructuredStatement
    ;


deductionDirectDerivation
    : deductionConclusion
      deductionFromClause
      deductionPremiseSource
      deductionStatementModifier*
    ;


deductionStructuredStatement
    : deductionPremiseClause
      deductionEvidenceClause*
      deductionAssumptionClause*
      deductionRuleClause*
      deductionConditionClause*
      deductionConclusionClause
      deductionStatementModifier*
    ;


/* ============================================================================
 * EXPRESSION FORM
 * ============================================================================
 *
 * This form exists so deduction can participate in larger expressions without
 * requiring an AI-specific statement language.
 *
 * Example semantic intent:
 *
 *     result = deduce(value);
 *
 * The expression itself remains an ordinary Zamani value.
 */

deductionExpression
    : deductionIntroducer
      LEFT_PAREN
      deductionExpressionArgumentList?
      RIGHT_PAREN
    ;


deductionExpressionArgumentList
    : deductionExpressionArgument
      (COMMA deductionExpressionArgument)*
    ;


deductionExpressionArgument
    : deductionNamedArgument
    | expression
    ;


/* ============================================================================
 * BODY
 * ============================================================================
 *
 * A deduction body is a sequence of deduction members.
 *
 * The body is deliberately NOT a duplicate of the general statement grammar.
 *
 * If ordinary Zamani statements need to occur around a deduction, the
 * containing function/block owns those statements.
 *
 * This avoids:
 *
 *     Deduction -> Statements -> AI -> Deduction
 *
 * dependency cycles.
 */

deductionBody
    : LBRACE
      deductionMember*
      RBRACE
    ;


deductionMember
    : deductionPremiseClause
    | deductionEvidenceClause
    | deductionAssumptionClause
    | deductionObservationClause
    | deductionRuleClause
    | deductionConditionClause
    | deductionDerivationClause
    | deductionConclusionClause
    | deductionExplanationClause
    | deductionProvenanceClause
    | deductionMetadataClause
    | deductionDirective
    ;


/* ============================================================================
 * PREMISES
 * ============================================================================
 *
 * Premises are source facts/expressions from which the conclusion is derived.
 *
 * No finite number of premises is imposed.
 */

deductionPremiseClause
    : deductionPremiseIntroducer
      deductionPremiseList
      SEMICOLON?
    ;


deductionPremiseIntroducer
    : PREMISE
    | PREMISES
    ;


deductionPremiseList
    : deductionPremise
      (COMMA deductionPremise)*
    ;


deductionPremise
    : expression
    ;


/* ============================================================================
 * CONCLUSION
 * ============================================================================
 */

deductionConclusionClause
    : CONCLUSION
      deductionConclusion
      SEMICOLON?
    ;


deductionConclusion
    : expression
    ;


/* ============================================================================
 * DIRECT FROM CLAUSE
 * ============================================================================
 */

deductionFromClause
    : FROM
    ;


deductionPremiseSource
    : deductionPremiseList
    ;


/* ============================================================================
 * ASSUMPTIONS
 * ============================================================================
 *
 * Assumptions are not automatically facts.
 *
 * Semantic analysis must preserve the distinction between:
 *
 *     premise
 *     assumption
 *     observation
 *     evidence
 *     conclusion
 *
 * This distinction is important for verification and provenance.
 */

deductionAssumptionClause
    : ASSUMPTION
      deductionAssumption
      SEMICOLON?
    ;


deductionAssumption
    : expression
    ;


/* ============================================================================
 * OBSERVATIONS
 * ============================================================================
 */

deductionObservationClause
    : OBSERVATION
      deductionObservation
      SEMICOLON?
    ;


deductionObservation
    : expression
    ;


/* ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Evidence is represented as a semantic reference/value.
 *
 * This grammar does not determine whether evidence is:
 *
 *     trusted;
 *     verified;
 *     probabilistic;
 *     external;
 *     generated;
 *     experimental;
 *     cryptographically authenticated.
 *
 * Those properties belong to evidence/provenance semantics.
 */

deductionEvidenceClause
    : EVIDENCE
      deductionEvidence
      SEMICOLON?
    ;


deductionEvidence
    : expression
    ;


/* ============================================================================
 * RULES
 * ============================================================================
 *
 * A rule is represented generically.
 *
 * Do not enumerate:
 *
 *     modus ponens;
 *     modus tollens;
 *     syllogism;
 *     resolution;
 *     Horn clauses;
 *     first-order rules;
 *     temporal rules;
 *     domain-specific proof rules;
 *     future logical systems.
 *
 * Those are semantic/library concepts.
 */

deductionRuleClause
    : deductionRuleIntroducer
      deductionRule
      SEMICOLON?
    ;


deductionRuleIntroducer
    : identifier
    ;


deductionRule
    : expression
    ;


/* ============================================================================
 * CONDITIONS
 * ============================================================================
 *
 * Conditions constrain applicability of a deduction.
 *
 * Conditions are ordinary expressions.
 *
 * This prevents the deduction grammar from inventing a second predicate
 * language.
 */

deductionConditionClause
    : deductionConditionIntroducer
      deductionCondition
      SEMICOLON?
    ;


deductionConditionIntroducer
    : WHEN
    | WHERE
    ;


deductionCondition
    : expression
    ;


/* ============================================================================
 * DERIVATION
 * ============================================================================
 *
 * A derivation explicitly records an intermediate deductive result.
 *
 * Example:
 *
 *     derive intermediate = expression;
 *
 * The word "derive" remains an identifier unless it becomes a formally
 * reserved language keyword elsewhere.
 *
 * This keeps the lexical vocabulary open.
 */

deductionDerivationClause
    : deductionDerivationIntroducer
      deductionDerivationTarget
      ASSIGN
      expression
      SEMICOLON?
    ;


deductionDerivationIntroducer
    : identifier
    ;


deductionDerivationTarget
    : identifier
    ;


/* ============================================================================
 * EXPLANATION
 * ============================================================================
 *
 * Explanation syntax is deliberately generic.
 *
 * Explanation semantics are owned by the explanation/provenance subsystem.
 */

deductionExplanationClause
    : EXPLAIN
      deductionExplanationPayload?
      SEMICOLON?
    ;


deductionExplanationPayload
    : expression
    | LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    ;


/* ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Provenance is referenced, not implemented, here.
 */

deductionProvenanceClause
    : PROVENANCE
      deductionProvenancePayload?
      SEMICOLON?
    ;


deductionProvenancePayload
    : expression
    | LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    ;


/* ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata is intentionally open-ended.
 *
 * The metadata name remains an identifier.
 *
 * Examples of semantic metadata may include:
 *
 *     confidence
 *     source
 *     domain
 *     method
 *     version
 *     authority
 *     timestamp
 *
 * Their actual meaning belongs to the semantic metadata/provenance systems.
 */

deductionMetadataClause
    : AT
      deductionMetadataName
      deductionMetadataPayload?
      SEMICOLON?
    ;


deductionMetadataName
    : identifier
    ;


deductionMetadataPayload
    : LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    | COLON
      typeExpression
    | ASSIGN
      expression
    ;


/* ============================================================================
 * GENERIC DIRECTIVE
 * ============================================================================
 *
 * A deduction-specific directive is permitted as an extension point.
 *
 * This does NOT mean that every possible directive becomes a core keyword.
 *
 * Examples:
 *
 *     @method(...)
 *     @logic(...)
 *     @domain(...)
 *     @solver(...)
 *     @strategy(...)
 *
 * remain semantic registrations.
 *
 * The grammar does not hard-code their meanings.
 */

deductionDirective
    : AT
      deductionDirectiveName
      deductionDirectivePayload?
      SEMICOLON?
    ;


deductionDirectiveName
    : identifier
    ;


deductionDirectivePayload
    : LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    | COLON
      typeExpression
    | ASSIGN
      expression
    | deductionDirectiveRegion
    ;


deductionDirectiveRegion
    : LBRACE
      deductionMember*
      RBRACE
    ;


/* ============================================================================
 * STATEMENT MODIFIERS
 * ============================================================================
 *
 * Modifiers remain semantically open.
 *
 * The grammar does not turn every reasoning strategy into a keyword.
 *
 * Existing language keywords may be introduced here later only when their
 * lexical ownership and semantic specification are already established.
 */

deductionStatementModifier
    : deductionModifierName
      deductionModifierPayload?
    ;


deductionModifierName
    : identifier
    ;


deductionModifierPayload
    : LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    | ASSIGN
      expression
    | COLON
      typeExpression
    ;


/* ============================================================================
 * PROOF / VERIFICATION BOUNDARY
 * ============================================================================
 *
 * These forms provide stable parser anchors for proof-oriented deduction.
 *
 * They do not execute a proof.
 */

deductionProofClause
    : PROVE
      deductionProofTarget
      SEMICOLON?
    ;


deductionProofTarget
    : expression
    ;


deductionVerificationClause
    : VERIFY
      deductionVerificationTarget
      SEMICOLON?
    ;


deductionVerificationTarget
    : expression
    ;


deductionValidationClause
    : VALIDATE
      deductionValidationTarget
      SEMICOLON?
    ;


deductionValidationTarget
    : expression
    ;


/* ============================================================================
 * OPTIONAL PROOF MEMBERS
 * ============================================================================
 *
 * Keep proof/verification forms as explicit members so semantic analysis and
 * tooling can distinguish them from arbitrary directives.
 */

deductionProofMember
    : deductionProofClause
    | deductionVerificationClause
    | deductionValidationClause
    ;


/* ============================================================================
 * EXTENDED MEMBER COMPOSITION
 * ============================================================================
 *
 * This rule is intentionally separate from deductionMember so downstream
 * orchestrators can opt into proof-oriented deduction without changing the
 * base deduction syntax.
 */

deductionExtendedMember
    : deductionMember
    | deductionProofMember
    ;


/* ============================================================================
 * STRUCTURED STATEMENT WITH PROOF
 * ============================================================================
 */

deductionVerifiedStatement
    : deductionIntroducer
      LBRACE
      deductionExtendedMember*
      RBRACE
      SEMICOLON?
    ;


/* ============================================================================
 * NAMED DERIVATION FORM
 * ============================================================================
 *
 * A named result may be expressed without introducing a new lexer keyword.
 *
 * Example semantic shape:
 *
 *     deduce result from premise;
 *
 * The result remains an ordinary expression/identifier at the source level.
 */

deductionNamedDerivation
    : deductionIntroducer
      deductionNamedConclusion
      deductionFromClause
      deductionPremiseSource
      SEMICOLON?
    ;


deductionNamedConclusion
    : identifier
    | expression
    ;


/* ============================================================================
 * DEDUCTION CONTEXT
 * ============================================================================
 *
 * A deduction may have an explicitly named semantic context.
 *
 * "context" remains an identifier unless separately reserved by the canonical
 * lexical vocabulary.
 */

deductionContextClause
    : deductionContextName
      deductionContextPayload
      SEMICOLON?
    ;


deductionContextName
    : identifier
    ;


deductionContextPayload
    : expression
    | LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    ;


/* ============================================================================
 * DEDUCTION TARGET
 * ============================================================================
 *
 * A deduction target is an ordinary semantic expression.
 *
 * This allows deductions to target:
 *
 *     scalar values;
 *     structured values;
 *     propositions;
 *     symbolic expressions;
 *     tensors;
 *     model outputs;
 *     measurements;
 *     data records;
 *     compiler facts;
 *     hardware properties;
 *     distributed observations.
 */

deductionTarget
    : expression
    ;


/* ============================================================================
 * GENERAL DERIVATION FORM
 * ============================================================================
 *
 * This form is useful to semantic tooling that wants an explicit:
 *
 *     premises -> conclusion
 *
 * relationship.
 *
 * The arrow itself is NOT defined here.
 *
 * The canonical language may represent relationships using the existing
 * expression/operator system.
 */

deductionDerivation
    : deductionPremiseList
      deductionFromClause
      deductionConclusion
    ;


/* ============================================================================
 * DEDUCTION ARGUMENT GROUP
 * ============================================================================
 *
 * Generic grouping for semantic tooling.
 */

deductionArgumentGroup
    : LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    ;


/* ============================================================================
 * SOURCE / TARGET ASSOCIATION
 * ============================================================================
 *
 * "from" is the only dedicated lexical relation required for the canonical
 * direct-derivation form.
 */

deductionSourceTarget
    : deductionTarget
      deductionFromClause
      deductionPremiseSource
    ;


/* ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * Every list in this grammar uses repetition:
 *
 *     *
 *     +
 *
 * rather than fixed cardinalities.
 *
 * There is NO language-level limit on:
 *
 *     - number of deductions;
 *     - number of declarations;
 *     - number of premises;
 *     - number of conclusions;
 *     - number of assumptions;
 *     - number of observations;
 *     - number of evidence references;
 *     - number of derivation steps;
 *     - number of rules;
 *     - number of conditions;
 *     - number of arguments;
 *     - number of metadata entries;
 *     - deduction nesting depth;
 *     - reasoning graph size.
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_PREMISES
 *     MAX_CONCLUSIONS
 *     MAX_RULES
 *     MAX_EVIDENCE
 *     MAX_ASSUMPTIONS
 *     MAX_DEDUCTIONS
 *     MAX_REASONING_DEPTH
 *     MAX_ARGUMENTS
 *     MAX_CONTEXTS
 *
 * or equivalent constants.
 *
 * Practical limits are implementation/resource conditions only.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Deduction syntax MUST NOT encode:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     ASIC count;
 *     QPU count;
 *     accelerator count;
 *     node count;
 *     thread count;
 *     register width;
 *     memory capacity;
 *     tensor rank;
 *     network size;
 *     physical topology.
 *
 * Deduction may reason ABOUT such properties when they appear as semantic
 * values, but it does not prescribe their physical realization.
 *
 * Example:
 *
 *     deduce capability_available from observed_capability;
 *
 * is portable.
 *
 * A rule such as:
 *
 *     deduce run_on_gpu_3 ...
 *
 * would be target realization and must not become universal deduction syntax.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements are owned by:
 *
 *     grammar/core/requirements.g4
 *     grammar/resources/
 *
 * Deduction does not redefine:
 *
 *     requires
 *     capability
 *     constraint
 *     prefer
 *     hint
 *
 * A containing source construct may combine deduction with resource intent.
 *
 * Example semantic composition:
 *
 *     requires capability("reasoning.compute");
 *     deduce conclusion from premise;
 *
 * The deduction grammar does not resolve the capability.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Deduction itself is a semantic operation.
 *
 * The grammar does not decide whether a particular deduction is:
 *
 *     pure;
 *     deterministic;
 *     probabilistic;
 *     IO-producing;
 *     network-dependent;
 *     native;
 *     foreign;
 *     reflective;
 *     distributed;
 *     quantum-dependent.
 *
 * Semantic analysis determines effects from the actual operation and evidence
 * dependencies.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Deduction may consume uncertain/probabilistic values through ordinary
 * expressions.
 *
 * This file does NOT define:
 *
 *     probability;
 *     distribution;
 *     confidence;
 *     belief;
 *     Bayesian inference;
 *     probabilistic programming.
 *
 * Those belong to the AI/type/semantic subsystems.
 *
 * A deduction can therefore consume:
 *
 *     probability_value
 *     confidence_value
 *     distribution_value
 *
 * without creating a second probabilistic grammar.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence is structurally referenced here.
 *
 * Evidence semantics belong to:
 *
 *     grammar/ai/evidence.g4
 *     grammar/spec/provenance.md
 *
 * Semantic analysis must preserve:
 *
 *     evidence identity;
 *     source;
 *     derivation;
 *     trust state;
 *     verification state;
 *     provenance.
 *
 * The parser does not evaluate evidence.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Deduction is provenance-sensitive.
 *
 * The semantic representation should be capable of preserving:
 *
 *     source premises;
 *     assumptions;
 *     observations;
 *     evidence;
 *     rules;
 *     transformations;
 *     intermediate derivations;
 *     final conclusion;
 *     explanation;
 *     verification;
 *     semantic version;
 *     source span.
 *
 * Provenance storage and identity are downstream responsibilities.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Deduction may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * but this file does not redefine those constructs.
 *
 * The semantic model must distinguish:
 *
 *     assumption
 *
 * from:
 *
 *     contractual precondition;
 *
 * and:
 *
 *     deduction conclusion
 *
 * from:
 *
 *     verified postcondition.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may control:
 *
 *     which evidence may be used;
 *     which reasoning mechanisms may be selected;
 *     whether external evidence is trusted;
 *     whether network-backed evidence is permitted;
 *     whether probabilistic reasoning is permitted;
 *     whether nondeterministic reasoning is permitted;
 *     whether a deduction may invoke an external solver.
 *
 * This grammar does not grant policy permissions.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Deduction MUST NOT automatically grant:
 *
 *     filesystem access;
 *     network access;
 *     native execution;
 *     foreign calls;
 *     reflection;
 *     process creation;
 *     hardware access.
 *
 * If deduction requires such capabilities, the semantic effect/capability
 * system must represent those requirements explicitly.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Deduction may consume and produce ordinary classical values.
 *
 * No classical numerical syntax is duplicated here.
 *
 * Mathematical operations remain owned by:
 *
 *     grammar/classical/
 *     grammar/expressions/
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Deduction may reason over quantum-derived semantic values, including:
 *
 *     measurement results;
 *     state descriptions;
 *     symbolic quantum results;
 *     error/resilience observations;
 *     quantum capability information.
 *
 * This file does NOT define:
 *
 *     qubits;
 *     gates;
 *     circuits;
 *     routing;
 *     coupling maps;
 *     physical qubits;
 *     calibration;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * If a deduction produces or consumes quantum semantic operations, the
 * downstream boundary remains:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * There is no deduction-specific quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Deduction may reason about semantic HDL/hardware information, for example:
 *
 *     verification properties;
 *     timing observations;
 *     resource capabilities;
 *     synthesis results;
 *     hardware constraints.
 *
 * It does not define:
 *
 *     wires;
 *     registers;
 *     physical widths;
 *     clocks;
 *     FPGA resources;
 *     device identifiers;
 *     physical placement.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Deduction may consume distributed observations or evidence.
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/distributed/
 *     grammar/concurrency/
 *     grammar/networking/
 *
 * The deduction grammar does not create:
 *
 *     node IDs;
 *     worker IDs;
 *     process IDs;
 *     mailbox IDs;
 *     cluster limits.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Deduction and learning are related but distinct.
 *
 * Deduction:
 *
 *     derives a conclusion from supplied semantic inputs.
 *
 * Learning:
 *
 *     changes or produces a model/state from data or experience.
 *
 * This file MUST NOT absorb training or adaptation syntax.
 *
 * Learning remains owned by the AI learning subsystem.
 *
 * ============================================================================
 * INFERENCE INTEGRATION
 * ============================================================================
 *
 * Deduction is one reasoning mechanism.
 *
 * Inference is the broader computational operation that may invoke:
 *
 *     deduction;
 *     induction;
 *     abduction;
 *     probabilistic reasoning;
 *     learned models;
 *     symbolic systems.
 *
 * Therefore:
 *
 *     inference.g4
 *          |
 *          v
 *     semantic reasoning model
 *          |
 *          +--> deduction
 *          +--> induction
 *          +--> abduction
 *          +--> learned inference
 *
 * There must not be competing semantic definitions of deduction.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * reasoning.g4 is the higher-level reasoning composition layer.
 *
 * This file is the specialized deductive leaf.
 *
 * The dependency direction is:
 *
 *     reasoning
 *          |
 *          v
 *     deduction
 *
 * and NOT:
 *
 *     deduction
 *          |
 *          v
 *     reasoning
 *
 * if that would create a circular grammar dependency.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the domain-neutral AST to
 * represent at least:
 *
 *     DeductionDeclaration
 *     DeductionInvocation
 *     DeductionStatement
 *     DeductionExpression
 *     DeductionPremise
 *     DeductionAssumption
 *     DeductionObservation
 *     DeductionEvidence
 *     DeductionRule
 *     DeductionCondition
 *     DeductionDerivation
 *     DeductionConclusion
 *     DeductionExplanation
 *     DeductionProvenance
 *     DeductionMetadata
 *
 * The AST must preserve:
 *
 *     source span;
 *     source ordering;
 *     expression structure;
 *     declaration name;
 *     parameters;
 *     declared type;
 *     premises;
 *     assumptions;
 *     observations;
 *     evidence;
 *     rules;
 *     conditions;
 *     derivations;
 *     conclusions;
 *     metadata.
 *
 * The AST MUST NOT contain:
 *
 *     physical device IDs;
 *     selected GPU;
 *     selected CPU;
 *     physical qubit mapping;
 *     scheduler decisions;
 *     routing decisions;
 *     solver implementation objects;
 *     runtime handles.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether a deduction is well formed;
 *     - whether premises are valid;
 *     - whether assumptions are distinguished from facts;
 *     - whether evidence is valid;
 *     - whether evidence is available;
 *     - whether the selected reasoning mechanism is permitted;
 *     - whether rules are applicable;
 *     - whether conditions hold;
 *     - whether the conclusion follows under the selected semantics;
 *     - whether the result is deterministic;
 *     - whether uncertainty is preserved;
 *     - whether provenance is complete;
 *     - whether policies permit the deduction;
 *     - whether required capabilities exist;
 *     - whether required resources exist;
 *     - whether the operation is portable.
 *
 * The parser performs none of these checks.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type checking must verify:
 *
 *     premise compatibility;
 *     conclusion compatibility;
 *     rule input/output compatibility;
 *     parameter compatibility;
 *     evidence compatibility;
 *     context compatibility.
 *
 * A deduction may be polymorphic where the universal type system permits it.
 *
 * No fixed machine width is implied.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Semantic analysis must derive effects from the actual deduction graph.
 *
 * Examples:
 *
 *     pure deduction;
 *     external evidence access;
 *     network-backed evidence;
 *     probabilistic computation;
 *     native solver invocation;
 *     foreign reasoning service;
 *     distributed evidence aggregation.
 *
 * The grammar itself is effect-free.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Deduction may require semantic capabilities such as:
 *
 *     reasoning.symbolic
 *     reasoning.proof
 *     reasoning.verification
 *     reasoning.external_evidence
 *     reasoning.probabilistic
 *     reasoning.distributed
 *
 * These names are semantic capability identifiers.
 *
 * They are not lexer keywords and are not hardware identifiers.
 *
 * Capability satisfaction belongs to downstream analysis.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Deduction may consume resources including:
 *
 *     compute;
 *     memory;
 *     storage;
 *     communication;
 *     solver capacity;
 *     evidence access;
 *     execution time;
 *     energy;
 *     target-specific resources.
 *
 * No universal quantity is hard-coded by this grammar.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * A semantic deduction result should be able to preserve:
 *
 *     premises
 *     assumptions
 *     observations
 *     evidence
 *     rule
 *     derivation
 *     conclusion
 *     explanation
 *     verification
 *     source
 *
 * This permits reproducibility and auditability across:
 *
 *     AI;
 *     science;
 *     compilation;
 *     security;
 *     quantum;
 *     distributed execution;
 *     hardware realization.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Semantic deduction may be:
 *
 *     deterministic;
 *     nondeterministic;
 *     probabilistic;
 *     externally dependent;
 *
 * but such properties must be represented semantically.
 *
 * Runtime nondeterminism MUST NOT affect parsing.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The same deduction source must remain semantically meaningful regardless of
 * whether the eventual target is:
 *
 *     tiny embedded hardware;
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU-assisted system;
 *     simulator;
 *     HPC system;
 *     cluster;
 *     distributed system;
 *     cloud infrastructure;
 *     future computational hardware.
 *
 * Scaling is therefore resource-driven rather than grammar-driven.
 *
 * ============================================================================
 * NO HARD-CODING CONTRACT
 * ============================================================================
 *
 * This file contains no universal machine ceilings.
 *
 * It must not introduce:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It must also not introduce deduction-specific ceilings such as:
 *
 *     MAX_PREMISES
 *     MAX_RULES
 *     MAX_EVIDENCE
 *     MAX_CONCLUSIONS
 *     MAX_DEDUCTIONS
 *     MAX_REASONING_DEPTH
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no execution;
 *     - performs no proof search;
 *     - performs no solver invocation;
 *     - accesses no filesystem;
 *     - accesses no network;
 *     - accesses no hardware;
 *     - loads no model;
 *     - reads no dataset;
 *     - performs no dynamic reflection;
 *     - contains no embedded Rust;
 *     - contains no semantic predicates;
 *     - contains no unsafe code.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The grammar feeds the existing Rust frontend.
 *
 * The implementation baseline is:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *
 * Grammar changes must not require unsafe Rust.
 *
 * Semantic quantities must not silently inherit host-width limitations merely
 * because a Rust implementation happens to use a host index internally.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR directly.
 *
 * The semantic representation should describe deduction using generic
 * operations and relationships such as:
 *
 *     premise;
 *     assumption;
 *     evidence;
 *     observation;
 *     rule;
 *     derivation;
 *     conclusion;
 *     provenance;
 *     explanation.
 *
 * The canonical lowering path is:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic deduction
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +-------------------+
 *       |                   |
 *       v                   v
 *   classical IR        quantum::ir
 *
 * Quantum values encountered by deduction remain ordinary semantic values
 * until the quantum boundary is reached.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * No backend decision belongs here.
 *
 * Deduction may eventually be realized through:
 *
 *     software;
 *     symbolic engines;
 *     numerical engines;
 *     learned models;
 *     accelerators;
 *     distributed execution;
 *     quantum-assisted computation;
 *     hardware verification;
 *     future reasoning engines.
 *
 * The grammar does not choose among them.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling should be able to use this grammar to support:
 *
 *     syntax highlighting;
 *     parsing;
 *     AST navigation;
 *     semantic navigation;
 *     premise/conclusion inspection;
 *     provenance visualization;
 *     explanation visualization;
 *     diagnostics;
 *     refactoring;
 *     formatting;
 *     IDE/LSP support.
 *
 * Source spans must be preserved.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify structural failures such as:
 *
 *     - missing deduction body;
 *     - missing invocation delimiter;
 *     - malformed premise list;
 *     - malformed conclusion;
 *     - malformed evidence;
 *     - malformed assumption;
 *     - malformed rule;
 *     - malformed condition;
 *     - malformed metadata.
 *
 * Semantic diagnostics should separately identify:
 *
 *     - invalid reasoning;
 *     - unavailable evidence;
 *     - invalid rule;
 *     - incompatible premise;
 *     - invalid conclusion;
 *     - policy violation;
 *     - capability failure;
 *     - resource failure;
 *     - provenance failure;
 *     - nondeterminism violation;
 *     - portability failure.
 *
 * Parser and semantic diagnostics MUST remain distinguishable.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum, tests must cover:
 *
 *     deduce(value);
 *
 *     deduce(result from source);
 *
 *     deduce conclusion from premise;
 *
 *     deduce conclusion from premise_a, premise_b;
 *
 *     deduce theorem {
 *         premise fact;
 *         conclusion result;
 *     }
 *
 *     deduce theorem: Result {
 *         premise fact;
 *         evidence source;
 *         conclusion result;
 *     }
 *
 *     deduce theorem(x, y) {
 *         premise x;
 *         premise y;
 *         conclusion x;
 *     }
 *
 *     deduce {
 *         premise observation;
 *         assumption condition;
 *         evidence source;
 *         conclusion result;
 *     }
 *
 * Tests must also combine deduction with:
 *
 *     classical expressions;
 *     tensor values;
 *     data values;
 *     quantum measurement results;
 *     hybrid values;
 *     distributed observations;
 *     contracts;
 *     capabilities;
 *     resources;
 *     provenance;
 *     policies.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Reject structurally invalid forms such as:
 *
 *     deduce;
 *
 *     deduce();
 *
 * where an invocation with no arguments is not semantically supported by the
 * chosen semantic profile.
 *
 * Also test:
 *
 *     deduce name
 *
 * without a body;
 *
 * malformed premise clauses;
 *
 * malformed conclusion clauses;
 *
 * malformed evidence clauses;
 *
 * malformed parameter lists;
 *
 * malformed metadata.
 *
 * Semantic tests must separately reject invalid reasoning even when syntax is
 * valid.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Required cross-domain tests include:
 *
 *     deduction + classical computation;
 *     deduction + AI inference;
 *     deduction + learning output;
 *     deduction + uncertainty;
 *     deduction + evidence;
 *     deduction + provenance;
 *     deduction + contracts;
 *     deduction + policy;
 *     deduction + resource requirements;
 *     deduction + quantum measurement;
 *     deduction + hybrid execution;
 *     deduction + HDL verification;
 *     deduction + distributed observations;
 *     deduction + FFI-derived values.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that the grammar accepts arbitrarily large source-level
 * structures within the implementation's available resources.
 *
 * The test suite must generate:
 *
 *     many premises;
 *     many evidence references;
 *     many derivation steps;
 *     many rules;
 *     many conclusions;
 *     deeply nested deduction structures;
 *     wide reasoning graphs;
 *     large symbolic expressions.
 *
 * Tests must not define an artificial "maximum valid" deduction size.
 *
 * Resource exhaustion must be reported as an implementation/resource failure,
 * not as a language-level semantic limit.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical lexical tokens are reused.
 *
 * No replacement lexer vocabulary is introduced.
 *
 * Existing source programs using the canonical DEDUCE token should remain
 * compatible where their syntax conforms to the stabilized deduction
 * specification.
 *
 * Deprecated forms, if later discovered, must be handled through:
 *
 *     grammar/compatibility/
 *
 * rather than contaminating this canonical grammar with competing meanings.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. grammar/antlr/ZamaniLexer.g4
 *
 *    Already supplies the public token vocabulary.
 *
 *    No lexer rule is added here.
 *
 *
 * 2. grammar/ai/ai.g4
 *
 *    Import this grammar and expose deduction through the AI composition
 *    boundary.
 *
 *    The AI orchestrator owns composition; this file owns deduction syntax.
 *
 *
 * 3. grammar/ai/reasoning.g4
 *
 *    Consume deductionConstruct as the deductive reasoning leaf.
 *
 *    reasoning.g4 must not redefine deduction.
 *
 *
 * 4. grammar/ai/inference.g4
 *
 *    Inference may reference deduction semantically.
 *
 *    It must not create a second deduction grammar.
 *
 *
 * 5. grammar/expressions/
 *
 *    If deductionExpression is exposed by the canonical expression
 *    orchestrator, it should be imported through the expression integration
 *    boundary rather than duplicated.
 *
 *
 * 6. grammar/statements/
 *
 *    If deductionStatement becomes a canonical general statement, the
 *    statement orchestrator should consume this public rule.
 *
 *    This file remains the owner of the deduction syntax itself.
 *
 *
 * 7. grammar/validation/
 *
 *    Contracts such as requires/ensures/invariant remain owned there.
 *
 *    Deduction only participates semantically.
 *
 *
 * 8. grammar/resources/
 *
 *    Resource requirements and capabilities remain owned there.
 *
 *
 * 9. grammar/effects/
 *
 *    Effects remain owned there.
 *
 *
 * 10. grammar/ai/evidence.g4
 *
 *     Evidence semantics remain owned there.
 *
 *
 * 11. grammar/ai/provenance.g4
 *
 *     Provenance semantics remain owned there.
 *
 *
 * 12. grammar/ai/explanations.g4
 *
 *     Explanation semantics remain owned there.
 *
 *
 * 13. grammar/ai/inference.g4
 *
 *     Inference remains the broader operation.
 *
 *
 * 14. grammar/quantum/
 *
 *     Quantum syntax remains owned there.
 *
 *
 * 15. grammar/hybrid/
 *
 *     Hybrid orchestration remains owned there.
 *
 *
 * 16. grammar/distributed/
 *
 *     Distributed semantics remain owned there.
 *
 *
 * 17. grammar/security/
 *
 *     Authorization and sandboxing remain owned there.
 *
 * ============================================================================
 * INTEGRATION ORDER
 * ============================================================================
 *
 * This file should be completed independently before modifying higher-level
 * composition files.
 *
 * Recommended order:
 *
 *     1. deduction.g4
 *     2. deduction parser tests
 *     3. deduction AST mapping
 *     4. deduction semantic model
 *     5. reasoning.g4 integration
 *     6. ai.g4 integration
 *     7. root Zamani.g4 integration
 *     8. semantic/effect/capability/resource integration
 *     9. cross-domain tests
 *
 * This prevents deduction.g4 from being repeatedly rewritten because another
 * AI grammar file changes.
 *
 * ============================================================================
 * INDEPENDENT COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] DEDUCE is consumed from the canonical lexer.
 *     [ ] No lexer rules exist in this file.
 *     [ ] Deduction declaration syntax is unambiguous.
 *     [ ] Deduction invocation syntax is unambiguous.
 *     [ ] Premises are represented generically.
 *     [ ] Conclusions are represented generically.
 *     [ ] Assumptions are represented distinctly.
 *     [ ] Observations are represented distinctly.
 *     [ ] Evidence is represented distinctly.
 *     [ ] Rules are represented generically.
 *     [ ] Conditions are represented generically.
 *     [ ] Derivations are represented.
 *     [ ] Explanation is represented as an integration boundary.
 *     [ ] Provenance is represented as an integration boundary.
 *     [ ] Metadata is extensible.
 *     [ ] General expressions are reused.
 *     [ ] General types are reused.
 *     [ ] No duplicate resource grammar exists.
 *     [ ] No duplicate capability grammar exists.
 *     [ ] No duplicate effect grammar exists.
 *     [ ] No duplicate contract grammar exists.
 *     [ ] No duplicate policy grammar exists.
 *     [ ] No duplicate quantum grammar exists.
 *     [ ] No hardware topology is encoded.
 *     [ ] No physical target is selected.
 *     [ ] No artificial scalability ceiling exists.
 *     [ ] No parser-time execution exists.
 *     [ ] No semantic predicates are required.
 *     [ ] No embedded Rust exists.
 *     [ ] No unsafe Rust is required.
 *     [ ] Source spans can be preserved.
 *     [ ] AST mapping is defined.
 *     [ ] Semantic mapping is defined.
 *     [ ] Canonical IR mapping is defined.
 *     [ ] Classical integration is defined.
 *     [ ] quantum::ir integration is defined.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Scalability tests exist.
 *     [ ] Determinism tests exist.
 *     [ ] Compatibility tests exist.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Deduction describes:
 *
 *     "what follows from what"
 *
 * It does NOT describe:
 *
 *     "which machine performs it".
 *
 * Therefore:
 *
 *     premises
 *       +
 *     assumptions
 *       +
 *     evidence
 *       +
 *     rules
 *       +
 *     conditions
 *       |
 *       v
 *     deduction semantic model
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *   classical              quantum::ir
 *       |                    |
 *       +---------+----------+
 *                 |
 *                 v
 *          target-independent
 *             optimization
 *                 |
 *                 v
 *          target realization
 *
 * This is the required property for POCO-REAF:
 *
 * the deduction source expresses portable computational meaning while
 * compilation, resource negotiation, capability resolution, optimization,
 * scheduling, routing, resilience and target realization determine how that
 * meaning is executed.
 *
 * ============================================================================
 */
 
parser grammar Deduction;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions;


/* ============================================================================
 * PUBLIC CONSTRUCT
 * ========================================================================== */

deductionConstruct
    : deductionDeclaration
    | deductionInvocation
    | deductionStatement
    | deductionExpression
    ;


/* ============================================================================
 * INTRODUCER
 * ========================================================================== */

deductionIntroducer
    : DEDUCE
    ;


/* ============================================================================
 * DECLARATION
 * ========================================================================== */

deductionDeclaration
    : deductionIntroducer
      identifier
      deductionDeclarationParameters?
      deductionDeclarationType?
      deductionBody
    ;


deductionDeclarationParameters
    : LEFT_PAREN
      deductionParameterList?
      RIGHT_PAREN
    ;


deductionParameterList
    : deductionParameter
      (COMMA deductionParameter)*
    ;


deductionParameter
    : identifier
    | identifier COLON typeExpression
    | identifier ASSIGN expression
    ;


deductionDeclarationType
    : COLON
      typeExpression
    ;


deductionBody
    : LBRACE
      deductionMember*
      RBRACE
    ;


/* ============================================================================
 * INVOCATION
 * ========================================================================== */

deductionInvocation
    : deductionIntroducer
      LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
      SEMICOLON?
    ;


deductionArgumentList
    : deductionArgument
      (COMMA deductionArgument)*
    ;


deductionArgument
    : deductionNamedArgument
    | expression
    ;


deductionNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * EXPRESSION FORM
 * ========================================================================== */

deductionExpression
    : deductionIntroducer
      LEFT_PAREN
      deductionExpressionArgumentList?
      RIGHT_PAREN
    ;


deductionExpressionArgumentList
    : deductionExpressionArgument
      (COMMA deductionExpressionArgument)*
    ;


deductionExpressionArgument
    : deductionNamedArgument
    | expression
    ;


/* ============================================================================
 * STATEMENT FORM
 * ========================================================================== */

deductionStatement
    : deductionIntroducer
      deductionStatementPayload
      SEMICOLON?
    ;


deductionStatementPayload
    : deductionDirectDerivation
    | deductionStructuredStatement
    ;


deductionDirectDerivation
    : deductionConclusion
      deductionFromClause
      deductionPremiseSource
      deductionStatementModifier*
    ;


deductionStructuredStatement
    : deductionPremiseClause
      deductionEvidenceClause*
      deductionAssumptionClause*
      deductionObservationClause*
      deductionRuleClause*
      deductionConditionClause*
      deductionDerivationClause*
      deductionConclusionClause
      deductionStatementModifier*
    ;


/* ============================================================================
 * PREMISES
 * ========================================================================== */

deductionPremiseClause
    : deductionPremiseIntroducer
      deductionPremiseList
      SEMICOLON?
    ;


deductionPremiseIntroducer
    : PREMISE
    | PREMISES
    ;


deductionPremiseList
    : deductionPremise
      (COMMA deductionPremise)*
    ;


deductionPremise
    : expression
    ;


/* ============================================================================
 * CONCLUSION
 * ========================================================================== */

deductionConclusionClause
    : CONCLUSION
      deductionConclusion
      SEMICOLON?
    ;


deductionConclusion
    : expression
    ;


/* ============================================================================
 * DIRECT FROM RELATION
 * ========================================================================== */

deductionFromClause
    : FROM
    ;


deductionPremiseSource
    : deductionPremiseList
    ;


/* ============================================================================
 * ASSUMPTIONS
 * ========================================================================== */

deductionAssumptionClause
    : ASSUMPTION
      deductionAssumption
      SEMICOLON?
    ;


deductionAssumption
    : expression
    ;


/* ============================================================================
 * OBSERVATIONS
 * ========================================================================== */

deductionObservationClause
    : OBSERVATION
      deductionObservation
      SEMICOLON?
    ;


deductionObservation
    : expression
    ;


/* ============================================================================
 * EVIDENCE
 * ========================================================================== */

deductionEvidenceClause
    : EVIDENCE
      deductionEvidence
      SEMICOLON?
    ;


deductionEvidence
    : expression
    ;


/* ============================================================================
 * RULES
 * ========================================================================== */

deductionRuleClause
    : deductionRuleIntroducer
      deductionRule
      SEMICOLON?
    ;


deductionRuleIntroducer
    : identifier
    ;


deductionRule
    : expression
    ;


/* ============================================================================
 * CONDITIONS
 * ========================================================================== */

deductionConditionClause
    : deductionConditionIntroducer
      deductionCondition
      SEMICOLON?
    ;


deductionConditionIntroducer
    : WHEN
    | WHERE
    ;


deductionCondition
    : expression
    ;


/* ============================================================================
 * DERIVATION
 * ========================================================================== */

deductionDerivationClause
    : deductionDerivationIntroducer
      deductionDerivationTarget
      ASSIGN
      expression
      SEMICOLON?
    ;


deductionDerivationIntroducer
    : identifier
    ;


deductionDerivationTarget
    : identifier
    ;


/* ============================================================================
 * EXPLANATION
 * ========================================================================== */

deductionExplanationClause
    : EXPLAIN
      deductionExplanationPayload?
      SEMICOLON?
    ;


deductionExplanationPayload
    : expression
    | LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    ;


/* ============================================================================
 * PROVENANCE
 * ========================================================================== */

deductionProvenanceClause
    : PROVENANCE
      deductionProvenancePayload?
      SEMICOLON?
    ;


deductionProvenancePayload
    : expression
    | LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    ;


/* ============================================================================
 * METADATA
 * ========================================================================== */

deductionMetadataClause
    : AT
      deductionMetadataName
      deductionMetadataPayload?
      SEMICOLON?
    ;


deductionMetadataName
    : identifier
    ;


deductionMetadataPayload
    : LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    | COLON
      typeExpression
    | ASSIGN
      expression
    ;


/* ============================================================================
 * OPEN DEDUCTION DIRECTIVE
 * ========================================================================== */

deductionDirective
    : AT
      deductionDirectiveName
      deductionDirectivePayload?
      SEMICOLON?
    ;


deductionDirectiveName
    : identifier
    ;


deductionDirectivePayload
    : LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    | COLON
      typeExpression
    | ASSIGN
      expression
    | deductionDirectiveRegion
    ;


deductionDirectiveRegion
    : LBRACE
      deductionMember*
      RBRACE
    ;


/* ============================================================================
 * STATEMENT MODIFIERS
 * ========================================================================== */

deductionStatementModifier
    : deductionModifierName
      deductionModifierPayload?
    ;


deductionModifierName
    : identifier
    ;


deductionModifierPayload
    : LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    | ASSIGN
      expression
    | COLON
      typeExpression
    ;


/* ============================================================================
 * PROOF / VERIFICATION BOUNDARIES
 * ========================================================================== */

deductionProofClause
    : PROVE
      deductionProofTarget
      SEMICOLON?
    ;


deductionProofTarget
    : expression
    ;


deductionVerificationClause
    : VERIFY
      deductionVerificationTarget
      SEMICOLON?
    ;


deductionVerificationTarget
    : expression
    ;


deductionValidationClause
    : VALIDATE
      deductionValidationTarget
      SEMICOLON?
    ;


deductionValidationTarget
    : expression
    ;


deductionProofMember
    : deductionProofClause
    | deductionVerificationClause
    | deductionValidationClause
    ;


deductionExtendedMember
    : deductionMember
    | deductionProofMember
    ;


deductionVerifiedStatement
    : deductionIntroducer
      LBRACE
      deductionExtendedMember*
      RBRACE
      SEMICOLON?
    ;


/* ============================================================================
 * NAMED DERIVATION
 * ========================================================================== */

deductionNamedDerivation
    : deductionIntroducer
      deductionNamedConclusion
      deductionFromClause
      deductionPremiseSource
      SEMICOLON?
    ;


deductionNamedConclusion
    : identifier
    | expression
    ;


/* ============================================================================
 * CONTEXT
 * ========================================================================== */

deductionContextClause
    : deductionContextName
      deductionContextPayload
      SEMICOLON?
    ;


deductionContextName
    : identifier
    ;


deductionContextPayload
    : expression
    | LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    ;


/* ============================================================================
 * GENERIC TARGET
 * ========================================================================== */

deductionTarget
    : expression
    ;


/* ============================================================================
 * GENERIC DERIVATION
 * ========================================================================== */

deductionDerivation
    : deductionPremiseList
      deductionFromClause
      deductionConclusion
    ;


deductionArgumentGroup
    : LEFT_PAREN
      deductionArgumentList?
      RIGHT_PAREN
    ;


deductionSourceTarget
    : deductionTarget
      deductionFromClause
      deductionPremiseSource
    ;


/* ============================================================================
 * MEMBER COMPOSITION
 * ========================================================================== */

deductionMember
    : deductionPremiseClause
    | deductionEvidenceClause
    | deductionAssumptionClause
    | deductionObservationClause
    | deductionRuleClause
    | deductionConditionClause
    | deductionDerivationClause
    | deductionConclusionClause
    | deductionExplanationClause
    | deductionProvenanceClause
    | deductionMetadataClause
    | deductionDirective
    ;