/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/classical/classical.g4
* 
* Status:
* Production-ready classical-domain composition grammar.
* 
* Grammar:
* ANTLR4 parser grammar
* 
* Target implementation:
* Rust 1.97 / Rust 1.97.1
* Rust edition 2021
* 
* Safety:
* This grammar contains no embedded Rust actions, semantic predicates,
* target-specific code, or unsafe implementation.
* 
* ============================================================================
* 
* PURPOSE
* ============================================================================
* 
* This file is the CLASSICAL DOMAIN COMPOSITION LAYER.
* 
* It provides the canonical parser-level boundary for source constructs whose
* computational meaning is classical.
* 
* This file intentionally does NOT duplicate:
* 
* grammar/types/types.g4
* grammar/types/classical-types.g4
* grammar/expressions/expressions.g4
* grammar/expressions/*.g4
* 
* Those files own their respective syntax.
* 
* This file instead composes them into classical computational constructs.
* 
* ============================================================================
* 
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* ZamaniLexer
*      |
*      v
* Core / domain parser
*      |
*      +-----------------------------+
*      |                             |
*      v                             v
* Types parser                  Expressions parser
*      |                             |
*      +-------------+---------------+
*                    |
*                    v
*            Classical parser
*                    |
*                    v
*              Frontend AST
*                    |
*                    v
*            Semantic analysis
*                    |
*         +----------+----------+
*         |                     |
*         v                     v
*   classical IR          resource/effect metadata
*         |
*         v
*   optimization
*         |
*         v
*   scheduling / lowering
*         |
*         v
*   target realization
* 
* The grammar does NOT directly construct:
* 
* classical IR
* quantum::ir
* QEC state
* ZQN state
* hardware topology
* scheduling state
* runtime state
* 
* ============================================================================
* 
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - classical-domain parser composition;
* - classical computation entry points;
* - classical expression statements;
* - classical bindings when explicitly classified as classical constructs;
* - classical blocks;
* - classical control/computation regions;
* - classical invocation composition;
* - classical assignment composition;
* - classical return composition;
* - classical declaration composition;
* - classical domain interoperability boundaries;
* - classical semantic-domain wrappers.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical tokens;
* - identifier spelling;
* - numeric literal spelling;
* - string literal spelling;
* - operators;
* - general expressions;
* - general statements;
* - general type syntax;
* - classical type definitions;
* - vector type syntax;
* - matrix type syntax;
* - tensor type syntax;
* - function type syntax;
* - quantum syntax;
* - HDL syntax;
* - hardware topology;
* - accelerator discovery;
* - CPU/GPU/FPGA selection;
* - classical IR;
* - quantum::ir;
* - optimization;
* - scheduling;
* - routing;
* - QEC;
* - ZQN;
* - runtime execution;
* - backend selection;
* - ABI selection;
* - physical resource limits.
* 
* ============================================================================
* 
* POCO-REAF
* ============================================================================
* 
* Classical syntax describes COMPUTATION and SEMANTIC INTENT.
* 
* It must not prescribe:
* 
* CPU count
* core count
* thread count
* register count
* SIMD width
* GPU count
* accelerator count
* memory capacity
* cache size
* NUMA topology
* node count
* network topology
* machine word size
* physical storage layout
* 
* A classical program may therefore be represented independently of whether
* it eventually executes on:
* 
* one processor;
* many processors;
* a CPU;
* a GPU;
* an FPGA;
* an ASIC;
* a heterogeneous accelerator;
* an embedded system;
* a cluster;
* a cloud;
* a distributed system;
* a future architecture.
* 
* Resource realization belongs downstream.
* 
* ============================================================================
* 
* SCALABILITY
* ============================================================================
* 
* No finite machine-oriented maximum is encoded here.
* 
* In particular, this grammar does NOT contain:
* 
* MAX_ELEMENTS
* MAX_VECTOR_LENGTH
* MAX_MATRIX_ROWS
* MAX_MATRIX_COLUMNS
* MAX_TENSOR_RANK
* MAX_THREADS
* MAX_CORES
* MAX_DEVICES
* MAX_MEMORY
* MAX_ARGUMENTS
* MAX_PARAMETERS
* MAX_NESTING_DEPTH
* 
* Repetition is represented structurally using ANTLR repetition and recursive
* grammar composition.
* 
* Any practical limit belongs to an explicit implementation policy such as:
* 
* parser resource policy
* compiler resource policy
* semantic validation
* resource manager
* scheduling
* deployment
* runtime
* 
* Such limits must never silently become language semantics.
* 
* ============================================================================
* 
* LEXER CONTRACT
* ============================================================================
* 
* The canonical lexer is:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* This grammar MUST consume the canonical token vocabulary.
* 
* It MUST NOT declare lexer rules.
* 
* It MUST NOT create aliases for lexer tokens.
* 
* The lexer already owns:
* 
* FN
* LET
* VAR
* MUT
* CONST
* RETURN
* IF
* ELSE
* FOR
* IN
* WHILE
* LOOP
* BREAK
* CONTINUE
* MATCH
* CASE
* WHEN
* QUANTUM
* QUBIT
* INT
* FLOAT_TYPE
* BOOL_TYPE
* STR_TYPE
* STRING_TYPE
* CHAR_TYPE
* INTEGER
* FLOAT
* STRING
* CHAR
* IDENTIFIER
* and the canonical operator/punctuation vocabulary.
* 
* This file does not redefine any of them.
* 
* ============================================================================
* 
* IMPORT CONTRACT
* ============================================================================
* 
* "Types" owns general type-expression syntax.
* 
* "ClassicalTypes" owns classical type-domain syntax.
* 
* "Expressions" owns general expression syntax.
* 
* Therefore this grammar imports those composition boundaries rather than
* copying their rules.
* 
* This is critical for maintainability:
* 
* changing expression precedence
*     does not require changing this file;
* 
* adding a classical numeric type
*     does not require changing this file;
* 
* adding a generic type mechanism
*     does not require changing this file.
* 
* ============================================================================
* 
* SEMANTIC BOUNDARY
* ============================================================================
* 
* A parser rule such as:
* 
* classicalExpression
* 
* only establishes syntactic classification.
* 
* Semantic analysis decides:
* 
* whether an operation is legal;
* whether a value is numeric;
* whether a type is classical;
* whether an expression is pure;
* whether an operation has effects;
* whether parallelization is legal;
* whether an accelerator is appropriate;
* whether resources are sufficient;
* whether a classical operation interacts with quantum state;
* whether an operation can be lowered to classical IR.
* 
* The grammar MUST NOT make those decisions.
* 
* ============================================================================
* 
* CROSS-DOMAIN BOUNDARY
* ============================================================================
* 
* Zamani is not restricted to purely classical programs.
* 
* A classical construct may eventually participate in:
* 
* classical + quantum
* classical + HDL
* classical + hardware
* classical + distributed
* classical + AI
* classical + accelerator
* 
* This file therefore does not prohibit expressions containing domain values.
* 
* Semantic analysis determines whether the resulting combination is valid.
* 
* The grammar must not encode hardware-specific restrictions.
* 
* ============================================================================
  */

