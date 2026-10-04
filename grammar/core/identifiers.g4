/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/identifiers.g4
 *
 * Grammar:
 *     CoreIdentifiers
 *
 * Role:
 *     Domain-neutral parser-level identifier consumption primitives.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     ANTLR4
 *
 * Safety:
 *     This grammar contains no embedded target-language actions.
 *     No unsafe Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       | IDENTIFIER
 *       v
 *     CoreIdentifiers
 *       |
 *       +------------------------------+
 *       |                              |
 *       v                              v
 *     names.g4                    consuming grammars
 *       |                              |
 *       v                              |
 *     qualified names                  |
 *                                      |
 *                         +------------+-------------+
 *                         |            |             |
 *                         v            v             v
 *                    declarations  expressions   patterns
 *                         |            |             |
 *                         +------------+-------------+
 *                                      |
 *                                      v
 *                              domain-neutral AST
 *                                      |
 *                                      v
 *                              semantic analysis
 *                                      |
 *                    +-----------------+------------------+
 *                    |                 |                  |
 *                    v                 v                  v
 *               classical          quantum::ir       other domains
 *
 * This file is intentionally below lexical identifier recognition and below
 * canonical name/qualified-name composition.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides reusable parser-level bridges for occurrences of the
 * canonical IDENTIFIER token.
 *
 * It answers:
 *
 *     "Where does a lexical identifier occur in core syntax?"
 *
 * It does NOT answer:
 *
 *     "What does this identifier mean?"
 *
 * Semantic interpretation belongs to later phases.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - parser-level identifier occurrence;
 *   - generic identifier binding occurrence;
 *   - generic identifier use occurrence;
 *   - generic identifier label occurrence;
 *   - generic identifier pattern occurrence;
 *   - reusable non-empty identifier lists;
 *   - reusable optional identifier lists;
 *   - reusable trailing-comma identifier lists;
 *   - generic identifier-pair syntax where a consumer requires two
 *     syntactically adjacent identifier occurrences.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical IDENTIFIER tokenization;
 *   - identifier-start characters;
 *   - identifier-continuation characters;
 *   - Unicode character classification;
 *   - Unicode normalization;
 *   - Unicode security policy;
 *   - keyword recognition;
 *   - reserved-word policy;
 *   - comments;
 *   - whitespace;
 *   - literals;
 *   - operators;
 *   - punctuation;
 *   - simple-name ownership;
 *   - qualified-name ownership;
 *   - namespace resolution;
 *   - module resolution;
 *   - symbol resolution;
 *   - scope construction;
 *   - type resolution;
 *   - expression semantics;
 *   - declaration semantics;
 *   - resource semantics;
 *   - capability semantics;
 *   - effect semantics;
 *   - contract semantics;
 *   - policy semantics;
 *   - provenance semantics;
 *   - quantum semantics;
 *   - HDL semantics;
 *   - hardware identity;
 *   - target selection;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime behavior.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *     grammar/lexer/tokens.g4
 *     grammar/lexer/identifiers.g4
 *     canonical token vocabulary exposed as ZamaniTokens
 *
 * EXPORTS:
 *     identifierReference
 *     identifierBinding
 *     identifierUse
 *     identifierLabel
 *     identifierPattern
 *     identifierReferenceList
 *     optionalIdentifierReferenceList
 *     identifierReferenceListTrailingComma
 *     identifierPair
 *
 * CONSUMED_BY:
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *     declaration grammars
 *     expression grammars
 *     statement grammars
 *     pattern grammars
 *     type grammars
 *     function grammars
 *     module grammars
 *     resource grammars
 *     capability grammars
 *     effect grammars
 *     validation grammars
 *     policy grammars
 *     classical grammars
 *     quantum grammars
 *     hybrid grammars
 *     HDL grammars
 *     hardware grammars
 *     distributed grammars
 *     AI grammars
 *     data grammars
 *     networking grammars
 *     security grammars
 *     interoperability grammars
 *     dialect grammars
 *     metaprogramming grammars
 *
 * AST_OWNER:
 *     src/ast/
 *     or the repository's canonical domain-neutral frontend AST owner.
 *
 * SEMANTIC_OWNER:
 *     canonical semantic/name-resolution subsystem.
 *
 * TYPE_OWNER:
 *     canonical type-analysis subsystem.
 *
 * EFFECT_OWNER:
 *     canonical effect-analysis subsystem.
 *
 * RESOURCE_OWNER:
 *     canonical resource-analysis subsystem.
 *
 * POLICY_OWNER:
 *     canonical policy-analysis subsystem.
 *
 * PROVENANCE_OWNER:
 *     canonical provenance subsystem.
 *
 * IR_OWNER:
 *     downstream semantic/domain IR owners.
 *
 * SPEC_OWNER:
 *     grammar/specification/
 *     grammar/spec/
 *
 * TEST_OWNER:
 *     grammar/tests/
 *     repository frontend/conformance tests.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * There are intentionally three separate layers:
 *
 *     grammar/lexer/identifiers.g4
 *         |
 *         +-- owns IDENTIFIER tokenization
 *         |
 *         v
 *     grammar/core/identifiers.g4
 *         |
 *         +-- owns identifier occurrence composition
 *         |
 *         v
 *     grammar/core/names.g4
 *         |
 *         +-- owns identifier/simple-name/qualified-name composition
 *
 * This file MUST NOT redefine:
 *
 *     IDENTIFIER
 *     IDENTIFIER_START
 *     IDENTIFIER_CONTINUE
 *     identifier
 *     qualifiedName
 *
 * This prevents competing lexical and parser authorities.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical token is:
 *
 *     IDENTIFIER
 *
 * It is owned by:
 *
 *     grammar/lexer/identifiers.g4
 *
 * This file consumes that token only.
 *
 * It MUST NOT:
 *
 *   - inspect source characters;
 *   - inspect Unicode code points;
 *   - perform normalization;
 *   - perform case folding;
 *   - maintain a keyword table;
 *   - decide whether a spelling is reserved;
 *   - manufacture identifier tokens;
 *   - reinterpret a keyword as an identifier.
 *
 * If a spelling is reserved, the canonical lexer determines its token.
 *
 * The parser therefore consumes the lexical result rather than duplicating
 * lexical policy.
 *
 * ============================================================================
 * KEYWORD CONTRACT
 * ============================================================================
 *
 * Keyword recognition belongs to the canonical lexical hierarchy.
 *
 * This file MUST NOT contain rules such as:
 *
 *     identifier
 *         : IDENTIFIER
 *         | 'fn'
 *         ;
 *
 * nor any equivalent spelling-based exclusion list.
 *
 * This is necessary because language keywords may evolve independently of
 * parser-level identifier composition.
 *
 * ============================================================================
 * UNICODE CONTRACT
 * ============================================================================
 *
 * Unicode identifier validity is determined by the lexical identifier owner.
 *
 * This file is Unicode-agnostic.
 *
 * It therefore works equally with:
 *
 *   - ASCII identifiers;
 *   - Unicode identifiers;
 *   - future approved Unicode identifier policy;
 *   - future lexical identifier extensions.
 *
 * No parser rule in this file assumes:
 *
 *   - byte width;
 *   - character width;
 *   - code-point count;
 *   - normalization form;
 *   - machine word size.
 *
 * ============================================================================
 * CASE CONTRACT
 * ============================================================================
 *
 * This file does not perform case folding.
 *
 * If the lexer emits two distinct identifier tokens for:
 *
 *     value
 *     Value
 *
 * this file consumes them according to the token stream.
 *
 * Whether those names are semantically distinct is a name-resolution concern.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Identifier syntax is independent of computational scale.
 *
 * This file imposes no language-level limits on:
 *
 *   - identifier length;
 *   - number of identifier occurrences;
 *   - number of declarations;
 *   - number of bindings;
 *   - number of uses;
 *   - number of modules;
 *   - number of namespaces;
 *   - number of domains;
 *   - number of resources;
 *   - number of capabilities;
 *   - number of quantum objects;
 *   - number of hardware objects;
 *   - number of machines;
 *   - number of nodes;
 *   - number of processes;
 *   - number of actors;
 *   - number of devices;
 *   - number of accelerators.
 *
 * Repeated syntax uses ANTLR repetition operators:
 *
 *     *
 *     +
 *
 * rather than artificial finite maxima.
 *
 * Practical implementation limits may exist because every finite compiler,
 * operating system and execution environment has finite resources.
 *
 * Those limits are implementation/deployment concerns and MUST NOT become
 * language semantics.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define or depend on language-wide capacity constants
 * such as:
 *
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_NAME_COUNT
 *     MAX_BINDINGS
 *     MAX_MODULES
 *     MAX_NAMESPACES
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
 * No identifier rule may indirectly encode such a ceiling.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * An identifier is a syntactic name occurrence.
 *
 * It may later denote:
 *
 *     variable
 *     constant
 *     parameter
 *     function
 *     type
 *     module
 *     namespace
 *     resource
 *     capability
 *     effect
 *     policy
 *     contract
 *     model
 *     dataset
 *     actor
 *     channel
 *     service
 *     quantum object
 *     quantum operation
 *     HDL object
 *     hardware abstraction
 *     network endpoint
 *     distributed object
 *     future-domain object
 *
 * This file MUST NOT create a separate grammar for any of those meanings.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum source syntax may consume:
 *
 *     identifierReference
 *     identifierBinding
 *     identifierUse
 *     identifierPattern
 *     identifierReferenceList
 *
 * for source-level names.
 *
 * This file MUST NOT create:
 *
 *     QubitId
 *     LogicalQubitId
 *     PhysicalQubitId
 *     GateKind
 *     CircuitId
 *     QPUId
 *
 * It MUST NOT encode physical qubit identity, topology, routing, calibration,
 * scheduling or error-correction state.
 *
 * Quantum meaning remains downstream:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
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
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * HDL and hardware grammars may consume these identifier occurrences for:
 *
 *     signal names
 *     module names
 *     component names
 *     resource names
 *     abstract interfaces
 *     capabilities
 *     timing-domain names
 *     logical hardware objects
 *
 * This file MUST NOT encode:
 *
 *     bus widths
 *     register widths
 *     physical device counts
 *     memory capacities
 *     clock counts
 *     topology dimensions
 *     synthesis targets
 *     physical placement.
 *
 * Those concerns belong downstream.
 *
 * ============================================================================
 * AI / KNOWLEDGE / REASONING BOUNDARY
 * ============================================================================
 *
 * Reasoning, knowledge, learning, adaptation, uncertainty, evidence,
 * explanation, decision and provenance constructs may all use ordinary
 * identifier occurrences.
 *
 * This file does not reserve application vocabulary.
 *
 * For example, names representing a model, concept, dataset, rule, agent or
 * evidence source remain ordinary identifier occurrences unless the canonical
 * lexer explicitly reserves their spelling.
 *
 * Semantic interpretation belongs to the appropriate AI/data/semantic owner.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Resource and capability grammars may consume identifiers for abstract
 * source-level names.
 *
 * For example:
 *
 *     required_memory
 *     required_qubits
 *     required_topology
 *     quantum_measurement
 *     tensor_compute
 *
 * This file does not determine whether any target provides those capabilities.
 *
 * The downstream pipeline is:
 *
 *     syntax
 *       ->
 *     AST
 *       ->
 *     semantic requirement
 *       ->
 *     capability/resource analysis
 *       ->
 *     negotiation
 *       ->
 *     execution planning
 *       ->
 *     target realization
 *
 * ============================================================================
 * EFFECT / CONTRACT / POLICY BOUNDARY
 * ============================================================================
 *
 * Identifier occurrences may appear in:
 *
 *     effects
 *     contracts
 *     policies
 *     requirements
 *     constraints
 *     provenance
 *
 * but this file does not assign those meanings.
 *
 * For example:
 *
 *     requires capability("tensor.compute")
 *
 * contains identifier-like semantic names, but the interpretation of the
 * capability belongs to the resource/capability semantic layer.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every identifier occurrence consumed by this grammar must remain traceable
 * to its source token/span.
 *
 * The parser MUST NOT:
 *
 *   - rewrite spelling;
 *   - normalize spelling;
 *   - resolve the symbol;
 *   - substitute a canonical symbol ID;
 *   - discard source location information.
 *
 * The AST/frontend layer must be able to retain:
 *
 *     original spelling
 *     source span
 *     occurrence kind
 *     syntactic context
 *
 * Name resolution may later attach semantic identity without destroying the
 * original source information.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar defines parser contexts, not AST classes.
 *
 * Recommended semantic distinction:
 *
 *     IdentifierOccurrence
 *         spelling
 *         span
 *         occurrence_kind
 *
 * where occurrence_kind may be established by the consuming syntax:
 *
 *     Binding
 *     Use
 *     Label
 *     Pattern
 *     Other
 *
 * The AST MUST remain domain-neutral.
 *
 * It MUST NOT encode:
 *
 *     CPU identity
 *     GPU identity
 *     QPU identity
 *     physical qubit identity
 *     FPGA placement
 *     routing
 *     scheduling
 *     calibration
 *     vendor instruction identity
 *
 * unless such information is explicitly part of a later semantic model.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser establishes only syntactic occurrence.
 *
 * Semantic analysis determines whether an occurrence denotes:
 *
 *     declaration
 *     binding
 *     reference
 *     type
 *     namespace
 *     module
 *     resource
 *     capability
 *     operation
 *     policy
 *     effect
 *     contract
 *     domain object
 *
 * Name resolution is therefore downstream of parsing.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Identifier syntax is independent of type syntax.
 *
 * This file does not parse:
 *
 *     generic arguments
 *     type bounds
 *     dependent constraints
 *     associated types
 *     type classes
 *     linearity
 *     affinity
 *
 * Type grammars may consume these identifier primitives and add type syntax.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Identifier parsing has no language effect.
 *
 * It produces no:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     quantum measurement
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * Effect analysis begins at the semantic construct that uses the identifier.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Identifier parsing requires no target resource.
 *
 * It MUST NOT inspect:
 *
 *     memory
 *     CPU
 *     GPU
 *     QPU
 *     FPGA
 *     accelerator
 *     network
 *     storage
 *     topology
 *
 * It therefore remains valid independently of target size.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Identifier parsing has no intrinsic precondition, postcondition, invariant,
 * assumption or guarantee beyond successful tokenization and parsing.
 *
 * Consuming constructs may attach:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * after this syntactic layer.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Identifier parsing is policy-neutral.
 *
 * Security, authorization, sandbox, deployment, resource and execution
 * policies are semantic concerns.
 *
 * A policy may restrict what an identifier denotes or what operation may be
 * performed with it, but the identifier grammar does not enforce that policy.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Identifier occurrences may contribute to:
 *
 *     symbol references
 *     type references
 *     operation names
 *     capability names
 *     resource expressions
 *     policy expressions
 *     provenance records
 *
 * after semantic analysis.
 *
 * The identifier grammar MUST NOT choose:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     backend IR
 *     vendor IR
 *
 * ============================================================================
 * PARSER DETERMINISM
 * ============================================================================
 *
 * These rules contain:
 *
 *   - no embedded actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no environment access;
 *   - no hardware discovery;
 *   - no runtime callbacks;
 *   - no randomness.
 *
 * Given the same token stream and grammar version, the result is deterministic.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * identifierReference
 *     One lexical identifier occurrence.
 *
 * identifierBinding
 *     Identifier occurrence used in a syntactic binding position.
 *
 * identifierUse
 *     Identifier occurrence used in a syntactic reference/use position.
 *
 * identifierLabel
 *     Identifier occurrence used where a consumer requires a label.
 *
 * identifierPattern
 *     Identifier occurrence available to pattern grammars.
 *
 * identifierReferenceList
 *     One or more identifier references separated by commas.
 *
 * optionalIdentifierReferenceList
 *     Zero or more identifier references represented as an optional list.
 *
 * identifierReferenceListTrailingComma
 *     One or more identifier references with an optional trailing comma.
 *
 * identifierPair
 *     Two adjacent identifier occurrences.
 *
 * ============================================================================
 * PRIVATE RULES
 * ============================================================================
 *
 * This file deliberately contains no private lexer fragments and no private
 * semantic rules.
 *
 * Every rule is a stable parser-level integration primitive.
 *
 * ============================================================================
 * RULE DESIGN
 * ============================================================================
 */

