/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/expressions/unary.g4
* 
* Role:
* Canonical parser-level grammar for unary expressions.
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Implementation baseline:
* Rust 1.97 / Rust 1.97.1
* 
* Safety:
* This grammar contains no embedded Rust code and therefore introduces
* no unsafe implementation.
* 
* Rust components consuming the resulting parser/AST MUST remain compatible
* with Rust 1.97 / 1.97.1 and MUST use:
* 
* #![forbid(unsafe_code)]
* 
* ============================================================================
* ARCHITECTURAL OWNERSHIP
* ============================================================================
* 
* OWNS:
* 
* - unary-expression syntax;
* - prefix unary operator syntax;
* - the association of a prefix operator with its operand;
* - the syntactic distinction between prefix and postfix operators;
* - unary-expression precedence relative to the expression layer that
*   invokes this grammar;
* - arbitrary unary-expression nesting;
* - source-level unary operator structure.
* 
* DOES NOT OWN:
* 
* - lexical token definitions;
* - identifier spelling;
* - keyword spelling;
* - literal spelling;
* - postfix-expression syntax;
* - binary operator precedence;
* - operator overload resolution;
* - type checking;
* - constant evaluation;
* - ownership/borrowing semantics;
* - pointer/reference safety semantics;
* - quantum semantics;
* - quantum allocation;
* - physical qubit selection;
* - QEC;
* - ZQN;
* - hardware discovery;
* - hardware topology;
* - routing;
* - scheduling;
* - optimization;
* - resource allocation;
* - runtime execution;
* - target selection;
* - backend selection;
* - machine-specific limits.
* 
* ============================================================================
* DEPENDENCY BOUNDARY
* ============================================================================
* 
* This file is a parser grammar and consumes canonical tokens from:
* 
* ZamaniLexer
* 
* The canonical lexer owns operator spellings.
* 
* The expression composition layer owns:
* 
* postfixExpression
* 
* This grammar therefore references "postfixExpression" but does not redefine
* it.
* 
* The canonical expression grammar imports/assembles this component.
* 
* Conceptual dependency:
* 
* ZamaniLexer
*      |
*      v
* Unary parser component
*      |
*      v
* Expressions
*      |
*      v
* Frontend AST
*      |
*      v
* semantic analysis
*      |
*      +--> type checking
*      +--> effect checking
*      +--> resource checking
*      +--> capability checking
*      |
*      v
* canonical semantic representation
*      |
*      +--> classical IR
*      +--> quantum::ir
*      +--> control/data IR
*      +--> resource metadata
*      |
*      v
* optimization
*      |
*      v
* routing / scheduling / lowering
*      |
*      v
* target realization
* 
* There is deliberately NO dependency:
* 
* unary.g4 -> IR
* unary.g4 -> runtime
* unary.g4 -> hardware
* unary.g4 -> QEC
* unary.g4 -> ZQN
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Unary syntax describes operations on source-level values.
* 
* It does not describe the physical machine executing those operations.
* 
* Therefore this grammar MUST NOT encode:
* 
* MAX_OPERANDS
* MAX_NESTING
* MAX_POINTER_DEPTH
* MAX_REFERENCE_DEPTH
* MAX_REGISTER_COUNT
* MAX_QUBITS
* MAX_CORES
* MAX_THREADS
* MAX_DEVICES
* MAX_MEMORY
* MAX_ACCELERATORS
* MAX_NODES
* 
* A unary expression may be nested arbitrarily from the language grammar's
* perspective:
* 
* !x
* !!x
* !!!x
* &&&&value
* 
* subject only to the practical resource limits of the compiler/parser and
* execution environment.
* 
* Those implementation limits MUST NOT become language-level semantic limits.
* 
* ============================================================================
* SEMANTIC NEUTRALITY
* ============================================================================
* 
* This grammar recognizes syntax such as:
* 
* -value
* +value
* !value
* ~value
* &value
* *value
* ++value
* --value
* 
* It does NOT decide what those operations mean for a particular type.
* 
* For example:
* 
* -scalar
* -vector
* -matrix
* -tensor
* -symbolic
* -hardware_value
* 
* may all be syntactically valid.
* 
* Whether the operation is semantically valid is determined downstream by
* type and semantic analysis.
* 
* ============================================================================
* QUANTUM CONTRACT
* ============================================================================
* 
* Unary operators may syntactically occur in quantum programs when their
* operands are expressions for which the semantic system defines the
* operation.
* 
* For example, an expression layer may permit:
* 
* -theta
* !condition
* ~classical_mask
* *reference
* 
* without this grammar deciding whether an operand represents:
* 
* - a classical value;
* - a symbolic parameter;
* - a quantum-related value;
* - a resource;
* - a hardware value;
* - a future domain value.
* 
* This grammar MUST NOT interpret:
* 
* - physical qubits;
* - logical qubits;
* - gate implementations;
* - QPU topology;
* - calibration;
* - noise;
* - QEC;
* - ZQN;
* - physical allocation.
* 
* Quantum semantics are lowered downstream into the canonical "quantum::ir"
* boundary.
* 
* ============================================================================
* HARDWARE / HDL CONTRACT
* ============================================================================
* 
* Unary syntax may be reused by classical, HDL, accelerator, embedded,
* hardware/software co-design, and other domains.
* 
* For example:
* 
* ~signal
* !enable
* -offset
* *reference
* &signal
* 
* are syntax-level possibilities.
* 
* This grammar does NOT encode:
* 
* bus width
* register width
* address width
* FPGA family
* ASIC family
* CPU architecture
* GPU architecture
* clock frequency
* physical address
* device count
* 
* Such properties belong to semantic, target, resource, capability, or
* hardware-description layers.
* 
* ============================================================================
* OPERATOR OWNERSHIP
* ============================================================================
* 
* The parser consumes the canonical operator tokens.
* 
* The currently established lexical operator vocabulary includes:
* 
* PLUS
* MINUS
* INCREMENT
* DECREMENT
* AMPERSAND
* TILDE
* STAR
* NOT
* 
* The lexer is responsible for converting source spellings into those tokens.
* 
* This grammar does NOT create lexer aliases such as:
* 
* LOGICAL_NOT
* BIT_NOT
* ADDRESS_OF
* DEREFERENCE
* 
* unless those names are explicitly established by the canonical lexer.
* 
* This prevents parser components from inventing a second token vocabulary.
* 
* ============================================================================
* IMPORTANT CANONICAL-LEXER RECONCILIATION
* ============================================================================
* 
* The current repository lexer contains both:
* 
* NOT : 'not'
* 
* and an operator spelling:
* 
* ! 
* 
* under the same conceptual token name.
* 
* A lexer cannot contain two independently defined rules with the same token
* name.
* 
* Therefore the canonical lexer assembly MUST resolve this lexical ownership
* before this grammar is assembled into the production parser.
* 
* Recommended stable contract:
* 
* NOT
*     : '!'
*     ;
* 
* and a separately named keyword token for the word:
* 
* not
* 
* if the word-form operator is retained by the language.
* 
* Alternatively, the language may deliberately make "not" and "!" the same
* lexical token through a single authoritative lexer rule/strategy.
* 
* The choice belongs to the lexical/language-version authority, NOT to this
* parser component.
* 
* Once resolved, this grammar consumes the canonical resulting token.
* 
* ============================================================================
* PRECEDENCE CONTRACT
* ============================================================================
* 
* Prefix unary expressions bind more tightly than ordinary multiplicative,
* additive, comparison, and logical expressions.
* 
* The canonical expression grammar therefore places:
* 
* unaryExpression
* 
* beneath exponentiation/postfix composition as specified by the expression
* precedence model.
* 
* This file itself does not define binary operator precedence.
* 
* The operand of a prefix unary operator is another unary expression:
* 
* unaryOperator unaryExpression
* 
* This makes prefix unary operators naturally right-associative.
* 
* Therefore:
* 
* !!!x
* 
* is structurally:
* 
* !( !( !x ) )
* 
* rather than a finite special case.
* 
* ============================================================================
* PREFIX / POSTFIX DISTINCTION
* ============================================================================
* 
* Prefix:
* 
* ++x
* --x
* +x
* -x
* !x
* ~x
* &x
* *x
* 
* Postfix:
* 
* x++
* x--
* 
* Postfix syntax belongs to "postfixExpression" and MUST NOT be redefined
* here.
* 
* This separation prevents ambiguous ownership between:
* 
* unary.g4
* 
* and:
* 
* expressions.g4
* 
* ============================================================================
* SEMANTIC OPERATION CATEGORIES
* ============================================================================
* 
* The parser preserves operator spelling/token identity.
* 
* Downstream semantic analysis may classify operators as:
* 
* arithmetic
* logical
* bitwise
* reference/address
* dereference
* mutation
* domain-specific overload
* future extensible operation
* 
* No semantic category is hard-coded into the parser beyond syntactic
* operator membership.
* 
* ============================================================================
* TYPE SYSTEM INTEGRATION
* ============================================================================
* 
* Unary operators do not directly depend on a particular type grammar.
* 
* Semantic analysis determines whether:
* 
* +T
* -T
* !T
* ~T
* &T
* *T
* ++T
* --T
* 
* is legal for a given type T.
* 
* This allows the same syntax to support:
* 
* primitive values
* user-defined values
* vectors
* matrices
* tensors
* symbolic expressions
* resource values
* hardware values
* domain-specific values
* future types
* 
* without requiring a new unary grammar rule for every type.
* 
* ============================================================================
* EFFECT / OWNERSHIP INTEGRATION
* ============================================================================
* 
* Prefix mutation operators:
* 
* ++
* --
* 
* may require assignability, mutability, ownership, borrowing, effect, or
* concurrency checks.
* 
* Address/reference operations:
* 
* &
* *
* 
* may require ownership, borrowing, reference, lifetime, pointer, or memory
* semantics.
* 
* Those checks MUST occur downstream.
* 
* The grammar only establishes the syntactic structure.
* 
* ============================================================================
* CONSTANT / COMPILE-TIME INTEGRATION
* ============================================================================
* 
* Unary expressions may appear in compile-time expressions.
* 
* The parser does not evaluate them.
* 
* For example:
* 
* -1
* ~mask
* !condition
* 
* are represented structurally.
* 
* Constant evaluation, overflow handling, arbitrary-precision behavior,
* symbolic evaluation, and target-specific lowering belong downstream.
* 
* This prevents the grammar from imposing a machine integer width.
* 
* ============================================================================
* ERROR-RECOVERY CONTRACT
* ============================================================================
* 
* This grammar deliberately does not contain catch-all tokens or semantic
* predicates for operator validity.
* 
* Invalid examples such as:
* 
* +
* -
* !
* ~
* &
* *
* 
* without a valid operand remain parser errors when no valid operand can
* follow.
* 
* Invalid semantic combinations such as:
* 
* ++immutable_value
* *non_reference_value
* ~floating_value
* 
* when prohibited by the type system are NOT grammar errors.
* 
* They are semantic diagnostics.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* For identical:
* 
* source
* language version
* canonical lexer
* 
* the parser must produce the same unary-expression structure.
* 
* Parsing MUST NOT depend on:
* 
* CPU count
* memory size
* GPU availability
* QPU availability
* hardware topology
* runtime state
* calibration
* network state
* scheduler state
* backend choice
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar creates no AST implementation code.
* 
* The frontend AST should preserve at least:
* 
* operator token/kind
* operand expression
* source span
* 
* and, where required by the frontend architecture:
* 
* language version
* source identity
* diagnostic/provenance information
* 
* The AST must not be forced to know the target hardware merely because a
* unary expression eventually executes on hardware.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This file has NO direct dependency on:
* 
* classical IR
* quantum::ir
* hardware IR
* scheduling IR
* routing IR
* runtime representation
* 
* Semantic lowering decides which canonical representation is appropriate.
* 
* For a quantum operand, downstream semantic lowering may eventually produce
* canonical "quantum::ir" operations.
* 
* The parser MUST NOT create those operations.
* 
* ============================================================================
* COMPILER CONTRACT
* ============================================================================
* 
* Compiler stages consuming this grammar must be able to:
* 
* parse
* build AST
* resolve names
* resolve operators
* check types
* check effects
* check ownership/reference rules
* check capabilities/resources
* lower to canonical semantic representations
* 
* without requiring changes to this grammar merely because a new backend is
* introduced.
* 
* ============================================================================
* RUNTIME CONTRACT
* ============================================================================
* 
* There is no runtime dependency.
* 
* Runtime implementation determines how a semantically valid unary operation
* is realized on a selected target.
* 
* ============================================================================
* TOOLING CONTRACT
* ============================================================================
* 
* IDEs, formatters, syntax highlighters, linters, language servers, and
* refactoring tools should use the parser's operator/token structure rather
* than reimplementing unary syntax independently.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* This grammar contains no bounded repetition for unary nesting.
* 
* The recursive rule:
* 
* unaryOperator unaryExpression
* 
* permits arbitrary nesting supported by parser implementation resources.
* 
* No fixed limit is encoded for:
* 
* nesting depth
* expression size
* operand representation
* source length
* 
* Practical parser/runtime resource exhaustion remains an implementation
* concern, not a language semantic restriction.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing unary syntax must be preserved where it is part of the canonical
* Zamani language surface:
* 
* +
* -
* !
* ~
* &
* *
* ++
* --
* 
* Changes to operator spelling require language-versioning and migration
* policy.
* 
* This component must not silently reinterpret an existing operator.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Required positive tests include:
* 
* +x
* -x
* !x
* ~x
* &x
* *x
* ++x
* --x
* 
* Nested:
* 
* !!x
* ~~x
* --++x
* -~x
* !~x
* **x
* &&x
* 
* Composition:
* 
* -x + y
* !(x == y)
* ~mask & value
* *ptr + offset
* ++x * y
* 
* Domain-neutral examples:
* 
* -vector
* ~tensor
* !condition
* -symbolic_value
* ~hardware_signal
* 
* Required negative tests include:
* 
* +
* -
* !
* ~
* &
* *
* ++
* --
* 
* when no operand exists.
* 
* Also test malformed sequences where the surrounding expression grammar
* cannot supply a valid operand.
* 
* Semantic-invalid cases such as mutation of immutable values belong in
* semantic-analysis tests rather than parser-only tests.
* 
* ============================================================================
* BOUNDARY TEST CONTRACT
* ============================================================================
* 
* Test:
* 
* x
* -x
* --x
* 
* and deeply nested unary expressions generated programmatically.
* 
* The test suite MUST NOT define the language's maximum nesting depth.
* 
* It may define an explicit test-resource budget to prevent CI exhaustion,
* but that budget is a test harness constraint and MUST NOT be represented
* as a grammar limit.
* 
* ============================================================================
* CROSS-DOMAIN TEST CONTRACT
* ============================================================================
* 
* Verify that unary expressions can occur in:
* 
* classical expressions
* numerical expressions
* symbolic expressions
* quantum parameter expressions
* quantum-classical conditions
* HDL expressions
* hardware expressions
* accelerator expressions
* distributed expressions
* AI/data expressions
* resource expressions
* 
* without introducing domain-specific unary syntax unnecessarily.
* 
* ============================================================================
* ROUND-TRIP CONTRACT
* ============================================================================
* 
* Where Zamani provides a source printer/formatter:
* 
* source
*   ->
* lexer
*   ->
* parser
*   ->
* AST
*   ->
* printer
*   ->
* parser
* 
* must preserve unary operator structure and intended semantics.
* 
* Parentheses must be retained or regenerated wherever necessary to preserve
* the expression tree.
* 
* ============================================================================
* NO-HARDCODING AUDIT
* ============================================================================
* 
* This file MUST remain free of:
* 
* MAX_UNARY_DEPTH
* MAX_OPERANDS
* MAX_EXPRESSION_SIZE
* MAX_POINTER_DEPTH
* MAX_REFERENCE_DEPTH
* MAX_QUBITS
* MAX_CORES
* MAX_THREADS
* MAX_DEVICES
* MAX_MEMORY
* MAX_ACCELERATORS
* DEVICE_ID
* HARDWARE_ID
* QPU_ID
* 
* Any such value would be an accidental scalability restriction.
* 
* ============================================================================
* ANTLR ASSEMBLY CONTRACT
* ============================================================================
* 
* This is intentionally a parser grammar.
* 
* It consumes:
* 
* ZamaniLexer
* 
* and is intended to be imported by the canonical expression grammar.
* 
* The importing expression grammar must provide:
* 
* postfixExpression
* 
* or import the parser component that owns it.
* 
* This file MUST NOT redefine:
* 
* postfixExpression
* primaryExpression
* expression
* binary expressions
* literals
* identifiers
* 
* ============================================================================
* PUBLIC RULES
* ============================================================================
* 
* unaryExpression
* unaryOperator
* 
* "unaryExpression" is the public integration boundary.
* 
* "unaryOperator" is a supporting parser rule and should remain available for
* diagnostics, parser tooling, and future grammar composition.
* 
* ============================================================================
  */

