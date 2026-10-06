/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/ai/abduction.g4
 *
 * Grammar:
 *     Abduction
 *
 * Status:
 *     CANONICAL ABDUCTIVE-REASONING LEAF GRAMMAR
 *
 * Baseline:
 *     Rust 1.97+
 *     Rust edition 2021
 *     Safe Rust only
 *     No unsafe Rust
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-level syntax for ABDUCTIVE reasoning.
 *
 * Abduction represents reasoning from observations/evidence and a desired
 * or observed effect toward one or more possible explanatory hypotheses.
 *
 * Conceptually:
 *
 *     observation / evidence
 *             +
 *        background
 *          knowledge
 *             +
 *          criteria
 *             |
 *             v
 *        hypothesis space
 *             |
 *             v
 *       candidate explanation
 *
 * This grammar describes the STRUCTURE of that computation.
 *
 * It does not implement:
 *
 *     - an abductive solver;
 *     - a theorem prover;
 *     - Bayesian inference;
 *     - probabilistic inference;
 *     - SAT/SMT solving;
 *     - machine learning;
 *     - a knowledge database;
 *     - a particular logic;
 *     - a particular search algorithm;
 *     - ranking algorithms;
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
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Abduction is a REASONING FORM, not a target architecture.
 *
 * Therefore the source program describes:
 *
 *     observations
 *     evidence
 *     background knowledge
 *     hypotheses
 *     criteria
 *     conclusions
 *
 * while downstream semantic analysis determines how that meaning can be
 * realized.
 *
 * The compilation boundary is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +-------------------+-------------------+
 *       |                   |                   |
 *       v                   v                   v
 *     types              effects            resources
 *       |                   |                   |
 *       +-------------------+-------------------+
 *                           |
 *                           v
 *                   semantic reasoning
 *                           |
 *                           v
 *                  canonical semantic model
 *                           |
 *             +-------------+-------------+
 *             |                           |
 *             v                           v
 *        classical                    quantum::ir
 *             |                           |
 *             +-------------+-------------+
 *                           |
 *                           v
 *                  optimization/lowering
 *                           |
 *                           v
 *                  target-independent plan
 *                           |
 *                           v
 *                 scheduling/routing/etc.
 *                           |
 *                           v
 *                      ZQN / HAL
 *                           |
 *                           v
 *                    target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - abductionConstruct;
 *     - abductionDeclaration;
 *     - abductionInvocation;
 *     - abductionBody;
 *     - abductive observations;
 *     - abductive evidence references;
 *     - abductive background/context references;
 *     - abductive hypothesis specifications;
 *     - abductive criteria;
 *     - abductive explanation targets;
 *     - abductive result specifications;
 *     - abductive derivation members;
 *     - abduction-local metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - general statements;
 *     - inference;
 *     - deduction;
 *     - induction;
 *     - reasoning orchestration;
 *     - knowledge storage;
 *     - assertions;
 *     - retraction;
 *     - queries;
 *     - probability;
 *     - uncertainty;
 *     - confidence semantics;
 *     - evidence semantics;
 *     - provenance semantics;
 *     - explanations;
 *     - decisions;
 *     - contracts;
 *     - policies;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - actors;
 *     - concurrency;
 *     - quantum operations;
 *     - HDL;
 *     - hardware;
 *     - networking;
 *     - distributed execution;
 *     - FFI;
 *     - ABI;
 *     - metaprogramming;
 *     - IR;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the ONLY canonical source grammar for the abductive reasoning
 * construct.
 *
 * The following grammars MUST NOT redefine abduction:
 *
 *     grammar/ai/reasoning.g4
 *     grammar/ai/inference.g4
 *     grammar/ai/deduction.g4
 *     grammar/ai/induction.g4
 *     grammar/expressions/reasoning.g4
 *     grammar/statements/reason.g4
 *     grammar/statements/infer.g4
 *
 * Those grammars may compose or reference the public rules exported here,
 * but they must not create competing syntax.
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * The grammar MUST remain open-ended.
 *
 * It MUST NOT enumerate:
 *
 *     - abductive algorithms;
 *     - search strategies;
 *     - logical systems;
 *     - probability models;
 *     - scoring algorithms;
 *     - domain-specific hypothesis types;
 *     - scientific disciplines;
 *     - AI model families;
 *     - solver implementations.
 *
 * New reasoning systems must be implementable through:
 *
 *     semantic models
 *     libraries
 *     dialects
 *     capabilities
 *     policies
 *     runtime implementations
 *
 * without modifying this universal grammar.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/types/
 *     grammar/expressions/expressions.g4
 *
 * EXPORTS:
 *
 *     abductionConstruct
 *     abductionDeclaration
 *     abductionInvocation
 *     abductionBody
 *     abductionMember
 *     abductiveObservation
 *     abductiveEvidence
 *     abductiveBackground
 *     abductiveHypothesis
 *     abductiveCriterion
 *     abductiveExplanation
 *     abductiveResult
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *     grammar/ai/reasoning.g4
 *     future universal reasoning composition
 *
 * AST_OWNER:
 *
 *     canonical domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     universal reasoning semantic subsystem
 *     with AI-domain integration where applicable
 *
 * IR_OWNER:
 *
 *     canonical semantic IR
 *
 *     Downstream realization may use:
 *
 *         classical IR
 *         quantum::ir
 *         hybrid semantic IR
 *         HDL/hardware lowering
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/abduction/
 *     grammar/tests/semantic/reasoning/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/spec/semantics.md
 *     grammar/spec/provenance.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * The canonical token vocabulary is:
 *
 *     ZamaniLexer
 *
 * The preferred architecture is to reserve only language-level vocabulary
 * that is genuinely required.
 *
 * This grammar intentionally does not introduce a token for every possible
 * abductive concept.
 *
 * The only dedicated introducer required here is:
 *
 *     ABDUCE
 *
 * If the repository's lexer has not yet promoted ABDUCE to canonical lexical
 * vocabulary, that token must be added in the lexical authority before this
 * grammar is enabled in the canonical parser build.
 *
 * Do NOT create:
 *
 *     ABDUCTION
 *     HYPOTHESIS_EXPRESSION
 *     OBSERVATION_EXPRESSION
 *     EVIDENCE_EXPRESSION
 *     ABDUCTIVE_RULE
 *
 * lexer tokens.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Expressions remain owned by:
 *
 *     grammar/expressions/
 *
 * This file consumes:
 *
 *     expression
 *
 * An abductive observation, hypothesis, criterion, evidence reference, or
 * explanation target may therefore be any valid Zamani expression.
 *
 * This permits the same abductive structure to operate over:
 *
 *     classical values
 *     tensors
 *     data
 *     knowledge
 *     model outputs
 *     measurements
 *     quantum results
 *     hardware observations
 *     distributed observations
 *     simulation results
 *     symbolic values
 *     user-defined values
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * General type syntax remains owned by:
 *
 *     grammar/types/
 *
 * This grammar consumes:
 *
 *     typeExpression
 *
 * No abductive-specific type system is introduced.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * This file does NOT define:
 *
 *     requires
 *     capability
 *     resource
 *     budget
 *     topology
 *     device
 *     placement
 *     scheduling
 *
 * Those are owned by:
 *
 *     grammar/resources/
 *
 * Abductive operations may semantically carry resource requirements and
 * capability requirements through the canonical semantic model.
 *
 * Example semantic relationship:
 *
 *     abduction
 *         |
 *         +--> capability requirement
 *         +--> resource requirement
 *         +--> effect requirement
 *         +--> policy
 *
 * No physical capacity is encoded here.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Effects are owned by:
 *
 *     grammar/effects/
 *
 * Abduction itself does not declare an implementation-specific effect.
 *
 * Semantic analysis may determine that a particular abductive operation
 * requires effects such as:
 *
 *     knowledge access
 *     external data access
 *     randomness
 *     model execution
 *     network access
 *     native/foreign execution
 *
 * Those effects are attached by semantic analysis rather than duplicated
 * inside this grammar.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * requires / ensures / invariant / assume / guarantee / property remain owned
 * by:
 *
 *     grammar/validation/
 *
 * This grammar therefore does not redefine contract syntax.
 *
 * Abduction may participate in contracts through the semantic model.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies remain owned by:
 *
 *     grammar/policies/
 *
 * Abduction may be constrained by:
 *
 *     authorization
 *     trust
 *     evidence requirements
 *     resource policy
 *     execution policy
 *     adaptation policy
 *
 * The grammar does not duplicate those policy constructs.
 *
 * ============================================================================
 * EVIDENCE / PROVENANCE CONTRACT
 * ============================================================================
 *
 * This file recognizes an abductive evidence boundary, but does not define
 * evidence storage or provenance semantics.
 *
 * Canonical semantic ownership remains with:
 *
 *     grammar/ai/evidence.g4
 *     grammar/ai/provenance.g4
 *
 * and their universal semantic counterparts.
 *
 * The semantic model must preserve relationships such as:
 *
 *     hypothesis
 *       <- supported_by <- evidence
 *       <- derived_from <- observation
 *       <- constrained_by <- background
 *
 * ============================================================================
 * DECLARATION MODEL
 * ============================================================================
 *
 * A reusable abductive specification has the form:
 *
 *     abduce Name {
 *         observation ...
 *         evidence ...
 *         background ...
 *         hypothesis ...
 *         criterion ...
 *         explanation ...
 *         result ...
 *     }
 *
 * The declaration name is an ordinary identifier.
 *
 * The body remains intentionally structured but open-ended.
 *
 * ============================================================================
 * INVOCATION MODEL
 * ============================================================================
 *
 * A direct abductive invocation has the form:
 *
 *     abduce(...)
 *
 * Named arguments are supported:
 *
 *     abduce(
 *         observation = value,
 *         evidence = evidence_value,
 *         hypothesis = candidate
 *     )
 *
 * The grammar imposes no fixed argument count.
 *
 * This is important for scalability and future extensions.
 *
 * ============================================================================
 * WHY NAMED FIELDS ARE OPEN-WORLD
 * ============================================================================
 *
 * Abduction naturally evolves as different domains introduce additional
 * semantic information.
 *
 * Examples include:
 *
 *     observation
 *     evidence
 *     background
 *     hypothesis
 *     hypotheses
 *     criterion
 *     criteria
 *     explanation
 *     result
 *     confidence
 *     ranking
 *     provenance_ref
 *     policy
 *     model
 *     context
 *
 * The grammar must not require a lexer token for every such field.
 *
 * Therefore body field names are ordinary identifiers.
 *
 * Semantic analysis determines which fields are standard, optional, required,
 * incompatible, or supplied by a dialect.
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This grammar contains no universal limits for:
 *
 *     number of observations
 *     number of evidence items
 *     number of hypotheses
 *     number of criteria
 *     number of derivation steps
 *     expression size
 *     nesting depth
 *     graph width
 *     graph size
 *     model size
 *     data size
 *     machine size
 *     memory size
 *     processor count
 *     accelerator count
 *     QPU count
 *     qubit count
 *     node count
 *
 * Any actual implementation limit is a resource/runtime property rather than
 * a language-level semantic ceiling.
 *
 * ============================================================================
 */

