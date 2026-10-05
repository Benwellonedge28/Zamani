/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/effects/io.g4
 *
 * Grammar:
 *     IO
 *
 * Status:
 *     CANONICAL IO-DOMAIN INTEGRATION GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime execution.
 *     - No environment inspection.
 *     - No hardware discovery.
 *     - No target selection.
 *     - No resource discovery.
 *     - No randomness.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical parser-level integration boundary for the
 * input/output (IO) semantic domain.
 *
 * IO is treated as an EFFECT DOMAIN, not as a second language.
 *
 * This grammar therefore provides reusable IO-domain parser boundaries while
 * delegating generic effect syntax to the canonical effect subsystem.
 *
 *
 * OWNS
 * -----
 *
 * This file owns only IO-domain integration wrappers:
 *
 *     ioOperationReference
 *     ioOperationInvocation
 *     ioOperationUse
 *     ioEffectReference
 *     ioEffectReferenceList
 *     ioEffectSet
 *
 * These rules identify syntax that semantic analysis may subsequently classify
 * as belonging to the IO domain.
 *
 *
 * DOES NOT OWN
 * -------------
 *
 * This file does NOT own:
 *
 *     effect declarations
 *     effect operation declarations
 *     effect references in general
 *     effect sets in general
 *     effect invocation in general
 *     perform syntax
 *     effect handlers
 *     handler arms
 *     expressions
 *     argument lists
 *     identifiers
 *     qualified names
 *     types
 *     capabilities
 *     resources
 *     requirements
 *     constraints
 *     policies
 *     contracts
 *     provenance
 *     security
 *     filesystem semantics
 *     network semantics
 *     device semantics
 *     operating-system semantics
 *     file descriptors
 *     sockets
 *     pipes
 *     streams
 *     buffering
 *     encoding
 *     storage layout
 *     memory layout
 *     scheduling
 *     target selection
 *     backend selection
 *     hardware discovery
 *     CPU selection
 *     GPU selection
 *     FPGA selection
 *     ASIC selection
 *     accelerator selection
 *     QPU selection
 *     quantum operations
 *     quantum topology
 *     quantum::ir
 *     HDL representation
 *     runtime implementation
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The production pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     effect analysis              IO-domain analysis
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 semantic model
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      classical     quantum::ir   HDL/hardware
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                  canonical IR
 *                        |
 *                        v
 *             optimization/lowering
 *                        |
 *                        v
 *                routing/scheduling
 *                        |
 *                        v
 *                 resilience/recovery
 *                        |
 *                        v
 *                       HAL
 *                        |
 *                        v
 *                 target realization
 *
 *
 * IO syntax MUST remain above target realization.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Generic effect syntax is already owned by:
 *
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effects.g4
 *
 * Therefore this file MUST NOT redefine:
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
 *
 * Doing so would create competing grammar authorities.
 *
 *
 * Generic effect declarations are owned by:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * Generic handlers are owned by:
 *
 *     grammar/effects/effect-handling.g4
 *
 * Statement-level effect integration is owned by:
 *
 *     grammar/statements/effects.g4
 *
 * Expression-level effect integration is owned by:
 *
 *     grammar/expressions/effects.g4
 *
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This file imports only the canonical generic effect boundaries that it
 * actually reuses.
 *
 * Dependency direction:
 *
 *     IO
 *      |
 *      +--> Core
 *      +--> Type
 *      +--> Expressions
 *      +--> EffectOperations
 *      +--> EffectSets
 *
 * The generic effect aggregate MUST NOT import IO.
 *
 * Therefore the dependency remains acyclic:
 *
 *     Effects
 *        |
 *        +--> generic effect grammars
 *
 *     IO
 *        |
 *        +--> generic effect grammars
 *
 * There is deliberately no:
 *
 *     Effects -> IO -> Effects
 *
 * cycle.
 *
 * ============================================================================
 * OPEN-WORLD IO MODEL
 * ============================================================================
 *
 * IO operations are NOT enumerated.
 *
 * The grammar MUST NOT contain a closed list such as:
 *
 *     read
 *     write
 *     open
 *     close
 *     seek
 *     flush
 *     send
 *     receive
 *     stdin
 *     stdout
 *     stderr
 *     file
 *     socket
 *     pipe
 *     console
 *     serial
 *     device
 *
 * Those are semantic vocabulary, library vocabulary, dialect vocabulary,
 * capability vocabulary, or implementation vocabulary.
 *
 * They are not universal grammar primitives.
 *
 *
 * Examples of valid open-world operation names include:
 *
 *     io::read
 *     io::write
 *     io::stream::receive
 *     io::stream::send
 *     io::file::open
 *     io::file::close
 *     io::database::query
 *     io::device::transfer
 *     application::custom_io
 *     vendor::transport::send
 *     future::io::operation
 *
 * A new IO domain therefore does not require modifying this grammar.
 *
 * ============================================================================
 * DOMAIN CLASSIFICATION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `ioOperationReference` is a syntactic integration boundary.
 *
 * It does NOT prove that the referenced operation belongs to the IO domain.
 *
 * Semantic analysis MUST determine whether the resolved operation:
 *
 *     - is declared;
 *     - is an effect operation;
 *     - belongs to the IO effect domain;
 *     - is available;
 *     - is compatible with its arguments;
 *     - satisfies required capabilities;
 *     - satisfies resource requirements;
 *     - satisfies applicable policies;
 *     - is permitted by security rules.
 *
 * This distinction is essential for an open-world language.
 *
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * IO EFFECT
 * ---------
 *
 * Describes observable interaction with an external computational boundary.
 *
 *
 * CAPABILITY
 * ----------
 *
 * Describes what the execution environment can provide or authorize.
 *
 * Examples of semantic capabilities may include:
 *
 *     io.read
 *     io.write
 *     io.stream
 *     io.storage
 *     io.device
 *
 * Capability names remain open-world.
 *
 *
 * RESOURCE
 * --------
 *
 * Describes resources consumed, required, available, or negotiated by the
 * execution.
 *
 * Examples include semantic requirements for:
 *
 *     bandwidth
 *     storage
 *     memory
 *     latency
 *     throughput
 *     energy
 *     availability
 *
 * These are NOT represented as fixed constants in this grammar.
 *
 *
 * REQUIREMENT
 * -----------
 *
 * Describes what a realization must provide.
 *
 *
 * CONSTRAINT
 * ----------
 *
 * Describes what a valid realization must obey.
 *
 *
 * PREFERENCE
 * ----------
 *
 * Describes a preferred realization without changing program meaning.
 *
 *
 * POLICY
 * ------
 *
 * Governs whether an IO action is permitted and under what conditions.
 *
 *
 * The IO effect MUST NOT silently imply any particular resource, capability,
 * operating system, device, network interface, filesystem, or backend.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The source program expresses IO intent.
 *
 * It MUST NOT encode a particular machine realization.
 *
 * For example:
 *
 *     perform io::read(source)
 *
 * means that an IO read operation is requested.
 *
 * It does NOT mean:
 *
 *     use Linux;
 *     use POSIX;
 *     use Windows;
 *     use file descriptor 0;
 *     use device 0;
 *     use CPU 0;
 *     use GPU 0;
 *     use node 0;
 *     use a particular filesystem;
 *     use a particular network interface;
 *     use a particular memory bank;
 *     use a particular bus;
 *     use a particular hardware topology.
 *
 * Those decisions belong downstream.
 *
 * The same semantic operation may be realized through:
 *
 *     local storage
 *     distributed storage
 *     network transport
 *     memory-mapped IO
 *     DMA
 *     accelerator interfaces
 *     embedded peripherals
 *     cloud services
 *     simulation
 *     virtualized devices
 *     future execution environments
 *
 * without changing the source-level meaning.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO artificial machine ceilings.
 *
 * In particular, it MUST NOT define:
 *
 *     MAX_IO_OPERATIONS
 *     MAX_IO_DOMAINS
 *     MAX_IO_ARGUMENTS
 *     MAX_IO_EFFECTS
 *     MAX_IO_HANDLERS
 *     MAX_IO_STREAMS
 *     MAX_IO_DEVICES
 *     MAX_IO_BUFFERS
 *     MAX_IO_BYTES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *
 * or equivalent constants.
 *
 * There is no language-level fixed limit on:
 *
 *     number of IO operations;
 *     number of IO domains;
 *     number of effect references;
 *     number of effect-set entries;
 *     number of operation arguments;
 *     qualified-name depth;
 *     program size;
 *     number of devices;
 *     number of nodes;
 *     amount of data;
 *     number of streams;
 *     number of concurrent operations.
 *
 * Practical limits are implementation/resource constraints and are not encoded
 * as language semantics.
 *
 * "Infinity" therefore means:
 *
 *     no artificial language-level ceiling.
 *
 * It does not claim that physical hardware or compiler resources are infinite.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar is deterministic with respect to the parser's specified input
 * and grammar version.
 *
 * It performs no:
 *
 *     IO;
 *     filesystem access;
 *     network access;
 *     hardware inspection;
 *     environment inspection;
 *     runtime execution;
 *     random selection;
 *     target selection.
 *
 * The parser therefore cannot accidentally change program meaning according
 * to the machine on which parsing occurs.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing an IO construct MUST NOT execute the IO operation.
 *
 * In particular, parsing:
 *
 *     perform io::read(source)
 *
 * MUST NOT:
 *
 *     open `source`;
 *     read a file;
 *     contact a network;
 *     access a device;
 *     invoke a process;
 *     access credentials;
 *     inspect the host;
 *     perform native calls.
 *
 * Authorization and execution occur only after semantic validation and
 * downstream execution planning.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The domain-neutral frontend AST MUST preserve enough information to support:
 *
 *     source span;
 *     qualified operation identity;
 *     operation arguments;
 *     effect classification;
 *     effect-set membership;
 *     source ordering;
 *     source-level metadata where applicable.
 *
 * The AST MUST NOT require:
 *
 *     file descriptors;
 *     OS handles;
 *     sockets;
 *     physical addresses;
 *     device IDs;
 *     process IDs;
 *     thread IDs;
 *     CPU IDs;
 *     GPU IDs;
 *     FPGA IDs;
 *     QPU IDs;
 *     backend IDs;
 *     machine IDs.
 *
 * Those are realization details.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis is responsible for:
 *
 *     1. name resolution;
 *     2. effect-domain classification;
 *     3. operation resolution;
 *     4. declaration validation;
 *     5. argument/type checking;
 *     6. effect checking;
 *     7. capability checking;
 *     8. resource analysis;
 *     9. requirement checking;
 *    10. constraint checking;
 *    11. policy checking;
 *    12. security/authorization analysis;
 *    13. determinism analysis where applicable;
 *    14. provenance construction;
 *    15. target-independent validation;
 *    16. lowering preparation.
 *
 * The grammar MUST NOT perform these semantic decisions.
 *
 * ============================================================================
 * EFFECT INFERENCE
 * ============================================================================
 *
 * An IO operation may participate in inferred effects.
 *
 * For example:
 *
 *     io::read
 *
 * may semantically contribute an IO effect even when the surrounding source
 * construct does not explicitly spell an effect set.
 *
 * The exact inference rules belong to semantic effect analysis.
 *
 * This file only supplies the syntactic IO-domain boundary.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * IO failure is NOT represented by hard-coded grammar alternatives.
 *
 * A particular IO operation may semantically return:
 *
 *     Result<T, E>
 *
 * or another declared error representation.
 *
 * The grammar does not impose one universal failure model.
 *
 * Effect handlers remain owned by:
 *
 *     grammar/effects/effect-handling.g4
 *
 * and generic error/result syntax remains owned by the type/expression
 * subsystem.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * IO operations may require capabilities.
 *
 * Example semantic intent:
 *
 *     requires capability("io.read");
 *
 * or:
 *
 *     requires capability("io.write");
 *
 * Capability syntax is NOT redefined here.
 *
 * The canonical capability/requirement owners remain:
 *
 *     grammar/core/capabilities.g4
 *     grammar/core/requirements.g4
 *     grammar/resources/
 *
 * Semantic analysis connects an IO operation to its declared capability
 * requirements.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * IO may have resource implications such as:
 *
 *     storage;
 *     bandwidth;
 *     memory;
 *     latency;
 *     throughput;
 *     energy;
 *     availability.
 *
 * The IO grammar MUST NOT encode fixed resource values.
 *
 * For example, this file must never prescribe:
 *
 *     buffer = 4096;
 *     bandwidth = 1Gbps;
 *     storage = 64GB;
 *
 * as universal language requirements.
 *
 * Symbolic or semantic resource requirements belong to the resource system.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * IO may be controlled by policies including:
 *
 *     permission;
 *     prohibition;
 *     sandboxing;
 *     trust;
 *     authorization;
 *     data-handling rules;
 *     network restrictions;
 *     provenance requirements.
 *
 * Policy syntax is not duplicated here.
 *
 * The policy subsystem remains authoritative.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * IO operations may participate in provenance.
 *
 * Semantic provenance may record:
 *
 *     operation identity;
 *     source location;
 *     data lineage;
 *     producing transformation;
 *     consuming transformation;
 *     policy decision;
 *     capability decision;
 *     execution realization.
 *
 * This grammar does not create provenance records.
 *
 * It merely preserves the syntactic operation identity required downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * IO may participate in hybrid quantum/classical programs.
 *
 * Example:
 *
 *     perform io::read(input);
 *     perform quantum::measure(state);
 *     perform io::write(result);
 *
 * The IO grammar MUST remain independent of quantum semantics.
 *
 * It MUST NOT define:
 *
 *     qubits;
 *     gates;
 *     physical qubits;
 *     coupling maps;
 *     calibration;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN;
 *     quantum::ir.
 *
 * If an IO operation participates in a hybrid computation, semantic lowering
 * may connect its result to quantum semantic operations, but the canonical
 * quantum boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * IO may represent hardware/software interaction through semantic operation
 * names.
 *
 * Example:
 *
 *     perform io::device::read(channel);
 *
 * or:
 *
 *     perform hardware::interface::transfer(data);
 *
 * This grammar does not decide whether the realization becomes:
 *
 *     CPU code;
 *     DMA;
 *     FPGA logic;
 *     ASIC logic;
 *     accelerator command;
 *     bus transaction;
 *     memory operation;
 *     peripheral access;
 *     network transfer.
 *
 * Those decisions belong downstream semantic and target systems.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * IO may be local or distributed.
 *
 * The grammar does not distinguish these by hardware assumptions.
 *
 * For example:
 *
 *     io::read(source)
 *
 * could be realized locally or through a distributed service if the semantic
 * contract permits it.
 *
 * Distributed execution must be selected through:
 *
 *     capabilities;
 *     resources;
 *     requirements;
 *     policies;
 *     target negotiation;
 *
 * rather than through a hard-coded source-level machine identity.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * IO operations may be simulated.
 *
 * Simulation is an execution strategy, not a different source language.
 *
 * The semantic/execution layer may map:
 *
 *     IO operation
 *
 * to:
 *
 *     real IO;
 *     virtual IO;
 *     deterministic simulation;
 *     test harness;
 *     replay;
 *     emulation.
 *
 * This grammar does not choose among them.
 *
 * ============================================================================
 * REPRODUCIBILITY
 * ============================================================================
 *
 * IO is inherently capable of introducing external state.
 *
 * Therefore reproducible execution is a semantic/runtime concern.
 *
 * The grammar does not falsely classify every IO operation as deterministic.
 *
 * Instead, semantic analysis may classify operations according to declared
 * properties such as:
 *
 *     deterministic;
 *     nondeterministic;
 *     replayable;
 *     observable;
 *     externally dependent.
 *
 * These properties must not be encoded as a finite grammar enumeration.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source code using generic effect operations remains valid through
 * the generic effect subsystem.
 *
 * This file is an integration boundary and therefore must not require existing
 * generic effect syntax to be rewritten.
 *
 * In particular:
 *
 *     perform io::read(source)
 *
 * remains structurally a normal effect operation.
 *
 * The semantic layer may additionally classify it as an IO operation.
 *
 * No historical operation spelling is reserved by this grammar.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Public rules exported by this grammar:
 *
 *     ioOperationReference
 *     ioOperationInvocation
 *     ioOperationUse
 *     ioEffectReference
 *     ioEffectReferenceList
 *     ioEffectSet
 *
 * These names are intentionally prefixed with `io` so they cannot accidentally
 * replace the canonical generic effect rules.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/core/core.g4
 *     grammar/types/types.g4
 *     grammar/expressions/expressions.g4
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *
 * LEXER:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * LEXICAL COMPOSITION:
 *
 *     grammar/lexer/tokens.g4
 *
 * AST OWNER:
 *
 *     domain-neutral frontend AST under src/ast/
 *
 * SEMANTIC OWNER:
 *
 *     effect/IO semantic analysis
 *
 * EFFECT OWNER:
 *
 *     grammar/effects/effects.g4
 *
 * RESOURCE OWNER:
 *
 *     grammar/resources/
 *
 * CAPABILITY OWNER:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *
 * POLICY OWNER:
 *
 *     grammar/core/policies.g4
 *     grammar/policies/ when present
 *
 * SECURITY OWNER:
 *
 *     grammar/security/
 *
 * PROVENANCE OWNER:
 *
 *     grammar/provenance/ or semantic provenance subsystem
 *
 * IR OWNER:
 *
 *     canonical semantic representation
 *
 * QUANTUM IR OWNER:
 *
 *     quantum::ir
 *
 * TEST OWNER:
 *
 *     grammar/tests/
 *     grammar/tests/effects/
 *     grammar/tests/effects/io/
 *
 * SPECIFICATION OWNER:
 *
 *     grammar/spec/effects.md
 *     grammar/specification/
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. The canonical lexer emits normal identifier/name/operator tokens.
 *
 * 2. Core provides:
 *
 *        identifier
 *        qualifiedName
 *        punctuation
 *        common source constructs
 *
 * 3. Type provides:
 *
 *        typeExpression
 *
 *    when IO declarations or semantic extensions need type syntax.
 *
 * 4. Expressions provides:
 *
 *        expression
 *        argumentList
 *
 *    through the canonical expression composition.
 *
 * 5. EffectOperations provides:
 *
 *        effectOperationReference
 *        effectInvocation
 *        effectOperationUse
 *
 * 6. EffectSets provides:
 *
 *        effectReference
 *        effectReferenceList
 *        effectSet
 *
 * 7. This file provides IO-prefixed wrappers only.
 *
 * 8. `grammar/effects/effects.g4` remains the generic effect composition root.
 *
 * 9. `grammar/statements/effects.g4` remains the universal statement-level
 *    effect adapter.
 *
 * 10. `grammar/expressions/effects.g4` remains the expression-level effect
 *     adapter.
 *
 * 11. Semantic analysis classifies resolved operation/effect identities as IO
 *     where appropriate.
 *
 * 12. Capability/resource/policy analysis occurs after parsing.
 *
 * 13. Target realization occurs after semantic analysis.
 *
 * ============================================================================
 * ANTLR IMPORT GRAPH
 * ============================================================================
 *
 * The intended dependency graph is:
 *
 *     IO
 *      |
 *      +--> Core
 *      |
 *      +--> Type
 *      |
 *      +--> Expressions
 *      |
 *      +--> EffectOperations
 *      |       |
 *      |       +--> Core
 *      |       +--> Expressions
 *      |
 *      +--> EffectSets
 *              |
 *              +--> Core
 *
 * No imported grammar may import IO.
 *
 * This keeps IO a leaf domain integration grammar.
 *
 * ============================================================================
 * RULE DESIGN
 * ============================================================================
 *
 * The rules below are deliberately thin.
 *
 * A thin domain adapter is preferable to duplicating generic effect syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ANTLR GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar IO;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Type,
    Expressions,
    EffectOperations,
    EffectSets
