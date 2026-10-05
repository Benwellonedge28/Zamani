/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/effects/foreign.g4
 *
 * GRAMMAR
 * -------
 * ForeignEffects
 *
 * STATUS
 * ------
 * CANONICAL FOREIGN-EFFECT DOMAIN ADAPTER
 *
 * LANGUAGE / RUNTIME BASELINE
 * ---------------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no runtime execution;
 *     - no hardware discovery;
 *     - no dynamic library loading;
 *     - no symbol resolution;
 *     - no native-memory access;
 *     - no unsafe Rust.
 *
 * Parsing is declarative and inert.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the EFFECT-SYSTEM ADAPTER for effects whose eventual semantic
 * realization may cross a foreign interoperability boundary.
 *
 * IMPORTANT:
 *
 * This file does NOT define the FFI boundary itself.
 *
 * FFI ownership remains:
 *
 *     grammar/interoperability/ffi.g4
 *
 * Foreign callable declaration ownership remains:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * Generic foreign-function declaration ownership also remains:
 *
 *     grammar/functions/foreign-functions.g4
 *
 * ABI ownership remains:
 *
 *     grammar/interoperability/abi.g4
 *
 * Calling-convention ownership remains:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * This file exists because EFFECT and FFI are related but distinct concepts.
 *
 * EFFECT
 *     Describes observable computational behavior.
 *
 * FFI
 *     Describes a source-level interoperability boundary.
 *
 * ABI
 *     Describes compatibility of callable/data representations.
 *
 * FOREIGN EFFECT
 *     Describes an effect whose semantic implementation may cross an
 *     interoperability boundary.
 *
 * The relationship is semantic.
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
 *     generic effect syntax
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     ordinary effects      foreign-effect adapter
 *                                 |
 *                                 v
 *                        domain-neutral frontend AST
 *                                 |
 *                                 v
 *                         semantic analysis
 *                                 |
 *             +-------------------+-------------------+
 *             |                   |                   |
 *             v                   v                   v
 *          effects             FFI/ABI          capabilities
 *                                 |
 *                                 v
 *                         resource analysis
 *                                 |
 *                                 v
 *                       canonical semantic model
 *                                 |
 *              +------------------+------------------+
 *              |                  |                  |
 *              v                  v                  v
 *        classical IR       quantum::ir       HDL/hardware
 *                                 |
 *                                 v
 *                        optimization/lowering
 *                                 |
 *                                 v
 *                       routing/scheduling
 *                                 |
 *                                 v
 *                         resilience / QEC
 *                                 |
 *                                 v
 *                                ZQN
 *                                 |
 *                                 v
 *                                HAL
 *                                 |
 *                                 v
 *                         target realization
 *
 * The dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     foreignEffect
 *     foreignEffectReference
 *     foreignEffectOperationReference
 *     foreignEffectOperation
 *     foreignEffectInvocation
 *     foreignEffectSet
 *     foreignEffectOperationSet
 *
 * These are effect-domain adapter rules.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *     effectHandling
 *     effect handlers
 *     FFI declarations
 *     FFI calls
 *     ABI declarations
 *     ABI layouts
 *     calling conventions
 *     linkage
 *     foreign types
 *     foreign symbols
 *     expressions
 *     identifiers
 *     qualified names
 *     capabilities
 *     resources
 *     requirements
 *     policies
 *     security
 *     runtime dispatch
 *     target selection
 *     quantum routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Generic effect syntax remains owned by:
 *
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *
 * Therefore this file MUST NOT redefine:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectInvocationArguments
 *     effectOperationCall
 *     effectOperationUse
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectSetBody
 *
 * This file merely wraps those canonical constructs with stable foreign-effect
 * adapter rule names.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * A foreign function can have observable effects.
 *
 * For example, an external callable may:
 *
 *     perform IO;
 *     access a network;
 *     mutate state;
 *     communicate with another process;
 *     invoke a hardware facility;
 *     invoke an accelerator;
 *     interact with a quantum runtime;
 *     invoke a simulator;
 *     perform distributed communication;
 *     access a foreign runtime;
 *     produce an externally observable event.
 *
 * None of those facts should require the generic effect grammar to know:
 *
 *     - which foreign language;
 *     - which library;
 *     - which ABI implementation;
 *     - which linker;
 *     - which operating system;
 *     - which CPU;
 *     - which GPU;
 *     - which FPGA;
 *     - which ASIC;
 *     - which QPU;
 *     - which physical device;
 *     - which network node;
 *     - which physical address.
 *
 * The effect remains an open semantic identity.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Foreign effects are OPEN-WORLD.
 *
 * The grammar MUST NOT enumerate:
 *
 *     C
 *     C++
 *     Rust
 *     Python
 *     Fortran
 *     Java
 *     SystemVerilog
 *     Verilog
 *     OpenQASM
 *     CUDA
 *     OpenCL
 *     POSIX
 *     Windows
 *     Linux
 *     vendor names
 *     library names
 *     device names
 *     ABI names
 *     calling conventions
 *
 * as effect alternatives.
 *
 * Those identities belong to semantic metadata and interoperability
 * contracts.
 *
 * A qualified effect name may therefore be arbitrarily extended by semantic
 * domains.
 *
 * Examples of possible semantic identities:
 *
 *     foreign::call
 *     foreign::runtime
 *     foreign::service
 *     foreign::callback
 *     foreign::transport
 *     foreign::device
 *     foreign::quantum
 *     foreign::hardware
 *     foreign::vendor::operation
 *     organization::extension::effect
 *     future::interop::operation
 *
 * These are examples of names, not reserved grammar vocabulary.
 *
 * ============================================================================
 * CRITICAL DISTINCTION
 * ============================================================================
 *
 * The presence of this adapter MUST NOT be interpreted as proof that a
 * particular effect is actually foreign.
 *
 * For example:
 *
 *     foreignEffectReference
 *         : effectReference
 *         ;
 *
 * only establishes the parser structure.
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the referenced effect exists;
 *     - whether it is an effect;
 *     - whether it is foreign;
 *     - which interoperability contract applies;
 *     - which ABI applies;
 *     - which capabilities are required;
 *     - which resources are required;
 *     - which policies apply;
 *     - whether the operation is authorized;
 *     - whether the target can realize it.
 *
 * The parser MUST NOT make those decisions.
 *
 * ============================================================================
 * EFFECT / FFI / ABI SEPARATION
 * ============================================================================
 *
 * The following relationship is normative:
 *
 *     effect
 *        |
 *        +--> semantic behavior
 *        |
 *        +--> may require capability
 *        |
 *        +--> may require resources
 *        |
 *        +--> may cross FFI
 *                     |
 *                     +--> ABI
 *                     |
 *                     +--> linkage
 *                     |
 *                     +--> foreign implementation
 *
 * FFI does not automatically mean a new effect.
 *
 * A foreign function may be semantically pure.
 *
 * Conversely, an ordinary-looking effect may eventually be realized through
 * a foreign implementation.
 *
 * Therefore:
 *
 *     foreign boundary != effect identity
 *
 * and:
 *
 *     effect identity != ABI identity
 *
 * ============================================================================
 * NATIVE EFFECT SEPARATION
 * ============================================================================
 *
 * grammar/effects/native.g4 already owns the native-effect adapter.
 *
 * This file MUST NOT duplicate or replace it.
 *
 * Native and foreign are semantically related but distinct:
 *
 *     native effect
 *         realization may use a target/runtime-native facility
 *
 *     foreign effect
 *         realization may cross an interoperability boundary
 *
 * A foreign effect may eventually use native machinery.
 *
 * A native effect may eventually cross an FFI boundary.
 *
 * These relationships are resolved semantically.
 *
 * ============================================================================
 * FFI INTEGRATION
 * ============================================================================
 *
 * FFI remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * The FFI grammar owns:
 *
 *     - FFI interfaces;
 *     - FFI bindings;
 *     - FFI callbacks;
 *     - FFI adapters;
 *     - FFI policies;
 *     - FFI contracts;
 *     - FFI boundary metadata;
 *     - foreign call syntax.
 *
 * This grammar MUST NOT import Ffi.
 *
 * Reason:
 *
 *     Effects
 *        |
 *        +--> EffectOperations
 *        |
 *        +--> EffectSets
 *
 * while:
 *
 *     Interoperability
 *        |
 *        +--> Ffi
 *
 * Mixing those composition roots would unnecessarily couple the generic effect
 * subsystem to interoperability and could introduce dependency cycles as the
 * repository grows.
 *
 * The relationship is therefore semantic rather than an import dependency.
 *
 * ============================================================================
 * FOREIGN FUNCTION INTEGRATION
 * ============================================================================
 *
 * Foreign callable declarations remain owned by:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/foreign-functions.g4
 *
 * This grammar does NOT redefine:
 *
 *     foreignFunctionDeclaration
 *     foreignFunction
 *     interoperabilityForeignFunctionDeclaration
 *     interoperabilityForeignFunctionMember
 *
 * A foreign function's effect attachment remains an ordinary effect-system
 * semantic contract.
 *
 * ============================================================================
 * ABI INTEGRATION
 * ============================================================================
 *
 * ABI syntax remains owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * This grammar does not define:
 *
 *     calling convention
 *     parameter ABI
 *     return ABI
 *     data layout
 *     alignment
 *     register assignment
 *     stack layout
 *     object format
 *     symbol mangling
 *     linkage implementation
 *
 * Those are downstream interoperability and lowering concerns.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A foreign effect may imply or require semantic capabilities.
 *
 * Examples:
 *
 *     foreign::call
 *     foreign::callback
 *     foreign::remote
 *     foreign::quantum
 *     foreign::hardware
 *
 * The grammar MUST NOT determine whether such capabilities exist.
 *
 * Capability analysis belongs to the resource/capability semantic layer.
 *
 * The distinction remains:
 *
 *     effect
 *         what computation may do
 *
 *     capability
 *         what the realization environment can provide
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * This grammar MUST NOT define physical resource limits.
 *
 * It does not own:
 *
 *     memory capacity
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     node count
 *     thread count
 *     network size
 *     device count
 *     register width
 *     pointer width
 *     bus width
 *     tensor rank
 *     qubit count
 *
 * A foreign effect may semantically carry resource requirements through the
 * general resource system.
 *
 * For example, semantic analysis may derive:
 *
 *     requires capability("foreign.call");
 *     requires memory >= required_memory;
 *
 * or another target-independent requirement.
 *
 * This grammar does not hard-code those values.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Foreign effects may be constrained by:
 *
 *     security policy
 *     execution policy
 *     deployment policy
 *     resource policy
 *     adaptation policy
 *     sandbox policy
 *     interoperability policy
 *
 * This grammar does not evaluate those policies.
 *
 * Policy evaluation occurs after parsing.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Parsing a foreign effect MUST NOT:
 *
 *     - load foreign code;
 *     - execute foreign code;
 *     - resolve symbols;
 *     - inspect the host;
 *     - inspect hardware;
 *     - inspect the network;
 *     - dereference pointers;
 *     - allocate native resources;
 *     - bypass capability checks;
 *     - bypass sandboxing;
 *     - bypass provenance;
 *     - bypass effect checking.
 *
 * Foreign execution is an explicitly controlled downstream operation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar introduces no source-language machine ceilings.
 *
 * It MUST NOT define:
 *
 *     MAX_FOREIGN_EFFECTS
 *     MAX_FOREIGN_OPERATIONS
 *     MAX_FOREIGN_ARGUMENTS
 *     MAX_FOREIGN_INTERFACES
 *     MAX_FOREIGN_CALLBACKS
 *     MAX_FOREIGN_TARGETS
 *     MAX_FFI_CALLS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICE_COUNT
 *
 * or equivalent limits.
 *
 * The grammar uses normal ANTLR repetition operators.
 *
 * Practical limits belong to:
 *
 *     compiler resource policies;
 *     runtime resource policies;
 *     target capabilities;
 *     deployment constraints.
 *
 * They are not language semantics.
 *
 * ============================================================================
 * "INFINITY" INTERPRETATION
 * ============================================================================
 *
 * Scalability means that this grammar has no artificial architectural ceiling.
 *
 * It does NOT claim that:
 *
 *     hardware resources are mathematically infinite;
 *     memory is infinite;
 *     compiler execution time is infinite;
 *     runtime capacity is infinite.
 *
 * It means that the language does not encode a finite universal ceiling.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A foreign effect may participate in hybrid or quantum computation.
 *
 * Examples of semantic identities include:
 *
 *     foreign::quantum
 *     foreign::quantum::submit
 *     foreign::quantum::measurement
 *
 * The grammar does NOT define:
 *
 *     QubitId
 *     physical qubits
 *     gates
 *     coupling maps
 *     topology
 *     calibration
 *     pulse schedules
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *
 * If semantic analysis determines that a foreign effect produces or consumes
 * quantum computation, the resulting semantic operation eventually crosses:
 *
 *     quantum::ir
 *
 * This file MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Foreign effects may represent semantic interaction with:
 *
 *     HDL tools
 *     hardware services
 *     accelerators
 *     device interfaces
 *     hardware runtimes
 *
 * This file does not define:
 *
 *     physical pins
 *     addresses
 *     bus widths
 *     register maps
 *     FPGA capacity
 *     ASIC topology
 *     accelerator identity
 *     device placement.
 *
 * Those belong downstream.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * A foreign effect may eventually be realized by:
 *
 *     local runtime
 *     remote service
 *     distributed computation
 *     network transport
 *     accelerator service
 *     cloud execution environment
 *     simulator
 *     future execution substrate
 *
 * The grammar does not choose among them.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The frontend AST MUST preserve:
 *
 *     - source span;
 *     - source ordering;
 *     - originating rule;
 *     - qualified-name segments;
 *     - operation identity;
 *     - invocation arguments;
 *     - effect-set structure.
 *
 * The parser MUST NOT create target-specific nodes such as:
 *
 *     ForeignLibrary
 *     ForeignFunctionPointer
 *     NativeHandle
 *     DeviceHandle
 *     PhysicalAddress
 *     QPUHandle
 *     GPUHandle
 *     FPGAHandle
 *     ABIRegister
 *
 * Those are semantic/backend concerns.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis after parsing is responsible for:
 *
 *     - effect name resolution;
 *     - operation resolution;
 *     - foreign-boundary classification;
 *     - FFI contract resolution;
 *     - ABI compatibility;
 *     - type compatibility;
 *     - ownership/lifetime validation;
 *     - effect checking;
 *     - capability checking;
 *     - resource checking;
 *     - security validation;
 *     - policy validation;
 *     - provenance;
 *     - compatibility;
 *     - target feasibility.
 *
 * Parsing MUST NOT perform those operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * The intended pipeline is:
 *
 *     foreign-effect syntax
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic foreign-effect model
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> future domain representation
 *
 * FFI and ABI lowering occur only after semantic validation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no runtime calls;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no hardware discovery;
 *     - no environment-dependent parsing;
 *     - no random decisions.
 *
 * Given:
 *
 *     same source;
 *     same token stream;
 *     same grammar version;
 *
 * the parser must produce the same parse structure.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors are limited to malformed structure.
 *
 * Examples:
 *
 *     malformed effect reference;
 *     malformed effect invocation;
 *     malformed effect set;
 *     malformed operation reference.
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     unknown effect;
 *     unknown operation;
 *     effect is not foreign;
 *     missing FFI contract;
 *     incompatible ABI;
 *     unavailable capability;
 *     insufficient resources;
 *     forbidden policy;
 *     unsupported target.
 *
 * These MUST NOT be converted into parser-level lexical errors.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar introduces no new lexer token.
 *
 * It therefore does not require adding another global reserved word.
 *
 * Existing effect syntax remains valid.
 *
 * Existing generic effect references remain valid.
 *
 * Existing effect operations remain valid.
 *
 * Existing effect sets remain valid.
 *
 * The foreign-effect adapter is an additional parser composition surface for
 * tooling and domain-aware grammar consumers.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/core/
 *
 * IMPORTED GRAMMAR IDENTITIES:
 *
 *     Core
 *     EffectOperations
 *     EffectSets
 *
 * EXPORTS:
 *
 *     foreignEffect
 *     foreignEffectReference
 *     foreignEffectOperationReference
 *     foreignEffectOperation
 *     foreignEffectInvocation
 *     foreignEffectSet
 *     foreignEffectOperationSet
 *
 * CONSUMED_BY:
 *
 *     effect-domain tooling
 *     semantic effect classification
 *     future effect-domain dispatchers
 *     conformance tooling
 *
 * It MUST NOT be inserted into the generic Effects composition root merely to
 * create another overlapping parse path.
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     semantic effect / interoperability analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *
 * TEST_OWNER:
 *
 *     grammar/tests/effects/
 *     grammar/tests/interoperability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/effects.md
 *     grammar/spec/interoperability.md
 *
 * ============================================================================
 * ANTLR COMPOSITION RULE
 * ============================================================================
 *
 * This is a parser grammar, not a lexer grammar.
 *
 * The parser-facing lexical vocabulary is therefore:
 *
 *     ZamaniLexer
 *
 * The grammar MUST NOT use:
 *
 *     ZamaniTokens
 *     ZamaniKeywords
 *     lexer
 *
 * as its parser-facing token vocabulary.
 *
 * ============================================================================
 * IMPORT GRAPH
 * ============================================================================
 *
 * The dependency graph is intentionally one-way:
 *
 *     ForeignEffects
 *          |
 *          +--> Core
 *          |
 *          +--> EffectOperations
 *          |       |
 *          |       +--> Core
 *          |       +--> Expressions
 *          |
 *          +--> EffectSets
 *                  |
 *                  +--> Core
 *
 * It MUST NOT become:
 *
 *     ForeignEffects
 *          -> Interoperability
 *          -> ForeignEffects
 *
 * or:
 *
 *     Effects
 *          -> Interoperability
 *          -> Effects
 *
 * FFI/ABI relationships are semantic relationships, not required grammar
 * imports.
 *
 * ============================================================================
 * RULE DESIGN
 * ============================================================================
 *
 * The rules below intentionally consist of aliases/adapters.
 *
 * This is important.
 *
 * A domain adapter should not create a second syntax for something already
 * defined by the generic effect subsystem.
 *
 * ============================================================================
 */

