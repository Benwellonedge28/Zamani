/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/calling-conventions.g4
 *
 * Grammar:
 *     InteroperabilityCallingConventions
 *
 * Status:
 *     Production-ready source-level calling-convention interoperability
 *     contract.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR 4
 *
 * Rust implementation baseline:
 *     Rust 1.97 or later
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust required or permitted by this grammar contract
 *
 * Architectural objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *     (POCO-REAF)
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns the reusable SOURCE-LEVEL SYNTAX for attaching a symbolic
 * calling-convention reference to an interoperability boundary.
 *
 * A calling convention identifies a callable interoperability contract.
 *
 * It does NOT itself define:
 *
 *     ABI layout
 *     parameter registers
 *     return registers
 *     stack layout
 *     alignment
 *     object format
 *     symbol decoration
 *     linkage
 *     ownership
 *     lifetime
 *     marshalling
 *     exception/unwind behavior
 *     target architecture
 *     target device
 *     hardware topology
 *     scheduling
 *     resource allocation
 *
 * Those concerns belong to their respective semantic, ABI, compiler, runtime,
 * and target layers.
 *
 * ============================================================================
 * 2. FUNDAMENTAL SEPARATION
 * ============================================================================
 *
 * The following concepts are intentionally distinct:
 *
 *     calling-convention syntax
 *             !=
 *     calling-convention semantic identity
 *             !=
 *     ABI contract
 *             !=
 *     FFI contract
 *             !=
 *     linkage
 *             !=
 *     target lowering
 *             !=
 *     runtime execution
 *
 * This file provides only the first item.
 *
 * ============================================================================
 * 3. ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     calling-convention syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *        ABI              interoperability
 *       analysis             analysis
 *          |                  |
 *          +--------+---------+
 *                   |
 *                   v
 *            canonical semantic model
 *                   |
 *                   v
 *              canonical IR
 *                   |
 *          +--------+---------+
 *          |        |         |
 *          v        v         v
 *      classical quantum::ir HDL/hardware
 *          |        |         |
 *          +--------+---------+
 *                   |
 *                   v
 *          target-independent
 *             optimization
 *                   |
 *                   v
 *          target lowering
 *                   |
 *                   v
 *             target realization
 *
 * This grammar MUST NOT reverse this dependency direction.
 *
 * ============================================================================
 * 4. OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     - the canonical reusable calling-convention reference syntax;
 *     - the canonical callable calling-convention attachment syntax;
 *     - the interoperability-level calling-convention adapter rules;
 *     - source-level adapters for ordinary functions;
 *     - source-level adapters for foreign functions;
 *     - source-level adapters for callbacks;
 *     - source-level source-span boundaries for this construct.
 *
 * ============================================================================
 * 5. DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexer token definitions;
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - function declaration syntax;
 *     - function type syntax;
 *     - parameter syntax;
 *     - return-type syntax;
 *     - generic syntax;
 *     - ABI declaration syntax;
 *     - ABI layout;
 *     - linkage syntax;
 *     - symbol resolution;
 *     - FFI declaration structure;
 *     - foreign-function declaration structure;
 *     - ownership semantics;
 *     - lifetime semantics;
 *     - nullability semantics;
 *     - marshalling semantics;
 *     - effect semantics;
 *     - capability semantics;
 *     - resource semantics;
 *     - policy semantics;
 *     - contracts;
 *     - target selection;
 *     - register allocation;
 *     - stack allocation;
 *     - machine instruction selection;
 *     - object generation;
 *     - linker behavior;
 *     - loader behavior;
 *     - runtime execution;
 *     - hardware discovery;
 *     - hardware placement;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL implementation.
 *
 * ============================================================================
 * 6. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The ownership chain is:
 *
 *     identifiers / qualified names
 *         -> grammar/core/names.g4
 *
 *     ordinary function declarations
 *         -> grammar/functions/
 *
 *     function-level compatibility adapter
 *         -> grammar/functions/calling-conventions.g4
 *
 *     foreign callable declarations
 *         -> grammar/interoperability/foreign-functions.g4
 *
 *     generic FFI boundary
 *         -> grammar/interoperability/ffi.g4
 *
 *     ABI contract
 *         -> grammar/interoperability/abi.g4
 *
 *     calling-convention attachment
 *         -> THIS FILE
 *
 * This file MUST NOT redefine ABI declarations.
 *
 * This file MUST NOT become a second function declaration grammar.
 *
 * This file MUST NOT become a second FFI grammar.
 *
 * This file MUST NOT become a second identifier or qualified-name grammar.
 *
 * ============================================================================
 * 7. CANONICAL LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It declares NO lexer rules.
 *
 * The canonical parser-facing lexer vocabulary is:
 *
 *     ZamaniLexer
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * MUST be used.
 *
 * This file MUST NOT introduce tokens for:
 *
 *     CALLING_CONVENTION
 *     ABI
 *     C
 *     CDECL
 *     STDCALL
 *     SYSV
 *     SYSV64
 *     WIN64
 *     AAPCS
 *     VECTORCALL
 *     THISCALL
 *     FASTCALL
 *
 * or any other current or future convention.
 *
 * Convention identities are open-world symbolic data.
 *
 * ============================================================================
 * 8. CANONICAL SHARED NAME CONTRACT
 * ============================================================================
 *
 * Name syntax is owned by:
 *
 *     grammar/core/names.g4
 *
 * This grammar imports:
 *
 *     Names
 *
 * and therefore reuses:
 *
 *     identifier
 *     qualifiedName
 *
 * It MUST NOT reproduce:
 *
 *     IDENTIFIER
 *     identifier
 *     qualifiedName
 *     nameSegment
 *     DOUBLE_COLON
 *
 * grammar/core/names.g4 remains the canonical owner of those constructs.
 *
 * ============================================================================
 * 9. OPEN-WORLD CONVENTION MODEL
 * ============================================================================
 *
 * Calling conventions are deliberately open-ended.
 *
 * The grammar MUST represent a convention without enumerating it.
 *
 * Valid examples include:
 *
 *     calling_convention = "c";
 *     calling_convention = "cdecl";
 *     calling_convention = "sysv64";
 *     calling_convention = "win64";
 *     calling_convention = "aapcs";
 *
 *     calling_convention = platform::native;
 *     calling_convention = vendor::extension::convention;
 *     calling_convention = future::calling::convention;
 *
 * A new convention therefore requires:
 *
 *     NO grammar modification
 *     NO lexer modification
 *     NO parser modification
 *
 * provided its source representation fits the existing symbolic model.
 *
 * Semantic validation determines whether the referenced convention is:
 *
 *     known
 *     supported
 *     deprecated
 *     aliased
 *     incompatible
 *     unavailable
 *     target-dependent
 *     intentionally unresolved
 *
 * ============================================================================
 * 10. STRING AND SYMBOLIC REFERENCES
 * ============================================================================
 *
 * Two source representations are supported:
 *
 *     quoted external identity
 *
 *         "c"
 *
 * and:
 *
 *     qualified symbolic identity
 *
 *         platform::native
 *
 * A quoted identity is appropriate where the external convention name is
 * not naturally a Zamani namespace symbol.
 *
 * A qualified symbolic identity is appropriate where the convention is
 * represented through a language-visible namespace.
 *
 * The grammar does not assign target-specific meaning to either representation.
 *
 * ============================================================================
 * 11. CANONICAL SOURCE FORM
 * ============================================================================
 *
 * The canonical attachment is:
 *
 *     calling_convention = "c";
 *
 * or:
 *
 *     calling_convention = platform::native;
 *
 * The assignment operator is the repository's canonical:
 *
 *     ASSIGN
 *
 * The declaration terminator is the repository's canonical:
 *
 *     SEMICOLON
 *
 * This file MUST NOT use obsolete or duplicate assignment tokens such as
 * EQUALS when the repository's canonical token is ASSIGN.
 *
 * ============================================================================
 * 12. KEY OWNERSHIP
 * ============================================================================
 *
 * The spelling:
 *
 *     calling_convention
 *
 * remains source-level metadata rather than a universal reserved keyword.
 *
 * Therefore the grammar accepts the metadata-key position as the canonical
 * identifier rule:
 *
 *     identifier
 *
 * Semantic analysis MUST verify that its canonical spelling is:
 *
 *     calling_convention
 *
 * This deliberately keeps the lexer open and avoids globally reserving a
 * keyword solely for interoperability metadata.
 *
 * A different identifier in this specific rule is therefore a semantic
 * metadata-key error, not a different calling convention.
 *
 * ============================================================================
 * 13. FUNCTION INTEGRATION
 * ============================================================================
 *
 * Ordinary function declarations remain owned by:
 *
 *     grammar/functions/
 *
 * The function grammar may consume:
 *
 *     interoperabilityFunctionCallingConvention
 *
 * at its declaration metadata/attribute boundary.
 *
 * It remains responsible for:
 *
 *     function name
 *     generic parameters
 *     parameters
 *     return type
 *     effects
 *     contracts
 *     body
 *     declaration/prototype structure
 *
 * This file contributes only the calling-convention attachment.
 *
 * ============================================================================
 * 14. FUNCTION COMPATIBILITY ADAPTER
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/functions/calling-conventions.g4
 *
 * MUST NOT become a competing implementation.
 *
 * Its long-term role is:
 *
 *     function grammar
 *          |
 *          v
 *     interoperabilityFunctionCallingConvention
 *          |
 *          v
 *     THIS FILE
 *
 * Existing source compatibility can be preserved by keeping the filename.
 *
 * The function-layer grammar should delegate its canonical convention
 * representation to this interoperability boundary when the parser
 * composition is integrated.
 *
 * ============================================================================
 * 15. FOREIGN FUNCTION INTEGRATION
 * ============================================================================
 *
 * Foreign callable declarations remain owned by:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * That grammar may consume:
 *
 *     interoperabilityForeignFunctionCallingConvention
 *
 * It MUST NOT reproduce:
 *
 *     calling_convention = ...
 *
 * independently.
 *
 * ============================================================================
 * 16. FFI INTEGRATION
 * ============================================================================
 *
 * Generic FFI remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * FFI may consume this grammar's calling-convention attachment.
 *
 * FFI remains responsible for the interoperability boundary.
 *
 * This grammar remains responsible only for the calling-convention reference.
 *
 * ============================================================================
 * 17. ABI INTEGRATION
 * ============================================================================
 *
 * ABI syntax remains owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * The semantic relationship is:
 *
 *     calling convention reference
 *              |
 *              v
 *     ABI compatibility analysis
 *              |
 *              v
 *     target ABI realization
 *
 * A calling convention does not automatically define:
 *
 *     pointer representation
 *     register allocation
 *     stack layout
 *     alignment
 *     object format
 *     symbol decoration
 *     linkage
 *     exception model
 *     unwind model
 *     parameter marshalling
 *
 * Those are separate ABI concerns.
 *
 * ============================================================================
 * 18. LINKAGE SEPARATION
 * ============================================================================
 *
 * Calling convention and linkage are independent semantic dimensions.
 *
 * For example:
 *
 *     calling_convention = "c";
 *
 * MUST NOT implicitly mean:
 *
 *     external linkage
 *     dynamic linkage
 *     static linkage
 *     weak linkage
 *     exported symbol
 *     imported symbol
 *
 * Linkage remains owned by the interoperability/ABI/function systems.
 *
 * ============================================================================
 * 19. CALLBACK INTEGRATION
 * ============================================================================
 *
 * Callback declaration structure remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/foreign-functions.g4
 *
 * as applicable.
 *
 * A callback may consume:
 *
 *     interoperabilityCallbackCallingConvention
 *
 * without defining a separate callback convention vocabulary.
 *
 * ============================================================================
 * 20. FUNCTION-TYPE SEPARATION
 * ============================================================================
 *
 * This grammar does NOT modify:
 *
 *     grammar/types/function.g4
 *
 * A declaration-level calling-convention attachment does not automatically
 * make calling convention part of every function type.
 *
 * If calling-convention metadata becomes part of a canonical function type,
 * that must be specified by the type-system authority and integrated through
 * the semantic type model.
 *
 * This grammar must not make that decision implicitly.
 *
 * ============================================================================
 * 21. GENERICS
 * ============================================================================
 *
 * Generic parameters and generic constraints remain owned by the function and
 * type grammars.
 *
 * A calling convention does not impose a generic-arity limit.
 *
 * Generic specialization, monomorphization, erasure, or other lowering is
 * compiler semantics, not grammar semantics.
 *
 * ============================================================================
 * 22. EFFECTS
 * ============================================================================
 *
 * Calling convention is NOT an effect.
 *
 * This grammar must not transform:
 *
 *     calling_convention = ...
 *
 * into an effect declaration.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * A foreign/native operation may separately carry an effect such as:
 *
 *     foreign
 *     native
 *     network
 *     IO
 *
 * without making the calling convention itself an effect.
 *
 * ============================================================================
 * 23. CAPABILITIES
 * ============================================================================
 *
 * A calling convention does not grant capabilities.
 *
 * For example:
 *
 *     calling_convention = "native";
 *
 * does NOT grant:
 *
 *     filesystem access
 *     process execution
 *     network access
 *     hardware access
 *     dynamic library loading
 *     reflection
 *
 * Capability requirements remain owned by:
 *
 *     grammar/resources/
 *     grammar/security/
 *
 * and their semantic layers.
 *
 * ============================================================================
 * 24. RESOURCES
 * ============================================================================
 *
 * Calling-convention syntax does not reserve or allocate resources.
 *
 * It must never encode:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     node count
 *     core count
 *     thread count
 *     memory capacity
 *     register count
 *     register width
 *     device count
 *     network size
 *
 * Resource requirements remain separate:
 *
 *     requires ...
 *     capability(...)
 *     resource(...)
 *
 * and are resolved downstream.
 *
 * ============================================================================
 * 25. POCO-REAF CONTRACT
 * ============================================================================
 *
 * Calling-convention metadata describes portable interoperability intent.
 *
 * It must not select:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     ASIC 0
 *     QPU 0
 *     node 0
 *     core 0
 *     thread 0
 *     register 0
 *     memory bank 0
 *     physical address
 *     physical qubit
 *
 * The same source-level convention reference may therefore participate in
 * compilation for:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future targets
 *
 * Target realization is determined downstream.
 *
 * ============================================================================
 * 26. QUANTUM INTEGRATION
 * ============================================================================
 *
 * This file contains NO quantum-specific calling-convention catalogue.
 *
 * It MUST NOT enumerate:
 *
 *     physical qubits
 *     gates
 *     coupling maps
 *     routing strategies
 *     calibration data
 *     QEC parameters
 *     QPU identifiers
 *
 * If a callable boundary participates in hybrid or quantum computation:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic interoperability analysis
 *       ->
 *     quantum semantic model
 *       ->
 *     quantum::ir
 *
 * The canonical quantum IR remains:
 *
 *     quantum::ir
 *
 * This file introduces no second quantum IR.
 *
 * ============================================================================
 * 27. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-facing calling conventions remain symbolic.
 *
 * This grammar does not define:
 *
 *     pins
 *     fixed buses
 *     physical addresses
 *     fixed register layouts
 *     universal clock widths
 *     device identifiers
 *     physical placement
 *
 * HDL and hardware semantic layers resolve those concerns.
 *
 * A calling convention may therefore participate in:
 *
 *     software <-> HDL
 *     software <-> accelerator
 *     software <-> hardware
 *
 * boundaries without introducing hardware-specific syntax here.
 *
 * ============================================================================
 * 28. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A distributed callable boundary may use a symbolic calling convention.
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/distributed/
 *     grammar/networking/
 *
 * This grammar does not define:
 *
 *     nodes
 *     endpoints
 *     routing
 *     transport protocols
 *     topology
 *     serialization
 *
 * Those are separate contracts.
 *
 * ============================================================================
 * 29. AI / DATA / DOMAIN INDEPENDENCE
 * ============================================================================
 *
 * Calling conventions are domain-neutral.
 *
 * The grammar must work equally for callable boundaries used by:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware
 *     AI
 *     data systems
 *     distributed systems
 *     networking
 *     metaprogramming
 *     dialects
 *
 * No application-specific convention vocabulary belongs here.
 *
 * ============================================================================
 * 30. AST CONTRACT
 * ============================================================================
 *
 * The preferred frontend representation is domain-neutral metadata:
 *
 *     metadata
 *         key:
 *             calling_convention
 *         value:
 *             symbolic convention reference
 *
 * The exact AST implementation is owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar must NOT require AST variants such as:
 *
 *     CpuCallingConvention
 *     GpuCallingConvention
 *     FpgaCallingConvention
 *     QpuCallingConvention
 *     HdlCallingConvention
 *     QuantumCallingConvention
 *
 * Source spans must remain available for:
 *
 *     metadata key
 *     assignment operator
 *     convention reference
 *     complete attachment
 *
 * ============================================================================
 * 31. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     1. verify the metadata key is `calling_convention`;
 *
 *     2. canonicalize the convention identity;
 *
 *     3. preserve the original source spelling for diagnostics/provenance;
 *
 *     4. resolve compatibility aliases according to language-version rules;
 *
 *     5. determine whether the convention is known or intentionally open;
 *
 *     6. validate compatibility with the callable signature;
 *
 *     7. validate compatibility with the applicable ABI;
 *
 *     8. detect conflicting calling-convention declarations;
 *
 *     9. distinguish malformed source from unsupported conventions;
 *
 *    10. distinguish unsupported targets from invalid source programs;
 *
 *    11. apply effect/capability/resource/security validation independently;
 *
 *    12. never infer a physical hardware selection from a convention name.
 *
 * Unknown convention names must not be treated as parser errors merely because
 * this grammar does not know their future semantics.
 *
 * Whether unknown conventions are accepted through compilation depends on the
 * language's interoperability/open-world policy and target validation.
 *
 * ============================================================================
 * 32. DUPLICATE-CONVENTION SEMANTICS
 * ============================================================================
 *
 * This grammar provides one attachment occurrence.
 *
 * If a broader metadata system allows multiple calling-convention metadata
 * entries at the same declaration boundary, semantic analysis must detect:
 *
 *     agreement
 *     compatible aliases
 *     conflicting declarations
 *     invalid redeclarations
 *
 * The grammar must not invent an arbitrary finite maximum.
 *
 * It must also never silently choose one conflicting convention.
 *
 * ============================================================================
 * 33. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     canonical lexer
 *     grammar version
 *     explicitly selected language configuration
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     target availability
 *     filesystem state
 *     network state
 *     environment variables
 *     installed libraries
 *     linker state
 *     runtime state
 *     scheduler state
 *     calibration state
 *     randomness
 *     wall-clock time
 *
 * ============================================================================
 * 34. SAFETY / INERTNESS
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no target-language actions;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no dynamic loading;
 *     no foreign execution;
 *     no environment inspection;
 *     no runtime callbacks.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 * 35. COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may transform:
 *
 *     symbolic convention reference
 *             |
 *             v
 *     validated semantic convention
 *             |
 *             v
 *     ABI compatibility
 *             |
 *             v
 *     target-specific lowering
 *
 * Target lowering may eventually determine:
 *
 *     parameter passing
 *     return passing
 *     register/stack strategy
 *     symbol naming
 *     linkage
 *     object format
 *     unwind behavior
 *     target-specific interoperability
 *
 * None of these decisions belong to this grammar.
 *
 * ============================================================================
 * 36. IR CONTRACT
 * ============================================================================
 *
 * This grammar does not create a separate IR.
 *
 * In particular, it must not create:
 *
 *     CallingConventionIR
 *     FunctionCallingConventionIR
 *     QuantumCallingConventionIR
 *     HardwareCallingConventionIR
 *
 * merely because the source contains calling-convention metadata.
 *
 * If a canonical IR requires calling-convention information, it belongs as
 * metadata on the canonical callable/external-call representation already
 * owned by that IR.
 *
 * ============================================================================
 * 37. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems must consume calling-convention information only after
 * semantic/compiler validation.
 *
 * Parsing this grammar does not authorize:
 *
 *     dynamic library loading
 *     symbol resolution
 *     foreign execution
 *     process execution
 *     hardware access
 *     capability acquisition
 *
 * Runtime authorization remains governed by the security, capability,
 * execution, and deployment systems.
 *
 * ============================================================================
 * 38. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The source spelling:
 *
 *     calling_convention
 *
 * is part of the metadata compatibility surface.
 *
 * Any future change to its spelling must pass through the language
 * compatibility/versioning system.
 *
 * Existing programs must not silently change meaning because a new keyword or
 * metadata spelling is introduced.
 *
 * ============================================================================
 * 39. ERROR CLASSIFICATION
 * ============================================================================
 *
 * Structural/parser errors include:
 *
 *     missing metadata key
 *     missing assignment operator
 *     missing convention reference
 *     malformed qualified name
 *     malformed string literal
 *     missing terminator where the enclosing grammar requires one
 *
 * Semantic errors include:
 *
 *     wrong metadata key
 *     unsupported convention
 *     incompatible convention
 *     conflicting convention declarations
 *     ABI incompatibility
 *     invalid callable signature
 *
 * Target/environment errors include:
 *
 *     convention unavailable on selected target
 *     required ABI unavailable
 *     required interoperability capability unavailable
 *
 * These classes must remain distinguishable.
 *
 * ============================================================================
 * 40. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite must accept the following forms when embedded at a
 * valid callable metadata boundary:
 *
 *     calling_convention = "c";
 *
 *     calling_convention = "cdecl";
 *
 *     calling_convention = "sysv64";
 *
 *     calling_convention = "win64";
 *
 *     calling_convention = "aapcs";
 *
 *     calling_convention = platform::native;
 *
 *     calling_convention = vendor::extension::convention;
 *
 *     calling_convention = future::calling::convention;
 *
 *     calling_convention = custom_convention;
 *
 *     calling_convention = custom::convention;
 *
 * Deep qualification must remain structurally supported.
 *
 * No fixed qualification depth is introduced.
 *
 * ============================================================================
 * 41. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite must reject or semantically diagnose:
 *
 *     calling_convention
 *
 *     calling_convention =
 *
 *     calling_convention = ;
 *
 *     calling_convention = platform::
 *
 *     calling_convention = ::
 *
 *     calling_convention = "a" "b";
 *
 *     calling_convention = ();
 *
 *     calling_convention = {};
 *
 *     calling_convention = wrong::;
 *
 *     calling_convention = ::wrong;
 *
 * A wrong metadata key such as:
 *
 *     wrong_key = "c";
 *
 * must not be interpreted semantically as a calling-convention declaration.
 *
 * ============================================================================
 * 42. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * The suite must exercise:
 *
 *     one convention;
 *     deeply qualified convention;
 *     long symbolic convention;
 *     quoted external convention;
 *     future convention;
 *     vendor convention;
 *     ordinary function boundary;
 *     generic function boundary;
 *     foreign function boundary;
 *     callback boundary;
 *     classical/foreign boundary;
 *     software/HDL boundary;
 *     software/accelerator boundary;
 *     distributed callable boundary;
 *     hybrid classical/quantum boundary.
 *
 * ============================================================================
 * 43. SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces no source-language ceiling on:
 *
 *     convention-name length;
 *     qualification depth;
 *     number of callable declarations;
 *     number of modules;
 *     number of functions;
 *     number of foreign functions;
 *     number of callbacks;
 *     number of parameters;
 *     number of targets;
 *     number of devices;
 *     number of processors;
 *     number of QPUs;
 *     number of nodes;
 *     memory;
 *     storage;
 *     topology;
 *     tensor rank;
 *     register width.
 *
 * Any practical limit belongs to compiler/resource policy rather than this
 * language grammar.
 *
 * "Infinity" therefore means:
 *
 *     no artificial language-defined ceiling.
 *
 * Actual execution remains bounded by available resources.
 *
 * ============================================================================
 * 44. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
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
 * It must also not encode:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     node 0
 *     register 0
 *     physical address
 *     fixed pointer width
 *     fixed word size
 *     fixed register width
 *
 * Convention names must remain symbolic.
 *
 * ============================================================================
 * 45. PROVENANCE CONTRACT
 * ============================================================================
 *
 * The frontend should preserve:
 *
 *     original convention spelling;
 *     canonical metadata key;
 *     source span;
 *     language version;
 *     compatibility context.
 *
 * Semantic resolution may additionally record:
 *
 *     canonical convention identity;
 *     alias resolution;
 *     ABI resolution;
 *     target compatibility result;
 *     diagnostics;
 *     compiler transformation provenance.
 *
 * This grammar itself does not perform provenance generation.
 *
 * ============================================================================
 * 46. TOOLING CONTRACT
 * ============================================================================
 *
 * IDE/LSP/formatter/tooling systems must be able to identify:
 *
 *     metadata key;
 *     assignment operator;
 *     convention reference;
 *     complete calling-convention attachment.
 *
 * Because convention identities are open-world, tooling must not assume that
 * an unknown convention is necessarily a syntax error.
 *
 * Tooling may provide semantic diagnostics after the semantic model resolves
 * the convention.
 *
 * ============================================================================
 * 47. INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED IMPORT:
 *
 *     grammar/core/names.g4
 *
 * Imported grammar:
 *
 *     Names
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical root parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Complete grammar root:
 *
 *     grammar/Zamani.g4
 *
 * FUNCTION CONSUMER:
 *
 *     grammar/functions/
 *
 * FUNCTION COMPATIBILITY ADAPTER:
 *
 *     grammar/functions/calling-conventions.g4
 *
 * FOREIGN FUNCTION CONSUMER:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * FFI CONSUMER:
 *
 *     grammar/interoperability/ffi.g4
 *
 * ABI CONSUMER:
 *
 *     grammar/interoperability/abi.g4
 *
 * NAME OWNER:
 *
 *     grammar/core/names.g4
 *
 * EFFECT OWNER:
 *
 *     grammar/effects/
 *
 * RESOURCE/CAPABILITY OWNER:
 *
 *     grammar/resources/
 *
 * SECURITY OWNER:
 *
 *     grammar/security/
 *
 * COMPATIBILITY OWNER:
 *
 *     grammar/compatibility/
 *
 * AST OWNER:
 *
 *     src/frontend/ast/
 *
 * SEMANTIC OWNER:
 *
 *     compiler/frontend semantic-analysis layer
 *
 * IR OWNER:
 *
 *     canonical IR / external-call representation
 *
 * QUANTUM IR:
 *
 *     quantum::ir
 *
 * TARGET LOWERING:
 *
 *     backend/target lowering layers
 *
 * ============================================================================
 * 48. INTEGRATION DIRECTION
 * ============================================================================
 *
 * The dependency direction is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Names
 *          |
 *          v
 *     InteroperabilityCallingConventions
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     function adapter       foreign-function adapter
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              domain-neutral AST
 *                     |
 *                     v
 *              semantic analysis
 *                     |
 *          +----------+-----------+
 *          |          |           |
 *          v          v           v
 *         ABI       effects    capabilities
 *          |          |           |
 *          +----------+-----------+
 *                     |
 *                     v
 *              canonical semantic
 *                  representation
 *                     |
 *                     v
 *                canonical IR
 *                     |
 *          +----------+-----------+
 *          |          |           |
 *          v          v           v
 *      classical  quantum::ir    HDL
 *          |          |           |
 *          +----------+-----------+
 *                     |
 *                     v
 *              target lowering
 *
 * No reverse dependency is permitted.
 *
 * ============================================================================
 * 49. AST / SEMANTIC / IR / BACKEND BOUNDARIES
 * ============================================================================
 *
 * AST:
 *
 *     Generic metadata or equivalent domain-neutral callable metadata.
 *
 * Semantic:
 *
 *     Symbolic convention identity and compatibility.
 *
 * ABI:
 *
 *     ABI-specific validation and realization contract.
 *
 * IR:
 *
 *     Existing canonical callable/external-call metadata representation.
 *
 * Backend:
 *
 *     Concrete calling sequence, parameter passing, register/stack decisions,
 *     object representation, and target-specific implementation.
 *
 * Runtime:
 *
 *     Validated execution only.
 *
 * ============================================================================
 * 50. RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Rust consumers must remain compatible with:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *
 * and must use safe Rust.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * Rust-specific ABI realization belongs downstream from the grammar.
 *
 * ============================================================================
 * 51. ANTLR GENERATION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar.
 *
 * Canonical declaration:
 *
 *     parser grammar InteroperabilityCallingConventions;
 *
 * Canonical lexer vocabulary:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Canonical imported grammar:
 *
 *     Names
 *
 * ANTLR generation must resolve the grammar library containing:
 *
 *     Names.g4
 *
 * and the canonical lexer vocabulary.
 *
 * This file must not require a second lexer vocabulary.
 *
 * ============================================================================
 * 52. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [x] It owns reusable interoperability calling-convention syntax.
 *
 *     [x] It does not own ABI layout.
 *
 *     [x] It does not own FFI declaration structure.
 *
 *     [x] It does not own function declaration structure.
 *
 *     [x] It reuses canonical Names grammar.
 *
 *     [x] It uses the canonical ZamaniLexer vocabulary.
 *
 *     [x] It uses the canonical ASSIGN token.
 *
 *     [x] It uses the canonical SEMICOLON token.
 *
 *     [x] It does not define lexer tokens.
 *
 *     [x] It does not enumerate calling conventions.
 *
 *     [x] Future convention names require no grammar modification.
 *
 *     [x] It has no hardware-specific calling-convention variants.
 *
 *     [x] It has no fixed hardware capacities.
 *
 *     [x] It has no register names.
 *
 *     [x] It has no physical device identifiers.
 *
 *     [x] It does not create a second ABI model.
 *
 *     [x] It does not create a second quantum IR.
 *
 *     [x] It remains domain-neutral.
 *
 *     [x] It preserves POCO-REAF.
 *
 *     [x] It separates syntax from semantic ABI resolution.
 *
 *     [x] It separates calling convention from linkage.
 *
 *     [x] It separates calling convention from effects.
 *
 *     [x] It separates calling convention from capabilities.
 *
 *     [x] It separates calling convention from resources.
 *
 *     [x] It separates calling convention from security authorization.
 *
 *     [x] It specifies AST integration.
 *
 *     [x] It specifies semantic integration.
 *
 *     [x] It specifies IR integration.
 *
 *     [x] It specifies backend integration.
 *
 *     [x] It specifies runtime boundaries.
 *
 *     [x] It specifies compatibility behavior.
 *
 *     [x] It specifies provenance requirements.
 *
 *     [x] It specifies tooling requirements.
 *
 *     [x] It specifies positive tests.
 *
 *     [x] It specifies negative tests.
 *
 *     [x] It specifies boundary tests.
 *
 *     [x] It specifies scalability tests.
 *
 *     [x] It specifies determinism tests.
 *
 *     [x] It specifies hard-coding prohibitions.
 *
 *     [x] It is compatible with safe Rust 1.97+ integration.
 *
 * ============================================================================
 * 53. GRAMMAR RULES
 * ============================================================================
 */

