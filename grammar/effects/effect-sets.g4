/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/effect-sets.g4
 *
 * Role:
 *     Canonical modular parser grammar for effect sets and effect references.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime calls, or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SYNTAX of effect collections.
 *
 * It defines:
 *
 *     - effect references;
 *     - qualified effect references;
 *     - effect-reference lists;
 *     - effect sets;
 *     - optional effect sets;
 *     - effect-set composition syntax;
 *     - empty effect sets;
 *     - trailing-comma support.
 *
 * This file deliberately does NOT define:
 *
 *     - effect declarations;
 *     - effect operations;
 *     - effect handlers;
 *     - effect execution;
 *     - effect dispatch;
 *     - capability semantics;
 *     - resource semantics;
 *     - hardware semantics;
 *     - quantum semantics;
 *     - QEC;
 *     - ZQN;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - runtime behavior.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         Zamani lexer
 *                              |
 *                              v
 *                    modular parser grammars
 *                              |
 *                              v
 *                    effect-sets.g4
 *                              |
 *                              v
 *                         frontend AST
 *                              |
 *                +-------------+-------------+
 *                |             |             |
 *                v             v             v
 *          name resolution  type/effect   capability
 *                           analysis       analysis
 *                |             |             |
 *                +-------------+-------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *                +-------------+-------------+
 *                |             |             |
 *                v             v             v
 *          classical IR   quantum::ir   resource metadata
 *                              |
 *                              v
 *                 optimization / routing /
 *                 scheduling / resilience /
 *                 ZQN / target lowering
 *                              |
 *                              v
 *                           runtime
 *
 * The grammar is therefore intentionally syntax-only.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Effect sets describe PORTABLE COMPUTATIONAL INTENT.
 *
 * They must not describe the physical machine that eventually realizes that
 * intent.
 *
 * Therefore this grammar imposes no source-language limits on:
 *
 *     - number of effects;
 *     - number of effect references;
 *     - number of effect-set entries;
 *     - number of nested scopes;
 *     - number of effect clauses;
 *     - number of modules;
 *     - number of handlers;
 *     - number of domains;
 *     - effect-name length;
 *     - qualified-name depth.
 *
 * No:
 *
 *     MAX_EFFECTS
 *     MAX_EFFECT_SET_SIZE
 *     MAX_EFFECT_REFERENCES
 *     MAX_EFFECT_DEPTH
 *
 * or equivalent language-level constants are permitted here.
 *
 * Any implementation/resource limit belongs to an explicit parser/compiler
 * resource policy and MUST NOT alter the language's semantic meaning.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Effect identities are names.
 *
 * This means the grammar does not enumerate effect kinds.
 *
 * Valid examples include:
 *
 *     IO
 *     Storage
 *     Network
 *     quantum::Measurement
 *     quantum::Reset
 *     qec::Correction
 *     zqn::NoiseObservation
 *     distributed::Consensus
 *     accelerator::Tensor
 *     future::domain::operation
 *     vendor::custom::effect
 *
 * New effect domains therefore do not require grammar modification.
 *
 * The semantic resolver determines what a referenced effect means.
 *
 * ============================================================================
 * EFFECT VS CAPABILITY VS RESOURCE
 * ============================================================================
 *
 * An effect reference answers:
 *
 *     "What computational interaction is part of this computation?"
 *
 * It does NOT answer:
 *
 *     "Which machine provides it?"
 *     "How many resources are allocated?"
 *     "Which QPU is selected?"
 *     "Which CPU is selected?"
 *     "Which topology is used?"
 *     "Which backend is used?"
 *
 * Those concerns belong to:
 *
 *     capabilities
 *     resources
 *     constraints
 *     preferences
 *     target selection
 *     scheduling
 *     routing
 *     runtime
 *
 * Consequently:
 *
 *     effects { quantum::Measurement }
 *
 * MUST NOT semantically imply:
 *
 *     - a fixed QPU;
 *     - a fixed number of qubits;
 *     - a fixed topology;
 *     - a fixed gate set;
 *     - a fixed calibration;
 *     - a fixed simulator;
 *     - a fixed backend.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum effects are represented only as names:
 *
 *     quantum::Measurement
 *     quantum::Reset
 *     quantum::DynamicControl
 *     quantum::Readout
 *
 * This file MUST NOT introduce:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     noise models
 *     QEC codes
 *     ZQN faults
 *
 * Quantum semantics remain downstream and the canonical quantum semantic
 * boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * SET SEMANTICS
 * ============================================================================
 *
 * The grammar represents the source spelling of a collection.
 *
 * Whether the semantic representation:
 *
 *     - removes duplicates;
 *     - preserves source order;
 *     - canonicalizes qualified names;
 *     - interns names;
 *     - attaches source spans;
 *     - resolves aliases;
 *
 * belongs to frontend/semantic analysis.
 *
 * Therefore the grammar does NOT reject duplicate references merely because
 * they may later normalize to the same semantic effect.
 *
 * For example:
 *
 *     effects { IO, IO }
 *
 * is syntactically valid.
 *
 * Semantic analysis may normalize it according to the language specification.
 *
 * This separation prevents syntax from accidentally defining semantic set
 * algebra.
 *
 * ============================================================================
 * EMPTY SET
 * ============================================================================
 *
 * The empty effect set is syntactically valid:
 *
 *     effects {}
 *
 * This represents an explicitly empty source-level effect collection.
 *
 * It MUST NOT automatically be interpreted as "pure" unless semantic analysis
 * defines that relationship.
 *
 * In particular:
 *
 *     empty effect set
 *
 * and:
 *
 *     pure
 *
 * are not made synonymous by this grammar.
 *
 * ============================================================================
 * TRAILING COMMAS
 * ============================================================================
 *
 * A trailing comma is accepted:
 *
 *     effects {
 *         IO,
 *         Network,
 *     }
 *
 * This makes generated source and large source changes easier to maintain.
 *
 * It does not impose any resource limit.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     lexer
 *         |
 *         +--> identifier tokens
 *         +--> qualified-name punctuation
 *         +--> braces
 *         +--> commas
 *
 *     Core parser grammar
 *         |
 *         +--> qualifiedName
 *
 * DOWNSTREAM:
 *
 *     effects.g4
 *         |
 *         +--> effectClause
 *         +--> effectRequirement
 *         +--> function/declaration effect annotations
 *
 *     semantic analysis
 *         |
 *         +--> effect-name resolution
 *         +--> duplicate normalization
 *         +--> effect compatibility
 *         +--> effect inference
 *
 *     capability analysis
 *         |
 *         +--> determine whether required effects can be realized
 *
 *     resource analysis
 *         |
 *         +--> determine resource consequences
 *
 *     canonical IR
 *         |
 *         +--> attach effect metadata
 *
 *     quantum::ir
 *         |
 *         +--> only when resolved semantics actually require quantum
 *             operations.
 *
 *     runtime
 *         |
 *         +--> effect dispatch only after semantic lowering.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * This file is the ONLY owner of:
 *
 *     effectReference
 *     effectReferenceList
 *     optionalEffectReferenceList
 *     effectSet
 *     optionalEffectSet
 *
 * Other grammar files MUST reference these rules rather than redefine them.
 *
 * In particular:
 *
 *     grammar/effects/effects.g4
 *
 * MUST import this grammar and MUST NOT define duplicate versions of the
 * rules above.
 *
 * The legacy:
 *
 *     grammar/antlr/Effects.g4
 *
 * MUST cease to be an authoritative competing owner of these rules when the
 * modular grammar becomes canonical.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no actions;
 *     - contains no semantic predicates;
 *     - performs no I/O;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no runtime dispatch;
 *     - performs no random operations.
 *
 * Given the same deterministic token stream, it produces the same parse tree.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Repetition is represented through ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * rather than fixed alternatives.
 *
 * No source-level resource capacity is embedded in the grammar.
 *
 * Practical parser limits such as:
 *
 *     stack depth
 *     allocation budget
 *     token-stream size
 *     recursion budget
 *     execution time
 *
 * are implementation/resource-policy concerns.
 *
 * They MUST NOT be encoded as language constants in this file.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Adding new effect names does not require a grammar version change.
 *
 * Grammar changes are required only when the syntax of effect sets changes.
 *
 * Semantic additions such as:
 *
 *     new effect domains;
 *     new hardware capabilities;
 *     new quantum effects;
 *     new ZQN effects;
 *     new QEC effects;
 *
 * must not require editing this grammar merely to add their names.
 *
 * ============================================================================
 */

