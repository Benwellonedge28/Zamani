/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/hygiene.g4
 *
 * Role:
 *     Macro-hygiene parser boundary.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, predicates, executable
 *     host-language code, filesystem access, network access, process
 *     execution, or unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * Macro hygiene is a compiler-semantic property that prevents macro expansion
 * from accidentally changing the binding structure of the caller's program.
 *
 * Conceptually:
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
 *     source AST
 *       |
 *       +--> macro resolution
 *       |
 *       +--> hygiene / provenance analysis
 *       |
 *       +--> controlled macro expansion
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical IR
 *
 * This file owns ONLY the parser-level boundary through which an annotation
 * may be associated with macro-hygiene intent.
 *
 * It does NOT implement hygiene.
 *
 * ============================================================================
 * SINGLE SOURCE OF TRUTH
 * ============================================================================
 *
 * This file deliberately REUSES the canonical `annotation` rule.
 *
 * The canonical annotation syntax is owned by:
 *
 *     grammar/core/annotations.g4
 *
 * That grammar owns:
 *
 *     annotation
 *     annotationList
 *     annotationName
 *     annotationArguments
 *     annotationArgument
 *     annotationValue
 *     ...
 *
 * This file MUST NOT redefine any of those rules.
 *
 * Consequently, hygiene syntax does not create a second:
 *
 *     identifier system
 *     annotation system
 *     attribute system
 *     qualified-name system
 *     literal system
 *     argument system
 *
 * ============================================================================
 * IMPORTANT DESIGN DECISION
 * ============================================================================
 *
 * There is currently no repository-wide canonical lexer contract establishing
 * dedicated tokens such as:
 *
 *     HYGIENE
 *     CAPTURE
 *     FRESH
 *     CALL_SITE
 *     DEF_SITE
 *     RAW_IDENTIFIER
 *     UNHYGIENIC
 *
 * Therefore this grammar MUST NOT invent such tokens.
 *
 * Likewise, this file MUST NOT invent syntax such as:
 *
 *     hygiene(...)
 *     capture(...)
 *     fresh(...)
 *     @hygienic(...)
 *     @capture(...)
 *
 * as special parser keywords.
 *
 * Those spellings, if eventually standardized, must first be established by
 * the language specification and canonical annotation/lexer contracts.
 *
 * The extensible mechanism already available is:
 *
 *     annotation
 *
 * Semantic analysis can recognize a registered hygiene annotation by its
 * canonical qualified name.
 *
 * This keeps the grammar stable when new hygiene mechanisms are introduced.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the parser-level macro hygiene annotation boundary;
 *     - one or more hygiene annotations;
 *     - optional hygiene annotation sequences;
 *     - the explicit integration point consumed by macro declarations and/or
 *       macro expansion syntax;
 *     - the fact that hygiene metadata is represented using canonical
 *       annotation syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - `@` lexical representation;
 *     - identifiers;
 *     - qualified names;
 *     - annotation argument syntax;
 *     - annotation value syntax;
 *     - macro declarations;
 *     - macro invocation syntax;
 *     - macro expansion;
 *     - macro resolution;
 *     - scope construction;
 *     - binding resolution;
 *     - capture analysis;
 *     - fresh-name generation;
 *     - symbol-table implementation;
 *     - source-map implementation;
 *     - provenance implementation;
 *     - AST implementation;
 *     - canonical IR;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL representation;
 *     - hardware discovery;
 *     - resource discovery;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * COMPILER RESPONSIBILITY BOUNDARY
 * ============================================================================
 *
 * LEXER
 *     Recognizes the canonical annotation marker and ordinary lexical tokens.
 *
 * PARSER
 *     Recognizes the structural annotation syntax and exposes it through the
 *     macro-hygiene boundary defined here.
 *
 * AST
 *     Stores the annotation structure together with source spans and
 *     provenance.
 *
 * NAME RESOLUTION
 *     Resolves annotation names and macro names.
 *
 * HYGIENE ANALYSIS
 *     Determines binding identity, lexical context, definition-site context,
 *     invocation-site context, and capture behavior.
 *
 * MACRO EXPANSION
 *     Applies hygiene transformations during controlled expansion.
 *
 * SEMANTIC ANALYSIS
 *     Determines whether a requested hygiene operation is legal.
 *
 * CANONICAL IR
 *     Receives already-resolved semantic constructs.
 *
 * ============================================================================
 * HYGIENE MODEL
 * ============================================================================
 *
 * Macro hygiene must preserve the distinction between at least:
 *
 *     1. caller-introduced identifiers;
 *     2. macro-definition identifiers;
 *     3. macro-generated identifiers;
 *     4. explicitly captured identifiers;
 *     5. referenced external bindings;
 *     6. generated declarations;
 *     7. generated references.
 *
 * The parser does not assign those identities.
 *
 * It merely preserves the source syntax from which the semantic layer can
 * construct the appropriate identity/provenance information.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Every hygiene annotation parsed through this file must remain traceable to:
 *
 *     source file
 *     source span
 *     annotation span
 *     enclosing declaration/invocation
 *     macro expansion context
 *
 * Provenance is essential for:
 *
 *     diagnostics
 *     debugging
 *     IDE tooling
 *     source maps
 *     deterministic builds
 *     reproducibility
 *     security auditing
 *     macro expansion diagnostics
 *
 * This grammar does not construct provenance objects itself.
 *
 * The parser/AST integration must preserve the source locations required by
 * the frontend provenance system.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Hygiene syntax must remain independent of execution hardware.
 *
 * The same source-level macro and hygiene semantics must be usable when the
 * resulting computation targets:
 *
 *     embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     quantum processors
 *     quantum simulators
 *     heterogeneous accelerators
 *     clusters
 *     supercomputers
 *     distributed systems
 *     cloud environments
 *     future architectures
 *
 * Hygiene must never imply:
 *
 *     device selection
 *     CPU selection
 *     GPU selection
 *     QPU selection
 *     qubit allocation
 *     memory allocation
 *     topology selection
 *     scheduler selection
 *     backend selection
 *
 * Those decisions belong to later compiler/resource/target layers.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately contains no finite machine-dependent limits.
 *
 * There is no:
 *
 *     MAX_MACRO_HYGIENE_ANNOTATIONS
 *     MAX_CAPTURE_COUNT
 *     MAX_IDENTIFIER_COUNT
 *     MAX_BINDINGS
 *     MAX_SCOPES
 *     MAX_EXPANSION_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * The grammar therefore does not impose an artificial scalability ceiling.
 *
 * Compiler implementations MAY impose configurable resource budgets for:
 *
 *     source bytes
 *     tokens
 *     AST nodes
 *     macro expansion steps
 *     expansion depth
 *     generated nodes
 *     memory
 *     compilation time
 *
 * Such limits are implementation/resource policies, not language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing of hygiene annotations is deterministic.
 *
 * Hygiene analysis and expansion must additionally be deterministic with
 * respect to:
 *
 *     source
 *     macro definitions
 *     macro arguments
 *     lexical scope
 *     semantic environment
 *     explicit compiler policy
 *
 * No random identifier generation may be used as an observable semantic
 * mechanism.
 *
 * If internally generated hygiene identities require unique IDs, those IDs
 * belong to the compiler's deterministic identity/provenance subsystem.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Hygiene metadata MUST NOT grant permission to:
 *
 *     access the filesystem
 *     access the network
 *     execute processes
 *     execute arbitrary host code
 *     mutate compiler configuration
 *     bypass capability checks
 *     bypass effect checks
 *     bypass resource checks
 *     bypass security checks
 *     select hardware
 *     select a backend
 *
 * A macro's hygiene annotation is declarative metadata.
 *
 * It is not a privilege escalation mechanism.
 *
 * ============================================================================
 * DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * The hygiene mechanism is domain-neutral.
 *
 * Macro expansion may eventually generate syntax for:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware
 *     distributed computation
 *     AI/ML
 *     networking
 *     cryptography
 *     scientific computing
 *     future dialects
 *
 * This grammar does not need to know which domain the generated syntax
 * belongs to.
 *
 * Domain semantics are resolved after macro expansion.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A macro may generate quantum syntax.
 *
 * Hygiene remains independent of quantum semantics.
 *
 * This file therefore MUST NOT define:
 *
 *     qubit hygiene
 *     quantum binding identity
 *     physical qubit identity
 *     logical qubit identity
 *     gate identity
 *     QEC behavior
 *     ZQN behavior
 *
 * Those concepts belong to the quantum semantic/compiler layers.
 *
 * After semantic lowering, quantum constructs continue toward:
 *
 *     quantum::ir
 *
 * which remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hygiene metadata does not select:
 *
 *     hardware device
 *     physical address
 *     topology
 *     resource count
 *     processor type
 *     accelerator
 *     timing model
 *
 * Hardware-independent source semantics must remain portable.
 *
 * ============================================================================
 * 1. SINGLE HYGIENE ANNOTATION
 * ============================================================================
 *
 * `macroHygieneAnnotation` is deliberately an aliasing boundary over the
 * canonical annotation rule.
 *
 * It does NOT reinterpret annotation syntax.
 *
 * Semantic analysis determines whether the annotation's qualified name
 * identifies a recognized hygiene facility.
 *
 * Conceptually:
 *
 *     @some_annotation
 *
 * becomes:
 *
 *     annotation
 *         |
 *         v
 *     macroHygieneAnnotation
 *
 * only when consumed through a macro-hygiene grammar context.
 *
 * The parser does not determine whether `some_annotation` is actually a
 * hygiene annotation.
 *
 * ============================================================================
 */

