/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/dependent.g4
 *
 * Grammar:
 *     DependentTypes
 *
 * Status:
 *     PRODUCTION TYPE-SYSTEM DELEGATE
 *
 * Language baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the source-level syntax for dependent function and
 * dependent product types.
 *
 * It is deliberately a NARROW DELEGATE.
 *
 * The complete Zamani type-expression language is owned by:
 *
 *     grammar/types/types.g4
 *
 * This file MUST therefore never define:
 *
 *     typeExpression
 *     typeCore
 *     typePath
 *     named types
 *     generic type applications
 *     arrays
 *     slices
 *     functions
 *     references
 *     pointers
 *     records
 *     unions
 *     sums
 *     ordinary expressions
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     dependentType
 *     dependentTypeForm
 *     dependentPiType
 *     dependentSigmaType
 *     dependentTypeConstructorPi
 *     dependentTypeConstructorSigma
 *     dependentTypeBinding
 *     dependentTypeParameter
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *     typePath
 *     typeValueExpression
 *     generic arguments
 *     generic declarations
 *     type bounds
 *     type constraints
 *     associated-type projections
 *     type-class declarations
 *     ordinary expressions
 *     contracts
 *     policies
 *     effects
 *     resources
 *     capabilities
 *     inference
 *     substitution
 *     unification
 *     normalization
 *     constraint solving
 *     proof checking
 *     specialization
 *     monomorphization
 *     resource negotiation
 *     capability negotiation
 *     hardware selection
 *     quantum allocation
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *     backend lowering
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * The dependency direction is:
 *
 *     grammar/types/types.g4
 *                  |
 *                  v
 *     grammar/types/dependent.g4
 *
 * NOT:
 *
 *     dependent.g4
 *          |
 *          v
 *     types.g4
 *
 * The canonical type orchestrator decides where `dependentType` is legal
 * inside the complete type-expression hierarchy.
 *
 * This delegate only supplies the dependent-type construct itself.
 *
 * ============================================================================
 * PUBLIC ENTRY CONTRACT
 * ============================================================================
 *
 * The public rule exported by this grammar is:
 *
 *     dependentType
 *
 * The canonical type orchestrator consumes it from `typeCore`.
 *
 * Conceptually:
 *
 *     typeExpression
 *         :
 *         typePrefix*
 *         typeCore
 *         typePostfix*
 *         ;
 *
 *     typeCore
 *         :
 *         ...
 *         | dependentType
 *         | ...
 *         ;
 *
 * ============================================================================
 * DEPENDENT TYPE MODEL
 * ============================================================================
 *
 * Zamani supports source-level dependent types whose later type structure may
 * depend upon an earlier bound value/type parameter.
 *
 * The initial normative forms are:
 *
 *     Pi x : A . B
 *
 *     Sigma x : A . B
 *
 * Pi represents a dependent function type.
 *
 * Sigma represents a dependent pair/product type.
 *
 * The grammar records structure only.
 *
 * Semantic analysis determines:
 *
 *     - whether the constructor is valid;
 *     - whether the binder type is well formed;
 *     - whether the body is well formed;
 *     - whether the binder is actually referenced;
 *     - whether references occur in legal scope;
 *     - whether substitutions are valid;
 *     - whether dependencies are legal;
 *     - whether constraints are satisfiable;
 *     - whether normalization is required;
 *     - whether specialization is possible;
 *     - whether the resulting type is representable on a selected target.
 *
 * ============================================================================
 * WHY PI AND SIGMA ARE CONTEXTUAL
 * ============================================================================
 *
 * `Pi` and `Sigma` are represented here through IDENTIFIER rather than
 * introducing lexer tokens owned by this file.
 *
 * This preserves the repository's single lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and prevents this grammar from creating duplicate lexical definitions.
 *
 * The syntax:
 *
 *     Pi x : A . B
 *
 * is recognized structurally.
 *
 * Semantic analysis establishes that the constructor identifier represents
 * the dependent Pi constructor in this context.
 *
 * Likewise:
 *
 *     Sigma x : A . B
 *
 * is recognized structurally and validated semantically.
 *
 * This keeps the source language extensible without requiring every future
 * type constructor to become a global lexer keyword.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * NO lexer rules are defined here.
 *
 * Required canonical tokens:
 *
 *     IDENTIFIER
 *     COLON
 *     DOT
 *
 * All token definitions belong to the canonical Zamani lexer.
 *
 * This grammar MUST NOT define:
 *
 *     PI
 *     SIGMA
 *     IDENTIFIER
 *     COLON
 *     DOT
 *
 * ============================================================================
 * PI TYPE
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     Pi x : A . B
 *
 * Meaning:
 *
 *     a dependent function type in which B may depend on x.
 *
 * Structural components:
 *
 *     constructor
 *     binder
 *     binder type
 *     body
 *
 * Scope:
 *
 *     x is NOT visible inside A.
 *
 *     x IS visible inside B.
 *
 * The grammar does not perform scope management.
 *
 * ============================================================================
 * SIGMA TYPE
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     Sigma x : A . B
 *
 * Meaning:
 *
 *     a dependent product/pair type in which B may depend on x.
 *
 * Scope:
 *
 *     x is NOT visible inside A.
 *
 *     x IS visible inside B.
 *
 * ============================================================================
 * BINDER CONTRACT
 * ============================================================================
 *
 * A dependent binder consists of:
 *
 *     IDENTIFIER COLON typeExpression
 *
 * Example:
 *
 *     x : Nat
 *
 * The parameter name is intentionally unrestricted.
 *
 * Names such as:
 *
 *     N
 *     Length
 *     Rows
 *     Columns
 *     Width
 *     Shape
 *
 * have no special grammar meaning.
 *
 * They remain ordinary identifiers whose meaning is determined by semantic
 * analysis.
 *
 * ============================================================================
 * TYPE-EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The binder type and dependent body both consume the canonical:
 *
 *     typeExpression
 *
 * rule supplied by the type orchestrator.
 *
 * This is essential.
 *
 * It permits:
 *
 *     Pi n : Nat . Vector[n]
 *
 *     Pi T : Type . List[T]
 *
 *     Pi n : Nat . Pi m : Nat . Matrix[n, m]
 *
 *     Sigma T : Type . T
 *
 *     Pi Q : quantum::Qubit . Register[Q]
 *
 * without introducing a second type grammar.
 *
 * ============================================================================
 * VALUE-PARAMETERIZED TYPE INTEGRATION
 * ============================================================================
 *
 * Bracketed value-parameterized type applications such as:
 *
 *     Vector[N]
 *
 *     Matrix[T][Rows, Columns]
 *
 *     Tensor[T][Shape]
 *
 * are NOT owned by this file.
 *
 * Their canonical composition belongs to the type orchestrator and its
 * type-level value-expression subsystem.
 *
 * In particular, this file MUST NOT define:
 *
 *     typeValueExpression
 *     typeValuePrimary
 *     typeValueQualifiedPath
 *
 * and MUST NOT define a second bracketed type application grammar.
 *
 * This separation is deliberate:
 *
 *     dependent.g4
 *         -> dependent Pi/Sigma syntax
 *
 *     types.g4
 *         -> complete type composition and value-parameterized type syntax
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic application remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * A dependent type may contain generic types because its binder type and
 * body consume the complete canonical `typeExpression`.
 *
 * Examples:
 *
 *     Pi n : Nat . Vector[n]
 *
 *     Pi T : Type . Container<T>
 *
 *     Pi n : Nat . Matrix<Value>[n, n]
 *
 * The dependent grammar does not parse the generic arguments itself.
 *
 * ============================================================================
 * ASSOCIATED-TYPE INTEGRATION
 * ============================================================================
 *
 * Associated type syntax remains owned by:
 *
 *     grammar/types/associated.g4
 *
 * A dependent body may contain an associated projection because the body
 * delegates to canonical `typeExpression`.
 *
 * No associated-type grammar is duplicated here.
 *
 * ============================================================================
 * TYPE-BOUND INTEGRATION
 * ============================================================================
 *
 * Type bounds remain owned by:
 *
 *     grammar/types/bounds.g4
 *
 * Dependent binders do not invent a second bound language.
 *
 * Any constraint on a dependent parameter is represented through the
 * canonical type/constraint system and checked semantically.
 *
 * ============================================================================
 * TYPE-CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * Dependent types may participate in constraints such as:
 *
 *     N > 0
 *
 *     Rows = Columns
 *
 *     Shape compatible with ...
 *
 *     index < length
 *
 * These are NOT grammar-level dependent-type rules.
 *
 * The grammar merely preserves the dependent type structure.
 *
 * Constraint construction and solving belong to the semantic/type-constraint
 * subsystem.
 *
 * ============================================================================
 * TYPE-LEVEL VALUE INTEGRATION
 * ============================================================================
 *
 * Dependent types may contain symbolic values through the canonical type
 * system.
 *
 * Examples:
 *
 *     Vector[N]
 *
 *     Matrix[T][Rows, Columns]
 *
 *     Register[Width]
 *
 *     Tensor<Element>[Shape]
 *
 * The grammar does not convert these values to:
 *
 *     usize
 *     u32
 *     u64
 *     machine word size
 *     host pointer size
 *
 * Symbolic values remain source-level semantic data until type-level
 * evaluation determines their value domain and validity.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO new AST hierarchy.
 *
 * Existing canonical frontend representations are:
 *
 *     TypeExpr::Pi
 *     TypeExpr::Sigma
 *
 * The source structure must map to the repository's canonical frontend
 * representation.
 *
 * Conceptually:
 *
 *     Pi x : A . B
 *
 * becomes:
 *
 *     TypeExpr::Pi {
 *         parameter: x,
 *         parameter_type: A,
 *         body: B
 *     }
 *
 * and:
 *
 *     Sigma x : A . B
 *
 * becomes:
 *
 *     TypeExpr::Sigma {
 *         parameter: x,
 *         parameter_type: A,
 *         body: B
 *     }
 *
 * The exact constructor shape is owned by:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * This grammar must not introduce:
 *
 *     DependentTypeExpr
 *     PiTypeExpr
 *     SigmaTypeExpr
 *     DependentValueTypeExpr
 *     QuantumDependentTypeExpr
 *     HardwareDependentTypeExpr
 *
 * ============================================================================
 * SOURCE-PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser/AST layer must preserve source locations for:
 *
 *     constructor
 *     binder
 *     binder type
 *     dot separator
 *     body
 *
 * This permits diagnostics such as:
 *
 *     invalid dependent binder
 *     unknown dependent constructor
 *     invalid dependency
 *     unsatisfied dependent constraint
 *
 * to point back to the exact source construct.
 *
 * This grammar itself does not construct source spans because the repository
 * keeps AST construction in its frontend implementation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing is structural only.
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     binder scope
 *     type formation
 *     dependency analysis
 *     substitution
 *     alpha-equivalence
 *     beta-reduction where applicable
 *     normalization
 *     unification
 *     inference
 *     constraint generation
 *     constraint solving
 *     proof obligations
 *     universe/kind checking
 *     recursive dependency checking
 *     specialization
 *     monomorphization
 *     compile-time evaluation
 *
 * A syntactically valid dependent type is therefore not necessarily
 * semantically valid.
 *
 * ============================================================================
 * BINDER SCOPE SEMANTICS
 * ============================================================================
 *
 * For:
 *
 *     Pi x : A . B
 *
 * semantic processing must behave conceptually as:
 *
 *     resolve A
 *          |
 *          v
 *     introduce x
 *          |
 *          v
 *     resolve B
 *
 * Therefore:
 *
 *     x is unavailable in A
 *
 * and:
 *
 *     x is available in B.
 *
 * The same rule applies to Sigma.
 *
 * The grammar deliberately does not use semantic predicates to enforce this
 * because scope is semantic information, not lexical structure.
 *
 * ============================================================================
 * DEPENDENCY SEMANTICS
 * ============================================================================
 *
 * A dependent body may refer to:
 *
 *     its immediate binder;
 *     enclosing dependent binders;
 *     enclosing generic parameters;
 *     enclosing type parameters;
 *     associated types;
 *     valid type-level values;
 *     other semantically visible declarations.
 *
 * Whether a reference is valid is determined downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Dependent types may describe symbolic quantities used by resource-aware
 * programs.
 *
 * Examples:
 *
 *     Register[N]
 *
 *     Tensor[T][Shape]
 *
 *     MemoryRegion[Size]
 *
 *     Buffer[Element][Capacity]
 *
 * Such a type describes source-level structure.
 *
 * It does NOT allocate resources.
 *
 * Resource analysis belongs to:
 *
 *     grammar/resources/
 *     semantic resource analysis
 *     compiler resource planning
 *     execution planning
 *     deployment
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Parsing a dependent type requires no target capability.
 *
 * A semantic type may later generate capability requirements.
 *
 * For example:
 *
 *     Register[N]
 *
 * may participate in a semantic requirement for a quantum capability.
 *
 * The dependent grammar does not perform capability discovery or negotiation.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Dependent types are domain-neutral and may describe quantum source
 * structures such as:
 *
 *     Register[N]
 *
 *     State[Dimension]
 *
 *     Circuit[Parameters]
 *
 *     Tensor[Shape]
 *
 * These are source-level symbolic descriptions.
 *
 * They do NOT mean:
 *
 *     allocate N physical qubits
 *
 * or:
 *
 *     select a QPU with N physical resources.
 *
 * The downstream path remains:
 *
 *     TypeExpr
 *         |
 *         v
 *     semantic quantum type
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     decomposition
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     resilience / QEC
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL
 *
 * This grammar introduces no physical quantum resource model.
 *
 * ============================================================================
 * HDL CONTRACT
 * ============================================================================
 *
 * Dependent types may describe symbolic HDL structures:
 *
 *     Bus[Width]
 *
 *     Signal[Width]
 *
 *     Matrix[Rows, Columns]
 *
 * They do not determine:
 *
 *     physical pin count
 *     FPGA fabric size
 *     ASIC technology
 *     placement
 *     routing
 *     timing implementation
 *     synthesis strategy
 *
 * Those belong to HDL/hardware semantic and backend layers.
 *
 * ============================================================================
 * CLASSICAL / DATA / AI CONTRACT
 * ============================================================================
 *
 * Dependent types are equally usable for:
 *
 *     classical structures
 *     numerical structures
 *     tensors
 *     datasets
 *     models
 *     probability structures
 *     knowledge structures
 *     distributed data
 *
 * No application-specific keyword set is introduced here.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing dependent types has no runtime effect.
 *
 * Dependent-type semantic analysis may perform compile-time computation,
 * constraint generation, normalization, or proof checking.
 *
 * Such effects belong to the semantic/compiler subsystem.
 *
 * This grammar does not execute code.
 *
 * ============================================================================
 * CONTRACT / VALIDATION CONTRACT
 * ============================================================================
 *
 * Dependent types can participate in semantic obligations such as:
 *
 *     index < length
 *
 *     rows = columns
 *
 *     width > 0
 *
 *     shape compatibility
 *
 *     valid domain/codomain formation
 *
 * These are not solved by this grammar.
 *
 * They belong to the type constraint, validation, and proof infrastructure.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies do not belong to dependent-type syntax.
 *
 * Resource, security, specialization, execution, deployment, adaptation,
 * and capability policies are evaluated by their respective semantic
 * subsystems.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The dependent-type AST must preserve enough source provenance to trace:
 *
 *     dependent constructor
 *     binder
 *     binder type
 *     body
 *
 * through later semantic transformations.
 *
 * This is required for deterministic diagnostics and compiler provenance.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level limit on:
 *
 *     number of dependent binders
 *     dependent nesting depth
 *     dependent body complexity
 *     number of type parameters
 *     number of generic parameters
 *     number of symbolic dimensions
 *     symbolic value magnitude
 *     tensor rank
 *     array cardinality
 *     quantum cardinality
 *     hardware cardinality
 *     distributed resource cardinality
 *
 * In particular, this file MUST NOT define language ceilings such as:
 *
 *     MAX_DEPENDENT_PARAMETERS
 *     MAX_DEPENDENT_DEPTH
 *     MAX_DEPENDENT_ARGUMENTS
 *     MAX_DIMENSIONS
 *     MAX_TENSOR_RANK
 *     MAX_ARRAY_LENGTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Compiler/parser resource protection may exist outside the grammar as an
 * explicit execution/resource policy.
 *
 * Such a policy MUST NOT alter the language's semantic meaning.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical:
 *
 *     source
 *     lexer configuration
 *     language version
 *     grammar version
 *
 * parsing must produce deterministic equivalent syntax structure.
 *
 * This grammar contains:
 *
 *     no semantic actions
 *     no semantic predicates
 *     no filesystem operations
 *     no network operations
 *     no hardware discovery
 *     no runtime execution
 *     no randomness
 *     no embedded Rust
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * The parser is responsible only for structural syntax errors.
 *
 * Structural errors include:
 *
 *     missing constructor
 *     missing binder
 *     missing colon
 *     missing binder type
 *     missing dot
 *     missing body
 *
 * Semantic errors include:
 *
 *     unknown type
 *     invalid binder scope
 *     invalid dependency
 *     illegal recursive dependency
 *     unsatisfied dependent constraint
 *     inconsistent dependent constraints
 *     invalid specialization
 *     unsupported type-level computation
 *
 * Semantic errors MUST NOT be encoded as grammar alternatives.
 *
 * ============================================================================
 * POSITIVE CONFORMANCE
 * ============================================================================
 *
 * The following structures must be representable:
 *
 *     Pi n : Nat . Vector[n]
 *
 *     Sigma n : Nat . Vector[n]
 *
 *     Pi T : Type . List[T]
 *
 *     Pi n : Nat . Pi m : Nat . Matrix[n, m]
 *
 *     Sigma T : Type . T
 *
 *     Pi Q : quantum::Qubit . Register[Q]
 *
 * The names used in these examples are ordinary identifiers.
 *
 * They are not universal keywords.
 *
 * ============================================================================
 * NEGATIVE CONFORMANCE
 * ============================================================================
 *
 * The following forms are structurally invalid:
 *
 *     Pi
 *
 *     Pi n
 *
 *     Pi n :
 *
 *     Pi n : A
 *
 *     Pi : A . B
 *
 *     Pi n A . B
 *
 *     Sigma
 *
 *     Sigma n
 *
 *     Sigma n :
 *
 *     Sigma n : A
 *
 *     Sigma : A . B
 *
 *     Sigma n A . B
 *
 * These should be rejected structurally by parser conformance tests.
 *
 * ============================================================================
 * BOUNDARY CONFORMANCE
 * ============================================================================
 *
 * Tests must include:
 *
 *     Pi + generic type
 *     Sigma + generic type
 *     Pi + array type
 *     Sigma + tuple type
 *     Pi + function type
 *     nested Pi
 *     nested Sigma
 *     Pi containing Sigma
 *     Sigma containing Pi
 *     dependent type inside a generic type
 *     dependent type inside a function type
 *     dependent type inside a tuple type
 *     dependent type inside a record type
 *     dependent type inside quantum source types
 *     dependent type inside HDL source types
 *     dependent type participating in resource descriptions
 *     dependent type participating in capability descriptions
 *     qualified names
 *     source-span preservation
 *     semantic binder visibility
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * The required downstream path is:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     frontend TypeExpr
 *       |
 *       v
 *     semantic dependent type
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL/hardware
 *       +--> distributed
 *       +--> future domains
 *
 * This grammar MUST NOT reference:
 *
 *     LLVM
 *     MLIR
 *     QIR
 *     vendor IR
 *     physical QPU topology
 *     FPGA fabric
 *     CPU topology
 *     GPU topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC implementation
 *     ZQN implementation
 *     HAL implementation
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical source forms:
 *
 *     Pi x : A . B
 *
 *     Sigma x : A . B
 *
 * remain valid.
 *
 * Existing frontend TypeExpr::Pi and TypeExpr::Sigma representations remain
 * the canonical AST target.
 *
 * No alternative dependent-type AST is introduced.
 *
 * Any future syntax change must be handled through the repository's language
 * compatibility and migration system.
 *
 * Deprecated syntax must not accumulate indefinitely as ambiguous grammar
 * alternatives.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     canonical Zamani lexer vocabulary
 *     typeExpression
 *
 * EXPORTS:
 *
 *     dependentType
 *     dependentTypeForm
 *     dependentPiType
 *     dependentSigmaType
 *     dependentTypeConstructorPi
 *     dependentTypeConstructorSigma
 *     dependentTypeBinding
 *     dependentTypeParameter
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *
 * AST_OWNER:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * LEGACY_AST_OWNER:
 *
 *     src/ast/mod.rs
 *
 * The legacy AST must converge toward the canonical frontend AST rather than
 * receiving a new dependent-type hierarchy.
 *
 * SEMANTIC_OWNER:
 *
 *     semantic/type-system implementation
 *
 * CONSTRAINT_OWNER:
 *
 *     type constraint / validation subsystem
 *
 * PROOF_OWNER:
 *
 *     semantic proof/obligation subsystem
 *
 * VALUE_EVALUATION_OWNER:
 *
 *     compile-time/type-level evaluation subsystem
 *
 * SPECIALIZATION_OWNER:
 *
 *     compiler specialization/monomorphization subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR lowering subsystem
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum semantic lowering and quantum::ir
 *
 * HDL_OWNER:
 *
 *     HDL/hardware semantic lowering
 *
 * RESOURCE_OWNER:
 *
 *     resource analysis and negotiation
 *
 * CAPABILITY_OWNER:
 *
 *     capability analysis and negotiation
 *
 * TEST_OWNER:
 *
 *     grammar/tests/type/dependent/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/syntax.md
 *     grammar/specification/types.md
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [ ] only dependent-type syntax is owned here;
 * [ ] no lexer rules exist here;
 * [ ] no embedded Rust exists here;
 * [ ] no semantic predicates exist here;
 * [ ] no value-expression grammar is duplicated here;
 * [ ] no generic grammar is duplicated here;
 * [ ] no complete type-expression grammar exists here;
 * [ ] Pi is structurally parseable;
 * [ ] Sigma is structurally parseable;
 * [ ] binder syntax is canonical;
 * [ ] binder scope is preserved for semantic analysis;
 * [ ] TypeExpr::Pi remains the AST destination;
 * [ ] TypeExpr::Sigma remains the AST destination;
 * [ ] no second dependent AST exists;
 * [ ] no second IR exists;
 * [ ] no target-specific semantics exist;
 * [ ] no physical resource allocation exists;
 * [ ] no hardware limits exist;
 * [ ] no language-level dependent-type limits exist;
 * [ ] generic integration is delegated;
 * [ ] type-value integration is delegated;
 * [ ] constraint solving is delegated;
 * [ ] proof checking is delegated;
 * [ ] specialization is delegated;
 * [ ] quantum lowering is delegated;
 * [ ] HDL lowering is delegated;
 * [ ] resource analysis is delegated;
 * [ ] capability analysis is delegated;
 * [ ] positive tests exist;
 * [ ] negative tests exist;
 * [ ] boundary tests exist;
 * [ ] scalability tests exist;
 * [ ] deterministic parsing tests exist;
 * [ ] compatibility tests exist;
 * [ ] canonical ANTLR generation succeeds;
 * [ ] canonical parser generation succeeds;
 * [ ] frontend AST integration succeeds;
 * [ ] semantic integration succeeds.
 *
 * ============================================================================
 */