parser grammar Abduction;

options {
    tokenVocab = ZamaniLexer;
}

import
    Types,
    Expressions
;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The AI composition grammar should consume this rule.
 *
 * There is exactly one canonical public abduction entry point.
 */

abductionConstruct
    : abductionDeclaration
    | abductionInvocation
    ;


/*
 * ============================================================================
 * INTRODUCER
 * ============================================================================
 *
 * ABDUCE is the unique lexical introducer.
 *
 * Do not use:
 *
 *     AT identifier
 *
 * here.
 *
 * Generic annotations have unrelated ownership and must remain distinguishable
 * from an executable abductive construct.
 */

abductionIntroducer
    : ABDUCE
    ;


/*
 * ============================================================================
 * DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     abduce Name {
 *         ...
 *     }
 *
 *     abduce Name(parameters) {
 *         ...
 *     }
 *
 *     abduce Name: ResultType {
 *         ...
 *     }
 *
 * A declaration requires a body.
 *
 * This prevents ambiguity with invocation syntax.
 */

abductionDeclaration
    : abductionIntroducer
      identifier
      abductionDeclarationParameters?
      abductionDeclarationType?
      abductionBody
    ;


abductionDeclarationParameters
    : LEFT_PAREN
      abductionParameterList?
      RIGHT_PAREN
    ;


