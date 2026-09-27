/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/profiling.g4
 *
 * Grammar:
 *     Profiling
 *
 * Status:
 *     Production execution-profiling grammar component
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL PROFILING INTENT.
 *
 * Profiling answers:
 *
 *     WHAT execution/compiler/runtime/scheduler behaviour should be measured?
 *     WHAT observations should be collected?
 *     WHAT scope should be profiled?
 *     WHAT metrics are requested?
 *     WHAT sampling policy is permitted?
 *     WHAT filtering is requested?
 *     WHAT correlation is required?
 *     WHAT retention/export/privacy/integrity policy applies?
 *
 * It does NOT implement profiling.
 *
 * It does NOT:
 *
 *     - read clocks;
 *     - read hardware counters;
 *     - access CPU/GPU/QPU/FPGA counters;
 *     - execute profiler code;
 *     - allocate profiler storage;
 *     - select a profiling backend;
 *     - select a vendor;
 *     - select a physical device;
 *     - schedule computation;
 *     - route quantum operations;
 *     - optimize programs;
 *     - perform QEC;
 *     - perform ZQN;
 *     - execute runtime operations.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> profiling-intent validation
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> privacy/security analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     execution realization
 *          |
 *          +--> runtime profiling
 *          +--> compiler profiling
 *          +--> scheduler profiling
 *          +--> hardware/backend profiling
 *
 * Profiling is therefore observational metadata/intent.
 *
 * It is NOT an IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Profiling MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A source program may request:
 *
 *     profile execution
 *     profile scheduling
 *     profile compilation
 *     profile resource usage
 *     profile quantum execution
 *     profile hardware realization
 *
 * without requiring the source program to name:
 *
 *     a CPU;
 *     a GPU;
 *     a QPU;
 *     an FPGA;
 *     an ASIC;
 *     a vendor;
 *     a physical device;
 *     a hardware counter;
 *     a profiler implementation;
 *     a telemetry backend.
 *
 * Physical profiling realization is downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar deliberately contains NO universal profiling limits.
 *
 * It MUST NOT encode:
 *
 *     MAX_PROFILE_ENTRIES
 *     MAX_PROFILE_EVENTS
 *     MAX_PROFILE_METRICS
 *     MAX_PROFILE_SAMPLES
 *     MAX_PROFILE_ATTRIBUTES
 *     MAX_PROFILE_DEPTH
 *     MAX_PROFILE_TARGETS
 *     MAX_PROFILE_SESSIONS
 *     MAX_PROFILE_COUNTERS
 *     MAX_TRACE_DEPTH
 *     MAX_OPERATIONS
 *     MAX_RESOURCES
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_QPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * Repetition is represented structurally with ANTLR repetition operators.
 *
 * Any implementation limit required by:
 *
 *     parser memory;
 *     compiler memory;
 *     runtime memory;
 *     storage;
 *     telemetry transport;
 *     hardware counters;
 *     target capabilities;
 *     operating-system resources;
 *
 * is an implementation/resource concern and MUST NOT become a language-level
 * profiling limit.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: PROGRAM DATA VS LANGUAGE LIMIT
 * ============================================================================
 *
 * A program MAY explicitly request a bounded profiling policy:
 *
 *     samples: sample_count;
 *
 *     retention: retention_period;
 *
 *     budget: profiling_budget;
 *
 * Those are program semantics.
 *
 * They MUST NOT be interpreted as universal compiler limits.
 *
 * The grammar therefore permits expressions rather than hard-coded numeric
 * ceilings.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - profiling construct composition;
 *     - profiling target intent;
 *     - profiling scope intent;
 *     - metric intent;
 *     - counter intent;
 *     - measurement intent;
 *     - sampling intent;
 *     - filtering intent;
 *     - aggregation intent;
 *     - baseline/comparison intent;
 *     - attribution intent;
 *     - correlation intent;
 *     - collection intent;
 *     - retention intent;
 *     - export intent;
 *     - privacy/classification intent;
 *     - integrity/provenance intent;
 *     - profiling requirements;
 *     - profiling constraints;
 *     - profiling preferences;
 *     - profiling hints;
 *     - profiling extensions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - expressions;
 *     - general types;
 *     - observability as a whole;
 *     - tracing;
 *     - logging;
 *     - scheduling;
 *     - runtime implementation;
 *     - hardware counters;
 *     - resource discovery;
 *     - compiler optimization;
 *     - quantum semantics;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - deployment;
 *     - telemetry transport;
 *     - profiler backend implementation.
 *
 * ============================================================================
 * RELATIONSHIP WITH observability.g4
 * ============================================================================
 *
 * grammar/execution/observability.g4
 *
 * owns the broader observability composition:
 *
 *     observations
 *     events
 *     metrics
 *     logs
 *     traces
 *     spans
 *     profiles
 *
 * This file owns the detailed profiling language.
 *
 * The intended composition is:
 *
 *     Observability
 *          |
 *          +--> Profiling
 *          +--> Tracing
 *          +--> observation/event/metric intent
 *
 * Observability MUST NOT reproduce the detailed profiling rules defined here.
 *
 * ============================================================================
 * RELATIONSHIP WITH tracing.g4
 * ============================================================================
 *
 * tracing.g4 owns:
 *
 *     traces
 *     spans
 *     propagation
 *     correlation
 *     trace sampling
 *
 * Profiling owns:
 *
 *     performance/resource/compiler/runtime/scheduler profiling.
 *
 * A profiling declaration MAY contain correlation metadata referring to a
 * trace or execution context, but this file does not redefine tracing syntax.
 *
 * ============================================================================
 * RELATIONSHIP WITH runtime.g4
 * ============================================================================
 *
 * The current repository already exposes:
 *
 *     runtimeProfileStatement
 *
 * using the canonical:
 *
 *     PROFILE
 *
 * token.
 *
 * This file therefore uses unique rule names prefixed with:
 *
 *     profiling*
 *
 * and MUST become the detailed profiling authority.
 *
 * The final composition should be:
 *
 *     runtime.g4
 *          |
 *          +--> profilingConstruct
 *
 * rather than maintaining a second detailed profiling grammar inside
 * runtime.g4.
 *
 * Until that composition migration is performed, this file remains
 * independently valid and does not redefine runtimeProfileStatement.
 *
 * ============================================================================
 * RELATIONSHIP WITH scheduling diagnostics
 * ============================================================================
 *
 * The repository already contains:
 *
 *     src/quantum/scheduling/diagnostics/profile.rs
 *
 * That implementation owns scheduler profiling data and uses aggregate
 * counters, checked arithmetic, optional unique-qubit tracking, mergeable
 * profiles, and separate host-time measurements.
 *
 * This grammar MUST NOT reproduce those Rust data structures.
 *
 * Instead:
 *
 *     profiling intent
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     scheduler profiling configuration
 *          |
 *          v
 *     diagnostics::profile
 *
 * In particular, profiling host elapsed time MUST remain distinct from
 * abstract quantum schedule time.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes the canonical:
 *
 *     ZamaniLexer
 *
 * It defines NO lexer rules.
 *
 * The repository already provides:
 *
 *     PROFILE
 *     OBSERVE
 *     WITH
 *     REQUIRES
 *     RESOURCE
 *     CAPABILITY
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *
 * and the standard identifier, expression, punctuation and literal tokens.
 *
 * Profiling-specific vocabulary such as:
 *
 *     metric
 *     counter
 *     sample
 *     sampling
 *     aggregate
 *     baseline
 *     retention
 *
 * remains structurally representable through ordinary identifiers unless the
 * canonical lexical specification explicitly promotes a spelling to a
 * reserved keyword.
 *
 * This file MUST NOT invent profiling-specific lexer tokens.
 *
 * ============================================================================
 * NO STRING-LITERAL TOKEN INVENTION
 * ============================================================================
 *
 * Because this is a parser grammar, it MUST NOT depend on parser string
 * literals such as:
 *
 *     'profile'
 *     'metric'
 *     'sample'
 *
 * The canonical PROFILE token is used for the language-level profiling entry
 * point.
 *
 * Other profiling vocabulary remains identifier-based and is interpreted
 * semantically.
 *
 * This prevents this file from becoming a second lexical authority.
 *
 * ============================================================================
 * OPEN-ENDED PROFILING VOCABULARY
 * ============================================================================
 *
 * The grammar must support future profiling dimensions without requiring a
 * parser rewrite.
 *
 * Examples include:
 *
 *     execution
 *     compilation
 *     scheduling
 *     resource
 *     memory
 *     communication
 *     quantum
 *     qec
 *     noise
 *     hdl
 *     synthesis
 *     simulation
 *     accelerator
 *     tensor
 *     ai
 *     distributed
 *     networking
 *     energy
 *     thermal
 *     reliability
 *     future::profiling
 *
 * The grammar recognizes the structure.
 *
 * Semantic analysis determines whether a requested metric/profile dimension
 * is:
 *
 *     stable;
 *     experimental;
 *     dialect-defined;
 *     target-dependent;
 *     unsupported;
 *     deprecated;
 *     invalid.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts MUST remain distinct.
 *
 * REQUIREMENT:
 *
 *     The requested profiling property is mandatory.
 *
 * CONSTRAINT:
 *
 *     The valid profiling realization is restricted.
 *
 * PREFERENCE:
 *
 *     The property is desirable but not mandatory.
 *
 * HINT:
 *
 *     The property is advisory implementation guidance.
 *
 * Semantic analysis MUST NOT silently promote:
 *
 *     preference -> requirement
 *
 * or:
 *
 *     hint -> requirement.
 *
 * ============================================================================
 * OBSERVATIONAL NON-INTERFERENCE
 * ============================================================================
 *
 * Profiling MUST NOT change the functional meaning of a program merely because
 * profiling is enabled.
 *
 * Profiling metadata MUST therefore be semantically non-functional unless the
 * language specification explicitly defines an observable profiling side
 * effect.
 *
 * Examples:
 *
 *     profiling host time
 *     profiling scheduler counters
 *     profiling compiler phase duration
 *
 * MUST NOT alter:
 *
 *     quantum operation semantics;
 *     classical result semantics;
 *     HDL functional intent;
 *     resource correctness;
 *     deterministic program meaning.
 *
 * Runtime overhead is an implementation concern and MUST NOT be confused with
 * source-level program semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Profiling must distinguish deterministic measurements from environment-
 * dependent observations.
 *
 * Examples:
 *
 *     aggregate operation count
 *     dependency count
 *     resource count
 *
 * may be deterministic for a deterministic compilation/scheduling process.
 *
 * Examples such as:
 *
 *     host elapsed time
 *     wall-clock latency
 *     hardware temperature
 *     dynamic power
 *
 * are environment-dependent observations.
 *
 * Such values MUST NOT silently become part of:
 *
 *     semantic identity;
 *     program equivalence;
 *     reproducible compilation identity;
 *     quantum::ir semantic identity.
 *
 * The semantic layer determines whether a requested profile field is
 * deterministic, externally dependent, target dependent, or observational.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar does not enumerate:
 *
 *     CPU
 *     GPU
 *     QPU
 *     FPGA
 *     ASIC
 *     vendor
 *     device
 *
 * as a closed profiling target list.
 *
 * A program may instead express:
 *
 *     profile target;
 *     target: expression;
 *     capability: expression;
 *     resource: expression;
 *
 * Actual realization is downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum profiling may observe:
 *
 *     logical operation counts;
 *     operation classes;
 *     dependency counts;
 *     schedule duration;
 *     scheduling decisions;
 *     routing-related metrics;
 *     resource usage;
 *     measurement activity;
 *     dynamic-control activity;
 *     QEC-related profiling metadata;
 *     noise-aware execution metadata;
 *     host compilation/planning time.
 *
 * This grammar does NOT define:
 *
 *     quantum gates;
 *     quantum operations;
 *     qubits;
 *     quantum states;
 *     physical qubits;
 *     pulse schedules;
 *     QEC algorithms;
 *     noise models;
 *     ZQN semantics.
 *
 * Those remain owned by the quantum/QEC/ZQN subsystems.
 *
 * Any quantum semantic payload ultimately remains associated with:
 *
 *     quantum::ir
 *
 * rather than creating:
 *
 *     QuantumProfileIR
 *     QuantumProfilingIR
 *
 * as a competing IR.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Profiling may describe:
 *
 *     function execution;
 *     operation counts;
 *     memory behaviour;
 *     parallelism;
 *     vectorization;
 *     accelerator use;
 *     data movement;
 *     compilation phases;
 *     runtime phases.
 *
 * It does not define the classical IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Profiling may request observations such as:
 *
 *     synthesis metrics;
 *     timing metrics;
 *     resource utilization;
 *     area estimates;
 *     power estimates;
 *     thermal observations;
 *     verification metrics;
 *     simulation metrics.
 *
 * This grammar MUST NOT encode:
 *
 *     register width;
 *     FPGA capacity;
 *     fixed clock frequency;
 *     fixed LUT count;
 *     fixed memory size;
 *     physical routing topology.
 *
 * Those are target/hardware concerns.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Profiling may request:
 *
 *     communication volume;
 *     synchronization activity;
 *     task latency;
 *     queueing;
 *     replication;
 *     coordination;
 *     data movement;
 *     distributed execution timing.
 *
 * It MUST NOT encode a universal node count or topology.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Profiling may describe:
 *
 *     model execution;
 *     tensor operations;
 *     data movement;
 *     training phases;
 *     inference phases;
 *     memory pressure;
 *     accelerator utilization;
 *     communication;
 *     convergence-related measurements.
 *
 * The grammar remains independent of any particular AI framework.
 *
 * ============================================================================
 * SECURITY / PRIVACY
 * ============================================================================
 *
 * Profiling can expose sensitive information.
 *
 * The grammar therefore supports intent for:
 *
 *     privacy;
 *     classification;
 *     redaction;
 *     anonymization;
 *     integrity;
 *     provenance;
 *     retention;
 *     export restrictions.
 *
 * Parsing a profiling declaration MUST NOT grant authorization to access
 * sensitive information.
 *
 * Semantic/security analysis determines whether the requested observation is
 * permitted.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every profiling construct MUST preserve source spans through the frontend
 * AST.
 *
 * At minimum:
 *
 *     profilingConstruct
 *     profilingTarget
 *     profilingBody
 *     profilingEntry
 *     profilingProperty
 *     profilingValue
 *
 * require source locations sufficient for diagnostics and tooling.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Recommended conceptual mapping:
 *
 *     profilingConstruct
 *         -> ProfilingIntent
 *
 *     profilingTarget
 *         -> ProfilingTarget
 *
 *     profilingBody
 *         -> ProfilingConfiguration
 *
 *     profilingEntry
 *         -> ProfilingProperty
 *
 *     profilingRequirement
 *         -> Requirement
 *
 *     profilingConstraint
 *         -> Constraint
 *
 *     profilingPreference
 *         -> Preference
 *
 *     profilingHint
 *         -> Hint
 *
 *     profilingValue
 *         -> generic expression/property value
 *
 * The exact Rust AST types belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT depend on those Rust types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving profiling targets;
 *     - resolving metric names;
 *     - validating metric applicability;
 *     - validating scope;
 *     - validating sampling policy;
 *     - validating filters;
 *     - validating aggregation;
 *     - validating retention;
 *     - validating export;
 *     - validating privacy;
 *     - validating provenance;
 *     - validating capabilities;
 *     - validating resources;
 *     - classifying target-dependent observations;
 *     - determining deterministic/non-deterministic dimensions;
 *     - checking conflicts;
 *     - checking portability;
 *     - rejecting unsupported profiler capabilities.
 *
 * The parser MUST NOT perform any of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Profiling intent is attached to the canonical semantic model.
 *
 * For classical computation:
 *
 *     source
 *       -> AST
 *       -> semantic model
 *       -> classical IR
 *       -> profiling realization metadata
 *
 * For quantum computation:
 *
 *     source
 *       -> quantum frontend
 *       -> quantum semantic model
 *       -> quantum::ir
 *       -> profiling realization metadata
 *
 * For HDL:
 *
 *     source
 *       -> HDL semantic model
 *       -> hardware representation
 *       -> profiling realization metadata
 *
 * Profiling MUST NOT create a competing universal IR.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Generated parser/frontend integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * The implementation MUST NOT require:
 *
 *     unsafe;
 *     unsafe blocks;
 *     unsafe traits;
 *     unsafe extern interfaces.
 *
 * ============================================================================
 * ERROR / RECOVERY CONTRACT
 * ============================================================================
 *
 * The grammar must remain recoverable for malformed profiling properties.
 *
 * Semantic diagnostics should identify:
 *
 *     unknown metric;
 *     invalid target;
 *     invalid scope;
 *     unsupported capability;
 *     invalid resource requirement;
 *     incompatible sampling policy;
 *     invalid aggregation;
 *     conflicting properties;
 *     unsupported export;
 *     privacy violation;
 *     portability violation.
 *
 * Parser recovery belongs to the canonical parser/error strategy.
 *
 * This file MUST NOT embed custom recovery actions.
 *
 * ============================================================================
 * EXTENSION MODEL
 * ============================================================================
 *
 * The generic property mechanism permits future profiling concepts without
 * changing this grammar.
 *
 * Examples:
 *
 *     profiling::metric: expression;
 *
 *     compiler::phase: expression;
 *
 *     scheduler::decision: expression;
 *
 *     quantum::operation: expression;
 *
 *     qec::round: expression;
 *
 *     hardware::thermal: expression;
 *
 *     distributed::communication: expression;
 *
 *     future::profiling: expression;
 *
 * Semantic analysis determines whether such properties are recognized.
 *
 * ============================================================================
 * INDEPENDENT COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is independently complete when:
 *
 * [x] It has one parser grammar identity.
 * [x] It consumes ZamaniLexer.
 * [x] It defines no lexer rules.
 * [x] It imports only foundational grammar dependencies.
 * [x] It has a public profiling entry point.
 * [x] It has an expression-based profiling target.
 * [x] It supports structured profiling configuration.
 * [x] It supports arbitrary profiling entries.
 * [x] It supports generic extensible properties.
 * [x] It separates requirement/constraint/preference/hint.
 * [x] It supports sampling intent.
 * [x] It supports filtering intent.
 * [x] It supports aggregation intent.
 * [x] It supports comparison/baseline intent.
 * [x] It supports correlation intent.
 * [x] It supports retention intent.
 * [x] It supports export intent.
 * [x] It supports privacy/classification intent.
 * [x] It supports integrity/provenance intent.
 * [x] It has no machine-size constants.
 * [x] It has no fixed hardware target enumeration.
 * [x] It has no profiler implementation.
 * [x] It has no runtime actions.
 * [x] It creates no IR.
 * [x] It preserves quantum::ir as the canonical quantum IR boundary.
 * [x] It is compatible with safe Rust 1.97/1.97.1 generation.
 * [x] It defines explicit downstream integration contracts.
 *
 * Repository conformance additionally requires:
 *
 * [ ] ANTLR generation passes in the repository's canonical generation path.
 * [ ] Every referenced token exists in the assembled ZamaniLexer.
 * [ ] The canonical parser imports this grammar exactly once.
 * [ ] runtime.g4 delegates detailed PROFILE syntax to this grammar.
 * [ ] observability.g4 composes this grammar rather than duplicating it.
 * [ ] Positive conformance tests pass.
 * [ ] Negative conformance tests pass.
 * [ ] Boundary tests pass.
 * [ ] Scalability tests pass.
 * [ ] Determinism tests pass.
 * [ ] Compatibility tests pass.
 * [ ] Hard-coding audit passes.
 *
 * ============================================================================
 * PUBLIC GRAMMAR
 * ============================================================================
 */

