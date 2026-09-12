/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/bitwise.g4
 *
 * Role:
 *     Canonical parser component for binary bitwise-expression syntax.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust implementation code.
 *     It introduces no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the syntactic precedence hierarchy for binary bitwise
 * operators:
 *
 *                         bitwiseExpression
 *                                |
 *                                v
 *                         bitwiseOrExpression
 *                                |
 *                                v
 *                        bitwiseXorExpression
 *                                |
 *                                v
 *                        bitwiseAndExpression
 *                                |
 *                                v
 *                        equalityExpression
 *
 * The hierarchy is:
 *
 *     bitwise AND
 *         &
 *
 *     bitwise XOR
 *         ^
 *
 *     bitwise OR
 *         |
 *
 * with AND binding more tightly than XOR, and XOR binding more tightly than OR.
 *
 * Example:
 *
 *     a | b ^ c & d
 *
 * parses structurally as:
 *
 *     a | (b ^ (c & d))
 *
 * It does NOT decide whether the operands are:
 *
 *     integers
 *     arbitrary-width integers
 *     fixed-width integers
 *     bit vectors
 *     packed data
 *     SIMD values
 *     vectors
 *     masks
 *     hardware signals
 *     registers
 *     tensors
 *     accelerator values
 *     quantum/classical representations
 *     user-defined types
 *
 * Those decisions belong to semantic analysis and downstream IR/lowering.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *   - binary bitwise-expression syntax;
 *   - bitwise AND syntax;
 *   - bitwise XOR syntax;
 *   - bitwise OR syntax;
 *   - precedence between those operators;
 *   - left-associative chaining of those operators;
 *   - the public bitwiseExpression parser entry rule;
 *   - syntactic composition with equalityExpression.
 *
 * DOES NOT OWN:
 *
 *   - operator lexical spelling;
 *   - lexer token definitions;
 *   - identifiers;
 *   - literals;
 *   - primary expressions;
 *   - unary operators;
 *   - arithmetic expressions;
 *   - shift expressions;
 *   - comparison expressions;
 *   - equality-expression semantics;
 *   - assignment expressions;
 *   - logical expressions;
 *   - function calls;
 *   - indexing;
 *   - member access;
 *   - types;
 *   - type checking;
 *   - operator overload resolution;
 *   - implicit conversions;
 *   - constant evaluation;
 *   - compile-time evaluation;
 *   - ownership;
 *   - borrowing;
 *   - lifetimes;
 *   - effects;
 *   - capabilities;
 *   - resources;
 *   - hardware selection;
 *   - hardware topology;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - simulation;
 *   - classical IR;
 *   - quantum::ir;
 *   - runtime behavior;
 *   - backend selection;
 *   - ABI selection;
 *   - machine-specific limits.
 *
 * ============================================================================
 * OPERATOR OWNERSHIP
 * ============================================================================
 *
 * The canonical lexer owns:
 *
 *     AMPERSAND
 *     CARET
 *     PIPE
 *
 * Their source spellings are therefore NOT repeated in this parser grammar.
 *
 * The parser owns their syntactic precedence.
 *
 * Semantic analysis owns their meaning.
 *
 * This preserves the repository boundary:
 *
 *     characters
 *         |
 *         v
 *     ZamaniLexer
 *         |
 *         v
 *     operator tokens
 *         |
 *         v
 *     bitwise.g4
 *         |
 *         v
 *     expression AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     canonical semantic representation / IR
 *
 * ============================================================================
 * OPERATOR SET
 * ============================================================================
 *
 * This file intentionally recognizes exactly the currently established
 * binary bitwise operators:
 *
 *     AMPERSAND    &
 *     CARET        ^
 *     PIPE         |
 *
 * It does NOT redefine:
 *
 *     TILDE        ~
 *
 * because TILDE is a unary operator owned by unary.g4.
 *
 * It does NOT redefine:
 *
 *     LEFT_SHIFT   <<
 *     RIGHT_SHIFT  >>
 *
 * because shiftExpression owns shift precedence.
 *
 * It does NOT redefine:
 *
 *     AMP_ASSIGN
 *     CARET_ASSIGN
 *     PIPE_ASSIGN
 *
 * because compound assignment belongs to assignment-expression syntax.
 *
 * ============================================================================
 * PRECEDENCE CONTRACT
 * ============================================================================
 *
 * The complete binary-expression precedence direction is conceptually:
 *
 *     logical OR
 *         |
 *     logical AND
 *         |
 *     bitwise OR
 *         |
 *     bitwise XOR
 *         |
 *     bitwise AND
 *         |
 *     equality
 *         |
 *     comparison
 *         |
 *     shift
 *         |
 *     additive
 *         |
 *     multiplicative
 *         |
 *     unary
 *         |
 *     postfix / primary
 *
 * The exact surrounding hierarchy is owned by the corresponding expression
 * grammar components.
 *
 * This file owns only:
 *
 *     bitwise OR
 *     bitwise XOR
 *     bitwise AND
 *
 * This prevents precedence duplication between:
 *
 *     expressions.g4
 *     binary.g4
 *     arithmetic.g4
 *     comparison.g4
 *     logical.g4
 *     unary.g4
 *     assignment.g4
 *
 * ============================================================================
 * ASSOCIATIVITY
 * ============================================================================
 *
 * Bitwise binary operators are parsed as left-associative chains.
 *
 * Therefore:
 *
 *     a & b & c
 *
 * has the syntactic structure:
 *
 *     (a & b) & c
 *
 * Likewise:
 *
 *     a ^ b ^ c
 *
 * becomes:
 *
 *     (a ^ b) ^ c
 *
 * and:
 *
 *     a | b | c
 *
 * becomes:
 *
 *     (a | b) | c
 *
 * The grammar uses iterative operator chains rather than recursive
 * left-recursive rules. This keeps arbitrary-length chains out of recursive
 * parser call structure and avoids introducing a language-level operator-count
 * limit.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This component depends on:
 *
 *     comparison.g4
 *
 * which MUST expose:
 *
 *     comparisonExpression
 *
 * as the expression immediately below the bitwise layer.
 *
 * The dependency direction is:
 *
 *     logical.g4
 *          |
 *          v
 *     bitwise.g4
 *          |
 *          v
 *     comparison.g4
 *          |
 *          v
 *     lower expression layers
 *
 * The reverse dependency is forbidden.
 *
 * In particular:
 *
 *     comparison.g4 MUST NOT import bitwise.g4
 *
 * and:
 *
 *     bitwise.g4 MUST NOT import expressions.g4
 *
 * because expressions.g4 is the higher-level composition layer.
 *
 * This prevents:
 *
 *     grammar -> grammar -> grammar -> cycle
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar and uses the canonical Zamani lexer vocabulary.
 *
 * The canonical parser architecture is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *                 |
 *                 v
 *     token vocabulary
 *                 |
 *                 v
 *     modular parser grammars
 *                 |
 *                 v
 *     grammar/antlr/ZamaniParser.g4
 *
 * This component therefore does not define lexer rules.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not construct an AST directly.
 *
 * The parser output must preserve enough structure for the frontend AST layer
 * to represent:
 *
 *     BitwiseOr
 *     BitwiseXor
 *     BitwiseAnd
 *
 * together with:
 *
 *     left operand
 *     operator token
 *     right operand
 *     source span
 *
 * For a chain such as:
 *
 *     a | b ^ c & d
 *
 * the parser must preserve the precedence structure rather than flattening
 * every operator into an undifferentiated binary node.
 *
 * The AST layer may subsequently normalize the iterative parse-chain form
 * into canonical binary-expression nodes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar deliberately accepts syntactically valid operand combinations
 * without deciding whether those combinations are semantically legal.
 *
 * Examples:
 *
 *     a & b
 *     a ^ b
 *     a | b
 *
 * are syntactically valid.
 *
 * Whether they are valid for the resolved types is determined downstream.
 *
 * Semantic analysis may determine that an operation applies to:
 *
 *     integer values
 *     arbitrary-precision integers
 *     bit vectors
 *     masks
 *     packed values
 *     hardware signals
 *     vector types
 *     accelerator-specific values
 *     user-defined operator implementations
 *
 * or reject the operation for the resolved types.
 *
 * This file MUST NOT encode fixed operand widths such as:
 *
 *     8
 *     16
 *     32
 *     64
 *     128
 *     256
 *
 * or any other implementation-specific width.
 *
 * ============================================================================
 * QUANTUM / CLASSICAL / HDL INTEGRATION
 * ============================================================================
 *
 * Bitwise syntax is domain-neutral.
 *
 * The same source-level operator structure may eventually be lowered into
 * different semantic representations depending on the resolved operand type
 * and program context.
 *
 * Classical example:
 *
 *     mask & value
 *
 * HDL example:
 *
 *     signal_a ^ signal_b
 *
 * Accelerator example:
 *
 *     vector_a | vector_b
 *
 * Quantum-related expressions may appear as operands only where the semantic
 * type system explicitly permits such an operation.
 *
 * This grammar itself MUST NOT decide that a bitwise expression:
 *
 *     creates a quantum gate
 *     allocates a qubit
 *     measures a qubit
 *     changes a physical qubit
 *     invokes QEC
 *     invokes ZQN
 *     selects a quantum backend
 *
 * If a valid semantic operation ultimately lowers into quantum::ir, that
 * lowering occurs after parsing and semantic analysis.
 *
 * Therefore:
 *
 *     grammar
 *         -> AST
 *         -> semantic analysis
 *         -> quantum::ir
 *
 * and never:
 *
 *     grammar
 *         -> private quantum IR
 *
 * ============================================================================
 * HARDWARE / HDL CONTRACT
 * ============================================================================
 *
 * Bitwise operators may be useful for hardware-oriented expressions, but this
 * grammar does not encode:
 *
 *     register count
 *     signal width
 *     bus width
 *     FPGA LUT count
 *     ASIC resources
 *     clock frequency
 *     device topology
 *     physical placement
 *
 * Hardware-specific constraints belong to:
 *
 *     type checking
 *     capability checking
 *     resource checking
 *     hardware description
 *     target selection
 *     lowering
 *
 * and not to this grammar.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A Zamani source program describes computation, not the size of the machine
 * that happens to execute it.
 *
 * This grammar therefore contains no:
 *
 *     MAX_BITS
 *     MAX_WIDTH
 *     MAX_OPERANDS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_EXPRESSION_COUNT
 *     MAX_VECTOR_WIDTH
 *     MAX_REGISTER_WIDTH
 *     MAX_SIGNAL_WIDTH
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * or equivalent fixed machine restrictions.
 *
 * An implementation may impose operational limits because of available
 * memory, parser configuration, compilation resources, or runtime policy.
 *
 * Such limits MUST remain implementation/resource policy and MUST NOT become
 * source-language syntax restrictions.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses repetition operators for arbitrary-length bitwise chains:
 *
 *     (AMPERSAND equalityExpression)*
 *     (CARET bitwiseAndExpression)*
 *     (PIPE bitwiseXorExpression)*
 *
 * There is no grammar-defined maximum number of operators.
 *
 * There is no grammar-defined maximum operand width.
 *
 * There is no grammar-defined maximum program size.
 *
 * There is no grammar-defined maximum nesting count.
 *
 * Practical resource exhaustion is an implementation concern rather than a
 * semantic language limit.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given the same token stream, this grammar must produce the same parse
 * structure.
 *
 * The operator hierarchy is unambiguous:
 *
 *     & > ^ > |
 *
 * Parentheses and lower/higher expression layers determine explicit grouping.
 *
 * No semantic state, hardware state, runtime state, or backend state may be
 * consulted while parsing these rules.
 *
 * ============================================================================
 * ERROR-RECOVERY CONTRACT
 * ============================================================================
 *
 * Syntax errors are parser concerns.
 *
 * This grammar must not:
 *
 *     print diagnostics;
 *     write to stdout;
 *     access files;
 *     access networks;
 *     inspect hardware;
 *     inspect runtime resources;
 *     silently reinterpret invalid operators;
 *     convert invalid syntax into comments.
 *
 * The canonical parser/frontend diagnostic layer owns error presentation and
 * source-span reporting.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing Zamani operator tokens remain authoritative:
 *
 *     AMPERSAND
 *     CARET
 *     PIPE
 *
 * Existing source forms therefore remain valid:
 *
 *     a & b
 *     a ^ b
 *     a | b
 *
 * No new operator spelling is introduced by this file.
 *
 * This is important because operator spellings belong to:
 *
 *     grammar/lexer/operators.g4
 *
 * while precedence belongs here.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar itself is language/runtime independent.
 *
 * The generated Zamani frontend must integrate with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and repository policy MUST prohibit unsafe Rust.
 *
 * No embedded Rust actions, predicates, or unsafe implementation code are
 * required by this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include at least:
 *
 *     a & b
 *     a ^ b
 *     a | b
 *
 *     a & b & c
 *     a ^ b ^ c
 *     a | b | c
 *
 *     a | b ^ c
 *     a ^ b & c
 *     a | b ^ c & d
 *
 *     (a | b) & c
 *     a | (b & c)
 *     (a ^ b) | (c & d)
 *
 *     nested expressions containing bitwise operators
 *     calls used as operands
 *     indexed values used as operands
 *     member-access expressions used as operands
 *     literal operands where the lower grammar permits them
 *
 * Precedence tests MUST establish:
 *
 *     a | b ^ c
 *
 * as:
 *
 *     a | (b ^ c)
 *
 * and:
 *
 *     a ^ b & c
 *
 * as:
 *
 *     a ^ (b & c)
 *
 * and:
 *
 *     a | b ^ c & d
 *
 * as:
 *
 *     a | (b ^ (c & d))
 *
 * Negative tests MUST include malformed forms such as:
 *
 *     a &
 *     a ^
 *     a |
 *     & b
 *     ^ b
 *     | b
 *     a & & b
 *     a ^ ^ b
 *     a | | b
 *
 * Boundary tests MUST include:
 *
 *     very long operator chains;
 *     very large source expressions;
 *     deeply nested parenthesized expressions;
 *     large arbitrary-width operand representations where the lower grammar
 *     permits them.
 *
 * The tests MUST NOT encode an artificial language maximum merely because a
 * test fixture happens to use a particular size.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Fixed machine/resource values:
 *
 *     NONE
 *
 * Fixed bit width:
 *
 *     NONE
 *
 * Fixed operand count:
 *
 *     NONE
 *
 * Fixed expression depth:
 *
 *     NONE
 *
 * Fixed hardware topology:
 *
 *     NONE
 *
 * Fixed quantum resource count:
 *
 *     NONE
 *
 * Fixed backend:
 *
 *     NONE
 *
 * Any future limit found in this file must be classified as:
 *
 *     1. language semantic requirement
 *     2. target-specific requirement
 *     3. resource constraint
 *     4. implementation limitation
 *     5. accidental hard-coding
 *     6. test-only limitation
 *     7. documentation-only limitation
 *
 * Accidental hard-coding MUST be removed.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before declaring this file complete:
 *
 * [ ] ZamaniLexer provides AMPERSAND.
 * [ ] ZamaniLexer provides CARET.
 * [ ] ZamaniLexer provides PIPE.
 * [ ] comparison.g4 exports comparisonExpression.
 * [ ] comparison.g4 does not import bitwise.g4.
 * [ ] logical.g4 consumes bitwiseExpression.
 * [ ] expressions.g4 exposes the complete expression hierarchy.
 * [ ] binary.g4 does not redefine these precedence rules.
 * [ ] unary.g4 owns unary TILDE.
 * [ ] assignment.g4 owns AMP_ASSIGN, CARET_ASSIGN and PIPE_ASSIGN.
 * [ ] shift expressions remain owned by the shift layer.
 * [ ] no local lexer rules exist here.
 * [ ] no semantic actions exist here.
 * [ ] no Rust code exists here.
 * [ ] no unsafe code is introduced.
 * [ ] no machine-specific limits exist here.
 * [ ] parser tests cover precedence.
 * [ ] parser tests cover associativity.
 * [ ] negative tests cover malformed chains.
 * [ ] scalability tests do not introduce artificial source limits.
 *
 * ============================================================================
 */