parser grammar ForeignEffects;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    EffectOperations,
    EffectSets
;


/*
 * ============================================================================
 * 1. FOREIGN EFFECT
 * ============================================================================
 *
 * Universal foreign-effect adapter.
 *
 * The alternatives are deliberately separated by the syntax already owned by
 * the generic effect subsystem:
 *
 *     effectOperationUse
 *         -> operation-use syntax
 *
 *     effectInvocation
 *         -> invocation syntax
 *
 *     effectReference
 *         -> effect identity
 *
 *     effectSet
 *         -> effect collection
 *
 * No foreign-specific syntax is introduced.
 *
 * Semantic classification determines whether the construct is genuinely
 * associated with a foreign interoperability boundary.
 */

foreignEffect
    : foreignEffectOperation
    | foreignEffectInvocation
    | foreignEffectReference
    | foreignEffectSet
    ;


/*
 * ============================================================================
 * 2. FOREIGN EFFECT REFERENCE
 * ============================================================================
 *
 * Adapter around the canonical effect-reference syntax.
 *
 * Example semantic identities may include:
 *
 *     foreign::call
 *     foreign::runtime
 *     foreign::service
 *     foreign::quantum
 *     vendor::extension::effect
 *
 * The grammar does not reserve or validate those names.
 */

foreignEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * 3. FOREIGN EFFECT OPERATION REFERENCE
 * ============================================================================
 *
 * Adapter around the canonical effect-operation reference.
 *
 * This rule is useful to tooling that needs to preserve the distinction
 * between:
 *
 *     effect identity
 *
 * and:
 *
 *     effect-operation identity
 *
 * without creating another operation-reference representation.
 */

