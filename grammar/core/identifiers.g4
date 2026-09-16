/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/identifiers.g4
 *
 * Role:
 *     Domain-neutral parser-level identifier contract and reusable identifier
 *     reference grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded actions or target-language code.
 *     Zamani compiler/runtime implementations MUST use safe Rust only.
 *     No `unsafe` Rust is required or permitted by this contract.
 *
 * ============================================================================
 *
 * IMPORTANT ARCHITECTURAL DECISION
 * ============================================================================
 *
 * The repository already has two distinct identifier responsibilities:
 *
 *     grammar/lexer/identifiers.g4
 *         |
 *         +-- lexical IDENTIFIER token
 *
 *     grammar/core/names.g4
 *         |
 *         +-- parser-level identifier/name/qualifiedName structures
 *
 * This file MUST NOT replace either owner.
 *
 * This file provides reusable CORE IDENTIFIER CONSUMPTION RULES.
 *
 * It therefore deliberately does NOT define:
 *
 *     IDENTIFIER
 *     IDENTIFIER_START
 *     IDENTIFIER_CONTINUE
 *     identifier
 *     qualifiedName
 *
 * where those rules are already canonically owned elsewhere.
 *
 * ============================================================================
 *
 * AUTHORITY
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/lexer/identifiers.g4
 *     grammar/lexer/tokens.g4
 *     grammar/lexer/unicode.g4
 *
 * Parser name authority:
 *
 *     grammar/core/names.g4
 *
 * This file:
 *
 *     grammar/core/identifiers.g4
 *
 * consumes the canonical lexical vocabulary and provides reusable parser
 * contexts for identifier references.
 *
 * ============================================================================
 *
 * PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       | IDENTIFIER
 *       v
 *     core identifier grammar
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     name/module/symbol resolution
 *       |
 *       v
 *     semantic model
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 *   classical IR           quantum::ir
 *                               |
 *                               v
 *                    optimization / routing /
 *                    scheduling / QEC / ZQN /
 *                    resilience / HAL
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - parser-level identifier references;
 *   - reusable identifier-reference lists;
 *   - reusable optional identifier-reference lists;
 *   - identifier binding references;
 *   - identifier labels where a single lexical identifier is required;
 *   - identifier patterns at the syntactic level;
 *   - syntactic separation between an identifier token and its later meaning.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifier characters;
 *   - Unicode identifier classes;
 *   - Unicode normalization;
 *   - Unicode security policy;
 *   - keyword recognition;
 *   - lexical tokenization;
 *   - reserved-word tables;
 *   - qualified-name semantics;
 *   - namespace resolution;
 *   - module resolution;
 *   - package resolution;
 *   - symbol tables;
 *   - type checking;
 *   - generic type semantics;
 *   - expression semantics;
 *   - quantum semantics;
 *   - QubitId;
 *   - PhysicalQubitId;
 *   - GateKind;
 *   - hardware identity;
 *   - device identity;
 *   - resource identity;
 *   - topology;
 *   - scheduling;
 *   - routing;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime state;
 *   - compiler backend behavior.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Identifier references are completely independent of execution resources.
 *
 * Nothing in this file imposes a limit on:
 *
 *   - identifier length;
 *   - number of identifiers;
 *   - number of declarations;
 *   - number of bindings;
 *   - number of namespaces;
 *   - number of modules;
 *   - number of domains;
 *   - number of quantum objects;
 *   - number of hardware resources;
 *   - number of machines;
 *   - number of nodes;
 *   - number of accelerators;
 *   - number of processes;
 *   - number of threads;
 *   - number of timelines.
 *
 * Repeated structures therefore use unbounded grammar repetition:
 *
 *     *
 *
 * rather than artificial finite maxima.
 *
 * Practical parser/compiler resource limits are implementation or deployment
 * concerns and MUST NOT become language semantics.
 *
 * ============================================================================
 *
 * OPEN-WORLD DOMAIN PRINCIPLE
 * ============================================================================
 *
 * An identifier is intentionally domain-neutral.
 *
 * The following can all be syntactically represented by the same identifier
 * mechanism when they are not reserved keywords:
 *
 *     algorithm
 *     tensor
 *     qubit_register
 *     accelerator
 *     qpu
 *     device
 *     node
 *     photonic
 *     neuromorphic
 *     molecular
 *     optical
 *     future_domain
 *
 * The grammar does not decide what any of these names mean.
 *
 * Meaning is assigned by later semantic/domain-specific layers.
 *
 * This allows new computational domains to be added without changing the
 * foundational identifier representation merely because a new domain exists.
 *
 * ============================================================================
 *
 * KEYWORD BOUNDARY
 * ============================================================================
 *
 * Keyword recognition belongs to the canonical lexer.
 *
 * This file MUST NOT reproduce keyword spelling such as:
 *
 *     fn
 *     let
 *     module
 *     quantum
 *     hardware
 *
 * as identifier exclusions.
 *
 * The lexer decides whether a source spelling produces:
 *
 *     IDENTIFIER
 *
 * or:
 *
 *     K_* keyword token
 *
 * The parser consumes the resulting token vocabulary.
 *
 * This preserves a single keyword authority and allows keyword evolution to
 * remain independent from the core identifier-reference grammar.
 *
 * ============================================================================
 *
 * UNICODE BOUNDARY
 * ============================================================================
 *
 * Unicode identifier validity is determined before this grammar is reached.
 *
 * This file MUST NOT:
 *
 *   - normalize Unicode;
 *   - case-fold Unicode;
 *   - reject Unicode based on compiler target;
 *   - inspect Unicode code points;
 *   - silently rewrite source spelling.
 *
 * Source spelling must remain available to diagnostics and AST/source-span
 * preservation.
 *
 * Unicode normalization and security policy belong to:
 *
 *     grammar/lexer/unicode.g4
 *     grammar/spec/lexical.md
 *     semantic name-resolution/security analysis
 *
 * ============================================================================
 *
 * NUMERIC BOUNDARY
 * ============================================================================
 *
 * This grammar consumes IDENTIFIER as a token.
 *
 * Therefore it does not need to distinguish:
 *
 *     123
 *     abc123
 *
 * at the character level.
 *
 * That distinction belongs to the lexer.
 *
 * ============================================================================
 *
 * CASE SENSITIVITY
 * ============================================================================
 *
 * This grammar does not perform case folding.
 *
 * These lexical spellings remain distinct when both are valid:
 *
 *     value
 *     Value
 *     VALUE
 *
 * Whether a language-level naming convention or semantic restriction exists
 * belongs downstream.
 *
 * ============================================================================
 *
 * NAME VS PATH
 * ============================================================================
 *
 * An identifier reference is not a filesystem path.
 *
 * This file MUST NOT recognize:
 *
 *     ./src
 *     ../module
 *     /absolute/path
 *     C:/path
 *     https://example
 *
 * Path syntax belongs to:
 *
 *     grammar/core/paths.g4
 *
 * A path may contain identifiers, but a path is not an identifier reference.
 *
 * ============================================================================
 *
 * NAME VS MEMBER ACCESS
 * ============================================================================
 *
 * This file does not own member-access syntax.
 *
 * For example:
 *
 *     object.member
 *
 * contains identifiers, but the `.` expression is owned by expression
 * grammar.
 *
 * Likewise:
 *
 *     object.method()
 *
 * is expression syntax.
 *
 * ============================================================================
 *
 * NAME VS QUALIFIED NAME
 * ============================================================================
 *
 * This file intentionally does not redefine:
 *
 *     qualifiedName
 *
 * Canonical qualified-name syntax remains owned by:
 *
 *     grammar/core/names.g4
 *
 * A consumer requiring a qualified name MUST consume the canonical
 * `qualifiedName` rule supplied by the core name grammar.
 *
 * This avoids two potentially divergent definitions such as:
 *
 *     core/identifiers.g4 -> qualifiedName
 *     core/names.g4       -> qualifiedName
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * These rules produce parser structure only.
 *
 * The frontend AST may preserve:
 *
 *     - source spelling;
 *     - token/span information;
 *     - identifier occurrence kind;
 *     - syntactic context.
 *
 * The AST MUST NOT resolve:
 *
 *     identifier -> symbol
 *
 * during parsing.
 *
 * Name resolution occurs later.
 *
 * ============================================================================
 *
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every identifier occurrence consumed through these rules must remain
 * traceable to its original source token/span.
 *
 * The parser MUST NOT synthesize replacement identifier text.
 *
 * Downstream diagnostics must therefore be able to identify:
 *
 *     source file
 *     source span
 *     original spelling
 *     syntactic context
 *
 * without re-lexing the source.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * An identifier occurrence is syntactic data.
 *
 * Semantic analysis determines whether it denotes:
 *
 *     variable
 *     constant
 *     function
 *     type
 *     module
 *     namespace
 *     resource
 *     capability
 *     effect
 *     quantum object
 *     hardware abstraction
 *     distributed object
 *     AI/data object
 *     HDL object
 *     future-domain object
 *
 * This file MUST NOT make that determination.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum grammar may consume these rules for source-level names:
 *
 *     q
 *     register
 *     state
 *     operation
 *     observable
 *     logical
 *
 * But this file does NOT create:
 *
 *     QubitId
 *     PhysicalQubitId
 *     LogicalQubitId
 *
 * and does not determine whether an identifier denotes a physical qubit.
 *
 * Quantum semantic lowering remains:
 *
 *     source syntax
 *         ->
 *     AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 *
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware grammar may consume identifier references for abstract names such
 * as:
 *
 *     accelerator
 *     memory
 *     interconnect
 *     capability
 *     target
 *
 * This file does not assign physical identities.
 *
 * It does not encode:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     node0
 *
 * as universal language resources.
 *
 * Such names, if explicitly written by a program, remain source-level names.
 * Their meaning is determined downstream.
 *
 * ============================================================================
 *
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed grammar may use identifier references for:
 *
 *     service
 *     actor
 *     channel
 *     process
 *     endpoint
 *     logical node
 *
 * There is no fixed node count or process count in this grammar.
 *
 * ============================================================================
 *
 * AI / DATA / HDL INTEGRATION
 * ============================================================================
 *
 * AI, data, HDL, networking, security, classical and future-domain grammars
 * may all reuse these rules.
 *
 * No domain gets its own lexical identifier universe.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing consumers that need the canonical parser rule:
 *
 *     identifier
 *
 * should continue using:
 *
 *     grammar/core/names.g4
 *
 * Existing consumers that need:
 *
 *     qualifiedName
 *
 * should continue using:
 *
 *     grammar/core/names.g4
 *
 * This file provides additional reusable rules whose names are intentionally
 * distinct so that adding this file cannot silently create duplicate parser
 * rules during grammar composition.
 *
 * ============================================================================
 *
 * NO CIRCULAR DEPENDENCIES
 * ============================================================================
 *
 * This grammar depends only on the canonical lexical token vocabulary:
 *
 *     ZamaniTokens
 *
 * It MUST NOT import or semantically depend on:
 *
 *     quantum grammar
 *     hardware grammar
 *     type grammar
 *     expression grammar
 *     module grammar
 *     runtime
 *     compiler
 *     IR
 *     HAL
 *     QEC
 *     ZQN
 *
 * Higher-level grammars consume these rules.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * These rules contain:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no runtime-dependent decisions;
 *     - no target-dependent decisions;
 *     - no randomness.
 *
 * The same token stream therefore produces the same syntactic result.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Parsing an identifier MUST have no side effects.
 *
 * An identifier such as:
 *
 *     tool::execute
 *
 * is still only syntactic data at this stage.
 *
 * The parser MUST NOT:
 *
 *     - execute commands;
 *     - open files;
 *     - access environment variables;
 *     - contact a network;
 *     - invoke a backend;
 *     - access credentials;
 *     - discover hardware;
 *     - mutate the filesystem.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] It does not redefine IDENTIFIER.
 *   [x] It does not redefine Unicode identifier fragments.
 *   [x] It does not redefine keyword recognition.
 *   [x] It does not duplicate canonical qualifiedName ownership.
 *   [x] It consumes ZamaniTokens.
 *   [x] It provides reusable parser-level identifier references.
 *   [x] It has no artificial identifier/list limits.
 *   [x] It has no target-specific rules.
 *   [x] It has no hardware-specific limits.
 *   [x] It has no quantum-specific identity types.
 *   [x] It has no semantic actions.
 *   [x] It has no Rust unsafe requirements.
 *   [x] It preserves the AST/name-resolution boundary.
 *   [x] It is reusable by all computational domains.
 *
 * ============================================================================
 */

