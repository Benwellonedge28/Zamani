/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/types/constraints.g4
* 
* Grammar:
* TypeConstraints
* 
* Status:
* Production parser delegate.
* 
* Purpose:
* Canonical source-level grammar for constraints that qualify a type
* parameter or type-level subject.
* 
* This file owns ONLY the reusable syntax of a type-constraint clause.
* It does not own:
* 
*     - the complete type system;
*     - generic parameter declarations;
*     - where-clause declarations;
*     - arbitrary value constraints;
*     - resource constraints;
*     - capability negotiation;
*     - contracts;
*     - policies;
*     - type inference;
*     - constraint solving;
*     - target selection;
*     - hardware selection;
*     - quantum routing;
*     - scheduling;
*     - lowering;
*     - runtime execution.
* 
* Rust baseline:
* Rust 1.97 / Rust 1.97.1
* 
* Edition:
* Rust 2021
* 
* Safety:
* Declarative ANTLR parser grammar.
* No embedded Rust.
* No actions.
* No semantic predicates.
* No unsafe code.
* No I/O.
* No runtime execution.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* parser composition
*      |
*      +----------------------------+
*      |                            |
*      v                            v
* Types.typeExpression       this TypeConstraints grammar
*      |                            |
*      +-------------+--------------+
*                    |
*                    v
*            domain-neutral AST
*                    |
*                    v
*            structural validation
*                    |
*                    v
*            semantic type system
*                    |
*         +----------+----------+
*         |          |          |
*         v          v          v
*      inference   solving   specialization
*                    |
*                    v
*             canonical semantic
*                   model
*                    |
*         +----------+----------+
*         |          |          |
*         v          v          v
*     classical   quantum::ir  HDL/hardware
*                    |
*                    v
*              target-independent
*                compilation
*                    |
*                    v
*              target realization
* 
* This grammar remains completely upstream of physical execution.
* 
* ============================================================================
* OWNERSHIP CONTRACT
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - typeConstraintClause;
* - typeConstraintBoundList;
* - typeConstraintBound;
* - the ':' separator associated with a type bound;
* - the '+' conjunction between type bounds;
* - source ordering of type bounds.
* 
* THIS FILE DOES NOT OWN:
* 
* - identifier syntax;
* - qualified-name syntax;
* - typeExpression;
* - genericParameter;
* - genericParameterList;
* - generic declarations;
* - where;
* - whereClause;
* - arbitrary constraint expressions;
* - boolean expressions;
* - arithmetic expressions;
* - resource requirements;
* - capability requirements;
* - contracts;
* - policies;
* - type inference;
* - type substitution;
* - type checking;
* - trait/interface resolution;
* - associated-type resolution;
* - dependent-value evaluation;
* - specialization;
* - monomorphization;
* - target discovery;
* - hardware discovery;
* - allocation;
* - placement;
* - routing;
* - scheduling;
* - optimization;
* - quantum operation semantics;
* - QEC;
* - ZQN;
* - HAL;
* - runtime behavior.
* 
* ============================================================================
* SINGLE TYPE-EXPRESSION AUTHORITY
* ============================================================================
* 
* The canonical type-expression grammar is:
* 
* grammar/types/types.g4
* 
* This file consumes:
* 
* typeExpression
* 
* It MUST NOT redefine any type-expression production.
* 
* In particular, this file MUST NOT define:
* 
* typeExpression
* typeCore
* typeAtom
* namedType
* genericType
* tupleType
* arrayType
* sliceType
* functionType
* referenceType
* pointerType
* resultType
* quantumType
* dependentType
* temporalType
* 
* This guarantees that every type bound has exactly the same source-level
* type syntax as every other type position in Zamani.
* 
* ============================================================================
* LEXER AUTHORITY
* ============================================================================
* 
* The canonical lexical composition is:
* 
* grammar/lexer/tokens.g4
* 
* whose lexer grammar is:
* 
* ZamaniTokens
* 
* The repository also contains:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* which is the parser-facing lexer composition/baseline used by the current
* parser grammar stack.
* 
* Parser delegates must converge on the repository's canonical parser-facing
* lexer vocabulary. This file therefore uses:
* 
* tokenVocab = ZamaniLexer;
* 
* No lexer rule is declared here.
* 
* Required tokens:
* 
* COLON
* PLUS
* 
* No token is invented locally.
* 
* ============================================================================
* WHY THIS FILE EXISTS
* ============================================================================
* 
* A type bound is intentionally narrower than a general constraint.
* 
* Example:
* 
* T: Numeric + Comparable
* 
* means that the type parameter T is qualified by two type-level bounds.
* 
* It does NOT mean:
* 
* value >= minimum
* 
* or:
* 
* memory >= required
* 
* or:
* 
* capability("quantum.measurement")
* 
* Those belong to other constraint systems.
* 
* This separation is essential for the universal Zamani architecture.
* 
* ============================================================================
* GENERIC BOUND MODEL
* ============================================================================
* 
* Canonical examples:
* 
* <T: Numeric>
* 
* <T: Numeric + Comparable>
* 
* <T: quantum::State>
* 
* <T: quantum::State + quantum::Measurable>
* 
* <T: collections::Iterable<Value>>
* 
* The generic parameter declaration is owned by the appropriate generic
* declaration grammar.
* 
* This file supplies only:
* 
* :
* bound
* +
* bound
* 
* ============================================================================
* TYPE BOUND SEMANTIC NEUTRALITY
* ============================================================================
* 
* A type bound is syntactically a type expression.
* 
* Its semantic interpretation is deliberately open-ended.
* 
* A bound may eventually denote:
* 
* trait satisfaction
* interface satisfaction
* subtype relation
* structural property
* associated-type requirement
* capability-like type property
* domain type property
* formal type predicate
* future type-system relation
* 
* This grammar does not decide which interpretation applies.
* 
* That decision belongs to semantic analysis.
* 
* ============================================================================
* OPEN-WORLD DOMAIN CONTRACT
* ============================================================================
* 
* This file MUST NOT enumerate domain-specific bound names.
* 
* Do NOT add alternatives such as:
* 
* Numeric
* Comparable
* Iterable
* QuantumState
* LogicalQubit
* GPU
* FPGA
* Accelerator
* Tensor
* Dataset
* NetworkEndpoint
* HardwareModule
* 
* Those are represented by the canonical typeExpression grammar.
* 
* Consequently all of the following remain syntactically possible without
* changing this file:
* 
* T: Numeric
* 
* T: quantum::State
* 
* T: hardware::Accelerator
* 
* T: hdl::Module
* 
* T: ai::Model
* 
* T: data::Dataset
* 
* T: networking::Endpoint
* 
* T: future::computing::Capability
* 
* This is required for open-world extensibility.
* 
* ============================================================================
* POCO-REAF CONTRACT
* ============================================================================
* 
* Type constraints describe source-level type intent.
* 
* They MUST NOT encode a finite universe of physical machines.
* 
* This file therefore contains no:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* MAX_GENERIC_ARITY
* MAX_BOUND_COUNT
* MAX_BOUND_DEPTH
* 
* A type may describe an arbitrarily large logical structure.
* 
* Whether a particular target can realize that structure is determined
* downstream through resource/capability analysis.
* 
* Therefore:
* 
* type constraint
*      !=
* resource allocation
* 
* type constraint
*      !=
* target selection
* 
* type constraint
*      !=
* physical topology
* 
* ============================================================================
* RESOURCE / CAPABILITY SEPARATION
* ============================================================================
* 
* Example:
* 
* T: quantum::LogicalState
* 
* is a type-level condition.
* 
* A separate resource requirement may state:
* 
* requires qubits >= n
* 
* and a capability requirement may state:
* 
* requires capability("quantum.measurement")
* 
* Those constructs are not parsed by this file.
* 
* Their eventual relationship is:
* 
* source type
*      |
*      v
* type semantics
*      |
*      +----------------------+
*      |                      |
*      v                      v
* type satisfiability   resource/capability analysis
*                              |
*                              v
*                       target realization
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Quantum types are accepted because quantum types are part of the canonical
* type-expression system.
* 
* Examples:
* 
* T: quantum::State
* 
* T: quantum::Operation
* 
* T: quantum::Observable
* 
* T: quantum::LogicalQubit
* 
* T: quantum::Circuit
* 
* This file MUST NOT enumerate quantum operations or physical hardware.
* 
* It MUST NOT contain:
* 
* H
* X
* Y
* Z
* CNOT
* physical qubit IDs
* topology
* calibration
* routing
* scheduling
* QEC
* ZQN
* HAL
* 
* Quantum semantics remain downstream.
* 
* The canonical quantum boundary remains:
* 
* quantum::ir
* 
* ============================================================================
* CLASSICAL INTEGRATION
* ============================================================================
* 
* Classical type properties are ordinary type expressions.
* 
* Examples:
* 
* T: Numeric
* 
* T: Comparable
* 
* T: Iterable
* 
* T: Serializable
* 
* No classical operation catalogue is introduced here.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Hardware and HDL properties remain open-ended type expressions.
* 
* Examples:
* 
* T: hdl::Module
* 
* T: hardware::Signal
* 
* T: hardware::Memory
* 
* T: hardware::Accelerator
* 
* The grammar does not select an FPGA, ASIC, CPU, GPU, QPU, memory bank,
* device instance, or physical topology.
* 
* ============================================================================
* AI / DATA INTEGRATION
* ============================================================================
* 
* AI and data types are likewise ordinary type expressions.
* 
* Examples:
* 
* T: ai::Model
* 
* T: ai::Trainable
* 
* T: data::Dataset
* 
* T: data::Serializable
* 
* No application-specific keyword inventory is required.
* 
* ============================================================================
* DISTRIBUTED / NETWORKING INTEGRATION
* ============================================================================
* 
* Examples:
* 
* T: distributed::Message
* 
* T: distributed::Actor
* 
* T: networking::Endpoint
* 
* T: networking::Stream
* 
* These remain domain types.
* 
* Distribution and networking resource constraints remain owned by their
* respective grammar/semantic subsystems.
* 
* ============================================================================
* TYPE-LEVEL VALUE PARAMETERS
* ============================================================================
* 
* Dependent/value-parameterized types remain owned by Types.
* 
* Examples:
* 
* Matrix<T>[Rows, Cols]
* 
* Tensor<T>[N, M, K]
* 
* QRegister<N>
* 
* Buffer<T, N>
* 
* A type bound may therefore contain a complete canonical type expression
* whose semantics depend on symbolic values.
* 
* This grammar does not evaluate those values.
* 
* ============================================================================
* SOURCE-ORDER CONTRACT
* ============================================================================
* 
* For:
* 
* T: A + B + C
* 
* the parser preserves:
* 
* A
* B
* C
* 
* in source order.
* 
* This grammar performs no:
* 
* sorting
* deduplication
* normalization
* canonicalization
* resolution
* simplification
* evaluation
* 
* Those transformations belong to semantic analysis.
* 
* ============================================================================
* PUBLIC RULE CONTRACT
* ============================================================================
* 
* The public reusable entry point is:
* 
* typeConstraintClause
* 
* It consumes:
* 
* COLON typeConstraintBoundList
* 
* The bound list requires at least one bound.
* 
* ============================================================================
* TYPE CONSTRAINT CLAUSE
* ============================================================================
* 
* Canonical form:
* 
* : Numeric
* 
* : Numeric + Comparable
* 
* : quantum::State
* 
* : collections::Iterable<Value>
* 
* A caller that owns a generic parameter can compose:
* 
* genericParameter
*     : IDENTIFIER typeConstraintClause?
*     ;
* 
* The generic grammar remains the owner of that complete production.
* 
* ============================================================================
* ORDERED BOUND LIST
* ============================================================================
* 
* The list uses one or more type bounds:
* 
* A
* A + B
* A + B + C
* 
* There is deliberately no upper bound on the number of bounds encoded by
* this grammar.
* 
* Practical parser/compiler resource limits are implementation safeguards,
* not language semantics.
* 
* ============================================================================
* SINGLE TYPE BOUND
* ============================================================================
* 
* Every individual bound is exactly one:
* 
* typeExpression
* 
* This ensures:
* 
* T: A
* 
* and:
* 
* T: collections::Iterable<U>
* 
* use precisely the same type syntax and AST type representation as all other
* type positions.
* 
* ============================================================================
* WHERE INTEGRATION
* ============================================================================
* 
* This file does NOT own:
* 
* where
* 
* or:
* 
* whereClause
* 
* The declaration/function grammar that owns a where clause should compose:
* 
* WHERE
* typeExpression
* typeConstraintClause
* 
* conceptually as:
* 
* whereClause
*     : WHERE whereTypeConstraint (COMMA whereTypeConstraint)*
*     ;
* 
* and:
* 
* whereTypeConstraint
*     : typeExpression typeConstraintClause
*     ;
* 
* if that is the established surrounding syntax.
* 
* The complete where-clause belongs to the declaration/function/constraint
* owner because future where clauses may contain non-type predicates.
* 
* This avoids making this file the authority for general constraint syntax.
* 
* ============================================================================
* GENERAL CONSTRAINT BOUNDARY
* ============================================================================
* 
* General constraints belong to:
* 
* grammar/core/constraints.g4
* 
* Examples include:
* 
* where value >= minimum
* 
* requires memory >= required_memory
* 
* requires capability("tensor.compute")
* 
* topology(...)
* 
* Those must not be imported here merely to implement:
* 
* T: Numeric
* 
* Type constraints are one specific class of source-level constraint.
* 
* ============================================================================
* CONTRACT / POLICY BOUNDARY
* ============================================================================
* 
* Contracts such as:
* 
* requires
* ensures
* invariant
* assume
* guarantee
* property
* 
* are not type-bound syntax.
* 
* Policies such as:
* 
* prefer
* forbid
* allow
* fallback
* 
* are not type-bound syntax.
* 
* A semantic analysis phase may relate those systems to a type constraint,
* but the parser ownership remains separate.
* 
* ============================================================================
* CAPABILITY BOUNDARY
* ============================================================================
* 
* A capability may itself be represented as a type:
* 
* T: capability::Numeric
* 
* or:
* 
* T: quantum::Measurable
* 
* if the type system defines such types.
* 
* However:
* 
* requires capability("quantum.measurement")
* 
* is not a type bound and belongs to the resource/capability grammar.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The grammar must lower into the repository's existing frontend type
* representation.
* 
* Conceptually:
* 
* typeConstraintClause
*     |
*     v
* TypeBounds
*     |
*     +--> ordered collection of TypeExpr
* 
* Each:
* 
* typeConstraintBound
* 
* corresponds to exactly one:
* 
* TypeExpr
* 
* in source order.
* 
* No new closed domain-specific AST enum is introduced here.
* 
* Do NOT introduce:
* 
* QuantumBound
* HardwareBound
* TensorBound
* AIBound
* GPUbound
* 
* The AST must remain domain-neutral.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis consumes the parsed type expressions and determines:
* 
* - whether the subject is a valid type parameter/type-level subject;
* - whether every bound resolves;
* - whether the bounds are compatible;
* - whether required type properties are satisfied;
* - whether associated types are valid;
* - whether subtype/interface/trait relationships hold;
* - whether dependent type conditions are satisfiable.
* 
* The parser does none of these operations.
* 
* ============================================================================
* TYPE INFERENCE CONTRACT
* ============================================================================
* 
* Type inference may generate constraints that are semantically related to
* these source-level bounds.
* 
* This grammar does not own the compiler's inference representation.
* 
* Existing compiler-side inference structures remain authoritative for:
* 
* equality constraints
* subtype constraints
* unification
* substitution
* inferred bounds
* solving
* 
* The grammar is only the source syntax boundary.
* 
* ============================================================================
* SPECIALIZATION CONTRACT
* ============================================================================
* 
* Generic specialization and monomorphization are downstream.
* 
* A source program may therefore express:
* 
* T: SomeProperty
* 
* without knowing at source-parse time whether the eventual realization will
* be:
* 
* scalar
* vector
* tensor
* CPU
* GPU
* FPGA
* ASIC
* QPU
* simulator
* accelerator
* distributed
* heterogeneous
* 
* The constraint system determines semantic validity; target realization is
* a later concern.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* This grammar imposes no language-level maximum on:
* 
* number of bounds
* qualified-name depth
* generic nesting
* generic argument count
* type-expression complexity
* 
* It therefore supports source-level constructs whose eventual semantic
* realization can scale with available resources.
* 
* Example:
* 
* T:
*     A
*   + B
*   + C
*   + ...
* 
* is structurally represented by:
* 
* typeConstraintBound (PLUS typeConstraintBound)*
* 
* rather than a finite list of alternatives.
* 
* ============================================================================
* HOSTILE INPUT / RESOURCE SAFETY
* ============================================================================
* 
* "No language-level maximum" does NOT mean an implementation must permit
* unbounded physical resource consumption.
* 
* Parser implementations may enforce configurable operational safeguards for:
* 
* input size
* parser stack usage
* wall-clock budget
* allocation budget
* diagnostic count
* 
* Such safeguards MUST be:
* 
* implementation policy
* 
* and MUST NOT become grammar constants defining Zamani's expressive
* capacity.
* 
* This grammar itself declares none of those limits.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* no actions;
* no semantic predicates;
* no embedded code;
* no mutable global state;
* no randomness;
* no I/O;
* no environment access;
* no hardware discovery;
* no resource allocation;
* no runtime execution.
* 
* For a fixed token stream and fixed imported grammar vocabulary, parsing is
* deterministic.
* 
* ============================================================================
* SOURCE LOCATION CONTRACT
* ============================================================================
* 
* The frontend/parser integration must preserve source locations for:
* 
* the complete typeConstraintClause;
* the colon;
* each typeConstraintBound;
* each '+' separator;
* the underlying typeExpression.
* 
* This permits diagnostics such as:
* 
* unsatisfied type bound
* 
* to point at the precise source-level bound without requiring semantic logic
* inside this grammar.
* 
* ============================================================================
* DIAGNOSTIC CONTRACT
* ============================================================================
* 
* The grammar must reject structurally incomplete clauses:
* 
* :
* : +
* : A +
* : + A
* : A + +
* : A + B +
* 
* The grammar must accept syntactically valid unresolved types:
* 
* : UnknownType
* 
* : future::UnknownCapability
* 
* because unresolved names are semantic errors, not parser errors.
* 
* ============================================================================
* LEGACY MIGRATION CONTRACT
* ============================================================================
* 
* The repository currently contains:
* 
* grammar/types/type-constraints.g4
* 
* That file overlaps this responsibility.
* 
* The migration must establish exactly ONE canonical implementation.
* 
* Recommended migration:
* 
* grammar/types/constraints.g4
*         |
*         v
* canonical TypeConstraints implementation
* 
* and:
* 
* grammar/types/type-constraints.g4
*         |
*         v
* compatibility/deprecation boundary
* 
* The legacy file MUST NOT remain a second implementation of the same parser
* rules.
* 
* During migration, consumers of:
* 
* typeConstraintClause
* typeConstraintBoundList
* typeConstraintBound
* 
* should converge on this file.
* 
* Existing names such as:
* 
* genericBoundList
* typeBound
* 
* must not be reintroduced here.
* 
* ============================================================================
* INTEGRATION WITH TYPES.G4
* ============================================================================
* 
* grammar/types/types.g4 remains the sole owner of:
* 
* typeExpression
* 
* This file consumes that rule.
* 
* Import direction:
* 
* Types
*   |
*   +--> TypeConstraints
* 
* OR, where the ANTLR composition topology requires the reverse direction,
* the canonical parser root must ensure that the resulting composed grammar
* still contains exactly one definition of every public rule.
* 
* The critical invariant is:
* 
* exactly one typeExpression authority.
* 
* ============================================================================
* INTEGRATION WITH GENERIC DECLARATIONS
* ============================================================================
* 
* Generic declaration owners include, as applicable:
* 
* grammar/functions/generics.g4
* grammar/declarations/types.g4
* grammar/declarations/structs.g4
* grammar/declarations/implementations.g4
* 
* They own the generic parameter itself.
* 
* They consume:
* 
* typeConstraintClause
* 
* where a generic parameter permits an inline bound.
* 
* Example semantic structure:
* 
* genericParameter
*     |
*     +--> parameter name
*     |
*     +--> optional typeConstraintClause
* 
* This file does not own generic parameter syntax.
* 
* ============================================================================
* INTEGRATION WITH WHERE CLAUSES
* ============================================================================
* 
* Where-clause owners should consume this reusable suffix.
* 
* Example:
* 
* where T: Numeric + Comparable
* 
* is structurally:
* 
* WHERE
* typeExpression
* typeConstraintClause
* 
* The where-clause owner controls whether multiple predicates, commas,
* boolean predicates, contracts, or other source constructs are permitted.
* 
* ============================================================================
* INTEGRATION WITH AST BUILDERS
* ============================================================================
* 
* Existing frontend type AST infrastructure must remain authoritative.
* 
* Relevant repository components include:
* 
* src/frontend/ast/node/types/type_expr.rs
* 
* and:
* 
* src/frontend/ast/node/builders/type_builder.rs
* 
* The parser context generated from this file must be mapped into the
* existing type representation rather than introducing a second type AST.
* 
* A future builder may conceptually perform:
* 
* typeConstraintBound*
*     ->
* Vec<TypeExpr>
* 
* while preserving source order and source spans.
* 
* ============================================================================
* INTEGRATION WITH TYPE INFERENCE
* ============================================================================
* 
* Existing compiler type inference remains the semantic consumer.
* 
* Relevant repository component:
* 
* src/compiler/type_inference.rs
* 
* The parser MUST NOT attempt to construct or solve compiler-side:
* 
* Constraint
* 
* values.
* 
* The source grammar and compiler inference model intentionally remain
* separate layers.
* 
* ============================================================================
* INTEGRATION WITH QUANTUM
* ============================================================================
* 
* A quantum type appearing as a bound is still only a TypeExpr at this layer.
* 
* Example:
* 
* T: quantum::LogicalState
* 
* later participates in:
* 
* semantic type analysis
*      |
*      v
* quantum semantic model
*      |
*      v
* quantum::ir
* 
* No physical quantum information is introduced by this grammar.
* 
* ============================================================================
* INTEGRATION WITH HARDWARE
* ============================================================================
* 
* A hardware type appearing as a bound:
* 
* T: hardware::Accelerator
* 
* does not select:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* vendor
* device instance
* 
* Target realization remains downstream through resource/capability
* negotiation and compilation.
* 
* ============================================================================
* INTEGRATION WITH AI / REASONING / LEARNING
* ============================================================================
* 
* Type-level AI abstractions remain ordinary type expressions:
* 
* T: ai::Model
* 
* T: ai::Trainable
* 
* T: ai::Reasoner
* 
* T: ai::Adaptable
* 
* This file does not introduce AI-specific bound keywords.
* 
* Reasoning, learning, adaptation, evidence, uncertainty, provenance and
* explainability remain semantic capabilities represented elsewhere.
* 
* ============================================================================
* INTEGRATION WITH EFFECTS
* ============================================================================
* 
* A type bound does not itself define an effect.
* 
* For example:
* 
* T: ai::Trainable
* 
* does not automatically mean:
* 
* effect(learning)
* 
* If an operation performs learning, the operation/effect system owns that
* semantic effect.
* 
* This prevents type qualification and effect tracking from becoming
* entangled.
* 
* ============================================================================
* INTEGRATION WITH RESOURCES
* ============================================================================
* 
* A type may imply semantic requirements through its definition, but this
* file does not express physical resource allocation.
* 
* Example:
* 
* T: resource::MemoryBacked
* 
* remains a type relation.
* 
* Resource requirements are evaluated by the resource subsystem.
* 
* ============================================================================
* INTEGRATION WITH CONTRACTS
* ============================================================================
* 
* A contract may depend on a type:
* 
* requires type_property(...)
* 
* but contract syntax is owned elsewhere.
* 
* This file supplies only the reusable type-bound syntax.
* 
* ============================================================================
* INTEGRATION WITH POLICIES
* ============================================================================
* 
* Policies may permit, forbid, prefer, or constrain the use of types.
* 
* Policy syntax and policy evaluation remain outside this grammar.
* 
* ============================================================================
* INTEGRATION WITH PROVENANCE
* ============================================================================
* 
* The parser itself does not create provenance records.
* 
* The frontend must preserve enough source information for downstream
* provenance to record:
* 
* source location
* source artifact
* parsed construct
* semantic interpretation
* later transformations
* 
* ============================================================================
* INTEROPERABILITY CONTRACT
* ============================================================================
* 
* Foreign type declarations may consume canonical type constraints rather
* than inventing a foreign constraint language.
* 
* Relevant repository interoperability grammars must therefore reuse:
* 
* typeConstraintClause
* 
* where appropriate.
* 
* ABI-specific restrictions remain owned by the interoperability/ABI layer.
* 
* ============================================================================
* DIALECT CONTRACT
* ============================================================================
* 
* Dialects may introduce new types and type properties.
* 
* They should normally express those through:
* 
* named types
* qualified types
* generic types
* type constructors
* 
* rather than modifying this grammar with a growing keyword catalogue.
* 
* A dialect cannot silently change the meaning of:
* 
* typeConstraintClause
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file contains:
* 
* no finite domain catalogue;
* no machine-size constants;
* no hardware-size constants;
* no quantum-size constants;
* no tensor-rank constants;
* no network-size constants;
* no thread-count constants;
* no device-count constants;
* no generic-arity constants;
* no bound-count constants.
* 
* The only repetition limits are grammatical minimums:
* 
* one bound is required after ':';
* 
* additional bounds are represented by:
* 
*     (PLUS typeConstraintBound)*
* 
* ============================================================================
* SECURITY AUDIT
* ============================================================================
* 
* This file:
* 
* - executes no user code;
* - performs no filesystem access;
* - performs no network access;
* - performs no hardware discovery;
* - performs no process execution;
* - performs no environment inspection;
* - contains no embedded Rust;
* - contains no unsafe code;
* - evaluates no type-level expressions.
* 
* Any hostile-input resource protection is implemented by the parser/toolchain
* policy layer rather than by artificial grammar capacity limits.
* 
* ============================================================================
* CONFORMANCE EXAMPLES
* ============================================================================
* 
* VALID CLAUSE FRAGMENTS:
* 
* : Numeric
* 
* : Numeric + Comparable
* 
* : quantum::State
* 
* : quantum::State + quantum::Observable
* 
* : collections::Iterable<Value>
* 
* : future::computing::Capability
* 
* : collections::Iterable<collections::Iterable<Value>>
* 
* : tensor::Shape<N, M>
* 
* VALID GENERIC INTEGRATION:
* 
* <T: Numeric>
* 
* <T: Numeric + Comparable>
* 
* <T: quantum::State + quantum::Measurable>
* 
* INVALID:
* 
* :
* 
* : +
* 
* : + A
* 
* : A +
* 
* : A + +
* 
* : A + B +
* 
* : A B
* 
* The final forms depend on the surrounding grammar, but the type-constraint
* clause itself accepts only the colon followed by a bound and optional
* '+'-separated additional bounds.
* 
* ============================================================================
* SCALABILITY CONFORMANCE
* ============================================================================
* 
* These must remain structurally representable:
* 
* T: A
* 
* T: A + B + C
* 
* T: A + B + C + D + ...
* 
* T: a::b::c::d::Type
* 
* T: Generic<A<B<C<D>>>> 
* 
* T: Domain<Type<OtherType<Value>>>
* 
* The grammar must not introduce a fixed maximum for any of these.
* 
* ============================================================================
* NEGATIVE CONFORMANCE
* ============================================================================
* 
* The following must fail at the appropriate structural location:
* 
* T:
* 
* T: +
* 
* T: A +
* 
* T: + A
* 
* T: A + +
* 
* T: A + B +
* 
* T: A B
* 
* A semantically unknown type must NOT be rejected merely because its name
* is unknown:
* 
* T: future::UnknownType
* 
* That is a semantic-resolution concern.
* 
* ============================================================================
* DETERMINISM TEST CONTRACT
* ============================================================================
* 
* Given the same token sequence and same grammar vocabulary:
* 
* parse(source)
* 
* must produce the same parser structure.
* 
* Tests must not depend on:
* 
* machine identity;
* CPU count;
* GPU count;
* memory size;
* QPU availability;
* wall-clock time;
* randomness;
* network state;
* environment variables.
* 
* ============================================================================
* COMPILER INTEGRATION TEST CONTRACT
* ============================================================================
* 
* The following pipeline must be testable:
* 
* source
*   |
*   v
* lexer
*   |
*   v
* parser
*   |
*   v
* typeConstraintClause
*   |
*   v
* existing frontend AST TypeExpr
*   |
*   v
* structural validation
*   |
*   v
* semantic type constraints
*   |
*   v
* type inference / constraint solving
* 
* This grammar must never directly invoke the latter stages.
* 
* ============================================================================
* COMPATIBILITY
* ============================================================================
* 
* Compatibility requirements:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* ANTLR4
* 
* No nightly-only Rust facility is relevant to this grammar.
* 
* Existing source forms:
* 
* T: Numeric
* 
* T: Numeric + Comparable
* 
* must remain representable.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is DONE when:
* 
* [ ] It is the single canonical implementation of type-constraint syntax.
* [ ] "grammar/types/type-constraints.g4" no longer competes with it.
* [ ] It uses the canonical parser-facing lexer vocabulary.
* [ ] It declares no lexer rules.
* [ ] It defines no type-expression rules.
* [ ] It consumes the canonical "typeExpression".
* [ ] It owns exactly the reusable type-bound clause.
* [ ] It requires at least one bound after ':'.
* [ ] It supports arbitrary source-level bound count.
* [ ] It preserves source order.
* [ ] It permits unresolved type names for semantic resolution.
* [ ] It does not enumerate domain types.
* [ ] It does not enumerate quantum operations.
* [ ] It does not encode hardware topology.
* [ ] It does not encode physical resources.
* [ ] It does not introduce machine-size limits.
* [ ] It does not introduce quantum-size limits.
* [ ] It does not introduce tensor-rank limits.
* [ ] It does not introduce generic-arity limits.
* [ ] It does not introduce bound-count limits.
* [ ] It does not own general constraints.
* [ ] It does not own where clauses.
* [ ] It does not own contracts.
* [ ] It does not own policies.
* [ ] It does not own resource requirements.
* [ ] It does not own capability negotiation.
* [ ] It does not own type inference.
* [ ] It does not own constraint solving.
* [ ] It maps into the existing TypeExpr infrastructure.
* [ ] It remains independent of quantum::ir.
* [ ] It remains independent of physical hardware.
* [ ] It contains no embedded Rust.
* [ ] It requires no unsafe Rust.
* [ ] Positive tests exist.
* [ ] Negative tests exist.
* [ ] Boundary tests exist.
* [ ] Scalability tests exist.
* [ ] Cross-domain tests exist.
* [ ] Determinism tests exist.
* 
* ============================================================================
* CANONICAL GRAMMAR
* ============================================================================
  */

parser grammar TypeConstraints;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* PUBLIC ENTRY POINT
* ============================================================================
* 
* Reusable type-level constraint suffix.
* 
* Examples:
* 
* : Numeric
* 
* : Numeric + Comparable
* 
* : quantum::State + quantum::Measurable
* 
* The surrounding generic/declaration grammar owns the subject.
  */
  typeConstraintClause
  : COLON typeConstraintBoundList
  ;

/*

* ============================================================================
* ORDERED TYPE-BOUND LIST
* ============================================================================
* 
* One or more bounds.
* 
* No language-level maximum exists.
* 
* Source order is preserved by the parser tree.
  /
  typeConstraintBoundList
  : typeConstraintBound
  (PLUS typeConstraintBound)
  ;

/*

* ============================================================================
* SINGLE TYPE BOUND
* ============================================================================
* 
* Exactly one canonical type expression.
* 
* No type-specific alternatives are introduced here.
  */
  typeConstraintBound
  : typeExpression
  ;
  }

/*

* NOTE:
* 
* The final "}" above must NOT be present in the actual file.
* The grammar ends after "typeConstraintBound".
  */