/*
 * --------------------------------------------------------------------------
 * 1. IDENTIFIER REFERENCE
 * --------------------------------------------------------------------------
 *
 * The fundamental parser bridge.
 *
 * IMPORTANT:
 *
 * This is intentionally NOT named `identifier`.
 *
 * `identifier` remains owned by grammar/core/names.g4 in the current
 * repository architecture.
 *
 * This avoids duplicate parser-rule ownership.
 */
identifierReference
    : IDENTIFIER
    ;


/*
 * --------------------------------------------------------------------------
 * 2. IDENTIFIER BINDING
 * --------------------------------------------------------------------------
 *
 * Syntactically identical to an identifier reference.
 *
 * The distinction exists so consuming grammars can communicate intent to
 * AST construction without performing semantic resolution here.
 *
 * Examples of possible consumers:
 *
 *     let name = value;
 *     fn parameter(...);
 *     pattern binding;
 *     declaration name;
 *
 * Whether a particular context permits a binding is decided by that context.
 */
identifierBinding
    : identifierReference
    ;


/*
 * --------------------------------------------------------------------------
 * 3. IDENTIFIER USE
 * --------------------------------------------------------------------------
 *
 * Generic syntactic use/reference occurrence.
 */
identifierUse
    : identifierReference
    ;