parser grammar CoreIdentifiers;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. SINGLE IDENTIFIER REFERENCE
 * ============================================================================
 *
 * This is the fundamental parser-level bridge from the lexical IDENTIFIER
 * token into core syntax.
 *
 * It intentionally does not use the rule name `identifier`, because that
 * canonical parser rule already belongs to grammar/core/names.g4.
 *
 * Consumers requiring the canonical `identifier` rule should continue to use
 * names.g4.
 */
identifierReference
    : IDENTIFIER
    ;


/* ============================================================================
 * 2. IDENTIFIER BINDING
 * ============================================================================
 *
 * A binding occurrence is syntactically an identifier.
 *
 * Whether the binding is a variable, parameter, field, type, resource, etc.
 * is determined by the surrounding grammar and semantic analysis.
 */
identifierBinding
    : identifierReference
    ;


/* ============================================================================
 * 3. IDENTIFIER LABEL
 * ============================================================================
 *
 * A label is still just an identifier at the lexical level.
 *
 * Control-flow grammars decide whether a label is legal in a particular
 * syntactic position.
 */
identifierLabel
    : identifierReference
    ;


/* ============================================================================
 * 4. IDENTIFIER PATTERN
 * ============================================================================
 *
 * Pattern grammars can use this rule as the generic identifier-pattern
 * boundary.
 *
 * It does not assign binding semantics.
 */
