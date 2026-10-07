/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/types/advanced-type-level-computation.g4
* 
* Grammar:
* AdvancedTypeLevelComputation
* 
* Status:
* PRODUCTION TYPE-SYSTEM COMPOSITION DELEGATE
* 
* Compiler baseline:
* Rust 1.97+
* Rust 2021
* safe Rust only
* no unsafe
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the canonical composition boundary for ADVANCED TYPE-LEVEL
* COMPUTATION.
* 
* IMPORTANT:
* 
* This file does NOT create a second type-level language.
* 
* The repository already has specialized owners for:
* 
* type-level computation:
*     grammar/metaprogramming/type-level.g4
* 
* type families:
*     grammar/types/type-families.g4
* 
* higher-kinded types:
*     grammar/types/higher-kinded-types.g4
* 
* dependent types:
*     grammar/types/dependent.g4
* 
* generic application:
*     grammar/types/generic.g4
* 
* type constraints:
*     grammar/types/type-constraints.g4
* 
* universal type composition:
*     grammar/types/types.g4
* 
* This file therefore provides ONE stable integration boundary through which
* advanced type-level computation can consume those facilities without
* duplicating their syntax or creating another AST/IR hierarchy.
* 
* ============================================================================
* ARCHITECTURAL PRINCIPLE
* ============================================================================
* 
* The correct ownership model is:
* 
* advanced-type-level-computation.g4
*             |
*   +---------+----------+----------------+----------------+
*   |                    |                |                |
*   v                    v                v                v
* type-level            type families       HKT          dependent types
* metaprogramming
*   |                    |                |                |
*   +--------------------+----------------+----------------+
*                        |
*                        v
*                canonical TypeExpr
*                        |
*                        v
*               semantic type system
*                        |
*         +--------------+--------------+
*         |              |              |
*         v              v              v
*     classical       quantum          HDL
*      semantic       semantic       semantic
*       model          model          model
*         |              |              |
*         |              v              |
*         |          quantum::ir       |
*         |                             |
*         +--------------+--------------+
*                        |
*                        v
*               canonical semantic IR
*                        |
*                        v
*               optimization/lowering
*                        |
*               routing/scheduling
*                        |
*                resilience / QEC
*                        |
*                        v
*                       ZQN
*                        |
*                        v
*                       HAL
*                        |
*                        v
*                 target realization
* 
* ============================================================================
* WHY THIS FILE EXISTS
* ============================================================================
* 
* Advanced type-level computation needs a stable composition point because
* Zamani supports all of the following simultaneously:
* 
* generic types
* dependent types
* higher-kinded types
* type families
* associated types
* type-level values
* type-level predicates
* normalization
* symbolic computation
* constraints
* refinements
* compile-time type transformations
* quantum type abstractions
* hardware-neutral type abstractions
* tensor/data type abstractions
* distributed type abstractions
* 
* These mechanisms must compose without becoming separate type systems.
* 
* ============================================================================
* SINGLE-OWNER RULE
* ============================================================================
* 
* THIS FILE OWNS ONLY:
* 
* advancedTypeLevelComputation
* advancedTypeLevelExpression
* advancedTypeLevelDeclaration
* advancedTypeLevelTypeResult
* advancedTypeLevelValueResult
* advancedTypeLevelPredicateResult
* advancedTypeLevelFamilyResult
* advancedTypeLevelKindResult
* advancedTypeLevelDependentResult
* advancedTypeLevelApplicationResult
* 
* These are composition-boundary rules.
* 
* THIS FILE DOES NOT OWN:
* 
* typeExpression
* typeCore
* typePostfix
* genericTypeArguments
* genericArgumentList
* genericParameter
* typeBound
* typeConstraintClause
* typeFamilyDeclaration
* typeFamilyInstance
* typeFamilyEquation
* higherKindedKind
* higherKindedParameter
* higherKindedTypeReference
* dependentType
* dependentPiType
* dependentSigmaType
* typeLevelExpression
* typeLevelDeclaration
* typeLevelValue
* typeLevelPredicate
* typeLevelOperation
* typeLevelQuery
* typeLevelLambda
* associated-type syntax
* ordinary expressions
* runtime statements
* reflection syntax
* source-generation syntax
* macro syntax
* specialization syntax
* effect syntax
* capability syntax
* resource syntax
* policy syntax
* contract syntax
* quantum operation syntax
* HDL syntax
* hardware realization
* 
* ============================================================================
* CRITICAL CORRECTION
* ============================================================================
* 
* A previous/naive implementation of this file would be incorrect if it
* redefined any of the following:
* 
* typeExpression
* genericTypeArguments
* genericArgumentList
* typeLevelExpression
* typeLevelValue
* typeLevelPredicate
* typeFamilyDeclaration
* typeFamilyEquation
* higherKindedKind
* dependentType
* 
* Doing so would create competing grammar authorities.
* 
* This file deliberately delegates instead.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar introduces NO new AST hierarchy.
* 
* All underlying constructs must converge on the repository's existing
* frontend representations.
* 
* Type-level results use the existing type/value representations, including
* the repository's canonical:
* 
* TypeExpr
* TypeValueExpr
* 
* Existing representations such as:
* 
* TypeExpr::Generic
* TypeExpr::TypeApplication
* TypeExpr::Hkt
* TypeExpr::Pi
* TypeExpr::Sigma
* TypeExpr::Associated
* 
* remain the canonical AST destination where applicable.
* 
* This file MUST NOT introduce:
* 
* AdvancedTypeExpr
* AdvancedTypeLevelExpr
* TypeComputationExpr
* TypeFamilyExpr
* KindExpr2
* DependentTypeExpr
* TypeLevelIR
* AdvancedTypeIR
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Parsing establishes only source structure.
* 
* Semantic analysis owns:
* 
* name resolution
* type resolution
* generic substitution
* kind inference
* kind checking
* kind unification
* type unification
* type-family reduction
* normalization
* dependent-type checking
* constraint generation
* constraint solving
* refinement checking
* termination checking
* confluence checking
* overlap checking
* specialization
* monomorphization
* compile-time evaluation
* provenance
* diagnostics
* 
* No semantic predicate is permitted in this grammar to inspect:
* 
* symbol tables
* type environments
* resource availability
* hardware
* capabilities
* runtime state
* 
* ============================================================================
* TYPE-LEVEL PURITY
* ============================================================================
* 
* Type-level computation describes compile-time semantic computation.
* 
* It MUST NOT implicitly become runtime execution.
* 
* In particular, parsing or evaluating a type-level construct must not
* implicitly provide access to:
* 
* filesystem
* network
* subprocesses
* credentials
* environment variables
* wall-clock time
* uncontrolled randomness
* target hardware
* physical quantum devices
* runtime mutable state
* 
* If a future type-level operation requires external state, its semantic
* implementation must explicitly participate in the repository's:
* 
* effect system
* capability system
* resource system
* policy system
* provenance system
* 
* The grammar itself remains target-independent.
* 
* ============================================================================
* ADVANCED EXPRESSION BOUNDARY
* ============================================================================
* 
* An advanced type-level expression is deliberately defined as a composition
* of existing type-level mechanisms.
* 
* This prevents the advanced type system from becoming a second expression
* language.
* 
* The composition is:
* 
* ordinary type-level expression
* type-level application
* type-level transformation
* type-level normalization
* type-level predicate
* type-level query
* type-level result
* type-level value
* 
* where those rules are owned by:
* 
* grammar/metaprogramming/type-level.g4
* 
* ============================================================================
* PUBLIC ENTRY
* ============================================================================
* 
* "advancedTypeLevelComputation" is intentionally a composition rule.
* 
* It is NOT a replacement for:
* 
* typeLevelMetaprogramming
* 
* and it is NOT a replacement for:
* 
* typeExpression
* 
* Its purpose is to give the canonical type-system orchestrator a stable
* named boundary for advanced type-level facilities.
  */
  advancedTypeLevelComputation
  : advancedTypeLevelDeclaration
  | advancedTypeLevelExpression
  ;