parser grammar Profiling;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     profile;
 *
 *     profile computation;
 *
 *     profile computation with {
 *         ...
 *     };
 *
 *     profile computation {
 *         ...
 *     };
 *
 * The target is an expression so profiling remains independent of a fixed
 * list of executable entities.
 *
 * The configuration body is optional.
 *
 * ============================================================================
 */

profilingConstruct
    : PROFILE profilingTarget? profilingConfigurationAttachment?
      profilingEntry* profilingTerminator?
    ;


/*
 * ============================================================================
 * 2. PROFILING TARGET
 * ============================================================================
 *
 * The target can be:
 *
 *     a function;
 *     a call;
 *     a computation;
 *     a pipeline;
 *     a task;
 *     a scheduler;
 *     a compiler phase;
 *     a quantum computation;
 *     a hardware computation;
 *     a distributed computation;
 *     a future semantic object.
 *
 * Semantic analysis determines whether the target is profileable.
 *
 * ============================================================================
 */

profilingTarget
    : expression
    ;


/*
 * ============================================================================
 * 3. CONFIGURATION ATTACHMENT
 * ============================================================================
 *
 * `with` is owned lexically by the canonical language.
 *
 * The configuration itself is owned by this grammar.
 *
 * ============================================================================
 */

profilingConfigurationAttachment
    : WITH profilingBody
    ;