abductionParameterList
    : abductionParameter
      (COMMA abductionParameter)*
    ;


abductionParameter
    : identifier
    | identifier COLON typeExpression
    | identifier ASSIGN expression
    ;


abductionDeclarationType
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * INVOCATION
 * ============================================================================
 *
 * Canonical invocation:
 *
 *     abduce(...)
 *
 * A parenthesized form is deliberately used so invocation cannot be confused
 * with declarations, ordinary identifiers, or annotations.
 */

abductionInvocation
    : abductionIntroducer
      LEFT_PAREN
      abductionArgumentList?
      RIGHT_PAREN
      SEMICOLON?
    ;


abductionArgumentList
    : abductionArgument
      (COMMA abductionArgument)*
    ;


abductionArgument
    : abductionNamedArgument
    | expression
    ;


abductionNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 *
 * The body is a sequence of abductive members.
 *
 * It is deliberately NOT a copy of the general statement grammar.
 *
 * Ordinary computation surrounding an abductive operation remains owned by
 * the containing Zamani block/function/module.
 */

abductionBody
    : LBRACE
      abductionMember*
      RBRACE
    ;


abductionMember
    : abductiveObservationClause
    | abductiveEvidenceClause
    | abductiveBackgroundClause
    | abductiveHypothesisClause
    | abductiveCriterionClause
    | abductiveExplanationClause
    | abductiveResultClause
    | abductiveContextClause
    | abductiveDerivationClause
    | abductiveMetadataClause
    ;