parser grammar hygiene;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 2. SINGLE HYGIENE ANNOTATION
 * ============================================================================
 *
 * Reuse the canonical annotation grammar.
 *
 * Do NOT replace this with:
 *
 *     AT IDENTIFIER
 *
 * because the repository's canonical annotation syntax owns:
 *
 *     annotation names
 *     qualified names
 *     arguments
 *     structured values
 *     nested values
 *
 * Reusing `annotation` prevents a second annotation language from emerging.
 */

macroHygieneAnnotation
    : annotation
    ;


/*
 * ============================================================================
 * 3. HYGIENE ANNOTATION LIST
 * ============================================================================
 *
 * A macro construct may carry multiple hygiene-related annotations.
 *
 * No finite maximum is imposed.
 *
 * Example semantic possibilities include future forms equivalent to:
 *
 *     @hygiene::...
 *     @capture::...
 *     @fresh::...
 *     @scope::...
 *
 * but these names are NOT reserved here.
 *
 * Their recognition belongs to the semantic annotation registry.
 */

macroHygieneAnnotations
    : macroHygieneAnnotation+
    ;


/*
 * ============================================================================
 * 4. OPTIONAL HYGIENE ANNOTATIONS
 * ============================================================================
 *
 * Consumers that permit hygiene metadata may use this rule.
 *
 * The empty case is valid.
 *
 * The consumer decides whether hygiene metadata is permitted at its specific
 * syntactic attachment point.
 */

