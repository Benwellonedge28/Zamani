/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/lexer-errors.g4
 *
 * Grammar:
 *     ZamaniLexerErrors
 *
 * Role:
 *     Reusable lexical-error recognition primitives for the canonical Zamani
 *     lexer.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This file contains ANTLR grammar only.
 *     It contains no Rust code and requires no `unsafe`.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file defines lexical-error SENTINELS for malformed lexical constructs
 * whose beginning is unambiguously recognizable but whose required terminator
 * is missing.
 *
 * The principal purpose is deterministic diagnostics.
 *
 * Examples include:
 *
 *     unterminated block comment
 *     unterminated documentation block comment
 *     unterminated string
 *     unterminated character literal
 *
 * IMPORTANT:
 *
 * This file does not replace the canonical lexer.
 *
 * It does not define the normal token vocabulary.
 *
 * It does not define valid comments, strings, characters, identifiers,
 * operators, literals, keywords, or punctuation.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     - malformed lexical sentinel recognition;
 *     - token kinds used to identify specific lexical failures;
 *     - lexical-error source boundaries;
 *     - deterministic classification of malformed constructs where practical.
 *
 * DOES NOT OWN:
 *
 *     - valid token syntax;
 *     - parser syntax;
 *     - AST construction;
 *     - semantic diagnostics;
 *     - type checking;
 *     - identifier resolution;
 *     - Unicode normalization;
 *     - Unicode security policy;
 *     - quantum semantics;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - hardware discovery;
 *     - target selection;
 *     - runtime errors;
 *     - resource exhaustion policy;
 *     - compiler recovery policy.
 *
 * ============================================================================
 *
 * ERROR PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       +--> valid token
 *       |
 *       +--> lexical-error sentinel
 *                    |
 *                    v
 *             lexical diagnostics
 *                    |
 *                    v
 *                 frontend
 *
 * The parser MUST NOT interpret an error sentinel as valid language syntax.
 *
 * ============================================================================
 *
 * IMPORTANT ANTLR BOUNDARY
 * ============================================================================
 *
 * This file is a lexer grammar containing fragments and/or error-token rules
 * intended for integration into the canonical lexer.
 *
 * A production build MUST NOT independently instantiate this grammar as an
 * alternative lexer and then select between two token streams.
 *
 * There must be exactly one canonical lexer authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file supplies lexical-error definitions to that authority.
 *
 * ============================================================================
 *
 * ERROR REPRESENTATION
 * ============================================================================
 *
 * The preferred design is:
 *
 *     malformed source
 *          |
 *          v
 *     dedicated lexical-error token
 *          |
 *          v
 *     diagnostic layer
 *
 * rather than:
 *
 *     malformed source
 *          |
 *          v
 *     silently skipped input
 *
 * or:
 *
 *     malformed source
 *          |
 *          v
 *     valid-looking token
 *
 * Silent recovery is forbidden because it can change the program presented
 * to later compiler stages.
 *
 * ============================================================================
 *
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * Error tokens MUST preserve their complete source lexeme.
 *
 * The lexer must not:
 *
 *     - rewrite the malformed input;
 *     - normalize Unicode;
 *     - trim the source;
 *     - discard the source span;
 *     - manufacture a replacement token;
 *     - silently continue as though the malformed construct did not exist.
 *
 * Diagnostic infrastructure can then report the exact source region.
 *
 * ============================================================================
 *
 * BLOCK COMMENTS
 * ============================================================================
 *
 * Zamani block comments are non-nesting.
 *
 * Valid syntax is owned by comments.g4:
 *
 *     /*
 *     ...
 *     */
 *
 * An opening:
 *
 *     /*
 *
 * without a corresponding:
 *
 *     */
 *
 * before end of input is a lexical error.
 *
 * This file therefore exposes a dedicated sentinel for that condition.
 *
 * ============================================================================
 *
 * DOCUMENTATION BLOCK COMMENTS
 * ============================================================================
 *
 * Documentation block comments use:
 *
 *     /**
 *     ...
 *     */
 *
 * and are likewise non-nesting.
 *
 * An unterminated documentation block comment is an error.
 *
 * ============================================================================
 *
 * LINE COMMENTS
 * ============================================================================
 *
 * Ordinary and documentation line comments do NOT require a dedicated
 * unterminated-error sentinel.
 *
 * A line comment is terminated by:
 *
 *     CR
 *     LF
 *     EOF
 *
 * Therefore EOF after a line comment is valid.
 *
 * ============================================================================
 *
 * STRINGS
 * ============================================================================
 *
 * A string beginning with:
 *
 *     "
 *
 * must have its required closing delimiter before a prohibited line boundary
 * or end of input, according to string-literals.g4.
 *
 * The normal STRING token remains owned by string-literals.g4.
 *
 * This file only identifies the malformed case for diagnostics.
 *
 * ============================================================================
 *
 * CHARACTER LITERALS
 * ============================================================================
 *
 * A character literal beginning with:
 *
 *     '
 *
 * must satisfy the character-literal grammar.
 *
 * This file does not redefine character syntax.
 *
 * It only supplies a malformed-input classification where the canonical lexer
 * needs deterministic error reporting.
 *
 * ============================================================================
 *
 * IMPORTANT: NO UNIVERSAL "INVALID CHARACTER" RULE
 * ============================================================================
 *
 * Do not add a broad rule such as:
 *
 *     INVALID : . ;
 *
 * here.
 *
 * Such a rule is dangerous because it can:
 *
 *     - mask lexer bugs;
 *     - swallow valid future syntax;
 *     - interfere with longest-match behavior;
 *     - make lexical diagnostics less precise;
 *     - turn future language extensions into misleading errors.
 *
 * Unknown characters should be handled by the canonical lexer error listener
 * or by explicitly specified invalid-character sentinels.
 *
 * ============================================================================
 *
 * IMPORTANT: NO UNIVERSAL EOF RULE
 * ============================================================================
 *
 * Do not add a generic:
 *
 *     ERROR_EOF : EOF ;
 *
 * rule.
 *
 * EOF is a valid lexer boundary and must not itself be an error.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * There are no artificial source-size limits in this grammar.
 *
 * Do not introduce:
 *
 *     MAX_SOURCE_LENGTH
 *     MAX_COMMENT_LENGTH
 *     MAX_STRING_LENGTH
 *     MAX_CHARACTER_LENGTH
 *     MAX_ERROR_LENGTH
 *     MAX_LINE_LENGTH
 *
 * or equivalent constants.
 *
 * The language does not impose a machine-dependent source-size ceiling.
 *
 * Practical limits belong to the source representation, compiler resources,
 * operating environment, or explicitly specified resource policy.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Lexical-error classification is independent of the eventual execution
 * target.
 *
 * The same malformed source must produce the same lexical classification
 * whether the intended target is:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     cluster
 *     supercomputer
 *     cloud
 *     distributed system
 *     future architecture
 *
 * Lexical validity is therefore established before target realization.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Given:
 *
 *     identical source
 *     identical language version
 *     identical lexical configuration
 *
 * the lexical-error classification must be deterministic.
 *
 * It must not depend on:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     hardware topology
 *     network state
 *     filesystem state
 *     clock time
 *     random state
 *     hash iteration order
 *
 * ============================================================================
 *
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Each error token provides the diagnostic layer with:
 *
 *     token type
 *     source text
 *     start position
 *     end position
 *     line
 *     column
 *
 * The exact public diagnostic code/message belongs to the diagnostics/error
 * subsystem rather than this grammar file.
 *
 * The grammar therefore must not hard-code user-facing localization strings.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * These token names are part of the internal lexer contract.
 *
 * They may be renamed only through the grammar version/compatibility process.
 *
 * User-facing diagnostic wording must NOT be treated as the stable identifier.
 *
 * The stable identity is the lexical-error category.
 *
 * ============================================================================
 *
 * TESTING CONTRACT
 * ============================================================================
 *
 * Required positive/error tests include:
 *
 *     /*
 *     /*
 *     hello
 *     /**
 *     /**
 *     documentation
 *     "
 *     "unterminated
 *     '
 *     'unterminated
 *
 * Required valid controls include:
 *
 *     /**/
 *     /** doc */
 *     // comment
 *     /// documentation
 *     "valid"
 *     'a'
 *
 * Required boundary tests include:
 *
 *     empty input
 *     one-character input
 *     EOF immediately after an opener
 *     EOF immediately before a closer
 *     CRLF
 *     LF
 *     CR
 *     Unicode inside malformed constructs
 *     very large malformed comments
 *     very large malformed strings
 *
 * The tests must verify that malformed constructs are rejected and never
 * silently converted into valid source.
 *
 * ============================================================================
 *
 * RUST INTEGRATION
 * ============================================================================
 *
 * ANTLR grammar generation must produce Rust compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The generated compiler/frontend implementation must remain safe Rust.
 *
 * No `unsafe` block or `unsafe` dependency is required by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * UNTERMINATED DOCUMENTATION BLOCK COMMENT
 * ============================================================================
 *
 * This rule is intentionally placed before the ordinary unterminated block
 * comment rule.
 *
 * A documentation block comment starts with:
 *
 *     /**
 *
 * whereas an ordinary block comment starts with:
 *
 *     /*
 *
 * The source is consumed through EOF when no closing `*/` exists.
 *
 * This token is an ERROR SENTINEL.
 *
 * It must never be placed on the HIDDEN channel.
 *
 * A diagnostic layer must observe it and report an error.
 */
UNTERMINATED_DOC_BLOCK_COMMENT
    : '/**' ( '*' ~[/] | ~'*' )* EOF
    ;


/*
 * ============================================================================
 * UNTERMINATED ORDINARY BLOCK COMMENT
 * ============================================================================
 *
 * Valid block comments are owned by comments.g4.
 *
 * This rule handles only the malformed case where the source reaches EOF
 * without a closing `*/`.
 *
 * It deliberately does not attempt nested-comment semantics.
 */
UNTERMINATED_BLOCK_COMMENT
    : '/*' ( '*' ~[/] | ~'*' )* EOF
    ;


/*
 * ============================================================================
 * UNTERMINATED STRING
 * ============================================================================
 *
 * This sentinel exists only if the canonical string lexer incorporates it.
 *
 * A newline or EOF before the closing quote makes the construct malformed
 * under the current Zamani string-literal policy.
 *
 * The valid STRING rule remains owned by string-literals.g4.
 *
 * Escape handling mirrors the canonical lexical escape boundary only far
 * enough to avoid mistaking an escaped quote for the closing delimiter.
 */
UNTERMINATED_STRING
    : '"' ( '\\' . | ~["\\\r\n] )* ( '\r' | '\n' | EOF )
    ;


/*
 * ============================================================================
 * UNTERMINATED CHARACTER LITERAL
 * ============================================================================
 *
 * This sentinel exists only if incorporated into the canonical character
 * lexer.
 *
 * Valid character-literal semantics remain owned by character-literals.g4.
 *
 * This rule merely recognizes a quote-delimited malformed construct ending
 * at a forbidden line boundary or EOF.
 */
UNTERMINATED_CHARACTER
    : '\'' ( '\\' . | ~['\\\r\n] )* ( '\r' | '\n' | EOF )
    ;