/*
 * ============================================================================
 * OBSERVATION
 * ============================================================================
 *
 * An observation is an expression representing an observed state, event,
 * result, measurement, or other source of abductive information.
 *
 * Examples:
 *
 *     observation sensor_value;
 *     observation measurement;
 *     observation model_output;
 *
 * The semantic layer determines what "observation" means in a domain.
 */

abductiveObservationClause
    : ABDUCTION_OBSERVATION_LABEL
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Evidence is kept distinct from observation.
 *
 * This allows semantic analysis to distinguish:
 *
 *     observed data
 *     supporting evidence
 *     externally supplied evidence
 *     derived evidence
 *     verified evidence
 *
 * without forcing the grammar to know the implementation.
 */

abductiveEvidenceClause
    : ABDUCTION_EVIDENCE_LABEL
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * BACKGROUND
 * ============================================================================
 *
 * Background information represents contextual knowledge or assumptions used
 * when evaluating candidate explanations.
 *
 * It is deliberately represented by an expression.
 */

abductiveBackgroundClause
    : ABDUCTION_BACKGROUND_LABEL
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * HYPOTHESIS
 * ============================================================================
 *
 * A hypothesis is a candidate explanation.
 *
 * Multiple hypothesis clauses are permitted.
 *
 * There is no fixed number of hypotheses.
 */

abductiveHypothesisClause
    : ABDUCTION_HYPOTHESIS_LABEL
      abductiveHypothesis
      SEMICOLON?
    ;


abductiveHypothesis
    : expression
    ;


/*
 * ============================================================================
 * CRITERION
 * ============================================================================
 *
 * A criterion describes a semantic condition used to evaluate candidate
 * explanations.
 *
 * The criterion does not define a particular ranking or scoring algorithm.
 */

abductiveCriterionClause
    : ABDUCTION_CRITERION_LABEL
      abductiveCriterion
      SEMICOLON?
    ;


abductiveCriterion
    : expression
    ;


/*
 * ============================================================================
 * EXPLANATION
 * ============================================================================
 *
 * Explanation is an output/semantic boundary.
 *
 * The actual explanation representation is owned by the explanation and
 * provenance subsystems.
 */

abductiveExplanationClause
    : ABDUCTION_EXPLANATION_LABEL
      abductiveExplanation
      SEMICOLON?
    ;


abductiveExplanation
    : expression
    ;


/*
 * ============================================================================
 * RESULT
 * ============================================================================
 *
 * Result describes the value or object produced by the abductive operation.
 *
 * It is not tied to a particular implementation.
 */

abductiveResultClause
    : ABDUCTION_RESULT_LABEL
      abductiveResult
      SEMICOLON?
    ;


abductiveResult
    : expression
    ;


