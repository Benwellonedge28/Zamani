/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/metaprogramming/type-level.g4
* 
* Grammar:
* TypeLevelMetaprogramming
* 
* Status:
* Production metaprogramming delegate
* 
* Compiler baseline:
* Rust 1.97+
* Rust 2021+
* safe Rust only
* no unsafe Rust
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This grammar defines the SOURCE SYNTAX boundary for type-level
* metaprogramming.
* 
* It does not define another type system.
* 
* It does not define another AST.
* 
* It does not define another IR.
* 
* It does not evaluate type-level programs.
* 
* It does not perform type checking, kind checking, normalization,
* substitution, specialization, reflection, resource discovery, capability
* resolution, target selection, quantum routing, scheduling, QEC, ZQN, HAL,
* or runtime execution.
* 
* Its responsibility is limited to representing source-level requests for
* computation ABOUT types and type-level values.
* 
* The canonical pipeline remains:
* 
* source
*   |
*   v
* lexer
*   |
*   v
* ANTLR parser
*   |
*   v
* domain-neutral frontend AST
*   |
*   v
* structural validation
*   |
*   v
* semantic analysis
*   |
*   +----------------------+----------------------+
*   |                      |                      |
*   v                      v                      v
* types              type-level semantics     metaprogramming
*   |                      |                      |
*   +----------------------+----------------------+
*                          |
*                          v
*                canonical semantic model
*                          |
*              +-----------+-----------+
*              |                       |
*              v                       v
*         classical IR           quantum::ir
*              |                       |
*              +-----------+-----------+
*                          |
*                          v
*                optimization/lowering
*                          |
*                routing/scheduling
*                          |
*                resilience / QEC / ZQN
*                          |
*                          v
*                         HAL
*                          |
*                          v
*                   target realization
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - type-level metaprogramming entry syntax;
* - type-level computation expressions;
* - type-level bindings;
* - type-level predicates;
* - type-level conditionals;
* - type-level value expressions;
* - type-level transformation requests;
* - type-level query requests;
* - type-level normalization requests;
* - type-level namespaced extension requests;
* - type-level result boundaries.
* 
* THIS FILE DOES NOT OWN:
* 
* - ordinary type expressions;
* - generic declarations;
* - generic type applications;
* - dependent type definitions;
* - associated types;
* - type classes;
* - ordinary expressions;
* - ordinary patterns;
* - ordinary statements;
* - reflection syntax;
* - source generation syntax;
* - macro syntax;
* - specialization syntax;
* - compile-time execution syntax;
* - resource requirements;
* - capabilities;
* - effects;
* - policies;
* - contracts;
* - quantum operations;
* - HDL syntax;
* - hardware syntax.
* 
* Those remain owned by their respective canonical grammar components.
* 
* ============================================================================
* SINGLE-OWNER RULE
* ============================================================================
* 
* The following canonical rules MUST NOT be redefined here:
* 
* typeExpression
* expression
* pattern
* identifier
* qualifiedName
* attribute
* block
* genericParameterList
* genericArgumentList
* 
* In particular, this file MUST NOT introduce another "typeExpression".
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The parser output is adapted into the existing frontend AST.
* 
* Type-level type results map to the existing type representation, including
* the repository's existing:
* 
* TypeExpr
* TypeValueExpr
* TypeExtension
* TypeValueExtension
* 
* This file MUST NOT introduce:
* 
* TypeLevelAst
* TypeMetaAst
* TypeLevelType
* TypeLevelIR
* UniversalTypeIR
* 
* Type-level syntax is syntax, not a new intermediate representation.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Downstream semantic analysis determines:
* 
* - name resolution;
* - type/kind resolution;
* - generic substitution;
* - type-level evaluation;
* - normalization;
* - equality;
* - compatibility;
* - constraint satisfaction;
* - termination;
* - determinism;
* - effect legality;
* - capability requirements;
* - resource requirements;
* - provenance;
* - policy compliance.
* 
* None of those decisions are made by this grammar.
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Type-level syntax describes portable program meaning.
* 
* It MUST NOT encode a fixed physical universe.
* 
* No grammar-level limit may be imposed on:
* 
* type parameters
* type arguments
* dimensions
* shapes
* qubits
* registers
* tensor rank
* memory
* devices
* nodes
* processors
* accelerators
* generated declarations
* type-level computation size
* 
* The grammar contains no:
* 
* MAX_TYPE_LEVEL_*
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_DEVICE_COUNT
* 
* Practical compiler limits are implementation/resource policies and MUST
* remain outside the language grammar.
* 
* ============================================================================
* LEXER CONTRACT
* ============================================================================
* 
* This is a parser grammar.
* 
* It declares no lexer rules.
* 
* All tokens must originate from the canonical Zamani lexer.
* 
* The repository uses the K_* keyword convention. This file therefore uses
* canonical K_* tokens only where they are already established by the lexical
* architecture.
* 
* New keywords MUST NOT be invented in this file.
* 
* If a future feature requires a new reserved word, that change belongs first
* in the lexical specification and lexer authority, followed by the parser
* integration.
* 
* ============================================================================
* EXTENSIBILITY PRINCIPLE
* ============================================================================
* 
* Type-level operations are intentionally OPEN.
* 
* A finite list such as:
* 
* normalize
* map
* filter
* project
* quantumShape
* tensorRank
* hardwareWidth
* 
* must not become the universal grammar.
* 
* New domains should normally use a qualified operation path:
* 
* namespace::operation(...)
* 
* Examples:
* 
* tensor::rank(T)
* quantum::shape(T)
* hardware::capability(T)
* resource::requirement(T)
* 
* Whether such an operation exists is determined semantically by the
* extension/dialect registry.
* 
* ============================================================================
* TYPE-LEVEL PHASE
* ============================================================================
* 
* Type-level computation is a compile-time semantic phase.
* 
* The source syntax does not imply runtime execution.
* 
* Runtime reflection, runtime metadata, runtime adaptation, and runtime
* execution remain separate mechanisms.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Parsing a type-level expression MUST never execute it.
* 
* Before evaluation, semantic infrastructure must enforce the applicable:
* 
* type rules
* kind rules
* effect rules
* capability rules
* resource rules
* policy rules
* provenance rules
* compile-time security rules
* 
* Type-level syntax provides no implicit access to:
* 
* filesystem
* network
* subprocesses
* credentials
* environment state
* hardware state
* wall-clock time
* uncontrolled randomness
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing is deterministic.
* 
* Type-level semantic evaluation is deterministic when the referenced
* operation is declared deterministic.
* 
* Operations requiring external state must be explicitly represented through
* the repository's effect/capability model.
* 
* ============================================================================
* INTEGRATION
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/antlr/ZamaniLexer.g4
* grammar/types/types.g4
* grammar/expressions/
* grammar/core/
* grammar/metaprogramming/specialization.g4
* grammar/metaprogramming/reflection.g4
* grammar/metaprogramming/generation.g4
* grammar/metaprogramming/compile-time*.g4
* grammar/macros/
* 
* EXPORTS:
* 
* typeLevelMetaprogramming
* typeLevelDeclaration
* typeLevelExpression
* typeLevelValue
* typeLevelPredicate
* typeLevelOperation
* typeLevelQuery
* typeLevelTransform
* 
* AST_OWNER:
* 
* src/frontend/ast/node/types/
* 
* SEMANTIC_OWNER:
* 
* canonical semantic/type-analysis subsystem
* 
* IR_OWNER:
* 
* canonical semantic IR
* quantum::ir where quantum semantics are produced
* 
* TEST_OWNER:
* 
* grammar/tests/metaprogramming/type-level/
* 
* SPEC_OWNER:
* 
* grammar/spec/type-system.md
* grammar/spec/metaprogramming.md
* grammar/specification/
* 
* ============================================================================
* IMPORTANT ANTLR COMPOSITION CONTRACT
* ============================================================================
* 
* This file is a DELEGATE grammar.
* 
* It must be imported by the canonical metaprogramming composition grammar.
* 
* The repository's composition root remains responsible for deciding where
* type-level constructs are legal in a complete Zamani source unit.
* 
* This file does not become the root parser.
* 
* ============================================================================
  */