;


/*
 * ============================================================================
 * IO OPERATION REFERENCE
 * ============================================================================
 *
 * An IO operation is represented by the canonical generic effect-operation
 * reference.
 *
 * Examples:
 *
 *     io::read
 *     io::write
 *     io::stream::receive
 *     io::file::open
 *     io::device::transfer
 *
 * No operation names are enumerated.
 *
 * Semantic analysis determines whether the resolved operation belongs to the
 * IO domain.
 */

ioOperationReference
    : effectOperationReference
    ;


/*
 * ============================================================================
 * IO OPERATION INVOCATION
 * ============================================================================
 *
 * Reuses the canonical effect invocation syntax.
 *
 * Examples:
 *
 *     io::read(source)
 *     io::write(value)
 *     io::stream::receive(channel)
 *
 * Argument syntax remains owned by the expression subsystem.
 */

ioOperationInvocation
    : effectInvocation
    ;


/*
 * ============================================================================
 * IO OPERATION USE
 * ============================================================================
 *
 * Stable IO-domain wrapper around the canonical effect operation use.
 *
 * This rule exists for:
 *
 *     - domain-specific semantic tooling;
 *     - parser listeners;
 *     - AST conversion boundaries;
 *     - conformance tooling;
 *     - future IO-domain extensions.
 *
 * It does not create a second effect-operation language.
 */

