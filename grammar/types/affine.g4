/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/affine.g4
 *
 * Grammar:
 *     AffineTypes
 *
 * Status:
 *     PRODUCTION
 *
 * Language role:
 *     Source-level affine type qualifier.
 *
 * Compiler baseline:
 *     Rust 1.97+
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     Declarative ANTLR grammar only.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No parser actions.
 *     No unsafe code.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns exactly one source-level type qualifier:
 *
 *     affine
 *
 * It recognizes the lexical token:
 *
 *     AFFINE
 *
 * and exports the parser rule:
 *
 *     affineQualifier
 *
 * The operand type is deliberately NOT parsed here.
 *
 * Complete type composition is owned by:
 *
 *     grammar/types/types.g4
 *
 * The canonical source-level structure is therefore:
 *
 *     affine T
 *
 *       affineQualifier
 *       +
 *       canonical typeExpression
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Affine typing is an ownership/usage discipline.
 *
 * At the semantic level, an affine value is normally permitted to have:
 *
 *     zero or one consuming use
 *
 * The exact rules for:
 *
 *     ownership
 *     moves
 *     copies
 *     drops
 *     borrowing
 *     aliasing
 *     lifetime
 *     destruction
 *     resource consumption
 *
 * belong to semantic analysis.
 *
 * This grammar records only the source-level qualifier.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     affineQualifier
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     typePrefix
 *     typePostfix
 *     named types
 *     qualified names
 *     generic arguments
 *     tuples
 *     arrays
 *     slices
 *     functions
 *     references
 *     pointers
 *     dependent types
 *     associated types
 *     type classes
 *     linear types
 *     quantum types
 *     classical types
 *     HDL types
 *     hardware types
 *     resource types
 *     capability types
 *     temporal types
 *     effects
 *     contracts
 *     policies
 *     provenance
 *     ownership checking
 *     borrow checking
 *     lifetime checking
 *     move checking
 *     copy checking
 *     resource checking
 *     capability negotiation
 *     target selection
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT DEPENDENCY:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume the public token vocabulary exposed by
 * ZamaniLexer.
 *
 * This file MUST NOT define lexer rules.
 *
 * The lexical spelling:
 *
 *     affine
 *
 * and the token:
 *
 *     AFFINE
 *
 * belong to the canonical lexer hierarchy.
 *
 * INDIRECT LEXICAL OWNERS:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/lexer.g4
 *
 * This file must not import those lexical component grammars directly.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It therefore uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It exports only:
 *
 *     affineQualifier
 *
 * The canonical type orchestrator:
 *
 *     grammar/types/types.g4
 *
 * consumes this rule.
 *
 * Dependency direction:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     AffineTypes
 *          |
 *          | affineQualifier
 *          v
 *     Type
 *          |
 *          | typeExpression
 *          v
 *     frontend AST
 *
 * This file MUST NOT import the canonical type grammar.
 *
 * In particular, this file MUST NOT contain:
 *
 *     import Type;
 *
 * or any equivalent dependency on the complete type-expression grammar.
 *
 * This prevents:
 *
 *     Type -> AffineTypes -> Type
 *
 * circularity.
 *
 * ============================================================================
 * PUBLIC GRAMMAR API
 * ============================================================================
 *
 * Public rule:
 *
 *     affineQualifier
 *
 * Input:
 *
 *     AFFINE
 *
 * Output:
 *
 *     one affine qualifier node in the parser tree
 *
 * The rule consumes exactly one AFFINE token.
 *
 * It deliberately does not consume the following type.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AffineTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * AFFINE QUALIFIER
 * ============================================================================
 *
 * Source:
 *
 *     affine
 *
 * Parser representation:
 *
 *     affineQualifier
 *
 * The enclosing canonical type grammar supplies the type being qualified.
 *
 * Examples at the composed-language level include:
 *
 *     affine T
 *     affine User
 *     affine module::User
 *     affine Vec<T>
 *     affine Result<T, E>
 *     affine Tensor<T, Shape>
 *     affine Qubit
 *     affine Resource<T>
 *
 * This rule intentionally does not know any of those type constructors.
 *
 * Therefore adding a future type constructor does not require modifying
 * this file.
 * ============================================================================
 */