/*
 * --------------------------------------------------------------------------
 * 4. IDENTIFIER LABEL
 * --------------------------------------------------------------------------
 *
 * Generic label occurrence.
 *
 * This rule does not define label syntax such as:
 *
 *     label:
 *
 * because the surrounding control-flow grammar owns the delimiter and
 * complete label construct.
 */
identifierLabel
    : identifierReference
    ;


/*
 * --------------------------------------------------------------------------
 * 5. IDENTIFIER PATTERN
 * --------------------------------------------------------------------------
 *
 * Generic identifier occurrence for pattern grammars.
 *
 * Pattern semantics, binding behavior, destructuring and exhaustiveness
 * checking remain downstream.
 */
identifierPattern
    : identifierReference
    ;


/*
 * --------------------------------------------------------------------------
 * 6. NON-EMPTY IDENTIFIER LIST
 * --------------------------------------------------------------------------
 *
 * One or more identifiers separated by commas.
 *
 * There is no finite maximum.
 *
 * Examples:
 *
 *     a
 *     a, b
 *     a, b, c
 *
 * The rule uses iterative repetition rather than recursive list construction
 * to avoid unnecessary parser-stack growth for large lists.
 */
identifierReferenceList
    : identifierReference (COMMA identifierReference)*
    ;


/*
 * --------------------------------------------------------------------------
 * 7. OPTIONAL IDENTIFIER LIST
 * --------------------------------------------------------------------------
 *
 * Empty is represented by absence of the list.
 *
 * Consumers that require one or more identifiers must use
 * identifierReferenceList directly.
 */
