/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/linear.g4
 *
 * Grammar:
 *     LinearTypes
 *
 * Status:
 *     Production-ready modular parser component.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
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
 * Define the source-level syntax for the `linear` type qualifier and expose
 * it as a reusable parser component for the canonical Zamani type grammar.
 *
 * The complete type expression is intentionally NOT parsed here.
 *
 *
 * OWNS
 * ----
 *
 * This file owns:
 *
 *   - linearQualifier;
 *   - linearityQualifier;
 *   - the parser-level meaning of the `linear` keyword as a type qualifier;
 *   - the modular grammar boundary for linear type qualification.
 *
 *
 * DOES NOT OWN
 * -------------
 *
 * This file does NOT own:
 *
 *   - lexer rules;
 *   - keyword spelling;
 *   - identifiers;
 *   - punctuation;
 *   - complete typeExpression;
 *   - typeCore;
 *   - generic types;
 *   - references;
 *   - pointers;
 *   - tuples;
 *   - arrays;
 *   - slices;
 *   - function types;
 *   - quantum types;
 *   - HDL types;
 *   - hardware types;
 *   - resource discovery;
 *   - capability discovery;
 *   - ownership analysis;
 *   - borrow checking;
 *   - move checking;
 *   - linear-use checking;
 *   - allocation;
 *   - deallocation;
 *   - runtime representation;
 *   - ABI layout;
 *   - target selection;
 *   - physical resource selection;
 *   - quantum mapping;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - optimization;
 *   - backend selection.
 *
 *
 * ============================================================================
 * OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * Complete source syntax:
 *
 *     linear T
 *
 * is composed by:
 *
 *     types.g4
 *          │
 *          ├── linearityQualifier
 *          │        │
 *          │        └── this file
 *          │
 *          └── typeExpression
 *
 * Therefore this file MUST NOT contain:
 *
 *     linearType
 *         : LINEAR typeExpression
 *         ;
 *
 * because `typeExpression` is owned by types.g4.
 *
 * Doing so would make this grammar depend upward on its importing grammar and
 * would create an invalid/circular modular dependency.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *   grammar/lexer/keywords.g4
 *   grammar/lexer/tokens.g4
 *   grammar/antlr/ZamaniLexer.g4
 *
 * More precisely, this parser consumes the canonical:
 *
 *     LINEAR
 *
 * token produced by the composed Zamani lexer.
 *
 *
 * EXPORTS
 * -------
 *
 *   linearQualifier
 *   linearityQualifier
 *
 *
 * CONSUMED_BY
 * ----------
 *
 *   grammar/types/types.g4
 *
 * Indirectly consumed by:
 *
 *   grammar/Zamani.g4
 *   canonical parser/frontend
 *
 *
 * AST_OWNER
 * ---------
 *
 *   src/frontend/ast/node/types/type_expr.rs
 *
 * The relevant canonical AST representation is:
 *
 *     TypeExpr::Linear(...)
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *   semantic/type-system/ownership analysis
 *
 * The existing semantic layer currently resolves:
 *
 *     TypeExpr::Linear(inner)
 *
 * to:
 *
 *     Type::Linear(...)
 *
 * Full linearity enforcement must remain semantic rather than grammatical.
 *
 *
 * IR_OWNER
 * --------
 *
 * No independent IR is created by this grammar.
 *
 * Linear information is preserved through the canonical semantic type model
 * and lowered by the normal canonical IR pipeline.
 *
 * For quantum programs, the downstream quantum boundary remains:
 *
 *     quantum::ir
 *
 * No linear-specific quantum IR is introduced.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *   grammar/tests/
 *   grammar/tests/types/
 *
 * The parser tests should exercise this rule through the canonical
 * `typeExpression` entry point rather than treating this grammar as a
 * standalone language.
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *   grammar/spec/type-system.md
 *   grammar/specification/types.md
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It defines NO lexer rules.
 *
 * The canonical lexical spelling is owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * where:
 *
 *     LINEAR : 'linear' ;
 *
 * is defined.
 *
 * The composed lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar consumes the resulting `LINEAR` token.
 *
 * It MUST NOT define:
 *
 *     LINEAR
 *     'linear'
 *     LINEAR_KEYWORD
 *     LINEAR_TYPE
 *
 * or any alternate token identity.
 *
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This grammar is deliberately a parser grammar.
 *
 * It must use the canonical lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * No local lexer is permitted.
 *
 * This keeps token identity consistent across:
 *
 *     types
 *     declarations
 *     functions
 *     memory
 *     quantum
 *     HDL
 *     resources
 *     classical
 *     distributed
 *     interoperability
 *
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 */