/*
 * ============================================================================
 * CONTEXT
 * ============================================================================
 *
 * Context provides additional semantic information without requiring the
 * grammar to enumerate every possible context category.
 */

abductiveContextClause
    : ABDUCTION_CONTEXT_LABEL
      abductiveContext
      SEMICOLON?
    ;


abductiveContext
    : expression
    ;


/*
 * ============================================================================
 * DERIVATION
 * ============================================================================
 *
 * A derivation records an intermediate semantic relationship.
 *
 * The grammar does not prescribe a proof calculus or solver.
 */

abductiveDerivationClause
    : ABDUCTION_DERIVATION_LABEL
      abductiveDerivation
      SEMICOLON?
    ;


abductiveDerivation
    : expression
    ;


/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata is an open extension point.
 *
 * It permits domain/tooling metadata without turning every metadata property
 * into a reserved keyword.
 *
 * Example:
 *
 *     @source(...)
 *     @domain(...)
 *     @trace(...)
 *
 * Actual annotation semantics remain owned elsewhere.
 */

abductiveMetadataClause
    : AT
      identifier
      abductiveMetadataPayload?
      SEMICOLON?
    ;


abductiveMetadataPayload
    : LEFT_PAREN
      abductionArgumentList?
      RIGHT_PAREN
    | COLON
      typeExpression
    | ASSIGN
      expression
    ;


/*
 * ============================================================================
 * LABEL REPRESENTATION
 * ============================================================================
 *
 * These labels are intentionally parser-level symbolic boundaries.
 *
 * IMPORTANT:
 *
 * The preferred production architecture is to promote only genuinely stable
 * language-level labels to lexer tokens.
 *
 * The rules below therefore use dedicated parser-facing symbolic tokens.
 *
 * If the repository already has canonical tokens with different names,
 * these aliases must be mapped to those canonical tokens rather than creating
 * duplicate lexical vocabulary.
 *
 * ============================================================================
 */

/*
 * The following tokens are expected to be supplied by ZamaniLexer:
 *
 *     ABDUCE
 *
 * The abductive body labels are preferably ordinary reserved vocabulary only
 * when the language specification has explicitly stabilized them.
 *
 * They are represented below as parser tokens so that the grammar has a clear
 * structural contract.
 *
 * If these labels are not yet canonical lexer tokens in the current branch,
 * use the identifier-field form during migration and promote the labels only
 * through the lexer authority.
 */

abductiveObservationLabel
    : identifier
    ;


abductiveEvidenceLabel
    : identifier
    ;


abductiveBackgroundLabel
    : identifier
    ;


abductiveHypothesisLabel
    : identifier
    ;


abductiveCriterionLabel
    : identifier
    ;


abductiveExplanationLabel
    : identifier
    ;


abductiveResultLabel
    : identifier
    ;


abductiveContextLabel
    : identifier
    ;


abductiveDerivationLabel
    : identifier
    ;


/*
 * ============================================================================
 * STABLE LABEL BOUNDARIES
 * ============================================================================
 *
 * The following aliases make the semantic contract explicit while keeping
 * lexical ownership outside this file.
 *
 * They intentionally resolve to identifiers.
 *
 * Semantic analysis validates the canonical field names:
 *
 *     observation
 *     evidence
 *     background
 *     hypothesis
 *     criterion
 *     explanation
 *     result
 *     context
 *     derivation
 *
 * A future dialect may extend this vocabulary without requiring this grammar
 * to enumerate every domain-specific field.
 *
 * ============================================================================
 */

ABDUCTION_OBSERVATION_LABEL
    : identifier
    ;

ABDUCTION_EVIDENCE_LABEL
    : identifier
    ;

ABDUCTION_BACKGROUND_LABEL
    : identifier
    ;

ABDUCTION_HYPOTHESIS_LABEL
    : identifier
    ;

ABDUCTION_CRITERION_LABEL
    : identifier
    ;

ABDUCTION_EXPLANATION_LABEL
    : identifier
    ;

ABDUCTION_RESULT_LABEL
    : identifier
    ;

ABDUCTION_CONTEXT_LABEL
    : identifier
    ;

ABDUCTION_DERIVATION_LABEL
    : identifier
    ;