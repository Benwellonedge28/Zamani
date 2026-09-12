/**
 * Zamani Language Grammar
 * ========================
 *
 * File:
 *   grammar/core/versioning.g4
 *
 * Responsibility:
 *   Syntax for language-version declarations, version identifiers,
 *   compatibility requirements, version ranges, version predicates,
 *   version channels, and version metadata.
 *
 * Architectural boundary:
 *
 *   Source text
 *       |
 *       v
 *   Lexer
 *       |
 *       v
 *   Versioning grammar  <-- THIS FILE
 *       |
 *       v
 *   Parse tree
 *       |
 *       v
 *   AST / semantic analysis
 *       |
 *       v
 *   language-version model
 *       |
 *       +--> compiler compatibility
 *       +--> dialect compatibility
 *       +--> package compatibility
 *       +--> IR compatibility
 *       +--> runtime compatibility
 *       +--> target capability negotiation
 *
 * This grammar deliberately does NOT:
 *
 *   - determine whether a version is supported;
 *   - determine whether two versions are compatible;
 *   - select a compiler;
 *   - select hardware;
 *   - select a backend;
 *   - select a quantum processor;
 *   - select a CPU/GPU/FPGA/ASIC;
 *   - impose machine-size limits;
 *   - impose qubit limits;
 *   - impose topology limits;
 *   - encode runtime capabilities;
 *   - encode compiler implementation details;
 *   - encode Rust;
 *   - execute arbitrary code;
 *   - perform filesystem/network access.
 *
 * Those responsibilities belong to semantic analysis, compilation,
 * capability negotiation, target selection, and runtime layers.
 *
 * POCO-REAF:
 *
 *   Version declarations describe the language/contract being used.
 *   They must never turn temporary machine characteristics into
 *   permanent source-level semantics.
 *
 * Scalability:
 *
 *   No maximum version number, component count, range count, metadata
 *   count, or compatibility requirement count is encoded here.
 *
 * Determinism:
 *
 *   Grammar rules are declarative and contain no semantic actions.
 *
 * Safety:
 *
 *   No generated-code actions are present.
 *
 * ANTLR integration:
 *
 *   This is a parser grammar and therefore consumes lexer tokens.
 *   The lexer vocabulary is owned by grammar/lexer/.
 *
 * Expected lexical vocabulary:
 *
 *   LANGUAGE
 *   VERSION
 *   COMPATIBLE
 *   REQUIRES
 *   SUPPORTS
 *   TARGET
 *   DIALECT
 *   PACKAGE
 *   API
 *   ABI
 *   GRAMMAR
 *   LANGUAGE_VERSION
 *   COMPILER
 *   RUNTIME
 *   AND
 *   OR
 *   NOT
 *   EXACT
 *   MAJOR
 *   MINOR
 *   PATCH
 *   PRE_RELEASE
 *   BUILD
 *   STABLE
 *   BETA
 *   ALPHA
 *   RC
 *   DEV
 *
 * The concrete keyword vocabulary may be normalized by the lexer
 * without changing the semantic model represented by these rules.
 *
 * Existing compatibility:
 *
 *   The existing grammar/documentation accepts forms such as:
 *
 *       language Zamani "1.0";
 *
 * Versioning retains that conceptual capability while extending it
 * into a general, versionable language contract.
 */

parser grammar Versioning;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. LANGUAGE VERSION DECLARATION
 * ============================================================================
 *
 * Existing Zamani syntax:
 *
 *     language Zamani "1.0";
 *
 * The generalized production permits the language name and version
 * contract to remain independent of the implementation/compiler.
 */

languageVersionDeclaration
    : LANGUAGE languageIdentifier languageVersionSpec? languageVersionAttributes? SEMICOLON
    ;


/*
 * A language identifier is deliberately syntactic.
 *
 * It must not be restricted to "Zamani" here because:
 *
 *   - dialects may reuse the versioning machinery;
 *   - embedded languages may exist;
 *   - interoperability grammars may need version declarations;
 *   - future Zamani language families may be introduced.
 */

languageIdentifier
    : identifier
    ;