identifierPattern
    : identifierReference
    ;


/* ============================================================================
 * 5. IDENTIFIER LIST
 * ============================================================================
 *
 * One or more identifiers separated by commas.
 *
 * There is deliberately no finite maximum.
 *
 * Examples:
 *
 *     a
 *     a, b
 *     a, b, c
 *
 * Larger lists remain syntactically valid subject only to implementation and
 * resource availability.
 */
identifierReferenceList
    : identifierReference (COMMA identifierReference)*
    ;


/* ============================================================================
 * 6. OPTIONAL IDENTIFIER LIST
 * ============================================================================
 *
 * Empty lists are represented here only where the consuming grammar permits
 * an optional list.
 *
 * The non-empty form remains `identifierReferenceList`.
 */
optionalIdentifierReferenceList
    : identifierReferenceList?
    ;


/* ============================================================================
 * 7. TRAILING-COMMA IDENTIFIER LIST
 * ============================================================================
 *
 * Some declaration constructs permit a trailing comma.
 *
 * This reusable rule prevents every consumer from independently recreating
 * the same list structure.
 */
identifierReferenceListTrailingComma
    : identifierReference (COMMA identifierReference)* COMMA?
    ;


/* ============================================================================
 * 8. NON-EMPTY IDENTIFIER LIST WITH EXPLICIT SEPARATOR
 * ============================================================================
 *
 * Reusable form for contexts whose surrounding syntax already establishes
 * the semantic purpose of the list.
 *
 * Example conceptual use:
 *
 *     (a, b, c)
 *
 * This rule owns only the identifier sequence.
 */