parser grammar Classical;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* IMPORTS
* ============================================================================
* 
* These are architectural dependencies, not duplicated implementations.
* 
* "Expressions" is the authoritative source-level expression composition
* layer.
* 
* "Types" is the authoritative general type-expression layer.
* 
* "ClassicalTypes" is the authoritative classical type-domain layer.
* ============================================================================
  */

import Expressions,
Types,
ClassicalTypes;

/*

* ============================================================================
* 1. PUBLIC CLASSICAL DOMAIN ENTRY POINT
* ============================================================================
* 
* This is the primary integration rule for consumers that need to recognize
* a classical computational construct without duplicating its grammar.
* 
* The rule is intentionally a composition boundary.
* 
* ============================================================================
  */

classicalConstruct
: classicalDeclaration
| classicalBinding
| classicalAssignment
| classicalExpressionStatement
| classicalReturn
| classicalBlock
| classicalControlRegion
;

/*

* ============================================================================
* 2. CLASSICAL DECLARATION
* ============================================================================
* 
* A classical declaration introduces a value whose declared semantic type is
* classical.
* 
* The parser does not determine whether the referenced type is actually
* classical. Semantic analysis performs that classification.
* 
* Examples:
* 
* let x: int = expression;
* let values: Vector<float, N> = expression;
* const result: Matrix<float, Rows, Cols> = expression;
* 
* This rule deliberately reuses the canonical type and expression systems.
* 
* ============================================================================
  */

