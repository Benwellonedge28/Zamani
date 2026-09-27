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
 *     Production source-level calling-convention interoperability contract
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     SAFE RUST ONLY
 *     NO UNSAFE
 *
 * Architectural objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *     (POCO-REAF)
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the INTEROPERABILITY-LAYER SOURCE SYNTAX for reusable
 * calling-convention references and attachments.
 *
 * It describes which symbolic calling-convention contract a callable
 * declaration requires or references.
 *
 * It does NOT implement an ABI.
 *
 * It does NOT calculate ABI layout.
 *
 * It does NOT select a target.
 *
 * It does NOT select a CPU, GPU, FPGA, ASIC, QPU, accelerator, node,
 * register, memory bank, physical address, or device.
 *
 * The distinction is fundamental:
 *
 *     calling-convention syntax
 *             !=
 *     calling-convention semantics
 *             !=
 *     ABI contract
 *             !=
 *     ABI implementation
 *             !=
 *     target lowering
 *             !=
 *     runtime execution
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
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
 *     interoperability calling-convention syntax
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-----------------------------+
 *          |              |              |
 *          v              v              v
 *        types          effects       capabilities
 *          |              |              |
 *          +--------------+--------------+
 *                         |
 *                         v
 *                 canonical semantics
 *                         |
 *                         v
 *                    ABI resolution
 *                         |
 *                         v
 *                 canonical IR boundary
 *                         |
 *          +--------------+---------------+
 *          |              |               |
 *          v              v               v
 *      classical       quantum::ir     HDL/hardware
 *          |              |               |
 *          +--------------+---------------+
 *                         |
 *                         v
 *                    optimization
 *                         |
 *                         v
 *                 routing / scheduling
 *                         |
 *                         v
 *                 resilience / QEC / ZQN
 *                         |
 *                         v
 *                         HAL
 *                         |
 *                         v
 *                  target realization
 *
 * This grammar MUST NOT reverse that dependency direction.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *   - reusable interoperability calling-convention references;
 *   - source-level calling-convention attachment syntax;
 *   - the canonical calling-convention metadata key;
 *   - symbolic/open-world convention references;
 *   - composition adapters for ordinary functions;
 *   - composition adapters for foreign functions;
 *   - source-level syntactic boundaries for calling-convention metadata.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - qualified names;
 *   - ordinary function declarations;
 *   - parameters;
 *   - return types;
 *   - function types;
 *   - effects;
 *   - contracts;
 *   - FFI marshalling;
 *   - ownership checking;
 *   - lifetime checking;
 *   - nullability semantics;
 *   - linkage semantics;
 *   - symbol resolution;
 *   - ABI declarations;
 *   - ABI layout;
 *   - register allocation;
 *   - stack layout;
 *   - calling-sequence generation;
 *   - machine instructions;
 *   - object-file generation;
 *   - linker behavior;
 *   - dynamic loading;
 *   - foreign execution;
 *   - operating-system APIs;
 *   - hardware discovery;
 *   - CPU selection;
 *   - GPU selection;
 *   - FPGA selection;
 *   - ASIC selection;
 *   - QPU selection;
 *   - accelerator selection;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - HAL;
 *   - physical placement;
 *   - resource allocation;
 *   - canonical IR definition.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * ABI contract syntax remains owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * FFI boundary syntax remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * Foreign callable declaration structure remains owned by:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * Ordinary function declaration structure remains owned by:
 *
 *     grammar/functions/functions.g4
 *
 * The existing:
 *
 *     grammar/functions/calling-conventions.g4
 *
 * remains an existing compatibility/function-layer adapter and MUST NOT be
 * unnecessarily renamed.
 *
 * This file does NOT recreate:
 *
 *     abiDeclaration
 *     abiConventionDeclaration
 *     abiProfileDeclaration
 *     abiLinkageDeclaration
 *
 * nor does it create a second ABI model.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Calling conventions are interoperability concepts and therefore need a
 * stable home beneath the interoperability composition boundary.
 *
 * The existing function-level calling-convention grammar is useful as a
 * compatibility adapter, but the interoperability subsystem needs its own
 * canonical reusable representation so that:
 *
 *     ordinary functions
 *     foreign functions
 *     callbacks
 *     adapters
 *     cross-language boundaries
 *     software/HDL boundaries
 *     classical/quantum boundaries
 *
 * can reference the same semantic concept without duplicating grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It declares NO lexer rules.
 *
 * It MUST NOT introduce dedicated lexer tokens for:
 *
 *     CALLING
 *     CONVENTION
 *     ABI
 *     C
 *     CDECL
 *     STDCALL
 *     SYSV
 *     WIN64
 *     AAPCS
 *     VECTORCALL
 *     GPU
 *     QPU
 *
 * Calling-convention names are data, not a closed language enumeration.
 *
 * ============================================================================
 * TOKEN-VOCABULARY CONTRACT
 * ============================================================================
 *
 * The repository currently contains legacy modular parser delegates using:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * while the production architecture identifies:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * as the canonical lexer boundary.
 *
 * This file follows the existing modular parser-delegate convention:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * It MUST NOT create another lexical vocabulary.
 *
 * The eventual repository-wide migration from ZamaniTokens to the canonical
 * parser-facing ZamaniLexer vocabulary belongs to the grammar composition and
 * lexical-conformance work, not to this file.
 *
 * ============================================================================
 * SHARED GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * Only the canonical name grammar is imported here.
 *
 * This file intentionally does NOT import:
 *
 *     Types
 *     Expressions
 *     Functions
 *     FFI
 *     ABI
 *
 * because doing so would create ownership duplication and unnecessary grammar
 * cycles.
 *
 * The convention reference is deliberately limited to:
 *
 *     STRING_LITERAL
 *
 * or:
 *
 *     qualifiedName
 *
 * ============================================================================
 * OPEN-WORLD REQUIREMENT
 * ============================================================================
 *
 * Calling conventions MUST be open-world.
 *
 * The grammar MUST NOT contain a closed alternative such as:
 *
 *     c
 *     cdecl
 *     stdcall
 *     fastcall
 *     thiscall
 *     sysv64
 *     win64
 *     aapcs
 *     vectorcall
 *
 * A future convention must be representable without modifying this grammar.
 *
 * Examples:
 *
 *     "c"
 *     "cdecl"
 *     "sysv64"
 *     "future-convention"
 *
 *     platform::native
 *     vendor::extension::convention
 *     future::architecture::convention
 *
 * These are symbolic values.
 *
 * They are NOT grammar alternatives.
 *
 * ============================================================================
 * CANONICAL SOURCE FORM
 * ============================================================================
 *
 * The canonical reusable clause is:
 *
 *     calling_convention = "c"
 *
 * or:
 *
 *     calling_convention = platform::native
 *
 * The key intentionally remains an ordinary identifier because
 * calling_convention is not required to become a global reserved keyword.
 *
 * Semantic analysis MUST canonicalize the key to:
 *
 *     calling_convention
 *
 * and reject unrelated metadata keys when this rule is being consumed as a
 * calling-convention clause.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * Ordinary function declarations remain owned by:
 *
 *     grammar/functions/functions.g4
 *
 * That grammar should consume:
 *
 *     interoperabilityCallingConventionClause
 *
 * at its metadata boundary.
 *
 * The function grammar remains responsible for:
 *
 *     function name
 *     generic parameters
 *     parameters
 *     return type
 *     effects
 *     contracts
 *     body
 *     prototype/definition structure
 *
 * This file contributes only the calling-convention attachment.
 *
 * ============================================================================
 * FOREIGN FUNCTION INTEGRATION
 * ============================================================================
 *
 * Foreign function declaration structure remains owned by:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * That grammar may consume:
 *
 *     interoperabilityForeignFunctionCallingConvention
 *
 * without reproducing the calling-convention syntax.
 *
 * This avoids:
 *
 *     foreign-functions.g4
 *             +
 *     functions/calling-conventions.g4
 *             +
 *     interoperability/calling-conventions.g4
 *
 * becoming three competing implementations.
 *
 * The canonical interoperability composition layer should expose exactly one
 * semantic calling-convention construct.
 *
 * ============================================================================
 * ABI INTEGRATION
 * ============================================================================
 *
 * The source-to-target flow is:
 *
 *     interoperabilityCallingConventionClause
 *                  |
 *                  v
 *          domain-neutral AST
 *                  |
 *                  v
 *          semantic ABI reference
 *                  |
 *                  v
 *          ABI contract validation
 *                  |
 *                  v
 *          target ABI resolution
 *                  |
 *                  v
 *          concrete target lowering
 *
 * This file never invokes ABI resolution.
 *
 * ============================================================================
 * ABI DISTINCTION
 * ============================================================================
 *
 * The following are distinct concepts:
 *
 *     calling convention
 *     ABI identity
 *     linkage
 *     symbol naming
 *     representation
 *     parameter passing
 *     return passing
 *     ownership
 *     lifetime
 *     marshalling
 *
 * A calling convention MUST NOT silently define the others.
 *
 * For example:
 *
 *     calling_convention = "c"
 *
 * does not itself specify:
 *
 *     object format
 *     symbol decoration
 *     pointer width
 *     alignment
 *     register allocation
 *     stack layout
 *     exception/unwind behavior
 *
 * Those are downstream ABI concerns.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Calling-convention syntax describes portable interoperability intent.
 *
 * It MUST NOT select physical hardware.
 *
 * It MUST NOT imply:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     node 0
 *     core 0
 *     thread 0
 *     register 0
 *     memory bank 0
 *     physical address
 *     physical qubit
 *
 * It MUST NOT encode:
 *
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_DEVICE_COUNT
 *
 * A calling convention is a compatibility requirement, not a hardware
 * capacity declaration.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar has no language-level finite limit on:
 *
 *     functions
 *     foreign functions
 *     callbacks
 *     parameters
 *     convention names
 *     ABI profiles
 *     interfaces
 *     domains
 *     targets
 *     devices
 *     nodes
 *     cores
 *     threads
 *     qubits
 *     memory
 *
 * Repetition is structural.
 *
 * No artificial language ceiling is introduced here.
 *
 * Any practical parser/compiler limit must be represented as implementation
 * resource policy rather than as source-language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     canonical token stream
 *     grammar version
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     filesystem state
 *     network state
 *     environment variables
 *     installed libraries
 *     linker state
 *     runtime state
 *     target discovery
 *     scheduler state
 *     calibration state
 *     randomness
 *     wall-clock time
 *
 * ============================================================================
 * SAFETY / INERTNESS
 * ============================================================================
 *
 * This grammar is declarative only.
 *
 * Parsing MUST NOT:
 *
 *     open files
 *     access networks
 *     inspect hardware
 *     load libraries
 *     resolve symbols
 *     invoke foreign functions
 *     execute processes
 *     access secrets
 *     query target capabilities
 *
 * The compiler implementation consuming this grammar must remain:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST map to the existing domain-neutral AST.
 *
 * Preferred representation:
 *
 *     metadata/attribute
 *         key:
 *             calling_convention
 *         value:
 *             symbolic convention reference
 *
 * The exact AST type is owned by:
 *
 *     src/frontend/ast/
 *
 * This file MUST NOT require ABI-specific AST variants such as:
 *
 *     CpuCallingConvention
 *     GpuCallingConvention
 *     FpgaCallingConvention
 *     QpuCallingConvention
 *     HdlCallingConvention
 *     QuantumCallingConvention
 *
 * Source spans MUST be preserved for:
 *
 *     metadata key
 *     convention reference
 *     complete clause
 *
 * so diagnostics, IDE tooling, source mapping, and deterministic compilation
 * remain possible.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntax only.
 *
 * Semantic analysis MUST:
 *
 *     1. verify that the metadata key is calling_convention;
 *     2. canonicalize the convention reference;
 *     3. resolve compatibility aliases according to language-version policy;
 *     4. validate the convention against the applicable ABI;
 *     5. validate compatibility with the callable signature;
 *     6. detect conflicting convention declarations;
 *     7. distinguish unknown conventions from malformed source;
 *     8. distinguish unsupported targets from invalid source;
 *     9. apply capability/resource/effect/security rules independently;
 *    10. prevent a convention declaration from silently selecting hardware.
 *
 * Open-world convention resolution is a semantic/compiler concern.
 *
 * ============================================================================
 * DUPLICATE-CONVENTION RULE
 * ============================================================================
 *
 * This grammar intentionally permits one syntactic attachment at each
 * attachment point.
 *
 * It does not encode an arbitrary list of alternative conventions.
 *
 * If a source declaration contains multiple calling-convention metadata items
 * through a broader metadata mechanism, semantic analysis MUST determine
 * whether they:
 *
 *     agree;
 *     are compatible aliases;
 *     conflict;
 *     or are invalid.
 *
 * The grammar must not invent a finite conflict-resolution policy.
 *
 * ============================================================================
 * FUNCTION-TYPE RULE
 * ============================================================================
 *
 * This file does NOT make calling conventions part of every function type.
 *
 * A declaration-level convention:
 *
 *     calling_convention = ...
 *
 * does not automatically change:
 *
 *     grammar/types/function.g4
 *
 * If function-type ABI metadata is eventually standardized, that must be
 * specified independently by the canonical type-system contract.
 *
 * ============================================================================
 * CALLBACK INTEGRATION
 * ============================================================================
 *
 * A callback crossing an interoperability boundary may use the same semantic
 * calling-convention reference.
 *
 * This file does not define callback signatures.
 *
 * Callback structure remains owned by the applicable FFI/foreign-function
 * grammar.
 *
 * The semantic model associates:
 *
 *     callback signature
 *     calling convention
 *     ABI
 *     ownership
 *     lifetime
 *     effects
 *     capabilities
 *
 * without creating a callback-specific calling-convention grammar.
 *
 * ============================================================================
 * C / C++ / RUST / ZIG / PYTHON
 * ============================================================================
 *
 * Language-specific interoperability grammars remain responsible for their
 * own language boundary syntax:
 *
 *     c.g4
 *     cpp.g4
 *     rust.g4
 *     zig.g4
 *     python.g4
 *
 * This file does NOT enumerate their calling conventions.
 *
 * Examples such as:
 *
 *     calling_convention = "C"
 *     calling_convention = "Rust"
 *     calling_convention = "cdecl"
 *
 * are symbolic source data.
 *
 * Future foreign languages and future conventions remain representable
 * without changing this grammar.
 *
 * ============================================================================
 * ASSEMBLY INTEGRATION
 * ============================================================================
 *
 * Assembly interoperability may require target-specific calling conventions.
 *
 * This file still represents the convention symbolically.
 *
 * AssemblyLanguage.g4 owns assembly-language syntax.
 *
 * Target-specific instruction-set details remain downstream.
 *
 * ============================================================================
 * OPENQASM / QUANTUM INTEGRATION
 * ============================================================================
 *
 * A callable boundary involving quantum computation may carry calling-
 * convention metadata.
 *
 * This file does NOT define:
 *
 *     qubits
 *     quantum operations
 *     gates
 *     physical qubits
 *     topology
 *     calibration
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *
 * Quantum computation continues through the canonical:
 *
 *     quantum::ir
 *
 * path.
 *
 * No quantum calling-convention IR is introduced.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A calling convention may annotate a software/hardware callable boundary.
 *
 * It does NOT define:
 *
 *     pins
 *     wires
 *     buses
 *     register addresses
 *     fixed widths
 *     clock counts
 *     pipeline counts
 *     physical placement
 *
 * HDL/hardware grammars retain ownership of those source concepts.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Calling-convention metadata does NOT grant capabilities.
 *
 * For example:
 *
 *     calling_convention = "native"
 *
 * MUST NOT automatically grant:
 *
 *     filesystem
 *     network
 *     process
 *     hardware
 *     quantum
 *     foreign-library
 *
 * capabilities.
 *
 * Capabilities remain explicit semantic requirements.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Calling convention is distinct from:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     budget
 *     capability
 *
 * For example:
 *
 *     calling_convention = "c"
 *
 * does not mean:
 *
 *     requires cpu
 *
 * and:
 *
 *     requires capability::foreign_call
 *
 * does not mean:
 *
 *     calling_convention = "c"
 *
 * unless the semantic specification explicitly establishes that relationship.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Calling convention is NOT an effect.
 *
 * A foreign call may separately have:
 *
 *     IO effects
 *     network effects
 *     process effects
 *     hardware effects
 *     quantum effects
 *     asynchronous effects
 *
 * Those effects remain owned by the canonical effect system.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * It MUST NOT introduce:
 *
 *     CallingConventionIR
 *     FunctionCallingConventionIR
 *     ForeignCallingConventionIR
 *     QuantumCallingConventionIR
 *     HardwareCallingConventionIR
 *
 * If an existing canonical IR needs calling-convention metadata, that metadata
 * belongs on the existing callable/external-symbol/function representation.
 *
 * Quantum lowering remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Downstream compilation may transform:
 *
 *     symbolic convention
 *          ->
 *     validated ABI identity
 *          ->
 *     target ABI
 *          ->
 *     concrete parameter/return lowering
 *
 * Target lowering may determine:
 *
 *     register/stack use
 *     parameter passing
 *     return passing
 *     symbol decoration
 *     object format
 *     unwind behavior
 *     target-specific interoperability
 *
 * None of these are grammar responsibilities.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime code MUST consume already validated compiler/semantic output.
 *
 * A raw parser value is never authorization to:
 *
 *     load a library;
 *     resolve a symbol;
 *     execute foreign code;
 *     access hardware.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * A calling convention MUST NOT be treated as a capability.
 *
 * The following conceptual transformation is forbidden:
 *
 *     calling_convention = "native"
 *                |
 *                +--> unrestricted native execution
 *
 * The correct model is:
 *
 *     calling_convention
 *          |
 *          v
 *     interoperability contract
 *
 * while:
 *
 *     capability requirement
 *          |
 *          v
 *     security authorization
 *
 * remains a separate semantic path.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The existing:
 *
 *     grammar/functions/calling-conventions.g4
 *
 * MUST NOT be unnecessarily renamed.
 *
 * Existing rule names there remain compatibility surfaces until the repository
 * completes the planned migration.
 *
 * The interoperability composition layer SHOULD migrate function-level
 * consumers to:
 *
 *     interoperabilityCallingConventionClause
 *
 * through normal compatibility work rather than duplicating grammar.
 *
 * Existing source spellings must not silently change meaning.
 *
 * Any spelling migration belongs in:
 *
 *     grammar/compatibility/
 *
 * and:
 *
 *     grammar/spec/compatibility.md
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical interoperability composition grammar:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * SHOULD import:
 *
 *     InteroperabilityCallingConventions
 *
 * and expose:
 *
 *     interoperabilityCallingConventionClause
 *
 * through its interoperability metadata dispatcher.
 *
 * The ordinary function composition grammar:
 *
 *     grammar/functions/functions.g4
 *
 * SHOULD import this grammar, or receive the rule through its established
 * composition hierarchy, and consume:
 *
 *     interoperabilityFunctionCallingConvention
 *
 * at the function metadata boundary.
 *
 * The foreign-function grammar:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * SHOULD consume:
 *
 *     interoperabilityForeignFunctionCallingConvention
 *
 * rather than duplicating the production.
 *
 * This produces:
 *
 *     one source syntax
 *          |
 *          +--> ordinary function
 *          |
 *          +--> foreign function
 *          |
 *          +--> callback
 *          |
 *          +--> interoperability boundary
 *          |
 *          v
 *     one semantic calling-convention contract
 *
 * ============================================================================
 * NO COMPOSITION CYCLE
 * ============================================================================
 *
 * The dependency direction MUST remain:
 *
 *     InteroperabilityCallingConventions
 *             |
 *             v
 *     interoperability composition
 *             |
 *             v
 *     canonical parser composition
 *
 * This file MUST NOT import:
 *
 *     ZamaniParser
 *     Zamani.g4
 *
 * and MUST NOT depend on a higher-level parser grammar.
 *
 * ============================================================================
 * CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * Positive syntactic examples:
 *
 *     calling_convention = "c"
 *
 *     calling_convention = "cdecl"
 *
 *     calling_convention = platform::native
 *
 *     calling_convention = vendor::extension::convention
 *
 *     calling_convention = future::architecture::convention
 *
 *     calling_convention = "any-future-convention"
 *
 * These are examples of syntax only.
 *
 * Semantic availability is determined downstream.
 *
 * ============================================================================
 * NEGATIVE / SEMANTIC EXAMPLES
 * ============================================================================
 *
 * These must be rejected or diagnosed semantically where the surrounding
 * grammar permits them syntactically:
 *
 *     wrong_key = "c"
 *
 *     calling_convention =
 *
 *     calling_convention = platform::
 *
 *     calling_convention = ""
 *
 *     calling_convention = "c" calling_convention = "other"
 *
 * The grammar does not need to encode every semantic error.
 *
 * Structural syntax and semantic validity remain separate.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * The conformance suite must exercise:
 *
 *     one convention reference;
 *     deeply qualified convention reference;
 *     long symbolic convention reference;
 *     quoted external convention identity;
 *     generic function containing the clause;
 *     foreign function containing the clause;
 *     callback containing the clause;
 *     classical/foreign boundary;
 *     quantum/classical boundary;
 *     software/HDL boundary;
 *     hardware/software boundary;
 *     distributed/service boundary.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Scalability tests must verify that no source-language limit is introduced
 * by this grammar.
 *
 * Tests should vary:
 *
 *     number of declarations;
 *     qualified-name depth;
 *     source size;
 *     generic nesting;
 *     number of independent callable boundaries.
 *
 * Tests MUST NOT establish a language maximum such as:
 *
 *     MAX_CALLING_CONVENTIONS
 *     MAX_ABI_PROFILES
 *     MAX_FUNCTIONS
 *     MAX_PARAMETERS
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse the same source repeatedly and verify identical parse acceptance and
 * equivalent parse-tree structure.
 *
 * Parsing must not change when:
 *
 *     hardware changes;
 *     target availability changes;
 *     libraries are installed/removed;
 *     network connectivity changes;
 *     environment variables change;
 *     runtime state changes.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
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
 * Also forbidden as universal grammar semantics:
 *
 *     register RAX
 *     register X0
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     physical_qubit 0
 *     fixed_pointer_width
 *     fixed_word_size
 *     fixed_register_width
 *
 * Numeric literals remain program data when supplied by other grammar
 * components. This file introduces none.
 *
 * ============================================================================
 * PRODUCTION COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is considered complete as an independent grammar contract when:
 *
 * [x] Purpose is defined.
 * [x] Ownership is defined.
 * [x] Non-ownership is defined.
 * [x] Lexer contract is defined.
 * [x] Shared grammar dependencies are defined.
 * [x] Open-world convention model is defined.
 * [x] No finite convention enumeration exists.
 * [x] Function integration is defined.
 * [x] Foreign-function integration is defined.
 * [x] ABI integration is defined.
 * [x] FFI separation is defined.
 * [x] AST contract is defined.
 * [x] Semantic contract is defined.
 * [x] IR contract is defined.
 * [x] Quantum integration is defined.
 * [x] HDL/hardware integration is defined.
 * [x] Capability separation is defined.
 * [x] Resource separation is defined.
 * [x] Effect separation is defined.
 * [x] Compiler contract is defined.
 * [x] Runtime contract is defined.
 * [x] Security/inertness contract is defined.
 * [x] Compatibility contract is defined.
 * [x] Composition direction is defined.
 * [x] No grammar cycle is introduced.
 * [x] No hardware limits are introduced.
 * [x] No second quantum IR is introduced.
 * [x] Rust 1.97 / 1.97.1 compatibility is defined.
 * [x] Safe Rust / no unsafe requirement is preserved.
 * [x] Positive tests are defined.
 * [x] Negative tests are defined.
 * [x] Boundary tests are defined.
 * [x] Scalability tests are defined.
 * [x] Determinism tests are defined.
 * [x] Hard-coding audit is defined.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar InteroperabilityCallingConventions;

options {
    tokenVocab = ZamaniTokens;
}

import Names;


/*
 * ============================================================================
 * 1. CALLING-CONVENTION REFERENCE
 * ============================================================================
 *
 * A convention is symbolic and open-world.
 *
 * Examples:
 *
 *     "c"
 *     "cdecl"
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
 * 2. CANONICAL CALLING-CONVENTION CLAUSE
 * ============================================================================
 *
 * Canonical source form:
 *
 *     calling_convention = "c"
 *
 * or:
 *
 *     calling_convention = platform::native
 *
 * No semicolon is included because the enclosing declaration owns its
 * terminator.
 *
 * IDENTIFIER is deliberately used for the metadata key because the repository
 * does not currently require calling_convention to be a globally reserved
 * lexer keyword.
 *
 * Semantic analysis canonicalizes the key.
 */
interoperabilityCallingConventionClause
    : IDENTIFIER
      EQUALS
      interoperabilityCallingConventionReference
    ;


/*
 * ============================================================================
 * 3. COMPLETE METADATA ITEM
 * ============================================================================
 *
 * This rule is useful where an interoperability composition grammar needs a
 * complete metadata item with its own terminator.
 */
interoperabilityCallingConventionDeclaration
    : interoperabilityCallingConventionClause
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. ORDINARY FUNCTION ADAPTER
 * ============================================================================
 *
 * The complete function declaration remains owned by functions/functions.g4.
 *
 * This rule provides a stable integration name without duplicating the
 * calling-convention clause.
 */
