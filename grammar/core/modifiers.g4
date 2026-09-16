/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/modifiers.g4
 *
 * Grammar identity:
 *     Modifiers
 *
 * Status:
 *     Production parser component.
 *
 * Purpose:
 *     Canonical, reusable parser grammar for source-level modifiers.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime callbacks, or unsafe code.
 *
 * ============================================================================
 * 1. ARCHITECTURAL ROLE
 * ============================================================================
 *
 * A modifier is source-level declaration/type/member/function/parameter
 * metadata that changes or qualifies the syntactic or semantic interpretation
 * of the construct to which it is attached.
 *
 * This grammar answers:
 *
 *     "Does this sequence form a syntactically valid modifier?"
 *
 * It does NOT answer:
 *
 *     "Is this modifier legal here?"
 *     "What does this modifier mean?"
 *     "Can this modifier be honored by the target?"
 *
 * Those questions belong to structural and semantic analysis.
 *
 * The intended pipeline is:
 *
 *     source
 *         |
 *         v
 *     ZamaniTokens
 *         |
 *         v
 *     Modifiers
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     structural validation
 *         |
 *         v
 *     semantic modifier registry
 *         |
 *         +--> type semantics
 *         +--> effect semantics
 *         +--> capability semantics
 *         +--> resource semantics
 *         +--> compilation policy
 *         +--> domain semantics
 *         |
 *         v
 *     canonical semantic model / IR
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL / hardware IR
 *         |
 *         v
 *     optimization / routing / scheduling / resilience / ZQN / HAL
 *         |
 *         v
 *     target realization
 *
 * ============================================================================
 * 2. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - generic modifier syntax;
 *     - modifier sequences;
 *     - individual modifier categories;
 *     - visibility modifiers;
 *     - storage/declaration modifiers;
 *     - linkage modifiers;
 *     - behavioral modifiers;
 *     - type/object-model modifiers;
 *     - safety modifiers;
 *     - extension/namespaced modifiers;
 *     - optional modifier values;
 *     - modifier lists;
 *     - reusable modifier integration points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifier lexical rules;
 *     - keyword spelling;
 *     - qualified-name syntax;
 *     - attributes;
 *     - annotations;
 *     - expressions;
 *     - types;
 *     - declarations;
 *     - functions;
 *     - modules;
 *     - effects;
 *     - memory;
 *     - concurrency;
 *     - classical semantics;
 *     - quantum semantics;
 *     - quantum::ir;
 *     - HDL semantics;
 *     - hardware discovery;
 *     - resource allocation;
 *     - target selection;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - runtime execution;
 *     - deployment.
 *
 * ============================================================================
 * 3. LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical modular lexical vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * whose grammar identity is:
 *
 *     ZamaniTokens
 *
 * This file therefore uses:
 *
 *     tokenVocab = ZamaniTokens
 *
 * and MUST NOT define lexer rules.
 *
 * In particular, this grammar does not redefine:
 *
 *     IDENTIFIER
 *     K_PUB
 *     K_PUBLIC
 *     K_PRIVATE
 *     K_PROTECTED
 *     K_INTERNAL
 *     K_STATIC
 *     K_CONST
 *     K_LET
 *     K_VAR
 *     K_VAL
 *     K_MUT
 *     K_EXTERN
 *     K_VOLATILE
 *     K_INLINE
 *     K_FINAL
 *     K_SEALED
 *     K_PARTIAL
 *     K_OVERRIDE
 *     K_VIRTUAL
 *     K_ABSTRACT
 *     K_ASYNC
 *     K_SAFE
 *     K_UNSAFE
 *     DOUBLE_COLON
 *     EQUALS
 *
 * The lexer remains the sole owner of their spelling and token identity.
 *
 * ============================================================================
 * 4. NAME AUTHORITY
 * ============================================================================
 *
 * Canonical source-level name syntax is owned by:
 *
 *     grammar/core/names.g4
 *
 * whose grammar identity is:
 *
 *     Names
 *
 * This grammar imports Names and reuses:
 *
 *     identifier
 *     qualifiedName
 *
 * It MUST NOT redefine:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *
 * This permits open-ended extension modifiers such as:
 *
 *     zamani::async
 *     zamani::inline
 *     quantum::entry
 *     quantum::adaptive
 *     hardware::pipeline
 *     distributed::replicated
 *     future::domain::modifier
 *
 * without changing this grammar.
 *
 * ============================================================================
 * 5. FUNDAMENTAL DESIGN: CLOSED CORE, OPEN EXTENSIONS
 * ============================================================================
 *
 * The core language has a finite set of reserved modifier keywords because
 * their lexical spelling and language-level role are part of the current
 * Zamani language contract.
 *
 * Extension modifiers remain open-ended.
 *
 * Therefore:
 *
 *     modifier
 *         -> reservedModifier
 *         | extensionModifier
 *
 * Reserved modifiers provide stable core semantics.
 *
 * Extension modifiers provide future/domain/dialect extensibility without
 * requiring every new modifier to become a core keyword.
 *
 * This is deliberately NOT an unrestricted:
 *
 *     IDENTIFIER
 *
 * modifier.
 *
 * An arbitrary identifier by itself remains an ordinary name. A modifier
 * extension must have explicit namespace qualification so that declarations
 * do not accidentally consume ordinary identifiers as modifiers.
 *
 * ============================================================================
 * 6. MODIFIER VS ATTRIBUTE
 * ============================================================================
 *
 * Modifiers and attributes are distinct syntactic mechanisms.
 *
 * Modifiers:
 *
 *     pub fn ...
 *     async fn ...
 *     quantum::adaptive fn ...
 *
 * Attributes:
 *
 *     @quantum::resource(...)
 *     @compile(...)
 *
 * Attribute syntax is owned by:
 *
 *     grammar/core/attributes.g4
 *
 * Annotation compatibility is owned by the annotation/metadata layer.
 *
 * This grammar MUST NOT redefine:
 *
 *     @
 *     attribute arguments
 *     attribute maps
 *     attribute lists
 *
 * A semantic system may map modifier information and attribute information
 * into related metadata, but the parser must preserve their distinct source
 * structures.
 *
 * ============================================================================
 * 7. MODIFIER VS REQUIREMENT / CAPABILITY / HINT
 * ============================================================================
 *
 * A modifier MUST NOT silently become:
 *
 *     - a requirement;
 *     - a capability declaration;
 *     - a resource allocation;
 *     - a placement command;
 *     - a scheduling command;
 *     - a routing command;
 *     - an optimization command;
 *     - a hardware selection.
 *
 * For example:
 *
 *     inline
 *
 * is not:
 *
 *     requires inline
 *
 * and:
 *
 *     quantum::adaptive
 *
 * does not select a particular QPU.
 *
 * Semantic analysis determines the meaning of each modifier.
 *
 * ============================================================================
 * 8. POCO-REAF CONTRACT
 * ============================================================================
 *
 * Modifiers MUST remain target-independent at the language level.
 *
 * A modifier may express source-level intent such as:
 *
 *     async
 *     inline
 *     const
 *     static
 *     quantum::adaptive
 *     hardware::pipeline
 *     distributed::replicated
 *
 * but it MUST NOT encode:
 *
 *     cpu0
 *     gpu0
 *     qpu7
 *     fpga3
 *     physical_qubit17
 *     memory_bank2
 *     node42
 *     device_address
 *
 * Target realization belongs downstream.
 *
 * The same source program may therefore be compiled against:
 *
 *     a tiny embedded system;
 *     a multicore CPU;
 *     a GPU;
 *     an FPGA;
 *     an ASIC;
 *     a simulator;
 *     a QPU;
 *     a distributed cluster;
 *     a future computational architecture.
 *
 * ============================================================================
 * 9. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_MODIFIERS
 *     MAX_MODIFIER_ARGUMENTS
 *     MAX_MODIFIER_DEPTH
 *     MAX_NAMESPACE_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_TENSOR_RANK
 *
 * It MUST NOT contain physical resource enumerations such as:
 *
 *     cpuModifier
 *     gpuModifier
 *     qpuModifier
 *     fpgaModifier
 *
 * merely to represent hardware.
 *
 * Hardware capabilities and requirements belong to the corresponding
 * capability/resource/target grammars.
 *
 * ============================================================================
 * 10. SCALABILITY
 * ============================================================================
 *
 * There is no language-level finite limit on:
 *
 *     - number of modifiers;
 *     - number of modifier sequences;
 *     - number of extension modifiers;
 *     - qualified-name depth;
 *     - modifier value size;
 *     - number of declarations;
 *     - number of functions;
 *     - number of types;
 *     - number of computational domains;
 *     - number of resources;
 *     - number of devices;
 *     - number of qubits;
 *     - number of distributed nodes.
 *
 * Repetition is represented with ANTLR repetition operators.
 *
 * Practical parser/compiler limits are implementation/resource policy and
 * MUST NOT become language semantics.
 *
 * ============================================================================
 * 11. MODIFIER ORDER
 * ============================================================================
 *
 * The grammar preserves source order.
 *
 * It intentionally does NOT canonicalize:
 *
 *     public async inline fn ...
 *
 * into:
 *
 *     inline public async fn ...
 *
 * Semantic analysis may later determine whether modifier order is:
 *
 *     - irrelevant;
 *     - constrained;
 *     - canonicalizable;
 *     - conflicting.
 *
 * This preserves source provenance and deterministic diagnostics.
 *
 * ============================================================================
 * 12. DUPLICATES
 * ============================================================================
 *
 * The grammar permits repeated modifiers structurally.
 *
 * Example:
 *
 *     inline inline fn ...
 *
 * Whether this is legal is a semantic rule.
 *
 * This is intentional.
 *
 * The parser must preserve source structure rather than silently discarding
 * repeated modifiers.
 *
 * Examples of possible semantic outcomes include:
 *
 *     accepted;
 *     warning;
 *     error;
 *     normalized;
 *
 * The grammar does not choose among them.
 *
 * ============================================================================
 * 13. CONFLICTS
 * ============================================================================
 *
 * The grammar does not encode modifier conflicts.
 *
 * For example, a sequence such as:
 *
 *     abstract final
 *
 * may or may not be semantically meaningful depending on the construct and
 * language version.
 *
 * Semantic validation determines:
 *
 *     - incompatibilities;
 *     - required combinations;
 *     - mutually exclusive modifiers;
 *     - context-specific modifiers;
 *     - version-specific legality.
 *
 * This avoids duplicating declaration-specific semantic policy throughout
 * the parser grammar.
 *
 * ============================================================================
 * 14. CONTEXT-SPECIFIC VALIDATION
 * ============================================================================
 *
 * This file provides generic modifier syntax.
 *
 * Consuming grammars determine where modifiers may occur.
 *
 * Examples:
 *
 *     functions/functions.g4
 *     declarations/*.g4
 *     modules/*.g4
 *     types/*.g4
 *     memory/*.g4
 *     concurrency/*.g4
 *     quantum/*.g4
 *     hdl/*.g4
 *     hardware/*.g4
 *
 * A consumer may therefore define:
 *
 *     functionModifierList
 *         : modifierList
 *         ;
 *
 * or:
 *
 *     typeModifierList
 *         : modifierList
 *         ;
 *
 * without duplicating modifier syntax.
 *
 * The semantic layer determines whether each modifier is legal for that
 * context.
 *
 * ============================================================================
 * 15. RESERVED VISIBILITY MODIFIERS
 * ============================================================================
 *
 * These tokens already exist in the canonical lexer:
 *
 *     K_PUB
 *     K_PUBLIC
 *     K_PRIVATE
 *     K_PROTECTED
 *     K_INTERNAL
 *
 * Both `pub` and `public` are accepted because both are already part of the
 * repository's lexical vocabulary.
 *
 * They are syntax-level alternatives.
 *
 * Semantic normalization may determine whether they are equivalent.
 *
 * ============================================================================
 * 16. STORAGE / MUTABILITY MODIFIERS
 * ============================================================================
 *
 * The canonical lexer currently exposes:
 *
 *     K_STATIC
 *     K_CONST
 *     K_LET
 *     K_VAR
 *     K_VAL
 *     K_MUT
 *
 * These are syntactically available through this generic modifier grammar.
 *
 * IMPORTANT:
 *
 * This does not mean every declaration may use every one of them.
 *
 * Contextual legality remains semantic/declaration-owned.
 *
 * ============================================================================
 * 17. LINKAGE MODIFIERS
 * ============================================================================
 *
 * The canonical lexical vocabulary provides:
 *
 *     K_EXTERN
 *
 * `extern` is therefore a generic modifier token.
 *
 * ABI/calling-convention semantics belong to interoperability/function
 * subsystems.
 *
 * This grammar does not select an ABI.
 *
 * ============================================================================
 * 18. BEHAVIORAL / IMPLEMENTATION MODIFIERS
 * ============================================================================
 *
 * The canonical lexical vocabulary currently provides:
 *
 *     K_VOLATILE
 *     K_INLINE
 *     K_ASYNC
 *
 * These are accepted syntactically here.
 *
 * Their exact legality depends on the consuming declaration/context.
 *
 * ============================================================================
 * 19. OBJECT-MODEL / TYPE MODIFIERS
 * ============================================================================
 *
 * The canonical lexer provides:
 *
 *     K_FINAL
 *     K_SEALED
 *     K_PARTIAL
 *     K_OVERRIDE
 *     K_VIRTUAL
 *     K_ABSTRACT
 *
 * These remain generic modifier syntax.
 *
 * Class/interface/trait/declaration grammars decide which combinations are
 * valid.
 *
 * ============================================================================
 * 20. SAFETY MODIFIERS
 * ============================================================================
 *
 * The canonical lexer provides:
 *
 *     K_SAFE
 *     K_UNSAFE
 *
 * The presence of `unsafe` in the lexical vocabulary does NOT mean that
 * Zamani's compiler is permitted to use Rust `unsafe`.
 *
 * These are language-level source constructs.
 *
 * The Rust implementation remains required to use safe Rust.
 *
 * In particular:
 *
 *     Zamani `unsafe`
 *
 * and:
 *
 *     Rust `unsafe`
 *
 * are completely different architectural concepts.
 *
 * Semantic validation decides whether a Zamani construct may use an unsafe
 * language capability.
 *
 * The Rust compiler implementation MUST still compile without `unsafe` code.
 *
 * ============================================================================
 * 21. EXTENSION MODIFIERS
 * ============================================================================
 *
 * Extension modifiers are explicitly namespaced:
 *
 *     qualifiedName
 *
 * Examples:
 *
 *     quantum::adaptive
 *     quantum::entry
 *     hardware::pipeline
 *     distributed::replicated
 *     ai::differentiable
 *     data::streaming
 *     future::domain::modifier
 *
 * This mechanism is essential for POCO-REAF.
 *
 * A future domain does not need to modify the core modifier grammar merely
 * because it introduces a new modifier identity.
 *
 * The extension registry determines:
 *
 *     - whether the modifier exists;
 *     - who owns it;
 *     - which constructs accept it;
 *     - its version;
 *     - its semantic meaning;
 *     - its compatibility;
 *     - its AST/semantic mapping.
 *
 * ============================================================================
 * 22. EXTENSION MODIFIER VALUES
 * ============================================================================
 *
 * Extension modifiers may optionally carry a structural value:
 *
 *     quantum::mode = symbolic
 *     hardware::policy = portable
 *     future::domain::mode = "example"
 *
 * The grammar intentionally restricts modifier values to structural forms:
 *
 *     literal
 *     qualifiedName
 *
 * It does NOT create a second expression language.
 *
 * If arbitrary expressions are eventually permitted, that must be integrated
 * explicitly with the canonical expressions grammar.
 *
 * ============================================================================
 * 23. VALUE OWNERSHIP
 * ============================================================================
 *
 * Literal syntax belongs to the canonical lexer/literal grammar.
 *
 * Name syntax belongs to Names.
 *
 * This file therefore uses:
 *
 *     modifierValue
 *
 * as a structural integration boundary.
 *
 * Semantic analysis determines:
 *
 *     - expected type;
 *     - valid value domain;
 *     - constant requirements;
 *     - compatibility;
 *     - portability;
 *     - target applicability.
 *
 * ============================================================================
 * 24. GENERIC MODIFIER STRUCTURE
 * ============================================================================
 *
 * The primary public rule is:
 *
 *     modifier
 *
 * and the reusable list rule is:
 *
 *     modifierList
 *
 * Consumers should normally use:
 *
 *     modifierList
 *
 * rather than reproducing:
 *
 *     modifier*
 *
 * throughout the repository.
 *
 * This creates a stable integration point for future syntax evolution.
 *
 * ============================================================================
 * 25. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - modifier ordering;
 *     - modifier spelling/token identity where required;
 *     - modifier category;
 *     - extension namespace;
 *     - extension name;
 *     - optional value;
 *     - source span;
 *     - source order.
 *
 * Conceptual representation:
 *
 *     Modifier
 *         Reserved(...)
 *         Extension {
 *             name,
 *             value
 *         }
 *
 * The repository's function AST already uses a namespaced/open-ended
 * `FunctionModifier` representation. This grammar is therefore designed to
 * lower naturally into that model rather than forcing a closed Rust enum.
 *
 * The exact Rust AST representation belongs to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT import Rust types.
 *
 * ============================================================================
 * 26. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - modifier resolution;
 *     - context validation;
 *     - duplicate detection;
 *     - conflict detection;
 *     - required-combination checking;
 *     - version checking;
 *     - dialect/extension validation;
 *     - capability validation;
 *     - resource implications;
 *     - portability analysis;
 *     - deprecation diagnostics;
 *     - normalization;
 *     - semantic lowering.
 *
 * Unknown extension modifiers may remain syntactically valid.
 *
 * The active compatibility policy determines whether an unknown modifier is:
 *
 *     informational;
 *     warning;
 *     compatibility diagnostic;
 *     semantic error.
 *
 * This decision MUST NOT be encoded in parser syntax.
 *
 * ============================================================================
 * 27. IR CONTRACT
 * ============================================================================
 *
 * Modifiers do not constitute an independent IR.
 *
 * After semantic validation, a modifier may become:
 *
 *     - semantic metadata;
 *     - effect metadata;
 *     - capability requirements;
 *     - resource requirements;
 *     - optimization metadata;
 *     - execution policy;
 *     - interoperability metadata;
 *     - quantum operation metadata;
 *     - HDL/hardware intent metadata.
 *
 * A modifier may influence lowering, but:
 *
 *     modifier grammar
 *
 * MUST NOT become:
 *
 *     modifier IR
 *
 * merely to duplicate the canonical semantic model.
 *
 * ============================================================================
 * 28. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum modifiers remain source-level intent.
 *
 * Examples:
 *
 *     quantum::adaptive
 *     quantum::entry
 *     quantum::logical
 *     quantum::resource
 *
 * The modifier grammar MUST NOT import:
 *
 *     quantum::ir
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *
 * A quantum modifier may influence semantic analysis and subsequently affect
 * lowering toward:
 *
 *     quantum::ir
 *
 * followed by:
 *
 *     optimization
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     HAL
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 * 29. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical constructs may use:
 *
 *     static
 *     const
 *     inline
 *     async
 *     and extension modifiers.
 *
 * The grammar does not distinguish CPU architectures, instruction sets,
 * vector widths, register files, or core counts.
 *
 * Those concerns belong downstream.
 *
 * ============================================================================
 * 30. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware domains may use namespaced extension modifiers:
 *
 *     hdl::pipeline
 *     hdl::synthesizable
 *     hardware::pipeline
 *     hardware::resource_sharing
 *
 * These are syntactic identifiers only.
 *
 * They do NOT select:
 *
 *     FPGA model
 *     ASIC process
 *     clock network
 *     physical placement
 *     synthesis tool
 *     vendor
 *     device
 *
 * Such decisions belong to hardware/compiler/backend layers.
 *
 * ============================================================================
 * 31. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed extensions may express semantic intent such as:
 *
 *     distributed::replicated
 *     distributed::partitioned
 *     distributed::locality
 *
 * They MUST NOT encode:
 *
 *     node0
 *     node1
 *     host42
 *     fixed cluster size
 *     fixed provider
 *
 * Placement and deployment remain downstream.
 *
 * ============================================================================
 * 32. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Open-ended extension modifiers can express:
 *
 *     ai::differentiable
 *     ai::batched
 *     data::streaming
 *     data::parallel
 *     tensor::layout
 *
 * without making AI frameworks or tensor dimensions part of core syntax.
 *
 * ============================================================================
 * 33. SECURITY INTEGRATION
 * ============================================================================
 *
 * Security-related modifiers may express language-level properties:
 *
 *     security::confidential
 *     security::constant_time
 *     security::verified
 *
 * but they do not grant authorization and do not perform security operations.
 *
 * Authorization, key management, cryptographic execution and secure hardware
 * selection remain downstream responsibilities.
 *
 * ============================================================================
 * 34. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime callbacks;
 *     - no randomness.
 *
 * Parsing therefore depends only on the input token stream.
 *
 * ============================================================================
 * 35. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must preserve source spans for:
 *
 *     modifier;
 *     modifier category;
 *     extension namespace;
 *     extension name;
 *     optional modifier value.
 *
 * This is required for:
 *
 *     - diagnostics;
 *     - IDE tooling;
 *     - formatting;
 *     - source maps;
 *     - provenance;
 *     - compatibility migration;
 *     - semantic validation.
 *
 * ============================================================================
 * 36. DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics are limited to structural errors.
 *
 * Examples:
 *
 *     quantum:: fn ...
 *     ::adaptive fn ...
 *     quantum::adaptive:: fn ...
 *
 * may be syntax errors depending on the exact token sequence.
 *
 * The parser MUST NOT report:
 *
 *     "quantum::adaptive is unsupported by this QPU"
 *
 * because that is semantic/runtime information.
 *
 * Similarly, the parser MUST NOT inspect hardware while parsing.
 *
 * ============================================================================
 * 37. COMPILER INTEGRATION
 * ============================================================================
 *
 * Consumers should import this grammar and use:
 *
 *     modifier
 *     modifierList
 *
 * rather than implementing local modifier alternatives.
 *
 * In particular, `grammar/functions/functions.g4` currently owns a closed
 * `functionModifier` rule. Production integration should replace that local
 * vocabulary with:
 *
 *     functionModifier
 *         : modifier
 *         ;
 *
 * or:
 *
 *     functionModifierList
 *         : modifierList
 *         ;
 *
 * while keeping the existing public function grammar names as compatibility
 * wrappers where necessary.
 *
 * This avoids forcing a rename of existing function grammar rules.
 *
 * The same wrapper strategy applies to declaration-specific grammars.
 *
 * ============================================================================
 * 38. COMPATIBILITY STRATEGY
 * ============================================================================
 *
 * Existing consumers do not need to rename their contextual rule immediately.
 *
 * They may retain:
 *
 *     functionModifier
 *     typeModifier
 *     declarationModifier
 *
 * as thin forwarding rules:
 *
 *     functionModifier
 *         : modifier
 *         ;
 *
 * This preserves existing public rule names while centralizing ownership.
 *
 * The canonical syntax remains owned by this file.
 *
 * ============================================================================
 * 39. RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * Generated parser/frontend integration MUST:
 *
 *     - compile with Rust 1.97;
 *     - compile with Rust 1.97.1;
 *     - use Edition 2021;
 *     - use safe Rust;
 *     - contain no `unsafe`;
 *     - preserve source spans;
 *     - preserve deterministic parse structure;
 *     - avoid machine-specific assumptions.
 *
 * The Rust implementation should enforce the repository's safe-Rust policy,
 * including denial of unsafe code at the crate level where configured.
 *
 * ============================================================================
 * 40. TEST CONTRACT
 * ============================================================================
 *
 * The production test suite should exercise this file independently.
 *
 * --------------------------------------------------------------------------
 * Positive syntax
 * --------------------------------------------------------------------------
 *
 *     public
 *     pub
 *     private
 *     protected
 *     internal
 *     static
 *     const
 *     let
 *     var
 *     val
 *     mut
 *     extern
 *     volatile
 *     inline
 *     final
 *     sealed
 *     partial
 *     override
 *     virtual
 *     abstract
 *     async
 *     safe
 *     unsafe
 *
 * --------------------------------------------------------------------------
 * Modifier sequences
 * --------------------------------------------------------------------------
 *
 *     public static
 *     pub const
 *     private inline
 *     public async
 *     extern unsafe
 *     public static inline
 *
 * --------------------------------------------------------------------------
 * Extension modifiers
 * --------------------------------------------------------------------------
 *
 *     quantum::adaptive
 *     quantum::entry
 *     hardware::pipeline
 *     distributed::replicated
 *     ai::differentiable
 *     data::streaming
 *     future::domain::modifier
 *
 * --------------------------------------------------------------------------
 * Extension values
 * --------------------------------------------------------------------------
 *
 *     quantum::mode = symbolic
 *     hardware::policy = portable
 *     future::domain::mode = "example"
 *
 * --------------------------------------------------------------------------
 * Long sequences
 * --------------------------------------------------------------------------
 *
 * Tests should construct arbitrarily large modifier sequences subject only to
 * the test runner's resource budget.
 *
 * No grammar test may assert a fixed maximum modifier count.
 *
 * --------------------------------------------------------------------------
 * Negative syntax
 * --------------------------------------------------------------------------
 *
 *     ::modifier
 *     namespace::
 *     namespace::::modifier
 *     ::
 *     namespace:: = value
 *     namespace::modifier =
 *
 * where the relevant token sequence is structurally invalid.
 *
 * --------------------------------------------------------------------------
 * Semantic-negative cases
 * --------------------------------------------------------------------------
 *
 * These MUST be tested downstream rather than rejected here:
 *
 *     abstract final
 *     duplicate visibility
 *     duplicate const
 *     invalid modifier for a declaration kind
 *     unknown extension modifier
 *     deprecated modifier
 *     incompatible modifier combination
 *     target-inapplicable modifier
 *
 * The parser should preserve valid structure so semantic validation can issue
 * the correct diagnostic.
 *
 * --------------------------------------------------------------------------
 * Boundary cases
 * --------------------------------------------------------------------------
 *
 *     one modifier;
 *     many modifiers;
 *     deeply qualified extension modifier;
 *     long modifier value;
 *     repeated modifiers;
 *     mixed reserved and extension modifiers;
 *     modifiers in different declaration contexts.
 *
 * --------------------------------------------------------------------------
 * Scalability cases
 * --------------------------------------------------------------------------
 *
 * Verify that the grammar imposes no fixed limit on:
 *
 *     modifier count;
 *     extension namespace depth;
 *     value size;
 *     declaration count;
 *     computational domain count.
 *
 * --------------------------------------------------------------------------
 * Compatibility cases
 * --------------------------------------------------------------------------
 *
 * Verify that:
 *
 *     functions/functions.g4
 *     declarations/*.g4
 *     modules/*.g4
 *     types/*.g4
 *     interoperability/*.g4
 *     domain grammars
 *
 * all consume the same canonical modifier structure.
 *
 * ============================================================================
 * 41. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] grammar identity is `Modifiers`;
 *     [x] canonical lexer vocabulary is `ZamaniTokens`;
 *     [x] canonical name grammar is imported from `Names`;
 *     [x] no lexer rules are duplicated;
 *     [x] no identifier rules are duplicated;
 *     [x] generic modifier syntax is defined;
 *     [x] modifier lists are defined;
 *     [x] visibility modifiers are centralized;
 *     [x] storage/mutability modifiers are centralized;
 *     [x] linkage modifiers are centralized;
 *     [x] behavioral modifiers are centralized;
 *     [x] object-model modifiers are centralized;
 *     [x] safety modifiers are centralized;
 *     [x] open-ended namespaced modifiers are supported;
 *     [x] modifier values have an explicit structural boundary;
 *     [x] modifier order is preserved;
 *     [x] duplicate modifiers remain available for semantic validation;
 *     [x] context-specific legality remains downstream;
 *     [x] no hardware IDs are encoded;
 *     [x] no machine limits are encoded;
 *     [x] no quantum IR is duplicated;
 *     [x] quantum modifiers remain source-level intent;
 *     [x] HDL/hardware modifiers remain target-independent;
 *     [x] distributed modifiers do not select nodes;
 *     [x] AI/data modifiers do not encode framework-specific limits;
 *     [x] source spans are preserved by the frontend contract;
 *     [x] diagnostics remain deterministic;
 *     [x] Rust 1.97/1.97.1 compatibility is specified;
 *     [x] no unsafe Rust is required;
 *     [x] positive/negative/boundary/scalability/compatibility tests are
 *         specified;
 *     [x] existing contextual rule names can remain as compatibility wrappers.
 *
 * ============================================================================
 */

