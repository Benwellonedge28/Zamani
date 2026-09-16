/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/visibility.g4
 *
 * Status:
 *     Production-ready canonical parser component.
 *
 * Purpose:
 *     Defines the single reusable source-level visibility syntax contract
 *     shared by all Zamani declaration domains.
 *
 * Grammar layer:
 *     Parser / concrete syntax only.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     This file contains grammar only.
 *     No Rust `unsafe` code is permitted by the surrounding implementation.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file is the canonical owner of the Zamani visibility grammar rule:
 *
 *     visibilityModifier
 *
 * Other grammar components MUST consume this rule rather than redefine the
 * visibility alternatives.
 *
 * The lexical spelling is owned by:
 *
 *     grammar/lexer/tokens.g4
 *
 * using the canonical ZamaniTokens vocabulary.
 *
 * This file does NOT define lexer rules.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniTokens
 *       |
 *       v
 *     Visibility
 *       |
 *       +-----------------------------+
 *       |             |               |
 *       v             v               v
 *     modules     declarations     functions
 *       |             |               |
 *       +-------------+---------------+
 *                     |
 *                     v
 *                    AST
 *                     |
 *                     v
 *             semantic visibility
 *                     |
 *                     v
 *             canonical semantic model
 *                     |
 *                     v
 *                    IR
 *
 * Visibility is therefore upstream of:
 *
 *     semantic analysis
 *     symbol resolution
 *     type checking
 *     capability analysis
 *     resource analysis
 *     classical IR
 *     quantum::ir
 *     HDL/hardware IR
 *     optimization
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime
 *     deployment
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the canonical visibility modifier;
 *   - the source-level visibility alternatives;
 *   - the syntax of a single visibility modifier;
 *   - the distinction between an explicitly written modifier and absence;
 *   - the parser-level vocabulary shared by all declaration domains.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - identifiers;
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
 *   - namespaces;
 *   - imports;
 *   - exports;
 *   - symbol tables;
 *   - scope resolution;
 *   - access checking;
 *   - inheritance;
 *   - ABI;
 *   - linkage;
 *   - object-file visibility;
 *   - linker visibility;
 *   - dynamic-loader policy;
 *   - hardware;
 *   - quantum semantics;
 *   - classical semantics;
 *   - HDL semantics;
 *   - resource allocation;
 *   - capability discovery;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - optimization;
 *   - runtime behavior.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Visibility is a source-level semantic property.
 *
 * It MUST NOT depend on:
 *
 *   - CPU count;
 *   - CPU architecture;
 *   - core count;
 *   - thread count;
 *   - GPU count;
 *   - FPGA count;
 *   - accelerator count;
 *   - QPU count;
 *   - qubit count;
 *   - memory capacity;
 *   - storage capacity;
 *   - register width;
 *   - vector width;
 *   - node count;
 *   - cluster size;
 *   - network size;
 *   - topology;
 *   - device identifier;
 *   - physical address;
 *   - vendor;
 *   - calibration;
 *   - deployment location.
 *
 * Therefore the same visibility syntax is valid for:
 *
 *   - tiny systems;
 *   - embedded systems;
 *   - CPUs;
 *   - GPUs;
 *   - FPGAs;
 *   - ASICs;
 *   - quantum systems;
 *   - simulators;
 *   - heterogeneous systems;
 *   - distributed systems;
 *   - clusters;
 *   - supercomputers;
 *   - future computational architectures.
 *
 * This is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level limit on:
 *
 *   - number of declarations;
 *   - number of modules;
 *   - number of packages;
 *   - number of visibility-bearing declarations;
 *   - source-file size;
 *   - module-graph size;
 *   - dependency-graph size;
 *   - qualification depth;
 *   - computational-domain size.
 *
 * The grammar MUST NOT introduce:
 *
 *     MAX_VISIBILITY
 *     MAX_DECLARATIONS
 *     MAX_MODULES
 *     MAX_PACKAGES
 *     MAX_VISIBILITY_DEPTH
 *
 * Parser implementations may have configurable operational resource budgets.
 * Such budgets are implementation constraints and MUST NOT become language
 * semantics.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Canonical lexical owner:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical lexer:
 *
 *     ZamaniTokens
 *
 * Canonical visibility tokens:
 *
 *     K_PUB
 *     K_PUBLIC
 *     K_PRIVATE
 *     K_PROTECTED
 *     K_INTERNAL
 *
 * This file deliberately does not define lexer rules.
 *
 * The lexer owns spelling.
 * This parser grammar owns composition.
 * The semantic layer owns meaning.
 *
 * ============================================================================
 * VISIBILITY VOCABULARY
 * ============================================================================
 *
 * The currently established Zamani source spellings are:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * `pub` and `public` remain distinct source spellings even if the semantic
 * layer canonicalizes them to the same visibility category.
 *
 * The parser MUST preserve the token identity so source provenance and
 * diagnostics remain possible.
 *
 * ============================================================================
 * SINGLE-MODIFIER CONTRACT
 * ============================================================================
 *
 * This grammar deliberately accepts exactly ONE visibility modifier through:
 *
 *     visibilityModifier
 *
 * It does NOT accept:
 *
 *     pub private
 *     public internal
 *     protected private
 *     internal public
 *
 * merely because multiple modifiers happen to be syntactically possible.
 *
 * Conflicting visibility declarations therefore remain parser errors rather
 * than being silently accepted and resolved arbitrarily.
 *
 * ============================================================================
 * ABSENT VISIBILITY
 * ============================================================================
 *
 * This file intentionally does NOT define:
 *
 *     visibilityModifier?
 *
 * as part of the canonical rule itself.
 *
 * Instead, declaration grammars decide whether visibility is optional:
 *
 *     visibilityModifier?
 *     declaration
 *
 * or mandatory:
 *
 *     visibilityModifier
 *     declaration
 *
 * This distinction is important because the legal presence of visibility is
 * a property of the surrounding declaration syntax.
 *
 * The semantic meaning of omitted visibility is NOT owned by this file.
 *
 * ============================================================================
 * CANONICAL RULE
 * ============================================================================
 */

