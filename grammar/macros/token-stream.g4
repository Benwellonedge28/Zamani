/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/token-stream.g4
 *
 * Grammar:
 *     tokenStream
 *
 * Status:
 *     CANONICAL MACRO TOKEN-TREE PARSER COMPONENT
 *
 * Purpose:
 *     Define a domain-neutral, recursively structured representation of
 *     source tokens for macro input, token capture, and syntax-preserving
 *     macro processing.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * Portability:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
 *
 * Scalability:
 *     No language-defined maximum token count, nesting depth, delimiter
 *     count, macro count, or generated token count.
 *
 * ============================================================================
 * 1. ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * A macro must be able to receive source syntax without requiring the macro
 * grammar to understand every Zamani language domain.
 *
 * This component therefore defines balanced token trees rather than a
 * second expression, statement, type, quantum, or HDL grammar.
 *
 * Conceptual representation:
 *
 *     token stream
 *          |
 *          +--> ordinary token
 *          |
 *          +--> balanced parenthesized group
 *          |
 *          +--> balanced bracketed group
 *          |
 *          +--> balanced braced group
 *          |
 *          v
 *     canonical token-tree structure
 *          |
 *          v
 *     macro AST / token-tree representation
 *          |
 *          v
 *     controlled macro processing
 *
 * This grammar recognizes structure only.
 *
 * It does not expand, execute, evaluate, resolve, or interpret macro input.
 *
 * ============================================================================
 * 2. SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * This file owns:
 *
 *     tokenStream
 *     tokenTree
 *     tokenTreeElement
 *     tokenTreeGroup
 *     tokenTreeLeaf
 *     parenthesizedTokenTree
 *     bracketedTokenTree
 *     bracedTokenTree
 *
 * This file does not own:
 *
 *     lexer rules
 *     token definitions
 *     keyword definitions
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     statements
 *     declarations
 *     macro declarations
 *     macro invocations
 *     macro parameters
 *     macro expansion
 *     macro hygiene
 *     source-span implementation
 *     AST implementation
 *     semantic analysis
 *     compiler resource policy
 *
 * No other grammar component may independently redefine the productions
 * owned here.
 *
 * Existing macro components remain authoritative for their respective
 * responsibilities:
 *
 *     macros.g4
 *         Macro composition.
 *
 *     declarations.g4
 *         Macro declaration syntax.
 *
 *     invocations.g4
 *         Macro invocation syntax.
 *
 *     expansion.g4
 *         Explicit expansion-related syntax.
 *
 *     hygiene.g4
 *         Explicit hygiene-related syntax.
 *
 *     expressions/macros.g4
 *         Expression-side macro integration.
 *
 * ============================================================================
 * 3. LEXER CONTRACT
 * ============================================================================
 *
 * The canonical Zamani lexer supplies every token consumed by this grammar.
 *
 * This component is a parser grammar, not a combined grammar.
 *
 * It uses the canonical ZamaniLexer vocabulary.
 *
 * It MUST NOT:
 *
 *     - declare lexer tokens;
 *     - duplicate keywords;
 *     - introduce a second identifier rule;
 *     - invent alternative delimiter tokens;
 *     - reinterpret ordinary operators;
 *     - introduce domain-specific tokens;
 *     - use literal strings as replacement tokens.
 *
 * Whitespace and comments are governed by the canonical lexer.
 *
 * If the lexer discards trivia, this grammar cannot reconstruct the original
 * whitespace or comment placement. Exact source preservation therefore
 * remains the responsibility of the lexer/source representation and its
 * token metadata.
 *
 * A token tree preserves token structure, not necessarily original source
 * formatting.
 *
 * ============================================================================
 * 4. DELIMITER CONTRACT
 * ============================================================================
 *
 * Balanced groups use the existing canonical delimiter tokens:
 *
 *     LPAREN  RPAREN
 *     LBRACK  RBRACK
 *     LBRACE  RBRACE
 *
 * Each opening delimiter must be closed by its corresponding delimiter.
 *
 * Crossed or mismatched groups are not accepted.
 *
 * Examples:
 *
 *     (a, b)
 *     [a, b]
 *     { a + b }
 *
 * Nested groups:
 *
 *     ({ [a] })
 *     ([{ a }])
 *     { call([value]) }
 *
 * The grammar does not impose a finite nesting limit.
 *
 * A parser implementation may enforce a configurable nesting/resource
 * budget for protection against hostile input. Such a budget is not a
 * language-level syntax restriction.
 *
 * ============================================================================
 * 5. TOKEN-TREE MODEL
 * ============================================================================
 *
 * A token tree is either:
 *
 *     - one non-delimiter token; or
 *     - one balanced delimiter group containing zero or more token trees.
 *
 * A token tree is not necessarily a valid Zamani expression.
 *
 * For example, the following can be structurally captured as token trees:
 *
 *     quantum_operation!(register)
 *     tensor<T, shape>
 *     module::operation(value)
 *     hardware { requires capability("compute") }
 *     always @(posedge clock) { signal = value; }
 *
 * Whether these constructs are valid Zamani syntax is determined by the
 * canonical parser and subsequent semantic analysis after the relevant
 * macro-processing stage.
 *
 * ============================================================================
 * 6. TOKEN PRESERVATION
 * ============================================================================
 *
 * A token leaf represents one token from the canonical lexer vocabulary.
 *
 * The grammar deliberately does not enumerate every possible token type.
 *
 * This allows future lexical additions to be captured without editing this
 * file, provided they do not alter the canonical delimiter contract.
 *
 * The parser's token stream remains the source of truth for:
 *
 *     token type
 *     token text
 *     token index
 *     source location
 *     source interval
 *
 * The AST/token-tree implementation must preserve the metadata required by
 * diagnostics, provenance, deterministic expansion, and source mapping.
 *
 * ============================================================================
 * 7. UNMATCHED DELIMITERS
 * ============================================================================
 *
 * Unmatched closing delimiters are not token-tree leaves.
 *
 * An unmatched opening delimiter cannot complete a token-tree group.
 *
 * This grammar therefore rejects malformed grouping rather than silently
 * treating a delimiter as an ordinary token.
 *
 * Examples that must be rejected:
 *
 *     (a]
 *     {a)
 *     [a}
 *     )
 *     ]
 *     }
 *
 * Error recovery may report additional diagnostics, but must not silently
 * reinterpret malformed delimiter structure as valid source.
 *
 * ============================================================================
 * 8. EMPTY GROUPS AND EMPTY STREAMS
 * ============================================================================
 *
 * Empty balanced groups are valid:
 *
 *     ()
 *     []
 *     {}
 *
 * An empty token stream is valid when the caller explicitly requests a
 * token-stream parse.
 *
 * Whether an empty stream is a valid macro argument or macro body is decided
 * by the owning declaration/invocation grammar.
 *
 * This component does not impose that policy.
 *
 * ============================================================================
 * 9. MACRO INTEGRATION
 * ============================================================================
 *
 * Intended downstream uses include:
 *
 *     - token-oriented macro parameters;
 *     - syntax quotation;
 *     - token capture;
 *     - structured macro input;
 *     - syntax-preserving transformations;
 *     - generated syntax;
 *     - diagnostics and source provenance.
 *
 * This file does not require every macro to use token trees.
 *
 * Expression-oriented macros may continue to use the existing canonical
 * expression and argument rules.
 *
 * The macro declaration/invocation contracts determine where token trees
 * are accepted and how their AST representation is selected.
 *
 * No existing public invocation spelling is changed by this file.
 *
 * ============================================================================
 * 10. AST CONTRACT
 * ============================================================================
 *
 * The grammar produces parser contexts only.
 *
 * The frontend AST owns the stable source representation.
 *
 * A downstream token-tree AST representation must preserve:
 *
 *     - token leaves in source order;
 *     - nested delimiter groups;
 *     - opening and closing delimiter identity;
 *     - source spans;
 *     - token provenance;
 *     - deterministic traversal order.
 *
 * It must not introduce a second general-purpose expression AST.
 *
 * It must not store resolved macro definitions, expansion state, hardware
 * identity, or target-specific information.
 *
 * Existing macro invocation AST ownership remains with:
 *
 *     src/frontend/ast/node/expressions/macro.rs
 *
 * This file does not require changing that invocation node merely to add
 * token-tree parsing.
 *
 * If the existing AST cannot represent token trees, a separate AST contract
 * must be approved before wiring this parser component into production.
 *
 * ============================================================================
 * 11. SEMANTIC AND EXPANSION BOUNDARY
 * ============================================================================
 *
 * Parsing token trees does not:
 *
 *     - resolve macro names;
 *     - bind macro parameters;
 *     - evaluate defaults;
 *     - expand macros;
 *     - execute generated code;
 *     - validate generated syntax;
 *     - establish hygiene;
 *     - grant capabilities;
 *     - bypass semantic validation.
 *
 * The downstream pipeline remains:
 *
 *     token-tree parsing
 *          |
 *          v
 *     canonical frontend AST
 *          |
 *          v
 *     macro resolution
 *          |
 *          v
 *     controlled expansion
 *          |
 *          v
 *     hygiene / provenance
 *          |
 *          v
 *     ordinary syntax validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *
 * Generated syntax must undergo the same applicable validation as directly
 * authored syntax.
 *
 * ============================================================================
 * 12. DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Token trees may contain syntax from any Zamani domain:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     metaprogramming
 *     future domains
 *
 * This grammar does not define domain-specific token-tree variants.
 *
 * It does not enumerate quantum gates, physical devices, instruction sets,
 * tensor operations, HDL signal types, or vendor operations.
 *
 * Quantum constructs eventually pass through the established canonical
 * quantum::ir boundary after semantic processing.
 *
 * No additional quantum IR is introduced.
 *
 * ============================================================================
 * 13. POCO-REAF AND RESOURCE SCALABILITY
 * ============================================================================
 *
 * This grammar introduces no language-defined finite maximum for:
 *
 *     token count
 *     token-tree element count
 *     group count
 *     nesting depth
 *     macro argument count
 *     macro invocation count
 *     source size
 *     generated syntax size
 *
 * There are no MAX_* constants in this grammar.
 *
 * In particular, it does not impose limits on:
 *
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     nodes
 *     memory
 *     threads
 *     tensor rank
 *     register width
 *     network size
 *     device count
 *
 * Resource protection belongs to configurable compiler policy.
 *
 * Such policy must distinguish:
 *
 *     language validity
 *     implementation capacity
 *     configured compilation budget
 *     available system resources
 *
 * Resource exhaustion must produce an explicit diagnostic rather than
 * silent truncation or reinterpretation.
 *
 * ============================================================================
 * 14. DETERMINISM
 * ============================================================================
 *
 * Given the same canonical token sequence and parser configuration, token
 * tree parsing must produce structurally equivalent parse trees.
 *
 * Child order is source order.
 *
 * This grammar introduces no unordered collection or semantic lookup.
 *
 * Macro expansion determinism remains the responsibility of the compiler
 * macro-processing subsystem.
 *
 * ============================================================================
 * 15. SECURITY
 * ============================================================================
 *
 * This grammar contains no executable actions.
 *
 * Parsing must not:
 *
 *     - access the filesystem;
 *     - access the network;
 *     - spawn processes;
 *     - inspect environment variables;
 *     - access secrets;
 *     - inspect hardware;
 *     - execute macro code;
 *     - mutate compiler-global state.
 *
 * Token capture is not permission to execute captured content.
 *
 * The Rust implementation must use safe Rust only, including under Rust
 * 1.97 and Rust 1.97.1.
 *
 * ============================================================================
 * 16. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/lexer/*
 *         Canonical lexical vocabulary and delimiter tokens.
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         Canonical lexer vocabulary, where used by the build.
 *
 *     grammar/spec/syntax.md
 *         Normative syntax and delimiter rules.
 *
 *     grammar/spec/semantics.md
 *         Macro and semantic boundaries.
 *
 *     grammar/macros/macros.g4
 *         Macro grammar composition.
 *
 * Downstream:
 *
 *     grammar/macros/declarations.g4
 *         May reference tokenTree where a declaration contract permits it.
 *
 *     grammar/macros/invocations.g4
 *         May reference tokenTree where an invocation contract permits it.
 *
 *     grammar/macros/expansion.g4
 *         May reference tokenTree for explicit source-level expansion syntax.
 *
 *     grammar/metaprogramming/*
 *         May consume token-tree syntax through an approved composition
 *         contract.
 *
 *     src/frontend/ast/
 *         Owns persistent token-tree representation and source spans.
 *
 *     src/compiler/macro_engine.rs
 *         Owns macro registration/expansion policy, not grammar parsing.
 *
 *     semantic analysis
 *         Validates expanded source meaning.
 *
 * Integration rule:
 *
 *     A consuming grammar must import or compose this grammar through the
 *     repository's supported ANTLR composition mechanism.
 *
 *     It must not copy these productions into another file.
 *
 *     It must not create a competing lexer vocabulary.
 *
 *     It must not introduce circular grammar imports.
 *
 * The exact build composition must be validated against the repository's
 * canonical ANTLR generation configuration before this component is marked
 * integrated.
 *
 * ============================================================================
 * 17. GRAMMAR CONTRACT
 * ============================================================================
 *
 * tokenStream:
 *     Complete token-tree stream followed by EOF.
 *
 * tokenTree:
 *     One or more token-tree elements.
 *
 * tokenTreeElement:
 *     One balanced group or one non-delimiter token.
 *
 * tokenTreeGroup:
 *     One balanced parenthesized, bracketed, or braced group.
 *
 * tokenTreeLeaf:
 *     One token other than a grouping delimiter.
 *
 * The inline tokenTree rule deliberately does not consume EOF.
 *
 * This permits callers to embed tokenTree inside larger parser productions.
 *
 * ============================================================================
 * 18. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     empty stream
 *     one token
 *     multiple tokens
 *     empty groups
 *     nested parentheses
 *     nested brackets
 *     nested braces
 *     mixed balanced groups
 *     operators as leaves
 *     identifiers as leaves
 *     literals as leaves
 *     domain-specific tokens as leaves
 *     nested macro-like token sequences
 *
 * Negative:
 *
 *     unmatched opening delimiter
 *     unmatched closing delimiter
 *     crossed delimiters
 *     mismatched closing delimiter
 *     malformed nested groups
 *
 * Boundary:
 *
 *     empty group
 *     one leaf
 *     one group
 *     adjacent groups
 *     deeply nested groups within configured test resources
 *
 * Scalability:
 *
 *     increasing token count
 *     increasing sibling group count
 *     increasing nesting
 *     large mixed-domain token trees
 *     large source streams
 *
 * Determinism:
 *
 *     identical token input produces equivalent parse structure
 *     source ordering remains stable
 *
 * Compatibility:
 *
 *     existing macro invocation syntax remains unchanged
 *     ordinary expression parsing remains owned by expressions/
 *     canonical lexer token meanings remain unchanged
 *
 * ============================================================================
 * 19. DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It is a parser grammar using the canonical lexer vocabulary.
 *     [x] It owns only token-tree syntax.
 *     [x] It does not redefine lexical tokens.
 *     [x] It supports balanced parentheses, brackets, and braces.
 *     [x] It rejects mismatched delimiter structures.
 *     [x] It permits empty balanced groups.
 *     [x] It permits an empty complete token stream.
 *     [x] It has no fixed token or nesting limits.
 *     [x] It contains no executable grammar actions.
 *     [x] It introduces no domain-specific grammar.
 *     [x] It preserves the canonical macro ownership model.
 *     [x] It documents AST, semantic, and compiler integration.
 *     [x] It preserves the canonical quantum::ir boundary.
 *     [x] It requires no unsafe Rust.
 *
 * Repository integration remains complete only after:
 *
 *     [ ] Canonical lexer delimiter names are verified.
 *     [ ] ANTLR generation succeeds with the actual build configuration.
 *     [ ] Composition imports are wired without duplicate productions.
 *     [ ] Token-tree AST mapping is implemented or explicitly deferred.
 *     [ ] Source spans and provenance are preserved.
 *     [ ] Positive and negative parser tests pass.
 *     [ ] Scalability and determinism tests pass.
 *     [ ] Rust 1.97.1 safe-Rust CI passes.
 *
 * ============================================================================
 * END OF ARCHITECTURAL CONTRACT
 * ============================================================================
 */