parser grammar InteroperabilityCallingConventions;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 53.1 SYMBOLIC CALLING-CONVENTION REFERENCE
 * ============================================================================
 *
 * Examples:
 *
 *     "c"
 *     "future-convention"
 *     platform::native
 *     vendor::extension::convention
 *
 * No convention names are enumerated.
 */
interoperabilityCallingConventionReference
    : STRING_LITERAL
    | qualifiedName
    ;


/*
 * ============================================================================
 * 53.2 CANONICAL CALLING-CONVENTION CLAUSE
 * ============================================================================
 *
 * Canonical source:
 *
 *     calling_convention = "c";
 *
 *     calling_convention = platform::native;
 *
 * The metadata key is syntactically an identifier and is validated
 * semantically as `calling_convention`.
 *
 * The enclosing declaration owns the declaration-level composition rules.
 */
interoperabilityCallingConventionClause
    : identifier
      ASSIGN
      interoperabilityCallingConventionReference
    ;


/*
 * ============================================================================
 * 53.3 COMPLETE DECLARATION-LEVEL ATTACHMENT
 * ============================================================================
 *
 * This rule is used where the interoperability composition layer owns the
 * metadata terminator.
 */
interoperabilityCallingConventionDeclaration
    : interoperabilityCallingConventionClause
      SEMICOLON
    ;


