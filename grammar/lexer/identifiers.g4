/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/identifiers.g4
 *
 * Role:
 *     Canonical lexical definition of Zamani identifiers.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 lexer grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar requires no Rust `unsafe`.
 *     The Zamani compiler/runtime implementation MUST use safe Rust only.
 *
 * ============================================================================
 *
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This file owns ONLY the lexical shape of identifiers.
 *
 * It answers:
 *
 *     "Can this sequence of source characters be tokenized as an identifier?"
 *
 * It does NOT answer:
 *
 *     "What does this identifier mean?"
 *
 * Meaning is resolved later by:
 *
 *     parser
 *       ->
 *     AST
 *       ->
 *     name resolution
 *       ->
 *     symbol/type/effect/capability analysis
 *       ->
 *     semantic IR
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *   - IDENTIFIER token
 *   - identifier-start characters
 *   - identifier-continuation characters
 *   - Unicode identifier character classes
 *   - underscore handling
 *   - lexical distinction between identifiers and numeric literals
 *
 * DOES NOT OWN:
 *
 *   - keywords
 *   - reserved words
 *   - literals
 *   - operators
 *   - punctuation
 *   - comments
 *   - whitespace
 *   - Unicode normalization
 *   - case folding
 *   - symbol tables
 *   - scopes
 *   - name resolution
 *   - namespaces
 *   - types
 *   - generics
 *   - quantum semantics
 *   - quantum gate identity
 *   - hardware identity
 *   - device identity
 *   - resource limits
 *   - topology
 *   - compiler targets
 *   - runtime capabilities
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * Identifier length is NOT bounded here.
 *
 * There is deliberately no rule such as:
 *
 *     ID_START ID_CONTINUE{0,63}
 *
 * or:
 *
 *     MAX_IDENTIFIER_LENGTH = 256
 *
 * or any equivalent artificial limit.
 *
 * A valid identifier may therefore be as small or as large as the source
 * representation, compiler memory, parser/runtime implementation, and
 * deployment environment permit.
 *
 * This grammar MUST NOT encode:
 *
 *     - machine word size;
 *     - register width;
 *     - address width;
 *     - CPU count;
 *     - GPU count;
 *     - QPU count;
 *     - qubit count;
 *     - memory capacity;
 *     - hardware topology;
 *     - cluster size;
 *     - device count;
 *     - tensor rank limits;
 *     - tensor dimensions.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Identifier syntax is target-independent.
 *
 * The same identifier must be lexically valid regardless of whether the
 * surrounding program is eventually compiled for:
 *
 *     - a tiny embedded system;
 *     - a CPU;
 *     - a multicore CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a quantum simulator;
 *     - a heterogeneous accelerator;
 *     - a cluster;
 *     - a supercomputer;
 *     - a distributed system;
 *     - a cloud environment;
 *     - a future execution architecture.
 *
 * ============================================================================
 *
 * UNICODE POLICY
 * ============================================================================
 *
 * Zamani identifiers support Unicode identifier characters.
 *
 * Unicode normalization is deliberately NOT performed here.
 *
 * The lexer preserves source spelling.
 *
 * If Zamani adopts a canonical normalization or Unicode security policy,
 * that policy belongs to:
 *
 *     grammar/spec/
 *     semantic name resolution
 *     compiler diagnostics
 *     tooling
 *
 * and MUST NOT silently mutate source text during lexing.
 *
 * ============================================================================
 *
 * KEYWORD INTERACTION
 * ============================================================================
 *
 * Keywords are NOT duplicated in this file.
 *
 * For example, if the canonical keyword grammar reserves:
 *
 *     fn
 *     let
 *     quantum
 *     module
 *
 * then those spellings must be recognized by the keyword rules of the
 * canonical Zamani lexer before ordinary identifier matching is considered.
 *
 * This file therefore remains reusable as the identifier component of the
 * canonical lexer.
 *
 * Domain-specific names such as:
 *
 *     H
 *     X
 *     CNOT
 *     vendor_gate
 *     device_name
 *     gpu
 *     qpu_name
 *     custom_operation
 *
 * are identifiers unless the language specification explicitly reserves
 * their spelling as syntax.
 *
 * In particular, this file does not encode an exhaustive quantum, hardware,
 * vendor, accelerator, or library vocabulary.
 *
 * ============================================================================
 *
 * NUMERIC INTERACTION
 * ============================================================================
 *
 * An identifier MUST NOT begin with an ASCII decimal digit.
 *
 * This prevents lexical ambiguity between:
 *
 *     123
 *
 * and:
 *
 *     abc123
 *
 * Numeric literal syntax belongs to the numeric literal grammar.
 *
 * Unicode decimal digits are permitted in identifier continuation positions
 * but not as the first character.
 *
 * ============================================================================
 *
 * COMBINING MARKS
 * ============================================================================
 *
 * Unicode combining marks may continue an identifier after an identifier-start
 * character.
 *
 * This permits legitimate Unicode identifiers while keeping the first
 * character structurally distinguishable from a numeric literal.
 *
 * ============================================================================
 *
 * JOINERS
 * ============================================================================
 *
 * Unicode connector punctuation is permitted in continuation positions.
 *
 * Zero-width joiner / non-joiner characters are intentionally NOT silently
 * accepted unless and until the Zamani Unicode identifier specification
 * explicitly adopts them.
 *
 * This prevents invisible source distinctions from entering the language
 * without an explicit security policy.
 *
 * ============================================================================
 *
 * CASE SENSITIVITY
 * ============================================================================
 *
 * This grammar is case-sensitive.
 *
 *     value
 *
 * and:
 *
 *     Value
 *
 * are different lexical spellings.
 *
 * Case folding, case-insensitive lookup, or language-level naming policy
 * belongs to semantic/name-resolution layers and MUST NOT be implemented
 * here.
 *
 * ============================================================================
 *
 * RESERVED-WORD SAFETY
 * ============================================================================
 *
 * This grammar intentionally does not contain a semantic "is keyword"
 * predicate.
 *
 * Keyword recognition belongs to the canonical lexer assembly.
 *
 * This separation prevents:
 *
 *     identifiers.g4
 *         ->
 *     semantic keyword table
 *
 * and prevents the identifier grammar from becoming coupled to a particular
 * language version or domain vocabulary.
 *
 * ============================================================================
 *
 * GENERATED-LEXER INTEGRATION
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * should assemble this grammar together with the keyword and other lexical
 * components.
 *
 * The canonical parser continues to consume:
 *
 *     IDENTIFIER
 *
 * from:
 *
 *     ZamaniLexer
 *
 * Parser grammars MUST NOT recreate this rule.
 *
 * ============================================================================
 */

