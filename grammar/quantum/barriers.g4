/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/barriers.g4
 *
 * Grammar:
 *     QuantumBarriers
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM BARRIER GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE GRAMMAR OWNER for source-level quantum barrier
 * syntax.
 *
 * A barrier expresses a logical ordering/synchronization boundary in a
 * quantum computation.
 *
 * The barrier says:
 *
 *     "The semantic operations associated with these quantum resources must
 *      not be freely reordered across this boundary unless the relevant
 *      semantic/optimization contract explicitly permits it."
 *
 * It does NOT specify:
 *
 *     - a physical clock cycle;
 *     - a hardware synchronization primitive;
 *     - a pulse;
 *     - a pulse duration;
 *     - a device;
 *     - a QPU;
 *     - a physical qubit;
 *     - a coupling map;
 *     - a topology;
 *     - a scheduling slot;
 *     - a calibration;
 *     - a vendor instruction;
 *     - a backend implementation.
 *
 * Those concerns belong downstream.
 *
 * ============================================================================
 * CANONICAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     QuantumBarriers
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> quantum type validation
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> effect analysis
 *          +--> ordering semantics
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime
 *
 * This grammar MUST NOT bypass the frontend AST, semantic analysis, or
 * canonical quantum::ir boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     quantumBarrierStatement
 *     quantumBarrierTargetList
 *     quantumBarrierTarget
 *     quantumBarrierReference
 *     quantumBarrierSelector
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - keywords;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - general types;
 *     - qubit declarations;
 *     - quantum register declarations;
 *     - quantum operations;
 *     - operation parameters;
 *     - controlled operations;
 *     - adjoints;
 *     - inverse operations;
 *     - measurements;
 *     - reset;
 *     - observables;
 *     - dynamic circuits;
 *     - classical control flow;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware discovery;
 *     - physical allocation;
 *     - calibration;
 *     - backend selection;
 *     - runtime execution;
 *     - canonical quantum::ir types.
 *
 * There MUST be exactly one canonical owner for every production listed
 * above.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     canonical lexer
 *           |
 *           v
 *     Names + Expressions
 *           |
 *           v
 *     QuantumBarriers
 *           |
 *           v
 *     quantum statement composition
 *           |
 *           v
 *     domain-neutral frontend AST
 *           |
 *           v
 *     semantic analysis
 *           |
 *           v
 *     quantum::ir
 *
 * This grammar MUST NOT import:
 *
 *     Operations
 *     Controls
 *     Adjoints
 *     Measurement
 *     Reset
 *     QEC
 *     ZQN
 *     Hardware
 *     Resources
 *     Runtime
 *
 * merely to perform semantic validation.
 *
 * This keeps the dependency graph acyclic.
 *
 * ============================================================================
 * CANONICAL LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical parser lexer boundary is:
 *
 *     ZamaniLexer
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * MUST be used.
 *
 * This file MUST NOT use:
 *
 *     ZamaniTokens
 *     K_BARRIER
 *     K_BARRIER_TOKEN
 *     BARRIER_TOKEN
 *
 * as alternative barrier vocabularies.
 *
 * The canonical keyword identified by the existing lexical specification is:
 *
 *     barrier
 *
 * with canonical token:
 *
 *     BARRIER
 *
 * This file introduces no lexer tokens.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Names supplies reusable source-name syntax.
 *
 * Expressions supplies expression syntax used by selectors.
 *
 * Only those dependencies are required by this grammar.
 *
 * The file intentionally does not import Types merely to parse a target.
 *
 * Whether a target is quantum, resettable, measurable, or otherwise valid is
 * a semantic concern.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Barrier syntax is target-independent.
 *
 * This file contains NO universal machine-size limits.
 *
 * In particular, it contains no:
 *
 *     MAX_QUBITS
 *     MAX_TARGETS
 *     MAX_BARRIERS
 *     MAX_CIRCUIT_DEPTH
 *     MAX_REGISTERS
 *     MAX_DEVICES
 *     MAX_QPUS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_THREADS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TIMELINES
 *
 * It also contains no:
 *
 *     q0
 *     q1
 *     physical_qubit_0
 *     qpu_0
 *     device_0
 *     core_0
 *     channel_0
 *
 * as universal physical resources.
 *
 * Source cardinality is represented structurally through ANTLR repetition.
 *
 * Therefore:
 *
 *     barrier q;
 *
 * and:
 *
 *     barrier q0, q1, q2, ...;
 *
 * use the same grammar model.
 *
 * The practical size of a program is bounded only by:
 *
 *     - source representation;
 *     - parser resources;
 *     - compiler resources;
 *     - semantic resources;
 *     - runtime resources;
 *     - target capabilities;
 *     - explicit program/resource constraints.
 *
 * "Unbounded" here means that this grammar does not impose an artificial
 * finite machine-specific ceiling.
 *
 * ============================================================================
 * SEMANTIC MEANING
 * ============================================================================
 *
 * A barrier is an ordering/synchronization marker.
 *
 * For example:
 *
 *     apply H(q);
 *     barrier q;
 *     apply X(q);
 *
 * does not mean:
 *
 *     execute a particular hardware barrier instruction.
 *
 * It means that the semantic ordering boundary represented by the barrier
 * must be preserved according to the language's quantum execution semantics.
 *
 * The optimizer, scheduler, lowering system, and target backend determine
 * whether the barrier:
 *
 *     - must become an explicit target operation;
 *     - can be represented as an ordering constraint;
 *     - can be removed because it is semantically redundant;
 *     - can be weakened under a formally valid optimization;
 *     - requires target-specific realization.
 *
 * Such transformations MUST preserve the observable semantics of the
 * program.
 *
 * ============================================================================
 * CORE SOURCE FORMS
 * ============================================================================
 *
 * The canonical source form is:
 *
 *     barrier q;
 *
 * Multiple logical targets:
 *
 *     barrier q0, q1;
 *
 * Register/reference form:
 *
 *     barrier register;
 *
 * Indexed target:
 *
 *     barrier register[i];
 *
 * Symbolic selector:
 *
 *     barrier register[index];
 *
 * Selector expression:
 *
 *     barrier register[offset + i];
 *
 * Qualified target:
 *
 *     barrier module.register;
 *
 * Qualified indexed target:
 *
 *     barrier module.register[index];
 *
 * Multiple selected targets:
 *
 *     barrier q0, register[i], module.q[j];
 *
 * Target syntax remains open-ended.
 *
 * The semantic layer decides whether each resolved target is a valid quantum
 * barrier operand.
 *
 * ============================================================================
 * TARGET CARDINALITY
 * ============================================================================
 *
 * A barrier accepts one or more targets.
 *
 * The grammar deliberately uses:
 *
 *     *
 *     +
 *
 * rather than fixed alternatives.
 *
 * It therefore does NOT define:
 *
 *     barrier1
 *     barrier2
 *     barrier4
 *     barrier8
 *     barrier16
 *     barrier32
 *
 * or any equivalent finite cardinality model.
 *
 * ============================================================================
 * WHY A BARRIER REQUIRES A TARGET
 * ============================================================================
 *
 * The existing canonical quantum IR validation requires a barrier to contain
 * actual operands.
 *
 * Therefore this grammar intentionally requires:
 *
 *     quantumBarrierTargetList
 *
 * rather than accepting:
 *
 *     barrier;
 *
 * as a valid canonical quantum barrier.
 *
 * If the language later defines a distinct global synchronization barrier,
 * that construct MUST receive its own semantic contract rather than silently
 * changing the meaning of this rule.
 *
 * ============================================================================
 * TARGET REFERENCE MODEL
 * ============================================================================
 *
 * A barrier target is represented as:
 *
 *     qualifiedName selector*
 *
 * This supports:
 *
 *     q
 *     register
 *     module.register
 *     register[i]
 *     register[i + offset]
 *     namespace.register[index]
 *
 * without requiring a fixed qubit/register representation.
 *
 * A simple identifier is already representable through qualifiedName.
 *
 * This avoids the unnecessary ambiguous pattern:
 *
 *     identifier
 *     | qualifiedName
 *
 * because an ordinary identifier is a valid simple qualified name.
 *
 * ============================================================================
 * SELECTOR MODEL
 * ============================================================================
 *
 * A selector is:
 *
 *     [ expression ]
 *
 * The expression grammar owns the expression itself.
 *
 * This permits selectors whose values are:
 *
 *     constants;
 *     variables;
 *     generic values;
 *     symbolic expressions;
 *     computed indices;
 *     supported ranges;
 *     future expression forms.
 *
 * This grammar does not impose:
 *
 *     integer width;
 *     register width;
 *     array dimension;
 *     index maximum;
 *     selector count.
 *
 * Semantic analysis determines whether the selector is valid for the
 * resolved quantum resource.
 *
 * ============================================================================
 * RANGE / SLICE SEMANTICS
 * ============================================================================
 *
 * If the general expression grammar represents a range/slice expression,
 * forms such as:
 *
 *     barrier register[start .. end];
 *
 * may be represented structurally by:
 *
 *     quantumBarrierSelector
 *
 * without adding a special barrier-specific range grammar.
 *
 * This file therefore does not duplicate range syntax.
 *
 * The semantic layer determines:
 *
 *     - whether the selected resource supports slicing;
 *     - whether the range is valid;
 *     - whether the resulting collection is a legal barrier operand.
 *
 * ============================================================================
 * TARGET ORDER
 * ============================================================================
 *
 * Target order is source information.
 *
 * The grammar preserves:
 *
 *     barrier a, b;
 *
 * as an ordered target sequence.
 *
 * The parser MUST NOT sort, deduplicate, canonicalize, or otherwise reorder
 * targets.
 *
 * Semantic analysis may determine whether target ordering is semantically
 * relevant or whether equivalent targets can be normalized.
 *
 * Such normalization belongs downstream.
 *
 * ============================================================================
 * DUPLICATE TARGETS
 * ============================================================================
 *
 * Duplicate targets are syntactically representable:
 *
 *     barrier q, q;
 *
 * The grammar deliberately does not silently remove the duplicate.
 *
 * Semantic analysis decides whether duplicate barrier operands are:
 *
 *     - invalid;
 *     - redundant;
 *     - semantically equivalent;
 *     - accepted for a particular dialect.
 *
 * This preserves source information and gives semantic analysis ownership of
 * quantum resource rules.
 *
 * ============================================================================
 * EMPTY TARGETS
 * ============================================================================
 *
 * These are structurally invalid:
 *
 *     barrier;
 *     barrier ,;
 *     barrier q,;
 *     barrier q,,r;
 *
 * The canonical barrier requires at least one target and does not accept a
 * trailing comma.
 *
 * The no-trailing-comma rule is deliberate because it keeps barrier syntax
 * consistent with a required operand list and avoids silently representing
 * an empty operand.
 *
 * ============================================================================
 * NO HARD-CODED PHYSICAL TOPOLOGY
 * ============================================================================
 *
 * This grammar does not express:
 *
 *     nearest-neighbour topology;
 *     linear topology;
 *     grid topology;
 *     heavy-hex topology;
 *     all-to-all topology;
 *     physical coupling;
 *     physical channels;
 *     device coordinates.
 *
 * A barrier is therefore portable across:
 *
 *     CPU simulation;
 *     GPU simulation;
 *     FPGA implementation;
 *     ASIC implementation;
 *     QPU execution;
 *     distributed quantum execution;
 *     future quantum architectures.
 *
 * The actual realization is a downstream concern.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough source structure for the existing
 * domain-neutral frontend AST.
 *
 * The AST mapping must preserve at least:
 *
 *     - source span;
 *     - barrier keyword/source span where supported;
 *     - ordered barrier target expressions;
 *     - each target's source span;
 *     - each selector expression;
 *     - selector source spans;
 *     - delimiter/source information required for diagnostics and tooling.
 *
 * This grammar MUST NOT require a hardware-specific AST node such as:
 *
 *     PhysicalBarrierNode
 *     QpuBarrierNode
 *     HardwareBarrierNode
 *     ScheduledBarrierNode
 *
 * The preferred flow is:
 *
 *     quantumBarrierStatement
 *            |
 *            v
 *     domain-neutral AST
 *            |
 *            v
 *     semantic barrier operation
 *            |
 *            v
 *     quantum::ir
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - target name resolution;
 *     - quantum type checking;
 *     - selector validation;
 *     - target lifetime validation;
 *     - ownership/borrowing validation;
 *     - duplicate-target policy;
 *     - overlap analysis;
 *     - ordering semantics;
 *     - effect analysis;
 *     - capability requirements;
 *     - resource requirements;
 *     - dialect-specific barrier semantics;
 *     - optimization legality;
 *     - language-version compatibility.
 *
 * Syntax validity does not imply semantic validity.
 *
 * For example:
 *
 *     barrier classical_value;
 *
 * may be syntactically valid according to the grammar, but semantic analysis
 * must reject it if the resolved value is not a valid quantum barrier target.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * The grammar does not determine whether a target or execution environment
 * has sufficient resources.
 *
 * Semantic analysis may derive requirements such as:
 *
 *     capability("quantum.barrier")
 *
 * or target-specific ordering/synchronization capabilities when required.
 *
 * Such requirements MUST remain semantic data.
 *
 * This file must never encode a fixed resource quantity.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A barrier is not a state-transforming quantum gate merely because it
 * appears between quantum operations.
 *
 * Its primary semantic role is ordering/synchronization.
 *
 * Semantic/effect analysis determines:
 *
 *     - whether a barrier is observable;
 *     - whether it constrains reordering;
 *     - whether it introduces synchronization;
 *     - whether it affects dynamic-circuit dependencies;
 *     - whether it is removable under the language's optimization rules.
 *
 * The parser only preserves the construct.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * The canonical downstream boundary remains:
 *
 *     quantum::ir
 *
 * This file MUST NOT define:
 *
 *     QuantumBarrierIR
 *     BarrierInstructionIR
 *     PhysicalBarrierIR
 *     HardwareBarrierIR
 *
 * as a competing intermediate representation.
 *
 * Semantic lowering must use the repository's existing canonical quantum IR
 * barrier representation.
 *
 * The repository already has a canonical quantum IR barrier concept in the
 * quantum gate/standard IR path.
 *
 * The barrier lowering must therefore reuse that existing representation
 * rather than creating a second quantum IR hierarchy.
 *
 * ============================================================================
 * OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * Optimizers MAY transform barriers only when semantic ordering guarantees
 * remain unchanged.
 *
 * Examples of potentially valid downstream transformations include:
 *
 *     barrier q;
 *     barrier q;
 *
 * becoming an equivalent normalized representation if the language semantics
 * establish that the second boundary adds no observable information.
 *
 * However, the grammar itself MUST NOT perform this transformation.
 *
 * The parser preserves source barriers exactly.
 *
 * ============================================================================
 * ROUTING CONTRACT
 * ============================================================================
 *
 * Routing is downstream.
 *
 * A barrier target such as:
 *
 *     barrier logical_register;
 *
 * does not identify:
 *
 *     physical qubit 17
 *     physical qubit 18
 *     coupling edge 4
 *     device 0
 *
 * Routing may later determine physical realization while preserving the
 * semantic barrier boundary.
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * Scheduling is downstream.
 *
 * The grammar does not specify:
 *
 *     cycle;
 *     tick;
 *     nanoseconds;
 *     microseconds;
 *     clock period;
 *     pulse duration;
 *     readout time;
 *     synchronization slot.
 *
 * A barrier represents semantic ordering intent.
 *
 * The scheduler determines whether and how that intent becomes a concrete
 * execution schedule.
 *
 * ============================================================================
 * QEC / ZQN / RESILIENCE CONTRACT
 * ============================================================================
 *
 * This grammar does not implement:
 *
 *     QEC;
 *     syndrome extraction;
 *     decoding;
 *     noise modeling;
 *     mitigation;
 *     fault injection;
 *     recovery;
 *     retry;
 *     escalation;
 *     backend failover.
 *
 * Downstream systems may consume barrier semantics when required.
 *
 * The grammar remains independent of their implementations.
 *
 * ============================================================================
 * HARDWARE CONTRACT
 * ============================================================================
 *
 * No hardware-specific realization belongs in this file.
 *
 * Forbidden language-level assumptions include:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     qubit count;
 *     register width;
 *     memory capacity;
 *     device count;
 *     topology;
 *     clock rate;
 *     pulse duration;
 *     physical qubit IDs;
 *     physical channel IDs.
 *
 * These belong to resource/capability/hardware/target layers.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given the same token stream and language/dialect configuration, this grammar
 * must produce the same parse structure.
 *
 * Parsing must not inspect:
 *
 *     hardware;
 *     target capabilities;
 *     filesystem state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     environment variables.
 *
 * Target availability is therefore irrelevant to parsing.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no device access;
 *     - no command execution;
 *     - no dynamic code execution.
 *
 * Untrusted source can therefore be parsed as ordinary frontend input.
 *
 * Safe Rust remains a downstream implementation requirement:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     no unsafe Rust
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * Target lists use structural repetition:
 *
 *     (COMMA quantumBarrierTarget)*
 *
 * rather than fixed-size alternatives.
 *
 * Selector lists use:
 *
 *     quantumBarrierSelector*
 *
 * rather than a fixed number of indices.
 *
 * Therefore the grammar imposes no artificial maximum on:
 *
 *     - number of barriers in a program;
 *     - number of targets in a barrier;
 *     - number of selectors in a target.
 *
 * Practical implementation limits are resource limits rather than language
 * semantics.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Structural syntax errors include:
 *
 *     barrier;
 *     barrier ,;
 *     barrier q,;
 *     barrier q,,r;
 *     barrier q[;
 *     barrier q[];
 *     barrier q[expr;
 *     barrier q];
 *
 * These should be reported by the parser with the most precise source span
 * available.
 *
 * Semantic diagnostics include:
 *
 *     invalid barrier target;
 *     unresolved barrier target;
 *     non-quantum barrier target;
 *     invalid selector;
 *     selector out of bounds;
 *     invalid target lifetime;
 *     duplicate/conflicting target;
 *     unsupported synchronization semantics;
 *     unavailable required capability;
 *     insufficient resources.
 *
 * Semantic errors MUST NOT be disguised as parser errors.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The existing canonical source spelling remains:
 *
 *     barrier <targets>;
 *
 * The existing language specification also describes:
 *
 *     QuantumBarrierStatement ::=
 *         "barrier"
 *         QuantumTargetList
 *         [";"]
 *
 * This implementation chooses a required statement terminator at this
 * canonical detailed grammar boundary:
 *
 *     barrier <targets>;
 *
 * If the language-wide statement layer later establishes a universal
 * terminator policy, the compatibility change must be handled centrally
 * through the specification/compatibility process rather than by creating
 * another barrier grammar.
 *
 * ============================================================================
 * INTEGRATION WITH quantum.g4
 * ============================================================================
 *
 * `grammar/quantum/quantum.g4` is the quantum orchestration/composition layer.
 *
 * It MUST NOT reimplement barrier syntax.
 *
 * Its quantum barrier element should ultimately delegate to:
 *
 *     quantumBarrierStatement
 *
 * owned by this file.
 *
 * Conceptually:
 *
 *     quantumBarrierElement
 *         : quantumBarrierStatement
 *         ;
 *
 * The exact composition rule belongs to quantum.g4.
 *
 * This file does not import quantum.g4 because doing so would create the
 * wrong dependency direction.
 *
 * ============================================================================
 * INTEGRATION WITH statements/quantum.g4
 * ============================================================================
 *
 * `grammar/statements/quantum.g4` is the statement-family composition layer.
 *
 * Its existing adapter:
 *
 *     quantumBarrierStatement
 *         : barrierStatement
 *         ;
 *
 * must be reconciled with this file.
 *
 * After this file becomes the canonical barrier owner, the statement-layer
 * adapter MUST NOT introduce another concrete barrier production.
 *
 * The final composition should have exactly one effective:
 *
 *     quantumBarrierStatement
 *
 * rule.
 *
 * Therefore the repository must choose one of these ownership arrangements:
 *
 *     A. This file owns quantumBarrierStatement directly and the statement
 *        layer imports it without redefining the rule;
 *
 * OR
 *
 *     B. This file owns a detailed rule and statements/quantum.g4 owns only
 *        the adapter name.
 *
 * The preferred production architecture is A because it eliminates an
 * unnecessary compatibility alias.
 *
 * ============================================================================
 * INTEGRATION WITH quantum/gates.g4
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The current repository contains an older/consolidated barrier owner in:
 *
 *     grammar/quantum/gates.g4
 *
 * specifically:
 *
 *     barrierStatement
 *     barrierOperandList
 *
 * Those rules MUST NOT remain competing canonical owners once this file is
 * integrated.
 *
 * The final architecture must therefore remove or retire the concrete
 * barrier productions from gates.g4 and make this file the canonical owner.
 *
 * The gate grammar may continue to own gate syntax, but it MUST NOT own
 * barrier syntax.
 *
 * This preserves the intended separation:
 *
 *     gates.g4
 *         gate syntax
 *
 *     barriers.g4
 *         barrier syntax
 *
 * ============================================================================
 * INTEGRATION WITH quantum/operations.g4
 * ============================================================================
 *
 * Operations do not own barrier syntax.
 *
 * A barrier is not represented as:
 *
 *     apply barrier(...)
 *
 * in the canonical language.
 *
 * Operations remain responsible for operation invocation.
 *
 * Barrier remains a dedicated synchronization construct.
 *
 * ============================================================================
 * INTEGRATION WITH quantum/measurement.g4
 * ============================================================================
 *
 * Measurement and barrier are separate semantic constructs.
 *
 * Measurement owns:
 *
 *     measurement targets;
 *     destinations;
 *     measurement options.
 *
 * Barrier owns:
 *
 *     barrier targets;
 *     synchronization intent.
 *
 * Neither grammar imports the other.
 *
 * ============================================================================
 * INTEGRATION WITH quantum/reset.g4
 * ============================================================================
 *
 * Reset and barrier are separate constructs.
 *
 * Reset changes the quantum state according to reset semantics.
 *
 * Barrier establishes an ordering/synchronization boundary.
 *
 * This file must not import reset.g4.
 *
 * ============================================================================
 * INTEGRATION WITH expressions/quantum.g4
 * ============================================================================
 *
 * The repository currently contains a legacy expression-level barrier form
 * using:
 *
 *     K_BARRIER
 *
 * That token is inconsistent with the canonical lexical vocabulary, which
 * defines:
 *
 *     BARRIER
 *
 * This file MUST NOT introduce K_BARRIER as a compatibility token.
 *
 * If barrier expressions are retained, they must be reconciled separately
 * with the canonical BARRIER token and with the language's expression/statement
 * semantics.
 *
 * A statement-level barrier MUST remain the canonical form for:
 *
 *     barrier <targets>;
 *
 * ============================================================================
 * INTEGRATION WITH lexer/keywords.md
 * ============================================================================
 *
 * The canonical keyword registry already identifies:
 *
 *     barrier
 *
 * as:
 *
 *     BARRIER
 *
 * This file consumes that token.
 *
 * No keyword registry changes are required for this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH spec/syntax.md
 * ============================================================================
 *
 * The syntax specification defines:
 *
 *     QuantumBarrierStatement
 *
 * in terms of:
 *
 *     "barrier"
 *     QuantumTargetList
 *
 * This grammar implements that syntax using the canonical lexer token and
 * the target structure defined locally for barrier operands.
 *
 * Any future syntax change must be promoted through the specification and
 * compatibility system.
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM IR
 * ============================================================================
 *
 * The repository already contains a canonical quantum IR barrier concept.
 *
 * This grammar does not recreate it.
 *
 * Required lowering path:
 *
 *     quantumBarrierStatement
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic barrier operation
 *          |
 *          v
 *     existing quantum::ir barrier representation
 *
 * The IR remains the canonical boundary for downstream:
 *
 *     optimization;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN;
 *     resilience;
 *     HAL;
 *     target lowering.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Barrier syntax belongs to quantum computation, but its semantic concept of
 * ordering may interact with:
 *
 *     classical computation;
 *     hybrid execution;
 *     distributed execution;
 *     accelerator execution;
 *     HDL/co-design;
 *     asynchronous execution.
 *
 * Those domains MUST consume the semantic/IR representation rather than
 * importing this grammar and assigning their own physical meaning to the
 * source keyword.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive parser tests:
 *
 *     barrier q;
 *
 *     barrier q0, q1;
 *
 *     barrier register;
 *
 *     barrier register[i];
 *
 *     barrier register[index];
 *
 *     barrier register[offset + i];
 *
 *     barrier module.register;
 *
 *     barrier module.register[index];
 *
 *     barrier q0, register[i], module.q[j];
 *
 * Required boundary tests:
 *
 *     one target;
 *
 *     many targets;
 *
 *     deeply qualified target;
 *
 *     one selector;
 *
 *     multiple selectors;
 *
 *     symbolic selector;
 *
 *     large numeric selector expression;
 *
 *     large target list;
 *
 *     multiple barrier statements;
 *
 *     barriers separated by quantum operations;
 *
 *     barriers inside nested quantum blocks.
 *
 * Required negative parser tests:
 *
 *     barrier;
 *
 *     barrier ,;
 *
 *     barrier q,;
 *
 *     barrier q,,r;
 *
 *     barrier q[;
 *
 *     barrier q[];
 *
 *     barrier q[expr;
 *
 *     barrier q];
 *
 *     barrier [expr];
 *
 * Required semantic tests:
 *
 *     unresolved target;
 *
 *     classical target;
 *
 *     invalid quantum target;
 *
 *     invalid selector;
 *
 *     selector outside resolved resource bounds;
 *
 *     duplicate target;
 *
 *     overlapping target selections;
 *
 *     invalid target lifetime;
 *
 *     unavailable synchronization capability.
 *
 * Required scalability tests:
 *
 *     generated barriers with increasing target cardinality;
 *
 *     generated programs with increasing barrier count;
 *
 *     generated targets with increasing selector depth;
 *
 *     large symbolic selector expressions;
 *
 *     large source files;
 *
 *     no artificial grammar maximum.
 *
 * Required determinism tests:
 *
 *     identical source -> equivalent parse tree;
 *
 *     identical token stream -> equivalent parse tree;
 *
 *     target availability does not change parse result.
 *
 * Required compatibility tests:
 *
 *     existing barrier source form remains accepted;
 *
 *     canonical BARRIER token is used;
 *
 *     stale K_BARRIER is not required;
 *
 *     gates.g4 no longer provides a competing barrier owner;
 *
 *     statement-level composition resolves to this owner.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST pass the following audit:
 *
 * [x] No MAX_QUBITS.
 * [x] No MAX_TARGETS.
 * [x] No MAX_BARRIERS.
 * [x] No MAX_CIRCUIT_DEPTH.
 * [x] No MAX_DEVICES.
 * [x] No MAX_QPUS.
 * [x] No MAX_CPUS.
 * [x] No MAX_GPUS.
 * [x] No MAX_FPGAS.
 * [x] No MAX_THREADS.
 * [x] No MAX_NODES.
 * [x] No MAX_MEMORY.
 * [x] No fixed register width.
 * [x] No fixed qubit count.
 * [x] No physical device identifiers.
 * [x] No physical topology.
 * [x] No fixed timing.
 * [x] No fixed pulse duration.
 * [x] No vendor instruction inventory.
 * [x] No backend selection.
 * [x] No physical allocation.
 *
 * Ordinary source values remain legal where permitted by the expression
 * grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * Independent grammar-contract completion:
 *
 * [x] Parser grammar declaration exists.
 * [x] Canonical ZamaniLexer vocabulary is consumed.
 * [x] Canonical BARRIER token is used.
 * [x] No K_BARRIER token is introduced.
 * [x] Reusable Names grammar is imported.
 * [x] Reusable Expressions grammar is imported.
 * [x] Barrier syntax has one documented owner.
 * [x] Barrier target-list syntax is structurally unbounded.
 * [x] Target selectors are expression-based.
 * [x] Target ordering is preserved.
 * [x] Duplicate targets are preserved for semantic validation.
 * [x] Empty target lists are rejected structurally.
 * [x] No hardware topology is encoded.
 * [x] No physical resource IDs are encoded.
 * [x] No machine-size constants are encoded.
 * [x] No second quantum IR is defined.
 * [x] AST contract is specified.
 * [x] Semantic contract is specified.
 * [x] quantum::ir integration is specified.
 * [x] optimization boundary is specified.
 * [x] routing boundary is specified.
 * [x] scheduling boundary is specified.
 * [x] QEC boundary is specified.
 * [x] ZQN boundary is specified.
 * [x] resilience boundary is specified.
 * [x] hardware boundary is specified.
 * [x] deterministic parsing contract is specified.
 * [x] security contract is specified.
 * [x] scalability contract is specified.
 * [x] diagnostic contract is specified.
 * [x] compatibility contract is specified.
 * [x] positive tests are specified.
 * [x] negative tests are specified.
 * [x] boundary tests are specified.
 * [x] scalability tests are specified.
 * [x] determinism tests are specified.
 * [x] compatibility tests are specified.
 * [x] hard-coding audit is specified.
 *
 * Repository integration completion:
 *
 * [ ] gates.g4 no longer owns concrete barrier syntax.
 * [ ] statements/quantum.g4 imports/admits this canonical barrier owner
 *     without creating a duplicate rule.
 * [ ] quantum.g4 composes this rule without redefining it.
 * [ ] stale K_BARRIER expression usage has been reconciled.
 * [ ] generated ANTLR parser accepts the positive corpus.
 * [ ] generated ANTLR parser rejects the structural negative corpus.
 * [ ] Rust lexer emits BARRIER consistently.
 * [ ] frontend AST lowering preserves all required source information.
 * [ ] semantic validation validates quantum targets and selectors.
 * [ ] canonical quantum::ir lowering succeeds.
 * [ ] downstream optimization preserves barrier semantics.
 * [ ] routing preserves logical barrier scope.
 * [ ] scheduling preserves required ordering.
 * [ ] cross-domain tests pass.
 *
 * ============================================================================
 * CANONICAL GRAMMAR
 * ============================================================================
 */

parser grammar QuantumBarriers;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. CANONICAL BARRIER STATEMENT
 * ============================================================================
 *
 * Canonical source form:
 *
 *     barrier <target>, <target>, ...;
 *
 * At least one target is required.
 *
 * The semicolon is deliberately owned by the detailed statement grammar,
 * matching the canonical measurement statement's ownership model.
 */
quantumBarrierStatement
    : BARRIER
      quantumBarrierTargetList
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. TARGET LIST
 * ============================================================================
 *
 * One or more targets.
 *
 * There is intentionally no finite target-count alternative.
 */
quantumBarrierTargetList
    : quantumBarrierTarget
      (COMMA quantumBarrierTarget)*
    ;


/*
 * ============================================================================
 * 3. TARGET
 * ============================================================================
 *
 * A target consists of a reusable source reference followed by zero or more
 * selectors.
 *
 * Examples:
 *
 *     q
 *     register
 *     module.register
 *     register[i]
 *     register[i][j]
 */
quantumBarrierTarget
    : quantumBarrierReference
      quantumBarrierSelector*
    ;


/*
 * ============================================================================
 * 4. TARGET REFERENCE
 * ============================================================================
 *
 * qualifiedName already represents a simple identifier as well as qualified
 * names, so a separate identifier alternative is unnecessary.
 */
quantumBarrierReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 5. TARGET SELECTOR
 * ============================================================================
 *
 * The selector expression is owned by the general expression grammar.
 *
 * Examples:
 *
 *     [i]
 *     [index]
 *     [offset + i]
 *     [start .. end]
 *
 * where the latter is available only when the general expression grammar
 * supports that range form.
 */
quantumBarrierSelector
    : LBRACK expression RBRACK
    ;