parser grammar TypeLevelMetaprogramming;

options {
tokenVocab = ZamaniLexer;
}

/* ============================================================================

* 1. PUBLIC ENTRY
* ============================================================================
* 
* The metaprogramming orchestrator should delegate to this rule.
* 
* This rule deliberately does not include ordinary runtime statements.
  */
  typeLevelMetaprogramming
  : typeLevelDeclaration
  | typeLevelExpression
  ;

/* ============================================================================

* 2. DECLARATION
* ============================================================================
* 
* A type-level declaration binds a compile-time name to a type-level result.
* 
* The semantic layer determines the resulting kind.
* 
* The declaration does not introduce a second declaration/type system.
  */
  typeLevelDeclaration
  : K_LET
  IDENTIFIER
  typeLevelBindingType?
  ASSIGN
  typeLevelExpression
  SEMICOLON
  ;

/*

* Optional result annotation.
* 
* The type grammar remains authoritative.
  */
  typeLevelBindingType
  : COLON
  typeExpression
  ;

/* ============================================================================

* 3. TYPE-LEVEL EXPRESSION
* ============================================================================
* 
* The expression is intentionally factored to avoid the ambiguity present in
* the previous implementation.
* 
* The structure is:
* 
* conditional
*   -> predicate
*   -> expression
*   -> expression
* 
* operation
*   -> qualified operation name
*   -> arguments
* 
* primary
*   -> ordinary type
*   -> type-level value
*   -> name
*   -> parenthesized expression
* 
* There is no recursive "everything contains everything" construction.
  */
  typeLevelExpression
  : typeLevelConditionalExpression
  | typeLevelOperationExpression
  | typeLevelApplicationExpression
  | typeLevelNormalizeExpression
  | typeLevelPredicateExpression
  | typeLevelValueExpression
  | typeLevelTypeResult
  | typeLevelParenthesizedExpression
  ;

