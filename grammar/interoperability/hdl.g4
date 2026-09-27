/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/interoperability/hdl.g4
 *
 * STATUS
 * ------
 * PRODUCTION HDL INTEROPERABILITY ADAPTER
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * RUST BASELINE
 * -------------
 * Rust 1.97 / Rust 1.97.1
 * Rust edition 2021
 *
 * SAFETY
 * ------
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no semantic actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware access;
 *   - no runtime execution;
 *   - no target discovery;
 *   - no unsafe implementation requirement.
 *
 * The Rust implementation consuming this grammar MUST remain safe Rust.
 *
 * Recommended compiler-level enforcement:
 *
 *     #![deny(unsafe_code)]
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the interoperability boundary for HDL represented through
 * Zamani's canonical HDL grammar.
 *
 * IMPORTANT:
 *
 * This file is NOT a second HDL grammar.
 *
 * The authoritative detailed HDL syntax remains:
 *
 *     grammar/hdl/hdl.g4
 *
 * The interoperability layer imports that grammar and exposes a stable
 * interoperability entry point.
 *
 * This prevents:
 *
 *     grammar/hdl/hdl.g4
 *     grammar/interoperability/hdl.g4
 *
 * from independently defining the same HDL language.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * The architectural authority chain is:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/specification/
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          v
 *     grammar/interoperability/hdl.g4
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical hardware semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          +-------------------------------+
 *          |               |               |
 *          v               v               v
 *       optimize       verify          schedule
 *          |               |               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                    placement/routing
 *                          |
 *                          v
 *                       synthesis
 *                          |
 *                          v
 *                   target realization
 *
 * This file does not supersede any stage above it.
 *
 * ============================================================================
 * SINGLE HDL AUTHORITY
 * ============================================================================
 *
 * The detailed HDL language is owned by:
 *
 *     grammar/hdl/hdl.g4
 *
 * Existing HDL components remain owned by their existing files, including
 * where present:
 *
 *     grammar/hdl/hardware-modules.g4
 *     grammar/hdl/hardware-generics.g4
 *     grammar/hdl/ports.g4
 *     grammar/hdl/signals.g4
 *     grammar/hdl/nets.g4
 *     grammar/hdl/registers.g4
 *     grammar/hdl/memories.g4
 *     grammar/hdl/clocks.g4
 *     grammar/hdl/clocking.g4
 *     grammar/hdl/timing.g4
 *     grammar/hdl/processes.g4
 *     grammar/hdl/combinational.g4
 *     grammar/hdl/sequential.g4
 *     grammar/hdl/state-machines.g4
 *     grammar/hdl/pipelines.g4
 *     grammar/hdl/generate.g4
 *     grammar/hdl/interfaces.g4
 *     grammar/hdl/hardware-interfaces.g4
 *     grammar/hdl/assertions.g4
 *     grammar/hdl/synthesis.g4
 *     grammar/hdl/simulation.g4
 *     grammar/hdl/verification.g4
 *     grammar/hdl/physical-intent.g4
 *     grammar/hdl/co-design.g4
 *
 * The exact repository tree is authoritative for which subordinate files
 * currently exist.
 *
 * This interoperability file MUST NOT copy their rules.
 *
 * ============================================================================
 * WHAT THIS FILE OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *   1. the HDL interoperability parser-grammar identity;
 *   2. import of the canonical HDL parser grammar;
 *   3. the stable HDL interoperability entry point;
 *   4. the stable source-boundary name used by interoperability composition;
 *   5. interoperability-level documentation of the HDL boundary.
 *
 * ============================================================================
 * WHAT THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - HDL modules;
 *   - ports;
 *   - signals;
 *   - nets;
 *   - wires;
 *   - registers;
 *   - memories;
 *   - clocks;
 *   - resets;
 *   - timing;
 *   - processes;
 *   - combinational logic;
 *   - sequential logic;
 *   - state machines;
 *   - pipelines;
 *   - generation;
 *   - interfaces;
 *   - protocols;
 *   - assertions;
 *   - simulation;
 *   - synthesis;
 *   - physical intent;
 *   - co-design;
 *   - hardware discovery;
 *   - target selection;
 *   - placement;
 *   - routing;
 *   - physical design;
 *   - vendor implementation;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution.
 *
 * Those responsibilities remain downstream or belong to their existing
 * owning grammar/domain.
 *
 * ============================================================================
 * WHY THIS FILE IS NECESSARY
 * ============================================================================
 *
 * The repository already has:
 *
 *     grammar/hdl/hdl.g4
 *
 * as the canonical HDL composition grammar.
 *
 * Interoperability nevertheless needs a stable HDL boundary because HDL can
 * participate in the wider Zamani interoperability system alongside:
 *
 *     C
 *     C++
 *     Python
 *     Rust
 *     OpenQASM
 *     QIR
 *     Verilog/SystemVerilog-style formats
 *     other HDL representations
 *     accelerator interfaces
 *     hardware/software co-design
 *
 * This file provides that boundary WITHOUT creating another HDL language.
 *
 * ============================================================================
 * ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * There is exactly ONE detailed Zamani HDL syntax authority:
 *
 *     grammar/hdl/hdl.g4
 *
 * There is exactly ONE interoperability HDL boundary:
 *
 *     grammar/interoperability/hdl.g4
 *
 * Therefore:
 *
 *     HDL syntax
 *         |
 *         v
 *     grammar/hdl/hdl.g4
 *         |
 *         v
 *     grammar/interoperability/hdl.g4
 *         |
 *         v
 *     interoperability composition
 *
 * The interoperability layer MUST NOT fork HDL syntax.
 *
 * ============================================================================
 * ANTLR COMPOSITION MODEL
 * ============================================================================
 *
 * `HDL` is a parser grammar.
 *
 * This parser grammar imports it.
 *
 * This is deliberately different from importing a combined lexer/parser
 * grammar.
 *
 * The canonical HDL grammar already consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Consequently this interoperability grammar consumes the same canonical
 * lexical vocabulary.
 *
 * There is no interoperability-specific lexer.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The lexical authority remains the Zamani lexer.
 *
 * Conceptually:
 *
 *     grammar/lexer/
 *             |
 *             v
 *     grammar/antlr/ZamaniLexer.g4
 *             |
 *             v
 *     ZamaniLexer
 *             |
 *             +----------------------+
 *             |                      |
 *             v                      v
 *        canonical HDL         interoperability
 *
 * This file MUST NOT declare lexer rules.
 *
 * This file MUST NOT introduce:
 *
 *     HDL_IDENTIFIER
 *     HDL_NUMBER
 *     HDL_STRING
 *     HDL_WIDTH
 *     HDL_REGISTER
 *
 * or any other second lexical vocabulary.
 *
 * ============================================================================
 * IMPORT
 * ============================================================================
 *
 * `HDL` is the canonical HDL parser composition grammar.
 *
 * All detailed HDL syntax remains there.
 *
 * ============================================================================
 */