/* ============================================================================

* ADVANCED TYPE-LEVEL DECLARATION
* ============================================================================
* 
* Declaration syntax remains owned by the type-level metaprogramming grammar.
* 
* This wrapper prevents declaration syntax from being duplicated here.
  */
  advancedTypeLevelDeclaration
  : typeLevelDeclaration
  ;

/* ============================================================================

* ADVANCED TYPE-LEVEL EXPRESSION
* ============================================================================
* 
* Every alternative delegates to an existing owner.
* 
* No new computation operator is introduced here.
  */
  advancedTypeLevelExpression
  : advancedTypeLevelApplicationResult
  | advancedTypeLevelTypeResult
  | advancedTypeLevelValueResult
  | advancedTypeLevelPredicateResult
  | advancedTypeLevelFamilyResult
  | advancedTypeLevelKindResult
  | advancedTypeLevelDependentResult
  ;

/* ============================================================================

* TYPE-LEVEL APPLICATION
* ============================================================================
* 
* Ordinary type-level application remains owned by the existing type-level
* metaprogramming grammar.
  */
  advancedTypeLevelApplicationResult
  : typeLevelApplicationExpression
  ;

/* ============================================================================

* TYPE-LEVEL TYPE RESULT
* ============================================================================
* 
* A canonical Zamani type is already the authoritative type-level result.
* 
* This is deliberately:
* 
* typeExpression
* 
* and never a second advanced type grammar.
  */
  advancedTypeLevelTypeResult
  : typeLevelTypeResult
  ;

