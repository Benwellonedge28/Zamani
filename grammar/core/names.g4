/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/names.g4
 *
 * Grammar identity:
 *     Names
 *
 * Role:
 *     Canonical parser-level name syntax for the Zamani language.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, predicates, filesystem
 *     access, network access, runtime callbacks, or unsafe code.
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
 *       |  IDENTIFIER
 *       |  K_AS
 *       |  DOUBLE_COLON
 *       |
 *       v
 *     grammar/core/names.g4
 *       |
 *       +--> declarations
 *       +--> modules
 *       +--> types
 *       +--> functions
 *       +--> expressions
 *       +--> effects
 *       +--> memory
 *       +--> concurrency
 *       +--> classical
 *       +--> quantum
 *       +--> hybrid
 *       +--> HDL
 *       +--> hardware
 *       +--> distributed
 *       +--> AI
 *       +--> data
 *       +--> networking
 *       +--> security
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     structural / semantic analysis
 *       |
 *       v
 *     symbol / module / type resolution
 *       |
 *       v
 *     canonical semantic model / IR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines reusable STRUCTURAL NAME SYNTAX.
 *
 * It answers:
 *
 *     "Does this token sequence form a valid source-level name?"
 *
 * It does NOT answer:
 *
 *     "What does this name mean?"
 *
 * Semantic meaning belongs to later compiler phases.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - simple names;
 *   - identifier references;
 *   - qualified names;
 *   - name segments;
 *   - lists of names;
 *   - lists of qualified names;
 *   - name aliases;
 *   - reusable name-reference syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical identifier spelling;
 *   - Unicode identifier classification;
 *   - Unicode normalization;
 *   - keyword recognition;
 *   - comments;
 *   - whitespace;
 *   - literals;
 *   - filesystem paths;
 *   - URLs;
 *   - package resolution;
 *   - module resolution;
 *   - symbol resolution;
 *   - type resolution;
 *   - scope construction;
 *   - resource allocation;
 *   - hardware discovery;
 *   - target selection;
 *   - physical qubit selection;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - runtime execution.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexical authority is:
 *
 *     grammar/lexer/tokens.g4
 *
 * This grammar consumes its tokens through:
 *
 *     tokenVocab = ZamaniTokens
 *
 * In particular:
 *
 *     IDENTIFIER
 *     K_AS
 *     DOUBLE_COLON
 *     COMMA
 *
 * are lexer-owned concepts.
 *
 * This file MUST NOT redefine IDENTIFIER.
 *
 * It MUST NOT recreate lexical Unicode/ASCII rules.
 *
 * It MUST NOT impose identifier-length limits.
 *
 * It MUST NOT maintain a second keyword table.
 *
 * ============================================================================
 * NAME SEMANTICS
 * ============================================================================
 *
 * Names are syntactic values.
 *
 * Their interpretation is determined later by context and semantic analysis.
 *
 * A qualified name such as:
 *
 *     quantum::ir
 *
 * may denote a namespace, module, type, declaration, semantic subsystem,
 * domain object, or another language-defined entity depending on context.
 *
 * The parser MUST NOT decide which one.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Name syntax is independent of execution hardware.
 *
 * There are deliberately no grammar limits on:
 *
 *   - identifier length;
 *   - qualified-name depth;
 *   - namespace depth;
 *   - number of names;
 *   - number of aliases;
 *   - number of declarations;
 *   - number of modules;
 *   - number of resources;
 *   - number of machines;
 *   - number of qubits;
 *   - number of CPUs;
 *   - number of GPUs;
 *   - number of FPGAs;
 *   - number of nodes;
 *   - number of accelerators.
 *
 * Repetition is represented structurally with `*` or `+`.
 *
 * Any practical resource limitation belongs to implementation policy,
 * compiler configuration, operating-system resources, or deployment
 * resources—not to the language's name grammar.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT contain constructs such as:
 *
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_NAME_DEPTH
 *     MAX_NAMESPACE_DEPTH
 *     MAX_MODULES
 *     MAX_ALIASES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *
 * It also MUST NOT contain fixed domain-specific name categories such as:
 *
 *     QubitName
 *     PhysicalQubitName
 *     GPUName
 *     CPUName
 *     FPGAName
 *     QPUName
 *     AcceleratorName
 *
 * Domain-specific meaning is established after parsing.
 *
 * ============================================================================
 * QUALIFICATION
 * ============================================================================
 *
 * Zamani uses DOUBLE_COLON (`::`) for qualified names.
 *
 * Examples:
 *
 *     math::linear
 *     math::linear::matrix
 *     quantum::ir
 *     hardware::capability
 *     accelerator::tensor
 *
 * No finite qualification depth is imposed.
 *
 * ============================================================================
 * PATH SEPARATION
 * ============================================================================
 *
 * A qualified name is NOT a filesystem path.
 *
 * Examples that do not belong to this grammar:
 *
 *     ./src/module.zm
 *     ../module.zm
 *     /absolute/path
 *     https://example.org
 *
 * Path syntax belongs to:
 *
 *     grammar/core/paths.g4
 *
 * A path may contain names, but the path itself is not a name.
 *
 * ============================================================================
 * MEMBER ACCESS SEPARATION
 * ============================================================================
 *
 * This:
 *
 *     object.member
 *
 * is expression/member-access syntax.
 *
 * It is NOT a qualified name.
 *
 * Therefore DOT is deliberately not used by qualifiedName.
 *
 * Likewise:
 *
 *     object.member.method
 *
 * remains expression syntax.
 *
 * ============================================================================
 * TYPE / GENERIC SEPARATION
 * ============================================================================
 *
 * A name may participate in a type expression:
 *
 *     math::Vector
 *
 * but generic arguments belong to the type grammar:
 *
 *     math::Vector<T>
 *
 * This file therefore does not consume generic argument syntax.
 *
 * ============================================================================
 * ALIAS SEMANTICS
 * ============================================================================
 *
 * This file defines only the structural form:
 *
 *     name as alias
 *
 * The legality of an alias in a particular context is decided by the
 * consuming grammar and semantic analysis.
 *
 * The `as` spelling is represented by the canonical lexical token:
 *
 *     K_AS
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve enough source information to construct
 * canonical name nodes without resolving them.
 *
 * Recommended conceptual forms:
 *
 *     IdentifierName
 *         spelling
 *         source_span
 *
 *     QualifiedName
 *         segments[]
 *         source_span
 *
 *     NameAlias
 *         target
 *         alias
 *         source_span
 *
 * The exact AST type names belong to src/frontend/ast/.
 *
 * This grammar MUST NOT depend on the AST implementation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing:
 *
 *     identifier
 *         -> unresolved source name
 *
 *     qualifiedName
 *         -> ordered unresolved name segments
 *
 *     nameAlias
 *         -> alias declaration/reference syntax
 *
 * Semantic analysis is responsible for:
 *
 *     scope lookup
 *     namespace lookup
 *     module resolution
 *     package resolution
 *     symbol resolution
 *     type resolution
 *     capability resolution
 *     resource interpretation
 *     domain interpretation
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Names are not themselves a domain IR.
 *
 * They may become:
 *
 *     symbol references
 *     type references
 *     module references
 *     operation names
 *     capability names
 *     resource requirement names
 *     quantum operation names
 *     HDL identifiers
 *     hardware capability identifiers
 *
 * according to semantic context.
 *
 * Quantum names MUST eventually lower through the established semantic
 * quantum pipeline and MUST NOT create a second competing quantum IR.
 *
 * ============================================================================
 * QUANTUM INDEPENDENCE
 * ============================================================================
 *
 * This grammar intentionally does not distinguish:
 *
 *     qubit
 *     logical qubit
 *     physical qubit
 *     gate
 *     circuit
 *     QPU
 *
 * at the identifier syntax level.
 *
 * For example:
 *
 *     q
 *     logical_q
 *     physical_q
 *     custom_gate
 *     quantum::ir
 *
 * are ordinary names as far as this grammar is concerned.
 *
 * Their meaning belongs to quantum semantic analysis and quantum::ir.
 *
 * This permits future quantum architectures without changing name syntax.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * A name such as:
 *
 *     gpu
 *     qpu
 *     accelerator
 *     node
 *     memory
 *     device
 *
 * is not a hardware allocation merely because of its spelling.
 *
 * Physical mapping, placement, topology, capacity, calibration and target
 * selection remain downstream concerns.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * The same name grammar is reusable by:
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
 *     memory
 *     concurrency
 *     effects
 *     interoperability
 *     dialects
 *     macros
 *     metaprogramming
 *
 * Domain grammars may add contextual wrappers but MUST NOT create another
 * general-purpose identifier/qualified-name implementation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime callbacks;
 *     - no random behavior;
 *     - no target-specific branches.
 *
 * Therefore name parsing depends only on the input token stream.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing consumers that currently implement:
 *
 *     identifier
 *     (DOUBLE_COLON identifier)*
 *
 * should migrate to this canonical rule instead of defining another
 * equivalent qualified-name grammar.
 *
 * Domain-specific wrappers are permitted, for example:
 *
 *     moduleName
 *         : qualifiedName
 *         ;
 *
 *     typeName
 *         : qualifiedName
 *         ;
 *
 *     capabilityName
 *         : qualifiedName
 *         ;
 *
 * Such wrappers express context without duplicating name syntax.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated parser integration MUST:
 *
 *     - compile with Rust 1.97 / 1.97.1;
 *     - remain safe Rust;
 *     - require no `unsafe`;
 *     - preserve source spans;
 *     - preserve deterministic parsing;
 *     - avoid machine-size assumptions.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive cases MUST include:
 *
 *     value
 *     state
 *     quantum::ir
 *     math::linear::matrix
 *     hardware::capability
 *     accelerator::tensor
 *     a, b, c
 *     math::Vector, math::Matrix
 *     math::linear as linear
 *     quantum::operation as operation
 *
 * Negative cases MUST include:
 *
 *     ::name
 *     name::
 *     name::::other
 *     ::
 *     name as
 *     as name
 *     name as as
 *
 * Boundary cases MUST include:
 *
 *     one-character identifiers
 *     underscore identifiers where accepted by the lexer
 *     very long identifiers
 *     deeply qualified names
 *     large name lists
 *     large qualified-name lists
 *     repeated aliases
 *
 * Scalability tests MUST verify that the grammar itself does not impose
 * finite language limits.
 *
 * Compatibility tests MUST verify that all domain consumers use the same
 * canonical qualified-name structure.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] canonical lexer vocabulary is used;
 *     [x] IDENTIFIER is not redefined;
 *     [x] K_AS is used for `as`;
 *     [x] DOUBLE_COLON is used for `::`;
 *     [x] simple names are defined;
 *     [x] qualified names are defined;
 *     [x] name lists are defined;
 *     [x] qualified-name lists are defined;
 *     [x] aliases are defined;
 *     [x] name-reference composition is defined;
 *     [x] no physical-resource limits exist;
 *     [x] no domain-specific identifier grammar exists;
 *     [x] semantic resolution remains downstream;
 *     [x] paths remain separate;
 *     [x] member access remains separate;
 *     [x] generic type syntax remains separate;
 *     [x] quantum::ir remains downstream;
 *     [x] Rust integration requires no unsafe;
 *     [x] source-span preservation is specified;
 *     [x] positive/negative/boundary/scalability contracts are defined.
 *
 * ============================================================================
 */

parser grammar Names;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * CANONICAL SIMPLE IDENTIFIER
 * ============================================================================
 *
 * The lexer owns the lexical definition of IDENTIFIER.
 *
 * This rule owns its parser-level use.
 *
 * Examples:
 *
 *     value
 *     state
 *     matrix
 *     q
 *     algorithm
 */
identifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * SIMPLE NAME
 * ============================================================================
 *
 * A simpleName is an identifier used as a complete source-level name.
 *
 * This wrapper is useful when consumers need to distinguish a complete
 * simple-name occurrence from the lexical identifier token without creating
 * a second lexical concept.
 */
simpleName
    : identifier
    ;


/*
 * ============================================================================
 * QUALIFIED NAME SEGMENT
 * ============================================================================
 *
 * A segment is currently one canonical identifier.
 *
 * Keeping the segment as a named parser rule gives future language evolution
 * an explicit extension point without duplicating qualified-name syntax in
 * domain grammars.
 */
nameSegment
    : identifier
    ;


/*
 * ============================================================================
 * QUALIFIED NAME
 * ============================================================================
 *
 * One or more name segments separated by DOUBLE_COLON.
 *
 * Examples:
 *
 *     math
 *     math::linear
 *     math::linear::matrix
 *     quantum::ir
 *     hardware::capability
 *
 * There is deliberately no finite maximum number of segments.
 */