parser grammar HdlInterop;

options {
    tokenVocab = ZamaniLexer;
}

import HDL;


/*
 * ============================================================================
 * PUBLIC INTEROPERABILITY ENTRY POINT
 * ============================================================================
 *
 * `hdlInteroperability` is the stable entry point for the interoperability
 * subsystem.
 *
 * It delegates completely to the canonical HDL source/design grammar.
 *
 * The imported `hdlDesign` rule remains the detailed HDL source boundary.
 *
 * Because `hdlDesign` already owns the complete source-unit and EOF boundary,
 * this adapter does not add another EOF or source-unit grammar.
 *
 * This is intentional.
 *
 * The interoperability layer therefore cannot accidentally create:
 *
 *     HDL source
 *     HDL source
 *     HDL source
 *
 * with subtly different parsing semantics.
 *
 * ============================================================================
 */

hdlInteroperability
    : hdlDesign
    ;


/*
 * ============================================================================
 * EXPLICIT CANONICAL ALIAS
 * ============================================================================
 *
 * This rule provides a descriptive interoperability name for consumers that
 * want to distinguish:
 *
 *     ordinary HDL grammar composition
 *
 * from:
 *
 *     HDL used as an interoperability boundary.
 *
 * It does not alter syntax or semantics.
 *
 * ============================================================================
 */