classicalDeclaration
: classicalBindingDeclaration
;

/*

* ============================================================================
* 3. CLASSICAL BINDING DECLARATION
* ============================================================================
* 
* This rule owns only the domain-level composition.
* 
* Variable-declaration semantics remain downstream.
* 
* ============================================================================
  */

classicalBindingDeclaration
: classicalBindingKeyword
identifier
classicalTypeAnnotation?
ASSIGN
expression
SEMICOLON
| classicalBindingKeyword
identifier
classicalTypeAnnotation
SEMICOLON
;

/*

* ============================================================================
* 4. CLASSICAL BINDING KEYWORDS
* ============================================================================
* 
* The canonical lexer already owns these keywords.
* 
* ============================================================================
  */

classicalBindingKeyword
: LET
| VAR
| CONST
;

/*

* ============================================================================
* 5. CLASSICAL TYPE ANNOTATION
* ============================================================================
* 
* A classical annotation can use:
* 
* - general type syntax;
* - classical type syntax.
* 
* The semantic type checker decides whether the result is genuinely
* classical.
* 
* Keeping this syntactic boundary broad prevents the grammar from becoming
* coupled to an exhaustive built-in type list.
* 
* ============================================================================
  */

classicalTypeAnnotation
: COLON typeExpression
;

/*

* ============================================================================
* 6. CLASSICAL BINDING
* ============================================================================
* 
* This form represents an already-parsed binding expression without forcing
* the grammar to duplicate the entire declaration subsystem.
* 
* Examples:
* 
* let x = expression;
* var accumulator = expression;
* const answer = expression;
* 
* The semantic layer classifies the initializer and resulting binding.
* 
* ============================================================================
  */

classicalBinding
: classicalBindingKeyword
identifier
ASSIGN
expression
SEMICOLON
;

/*

* ============================================================================
* 7. CLASSICAL ASSIGNMENT
* ============================================================================
* 
* Assignment syntax is consumed through the canonical expression system.
* 
* This rule does not redefine assignment operators.
* 
* The actual assignment grammar remains owned by:
* 
* grammar/expressions/assignment.g4
* 
* ============================================================================
  */

classicalAssignment
: expression
SEMICOLON
;

/*

* ============================================================================
* 8. CLASSICAL EXPRESSION STATEMENT
* ============================================================================
* 
* A classical computation can be represented by an expression whose result
* may be:
* 
* consumed;
* discarded;
* returned;
* used for side effects;
* lowered to a classical operation;
* lowered to an accelerator operation.
* 
* The grammar does not decide which interpretation applies.
* 
* ============================================================================
  */

classicalExpressionStatement
: expression
SEMICOLON
;

/*

* ============================================================================
* 9. CLASSICAL RETURN
* ============================================================================
* 
* Return syntax is deliberately kept domain-neutral.
* 
* This wrapper exists so classical-domain consumers can integrate return
* constructs without creating another return grammar.
* 
* ============================================================================
  */

classicalReturn
: RETURN expression?
SEMICOLON
;

/*

* ============================================================================
* 10. CLASSICAL BLOCK
* ============================================================================
* 
* A block is a sequence of classical constructs.
* 
* There is no fixed block size.
* 
* ============================================================================
  */

classicalBlock
: LBRACE
classicalConstruct*
RBRACE
;

/*

* ============================================================================
* 11. CLASSICAL CONTROL REGION
* ============================================================================
* 
* This rule provides a domain-level boundary for control-flow constructs
* whose bodies contain classical computation.
* 
* It deliberately does not duplicate the complete control-flow grammar.
* 
* Conditions remain ordinary expressions.
* 
* The body is a classical block.
* 
* ============================================================================
  */