identifierSequence
    : identifierReference (COMMA identifierReference)*
    ;


/* ============================================================================
 * 9. IDENTIFIER + ALIAS BRIDGE
 * ============================================================================
 *
 * This rule intentionally handles only:
 *
 *     identifier as identifier
 *
 * Qualified-name aliasing remains owned by names.g4 and module/import
 * grammars.
 *
 * The AS token is owned by the canonical lexer vocabulary.
 */
identifierAlias
    : identifierReference K_AS identifierReference
    ;


/* ============================================================================
 * 10. IDENTIFIER ALIAS LIST
 * ============================================================================
 *
 * Repeated aliases are unlimited by language design.
 */
identifierAliasList
    : identifierAlias (COMMA identifierAlias)*
    ;


/* ============================================================================
 * 11. OPTIONAL IDENTIFIER ALIAS LIST
 * ============================================================================
 */
optionalIdentifierAliasList
    : identifierAliasList?
    ;


/* ============================================================================
 * 12. IDENTIFIER PAIR
 * ============================================================================
 *
 * Useful for syntax where two identifier occurrences have a syntactic
 * relationship, while deliberately leaving semantic interpretation to the
 * consumer.
 */
identifierPair
    : identifierReference identifierReference
    ;


/* ============================================================================
 * 13. IDENTIFIER ASSIGNMENT LABEL
 * ============================================================================
 *
 * This is deliberately not an expression assignment.
 *
 * It provides a reusable syntactic form for constructs such as:
 *
 *     name = ...
 *
 * where the consuming grammar owns the right-hand side.
 *
 * The '=' token remains owned by the canonical lexer.
 */