hdlInteroperabilitySource
    : hdlInteroperability
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar establishes NO new HDL semantics.
 *
 * Therefore:
 *
 *     hdlInteroperability
 *         |
 *         v
 *     hdlDesign
 *         |
 *         v
 *     existing HDL AST contract
 *
 * Any semantic meaning is determined by the canonical HDL semantic layer.
 *
 * This prevents interoperability from introducing a second hardware semantic
 * model.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The interoperability boundary MUST NOT introduce a second AST for HDL.
 *
 * The expected path is:
 *
 *     HDL interoperability source
 *             |
 *             v
 *     canonical HDL parser grammar
 *             |
 *             v
 *     existing domain-neutral frontend AST
 *             |
 *             v
 *     hardware semantic analysis
 *             |
 *             v
 *     canonical hardware/HDL semantic representation
 *             |
 *             v
 *     canonical IR
 *
 * In particular, this file MUST NOT introduce:
 *
 *     HdlInteropAst
 *     HdlInteropModule
 *     HdlInteropPort
 *     HdlInteropSignal
 *     HdlInteropNet
 *     HdlInteropMemory
 *     HdlInteropRegister
 *     HdlInteropNetlist
 *
 * merely because the source arrived through interoperability.
 *
 * Interoperability is a source boundary, not a new semantic universe.
 *
 * ============================================================================
 * SOURCE PROVENANCE
 * ============================================================================
 *
 * The adapter must preserve source provenance.
 *
 * The source span of:
 *
 *     hdlInteroperability
 *
 * is the source span of the delegated canonical HDL design.
 *
 * No semantic information may be lost merely because the source passed
 * through this interoperability boundary.
 *
 * Source spans remain necessary for:
 *
 *     diagnostics;
 *     IDE navigation;
 *     source maps;
 *     semantic errors;
 *     synthesis diagnostics;
 *     verification diagnostics;
 *     portability diagnostics.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file defines NO new IR.
 *
 * The intended path is:
 *
 *     HDL source
 *         |
 *         v
 *     canonical frontend AST
 *         |
 *         v
 *     hardware semantic model
 *         |
 *         v
 *     canonical hardware/HDL IR
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     verification
 *         |
 *         v
 *     synthesis
 *         |
 *         v
 *     placement
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     target realization
 *
 * For mixed quantum/hardware computation:
 *
 *     HDL source
 *         |
 *         +-----------------------------+
 *         |                             |
 *         v                             v
 *     hardware semantics          quantum semantics
 *                                       |
 *                                       v
 *                                  quantum::ir
 *
 * There MUST NOT be:
 *
 *     HdlQuantumIR
 *     HdlQubitIR
 *     HdlGateIR
 *
 * introduced by this file.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * HDL interoperability may participate in quantum/hybrid programs.
 *
 * Examples of legitimate architectural relationships include:
 *
 *     HDL control
 *          |
 *          v
 *     quantum computation
 *
 * and:
 *
 *     classical control
 *          |
 *          v
 *     HDL accelerator
 *          |
 *          v
 *     quantum device
 *
 * However:
 *
 *     HDL interoperability != quantum syntax
 *
 * and:
 *
 *     HDL interoperability != quantum IR
 *
 * Quantum syntax remains owned by the quantum subsystem.
 *
 * Quantum operations ultimately lower through:
 *
 *     quantum::ir
 *
 * QEC remains downstream.
 *
 * ZQN remains downstream.
 *
 * Routing remains downstream.
 *
 * Scheduling remains downstream.
 *
 * HAL remains downstream.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * HDL interoperability must remain compatible with classical Zamani
 * computation.
 *
 * The same source program may contain semantic relationships such as:
 *
 *     classical computation
 *          |
 *          v
 *     accelerator
 *          |
 *          v
 *     HDL implementation
 *          |
 *          v
 *     classical result
 *
 * The interoperability adapter must not force a separate programming model
 * for the hardware boundary.
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * HDL can participate in:
 *
 *     classical + HDL
 *     quantum + HDL
 *     classical + quantum + HDL
 *     AI + HDL
 *     distributed + HDL
 *     data + HDL
 *
 * The interoperability boundary remains syntactic only.
 *
 * Cross-domain semantic composition belongs to the semantic layer.
 *
 * ============================================================================
 * RESOURCE AND CAPABILITY INTEGRATION
 * ============================================================================
 *
 * HDL source may express portable resource and capability intent through the
 * canonical Zamani resource system.
 *
 * Examples of semantic intent include:
 *
 *     requires capability("hardware.pipeline")
 *
 *     requires capability("hardware.memory")
 *
 *     requires capability("hardware.interface")
 *
 *     requires capability("accelerator.compute")
 *
 *     requires memory >= required_memory
 *
 * Such requirements do NOT allocate physical resources during parsing.
 *
 * This grammar does not evaluate them.
 *
 * Target satisfaction is checked downstream.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / HINT
 * ============================================================================
 *
 * The interoperability boundary preserves the distinction between:
 *
 *     REQUIREMENT
 *         Must be satisfied.
 *
 *     CAPABILITY
 *         Must be provided by a realization.
 *
 *     CONSTRAINT
 *         Narrows legal realizations.
 *
 *     PREFERENCE
 *         Optimization guidance.
 *
 *     HINT
 *         Non-binding implementation information.
 *
 *     REALIZATION
 *         Actual target-specific implementation decision.
 *
 * This file does not collapse those concepts.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The purpose of this boundary is consistent with:
 *
 *     Program_Once
 *     Compile_Once
 *     Run_Everywhere
 *     Run_Anywhere
 *     Run_Forever
 *
 * An HDL interoperability source should describe portable hardware intent
 * where portability is intended.
 *
 * It should not silently force:
 *
 *     one FPGA;
 *     one ASIC;
 *     one board;
 *     one package;
 *     one physical pin;
 *     one routing track;
 *     one BRAM;
 *     one DSP;
 *     one clock primitive;
 *     one vendor;
 *     one accelerator;
 *     one physical memory block.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file imposes NO language-level maximum on:
 *
 *     modules
 *     interfaces
 *     ports
 *     signals
 *     nets
 *     registers
 *     memories
 *     array dimensions
 *     states
 *     transitions
 *     pipeline stages
 *     instances
 *     generated instances
 *     processes
 *     clock domains
 *     timing relationships
 *     hierarchy depth
 *     generic parameters
 *     source elements
 *
 * The imported canonical HDL grammar is likewise required to remain free of
 * artificial universal hardware ceilings.
 *
 * "Infinity" means:
 *
 *     no arbitrary language-level hardware maximum.
 *
 * It does NOT mean:
 *
 *     infinite physical resources;
 *     infinite compilation resources;
 *     infinite synthesis capacity;
 *     infinite memory;
 *     infinite execution time.
 *
 * If a target cannot satisfy a program's requirements, the semantic/compiler/
 * deployment layers report a resource or capability failure.
 *
 * They do not rewrite the language with a smaller universal limit.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This adapter MUST NOT introduce or validate universal limits such as:
 *
 *     MAX_MODULES
 *     MAX_PORTS
 *     MAX_SIGNALS
 *     MAX_NETS
 *     MAX_REGISTERS
 *     MAX_MEMORIES
 *     MAX_WIDTH
 *     MAX_DEPTH
 *     MAX_PIPELINE_STAGES
 *     MAX_INSTANCES
 *     MAX_CLOCKS
 *     MAX_DEVICES
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * It must not introduce equivalent hidden limits.
 *
 * A source-level quantity such as:
 *
 *     width = 32
 *
 *     depth = 1024
 *
 *     lanes = 8
 *
 * remains legitimate program semantics.
 *
 * What is forbidden is interpreting those values as universal language
 * ceilings.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Portable HDL interoperability must not silently select:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     physical qubit 0
 *     physical pin 0
 *     memory bank 0
 *     routing channel 0
 *
 * Target-specific realization is a downstream concern.
 *
 * If target-specific syntax is deliberately requested, it must be represented
 * through an explicit target-specific semantic/dialect contract rather than
 * accidentally becoming universal HDL syntax.
 *
 * ============================================================================
 * VENDOR EXTENSIONS
 * ============================================================================
 *
 * Vendor-specific HDL constructs MUST NOT be added to this interoperability
 * adapter.
 *
 * Vendor extensions belong under the repository's explicit dialect/
 * interoperability mechanisms.
 *
 * A vendor extension must identify, as applicable:
 *
 *     dialect name;
 *     dialect version;
 *     syntax ownership;
 *     semantic mapping;
 *     capability requirements;
 *     target constraints;
 *     compatibility;
 *     lowering behavior.
 *
 * This prevents vendor syntax from becoming accidental core Zamani syntax.
 *
 * ============================================================================
 * FOREIGN HDL FORMATS
 * ============================================================================
 *
 * This file represents HDL interoperability at the Zamani grammar boundary.
 *
 * It does NOT become a Verilog parser, VHDL parser, SystemVerilog parser, or
 * vendor HDL parser.
 *
 * External HDL formats must have their own import/parsing adapters where
 * required.
 *
 * The resulting semantic representation must converge on Zamani's canonical
 * semantic hardware model.
 *
 * Therefore:
 *
 *     external HDL
 *         |
 *         v
 *     format adapter
 *         |
 *         v
 *     canonical Zamani semantic representation
 *
 * rather than:
 *
 *     external HDL
 *         |
 *         v
 *     permanent second HDL semantic universe
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes the same semantic representation regardless of
 * whether HDL arrived through:
 *
 *     ordinary Zamani HDL syntax
 *
 * or:
 *
 *     the interoperability HDL boundary.
 *
 * This allows:
 *
 *     parse
 *       |
 *       v
 *     semantic normalization
 *       |
 *       v
 *     canonical representation
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     synthesis
 *
 * without creating a separate compiler path solely because the source was
 * classified as interoperability.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * This grammar has NO runtime dependency.
 *
 * Parsing must never:
 *
 *     execute hardware;
 *     program an FPGA;
 *     configure an ASIC;
 *     contact a QPU;
 *     discover a device;
 *     allocate physical memory;
 *     invoke a simulator;
 *     invoke a synthesis tool;
 *     invoke a linker;
 *     access the network;
 *     access the filesystem.
 *
 * Runtime integration occurs after compilation and lowering.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling may use this boundary for:
 *
 *     syntax highlighting;
 *     parsing;
 *     completion;
 *     navigation;
 *     diagnostics;
 *     source mapping;
 *     semantic inspection;
 *     documentation;
 *     dependency analysis;
 *     interoperability analysis.
 *
 * Tooling must not interpret parsing as permission to execute foreign or
 * hardware behavior.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing through this grammar must depend only on:
 *
 *     source text;
 *     canonical token vocabulary;
 *     grammar version;
 *     explicitly selected dialect configuration.
 *
 * Parsing must not depend on:
 *
 *     CPU count;
 *     GPU availability;
 *     FPGA availability;
 *     QPU availability;
 *     target topology;
 *     filesystem contents;
 *     network state;
 *     environment variables;
 *     wall-clock time;
 *     randomness;
 *     runtime state.
 *
 * Identical source and identical parser configuration must produce equivalent
 * parse structures.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no execution mechanism.
 *
 * It must not:
 *
 *     execute source;
 *     load HDL libraries;
 *     invoke vendor tools;
 *     execute synthesis;
 *     access a device;
 *     inspect secrets;
 *     access environment variables;
 *     perform network requests;
 *     perform filesystem operations.
 *
 * ANTLR actions and predicates that violate these boundaries MUST NOT be
 * introduced.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Semantic errors belong to semantic analysis.
 *
 * Resource failures belong to resource analysis/compiler/deployment.
 *
 * Capability failures belong to capability analysis.
 *
 * Synthesis failures belong to synthesis.
 *
 * Timing failures belong to timing analysis.
 *
 * Placement failures belong to placement.
 *
 * Routing failures belong to routing.
 *
 * Runtime failures belong to runtime.
 *
 * This adapter MUST NOT convert downstream failures into parser semantics.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical HDL syntax remains owned by:
 *
 *     grammar/hdl/hdl.g4
 *
 * Existing HDL programs therefore retain the same grammar authority.
 *
 * Introducing this interoperability boundary must not require a second HDL
 * spelling merely to cross the interoperability subsystem.
 *
 * Compatibility migration must normalize equivalent representations into the
 * same semantic model.
 *
 * ============================================================================
 * RELATIONSHIP TO grammar/interoperability/README.md
 * ============================================================================
 *
 * That README owns the interoperability subsystem architecture.
 *
 * This file implements the HDL-specific grammar boundary described there.
 *
 * This file must not duplicate the complete interoperability architecture.
 *
 * ============================================================================
 * RELATIONSHIP TO grammar/hdl/README.md
 * ============================================================================
 *
 * grammar/hdl/README.md owns the HDL subsystem architecture.
 *
 * grammar/hdl/hdl.g4 owns detailed HDL grammar composition.
 *
 * This file is only the interoperability bridge.
 *
 * ============================================================================
 * RELATIONSHIP TO grammar/Zamani.g4
 * ============================================================================
 *
 * The canonical root grammar remains:
 *
 *     grammar/Zamani.g4
 *
 * It should expose interoperability through one canonical interoperability
 * composition boundary.
 *
 * Conceptually:
 *
 *     Zamani
 *        |
 *        v
 *     interoperability
 *        |
 *        v
 *     hdlInteroperability
 *        |
 *        v
 *     hdlDesign
 *        |
 *        v
 *     canonical HDL grammar
 *
 * The root grammar MUST NOT copy HDL rules merely to consume this adapter.
 *
 * ============================================================================
 * RELATIONSHIP TO grammar/grammar.md
 * ============================================================================
 *
 * grammar/grammar.md is the implementation-conformance reference.
 *
 * Once this file is integrated into the canonical composition path, its
 * implementation status should be reflected there.
 *
 * This file itself is not an implementation-conformance authority.
 *
 * ============================================================================
 * RELATIONSHIP TO Zamani-Grammar.md
 * ============================================================================
 *
 * Historical or proposed HDL interoperability syntax described in
 * Zamani-Grammar.md does not automatically become legal syntax.
 *
 * Promotion remains:
 *
 *     historical/proposed design
 *          |
 *          v
 *     semantic specification
 *          |
 *          v
 *     AST contract
 *          |
 *          v
 *     canonical grammar
 *          |
 *          v
 *     implementation
 *          |
 *          v
 *     tests
 *          |
 *          v
 *     stable feature
 *
 * ============================================================================
 * CROSS-DOMAIN PRESERVATION
 * ============================================================================
 *
 * This adapter must preserve the ability to compose HDL with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     memory
 *     concurrency
 *     resources
 *     hardware
 *     compile
 *     execution
 *
 * It must not introduce separate semantic rules merely because another domain
 * participates.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This grammar requires tests in:
 *
 *     grammar/tests/interoperability/
 *
 * and/or the repository's existing interoperability/HDL test locations.
 *
 * Minimum categories:
 *
 *     positive
 *     negative
 *     boundary
 *     scalability
 *     determinism
 *     compatibility
 *     cross-domain
 *     source-span
 *
 * ============================================================================
 * POSITIVE TESTS
 * ============================================================================
 *
 * Positive tests must demonstrate that canonical HDL source can be accepted
 * through the interoperability boundary, including representative constructs
 * already owned by grammar/hdl/.
 *
 * Examples should cover, as supported by the canonical HDL grammar:
 *
 *     modules
 *     parameters
 *     generics
 *     ports
 *     interfaces
 *     signals
 *     nets
 *     registers
 *     memories
 *     clocks
 *     resets
 *     processes
 *     combinational logic
 *     sequential logic
 *     state machines
 *     pipelines
 *     generated structures
 *     assertions
 *     verification intent
 *     synthesis intent
 *     physical intent
 *     co-design
 *
 * The exact accepted syntax must be inherited from grammar/hdl/hdl.g4 rather
 * than duplicated in this file.
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * Negative tests must verify that malformed canonical HDL remains rejected
 * when parsed through this interoperability boundary.
 *
 * Examples include:
 *
 *     malformed module
 *     malformed port
 *     malformed signal
 *     malformed memory
 *     malformed process
 *     malformed pipeline
 *     malformed state machine
 *     malformed assertion
 *     malformed timing construct
 *
 * Negative tests must validate parser behavior, not hardware capacity.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Boundary tests must include:
 *
 *     empty HDL design where the canonical grammar permits it;
 *     one source element;
 *     many source elements;
 *     long qualified names;
 *     deeply nested hierarchy;
 *     large parameter lists;
 *     large port lists;
 *     large signal collections;
 *     large generated structures;
 *     many pipeline stages;
 *     many state transitions;
 *     large memory dimensions.
 *
 * No test should establish an artificial universal maximum.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Scalability must be tested parametrically.
 *
 * The same grammar must be able to represent designs whose sizes are
 * determined by source-level parameters.
 *
 * Examples:
 *
 *     module<Small>
 *     module<Large>
 *     module<Parameterized>
 *
 * where size is a program/semantic property rather than a grammar constant.
 *
 * A compiler implementation may have explicit operational resource limits for
 * denial-of-service protection or resource management.
 *
 * Such implementation limits MUST remain outside language semantics.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Required integration coverage includes, where supported:
 *
 *     classical + HDL
 *     quantum + HDL
 *     classical + quantum + HDL
 *     AI + HDL
 *     distributed + HDL
 *     data + HDL
 *     hardware + HDL
 *     resources + HDL
 *     compile + HDL
 *     execution + HDL
 *
 * These tests must verify that all domains converge into the same semantic
 * architecture.
 *
 * ============================================================================
 * QUANTUM PRESERVATION TESTS
 * ============================================================================
 *
 * A mixed HDL/quantum program must preserve quantum semantics through:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *
 * The HDL interoperability boundary must not introduce:
 *
 *     physical qubit IDs;
 *     fixed qubit counts;
 *     duplicate gate semantics;
 *     duplicate quantum IR;
 *     target-specific quantum placement.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Audit this file and all imported HDL composition boundaries for:
 *
 *     fixed widths;
 *     fixed depths;
 *     fixed module counts;
 *     fixed port counts;
 *     fixed signal counts;
 *     fixed register counts;
 *     fixed memory capacities;
 *     fixed pipeline lengths;
 *     fixed FPGA resources;
 *     fixed ASIC resources;
 *     fixed CPU resources;
 *     fixed GPU resources;
 *     fixed QPU resources;
 *     fixed node counts;
 *     fixed device counts;
 *     fixed topology;
 *     fixed physical addresses;
 *     fixed physical pins.
 *
 * Each occurrence must be classified as:
 *
 *     1. source-level program value;
 *     2. semantic requirement;
 *     3. explicit target constraint;
 *     4. implementation limitation;
 *     5. test fixture;
 *     6. documentation example;
 *     7. accidental language hard-coding.
 *
 * Accidental language hard-coding MUST be removed.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * No Rust code is embedded in this grammar.
 *
 * The parser implementation must support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * with no unsafe Rust.
 *
 * This grammar itself cannot introduce `unsafe` because it contains no
 * executable Rust actions.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * This adapter adds only a parser-rule delegation layer.
 *
 * It MUST NOT:
 *
 *     duplicate parsing;
 *     duplicate AST construction;
 *     reparse the same HDL source;
 *     copy large source structures unnecessarily;
 *     perform semantic work;
 *     perform target discovery.
 *
 * Performance-critical work remains downstream.
 *
 * ============================================================================
 * MAINTAINABILITY
 * ============================================================================
 *
 * The critical maintainability invariant is:
 *
 *     grammar/hdl/hdl.g4
 *             |
 *             v
 *     grammar/interoperability/hdl.g4
 *
 * rather than:
 *
 *     grammar/hdl/hdl.g4
 *             |
 *             +--------------------+
 *             |                    |
 *             v                    v
 *       HDL definition      duplicate HDL definition
 *
 * The second architecture is explicitly prohibited.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] It is a parser grammar.
 *   [x] It uses the canonical ZamaniLexer vocabulary.
 *   [x] It declares no lexer rules.
 *   [x] It imports the canonical HDL parser grammar.
 *   [x] It does not duplicate HDL syntax.
 *   [x] It exposes one stable interoperability entry point.
 *   [x] It preserves the canonical HDL source boundary.
 *   [x] It introduces no second HDL AST.
 *   [x] It introduces no second hardware semantic model.
 *   [x] It introduces no second HDL IR.
 *   [x] It introduces no quantum IR.
 *   [x] It preserves the quantum::ir boundary.
 *   [x] It performs no semantic evaluation.
 *   [x] It performs no hardware discovery.
 *   [x] It performs no target selection.
 *   [x] It performs no synthesis.
 *   [x] It performs no placement.
 *   [x] It performs no routing.
 *   [x] It performs no runtime execution.
 *   [x] It introduces no artificial hardware limits.
 *   [x] It introduces no fixed device limits.
 *   [x] It introduces no fixed CPU/GPU/FPGA/QPU limits.
 *   [x] It requires no unsafe Rust.
 *   [x] It is compatible with Rust 1.97 / 1.97.1 and Rust 2021.
 *   [x] It has an explicit AST boundary.
 *   [x] It has an explicit semantic boundary.
 *   [x] It has an explicit IR boundary.
 *   [x] It has an explicit compiler boundary.
 *   [x] It has an explicit runtime boundary.
 *   [x] It has an explicit cross-domain boundary.
 *   [x] It has an explicit testing contract.
 *   [x] It has an explicit hard-coding audit.
 *
 * Integration tests are required before marking the repository-level feature
 * as IMPLEMENTED.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file exists to make HDL interoperable without making HDL a second
 * language.
 *
 * Therefore:
 *
 *     ONE HDL SYNTAX AUTHORITY
 *             +
 *     ONE INTEROPERABILITY BOUNDARY
 *             +
 *     ONE DOMAIN-NEUTRAL AST
 *             +
 *     ONE CANONICAL SEMANTIC MODEL
 *             +
 *     ONE DOWNSTREAM HARDWARE IR
 *             +
 *     ONE quantum::ir FOR QUANTUM SEMANTICS
 *             +
 *     TARGET-INDEPENDENT RESOURCE/CAPABILITY INTENT
 *             =
 *     SCALABLE HDL INTEROPERABILITY
 *
 * The resulting architecture supports the larger Zamani objective:
 *
 *     Program_Once
 *     Compile_Once
 *     Run_Everywhere
 *     Run_Anywhere
 *     Run_Forever
 *
 * while preserving:
 *
 *     semantic stability
 *     hardware independence
 *     quantum/classical interoperability
 *     HDL interoperability
 *     deterministic parsing
 *     safe Rust
 *     extensibility
 *     compatibility
 *     scalability
 *
 * ============================================================================
 */