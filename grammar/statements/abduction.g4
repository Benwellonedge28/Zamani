/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/abduction.g4
 *
 * Grammar:
 *     AbductionStatements
 *
 * Purpose:
 *     Universal source-level abductive reasoning statement.
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 */

parser grammar AbductionStatements;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions
    ;


/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * Canonical form:
 *
 *     abduce HYPOTHESIS from OBSERVATIONS;
 *
 * Optional semantic context:
 *
 *     abduce HYPOTHESIS from OBSERVATIONS with (CONTEXT);
 *
 * Examples:
 *
 *     abduce explanation from observations;
 *
 *     abduce fault_model from measurements;
 *
 *     abduce hypothesis from evidence with (confidence);
 *
 *     abduce explanation from quantum_result with (policy);
 *
 * The parser preserves structure.
 *
 * Semantic analysis determines whether the expressions are valid hypotheses,
 * observations, evidence, models, measurements, or other domain values.
 * ============================================================================
 */

abductionStatement
    : ABDUCE
      abductionHypothesis
      abductionSourceClause
      abductionContextClause?
      SEMICOLON
    ;


/*
 * ============================================================================
 * HYPOTHESIS
 * ============================================================================
 */

abductionHypothesis
    : expression
    ;


/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 *
 * The source is mandatory.
 *
 * Requiring an explicit source prevents malformed constructs such as:
 *
 *     abduce hypothesis;
 *
 * from entering the semantic pipeline without evidence/observation context.
 * ============================================================================
 */

abductionSourceClause
    : FROM
      abductionEvidenceSource
    ;


abductionEvidenceSource
    : expression
    ;


/*
 * ============================================================================
 * CONTEXT
 * ============================================================================
 *
 * Context is optional but, when present, must contain at least one expression.
 *
 * No trailing comma is accepted.
 * ============================================================================
 */

abductionContextClause
    : WITH
      LPAREN
      abductionContextList
      RPAREN
    ;


abductionContextList
    : abductionContextItem
      (
          COMMA
          abductionContextItem
      )*
    ;


abductionContextItem
    : expression
    ;