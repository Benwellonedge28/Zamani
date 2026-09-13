/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/quantum.g4
 *
 * Status:
 *     Canonical modular parser grammar for QUANTUM EFFECT REFERENCES.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no filesystem access;
 *       - no network access;
 *       - no runtime calls;
 *       - no hardware discovery;
 *       - no unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX SPECIFIC TO QUANTUM EFFECT
 * NAMESPACING AND QUANTUM EFFECT QUALIFICATION.
 *
 * It provides a stable grammar boundary for quantum effects while preserving
 * the OPEN-WORLD effect model of Zamani.
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::readout
 *     quantum::reset
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::logical_state
 *     quantum::entanglement
 *     quantum::custom::future_operation
 *
 * The grammar deliberately does NOT enumerate a closed list of quantum
 * effects.
 *
 * A new quantum effect therefore does NOT require this grammar to change.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniTokens
 *   |
 *   v
 * Core names
 *   |
 *   v
 * QuantumEffects                    <-- THIS FILE
 *   |
 *   +--------------------+
 *   |                    |
 *   v                    v
 * EffectSets        EffectDeclarations
 *   |                    |
 *   +---------+----------+
 *             |
 *             v
 *      Effect semantic analysis
 *             |
 *       +-----+-----+----------------+
 *       |           |                |
 *       v           v                v
 *   capability   resource       type/effect
 *    analysis     analysis        analysis
 *       |           |                |
 *       +-----------+----------------+
 *                   |
 *                   v
 *       canonical semantic representation
 *                   |
 *        +----------+----------+
 *        |          |          |
 *        v          v          v
 *   classical    quantum::ir  HDL/hardware
 *      IR
 *                   |
 *                   v
 *       optimization / routing /
 *       scheduling / resilience /
 *       ZQN / runtime
 *
 * This file therefore establishes syntax only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantum effect namespace qualification;
 *     - quantum effect references;
 *     - quantum effect namespace paths;
 *     - quantum effect reference lists;
 *     - quantum effect grouping;
 *     - quantum effect source-level aliases/qualifiers where explicitly
 *       supported by this grammar;
 *     - syntactic distinction between the canonical `quantum` effect namespace
 *       and arbitrary effect namespaces.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general effect references;
 *     - general effect declarations;
 *     - effect operation declarations;
 *     - generic effect declarations;
 *     - effect handlers;
 *     - effect invocation;
 *     - capability declarations;
 *     - capability references;
 *     - resources;
 *     - resource quantities;
 *     - target selection;
 *     - hardware discovery;
 *     - hardware topology;
 *     - QubitId;
 *     - PhysicalQubitId;
 *     - quantum gates;
 *     - quantum operations;
 *     - quantum circuits;
 *     - quantum states;
 *     - quantum measurement semantics;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - resilience;
 *     - runtime dispatch;
 *     - quantum::ir.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * `effect-sets.g4` already provides the general open-world rule:
 *
 *     effectReference : qualifiedName ;
 *
 * That is correct for the general effect system.
 *
 * This file exists so the quantum namespace has a dedicated, reusable
 * syntactic boundary without changing the general effect model.
 *
 * This is important because quantum effects are a semantic domain, while the
 * general effect system must remain open to:
 *
 *     classical::
 *     quantum::
 *     hardware::
 *     distributed::
 *     networking::
 *     ai::
 *     future::
 *     vendor::
 *     custom::
 *
 * and arbitrary future domains.
 *
 * The quantum namespace therefore becomes a specialization of the general
 * naming system, not a replacement for it.
 *
 * ============================================================================
 * OPEN-WORLD QUANTUM EFFECT MODEL
 * ============================================================================
 *
 * VALID:
 *
 *     quantum::measurement
 *     quantum::reset
 *     quantum::readout
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     quantum::logical_qubit
 *     quantum::error_correction
 *     quantum::future::operation
 *     quantum::photonic::interaction
 *     quantum::custom::domain::effect
 *
 * The grammar MUST NOT contain:
 *
 *     quantumEffect
 *         : MEASUREMENT
 *         | RESET
 *         | READOUT
 *         | ...
 *
 * because that would create a closed quantum-effect catalogue.
 *
 * Semantic registries determine whether a particular effect exists and what
 * it means.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Quantum effects describe SOURCE-LEVEL COMPUTATIONAL BEHAVIOR.
 *
 * They MUST NOT encode:
 *
 *     - a particular QPU;
 *     - a vendor;
 *     - a backend;
 *     - a device ID;
 *     - a physical qubit;
 *     - a qubit count;
 *     - a topology;
 *     - a coupling map;
 *     - a calibration;
 *     - a pulse schedule;
 *     - a gate duration;
 *     - a noise model;
 *     - a QEC decoder;
 *     - a routing strategy;
 *     - a scheduling strategy.
 *
 * Therefore:
 *
 *     quantum::measurement
 *
 * does NOT mean:
 *
 *     use QPU X
 *
 * or:
 *
 *     allocate N physical qubits.
 *
 * It means only that the source computation has the named quantum effect.
 *
 * Realization is determined downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file imposes no language-level maximum on:
 *
 *     - effect namespace depth;
 *     - effect name length;
 *     - number of quantum effect references;
 *     - number of quantum effects in a set;
 *     - number of effect clauses;
 *     - number of source declarations;
 *     - number of quantum operations;
 *     - number of qubits;
 *     - number of devices;
 *     - number of processors;
 *     - number of nodes;
 *     - number of accelerators.
 *
 * Repetition is represented with ANTLR repetition operators.
 *
 * There are deliberately no:
 *
 *     MAX_QUANTUM_EFFECTS
 *     MAX_EFFECT_DEPTH
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_QPU_COUNT
 *
 * or equivalent constants.
 *
 * Practical parser limits belong to compiler resource policies and are not
 * language semantics.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * NEVER encode hardware-specific concepts here.
 *
 * Forbidden examples:
 *
 *     quantum::qpu0
 *     quantum::ibm_backend
 *     quantum::physical_qubit_0
 *     quantum::topology_ring
 *     quantum::coupling_map
 *     quantum::pulse_channel
 *
 * Such information belongs to target, hardware, scheduling, routing,
 * calibration, deployment, or runtime subsystems.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT construct or define quantum IR.
 *
 * Source:
 *
 *     quantum::measurement
 *
 * may eventually become semantic metadata associated with operations in
 * `quantum::ir`, but that conversion occurs AFTER parsing and semantic
 * analysis.
 *
 * The dependency is therefore:
 *
 *     grammar
 *        |
 *        v
 *     frontend AST
 *        |
 *        v
 *     semantic effect model
 *        |
 *        v
 *     quantum::ir
 *
 * and NEVER:
 *
 *     grammar
 *        |
 *        v
 *     quantum::ir
 *        |
 *        v
 *     grammar
 *
 * ============================================================================
 * QEC BOUNDARY
 * ============================================================================
 *
 * Quantum error correction is NOT implemented here.
 *
 * A source-level effect reference such as:
 *
 *     quantum::error_correction
 *
 * may be syntactically represented.
 *
 * The grammar does not determine:
 *
 *     - code family;
 *     - distance;
 *     - decoder;
 *     - syndrome extraction;
 *     - logical-qubit implementation;
 *     - physical-qubit mapping;
 *     - correction strategy.
 *
 * QEC remains the responsibility of the QEC subsystem.
 *
 * ============================================================================
 * ZQN BOUNDARY
 * ============================================================================
 *
 * ZQN describes quantum noise/fault semantics.
 *
 * This grammar does not define:
 *
 *     - noise channels;
 *     - fault distributions;
 *     - correlated faults;
 *     - leakage;
 *     - loss;
 *     - erasure;
 *     - calibration data;
 *     - fault locations.
 *
 * A future or existing effect name such as:
 *
 *     zqn::observation
 *
 * remains a normal effect reference outside this file.
 *
 * Quantum effects may coexist with ZQN metadata downstream without making
 * this grammar responsible for ZQN.
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Capabilities are owned by:
 *
 *     grammar/core/capabilities.g4
 *     grammar/effects/capabilities.g4
 *
 * This file MUST NOT redefine:
 *
 *     capabilityReference
 *     capabilityVersionClause
 *     capabilityDeclaration
 *
 * For example:
 *
 *     quantum::measurement
 *
 * is an EFFECT identity.
 *
 * A capability such as:
 *
 *     quantum::measurement
 *
 * may independently exist in the capability namespace.
 *
 * Semantic analysis determines the relationship.
 *
 * The grammar MUST NOT assume:
 *
 *     effect == capability
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * This file does not own resource requirements.
 *
 * Therefore it MUST NOT introduce syntax such as:
 *
 *     quantum::measurement requires 8 qubits
 *
 * merely to describe a quantum effect.
 *
 * Resource expressions belong to:
 *
 *     grammar/resources/
 *
 * and are attached through the appropriate general requirement/resource
 * mechanisms.
 *
 * This keeps:
 *
 *     effect
 *     capability
 *     resource
 *     requirement
 *     constraint
 *     preference
 *
 * as distinct concepts.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar does not know whether an effect is realized by:
 *
 *     - a quantum processor;
 *     - a quantum simulator;
 *     - a CPU simulation;
 *     - a GPU simulator;
 *     - a distributed simulator;
 *     - a future quantum architecture;
 *     - another computational substrate.
 *
 * Semantic lowering and target selection determine realization.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no runtime calls;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no target-dependent branches.
 *
 * Therefore parsing is determined exclusively by the token stream.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     - source ordering;
 *     - namespace path;
 *     - effect segments;
 *     - source span;
 *     - original source spelling where diagnostics require it.
 *
 * Semantic analysis may canonicalize the namespace path after parsing.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar conceptually produces:
 *
 *     QuantumEffectReferenceAst
 *     {
 *         namespace
 *         path
 *         source_span
 *     }
 *
 * and:
 *
 *     QuantumEffectReferenceListAst
 *     {
 *         references
 *         source_span
 *     }
 *
 * The actual Rust AST types remain owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser establishes:
 *
 *     1. The reference begins in the canonical `quantum` namespace.
 *     2. The reference contains one or more effect path segments.
 *     3. The syntax is structurally valid.
 *
 * Semantic analysis determines:
 *
 *     - whether the effect exists;
 *     - whether it is imported;
 *     - whether it is visible;
 *     - whether it is deprecated;
 *     - whether its version is compatible;
 *     - whether its use is legal in the enclosing context;
 *     - what capabilities it requires;
 *     - what resources it may require;
 *     - what quantum semantic operations it induces;
 *     - whether it can be lowered to quantum::ir;
 *     - whether the selected target can realize it.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT SETS
 * ============================================================================
 *
 * `grammar/effects/effect-sets.g4` remains the canonical owner of GENERAL
 * effect references:
 *
 *     effectReference
 *
 * Therefore this file MUST NOT replace that rule.
 *
 * Instead, effect sets may use:
 *
 *     quantumEffectReference
 *
 * when a grammar context specifically needs to preserve the fact that the
 * referenced effect belongs to the quantum namespace.
 *
 * Example:
 *
 *     effects {
 *         quantum::measurement,
 *         quantum::readout,
 *     }
 *
 * The general effect parser may parse those names through `qualifiedName`.
 *
 * This specialized grammar provides a reusable, explicitly named boundary
 * for components that need quantum-domain awareness.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT DECLARATIONS
 * ============================================================================
 *
 * `grammar/effects/effect-declarations.g4` owns:
 *
 *     effectDeclaration
 *     effectOperationDeclaration
 *
 * It MUST NOT be redefined here.
 *
 * A quantum effect declaration remains an ordinary effect declaration whose
 * identity may be in the quantum namespace at semantic/module level.
 *
 * This file supplies only quantum effect references.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECT HANDLERS
 * ============================================================================
 *
 * `grammar/effects/effect-handling.g4` owns handler syntax.
 *
 * This file MUST NOT define:
 *
 *     handle
 *     resume
 *     abort
 *     handler
 *     continuation
 *
 * A quantum effect may be handled by the general effect system.
 *
 * Example:
 *
 *     handle computation {
 *         case quantum::measurement(q) => resume(result);
 *     }
 *
 * Handler semantics remain outside this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH CAPABILITIES
 * ============================================================================
 *
 * `grammar/effects/capabilities.g4` remains responsible for the syntax that
 * attaches capability requirements to effects.
 *
 * Example semantic relationship:
 *
 *     quantum::measurement
 *         |
 *         +--> requires quantum::readout capability
 *
 * The grammar does not determine that relationship.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM GRAMMAR
 * ============================================================================
 *
 * `grammar/quantum/` owns quantum computational source syntax, including:
 *
 *     - qubits;
 *     - quantum operations;
 *     - measurements;
 *     - reset;
 *     - circuits;
 *     - dynamic circuits;
 *     - quantum/classical interaction.
 *
 * This file MUST NOT duplicate any of those rules.
 *
 * The existing quantum parser architecture already establishes the flow:
 *
 *     quantum source
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * This file only contributes effect metadata at the effect-system boundary.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE
 * ============================================================================
 *
 * Hardware grammar owns:
 *
 *     - hardware descriptions;
 *     - targets;
 *     - capabilities;
 *     - topology;
 *     - placement;
 *     - accelerators.
 *
 * This file must not import hardware implementation grammar merely to parse
 * a quantum effect.
 *
 * Quantum effect syntax remains hardware-independent.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Resource grammar owns:
 *
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - resource expressions;
 *     - scalability;
 *     - latency;
 *     - energy;
 *     - reliability;
 *     - portability.
 *
 * This file references none of those concrete resource structures.
 *
 * Semantic analysis connects a quantum effect to resource requirements.
 *
 * ============================================================================
 * INTEGRATION WITH SCHEDULING
 * ============================================================================
 *
 * Scheduling MUST NOT depend directly on this grammar.
 *
 * Scheduling consumes semantic/IR information after parsing and analysis.
 *
 * Quantum effects may influence scheduling indirectly through semantic
 * operation/effect metadata.
 *
 * ============================================================================
 * INTEGRATION WITH ROUTING
 * ============================================================================
 *
 * Routing MUST NOT depend directly on this grammar.
 *
 * Routing consumes canonical quantum semantics and target capabilities.
 *
 * A quantum effect does not select a physical topology.
 *
 * ============================================================================
 * INTEGRATION WITH OPTIMIZATION
 * ============================================================================
 *
 * Optimization MUST NOT interpret this grammar directly.
 *
 * Optimization consumes canonical semantic/IR representations.
 *
 * Effect metadata may constrain transformations when semantic preservation
 * requires it.
 *
 * ============================================================================
 * INTEGRATION WITH RESILIENCE
 * ============================================================================
 *
 * Resilience may consume the semantic representation of quantum effects when
 * deciding whether an adaptation or recovery strategy preserves program
 * semantics.
 *
 * This grammar does NOT implement:
 *
 *     retry
 *     restart
 *     rollback
 *     remap
 *     reroute
 *     reschedule
 *     recompile
 *     backend switching
 *     quarantine
 *     abort policy.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Quantum effects may eventually be lowered to:
 *
 *     - Zamani quantum::ir;
 *     - OpenQASM;
 *     - QIR;
 *     - simulator representations;
 *     - vendor-independent target representations;
 *     - other future quantum representations.
 *
 * Those conversions belong to interoperability/lowering layers.
 *
 * This grammar does not encode external representation syntax.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Adding:
 *
 *     quantum::measurement
 *     quantum::reset
 *     quantum::new_future_effect
 *
 * does NOT require a grammar version change.
 *
 * A grammar version change is required only if the SYNTAX of quantum effect
 * references changes.
 *
 * This preserves POCO-REAF and future extensibility.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical namespace spelling is:
 *
 *     quantum
 *
 * followed by:
 *
 *     ::
 *
 * followed by one or more identifier segments.
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::readout
 *     quantum::dynamic::control
 *
 * Existing general effect syntax:
 *
 *     effects { quantum::Measurement }
 *
 * remains compatible because `quantum::Measurement` is also a qualified name.
 *
 * ============================================================================
 * NEGATIVE SEMANTIC EXAMPLES
 * ============================================================================
 *
 * These are NOT parser errors necessarily; they are intentionally outside the
 * responsibility of this grammar:
 *
 *     quantum::unknown_effect
 *
 * The parser accepts it.
 *
 * Semantic analysis may reject it if no definition exists.
 *
 * Likewise:
 *
 *     quantum::measurement
 *
 * does not imply a particular machine.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_QUBITS
 *     MAX_QUANTUM_EFFECTS
 *     MAX_QPU
 *     MAX_DEVICES
 *     MAX_EFFECTS
 *     IBM
 *     NVIDIA
 *     Rigetti
 *     IonQ
 *     physical qubit identifiers
 *     fixed topology names
 *     fixed gate inventories
 *     fixed calibration identifiers
 *
 * The only domain-specific lexical reservation used here is:
 *
 *     K_QUANTUM
 *
 * because `quantum` is an established language namespace keyword.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/lexer/tokens.g4
 *
 * Lexer grammar:
 *
 *     ZamaniTokens
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniTokens
 *
 * is mandatory.
 *
 * Canonical parser dependencies:
 *
 *     Core
 *
 * Core provides:
 *
 *     identifier
 *
 * This file MUST NOT redefine identifier syntax.
 *
 * ============================================================================
 * IMPORT RULE
 * ============================================================================
 *
 * This grammar intentionally imports only Core.
 *
 * It does NOT import:
 *
 *     Quantum
 *     QuantumTypes
 *     Effects
 *     EffectSets
 *     Capabilities
 *     Resources
 *     Hardware
 *
 * because the namespace/reference grammar must remain independent of those
 * higher-level grammars.
 *
 * This makes this file independently completable and prevents cycles such as:
 *
 *     quantum effects
 *          -> quantum grammar
 *          -> effect grammar
 *          -> quantum effects
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniTokens
 *          |
 *          v
 *        Core
 *          |
 *          v
 *    QuantumEffects
 *          |
 *          +--> EffectSets
 *          +--> EffectDeclarations
 *          +--> EffectHandling
 *          |
 *          v
 *    semantic analysis
 *          |
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> quantum semantic analysis
 *          |
 *          v
 *      quantum::ir
 *
 * The reverse direction is forbidden.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar QuantumEffects;

