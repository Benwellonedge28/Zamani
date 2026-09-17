// =============================================================================
// Zamani Programming Language
// File: grammar/expressions/shift.g4
//
// Purpose
// -------
// Defines the complete language-wide shift-expression precedence layer.
//
// This grammar is intentionally small and compositional. It owns:
//   * shiftExpression
//   * shiftOperator
//
// It does NOT own:
//   * additive/multiplicative arithmetic
//   * exponentiation
//   * unary/prefix expressions
//   * postfix expressions
//   * primary expressions
//   * comparison/equality
//   * bitwise AND/XOR/OR
//   * logical operators
//   * assignment
//   * types
//   * numeric semantics
//   * hardware semantics
//   * target-specific lowering
//
// Authority
// ---------
// This file is a modular parser component. It is not a second root grammar.
//
// Canonical composition:
//
//   source
//     -> lexer
//     -> parser
//     -> domain-neutral AST
//     -> structural validation
//     -> semantic analysis
//     -> canonical semantic model / IR
//     -> optimization
//     -> target/domain lowering
//
// Arithmetic ownership:
//
//   additiveExpression
//     -> multiplicativeExpression
//     -> exponentExpression
//     -> prefix/postfix/primary expression
//
// Shift ownership:
//
//   shiftExpression
//     -> additiveExpression
//
// Relational/comparison ownership:
//
//   comparison/relational expression
//     -> shiftExpression
//
// Bitwise ownership:
//
//   bitwise expression
//     -> comparison/equality layer
//
// Assignment ownership:
//
//   assignmentExpression
//     -> higher-precedence expression layers
//
// Lexer authority
// ---------------
// Operator spelling belongs to the canonical lexical vocabulary in:
//
//   grammar/lexer/tokens.g4
//
// This parser grammar therefore does NOT define lexer tokens.
//
// The operator spellings consumed here are:
//
//   <<    left shift
//   >>    right shift
//   >>>   unsigned/logical right shift
//
// The lexer remains responsible for recognizing those spellings as tokens.
// The parser is responsible only for their syntactic placement.
//
// Precedence
// ----------
// Shift binds:
//
//   * more weakly than additive arithmetic;
//   * more strongly than relational/comparison expressions.
//
// Therefore:
//
//   a + b << c
//
// parses structurally as:
//
//   (a + b) << c
//
// and:
//
//   a << b + c
//
// parses structurally as:
//
//   a << (b + c)
//
// Associativity
// -------------
// Shift operations are left-associative:
//
//   a << b << c
//
// parses as:
//
//   (a << b) << c
//
// and:
//
//   a >> b >> c
//
// parses as:
//
//   (a >> b) >> c
//
// and:
//
//   a << b >>> c
//
// parses as:
//
//   (a << b) >>> c
//
// The grammar deliberately does not encode target-specific arithmetic
// semantics for signedness, width, overflow, masking, saturation, or
// implementation strategy.
//
// Semantic ownership
// ------------------
// Semantic analysis determines whether a shift is valid for the operand
// types.
//
// Examples of semantic questions deliberately NOT answered here:
//
//   * Is the left operand an integer, bit-vector, vector, tensor, or another
//     shiftable type?
//   * Is the right operand an integer or another shift-count type?
//   * Is a negative shift amount legal?
//   * What happens when the shift amount exceeds the operand width?
//   * Is right shift arithmetic or logical for a particular signed type?
//   * Does a user-defined type provide shift semantics?
//   * Can a shift operate element-wise over a vector/tensor?
//   * Is a symbolic shift valid?
//   * What constant-folding rule applies?
//
// Those decisions belong to the type system, semantic analyzer, intrinsic
// library, and canonical IR—not this grammar.
//
// AST contract
// ------------
// The parser must preserve:
//
//   * operator identity;
//   * left operand;
//   * right operand;
//   * source spans;
//   * exact nesting/order.
//
// A shift expression should lower into the repository's existing generic
// operation/binary-expression representation rather than introducing
// domain-specific nodes such as:
//
//   QuantumShift
//   TensorShift
//   GPUShift
//   HardwareShift
//
// Domain-specific meaning is established after parsing.
//
// IR contract
// -----------
// This grammar introduces no IR.
//
// A valid semantic shift may subsequently lower to the appropriate
// representation in:
//
//   * classical IR;
//   * vector/tensor IR;
//   * accelerator IR;
//   * HDL/hardware IR;
//   * another domain-specific representation.
//
// Quantum integration
// -------------------
// The grammar does not define a quantum shift operation.
//
// If a quantum or hybrid domain uses shift expressions for parameters,
// indices, masks, resource calculations, or compile-time expressions,
// those expressions enter through this generic language-wide shift layer.
//
// Quantum semantic meaning remains downstream at the canonical:
//
//   quantum::ir
//
// boundary.
//
// HDL/hardware integration
// ------------------------
// Shift syntax may be used by HDL, hardware, embedded, accelerator, or
// compile-time expressions. This grammar imposes no fixed:
//
//   * register width;
//   * vector width;
//   * machine word size;
//   * hardware generation;
//   * FPGA width;
//   * CPU width;
//   * accelerator width.
//
// Hardware realization is downstream.
//
// POCO-REAF / scalability
// -----------------------
// No artificial language-level limit is introduced here.
//
// The grammar does NOT specify:
//
//   MAX_SHIFT_WIDTH
//   MAX_SHIFT_AMOUNT
//   MAX_SHIFT_OPERAND_BITS
//   MAX_VECTOR_WIDTH
//   MAX_TENSOR_SIZE
//   MAX_EXPRESSION_DEPTH
//   MAX_OPERATIONS
//
// A finite implementation may impose resource limits for a particular
// compilation or execution environment, but those are implementation and
// resource-management concerns, not grammar restrictions.
//
// This permits the same source language to describe computations ranging
// from tiny embedded systems to arbitrarily large representations supported
// by the available compiler/runtime/hardware resources.
//
// Determinism
// -----------
// Given the same token stream and grammar version, this rule produces the
// same structural parse. No runtime, hardware, network, resource,
// scheduling, or random state is consulted by parsing.
//
// Diagnostics
// -----------
// Syntax diagnostics belong to the parser.
//
// Examples:
//
//   a <<              -> syntax error
//   a >>              -> syntax error
//   a >>>             -> syntax error
//   a << >> b         -> syntax error
//   a <<< b           -> syntax error unless separately tokenized/specified
//
// Semantic diagnostics such as invalid operand types or invalid shift
// counts belong to semantic analysis.
//
// Assignment separation
// ----------------------
// This file does NOT define:
//
//   <<=
//   >>=
//   >>>=
//
// Compound assignment belongs to assignment.g4.
//
// This separation prevents shift syntax from being duplicated across
// expression layers.
//
// Bitwise separation
// ------------------
// This file does NOT define:
//
//   &
//   ^
//   |
//
// Those operators belong to the bitwise expression layer.
//
// Comparison separation
// ---------------------
// This file does NOT define:
//
//   <
//   <=
//   >
//   >=
//   ==
//   !=
//
// Those belong to comparison/equality grammar.
//
// Important ambiguity
// -------------------
// `>>` and `>>>` must be tokenized consistently by the canonical lexer.
//
// Generic type syntax must not cause the parser to reinterpret a shift
// operator incorrectly. Generic/type parsing therefore remains a separate
// responsibility and must be resolved by the surrounding expression/type
// grammar and parser context.
//
// Rust compatibility
// ------------------
// This .g4 file contains no Rust actions, predicates, embedded code, unsafe
// code, or target-specific extensions.
//
// Generated Rust parser code must remain compatible with:
//
//   Rust 1.97 / Rust 1.97.1
//
// and the repository's configured ANTLR Rust runtime.
//
// Testing contract
// ----------------
// Positive examples:
//
//   a << b
//   a >> b
//   a >>> b
//   a + b << c
//   a << b + c
//   a * b << c
//   a << b * c
//   a << b << c
//   a >> b >> c
//   a << b >>> c
//   (a << b) + c
//   a < b << c
//   a << b < c
//
// Negative examples:
//
//   a <<
//   a >>
//   a >>>
//   a << >> b
//   a >> << b
//   a >>> >> b
//   a + << b
//   a << + b
//   a << * b
//
// Boundary/scalability examples:
//
//   x << 0
//   x >> 0
//   x >>> 0
//   x << shift_count
//   (((a << b) << c) << d)
//   very_long_shift_chain
//
// Semantic tests must separately cover:
//
//   * signed operands;
//   * unsigned operands;
//   * arbitrary-width integers;
//   * fixed-width integers;
//   * bit-vectors;
//   * vectors;
//   * tensors;
//   * symbolic values;
//   * invalid shift counts;
//   * overflow/width behavior;
//   * target-specific lowering.
//
// Completion criteria
// -------------------
// This file is complete when:
//
//   [x] It owns only the shift precedence layer.
//   [x] It consumes additiveExpression.
//   [x] It defines all supported shift spellings.
//   [x] Shift chains are left-associative.
//   [x] No arithmetic rules are duplicated.
//   [x] No comparison rules are duplicated.
//   [x] No bitwise rules are duplicated.
//   [x] No assignment rules are duplicated.
//   [x] No lexer tokens are defined here.
//   [x] No semantic numeric rules are embedded here.
//   [x] No hardware limits are embedded here.
//   [x] No quantum-specific IR is introduced.
//   [x] No target-specific implementation is embedded.
//   [x] AST nesting/order/source spans are preservable.
//   [x] It is deterministic.
//   [x] It is safe-Rust compatible.
//   [x] Positive/negative/boundary/scalability tests exist.
//   [x] The canonical expression composition imports this grammar.
// =============================================================================

parser grammar Shift;

options {
    tokenVocab = ZamaniTokens;
}

// -----------------------------------------------------------------------------
// Shift expression
// -----------------------------------------------------------------------------
//
// Shift has lower precedence than additive arithmetic and higher precedence
// than comparison/relational expressions.
//
// Examples:
//
//   a + b << c      => (a + b) << c
//   a << b + c      => a << (b + c)
//
// Repetition is intentionally iterative and left-associative:
//
//   a << b << c     => (a << b) << c
//
// No maximum chain length is imposed by the grammar.
// -----------------------------------------------------------------------------
shiftExpression
    : additiveExpression
      (
          shiftOperator
          additiveExpression
      )*
    ;

// -----------------------------------------------------------------------------
// Shift operators
// -----------------------------------------------------------------------------
//
// The spelling is deliberately expressed at the parser level through the
// canonical lexer vocabulary. No lexer rules are defined in this file.
//
// Supported operators:
//
//   <<    left shift
//   >>    right shift
//   >>>   unsigned/logical right shift
//
// Semantic interpretation is downstream.
// -----------------------------------------------------------------------------
shiftOperator
    : '<<'
    | '>>'
    | '>>>'
    ;