foreignEffectOperationReference
    : effectOperationReference
    ;


/*
 * ============================================================================
 * 4. FOREIGN EFFECT OPERATION
 * ============================================================================
 *
 * Adapter around the canonical effect-operation-use syntax.
 *
 * The operation invocation syntax itself remains owned by:
 *
 *     grammar/effects/effect-operations.g4
 *
 * This file does not redefine `perform`.
 */

foreignEffectOperation
    : effectOperationUse
    ;


/*
 * ============================================================================
 * 5. FOREIGN EFFECT INVOCATION
 * ============================================================================
 *
 * Adapter around canonical effect invocation syntax.
 *
 * This does not execute anything.
 *
 * It only provides a stable domain-specific parser rule for downstream
 * composition and tooling.
 */

foreignEffectInvocation
    : effectInvocation
    ;


/*
 * ============================================================================
 * 6. FOREIGN EFFECT SET
 * ============================================================================
 *
 * Adapter around the canonical effect-set syntax.
 *
 * Foreign effects may coexist with ordinary effects:
 *
 *     {
 *         io::read,
 *         foreign::call,
 *         quantum::measurement
 *     }
 *
 * The semantic layer determines the classification of each member.
 */

foreignEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * 7. FOREIGN EFFECT OPERATION SET
 * ============================================================================
 *
 * This is deliberately an alias of the canonical effect-set representation.
 *
 * There is no second collection syntax.
 *
 * Semantic analysis determines which entries represent foreign effect
 * operations.
 */

