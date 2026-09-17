/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/types/affine.g4
* 
* Status:
* Production-ready modular affine-type parser component.
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Compiler baseline:
* Rust 1.97 / Rust 1.97.1
* 
* Rust edition:
* 2021
* 
* Safety:
* Safe Rust only.
* No unsafe Rust is required or permitted by the Zamani architecture.
* 
* ============================================================================
* 
* PURPOSE
* ============================================================================
* 
* This file owns the SOURCE SYNTAX for affine type qualification.
* 
* An affine type expresses the source-level ownership/resource property:
* 
* a value may be used at most once.
* 
* The exact ownership semantics are NOT implemented by this grammar.
* 
* This grammar records only the programmer's syntactic intent:
* 
* affine T
* 
* Semantic analysis determines whether:
* 
* - T is a valid type;
* - affine qualification is legal in that context;
* - a value of the resulting type may be copied;
* - a value may be dropped without use;
* - a value may be moved;
* - a use consumes the value;
* - ownership obligations are satisfied;
* - affine constraints interact correctly with generics;
* - affine constraints interact correctly with quantum resources;
* - affine constraints interact correctly with memory/resources;
* - affine constraints survive lowering.
* 
* ============================================================================
* 
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* parser
*      |
*      +----------------------+
*      |                      |
*      v                      v
* types/types.g4        memory/ownership.g4
*      |                      |
*      +----------+-----------+
*                 |
*                 v
*         affine.g4
*                 |
*                 v
*         frontend TypeExpr
*                 |
*                 v
*      structural validation
*                 |
*                 v
*         semantic analysis
*                 |
*                 v
*      ownership/resource model
*                 |
*                 v
*          canonical semantic IR
*                 |
*      +----------+-----------+
*      |          |           |
*      v          v           v
*  classical   quantum      HDL/
*     IR       quantum::ir   resource
*      |          |           |
*      +----------+-----------+
*                 |
*                 v
*   optimization / lowering
*                 |
*      +----------+-----------+
*      |          |           |
*      v          v           v
*   routing   scheduling   resilience
*                              |
*                              v
*                            ZQN
*                              |
*                              v
*                             HAL
*                              |
*                              v
*                     target realization
* 
* ============================================================================
* 
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - the "affine" type qualifier;
* - the affine type production;
* - the syntactic operand of an affine type;
* - affine-type composition hooks;
* - source-level affine qualification structure.
* 
* THIS FILE DOES NOT OWN:
* 
* - the complete type-expression grammar;
* - identifiers;
* - keywords generally;
* - operators generally;
* - punctuation generally;
* - generic types;
* - arrays;
* - tuples;
* - references;
* - pointers;
* - function types;
* - quantum types;
* - hardware types;
* - resource discovery;
* - resource allocation;
* - ownership checking;
* - borrow checking;
* - lifetime checking;
* - move analysis;
* - copy analysis;
* - drop analysis;
* - runtime memory management;
* - hardware selection;
* - physical memory placement;
* - physical qubit allocation;
* - routing;
* - scheduling;
* - calibration;
* - optimization;
* - QEC;
* - ZQN;
* - HAL;
* - backend selection;
* - ABI layout.
* 
* ============================================================================
* 
* CANONICAL SEMANTIC MEANING
* ============================================================================
* 
* The source form:
* 
* affine T
* 
* denotes:
* 
* Affine(T)
* 
* at the source-type level.
* 
* It does NOT mean:
* 
* allocate T
* place T in special memory
* use a particular CPU register
* use a particular memory bank
* use a particular accelerator
* use a particular physical qubit
* 
* The resulting source AST is expected to map to:
* 
* affineType
*     ->
* TypeExpr::Affine
* 
* with "T" represented by the existing canonical "TypeExpr".
* 
* The frontend AST already provides an "Affine" type-expression variant and
* an "affine(inner: TypeExpr)" constructor. This grammar therefore introduces
* no new AST concept.
* 
* ============================================================================
* 
* POCO-REAF / SCALABILITY CONTRACT
* ============================================================================
* 
* Affine typing is independent of machine scale.
* 
* This grammar MUST NOT encode any implementation limit such as:
* 
* MAX_VALUES
* MAX_AFFINE_VALUES
* MAX_TYPES
* MAX_GENERIC_PARAMETERS
* MAX_NESTING
* MAX_MEMORY
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_QPUS
* MAX_QUBITS
* MAX_NODES
* MAX_DEVICES
* MAX_RESOURCE_COUNT
* 
* In particular, the grammar MUST NOT contain finite alternatives such as:
* 
* affineType
*     : affineInt
*     | affineFloat
*     | affineQubit
*     | ...
* 
* because that would create a closed list of affine-capable types.
* 
* Instead:
* 
* affine T
* 
* works for any syntactically valid source type "T" accepted by the canonical
* type composition grammar.
* 
* This permits:
* 
* affine Int
* affine UserType
* affine Resource<Qubit>
* affine Qubit
* affine QRegister<N>
* affine Tensor<T, N>
* affine Foo::Bar
* affine FutureDomainType
* 
* subject to semantic validation.
* 
* The same language construct therefore scales from tiny systems to systems
* whose size is determined only by available implementation resources.
* 
* ============================================================================
* 
* IMPORTANT COMPOSITION RULE
* ============================================================================
* 
* This file MUST NOT define:
* 
* typeExpression
* typePrimary
* primitiveType
* genericType
* arrayType
* tupleType
* referenceType
* pointerType
* quantumType
* resourceType
* 
* Those are owned by the canonical type composition grammar:
* 
* grammar/types/types.g4
* 
* This file provides the specialized:
* 
* affineType
* 
* production consumed by that composition layer.
* 
* ============================================================================
* 
* ANTLR COMPOSITION CONTRACT
* ============================================================================
* 
* "affine.g4" is deliberately a parser grammar.
* 
* It is intended to be imported by the canonical type grammar:
* 
* grammar/types/types.g4
* 
* The composition layer is responsible for providing the canonical
* "typeExpression" rule.
* 
* The conceptual integration is:
* 
* typePrimary
*     :
*       ...
*     | affineType
*     | ...
*     ;
* 
* The canonical type grammar MUST remain the sole owner of the public
* "typeExpression" entry point.
* 
* This file therefore does not define a competing root.
* 
* ============================================================================
* 
* LEXER CONTRACT
* ============================================================================
* 
* The canonical lexical authority is:
* 
* grammar/lexer/tokens.g4
* 
* and its composed lexer vocabulary.
* 
* This grammar does NOT define lexer rules.
* 
* The only dedicated lexical token required by this grammar is:
* 
* AFFINE
* 
* "AFFINE" is already part of the repository's canonical lexical vocabulary.
* The existing Rust lexer also contains the corresponding "KeywordAffine"
* token concept. The grammar therefore consumes the established token instead
* of introducing a second spelling or token.
* 
* The following MUST NOT be declared here:
* 
* AFFINE : 'affine' ;
* 
* IDENTIFIER : ... ;
* 
* or any other lexer rule.
* 
* ============================================================================
* 
* TOKEN / AST / SEMANTIC CHAIN
* ============================================================================
* 
* "affine"
*     |
*     v
* canonical lexer
*     |
*     v
* AFFINE
*     |
*     v
* affineType
*     |
*     v
* canonical TypeExpr
*     |
*     v
* TypeExpr::Affine
*     |
*     v
* semantic affine/ownership type
*     |
*     v
* canonical semantic IR
* 
* There must be no intermediate:
* 
* AffineTypeIR
* AffineQuantumIR
* AffineHardwareIR
* 
* created by this grammar.
* 
* ============================================================================
* 
* SOURCE SYNTAX
* ============================================================================
* 
* Canonical spelling:
* 
* affine T
* 
* Examples:
* 
* affine Int
* 
* affine User
* 
* affine Resource<Qubit>
* 
* affine Qubit
* 
* affine QRegister<N>
* 
* affine Vec<T>
* 
* affine module::Type
* 
* affine Foo<T, N>
* 
* The operand is a complete source-level type expression.
* 
* The canonical type composition layer owns the complete operand grammar.
* 
* ============================================================================
* 
* QUALIFIER COMPOSITION
* ============================================================================
* 
* Affine is a type qualifier, not a new type universe.
* 
* Therefore:
* 
* affine T
* 
* means:
* 
* Affine(T)
* 
* rather than:
* 
* NamedType("affine")
* 
* This distinction is important because "affine" is semantically meaningful
* only in a type-qualification position.
* 
* ============================================================================
* 
* AFFINE + GENERICS
* ============================================================================
* 
* Examples:
* 
* affine Vec<T>
* 
* affine Map<K, V>
* 
* affine Resource<Qubit>
* 
* affine Tensor<T, N, M>
* 
* Generic substitution and validation are semantic concerns.
* 
* This grammar does not determine whether the generic constructor:
* 
* Vec
* Map
* Resource
* Tensor
* 
* permits affine qualification.
* 
* ============================================================================
* 
* AFFINE + QUANTUM
* ============================================================================
* 
* Examples:
* 
* affine Qubit
* 
* affine QRegister<N>
* 
* affine QuantumState<T>
* 
* are syntactically valid when those names are valid source-level types.
* 
* Semantic analysis determines whether affine ownership is legal for the
* corresponding quantum abstraction.
* 
* This grammar does NOT:
* 
* - allocate qubits;
* - select physical qubits;
* - assign QPU devices;
* - inspect coupling maps;
* - perform routing;
* - perform scheduling;
* - perform QEC;
* - perform noise analysis;
* - invoke ZQN;
* - inspect HAL capabilities.
* 
* Quantum semantic lowering ultimately crosses the existing:
* 
* quantum::ir
* 
* boundary.
* 
* ============================================================================
* 
* AFFINE + HARDWARE
* ============================================================================
* 
* Hardware/resource types may be affine-qualified when their source-level
* grammar permits it.
* 
* Examples:
* 
* affine Accelerator<A>
* 
* affine Resource<R>
* 
* affine Memory<T, N>
* 
* Whether a resource may be consumed at most once is a semantic property.
* 
* The grammar does not determine whether a target actually has the resource.
* 
* ============================================================================
* 
* AFFINE + DEPENDENT / SYMBOLIC TYPES
* ============================================================================
* 
* Affine qualification must work with symbolic type arguments:
* 
* affine Vector<T, N>
* 
* affine Matrix<T, Rows, Cols>
* 
* affine Tensor<T, Shape>
* 
* where "N", "Rows", "Cols", or "Shape" may remain symbolic until semantic
* analysis and compilation.
* 
* No machine-sized integer conversion is performed by this grammar.
* 
* ============================================================================
* 
* AFFINE + REFERENCES / POINTERS
* ============================================================================
* 
* Examples such as:
* 
* affine &T
* 
* affine *T
* 
* are syntactically delegated to the canonical reference/pointer type
* productions when those forms are legal in the language.
* 
* The grammar does not decide whether affine ownership is compatible with:
* 
* borrowing;
* mutable references;
* shared references;
* raw pointers;
* address spaces;
* lifetimes.
* 
* Those decisions belong to semantic analysis.
* 
* ============================================================================
* 
* NESTED QUALIFIERS
* ============================================================================
* 
* This file deliberately owns only the affine constructor.
* 
* It does not define arbitrary qualifier nesting such as:
* 
* affine linear T
* 
* or:
* 
* linear affine T
* 
* as an independent grammar.
* 
* If Zamani permits multiple ownership/type qualifiers, their ordering and
* compatibility must be defined once by the canonical type composition layer.
* 
* That prevents each qualifier file from creating a competing qualifier
* language.
* 
* ============================================================================
* 
* AFFINE VS LINEAR
* ============================================================================
* 
* Affine and linear are related but distinct:
* 
* linear T
* 
* generally expresses:
* 
* T must be used exactly once.
* 
* affine T
* 
* generally expresses:
* 
* T may be used at most once.
* 
* This grammar does not encode those semantic usage rules.
* 
* The semantic layer is responsible for distinguishing:
* 
* exactly once
* 
* from:
* 
* zero or one time.
* 
* ============================================================================
* 
* RELATIONSHIP TO grammar/memory/ownership.g4
* ============================================================================
* 
* "grammar/memory/ownership.g4" already identifies:
* 
* linear
* affine
* 
* as foundational ownership modes.
* 
* That grammar owns ownership-domain syntax such as ownership qualifiers and
* ownership operations.
* 
* This file owns the TYPE-SYSTEM spelling:
* 
* affine T
* 
* The two files must not be merged into one grammar because they serve
* different composition boundaries:
* 
* types/affine.g4
*     -> type syntax
* 
* memory/ownership.g4
*     -> ownership-domain syntax
* 
* Both ultimately feed the same semantic ownership/type model.
* 
* ============================================================================
* 
* RELATIONSHIP TO src/frontend/ast/node/types/type_expr.rs
* ============================================================================
* 
* The canonical frontend AST already provides:
* 
* TypeExpr::Affine
* 
* and an affine constructor.
* 
* Therefore:
* 
* affineType
* 
* MUST map to:
* 
* TypeExpr::Affine(inner)
* 
* where "inner" is the existing canonical "TypeExpr" for the operand.
* 
* This file must not require a new:
* 
* AffineType
* 
* AST node.
* 
* ============================================================================
* 
* RELATIONSHIP TO LEGACY src/ast/mod.rs
* ============================================================================
* 
* The legacy AST also contains an "Affine" type-expression variant.
* 
* That compatibility representation must not become a second semantic
* authority.
* 
* The migration direction is:
* 
* parser
*   ->
* frontend AST TypeExpr
*   ->
* semantic type system
* 
* Legacy AST compatibility adapters may translate as required, but this
* grammar does not depend on the legacy AST.
* 
* ============================================================================
* 
* RELATIONSHIP TO src/compiler_types.rs
* ============================================================================
* 
* The repository's compiler type model already contains:
* 
* Affine(...)
* 
* This grammar does not depend on that implementation representation.
* 
* The compiler type system consumes the semantic result after validation.
* 
* Therefore the grammar remains stable if the compiler's internal
* representation changes.
* 
* ============================================================================
* 
* RELATIONSHIP TO src/semantic.rs
* ============================================================================
* 
* The repository already tracks linear/affine usage information.
* 
* This grammar must not duplicate that tracking.
* 
* Semantic analysis is responsible for:
* 
* - counting uses;
* - detecting illegal duplication;
* - detecting illegal repeated consumption;
* - validating legal dropping;
* - propagating affine constraints;
* - checking control-flow joins;
* - checking loops;
* - checking closures;
* - checking async boundaries;
* - checking concurrency boundaries;
* - checking FFI boundaries;
* - checking domain-specific resource usage.
* 
* The grammar supplies only the source-level type constructor.
* 
* ============================================================================
* 
* RELATIONSHIP TO RUNTIME MEMORY MANAGEMENT
* ============================================================================
* 
* The runtime may have allocation strategies for affine resources, but
* "affine.g4" must never imply a particular allocator.
* 
* For example, this grammar does not imply:
* 
* stack allocation;
* heap allocation;
* reference counting;
* garbage collection;
* linear arena;
* physical memory placement.
* 
* Runtime policy remains downstream.
* 
* ============================================================================
* 
* FFI / INTEROPERABILITY
* ============================================================================
* 
* Affine types crossing FFI boundaries require semantic ownership contracts.
* 
* The grammar remains unchanged.
* 
* Foreign ABI rules belong to:
* 
* grammar/interoperability/
* 
* and the compiler/semantic layers.
* 
* This keeps affine syntax portable across:
* 
* C
* C++
* Rust
* Python
* WebAssembly
* QIR
* OpenQASM
* HDL
* future foreign interfaces.
* 
* ============================================================================
* 
* ERROR / DIAGNOSTIC CONTRACT
* ============================================================================
* 
* This grammar should permit the parser to identify the source span of:
* 
* affine
* 
* and the complete operand type.
* 
* Semantic diagnostics should be able to report:
* 
* - invalid affine target;
* - illegal qualifier combination;
* - invalid ownership use;
* - duplicated affine use;
* - illegal drop;
* - incompatible generic instantiation;
* - incompatible FFI boundary;
* - incompatible domain operation.
* 
* The grammar itself should not manufacture semantic diagnostics.
* 
* ============================================================================
* 
* SOURCE SPANS
* ============================================================================
* 
* The parser/frontend integration must preserve:
* 
* start span = `AFFINE`
* end span   = end of the operand type
* 
* so a semantic diagnostic can highlight the complete affine type.
* 
* The grammar introduces no custom source-location mechanism.
* 
* Source locations remain owned by the parser/frontend infrastructure.
* 
* ============================================================================
* 
* DETERMINISM
* ============================================================================
* 
* This grammar has one public affine production.
* 
* There are no unordered alternatives based on:
* 
* hardware;
* runtime;
* resource availability;
* target discovery;
* vendor identity.
* 
* Parsing the same source under the same grammar version produces the same
* syntactic structure.
* 
* ============================================================================
* 
* SECURITY
* ============================================================================
* 
* This grammar:
* 
* - performs no I/O;
* - executes no code;
* - accesses no hardware;
* - allocates no resources;
* - performs no network operations;
* - evaluates no type-level computation;
* - invokes no compiler backend.
* 
* Resource-exhaustion protection belongs to parser/compilation policy rather
* than language meaning.
* 
* ============================================================================
* 
* PERFORMANCE
* ============================================================================
* 
* The production is intentionally simple:
* 
* AFFINE affineTypeOperand
* 
* It does not enumerate all possible type categories.
* 
* This keeps affine recognition independent from the number of:
* 
* types;
* domains;
* hardware targets;
* quantum operations;
* resource kinds;
* dialects.
* 
* ============================================================================
* 
* COMPATIBILITY
* ============================================================================
* 
* Existing source spelling:
* 
* affine T
* 
* remains supported.
* 
* The token:
* 
* AFFINE
* 
* remains the canonical lexical representation.
* 
* No legacy filename is renamed.
* 
* No competing affine grammar is introduced.
* 
* ============================================================================
* 
* HARD-CODING AUDIT
* ============================================================================
* 
* This file contains no:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_ASICS
* MAX_QPUS
* MAX_NODES
* MAX_MEMORY
* MAX_REGISTER_WIDTH
* MAX_TENSOR_RANK
* MAX_TENSOR_DIMENSION
* MAX_ARRAY_LENGTH
* MAX_GENERIC_ARITY
* MAX_TUPLE_ARITY
* MAX_AFFINE_VALUES
* MAX_AFFINE_DEPTH
* 
* It therefore imposes no target-size restriction.
* 
* ============================================================================
* 
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when all of the following are true:
* 
* [x] AFFINE is consumed from the canonical lexer vocabulary.
* 
* [x] No lexer rules are duplicated here.
* 
* [x] "affineType" is the sole public rule owned by this component.
* 
* [x] The complete operand is delegated to the canonical type-expression
*   composition layer.
* 
* [x] No second TypeExpr is introduced.
* 
* [x] The intended AST mapping is TypeExpr::Affine.
* 
* [x] Semantic ownership analysis remains downstream.
* 
* [x] Resource allocation remains downstream.
* 
* [x] Quantum physical realization remains downstream.
* 
* [x] No hardware limits are encoded.
* 
* [x] No finite type inventory is encoded.
* 
* [x] No runtime behavior is encoded.
* 
* [x] No unsafe implementation dependency exists.
* 
* [x] Rust 1.97 / 1.97.1 compatibility is preserved.
* 
* [x] Existing filenames are preserved.
* 
* [x] The grammar can be composed into "types/types.g4".
* 
* [x] The canonical root grammar can delegate to the type composition
*   layer without copying this rule.
* 
* ============================================================================
* 
* TEST CONTRACT
* ============================================================================
* 
* The owning test suite should include at least:
* 
* POSITIVE:
* 
* affine Int
* affine Float
* affine User
* affine module::Type
* affine Vec<Int>
* affine Map<Key, Value>
* affine Qubit
* affine QRegister<N>
* affine Resource<Qubit>
* affine Tensor<T, N>
* 
* NEGATIVE:
* 
* affine
* affine <
* affine >
* affine ,
* 
* and malformed operands rejected by the canonical type grammar.
* 
* BOUNDARY:
* 
* affine T
* affine T<U>
* affine T<U, V, W, ...>
* affine module::nested::Type
* affine Array<T, symbolic_dimension>
* 
* SCALABILITY:
* 
* affine T
* affine Generic<T, N>
* affine Resource<Capability<C>>
* affine QRegister<N>
* affine Tensor<T, Shape>
* 
* where symbolic quantities remain symbolic and are not converted to
* hard-coded machine limits.
* 
* COMPATIBILITY:
* 
* existing affine source syntax;
* canonical AFFINE token;
* frontend TypeExpr::Affine;
* semantic affine ownership checking.
* 
* DETERMINISM:
* 
* The same source and grammar version must produce the same parse structure.
* 
* ============================================================================
  */

parser grammar Affine;

options {
tokenVocab = ZamaniLexer;
}

/* ============================================================================

* PUBLIC RULE
* ========================================================================== */

/**

* Canonical affine type constructor.
* 
* Source form:
* 
* affine T
* 
* The operand is the canonical Zamani type-expression grammar owned by the
* type-composition layer.
* 
* "typeExpression" is deliberately referenced rather than recreated here.
* 
* This prevents affine.g4 from becoming a second type system.
  */
  affineType
  : AFFINE typeExpression
  ;
  :::end