/* ============================================================================

* TYPE-LEVEL VALUE RESULT
* ============================================================================
* 
* Type-level values remain owned by TypeLevelMetaprogramming.
* 
* The semantic layer determines:
* 
* value kind
* constant evaluability
* symbolic evaluation
* normalization
* overflow behavior
* constraint generation
* 
* This grammar performs none of those operations.
  */
  advancedTypeLevelValueResult
  : typeLevelValueResult
  ;

/* ============================================================================

* TYPE-LEVEL PREDICATE RESULT
* ============================================================================
* 
* Predicates remain owned by the existing type-level grammar.
* 
* They may later contribute semantic obligations to:
* 
* constraints
* refinements
* contracts
* proofs
* 
* but this grammar does not solve them.
  */
  advancedTypeLevelPredicateResult
  : typeLevelPredicateExpression
  ;

/* ============================================================================

* TYPE-FAMILY RESULT
* ============================================================================
* 
* Type-family declarations and equations remain declaration-level constructs.
* 
* A family application itself continues to use ordinary generic/type
* application syntax.
* 
* This wrapper exists only so the advanced type-level composition boundary can
* explicitly consume a family-related result where the surrounding grammar
* permits declaration/evaluation composition.
* 
* It MUST NOT redefine:
* 
* F<T>
* 
* because that remains owned by grammar/types/generic.g4.
  */
  advancedTypeLevelFamilyResult
  : typeFamilyEquation
  ;

/* ============================================================================

* HIGHER-KINDED RESULT
* ============================================================================
* 
* Higher-kinded structure remains owned by HigherKindedTypes.
* 
* This rule exposes only the existing higher-kinded type reference.
* 
* Kind checking and kind inference remain semantic operations.
  */
  advancedTypeLevelKindResult
  : higherKindedTypeReference
  ;

/* ============================================================================

* DEPENDENT RESULT
* ============================================================================
* 
* Dependent Pi/Sigma syntax remains owned by dependent.g4.
* 
* The semantic layer determines:
* 
* binder scope
* dependency validity
* substitution
* normalization
* proof obligations
* specialization
* 
* This wrapper performs none of those operations.
  */
  advancedTypeLevelDependentResult
  : dependentType
  ;

