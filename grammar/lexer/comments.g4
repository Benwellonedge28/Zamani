/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/comments.g4
 *
 * Grammar:
 *     ZamaniComments
 *
 * Role:
 *     Canonical lexical definition of Zamani comments.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * This grammar owns ONLY comment syntax:
 *
 *     // ordinary line comments
 *     /* ordinary block comments *\/
 *     /// documentation line comments
 *     /** documentation block comments *\/
 *
 * It also owns the lexical disposition of those comments:
 *
 *     ordinary comments       -> HIDDEN channel
 *     documentation comments  -> HIDDEN channel
 *
 * Comments therefore remain available to tooling while remaining invisible
 * to ordinary parser syntax.
 *
 * ============================================================================
 *
 * DOES NOT OWN
 * ============================================================================
 *
 * This grammar does NOT own:
 *
 *     - whitespace;
 *     - newlines as a language construct;
 *     - identifiers;
 *     - keywords;
 *     - literals;
 *     - operators;
 *     - punctuation;
 *     - annotations;
 *     - attributes;
 *     - AST construction;
 *     - documentation semantics;
 *     - semantic analysis;
 *     - type checking;
 *     - effects;
 *     - capabilities;
 *     - resource analysis;
 *     - quantum semantics;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - hardware discovery;
 *     - compilation targets;
 *     - runtime behavior.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Comments are source-level metadata, not executable semantics.
 *
 * Therefore:
 *
 *     source
 *       |
 *       v
 *     comment lexer
 *       |
 *       +--> ordinary comments -> hidden token stream
 *       |
 *       +--> documentation    -> hidden token stream
 *       |
 *       v
 *     parser
 *
 * The parser normally ignores these tokens.
 *
 * Tooling may inspect the hidden channel for:
 *
 *     - documentation generation;
 *     - formatting;
 *     - IDE features;
 *     - source-to-source transformation;
 *     - diagnostics;
 *     - code navigation;
 *     - refactoring;
 *     - macro tooling;
 *     - source preservation.
 *
 * ============================================================================
 *
 * COMMENT MODEL
 * ============================================================================
 *
 * Ordinary line comment:
 *
 *     //
 *
 * continues until CR, LF, or end of input.
 *
 * Documentation line comment:
 *
 *     ///
 *
 * follows the same termination rule.
 *
 * Ordinary block comment:
 *
 *     /*
 *     ...
 *     *\/
 *
 * Documentation block comment:
 *
 *     /**
 *     ...
 *     *\/
 *
 * The baseline language defines block comments as NON-NESTING.
 *
 * This is deliberate.
 *
 * The grammar must not silently interpret:
 *
 *     /* outer /* inner *\/ outer *\/
 *
 * as a nested comment.
 *
 * If nested comments are introduced in a future language version, that is a
 * language-versioning change requiring an explicit specification and
 * compatibility policy.
 *
 * ============================================================================
 *
 * DOCUMENTATION COMMENT MODEL
 * ============================================================================
 *
 * Documentation comments are lexically distinct from ordinary comments.
 *
 * They do NOT automatically create declarations or semantic nodes.
 *
 * Their association with:
 *
 *     module
 *     declaration
 *     function
 *     type
 *     field
 *     hardware object
 *     quantum object
 *     etc.
 *
 * belongs to frontend/source-model tooling.
 *
 * This prevents the lexer from acquiring semantic knowledge of the rest of
 * Zamani.
 *
 * ============================================================================
 *
 * CHANNEL POLICY
 * ============================================================================
 *
 * Comments use ANTLR's HIDDEN channel rather than `skip`.
 *
 * This distinction is important.
 *
 * `skip` would remove the token completely from the token stream.
 *
 * `channel(HIDDEN)` preserves:
 *
 *     token type
 *     source text
 *     source span
 *     token ordering
 *
 * for consumers that need source fidelity.
 *
 * The ordinary parser can continue to ignore hidden-channel comments.
 *
 * ============================================================================
 *
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The grammar deliberately does not transform comment contents.
 *
 * In particular, it does NOT:
 *
 *     - trim indentation;
 *     - normalize Unicode;
 *     - normalize line endings;
 *     - remove leading documentation markers;
 *     - interpret Markdown;
 *     - interpret HTML;
 *     - interpret Rustdoc-style directives;
 *     - execute directives;
 *     - expand macros;
 *     - resolve references.
 *
 * Such processing belongs to documentation/source tooling.
 *
 * ============================================================================
 *
 * ERROR MODEL
 * ============================================================================
 *
 * Unterminated block comments are invalid source.
 *
 * The lexer must therefore allow the generated ANTLR lexer to report a
 * deterministic lexical error when:
 *
 *     /*
 *
 * is not followed by a terminating:
 *
 *     *\/
 *
 * before end of input.
 *
 * There is intentionally no recovery rule that converts an unterminated
 * comment into a valid comment.
 *
 * This prevents silent source corruption.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No machine-dependent limits are encoded here.
 *
 * There is no:
 *
 *     MAX_COMMENT_LENGTH
 *     MAX_COMMENT_LINES
 *     MAX_SOURCE_SIZE
 *     MAX_PROGRAM_SIZE
 *
 * or equivalent language-level limit.
 *
 * A comment may therefore be as large as the available source representation,
 * lexer implementation, memory, and compilation environment permit.
 *
 * Such implementation/resource limits are not language semantics.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * For identical source input and identical language version/configuration:
 *
 *     comment text
 *     comment token kind
 *     source span
 *     token ordering
 *
 * must be deterministic.
 *
 * The result must not depend on:
 *
 *     - CPU architecture;
 *     - number of CPU cores;
 *     - available QPUs;
 *     - GPU availability;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - random state;
 *     - hash-map iteration order.
 *
 * ============================================================================
 *
 * LONGEST-MATCH REQUIREMENT
 * ============================================================================
 *
 * Documentation comment forms must be recognized before ordinary comment
 * forms because:
 *
 *     ///
 *
 * begins with:
 *
 *     //
 *
 * and:
 *
 *     /**
 *
 * begins with:
 *
 *     /*
 *
 * ANTLR's lexer matching behavior is therefore deliberately reinforced by
 * placing documentation rules before ordinary comment rules.
 *
 * ============================================================================
 *
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file is ANTLR grammar source.
 *
 * It contains no Rust code and therefore contains no unsafe Rust.
 *
 * Generated Rust lexer/parser integration MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and the Zamani compiler must maintain its safe-Rust policy.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Comments must never encode machine-specific execution semantics.
 *
 * A comment may describe:
 *
 *     intent
 *     documentation
 *     rationale
 *     API information
 *     source metadata
 *
 * but it must not become an implicit:
 *
 *     hardware requirement
 *     device selection
 *     qubit allocation
 *     topology requirement
 *     scheduler instruction
 *     runtime command
 *
 * unless a separate, explicitly specified language construct exists for that
 * purpose.
 *
 * ============================================================================
 */