affineQualifier
    : AFFINE
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not construct Rust AST values directly.
 *
 * The canonical frontend AST owner is:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * The canonical affine representation is:
 *
 *     TypeExpr::Affine(inner)
 *
 * The parser/frontend integration is responsible for combining:
 *
 *     affineQualifier
 *
 * with:
 *
 *     typeExpression
 *
 * and constructing the equivalent:
 *
 *     TypeExpr::Affine(...)
 *
 * structure.
 *
 * No second affine AST node is permitted.
 *
 * This file MUST NOT introduce:
 *
 *     AffineTypeExpr
 *     AffineType
 *     OwnershipTypeExpr
 *     ResourceAffineType
 *     QuantumAffineType
 *
 * as competing universal AST representations.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing:
 *
 *     affine T
 *
 * establishes only that the source requests affine qualification.
 *
 * Semantic analysis determines:
 *
 *     - whether T is a valid affine-qualified type;
 *     - whether affine qualification is permitted for T;
 *     - how ownership is tracked;
 *     - how many consuming uses are permitted;
 *     - whether copying is legal;
 *     - whether moving is legal;
 *     - whether borrowing is legal;
 *     - whether aliases are legal;
 *     - whether destruction consumes the value;
 *     - whether generic constraints permit the qualification;
 *     - whether nested ownership qualifiers are valid;
 *     - whether resource semantics impose additional requirements.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * LINEAR TYPE INTEGRATION
 * ============================================================================
 *
 * The related source-level qualifier:
 *
 *     linear
 *
 * is owned by:
 *
 *     grammar/types/linear.g4
 *
 * That file exports:
 *
 *     linearQualifier
 *
 * This file exports:
 *
 *     affineQualifier
 *
 * The canonical type orchestrator combines both qualifiers.
 *
 * Conceptually:
 *
 *     typeExpression
 *         : typePrefix* typeCore typePostfix*
 *         ;
 *
 *     typePrefix
 *         : linearTypePrefix
 *         | affineTypePrefix
 *         | ...
 *         ;
 *
 *     linearTypePrefix
 *         : linearQualifier
 *         ;
 *
 *     affineTypePrefix
 *         : affineQualifier
 *         ;
 *
 * Linear and affine semantics must remain distinct downstream:
 *
 *     linear
 *         exactly one required consuming use
 *
 *     affine
 *         zero or one consuming use
 *
 * These descriptions are semantic contracts, not parser enforcement.
 *
 * ============================================================================
 * QUALIFIER COMPOSITION
 * ============================================================================
 *
 * This file MUST NOT parse:
 *
 *     affine typeExpression
 *
 * directly.
 *
 * Doing so would make this component responsible for the complete type
 * expression and create a dependency on its own parent.
 *
 * Correct composition is performed by:
 *
 *     grammar/types/types.g4
 *
 * Example:
 *
 *     affine Vec<T>
 *
 * is structurally composed as:
 *
 *     affineQualifier
 *     typeCore
 *
 * where the canonical type grammar determines how Vec<T> is represented.
 *
 * ============================================================================
 * GENERIC TYPE INTEGRATION
 * ============================================================================
 *
 * Affine qualification is independent of generic syntax.
 *
 * Consequently the following forms can be composed whenever the canonical
 * type grammar accepts their underlying type:
 *
 *     affine Vec<T>
 *     affine Map<K, V>
 *     affine Result<T, E>
 *     affine Model<T>
 *     affine Dataset<T>
 *     affine Tensor<T, Shape>
 *     affine Resource<R>
 *
 * This grammar does not enumerate generic constructors.
 *
 * Generic ownership belongs to the canonical generic/type machinery.
 *
 * ============================================================================
 * SYMBOLIC / DEPENDENT TYPE INTEGRATION
 * ============================================================================
 *
 * Affine qualification must remain compatible with symbolic type parameters
 * and value parameters.
 *
 * Examples:
 *
 *     affine Vector<T, N>
 *     affine Matrix<T, Rows, Columns>
 *     affine Tensor<T, Shape>
 *
 * This grammar does not:
 *
 *     - evaluate N;
 *     - evaluate Rows;
 *     - evaluate Columns;
 *     - evaluate Shape;
 *     - convert symbolic values to host integers;
 *     - impose bounds;
 *     - impose machine limits;
 *     - select storage;
 *     - select hardware.
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * REFERENCE AND BORROWING INTEGRATION
 * ============================================================================
 *
 * Reference syntax is owned by the canonical reference grammar.
 *
 * This file must not define:
 *
 *     referenceType
 *     referencePrefix
 *     lifetimeAnnotation
 *     borrowType
 *
 * The composed type system may therefore encounter source structures such as:
 *
 *     affine &T
 *     affine &mut T
 *
 * where supported by the canonical type grammar.
 *
 * Whether such combinations are semantically valid is determined by the
 * ownership, borrowing, and lifetime systems.
 *
 * ============================================================================
 * POINTER INTEGRATION
 * ============================================================================
 *
 * Pointer syntax is owned by the pointer type grammar.
 *
 * This file does not define:
 *
 *     pointerType
 *     *T
 *     *mut T
 *     *const T
 *
 * If an affine-qualified pointer is legal, that legality is determined by
 * semantic analysis after normal type composition.
 *
 * No pointer representation is implied by this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Affine ownership can be useful for semantic management of quantum values
 * and resources.
 *
 * Examples of source-level types that may be qualified include:
 *
 *     affine Qubit
 *     affine LogicalQubit
 *     affine QRegister<N>
 *     affine QuantumState<T>
 *     affine QuantumResource<R>
 *
 * These names are not defined by this file.
 *
 * The grammar only recognizes:
 *
 *     affine
 *
 * The quantum subsystem remains responsible for semantic interpretation.
 *
 * If the resulting type participates in quantum computation, the downstream
 * quantum pipeline remains:
 *
 *     source AST
 *         |
 *         v
 *     semantic quantum model
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
 * This file MUST NOT:
 *
 *     - allocate qubits;
 *     - identify physical qubits;
 *     - select a QPU;
 *     - inspect coupling topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - choose calibration;
 *     - choose QEC;
 *     - construct pulses.
 *
 * ============================================================================
 * CLASSICAL / AI / DATA INTEGRATION
 * ============================================================================
 *
 * Affine qualification is domain-neutral.
 *
 * It may therefore qualify ordinary or domain-defined types used for:
 *
 *     classical computation;
 *     numerical computation;
 *     scientific computation;
 *     data processing;
 *     models;
 *     datasets;
 *     reasoning state;
 *     evidence;
 *     probabilistic values;
 *     learned state;
 *     agent state.
 *
 * No domain-specific type catalogue belongs here.
 *
 * Domain semantics remain downstream of the common type system.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Affine qualification may be applied to source-level types used by HDL or
 * hardware/software co-design where the semantic type system permits it.
 *
 * This grammar does not encode:
 *
 *     signal widths;
 *     register widths;
 *     memory capacities;
 *     physical addresses;
 *     device identifiers;
 *     FPGA resources;
 *     ASIC resources;
 *     placement;
 *     timing closure;
 *     routing;
 *     synthesis decisions.
 *
 * Those responsibilities belong to:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/resources/
 *
 * and their downstream semantic/compiler systems.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO resource requirement.
 *
 * In particular, it does not specify:
 *
 *     memory
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     qubit count
 *     node count
 *     thread count
 *     device count
 *     tensor rank
 *     register width
 *     network size
 *
 * Affine ownership is a language-semantic property, not a hardware capacity.
 *
 * A semantic type may later participate in resource analysis, but that
 * analysis is separate from parsing.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Parsing an affine qualifier requires no target capability.
 *
 * A downstream semantic operation involving an affine-qualified resource may
 * require capabilities.
 *
 * Capability resolution belongs to the resource/capability subsystem.
 *
 * This grammar must never inspect:
 *
 *     CPU availability
 *     GPU availability
 *     FPGA availability
 *     QPU availability
 *     network availability
 *     hardware generation
 *     vendor identity
 *     physical topology
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * The affine qualifier itself has no runtime effect.
 *
 * It is declarative type information.
 *
 * Operations performed on affine-qualified values may produce effects, but
 * those effects belong to the canonical effect system.
 *
 * This grammar therefore does not declare or execute:
 *
 *     allocation
 *     deallocation
 *     mutation
 *     I/O
 *     networking
 *     native execution
 *     foreign execution
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *
 * ============================================================================
 * CONTRACT / POLICY INTEGRATION
 * ============================================================================
 *
 * Affine-qualified types may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * and policy-controlled ownership/resource decisions.
 *
 * The corresponding grammar owners remain:
 *
 *     grammar/validation/
 *     grammar/policies/
 *
 * This file does not duplicate those grammars.
 *
 * Semantic analysis may use affine information when checking contracts and
 * policies.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser must preserve the source location of the AFFINE token through
 * the normal parser/frontend source-span mechanism.
 *
 * The enclosing AST builder must preserve sufficient source information for:
 *
 *     diagnostics
 *     IDE/LSP
 *     formatting
 *     source maps
 *     incremental compilation
 *     provenance
 *     compatibility analysis
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * POCO-REAF SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar has no artificial capacity limit.
 *
 * It MUST NOT define or depend on limits such as:
 *
 *     MAX_AFFINE_VALUES
 *     MAX_AFFINE_TYPES
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_TENSOR_RANK
 *     MAX_RESOURCE_COUNT
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also must not enumerate a closed collection of types that may be
 * affine-qualified.
 *
 * The scalable property is:
 *
 *     affine + canonical typeExpression
 *
 * Therefore a future type constructor can participate without changing this
 * grammar, provided the canonical type system supports it.
 *
 * "Infinity" means that the language imposes no artificial physical-capacity
 * ceiling through this grammar. Actual execution remains bounded by available
 * computational resources and implementation limits.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing:
 *
 *     affine
 *
 * must be deterministic.
 *
 * Recognition depends only on:
 *
 *     source tokens
 *     lexical vocabulary
 *     grammar
 *     selected language/compatibility version
 *
 * It must not depend on:
 *
 *     hardware
 *     resources
 *     target selection
 *     network state
 *     filesystem state
 *     scheduler state
 *     wall-clock time
 *     random state
 *     runtime state
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar is declarative and side-effect free.
 *
 * It contains:
 *
 *     - no embedded executable code;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware access;
 *     - no command execution;
 *     - no dynamic evaluation.
 *
 * Parser resource limits used to defend against hostile input are compiler
 * configuration concerns and must not change the meaning of affine typing.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors are limited to malformed source at the lexical/parser level.
 *
 * Examples of parser-level errors include:
 *
 *     malformed AFFINE token stream
 *     invalid token sequence around the qualifier
 *
 * The following are NOT parser errors produced by this file:
 *
 *     unknown type
 *     invalid ownership use
 *     duplicate consumption
 *     illegal copy
 *     invalid borrow
 *     unsatisfied type constraint
 *     unavailable capability
 *     insufficient resource
 *     unsupported hardware
 *
 * Those are semantic/compiler diagnostics.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable source spelling:
 *
 *     affine
 *
 * Stable lexical token:
 *
 *     AFFINE
 *
 * Stable parser rule:
 *
 *     affineQualifier
 *
 * The rule must remain source-compatible unless a deliberate language-version
 * migration changes the normative type syntax.
 *
 * Compatibility handling belongs to:
 *
 *     grammar/compatibility/
 *
 * This file must not implement version-dependent semantic behavior.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Tests owned by the affine feature should verify the parser boundary.
 *
 * POSITIVE:
 *
 *     affine T
 *     affine User
 *     affine module::User
 *     affine Vec<T>
 *     affine Result<T, E>
 *     affine Tensor<T, Shape>
 *     affine Qubit
 *     affine Resource<T>
 *
 * The test harness should parse these through the canonical type-expression
 * entry point rather than treating affineQualifier as a substitute for the
 * complete type grammar.
 *
 * NEGATIVE:
 *
 *     affine
 *
 * when a complete type is required by the enclosing type grammar.
 *
 * Additional semantic-negative cases belong to the semantic/type-checking
 * test suite, not this parser grammar.
 *
 * BOUNDARY:
 *
 *     linear T
 *     affine T
 *     affine linear T
 *     linear affine T
 *
 * where the canonical type system determines whether combinations are legal.
 *
 * CROSS-DOMAIN:
 *
 *     affine classical_type
 *     affine quantum_type
 *     affine hardware_type
 *     affine Resource<T>
 *
 * where those underlying types are available through the canonical type
 * grammar.
 *
 * SCALABILITY:
 *
 * Use symbolic and nested type constructions rather than tests containing
 * fixed machine capacities.
 *
 * DETERMINISM:
 *
 * Parsing the same token sequence repeatedly must produce equivalent parse
 * structure and diagnostics.
 *
 * ============================================================================
 * INTEGRATION MATRIX
 * ============================================================================
 *
 * SPECIFICATION
 *     grammar/spec/type-system.md
 *     grammar/specification/types.md
 *
 * LEXER
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/
 *
 * GRAMMAR
 *     grammar/types/affine.g4
 *     grammar/types/types.g4
 *
 * AST
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * SEMANTICS
 *     ownership / type-system semantic implementation
 *
 * CONSTRAINTS
 *     grammar/types/constraints.g4
 *     downstream constraint solver
 *
 * EFFECTS
 *     grammar/effects/
 *
 * RESOURCES
 *     grammar/resources/
 *
 * CAPABILITIES
 *     grammar/resources/
 *     downstream capability analysis
 *
 * CONTRACTS
 *     grammar/validation/
 *
 * POLICIES
 *     grammar/policies/
 *
 * PROVENANCE
 *     compiler/frontend provenance infrastructure
 *
 * QUANTUM
 *     semantic quantum type handling
 *     quantum::ir
 *
 * HDL/HARDWARE
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * COMPILATION
 *     grammar/compile/
 *
 * EXECUTION
 *     grammar/execution/
 *
 * TARGET REALIZATION
 *     downstream compiler/backend/HAL systems
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 * [x] It is a parser grammar.
 *
 * [x] It uses the canonical ZamaniLexer vocabulary.
 *
 * [x] It defines no lexer rules.
 *
 * [x] It exports exactly affineQualifier.
 *
 * [x] affineQualifier consumes exactly AFFINE.
 *
 * [x] It does not define typeExpression.
 *
 * [x] It does not parse the operand type.
 *
 * [x] It does not create a competing AST.
 *
 * [x] It does not implement semantic ownership checking.
 *
 * [x] It does not implement resource negotiation.
 *
 * [x] It does not implement capability negotiation.
 *
 * [x] It does not select a target.
 *
 * [x] It does not contain hardware limits.
 *
 * [x] It does not enumerate supported affine-qualified types.
 *
 * [x] It is independent of quantum physical realization.
 *
 * [x] It is independent of HDL physical realization.
 *
 * [x] It is independent of AI/data implementation.
 *
 * [x] It preserves source-level extensibility.
 *
 * [x] It is deterministic.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is compatible with the Rust 1.97+ compiler architecture because it
 *     contains no Rust implementation code.
 *
 * [x] It has a documented downstream integration contract.
 *
 * ============================================================================
 * END OF FEATURE CONTRACT
 * ============================================================================
 */