qualifiedName
    : nameSegment (DOUBLE_COLON nameSegment)*
    ;


/*
 * ============================================================================
 * NAME LIST
 * ============================================================================
 *
 * One or more simple identifiers separated by commas.
 *
 * Example:
 *
 *     a, b, c
 *
 * The grammar does not impose a maximum list size.
 */
nameList
    : identifier (COMMA identifier)*
    ;


/*
 * ============================================================================
 * OPTIONAL NAME LIST
 * ============================================================================
 *
 * Explicit nullable wrapper.
 *
 * Keeping nameList non-empty makes it reusable in contexts where at least
 * one name is mandatory.
 */
optionalNameList
    : nameList?
    ;


/*
 * ============================================================================
 * QUALIFIED NAME LIST
 * ============================================================================
 *
 * One or more qualified names separated by commas.
 *
 * Examples:
 *
 *     math::Vector, math::Matrix
 *
 *     classical::vector,
 *     quantum::state,
 *     hardware::capability
 */
qualifiedNameList
    : qualifiedName (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * OPTIONAL QUALIFIED NAME LIST
 * ============================================================================
 */
optionalQualifiedNameList
    : qualifiedNameList?
    ;


/*
 * ============================================================================
 * NAME ALIAS
 * ============================================================================
 *
 * Generic source-level alias form:
 *
 *     qualified::name as alias
 *
 *     name as alias
 *
 * The lexical token for `as` is K_AS.
 *
 * This rule defines syntax only. Whether aliases are permitted in a
 * particular declaration/import/use context is decided by the consumer.
 */
nameAlias
    : qualifiedName K_AS identifier
    ;


/*
 * ============================================================================
 * NAME REFERENCE
 * ============================================================================
 *
 * A name reference may be:
 *
 *     name
 *
 * or:
 *
 *     name as alias
 *
 * Alias is listed first intentionally because it is the more structured
 * alternative and avoids making consumers depend on alternative ordering
 * elsewhere.
 */
nameReference
    : nameAlias
    | qualifiedName
    ;


/*
 * ============================================================================
 * NAME REFERENCE LIST
 * ============================================================================
 *
 * Reusable comma-separated sequence of names and aliases.
 */
nameReferenceList
    : nameReference (COMMA nameReference)*
    ;


/*
 * ============================================================================
 * OPTIONAL NAME REFERENCE LIST
 * ============================================================================
 */
optionalNameReferenceList
    : nameReferenceList?
    ;