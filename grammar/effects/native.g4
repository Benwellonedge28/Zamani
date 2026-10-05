/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/effects/native.g4
 *
 * GRAMMAR
 * -------
 * NativeEffects
 *
 * STATUS
 * ------
 * CANONICAL NATIVE-EFFECT DOMAIN ADAPTER
 *
 * LANGUAGE / RUNTIME BASELINE
 * ---------------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no runtime execution;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no hardware discovery;
 *     - no native-library loading;
 *     - no unsafe implementation requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the grammar boundary for NATIVE COMPUTATIONAL EFFECTS.
 *
 * "Native" here means an effect whose eventual implementation may cross from
 * the portable Zamani semantic model into a target/runtime-native facility.
 *
 * Examples of possible semantic identities include:
 *
 *     native::call
 *     native::memory
 *     native::atomic
 *     native::system
 *     native::intrinsic
 *     native::accelerator
 *     native::extension
 *     vendor::native::operation
 *     target::native::operation
 *
 * These names are NOT enumerated by this grammar.
 *
 * Native operation identity remains open-world and is resolved semantically.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * THIS FILE IS AN ADAPTER, NOT A SECOND EFFECT LANGUAGE.
 *
 * It MUST reuse the canonical effect infrastructure:
 *
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/core/
 *
 * It MUST NOT redefine:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectInvocationArguments
 *     effectOperationCall
 *     effectOperationUse
 *     performEffectOperation
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     qualifiedName
 *     expression
 *     argumentList
 *
 * Those constructs have canonical owners elsewhere.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     nativeEffect
 *     nativeEffectReference
 *     nativeEffectOperation
 *     nativeEffectInvocation
 *     nativeEffectSet
 *     nativeEffectOperationSet
 *
 * These rules are domain-adapter boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     FFI declarations
 *     ABI declarations
 *     foreign functions
 *     calling conventions
 *     symbol resolution
 *     native library loading
 *     native pointer representation
 *     native memory allocation
 *     operating-system APIs
 *     processor instructions
 *     GPU instructions
 *     FPGA primitives
 *     ASIC implementation
 *     QPU implementation
 *     physical addresses
 *     physical devices
 *     device selection
 *     target selection
 *     runtime dispatch
 *     security authorization
 *     capability possession
 *     resource allocation
 *
 * ============================================================================
 * WHY A NATIVE EFFECT ADAPTER EXISTS
 * ============================================================================
 *
 * Zamani has several distinct concepts which must not be conflated:
 *
 *     EFFECT
 *         observable computational behavior.
 *
 *     FFI
 *         source-level foreign interoperability boundary.
 *
 *     ABI
 *         calling/data compatibility contract.
 *
 *     CAPABILITY
 *         what an execution environment can provide.
 *
 *     RESOURCE
 *         computational resources involved in realization.
 *
 *     POLICY
 *         restrictions/preferences governing realization.
 *
 *     NATIVE EFFECT
 *         an effect whose semantic implementation may cross into a
 *         target/runtime-native facility.
 *
 * Native effects therefore belong in the effect system while FFI/ABI remain
 * separate interoperability contracts.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *     native effect syntax
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic resolution
 *          |
 *          +--> native-effect classification
 *          +--> operation resolution
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> policy analysis
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future domain representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     target realization
 *
 * This grammar MUST remain above all physical realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Native effects MUST NOT destroy source portability.
 *
 * A source program may express native computational intent without specifying:
 *
 *     CPU model
 *     GPU model
 *     FPGA model
 *     ASIC technology
 *     accelerator model
 *     QPU model
 *     operating-system identity
 *     physical device identity
 *     memory-bank identity
 *     register identity
 *     physical address
 *     machine topology
 *     node identity
 *     deployment location
 *
 * Example semantic intent:
 *
 *     perform native::operation(value);
 *
 * does not mean:
 *
 *     use CPU X
 *     use GPU Y
 *     use device Z
 *
 * Target realization is determined downstream.
 *
 * ============================================================================
 * OPEN-WORLD NATIVE EFFECT MODEL
 * ============================================================================
 *
 * Native effects are deliberately OPEN-WORLD.
 *
 * This grammar MUST NOT contain a closed enumeration such as:
 *
 *     nativeOperation
 *         : NATIVE_CALL
 *         | NATIVE_MEMORY
 *         | NATIVE_ATOMIC
 *         | NATIVE_INTRINSIC
 *         ;
 *
 * Such enumeration would make every future native facility require a grammar
 * modification.
 *
 * Instead, native identities are ordinary qualified names.
 *
 * The semantic layer determines whether a referenced identity is:
 *
 *     - a native effect;
 *     - an ordinary effect;
 *     - a foreign effect;
 *     - a vendor extension;
 *     - a dialect-provided effect;
 *     - an unknown effect.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * Canonical dependencies:
 *
 *     Core
 *         |
 *         +--> qualifiedName
 *         +--> shared source syntax
 *
 *     EffectOperations
 *         |
 *         +--> effectOperationReference
 *         +--> effectInvocation
 *         +--> effectOperationUse
 *
 *     EffectSets
 *         |
 *         +--> effectReference
 *         +--> effectSet
 *
 * This file deliberately contains no lexer rules.
 *
 * ============================================================================
 */