/* ============================================================================

* TYPE-LEVEL RESULT CONTRACT
* ============================================================================
* 
* Every successful advanced computation ultimately resolves to one or more
* existing canonical semantic entities:
* 
* TypeExpr
* TypeValueExpr
* type constraint
* kind information
* declaration metadata
* 
* There is no:
* 
* AdvancedTypeLevelIR
* 
* and no:
* 
* AdvancedTypeLevelRuntime
* 
* ============================================================================
* GENERIC INTEGRATION
* ============================================================================
* 
* Generic application remains owned by:
* 
* grammar/types/generic.g4
* 
* Example:
* 
* Map<Key, Value>
* 
* remains ordinary generic syntax.
* 
* An advanced computation may produce or transform generic arguments, but the
* generic application grammar itself is never duplicated here.
* 
* The semantic pipeline is:
* 
* advanced type-level computation
*      |
*      v
* canonical TypeExpr
*      |
*      v
* generic/type resolution
*      |
*      v
* substitution
*      |
*      v
* normalization
* 
* ============================================================================
* TYPE-FAMILY INTEGRATION
* ============================================================================
* 
* Type families remain owned by:
* 
* grammar/types/type-families.g4
* 
* The family grammar already guarantees that family application is represented
* through the ordinary generic/type-application machinery.
* 
* Therefore:
* 
* Element<List<T>>
* 
* MUST NOT acquire a second parser representation merely because it participates
* in advanced computation.
* 
* Semantic processing determines whether "Element" denotes:
* 
* ordinary type constructor
* generic constructor
* type family
* higher-kinded constructor
* other registered type-level entity
* 
* ============================================================================
* HIGHER-KINDED INTEGRATION
* ============================================================================
* 
* Higher-kinded syntax remains owned by:
* 
* grammar/types/higher-kinded-types.g4
* 
* Examples:
* 
* F : *
* F : * -> *
* F : * -> * -> *
* 
* No finite kind arity or nesting limit is encoded.
* 
* Semantic analysis determines:
* 
* kind inference
* kind unification
* constructor saturation
* partial application
* kind compatibility
* 
* ============================================================================
* DEPENDENT-TYPE INTEGRATION
* ============================================================================
* 
* Dependent types remain owned by:
* 
* grammar/types/dependent.g4
* 
* Examples:
* 
* Pi n : Nat . Vector[n]
* 
* Sigma n : Nat . Matrix[n, n]
* 
* Advanced type-level computation may produce values used by dependent types.
* 
* The boundary is:
* 
* type-level value
*      |
*      v
* TypeValueExpr
*      |
*      v
* dependent type semantic checking
* 
* The grammar does not evaluate the value.
* 
* ============================================================================
* CONSTRAINT INTEGRATION
* ============================================================================
* 
* Advanced computation may generate or consume type constraints.
* 
* It does not own:
* 
* typeBound
* typeConstraintClause
* whereClause
* resource constraints
* capability constraints
* contracts
* 
* Existing owners remain authoritative.
* 
* Constraint solving is semantic.
* 
* ============================================================================
* REFINEMENT INTEGRATION
* ============================================================================
* 
* Refinements remain semantic relationships over canonical types and values.
* 
* A type-level predicate may produce a refinement obligation.
* 
* Example conceptual relationship:
* 
* Vector[N]
* 
* together with:
* 
* N > 0
* 
* is represented structurally first and validated semantically later.
* 
* No theorem prover or solver is embedded in this grammar.
* 
* ============================================================================
* ASSOCIATED-TYPE INTEGRATION
* ============================================================================
* 
* Associated projections remain owned by:
* 
* grammar/types/associated.g4
* 
* Examples:
* 
* T::Item
* Iterator<T>::Item
* 
* This file must never introduce another "DOUBLE_COLON" projection grammar.
* 
* ============================================================================
* TYPE-CLASS / BOUND INTEGRATION
* ============================================================================
* 
* Type classes, traits, and bounds remain owned by their existing grammars.
* 
* Advanced computation may consume types satisfying such bounds.
* 
* Semantic analysis determines:
* 
* implementation existence
* bound satisfaction
* coherence
* associated-type obligations
* higher-kinded compatibility
* 
* ============================================================================
* COMPILE-TIME INTEGRATION
* ============================================================================
* 
* Advanced type-level computation is compile-time semantic work.
* 
* It may interact with the repository's compile-time facilities, but it must
* remain distinct from unrestricted compile-time execution.
* 
* The implementation must preserve the distinction between:
* 
* pure type-level computation
* 
* and:
* 
* compile-time execution requiring effects/capabilities/resources.
* 
* ============================================================================
* METAPROGRAMMING INTEGRATION
* ============================================================================
* 
* The existing metaprogramming subsystem owns:
* 
* type-level computation
* reflection
* generation
* specialization
* compile-time execution
* 
* This file provides only the type-system-facing composition boundary.
* 
* Generated source must re-enter the normal:
* 
* lexer
* parser
* AST
* structural validation
* semantic analysis
* 
* pipeline.
* 
* Type-level computation MUST NOT bypass normal validation.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Advanced type-level computation may describe quantum abstractions:
* 
* Register[N]
* State[Dimension]
* Circuit[Parameters]
* QuantumState<T>
* LogicalQubit
* 
* These remain source-level semantic types.
* 
* This grammar does not:
* 
* allocate qubits
* count physical qubits
* select QPU hardware
* choose topology
* route operations
* schedule operations
* choose calibration
* choose QEC
* 
* The semantic path remains:
* 
* TypeExpr
*    |
*    v
* semantic quantum type
*    |
*    v
* quantum::ir
*    |
*    v
* optimization
*    |
*    v
* decomposition
*    |
*    v
* routing
*    |
*    v
* scheduling
*    |
*    v
* resilience / QEC
*    |
*    v
* ZQN
*    |
*    v
* HAL
* 
* ============================================================================
* CLASSICAL / DATA / AI INTEGRATION
* ============================================================================
* 
* The same type-level machinery can represent:
* 
* Vector<T, N>
* Matrix<T, Rows, Columns>
* Tensor<T, Shape>
* Dataset<Schema>
* Model<Input, Output>
* Distribution<T>
* Knowledge<T>
* 
* These are ordinary source-level types and generic applications.
* 
* No application-specific type keyword catalogue is introduced.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Advanced type-level computation may parameterize abstract hardware/HDL
* types:
* 
* Signal<T>
* Bus<Width>
* Register<Value>
* Pipeline<Stages>
* Memory<Layout>
* 
* The grammar does not encode:
* 
* physical register width
* physical memory size
* FPGA fabric size
* ASIC technology
* number of ports
* number of devices
* topology
* clock count
* 
* Such properties are semantic/resource/target information.
* 
* ============================================================================
* RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* Type-level computation does not discover resources.
* 
* A computed type may participate in a program that declares:
* 
* requires capability("tensor.compute")
* 
* requires capability("quantum.measurement")
* 
* requires memory >= required_memory
* 
* requires topology(required_topology)
* 
* But those requirements belong to:
* 
* grammar/resources/
* grammar/core/
* grammar/hardware/
* 
* and their semantic consumers.
* 
* ============================================================================
* EFFECT INTEGRATION
* ============================================================================
* 
* The grammar introduces no effects.
* 
* Type-level semantic operations may nevertheless be classified by the
* semantic effect system where the language explicitly permits external
* state.
* 
* Possible semantic effect categories include:
* 
* reflection
* code_generation
* simulation
* native
* foreign
* randomness
* external_state
* 
* No effect is implied merely by parsing a type-level construct.
* 
* ============================================================================
* POLICY INTEGRATION
* ============================================================================
* 
* Type-level evaluation may be subject to:
* 
* compilation policy
* security policy
* determinism policy
* resource policy
* provenance policy
* metaprogramming policy
* 
* Policy ownership remains outside this grammar.
* 
* The type-level evaluator must not bypass policy enforcement.
* 
* ============================================================================
* PROVENANCE INTEGRATION
* ============================================================================
* 
* Every advanced type-level construct must remain traceable to its source.
* 
* The parser must preserve ordinary source spans.
* 
* Semantic provenance may later record:
* 
* source expression
* operation
* inputs
* substitutions
* normalization
* family equation
* inferred kind
* generated result
* verification
* diagnostic cause
* 
* This file does not implement provenance storage.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* For identical:
* 
* source
* language version
* grammar version
* lexical configuration
* dialect configuration
* 
* parsing must produce deterministic structure.
* 
* No parser decision may depend on:
* 
* target hardware
* memory availability
* processor count
* network state
* filesystem state
* runtime state
* randomness
* wall-clock time
* 
* ============================================================================
* SCALABILITY / POCO-REAF
* ============================================================================
* 
* This file imposes NO language-level maximum on:
* 
* type-level expressions
* type-level declarations
* generic arity
* family arity
* family equations
* kind arity
* kind nesting
* dependent binders
* dependent nesting
* symbolic dimensions
* tensor rank
* quantum cardinality
* hardware cardinality
* distributed cardinality
* source size
* 
* It MUST NOT define or imply:
* 
* MAX_TYPE_LEVEL_*
* MAX_TYPE_FAMILY_*
* MAX_KIND_*
* MAX_GENERIC_*
* MAX_DEPENDENT_*
* MAX_TENSOR_RANK
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_REGISTER_WIDTH
* MAX_DEVICE_COUNT
* 
* Operational parser/compiler safeguards are allowed only as explicit
* implementation/resource policy.
* 
* Such safeguards MUST:
* 
* be configurable;
* be observable;
* be diagnosable;
* not change source-language meaning;
* not depend on physical target capacity.
* 
* ============================================================================
* TARGET INDEPENDENCE
* ============================================================================
* 
* Advanced type-level computation MUST NOT select:
* 
* CPU
* GPU
* FPGA
* ASIC
* accelerator
* QPU
* simulator
* cluster
* cloud
* vendor
* ABI
* physical topology
* 
* Type-level meaning remains stable across target realizations.
* 
* Target-specific specialization happens downstream.
* 
* ============================================================================
* ERROR CONTRACT
* ============================================================================
* 
* Parser-level errors belong to the delegated grammar that owns the syntax.
* 
* This composition file must not invent alternative error productions for
* delegated constructs.
* 
* Semantic errors include:
* 
* unknown type
* unknown type family
* invalid family reduction
* kind mismatch
* failed kind inference
* invalid dependent type
* unsatisfied constraint
* failed normalization
* non-terminating type computation
* ambiguous type-level operation
* invalid specialization
* invalid generated type
* 
* These are semantic/compiler diagnostics, not grammar alternatives.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* This file contains:
* 
* no embedded Rust
* no semantic predicates
* no filesystem access
* no network access
* no subprocess execution
* no environment inspection
* no hardware probing
* no credential access
* no runtime execution
* no unsafe code
* 
* ============================================================================
* RUST CONTRACT
* ============================================================================
* 
* ANTLR grammar source is target-neutral.
* 
* The Rust frontend generated/consuming this grammar must remain compatible
* with:
* 
* Rust 1.97+
* Rust 2021
* 
* and:
* 
* safe Rust only
* no unsafe
* 
* No nightly-only Rust feature is required by this grammar.
* 
* ============================================================================
* COMPOSITION CONTRACT
* ============================================================================
* 
* This file is intended to be imported by the canonical type-system
* composition hierarchy.
* 
* Recommended ownership topology:
* 
* Zamani.g4
*      |
*      v
* ZamaniParser.g4
*      |
*      v
* Type
*      |
*      +--> AdvancedTypeLevelComputation
*      |
*      +--> Generic
*      +--> TypeFamilies
*      +--> HigherKindedTypes
*      +--> Dependent
*      +--> TypeConstraints
*      +--> other specialized type delegates
* 
* The advanced delegate MUST NOT itself redefine those delegates' rules.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH grammar/types/types.g4
* ============================================================================
* 
* "types.g4" remains the sole owner of:
* 
* typeExpression
* typeCore
* typePostfix
* typeExtension
* 
* It should expose the advanced boundary through its "typeCore"/specialized
* composition only where an advanced type-level construct is syntactically
* legal as a type.
* 
* In particular, "advancedTypeLevelComputation" MUST NOT be inserted directly
* into "typeExpression" as an unconditional alternative if that would allow
* declaration-level constructs in places where a type is required.
* 
* Correct semantic distinction:
* 
* type-level declaration
*     -> declaration grammar
* 
* type-level expression
*     -> type-level/metaprogramming grammar
* 
* resulting type
*     -> typeExpression / TypeExpr
* 
* This prevents declaration syntax from becoming a type constructor.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH grammar/metaprogramming/type-level.g4
* ============================================================================
* 
* "TypeLevelMetaprogramming" remains the canonical owner of:
* 
* typeLevelMetaprogramming
* typeLevelDeclaration
* typeLevelExpression
* typeLevelValue
* typeLevelPredicate
* typeLevelOperation
* typeLevelQuery
* typeLevelLambda
* typeLevelApplicationExpression
* typeLevelNormalizeExpression
* 
* This file consumes those rules.
* 
* No rule in this file may redefine them.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH grammar/types/type-families.g4
* ============================================================================
* 
* "TypeFamilies" remains the canonical owner of:
* 
* typeFamilyDeclaration
* typeFamilyInstance
* typeFamilyEquation
* typeFamilyName
* typeFamilyParameterList
* typeFamilyParameter
* typeFamilyParameterKind
* typeFamilyResultKind
* typeFamilyEquationArguments
* 
* Family applications remain ordinary type applications.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH grammar/types/higher-kinded-types.g4
* ============================================================================
* 
* "HigherKindedTypes" remains the canonical owner of:
* 
* higherKindedKind
* higherKindedKindAtom
* higherKindedKindArrow
* higherKindedParameter
* higherKindedParameterList
* higherKindedTypeReference
* 
* No HKT syntax is duplicated here.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH grammar/types/dependent.g4
* ============================================================================
* 
* "Dependent" remains the canonical owner of:
* 
* dependentType
* dependentPiType
* dependentSigmaType
* dependentTypeBinding
* 
* Advanced computation may produce values consumed by dependent types.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH grammar/types/generic.g4
* ============================================================================
* 
* "Generic" remains the sole owner of:
* 
* genericTypeArguments
* genericArgumentList
* genericTypeApplicationSuffix
* 
* This file must never create a second generic argument grammar.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH grammar/types/type-constraints.g4
* ============================================================================
* 
* Type constraints remain compatibility-owned by:
* 
* TypeConstraints
* 
* with canonical implementation in:
* 
* TypeBounds
* 
* Advanced computation may generate semantic obligations but does not parse
* them independently.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH AST
* ============================================================================
* 
* The frontend must consume the delegated parse tree through the existing
* canonical AST conversion path.
* 
* Required destination categories include:
* 
* TypeExpr
* TypeValueExpr
* declaration nodes
* semantic metadata
* 
* No advanced-only AST is permitted.
* 
* If a semantic feature cannot currently be represented by the canonical AST,
* that AST change must be completed as a coordinated repository change before
* that feature is marked stable.
* 
* The grammar must not silently discard syntax to compensate for missing AST
* support.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH SEMANTICS
* ============================================================================
* 
* The semantic pipeline is:
* 
* parse
*   |
*   v
* canonical AST
*   |
*   v
* name resolution
*   |
*   v
* generic/type resolution
*   |
*   v
* kind analysis
*   |
*   v
* type-family resolution
*   |
*   v
* dependent-type analysis
*   |
*   v
* constraint generation
*   |
*   v
* constraint solving
*   |
*   v
* normalization
*   |
*   v
* specialization
*   |
*   v
* canonical semantic type model
* 
* ============================================================================
* REQUIRED INTEGRATION WITH IR
* ============================================================================
* 
* This grammar emits NO IR.
* 
* The semantic result flows into the existing canonical IR architecture.
* 
* When a resulting semantic type participates in quantum computation:
* 
* semantic quantum model
*         |
*         v
*     quantum::ir
* 
* No:
* 
* AdvancedTypeIR
* 
* or:
* 
* TypeLevelIR
* 
* may be introduced.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH COMPILER
* ============================================================================
* 
* Compiler consumers may use normalized type information for:
* 
* specialization
* monomorphization
* optimization
* layout
* vectorization
* tensor lowering
* quantum lowering
* hardware mapping
* resource planning
* distributed execution
* 
* These are downstream consumers.
* 
* This grammar does not select any implementation.
* 
* ============================================================================
* REQUIRED INTEGRATION WITH RUNTIME
* ============================================================================
* 
* Runtime code MUST NOT execute this grammar.
* 
* If compile-time type information is retained as runtime metadata, that
* metadata must be explicitly represented by the semantic/compiler/runtime
* metadata systems.
* 
* Type-level computation must not silently become runtime reflection.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Required positive tests:
* 
* advanced type-level expression
* type-level application
* type-level normalization
* type-level predicate
* type-level value
* type-level result
* generic interaction
* family interaction
* HKT interaction
* dependent-type interaction
* associated-type interaction
* nested type-level computation
* quantum type interaction
* HDL/hardware-neutral type interaction
* tensor/data type interaction
* distributed type interaction
* 
* Required negative tests:
* 
* malformed delegated expression
* malformed type family
* malformed HKT
* malformed dependent type
* invalid generic syntax
* invalid type-level argument syntax
* incomplete normalization expression
* incomplete application expression
* 
* Required semantic tests:
* 
* unresolved type
* unresolved family
* kind mismatch
* invalid family equation
* failed normalization
* failed constraint
* invalid dependent substitution
* non-terminating type computation
* ambiguous resolution
* 
* Required boundary tests:
* 
* generic + HKT
* HKT + family
* family + dependent type
* dependent + type-level value
* associated + generic
* quantum + dependent
* tensor + dependent
* HDL + symbolic dimension
* distributed + resource type
* 
* Required scalability tests:
* 
* increasing generic arity
* increasing family arity
* increasing kind structure
* increasing dependent nesting
* increasing type-level expression size
* increasing symbolic dimension complexity
* 
* Test generation may use configurable resource budgets.
* 
* Those test budgets are not language limits.
* 
* ============================================================================
* DETERMINISM TEST CONTRACT
* ============================================================================
* 
* Identical source/configuration must produce identical parse structure.
* 
* Tests must verify that parsing is independent of:
* 
* target hardware
* CPU count
* GPU count
* QPU count
* memory capacity
* network state
* filesystem state
* runtime state
* wall-clock time
* randomness
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing valid constructs must retain their meaning.
* 
* In particular, this file must not alter the established syntax or meaning
* of:
* 
* T
* T<U>
* T::Item
* T: Bound
* Pi x : A . B
* Sigma x : A . B
* F : * -> *
* type family F<T> : *
* type family instance F<A> = B;
* 
* Compatibility changes must go through the repository compatibility/spec
* system.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file contains:
* 
* no physical capacity constants;
* no hardware constants;
* no target identifiers;
* no finite domain catalogue;
* no fixed generic arity;
* no fixed kind arity;
* no fixed dependent-type depth;
* no fixed tensor rank;
* no fixed quantum cardinality;
* no fixed distributed-node count.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is DONE when:
* 
* [x] It has exactly one composition purpose.
* [x] It does not create a second type system.
* [x] It does not create a second type-level language.
* [x] It does not redefine generic syntax.
* [x] It does not redefine family syntax.
* [x] It does not redefine HKT syntax.
* [x] It does not redefine dependent syntax.
* [x] It does not redefine type-level expression syntax.
* [x] It does not define lexer rules.
* [x] It uses canonical Zamani tokens indirectly through delegates.
* [x] It introduces no semantic predicates.
* [x] It introduces no embedded Rust.
* [x] It introduces no unsafe implementation requirement.
* [x] It introduces no hardware/resource limits.
* [x] It introduces no target-specific semantics.
* [x] It introduces no second AST.
* [x] It introduces no second IR.
* [x] It preserves the canonical TypeExpr boundary.
* [x] It preserves TypeValueExpr semantics.
* [x] It preserves quantum::ir as the quantum semantic IR boundary.
* [x] It remains compatible with Rust 1.97+ safe Rust.
* [x] It has positive parser coverage.
* [x] It has negative parser coverage.
* [x] It has semantic coverage.
* [x] It has boundary coverage.
* [x] It has scalability coverage.
* [x] It has determinism coverage.
* [x] It has compatibility coverage.
* 
* ============================================================================
* IMPORTANT INTEGRATION NOTE
* ============================================================================
* 
* This file intentionally references rules owned by sibling delegate grammars.
* 
* ANTLR grammar imports compose delegate grammars into the final grammar.
* The canonical root must therefore ensure that the required delegate
* grammars are imported exactly once in the composition hierarchy.
* 
* The production composition should avoid importing the same delegate through
* multiple competing paths when that creates duplicate rule ownership.
* 
* Recommended topology:
* 
* Zamani.g4
*     |
*     v
* ZamaniParser.g4
*     |
*     v
* Type
*     |
*     +--> AdvancedTypeLevelComputation
*     +--> Generic
*     +--> TypeFamilies
*     +--> HigherKindedTypes
*     +--> Dependent
*     +--> TypeConstraints
*     +--> TypeLevelMetaprogramming
* 
* If the repository's existing parser hierarchy already imports any of these
* delegates, the integration owner must consolidate the import path rather
* than importing the same delegate twice.
* 
* ============================================================================
* NO LEXER CHANGES REQUIRED BY THIS FILE
* ============================================================================
* 
* This file introduces no new source keyword.
* 
* Existing lexical ownership remains unchanged.
* 
* In particular, advanced type-level computation is deliberately expressed
* through already established constructs rather than adding a new universal
* keyword.
* 
* This preserves the open-world design of Zamani.
* 
* ============================================================================
* FINAL ARCHITECTURAL GUARANTEE
* ============================================================================
* 
* This file establishes:
* 
* ONE advanced type-level composition boundary
* 
* while preserving:
* 
* ONE source-level type language
* ONE canonical TypeExpr hierarchy
* ONE semantic type system
* ONE canonical semantic IR architecture
* ONE quantum semantic boundary: quantum::ir
* 
* The result can scale from:
* 
* symbolic scalar types
* 
* through:
* 
* generic systems
* dependent structures
* tensor systems
* quantum programs
* HDL/hardware descriptions
* distributed systems
* future computational domains
* 
* without embedding today's machine size into the language.
* 
* ============================================================================
* END OF FILE
* ============================================================================
  */