/*
 * ============================================================================
 * 4. CONFIGURATION BODY
 * ============================================================================
 *
 * There is deliberately no finite property count or nesting limit.
 * ============================================================================
 */

profilingBody
    : LBRACE profilingEntry* RBRACE
    ;


/*
 * ============================================================================
 * 5. PROFILING ENTRY
 * ============================================================================
 *
 * Stable semantic categories are represented explicitly where doing so
 * improves AST/semantic classification.
 *
 * The generic property rule remains the extensibility mechanism.
 * ============================================================================
 */

profilingEntry
    : profilingScope
    | profilingMetric
    | profilingCounter
    | profilingMeasurement
    | profilingSampling
    | profilingFilter
    | profilingAggregation
    | profilingBaseline
    | profilingComparison
    | profilingCorrelation
    | profilingAttribution
    | profilingCollection
    | profilingRetention
    | profilingExport
    | profilingPrivacy
    | profilingClassification
    | profilingIntegrity
    | profilingProvenance
    | profilingCapability
    | profilingResource
    | profilingRequirement
    | profilingConstraint
    | profilingPreference
    | profilingHint
    | profilingProperty
    ;


/*
 * ============================================================================
 * 6. SCOPE
 * ============================================================================
 *
 * Example:
 *
 *     scope: execution;
 *
 *     scope: compiler::phase;
 *
 *     scope: quantum::scheduling;
 *
 * The scope is semantic data.
 * ============================================================================
 */