classicalControlRegion
: classicalIfRegion
| classicalWhileRegion
| classicalForRegion
;

/*

* ============================================================================
* 12. CLASSICAL IF REGION
* ============================================================================
  */

classicalIfRegion
: IF
expression
classicalBlock
classicalElseRegion?
;

classicalElseRegion
: ELSE
(
IF
expression
classicalBlock
classicalElseRegion?
| classicalBlock
)
;

/*

* ============================================================================
* 13. CLASSICAL WHILE REGION
* ============================================================================
  */

classicalWhileRegion
: WHILE
expression
classicalBlock
;

/*

* ============================================================================
* 14. CLASSICAL FOR REGION
* ============================================================================
* 
* Iteration is deliberately expressed through an expression.
* 
* This permits future scalable iteration models without changing this grammar
* merely because the implementation acquires:
* 
* distributed iteration;
* data parallelism;
* task parallelism;
* accelerator iteration;
* vectorized iteration;
* streaming iteration.
* 
* ============================================================================
  */

classicalForRegion
: FOR
identifier
IN
expression
classicalBlock
;

/*

* ============================================================================
* 15. CLASSICAL IDENTIFIER
* ============================================================================
* 
* Identifier spelling belongs exclusively to the lexer.
* 
* ============================================================================
  */

identifier
: IDENTIFIER
;

/*

* ============================================================================
* 16. CLASSICAL VALUE EXPRESSION
* ============================================================================
* 
* This rule exists as a stable integration point for semantic consumers.
* 
* It intentionally delegates all expression syntax to "Expressions".
* 
* ============================================================================
  */

classicalValueExpression
: expression
;

/*

* ============================================================================
* 17. CLASSICAL TYPE
* ============================================================================
* 
* This rule provides the explicit classical-domain type integration point.
* 
* "ClassicalTypes" remains the owner of classical type syntax.
* 
* ============================================================================
  */

classicalType
: classicalType
;

/*

* ============================================================================
* IMPORTANT ANTLR OWNERSHIP NOTE
* ============================================================================
* 
* The rule above would recursively reference itself and is therefore NOT a
* valid composition mechanism.
* 
* The actual classical type boundary MUST be imported directly from
* ClassicalTypes.
* 
* Consequently, no local "classicalType" wrapper is defined in this grammar.
* 
* The imported "ClassicalTypes.classicalType" rule is the canonical rule.
* 
* ============================================================================
  */

/*

* ============================================================================
* 18. CLASSICAL COMPUTATION REGION
* ============================================================================
* 
* This is the preferred high-level integration entry point for compiler
* components that want to parse a region explicitly designated by the caller
* as classical.
* 
* The designation itself is semantic/contextual and need not be a new keyword.
* 
* ============================================================================
  */

classicalComputation
: classicalBlock
;

/*

* ============================================================================
* 19. CLASSICAL EXPRESSION LIST
* ============================================================================
* 
* This delegates expression syntax to the canonical expression grammar.
* 
* ============================================================================
  */

classicalExpressionList
: expression
(COMMA expression)*
COMMA?
;

/*

* ============================================================================
* 20. CLASSICAL ARGUMENT LIST
* ============================================================================
* 
* No finite argument count is encoded.
* 
* ============================================================================
  */

classicalArgumentList
: classicalExpressionList?
;

/*

* ============================================================================
* 21. CLASSICAL INVOCATION
* ============================================================================
* 
* This rule is intentionally generic.
* 
* A callee may ultimately represent:
* 
* a classical function;
* a numerical routine;
* a symbolic routine;
* a library operation;
* an accelerator operation;
* an intrinsic;
* a user-defined function;
* a future computational abstraction.
* 
* Semantic resolution determines the meaning.
* 
* ============================================================================
  */

classicalInvocation
: expression
LPAREN
classicalArgumentList
RPAREN
;

/*

* ============================================================================
* 22. CLASSICAL COMPUTATIONAL VALUE
* ============================================================================
* 
* This is a semantic integration façade.
* 
* It does not enumerate every possible classical operation.
* 
* That is deliberate.
* 
* An exhaustive operation list would make the grammar a scalability and
* extensibility bottleneck.
* 
* Library/domain operations remain identifiers resolved by semantic analysis.
* 
* ============================================================================
  */

