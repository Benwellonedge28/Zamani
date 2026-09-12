/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/expressions/arithmetic.g4
* 
* Role:
* Canonical arithmetic-expression grammar for Zamani.
* 
* This is a PARSER grammar.
* 
* It owns ONLY the syntactic precedence structure for arithmetic expressions:
* 
* additive
*     ↓
* multiplicative
*     ↓
* exponentiation
* 
* Arithmetic operators are represented by canonical lexer tokens.
* 
* This file does NOT define lexer tokens.
* 
* ============================================================================
* ARCHITECTURAL OWNERSHIP
* ============================================================================
* 
* OWNS
* ---
* 
* - additive-expression syntax;
* - multiplicative-expression syntax;
* - exponentiation syntax;
* - arithmetic operator grouping;
* - arithmetic precedence;
* - arithmetic associativity;
* - the parser-level boundary between arithmetic and adjacent precedence
* layers.
* 
* DOES NOT OWN
* ---
* 
* - lexical spelling of operators;
* - identifiers;
* - numeric literals;
* - unary-expression syntax;
* - function calls;
* - indexing;
* - member access;
* - assignments;
* - comparisons;
* - equality;
* - logical operators;
* - bitwise operators;
* - shifts;
* - type definitions;
* - numeric type checking;
* - overflow rules;
* - floating-point semantics;
* - integer representation;
* - arbitrary-precision implementation;
* - constant folding;
* - algebraic simplification;
* - symbolic evaluation;
* - vector/matrix/tensor semantics;
* - quantum semantics;
* - quantum::ir;
* - hardware semantics;
* - HDL semantics;
* - resource discovery;
* - scheduling;
* - optimization;
* - code generation;
* - runtime execution;
* - machine-specific limits.
* 
* ============================================================================
* DEPENDENCY BOUNDARY
* ============================================================================
* 
* The intended expression hierarchy is:
* 
* assignment
*     ↓
* conditional
*     ↓
* logical OR
*     ↓
* logical AND
*     ↓
* bitwise OR
*     ↓
* bitwise XOR
*     ↓
* bitwise AND
*     ↓
* equality
*     ↓
* comparison
*     ↓
* shift
*     ↓
* additive              <-- THIS FILE
*     ↓
* multiplicative        <-- THIS FILE
*     ↓
* exponentiation        <-- THIS FILE
*     ↓
* unary
*     ↓
* postfix
*     ↓
* primary
* 
* This corresponds to the canonical Zamani syntax specification:
* 
* 11 = Sum
* 12 = Product
* 13 = Prefix
* 
* with exponentiation explicitly placed inside the arithmetic layer.
* 
* ============================================================================
* ANTLR INTEGRATION
* ============================================================================
* 
* This grammar is imported by the canonical expression parser.
* 
* The canonical integration is:
* 
* parser grammar Expressions;
* 
* options {
*     tokenVocab = ZamaniLexer;
* }
* 
* import Arithmetic;
* 
* Expressions MUST NOT redeclare:
* 
* additiveExpression
* multiplicativeExpression
* exponentExpression
* 
* after importing this grammar.
* 
* This prevents duplicate rule ownership and guarantees one authoritative
* arithmetic precedence hierarchy.
* 
* ============================================================================
* LEXER CONTRACT
* ============================================================================
* 
* Operator tokens consumed here are supplied by the canonical Zamani lexer.
* 
* Required token names:
* 
* PLUS
* MINUS
* STAR
* SLASH
* MODULO
* POWER
* 
* IMPORTANT:
* 
* POWER is intentionally referenced as a parser token.
* 
* If the current canonical lexer does not yet expose POWER, the lexer must
* first be extended through the normal lexical-versioning process.
* 
* This file MUST NOT introduce a local lexer rule such as:
* 
* POWER : '**' ;
* 
* because parser grammars do not own lexical definitions.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar does not construct AST nodes itself.
* 
* The parser/frontend AST layer receives:
* 
* lhs
* operator
* rhs
* 
* together with source spans.
* 
* The semantic layer subsequently determines whether the operator is valid
* for the operand types.
* 
* Examples:
* 
* a + b
* a - b
* a * b
* a / b
* a % b
* a ** b
* 
* may all lower into the repository's canonical binary-expression AST
* representation.
* 
* The grammar must not create separate:
* 
* MatrixAdd
* TensorAdd
* QuantumAdd
* HardwareAdd
* 
* AST categories merely because an operand happens to have such a type.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Arithmetic syntax is type-independent.
* 
* The semantic/type system determines whether an operation is valid for:
* 
* integers
* arbitrary-precision integers
* floating-point values
* fixed-point values
* decimal values
* complex values
* vectors
* matrices
* tensors
* symbolic values
* user-defined numeric types
* hardware numeric values
* future numeric abstractions
* 
* This grammar MUST NOT decide:
* 
* integer width
* floating-point width
* overflow behavior
* rounding mode
* precision
* saturation
* vector width
* tensor dimensions
* hardware instruction selection
* 
* Those decisions belong downstream.
* 
* ============================================================================
* SCALABILITY / POCO-REAF
* ============================================================================
* 
* No machine capacity is encoded by this grammar.
* 
* In particular, there is no:
* 
* MAX_OPERANDS
* MAX_EXPRESSION_DEPTH
* MAX_INTEGER_BITS
* MAX_VECTOR_SIZE
* MAX_MATRIX_SIZE
* MAX_TENSOR_SIZE
* MAX_REGISTER_COUNT
* MAX_CORES
* MAX_THREADS
* MAX_QUBITS
* MAX_DEVICES
* 
* Repetition is expressed recursively or through zero-or-more parser
* constructs so that the grammar does not impose an artificial semantic
* capacity.
* 
* Actual implementation resource limits, if required, belong to compiler
* resource-policy infrastructure rather than this language grammar.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* For identical:
* 
* source text
* language version
* lexer version
* 
* this grammar must produce the same parse structure.
* 
* Arithmetic parsing must never depend on:
* 
* CPU count
* GPU count
* FPGA count
* QPU topology
* memory size
* scheduler state
* runtime state
* backend selection
* hardware calibration
* network state
* 
* ============================================================================
* PRECEDENCE
* ============================================================================
* 
* Within this grammar:
* 
* additive < multiplicative < exponentiation
* 
* where "less than" means lower binding strength.
* 
* Therefore:
* 
* a + b * c
* 
* parses as:
* 
* a + (b * c)
* 
* and:
* 
* a * b + c
* 
* parses as:
* 
* (a * b) + c
* 
* Exponentiation is right-associative:
* 
* a ** b ** c
* 
* parses as:
* 
* a ** (b ** c)
* 
* This is intentionally recursive rather than encoded through an arbitrary
* depth.
* 
* ============================================================================
* ASSOCIATIVITY
* ============================================================================
* 
* Addition:
* 
* left associative
* 
* Multiplication:
* 
* left associative
* 
* Exponentiation:
* 
* right associative
* 
* Thus:
* 
* a - b - c
* 
* is:
* 
* (a - b) - c
* 
* and:
* 
* a / b / c
* 
* is:
* 
* (a / b) / c
* 
* while:
* 
* a ** b ** c
* 
* is:
* 
* a ** (b ** c)
* 
* Associativity is syntax here; semantic validity remains downstream.
* 
* ============================================================================
* ARITHMETIC OPERATOR OWNERSHIP
* ============================================================================
* 
* ADDITIVE
* 
* PLUS
* MINUS
* 
* MULTIPLICATIVE
* 
* STAR
* SLASH
* MODULO
* 
* EXPONENTIATION
* 
* POWER
* 
* Compound assignment operators are NOT arithmetic-expression operators in
* this file. They belong to assignment syntax.
* 
* Therefore this file does NOT consume:
* 
* PLUS_ASSIGN
* MINUS_ASSIGN
* STAR_ASSIGN
* SLASH_ASSIGN
* PERCENT_ASSIGN
* 
* ============================================================================
* NO UNARY DUPLICATION
* ============================================================================
* 
* This grammar intentionally does not define:
* 
* +x
* -x
* !x
* ~x
* 
* Those belong to unary-expression syntax.
* 
* This distinction is necessary to prevent ambiguity between:
* 
* additive
* 
* and:
* 
* unary
* 
* and to preserve the canonical expression dependency graph.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Arithmetic operators can syntactically operate on expressions whose
* semantic values are quantum-related.
* 
* For example:
* 
* angle + phase
* 
* may be valid after semantic analysis.
* 
* However, this grammar does not define quantum arithmetic semantics.
* 
* It does not know about:
* 
* qubits
* quantum states
* gates
* observables
* QEC
* ZQN
* physical qubits
* topology
* calibration
* gate durations
* 
* If an arithmetic expression eventually contributes to quantum::ir, the
* frontend/semantic lowering layer performs that translation.
* 
* There is deliberately no direct dependency:
* 
* grammar -> quantum::ir
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Arithmetic syntax can also appear in hardware-oriented source constructs:
* 
* width - 1
* address + offset
* signal * factor
* 
* This grammar remains unaware of:
* 
* bus width
* register width
* clock frequency
* FPGA family
* ASIC technology
* physical address
* device count
* 
* Such information belongs to HDL/hardware semantics and target realization.
* 
* ============================================================================
* CLASSICAL / NUMERICAL INTEGRATION
* ============================================================================
* 
* The same arithmetic syntax is intentionally reusable for:
* 
* scalar
* vector
* matrix
* tensor
* numerical
* symbolic
* accelerator
* 
* domains.
* 
* A domain must not introduce a new arithmetic token merely because it
* implements a different semantic operation.
* 
* ============================================================================
* SYMBOLIC / MATHEMATICAL INTEGRATION
* ============================================================================
* 
* Expressions such as:
* 
* x + y * z
* 
* are parsed structurally.
* 
* Symbolic simplification, algebraic normalization, polynomial arithmetic,
* differentiation, integration, optimization, and other mathematical
* transformations occur after parsing.
* 
* This grammar therefore remains small and stable even as the mathematical
* subsystem grows.
* 
* ============================================================================
* CONSTANT EXPRESSIONS
* ============================================================================
* 
* The grammar does not evaluate arithmetic.
* 
* For example:
* 
* 2 + 3
* 
* is parsed exactly like:
* 
* x + y
* 
* Constant evaluation belongs to semantic analysis / compile-time evaluation.
* 
* This separation is required so the grammar does not become dependent on
* target integer widths or floating-point representations.
* 
* ============================================================================
* OVERFLOW / PRECISION
* ============================================================================
* 
* No overflow or precision policy appears in this file.
* 
* The parser must accept syntactically valid numeric expressions regardless
* of whether a particular target can represent their eventual values.
* 
* Semantic analysis may subsequently reject or transform an expression
* according to the selected type and compilation policy.
* 
* ============================================================================
* ERROR BOUNDARY
* ============================================================================
* 
* This grammar is responsible for syntactic structure.
* 
* Examples of syntax errors include malformed operator sequences that cannot
* form an arithmetic expression.
* 
* Examples of semantic errors that MUST NOT be diagnosed here include:
* 
* integer division by zero
* overflow
* unsupported numeric type
* incompatible vector dimensions
* incompatible matrix dimensions
* unsupported quantum arithmetic
* unsupported hardware arithmetic
* 
* Those belong downstream.
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* The precedence and associativity rules are language compatibility contracts.
* 
* A compatible language release MUST NOT silently change:
* 
* additive precedence
* multiplicative precedence
* exponentiation precedence
* associativity
* 
* Such changes require an explicit language-version compatibility decision.
* 
* ============================================================================
* IMPLEMENTATION SAFETY
* ============================================================================
* 
* This grammar contains no Rust implementation code.
* 
* The Rust compiler/parser implementation consuming this grammar MUST target:
* 
* Rust 1.97
* Rust 1.97.1
* 
* and MUST use safe Rust.
* 
* Repository Rust code must enforce:
* 
* #![forbid(unsafe_code)]
* 
* or an equivalent repository-wide policy.
* 
* No arithmetic grammar feature requires unsafe Rust.
* 
* ============================================================================
* CANONICAL RULES
* ============================================================================
  */