options {
    tokenVocab = ZamaniTokens;
}

import Core;


/*
 * ============================================================================
 * 1. QUANTUM EFFECT REFERENCE
 * ============================================================================
 *
 * Canonical forms:
 *
 *     quantum::measurement
 *     quantum::readout
 *     quantum::reset
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *
 * The first segment is the reserved `quantum` namespace.
 *
 * Remaining segments are open-world identifiers.
 *
 * There is intentionally no closed enumeration of effect names.
 */
quantumEffectReference
    : K_QUANTUM
      DOUBLE_COLON
      quantumEffectPath
    ;


/*
 * ============================================================================
 * 2. QUANTUM EFFECT PATH
 * ============================================================================
 *
 * One or more identifier segments.
 *
 * Examples:
 *
 *     measurement
 *     readout
 *     dynamic_control
 *     dynamic::control
 *     photonic::interaction
 *     future::measurement
 *
 * No maximum path depth is encoded.
 */
quantumEffectPath
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/*
 * ============================================================================
 * 3. QUANTUM EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Examples:
 *
 *     quantum::measurement
 *
 *     quantum::measurement,
 *     quantum::readout
 *
 *     quantum::measurement,
 *     quantum::readout,
 *     quantum::reset,
 *
 * A trailing comma is intentionally accepted.
 */
