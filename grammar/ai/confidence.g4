/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/confidence.g4
 *
 * Grammar:
 *     AIConfidence
 *
 * Status:
 *     CANONICAL AI-DOMAIN CONFIDENCE COMPOSITION GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97 or later
 *     Rust 2021 edition
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the AI-domain composition boundary for confidence.
 *
 * IMPORTANT:
 *
 *     This file DOES NOT define a second confidence language.
 *
 * Confidence is a semantic property of uncertainty-bearing values and is
 * therefore already represented by the canonical uncertainty expression:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * The canonical source-level forms include, for example:
 *
 *     uncertain(value, confidence: c)
 *
 *     uncertainty(value, confidence: c)
 *
 * This file only provides an explicit AI-domain grammar boundary through
 * which the AI composition grammar can consume that canonical construct.
 *
 * The architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     Expressions                    AIConfidence
 *          |                             |
 *          |                             v
 *          |                    UncertaintyExpressions
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 domain-neutral AST
 *                        |
 *                        v
 *                 structural validation
 *                        |
 *          +-------------+----------------------------+
 *          |             |             |              |
 *          v             v             v              v
 *        types        semantics      effects       provenance
 *          |             |             |              |
 *          +-------------+-------------+--------------+
 *                        |
 *                        v
 *                canonical semantic model
 *                        |
 *             +----------+-----------+
 *             |                      |
 *             v                      v
 *         classical              quantum::ir
 *             |                      |
 *             +----------+-----------+
 *                        |
 *                        v
 *                 optimization
 *                        |
 *                     lowering
 *                        |
 *                 target realization
 *
 * This grammar creates no IR and performs no runtime computation.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Confidence is NOT inherently an AI-only concept.
 *
 * It may describe:
 *
 *     - statistical results;
 *     - scientific observations;
 *     - sensor observations;
 *     - model predictions;
 *     - reasoning results;
 *     - knowledge queries;
 *     - distributed observations;
 *     - simulation results;
 *     - hardware measurements;
 *     - verification results;
 *     - quantum measurement interpretations;
 *     - reliability estimates;
 *     - probabilistic computation;
 *     - future computational domains.
 *
 * Consequently:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * remains the universal source-level owner.
 *
 * This file exists only because the AI domain needs an explicit composition
 * boundary for consuming confidence-bearing uncertainty values.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Exactly one grammar owns the source-level uncertainty construct:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Grammar:
 *
 *     UncertaintyExpressions
 *
 * Public rule:
 *
 *     uncertaintyExpression
 *
 * This file MUST NOT redefine:
 *
 *     uncertaintyExpression
 *     uncertainValueExpression
 *     uncertaintyValueExpression
 *     uncertaintyValue
 *     uncertaintyArgumentList
 *     uncertaintyArgument
 *     uncertaintyNamedArgument
 *     uncertaintyPositionalArgument
 *     uncertaintyFieldName
 *
 * Those rules remain exclusively owned by:
 *
 *     UncertaintyExpressions
 *
 * ============================================================================
 * CONFIDENCE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - AI-domain composition of confidence-bearing uncertainty;
 *     - the stable AI confidence parser boundary;
 *     - the dependency from AI confidence semantics to universal uncertainty;
 *     - explicit AI composition of an already canonical construct;
 *     - prevention of duplicate confidence syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - confidence mathematics;
 *     - confidence intervals;
 *     - probability;
 *     - distributions;
 *     - belief;
 *     - likelihood;
 *     - evidence;
 *     - provenance;
 *     - statistical algorithms;
 *     - inference algorithms;
 *     - learning algorithms;
 *     - model semantics;
 *     - reasoning semantics;
 *     - decision semantics;
 *     - uncertainty type semantics;
 *     - expression precedence;
 *     - identifiers;
 *     - literals;
 *     - function calls;
 *     - operators;
 *     - punctuation;
 *     - lexer rules;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - contracts;
 *     - policies;
 *     - security;
 *     - runtime execution;
 *     - scheduling;
 *     - target selection;
 *     - hardware discovery;
 *     - classical IR;
 *     - AI-specific IR;
 *     - quantum IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Grammar:
 *
 *     UncertaintyExpressions
 *
 * Public rule consumed:
 *
 *     uncertaintyExpression
 *
 * The dependency direction is:
 *
 *     Expressions
 *          |
 *          +----> UncertaintyExpressions
 *                         ^
 *                         |
 *                  AIConfidence
 *
 * This file therefore does not import:
 *
 *     Expressions
 *
 * merely to obtain confidence syntax.
 *
 * Doing so would unnecessarily couple this AI leaf grammar to the complete
 * expression composition root and can create undesirable grammar dependency
 * relationships.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PUBLIC RULE:
 *
 *     aiConfidenceConstruct
 *
 * The rule represents an AI-domain use of the canonical uncertainty
 * expression where confidence is interpreted semantically by the AI
 * subsystem.
 *
 * No additional public confidence rules are required.
 *
 * Supporting syntax remains owned by UncertaintyExpressions.
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/ai/ai.g4
 *
 * Required integration:
 *
 *     import
 *         ...
 *         AIConfidence
 *     ;
 *
 * and:
 *
 *     aiConstruct
 *         : ...
 *         | aiConfidenceConstruct
 *         | ...
 *         ;
 *
 * The AI composition root remains the only AI-domain dispatcher.
 *
 * ============================================================================
 * ROOT-PARSER INTEGRATION
 * ============================================================================
 *
 * This file MUST NOT be imported directly by:
 *
 *     grammar/Zamani.g4
 *
 * or:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * when AI constructs are already routed through:
 *
 *     AI
 *
 * The intended path is:
 *
 *     ZamaniParser
 *          |
 *          v
 *         AI
 *          |
 *          v
 *     AIConfidence
 *          |
 *          v
 *     UncertaintyExpressions
 *
 * This keeps the root parser independent of individual AI leaf grammars.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * It introduces:
 *
 *     no tokens;
 *     no keywords;
 *     no punctuation;
 *     no operators;
 *     no lexer modes;
 *     no lexical aliases.
 *
 * The canonical lexical vocabulary remains owned by:
 *
 *     grammar/lexer/
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The existing canonical uncertainty vocabulary includes:
 *
 *     UNCERTAIN
 *     UNCERTAINTY
 *     CONFIDENCE
 *     PROBABILITY
 *     PROBABILISTIC
 *     DISTRIBUTION
 *     BELIEF
 *     LIKELIHOOD
 *     EVIDENCE
 *     PROVENANCE
 *     SOURCE
 *
 * This file does not consume those tokens directly because the canonical
 * uncertainty grammar already owns their syntactic interpretation.
 *
 * ============================================================================
 * NO KEYWORD EXPLOSION
 * ============================================================================
 *
 * This file deliberately does not introduce:
 *
 *     confidence_value
 *     confidence_score
 *     confidence_level
 *     confidence_interval
 *     model_confidence
 *     prediction_confidence
 *     neural_confidence
 *     quantum_confidence
 *     hardware_confidence
 *
 * as reserved language constructs.
 *
 * Such concepts remain:
 *
 *     expressions;
 *     types;
 *     library values;
 *     semantic metadata;
 *     dialect constructs;
 *     provider-defined values;
 *
 * unless a future language-wide specification explicitly establishes a
 * separate canonical construct.
 *
 * ============================================================================
 * WHY THIS FILE IS A COMPOSITION FACADE
 * ============================================================================
 *
 * The universal uncertainty grammar already supports:
 *
 *     uncertain(value, confidence: c)
 *
 *     uncertainty(value, confidence: c)
 *
 * and arbitrary expression values.
 *
 * Therefore creating:
 *
 *     confidence(value)
 *
 * here would create a second source-level representation of the same semantic
 * concept.
 *
 * That would introduce:
 *
 *     duplicate syntax;
 *     duplicate AST interpretation;
 *     duplicate diagnostics;
 *     duplicate semantic handling;
 *     compatibility ambiguity;
 *     unnecessary keyword coupling.
 *
 * The production architecture instead uses:
 *
 *     canonical uncertainty syntax
 *             |
 *             v
 *     AI confidence composition
 *             |
 *             v
 *     shared semantic model
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * `aiConfidenceConstruct` is intentionally a very small façade.
 *
 * It consumes the canonical:
 *
 *     uncertaintyExpression
 *
 * rather than reproducing its internal productions.
 *
 * ============================================================================
 */

parser grammar AIConfidence;

options {
    tokenVocab = ZamaniLexer;
}

import
    UncertaintyExpressions
    ;

/*
 * ============================================================================
 * AI CONFIDENCE CONSTRUCT
 * ============================================================================
 *
 * This is an AI-domain composition boundary.
 *
 * The underlying syntax remains owned by:
 *
 *     UncertaintyExpressions
 *
 * Therefore all canonical uncertainty forms remain available without
 * duplication:
 *
 *     uncertain(value)
 *
 *     uncertainty(value)
 *
 *     uncertain(value, confidence: confidence_value)
 *
 *     uncertain(value, confidence = confidence_value)
 *
 *     uncertainty(
 *         value,
 *         confidence: confidence_value
 *     )
 *
 * The semantic layer determines whether the resulting uncertainty-bearing
 * value actually carries a confidence property and whether that property is
 * valid.
 *
 * The parser deliberately does not inspect semantic metadata here.
 */
aiConfidenceConstruct
    : uncertaintyExpression
    ;