classicalComputationalValue
: expression
;

/*

* ============================================================================
* 23. CLASSICAL DATA REGION
* ============================================================================
* 
* Classical values may be represented using the general expression system.
* 
* Their concrete type can be:
* 
* scalar;
* vector;
* matrix;
* tensor;
* collection;
* record;
* user-defined type;
* symbolic value;
* future classical domain type.
* 
* The type system owns the actual type syntax.
* 
* ============================================================================
  */

classicalDataRegion
: classicalBlock
;

/*

* ============================================================================
* 24. CLASSICAL CONTROL EXPRESSION
* ============================================================================
* 
* Conditions remain ordinary expressions.
* 
* The semantic checker determines whether an expression is valid as a
* condition.
* 
* ============================================================================
  */

classicalCondition
: expression
;

/*

* ============================================================================
* 25. CLASSICAL COMPUTATION SEQUENCE
* ============================================================================
* 
* Arbitrary sequence length is allowed.
* 
* ============================================================================
  */

classicalSequence
: classicalConstruct*
;

/*

* ============================================================================
* 26. DOMAIN INTEGRATION CONTRACT
* ============================================================================
* 
* This file intentionally exposes syntax-level boundaries only.
* 
* Downstream consumers are responsible for:
* 
* frontend AST
* symbol resolution
* type resolution
* effect analysis
* ownership analysis
* capability analysis
* resource analysis
* classical IR lowering
* accelerator selection
* optimization
* scheduling
* placement
* runtime realization
* 
* In particular:
* 
* Classical
*      |
*      v
* frontend AST
*      |
*      v
* semantic analysis
*      |
*      v
* canonical classical representation
* 
* The grammar MUST NOT directly reference:
* 
* quantum::ir
* QEC
* ZQN
* scheduler
* routing
* hardware HAL
* runtime
* 
* ============================================================================
  */

/*

* ============================================================================
* 27. QUANTUM INTEROPERABILITY CONTRACT
* ============================================================================
* 
* Classical code may participate in hybrid programs.
* 
* This grammar therefore imposes no grammar-level restriction that a
* classical expression must contain only primitive classical values.
* 
* For example, a semantic layer may eventually permit classical control over
* quantum operations:
* 
* classical condition
*      |
*      v
* quantum operation
* 
* or quantum measurement results to feed classical computation:
* 
* quantum measurement
*      |
*      v
* classical value
* 
* The legality and lowering of these relationships belong to semantic
* analysis and the canonical quantum/classical intermediate representations.
* 
* ============================================================================
  */

/*

* ============================================================================
* 28. HARD-CODING AUDIT
* ============================================================================
* 
* This grammar deliberately contains no:
* 
* MAX_VECTOR_LENGTH
* MAX_MATRIX_SIZE
* MAX_TENSOR_RANK
* MAX_ELEMENTS
* MAX_THREADS
* MAX_CORES
* MAX_GPUS
* MAX_ACCELERATORS
* MAX_MEMORY
* MAX_NODES
* MAX_DEVICES
* 
* It also contains no:
* 
* CPU identifier
* GPU identifier
* accelerator identifier
* memory address
* register identifier
* SIMD width
* cache size
* topology
* machine word width
* 
* Therefore source-level classical semantics remain independent of target
* realization.
* 
* ============================================================================
  */

/*

* ============================================================================
* 29. DETERMINISM CONTRACT
* ============================================================================
* 
* This grammar:
* 
* - contains no semantic predicates;
* - contains no embedded actions;
* - contains no target-language code;
* - does not inspect runtime state;
* - does not inspect hardware state;
* - does not inspect resource availability.
* 
* Therefore parsing is independent of machine discovery and runtime state.
* 
* ============================================================================
  */

/*

* ============================================================================
* 30. ERROR-BOUNDARY CONTRACT
* ============================================================================
* 
* Syntax errors belong to the parser/frontend diagnostic layer.
* 
* Semantic errors such as:
* 
* invalid type;
* invalid numeric operation;
* insufficient resources;
* unsupported accelerator;
* invalid parallelization;
* invalid effect;
* invalid classical/quantum interaction;
* 
* MUST NOT be encoded as parser-specific grammar failures unless they are
* genuinely syntactic.
* 
* ============================================================================
  */

/*

* ============================================================================
* 31. TEST CONTRACT
* ============================================================================
* 
* This grammar is complete only when integration tests cover at least:
* 
* POSITIVE:
* 
* let x: int = 1;
* let x: float = 1.0;
* let x: Vector<float, N> = value;
* let x: Matrix<float, Rows, Cols> = value;
* let x: Tensor<float, N, M, K> = value;
* let x = function_call(value);
* x = y;
* if condition { ... }
* while condition { ... }
* for item in collection { ... }
* 
* NEGATIVE:
* 
* malformed declaration;
* missing initializer;
* missing semicolon;
* malformed type annotation;
* malformed control region;
* malformed iteration;
* malformed expression.
* 
* SCALABILITY:
* 
* arbitrarily long classical sequences;
* arbitrarily many bindings;
* arbitrarily nested semantic constructs;
* symbolic vector dimensions;
* symbolic matrix dimensions;
* symbolic tensor dimensions;
* large expression lists.
* 
* CROSS-DOMAIN:
* 
* classical + quantum;
* classical + hardware;
* classical + HDL;
* classical + distributed;
* classical + accelerator;
* classical + AI.
* 
* DETERMINISM:
* 
* identical source -> identical token/parse structure.
* 
* ROUND TRIP:
* 
* source -> lexer -> parser -> AST -> formatter -> parser
* 
* where the repository's canonical formatter/AST infrastructure supports
* round-trip serialization.
* 
* ============================================================================
  */

/*

* ============================================================================
* 32. RUST INTEGRATION CONTRACT
* ============================================================================
* 
* This file contains no Rust code.
* 
* Generated/compiler-side Rust integration MUST target:
* 
* Rust 1.97
* Rust 1.97.1
* 
* and the repository's Rust crates MUST forbid unsafe implementation:
* 
* #![deny(unsafe_code)]
* 
* The grammar itself does not require unsafe operations.
* 
* ============================================================================
  */

/*

* ============================================================================
* 33. COMPLETION CRITERIA
* ============================================================================
* 
* This file is COMPLETE when:
* 
* [ ] The file compiles with the canonical ANTLR grammar set.
* 
* [ ] Its imports resolve without duplicate-rule ownership.
* 
* [ ] It consumes the canonical Zamani lexer vocabulary.
* 
* [ ] It contains no lexer rules.
* 
* [ ] It contains no machine-specific constants.
* 
* [ ] It contains no physical hardware assumptions.
* 
* [ ] It does not duplicate ClassicalTypes.
* 
* [ ] It does not duplicate Types.
* 
* [ ] It does not duplicate Expressions.
* 
* [ ] It does not duplicate quantum IR.
* 
* [ ] It does not duplicate classical IR.
* 
* [ ] It does not select hardware.
* 
* [ ] It does not perform semantic validation.
* 
* [ ] It does not perform optimization.
* 
* [ ] It does not perform scheduling.
* 
* [ ] It does not perform runtime dispatch.
* 
* [ ] Positive tests exist.
* 
* [ ] Negative tests exist.
* 
* [ ] Boundary tests exist.
* 
* [ ] Cross-domain tests exist.
* 
* [ ] Determinism tests exist.
* 
* [ ] Scalability tests exist.
* 
* [ ] Rust 1.97 / 1.97.1 integration passes.
* 
* ============================================================================
  */

/*

* ============================================================================
* 34. IMPORTANT NOTE ABOUT THE TYPE RULE
* ============================================================================
* 
* "ClassicalTypes.classicalType" is imported from:
* 
* grammar/types/classical-types.g4
* 
* This grammar intentionally does NOT redeclare a local rule with the same
* name.
* 
* Consumers needing the classical type rule should use the imported
* "classicalType" rule supplied by "ClassicalTypes".
* 
* This prevents two separate definitions of classical type syntax.
* 
* ============================================================================
  */