parser grammar Unary;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* PUBLIC UNARY EXPRESSION
* ============================================================================
* 
* Prefix unary expressions are right-associative.
* 
* Base case:
* 
* postfixExpression
* 
* Recursive case:
* 
* unaryOperator unaryExpression
* 
* This permits arbitrary nesting without a hard-coded depth.
* 
* Examples:
* 
* -x
* !x
* ~~x
* !!!x
* -~value
* ++--x
* 
* The operand eventually bottoms out in the expression layer's
* "postfixExpression".
* 
* ============================================================================
  */

unaryExpression
: postfixExpression
| unaryOperator unaryExpression
;

/*

* ============================================================================
* PREFIX UNARY OPERATORS
* ============================================================================
* 
* The token names here are the canonical lexical contract currently exposed
* by the Zamani operator lexer.
* 
* Their semantic interpretation is downstream.
* 
* PLUS
* unary numeric/sign operation where semantically supported.
* 
* MINUS
* unary negation/sign operation where semantically supported.
* 
* NOT
* logical/boolean negation where semantically supported.
* 
* TILDE
* bitwise/complement operation where semantically supported.
* 
* AMPERSAND
* address/reference operation where semantically supported.
* 
* STAR
* dereference/indirection operation where semantically supported.
* 
* INCREMENT
* prefix increment/mutation where semantically supported.
* 
* DECREMENT
* prefix decrement/mutation where semantically supported.
* 
* No type-specific meaning is encoded here.
* 
* ============================================================================
  */

