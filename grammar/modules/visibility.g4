/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/visibility.g4
 *
 * Role:
 *     Canonical parser component for source-level visibility/access
 *     declarations.
 *
 * Grammar layer:
 *     Concrete syntax only.
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     This file contains ANTLR grammar only.
 *     It contains no Rust implementation code.
 *     Generated/compiler/runtime code MUST use safe Rust only.
 *     The repository Rust crates MUST enforce unsafe-code prohibition.
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * Visibility is a LANGUAGE-LEVEL declaration property.
 *
 * This file provides one canonical syntax vocabulary that can be consumed by:
 *
 *     modules
 *     packages
 *     functions
 *     declarations
 *     types
 *     implementations
 *     interfaces
 *     traits
 *     foreign declarations
 *     hardware declarations
 *     quantum declarations
 *     classical declarations
 *     distributed declarations
 *     future language domains
 *
 * Visibility syntax is deliberately independent of the computational domain.
 *
 * For example, the same visibility syntax can apply to:
 *
 *     classical::value
 *     quantum::operation
 *     hdl::module
 *     hardware::interface
 *     distributed::service
 *     ai::model
 *
 * without visibility.g4 needing to know what any of those things mean.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the canonical visibility modifier;
 *   - the canonical visibility modifier sequence;
 *   - visibility alternatives;
 *   - optional/default visibility syntax where defined by the language;
 *   - the syntactic representation of visibility source spans;
 *   - the syntax-level distinction between explicit and absent visibility.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - lexical tokens;
 *   - declarations;
 *   - modules;
 *   - packages;
 *   - functions;
 *   - types;
 *   - structs;
 *   - enums;
 *   - traits;
 *   - interfaces;
 *   - implementations;
 *   - expressions;
 *   - statements;
 *   - effects;
 *   - capabilities;
 *   - resources;
 *   - hardware;
 *   - quantum semantics;
 *   - classical semantics;
 *   - HDL semantics;
 *   - module resolution;
 *   - package resolution;
 *   - symbol tables;
 *   - name lookup;
 *   - access checking;
 *   - inheritance;
 *   - ABI;
 *   - IR construction;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - hardware discovery;
 *   - resource discovery;
 *   - runtime dispatch;
 *   - filesystem access;
 *   - network access;
 *   - package registries;
 *   - dependency resolution.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * Visibility MUST have exactly one canonical grammar owner.
 *
 * Other grammar components MUST NOT redefine:
 *
 *     visibilityModifier
 *
 * or create competing copies such as:
 *
 *     moduleVisibility
 *     packageVisibility
 *     functionVisibility
 *     typeVisibility
 *     declarationVisibility
 *
 * when those rules merely duplicate the same visibility vocabulary.
 *
 * Domain-specific grammar files MAY define a contextual wrapper around the
 * canonical visibility rule when the surrounding declaration requires it.
 *
 * Such wrappers MUST delegate to:
 *
 *     visibilityModifier
 *
 * rather than duplicate the alternatives.
 *
 * Example:
 *
 *     functionDeclaration
 *         : visibilityModifier?
 *           K_FN
 *           ...
 *         ;
 *
 * This keeps the visibility vocabulary centralized.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar answers only:
 *
 *     "What visibility syntax was written?"
 *
 * It does NOT answer:
 *
 *     "Is this declaration actually accessible?"
 *
 * Semantic analysis determines:
 *
 *   - whether a visibility modifier is legal in its declaration context;
 *   - whether multiple visibility modifiers conflict;
 *   - whether a declaration has a default visibility;
 *   - whether public/private/protected/internal are meaningful for that
 *     declaration kind;
 *   - whether a referenced symbol is accessible;
 *   - whether inheritance affects protected visibility;
 *   - whether module boundaries affect visibility;
 *   - whether package boundaries affect visibility;
 *   - whether an exported declaration is actually public;
 *   - whether a visibility rule conflicts with another language rule;
 *   - whether visibility affects ABI or interoperability;
 *   - whether visibility affects generated artifacts.
 *
 * The parser MUST NOT perform any of those checks.
 *
 * ============================================================================
 * POCO-REAF / HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * Visibility has no relationship to physical resources.
 *
 * It MUST NOT encode:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     device identifiers
 *     device addresses
 *     topology
 *     network size
 *     cluster size
 *     accelerator count
 *     deployment size
 *
 * Therefore visibility syntax remains identical whether a program executes on:
 *
 *     a tiny embedded system
 *     one CPU
 *     many CPUs
 *     a GPU
 *     an FPGA
 *     an ASIC
 *     a quantum processor
 *     a simulator
 *     a heterogeneous system
 *     a cluster
 *     a supercomputer
 *     a distributed deployment
 *     a future architecture
 *
 * This is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite language-level limit is encoded for:
 *
 *     - number of declarations;
 *     - number of modules;
 *     - number of packages;
 *     - number of visibility-bearing declarations;
 *     - source-file size;
 *     - module-graph size;
 *     - dependency-graph size;
 *     - computational-domain size.
 *
 * This file MUST NOT contain:
 *
 *     MAX_VISIBILITY
 *     MAX_DECLARATIONS
 *     MAX_MODULES
 *     MAX_PACKAGES
 *     MAX_DEPTH
 *
 * Repetition and source size are bounded only by explicit implementation
 * resource policies.
 *
 * Such operational limits MUST NOT redefine language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem I/O
 *     network I/O
 *     environment inspection
 *     clock access
 *     randomness
 *     package lookup
 *     registry lookup
 *     hardware discovery
 *     backend discovery
 *     resource discovery
 *
 * Therefore identical token streams produce identical syntactic structures.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * Grammar:
 *
 *     ZamaniTokens
 *
 * Required tokens:
 *
 *     K_PUB
 *     K_PUBLIC
 *     K_PRIVATE
 *     K_PROTECTED
 *     K_INTERNAL
 *
 * This file MUST NOT declare lexer rules.
 *
 * ============================================================================
 * VISIBILITY VOCABULARY
 * ============================================================================
 *
 * The current canonical lexical vocabulary provides:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * This grammar intentionally does not introduce additional visibility
 * keywords.
 *
 * New visibility concepts require an explicit language-version/specification
 * change before being added to the lexer and this grammar.
 *
 * ============================================================================
 * SYNTACTIC MODEL
 * ============================================================================
 *
 * The canonical visibility syntax is:
 *
 *     visibilityModifier
 *
 * Examples:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * A declaration may then consume:
 *
 *     visibilityModifier?
 *
 * when absence means "use the contextual default".
 *
 * This file does NOT define what that default is.
 *
 * ============================================================================
 * ALIASING OF VOCABULARY
 * ============================================================================
 *
 * `pub` and `public` are separate lexical spellings.
 *
 * The parser preserves which spelling was written.
 *
 * Semantic analysis MAY canonicalize both into one semantic visibility
 * representation, but that canonicalization MUST occur outside this grammar.
 *
 * Likewise:
 *
 *     private
 *     protected
 *     internal
 *
 * remain distinct source-level visibility modifiers.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST SHOULD preserve:
 *
 *     VisibilitySyntax {
 *         kind,
 *         span
 *     }
 *
 * where `kind` identifies the source spelling/category.
 *
 * The exact Rust AST type is owned by the frontend implementation and MUST NOT
 * be defined by this grammar.
 *
 * At minimum, the AST must retain enough information for semantic analysis to
 * distinguish:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *     absent
 *
 * when source provenance requires that distinction.
 *
 * If the semantic model intentionally treats `pub` and `public` identically,
 * the semantic layer may canonicalize them after preserving source provenance.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic layer owns the mapping from source syntax to semantic access
 * policy.
 *
 * Conceptually:
 *
 *     VisibilitySyntax
 *             |
 *             v
 *     visibility semantic analysis
 *             |
 *             v
 *     canonical visibility model
 *
 * This grammar MUST NOT create that semantic model.
 *
 * ============================================================================
 * MODULE INTEGRATION
 * ============================================================================
 *
 * `modules/modules.g4` currently contains a local visibility production.
 *
 * That production MUST be migrated to this canonical rule.
 *
 * Before:
 *
 *     moduleVisibility
 *         : K_PUB
 *         | K_PUBLIC
 *         | K_PRIVATE
 *         | K_PROTECTED
 *         | K_INTERNAL
 *         ;
 *
 * After:
 *
 *     moduleDeclaration
 *         : visibilityModifier?
 *           K_MODULE
 *           ...
 *         ;
 *
 * The module grammar then consumes the canonical visibility vocabulary without
 * owning it.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * `functions/functions.g4` currently contains:
 *
 *     functionModifier
 *         : K_PUB
 *         | K_PUBLIC
 *         | K_PRIVATE
 *         | K_PROTECTED
 *         | K_INTERNAL
 *         | ...
 *         ;
 *
 * The visibility alternatives MUST be removed from that local ownership.
 *
 * Function modifiers should instead compose:
 *
 *     visibilityModifier?
 *
 * alongside function-specific modifiers.
 *
 * The function grammar continues to own:
 *
 *     static
 *     const
 *     async
 *     extern
 *     inline
 *     volatile
 *     final
 *     sealed
 *     partial
 *     virtual
 *     override
 *     abstract
 *
 * only when those are genuinely function-specific.
 *
 * ============================================================================
 * DECLARATION INTEGRATION
 * ============================================================================
 *
 * Declaration grammars should consume:
 *
 *     visibilityModifier?
 *
 * where visibility is valid.
 *
 * Examples:
 *
 *     struct
 *     enum
 *     trait
 *     interface
 *     type
 *     implementation
 *     constant
 *     variable
 *
 * Each declaration grammar remains responsible for deciding syntactically
 * where the visibility modifier may appear.
 *
 * This file remains responsible only for the modifier itself.
 *
 * ============================================================================
 * PACKAGE INTEGRATION
 * ============================================================================
 *
 * Package declarations MAY consume:
 *
 *     visibilityModifier?
 *
 * if the language specification permits package visibility.
 *
 * The package grammar MUST NOT duplicate:
 *
 *     K_PUB
 *     K_PUBLIC
 *     K_PRIVATE
 *     K_INTERNAL
 *
 * as an independent rule.
 *
 * Whether protected visibility is meaningful for packages is a semantic
 * question, not a reason to duplicate grammar vocabulary.
 *
 * ============================================================================
 * EXPORT INTEGRATION
 * ============================================================================
 *
 * Visibility and export are intentionally separate.
 *
 * For example:
 *
 *     pub fn compute() { ... }
 *
 * and:
 *
 *     export compute;
 *
 * are different source-level concepts.
 *
 * Visibility answers:
 *
 *     who may access a declaration?
 *
 * Export answers:
 *
 *     which declarations are intentionally exposed through a module API?
 *
 * `exports.g4` therefore MUST NOT import or duplicate this grammar merely to
 * parse `export`.
 *
 * Semantic analysis may later verify consistency between visibility and export
 * policy.
 *
 * ============================================================================
 * IMPORT INTEGRATION
 * ============================================================================
 *
 * `imports.g4` does not need visibility syntax merely to parse imports.
 *
 * Import resolution may later consult semantic visibility information.
 *
 * Therefore:
 *
 *     grammar/modules/imports.g4
 *
 * MUST depend semantically on visibility analysis, not make visibility part of
 * its own duplicated grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum grammar files may consume:
 *
 *     visibilityModifier?
 *
 * for declarations such as:
 *
 *     quantum functions
 *     quantum operations
 *     circuit declarations
 *     logical-qubit declarations
 *     quantum abstractions
 *
 * However this file MUST NOT know:
 *
 *     QubitId
 *     PhysicalQubitId
 *     quantum::ir
 *     QEC
 *     ZQN
 *     device topology
 *     backend
 *     calibration
 *     scheduling
 *
 * Visibility therefore remains independent of quantum-machine scale.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware grammar components may consume:
 *
 *     visibilityModifier?
 *
 * for declarations whose language specification permits visibility.
 *
 * This does NOT make visibility a hardware-access mechanism.
 *
 * For example:
 *
 *     pub hardware::module
 *
 * describes source-level API visibility.
 *
 * It does NOT mean:
 *
 *     publicly allocate hardware
 *     expose a device
 *     select a physical device
 *     expose an address
 *     expose a bus
 *
 * Those meanings belong to later semantic/target layers.
 *
 * ============================================================================
 * CLASSICAL / DISTRIBUTED / AI / FUTURE DOMAINS
 * ============================================================================
 *
 * No domain-specific visibility syntax is necessary.
 *
 * Future declarations should consume:
 *
 *     visibilityModifier
 *
 * rather than introducing:
 *
 *     quantumVisibility
 *     gpuVisibility
 *     clusterVisibility
 *     aiVisibility
 *     hardwareVisibility
 *
 * unless a genuinely different language semantic is introduced.
 *
 * This keeps the grammar extensible without continuously modifying the
 * visibility vocabulary as new computational domains appear.
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCIES
 * ============================================================================
 *
 * Dependency direction:
 *
 *     ZamaniTokens
 *          |
 *          v
 *     Visibility
 *          |
 *          +--> modules
 *          +--> declarations
 *          +--> functions
 *          +--> types
 *          +--> quantum
 *          +--> hardware
 *          +--> HDL
 *          +--> future domains
 *
 * Visibility MUST NOT depend on:
 *
 *     AST
 *     semantic analysis
 *     IR
 *     quantum::ir
 *     runtime
 *     hardware
 *     scheduling
 *     routing
 *     optimization
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar delegate.
 *
 * Canonical form:
 *
 *     parser grammar Visibility;
 *
 *     options {
 *         tokenVocab = ZamaniTokens;
 *     }
 *
 * Aggregate parser composition should import:
 *
 *     Visibility
 *
 * before or alongside the declaration grammars that consume
 * `visibilityModifier`.
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * MULTI-MODIFIER POLICY
 * ============================================================================
 *
 * This grammar deliberately provides:
 *
 *     visibilityModifier
 *
 * and:
 *
 *     optional visibilityModifier
 *
 * rather than accepting arbitrary repeated visibility modifiers.
 *
 * Therefore:
 *
 *     pub private fn ...
 *
 * MUST NOT be accepted merely because both tokens are individually valid.
 *
 * Such combinations are syntactically invalid at the canonical visibility
 * layer.
 *
 * If the language later requires compound access policies, that should be
 * introduced explicitly as a new language construct rather than accidentally
 * permitting ambiguous modifier combinations.
 *
 * ============================================================================
 * DEFAULT VISIBILITY
 * ============================================================================
 *
 * Absence of a visibility modifier is represented by:
 *
 *     no visibilityModifier
 *
 * This grammar intentionally does not encode:
 *
 *     default = private
 *
 *     default = public
 *
 *     default = internal
 *
 * or any other semantic default.
 *
 * The declaration context and language semantic specification own that rule.
 *
 * This avoids forcing every declaration domain to share an assumption that
 * may later prove incorrect.
 *
 * ============================================================================
 * SOURCE COMPATIBILITY
 * ============================================================================
 *
 * Existing accepted visibility spellings are:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * They MUST remain accepted unless the language-version/compatibility policy
 * explicitly changes them.
 *
 * Moving the alternatives from module/function-specific grammar files into
 * this canonical grammar is an ownership refactor, not a language-breaking
 * syntax change.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Visibility syntax contains no executable behavior.
 *
 * Parsing visibility MUST NOT:
 *
 *     access files;
 *     access networks;
 *     execute commands;
 *     inspect environment variables;
 *     access credentials;
 *     resolve packages;
 *     discover hardware;
 *     access devices;
 *     allocate resources.
 *
 * All such operations are outside the grammar.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The grammar should allow ANTLR to report ordinary parser diagnostics for
 * invalid visibility syntax.
 *
 * It MUST NOT embed target-specific error messages.
 *
 * Examples of syntactic errors include:
 *
 *     pub private
 *     public private
 *     protected internal
 *
 * when these occur where a declaration expects at most one visibility
 * modifier.
 *
 * Semantic diagnostics such as:
 *
 *     protected is not valid for this declaration
 *
 * belong to semantic analysis, not this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * Each must tokenize and parse through:
 *
 *     visibilityModifier
 *
 * POSITIVE DECLARATION INTEGRATION TESTS
 *
 *     pub fn example() {}
 *     public fn example() {}
 *     private fn example() {}
 *     protected fn example() {}
 *     internal fn example() {}
 *
 *     pub module example;
 *     private module example;
 *
 * The exact declaration forms must be tested in their owning declaration
 * grammar as well.
 *
 * NEGATIVE TESTS
 *
 *     pub public
 *     public private
 *     private protected
 *     protected internal
 *     internal pub
 *
 * when supplied where a single visibility modifier is expected.
 *
 * Also test malformed/incomplete forms:
 *
 *     pub fn
 *     public module
 *
 * through the aggregate parser.
 *
 * SEMANTIC NEGATIVE TESTS
 *
 * These MUST NOT be grammar tests:
 *
 *     protected used where declaration context forbids it
 *     inaccessible private symbol referenced externally
 *     conflicting export/visibility policy
 *
 * Those belong to semantic-analysis tests.
 *
 * CROSS-DOMAIN TESTS
 *
 * Visibility syntax must work without modification for declarations associated
 * with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     networking
 *     future dialects
 *
 * SCALABILITY TESTS
 *
 * Verify that there is no grammar-level limit associated with:
 *
 *     declaration count
 *     module count
 *     package count
 *     source size
 *     computational domain count
 *
 * DETERMINISM TESTS
 *
 * Parse identical token streams repeatedly and verify identical parse
 * structure.
 *
 * ROUND-TRIP TESTS
 *
 * Where the repository has a canonical source printer:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     printer
 *       ->
 *     parser
 *
 * must preserve the visibility intent.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * FORBIDDEN IN THIS FILE:
 *
 *     MAX_VISIBILITY
 *     MAX_DECLARATIONS
 *     MAX_MODULES
 *     MAX_PACKAGES
 *     MAX_DOMAINS
 *     fixed hardware counts
 *     fixed CPU counts
 *     fixed GPU counts
 *     fixed FPGA counts
 *     fixed ASIC counts
 *     fixed qubit counts
 *     device IDs
 *     device addresses
 *     topology assumptions
 *     backend names
 *     vendor names
 *     filesystem paths
 *     registry addresses
 *     network endpoints
 *
 * Visibility is entirely independent of machine resources.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no Rust.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The compiler/runtime crates MUST enforce:
 *
 *     #![deny(unsafe_code)]
 *
 * and MUST NOT introduce unsafe implementation requirements merely because
 * visibility syntax is parsed.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when ALL of the following are true:
 *
 *   1. `Visibility` compiles as an ANTLR parser grammar.
 *
 *   2. It uses only the canonical `ZamaniTokens` vocabulary.
 *
 *   3. It defines exactly one canonical visibility production.
 *
 *   4. Existing visibility spellings remain source-compatible:
 *
 *          pub
 *          public
 *          private
 *          protected
 *          internal
 *
 *   5. It accepts one visibility modifier where the consuming declaration
 *      permits visibility.
 *
 *   6. It does not accept arbitrary repeated visibility modifiers.
 *
 *   7. It does not define declaration-specific duplicate visibility rules.
 *
 *   8. `modules.g4` delegates to it.
 *
 *   9. `functions.g4` delegates to it.
 *
 *  10. Declaration/type/trait/interface grammars delegate to it wherever
 *      visibility is permitted.
 *
 *  11. `exports.g4` remains independent because export and visibility are
 *      distinct language concepts.
 *
 *  12. No AST construction occurs in this grammar.
 *
 *  13. No semantic visibility checking occurs in this grammar.
 *
 *  14. No module resolution occurs in this grammar.
 *
 *  15. No package resolution occurs in this grammar.
 *
 *  16. No filesystem or network access occurs in this grammar.
 *
 *  17. No hardware discovery occurs in this grammar.
 *
 *  18. No quantum backend or resource knowledge occurs in this grammar.
 *
 *  19. No machine-size limit is encoded.
 *
 *  20. No hard-coded computational-domain list is required.
 *
 *  21. New future computational domains can reuse the same visibility
 *      production without changing this file.
 *
 *  22. Parser behavior is deterministic.
 *
 *  23. Positive, negative, boundary, integration, determinism, and
 *      round-trip tests pass.
 *
 *  24. Rust 1.97/1.97.1 integration remains safe-Rust-only.
 *
 *  25. No downstream file needs to redefine the visibility vocabulary.
 *
 * ============================================================================
 */

parser grammar Visibility;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. CANONICAL VISIBILITY MODIFIER
 * ============================================================================
 *
 * Exactly one source-level visibility modifier.
 *
 * The semantic meaning is resolved downstream.
 */
visibilityModifier
    : K_PUB
    | K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;


/*
 * ============================================================================
 * 2. OPTIONAL VISIBILITY
 * ============================================================================
 *
 * Declaration grammars should normally use:
 *
 *     visibilityModifier?
 *
 * directly.
 *
 * This rule exists only as a named integration point for declaration families
 * that need an explicit optional-visibility production.
 *
 * It does not assign a semantic default.
 */
optionalVisibilityModifier
    : visibilityModifier
    |
    ;