optionalIdentifierReferenceList
    : identifierReferenceList?
    ;


/*
 * --------------------------------------------------------------------------
 * 8. TRAILING-COMMA IDENTIFIER LIST
 * --------------------------------------------------------------------------
 *
 * Reusable form for contexts that explicitly permit a trailing comma.
 *
 * Examples:
 *
 *     a,
 *     a, b,
 *     a, b,
 *
 * This rule does not imply that every identifier-list context permits a
 * trailing comma. The consuming grammar must select this rule deliberately.
 */
identifierReferenceListTrailingComma
    : identifierReference (COMMA identifierReference)* COMMA?
    ;


/*
 * --------------------------------------------------------------------------
 * 9. IDENTIFIER PAIR
 * --------------------------------------------------------------------------
 *
 * Two syntactically adjacent identifier occurrences.
 *
 * This is deliberately generic.
 *
 * It MUST NOT be interpreted here as:
 *
 *     inheritance
 *     type relation
 *     key/value relation
 *     source/target relation
 *     capability/resource relation
 *
 * The consuming grammar owns that meaning.
 */
identifierPair
    : identifierReference identifierReference
    ;


/*
 * ============================================================================
 * INTEGRATION RULES
 * ============================================================================
 *
 * 1. names.g4 remains the canonical owner of:
 *
 *        identifier
 *        simpleName
 *        nameSegment
 *        qualifiedName
 *        nameList
 *        qualifiedNameList
 *        nameAlias
 *
 * 2. qualified-names.g4 must not redefine IDENTIFIER lexical syntax.
 *
 * 3. Any grammar needing a single identifier occurrence may consume:
 *
 *        identifierReference
 *
 * 4. A grammar needing canonical `identifier` syntax may continue to consume:
 *
 *        identifier
 *
 *    from names.g4.
 *
 * 5. A grammar needing qualified names must consume:
 *
 *        qualifiedName
 *
 *    from names.g4 / the canonical qualified-name owner.
 *
 * 6. No domain grammar may create:
 *
 *        quantumIdentifier
 *        hardwareIdentifier
 *        gpuIdentifier
 *        qpuIdentifier
 *        tensorIdentifier
 *        actorIdentifier
 *
 *    as alternate general identifier systems.
 *
 * 7. Domain-specific wrappers are permitted only when they communicate
 *    syntactic context and do not redefine identifier structure.
 *
 * 8. Semantic interpretation remains downstream.
 *
 * ============================================================================
 * CONSUMER CONTRACT
 * ============================================================================
 *
 * A consumer grammar MUST NOT rely on this file to:
 *
 *   - reject a semantically invalid name;
 *   - resolve a name;
 *   - determine whether a name is declared;
 *   - determine whether a name is in scope;
 *   - determine whether a name denotes a type;
 *   - determine whether a name denotes a resource;
 *   - determine whether a capability exists;
 *   - determine whether a hardware target exists;
 *   - determine whether a quantum object exists.
 *
 * Those are semantic responsibilities.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every rule in this file consumes existing lexer tokens directly.
 *
 * Consequently, the parser context provides the source-token boundary needed
 * by the frontend to preserve source locations.
 *
 * The frontend MUST preserve:
 *
 *     token index
 *     source span
 *     source spelling
 *
 * or an equivalent lossless representation.
 *
 * This file does not synthesize text.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing an identifier MUST have no side effects.
 *
 * For example, a source name such as:
 *
 *     system::execute
 *
 * remains syntactic data.
 *
 * The parser MUST NOT:
 *
 *   - execute it;
 *   - resolve it against the filesystem;
 *   - contact a network;
 *   - inspect environment variables;
 *   - access credentials;
 *   - discover hardware;
 *   - invoke native code;
 *   - invoke foreign code;
 *   - modify compiler state outside parser construction.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar itself has no artificial identifier or list ceiling.
 *
 * Large programs may therefore contain:
 *
 *     many identifier occurrences;
 *     large declaration lists;
 *     large parameter lists;
 *     large pattern lists;
 *     large module sets;
 *     deeply nested semantic structures.
 *
 * The language does not promise that a finite implementation can successfully
 * parse literally unlimited source.
 *
 * Instead:
 *
 *     language capacity = not artificially bounded by this grammar
 *
 * while:
 *
 *     implementation capacity = bounded by available resources
 *
 * Resource exhaustion MUST be diagnosed by the implementation rather than
 * encoded as a language-level semantic maximum.
 *
 * ============================================================================
 * ANTLR / RUST INTEGRATION
 * ============================================================================
 *
 * ANTLR:
 *
 *     parser grammar CoreIdentifiers;
 *
 * The grammar consumes the canonical token vocabulary:
 *
 *     ZamaniTokens
 *
 * through:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * The public Zamani parser remains:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * with:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The build system MUST make the token vocabulary available when compiling
 * imported parser grammars.
 *
 * Rust:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *
 * Generated parser integration MUST use safe Rust.
 *
 * No embedded actions are permitted in this grammar.
 *
 * No unsafe Rust is required.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is additive at the parser-primitive level.
 *
 * It does not change the lexical token:
 *
 *     IDENTIFIER
 *
 * It does not change:
 *
 *     identifier
 *     qualifiedName
 *
 * in names.g4.
 *
 * Existing consumers may continue to use names.g4.
 *
 * New consumers should use the smallest appropriate rule from this file
 * instead of inventing another identifier bridge.
 *
 * If the public AST eventually distinguishes identifier occurrence kinds,
 * that is an AST/semantic evolution and does not require changing the lexical
 * identifier definition.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * This file does not create custom identifier diagnostics.
 *
 * Examples of errors that belong elsewhere:
 *
 *   invalid Unicode identifier character
 *       -> lexer
 *
 *   reserved keyword used where a name is required
 *       -> lexer/parser/contextual validation
 *
 *   unresolved identifier
 *       -> name resolution
 *
 *   duplicate declaration
 *       -> semantic validation
 *
 *   unknown type
 *       -> type analysis
 *
 *   unavailable capability
 *       -> capability analysis
 *
 *   unavailable target resource
 *       -> resource/target feasibility analysis
 *
 *   unsupported quantum operation
 *       -> quantum semantic analysis
 *
 * A target-capability failure MUST NOT be converted into an identifier
 * parsing error.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * ============================================================================
 *
 * The following token streams must be accepted where the surrounding grammar
 * permits an identifier:
 *
 *     value
 *     state
 *     q
 *     tensor
 *     accelerator
 *     future_domain
 *     model_α
 *     data123
 *
 * Unicode examples are subject to the lexical Unicode policy.
 *
 * Identifier lists:
 *
 *     a
 *     a, b
 *     a, b, c
 *
 * Trailing-comma lists:
 *
 *     a,
 *     a, b,
 *     a, b, c,
 *
 * Identifier pairs:
 *
 *     source target
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * These rules must reject token streams that do not contain IDENTIFIER in the
 * required position.
 *
 * Examples:
 *
 *     integer literal
 *     floating literal
 *     string literal
 *     punctuation
 *     operator
 *     EOF
 *
 * The exact diagnostic is owned by the parser/frontend diagnostic subsystem.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Required boundary cases:
 *
 *     one-character identifier
 *     underscore identifier
 *     identifier containing digits after the first character
 *     approved Unicode identifier
 *     very long identifier
 *     large identifier list
 *     trailing comma
 *     empty optional list
 *     adjacent identifier pair
 *
 * The tests must verify that no arbitrary grammar-level maximum exists.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Identifier primitives must be consumable from:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     resources
 *     effects
 *     validation
 *     policies
 *     interoperability
 *     dialects
 *     macros
 *     metaprogramming
 *
 * The same lexical identifier mechanism must be reused across all domains.
 *
 * ============================================================================
 * QUANTUM TESTS
 * ============================================================================
 *
 * Verify that identifiers used for:
 *
 *     q
 *     register
 *     state
 *     operation
 *     observable
 *     logical
 *     custom_operation
 *
 * remain ordinary identifier syntax.
 *
 * Verify that this file does not enumerate or restrict quantum operations.
 *
 * Verify that quantum semantic interpretation continues toward:
 *
 *     quantum::ir
 *
 * rather than a grammar-specific quantum identifier type.
 *
 * ============================================================================
 * HARDWARE TESTS
 * ============================================================================
 *
 * Verify that names such as:
 *
 *     cpu
 *     gpu
 *     fpga
 *     qpu
 *     accelerator
 *     device
 *     node
 *
 * are not assigned physical semantics by this grammar.
 *
 * Verify that no fixed resource count appears in the grammar.
 *
 * ============================================================================
 * AI / REASONING TESTS
 * ============================================================================
 *
 * Verify that ordinary identifiers can name:
 *
 *     rule
 *     premise
 *     conclusion
 *     model
 *     dataset
 *     evidence
 *     decision
 *     agent
 *     policy
 *
 * without creating a second identifier system.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Given the same token stream and grammar version:
 *
 *     parse result MUST be equivalent
 *     source spans MUST be equivalent
 *     occurrence structure MUST be equivalent
 *
 * The grammar must not depend on:
 *
 *     target hardware
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *     runtime state
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Tests should progressively exercise:
 *
 *     short identifiers;
 *     long identifiers;
 *     large identifier lists;
 *     large declaration sets;
 *     large source units.
 *
 * Tests MUST verify:
 *
 *     no artificial identifier-length grammar limit;
 *     no artificial list-length grammar limit;
 *     no machine-size assumptions;
 *     no domain-specific capacity assumptions.
 *
 * Resource exhaustion tests must distinguish:
 *
 *     implementation resource exhaustion
 *
 * from:
 *
 *     language semantic rejection.
 *
 * ============================================================================
 * INTEGRATION TESTS
 * ============================================================================
 *
 * Verify the following ownership chain:
 *
 *     grammar/lexer/identifiers.g4
 *          |
 *          v
 *     IDENTIFIER
 *          |
 *          v
 *     grammar/core/identifiers.g4
 *          |
 *          +--> identifierReference
 *          +--> identifierBinding
 *          +--> identifierUse
 *          +--> identifierLabel
 *          +--> identifierPattern
 *          +--> identifierReferenceList
 *          |
 *          v
 *     grammar/core/names.g4
 *          |
 *          v
 *     qualifiedName
 *          |
 *          v
 *     consuming grammar
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic analysis
 *
 * The integration suite MUST detect accidental duplicate ownership.
 *
 * ============================================================================
 * NO-CIRCULAR-DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This file must remain a leaf parser utility.
 *
 * It MUST NOT import:
 *
 *     names.g4
 *     expressions
 *     statements
 *     declarations
 *     quantum
 *     hardware
 *     AI
 *     runtime
 *     compiler
 *     IR
 *     HAL
 *
 * Higher-level grammars consume this file.
 *
 * In particular:
 *
 *     identifiers -> names
 *
 * is permitted conceptually through consumption/import composition,
 * while:
 *
 *     identifiers -> names -> identifiers
 *
 * is forbidden.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * Purpose:
 *     Provide reusable parser-level identifier occurrence primitives.
 *
 * Owns:
 *     Generic identifier occurrence and list syntax.
 *
 * Does Not Own:
 *     Lexical identifier recognition or canonical name/qualified-name syntax.
 *
 * Public Rules:
 *     identifierReference
 *     identifierBinding
 *     identifierUse
 *     identifierLabel
 *     identifierPattern
 *     identifierReferenceList
 *     optionalIdentifierReferenceList
 *     identifierReferenceListTrailingComma
 *     identifierPair
 *
 * Private Rules:
 *     None.
 *
 * Lexer Dependencies:
 *     IDENTIFIER
 *
 * Grammar Dependencies:
 *     None.
 *
 * AST Contract:
 *     Preserve identifier spelling, span and occurrence context.
 *
 * Semantic Contract:
 *     Meaning is assigned during name/semantic resolution.
 *
 * Type Contract:
 *     No type semantics.
 *
 * Effect Contract:
 *     No effects.
 *
 * Capability Contract:
 *     No capabilities.
 *
 * Resource Contract:
 *     No resource requirements.
 *
 * Contract Contract:
 *     No intrinsic contracts.
 *
 * Policy Contract:
 *     No intrinsic policies.
 *
 * Provenance Contract:
 *     Preserve source origin of every occurrence.
 *
 * IR Contract:
 *     No direct IR.
 *
 * Quantum Boundary:
 *     Names only; no quantum identity or physical realization.
 *
 * HDL Boundary:
 *     Names only; no physical hardware realization.
 *
 * Backend Boundary:
 *     None.
 *
 * Diagnostics:
 *     Syntax-level failures are reported by the parser/frontend.
 *
 * Positive Tests:
 *     Identifier occurrences and lists.
 *
 * Negative Tests:
 *     Non-identifier tokens in identifier positions.
 *
 * Boundary Tests:
 *     Unicode, long identifiers, large lists and trailing commas.
 *
 * Scalability Tests:
 *     No artificial identifier/list ceilings.
 *
 * Compatibility:
 *     Existing names.g4 ownership remains unchanged.
 *
 * Integration:
 *     Imported/consumed below the canonical names and domain grammars.
 *
 * Completion Criteria:
 *     One parser-level identifier primitive exists without duplicating
 *     lexical or qualified-name ownership.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] IDENTIFIER lexical ownership remains in grammar/lexer/identifiers.g4.
 *
 * [x] `identifier` ownership remains in grammar/core/names.g4.
 *
 * [x] `qualifiedName` ownership remains in grammar/core/names.g4.
 *
 * [x] No keyword table is duplicated.
 *
 * [x] No Unicode lexical rules are duplicated.
 *
 * [x] No domain-specific identifier system exists.
 *
 * [x] Identifier occurrence rules are reusable.
 *
 * [x] Identifier list rules are reusable.
 *
 * [x] Trailing-comma behavior is explicit.
 *
 * [x] Optional-list behavior is explicit.
 *
 * [x] No artificial finite identifier/list limits exist.
 *
 * [x] No hardware assumptions exist.
 *
 * [x] No quantum physical assumptions exist.
 *
 * [x] No AI/application vocabulary is reserved here.
 *
 * [x] No semantic resolution occurs in the parser.
 *
 * [x] No AST implementation is embedded in the grammar.
 *
 * [x] No IR is created here.
 *
 * [x] No filesystem/network/hardware/runtime side effects exist.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is documented.
 *
 * [x] Cross-domain reuse is defined.
 *
 * [x] Source-span preservation is defined.
 *
 * [x] Positive, negative, boundary, scalability and integration tests
 *     are specified.
 *
 * [x] The dependency direction remains acyclic.
 *
 * ============================================================================
 * FINAL PRINCIPLE
 * ============================================================================
 *
 * This file describes WHERE an identifier occurs.
 *
 * The lexer describes WHAT constitutes an identifier.
 *
 * names.g4 describes HOW identifiers compose into names.
 *
 * Semantic analysis describes WHAT those names mean.
 *
 * Domain IR describes HOW that meaning is realized.
 *
 * Hardware/runtime layers determine WHERE and HOW computation executes.
 *
 * Therefore:
 *
 *     identifier syntax
 *         !=
 *     semantic identity
 *         !=
 *     hardware identity
 *
 * Keeping those distinctions explicit is necessary for:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * without turning the foundational grammar into a catalogue of hardware,
 * quantum devices, AI applications or implementation-specific limits.
 *
 * ============================================================================
 */
 