lexer grammar ZamaniComments;


/*
 * ============================================================================
 * DOCUMENTATION LINE COMMENTS
 * ============================================================================
 *
 * `///` is reserved for documentation-oriented source comments.
 *
 * The complete source text is preserved as the token text.
 *
 * The parser normally ignores the token because it is on HIDDEN.
 */

DOC_LINE_COMMENT
    : '///' ~[\r\n]* -> channel(HIDDEN)
    ;


/*
 * ============================================================================
 * DOCUMENTATION BLOCK COMMENTS
 * ============================================================================
 *
 * `/** ... *\/` is the documentation-oriented block-comment form.
 *
 * This rule is intentionally non-nesting.
 *
 * The body consumes characters until the first terminating `*\/`.
 *
 * Because `.` is deliberately NOT used here, the rule explicitly handles
 * arbitrary source characters including line terminators.
 *
 * The terminating sequence is:
 *
 *     *\/
 *
 * and is consumed as part of the token.
 */

DOC_BLOCK_COMMENT
    : '/**' ( '*' ~[/] | ~'*' )* '*/' -> channel(HIDDEN)
    ;


/*
 * ============================================================================
 * ORDINARY LINE COMMENTS
 * ============================================================================
 *
 * `//` begins an ordinary source comment.
 *
 * The comment continues through all non-newline characters.
 *
 * CR and LF remain available to the lexer as source boundaries.
 *
 * The newline itself is therefore NOT consumed by this rule.
 */

LINE_COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;


/*
 * ============================================================================
 * ORDINARY BLOCK COMMENTS
 * ============================================================================
 *
 * `/* ... *\/` is an ordinary block comment.
 *
 * The implementation is explicitly NON-NESTING.
 *
 * The body:
 *
 *     ( '*' ~[/] | ~'*' )*
 *
 * consumes:
 *
 *     - a star only when it is not the beginning of `*\/`;
 *     - any non-star character.
 *
 * The lexer therefore stops at the first valid closing delimiter.
 *
 * An unterminated block comment cannot match this rule and must produce a
 * lexical error rather than silently consuming the remainder of the file.
 */

BLOCK_COMMENT
    : '/*' ( '*' ~[/] | ~'*' )* '*/' -> channel(HIDDEN)
    ;