ioOperationUse
    : effectOperationUse
    ;


/*
 * ============================================================================
 * IO EFFECT REFERENCE
 * ============================================================================
 *
 * Reuses the canonical effect reference syntax.
 *
 * Examples:
 *
 *     io
 *     io::storage
 *     io::stream
 *     io::device
 *
 * Whether the referenced effect is actually declared as an IO effect is a
 * semantic question.
 */

ioEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * IO EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Reuses the canonical unbounded effect-reference list.
 *
 * Examples:
 *
 *     io
 *
 *     io, data
 *
 *     io::read, io::write, data::transform
 *
 * No fixed number of effects is imposed.
 */

ioEffectReferenceList
    : effectReferenceList
    ;


/*
 * ============================================================================
 * IO EFFECT SET
 * ============================================================================
 *
 * Reuses the canonical effect-set representation.
 *
 * Example:
 *
 *     {
 *         io::read,
 *         io::write
 *     }
 *
 * This grammar does not assign special semantics to the entries.
 *
 * Semantic analysis performs IO-domain classification.
 */

ioEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 * [x] It is a parser grammar.
 *
 * [x] It uses ZamaniLexer as its token vocabulary.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not duplicate generic effect-operation syntax.
 *
 * [x] It does not duplicate generic effect-set syntax.
 *
 * [x] It does not duplicate generic handler syntax.
 *
 * [x] It does not define a closed IO operation catalogue.
 *
 * [x] It does not reserve application-specific IO words.
 *
 * [x] It does not select hardware.
 *
 * [x] It does not select an operating system.
 *
 * [x] It does not impose buffer-size limits.
 *
 * [x] It does not impose device-count limits.
 *
 * [x] It does not impose machine-size limits.
 *
 * [x] It contains no MAX_* machine-capacity constants.
 *
 * [x] It contains no Rust actions.
 *
 * [x] It contains no unsafe implementation requirement.
 *
 * [x] It preserves the canonical effect-operation boundary.
 *
 * [x] It preserves the canonical effect-set boundary.
 *
 * [x] It remains open-world.
 *
 * [x] It can be integrated without modifying generic effect semantics.
 *
 * [x] It can participate in classical, quantum, HDL, distributed, AI,
 *     accelerator, embedded, and future-domain programs without changing the
 *     universal IO grammar.
 *
 * ============================================================================
 * REQUIRED CONFORMANCE TESTS
 * ============================================================================
 *
 * Positive parser cases:
 *
 *     io::read
 *     io::write
 *     io::file::open
 *     io::stream::receive
 *     io::device::transfer
 *     vendor::io::operation
 *     future::io::operation
 *
 * Positive invocation cases:
 *
 *     io::read(source)
 *     io::write(value)
 *     io::stream::receive(channel)
 *     io::device::transfer(data)
 *
 * Positive effect references:
 *
 *     io
 *     io::storage
 *     io::stream
 *     io::device
 *
 * Positive effect sets:
 *
 *     {
 *         io::read,
 *         io::write
 *     }
 *
 *     {
 *         io::read,
 *         data::transform,
 *         networking::request
 *     }
 *
 * Cross-domain cases:
 *
 *     perform io::read(input);
 *     perform quantum::measure(state);
 *     perform io::write(result);
 *
 *     perform io::read(data);
 *     perform accelerator::compute(data);
 *     perform io::write(result);
 *
 *     perform io::read(input);
 *     perform distributed::send(input);
 *
 * Negative semantic cases:
 *
 *     unknown IO operation
 *     unresolved IO effect
 *     missing IO capability
 *     prohibited IO policy
 *     incompatible IO argument type
 *     unavailable IO realization
 *
 * These semantic failures MUST NOT be encoded as parser keyword enumerations.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     deeply qualified IO names;
 *     large effect sets;
 *     large argument lists;
 *     large source programs;
 *     many independent IO operations;
 *     many domains sharing the same effect machinery;
 *     nested effect usage;
 *     concurrent IO operations;
 *     distributed IO operations;
 *     hybrid classical/quantum IO;
 *     hardware-facing IO;
 *     simulated IO.
 *
 * The tests MUST use generated/scaled inputs rather than introducing grammar
 * constants.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
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
 * Also forbidden:
 *
 *     fixed IO operation catalogues;
 *     fixed device catalogues;
 *     fixed filesystem catalogues;
 *     fixed operating-system catalogues;
 *     fixed network-interface catalogues;
 *     physical device IDs;
 *     backend IDs;
 *     machine IDs.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This grammar describes IO as an OPEN semantic domain over Zamani's universal
 * effect system.
 *
 * Therefore:
 *
 *     source program
 *          |
 *          v
 *     generic effect syntax
 *          |
 *          v
 *     IO semantic classification
 *          |
 *          v
 *     capability/resource/policy analysis
 *          |
 *          v
 *     target-independent semantic model
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir        HDL/hardware
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                              v
 *                     lowering/scheduling
 *                              |
 *                              v
 *                         HAL/realization
 *
 * No IO construct in this file requires rewriting when a new:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     embedded platform
 *     cluster
 *     cloud platform
 *     network fabric
 *     storage system
 *     future computational substrate
 *
 * is introduced.
 *
 * That is the required POCO-REAF property of the IO grammar.
 *
 * ============================================================================
 */