/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/option.g4
 *
 * Grammar:
 *     Option
 *
 * Role:
 *     CANONICAL OPTIONAL-TYPE SYNTAX DELEGATE
 *
 * Status:
 *     Production
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE parser-level owner of the postfix optional-type
 * marker:
 *
 *     ?
 *
 * Canonical source form:
 *
 *     T?
 *
 * Examples:
 *
 *     int?
 *     bool?
 *     User?
 *     Qubit?
 *     Vec<int>?
 *     Option<T>?
 *     (int, bool)?
 *     [T]?
 *     [T; N]?
 *     fn(T) -> U?
 *
 * IMPORTANT:
 *
 * This file owns ONLY the optional postfix marker.
 *
 * It does NOT own:
 *
 *     typeExpression
 *     typeCore
 *     typeAtom
 *     typePath
 *     namedType
 *     genericType
 *     genericTypeArguments
 *     tupleType
 *     arrayType
 *     sliceType
 *     functionType
 *     referenceType
 *     pointerType
 *     Result<T, E>
 *     quantum types
 *     dependent types
 *     value-level option constructors
 *     semantic nullability
 *     type inference
 *     type normalization
 *     ownership
 *     borrowing
 *     resource allocation
 *     capability negotiation
 *     hardware selection
 *     quantum allocation
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime representation
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Types.typeExpression
 *       |
 *       +--> typeCore
 *       |
 *       +--> typePostfix
 *                 |
 *                 +--> Option.optionalMarker
 *       |
 *       v
 *     frontend TypeExpr
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type analysis
 *       |
 *       +--------------------+----------------------+
 *       |                    |                      |
 *       v                    v                      v
 *   classical           quantum semantics       other domains
 *                            |
 *                            v
 *                        quantum::ir
 *
 * Optional syntax remains completely independent of target realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - canonical optional-type postfix syntax;
 *     - optionalMarker;
 *     - the parser integration boundary for `T?`;
 *     - source-level optional postfix compatibility.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - complete type expressions;
 *     - generic Option<T>;
 *     - optional values;
 *     - null values;
 *     - Some/None/nil/null constructors;
 *     - pattern matching;
 *     - flow-sensitive narrowing;
 *     - semantic nullability;
 *     - ownership semantics;
 *     - resource semantics;
 *     - quantum semantics;
 *     - HDL semantics;
 *     - hardware semantics;
 *     - IR construction.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical QUESTION token
 *
 * EXPORTS:
 *
 *     optionalMarker
 *     optionalTypePostfix
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *     grammar/types/composite-types.g4 where classification is required
 *     future type-composition grammars
 *
 * AST_OWNER:
 *
 *     existing frontend TypeExpr
 *
 * SEMANTIC_OWNER:
 *
 *     canonical type semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic/type IR
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/
 *     repository parser/type conformance tests
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The lexer is authoritative elsewhere.
 *
 * This parser grammar MUST NOT define:
 *
 *     QUESTION
 *     QUESTION_MARK
 *     '?'
 *
 * The canonical lexical contract is:
 *
 *     '?' -> QUESTION
 *
 * Therefore this file consumes:
 *
 *     QUESTION
 *
 * The lexer migration must establish exactly one owner for `?`.
 *
 * No second token such as:
 *
 *     QUESTION_MARK
 *     Question
 *     QuestionMark
 *
 * may coexist as an independent lexical identity for the same spelling.
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * `optionalMarker` is the primary public rule.
 *
 * It recognizes exactly:
 *
 *     QUESTION
 *
 * The complete type:
 *
 *     T?
 *
 * is assembled by the canonical Types grammar.
 *
 * This prevents circular ownership such as:
 *
 *     Option -> typeExpression
 *     Types -> Option
 *     typeExpression -> Option -> typeExpression
 *
 * ============================================================================
 * CANONICAL GRAMMAR
 * ============================================================================
 */