identifierAssignmentTarget
    : identifierReference
    ;


/* ============================================================================
 * 14. IDENTIFIER INHERITANCE / IMPLEMENTATION TARGET
 * ============================================================================
 *
 * This rule provides a neutral identifier occurrence for higher-level
 * declaration grammars.
 *
 * It does not decide whether the name identifies a type, trait, interface,
 * capability, or another declaration.
 */
identifierTypeReference
    : identifierReference
    ;


/* ============================================================================
 * 15. IDENTIFIER RESOURCE REFERENCE
 * ============================================================================
 *
 * Hardware/resource grammars may use this parser bridge.
 *
 * The identifier remains abstract source syntax.
 *
 * It does not imply:
 *
 *     physical device
 *     physical memory
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     node
 *
 * or any other concrete resource.
 */
identifierResourceReference
    : identifierReference
    ;


/* ============================================================================
 * 16. IDENTIFIER CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capability grammars may use this rule for a source-level capability name.
 *
 * Capability satisfaction remains semantic analysis.
 */
identifierCapabilityReference
    : identifierReference
    ;


/* ============================================================================
 * 17. IDENTIFIER EFFECT REFERENCE
 * ============================================================================
 *
 * Effect grammars may use this rule for source-level effect names.
 *
 * Effect semantics remain outside this grammar.
 */
identifierEffectReference
    : identifierReference
    ;


/* ============================================================================
 * 18. IDENTIFIER DOMAIN REFERENCE
 * ============================================================================
 *
 * Domain grammars may use this for names representing:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     future domains
 *
 * The identifier itself carries no domain semantics.
 */
identifierDomainReference
    : identifierReference
    ;


/* ============================================================================
 * 19. IDENTIFIER TARGET REFERENCE
 * ============================================================================
 *
 * Compilation/execution grammar may use this for an abstract target name.
 *
 * It does not select or discover a physical target.
 */
identifierTargetReference
    : identifierReference
    ;


/* ============================================================================
 * 20. IDENTIFIER NAMESPACE COMPONENT
 * ============================================================================
 *
 * This is deliberately only one component.
 *
 * Complete qualification remains owned by names.g4.
 *
 * This prevents this file from creating a second qualified-name grammar.
 */
identifierNamespaceComponent
    : identifierReference
    ;


/* ============================================================================
 * 21. IDENTIFIER DECLARATION NAME
 * ============================================================================
 *
 * A declaration grammar may use this rule when it needs to make the
 * syntactic distinction between a declaration name and another occurrence.
 *
 * Semantic declaration registration remains outside the grammar.
 */
identifierDeclarationName
    : identifierReference
    ;


/* ============================================================================
 * 22. IDENTIFIER USE
 * ============================================================================
 *
 * A generic identifier-use occurrence.
 *
 * This distinction is useful to AST construction without performing semantic
 * resolution in the parser.
 */
identifierUse
    : identifierReference
    ;


/* ============================================================================
 * 23. IDENTIFIER LIST WITH TRAILING COMMA
 * ============================================================================
 *
 * Reusable declaration-friendly list.
 */
identifierDeclarationList
    : identifierDeclarationName
      (COMMA identifierDeclarationName)*
      COMMA?
    ;


/* ============================================================================
 * 24. OPTIONAL DECLARATION LIST
 * ============================================================================
 */
optionalIdentifierDeclarationList
    : identifierDeclarationList?
    ;


/* ============================================================================
 * 25. INTEGRATION SENTINEL
 * ============================================================================
 *
 * This rule intentionally consumes one identifier only.
 *
 * It exists as a stable minimal integration point for grammar validation
 * tests and tooling.
 */
identifierCore
    : IDENTIFIER
    ;