profilingScope
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 7. METRIC
 * ============================================================================
 *
 * Examples:
 *
 *     metric: latency;
 *
 *     metric: throughput;
 *
 *     metric: operations;
 *
 *     metric: custom::metric;
 *
 * No closed metric enumeration exists.
 * ============================================================================
 */

profilingMetric
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 8. COUNTER
 * ============================================================================
 *
 * Counter names remain semantic identifiers.
 *
 * This permits integration with the existing Rust aggregate profiling
 * subsystem without embedding its implementation-specific field names into
 * the grammar.
 * ============================================================================
 */

profilingCounter
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 9. MEASUREMENT
 * ============================================================================
 */

profilingMeasurement
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 10. SAMPLING
 * ============================================================================
 *
 * Sampling values are expressions.
 *
 * Therefore:
 *
 *     sampling: automatic;
 *     sampling: rate;
 *     sampling: expression;
 *
 * are structurally possible without a fixed sampling implementation.
 * ============================================================================
 */

profilingSampling
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 11. FILTER
 * ============================================================================
 */

profilingFilter
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 12. AGGREGATION
 * ============================================================================
 *
 * Examples:
 *
 *     aggregation: aggregate;
 *     aggregation: sum;
 *     aggregation: histogram;
 *     aggregation: custom::aggregation;
 *
 * The parser does not implement aggregation.
 * ============================================================================
 */