parser grammar bitwise;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * PUBLIC ENTRY
 * ============================================================================
 *
 * Higher-level expression grammars should consume:
 *
 *     bitwiseExpression
 *
 * rather than reaching directly into the individual precedence layers.
 *
 * This keeps the internal hierarchy replaceable without forcing every
 * downstream grammar to depend on all implementation details.
 */
bitwiseExpression
    : bitwiseOrExpression
    ;


/*
 * ============================================================================
 * BITWISE OR
 * ============================================================================
 *
 * Lowest precedence within the bitwise family.
 *
 *     a | b | c
 *
 * is represented as an ordered left-associative chain.
 *
 * The semantic/AST layer is responsible for lowering the chain into canonical
 * binary-expression nodes while preserving source order and spans.
 */
bitwiseOrExpression
    : bitwiseXorExpression
      (
          PIPE
          bitwiseXorExpression
      )*
    ;


/*
 * ============================================================================
 * BITWISE XOR
 * ============================================================================
 *
 * Higher precedence than bitwise OR and lower precedence than bitwise AND.
 *
 *     a ^ b ^ c
 *
 * is parsed as a left-associative chain.
 */
bitwiseXorExpression
    : bitwiseAndExpression
      (
          CARET
          bitwiseAndExpression
      )*
    ;


/*
 * ============================================================================
 * BITWISE AND
 * ============================================================================
 *
 * Highest precedence within the binary bitwise family.
 *
 * The operand below this level is equalityExpression.
 *
 * This is intentional:
 *
 *     equality
 *         |
 *     bitwise AND
 *         |
 *     bitwise XOR
 *         |
 *     bitwise OR
 *
 * Therefore:
 *
 *     a == b & c == d
 *
 * is structurally governed by the equality layer before entering this
 * bitwise layer, according to the repository's canonical expression hierarchy.
 */
bitwiseAndExpression
    : equalityExpression
      (
          AMPERSAND
          equalityExpression
      )*
    ;