parser grammar NativeEffects;

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
 * 1. NATIVE EFFECT
 * ============================================================================
 *
 * General native-effect adapter.
 *
 * The alternatives intentionally delegate to canonical effect constructs.
 *
 * No native-specific operation syntax is invented here.
 *
 * Semantic validation determines whether the referenced identity actually
 * denotes a native effect.
 */

nativeEffect
    : nativeEffectOperation
    | nativeEffectReference
    | nativeEffectSet
    ;


/*
 * ============================================================================
 * 2. NATIVE EFFECT REFERENCE
 * ============================================================================
 *
 * A native effect reference reuses the canonical effect-reference grammar.
 *
 * Examples of possible source identities:
 *
 *     native::memory
 *     native::atomic
 *     native::intrinsic
 *     native::system
 *     vendor::native::extension
 *     future::native::operation
 *
 * These are examples of symbolic identities only.
 *
 * The grammar does not reserve these names.
 */

nativeEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * 3. NATIVE EFFECT OPERATION
 * ============================================================================
 *
 * Reuses the canonical effect-operation-use grammar.
 *
 * Possible source form:
 *
 *     perform native::operation(value);
 *
 * The `perform` syntax itself remains owned by EffectOperations.
 *
 * This rule therefore does NOT redefine `perform`.
 */

nativeEffectOperation
    : effectOperationUse
    ;


/*
 * ============================================================================
 * 4. NATIVE EFFECT INVOCATION
 * ============================================================================
 *
 * Explicit invocation adapter.
 *
 * This is useful to grammar consumers that need to distinguish a native
 * invocation boundary without duplicating invocation syntax.
 *
 * Example:
 *
 *     native::operation(value)
 *
 * The actual argument expressions remain ordinary Zamani expressions.
 */

nativeEffectInvocation
    : effectInvocation
    ;


/*
 * ============================================================================
 * 5. NATIVE EFFECT SET
 * ============================================================================
 *
 * Native effects may participate in ordinary effect sets.
 *
 * Example semantic intent:
 *
 *     {
 *         native::operation,
 *         io::read
 *     }
 *
 * The actual effect-set syntax remains owned by EffectSets.
 *
 * The semantic layer determines which members are native.
 */

nativeEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * 6. NATIVE EFFECT OPERATION SET
 * ============================================================================
 *
 * A reusable adapter for consumers that need a set of native operation-use
 * boundaries.
 *
 * This rule does not introduce a new collection syntax.
 *
 * It deliberately uses the canonical effect-set structure.
 *
 * Native classification remains semantic.
 */

nativeEffectOperationSet
    : nativeEffectSet
    ;