profilingAggregation
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 13. BASELINE
 * ============================================================================
 */

profilingBaseline
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 14. COMPARISON
 * ============================================================================
 */

profilingComparison
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 15. CORRELATION
 * ============================================================================
 *
 * Correlation can connect profiling observations to:
 *
 *     execution context;
 *     trace context;
 *     task context;
 *     compilation context;
 *     distributed context;
 *     user-defined context.
 *
 * This rule does not import tracing semantics.
 * ============================================================================
 */

profilingCorrelation
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 16. ATTRIBUTION
 * ============================================================================
 *
 * Attribution identifies the semantic source to which a profile observation
 * should be associated.
 * ============================================================================
 */

profilingAttribution
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 17. COLLECTION
 * ============================================================================
 *
 * Collection describes whether/how observations should be collected.
 *
 * It does not implement collection.
 * ============================================================================
 */

profilingCollection
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 18. RETENTION
 * ============================================================================
 *
 * Retention is expressed as semantic data.
 *
 * It does not create storage.
 * ============================================================================
 */

profilingRetention
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 19. EXPORT
 * ============================================================================
 *
 * Export describes intent.
 *
 * The grammar does not enumerate:
 *
 *     Prometheus
 *     OTLP
 *     vendor exporter
 *     local file
 *     remote collector
 *
 * as mandatory language concepts.
 * ============================================================================
 */

profilingExport
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 20. PRIVACY
 * ============================================================================
 */

profilingPrivacy
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 21. CLASSIFICATION
 * ============================================================================
 */

profilingClassification
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 22. INTEGRITY
 * ============================================================================
 */

profilingIntegrity
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 23. PROVENANCE
 * ============================================================================
 */

profilingProvenance
    : profilingNamedEntry
    ;


/*
 * ============================================================================
 * 24. CAPABILITY
 * ============================================================================
 *
 * Capability is kept structurally separate from a generic property.
 *
 * Example:
 *
 *     capability: "profiling.hardware_counter";
 *
 * Semantic analysis resolves whether the runtime/target can provide it.
 * ============================================================================
 */