parser grammar Modifiers;

options {
    tokenVocab = ZamaniTokens;
}

import Names;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A single modifier.
 *
 * Reserved modifiers have stable core lexical identities.
 * Extension modifiers use explicit qualified names.
 */
modifier
    : reservedModifier
    | extensionModifier
    ;


/*
 * ============================================================================
 * PUBLIC MODIFIER LIST
 * ============================================================================
 *
 * One or more modifiers.
 *
 * Contextual consumers that need zero or more modifiers should use:
 *
 *     modifierList?
 *
 * rather than duplicating the underlying alternatives.
 */
modifierList
    : modifier+
    ;


/*
 * ============================================================================
 * RESERVED MODIFIER
 * ============================================================================
 *
 * These alternatives correspond only to modifier keywords already present in
 * the canonical Zamani lexical vocabulary.
 *
 * Their semantic legality is contextual.
 */
reservedModifier
    : visibilityModifier
    | storageModifier
    | linkageModifier
    | behaviorModifier
    | objectModelModifier
    | safetyModifier
    ;


/*
 * ============================================================================
 * VISIBILITY
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
 * STORAGE / MUTABILITY
 * ============================================================================
 */

storageModifier
    : K_STATIC
    | K_CONST
    | K_LET
    | K_VAR
    | K_VAL
    | K_MUT
    ;