/*
 * ============================================================================
 * 2. VERSION SPECIFICATION
 * ============================================================================
 *
 * Supports:
 *
 *     "1"
 *     "1.0"
 *     "1.0.0"
 *     1
 *     1.0
 *     1.0.0
 *     1.0.0-alpha
 *     1.0.0+build
 *     1.0.0-alpha+build
 *
 * String-form versions remain useful for compatibility with the existing
 * grammar, while structured versions permit semantic tooling to understand
 * components without requiring source rewriting.
 */

languageVersionSpec
    : VERSION versionExpression
    | versionExpression
    ;


/*
 * ============================================================================
 * 3. VERSION EXPRESSION
 * ============================================================================
 *
 * A version expression may be:
 *
 *   - an exact version;
 *   - a version range;
 *   - a version constraint set;
 *   - a named version channel;
 *   - a version identifier.
 *
 * Semantic analysis determines whether a particular expression is valid
 * for a given language/compiler/dialect.
 */

versionExpression
    : exactVersion
    | versionRange
    | versionConstraintSet
    | versionChannel
    | versionReference
    ;


/*
 * ============================================================================
 * 4. EXACT VERSION
 * ============================================================================
 */

exactVersion
    : versionCore versionSuffix?
    | versionString
    ;


/*
 * ============================================================================
 * 5. VERSION CORE
 * ============================================================================
 *
 * There is intentionally no upper bound on any numeric component in this
 * grammar.
 *
 * The lexer owns integer syntax.
 *
 * Semantic validation decides whether a representation is acceptable for
 * a particular versioning scheme.
 *
 * Therefore this grammar does NOT contain rules such as:
 *
 *     INTEGER INTEGER
 *     0..255
 *     0..65535
 *     MAX_VERSION
 *
 * This is essential for long-term language evolution.
 */

versionCore
    : versionMajor
    | versionMajor DOT versionMinor
    | versionMajor DOT versionMinor DOT versionPatch
    | versionMajor DOT versionMinor DOT versionPatch DOT versionRevision
    ;


versionMajor
    : INTEGER
    ;


versionMinor
    : INTEGER
    ;


versionPatch
    : INTEGER
    ;


versionRevision
    : INTEGER
    ;


/*
 * ============================================================================
 * 6. VERSION SUFFIX
 * ============================================================================
 */

versionSuffix
    : preReleaseSuffix buildMetadataSuffix?
    | buildMetadataSuffix
    ;


preReleaseSuffix
    : MINUS versionIdentifierList
    ;


buildMetadataSuffix
    : PLUS versionIdentifierList
    ;


versionIdentifierList
    : versionIdentifier
    | versionIdentifierList DOT versionIdentifier
    ;


versionIdentifier
    : identifier
    | INTEGER
    | versionString
    ;


/*
 * ============================================================================
 * 7. STRING VERSION
 * ============================================================================
 *
 * Existing Zamani syntax uses STRING for language versions.
 *
 * This rule intentionally preserves that compatibility surface.
 *
 * Semantic analysis must parse and validate the string according to the
 * active versioning policy.
 */

versionString
    : STRING
    ;


/*
 * ============================================================================
 * 8. VERSION REFERENCES
 * ============================================================================
 *
 * Named references allow a source program to refer to a version contract
 * symbol rather than embedding a machine/compiler-specific value.
 *
 * Example:
 *
 *     language Zamani stable;
 *     requires language.version;
 *
 * The meaning of the reference is resolved semantically.
 */

versionReference
    : qualifiedVersionReference
    ;


qualifiedVersionReference
    : identifier
    | qualifiedIdentifier
    ;


/*
 * ============================================================================
 * 9. VERSION RANGES
 * ============================================================================
 *
 * Examples represented by this syntax include:
 *
 *     >= 1.0.0
 *     > 1.0.0
 *     <= 2.0.0
 *     < 2.0.0
 *     1.0.0 .. 2.0.0
 *     1.0.0 ..= 2.0.0
 *
 * The grammar represents the user's intent.
 *
 * It does not decide whether the range is satisfiable.
 */

versionRange
    : versionRangeEndpoint
    | versionRangeStart RANGE_EXCLUSIVE versionRangeEnd
    | versionRangeStart RANGE_INCLUSIVE versionRangeEnd
    ;


versionRangeStart
    : versionRangeEndpoint
    ;


versionRangeEnd
    : versionRangeEndpoint
    ;


versionRangeEndpoint
    : versionComparator versionEndpoint
    | versionEndpoint
    ;