/* ============================================================================

* 4. CONDITIONAL
* ============================================================================
* 
* Conditional type-level computation.
* 
* The condition is a type-level predicate.
  */
  typeLevelConditionalExpression
  : K_IF
  typeLevelPredicate
  K_THEN
  typeLevelExpression
  K_ELSE
  typeLevelExpression
  ;

/* ============================================================================

* 5. GENERAL TYPE-LEVEL OPERATION
* ============================================================================
* 
* Open operation namespace.
* 
* Examples:
* 
* normalize(...)
* tensor::rank(...)
* quantum::shape(...)
* 
* No built-in operation catalogue is embedded here.
  */
  typeLevelOperationExpression
  : K_TRANSFORM
  typeLevelOperation
  typeLevelArgumentList?
  ;

/*

* Qualified operation name.
* 
* Examples:
* 
* normalize
* tensor::rank
* quantum::shape

/
typeLevelOperation
: IDENTIFIER
(
DOUBLE_COLON
IDENTIFIER
)
;

/* ============================================================================

* 6. APPLICATION
* ============================================================================
* 
* Explicit application of a previously resolved type-level callable.
* 
* The callable is named rather than represented by an unrestricted ordinary
* expression. This prevents the type-level grammar from becoming a duplicate
* general expression grammar.
  */
  typeLevelApplicationExpression
  : K_APPLY
  typeLevelReference
  typeLevelArgumentList
  ;

/* ============================================================================

* 7. NORMALIZATION
* ============================================================================
* 
* Normalization is requested here and performed later.
  */
  typeLevelNormalizeExpression
  : K_NORMALIZE
  LPAREN
  typeLevelSubject
  RPAREN
  ;

/* ============================================================================

* 8. PREDICATES
* ============================================================================
* 
* Predicates represent semantic propositions.
* 
* They are not proofs.
  */
  typeLevelPredicateExpression
  : K_IS
  typeLevelPredicate
  ;

typeLevelPredicate
: typeLevelPredicatePrimary
(
typeLevelPredicateOperator
typeLevelPredicatePrimary
)?
;

typeLevelPredicatePrimary
: typeLevelPredicateComparison
| typeLevelPredicateReference
| LPAREN
typeLevelPredicate
RPAREN
;

typeLevelPredicateComparison
: typeLevelSubject
typeLevelPredicateOperator
typeLevelSubject
;

typeLevelPredicateReference
: typeLevelQualifiedReference
;

typeLevelPredicateOperator
: EQUAL
| NOT_EQUAL
| LESS_THAN
| LESS_EQUAL
| GREATER_THAN
| GREATER_EQUAL
;

/* ============================================================================

* 9. TYPE-LEVEL SUBJECT
* ============================================================================
* 
* A subject is deliberately restricted to semantic type-level entities.
* 
* Ordinary runtime expressions are not accepted here. This prevents accidental
* mixing of runtime evaluation with compile-time type computation.
  */
  typeLevelSubject
  : typeExpression
  | typeLevelValue
  | typeLevelQualifiedReference
  | typeLevelParenthesizedExpression
  ;

/* ============================================================================

* 10. TYPE-LEVEL TYPE RESULT
* ============================================================================
* 
* An ordinary type expression can be used as the result of type-level
* computation.
* 
* The type itself remains owned by grammar/types/.
  */
  typeLevelTypeResult
  : typeExpression
  ;