/*
 * ============================================================================
 * LINKAGE
 * ============================================================================
 */

linkageModifier
    : K_EXTERN
    ;


/*
 * ============================================================================
 * BEHAVIOR / IMPLEMENTATION
 * ============================================================================
 */

behaviorModifier
    : K_VOLATILE
    | K_INLINE
    | K_ASYNC
    ;


/*
 * ============================================================================
 * OBJECT MODEL / TYPE QUALIFICATION
 * ============================================================================
 */

objectModelModifier
    : K_FINAL
    | K_SEALED
    | K_PARTIAL
    | K_OVERRIDE
    | K_VIRTUAL
    | K_ABSTRACT
    ;


/*
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * These are Zamani source-language modifiers.
 *
 * They do NOT authorize Rust `unsafe`.
 */
safetyModifier
    : K_SAFE
    | K_UNSAFE
    ;


/*
 * ============================================================================
 * OPEN-WORLD EXTENSION MODIFIER
 * ============================================================================
 *
 * Extension modifiers require a qualified name.
 *
 * Examples:
 *
 *     quantum::adaptive
 *     hardware::pipeline
 *     distributed::replicated
 *     future::domain::modifier
 *
 * The namespace/name structure is inherited from the canonical Names grammar.
 *
 * A bare identifier is deliberately not accepted here because it could be
 * indistinguishable from an ordinary declaration name.
 */