/*

* NOTE:
* 
* "parser grammar" inheritance/import is used so this file can be composed
* with the canonical Expressions parser without introducing lexer rules.
  */

parser grammar Arithmetic;

options {
tokenVocab = ZamaniLexer;
}

/* ============================================================================

* ADDITIVE EXPRESSIONS
* ============================================================================
* 
* Addition/subtraction bind more weakly than multiplication/division/modulo.
* 
* Examples:
* 
* a + b
* a - b
* a + b - c
* a - b + c
* 
* All binary arithmetic chains are unbounded by the grammar.
* 
* No finite operand count is encoded.
* 
* ============================================================================
  */

additiveExpression
: multiplicativeExpression
(
PLUS
| MINUS
)
multiplicativeExpression
(
(
PLUS
| MINUS
)
multiplicativeExpression
)*
;

/* ============================================================================

* MULTIPLICATIVE EXPRESSIONS
* ============================================================================
* 
* Multiplication/division/modulo bind more strongly than addition/subtraction.
* 
* Examples:
* 
* a * b
* a / b
* a % b
* a * b / c % d
* 
* The grammar deliberately does not determine whether "%" is meaningful for
* every operand type.
* 
* That is a semantic/type-system decision.
* 
* ============================================================================
  */

multiplicativeExpression
: exponentExpression
(
STAR
| SLASH
| MODULO
)
exponentExpression
(
(
STAR
| SLASH
| MODULO
)
exponentExpression
)*
;