/* ============================================================================

* 11. TYPE-LEVEL REFERENCE
* ============================================================================
* 
* This rule deliberately does NOT contain:
* 
* typeExpression
* 
* because "typeExpression" already accepts named types.
* 
* The previous:
* 
* typeExpression | identifier
* 
* created a structural overlap.
  */
  typeLevelReference
  : typeLevelQualifiedReference
  ;

typeLevelQualifiedReference
: IDENTIFIER
(
DOUBLE_COLON
IDENTIFIER
)*
;

/* ============================================================================

* 12. TYPE-LEVEL ARGUMENTS
* ============================================================================
* 
* No finite argument limit is encoded.
  /
  typeLevelArgumentList
  : LPAREN
  typeLevelArgument
  (
  COMMA
  typeLevelArgument
  )
  COMMA?
  RPAREN
  ;

typeLevelArgument
: typeExpression
| typeLevelValue
| typeLevelExpression
;

/* ============================================================================

* 13. TYPE-LEVEL VALUE
* ============================================================================
* 
* Type-level values are compile-time values used by the canonical type
* representation.
* 
* They are intentionally smaller than the runtime expression grammar.
* 
* Their semantic representation is the existing TypeValueExpr.
  */
  typeLevelValue
  : typeLevelValueBinary
  ;

typeLevelValueBinary
: typeLevelValueUnary
(
typeLevelValueBinaryOperator
typeLevelValueUnary
)*
;

typeLevelValueUnary
: typeLevelValueUnaryOperator?
typeLevelValuePrimary
;

typeLevelValueUnaryOperator
: PLUS
| MINUS
| BIT_NOT
;

typeLevelValuePrimary
: INTEGER_LITERAL
| typeLevelQualifiedReference
| typeLevelValueParenthesized
;

typeLevelValueParenthesized
: LPAREN
typeLevelValue
RPAREN
;

typeLevelValueBinaryOperator
: PLUS
| MINUS
| STAR
| SLASH
| MODULO
| LEFT_SHIFT
| RIGHT_SHIFT
| BIT_AND
| BIT_OR
| CARET
;

/* ============================================================================

* 14. TYPE-LEVEL VALUE CALL
* ============================================================================
* 
* A type-level value function is represented by a qualified reference.
* 
* It is not an arbitrary runtime function call.
  */
  typeLevelValueCall
  : typeLevelQualifiedReference
  LPAREN
  typeLevelArgumentList?
  RPAREN
  ;

/* ============================================================================

* 15. TYPE-LEVEL QUERY
* ============================================================================
* 
* Queries are open and namespace-qualified.
* 
* The semantic registry decides which query is valid.
* 
* Examples:
* 
* query kind(T)
* query tensor::rank(T)
* query quantum::shape(T)
* 
* The grammar does not enumerate query names.
  */
  typeLevelQueryExpression
  : K_QUERY
  typeLevelOperation
  LPAREN
  typeLevelSubject
  RPAREN
  ;

/* ============================================================================

* 16. TYPE-LEVEL RESULT BOUNDARY
* ============================================================================
* 
* This rule provides a stable integration point for semantic adapters.
  */
  typeLevelResult
  : typeExpression
  | typeLevelValue
  | typeLevelExpression
  ;

/* ============================================================================

* 17. TYPE-LEVEL EXTENSION
* ============================================================================
* 
* Domain-specific type-level functionality must remain namespaced.
* 
* This rule does not distinguish domains.
* 
* Domain registration and validation happen semantically.
  */
  typeLevelExtensionExpression
  : typeLevelQualifiedReference
  LPAREN
  typeLevelArgumentList?
  RPAREN
  ;

/* ============================================================================

* 18. PARENTHESIZED EXPRESSION
* ============================================================================
  */
  typeLevelParenthesizedExpression
  : LPAREN
  typeLevelExpression
  RPAREN
  ;

/* ============================================================================

* 19. EXPLICIT RESULT ASSERTION
* ============================================================================
* 
* An expected result type may be attached to a type-level expression.
* 
* The parser preserves the relationship; semantic analysis checks it.
  */
  typeLevelResultAnnotation
  : COLON
  typeExpression
  ;

typeLevelAnnotatedResult
: typeLevelExpression
typeLevelResultAnnotation
;

/* ============================================================================

* 20. TYPE-LEVEL BLOCK
* ============================================================================
* 
* Blocks are deliberately not treated as an alternative function language.
* 
* A block is a sequence of type-level declarations/expressions.
* 
* The canonical metaprogramming composition layer decides where such a block
* is legal.
  /
  typeLevelBlock
  : LBRACE
  typeLevelBlockElement
  RBRACE
  ;

typeLevelBlockElement
: typeLevelDeclaration
| typeLevelExpression
SEMICOLON
;

/* ============================================================================

* 21. TYPE-LEVEL FUNCTION-LIKE TRANSFORMATION
* ============================================================================
* 
* This is a semantic transformation expression, not a second function
* declaration system.
* 
* Parameter kinds are represented by the existing type system.
  */
  typeLevelLambda
  : K_FN
  LPAREN
  typeLevelParameterList?
  RPAREN
  ARROW
  typeLevelExpression
  ;