parser grammar tokenStream;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Complete token-stream entry point.
 *
 * EOF belongs only to this complete-stream entry point. Inline consumers
 * should use tokenTree instead.
 */
tokenStream
    : tokenTreeElement* EOF
    ;

/*
 * Inline token-tree entry point.
 *
 * One or more elements are required. Empty content is represented by an
 * empty balanced group or by tokenStream's zero-element alternative.
 */
tokenTree
    : tokenTreeElement+
    ;

/*
 * A token-tree element is either a balanced group or one ordinary token.
 */
tokenTreeElement
    : tokenTreeGroup
    | tokenTreeLeaf
    ;

/*
 * Delimiter groups are balanced recursively.
 *
 * The alternatives are explicit so crossed delimiters cannot be accepted.
 */
tokenTreeGroup
    : parenthesizedTokenTree
    | bracketedTokenTree
    | bracedTokenTree
    ;

parenthesizedTokenTree
    : LPAREN tokenTreeElement* RPAREN
    ;

bracketedTokenTree
    : LBRACK tokenTreeElement* RBRACK
    ;

bracedTokenTree
    : LBRACE tokenTreeElement* RBRACE
    ;

/*
 * A leaf is any canonical lexer token except a grouping delimiter.
 *
 * The negated token set intentionally allows future lexer tokens to be
 * captured without modifying this grammar.
 *
 * EOF is not a leaf and is therefore not consumed here.
 *
 * If the canonical lexer uses different delimiter token names, reconcile
 * those names at the lexer/grammar integration boundary rather than
 * introducing duplicate token definitions here.
 */
tokenTreeLeaf
    : ~(LPAREN | RPAREN | LBRACK | RBRACK | LBRACE | RBRACE)
    ;