parser grammar EffectSets;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. EFFECT REFERENCE
 * ============================================================================
 *
 * An effect reference is a canonical qualified name.
 *
 * Examples:
 *
 *     IO
 *     Storage
 *     Network
 *     quantum::Measurement
 *     quantum::Reset
 *     distributed::Consensus
 *     future::domain::effect
 *
 * The meaning of the name is deliberately outside the grammar.
 *
 * `qualifiedName` is imported from the canonical core grammar so that this
 * file does not create a second naming system.
 */

effectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 2. EFFECT REFERENCE LIST
 * ============================================================================
 *
 * A non-empty comma-separated list of effect references.
 *
 * Examples:
 *
 *     IO
 *
 *     IO, Network
 *
 *     IO, Network, quantum::Measurement
 *
 *     IO, Network,
 *
 * A trailing comma is accepted intentionally.
 *
 * No fixed list length is encoded.
 */

effectReferenceList
    : effectReference
      (
          COMMA effectReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 3. OPTIONAL EFFECT REFERENCE LIST
 * ============================================================================
 *
 * This rule exists to make integration sites explicit and avoid repeated
 * optional-list patterns across downstream grammar files.
 *
 * It does not introduce new semantics.
 */

optionalEffectReferenceList
    : effectReferenceList?
    ;


/*
 * ============================================================================
 * 4. EFFECT SET
 * ============================================================================
 *
 * Canonical source syntax:
 *
 *     {}
 *
 *     { IO }
 *
 *     { IO, Network }
 *
 *     {
 *         IO,
 *         Network,
 *         quantum::Measurement,
 *     }
 *
 * The empty set is valid.
 *
 * Duplicate effect references are syntactically valid because duplicate
 * elimination is a semantic normalization concern.
 */

effectSet
    : LBRACE
      optionalEffectReferenceList
      RBRACE
    ;


/*
 * ============================================================================
 * 5. OPTIONAL EFFECT SET
 * ============================================================================
 *
 * Useful for declarations that permit an effect set to be omitted entirely.
 *
 * Examples:
 *
 *     no effect clause
 *
 * versus:
 *
 *     {}
 *
 * The semantic distinction, if any, belongs downstream.
 */

optionalEffectSet
    : effectSet?
    ;


/*
 * ============================================================================
 * 6. EFFECT SET BODY
 * ============================================================================
 *
 * Named integration rule for grammar components that need to consume only the
 * interior of an effect set.
 *
 * Example:
 *
 *     {
 *         IO,
 *         Network,
 *     }
 *
 * The braces remain owned by `effectSet`.
 */

effectSetBody
    : optionalEffectReferenceList
    ;


/*
 * ============================================================================
 * 7. EFFECT SET COMPOSITION
 * ============================================================================
 *
 * This rule provides a syntactic composition surface for grammar clients that
 * need to represent multiple source effect-set expressions.
 *
 * It intentionally does NOT define semantic union/intersection/subtraction.
 *
 * Semantic set algebra belongs to effect analysis, not parsing.
 *
 * The grammar therefore does not invent operators such as:
 *
 *     +
 *     -
 *     |
 *     &
 *
 * for effect sets.
 *
 * If Zamani later standardizes effect-set algebra, that syntax should be
 * introduced deliberately in a separate grammar change with an explicit
 * semantic contract.
 *
 * For now, composition is structural:
 *
 *     effectSetComposition
 *         : effectSet+
 *         ;
 */

effectSetComposition
    : effectSet+
    ;


/*
 * ============================================================================
 * 8. EFFECT REFERENCE
 * ============================================================================
 *
 * A single-reference convenience rule for downstream grammar composition.
 *
 * This is intentionally an alias and carries no new semantics.
 *
 * It exists only where a grammar consumer needs a clearly named
 * "single effect" slot.
 */

singleEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * 9. EFFECT SET ENTRY
 * ============================================================================
 *
 * Explicitly names one member of an effect set.
 *
 * This keeps downstream grammar rules from reaching into the list structure
 * when they need to describe one entry.
 */

effectSetEntry
    : effectReference
    ;


/*
 * ============================================================================
 * 10. EFFECT SET ENTRIES
 * ============================================================================
 *
 * Non-empty entry collection.
 *
 * This rule deliberately mirrors the source list without changing its
 * semantics.
 */

effectSetEntries
    : effectSetEntry
      (
          COMMA effectSetEntry
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. EFFECT SET WITH ENTRIES
 * ============================================================================
 *
 * Convenience composition rule for consumers that require a non-empty effect
 * set.
 *
 * This is distinct from `effectSet`, because an empty set is valid for the
 * canonical effect-set syntax.
 */

nonEmptyEffectSet
    : LBRACE
      effectSetEntries
      RBRACE
    ;


/*
 * ============================================================================
 * 12. EFFECT SET MEMBER
 * ============================================================================
 *
 * Semantic analysis may use this parse-tree boundary when constructing the
 * frontend AST.
 *
 * It remains a syntactic alias.
 */

effectSetMember
    : effectReference
    ;