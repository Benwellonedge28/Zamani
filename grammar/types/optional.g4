/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/optional.g4
 *
 * Grammar:
 *     OptionalType
 *
 * Status:
 *     Production-ready modular parser grammar.
 *
 * Purpose:
 *     Authoritative parser-level ownership of postfix optional-type syntax.
 *
 * Canonical surface syntax:
 *
 *     T?
 *
 * Examples:
 *
 *     int?
 *     bool?
 *     str?
 *     User?
 *     quantum::Qubit?
 *     Vec<int>?
 *     Option<T>?
 *     [T]?
 *     (T, U)?
 *     fn(T) -> U?
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     grammar/lexer/tokens.g4
 *       |
 *       |  QUESTION
 *       v
 *     modular parser grammars
 *       |
 *       v
 *     grammar/types/types.g4
 *       |
 *       +--> typePostfix
 *               |
 *               +--> optionalMarker
 *                       |
 *                       v
 *                  TypeExpr::Optional
 *                       |
 *                       v
 *                  semantic type model
 *                       |
 *                       +------------------------------+
 *                       |                              |
 *                       v                              v
 *                  classical semantics          quantum/resource semantics
 *                                                      |
 *                                                      v
 *                                                canonical IR
 *
 * This grammar participates only in the parser layer.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the optional-type postfix marker;
 *   - the parser rule representing `?`;
 *   - the lexical-to-parser integration contract for QUESTION;
 *   - the source-level optionality syntax boundary;
 *   - compatibility aliases needed by the type grammar, if explicitly
 *     retained by the repository's composition layer.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - typeExpression;
 *   - primitive types;
 *   - named types;
 *   - generic types;
 *   - arrays;
 *   - slices;
 *   - tuples;
 *   - function types;
 *   - references;
 *   - pointers;
 *   - result types;
 *   - never types;
 *   - unit types;
 *   - quantum types;
 *   - hardware types;
 *   - resource types;
 *   - capability types;
 *   - dependent types;
 *   - generic Option<T> construction;
 *   - Some / None / nil / null values;
 *   - pattern matching;
 *   - nullability analysis;
 *   - type inference;
 *   - type checking;
 *   - ownership;
 *   - borrowing;
 *   - resource allocation;
 *   - hardware discovery;
 *   - physical qubit allocation;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - HAL;
 *   - compiler backend selection;
 *   - runtime representation;
 *   - ABI layout.
 *
 * ============================================================================
 * SINGLE SOURCE OF TRUTH
 * ============================================================================
 *
 * The canonical lexical spelling of the optional marker is:
 *
 *     ?
 *
 * The canonical lexer token is:
 *
 *     QUESTION
 *
 * The lexer is owned by:
 *
 *     grammar/lexer/tokens.g4
 *
 * This file MUST NOT define a lexer rule.
 *
 * In particular, this file MUST NOT define:
 *
 *     QUESTION
 *     QUESTION_MARK
 *     '?'
 *
 * as lexer rules.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * This parser grammar consumes:
 *
 *     ZamaniTokens.QUESTION
 *
 * through:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * The lexer therefore remains the only owner of the textual spelling:
 *
 *     ?
 *
 * This avoids competing lexical vocabularies.
 *
 * ============================================================================
 * TYPE-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * `grammar/types/types.g4` remains the sole owner of:
 *
 *     typeExpression
 *
 * and the composition of type constructors.
 *
 * It should consume this grammar through:
 *
 *     typePostfix
 *         : optionalMarker
 *         ;
 *
 * The conceptual composition is:
 *
 *     typeExpression
 *         : typeQualifier* typePrimary typePostfix*
 *         ;
 *
 *     typePostfix
 *         : optionalMarker
 *         ;
 *
 *     optionalMarker
 *         : QUESTION
 *         ;
 *
 * This file therefore MUST NOT define:
 *
 *     typeExpression
 *
 * and MUST NOT recursively consume `typeExpression`.
 *
 * ============================================================================
 * WHY THE COMPLETE OPTIONAL TYPE IS NOT DEFINED HERE
 * ============================================================================
 *
 * It is tempting to write:
 *
 *     optionalType
 *         : typeExpression QUESTION
 *         ;
 *
 * That is architecturally incorrect because `typeExpression` owns the
 * complete type-expression composition and may itself contain postfixes.
 *
 * Such a rule creates recursive ownership:
 *
 *     Types -> OptionalType -> Types
 *
 * and makes the optional grammar a second owner of the complete type syntax.
 *
 * Instead, this file owns exactly one reusable syntactic component:
 *
 *     optionalMarker
 *
 * The enclosing `Types` grammar decides which complete type the marker
 * modifies.
 *
 * ============================================================================
 * CANONICAL SOURCE SEMANTICS
 * ============================================================================
 *
 * Source:
 *
 *     T?
 *
 * structurally represents:
 *
 *     TypeExpr::Optional(Box<T>)
 *
 * The parser recognizes syntax.
 *
 * The AST builder constructs the canonical frontend representation.
 *
 * Semantic analysis determines:
 *
 *     - whether T is a valid optional inner type;
 *     - whether nested optionality is permitted;
 *     - absence semantics;
 *     - nullability semantics;
 *     - flow-sensitive narrowing;
 *     - pattern matching behavior;
 *     - ownership interaction;
 *     - borrowing interaction;
 *     - generic interaction;
 *     - resource interaction;
 *     - quantum-domain legality;
 *     - conversions and coercions.
 *
 * None of those decisions belong here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO AST type.
 *
 * The parser/AST builder must map:
 *
 *     T?
 *
 * to the repository's canonical:
 *
 *     TypeExpr::Optional(Box<TypeExpr>)
 *
 * The authoritative frontend AST is:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * No:
 *
 *     OptionalTypeAst
 *     OptionalTypeNode
 *     OptionalTypeIr
 *     OptionTypeExpression
 *
 * may be introduced merely to represent this grammar rule.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The grammar establishes only:
 *
 *     optionality is expressed as a postfix type constructor.
 *
 * It does NOT establish:
 *
 *     optionality == nullability
 *     optionality == pointer
 *     optionality == nullable reference
 *     optionality == resource absence
 *     optionality == allocation failure
 *     optionality == device failure
 *     optionality == quantum measurement
 *     optionality == qubit release
 *
 * These meanings are determined by semantic analysis.
 *
 * ============================================================================
 * GENERIC OPTION INTEGRATION
 * ============================================================================
 *
 * This grammar does NOT own:
 *
 *     Option<T>
 *
 * because that is generic type application.
 *
 * These are syntactically distinct:
 *
 *     T?
 *     Option<T>
 *
 * They may or may not have equivalent semantic representations according to
 * the language specification, but that decision is downstream.
 *
 * Valid compositions can include:
 *
 *     Option<T>?
 *     Vec<Option<T>>
 *     Option<Vec<T>?>
 *     Result<Option<T>, E>
 *
 * subject to the canonical generic/type grammar and semantic validation.
 *
 * ============================================================================
 * VALUE-LEVEL INTEGRATION
 * ============================================================================
 *
 * Optional type syntax must remain separate from optional values.
 *
 * This grammar does NOT parse:
 *
 *     Some(value)
 *     None
 *     nil
 *     null
 *
 * Existing lexer vocabulary may contain tokens such as:
 *
 *     K_SOME
 *     K_NIL
 *     K_NULL
 *
 * Those belong to value/expression syntax.
 *
 * This separation is required so:
 *
 *     type syntax
 *
 * and:
 *
 *     value syntax
 *
 * cannot accidentally acquire competing ownership.
 *
 * ============================================================================
 * NESTED OPTIONALITY
 * ============================================================================
 *
 * The enclosing type grammar permits repeated postfix constructors when its
 * `typePostfix*` composition is used.
 *
 * Therefore the grammar does not establish a finite optional nesting limit.
 *
 * Examples structurally representable by the grammar include:
 *
 *     T?
 *     T??
 *     T???
 *
 * Whether:
 *
 *     T??
 *
 * is semantically normalized, rejected, preserved as nested optionality, or
 * otherwise interpreted is a semantic/type-system decision.
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_OPTIONAL_DEPTH
 *     MAX_OPTIONAL_NESTING
 *     MAX_TYPE_DEPTH
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Optionality is a source-level semantic constructor.
 *
 * It does not impose machine limits.
 *
 * This grammar contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_ARRAY_LENGTH
 *     MAX_GENERIC_ARITY
 *     MAX_TYPE_DEPTH
 *     MAX_OPTIONAL_DEPTH
 *
 * The number of optional types, nesting depth, program size, resource count,
 * quantum count, hardware count, or distributed-node count is not constrained
 * by this grammar.
 *
 * A compiler may have explicit configurable operational safety policies, but
 * those policies are implementation constraints and MUST NOT become Zamani
 * language semantics.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Optionality is domain-neutral.
 *
 * If the canonical type grammar admits a quantum type, then the optional
 * constructor can syntactically wrap it:
 *
 *     Qubit?
 *     LogicalQubit?
 *     QuantumState<T>?
 *     QuantumRegister<N>?
 *
 * This grammar does NOT mean:
 *
 *     optional physical qubit;
 *     physical qubit allocation;
 *     physical qubit release;
 *     QPU availability;
 *     hardware failure;
 *     measurement;
 *     reset;
 *     routing;
 *     scheduling;
 *     calibration.
 *
 * Those belong to later semantic/resource/quantum layers.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar has no direct dependency on that IR.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Optionality may wrap classical types:
 *
 *     int?
 *     float?
 *     bool?
 *     str?
 *     User?
 *     Vec<int>?
 *
 * Runtime representation is deliberately unspecified.
 *
 * The grammar does not choose:
 *
 *     tagged representation;
 *     sentinel representation;
 *     pointer representation;
 *     register representation;
 *     memory representation;
 *     ABI representation.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Optionality may wrap source-level HDL or hardware abstractions when those
 * types are admitted by the canonical type grammar:
 *
 *     HardwareSignal<T>?
 *     Resource<Accelerator>?
 *
 * The grammar does not imply:
 *
 *     optional physical wire;
 *     optional FPGA register;
 *     optional pin;
 *     optional device;
 *     optional clock;
 *     optional memory bank.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * A type such as:
 *
 *     Resource<T>?
 *
 * describes a source-level optional resource value.
 *
 * It does NOT:
 *
 *     allocate T;
 *     reserve T;
 *     release T;
 *     discover T;
 *     select a device;
 *     select a node;
 *     select a physical qubit.
 *
 * Requirement, capability, preference, placement and implementation decisions
 * remain distinct downstream concepts.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Optionality may wrap distributed abstractions:
 *
 *     Service<T>?
 *     Remote<T>?
 *     Channel<T>?
 *
 * The grammar does not define absence as:
 *
 *     node failure;
 *     network failure;
 *     service failure;
 *     process failure;
 *     resource retirement.
 *
 * Runtime and resilience semantics determine those meanings.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT directly reference:
 *
 *     quantum::ir
 *     ZUIR
 *     classical IR
 *     HDL IR
 *     LLVM
 *     MLIR
 *     QIR
 *     vendor IRs
 *
 * The intended lowering chain is:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     TypeExpr::Optional
 *       |
 *       v
 *     semantic type
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 *   classical semantics    quantum semantics
 *                               |
 *                               v
 *                           quantum::ir
 *
 * Optionality therefore cannot accidentally create a second quantum IR.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may lower optionality differently depending on target
 * capabilities.
 *
 * Examples include:
 *
 *     tagged value;
 *     discriminant + payload;
 *     nullable representation;
 *     optimized representation;
 *     erased representation where statically proven unnecessary.
 *
 * Those are implementation decisions.
 *
 * They MUST NOT alter the source grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime representation is not specified here.
 *
 * The same source:
 *
 *     T?
 *
 * must remain portable across:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     embedded target
 *     distributed runtime
 *     future target
 *
 * provided the target satisfies the program's semantic requirements.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This grammar should permit the parser to report the syntactic location of
 * the QUESTION token.
 *
 * Examples of syntax errors that belong to enclosing grammar/diagnostic
 * layers include:
 *
 *     unexpected QUESTION
 *     malformed type before QUESTION
 *     incomplete type expression
 *
 * Semantic diagnostics such as:
 *
 *     optionality is not permitted for this type
 *     nested optionality is prohibited
 *     optional resource cannot be borrowed here
 *
 * do NOT belong to this grammar.
 *
 * Source spans are inherited from the QUESTION token and the enclosing type
 * expression by the frontend AST builder.
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * This grammar intentionally has no embedded parser actions.
 *
 * Error recovery is delegated to the canonical parser/frontend diagnostic
 * infrastructure.
 *
 * The rule must remain deterministic and side-effect free.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar has one lexical alternative for the optional marker:
 *
 *     QUESTION
 *
 * There is no target-dependent behavior, semantic lookup, I/O, or runtime
 * state.
 *
 * Therefore parsing this component is deterministic for a given token stream.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - executes no user code;
 *     - performs no allocation decisions;
 *     - contains no Rust actions;
 *     - contains no embedded predicates;
 *     - contains no native calls;
 *     - contains no `unsafe`;
 *     - contains no target-specific behavior.
 *
 * Generated Rust integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021 edition
 *
 * and must not require unsafe Rust.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Canonical spelling:
 *
 *     T?
 *
 * The marker is postfix.
 *
 * The grammar does NOT introduce:
 *
 *     ?T
 *
 * as an alternative spelling.
 *
 * It also does not make:
 *
 *     Optional<T>
 *
 * a synonym at grammar level. `Option<T>`/other generic forms remain owned by
 * generic type syntax.
 *
 * Existing source compatibility must be determined by the language-version
 * compatibility specification, not by adding competing grammar alternatives
 * here.
 *
 * ============================================================================
 * EXISTING option-types.g4 INTEGRATION
 * ============================================================================
 *
 * The repository already contains:
 *
 *     grammar/types/option-types.g4
 *
 * That file documents the same architectural concept and identifies
 * `optionalMarker` as its intended parser boundary.
 *
 * To prevent two authorities, the repository's final composition must select
 * ONE owner for `optionalMarker`.
 *
 * This file is the canonical owner for the path:
 *
 *     grammar/types/optional.g4
 *
 * The existing `option-types.g4` must therefore be treated as a compatibility
 * / migration surface rather than an independent competing implementation.
 *
 * It MUST NOT introduce another independently recognized `optionalMarker`.
 *
 * No parser grammar should import both implementations as independent owners.
 *
 * ============================================================================
 * TYPES.G4 INTEGRATION
 * ============================================================================
 *
 * `grammar/types/types.g4` currently owns:
 *
 *     typeExpression
 *     typePrimary
 *     typePostfix
 *
 * Its canonical composition should be:
 *
 *     typePostfix
 *         : optionalMarker
 *         ;
 *
 * where `optionalMarker` is supplied by this grammar.
 *
 * The existing `types.g4` source currently contains the token spelling:
 *
 *     QUESTION_MARK
 *
 * That must NOT be copied here.
 *
 * The canonical lexer vocabulary identified by the existing optional-type
 * grammar is:
 *
 *     QUESTION
 *
 * Therefore the final integration must use:
 *
 *     QUESTION
 *
 * from:
 *
 *     grammar/lexer/tokens.g4
 *
 * rather than creating a second lexical token.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file's parser-level conformance cases are:
 *
 * POSITIVE:
 *
 *     int?
 *     bool?
 *     User?
 *     quantum::Qubit?
 *     Vec<int>?
 *     Option<T>?
 *     [int]?
 *     (int, bool)?
 *     fn(int) -> bool?
 *
 * NESTED:
 *
 *     T??
 *     T???
 *     Vec<T?>?
 *     Option<T?>?
 *
 * DOMAIN-NEUTRAL:
 *
 *     Resource<T>?
 *     Capability<C>?
 *     HardwareSignal<T>?
 *     Service<T>?
 *     QuantumState<T>?
 *
 * NEGATIVE:
 *
 *     ?
 *     ???
 *     ?T
 *
 * Note that a negative case such as:
 *
 *     ?
 *
 * is invalid as a complete type expression, while this grammar alone merely
 * recognizes the QUESTION token. The enclosing `Types.typeExpression`
 * composition is responsible for complete-type validation.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes no semantic upper bound on:
 *
 *     optional nesting;
 *     generic nesting;
 *     type-path depth;
 *     type constructor count;
 *     quantum cardinality;
 *     resource cardinality;
 *     distributed-node count;
 *     hardware scale.
 *
 * Operational parser limits, where required for denial-of-service protection,
 * must be configurable outside the grammar and must not alter language
 * semantics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_*
 *     fixed machine counts
 *     fixed qubit counts
 *     fixed device counts
 *     fixed node counts
 *     fixed memory sizes
 *     fixed register widths
 *     fixed accelerator counts
 *     physical identifiers
 *     vendor identifiers
 *     topology assumptions
 *     backend assumptions
 *     ABI assumptions
 *
 * Audit result:
 *
 *     PASS
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] parser grammar only;
 *   [x] canonical token vocabulary selected;
 *   [x] no lexer duplication;
 *   [x] no complete type-expression duplication;
 *   [x] optional marker has one owner;
 *   [x] TypeExpr::Optional is the AST boundary;
 *   [x] semantic interpretation remains downstream;
 *   [x] no IR dependency;
 *   [x] quantum::ir remains canonical;
 *   [x] no hardware assumptions;
 *   [x] no resource limits;
 *   [x] no unsafe/actions/predicates;
 *   [x] Rust 1.97/1.97.1 compatibility contract documented;
 *   [x] deterministic;
 *   [x] source-span compatible;
 *   [x] nested optionality remains semantically controlled downstream;
 *   [x] generic Option<T> remains separately owned;
 *   [x] test contract defined;
 *   [x] compatibility contract defined.
 *
 * ============================================================================
 */

parser grammar OptionalType;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * The canonical reusable parser boundary for optionality.
 *
 * IMPORTANT:
 *
 * This is deliberately NOT:
 *
 *     optionalType : typeExpression QUESTION ;
 *
 * because `typeExpression` is owned by grammar/types/types.g4.
 *
 * The enclosing type grammar applies this postfix to the preceding complete
 * type expression.
 */
optionalMarker
    : QUESTION
    ;


/*
 * ============================================================================
 * EXPLICIT POSTFIX ALIAS
 * ============================================================================
 *
 * This rule exists as a descriptive integration point for tools that need to
 * refer to the optional postfix constructor without taking ownership of the
 * complete type expression.
 *
 * It remains a one-token delegate.
 */
optionalTypePostfix
    : optionalMarker
    ;