/* ============================================================================
 * PUBLIC DEPENDENT-TYPE ENTRY
 * ============================================================================
 *
 * `types.g4` consumes this rule from its `typeCore` composition.
 */
dependentType
    : dependentTypeForm
    ;


/* ============================================================================
 * DEPENDENT-TYPE FORM
 * ============================================================================
 *
 * The current normative dependent constructors are Pi and Sigma.
 *
 * The constructor spellings remain contextual identifiers so this grammar
 * does not create a second lexical authority.
 */
dependentTypeForm
    : dependentPiType
    | dependentSigmaType
    ;


/* ============================================================================
 * PI TYPE
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     Pi x : A . B
 *
 * The binder is parsed before the body so the AST preserves the source
 * structure required for semantic scope construction.
 */
dependentPiType
    : dependentTypeConstructorPi
      dependentTypeBinding
      DOT
      typeExpression
    ;


/* ============================================================================
 * SIGMA TYPE
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     Sigma x : A . B
 */
dependentSigmaType
    : dependentTypeConstructorSigma
      dependentTypeBinding
      DOT
      typeExpression
    ;


/* ============================================================================
 * PI CONSTRUCTOR
 * ============================================================================
 *
 * Contextual spelling:
 *
 *     Pi
 *
 * represented lexically as IDENTIFIER.
 *
 * Semantic analysis validates the constructor's contextual meaning.
 */
dependentTypeConstructorPi
    : IDENTIFIER
    ;


/* ============================================================================
 * SIGMA CONSTRUCTOR
 * ============================================================================
 *
 * Contextual spelling:
 *
 *     Sigma
 *
 * represented lexically as IDENTIFIER.
 */
dependentTypeConstructorSigma
    : IDENTIFIER
    ;


/* ============================================================================
 * DEPENDENT BINDER
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     x : A
 *
 * The binder identifier is not in scope while its own type is being parsed.
 *
 * Scope introduction belongs to semantic analysis after the binder type has
 * been resolved.
 */
dependentTypeBinding
    : dependentTypeParameter
      COLON
      typeExpression
    ;


/* ============================================================================
 * DEPENDENT PARAMETER
 * ============================================================================
 *
 * Any canonical identifier may be used.
 *
 * No predefined dimension/resource names are required.
 */
dependentTypeParameter
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * END OF DEPENDENT TYPES
 * ============================================================================
 */