interoperabilityFunctionCallingConvention
    : interoperabilityCallingConventionClause
    ;


/*
 * ============================================================================
 * 5. FOREIGN-FUNCTION ADAPTER
 * ============================================================================
 *
 * The complete foreign-function declaration remains owned by
 * interoperability/foreign-functions.g4.
 */
interoperabilityForeignFunctionCallingConvention
    : interoperabilityCallingConventionClause
    ;


/*
 * ============================================================================
 * 6. CALLBACK ADAPTER
 * ============================================================================
 *
 * Callback signature structure remains owned by FFI/foreign-function grammar.
 *
 * This rule only supplies the reusable convention attachment.
 */
interoperabilityCallbackCallingConvention
    : interoperabilityCallingConventionClause
    ;


/*
 * ============================================================================
 * 7. ATTACHMENT ALIAS
 * ============================================================================
 *
 * A single effective calling-convention attachment is the semantic model for
 * a callable declaration.
 *
 * Conflicts introduced through broader metadata composition are semantic
 * diagnostics, not a reason to introduce grammar-level fixed limits.
 */
interoperabilityCallingConventionAttachment
    : interoperabilityCallingConventionClause
    ;


/*
 * ============================================================================
 * 8. REFERENCE LIST FOR TOOLING
 * ============================================================================
 *
 * This helper is intentionally structural.
 *
 * It does NOT define how multiple convention references are combined.
 *
 * Semantic analysis owns compatibility/conflict resolution.
 */
interoperabilityCallingConventionReferenceList
    : interoperabilityCallingConventionReference
      (
          COMMA
          interoperabilityCallingConventionReference
      )*
    ;


/*
 * ============================================================================
 * 9. END-TO-END EXAMPLE CONTRACT
 * ============================================================================
 *
 * Example enclosing source:
 *
 *     fn compute(value: T) -> R
 *         calling_convention = vendor::convention
 *     ;
 *
 * This file recognizes only:
 *
 *     calling_convention = vendor::convention
 *
 * The enclosing function grammar recognizes the complete declaration.
 *
 * ============================================================================
 */