optionalMacroHygieneAnnotations
    : macroHygieneAnnotation*
    ;


/*
 * ============================================================================
 * 5. HYGIENE ANNOTATION PREFIX
 * ============================================================================
 *
 * This named boundary exists so downstream macro grammar components can
 * explicitly declare:
 *
 *     "this construct accepts hygiene metadata"
 *
 * without duplicating annotation syntax.
 *
 * It intentionally does not determine which annotations are legal.
 *
 * Semantic validation performs that determination.
 */

macroHygienePrefix
    : macroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 6. OPTIONAL HYGIENE PREFIX
 * ============================================================================
 *
 * This is the preferred integration rule for macro declarations and other
 * macro-owned syntax that permits optional hygiene metadata.
 *
 * Example structural shape:
 *
 *     [optional hygiene annotations]
 *     macro declaration
 *
 * The exact attachment position belongs to the consuming macro grammar.
 *
 * This file does not force annotations onto every macro declaration.
 */

optionalMacroHygienePrefix
    : optionalMacroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 7. INVOCATION-SIDE HYGIENE BOUNDARY
 * ============================================================================
 *
 * Macro invocation syntax remains owned by:
 *
 *     grammar/macros/invocations.g4
 *
 * This rule provides an integration boundary for an invocation grammar that
 * explicitly supports hygiene metadata.
 *
 * It does NOT redefine:
 *
 *     macroInvocation
 *     macroPath
 *     argumentList
 *
 * A consuming grammar may attach this boundary to the invocation syntax once
 * the language specification defines the permitted source position.
 *
 * The attachment position must be standardized before it is accepted as a
 * new source construct.
 */

macroInvocationHygiene
    : macroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 8. DECLARATION-SIDE HYGIENE BOUNDARY
 * ============================================================================
 *
 * Macro declarations may eventually expose declaration-level hygiene policy.
 *
 * This grammar provides the reusable syntax boundary.
 *
 * The declaration grammar remains responsible for deciding where this rule
 * appears.
 */