lexer grammar ZamaniIdentifiers;


/* ============================================================================
 * IDENTIFIER
 * ========================================================================= */

/*
 * A Zamani identifier consists of:
 *
 *     identifier-start identifier-continuation*
 *
 * There is intentionally no maximum repetition count.
 */
IDENTIFIER
    : IDENTIFIER_START IDENTIFIER_CONTINUE*
    ;


/* ============================================================================
 * IDENTIFIER START
 * ============================================================================
 *
 * ASCII:
 *
 *     A-Z
 *     a-z
 *     _
 *
 * Unicode:
 *
 *     Unicode letters
 *     Unicode letter numbers
 *
 * `_` is permitted as a leading identifier character.
 *
 * Decimal digits are deliberately excluded from the first position.
 */
fragment IDENTIFIER_START
    : '_'
    | [A-Z]
    | [a-z]
    | [\p{L}]
    | [\p{Nl}]
    ;


/* ============================================================================
 * IDENTIFIER CONTINUATION
 * ============================================================================
 *
 * Continuation characters include:
 *
 *     identifier-start
 *     Unicode combining marks
 *     Unicode decimal digits
 *     Unicode connector punctuation
 *
 * This provides a Unicode-capable identifier model without imposing a
 * machine-dependent maximum identifier length.
 */
fragment IDENTIFIER_CONTINUE
    : IDENTIFIER_START
    | [\p{Mn}]
    | [\p{Mc}]
    | [\p{Nd}]
    | [\p{Pc}]
    ;