quantumEffectReferenceList
    : quantumEffectReference
      (
          COMMA
          quantumEffectReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. QUANTUM EFFECT GROUP
 * ============================================================================
 *
 * This rule provides a reusable grouping boundary for grammar consumers that
 * need a collection containing only quantum effects.
 *
 * Example:
 *
 *     {
 *         quantum::measurement,
 *         quantum::readout,
 *     }
 *
 * Empty groups are syntactically valid.
 *
 * Whether an empty group has special semantic meaning is decided downstream.
 */
quantumEffectGroup
    : LBRACE
      quantumEffectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 5. NON-EMPTY QUANTUM EFFECT GROUP
 * ============================================================================
 *
 * Useful for grammar contexts where at least one quantum effect must be
 * explicitly present.
 *
 * This is a syntactic distinction only.
 */
nonEmptyQuantumEffectGroup
    : LBRACE
      quantumEffectReferenceList
      RBRACE
    ;


/*
 * ============================================================================
 * 6. SINGLE QUANTUM EFFECT
 * ============================================================================
 *
 * Explicitly named alias for grammar consumers that need one quantum effect
 * without reaching into the reference structure.
 */
singleQuantumEffect
    : quantumEffectReference
    ;


/*
 * ============================================================================
 * 7. QUANTUM EFFECT ENTRY
 * ============================================================================
 *
 * Alias for collection-oriented grammar consumers.
 */
quantumEffectEntry
    : quantumEffectReference
    ;


/*
 * ============================================================================
 * 8. QUANTUM EFFECT ENTRIES
 * ============================================================================
 *
 * Non-empty source collection.
 */
quantumEffectEntries
    : quantumEffectEntry
      (
          COMMA
          quantumEffectEntry
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. QUANTUM EFFECT QUALIFIER
 * ============================================================================
 *
 * Provides a reusable syntactic marker for contexts where a grammar needs to
 * state that the following reference belongs to the quantum effect namespace.
 *
 * Example:
 *
 *     quantum::measurement
 *
 * This rule deliberately carries no semantic interpretation.
 */
quantumEffectQualifier
    : K_QUANTUM
      DOUBLE_COLON
    ;


/*
 * ============================================================================
 * 10. QUANTUM EFFECT NAME
 * ============================================================================
 *
 * Represents the terminal semantic name after the quantum namespace.
 *
 * Example:
 *
 *     quantum::measurement
 *                    ^^^^^^^^^^^
 *
 * Nested namespace paths are represented by quantumEffectPath.
 *
 * This rule is useful only for grammar consumers that intentionally need the
 * final segment.
 */
quantumEffectName
    : identifier
    ;


/*
 * ============================================================================
 * 11. QUANTUM EFFECT REFERENCE WITH FINAL NAME
 * ============================================================================
 *
 * Explicit decomposition:
 *
 *     namespace qualifier + path
 *
 * This rule is equivalent in accepted syntax to quantumEffectReference but
 * provides a stable parse-tree boundary for tooling.
 */
quantumEffectQualifiedReference
    : quantumEffectQualifier
      quantumEffectPath
    ;


/*
 * ============================================================================
 * 12. QUANTUM EFFECT COLLECTION
 * ============================================================================
 *
 * General collection boundary.
 *
 * The collection may be empty.
 */
quantumEffectCollection
    : quantumEffectGroup
    ;


/*
 * ============================================================================
 * 13. QUANTUM EFFECT DECLARATION REFERENCE
 * ============================================================================
 *
 * This rule represents a reference to an already-declared quantum effect.
 *
 * IMPORTANT:
 *
 * It does not declare an effect.
 *
 * Declaration ownership remains with:
 *
 *     grammar/effects/effect-declarations.g4
 */
quantumEffectDeclarationReference
    : quantumEffectReference
    ;


/*
 * ============================================================================
 * 14. QUANTUM EFFECT REQUIREMENT REFERENCE
 * ============================================================================
 *
 * This is a syntactic boundary for requirement-bearing consumers.
 *
 * It does NOT define resource requirements or capabilities.
 *
 * Example:
 *
 *     quantum::measurement
 *
 * Semantic analysis determines what is required to realize it.
 */
quantumEffectRequirementReference
    : quantumEffectReference
    ;


/*
 * ============================================================================
 * 15. QUANTUM EFFECT HANDLER REFERENCE
 * ============================================================================
 *
 * Handler grammars may use this rule when they need an explicitly quantum
 * effect reference.
 *
 * Handler implementation remains outside this file.
 */
quantumEffectHandlerReference
    : quantumEffectReference
    ;


/*
 * ============================================================================
 * 16. QUANTUM EFFECT INVOCATION TARGET
 * ============================================================================
 *
 * This rule intentionally stops at the effect identity.
 *
 * Arguments, expressions and invocation semantics belong to the general
 * effect-handling/effect-invocation grammar.
 *
 * This prevents the quantum effect grammar from creating a second call syntax.
 */
quantumEffectInvocationTarget
    : quantumEffectReference
    ;


/*
 * ============================================================================
 * 17. QUANTUM EFFECT PATH SEGMENT
 * ============================================================================
 *
 * Explicit single-segment boundary for tooling and diagnostics.
 */
quantumEffectPathSegment
    : identifier
    ;


/*
 * ============================================================================
 * 18. QUANTUM EFFECT NAMESPACE
 * ============================================================================
 *
 * Canonical namespace token.
 *
 * This rule exists as an explicit named boundary so semantic tooling can
 * identify the quantum namespace without depending on token-level details.
 */
quantumEffectNamespace
    : K_QUANTUM
    ;


/*
 * ============================================================================
 * 19. QUANTUM EFFECT QUALIFIED PATH
 * ============================================================================
 *
 * Equivalent structural representation of:
 *
 *     quantum::foo::bar::baz
 *
 * The namespace is fixed to `quantum`; the remainder remains open-world.
 */
quantumEffectQualifiedPath
    : quantumEffectNamespace
      DOUBLE_COLON
      quantumEffectPath
    ;


/*
 * ============================================================================
 * 20. QUANTUM EFFECT LIST ENTRY
 * ============================================================================
 *
 * Explicit list-entry boundary.
 */
quantumEffectListEntry
    : quantumEffectQualifiedPath
    ;


/*
 * ============================================================================
 * 21. QUANTUM EFFECT LIST
 * ============================================================================
 *
 * Arbitrarily many entries.
 */
quantumEffectList
    : quantumEffectListEntry
      (
          COMMA
          quantumEffectListEntry
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 22. QUANTUM EFFECT SET
 * ============================================================================
 *
 * Source-level quantum effect collection.
 *
 * This does not decide whether the collection is semantically a set, sequence,
 * multiset, or normalized collection.
 */
quantumEffectSet
    : LBRACE
      quantumEffectList?
      RBRACE
    ;


/*
 * ============================================================================
 * 23. NON-EMPTY QUANTUM EFFECT SET
 * ============================================================================
 */
nonEmptyQuantumEffectSet
    : LBRACE
      quantumEffectList
      RBRACE
    ;


/*
 * ============================================================================
 * 24. QUANTUM EFFECT REFERENCE ALIAS
 * ============================================================================
 *
 * Structural alias for parser consumers.
 *
 * This does not introduce an alias declaration syntax.
 */
quantumEffectReferenceAlias
    : quantumEffectReference
    ;


/*
 * ============================================================================
 * 25. QUANTUM EFFECT DOMAIN PATH
 * ============================================================================
 *
 * Allows future namespaces below `quantum` without modifying this grammar.
 *
 * Examples:
 *
 *     quantum::error_correction
 *     quantum::error_correction::syndrome
 *     quantum::photonic::interaction
 *     quantum::future::operation
 */
quantumEffectDomainPath
    : quantumEffectPath
    ;


/*
 * ============================================================================
 * 26. QUANTUM EFFECT DOMAIN REFERENCE
 * ============================================================================
 *
 * Explicit domain-level boundary.
 */
quantumEffectDomainReference
    : quantumEffectNamespace
      DOUBLE_COLON
      quantumEffectDomainPath
    ;


/*
 * ============================================================================
 * 27. QUANTUM EFFECT SOURCE REFERENCE
 * ============================================================================
 *
 * Canonical source reference boundary for downstream grammar composition.
 */
quantumEffectSourceReference
    : quantumEffectDomainReference
    ;


/*
 * ============================================================================
 * 28. QUANTUM EFFECT METADATA TARGET
 * ============================================================================
 *
 * Represents an effect identity to which semantic metadata may later be
 * attached.
 *
 * This grammar does not define the metadata itself.
 */
quantumEffectMetadataTarget
    : quantumEffectReference
    ;


/*
 * ============================================================================
 * 29. QUANTUM EFFECT SYMBOL
 * ============================================================================
 *
 * Named symbol boundary for diagnostics and tooling.
 */
quantumEffectSymbol
    : quantumEffectReference
    ;


/*
 * ============================================================================
 * 30. QUANTUM EFFECT REFERENCE ROOT
 * ============================================================================
 *
 * Stable aggregate entry rule.
 *
 * Grammar clients that need exactly one quantum effect should depend on this
 * rule instead of depending on internal decomposition.
 */
quantumEffectReferenceRoot
    : quantumEffectReference
    ;