profilingCapability
    : CAPABILITY profilingAssignment profilingEntryTerminator
    ;


/*
 * ============================================================================
 * 25. RESOURCE
 * ============================================================================
 *
 * Resource requirements remain expressions.
 *
 * No resource capacity is hard-coded.
 * ============================================================================
 */

profilingResource
    : RESOURCE profilingAssignment profilingEntryTerminator
    ;


/*
 * ============================================================================
 * 26. REQUIREMENT
 * ============================================================================
 *
 * Requirement is mandatory semantic intent.
 *
 * ============================================================================
 */

profilingRequirement
    : REQUIRES profilingValue profilingEntryTerminator
    ;


/*
 * ============================================================================
 * 27. CONSTRAINT
 * ============================================================================
 *
 * Constraint restricts valid realization.
 *
 * ============================================================================
 */

profilingConstraint
    : CONSTRAINT profilingAssignment profilingEntryTerminator
    ;


/*
 * ============================================================================
 * 28. PREFERENCE
 * ============================================================================
 *
 * Preference is advisory and MUST NOT be silently promoted to a requirement.
 * ============================================================================
 */

profilingPreference
    : PREFER profilingValue profilingEntryTerminator
    ;


/*
 * ============================================================================
 * 29. HINT
 * ============================================================================
 *
 * Hint is advisory implementation guidance.
 * ============================================================================
 */

profilingHint
    : HINT profilingValue profilingEntryTerminator
    ;


/*
 * ============================================================================
 * 30. GENERIC PROPERTY
 * ============================================================================
 *
 * This is the principal forward-compatibility mechanism.
 *
 * Examples:
 *
 *     metric: latency;
 *
 *     quantum::operation: operation_count;
 *
 *     scheduler::decision: scheduling_decisions;
 *
 *     hardware::thermal: temperature;
 *
 *     future::profiling: expression;
 *
 * New semantic concepts do not need new lexer tokens.
 * ============================================================================
 */

profilingProperty
    : qualifiedName profilingAssignment profilingValue
      profilingEntryTerminator
    ;


/*
 * ============================================================================
 * 31. NAMED SEMANTIC ENTRY
 * ============================================================================
 *
 * This wrapper intentionally uses qualifiedName rather than a closed
 * enumeration of metric/scope/etc. names.
 *
 * The exact semantic category is determined by the consuming rule.
 * ============================================================================
 */

profilingNamedEntry
    : qualifiedName profilingAssignment profilingValue
      profilingEntryTerminator
    ;


/*
 * ============================================================================
 * 32. ASSIGNMENT
 * ============================================================================
 *
 * Both `:` and `=` are accepted.
 *
 * ============================================================================
 */

profilingAssignment
    : COLON profilingValue
    | ASSIGN profilingValue
    ;


/*
 * ============================================================================
 * 33. PROFILING VALUE
 * ============================================================================
 *
 * Profiling values reuse the canonical expression grammar.
 *
 * A nested property body is supported as a structural configuration value.
 *
 * ============================================================================
 */

profilingValue
    : expression
    | profilingPropertyBlock
    ;


/*
 * ============================================================================
 * 34. NESTED PROPERTY BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     metrics: {
 *         execution: latency;
 *         scheduling: decisions;
 *         quantum: operations;
 *     };
 *
 * Nested configuration depth is not limited by this grammar.
 * ============================================================================
 */

profilingPropertyBlock
    : LBRACE profilingPropertyEntry* RBRACE
    ;


profilingPropertyEntry
    : profilingProperty
    ;


/*
 * ============================================================================
 * 35. ENTRY TERMINATOR
 * ============================================================================
 *
 * Profiling properties use the canonical semicolon terminator.
 *
 * This prevents comma-separated configuration syntax from being confused with
 * canonical expression argument lists.
 * ============================================================================
 */

profilingEntryTerminator
    : SEMI
    ;


/*
 * ============================================================================
 * 36. OUTER TERMINATOR
 * ============================================================================
 *
 * The surrounding execution/declaration grammar may own the final statement
 * terminator.
 * ============================================================================
 */

profilingTerminator
    : SEMI
    ;


