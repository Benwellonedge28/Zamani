/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/option-types.g4
 *
 * Grammar:
 *     OptionTypes
 *
 * Status:
 *     Production-ready optional-type syntax delegate.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-LEVEL OPTIONAL-TYPE MARKER used by Zamani.
 *
 * Canonical surface form:
 *
 *     T?
 *
 * Examples:
 *
 *     int?
 *     string?
 *     User?
 *     Qubit?
 *     Vec<int>?
 *     (int, bool)?
 *     [int]?
 *     [int; N]?
 *     fn(int) -> bool?
 *
 * The surrounding canonical type grammar owns `typeExpression`.
 *
 * This file therefore deliberately does NOT define:
 *
 *     typeExpression
 *     typeAtom
 *     primitiveType
 *     namedType
 *     genericType
 *     tupleType
 *     arrayType
 *     functionType
 *     referenceType
 *     pointerType
 *     identifier
 *     genericArguments
 *
 * Those belong to their authoritative grammar components.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - optional-type marker syntax;
 *     - the parser-level optional marker;
 *     - the integration boundary used by Types.typePostfix;
 *     - optional-marker syntax compatibility.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the complete type-expression grammar;
 *     - generic applications;
 *     - the `Option<T>` generic constructor;
 *     - `Some` / `nil` / `null` values;
 *     - option construction expressions;
 *     - pattern matching;
 *     - nullability analysis;
 *     - type inference;
 *     - type checking;
 *     - ownership;
 *     - borrowing;
 *     - resource allocation;
 *     - quantum allocation;
 *     - physical qubit selection;
 *     - hardware selection;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - canonical quantum IR;
 *     - classical IR;
 *     - runtime representation;
 *     - ABI layout;
 *     - target-specific representation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     Types.typeExpression
 *          |
 *          v
 *     typePostfix
 *          |
 *          v
 *     OptionTypes.optionalMarker
 *          |
 *          v
 *     AST builder
 *          |
 *          v
 *     TypeExpr::Optional(Box<TypeExpr>)
 *          |
 *          v
 *     semantic type analysis
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     classical semantics   quantum/resource semantics
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *                 canonical IR
 *
 * The grammar itself never depends on the downstream semantic or IR layers.
 *
 * ============================================================================
 * CANONICAL AST BOUNDARY
 * ============================================================================
 *
 * The canonical source-level representation is:
 *
 *     TypeExpr::Optional(Box<TypeExpr>)
 *
 * This grammar does NOT introduce:
 *
 *     OptionalTypeNode
 *     OptionalTypeAst
 *     OptionalTypeIr
 *     OptionTypeExpression
 *
 * or another competing representation.
 *
 * `OptionalType` in the frontend is a typed façade over `TypeExpr::Optional`.
 *
 * The parser/AST builder is responsible for transforming:
 *
 *     typeExpression + optionalMarker
 *
 * into:
 *
 *     TypeExpr::Optional(Box<TypeExpr>)
 *
 * This grammar only recognizes the syntax.
 *
 * ============================================================================
 * CRITICAL COMPOSITION RULE
 * ============================================================================
 *
 * Do NOT write:
 *
 *     optionalType
 *         : typeExpression QUESTION
 *         ;
 *
 * in this delegate.
 *
 * `typeExpression` is owned by `Types`.
 *
 * If this file attempted to consume `typeExpression` directly, the grammar
 * composition would create recursive ownership and make the optional delegate
 * depend on the grammar that imports it.
 *
 * Instead this file owns:
 *
 *     optionalMarker
 *
 * and the canonical Types grammar owns:
 *
 *     typePostfix
 *
 * Conceptually:
 *
 *     typeExpression
 *         : typeQualifier* typeAtom typePostfix*
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
 * This gives OptionTypes a single, precise responsibility.
 *
 * ============================================================================
 * SURFACE SYNTAX
 * ============================================================================
 *
 * The canonical postfix form is:
 *
 *     T?
 *
 * NOT:
 *
 *     ?T
 *
 * and not:
 *
 *     OptionalType<T>
 *
 * unless a separate generic constructor is used.
 *
 * The existing canonical AST formatter represents optionality as:
 *
 *     <inner>?
 *
 * and the frontend OptionalType façade explicitly identifies
 * `TypeExpr::Optional` as the authoritative representation.
 *
 * Therefore the postfix spelling is preserved here.
 *
 * ============================================================================
 * NESTED OPTIONAL TYPES
 * ============================================================================
 *
 * The grammar intentionally permits repeated postfix markers through the
 * surrounding `typePostfix*` rule.
 *
 * Therefore:
 *
 *     T?
 *     T??
 *     T???
 *
 * are syntactically representable.
 *
 * This grammar MUST NOT impose:
 *
 *     MAX_OPTIONAL_DEPTH
 *     MAX_OPTIONAL_NESTING
 *     MAX_TYPE_DEPTH
 *
 * Whether nested optionality is semantically meaningful or whether it should
 * normalize to another representation is a semantic/type-system decision.
 *
 * The grammar preserves source structure and does not silently normalize it.
 *
 * ============================================================================
 * GENERIC OPTION INTEGRATION
 * ============================================================================
 *
 * `Option<T>` is NOT owned by this file.
 *
 * It is a generic type application and therefore belongs to:
 *
 *     grammar/types/generic-types.g4
 *
 * Examples:
 *
 *     Option<int>
 *     Option<User>
 *     Option<Qubit>
 *     Option<Vec<int>>
 *     Option<Result<T, E>>
 *
 * This file owns only postfix optionality:
 *
 *     int?
 *     User?
 *     Qubit?
 *
 * If the language specification chooses both spellings, they remain distinct
 * source constructs until semantic analysis determines whether they have
 * equivalent semantics.
 *
 * This avoids making `Option<T>` and `T?` accidentally share syntax ownership.
 *
 * ============================================================================
 * VALUE-LEVEL OPTION INTEGRATION
 * ============================================================================
 *
 * Optional type syntax is distinct from optional value syntax.
 *
 * Existing lexical tokens include constructs such as:
 *
 *     Some
 *     nil
 *     null
 *
 * Those are value-level vocabulary and are NOT parsed by this grammar.
 *
 * This file does not define:
 *
 *     Some(value)
 *     None
 *     null
 *     nil
 *
 * nor does it decide which value constructors correspond to an optional type.
 *
 * That responsibility belongs to expression/value grammar and semantic
 * analysis.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The authoritative lexer vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar name is:
 *
 *     ZamaniTokens
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniTokens
 *
 * is mandatory.
 *
 * The optional marker is the canonical lexer token:
 *
 *     QUESTION
 *
 * corresponding to:
 *
 *     ?
 *
 * This grammar MUST NOT redeclare:
 *
 *     QUESTION
 *     '?'
 *
 * or any other lexer rule.
 *
 * ============================================================================
 * TOKEN OWNERSHIP
 * ============================================================================
 *
 * Lexer:
 *
 *     '?' -> QUESTION
 *
 * Parser:
 *
 *     QUESTION -> optionalMarker
 *
 * AST:
 *
 *     optionalMarker + inner TypeExpr
 *         -> TypeExpr::Optional(Box<TypeExpr>)
 *
 * Semantic analysis:
 *
 *     TypeExpr::Optional
 *         -> semantic optional/nullability representation
 *
 * ============================================================================
 * TYPE-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * `Types` remains the sole owner of:
 *
 *     typeExpression
 *
 * The intended composition is:
 *
 *     Types
 *       |
 *       +--> typePostfix
 *                |
 *                +--> OptionTypes.optionalMarker
 *
 * The exact import/delegation mechanism must follow the repository's ANTLR
 * grammar-composition arrangement.
 *
 * No second `typeExpression` is introduced here.
 *
 * ============================================================================
 * COMPOSITE-TYPES INTEGRATION
 * ============================================================================
 *
 * `grammar/types/composite-types.g4` identifies optional types as one of the
 * repository's composite type families.
 *
 * That classification MUST NOT be implemented by duplicating the recursive
 * `T?` grammar here.
 *
 * The canonical ownership is:
 *
 *     OptionTypes
 *         |
 *         +--> optionalMarker
 *
 *     Types
 *         |
 *         +--> typePostfix
 *                 |
 *                 +--> optionalMarker
 *
 *     CompositeTypes
 *         |
 *         +--> aggregate/classification boundary where required
 *
 * This prevents multiple grammars from recognizing the same complete optional
 * type syntax independently.
 *
 * ============================================================================
 * GENERIC-TYPES INTEGRATION
 * ============================================================================
 *
 * `grammar/types/generic-types.g4` owns generic application syntax.
 *
 * Therefore:
 *
 *     Option<T>
 *
 * remains a generic application.
 *
 * This file MUST NOT redefine:
 *
 *     genericType
 *     genericArguments
 *     genericArgument
 *     typePath
 *
 * Examples of valid composition:
 *
 *     Option<T>?
 *     Vec<Option<T>>
 *     Option<Vec<T>?>
 *
 * subject to the canonical type-expression composition and semantic rules.
 *
 * ============================================================================
 * ARRAY / TUPLE / FUNCTION INTEGRATION
 * ============================================================================
 *
 * Optionality applies to the complete preceding type expression.
 *
 * Examples:
 *
 *     int?
 *
 *     (int, bool)?
 *
 *     [int]?
 *
 *     [int; N]?
 *
 *     fn(int) -> bool?
 *
 * The canonical `Types.typeExpression` parser determines the inner structure.
 *
 * This delegate only supplies the `?` postfix marker.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Optionality is domain-neutral.
 *
 * Syntactically, the following can be represented if their inner types are
 * valid according to the canonical type grammar:
 *
 *     qubit?
 *
 *     Qubit?
 *
 *     Register<Qubit>?
 *
 *     QuantumState<T>?
 *
 * This does NOT mean:
 *
 *     a physical qubit is absent;
 *     a physical qubit is released;
 *     a QPU allocation is optional;
 *     a hardware resource may disappear;
 *     a measurement occurred;
 *     a reset occurred;
 *     a qubit was deallocated.
 *
 * Those meanings belong to semantic quantum/resource layers.
 *
 * This grammar MUST NOT depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     hardware
 *     routing
 *     scheduling
 *     resilience
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Optionality may wrap arbitrary source-level classical types:
 *
 *     int?
 *     float?
 *     bool?
 *     string?
 *     UserType?
 *     Vec<int>?
 *
 * The grammar does not decide:
 *
 *     representation;
 *     tagging;
 *     null-pointer representation;
 *     sentinel representation;
 *     memory layout;
 *     register representation;
 *     ABI representation.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Optionality may syntactically wrap a source-level hardware or HDL
 * abstraction when the canonical type grammar admits that abstraction.
 *
 * For example, a future semantic type could permit:
 *
 *     HardwareSignal<T>?
 *
 * This does not mean:
 *
 *     optional physical wire;
 *     optional FPGA register;
 *     optional device;
 *     optional pin;
 *     optional clock;
 *     optional hardware address.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Optionality may wrap distributed/service/resource abstractions:
 *
 *     Remote<T>?
 *     Service<T>?
 *     Resource<T>?
 *
 * The grammar does not interpret absence as:
 *
 *     node failure;
 *     network failure;
 *     service failure;
 *     resource deallocation.
 *
 * Such interpretation belongs to distributed/runtime/resilience semantics.
 *
 * ============================================================================
 * RESOURCE / POCO-REAF INTEGRATION
 * ============================================================================
 *
 * Optionality expresses source-level type semantics.
 *
 * It does not allocate resources.
 *
 * For example:
 *
 *     Qubit?
 *
 * does not request:
 *
 *     one physical qubit;
 *
 * nor does:
 *
 *     Resource<T>?
 *
 * reserve or release any runtime resource.
 *
 * Resource requirements, capabilities, preferences, constraints, placement,
 * scheduling and target realization remain downstream.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser recognizes:
 *
 *     T?
 *
 * The AST builder constructs:
 *
 *     TypeExpr::Optional(Box<T>)
 *
 * Semantic analysis determines:
 *
 *     - whether T may be optional;
 *     - whether nested optionality is legal;
 *     - absence semantics;
 *     - nullability semantics;
 *     - pattern-matching behavior;
 *     - flow-sensitive narrowing;
 *     - ownership interaction;
 *     - borrowing interaction;
 *     - resource semantics;
 *     - conversion/coercion rules;
 *     - generic interaction;
 *     - quantum/resource legality.
 *
 * None of these decisions belong in this grammar.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * This grammar has NO direct dependency on any IR.
 *
 * The intended pipeline is:
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
 *     canonical IR
 *
 * If optionality affects quantum semantics, the semantic layer decides how
 * that information is represented before interaction with:
 *
 *     quantum::ir
 *
 * This grammar must never import, reference or construct quantum IR nodes.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages may impose explicit resource/safety policies.
 *
 * Such policies MUST NOT be encoded here as grammar constants.
 *
 * For example, a compiler invocation may choose an explicit policy for:
 *
 *     maximum parse depth;
 *     maximum AST size;
 *     maximum compilation memory;
 *     maximum diagnostic count.
 *
 * Those are implementation/security policies, not Zamani syntax.
 *
 * Therefore this file contains no:
 *
 *     MAX_OPTIONALS
 *     MAX_TYPE_DEPTH
 *     MAX_AST_DEPTH
 *     MAX_PROGRAM_SIZE
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * There is no runtime dependency.
 *
 * Runtime representation of optional values may differ by target:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     embedded system
 *     distributed runtime
 *
 * without changing the source grammar.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Formatter:
 *
 *     canonical optional spelling -> T?
 *
 * Parser diagnostics:
 *
 *     unexpected QUESTION
 *     malformed preceding type
 *
 * Syntax highlighting:
 *
 *     QUESTION is an optional-type postfix marker only in type context.
 *
 * Documentation tooling:
 *
 *     should identify optionality through the canonical AST rather than
 *     attempting to infer semantics directly from raw source text.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded target-language actions;
 *     - no random behavior;
 *     - no machine-dependent state;
 *     - no generated identifiers;
 *     - no runtime calls.
 *
 * Equal token streams therefore produce equal optional-marker parse structure
 * under the same grammar/version.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * This grammar does not silently reinterpret malformed syntax.
 *
 * Examples that must remain syntax errors when the surrounding type grammar
 * cannot form a valid type:
 *
 *     ?
 *     ??
 *     int?
 *     ?
 *
 * The final validity of:
 *
 *     int??
 *
 * depends on the canonical repeated-postfix policy. This delegate itself does
 * not impose a nesting limit.
 *
 * Parser recovery belongs to the canonical parser/error-diagnostic layer.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no command execution;
 *     - performs no runtime evaluation;
 *     - contains no embedded Rust;
 *     - contains no unsafe operations;
 *     - contains no hardware access;
 *     - contains no resource allocation;
 *     - contains no external-process invocation.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is no fixed optionality limit.
 *
 * The grammar does not encode:
 *
 *     MAX_OPTIONAL_DEPTH
 *     MAX_OPTIONAL_SIZE
 *     MAX_OPTIONAL_TYPES
 *     MAX_GENERIC_ARITY
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * A source program can therefore express optionality at whatever structural
 * scale the compiler and available resources can process.
 *
 * Any practical parser/compiler limit must be an explicit implementation
 * policy rather than a language grammar constant.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical optional syntax:
 *
 *     T?
 *
 * is preserved.
 *
 * Existing generic syntax:
 *
 *     Option<T>
 *
 * remains owned by GenericTypes.
 *
 * These two mechanisms must not be silently collapsed by the parser.
 *
 * Semantic equivalence, if desired, belongs to semantic normalization.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This is ANTLR grammar source and contains no Rust implementation code.
 *
 * The surrounding Zamani frontend/compiler must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must forbid unsafe code:
 *
 *     #![forbid(unsafe_code)]
 *
 * This grammar introduces no unsafe operation.
 *
 * ============================================================================
 * TESTING CONTRACT
 * ============================================================================
 *
 * Tests belong under the repository's grammar test hierarchy.
 *
 * REQUIRED POSITIVE TESTS:
 *
 *     int?
 *     bool?
 *     string?
 *     User?
 *     Qubit?
 *     Vec<int>?
 *     Option<int>?
 *     (int, bool)?
 *     [int]?
 *     [int; N]?
 *     fn(int) -> bool?
 *
 * REQUIRED NESTING TESTS:
 *
 *     int??
 *     int???
 *     Vec<Option<int>?>
 *     Option<Vec<int>?>?
 *
 * REQUIRED NEGATIVE TESTS:
 *
 *     ?
 *     ?
 *     malformed type followed by ?
 *     missing type before ?
 *
 * REQUIRED CROSS-DOMAIN TESTS:
 *
 *     Qubit?
 *     QuantumState<T>?
 *     Resource<T>?
 *     HardwareType?
 *     Distributed<T>?
 *
 * where the referenced domain types are provided by their canonical grammar
 * components.
 *
 * REQUIRED SCALABILITY TESTS:
 *
 *     no finite optional nesting constant;
 *     no finite resource constant;
 *     no finite qubit constant;
 *     no machine-dependent syntax.
 *
 * REQUIRED DETERMINISM TEST:
 *
 *     identical token stream + identical grammar version
 *         => identical parse tree
 *
 * REQUIRED ROUND-TRIP TEST:
 *
 *     T?
 *       -> AST
 *       -> formatter
 *       -> T?
 *
 * with semantic equivalence preserved.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_OPTIONAL_DEPTH
 *     MAX_TYPE_DEPTH
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *     physical device identifiers
 *     topology identifiers
 *     hardware addresses
 *     fixed resource capacities
 *
 * The only concrete syntax owned here is:
 *
 *     QUESTION
 *
 * which is a language token, not a machine limitation.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when ALL of the following are true:
 *
 *     [ ] Grammar name is OptionTypes.
 *     [ ] tokenVocab is ZamaniTokens.
 *     [ ] QUESTION is consumed from the canonical lexer.
 *     [ ] No lexer rule is duplicated.
 *     [ ] No typeExpression rule is duplicated.
 *     [ ] No generic application grammar is duplicated.
 *     [ ] No Option<T> grammar is duplicated.
 *     [ ] Optionality is represented by the canonical postfix marker.
 *     [ ] Types remains the owner of recursive typeExpression.
 *     [ ] AST lowering produces TypeExpr::Optional.
 *     [ ] OptionalType remains only an AST façade.
 *     [ ] No second optional AST is introduced.
 *     [ ] No IR dependency exists.
 *     [ ] No quantum::ir dependency exists.
 *     [ ] No QEC/ZQN dependency exists.
 *     [ ] No hardware dependency exists.
 *     [ ] No runtime dependency exists.
 *     [ ] No machine-size assumption exists.
 *     [ ] No optional-depth limit exists in grammar.
 *     [ ] Deterministic parsing is preserved.
 *     [ ] Diagnostics remain owned by the parser/diagnostic layer.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Nested/boundary tests exist.
 *     [ ] Cross-domain tests exist.
 *     [ ] Round-trip tests exist.
 *     [ ] Rust integration remains compatible with Rust 1.97/1.97.1.
 *     [ ] Rust integration contains no unsafe code.
 *
 * ============================================================================
 */

parser grammar OptionTypes;

options {
    tokenVocab = ZamaniTokens;
}

/**
 * Canonical optional-type postfix marker.
 *
 * This rule intentionally consumes only the marker.
 *
 * The surrounding `Types` grammar owns the recursive `typeExpression` and
 * applies this rule through its postfix-type composition.
 *
 * Examples of complete types produced by the composed grammar:
 *
 *     int?
 *     User?
 *     Qubit?
 *     Vec<int>?
 *     (int, bool)?
 *     [int; N]?
 *
 * The AST builder combines the preceding type expression with this marker to
 * produce:
 *
 *     TypeExpr::Optional(Box<TypeExpr>)
 *
 * No semantic interpretation occurs here.
 */
optionalMarker
    : QUESTION
    ;