parser grammar AdvancedTypeLevelComputation;

options {
tokenVocab = ZamaniLexer;
}

/* ============================================================================

* PUBLIC COMPOSITION ENTRY
* ========================================================================== */

advancedTypeLevelComputation
: advancedTypeLevelDeclaration
| advancedTypeLevelExpression
;

/* ============================================================================

* DECLARATION BOUNDARY
* ========================================================================== */

advancedTypeLevelDeclaration
: typeLevelDeclaration
;

/* ============================================================================

* EXPRESSION BOUNDARY
* ========================================================================== */

advancedTypeLevelExpression
: advancedTypeLevelApplicationResult
| advancedTypeLevelTypeResult
| advancedTypeLevelValueResult
| advancedTypeLevelPredicateResult
| advancedTypeLevelFamilyResult
| advancedTypeLevelKindResult
| advancedTypeLevelDependentResult
;

/* ============================================================================

* APPLICATION
* ========================================================================== */

advancedTypeLevelApplicationResult
: typeLevelApplicationExpression
;

/* ============================================================================

* TYPE RESULT
* ========================================================================== */

advancedTypeLevelTypeResult
: typeLevelTypeResult
;

/* ============================================================================

* VALUE RESULT
* ========================================================================== */

advancedTypeLevelValueResult
: typeLevelValueResult
;

/* ============================================================================

* PREDICATE RESULT
* ========================================================================== */

advancedTypeLevelPredicateResult
: typeLevelPredicateExpression
;

/* ============================================================================

* TYPE-FAMILY RESULT
* ========================================================================== */

advancedTypeLevelFamilyResult
: typeFamilyEquation
;

/* ============================================================================

* HIGHER-KINDED RESULT
* ========================================================================== */

advancedTypeLevelKindResult
: higherKindedTypeReference
;

/* ============================================================================

* DEPENDENT RESULT
* ========================================================================== */

advancedTypeLevelDependentResult
: dependentType
;