/*
 * ============================================================================
 * NATIVE / FFI / ABI SEPARATION
 * ============================================================================
 *
 * Native effects MUST NOT replace the interoperability subsystem.
 *
 * The repository already has dedicated owners:
 *
 *     grammar/interoperability/ffi.g4
 *         -> FFI boundary syntax
 *
 *     grammar/interoperability/abi.g4
 *         -> ABI contract syntax
 *
 *     grammar/interoperability/foreign-functions.g4
 *         -> foreign callable declarations
 *
 * Therefore:
 *
 *     native.g4
 *
 * does NOT define:
 *
 *     extern
 *     library declarations
 *     symbol declarations
 *     calling conventions
 *     ABI layouts
 *     marshaling rules
 *     foreign type declarations
 *
 * Native effects can reference or invoke operations whose semantic realization
 * happens to cross an FFI/ABI boundary, but the boundary contract itself
 * remains owned by interoperability.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * NATIVE EFFECT
 *
 *     Describes computational behavior whose realization may use a
 *     native/target facility.
 *
 * CAPABILITY
 *
 *     Describes whether the realization environment can provide the required
 *     native facility.
 *
 * RESOURCE
 *
 *     Describes the abstract resources needed by the realization.
 *
 * POLICY
 *
 *     Determines whether native realization is permitted or preferred.
 *
 * SECURITY
 *
 *     Determines whether the operation is authorized.
 *
 * FFI
 *
 *     Describes an external interoperability boundary when one exists.
 *
 * ABI
 *
 *     Describes compatibility between callable/data representations.
 *
 * These concepts MUST remain separate.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only structural validity.
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the referenced effect exists;
 *     - whether it belongs to the native-effect category;
 *     - whether the operation exists;
 *     - whether the operation is accessible;
 *     - whether arguments satisfy its signature;
 *     - what result type it produces;
 *     - what effects it propagates;
 *     - what capabilities it requires;
 *     - what resources it requires;
 *     - what policies apply;
 *     - whether security authorization is required;
 *     - whether an FFI boundary is crossed;
 *     - whether an ABI contract applies;
 *     - whether the selected realization is compatible.
 *
 * None of these questions are answered by this grammar.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A native effect MUST NOT imply possession of a capability.
 *
 * For example:
 *
 *     perform native::operation(value);
 *
 * does not grant:
 *
 *     capability::native_execute
 *
 * or any other authority.
 *
 * A semantic capability analysis may determine that realization requires a
 * capability such as:
 *
 *     native.execution
 *     native.memory
 *     native.atomic
 *     native.intrinsic
 *     foreign.call
 *
 * Such identities remain open-world symbolic capabilities.
 *
 * The actual capability model is owned by:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *     grammar/security/
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Native effects may have resource consequences.
 *
 * Those consequences belong to semantic resource analysis.
 *
 * This grammar MUST NOT encode:
 *
 *     fixed memory sizes;
 *     fixed register widths;
 *     fixed register counts;
 *     fixed native stack sizes;
 *     fixed device counts;
 *     fixed accelerator counts;
 *     fixed thread counts;
 *     fixed processor counts;
 *     fixed address widths.
 *
 * Resource requirements must remain symbolic and target-independent.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Native execution may cross a security boundary.
 *
 * Parsing MUST NOT:
 *
 *     authenticate;
 *     authorize;
 *     load code;
 *     execute code;
 *     access secrets;
 *     inspect credentials;
 *     inspect the operating system;
 *     access devices;
 *     modify process state.
 *
 * Security decisions belong downstream.
 *
 * A policy may reject a native effect even though the source syntax is valid.
 *
 * ============================================================================
 * SANDBOX CONTRACT
 * ============================================================================
 *
 * Native effects are especially relevant to sandboxing.
 *
 * A sandbox may semantically:
 *
 *     allow native effects;
 *     deny native effects;
 *     allow selected native capabilities;
 *     restrict FFI;
 *     restrict resource classes;
 *     require provenance;
 *     require deterministic execution;
 *     require simulation instead of native execution.
 *
 * None of those policies belong in this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing native-effect syntax MUST be deterministic.
 *
 * Parsing MUST depend only on:
 *
 *     source token sequence;
 *     active grammar;
 *     language/grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *     host CPU;
 *     host operating system;
 *     installed libraries;
 *     runtime state;
 *     environment variables;
 *     hardware discovery;
 *     device availability;
 *     network state;
 *     native linker state;
 *     randomness.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Native-effect realization may require provenance.
 *
 * Semantic/provenance infrastructure may record:
 *
 *     source identity;
 *     effect identity;
 *     operation identity;
 *     semantic version;
 *     provider identity;
 *     ABI identity;
 *     capability decision;
 *     policy decision;
 *     target realization;
 *     transformation;
 *     verification result.
 *
 * The grammar itself only preserves the source-level structure needed to
 * construct that provenance.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The domain-neutral AST should preserve, as applicable:
 *
 *     qualified effect identity;
 *     operation identity;
 *     operation arguments;
 *     effect-set membership;
 *     source ordering;
 *     source spans;
 *     surrounding effect context.
 *
 * Conceptually:
 *
 *     NativeEffectReference
 *         identity: QualifiedName
 *
 *     NativeEffectOperation
 *         operation: QualifiedName
 *         arguments: Expression[]
 *
 * The exact AST types remain owned by the frontend AST implementation.
 *
 * This grammar MUST NOT force AST nodes such as:
 *
 *     NativeCpuInstruction
 *     NativeGpuInstruction
 *     NativePointer
 *     NativeRegister
 *     PhysicalDevice
 *     PhysicalAddress
 *     QPUInstruction
 *     FFIHandle
 *
 * merely because a source effect is classified as native.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * The expected semantic path is:
 *
 *     native effect syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic native-effect model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> other domain representation
 *
 * Native classification MUST NOT create a second IR solely for native
 * execution.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A native effect may occur in a computation that also contains quantum
 * semantics.
 *
 * Example:
 *
 *     perform native::classical_control(value);
 *
 * followed by a quantum operation.
 *
 * Native-effect syntax MUST NOT define:
 *
 *     qubits;
 *     physical qubits;
 *     quantum gates;
 *     coupling maps;
 *     routing;
 *     scheduling;
 *     calibration;
 *     QEC;
 *     ZQN.
 *
 * If semantic analysis determines that an operation is quantum, its canonical
 * quantum representation remains:
 *
 *     quantum::ir
 *
 * No native-specific quantum IR is introduced.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Native effects may describe hardware-adjacent computational intent.
 *
 * They MUST NOT encode physical realization details such as:
 *
 *     physical pins;
 *     fixed bus widths;
 *     fixed register widths;
 *     fixed memory capacity;
 *     fixed FPGA resource counts;
 *     fixed ASIC cell counts;
 *     physical placement;
 *     physical routing;
 *     device identifiers.
 *
 * Hardware realization remains owned downstream by:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *     resource analysis;
 *     compiler lowering;
 *     backend/HAL infrastructure.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * A native effect may eventually be realized locally or remotely.
 *
 * This grammar does not determine whether:
 *
 *     native::operation
 *
 * runs:
 *
 *     locally;
 *     on another processor;
 *     on an accelerator;
 *     through a service;
 *     in a simulator;
 *     on a heterogeneous target.
 *
 * Distributed and networking semantics remain owned by their respective
 * subsystems.
 *
 * ============================================================================
 * SIMULATION BOUNDARY
 * ============================================================================
 *
 * A native effect may have a simulation realization.
 *
 * For example, a compiler/runtime may choose a simulator or emulation layer
 * when:
 *
 *     native capability is unavailable;
 *     policy requires isolation;
 *     reproducibility is required;
 *     testing requires deterministic execution;
 *     target realization is deferred.
 *
 * This is an execution-policy decision.
 *
 * It does not require a second native grammar.
 *
 * ============================================================================
 * ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Native effects may participate in adaptive execution.
 *
 * A downstream execution planner may:
 *
 *     detect;
 *     evaluate;
 *     select;
 *     fallback;
 *     retry;
 *     recover;
 *     simulate;
 *     specialize.
 *
 * Native-effect syntax itself remains unchanged.
 *
 * This is essential for POCO-REAF:
 *
 *     source intent
 *          |
 *          v
 *     capability/resource analysis
 *          |
 *          v
 *     realization selection
 *
 * rather than:
 *
 *     source
 *          |
 *          v
 *     fixed native implementation
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Adding a new native effect identity MUST NOT require a grammar change.
 *
 * For example, a future implementation may introduce:
 *
 *     future::native::operation
 *
 * without modifying this file.
 *
 * Compatibility changes are required only when the SOURCE SYNTAX changes.
 *
 * Semantic additions remain open-world.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no language-level finite limit on:
 *
 *     native effects;
 *     native effect references;
 *     native operations;
 *     effect-set members;
 *     operation arguments;
 *     namespace depth;
 *     modules;
 *     declarations;
 *     programs;
 *     domains;
 *     targets;
 *     execution environments.
 *
 * It contains no capacity constants.
 *
 * Compiler/parser/runtime resource exhaustion is an implementation concern,
 * not a semantic property of the language.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no language-level machine limits.
 *
 * It MUST NOT introduce constants or syntax representing fixed:
 *
 *     processor counts;
 *     core counts;
 *     thread counts;
 *     accelerator counts;
 *     device counts;
 *     memory capacities;
 *     register widths;
 *     register counts;
 *     address widths;
 *     bus widths;
 *     node counts;
 *     network sizes;
 *     qubit counts;
 *     tensor dimensions;
 *     tensor rank.
 *
 * It also does not select:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     node;
 *     device;
 *     memory bank;
 *     register.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *             |
 *             v
 *     Core
 *             |
 *             +--> qualifiedName
 *             |
 *             v
 *     EffectOperations
 *             |
 *             +--> effectOperationUse
 *             +--> effectInvocation
 *             |
 *             v
 *     EffectSets
 *             |
 *             +--> effectReference
 *             +--> effectSet
 *
 * THIS FILE
 * ---------
 *
 * Provides:
 *
 *     nativeEffect
 *     nativeEffectReference
 *     nativeEffectOperation
 *     nativeEffectInvocation
 *     nativeEffectSet
 *     nativeEffectOperationSet
 *
 * DOWNSTREAM
 * ----------
 *
 *     frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic effect resolution
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> policy analysis
 *          +--> security analysis
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL/hardware
 *          +--> distributed
 *          +--> accelerator
 *          +--> future domains
 *
 * ============================================================================
 * FFI INTEGRATION
 * ============================================================================
 *
 * When a native effect crosses a foreign boundary:
 *
 *     native effect
 *          |
 *          v
 *     semantic effect resolution
 *          |
 *          v
 *     FFI contract
 *          |
 *          v
 *     ABI contract
 *          |
 *          v
 *     target realization
 *
 * The FFI grammar remains:
 *
 *     grammar/interoperability/ffi.g4
 *
 * The ABI grammar remains:
 *
 *     grammar/interoperability/abi.g4
 *
 * Neither should import NativeEffects merely to recognize ordinary native
 * effect names.
 *
 * ============================================================================
 * EFFECT COMPOSITION INTEGRATION
 * ============================================================================
 *
 * `grammar/effects/effects.g4` remains the generic effect composition root.
 *
 * It MUST NOT import this grammar merely to add native operation names.
 *
 * The reason is important:
 *
 *     Effects
 *         |
 *         +--> EffectOperations
 *         +--> EffectSets
 *
 * already provide the universal open-world effect syntax.
 *
 * NativeEffects is an optional domain adapter consumed by grammar components
 * that specifically need a native-effect semantic boundary.
 *
 * This prevents a circular or redundant effect grammar hierarchy.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * `grammar/expressions/effects.g4` remains the canonical owner of:
 *
 *     perform ...
 *
 * Therefore:
 *
 *     perform native::operation(value)
 *
 * is parsed through the ordinary effect-expression path.
 *
 * NativeEffects MUST NOT introduce another `perform` expression.
 *
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 *
 * `grammar/statements/effects.g4` remains the statement-level adapter.
 *
 * It MUST NOT duplicate native-effect statement syntax.
 *
 * Native effects are ordinary effect statements after semantic classification.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Native effects may require capabilities/resources.
 *
 * Those requirements are attached downstream using:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *
 * This grammar does not discover or allocate resources.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Native effects may be restricted by:
 *
 *     grammar/security/
 *
 * Security policies may allow or reject native execution.
 *
 * A syntactically valid native effect is never proof of authorization.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Native realization may be recorded by the repository's provenance/audit
 * infrastructure.
 *
 * The grammar preserves the symbolic source identity required for that record.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE STRUCTURAL CASES
 * -------------------------
 *
 * These should parse through the normal effect infrastructure:
 *
 *     perform native::operation(value);
 *
 *     perform native::memory(address, value);
 *
 *     perform native::atomic(operation, location);
 *
 *     perform vendor::native::operation(value);
 *
 *     perform future::native::operation(value);
 *
 *     {
 *         native::operation,
 *         io::read
 *     }
 *
 * The examples above demonstrate open symbolic identity. They do not establish
 * a built-in native operation catalogue.
 *
 * ============================================================================
 * NEGATIVE STRUCTURAL CASES
 * ============================================================================
 *
 * The following malformed forms must be rejected by the canonical grammar:
 *
 *     perform native::;
 *
 *     perform native::operation(;
 *
 *     perform native::operation(,);
 *
 *     perform ::native::operation;
 *
 *     native::;
 *
 *     native::
 *
 * These failures are ordinary syntax diagnostics.
 *
 * Semantic errors such as:
 *
 *     unknown native operation;
 *     unavailable capability;
 *     unauthorized native execution;
 *     incompatible ABI;
 *     unavailable target;
 *
 * MUST be reported by downstream semantic/security/resource analysis.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Native effects must be testable together with:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware intent;
 *     accelerator computation;
 *     AI/ML computation;
 *     tensor/data computation;
 *     distributed execution;
 *     networking;
 *     FFI;
 *     ABI;
 *     sandbox policies;
 *     simulation;
 *     provenance;
 *     contracts;
 *     capabilities;
 *     resource requirements.
 *
 * ============================================================================
 * POCO-REAF TEST
 * ============================================================================
 *
 * The same source-level native effect syntax must remain structurally valid
 * regardless of whether semantic realization eventually targets:
 *
 *     embedded;
 *     CPU;
 *     multicore;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed infrastructure;
 *     cloud;
 *     future computational substrate.
 *
 * Target feasibility is a downstream question.
 *
 * ============================================================================
 * DETERMINISM TEST
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     grammar version;
 *     lexical vocabulary;
 *
 * the parse result must be identical.
 *
 * Native library availability MUST NOT alter parsing.
 *
 * ============================================================================
 * SCALABILITY TEST
 * ============================================================================
 *
 * Test structurally increasing:
 *
 *     qualified-name depth;
 *     operation arguments;
 *     effect-set membership;
 *     nested source constructs;
 *     native-effect references;
 *     modules;
 *     program size.
 *
 * No test may assume a language-defined finite maximum.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar requires no Rust implementation code.
 *
 * Generated parser/compiler integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *
 * No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS COMPLETE WHEN:
 *
 * [x] NativeEffects is the sole grammar identity for this file.
 * [x] The canonical ZamaniLexer vocabulary is consumed.
 * [x] Core owns qualified names.
 * [x] EffectOperations owns effect invocation syntax.
 * [x] EffectSets owns effect-set syntax.
 * [x] NativeEffects does not duplicate `perform`.
 * [x] NativeEffects does not duplicate FFI.
 * [x] NativeEffects does not duplicate ABI.
 * [x] Native operation names remain open-world.
 * [x] Native effect names remain open-world.
 * [x] No target is selected by parsing.
 * [x] No runtime operation occurs during parsing.
 * [x] No hardware discovery occurs during parsing.
 * [x] Capability semantics remain downstream.
 * [x] Resource semantics remain downstream.
 * [x] Security semantics remain downstream.
 * [x] Policy semantics remain downstream.
 * [x] Provenance remains downstream.
 * [x] No second IR is introduced.
 * [x] Quantum semantics continue through quantum::ir.
 * [x] HDL/hardware semantics remain downstream.
 * [x] FFI remains owned by interoperability/ffi.g4.
 * [x] ABI remains owned by interoperability/abi.g4.
 * [x] No language-level machine capacity is encoded.
 * [x] No fixed implementation catalogue is encoded.
 * [x] No unsafe implementation is required.
 * [x] Integration tests are defined.
 * [x] Positive tests are defined.
 * [x] Negative tests are defined.
 * [x] Boundary tests are defined.
 * [x] Scalability tests are defined.
 * [x] Determinism tests are defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers exactly one question:
 *
 *     "What syntactic structures allow another grammar component to identify
 *      a source construct as belonging to the native-effect boundary?"
 *
 * It does NOT answer:
 *
 *     "Which native implementation runs?"
 *     "Which library is loaded?"
 *     "Which ABI is selected?"
 *     "Which processor executes it?"
 *     "Which accelerator executes it?"
 *     "Which device is selected?"
 *     "Which capability is possessed?"
 *     "Which resource is allocated?"
 *     "Which security policy permits it?"
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */