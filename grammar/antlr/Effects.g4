/*
 * ============================================================================
 * Zamani Programming Language
 * Canonical Algebraic Effects Parser Grammar
 * ============================================================================
 *
 * File:
 *     grammar/antlr/Effects.g4
 *
 * Role:
 *     Reusable parser grammar for the Zamani algebraic-effects subsystem.
 *
 * Language:
 *     Zamani
 *
 * Compiler:
 *     ZUTC
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     The Zamani compiler MUST be implemented in safe Rust.
 *     This grammar contains no Rust code and introduces no unsafe requirement.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       ZamaniLexer.g4
 *                              |
 *                              v
 *                       ZamaniParser.g4
 *                              |
 *                 +------------+-------------+
 *                 |                          |
 *                 v                          v
 *             Core syntax               Effects.g4
 *                 |                          |
 *                 +------------+-------------+
 *                              |
 *                              v
 *                         Frontend AST
 *                              |
 *                              v
 *                   semantic/effect analysis
 *                              |
 *                              v
 *                     canonical semantic IR
 *                              |
 *                              +--> effect metadata
 *                              +--> resource metadata
 *                              +--> capability metadata
 *                              +--> quantum metadata
 *                              +--> temporal metadata
 *                              |
 *                              v
 *                           lowering
 *
 * Effects.g4 owns SYNTAX ONLY.
 *
 * It MUST NOT own:
 *
 *   - effect execution;
 *   - effect dispatch;
 *   - effect implementation;
 *   - effect handlers' runtime behavior;
 *   - capability checking;
 *   - resource allocation;
 *   - quantum hardware;
 *   - quantum gate inventories;
 *   - QPU topology;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - runtime handles;
 *   - filesystem access;
 *   - network access;
 *   - vendor SDKs;
 *   - backend selection.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Effect syntax describes portable computational intent.
 *
 * A source program MUST NOT need to change merely because the eventual
 * implementation runs on:
 *
 *   - a tiny machine;
 *   - a workstation;
 *   - a cluster;
 *   - a distributed system;
 *   - a simulator;
 *   - a classical accelerator;
 *   - a quantum processor;
 *   - a heterogeneous system;
 *   - a future computational substrate.
 *
 * No fixed:
 *
 *   MAX_EFFECTS
 *   MAX_EFFECT_OPERATIONS
 *   MAX_EFFECT_PARAMETERS
 *   MAX_EFFECT_HANDLERS
 *   MAX_EFFECT_DEPTH
 *
 * is encoded in this grammar.
 *
 * Practical compiler resource limits, if required, belong to explicit
 * compiler policy rather than the language grammar.
 *
 * ============================================================================
 *
 * CANONICAL AST CONTRACT
 * ============================================================================
 *
 * The canonical frontend effect declaration currently represents:
 *
 *     Effect
 *       |
 *       +-- name
 *       +-- generic parameters
 *       +-- parameters
 *       +-- optional return type
 *
 * Effect declaration syntax:
 *
 *     effect Name;
 *
 *     effect Name<T>;
 *
 *     effect Name<T>(value: T) -> Result;
 *
 *     effect Name {
 *         fn operation(value: T) -> Result;
 *     }
 *
 * The AST owns structural representation.
 * Semantic analysis owns meaning.
 *
 * ============================================================================
 *
 * EFFECT DECLARATION VS EFFECT USE
 * ============================================================================
 *
 * Declaration:
 *
 *     effect Read<T>(key: T) -> Value;
 *
 * Invocation:
 *
 *     perform Read(key);
 *
 * Handling:
 *
 *     handle computation {
 *         case Read(key) => ...
 *     }
 *
 * These are intentionally separate syntactic/semantic concepts.
 *
 * Effects.g4 MUST NOT collapse declaration and invocation into one construct.
 *
 * ============================================================================
 *
 * EXTENSIBILITY
 * ============================================================================
 *
 * Effect identities are names, not a closed enum.
 *
 * This permits:
 *
 *     effect IO;
 *     effect Storage;
 *     effect Network;
 *     effect QuantumMeasurement;
 *     effect Decoherence;
 *     effect DistributedConsensus;
 *     effect UserDefined<T>;
 *     effect FutureDomainSpecificEffect;
 *
 * without changing this grammar.
 *
 * The grammar therefore never contains a finite list of effect names.
 *
 * ============================================================================
 *
 * SEMANTIC NEUTRALITY
 * ============================================================================
 *
 * The following are deliberately NOT grammar-level effect properties:
 *
 *     pure
 *     impure
 *     resumable
 *     non-resumable
 *     asynchronous
 *     nondeterministic
 *     quantum
 *     hardware-backed
 *     distributed
 *     security-sensitive
 *     resource-consuming
 *
 * Such properties may be expressed through ordinary Zamani types,
 * attributes, declarations or semantic metadata, but their interpretation
 * belongs outside this grammar.
 *
 * ============================================================================
 *
 * ERROR / RECOVERY POLICY
 * ============================================================================
 *
 * This grammar does not silently reinterpret malformed effect syntax.
 *
 * In particular:
 *
 *     effect;
 *     effect ();
 *     effect Name(;
 *     effect Name<T;
 *     effect Name { fn ; }
 *
 * must remain parser errors.
 *
 * Parser recovery is owned by the ANTLR parser/runtime integration.
 *
 * ============================================================================
 */

parser grammar Effects;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * EFFECT DECLARATIONS
 * ========================================================================== */

/**
 * Complete effect declaration.
 *
 * Canonical forms:
 *
 *     effect Name;
 *     effect Name<T>;
 *     effect Name<T>(x: T) -> R;
 *     effect Name<T> {
 *         fn read(x: T) -> R;
 *     }
 *
 * A declaration without an operation body describes the effect identity and
 * optional signature.
 *
 * A body describes named operations belonging to the effect declaration.
 */
effectDeclaration
    : visibilityModifier?
      EFFECT
      identifier
      genericParameters?
      effectDeclarationSignature?
      effectBody?
      SEMI?
    ;


/**
 * Optional declaration-level signature.
 *
 * Examples:
 *
 *     effect Read<T>(key: T);
 *     effect Read<T>(key: T) -> Value;
 */
effectDeclarationSignature
    : LPAREN parameterList? RPAREN
      returnType?
    ;


/**
 * Effect operation container.
 *
 * The body is deliberately declarative.
 * Runtime behavior belongs to semantic analysis and lowering.
 */
effectBody
    : LBRACE effectOperation* RBRACE
    ;


/**
 * One named operation supplied by an effect declaration.
 *
 * Example:
 *
 *     fn read(key: Key) -> Value;
 *
 *     fn write(key: Key, value: Value) -> Unit;
 *
 * Operations are not assigned a finite vocabulary.
 */
effectOperation
    : attributes*
      visibilityModifier?
      ASYNC?
      FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      SEMI?
    ;


/* ============================================================================
 * EFFECT REFERENCES
 * ========================================================================== */

/**
 * A source-level reference to an effect.
 *
 * Effect identity is name/path based.
 *
 * Examples:
 *
 *     IO
 *     std::io::Read
 *     quantum::Measurement
 */
effectReference
    : qualifiedName
    ;


/**
 * A comma-separated effect reference list.
 *
 * Examples:
 *
 *     IO, Network
 *     quantum::Measurement, Storage
 */
effectReferenceList
    : effectReference
      (COMMA effectReference)*
      COMMA?
    ;


/* ============================================================================
 * EFFECT CLAUSES
 * ========================================================================== */

/**
 * Function-level effect requirement.
 *
 * Canonical form:
 *
 *     fn read() -> Value with effects { IO, Storage }
 */
effectClause
    : WITH
      EFFECTS
      LBRACE
      effectReferenceList?
      RBRACE
    ;


/**
 * A compact effect requirement.
 *
 * This rule is useful for grammar composition and tooling.
 *
 * Example:
 *
 *     with effects { IO, Network }
 */
effectRequirement
    : WITH
      EFFECTS
      LBRACE
      effectReferenceList?
      RBRACE
    ;


/* ============================================================================
 * EFFECT INVOCATION
 * ========================================================================== */

