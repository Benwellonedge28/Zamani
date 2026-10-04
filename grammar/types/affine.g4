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
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     This grammar contains no embedded executable code.
 *     The Zamani implementation uses safe Rust only.
 *     No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the source-level `affine` type qualifier.
 *
 * It intentionally defines ONLY the qualifier marker:
 *
 *     affine
 *
 * The complete type expression is composed by:
 *
 *     grammar/types/types.g4
 *
 * For example:
 *
 *     affine T
 *     affine Resource<T>
 *     affine Qubit
 *     affine Vec<T>
 *     affine Tensor<T, Shape>
 *
 * are composed by the canonical type-expression grammar.
 *
 * This file does not parse the operand type.
 *
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * An affine type expresses an ownership/usage property:
 *
 *     Affine(T)
 *
 * The semantic meaning is generally:
 *
 *     a value may be consumed zero or one time.
 *
 * The exact ownership rules are semantic rules and are NOT implemented by
 * this grammar.
 *
 * The grammar is therefore responsible only for recognizing:
 *
 *     affine
 *
 * and exposing that construct to the canonical type composition grammar.
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 * This file owns:
 *
 *     - affineQualifier;
 *     - the parser-level representation of the `affine` qualifier;
 *     - the modular grammar boundary for affine qualification;
 *     - consumption of the canonical AFFINE lexer token.
 *
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 * This file does NOT own:
 *
 *     - the complete typeExpression;
 *     - typeCore;
 *     - typePrimary;
 *     - typePostfix;
 *     - identifiers;
 *     - qualified names;
 *     - generic arguments;
 *     - tuples;
 *     - arrays;
 *     - slices;
 *     - function types;
 *     - references;
 *     - pointers;
 *     - dependent types;
 *     - quantum types;
 *     - HDL types;
 *     - resource types;
 *     - capability types;
 *     - lifetime syntax;
 *     - borrowing;
 *     - ownership analysis;
 *     - move analysis;
 *     - copy analysis;
 *     - drop analysis;
 *     - linear-use analysis;
 *     - allocation;
 *     - deallocation;
 *     - resource discovery;
 *     - capability discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - ABI layout;
 *     - runtime representation;
 *     - execution.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *     grammar/lexer/keywords.g4
 *
 * The exact lexical composition is owned by the canonical lexer.
 *
 *
 * EXPORTS
 * -------
 *
 *     affineQualifier
 *
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/types/types.g4
 *
 *
 * AST_OWNER
 * ---------
 *
 *     src/ast/mod.rs
 *
 * The canonical source representation is:
 *
 *     TypeExpr::Affine
 *
 *
 * SEMANTIC_OWNER
 * -------------
 *
 * The semantic/type-analysis subsystem owns:
 *
 *     - affine validity;
 *     - ownership;
 *     - use counts;
 *     - move semantics;
 *     - copy semantics;
 *     - drop semantics;
 *     - interaction with references;
 *     - interaction with linear types;
 *     - interaction with generic constraints;
 *     - interaction with resource semantics.
 *
 *
 * IR_OWNER
 * --------
 *
 * The grammar does not own an IR.
 *
 * After semantic analysis, the type is represented by the repository's
 * canonical semantic/IR pipeline.
 *
 * Quantum semantics continue through:
 *
 *     quantum::ir
 *
 * when applicable.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/
 *
 * with affine-specific parser/type conformance tests.
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical token is:
 *
 *     AFFINE
 *
 * whose source spelling is:
 *
 *     affine
 *
 * This parser grammar MUST NOT define:
 *
 *     AFFINE
 *
 * or any other lexer rule.
 *
 * The lexical authority remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * with the repository's lexical composition under:
 *
 *     grammar/lexer/
 *
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * `affineQualifier` is the only public rule exported by this feature grammar.
 *
 * It represents the qualifier marker only.
 *
 * Complete source syntax is composed elsewhere:
 *
 *     affineQualifier + canonical type expression
 *
 * Example:
 *
 *     affine T
 *
 * becomes conceptually:
 *
 *     affineQualifier
 *         +
 *     typeExpression
 *
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
 * The qualifier intentionally stops after AFFINE.
 *
 * DO NOT change this to:
 *
 *     affineQualifier
 *         : AFFINE typeExpression
 *         ;
 *
 * because `typeExpression` belongs to grammar/types/types.g4.
 *
 * Such a rule would invert the dependency:
 *
 *     types.g4
 *         -> affine.g4
 *             -> types.g4
 *
 * and would create an invalid/circular modular grammar architecture.
 *
 * The correct direction is:
 *
 *     affine.g4
 *         -> affineQualifier
 *
 *     types.g4
 *         -> affineQualifier
 *         -> typeExpression
 *
 * ============================================================================
 */