parser grammar Visibility;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * CANONICAL VISIBILITY MODIFIER
 * ============================================================================
 *
 * Exactly one source-level visibility modifier.
 *
 * Semantic normalization happens after parsing.
 *
 * The parser preserves the concrete spelling through the token selected by
 * this rule, allowing the AST/source-span layer to retain source provenance.
 *
 * ============================================================================
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
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Higher-level grammar components MUST compose this rule.
 *
 * Example:
 *
 *     functionDeclaration
 *         : visibilityModifier?
 *           K_FN
 *           identifier
 *           ...
 *         ;
 *
 * Example:
 *
 *     structDeclaration
 *         : visibilityModifier?
 *           K_STRUCT
 *           identifier
 *           ...
 *         ;
 *
 * Example:
 *
 *     enumDeclaration
 *         : visibilityModifier?
 *           K_ENUM
 *           identifier
 *           ...
 *         ;
 *
 * The higher-level grammar owns the placement.
 *
 * This file owns the vocabulary.
 *
 * ============================================================================
 * REQUIRED CONSUMERS
 * ============================================================================
 *
 * The canonical rule is intended for reuse by:
 *
 *     grammar/modules/
 *     grammar/declarations/
 *     grammar/functions/
 *     grammar/types/
 *     grammar/effects/
 *     grammar/memory/
 *     grammar/concurrency/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/networking/
 *     grammar/security/
 *     grammar/interoperability/
 *     grammar/macros/
 *     grammar/metaprogramming/
 *     future grammar domains
 *
 * A domain MUST NOT create:
 *
 *     quantumVisibility
 *     hardwareVisibility
 *     gpuVisibility
 *     qpuVisibility
 *     clusterVisibility
 *     aiVisibility
 *     moduleVisibility
 *     functionVisibility
 *     typeVisibility
 *
 * when such a rule merely duplicates the same source-level vocabulary.
 *
 * ============================================================================
 * MODULE INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/modules.g4` and related module grammars should consume:
 *
 *     visibilityModifier?
 *
 * when the language specification permits visibility on the corresponding
 * declaration.
 *
 * The old local visibility production in:
 *
 *     grammar/modules/visibility.g4
 *
 * MUST NOT remain an independent competing authority.
 *
 * Migration target:
 *
 *     grammar/modules/visibility.g4
 *             |
 *             +--> compatibility/delegation documentation
 *             |
 *             +--> no duplicate visibility alternatives
 *
 * The canonical ownership moves to:
 *
 *     grammar/core/visibility.g4
 *
 * Existing module syntax MUST NOT be renamed merely because the ownership
 * location changes.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * Function grammar should compose:
 *
 *     visibilityModifier?
 *
 * with function-specific modifiers.
 *
 * Example conceptual structure:
 *
 *     functionDeclaration
 *         : visibilityModifier?
 *           functionModifier*
 *           K_FN
 *           identifier
 *           ...
 *         ;
 *
 * Function-specific modifiers remain owned by functions/functions.g4.
 *
 * Visibility remains owned here.
 *
 * ============================================================================
 * DECLARATION INTEGRATION
 * ============================================================================
 *
 * Declaration grammars may consume this rule for:
 *
 *     struct
 *     enum
 *     record
 *     class
 *     interface
 *     trait
 *     implementation
 *     type alias
 *     constants
 *     variables
 *     domain declarations
 *     capability declarations
 *     resource declarations
 *
 * The declaration grammar determines whether the modifier is permitted.
 *
 * This grammar does not decide declaration legality.
 *
 * ============================================================================
 * EXPORT INTEGRATION
 * ============================================================================
 *
 * Visibility and export are separate concepts.
 *
 * Visibility:
 *
 *     who may access a declaration?
 *
 * Export:
 *
 *     which declarations form a module's exported interface?
 *
 * Therefore:
 *
 *     export
 *
 * MUST remain owned by the module/export grammar.
 *
 * This grammar MUST NOT redefine export syntax.
 *
 * Semantic analysis may subsequently validate consistency between:
 *
 *     visibility
 *     export policy
 *     module boundaries
 *     package boundaries
 *
 * ============================================================================
 * IMPORT INTEGRATION
 * ============================================================================
 *
 * Imports do not need their own visibility vocabulary.
 *
 * Import resolution may consult semantic visibility information after parsing.
 *
 * This grammar therefore has no dependency on:
 *
 *     imports.g4
 *     module loading
 *     filesystem access
 *     package resolution
 *     network access
 *
 * ============================================================================
 * NAMESPACE INTEGRATION
 * ============================================================================
 *
 * If namespace declarations permit source-level visibility, they consume:
 *
 *     visibilityModifier?
 *
 * They MUST NOT redefine visibility alternatives.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Type declarations may consume:
 *
 *     visibilityModifier?
 *
 * for declarations such as:
 *
 *     struct
 *     enum
 *     class
 *     interface
 *     trait
 *     type
 *     record
 *
 * Field/member visibility may also reuse the same canonical rule where the
 * language specification defines member visibility using the same vocabulary.
 *
 * If member visibility has genuinely different semantics, that semantic
 * difference MUST be represented in the semantic model rather than silently
 * duplicated in the parser grammar.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Effect declarations may consume:
 *
 *     visibilityModifier?
 *
 * The effect grammar remains responsible for:
 *
 *     effect
 *     operation
 *     handler
 *
 * syntax.
 *
 * Visibility remains independent of effect semantics.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Declarations such as:
 *
 *     task
 *     actor
 *     channel
 *     synchronization abstraction
 *
 * may consume the canonical rule where specified.
 *
 * Visibility MUST NOT encode:
 *
 *     number of workers
 *     number of threads
 *     number of actors
 *     number of nodes
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical declarations may consume this rule without any dependency on:
 *
 *     CPU
 *     ISA
 *     vector width
 *     register count
 *     cache size
 *     memory size
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum declarations may consume:
 *
 *     visibilityModifier?
 *
 * for source-level declarations such as:
 *
 *     quantum functions
 *     quantum operations
 *     circuits
 *     logical abstractions
 *     quantum modules
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     CircuitIR
 *     quantum::ir
 *
 * It MUST NOT depend on:
 *
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     calibration
 *     HAL
 *     backend topology
 *
 * Quantum semantic lowering remains:
 *
 *     source
 *       -> AST
 *       -> semantic analysis
 *       -> quantum::ir
 *       -> optimization
 *       -> routing
 *       -> scheduling
 *       -> QEC / resilience / ZQN
 *       -> HAL
 *       -> target
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware declarations may consume the same visibility syntax.
 *
 * For example:
 *
 *     pub hwmodule ...
 *
 * expresses source-level visibility only.
 *
 * It does NOT:
 *
 *     expose a physical device;
 *     allocate hardware;
 *     select an FPGA;
 *     select an ASIC;
 *     select a bus;
 *     expose a physical address;
 *     select a clock source.
 *
 * Those are downstream semantic/target concerns.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed declarations may reuse visibility without encoding:
 *
 *     node count
 *     cluster size
 *     service count
 *     replica count
 *     network topology
 *     physical endpoint
 *
 * Visibility is source/API access policy, not deployment placement.
 *
 * ============================================================================
 * AI / DATA / NETWORKING / SECURITY INTEGRATION
 * ============================================================================
 *
 * AI models, datasets, transformations, services, networking abstractions,
 * cryptographic abstractions, and security declarations may reuse:
 *
 *     visibilityModifier?
 *
 * They MUST NOT create domain-specific visibility vocabularies merely to
 * represent different access semantics.
 *
 * Domain-specific authorization semantics belong downstream.
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * Foreign declarations may reuse visibility.
 *
 * However visibility does not define:
 *
 *     ABI
 *     calling convention
 *     linker symbol visibility
 *     dynamic-loader policy
 *     foreign language access
 *
 * Interoperability grammar and semantic analysis own those concerns.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every occurrence of:
 *
 *     visibilityModifier
 *
 * MUST produce a source-level AST representation that preserves:
 *
 *     - visibility category/spelling;
 *     - source span;
 *     - declaration association.
 *
 * Conceptually:
 *
 *     VisibilitySyntax
 *     {
 *         kind,
 *         span
 *     }
 *
 * The exact Rust type is owned by:
 *
 *     src/frontend/ast/
 *
 * or the repository's active canonical frontend AST implementation.
 *
 * This grammar MUST NOT define Rust AST structures.
 *
 * Existing AST nodes already expose visibility information; this grammar must
 * therefore remain compatible with that representation rather than introducing
 * a competing visibility abstraction.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The parser/frontend MUST preserve the exact source span covering the
 * visibility token.
 *
 * The grammar itself does not construct spans.
 *
 * Source-span construction belongs to the parser/AST frontend.
 *
 * This permits diagnostics such as:
 *
 *     conflicting visibility modifiers
 *     visibility not allowed here
 *     inaccessible declaration
 *
 * to point to the exact source modifier.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic layer maps parsed visibility syntax into the canonical semantic
 * visibility model.
 *
 * Conceptually:
 *
 *     K_PUB
 *        |
 *        +--> source visibility representation
 *                         |
 *                         v
 *                semantic visibility
 *
 * and:
 *
 *     K_PUBLIC
 *        |
 *        +--> source visibility representation
 *                         |
 *                         v
 *                semantic visibility
 *
 * The semantic layer MAY canonicalize `pub` and `public` to the same semantic
 * category while retaining source provenance for diagnostics, formatting,
 * source maps, tooling, and round-tripping where required.
 *
 * This file does not decide whether:
 *
 *     pub == public
 *
 * semantically.
 *
 * That decision belongs to the language specification/semantic layer.
 *
 * ============================================================================
 * ACCESS-CHECKING CONTRACT
 * ============================================================================
 *
 * Access checking belongs downstream.
 *
 * The semantic layer determines:
 *
 *     declaration visibility
 *     current module
 *     current package
 *     current namespace
 *     caller context
 *     inheritance context
 *     export context
 *     import context
 *
 * and decides whether an access is legal.
 *
 * Parser acceptance does NOT imply semantic accessibility.
 *
 * ============================================================================
 * DEFAULT VISIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar intentionally does not define a default visibility.
 *
 * If no modifier is present:
 *
 *     visibilityModifier?
 *
 * the semantic layer determines the default according to the authoritative
 * language specification and declaration context.
 *
 * This prevents the core grammar from accidentally embedding module/package
 * policy.
 *
 * ============================================================================
 * PROTECTED VISIBILITY CONTRACT
 * ============================================================================
 *
 * `protected` is syntactically accepted because it is part of the established
 * Zamani vocabulary.
 *
 * Whether `protected` is meaningful for:
 *
 *     functions
 *     modules
 *     packages
 *     structs
 *     traits
 *     interfaces
 *     quantum declarations
 *     HDL declarations
 *     hardware declarations
 *
 * is determined by declaration-specific semantic rules.
 *
 * This grammar must not encode inheritance or object-model semantics.
 *
 * ============================================================================
 * INTERNAL VISIBILITY CONTRACT
 * ============================================================================
 *
 * `internal` is syntactically accepted as a general visibility category.
 *
 * Its exact semantic boundary, such as module/package/crate/unit scope, belongs
 * to semantic analysis and the language specification.
 *
 * The parser must not assume a particular repository/package implementation.
 *
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * Dialects MUST NOT silently replace the canonical visibility rule.
 *
 * A dialect may:
 *
 *     - restrict where visibility may occur;
 *     - add semantic policy;
 *     - define a feature gate;
 *     - provide domain-specific interpretation;
 *
 * but must reuse the canonical vocabulary unless the language specification
 * explicitly introduces a new visibility concept.
 *
 * A dialect MUST NOT create a second global visibility authority.
 *
 * ============================================================================
 * MACRO CONTRACT
 * ============================================================================
 *
 * Macro expansion MUST NOT bypass visibility semantics.
 *
 * Macro-generated declarations must enter the same semantic visibility
 * analysis pipeline as source-written declarations.
 *
 * This grammar does not define macro expansion.
 *
 * ============================================================================
 * METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Generated declarations must ultimately use the same visibility semantic
 * model as ordinary declarations.
 *
 * Reflection and code generation cannot create a private/public state that
 * bypasses normal semantic validation.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
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
 *     resource discovery
 *     backend discovery
 *
 * Given the same token stream and parser configuration, parsing is deterministic.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Visibility parsing MUST NOT:
 *
 *     - execute commands;
 *     - load files;
 *     - load packages;
 *     - contact networks;
 *     - inspect credentials;
 *     - inspect environment variables;
 *     - discover hardware;
 *     - invoke compilers;
 *     - invoke runtimes.
 *
 * Visibility is declarative source syntax.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * This grammar intentionally does not perform semantic error reporting.
 *
 * Syntactic errors are handled by the parser infrastructure.
 *
 * Semantic diagnostics belong to later analysis, including:
 *
 *     duplicate/conflicting visibility;
 *     visibility not allowed for declaration kind;
 *     inaccessible symbol;
 *     invalid inheritance visibility;
 *     export/visibility conflict;
 *     dialect visibility violation.
 *
 * ============================================================================
 * COMPILER / IR CONTRACT
 * ============================================================================
 *
 * Visibility may influence downstream:
 *
 *     symbol tables
 *     module interfaces
 *     semantic reachability
 *     linkage
 *     export sets
 *     dead-code analysis
 *     code generation
 *     interoperability
 *
 * It MUST NOT directly control:
 *
 *     quantum::ir
 *     QEC
 *     routing
 *     scheduling
 *     ZQN
 *     physical qubit allocation
 *     hardware topology
 *     resource discovery
 *
 * Any IR-level representation is created by the semantic/IR layer.
 *
 * ============================================================================
 * RUST 1.97 / 1.97.1 CONTRACT
 * ============================================================================
 *
 * This grammar has no Rust-language dependency.
 *
 * Rust implementation code consuming this grammar MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and MUST NOT require:
 *
 *     unsafe code
 *
 * for visibility parsing, AST construction, semantic analysis, or diagnostics.
 *
 * The Rust crate should enforce this independently with an appropriate
 * repository-level unsafe-code prohibition.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS:
 *
 *   - no machine capacity;
 *   - no CPU limit;
 *   - no GPU limit;
 *   - no FPGA limit;
 *   - no QPU limit;
 *   - no qubit limit;
 *   - no node limit;
 *   - no memory limit;
 *   - no register limit;
 *   - no topology;
 *   - no device identifiers;
 *   - no vendor identifiers;
 *   - no fixed declaration count;
 *   - no fixed module count;
 *   - no fixed package count;
 *   - no finite qualification-depth grammar;
 *   - no runtime behavior;
 *   - no target-specific visibility syntax.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when ALL of the following are true:
 *
 *   [x] Canonical visibility vocabulary is centralized.
 *   [x] Lexer ownership remains in ZamaniTokens.
 *   [x] Parser ownership is isolated here.
 *   [x] No lexer rules are duplicated.
 *   [x] No declaration grammar is duplicated here.
 *   [x] No semantic rules are embedded here.
 *   [x] No hardware limits are embedded here.
 *   [x] No runtime behavior is embedded here.
 *   [x] Exactly one visibility modifier is accepted per occurrence.
 *   [x] Optionality remains owned by consuming declarations.
 *   [x] Source spelling can be preserved through token identity.
 *   [x] AST integration is predetermined.
 *   [x] Semantic integration is predetermined.
 *   [x] Module integration is predetermined.
 *   [x] Function integration is predetermined.
 *   [x] Declaration integration is predetermined.
 *   [x] Quantum integration is predetermined.
 *   [x] HDL/hardware integration is predetermined.
 *   [x] Domain extensions reuse the same contract.
 *   [x] No circular dependency is introduced.
 *   [x] No unsafe Rust is required.
 *   [x] POCO-REAF constraints are satisfied.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive examples:
 *
 *     pub fn compute() {}
 *     public fn compute() {}
 *     private fn compute() {}
 *     protected fn compute() {}
 *     internal fn compute() {}
 *
 * Declaration consumers should similarly accept the canonical rule where
 * permitted by their own grammar.
 *
 * Negative examples:
 *
 *     pub private fn compute() {}
 *     public private fn compute() {}
 *     protected internal fn compute() {}
 *
 * These must not be accepted as one visibility modifier.
 *
 * Boundary examples:
 *
 *     pub
 *     private
 *     internal
 *
 * as complete modifier tokens.
 *
 * Scalability examples:
 *
 *     pub domain::a::b::c::declaration
 *
 * and arbitrarily larger qualified declaration structures must not require
 * changes to this grammar.
 *
 * Domain portability examples must verify that the same visibility syntax is
 * usable across:
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
 *     future domains
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */