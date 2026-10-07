/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/bounds.g4
 *
 * Grammar:
 *     TypeBounds
 *
 * Status:
 *     CANONICAL TYPE-BOUND PARSER DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the SINGLE OWNER of the reusable source-level syntax for
 * type bounds.
 *
 * A type bound qualifies a type parameter or another type-level subject with
 * one or more type expressions.
 *
 * Canonical examples:
 *
 *     T: Numeric
 *
 *     T: Numeric + Comparable
 *
 *     T: quantum::State
 *
 *     T: quantum::State + quantum::Measurable
 *
 *     T: collections::Iterable<Value>
 *
 *     T: hardware::Accelerator<Model>
 *
 *     T: future::computing::Capability
 *
 * This grammar deliberately accepts complete canonical type expressions.
 *
 * It therefore does NOT maintain a catalogue of:
 *
 *     Numeric
 *     Comparable
 *     Iterable
 *     QuantumState
 *     LogicalQubit
 *     Tensor
 *     GPU
 *     FPGA
 *     Accelerator
 *     Dataset
 *     Model
 *     NetworkEndpoint
 *     or any other domain-specific bound name.
 *
 * Names are resolved semantically.
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * A type bound is:
 *
 *     source-level type syntax
 *
 * followed by:
 *
 *     semantic type checking
 *
 * followed by:
 *
 *     constraint solving
 *
 * It is NOT:
 *
 *     resource allocation
 *     hardware selection
 *     capability discovery
 *     target selection
 *     physical topology
 *     routing
 *     scheduling
 *     optimization
 *     runtime execution
 *
 * The complete pipeline is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     type bound
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type system
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 *     type inference        constraint solving
 *       |                         |
 *       +------------+------------+
 *                    |
 *                    v
 *             canonical semantic
 *                 model / IR
 *                    |
 *          +---------+---------+
 *          |                   |
 *          v                   v
 *     classical semantics   quantum semantics
 *                              |
 *                              v
 *                          quantum::ir
 *                              |
 *                              v
 *                    target-independent compilation
 *                              |
 *                              v
 *                    target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     typeBoundClause
 *     typeBoundList
 *     typeBound
 *
 * and the syntactic relationship:
 *
 *     ':' type-bound ('+' type-bound)*
 *
 * THIS FILE ALSO OWNS:
 *
 *     source ordering of bounds
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     primitive types
 *     named types
 *     generic type applications
 *     tuples
 *     arrays
 *     slices
 *     function types
 *     references
 *     pointers
 *     records
 *     sums
 *     unions
 *     dependent types
 *     associated types
 *     type classes
 *     linear types
 *     affine types
 *     classical types
 *     quantum types
 *     hardware types
 *     resource types
 *     capability types
 *     temporal types
 *     effect-qualified types
 *
 * It also does not own:
 *
 *     generic parameter declarations
 *     generic argument declarations
 *     where clauses
 *     arbitrary value constraints
 *     contracts
 *     policies
 *     resource requirements
 *     capability negotiation
 *     inference
 *     unification
 *     substitution
 *     specialization
 *     monomorphization
 *     target selection
 *     hardware discovery
 *     placement
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime behavior
 *
 * ============================================================================
 * SINGLE TYPE-EXPRESSION AUTHORITY
 * ============================================================================
 *
 * The complete source-level type expression remains owned by:
 *
 *     grammar/types/types.g4
 *
 * This file consumes that canonical `typeExpression` rule.
 *
 * It MUST NOT define another type-expression grammar.
 *
 * In particular, this file must never introduce:
 *
 *     boundsTypeExpression
 *     boundTypeExpression
 *     genericBoundType
 *     quantumBoundType
 *     hardwareBoundType
 *     aiBoundType
 *     resourceBoundType
 *
 * Every bound uses the same type-expression language used everywhere else.
 *
 * ============================================================================
 * SINGLE LEXER AUTHORITY
 * ============================================================================
 *
 * This file contains no lexer rules.
 *
 * Tokens are supplied by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through the repository's canonical lexer composition.
 *
 * Required punctuation:
 *
 *     COLON
 *     PLUS
 *
 * This file MUST NOT define those tokens locally.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is intentionally a parser delegate.
 *
 * It is intended to be imported by the canonical type grammar composition
 * that exposes `typeExpression`.
 *
 * The intended dependency direction is:
 *
 *     Type orchestrator
 *          |
 *          +--> TypeBounds
 *          |
 *          +--> Generic
 *          |
 *          +--> Named
 *          |
 *          +--> Array
 *          |
 *          +--> Quantum
 *          |
 *          +--> Hardware
 *          |
 *          +--> ...
 *
 * There MUST NOT be a dependency cycle such as:
 *
 *     Types
 *       -> Bounds
 *       -> Types
 *
 * through a second imported copy of the type-expression grammar.
 *
 * The repository's composition root must provide exactly one
 * `typeExpression` rule.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * The reusable public boundary supplied by this file is:
 *
 *     typeBoundClause
 *
 * It represents:
 *
 *     ':' typeBoundList
 *
 * The following rules are implementation details of this delegate:
 *
 *     typeBoundList
 *     typeBound
 *
 * They remain named and reusable so surrounding grammars can refer to the
 * complete bound structure when needed.
 *
 * ============================================================================
 * TYPE BOUND CLAUSE
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     : Numeric
 *
 *     : Numeric + Comparable
 *
 *     : quantum::State
 *
 *     : quantum::State + quantum::Measurable
 *
 * The colon belongs to the bound clause.
 *
 * The generic parameter itself belongs to the generic declaration owner.
 *
 * Therefore this file does NOT define:
 *
 *     genericParameter
 *
 * or:
 *
 *     genericParameterList
 *
 * ============================================================================
 * ORDERED BOUNDS
 * ============================================================================
 *
 * The canonical structure is:
 *
 *     A
 *     A + B
 *     A + B + C
 *     A + B + C + ...
 *
 * represented by:
 *
 *     typeBound (PLUS typeBound)*
 *
 * This creates no grammar-level maximum for the number of bounds.
 *
 * The implementation may impose operational resource safeguards against
 * hostile input, but those safeguards are compiler policy and are NOT
 * language-level expressive limits.
 *
 * ============================================================================
 * TYPE BOUND
 * ============================================================================
 *
 * Every individual bound is exactly one complete:
 *
 *     typeExpression
 *
 * This is essential.
 *
 * For example:
 *
 *     T: Numeric
 *
 *     T: collections::Iterable<Value>
 *
 *     T: quantum::State<N>
 *
 *     T: hardware::Accelerator<Model<Input, Output>>
 *
 * all use the same type-expression authority.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * New computational domains MUST NOT require modification of this file.
 *
 * For example, future source types may be used as bounds:
 *
 *     T: quantum::State
 *     T: hdl::Module
 *     T: hardware::Accelerator
 *     T: ai::Model
 *     T: data::Dataset
 *     T: distributed::Message
 *     T: networking::Endpoint
 *     T: future::domain::Capability
 *
 * without adding a new alternative here.
 *
 * This is mandatory for long-term language scalability.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Type bounds describe logical program requirements.
 *
 * They do not identify physical resources.
 *
 * This file therefore contains no finite capacity assumptions.
 *
 * In particular, it contains no:
 *
 *     maximum qubit count
 *     maximum CPU count
 *     maximum GPU count
 *     maximum FPGA count
 *     maximum node count
 *     maximum memory
 *     maximum thread count
 *     maximum tensor rank
 *     maximum register width
 *     maximum network size
 *     maximum device count
 *     maximum generic arity
 *     maximum bound count
 *     maximum bound nesting depth
 *
 * A source program may therefore express a bound structure whose eventual
 * realization scales according to available implementation resources.
 *
 * "Infinity" here means that the LANGUAGE GRAMMAR introduces no artificial
 * finite ceiling.
 *
 * Physical resources remain finite and are handled downstream.
 *
 * ============================================================================
 * RESOURCE SEPARATION
 * ============================================================================
 *
 * This:
 *
 *     T: quantum::State
 *
 * is a type-level condition.
 *
 * It is not equivalent to:
 *
 *     requires capability("quantum.measurement")
 *
 * nor:
 *
 *     requires qubits >= n
 *
 * nor:
 *
 *     requires topology(required_topology)
 *
 * Those belong to resource/capability semantics.
 *
 * The relationship is:
 *
 *     type bound
 *          |
 *          v
 *     type satisfiability
 *
 * while:
 *
 *     resource requirement
 *          |
 *          v
 *     resource/capability negotiation
 *          |
 *          v
 *     target realization
 *
 * A compiler may use both analyses, but this grammar does not merge them.
 *
 * ============================================================================
 * CAPABILITY SEPARATION
 * ============================================================================
 *
 * A capability may be represented by a type if the type system defines such
 * a type:
 *
 *     T: quantum::Measurable
 *
 * But runtime/compiler capability requirements remain separate:
 *
 *     requires capability("quantum.measurement")
 *
 * This prevents type syntax from becoming a hardware-discovery mechanism.
 *
 * ============================================================================
 * CONTRACT SEPARATION
 * ============================================================================
 *
 * These constructs are NOT type bounds:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * They belong to the contract/validation architecture.
 *
 * A semantic checker may combine contract information with type information,
 * but the parser ownership remains separate.
 *
 * ============================================================================
 * POLICY SEPARATION
 * ============================================================================
 *
 * These constructs are NOT type bounds:
 *
 *     allow
 *     forbid
 *     prefer
 *     fallback
 *     constrain
 *
 * They belong to policy semantics.
 *
 * ============================================================================
 * DEPENDENT TYPE INTEGRATION
 * ============================================================================
 *
 * A bound may contain a dependent or value-parameterized type whenever the
 * canonical type-expression grammar permits it.
 *
 * Examples:
 *
 *     T: Matrix<Element, Rows, Columns>
 *
 *     T: Tensor<Element, Shape>
 *
 *     T: Register<Qubit, Count>
 *
 * The grammar does not evaluate:
 *
 *     Rows
 *     Columns
 *     Shape
 *     Count
 *
 * Evaluation and satisfiability are semantic responsibilities.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic declarations own parameter syntax.
 *
 * Conceptually:
 *
 *     genericParameter
 *         : IDENTIFIER typeBoundClause?
 *         ;
 *
 * This file supplies only:
 *
 *     typeBoundClause
 *
 * It does not own the generic parameter.
 *
 * Generic type application remains separate:
 *
 *     Type<Argument>
 *
 * and is owned by:
 *
 *     grammar/types/generic.g4
 *
 * ============================================================================
 * WHERE-CLAUSE INTEGRATION
 * ============================================================================
 *
 * This file does not own `where`.
 *
 * A surrounding declaration grammar may compose:
 *
 *     whereTypeConstraint
 *         : typeExpression typeBoundClause
 *         ;
 *
 * if that is the repository's normative where-clause form.
 *
 * A complete `where` grammar may eventually support additional semantic
 * constraint categories. Therefore `where` must remain owned by its
 * declaration/constraint subsystem rather than being embedded here.
 *
 * ============================================================================
 * ASSOCIATED TYPE INTEGRATION
 * ============================================================================
 *
 * Associated types remain ordinary canonical type expressions.
 *
 * Therefore a bound may structurally contain an associated type expression
 * wherever the canonical type grammar permits one.
 *
 * This grammar does not resolve associated-type declarations.
 *
 * Resolution belongs to semantic type checking.
 *
 * ============================================================================
 * TYPE-CLASS / TRAIT INTEGRATION
 * ============================================================================
 *
 * A type-class/trait-like property is represented structurally as a type
 * expression.
 *
 * Examples:
 *
 *     T: Numeric
 *
 *     T: Comparable
 *
 *     T: collections::Iterable<Value>
 *
 * Whether these denote:
 *
 *     traits
 *     interfaces
 *     structural properties
 *     nominal relations
 *     type predicates
 *
 * is a semantic decision.
 *
 * The grammar does not hard-code one interpretation.
 *
 * ============================================================================
 * LINEAR / AFFINE INTEGRATION
 * ============================================================================
 *
 * Bounds may reference types participating in linear or affine semantics.
 *
 * This grammar does not enforce ownership or usage rules.
 *
 * Those remain owned by the semantic type system and the specialized:
 *
 *     grammar/types/linear.g4
 *     grammar/types/affine.g4
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types are accepted only through canonical `typeExpression`.
 *
 * This file does not define:
 *
 *     gates
 *     physical qubits
 *     coupling maps
 *     calibration
 *     routing
 *     scheduling
 *     error-correction layout
 *     pulses
 *     QPU identifiers
 *
 * Example:
 *
 *     T: quantum::State
 *
 * remains a source-level type relation.
 *
 * Semantic quantum lowering remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic quantum model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience/QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical type properties are ordinary type expressions.
 *
 * Examples:
 *
 *     T: Numeric
 *     T: Comparable
 *     T: Iterable
 *     T: Serializable
 *
 * No catalogue of classical operations is required.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL properties are ordinary source-level type expressions.
 *
 * Examples:
 *
 *     T: hdl::Module
 *     T: hardware::Signal
 *     T: hardware::Memory
 *     T: hardware::Accelerator
 *
 * Physical realization is not performed here.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Data and model types remain ordinary types.
 *
 * Examples:
 *
 *     T: ai::Model
 *     T: data::Dataset
 *     T: data::Serializable
 *     T: knowledge::Evidence
 *
 * This grammar therefore requires no application-specific type-bound
 * keywords.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * Bounds can refer to distributed and networking types:
 *
 *     T: distributed::Message
 *     T: distributed::Actor
 *     T: networking::Endpoint
 *     T: networking::Stream
 *
 * Resource topology and network availability remain downstream concerns.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * This file does not define effects.
 *
 * Effect-qualified type syntax, where supported, is consumed through the
 * canonical type-expression system.
 *
 * Effect semantics remain owned by:
 *
 *     grammar/effects/
 *
 * and the semantic effect system.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * The parser must preserve enough source structure and locations for the
 * frontend to associate:
 *
 *     complete bound clause
 *     colon
 *     individual bounds
 *     plus separators
 *     underlying type expressions
 *
 * with source spans.
 *
 * This enables downstream diagnostics and provenance without embedding
 * semantic actions in this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO domain-specific AST node.
 *
 * The conceptual semantic structure is:
 *
 *     TypeBoundClause
 *         |
 *         +--> ordered TypeExpr[]
 *
 * Each `typeBound` corresponds to exactly one canonical `TypeExpr`.
 *
 * IMPORTANT:
 *
 * The current Rust AST contains a legacy/narrow representation:
 *
 *     TypeBound::Trait(Identifier)
 *     TypeBound::Lifetime(String)
 *
 * That representation is insufficient for the complete open-world type
 * expression accepted by this grammar.
 *
 * The required frontend evolution is therefore:
 *
 *     parsed type bound
 *          |
 *          v
 *     canonical TypeExpr
 *          |
 *          v
 *     semantic type-bound representation
 *
 * The grammar MUST NOT work around that AST limitation by adding:
 *
 *     QuantumBound
 *     HardwareBound
 *     TensorBound
 *     AIBound
 *     GPUBound
 *     FPGA_Bound
 *     ResourceBound
 *
 * Such domain-specific AST variants would destroy the open-world property.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     whether the bound subject is valid;
 *     whether each bound resolves;
 *     whether the bound is a type;
 *     whether the relationship is supported;
 *     whether multiple bounds are compatible;
 *     whether associated types resolve;
 *     whether dependent conditions are satisfiable;
 *     whether type-class/trait relations hold;
 *     whether generic substitution preserves validity.
 *
 * This grammar performs none of those operations.
 *
 * ============================================================================
 * TYPE INFERENCE
 * ============================================================================
 *
 * Inference may generate constraints corresponding to source-level bounds.
 *
 * Such inferred constraints belong to the compiler's semantic/inference
 * representation.
 *
 * This grammar remains only the source syntax boundary.
 *
 * ============================================================================
 * SPECIALIZATION
 * ============================================================================
 *
 * Generic specialization and monomorphization occur downstream.
 *
 * A bound therefore does not commit the source program to:
 *
 *     scalar
 *     vector
 *     tensor
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     cluster
 *     distributed execution
 *     cloud execution
 *
 * Target realization remains separate from source type semantics.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * The following forms MUST be rejected structurally:
 *
 *     :
 *     : +
 *     : A +
 *     : + A
 *     : A + +
 *     : A + B +
 *
 * The following forms are structurally valid even if their names later fail
 * semantic resolution:
 *
 *     : UnknownType
 *
 *     : future::UnknownType
 *
 *     : future::UnknownType<Value>
 *
 * Unknown names are semantic errors, not grammar errors.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * For:
 *
 *     T: A + B + C
 *
 * the parser preserves:
 *
 *     A
 *     B
 *     C
 *
 * in source order.
 *
 * This grammar performs no:
 *
 *     sorting
 *     deduplication
 *     normalization
 *     canonicalization
 *     simplification
 *     evaluation
 *     resolution
 *
 * Those operations belong downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no embedded Rust
 *     no mutable global state
 *     no randomness
 *     no I/O
 *     no environment access
 *     no hardware access
 *     no resource discovery
 *     no runtime execution
 *
 * Therefore parsing is deterministic for a fixed token stream and fixed
 * grammar vocabulary.
 *
 * ============================================================================
 * RESOURCE SAFETY
 * ============================================================================
 *
 * No language-level maximum is encoded here.
 *
 * Implementations may independently enforce configurable operational limits
 * for:
 *
 *     source size
 *     parser work
 *     memory consumption
 *     diagnostic volume
 *     compilation time
 *
 * Such limits are implementation safety policies.
 *
 * They MUST NOT appear as grammar constants and MUST NOT redefine what
 * programs Zamani can express.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Historical type-bound syntax must be handled by:
 *
 *     grammar/compatibility/
 *
 * and not duplicated here.
 *
 * Compatibility aliases must converge on the same canonical bound structure.
 *
 * This file therefore does not provide alternate spellings for historical
 * constructs.
 *
 * ============================================================================
 * LEGACY OVERLAP
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/types/constraints.g4
 *     grammar/types/type-constraints.g4
 *
 * Both files currently describe overlapping type-bound responsibilities.
 *
 * Production architecture MUST establish:
 *
 *     bounds.g4
 *          |
 *          v
 *     canonical type-bound implementation
 *
 * while the overlapping files become compatibility/deprecation boundaries.
 *
 * They MUST NOT remain independent competing definitions of:
 *
 *     typeBound
 *     typeBoundList
 *     typeConstraintClause
 *
 * The canonical names supplied here are:
 *
 *     typeBoundClause
 *     typeBoundList
 *     typeBound
 *
 * A compatibility facade may expose an old public name, but it must delegate
 * to these canonical rules rather than reimplementing them.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/types/types.g4
 *
 * CONSUMES:
 *
 *     COLON
 *     PLUS
 *     typeExpression
 *
 * EXPORTS:
 *
 *     typeBoundClause
 *     typeBoundList
 *     typeBound
 *
 * AST_OWNER:
 *
 *     frontend AST/type representation
 *
 * SEMANTIC_OWNER:
 *
 *     semantic type/constraint system
 *
 * IR_OWNER:
 *
 *     canonical semantic model / target-independent IR
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/ and resource semantic analysis
 *
 * CAPABILITY_OWNER:
 *
 *     grammar/resources/ and capability semantic analysis
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/
 *
 * EFFECT_OWNER:
 *
 *     grammar/effects/
 *
 * PROVENANCE_OWNER:
 *
 *     grammar/spec/provenance.md and frontend source-span/provenance layer
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ and repository parser/type conformance tests
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/types.md
 *     grammar/specification/
 *
 * ============================================================================
 * INTEGRATION WITH TYPES.G4
 * ============================================================================
 *
 * `types.g4` remains the sole owner of:
 *
 *     typeExpression
 *
 * The composition target is:
 *
 *     Types
 *       |
 *       +--> TypeBounds
 *       |
 *       +--> Generic
 *       |
 *       +--> Named
 *       |
 *       +--> ...
 *
 * `types.g4` MUST NOT duplicate the bound-list implementation.
 *
 * If `types.g4` currently contains local bound rules, those rules must be
 * removed or converted into direct delegation to this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH GENERIC.G4
 * ============================================================================
 *
 * `generic.g4` owns generic type APPLICATION:
 *
 *     Type<Arguments>
 *
 * It does NOT own generic parameter bounds.
 *
 * Generic declaration grammars consume:
 *
 *     typeBoundClause?
 *
 * for example conceptually:
 *
 *     genericParameter
 *         : IDENTIFIER typeBoundClause?
 *         ;
 *
 * The exact generic-parameter declaration remains owned by the declaration
 * grammar used by the repository.
 *
 * ============================================================================
 * INTEGRATION WITH DEPENDENT.G4
 * ============================================================================
 *
 * `dependent.g4` owns dependent/value-parameterized type syntax.
 *
 * This grammar consumes that syntax indirectly through:
 *
 *     typeExpression
 *
 * It must not duplicate dependent-value expression rules.
 *
 * ============================================================================
 * INTEGRATION WITH TYPE-CLASS.G4
 * ============================================================================
 *
 * `type-class.g4` owns type-class/trait declaration semantics and syntax.
 *
 * A bound may reference a type-class-like type expression.
 *
 * This grammar does not decide whether a bound denotes:
 *
 *     trait
 *     interface
 *     structural property
 *     subtype
 *     type predicate
 *
 * ============================================================================
 * INTEGRATION WITH ASSOCIATED.G4
 * ============================================================================
 *
 * Associated-type syntax remains owned by:
 *
 *     grammar/types/associated.g4
 *
 * Bounds consume associated-type structures only through canonical
 * `typeExpression`.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCE/CAPABILITY SYSTEM
 * ============================================================================
 *
 * Do NOT put:
 *
 *     requires memory >= ...
 *     requires qubits >= ...
 *     requires capability(...)
 *     requires topology(...)
 *
 * into this file.
 *
 * Those are resource/capability requirements, not type bounds.
 *
 * The semantic layers may correlate them:
 *
 *     type satisfiability
 *              +
 *     resource feasibility
 *              +
 *     capability availability
 *              +
 *     policy
 *              =
 *     executable plan
 *
 * but the grammar ownership remains separated.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * A quantum type may appear as a bound without changing this file.
 *
 * Examples:
 *
 *     T: quantum::State
 *     T: quantum::Observable
 *     T: quantum::Circuit
 *     T: quantum::LogicalQubit
 *
 * Quantum operations remain outside this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH HDL / HARDWARE
 * ============================================================================
 *
 * Hardware and HDL types may appear as bounds:
 *
 *     T: hardware::Signal
 *     T: hardware::Memory
 *     T: hardware::Accelerator
 *     T: hdl::Module
 *
 * This does not select a physical implementation.
 *
 * ============================================================================
 * INTEGRATION WITH DATA / AI / REASONING
 * ============================================================================
 *
 * Generic computational concepts such as:
 *
 *     Model
 *     Dataset
 *     Evidence
 *     Distribution
 *     Knowledge
 *
 * are ordinary types.
 *
 * Therefore:
 *
 *     T: ai::Model
 *
 * requires no special rule here.
 *
 * ============================================================================
 * INTEGRATION WITH DISTRIBUTED / NETWORKING
 * ============================================================================
 *
 * Distributed and networking types are ordinary canonical type expressions.
 *
 * Examples:
 *
 *     T: distributed::Message
 *     T: distributed::Actor
 *     T: networking::Endpoint
 *
 * Physical node counts and network topology remain resource/runtime concerns.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following positive parser tests are mandatory:
 *
 *     T: Numeric
 *     T: Numeric + Comparable
 *     T: quantum::State
 *     T: quantum::State + quantum::Measurable
 *     T: collections::Iterable<Value>
 *     T: hardware::Accelerator<Model>
 *     T: future::domain::Capability
 *
 * The following negative parser tests are mandatory:
 *
 *     :
 *     : +
 *     : A +
 *     : + A
 *     : A + +
 *     : A + B +
 *
 * The following semantic-error tests must still parse successfully:
 *
 *     T: UnknownType
 *     T: future::UnknownType
 *     T: future::UnknownType<Value>
 *
 * The following scalability tests are mandatory:
 *
 *     many bounds in one clause;
 *     deeply nested canonical type expressions;
 *     generic bounds containing generic types;
 *     bounds containing dependent/value-parameterized types;
 *     bounds containing qualified names;
 *     cross-domain type bounds;
 *     large source files;
 *     repeated bound clauses.
 *
 * No test may assert a finite language-level bound count.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * At minimum, the conformance suite must exercise bounds involving:
 *
 *     classical
 *     numerical
 *     data
 *     AI/model types
 *     knowledge/evidence types
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     distributed
 *     networking
 *     future qualified types
 *
 * All must use the same `typeExpression` boundary.
 *
 * ============================================================================
 * DETERMINISM TEST
 * ============================================================================
 *
 * Given identical source and identical lexer/parser configuration:
 *
 *     parse(source)
 *
 * must produce the same syntactic structure and source locations.
 *
 * No hardware availability, resource state, randomness, clock, network state,
 * or runtime state may influence parsing.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE only when:
 *
 * [ ] It is the sole implementation owner of type-bound list syntax.
 *
 * [ ] It defines no competing type-expression grammar.
 *
 * [ ] It defines no lexer rules.
 *
 * [ ] It introduces no domain-specific bound catalogue.
 *
 * [ ] It introduces no physical resource limits.
 *
 * [ ] It introduces no target-specific assumptions.
 *
 * [ ] It preserves bound source ordering.
 *
 * [ ] It accepts one or more bounds.
 *
 * [ ] It rejects incomplete bound clauses.
 *
 * [ ] It accepts unresolved type names for later semantic diagnostics.
 *
 * [ ] It composes with canonical `types.g4`.
 *
 * [ ] Generic declaration grammars can consume `typeBoundClause`.
 *
 * [ ] Existing overlapping bound grammars delegate to this implementation
 *     or are explicitly deprecated.
 *
 * [ ] The frontend preserves each bound as canonical TypeExpr structure.
 *
 * [ ] Semantic analysis owns bound interpretation.
 *
 * [ ] Resource/capability analysis remains separate.
 *
 * [ ] Quantum lowering remains behind `quantum::ir`.
 *
 * [ ] Rust integration remains compatible with Rust 1.97+.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Scalability tests pass within configurable implementation resources.
 *
 * ============================================================================
 * CANONICAL RULES
 * ============================================================================
 */

parser grammar TypeBounds;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC BOUND CLAUSE
 * ============================================================================
 *
 * Examples:
 *
 *     : Numeric
 *     : Numeric + Comparable
 *     : quantum::State
 *
 * The surrounding generic/declaration grammar owns the subject preceding the
 * colon.
 */
typeBoundClause
    : COLON typeBoundList
    ;


/*
 * ============================================================================
 * ORDERED TYPE-BOUND LIST
 * ============================================================================
 *
 * There is deliberately no grammar-level maximum.
 *
 * The `*` permits:
 *
 *     A
 *     A + B
 *     A + B + C
 *     ...
 *
 * Every member is a complete canonical type expression.
 */
typeBoundList
    : typeBound
      (PLUS typeBound)*
    ;


/*
 * ============================================================================
 * SINGLE TYPE BOUND
 * ============================================================================
 *
 * The bound is exactly one canonical type expression.
 *
 * IMPORTANT:
 *
 * `typeExpression` is supplied by the canonical type-expression composition
 * root. It is intentionally NOT redefined here.
 */
typeBound
    : typeExpression
    ;