foreignEffectOperationSet
    : effectSet
    ;


/*
 * ============================================================================
 * INTEGRATION ALIASES
 * ============================================================================
 *
 * These aliases provide stable composition points without duplicating syntax.
 *
 * ============================================================================
 */

/*
 * A reference-only entry point.
 */
foreignEffectReferenceConstruct
    : foreignEffectReference
    ;


/*
 * An operation-reference-only entry point.
 */
foreignEffectOperationReferenceConstruct
    : foreignEffectOperationReference
    ;


/*
 * An invocation-only entry point.
 */
foreignEffectInvocationConstruct
    : foreignEffectInvocation
    ;


/*
 * A set-only entry point.
 */
foreignEffectSetConstruct
    : foreignEffectSet
    ;


/*
 * ============================================================================
 * DOMAIN CLASSIFICATION BOUNDARY
 * ============================================================================
 *
 * The following rule intentionally does NOT exist:
 *
 *     foreignEffectName
 *         : FOREIGN ...
 *
 * because the global lexical vocabulary already provides `FOREIGN`, but
 * introducing a second surface syntax solely for this adapter would create a
 * misleading implication that every foreign effect must literally be written
 * with that keyword.
 *
 * Foreignness is semantic classification.
 *
 * The adapter therefore consumes canonical effect syntax.
 *
 * ============================================================================
 * EXAMPLES OF SEMANTIC USE
 * ============================================================================
 *
 * These examples illustrate semantic identities only.
 *
 * They do not constitute a closed vocabulary.
 *
 * ---------------------------------------------------------------------------
 *
 *     perform foreign::call(value);
 *
 * ---------------------------------------------------------------------------
 *
 *     perform foreign::runtime::operation(value);
 *
 * ---------------------------------------------------------------------------
 *
 *     perform vendor::extension::effect(value);
 *
 * ---------------------------------------------------------------------------
 *
 *     {
 *         io::read,
 *         foreign::call,
 *         security::authorize
 *     }
 *
 * ---------------------------------------------------------------------------
 *
 *     perform foreign::quantum::submit(program);
 *
 * ---------------------------------------------------------------------------
 *
 * The final example may eventually cross:
 *
 *     foreign effect
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     hybrid/quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * The grammar itself never creates that IR.
 *
 * ============================================================================
 * NEGATIVE ARCHITECTURAL EXAMPLES
 * ============================================================================
 *
 * This file MUST NOT evolve into syntax such as:
 *
 *     foreign c call ...
 *     foreign python call ...
 *     foreign cuda call ...
 *     foreign qpu0 call ...
 *     foreign gpu0 call ...
 *     foreign cpu0 call ...
 *
 * Such syntax would incorrectly bind source semantics to particular
 * implementation environments.
 *
 * The language must instead represent portable semantic intent.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No machine-capacity constants exist here.
 *
 * No vendor list exists here.
 *
 * No operating-system list exists here.
 *
 * No ABI list exists here.
 *
 * No calling-convention list exists here.
 *
 * No foreign-language list exists here.
 *
 * No device list exists here.
 *
 * No hardware topology exists here.
 *
 * No quantum operation catalogue exists here.
 *
 * No QEC catalogue exists here.
 *
 * No physical resource allocation exists here.
 *
 * No target identifier is required by this grammar.
 *
 * ============================================================================
 * SCALABILITY AUDIT
 * ============================================================================
 *
 * The grammar imposes no language-level maximum on:
 *
 *     effect references;
 *     operation references;
 *     invocations;
 *     effect-set members;
 *     qualified-name depth;
 *     source declarations;
 *     programs;
 *     domains;
 *     interoperability boundaries;
 *     machines;
 *     devices;
 *     nodes;
 *     processors;
 *     accelerators;
 *     qubits;
 *     memory;
 *     network resources.
 *
 * Repetition and nesting remain governed by the canonical grammar and by
 * implementation/resource policies.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive structural tests MUST cover:
 *
 *     1. foreign effect reference;
 *     2. foreign effect operation use;
 *     3. foreign effect invocation;
 *     4. foreign effect set;
 *     5. mixed effect set;
 *     6. deeply qualified foreign effect identity;
 *     7. custom/vendor/future symbolic effect identity;
 *     8. quantum-related foreign effect identity;
 *     9. hardware-related foreign effect identity;
 *     10. distributed/remote foreign effect identity.
 *
 * Negative tests MUST cover malformed constructs owned by the generic effect
 * grammar, including:
 *
 *     - incomplete qualified names;
 *     - malformed invocation arguments;
 *     - malformed effect sets;
 *     - malformed operation-use syntax.
 *
 * Semantic negative tests MUST remain outside this grammar and cover:
 *
 *     - unresolved foreign effect;
 *     - unresolved operation;
 *     - invalid FFI contract;
 *     - incompatible ABI;
 *     - missing capability;
 *     - insufficient resources;
 *     - forbidden policy;
 *     - invalid security authorization;
 *     - unsupported target.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Cross-domain tests MUST verify that foreign effects can coexist with:
 *
 *     classical effects;
 *     quantum effects;
 *     HDL/hardware effects;
 *     AI effects;
 *     data effects;
 *     networking effects;
 *     distributed effects;
 *     security effects;
 *     simulation effects;
 *     native effects.
 *
 * These tests verify semantic composition rather than physical realization.
 *
 * ============================================================================
 * QUANTUM BOUNDARY TEST
 * ============================================================================
 *
 * A quantum-related foreign effect MUST remain source-level symbolic intent.
 *
 * Example:
 *
 *     perform foreign::quantum::submit(program);
 *
 * Expected pipeline:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic effect
 *       ->
 *     hybrid/quantum semantic operation
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience/QEC/ZQN
 *       ->
 *     HAL
 *
 * The grammar must not select a physical QPU or physical qubit.
 *
 * ============================================================================
 * FFI BOUNDARY TEST
 * ============================================================================
 *
 * A foreign function may declare an effect using the existing function/FFI
 * grammars.
 *
 * The effect remains owned by the effect subsystem.
 *
 * The FFI declaration remains owned by interoperability.
 *
 * The semantic layer joins them.
 *
 * Expected conceptual model:
 *
 *     foreign function
 *          |
 *          +--> type contract
 *          +--> effect contract
 *          +--> capability requirements
 *          +--> resource requirements
 *          +--> ABI contract
 *          |
 *          v
 *     semantic interoperability model
 *
 * There must be no duplicated AST or IR merely because the effect originated
 * at an FFI boundary.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * The same source and grammar version MUST produce the same parse tree.
 *
 * This file contains no:
 *
 *     actions;
 *     semantic predicates;
 *     runtime queries;
 *     environment queries;
 *     hardware queries;
 *     random behavior.
 *
 * Therefore no target-dependent parsing is permitted.
 *
 * ============================================================================
 * COMPATIBILITY TEST CONTRACT
 * ============================================================================
 *
 * Adding this file must not invalidate existing ordinary effect syntax.
 *
 * In particular, existing:
 *
 *     effect references;
 *     effect operation uses;
 *     effect invocations;
 *     effect sets
 *
 * remain owned by their existing grammar files.
 *
 * This adapter adds named integration surfaces rather than changing the
 * canonical generic effect syntax.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] parser grammar name is `ForeignEffects`;
 *
 * [ ] parser-facing vocabulary is `ZamaniLexer`;
 *
 * [ ] no lexer rules exist here;
 *
 * [ ] no embedded Rust exists here;
 *
 * [ ] no semantic predicates exist here;
 *
 * [ ] no runtime behavior exists here;
 *
 * [ ] `EffectOperations` remains the owner of operation-use syntax;
 *
 * [ ] `EffectSets` remains the owner of effect-set syntax;
 *
 * [ ] generic effect references are not redefined;
 *
 * [ ] FFI syntax is not duplicated;
 *
 * [ ] ABI syntax is not duplicated;
 *
 * [ ] foreign-function declarations are not duplicated;
 *
 * [ ] no vendor/platform catalogue exists;
 *
 * [ ] no hardware limits exist;
 *
 * [ ] no quantum limits exist;
 *
 * [ ] no target-specific realization exists;
 *
 * [ ] the grammar is open-world;
 *
 * [ ] source portability is preserved;
 *
 * [ ] semantic classification remains downstream;
 *
 * [ ] quantum semantics can reach `quantum::ir`;
 *
 * [ ] foreign execution remains downstream;
 *
 * [ ] positive tests exist;
 *
 * [ ] negative tests exist;
 *
 * [ ] boundary tests exist;
 *
 * [ ] scalability tests exist;
 *
 * [ ] determinism tests exist;
 *
 * [ ] compatibility tests exist;
 *
 * [ ] Rust implementation remains compatible with Rust 1.97/1.97.1;
 *
 * [ ] the implementation requires no unsafe Rust.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * The purpose of this file is intentionally narrow:
 *
 *     classify and compose foreign-capable EFFECT syntax
 *
 * without turning the effect subsystem into an FFI implementation.
 *
 * The correct ownership chain is:
 *
 *     foreign effect syntax
 *             |
 *             v
 *     generic effect infrastructure
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic effect classification
 *             |
 *             +----------------------+
 *             |                      |
 *             v                      v
 *         FFI contract          non-FFI realization
 *             |
 *             v
 *         ABI analysis
 *             |
 *             v
 *      capability/resource/
 *      policy/security analysis
 *             |
 *             v
 *       canonical semantic model
 *             |
 *       +-----+------+----------------+
 *       |            |                |
 *       v            v                v
 *   classical    quantum::ir     HDL/hardware
 *       |            |                |
 *       +------------+----------------+
 *                    |
 *                    v
 *             target-independent
 *             optimization/lowering
 *                    |
 *                    v
 *              target realization
 *
 * This preserves the central Zamani rule:
 *
 *     source describes portable computation and intent;
 *     downstream layers determine realization.
 *
 * ============================================================================
 */