parser grammar CoreIdentifiers;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * PUBLIC PARSER PRIMITIVES
 * ============================================================================
 */

/*
 * A single lexical identifier occurrence.
 *
 * `identifier` is intentionally not defined here because the canonical
 * simple-name owner is grammar/core/names.g4.
 */
identifierReference
    : IDENTIFIER
    ;


/*
 * Identifier in a syntactic binding position.
 */
identifierBinding
    : identifierReference
    ;


/*
 * Identifier in a syntactic use/reference position.
 */
identifierUse
    : identifierReference
    ;


/*
 * Identifier in a label position.
 */
identifierLabel
    : identifierReference
    ;


/*
 * Identifier available to pattern syntax.
 */
identifierPattern
    : identifierReference
    ;


/*
 * One or more identifier references.
 */
identifierReferenceList
    : identifierReference (COMMA identifierReference)*
    ;


/*
 * Zero or one identifier list.
 */
optionalIdentifierReferenceList
    : identifierReferenceList?
    ;


/*
 * One or more identifier references with an explicitly permitted trailing
 * comma.
 */
identifierReferenceListTrailingComma
    : identifierReference (COMMA identifierReference)* COMMA?
    ;


/*
 * Two adjacent identifier occurrences.
 *
 * Meaning is owned by the consuming grammar.
 */
identifierPair
    : identifierReference identifierReference
    ;