versionComparator
    : LT
    | LE
    | GT
    | GE
    | EQ
    | NE
    ;


versionEndpoint
    : exactVersion
    | versionReference
    ;


/*
 * ============================================================================
 * 10. VERSION CONSTRAINT SETS
 * ============================================================================
 *
 * Multiple constraints can be expressed without imposing a fixed number
 * of constraints.
 *
 * Examples:
 *
 *     >= 1.0.0 and < 2.0.0
 *     >= 1.0.0 and < 2.0.0 and != 1.4.0
 *
 * The semantic layer determines whether the constraints are satisfiable.
 */

versionConstraintSet
    : versionConstraintExpression
    ;


versionConstraintExpression
    : versionConstraintTerm
    | versionConstraintExpression AND versionConstraintTerm
    | versionConstraintExpression OR versionConstraintTerm
    | NOT versionConstraintExpression
    | LEFT_PAREN versionConstraintExpression RIGHT_PAREN
    ;


versionConstraintTerm
    : versionRange
    | exactVersion
    | versionReference
    ;


/*
 * ============================================================================
 * 11. COMPATIBILITY DECLARATIONS
 * ============================================================================
 *
 * Versioning is broader than the language version itself.
 *
 * Zamani may need to express compatibility requirements for:
 *
 *   language
 *   compiler
 *   runtime
 *   grammar
 *   package
 *   API
 *   ABI
 *   dialect
 *
 * These are contracts, not machine descriptions.
 */

compatibilityDeclaration
    : COMPATIBLE compatibilitySubject compatibilityVersionSpec? SEMICOLON
    | REQUIRES compatibilitySubject compatibilityVersionSpec? SEMICOLON
    | SUPPORTS compatibilitySubject compatibilityVersionSpec? SEMICOLON
    ;


compatibilitySubject
    : languageCompatibilitySubject
    | compilerCompatibilitySubject
    | runtimeCompatibilitySubject
    | grammarCompatibilitySubject
    | packageCompatibilitySubject
    | apiCompatibilitySubject
    | abiCompatibilitySubject
    | dialectCompatibilitySubject
    | targetCompatibilitySubject
    ;


compatibilityVersionSpec
    : versionExpression
    ;


/*
 * ============================================================================
 * 12. LANGUAGE COMPATIBILITY
 * ============================================================================
 */

languageCompatibilitySubject
    : LANGUAGE
    ;


compilerCompatibilitySubject
    : COMPILER
    ;


runtimeCompatibilitySubject
    : RUNTIME
    ;


grammarCompatibilitySubject
    : GRAMMAR
    ;


packageCompatibilitySubject
    : PACKAGE
    ;


apiCompatibilitySubject
    : API
    ;


abiCompatibilitySubject
    : ABI
    ;


dialectCompatibilitySubject
    : DIALECT
    ;


/*
 * ============================================================================
 * 13. TARGET COMPATIBILITY
 * ============================================================================
 *
 * "target" here identifies a semantic target contract, not a concrete
 * machine or device.
 *
 * It MUST NOT be interpreted by the grammar as:
 *
 *   CPU count
 *   GPU count
 *   QPU count
 *   FPGA count
 *   memory size
 *   topology
 *   device identifier
 *
 * Those belong to target/capability/resource models.
 */

targetCompatibilitySubject
    : TARGET identifier
    ;


/*
 * ============================================================================
 * 14. VERSION CHANNELS
 * ============================================================================
 *
 * Channels are names, not fixed numeric versions.
 *
 * Examples:
 *
 *     stable
 *     beta
 *     alpha
 *     rc
 *     dev
 *
 * Additional channels may be introduced without changing this grammar
 * because identifier-based extensibility is intentional.
 */

versionChannel
    : STABLE
    | BETA
    | ALPHA
    | RC
    | DEV
    | identifier
    ;


/*
 * ============================================================================
 * 15. VERSION ATTRIBUTES
 * ============================================================================
 *
 * Metadata is deliberately open-ended.
 *
 * This prevents the grammar from becoming a bottleneck when future
 * language-versioning requirements appear.
 */

languageVersionAttributes
    : LEFT_BRACKET languageVersionAttributeList RIGHT_BRACKET
    ;


languageVersionAttributeList
    : languageVersionAttribute
    | languageVersionAttributeList COMMA languageVersionAttribute
    ;