parser grammar Option;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * OPTIONAL TYPE POSTFIX
 * ============================================================================
 *
 * This is deliberately a one-token rule.
 *
 * The enclosing Types grammar owns:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *
 * and composes:
 *
 *     typePostfix
 *         : optionalMarker
 *         ;
 *
 * Therefore:
 *
 *     T?
 *
 * is structurally parsed as:
 *
 *     preceding type expression
 *         +
 *     optionalMarker
 *
 * The AST builder then constructs:
 *
 *     TypeExpr::Optional(Box<TypeExpr>)
 * ============================================================================
 */

optionalMarker
    : QUESTION
    ;


/*
 * ============================================================================
 * DESCRIPTIVE POSTFIX ALIAS
 * ============================================================================
 *
 * This rule is intentionally only an alias to the canonical marker.
 *
 * It exists for tooling/composition that wants to refer explicitly to an
 * optional postfix constructor without owning a complete type expression.
 *
 * It MUST NOT acquire independent semantics.
 * ============================================================================
 */

optionalTypePostfix
    : optionalMarker
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO AST node.
 *
 * The existing AST representation remains:
 *
 *     TypeExpr::Optional(Box<TypeExpr>)
 *
 * Therefore:
 *
 *     int?
 *
 * becomes structurally:
 *
 *     TypeExpr::Optional(
 *         Box::new(TypeExpr::Identifier(...))
 *     )
 *
 * The grammar itself does not construct that Rust value.
 *
 * The parser/AST conversion layer owns that transformation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar establishes only:
 *
 *     optionality is a postfix type constructor.
 *
 * It does NOT establish that:
 *
 *     optional == nullable
 *     optional == pointer
 *     optional == null reference
 *     optional == absent resource
 *     optional == unavailable device
 *     optional == failed allocation
 *     optional == failed network node
 *     optional == absent qubit
 *     optional == measurement result
 *
 * Those are semantic questions.
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the inner type may be optional;
 *     - whether nested optionality is permitted;
 *     - whether nested optionals normalize;
 *     - whether optionality participates in subtyping;
 *     - conversion rules;
 *     - pattern matching;
 *     - flow-sensitive narrowing;
 *     - ownership interaction;
 *     - borrowing interaction;
 *     - generic substitution;
 *     - trait/bound interaction;
 *     - domain-specific legality.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Optionality applies to an already-valid source-level type.
 *
 * Examples:
 *
 *     int?
 *     User?
 *     Vec<int>?
 *     Result<T, E>?
 *     (A, B)?
 *     fn(A) -> B?
 *
 * The type grammar determines the complete preceding type.
 *
 * This file does not inspect or validate the inner type.
 *
 * ============================================================================
 * GENERIC OPTION CONTRACT
 * ============================================================================
 *
 * `Option<T>` is NOT owned by this file.
 *
 * It is a generic type application.
 *
 * Therefore:
 *
 *     Option<T>
 *
 * and:
 *
 *     T?
 *
 * are distinct syntactic constructs.
 *
 * Both may be valid:
 *
 *     Option<T>?
 *     Vec<Option<T>>
 *     Option<Vec<T>?>
 *     Result<Option<T>, E>
 *
 * Their semantic equivalence, if any, is decided by type semantics.
 *
 * This prevents:
 *
 *     generic syntax
 *
 * from becoming coupled to:
 *
 *     postfix optional syntax.
 *
 * ============================================================================
 * VALUE-LEVEL CONTRACT
 * ============================================================================
 *
 * This file does NOT parse optional values.
 *
 * It does not own:
 *
 *     Some(value)
 *     None
 *     nil
 *     null
 *
 * Value constructors belong to expression/value grammar and semantic
 * analysis.
 *
 * This separation is mandatory:
 *
 *     TYPE:
 *         T?
 *
 *     VALUE:
 *         optional-value expression
 *
 * must not become one grammar responsibility.
 *
 * ============================================================================
 * NESTED OPTIONALITY
 * ============================================================================
 *
 * This file deliberately does not establish a finite nesting limit.
 *
 * If the enclosing type grammar permits repeated postfixes, structurally:
 *
 *     T?
 *     T??
 *     T???
 *
 * may reach the parser.
 *
 * The semantic layer decides whether:
 *
 *     T??
 *
 * is:
 *
 *     - legal nested optionality;
 *     - normalized;
 *     - rejected;
 *     - represented distinctly.
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_OPTIONAL_DEPTH
 *     MAX_OPTIONAL_NESTING
 *     MAX_TYPE_DEPTH
 *
 * or equivalent language-level limits.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Optionality is domain-neutral.
 *
 * Classical examples:
 *
 *     int?
 *     float?
 *     bool?
 *     string?
 *     User?
 *     Tensor<T>?
 *
 * Runtime representation is unspecified here.
 *
 * The backend may choose an appropriate representation according to:
 *
 *     type semantics
 *     target capabilities
 *     ABI
 *     optimization
 *     resource availability
 *
 * without changing source syntax.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Optionality may wrap source-level quantum abstractions when those types are
 * admitted by the canonical type system:
 *
 *     Qubit?
 *     LogicalQubit?
 *     QuantumState<T>?
 *     QuantumRegister<T>?
 *
 * This does NOT mean:
 *
 *     optional physical qubit;
 *     optional QPU;
 *     optional allocation;
 *     automatic release;
 *     measurement;
 *     reset;
 *     hardware failure.
 *
 * Quantum meaning remains downstream.
 *
 * Required boundary:
 *
 *     source type
 *         |
 *         v
 *     TypeExpr::Optional
 *         |
 *         v
 *     semantic quantum type
 *         |
 *         v
 *     quantum::ir
 *
 * This file MUST NOT import or construct quantum::ir.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Optionality may wrap source-level hardware abstractions:
 *
 *     Signal<T>?
 *     Register<T>?
 *     Resource<Device>?
 *     Accelerator<Model>?
 *
 * It does not represent:
 *
 *     optional physical wire;
 *     optional register;
 *     optional pin;
 *     optional clock;
 *     optional FPGA resource;
 *     optional ASIC component.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Optionality can wrap source-level distributed abstractions:
 *
 *     Service<T>?
 *     Remote<T>?
 *     Channel<T>?
 *     DistributedState<T>?
 *
 * The grammar does not interpret absence as:
 *
 *     network failure;
 *     node failure;
 *     service failure;
 *     process failure;
 *     resource retirement.
 *
 * Runtime/resilience semantics determine those meanings.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A type such as:
 *
 *     Resource<T>?
 *
 * does NOT allocate or reserve T.
 *
 * This file performs no:
 *
 *     capability negotiation;
 *     resource discovery;
 *     placement;
 *     scheduling;
 *     memory allocation;
 *     device selection;
 *     qubit allocation.
 *
 * Resource requirements remain separate:
 *
 *     requires capability(...);
 *     requires memory >= required_memory;
 *     requires qubits >= required_qubits;
 *
 * Optionality itself creates no resource requirement.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing an optional type has no effect.
 *
 * The presence of:
 *
 *     T?
 *
 * MUST NOT automatically introduce:
 *
 *     IO
 *     network
 *     randomness
 *     mutation
 *     measurement
 *     learning
 *     adaptation
 *     native execution
 *     FFI
 *
 * Effects belong to executable constructs and semantic operations.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * This grammar requires no capability.
 *
 * The existence of an optional type must not cause the compiler to request:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     network
 *     memory
 *     storage
 *
 * Capability checking belongs downstream.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Optional types may appear inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This file does not define those contracts.
 *
 * The type checker may use optional-type information when validating them.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Optional types may occur inside constructs controlled by policies.
 *
 * This grammar does not define or evaluate:
 *
 *     permissions;
 *     prohibitions;
 *     sandbox policies;
 *     deployment policies;
 *     execution policies;
 *     resource policies.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser/frontend must preserve source spans for:
 *
 *     - the `?` marker;
 *     - the preceding type expression;
 *     - the complete optional type.
 *
 * This enables:
 *
 *     diagnostics;
 *     formatting;
 *     IDE/LSP;
 *     refactoring;
 *     source maps;
 *     provenance;
 *     reproducible compilation.
 *
 * The grammar itself does not construct provenance records.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has NO direct IR dependency.
 *
 * Required pipeline:
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
 *       +-----------------------+
 *       |                       |
 *       v                       v
 *   classical semantics    quantum semantics
 *                               |
 *                               v
 *                           quantum::ir
 *
 * There is no:
 *
 *     OptionalIR
 *     OptionalQuantumIR
 *     OptionalHardwareIR
 *
 * introduced here.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * The backend may lower optional types differently according to target
 * capabilities.
 *
 * Possible implementation strategies include:
 *
 *     tagged representation;
 *     discriminant + payload;
 *     nullable representation;
 *     optimized representation;
 *     erased representation when proven unnecessary.
 *
 * These choices are target realization details.
 *
 * They MUST NOT change:
 *
 *     T?
 *
 * source semantics.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO artificial limits for:
 *
 *     optional nesting;
 *     type nesting;
 *     generic arity;
 *     program size;
 *     number of types;
 *     number of quantum values;
 *     number of hardware resources;
 *     number of devices;
 *     number of nodes;
 *     memory capacity;
 *     register width;
 *     tensor rank;
 *     network size.
 *
 * In particular, this file contains none of:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * "Unlimited" here means:
 *
 *     no language-level artificial machine ceiling.
 *
 * It does not claim that physical hardware or compiler resources are
 * infinite.
 *
 * Practical parser/compiler limits must be explicit implementation/security
 * policies outside the grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - has no semantic predicates;
 *     - has no target-language actions;
 *     - has no runtime calls;
 *     - has no I/O;
 *     - has no random state;
 *     - has no hardware inspection;
 *     - has no environment-dependent behavior.
 *
 * Therefore identical token streams under the same grammar/version produce
 * identical optional-marker parse structure.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This file:
 *
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - executes no source code;
 *     - accesses no hardware;
 *     - allocates no resources;
 *     - contains no embedded Rust;
 *     - contains no semantic predicates;
 *     - requires no unsafe Rust.
 *
 * Generated Rust integration remains subject to the repository's:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * policy.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/types/optional.g4
 *     grammar/types/option-types.g4
 *
 * and `grammar/types/types.g4` currently contains its own optional postfix
 * rule.
 *
 * These MUST NOT remain three independent owners.
 *
 * Production migration:
 *
 *     grammar/types/option.g4
 *              |
 *              v
 *        optionalMarker
 *              |
 *              v
 *     grammar/types/types.g4
 *
 * Existing:
 *
 *     grammar/types/optional.g4
 *
 * becomes a compatibility/delegation surface.
 *
 * Existing:
 *
 *     grammar/types/option-types.g4
 *
 * becomes a compatibility/delegation surface.
 *
 * Neither compatibility surface may redefine `optionalMarker`.
 *
 * `grammar/types/types.g4` must remove its local:
 *
 *     typePostfix : QUESTION_MARK ;
 *
 * and instead compose:
 *
 *     typePostfix
 *         : optionalMarker
 *         ;
 *
 * This is a structural ownership correction, not a language feature change.
 *
 * ============================================================================
 * LEXICAL MIGRATION CONTRACT
 * ============================================================================
 *
 * The current repository has inconsistent optional-marker naming.
 *
 * The final canonical spelling must be:
 *
 *     QUESTION
 *
 * for:
 *
 *     ?
 *
 * Therefore the lexical layer must:
 *
 *     1. add exactly one QUESTION token;
 *     2. assign `?` to exactly one lexical owner;
 *     3. remove/deprecate QUESTION_MARK;
 *     4. remove any duplicate Question/QuestionMark token;
 *     5. preserve `?.` as QUESTION_DOT;
 *     6. preserve `??` as NULL_COALESCE;
 *     7. verify maximal matching.
 *
 * This file must NOT be changed when those lexer corrections are made.
 *
 * That is deliberate: its dependency contract is established here in advance.
 *
 * ============================================================================
 * TYPES.G4 INTEGRATION CONTRACT
 * ============================================================================
 *
 * `grammar/types/types.g4` remains the sole owner of:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *
 * It should compose this file so that:
 *
 *     typePostfix
 *         : optionalMarker
 *         ;
 *
 * The final structure is:
 *
 *     typeExpression
 *         : typeQualifier*
 *           typeCore
 *           typePostfix*
 *         ;
 *
 * This allows optionality to remain a reusable postfix constructor without
 * duplicating the type system.
 *
 * ============================================================================
 * COMPOSITE TYPES INTEGRATION
 * ============================================================================
 *
 * `grammar/types/composite-types.g4` may classify optional types, but must not
 * recognize `T?` independently.
 *
 * If it needs optionality:
 *
 *     import/use Option.optionalMarker
 *
 * rather than defining another optional rule.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic type syntax remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * or the canonical generic composition selected by the type system.
 *
 * This file does not define:
 *
 *     genericType
 *     genericArguments
 *     genericArgumentList
 *
 * Therefore:
 *
 *     Option<T>?
 *
 * is assembled as:
 *
 *     genericType
 *       +
 *     optionalMarker
 *
 * ============================================================================
 * RESULT INTEGRATION
 * ============================================================================
 *
 * Result syntax remains independent:
 *
 *     Result<T, E>
 *
 * Optionality can wrap it:
 *
 *     Result<T, E>?
 *
 * but this grammar does not own Result syntax.
 *
 * ============================================================================
 * PATTERN-MATCHING INTEGRATION
 * ============================================================================
 *
 * Optional values may be consumed by the existing pattern subsystem.
 *
 * For example, semantic layers may support patterns corresponding to:
 *
 *     Some(value)
 *     None
 *
 * or equivalent canonical patterns.
 *
 * Pattern syntax does not belong here.
 *
 * ============================================================================
 * TYPE-CHECKING INTEGRATION
 * ============================================================================
 *
 * The type checker must:
 *
 *     - resolve the inner type;
 *     - construct the semantic optional type;
 *     - preserve nested structure until normalization policy is applied;
 *     - report invalid optional-type uses;
 *     - preserve source spans;
 *     - avoid target-specific assumptions.
 *
 * The grammar must never perform these operations.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime representation is target-dependent.
 *
 * The same source-level:
 *
 *     T?
 *
 * must remain source-compatible across:
 *
 *     tiny systems;
 *     embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future computational targets.
 *
 * provided the target can satisfy the semantic program requirements.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Formatter:
 *
 *     canonical spelling is `T?`.
 *
 * Parser diagnostics:
 *
 *     preserve the source span of `QUESTION`.
 *
 * Syntax highlighting:
 *
 *     `QUESTION` receives optional-type styling only in type context.
 *
 * Documentation:
 *
 *     derive optionality from the AST/type model, not from raw token matching.
 *
 * IDE/LSP:
 *
 *     hover and type information should use the canonical semantic TypeExpr.
 *
 * Refactoring:
 *
 *     transformations must preserve optionality semantics.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser-level malformed complete types include:
 *
 *     ?
 *     ?T
 *
 * These are rejected by the enclosing type grammar because there is no
 * preceding type expression.
 *
 * This file itself should not create custom semantic diagnostics.
 *
 * Semantic diagnostics include, but are not limited to:
 *
 *     optionality not permitted for a type;
 *     invalid nested optionality;
 *     invalid conversion;
 *     invalid ownership interaction;
 *     invalid resource interaction.
 *
 * Those belong downstream.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     int?
 *     bool?
 *     string?
 *     User?
 *     Qubit?
 *     Vec<int>?
 *     Option<int>?
 *     Result<int, Error>?
 *     (int, bool)?
 *     [int]?
 *     [int; N]?
 *     fn(int) -> bool?
 *
 * NESTED:
 *
 *     int??
 *     int???
 *     Vec<int?>?
 *     Option<Vec<int>?>?
 *
 * CROSS-DOMAIN:
 *
 *     Qubit?
 *     QuantumState<T>?
 *     Resource<T>?
 *     Signal<T>?
 *     Accelerator<T>?
 *     Service<T>?
 *
 * NEGATIVE COMPLETE-TYPE CASES:
 *
 *     ?
 *     ?int
 *
 * These negative cases are tested through the composed Types grammar.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Verify:
 *
 *     Option<T>?
 *     Result<Option<T>, E>?
 *     Vec<Option<T>?>
 *     fn(T?) -> U?
 *     (T?, U?)?
 *     Resource<Qubit?>?
 *
 * Also verify interaction with:
 *
 *     references;
 *     pointers;
 *     linear types;
 *     affine types;
 *     dependent types;
 *     quantum types;
 *     hardware abstractions;
 *     distributed abstractions.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Generated tests must exercise increasing:
 *
 *     optional nesting;
 *     generic nesting;
 *     type-expression depth;
 *     source size.
 *
 * The grammar must not contain a semantic ceiling.
 *
 * Resource exhaustion during testing must be reported as an implementation
 * resource limit, never converted into a language-level limit.
 *
 * ============================================================================
 * DETERMINISM TEST
 * ============================================================================
 *
 * Given:
 *
 *     identical source;
 *     identical lexer configuration;
 *     identical language version;
 *     identical grammar version;
 *
 * the parser must produce the same optional-marker structure and source spans.
 *
 * ============================================================================
 * ROUND-TRIP TEST
 * ============================================================================
 *
 * Required:
 *
 *     source
 *       ->
 *     parser
 *       ->
 *     TypeExpr::Optional
 *       ->
 *     formatter
 *       ->
 *     canonical source
 *
 * For canonical input:
 *
 *     T?
 *
 * the formatter must preserve:
 *
 *     T?
 *
 * modulo explicitly specified formatting rules.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN:
 *
 *     MAX_OPTIONAL_DEPTH
 *     MAX_OPTIONAL_NESTING
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Also forbidden:
 *
 *     vendor-specific optional syntax;
 *     physical device IDs;
 *     target-specific representations;
 *     fixed resource capacities;
 *     fixed quantum capacities.
 *
 * ALLOWED:
 *
 *     QUESTION token;
 *     parser repetition supplied by the enclosing type grammar;
 *     source-level type syntax;
 *     compatibility metadata.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * Generated/compiler integration must remain:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * No target-specific parser behavior may be introduced.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * `grammar/types/option.g4` is DONE when:
 *
 * [ ] `Option` is the grammar name.
 *
 * [ ] `tokenVocab = ZamaniLexer`.
 *
 * [ ] `optionalMarker` is the sole canonical optional-marker rule.
 *
 * [ ] `optionalTypePostfix` is only an alias.
 *
 * [ ] No complete `optionalType` rule exists here.
 *
 * [ ] No `typeExpression` rule exists here.
 *
 * [ ] No generic grammar exists here.
 *
 * [ ] No value-level option grammar exists here.
 *
 * [ ] No lexer rule exists here.
 *
 * [ ] `QUESTION` is the canonical token.
 *
 * [ ] `QUESTION_MARK` is not consumed here.
 *
 * [ ] `TypeExpr::Optional` remains the AST representation.
 *
 * [ ] Semantic optionality remains downstream.
 *
 * [ ] No effect is created by the type syntax.
 *
 * [ ] No capability is created by the type syntax.
 *
 * [ ] No resource is allocated by the type syntax.
 *
 * [ ] No policy is evaluated by the type syntax.
 *
 * [ ] No provenance record is fabricated by the grammar.
 *
 * [ ] No IR dependency exists.
 *
 * [ ] `quantum::ir` remains the canonical quantum boundary.
 *
 * [ ] No hardware assumptions exist.
 *
 * [ ] No machine-capacity constants exist.
 *
 * [ ] No artificial nesting limit exists.
 *
 * [ ] Parser behavior is deterministic.
 *
 * [ ] Source spans can be preserved.
 *
 * [ ] Existing optional syntax remains compatible.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Round-trip tests exist.
 *
 * [ ] Rust 1.97/1.97.1 integration remains safe.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 *     ONE OPTIONAL SYNTAX OWNER
 *             |
 *             v
 *     QUESTION -> optionalMarker
 *             |
 *             v
 *     Types.typePostfix
 *             |
 *             v
 *     TypeExpr::Optional
 *             |
 *             v
 *     semantic type
 *             |
 *       +-----+------+
 *       |            |
 *       v            v
 *   classical    quantum/domain
 *                    |
 *                    v
 *                quantum::ir
 *             |
 *             v
 *     target-independent
 *     lowering and realization
 *
 * The optional-type grammar therefore remains independent of machine size,
 * hardware inventory, physical topology, vendor implementation, and runtime
 * resources.
 *
 * ============================================================================
 */