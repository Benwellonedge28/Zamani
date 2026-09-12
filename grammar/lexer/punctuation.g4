/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/lexer/punctuation.g4
 *
 * Role:
 *     Canonical lexical component for structural punctuation.
 *
 * Language:
 *     Zamani
 *
 * Target/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This file contains ANTLR grammar source only.
 *     Generated/compiler Rust integration MUST use safe Rust.
 *     No `unsafe` Rust is required or permitted by the Zamani compiler
 *     architecture.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This grammar owns the lexical recognition of punctuation whose primary
 * purpose is structural delimitation or separation of source constructs.
 *
 * It intentionally does NOT own:
 *
 *     - keywords;
 *     - identifiers;
 *     - numeric literals;
 *     - string literals;
 *     - character literals;
 *     - quantum literals;
 *     - annotations as semantic constructs;
 *     - operators;
 *     - comments;
 *     - whitespace;
 *     - AST construction;
 *     - semantic analysis;
 *     - type checking;
 *     - capability checking;
 *     - resource analysis;
 *     - classical IR;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - hardware discovery;
 *     - calibration;
 *     - runtime execution;
 *     - machine-specific resource limits.
 *
 * ============================================================================
 *
 * CANONICAL LEXICAL OWNERSHIP
 * ============================================================================
 *
 * Structural punctuation owned here:
 *
 *     (   RPAREN/LPAREN
 *     {   RBRACE/LBRACE
 *     [   RBRACKET/LBRACKET
 *     ,   COMMA
 *     .   DOT
 *     ;   SEMICOLON
 *     :   COLON
 *     @   AT
 *     #   HASH
 *
 * The following source characters/spellings appear in the broader Zamani
 * punctuation/operator specification but are intentionally owned by the
 * operator grammar rather than duplicated here:
 *
 *     ::
 *     ?
 *     !
 *     ~
 *     ->
 *     =>
 *     ..
 *     ..=
 *
 * This distinction is important:
 *
 *     specification category
 *
 * is not necessarily identical to:
 *
 *     lexical implementation owner.
 *
 * Every concrete token spelling MUST have exactly one lexical owner in the
 * assembled lexer.
 *
 * ============================================================================
 *
 * WHY SINGLE-CHARACTER PUNCTUATION IS SAFE HERE
 * ============================================================================
 *
 * Several punctuation characters are prefixes of longer tokens.
 *
 * Examples:
 *
 *     .     ..     ..=
 *     :     ::
 *     ?     ?.
 *     !     !=
 *
 * The longer forms are owned by the operator/literal grammars.
 *
 * ANTLR's combined lexer therefore resolves the complete lexical token
 * according to the assembled lexer rules and their longest-match behavior.
 *
 * This file itself deliberately does not duplicate the longer forms.
 *
 * ============================================================================
 *
 * ANNOTATIONS
 * ============================================================================
 *
 * `@` is punctuation at the lexical level.
 *
 * It is NOT an annotation token.
 *
 * For example:
 *
 *     @atom
 *     @molecule(x)
 *     @custom.attribute
 *
 * must be assembled from the punctuation marker plus the appropriate
 * identifier/name/parser constructs.
 *
 * The lexer must therefore NOT contain:
 *
 *     ATOM_ANNOTATION
 *     MOLECULE_ANNOTATION
 *     DEVICE_ANNOTATION
 *     QPU_ANNOTATION
 *
 * or similar semantic token names.
 *
 * Annotation meaning belongs to parser/AST/semantic layers.
 *
 * ============================================================================
 *
 * ATTRIBUTES
 * ============================================================================
 *
 * `#` is also structural punctuation.
 *
 * It can participate in parser-level constructs such as:
 *
 *     #[attribute]
 *
 * without making individual attributes lexical keywords.
 *
 * Attribute semantics belong to the attribute/metadata semantic layer.
 *
 * ============================================================================
 *
 * DOT
 * ============================================================================
 *
 * `.` is structural punctuation.
 *
 * It may participate in:
 *
 *     member access;
 *     qualified source constructs;
 *     numeric literals;
 *     future syntax extensions.
 *
 * It must NOT be combined with `.` to define:
 *
 *     ..
 *     ..=
 *     ?.
 *
 * because those are compound/operator forms owned elsewhere.
 *
 * Numeric literal rules must be assembled so that valid floating-point
 * literals are recognized according to the canonical literal grammar rather
 * than accidentally decomposed into INTEGER + DOT + INTEGER.
 *
 * ============================================================================
 *
 * COLON
 * ============================================================================
 *
 * `:` is structural punctuation.
 *
 * It may participate in:
 *
 *     type annotations;
 *     generic bounds;
 *     parameter declarations;
 *     pattern syntax;
 *     named fields;
 *     labels;
 *     other parser-level constructs.
 *
 * `::` is intentionally NOT defined here.
 *
 * `::` belongs to the operator/qualified-name lexical ownership contract.
 *
 * ============================================================================
 *
 * BRACKETS
 * ============================================================================
 *
 * `[` and `]` are structural delimiters.
 *
 * They must remain independent of any particular machine representation.
 *
 * They may therefore delimit:
 *
 *     arrays;
 *     indexing;
 *     slices;
 *     quantum register selections;
 *     hardware-independent resource expressions;
 *     metadata;
 *     attributes;
 *     future domain-specific syntax.
 *
 * Nothing here assumes:
 *
 *     q[0]
 *     q[1]
 *
 * or any maximum index/register size.
 *
 * The meaning and validity of an index belongs to semantic analysis.
 *
 * ============================================================================
 *
 * BRACES
 * ============================================================================
 *
 * `{` and `}` delimit parser-level blocks and aggregate constructs.
 *
 * They impose no machine or resource limit.
 *
 * ============================================================================
 *
 * PARENTHESES
 * ============================================================================
 *
 * `(` and `)` delimit:
 *
 *     calls;
 *     grouping;
 *     parameter lists;
 *     expressions;
 *     annotations;
 *     generic domain constructs;
 *     future syntax.
 *
 * No finite nesting depth is encoded in this grammar.
 *
 * Practical limits are implementation/resource limits rather than language
 * semantics.
 *
 * ============================================================================
 *
 * COMMA
 * ============================================================================
 *
 * `,` separates grammar-level list elements.
 *
 * Examples include:
 *
 *     arguments
 *     parameters
 *     fields
 *     tuple elements
 *     imports
 *     generic parameters
 *     quantum operands
 *     hardware ports
 *     resource expressions
 *
 * Cardinality is a parser/semantic concern.
 *
 * This grammar imposes no fixed maximum number of elements.
 *
 * ============================================================================
 *
 * SEMICOLON
 * ============================================================================
 *
 * `;` is a structural terminator/separator.
 *
 * Whether a particular construct requires, permits, or forbids a semicolon
 * belongs to the parser grammar.
 *
 * This lexer merely recognizes the token.
 *
 * ============================================================================
 *
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     MAX_PORTS
 *     MAX_ARRAY_SIZE
 *     MAX_TENSOR_RANK
 *     MAX_CIRCUIT_DEPTH
 *     MAX_PROGRAM_SIZE
 *     MAX_HARDWARE_SIZE
 *
 * Punctuation has no machine-size dependency.
 *
 * Therefore the lexical layer remains compatible with:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     clusters
 *     distributed systems
 *     supercomputers
 *     future computational substrates
 *
 * POCO-REAF principle:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Anywhere
 *     Forever
 *
 * is protected by keeping machine realization outside lexical punctuation.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * For every punctuation character owned by this grammar:
 *
 *     one source spelling
 *         ->
 *     one canonical token type
 *
 * There must be no duplicate lexical owner for the same spelling in the
 * assembled lexer.
 *
 * ============================================================================
 *
 * SOURCE SPANS
 * ============================================================================
 *
 * Source locations are supplied by the lexer/runtime.
 *
 * This grammar does not calculate:
 *
 *     byte offsets;
 *     line numbers;
 *     columns;
 *     UTF-8 widths;
 *     source-map entries.
 *
 * The canonical compiler Span/source-map infrastructure owns those concerns.
 *
 * ============================================================================
 *
 * UNICODE
 * ============================================================================
 *
 * The punctuation tokens in this file are ASCII structural punctuation.
 *
 * Unicode semantic symbols, such as quantum notation delimiters or
 * mathematical symbols, belong to their appropriate lexical literal/symbol
 * grammars and must not be duplicated here.
 *
 * ============================================================================
 *
 * INTEGRATION PIPELINE
 * ============================================================================
 *
 *     source text
 *         |
 *         v
 *     canonical Zamani lexer
 *         |
 *         +--> punctuation.g4
 *         +--> operators.g4
 *         +--> keywords.g4
 *         +--> identifiers.g4
 *         +--> literals.g4
 *         +--> comments.g4
 *         +--> other lexical components
 *         |
 *         v
 *     parser
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic/type/effect/resource analysis
 *         |
 *         v
 *     canonical IR
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL/hardware representations
 *         +--> control/data representations
 *         +--> resource/capability metadata
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     routing / scheduling
 *         |
 *         v
 *     target realization
 *         |
 *         v
 *     runtime / hardware
 *
 * Punctuation does not participate in reverse dependencies.
 *
 * ============================================================================
 *
 * IMPORTANT ANTLR INTEGRATION RULE
 * ============================================================================
 *
 * This file is a lexical component, not a parser grammar.
 *
 * The canonical assembled lexer must import/include this grammar according
 * to the project's ANTLR build architecture.
 *
 * It must NOT be compiled as an unrelated second lexer and then treated as
 * authoritative independently of the canonical lexer.
 *
 * ============================================================================
 */

lexer grammar ZamaniPunctuation;


/*
 * ============================================================================
 * STRUCTURAL DELIMITERS
 * ============================================================================
 *
 * Parentheses
 *
 *     ( ... )
 *
 * Used by parser constructs including:
 *
 *     calls
 *     grouping
 *     parameter lists
 *     argument lists
 *     conditions
 *     annotation arguments
 *
 * No nesting limit is encoded.
 * ============================================================================
 */

LPAREN
    : '('
    ;

RPAREN
    : ')'
    ;


/*
 * ============================================================================
 * BLOCK / AGGREGATE DELIMITERS
 * ============================================================================
 *
 * Braces
 *
 *     { ... }
 *
 * The parser determines whether a brace-delimited construct is:
 *
 *     a block;
 *     an aggregate;
 *     a module body;
 *     a hardware body;
 *     a quantum body;
 *     or another language construct.
 *
 * This lexer has no knowledge of that meaning.
 * ============================================================================
 */

LBRACE
    : '{'
    ;

RBRACE
    : '}'
    ;


/*
 * ============================================================================
 * INDEX / COLLECTION DELIMITERS
 * ============================================================================
 *
 * Brackets
 *
 *     [ ... ]
 *
 * Used by parser/semantic layers for constructs such as:
 *
 *     indexing;
 *     arrays;
 *     slices;
 *     register selection;
 *     resource expressions;
 *     attributes and metadata where applicable.
 *
 * No fixed index or collection size is encoded.
 * ============================================================================
 */

LBRACKET
    : '['
    ;

RBRACKET
    : ']'
    ;


/*
 * ============================================================================
 * LIST SEPARATOR
 * ============================================================================
 */

COMMA
    : ','
    ;


/*
 * ============================================================================
 * MEMBER / STRUCTURAL DOT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * DOT owns only the single-character spelling:
 *
 *     .
 *
 * It does NOT own:
 *
 *     ..
 *     ..=
 *     ?.
 *
 * Those spellings belong to the appropriate compound/operator rules.
 *
 * Numeric literal grammars may consume `.` as part of a larger literal
 * token. The assembled lexer must preserve the canonical literal precedence
 * and longest-match behavior.
 * ============================================================================
 */

DOT
    : '.'
    ;


/*
 * ============================================================================
 * STATEMENT / DECLARATION TERMINATOR
 * ============================================================================
 */

SEMICOLON
    : ';'
    ;


/*
 * ============================================================================
 * TYPE / DECLARATION / NAMED-FIELD SEPARATOR
 * ============================================================================
 *
 * COLON owns only:
 *
 *     :
 *
 * DOUBLE_COLON / `::` is intentionally owned by the operator/qualified-name
 * lexical contract and is NOT duplicated here.
 * ============================================================================
 */

COLON
    : ':'
    ;


/*
 * ============================================================================
 * ANNOTATION / ATTRIBUTE MARKER
 * ============================================================================
 *
 * `@` is structural punctuation.
 *
 * The lexer must NOT turn:
 *
 *     @atom
 *     @molecule
 *     @foo
 *
 * into one semantic annotation token.
 *
 * Instead:
 *
 *     @
 *     identifier
 *
 * are separate lexical components.
 *
 * Annotation interpretation belongs to parser/AST/semantic layers.
 *
 * This also prevents the lexical layer from becoming coupled to a finite
 * list of domain annotations.
 * ============================================================================
 */

AT
    : '@'
    ;


/*
 * ============================================================================
 * ATTRIBUTE / DIRECTIVE MARKER
 * ============================================================================
 *
 * `#` is structural punctuation.
 *
 * Parser-level constructs such as:
 *
 *     #[attribute]
 *
 * are assembled from:
 *
 *     HASH
 *     LBRACKET
 *     ...
 *     RBRACKET
 *
 * Individual attribute names are not lexer-owned here.
 * ============================================================================
 */

HASH
    : '#'
    ;