languageVersionAttribute
    : identifier
    | identifier ASSIGN versionAttributeValue
    ;


versionAttributeValue
    : versionString
    | exactVersion
    | versionChannel
    | identifier
    | INTEGER
    | DECIMAL
    | booleanLiteral
    ;


/*
 * ============================================================================
 * 16. VERSION REQUIREMENT BLOCK
 * ============================================================================
 *
 * Allows multiple version contracts to be associated with a source unit
 * without encoding a fixed number of requirements.
 *
 * Example:
 *
 *     requires version {
 *         language >= 1.0.0;
 *         grammar >= 1.0.0;
 *         runtime >= 1.0.0;
 *     }
 *
 * This is syntax only. Capability resolution happens later.
 */

versionRequirementBlock
    : REQUIRES VERSION LEFT_BRACE versionRequirement* RIGHT_BRACE
    ;


versionRequirement
    : compatibilityDeclaration
    | versionRequirementEntry
    ;


versionRequirementEntry
    : compatibilitySubject versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 17. VERSION CAPABILITY DECLARATION
 * ============================================================================
 *
 * A source unit may declare the versions it understands.
 *
 * This is different from selecting a particular machine.
 */

versionCapabilityDeclaration
    : SUPPORTS VERSION LEFT_BRACE versionCapabilityEntry* RIGHT_BRACE
    ;


versionCapabilityEntry
    : compatibilitySubject versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 18. DIALECT VERSIONING
 * ============================================================================
 *
 * Dialects are independently versionable.
 *
 * This is important for:
 *
 *   quantum
 *   hardware
 *   HDL
 *   AI
 *   distributed
 *   accelerator
 *   interoperability
 *   future domains
 *
 * The grammar does not enumerate all future dialects.
 */

dialectVersionDeclaration
    : DIALECT identifier VERSION versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 19. PACKAGE VERSIONING
 * ============================================================================
 *
 * Packages are independently versionable from the language itself.
 *
 * This prevents package evolution from becoming coupled to the core
 * language grammar.
 */

packageVersionDeclaration
    : PACKAGE identifier VERSION versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 20. API / ABI VERSIONING
 * ============================================================================
 */

apiVersionDeclaration
    : API identifier VERSION versionExpression SEMICOLON
    ;


abiVersionDeclaration
    : ABI identifier VERSION versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 21. COMPILER / RUNTIME VERSION CONTRACTS
 * ============================================================================
 *
 * These describe compatibility requirements.
 *
 * They do NOT select a compiler executable or runtime implementation.
 */

compilerVersionRequirement
    : COMPILER versionExpression SEMICOLON
    ;


runtimeVersionRequirement
    : RUNTIME versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 22. GRAMMAR VERSION CONTRACT
 * ============================================================================
 *
 * This is particularly important for POCO-REAF.
 *
 * A source program can identify the grammar contract under which it was
 * authored without embedding the implementation version of the parser.
 */

grammarVersionRequirement
    : GRAMMAR versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 23. VERSION POLICY
 * ============================================================================
 *
 * The grammar permits an explicit policy name.
 *
 * Examples:
 *
 *     exact
 *     compatible
 *     minimum
 *     maximum
 *     range
 *
 * Policy semantics belong to semantic analysis.
 */

versionPolicy
    : EXACT
    | COMPATIBLE
    | identifier
    ;


versionPolicyDeclaration
    : VERSION versionPolicy versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 24. VERSIONED CONTRACT
 * ============================================================================
 *
 * Generic extensibility point for future version-aware language constructs.
 *
 * This avoids adding a new parser rule every time a new subsystem requires
 * version metadata.
 */

versionedContract
    : identifier VERSION versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 25. IDENTIFIER BRIDGE
 * ============================================================================
 *
 * These rules intentionally delegate identifier structure to the lexical
 * foundation.
 *
 * They must NOT duplicate identifier rules from lexer/identifiers.g4 or
 * core/names.g4.
 */

identifier
    : IDENTIFIER
    ;


qualifiedIdentifier
    : identifier
    | qualifiedIdentifier NAMESPACE_SEPARATOR identifier
    ;


/*
 * ============================================================================
 * 26. BOOLEAN BRIDGE
 * ============================================================================
 */

booleanLiteral
    : TRUE
    | FALSE
    ;