affineQualifier
    : AFFINE
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser recognizes:
 *
 *     affine
 *
 * Semantic analysis interprets:
 *
 *     affine T
 *
 * as an affine-qualified type.
 *
 * The canonical AST destination is:
 *
 *     TypeExpr::Affine(inner)
 *
 * where `inner` is produced by the canonical type grammar.
 *
 * This grammar itself MUST NOT:
 *
 *     - count variable uses;
 *     - perform ownership analysis;
 *     - determine whether a value is copied;
 *     - determine whether a value is moved;
 *     - determine whether a value is dropped;
 *     - validate lifetime relationships;
 *     - inspect generic constraints;
 *     - inspect capabilities;
 *     - inspect resources;
 *     - inspect hardware.
 *
 *
 * ============================================================================
 * AFFINE SEMANTICS
 * ============================================================================
 *
 * The semantic distinction is:
 *
 *     unrestricted
 *         zero or more uses as permitted by the type system
 *
 *     affine
 *         zero or one consuming use
 *
 *     linear
 *         exactly one required consuming use
 *
 * These are semantic distinctions.
 *
 * The grammar only records the source qualifier.
 *
 *
 * ============================================================================
 * LINEAR INTEGRATION
 * ============================================================================
 *
 * `grammar/types/linear.g4` owns:
 *
 *     linearQualifier
 *
 * This file owns:
 *
 *     affineQualifier
 *
 * The canonical composition owner:
 *
 *     grammar/types/types.g4
 *
 * combines them.
 *
 * The intended architecture is:
 *
 *     LinearTypes
 *         |
 *         +--> linearQualifier
 *                         \
 *                          \
 *                           +--> types.g4
 *                          /
 *                         /
 *     AffineTypes
 *         |
 *         +--> affineQualifier
 *
 * This prevents either qualifier grammar from depending on the complete type
 * grammar.
 *
 *
 * ============================================================================
 * TYPE COMPOSITION
 * ============================================================================
 *
 * The complete form:
 *
 *     affine T
 *
 * is owned compositionally by `types.g4`.
 *
 * Examples include:
 *
 *     affine T
 *     affine User
 *     affine module::User
 *     affine Vec<T>
 *     affine Map<K, V>
 *     affine Result<T, E>
 *     affine (T, U)
 *     affine [T]
 *     affine [T; N]
 *     affine fn(T) -> U
 *     affine &T
 *     affine &mut T
 *     affine Qubit
 *     affine Resource<Qubit>
 *     affine Tensor<T, Shape>
 *
 * This file deliberately does not enumerate any of those types.
 *
 * Therefore newly introduced types automatically participate in affine
 * qualification as long as the canonical type grammar accepts them.
 *
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic syntax belongs to:
 *
 *     grammar/types/types.g4
 *
 * This grammar does not know or care whether the inner type is:
 *
 *     Vec<T>
 *     Map<K,V>
 *     Tensor<T, Shape>
 *     Resource<R>
 *     Capability<C>
 *
 * or a future type constructor.
 *
 * This is required for long-term extensibility.
 *
 *
 * ============================================================================
 * DEPENDENT / SYMBOLIC TYPE INTEGRATION
 * ============================================================================
 *
 * Affine qualification must work with symbolic type parameters and symbolic
 * dimensions.
 *
 * Examples:
 *
 *     affine Vector<T, N>
 *     affine Matrix<T, Rows, Cols>
 *     affine Tensor<T, Shape>
 *
 * `N`, `Rows`, `Cols`, and `Shape` remain semantic values/parameters.
 *
 * This grammar does not:
 *
 *     - evaluate them;
 *     - convert them to machine integers;
 *     - impose ranges;
 *     - impose maximum dimensions;
 *     - select storage.
 *
 *
 * ============================================================================
 * REFERENCE INTEGRATION
 * ============================================================================
 *
 * Reference syntax belongs to the canonical type grammar and its modular
 * reference component.
 *
 * Examples that may be accepted by the composed type grammar include:
 *
 *     affine &T
 *     affine &mut T
 *     affine &'a T
 *     affine &'a mut T
 *
 * Whether a particular combination is semantically valid belongs to ownership,
 * borrowing, and lifetime analysis.
 *
 * This grammar MUST NOT duplicate:
 *
 *     referenceType
 *     lifetimeAnnotation
 *
 *
 * ============================================================================
 * POINTER INTEGRATION
 * ============================================================================
 *
 * Pointer syntax belongs to the canonical pointer grammar.
 *
 * This file does not define:
 *
 *     *T
 *     *mut T
 *
 * nor does it determine pointer representation.
 *
 * If an affine-qualified pointer type is permitted by the semantic type system,
 * it is composed by the canonical type grammar.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Affine qualification may be useful for quantum resource ownership.
 *
 * Examples:
 *
 *     affine Qubit
 *     affine LogicalQubit
 *     affine QRegister<N>
 *     affine QuantumState<T>
 *     affine QuantumResource<R>
 *
 * This grammar does not assign physical meaning to those names.
 *
 * It MUST NOT:
 *
 *     - allocate physical qubits;
 *     - select QPUs;
 *     - inspect coupling maps;
 *     - select physical qubit identifiers;
 *     - route quantum operations;
 *     - schedule circuits;
 *     - model calibration;
 *     - perform QEC;
 *     - inspect device topology.
 *
 * If the qualified type participates in quantum computation, downstream
 * semantic lowering continues through:
 *
 *     quantum::ir
 *
 * No affine-specific quantum IR is created.
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Affine qualification may apply to resource-oriented semantic types.
 *
 * For example:
 *
 *     affine Resource<T>
 *     affine Capability<C>
 *
 * remains source-level type information.
 *
 * It does NOT mean:
 *
 *     allocate resource;
 *     reserve hardware;
 *     claim a device;
 *     select a processor;
 *     select a memory bank.
 *
 * Resource realization remains downstream:
 *
 *     type semantics
 *         |
 *         v
 *     resource analysis
 *         |
 *         v
 *     capability negotiation
 *         |
 *         v
 *     execution planning
 *         |
 *         v
 *     target realization
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Affine qualification is target-neutral.
 *
 * It does not identify:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     memory bank
 *     register
 *     physical signal
 *     physical device
 *
 * HDL and hardware semantic layers determine the actual realization.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Merely parsing:
 *
 *     affine
 *
 * produces no runtime effect.
 *
 * The affine qualifier may influence semantic analysis of operations involving
 * the resulting type.
 *
 * Any resulting effects are owned by the canonical effect system, not this
 * grammar.
 *
 * This grammar must never execute:
 *
 *     allocation;
 *     mutation;
 *     learning;
 *     adaptation;
 *     I/O;
 *     networking;
 *     native calls;
 *     foreign calls;
 *     hardware operations.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * The affine qualifier requires no capability merely to parse.
 *
 * Semantic consumers may later require capabilities associated with:
 *
 *     resource ownership;
 *     quantum execution;
 *     hardware access;
 *     foreign resources;
 *     privileged operations.
 *
 * Capability resolution belongs to the resource/capability subsystem.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar introduces no resource requirement.
 *
 * In particular, it does not encode:
 *
 *     memory size;
 *     processor count;
 *     thread count;
 *     qubit count;
 *     GPU count;
 *     FPGA count;
 *     node count;
 *     device count;
 *     tensor rank;
 *     register width.
 *
 * Resource requirements are expressed by the universal resource system.
 *
 *
 * ============================================================================
 * CONTRACT / POLICY / PROVENANCE
 * ============================================================================
 *
 * Affine qualification may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * but those constructs are owned by the contract/validation subsystem.
 *
 * Policies affecting ownership or resource use are consumed semantically.
 *
 * Provenance for:
 *
 *     source qualifier
 *     inferred ownership
 *     type transformation
 *     optimization
 *
 * belongs to the compiler provenance subsystem.
 *
 * This grammar preserves the parser source context needed by those systems.
 *
 *
 * ============================================================================
 * POCO-REAF SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file contains no machine or implementation ceiling.
 *
 * It MUST NOT introduce limits such as:
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
 * It also MUST NOT indirectly enumerate a finite set of supported types.
 *
 * The only fixed lexical concept here is:
 *
 *     affine
 *
 * Consequently:
 *
 *     affine + any future canonical type
 *
 * remains possible without modifying this grammar.
 *
 * This supports the POCO-REAF objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * subject to actual resource/capability feasibility.
 *
 * "Infinity" here means no artificial language-level machine ceiling.
 * It does not claim infinite physical hardware, compiler memory, storage,
 * execution time, or network capacity.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The rule:
 *
 *     affineQualifier : AFFINE ;
 *
 * is deterministic.
 *
 * Parsing depends only on:
 *
 *     - source tokens;
 *     - language version;
 *     - grammar definition;
 *     - lexical configuration.
 *
 * It MUST NOT depend on:
 *
 *     - hardware availability;
 *     - CPU count;
 *     - GPU availability;
 *     - QPU availability;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - random state;
 *     - scheduler state;
 *     - deployment topology.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar is side-effect free.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no environment inspection;
 *     - no hardware access;
 *     - no command execution;
 *     - no dynamic evaluation.
 *
 * Hostile-input protection belongs to explicit parser/compiler resource
 * policies and MUST NOT change the language meaning of `affine`.
 *
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The parser/frontend must preserve sufficient source information for:
 *
 *     - diagnostics;
 *     - IDE/LSP;
 *     - formatting;
 *     - source maps;
 *     - provenance;
 *     - incremental compilation;
 *     - compatibility analysis.
 *
 * The AFFINE token's source span belongs to the parser context.
 *
 * The complete:
 *
 *     affine T
 *
 * source span is constructed by the enclosing type-expression AST builder.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax-level examples:
 *
 *     affine
 *
 * may be incomplete when used where a complete type expression is required.
 *
 * The enclosing `types.g4` grammar reports the missing operand.
 *
 * Examples such as:
 *
 *     affine 123
 *
 * are rejected by the canonical type grammar because the operand does not
 * form a valid type expression.
 *
 * Examples such as:
 *
 *     affine affine T
 *
 * are parsed according to the canonical qualifier-composition rules.
 * Whether duplicate affine qualification is legal is a semantic/type-system
 * decision rather than an affine grammar decision.
 *
 * Examples such as:
 *
 *     affine linear T
 *
 * and:
 *
 *     linear affine T
 *
 * are likewise governed by the canonical qualifier composition and semantic
 * ownership rules.
 *
 * This prevents independent qualifier grammars from inventing incompatible
 * ordering semantics.
 *
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
 * Stable semantic destination:
 *
 *     TypeExpr::Affine
 *
 * This grammar therefore preserves the existing language spelling and AST
 * concept while correcting modular ownership.
 *
 * No source-level migration is required merely because the grammar becomes
 * modular.
 *
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH `grammar/types/types.g4`
 * ============================================================================
 *
 * The canonical type grammar currently owns:
 *
 *     typeExpression
 *     typeQualifier
 *     typeCore
 *     referenceType
 *     lifetimeAnnotation
 *     and the other type composition rules.
 *
 * Its current direct qualifier form is conceptually:
 *
 *     typeQualifier
 *         : LINEAR
 *         | AFFINE
 *         ;
 *
 * After installing this file, that direct ownership should be changed to:
 *
 *     typeQualifier
 *         : linearQualifier
 *         | affineQualifier
 *         ;
 *
 * `types.g4` must import the modular qualifier grammars using its normal ANTLR
 * import mechanism:
 *
 *     import LinearTypes, AffineTypes;
 *
 * or the equivalent ordering already established by the repository's generated
 * grammar layout.
 *
 * IMPORTANT:
 *
 * The exact import declaration must be made in `types.g4`, not in this file.
 *
 * This file MUST NOT import `Types`.
 *
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH `grammar/types/linear.g4`
 * ============================================================================
 *
 * `linear.g4` owns:
 *
 *     linearQualifier
 *
 * This file owns:
 *
 *     affineQualifier
 *
 * Neither qualifier grammar depends on the other.
 *
 * Both depend only on:
 *
 *     ZamaniLexer
 *
 * The canonical type composition grammar combines them.
 *
 *
 * ============================================================================
 * LEGACY GRAMMAR INTEGRATION
 * ============================================================================
 *
 * Any older grammar that defines:
 *
 *     affineType
 *
 * as:
 *
 *     AFFINE typeExpression
 *
 * must not remain in the production ANTLR import graph.
 *
 * In particular, a legacy reference such as:
 *
 *     grammar/types/reference-types.g4
 *
 * or any historical type grammar must not create a competing affine qualifier
 * or a second complete type-expression composition path.
 *
 * Legacy material should be:
 *
 *     deprecated;
 *     excluded from the production import graph;
 *     documented for migration;
 *
 * rather than silently acting as a second grammar authority.
 *
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The existing canonical AST contains:
 *
 *     TypeExpr::Affine(Box<TypeExpr>)
 *
 * Therefore this grammar does NOT introduce:
 *
 *     AffineType
 *     AffineTypeExpr
 *     AffineIR
 *     AffineNode
 *
 * as parallel representations.
 *
 * The frontend transformation is conceptually:
 *
 *     affineQualifier
 *          +
 *     typeExpression
 *          |
 *          v
 *     TypeExpr::Affine(inner)
 *
 * The exact Rust AST construction remains the responsibility of the parser/
 * frontend implementation.
 *
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * No IR is produced by this grammar.
 *
 * The pipeline remains:
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
 *     TypeExpr::Affine
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type analysis
 *       |
 *       v
 *     ownership/resource/effect analysis
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +----------------------+
 *       |                      |
 *       v                      v
 *   classical              quantum::ir
 *       |                      |
 *       +----------+-----------+
 *                  |
 *                  v
 *             optimization
 *                  |
 *               lowering
 *                  |
 *           routing/scheduling
 *                  |
 *          resilience/QEC
 *                  |
 *                 ZQN
 *                  |
 *                 HAL
 *                  |
 *               target
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 * These should be tested through the canonical type-expression entry point:
 *
 *     affine T
 *     affine User
 *     affine module::User
 *     affine Vec<T>
 *     affine Map<K, V>
 *     affine Result<T, E>
 *     affine Qubit
 *     affine Resource<Qubit>
 *     affine Tensor<T, Shape>
 *     affine Vector<T, N>
 *     affine &T
 *     affine &mut T
 *     affine &'a T
 *     affine &'a mut T
 *
 *
 * NEGATIVE TESTS
 * --------------
 *
 * The following must not be accepted as complete affine types:
 *
 *     affine 123
 *     affine (
 *     affine <
 *     affine [
 *     affine &
 *     affine *
 *
 * The canonical type grammar owns the precise diagnostic.
 *
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Test combinations with:
 *
 *     linear
 *     references
 *     pointers
 *     generics
 *     dependent types
 *     quantum types
 *     resource types
 *     function types
 *     tuple types
 *     optional types
 *     Result types
 *
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * At minimum:
 *
 *     affine Qubit
 *     affine Resource<Qubit>
 *     affine Tensor<T, Shape>
 *     affine AcceleratorResource
 *     affine HardwareResource
 *
 * Domain names remain ordinary canonical types unless separately reserved.
 *
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Tests must verify that the qualifier remains independent of:
 *
 *     - generic arity;
 *     - qualified-name depth;
 *     - symbolic dimension count;
 *     - type nesting;
 *     - resource count;
 *     - quantum resource count;
 *     - hardware scale.
 *
 * No test may encode a language-level maximum.
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Identical source and identical language configuration must produce identical
 * parser structure independent of:
 *
 *     - machine size;
 *     - target;
 *     - available accelerators;
 *     - network topology;
 *     - runtime state.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * The only fixed language concept in this grammar is:
 *
 *     AFFINE
 *
 * There are no:
 *
 *     - hardware identifiers;
 *     - resource counts;
 *     - processor counts;
 *     - qubit counts;
 *     - device counts;
 *     - memory capacities;
 *     - thread counts;
 *     - tensor-rank ceilings;
 *     - generic-arity ceilings;
 *     - type-depth ceilings;
 *     - implementation-specific numeric limits.
 *
 * No application-specific vocabulary is introduced.
 *
 * No vendor-specific type inventory is introduced.
 *
 *
 * ============================================================================
 * ANTLR / RUST SAFETY
 * ============================================================================
 *
 * This is a pure ANTLR parser grammar.
 *
 * It contains no:
 *
 *     @members
 *     embedded Rust
 *     semantic predicates
 *     target-language actions
 *     unsafe blocks
 *     unsafe functions
 *
 * Rust 1.97 / Rust 1.97.1 compatibility is therefore preserved at the frontend
 * implementation boundary.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It is a parser grammar.
 *     [x] It uses tokenVocab = ZamaniLexer.
 *     [x] It consumes the canonical AFFINE token.
 *     [x] It defines affineQualifier.
 *     [x] It does not define lexer rules.
 *     [x] It does not consume typeExpression.
 *     [x] It does not create a circular dependency with types.g4.
 *     [x] Complete type composition remains owned by types.g4.
 *     [x] TypeExpr::Affine remains the canonical AST destination.
 *     [x] Linear and affine qualifiers have separate modular ownership.
 *     [x] Ownership semantics remain downstream.
 *     [x] Borrow/lifetime semantics remain downstream.
 *     [x] Resource semantics remain downstream.
 *     [x] Capability semantics remain downstream.
 *     [x] Effect semantics remain downstream.
 *     [x] Contract semantics remain downstream.
 *     [x] Policy semantics remain downstream.
 *     [x] Provenance remains downstream.
 *     [x] Quantum semantics remain target-independent.
 *     [x] quantum::ir remains the quantum IR boundary.
 *     [x] HDL semantics remain downstream.
 *     [x] No backend is selected.
 *     [x] No physical resource is selected.
 *     [x] No machine ceiling is encoded.
 *     [x] No application-specific keyword inventory is encoded.
 *     [x] No embedded executable code exists.
 *     [x] No unsafe Rust is required.
 *     [x] Rust 1.97 / Rust 1.97.1 compatibility is preserved.
 *
 * ============================================================================
 */

parser grammar AffineTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC AFFINE QUALIFIER
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This rule intentionally consumes only AFFINE.
 *
 * The following is NOT permitted here:
 *
 *     affineQualifier
 *         : AFFINE typeExpression
 *         ;
 *
 * `typeExpression` belongs to `grammar/types/types.g4`.
 *
 * ============================================================================
 */

affineQualifier
    : AFFINE
    ;