typeLevelParameterList
: typeLevelParameter
(
COMMA
typeLevelParameter
)*
COMMA?
;

typeLevelParameter
: IDENTIFIER
typeLevelParameterType?
;

typeLevelParameterType
: COLON
typeExpression
;

/* ============================================================================

* 22. TYPE-LEVEL ASSERTION
* ============================================================================
* 
* Assertions belong semantically to validation/contracts.
* 
* This grammar only provides the explicit type-level assertion boundary.
  */
  typeLevelAssertion
  : K_ASSERT
  LPAREN
  typeLevelPredicate
  RPAREN
  ;

/* ============================================================================

* 23. TYPE-LEVEL QUERY RESULT
* ============================================================================
  */
  typeLevelQueryResult
  : typeLevelQueryExpression
  ;

/* ============================================================================

* 24. TYPE-LEVEL OPERATION RESULT
* ============================================================================
  */
  typeLevelOperationResult
  : typeLevelOperationExpression
  ;

/* ============================================================================

* 25. TYPE-LEVEL VALUE RESULT
* ============================================================================
  */
  typeLevelValueResult
  : typeLevelValue
  ;

/* ============================================================================

* 26. TYPE-LEVEL EXTENSION RESULT
* ============================================================================
  */
  typeLevelExtensionResult
  : typeLevelExtensionExpression
  ;