/**
 * Effect invocation.
 *
 * Canonical source form:
 *
 *     perform Read(key);
 *     perform QuantumMeasurement(q);
 *     perform Storage::read(key);
 *
 * The expression following `perform` is intentionally delegated to the
 * canonical Zamani expression grammar.
 *
 * The effect grammar therefore does not define a second expression language.
 */
performExpression
    : PERFORM
      expression
    ;


/**
 * Statement form of effect invocation.
 */
performStatement
    : performExpression
      SEMI?
    ;


/* ============================================================================
 * EFFECT HANDLING
 * ========================================================================== */

/**
 * Effect handler statement.
 *
 * Canonical structural form:
 *
 *     handle expression {
 *         ...
 *     }
 *
 * Example:
 *
 *     handle computation {
 *         ...
 *     }
 *
 * Handler semantics are resolved later.
 */
handleStatement
    : HANDLE
      expression
      effectHandlerBody
    ;


/**
 * Handler body.
 *
 * The body is composed of effect handlers.
 */
effectHandlerBody
    : LBRACE
      effectHandler*
      RBRACE
    ;


/**
 * One handler arm.
 *
 * Canonical conceptual form:
 *
 *     case EffectName(arguments) => { ... }
 *
 * The effect name remains an identifier/path rather than a closed keyword
 * vocabulary.
 */
effectHandler
    : CASE
      effectHandlerPattern
      FAT_ARROW
      (
          blockExpression
        | expression
      )
      COMMA?
    ;


/**
 * Handler operation pattern.
 *
 * Examples:
 *
 *     Read(key)
 *     Storage::Read(key)
 *     QuantumMeasurement(q)
 *
 * A bare effect identity is also valid:
 *
 *     case Read => ...
 */
effectHandlerPattern
    : qualifiedName
      (
          LPAREN argumentList? RPAREN
      )?
    ;


/* ============================================================================
 * EFFECT CONTINUATIONS / RESUMPTION
 * ========================================================================== */

/**
 * Explicit continuation/resumption syntax.
 *
 * `resume` is intentionally a parser-level operation rather than a semantic
 * guarantee that a particular runtime implementation supports resumability.
 *
 * Semantic analysis determines whether the enclosing effect handler permits
 * the requested continuation behavior.
 */
resumeExpression
    : RESUME
      (
          LPAREN argumentList? RPAREN
        | expression
      )?
    ;


/**
 * Explicit abort of the current effect handling computation.
 *
 * `abort` is likewise semantic rather than backend-specific.
 */
abortExpression
    : ABORT
      expression?
    ;


/* ============================================================================
 * EFFECT HANDLER CLAUSES
 * ========================================================================== */

/**
 * Optional explicit handler clause.
 *
 * This form supports declarations that describe how an effect is handled
 * without making the handler implementation part of the effect declaration.
 *
 * Example:
 *
 *     handle value {
 *         case Read(key) => ...
 *     }
 */
handlerClause
    : HANDLE
      expression
      effectHandlerBody
    ;


/* ============================================================================
 * EFFECT TYPE / CAPABILITY REFERENCES
 * ========================================================================== */

/**
 * Effect type reference.
 *
 * This is intentionally a named semantic reference rather than a grammar
 * catalogue.
 *
 * Examples:
 *
 *     IO
 *     IO<T>
 *     quantum::Measurement<Q>
 */
effectTypeReference
    : qualifiedName
      genericArguments?
    ;


/**
 * Effect type argument list.
 *
 * This rule deliberately delegates each type argument to the canonical
 * Zamani type grammar.
 */
genericArguments
    : LESS_THAN
      typeExpression
      (COMMA typeExpression)*
      COMMA?
      GREATER_THAN
    ;


/* ============================================================================
 * EFFECT ATTRIBUTE TARGETS
 * ========================================================================== */

/**
 * Effect declarations may carry ordinary Zamani attributes.
 *
 * Attribute meaning is owned by semantic analysis.
 *
 * This grammar never interprets:
 *
 *     @pure
 *     @resumable
 *     @async
 *     @quantum
 *     @capability(...)
 *     @resource(...)
 *
 * as built-in semantic behavior.
 */
effectAttribute
    : attribute
    ;


/* ============================================================================
 * EFFECT-SCOPED BLOCK
 * ========================================================================== */

