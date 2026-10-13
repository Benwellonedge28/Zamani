
/*
 * ============================================================================
 * Zamani — Preconditions Validation Grammar
 * ============================================================================
 *
 * FILE
 *   grammar/validation/preconditions.g4
 *
 * GRAMMAR
 *   PreconditionsValidation
 *
 * STATUS
 *   Production-oriented validation facade
 *
 * PURPOSE
 *   Provide an isolated ANTLR entry point for validating canonical
 *   standalone and function-contract preconditions.
 *
 * AUTHORITY
 *   grammar/DESIGN.md
 *   grammar/specification/semantic-model.md
 *   grammar/specification/grammar-authority.md
 *   grammar/specification/contracts.md
 *   grammar/validation/semantic-boundaries.md
 *   grammar/statements/contract.g4
 *   grammar/functions/contracts.g4
 *   grammar/expressions/
 *   grammar/antlr/ZamaniLexer.g4
 *
 * OWNS
 *   preconditionsValidationUnit
 *   preconditionsValidationItem
 *   standalonePreconditionValidation
 *   functionPreconditionValidation
 *
 * DOES_NOT_OWN
 *   requiresStatement
 *   functionRequiresContract
 *   contractCondition
 *   expression
 *   functionContractTerminator
 *   statementTerminator
 *   lexer tokens or keyword definitions
 *   AST or semantic-model types
 *   type, effect, capability, resource, or policy semantics
 *   IR generation or target realization
 *
 * DEPENDENCIES
 *   ContractStatements
 *     grammar/statements/contract.g4
 *
 *   FunctionContracts
 *     grammar/functions/contracts.g4
 *
 *   ZamaniLexer
 *     grammar/antlr/ZamaniLexer.g4
 *
 * RUST BASELINE
 *   Rust 1.97 or later; Edition 2021; safe Rust only.
 *
 * NOTE
 *   Rust requirements apply to the implementation and validation tooling.
 *   This file is ANTLR grammar, not Rust source code.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * A precondition is a condition that must hold at the boundary defined by
 * its enclosing semantic construct.
 *
 * The canonical source spelling is `requires`, not a newly invented
 * precondition keyword or an alternative syntax.
 *
 * Standalone source syntax is owned by ContractStatements.
 * Function-attached contract syntax is owned by FunctionContracts.
 *
 * This grammar only exposes those constructs for isolated validation.
 *
 * The expression is parsed, not evaluated, by this grammar.
 *
 * The native frontend AST preserves the source expression, source span,
 * enclosing declaration or scope, source order, and provenance.
 *
 * Semantic analysis determines:
 *   - whether the expression has a valid type;
 *   - whether its names resolve;
 *   - whether its effects are permitted;
 *   - whether capability and resource obligations are well-formed;
 *   - whether contract obligations can be established or discharged.
 *
 * Resource feasibility and target availability are downstream concerns.
 *
 * ============================================================================
 * SCALABILITY AND SAFETY
 * ============================================================================
 *
 * No machine-capacity or computational-domain ceilings are encoded here.
 *
 * No fixed maximum is imposed on the number of preconditions by the grammar.
 * Actual processing is bounded by available resources and explicit
 * operational budgets in the parser and validation tooling.
 *
 * The grammar contains no embedded actions, semantic predicates, runtime
 * evaluation, hardware discovery, resource allocation, or backend selection.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar PreconditionsValidation;

options {
    tokenVocab = ZamaniLexer;
}

import
    ContractStatements,
    FunctionContracts
;

/*
 * A complete validation input must contain at least one precondition and
 * consume all input.
 *
 * Use the production parser for complete programs. This entry point is
 * deliberately restricted to precondition validation cases.
 *
 * The repetition has no grammar-defined finite maximum.
 */
preconditionsValidationUnit
    : preconditionsValidationItem+ EOF
    ;

/*
 * Delegate to one of the canonical syntax owners.
 *
 * This rule does not define or duplicate source-language syntax.
 */
preconditionsValidationItem
    : standalonePreconditionValidation
    | functionPreconditionValidation
    ;

/*
 * Standalone precondition.
 *
 * Canonical owner:
 *   grammar/statements/contract.g4
 *     -> ContractStatements
 *       -> requiresStatement
 */
standalonePreconditionValidation
    : requiresStatement
    ;

/*
 * Function-attached precondition.
 *
 * Canonical owner:
 *   grammar/functions/contracts.g4
 *     -> FunctionContracts
 *       -> functionRequiresContract
 *
 * This entry point validates the precondition item itself. The production
 * function parser remains responsible for its enclosing declaration and
 * contract-block placement.
 */
functionPreconditionValidation
    : functionRequiresContract
    ;