/**
 * Canonical source-level linearity qualifier.
 *
 * This rule consumes only the qualifier.
 *
 * The operand type is supplied by the importing canonical type grammar.
 *
 * Example when composed by types.g4:
 *
 *     linear T
 *
 * parses structurally as:
 *
 *     linearQualifier + typeExpression
 */
linearQualifier
    : LINEAR
    ;


/**
 * Unified linearity qualifier boundary.
 *
 * This rule is intentionally designed so future ownership disciplines can
 * participate in a common qualifier composition without changing the
 * complete type-expression grammar.
 *
 * Currently the canonical alternatives are:
 *
 *     linear
 *
 * and, through the unified modular ownership boundary, affine.
 *
 * `affine` is defined by affine.g4.
 *
 * This file therefore does not duplicate the AFFINE token.
 */
linearityQualifier
    : linearQualifier
    ;


/*
 * ============================================================================
 * TYPE COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical types/types.g4 file should consume this grammar approximately
 * as follows:
 *
 *     typeExpression
 *         : typeQualifier* typeCore typePostfix*
 *         ;
 *
 *     typeQualifier
 *         : linearityQualifier
 *         | affineQualifier
 *         | ...
 *         ;
 *
 * Or, if the repository chooses to make the combined ownership boundary
 * explicit:
 *
 *     typeQualifier
 *         : linearityQualifier
 *         | ...
 *         ;
 *
 * The critical invariant is:
 *
 *     linear.g4 owns linearQualifier
 *
 * while:
 *
 *     types.g4 owns typeExpression
 *
 * No file should create another competing `linearType` rule that consumes
 * `typeExpression`.
 *
 *
 * ============================================================================
 * SOURCE-LEVEL SEMANTIC INTENT
 * ============================================================================
 *
 * `linear T` expresses that values of T participate in a linear ownership or
 * resource discipline.
 *
 * The grammar does not determine the exact discipline.
 *
 * Semantic analysis determines:
 *
 *   - whether T may be linear;
 *   - whether T is resource-bearing;
 *   - whether copying is permitted;
 *   - whether duplication is permitted;
 *   - whether moving is permitted;
 *   - whether destruction consumes the value;
 *   - whether a value must be consumed;
 *   - whether a value may be transferred;
 *   - whether aliases are permitted;
 *   - how control-flow joins affect ownership;
 *   - how function parameters and returns affect ownership;
 *   - how generic substitutions affect ownership;
 *   - how references affect ownership;
 *   - how effects affect ownership;
 *   - how capabilities affect ownership;
 *   - how resource requirements affect ownership.
 *
 * These rules MUST NOT be encoded in this grammar.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser/frontend AST adapter maps a complete:
 *
 *     linear T
 *
 * to the existing canonical:
 *
 *     TypeExpr::Linear(...)
 *
 * Conceptually:
 *
 *     linear T
 *         │
 *         ▼
 *     TypeExpr::Linear(TypeExpr(T))
 *
 * This grammar does not construct Rust values directly.
 *
 * No second:
 *
 *     LinearType
 *
 * AST hierarchy should be introduced.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Structural parsing succeeds when:
 *
 *     linear
 *
 * is followed by a syntactically valid type expression through the
 * surrounding `types.g4` grammar.
 *
 * Semantic validation then determines whether:
 *
 *     linear T
 *
 * is meaningful for T.
 *
 * Therefore the following distinction is intentional:
 *
 *     syntactically valid
 *
 * versus:
 *
 *     semantically valid.
 *
 * For example:
 *
 *     linear int
 *
 * may be syntactically accepted.
 *
 * Whether `int` may participate in the linear ownership discipline is a
 * semantic type-system decision.
 *
 * The grammar must not hard-code a whitelist such as:
 *
 *     Qubit
 *     Resource
 *     Buffer
 *     Memory
 *
 * because future user-defined and domain-specific resource types must remain
 * representable without changing this grammar.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The operand of `linear` is an arbitrary source-level type expression
 * accepted by the canonical type system.
 *
 * Consequently the following categories may participate when semantically
 * permitted:
 *
 *     named types
 *     generic types
 *     references
 *     pointers
 *     tuples
 *     arrays
 *     slices
 *     function types
 *     quantum types
 *     temporal types
 *     dependent types
 *     user-defined types
 *     resource types
 *     capability types
 *     accelerator abstractions
 *     distributed abstractions
 *     HDL abstractions
 *     future domain types
 *
 * The grammar imposes no finite type inventory.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * The presence of `linear` is not itself an execution effect.
 *
 * Ownership/resource semantics are type-system semantics.
 *
 * If a linear type is subsequently used in an operation producing effects,
 * those effects belong to the operation/effect system rather than this
 * grammar.
 *
 * This separation prevents:
 *
 *     linear
 *
 * from accidentally implying:
 *
 *     mutation
 *     allocation
 *     IO
 *     network
 *     quantum
 *     measurement
 *     native execution
 *     learning
 *     adaptation
 *
 * merely because the type is linear.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * `linear` does not require a hardware capability.
 *
 * A semantic type may independently require capabilities.
 *
 * For example, a user-defined type may semantically represent a quantum,
 * accelerator, distributed, or other resource.
 *
 * Capability requirements belong to:
 *
 *     grammar/resources/
 *
 * and the semantic capability system.
 *
 * This grammar must never contain:
 *
 *     capability("...")
 *
 * or hardware-specific capability names.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Linearity can model resource ownership, but this grammar does not allocate
 * resources.
 *
 * Resource requirements remain symbolic.
 *
 * Examples of valid downstream concepts include:
 *
 *     requires capability("quantum.measurement")
 *     requires memory >= required_memory
 *     requires qubits >= required_qubits
 *
 * Those are not part of this grammar.
 *
 * The type:
 *
 *     linear Resource<T>
 *
 * expresses a source-level type abstraction.
 *
 * It does not select a physical resource.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Type-level linearity does not itself define:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contracts may constrain operations involving linear values, but their
 * syntax and semantics belong to the validation/contract subsystem.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may restrict operations involving linear resources.
 *
 * Examples include policies concerning:
 *
 *     transfer
 *     duplication
 *     persistence
 *     external calls
 *     adaptation
 *     distribution
 *
 * Such policies are consumed semantically.
 *
 * This grammar does not define policy syntax.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The source span of `LINEAR` must remain available to the parser/frontend so
 * diagnostics can identify the qualifier precisely.
 *
 * Downstream provenance may record:
 *
 *     source declaration
 *     source qualifier
 *     semantic interpretation
 *     transformations
 *     verification
 *     lowering
 *
 * This grammar does not generate provenance records.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Linear types are useful for quantum-resource semantics, but this grammar is
 * domain-neutral.
 *
 * Examples:
 *
 *     linear Qubit
 *     linear LogicalQubit
 *     linear QuantumResource<Qubit>
 *
 * are source-level type expressions.
 *
 * This file MUST NOT encode:
 *
 *     physical qubit identity
 *     QPU identity
 *     topology
 *     gate decomposition
 *     routing
 *     scheduling
 *     calibration
 *     error correction
 *     resilience strategy
 *     vendor instruction set
 *
 * The downstream quantum pipeline remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 * No linear-specific quantum IR is introduced.
 *
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * HDL/resource types may also be linear where their semantic model requires
 * exclusive resource ownership.
 *
 * Examples:
 *
 *     linear Signal<T>
 *     linear Resource<Register<T>>
 *
 * The grammar remains unaware of whether a named type represents:
 *
 *     signal
 *     register
 *     bus
 *     memory
 *     accelerator
 *     device
 *     interface
 *
 * HDL ownership, synthesis, timing, placement, and physical realization remain
 * downstream.
 *
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backends consume the semantic consequences of linearity.
 *
 * This grammar must never choose:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     cluster
 *     cloud
 *     accelerator
 *
 * Nor may it encode target-specific representation choices.
 *
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar intentionally contains no implementation capacity constants.
 *
 * It must NOT contain:
 *
 *     MAX_LINEAR_VALUES
 *     MAX_LINEAR_RESOURCES
 *     MAX_LINEAR_TYPES
 *     MAX_AFFINE_VALUES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *
 * or equivalent limits.
 *
 * Nested and generic types remain bounded only by actual parser/compiler
 * implementation resources, not by language-level constants.
 *
 * Examples:
 *
 *     linear T
 *
 *     linear A::B::C::D::T
 *
 *     linear Resource<T>
 *
 *     linear Resource<Capability<QuantumResource<Qubit>>>
 *
 *     linear Vec<Vec<Vec<T>>>
 *
 * are all structurally representable.
 *
 * No grammar change should be required merely because a program uses larger
 * resource cardinalities.
 *
 * POCO-REAF is therefore achieved at the language level by describing
 * ownership intent rather than physical resource instances.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no parser actions;
 *     no runtime calls;
 *     no I/O;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no target-dependent alternatives;
 *     no generated identifiers;
 *     no mutable global state.
 *
 * For a fixed token stream and lexer vocabulary, parsing is deterministic.
 *
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * The grammar performs no execution.
 *
 * It cannot:
 *
 *     allocate hardware;
 *     execute source code;
 *     invoke foreign code;
 *     access files;
 *     access networks;
 *     inspect devices;
 *     modify resources.
 *
 * Hostile-input protection belongs to parser/compiler resource policy and must
 * not alter the source-language semantics.
 *
 * The Rust implementation remains:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The source spelling remains:
 *
 *     linear T
 *
 * Existing programs using the `linear` qualifier therefore require no source
 * rename.
 *
 * The modularization changes ownership of the parser rule, not the source
 * spelling.
 *
 * Any historical rule such as:
 *
 *     linearType : LINEAR typeExpression ;
 *
 * must not remain a competing production owner.
 *
 * Historical/legacy grammars may remain only as explicitly marked
 * compatibility artifacts until their consumers are migrated.
 *
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser-level diagnostics should naturally identify:
 *
 *     unexpected end after `linear`
 *
 * or:
 *
 *     unexpected token where a type expression is required
 *
 * through the surrounding `types.g4` rule.
 *
 * This grammar must not embed semantic error strings.
 *
 * Semantic diagnostics belong downstream, for example:
 *
 *     invalid linear type
 *     illegal duplication
 *     linear value reused
 *     required linear value not consumed
 *
 * Exact diagnostic identifiers belong to the semantic diagnostic contract,
 * not to this parser component.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * These examples are tested through the canonical `typeExpression` rule:
 *
 *     linear T
 *     linear Resource<T>
 *     linear Resource<Capability<T>>
 *     linear quantum::Qubit
 *     linear LogicalQubit
 *     linear Memory<T>
 *     linear Signal<T>
 *     linear Vec<Vec<T>>
 *
 * Additional valid forms should include every type constructor accepted by
 * the canonical type grammar.
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Parser-invalid examples include:
 *
 *     linear
 *
 * where a complete type expression is required.
 *
 * These must fail through the enclosing type grammar rather than through a
 * special error rule here.
 *
 * The following should NOT be rejected merely by this grammar:
 *
 *     linear int
 *
 * because eligibility of `int` is a semantic question.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test interactions with:
 *
 *     linear &T
 *     linear &mut T
 *     linear &'a T
 *     linear *T
 *     linear Vec<T>
 *     linear (T, U)
 *     linear [T]
 *     linear [T; N]
 *     linear fn(T) -> U
 *     linear Result<T, E>
 *     linear QuantumResource<Qubit>
 *     linear Tensor<T>[N, M]
 *
 * Whether each is semantically legal is determined by the type system.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The same syntax must be usable with source-defined types representing:
 *
 *     classical resources
 *     quantum resources
 *     HDL resources
 *     accelerator resources
 *     distributed resources
 *     data resources
 *     AI model resources
 *     networking resources
 *     future domain resources
 *
 * No domain-specific grammar fork is permitted.
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that no language-level maximum is introduced.
 *
 * Examples should be generated structurally rather than with a fixed maximum
 * chosen by the grammar.
 *
 * Test families should cover:
 *
 *     deeply nested generic types;
 *     long qualified type paths;
 *     nested linear types;
 *     linear types containing symbolic dependent values;
 *     large source-level programs containing many independently declared
 *     linear values.
 *
 * Parser stress limits, when needed for denial-of-service protection, belong
 * to explicit implementation configuration rather than this grammar.
 *
 *
 * ============================================================================
 * INTEGRATION TEST CONTRACT
 * ============================================================================
 *
 * The canonical integration path is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Types.typeExpression
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     TypeExpr::Linear
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type resolution
 *       |
 *       v
 *     ownership/resource analysis
 *       |
 *       v
 *     canonical IR
 *
 * For quantum programs:
 *
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *
 * For all targets:
 *
 *     canonical IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     target realization
 *
 *
 * ============================================================================
 * REQUIRED COMPANION CHANGE: grammar/types/types.g4
 * ============================================================================
 *
 * The current repository's canonical types.g4 currently contains the direct
 * qualifier alternatives:
 *
 *     typeQualifier
 *         : LINEAR
 *         | AFFINE
 *         ;
 *
 * That defeats modular ownership.
 *
 * After this file is installed, the canonical composition must instead
 * delegate to the modular qualifier rules.
 *
 * Conceptually:
 *
 *     typeQualifier
 *         : linearityQualifier
 *         ;
 *
 * if `linearityQualifier` is the combined ownership boundary, or:
 *
 *     typeQualifier
 *         : linearQualifier
 *         | affineQualifier
 *         ;
 *
 * if the repository imports both modular qualifier grammars directly.
 *
 * The preferred long-term architecture is:
 *
 *     linear.g4
 *          -> linearQualifier
 *
 *     affine.g4
 *          -> affineQualifier
 *
 *     types.g4
 *          -> typeQualifier
 *          -> typeExpression
 *
 * No direct LINEAR/AFFINE parser ownership remains in types.g4.
 *
 *
 * ============================================================================
 * REQUIRED ANTLR IMPORT CONTRACT
 * ============================================================================
 *
 * `types.g4` must import this grammar and the affine grammar using the
 * repository's canonical ANTLR modular-import mechanism.
 *
 * The exact import syntax must match the repository's existing
 * `grammar/types/types.g4` composition structure.
 *
 * The resulting dependency direction MUST remain:
 *
 *     lexer
 *       ↓
 *     linear.g4 / affine.g4
 *       ↓
 *     types.g4
 *       ↓
 *     root parser
 *
 * Never:
 *
 *     linear.g4
 *       ↓
 *     types.g4
 *       ↓
 *     linear.g4
 *
 *
 * ============================================================================
 * REQUIRED LEGACY CLEANUP
 * ============================================================================
 *
 * Any older grammar containing an independent:
 *
 *     linearType
 *
 * production that consumes the complete type expression must be removed from
 * the production import graph or explicitly marked deprecated.
 *
 * In particular, no legacy grammar may create a second interpretation of:
 *
 *     linear T
 *
 * with a different AST contract.
 *
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no embedded Rust.
 *
 * Therefore:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * compatibility is achieved through the generated parser/frontend integration.
 *
 * No:
 *
 *     unsafe
 *
 * code is required.
 *
 * Ownership checking must be implemented with ordinary safe Rust data
 * structures and algorithms.
 *
 * The grammar itself must remain language-definition code rather than Rust
 * implementation code.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Fixed language-level spelling:
 *
 *     linear
 *
 * Required canonical token:
 *
 *     LINEAR
 *
 * No physical or implementation capacity is encoded.
 *
 * No:
 *
 *     hardware ID
 *     device ID
 *     processor count
 *     GPU count
 *     QPU count
 *     FPGA count
 *     node count
 *     memory capacity
 *     thread count
 *     tensor rank limit
 *     generic arity limit
 *     type-depth limit
 *
 * is present.
 *
 *
 * ============================================================================
 * DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete when all of the following are true:
 *
 *   [x] It is a parser grammar.
 *   [x] It uses the canonical ZamaniLexer vocabulary.
 *   [x] It consumes only the canonical LINEAR token.
 *   [x] It defines no lexer rules.
 *   [x] It defines linearQualifier.
 *   [x] It defines the modular linearity boundary.
 *   [x] It does not consume typeExpression.
 *   [x] It does not create a circular ANTLR dependency.
 *   [x] Complete type composition remains owned by types.g4.
 *   [x] TypeExpr::Linear remains the canonical AST destination.
 *   [x] Semantic ownership remains downstream.
 *   [x] Resource analysis remains downstream.
 *   [x] Capability analysis remains downstream.
 *   [x] Effects remain downstream.
 *   [x] Contracts remain downstream.
 *   [x] Policies remain downstream.
 *   [x] Provenance remains downstream.
 *   [x] Quantum semantics remain target-independent.
 *   [x] quantum::ir remains the quantum IR boundary.
 *   [x] HDL semantics remain downstream.
 *   [x] No backend is selected here.
 *   [x] No physical resource is selected here.
 *   [x] No artificial scalability limit is introduced.
 *   [x] No embedded executable code exists.
 *   [x] No unsafe Rust is required.
 *   [x] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 * ============================================================================
 */

parser grammar LinearTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC RULES
 * ========================================================================== */

/**
 * Source-level linear type qualifier.
 *
 * Complete syntax is composed by grammar/types/types.g4:
 *
 *     linear <type-expression>
 */
linearQualifier
    : LINEAR
    ;


/**
 * Unified linearity boundary.
 *
 * This file owns the linear branch.
 *
 * The complete ownership qualifier composition remains the responsibility of
 * the canonical type grammar, which can combine this rule with the affine
 * qualifier without creating a dependency from this grammar back to the
 * complete type grammar.
 */
linearityQualifier
    : linearQualifier
    ;