/**
 * A reusable effect-scoped computation.
 *
 * This rule exists so effect-aware grammar extensions can refer to the
 * canonical block syntax without introducing another block language.
 */
effectComputation
    : blockExpression
    | expression
    ;


/* ============================================================================
 * EFFECT HANDLER BINDINGS
 * ========================================================================== */

/**
 * Optional binding form for handler operations.
 *
 * Example:
 *
 *     case Read(key) => ...
 *
 * Binding semantics belong to the frontend AST and semantic resolver.
 */
effectBinding
    : identifier
    ;


/**
 * Handler operation argument pattern.
 *
 * The argument itself is parsed using the canonical expression grammar.
 */
effectArgumentPattern
    : expression
    ;


/* ============================================================================
 * EFFECT SIGNATURE SUPPORT
 * ========================================================================== */

/**
 * Reusable operation signature without a body.
 *
 * This is useful for tools which need to inspect an effect declaration's
 * operation surface without interpreting implementation details.
 */
effectOperationSignature
    : FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      SEMI?
    ;


/* ============================================================================
 * EFFECT SET
 * ========================================================================== */

/**
 * Explicit effect set.
 *
 * Example:
 *
 *     effects { IO, Network, Storage }
 *
 * The set is syntactically ordered. Whether it is semantically treated as
 * a mathematical set, row, multiset, capability collection, or another
 * representation is decided downstream.
 */
effectSet
    : LBRACE
      effectReferenceList?
      RBRACE
    ;


/**
 * Named effect set expression.
 *
 * Example:
 *
 *     effects IO
 *
 * This rule is intentionally small because effect algebra belongs to the
 * semantic layer.
 */
effectSetReference
    : EFFECTS
      effectReference
    ;


/* ============================================================================
 * EFFECT COMPOSITION
 * ========================================================================== */

/**
 * Effect composition is represented syntactically as named references.
 *
 * The grammar does not prescribe:
 *
 *     union;
 *     subtraction;
 *     intersection;
 *     normalization;
 *     ordering;
 *     commutativity;
 *     idempotence.
 *
 * Those are semantic properties.
 */
effectComposition
    : effectReferenceList
    ;


/* ============================================================================
 * EFFECT DECLARATION IDENTIFIERS
 * ========================================================================== */

/**
 * Effect operation names deliberately use the canonical identifier rule.
 *
 * There is no finite list of legal operation names.
 */
effectName
    : identifier
    ;


/**
 * Qualified effect name.
 *
 * Delegates to the canonical qualified-name rule.
 */
qualifiedEffectName
    : qualifiedName
    ;


/* ============================================================================
 * VALIDATION-ORIENTED RULES
 * ========================================================================== */

/**
 * Effect declaration name.
 *
 * Kept as a named rule so frontend tooling can identify the exact syntactic
 * ownership of the declaration identifier.
 */
effectDeclarationName
    : identifier
    ;


/**
 * Effect operation name.
 */
effectOperationName
    : identifier
    ;


/* ============================================================================
 * GRAMMAR INTEGRATION CONTRACT
 * ========================================================================== *
 *
 * This grammar intentionally references canonical rules supplied by the
 * composed Zamani parser:
 *
 *     identifier
 *     qualifiedName
 *     genericParameters
 *     parameterList
 *     returnType
 *     whereClause
 *     expression
 *     argumentList
 *     blockExpression
 *     typeExpression
 *     attributes
 *     visibilityModifier
 *
 * Effects.g4 MUST therefore be composed into the root parser rather than
 * compiled as an independent complete Zamani language.
 *
 * The composition boundary is:
 *
 *     ZamaniLexer.g4
 *          |
 *          +--> Core.g4
 *          |
 *          +--> Effects.g4
 *          |
 *          +--> Quantum.g4
 *          |
 *          +--> future domain grammars
 *          |
 *          v
 *     ZamaniParser.g4
 *
 * There must be only one definition of each shared lexical or expression
 * construct.
 *
 * In particular, Effects.g4 MUST NOT define another:
 *
 *     identifier
 *     expression
 *     statement
 *     typeExpression
 *     parameterList
 *     argumentList
 *     blockExpression
 *     lexer
 *
 * ============================================================================
 */