unaryOperator
: PLUS
| MINUS
| NOT
| TILDE
| AMPERSAND
| STAR
| INCREMENT
| DECREMENT
;
::

Why this is the correct boundary

The repository's current "expressions.g4" explicitly says that expression grammar owns unary syntax but must not own lexical definitions, type checking, hardware discovery, QEC, ZQN, routing, scheduling, optimization, or "quantum::ir".

The existing lexer already separates operator ownership from precedence and semantic interpretation, and explicitly places "+", "-", "*", "&", "~", "++", and "--" in the lexical operator layer.

The identifier layer likewise confirms that identifiers are lexical objects whose meaning is resolved later, rather than domain-specific hardware/quantum identities being hard-coded into the lexer.

Required integration — determined now, not later

There are two repository-level issues that should be fixed as part of the grammar assembly, rather than hiding them inside "unary.g4":

1. "expressions.g4" must stop redefining "unaryExpression" itself.
   Its existing "unaryExpression"/"unaryOperator" definitions become owned by this component. The canonical "Expressions" parser should import "Unary" and use its "unaryExpression" rule.

2. The lexer token contract must be reconciled before the parser is generated.
   The current lexer uses "NOT" in the logical-keyword section and also has the operator spelling "!" associated with "NOT"; this needs one authoritative token definition. The unary grammar deliberately does not invent "LOGICAL_NOT", "BIT_NOT", "ADDRESS_OF", or "DEREFERENCE" tokens merely to paper over that lexer inconsistency.

That means the dependency order is:

grammar/specification/
        ↓
grammar/lexer/tokens.g4
grammar/lexer/keywords.g4
grammar/lexer/operators.g4
        ↓
canonical ZamaniLexer
        ↓
grammar/expressions/identifiers.g4
grammar/expressions/literals.g4
        ↓
grammar/expressions/unary.g4     ← THIS FILE
        ↓
grammar/expressions/binary.g4
grammar/expressions/postfix/call/index/member
        ↓
grammar/expressions/expressions.g4
        ↓
frontend AST
        ↓
semantic analysis
        ↓
canonical IR
        ├── classical IR
        └── quantum::ir
        ↓
optimization / routing / scheduling / lowering
        ↓
hardware / runtime

Completion criterion for "unary.g4": once the canonical lexer token names are frozen and "Expressions" imports this grammar rather than duplicating its rules, this file itself does not need to be reopened merely because quantum, HDL, GPU, FPGA, AI, distributed, or future-target functionality is added. Those domains consume the same unary-expression syntax and attach their semantics downstream.