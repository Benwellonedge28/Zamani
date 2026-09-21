/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/modules/visibility.g4
 *
 * GRAMMAR
 * -------
 * Visibility
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION VISIBILITY PARSER COMPONENT
 *
 * PURPOSE
 * -------
 * This file is the single canonical parser owner for source-level visibility
 * syntax in Zamani.
 *
 * It is deliberately domain-neutral and may be consumed by:
 *
 *   - modules
 *   - packages
 *   - functions
 *   - declarations
 *   - types
 *   - structs
 *   - records
 *   - enums
 *   - classes
 *   - interfaces
 *   - traits
 *   - implementations
 *   - classical declarations
 *   - quantum declarations
 *   - hybrid declarations
 *   - HDL declarations
 *   - hardware declarations
 *   - distributed declarations
 *   - AI/data declarations
 *   - networking declarations
 *   - future Zamani domains
 *
 * ARCHITECTURAL RULE
 * ------------------
 * Visibility answers only:
 *
 *     "What source-level access modifier was written?"
 *
 * It does NOT answer:
 *
 *     "Who is allowed to access this declaration?"
 *
 * Access checking, module resolution, package resolution, inheritance,
 * export policy, ABI visibility, interoperability, and target realization
 * belong to semantic/compiler layers downstream.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file owns:
 *
 *     visibilityModifier
 *
 * and the compatibility wrappers explicitly defined below.
 *
 * It does NOT own:
 *
 *     identifiers
 *     qualified names
 *     modules
 *     packages
 *     exports
 *     imports
 *     declarations
 *     functions
 *     types
 *     expressions
 *     statements
 *     semantic access checking
 *     symbol resolution
 *     package resolution
 *     dependency resolution
 *     ABI generation
 *     IR construction
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     optimization
 *     hardware discovery
 *     resource discovery
 *     runtime dispatch
 *
 * No other grammar component should redefine the visibility alternatives.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The repository's current canonical lexical composition uses:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and its imported lexical components.
 *
 * The current keyword vocabulary defines the visibility tokens:
 *
 *     PUB
 *     PUBLIC
 *     PRIVATE
 *     PROTECTED
 *     INTERNAL
 *
 * Therefore this parser grammar MUST consume:
 *
 *     ZamaniLexer
 *
 * and MUST NOT reference an obsolete:
 *
 *     ZamaniTokens
 *
 * vocabulary.
 *
 * This is an intentional correction of the previous version of this file.
 *
 * ============================================================================
 * SOURCE-COMPATIBLE VISIBILITY SPELLINGS
 * ============================================================================
 *
 * The existing Zamani language surface contains:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * All five spellings remain accepted.
 *
 * `pub` and `public` are intentionally separate lexical spellings.
 *
 * Semantic analysis may canonicalize them to the same semantic visibility
 * value if the language specification defines them as aliases.
 *
 * The grammar preserves the source spelling through the selected token.
 *
 * ============================================================================
 * SINGLE OWNERSHIP
 * ============================================================================
 *
 * There is exactly one canonical visibility production:
 *
 *     visibilityModifier
 *
 * Other grammar components MUST compose this rule.
 *
 * They MUST NOT recreate alternatives such as:
 *
 *     moduleVisibility
 *     functionVisibility
 *     declarationVisibility
 *     typeVisibility
 *     packageVisibility
 *     quantumVisibility
 *     hardwareVisibility
 *
 * merely to repeat:
 *
 *     PUB
 *     PUBLIC
 *     PRIVATE
 *     PROTECTED
 *     INTERNAL
 *
 * Context-specific wrapper rules are acceptable only when they delegate to:
 *
 *     visibilityModifier
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The grammar establishes syntax only.
 *
 * Semantic analysis determines:
 *
 *     - the declaration's effective visibility;
 *     - the declaration context in which the modifier is valid;
 *     - default visibility when the modifier is absent;
 *     - whether protected visibility is meaningful for the declaration;
 *     - module-boundary accessibility;
 *     - package-boundary accessibility;
 *     - inheritance-related access;
 *     - export/visibility consistency;
 *     - import/access consistency;
 *     - symbol reachability;
 *     - ABI implications;
 *     - interoperability implications;
 *     - generated-artifact visibility.
 *
 * The parser MUST NOT perform those checks.
 *
 * ============================================================================
 * DEFAULT VISIBILITY
 * ============================================================================
 *
 * Absence of visibility is represented by the consuming grammar simply not
 * invoking:
 *
 *     visibilityModifier
 *
 * This file deliberately does NOT define the semantic default.
 *
 * In particular, this file does not decide:
 *
 *     absent = private
 *     absent = public
 *     absent = internal
 *
 * Such a rule belongs to the language semantic specification and declaration
 * context.
 *
 * ============================================================================
 * CARDINALITY
 * ============================================================================
 *
 * A visibility-bearing declaration accepts at most one visibility modifier
 * through this component.
 *
 * Consumers should normally use:
 *
 *     visibilityModifier?
 *
 * NOT:
 *
 *     visibilityModifier*
 *
 * and MUST NOT silently accept:
 *
 *     pub private
 *     public internal
 *     protected private
 *
 * as a sequence of visibility modifiers.
 *
 * If Zamani later introduces compound access policies, that must be a separate
 * explicitly specified construct. It must not be created accidentally by
 * changing this rule to accept arbitrary repetition.
 *
 * ============================================================================
 * OPEN-WORLD / FUTURE DOMAIN CONTRACT
 * ============================================================================
 *
 * Visibility is intentionally independent of computational domain.
 *
 * A future domain does not require a new visibility grammar.
 *
 * Examples:
 *
 *     pub classical_function
 *     pub quantum_operation
 *     pub hardware_interface
 *     pub distributed_service
 *     pub ai_model
 *     pub network_protocol
 *     pub future_domain_declaration
 *
 * all reuse:
 *
 *     visibilityModifier
 *
 * This is necessary for the open-world Zamani architecture.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Visibility MUST NOT encode physical realization.
 *
 * It MUST NOT contain:
 *
 *     CPU counts
 *     core counts
 *     thread counts
 *     GPU counts
 *     FPGA counts
 *     ASIC counts
 *     QPU counts
 *     qubit counts
 *     memory capacities
 *     register widths
 *     vector widths
 *     tensor dimensions
 *     accelerator counts
 *     node counts
 *     cluster sizes
 *     network topology
 *     physical device IDs
 *     hardware addresses
 *     backend identifiers
 *     vendor identifiers
 *     deployment sizes
 *
 * Visibility therefore has identical source-level syntax whether the program
 * eventually executes on:
 *
 *     - a tiny embedded target;
 *     - one CPU;
 *     - many CPUs;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a quantum simulator;
 *     - a heterogeneous machine;
 *     - a cluster;
 *     - an HPC system;
 *     - a distributed deployment;
 *     - a cloud environment;
 *     - a future computational architecture.
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no language-level finite limit on:
 *
 *     - declarations;
 *     - modules;
 *     - packages;
 *     - visibility-bearing declarations;
 *     - module depth;
 *     - package depth;
 *     - source size;
 *     - dependency-graph size;
 *     - computational-domain count.
 *
 * The grammar contains no:
 *
 *     MAX_VISIBILITY
 *     MAX_DECLARATIONS
 *     MAX_MODULES
 *     MAX_PACKAGES
 *     MAX_DEPTH
 *     MAX_DOMAINS
 *
 * Any implementation resource limits used to protect the compiler from
 * hostile or exhausted input are operational policies and MUST NOT redefine
 * the language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar has no semantic predicates or executable actions.
 *
 * Parsing depends only on:
 *
 *     - the token stream;
 *     - the grammar;
 *     - the imported vocabulary;
 *     - the parser configuration.
 *
 * It performs no:
 *
 *     filesystem I/O
 *     network I/O
 *     environment inspection
 *     clock access
 *     randomness
 *     package lookup
 *     registry lookup
 *     hardware discovery
 *     resource discovery
 *     backend discovery
 *
 * Identical token streams therefore produce equivalent visibility parse
 * structures under the same grammar/parser configuration.
 *
 * ============================================================================
 * SOURCE SPAN / AST CONTRACT
 * ============================================================================
 *
 * This grammar does not construct the AST.
 *
 * The frontend AST adapter MUST preserve enough information to represent:
 *
 *     - whether visibility was explicitly written;
 *     - which visibility token was written;
 *     - its source span;
 *     - source ordering relative to other declaration modifiers.
 *
 * Conceptual representation:
 *
 *     VisibilitySyntax
 *     {
 *         spelling,
 *         span
 *     }
 *
 * The concrete Rust AST type remains owned by the frontend AST implementation.
 *
 * The grammar MUST NOT introduce a competing AST type.
 *
 * ============================================================================
 * SEMANTIC LOWERING CONTRACT
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Visibility
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic visibility/access model
 *       |
 *       v
 *     module/package/symbol analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> accelerator representation
 *       |
 *       v
 *     optimization / lowering / routing / scheduling
 *       |
 *       v
 *     target realization
 *
 * Visibility MUST NOT bypass the semantic layer.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum declarations may consume:
 *
 *     visibilityModifier?
 *
 * where the relevant declaration grammar permits it.
 *
 * Examples include:
 *
 *     public quantum declarations
 *     private quantum abstractions
 *     internal quantum operations
 *
 * This grammar does NOT know about:
 *
 *     QubitId
 *     PhysicalQubitId
 *     quantum::ir
 *     gates
 *     circuits
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     calibration
 *     QPU topology
 *     backend selection
 *
 * Visibility therefore remains independent of quantum-machine scale.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware declarations may consume:
 *
 *     visibilityModifier?
 *
 * where their declaration contracts permit it.
 *
 * Visibility means source/API access.
 *
 * It does NOT mean:
 *
 *     allocate a device
 *     select a device
 *     expose a physical address
 *     expose a bus
 *     expose a memory bank
 *     select a processor
 *     select an accelerator
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Visibility is independent from:
 *
 *     requirement
 *     constraint
 *     capability
 *     resource
 *     preference
 *     hint
 *     target
 *     deployment
 *
 * For example:
 *
 *     pub fn compute(...) ...
 *
 * does not imply any hardware capability.
 *
 * Conversely:
 *
 *     requires capability("quantum.measurement")
 *
 * does not imply any visibility property.
 *
 * These are separate language concepts and must remain separately owned.
 *
 * ============================================================================
 * MODULE INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/modules.g4` is responsible for module declaration syntax.
 *
 * It MUST consume:
 *
 *     visibilityModifier?
 *
 * when module visibility is part of the language specification.
 *
 * It MUST NOT define another list of visibility tokens.
 *
 * Example composition:
 *
 *     moduleDeclaration
 *         : moduleAttributes?
 *           visibilityModifier?
 *           MODULE
 *           moduleName
 *           moduleBody
 *         ;
 *
 * The exact surrounding declaration order remains owned by `Modules`.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/functions.g4` owns function declaration syntax.
 *
 * It MUST consume the canonical visibility rule rather than repeating:
 *
 *     PUBLIC
 *     PUB
 *     PRIVATE
 *     PROTECTED
 *     INTERNAL
 *
 * Example:
 *
 *     functionSignatureCore
 *         : attribute*
 *           visibilityModifier?
 *           functionSpecificModifier*
 *           FN
 *           ...
 *         ;
 *
 * The exact function modifier ordering remains owned by `Functions`.
 *
 * ============================================================================
 * MODIFIER INTEGRATION
 * ============================================================================
 *
 * `grammar/core/modifiers.g4` is a broader modifier grammar.
 *
 * It MUST NOT become a second owner of the visibility vocabulary.
 *
 * Its canonical visibility branch should delegate to:
 *
 *     visibilityModifier
 *
 * or the aggregate parser should ensure only one visibility owner is used.
 *
 * In particular, the old pattern:
 *
 *     visibilityModifier
 *         : K_PUB
 *         | K_PUBLIC
 *         | K_PRIVATE
 *         | K_PROTECTED
 *         | K_INTERNAL
 *         ;
 *
 * is invalid for the current repository because those token names do not match
 * the current lexer vocabulary.
 *
 * The canonical token names are:
 *
 *     PUB
 *     PUBLIC
 *     PRIVATE
 *     PROTECTED
 *     INTERNAL
 *
 * ============================================================================
 * DECLARATION INTEGRATION
 * ============================================================================
 *
 * Declaration grammars should consume:
 *
 *     visibilityModifier?
 *
 * at their declaration boundary where visibility is specified.
 *
 * This includes, where permitted by the language specification:
 *
 *     type
 *     alias
 *     struct
 *     record
 *     enum
 *     union
 *     class
 *     interface
 *     trait
 *     implementation
 *     constant
 *     variable
 *     resource
 *     capability
 *     domain
 *
 * The declaration grammar remains responsible for deciding whether visibility
 * is syntactically allowed at that declaration site.
 *
 * This file owns only the vocabulary.
 *
 * ============================================================================
 * PACKAGE / NAMESPACE INTEGRATION
 * ============================================================================
 *
 * Package and namespace grammars may consume:
 *
 *     visibilityModifier?
 *
 * only where the language specification explicitly permits package/namespace
 * visibility.
 *
 * They MUST NOT copy the token alternatives.
 *
 * Whether `protected` is semantically meaningful for a package or namespace is
 * a semantic question.
 *
 * The parser component remains reusable without embedding that policy.
 *
 * ============================================================================
 * IMPORT / EXPORT INTEGRATION
 * ============================================================================
 *
 * Visibility and export are different concepts.
 *
 * Visibility answers:
 *
 *     who can access the declaration?
 *
 * Export answers:
 *
 *     which declaration is intentionally exposed through an API/module
 *     boundary?
 *
 * Therefore:
 *
 *     export
 *
 * MUST NOT be made an alias of:
 *
 *     public
 *
 * inside this grammar.
 *
 * `grammar/modules/exports.g4` owns export syntax.
 *
 * `grammar/modules/imports.g4` owns import syntax.
 *
 * Semantic analysis may later check consistency between:
 *
 *     visibility
 *     export
 *     import
 *     module reachability
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The following source spellings remain accepted:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * This replacement changes the parser-vocabulary integration from the stale:
 *
 *     ZamaniTokens
 *     K_PUB
 *     K_PUBLIC
 *     K_PRIVATE
 *     K_PROTECTED
 *     K_INTERNAL
 *
 * to the repository's current lexer contract:
 *
 *     ZamaniLexer
 *     PUB
 *     PUBLIC
 *     PRIVATE
 *     PROTECTED
 *     INTERNAL
 *
 * This is an integration correction, not a source-language visibility
 * breaking change.
 *
 * ============================================================================
 * COMPATIBILITY WRAPPERS
 * ============================================================================
 *
 * The following wrappers are intentionally thin.
 *
 * They do NOT introduce new vocabulary.
 *
 * They exist only where an existing consumer needs a named visibility boundary.
 *
 * `optionalVisibilityModifier` is provided as the canonical optional wrapper.
 *
 * It is equivalent structurally to:
 *
 *     visibilityModifier?
 *
 * but does not establish a semantic default.
 *
 * `visibility` is retained as a compatibility entry point for tools or grammar
 * consumers that previously referred to a generic visibility rule.
 *
 * Both wrappers delegate directly to `visibilityModifier`.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntactic errors include cases where a consumer expects:
 *
 *     visibilityModifier?
 *
 * but receives more than one visibility token before the declaration grammar
 * can continue.
 *
 * Examples:
 *
 *     pub private fn ...
 *     public internal fn ...
 *     protected private type ...
 *
 * The semantic layer, not this grammar, reports context-specific errors such
 * as:
 *
 *     protected is not permitted for this declaration kind
 *
 * or:
 *
 *     declaration is not reachable from this module
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - executes no source code;
 *     - reads no environment variables;
 *     - reads no credentials;
 *     - discovers no hardware;
 *     - discovers no resources;
 *     - allocates no devices;
 *     - performs no package resolution;
 *     - performs no registry resolution.
 *
 * Generated parser/compiler integration must remain safe Rust.
 *
 * The Rust implementation MUST NOT require `unsafe`.
 *
 * ============================================================================
 * RUST 1.97 / 1.97.1 CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * It is therefore independent of Rust memory representation.
 *
 * Generated Zamani parser/frontend code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and repository Rust crates should enforce:
 *
 *     #![deny(unsafe_code)]
 *
 * or an equivalent workspace-wide unsafe-code prohibition.
 *
 * This grammar must never require an unsafe Rust implementation.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The grammar component itself requires the following conformance classes.
 *
 * POSITIVE VISIBILITY TESTS
 * -------------------------
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * INTEGRATION POSITIVE TESTS
 * --------------------------
 *
 *     pub fn example() {}
 *     public fn example() {}
 *     private fn example() {}
 *     protected fn example() {}
 *     internal fn example() {}
 *
 * and equivalent valid declaration forms supplied by the owning declaration
 * grammars.
 *
 * NEGATIVE INTEGRATION TESTS
 * --------------------------
 *
 * Where a declaration expects at most one visibility:
 *
 *     pub private fn example() {}
 *     public internal fn example() {}
 *     private protected fn example() {}
 *     protected public fn example() {}
 *
 * These must not be accepted as two visibility modifiers by this component.
 *
 * SEMANTIC NEGATIVE TESTS
 * -----------------------
 *
 * These belong outside this grammar:
 *
 *     protected used on a declaration that forbids it;
 *     inaccessible private symbol referenced externally;
 *     conflicting export/visibility policy;
 *     visibility violating module/package policy.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Verify reuse without modification by:
 *
 *     classical declarations
 *     quantum declarations
 *     hybrid declarations
 *     HDL declarations
 *     hardware declarations
 *     distributed declarations
 *     AI/data declarations
 *     networking declarations
 *     future dialect declarations
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Verify that visibility syntax remains unchanged as the surrounding program
 * scales in:
 *
 *     declaration count
 *     module count
 *     package count
 *     namespace depth
 *     dependency-graph size
 *     computational-domain count
 *     source size
 *
 * No artificial maximum may be introduced by this grammar.
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Repeated parsing of identical token streams must produce equivalent parse
 * structures.
 *
 * ROUND-TRIP TESTS
 * ----------------
 *
 * When the repository's canonical source printer is available:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> lexer
 *       -> parser
 *
 * must preserve visibility intent.
 *
 * SOURCE-PROVENANCE TESTS
 * -----------------------
 *
 * The frontend must be able to distinguish source spellings when required:
 *
 *     pub
 *     public
 *
 * even if semantic analysis later canonicalizes them to the same semantic
 * visibility.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_VISIBILITY
 *     MAX_DECLARATIONS
 *     MAX_MODULES
 *     MAX_PACKAGES
 *     MAX_DOMAINS
 *     fixed CPU counts
 *     fixed GPU counts
 *     fixed FPGA counts
 *     fixed QPU counts
 *     fixed qubit counts
 *     fixed node counts
 *     fixed accelerator counts
 *     device IDs
 *     physical addresses
 *     topology assumptions
 *     backend names
 *     vendor names
 *     filesystem paths
 *     registry endpoints
 *     network endpoints
 *
 * The only finite list in this file is the language's currently specified
 * visibility vocabulary.
 *
 * That finite vocabulary is a language-semantic choice, not a machine-resource
 * limit.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] The filename remains grammar/modules/visibility.g4.
 *     [x] The grammar name remains Visibility.
 *     [x] The file is a parser grammar.
 *     [x] The current ZamaniLexer vocabulary is consumed.
 *     [x] The current token names are used.
 *     [x] No obsolete ZamaniTokens dependency remains.
 *     [x] No K_* token names are referenced.
 *     [x] One canonical visibility production exists.
 *     [x] All five existing visibility spellings remain accepted.
 *     [x] `pub` and `public` remain source-distinguishable.
 *     [x] Absence remains distinct from explicit visibility.
 *     [x] No semantic default is encoded.
 *     [x] No arbitrary repeated visibility modifiers are accepted through the
 *         canonical rule.
 *     [x] No declaration grammar is duplicated here.
 *     [x] No module resolution occurs here.
 *     [x] No package resolution occurs here.
 *     [x] No export semantics occur here.
 *     [x] No import semantics occur here.
 *     [x] No AST is constructed here.
 *     [x] No semantic model is constructed here.
 *     [x] No IR is constructed here.
 *     [x] No quantum-specific IR is constructed here.
 *     [x] No QEC/ZQN/routing/scheduling logic is present.
 *     [x] No hardware topology is represented.
 *     [x] No resource capacity is represented.
 *     [x] No target/backend is selected.
 *     [x] No machine-size limit exists.
 *     [x] No filesystem/network/environment access exists.
 *     [x] No Rust code exists in the grammar.
 *     [x] No unsafe Rust is required.
 *     [x] The component is deterministic.
 *     [x] The component is domain-neutral.
 *     [x] The component is reusable by future domains.
 *     [x] Positive tests are specified.
 *     [x] Negative tests are specified.
 *     [x] Boundary tests are specified.
 *     [x] Scalability tests are specified.
 *     [x] Determinism tests are specified.
 *     [x] Round-trip tests are specified.
 *     [x] Source-provenance requirements are specified.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST FOR OTHER FILES
 * ============================================================================
 *
 * This file itself is independently complete once the rules below are the
 * canonical contract.
 *
 * Other files must integrate against it as follows:
 *
 *     grammar/modules/modules.g4
 *         -> visibilityModifier?
 *
 *     grammar/functions/functions.g4
 *         -> visibilityModifier?
 *
 *     declaration grammars
 *         -> visibilityModifier?
 *
 *     grammar/core/modifiers.g4
 *         -> delegate to visibilityModifier
 *
 *     grammar/modules/exports.g4
 *         -> remain independent
 *
 *     grammar/modules/imports.g4
 *         -> remain independent
 *
 * No downstream file should redefine the five token alternatives.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * Visibility is a source-level access property.
 *
 * It is not:
 *
 *     a hardware property;
 *     a resource property;
 *     a deployment property;
 *     a backend property;
 *     a quantum property;
 *     an HDL property;
 *     a scheduling property;
 *     a routing property.
 *
 * Therefore the same visibility syntax can survive the complete Zamani
 * compilation path:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> semantic analysis
 *       -> canonical semantic model
 *       -> IR
 *       -> optimization
 *       -> routing
 *       -> scheduling
 *       -> resilience / QEC / ZQN where applicable
 *       -> HAL
 *       -> target
 *
 * without changing merely because the available machine grows from tiny to
 * arbitrarily large or changes computational architecture.
 *
 * ============================================================================
 */

parser grammar Visibility;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL VISIBILITY PRODUCTION
 * ============================================================================
 *
 * Exactly one source-level visibility modifier.
 *
 * IMPORTANT:
 *
 * These token names are the names actually exported by the repository's
 * canonical Zamani lexer vocabulary:
 *
 *     PUB
 *     PUBLIC
 *     PRIVATE
 *     PROTECTED
 *     INTERNAL
 *
 * Do not rename them here to K_* aliases.
 */
visibilityModifier
    : PUB
    | PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/*
 * ============================================================================
 * OPTIONAL VISIBILITY INTEGRATION POINT
 * ============================================================================
 *
 * Consumers that need optional visibility may use this rule:
 *
 *     optionalVisibilityModifier
 *
 * This is intentionally equivalent to:
 *
 *     visibilityModifier?
 *
 * It does not define a semantic default.
 */
optionalVisibilityModifier
    : visibilityModifier?
    ;


/*
 * ============================================================================
 * COMPATIBILITY ENTRY POINT
 * ============================================================================
 *
 * `visibility` is a thin compatibility alias for tools or consumers that
 * previously referred to a generic visibility rule.
 *
 * It MUST NOT become a second vocabulary.
 */
visibility
    : visibilityModifier
    ;