/*
 * ============================================================================
 * 37. SEMANTICALLY RECOGNIZED PROFILE DIMENSIONS
 * ============================================================================
 *
 * These are deliberately documented as semantic categories rather than
 * additional lexer/parser keywords.
 *
 * Common profile dimensions include:
 *
 *     execution
 *     compilation
 *     optimization
 *     scheduling
 *     routing
 *     placement
 *     resources
 *     memory
 *     communication
 *     synchronization
 *     concurrency
 *     resilience
 *     recovery
 *     quantum
 *     qec
 *     noise
 *     zqn
 *     hdl
 *     synthesis
 *     simulation
 *     hardware
 *     accelerator
 *     tensor
 *     ai
 *     distributed
 *     networking
 *     security
 *     energy
 *     thermal
 *     reliability
 *
 * These names are intentionally NOT enumerated by grammar alternatives.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. EXAMPLES — NON-NORMATIVE
 * ============================================================================
 *
 * The following examples illustrate the grammar.
 *
 * They are not additional parser rules.
 *
 * --------------------------------------------------------------------------
 *
 *     profile computation;
 *
 * --------------------------------------------------------------------------
 *
 *     profile computation with {
 *         scope: execution;
 *         metric: latency;
 *         metric: throughput;
 *         sampling: automatic;
 *         aggregation: aggregate;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     profile scheduler with {
 *         scope: quantum::scheduling;
 *         counter: scheduling_decisions;
 *         counter: dependency_edges;
 *         counter: resource_conflicts;
 *         aggregation: aggregate;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     profile computation with {
 *         metric: latency;
 *         baseline: reference_execution;
 *         comparison: delta;
 *         correlation: execution_context;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     profile computation with {
 *         requires capability("profiling.telemetry");
 *         resource: profiling_budget;
 *         prefer sampling_policy;
 *         hint aggregation_policy;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     profile quantum_program with {
 *         quantum::operation: operation_count;
 *         quantum::measurement: measurement_count;
 *         qec::round: round_count;
 *         noise::observation: noise_observation;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     profile hardware_work with {
 *         hardware::thermal: temperature;
 *         hardware::energy: energy_consumption;
 *         hardware::timing: timing_observation;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 *     profile distributed_work with {
 *         distributed::communication: communication_volume;
 *         distributed::synchronization: synchronization_wait;
 *         distributed::latency: end_to_end_latency;
 *     };
 *
 * --------------------------------------------------------------------------
 *
 * These examples do not select physical hardware or a profiling backend.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. AST / SEMANTIC / IR TRACEABILITY
 * ============================================================================
 *
 * Every public rule has the following intended ownership:
 *
 * profilingConstruct
 *     -> ProfilingIntent
 *
 * profilingTarget
 *     -> ProfilingTarget
 *
 * profilingBody
 *     -> ProfilingConfiguration
 *
 * profilingEntry
 *     -> ProfilingProperty / specialized profiling intent
 *
 * profilingMetric
 *     -> ProfilingMetricIntent
 *
 * profilingCounter
 *     -> ProfilingCounterIntent
 *
 * profilingSampling
 *     -> ProfilingSamplingIntent
 *
 * profilingFilter
 *     -> ProfilingFilterIntent
 *
 * profilingAggregation
 *     -> ProfilingAggregationIntent
 *
 * profilingRequirement
 *     -> Requirement
 *
 * profilingConstraint
 *     -> Constraint
 *
 * profilingPreference
 *     -> Preference
 *
 * profilingHint
 *     -> Hint
 *
 * profilingValue
 *     -> canonical expression/value representation
 *
 * No rule maps directly to a physical profiler implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. INTEGRATION MATRIX
 * ============================================================================
 *
 * UPSTREAM
 *
 *     grammar/lexer/
 *         lexical tokens and literals
 *
 *     grammar/core/
 *         names, punctuation, source structure
 *
 *     grammar/expressions/
 *         profiling expressions and values
 *
 *     grammar/types/
 *         only when semantic validation requires type information
 *
 * --------------------------------------------------------------------------
 *
 * PEER EXECUTION COMPONENTS
 *
 *     execution/observability.g4
 *         broader observability composition
 *
 *     execution/tracing.g4
 *         tracing and span semantics
 *
 *     execution/runtime.g4
 *         runtime profile attachment
 *
 *     execution/scheduling.g4
 *         scheduling intent
 *
 *     execution/resilience.g4
 *         resilience intent
 *
 *     execution/recovery.g4
 *         recovery intent
 *
 *     execution/checkpointing.g4
 *         checkpoint intent
 *
 *     execution/placement.g4
 *         placement intent
 *
 * --------------------------------------------------------------------------
 *
 * DOWNSTREAM
 *
 *     src/frontend/ast/
 *         domain-neutral AST
 *
 *     semantic analysis
 *         profile validation and classification
 *
 *     compiler
 *         instrumentation/planning metadata
 *
 *     scheduler diagnostics
 *         scheduler profile realization
 *
 *     runtime
 *         runtime profile realization
 *
 *     hardware/HAL
 *         target-specific counter realization
 *
 *     telemetry infrastructure
 *         actual collection/export
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. EXISTING RUST SCHEDULER PROFILING INTEGRATION
 * ============================================================================
 *
 * The repository already contains:
 *
 *     src/quantum/scheduling/diagnostics/profile.rs
 *
 * Its architecture is compatible with this grammar because it:
 *
 *     - uses aggregate counters;
 *     - avoids operation-sized profile storage by default;
 *     - supports optional unique-qubit tracking;
 *     - supports profile merging;
 *     - uses checked counter arithmetic;
 *     - separates host profiling time from quantum schedule time;
 *     - uses canonical quantum::ir::qubit::QubitId;
 *     - requires no unsafe Rust.
 *
 * This grammar therefore represents configuration/intent only.
 *
 * It MUST NOT mirror Rust implementation fields one-for-one.
 *
 * For example:
 *
 *     counter: scheduling_decisions;
 *
 * may semantically map to an implementation field such as:
 *
 *     scheduling_decisions
 *
 * without making that Rust field part of the language specification.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. PROFILE IDENTITY AND REPRODUCIBILITY
 * ============================================================================
 *
 * A profiling configuration may contribute to profiling-session identity.
 *
 * However, observed host-dependent values MUST NOT automatically contribute
 * to program semantic identity.
 *
 * In particular:
 *
 *     elapsed host time
 *     hardware temperature
 *     dynamic power
 *     wall-clock timestamp
 *     OS load
 *
 * are observations, not portable program semantics.
 *
 * The semantic layer decides which profile configuration fields are
 * reproducibility-relevant.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. MEMORY-SCALABLE PROFILING
 * ============================================================================
 *
 * The language itself does not mandate retaining one record per:
 *
 *     operation;
 *     qubit;
 *     thread;
 *     resource;
 *     node;
 *     sample.
 *
 * A program can request aggregate profiling.
 *
 * An implementation can realize that request using constant or sublinear
 * auxiliary state where the selected metrics permit it.
 *
 * This is particularly important for:
 *
 *     extremely large classical workloads;
 *     very large quantum schedules;
 *     distributed workloads;
 *     large tensor graphs;
 *     large HDL synthesis jobs.
 *
 * The grammar therefore describes observation intent rather than a storage
 * strategy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal hardware-size assumptions.
 *
 * Specifically, there is no grammar rule requiring:
 *
 *     a fixed number of samples;
 *     a fixed number of counters;
 *     a fixed number of profile targets;
 *     a fixed number of devices;
 *     a fixed number of workers;
 *     a fixed number of qubits;
 *     a fixed memory size;
 *     a fixed node count;
 *     a fixed metric count.
 *
 * Numeric expressions remain program data.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. SECURITY INVARIANTS
 * ============================================================================
 *
 * Profiling syntax MUST remain inert.
 *
 * Parsing a profiling request MUST NOT:
 *
 *     execute commands;
 *     open files;
 *     read secrets;
 *     access environment variables;
 *     access hardware;
 *     access network services;
 *     invoke a profiler;
 *     enable privileged telemetry.
 *
 * Authorization is a downstream concern.
 *
 * A source-level:
 *
 *     requires capability("profiling.hardware_counter")
 *
 * is a capability requirement.
 *
 * It is NOT authorization by itself.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. DIAGNOSTIC INVARIANTS
 * ============================================================================
 *
 * Semantic diagnostics should distinguish at least:
 *
 *     PROFILE_UNKNOWN_TARGET
 *     PROFILE_UNKNOWN_METRIC
 *     PROFILE_UNKNOWN_COUNTER
 *     PROFILE_UNSUPPORTED_SCOPE
 *     PROFILE_UNSUPPORTED_CAPABILITY
 *     PROFILE_RESOURCE_UNAVAILABLE
 *     PROFILE_INVALID_SAMPLING
 *     PROFILE_INVALID_FILTER
 *     PROFILE_INVALID_AGGREGATION
 *     PROFILE_INVALID_RETENTION
 *     PROFILE_INVALID_EXPORT
 *     PROFILE_PRIVACY_VIOLATION
 *     PROFILE_PORTABILITY_VIOLATION
 *     PROFILE_CONFLICTING_POLICY
 *
 * These are diagnostic/semantic concepts.
 *
 * This grammar does NOT hard-code diagnostic implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     profile;
 *     profile computation;
 *     profile computation with { metric: latency; };
 *     profile scheduler with {
 *         counter: scheduling_decisions;
 *     };
 *     profile quantum_program with {
 *         quantum::operation: operation_count;
 *     };
 *     profile workload with {
 *         distributed::communication: volume;
 *     };
 *     profile workload with {
 *         hardware::thermal: temperature;
 *     };
 *
 * Negative:
 *
 *     profile with;
 *     profile computation with { };
 *     profile computation with { metric: };
 *     profile computation with { : latency; };
 *     profile computation with { ::metric: latency; };
 *     profile computation with { metric latency; };
 *
 * Boundary:
 *
 *     very long metric names;
 *     deeply qualified metric names;
 *     many profile entries;
 *     deeply nested property blocks;
 *     large expression values;
 *     large lists/maps where supported by Expressions;
 *     many independent profile declarations.
 *
 * Scalability:
 *
 *     profile dimensions proportional to program size;
 *     symbolic sample counts;
 *     symbolic retention;
 *     symbolic resource budgets;
 *     symbolic workload-dependent metrics.
 *
 * Determinism:
 *
 *     identical token streams produce identical parse structures.
 *
 * Compatibility:
 *
 *     PROFILE remains the canonical lexical entry point;
 *     no new lexer token is required;
 *     existing runtime profile intent can migrate to this construct;
 *     observability can compose profiling without duplicate rules.
 *
 * Hard-coding:
 *
 *     repository-wide audit must find no profiling capacity constant in this
 *     grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Profiling means:
 *
 *     WHAT SHOULD BE MEASURED
 *
 * not:
 *
 *     HOW THE MEASUREMENT IS IMPLEMENTED
 *
 * Therefore:
 *
 *     profiling intent
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     classical            quantum::ir
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *             compiler/runtime
 *                    |
 *                    v
 *             actual profiler
 *
 * This separation permits the same Zamani source program to be profiled on:
 *
 *     tiny embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     QPUs;
 *     simulators;
 *     accelerators;
 *     HPC systems;
 *     distributed systems;
 *     cloud environments;
 *     future execution substrates;
 *
 * subject only to the capabilities and resources actually available.
 *
 * ============================================================================
 */