/* ============================================================================

* 27. SEMANTIC INTEGRATION: TYPES
* ============================================================================
* 
* "typeExpression" is imported/visible through the canonical type grammar.
* 
* This file must never duplicate:
* 
* generic type syntax
* tuple syntax
* array syntax
* dependent syntax
* associated type syntax
* linear syntax
* affine syntax
* quantum type syntax
* hardware type syntax
* resource type syntax.
* 
* Type-level operations consume the canonical representation.
* 
* ============================================================================
* 28. SEMANTIC INTEGRATION: TYPE VALUES
* ============================================================================
* 
* Type-level value syntax maps to the existing TypeValueExpr representation.
* 
* The semantic layer determines:
* 
* value kind
* admissible operators
* constant evaluability
* normalization
* overflow/error behavior
* symbolic evaluation
* dependency tracking.
* 
* The grammar never converts a source number into a host-sized integer.
* 
* ============================================================================
* 29. SEMANTIC INTEGRATION: DEPENDENT TYPES
* ============================================================================
* 
* Dependent types remain owned by:
* 
* grammar/types/dependent.g4
* 
* Type-level values can provide dependent arguments.
* 
* The flow is:
* 
* type-level value
*     |
*     v
* TypeValueExpr
*     |
*     v
* dependent type semantics
* 
* This file does not redefine dependent type syntax.
* 
* ============================================================================
* 30. SEMANTIC INTEGRATION: GENERICS
* ============================================================================
* 
* Generic declarations and ordinary generic application remain owned by the
* canonical type/function grammar.
* 
* Type-level computation may calculate generic arguments or constraints, but
* it does not replace generic syntax.
* 
* ============================================================================
* 31. SEMANTIC INTEGRATION: REFLECTION
* ============================================================================
* 
* Reflection syntax belongs to:
* 
* grammar/metaprogramming/reflection.g4
* 
* Reflection results may become type-level subjects where the semantic model
* permits them.
* 
* No reflection syntax is duplicated here.
* 
* ============================================================================
* 32. SEMANTIC INTEGRATION: GENERATION
* ============================================================================
* 
* Source generation belongs to:
* 
* grammar/metaprogramming/generation.g4
* 
* Type-level computation may provide generation inputs.
* 
* Generated source must return through normal parsing and semantic validation.
* 
* No direct AST mutation is permitted by this grammar.
* 
* ============================================================================
* 33. SEMANTIC INTEGRATION: SPECIALIZATION
* ============================================================================
* 
* Specialization belongs to:
* 
* grammar/metaprogramming/specialization.g4
* 
* This file does not define "K_SPECIALIZE".
* 
* A specialization request may consume a type-level value or type-level
* result through the canonical specialization boundary.
* 
* ============================================================================
* 34. SEMANTIC INTEGRATION: COMPILE-TIME EXECUTION
* ============================================================================
* 
* General compile-time execution belongs to the compile-time grammar.
* 
* Type-level computation is one semantic consumer of compile-time execution,
* not a replacement for it.
* 
* ============================================================================
* 35. SEMANTIC INTEGRATION: MACROS
* ============================================================================
* 
* Macros may generate type-level syntax.
* 
* Macro expansion must preserve:
* 
* source span
* hygiene
* provenance
* deterministic ordering
* 
* Expanded source is validated through the ordinary semantic pipeline.
* 
* ============================================================================
* 36. EFFECT INTEGRATION
* ============================================================================
* 
* Type-level computation is not implicitly pure merely because it happens
* during compilation.
* 
* The semantic layer determines the effect set of each registered operation.
* 
* Operations requiring:
* 
* IO
* network
* randomness
* native access
* foreign access
* external state
* 
* must be explicitly represented through the effect/capability architecture.
* 
* ============================================================================
* 37. CAPABILITY INTEGRATION
* ============================================================================
* 
* Type-level queries may refer to capabilities through namespaced operations.
* 
* This grammar does not resolve capabilities.
* 
* Capability resolution remains owned by:
* 
* grammar/resources/
* grammar/hardware/
* semantic capability analysis.
* 
* ============================================================================
* 38. RESOURCE INTEGRATION
* ============================================================================
* 
* Type-level values may represent resource parameters.
* 
* Example semantic concepts include:
* 
* Resource[N]
* Register[N]
* Tensor[Shape]
* 
* The grammar does not allocate or discover resources.
* 
* Physical feasibility remains downstream.
* 
* ============================================================================
* 39. POLICY INTEGRATION
* ============================================================================
* 
* Type-level evaluation may be restricted by compile-time policy.
* 
* Policies remain external to this grammar.
* 
* This prevents metaprogramming from silently bypassing security, resource,
* provenance, or determinism controls.
* 
* ============================================================================
* 40. PROVENANCE
* ============================================================================
* 
* The parser must preserve source spans.
* 
* Semantic/generated artifacts must be able to record:
* 
* source
* operation
* inputs
* derived result
* transformation
* originating declaration
* 
* Provenance belongs to the semantic/compiler layers.
* 
* ============================================================================
* 41. QUANTUM INTEGRATION
* ============================================================================
* 
* Type-level syntax may describe quantum types and quantum-related symbolic
* values.
* 
* Examples:
* 
* Qubit[N]
* Register[N]
* State[Shape]
* 
* This does NOT:
* 
* allocate physical qubits;
* select a QPU;
* select topology;
* route operations;
* schedule operations;
* choose calibration;
* perform QEC.
* 
* The canonical path remains:
* 
* TypeExpr
*   |
*   v
* semantic quantum type
*   |
*   v
* quantum::ir
*   |
*   v
* optimization
*   |
*   v
* decomposition
*   |
*   v
* routing
*   |
*   v
* scheduling
*   |
*   v
* resilience/QEC/ZQN
*   |
*   v
* HAL
* 
* No quantum-specific IR is defined here.
* 
* ============================================================================
* 42. HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Type-level values can parameterize abstract hardware types:
* 
* Bus[Width]
* Memory[Depth]
* Pipeline[Stages]
* 
* No physical width/depth/stage ceiling is encoded.
* 
* Hardware realization remains downstream.
* 
* ============================================================================
* 43. CLASSICAL / DATA / AI INTEGRATION
* ============================================================================
* 
* The same type-level mechanism may represent:
* 
* Vector[T, N]
* Matrix[T, Rows, Columns]
* Tensor[T, Shape]
* Dataset[Schema]
* Model[Input, Output]
* 
* No domain-specific type-level grammar is created for these concepts.
* 
* ============================================================================
* 44. DISTRIBUTED INTEGRATION
* ============================================================================
* 
* Type-level values may describe symbolic partitioning:
* 
* Shard[T, Count]
* Partition[T, Shape]
* Replica[T, Count]
* 
* "Count" remains a program-level symbolic value.
* 
* It is not interpreted by this grammar as a physical node count or universal
* limit.
* 
* ============================================================================
* 45. SAFE RUST IMPLEMENTATION CONTRACT
* ============================================================================
* 
* This grammar contains no embedded target-language actions.
* 
* Generated parser integration must therefore remain compatible with:
* 
* Rust 1.97+
* Rust 2021+
* safe Rust
* 
* No "unsafe" implementation is required or permitted for this grammar.
* 
* Compile-time evaluation must be implemented using safe Rust abstractions,
* explicit resource policies, cancellation, diagnostics, and deterministic
* data structures.
* 
* ============================================================================
* 46. RESOURCE EXHAUSTION
* ============================================================================
* 
* An implementation may impose compiler-resource admission policies for:
* 
* memory
* compilation time
* evaluation work
* expansion work
* recursion
* generated artifacts
* 
* Those are not language-level maxima.
* 
* A resource failure must be reported as a compiler/resource diagnostic rather
* than as evidence that the language itself has a fixed semantic limit.
* 
* ============================================================================
* 47. DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Syntax diagnostics belong to the parser.
* 
* Semantic diagnostics should distinguish, where applicable:
* 
* unknown type-level name
* invalid type-level operation
* invalid kind
* invalid argument
* incompatible type-level result
* failed normalization
* failed evaluation
* non-terminating evaluation
* unavailable capability
* forbidden effect
* unavailable resource
* policy violation
* provenance failure
* 
* Diagnostic identifiers are semantic contracts, not lexer tokens.
* 
* ============================================================================
* 48. POSITIVE TEST CONTRACT
* ============================================================================
* 
* The conformance suite should cover, at minimum:
* 
* let N = 4;
* 
* transform tensor::rank(T);
* 
* apply F(T);
* 
* normalize(T);
* 
* is T == U;
* 
* if is T == U then T else U;
* 
* transform quantum::shape(Register);
* 
* transform hardware::capability(T);
* 
* transform resource::requirement(T);
* 
* transform tensor::shape(T);
* 
* nested type-level expressions;
* 
* symbolic values;
* 
* qualified names;
* 
* trailing commas;
* 
* arbitrary finite argument collections.
* 
* The exact accepted keyword spelling must follow the canonical lexer.
* 
* ============================================================================
* 49. NEGATIVE TEST CONTRACT
* ============================================================================
* 
* The suite must reject or semantically diagnose:
* 
* malformed type-level declarations;
* missing declaration values;
* malformed qualified names;
* missing operation arguments where required;
* invalid predicate operators;
* invalid type-level values;
* runtime-only expressions used where type-level values are required;
* unknown operations;
* invalid type-level names;
* incompatible type-level results;
* unauthorized external-state evaluation.
* 
* ============================================================================
* 50. BOUNDARY TEST CONTRACT
* ============================================================================
* 
* Test:
* 
* empty optional argument lists;
* one argument;
* many arguments;
* nested expressions;
* nested generic types;
* symbolic dimensions;
* large symbolic values;
* deeply nested but valid source structures;
* namespaced extensions;
* recursive semantic definitions where legal;
* quantum type parameters;
* tensor dimensions;
* HDL parameters;
* distributed symbolic counts.
* 
* No test may define a universal maximum merely to make the grammar pass.
* 
* ============================================================================
* 51. SCALABILITY CONTRACT
* ============================================================================
* 
* Repetition uses parser repetition operators rather than finite enumerations.
* 
* Examples:
* 
* *
* +
* recursive expression structure
* 
* are used where appropriate.
* 
* The grammar does not establish:
* 
* MAX_ARGUMENTS
* MAX_PARAMETERS
* MAX_DEPTH
* MAX_GENERATED_TYPES
* MAX_DIMENSIONS
* 
* Actual resource exhaustion belongs to compiler/runtime policy.
* 
* ============================================================================
* 52. DETERMINISM CONTRACT
* ============================================================================
* 
* Identical source, language version, and dialect configuration must produce
* identical parse structure.
* 
* Source ordering must be preserved.
* 
* Argument ordering must be preserved.
* 
* Qualified-name component ordering must be preserved.
* 
* No grammar action may introduce nondeterminism.
* 
* ============================================================================
* 53. COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing ordinary type syntax remains authoritative.
* 
* Existing generic syntax remains authoritative.
* 
* Existing dependent-type syntax remains authoritative.
* 
* Existing specialization syntax remains authoritative.
* 
* Existing reflection syntax remains authoritative.
* 
* Existing generation syntax remains authoritative.
* 
* This file must not silently change their meaning.
* 
* Historical/proposed constructs described elsewhere are not automatically
* legal merely because they are documented.
* 
* ============================================================================
* 54. HARD-CODING AUDIT
* ============================================================================
* 
* Forbidden universal limits:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* MAX_REGISTER_WIDTH
* MAX_VECTOR_WIDTH
* MAX_TENSOR_RANK
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* 
* None are defined here.
* 
* Numeric source values are data, not implementation ceilings.
* 
* ============================================================================
* 55. RULE OWNERSHIP SUMMARY
* ============================================================================
* 
* typeExpression
* -> grammar/types/
* 
* expression
* -> grammar/expressions/
* 
* generic syntax
* -> grammar/types/ and grammar/functions/
* 
* dependent types
* -> grammar/types/dependent.g4
* 
* specialization
* -> grammar/metaprogramming/specialization.g4
* 
* reflection
* -> grammar/metaprogramming/reflection.g4
* 
* generation
* -> grammar/metaprogramming/generation.g4
* 
* compile-time execution
* -> grammar/metaprogramming/compile-time*.g4
* 
* macros
* -> grammar/macros/
* 
* resources
* -> grammar/resources/
* 
* capabilities
* -> grammar/resources/ and grammar/hardware/
* 
* effects
* -> grammar/effects/
* 
* contracts
* -> grammar/validation/
* 
* policies
* -> grammar/policies/ / grammar/security/
* 
* type-level metaprogramming
* -> THIS FILE
* 
* ============================================================================
* 56. COMPLETION CRITERIA
* ============================================================================
* 
* This file is DONE when all of the following are true:
* 
* [ ] It composes with the canonical metaprogramming parser.
* 
* [ ] All referenced lexer tokens exist in the canonical lexer.
* 
* [ ] No lexer rule exists in this file.
* 
* [ ] "typeExpression" is not redefined.
* 
* [ ] "expression" is not redefined.
* 
* [ ] Generic syntax is not redefined.
* 
* [ ] Dependent-type syntax is not redefined.
* 
* [ ] Reflection syntax is not redefined.
* 
* [ ] Generation syntax is not redefined.
* 
* [ ] Specialization syntax is not redefined.
* 
* [ ] Compile-time execution syntax is not redefined.
* 
* [ ] Macro syntax is not redefined.
* 
* [ ] No competing AST exists.
* 
* [ ] No competing IR exists.
* 
* [ ] No quantum IR exists here.
* 
* [ ] No hardware topology is encoded.
* 
* [ ] No physical resource allocation is encoded.
* 
* [ ] No universal machine limit exists.
* 
* [ ] Type-level values map to the existing TypeValueExpr architecture.
* 
* [ ] Type results map to the existing TypeExpr architecture.
* 
* [ ] Semantic evaluation remains downstream.
* 
* [ ] Effect checking remains downstream.
* 
* [ ] Capability checking remains downstream.
* 
* [ ] Resource checking remains downstream.
* 
* [ ] Policy checking remains downstream.
* 
* [ ] Provenance remains preserved.
* 
* [ ] Quantum results ultimately use quantum::ir.
* 
* [ ] Positive tests exist.
* 
* [ ] Negative tests exist.
* 
* [ ] Boundary tests exist.
* 
* [ ] Scalability tests exist.
* 
* [ ] Determinism tests exist.
* 
* [ ] Compatibility tests exist.
* 
* [ ] Rust implementation remains compatible with Rust 1.97+.
* 
* [ ] No unsafe Rust is required.
* 
* ============================================================================
* FINAL ARCHITECTURAL INVARIANT
* ============================================================================
* 
* Type-level metaprogramming means:
* 
* COMPUTATION ABOUT TYPES
* 
* It does not mean:
* 
* A SECOND TYPE SYSTEM
* 
* A SECOND AST
* 
* A SECOND IR
* 
* A SECOND QUANTUM IR
* 
* A HARDWARE DESCRIPTION LANGUAGE
* 
* A RUNTIME EXECUTION ENGINE
* 
* The final architecture is:
* 
* type-level source
*      |
*      v
* canonical frontend AST
*      |
*      v
* semantic type system
*      |
*      v
* canonical semantic model
*      |
*      +-----------------------------+
*      |                             |
*      v                             v
* classical semantics          quantum semantics
*                                    |
*                                    v
*                               quantum::ir
*      |
*      v
* target-independent optimization
*      |
*      v
* lowering
*      |
*      v
* routing / scheduling / resilience
*      |
*      v
* QEC / ZQN where applicable
*      |
*      v
* HAL
*      |
*      v
* target realization
* 
* This preserves the POCO-REAF architecture:
* 
* Program Once
*      ->
* Compile Once
*      ->
* Run Everywhere
*      ->
* Run Anywhere
*      ->
* Run Forever
* 
* without freezing the language to today's hardware or introducing
* artificial limits into the grammar.
* 
* ============================================================================
  */
  parser grammar TypeLevelMetaprogramming;

options {
tokenVocab = ZamaniLexer;
}