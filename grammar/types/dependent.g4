/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/types/dependent.g4
* 
* Grammar:
* DependentTypes
* 
* Status:
* PRODUCTION TYPE-SYSTEM DELEGATE
* 
* Rust implementation baseline:
* Rust 1.97+
* 
* Rust edition:
* 2021
* 
* Safety:
* This grammar contains no embedded Rust, no actions, no semantic
* predicates, no native code and no unsafe implementation.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This grammar owns the reusable SOURCE-SYNTAX fragments for dependent and
* value-parameterized type constructs.
* 
* It is deliberately NOT the complete type grammar.
* 
* The canonical complete type-expression owner is:
* 
* grammar/types/types.g4
* 
* Therefore this file MUST remain a delegate.
* 
* Architectural pipeline:
* 
* source
*   |
*   v
* canonical lexer
*   |
*   v
* grammar/types/types.g4
*   |
*   +-----------------------------+
*   |                             |
*   v                             v
* ordinary types          dependent delegates
*                                 |
*                                 v
*                           domain-neutral AST
*                                 |
*                                 v
*                         structural validation
*                                 |
*                                 v
*                          semantic type system
*                                 |
*             +-------------------+-------------------+
*             |                   |                   |
*             v                   v                   v
*          classical          quantum              HDL
*             |                   |                   |
*             +-------------------+-------------------+
*                                 |
*                                 v
*                        canonical semantic model
*                                 |
*                                 v
*                          target-independent IR
*                                 |
*                                 v
*                       domain lowering / optimization
*                                 |
*                                 v
*                     resource / capability analysis
*                                 |
*                                 v
*                       routing / scheduling
*                                 |
*                                 v
*                          target realization
* 
* ============================================================================
* SINGLE-OWNER RULE
* ============================================================================
* 
* THIS FILE OWNS:
* 
* dependentType
* dependentTypeForm
* dependentPiType
* dependentSigmaType
* dependentTypeParameter
* dependentTypeBinding
* dependentTypeArgumentList
* dependentTypeArgument
* 
* THIS FILE DOES NOT OWN:
* 
* typeExpression
* typeCore
* named types
* generic types
* primitive types
* arrays
* slices
* functions
* references
* pointers
* records
* sums
* unions
* ordinary expressions
* general expression constraints
* contract syntax
* resource requirements
* capability requirements
* policy syntax
* type inference
* type checking
* type unification
* constraint solving
* proof checking
* compile-time evaluation
* specialization
* monomorphization
* hardware selection
* quantum allocation
* routing
* scheduling
* backend lowering
* 
* In particular, this file MUST NOT become a second implementation of
* "typeExpression".
* 
* ============================================================================
* PUBLIC DELEGATE CONTRACT
* ============================================================================
* 
* "grammar/types/types.g4" consumes:
* 
* dependentType
* 
* The dependent grammar may internally expose:
* 
* dependentTypeForm
* dependentPiType
* dependentSigmaType
* dependentTypeParameter
* dependentTypeBinding
* dependentTypeArgumentList
* dependentTypeArgument
* 
* The importing grammar remains responsible for deciding where
* "dependentType" is legal in the complete type-expression hierarchy.
* 
* ============================================================================
* LEXER CONTRACT
* ============================================================================
* 
* This grammar defines NO lexer rules.
* 
* All lexical tokens come from the canonical Zamani lexer vocabulary.
* 
* Required lexical vocabulary:
* 
* IDENTIFIER
* COLON
* DOT
* LBRACKET
* RBRACKET
* COMMA
* 
* Dependent-type constructor names:
* 
* Pi
* Sigma
* 
* are intentionally treated as contextual identifiers rather than requiring
* globally reserved lexer tokens.
* 
* This preserves Zamani's extensibility and avoids adding permanent keywords
* where ordinary identifier recognition is sufficient.
* 
* The parser therefore recognizes:
* 
* IDENTIFIER
* 
* and semantic analysis determines whether the identifier is the contextual
* dependent constructor "Pi" or "Sigma" in this type position.
* 
* This also avoids creating duplicate lexical authorities.
* 
* ============================================================================
* WHY PI AND SIGMA ARE NOT LEXER TOKENS
* ============================================================================
* 
* The existing lexer/token registry does not establish dedicated PI/SIGMA
* tokens.
* 
* The normative syntax nevertheless defines:
* 
* Pi
* Sigma
* 
* as dependent-type constructors.
* 
* Therefore the production grammar uses:
* 
* IDENTIFIER
* 
* plus semantic/contextual recognition.
* 
* This is preferable to introducing:
* 
* PI
* SIGMA
* 
* merely for these constructs because:
* 
* 1. the lexer remains centrally owned;
* 2. no duplicate keyword vocabulary is created;
* 3. compatibility remains easier to manage;
* 4. future dialects can register equivalent constructors;
* 5. ordinary identifiers remain extensible elsewhere.
* 
* If a future language-version contract explicitly reserves these spellings,
* the lexer may promote them to canonical tokens. That migration belongs to
* the lexical authority, not this file.
* 
* ============================================================================
* DEPENDENT-TYPE MODEL
* ============================================================================
* 
* Zamani supports two related classes of dependent type syntax:
* 
* A. dependent function/product forms
* 
*    Pi
*    Sigma
* 
* B. value-parameterized type applications
* 
*    Type[value]
* 
* Examples of B include source-level abstractions such as:
* 
* Vector[T][N]
* Matrix[T][Rows, Columns]
* Register[Width]
* Tensor[T][Shape]
* Packet[Payload][Size]
* 
* The grammar does not assign domain-specific meaning to any of these names.
* 
* Their meaning is determined by semantic type registration and validation.
* 
* ============================================================================
* PI TYPES
* ============================================================================
* 
* Normative source form:
* 
* Pi x : A . B
* 
* Meaning:
* 
* a dependent function type whose codomain B may depend on x.
* 
* The grammar only records:
* 
* constructor
* binder
* domain type
* codomain type
* 
* It does not perform:
* 
* substitution
* beta reduction
* normalization
* type checking
* universe checking
* termination checking
* proof checking
* 
* Those belong to semantic analysis.
* 
* ============================================================================
* SIGMA TYPES
* ============================================================================
* 
* Normative source form:
* 
* Sigma x : A . B
* 
* Meaning:
* 
* a dependent pair/product type whose second component B may depend
* on x.
* 
* Again, the grammar records syntax only.
* 
* ============================================================================
* VALUE-PARAMETERIZED TYPES
* ============================================================================
* 
* A dependent type application is represented structurally:
* 
* typePath
* [
*     dependentTypeArgument
*     (COMMA dependentTypeArgument)*
* ]
* 
* The actual "typePath" remains owned by "grammar/types/types.g4".
* 
* The actual type-level value-expression language also remains owned by the
* canonical type-expression/value subsystem.
* 
* This file therefore consumes a structural value-argument boundary instead
* of creating another general expression grammar.
* 
* ============================================================================
* IMPORTANT COMPOSITION RULE
* ============================================================================
* 
* This file MUST NOT import:
* 
* grammar/types/types.g4
* 
* and MUST NOT define:
* 
* typeExpression
* typeCore
* typePath
* 
* because those belong to the canonical type orchestrator.
* 
* The direction of dependency is:
* 
* types.g4
*      |
*      v
* dependent.g4
* 
* never:
* 
* dependent.g4
*      |
*      v
* types.g4
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The grammar must map into the existing frontend source-level type model.
* 
* Existing canonical frontend representation includes:
* 
* TypeExpr::Pi
* TypeExpr::Sigma
* 
* and the repository's type system also has:
* 
* TypeExpr::Identity
* 
* Identity types are intentionally NOT parsed by this file because their
* normative grammar is an expression-bearing construct:
* 
* Identity<Expression, Expression>
* 
* Identity-type ownership therefore belongs to the appropriate type/expression
* composition boundary.
* 
* This prevents dependent.g4 from importing or duplicating the complete
* expression grammar.
* 
* ============================================================================
* AST MAPPING
* ============================================================================
* 
* Pi:
* 
* Pi x : A . B
* 
* maps structurally toward:
* 
* TypeExpr::Pi {
*     parameter: x,
*     parameter_type: A,
*     body: B
* }
* 
* Sigma:
* 
* Sigma x : A . B
* 
* maps structurally toward:
* 
* TypeExpr::Sigma {
*     parameter: x,
*     parameter_type: A,
*     body: B
* }
* 
* Value-parameterized applications must be mapped through the existing
* TypeExpr generic/value representation rather than creating another
* dependent-type AST hierarchy.
* 
* This grammar MUST NOT introduce:
* 
* DependentTypeExpr
* PiTypeExpr
* SigmaTypeExpr
* UniversalDependentType
* HardwareDependentType
* QuantumDependentType
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* The parser produces structure only.
* 
* Semantic analysis owns:
* 
* name resolution
* binder scope
* dependent-variable visibility
* type well-formedness
* dependency checking
* substitution
* unification
* normalization
* constraint generation
* constraint solving
* proof obligations
* universe/kind checking
* recursive-type legality
* specialization
* monomorphization
* compile-time evaluation
* 
* A syntactically valid dependent type is therefore not necessarily
* semantically valid.
* 
* ============================================================================
* BINDER SCOPE
* ============================================================================
* 
* In:
* 
* Pi x : A . B
* 
* and:
* 
* Sigma x : A . B
* 
* the identifier "x" is bound in B.
* 
* It is NOT bound in A.
* 
* The grammar preserves the source ordering required for the semantic
* analyzer to construct the correct scope:
* 
* parse x
*   |
*   v
* parse A
*   |
*   v
* introduce x
*   |
*   v
* parse B
* 
* The grammar does not implement scope mutation.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* A dependent body may refer to:
* 
* its enclosing binder;
* enclosing generic parameters;
* enclosing type parameters;
* other semantically visible symbols;
* symbolic values admitted by the type-level value system.
* 
* Whether a particular reference is legal is semantic analysis.
* 
* ============================================================================
* VALUE ARGUMENT CONTRACT
* ============================================================================
* 
* A dependent type argument is source-level symbolic information.
* 
* It MUST NOT be converted by the grammar into:
* 
* usize
* u64
* host pointer size
* machine word size
* physical resource count
* 
* Examples:
* 
* N
* Length
* Rows
* Columns
* Width
* Shape
* 
* remain symbolic until semantic analysis determines their meaning.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* There are NO grammar-level limits on:
* 
* dependent nesting
* number of binders
* number of dependent arguments
* number of dimensions
* symbolic value magnitude
* type nesting
* generic arity
* tensor rank
* array length
* quantum resource cardinality
* hardware resource cardinality
* distributed resource cardinality
* 
* The grammar MUST NOT introduce constants such as:
* 
* MAX_DEPENDENT_PARAMETERS
* MAX_DEPENDENT_ARGUMENTS
* MAX_DEPENDENT_DEPTH
* MAX_DIMENSIONS
* MAX_TENSOR_RANK
* MAX_ARRAY_LENGTH
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_NODES
* MAX_MEMORY
* 
* Any practical parser/compiler safety limits belong to explicit compiler
* resource policies and must not become language semantics.
* 
* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* Dependent types may describe resource-related quantities symbolically.
* 
* Example semantic intent:
* 
* Register[Width]
* Tensor[T][Shape]
* MemoryRegion[Size]
* 
* However, dependent.g4 does not perform resource negotiation.
* 
* Resource feasibility belongs to:
* 
* grammar/resources/
* semantic resource analysis
* compilation
* execution
* deployment
* 
* A dependent type therefore describes program meaning, not target capacity.
* 
* ============================================================================
* CAPABILITY CONTRACT
* ============================================================================
* 
* No target capability is required merely because a dependent type is parsed.
* 
* A semantic type may later participate in capability analysis.
* 
* Example:
* 
* QuantumRegister[N]
* 
* may eventually cause semantic/resource requirements to be generated, but
* the grammar itself must remain target-independent.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Parsing dependent types has no runtime effect.
* 
* Dependent-type checking may cause compile-time semantic work.
* 
* Compile-time evaluation, if required, is owned by the compile-time/type-level
* subsystem.
* 
* This grammar does not execute dependent expressions.
* 
* ============================================================================
* CONTRACT / VALIDATION CONTRACT
* ============================================================================
* 
* Dependent types may generate semantic obligations such as:
* 
* index < length
* rows = columns
* width > 0
* shape compatibility
* domain/codomain well-formedness
* 
* The grammar does not solve these obligations.
* 
* They are passed to the semantic constraint/proof subsystem.
* 
* ============================================================================
* POLICY CONTRACT
* ============================================================================
* 
* Policies do not belong to this grammar.
* 
* Security, execution, resource, deployment, adaptation, and specialization
* policies are evaluated by their respective policy/semantic subsystems.
* 
* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* The parser must preserve source spans for:
* 
* constructor
* binder
* binder type
* dependent body
* value arguments
* 
* This allows later diagnostics to identify precisely which source construct
* generated a semantic error or proof obligation.
* 
* ============================================================================
* QUANTUM CONTRACT
* ============================================================================
* 
* Dependent types are especially important for quantum/classical hybrid
* programming because symbolic source types may express quantities such as:
* 
* register width
* vector dimension
* circuit parameter count
* tensor shape
* logical resource dimensions
* 
* Example:
* 
* Register[N]
* 
* does NOT mean:
* 
* allocate N physical qubits now.
* 
* It means that the source-level type contains a symbolic parameter N.
* 
* Physical realization remains downstream:
* 
* semantic type
*     |
*     v
* quantum::ir
*     |
*     v
* optimization
*     |
*     v
* decomposition
*     |
*     v
* routing
*     |
*     v
* scheduling
*     |
*     v
* resilience / QEC
*     |
*     v
* ZQN
*     |
*     v
* HAL
* 
* No physical-QPU capacity is encoded here.
* 
* ============================================================================
* HDL CONTRACT
* ============================================================================
* 
* Dependent types may represent symbolic hardware/software relationships:
* 
* Bus[Width]
* Matrix[Rows, Columns]
* Signal[Width]
* 
* They do not determine:
* 
* physical pin count
* implementation technology
* placement
* timing
* routing
* synthesis strategy
* FPGA fabric size
* ASIC implementation
* 
* Those are downstream concerns.
* 
* ============================================================================
* CLASSICAL / AI / DATA CONTRACT
* ============================================================================
* 
* Dependent types are domain-neutral.
* 
* They can represent symbolic structures used by:
* 
* classical computing
* numerical computing
* tensors
* datasets
* models
* knowledge structures
* probabilistic structures
* distributed data
* 
* No application-specific keyword inventory is required.
* 
* ============================================================================
* GENERIC INTEGRATION
* ============================================================================
* 
* Dependent types must compose with generic types without introducing a
* second generic-type grammar.
* 
* Examples:
* 
* Matrix<T>[Rows, Columns]
* Tensor<T>[Shape]
* Register<Qubit>[N]
* 
* The generic argument grammar remains owned by types.g4.
* 
* The dependent argument grammar is a separate semantic dimension:
* 
* type arguments
* +
* value arguments
* 
* The semantic type system decides how these parameters interact.
* 
* ============================================================================
* TYPE-LEVEL VALUE INTEGRATION
* ============================================================================
* 
* The repository already has a canonical type-level value-expression
* subsystem in grammar/types/types.g4.
* 
* This delegate MUST NOT duplicate the complete arithmetic/value grammar.
* 
* The integration boundary is:
* 
* types.g4
*      |
*      +--> typeExpression
*      |
*      +--> typeValueExpression
*      |
*      +--> dependentType
* 
* "types.g4" remains the composition authority.
* 
* ============================================================================
* REQUIRED CHANGE TO grammar/types/types.g4
* ============================================================================
* 
* The existing inline dependentType rule must be removed from types.g4.
* 
* Current duplicated ownership:
* 
* dependentType
*     : typePath
*       LBRACKET
*       typeValueExpression
*       ...
* 
* Production ownership must instead become:
* 
* dependentType
*     : dependentTypeForm
*     ;
* 
* where the imported DependentTypes delegate supplies the dependent
* constructors.
* 
* The exact integration should be:
* 
* import DependentTypes;
* 
* and:
* 
* dependentType
*     : dependentTypeForm
*     ;
* 
* if the repository's generated grammar arrangement requires an explicit
* wrapper.
* 
* If the imported grammar already exposes "dependentType" directly, the
* wrapper must not be duplicated.
* 
* There must ultimately be exactly ONE parser rule named "dependentType".
* 
* ============================================================================
* REQUIRED CHANGE TO types.g4 FOR VALUE-PARAMETERIZED TYPES
* ============================================================================
* 
* The current "types.g4" syntax:
* 
* typePath [ typeValueExpression, ... ]
* 
* should remain the canonical value-parameterized type syntax.
* 
* However, it must not also be represented as a second unrelated dependent
* grammar construct.
* 
* Therefore the production integration should use a distinct delegate rule:
* 
* dependentValueApplication
* 
* with the complete type/value composition still owned by types.g4.
* 
* If the repository chooses to retain bracketed value applications directly
* in "types.g4", "dependent.g4" should own only Pi/Sigma and the shared
* dependent parameter contracts.
* 
* The critical invariant is:
* 
* one syntax
* one parser owner
* one AST mapping
* one semantic meaning
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* Existing source syntax:
* 
* Pi x : A . B
* Sigma x : A . B
* 
* must remain representable.
* 
* Existing "TypeExpr::Pi" and "TypeExpr::Sigma" mappings must remain stable.
* 
* Existing bracketed symbolic type applications must not silently change
* meaning.
* 
* Migration from older spellings must be handled by the language
* compatibility/deprecation subsystem, not by adding duplicate ambiguous
* parser alternatives indefinitely.
* 
* ============================================================================
* DIAGNOSTICS
* ============================================================================
* 
* Parser-level diagnostics must identify malformed structure, including:
* 
* missing dependent constructor
* missing binder
* missing colon
* missing binder type
* missing dot
* missing dependent body
* malformed value-argument list
* missing closing bracket
* malformed separator
* 
* Semantic diagnostics are downstream and include:
* 
* unknown binder
* invalid binder scope
* invalid dependent reference
* ill-formed domain
* ill-formed codomain
* illegal dependency
* unsatisfied dependent constraint
* inconsistent dependent constraints
* unsupported compile-time value
* invalid type/value parameter
* invalid recursive dependency
* invalid specialization
* 
* Diagnostics must retain source spans.
* 
* ============================================================================
* POSITIVE CONFORMANCE EXAMPLES
* ============================================================================
* 
* The following forms must be representable:
* 
* Pi n : Nat . Vector[n]
* 
* Sigma n : Nat . Vector[n]
* 
* Vector[T][N]
* 
* Matrix[T][Rows, Columns]
* 
* Tensor[T][Shape]
* 
* Register[Qubit][N]
* 
* nested dependent forms
* 
* dependent forms inside generic types
* 
* dependent forms inside function types
* 
* dependent forms inside tuple/record types
* 
* dependent forms in quantum source types
* 
* dependent forms in HDL source types
* 
* The examples are illustrative only.
* 
* They do not reserve:
* 
* Nat
* Vector
* Matrix
* Tensor
* Register
* Qubit
* 
* as universal keywords.
* 
* ============================================================================
* NEGATIVE CONFORMANCE EXAMPLES
* ============================================================================
* 
* These must fail structurally:
* 
* Pi
* 
* Pi n
* 
* Pi n :
* 
* Pi n : A
* 
* Pi : A . B
* 
* Pi n A . B
* 
* Sigma
* 
* Sigma n :
* 
* Sigma n : A
* 
* Type[
* 
* Type[N
* 
* Type[N,
* 
* Type[N,,M]
* 
* Semantic-invalid examples must reach semantic analysis rather than being
* rejected merely because their identifiers are unfamiliar.
* 
* ============================================================================
* BOUNDARY TESTS
* ============================================================================
* 
* The test suite must cover:
* 
* Pi + generic types
* Sigma + generic types
* Pi + array types
* Sigma + tensor types
* dependent + quantum types
* dependent + HDL types
* dependent + resource abstractions
* dependent + capability abstractions
* dependent + function types
* nested Pi
* nested Sigma
* Pi containing Sigma
* Sigma containing Pi
* symbolic values
* qualified type paths
* source spans
* semantic binder visibility
* serialization/deserialization of resulting AST
* 
* ============================================================================
* SCALABILITY TESTS
* ============================================================================
* 
* Production tests must verify that no source-language ceiling is introduced
* by this grammar.
* 
* Tests should construct progressively larger inputs subject only to the
* explicit compiler/test resource policy.
* 
* Required categories:
* 
* many dependent binders
* deeply nested dependent types
* many dependent arguments
* large symbolic values
* large symbolic dimensions
* large generic/dependent combinations
* large quantum type parameters
* large HDL dimensions
* 
* The test harness may impose a resource budget.
* 
* That budget is a test/compiler execution policy and is NOT a language
* constant.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Given identical:
* 
* source
* lexer configuration
* grammar version
* language version
* 
* parsing must produce equivalent deterministic syntax structure.
* 
* No semantic decisions are permitted in this grammar.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar emits NO IR.
* 
* The required lowering path is:
* 
* dependent source syntax
*      |
*      v
* frontend TypeExpr
*      |
*      v
* semantic dependent type
*      |
*      v
* canonical semantic model
*      |
*      v
* canonical IR
*      |
*      +--> classical
*      +--> quantum::ir
*      +--> HDL/hardware
*      +--> distributed
*      +--> future domains
* 
* The dependent grammar must never directly reference:
* 
* LLVM
* MLIR
* QIR
* vendor IR
* QPU topology
* FPGA fabric
* CPU count
* GPU count
* physical memory
* routing
* scheduling
* QEC
* calibration
* 
* ============================================================================
* RUST CONTRACT
* ============================================================================
* 
* This grammar itself contains no Rust implementation.
* 
* The downstream implementation must remain compatible with:
* 
* Rust 1.97+
* Rust 2021
* stable Rust
* no unsafe
* 
* The grammar must not require unsafe parser/runtime integration.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* canonical Zamani lexer vocabulary
* IDENTIFIER
* COLON
* DOT
* LBRACKET
* RBRACKET
* COMMA
* 
* EXPORTS:
* 
* dependentType
* dependentTypeForm
* dependentPiType
* dependentSigmaType
* dependentTypeParameter
* dependentTypeBinding
* dependentTypeArgumentList
* dependentTypeArgument
* 
* CONSUMED_BY:
* 
* grammar/types/types.g4
* 
* AST_OWNER:
* 
* src/frontend/ast/node/types/type_expr.rs
* 
* SEMANTIC_OWNER:
* 
* Zamani semantic/type-analysis subsystem
* 
* CONSTRAINT_OWNER:
* 
* type constraint / semantic validation subsystem
* 
* PROOF_OWNER:
* 
* semantic contract/proof subsystem
* 
* VALUE_EVALUATION_OWNER:
* 
* compile-time/type-level evaluation subsystem
* 
* IR_OWNER:
* 
* canonical Zamani semantic/IR lowering subsystem
* 
* QUANTUM_IR_OWNER:
* 
* quantum semantic lowering and quantum::ir
* 
* TEST_OWNER:
* 
* grammar/tests/type/dependent/
* 
* SPEC_OWNER:
* 
* grammar/spec/syntax.md
* grammar/specification/types.md
* 
* COMPATIBILITY_OWNER:
* 
* grammar/compatibility/
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is DONE only when:
* 
* [ ] it contains no complete type-expression grammar;
* [ ] it contains no lexer rules;
* [ ] it contains no embedded Rust;
* [ ] it contains no semantic predicates;
* [ ] it contains no unsafe implementation;
* [ ] Pi syntax matches the normative syntax;
* [ ] Sigma syntax matches the normative syntax;
* [ ] binder scope is structurally preserved;
* [ ] dependent value applications have one canonical owner;
* [ ] no duplicate dependentType rule exists elsewhere;
* [ ] types.g4 remains the complete type-expression authority;
* [ ] TypeExpr::Pi is reachable from Pi syntax;
* [ ] TypeExpr::Sigma is reachable from Sigma syntax;
* [ ] source spans remain available to the AST;
* [ ] semantic validation remains downstream;
* [ ] constraint solving remains downstream;
* [ ] proof checking remains downstream;
* [ ] resource negotiation remains downstream;
* [ ] capability negotiation remains downstream;
* [ ] quantum realization remains downstream;
* [ ] HDL realization remains downstream;
* [ ] no hardware-size constants exist;
* [ ] no artificial dependent-type limits exist;
* [ ] positive tests exist;
* [ ] negative tests exist;
* [ ] boundary tests exist;
* [ ] scalability tests exist;
* [ ] deterministic parsing tests exist;
* [ ] compatibility tests exist;
* [ ] Rust 1.97+ implementation passes without unsafe;
* [ ] canonical grammar generation succeeds;
* [ ] canonical parser generation succeeds;
* [ ] AST integration succeeds;
* [ ] semantic integration succeeds.
* 
* ============================================================================
  */

/* ============================================================================

* PUBLIC DEPENDENT-TYPE ENTRY
* ========================================================================== */

/*

* The complete dependent-type form is deliberately delegated.
* 
* "types.g4" owns the surrounding type-expression context.
  */
  dependentType
  : dependentTypeForm
  ;

/* ============================================================================

* DEPENDENT-TYPE FORMS
* ========================================================================== */

/*

* The currently normative dependent constructors are Pi and Sigma.
* 
* They are recognized contextually through IDENTIFIER rather than through
* additional globally reserved lexer tokens.
  */
  dependentTypeForm
  : dependentPiType
  | dependentSigmaType
  ;

/* ============================================================================

* PI TYPE
* ========================================================================== */

/*

* Normative form:
* 
* Pi IDENT : TypeExpression . TypeExpression
* 
* Example:
* 
* Pi n : Nat . Vector[n]
* 
* The surrounding typeExpression rule is intentionally referenced here as
* the canonical composition boundary supplied by the importing type grammar.
* 
* No complete type grammar is defined locally.
  */
  dependentPiType
  : dependentTypeConstructorPi
  dependentTypeBinding
  DOT
  typeExpression
  ;

/*

* Contextual constructor:
* 
* Pi
* 
* "IDENTIFIER" is used intentionally.
* 
* Semantic/contextual validation determines whether the identifier is the
* dependent constructor in this position.
  */
  dependentTypeConstructorPi
  : IDENTIFIER
  ;

/* ============================================================================

* SIGMA TYPE
* ========================================================================== */

/*

* Normative form:
* 
* Sigma IDENT : TypeExpression . TypeExpression
* 
* Example:
* 
* Sigma n : Nat . Vector[n]

*/
dependentSigmaType
: dependentTypeConstructorSigma
dependentTypeBinding
DOT
typeExpression
;

dependentTypeConstructorSigma
: IDENTIFIER
;

/* ============================================================================

* DEPENDENT BINDING
* ========================================================================== */

/*

* The binding is shared by Pi and Sigma.
* 
* The binder type is a complete Zamani type expression.
* 
* Scope semantics:
* 
* parameter is NOT in scope in parameterType
* parameter IS in scope in the dependent body

*/
dependentTypeBinding
: dependentTypeParameter
COLON
typeExpression
;

/*

* The binder name is deliberately unrestricted beyond canonical identifier
* syntax.
* 
* No predefined names such as:
* 
* N
* Length
* Rows
* Columns
* Width
* 
* are required.
  */
  dependentTypeParameter
  : IDENTIFIER
  ;

/* ============================================================================

* DEPENDENT VALUE-ARGUMENT DELEGATE
* ========================================================================== */

/*

* These rules are provided as a structural delegate for type compositions
* that use value parameters.
* 
* The actual type-expression/value-expression owner remains the canonical
* type orchestrator.
* 
* The rules intentionally do not define arithmetic, boolean, comparison,
* function-call, or ordinary expression syntax.
  /
  dependentTypeArgumentList
  : dependentTypeArgument
  (COMMA dependentTypeArgument)
  COMMA?
  ;

dependentTypeArgument
: typeValueExpression
;

/* ============================================================================

* END
* ============================================================================
  */