macroDeclarationHygiene
    : macroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 9. EXPANSION-SIDE HYGIENE BOUNDARY
 * ============================================================================
 *
 * Expansion syntax may need to preserve hygiene metadata attached to generated
 * source structures.
 *
 * The expansion grammar consumes this rule.
 *
 * Expansion itself is NOT performed here.
 */

macroExpansionHygiene
    : macroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 10. SEMANTIC REGISTRY BOUNDARY
 * ============================================================================
 *
 * This grammar intentionally accepts generic canonical annotations.
 *
 * The semantic annotation registry is responsible for deciding whether an
 * annotation is a recognized hygiene annotation.
 *
 * Examples of possible semantic categories include:
 *
 *     capture avoidance
 *     explicit capture
 *     definition-site binding
 *     invocation-site binding
 *     fresh generated binding
 *     transparent binding
 *     scope preservation
 *     provenance preservation
 *     hygiene diagnostics
 *
 * These are semantic categories, not grammar productions.
 *
 * Adding a new hygiene semantic operation MUST NOT require modifying this
 * grammar unless its source syntax itself changes.
 *
 * ============================================================================
 * 11. INTENTIONAL CAPTURE
 * ============================================================================
 *
 * Intentional capture is a semantic operation.
 *
 * This grammar does not create a special `capture` keyword.
 *
 * If the language standardizes an explicit capture annotation, it should use
 * the canonical annotation infrastructure and be registered semantically.
 *
 * This provides:
 *
 *     syntax stability
 *     namespace extensibility
 *     compatibility
 *     domain independence
 *
 * without turning the parser into a list of compiler implementation details.
 *
 * ============================================================================
 * 12. FRESH IDENTIFIERS
 * ============================================================================
 *
 * Fresh binding generation is NOT a lexical or parser operation.
 *
 * The parser must preserve the source structure that requires hygiene.
 *
 * The macro expansion subsystem is responsible for creating hygienic binding
 * identities.
 *
 * It must not derive semantic uniqueness merely from:
 *
 *     source spelling
 *     textual suffixes
 *     machine IDs
 *     memory addresses
 *     process IDs
 *     timestamps
 *     random values
 *
 * unless such mechanisms are explicitly hidden implementation details and do
 * not affect deterministic semantic identity.
 *
 * ============================================================================
 * 13. CALL-SITE / DEFINITION-SITE CONTEXT
 * ============================================================================
 *
 * Call-site and definition-site context are semantic provenance properties.
 *
 * This grammar does not define separate identifier tokens for them.
 *
 * The AST/provenance layer must retain enough source identity for the hygiene
 * engine to distinguish:
 *
 *     caller source
 *     macro definition source
 *     generated source
 *     nested expansion source
 *
 * ============================================================================
 * 14. NESTED MACROS
 * ============================================================================
 *
 * Hygiene must remain compositional across nested expansion.
 *
 * Example conceptual structure:
 *
 *     macro A
 *         |
 *         +--> macro B
 *                 |
 *                 +--> generated binding
 *
 * Each expansion must retain its provenance and lexical context.
 *
 * This grammar does not impose a nesting limit.
 *
 * Expansion depth limits, if required for resource protection, belong to the
 * macro expansion policy and must be configurable.
 *
 * ============================================================================
 * 15. CROSS-DOMAIN MACROS
 * ============================================================================
 *
 * Hygiene must work identically when macros generate:
 *
 *     classical syntax
 *     quantum syntax
 *     hybrid syntax
 *     HDL syntax
 *     hardware syntax
 *     distributed syntax
 *     AI syntax
 *     data syntax
 *     networking syntax
 *     security syntax
 *     future dialect syntax
 *
 * No domain-specific hygiene grammar is permitted here.
 *
 * ============================================================================
 * 16. RESOURCE AND CAPABILITY SEPARATION
 * ============================================================================
 *
 * Hygiene annotations must never silently become resource requirements.
 *
 * For example, macro hygiene must not implicitly mean:
 *
 *     requires N qubits
 *     requires N CPUs
 *     requires GPU
 *     requires FPGA
 *     requires QPU
 *     requires device X
 *
 * If an annotation also carries resource metadata, that meaning must be
 * explicitly represented by the canonical resource/capability semantic system.
 *
 * Hygiene and resource requirements remain separate semantic dimensions.
 *
 * ============================================================================
 * 17. EFFECT SEPARATION
 * ============================================================================
 *
 * Macro hygiene is not an effect.
 *
 * Hygiene metadata must not bypass:
 *
 *     IO checking
 *     capability checking
 *     security checking
 *     effect checking
 *     resource checking
 *
 * Macro expansion must produce syntax that is subsequently subjected to the
 * normal semantic analysis pipeline.
 *
 * ============================================================================
 * 18. AST CONTRACT
 * ============================================================================
 *
 * This grammar does not introduce a second macro-hygiene AST hierarchy.
 *
 * The parser should represent:
 *
 *     macroHygieneAnnotation
 *
 * using the repository's canonical annotation AST representation.
 *
 * The enclosing macro declaration/invocation/expansion node remains the owner
 * of its macro-specific structure.
 *
 * The resulting AST must preserve:
 *
 *     annotation identity
 *     annotation name
 *     annotation arguments
 *     source span
 *     enclosing construct
 *     source provenance
 *
 * The hygiene engine may then create its own semantic analysis structures.
 *
 * Those structures are NOT parser AST nodes.
 *
 * ============================================================================
 * 19. NO SECOND IDENTIFIER SYSTEM
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     HYGIENE_IDENTIFIER
 *     CAPTURE_IDENTIFIER
 *     FRESH_IDENTIFIER
 *     RAW_IDENTIFIER
 *     CALL_SITE_IDENTIFIER
 *     DEF_SITE_IDENTIFIER
 *
 * unless the language specification explicitly establishes such lexical
 * categories in the canonical lexer.
 *
 * Ordinary identifiers remain ordinary identifiers.
 *
 * Hygiene identity is semantic identity.
 *
 * ============================================================================
 * 20. NO SECOND QUALIFIED-NAME SYSTEM
 * ============================================================================
 *
 * Hygiene annotation names use the canonical annotation/qualified-name
 * infrastructure.
 *
 * This file MUST NOT redefine:
 *
 *     qualifiedName
 *     path
 *     identifier
 *
 * ============================================================================
 * 21. NO SECOND ATTRIBUTE SYSTEM
 * ============================================================================
 *
 * `annotation` is the canonical source-level annotation construct.
 *
 * This file must not introduce:
 *
 *     hygieneAttribute
 *     macroAttribute
 *     captureAttribute
 *
 * as competing syntax systems.
 *
 * If the semantic layer distinguishes attributes from annotations, that
 * distinction must remain owned by the canonical core grammar/specification.
 *
 * ============================================================================
 * 22. MACRO DECLARATION INTEGRATION
 * ============================================================================
 *
 * `grammar/macros/declarations.g4` remains responsible for macro declaration
 * syntax.
 *
 * It may consume:
 *
 *     optionalMacroHygienePrefix
 *
 * if the language specification permits hygiene metadata on declarations.
 *
 * It must NOT redefine:
 *
 *     macroHygieneAnnotation
 *     annotation
 *     annotationList
 *
 * Example conceptual composition:
 *
 *     optionalMacroHygienePrefix
 *     macro declaration core
 *
 * The exact ordering must be chosen once by the macro syntax specification and
 * kept stable for compatibility.
 *
 * ============================================================================
 * 23. MACRO INVOCATION INTEGRATION
 * ============================================================================
 *
 * `grammar/macros/invocations.g4` remains responsible for:
 *
 *     macroPath
 *     macroInvocation
 *     macroExpression
 *
 * It may consume `macroInvocationHygiene` only after the invocation-side
 * placement has been explicitly standardized.
 *
 * This file does not redefine invocation syntax.
 *
 * The existing canonical invocation structure remains:
 *
 *     macroPath
 *     BANG
 *     LPAREN
 *     argumentList?
 *     RPAREN
 *
 * ============================================================================
 * 24. MACRO EXPANSION INTEGRATION
 * ============================================================================
 *
 * `grammar/macros/expansion.g4` owns source-level expansion directives, if
 * such directives are standardized.
 *
 * It may consume:
 *
 *     macroExpansionHygiene
 *
 * but must not implement the expansion algorithm in ANTLR grammar actions.
 *
 * Expansion belongs to safe Rust compiler infrastructure.
 *
 * ============================================================================
 * 25. MACROS.G4 INTEGRATION
 * ============================================================================
 *
 * `grammar/macros/macros.g4` should become the macro grammar composition
 * boundary.
 *
 * It must import/delegate to the macro subcomponents rather than redefining
 * hygiene rules.
 *
 * The final architecture should therefore be conceptually:
 *
 *     macros.g4
 *       |
 *       +--> declarations.g4
 *       |
 *       +--> invocations.g4
 *       |
 *       +--> hygiene.g4
 *       |
 *       +--> expansion.g4
 *
 * Each component owns a distinct concern.
 *
 * ============================================================================
 * 26. META.G4 RECONCILIATION
 * ============================================================================
 *
 * The repository currently has macro-related syntax in:
 *
 *     grammar/antlr/Meta.g4
 *
 * while the newer architecture also has:
 *
 *     grammar/macros/
 *
 * This creates a duplicate ownership risk.
 *
 * `Meta.g4` MUST NOT remain a competing canonical definition of macro hygiene.
 *
 * The migration strategy should be:
 *
 *     Meta.g4
 *         |
 *         +--> consume/delegate to canonical macros grammar
 *         |
 *         or
 *         |
 *         +--> remove duplicated macro productions after compatibility
 *             migration
 *
 * There must ultimately be exactly one canonical source grammar for macro
 * hygiene.
 *
 * ============================================================================
 * 27. ANNOTATION LEXER INTEGRATION
 * ============================================================================
 *
 * The canonical annotation marker is owned by the lexer annotation component.
 *
 * This parser grammar must consume the canonical annotation token indirectly
 * through:
 *
 *     annotation
 *
 * It must NOT define:
 *
 *     AT
 *     ANNOTATION_MARKER
 *     IDENTIFIER
 *
 * itself.
 *
 * This is especially important because the repository currently has an
 * integration mismatch between macro documentation using `AT` and the
 * canonical annotation grammar using `ANNOTATION_MARKER`.
 *
 * That token naming discrepancy must be resolved at the canonical lexer/parser
 * integration boundary, not duplicated or hidden inside this file.
 *
 * ============================================================================
 * 28. HARDWARE / TARGET NON-DEPENDENCY
 * ============================================================================
 *
 * This file has no dependency on:
 *
 *     hardware
 *     target
 *     backend
 *     scheduler
 *     routing
 *     optimization
 *     runtime
 *     quantum device
 *     simulator
 *
 * This non-dependency is intentional and MUST remain true.
 *
 * ============================================================================
 * 29. QUANTUM IR NON-DEPENDENCY
 * ============================================================================
 *
 * This file does not import or reference:
 *
 *     quantum::ir
 *
 * Hygiene occurs before semantic lowering.
 *
 * If a macro expands into quantum constructs, those constructs eventually
 * undergo normal semantic lowering into the canonical quantum IR.
 *
 * No macro grammar file may become a second quantum semantic boundary.
 *
 * ============================================================================
 * 30. ZQN / QEC NON-DEPENDENCY
 * ============================================================================
 *
 * This file does not own:
 *
 *     QEC
 *     ZQN
 *     fault models
 *     noise models
 *     error correction
 *     mitigation
 *
 * Macro hygiene is independent of execution faults.
 *
 * ============================================================================
 * 31. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are limited to syntactic failures.
 *
 * Examples of parser-level failures:
 *
 *     malformed annotation
 *     malformed annotation arguments
 *     malformed annotation value
 *
 * Examples that are NOT parser errors:
 *
 *     unknown hygiene annotation
 *     illegal capture request
 *     capture of an unavailable binding
 *     hygiene conflict
 *     invalid macro scope
 *     expansion-cycle violation
 *     expansion budget exhaustion
 *
 * Those belong to semantic/macro-expansion diagnostics.
 *
 * ============================================================================
 * 32. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid annotation syntax must remain valid.
 *
 * Existing annotation names must not become parser-reserved merely because a
 * future annotation is used for macro hygiene.
 *
 * Introducing a new recognized hygiene annotation is therefore normally a
 * semantic-registry change rather than a grammar-breaking change.
 *
 * If a future hygiene facility requires genuinely new punctuation or lexical
 * syntax, that change must proceed through:
 *
 *     language specification
 *     lexer contract
 *     parser contract
 *     AST contract
 *     compatibility policy
 *     tests
 *
 * ============================================================================
 * 33. VERSIONING
 * ============================================================================
 *
 * Hygiene semantics must be version-aware.
 *
 * The grammar itself should remain as stable as possible by treating hygiene
 * names as canonical annotation names rather than hard-coded parser keywords.
 *
 * A compiler may therefore distinguish:
 *
 *     language version
 *     annotation schema version
 *     macro system version
 *     hygiene semantic version
 *
 * without changing this parser boundary.
 *
 * ============================================================================
 * 34. DETERMINISTIC EXPANSION CONTRACT
 * ============================================================================
 *
 * The macro engine must ensure that identical:
 *
 *     source
 *     macro definitions
 *     arguments
 *     language version
 *     semantic environment
 *     compiler policy
 *
 * produce equivalent hygiene semantics.
 *
 * The parser contributes deterministic source structure.
 *
 * It must not use:
 *
 *     timestamps
 *     randomness
 *     process identifiers
 *     memory addresses
 *     machine identifiers
 *
 * as semantic input.
 *
 * ============================================================================
 * 35. RESOURCE-BUDGET CONTRACT
 * ============================================================================
 *
 * A production compiler may need protection against pathological macro input.
 *
 * Examples:
 *
 *     enormous annotation lists
 *     deeply nested annotations
 *     enormous annotation values
 *     pathological macro nesting
 *     expansion cycles
 *
 * Those limits belong to configurable compiler resource policy.
 *
 * They MUST NOT be represented as grammar constants.
 *
 * ============================================================================
 * 36. TEST CONTRACT — POSITIVE
 * ============================================================================
 *
 * The macro grammar integration tests must verify that canonical annotations
 * can reach the hygiene boundary without creating a second annotation syntax.
 *
 * Examples:
 *
 *     @hygiene
 *     @hygiene::scope
 *     @custom_hygiene
 *     @future::hygiene
 *
 * The exact names are examples only.
 *
 * Tests should use the repository's canonical annotation forms.
 *
 * ============================================================================
 * 37. TEST CONTRACT — STRUCTURED ANNOTATIONS
 * ============================================================================
 *
 * Verify that hygiene annotations can preserve canonical annotation structure,
 * including forms equivalent to:
 *
 *     @name()
 *     @name(value)
 *     @name(key = value)
 *     @namespace::name(value)
 *     @name([value1, value2])
 *     @name({key = value})
 *
 * These are handled by the canonical annotation grammar.
 *
 * `hygiene.g4` must not duplicate those productions.
 *
 * ============================================================================
 * 38. TEST CONTRACT — NEGATIVE
 * ============================================================================
 *
 * Negative tests must verify that this file does not accidentally accept
 * invented hygiene-specific syntax.
 *
 * In particular, unless separately standardized, the parser must NOT acquire
 * special syntax merely because source contains conceptual words such as:
 *
 *     capture
 *     fresh
 *     hygienic
 *     call_site
 *     def_site
 *
 * as bare special keywords.
 *
 * Ordinary identifiers remain governed by the canonical language grammar.
 *
 * ============================================================================
 * 39. TEST CONTRACT — CROSS-DOMAIN
 * ============================================================================
 *
 * Hygiene metadata must remain usable around syntax involving:
 *
 *     classical constructs
 *     quantum constructs
 *     hybrid constructs
 *     HDL constructs
 *     hardware constructs
 *     distributed constructs
 *     AI/data constructs
 *     networking constructs
 *     future dialect constructs
 *
 * The hygiene grammar must not require domain-specific branches.
 *
 * ============================================================================
 * 40. TEST CONTRACT — SCALABILITY
 * ============================================================================
 *
 * Tests must verify that no grammar-level artificial limit exists for:
 *
 *     number of hygiene annotations
 *     annotation nesting
 *     qualified annotation names
 *     macro declarations
 *     macro invocations
 *     nested macro structures
 *
 * Test harnesses MAY use finite values for practical execution.
 *
 * Such finite test values are test configuration, not language limits.
 *
 * ============================================================================
 * 41. TEST CONTRACT — DETERMINISM
 * ============================================================================
 *
 * Parsing the same source repeatedly must produce structurally equivalent
 * parse trees.
 *
 * Hygiene analysis must additionally be tested for deterministic semantic
 * identity.
 *
 * ============================================================================
 * 42. TEST CONTRACT — ROUND TRIP
 * ============================================================================
 *
 * Where the repository provides a source printer:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * must preserve the intended annotation/hygiene structure.
 *
 * ============================================================================
 * 43. TEST CONTRACT — PROVENANCE
 * ============================================================================
 *
 * Tests must verify that hygiene annotations retain source locations through:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> macro resolution
 *       -> expansion
 *
 * Diagnostics for hygiene failures must be able to identify the relevant
 * source locations.
 *
 * ============================================================================
 * 44. TEST CONTRACT — MACRO EXPANSION
 * ============================================================================
 *
 * Integration tests must verify that:
 *
 *     macro definition
 *         +
 *     invocation
 *         +
 *     hygiene metadata
 *
 * produces a semantic expansion whose bindings do not accidentally capture
 * caller bindings.
 *
 * These tests belong primarily to the macro semantic/expansion test suite.
 *
 * The parser test verifies only structural preservation.
 *
 * ============================================================================
 * 45. TEST CONTRACT — INTENTIONAL CAPTURE
 * ============================================================================
 *
 * If the semantic annotation registry later provides an explicit intentional
 * capture facility, tests must verify:
 *
 *     implicit capture -> rejected/prevented
 *
 *     explicit capture -> accepted only where policy permits
 *
 *     unrelated binding -> remains unaffected
 *
 * The grammar itself remains unchanged unless the source syntax changes.
 *
 * ============================================================================
 * 46. TEST CONTRACT — NESTED EXPANSION
 * ============================================================================
 *
 * Test:
 *
 *     macro A
 *       -> invokes macro B
 *          -> generates binding
 *
 * and verify that hygiene identities remain distinct and provenance remains
 * recoverable.
 *
 * ============================================================================
 * 47. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must contain no:
 *
 *     MAX_*
 *     MIN_*
 *     fixed qubit count
 *     fixed CPU count
 *     fixed GPU count
 *     fixed FPGA count
 *     fixed node count
 *     fixed memory size
 *     fixed topology
 *     fixed device ID
 *     fixed backend ID
 *     fixed hardware address
 *
 * Any finite value appearing in tests is test infrastructure only.
 *
 * ============================================================================
 * 48. SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Generated Zamani compiler code must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must use safe Rust.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * 49. COMPLETION CRITERIA
 * ============================================================================
 *
 * `grammar/macros/hygiene.g4` is COMPLETE only when all of the following are
 * true:
 *
 *     [ ] File name matches grammar name: hygiene.g4 / hygiene.
 *
 *     [ ] It is a parser grammar.
 *
 *     [ ] `tokenVocab = ZamaniLexer` resolves in the canonical build.
 *
 *     [ ] It defines no lexer rules.
 *
 *     [ ] It defines no duplicate annotation grammar.
 *
 *     [ ] It defines no duplicate identifier grammar.
 *
 *     [ ] It defines no duplicate qualified-name grammar.
 *
 *     [ ] It defines no macro declaration grammar.
 *
 *     [ ] It defines no macro invocation grammar.
 *
 *     [ ] It introduces no speculative hygiene keywords.
 *
 *     [ ] It reuses the canonical `annotation` rule.
 *
 *     [ ] It introduces no hardware assumptions.
 *
 *     [ ] It introduces no quantum-machine assumptions.
 *
 *     [ ] It introduces no resource limits.
 *
 *     [ ] It introduces no runtime behavior.
 *
 *     [ ] It introduces no macro-expansion algorithm.
 *
 *     [ ] It preserves source/provenance integration requirements.
 *
 *     [ ] It integrates with declarations.g4.
 *
 *     [ ] It integrates with invocations.g4 without redefining invocation
 *         syntax.
 *
 *     [ ] It integrates with expansion.g4.
 *
 *     [ ] It is reachable through macros.g4.
 *
 *     [ ] Duplicate macro/hygiene definitions in Meta.g4 are reconciled.
 *
 *     [ ] Canonical annotation token naming is reconciled at lexer integration.
 *
 *     [ ] Positive parser tests pass.
 *
 *     [ ] Negative parser tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Round-trip tests pass where supported.
 *
 *     [ ] Provenance tests pass.
 *
 *     [ ] Safe-Rust generation/build tests pass under Rust 1.97/1.97.1.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file establishes:
 *
 *     annotation syntax
 *         !=
 *     hygiene semantics
 *
 * and:
 *
 *     macro syntax
 *         !=
 *     macro expansion
 *
 * and:
 *
 *     source binding
 *         !=
 *     physical resource identity
 *
 * and:
 *
 *     language semantics
 *         !=
 *     target hardware.
 *
 * Therefore macro hygiene remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * and with Zamani's larger principle:
 *
 *     one program
 *         -> one semantic meaning
 *         -> many targets
 *         -> many architectures
 *         -> many scales
 *         -> many execution environments
 *
 * ============================================================================
 */