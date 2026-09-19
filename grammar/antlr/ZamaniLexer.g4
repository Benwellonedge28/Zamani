/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * Canonical production lexer orchestrator.
 *
 * All lexical token ownership belongs to:
 *
 *     grammar/lexer/
 *
 * The composed lexical vocabulary is:
 *
 *     ZamaniTokens
 *
 * This file MUST NOT duplicate keyword, operator, punctuation, identifier,
 * literal, comment, annotation, or lexical-error rules.
 *
 * ============================================================================
 */

lexer grammar ZamaniLexer;

import ZamaniTokens;