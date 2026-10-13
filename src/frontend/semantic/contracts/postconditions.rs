
/*
 * Zamani Universal Computing Language
 *
 * FILE:
 *   grammar/validation/postconditions.g4
 *
 * GRAMMAR:
 *   Postconditions
 *
 * PURPOSE:
 *   Provide a complete-input validation facade for one standalone
 *   postcondition, delegating all source syntax to the canonical
 *   ContractStatements grammar.
 *
 * AUTHORITY:
 *   grammar/statements/contract.g4
 *
 * STATUS:
 *   Proposed corrected implementation; ANTLR generation and repository
 *   integration must pass before this file is marked production-ready.
 *
 * OWNS:
 *   postconditionValidationUnit
 *   postconditionValidationItem
 *
 * DOES NOT OWN:
 *   Source-language contract syntax, lexer tokens, expressions, AST nodes,
 *   semantic verification, type checking, effects, capabilities, resources,
 *   policies, provenance, IR, target selection or runtime evaluation.
 *
 * INTEGRATION:
 *   Canonical source rule:
 *     ContractStatements.ensuresStatement
 *
 *   Canonical condition rule:
 *     ContractStatements.contractCondition
 *
 *   Canonical expression rule:
 *     Expressions.expression
 *
 *   Function-attached postconditions are owned by:
 *     grammar/functions/contracts.g4
 *     FunctionContracts.functionEnsuresContract
 *
 * SCALABILITY:
 *   This grammar introduces no machine-capacity limits, fixed domain lists,
 *   resource allocation or target-specific assumptions.
 *
 * RUST BASELINE:
 *   Rust 1.97 or later, Edition 2021, safe Rust only.
 *
 * Note:
 *   Rust safety requirements apply to the implementation. This ANTLR file
 *   contains no embedded Rust code.
 */

parser grammar Postconditions;

options {
    tokenVocab = ZamaniLexer;
}

import ContractStatements;

/*
 * COMPLETE VALIDATION UNIT
 *
 * Accept exactly one complete standalone postcondition and require EOF.
 *
 * Accepted according to the canonical grammar:
 *   ensures(result >= 0);
 *
 * Rejected as a complete unit:
 *   ensures(result >= 0); trailing
 *
 * The condition's syntax is not redefined here.
 */
postconditionValidationUnit
    : postconditionValidationItem EOF
    ;

/*
 * VALIDATION ITEM
 *
 * This wrapper supplies a stable parser context for validation tooling.
 * The canonical grammar remains responsible for the actual syntax.
 */
postconditionValidationItem
    : ensuresStatement
    ;