/*
 * ============================================================================
 * 53.4 ORDINARY FUNCTION ADAPTER
 * ============================================================================
 *
 * Function declaration structure remains outside this grammar.
 */
interoperabilityFunctionCallingConvention
    : interoperabilityCallingConventionClause
    ;


/*
 * ============================================================================
 * 53.5 FOREIGN FUNCTION ADAPTER
 * ============================================================================
 *
 * Foreign declaration structure remains outside this grammar.
 */
interoperabilityForeignFunctionCallingConvention
    : interoperabilityCallingConventionClause
    ;


/*
 * ============================================================================
 * 53.6 CALLBACK ADAPTER
 * ============================================================================
 *
 * Callback declaration/signature structure remains outside this grammar.
 */
interoperabilityCallbackCallingConvention
    : interoperabilityCallingConventionClause
    ;


/*
 * ============================================================================
 * 53.7 CANONICAL ATTACHMENT ALIAS
 * ============================================================================
 *
 * Provides one stable integration rule for generic interoperability
 * composition without introducing another semantic model.
 */
interoperabilityCallingConventionAttachment
    : interoperabilityCallingConventionClause
    ;


/*
 * ============================================================================
 * 53.8 SOURCE-LEVEL REFERENCE
 * ============================================================================
 *
 * This alias is useful for semantic/frontend integrations that need to
 * distinguish the reference from the complete assignment attachment.
 */
interoperabilityCallingConvention
    : interoperabilityCallingConventionReference
    ;


/*
 * ============================================================================
 * 53.9 OPTIONAL ATTACHMENT
 * ============================================================================
 *
 * This rule does not introduce a new semantic construct. It simply permits
 * callers to compose the canonical attachment optionally.
 */
interoperabilityOptionalCallingConvention
    : interoperabilityCallingConventionClause?
    ;