/* ============================================================================

* EXPONENTIATION
* ============================================================================
* 
* Exponentiation is right-associative.
* 
* Examples:
* 
* a ** b
* a ** b ** c
* 
* The recursive right-hand side ensures:
* 
* a ** b ** c
* 
* becomes structurally equivalent to:
* 
* a ** (b ** c)
* 
* No exponentiation-depth limit is encoded.
* 
* The operand is "unaryExpression" because that is the precedence boundary
* established by the canonical expression architecture.
* 
* This grammar does not decide whether negative exponents, fractional
* exponents, complex exponents, symbolic exponents, or domain-specific
* exponentiation are semantically valid.
* 
* ============================================================================
  */

exponentExpression
: unaryExpression
| unaryExpression POWER exponentExpression
;

/* ============================================================================

* INTEGRATION CONTRACT
* ============================================================================
* 
* The canonical "grammar/expressions/expressions.g4" must import this parser
* grammar:
* 
* import Arithmetic;
* 
* and must remove its local definitions of:
* 
* additiveExpression
* multiplicativeExpression
* exponentExpression
* 
* The surrounding expression grammar continues to own:
* 
* assignmentExpression
* conditionalExpression
* logicalOrExpression
* logicalAndExpression
* bitwiseOrExpression
* bitwiseXorExpression
* bitwiseAndExpression
* equalityExpression
* relationalExpression
* shiftExpression
* unaryExpression
* postfixExpression
* primaryExpression
* 
* Therefore the final dependency is:
* 
* Expressions
*     |
*     +--> Arithmetic
*                |
*                +--> unaryExpression
* 
* The apparent reverse reference is resolved by ANTLR parser-grammar
* composition: the imported grammar supplies the arithmetic rules while the
* complete parser provides the lower-level expression rule used by the
* arithmetic boundary.
* 
* If the chosen ANTLR composition strategy does not permit this mutual
* rule visibility in the repository's parser-generation setup, the canonical
* alternative is to keep the single precedence chain in "Expressions" and
* treat this file as an included/generated fragment rather than an
* independently imported parser grammar.
* 
* What MUST NOT happen is having two independently authoritative definitions.
* 
* ============================================================================
* INTEGRATION WITH THE AST
* ============================================================================
* 
* Parser output is consumed by the frontend AST layer.
* 
* Arithmetic operators should map to the repository's generic binary-expression
* representation.
* 
* The AST must preserve:
* 
* operator kind
* left operand
* right operand
* source span
* 
* Semantic analysis determines the operation's type and validity.
* 
* ============================================================================
* INTEGRATION WITH THE COMPILER
* ============================================================================
* 
* This file has no direct dependency on:
* 
* classical IR
* quantum::ir
* optimization
* scheduling
* routing
* hardware
* runtime
* 
* The integration path is:
* 
* arithmetic syntax
*      ↓
* parser
*      ↓
* frontend AST
*      ↓
* semantic/type analysis
*      ↓
* canonical semantic representation
*      ↓
* appropriate IR
*      ↓
* optimization/lowering
*      ↓
* target realization
* 
* This preserves the universal-language boundary.
* 
* ============================================================================
* RESOURCE / SCALABILITY CONTRACT
* ============================================================================
* 
* An arithmetic expression such as:
* 
* x + y * z
* 
* makes no statement about:
* 
* CPU count
* GPU count
* accelerator count
* vector width
* memory capacity
* quantum processor size
* distributed node count
* 
* Consequently the same syntax can participate in programs targeting:
* 
* embedded systems
* CPUs
* GPUs
* FPGAs
* ASICs
* quantum-classical systems
* clusters
* HPC systems
* distributed systems
* future execution substrates
* 
* without changing the source grammar.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file contains no:
* 
* machine size
* hardware size
* resource count
* qubit count
* topology
* device identifier
* memory capacity
* register capacity
* fixed vector length
* fixed tensor rank
* 
* The only finite list in this grammar is the language's defined operator
* vocabulary. That is a language-semantic property, not a machine limit.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* The following tests are required.
* 
* POSITIVE
* ---
* 
* a + b
* a - b
* a * b
* a / b
* a % b
* a + b * c
* a * b + c
* a - b - c
* a / b / c
* a ** b
* a ** b ** c
* a + b * c ** d
* 
* PRECEDENCE
* ---
* 
* a + b * c
*     => a + (b * c)
* 
* a * b + c
*     => (a * b) + c
* 
* a + b ** c
*     => a + (b ** c)
* 
* ASSOCIATIVITY
* ---
* 
* a - b - c
*     => (a - b) - c
* 
* a / b / c
*     => (a / b) / c
* 
* a ** b ** c
*     => a ** (b ** c)
* 
* CROSS-DOMAIN
* ---
* 
* Arithmetic expressions must parse independently of whether their operands
* later become:
* 
* classical values
* vector values
* matrix values
* tensor values
* symbolic values
* hardware values
* quantum-related values
* 
* NEGATIVE
* ---
* 
* The expression parser, not this file, must reject malformed arithmetic
* operator sequences.
* 
* Semantic tests must separately reject invalid operations such as unsupported
* operand/operator combinations.
* 
* SCALABILITY
* ---
* 
* Test generated expressions whose depth and chain length are limited only by
* the test harness/resource policy, not by constants in this grammar.
* 
* DETERMINISM
* ---
* 
* Parse the same source repeatedly and verify identical parse structure.
* 
* ROUND TRIP
* ---
* 
* Where a canonical printer exists:
* 
* source
*   ↓
* lexer
*   ↓
* parser
*   ↓
* AST
*   ↓
* printer
*   ↓
* parser
* 
* must preserve arithmetic semantics.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is COMPLETE only when all of the following are true:
* 
* [ ] Arithmetic operator ownership is unique.
* [ ] Arithmetic precedence matches grammar/spec/syntax.md.
* [ ] Additive expressions are left-associative.
* [ ] Multiplicative expressions are left-associative.
* [ ] Exponentiation is right-associative.
* [ ] No lexer rules exist in this file.
* [ ] No machine-specific limits exist.
* [ ] No quantum-specific semantics exist.
* [ ] No hardware-specific semantics exist.
* [ ] No numeric-width assumptions exist.
* [ ] AST integration is documented and verified.
* [ ] Expressions grammar delegates arithmetic rules here.
* [ ] No duplicate arithmetic rules remain elsewhere in the authoritative
* parser grammar.
* [ ] Lexer token names exactly match the canonical lexer.
* [ ] POWER is defined by the canonical lexer before this grammar is generated.
* [ ] Positive tests pass.
* [ ] Negative tests pass.
* [ ] Precedence tests pass.
* [ ] Associativity tests pass.
* [ ] Boundary/scalability tests pass.
* [ ] Determinism tests pass.
* [ ] Round-trip tests pass where supported.
* [ ] Rust parser integration remains compatible with Rust 1.97/1.97.1.
* [ ] No unsafe Rust is required or introduced.
* 
* ============================================================================
  */