extensionModifier
    : qualifiedModifierName modifierValueClause?
    ;


/*
 * ============================================================================
 * EXTENSION MODIFIER NAME
 * ============================================================================
 *
 * A modifier extension is a qualified name containing at least two segments.
 *
 * `qualifiedName` is the canonical name syntax.
 *
 * The separate rule makes the minimum qualification requirement explicit
 * without redefining qualified-name syntax.
 */
qualifiedModifierName
    : identifier DOUBLE_COLON identifier
      (DOUBLE_COLON identifier)*
    ;


/*
 * ============================================================================
 * OPTIONAL EXTENSION MODIFIER VALUE
 * ============================================================================
 *
 * Example:
 *
 *     quantum::mode = symbolic
 *     hardware::policy = portable
 *     future::domain::mode = "example"
 *
 * The value is structural data, not an arbitrary expression.
 */
modifierValueClause
    : EQUALS modifierValue
    ;


/*
 * ============================================================================
 * MODIFIER VALUE
 * ============================================================================
 *
 * A modifier value intentionally does not define a second expression grammar.
 *
 * Values are limited to:
 *
 *     - canonical qualified names;
 *     - lexical literals.
 *
 * If arbitrary expressions become a language requirement, this rule must be
 * integrated with the canonical expression grammar rather than creating a
 * duplicate expression implementation here.
 */
modifierValue
    : qualifiedName
    | INTEGER_LITERAL
    | DECIMAL_LITERAL
    | STRING_LITERAL
    | CHARACTER_LITERAL
    | BOOLEAN_LITERAL
    | QUANTUM_LITERAL
    | HARDWARE_LITERAL
    | DURATION_LITERAL
    | SIZE_LITERAL
    ;